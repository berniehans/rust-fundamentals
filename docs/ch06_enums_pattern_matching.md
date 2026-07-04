# Capítulo 06: Enums y Pattern Matching (Coincidencia de Patrones)

Este documento ofrece un desglose de nivel arquitectónico del sistema de tipos algebraicos y coincidencia de patrones en Rust, analizando los Enums, el tipo `Option<T>`, el flujo de control exhaustivo y el comportamiento físico en memoria de estas abstracciones.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Tipos Algebraicos de Datos (ADTs) y Enums
En Rust, los **Enums (Enumeraciones)** adoptan una filosofía de la teoría de lenguajes y la programación funcional conocida como **Tipos Algebraicos de Datos (Algebraic Data Types - ADTs)**. Específicamente, se comportan como **Tipos Suma (Sum Types)**.

Mientras que una estructura (*struct*) es un tipo producto (representa el Campo A **Y** el Campo B), un enum representa una suma lógica: un valor solo puede pertenecer a la Variante A **O** a la Variante B **O** a la Variante C. Además, cada variante puede contener datos asociados de cualquier tipo (estructuras anónimas, tuplas o tipos primitivos), haciendo que los enums sean muy útiles para modelar máquinas de estado y jerarquías lógicas.

### El Problema del Valor Nulo y el Enfoque Seguro de Rust
La referencia nula (`null`) es propensa a producir vulnerabilidades y caídas de sistemas debido a dereferencias de punteros nulos. Rust elimina el concepto de `null` de su sistema de tipos por diseño. Para expresar la posibilidad de que un valor esté ausente, Rust utiliza el enum genérico **`Option<T>`**:
*   `Some(T)`: Representa la presencia de un valor de tipo `T`.
*   `None`: Representa la ausencia de valor.

Dado que `Option<T>` y `T` son tipos completamente diferentes ante el compilador, es imposible utilizar un valor potencialmente ausente directamente sin antes desempaquetarlo de manera obligatoria y explícita, previniendo errores de desreferenciación nula en tiempo de ejecución.

### Exhaustividad en la Coincidencia de Patrones (Pattern Matching)
Para extraer y procesar los datos encapsulados dentro de los enums, Rust provee el operador de control de flujo **`match`**. A diferencia de otros lenguajes, `match` en Rust impone una regla estricta de **exhaustividad**: el desarrollador debe cubrir todas las posibles variantes. Si se omite una sola variante, el compilador detendrá la construcción.

---

## 2. Anatomía y Semántica de la Sintaxis

### Declaración e Instanciación de Enums con Carga Útil
En Rust, un mismo enum puede contener variantes con diferentes estructuras sintácticas:

```rust
enum Mensaje {
    Salir,                                    // Variante unitaria (sin datos)
    Mover { x: i32, y: i32 },                 // Variante estructurada
    Escribir(String),                         // Variante de tupla con String
    CambiarColor(u8, u8, u8),                 // Variante de tupla con enteros
}

fn main() {
    let salir = Mensaje::Salir;
    let mover = Mensaje::Mover { x: 10, y: 20 };
    let escribir = Mensaje::Escribir(String::from("Hola"));
}
```

### El Tipo `Option<T>` y su Integración
El tipo `Option<T>` se encuentra pre-importado en el preludio de Rust, por lo que se pueden instanciar sus variantes de forma directa sin el prefijo `Option::`:

```rust
fn main() {
    let algun_numero: Option<i32> = Some(5);
    let numero_ausente: Option<i32> = None;
}
```

