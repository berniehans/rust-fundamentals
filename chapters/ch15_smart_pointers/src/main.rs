// ch15_smart_pointers - Demostración Educativa de Punteros Inteligentes
// Este archivo profundiza en Box<T>, Rc<T>, RefCell<T>, Deref Coercion, Drop y Weak<T>.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// 1. Tipo recursivo habilitado mediante Box<T>
#[allow(dead_code)]
#[derive(Debug)]
enum ListaCons {
    Cons(i32, Box<ListaCons>),
    Nil,
}

// 2. Puntero inteligente propio implementando Deref y Drop
#[derive(Debug)]
struct MiBox<T>(T);

impl<T> MiBox<T> {
    fn new(x: T) -> MiBox<T> {
        MiBox(x)
    }
}

impl<T> Deref for MiBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MiBox<T> {
    fn drop(&mut self) {
        println!("[Drop Hook]: Limpiando recurso MiBox de memoria.");
    }
}

// 3. Estructura de Nodo para grafos con Rc y RefCell (Propiedad compartida y mutabilidad interna)
#[allow(dead_code)]
#[derive(Debug)]
struct NodoArbol {
    valor: i32,
    padre: RefCell<Weak<NodoArbol>>,
    hijos: RefCell<Vec<Rc<NodoArbol>>>,
}

fn main() {
    println!("=== CAPÍTULO 15: PUNTEROS INTELIGENTES (SMART POINTERS) ===");

    demostrar_box_y_tipos_recursivos();
    demostrar_deref_y_drop();
    demostrar_rc_y_refcell();
    demostrar_weak_y_arboles();

    println!("\n¡Capítulo 15 ejecutado con éxito!");
}

fn demostrar_box_y_tipos_recursivos() {
    println!("\n--- 1. BOX<T> Y TIPOS RECURSIVOS ---");

    // Asignación explícita en Heap
    let b = Box::new(42);
    println!(
        "Valor en Heap apuntado por Box: {b}, desreferenciado: {}",
        *b
    );

    // Lista enlazada Cons (1 -> 2 -> 3 -> Nil)
    let lista = ListaCons::Cons(
        1,
        Box::new(ListaCons::Cons(
            2,
            Box::new(ListaCons::Cons(3, Box::new(ListaCons::Nil))),
        )),
    );
    println!("Estructura recursiva ListaCons en Heap: {:?}", lista);
}

fn demostrar_deref_y_drop() {
    println!("\n--- 2. DEREF COERCION Y EL TRAIT DROP ---");

    let mi_puntero = MiBox::new(String::from("Rust Smart Pointers"));

    // Desreferenciación gracias al trait Deref
    println!("Acceso mediante deref (*): {}", *mi_puntero);

    // Deref Coercion: &MiBox<String> se coerciona automáticamente a &str
    fn imprimir_slice(s: &str) {
        println!("Función recibió slice mediante Deref Coercion: '{s}'");
    }
    imprimir_slice(&mi_puntero);

    // Drop manual temprano mediante std::mem::drop
    drop(mi_puntero);
    println!("MiBox fue liberado explícitamente antes del final del scope.");
}

fn demostrar_rc_y_refcell() {
    println!("\n--- 3. RC<T> Y REFCELL<T> (MUTABILIDAD INTERNA COMPARTIDA) ---");

    // Rc habilita múltiples propietarios en un único hilo
    let valor_compartido = Rc::new(RefCell::new(100));

    let clon_a = Rc::clone(&valor_compartido);
    let clon_b = Rc::clone(&valor_compartido);

    println!(
        "Conteo de referencias fuertes inicial: {}",
        Rc::strong_count(&valor_compartido)
    );

    // Mutamos el valor a través de clon_a usando borrow_mut()
    *clon_a.borrow_mut() += 50;

    // Leemos el valor reflejado desde clon_b usando borrow()
    println!(
        "Valor tras mutar clon_a, leído desde clon_b: {}",
        *clon_b.borrow()
    );
    println!(
        "Conteo de referencias final: {}",
        Rc::strong_count(&valor_compartido)
    );
}

fn demostrar_weak_y_arboles() {
    println!("\n--- 4. WEAK<T> Y PREVENCIÓN DE CICLOS DE REFERENCIA ---");

    let hoja = Rc::new(NodoArbol {
        valor: 3,
        padre: RefCell::new(Weak::new()),
        hijos: RefCell::new(vec![]),
    });

    println!(
        "Hoja strong_count = {}, weak_count = {}",
        Rc::strong_count(&hoja),
        Rc::weak_count(&hoja)
    );

    {
        let rama = Rc::new(NodoArbol {
            valor: 5,
            padre: RefCell::new(Weak::new()),
            hijos: RefCell::new(vec![Rc::clone(&hoja)]),
        });

        // Apuntamos el padre de hoja a rama usando un puntero débil (Weak)
        *hoja.padre.borrow_mut() = Rc::downgrade(&rama);

        println!("Dentro del bloque:");
        println!(
            "  Rama strong_count = {}, weak_count = {}",
            Rc::strong_count(&rama),
            Rc::weak_count(&rama)
        );
        println!(
            "  Hoja strong_count = {}, weak_count = {}",
            Rc::strong_count(&hoja),
            Rc::weak_count(&hoja)
        );

        // Acceso seguro al padre desde la hoja con .upgrade()
        if let Some(padre_rc) = hoja.padre.borrow().upgrade() {
            println!(
                "  Padre de la hoja resuelto exitosamente: valor = {}",
                padre_rc.valor
            );
        }
    }

    // Rama sale de ámbito y su memoria se libera porque hoja solo tenía una referencia débil
    println!("Fuera del bloque:");
    println!(
        "  Padre de la hoja tras drop de rama: {:?}",
        hoja.padre.borrow().upgrade()
    );
}
