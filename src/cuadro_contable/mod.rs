use std::fmt::Display;

use chrono::NaiveDate;

use crate::cuadro_contable::asiento::Asiento;

use self::movimiento::Movimiento;

mod cuenta;
pub mod movimiento;
mod asiento;

/// Este struct almacena las cuentas que se usarán.
/// Su misión principal es realizar y centralizar las operaciones
#[derive(Debug, PartialEq)]
pub struct Cuadro {
    /// Almacena las cuentas
    cuentas: Vec<cuenta::Cuenta>,
    /// Almacena los asientos
    libro_diario: Vec<asiento::Asiento>

}


impl<'a> Cuadro {

    /// Crea un nuevo cuadro de cuentas
    pub fn create() -> Cuadro {
        Cuadro { cuentas: vec![], libro_diario: vec![] }
    }

    /// Envuelve la creación de una cuenta
    pub fn crear_cuenta(&mut self, nombre_cuenta: &str, codigo_cuenta: &str) {
        let cuenta = cuenta::Cuenta::new(nombre_cuenta, codigo_cuenta);
        self.cuentas.push(cuenta);
    }

    /// Crea un asiento con, al menos, un movimiento
    pub fn crear_asiento(&mut self, concepto: String, fecha: Option<NaiveDate>, debe: Vec<Movimiento>, haber: Vec<Movimiento>, codigo: String) {
        
        // Crea el asiento y deja la referencia modificable.
        let mut asiento = asiento::Asiento::new(concepto);
        asiento.fecha(fecha);
        asiento.codigo(Some(codigo));

        // Hidrata e inserta los movimientos del debe
        for mut m in debe.into_iter() {
            m.hidratar_cuenta(&self);
            asiento.insertar_debe(m)
        }

        // Inserta los movimientos del haber
        for mut m in haber.into_iter() {
            m.hidratar_cuenta(&self);
            asiento.insertar_haber(m)
        }

        // Guarda el asiento
        self.libro_diario.push(asiento);
        
    }

    /// Encuentra una cuenta y devuelve su referencia inmutable si la encuentra
    pub fn find_cuenta(&self, codigo_cuenta: &str) -> Option<&cuenta::Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(codigo_cuenta) == self.cuentas[id].codigo() {
                return Some(&self.cuentas[id])
            }
        };

        None
    }

    /// Devuelve una lista de los asientos que existen
    pub fn libro_diario(&self) -> &Vec<asiento::Asiento> {

        &self.libro_diario
    }

    /// Imprime el listado de asientos que existen
    pub fn print_libro_diario(&self) {

        println!("{:#^40} LIBRO DIARIO {:#^40}", "#", "#");

        let mut asientos_erroneos: Vec<&Asiento> = vec![];
        let mut total_asientos: usize = 0;

        for asiento in &self.libro_diario {
            println!("{}", asiento);
            total_asientos += 1;
            if !asiento.validar() {
                asientos_erroneos.push(asiento)
            }
        }

        println!("Total asientos: {}", total_asientos);
        
        if asientos_erroneos.len() > 0 {
            println!("Hay asientos en los que no coinciden las anotaciones del debe y del haber:");
            for asiento_erroneo in asientos_erroneos {
                println!("{}", asiento_erroneo.get_codigo());
            }
        } else { println!("No hay errores")}

        println!("\n");
        
    }

    /// Mayoriza e imprime las cuentas
    pub fn print_libro_mayor(&mut self) {

        println!("{:#^40} LIBRO MAYOR {:#^40}", "#", "#");

        for cuenta in self.cuentas.iter_mut() {
            cuenta.mayorizar_cuenta(&self.libro_diario);
            println!("{}", cuenta);
        }

    }

}

impl Display for Cuadro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cuenta in &self.cuentas {
            write!(f, "{}\n", cuenta)?;
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn crear_cuenta_crea_e_inserta_una_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.crear_cuenta("Activo corriente", "100");

        assert_eq!(cuadro.cuentas.len(), 1);
        assert_eq!(cuadro.cuentas[0].nombre(), "Activo corriente");
    }

}