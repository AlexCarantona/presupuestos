use std::fmt::Display;

use chrono::{NaiveDate, offset};

use super::movimiento::Movimiento;

/// Representa un asiento contable.
#[derive(PartialEq, Debug)]
pub struct Asiento<'a> {
    debe: Vec<Movimiento<'a>>,
    haber: Vec<Movimiento<'a>>,
    concepto: String,
    fecha: NaiveDate, 
}

impl Display for Asiento<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "+{:-^78}+\n", "-")?;

        write!(f, "|{:^78}|\n", self.concepto)?;

        write!(f, "|{:^78}|\n", &self.fecha.format("%Y-%m-%d"))?;

        write!(f, "+{:-^78}+\n", "-")?;

        write!(f, "{:>41}\n", "|")?;


        let mut debe_iter = self.debe.iter();
        let mut haber_iter = self.haber.iter();

        loop {
            let l_element = debe_iter.next();
            let r_element = haber_iter.next();

            if  l_element == None && r_element == None {
                break;
            }

            match l_element {
                Some(v) => {write!(f, "€ {:<6} {:<31}", v.importe(), v.cuenta())?},
                None => {write!(f, "{:^40}", "~")?},
            };

            write!(f, "|")?;

            match r_element {
                Some(v) => {write!(f, "{:>31}{:>6} € ", v.cuenta(), v.importe())?;},
                None => {write!(f, "{:^40}", "~")?},
            };

            write!(f, "\n")?;

        }

        write!(f, "{:>41}\n", "|")?;


        Ok(())

    }
}

impl Asiento<'_> {

    /// Crea un nuevo asiento a partir de un concepto
    pub fn new(concepto: &str) -> Asiento {
        Asiento {
            concepto: String::from(concepto),
            fecha: offset::Local::now().date_naive(),
            debe: vec![],
            haber: vec![],
        }
    }


    /// Devuelve la fecha y, si recibe argumentos, la modifica
    pub fn fecha(&mut self, fecha: Option<NaiveDate>) -> NaiveDate {

        if let Some(f) =  fecha {
            self.fecha = f;
        };

        self.fecha
    }

    /// Inserta movimientos al debe
    pub fn insertar_debe(&mut self, movimiento: Movimiento) {
        self.debe.push(movimiento);
    }

    /// Inserta movimientos al haber
    pub fn insertar_haber(&mut self, movimiento: Movimiento) {
        self.haber.push(movimiento);
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
            fecha: offset::Local::now().date_naive()
        };

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