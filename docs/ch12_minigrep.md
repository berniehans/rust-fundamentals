# Capítulo 12: Proyecto I/O (Construyendo un Programa de Línea de Comandos)

Este documento proporciona un análisis detallado y de bajo nivel para la construcción de una utilidad de consola (CLI) eficiente en Rust. Se analizan la captura de argumentos del sistema operativo, el manejo robusto de Entrada/Salida (I/O), la modularización arquitectónica y las implicaciones de rendimiento entre la clonación de datos y la gestión de variables de entorno.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Arquitectura de un Programa CLI en Rust
El desarrollo de utilidades de consola de alto rendimiento en Rust se rige por un principio de **separación estricta de responsabilidades (Separation of Concerns)**. 

A medida que una aplicación crece, escribir toda la lógica dentro de la función `main` degrada la mantenibilidad del código y dificulta la ejecución de pruebas automatizadas. Por lo tanto, la estructura idiomática recomendada divide el software en dos partes físicas:
1.  **`src/main.rs`:** Actúa únicamente como el punto de entrada del programa. Se encarga de:
    *   Capturar los argumentos de la línea de comandos.
    *   Configurar el estado inicial.
    *   Llamar a la lógica de ejecución del negocio.
    *   Capturar y reportar de forma legible los errores, finalizando el proceso de forma controlada.
2.  **`src/lib.rs`:** Alberga toda la lógica de negocio real. Define las estructuras de datos principales, implementa los algoritmos de búsqueda y procesamiento, y exporta una API pública limpia que puede ser probada de manera unitaria y de integración sin depender del entorno de la CLI.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Captura de Argumentos con `std::env::args`
Para leer los argumentos que el usuario pasa a la aplicación en la terminal, se utiliza la función `std::env::args` de la biblioteca estándar:

```rust
use std::env;

fn main() {
    // std::env::args devuelve un iterador que produce Strings
    // .collect() transforma el iterador en una colección física como un Vec
    let args: Vec<String> = env::args().collect();

    // Nota: El primer argumento (args[0]) es siempre la ruta del ejecutable
    let _programa = &args[0]; 
    if args.len() > 1 {
        let primer_arg = &args[1];
        println!("Argumento recibido: {primer_arg}");
    }
}
```

### 2. Lectura de Archivos con `std::fs::read_to_string`
La lectura de archivos en disco se maneja de forma segura mediante el módulo de sistema de archivos `std::fs`:

```rust
use std::fs;

fn leer_archivo(ruta: &str) -> Result<String, std::io::Error> {
    // Lee todo el contenido del archivo y lo almacena en un String en el montón
    let contenido: String = fs::read_to_string(ruta)?;
    Ok(contenido)
}
```

### 3. La Refactorización Modular (`Config` y `run`)

#### Diseño Seguro de la Configuración (`Config::build`)
En lugar de procesar los argumentos directamente en variables dispersas, agrupamos la configuración en una estructura `Config`. Implementamos un constructor seguro `build` que valida los datos y retorna un `Result` en lugar de causar pánicos automáticos.

```rust
// src/lib.rs
pub struct Config {
    pub consulta: String,
    pub ruta_archivo: String,
    pub ignorar_mayusculas: bool,
}

impl Config {
    // Usamos un iterador o un slice de Strings para construir de forma segura
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Argumentos insuficientes. Uso: minigrep <consulta> <ruta_archivo>");
        }

        let consulta = args[1].clone();
        let ruta_archivo = args[2].clone();

        // Leemos una variable de entorno para determinar si ignoramos mayúsculas
        // std::env::var devuelve un Result; si está presente (Ok), activamos la opción
        let ignorar_mayusculas = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            consulta,
            ruta_archivo,
            ignorar_mayusculas,
        })
    }
}
```

#### Encapsulación de la Lógica Principal en `run`
La lógica que lee el archivo y ejecuta la búsqueda se aísla en una función `run` dentro de `src/lib.rs`:

```rust
// src/lib.rs
use std::error::Error;
use std::fs;

// Retornamos Box<dyn Error> para permitir propagar cualquier error de I/O
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenido = fs::read_to_string(config.ruta_archivo)?;

    let resultados = if config.ignorar_mayusculas {
        buscar_insensible(&config.consulta, &contenido)
    } else {
        buscar_sensible(&config.consulta, &contenido)
    };

    for linea in resultados {
        println!("{linea}");
    }

    Ok(())
}
```

#### Implementaciones de Búsqueda
```rust
// Búsqueda Sensible a Mayúsculas
pub fn buscar_sensible<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultados = Vec::new();
    // Iteramos por líneas y devolvemos referencias que apuntan al contenido original
    for linea in contenido.lines() {
        if linea.contains(consulta) {
            resultados.push(linea);
        }
    }
    resultados
}

// Búsqueda Insensible a Mayúsculas (Convertimos ambas a minúsculas)
pub fn buscar_insensible<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str> {
    let consulta_minuscula = consulta.to_lowercase();
    let mut resultados = Vec::new();
    for linea in contenido.lines() {
        if linea.to_lowercase().contains(&consulta_minuscula) {
            resultados.push(linea);
        }
    }
    resultados
}
```

#### El Punto de Entrada Simplificado (`src/main.rs`)
Gracias a esta refactorización, `main` queda reducido a una función limpia de enrutamiento y captura de fallos:

