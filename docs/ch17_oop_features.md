# Capítulo 17: Características de Programación Orientada a Objetos (OOP)

Este documento proporciona un análisis exhaustivo de cómo Rust adopta e implementa las características fundamentales de la Programación Orientada a Objetos (OOP). Se detallan la encapsulación de datos, el polimorfismo mediante Trait Objects (despacho dinámico), el análisis de la herencia frente a la composición, y las reglas estrictas de seguridad de objetos (*Object Safety*).

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Es Rust un Lenguaje Orientado a Objetos?
La respuesta corta es: **depende de la definición de OOP**. Rust es un lenguaje multiparadigma. Si analizamos los tres pilares clásicos de la programación orientada a objetos:

1.  **Encapsulación (Ocultamiento de datos):** **Sí.** Rust implementa esto mediante la palabra clave `pub` combinada con el sistema de módulos, permitiendo ocultar detalles internos de estructuras y funciones de forma estricta.
2.  **Polimorfismo:** **Sí.** Rust implementa polimorfismo paramétrico (genéricos con despacho estático) y polimorfismo de subtipos (Trait Objects con despacho dinámico).
3.  **Herencia:** **No de forma tradicional.** Rust no permite que una estructura física herede los campos o métodos de otra estructura física. En su lugar, Rust promueve la **composición sobre la herencia** y utiliza traits para compartir interfaces lógicas.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Polimorfismo con Trait Objects (`dyn Trait`)
Cuando diseñamos interfaces gráficas o sistemas de complementos (*plugins*), a menudo necesitamos almacenar colecciones de elementos heterogéneos en memoria que comparten un comportamiento común (ej. una lista de diferentes componentes que pueden dibujarse).
Para ello, utilizamos **Trait Objects** especificando `dyn Trait` detrás de un puntero inteligente:

```rust
// Definimos el Trait
pub trait Dibujable {
    fn dibujar(&self);
}

// Estructura que almacena una colección heterogénea de elementos que implementan el trait
pub struct Pantalla {
    // dyn Dibujable representa un tipo que implementa dinámicamente el trait
    pub componentes: Vec<Box<dyn Dibujable>>,
}

impl Pantalla {
    pub fn renderizar(&self) {
        for componente in &self.componentes {
            componente.dibujar();
        }
    }
}
```

#### Implementación concreta de componentes:
```rust
pub struct Boton {
    pub ancho: u32,
    pub alto: u32,
    pub texto: String,
}

impl Dibujable for Boton {
    fn dibujar(&self) {
        println!("Dibujando un botón con texto: '{}'", self.texto);
    }
}

pub struct Selector {
    pub opciones: Vec<String>,
}

impl Dibujable for Selector {
    fn dibujar(&self) {
        println!("Dibujando un selector de opciones.");
    }
}
```

---

### 2. El Patrón State en Rust (OOP Tradicional vs. Enfoque Idiomático)

El libro oficial de Rust demuestra cómo modelar un sistema de flujo de publicación de blogs (Post) utilizando dos arquitecturas de diseño:

#### Enfoque A: El Patrón State tradicional (OOP)
Se define un trait `Estado` y estructuras para cada estado de la publicación (`Borrador`, `Revision`, `Publicado`). El objeto `Post` delega el comportamiento a una caja inteligente `Box<dyn Estado>`.
*   **Inconveniente en Rust:** Obliga al uso de mutabilidad interna o re-asignaciones complejas de estados dentro de punteros, y los fallos de flujo de estado (ej: intentar aprobar un borrador que no ha sido revisado) se descubren únicamente en tiempo de ejecución.

#### Enfoque B: El Patrón Typestate (Enfoque Idiomático de Rust)
En lugar de encapsular el estado en un puntero dinámico dentro de una estructura única, **codificamos los estados en tipos de datos físicos distintos en compilación**:

