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

    println!("\n--- 1. INICIALIZACIÓN DEL THREADPOOL INDUSTRIAL (4 HILOS) ---");
    let pool = ThreadPool::nuevo(4);

    println!("\n--- 2. SIMULACIÓN DE PETICIONES HTTP CONCURRENTES ---");

    // Simulamos 8 peticiones HTTP concurrentes entrando al servidor
    for i in 1..=8 {
        pool.ejecutar(move || {
            let tiempo_proceso = if i % 2 == 0 { 50 } else { 20 };
            println!(
                "    -> Procesando solicitud HTTP #{} (duración simulada: {}ms)",
                i, tiempo_proceso
            );
            thread::sleep(Duration::from_millis(tiempo_proceso));
        });
    }

    // Pequeña pausa para permitir que los trabajadores procesen la carga simulada
    thread::sleep(Duration::from_millis(200));

    println!("\n--- 3. DETALLES DE INTEGRACIÓN TCP ---");
    println!("Para enlazar este ThreadPool a sockets de red TCP reales:");
    println!("  let listener = std::net::TcpListener::bind(\"127.0.0.1:7878\").unwrap();");
    println!("  for stream in listener.incoming() {{");
    println!("      let stream = stream.unwrap();");
    println!("      pool.ejecutar(|| {{ procesar_stream_tcp(stream); }});");
    println!("  }}");

    // Al salir de main(), pool sale de ámbito y se invoca automáticamente Drop
}
