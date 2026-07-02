# Capítulo 03: Conceptos Comunes de Programación en Rust

Este documento proporciona un análisis técnico riguroso de los fundamentos de programación en Rust, cubriendo variables, tipos de datos, funciones y control de flujo. Abordaremos el comportamiento de estas construcciones tanto a nivel sintáctico como en la gestión de memoria y el comportamiento del compilador (`rustc`).

---

## 1. Conceptos Fundamentales (Desde Cero)

### Mutabilidad, Inmutabilidad y Constantes
*   **Variables e Inmutabilidad por Defecto:** En Rust, las variables declaradas mediante la palabra clave `let` son inmutables por defecto. Si intentamos reasignar un valor a una variable inmutable, el compilador generará un error de compilación. Este principio de diseño fomenta la seguridad en entornos concurrentes, previene efectos secundarios accidentales y permite al optimizador de LLVM asumir invariantes fuertes sobre los datos.
*   **La Palabra Clave `mut`:** Cuando un dato necesita cambiar de estado a lo largo del tiempo, se debe declarar explícitamente como mutable anteponiendo `mut` al identificador (ej. `let mut x = 5;`). Esto documenta de forma clara la intención del programador y activa la posibilidad de reescribir la memoria asociada a dicha variable.
*   **Constantes (`const`):** A diferencia de las variables inmutables, las constantes son inmutables de forma permanente y no pueden ser marcadas con `mut`. Sus características distintivas son:
    1.  Se declaran con la palabra clave `const` y su tipo de dato **debe ser anotado explícitamente** (no se permite la inferencia de tipos).
    2.  Su valor debe ser una expresión constante evaluable en tiempo de compilación (no puede depender del resultado de llamadas a funciones en tiempo de ejecución o valores dinámicos del sistema).
    3.  Se pueden declarar en cualquier ámbito, incluido el ámbito global (fuera de `fn`).
*   **Shadowing (Enmascaramiento):** El shadowing permite declarar una nueva variable con el mismo nombre que una variable declarada anteriormente en el mismo ámbito o en un ámbito anidado. La nueva variable "enmascara" a la anterior. A diferencia de marcar una variable como `mut`, el shadowing:
    1.  Requiere el uso repetido de la palabra clave `let`.
    2.  Permite cambiar el tipo de dato del identificador preservando el nombre (evitando la necesidad de inventar nombres alternativos como `spaces_str` y `spaces_len`).
    3.  Garantiza que, tras las transformaciones deseadas, la variable siga siendo inmutable a menos que la última declaración use `mut`.

### Tipos de Datos
Rust es un lenguaje de **tipado estático** y **fuertemente tipado**. El compilador debe conocer el tipo de todas las variables en tiempo de compilación. A menudo, `rustc` puede inferir el tipo a partir del valor asignado, pero en casos de ambigüedad (como al usar el método `.parse()`), se debe proporcionar una anotación de tipo explícita.

#### Tipos Escalares
Representan un único valor dentro de una escala. Existen cuatro tipos principales:
1.  **Enteros (Integers):** Números sin componente fraccionaria. Se clasifican por su tamaño en bits y si admiten signo (`i`) o no (`u`):
    *   Tamaño fijo: `i8`/`u8` (8 bits), `i16`/`u16` (16 bits), `i32`/`u32` (32 bits), `i64`/`u64` (64 bits), `i128`/`u128` (128 bits).
    *   Tamaño dinámico: `isize`/`usize`. Su tamaño depende de la arquitectura del procesador del sistema objetivo (32 bits en arquitecturas de 32 bits, 64 bits en arquitecturas de 64 bits). Se utilizan principalmente para indexar colecciones y medir el tamaño en bytes de estructuras en memoria.
    *   *Literales:* Rust permite escribir números con guiones bajos para mejorar la legibilidad (ej. `1_000_000`) y prefijos según su base: hexadecimal (`0xff`), octal (`0o77`), binario (`0b1111_0000`) y bytes literales (`b'A'` únicamente para `u8`).
2.  **Punto Flotante (Floating-Point):** Números con decimales. Rust ofrece dos tipos según el estándar IEEE-754: `f32` (precisión simple) y `f64` (precisión doble). El tipo por defecto es `f64` porque las CPUs modernas procesan este tipo con una velocidad similar al `f32` pero con mayor precisión matemática.
3.  **Booleano (`bool`):** Representa un valor de verdad y solo puede tomar las variantes `true` o `false`. Ocupa exactamente 1 byte de memoria física.
4.  **Carácter (`char`):** Representa un valor escalar Unicode (Unicode Scalar Value). A diferencia de otros lenguajes donde un carácter es un byte, en Rust `char` tiene un tamaño de **4 bytes** y puede representar emojis, caracteres chinos, japoneses, cirílicos, acentos, etc. Se delimita con comillas simples (ej. `'z'`, `'ℤ'`, `'😻'`).

