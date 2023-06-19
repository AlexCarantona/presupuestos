/// Representa una cuenta
#[derive(PartialEq, Debug)]
pub struct Cuenta {
    /// El saldo, privado, se representa en euros con tipo f64.
    saldo: f64,
    /// El nombre de la cuenta, que no tiene por qué ser único.
    nombre: String,
}

impl Cuenta {
    /// Crea una nueva cuenta con saldo cero.
    pub fn new(nombre: &str) -> Cuenta {
        Cuenta {
            saldo: 0.00,
            nombre: String::from(nombre),
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

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_crea_cuenta() {
        let cuenta_test = Cuenta { saldo: 0.00, nombre: String::from("Cuenta 1")};

        let cuenta_creada = Cuenta::new("Cuenta 1");

        assert_eq!(cuenta_test, cuenta_creada);
    }

    #[test]
    fn incrementar_saldo_modifica_saldo() {
        let mut cuenta = Cuenta { saldo: 12.00, nombre: String::from("test_incremento")};

        let test_incremento = cuenta.incrementar_saldo(&5.00);

        assert_eq!(cuenta.saldo, 17.00);
        assert_eq!(test_incremento, 17.00);
    }

    #[test]
    fn reducir_saldo_modifica_saldo() {
        let mut cuenta = Cuenta { saldo: 12.00, nombre: String::from("test_reducción")};

        let test_reduccion = cuenta.reducir_saldo(&5.00);

        assert_eq!(cuenta.saldo, 7.00);
        assert_eq!(test_reduccion, 7.00);
    }
}