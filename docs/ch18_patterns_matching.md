# Capítulo 18: Patrones y Coincidencias (Pattern Matching)

Este documento proporciona un análisis exhaustivo de los mecanismos de coincidencia de patrones en Rust. Se detallan las construcciones de control de flujo basadas en patrones, la distinción matemática entre patrones refutables e irrefutables, las técnicas avanzadas de desestructuración, y la optimización de árboles de decisión en tiempo de compilación.

---

## 1. Conceptos Fundamentales (Desde Cero)

### ¿Qué es la Coincidencia de Patrones?
La **Coincidencia de Patrones (Pattern Matching)** es una herramienta de flujo de control extremadamente potente que permite comparar la estructura de un dato con formas predefinidas, extrayendo partes individuales del dato en variables locales de manera simultánea.
En Rust, los patrones no son simples comparaciones de igualdad (como un bloque `switch-case` en C/Java); son abstracciones declarativas integradas profundamente en el sistema de tipos.

### Patrones Refutables vs. Irrefutables

El compilador clasifica los patrones en dos categorías matemáticas según su probabilidad de coincidencia:

1.  **Patrones Irrefutables (Irrefutable Patterns):** Son aquellos patrones que tienen la garantía matemática de coincidir con cualquier valor posible del tipo de dato evaluado.
    *   **Ejemplo:** `let x = 5;`. El patrón `x` es irrefutable porque siempre se puede asignar cualquier valor a una variable.
2.  **Patrones Refutables (Refutable Patterns):** Son aquellos patrones que pueden fallar al intentar coincidir con el valor evaluado.
    *   **Ejemplo:** `if let Some(x) = valor_option`. El patrón `Some(x)` es refutable porque `valor_option` podría ser `None`, en cuyo caso la coincidencia falla.

#### Restricciones del Compilador:
*   Las sentencias `let`, los parámetros de funciones y los bucles `for` **solo aceptan patrones irrefutables**. Si el compilador permitiera un patrón refutable ahí (ej. `let Some(x) = operacion()`), el programa no sabría qué hacer si la operación devuelve `None`, comprometiendo la seguridad de ejecución.
*   Las construcciones `if let`, `while let` y `let-else` aceptan patrones refutables e irrefutables (aunque el compilador arrojará advertencias de redundancia si se usa un patrón irrefutable).

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Dónde se pueden usar patrones en Rust

#### Bloques `match`
Es la forma más común. Exige que los patrones sean **exhaustivos**:

```rust
match valor {
    1 => println!("Uno"),
    2 | 3 => println!("Dos o Tres"), // Operador OR en patrones
    _ => println!("Cualquier otro número"), // Comodín por defecto
}
```

#### Condicionales `if let` e `if let ... else`
Útil cuando solo nos interesa reaccionar a un patrón refutable específico, ignorando el resto de casos:

```rust
if let Some(valor) = opcion {
    println!("Valor: {valor}");
}
```

#### Bucles `while let`
Ejecuta el bloque repetidamente mientras el patrón refutable siga coincidiendo exitosamente:

```rust
let mut pila = vec![1, 2, 3];
while let Some(elemento) = pila.pop() {
    println!("Elemento extraído: {elemento}");
}
```

#### Sentencia `let-else` (Manejo de Errores con Retorno Anticipado)
Introducida para aplanar el código, desestructura un patrón refutable. Si falla, ejecuta obligatoriamente un bloque `else` que debe divergir (ej: retornar con `return`, `break` o `panic!`):

```rust
fn procesar_edad(usuario: Option<u32>) -> Result<u32, String> {
    // Si es Some, extrae 'edad'. Si es None, ejecuta el else e interrumpe
    let Some(edad) = usuario else {
        return Err(String::from("Usuario sin edad registrada"));
    };

    Ok(edad + 1)
}
```

#### Parámetros de Funciones y Tuplas en `let`
Los parámetros de funciones y los `let` simples realizan desestructuración en sitio mediante patrones irrefutables:

```rust
// Desestructuración directa de una tupla en variables locales
let (x, y, z) = (10, 20, 30);

// Desestructuración de los campos de una estructura en los argumentos
struct Punto { x: i32, y: i32 }
fn imprimir_coordenadas(Punto { x, y }: Punto) {
    println!("Coordenadas: ({x}, {y})");
}
```

---

### 2. Sintaxis de Patrones Avanzados

#### Rangos (`..=`)
Permite comprobar si un valor se encuentra dentro de un intervalo inclusivo:
```rust
let letra = 'c';
match letra {
    'a'..='z' => println!("Letra minúscula"),
    _ => println!("Otro carácter"),
}
```

#### Ignorar valores con `..`
Puedes ignorar partes específicas de una estructura o tupla sin tener que escribir comodines `_` para cada campo:
```rust
struct Coordenada3D { x: i32, y: i32, z: i32 }
let c = Coordenada3D { x: 0, y: 10, z: 20 };

match c {
    // Solo nos interesa comprobar que x es 0, ignorando y y z
    Coordenada3D { x: 0, .. } => println!("En el origen del eje X"),
    _ => (),
}
```

