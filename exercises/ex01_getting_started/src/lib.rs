// ex01_getting_started - Ejercicios Prácticos Extra
// Este archivo contiene retos de codificación básicos para familiarizarse con la sintaxis de Rust.

/// # Desafío 1: Generador de Saludos Dinámicos
/// Implementa una función que tome una referencia a una cadena de texto (&str) con un nombre
/// y retorne un objeto String con un saludo personalizado.
///
/// Ejemplo: "Mundo" -> "¡Hola, Mundo!"
pub fn generar_saludo(nombre: &str) -> String {
    // Usamos la macro format! para construir una String en memoria Heap de forma segura.
    format!("¡Hola, {}!", nombre)
}

/// # Desafío 2: Suma de Enteros
/// Escribe una función que tome dos enteros de 32 bits con signo (i32) y retorne su suma.
/// Demuestra el uso del retorno implícito de Rust (omitir la palabra clave 'return' y el punto y coma final).
pub fn sumar_numeros(a: i32, b: i32) -> i32 {
    // En Rust, la última línea de un bloque es una expresión de retorno si no lleva punto y coma.
    a + b
}

/// # Desafío 3: Comprobación de Paridad
/// Escribe una función que tome un entero de 32 bits con signo (i32) y determine si es par.
/// Retorna true si es par, false en caso contrario.
pub fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

#[cfg(test)]
mod tests {
    // Importamos todas las funciones del módulo padre para poder testearlas.
    use super::*;

    #[test]
    fn test_generar_saludo() {
        assert_eq!(generar_saludo("Rustaceo"), "¡Hola, Rustaceo!");
        assert_eq!(generar_saludo("Mundo"), "¡Hola, Mundo!");
    }

    #[test]
    fn test_sumar_numeros() {
        assert_eq!(sumar_numeros(5, 7), 12);
        assert_eq!(sumar_numeros(-3, 3), 0);
    }

    #[test]
    fn test_es_par() {
        assert!(es_par(4));
        assert!(!es_par(7));
        assert!(es_par(0));
    }
}
