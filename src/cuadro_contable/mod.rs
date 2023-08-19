use std::fmt::Display;

use chrono::NaiveDate;

mod cuenta;
mod movimiento;
mod asiento;
mod cuentas_pgc;
pub mod masa;

/// Este struct almacena las cuentas,
/// y ejecuta las operaciones superficiales relacionadas con ellas
#[derive(Debug, PartialEq)]
pub struct Cuadro {
    /// Almacena las cuentas
    cuentas: Vec<cuenta::Cuenta>,
}

/// Manejo de posibles errores de cuadro
#[derive(Debug, PartialEq)]
pub enum CuadroError {
    CuadroNoVacio,
    CuentaDuplicada(String),
    CuentaInexistente(String),
}

impl Display for CuadroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CuadroError::CuadroNoVacio => write!(f, "El cuadro ya contiene cuentas. Puedes añadir de una en una, pero no cargar el PGC"),
            CuadroError::CuentaDuplicada(cuenta_s) => write!(f, "La cuenta '{}' ya existe", cuenta_s),
            CuadroError::CuentaInexistente(cuenta_s) => write!(f, "El código de cuenta '{}' no existe", cuenta_s),
        }
    }
}

impl Cuadro {

    /// Crea un nuevo cuadro de cuentas
    pub fn new() -> Cuadro {     
        Cuadro { cuentas: vec![] }
    }

    /// Carga todas las cuentas del Plan General de Contabilidad en el cuadro de cuentas, si este está vacío
    pub fn cargar_pgc(&mut self) -> Result<(), CuadroError> {

        if self.cuentas.len() == 0 {
            for (nombre_cuenta, codigo_cuenta) in cuentas_pgc::CUENTAS_PGC {
                let masa = masa::interpretar_codigo(codigo_cuenta);
                if let Some(m) = masa {
                    self.crear_cuenta(nombre_cuenta, codigo_cuenta, m)?;
                } else {
                    println!("Código perdido al cargar el PGC: {}", codigo_cuenta);
                }
            };
        } else { 
            return Err(CuadroError::CuadroNoVacio)
        }
        Ok(())
    }
    
    /// Encuentra una cuenta y devuelve su referencia mutable si la encuentra
    pub fn buscar_cuenta(&mut self, codigo_cuenta: &str) -> Option<&mut cuenta::Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(codigo_cuenta) == self.cuentas[id].codigo() {
                return Some(&mut self.cuentas[id])
            }
        };
        None
    }

    /// Crea una cuenta y la inserta en el cuadro, si no existe ya
    pub fn crear_cuenta(&mut self, nombre_cuenta: &str, codigo_cuenta: &str, masa: masa::Masa) -> Result<(), CuadroError> {

        match self.buscar_cuenta(codigo_cuenta) {
            Some(c) => {
                Err(CuadroError::CuentaDuplicada(format!("{} ~ {}", c.codigo(), c.nombre())))
            },
            None => {
                let cuenta = cuenta::Cuenta::new(nombre_cuenta, codigo_cuenta, masa);
                self.cuentas.push(cuenta);
                Ok(())
            }
        }
    }

}

impl Display for Cuadro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cuenta in &self.cuentas {
            write!(f, "{}\n", cuenta)?;
        };
        Ok(())
    }
}

#[cfg(test)]
mod cuadro_tests {

    use super::*;

    #[test]
    fn new_crea_cuadro_vacio() {
      
      let cuadro = Cuadro::new();

      assert_eq!(cuadro, Cuadro { cuentas: vec![] });

    }

    #[test]
    fn cargar_pgc_carga_cuentas_plan_general_contable() {
        
        let mut cuadro = Cuadro::new();

        assert!(cuadro.cargar_pgc().is_ok());
        assert_eq!(cuadro.cuentas.len(), 899);
    }

    #[test]
    fn cargar_pgc_falla_si_ya_hay_cuentas() {
        let mut cuadro = Cuadro::new();
        let cuenta = cuenta::Cuenta::new("test", "0000", masa::Masa::ActivoCorriente);
        cuadro.cuentas.push(cuenta);

        assert_eq!(cuadro.cargar_pgc(), Err(CuadroError::CuadroNoVacio));
    }

    #[test]
    fn buscar_cuenta_encuentra_una_cuenta_por_codigo() {
        let mut cuadro = Cuadro::new();
        let cuenta = cuenta::Cuenta::new("test", "0000", masa::Masa::ActivoCorriente);
        cuadro.cuentas.push(cuenta);

        assert_eq!(cuadro.buscar_cuenta("0001"), None);
        assert!( match cuadro.buscar_cuenta("0000") {
            Some(v) => {
                assert_eq!(v.nombre(), "test");
                assert_eq!(v.codigo(), "0000");
                true
            }
            None => {false}
        })
    }

