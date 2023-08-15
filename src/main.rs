use std::fs::{self};
use std::str::Split;
use regex;

use chrono::{NaiveDate, Utc};
use presupuestos::cuadro_contable::{Cuadro, movimiento::Movimiento};

fn main() {

    let mut args = std::env::args();

    let mut path_diario = "diario".to_string();

    if let Some(v) = args.nth(1) {
        path_diario = v;
    }

    
    let mut cuadro = Cuadro::new();

    leer_balance_inicial(&mut cuadro);

    cargar_cuadro(&mut cuadro);
    cargar_diario(&mut cuadro, path_diario);

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
fn cargar_diario(cuadro: &mut Cuadro, path: String) {

    let carpeta = fs::read_dir(path)
        .expect("Imposible listar el directorio diario");

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

/// Lee todos los asientos de una ruta dada, y los guarda en el cuadro.
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

#[derive(Debug, PartialEq)]
enum Masa {
    Activo,
    Pasivo,
    Patrimonio,
    Gasto,
    Ingreso
}

fn leer_balance_inicial(cuadro: &mut Cuadro) {

    let archivo = fs::read_to_string("balance_inicial.txt").unwrap();

    let mut vec_debe: Vec<Movimiento> = vec![];
    let mut vec_haber: Vec<Movimiento> = vec![];

    let iterador_archivo: Split<&str> = archivo.as_str().split("\n");

    let mut grupo: Masa = Masa::Activo;

    for linea in iterador_archivo {
        let modo: Option<Masa> = match linea {
            "ACTIVO" => Some(Masa::Activo),
            "ACTIVO CORRIENTE" => Some(Masa::Activo),
            "ACTIVO NO CORRIENTE" => Some(Masa::Activo),
            "PASIVO" => Some(Masa::Pasivo),
            "PASIVO CORRIENTE" => Some(Masa::Pasivo),
            "PASIVO NO CORRIENTE" => Some(Masa:: Pasivo),
            "PATRIMONIO NETO" => Some(Masa::Patrimonio),
            _ => None
        };

        if let Some(mode) = modo {
            grupo = mode;
        } else {
            let read: Vec<&str> = linea.split_whitespace().take(2).collect();

            if let [codigo_cuenta, importe] = read[..] {
                let importe_parsed: f64 = importe.parse::<f64>().unwrap();

                let movimiento = Movimiento::new(importe_parsed, codigo_cuenta.to_string());

                match grupo {
                    Masa::Activo => vec_debe.push(movimiento),
                    _ => vec_haber.push(movimiento),
                }
            }
        };

    }
    cuadro.crear_asiento("Asiento de apertura".to_string(), None, vec_debe, vec_haber, generar_codigo(0));
    cuadro.libro_diario()[0].guardar_asiento("segundo");

}

fn generar_codigo(orden: usize) -> String {

    let hoy = Utc::now().date_naive().format("%Y%m%d");

    let s = format!("{}{}", hoy.to_string(), orden);

    s
}