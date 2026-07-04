# Capítulo 04: Entendiendo el Modelo de Propiedad (Ownership)

El sistema de propiedad (*Ownership*) es la característica más distintiva de Rust y le permite garantizar seguridad de memoria sin necesidad de un recolector de basura (*Garbage Collector*) ni de una gestión de memoria manual propensa a errores.

---

## 1. Conceptos Fundamentales (Desde Cero)

### El Modelo de Ownership
En lenguajes como C o C++, el programador debe asignar y liberar explícitamente la memoria en el Heap, lo que causa errores graves como fugas de memoria o punteros colgantes. En lenguajes como Java o Go, un *Garbage Collector* busca de forma automática la memoria que ya no está en uso en tiempo de ejecución, lo que introduce pausas (*latencia*).

Rust adopta un tercer enfoque: la gestión de memoria se realiza a través de un **sistema de propiedad** gobernado por tres reglas fundamentales que el compilador comprueba estáticamente:

1.  **Cada valor en Rust tiene un dueño** (una variable).
2.  **Solo puede haber un dueño a la vez** para cualquier recurso.
3.  **Cuando el dueño sale del ámbito (*scope*), el valor se destruye** y la memoria se libera automáticamente.

#### El Ámbito (*Scope*) y la Función `drop`
El ámbito es la región del código dentro de la cual un elemento es válido. Comienza cuando se declara y termina cuando el bloque (delimitado por llaves `{}`) finaliza. Cuando una variable que posee recursos en el Heap sale de su ámbito, Rust llama automáticamente a un método especial llamado `drop`. Este método actúa como un destructor, liberando los recursos en el montón.

### Referencias y Préstamos (Borrowing)
Pasar la propiedad de una variable en cada llamada a función es ineficiente y engorroso. Para solucionar esto, Rust utiliza las **referencias**, permitiendo que el código acceda a un valor sin tomar posesión del mismo. A este acto de crear una referencia se le denomina **Préstamo (*Borrowing*)**.

#### Reglas de los Préstamos
Para prevenir la corrupción de memoria y condiciones de carrera, el compilador impone dos reglas de préstamo mutuamente excluyentes en cualquier momento dado dentro de un ámbito:
1.  Puedes tener **cualquier número de referencias inmutables** (`&T`) a un recurso.
2.  O puedes tener **exactamente una referencia mutable** (`&mut T`) a un recurso.
3.  **Las referencias deben ser siempre válidas:** Rust prohíbe las referencias colgantes (*dangling references*), asegurando que el recurso de memoria original viva más tiempo que la referencia que apunta a él.

### Concepto de Slices (Porciones)
Un slice es una referencia a una secuencia contigua de elementos dentro de una colección (como una cadena o un arreglo). Al ser una referencia, **no posee** los datos subyacentes. Permite acceder de manera segura a una parte específica de una estructura de datos sin realizar copias en memoria.

---

## 2. Anatomía y Semántica de la Sintaxis

### Operadores de Ownership y Referencias
*   `&`: Operador de referencia inmutable. Permite leer el valor apuntado sin modificarlo ni tomar su propiedad.
*   `&mut`: Operador de referencia mutable. Permite leer y escribir en la memoria apuntada sin tomar su propiedad.
*   `*`: Operador de desreferenciación. Permite acceder al valor al que apunta una referencia para leerlo o escribir en él.

```rust
fn main() {
    let mut s1 = String::from("hola");

    // Firma que toma una referencia inmutable (Lectura)
    let longitud = calcular_longitud(&s1); 

    // Firma que toma una referencia mutable (Escritura)
    modificar_cadena(&mut s1); 
    
    // Al pasar s1 a esta función, transferimos la propiedad (Move)
    consumir_cadena(s1);
    // s1 ya no es válida aquí.
}

fn calcular_longitud(s: &String) -> usize {
    s.len() // s es una referencia a String. No es dueño de los datos.
}

fn modificar_cadena(s: &mut String) {
    s.push_str(", mundo"); // Permite modificar el valor subyacente.
}

fn consumir_cadena(s: String) {
    println!("Cadena consumida: {s}");
} // Aquí se llama automáticamente a `drop` para liberar la memoria de s.
```

### Sintaxis de Slices (String y Array Slices)
Un slice se define utilizando un rango encerrado entre corchetes:

```rust
let s = String::from("hola mundo");

// [inicio..fin] es un rango exclusivo en el extremo final (no incluye el índice 'fin')
let hola: &str = &s[0..4];  // Apunta a "hola"
let mundo: &str = &s[5..10]; // Apunta a "mundo"

// Slices de arreglos genéricos
let array = [1, 2, 3, 4, 5];
let slice_array: &[i32] = &array[1..3]; // Hace referencia a [2, 3]
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria (Stack vs. Heap)
*   **El Stack (Pila):** Almacena datos con tamaño fijo y conocido en tiempo de compilación. Es extremadamente rápido debido a que los datos están contiguos y el procesador solo necesita mover un puntero de pila (*stack pointer*).
*   **El Heap (Montículo):** Almacena datos dinámicos cuya capacidad puede cambiar (ej. `String`). Requiere una llamada al sistema operativo para reservar bloques de memoria y devuelve un puntero con la dirección de inicio.

```
Pila (Stack)                             Montículo (Heap)
[ Variable 's1' ]                        [ Datos reales ]
+-----------+---------+                  +-------+-------+
| Campo     | Valor   |                  | Index | Valor |
+-----------+---------+                  +-------+-------+
| ptr       | --------+----------------> |   0   |  'h'  |
| len       | 4       |                  |   1   |  'o'  |
| capacidad | 4       |                  |   2   |  'l'  |
+-----------+---------+                  |   3   |  'a'  |
                                         +-------+-------+
