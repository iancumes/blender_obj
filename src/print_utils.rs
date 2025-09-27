
use crate::obj::{FaceIndex};

pub fn print_title(title: &str) {
    println!("{}", title);
    println!("{}", "-".repeat(title.chars().count()));
}

pub fn print_vertices(vertices: &[[f32;3]]) {
    if vertices.is_empty() {
        println!("  (sin vértices)");
        return;
    }
    for (i, v) in vertices.iter().enumerate() {
        println!("  v {:>4}: ({:>8.4}, {:>8.4}, {:>8.4})", i+1, v[0], v[1], v[2]);
    }
}

pub fn print_normals(normals: &[[f32;3]]) {
    if normals.is_empty() {
        println!("  (sin normales)");
        return;
    }
    for (i, n) in normals.iter().enumerate() {
        println!("  vn {:>3}: ({:>8.4}, {:>8.4}, {:>8.4})", i+1, n[0], n[1], n[2]);
    }
}

pub fn print_uvs(texcoords: &[[f32;2]]) {
    if texcoords.is_empty() {
        println!("  (sin UVs)");
        return;
    }
    for (i, t) in texcoords.iter().enumerate() {
        println!("  vt {:>3}: ({:>8.4}, {:>8.4})", i+1, t[0], t[1]);
    }
}

pub fn print_faces(faces: &[Vec<FaceIndex>]) {
    if faces.is_empty() {
        println!("  (sin caras)");
        return;
    }
    for (i, face) in faces.iter().enumerate() {
        let parts: Vec<String> = face.iter().map(|fi| {
            let v  = fi.v.map(|x| (x+1).to_string()).unwrap_or_else(|| "-".into());
            let vt = fi.vt.map(|x| (x+1).to_string()).unwrap_or_else(|| "-".into());
            let vn = fi.vn.map(|x| (x+1).to_string()).unwrap_or_else(|| "-".into());
            format!("v={},vt={},vn={}", v, vt, vn)
        }).collect();
        println!("  f {:>4}: [ {} ]", i+1, parts.join(" | "));
    }
}
