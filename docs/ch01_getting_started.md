# Capítulo 01: Empezando con Rust (Getting Started)

Este documento contiene un análisis riguroso y profundo del primer capítulo del libro oficial de Rust. A través de este material, comprenderás la arquitectura del ecosistema de Rust, la sintaxis esencial de su punto de entrada y cómo funciona la gestión de memoria y el proceso de compilación a bajo nivel.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es Rust y por qué es relevante?
Rust es un lenguaje de programación de sistemas enfocado en tres pilares principales: **seguridad (safety)**, **velocidad (speed)** y **concurrencia sin miedo (fearless concurrency)**. A diferencia de otros lenguajes de sistemas como C o C++, Rust garantiza seguridad de memoria en tiempo de compilación sin la necesidad de un recolector de basura (Garbage Collector).

### El Ecosistema de Herramientas
El éxito de Rust reside en parte en su conjunto de herramientas integradas:
*   **Rustup:** El instalador y gestor de versiones de Rust. Permite cambiar fácilmente entre canales de lanzamiento (stable, beta, nightly) y descargar toolchains para compilación cruzada.
*   **Rustc:** El compilador de Rust. Toma el código fuente (.rs) y lo traduce a código de máquina (binarios ejecutables o bibliotecas).
*   **Cargo:** El sistema de construcción (build system) y gestor de paquetes (package manager) oficial de Rust. Cargo automatiza tareas comunes como descargar dependencias, compilar bibliotecas, ejecutar pruebas y empaquetar el software.

### Compilación Ahead-Of-Time (AOT)
Rust es un lenguaje compilado directamente a código de máquina nativo (Ahead-Of-Time). Esto significa que puedes compilar tu programa de Rust y entregar el archivo ejecutable final a otra persona para que lo corra sin necesidad de tener Rust instalado en su máquina. Esto contrasta con lenguajes interpretados o basados en máquinas virtuales (como Python o Java), los cuales requieren un intérprete o un runtime instalado en el entorno de ejecución del usuario.

---

## 2. Anatomía y Semántica de la Sintaxis

Analicemos la estructura sintáctica elemental de un programa mínimo en Rust:

```rust
fn main() {
    println!("Hello, world!");
}
```

### La Función Principal (`fn main`)
*   `fn`: Es la palabra clave reservada para declarar funciones en Rust.
*   `main`: Es el nombre asignado al punto de entrada (entry point) ejecutable del programa. Todo programa ejecutable de Rust *debe* tener una función llamada `main` para poder ejecutarse.
*   `()`: Especifica la lista de parámetros. En este caso, no recibe ningún argumento.
*   `{}`: Las llaves delimitan el cuerpo de la función. Rust requiere el uso de llaves para delimitar bloques, a diferencia de lenguajes donde es opcional para sentencias de una sola línea.

### La Expresión de Impresión (`println!`)
*   `println!`: No es una función convencional; **es una macro de Rust**. La presencia del signo de exclamación (`!`) al final de un identificador indica una invocación a una macro.
*   **¿Por qué una macro en lugar de una función?** Las funciones en Rust tienen un número fijo de argumentos y tipos estrictamente definidos en su firma. La macro `println!` requiere variadicidad (número variable de argumentos) y necesita parsear y validar la cadena de formato (ej. `"{}"`) en tiempo de compilación para garantizar que los tipos de datos pasados coincidan con los placeholders. Si intentaras imprimir una variable con tipos incompatibles o faltasen argumentos, el compilador generaría un error de compilación inmediato, evitando fallos en tiempo de ejecución.
*   `"Hello, world!"`: Es un literal de cadena (string literal) que se pasa como argumento a la macro.
*   `;`: El punto y coma. En Rust, las sentencias (statements) se terminan con punto y coma. Rust hace una distinción fundamental entre **expresiones** (que evalúan a un valor y no llevan punto y coma final) y **sentencias** (que realizan una acción pero devuelven la tupla vacía `()`).

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Enlazado Estático (Static Linking)
Cuando compilas un ejecutable básico con `cargo build --release`, el compilador `rustc` invoca a un enlazador (linker) del sistema. Por defecto, en la mayoría de las plataformas, Rust prefiere el **enlazado estático** para su propia biblioteca estándar (`std`). 
Esto significa que toda la funcionalidad necesaria de la biblioteca estándar de Rust para formatear e imprimir texto en la consola se incluye directamente dentro del binario ejecutable final. El beneficio es la portabilidad y la garantía de que el ejecutable funcionará sin dependencias externas del lenguaje, a costa de un tamaño de binario ligeramente mayor que el equivalente compilado en C con enlazado dinámico.

### Ausencia de Recolector de Basura (Zero Garbage Collection)
Rust **no tiene** recolector de basura. La memoria se gestiona de forma determinista mediante reglas de propiedad (*Ownership*), que el compilador valida en tiempo de compilación. El compilador inserta llamadas de liberación (equivalentes a `free` o destructores) exactamente donde las variables salen de su ámbito (*scope*). Esto elimina la latencia impredecible causada por los ciclos de recolección de basura, haciendo a Rust ideal para sistemas de tiempo real y de alta performance.

### Runtime Mínimo y Costo Cero
A diferencia de lenguajes como Go o Java, que empaquetan un runtime de software complejo (que maneja hilos virtuales, recolectores de basura, metadatos de reflexión, etc.), el runtime de Rust es extremadamente minimalista. Consiste principalmente en:
1.  Un pequeño bloque de código de inicialización que prepara el entorno para el entrypoint `main`.
2.  Manejadores de pánico (para dar información útil en caso de caídas controladas).
3.  Configuración inicial de la pila de llamadas (stack layout).

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen de Comandos del Ecosistema de Cargo

| Comando | Propósito | Comportamiento en Memoria / Compilación |
| :--- | :--- | :--- |
| `cargo new app` | Crea un nuevo proyecto | Genera la estructura básica con `Cargo.toml` y `src/main.rs`. |
| `cargo check` | Chequea la sintaxis rápidamente | No genera binario físico; omite la generación de código máquina. |
| `cargo build` | Compila en modo Debug | Genera binario no optimizado con símbolos de debug en `target/debug/`. |
| `cargo run` | Compila y ejecuta | Ejecuta el binario inmediatamente tras una compilación exitosa. |
| `cargo build --release` | Compila en modo Release | Genera binario optimizado en `target/release/` eliminando símbolos de debug. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Invocar macros sin la exclamación (`!`)
Si intentas llamar a una macro del sistema como si fuera una función ordinaria del lenguaje:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    println("Hello, world!"); // Error: falta el signo '!'
}
```
*   **Mensaje de Error:** `error[E0423]: expected function, found macro `println``
*   ✔️ **Solución:** Añadir el modificador `!` al identificador para indicarle a `rustc` que realice la expansión de la macro:
    ```rust
    println!("Hello, world!");
    ```

#### 2. Omitir la función de punto de entrada (`fn main`)
Si compilas un archivo ejecutable sin definir la función `main`:
❌ **Código Erróneo:**
```rust,compile_fail
// Archivo vacío o con funciones auxiliares pero sin main()
fn saludar() {
    println!("Hola");
}
```
*   **Mensaje de Error:** `error[E0601]: `main` function not found in crate`
*   ✔️ **Solución:** Definir siempre la función `main` en tu archivo binario principal para que el enlazador sepa dónde iniciar la ejecución física:
    ```rust
    fn main() {
        saludar();
    }
    ```
