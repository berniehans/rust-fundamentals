// exercises/ex03_common_concepts/src/lib.rs
// Crate de ejercicios lógicos prácticos para el Capítulo 03.

/// Convierte una temperatura entre escalas termodinámicas (Celsius 'C', Fahrenheit 'F', Kelvin 'K').
/// Valida de forma estricta que la temperatura no sea inferior al cero absoluto físico
/// (-273.15 °C, -459.67 °F, 0.0 K). En caso de violación, la función entra en pánico.
///
/// # Parámetros
/// * `valor` - La magnitud numérica de la temperatura a convertir.
/// * `origen` - Carácter que indica la escala de origen ('C', 'F', 'K').
/// * `destino` - Carácter que indica la escala de destino ('C', 'F', 'K').
///
/// # Retorno
/// * `f64` - El valor de la temperatura convertida en la escala especificada.
///
/// # Panics
/// La función causará un pánico si el valor ingresado es menor al cero absoluto en la escala de origen,
/// o si se especifican escalas con caracteres distintos a 'C', 'F' o 'K'.
///
/// # Ejemplos
/// ```
/// use ex03_common_concepts::convertir_temperatura;
/// let resultado = convertir_temperatura(100.0, 'C', 'F');
/// assert!((resultado - 212.0).abs() < 1e-9);
/// ```
pub fn convertir_temperatura(valor: f64, origen: char, destino: char) -> f64 {
    // 1. Validar validez de las escalas de temperatura
    let origen_upper = origen.to_ascii_uppercase();
    let destino_upper = destino.to_ascii_uppercase();

    let es_valido = |c| c == 'C' || c == 'F' || c == 'K';
    if !es_valido(origen_upper) || !es_valido(destino_upper) {
        panic!("Escala de temperatura inválida. Use 'C', 'F' o 'K'.");
    }

    // 2. Validar que no se viole el Cero Absoluto
    match origen_upper {
        'C' if valor < -273.15 => panic!("Temperatura inferior al cero absoluto (-273.15 °C)"),
        'F' if valor < -459.67 => panic!("Temperatura inferior al cero absoluto (-459.67 °F)"),
        'K' if valor < 0.0 => panic!("Temperatura inferior al cero absoluto (0.0 K)"),
        _ => {}
    }

    // 3. Expresión de conversión: convertir primero a Celsius como escala intermedia
    let celsius = match origen_upper {
        'C' => valor,
        'F' => (valor - 32.0) * 5.0 / 9.0,
        'K' => valor - 273.15,
        _ => unreachable!(),
    };

    // 4. Convertir de Celsius a la escala destino requerida
    match destino_upper {
        'C' => celsius,
        'F' => celsius * 9.0 / 5.0 + 32.0,
        'K' => celsius + 273.15,
        _ => unreachable!(),
    }
}

/// Procesa un array de lecturas analógicas de tamaño fijo en el Stack y retorna estadísticas descriptivas.
/// Demuestra el uso de bucles seguros, arrays fijos y desestructuración de tuplas en el Stack.
///
/// # Parámetros
/// * `lecturas` - Un array contiguo de 8 enteros de 32 bits (`[i32; 8]`) asignado en el Stack.
/// * `umbral` - El valor mínimo a partir del cual se considera una lectura como crítica.
///
/// # Retorno
/// * Una tupla conteniendo: `(minimo: i32, maximo: i32, promedio: f64, lecturas_criticas: usize)`
///
/// # Ejemplos
/// ```
/// use ex03_common_concepts::analizar_lecturas;
/// let datos = [12, 15, 8, 20, 25, 30, 5, 18];
/// let (min, max, prom, criticas) = analizar_lecturas(datos, 15);
/// assert_eq!(min, 5);
/// assert_eq!(max, 30);
/// assert_eq!(criticas, 4); // 18, 20, 25, 30 superan el umbral 15
/// ```
pub fn analizar_lecturas(lecturas: [i32; 8], umbral: i32) -> (i32, i32, f64, usize) {
    let mut minimo = lecturas[0];
    let mut maximo = lecturas[0];
    let mut sumatoria: i64 = 0; // Usamos i64 para prevenir desbordamientos físicos en sumas
    let mut criticas = 0;

    // Iteración segura sobre el array en memoria
    for &lectura in lecturas.iter() {
        if lectura < minimo {
            minimo = lectura;
        }
        if lectura > maximo {
            maximo = lectura;
        }
        sumatoria += lectura as i64;
        if lectura > umbral {
            criticas += 1;
        }
    }

    // Retorna la tupla directamente como una expresión de bloque
    let promedio = sumatoria as f64 / 8.0;
    (minimo, maximo, promedio, criticas)
}

