# Capítulo 05: Usando Estructuras (Structs) para Agrupar Datos Relacionados

Este documento ofrece una exploración exhaustiva y de nivel arquitectónico sobre las estructuras (*structs*) en Rust, abarcando su semántica, anatomía sintáctica, comportamiento en memoria física y optimizaciones del compilador.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es una Estructura (Struct)?
En el desarrollo de software, modelar el dominio del negocio requiere agrupar datos heterogéneos bajo una misma entidad de forma coherente. Mientras que las tuplas permiten agrupar valores sin nombre (dependiendo de índices posicionales implícitos como `tup.0`), las **estructuras (structs)** introducen etiquetas de nombre para cada componente o **campo (field)**. Esto proporciona semántica explícita y robustez, ya que el significado de los datos no depende del orden en el que se declaran.

Rust clasifica las estructuras en tres grandes grupos arquitectónicos:
1.  **Estructuras Clásicas (Classic Structs):** Tienen campos nombrados explícitamente y tipos definidos. Son la opción predeterminada para modelar objetos complejos del dominio que poseen múltiples atributos (ej. un registro de base de datos o una configuración del sistema).
2.  **Estructuras de Tupla (Tuple Structs):** Tienen un identificador de tipo, pero sus campos carecen de nombre individual y se referencian por su orden (ej. `struct Coordenadas(f64, f64);`). Son especialmente valiosas para el **patrón *Newtype***. Este patrón envuelve un tipo de dato primitivo en una estructura para imponer seguridad de tipos en tiempo de compilación (ej. impedir que se asigne accidentalmente un valor de tipo `Centimetros` a una función que espera `Pulgadas`).
3.  **Estructuras Unitarias (Unit-like Structs):** Carecen por completo de campos. Se comportan de manera similar a la tupla vacía `()`. Su propósito principal es actuar como **marcadores de tipo** o como entidades para implementar un comportamiento común (*traits*) en estructuras que no requieren almacenar datos o estados propios (comportamiento puro).

### Propiedad (Ownership) de los Campos de una Estructura
Por defecto, una estructura clásica en Rust es dueña (*owner*) de todos los datos que contienen sus campos. Cuando inicializamos una estructura con campos como `String`, la propiedad de dichos datos se transfiere a la estructura. En consecuencia, cuando la instancia de la estructura sale de su ámbito, la función `drop` libera de forma jerárquica la memoria de cada campo contenedor.

Si deseamos que una estructura guarde una referencia a un dato externo que pertenece a otra variable (ej. `&str` o `&u32`), Rust exige el uso explícito de **Ciclos de Vida (*Lifetimes*)**. El compilador necesita esta anotación genérica para validar estáticamente que el dueño original de los datos no libere la memoria antes de que la estructura termine su ciclo de vida, previniendo de raíz el error de puntero colgante (*dangling pointer*).

### Métodos y Funciones Asociadas
Rust separa la definición de los datos de su comportamiento. Los datos se definen dentro del bloque `struct`, mientras que las funciones asociadas y métodos se declaran dentro de un bloque `impl` (implementación).
*   **Métodos:** Son funciones que operan en el contexto de una instancia específica de la estructura. Tienen como primer parámetro la palabra clave `self` (o sus variantes con préstamos `&self` o `&mut self`), la cual enlaza dinámicamente la llamada al valor actual.
*   **Funciones Asociadas (o Métodos Estáticos):** Se definen en el bloque `impl` pero no toman ningún parámetro `self`. No actúan sobre una instancia específica; se invocan utilizando el operador de ruta `::` (ej. `Rectangulo::nuevo(5, 10)`). Su principal uso es servir como constructores alternativos o de fábrica.
*   **Múltiples bloques `impl`:** Rust permite declarar múltiples bloques `impl` para una misma estructura. Esto facilita la organización del código, la separación de implementaciones de *traits* específicos y la compilación condicional de características.

