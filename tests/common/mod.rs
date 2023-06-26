use presupuestos::elementos::{Cuenta, Masa, Activos, Patrimonios};
use std::rc::Rc;


pub fn setup_cuentas() -> [Rc<Cuenta>; 2] {
    let mut cuenta1 = Cuenta::new("Cuenta 1", Masa::Activo(Activos::ActivoCorriente));
    let mut cuenta2 =  Cuenta::new("Cuenta 2", Masa::Patrimonio(Patrimonios::Capital));

    cuenta1.incrementar_saldo(&20.00);
    cuenta2.incrementar_saldo(&20.00);

    [Rc::new(cuenta1), Rc::new(cuenta2)]
}
