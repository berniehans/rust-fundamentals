# Capítulo 06: Enums y Pattern Matching (Coincidencia de Patrones)

Este documento ofrece un desglose arquitectónico e ingenieril del sistema de tipos algebraicos y coincidencia de patrones en Rust, analizando los Enums, el tipo `Option<T>`, el flujo de control exhaustivo y el comportamiento físico en memoria de estas abstracciones.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Tipos Algebraicos de Datos (ADTs) y Enums
En la programación convencional (como C, C++ o Java), las enumeraciones son simplemente listas de constantes numéricas (enteros asociados a nombres legibles). En Rust, los **Enums (Enumeraciones)** adoptan una filosofía de la teoría de lenguajes y la programación funcional conocida como **Tipos Algebraicos de Datos (Algebraic Data Types - ADTs)**. Específicamente, se comportan como **Tipos Suma (Sum Types)**.

Mientras que una estructura (*struct*) es un tipo producto (representa el Campo A **Y** el Campo B), un enum representa una suma lógica: un valor solo puede pertenecer a la Variante A **O** a la Variante B **O** a la Variante C en un instante de tiempo dado. Además, en Rust cada variante de un enum puede contener datos asociados de cualquier tipo y forma (estructuras anónimas, tuplas o tipos primitivos), haciendo que los enums sean herramientas extremadamente flexibles para modelar máquinas de estado y jerarquías lógicas complejas.

### El Problema del Valor Nulo y el Enfoque Seguro de Rust
En 2009, Sir Tony Hoare se disculpó públicamente por haber inventado la referencia nula (`null`) en 1965, calificándolo como su **"error de los mil millones de dólares"** debido a la ingente cantidad de vulnerabilidades y caídas de sistemas producidas por dereferencias de punteros nulos en producción. El problema fundamental radica en que, en lenguajes con referencias nulas, cualquier variable de tipo objeto puede tener un valor válido o ser `null`, obligando al programador a recordar añadir comprobaciones manuales propensas al olvido.

Rust elimina el concepto de `null` de su sistema de tipos por diseño. Para expresar la posibilidad de que un valor esté ausente o no inicializado, Rust utiliza el enum genérico **`Option<T>`**:
*   `Some(T)`: Representa la presencia de un valor de tipo `T`.
*   `None`: Representa la ausencia de valor.

Dado que `Option<T>` y `T` son tipos completamente diferentes ante el compilador, es imposible utilizar un valor potencialmente ausente directamente sin antes desempaquetarlo de manera obligatoria y explícita. El compilador valida estáticamente que el desarrollador gestione el caso `None`, erradicando por completo el error de desreferenciación nula en tiempo de ejecución.

### Exhaustividad en la Coincidencia de Patrones (Pattern Matching)
Para extraer y procesar los datos encapsulados dentro de los enums, Rust provee el operador de control de flujo **`match`**. A diferencia de las sentencias `switch` de otros lenguajes, `match` en Rust impone una regla estricta de **exhaustividad**: el desarrollador debe cubrir todas las posibles variantes y casos lógicos del valor evaluado. Si se omite una sola variante o rama de evaluación, el compilador detendrá el proceso de construcción del binario.

### `if let` como Azúcar Sintáctico
Aunque `match` proporciona exhaustividad y seguridad matemática, puede resultar verboso cuando solo nos interesa reaccionar a una variante específica e ignorar todas las demás. Rust resuelve esto mediante la sintaxis **`if let`**, la cual permite combinar coincidencia de patrones y asignación en un único bloque simplificado, actuando como un atajo visual y sintáctico a costa de omitir la verificación de exhaustividad global.

---

## 2. Anatomía y Semántica de la Sintaxis

### Declaración e Instanciación de Enums con Carga Útil
En Rust, un mismo enum puede contener variantes con diferentes estructuras sintácticas:

```rust
// Definición de un enum con variantes heterogéneas
enum Mensaje {
    Salir,                                    // Variante unitaria (sin datos)
    Mover { x: i32, y: i32 },                 // Variante estructurada con campos nombrados
    Escribir(String),                         // Variante de tupla con un único String
    CambiarColor(u8, u8, u8),                 // Variante de tupla con múltiples enteros
}

// Implementación de comportamiento sobre enums
impl Mensaje {
    fn procesar(&self) {
        // El cuerpo del método puede evaluar el estado del enum
        println!("Mensaje recibido");
    }
}

fn main() {
    // Instanciación de variantes
    let salir = Mensaje::Salir;
    let mover = Mensaje::Mover { x: 10, y: 20 };
    let escribir = Mensaje::Escribir(String::from("Hola"));
    
    // Invocación del método asociado al enum
    escribir.procesar();
}
```

