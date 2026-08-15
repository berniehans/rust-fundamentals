// ch07_managing_projects - Demostración Educativa de Módulos, Paquetes y Visibilidad
// Este archivo profundiza en la modularización jerárquica, encapsulación, visibilidad y reexportación.

// Definición de árbol modular en línea
pub mod restaurante {
    // Submódulo público: gestión de mesas y clientes
    pub mod recepcion {
        #[derive(Debug)]
        pub enum TipoMesa {
            Interior,
            Terraza,
        }

        pub fn asignar_mesa(tipo: &TipoMesa) {
            println!("[Recepción]: Asignando mesa en área: {:?}", tipo);
        }
    }

    // Submódulo privado al restaurante: cocina y suministros
    pub mod cocina {
        // Estructura pública con campos privados (encapsulación)
        #[derive(Debug)]
        pub struct PlatoPrincipal {
            pub nombre: String,          // Campo público: el cliente puede elegir el plato
            ingrediente_secreto: String, // Campo privado: solo accesible dentro de 'cocina'
        }

        impl PlatoPrincipal {
            // Constructor público necesario porque tiene campos privados
            pub fn nuevo(nombre: &str, secreto: &str) -> Self {
                Self {
                    nombre: String::from(nombre),
                    ingrediente_secreto: String::from(secreto),
                }
            }

            pub fn cocinar(&self) {
                // Dentro del módulo cocina tenemos acceso al campo privado
                println!(
                    "[Cocina]: Preparando plato '{}' con el toque secreto '{}'.",
                    self.nombre, self.ingrediente_secreto
                );
            }
        }

        // Función que usa 'super' para llamar a una función del módulo padre
        pub fn solicitar_limpieza() {
            println!("[Cocina]: Solicitando limpieza de estación.");
            super::mantenimiento::limpiar_estacion();
        }
    }

    // Módulo interno privado
    mod mantenimiento {
        pub(super) fn limpiar_estacion() {
            println!("[Mantenimiento]: Estación desinfectada según normativa.");
        }
    }
}

// Módulo de logística y facturación
pub mod logistica {
    pub mod envios {
        pub fn despachar_pedido(id: u64) {
            println!("[Logística]: Despachando pedido #{id} hacia el cliente.");
        }
    }
}

// Reexportación de símbolos para crear una API más ergonómica (pub use)
pub use crate::logistica::envios::despachar_pedido;
use crate::restaurante::cocina::PlatoPrincipal;
use crate::restaurante::recepcion::{TipoMesa, asignar_mesa};

fn main() {
    println!("=== CAPÍTULO 07: GESTIÓN DE PROYECTOS, MÓDULOS Y VISIBILIDAD ===");

    demostrar_rutas_absolutas_y_relativas();
    demostrar_privacidad_estructuras();
    demostrar_reexportacion_y_use();

    println!("\n¡Capítulo 07 ejecutado con éxito!");
}

fn demostrar_rutas_absolutas_y_relativas() {
    println!("\n--- 1. RUTAS ABSOLUTAS Y RELATIVAS EN EL ÁRBOL DE MÓDULOS ---");

    // Ruta absoluta mediante el alias 'use' traído al ámbito
    let mesa = TipoMesa::Terraza;
    asignar_mesa(&mesa);

    // Ruta absoluta explícita desde la raíz del crate
    crate::restaurante::recepcion::asignar_mesa(&TipoMesa::Interior);

    // Llamada interna que hace uso de 'super' entre módulos hermanos
    restaurante::cocina::solicitar_limpieza();
}

fn demostrar_privacidad_estructuras() {
    println!("\n--- 2. PRIVACIDAD EN ESTRUCTURAS VS ENUMS ---");

    // Las variantes de un enum público son públicas automáticamente:
    let opcion_mesa = restaurante::recepcion::TipoMesa::Interior;
    println!("Variante pública de Enum: {:?}", opcion_mesa);

    // Una estructura con campos privados NO se puede instanciar con la sintaxis de llaves
    // fuera de su módulo de origen; requiere un constructor público:
    let plato = PlatoPrincipal::nuevo("Salmón Ahumado", "Eneldo Nórdico");
    println!(
        "Plato creado mediante constructor público: '{}'",
        plato.nombre
    );
    plato.cocinar();
}

fn demostrar_reexportacion_y_use() {
    println!("\n--- 3. REEXPORTACIÓN (PUB USE) Y ALIAS ERGONÓMICOS ---");

    // Gracias al 'pub use', podemos llamar a 'despachar_pedido' directamente
    // sin tener que escribir la ruta anidada 'crate::logistica::envios::despachar_pedido'
    despachar_pedido(42001);
}