---

## 2. Anatomía y Semántica de la Sintaxis

### Declaración e Instanciación de Estructuras
```rust
// 1. Estructura Clásica con derivación de traits para depuración
#[derive(Debug)] // Permite formatear con {:?} y {:#?}
struct Usuario {
    activo: bool,
    nombre: String,
    correo: String,
    inicios_sesion: u64,
}

// 2. Estructura de Tupla (Tuple Struct)
struct ColorRGB(u8, u8, u8);

// 3. Estructura Unitaria (Unit-like Struct)
struct LectorJson;

fn main() {
    let correo = String::from("admin@empresa.com");

    // Sintaxis Abreviada de Inicialización de Campos (Field Init Shorthand)
    // Evita repetir 'correo: correo' porque la variable local coincide con el campo.
    let mut usuario_1 = Usuario {
        activo: true,
        nombre: String::from("Elena"),
        correo, 
        inicios_sesion: 12,
    };

    // Modificación de campos (requiere que la variable completa sea 'mut')
    usuario_1.activo = false;

    // Sintaxis de Actualización de Estructuras (Struct Update Syntax)
    // El operador '..' indica que los campos no definidos explícitamente se copian o mueven de usuario_1.
    // IMPORTANTE: Dado que 'nombre' es un String (no implementa Copy), su propiedad se MOVIÓ a usuario_2.
    // usuario_1 ya no puede ser usado de manera completa tras esta operación.
    let usuario_2 = Usuario {
        correo: String::from("soporte@empresa.com"),
        ..usuario_1
    };

    // Acceso a Tuple Structs por índices
    let negro = ColorRGB(0, 0, 0);
    let intensidad_rojo = negro.0;

    // Instanciación de Unit-like Struct
    let lector = LectorJson;
}
```

### Firmas del Bloque `impl` y la Semántica de `self`
El parámetro `self` en una firma de método es una abreviatura que el compilador traduce a un tipo explícito:

*   `self`: Traducido como `self: Self`. Toma la **propiedad** de la instancia. Consume la estructura, lo que significa que el llamador original ya no puede acceder a ella tras la llamada (útil para transformaciones de tipos o destrucciones seguras).
*   `&self`: Traducido como `self: &Self`. Toma un **préstamo inmutable**. Es la forma más común y permite leer los campos de la instancia sin modificar ni consumir la memoria.
*   `&mut self`: Traducido como `self: &mut Self`. Toma un **préstamo mutable**. Permite modificar los campos internos de la instancia sin destruirla.

```rust
struct Contenedor {
    capacidad: u32,
}

impl Contenedor {
    // Función asociada (Constructor)
    fn nuevo(capacidad: u32) -> Self {
        Self { capacidad } // 'Self' hace referencia al tipo actual (Contenedor)
    }
}

// Segundo bloque 'impl' para demostrar la separación de comportamiento
impl Contenedor {
    // Método con préstamo inmutable
    fn consultar_capacidad(&self) -> u32 {
        self.capacidad
    }

    // Método con préstamo mutable
    fn duplicar_capacidad(&mut self) {
        self.capacidad *= 2;
    }
}
```

### Macros y Atributos de Visualización: `#[derive(Debug)]` y `dbg!`
*   **`#[derive(Debug)]`:** Es un atributo de derivación (*derive macro*) que instruye al compilador a generar automáticamente una implementación del trait `std::fmt::Debug` para la estructura. Permite imprimir la estructura usando `println!("{:?}", instancia);` (formato compacto) o `println!("{:#?}", instancia);` (formato con sangrías estructurado).
*   **La Macro `dbg!`:** A diferencia de `println!`, la macro `dbg!` toma la propiedad de una expresión, imprime el nombre del archivo, la línea de código donde se invoca, la representación en consola de la expresión y luego **devuelve la propiedad** del valor evaluado. Esto permite interceptar llamadas a funciones o variables en flujo sin romper el ownership (ej. `let y = dbg!(x.area());`).

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria: Alineación (Alignment) y Relleno (Padding)
Una estructura clásica se almacena en memoria de forma física contigua.
*   Si se declara localmente, sus descriptores de pila se guardan en el **Stack**.
*   Si se encuentra dentro de un tipo dinámico (ej. `Box<Usuario>`), se almacena en el **Heap**.

