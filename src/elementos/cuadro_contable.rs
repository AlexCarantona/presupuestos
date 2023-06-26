use chrono::NaiveDate;

use crate::elementos::{Masa, Cuenta, Movimiento, Asiento, Patrimonios, Activos};


/// Este struct almacena las cuentas que se usarán.
/// Su misión principal es validar que existan cuando se inserta un movimiento,
/// además de modificar los saldos cuando procede.
#[derive(Debug, PartialEq)]
pub struct Cuadro {
    /// Almacena las cuentas
    cuentas: Vec<Cuenta>,
    asientos: Vec<Asiento>

}

impl Cuadro {

    /// Crea un nuevo cuadro de cuentas
    pub fn create() -> Cuadro {
        Cuadro { cuentas: vec![], asientos: vec![] }
    }

    /// Envuelve la creación de una Cuenta
    pub fn insertar_cuenta(&mut self, nombre_cuenta: &str, masa_cuenta: Masa) {
        let cuenta = Cuenta::new(nombre_cuenta, masa_cuenta);
        self.cuentas.push(cuenta);
    }

    /// Envuelve la creación de un Movimiento, rompe el programa si falla
    fn crear_movimiento(&mut self, nombre_cuenta_deudora: &str, nombre_cuenta_acreedora: &str, importe: f64) -> Movimiento {
        
        let cuenta_deudora: String;
        let cuenta_acreedora: String;

        match self.cuenta(nombre_cuenta_deudora) {
            Some(cuenta) => {
                cuenta.incrementar_saldo(&importe);
                cuenta_deudora = cuenta.nombre();
            },
            None => {panic!("La cuenta deudora no existe")}
        };

        match self.cuenta(nombre_cuenta_acreedora) {
            Some(cuenta) => {
                cuenta.reducir_saldo(&importe);
                cuenta_acreedora = cuenta.nombre();
            },
            None => {panic!("La cuenta acreedora no existe")}
        };

        Movimiento::new(importe, cuenta_deudora, cuenta_acreedora)

    }

    /// Crea un asiento con, al menos, un movimiento
    pub fn crear_asiento(&mut self, concepto: &str, fecha: Option<NaiveDate>, movimientos: Vec<(&str, &str, f64)>) {
        
        // Crea el asiento y deja la referencia modificable.
        let mut asiento = Asiento::new(concepto);
        asiento.fecha(fecha);

        // Crea los movimientos correspondientes y los guarda en el asiento
        for movimiento in movimientos.iter() {
            let m = self.crear_movimiento(movimiento.0, movimiento.1, movimiento.2);
            asiento.insertar_movimiento(m);
        }

        // Guarda el asiento
        self.asientos.push(asiento);
        
    }

    /// Valida que una cuenta existe en el cuadro si se le pasa su nombre
    pub fn validar_cuenta(&self, nombre_cuenta: &str) -> bool {
        self.cuentas.iter().any(|c| c.nombre() == String::from(nombre_cuenta))
    }

    /// Encuentra una cuenta y devuelve su referencia mutable si la encuentra
    pub fn cuenta(&mut self, nombre_cuenta: &str) -> Option<&mut Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(nombre_cuenta) == self.cuentas[id].nombre() {
                return Some(&mut self.cuentas[id])
            }
        };
        None
    }

   
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setupcuadro() -> Cuadro {
        let mut cuadro = Cuadro::create();

        cuadro.insertar_cuenta("Capital", Masa::Patrimonio(Patrimonios::Capital));
        cuadro.insertar_cuenta("Bancos", Masa::Activo(Activos::ActivoCorriente));
        cuadro.insertar_cuenta("Alimerka", Masa::Patrimonio(Patrimonios::Gastos));


        cuadro

    }

    #[test]
    fn insertar_cuenta_crea_e_inserta_una_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.insertar_cuenta("Cuenta 1", Masa::Patrimonio(Patrimonios::Capital));

        assert_eq!(cuadro.cuentas.len(), 1);
        assert_eq!(cuadro.cuentas[0].saldo(), 0.00);
        assert_eq!(cuadro.cuentas[0].nombre(), "Cuenta 1");
    }

    #[test]
    fn validar_cuenta_devuelve_true_nombre_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.insertar_cuenta("Cuenta 1", Masa::Patrimonio(Patrimonios::Capital));

        assert!(cuadro.validar_cuenta("Cuenta 1"));
        assert!(!cuadro.validar_cuenta("Cuenta 2"));
    }

    #[test]
    fn cuenta_devuelve_referencia_mutable_a_cuenta() {
        let mut cuadro = Cuadro::create();

        cuadro.insertar_cuenta("Cuenta test", Masa::Patrimonio(Patrimonios::Capital));

        assert_eq!(cuadro.cuenta("Cuenta test"), Some(&mut Cuenta::new("Cuenta test", Masa::Patrimonio(Patrimonios::Capital))));
        assert_eq!(cuadro.cuenta("Ninguna"), None);

    }

    #[test]
    fn insertar_movimiento_contable_crea_movimiento_y_actualiza_cuentas() {
        let mut cuadro = Cuadro::create();

        cuadro.insertar_cuenta("Caja Rural", Masa::Activo(Activos::ActivoCorriente));
        cuadro.insertar_cuenta("Alimerka", Masa::Patrimonio(Patrimonios::Gastos));

        cuadro.crear_movimiento("Alimerka", "Caja Rural", 20.00);


        if let Some(cuenta1) = cuadro.cuenta("Caja Rural") {
            assert_eq!(cuenta1.saldo(), -20.00);
        };
        if let Some(cuenta2) = cuadro.cuenta("Alimerka") {
            assert_eq!(cuenta2.saldo(), 20.00);
        };


    }

    #[test]
    fn crear_asiento_con_un_movimiento_crea_movimiento_y_modifica_saldos() {

        let mut cuadro = setupcuadro();

        let movimiento = vec![
            ("Bancos", "Capital", 300.00)
        ];

        cuadro.crear_asiento("Un asiento", None, movimiento);


        match cuadro.cuenta("Bancos") {
            Some(v) => {assert_eq!(v.saldo(), 300.00)},
            None => panic!("El saldo no se ha modificado")

        }
        
    }

    #[test]
    fn crear_asiento_con_varios_movimientos() {
        let mut cuadro = setupcuadro();

        let movimientos = vec![
            ("Bancos", "Capital", 300.00),
            ("Alimerka", "Bancos", 20.00),
        ];

        cuadro.crear_asiento("Un asiento", None, movimientos);

        match cuadro.cuenta("Capital") {
            Some(v) => assert_eq!(v.saldo(), -300.00),
            None => panic!("No se ha transferido el capital")
        }

        match cuadro.cuenta("Alimerka") {
            Some(v) => assert_eq!(v.saldo(), 20.00),
            None => panic!("No se ha cargo la cuenta de Alimerka")
        }

        match cuadro.cuenta("Bancos") {
            Some(v) => assert_eq!(v.saldo(), 280.00),
            None => panic!("No se han reflejado los movimientos del banco")
        }
    }
}