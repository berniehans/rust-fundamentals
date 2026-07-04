# Capítulo 09: Manejo de Errores (panic! y Result)

Este documento proporciona un análisis exhaustivo y de bajo nivel de los mecanismos de gestión de errores en Rust. Se detallan su semántica, el comportamiento del runtime ante fallos catastróficos, el layout de memoria física de las abstracciones de control de flujo, y las optimizaciones a nivel de compilador para garantizar la seguridad y el alto rendimiento.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Filosofía del Manejo de Errores en Rust
A diferencia de lenguajes como C++, Java, C# o Python, Rust **no implementa excepciones tradicionales** basadas en bloques `try-catch`. La ausencia de excepciones es una decisión de diseño fundamental guiada por los siguientes principios:
*   **Claridad e Invisibilidad:** Las excepciones tradicionales introducen caminos de ejecución invisibles. Una función puede lanzar una excepción que se propaga varias capas arriba sin que sea evidente al leer el código de llamada. En Rust, cualquier función que pueda fallar lo declara explícitamente en su firma de tipo.
*   **Seguridad de Recursos:** Las excepciones tradicionales complican la liberación garantizada de recursos (memoria, descriptores de archivos, mutexes) si ocurre un fallo a mitad de una función. El modelo de *Ownership* de Rust y su trait `Drop` aseguran que la limpieza se maneje correctamente a través del sistema de tipos.
*   **Rendimiento Predecible:** La gestión de excepciones suele implicar un sobrecosto en tiempo de ejecución para registrar bloques de control e inspeccionar la pila (*stack walking*) al buscar controladores adecuados.

### Errores Recuperables vs. Irrecuperables
Rust divide los errores en dos categorías principales:

1.  **Errores Recuperables (Recoverable Errors):** Representan fallos previsibles en situaciones operativas normales, como "archivo no encontrado" o "tiempo de espera de red agotado". Estos errores son de naturaleza informativa. El programa no debe detenerse; en su lugar, debe reportar la anomalía al usuario, intentar una estrategia alternativa o propagar el fallo. Rust modela estos casos usando el tipo algebraico de datos `Result<T, E>`.
2.  **Errores Irrecuperables (Unrecoverable Errors):** Son síntomas de bugs lógicos graves en el programa, como acceder a un índice fuera de los límites de un vector, dividir por cero o desreferenciar punteros inválidos en código inseguro. El programa ha entrado en un estado inconsistente y continuar su ejecución podría comprometer la integridad de la memoria o la seguridad del sistema. Rust detiene inmediatamente el hilo de ejecución mediante la macro `panic!`.

---

## 2. Anatomía y Semántica de la Sintaxis

### Errores Irrecuperables con `panic!`
Cuando se ejecuta la macro `panic!`, el programa realiza dos acciones: imprime un mensaje de error y la ubicación física en el código fuente, limpia la pila (liberando memoria y destructores de variables activas) y termina el proceso con un código de salida distinto de cero.

```rust
fn main() {
    // 1. Pánico explícito inducido por el programador
    panic!("¡Fallo catastrófico en el motor de renderizado!");

    // 2. Pánico implícito (provocado por el runtime)
    let v = vec![1, 2, 3];
    let _elemento = v[99]; // Provoca pánico: 'index out of bounds: the len is 3 but the index is 99'
}
```

### Errores Recuperables con `Result<T, E>`
El tipo `Result` es un `enum` genérico definido en la biblioteca estándar con dos variantes:
```rust
enum Result<T, E> {
    Ok(T),  // Representa el éxito y envuelve el valor retornado
    Err(E), // Representa el fallo y envuelve el error detallado
}
```

#### Manejo Básico de `Result` mediante Coincidencia de Patrones (`match`)
La forma fundamental de manejar un `Result` es inspeccionar sus variantes mediante un bloque `match`:

```rust
use std::fs::File;

fn main() {
    let archivo_resultado = File::open("datos.txt");

    let _archivo = match archivo_resultado {
        Ok(archivo) => archivo, // Extrae el descriptor de archivo
        Err(error) => {
            println!("No se pudo abrir el archivo: {error:?}");
            return;
        }
    };
}
```

#### Coincidencia Avanzada por Categoría de Error (`ErrorKind`)
A menudo es necesario reaccionar de forma diferente según el motivo del fallo. En el caso de operaciones de Entrada/Salida, Rust provee el enum `io::ErrorKind` dentro de la estructura `io::Error`.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let archivo_resultado = File::open("config.json");

    let _archivo = match archivo_resultado {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("config.json") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema al crear el archivo: {e:?}"),
            },
            otro_error => {
                panic!("Problema al abrir el archivo: {otro_error:?}");
            }
        },
    };
}
```

#### Métodos de Atajo: `unwrap` y `expect`
Cuando el desarrollador está seguro de que la operación no fallará, o si prefiere que el programa entre en pánico inmediatamente si ocurre un error, existen métodos abreviados:

*   **`unwrap()`**: Retorna el valor interno si es `Ok`. Si es `Err`, entra en pánico con el mensaje de error por defecto.
*   **`expect("mensaje")`**: Igual que `unwrap()`, pero permite especificar un mensaje de error personalizado para facilitar la depuración.

```rust
use std::fs::File;

fn main() {
    // Si no existe, causa pánico genérico
    let _f1 = File::open("archivo_critico.bin").unwrap();

    // Si no existe, causa pánico con el contexto proporcionado
    let _f2 = File::open("archivo_critico.bin")
        .expect("El archivo binario crítico es indispensable para la base de datos");
}
```

### Propagación de Errores con el Operador `?`
En lugar de manejar el error en el sitio, una función puede devolver el error a la función que la invocó. El operador **`?`** simplifica enormemente este proceso:

```rust
use std::fs::File;
use std::io::{self, Read};

fn leer_usuario_desde_archivo() -> Result<String, io::Error> {
    // Si File::open falla, retorna inmediatamente el Err(io::Error) de esta función
    let mut archivo = File::open("usuario.txt")?;
    let mut nombre = String::new();
    
    // Si read_to_string falla, retorna el error
    archivo.read_to_string(&mut nombre)?;
    
    Ok(nombre)
}
```

#### Reglas y Compatibilidad del Operador `?`
1.  **Compatibilidad de Retorno:** El operador `?` solo se puede utilizar dentro de funciones que devuelvan un tipo compatible con el valor sobre el cual se aplica (como `Result`, `Option` o tipos que implementen el trait interno `FromResidual`).
2.  **Conversión Automática de Tipos (El Trait `From`):** Cuando se usa `?`, el valor del error pasa automáticamente por la función `from` definida en el trait `std::convert::From`. Esto permite convertir un error específico en uno más genérico de forma transparente.

```rust
use std::error::Error;
use std::fs::File;

// Retornamos Box<dyn Error> (cualquier error que implemente el trait Error)
fn procesar() -> Result<(), Box<dyn Error>> {
    // File::open devuelve io::Error, pero '?' lo convierte automáticamente a Box<dyn Error>
    let _archivo = File::open("config.toml")?;
    Ok(())
}
```

#### El Operador `?` sobre `Option<T>`
El operador `?` también se puede aplicar sobre valores del tipo `Option<T>`. Si el valor es `None`, la función terminará prematuramente retornando `None`.

```rust
fn ultimo_caracter_de_primera_linea(texto: &str) -> Option<char> {
    texto.lines().next()?.chars().last()
}
```

### Retorno de `Result` en `main` y el Trait `Termination`
Por defecto, la función `main` retorna la tupla vacía `()`. Sin embargo, Rust permite que `main` retorne un `Result<(), E>`, lo que habilita el uso del operador `?` en el punto de entrada del programa:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _archivo = File::open("clave_privada.pem")?;
    Ok(())
}
```

