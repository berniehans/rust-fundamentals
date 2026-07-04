# Capítulo 07: Gestión de Proyectos en Crecimiento (Paquetes, Crates y Módulos)

Este documento ofrece un análisis riguroso de las herramientas de encapsulación, modularización y estructuración de proyectos de gran escala en Rust, detallando su funcionamiento interno en tiempo de compilación y ejecución.

---

## 1. Conceptos Fundamentales (Desde Cero)

### El Sistema de Módulos de Rust
Para construir arquitecturas de software mantenibles, es imprescindible contar con herramientas de modularización que permitan segmentar el código en fronteras lógicas y controlar los niveles de exposición de la información. Rust cuenta con un robusto sistema de módulos compuesto por cuatro elementos jerárquicos:

1.  **Paquetes (Packages):** Una característica de Cargo que permite construir, probar y compartir crates. Un paquete se define mediante un archivo manifiesto `Cargo.toml`. Contiene una colección de uno o más crates (como máximo puede contener un único crate de biblioteca (`lib`), pero puede almacenar múltiples crates binarios (`bin`)).
2.  **Crates (Cajas / Unidades de Compilación):** Es la unidad fundamental de compilación para el compilador de Rust (`rustc`). Genera un archivo ejecutable (binario) o una biblioteca (`.rlib`). Los crates contienen un árbol jerárquico de módulos.
    *   *Crate Root:* El archivo de origen desde el cual se inicia la compilación y que conforma el módulo raíz del crate (típicamente `src/main.rs` para un binario y `src/lib.rs` para una biblioteca).
3.  **Módulos (Modules):** Unidades internas que permiten organizar y estructurar el código de un crate en namespaces distintos, previniendo colisiones de nombres.
4.  **Rutas (Paths):** La notación formal para localizar y referenciar un elemento (función, estructura, enum o módulo) dentro de la jerarquía modular.

### Jerarquía y Módulos como Árboles
El sistema de módulos no se define únicamente por la estructura física de directorios en el disco duro, sino por una estructura de **árbol lógico** cuyo nodo principal es el *crate root*. La relación entre módulos es jerárquica (módulos padres que encapsulan submódulos hijos), análoga a la organización de carpetas y archivos en un sistema operativo.

### La Regla de Oro: Privado por Defecto (Private by Default)
En Rust, la privacidad es una propiedad estricta. **Todo elemento (módulo, función, método, estructura, enum, constante) es privado por defecto**. 
*   Un módulo secundario (hijo) tiene acceso total a los elementos privados definidos en sus módulos antecesores (padres o superiores).
*   Un módulo principal (padre) **no puede** acceder a los elementos privados declarados dentro de sus módulos secundarios.
*   Cualquier código externo al crate solo puede acceder a elementos que hayan sido declarados explícitamente con la palabra clave **`pub`**.

---

## 2. Anatomía y Semántica de la Sintaxis

### Declaración de Módulos y Control de Acceso (`pub`)
Un módulo puede declararse en línea utilizando llaves o delegar su contenido a un archivo externo del disco:

```rust
// 1. Declaración de módulo en línea
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

### Privacidad Selectiva en Estructuras y Enums
*   **Campos de Estructuras:** Si declaramos una estructura como pública (`pub struct`), la estructura en sí es accesible externamente, pero **sus campos individuales siguen siendo privados por defecto**. Se debe marcar con `pub` cada campo que se desee exponer.
*   **Variantes de Enums:** A diferencia de las estructuras, si declaramos un enum como público (`pub enum`), **todas sus variantes pasan a ser públicas automáticamente**. No es necesario ni válido anotar cada variante individual con `pub`.

```rust
pub struct Desayuno {
    pub pan: String,      // Campo público
    fruta_temporada: String, // Campo privado
}

impl Desayuno {
    pub fn con_centeno(fruta: &str) -> Desayuno {
        Desayuno {
            pan: String::from("Centeno"),
            fruta_temporada: String::from(fruta),
        }
    }
}
```

### Rutas Absolutas y Relativas
*   **Ruta Absoluta:** Comienza desde la raíz del crate utilizando la palabra clave `crate::` (o el nombre de una dependencia externa).
*   **Ruta Relativa:** Comienza desde el módulo actual utilizando identificadores de ruta, `self` (el módulo actual) o `super` (el módulo padre inmediato).

```rust
pub fn servir_mesa() {
    // Ruta Absoluta
    crate::front_of_house::hosting::add_to_waitlist();
}
```

### Directiva `use` y Reexportación (`pub use`)
La palabra clave `use` crea un enlace directo del elemento en el ámbito actual, actuando como un alias para evitar escribir rutas extensas en cada invocación.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Resolución de Módulos en Compilación
Cuando declaras un módulo externo utilizando la sintaxis `mod back_of_house;` (sin llaves de bloque):
1.  El compilador `rustc` no busca todas las dependencias en tiempo de ejecución. En su lugar, durante la fase de análisis sintáctico de compilación, busca un archivo físico en el disco que coincida con el nombre del módulo.
2.  Las rutas permitidas para encontrar el archivo de un módulo llamado `modulo` secundario de `main.rs` son:
    *   `src/modulo.rs` (Estilo moderno de Rust 2018+).
    *   `src/modulo/mod.rs` (Estilo antiguo de Rust 2015, aún soportado pero menos preferido).
3.  Si encuentra el archivo, concatena lógicamente su árbol AST al nodo principal del crate root, generando una sola unidad de compilación coherente.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Visibilidad de Elementos

| Sintaxis | Visibilidad / Alcance | Accesible por Crate Externo |
| :--- | :--- | :--- |
| `mod modulo` | Privado por defecto al módulo padre. | No |
| `pub mod modulo` | Módulo público a nivel externo. | Sí |
| `pub struct S` | Estructura pública (campos privados). | Sí (pero no instanciable mediante literal) |
| `pub enum E` | Enum público (variantes públicas). | Sí (todas sus variantes) |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Intentar acceder a un campo privado de una estructura pública
Intentar leer o escribir en un campo que carece de la palabra clave `pub` desde fuera de su módulo de origen:
❌ **Código Erróneo:**
```rust,compile_fail
// En src/restaurante.rs
pub struct Desayuno {
    pub pan: String,
    fruta_temporada: String, // Campo privado
}

// En src/main.rs
fn main() {
    let mut orden = restaurante::Desayuno::con_centeno("Fresa");
    // Error: fruta_temporada es privada
    orden.fruta_temporada = String::from("Manzana"); 
}
```
*   **Mensaje de Error:** `error[E0616]: field `fruta_temporada` of struct `Desayuno` is private`
*   ✔️ **Solución:** Si el negocio requiere modificar ese campo directamente, añádele la palabra clave `pub`. Si no, expón un método mutable público (`pub fn set_fruta(&mut self, ...)`) en el bloque `impl` para encapsular el acceso:
    ```rust
    pub struct Desayuno {
        pub pan: String,
        pub fruta_temporada: String, // Campo público
    }
    ```

#### 2. Declaración de módulo sin archivo físico correspondiente en disco
Declarar un módulo externo usando `mod` pero olvidar crear el archivo o colocarlo en la ruta incorrecta:
❌ **Código en `src/lib.rs`:**
```rust,compile_fail
// El compilador busca src/servicios.rs o src/servicios/mod.rs
mod servicios; 
```
*   **Mensaje de Error:** `error[E0583]: file not found for module `servicios``
*   ✔️ **Solución:** Crear el archivo en la ruta esperada por `rustc` para la edición actual (ej. `src/servicios.rs`) o ajustar el path si se encuentra en un submódulo anidado.
