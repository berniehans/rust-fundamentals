// ch11_writing_tests - Demostración Educativa de Pruebas Automatizadas en Rust
// Este archivo explica el Test Harness integrado de Rust, aserciones, #[should_panic] y tests con Result.

#[derive(Debug, PartialEq)]
pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

impl Rectangulo {
    pub fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }

    pub fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}

pub fn sumar_dos(a: i32) -> i32 {
    a + 2
}

pub fn saludo_personalizado(nombre: &str) -> String {
    format!("¡Hola, {nombre}!")
}

pub fn procesar_adivinanza(valor: i32) -> i32 {
    if !(1..=100).contains(&valor) {
        panic!("El valor de la adivinanza debe estar entre 1 y 100, se recibió: {valor}");
    }
    valor
}

fn main() {
    println!("=== CAPÍTULO 11: ESCRIBIR PRUEBAS AUTOMATIZADAS ===");

    println!("Demostración interactiva de funciones a validar:");
    let r1 = Rectangulo {
        ancho: 30,
        alto: 50,
    };
    let r2 = Rectangulo {
        ancho: 10,
        alto: 20,
    };

    println!("  r1: {:?}, r2: {:?}", r1, r2);
    println!("  r1.puede_contener(&r2): {}", r1.puede_contener(&r2));
    println!("  sumar_dos(4): {}", sumar_dos(4));
    println!("  saludo: {}", saludo_personalizado("Rustaceo"));

    println!("\nPara correr la suite completa de pruebas unitarias de este módulo ejecuta:");
    println!("  cargo test -p ch11_writing_tests");
    println!("  cargo test -p ch11_writing_tests -- --show-output");

    println!("\n¡Capítulo 11 ejecutado con éxito!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangulo_puede_contener_menor() {
        let mayor = Rectangulo { ancho: 8, alto: 7 };
        let menor = Rectangulo { ancho: 5, alto: 1 };
        assert!(mayor.puede_contener(&menor));
    }

    #[test]
    fn test_rectangulo_no_puede_contener_mayor() {
        let mayor = Rectangulo { ancho: 8, alto: 7 };
        let menor = Rectangulo { ancho: 5, alto: 1 };
        assert!(!menor.puede_contener(&mayor));
    }

    #[test]
    fn test_sumar_dos_exito() {
        assert_eq!(sumar_dos(2), 4);
        assert_ne!(sumar_dos(2), 5);
    }

    #[test]
    fn test_saludo_mensaje_personalizado() {
        let res = saludo_personalizado("Elena");
        assert!(
            res.contains("Elena"),
            "El saludo no contenía el nombre esperado. Resultado obtenido: '{res}'"
        );
    }

    #[test]
    #[should_panic(expected = "debe estar entre 1 y 100")]
    fn test_adivinanza_fuera_de_rango_panic() {
        procesar_adivinanza(150);
    }

    #[test]
    fn test_con_tipo_result() -> Result<(), String> {
        let r = Rectangulo { ancho: 10, alto: 5 };
        if r.area() == 50 {
            Ok(())
        } else {
            Err(String::from("El cálculo del área es erróneo."))
        }
    }
}
