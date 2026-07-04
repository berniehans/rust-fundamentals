// exercises/ex20_web_server/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 20.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Un pool de hilos de ejecución para procesar múltiples tareas de forma concurrente.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Inicializa un nuevo ThreadPool.
    ///
    /// # Panics
    /// La función causará un pánico si el tamaño especificado es 0.
    pub fn nuevo(tamaño: usize) -> Self {
        assert!(tamaño > 0, "El tamaño del ThreadPool debe ser mayor que 0.");

        let (tx, rx) = mpsc::channel();
        // Envolvemos el receptor en Arc y Mutex para compartirlo de forma segura entre hilos
        let receiver = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(tamaño);

        for id in 0..tamaño {
            workers.push(Worker::nuevo(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(tx),
        }
    }

    /// Encola una tarea closure para ser ejecutada asíncronamente en el pool.
    pub fn ejecutar<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Implementa la destrucción limpia (Graceful Shutdown) del pool de hilos.
    fn drop(&mut self) {
        // Cerramos el canal enviando None (al descartar el sender)
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(hilo) = worker.thread.take() {
                hilo.join().unwrap();
            }
        }
    }
}

/// Trabajador interno que gestiona un hilo y escucha en la cola de tareas del pool.
struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn nuevo(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // Obtenemos la tarea de la cola adquiriendo el lock del mutex de forma segura
            let mensaje = receiver.lock().unwrap().recv();

            match mensaje {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    // El canal se ha cerrado: salimos del bucle limpiamente
                    break;
                }
            }
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_thread_pool_ejecucion_tareas() {
        let pool = ThreadPool::nuevo(3);
        let contador = Arc::new(AtomicUsize::new(0));

        // Encolamos 6 tareas sencillas que incrementan el contador
        for _ in 0..6 {
            let contador_clone = Arc::clone(&contador);
            pool.ejecutar(move || {
                contador_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Forzamos la destrucción del pool para esperar a que terminen todos los hilos
        drop(pool);

        assert_eq!(contador.load(Ordering::SeqCst), 6);
    }

    #[test]
    #[should_panic(expected = "El tamaño del ThreadPool debe ser mayor que 0.")]
    fn test_thread_pool_tamaño_cero_panic() {
        ThreadPool::nuevo(0);
    }
}
