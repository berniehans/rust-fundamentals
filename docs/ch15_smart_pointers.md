# Capítulo 15: Punteros Inteligentes (Smart Pointers)

Este documento proporciona un análisis exhaustivo de los punteros inteligentes en Rust. Se detallan el funcionamiento de la memoria en el montón (Heap), la semántica de la propiedad compartida, el patrón de mutabilidad interna en tiempo de ejecución, y los traits fundamentales de control de flujo de bajo nivel: `Deref` y `Drop`.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es un Puntero Inteligente?
En ciencias de la computación, un puntero es una variable que contiene una dirección física de memoria. En Rust, las referencias comunes (`&T` y `&mut T`) son punteros simples que únicamente toman prestados datos de forma inmutable o mutable. No tienen metadatos ni capacidades de control adicionales.

Un **Puntero Inteligente (Smart Pointer)** es una estructura de datos que actúa como un puntero pero incorpora metadatos y lógica adicional de control de flujo. En Rust, los punteros inteligentes suelen poseer la propiedad (*Ownership*) de los datos a los que apuntan, a diferencia de las referencias ordinarias.

### Los Traits Pilares: `Deref` y `Drop`
Cualquier estructura en Rust puede actuar como un puntero inteligente si implementa estos dos traits:
1.  **`std::ops::Deref`:** Permite personalizar el comportamiento del operador de desreferencia `*`. Facilita la **coerción de desreferencia (Deref Coercion)**, que traduce automáticamente referencias a punteros inteligentes (como `&Box<String>`) a referencias de tipos internos (como `&str`).
2.  **`std::ops::Drop`:** Permite ejecutar código personalizado cuando el puntero sale de su ámbito lúdico (*scope*). Es el equivalente a los destructores en C++, liberando memoria y sockets de forma automática.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. `Box<T>` (Asignación en el Heap)
`Box<T>` es el puntero inteligente más sencillo. Permite almacenar datos en el montón (Heap) en lugar del Stack. En el Stack únicamente reside el puntero de 8 bytes que apunta a la dirección del montón.

```rust
fn main() {
    // Guarda el entero 5 en el Heap
    let b = Box::new(5);
    println!("b = {}", *b); // Desreferencia física para leer
}
```
*   **Caso de uso principal (Tipos Recursivos):** En Rust, el tamaño de un tipo debe conocerse en tiempo de compilación. Las estructuras recursivas (como una Lista Enlazada o un Árbol) no tienen un tamaño predecible a menos que se use un puntero para romper la recursión:

```rust
// Sin Box, esto no compila porque el tamaño sería infinito
enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}
```

---

### 2. `Rc<T>` (Conteo de Referencias)
Hay casos donde múltiples partes de un programa necesitan leer el mismo valor en el Heap y no es posible definir un único propietario único. `Rc<T>` (Reference Counted) habilita la **propiedad compartida inmutable**.
*   **Nota:** `Rc<T>` **no es seguro para subprocesos (not thread-safe)**. Solo debe usarse en un único hilo.

```rust
use std::rc::Rc;

fn main() {
    let dato_compartido = Rc::new(String::from("datos"));
    
    // Rc::clone no clona el String en el Heap; solo incrementa el contador de referencias
    let _a = Rc::clone(&dato_compartido);
    let _b = Rc::clone(&dato_compartido);

    println!("Contador de referencias: {}", Rc::strong_count(&dato_compartido)); // Imprime 3
}
```

---

