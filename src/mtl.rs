
use std::{fs, path::Path};
use anyhow::{Result, Context};

/// Devuelve la lista de nombres de materiales (`newmtl`) definidos en un .mtl
pub fn parse_mtl_names(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("No se pudo leer el archivo MTL {}", path.display()))?;

    let mut names = Vec::<String>::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let mut it = line.split_whitespace();
        if let Some(tag) = it.next() {
            if tag == "newmtl" {
                if let Some(name) = it.next() {
                    names.push(name.to_string());
                }
            }
        }
    }
    Ok(names)
}
