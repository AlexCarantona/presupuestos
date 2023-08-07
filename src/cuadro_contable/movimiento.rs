use std::fmt::Display;

/// Representa un movimiento
#[derive(PartialEq, Debug)]
pub struct Movimiento {
    importe: f64,
    cuenta_deudora: String,
    cuenta_acreedora: String,
}

impl Display for Movimiento {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
        {:<20} debe {:.2} € a {:>20}
        ", self.cuenta_deudora,
        self.importe,
        self.cuenta_acreedora
    )
    }
}

impl Movimiento {

    /// Almacena un movimiento con importe y nombres de sus cuentas participantes.
    pub fn new(importe: f64, cuenta_deudora: String, cuenta_acreedora: String) -> Movimiento {

        Movimiento { importe, cuenta_deudora, cuenta_acreedora }
    }

}