### 3. `RefCell<T>` (Mutabilidad Interna)
Las reglas de préstamos de Rust impiden tener referencias mutables si existen referencias inmutables. El patrón de **Mutabilidad Interna (Interior Mutability)** permite modificar datos incluso cuando están resguardados detrás de una referencia inmutable.
`RefCell<T>` traslada las comprobaciones de préstamo de tiempo de compilación a **tiempo de ejecución**.

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);

    // borrow() obtiene una referencia inmutable Ref<T>
    // borrow_mut() obtiene una referencia mutable RefMut<T>
    {
        let mut y = x.borrow_mut();
        *y += 10;
    } // y sale de ámbito aquí, liberando el préstamo mutable

    println!("Valor modificado: {:?}", x.borrow()); // Imprime 15
}
```

---

### 4. Evitar Memory Leaks con `Weak<T>`
Si creas referencias cíclicas (el elemento A apunta a B, y B apunta a A) usando `Rc` y `RefCell`, el contador de referencias nunca llegará a 0, causando una **fuga de memoria (memory leak)** en el montón.
Para resolverlo, Rust provee referencias débiles (`Weak<T>`) mediante `Rc::downgrade`:
*   `Rc::clone` incrementa el contador fuerte `strong_count`.
*   `Rc::downgrade` devuelve un puntero `Weak<T>` e incrementa el contador débil `weak_count`.
*   Para leer un puntero débil, primero debes convertirlo en fuerte usando `.upgrade()`, el cual devuelve un `Option<Rc<T>>` en caso de que el dato original haya sido destruido.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout Físico en Memoria Stack/Heap

#### 1. `Box<T>`
En una arquitectura de 64 bits, `Box<T>` mide exactamente 8 bytes en el Stack, guardando la dirección de inicio del dato en el Heap.
```
Stack (8 bytes)               Heap
+-----------------+          +-----------------+
| Address (0x500) | -------> | Payload (Value) |
+-----------------+          +-----------------+
```

#### 2. `Rc<T>`
`Rc<T>` no apunta directamente al payload en el montón; apunta a una estructura interna en el Heap que agrupa:
1.  El contador fuerte (`strong_count`, de tipo `usize`).
2.  El contador débil (`weak_count`, de tipo `usize`).
3.  El valor real del payload.

```
Stack (8 bytes)               Heap (Estructura interna de Rc)
+-----------------+          +----------------------------------+
| Address (0x700) | -------> | strong_count | weak_count | Value|
+-----------------+          +----------------------------------+
```
Cuando llamas a `Rc::clone`, se ejecuta una instrucción rápida de incremento a nivel de CPU sobre `strong_count` (coste $O(1)$ sin copia física). Cuando el clon sale de ámbito, decrementa el valor. Si `strong_count` llega a 0, el payload se destruye.

#### 3. `RefCell<T>`
`RefCell<T>` almacena junto con el dato un **contador dinámico de préstamos** (`borrow` flag) de tipo `Cell<isize>` (usualmente 4 u 8 bytes):
*   `0`: El valor no está prestado.
*   `> 0`: El valor está prestado de forma inmutable por esa cantidad de referencias activas.
*   `< 0` (habitualmente `-1`): El valor está prestado de forma mutable por una referencia activa.

Cuando llamas a `.borrow_mut()`, el runtime comprueba el flag en CPU. Si es distinto de `0`, el programa entra en pánico de inmediato en lugar de permitir comportamientos indefinidos de memoria.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Selección de Puntero Inteligente

| Puntero | Ubicación del Dato | Hilos de Ejecución | Permite Mutabilidad | Contador de Referencias |
| :--- | :--- | :--- | :--- | :--- |
| `Box<T>` | Heap | Multi-hilo / Único | Sí (con `&mut`) | No (Propietario Único) |
| `Rc<T>` | Heap | **Solo Un Hilo** | No (Inmutable) | Sí (Fuerte/Débil) |
| `RefCell<T>` | Stack/Heap | **Solo Un Hilo** | Sí (Mutabilidad Interna) | No (Comprobación en Runtime) |
| `Arc<T>` | Heap | **Seguro Multi-hilo** | No (Inmutable) | Sí (Atómico) |

---

### Errores Comunes y Pánicos

#### 1. Pánico por préstamos concurrentes conflictivos en `RefCell`
❌ **Código Erróneo:**
```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let r1 = x.borrow();     // Préstamo inmutable exitoso
    let mut r2 = x.borrow_mut(); // ERROR: Pánico en tiempo de ejecución
    *r2 += 10;
}
```
*   **Mensaje de Pánico:** `thread 'main' panicked at 'already borrowed: BorrowMutError'`
*   ✔️ **Solución:** Asegurar que los ámbitos de préstamos no se solapen. Puedes envolver los préstamos mutables en bloques locales `{}` para forzar su destrucción antes de realizar nuevos préstamos.

#### 2. Pérdida de rendimiento por clonación profunda innecesaria
*   **Problema:** Confundir `Rc::clone(&ptr)` con `.clone()`.
*   ✔️ **Solución:** Invocar `Rc::clone` explícitamente pasando la referencia (`Rc::clone(&var)`) en lugar de llamar a `var.clone()`. Esto ayuda a visualizar que solo se está copiando un puntero e incrementando un entero de control en memoria, en lugar de realizar una copia costosa del payload del montón.
