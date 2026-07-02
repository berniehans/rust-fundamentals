use rand::Rng; // Importa el trait Rng para poder usar el método gen_range
use std::cmp::Ordering; // Importa el enum Ordering para comparar valores
use std::io; // Importa la biblioteca de entrada/salida

fn main() {
    println!("¡Adivina el número!");

    // Genera un número secreto aleatorio entre 1 y 100 inclusive (1..=100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Bucle infinito para permitir múltiples intentos
    loop {
        println!("Por favor, introduce tu suposición.");

        // Variable mutable para almacenar la entrada del usuario como String
        let mut guess = String::new();

        // Lee la línea de la entrada estándar y escribe el resultado en `guess`
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");

        // Shadowing: Convierte `guess` de String a entero sin signo de 32 bits (u32)
        // Maneja el resultado con un bloque match para evitar abortar en caso de entrada no numérica
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Si fue exitoso, devuelve el número
            Err(_) => {
                // Si hubo error (ej. se introdujo texto), se notifica y se salta a la siguiente iteración
                println!("Por favor, introduce un número válido.");
                continue;
            }
        };

        println!("Tu suposición fue: {guess}");

        // Compara el número del usuario con el número secreto usando pattern matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("¡Muy pequeño!"),
            Ordering::Greater => println!("¡Muy grande!"),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break; // Sale del bucle infinitamente y termina el juego
            }
        }
    }
}