### El Tipo `Option<T>` y su Integración
El tipo `Option<T>` se encuentra pre-importado en el preludio de Rust, por lo que se pueden instanciar sus variantes de forma directa sin el prefijo `Option::`:

```rust
fn main() {
    let algun_numero: Option<i32> = Some(5);
    let algun_texto: Option<&str> = Some("cadena");
    
    // Para valores ausentes se debe especificar el tipo genérico de forma explícita si no se infiere
    let numero_ausente: Option<i32> = None;
}
```

### La Expresión `match` y la Captura de Variables
El flujo `match` compara una expresión con patrones secuenciales y ejecuta el bloque de código correspondiente al primer patrón coincidente.

```rust
enum EstadoMoneda {
    Cara,
    Cruz,
}

enum Moneda {
    Centavo,
    Peso(EstadoMoneda), // Variante que encapsula otro enum
}

fn evaluar_moneda(moneda: Moneda) -> u8 {
    match moneda {
        Moneda::Centavo => {
            println!("Moneda de un centavo.");
            1
        }
        // Desestructuración y enlace de variables (binding)
        Moneda::Peso(estado) => {
            match estado {
                EstadoMoneda::Cara => println!("¡Cara!"),
                EstadoMoneda::Cruz => println!("¡Cruz!"),
            }
            100
        }
    }
}
```

#### Patrones Comodín y de Captura Total
Para lidiar con tipos de datos de rangos amplios (como enteros) o variantes que no deseamos manejar explícitamente, Rust implementa patrones comodín:

```rust
let valor_dado = 9;
match valor_dado {
    3 => println!("Ganaste tres puntos"),
    7 => println!("Perdiste siete puntos"),
    // Patrón comodín '_': coincide con cualquier valor pero no lo enlaza a ninguna variable
    _ => println!("No pasa nada"), 
}

match valor_dado {
    3 => println!("Ganaste tres puntos"),
    // Patrón de captura total: coincide con cualquier valor y lo asigna a la variable 'otro'
    otro => println!("Valor no reconocido: {otro}"),
}
```

