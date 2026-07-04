# Capítulo 08: Colecciones Comunes

Este documento proporciona un análisis de las colecciones dinámicas de la biblioteca estándar de Rust, detallando su semántica, layout de memoria física, consideraciones de rendimiento y optimizaciones de seguridad implementadas por el compilador.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Colecciones Dinámicas y la Memoria Heap
A diferencia de los tipos de datos primitivos como los arreglos fijados (`[T; N]`) y las tuplas, cuyas capacidades se fijan en compilación y se reservan en el Stack, las **Colecciones Comunes** de Rust están diseñadas para gestionar volúmenes variables de datos durante la ejecución, almacenando sus datos reales en el **Heap (Montículo)**. El Stack únicamente almacena descriptores de metadatos de tamaño constante.

Rust implementa tres colecciones fundamentales en su biblioteca estándar:
1.  **Vectores (`Vec<T>`):** Secuencias contiguas de elementos del mismo tipo.
2.  **Cadenas de Texto (`String`):** Colección de bytes que almacena texto codificado obligatoriamente bajo el estándar UTF-8.
3.  **Mapas Hash (`HashMap<K, V>`):** Colección de pares clave-valor indexados mediante hashing.

### El Desafío de Unicode y la Indexación de Texto
Muchos programadores asumen que es válido acceder a un carácter en particular mediante un índice entero (ej. `s[0]`). En Rust, esto está estrictamente prohibido a nivel sintáctico.

La razón es la codificación **UTF-8**:
*   Un carácter lógico Unicode (denominado valor escalar Unicode o *char*) no tiene un tamaño fijo de bytes; puede medir entre 1 y 4 bytes físicos en memoria.
*   Si Rust permitiera indexar `s[0]`, el compilador no podría garantizar que se obtuviera un carácter válido en tiempo constante $O(1)$. Si el primer carácter es de 3 bytes (ej. un carácter en kanji japonés como `'𠜎'`), acceder al byte en la posición `0` devolvería un byte truncado incompleto, rompiendo la coherencia de la codificación y provocando un estado de memoria inválido.

---

## 2. Anatomía y Semántica de la Sintaxis

### Vectores (`Vec<T>`)
Un vector almacena valores del mismo tipo de forma contigua en el montón.

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new(); // Inicialización vacía
    let mut v2 = vec![1, 2, 3];        // Inicialización rápida mediante macro
    v2.push(4);                       // Añade al final

    // Lectura de elementos
    let tercer: &i32 = &v2[2]; // Método A: Indexación (Pánico si no existe)

    match v2.get(2) {          // Método B: get() seguro (Retorna Option<&T>)
        Some(elemento) => println!("El elemento es {elemento}"),
        None => println!("Índice fuera de límites."),
    }
}
```

### Cadenas de Texto (`String`)
El tipo `str` (generalmente visto como préstamo `&str`) representa slices de cadena inmutables. El tipo `String` es mutable, de tamaño variable y posee propiedad de sus bytes en el montón.

```rust
fn main() {
    let mut s = String::from("hola");
    s.push_str(" mundo"); // Añade un slice de cadena
    s.push('!');         // Añade un único carácter

    // El operador '+' (Consume la primera cadena y toma referencias del resto)
    let s1 = String::from("hola ");
    let s2 = String::from("mundo");
    let s3 = s1 + &s2; // s1 se ha movido (invalida); s2 se presta
}
```

#### Iteración de Cadenas
Debido a la naturaleza UTF-8, debemos especificar cómo deseamos recorrer la cadena:
```rust
let cadena = "🌎";

// Iterar por caracteres lógicos (Unicode Scalar Values)
for c in cadena.chars() { println!("{c}"); } // 1 iteración

// Iterar por bytes físicos representados
for b in cadena.bytes() { println!("{b}"); } // 4 iteraciones
```

### Mapas Hash (`HashMap<K, V>`)
Asocia claves a valores. Requiere ser importado explícitamente desde `std::collections`.

```rust
use std::collections::HashMap;