Este comportamiento es posible porque el tipo de retorno de `main` debe implementar el trait `std::process::Termination`. Si `main` retorna `Ok(())`, el proceso finaliza con un código de salida `0`. Si retorna un `Err`, imprime la representación de depuración (`Debug`) del error y sale con un código distinto de cero (usualmente `1`).

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Stack Unwinding (Desenredado) vs. Abort (Abortar)
Cuando ocurre un pánico, Rust puede limpiar la pila de dos maneras configurables en el archivo `Cargo.toml`:

#### 1. Unwinding (Comportamiento por defecto)
El compilador inserta metadatos y código de soporte para recorrer la pila hacia atrás en caso de pánico:
*   Se ejecutan los destructores (`drop()`) de todas las variables activas en cada marco de pila (*stack frame*).
*   Esto previene fugas de recursos del sistema ajenos a la memoria (como descriptores de sockets de red abiertos, bloqueos de archivos o bases de datos inconsistentes).
*   Tiene un costo en el tamaño del ejecutable debido a las tablas de búsqueda necesarias para rastrear el flujo de desapilado.

#### 2. Aborting
El programa detiene inmediatamente la ejecución y delega la limpieza al sistema operativo:
*   No se ejecutan los destructores (`drop()`) de los objetos en memoria.
*   Reduce significativamente el tamaño del binario compilado y el tiempo de compilación.
*   Es ideal para sistemas embebidos, contenedores minimalistas o aplicaciones de máximo rendimiento.

Para configurar el aborto en caso de pánico, se añade al `Cargo.toml`:
```toml
[profile.release]
panic = "abort"
```

---

### Anatomía en Memoria de `Result<T, E>` y `Option<T>`
En el plano de memoria física, los tipos `Result<T, E>` y `Option<T>` son enums estructurados. En circunstancias estándar, el compilador debe reservar espacio en el Stack para almacenar:
1.  El **payload** de la variante con el tamaño máximo de entre los dos tipos ($max(sizeof(T), sizeof(E))$).
2.  Un **tag o discriminante** (generalmente 1 byte, alineado a la palabra del procesador) para indicar si el estado actual es éxito o error.

```
Layout General en Memoria Stack (Result<u32, u8> sin optimizar)
+-----------------------+-----------------------+
| Discriminante (1 byte)| Payload (4 bytes)     |
| (0 = Ok, 1 = Err)     | (Valor u32 / u8)      |
+-----------------------+-----------------------+
Total físico: 8 bytes (debido a reglas de alineación de memoria para u32)
```

#### Optimización de Nicho (Niche Optimization)
Para evitar el sobrecosto de memoria que añade el discriminante, el compilador de Rust aplica una técnica llamada **Optimización de Nicho** (o *Null Pointer Optimization*). 

Si un tipo posee valores prohibidos o no válidos dentro de su representación binaria (estos huecos de valores se llaman "nichos"), el compilador utiliza esos valores inválidos para representar la variante alternativa sin añadir un discriminante físico.

El ejemplo clásico es la referencia `&T` (o el puntero inteligente `Box<T>`). Por definición, las referencias en Rust **nunca pueden ser nulas** (la dirección `0x0` es un valor prohibido).
*   `size_of::<&i32>()` = 8 bytes.
*   `size_of::<Option<&i32>>()` = **8 bytes** (en lugar de 16 bytes).

El compilador mapea la variante `None` al valor binario `0x0`. Si el puntero contiene cualquier valor distinto de cero, el runtime sabe que representa `Some(&T)`. Esto permite que el uso de enums seguros no tenga ningún costo de almacenamiento en comparación con punteros en C/C++.

---

### Abstracciones de Cero Costo en el Camino Feliz (Happy Path)
En el camino feliz (cuando no ocurren errores), el uso de `Result` es una **abstracción de coste cero**:
*   La comprobación de la variante Ok se traduce en instrucciones de comparación de CPU de un solo ciclo (ej: `test` o `cmp` en ensamblador x86-64) y saltos condicionales rápidos (`je`/`jne`).
*   A diferencia de las excepciones de otros lenguajes, no hay llamadas al sistema para registrar manejadores de excepciones al entrar a una función, ni asignación dinámica de memoria para instanciar el objeto del error a menos que se solicite de forma explícita.
*   Si se activa `RUST_BACKTRACE=1`, el costo de capturar e imprimir el mapa de llamadas solo se paga si ocurre un `panic!`, nunca durante el flujo normal con `Result`.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen de Patrones Sintácticos

