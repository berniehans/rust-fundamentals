# Capítulo 03: Conceptos Comunes de Programación en Rust

Este documento proporciona un análisis técnico riguroso de los fundamentos de programación en Rust, cubriendo variables, tipos de datos, funciones y control de flujo. Abordaremos el comportamiento de estas construcciones tanto a nivel sintáctico como en la gestión de memoria y el comportamiento del compilador (`rustc`).

---

## 1. Conceptos Fundamentales (Desde Cero)

### Mutabilidad, Inmutabilidad y Constantes
*   **Variables e Inmutabilidad por Defecto:** En Rust, las variables declaradas mediante la palabra clave `let` son inmutables por defecto. Si intentamos reasignar un valor a una variable inmutable, el compilador generará un error de compilación. Este principio de diseño fomenta la seguridad en entornos concurrentes, previene efectos secundarios accidentales y permite al optimizador de LLVM asumir invariantes fuertes sobre los datos.
*   **La Palabra Clave `mut`:** Cuando un dato necesita cambiar de estado a lo largo del tiempo, se debe declarar explícitamente como mutable anteponiendo `mut` al identificador (ej. `let mut x = 5;`). Esto activa la posibilidad de escribir en la celda de memoria asignada a esa variable.
*   **Constantes (`const`):** A diferencia de las variables inmutables, las constantes son inmutables de forma permanente y no pueden ser marcadas con `mut`. Sus características distintivas son:
    1.  Se declaran con la palabra clave `const` y su tipo de dato **debe ser anotado explícitamente** (no se permite la inferencia de tipos).
    2.  Su valor debe ser una expresión constante evaluable en tiempo de compilación (no puede depender de ejecuciones dinámicas).
    3.  Se pueden declarar en cualquier ámbito, incluido el ámbito global.
*   **Shadowing (Enmascaramiento):** El shadowing permite declarar una nueva variable con el mismo nombre que una variable declarada anteriormente en el mismo ámbito. A diferencia de `mut`, el shadowing:
    1.  Requiere el uso repetido de la palabra clave `let`.
    2.  Permite cambiar el tipo de dato del identificador preservando el nombre original.
    3.  Garantiza que la variable siga siendo inmutable tras las transformaciones a menos que declaremos explícitamente `mut` en la última declaración.

### Tipos de Datos
Rust es un lenguaje de **tipado estático** y **fuertemente tipado**. El compilador debe conocer el tipo de todas las variables en tiempo de compilación.

#### Tipos Escalares
1.  **Enteros (Integers):** Números sin componente fraccionaria. Se clasifican por su tamaño en bits y si admiten signo (`i`) o no (`u`):
    *   Tamaño fijo: `i8`/`u8`, `i16`/`u16`, `i32`/`u32`, `i64`/`u64`, `i128`/`u128`.
    *   Tamaño dinámico: `isize`/`usize`. Su tamaño depende de la arquitectura del procesador (32 o 64 bits). Se utilizan principalmente para indexar colecciones y medir el tamaño en bytes de estructuras en memoria.
2.  **Punto Flotante (Floating-Point):** Números con decimales. Rust ofrece dos tipos según el estándar IEEE-754: `f32` y `f64`. El tipo por defecto es `f64`.
3.  **Booleano (`bool`):** Ocupa exactamente 1 byte de memoria física y solo puede tomar las variantes `true` o `false`.
4.  **Carácter (`char`):** Representa un valor escalar Unicode (Unicode Scalar Value). Tiene un tamaño de **4 bytes** y puede representar cualquier carácter Unicode (ej: emojis). Se delimita con comillas simples.

#### Tipos Compuestos
1.  **Tuplas:** Grupo ordenado de valores de **diferentes tipos** con una longitud fija.
    *   Declaración: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    *   Acceso: Mediante desestructuración (`let (x, y, z) = tup;`) o acceso por índice con el punto (ej. `tup.0`).
    *   Tupla unitaria: La tupla vacía `()` representa la ausencia de valor (similar a `void`).
2.  **Arreglos (Arrays):** Grupo ordenado de valores del **mismo tipo** con una longitud fija en el Stack.
    *   Declaración: `let a: [i32; 5] = [1, 2, 3, 4, 5];` o inicialización por repetición `let a = [3; 5];` (`[3, 3, 3, 3, 3]`).

---

## 2. Anatomía y Semántica de la Sintaxis

### Expresiones vs. Declaraciones
Rust hace una distinción fundamental en su sintaxis:
*   **Declaraciones (Statements):** Son instrucciones que realizan alguna acción pero no devuelven ningún valor. Terminan con un punto y coma `;`. Por ejemplo, `let y = 6;` es una declaración.
*   **Expresiones (Expressions):** Evalúan y producen un valor resultante. No terminan con punto y coma. Si añades un punto y coma al final de una expresión, la conviertes en una declaración y su valor de retorno pasa a ser el tipo unitario `()`.

