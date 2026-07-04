// exercises/ex05_using_structs/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 05.

/// Estructura clásica que modela un rectángulo bidimensional en una grilla de coordenadas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

impl Rectangulo {
    /// Inicializa una nueva instancia de Rectangulo con dimensiones específicas.
    pub fn nuevo(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    /// Crea un constructor asociado (fábrica) para generar cuadrados simétricos.
    ///
    /// # Ejemplos
    /// ```
    /// use ex05_using_structs::Rectangulo;
    /// let sq = Rectangulo::cuadrado(5);
    /// assert_eq!(sq.ancho, 5);
    /// assert_eq!(sq.alto, 5);
    /// ```
    pub fn cuadrado(lado: u32) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }

    /// Calcula el área física de la superficie plana del rectángulo.
    pub fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    /// Evalúa si el rectángulo actual es capaz de contener físicamente a otro menor
    /// en su totalidad sin realizar rotaciones.
    ///
    /// # Ejemplos
    /// ```
    /// use ex05_using_structs::Rectangulo;
    /// let r1 = Rectangulo::nuevo(30, 50);
    /// let r2 = Rectangulo::nuevo(10, 40);
    /// assert!(r1.puede_contener(&r2));
    /// ```
    pub fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }
}

/// Estructura que modela el perfil de acceso de un usuario.
#[derive(Debug, Clone, PartialEq)]
pub struct CuentaUsuario {
    pub nombre: String,
    pub correo: String,
    pub activo: bool,
}

impl CuentaUsuario {
    /// Genera una nueva cuenta de usuario validando la sintaxis lógica básica del correo.
    /// Retorna `Err` si el correo no contiene un carácter `@`.
    ///
    /// # Ejemplos
    /// ```
    /// use ex05_using_structs::CuentaUsuario;
    /// let cuenta = CuentaUsuario::nuevo("Elena", "elena@empresa.com");
    /// assert!(cuenta.is_ok());
    /// ```
    pub fn nuevo(nombre: &str, correo: &str) -> Result<Self, &'static str> {
        if !correo.contains('@') {
            return Err("Formato de correo electrónico inválido.");
        }
        Ok(Self {
            nombre: String::from(nombre),
            correo: String::from(correo),
            activo: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangulo_area() {
        let r = Rectangulo::nuevo(10, 5);
        assert_eq!(r.area(), 50);
    }

    #[test]
    fn test_rectangulo_puede_contener() {
        let r1 = Rectangulo::nuevo(20, 20);
        let r2 = Rectangulo::nuevo(10, 10);
        let r3 = Rectangulo::nuevo(25, 10);

        assert!(r1.puede_contener(&r2));
        assert!(!r2.puede_contener(&r1));
        assert!(!r1.puede_contener(&r3));
    }

    #[test]
    fn test_cuenta_usuario_valida() {
        let c = CuentaUsuario::nuevo("Admin", "admin@web.com");
        assert!(c.is_ok());
        let u = c.unwrap();
        assert_eq!(u.nombre, "Admin");
        assert!(u.activo);
    }

    #[test]
    fn test_cuenta_usuario_invalida() {
        let c = CuentaUsuario::nuevo("User", "correo_sin_arroba");
        assert!(c.is_err());
        assert_eq!(c.err(), Some("Formato de correo electrónico inválido."));
    }
}
