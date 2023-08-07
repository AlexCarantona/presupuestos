use std::fmt::Display;

/// Activos
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Activos {
    ActivoCorriente,
    ActivoNoCorriente
}

/// Pasivos
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Pasivos {
    PasivoCorriente,
    PasivoNoCorriente
}

/// Patrimonios
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Patrimonios {
    Capital,
    Gastos,
    Ingresos
}


/// Representa una masa patrimonial
#[derive(PartialEq, Debug, Hash, Eq)]
pub enum Masa {
    Activo(Activos),
    Pasivo(Pasivos),
    Patrimonio(Patrimonios)
}

#[derive(PartialEq, Debug)]
pub struct CuentaError;

/// Representa una cuenta
#[derive(PartialEq, Debug)]
pub struct Cuenta {
    /// El saldo, privado, se representa en euros con tipo f64.
    saldo: f64,
    /// El nombre de la cuenta, que debe ser único.
    nombre: String,
    /// La masa patrimonial a la que pertenece la cuenta.
    masa: Masa,

}

impl Display for Cuenta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:_<25}{:_>25} €", self.nombre, self.saldo)
    }
}

impl Cuenta {
    /// Crea una nueva cuenta con saldo cero.
    pub fn new(nombre: &str, masa: Masa) -> Cuenta {
        Cuenta {
            saldo: 0.00,
            nombre: String::from(nombre),
            masa,
        }
    }

    /// Incrementa el saldo en tanta cantidad como se le indique, y devuelve el importe del saldo actualizado.
    pub fn incrementar_saldo(&mut self, importe: &f64) -> f64 {
        self.saldo += importe;
        self.saldo
    }

    /// Reduce el saldo en tanta cantidad como se le indique, y devuelve el importe del saldo actualizado.
    pub fn reducir_saldo(&mut self, importe: &f64) -> f64 {
        self.saldo -= importe;
        self.saldo
    }

    /// Devuelve el saldo actual de la cuenta.
    pub fn saldo(&self) -> f64 {
        self.saldo
    }

    /// Devuelve el nombre de la cuenta
    pub fn nombre(&self) -> String {
        String::clone(&self.nombre)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup_cuenta() -> Cuenta {
        Cuenta { saldo: 0.00, nombre: String::from("Cuenta test"), masa: Masa::Activo(Activos::ActivoCorriente)}
    }
    
    #[test]
    fn new_crea_cuenta() {
        let cuenta_test = setup_cuenta();
    
        let cuenta_creada = Cuenta::new("Cuenta test", Masa::Activo(Activos::ActivoCorriente));
    
        assert_eq!(cuenta_test, cuenta_creada);
    }
    
    #[test]
    fn incrementar_saldo_modifica_saldo() {
    
        let mut cuenta = setup_cuenta();

        let test_incremento = cuenta.incrementar_saldo(&17.00);
    
        assert_eq!(cuenta.saldo, 17.00);
        assert_eq!(test_incremento, 17.00);
    }
    
    #[test]
    fn reducir_saldo_modifica_saldo() {
        let mut cuenta = setup_cuenta();
    
        let test_reduccion = cuenta.reducir_saldo(&17.00);
    
        assert_eq!(cuenta.saldo, -17.00);
        assert_eq!(test_reduccion, -17.00);
    }

    #[test]
    fn saldo_devuelve_saldo() {
        let test_cuenta = setup_cuenta();

        assert_eq!(test_cuenta.saldo(), 0.00);
    }
}


