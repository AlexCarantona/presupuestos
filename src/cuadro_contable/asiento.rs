use std::fmt::Display;
use itertools::{Itertools, EitherOrBoth};

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
}

impl Display for Asiento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let cod_fmt = format!("N.º {}", self.codigo);
        let vec_concepto = self.concepto.split('\n');
        let w = 80;

        write!(f, "+{:-^width$}+\n","", width=w - 2)?;
        write!(f, "|{:^width$}|\n", cod_fmt, width=w - 2)?;
        for line in vec_concepto {
            write!(f, "|{:^width$}|\n", line, width=w - 2)?;
        }
        write!(f, "|{:^width$}|\n",&self.fecha.format("%Y-%m-%d"), width=w - 2)?;
        write!(f, "+{:-^width$}+\n","", width=w - 2)?;

        for par in self.debe.iter().zip_longest(&self.haber) {
            write!(f, "|")?;
            match par {
                EitherOrBoth::Both(izq, der) => {
                    let izq_fmt = format!("{:10} € {}", izq.importe(), izq.cuenta());
                    let der_fmt = format!("{} {:10} €", der.cuenta(), der.importe());
                    write!(f, "{:<width$}||{:>width$}", izq_fmt, der_fmt, width = w /2 -2)?
                },
                EitherOrBoth::Left(izq) => {
                    let izq_fmt = format!("{:10} € {}", izq.importe(), izq.cuenta());
                    write!(f, "{:<width$}||{:^width$}", izq_fmt, "~", width = w /2 -2)?
                },
                EitherOrBoth::Right(der) => {
                    let der_fmt = format!("{} {:10} €", der.cuenta(), der.importe());
                    write!(f, "{:^width$}||{:>width$}", "~", der_fmt, width = w /2 -2)?
                }
            }
            write!(f, "|\n")?;
        }

        write!(f, "+{:-^width$}+\n","", width=w - 2)?;

        Ok(())

    }
}

impl<'a> Asiento {

    /// Crea un nuevo asiento a partir de un concepto
    pub fn new(concepto: String) -> Asiento {
        Asiento {
            concepto,
            fecha: offset::Local::now().date_naive(),
            debe: vec![],
            haber: vec![],
            codigo: String::new(),
        }
    }


    /// Devuelve la fecha y, si recibe argumentos, la modifica
    pub fn fecha(&mut self, fecha: Option<NaiveDate>) -> NaiveDate {

        if let Some(f) =  fecha {
            self.fecha = f;
        };

        self.fecha
    }

    /// Devuelve el concepto del asiento
    pub fn concepto(&self) -> &str {
        self.concepto.as_str()
    }

    /// Inserta un movimiento en el debe
    pub fn insertar_debe(&mut self, movimiento: Movimiento) {
        self.debe.push(movimiento);
    }

    /// Inserta un movimiento en el haber
    pub fn insertar_haber(&mut self, movimiento: Movimiento) {
        self.haber.push(movimiento);
    }

    /// Devuelve una referencia a los movimientos del debe
    pub fn debe(&self) -> &Vec<Movimiento> {
        &self.debe
    } 
    
    /// Devuelve una referencia a los movimientos del haber
    pub fn haber(&self) -> &Vec<Movimiento> {
        &self.haber
    }

    /// Modifica y devuelve el código
    pub fn codigo(&mut self, codigo: Option<String>) -> &str {

        if let Some(c) = codigo {
            self.codigo = c;
        }

        self.codigo.as_str()
    }

    // Devuelve el código
    pub fn get_codigo(&self) -> &str {
        self.codigo.as_str()
    }

    /// Valida el asiento: las anotaciones del debe han de sumar lo mismo que las del haber
    pub fn validar(&self) -> bool {
        let debe_reducido: Option<f64> = self.debe
            .iter()
            .map(|element| (element.importe() * 100.00).round())
            .reduce(|a, b| a + b);
        let haber_reducido: Option<f64> = self.haber
            .iter()
            .map(|element| (element.importe() * 100.00).round())
            .reduce(|a, b| a + b);


        debe_reducido == haber_reducido
    }



}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use  super::*;

    #[test]
    fn crear_asiento_crea_asiento() {
        let asiento = Asiento { 
            concepto: String::from("asiento de muestra"), 
            debe: vec![], 
            haber: vec![], 
            fecha: offset::Local::now().date_naive(),
            codigo: String::new(),
        };

        let asiento_test = Asiento::new(String::from("asiento de muestra"));

        assert_eq!(asiento, asiento_test);
    }

    #[test]
    fn fecha_modifica_y_devuelve_la_fecha() {
        let mut asiento = Asiento::new(String::from("concepto"));

        asiento.fecha(NaiveDate::from_ymd_opt(2023, 1, 1));

        assert_eq!(asiento.fecha.year(), 2023);
        assert_eq!(asiento.fecha.month(), 1);
        assert_eq!(asiento.fecha.day(), 1);

        assert_eq!(asiento.fecha(None), asiento.fecha);    


    }
}