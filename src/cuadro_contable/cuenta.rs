use std::fmt::Display;
use super::masa::Masa;

#[derive(PartialEq, Debug)]
pub struct CuentaError;

/// Representa una cuenta
#[derive(PartialEq, Debug)]
pub struct Cuenta {
    /// El nombre de la cuenta, que debe ser único.
    nombre: String,
    /// El código de la cuenta, que debe ser único e informa también del grupo al que pertence.
    codigo: String,
    /// Los importes del debe
    debe: Vec<f64>,
    /// Los importes del haber
    haber: Vec<f64>,
    /// El saldo deudor
    saldo_deudor: f64,
    /// El saldo acreedor
    saldo_acreedor: f64,
    /// Masa
    masa: Masa

}

impl Display for Cuenta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        
        if let Some(w) = f.width() { // Si se le pasa ancho, rellena la fila completa

            // Cadena de saldo
            let saldo_str = format!("{:.2} €", self.saldo_deudor - self.saldo_acreedor);

            // Cadena de código y nombre
            let codigo_nombre_str = format!("({}) {}", self.codigo, self.nombre);

            // Si el ancho es suficiente, deja espacio a los puntos intermedios
            if w >= codigo_nombre_str.len() + 1 {
                write!(f,"{}{:.>width$}", codigo_nombre_str, saldo_str, width=w - codigo_nombre_str.len())?;
            } else { // En caso contrario, imprime con espacio
                write!(f, "{} {}", codigo_nombre_str, saldo_str)?;
            }

        } else {
            // Formato estándar
            write!(f, "({}) {} ~ {:.2} €", self.codigo, self.nombre, self.saldo_deudor - self.saldo_acreedor)?;
        }

        Ok(())
    }
}

impl Cuenta {
    /// Crea una nueva cuenta con saldo cero.
    pub fn new(nombre: &str, codigo: &str, masa: Masa) -> Cuenta {
        Cuenta {
            nombre: String::from(nombre),
            codigo: String::from(codigo),
            debe: vec![],
            haber: vec![],
            saldo_deudor: 0.00,
            saldo_acreedor: 0.00,
            masa,
        }
    }

    /// Incrementa el saldo por el debe (carga la cuenta)
    pub fn saldo_deudor(&mut self, importe: f64) {
        self.saldo_deudor += importe;
    } 

    /// Reduce el saldo
    pub fn saldo_acreedor(&mut self, importe: f64) {
        self.saldo_acreedor += importe;
    } 

    /// Devuelve el nombre de la cuenta
    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }

    /// Devuelve el código de la cuenta
    pub fn codigo(&self) -> String {
        self.codigo.clone()
    }

    /// Devuelve el saldo de la cuenta
    pub fn saldo(&self) -> f64 {
        self.saldo_deudor - self.saldo_acreedor
    }

}

#[cfg(test)]
mod cuenta_tests {

    use super::*;

    fn setup_cuenta() -> Cuenta {
        Cuenta {
            nombre: "test".to_string(),
            codigo: "0000".to_string(),
            debe: vec![],
            haber: vec![],
            saldo_deudor: 0.00,
            saldo_acreedor: 0.00,
            masa: Masa::ActivoCorriente,
        }
    }

    #[test]
    fn new_crea_nueva_cuenta() {
        let cuenta = Cuenta::new("Cuenta 1", "101", Masa::ActivoCorriente);

        assert_eq!(cuenta, Cuenta {
            nombre: "Cuenta 1".to_string(),
            codigo: "101".to_string(),
            debe: vec![],
            haber: vec![],
            saldo_deudor: 0.00,
            saldo_acreedor: 0.00,
            masa: Masa::ActivoCorriente
        })
    }

    #[test]
    fn saldo_deudor() {
        let mut cuenta = setup_cuenta();

        cuenta.saldo_deudor(20.05);

        assert_eq!(cuenta.saldo(), 20.05);
    }

    #[test]
    fn saldo_acreedor() {
        let mut cuenta = setup_cuenta();

        cuenta.saldo_acreedor(20.05);
        assert_eq!(cuenta.saldo(), -20.05);
    }

    #[test]
    fn nombre_clona_nombre_cuenta() {
        let cuenta = setup_cuenta();

        assert_eq!(cuenta.nombre(), "test".to_string());
    }

    #[test]
    fn codigo_clona_codigo_cuenta() {
        let cuenta = setup_cuenta();

        assert_eq!(cuenta.codigo(), "0000".to_string());
    }

    #[test]
    fn saldo_devuelve_saldo() {
        let cuenta = setup_cuenta();

        assert_eq!(cuenta.saldo(), 0.00);
    }

    #[test]
    fn display_muestra_codigo_nombre_y_saldo() {

        let cuenta = setup_cuenta();

        assert_eq!(cuenta.to_string(), "(0000) test ~ 0.00 €");
    }

    #[test]
    fn display_con_ancho_ocupa_todo_el_ancho() {

        let cuenta = setup_cuenta();

        assert_eq!(format!("{:width$}", cuenta, width=20), "(0000) test...0.00 €");

    }

    #[test]
    fn display_con_ancho_pero_sin_ancho_sufciente_imprime_estandar() {

        let cuenta = setup_cuenta();

        assert_eq!(format!("{:width$}", cuenta, width=10), "(0000) test 0.00 €");

    }
}
