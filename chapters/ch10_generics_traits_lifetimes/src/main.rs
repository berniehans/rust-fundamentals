// ch10_generics_traits_lifetimes - Demostración Educativa de Genéricos, Traits y Lifetimes
// Este archivo profundiza en polimorfismo estático (monomorfización), interfaces con traits y contratos de lifetimes.

// 1. Estructura Genérica con múltiples variables de tipo
#[derive(Debug)]
struct Punto<T, U> {
    x: T,
    y: U,
}

impl<T, U> Punto<T, U> {
    fn mezclar<V, W>(self, otro: Punto<V, W>) -> Punto<T, W> {
        Punto {
            x: self.x,
            y: otro.y,
        }
    }
}

// 2. Definición de un Trait (Interfaz de Comportamiento Compartido)
pub trait Resumible {
    // Firma abstracta requerida
    fn resumir(&self) -> String;

    // Método con implementación predeterminada
    fn encabezado(&self) -> String {
        format!("(Notificación general): {}", self.resumir())
    }
}

pub struct Articulo {
    pub titular: String,
    pub autor: String,
    pub contenido: String,
}

impl Resumible for Articulo {
    fn resumir(&self) -> String {
        format!(
            "'{}' por {} ({} palabras)",
            self.titular,
            self.autor,
            self.contenido.len()
        )
    }
}

pub struct Tweet {
    pub usuario: String,
    pub mensaje: String,
}

impl Resumible for Tweet {
    fn resumir(&self) -> String {
        format!("@{}: {}", self.usuario, self.mensaje)
    }

    // Sobreescritura opcional del método con valor por defecto
    fn encabezado(&self) -> String {
        format!("(Nuevo Tweet en Vivo): {}", self.resumir())
    }
}

// Función con Trait Bounds y sintaxis 'where'
fn notificar_alerta<T>(item: &T)
where
    T: Resumible,
{
    println!("[Alerta Sistema]: {}", item.encabezado());
}

// 3. Estructura que almacena una referencia y requiere anotación explícita de Lifetime ('a)
#[allow(dead_code)]
#[derive(Debug)]
struct CitaDestacada<'a> {
    texto: &'a str,
}

// Función con Lifetime genérico explícito:
// El compilador garantiza que la referencia retornada vivirá tanto como el menor de los lifetimes de x e y.
fn mayor_referencia<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("=== CAPÍTULO 10: GENÉRICOS, TRAITS Y LIFETIMES ===");

    demostrar_genericos();
    demostrar_traits_y_bounds();
    demostrar_lifetimes_y_referencias();

    println!("\n¡Capítulo 10 ejecutado con éxito!");
}

fn demostrar_genericos() {
    println!("\n--- 1. POLIMORFISMO ESTÁTICO Y MONOMORFIZACIÓN ---");

    let p1 = Punto { x: 5, y: 10.4 }; // Punto<i32, f64>
    let p2 = Punto { x: "Hola", y: 'c' }; // Punto<&str, char>
    let p3 = p1.mezclar(p2); // Punto<i32, char>

    println!("Punto combinado mediante genéricos: x={}, y={}", p3.x, p3.y);
    println!("(Cero sobrecosto en runtime gracias a la monomorfización en tiempo de compilación).");
}

fn demostrar_traits_y_bounds() {
    println!("\n--- 2. COMPORTAMIENTO COMPARTIDO CON TRAITS Y WHERE CLAUSE ---");

    let noticia = Articulo {
        titular: String::from("Rust 2024 revoluciona los sistemas embebidos"),
        autor: String::from("Comunidad Rust"),
        contenido: String::from(
            "El nuevo ecosistema agiliza la seguridad sin recolector de basura...",
        ),
    };

    let tweet = Tweet {
        usuario: String::from("rustlang"),
        mensaje: String::from("¡El Cargo Workspace está completamente operativo!"),
    };

    notificar_alerta(&noticia);
    notificar_alerta(&tweet);
}

fn demostrar_lifetimes_y_referencias() {
    println!("\n--- 3. CONTRATOS DE LIFETIME ('A) Y BORROW CHECKER ---");

    let cadena1 = String::from("cadena_larga_principal");
    let resultado: &str;
    {
        let cadena2 = "corta";
        // Ambas referencias son válidas durante la llamada
        resultado = mayor_referencia(cadena1.as_str(), cadena2);
        println!(
            "La mayor cadena entre '{}' y '{}' es: '{}'",
            cadena1, cadena2, resultado
        );
    }

    let parrafo = String::from("El diseño seguro de memoria es prioritario. Rust es el estándar.");
    let primera_frase = parrafo.split('.').next().unwrap_or("");
    let cita = CitaDestacada {
        texto: primera_frase,
    };
    println!(
        "Cita destacada con lifetime 'a vinculada a 'parrafo': {:?}",
        cita
    );

    // Lifetime 'static: vive durante toda la ejecución del binario en la sección de datos de sólo lectura
    let literal_estatico: &'static str = "Constante estática almacenada en el binario";
    println!("Referencia con lifetime 'static: {literal_estatico}");
}
