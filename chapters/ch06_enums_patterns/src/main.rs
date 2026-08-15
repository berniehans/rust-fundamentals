// ch06_enums_patterns - Demostración Educativa de Enums y Pattern Matching
// Este archivo cubre tipos suma algebraicos, Option<T>, control exhaustivo y la Null Pointer Optimization.

use std::mem::size_of;

#[derive(Debug, Clone, PartialEq)]
enum MensajeRed {
    Desconectar,                    // Variante unitaria (sin payload)
    MoverCursor { x: i32, y: i32 }, // Variante estructurada con campos con nombre
    EscribirTexto(String),          // Variante de tupla con String (en Heap)
    CambiarColor(u8, u8, u8),       // Variante de tupla con bytes RGB
}

impl MensajeRed {
    // Los enums también pueden tener bloques impl y definir métodos asociados
    fn procesar(&self) {
        match self {
            MensajeRed::Desconectar => {
                println!("[Evento Red]: Desconexión solicitada.");
            }
            MensajeRed::MoverCursor { x, y } => {
                println!("[Evento Red]: Cursor movido a ({x}, {y}).");
            }
            MensajeRed::EscribirTexto(texto) => {
                println!("[Evento Red]: Mensaje de chat recibido: \"{texto}\"");
            }
            MensajeRed::CambiarColor(r, g, b) => {
                println!("[Evento Red]: Color de interfaz actualizado a RGB({r}, {g}, {b}).");
            }
        }
    }
}

// Modelado de monedas y coincidencia anidada
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EstadoUs {
    Alabama,
    Alaska,
    California,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Moneda {
    Centavo,
    Niquel,
    DiezCentavos,
    Cuarto(EstadoUs),
}

fn valor_en_centavos(moneda: Moneda) -> u32 {
    match moneda {
        Moneda::Centavo => 1,
        Moneda::Niquel => 5,
        Moneda::DiezCentavos => 10,
        Moneda::Cuarto(estado) => {
            println!("¡Cuarto de dólar de edición especial de {:?}!", estado);
            25
        }
    }
}

fn main() {
    println!("=== CAPÍTULO 06: ENUMS Y PATTERN MATCHING ===");

    demostrar_variantes_enum();
    demostrar_tipo_option();
    demostrar_if_let_y_let_else();
    demostrar_layout_y_npo();

    println!("\n¡Capítulo 06 ejecutado con éxito!");
}

fn demostrar_variantes_enum() {
    println!("\n--- 1. ENUMS CON CARGA ÚTIL (PAYLOAD) Y DISCRIMINANTE ---");

    let eventos = vec![
        MensajeRed::MoverCursor { x: 100, y: 200 },
        MensajeRed::EscribirTexto(String::from("¡Hola desde Rust!")),
        MensajeRed::CambiarColor(34, 139, 34),
        MensajeRed::Desconectar,
    ];

    for evento in &eventos {
        evento.procesar();
    }

    println!("\nEvaluación con match y valores asociados:");
    let moneda1 = Moneda::Cuarto(EstadoUs::California);
    let valor = valor_en_centavos(moneda1);
    println!("Valor total en centavos: {valor}¢");
}

fn demostrar_tipo_option() {
    println!("\n--- 2. EL TIPO OPTION<T> FRENTE AL CONCEPTO DE NULL ---");

    let numero_presente: Option<i32> = Some(5);
    let numero_ausente: Option<i32> = None;

    #[allow(clippy::manual_map)]
    fn sumar_uno(opcion: Option<i32>) -> Option<i32> {
        match opcion {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    println!("Option Some(5) + 1: {:?}", sumar_uno(numero_presente));
    println!("Option None + 1: {:?}", sumar_uno(numero_ausente));
}

fn demostrar_if_let_y_let_else() {
    println!("\n--- 3. CONTROL DE FLUJO CON IF LET Y LET-ELSE ---");

    let configuracion_opcional: Option<&str> = Some("Modo Producción");

    // if let es azúcar sintáctico para un match que solo maneja una variante
    if let Some(modo) = configuracion_opcional {
        println!("Configuración detectada mediante 'if let': {modo}");
    }

    // let-else: desempaqueta el valor o sale del flujo inmediatamente con return/break/panic
    let valor_valido: Option<u32> = Some(100);
    let Some(numero) = valor_valido else {
        println!("Valor ausente, abortando...");
        return;
    };
    println!("Valor extraído mediante 'let-else': {numero}");
}

fn demostrar_layout_y_npo() {
    println!("\n--- 4. BAJO EL CAPÓ: NULL POINTER OPTIMIZATION (NPO) ---");

    // Un puntero o referencia estándar en 64-bits ocupa 8 bytes
    println!("Tamaño de una referencia &i32: {} bytes", size_of::<&i32>());

    // Gracias a NPO, Option<&i32> ocupa EXACTAMENTE los mismos 8 bytes,
    // usando la dirección 0x0 para representar None sin ningún overhead de memoria.
    println!(
        "Tamaño de Option<&i32> con NPO: {} bytes (¡Cero coste adicional!)",
        size_of::<Option<&i32>>()
    );

    // Mientras que un Option<i32> necesita el discriminante (tag), sumando padding
    println!(
        "Tamaño de Option<i32> con tag discriminante: {} bytes",
        size_of::<Option<i32>>()
    );
}