/// Simula un acumulador que procesa una secuencia multiplicativa usando shadowing y expresiones de bloque.
/// Detiene la ejecución antes de que ocurra un desbordamiento físico de enteros (Integer Overflow)
/// en el tipo de dato `u8` (límite 255), devolviendo el último valor seguro y las iteraciones realizadas.
///
/// # Parámetros
/// * `base` - Valor inicial del acumulador.
/// * `multiplicador` - El factor de escala multiplicativo en cada ciclo.
///
/// # Retorno
/// * Una tupla conteniendo: `(ultimo_valor_seguro: u8, iteraciones_realizadas: usize)`
///
/// # Ejemplos
/// ```
/// use ex03_common_concepts::acumulador_con_limite;
/// let (valor, it) = acumulador_con_limite(3, 3);
/// assert_eq!(valor, 243); // 3 -> 9 -> 27 -> 81 -> 243. Siguiente (729) excede 255.
/// assert_eq!(it, 4);
/// ```
pub fn acumulador_con_limite(base: u8, multiplicador: u8) -> (u8, usize) {
    let mut acumulador = base;
    let mut iteraciones = 0;

    loop {
        // Bloque de expresión para calcular la siguiente operación de forma segura
        let siguiente = {
            let temp = acumulador.checked_mul(multiplicador);
            match temp {
                Some(val) => val,
                None => break, // Desbordamiento detectado de forma segura: salimos del bucle
            }
        };

        // Evitar bucles infinitos si el valor deja de cambiar (multiplicaciones por 1 o base 0)
        if siguiente == acumulador {
            break;
        }

        // Shadowing conceptual de la variable con el nuevo estado verificado
        acumulador = siguiente;
        iteraciones += 1;
    }

    (acumulador, iteraciones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convertir_temperatura_exito() {
        // Conversión Celsius a Fahrenheit (Agua hirviendo)
        let f = convertir_temperatura(100.0, 'C', 'F');
        assert!((f - 212.0).abs() < 1e-9);

        // Conversión Kelvin a Celsius (Cero absoluto)
        let c = convertir_temperatura(0.0, 'K', 'C');
        assert!((c - -273.15).abs() < 1e-9);

        // Conversión Fahrenheit a Kelvin
        let k = convertir_temperatura(-459.67, 'F', 'K');
        assert!((k - 0.0).abs() < 1e-9);
    }

    #[test]
    #[should_panic(expected = "Temperatura inferior al cero absoluto (-273.15 °C)")]
    fn test_convertir_temperatura_panic_celsius() {
        convertir_temperatura(-275.0, 'C', 'K');
    }

    #[test]
    #[should_panic(expected = "Temperatura inferior al cero absoluto (0.0 K)")]
    fn test_convertir_temperatura_panic_kelvin() {
        convertir_temperatura(-1.0, 'K', 'C');
    }

    #[test]
    #[should_panic(expected = "Escala de temperatura inválida")]
    fn test_convertir_temperatura_panic_escala_invalida() {
        convertir_temperatura(25.0, 'C', 'Z');
    }

    #[test]
    fn test_analizar_lecturas_exito() {
        let datos = [10, -5, 30, 25, 0, 15, 8, 45];
        // Mínimo = -5, Máximo = 45
        // Suma = 128, Promedio = 128 / 8 = 16.0
        // Umbral = 15 -> Críticos: 30, 25, 45 (3 elementos)
        let (min, max, prom, criticas) = analizar_lecturas(datos, 15);
        assert_eq!(min, -5);
        assert_eq!(max, 45);
        assert_eq!(prom, 16.0);
        assert_eq!(criticas, 3);
    }

    #[test]
    fn test_acumulador_con_limite_exito() {
        // Caso básico de crecimiento geométrico
        let (valor, it) = acumulador_con_limite(2, 2);
        assert_eq!(valor, 128); // 2 -> 4 -> 8 -> 16 -> 32 -> 64 -> 128. 256 excede u8.
        assert_eq!(it, 6);

        // Multiplicador de 1: debe detenerse inmediatamente para evitar bucle infinito
        let (valor, it) = acumulador_con_limite(10, 1);
        assert_eq!(valor, 10);
        assert_eq!(it, 0);

        // Multiplicador de 0: debe correr 1 iteración y luego detenerse al no cambiar (0 * 0 = 0)
        let (valor, it) = acumulador_con_limite(5, 0);
        assert_eq!(valor, 0);
        assert_eq!(it, 1);
    }
}
