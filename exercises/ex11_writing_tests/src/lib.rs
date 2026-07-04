// exercises/ex11_writing_tests/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 11.

/// Valida la validez de un número de tarjeta de crédito utilizando el Algoritmo de Luhn (módulo 10).
///
/// # Reglas
/// 1. Debe contener exactamente 16 dígitos numéricos.
/// 2. Debe superar el algoritmo de Luhn: comenzando desde el penúltimo dígito y moviéndose hacia la izquierda,
///    duplica el valor de cada segundo dígito. Si el resultado es mayor a 9, resta 9.
///    Suma todos los dígitos. Si la suma total es divisible por 10, la tarjeta es válida.
///
/// # Parámetros
/// * `numero` - Cadena conteniendo el número de tarjeta. Puede incluir espacios que se limpiarán.
///
/// # Retorno
/// * `Ok(())` si la tarjeta es estructuralmente válida.
/// * `Err(&'static str)` con el motivo si es inválida.
///
/// # Ejemplos
/// ```
/// use ex11_writing_tests::validar_tarjeta_credito;
/// // Ejemplo de número de tarjeta válido por Luhn
/// assert!(validar_tarjeta_credito("49927398716").is_ok());
/// ```
pub fn validar_tarjeta_credito(numero: &str) -> Result<(), &'static str> {
    // 1. Limpiar espacios en blanco
    let limpia: String = numero.chars().filter(|c| !c.is_whitespace()).collect();

    // 2. Verificar que solo contenga dígitos numéricos
    if limpia.is_empty() || !limpia.chars().all(|c| c.is_ascii_digit()) {
        return Err("El número contiene caracteres no permitidos.");
    }

    // 3. Algoritmo de Luhn
    let mut suma = 0;
    let mut duplicar = false;

    // Iterar en reversa
    for c in limpia.chars().rev() {
        let mut digito = c.to_digit(10).unwrap() as i32;

        if duplicar {
            digito *= 2;
            if digito > 9 {
                digito -= 9;
            }
        }

        suma += digito;
        duplicar = !duplicar;
    }

    if suma % 10 == 0 {
        Ok(())
    } else {
        Err("Número de tarjeta inválido según algoritmo de Luhn.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarjeta_valida_luhn() {
        // Tarjeta de prueba válida por Luhn
        assert!(validar_tarjeta_credito("49927398716").is_ok());
        assert!(validar_tarjeta_credito("4992 7398 716").is_ok()); // Con espacios
    }

    #[test]
    fn test_tarjeta_invalida_luhn() {
        assert_eq!(
            validar_tarjeta_credito("49927398717"),
            Err("Número de tarjeta inválido según algoritmo de Luhn.")
        );
    }

    #[test]
    fn test_tarjeta_caracteres_invalidos() {
        assert_eq!(
            validar_tarjeta_credito("4992-7398-716"),
            Err("El número contiene caracteres no permitidos.")
        );
        assert_eq!(
            validar_tarjeta_credito("4992abc9871"),
            Err("El número contiene caracteres no permitidos.")
        );
    }

    #[test]
    fn test_tarjeta_vacia() {
        assert_eq!(
            validar_tarjeta_credito(""),
            Err("El número contiene caracteres no permitidos.")
        );
    }
}
