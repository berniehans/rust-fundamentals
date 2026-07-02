# Capítulo 08: Colecciones Comunes

Este documento proporciona un análisis exhaustivo y de bajo nivel de las colecciones dinámicas provistas por la biblioteca estándar de Rust, detallando su semántica, layout de memoria física, consideraciones de rendimiento y optimizaciones de seguridad implementadas por el compilador.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Colecciones Dinámicas y la Memoria Heap
A diferencia de los tipos de datos primitivos como los arreglos fijados (`[T; N]`) y las tuplas, cuyos tamaños deben conocerse con precisión milimétrica en tiempo de compilación para reservarse en el Stack, las **Colecciones Comunes** de Rust están diseñadas para gestionar flujos dinámicos de datos cuyo volumen puede variar arbitrariamente durante la ejecución del programa. 

Para dotar al software de esta capacidad de crecimiento y contracción, las colecciones almacenan sus datos reales en el **Montículo (Heap)**. El Stack únicamente almacena descriptores de metadatos de tamaño constante que apuntan a las asignaciones del montón.

Rust implementa tres colecciones fundamentales en su biblioteca estándar:
1.  **Vectores (`Vec<T>`):** Secuencias contiguas de elementos del mismo tipo.
2.  **Cadenas de Texto (`String`):** Colección de bytes que almacena texto codificado obligatoriamente bajo el estándar UTF-8.
3.  **Mapas Hash (`HashMap<K, V>`):** Colección de pares clave-valor que utiliza una función hashing para indexar los elementos en memoria.

### El Desafío de Unicode y la Indexación de Texto
Muchos programadores noveles que vienen de lenguajes como Python o C# asumen que una cadena de texto es un arreglo simple de caracteres y que es válido acceder a un carácter en particular mediante un índice entero (ej. `s[0]`). En Rust, esto está estrictamente prohibido a nivel sintáctico.

La razón es la codificación **UTF-8**:
*   Un carácter lógico Unicode (denominado valor escalar Unicode o *char*) no tiene un tamaño fijo de bytes; puede medir entre 1 y 4 bytes físicos en memoria.
*   Si Rust permitiera indexar `s[0]`, el compilador no podría garantizar que se obtuviera un carácter válido en tiempo constante $O(1)$. Si el primer carácter es de 3 bytes (ej. un carácter en kanji japonés como `'𠜎'`), acceder al byte en la posición `0` devolvería un byte truncado incompleto, rompiendo la coherencia de la codificación y provocando un estado de memoria inválido.
*   Para prevenir este tipo de bugs, Rust exige recorrer secuencialmente la cadena mediante iteradores específicos para buscar caracteres lógicos, sacrificando la velocidad sintáctica en favor de la corrección absoluta.

---

## 2. Anatomía y Semántica de la Sintaxis

### Vectores (`Vec<T>`)
Un vector almacena valores del mismo tipo de forma contigua en el montón.

```rust
fn main() {
    // 1. Inicialización vacía (Requiere anotación de tipo si no hay uso posterior)
    let mut v: Vec<i32> = Vec::new();

    // 2. Inicialización rápida mediante macro 'vec!'
    let mut v2 = vec![1, 2, 3];

    // 3. Modificación
    v2.push(4); // Añade al final

    // 4. Lectura de elementos
    // Método A: Indexación directa (Produce pánico inmediato si el índice no existe)
    let tercer: &i32 = &v2[2];

    // Método B: Uso del método seguro .get() (Retorna Option<&T>)
    match v2.get(2) {
        Some(elemento) => println!("El elemento es {elemento}"),
        None => println!("Índice fuera de límites."),
    }

    // 5. Iteración mutable
    for elemento in &mut v2 {
        *elemento += 50; // Desreferenciación para editar en sitio
    }
}
```

### Cadenas de Texto (`String`)
En Rust, el tipo de cadena fundamental del núcleo del lenguaje es el slice de cadena inmutable **`str`**, habitualmente visto como préstamo (`&str`). El tipo **`String`** es provisto por la biblioteca estándar, es mutable y posee propiedad de sus bytes en el montón.

