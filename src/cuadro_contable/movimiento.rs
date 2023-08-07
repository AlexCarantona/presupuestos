use super::cuenta::Cuenta;

/// Representa un movimiento
#[derive(PartialEq, Debug)]
pub struct Movimiento<'a> {
    importe: f64,
    cuenta: &'a Cuenta,
}

impl Movimiento<'_> {

    /// Almacena un movimiento con importe y cuenta.
    pub fn new(importe: f64, cuenta: &Cuenta) -> Movimiento {

        Movimiento { importe, cuenta }
    }

    /// Devuelve el importe.
    pub fn importe(&self) -> f64 {
        self.importe
    }

    /// Devuelve la cuenta.
    pub fn cuenta(&self) -> &Cuenta {

        self.cuenta
    }

}