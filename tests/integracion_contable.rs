mod common;

use presupuestos::elementos::{Movimiento, Asiento};


#[test]
fn insertar_movimiento_actualiza_asiento() {

    let [cuenta1, cuenta2] = common::setup_cuentas();
    let movimiento = Movimiento::new(20.00, cuenta1.nombre(), cuenta2.nombre());

   let mut asiento = Asiento::new("asiento de prueba");

   assert_eq!(asiento.n_movimientos(), 0);

   asiento.insertar_movimiento(movimiento);

   assert_eq!(asiento.n_movimientos(), 1);
}


