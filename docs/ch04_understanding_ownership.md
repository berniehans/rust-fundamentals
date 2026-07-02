# Capítulo 04: Entendiendo el Modelo de Propiedad (Ownership)

El sistema de propiedad (*Ownership*) es la característica más distintiva de Rust y le permite garantizar seguridad de memoria sin necesidad de un recolector de basura (*Garbage Collector*) ni de una gestión de memoria manual propensa a errores.

---

## 1. Conceptos Fundamentales (Desde Cero)

### El Modelo de Ownership
En lenguajes como C o C++, el programador debe asignar y liberar explícitamente la memoria en el Heap (usando `malloc` y `free`), lo que a menudo causa errores graves como fugas de memoria o punteros colgantes. En lenguajes como Java o Go, un *Garbage Collector* busca de forma automática la memoria que ya no está en uso en tiempo de ejecución, lo que introduce pausas (*latencia*) y consumo de recursos adicionales.

Rust adopta un tercer enfoque: la gestión de memoria se realiza a través de un **sistema de propiedad** gobernado por tres reglas fundamentales que el compilador comprueba estáticamente:

1.  **Cada valor en Rust tiene un dueño** (una variable).
2.  **Solo puede haber un dueño a la vez** para cualquier recurso.
3.  **Cuando el dueño sale del ámbito (*scope*), el valor se destruye** y la memoria se libera automáticamente.

#### El Ámbito (*Scope*) y la Función `drop`
El ámbito es la región del código dentro de la cual un elemento es válido. Comienza cuando se declara y termina cuando el bloque (delimitado por llaves `{}`) finaliza. Cuando una variable que posee recursos en el Heap sale de su ámbito, Rust llama automáticamente a un método especial llamado `drop`. Este método actúa de manera similar a un destructor, liberando inmediatamente los recursos en el montón.

### Referencias y Préstamos (Borrowing)
Pasar la propiedad de una variable en cada llamada a función es ineficiente y engorroso. Para solucionar esto, Rust utiliza las **referencias**, permitiendo que el código acceda a un valor sin tomar posesión del mismo. A este acto de crear una referencia se le denomina **Préstamo (*Borrowing*)**.

#### Reglas de los Préstamos
Para prevenir la corrupción de memoria y condiciones de carrera, el compilador impone dos reglas de préstamo mutuamente excluyentes en cualquier momento dado dentro de un ámbito:
1.  Puedes tener **cualquier número de referencias inmutables** (`&T`) a un recurso.
2.  O puedes tener **exactamente una referencia mutable** (`&mut T`) a un recurso.
3.  **Las referencias deben ser siempre válidas:** Rust prohíbe las referencias colgantes (*dangling references*), asegurando que el recurso de memoria original viva más tiempo que la referencia que apunta a él.

### Concepto de Slices (Porciones)
Un slice es una referencia a una secuencia contigua de elementos dentro de una colección (como una cadena o un arreglo). Al ser una referencia, **no posee** los datos subyacentes. Permite acceder de manera segura e inmutable a una parte específica de una estructura de datos sin realizar copias en memoria.

---

## 2. Anatomía y Semántica de la Sintaxis

### Operadores de Ownership y Referencias
*   `&`: Operador de referencia inmutable. Permite leer el valor apuntado sin modificarlo ni tomar su propiedad.
*   `&mut`: Operador de referencia mutable. Permite leer y escribir en la memoria apuntada sin tomar su propiedad.
*   `*`: Operador de desreferenciación. Permite acceder de manera directa al valor al que apunta una referencia para leerlo o escribir en él.

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

// Atajos sintácticos de rangos
let todo_el_slice = &s[..];      // Equivalente a &s[0..s.len()]
let desde_inicio = &s[..4];      // Equivalente a &s[0..4]
let hasta_final = &s[5..];       // Equivalente a &s[5..s.len()]

