// exercises/ex17_oop_features/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 17.

/// Trait que define las transiciones y comportamientos lógicos de cada estado de un documento.
pub trait Estado {
    fn solicitar_revision(self: Box<Self>) -> Box<dyn Estado>;
    fn publicar(self: Box<Self>) -> Box<dyn Estado>;
    fn contenido<'a>(&self, _doc: &'a Documento) -> &'a str {
        ""
    }
}

/// Estado inicial de un documento.
pub struct Borrador;
impl Estado for Borrador {
    fn solicitar_revision(self: Box<Self>) -> Box<dyn Estado> {
        Box::new(EnRevision)
    }

    fn publicar(self: Box<Self>) -> Box<dyn Estado> {
        self // Un borrador no puede publicarse directamente
    }
}

/// Estado intermedio de revisión.
pub struct EnRevision;
impl Estado for EnRevision {
    fn solicitar_revision(self: Box<Self>) -> Box<dyn Estado> {
        self // Ya está en revisión
    }

    fn publicar(self: Box<Self>) -> Box<dyn Estado> {
        Box::new(Publicado)
    }
}

/// Estado final publicado.
pub struct Publicado;
impl Estado for Publicado {
    fn solicitar_revision(self: Box<Self>) -> Box<dyn Estado> {
        self // Ya está publicado, no vuelve a revisión
    }

    fn publicar(self: Box<Self>) -> Box<dyn Estado> {
        self
    }

    /// Sobrescribe el método para exponer el texto cuando el documento ya está publicado.
    fn contenido<'a>(&self, doc: &'a Documento) -> &'a str {
        &doc.texto
    }
}

/// Objeto de dominio que encapsula el texto y delega su estado al patrón State.
pub struct Documento {
    estado: Option<Box<dyn Estado>>,
    texto: String,
}

impl Documento {
    /// Inicializa un nuevo documento en estado de Borrador.
    pub fn nuevo() -> Self {
        Self {
            estado: Some(Box::new(Borrador)),
            texto: String::new(),
        }
    }

    /// Permite añadir texto al cuerpo del documento.
    pub fn agregar_texto(&mut self, t: &str) {
        self.texto.push_str(t);
    }

    /// Consulta el contenido expuesto. Retorna cadena vacía a menos que esté publicado.
    pub fn contenido(&self) -> &str {
        self.estado.as_ref().unwrap().contenido(self)
    }

    /// Transiciona el estado del documento a revisión.
    pub fn solicitar_revision(&mut self) {
        if let Some(s) = self.estado.take() {
            self.estado = Some(s.solicitar_revision());
        }
    }

    /// Transiciona el estado del documento a publicado.
    pub fn publicar(&mut self) {
        if let Some(s) = self.estado.take() {
            self.estado = Some(s.publicar());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ciclo_vida_documento_state_pattern() {
        let mut doc = Documento::nuevo();
        doc.agregar_texto("Hola Rust OOP");

        // En borrador, el contenido debe estar oculto/vacío
        assert_eq!(doc.contenido(), "");

        // En revisión, el contenido sigue vacío
        doc.solicitar_revision();
        assert_eq!(doc.contenido(), "");

        // Publicado: el contenido ahora debe estar expuesto
        doc.publicar();
        assert_eq!(doc.contenido(), "Hola Rust OOP");
    }
}
