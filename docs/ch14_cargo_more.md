# Capítulo 14: Más sobre Cargo y Crates.io

Este documento proporciona un análisis exhaustivo de Cargo, el ecosistema de distribución Crates.io y las técnicas avanzadas de empaquetado de proyectos en Rust. Se detallan las opciones de optimización de compilación, la generación de documentación ejecutable (Doctests), la re-exportación de APIs públicas y el diseño de espacios de trabajo estructurados (Cargo Workspaces).

---

## 1. Conceptos Fundamentales (Desde Cero)

### Cargo como Herramienta de Grado Industrial
A diferencia de otros lenguajes de programación de sistemas que delegan la compilación y la gestión de librerías a herramientas fragmentadas de terceros (como Make, CMake, Conan o NuGet), Rust provee **Cargo**. Cargo no es solo un gestor de dependencias; es una suite completa de orquestación que unifica:
*   La compilación de código fuente llamando internamente a `rustc`.
*   La descarga y actualización automática de dependencias externas.
*   El formateo de código (`cargo fmt`) y el análisis de estilo y buenas prácticas (`cargo clippy`).
*   La ejecución de suites de prueba, benchmarks y la generación de documentación.

### Ecosistema Crates.io
Toda librería pública de código abierto compartida por la comunidad reside en **Crates.io**, el registro central de paquetes de Rust. Cargo está preconfigurado para buscar y descargar cualquier dependencia declarada en este registro, promoviendo una rápida reutilización de código de manera segura.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Perfiles de Compilación (Release Profiles)
Los perfiles de compilación son conjuntos preconfigurados de directivas que permiten al desarrollador tener control fino sobre los parámetros de compilación de LLVM. Por defecto, Cargo tiene dos perfiles predefinidos:
1.  **`dev`:** Optimizado para el desarrollo rápido (usado en `cargo build`).
2.  **`release`:** Optimizado para la ejecución en producción (usado en `cargo build --release`).

Se pueden personalizar agregando secciones en el archivo `Cargo.toml` de la raíz:

```toml
# Configuración del perfil de desarrollo
[profile.dev]
opt-level = 0      # 0 = Sin optimizaciones (compilación ultra rápida)
panic = "unwind"   # Limpieza de pila activa en pánicos

# Configuración del perfil de producción
[profile.release]
opt-level = 3      # 3 = Optimizaciones agresivas de LLVM (código máquina ultra rápido)
panic = "abort"    # Aborta inmediatamente en pánicos (reduce tamaño del binario)
lto = true         # Activa Link-Time Optimization para optimización global de enlazado
```

---

### 2. Documentación Avanzada de Código (`rustdoc`)
Rust cuenta con la herramienta integrada `rustdoc`. Al correr `cargo doc`, esta herramienta procesa los comentarios especiales de documentación y genera un sitio web estático en HTML de apariencia idéntica a la documentación oficial.

#### Comentarios de Item (`///`) y Comentarios de Crate/Módulo (`//!`)
*   **`///` (Comentarios de Item):** Se sitúan inmediatamente arriba del elemento que documentan (funciones, structs, enums).
*   **`//!` (Comentarios de Crate/Módulo):** Se colocan al inicio del archivo (ej. primera línea de `src/lib.rs`) para documentar el propósito general del módulo o de la librería en sí.

```rust
//! # Mi Super Biblioteca Científica
//!
//! Esta biblioteca proporciona utilidades matemáticas avanzadas
//! diseñadas para cálculos físicos de alto rendimiento.

/// Suma dos enteros de 32 bits de forma segura.
///
/// # Examples
///
/// ```
/// let resultado = mi_crate::sumar(2, 3);
/// assert_eq!(resultado, 5);
/// ```
///
/// # Panics
///
/// Este método provocará pánico si el resultado excede los límites de `i32`.
pub fn sumar(a: i32, b: i32) -> i32 {
    a.checked_add(b).expect("Desbordamiento en suma")
}
```

#### El Poder de los Doctests (Pruebas de Documentación)
Las secciones marcadas bajo la anotación de bloque de código ` ``` ` dentro de comentarios de documentación **son ejecutadas como pruebas unitarias reales por `cargo test`**. 
*   Si modificas la firma de una función pero olvidas actualizar el ejemplo en su comentario de documentación, `cargo test` fallará.
*   Esto garantiza que los ejemplos de código provistos en tu documentación sigan siendo funcionales a lo largo del tiempo.

---

