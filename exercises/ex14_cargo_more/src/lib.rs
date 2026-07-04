//! # ex14_cargo_more - Biblioteca Matemática y Utilidades de Cargo
//!
//! Esta biblioteca de ejemplo demuestra las capacidades avanzadas de documentación
//! de crates (`//!`) e ítems (`///`), perfiles de compilación y pruebas integradas (Doctests).
//! Provee funciones matemáticas elementales de sistemas.

/// Calcula el factorial de un número entero sin signo de forma iterativa y segura.
///
/// # Parámetros
/// * `n` - El número entero a evaluar.
///
/// # Retorno
/// * `u64` - El factorial resultante.
///
/// # Ejemplos
/// ```
/// use ex14_cargo_more::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    let mut resultado = 1;
    for i in 1..=n {
        resultado *= i;
    }
    resultado
}

/// Calcula el n-ésimo número de la sucesión de Fibonacci de manera eficiente.
///
/// # Parámetros
/// * `n` - Posición en la secuencia (0-indexada).
///
/// # Retorno
/// * `u64` - El valor de Fibonacci en dicha posición.
///
/// # Ejemplos
/// ```
/// use ex14_cargo_more::fibonacci;
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(6), 8);
/// ```
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
