use std::fmt::Display;

use chrono::NaiveDate;

use self::movimiento::Movimiento;

mod cuenta;
pub mod movimiento;
mod asiento;

/// Este struct almacena las cuentas que se usarán.
/// Su misión principal es realizar y centralizar las operaciones
#[derive(Debug, PartialEq)]
pub struct Cuadro<'a> {
    /// Almacena las cuentas
    cuentas: Vec<cuenta::Cuenta>,
    /// Almacena los asientos
    asientos: Vec<asiento::Asiento<'a>>

}


impl Cuadro<'_> {

    /// Crea un nuevo cuadro de cuentas
    pub fn create() -> Cuadro<'static> {
        Cuadro { cuentas: vec![], asientos: vec![] }
    }

    /// Envuelve la creación de una cuenta::Cuenta
    fn crear_cuenta(&mut self, nombre_cuenta: &str, masa_cuenta: cuenta::Masa) {
        let cuenta = cuenta::Cuenta::new(nombre_cuenta, masa_cuenta);
        self.cuentas.push(cuenta);
    }

    /// Crea una cuenta de gastos
    pub fn crear_cuenta_gasto(&mut self, nombre_cuenta: &str) {
        self.crear_cuenta(nombre_cuenta, cuenta::Masa::Patrimonio(cuenta::Patrimonios::Gastos));
    }

    /// Crea una cuenta de activo corriente
    pub fn crear_cuenta_activo_corriente(&mut self, nombre_cuenta: &str) {
        self.crear_cuenta(nombre_cuenta, cuenta::Masa::Activo(cuenta::Activos::ActivoCorriente));
    }


    /// Crea un asiento con, al menos, un movimiento
    pub fn crear_asiento(&mut self, concepto: &str, fecha: Option<NaiveDate>, debe: Vec<Movimiento>, haber: Vec<Movimiento>) {
        
        // Crea el asiento y deja la referencia modificable.
        let mut asiento = asiento::Asiento {
                concepto: String::from(concepto),
                debe: vec![],
                haber: vec![],
            };
        asiento.fecha(fecha);

        // Inserta los movimientos del debe
        for m in debe.into_iter() {
            asiento.insertar_debe(m)
        }

        // Inserta los movimientos del haber
        for  m in haber.into_iter() {
            asiento.insertar_haber(m)
        }

        // Guarda el asiento
        self.asientos.push(asiento);
        
    }

    /// Valida que una cuenta existe en el cuadro si se le pasa su nombre
    pub fn validar_cuenta(&self, nombre_cuenta: &str) -> bool {
        self.cuentas.iter().any(|c| c.nombre() == String::from(nombre_cuenta))
    }

    /// Encuentra una cuenta y devuelve su referencia mutable si la encuentra
    fn cuenta(&mut self, nombre_cuenta: &str) -> Option<&mut cuenta::Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(nombre_cuenta) == self.cuentas[id].nombre() {
                return Some(&mut self.cuentas[id])
            }
        };
        None
    }

    /// Encuentra una cuenta y devuelve su referencia inmutable si la encuentra
    pub fn cuenta_pub(&self, nombre_cuenta: &str) -> Option<&cuenta::Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(nombre_cuenta) == self.cuentas[id].nombre() {
                return Some(&self.cuentas[id])
            }
        };

        None
    }

    pub fn asientos(&self) -> &Vec<asiento::Asiento> {

        &self.asientos
    }

   
}

impl Display for Cuadro<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cuenta in &self.cuentas {
            println!("{}", cuenta)
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setupcuadro() -> Cuadro<'static> {
        let mut cuadro = Cuadro::create();

        cuadro.crear_cuenta("Capital", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Capital));
        cuadro.crear_cuenta("Bancos", cuenta::Masa::Activo(cuenta::Activos::ActivoCorriente));
        cuadro.crear_cuenta("Alimerka", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Gastos));
        cuadro

    }

    #[test]
    fn crear_cuenta_crea_e_inserta_una_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.crear_cuenta("cuenta::Cuenta 1", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Capital));

        assert_eq!(cuadro.cuentas.len(), 1);
        assert_eq!(cuadro.cuentas[0].saldo(), 0.00);
        assert_eq!(cuadro.cuentas[0].nombre(), "cuenta::Cuenta 1");
    }

    #[test]
    fn validar_cuenta_devuelve_true_nombre_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.crear_cuenta("cuenta::Cuenta 1", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Capital));

        assert!(cuadro.validar_cuenta("cuenta::Cuenta 1"));
        assert!(!cuadro.validar_cuenta("cuenta::Cuenta 2"));
    }

    #[test]
    fn cuenta_devuelve_referencia_mutable_a_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.crear_cuenta("cuenta::Cuenta test", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Capital));

        assert_eq!(cuadro.cuenta("cuenta::Cuenta test"), Some(&mut cuenta::Cuenta::new("cuenta::Cuenta test", cuenta::Masa::Patrimonio(cuenta::Patrimonios::Capital))));
        assert_eq!(cuadro.cuenta("Ninguna"), None);

    }


}