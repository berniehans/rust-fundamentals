// ch01_getting_started - Punto de Entrada Educativo
// Este archivo sirve como el primer programa ejecutable dentro del workspace "rust-fundamentals".

// La palabra clave 'fn' se utiliza para declarar una función en Rust.
// Rust requiere que todo programa ejecutable tenga una función llamada 'main' sin parámetros.
// Esta función actúa como el entrypoint del sistema operativo al lanzar el proceso binario.
fn main() {
    // Aquí invocamos a la macro 'println!'. 
    // NOTA CLAVE: El símbolo de exclamación '!' indica que esto es una MACRO y no una función ordinaria.
    // A diferencia de una función, la macro procesa y valida sus argumentos en tiempo de compilación.
    // Esto garantiza que cualquier formateo incorrecto se detecte antes de generar el código de máquina.
    //
    // El argumento pasado es una referencia a una cadena literal estática (&str): "Hello, world!".
    // El punto y coma ';' al final de la línea convierte esta expresión de impresión en una sentencia (statement),
    // indicando al compilador de Rust que finalice la ejecución de esta instrucción.
    println!("¡Hola, mundo desde Rust! (Capítulo 01: Getting Started)");

    // Demostración didáctica adicional:
    // Rust es un lenguaje fuertemente tipado. Declaramos una variable inmutable básica para imprimir en consola.
    // La macro println! nos permite interpolar valores de forma segura utilizando llaves '{}'.
    // Durante la compilación, Rust comprobará que el número de llaves '{}' coincida exactamente con
    // el número de argumentos que le siguen a la cadena de formato.
    let capitulo = 1;
    println!("Estás aprendiendo el Capítulo {:02} con rigor de nivel de sistemas.", capitulo);
}
