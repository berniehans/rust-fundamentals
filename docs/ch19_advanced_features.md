# Capítulo 19: Características Avanzadas de Rust

Este documento proporciona un análisis exhaustivo y de bajo nivel de las herramientas avanzadas y de bajo nivel de Rust: Unsafe Rust (código inseguro), traits avanzados (tipos asociados, supertraits), tipos avanzados (DSTs, ZSTs, tipo never), funciones y closures avanzados (punteros de función), y el sistema de macros declarativas y procedimentales.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es Unsafe Rust (Rust Inseguro)?
Todo el código de Rust analizado en capítulos anteriores está regido por garantías estrictas de seguridad de memoria validadas por el compilador. Sin embargo, los computadores reales funcionan de forma diferente; los sistemas operativos y el hardware interactúan mediante punteros crudos y manipulación directa de registros físicos de memoria.

**Unsafe Rust** es un "superpoder" oculto que permite al desarrollador desactivar temporalmente algunas de las comprobaciones más estrictas del compilador. Es una herramienta indispensable para:
*   Interactuar directamente con el hardware o escribir controladores de dispositivos (Kernels / Embedded).
*   Escribir código de bajísimo nivel altamente optimizado (como la propia biblioteca estándar de Rust, incluyendo `Vec` y `HashMap`).
*   Interactuar con librerías de otros lenguajes (FFI - *Foreign Function Interface* como C o C++).

#### La Regla de Oro de Unsafe:
Unsafe **no desactiva** el verificador de préstamos ni elimina el tipado estricto. Únicamente te otorga acceso a cinco operaciones específicas (los "cinco superpoderes de unsafe"). El programador asume el 100% de la responsabilidad matemática de evitar la corrupción de memoria.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Los Cinco Superpoderes de Unsafe

#### Desreferenciar Punteros Crudos (`Raw Pointers`)
Los punteros crudos se escriben como `*const T` (inmutable) y `*mut T` (mutable). A diferencia de las referencias comunes:
*   Ignoran por completo las reglas de préstamo de Rust (puedes tener múltiples punteros mutables apuntando al mismo dato).
*   Pueden ser nulos (`0x0`) o contener direcciones de memoria inválidas.
*   No implementan lifetimes automáticos.

```rust
fn main() {
    let mut num = 5;

    // Crear punteros crudos es SEGURO (no requiere bloque unsafe)
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // DESREFERENCIAR punteros crudos es INSEGURO (requiere bloque unsafe)
    unsafe {
        println!("r1 es: {}", *r1);
        *r2 = 10; // Modificación directa de la memoria
        println!("r2 modificado es: {}", *r2);
    }
}
```

#### Llamar a Funciones o Métodos Inseguros
Una función marcada como `unsafe fn` contiene operaciones que pueden provocar comportamiento indefinido si no se cumplen ciertos requisitos de llamada:

```rust
// Esta función asume que el puntero crudo pasado es válido
unsafe fn desreferenciar_inseguro(puntero: *const i32) -> i32 {
    *puntero
}

fn main() {
    let x = 42;
    unsafe {
        let valor = desreferenciar_inseguro(&x);
        println!("Valor: {valor}");
    }
}
```

*   **FFI (Foreign Function Interface):** Para llamar a funciones escritas en C:
```rust
// Declaramos la vinculación externa
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // Invocar funciones externas siempre requiere unsafe
        println!("Valor absoluto de -3 en C: {}", abs(-3));
    }
}
```

#### Modificar Variables Estáticas Mutables
En Rust, las variables globales estáticas mutables (`static mut`) son peligrosas en sistemas concurrentes. Leer o modificar una variable global mutable requiere `unsafe`:

```rust
static mut CONTADOR_GLOBAL: u32 = 0;

fn incrementar() {
    unsafe {
        CONTADOR_GLOBAL += 1;
    }
}
```

#### Implementar un Trait Inseguro
Un trait es inseguro (`unsafe trait`) si sus métodos exigen garantías lógicas que el compilador no puede verificar (ej: el trait `Send` o `Sync` implementado a mano sobre tipos que contienen punteros crudos):
```rust
unsafe trait Inseguro {}
unsafe impl Inseguro for i32 {}
```