#### Guardas de Match (`if` adicional)
Una guarda de match es una condición booleana adicional especificada después del patrón que debe cumplirse para que el brazo de coincidencia sea seleccionado:
```rust
let numero = Some(4);
match numero {
    Some(x) if x % 2 == 0 => println!("Es un número par: {x}"),
    Some(x) => println!("Es un número impar: {x}"),
    None => (),
}
```

#### El Enlace `@` (Bindings)
El operador `@` permite crear una variable que almacena un valor al mismo tiempo que probamos si dicho valor coincide con un patrón complejo o rango:

```rust
enum Mensaje {
    Hola { id: i32 },
}

let mensaje = Mensaje::Hola { id: 5 };

match mensaje {
    Mensaje::Hola {
        // Guardamos el id en la variable 'id_variable' SOLO si está en el rango 3..=7
        id: id_variable @ 3..=7,
    } => println!("ID en rango medio encontrado: {id_variable}"),
    Mensaje::Hola { id: 10..=20 } => println!("ID en rango alto sin capturar variable"),
    _ => (),
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Árboles de Decisión del Compilador
A nivel de código máquina, la coincidencia de patrones complejos no se traduce en una secuencia ineficiente de comparaciones lineales `if-else` de arriba a abajo.
El compilador de Rust analiza la estructura de los patrones y construye un **Árbol de Decisión Matemático**:
*   Si los patrones evalúan un enum discriminado, el compilador traduce el `match` en una **tabla de saltos de CPU (Jump Table)** en ensamblador. El procesador salta directamente a la dirección de memoria de la instrucción correspondiente en un costo constante $O(1)$, maximizando el rendimiento del predictor de saltos del hardware.
*   Las desestructuraciones no realizan copias físicas de memoria a menos que se solicite de forma explícita. Se traducen en desplazamientos directos de punteros en el Stack.

### Verificación de Exhaustividad (Exhaustiveness Checking)
El analizador estático de Rust comprueba que la unión de todos los brazos de un `match` cubra el 100% de los valores representables del tipo de dato.
*   Si el compilador detecta que falta una sola posibilidad, detiene la compilación.
*   Esto previene una categoría clásica de bugs en producción: agregar una nueva variante a un `enum` (ej. agregar un nuevo método de pago `Cryptocurrency` a un enum `PaymentMethod`) y olvidar actualizar las secciones del software que procesan los pagos. En Rust, añadir la variante provoca que el compilador alerte inmediatamente de todos los bloques `match` desactualizados a lo largo del codebase.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet de Sintaxis de Patrones

| Patrón | Significado / Comportamiento | Ejemplo |
| :--- | :--- | :--- |
| `x | y` | Coincide si el valor es `x` o es `y`. | `1 | 2 => ...` |
| `rango..=rango` | Coincide si el valor está en el rango inclusivo. | `'a'..='d' => ...` |
| `..` | Ignora todos los campos restantes no especificados. | `Punto { x, .. }` |
| `ref x` / `ref mut x` | Crea una referencia a la variable capturada (obsoleto por `.as_ref()`). | `Some(ref x) => ...` |
| `var @ pat` | Enlaza el valor que coincide con `pat` a la variable `var`. | `id @ 1..=5` |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Coincidencia No Exhaustiva (*Non-Exhaustive Patterns*)
❌ **Código Erróneo:**
```rust
enum Estado { Activo, Inactivo, Suspendido }

fn procesar(estado: Estado) {
    match estado {
        Estado::Activo => println!("Online"),
        Estado::Inactivo => println!("Offline"),
        // Error: Falta contemplar la variante Estado::Suspendido
    }
}
```
*   **Mensaje de Error:** `error[E0004]: non-exhaustive patterns: `Estado::Suspendido` not covered`
*   ✔️ **Solución A:** Añadir un brazo explícito para contemplar la variante faltante (recomendado para detectar desajustes futuros).
*   ✔️ **Solución B:** Usar el comodín por defecto `_` si no nos interesa distinguir el resto de casos.

#### 2. Ensombrecimiento Accidental de Variables (*Variable Shadowing*)
Intentar comparar un valor contra una variable existente del entorno en lugar de usar un literal:
❌ **Código Erróneo:**
```rust
fn main() {
    let id_esperado = 10;
    let id_recibido = 5;

    match id_recibido {
        // Error: Esto no compara id_recibido con id_esperado.
        // Rust asume que estamos declarando una nueva variable local llamada 'id_esperado'
        // que ensombrece a la anterior y coincide con cualquier valor.
        id_esperado => println!("ID correcto: {id_esperado}"), 
    }
}
```
*   **Comportamiento Anómalo:** El primer brazo coincide siempre, imprimiendo `"ID correcto: 5"`.
*   ✔️ **Solución A:** Utilizar guardas de match para realizar la comparación contra variables externas:
    ```rust
    match id_recibido {
        id if id == id_esperado => println!("ID correcto: {id}"),
        _ => println!("ID incorrecto"),
    }
    ```
*   ✔️ **Solución B:** Si el valor esperado es constante, definirlo como una constante formal (`const ID_ESPERADO: i32 = 10;`), ya que las constantes sí pueden usarse directamente en patrones sin ensombrecer.
