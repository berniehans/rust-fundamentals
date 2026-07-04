// exercises/ex19_advanced_features/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 19.

/// Lee el valor entero apuntado por una dirección de memoria cruda inmutable.
/// Demuestra las restricciones e invocación de bloques e instrucciones `unsafe`.
///
/// # Safety
/// El llamador debe garantizar que el puntero crudo es válido, no nulo y apunta a un i32 inicializado.
///
/// # Ejemplos
/// ```
/// use ex19_advanced_features::desreferenciar_puntero;
/// let val = 42;
/// let ptr = &val as *const i32;
/// unsafe {
///     assert_eq!(desreferenciar_puntero(ptr), 42);
/// }
/// ```
pub unsafe fn desreferenciar_puntero(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

/// Trait avanzado que define operaciones de procesamiento utilizando tipos asociados.
pub trait Analizador {
    type Entrada;
    type Salida;

    fn analizar(&self, entrada: Self::Entrada) -> Self::Salida;
}

/// Analizador concreto para números enteros.
pub struct ParseadorEnteros;

impl Analizador for ParseadorEnteros {
    type Entrada = String;
    type Salida = Result<i32, &'static str>;

    /// Parsea una cadena de texto a un entero de 32 bits usando el tipo asociado.
    fn analizar(&self, entrada: Self::Entrada) -> Self::Salida {
        match entrada.trim().parse() {
            Ok(num) => Ok(num),
            Err(_) => Err("No se pudo parsear el valor como i32."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_desreferencia_puntero() {
        let x = 100;
        let ptr = &x as *const i32;
        unsafe {
            assert_eq!(desreferenciar_puntero(ptr), 100);
        }
    }

    #[test]
    fn test_analizador_tipo_asociado() {
        let parser = ParseadorEnteros;
        
        let res_ok = parser.analizar(String::from("  42  "));
        assert_eq!(res_ok, Ok(42));

        let res_err = parser.analizar(String::from("no_soy_numero"));
        assert_eq!(res_err, Err("No se pudo parsear el valor como i32."));
    }
}
