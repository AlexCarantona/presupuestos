use std::fs::{self};
use regex;

use chrono::NaiveDate;
use presupuestos::cuadro_contable::{Cuadro, movimiento::Movimiento};

fn main() {
    let mut cuadro = Cuadro::create();

    cargar_cuadro(&mut cuadro);
    cargar_diario(&mut cuadro);

    cuadro.print_libro_diario();
    cuadro.print_libro_mayor();

}

/// Lee un archivo llamado 'cuadro.txt' para recuperar las cuentas, imprime error si no lo logra
fn cargar_cuadro(cuadro: &mut Cuadro) {
    let archivo = fs::read_to_string("cuadro.txt");

    match archivo {
        Ok(contenido) => {procesar_cadena(contenido, cuadro)},
        Err(e) => println!("Ha habido un error al leer el archivo 'cuadro.txt'.: {e}")
    }   
}

/// Toma una serie leída y procesa cada línea escrita en formato <CÓDIGO> <NOMBRE> como una cuenta
fn procesar_cadena(cadena: String, cuadro: &mut Cuadro) {

    let cadena = cadena
        .as_str();

    let re_codigo: regex::Regex = regex::Regex::new(r"(?P<codigo>[0-9]+)\s(?P<nombre>.+)\n").unwrap();

    let capturas = re_codigo.captures_iter(cadena);

    for c in capturas {
        cuadro.crear_cuenta(&c["nombre"], &c["codigo"])
    }
}

/// Procesa una carpeta y procesa los posibles archivos de asientos, que deben tener formato <YYYYMMDD.data>
fn cargar_diario(cuadro: &mut Cuadro) {
    let carpeta = fs::read_dir("./diario/")
        .expect("Imposible listar el directorio diario/");

    for ruta in carpeta {
        if let Ok(archivo) = ruta {
            let validado = validar_archivo(&archivo);

            if let Some(_fecha) = validado {
                    leer_asientos(&archivo, cuadro);
                }
        }
    }
}

/// Valida que la ruta y archivo son correctos. Devuelve una fecha si lo ha leído bien.
fn validar_archivo(ruta: &fs::DirEntry) -> Option<NaiveDate> {

    let mut respuesta = None;

    match ruta.file_name().into_string() {
        Ok(c) => {
            let formato_archivo = regex::Regex::new(r"^(?P<fecha>[0-9]{8})(?:\d+)\.data$").unwrap();
            let capturas = formato_archivo.captures(c.as_str());
            if let Some(cap) = capturas {
                let fecha = NaiveDate::parse_from_str(
                    &cap["fecha"],
                    "%Y%m%d"
            );
            if let Ok(f) = fecha {
                respuesta = Some(f);
            }
            }
        },
        Err(_e) => {
            println!("Error al procesar tu ruta, sigue siendo una cadena de sistema")
        },
    }
    respuesta
}

fn leer_asientos(ruta: &fs::DirEntry, cuadro: &mut Cuadro) {
    let mut fecha: Option<NaiveDate> = None;
    let mut codigo: String = String::new();

    let leido = fs::read_to_string(ruta.path())
        .expect("Imposible leer el archivo");

    let fecha_expr = regex::Regex::new(r"^(?P<fecha>[0-9]{8})[0-9]*").unwrap();

    for cap in fecha_expr.captures_iter(&ruta.file_name().into_string().unwrap()) {
        fecha = Some(NaiveDate::parse_from_str(&cap["fecha"], "%Y%m%d").unwrap());
        codigo = cap[0].to_string();
    };

    let concepto_expr = regex::Regex::new(r"^(?s)(?P<concepto>.+)\n\nDEBE\n(?P<debe>.+)\n\nHABER\n(?P<haber>.+)\n\n///").unwrap();

    let captura = concepto_expr.captures(&leido);

    if let Some(cap) = captura {

        // Concepto del asiento
        let concepto = cap["concepto"].to_string();
        
        // Movimientos del debe
        let debe: Vec<Movimiento> = cap["debe"]
            .split('\n')
            .map(|v| {
                let movimiento:Vec<&str> = v.split(' ').collect();
                let codigo_cuenta = movimiento[0].to_string();
                let mut importe: f64 = 0.00;

                if let Ok(v) = movimiento[1].trim().parse() {
                    importe = v;
                }

                Movimiento::new(importe, codigo_cuenta)
            })
            .collect();

        // Movimientos del haber
        let haber: Vec<Movimiento> = cap["haber"]
        .split('\n')
        .map(|v| {
            let movimiento:Vec<&str> = v.split(' ').collect();

            let codigo_cuenta = movimiento[0].to_string();
            let mut importe: f64 = 0.00;

            if let Ok(v) = movimiento[1].trim().parse() {
                importe = v;
            }

            Movimiento::new(importe, codigo_cuenta)
        })
        .collect();  

        cuadro.crear_asiento(concepto, fecha, debe, haber, codigo);  
    }
}