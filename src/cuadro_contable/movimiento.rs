use super::Cuadro;

/// Representa un movimiento
#[derive(PartialEq, Debug)]
pub struct Movimiento {
    importe: f64,
    codigo_cuenta: String,
    nombre_cuenta: String,
}

impl Movimiento {

    /// Almacena un movimiento con importe y cuenta.
    pub fn new(importe: f64, codigo_cuenta: String) -> Movimiento {
        Movimiento { importe, codigo_cuenta, nombre_cuenta: String::new() }
    }

    /// Devuelve el importe.
    pub fn importe(&self) -> f64 {
        self.importe
    }

    /// Devuelve el nombre y código de la cuenta.
    pub fn cuenta(&self) -> String {
        format!("({}) {}", self.codigo_cuenta, self.nombre_cuenta)
    }

    /// Devuelve el codigo de la cuenta.
    pub fn codigo_cuenta(&self) -> String {
        self.codigo_cuenta.to_string()
    }

    /// Hidrata la cuenta, es decir, asigna su nombre al código.
    pub fn hidratar_cuenta(&mut self, cuadro: &Cuadro) {

        let nombre_cuenta = cuadro.find_cuenta(&self.codigo_cuenta);

        if let Some(v) = nombre_cuenta {
            self.nombre_cuenta = v.nombre()
        }
    }

}