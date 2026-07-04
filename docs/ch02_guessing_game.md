# Capítulo 02: Programando el Juego de Adivinanza (Guessing Game)

Este documento ofrece un análisis exhaustivo y de bajo nivel de los conceptos introducidos en el segundo capítulo del libro oficial de Rust. A través de la construcción de un juego de adivinanza interactivo, exploraremos la entrada/salida de datos, el sistema de tipos de Rust, el shadowing de variables, el manejo de errores básico y la inclusión de dependencias externas.

---

## 1. Entrada de Datos y Mutabilidad

El primer paso del juego consiste en capturar la entrada del usuario desde la consola.

```rust
use std::io;

fn main() {
    println!("¡Adivina el número!");
    println!("Por favor, introduce tu suposición.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al leer la línea");

    println!("Suposición: {guess}");
}
```

### Importación de Bibliotecas (`use`)
*   `use std::io;`: Por defecto, Rust importa un conjunto de elementos esenciales en todo programa llamado **prelude** (preludio). Como la biblioteca de entrada y salida (`io`) no está en el preludio, debemos importarla explícitamente desde la biblioteca estándar (`std`).

### Variables y Mutabilidad
*   En Rust, las variables son **inmutables por defecto**. Esto promueve la seguridad y la concurrencia.
*   `let mut guess`: Para permitir que una variable cambie de valor, debemos agregar la palabra clave `mut` (mutable).
*   `String::new()`: Crea una nueva instancia vacía de un `String`. `new` es una **función asociada** (associated function) al tipo `String` (similar a un método estático en otros lenguajes) que actúa como constructor en el montón (Heap).

### Referencias y Seguridad de Memoria
*   `.read_line(&mut guess)`: Pasamos `&mut guess` como argumento.
*   El símbolo `&` indica que estamos pasando una **referencia** a la variable, lo que permite que diferentes partes del código accedan a la misma porción de memoria sin necesidad de copiarla.
*   Al igual que las variables, las referencias en Rust son **inmutables por defecto**. Por lo tanto, debemos escribir `&mut guess` en lugar de `&guess` para que el método pueda modificar el contenido de la variable.

---

## 2. Dependencias Externas (El Crate `rand`)

Para generar un número secreto de forma aleatoria, necesitamos acudir al ecosistema de paquetes de Rust (*crates*).

