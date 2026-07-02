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
*   `String::new()`: Crea una nueva instancia vacía de un `String`. `new` es una **función asociada** (associated function) al tipo `String` (similar a un método estático en otros lenguajes) que actúa como constructor.

### Referencias y Seguridad de Memoria
*   `.read_line(&mut guess)`: Pasamos `&mut guess` como argumento.
*   El símbolo `&` indica que estamos pasando una **referencia** a la variable, lo que permite que diferentes partes del código accedan a la misma porción de memoria sin necesidad de copiarla.
*   Al igual que las variables, las referencias en Rust son **inmutables por defecto**. Por lo tanto, debemos escribir `&mut guess` en lugar de `&guess` para que el método pueda modificar el contenido de la variable.

### Manejo de Errores con `Result`
*   `read_line` devuelve un valor del tipo `std::io::Result`, el cual es un enum que representa el éxito o fracaso de una operación. Cuenta con dos variantes: `Ok` (éxito, contiene el valor de retorno) y `Err` (fracaso, contiene información del error).
*   `Result` tiene un método llamado `.expect()`. Si la instancia de `Result` es `Err`, `.expect()` detendrá el programa de forma inmediata (*panic*) mostrando el mensaje provisto. Si es `Ok`, extraerá y devolverá el valor de retorno (en este caso, la cantidad de bytes leídos).
*   **Advertencia de Compilación:** Si no manejas el tipo `Result` (por ejemplo, omitiendo `.expect`), el compilador de Rust generará un warning indicando que no estás usando un valor que podría contener un error.

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

### El Rol de `Cargo.lock`
Cuando compilas por primera vez después de añadir una dependencia, Cargo descarga la versión compatible más reciente y la registra en `Cargo.lock`. 
*   **Garantía de Reproducibilidad:** A partir de ese momento, Cargo solo utilizará las versiones especificadas en `Cargo.lock`, asegurando que el proyecto compile de la misma forma en cualquier computadora.
*   **Actualizaciones Controladas:** Si deseas actualizar una dependencia a su siguiente versión compatible, debes ejecutar explícitamente `cargo update`.

### Generando el Número Secreto
```rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```
*   `use rand::Rng;`: El trait `Rng` define métodos que los generadores de números aleatorios implementan. Este trait debe estar en el scope para poder usar métodos como `gen_range`.
*   `rand::thread_rng()`: Obtiene un generador de números aleatorios local al hilo de ejecución actual y sembrado por el sistema operativo.
*   `gen_range(1..=100)`: Genera un número aleatorio dentro de un rango. La notación `1..=100` define un **rango inclusivo** (de 1 a 100). Si fuera `1..100`, sería un rango exclusivo (del 1 al 99).

---

## 3. Comparación y el Sistema de Tipos

Una vez capturada la entrada del usuario y generado el número aleatorio, debemos compararlos.

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("¡Muy pequeño!"),
    Ordering::Greater => println!("¡Muy grande!"),
    Ordering::Equal => println!("¡Ganaste!"),
}
```

### El Enum `Ordering` y `match`
*   `std::cmp::Ordering`: Es un enum que contiene tres variantes: `Less`, `Greater` y `Equal`. Se utiliza al comparar dos valores.
*   `match` (Pattern Matching): Es una estructura de control de flujo extremadamente potente en Rust. Evalúa una expresión y ejecuta el bloque del "brazo" (*arm*) cuyo patrón coincida con el valor resultante.
*   **Exhaustividad:** El compilador de Rust obliga a que los bloques `match` sean exhaustivos. Debes cubrir todas las posibilidades del valor evaluado; de lo contrario, el código no compilará.

### Conversión de Tipos y Shadowing (Enmascaramiento)
Por defecto, Rust infiere que la variable `guess` es un `String`, pero `secret_number` se infiere como un tipo numérico (típicamente `i32` o `u32`). No podemos comparar un `String` con un número.

Para solucionar esto, convertimos la entrada del usuario:

```rust
let guess: u32 = guess.trim().parse().expect("¡Por favor introduce un número!");
```

*   **Shadowing (Enmascaramiento):** Rust nos permite declarar una nueva variable con el mismo nombre que otra anterior (`let guess`). Esto se utiliza frecuentemente cuando se desea transformar el tipo de un valor sin tener que crear nombres únicos como `guess_str` y `guess_int`.
*   `trim()`: Elimina cualquier espacio en blanco al inicio y final del `String`, incluyendo el carácter de salto de línea (`\n` o `\r\n`) introducido al presionar Enter en la consola.
*   `parse()`: Convierte el `String` a otro tipo de datos. Al escribir `let guess: u32`, le estamos indicando explícitamente a `parse` que queremos convertir el texto en un entero sin signo de 32 bits.
*   `parse()` devuelve un tipo `Result`, por lo que nuevamente debemos usar `.expect()` para manejar un posible error si el usuario introduce texto no numérico.

---

## 4. Control de Bucles y Manejo de Errores Avanzado

Para permitir que el usuario intente adivinar varias veces, introducimos un bucle infinito y un manejo de errores más refinado.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("¡Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor, introduce tu suposición.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");

        // Manejo de errores sin abortar el programa
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, introduce solo números.");
                continue;
            }
        };

        println!("Suposición: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Muy pequeño!"),
            Ordering::Greater => println!("¡Muy grande!"),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break; // Sale del bucle loop
            }
        }
    }
}
```

### Bucle Infinito (`loop`) y Salida (`break`)
*   `loop`: Crea un bucle infinito.
*   `break`: Detiene la ejecución del bucle actual de forma inmediata. En este caso, cuando el usuario adivina el número (`Ordering::Equal`), el juego termina.
*   `continue`: Detiene la iteración actual del bucle e inicia la siguiente desde el principio.

### Manejo de Errores con `match`
En lugar de crasear el programa con `.expect()` si el usuario ingresa algo inválido, podemos usar un bloque `match` para procesar el tipo `Result` devuelto por `parse()`:
*   `Ok(num)`: Si la conversión fue exitosa, extraemos el número y lo asignamos a la variable `guess`.
*   `Err(_)`: El guion bajo `_` es un comodín que coincide con cualquier valor de error sin necesidad de guardarlo. En este caso, imprimimos un mensaje de aviso y usamos `continue` para saltar el resto del bucle e iniciar una nueva iteración.
