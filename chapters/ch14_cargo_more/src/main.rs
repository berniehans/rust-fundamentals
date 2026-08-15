// ch14_cargo_more - Demostración Educativa de Cargo, Crates.io y Documentación Avanzada
// Este archivo profundiza en rustdoc, doctests, reexportación ergonómica de APIs y perfiles de compilación.

//! # Crate Demostrativo del Capítulo 14
//!
//! Este crate expone utilidades matemáticas y muestra cómo estructurar
//! documentación de nivel profesional y APIs públicas optimizadas para Crates.io.

pub mod matematicas {
    pub mod algebra {
        /// Calcula el factorial de un entero sin signo de forma iterativa y segura.
        ///
        /// # Ejemplos
        ///
        /// ```
        /// let resultado = 5 * 4 * 3 * 2 * 1;
        /// assert_eq!(resultado, 120);
        /// ```
        ///
        /// # Pánicos
        ///
        /// Provoca un pánico si el cálculo causa un desbordamiento numérico en `u64`.
        pub fn factorial(n: u64) -> u64 {
            let mut acc: u64 = 1;
            for i in 1..=n {
                acc = acc
                    .checked_mul(i)
                    .expect("Desbordamiento aritmético en factorial");
            }
            acc
        }
    }

    pub mod series {
        /// Genera el enésimo número en la sucesión de Fibonacci.
        pub fn fibonacci(n: u32) -> u64 {
            if n == 0 {
                return 0;
            }
            let mut a = 0;
            let mut b = 1;
            for _ in 1..n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

// Reexportación de símbolos para aplanar la API pública (Convenience API)
pub use crate::matematicas::algebra::factorial;
pub use crate::matematicas::series::fibonacci;

fn main() {
    println!("=== CAPÍTULO 14: MÁS SOBRE CARGO Y CRATES.IO ===");

    demostrar_api_reexportada();
    demostrar_perfiles_cargo();

    println!("\n¡Capítulo 14 ejecutado con éxito!");
}

fn demostrar_api_reexportada() {
    println!("\n--- 1. API PÚBLICA APLANADA CON PUB USE ---");

    // Gracias a la reexportación no es necesario escribir la ruta profunda
    // 'crate::matematicas::algebra::factorial'
    let fact_5 = factorial(5);
    let fib_10 = fibonacci(10);

    println!("Factorial de 5 (reexportado): {fact_5}");
    println!("Fibonacci número 10 (reexportado): {fib_10}");
}

fn demostrar_perfiles_cargo() {
    println!("\n--- 2. PERFILES DE COMPILACIÓN Y WORKSPACES ---");
    println!("Comandos clave de Cargo demostrados en este capítulo:");
    println!("  1. Generar documentación HTML y abrirla en navegador: cargo doc --open");
    println!("  2. Compilar con optimizaciones agresivas de LLVM:     cargo build --release");
    println!("  3. Instalar herramientas binarias locales:             cargo install --path .");
    println!("  4. Verificar que la documentación compila sin errores: cargo test --doc");
}