```rust
// 1. Representa un post en borrador. Solo permite añadir texto.
pub struct BorradorPost {
    contenido: String,
}

impl BorradorPost {
    pub fn nuevo() -> BorradorPost {
        BorradorPost { contenido: String::new() }
    }

    pub fn agregar_texto(&mut self, texto: &str) {
        self.contenido.push_str(texto);
    }

    // Transiciona a un tipo diferente consumiendo la propiedad (self)
    pub fn solicitar_revision(self) -> PostEnRevision {
        PostEnRevision { contenido: self.contenido }
    }
}

// 2. Representa un post en revisión. No permite editar ni leer el contenido directamente.
pub struct PostEnRevision {
    contenido: String,
}

impl PostEnRevision {
    // Aprueba el post y devuelve el tipo final publicado
    pub fn aprobar(self) -> PostPublicado {
        PostPublicado { contenido: self.contenido }
    }
}

// 3. Representa el post publicado. Permite leer su contenido.
pub struct PostPublicado {
    contenido: String,
}

impl PostPublicado {
    pub fn contenido(&self) -> &str {
        &self.contenido
    }
}
```
*   **Ventaja:** Si el desarrollador intenta llamar a `.contenido()` sobre un post que está en estado `BorradorPost`, el programa **no compilará**. Los errores de lógica de estado se detectan en tiempo de compilación sin sobrecostos de ejecución.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Despacho Dinámico y Seguridad de Objetos (Object Safety)
Cuando usamos `dyn Trait`, Rust implementa **Despacho Dinámico (Dynamic Dispatch)**. El compilador no puede saber qué función de código máquina llamar en tiempo de compilación. Por ello, genera una tabla de métodos virtuales (`vtable`) en memoria y accede a ella en ejecución a través de un puntero gordo (*Fat Pointer*).

#### ¿Por qué no todos los Traits pueden convertirse en Trait Objects?
Para que un trait pueda usarse de forma dinámica (ser **Object Safe**), debe cumplir dos reglas matemáticas estrictas que garantizan que el compilador pueda construir su `vtable` de forma coherente:

1.  **El método no debe retornar `Self`:**
    *   **Por qué:** El compilador necesita saber el tamaño físico del tipo de retorno en compilación. Dado que `dyn Trait` representa tipos heterogéneos de diferentes tamaños, si una función devuelve `Self` (el tipo concreto original), el tamaño de retorno es variable y el compilador no puede reservar el marco de la pila (*stack frame*).
2.  **El método no debe tener parámetros de tipo genérico:**
    *   **Por qué:** Un método genérico requiere monomorfización (generar código en ensamblador por cada tipo con el que se le llame). Si el trait se convierte en dinámico, LLVM no puede predecir en tiempo de compilación qué tipos concretos invocará el cliente externo, imposibilitando la generación previa de las tablas virtuales necesarias.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Comparativa: Composición vs. Herencia

| Operación / Concepto | OOP Tradicional (C++, Java, C#) | Enfoque Rust |
| :--- | :--- | :--- |
| **Reutilización de Código** | Herencia de Clases (`class B extends A`). | Composición de estructuras (`struct B { a: A }`). |
| **Compartir Interfaz** | Métodos virtuales o interfaces públicas. | Implementación de Traits comunes (`impl Trait for T`). |
| **Despacho de Métodos** | Dinámico por defecto (métodos virtuales). | Estático por defecto (genéricos), dinámico explícito (`dyn`). |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Intentar instanciar un Trait Object sin usar un puntero
❌ **Código Erróneo:**
```rust
// Error: dyn Dibujable no tiene un tamaño conocido en tiempo de compilación
let componente: dyn Dibujable = Boton {
    ancho: 10,
    alto: 20,
    texto: String::from("OK"),
};
```
*   **Mensaje de Error:** `error[E0277]: the size for values of type `dyn Dibujable` cannot be known at compilation time`
*   ✔️ **Solución:** Los tipos dinámicos deben residir siempre detrás de una referencia física que proporcione un tamaño constante al compilador (como `Box<dyn Dibujable>` o `&dyn Dibujable`).

#### 2. Intentar usar un Trait que no es Object Safe como dinámico
❌ **Código Erróneo:**
```rust
pub trait ClonableSeguro {
    fn duplicar(&self) -> Self; // Retorna 'Self' -> No es Object Safe
}

fn procesar(x: Box<dyn ClonableSeguro>) {
    // Error de compilación al intentar crear el Trait Object
}
```
*   **Mensaje de Error:** `error[E0038]: the trait `ClonableSeguro` cannot be made into an object`
*   ✔️ **Solución:** Rediseñar la interfaz para evitar retornar `Self` en métodos que requieran despacho dinámico, o utilizar despacho estático con parámetros genéricos (`T: ClonableSeguro`) si es posible.
