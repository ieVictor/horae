# Horae - Rastreador de estudio para la terminal

[English](README.md) | [Русский](README-ru.md) | [中文](README-zh.md) | [Português (BR)](README-ptbr.md) | **Español**

Un rastreador de estudio para la terminal creado con Rust y Ratatui.

![pantalla del temporizador de horae](assets/tracking-your-time.png)

## Características

- **Temporizador de dígitos grandes** — el tiempo de la sesión se muestra con caracteres de bloque y el total diario aparece debajo
- **Asignaturas** — organiza las sesiones por asignatura; siempre está disponible la asignatura predeterminada "General"
- **Sesiones de estudio** — cada sesión se guarda en SQLite con hora de inicio, hora de fin y duración
- **Tareas** — lista de pendientes ligera con título, descripción, prioridad y estado hecha/abierta
- **Analíticas** — gráfico de barras semanal de las horas de estudio y el historial completo de sesiones anteriores

## Atajos de teclado

### Temporizador (inactivo)

| Tecla | Acción |
|-----|--------|
| `Space` | Abre el selector de asignaturas para iniciar una sesión |
| `a` | Abre las analíticas |
| `t` | Abre las tareas |
| `s` | Abre las asignaturas |
| `q` | Salir |

### Temporizador (estudiando)

| Tecla | Acción |
|-----|--------|
| `Space` | Detiene la sesión actual |
| `t` | Abre las tareas |
| `q` | Salir |

### Selector de asignaturas

| Tecla | Acción |
|-----|--------|
| `j` / `↓` | Bajar |
| `k` / `↑` | Subir |
| `Enter` | Inicia la sesión con la asignatura seleccionada |
| `Esc` / `q` | Cancelar |

### Analíticas / Tareas / Asignaturas

| Tecla | Acción |
|-----|--------|
| `j` / `↓` | Bajar |
| `k` / `↑` | Subir |
| `l` / `→` | Expande el elemento seleccionado |
| `h` / `←` / `Esc` | Contrae / vuelve atrás |
| `n` | Nuevo elemento (tareas y asignaturas) |
| `d` | Elimina el elemento seleccionado |
| `Space` | Alterna el estado de la tarea (solo tareas) |
| `q` | Salir |

## Compilar y ejecutar

Requiere Rust 1.88 o posterior.

```sh
cargo run --release
```

La base de datos SQLite (`horae.db`) se crea en el directorio de trabajo actual en la primera ejecución.