#### Tipos Compuestos
Agrupan múltiples valores en un solo tipo.
1.  **Tuplas:** Grupo ordenado de valores de **diferentes tipos** con una longitud fija. Una vez declarada, su tamaño no puede crecer ni disminuir.
    *   Declaración: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    *   Acceso: Se puede extraer información mediante la desestructuración de patrones (`let (x, y, z) = tup;`) o mediante acceso directo por índice utilizando el punto (ej. `tup.0`, `tup.1`).
    *   Tupla unitaria: La tupla vacía `()` representa un tipo y un valor único. Se le conoce como *unit type* y representa la ausencia de valor (similar a `void` en otros lenguajes).
2.  **Arreglos (Arrays):** Grupo ordenado de valores del **mismo tipo** con una longitud fija.
    *   Declaración: `let a: [i32; 5] = [1, 2, 3, 4, 5];`. El tipo se anota como `[tipo; longitud]`.
    *   Inicialización por repetición: `let a = [3; 5];` produce `[3, 3, 3, 3, 3]`.
    *   Acceso: Se accede a los elementos mediante subíndices encerrados en corchetes (ej. `let primer = a[0];`).

### Funciones
Las funciones se definen con la palabra clave `fn`. Rust utiliza la convención de estilo *snake_case* para nombrar funciones y variables.
*   **Parámetros:** Deben declarar obligatoriamente su tipo en la firma de la función (ej. `fn sumar(x: i32, y: i32)`). Esto permite al compilador realizar comprobaciones estáticas de tipos sin requerir inferencia global en todo el programa.
*   **Declaraciones (*Statements*) vs. Expresiones (*Expressions*):**
    *   Las **declaraciones** son instrucciones que realizan alguna acción pero no devuelven ningún valor. Terminan con un punto y coma `;`. Por ejemplo, `let y = 6;` es una declaración. Por lo tanto, no es válido escribir `let x = (let y = 6);`.
    *   Las **expresiones** evalúan y producen un valor resultante. La mayoría de las líneas en Rust son expresiones (operaciones matemáticas, llamadas a funciones, bloques `{}`). Una característica clave es que las expresiones **no terminan con punto y coma**. Si añades un punto y coma al final de una expresión, la conviertes en una declaración y su valor de retorno pasa a ser el tipo unitario `()`.
*   **Valores de Retorno:** Se declaran tras el símbolo `->` en la firma. No es necesario nombrar el valor de retorno, solo su tipo (ej. `fn cinco() -> i32`). Para retornar un valor:
    1.  Se puede usar la palabra clave `return` de forma explícita para salir temprano de la función.
    2.  O de forma idiomática, simplemente escribir una expresión como la última línea del bloque de la función (sin punto y coma).

### Control de Flujo

#### Expresión `if`
Permite bifurcar el código basándose en condiciones booleanas.
*   La condición asociada a un `if` **debe ser estrictamente de tipo `bool`**. Rust no realiza conversiones implícitas de tipos (no existe el concepto de valores "truthy" o "falsy" como en JavaScript o C).
*   Al ser `if` una expresión, puede usarse al lado derecho de un enlace `let`:
    ```rust
    let condicion = true;
    let numero = if condicion { 5 } else { 6 }; // Ambos brazos deben evaluar al mismo tipo (i32)
    ```

#### Bucles (`loop`, `while`, `for`)
1.  **`loop`:** Ejecuta un bucle infinito hasta que se encuentre un comando `break` de forma explícita. Al ser una expresión, `loop` puede retornar un valor al resto del programa a través de la instrucción `break`:
    ```rust
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 10 {
            break contador * 2; // Retorna 20
        }
    };
    ```
    *   *Etiquetas de bucle (Loop Labels):* Si hay bucles anidados, se pueden usar etiquetas (como `'nombre_bucle:`) para especificar a qué bucle afecta un `break` o `continue`.
