// ch18_patterns_matching - Demostración Educativa de Sintaxis Avanzada de Patrones
// Este archivo profundiza en patrones refutables vs irrefutables, desestructuración, match guards y @ bindings.

#[allow(dead_code)]
#[derive(Debug)]
struct Punto3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
enum MensajeApp {
    Salir,
    Mover { x: i32, y: i32 },
    CambiarColorRGB(u8, u8, u8),
    MensajeID { id: u32 },
}

fn main() {
    println!("=== CAPÍTULO 18: PATRONES Y COINCIDENCIAS (PATTERN MATCHING) ===");

    demostrar_lugares_de_patrones();
    demostrar_desestructuracion_avanzada();
    demostrar_match_guards_y_at_bindings();

    println!("\n¡Capítulo 18 ejecutado con éxito!");
}

fn demostrar_lugares_de_patrones() {
    println!("\n--- 1. LUGARES DE PATRONES: WHILE LET, LET-ELSE Y TUPLAS ---");

    // A. Bucle while let: se ejecuta mientras pop() retorne Some(v)
    let mut pila = vec![10, 20, 30];
    print!("Vaciando pila con while let: ");
    while let Some(top) = pila.pop() {
        print!("{top} ");
    }
    println!();

    // B. Desestructuración de tuplas en parámetros de for
    let pares = vec![('A', 1), ('B', 2), ('C', 3)];
    println!("Iteración con desestructuración en for:");
    for (letra, valor) in pares {
        println!("  Letra: '{letra}' -> Valor: {valor}");
    }

    // C. Sentencia let-else con patrón refutable
    fn extraer_id_valido(opt_id: Option<u32>) -> u32 {
        let Some(id) = opt_id else {
            return 0; // Divergencia obligatoria si falla el patrón
        };
        id
    }
    println!(
        "Extracción let-else con Some: {}",
        extraer_id_valido(Some(99))
    );
    println!("Extracción let-else con None: {}", extraer_id_valido(None));
}

fn demostrar_desestructuracion_avanzada() {
    println!("\n--- 2. DESESTRUCTURACIÓN DE ESTRUCTURAS, ENUMS Y RANGOS ---");

    let origen = Punto3D { x: 0, y: 15, z: 25 };

    // Desestructuración con renombrado y comodín '..' para ignorar campos restantes
    let Punto3D { x: coord_x, y, .. } = origen;
    println!("Punto desestructurado: x={coord_x}, y={y} (z ignorado con '..')");

    // Coincidencia sobre Enums con rangos numéricos y OR múltiple
    let mensajes = vec![
        MensajeApp::Salir,
        MensajeApp::Mover { x: 0, y: 0 },
        MensajeApp::Mover { x: 10, y: 50 },
        MensajeApp::CambiarColorRGB(255, 0, 0),
        MensajeApp::MensajeID { id: 5 },
    ];

    for msg in mensajes {
        match msg {
            MensajeApp::Salir => println!("  Mensaje: Salir de la aplicación."),
            MensajeApp::Mover { x: 0, y: 0 } => {
                println!("  Mensaje: Regreso al punto de origen (0, 0).")
            }
            MensajeApp::Mover { x, y } => println!("  Mensaje: Mover a ({x}, {y})."),
            MensajeApp::CambiarColorRGB(255, 0, 0) | MensajeApp::CambiarColorRGB(0, 255, 0) => {
                println!("  Mensaje: Color primario detectado (Rojo Puro o Verde Puro).");
            }
            MensajeApp::CambiarColorRGB(r, g, b) => {
                println!("  Mensaje: Color personalizado RGB({r}, {g}, {b}).");
            }
            MensajeApp::MensajeID { id: id @ 1..=10 } => {
                // @ binding: comprueba el rango 1..=10 Y vincula el valor a la variable 'id'
                println!("  Mensaje: ID prioritario en rango reservado (1-10): ID #{id}");
            }
            MensajeApp::MensajeID { id } => {
                println!("  Mensaje: ID estándar: #{id}");
            }
        }
    }
}

fn demostrar_match_guards_y_at_bindings() {
    println!("\n--- 3. MATCH GUARDS (GUARDAS CONDICIONALES 'IF') ---");

    let numero = Some(4);
    let es_par = true;

    match numero {
        // Match Guard: Añade una condición booleana adicional después del patrón
        Some(x) if x % 2 == 0 && es_par => {
            println!("El número {x} es par y el flag 'es_par' está activo.");
        }
        Some(x) => println!("Número recibido: {x}"),
        None => println!("Ningún número disponible."),
    }
}
