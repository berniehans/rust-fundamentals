# Capítulo 16: Concurrencia Sin Miedo (Fearless Concurrency)

Este documento proporciona un análisis exhaustivo del modelo de concurrencia segura en Rust. Se detallan la instanciación de hilos de ejecución, el paso de mensajes mediante canales MPSC, el acceso a datos concurrentes con exclusión mutua (`Mutex`) y conteo atómico de referencias (`Arc`), y los traits fundamentales del compilador para concurrencia segura: `Send` y `Sync`.

---

## 1. Conceptos Fundamentales (Desde Cero)

### El Desafío de la Concurrencia Tradicional
En lenguajes de programación de sistemas tradicionales (como C o C++), escribir software multiproceso es una de las tareas más propensas a errores. Los desarrolladores deben lidiar con:
*   **Condiciones de Carrera (Data Races):** Ocurren cuando dos o más hilos acceden a la misma dirección física de memoria simultáneamente, al menos uno de los accesos es de escritura, y no existe sincronización entre ellos. Esto corrompe los datos y causa un comportamiento indefinido.
*   **Bloqueos Mutuos (Deadlocks):** Suceden cuando dos hilos se bloquean mutuamente esperando recursos que el otro hilo posee, deteniendo la ejecución del software de manera indefinida.

### La Filosofía de Rust: Concurrencia Sin Miedo
Rust elimina estas categorías de bugs en tiempo de compilación. Gracias a las reglas del modelo de *Ownership* y los traits del compilador:
1.  **Si el código compila, está libre de condiciones de carrera de memoria (data races).** El borrow checker impide físicamente compartir variables mutables entre hilos sin herramientas explícitas de sincronización.
2.  El software puede beneficiarse de paralelismo real aprovechando la aceleración de hardware sin añadir inestabilidad.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Invocación de Hilos (`thread::spawn`)
Para crear un nuevo hilo de ejecución en el sistema operativo, se utiliza `std::thread::spawn` pasándole un closure con el código a ejecutar:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn devuelve un JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hilo secundario: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // .join() bloquea el hilo principal hasta que el hilo secundario termine
    handle.join().unwrap();
}
```

#### Uso de `move` en Closures de Hilos
Dado que Rust no puede predecir cuánto tiempo vivirá un hilo secundario en ejecución, **no permite pasar referencias locales** al closure. Se debe forzar la transferencia de propiedad total de las variables usando `move`:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // Sin 'move', esto no compila porque 'v' podría destruirse en el hilo principal
    // mientras el secundario sigue leyéndolo.
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

---

### 2. Paso de Mensajes (Canales MPSC)
Rust implementa el lema de Go: *"No te comuniques compartiendo memoria; en su lugar, comparte memoria comunicándote"*. 
Para ello, provee **MPSC (Multi-Producer, Single-Consumer)**: múltiples hilos emisores y un único hilo receptor.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // tx = Transmisor (Transmitter), rx = Receptor (Receiver)
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // Clonamos el transmisor para tener múltiples productores

    thread::spawn(move || {
        let val = String::from("mensaje desde Hilo 1");
        tx1.send(val).unwrap(); // send toma propiedad del dato enviado
    });

    thread::spawn(move || {
        let val = String::from("mensaje desde Hilo 2");
        tx.send(val).unwrap();
    });

    // El receptor lee secuencialmente de forma bloqueante
    for mensaje en rx {
        println!("Recibido: {mensaje}");
    }
}
```
*   **`.send()`:** Transfiere la propiedad del dato enviado. No bloquea el hilo emisor.
*   **`.recv()`:** Bloquea el hilo receptor hasta que llegue un mensaje o todos los emisores se destruyan.

---

### 3. Exclusión Mutua (`Mutex<T>`) y Estado Compartido
Cuando el paso de mensajes no es viable y se requiere mutar un recurso compartido desde múltiples hilos, se utiliza `Mutex<T>` (Mutual Exclusion).

