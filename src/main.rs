
use std::{env, fs, path::{Path, PathBuf}};
use anyhow::{bail, Context, Result};

mod obj;
mod mtl;
mod print_utils;

use obj::{ObjData, parse_obj};
use mtl::parse_mtl_names;
use print_utils::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <ruta/al/archivo.obj>", args.get(0).unwrap_or(&"obj_resumen".to_string()));
        eprintln!("Ejemplo: {} ./examples/cubo.obj", args.get(0).unwrap_or(&"obj_resumen".to_string()));
        bail!("Falta la ruta al .obj");
    }

    let obj_path = Path::new(&args[1]);
    if !obj_path.exists() {
        bail!("No se encontró el archivo: {}", obj_path.display());
    }

    // 1) Parsear OBJ
    let (data, mtl_paths) = parse_obj(obj_path)
        .with_context(|| format!("No se pudo parsear el OBJ: {}", obj_path.display()))?;

    // 2) Parsear nombres de materiales del/los MTL referenciados
    let mut mtl_defined_names = vec![];
    for mtl in mtl_paths {
        match parse_mtl_names(&mtl) {
            Ok(mut names) => mtl_defined_names.append(&mut names),
            Err(e) => {
                eprintln!("[Aviso] No se pudo leer MTL {}: {}", mtl.display(), e);
            }
        }
    }
    mtl_defined_names.sort();
    mtl_defined_names.dedup();

    // 3) Imprimir
    print_title("Vectores");
    print_vertices(&data.vertices);

    print_title("Materiales (definidos en MTL)");
    if mtl_defined_names.is_empty() {
        println!("  (sin MTL o sin 'newmtl')");
    } else {
        for name in &mtl_defined_names {
            println!("  - {}", name);
        }
    }

    print_title("Materiales usados en el OBJ (usemtl)");
    if data.used_materials.is_empty() {
        println!("  (no hay 'usemtl' en el OBJ)");
    } else {
        for m in &data.used_materials {
            println!("  - {}", m);
        }
    }

    print_title("Normales");
    print_normals(&data.normals);

    print_title("UVs (vt)");
    print_uvs(&data.texcoords);

    print_title("Caras (índices a v/vt/vn)");
    print_faces(&data.faces);

    print_title("Grupos/Objetos");
    if !data.objects.is_empty() {
        println!("  Objetos (o): {}", data.objects.join(", "));
    } else {
        println!("  Objetos (o): (ninguno)");
    }
    if !data.groups.is_empty() {
        println!("  Grupos  (g): {}", data.groups.join(", "));
    } else {
        println!("  Grupos  (g): (ninguno)");
    }

    Ok(())
}
