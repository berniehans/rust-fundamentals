// ch05_using_structs - Demostración Educativa de Estructuras (Structs) y Métodos
// Este archivo profundiza en estructuras clásicas, de tupla, unitarias y bloques impl en Rust.

#[derive(Debug, Clone, PartialEq)]
struct Usuario {
    activo: bool,
    nombre: String,
    correo: String,
    inicios_sesion: u64,
}

// Estructura de Tupla: útil para el patrón Newtype y tipos con significado posicional
#[derive(Debug, Clone, Copy, PartialEq)]
struct ColorRGB(u8, u8, u8);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Punto2D(f64, f64);

// Estructura Unitaria (Unit-like struct): no almacena estado, actúa como marcador de tipo
#[derive(Debug)]
struct ValidadorSeguridad;

impl ValidadorSeguridad {
    fn validar_longitud(&self, texto: &str) -> bool {
        !texto.trim().is_empty()
    }
}

// Estructura clásica para modelar figuras geométricas y métodos de cálculo
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // 1. Función Asociada / Constructor (no toma self como parámetro)
    fn nuevo(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    // Constructor para cuadrados
    fn cuadrado(lado: u32) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }

    // 2. Método con Préstamo Inmutable (&self): solo lee el estado
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // Comprueba si este rectángulo puede contener a otro completamente
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho >= otro.ancho && self.alto >= otro.alto
    }

    // 3. Método con Préstamo Mutable (&mut self): modifica las dimensiones
    fn escalar(&mut self, factor: u32) {
        self.ancho *= factor;
        self.alto *= factor;
    }

    // 4. Método que Consume la Instancia (self): transfiere la propiedad
    fn transformar_a_area_total(self) -> u32 {
        self.ancho * self.alto
    }
}

fn main() {
    println!("=== CAPÍTULO 05: USANDO ESTRUCTURAS (STRUCTS) ===");

    demostrar_estructuras_basicas();
    demostrar_tuple_y_unit_structs();
    demostrar_metodos_y_asociadas();

    println!("\n¡Capítulo 05 ejecutado con éxito!");
}

fn demostrar_estructuras_basicas() {
    println!("\n--- 1. ESTRUCTURAS CLÁSICAS Y SINTAXIS DE ACTUALIZACIÓN ---");

    let correo = String::from("usuario@rust-lang.org");

    // Sintaxis abreviada de inicialización (field init shorthand)
    let usuario1 = Usuario {
        activo: true,
        nombre: String::from("Ferris"),
        correo,
        inicios_sesion: 5,
    };

    println!("Instancia usuario1 creada: {:?}", usuario1);
    println!("Detalle formateado con Pretty-Debug {:#?}:", usuario1);

    // Sintaxis de actualización de estructuras (struct update syntax)
    // El operador '..' copia/mueve los campos restantes de usuario1
    let usuario2 = Usuario {
        nombre: String::from("Corrosivo"),
        correo: String::from("corrosivo@rust-lang.org"),
        ..usuario1
    };

    println!(
        "Nuevo usuario2 generado mediante update syntax: {:?}",
        usuario2
    );
}

fn demostrar_tuple_y_unit_structs() {
    println!("\n--- 2. TUPLE STRUCTS Y UNIT-LIKE STRUCTS ---");

    let rojo = ColorRGB(255, 0, 0);
    let origen = Punto2D(0.0, 0.0);

    println!("Color RGB: R={}, G={}, B={}", rojo.0, rojo.1, rojo.2);
    println!("Punto cartesiano 2D: ({:.1}, {:.1})", origen.0, origen.1);

    let validador = ValidadorSeguridad;
    let es_valido = validador.validar_longitud("Texto de prueba");
    println!("Validación con Unit Struct: {}", es_valido);
}

fn demostrar_metodos_y_asociadas() {
    println!("\n--- 3. MÉTODOS Y FUNCIONES ASOCIADAS EN BLOQUES IMPL ---");

    // Uso de constructor asociado
    let mut rect1 = Rectangulo::nuevo(30, 50);
    let rect2 = Rectangulo::nuevo(10, 40);
    let cuadrado1 = Rectangulo::cuadrado(25);

    println!("Rectángulo 1: {:?}", rect1);
    println!("Área de Rectángulo 1 (&self): {} px²", rect1.area());
    println!(
        "¿Rectángulo 1 puede contener a Rectángulo 2?: {}",
        rect1.puede_contener(&rect2)
    );
    println!(
        "¿Rectángulo 1 puede contener a Cuadrado 1?: {}",
        rect1.puede_contener(&cuadrado1)
    );

    // Modificación de campos mediante método mutable (&mut self)
    rect1.escalar(2);
    println!(
        "Rectángulo 1 escalado x2 (&mut self): {:?}, nueva área: {}",
        rect1,
        rect1.area()
    );

    // Método que consume la propiedad (self)
    let area_consumida = rect1.transformar_a_area_total();
    println!(
        "Instancia consumida (self). Área final resultante: {}",
        area_consumida
    );
}
