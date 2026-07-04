// exercises/ex10_generics_traits_lifetimes/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 10.

/// Trait que define el comportamiento para resumir o sintetizar un contenido textual.
pub trait Resumible {
    fn resumir(&self) -> String;
}

/// Estructura que modela un artículo de prensa.
#[derive(Debug, Clone)]
pub struct Noticia {
    pub titular: String,
    pub autor: String,
    pub contenido: String,
}

impl Resumible for Noticia {
    /// Resume una noticia combinando el titular con el autor del artículo.
    fn resumir(&self) -> String {
        format!("{}: por {}", self.titular, self.autor)
    }
}

/// Estructura que modela una publicación en redes sociales.
#[derive(Debug, Clone)]
pub struct Tweet {
    pub usuario: String,
    pub mensaje: String,
}

impl Resumible for Tweet {
    /// Resume un tweet mostrando el usuario y el mensaje publicado.
    fn resumir(&self) -> String {
        format!("@{}: {}", self.usuario, self.mensaje)
    }
}

/// Compara dos slices de cadena y devuelve el que posea la mayor longitud de caracteres.
/// Demuestra el uso de anotaciones genéricas de lifetimes (`'a`) para referenciar retornos válidos.
///
/// # Ejemplos
/// ```
/// use ex10_generics_traits_lifetimes::mayor_con_lifetime;
/// let c1 = "corta";
/// let c2 = "larga_cadena";
/// let result = mayor_con_lifetime(c1, c2);
/// assert_eq!(result, "larga_cadena");
/// ```
pub fn mayor_con_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resumible_noticia() {
        let noticia = Noticia {
            titular: String::from("Rust 2024 Lanzado"),
            autor: String::from("Equipo de Rust"),
            contenido: String::from("Nuevas características y mejoras..."),
        };
        assert_eq!(noticia.resumir(), "Rust 2024 Lanzado: por Equipo de Rust");
    }

    #[test]
    fn test_resumible_tweet() {
        let tweet = Tweet {
            usuario: String::from("rust_lang"),
            mensaje: String::from("¡Aprender Rust con Cargo Workspaces es genial!"),
        };
        assert_eq!(tweet.resumir(), "@rust_lang: ¡Aprender Rust con Cargo Workspaces es genial!");
    }

    #[test]
    fn test_mayor_con_lifetime() {
        let s1 = String::from("primera");
        let s2 = "segunda_larga";
        
        let res = mayor_con_lifetime(&s1, s2);
        assert_eq!(res, "segunda_larga");
    }
}
