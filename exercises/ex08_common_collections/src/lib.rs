// exercises/ex08_common_collections/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 08.

use std::collections::HashMap;

/// Calcula la media, la mediana y la moda de una lista de enteros en el montón.
///
/// # Reglas
/// * **Media:** Promedio aritmético flotante de los números.
/// * **Mediana:** El valor central de la lista ordenada (si la lista es de tamaño par,
///   es el promedio flotante de los dos centrales).
/// * **Moda:** El valor que ocurre con mayor frecuencia. Si hay empate, retorna cualquiera.
///
/// # Ejemplos
/// ```
/// use ex08_common_collections::calcular_media_mediana_moda;
/// let datos = [1, 2, 2, 3, 4];
/// let (media, mediana, moda) = calcular_media_mediana_moda(&datos);
/// assert_eq!(media, 2.4);
/// assert_eq!(mediana, 2.0);
/// assert_eq!(moda, 2);
/// ```
pub fn calcular_media_mediana_moda(numeros: &[i32]) -> (f64, f64, i32) {
    if numeros.is_empty() {
        return (0.0, 0.0, 0);
    }

    // 1. Calcular Media
    let suma: i64 = numeros.iter().map(|&x| x as i64).sum();
    let media = suma as f64 / numeros.len() as f64;

    // 2. Calcular Mediana (Ordenando una copia del vector en el Heap)
    let mut v = numeros.to_vec();
    v.sort();
    let mediana = if v.len() % 2 == 1 {
        v[v.len() / 2] as f64
    } else {
        let mid = v.len() / 2;
        (v[mid - 1] as f64 + v[mid] as f64) / 2.0
    };

    // 3. Calcular Moda usando un HashMap
    let mut frecuencias = HashMap::new();
    for &num in numeros {
        let count = frecuencias.entry(num).or_insert(0);
        *count += 1;
    }

    let mut moda = numeros[0];
    let mut max_frecuencia = 0;
    for (num, count) in frecuencias {
        if count > max_frecuencia {
            max_frecuencia = count;
            moda = num;
        }
    }

    (media, mediana, moda)
}

/// Tokeniza una cadena de texto, limpia caracteres especiales y cuenta las frecuencias de palabras.
/// Demuestra el uso de `String`, `HashMap` y la `Entry API`.
///
/// # Ejemplos
/// ```
/// use ex08_common_collections::contar_frecuencia_palabras;
/// let freq = contar_frecuencia_palabras("Hola, hola, mundo.");
/// assert_eq!(*freq.get("hola").unwrap(), 2);
/// assert_eq!(*freq.get("mundo").unwrap(), 1);
/// ```
pub fn contar_frecuencia_palabras(texto: &str) -> HashMap<String, usize> {
    let mut freq_map = HashMap::new();

    // Dividimos por espacios en blanco
    for palabra in texto.split_whitespace() {
        // Limpiamos puntuaciones (ej: comas, puntos) y pasamos a minúsculas
        let limpia: String = palabra
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if !limpia.is_empty() {
            let count = freq_map.entry(limpia).or_insert(0);
            *count += 1;
        }
    }

    freq_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_mediana_moda_impar() {
        let datos = [3, 1, 4, 1, 5];
        // Ordenado: 1, 1, 3, 4, 5. Mediana = 3. Moda = 1.
        // Suma = 14, Media = 14 / 5 = 2.8
        let (media, mediana, moda) = calcular_media_mediana_moda(&datos);
        assert_eq!(media, 2.8);
        assert_eq!(mediana, 3.0);
        assert_eq!(moda, 1);
    }

    #[test]
    fn test_media_mediana_moda_par() {
        let datos = [1, 2, 3, 4];
        // Ordenado: 1, 2, 3, 4. Mediana = (2+3)/2 = 2.5.
        // Suma = 10, Media = 10 / 4 = 2.5.
        let (media, mediana, _moda) = calcular_media_mediana_moda(&datos);
        assert_eq!(media, 2.5);
        assert_eq!(mediana, 2.5);
    }

    #[test]
    fn test_frecuencia_palabras() {
        let t = "Rust es rápido. Sí, increíblemente rápido.";
        let res = contar_frecuencia_palabras(t);
        assert_eq!(res.get("rápido"), Some(&2));
        assert_eq!(res.get("rust"), Some(&1));
        assert_eq!(res.get("sí"), Some(&1));
    }
}
