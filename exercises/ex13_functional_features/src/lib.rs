// exercises/ex13_functional_features/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 13.

/// Filtra los números enteros de un vector que sean mayores o iguales a un umbral dado,
/// y eleva cada uno de ellos al cuadrado. Demuestra el uso de adaptadores de iteradores.
///
/// # Ejemplos
/// ```
/// use ex13_functional_features::filtrar_y_elevar_cuadrado;
/// let numeros = vec![1, 2, 3, 4, 5];
/// let resultado = filtrar_y_elevar_cuadrado(numeros, 3);
/// assert_eq!(resultado, vec![9, 16, 25]);
/// ```
pub fn filtrar_y_elevar_cuadrado(numeros: Vec<i32>, umbral: i32) -> Vec<i32> {
    numeros
        .into_iter()
        .filter(|&x| x >= umbral)
        .map(|x| x * x)
        .collect()
}

/// Estructura que envuelve una lógica de filtro utilizando closures flexibles (`FnMut`).
pub struct FiltroPersonalizado;

impl FiltroPersonalizado {
    /// Filtra los elementos de un vector aplicando un predicado dinámico (closure `FnMut`).
    ///
    /// # Ejemplos
    /// ```
    /// use ex13_functional_features::FiltroPersonalizado;
    /// let filtro = FiltroPersonalizado;
    /// let datos = vec![1, 2, 3, 4, 5, 6];
    /// let pares = filtro.filtrar(datos, |x| x % 2 == 0);
    /// assert_eq!(pares, vec![2, 4, 6]);
    /// ```
    pub fn filtrar<F>(&self, coleccion: Vec<i32>, mut predicado: F) -> Vec<i32>
    where
        F: FnMut(i32) -> bool,
    {
        let mut filtrado = Vec::new();
        for item in coleccion {
            if predicado(item) {
                filtrado.push(item);
            }
        }
        filtrado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtrar_y_elevar_cuadrado() {
        let v = vec![10, -2, 5, 0, 7];
        // Umbral = 5 -> Filtrados: 10, 5, 7. Cuadrados: 100, 25, 49.
        assert_eq!(filtrar_y_elevar_cuadrado(v, 5), vec![100, 25, 49]);
    }

    #[test]
    fn test_filtro_personalizado_closure() {
        let filtro = FiltroPersonalizado;
        let v = vec![1, 3, 5, 10, 15];
        
        // Filtrar números divisibles por 5
        let res = filtro.filtrar(v, |x| x % 5 == 0);
        assert_eq!(res, vec![5, 10, 15]);
    }
}