#### La Solución Atómica: `Arc<T>`
Un `Mutex` común no puede ser compartido entre hilos porque carece de la propiedad de ser clonado por múltiples dueños. Usar `Rc<Mutex<T>>` fallará en compilación porque `Rc` no es seguro en multi-hilo. Se debe utilizar **`Arc<T>` (Atomic Reference Counted)**:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Creamos un contador protegido en el montón
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clon = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            // .lock() bloquea el mutex. Devuelve un MutexGuard (puntero inteligente)
            let mut num = contador_clon.lock().unwrap();
            *num += 1;
        }); // MutexGuard sale de ámbito aquí y libera el bloqueo de forma automática
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Resultado: {}", *contador.lock().unwrap()); // Imprime 10
}
```

---

### 4. Los Traits de Concurrencia: `Send` y `Sync`
A diferencia de otros conceptos de concurrencia, `Send` y `Sync` son **traits marcadores (marker traits)** del compilador:
*   **`Send`:** Indica que la propiedad de un tipo puede transferirse a otro hilo de ejecución de manera segura. Casi todos los tipos de Rust son `Send`, excepto punteros no seguros o `Rc<T>` (ya que su contador no atómico podría corromperse).
*   **`Sync`:** Indica que es seguro acceder a referencias del mismo tipo desde múltiples hilos concurrentemente. Un tipo `T` es `Sync` si y solo si la referencia `&T` es `Send`.
*   **Garantías Automáticas:** Estos traits son auto-implementados por el compilador. Si una estructura de datos contiene únicamente campos que son `Send` y `Sync`, la estructura se marca automáticamente como tal.

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### La Guardia de Exclusión Mutua (`MutexGuard`)
Cuando llamas exitosamente a `.lock()`, no obtienes una referencia directa a los datos; obtienes un tipo de datos especial llamado `MutexGuard`:
*   Es un puntero inteligente que implementa `Deref` apuntando a los datos protegidos.
*   Implementa `Drop`. Al salir de ámbito, su destructor ejecuta la llamada a nivel de sistema operativo para liberar el bloqueo del semáforo. Esto previene bugs comunes de C/C++ donde un desarrollador olvida liberar un Mutex en una ramificación de retorno anticipado.

---

### Conteo Atómico de Referencias (`Arc<T>` vs. `Rc<T>`)
¿Por qué existen `Rc` y `Arc` por separado si hacen la misma función?

*   **`Rc<T>`:** Utiliza operaciones aritméticas ordinarias de CPU para incrementar y decrementar los contadores en memoria. Estas operaciones no coordinan accesos concurrentes de diferentes núcleos de CPU, lo que provocaría que dos hilos intentando clonar al mismo tiempo corrompan el contador, causando fugas de memoria o liberaciones prematuras (*double-free*).
*   **`Arc<T>`:** Utiliza **instrucciones atómicas a nivel de hardware** (instrucciones de CPU especializadas como *Fetch-and-Add* o *Compare-and-Swap*). Estas instrucciones coordinan físicamente los núcleos del procesador en el bus del hardware.
    *   **Coste de Rendimiento:** Las instrucciones atómicas son significativamente más costosas en ciclos de reloj de CPU que la aritmética común debido a la necesidad de sincronizar caches de CPU. Por ello, si tu código corre en un único hilo, debes utilizar `Rc` para maximizar la velocidad, y reservar `Arc` estrictamente para sistemas concurrentes.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Sincronización Concurrente en Rust

| Técnica / Tipo | Caso de Uso Recomendado | Ventajas | Desventajas |
| :--- | :--- | :--- | :--- |
| **Canales MPSC** | Flujos de datos lineales (pipeline), paso de tareas a trabajadores. | Sin estado compartido directo, muy limpio de razonar. | Puede requerir clonar muchos transmisores, copia de datos. |
| **`Arc<Mutex<T>>`** | Modificación directa de estructuras complejas (bases de datos en memoria). | Acceso directo a datos compartidos en sitio. | Peligro potencial de Deadlocks, coste de bloqueo. |
| **`std::sync::RwLock<T>`**| Sistemas con alta tasa de lecturas y muy pocas escrituras. | Múltiples lectores concurrentes al mismo tiempo. | Más costoso de bloquear en escrituras que un Mutex simple. |

---

### Errores Comunes de Compilación y Ejecución

#### 1. Deadlock al solicitar bloqueos concurrentes cruzados
Si el hilo 1 adquiere el bloqueo A e intenta adquirir B, mientras el hilo 2 adquiere B e intenta adquirir A:
*   **Síntoma:** El programa se congela indefinidamente consumiendo 0% de CPU.
*   ✔️ **Solución A:** Asegurar que todos los hilos siempre soliciten los bloqueos en el mismo orden físico exacto.
*   ✔️ **Solución B:** Mantener los bloqueos activos por el menor tiempo posible (usando bloques de llaves `{}` para desechar la guardia antes de solicitar un segundo bloqueo).

#### 2. Intentar pasar un `Rc` a través de un hilo secundario
❌ **Código Erróneo:**
```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let r = Rc::new(5);
    let r_clon = Rc::clone(&r);
    thread::spawn(move || {
        println!("{}", r_clon); // Error: Rc no se puede transferir entre hilos
    });
}
```
*   **Mensaje de Error:** `error[E0277]: 'Rc<i32>' cannot be sent between threads safely; the trait 'Send' is not implemented`
*   ✔️ **Solución:** Reemplazar `Rc` por `Arc`.
