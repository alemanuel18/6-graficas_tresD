# Raycaster estilo Wolfenstein 3D

Pequeño motor de raycasting en Rust y Raylib. El nivel sigue siendo un mapa ASCII
2D, pero se representa como una escena de primera persona al lanzar un rayo por
cada columna de la pantalla.

## Ejecutar

```bash
cargo run
```

- `W`/`↑` y `S`/`↓`: avanzar y retroceder.
- `A`/`D`: desplazamiento lateral.
- `←`/`→`: girar.
- `M`: alternar entre la vista 3D y el mapa cenital.
- `Esc`: salir.

## Diseño

```text
main ──> controls ──> player + map
  │
  └──> renderer ──> raycaster ──> map
            │
            └──> framebuffer ──> textura de Raylib
```

- `map.rs`: carga `src/map/map.txt`, normaliza sus filas y ofrece consultas de
  paredes, punto de aparición y colisión. Los caracteres ` `, `p` y `g` son
  transitables; cualquier otro carácter es pared. `p` marca inicio y `g` meta.
- `player.rs`: contiene exclusivamente el estado de la cámara (`position`,
  `angle`, `fov`) y calcula su vector frontal.
- `controls.rs`: traduce entrada y tiempo transcurrido a movimiento. Comprueba
  primero X y luego Y para permitir deslizarse por paredes.
- `raycaster.rs`: implementa DDA. Salta de frontera de celda a frontera de
  celda, en vez de avanzar píxel por píxel, y devuelve distancia, punto de
  choque, símbolo de muro y orientación de la cara impactada.
- `renderer.rs`: convierte cada impacto en una columna vertical. Corrige la
  distancia con el coseno del ángulo para evitar el efecto de ojo de pez. El
  minimapa dibuja una muestra de rayos, jugador y dirección para depuración.
- `framebuffer.rs`: es un búfer RGBA8 de CPU. Se actualiza una textura de GPU
  persistente por fotograma; no se crean imágenes ni texturas dentro del bucle.

## Fórmula de proyección

Para cada columna se emite un rayo dentro del FOV. Si `d` es su distancia y
`Δa` su diferencia de ángulo respecto a la cámara, se usa
`d_perpendicular = d * cos(Δa)`. La altura de la pared es:

```text
altura = TILE_SIZE / d_perpendicular * plano_de_proyeccion
plano_de_proyeccion = ancho_pantalla / (2 * tan(FOV / 2))
```

Así, los muros cercanos ocupan más pantalla y los lejanos menos, simulando
profundidad sin geometría 3D ni una cámara 3D real.

## Verificación

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

Las pruebas cubren la semántica de paredes y una intersección DDA conocida.
