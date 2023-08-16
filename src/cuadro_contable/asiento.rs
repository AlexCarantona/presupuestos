use std::fmt::Display;

use chrono::{NaiveDate, offset};

use super::movimiento::Movimiento;

/// Representa un asiento contable.
#[derive(PartialEq, Debug)]
pub struct Asiento {
    debe: Vec<Movimiento>,
    haber: Vec<Movimiento>,
    concepto: String,
    fecha: NaiveDate,
    codigo: String, 
    comprobacion: f64,
}

impl Display for Asiento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let cod_fmt = format!("N.º {}", self.codigo);
        let vec_concepto = self.concepto.split('\n');
        let w = 120;

        write!(f, "+{:-^width$}+\n","", width=w - 2)?;
        write!(f, "|{:^width$}|\n", cod_fmt, width=w - 2)?;
        for line in vec_concepto {
            write!(f, "|{:^width$}|\n", line, width=w - 2)?;
        }
        write!(f, "|{:^width$}|\n",&self.fecha.format("%Y-%m-%d"), width=w - 2)?;
        write!(f, "+{:-^width$}+\n","", width=w - 2)?;


        write!(f, "+{:-^width$}+\n","", width=w - 2)?;

        Ok(())

    }
}

impl Asiento {

    /// Crea un nuevo asiento a partir de un concepto
    pub fn new(concepto: &str, fecha: Option<NaiveDate>, debe: Vec<Movimiento>, haber: Vec<Movimiento>) -> Asiento {
        let saldo_debe = debe
            .iter()
            .map(|x| x.importe())
            .reduce(|a, b| a + b)
            .unwrap();
    
        let saldo_haber = haber
            .iter()
            .map(|x| x.importe())
            .reduce(|a, b| a + b)
            .unwrap();
        
        Asiento {
            concepto: concepto.to_string(),
            fecha: match fecha {
                Some(v) => v,
                None => offset::Local::now().date_naive(),
            },
            debe,
            haber,
            codigo: String::new(),
            comprobacion: saldo_debe - saldo_haber,
        }
    }

    /// Valida el asiento: las anotaciones del debe han de sumar lo mismo que las del haber
    pub fn validar_saldos(&self) -> bool {
        self.comprobacion == 0.00
    }


}