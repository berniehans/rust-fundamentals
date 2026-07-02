// ex02_guessing_game - Ejercicios Prácticos Extra
// Este archivo contiene retos de codificación para practicar los conceptos del Capítulo 2.

use std::cmp::Ordering;

/// # Desafío 1: Comparar Suposición
/// Implementa una función que reciba la suposición del usuario (`suposicion`) y el número secreto (`secreto`),
/// y compare ambos valores usando `std::cmp::Ordering` y pattern matching. Debe retornar una cadena estática (`&'static str`):
/// - "¡Muy pequeño!" si la suposición es menor que el número secreto.
/// - "¡Muy grande!" si la suposición es mayor que el número secreto.
/// - "¡Ganaste!" si ambos números son iguales.
pub fn comparar_suposicion(suposicion: u32, secreto: u32) -> &'static str {
    match suposicion.cmp(&secreto) {
        Ordering::Less => "¡Muy pequeño!",
        Ordering::Greater => "¡Muy grande!",
        Ordering::Equal => "¡Ganaste!",
    }
}

/// # Desafío 2: Validar Entrada de Texto
/// Implementa una función que tome una referencia a una cadena (`entrada`), limpie los espacios en blanco
/// a los lados e intente parsearlo como un entero de 32 bits sin signo (`u32`).
/// - Si el parseo es exitoso, retorna `Ok(numero)`.
/// - Si el parseo falla (por ejemplo, si contiene texto no numérico), retorna un `Err` con el mensaje:
///   "Por favor, introduce un número válido."
pub fn validar_entrada(entrada: &str) -> Result<u32, &'static str> {
    match entrada.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Por favor, introduce un número válido."),
    }
}

/// # Desafío 3: Verificar Rango Inclusivo
/// En el juego de adivinanza, generamos un número en un rango inclusivo como `1..=100`.
/// Implementa una función que verifique si un número dado está dentro de un rango inclusivo especificado
/// por un mínimo y un máximo. Retorna `true` si está dentro, o `false` en caso contrario.
pub fn esta_en_rango(numero: u32, min: u32, max: u32) -> bool {
    (min..=max).contains(&numero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparar_suposicion() {
        assert_eq!(comparar_suposicion(50, 70), "¡Muy pequeño!");
        assert_eq!(comparar_suposicion(90, 70), "¡Muy grande!");
        assert_eq!(comparar_suposicion(70, 70), "¡Ganaste!");
    }

    #[test]
    fn test_validar_entrada_exitosa() {
        assert_eq!(validar_entrada("42\n"), Ok(42));
        assert_eq!(validar_entrada("  100  \r\n"), Ok(100));
        assert_eq!(validar_entrada("0"), Ok(0));
    }

    #[test]
    fn test_validar_entrada_fallida() {
        assert_eq!(validar_entrada("hola"), Err("Por favor, introduce un número válido."));
        assert_eq!(validar_entrada("42a"), Err("Por favor, introduce un número válido."));
        assert_eq!(validar_entrada(""), Err("Por favor, introduce un número válido."));
    }

    #[test]
    fn test_esta_en_rango() {
        assert!(esta_en_rango(50, 1, 100));
        assert!(esta_en_rango(1, 1, 100));
        assert!(esta_en_rango(100, 1, 100));
        assert!(!esta_en_rango(0, 1, 100));
        assert!(!esta_en_rango(101, 1, 100));
    }
}
