// ch13_functional_features - Demostración Educativa de Características Funcionales
// Este archivo cubre closures (Fn, FnMut, FnOnce), captura de entorno con 'move', e iteradores perezosos con adaptadores.

// 1. Estructura personalizada que implementa el Trait Iterator
#[derive(Debug)]
struct Contador {
    actual: u32,
    limite: u32,
}

impl Contador {
    fn nuevo(limite: u32) -> Self {
        Self { actual: 0, limite }
    }
}

impl Iterator for Contador {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actual < self.limite {
            self.actual += 1;
            Some(self.actual)
        } else {
            None
        }
    }
}

fn main() {
    println!("=== CAPÍTULO 13: CARACTERÍSTICAS FUNCIONALES (CLOSURES E ITERADORES) ===");

    demostrar_closures_y_traits();
    demostrar_iteradores_basicos();
    demostrar_adaptadores_iteracion();
    demostrar_iterador_personalizado();

    println!("\n¡Capítulo 13 ejecutado con éxito!");
}

fn demostrar_closures_y_traits() {
    println!("\n--- 1. CLOSURES Y MODOS DE CAPTURA (FN, FNMUT, FNONCE) ---");

    // A. Captura por Préstamo Inmutable (&T) -> Trait `Fn`
    let factor = 10;
    let multiplicar = |x: i32| x * factor;
    println!("Closure Fn (lectura): 5 * 10 = {}", multiplicar(5));

    // B. Captura por Préstamo Mutable (&mut T) -> Trait `FnMut`
    let mut acumulador = 0;
    let mut sumar_al_acumulador = |delta: i32| {
        acumulador += delta;
        acumulador
    };
    println!("Closure FnMut llamada 1: {}", sumar_al_acumulador(15));
    println!("Closure FnMut llamada 2: {}", sumar_al_acumulador(25));

    // C. Captura por Consumo de Propiedad (T) -> Trait `FnOnce` con 'move'
    let texto_heap = String::from("Recurso en Heap");
    let consumir_recurso = move || {
        println!("Closure FnOnce: Consumiendo '{texto_heap}' por valor.");
    };
    consumir_recurso();
    // texto_heap ya no es accesible aquí
}

fn demostrar_iteradores_basicos() {
    println!("\n--- 2. LA NATURALEZA PEREZOSA (LAZINESS) DE LOS ITERADORES ---");

    let numeros = [1, 2, 3, 4, 5];

    // .iter() produce referencias (&T)
    let mut iterador = numeros.iter();

    println!("Invocando .next() manualmente:");
    println!("  Paso 1: {:?}", iterador.next()); // Some(&1)
    println!("  Paso 2: {:?}", iterador.next()); // Some(&2)
    println!("  Paso 3: {:?}", iterador.next()); // Some(&3)
}

fn demostrar_adaptadores_iteracion() {
    println!("\n--- 3. ADAPTADORES DE ITERACIÓN VS CONSUMIDORES ---");

    let valores = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Pipeline funcional: filtrar pares -> elevar al cuadrado -> recolectar
    let resultado: Vec<i32> = valores
        .iter()
        .filter(|&&x| x % 2 == 0) // Adaptador: filtra pares
        .map(|&x| x * x) // Adaptador: eleva al cuadrado
        .collect(); // Consumidor: evalúa y construye el vector

    println!("Vector filtrado y elevado al cuadrado: {:?}", resultado);

    // Consumidor .sum()
    let suma_cuadrados: i32 = resultado.iter().sum();
    println!("Suma agregada calculada con .sum(): {suma_cuadrados}");
}

fn demostrar_iterador_personalizado() {
    println!("\n--- 4. IMPLEMENTACIÓN PROPIA DEL TRAIT ITERATOR ---");

    let contador = Contador::nuevo(5);
    println!("Iterador personalizado Contador (1..=5):");

    for num in contador {
        println!("  Contador emitió: {num}");
    }

    // Combinación de dos iteradores Contador mediante .zip()
    let c1 = Contador::nuevo(3);
    let c2 = Contador::nuevo(3).skip(1);
    let pares: Vec<(u32, u32)> = c1.zip(c2).collect();
    println!("Pares generados con zip y skip: {:?}", pares);
}