2.  **`while`:** Bucle condicional clásico. Evalúa la condición antes de cada iteración.
3.  **`for`:** Es el bucle más seguro e idiomático en Rust para recorrer colecciones. Evita los desbordamientos de índices y optimiza el rendimiento del acceso a datos.
    ```rust
    let elementos = [10, 20, 30, 40, 50];
    for elemento in elementos {
        println!("El valor es: {elemento}");
    }
    ```
    *   Se puede utilizar con rangos provistos por la biblioteca estándar, como `(1..4)` (excluyente, genera 1, 2 y 3) o `(1..=4)` (incluyente, genera 1, 2, 3 y 4).

---

## 2. Anatomía y Semántica de la Sintaxis

### Firmas y Declaraciones de Conceptos Comunes
A continuación se detalla la sintaxis formal de los conceptos del capítulo 3:

```rust
// 1. Declaración de variables, mutabilidad y shadowing
let x = 10;                     // Inmutable, tipo inferido como i32 por defecto.
let mut y = 20;                 // Mutable.
y = 25;                         // Reasignación válida.
let x = "texto";                // Shadowing: cambia el tipo de x de i32 a &str.

// 2. Definición de Constantes (Ámbito global o local)
const SEGUNDOS_EN_UN_DIA: u32 = 60 * 60 * 24; // Expresión constante evaluable en compilación.

// 3. Tipos compuestos
let tupla: (i32, f64, char) = (500, 6.4, '🌎');
let (a, b, c) = tupla;          // Desestructuración por patrón.
let primer_elemento = tupla.0;  // Acceso por índice.

let array: [i32; 3] = [10, 20, 30]; // Array de enteros de tamaño 3.
let array_repetido = [0; 100];      // Crea un array con cien ceros.
let primer_valor = array[0];        // Acceso por índice en el array.

// 4. Firmas de Funciones y Bloques Expresión
fn procesar_datos(valor: i32, factor: f64) -> f64 {
    println!("Procesando valor: {valor}"); // Declaración (Statement)
    
    let operacion_interna = {
        let temporal = valor as f64 * 1.5;
        temporal * factor // Expresión: produce el valor asignado a operacion_interna
    }; // El bloque completo actúa como una expresión.
    
    operacion_interna // Expresión de retorno final (sin return ni punto y coma).
}
```

### Validación Estática del Compilador (`rustc`)
El compilador realiza varios análisis críticos sobre estas estructuras antes de emitir código binario:
1.  **Análisis de Inicialización Estricta:** `rustc` realiza un análisis del flujo de control para verificar que todas las variables estén inicializadas antes de cualquier posible lectura de memoria. Si existe un solo camino de ejecución en el que una variable pueda ser leída sin haber sido inicializada, se produce un error en tiempo de compilación.
2.  **Verificación de Mutabilidad Fuerte:** Si se intenta reasignar un valor a una variable declarada con `let` sin la palabra clave `mut`, el compilador genera un error de tipo `cannot assign twice to immutable variable`.
3.  **Exhaustividad y Coherencia de Tipos en Control de Flujo:** En la expresión `if`, `rustc` verifica estáticamente que los tipos evaluados en el bloque `if` y en el bloque `else` sean idénticos. Si difieren, genera un error indicando que las ramas tienen tipos incompatibles.
4.  **Evaluación Constante (`const eval`):** Para las constantes, el subsistema del compilador realiza la evaluación de la expresión matemática/lógica en tiempo de compilación. Si la expresión contiene una llamada a una función no calificada como `const fn`, el compilador detiene el proceso con un error de inicialización no constante.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria (Stack vs. Heap)
*   **Variables Locales y Tipos Escalares:** Todos los tipos escalares descritos (`i32`, `f64`, `bool`, `char`) tienen un tamaño conocido y estático en tiempo de compilación. Debido a esto, se asignan directamente en el **Stack** (Pila). Su tiempo de vida está acotado por la entrada y salida de su bloque contenedor (*stack frame*).
*   **Tuplas y Arrays:** 
    *   Un array `[T; N]` se almacena como un bloque de memoria física **contiguo** en la pila. No contiene metadatos de tamaño en tiempo de ejecución; su tamaño exacto en bytes en la pila es `size_of::<T>() * N`.
    *   Una tupla se almacena como un conjunto alineado de sus elementos individuales en la pila. La alineación física de los bytes de la tupla puede diferir del orden lógico de declaración de sus miembros para minimizar el desperdicio de memoria (padding) impuesto por los requerimientos del procesador.