```rust
// src/main.rs
use std::env;
use std::process;
use ch12_minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Manejo de errores al construir la configuración
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Escribimos el error en stderr para separar logs de resultados reales
        eprintln!("Error al procesar argumentos: {err}");
        process::exit(1); // Terminamos inmediatamente con código de error
    });

    // Manejo de errores al ejecutar el motor del programa
    if let Err(e) = run(config) {
        eprintln!("Error en la aplicación: {e}");
        process::exit(1);
    }
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Propiedad vs. Clonación en Argumentos CLI
En la implementación inicial del libro, se utiliza `.clone()` para transferir la propiedad de las cadenas de argumentos a la estructura `Config`. 
*   **Por qué se hace:** Evita lidiar con lifetimes complejos (`&'a str`), ya que al clonar, `Config` pasa a ser dueña absoluta de sus propios datos (`String`) y puede vivir de forma independiente.
*   **Costo de Rendimiento:** Clonar cadenas de texto requiere solicitar asignación de memoria dinámica en el Heap. Aunque en una utilidad CLI pequeña como `minigrep` el impacto es imperceptible, en programas de gran escala se prefiere evitar. En capítulos posteriores se demuestra cómo evitarlo consumiendo directamente el iterador devuelto por `env::args`, tomando posesión del valor original sin clonarlo.

### Robustez del Sistema Operativo: `args` vs. `args_os`
Es fundamental entender la diferencia física entre capturar argumentos Unicode y nativos:
1.  **`std::env::args`:** Retorna un iterador de objetos `String`. **Importante:** Si el usuario pasa un argumento que contiene bytes que no representan una cadena Unicode UTF-8 válida, esta función **provoca un pánico inmediato**.
2.  **`std::env::args_os`:** Retorna un iterador de objetos `std::ffi::OsString`. Esta estructura representa cadenas nativas del sistema operativo tal cual vienen de la API del kernel (sin forzar validación UTF-8).
    *   **Uso Recomendado:** Si estás construyendo herramientas CLI de grado de producción que procesan rutas de archivos arbitrarias en Linux (donde las rutas son simplemente bytes arbitrarios) o Windows (UTF-16), se debe utilizar `args_os` y manejar los datos mediante `OsString`/`OsStr`.

### Salida Controlada del Proceso (`std::process::exit`)
Cuando ocurre un error crítico en `main`, invocamos `process::exit(status)`.
*   A diferencia de un pánico ordinario, `process::exit` finaliza el proceso de inmediato.
*   **Importante:** En la versión estándar de Rust, `process::exit` **no ejecuta los destructores** (no realiza *Stack Unwinding*). Si tienes recursos en memoria que requieren una desconexión explícita o limpieza manual de persistencia, debes asegurarte de liberarlos antes de invocar la salida, o estructurar la aplicación para que la terminación ocurra fuera de las funciones críticas de negocio.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet de Funciones del Entorno e I/O

| Expresión / Llamada | Tipo de Retorno | Propósito y Comportamiento |
| :--- | :--- | :--- |
| `std::env::args()` | `Args` (Iterador) | Capturar argumentos de terminal (causa pánico si no son UTF-8). |
| `std::env::args_os()` | `ArgsOs` (Iterador) | Capturar argumentos de terminal con soporte de bytes inválidos (`OsString`). |
| `std::fs::read_to_string(ruta)` | `Result<String, io::Error>` | Lee el archivo completo en el montón en una sola llamada. |
| `std::env::var("NOMBRE")` | `Result<String, VarError>` | Intenta capturar el valor de una variable de entorno del sistema. |
| `eprintln!("msg");` | Macro (Salida en Stderr) | Imprime en la consola a través del canal de errores estándar. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Pánico por acceso a índices fijos de argumentos sin comprobación previa
❌ **Código Erróneo:**
```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Si el usuario no pasa argumentos, la aplicación explota aquí
    let consulta = &args[1]; 
}
```
*   **Síntoma en ejecución:** `thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'`
*   ✔️ **Solución:** Validar siempre la longitud de la colección (`args.len()`) o utilizar coincidencia de patrones (matching) antes de acceder a posiciones del vector.

#### 2. Intentar retornar referencias a cadenas creadas localmente dentro de `Config::build`
❌ **Código Erróneo:**
```rust
struct Config<'a> {
    consulta: &'a str,
}

impl<'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        // Intentar guardar un slice temporal del String contenido en el vector args
        let consulta = &args[1][..]; 
        Ok(Config { consulta })
    }
}
```
*   **Mensaje de Error:** El compilador puede arrojar errores de lifetime si `args` es destruido en `main` mientras intentamos propagar `Config` a otras funciones fuera de su ámbito de vida.
*   ✔️ **Solución:** En proyectos CLI iniciales, transferir la propiedad completa de los datos usando `.clone()` convirtiendo los campos de `Config` a tipos `String` adueñados.

#### 3. Conflictos de Variables de Entorno en Pruebas Concurrentes
Al testear funciones que dependen de `std::env::set_var` y `std::env::var`:
❌ **Código de Test:**
```rust
#[test]
fn test_busqueda_insensible() {
    std::env::set_var("IGNORE_CASE", "1");
    // Lógica de test...
}

#[test]
fn test_busqueda_sensible() {
    std::env::remove_var("IGNORE_CASE");
    // Lógica de test...
}
```
*   **Síntoma:** Al correr `cargo test` de forma paralela (por defecto), los hilos comparten el mismo entorno del proceso y se sobrescriben mutuamente la variable `IGNORE_CASE`, provocando fallos aleatorios e intermitentes en la suite de pruebas.
*   ✔️ **Solución A:** Ejecutar las pruebas en serie mediante `cargo test -- --test-threads=1`.
*   ✔️ **Solución B (Recomendada):** Evitar leer `std::env::var` dentro de las funciones de lógica profunda. En su lugar, lee la variable de entorno únicamente en `main.rs` (o en la construcción de `Config`), y pasa la bandera booleana (`ignorar_mayusculas: bool`) de forma explícita a la estructura y a las funciones de búsqueda.
