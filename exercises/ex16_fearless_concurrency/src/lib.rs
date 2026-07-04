// exercises/ex16_fearless_concurrency/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 16.

use std::sync::mpsc;
use std::thread;

/// Suma de manera paralela los elementos de una porción (slice) dividiendo el trabajo en hilos.
///
/// # Ejemplos
/// ```
/// use ex16_fearless_concurrency::suma_paralela;
/// let datos = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// assert_eq!(suma_paralela(&datos, 2), 36);
/// ```
pub fn suma_paralela(datos: &[i32], num_hilos: usize) -> i32 {
    if datos.is_empty() || num_hilos == 0 {
        return 0;
    }

    let tamaño_chunk = (datos.len() + num_hilos - 1) / num_hilos;

    thread::scope(|s| {
        let mut manejadores = Vec::new();

        for chunk in datos.chunks(tamaño_chunk) {
            // Pasamos la porción de lectura inmutable directamente al closure del hilo del scope
            let manejador = s.spawn(move || {
                let suma: i32 = chunk.iter().sum();
                suma
            });
            manejadores.push(manejador);
        }

        let mut suma_total = 0;
        for manejador in manejadores {
            suma_total += manejador.join().unwrap();
        }

        suma_total
    })
}

/// Simula un pipeline de procesamiento utilizando canales de comunicación (`mpsc`).
/// Un hilo productor envía los números, un hilo procesador los eleva al cuadrado,
/// y el hilo principal recolecta los resultados.
///
/// # Ejemplos
/// ```
/// use ex16_fearless_concurrency::pipeline_cuadrados;
/// let datos = vec![1, 2, 3];
/// assert_eq!(pipeline_cuadrados(datos), vec![1, 4, 9]);
/// ```
pub fn pipeline_cuadrados(entrada: Vec<i32>) -> Vec<i32> {
    let (tx_prod, rx_proc) = mpsc::channel();
    let (tx_proc, rx_main) = mpsc::channel();

    // Hilo 1: Productor
    thread::spawn(move || {
        for num in entrada {
            tx_prod.send(num).unwrap();
        }
    });

    // Hilo 2: Procesador
    thread::spawn(move || {
        while let Ok(num) = rx_proc.recv() {
            tx_proc.send(num * num).unwrap();
        }
    });

    // Hilo principal: Recolector
    let mut resultados = Vec::new();
    while let Ok(cuadrado) = rx_main.recv() {
        resultados.push(cuadrado);
    }

    resultados
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_paralela() {
        let datos = vec![10; 100]; // 100 dieces = 1000
        assert_eq!(suma_paralela(&datos, 4), 1000);
        assert_eq!(suma_paralela(&datos, 1), 1000);
    }

    #[test]
    fn test_pipeline_cuadrados() {
        let v = vec![2, 4, 6];
        assert_eq!(pipeline_cuadrados(v), vec![4, 16, 36]);
    }
}
