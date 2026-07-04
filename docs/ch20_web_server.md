# Capítulo 20: Proyecto Final (Servidor Web Multihilo)

Este documento proporciona un análisis exhaustivo y de bajo nivel para el desarrollo del proyecto final: un servidor HTTP multihilo de alto rendimiento construido desde cero. Se detallan la manipulación de sockets TCP, el análisis de peticiones y respuestas HTTP, el diseño arquitectónico de un pool de hilos personalizado (ThreadPool) y la prevención de bloqueos mutuos o serialización accidental de tareas concurrentes.

---

## 1. Conceptos Fundamentales (Desde Cero)

### Arquitectura de Servidores de Red
Un servidor web es una aplicación de red que escucha peticiones en un puerto físico determinado, procesa los datos entrantes bajo el protocolo de capa de aplicación HTTP, y devuelve una respuesta estructurada.

En el desarrollo de servidores de red existen tres modelos de concurrencia principales:
1.  **Monohilo (Single-threaded):** El servidor procesa una única solicitud a la vez. Si una petición tarda 5 segundos en leer una base de datos, el resto de usuarios conectados al socket de red experimentan un bloqueo completo (latencia extrema).
2.  **Un hilo por petición (Thread-per-request):** El servidor invoca `thread::spawn` por cada conexión entrante.
    *   **Inconveniente:** Aunque es sencillo, es susceptible a ataques de Denegación de Servicio (DoS). Crear un hilo de ejecución real del sistema operativo consume recursos físicos (aproximadamente 1MB a 8MB de memoria virtual para la pila del hilo). Un atacante podría abrir miles de conexiones simuladas, agotando la RAM del servidor y saturando la CPU con el cambio de contexto (*context switching*).
3.  **Pool de Hilos (Thread Pool):** El servidor pre-asigna un número fijo y controlado de hilos (ej. 4 u 8 hilos según la cantidad de núcleos físicos de la CPU). Las solicitudes entrantes se encolan en un búfer y los hilos libres las procesan secuencialmente. Esto garantiza un consumo de recursos estable y predecible bajo cualquier volumen de carga.

---

## 2. Anatomía y Semántica de la Sintaxis

### 1. Escucha TCP básica (`TcpListener` y `TcpStream`)
La biblioteca estándar proporciona abstracciones de red en `std::net`:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};

fn main() {
    // Escuchamos en la dirección local en el puerto 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // .incoming() devuelve un iterador sobre las conexiones entrantes (TcpStream)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        procesar_conexion(stream);
    }
}

