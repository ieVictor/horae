[English](README.md) | [Português](README.pt-BR.md) | **Español**

# Horae - Seguimiento de estudio en la terminal

Un seguimiento de estudio para la terminal, construido con Rust y Ratatui.

![horae timer screen](assets/tracking-your-time.png)

## Características

- **Temporizador con dígitos grandes** — el tiempo de la sesión se muestra con caracteres de bloque, con el total del día debajo
- **Materias** — organiza las sesiones por materia; la materia predeterminada "General" siempre está disponible
- **Sesiones de estudio** — cada sesión se guarda en SQLite con hora de inicio, hora de fin y duración
- **Tareas** — lista de tareas sencilla con título, descripción, prioridad y estado (hecha/abierta)
- **Analíticas** — gráfico de barras semanal de las horas de estudio y el historial completo de sesiones anteriores

## Atajos de teclado

### Temporizador (inactivo)

| Tecla | Acción |
|-----|--------|
| `Space` | Abrir el selector de materia para iniciar una sesión |
| `a` | Abrir analíticas |
| `t` | Abrir tareas |
| `s` | Abrir materias |
| `q` | Salir |

### Temporizador (estudiando)

| Tecla | Acción |
|-----|--------|
| `Space` | Detener la sesión actual |
| `t` | Abrir tareas |
| `q` | Salir |

### Selector de materia

| Tecla | Acción |
|-----|--------|
| `j` / `↓` | Mover hacia abajo |
| `k` / `↑` | Mover hacia arriba |
| `Enter` | Iniciar la sesión con la materia seleccionada |
| `Esc` / `q` | Cancelar |

### Analíticas / Tareas / Materias

| Tecla | Acción |
|-----|--------|
| `j` / `↓` | Mover hacia abajo |
| `k` / `↑` | Mover hacia arriba |
| `l` / `→` | Expandir el elemento seleccionado |
| `h` / `←` / `Esc` | Contraer / volver |
| `n` | Nuevo elemento (tareas y materias) |
| `d` | Eliminar el elemento seleccionado |
| `Space` | Alternar el estado de la tarea (solo en tareas) |
| `q` | Salir |

## Compilar y ejecutar

Requiere Rust 1.88 o superior.

```sh
cargo run --release
```

La base de datos SQLite (`horae.db`) se crea en el directorio de trabajo actual la primera vez que se ejecuta.