```rust
fn main() {
    // Un bloque de llaves es una expresión
    let y = {
        let x = 3;
        x + 1 // Expresión: produce el valor 4
    };
}
```

### Control de Flujo
*   **Expresión `if`:** bifurca el código basándose en condiciones booleanas. Al ser una expresión, puede usarse al lado derecho de un `let`:
    ```rust
    let numero = if condicion { 5 } else { 6 }; // Ambos brazos deben retornar el mismo tipo
    ```
*   **Loop con Retorno de Valor:** `loop` ejecuta un bucle infinito. Puede devolver un valor al resto del programa mediante la instrucción `break`:
    ```rust
    let resultado = loop {
        if condicion { break 42; }
    };
    ```
*   **Etiquetas de bucle (Loop Labels):** Permiten especificar a qué bucle afecta un `break` o `continue` en caso de anidamientos complejos:
    ```rust
    'externo: loop {
        loop {
            // Rompe el bucle externo directamente
            break 'externo;
        }
    }
    ```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria (Stack vs. Heap)
*   **Stack:** Todos los tipos escalares y compuestos (`arrays`, `tuplas`) se asignan directamente en el Stack si son de tamaño estático conocido. Un array `[T; N]` ocupa físicamente `size_of::<T>() * N` bytes contiguos en la pila.
*   **Constantes (`const`):** No se les asigna una dirección física de memoria en ejecución. El compilador aplica **inlining**, reemplazando la constante por su valor directamente en el código de máquina generado.

### Desbordamiento de Enteros (Integer Overflow)
*   **Modo Debug:** `rustc` inyecta comprobaciones de desbordamiento en la CPU. Si una operación matemática excede el límite del tipo (ej: `255u8 + 1`), el programa causa un **panic** y aborta inmediatamente para prevenir datos inválidos.
*   **Modo Release:** Rust deshabilita estas validaciones por motivos de rendimiento. En su lugar, realiza un **desbordamiento envuelto en complemento a dos** (wrapping). Así, `255u8 + 1` se convierte en `0u8`. Aunque esto evita caídas y corrupción física de memoria, puede causar errores de lógica si no se controla adecuadamente.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen Sintáctico de Conceptos de Programación

| Sintaxis Exacta | Propósito | Comportamiento |
| :--- | :--- | :--- |
| `let mut x = 5;` | Declarar variable mutable | Permite reescribir la celda de memoria en el Stack. |
| `let x = "txt";` | Shadowing (Enmascarar) | Declara una nueva variable, pudiendo cambiar el tipo de dato. |
| `const N: u32 = 10;`| Declarar constante | Evaluada en compilación, requiere tipo obligatorio. |
| `'bucle: loop` | Etiqueta de bucle | Permite aplicar `break`/`continue` a bucles padres anidados. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Reasignar un valor a una variable inmutable
Si intentas modificar el valor de una variable que no fue marcada con `mut`:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let x = 5;
    x = 10; // Error: reasignación de variable inmutable
}
```
*   **Mensaje de Error:** `error[E0384]: cannot assign twice to immutable variable `x``
*   ✔️ **Solución:** Declarar la variable como mutable usando `mut` o usar shadowing si deseas transformarla estáticamente:
    ```rust
    let mut x = 5; // Solución A
    x = 10;

    let x = 5;
    let x = 10; // Solución B (Shadowing)
    ```

#### 2. Omitir la anotación de tipo de dato en constantes
Intentar declarar una constante confiando en la inferencia de tipos de `rustc`:
❌ **Código Erróneo:**
```rust,compile_fail
const MAX_CONEXIONES = 100; // Error: falta el tipo de dato
```
*   **Mensaje de Error:** `error: missing type for const item`
*   ✔️ **Solución:** Las constantes requieren siempre que su tipo sea explícito e inequívoco en compilación:
    ```rust
    const MAX_CONEXIONES: u32 = 100;
    ```

#### 3. Ramas con tipos de retorno incompatibles en expresiones condicionales
Intentar asignar una expresión `if/else` donde los bloques evalúan a tipos de datos diferentes:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let condicion = true;
    // Error: una rama devuelve entero y la otra devuelve cadena
    let resultado = if condicion { 5 } else { "cinco" }; 
}
```
*   **Mensaje de Error:** `error[E0308]: `if` and `else` have incompatible types`
*   ✔️ **Solución:** Asegurar que todos los caminos lógicos de la expresión `if/else` evalúen exactamente al mismo tipo de dato:
    ```rust
    let resultado = if condicion { 5 } else { 6 };
    ```
