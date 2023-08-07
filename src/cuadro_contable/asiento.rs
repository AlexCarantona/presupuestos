use std::fmt::Display;

use super::movimiento;
use chrono::{NaiveDate, offset};

/// Representa un asiento contable, con uno o varios movimientos.
#[derive(PartialEq, Debug)]
pub struct Asiento {
    movimientos: Vec<movimiento::Movimiento>,
    concepto: String,
    fecha: NaiveDate, 
}

impl Display for Asiento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:-^70}\n", self.concepto)?;

        write!(f, "{:-^70}\n", &self.fecha.format("%Y-%m-%d"))?;

        for movimiento in self.movimientos.iter() {
            write!(f, "{}", movimiento)?;
        }

        Ok(())

    }
}

impl Asiento {

    /// Crea un nuevo asiento a partir de un concepto
    pub fn new(concepto: &str) -> Asiento {
        Asiento {
            concepto: String::from(concepto),
            movimientos: vec![],
            fecha: offset::Local::now().date_naive(),
        }
    }

    /// Inserta un movimiento en el asiento
    pub fn insertar_movimiento(&mut self, movimiento: movimiento::Movimiento) {
        self.movimientos.push(movimiento);
    }

    /// Devuelve el número de movimientos
    pub fn n_movimientos(&self) -> usize {
        self.movimientos.len()
    }

    /// Devuelve la fecha y, si recibe argumentos, la modifica
    pub fn fecha(&mut self, fecha: Option<NaiveDate>) -> NaiveDate {

        if let Some(f) =  fecha {
            self.fecha = f;
        };

        self.fecha
    }


}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use  super::*;

    #[test]
    fn crear_asiento_crea_asiento() {
        let asiento = Asiento { concepto: String::from("asiento de muestra"), movimientos: vec![], fecha: offset::Local::now().date_naive()};

        let asiento_test = Asiento::new("asiento de muestra");

        assert_eq!(asiento, asiento_test);
    }

    #[test]
    fn fecha_modifica_y_devuelve_la_fecha() {
        let mut asiento = Asiento::new("concepto");

        asiento.fecha(NaiveDate::from_ymd_opt(2023, 1, 1));

        assert_eq!(asiento.fecha.year(), 2023);
        assert_eq!(asiento.fecha.month(), 1);
        assert_eq!(asiento.fecha.day(), 1);

        assert_eq!(asiento.fecha(None), asiento.fecha);    


    }
}