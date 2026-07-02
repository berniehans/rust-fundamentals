//! ch16_fearless_concurrency/src/main.rs
//!
//! Este subproyecto sirve como bitácora de estudio para el Capítulo 16: "Fearless Concurrency" (Concurrencia sin miedo).
//! Aquí se demuestran los tres enfoques principales de concurrencia en Rust de manera didáctica:
//! 1. Creación básica de hilos (threads) con paso de datos mediante clausuras (`spawn` y `join`).
//! 2. Transferencia de mensajes entre hilos usando canales (MPSC: Multiple Producer, Single Consumer).
//! 3. Concurrencia de estado compartido usando exclusión mutua (`Mutex`) y contadores de referencias seguros para hilos (`Arc`).

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("============================================================");
    println!("   BITÁCORA DE APRENDIZAJE: CAPÍTULO 16 - CONCURRENCIA       ");
    println!("============================================================\n");

    demo_hilos_basicos();
    println!("\n------------------------------------------------------------\n");
    demo_canales_mpsc();
    println!("\n------------------------------------------------------------\n");
    demo_estado_compartido();
    println!("\n============================================================");
}

/// Demuestra la creación básica de un hilo con thread::spawn y la sincronización con join.
fn demo_hilos_basicos() {
    println!("[1] DEMOSTRACIÓN: Hilos Básicos y Join Handles");

    // Creamos un hilo secundario
    let handle = thread::spawn(|| {
        for i in 1..6 {
            println!("   [Hilo Secundario] Mensaje número {i}...");
            thread::sleep(Duration::from_millis(50));
        }
        "¡Hilo secundario terminado con éxito!"
    });

    // Mientras el hilo corre, el hilo principal puede seguir ejecutando
    for i in 1..3 {
        println!("   [Hilo Principal] Trabajando en paso {i}...");
        thread::sleep(Duration::from_millis(30));
    }

    println!("   [Hilo Principal] Esperando a que el hilo secundario termine...");
    
    // `.join()` bloquea el hilo actual hasta que el hilo representado por el handle finalice.
    // Retorna un Result con el valor de retorno de la clausura o un error si el hilo hizo panic.
    match handle.join() {
        Ok(resultado) => println!("   [Hilo Principal] Resultado recibido del hilo: {resultado}"),
        Err(e) => println!("   [Hilo Principal] Ocurrió un error (panic) en el hilo: {e:?}"),
    }
}

/// Demuestra la comunicación entre hilos por paso de mensajes (Channels).
/// MPSC = Multiple Producer, Single Consumer (Múltiples Productores, Un Solo Consumidor).
fn demo_canales_mpsc() {
    println!("[2] DEMOSTRACIÓN: Canales (Message Passing)");

    // Creamos un canal. tx es el transmisor (Sender) y rx es el receptor (Receiver)
    let (tx, rx) = mpsc::channel();

    // Para demostrar múltiples productores, clonamos el transmisor
    let tx1 = tx.clone();

    // Hilo Productor 1
    thread::spawn(move || {
        let mensajes = vec![
            String::from("Hola"),
            String::from("desde"),
            String::from("el Hilo Productor 1"),
        ];

        for mensaje in mensajes {
            // send() transfiere la propiedad del mensaje al receptor.
            // Si el receptor ha sido liberado, retorna un Err.
            if let Err(e) = tx1.send(mensaje) {
                eprintln!("Error al enviar desde Hilo 1: {e}");
                break;
            }
            thread::sleep(Duration::from_millis(80));
        }
    });

    // Hilo Productor 2
    thread::spawn(move || {
        let mensajes = vec![
            String::from("Saludos"),
            String::from("más"),
            String::from("del Hilo Productor 2"),
        ];

        for mensaje in mensajes {
            if let Err(e) = tx.send(mensaje) {
                eprintln!("Error al enviar desde Hilo 2: {e}");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // El receptor principal lee los mensajes del canal.
    // Tratar a 'rx' como un iterador nos permite recibir mensajes de forma segura
    // hasta que todos los transmisores (tx y tx1) salgan de alcance (scope).
    println!("   [Hilo Principal] Escuchando mensajes del canal...");
    for mensaje_recibido in rx {
        println!("   [Receptor] Recibido: \"{mensaje_recibido}\"");
    }
    println!("   [Hilo Principal] El canal se ha cerrado (todos los emisores finalizaron).");
}

/// Demuestra el estado compartido y la exclusión mutua usando Arc y Mutex.
/// Arc = Atomic Reference Counted (Contador de Referencias Atómico, para compartir propiedad entre hilos).
/// Mutex = Mutual Exclusion (Exclusión Mutua, para garantizar acceso exclusivo de escritura/lectura).
fn demo_estado_compartido() {
    println!("[3] DEMOSTRACIÓN: Estado Compartido (Arc y Mutex)");

    // Creamos un contador protegido por un Mutex, envuelto en un Arc
    // para poder compartirlo de forma segura entre múltiples hilos.
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("   [Hilo Principal] Lanzando 10 hilos para incrementar el contador común...");

    for i in 1..11 {
        // Clonamos el Arc (incrementa el contador de referencias atómico, no copia los datos)
        let contador_clon = Arc::clone(&contador);

        let handle = thread::spawn(move || {
            // lock() bloquea el mutex. Retorna un MutexGuard envuelto en Result.
            // Si otro hilo hizo panic mientras sostenía el lock, retorna un Err (poisoned).
            // El MutexGuard implementa Deref para acceder al valor interno y Drop
            // para liberar el bloqueo automáticamente al salir de este scope.
            let mut datos = contador_clon.lock().unwrap();
            *datos += 1;
            println!("      -> [Hilo #{i:2}] Incrementó el contador. Valor actual: {datos}");
        });
        handles.push(handle);
    }

    // Esperamos a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    // Leemos el resultado final adquiriendo el lock en el hilo principal
    println!(
        "   [Hilo Principal] Todos los hilos terminaron. Valor final del contador: {}",
        *contador.lock().unwrap()
    );
}
