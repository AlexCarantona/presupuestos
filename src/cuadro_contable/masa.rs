use regex::Regex;

/// Categorización en masas para clasificar las cuentas, calcular sus saldos
/// y ayudar en la interpretación de balances de apertura y la redacción
/// de cierres contables

#[derive(Debug, PartialEq)]
pub enum Masa {
    ActivoCorriente,
    ActivoNoCorriente,
    PasivoCorriente,
    PasivoNoCorriente,
    Patrimonio,
    Ingreso,
    Gasto
}

/// Toma un código númerico e interpreta, según en PGC, a qué masa corresponde
pub fn interpretar_codigo(codigo: &str) -> Option<Masa> {

    let re = Regex::new(r"(\d{1})(\d{1})\d*").unwrap();

    // Grupo y número de cuenta
    let mut grupo= "";
    let mut subgrupo = "";
    let mut n_cuenta = "";

    // Captura el código
    if let Some(cap) = re.captures(codigo) {
        grupo = cap.get(1).unwrap().as_str();
        if cap.len() > 2 { // Hay subgrupo
            subgrupo = cap.get(2).unwrap().as_str();
        }
        n_cuenta = cap.get(0).unwrap().as_str();
    }

    return match grupo {
        "2" => Some(Masa::ActivoNoCorriente),
        "3" => Some(Masa::ActivoCorriente),
        "5" => match subgrupo {
            "7" => Some(Masa::ActivoCorriente),
            _ => None
        },
        "6" => Some(Masa::Gasto),
        "7" => Some(Masa::Ingreso),
        "8" => Some(Masa::Gasto),
        "9" => Some(Masa::Ingreso),
        _ => None
    }
}

#[cfg(test)]
mod masa_tests {
    use crate::cuadro_contable::masa::interpretar_codigo;


    use super::*;

    #[test]
    fn interpretar_codigo_devuelve_none_si_no_esta_bien_formado() {

        let random_str = "hsjhuek";
        assert_eq!(interpretar_codigo(random_str), None);
    }

    #[test]
    fn interpretar_codigo_devuelve_Masa() {
        let codigo = "60";
        assert_eq!(interpretar_codigo(codigo), Some(Masa::Gasto));
    }

}