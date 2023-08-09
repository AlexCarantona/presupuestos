use std::fmt::Display;

use super::asiento::Asiento;



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
    /// El saldo de la cuenta
    saldo: f64,

}

impl Display for Cuenta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        if self.debe.len() > 0 || self.haber.len() > 0 {
            write!(f, "({}) {} ~ {:.2} €\n", self.codigo, self.nombre, self.saldo)?;
            write!(f, "\n")?;

            let mut debe_iter = self.debe.iter();
            let mut haber_iter = self.haber.iter();

            loop {
                let l_element = debe_iter.next();
                let r_element = haber_iter.next();

                if  l_element == None && r_element == None {
                    break;
                }

                match l_element {
                    Some(v) => {
                        write!(f, "\t€ {:<10}", v)?;
                    
                    },
                    None => {write!(f, "\t{:^12}", "~")?},
                };

                write!(f, "|")?;

                match r_element {
                    Some(v) => {write!(f, "{:>10} € ", v)?;},
                    None => {write!(f, "{:^12}", "~")?},
                };

                write!(f, "\n")?;

            }
        } else {
            write!(f, "({}) {}", self.codigo, self.nombre)?;
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

