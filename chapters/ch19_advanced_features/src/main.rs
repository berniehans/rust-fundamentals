// ch19_advanced_features - Demostración Educativa de Características Avanzadas de Rust
// Este archivo profundiza en Unsafe Rust (punteros crudos, FFI), traits avanzados (tipos asociados,
// sobrecarga de operadores, supertraits), tipos avanzados y macros declarativas.

use std::fmt;
use std::ops::Add;

// 1. Sobrecarga de Operadores mediante std::ops::Add
#[derive(Debug, Copy, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

impl Add for Punto {
    type Output = Punto;

    fn add(self, otro: Punto) -> Punto {
        Punto {
            x: self.x + otro.x,
            y: self.y + otro.y,
        }
    }
}

// 2. Trait con Tipos Asociados (Associated Types)
trait Analizador {
    type Entrada;
    type Salida;

    fn procesar(&self, dato: Self::Entrada) -> Self::Salida;
}

struct ParseadorNumerico;

impl Analizador for ParseadorNumerico {
    type Entrada = String;
    type Salida = Result<i32, std::num::ParseIntError>;

    fn procesar(&self, dato: Self::Entrada) -> Self::Salida {
        dato.trim().parse()
    }
}

// 3. Supertraits: Un trait que requiere que el tipo implemente Display
trait Esbozo: fmt::Display {
    fn imprimir_en_caja(&self) {
        let salida = self.to_string();
        let len = salida.len();
        println!("+---+");
        println!("| * | {}", salida);
        println!("+---+ (longitud: {len})");
    }
}

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Esbozo for Punto {}

// 4. Macro Declarativa personalizada
#[macro_export]
macro_rules! mi_vec {
    ( $( $x:expr ),* ) => {
        {
            #[allow(clippy::vec_init_then_push)]
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 5. Punteros de Función (Function Pointers 'fn')
fn duplicar(x: i32) -> i32 {
    x * 2
}

fn aplicar_transformacion(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn main() {
    println!("=== CAPÍTULO 19: CARACTERÍSTICAS AVANZADAS DE RUST ===");

    demostrar_unsafe_rust();
    demostrar_traits_avanzados();
    demostrar_punteros_de_funcion_y_macros();

    println!("\n¡Capítulo 19 ejecutado con éxito!");
}

fn demostrar_unsafe_rust() {
    println!("\n--- 1. UNSAFE RUST: PUNTEROS CRUDOS Y DESREFERENCIACIÓN ---");

    let mut valor: i32 = 42;

    // Crear punteros crudos es una operación completamente segura en código Safe
    let p_const = &valor as *const i32;
    let p_mut = &mut valor as *mut i32;

    println!("Dirección de memoria física de valor: {:p}", p_const);

    // Desreferenciar o modificar memoria a través de punteros crudos REQUIERE un bloque unsafe
    unsafe {
        println!("Lectura a través de *const: {}", *p_const);
        *p_mut = 100;
        println!(
            "Valor modificado directamente en memoria física: {}",
            *p_mut
        );
    }
    println!("Valor comprobado en Rust seguro: {valor}");

    // FFI externa (Foreign Function Interface) con la biblioteca estándar de C
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        let abs_c = abs(-99);
        println!("Invocación de función externa en C 'abs(-99)': {abs_c}");
    }
}

fn demostrar_traits_avanzados() {
    println!("\n--- 2. SOBRECARGA DE OPERADORES, TIPOS ASOCIADOS Y SUPERTRAITS ---");

    // Sobrecarga de operador '+'
    let p1 = Punto { x: 1, y: 2 };
    let p2 = Punto { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("Suma sobrecargada p1 + p2: {:?}", p3);

    // Supertrait
    p3.imprimir_en_caja();

    // Tipo asociado
    let parser = ParseadorNumerico;
    let resultado = parser.procesar(String::from("  12345  "));
    println!("Tipo asociado procesado con éxito: {:?}", resultado);
}

#[allow(clippy::vec_init_then_push)]
fn demostrar_punteros_de_funcion_y_macros() {
    println!("\n--- 3. PUNTEROS DE FUNCIÓN Y MACROS DECLARATIVAS ---");

    // Puntero de función (fn type)
    let res = aplicar_transformacion(duplicar, 25);
    println!("Puntero de función fn(i32) -> i32 aplicado a 25: {res}");

    // Macro declarativa construyendo un vector
    let vector_macro = mi_vec![10, 20, 30, 40];
    println!(
        "Vector creado mediante macro declarativa 'mi_vec!': {:?}",
        vector_macro
    );
}
