// ch04_understanding_ownership - Demostración Educativa del Modelo de Ownership
// Este archivo contiene ejemplos prácticos exhaustivamente comentados sobre el sistema de
// propiedad, préstamos (borrowing), referencias y porciones de memoria (slices) en Rust.

fn main() {
    println!("=== CAPÍTULO 04: ENTENDIENDO EL MODELO DE OWNERSHIP ===");

    demostrar_reglas_ownership_y_scope();
    demostrar_move_vs_clone_vs_copy();
    demostrar_referencias_y_borrowing();
    demostrar_referencias_mutables();
    demostrar_slices_y_fat_pointers();

    println!("\n¡Capítulo 04 ejecutado con éxito!");
}

/// 1. Demuestra las reglas de ámbito (scope) y liberación automática de recursos (drop)
fn demostrar_reglas_ownership_y_scope() {
    println!("\n--- 1. ÁMBITO (SCOPE) Y FUNCIÓN DROP ---");

    {
        // s solo es válida dentro de este bloque
        let s = String::from("Rust y Ownership");
        println!("Variable viva dentro de su scope: '{s}'");
        // Al llegar a la llave de cierre '}', Rust invoca automáticamente la función `drop`
        // liberando la memoria reservada en el Heap sin necesidad de recolector de basura (GC).
    }
    // s ya no existe en este punto de la ejecución.
    println!("Memoria en Heap liberada automáticamente al salir del bloque.");
}

/// 2. Demuestra la semántica de Transferencia (Move), Clonación (Clone) y Copia (Copy)
fn demostrar_move_vs_clone_vs_copy() {
    println!("\n--- 2. MOVE VS CLONE VS COPY ---");

    // A. SEMÁNTICA MOVE (Tipos dinámicos en Heap sin trait Copy)
    // s1 posee un descriptor en la pila (puntero, len=4, capacidad=4) apuntando al Heap.
    let s1 = String::from("hola");

    // Al asignar s1 a s2, Rust copia el descriptor del Stack e INVALIDA s1 (Move).
    // Esto previene el error crítico de Doble Liberación ("Double Free").
    let s2 = s1;
    println!("Move completado: s2 es dueña del recurso ('{s2}'). s1 fue invalidada.");

    // B. SEMÁNTICA CLONE (Copia profunda explícita)
    // Duplica tanto el descriptor en la pila como el búfer de bytes en el Heap.
    let s3 = s2.clone();
    println!("Clone completado: s2='{s2}' y s3='{s3}' son independientes en el Heap.");

    // C. SEMÁNTICA COPY (Tipos almacenados íntegramente en Stack con tamaño conocido)
    // Los enteros (i32) implementan el trait `Copy`. Se copian bit-a-bit directamente.
    let x: i32 = 42;
    let y = x; // Copia directa en el Stack
    println!("Copy primitivo: x={x}, y={y} (ambas siguen vivas y válidas).");
}

/// 3. Demuestra referencias inmutables y el concepto de préstamo (Borrowing)
fn demostrar_referencias_y_borrowing() {
    println!("\n--- 3. REFERENCIAS INMUTABLES (BORROWING) ---");

    let texto = String::from("Sistemas Confiables");

    // Pasamos una referencia (&texto). La función NO toma posesión (ownership).
    let len = calcular_longitud(&texto);

    // Como solo prestamos el valor, `texto` sigue siendo completamente válido aquí.
    println!("La cadena '{texto}' tiene {len} bytes de longitud.");
}

#[allow(clippy::ptr_arg)]
fn calcular_longitud(s: &String) -> usize {
    // s es un puntero a la estructura String en el Stack del llamador
    s.len()
} // s sale de ámbito aquí, pero como no es el dueño, NO se llama a drop.

/// 4. Demuestra préstamos mutables (&mut) y prevención de Data Races
fn demostrar_referencias_mutables() {
    println!("\n--- 4. PRÉSTAMOS MUTABLES Y REGLAS DEL BORROW CHECKER ---");

    let mut mensaje = String::from("Hola");
    println!("Mensaje inicial: '{mensaje}'");

    // Modificamos el contenido a través de una referencia mutable exclusiva (&mut)
    agregar_sufijo(&mut mensaje);
    println!("Mensaje tras mutación por referencia: '{mensaje}'");

    // Demostración de Scope no solapado para referencias mutables:
    // Rust permite múltiples préstamos mutables siempre que no coexistan en el mismo instante.
    {
        let r1 = &mut mensaje;
        r1.push('!');
    } // r1 sale de ámbito, permitiendo un nuevo préstamo

    let r2 = &mut mensaje;
    r2.push_str(" 🦀");
    println!("Mensaje final tras préstamos mutables secuenciales: '{mensaje}'");
}

fn agregar_sufijo(s: &mut String) {
    s.push_str(", Mundo Rustáceo");
}

/// 5. Demuestra Slices (&str, &[T]) y el concepto de Fat Pointers (16 bytes en x86_64)
fn demostrar_slices_y_fat_pointers() {
    println!("\n--- 5. SLICES (PORCIONES) Y FAT POINTERS ---");

    let frase = String::from("Rust es increiblemente rápido");

    // Slices de cadenas (&str): Puntero al byte inicial + longitud
    let primera = primera_palabra(&frase);
    println!("Primera palabra extraída con slice: '{primera}'");

    let palabra_medio: &str = &frase[8..21];
    println!("Porción intermedia con rango [8..21]: '{palabra_medio}'");

    // Slices de arreglos de enteros contiguos (&[i32])
    let numeros = [10, 20, 30, 40, 50];
    let porcion_numerica: &[i32] = &numeros[1..4]; // Elementos [20, 30, 40]
    println!("Porción de arreglo [1..4]: {:?}", porcion_numerica);
}

/// Algoritmo clásico del libro: encuentra la primera palabra retornando un string slice (&str)
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
