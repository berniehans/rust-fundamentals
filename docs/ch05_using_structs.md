# Capítulo 05: Usando Estructuras (Structs) para Agrupar Datos Relacionados

Este documento ofrece una exploración de nivel arquitectónico sobre las estructuras (*structs*) en Rust, abarcando su semántica, anatomía sintáctica, comportamiento en memoria física y optimizaciones del compilador.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es una Estructura (Struct)?
Las **estructuras (structs)** introducen etiquetas de nombre para cada componente o **campo (field)** de datos relacionados. Esto proporciona semántica explícita y robustez, ya que el significado de los datos no depende del orden en el que se declaran.

Rust clasifica las estructuras en tres grandes grupos:
1.  **Estructuras Clásicas (Classic Structs):** Tienen campos nombrados explícitamente y tipos definidos.
2.  **Estructuras de Tupla (Tuple Structs):** Tienen un identificador de tipo, pero sus campos carecen de nombre individual y se referencian por su orden posicional (ej. `struct Coordenadas(f64, f64);`). Son muy valiosas para el **patrón *Newtype***.
3.  **Estructuras Unitarias (Unit-like Structs):** Carecen por completo de campos. Se comportan de manera similar a la tupla vacía `()`. Su propósito principal es actuar como **marcadores de tipo** o implementar comportamientos lógicos (*traits*) sin almacenar estado.

### Propiedad (Ownership) de los Campos de una Estructura
Por defecto, una estructura clásica en Rust es dueña (*owner*) de todos los datos que contienen sus campos. Cuando inicializamos una estructura con campos como `String`, la propiedad de dichos datos se transfiere a la estructura. En consecuencia, cuando la instancia de la estructura sale de su ámbito, la función `drop` libera de forma jerárquica la memoria de cada campo.

Si una estructura guarda referencias a datos externos (`&str`), se requiere el uso explícito de **Ciclos de Vida (*Lifetimes*)** para garantizar que los datos prestados sigan siendo válidos mientras exista la estructura.

---

## 2. Anatomía y Semántica de la Sintaxis

### Declaración e Instanciación de Estructuras
```rust
#[derive(Debug)] // Permite formatear con {:?} y {:#?}
struct Usuario {
    activo: bool,
    nombre: String,
    correo: String,
    inicios_sesion: u64,
}

struct ColorRGB(u8, u8, u8);

struct LectorJson;

fn main() {
    let correo = String::from("admin@empresa.com");

    // Sintaxis Abreviada de Inicialización de Campos (Field Init Shorthand)
    let mut usuario_1 = Usuario {
        activo: true,
        nombre: String::from("Elena"),
        correo, // Asignación directa
        inicios_sesion: 12,
    };

    // Modificación de campos (requiere que la variable completa sea 'mut')
    usuario_1.activo = false;

    // Sintaxis de Actualización de Estructuras (Struct Update Syntax)
    // El operador '..' indica que los campos no definidos explícitamente se copian o mueven de usuario_1.
    // IMPORTANTE: Dado que 'nombre' es un String (no implementa Copy), su propiedad se MOVIÓ a usuario_2.
    let usuario_2 = Usuario {
        correo: String::from("soporte@empresa.com"),
        ..usuario_1
    };
}
```

