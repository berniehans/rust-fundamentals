// ch12_minigrep - Demostración Educativa del Proyecto CLI Minigrep
// Este archivo implementa una herramienta CLI tipo grep con separación de responsabilidades,
// configuración basada en argumentos/variables de entorno, búsqueda sensible/insensible y manejo de I/O.

use std::env;
use std::error::Error;
use std::process;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub consulta: String,
    pub ruta_archivo: String,
    pub ignorar_mayusculas: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Ignoramos el primer argumento (el nombre del binario)
        args.next();

        let consulta = match args.next() {
            Some(arg) => arg,
            None => return Err("No se especificó el término de búsqueda."),
        };

        let ruta_archivo = match args.next() {
            Some(arg) => arg,
            None => return Err("No se especificó la ruta del archivo."),
        };

        // Verificamos si la variable de entorno IGNORE_CASE está configurada
        let ignorar_mayusculas = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            consulta,
            ruta_archivo,
            ignorar_mayusculas,
        })
    }
}

pub fn buscar<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str> {
    let mut resultados = Vec::new();
    for linea in contenido.lines() {
        if linea.contains(consulta) {
            resultados.push(linea);
        }
    }
    resultados
}

pub fn buscar_insensible<'a>(consulta: &str, contenido: &'a str) -> Vec<&'a str> {
    let consulta_minuscula = consulta.to_lowercase();
    let mut resultados = Vec::new();
    for linea in contenido.lines() {
        if linea.to_lowercase().contains(&consulta_minuscula) {
            resultados.push(linea);
        }
    }
    resultados
}

pub fn ejecutar_busqueda(
    config: &Config,
    texto_fuente: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let lineas_encontradas = if config.ignorar_mayusculas {
        buscar_insensible(&config.consulta, texto_fuente)
    } else {
        buscar(&config.consulta, texto_fuente)
    };

    Ok(lineas_encontradas.into_iter().map(String::from).collect())
}

fn main() {
    println!("=== CAPÍTULO 12: PROYECTO I/O (CLI MINIGREP) ===");

    let args: Vec<String> = env::args().collect();

    // Si no se pasaron argumentos por la CLI, usamos una demostración didáctica interactiva
    if args.len() < 3 {
        println!("Aviso: No se proporcionaron argumentos en la terminal.");
        println!("Ejecutando demostración integrada con datos en memoria:\n");

        let texto_ejemplo = "\
Rust:
rápido, confiable y seguro.
Elige tres.
Rendimiento extremo con seguridad de memoria.
Ductilidad de abstracciones cero-costo.";

        let config_demo = Config {
            consulta: String::from("seguro"),
            ruta_archivo: String::from("poema.txt"),
            ignorar_mayusculas: true,
        };

        println!("Configuración aplicada: {:?}", config_demo);
        println!("Texto fuente:\n---\n{}\n---", texto_ejemplo);

        match ejecutar_busqueda(&config_demo, texto_ejemplo) {
            Ok(coincidencias) => {
                println!("\nCoincidencias encontradas ({}) :", coincidencias.len());
                for (i, coincidencia) in coincidencias.iter().enumerate() {
                    println!("  [{}] {}", i + 1, coincidencia);
                }
            }
            Err(e) => {
                eprintln!("Error en la ejecución de búsqueda: {e}");
                process::exit(1);
            }
        }

        println!("\nPara probar con argumentos reales ejecuta:");
        println!("  cargo run -p ch12_minigrep -- <consulta> <archivo>");
        println!("O con variable de entorno en PowerShell:");
        println!("  $env:IGNORE_CASE=1; cargo run -p ch12_minigrep -- Rust README.md");
        return;
    }

    // Modo CLI real con argumentos de usuario
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema al parsear los argumentos: {err}");
        process::exit(1);
    });

    println!(
        "Buscando '{}' en el archivo '{}'...",
        config.consulta, config.ruta_archivo
    );

    // Leer el contenido del archivo especificado por el usuario
    let contenido = std::fs::read_to_string(&config.ruta_archivo).unwrap_or_else(|err| {
        eprintln!("Error al leer el archivo '{}': {err}", config.ruta_archivo);
        process::exit(1);
    });

    // Ejecutar la búsqueda y mostrar los resultados
    match ejecutar_busqueda(&config, &contenido) {
        Ok(coincidencias) => {
            if coincidencias.is_empty() {
                println!("No se encontraron coincidencias.");
            } else {
                println!("Coincidencias encontradas ({}):", coincidencias.len());
                for linea in &coincidencias {
                    println!("{linea}");
                }
            }
        }
        Err(e) => {
            eprintln!("Error en la búsqueda: {e}");
            process::exit(1);
        }
    }
}