### Modificando el Manifiesto `Cargo.toml`
Para instalar bibliotecas de terceros de [crates.io](https://crates.io/), debemos declararlas en la sección `[dependencies]` de nuestro manifiesto:

```toml
[dependencies]
rand = "0.8.5"
```

Rust utiliza **Versionamiento Semántico (SemVer)**. La declaración `"0.8.5"` es en realidad una abreviatura de `^0.8.5`, lo que le indica a Cargo que puede utilizar cualquier versión que sea compatible a nivel de API con la versión `0.8.5` (por ejemplo, `0.8.6`, pero nunca `0.9.0`).

### Generando el Número Secreto
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```
*   `use rand::Rng;`: El trait `Rng` define métodos que los generadores de números aleatorios implementan. Este trait debe estar en el scope para poder usar métodos como `gen_range`.
*   `rand::thread_rng()`: Obtiene un generador de números aleatorios local al hilo de ejecución actual y sembrado por el sistema operativo.
*   `gen_range(1..=100)`: Genera un número aleatorio dentro de un rango. La notación `1..=100` define un **rango inclusivo** (de 1 a 100). Si fuera `1..100`, sería un rango exclusivo (del 1 al 99).

---

## 3. Comparación y el Sistema de Tipos

Una vez capturada la entrada del usuario y generado el número aleatorio, debemos compararlos.

```rust
use std::cmp::Ordering;

fn main() {
    let secret_number = 50; // Inferencia de tipo i32
    let mut guess = String::from("50");

    // Para comparar, primero debemos convertir guess a entero
    let guess: u32 = guess.trim().parse().expect("Introduce un número");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("¡Muy pequeño!"),
        Ordering::Greater => println!("¡Muy grande!"),
        Ordering::Equal => println!("¡Ganaste!"),
    }
}
```

### El Enum `Ordering` y `match`
*   `std::cmp::Ordering`: Es un enum que contiene tres variantes: `Less`, `Greater` y `Equal`. Se utiliza al comparar dos valores.
*   `match` (Pattern Matching): Evalúa una expresión y ejecuta el brazo (*arm*) cuyo patrón coincida con el valor resultante. El compilador de Rust obliga a que los bloques `match` sean exhaustivos.

### Shadowing (Enmascaramiento)
*   Rust nos permite declarar una nueva variable con el mismo nombre que otra anterior (`let guess`). Esto se utiliza frecuentemente cuando se desea transformar el tipo de un valor sin tener que crear nombres únicos como `guess_str` y `guess_int`.
*   `trim()`: Elimina espacios en blanco y el carácter de salto de línea (`\n` o `\r\n`) introducido al presionar Enter en la terminal.
*   `parse()`: Convierte el `String` a otro tipo de datos (en este caso, un `u32` explícito). Devuelve un tipo `Result`.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen de APIs de Entrada/Salida y Utilidades

| Expresión / Método | Tipo de Retorno | Propósito |
| :--- | :--- | :--- |
| `String::new()` | `String` | Crea una cadena vacía en el montón (Heap). |
| `io::stdin().read_line(&mut s)` | `Result<usize, Error>` | Lee una línea de consola y la añade a la variable mutable `s`. |
| `s.trim()` | `&str` | Elimina saltos de línea y espacios en los extremos de la cadena. |
| `s.parse()` | `Result<T, ParseError>` | Intenta analizar la cadena y convertirla en otro tipo de datos `T`. |
| `std::cmp::Ordering` | Enum | Contiene `Less`, `Greater` y `Equal` para comparaciones. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Pasar una referencia inmutable a una función que modifica el valor
Si pasas un préstamo inmutable (`&`) a un método como `read_line` que requiere modificar el valor interno:
❌ **Código Erróneo:**
```rust,compile_fail
use std::io;

fn main() {
    let mut guess = String::new();
    // Error: se pasa &guess en lugar de &mut guess
    io::stdin().read_line(&guess).unwrap(); 
}
```
*   **Mensaje de Error:** `error[E0308]: mismatched types: expected mutable reference `&mut String`, found reference `&String``
*   ✔️ **Solución:** Las referencias son inmutables por defecto. Asegúrate de anteponer `&mut` en la llamada:
    ```rust
    io::stdin().read_line(&mut guess).unwrap();
    ```

#### 2. Comparación directa de tipos incompatibles (ej. String vs. Entero)
Si intentas comparar variables de tipos distintos sin antes parsear o convertir:
❌ **Código Erróneo:**
```rust,compile_fail
use std::cmp::Ordering;

fn main() {
    let guess = String::from("42");
    let numero_secreto = 42;

    // Error: no se puede comparar String con un entero directamente
    match guess.cmp(&numero_secreto) {
        Ordering::Equal => println!("Iguales"),
        _ => println!("Diferentes"),
    }
}
```
*   **Mensaje de Error:** `error[E0308]: mismatched types: expected `&String`, found `&{integer}``
*   ✔️ **Solución:** Utilizar `.parse()` para transformar el String en un entero del mismo tipo antes de llamar al comparador:
    ```rust
    let guess: i32 = guess.trim().parse().unwrap();
    match guess.cmp(&numero_secreto) { ... }
    ```

#### 3. Ignorar valores de tipo `Result` (Advertencia del compilador)
Omitir el manejo de operaciones que pueden fallar y devuelven un `Result`:
❌ **Código Erróneo:**
```rust
use std::io;

fn main() {
    let mut guess = String::new();
    // Compila pero emite una advertencia al no manejar el Result devuelto
    io::stdin().read_line(&mut guess); 
}
```
*   **Mensaje de Advertencia:** `warning: unused `Result` that must be used`
*   ✔️ **Solución:** Llamar a `.expect()` o usar un bloque `match` sobre el enum retornado para gestionar explícitamente el posible error de I/O:
    ```rust
    io::stdin().read_line(&mut guess).expect("Fallo al leer");
    ```
