// exercises/ex12_minigrep/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 12.

/// Realiza una búsqueda sensible a mayúsculas y minúsculas (case-sensitive) en un bloque de texto,
/// devolviendo todas las líneas que contengan la palabra de búsqueda.
///
/// # Ejemplos
/// ```
/// use ex12_minigrep::buscar;
/// let query = "duct";
/// let contenido = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
/// assert_eq!(buscar(query, contenido), vec!["safe, fast, productive."]);
/// ```
pub fn buscar<'a>(query: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultados = Vec::new();

    for linea in contenido.lines() {
        if linea.contains(query) {
            resultados.push(linea);
        }
    }

    resultados
}

/// Realiza una búsqueda insensible a mayúsculas y minúsculas (case-insensitive) en un bloque de texto,
/// devolviendo todas las líneas que contengan la palabra de búsqueda.
///
/// # Ejemplos
/// ```
/// use ex12_minigrep::buscar_insensible;
/// let query = "rUsT";
/// let contenido = "\
/// Rust:
/// safe, fast, productive.
/// Trust me.";
/// assert_eq!(buscar_insensible(query, contenido), vec!["Rust:", "Trust me."]);
/// ```
pub fn buscar_insensible<'a>(query: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultados = Vec::new();
    let query_lower = query.to_lowercase();

    for linea in contenido.lines() {
        if linea.to_lowercase().contains(&query_lower) {
            resultados.push(linea);
        }
    }

    resultados
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buscar_sensible() {
        let query = "rapido";
        let contenido = "\
Rust es rapido, seguro
y muy divertido.
No es tan rapido como C, pero es más seguro.";
        
        assert_eq!(
            buscar(query, contenido),
            vec!["Rust es rapido, seguro", "No es tan rapido como C, pero es más seguro."]
        );
    }

    #[test]
    fn test_buscar_insensible() {
        let query = "RuSt";
        let contenido = "\
Rust:
safe, fast, productive.
rustaceo de corazon.";

        assert_eq!(
            buscar_insensible(query, contenido),
            vec!["Rust:", "rustaceo de corazon."]
        );
    }
}
