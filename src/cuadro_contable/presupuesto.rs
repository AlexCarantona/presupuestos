use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, offset, Days, Months};

use super::Cuadro;

use super::cuenta;


/// Almacena los rangos de fechas de inicio y fin a los que se aplica un presupuesto.
#[derive(Debug, PartialEq)]
struct RangoFechas {
    inicio: NaiveDate,
    fin: NaiveDate
}

#[derive(Debug, PartialEq)]
pub struct RangoError;

impl RangoFechas {

    fn create(fecha_inicio: Option<NaiveDate>, fecha_fin: Option<NaiveDate>) -> Result<RangoFechas, RangoError> {
        
        let mut inicio: NaiveDate;
        let mut fin: NaiveDate;

        let now = offset::Local::now().date_naive();
        let next_month_begin = {now + Months::new(1)}.with_day(1).ok_or(RangoError);
        let next_month_end = {now + Months::new(2)}.with_day(1).ok_or(RangoError);

        match fecha_inicio {
            Some(d) => inicio = d,
            None => inicio = next_month_begin.unwrap(),
        }

        match fecha_fin {
            Some(d) => fin = d,
            None => fin = next_month_end.unwrap() - Days::new(1),
        }

        // Falla si la fecha de fin es anterior a la de inicio
        if fin < inicio {
            return Err(RangoError)
        }

        Ok(RangoFechas { inicio, fin })
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn create_rangoFechas_funciona() {
        let fechas = RangoFechas::create(None, None).unwrap();

        let now = offset::Local::now().date_naive();

        assert_eq!(fechas.inicio.month(), now.month() + 1);
        assert_eq!(fechas.fin.month(), now.month() + 1);
    }

    #[test]
    fn create_rangoFechas_falla_si_fin_es_anterior_a_inicio() {
        let fallo = RangoFechas::create(
            NaiveDate::from_ymd_opt(2023, 12, 12),
            NaiveDate::from_ymd_opt(2022, 12, 12)
        );

        assert_eq!(fallo, Err(RangoError));
    }
}

/// Permite distinguir gastos o ingresos para luego realizar los cálculos necesarios
#[derive(Debug, PartialEq)]
enum ImportePresupuesto {
    Diario(f64),
    Puntual(f64),
}

/// Almacena cada item por separado, para luego ofrecer una abstracción por cuentas que se pueda comparar
#[derive(Debug, PartialEq)]
pub struct ItemPresupuesto {
    concepto: String,
    cuenta: String,
    presupuesto: ImportePresupuesto
}

impl ItemPresupuesto {

    fn item_diario(concepto: &str, cuadro: &Cuadro, cuenta: &str, importe: f64) -> Result<ItemPresupuesto, cuenta::CuentaError> {

        if !cuadro.validar_cuenta(cuenta) {
            return Err(cuenta::CuentaError)
        }

        Ok( ItemPresupuesto { concepto: concepto.to_string(), cuenta: cuenta.to_string(), presupuesto: ImportePresupuesto::Diario(importe) })
    }

    
    fn item_puntual(concepto: &str, cuadro: &Cuadro, cuenta: &str, importe: f64) -> Result<ItemPresupuesto, cuenta::CuentaError> {

        if !cuadro.validar_cuenta(cuenta) {
            return Err(cuenta::CuentaError)
        }

        Ok( ItemPresupuesto { concepto: concepto.to_string(), cuenta: cuenta.to_string(), presupuesto: ImportePresupuesto::Puntual(importe) })
    }
}

/// Contiene una previsión de ingresos y gastos para un periodo determinado, ordenados por Cuentas
#[derive(Debug, PartialEq)]
pub struct Presupuesto<'a> {
    // Imprescindibles fechas de inicio y fin; por defecto, del próximo mes
    fechas: RangoFechas,
    // Listado de elementos presupuestados
    items: Vec<ItemPresupuesto>,
    // Partidas presupuestarias resumidas por cuentas
    partidas: HashMap<String, f64>,
    // Cuadro contable de referencia
    cuadro: &'a Cuadro
}

impl Presupuesto<'_> {

    pub fn new(inicio: Option<NaiveDate>, fin: Option<NaiveDate>, cuadro: &Cuadro) -> Result<Presupuesto, RangoError> {

        let rango = RangoFechas::create(inicio, fin).unwrap();

        Ok(Presupuesto {
            fechas: rango,
            items: vec![],
            partidas: HashMap::new(),
            cuadro
        })
    }

    fn actualizar_partida(&mut self, item: &ItemPresupuesto) {

        let mut partida = self.partidas.entry(item.cuenta.clone()).or_insert(0.00);

        let importe = match item.presupuesto {
            ImportePresupuesto::Diario(v) => v * {{self.fechas.fin - self.fechas.inicio}.num_days() as f64 + 1.00} ,
            ImportePresupuesto::Puntual(v) => v,
        };

        *partida += importe;

    }

    pub fn insertar_gasto_diario(&mut self, concepto: &str, cuenta: &str, importe: f64) {
        let gasto = ItemPresupuesto::item_diario(concepto, &self.cuadro, cuenta, importe).unwrap();
        self.actualizar_partida(&gasto);
        self.items.push(gasto);
    }


}

#[cfg(test)]
mod tests2 {
    use chrono::{NaiveDate};

    use crate::cuadro_contable::{Cuadro};

    use super::Presupuesto;


    #[test]
    fn insertar_item_actualiza_presupuesto() {

        let mut cuadro = Cuadro::create();
        cuadro.crear_cuenta("Supermercados", crate::cuadro_contable::cuenta::Masa::Patrimonio(crate::cuadro_contable::cuenta::Patrimonios::Gastos));

        let fecha_inicio = NaiveDate::from_ymd_opt(2023, 6, 28);
        let fecha_fin= NaiveDate::from_ymd_opt(2023, 6, 30);
        let mut presupuesto = Presupuesto::new(fecha_inicio, fecha_fin, &cuadro).unwrap();

        presupuesto.insertar_gasto_diario("Compra básica", "Supermercados", 15.00);

        assert_eq!(presupuesto.partidas.get(&String::from("Supermercados")), Some(&45.00));
        assert_eq!(presupuesto.items.len(), 1);
    }
}