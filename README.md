# Raycaster estilo Wolfenstein 3D

Proyecto final de la tercera parte del curso: un ray caster jugable en Rust y
Raylib, con dos niveles ASCII representados en primera persona. El motor lanza
un rayo DDA por cada columna, muestrea la textura de paredes de `Assets`,
proyecta sprites enemigos y mantiene la colisión en el plano 2D.

## Ejecutar

```bash
cargo run
```

- `1`/`2`: seleccionar nivel en el menú o arma durante la partida.
- `W`/`↑` y `S`/`↓`: avanzar y retroceder.
- `A`/`D`: desplazamiento lateral.
- `Mouse`: rotación horizontal; `←`/`→` también funcionan.
- `Click`/`Espacio`: disparar. La pistola dispara rápido y hace 25 de daño;
  la escopeta hace 70, pero recarga más lento.
- `Enter`: iniciar o repetir nivel. `M`/`Backspace`: volver al menú desde
  victoria/derrota. `Esc`: cerrar el juego.
- La mirilla se dibuja en el centro; cambia a rojo cuando apunta a un enemigo
  visible y dentro del alcance del rayo.

## Diseño

```text
main ──> game ──> controls ──> player + map
  │              └──> enemigos + armas + estados
  └──> renderer ──> raycaster ──> framebuffer ──> textura de Raylib
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
- `game.rs`: reglas de niveles, vida, enemigos, jefes, armas, disparo y estados
  de victoria/derrota.
- `renderer.rs`: convierte cada impacto en una columna vertical, recorta el
  atlas de paredes por cuadros de 65 px, selecciona la textura según el tipo
  de muro y orientación, proyecta sprites animados y dibuja el minimapa. El
  atlas SS se interpreta como 8 columnas por 7 filas y el del jefe como 4 por 3.
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

Los símbolos `e` y `b` de los mapas son enemigos y jefe. Se consideran celdas
transitables para DDA y colisión, mientras el resto de símbolos continúa siendo
una pared. Los WAV disponibles se cargan al arrancar y se reproducen al usar
cada arma y al completar el nivel.

## Verificación

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

Las pruebas cubren la semántica de paredes y una intersección DDA conocida.