    #[test]
    fn crear_cuenta_falla_si_ya_existe() {
        let mut cuadro = Cuadro::new();
        let cuenta = cuenta::Cuenta::new("test", "0000", masa::Masa::ActivoCorriente);
        cuadro.cuentas.push(cuenta);

        let r = cuadro.crear_cuenta("Nueva cuenta", "0000", masa::Masa::ActivoCorriente);

        assert!(r.is_err());
        assert!(match r {
            Ok(()) => false,
            Err(e) => {
                assert_eq!(e.to_string(), "La cuenta '0000 ~ test' ya existe");
                true
            }
        })
    }
  
}

/// Este struct se ocupa del manejo de asientos
pub struct LibroDiario {
    asientos: Vec<asiento::Asiento>
}

#[derive(Debug, PartialEq)]
pub enum LibroDiarioError {
    AsientoDesequilibrado
}

impl Display for LibroDiarioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AsientoDesequilibrado => write!(f, "el debe y el haber del asiento que intentas insertar no coinciden")
        }
    }
}

impl LibroDiario {

    /// Crea un Libro Diario
    pub fn new() -> LibroDiario {
        LibroDiario { asientos: vec![] }
    }

    /// Crea e inserta un asiento. Este es el punto de conexión entre Libro Diario y Cuadro de Cuentas
    pub fn insertar_asiento(&mut self, concepto: &str, fecha: Option<NaiveDate>, debe: Vec<(&str, f64)>, haber: Vec<(&str, f64)>, cuadro: &mut Cuadro) -> Result<(), LibroDiarioError> {

        // Vectores para guardar movimientos de debe y haber
        let mut vec_debe: Vec<movimiento::Movimiento> = vec![];
        let mut vec_haber: Vec<movimiento::Movimiento> = vec![];

        // Busca las cuentas de debe y haber y crea un movimiento copiándolas, además de modificar sus saldos
        for (codigo_cuenta, importe) in debe.into_iter() {
            let cuenta = cuadro.buscar_cuenta(codigo_cuenta);
            if let Some(c) = cuenta {
                let movimiento = movimiento::Movimiento::new(importe, c);
                c.saldo_deudor(importe);
                vec_debe.push(movimiento)
            }
        }

        for (codigo_cuenta, importe) in haber.into_iter() {
            let cuenta = cuadro.buscar_cuenta(codigo_cuenta);
            if let Some(c) = cuenta {
                let movimiento = movimiento::Movimiento::new(importe, c);
                c.saldo_acreedor(importe);
                vec_haber.push(movimiento)
            }
        }

        // Crea el asiento
        let asiento = asiento::Asiento::new(concepto, fecha, vec_debe, vec_haber);

        // Valida e inserta
        if asiento.validar_saldos() {
            // Lo inserta en el Libro Diario
            self.asientos.push(asiento)
        } else {
            return Err(LibroDiarioError::AsientoDesequilibrado)
        }

        Ok(())

    }

}


#[cfg(test)]
mod libro_diario_tests {

    use super::*;

    fn setup_cuadro() -> Cuadro {
        let mut cuadro = Cuadro::new();

        cuadro.crear_cuenta("test", "0000", masa::Masa::ActivoCorriente).unwrap();
        cuadro.crear_cuenta("test1", "0001", masa::Masa::Patrimonio).unwrap();
        cuadro.crear_cuenta("test2", "0002", masa::Masa::PasivoCorriente).unwrap();

        cuadro
    }

    #[test]
    fn insertar_asiento_crea_asiento_y_modifica_las_cuentas() {
        let mut cuadro = setup_cuadro();
        let mut libro_diario = LibroDiario::new();

        let insercion = libro_diario.insertar_asiento(
            "Primer asiento", 
            None, 
            vec![("0000", 20.0)],
            vec![("0001", 20.0)], 
            &mut cuadro
        );

        assert!(insercion.is_ok());
        assert_eq!(libro_diario.asientos.len(), 1);

        let cuenta0000 = cuadro.buscar_cuenta("0000");
        assert!( match cuenta0000 {
            Some(v) => {assert_eq!(v.saldo(), 20.00); true},
            None => false
        });
        let cuenta0001 = cuadro.buscar_cuenta("0001");
        assert!( match cuenta0001 {
            Some(v) => {assert_eq!(v.saldo(), -20.00); true},
            None => false
        })

    }

    #[test]
    fn insertar_asiento_mal_formado_falla() {
        let mut cuadro = setup_cuadro();
        let mut libro_diario = LibroDiario::new();

        let insercion = libro_diario.insertar_asiento(
            "Primer asiento", 
            None, 
            vec![("0000", 20.0)],
            vec![("0001", 22.0)], 
            &mut cuadro
        );

        assert!(insercion.is_err());
        assert_eq!(insercion, Err(LibroDiarioError::AsientoDesequilibrado));
    }
}