// exercises/ex18_patterns_matching/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 18.

/// Representa coordenadas en un plano 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordenada {
    pub x: i32,
    pub y: i32,
}

/// Enumeración de comandos gráficos de dibujo.
#[derive(Debug, Clone, PartialEq)]
pub enum Accion {
    Mover(Coordenada),
    Pintar { r: u8, g: u8, b: u8 },
    Ninguna,
}

/// Procesa una acción gráfica utilizando coincidencia de patrones avanzada.
/// Demuestra el uso de desestructuración, guardas `if` y coincidencia de rangos.
///
/// # Ejemplos
/// ```
/// use ex18_patterns_matching::{Accion, Coordenada, procesar_accion};
/// let accion = Accion::Mover(Coordenada { x: 0, y: 0 });
/// assert_eq!(procesar_accion(accion), "Origen detectado.");
/// ```
pub fn procesar_accion(accion: Accion) -> String {
    match accion {
        // 1. Desestructuración con guardas condicionales (match guard)
        Accion::Mover(Coordenada { x, y }) if x == 0 && y == 0 => {
            String::from("Origen detectado.")
        }
        Accion::Mover(Coordenada { x, y }) => {
            format!("Moviendo a posición física x={x}, y={y}.")
        }
        // 2. Coincidencia estructurada y rangos numéricos
        Accion::Pintar { r: 255, g: 0, b: 0 } => {
            String::from("Pintando de color Rojo Puro.")
        }
        Accion::Pintar { r, g, b } => {
            format!("Pintando con color RGB({}, {}, {}).", r, g, b)
        }
        Accion::Ninguna => String::from("Ninguna acción a procesar."),
    }
}

/// Evalúa un ID de usuario y determina su nivel de acceso utilizando el operador `@` (bindings).
///
/// # Ejemplos
/// ```
/// use ex18_patterns_matching::clasificar_acceso;
/// assert_eq!(clasificar_acceso(5), "Acceso Administrador Especial: ID 5");
/// assert_eq!(clasificar_acceso(50), "Acceso Usuario Estándar.");
/// assert_eq!(clasificar_acceso(200), "ID de usuario fuera de rango.");
/// ```
pub fn clasificar_acceso(id: u32) -> String {
    match id {
        // Enlace del valor coincidente dentro del rango a una variable local mediante `@`
        admin_id @ 1..=10 => format!("Acceso Administrador Especial: ID {admin_id}"),
        11..=100 => String::from("Acceso Usuario Estándar."),
        _ => String::from("ID de usuario fuera de rango."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_accion() {
        assert_eq!(
            procesar_accion(Accion::Mover(Coordenada { x: 0, y: 0 })),
            "Origen detectado."
        );
        assert_eq!(
            procesar_accion(Accion::Mover(Coordenada { x: 5, y: -2 })),
            "Moviendo a posición física x=5, y=-2."
        );
        assert_eq!(
            procesar_accion(Accion::Pintar { r: 255, g: 0, b: 0 }),
            "Pintando de color Rojo Puro."
        );
        assert_eq!(
            procesar_accion(Accion::Pintar { r: 10, g: 20, b: 30 }),
            "Pintando con color RGB(10, 20, 30)."
        );
        assert_eq!(procesar_accion(Accion::Ninguna), "Ninguna acción a procesar.");
    }

    #[test]
    fn test_clasificar_acceso() {
        assert_eq!(clasificar_acceso(1), "Acceso Administrador Especial: ID 1");
        assert_eq!(clasificar_acceso(50), "Acceso Usuario Estándar.");
        assert_eq!(clasificar_acceso(999), "ID de usuario fuera de rango.");
    }
}