A nivel de procesador, las CPUs leen y escriben datos en la memoria RAM en base a "palabras" alineadas (ej. 4 bytes en 32 bits, 8 bytes en 64 bits). Si una variable de 64 bits (`u64`) no está colocada en una dirección de memoria que sea múltiplo de 8, la CPU requerirá dos lecturas de memoria en lugar de una, degradando gravemente el rendimiento.

Para evitar esto, los compiladores inyectan **padding** (bytes de relleno inactivos) entre variables.
*   **Comportamiento en C/C++:** Los campos se colocan en memoria en el orden exacto de declaración. Si declaras `struct { a: u8, b: u64, c: u16 }`, el compilador insertará 7 bytes de padding entre `a` y `b`, y 6 bytes de padding al final de `c` para alinear la estructura a 8 bytes. Tamaño total: 24 bytes (desperdicio de 13 bytes).
*   **Comportamiento en Rust (Optimización Automática):** Rust no garantiza el orden de los campos en memoria por defecto. El compilador `rustc` analiza los tipos y realiza una reorganización física óptima para minimizar o eliminar el padding. Reorganizará los campos de mayor a menor tamaño de alineación: `b` (8 bytes), `c` (2 bytes), `a` (1 byte). Tamaño total: 16 bytes (desperdicio de solo 5 bytes de padding al final para alinear la estructura completa).
*   **FFI (Foreign Function Interface):** Si es necesario comunicarse con código de C, podemos forzar al compilador de Rust a no reorganizar los campos utilizando el atributo de representación `#[repr(C)]`.

```
Visualización del Layout físico de memoria (u8, u64, u16):

Rust layout (Reordenamiento optimizado de campos para mitigar padding):
+-------------------+---------+----+-------------------+
| u64 (8 bytes)     | u16 (2) | u8 | Padding (5 bytes) |
+-------------------+---------+----+-------------------+
Total: 16 bytes en memoria.

C layout (Orden estricto de declaración con padding forzado):
+----+-------------------+-------------------+---------+-------------------+
| u8 | Padding (7 bytes) | u64 (8 bytes)     | u16 (2) | Padding (6 bytes) |
+----+-------------------+-------------------+---------+-------------------+
Total: 24 bytes en memoria.
```

### Seguridad en Memoria (Memory Safety)

#### Préstamos Parciales (Partial Borrowing)
El *Borrow Checker* de Rust es lo suficientemente inteligente como para analizar los campos de las estructuras de forma independiente en lugar de tratarlas como un único bloque monolítico indivisible. Esto permite que coexistan préstamos independientes sobre campos distintos de una misma estructura mutable.

```rust
struct Par {
    a: String,
    b: String,
}

let mut p = Par { a: String::from("A"), b: String::from("B") };
let r1 = &p.a;      // Préstamo inmutable del campo 'a'
let r2 = &mut p.b;  // Préstamo mutable del campo 'b' (VÁLIDO, no se solapan)
```

Sin embargo, si intentamos tomar una referencia de la estructura completa (ej. `let r3 = &p;`), el compilador denegará el acceso ya que existe un préstamo mutable activo sobre uno de sus campos (`b`), garantizando la prevención de data races a nivel de campos individuales.

