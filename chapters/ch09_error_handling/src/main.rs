// ch09_error_handling - Demostración Educativa del Manejo de Errores
// Este archivo profundiza en Result<T, E>, panic!, el operador '?', ErrorKind y tipos de error propios.

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// 1. Definición de un Tipo de Error Personalizado
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum ErrorConfiguracion {
    ArchivoInvalido(String),
    PuertoInvalido(u32),
    IoError(String),
}

impl fmt::Display for ErrorConfiguracion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorConfiguracion::ArchivoInvalido(msg) => write!(f, "Error en archivo: {msg}"),
            ErrorConfiguracion::PuertoInvalido(puerto) => {
                write!(f, "Puerto fuera de rango válido (1024-65535): {puerto}")
            }
            ErrorConfiguracion::IoError(msg) => write!(f, "Error de E/S de bajo nivel: {msg}"),
        }
    }
}

impl Error for ErrorConfiguracion {}

// Conversión automática de std::io::Error a nuestro ErrorConfiguracion usando el trait From
impl From<io::Error> for ErrorConfiguracion {
    fn from(err: io::Error) -> Self {
        ErrorConfiguracion::IoError(err.to_string())
    }
}

fn main() {
    println!("=== CAPÍTULO 09: MANEJO DE ERRORES (PANIC! Y RESULT) ===");

    demostrar_manejo_result_basico();
    demostrar_operador_propagacion();
    demostrar_errores_personalizados();

    println!("\n¡Capítulo 09 ejecutado con éxito!");
}

fn demostrar_manejo_result_basico() {
    println!("\n--- 1. MANEJO DE RESULT CON MATCH Y ERROR_KIND ---");

    let nombre_archivo = "archivo_inexistente_ejemplo.txt";
    let archivo_resultado = File::open(nombre_archivo);

    match archivo_resultado {
        Ok(archivo) => {
            println!("Archivo abierto con éxito: {:?}", archivo);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!(
                    "Archivo '{}' no encontrado (ErrorKind::NotFound). Manejo controlado sin pánico.",
                    nombre_archivo
                );
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "Error de permisos al intentar acceder a '{}'.",
                    nombre_archivo
                );
            }
            otro => {
                println!("Error inesperado de E/S: {:?}", otro);
            }
        },
    }

    // Métodos de conveniencia: unwrap_or y unwrap_or_else
    fn consultar_puerto_remoto() -> Result<u16, &'static str> {
        Err("puerto no disponible en red")
    }

    let puerto_defecto = consultar_puerto_remoto().unwrap_or(8080);
    println!("Puerto obtenido con unwrap_or(): {puerto_defecto}");
}

// Función que demuestra el operador '?' para propagación limpia de errores
fn leer_nombre_usuario_desde_archivo(ruta: &str) -> Result<String, io::Error> {
    let mut archivo = File::open(ruta)?; // Si falla, retorna Err(e) inmediatamente
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;
    Ok(contenido.trim().to_string())
}

fn demostrar_operador_propagacion() {
    println!("\n--- 2. PROPAGACIÓN ERGONÓMICA CON EL OPERADOR '?' ---");

    match leer_nombre_usuario_desde_archivo("usuario_inexistente.txt") {
        Ok(usuario) => println!("Usuario leído: '{usuario}'"),
        Err(e) => println!("Error propagado limpiamente mediante '?': {e}"),
    }
}

// Función que valida una configuración y retorna nuestro ErrorConfiguracion
fn validar_puerto_servidor(puerto: u32) -> Result<u16, ErrorConfiguracion> {
    if (1024..=65535).contains(&puerto) {
        Ok(puerto as u16)
    } else {
        Err(ErrorConfiguracion::PuertoInvalido(puerto))
    }
}

fn demostrar_errores_personalizados() {
    println!("\n--- 3. TIPOS DE ERROR PERSONALIZADOS Y TRAIT STD::ERROR::ERROR ---");

    let intentos = [8080, 80, 443, 9000, 70000];

    for &puerto in &intentos {
        match validar_puerto_servidor(puerto) {
            Ok(p) => println!("  [OK] Puerto {p} es válido y seguro para producción."),
            Err(e) => println!("  [FALLO] Rechazado: {e}"),
        }
    }
}
