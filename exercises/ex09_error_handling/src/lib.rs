// exercises/ex09_error_handling/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 09.

use std::fs;
use std::io;
use std::num::ParseIntError;

/// Catálogo de fallos controlados para nuestra aplicación.
#[derive(Debug, PartialEq)]
pub enum ErrorPersonalizado {
    NoEncontrado(String),
    ParseoInvalido(String),
}

impl std::fmt::Display for ErrorPersonalizado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPersonalizado::NoEncontrado(msg) => write!(f, "Archivo no encontrado: {msg}"),
            ErrorPersonalizado::ParseoInvalido(msg) => write!(f, "Formato numérico inválido: {msg}"),
        }
    }
}

impl std::error::Error for ErrorPersonalizado {}

// Implementaciones para permitir la conversión automática mediante el operador `?`

impl From<io::Error> for ErrorPersonalizado {
    fn from(err: io::Error) -> Self {
        ErrorPersonalizado::NoEncontrado(err.to_string())
    }
}

impl From<ParseIntError> for ErrorPersonalizado {
    fn from(err: ParseIntError) -> Self {
        ErrorPersonalizado::ParseoInvalido(err.to_string())
    }
}

/// Lee el contenido de un archivo de configuración de red y extrae un puerto de red (`u16`).
/// Demuestra la propagación y mapeo automático de errores con `?`.
///
/// # Ejemplos
/// ```
/// use ex09_error_handling::{leer_y_parsear_puerto, ErrorPersonalizado};
/// let result = leer_y_parsear_puerto("archivo_inexistente.txt");
/// assert!(result.is_err());
/// ```
pub fn leer_y_parsear_puerto(ruta_archivo: &str) -> Result<u16, ErrorPersonalizado> {
    // 1. Lee la cadena de texto del archivo. Lanza ErrorPersonalizado::NoEncontrado si falla.
    let contenido = fs::read_to_string(ruta_archivo)?;

    // 2. Parsea la cadena. Lanza ErrorPersonalizado::ParseoInvalido si falla.
    let puerto: u16 = contenido.trim().parse()?;

    Ok(puerto)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Usamos la API estándar de std::fs para escribir archivos temporales reales en el espacio de trabajo.

    #[test]
    fn test_leer_y_parsear_puerto_exito() {
        let temp_path = "temp_port_test_ok.txt";
        fs::write(temp_path, "8080").unwrap();

        let result = leer_y_parsear_puerto(temp_path);
        assert_eq!(result, Ok(8080));

        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_leer_y_parsear_puerto_error_lectura() {
        let result = leer_y_parsear_puerto("archivo_que_no_existe_seguro.txt");
        assert!(matches!(result, Err(ErrorPersonalizado::NoEncontrado(_))));
    }

    #[test]
    fn test_leer_y_parsear_puerto_error_parseo() {
        let temp_path = "temp_port_test_fail.txt";
        fs::write(temp_path, "puerto_invalido").unwrap();

        let result = leer_y_parsear_puerto(temp_path);
        assert!(matches!(result, Err(ErrorPersonalizado::ParseoInvalido(_))));

        fs::remove_file(temp_path).unwrap();
    }
}