#### Acceder a Campos de una `union`
Las uniones (`union`) se utilizan principalmente para interactuar con uniones de C. Dado que no almacenan un discriminante para saber qué variante está activa, acceder a sus campos es una operación insegura.

---

### 2. Traits Avanzados

#### Tipos Asociados (`Associated Types`) vs. Genéricos
Un Tipo Asociado actúa como un marcador de posición de tipo dentro de la definición de un trait:

```rust
pub trait Iterator {
    // Tipo Asociado
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
*   **Diferencia clave:** Si usáramos genéricos (`trait Iterator<T>`), un tipo concreto podría implementar `Iterator<i32>` e `Iterator<String>` simultáneamente, obligando al programador a anotar tipos en cada llamada. Con Tipos Asociados, un tipo solo puede implementar el trait **una sola vez** con un tipo de retorno específico fijado.

#### Sobrecarga de Operadores (Sintaxis Genérica por Defecto)
Rust permite sobrecargar operadores matemáticos implementando traits del módulo `std::ops`. Permite definir tipos genéricos por defecto:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Milimetros(u32);
struct Metros(u32);

// Add<Metros> indica que sumamos Metros a Milimetros.
// El tipo de salida (Output) se define en el Tipo Asociado.
impl Add<Metros> for Milimetros {
    type Output = Milimetros;

    fn add(self, otros: Metros) -> Milimetros {
        Milimetros(self.0 + (otros.0 * 1000))
    }
}
```

#### Desambiguación Total (Fully Qualified Syntax)
Si una estructura implementa múltiples traits que tienen métodos con el mismo nombre, debemos desambiguar la llamada:

```rust
trait Piloto { fn volar(&self); }
trait Mago { fn volar(&self); }
struct Humano;

impl Humano { fn volar(&self) { println!("Moviendo los brazos..."); } }
impl Piloto for Humano { fn volar(&self) { println!("Despegando avión..."); } }
impl Mago for Humano { fn volar(&self) { println!("Flotando en el aire..."); } }

fn main() {
    let h = Humano;
    // Llamada al método propio
    h.volar(); 

    // Llamadas desambiguadas para métodos con receptor (&self)
    Piloto::volar(&h);
    Mago::volar(&h);

    // Si el método no tiene receptor (método estático sin self), se usa Calificación Completa:
    // <Tipo as Trait>::metodo()
}
```

---

### 3. Macros en Rust
Las macros permiten escribir código que genera otro código en tiempo de compilación (metaprogramación). Existen dos tipos:

1.  **Macros Declarativas (`macro_rules!`):** Funcionan mediante coincidencia de patrones sintácticos (similares a un bloque `match`) reemplazando texto del AST (Abstract Syntax Tree).
2.  **Macros Procedimentales:** Funcionan como funciones que reciben código Rust como flujo de tokens, lo manipulan usando código Rust ordinario y devuelven un nuevo flujo de tokens al compilador. Existen tres variantes:
    *   **Custom Derive:** Añade código automáticamente (ej. `#[derive(Serialize)]`).
    *   **Attribute-like:** Atributos personalizados (ej. `#[route(GET, "/")]`).
    *   **Function-like:** Apariencia de llamada a función (ej. `sql!("SELECT * FROM usuarios")`).

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Comportamiento Indefinido (Undefined Behavior - UB)
Cuando ocurre Comportamiento Indefinido en un bloque `unsafe` (por ejemplo, desreferenciar un puntero nulo o violar el alias mutable de referencias):
*   A nivel físico, la CPU no explota de inmediato necesariamente; en su lugar, **los supuestos del optimizador de LLVM se rompen**.
*   LLVM asume que las reglas de Rust son verdaderas en un 100% para realizar optimizaciones matemáticas agresivas. Si ocurre UB, el compilador puede eliminar ramificaciones de control completas del ensamblador final por asumir que esa rama era imposible físicamente, produciendo fallos de seguridad silenciosos de extrema gravedad.

