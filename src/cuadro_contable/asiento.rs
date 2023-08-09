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
}

impl Display for Asiento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "+{:-^78}+\n", "-")?;

        write!(f, "|{:^78}|\n", self.concepto)?;

        write!(f, "|{:^78}|\n", &self.fecha.format("%Y-%m-%d"))?;

        write!(f, "+{:-^78}+\n", "-")?;

        write!(f, "{:>43}\n", "|")?;


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
                    write!(f, "€ {:<8} {:<31}", v.importe(), v.cuenta())?
                
                },
                None => {write!(f, "{:^42}", "~")?},
            };

            write!(f, "|")?;

            match r_element {
                Some(v) => {write!(f, "{:>31}{:>8} € ", v.cuenta(), v.importe())?;},
                None => {write!(f, "{:^42}", "~")?},
            };

            write!(f, "\n")?;

        }

        write!(f, "{:>43}\n", "|")?;


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