// Slices de arreglos genéricos
let array = [1, 2, 3, 4, 5];
let slice_array: &[i32] = &array[1..3]; // Hace referencia a [2, 3]
```

### Validación Estática del Compilador (*Borrow Checker*)
El *Borrow Checker* realiza un análisis de los ciclos de vida (*lifetimes*) en tiempo de compilación para verificar la validez del acceso a la memoria:
1.  **Evitar el retorno de referencias locales:** Si una función intenta retornar una referencia a una variable declarada dentro de ella, el compilador detendrá la compilación porque la variable local se destruirá al finalizar la función, dejando la referencia apuntando a memoria inválida (dangling pointer).
2.  **Verificación de exclusión mutua de mutabilidad:** El *Borrow Checker* calcula los rangos de validez de cada referencia desde su definición hasta su última lectura. Si detecta que una referencia mutable coexiste con lecturas u otras escrituras del mismo valor, genera un error estático.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria (Stack vs. Heap)
Para comprender el movimiento y la copia de datos, es vital analizar el layout físico de la memoria:

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
Cuando copiamos una variable que gestiona memoria en el Heap (como `String`), Rust copia el descriptor en el Stack (el puntero `ptr`, la longitud `len` y la capacidad `capacidad`), pero **invalida** la variable original. No realiza una copia de los datos del Heap porque sería una operación costosa.

```rust
let s1 = String::from("hola");
let s2 = s1; // Ocurre un Move. s1 queda invalidada.
```

Si Rust no invalidara `s1`, cuando ambas variables salieran de ámbito, ambas intentarían liberar el mismo bloque de memoria del montón. Esto produciría un error de **Doble Liberación (*Double Free Error*)**, el cual puede corromper la memoria del sistema y provocar fallos de seguridad graves.

#### El Clonado (Clone)
Para duplicar tanto el descriptor en la pila como los datos reales ubicados en el Heap, se debe llamar explícitamente a `.clone()`. Esta operación es costosa y requiere tiempo de ejecución para realizar la nueva asignación de memoria.

#### El Trait `Copy`
Los tipos que se almacenan completamente en el Stack (como enteros, booleanos y arreglos de tamaño fijo que contienen elementos `Copy`) implementan el trait `Copy`. Al asignar una variable `Copy` a otra, se realiza una copia bit a bit directa en la pila. La variable original sigue siendo completamente válida tras la asignación.

### Seguridad en Memoria sin Garbage Collector
*   **Prevención de Carreras de Datos (Data Races):** Una carrera de datos ocurre cuando dos o más punteros acceden de forma concurrente a la misma celda de memoria, al menos uno de ellos realiza una operación de escritura, y no existe ningún mecanismo de sincronización. Las reglas de préstamo de Rust impiden esto por completo en tiempo de compilación: no puedes tener múltiples referencias mutables del mismo dato activas a la vez.
*   **El Layout de un Slice:** Un slice (por ejemplo, `&str` o `&[i32]`) es un **Fat Pointer** (puntero gordo) en el Stack. Ocupa exactamente **16 bytes** en una arquitectura de 64 bits y consta de dos elementos:
    1.  Un puntero a los datos reales (8 bytes).
    2.  La longitud del slice (8 bytes).

```
Fat Pointer de un Slice en el Stack:
+-------------------+--------------------+
| ptr (8 bytes)     | longitud (8 bytes) |
+-------------------+--------------------+
```

### Costo de Ejecución
*   **Abstracción de Costo Cero:** La liberación de memoria en Rust se calcula en tiempo de compilación. No hay un proceso en segundo plano monitoreando variables ni un recolector que pause la ejecución de tu aplicación. El compilador inyecta las llamadas a `free` (o al asignador de memoria del sistema) exactamente donde las variables salen de ámbito.
*   **Referencias:** En lenguaje ensamblador, una referencia es un simple puntero físico de dirección. No tiene ningún costo adicional de CPU en comparación con los punteros tradicionales de C.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

| Sintaxis / Patrón Exacto | Propósito y Caso de Uso | Error Típico del Compilador (y Solución) |
| :--- | :--- | :--- |
| `let s2 = s1;` | Transferencia de propiedad (Move). Se utiliza para mover la pertenencia de los recursos del Heap de una variable a otra. | Intentar usar la variable original tras el Move:<br>❌ `println!("{}", s1);`<br>`error[E0382]: borrow of moved value: 's1'`<br>✔️ **Solución:** Utilizar `s1.clone()` si necesitas duplicar los datos en el Heap, o pasar una referencia `&s1` en lugar del valor completo. |
| `let ref_mut = &mut x;` | Préstamo mutable. Permite cambiar el valor del recurso sin poseer la propiedad. | Crear más de una referencia mutable en el mismo ámbito:<br>❌ `let r1 = &mut x; let r2 = &mut x;`<br>`error[E0499]: cannot borrow 'x' as mutable more than once at a time`<br>✔️ **Solución:** Limitar el tiempo de vida de la primera referencia mutable o estructurar el código en bloques `{}` separados para que salgan de ámbito. |
| `let ref_imm = &x;`<br>`let ref_mut = &mut x;` | Coexistencia de referencias mutables e inmutables. | Intentar modificar un valor mutable mientras existen lecturas inmutables activas:<br>❌ `let r1 = &x;`<br>❌ `let r2 = &mut x;`<br>❌ `println!("{}", r1);`<br>`error[E0502]: cannot borrow 'x' as mutable because it is also borrowed as immutable`<br>✔️ **Solución:** Asegurar que el uso de la referencia inmutable finalice antes de declarar y usar la referencia mutable. |
| `fn f() -> &String {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`let s = String::new();`<br>&nbsp;&nbsp;&nbsp;&nbsp;`&s`<br>`}` | Retornar referencias desde funciones. | Retornar referencias a variables creadas dentro de la propia función:<br>❌ `fn f() -> &String { ... &s }`<br>`error[E0515]: cannot return reference to local variable 's'`<br>✔️ **Solución:** Retornar el valor por propiedad (ej. retornar `String`), permitiendo que el llamador sea el nuevo dueño del recurso. |
| `let slice = &s[0..5];` | Creación de un slice (porción) de una cadena (`&str`) o arreglo. | Intentar indexar el slice fuera de los límites del arreglo o en límites de bytes incorrectos en caracteres Unicode multicanal:<br>❌ Acceder a un índice inválido en tiempo de ejecución.<br>`panic: thread 'main' panicked at 'byte index 3 is not a char boundary'`<br>✔️ **Solución:** Asegurar que los rangos de slice respeten los límites lógicos de bytes y caracteres (por ejemplo, validando si es un carácter Unicode completo). |