### 3. Re-exportación de APIs Públicas (`pub use`)
A menudo, la jerarquía de directorios y módulos interna de tu proyecto es compleja y jerárquica para mantener el código organizado en desarrollo. Sin embargo, obligar al programador externo a importar elementos a través de rutas extensas es molesto y expone detalles de implementación internos:

```rust
// Sin re-exportar: importación extensa y compleja
use mi_crate::utilidades::matematicas::aritmetica::sumar;
```

Para aplanar la estructura externa del API, usamos **`pub use`** en el módulo raíz (`src/lib.rs`), creando un atajo público directo:

```rust
// src/lib.rs
mod utilidades {
    pub mod matematicas {
        pub mod aritmetica {
            pub fn sumar(a: i32, b: i32) -> i32 { a + b }
        }
    }
}

// Re-exportamos para aplanar el acceso público
pub use self::utilidades::matematicas::aritmetica::sumar;
```

Ahora el cliente de tu crate puede consumirlo directamente:
```rust
use mi_crate::sumar; // Acceso limpio
```

---

### 4. Publicar en Crates.io
Antes de subir un crate a Crates.io, debes registrarte y configurar los metadatos obligatorios en `Cargo.toml`:

```toml
[package]
name = "mi_super_aritmetica_didactica"
version = "0.1.0"
authors = ["Tu Nombre <tu-correo@ejemplo.com>"]
edition = "2021"
description = "Una biblioteca didáctica para operaciones matemáticas de bajo costo."
license = "MIT"
repository = "https://github.com/tu-usuario/mi_super_aritmetica"
```

#### Comandos Clave:
1.  **`cargo login <token>`**: Autentica tu máquina local con tu cuenta de Crates.io.
2.  **`cargo publish`**: Empaqueta tu código, realiza una compilación limpia local y sube el paquete a la plataforma. **Atención:** Una vez publicado, el código es permanente; no se puede eliminar ni modificar la versión para evitar romper proyectos de terceros que dependan de él.
3.  **`cargo yank --version <version>`**: Si una versión publicada contiene un bug grave, puedes "tirar" (yank) de ella. Esto no elimina el código de Crates.io ni rompe builds existentes que ya la tengan fijada, pero **impide que cualquier nuevo proyecto la descargue** como dependencia inicial.

---

### 5. Espacios de Trabajo de Cargo (Cargo Workspaces)
Un espacio de trabajo es un conjunto de subproyectos (crates) que comparten un único archivo de resolución de dependencias `Cargo.lock` y un directorio de compilación `target/` global. Es ideal para organizar monorepos.

#### Estructura del `Cargo.toml` raíz de un Workspace:
```toml
[workspace]
members = [
    "chapters/ch01_getting_started",
    "chapters/ch02_guessing_game",
    "chapters/ch14_cargo_more",
]
```

*   **Coherencia de dependencias:** Todos los crates hijos comparten el mismo `Cargo.lock`. Si tres submódulos utilizan la librería `serde`, al compilar el proyecto global se descargará y compilará **una sola vez**, acelerando significativamente los tiempos de compilación.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### `opt-level` y el Proceso de Optimización de LLVM
El parámetro `opt-level` en los perfiles indica a LLVM qué transformaciones matemáticas y de flujo de control aplicar sobre la representación intermedia (IR) de tu código Rust antes de generar el ensamblador final:

*   **`opt-level = 0` (Desarrollo):** No se realiza inlining de funciones. Las variables locales permanecen en celdas de Stack dedicadas para facilitar la depuración física mediante inspectores de memoria como GDB o LLDB.
*   **`opt-level = 3` (Producción):** LLVM aplica optimizaciones complejas:
    *   **Inlining agresivo:** Elimina la instrucción de llamada de CPU de funciones pequeñas sustituyéndolas por sus instrucciones directas.
    *   **Vectorización SIMD:** Agrupa operaciones matemáticas secuenciales para ejecutarlas en registros vectoriales especiales del hardware (ej: SSE, AVX en x86) de forma paralela en un solo ciclo de CPU.
    *   **Eliminación de código muerto:** Remueve ramificaciones de control de flujo que matemáticamente nunca se alcanzarán.

