
use presupuestos::cuadro_contable::Cuadro;

fn main() {
    let mut cuadro = Cuadro::create();

    let mut args = std::env::args();

    // Ruta del programa.
    args.next();

    // Comando.
    match args.next() {
        Some(v) => {
            match v.as_str() {
                "--ayuda" => ayuda(),
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