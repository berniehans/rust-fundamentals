// exercises/ex04_understanding_ownership/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 04.

/// Invierte los caracteres de un objeto `String` directamente en su celda de memoria en el Heap (in-place).
/// Demuestra el uso de préstamos mutables (`&mut String`) para editar recursos sin tomar propiedad.
///
/// # Parámetros
/// * `s` - Referencia mutable a la cadena a invertir.
///
/// # Ejemplos
/// ```
/// use ex04_understanding_ownership::invertir_cadena_in_place;
/// let mut texto = String::from("hola");
/// invertir_cadena_in_place(&mut texto);
/// assert_eq!(texto, "aloh");
/// ```
pub fn invertir_cadena_in_place(s: &mut String) {
    // Convertimos la cadena temporalmente a un vector de caracteres para manipular índices seguros
    let mut caracteres: Vec<char> = s.chars().collect();
    let mut izq = 0;
    let mut der = caracteres.len().saturating_sub(1);

    while izq < der {
        caracteres.swap(izq, der);
        izq += 1;
        der = der.saturating_sub(1);
    }

    // Volvemos a escribir la cadena origen usando el vector de caracteres invertido
    *s = caracteres.into_iter().collect();
}

/// Extrae la primera palabra encontrada en un slice de cadena.
/// Demuestra la anatomía de los slices (`&str`) y el comportamiento del borrow checker al enlazar
/// la vida útil del slice de retorno con la vida útil de la cadena original de lectura.
///
/// # Parámetros
/// * `s` - Slice de cadena de entrada.
///
/// # Retorno
/// * `&str` - Slice correspondiente a la primera palabra de la cadena.
///
/// # Ejemplos
/// ```
/// use ex04_understanding_ownership::extraer_primera_palabra;
/// let frase = String::from("Rust es increible");
/// let palabra = extraer_primera_palabra(&frase);
/// assert_eq!(palabra, "Rust");
/// ```
pub fn extraer_primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invertir_cadena_in_place() {
        let mut s = String::from("hola mundo");
        invertir_cadena_in_place(&mut s);
        assert_eq!(s, "odnum aloh");

        let mut vacia = String::new();
        invertir_cadena_in_place(&mut vacia);
        assert_eq!(vacia, "");
    }

    #[test]
    fn test_extraer_primera_palabra() {
        let s = "Hola querido mundo";
        assert_eq!(extraer_primera_palabra(s), "Hola");

        let s_sola = "Palabra";
        assert_eq!(extraer_primera_palabra(s_sola), "Palabra");

        let vacia = "";
        assert_eq!(extraer_primera_palabra(vacia), "");
    }
}
