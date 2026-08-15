# 🦀 rust-fundamentals

[![Rust](https://img.shields.io/badge/rust-v1.96.0+-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Cargo Workspace](https://img.shields.io/badge/Cargo-Workspace-blue.svg?style=for-the-badge&logo=rust)](https://doc.rust-lang.org/cargo/reference/workspaces.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/berniehans/rust-fundamentals/actions/workflows/ci.yml/badge.svg)](https://github.com/berniehans/rust-fundamentals/actions/workflows/ci.yml)

¡Bienvenido/a a **rust-fundamentals**! Este repositorio es una bitácora de estudio completa, estructurada y en español del libro oficial [**"The Rust Programming Language"**](https://doc.rust-lang.org/book/). 

El proyecto ha sido organizado como un **Cargo Workspace** (Espacio de Trabajo) modular, dividiendo cada capítulo del libro en un subproyecto (*crate*) independiente dentro de la carpeta `chapters/` y sus retos prácticos en `exercises/`. Esto te permite repasar y ejecutar ejemplos de código de forma aislada, limpia y con tiempos de compilación optimizados gracias a la caché global.

---

## 📂 Estructura del Workspace

El repositorio está organizado en cuatro pilares complementarios:

```bash
rust-fundamentals/
├── Cargo.toml                # Manifiesto raíz y orquestador global del Workspace
├── book.toml                 # Configuración de mdBook para renderizar la documentación web
├── LICENSE                   # Licencia de código abierto MIT
├── README.md                 # Guía general del proyecto
├── setup_workspace.sh        # Script Bash para inicializar el workspace (Linux/macOS/Git Bash)
├── setup_workspace.ps1       # Script PowerShell para inicializar el workspace (Windows)
├── docs/                     # Pilar 1: Guías teóricas exhaustivas en español (20 capítulos)
├── exercises/                # Pilar 3: Retos algorítmicos con tests unitarios y doctests (20 crates)
├── architecture/             # Pilar 4: Documentación técnica de infraestructura del workspace
└── chapters/                 # Pilar 2: Código idiomático y ejecutable del libro (20 crates)
    ├── ch01_getting_started/
    ├── ch02_guessing_game/
    ├── ...
    └── ch20_web_server/
```

---

## 🗺️ Bitácora de Estudio: Mapeo de Capítulos

A continuación se muestra la correspondencia de los 20 capítulos del libro con su respectivo crate dentro de `chapters/`:

| Capítulo | Título Oficial del Libro | Código (Crate) | Guía Teórica (Docs) | Ejercicios (Crate) |
| :---: | :--- | :--- | :--- | :--- |
| **01** | Getting Started | [ch01_getting_started](chapters/ch01_getting_started) | [ch01_getting_started.md](docs/ch01_getting_started.md) | [ex01_getting_started](exercises/ex01_getting_started) |
| **02** | Programming a Guessing Game | [ch02_guessing_game](chapters/ch02_guessing_game) | [ch02_guessing_game.md](docs/ch02_guessing_game.md) | [ex02_guessing_game](exercises/ex02_guessing_game) |
| **03** | Common Programming Concepts (Variables, Control de flujo) | [ch03_common_concepts](chapters/ch03_common_concepts) | [ch03_common_concepts.md](docs/ch03_common_concepts.md) | [ex03_common_concepts](exercises/ex03_common_concepts) |
| **04** | Understanding Ownership (Ownership, Préstamos y Slices) | [ch04_understanding_ownership](chapters/ch04_understanding_ownership) | [ch04_understanding_ownership.md](docs/ch04_understanding_ownership.md) | [ex04_understanding_ownership](exercises/ex04_understanding_ownership) |
| **05** | Using Structs to Structure Related Data | [ch05_using_structs](chapters/ch05_using_structs) | [ch05_using_structs.md](docs/ch05_using_structs.md) | [ex05_using_structs](exercises/ex05_using_structs) |
| **06** | Enums and Pattern Matching | [ch06_enums_patterns](chapters/ch06_enums_patterns) | [ch06_enums_pattern_matching.md](docs/ch06_enums_pattern_matching.md) | [ex06_enums_patterns](exercises/ex06_enums_patterns) |
| **07** | Managing Growing Projects (Crates, Módulos, Visibilidad) | [ch07_managing_projects](chapters/ch07_managing_projects) | [ch07_managing_growing_projects.md](docs/ch07_managing_growing_projects.md) | [ex07_managing_projects](exercises/ex07_managing_projects) |
| **08** | Common Collections (Vectores, Strings, HashMaps) | [ch08_common_collections](chapters/ch08_common_collections) | [ch08_common_collections.md](docs/ch08_common_collections.md) | [ex08_common_collections](exercises/ex08_common_collections) |
| **09** | Error Handling (panic! y Result) | [ch09_error_handling](chapters/ch09_error_handling) | [ch09_error_handling.md](docs/ch09_error_handling.md) | [ex09_error_handling](exercises/ex09_error_handling) |
| **10** | Generic Types, Traits, and Lifetimes | [ch10_generics_traits_lifetimes](chapters/ch10_generics_traits_lifetimes) | [ch10_generics_traits_lifetimes.md](docs/ch10_generics_traits_lifetimes.md) | [ex10_generics_traits_lifetimes](exercises/ex10_generics_traits_lifetimes) |
| **11** | Writing Automated Tests | [ch11_writing_tests](chapters/ch11_writing_tests) | [ch11_writing_tests.md](docs/ch11_writing_tests.md) | [ex11_writing_tests](exercises/ex11_writing_tests) |
| **12** | An I/O Project: Building a Command Line Program (Minigrep) | [ch12_minigrep](chapters/ch12_minigrep) | [ch12_minigrep.md](docs/ch12_minigrep.md) | [ex12_minigrep](exercises/ex12_minigrep) |
| **13** | Functional Features: Closures & Iterators | [ch13_functional_features](chapters/ch13_functional_features) | [ch13_functional_features.md](docs/ch13_functional_features.md) | [ex13_functional_features](exercises/ex13_functional_features) |
| **14** | More about Cargo and Crates.io | [ch14_cargo_more](chapters/ch14_cargo_more) | [ch14_cargo_more.md](docs/ch14_cargo_more.md) | [ex14_cargo_more](exercises/ex14_cargo_more) |
| **15** | Smart Pointers (Box, Rc, RefCell, Weak) | [ch15_smart_pointers](chapters/ch15_smart_pointers) | [ch15_smart_pointers.md](docs/ch15_smart_pointers.md) | [ex15_smart_pointers](exercises/ex15_smart_pointers) |
| **16** | Fearless Concurrency (Hilos, Mutex, Canales MPSC) | [ch16_fearless_concurrency](chapters/ch16_fearless_concurrency) | [ch16_fearless_concurrency.md](docs/ch16_fearless_concurrency.md) | [ex16_fearless_concurrency](exercises/ex16_fearless_concurrency) |
| **17** | Object-Oriented Programming Features (Trait Objects & Typestate) | [ch17_oop_features](chapters/ch17_oop_features) | [ch17_oop_features.md](docs/ch17_oop_features.md) | [ex17_oop_features](exercises/ex17_oop_features) |
| **18** | Patterns and Matching (Pattern Matching & Guards) | [ch18_patterns_matching](chapters/ch18_patterns_matching) | [ch18_patterns_matching.md](docs/ch18_patterns_matching.md) | [ex18_patterns_matching](exercises/ex18_patterns_matching) |
| **19** | Advanced Features (Unsafe Rust, Advanced Traits, Macros) | [ch19_advanced_features](chapters/ch19_advanced_features) | [ch19_advanced_features.md](docs/ch19_advanced_features.md) | [ex19_advanced_features](exercises/ex19_advanced_features) |
| **20** | Final Project: Building a Multithreaded Web Server | [ch20_web_server](chapters/ch20_web_server) | [ch20_web_server.md](docs/ch20_web_server.md) | [ex20_web_server](exercises/ex20_web_server) |

---

## 🚀 Guía de Inicio Rápido

### 1. Clonar el repositorio

```bash
git clone https://github.com/berniehans/rust-fundamentals.git
cd rust-fundamentals
```

### 2. Comandos Globales del Cargo Workspace

Al usar un Cargo Workspace, puedes interactuar con todo el proyecto o con mini-proyectos individuales desde la raíz:

```bash
# 🔍 Verificar sintaxis y tipos en todo el workspace (Rápido)
cargo check --workspace

# 🏃 Correr un capítulo específico (ej: Servidor Web Multihilo)
cargo run -p ch20_web_server

# 🧪 Ejecutar todas las pruebas unitarias y doctests de los 20 ejercicios
cargo test --workspace

# 🧹 Linter de código limpio y buenas prácticas
cargo clippy --workspace -- -D warnings

# 🎨 Formatear todo el código de acuerdo al estándar de Rust
cargo fmt --all
```

---

## 📖 Visualizar la Documentación Web Interactiva (`mdBook`)

Las guías teóricas de `docs/` están preparadas para ser renderizadas como un libro digital mediante **`mdBook`**:

```bash
# 1. Instalar mdBook (si aún no lo tienes instalado)
cargo install mdbook

# 2. Servir el libro localmente y abrirlo en tu navegador
mdbook serve --open
```

El repositorio cuenta además con integración continua que compila y despliega automáticamente la documentación en **GitHub Pages**.

---

## 📚 Recursos Oficiales
- [Libro Oficial en Inglés (The Rust Programming Language)](https://doc.rust-lang.org/book/)
- [Rust by Example (Aprende con ejemplos interactivos)](https://doc.rust-lang.org/rust-by-example/)
- [Standard Library Documentation (std)](https://doc.rust-lang.org/std/)

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.
