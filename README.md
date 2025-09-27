# obj_resumen

Programa en Rust que **lee un archivo OBJ (y su MTL opcional)** y **imprime en consola**:
- **Vectores (v)**
- **Materiales** (definidos en el/los .mtl y usados en el .obj con `usemtl`)
- **Normales (vn)**
- **Coordenadas de textura / UVs (vt)**
- **Caras (f)** con sus índices por vértice (v, vt, vn)
- **Grupos/Objetos** (`g`, `o`), si existen

## Uso

```bash
# Compilar
cargo build

# Ejecutar con un OBJ
cargo run -- ./examples/cubo.obj

# También funciona con cualquier otro .obj:
cargo run -- /ruta/a/tu/modelo.obj
```

> El programa resuelve rutas relativas del/los `mtllib` con base en la carpeta del OBJ.

## Salida de ejemplo (resumida)

```
Vectores:
  v 1: ( -1.0000, -1.0000,  1.0000 )
  v 2: (  1.0000, -1.0000,  1.0000 )
  ...

Materiales (definidos en MTL):
  - blanco
  - negro

Materiales usados en el OBJ (usemtl):
  - blanco

Normales:
  vn 1: ( 0.0000, 0.0000, 1.0000 )
  ...

UVs (vt):
  vt 1: ( 0.0000, 0.0000 )
  ...

Caras (índices a v/vt/vn):
  f 1: [ v=1,vt=1,vn=1 | v=2,vt=2,vn=1 | v=3,vt=3,vn=1 ]
  ...

Grupos/Objetos:
  Objetos (o): cubo
  Grupos  (g): grupo_cubo
```

## Notas
- El parser es **tolerante** a `f` con formatos `v`, `v/vt`, `v//vn`, `v/vt/vn` y a índices negativos (relativos al final).
- Solo se indexa; **no triangula** ni valida geometría. Útil para **resumen/inspección**.
- Si no hay MTL, igual imprime materiales usados según `usemtl` (si existen en el OBJ).




Imagen de prueba:





<img width="1996" height="1518" alt="image" src="https://github.com/user-attachments/assets/9df7c539-0aad-497a-8e28-67a9bc113e53" />