```rust
fn main() {
    // Inicialización
    let mut s1 = String::new();
    let s2 = "contenido inicial".to_string();
    let mut s3 = String::from("hola");

    // Concatenación
    s3.push_str(" mundo"); // Añade un slice de cadena
    s3.push('!');         // Añade un único carácter (char)

    // El operador '+' (Consume la primera cadena y toma referencias del resto)
    // Firma interna: fn add(self, s: &str) -> String
    let s4 = String::from("hola");
    let s5 = String::from("mundo");
    let s6 = s4 + &s5; // s4 se ha movido (invalida); s5 se presta como referencia

    // Macro format! (No toma propiedad, ideal para concatenar múltiples cadenas)
    let s_format = format!("{s5}-{s6}");
}
```

#### Iteración de Cadenas
Debido a la naturaleza UTF-8, debemos especificar cómo deseamos recorrer la cadena:
```rust
let cadena = "🌎";

// Iterar por caracteres lógicos (Unicode Scalar Values)
for c in cadena.chars() {
    println!("{c}"); // Imprime '🌎' (1 iteración)
}

// Iterar por bytes físicos representados
for b in cadena.bytes() {
    println!("{b}"); // Imprime 4 bytes individuales (4 iteraciones)
}
```

### Mapas Hash (`HashMap<K, V>`)
El mapa hash asocia claves a valores. Requiere ser importado explícitamente desde `std::collections`.

```rust
use std::collections::HashMap;

fn main() {
    let mut puntajes = HashMap::new();

    // Inserción de elementos
    puntajes.insert(String::from("Azul"), 10);
    puntajes.insert(String::from("Amarillo"), 50);

    // Acceso seguro (Retorna Option<&V>)
    let nombre_equipo = String::from("Azul");
    let puntaje: Option<&i32> = puntajes.get(&nombre_equipo);

    // Iteración
    for (clave, valor) in &puntajes {
        println!("{clave}: {valor}");
    }

    // Inserción condicional (Entry API)
    // Inserta la clave con valor 25 SOLO si no existe previamente
    puntajes.entry(String::from("Rojo")).or_insert(25);
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria de Vectores y Cadenas
En el Stack de 64 bits, tanto `Vec<T>` como `String` ocupan exactamente **24 bytes** estructurados en tres bloques de 8 bytes:

```
Stack (Pila - 24 Bytes)                    Heap (Montículo)
+-----------+-------------------------+    +-------+-------+
| Campo     | Valor                   |    | Index | Valor |
+-----------+-------------------------+    +-------+-------+
| pointer   | ------------------------+--> |   0   |  10   |
| length    | 3                       |    |   1   |  20   |
| capacity  | 4                       |    |   2   |  30   |
+-----------+-------------------------+    |   3   | (vací)|
                                           +-------+-------+