### Sintaxis `if let`
```rust
let configuracion_maxima = Some(3u8);

// Enfoque clásico usando match (verboso para un solo caso)
match configuracion_maxima {
    Some(max) => println!("El máximo configurado es {max}"),
    _ => (), // Exigido por exhaustividad
}

// Enfoque idiomático usando if let
if let Some(max) = configuracion_maxima {
    println!("El máximo configurado es {max}");
} else {
    println!("No hay configuración máxima definida");
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria: Uniones Etiquetadas (Tagged Unions)
Para representar de forma física un tipo suma en el que coexisten variantes de diferentes tamaños, Rust implementa los enums como **Uniones Etiquetadas (Tagged Unions / Discriminated Unions)**. 

En memoria, un enum se organiza físicamente en dos secciones contiguas:
1.  **El Discriminante (Tag):** Un valor entero asignado por el compilador (usualmente representado con un byte `u8`) para identificar de forma única qué variante está activa actualmente en la instancia.
2.  **El Payload (Carga Útil):** El bloque de memoria donde se almacenan los datos de la variante activa. El tamaño de este bloque equivale exactamente al tamaño de la variante que requiera la mayor cantidad de bytes de todas las declaradas en el enum.

```
Layout de memoria de un Enum estándar en el Stack:
+-------------------+---------------------------------------------+
| Discriminante     | Payload (Tamaño de la variante más grande)  |
| (Tag - 1 Byte)    | (Relleno con Padding si la variante es menor)|
+-------------------+---------------------------------------------+
```

#### Ejemplo de Cálculo de Tamaño
Consideremos el siguiente enum:
```rust
enum Demo {
    A(u8),
    B(u64),
}
```
*   La variante `A` requiere 1 byte.
*   La variante `B` requiere 8 bytes y una alineación de 8 bytes.
*   El discriminante requiere 1 byte.
*   Para cumplir con las reglas de alineación del procesador, el tamaño total del enum `Demo` se alinea a múltiplos de 8 bytes. El discriminante de 1 byte se acompaña de 7 bytes de padding, seguidos del payload de 8 bytes. En total, la estructura ocupará **16 bytes** en memoria física.

### Optimización de Puntero Nulo (Null Pointer Optimization - NPO)
Una de las grandes ventajas competitivas de Rust en rendimiento es la **Optimización de Puntero Nulo (NPO)**. Para tipos cuyos valores válidos nunca pueden ser cero (como referencias `&T`, punteros inteligentes `Box<T>`, o tipos de enteros no nulos como `std::num::NonZeroU32`), el compilador reutiliza la representación de bits nula (`0x00000000` en punteros de 32 bits, `0x0000000000000000` en 64 bits) para representar el valor `None` de `Option<T>`.

Gracias a esto:
*   El tamaño de `Option<&T>` en memoria es exactamente **idéntico** al tamaño de un puntero simple `&T` (8 bytes en 64 bits).
*   No se reserva ningún byte físico adicional para almacenar un discriminante. La etiqueta se reduce a cero bits lógicos gracias a la reutilización semántica del cero a nivel binario. Esto representa una verdadera abstracción de costo cero en memoria.

```
Option<&T> en memoria RAM (Null Pointer Optimization):
Si los bits son distintos de cero: Se interpretan directamente como Some(&T).
Si todos los bits son cero (0x0): Se interpreta semánticamente como None.
```

### Seguridad en Memoria sin Garbage Collector
En lenguajes de programación como C, el uso de uniones clásicas (`union`) no está verificado de forma segura. Un desarrollador puede escribir datos interpretándolos como un tipo flotante y posteriormente leer la misma dirección de memoria interpretándola como un puntero, lo cual da acceso a lecturas arbitrarias y corrupción del flujo del programa. 

Rust garantiza la **seguridad del sistema de tipos** obligando a que toda extracción del payload de un enum se valide a través de coincidencia de patrones. No hay forma sintáctica ni física de acceder al payload de la variante `A` de forma accidental si la variante activa es `B`, ya que el compilador inyecta bifurcaciones basadas en el discriminante para proteger los accesos.

### Costo de Ejecución de `match`
*   **Monotonía de Saltos:** Cuando compilamos un bloque `match`, el compilador no genera necesariamente una lista secuencial de comparaciones `if/else` lentas si el número de variantes es alto.
*   **Jump Tables (Tablas de Salto):** Para enums con múltiples ramas, `rustc` y LLVM generan una tabla de saltos en ensamblador. El discriminante actúa como un índice para acceder directamente a la dirección de memoria de la instrucción del bloque correspondiente. La resolución de un bloque `match` complejo ocurre en un tiempo de ejecución constante **$O(1)$**, siendo igual de rápido que un salto directo en memoria de bajo nivel.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

| Sintaxis / Patrón Exacto | Propósito y Caso de Uso | Error Típico del Compilador (y Solución) |
| :--- | :--- | :--- |
| `enum E { A, B }`<br>`let x = E::A;` | Declaración básica e instanciación de una variante de enum. | Intentar usar la variante del enum sin calificar el nombre del espacio de nombres:<br>❌ `let x = A;`<br>`error[E0425]: cannot find value 'A' in this scope`<br>✔️ **Solución:** Calificar la variante anteponiendo el nombre del enum (`E::A`) o importar la variante mediante `use E::A;`. |
| `match x {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`E::A => 1,`<br>&nbsp;&nbsp;&nbsp;&nbsp;`_ => 2,`<br>`}` | Coincidencia de patrones básica utilizando un comodín `_` para ignorar variantes restantes. | Omitir la exhaustividad del match al evaluar variantes:<br>❌ `match x { E::A => 1 }`<br>`error[E0004]: non-exhaustive patterns: 'E::B' not covered`<br>✔️ **Solución:** Agregar brazos para cubrir todas las variantes restantes o introducir un comodín de captura general (`_` o `otro`). |
| `let val = opt.unwrap();` | Extraer el valor interno de un `Option<T>` asumiendo que es `Some`. | Llamar a `.unwrap()` sobre un valor que resulta ser `None` en tiempo de ejecución:<br>❌ `.unwrap()` en un valor `None`.<br>`panic: thread 'main' panicked at 'called Option::unwrap() on a None value'`<br>✔️ **Solución:** Usar `match`, `if let`, o métodos más seguros como `.unwrap_or(defecto)` o `.expect("Mensaje de error descriptivo")`. |
| `if let Some(x) = opt {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`println!("{x}");`<br>`}` | Azúcar sintáctico para realizar una acción si un `Option` contiene un valor, ignorando `None`. | Confundir la sintaxis intentando usar el operador de asignación tradicional o el de comparación (`==`):<br>❌ `if opt == Some(x) { ... }`<br>`error[E0425]: cannot find value 'x' in this scope`<br>✔️ **Solución:** Respetar la sintaxis estructurada `if let Patrón = Expresión`. |
| `match opt {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`Some(ref mut x) => ...`<br>`}` | Préstamo mutable del valor encapsulado dentro de un enum. | Intentar modificar el valor interno `x` de una variante usando un enlace inmutable de patrón:<br>❌ `Some(x) => *x = 10;`<br>`error[E0594]: cannot assign to immutable borrowed content`<br>✔️ **Solución:** Declarar explícitamente el préstamo como mutable en el brazo del patrón: `Some(ref mut x) => *x = 10` (o declarar el enum original mutable). |
