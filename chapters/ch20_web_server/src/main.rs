// ch20_web_server - Demostración Educativa del Proyecto Final: Servidor Web Multihilo
// Este archivo implementa una arquitectura completa de ThreadPool concurrente, paso de mensajes
// con Arc<Mutex<mpsc::Receiver>>, ejecución paralela de tareas y apagado elegante (Graceful Shutdown).

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// 1. Definición del tipo Job: envoltorio en Heap para un closure ejecutable una sola vez, seguro entre hilos
type Job = Box<dyn FnOnce() + Send + 'static>;

// 2. Estructura ThreadPool: gestiona un conjunto fijo de hilos trabajadores (Workers)
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Crea un nuevo ThreadPool con un número fijo de hilos trabajadores.
    ///
    /// # Pánicos
    ///
    /// Provoca un pánico si el tamaño es menor o igual a 0.
    pub fn nuevo(tamano: usize) -> ThreadPool {
        assert!(
            tamano > 0,
            "El tamaño del ThreadPool debe ser mayor que cero."
        );

        let (sender, receiver) = mpsc::channel();

        // Arc (Atomic Reference Counting) + Mutex (Mutual Exclusion) permite que múltiples
        // hilos trabajadores compartan el acceso al receptor de forma sincronizada y segura.
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

    /// Encola una tarea en el canal para que cualquier hilo libre del pool la ejecute.
    pub fn ejecutar<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(ref sender) = self.sender {
            sender
                .send(job)
                .expect("Error al enviar la tarea al ThreadPool.");
        }
    }
}

// 3. Implementación de Graceful Shutdown (Apagado Elegante) mediante el trait Drop
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("\n[ThreadPool]: Iniciando apagado elegante (Graceful Shutdown)...");

        // Al liberar el sender del canal (drop explícito), el canal se cierra.
        // Los receivers de los hilos detectan el cierre y terminan sus bucles de escucha.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!(
                "[ThreadPool]: Esperando a que el trabajador {} finalice...",
                worker.id
            );

            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect("Error al unir (join) el hilo del trabajador.");
            }
        }

        println!("[ThreadPool]: Todos los trabajadores han finalizado limpiamente.");
    }
}

// 4. Estructura Worker: representa un hilo de trabajo individual en ejecución
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn nuevo(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Adquirimos el lock del mutex para extraer la siguiente tarea de la cola
                let mensaje = receiver.lock().unwrap().recv();

                match mensaje {
                    Ok(job) => {
                        println!(
                            "  [Trabajador {}]: Tarea recibida. Iniciando procesamiento...",
                            id
                        );
                        job();
                        println!("  [Trabajador {}]: Tarea completada.", id);
                    }
                    Err(_) => {
                        // El canal se cerró: salimos del bucle limpiamente
                        println!(
                            "  [Trabajador {}]: Canal desconectado. Finalizando hilo de ejecución.",
                            id
                        );
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    println!("=== CAPÍTULO 20: PROYECTO FINAL (SERVIDOR WEB MULTIHILO) ===");

    // --- Parte 1: Demostración rápida del ThreadPool con tareas simuladas ---
    println!("\n--- 1. DEMOSTRACIÓN DEL THREADPOOL (TAREAS SIMULADAS) ---");
    {
        let pool = ThreadPool::nuevo(4);

        for i in 1..=8 {
            pool.ejecutar(move || {
                let tiempo_proceso = if i % 2 == 0 { 50 } else { 20 };
                println!(
                    "    -> Procesando tarea #{} (duración simulada: {}ms)",
                    i, tiempo_proceso
                );
                thread::sleep(Duration::from_millis(tiempo_proceso));
            });
        }

        thread::sleep(Duration::from_millis(200));
        // Al salir de este bloque, pool invoca Drop → Graceful Shutdown
    }

    // --- Parte 2: Servidor HTTP real con TcpListener ---
    println!("\n--- 2. SERVIDOR HTTP REAL (TCP MULTIHILO) ---");
    println!("Escuchando en http://127.0.0.1:7878");
    println!("El servidor atenderá 4 peticiones y luego se apagará elegantemente.");
    println!("Prueba con: curl http://127.0.0.1:7878");
    println!("       o:   curl http://127.0.0.1:7878/sleep  (simula carga pesada)\n");

    let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|err| {
        eprintln!("No se pudo enlazar al puerto 7878: {err}");
        eprintln!("(Si el puerto está ocupado, otro proceso lo está usando.)");
        std::process::exit(1);
    });

    let pool = ThreadPool::nuevo(4);

    // Atendemos solo las primeras 4 conexiones para que el ejemplo termine limpiamente
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        pool.ejecutar(|| {
            manejar_conexion(stream);
        });
    }

    // Al salir de main(), pool sale de ámbito y se invoca automáticamente Drop → Graceful Shutdown
}

/// Procesa una conexión TCP entrante: lee la petición HTTP y responde con HTML.
fn manejar_conexion(mut stream: std::net::TcpStream) {
    use std::io::{BufRead, BufReader, Write};

    let buf_reader = BufReader::new(&stream);

    // Leemos la primera línea de la petición HTTP (ej: "GET / HTTP/1.1")
    let linea_peticion = buf_reader
        .lines()
        .next()
        .unwrap_or(Ok(String::new()))
        .unwrap_or_default();

    println!("  [Petición recibida]: {linea_peticion}");

    let (linea_estado, cuerpo_html) = match linea_peticion.as_str() {
        "GET / HTTP/1.1" => {
            let html = "\
<!DOCTYPE html>
<html lang=\"es\">
<head><meta charset=\"utf-8\"><title>Rust Web Server</title></head>
<body>
<h1>🦀 ¡Hola desde el servidor web multihilo de Rust!</h1>
<p>Este servidor fue construido con <code>std::net::TcpListener</code> y un <code>ThreadPool</code> personalizado.</p>
<p>Prueba <a href=\"/sleep\">/sleep</a> para ver la concurrencia en acción.</p>
</body>
</html>";
            ("HTTP/1.1 200 OK", html)
        }
        "GET /sleep HTTP/1.1" => {
            // Simulamos una petición pesada que tarda 2 segundos
            thread::sleep(Duration::from_secs(2));
            let html = "\
<!DOCTYPE html>
<html lang=\"es\">
<head><meta charset=\"utf-8\"><title>Respuesta lenta</title></head>
<body>
<h1>⏳ Respuesta lenta completada</h1>
<p>Esta petición tardó 2 segundos intencionalmente para demostrar que otros hilos del pool siguen atendiendo peticiones mientras tanto.</p>
</body>
</html>";
            ("HTTP/1.1 200 OK", html)
        }
        _ => {
            let html = "\
<!DOCTYPE html>
<html lang=\"es\">
<head><meta charset=\"utf-8\"><title>404</title></head>
<body><h1>404 - Página no encontrada</h1></body>
</html>";
            ("HTTP/1.1 404 NOT FOUND", html)
        }
    };

    let longitud = cuerpo_html.len();
    let respuesta = format!(
        "{linea_estado}\r\nContent-Length: {longitud}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{cuerpo_html}"
    );

    stream
        .write_all(respuesta.as_bytes())
        .unwrap_or_else(|err| {
            eprintln!("  [Error al escribir respuesta]: {err}");
        });
}
