use presupuestos::elementos::{Cuenta, Movimiento, Asiento, Masa, Patrimonios, Activos};


fn main() {
    let cuenta1 = Cuenta::new("Supermercados", Masa::Patrimonio(Patrimonios::Gastos));
    let cuenta2 = Cuenta::new("Caja Rural", Masa::Activo(Activos::ActivoCorriente));
    let cuenta3 = Cuenta::new("Capital", Masa::Patrimonio(Patrimonios::Capital));

    let mut asiento = Asiento::new("Compra en el Alimerka");

    let movimiento = Movimiento::new(30.00, cuenta1.nombre(), cuenta2.nombre());
    asiento.insertar_movimiento(movimiento);
    let movimiento1 = Movimiento::new(50.00, cuenta2.nombre(), cuenta3.nombre());
    asiento.insertar_movimiento(movimiento1);

    println!("{}", asiento.imprimir());
}
