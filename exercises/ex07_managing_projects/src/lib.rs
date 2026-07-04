// exercises/ex07_managing_projects/src/lib.rs
// Crate de ejercicios prácticos para el Capítulo 07.

/// Módulo de gestión del inventario físico de productos.
pub mod almacen {
    /// Representa un producto en la base de datos física del almacén.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Producto {
        pub nombre: String,        // Nombre expuesto al exterior
        cantidad: u32,             // Inventario privado del almacén
    }

    impl Producto {
        /// Constructor público para inicializar el producto con su stock inicial.
        pub fn nuevo(nombre: &str, cantidad: u32) -> Self {
            Self {
                nombre: String::from(nombre),
                cantidad,
            }
        }

        /// Getter público para leer de forma segura el stock privado.
        pub fn consultar_existencias(&self) -> u32 {
            self.cantidad
        }

        /// Intenta restar existencias del almacén tras una venta.
        /// Retorna `Ok` si el stock es suficiente, o `Err` si no hay suficientes existencias.
        pub fn descontar_existencias(&mut self, cantidad_venta: u32) -> Result<(), &'static str> {
            if self.cantidad < cantidad_venta {
                return Err("Existencias insuficientes en almacén.");
            }
            self.cantidad -= cantidad_venta;
            Ok(())
        }
    }
}

/// Módulo de gestión de ventas y procesamiento de pedidos de clientes.
pub mod ventas {
    // Importamos Producto para usarlo cómodamente dentro del módulo
    use super::almacen::Producto;

    /// Procesa el pedido de un cliente descontando el inventario correspondiente.
    /// Demuestra la invocación inter-módulos calificada.
    ///
    /// # Ejemplos
    /// ```
    /// use ex07_managing_projects::almacen::Producto;
    /// use ex07_managing_projects::ventas::procesar_pedido;
    /// let mut prod = Producto::nuevo("Teclado Mecánico", 10);
    /// let result = procesar_pedido(&mut prod, 3);
    /// assert!(result.is_ok());
    /// assert_eq!(prod.consultar_existencias(), 7);
    /// ```
    pub fn procesar_pedido(producto: &mut Producto, cantidad: u32) -> Result<(), &'static str> {
        producto.descontar_existencias(cantidad)
    }
}

#[cfg(test)]
mod tests {
    use super::almacen::Producto;
    use super::ventas::procesar_pedido;

    #[test]
    fn test_gestion_existencias_almacen() {
        let mut p = Producto::nuevo("Monitor 4K", 5);
        assert_eq!(p.consultar_existencias(), 5);

        assert!(p.descontar_existencias(2).is_ok());
        assert_eq!(p.consultar_existencias(), 3);

        assert!(p.descontar_existencias(4).is_err());
    }

    #[test]
    fn test_procesamiento_pedido_ventas() {
        let mut p = Producto::nuevo("Ratón Gaming", 8);
        assert!(procesar_pedido(&mut p, 5).is_ok());
        assert_eq!(p.consultar_existencias(), 3);

        assert!(procesar_pedido(&mut p, 4).is_err());
    }
}
