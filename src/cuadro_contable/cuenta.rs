use std::fmt::Display;

use super::asiento::Asiento;

#[derive(PartialEq, Debug)]
pub struct CuentaError;

/// Representa una cuenta
#[derive(PartialEq, Debug)]
pub struct Cuenta {
    /// El nombre de la cuenta, que debe ser único.
    pub nombre: String,
    /// El código de la cuenta, que debe ser único e informa también del grupo al que pertence.
    pub codigo: String,
    /// Los importes del debe
    pub debe: Vec<f64>,
    /// Los importes del haber
    pub haber: Vec<f64>,
    /// El saldo de la cuenta
    pub saldo: f64,

}

impl Display for Cuenta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        
        if let Some(w) = f.width() { // Si se le pasa ancho, rellena la fila completa

            // Cadena de saldo
            let saldo_str = format!("{:.2} €", self.saldo);

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
            write!(f, "({}) {} ~ {:.2} €", self.codigo, self.nombre, self.saldo)?;
        }

        Ok(())
    }
}

impl Cuenta {
    /// Crea una nueva cuenta con saldo cero.
    pub fn new(nombre: &str, codigo: &str) -> Cuenta {
        Cuenta {
            nombre: String::from(nombre),
            codigo: String::from(codigo),
            debe: vec![],
            haber: vec![],
            saldo: 0.00
        }
    }

    /// Incrementa el saldo
    pub fn incrementar_saldo(&mut self, importe: f64) {
        self.saldo += importe;
    } 

    /// Reduce el saldo
    pub fn reducir_saldo(&mut self, importe: f64) {
        self.saldo -= importe;
    } 

    /// Devuelve el nombre de la cuenta
    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }

    /// Devuelve el código de la cuenta
    pub fn codigo(&self) -> String {
        self.codigo.clone()
    }

    /// Tomando un libro diario como argumento, completa sus campos restantes: debe, haber y saldo
    pub fn mayorizar_cuenta(&mut self, libro_diario: &Vec<Asiento>) {

        // Resetea los campos que va a modificar
        self.debe = vec![];
        self.haber = vec![];
        self.saldo = 0.00;

        // Repasa los asientos y mayoriza
        for asiento in libro_diario {
            for m in asiento.debe() {
                if self.codigo == m.codigo_cuenta() {
                    self.debe.push(m.importe());
                    self.incrementar_saldo(m.importe());
                }
            };
            for m in asiento.haber() {
                if self.codigo == m.codigo_cuenta() {
                    self.haber.push(m.importe());
                    self.reducir_saldo(m.importe());
                }
            };
        }
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
            saldo: 0.00,
        }
    }

    #[test]
    fn new_crea_nueva_cuenta() {
        let cuenta = Cuenta::new("Cuenta 1", "101");

        assert_eq!(cuenta, Cuenta {
            nombre: "Cuenta 1".to_string(),
            codigo: "101".to_string(),
            debe: vec![],
            haber: vec![],
            saldo: 0.00,
        })
    }

    #[test]
    fn incrementar_saldo() {
        let mut cuenta = setup_cuenta();

        cuenta.incrementar_saldo(20.05);

        assert_eq!(cuenta.saldo, 20.05);
    }

    #[test]
    fn reducir_saldo() {
        let mut cuenta = setup_cuenta();

        cuenta.reducir_saldo(20.05);
        assert_eq!(cuenta.saldo, -20.05);
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
