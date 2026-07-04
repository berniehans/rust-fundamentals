// src/main.rs
// Crate ch03_common_concepts
// Este programa demuestra físicamente los conceptos comunes de programación en Rust.

// 2. USO DE CONSTANTES
// Las constantes se evalúan en tiempo de compilación y se insertan ("inline") en el código.
// Deben declarar obligatoriamente el tipo de dato y pueden declararse en el ámbito global.
const MAX_INTENTOS: u32 = 3;

fn main() {
    println!("=== CAPÍTULO 03: CONCEPTOS COMUNES DE PROGRAMACIÓN ===");
    println!("Constante de compilación MAX_INTENTOS: {}", MAX_INTENTOS);

    // 1. MUTABILIDAD VS SHADOWING
    demostrar_mutabilidad_y_shadowing();

    // 3. TIPOS DE DATOS (ESCALARES Y COMPUESTOS)
    demostrar_tipos_datos();

    // 4. EXPRESIONES VS DECLARACIONES
    demostrar_expresiones_vs_declaraciones();

    // 5. CONTROL DE FLUJO
    demostrar_control_flujo();
}

/// Muestra la diferencia física entre mutar una variable y realizar shadowing.
fn demostrar_mutabilidad_y_shadowing() {
    println!("\n--- 1. MUTABILIDAD VS SHADOWING ---");

    // Mutabilidad:
    // let mut asigna un espacio de memoria en el Stack que puede ser modificado.
    // El tipo de dato debe permanecer idéntico.
    let mut x = 5;
    println!("Valor inicial de x mutable: {x}");
    x = 6; // Reasignación física de bytes en la pila
    println!("Valor modificado de x mutable: {x}");

    // Shadowing:
    // let redefine el identificador en el compilador. Permite cambiar el tipo de dato.
    // No incurre en sobrecosto en tiempo de ejecución; es una abstracción de costo cero.
    let espacios = "   "; // Tipo: &str (referencia a cadena de 16 bytes en el Stack)
    println!("Variable original 'espacios' (String Slice): '{}'", espacios);

    let espacios = espacios.len(); // Tipo: usize (entero de 8 bytes en 64-bits)
    println!("Variable enmascarada (shadowed) 'espacios' (usize): {espacios}");
}

/// Muestra los tipos escalares y compuestos almacenados en el Stack.
fn demostrar_tipos_datos() {
    println!("\n--- 2. TIPOS DE DATOS (STACK LAYOUT) ---");

    // Tipos Escalares: representan un único valor
    let entero: i32 = -42;         // Entero de 32 bits con signo
    let flotante: f64 = 3.14159;    // Punto flotante de 64 bits (IEEE-754)
    let booleano: bool = true;      // Booleano, ocupa exactamente 1 byte
    let caracter: char = '🦀';      // Carácter Unicode de 4 bytes en Rust

    println!("Escalares: i32={entero}, f64={flotante}, bool={booleano}, char={caracter}");

    // Tipos Compuestos: agrupan múltiples valores
    // Tuplas: Longitud fija, pueden agrupar tipos heterogéneos
    let tupla: (i32, f64, char) = (500, 6.4, '🌎');
    // Acceso mediante desestructuración por patrón
    let (t1, t2, t3) = tupla;
    // Acceso por índice directo posicional (.0, .1)
    println!("Tupla desestructurada: ({t1}, {t2}, {t3})");
    println!("Tupla por índice: (.0) = {}, (.1) = {}", tupla.0, tupla.1);

    // Arreglos (Arrays): Longitud fija, elementos homogéneos almacenados de forma contigua
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array: primer elemento = {}, tamaño físico = {} bytes", 
        array[0], 
        std::mem::size_of_val(&array) // 5 elementos * 4 bytes = 20 bytes
    );
}

/// Explica la distinción semántica entre declaraciones y expresiones en Rust.
fn demostrar_expresiones_vs_declaraciones() {
    println!("\n--- 3. DECLARACIONES VS EXPRESIONES ---");

    // Las declaraciones (statements) realizan una acción pero no devuelven valor. Terminan con ';'
    let _declaracion = 5; // let y = 6; es una declaración

    // Las expresiones (expressions) evalúan a un valor y NO terminan con ';'
    // Un bloque de llaves es una expresión si su última línea no tiene ';'
    let valor_retornado = {
        let x = 3;
        x + 1 // Expresión: produce 4. Si pusiéramos ';' devolvería ()
    };

    println!("Valor producido por el bloque de expresión: {valor_retornado}");
}

/// Muestra if como expresión, loop con retorno, y bucles con etiquetas.
fn demostrar_control_flujo() {
    println!("\n--- 4. CONTROL DE FLUJO ---");

    // 'if' es una expresión. Se puede usar en asignaciones
    let condicion = true;
    let numero = if condicion { 5 } else { 6 }; // Ambos bloques deben retornar el mismo tipo
    println!("Valor obtenido de 'if' como expresión: {numero}");

    // 'loop' con retorno de valor mediante 'break'
    let mut contador = 0;
    let resultado_loop = loop {
        contador += 1;
        if contador == 5 {
            break contador * 10; // Retorna 50
        }
    };
    println!("Resultado de 'loop' con break de valor: {resultado_loop}");

    let mut i = 0;
    println!("Iniciando bucles anidados con etiquetas...");
    'externo: loop {
        let mut j = 0;
        loop {
            if i == 2 {
                println!("i={i}, j={j}. Rompiendo bucle 'externo'...");
                break 'externo; // Rompe el bucle etiquetado 'externo
            }
            j += 1;
            println!("   Bucle interno: i={i}, j={j}");
            if j == 3 {
                break; // Rompe el bucle interno ordinario para pasar al siguiente i
            }
        }
        i += 1;
    }

    // Bucle 'for': el mecanismo más seguro para iterar sobre colecciones contiguas
    let coleccion = [10, 20, 30];
    print!("Iteración con for: ");
    for elemento in coleccion {
        print!("{elemento} ");
    }
    println!();
}
