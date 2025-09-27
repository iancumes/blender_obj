
use std::{fs, path::{Path, PathBuf}};
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct FaceIndex {
    pub v: Option<usize>,
    pub vt: Option<usize>,
    pub vn: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ObjData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub faces: Vec<Vec<FaceIndex>>,
    pub used_materials: Vec<String>,
    pub groups: Vec<String>,
    pub objects: Vec<String>,
    pub mtllibs: Vec<String>,
}

pub fn parse_obj(path: &Path) -> Result<(ObjData, Vec<PathBuf>)> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("No se pudo leer el archivo OBJ {}", path.display()))?;

    let mut vertices = Vec::<[f32;3]>::new();
    let mut normals  = Vec::<[f32;3]>::new();
    let mut texcoords= Vec::<[f32;2]>::new();
    let mut faces    = Vec::<Vec<FaceIndex>>::new();
    let mut used_materials = Vec::<String>::new();
    let mut groups   = Vec::<String>::new();
    let mut objects  = Vec::<String>::new();
    let mut mtllibs  = Vec::<String>::new();

    for (lineno, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        // split once for the tag
        let mut it = line.split_whitespace();
        let tag = it.next().unwrap_or("");

        match tag {
            "v" => {
                // v x y z
                let vals: Vec<&str> = it.collect();
                if vals.len() < 3 { continue; }
                let x: f32 = vals[0].parse().unwrap_or(0.0);
                let y: f32 = vals[1].parse().unwrap_or(0.0);
                let z: f32 = vals[2].parse().unwrap_or(0.0);
                vertices.push([x, y, z]);
            },
            "vn" => {
                let vals: Vec<&str> = it.collect();
                if vals.len() < 3 { continue; }
                let x: f32 = vals[0].parse().unwrap_or(0.0);
                let y: f32 = vals[1].parse().unwrap_or(0.0);
                let z: f32 = vals[2].parse().unwrap_or(0.0);
                normals.push([x, y, z]);
            },
            "vt" => {
                let vals: Vec<&str> = it.collect();
                if vals.len() < 2 { continue; }
                let u: f32 = vals[0].parse().unwrap_or(0.0);
                let v: f32 = vals[1].parse().unwrap_or(0.0);
                texcoords.push([u, v]);
            },
            "f" => {
                // f can be: v | v/vt | v//vn | v/vt/vn, possibly with >3 vertices (polygons)
                let mut face = Vec::<FaceIndex>::new();
                for tok in it {
                    let parts: Vec<&str> = tok.split('/').collect();
                    let v = parse_idx(parts.get(0).copied(), vertices.len());
                    let vt = parse_idx(parts.get(1).copied(), texcoords.len());
                    let vn = parse_idx(parts.get(2).copied(), normals.len());
                    face.push(FaceIndex { v, vt, vn });
                }
                if !face.is_empty() {
                    faces.push(face);
                }
            },
            "usemtl" => {
                if let Some(name) = it.next() {
                    used_materials.push(name.to_string());
                }
            },
            "mtllib" => {
                for name in it {
                    mtllibs.push(name.to_string());
                }
            },
            "g" => {
                let name = it.collect::<Vec<&str>>().join(" ");
                if !name.is_empty() { groups.push(name); }
            },
            "o" => {
                let name = it.collect::<Vec<&str>>().join(" ");
                if !name.is_empty() { objects.push(name); }
            },
            _ => {
                // ignore others
                let _ = (lineno);
            }
        }
    }

    // de-dup materials
    used_materials.sort();
    used_materials.dedup();

    // Resolver rutas de MTL relativas al OBJ
    let base = path.parent().unwrap_or(Path::new("."));
    let mut mtl_paths: Vec<PathBuf> = Vec::new();
    for m in &mtllibs {
        let p = base.join(m);
        if p.exists() {
            mtl_paths.push(p);
        }
    }

    Ok((ObjData {
        vertices, normals, texcoords, faces,
        used_materials, groups, objects, mtllibs
    }, mtl_paths))
}

fn parse_idx(tok: Option<&str>, len: usize) -> Option<usize> {
    match tok {
        Some(t) if !t.is_empty() => {
            // OBJ indices can be positive (1-based) or negative (relative to end)
            if let Ok(val) = t.parse::<isize>() {
                if val > 0 {
                    Some((val as usize) - 1)
                } else if val < 0 {
                    // e.g. -1 -> last element
                    let idx = (len as isize) + val;
                    if idx >= 0 { Some(idx as usize) } else { None }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