fn procesar_conexion(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // Leemos la primera línea (la línea de petición HTTP)
    let linea_peticion = buf_reader.lines().next().unwrap().unwrap();
    
    // Validamos la petición y preparamos la respuesta
    let (linea_estado, nombre_archivo) = if linea_peticion == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contenido = std::fs::read_to_string(nombre_archivo).unwrap();
    let longitud = contenido.len();

    let respuesta = format!(
        "{linea_estado}\r\nContent-Length: {longitud}\r\n\r\n{contenido}"
    );

    // Escribimos los bytes directamente en la conexión de red
    stream.write_all(respuesta.as_bytes()).unwrap();
}
```

---

### 2. Diseño del Pool de Hilos (`ThreadPool`)
Para procesar las conexiones de forma concurrente sin el peligro de agotar la memoria, implementamos un `ThreadPool` en `src/lib.rs`.

```rust
// src/lib.rs
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// Tipo alias para envolver la tarea (closure) a ejecutar
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn nuevo(tamano: usize) -> ThreadPool {
        assert!(tamano > 0);

        let (sender, receiver) = mpsc::channel();
        
        // Compartimos de forma segura el receptor entre múltiples hilos trabajadores
        // envolviéndolo en Arc (conteo atómico) y Mutex (exclusión mutua)
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(tamano);

        for id in 0..tamano {
            workers.push(Worker::nuevo(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn ejecutar<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
```

#### La Estructura `Worker` (Trabajador)
Un `Worker` es responsable de mantener activo un hilo del sistema operativo ejecutando un bucle infinito que espera tareas del canal:

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn nuevo(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // 1. Bloqueamos el mutex del receptor y esperamos una tarea
            let resultado = receiver.lock().unwrap().recv();

            // 2. Si el canal se cierra, rompemos el bucle para permitir un apagado limpio
            match resultado {
                Ok(job) => {
                    println!("Trabajador {id} ha obtenido una tarea; ejecutando...");
                    job(); // Ejecutamos el closure
                }
                Err(_) => {
                    println!("Trabajador {id} desconectado; apagando hilo.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

---

## 3. Bajo el Capó (Gestión de Memoria y Rendimiento)

### Flujo de Ejecución Multihilo

```
             [ Cliente Web ]
                    | (Petición HTTP)
                    v
             [ TcpListener ]
                    |
                    v (TcpStream)
             [ ThreadPool ] --.ejecutar(job)
                    |
                    v (Cola MPSC)
             [ Arc<Mutex<Receiver>> ]
              /     |      \
             /      v       \  (Competencia por el Mutex)
         [Worker0] [Worker1] [Worker2]
            |
            v (Bloqueo liberado, ejecuta Job)
         [ Hilo de OS ] -> Procesa respuesta -> Envía bytes al socket
```

---

### Evitar la Serialización Accidental (Evitar Retener el Lock)
Uno de los aspectos más sutiles y de bajo nivel al construir un `ThreadPool` es cómo y cuándo se libera el Mutex del receptor del canal.

Considere este código alternativo erróneo para el bucle del `Worker`:
```rust
// CÓDIGO ERRONEO (Serialización del Pool)
impl Worker {
    fn nuevo(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // El bloque 'while let' retiene el bloqueo del Mutex durante TODO el cuerpo del bucle.
            // Por lo tanto, el trabajador actual ejecuta 'job()' sin liberar el Mutex,
            // impidiendo que cualquier otro trabajador intente leer una tarea concurrente.
            while let Ok(job) = receiver.lock().unwrap().recv() {
                job(); 
            }
        });
        // ...
    }
}
```
*   **Problema:** Al usar `while let`, la variable temporal que mantiene el bloqueo del Mutex (`MutexGuard`) vive hasta el final de la iteración del bucle. Esto hace que el servidor se comporte de forma **monohilo**, ya que un solo hilo trabajador acapara el Mutex de tareas mientras ejecuta la petición lenta, bloqueando al resto de trabajadores.
*   **Solución Idiomática:** Utilizar la estructura `let job = receiver.lock().unwrap().recv().unwrap();`. Al asignar la tarea a una variable local separada, el objeto temporal `MutexGuard` devuelto por `.lock()` se destruye inmediatamente al finalizar la sentencia de asignación (en la misma línea), **liberando el Mutex antes de ejecutar la tarea pesada `job()`**. Esto permite que otros hilos accedan al canal concurrentemente.

---

## 4. Cheat Sheet de Sintaxis y Errores Comunes

### Apagado Limpio (Graceful Shutdown)
Para apagar el servidor web de manera controlada (liberando recursos del sistema operativo sin abortar transacciones activas de usuarios), implementamos el trait `Drop` sobre `ThreadPool`:

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 1. Destruimos el transmisor del canal. 
        // Esto provoca que las llamadas .recv() de los trabajadores devuelvan un error.
        drop(self.sender.take());

        // 2. Esperamos a que cada hilo termine su tarea activa y salga del bucle
        for worker in &mut self.workers {
            println!("Apagando trabajador {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

---

### Errores Comunes de Compilación y Ejecución

#### 1. Intento de compartir datos no atómicos a través del pool
Si intentas pasar una variable no-Send (como `Rc`) dentro de la función `.ejecutar()`:
❌ **Código Erróneo:**
```rust
let datos = std::rc::Rc::new(10);
pool.ejecutar(move || {
    println!("{}", datos); // Error: Rc no se puede transferir entre hilos
});
```
*   **Mensaje de Error:** `error[E0277]: 'Rc<i32>' cannot be sent between threads safely`
*   ✔️ **Solución:** Envolver el dato en `Arc` para garantizar que sea seguro de transferir entre los hilos del pool.

#### 2. Mutex Starvation o Bloqueo Mutuo al no cerrar sockets de red
*   **Problema:** Si el hilo procesa una solicitud pero no cierra la conexión (`TcpStream`), el cliente web (navegador) permanece en espera infinita (pantalla de carga girando), consumiendo un hilo del pool de forma indefinida.
*   ✔️ **Solución:** Recordar que `TcpStream` implementa `Drop` y cierra automáticamente la conexión física al salir de ámbito. Asegurarse de no retener referencias de `stream` activas más allá de la finalización del procesamiento de la respuesta HTTP.
