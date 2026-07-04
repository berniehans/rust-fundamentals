# Capítulo 13: Características Funcionales (Closures e Iteradores)

Este documento proporciona un análisis exhaustivo y de bajo nivel de las herramientas de programación funcional disponibles en Rust: los closures (clausuras o funciones anónimas) y los iteradores, analizando su implementación interna, su layout físico en memoria, y los mecanismos de optimización del compilador que garantizan que estas abstracciones tengan un costo cero en tiempo de ejecución.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Programación Funcional en un Lenguaje de Sistemas
Rust no es un lenguaje puramente funcional (es multiparadigma, con bases imperativas y orientadas a objetos), pero adopta conceptos clave del paradigma funcional para mejorar la expresividad y la seguridad sin penalizar el rendimiento. Las dos abstracciones fundamentales de este enfoque son:

1.  **Closures (Clausuras):** Funciones anónimas que se pueden guardar en variables o pasar como argumentos a otras funciones. A diferencia de las funciones regulares (`fn`), los closures tienen la capacidad única de **capturar variables del entorno** en el que son declarados.
2.  **Iteradores:** Abstracciones para procesar secuencialmente colecciones de elementos de forma perezosa (*lazy*). Permiten expresar operaciones complejas de filtrado, transformación y reducción mediante un estilo declarativo muy limpio.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Closures (Funciones Anónimas)

#### Sintaxis Básica e Inferencia de Tipos
La sintaxis de un closure utiliza barras verticales `||` para delimitar los parámetros de entrada en lugar de paréntesis `()`:

```rust
// 1. Declaración explícita de tipos
let sumar_uno = |x: i32| -> i32 { x + 1 };

// 2. Inferencia de tipos total (El compilador infiere los tipos a partir del primer uso)
let sumar_dos = |x| x + 2; 

// Nota: A diferencia de las funciones 'fn', que exigen anotar tipos de forma explícita,
// los closures no los requieren porque el compilador restringe su uso a un contexto local cerrado.
```

#### Captura del Entorno y la Palabra Clave `move`
Un closure puede capturar variables del ámbito que lo rodea de tres maneras físicas distintas, lo cual determina qué trait implementará automáticamente el compilador:

1.  **Tomar en préstamo inmutable (`&T`):** El closure lee los datos del entorno.
2.  **Tomar en préstamo mutable (`&mut T`):** El closure puede modificar los datos del entorno.
3.  **Tomar propiedad (`T`):** El closure se adueña de los datos. Esto se fuerza explícitamente utilizando la palabra clave **`move`**, muy común al pasar closures a nuevos hilos de ejecución (*threads*):

```rust
fn main() {
    let lista = vec![1, 2, 3];

    // La palabra clave 'move' obliga a transferir la propiedad de 'lista' al closure
    let comprobar_propiedad = move || println!("Lista capturada por valor: {lista:?}");

    comprobar_propiedad();
    // println!("{lista:?}"); // Error: 'lista' ha sido movida al closure
}
```

#### Los Tres Traits de Closures (`Fn`, `FnMut` y `FnOnce`)
El compilador clasifica automáticamente cada closure en uno de los siguientes tres traits de la biblioteca estándar según cómo manipule las variables capturadas:

*   **`FnOnce` (Consumo):** El closure consume (toma propiedad de) las variables capturadas. Por lo tanto, **solo puede ser invocado una vez** en todo el programa, ya que tras la primera llamada los datos capturados se destruyen (*dropped*). Todos los closures implementan al menos `FnOnce`.
*   **`FnMut` (Mutación):** El closure muta el entorno pero no consume las variables. Puede ser invocado múltiples veces, pero requiere que la variable que almacena el closure sea marcada como mutable (`let mut`).
*   **`Fn` (Préstamo Inmutable):** El closure no muta ni consume las variables capturadas; solo las lee. Puede ser invocado concurrentemente y múltiples veces de forma libre.

---

### 2. Iteradores

El trait `Iterator` de Rust se define esencialmente de la siguiente manera:
```rust
pub trait Iterator {
    type Item; // Tipo asociado que representa el tipo de elemento
    fn next(&mut self) -> Option<Self::Item>; // Único método obligatorio
}
```

#### Naturaleza Perezosa (Laziness)
Los iteradores en Rust son **perezosos**. Esto significa que crear un iterador o encadenar transformaciones sobre él no ejecuta ningún cálculo real ni consume memoria del montón. La computación se inicia únicamente cuando se invoca un método que consume el iterador.

