# Capítulo 11: Escribir Pruebas Automatizadas

Este documento proporciona un análisis exhaustivo y de bajo nivel del arnés de pruebas integrado de Rust (Test Harness). Se detallan las aserciones, el comportamiento de la ejecución concurrente, la compilación condicional y la estructuración física entre pruebas unitarias e integración.

---

## 1. Conceptos Fundamentales (Desde Cero)

### La Filosofía del Testing en Rust
En el desarrollo de software seguro, las pruebas automatizadas son la última línea de defensa contra regresiones de código. Si bien el sistema de tipos estricto y el verificador de préstamos (Borrow Checker) de Rust previenen categóricamente bugs de memoria (como desbordamientos o punteros colgantes), no pueden garantizar la corrección de la lógica de negocio. Por ejemplo, el compilador no sabe si un algoritmo de ordenamiento ordena de forma ascendente o descendente.

### El Test Harness Integrado de Cargo y Rust
A diferencia de otros ecosistemas que requieren instalar paquetes y frameworks externos (como JUnit en Java, pytest en Python o Jest en JavaScript), **Rust incorpora su propio motor de pruebas (Test Harness) directamente en la biblioteca estándar y Cargo**. 

Esto proporciona:
*   Una sintaxis unificada para escribir aserciones y configurar suites de prueba.
*   Ejecución multihilo nativa para acelerar la validación.
*   Integración directa de documentación ejecutable (Doctests).

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. La Función de Prueba (`#[test]`)
En su nivel más básico, un test en Rust es una función anotada con el atributo **`#[test]`**. El arnés ejecuta estas funciones y registra si terminaron de forma normal (éxito) o si provocaron un pánico (fallo).

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn comprobar_suma() {
        let resultado = 2 + 2;
        // Si la aserción es falsa, invoca panic! y el test falla
        assert_eq!(resultado, 4);
    }
}
```

### 2. Macros de Aserción
El compilador provee tres macros fundamentales para validar estados en los tests:

*   **`assert!(expresion_booleana)`**: Evalúa una expresión. Si devuelve `false`, provoca un pánico.
*   **`assert_eq!(izquierda, derecha)`**: Valuda que ambos valores sean idénticos utilizando la implementación del trait `PartialEq`. Si fallan, imprimen ambos valores en la consola formateados mediante el trait `Debug`.
*   **`assert_ne!(izquierda, derecha)`**: Valida que los valores sean distintos.

#### Mensajes Personalizados
Todas las macros de aserción aceptan argumentos de formato adicionales similares a la macro `println!`. Esto es sumamente útil para proveer contexto en reportes de integración continua:

```rust
assert_eq!(
    resultado, 
    4, 
    "El resultado de la operación debía ser 4, pero obtuvimos {resultado}"
);
```

### 3. Validación de Pánicos con `#[should_panic]`
A veces es necesario comprobar que el código falla bajo condiciones inválidas (como ingresar datos fuera de rango). Para validar que una función provoca un pánico de manera controlada, se añade el atributo **`#[should_panic]`**:

```rust
#[test]
#[should_panic(expected = "índice fuera de límites")]
fn comprobar_acceso_invalido() {
    let v = vec![1, 2, 3];
    let _ = v[99]; // Provoca pánico, haciendo que el test pase con éxito.
}
```
*   **Filtro `expected`:** Es fundamental usar la propiedad `expected = "texto"` para verificar que el pánico ocurre debido al mensaje esperado, y no a causa de otro bug imprevisto en otra sección de la función.

### 4. Tests que retornan `Result<(), E>`
En lugar de forzar pánicos directos, Rust permite escribir pruebas que retornen un tipo `Result`:

```rust
#[test]
fn comprobar_apertura_archivo() -> Result<(), String> {
    let _archivo = std::fs::File::open("no_existe.txt")
        .map_err(|e| format!("Error de I/O: {e:?}"))?;
    Ok(())
}
```
*   Si la función retorna `Ok(())`, el test se marca como exitoso.
*   Si retorna `Err`, el test falla. Esto permite utilizar el operador `?` de forma limpia dentro del cuerpo de la prueba.

### 5. Control de Ejecución mediante la CLI de Cargo
El comando `cargo test` compila el código en modo de test y corre el binario resultante. Se le pueden pasar parámetros para alterar su comportamiento:

#### Paralelismo vs. Secuencialidad
Por defecto, Rust ejecuta todas las pruebas concurrentemente usando múltiples hilos para maximizar el uso de la CPU. Si tus pruebas interactúan con un recurso compartido único (como archivos locales o variables globales en memoria), debes forzar la ejecución en un solo hilo:
```bash
cargo test -- --test-threads=1
```

#### Captura de Salida (`stdout`)
Si una prueba es exitosa, Rust oculta todo lo impreso en consola mediante `println!` o `print!`. Si deseas ver la salida de las pruebas exitosas (por ejemplo, para depurar logs intermedios):
```bash
cargo test -- --show-output
```

#### Filtrado y Pruebas Ignoradas
*   **Filtrar por nombre:** `cargo test comprobar` ejecutará solo los tests cuyo nombre contenga la palabra "comprobar".
*   **Ignorar tests lentos:** Puedes marcar tests pesados con el atributo `#[ignore]`.
    ```rust
    #[test]
    #[ignore]
    fn simulacion_pesada() { ... }
    ```
*   **Ejecutar solo los ignorados:**
    ```bash
    cargo test -- --ignored
    ```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Compilación Condicional (`#[cfg(test)]`)
