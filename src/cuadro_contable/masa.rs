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

    let re = Regex::new(r"(\d{1})(\d{0,1})(\d{0,1})\d*").unwrap();

    // Grupo y número de cuenta
    let mut grupo= "";
    let mut subgrupo = "";
    let mut cuenta = "";
    let mut n_cuenta = "";

    // Captura el código
    if let Some(cap) = re.captures(codigo) {
        grupo = cap.get(1).unwrap().as_str();
        if cap.len() > 2 { // Hay subgrupo
            subgrupo = cap.get(2).unwrap().as_str();
        }
        if cap.len() >= 3 { // Hay cuenta
            cuenta = cap.get(3).unwrap().as_str();
        }
        n_cuenta = cap.get(0).unwrap().as_str();
    }

    // Interpretación de masas según el PGC
    return match grupo {
        "1" => match subgrupo { // Financiación básica
            "0" => Some(Masa::Patrimonio), // Capital
            "1" => Some(Masa::Patrimonio), // Reservas
            "2" => Some(Masa::Patrimonio), // Resultados
            "3" => Some(Masa::Patrimonio), // Subvenciones
            "4" => Some(Masa::PasivoNoCorriente), // Provisiones
            "5" => Some(Masa::PasivoNoCorriente), // Deudas a largo plazo con características especiales
            "6" => Some(Masa::PasivoNoCorriente), // Deudas a largo plazo con asociados
            "7" => Some(Masa::PasivoNoCorriente), // Deudas a largo plazo por préstamos y similares
            "8" => Some(Masa::PasivoNoCorriente), // Deudas a largo plazo por fianzas y garantías
            _ => None
        },
        "2" => Some(Masa::ActivoNoCorriente), // Inmovilizado
        "3" => Some(Masa::ActivoCorriente),
        "4" => match subgrupo { // Deudores y acreedores
            "0" => Some(Masa::PasivoCorriente), // Proveedores
            "1" => Some(Masa::PasivoCorriente), // Acreedores varios
            "2" => Some(Masa::PasivoNoCorriente), // No aparece en el PGC, cuadro libre para las deudas a largo plazo
            "3" => Some(Masa::ActivoCorriente), // Clientes
            "4" => Some(Masa::ActivoCorriente), // Deudores varios
            "5" => Some(Masa::ActivoNoCorriente), // No aparece en el PGC, cuadro libre para los créditos a largo plazo
            "6" => match n_cuenta { // Personal de la empresa
                "460" => Some(Masa::ActivoCorriente), // Anticipos de remuneraciones
                "465" => Some(Masa::PasivoCorriente), // Remuneraciones pendientes de pago
                _ => None
            },
            "7" => match cuenta { // Administraciones públicas
                "0" => Some(Masa::ActivoCorriente), // Créditos, Hacienda (devoluciones)
                "1" => Some(Masa::ActivoCorriente), // Créditos, SS
                "2" => Some(Masa::ActivoCorriente), // Créditos, Hacienda (IVA soportado)
                "3" => Some(Masa::ActivoCorriente), // Créditos, Hacienda (rretenciones y pagos a cuenta)
                "4" => Some(Masa::ActivoCorriente), // Créditos, Hacienda (pérdidas)
                "5" => Some(Masa::PasivoCorriente), // Deudas, Hacienda
                "6" => Some(Masa::PasivoCorriente), // Deudas, SS
                "7" => Some(Masa::PasivoCorriente), // Deudas, Hacienda (IVA repercutido)
                _ => None
            },
            "8" => match cuenta { // Gastos e ingresos anticipados
                "0" => Some(Masa::ActivoCorriente), // Gastos anticipados, imputación temporal
                "5" => Some(Masa::PasivoCorriente), // Ingresos anticipados, imputación temporal
                _ => None
            },
            "9" => Some(Masa::PasivoCorriente), // Provisiones por operaciones comerciales
            _=> None
        },
        "5" => match subgrupo { // Cuentas financieras
            "4" => Some(Masa::ActivoNoCorriente), // Créditos a corto plazo
            "7" => Some(Masa::ActivoCorriente),
            _ => None
        },
        "6" => Some(Masa::Gasto), // Compras y gastos
        "7" => Some(Masa::Ingreso), // Ventas e ingresos
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