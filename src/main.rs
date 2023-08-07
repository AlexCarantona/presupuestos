
use presupuestos::cuadro_contable::{Cuadro, movimiento::Movimiento};

fn main() {
    let mut cuadro = Cuadro::create();

    let mut args = std::env::args();

    testear(&mut cuadro);

    // Ruta del programa.
    args.next();

    // Comando.
    match args.next() {
        Some(v) => {
            match v.as_str() {
                "--ayuda" => ayuda(),
                "--prueba" => testear(&mut cuadro),
                _ => ayuda()
            } 
        },
        _ => ayuda(),
    }

    while let Some(v) = args.next() {
        println!("Me has llamado con el argumento: {}", v);
    }



}


fn ayuda() {
    println!(
"\
¡Bienvenido a mi gestor contable!\n
Puedes realizar alguna de estas acciones:\n
        --help: Muestra esta ayuda\n
        --plantillas: Crea en el directorio de ejecución plantillas para hacer un cuadro contable\n
"
    );
}

fn testear(cuadro: &mut Cuadro) {

    let cuenta1 = cuadro.crear_cuenta_activo_corriente("Cuenta de activo corriente");
    let cuenta2 = cuadro.crear_cuenta_gasto("Cuenta de gasto");

    let debe = Movimiento::new(20.00, String::from("Cuenta de activo corriente"));
    let haber = Movimiento::new(20.00, String::from("Cuenta de gasto"));

    cuadro.crear_asiento("Un asiento de prueba", None, vec![debe], vec![haber]);

    for asi in cuadro.asientos() {
        print!("{asi}");
    }

}