### La Expresión `match` y la Estructura Corta `if let`
```rust
enum Moneda {
    Centavo,
    Peso,
}

fn evaluar_moneda(moneda: Moneda) -> u8 {
    match moneda {
        Moneda::Centavo => 1,
        Moneda::Peso => 100,
    }
}

// if let como atajo sintáctico
fn mostrar_si_centavo(moneda: Moneda) {
    if let Moneda::Centavo = moneda {
        println!("Es un centavo!");
    }
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Layout de Memoria de Enums y el Tag Discriminante
Para representar un enum en memoria, el compilador genera una estructura física contigua compuesta por:
1.  Un **tag o discriminante** (un número entero invisible, usualmente de 1 byte) que indica qué variante está activa en ese momento.
2.  El **payload o carga útil** de la variante, cuyo tamaño físico en memoria equivale al tamaño de la variante más grande del enum.

```
Layout General en Memoria de Mensaje (con alineación de datos):
+--------------------+------------------------------------------------+
| Tag (1 byte: 0-3)  | Payload (Alineado al tamaño máximo del String) |
+--------------------+------------------------------------------------+
```

### Optimización de Puntero Nulo (Null Pointer Optimization - NPO)
Para evitar el sobrecosto de memoria que añade el discriminante, el compilador realiza la **Optimización de Puntero Nulo (NPO)** sobre el tipo `Option<T>`.

Cuando el tipo genérico `T` es una referencia (`&T`), un puntero inteligente (`Box<T>`, `Rc<T>`) o una función (`fn`), Rust garantiza por contrato de seguridad que la dirección física a la que apuntan **nunca puede ser nula (`0x0`)**.

El compilador aprovecha esto y representa `None` utilizando el valor de bits nulo `0x0` directamente en la celda del puntero. Cualquier otro valor de bits representará la variante `Some(T)` apuntando a la dirección correspondiente.
*   `size_of::<&i32>()` = 8 bytes.
*   `size_of::<Option<&i32>>()` = **8 bytes** (en lugar de 16 bytes).
No hay sobrecosto de memoria en comparación con un puntero nulo tradicional en C.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Opciones de Extracción de Datos en Enums

| Construcción | Comprobación de Exhaustividad | Caso de Uso Recomendado |
| :--- | :--- | :--- |
| `match valor { ... }` | **Sí (Obligatorio por compilador)** | Evaluación y control de flujo de enums complejos con múltiples variantes. |
| `if let Patrón = valor { ... }` | **No** | Ejecución condicional rápida cuando solo interesa una variante específica. |
| `while let Patrón = valor { ... }` | **No** | Bucles condicionales de extracción de datos secuenciales. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Coincidencia de patrones no exhaustiva en `match`
Intentar evaluar un enum omitiendo alguna de sus variantes en los brazos del `match`:
❌ **Código Erróneo:**
```rust,compile_fail
enum Acceso { Permitido, Denegado, Pendiente }

fn procesar(a: Acceso) {
    match a {
        Acceso::Permitido => println!("Entra"),
        Acceso::Denegado => println!("Fuera"),
        // Error: falta contemplar la variante Acceso::Pendiente
    }
}
```
*   **Mensaje de Error:** `error[E0004]: non-exhaustive patterns: `Acceso::Pendiente` not covered`
*   ✔️ **Solución:** Cubrir explícitamente todas las variantes del enum, o utilizar el comodín `_` para casos generales por defecto:
    ```rust
    match a {
        Acceso::Permitido => println!("Entra"),
        Acceso::Denegado => println!("Fuera"),
        Acceso::Pendiente => println!("Espere..."), // Opción A
    }
    
    match a {
        Acceso::Permitido => println!("Entra"),
        _ => println!("Acceso no permitido"), // Opción B
    }
    ```

#### 2. Ensombrecimiento accidental de variables en brazos del match
Intentar comparar un valor contra una variable existente en el entorno, introduciendo un identificador de variable en el patrón:
❌ **Código Erróneo:**
```rust,compile_fail
fn main() {
    let valor_esperado = 10;
    let valor_recibido = Some(5);

    match valor_recibido {
        // Error: 'valor_esperado' aquí no es una comparación de igualdad.
        // Declara una nueva variable local que coincide con cualquier Some.
        Some(valor_esperado) => println!("Correcto: {valor_esperado}"),
        None => (),
    }
}
```
*   **Comportamiento Anómalo:** El brazo coincide siempre, imprimiendo `"Correcto: 5"`.
*   ✔️ **Solución:** Utilizar una guarda de match (`if`) para validar contra variables externas del entorno, o definir el valor como una constante formal:
    ```rust
    match valor_recibido {
        Some(x) if x == valor_esperado => println!("Correcto: {x}"),
        _ => (),
    }
    ```