```

### Semántica de Transferencia: Move vs. Clone vs. Copy

#### El Movimiento (Move)
Cuando asignamos una variable que gestiona memoria en el Heap (como `String`), Rust copia el descriptor en el Stack (el puntero `ptr`, la longitud `len` y la capacidad `capacidad`), pero **invalida** la variable original. No realiza una copia de los datos del Heap porque sería una operación costosa.

```rust
let s1 = String::from("hola");
let s2 = s1; // Ocurre un Move. s1 queda invalidada.
```
Si Rust no invalidara `s1`, cuando ambas variables salieran de ámbito, ambas intentarían liberar el mismo bloque de memoria del montón. Esto produciría un error de **Doble Liberación (*Double Free Error*)**, el cual puede corromper la memoria del sistema.

#### El Clonado (Clone)
Para duplicar tanto el descriptor en la pila como los datos reales ubicados en el Heap, se debe llamar explícitamente a `.clone()`. Esta operación es costosa y requiere tiempo de ejecución para realizar la nueva asignación de memoria.

#### El Trait `Copy`
Los tipos que se almacenan completamente en el Stack (como enteros, booleanos y arreglos de tamaño fijo que contienen elementos `Copy`) implementan el trait `Copy`. Al asignar una variable `Copy` a otra, se realiza una copia bit a bit directa en la pila. La variable original sigue siendo completamente válida tras la asignación.

#### El Layout de un Slice
Un slice (por ejemplo, `&str` o `&[i32]`) es un **Fat Pointer** (puntero gordo) en el Stack. Ocupa exactamente **16 bytes** en una arquitectura de 64 bits y consta de dos elementos:
1.  Un puntero a los datos reales (8 bytes).
2.  La longitud del slice (8 bytes).

```
Fat Pointer de un Slice en el Stack:
+-------------------+--------------------+
| ptr (8 bytes)     | longitud (8 bytes) |
+-------------------+--------------------+
```

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Comparación de Transferencias de Propiedad

| Tipo de Dato | Operación | Comportamiento en Memoria | Identificador de Origen |
| :--- | :--- | :--- | :--- |
| **No-Copy** (ej: `String`) | `let b = a;` | **Move**: Se copia descriptor de Pila, se invalida origen. | **Inválido** (deja de existir) |
| **No-Copy** (ej: `String`) | `let b = a.clone();`| **Clone**: Copia descriptor en Pila y reserva/duplica datos en Heap. | **Válido** (independiente) |
| **Copy** (ej: `i32`) | `let b = a;` | **Copy**: Copia directa bit-a-bit en el Stack. | **Válido** (idéntico) |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Usar una variable después de que su propiedad fue movida (Move)
Intentar leer datos de una variable cuya propiedad fue transferida anteriormente a otra entidad:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let s1 = String::from("hola");
    let s2 = s1; // La propiedad se mueve a s2
    println!("{s1}"); // Error: s1 ya no es válida
}
```
*   **Mensaje de Error:** `error[E0382]: borrow of moved value: `s1``
*   ✔️ **Solución:** Clonar la variable si necesitas duplicar los datos físicos en el montón o pasar una referencia inmutable en lugar de transferir la propiedad completa:
    ```rust
    let s2 = s1.clone(); // Opción A (Clonado)
    let s2 = &s1;        // Opción B (Préstamo)
    ```

#### 2. Duplicar préstamos mutables en un mismo ámbito
Intentar crear más de una referencia mutable al mismo recurso simultáneamente en el mismo bloque:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let mut x = 5;
    let r1 = &mut x;
    let r2 = &mut x; // Error: segundo préstamo mutable
    println!("{}, {}", r1, r2);
}
```
*   **Mensaje de Error:** `error[E0499]: cannot borrow `x` as mutable more than once at a time`
*   ✔️ **Solución:** Rust prohíbe el aliasing mutable para evitar carreras de datos. Asegúrate de que el tiempo de vida de la primera referencia mutable finalice antes de crear la segunda:
    ```rust
    let mut x = 5;
    {
        let r1 = &mut x;
        *r1 += 1;
    } // r1 sale de ámbito y su préstamo es devuelto
    let r2 = &mut x; // Ahora es válido
    ```

#### 3. Coexistencia de referencias mutables e inmutables
Intentar modificar un valor mutable mientras existen lecturas inmutables activas:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let mut x = 5;
    let r1 = &x;     // Préstamo inmutable
    let r2 = &mut x; // Error: préstamo mutable mientras r1 está activo
    println!("{}", r1);
}
```
*   **Mensaje de Error:** `error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable`
*   ✔️ **Solución:** El valor original no debe mutar mientras existan lectores inmutables asumiendo la invariabilidad del dato. Mueve el bloque de uso de las referencias inmutables antes de declarar el préstamo mutable:
    ```rust
    let mut x = 5;
    let r1 = &x;
    println!("{}", r1); // Último uso de r1
    
    let r2 = &mut x; // Válido: r1 ya no está activo
    ```

#### 4. Retornar una referencia a una variable local (Dangling Reference)
Intentar retornar una referencia a memoria que se destruirá al finalizar la función:
❌ **Código Erróneo:**
```rust,compile_fail
fn obtener_referencia() -> &String {
    let s = String::from("hola");
    &s // Error: s se destruye al final de la función
}
```
*   **Mensaje de Error:** `error[E0515]: cannot return reference to local variable `s``
*   ✔️ **Solución:** En lugar de retornar una referencia, transfiere la propiedad del dato directamente para que la función llamadora pase a ser la dueña de la memoria:
    ```rust
    fn obtener_referencia() -> String {
        let s = String::from("hola");
        s // Devolvemos el valor completo (Move)
    }
    ```