El atributo `#[cfg(test)]` le indica al compilador de Rust (`rustc`) que compile el módulo asociado **únicamente** cuando se invoque el comando `cargo test`. 
*   **Builds de Producción:** Cuando corres `cargo build` o `cargo build --release`, el compilador ignora por completo el código contenido en bloques `#[cfg(test)]`.
*   **Cero Costo en Producción:** Las dependencias del módulo (como funciones de utilidad exclusivas de test o macros complejas de aserción) no se vinculan al binario final, reduciendo el tamaño físico del ejecutable y previniendo fugas de vectores de ataque o ingeniería inversa.

---

### Arquitectura Física: Pruebas Unitarias vs. Integración

Rust distingue físicamente entre dos metodologías de prueba, organizándolas en ubicaciones y semánticas distintas:

```
Estructura de Directorios del Crate
├── Cargo.toml
├── src/
│   ├── lib.rs            # Código productivo
│   └── tests.rs          # [Pruebas Unitarias] (dentro de src/)
└── tests/
    └── integracion.rs    # [Pruebas de Integración] (fuera de src/)
```

#### 1. Pruebas Unitarias (Unit Tests)
*   **Propósito:** Probar pequeños componentes aislados (funciones, structs) en condiciones controladas.
*   **Ubicación:** Dentro de los mismos archivos de código fuente en `src/`, encapsuladas en un submódulo `mod tests`.
*   **Acceso a Privacidad:** Al ser un submódulo hijo, las pruebas unitarias **tienen acceso total** a funciones, estructuras y campos privados del módulo padre utilizando `use super::*;`.

#### 2. Pruebas de Integración (Integration Tests)
*   **Propósito:** Validar que múltiples partes de la biblioteca funcionen de manera conjunta, interactuando con el crate exactamente igual a como lo haría un desarrollador externo.
*   **Ubicación:** En un directorio independiente llamado `tests/` situado en la raíz del proyecto (al mismo nivel que `src/`).
*   **Acceso a Privacidad:** Cada archivo en `tests/` es compilado como un **crate independiente e individual**. Por lo tanto, no tienen acceso a elementos privados del código de origen; solo pueden consumir la API pública expuesta por el crate (`pub`).
*   **Solo para Librerías:** Los crates de tipo binario puro (`src/main.rs`) sin `src/lib.rs` no pueden ser probados mediante tests de integración directamente porque no exponen una biblioteca importable.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Cheat Sheet Sintáctica y de Atributos

| Atributo / Comando | Ámbito | Propósito |
| :--- | :--- | :--- |
| `#[cfg(test)]` | Módulo | Indica compilación exclusiva en modo de testing. |
| `#[test]` | Función | Registra la función como punto de entrada de una prueba. |
| `#[should_panic]` | Función | Espera un fallo controlado (pánico) de la función. |
| `#[ignore]` | Función | Excluye la función de la suite de pruebas normal. |
| `cargo test -- --nocapture` | Comando CLI | Desactiva la captura de `stdout` para ver impresiones en vivo. |

---

### Errores Comunes de Compilación y sus Soluciones

#### 1. Intentar testear funciones privadas desde Pruebas de Integración
❌ **Código en `tests/integracion_test.rs`:**
```rust
use mi_crate::funcion_privada_interna;

#[test]
fn test_integracion() {
    // Error: funcion_privada_interna es privada en mi_crate
    assert_eq!(funcion_privada_interna(), 42); 
}
```
*   **Mensaje de Error:** `error[E0603]: function `funcion_privada_interna` is private`
*   ✔️ **Solución:** Mover la prueba al módulo unitario interno en `src/lib.rs` (marcado con `#[cfg(test)]`) donde sí se tiene acceso al ámbito privado mediante `use super::*;`.

#### 2. Carreras de datos (Race Conditions) y colisión de recursos globales
Si varios tests escriben en el mismo archivo físico al mismo tiempo de forma paralela:
❌ **Código de Test Concurrente:**
```rust
#[test]
fn test_guardar_datos() {
    std::fs::write("temp.txt", "datos_test_1").unwrap();
    let contenido = std::fs::read_to_string("temp.txt").unwrap();
    assert_eq!(contenido, "datos_test_1");
}

#[test]
fn test_guardar_otros_datos() {
    std::fs::write("temp.txt", "datos_test_2").unwrap();
    let contenido = std::fs::read_to_string("temp.txt").unwrap();
    assert_eq!(contenido, "datos_test_2");
}
```
*   **Síntoma:** Los tests fallan de forma intermitente (flaky tests) debido a que un hilo sobrescribe el archivo mientras el otro lo lee.
*   ✔️ **Solución A:** Ejecutar los tests de manera secuencial (`cargo test -- --test-threads=1`).
*   ✔️ **Solución B (Recomendada):** Utilizar nombres de archivos únicos generados al azar por prueba o emplear directorios temporales seguros (por ejemplo, con el crate `tempfile`).

#### 3. Olvidar el atributo `#[cfg(test)]` en el módulo de pruebas
Si declaras un módulo de pruebas sin el atributo de configuración condicional:
❌ **Código en `src/lib.rs`:**
```rust
mod tests {
    // No tiene #[cfg(test)]
    #[test]
    fn mi_prueba() {
        assert!(true);
    }
}
```
*   **Problema:** El compilador procesará el módulo `tests` en builds de producción. Si en este módulo utilizas dependencias de desarrollo (`[dev-dependencies]` en `Cargo.toml` como frameworks de aserción avanzada), la compilación fallará al hacer `cargo build --release` debido a que esas dependencias no están disponibles en modo de producción.
*   ✔️ **Solución:** Marcar siempre el módulo contenedor con el atributo condicional correspondiente:
    ```rust
    #[cfg(test)]
    mod tests { ... }
    ```
