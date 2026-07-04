# 🦀 rust-fundamentals

[![Rust](https://img.shields.io/badge/rust-v1.96.0+-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Cargo Workspace](https://img.shields.io/badge/Cargo-Workspace-blue.svg?style=for-the-badge&logo=rust)](https://doc.rust-lang.org/cargo/reference/workspaces.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

¡Bienvenido/a a **rust-fundamentals**! Este repositorio es una bitácora de estudio completa y estructurada del libro oficial [**"The Rust Programming Language"**](https://doc.rust-lang.org/book/). 

El proyecto ha sido organizado como un **Cargo Workspace** (Espacio de Trabajo) modular, dividiendo cada capítulo del libro en un subproyecto (*crate*) independiente dentro de la carpeta `chapters/`. Esto te permite repasar y ejecutar ejemplos de código de forma aislada y limpia.

---

## 📂 Estructura del Workspace

El repositorio tiene la siguiente estructura raíz:

```bash
rust-fundamentals/
├── Cargo.toml                # Configuración global del Workspace
├── setup_workspace.sh        # Script en Bash para inicializar el workspace (Unix/Git Bash)
├── setup_workspace.ps1       # Script en PowerShell para inicializar el workspace (Windows)
├── README.md                 # Guía general de estudio
├── docs/                     # Pilar 1: Guías teóricas completas en español (ej: docs/ch01_...)
├── exercises/                # Pilar 3: Retos algorítmicos con tests unitarios (ej: exercises/ex01_...)
└── chapters/                 # Pilar 2: Código del libro masivamente comentado (crates)
    ├── ch01_getting_started/
    ├── ch02_guessing_game/
    ├── ...
    └── ch20_web_server/
```

---

## 🗺️ Bitácora de Estudio: Mapeo de Capítulos

A continuación se muestra la correspondencia de los capítulos del libro con su respectivo crate dentro de `chapters/`:

| Capítulo | Título Oficial del Libro | Código (Crate) | Guía Teórica (Docs) | Ejercicios (Crate) |
| :---: | :--- | :--- | :--- | :--- |
| **01** | Getting Started | [ch01_getting_started](chapters/ch01_getting_started) | [ch01_getting_started.md](docs/ch01_getting_started.md) | [ex01_getting_started](exercises/ex01_getting_started) |
| **02** | Programming a Guessing Game | [ch02_guessing_game](chapters/ch02_guessing_game) | [ch02_guessing_game.md](docs/ch02_guessing_game.md) | [ex02_guessing_game](exercises/ex02_guessing_game) |
| **03** | Common Programming Concepts (Variables, Control de flujo) | [ch03_common_concepts](chapters/ch03_common_concepts) | [ch03_common_concepts.md](docs/ch03_common_concepts.md) | [ex03_common_concepts](exercises/ex03_common_concepts) |
| **04** | Understanding Ownership (El corazón de Rust: Ownership, Slices) | [ch04_understanding_ownership](chapters/ch04_understanding_ownership) | [ch04_understanding_ownership.md](docs/ch04_understanding_ownership.md) | [ex04_understanding_ownership](exercises/ex04_understanding_ownership) |
| **05** | Using Structs to Structure Related Data | [ch05_using_structs](chapters/ch05_using_structs) | [ch05_using_structs.md](docs/ch05_using_structs.md) | [ex05_using_structs](exercises/ex05_using_structs) |
| **06** | Enums and Pattern Matching | [ch06_enums_patterns](chapters/ch06_enums_patterns) | [ch06_enums_pattern_matching.md](docs/ch06_enums_pattern_matching.md) | [ex06_enums_patterns](exercises/ex06_enums_patterns) |
| **07** | Managing Growing Projects (Crates, Módulos, Scope) | [ch07_managing_projects](chapters/ch07_managing_projects) | [ch07_managing_growing_projects.md](docs/ch07_managing_growing_projects.md) | [ex07_managing_projects](exercises/ex07_managing_projects) |
| **08** | Common Collections (Vectores, Strings, HashMaps) | [ch08_common_collections](chapters/ch08_common_collections) | [ch08_common_collections.md](docs/ch08_common_collections.md) | [ex08_common_collections](exercises/ex08_common_collections) |
| **09** | Error Handling (panic! y Result) | [ch09_error_handling](chapters/ch09_error_handling) | [ch09_error_handling.md](docs/ch09_error_handling.md) | [ex09_error_handling](exercises/ex09_error_handling) |
| **10** | Generic Types, Traits, and Lifetimes | [ch10_generics_traits_lifetimes](chapters/ch10_generics_traits_lifetimes) | [ch10_generics_traits_lifetimes.md](docs/ch10_generics_traits_lifetimes.md) | [ex10_generics_traits_lifetimes](exercises/ex10_generics_traits_lifetimes) |
| **11** | Writing Automated Tests | [ch11_writing_tests](chapters/ch11_writing_tests) | [ch11_writing_tests.md](docs/ch11_writing_tests.md) | [ex11_writing_tests](exercises/ex11_writing_tests) |
| **12** | An I/O Project: Building a Command Line Program | [ch12_minigrep](chapters/ch12_minigrep) | [ch12_minigrep.md](docs/ch12_minigrep.md) | [ex12_minigrep](exercises/ex12_minigrep) |
| **13** | Functional Features: Closures & Iterators | [ch13_functional_features](chapters/ch13_functional_features) | [ch13_functional_features.md](docs/ch13_functional_features.md) | [ex13_functional_features](exercises/ex13_functional_features) |
| **14** | More about Cargo and Crates.io | [ch14_cargo_more](chapters/ch14_cargo_more) | [ch14_cargo_more.md](docs/ch14_cargo_more.md) | [ex14_cargo_more](exercises/ex14_cargo_more) |
| **15** | Smart Pointers (Box, Rc, RefCell, etc.) | [ch15_smart_pointers](chapters/ch15_smart_pointers) | [ch15_smart_pointers.md](docs/ch15_smart_pointers.md) | [ex15_smart_pointers](exercises/ex15_smart_pointers) |
| **16** | Fearless Concurrency (Threads, Mutex, Canales MPSC) | [ch16_fearless_concurrency](chapters/ch16_fearless_concurrency) | [ch16_fearless_concurrency.md](docs/ch16_fearless_concurrency.md) | [ex16_fearless_concurrency](exercises/ex16_fearless_concurrency) |
| **17** | Object-Oriented Programming Features | [ch17_oop_features](chapters/ch17_oop_features) | [ch17_oop_features.md](docs/ch17_oop_features.md) | [ex17_oop_features](exercises/ex17_oop_features) |
| **18** | Patterns and Matching | [ch18_patterns_matching](chapters/ch18_patterns_matching) | [ch18_patterns_matching.md](docs/ch18_patterns_matching.md) | [ex18_patterns_matching](exercises/ex18_patterns_matching) |
| **19** | Advanced Features (Unsafe Rust, Advanced Traits, Macros) | [ch19_advanced_features](chapters/ch19_advanced_features) | [ch19_advanced_features.md](docs/ch19_advanced_features.md) | [ex19_advanced_features](exercises/ex19_advanced_features) |
| **20** | Final Project: Building a Multithreaded Web Server | [ch20_web_server](chapters/ch20_web_server) | [ch20_web_server.md](docs/ch20_web_server.md) | [ex20_web_server](exercises/ex20_web_server) |

---

## 🚀 Guía de Inicio Rápido

### 1. Clonar el repositorio

```bash
git clone https://github.com/berniehans/rust-fundamentals.git
cd rust-fundamentals
```

### 2. Inicialización del Workspace (Scripts Automáticos)

Si deseas recrear o inicializar de nuevo el espacio de trabajo, cuentas con dos scripts automatizados según tu sistema operativo:

#### En Linux / macOS / Git Bash para Windows:
Asegúrate de dar permisos de ejecución al script antes de correrlo:
```bash
chmod +x setup_workspace.sh
./setup_workspace.sh
```

#### En Windows (PowerShell):
Ejecuta el script saltándote la política de restricción de scripts si es necesario:
```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\setup_workspace.ps1
```

*(Nota: Ambos scripts crearán la carpeta `chapters/`, generarán los subproyectos necesarios usando `cargo new` y eliminarán los directorios `src/` temporales sobrantes en la raíz).*

---

## 🛠️ Comandos Útiles de Cargo Workspace

Al usar un Cargo Workspace, puedes compilar, revisar y ejecutar proyectos individuales directamente desde la raíz del repositorio pasando la bandera `-p` (o `--package`) seguida del nombre del crate.

### 🔍 Verificar un capítulo específico (Check)
Realiza un chequeo rápido de sintaxis y tipos sin generar código máquina:
```bash
cargo check -p ch01_getting_started
```

### 🔨 Compilar un capítulo específico
Compila los binarios para un capítulo específico:
```bash
cargo build -p ch01_getting_started
```

### 🏃 Ejecutar un capítulo específico
Compila (si es necesario) y corre el binario asociado a un capítulo:
```bash
cargo run -p ch01_getting_started
```

### 🧪 Ejecutar pruebas automatizadas en un capítulo
Si el capítulo cuenta con tests unitarios o de integración, córrelos con:
```bash
cargo test -p ex01_getting_started
```

### 🧹 Limpiar todo el Workspace
Limpia todos los artefactos de compilación generados en la carpeta `target/` global:
```bash
cargo clean
```

---

## 💡 Ejemplo Destacado: Empezando con Rust (`ch01_getting_started`)

Como punto de partida didáctico, el módulo de **Empezando con Rust** (`ch01_getting_started`) está completamente implementado con comentarios explicativos detallados en español. Te muestra de forma práctica:
- La estructura elemental del punto de entrada `fn main()`.
- El uso y propósito de macros en Rust (como `println!`) frente a funciones comunes.
- Los fundamentos del proceso de compilación AOT y enlazado estático de bajo nivel.

Para ver este ejemplo en acción, simplemente ejecuta:
```bash
cargo run -p ch01_getting_started
```

---

## 📚 Recursos Adicionales
- [Libro Oficial en Inglés (The Rust Programming Language)](https://doc.rust-lang.org/book/)
- [Rust by Example (Aprende con ejemplos interactivos)](https://doc.rust-lang.org/rust-by-example/)