#### Adaptadores de Iteración vs. Adaptadores Consumidores

*   **Adaptadores de Iteración (Iterator Adaptors):** Toman un iterador y devuelven otro iterador modificado. Debido a la pereza, deben encadenarse con un consumidor para tener efecto. Ejemplos: `.map()`, `.filter()`, `.zip()`.
*   **Adaptadores Consumidores (Consuming Adaptors):** Consumen el iterador llamando internamente al método `.next()` hasta que devuelva `None`, retornando un valor final consolidado. Ejemplos: `.sum()`, `.collect()`, `.count()`, `.fold()`.

```rust
fn main() {
    let v = vec![1, 2, 3];

    // .iter() crea un iterador inmutable.
    // .map() es un adaptador de iteración (perezoso, no hace nada todavía).
    // .collect() es un adaptador consumidor que ejecuta la lógica y genera el nuevo Vec.
    let incrementados: Vec<i32> = v.iter().map(|x| x + 1).collect();

    assert_eq!(incrementados, vec![2, 3, 4]);
}
```

---

### 3. Refactorización Avanzada de `minigrep` con Iteradores

El uso de iteradores permite refactorizar el código de `minigrep` construido en el Capítulo 12, haciéndolo más conciso, expresivo y eficiente al evitar indexaciones directas y clones de memoria de cadenas.

#### `Config::build` usando Iteradores
En lugar de pasar un slice físico de strings `&[String]` y clonar sus elementos, pasamos el iterador de argumentos por valor y extraemos los valores secuencialmente:

```rust
// ch12_minigrep/src/lib.rs
impl Config {
    // Aceptamos cualquier tipo de iterador que produzca Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // El primer argumento es la ruta del ejecutable (lo consumimos y descartamos)
        args.next();

        let consulta = match args.next() {
            Some(arg) => arg,
            None => return Err("Falta el parámetro de consulta (query)."),
        };

        let ruta_archivo = match args.next() {
            Some(arg) => arg,
            None => return Err("Falta la ruta del archivo a buscar."),
        };

        let ignorar_mayusculas = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            consulta,
            ruta_archivo,
            ignorar_mayusculas,
        })
    }
}
```

#### `buscar` simplificado con Iteradores
Podemos reescribir la lógica de búsqueda de forma puramente declarativa reemplazando los bucles `for` por adaptadores funcionales:

```rust
pub fn buscar<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str> {
    contenido
        .lines() // Crea un iterador sobre las líneas
        .filter(|linea| linea.contains(consulta)) // Filtra las líneas que contienen la consulta
        .collect() // Consume el iterador y agrupa los resultados en un Vec
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Representación Física de los Closures en Memoria
En tiempo de compilación, un closure no es un puntero de función genérico. Para dar soporte a la captura del entorno sin costo en tiempo de ejecución, el compilador realiza lo siguiente:
1.  Genera una **estructura anónima autogenerada** única para el closure.
2.  Las variables capturadas del entorno se convierten en **campos físicos** dentro de esta estructura.
    *   Si la captura es por préstamo inmutable, el campo es de tipo `&T`.
    *   Si es por mutación, es `&mut T`.
    *   Si se usa `move`, el campo es de tipo `T`.

#### Layout en Stack de un Closure
Consideremos el siguiente código:
```rust
let offset = 10;
let sumar = |x| x + offset; // Captura 'offset' por referencia
```
El compilador genera un layout de memoria física equivalente a esto en el Stack:

```rust
// Representación lógica que crea el compilador
struct ClosureAnonimo<'a> {
    offset: &'a i32, // Campo de 8 bytes (puntero al valor en el Stack)
}
```
*   **Tamaño en memoria:** Si el closure captura una sola referencia, ocupará exactamente 8 bytes en una arquitectura de 64 bits.
*   **Zero-Sized Closures:** Si un closure no captura ninguna variable del entorno, la estructura generada estará vacía. En Rust, las estructuras vacías son **Tipos de Tamaño Cero (Zero-Sized Types - ZST)**. Por lo tanto, el closure ocupará **0 bytes** físicos en memoria y no consumirá ningún espacio en el Stack ni en el Heap, traduciéndose en una simple instrucción directa de salto en código máquina.

---

### La Abstracción de Costo Cero (Zero-Cost Abstractions) en Iteradores
El creador de C++, Bjarne Stroustrup, acuñó el término: *"lo que no usas, no lo pagas; y el código que escribes compila tan eficientemente como si lo hubieras optimizado a mano"*. Los iteradores de Rust son un ejemplo perfecto de esto.

Cuando se escribe:
```rust
let suma: i32 = v.iter().filter(|&&x| x > 5).map(|&x| x * 2).sum();
```
A primera vista, parece sumamente ineficiente en comparación con un bucle `for` indexado clásico: se crean múltiples objetos intermedios (`Filter`, `Map`), llamadas a funciones por cada elemento y closures.

#### ¿Cómo lo optimiza el compilador (LLVM)?
1.  **Inlining Agresivo:** El compilador de Rust sustituye las firmas de los métodos `.next()` de los adaptadores intermedios y los closures directamente en el cuerpo del bucle, eliminando por completo el coste de llamadas a funciones en la pila de CPU.
2.  **Fusión de Bucles (Loop Fusion) y Desenrrollado (Unrolling):** El compilador fusiona todas las etapas de transformación en un único ciclo de CPU.
3.  **Eliminación de Comprobación de Límites (Bound-Check Elimination):** En un bucle `for` imperativo tradicional con indexación directa (`v[i]`), el runtime de Rust debe validar en cada iteración que `i` no supere los límites del vector para evitar fugas de memoria, lo que añade una instrucción condicional en cada iteración. Con los iteradores, el compilador tiene garantías matemáticas de que el índice nunca superará el tamaño del vector, eliminando por completo las comprobaciones condicionales de límites y traduciéndose en instrucciones de ensamblador directas y lineales.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet de Métodos de Iteradores Comunes

| Método | Tipo de Adaptador | Firma Simplificada / Retorno | Propósito y Uso |
| :--- | :--- | :--- | :--- |
| `.iter()` | Adaptador de Origen | `Iter<'a, T>` | Crea un iterador que toma préstamos inmutables (`&T`) de los elementos. |
| `.iter_mut()` | Adaptador de Origen | `IterMut<'a, T>` | Crea un iterador que toma préstamos mutables (`&mut T`). |
| `.into_iter()` | Adaptador de Origen | `IntoIter<T>` | Consume la colección, tomando propiedad de sus elementos. |
| `.map(f)` | Iterator Adaptor | `Map<I, F>` | Transforma cada elemento aplicando el closure `f`. |
| `.filter(p)` | Iterator Adaptor | `Filter<I, P>` | Conserva solo los elementos que cumplen con el predicado `p`. |
| `.collect()` | Consuming Adaptor | `C` (Colección concreta) | Agrupa los elementos del iterador en una nueva colección física. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Intento de reusar variables capturadas por valor en closures `FnOnce`
❌ **Código Erróneo:**
```rust
fn ejecutar_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
    f(); // Error: f es FnOnce y no se puede llamar más de una vez
}
```
*   **Mensaje de Error:** `error[E0382]: use of moved value: 'f'`
*   ✔️ **Solución:** Si necesitas llamar al closure múltiples veces, debes restringir la firma genérica del parámetro de tipo para que implemente `FnMut` o `Fn` en lugar de `FnOnce`.

#### 2. Modificar una colección mientras se itera sobre ella
❌ **Código Erróneo:**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        v.push(*x); // Error: No puedes mutar 'v' mientras hay un préstamo activo en curso
    }
}
```
*   **Mensaje de Error:** `error[E0502]: cannot borrow 'v' as mutable because it is also borrowed as immutable`
*   ✔️ **Solución:** Recopila primero los datos necesarios mediante un adaptador de consumo como `.collect()` para liberar el préstamo inmutable, y realiza las inserciones después de que la iteración haya concluido:
    ```rust
    let mut v = vec![1, 2, 3];
    let elementos: Vec<i32> = v.iter().copied().collect();
    for x in elementos {
        v.push(x); // Correcto: El vector v ya no está prestado
    }
    ```

#### 3. Olvidar que los adaptadores de iteración son perezosos (Lazy Adaptors)
❌ **Código Erróneo:**
```rust
fn main() {
    let v = vec![1, 2, 3];
    // Compila, pero no hace nada en ejecución porque map es perezoso
    v.iter().map(|x| println!("Elemento: {x}")); 
}
```
*   **Advertencia del compilador:** `warning: unused 'Map' that must be used`
*   ✔️ **Solución:** Encadenar un adaptador consumidor que ejecute la lógica, como `.for_each(...)` o `.collect()`:
    ```rust
    v.iter().for_each(|x| println!("Elemento: {x}"));
    ```