```

1.  **Pointer (8 bytes):** Dirección física de inicio de los elementos en el Heap.
2.  **Length (8 bytes):** El número de elementos válidos que contiene la colección en la actualidad.
3.  **Capacity (8 bytes):** La cantidad máxima de elementos que caben en la sección reservada en el Heap.

#### El Algoritmo de Reasignación (Reallocation)
Cuando invocamos `.push()` y el número de elementos iguala a la capacidad (`length == capacity`):
1.  Rust solicita al asignador de memoria del sistema (*allocator*) reservar un nuevo bloque en el Heap, duplicando la capacidad anterior (estrategia de crecimiento por factor de 2).
2.  Copia bit a bit los elementos existentes del bloque antiguo al nuevo bloque.
3.  Libera (*free*) el bloque de memoria Heap antiguo.
4.  Actualiza el `pointer` y la `capacity` en el descriptor de la pila.

Este proceso de reasignación es de costo $O(N)$ en el peor caso. Sin embargo, al duplicarse la capacidad en cada reasignación, el coste se amortiza, logrando una complejidad de **inserción amortizada $O(1)$**. Para evitar reasignaciones en sistemas de alto rendimiento, se puede pre-asignar memoria usando `Vec::with_capacity(cap)`.

### El Algoritmo de Hashing de `HashMap`
*   **SipHash 1-3:** Por defecto, `HashMap` de Rust utiliza la función criptográfica SipHash 1-3. Este algoritmo proporciona resistencia contra ataques de denegación de servicio (DoS) basados en colisión de Hash. Si un atacante pudiera adivinar el algoritmo de hash simple de tu servidor web, podría inyectar solicitudes con claves diseñadas para caer en el mismo bucket del mapa, transformando búsquedas $O(1)$ rápidas en búsquedas secuenciales $O(N)$ lentas, saturando la CPU.
*   **Costo de SipHash:** Aunque es altamente seguro, SipHash introduce un sobrecosto de rendimiento de cálculo mayor que algoritmos simples (como FNV). Si tu aplicación no está expuesta a ataques externos y requiere la máxima velocidad de procesamiento de claves, puedes sustituir el hasher implementando el trait `BuildHasher` (ej. usando la biblioteca `fnv` o `ahash` de crates.io).

### Seguridad en Memoria: Prevención de Invalidez de Referencias
El compilador utiliza las reglas de préstamo para impedir una categoría de errores catastróficos presentes en C++: la **invalidez de referencias de iteradores**.

Analicemos físicamente el siguiente error:
```rust
let mut v = vec![1, 2, 3];
let primer = &v[0]; // Préstamo inmutable (primer apunta a la dirección de memoria Heap original)
v.push(4);          // Préstamo mutable e intento de empujar
println!("{primer}"); // error[E0502]: cannot borrow 'v' as mutable because it is also borrowed as immutable
```

**¿Por qué el compilador es tan estricto?**
Si `v.push(4)` detecta que `length == capacity`, provocará una **reasignación**. La memoria original donde residían `[1, 2, 3]` se liberará en el montón y los elementos se moverán a otra dirección física. Si Rust permitiera usar la variable `primer` después del `.push()`, `primer` apuntaría a una dirección de memoria que ya ha sido liberada y reasignada a otro proceso de tu servidor, provocando una **lectura de puntero colgante (Use-After-Free)**.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

| Sintaxis / Patrón Exacto | Propósito y Caso de Uso | Error Típico del Compilador (y Solución) |
| :--- | :--- | :--- |
| `let mut v = Vec::new();`<br>`v.push(val);` | Crear e incrementar dinámicamente un vector de tamaño variable. | Error de inferencia de tipo al declarar un vector vacío sin uso posterior:<br>❌ `let v = Vec::new();`<br>`error[E0282]: type annotations needed`<br>✔️ **Solución:** Añadir el tipo genérico explícito (`let v: Vec<i32> = Vec::new();`) o dejar que se infiera mediante el primer `.push()`. |
| `let val = &v[index];` | Indexación directa en un vector por posición. | Intentar indexar fuera de los límites de tamaño en tiempo de ejecución:<br>❌ Acceso a `v[10]` cuando el tamaño es menor.<br>`panic: thread 'main' panicked at 'index out of bounds'`<br>✔️ **Solución:** Utilizar el método seguro `v.get(index)` y validar el resultado mediante coincidencia de patrones. |
| `let slice = &s[0..3];` | Slice de cadena para obtener una vista parcial inmutable de un `String`. | Cortar la cadena en medio de un carácter Unicode multicanal (ej. emojis):<br>❌ Intentar obtener un rango que rompe los bytes de un carácter.<br>`panic: thread 'main' panicked at 'byte index 3 is not a char boundary'`<br>✔️ **Solución:** Utilizar rangos que coincidan exactamente con límites de caracteres válidos o procesar la cadena mediante iteradores `.chars()`. |
| `let s3 = s1 + &s2;` | Concatenación de cadenas utilizando el operador de adición `+`. | Intentar usar la cadena `s1` después de realizar la concatenación:<br>❌ `println!("{}", s1);`<br>`error[E0382]: borrow of moved value: 's1'`<br>✔️ **Solución:** Recordar que el operador de suma toma la propiedad de la cadena izquierda. Si deseas conservar ambas variables, utiliza la macro `format!` (ej. `let s3 = format!("{s1}{s2}");`). |
| `map.insert(k, v);` | Inserción de una clave y valor dentro de un `HashMap`. | Intentar usar variables de tipo no-Copy (como `String`) tras insertarlas en el mapa:<br>❌ `let k = String::from("A"); map.insert(k, 1); println!("{}", k);`<br>`error[E0382]: borrow of moved value: 'k'`<br>✔️ **Solución:** Clonar la clave si es necesario conservarla (`map.insert(k.clone(), 1)`), o realizar todas las lecturas de `k` antes de transferir su propiedad al mapa. |
