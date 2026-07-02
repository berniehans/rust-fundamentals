# Arquitectura de Desarrollo: Configuración del Cargo Workspace

Este documento detalla la infraestructura técnica y el diseño del repositorio `rust-fundamentals`. La adopción de un **Cargo Workspace** es fundamental para estructurar el aprendizaje de forma modular, emulando la organización típica de proyectos industriales a gran escala en Rust.

---

## 1. Diseño del Workspace

El repositorio está organizado jerárquicamente de la siguiente manera:

```text
rust-fundamentals/
├── Cargo.toml (Manifiesto raíz del Workspace)
├── Cargo.lock
├── target/ (Directorio único de compilación compartido)
├── docs/ (Pilar 1: Guías teóricas del libro)
├── chapters/ (Pilar 2: Código idiomático comentado del libro)
│   └── ch01_getting_started/
│       ├── Cargo.toml
│       └── src/main.rs
├── exercises/ (Pilar 3: Retos algorítmicos con tests)
│   └── ex01_getting_started/
│       ├── Cargo.toml
│       └── src/lib.rs
└── architecture/ (Pilar 4: Documentación técnica de infraestructura)
    └── workspace_setup.md
```

### El Manifiesto Raíz (`Cargo.toml`)
El archivo `Cargo.toml` de la raíz no define un paquete ejecutable o de biblioteca en sí mismo, sino que actúa como el orquestador del espacio de trabajo utilizando la sección `[workspace]`:

```toml
[workspace]
resolver = "2"
members = [
    "chapters/ch01_getting_started",
    # ... otros capítulos
    "exercises/ex01_getting_started",
    # ... otros ejercicios
]
```

*   **`resolver = "2"`:** Configura el resolvedor de dependencias de Cargo de segunda generación (introducido de forma predeterminada en la Edición 2021). Este resolvedor evita que las características (*features*) activadas por dependencias de desarrollo (*dev-dependencies*) se propaguen innecesariamente a la compilación de producción normal de otros paquetes miembros del workspace, optimizando los tiempos y el tamaño de la compilación.
*   **`members`:** Lista explícitamente las rutas a todos los mini-crates del proyecto.

---

## 2. Beneficios de Bajo Nivel del Workspace

### Directorio `target/` Único y Compartido
En lugar de que cada crate independiente compile y genere sus propios artefactos de compilación redundantes en su propia carpeta, Cargo genera un único directorio `/target` en la raíz del workspace.
*   **Evita la Recompilación:** Si varios crates en el workspace comparten dependencias externas idénticas (como bibliotecas de serialización `serde`, manejo de errores `thiserror`, etc.), estas se compilan exactamente una vez y sus artefactos `.rlib` o archivos de metadatos se almacenan en el caché común del workspace.
*   **Consumo de Disco Inteligente:** Reduce dramáticamente el espacio ocupado en disco duro. Rustc genera metadatos extensos para la compilación incremental y el análisis estático; consolidar todo bajo un único directorio `target` optimiza enormemente este almacenamiento.

---

## 3. Flujo de Trabajo y Comandos Clave

Para interactuar de forma eficiente con esta arquitectura, se utilizan los siguientes comandos desde la raíz del proyecto:

### Análisis Estático (Rápido)
Comprueba que el código compila sintácticamente y cumple con el sistema de tipos, pero sin generar código de máquina nativo final. Esto ahorra mucho tiempo durante el desarrollo interactivo:
```bash
# Analiza todo el workspace
cargo check

# Analiza solo el código del capítulo 1
cargo check -p ch01_getting_started

# Analiza solo el ejercicio del capítulo 1
cargo check -p ex01_getting_started
```

### Ejecución
Compila en modo Debug (por defecto) y arranca el binario del capítulo correspondiente:
```bash
cargo run -p ch01_getting_started
```

### Pruebas Unitarias
Compila las dependencias de test y ejecuta todas las aserciones dentro de los bloques `#[cfg(test)]`:
```bash
# Corre todas las pruebas del workspace
cargo test

# Corre las pruebas del ejercicio de un capítulo específico
cargo test -p ex01_getting_started
```

### Formateador y Linter
Es fundamental mantener un estándar de calidad y consistencia en el código:
```bash
# Formatea el código de todos los miembros del workspace de acuerdo con las reglas de estilo de Rust
cargo fmt --all

# Analiza el código buscando antipatrones (code smells) y sugiere mejoras idiomáticas
cargo clippy --all-targets --all-features -- -D warnings
```
