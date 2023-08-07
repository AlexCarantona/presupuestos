use std::fmt::Display;

/// Activos
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Activos {
    ActivoCorriente,
    ActivoNoCorriente
}

/// Pasivos
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Pasivos {
    PasivoCorriente,
    PasivoNoCorriente
}

/// Patrimonios
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Patrimonios {
    Capital,
    Gastos,
    Ingresos
}


/// Representa una masa patrimonial
#[derive(PartialEq, Debug, Hash, Eq)]
pub enum Masa {
    Activo(Activos),
    Pasivo(Pasivos),
    Patrimonio(Patrimonios)
}

#[derive(PartialEq, Debug)]
pub struct CuentaError;

/// Representa una cuenta
#[derive(PartialEq, Debug)]
pub struct Cuenta {
    /// El nombre de la cuenta, que debe ser único.
    nombre: String,
    /// El código de la cuenta, que debe ser único e informa también del grupo al que pertence.
    codigo: String,

}

impl Display for Cuenta {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:>6}) {}", self.codigo, self.nombre)
    }
}

impl Cuenta {
    /// Crea una nueva cuenta con saldo cero.
    pub fn new(nombre: &str, codigo: &str) -> Cuenta {
        Cuenta {
            nombre: String::from(nombre),
            codigo: String::from(codigo),
        }
    }

}

