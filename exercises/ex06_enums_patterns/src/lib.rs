// exercises/ex06_enums_patterns/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 06.

/// Representa el conjunto de comandos válidos recibidos en un socket de red.
#[derive(Debug, Clone, PartialEq)]
pub enum MensajeRed {
    Conectar,
    Desconectar,
    Datos(Vec<u8>),
}

impl MensajeRed {
    /// Evalúa la variante del mensaje y retorna una cadena legible descriptiva.
    ///
    /// # Ejemplos
    /// ```
    /// use ex06_enums_patterns::MensajeRed;
    /// let msg = MensajeRed::Conectar;
    /// assert_eq!(msg.procesar(), "Conexión establecida.");
    /// ```
    pub fn procesar(&self) -> String {
        match self {
            MensajeRed::Conectar => String::from("Conexión establecida."),
            MensajeRed::Desconectar => String::from("Conexión cerrada."),
            MensajeRed::Datos(buffer) => format!("Recibidos {} bytes de datos.", buffer.len()),
        }
    }
}

/// Realiza una suma condicional de dos variables lógicas opcionales.
///
/// # Reglas
/// * Si ambos contienen valor, retorna `Some(a + b)`.
/// * Si uno está ausente, retorna el que está presente.
/// * Si ambos están ausentes, retorna `None`.
///
/// # Ejemplos
/// ```
/// use ex06_enums_patterns::sumar_opcionales;
/// assert_eq!(sumar_opcionales(Some(10), Some(20)), Some(30));
/// assert_eq!(sumar_opcionales(Some(10), None), Some(10));
/// assert_eq!(sumar_opcionales(None, None), None);
/// ```
pub fn sumar_opcionales(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mensaje_red_procesar() {
        assert_eq!(MensajeRed::Conectar.procesar(), "Conexión establecida.");
        assert_eq!(MensajeRed::Desconectar.procesar(), "Conexión cerrada.");
        assert_eq!(
            MensajeRed::Datos(vec![1, 2, 3]).procesar(),
            "Recibidos 3 bytes de datos."
        );
    }

    #[test]
    fn test_sumar_opcionales() {
        assert_eq!(sumar_opcionales(Some(5), Some(5)), Some(10));
        assert_eq!(sumar_opcionales(Some(-10), None), Some(-10));
        assert_eq!(sumar_opcionales(None, Some(8)), Some(8));
        assert_eq!(sumar_opcionales(None, None), None);
    }
}
