// ch17_oop_features - Demostración Educativa de Conceptos OOP en Rust
// Este archivo profundiza en encapsulación, polimorfismo dinámico con Trait Objects (dyn Trait, vtables)
// y el patrón Typestate idiomático frente a la herencia clásica.

// 1. Polimorfismo Dinámico mediante Trait Objects
pub trait Dibujable {
    fn dibujar(&self);
}

pub struct Boton {
    pub ancho: u32,
    pub alto: u32,
    pub etiqueta: String,
}

impl Dibujable for Boton {
    fn dibujar(&self) {
        println!(
            "[GUI Botón]: Dibujando botón '{}' ({}x{} px)",
            self.etiqueta, self.ancho, self.alto
        );
    }
}

pub struct CampoTexto {
    pub placeholder: String,
}

impl Dibujable for CampoTexto {
    fn dibujar(&self) {
        println!(
            "[GUI CampoTexto]: Dibujando input con placeholder '{}'",
            self.placeholder
        );
    }
}

// Estructura contenedora que almacena elementos heterogéneos mediante despacho dinámico
pub struct Pantalla {
    pub componentes: Vec<Box<dyn Dibujable>>,
}

impl Pantalla {
    pub fn renderizar(&self) {
        println!("Renderizando componentes de la pantalla:");
        for comp in &self.componentes {
            comp.dibujar(); // Invocación a través del puntero de vtable en runtime
        }
    }
}

// 2. Patrón Typestate Idiomático (Seguridad de Estados en Tiempo de Compilación)
// En lugar de manejar estados con punteros nulos o variables mutables en runtime,
// codificamos cada fase del ciclo de vida en tipos físicos independientes.

pub struct BorradorPost {
    contenido: String,
}

impl BorradorPost {
    pub fn nuevo() -> Self {
        Self {
            contenido: String::new(),
        }
    }

    pub fn agregar_texto(&mut self, texto: &str) {
        self.contenido.push_str(texto);
    }

    // Transición de estado que consume el borrador (self) y retorna el nuevo tipo
    pub fn solicitar_revision(self) -> PostEnRevision {
        PostEnRevision {
            contenido: self.contenido,
        }
    }
}

impl Default for BorradorPost {
    fn default() -> Self {
        Self::nuevo()
    }
}

pub struct PostEnRevision {
    contenido: String,
}

impl PostEnRevision {
    // Solo un post en revisión puede ser aprobado
    pub fn aprobar(self) -> PostPublicado {
        PostPublicado {
            contenido: self.contenido,
        }
    }
}

pub struct PostPublicado {
    contenido: String,
}

impl PostPublicado {
    // Solo un post publicado expone su contenido públicamente
    pub fn contenido(&self) -> &str {
        &self.contenido
    }
}

fn main() {
    println!("=== CAPÍTULO 17: CARACTERÍSTICAS DE OOP EN RUST ===");

    demostrar_trait_objects_gui();
    demostrar_patron_typestate();

    println!("\n¡Capítulo 17 ejecutado con éxito!");
}

fn demostrar_trait_objects_gui() {
    println!("\n--- 1. POLIMORFISMO DINÁMICO CON TRAIT OBJECTS (DYN TRAIT) ---");

    let pantalla = Pantalla {
        componentes: vec![
            Box::new(Boton {
                ancho: 75,
                alto: 25,
                etiqueta: String::from("Aceptar"),
            }),
            Box::new(CampoTexto {
                placeholder: String::from("Ingrese su correo..."),
            }),
            Box::new(Boton {
                ancho: 60,
                alto: 20,
                etiqueta: String::from("Cancelar"),
            }),
        ],
    };

    pantalla.renderizar();
}

fn demostrar_patron_typestate() {
    println!("\n--- 2. PATRÓN TYPESTATE: MÁQUINAS DE ESTADO GARANTIZADAS POR EL COMPILADOR ---");

    let mut post = BorradorPost::nuevo();
    post.agregar_texto("Rust no necesita herencia tradicional porque tiene Composición y Traits.");
    println!("Fase 1: Post creado en estado Borrador.");

    let post_en_revision = post.solicitar_revision();
    println!("Fase 2: Post transferido a estado En Revisión (el borrador original fue consumido).");

    let post_publicado = post_en_revision.aprobar();
    println!("Fase 3: Post aprobado y Publicado.");
    println!(
        "Contenido final del post público: \"{}\"",
        post_publicado.contenido()
    );
}