| Expresión / Método | Comportamiento en Ok / Some | Comportamiento en Err / None | Caso de Uso Recomendado |
| :--- | :--- | :--- | :--- |
| `let val = resultado?;` | Retorna `val` localmente. | Retorna el error al invocador de la función. | Flujo de control estándar y limpio en funciones con retorno compatible. |
| `let val = resultado.unwrap();` | Retorna `val`. | Causa un **pánico inmediato** del hilo. | Prototipado rápido, pruebas unitarias o cuando el fallo es imposible físicamente. |
| `let val = resultado.expect("msg");` | Retorna `val`. | Causa un **pánico con el mensaje personalizado**. | Igual que unwrap, pero documentando el motivo por el cual es imposible fallar. |
| `let val = resultado.unwrap_or(def);` | Retorna `val`. | Retorna el valor por defecto `def` provisto. | Proporcionar valores de respaldo sencillos que no requieren costosa evaluación. |
| `let val = resultado.unwrap_or_else(\|e\| ...);` | Retorna `val`. | Ejecuta un cierre (closure) para calcular el valor. | Recuperación perezosa o costosa en recursos cuando ocurre un error. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Uso del operador `?` en funciones sin retorno compatible
❌ **Código Erróneo:**
```rust
fn main() {
    // main por defecto devuelve (), pero usamos '?' que requiere Result/Option/Termination
    let _archivo = std::fs::File::open("datos.json")?;
}
```
*   **Mensaje de Error:** `error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)`
*   ✔️ **Solución:** Cambiar el tipo de retorno de `main` a `Result<(), Box<dyn std::error::Error>>`, o utilizar coincidencia de patrones con `match`.

#### 2. Incompatibilidad de tipos de error al propagar con `?`
❌ **Código Erróneo:**
```rust
use std::fs::File;
use std::num::ParseIntError;

fn leer_y_parsear() -> Result<i32, ParseIntError> {
    // Error: File::open devuelve io::Error, pero la firma dice ParseIntError
    let _archivo = File::open("numero.txt")?; 
    Ok(10)
}
```
*   **Mensaje de Error:** `error[E0277]: `?` couldn't convert the error to `ParseIntError``
*   ✔️ **Solución:** Utilizar un tipo de error unificado (como un enum personalizado implementado con `thiserror`), mapear el error explícitamente usando `.map_err()` o retornar `Result<i32, Box<dyn std::error::Error>>`.
    ```rust
    // Solución con map_err
    let _archivo = File::open("numero.txt")
        .map_err(|_| ParseIntError::custom_or_similar())?;
    ```

#### 3. Pérdida de Ownership al hacer coincidencia de patrones en `match`
❌ **Código Erróneo:**
```rust
struct Contenedor {
    datos: Result<String, String>,
}

fn inspeccionar(c: Contenedor) {
    match c.datos {
        Ok(texto) => println!("Éxito: {texto}"),
        Err(e) => println!("Error: {e}"),
    }
    // Intentar volver a usar c.datos o c provocará error porque las variantes de c.datos
    // contenían tipos no-Copy (String) y fueron movidas por defecto.
}
```
*   **Mensaje de Error:** `error[E0382]: use of partially moved value: 'c'`
*   ✔️ **Solución:** Hacer coincidencia por referencia usando la palabra clave `ref` o empleando el método `.as_ref()` sobre el `Result`:
    ```rust
    match c.datos.as_ref() {
        Ok(texto) => println!("Éxito: {texto}"), // texto es de tipo &String
        Err(e) => println!("Error: {e}"),
    }
    ```