fn main() {
    let mut puntajes = HashMap::new();
    puntajes.insert(String::from("Azul"), 10);

    // Acceso seguro (Retorna Option<&V>)
    let puntaje = puntajes.get("Azul");

    // Inserción condicional (Entry API)
    // Inserta la clave con valor 50 SOLO si no existe previamente
    puntajes.entry(String::from("Amarillo")).or_insert(50);
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
2.  **Length (8 bytes):** El número de elementos válidos que contiene la colección actualmente.
3.  **Capacity (8 bytes):** La cantidad máxima de elementos que caben en la sección reservada en el Heap.

#### El Algoritmo de Reasignación (Reallocation)
Cuando invocamos `.push()` y el número de elementos iguala a la capacidad (`length == capacity`):
1.  Rust solicita al asignador de memoria del sistema (*allocator*) reservar un nuevo bloque en el Heap, duplicando la capacidad anterior (factor de crecimiento 2).
2.  Copia los elementos existentes del bloque antiguo al nuevo bloque.
3.  Libera (*free*) el bloque de memoria Heap antiguo.
4.  Actualiza el `pointer` y la `capacity` en el descriptor de la pila.

Este proceso de reasignación es de costo $O(N)$ en el peor caso. Sin embargo, al duplicarse la capacidad en cada reasignación, el coste se amortiza, logrando una complejidad de **inserción amortizada $O(1)$**. Para evitar reasignaciones en sistemas de alto rendimiento, se puede pre-asignar memoria usando `Vec::with_capacity(cap)`.

### El Algoritmo de Hashing de `HashMap`
Por defecto, `HashMap` de Rust utiliza la función criptográfica SipHash 1-3. Este algoritmo proporciona resistencia contra ataques de denegación de servicio (DoS) basados en colisión de Hash. Si un atacante pudiera adivinar el algoritmo de hash simple de tu servidor web, podría inyectar solicitudes con claves diseñadas para caer en el mismo bucket del mapa, transformando búsquedas $O(1)$ rápidas en búsquedas secuenciales $O(N)$ lentas, saturando la CPU.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen de Operaciones en Colecciones

| Colección | Inicialización | Inserción / Edición | Acceso Seguro |
| :--- | :--- | :--- | :--- |
| `Vec<T>` | `let mut v = Vec::new();` | `v.push(valor)` | `v.get(index)` -> `Option<&T>` |
| `String` | `let mut s = String::new();` | `s.push_str("txt")` | `s.get(rango)` -> `Option<&str>` |
| `HashMap<K, V>` | `let mut m = HashMap::new();` | `m.insert(k, v)` | `m.get(&k)` -> `Option<&V>` |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Invalidez de referencias de iteradores (Iterator Invalidation)
Intentar modificar la capacidad de un vector (ej: mediante `.push()`) mientras sostienes referencias físicas a sus elementos internos:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let mut v = vec![1, 2, 3];
    let primer = &v[0]; // Préstamo inmutable de un elemento
    v.push(4);          // Error: v se toma como mutable para empujar
    println!("{primer}"); // Uso posterior del préstamo
}
```
*   **Mensaje de Error:** `error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable`
*   ✔️ **Solución:** Si `v.push()` provoca una **reasignación**, la memoria del Heap original se liberará y los elementos se moverán a otra dirección física. Si Rust permitiera usar la variable `primer` después del `.push()`, `primer` apuntaría a una dirección de memoria libre, provocando un fallo catastrófico (Use-After-Free). Asegúrate de realizar lecturas antes de mutar la estructura, o copia los datos usando `.clone()` o `.copied()`:
    ```rust
    let mut v = vec![1, 2, 3];
    let primer = v[0]; // Copiamos el valor (i32 implementa Copy)
    v.push(4);          // Válido
    println!("{primer}");
    ```

#### 2. Indexación directa de caracteres en cadenas con números enteros
Intentar acceder a la posición de un carácter lógico indexando la cadena directamente:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let s = String::from("hola");
    let c = s[0]; // Error: no se puede indexar String con enteros
}
```
*   **Mensaje de Error:** `error[E0277]: the type `String` cannot be indexed by `{integer}``
*   ✔️ **Solución:** Rust prohíbe esto porque la indexación de cadenas no puede ocurrir en tiempo constante $O(1)$ debido al formato UTF-8 de tamaño variable. Utiliza iteradores explícitos o slices validados para extraer los datos de forma segura:
    ```rust
    // Obtener el primer carácter lógico de forma segura
    let primer_char = s.chars().next(); // Retorna Option<char>
    ```

#### 3. Uso posterior de variables movidas al insertarlas en un HashMap
Intentar reutilizar variables cuya propiedad fue transferida al insertarlas en la colección:
❌ **Código Erróneo:**
```rust,compile_fail
use std::collections::HashMap;

fn main() {
    let mut mapa = HashMap::new();
    let clave = String::from("clave_secreta");
    let valor = String::from("datos");

    mapa.insert(clave, valor); // clave y valor se mueven al mapa
    
    // Error: 'clave' ya no es dueña del String
    println!("Clave usada: {clave}"); 
}
```
*   **Mensaje de Error:** `error[E0382]: borrow of moved value: `clave``
*   ✔️ **Solución:** Si necesitas conservar la variable para otros fines, clónala al insertarla o realiza todas las operaciones previas a la inserción en la colección:
    ```rust
    mapa.insert(clave.clone(), valor);
    println!("Clave usada: {clave}"); // Válido
    ```