#### Link-Time Optimization (LTO)
Por defecto, Rust compila y optimiza cada crate de manera aislada como una unidad independiente. Cuando activas `lto = true` en `Cargo.toml`:
*   El compilador retrasa la optimización de código máquina final hasta la fase de enlazado global (*linking*).
*   LLVM analiza todo el binario final unificado, lo que le permite realizar inlining de funciones a través de las fronteras físicas de diferentes crates (ej: inlinear una función de un crate externo de crates.io en tu ejecutable principal).
*   Esto produce binarios mucho más compactos y veloces, a costa de un incremento masivo en el uso de memoria RAM y tiempo al momento de compilar la versión definitiva.

### Reproducibilidad de Compilación mediante `Cargo.lock`
*   **`Cargo.toml`:** Especifica qué dependencias *necesitas* y qué rangos de versión son aceptables (ej: `serde = "1.0"`).
*   **`Cargo.lock`:** Registra las versiones exactas que fueron descargadas en la última compilación exitosa (ej: `serde version = 1.0.197` y su hash criptográfico).
    *   **Garantía:** Si subes tu proyecto al repositorio y otro desarrollador lo clona, Cargo utilizará el archivo `Cargo.lock` para descargar exactamente las mismas dependencias con la misma versión bit-a-bit, impidiendo que actualizaciones accidentales en librerías externas introduzcan fallos inesperados en tu software.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet de Comandos Avanzados de Cargo

| Comando CLI | Propósito | Comportamiento |
| :--- | :--- | :--- |
| `cargo test --doc` | Correr solo Doctests | Compila y ejecuta únicamente los ejemplos de código incluidos en la documentación. |
| `cargo doc --open` | Generar y visualizar docs | Genera la web estática de tu API y la abre inmediatamente en tu navegador web predeterminado. |
| `cargo package` | Empaquetar localmente | Compila tu crate y genera un archivo `.crate` sin subirlo, ideal para verificar fallos antes de publicar. |
| `cargo yank --vers 1.0.0` | Revocar versión | Despublica una versión específica en crates.io impidiendo nuevas descargas de la misma. |
| `cargo install --path .` | Instalación local de binario | Compila e instala el binario en la carpeta bin local de cargo (usualmente `~/.cargo/bin`). |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Doctest Roto debido a cambios en firmas de funciones o imports faltantes
❌ **Código en `src/lib.rs`:**
```rust
/// Suma dos enteros.
/// ```
/// // Error: 'sumar' no está en el scope del doctest a menos que lo importemos
/// let r = sumar(2, 2); 
/// assert_eq!(r, 4);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 { a + b }
```
*   **Mensaje al hacer `cargo test`:** `error[E0425]: cannot find function `sumar` in this scope`
*   ✔️ **Solución:** Los doctests se compilan como si fueran un archivo ejecutable externo. Debes importar tu propio crate de forma explícita en el ejemplo de documentación:
    ```rust
    /// ```
    /// use mi_crate::sumar;
    /// let r = sumar(2, 2);
    /// assert_eq!(r, 4);
    /// ```
    ```

#### 2. Error al publicar en Crates.io por falta de metadatos requeridos
Al ejecutar `cargo publish`:
*   **Mensaje de Error:** `error: missing field: 'description' or 'license' in package section of Cargo.toml`
*   ✔️ **Solución:** Crates.io requiere que todos los paquetes indexados tengan descripciones y licencias legibles por la comunidad. Debes agregar campos válidos en la sección `[package]` de tu `Cargo.toml` (ej. `description = "..."`, `license = "MIT"`).

#### 3. Desalineación de Dependencias Compartidas en un Cargo Workspace
Si tienes un submódulo A que depende de `serde = "1.0.10"` y otro submódulo B que depende de `serde = "1.0.99"`:
*   **Síntoma:** El compilador procesará y compilará ambas versiones de `serde` por separado, duplicando el consumo de disco en `target/`, aumentando el tiempo de compilación global de tu espacio de trabajo y provocando errores de incompatibilidad de tipos si intentas pasar estructuras mapeadas por Serde entre ambos submódulos.
*   ✔️ **Solución (Cargo Workspace con Dependencias Heredadas):** Declara la dependencia una sola vez en el `Cargo.toml` raíz utilizando `[workspace.dependencies]` y luego impórtala en los crates hijos usando `workspace = true`:
    ```toml
    # Cargo.toml Raíz
    [workspace.dependencies]
    serde = { version = "1.0.197", features = ["derive"] }

    # chapters/ch14_cargo_more/Cargo.toml Hijo
    [dependencies]
    serde = { workspace = true }
    ```
    Esto garantiza coherencia absoluta de versiones en todo tu monorepo.
