// exercises/ex15_smart_pointers/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 15.

use std::cell::RefCell;
use std::rc::Rc;

/// Una lista enlazada recursiva inmutable clásica (Cons List) del Heap usando `Box`.
#[derive(Debug, Clone, PartialEq)]
pub enum ListaCons {
    Cons(i32, Box<ListaCons>),
    Nil,
}

impl ListaCons {
    /// Inicializa una lista vacía (`Nil`).
    pub fn nueva() -> Self {
        ListaCons::Nil
    }

    /// Añade un elemento al frente de la lista y devuelve la nueva lista.
    pub fn prepended(self, valor: i32) -> Self {
        ListaCons::Cons(valor, Box::new(self))
    }

    /// Calcula la longitud (número de elementos) de la lista recursivamente.
    pub fn longitud(&self) -> usize {
        match self {
            ListaCons::Cons(_, cola) => 1 + cola.longitud(),
            ListaCons::Nil => 0,
        }
    }
}

/// Nodo de un grafo con propiedad compartida y mutabilidad interna en el montón.
/// Demuestra el uso combinado de `Rc<T>` (referencias múltiples) y `RefCell<T>` (mutabilidad interna).
pub struct NodoGrafo {
    pub valor: RefCell<i32>,
    pub vecinos: RefCell<Vec<Rc<NodoGrafo>>>,
}

impl NodoGrafo {
    /// Crea un nuevo nodo de grafo independiente.
    pub fn nuevo(valor: i32) -> Rc<Self> {
        Rc::new(Self {
            valor: RefCell::new(valor),
            vecinos: RefCell::new(Vec::new()),
        })
    }

    /// Agrega una conexión unidireccional hacia otro nodo.
    pub fn agregar_vecino(nodo: &Rc<Self>, vecino: &Rc<Self>) {
        nodo.vecinos.borrow_mut().push(Rc::clone(vecino));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lista_cons_box() {
        let lista = ListaCons::nueva()
            .prepended(3)
            .prepended(2)
            .prepended(1);
        
        assert_eq!(lista.longitud(), 3);
    }

    #[test]
    fn test_grafo_rc_refcell() {
        let n1 = NodoGrafo::nuevo(10);
        let n2 = NodoGrafo::nuevo(20);

        NodoGrafo::agregar_vecino(&n1, &n2);

        // Validamos la mutabilidad interna de n2 desde n1 a través de Rc y RefCell
        {
            let vecino = &n1.vecinos.borrow()[0];
            *vecino.valor.borrow_mut() = 99;
        }

        assert_eq!(*n2.valor.borrow(), 99);
    }
}
