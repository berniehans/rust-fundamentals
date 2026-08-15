// ch08_common_collections - Demostración Educativa de Vectores, Strings y HashMaps
// Este archivo profundiza en las colecciones dinámicas en el Heap, UTF-8, Entry API y gestión de capacidad.

use std::collections::HashMap;
use std::mem::size_of;

#[allow(dead_code)]
#[derive(Debug)]
enum CeldaHojaCalculo {
    Entero(i32),
    Flotante(f64),
    Texto(String),
}

fn main() {
    println!("=== CAPÍTULO 08: COLECCIONES COMUNES (VEC, STRING, HASHMAP) ===");

    demostrar_vectores();
    demostrar_strings_y_utf8();
    demostrar_hashmaps_y_entry_api();
    demostrar_layout_memoria();

    println!("\n¡Capítulo 08 ejecutado con éxito!");
}

fn demostrar_vectores() {
    println!("\n--- 1. VECTORES DINÁMICOS (VEC<T>) ---");

    // Inicialización y mutación
    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);

    println!("Vector numérico inicial: {:?}", numeros);

    // Acceso mediante indexación directa vs método seguro get()
    let segundo = numeros[1]; // Acceso directo (puede causar panic si fuera de límites)
    println!("Elemento en índice 1 (directo): {segundo}");

    match numeros.get(5) {
        Some(valor) => println!("Elemento en índice 5: {valor}"),
        None => println!("Índice 5 fuera de límites (manejado de forma segura con Option)."),
    }

    // Iteración con mutación en el lugar (in-place)
    for n in &mut numeros {
        *n *= 2; // Desreferenciación para modificar el valor subyacente
    }
    println!("Vector duplicado con &mut: {:?}", numeros);

    // Uso de Enums para almacenar datos heterogéneos en un vector contiguo
    let fila = vec![
        CeldaHojaCalculo::Entero(42),
        CeldaHojaCalculo::Flotante(45.67),
        CeldaHojaCalculo::Texto(String::from("Ingresos Q1")),
    ];
    println!(
        "Fila con tipos heterogéneos encapsulados en Enum: {:?}",
        fila
    );
}

fn demostrar_strings_y_utf8() {
    println!("\n--- 2. CADENAS DE TEXTO (STRING) Y CODIFICACIÓN UTF-8 ---");

    let mut saludo = String::from("¡Hola");
    saludo.push_str(", Mundo"); // Añade un string slice
    saludo.push('!'); // Añade un carácter char
    println!("Cadena construida: {saludo}");

    // Concatenación con operador '+' (mueve s1 y toma referencia de s2)
    let s1 = String::from("Rustaceo");
    let s2 = String::from(" Pro");
    let s3 = s1 + &s2; // s1 queda invalidada tras esta línea
    println!("Concatenación con '+': '{s3}'");

    // Concatenación sin transferir propiedad usando format!
    let a = "Alto";
    let b = "Rendimiento";
    let formateado = format!("{a} {b}");
    println!("Formateado con format!: '{formateado}'");

    // Exploración de UTF-8: caracteres de longitud variable en bytes
    let emoji = "🦀";
    println!("\nAnálisis UTF-8 de '{}':", emoji);
    println!("  Longitud en bytes (len()): {} bytes", emoji.len());
    println!(
        "  Iteración por caracteres (.chars()): {} elemento(s)",
        emoji.chars().count()
    );
    for (i, c) in emoji.chars().enumerate() {
        println!("    Char [{i}]: '{c}'");
    }
    for (i, b) in emoji.bytes().enumerate() {
        println!("    Byte [{i}]: {:#04x}", b);
    }
}

fn demostrar_hashmaps_y_entry_api() {
    println!("\n--- 3. MAPAS HASH (HASHMAP<K, V>) Y ENTRY API ---");

    let mut inventario = HashMap::new();
    inventario.insert(String::from("Portátiles"), 15);
    inventario.insert(String::from("Monitores"), 8);

    // Acceso seguro mediante clave
    let clave = "Portátiles";
    if let Some(&stock) = inventario.get(clave) {
        println!("Stock actual de '{clave}': {stock} unidades.");
    }

    // Inserción condicional usando la Entry API:
    // Solo inserta el valor si la clave NO está presente en el mapa
    inventario.entry(String::from("Teclados")).or_insert(25);
    inventario.entry(String::from("Monitores")).or_insert(50); // No sobreescribe el 8 existente

    println!("Inventario actualizado con entry(): {:?}", inventario);

    // Conteo de frecuencia de palabras en un texto
    let texto = "aprender rust es genial aprender sistemas con rust";
    let mut frecuencias = HashMap::new();

    for palabra in texto.split_whitespace() {
        let contador = frecuencias.entry(palabra).or_insert(0);
        *contador += 1;
    }

    println!("Frecuencia de palabras: {:?}", frecuencias);
}

fn demostrar_layout_memoria() {
    println!("\n--- 4. BAJO EL CAPÓ: LAYOUT EN MEMORIA STACK ---");

    // En arquitecturas de 64 bits, tanto Vec<T> como String ocupan 24 bytes en el Stack:
    // [ Puntero al Heap (8 bytes) | Capacidad (8 bytes) | Longitud (8 bytes) ]
    println!(
        "Tamaño de Vec<i32> en Stack: {} bytes",
        size_of::<Vec<i32>>()
    );
    println!("Tamaño de String en Stack:   {} bytes", size_of::<String>());
    println!(
        "Tamaño de HashMap en Stack:  {} bytes",
        size_of::<HashMap<String, i32>>()
    );
}