#### Prevención de Doble Liberación (Drop Flags)
Cuando usamos la Sintaxis de Actualización (`..usuario_1`) y un campo del montón es movido a otra estructura, la estructura original `usuario_1` queda parcialmente desinicializada. Rust realiza un seguimiento en tiempo de compilación y coloca **Drop Flags** (banderas booleanas de estado inyectadas en la pila). Al salir de ámbito, la función de liberación evalúa las Drop Flags para destruir únicamente los campos que siguen siendo propiedad válida de la estructura original, previniendo de forma segura errores de doble liberación.

### Costo de Ejecución
*   **Monomorfización y Despacho Estático:** Las llamadas a métodos definidos en bloques `impl` se resuelven en tiempo de compilación mediante **Despacho Estático**. El compilador conoce el tipo exacto del objeto y genera una llamada directa a la dirección de memoria de la función de máquina correspondiente. No hay coste en tiempo de ejecución asociado a la resolución dinámica de tipos.
*   **Inlining:** LLVM analiza los métodos pequeños definidos en estructuras (ej. getters simples) y los expande directamente en el código del llamador, eliminando el sobrecosto del salto de pila de la llamada a la función (overhead de llamada a función = cero).

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

| Sintaxis / Patrón Exacto | Propósito y Caso de Uso | Error Típico del Compilador (y Solución) |
| :--- | :--- | :--- |
| `struct Newtype(u32);` | Patrón *Newtype*. Envolver tipos primitivos para forzar comprobación estricta de dominios en compilación. | Tratar de operar aritméticamente de forma directa con el tipo interno:<br>❌ `let x = Newtype(5) + 10;`<br>`error[E0369]: cannot add 'u32' to 'Newtype'`<br>✔️ **Solución:** Extraer el tipo interno usando `.0` (`x.0 + 10`) o implementar los traits de operaciones (`Add`) correspondientes. |
| `let u2 = Usuario {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`correo: s,`<br>&nbsp;&nbsp;&nbsp;&nbsp;`..u1`<br>`};` | Struct Update Syntax. Duplicar valores parciales de otra instancia en una nueva. | Intentar reutilizar la estructura origen completa cuando contenía campos sin trait `Copy` (ej. `String`):<br>❌ `println!("{}", u1.nombre);`<br>`error[E0382]: borrow of partially moved value: 'u1'`<br>✔️ **Solución:** Clonar los campos dinámicos explícitamente (`nombre: u1.nombre.clone()`) antes o durante el volcado de datos. |
| `impl Struct {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`fn f(&mut self) {`<br>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`self.val = 1;`<br>&nbsp;&nbsp;&nbsp;&nbsp;`}`<br>`}` | Método mutable que edita el estado de los datos internos de la instancia. | Invocar el método sobre una variable declarada de forma inmutable:<br>❌ `let s = Struct { val: 0 }; s.f();`<br>`error[E0596]: cannot borrow 's' as mutable, as it is not declared as mutable`<br>✔️ **Solución:** Declarar la variable original con el modificador mutable (`let mut s = Struct { val: 0 };`). |
| `struct Referencia {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`dato: &i32,`<br>`}` | Guardar referencias en lugar de propiedad de datos en los campos de una estructura. | Omitir la anotación explícita del ciclo de vida en la declaración:<br>❌ `struct Referencia { dato: &i32 }`<br>`error[E0106]: missing lifetime specifier`<br>✔️ **Solución:** Anotar el parámetro genérico de lifetime: `struct Referencia<'a> { dato: &'a i32 }`. |
| `impl Objeto {`<br>&nbsp;&nbsp;&nbsp;&nbsp;`fn consumir(self) {}`<br>`}` | Método que toma la propiedad del objeto (`self`) para consumirlo o transformarlo. | Intentar llamar a cualquier otro método o variable del objeto después de haber invocado el método consumidor:<br>❌ `obj.consumir(); obj.consultar();`<br>`error[E0382]: borrow of moved value: 'obj'`<br>✔️ **Solución:** Rediseñar la firma del método para tomar `&self` o `&mut self` si se desea mantener activo el valor en el flujo de ejecución. |