*   **Constantes (`const`):** No se les asigna una dirección de memoria física única en el Stack o el Heap del programa en tiempo de ejecución. El compilador aplica un mecanismo de **inlining**, reemplazando cada aparición del nombre de la constante por su valor directamente en el código de máquina generado. Si la constante es una estructura compleja o un array de bytes grande, podría colocarse en el segmento de solo lectura (`.rodata`) del binario final.

### Seguridad en Memoria (Memory Safety)
*   **Prevención de Accesos Fuera de Rango (Bounds Checking):** Para garantizar que el programa nunca acceda a memoria arbitraria y vulnerable, Rust realiza una comprobación en tiempo de ejecución al acceder a un arreglo usando índices dinámicos (ej. `a[indice]` donde `indice` es una variable). Si el índice es mayor o igual que el tamaño del arreglo, Rust aborta de forma segura el programa generando un **panic**, impidiendo lecturas o escrituras inválidas.
*   **Prevención de Desbordamiento de Enteros (Integer Overflow):** 
    *   En modo **Debug** (compilación sin optimizaciones), `rustc` inyecta instrucciones para validar que las operaciones aritméticas no excedan el valor representable por el tipo entero. Si ocurre un desbordamiento, el programa entra en *panic*.
    *   En modo **Release** (compilación con `--release`), Rust deshabilita estas comprobaciones. Realiza un comportamiento de **desbordamiento envuelto en complemento a dos** (wrapping). Aunque esto da un valor lógico incorrecto, previene la corrupción de memoria física de bajo nivel.

### Costo de Ejecución
*   **Abstracciones de Costo Cero:** El enmascaramiento de variables (*shadowing*) es una abstracción de costo cero pura. No tiene representación en tiempo de ejecución; no incrementa el uso de la memoria ni añade instrucciones adicionales de CPU. Únicamente remapea los nombres de variables en el compilador.
*   **Costo del Bounds Checking:** El análisis de límites en arrays introduce una instrucción de salto condicional antes de realizar el acceso a la memoria. Sin embargo, mediante la fase de optimización de LLVM, si el índice se deriva de un bucle `for` o si su valor puede determinarse estáticamente como seguro, el compilador elimina las instrucciones de validación de límites, reduciendo el sobrecosto a cero.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

| Sintaxis / Patrón Exacto | Propósito y Caso de Uso | Error Típico del Compilador (y Solución) |
| :--- | :--- | :--- |
| `let x = 5;`<br>`let x = x + 1;` | Shadowing (Enmascaramiento). Reutilizar un identificador con posible cambio de tipo de dato o mutabilidad de forma inmutable. | Intentar modificar la variable original sin usar la palabra clave `let`:<br>❌ `x = x + 1;`<br>`error[E0384]: cannot assign twice to immutable variable`<br>✔️ **Solución:** Anteponer `let` para declarar una nueva variable o declarar la variable original como `mut`. |
| `const LIMITE: u32 = 1000;` | Definición de constantes legibles globales o locales evaluadas en tiempo de compilación. | Omitir la anotación explícita del tipo de dato:<br>❌ `const LIMITE = 1000;`<br>`error: missing type for const item`<br>✔️ **Solución:** Proveer siempre el tipo exacto (ej. `: u32`). |
| `let a: [i32; 5] = [0; 5];` | Creación de un arreglo (array) en el Stack con 5 elementos inicializados en 0. | Intentar modificar un elemento de un array inmutable:<br>❌ `a[0] = 1;`<br>`error[E0596]: cannot mutate immutable variable`<br>✔️ **Solución:** Declarar el arreglo con mutabilidad utilizando `let mut a = [0; 5];`. |
| `fn f() -> i32 {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`let x = 10;`<br>&nbsp;&nbsp;&nbsp;&nbsp;`x + 5`<br>`}` | Retorno de un valor implícito desde una función utilizando una expresión al final del bloque. | Añadir un punto y coma al final de la expresión de retorno:<br>❌ `x + 5;`<br>`error[E0308]: mismatched types. Expected i32, found ()`<br>✔️ **Solución:** Eliminar el punto y coma `;` para conservar la naturaleza de expresión del bloque de código. |
| `let num = if c { 1 } else { 2 };` | Asignación condicional de una variable mediante una expresión `if`/`else`. | Retornar tipos diferentes en cada rama condicional:<br>❌ `if c { 1 } else { "dos" };`<br>`error[E0308]: mismatched types. expected integer, found &str`<br>✔️ **Solución:** Asegurar que todos los bloques de código condicionales evalúen exactamente al mismo tipo de dato. |
