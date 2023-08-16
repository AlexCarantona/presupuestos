use std::fmt::Display;

use super::cuenta;
use super::masa;

/// Representa un movimiento.
/// Este almacena solo el código de cuenta, puesto que no es probable que las cuentas cambien como tales
/// y solo deben servir de referencia. Además, al guardarse mediante una referencia, se garantiza que existirán
/// en el momento de ir a guardarlas.
#[derive(PartialEq, Debug)]
pub struct Movimiento {
    importe: f64,
    codigo_cuenta: String,
    nombre_cuenta: String,
}

impl Movimiento {

    /// Almacena un movimiento con importe y código de cuenta, que toma de una referencia
    pub fn new(importe: f64, cuenta: &mut cuenta::Cuenta) -> Movimiento {
        Movimiento { 
            importe, 
            codigo_cuenta: cuenta.codigo(),
            nombre_cuenta: cuenta.nombre(),
        }
    }

    /// Devuelve el importe que figura en el movimiento
    pub fn importe(&self) -> f64 {
        self.importe
    }

}

impl Display for Movimiento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {} {:.2} €", self.codigo_cuenta, self.nombre_cuenta, self.importe)
    }
}

#[cfg(test)]
mod movimiento_tests {
    
    use super::*;

    #[test]
    fn new_crea_movimiento() {
        let mut cuenta = cuenta::Cuenta::new("test", "0000", masa::Masa::ActivoCorriente);
        let movimiento = Movimiento::new(23.07, &mut cuenta);

        assert_eq!(movimiento, Movimiento { 
            codigo_cuenta: "0000".to_string(), 
            nombre_cuenta: "test".to_string(), 
            importe: 23.07
        });
    }
}