---

### Tipos de Tamaño Dinámico (Dynamically Sized Types - DST)
Rust necesita conocer el tamaño físico de los tipos en compilación. Los tipos como `str` o los slices `[T]` no tienen un tamaño conocido (no sabemos cuánto mide una cadena de texto en ejecución).
*   **Layout Físico:** Los DSTs no pueden almacenarse directamente en variables en el Stack. Deben residir siempre detrás de un **Fat Pointer** (puntero de datos de 8 bytes + metadato de tamaño de 8 bytes = 16 bytes).
*   **El Trait `Sized`:** Por defecto, todas las funciones genéricas añaden implícitamente la restricción `T: Sized`. Si deseas que una función acepte tanto tipos de tamaño fijo como DSTs, debes desactivar el límite con la sintaxis especial de interrogación `?Sized`:
```rust
// T puede o no tener un tamaño conocido en tiempo de compilación
fn procesar_dst<T: ?Sized>(valor: &T) { ... }
```

---

### Tipos de Tamaño Cero (Zero-Sized Types - ZST)
Los tipos que no contienen datos (como una estructura vacía `struct Vacia;` o el tipo unitario `()`) ocupan exactamente **0 bytes** en memoria.
*   **Optimización Extrema:** Rust optimiza esto a nivel físico. Si creas un `Vec<()>` e insertas un millón de elementos, el vector no reservará memoria en el montón. Simplemente incrementará su campo `length` en la pila. Al iterar sobre él, LLVM eliminará las lecturas físicas de memoria, traduciendo el bucle en un simple decremento del registro contador de la CPU, logrando una velocidad inalcanzable en otros lenguajes.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet Sintáctica de Características Avanzadas

| Sintaxis / Atributo | Propósito | Ámbito de Uso |
| :--- | :--- | :--- |
| `*const T` / `*mut T` | Puntero crudo inmutable/mutable. | Manipulación directa de memoria en `unsafe`. |
| `<Type as Trait>::fn` | Sintaxis de Calificación Completa. | Desambiguar métodos homónimos estáticos. |
| `fn(i32) -> i32` | Puntero de función (tipo de datos). | Pasar funciones limpias sin sobrecosto de closures. |
| `T: ?Sized` | Desactivar la restricción de tamaño. | Permitir que genéricos acepten slices (`[T]`) o `str`. |
| `#[proc_macro]` | Punto de entrada de Macro Procedimental.| Crates dedicados a metaprogramación. |

---

### Errores Comunes de Compilación y Ejecución

#### 1. Violación de acceso de memoria (Segmentation Fault) en Unsafe
❌ **Código Erróneo:**
```rust
fn main() {
    // Apuntamos un puntero crudo a una dirección de memoria aleatoria e inválida
    let puntero_basura = 0x12345678 as *const i32;
    unsafe {
        // Intento de lectura física de una dirección no asignada al proceso
        println!("{}", *puntero_basura); 
    }
}
```
*   **Resultado en Ejecución:** `error: process didn't exit successfully: ... (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)`
*   ✔️ **Solución:** Garantizar mediante aserciones o lógica matemática estricta que todo puntero desreferenciado apunte a una asignación de memoria activa y válida.

#### 2. Confundir el tipo de puntero de función `fn` con el trait de closure `Fn`
❌ **Código Erróneo:**
```rust
// fn (con f minúscula) es un puntero de función puro. No puede capturar entorno.
fn ejecutar(f: fn()) {
    f();
}

fn main() {
    let x = 10;
    // Error: El closure captura 'x' del entorno, por lo que no es un puntero de función puro
    ejecutar(|| println!("{x}")); 
}
```
*   **Mensaje de Error:** `error[E0308]: mismatched types: expected fn pointer, found closure`
*   ✔️ **Solución:** Si necesitas capturar datos del entorno, cambia la firma de la función receptora para que acepte un genérico restringido por los traits de closure (`F: Fn()`):
    ```rust
    fn ejecutar<F: Fn()>(f: F) {
        f();
    }
    ```