### Bloque `impl` y la Semántica de `self`
*   `self`: Traducido como `self: Self`. Toma la **propiedad** de la instancia, consumiendo la estructura.
*   `&self`: Traducido como `self: &Self`. Toma un **préstamo inmutable**, permitiendo lectura de campos.
*   `&mut self`: Traducido como `self: &mut Self`. Toma un **préstamo mutable**, permitiendo modificar campos.

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // Función asociada (Constructor)
    fn nuevo(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    // Método con préstamo inmutable
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria: Alineación (Alignment) y Relleno (Padding)
Una estructura clásica se almacena en memoria de forma física contigua.
*   **Comportamiento en C/C++:** Los campos se colocan en memoria en el orden exacto de declaración, insertando bytes de relleno (*padding*) si difieren en requisitos de alineación (desperdiciando espacio).
*   **Comportamiento en Rust (Optimización Automática):** Rust no garantiza el orden de los campos. `rustc` reordena automáticamente los campos en memoria de mayor a menor tamaño de alineación para minimizar el padding y optimizar las lecturas físicas de CPU.

```
Visualización del Layout físico de memoria (u8, u64, u16):

Rust layout (Reordenamiento optimizado de campos):
+-------------------+---------+----+-------------------+
| u64 (8 bytes)     | u16 (2) | u8 | Padding (5 bytes) |
+-------------------+---------+----+-------------------+
Total: 16 bytes en memoria.

C layout (Orden estricto de declaración):
+----+-------------------+-------------------+---------+-------------------+
| u8 | Padding (7 bytes) | u64 (8 bytes)     | u16 (2) | Padding (6 bytes) |
+----+-------------------+-------------------+---------+-------------------+
Total: 24 bytes en memoria.
```

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Resumen de Semántica de Métodos en `impl`

| Tipo de Receptor | Firma Traducida | Propósito y Caso de Uso | Estado de la Estructura Origen |
| :--- | :--- | :--- | :--- |
| `fn f(self)` | `fn f(self: Self)` | Método consumidor / Transformaciones | **Destruida / Movida** |
| `fn f(&self)` | `fn f(self: &Self)` | Método de lectura simple | **Válida e inalterada** |
| `fn f(&mut self)`| `fn f(self: &mut Self)`| Método de escritura/mutación interna | **Válida pero mutada** |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Usar una estructura origen parcialmente movida por Struct Update Syntax
Si utilizas la sintaxis `..u1` y la estructura `u1` contenía campos dinámicos que no implementan `Copy` (como `String`):
❌ **Código Erróneo:**
```rust,compile_fail
struct Usuario {
    nombre: String,
    activo: bool,
}

fn main() {
    let u1 = Usuario { nombre: String::from("Elena"), activo: true };
    let u2 = Usuario { activo: false, ..u1 };
    
    // Error: u1.nombre se movió a u2, por lo que u1 ya no es utilizable
    println!("{}", u1.nombre); 
}
```
*   **Mensaje de Error:** `error[E0382]: borrow of partially moved value: `u1``
*   ✔️ **Solución:** Clonar los campos que no implementan `Copy` si necesitas seguir usando la variable original:
    ```rust
    let u2 = Usuario {
        nombre: u1.nombre.clone(),
        activo: false,
    };
    println!("{}", u1.nombre); // Válido
    ```

#### 2. Invocar un método mutable en una instancia inmutable
Intentar llamar a una función que requiere `&mut self` cuando la variable se declaró como inmutable:
❌ **Código Erróneo:**
```rust,compile_fail
struct Contador {
    valor: u32,
}

impl Contador {
    fn incrementar(&mut self) { self.valor += 1; }
}

fn main() {
    let c = Contador { valor: 0 };
    c.incrementar(); // Error: c es inmutable
}
```
*   **Mensaje de Error:** `error[E0596]: cannot borrow `c` as mutable, as it is not declared as mutable`
*   ✔️ **Solución:** Declarar la variable original con el modificador mutable `mut`:
    ```rust
    let mut c = Contador { valor: 0 };
    c.incrementar(); // Válido
    ```

#### 3. Guardar referencias en structs sin anotar lifetimes
Intentar definir un campo de referencia física a memoria dentro de un struct sin decirle al compilador cuánto tiempo debe vivir:
❌ **Código Erróneo:**
```rust,compile_fail
struct Contenedor {
    // Error: falta el parámetro de tiempo de vida (lifetime)
    dato: &str, 
}
```
*   **Mensaje de Error:** `error[E0106]: missing lifetime specifier`
*   ✔️ **Solución:** Anotar el lifetime genérico para obligar al compilador a comprobar que el dato prestado dure más tiempo que el struct:
    ```rust
    struct Contenedor<'a> {
        dato: &'a str,
    }
    ```

#### 4. Reutilizar una estructura después de invocar un método que consume `self`
Intentar usar la variable tras invocar un método que toma posesión del valor por propiedad:
❌ **Código Erróneo:**
```rust,compile_fail
struct Post { texto: String }
impl Post {
    fn publicar(self) { println!("{}", self.texto); }
}

fn main() {
    let p = Post { texto: String::from("hola") };
    p.publicar(); // p se mueve y se destruye al final de publicar
    println!("{}", p.texto); // Error: p ya no es válida
}
```
*   **Mensaje de Error:** `error[E0382]: borrow of moved value: `p``
*   ✔️ **Solución:** Modificar la firma del método para tomar `&self` en lugar de `self` si deseas que la propiedad permanezca en la función original:
    ```rust
    impl Post {
        fn publicar(&self) { println!("{}", self.texto); }
    }
    ```
