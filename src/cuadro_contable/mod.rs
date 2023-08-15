use std::fmt::Display;

use chrono::NaiveDate;

use crate::cuadro_contable::asiento::Asiento;

use self::movimiento::Movimiento;

mod cuenta;
pub mod movimiento;
mod asiento;

/// Este struct almacena las cuentas y asientos,
/// y ejecuta las operaciones superficiales de inserción, control, redacción de balances, etc.
#[derive(Debug, PartialEq)]
pub struct Cuadro {
    /// Almacena las cuentas
    cuentas: Vec<cuenta::Cuenta>,
    /// Almacena los asientos
    libro_diario: Vec<asiento::Asiento>

}


/// Manejo de posibles errores de cuadro
#[derive(Debug, PartialEq)]
pub enum CuadroError {
    CuadroNoVacio,
    CuentaDuplicada(String),
}


impl Display for CuadroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            CuadroError::CuadroNoVacio => write!(f, "El cuadro ya contiene cuentas. Puedes añadir de una en una, pero no cargar el PGC"),
            CuadroError::CuentaDuplicada(cuenta_s) => write!(f, "La cuenta '{}' ya existe", cuenta_s)
        }
    }
}

impl<'a> Cuadro {

    /// Crea un nuevo cuadro de cuentas
    pub fn new() -> Cuadro {     
        Cuadro { cuentas: vec![], libro_diario: vec![] }
    }

    /// Carga todas las cuentas del Plan General de Contabilidad en el cuadro de cuentas, si este está vacío
    pub fn cargar_pgc(&mut self) -> Result<(), CuadroError> {

        if self.cuentas.len() == 0 {
            for (nombre_cuenta, codigo_cuenta) in CUENTAS_PGC {
                self.crear_cuenta(nombre_cuenta, codigo_cuenta)?;
            };
        } else { 
            return Err(CuadroError::CuadroNoVacio)
        }
        Ok(())
    }
    
    /// Encuentra una cuenta y devuelve su referencia inmutable si la encuentra
    pub fn find_cuenta(&self, codigo_cuenta: &str) -> Option<&cuenta::Cuenta> {
        for id in 0..self.cuentas.len() {
            if String::from(codigo_cuenta) == self.cuentas[id].codigo() {
                return Some(&self.cuentas[id])
            }
        };
        None
    }

    /// Crea una cuenta y la inserta en el cuadro, si no existe ya
    pub fn crear_cuenta(&mut self, nombre_cuenta: &str, codigo_cuenta: &str) -> Result<(), CuadroError> {

        match self.find_cuenta(codigo_cuenta) {
            Some(c) => {
                Err(CuadroError::CuentaDuplicada(format!("{} ~ {}", c.codigo(), c.nombre())))
            },
            None => {
                let cuenta = cuenta::Cuenta::new(nombre_cuenta, codigo_cuenta);
                self.cuentas.push(cuenta);
                Ok(())
            }
        }
    }

    /// Crea un asiento con, al menos, un movimiento. Si ya existe uno, lo sustituye.
    pub fn crear_asiento(&mut self, concepto: String, fecha: Option<NaiveDate>, debe: Vec<Movimiento>, haber: Vec<Movimiento>, codigo: String) {
        
        // Crea el asiento y deja la referencia modificable.
        let mut asiento = asiento::Asiento::new(concepto);
        asiento.fecha(fecha);
        asiento.codigo(Some(codigo.clone()));

        // Hidrata e inserta los movimientos del debe
        for mut m in debe.into_iter() {
            m.hidratar_cuenta(&self);
            asiento.insertar_debe(m)
        }

        // Inserta los movimientos del haber
        for mut m in haber.into_iter() {
            m.hidratar_cuenta(&self);
            asiento.insertar_haber(m)
        }

        // Guarda el asiento
        match self.libro_diario.iter().position(|v| v.get_codigo() == &codigo) {
            Some(r) => {self.libro_diario.remove(r); self.libro_diario.push(asiento)},
            None => {self.libro_diario.push(asiento)},
        };
        
        
    }

    /// Devuelve una lista de los asientos que existen
    pub fn libro_diario(&self) -> &Vec<asiento::Asiento> {

        &self.libro_diario
    }

    /// Imprime el listado de asientos que existen
    pub fn print_libro_diario(&self) {

        println!("{:#^120}", " LIBRO DIARIO ");

        let mut asientos_erroneos: Vec<&Asiento> = vec![];
        let mut total_asientos: usize = 0;

        for asiento in &self.libro_diario {
            println!("{}", asiento);
            total_asientos += 1;
            if !asiento.validar() {
                asientos_erroneos.push(asiento)
            }
        }

        println!("Total asientos: {}", total_asientos);
        
        if asientos_erroneos.len() > 0 {
            println!("Hay asientos en los que no coinciden las anotaciones del debe y del haber:");
            for asiento_erroneo in asientos_erroneos {
                println!("{}", asiento_erroneo.get_codigo());
            }
        } else { println!("No hay errores")}

        println!("\n");
        
    }

    /// Mayoriza e imprime las cuentas
    pub fn print_libro_mayor(&mut self) {

        println!("{:#^120}", " LIBRO MAYOR ");

        for cuenta in self.cuentas.iter_mut() {
            cuenta.mayorizar_cuenta(&self.libro_diario);
            println!("{}", cuenta);
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

      assert_eq!(cuadro, Cuadro { cuentas: vec![], libro_diario: vec![]});

    }

    #[test]
    fn cargar_pgc_carga_cuentas_plan_general_contable() {
        
        let mut cuadro = Cuadro::new();
        assert!(cuadro.cargar_pgc().is_ok());
        assert_eq!(cuadro.cuentas.len(), 902);
        
    }

    #[test]
    fn cargar_pgc_falla_si_ya_hay_cuentas() {
        let mut cuadro = Cuadro::new();
        let cuenta = cuenta::Cuenta::new("test", "0000");
        cuadro.cuentas.push(cuenta);

        assert_eq!(cuadro.cargar_pgc(), Err(CuadroError::CuadroNoVacio));
    }

    #[test]
    fn find_cuenta_encuentra_una_cuenta_por_codigo() {
        let mut cuadro = Cuadro::new();
        let cuenta = cuenta::Cuenta::new("test", "0000");
        cuadro.cuentas.push(cuenta);

        assert_eq!(cuadro.find_cuenta("0001"), None);
        assert!( match cuadro.find_cuenta("0000") {
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
        let cuenta = cuenta::Cuenta::new("test", "0000");
        cuadro.cuentas.push(cuenta);

        let r = cuadro.crear_cuenta("Nueva cuenta", "0000");
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

const CUENTAS_PGC: [(&str, &str); 902] = [
    ("CAPITAL", "10"),
    ("Capital social", "100"),
    ("Fondo social", "101"),
    ("Capital", "102"),
    ("Socios por desembolsos no exigidos", "103"),
    ("Socios por desembolsos no exigidos, capital social", "1030"),
    ("Socios por desembolsos no exigidos, capital pendiente de inscripción", "1034"),
    ("Socios por aportaciones no dinerarias pendientes", "104"),
    ("Socios por aportaciones no dinerarias pendientes, capital social", "1040"),
    ("Socios por aportaciones no dinerarias pendientes, capital pendiente de inscripción", "1044"),
    ("Acciones o participaciones propias en situaciones especiales", "108"),
    ("Acciones o participaciones propias para reducción de capital", "109"),
    ("RESERVAS Y OTROS INSTRUMENTOS DE PATRIMONIO", "11"),
    ("Prima de emisión o asunción", "110"),
    ("Otros instrumentos de patrimonio neto", "111"),
    ("Patrimonio neto por emisión de instrumentos financieros compuestos", "1110"),
    ("Resto de instrumentos de patrimonio neto", "1111"),
    ("Reserva legal", "112"),
    ("Reservas voluntarias", "113"),
    ("Reservas especiales", "114"),
    ("Reservas para acciones o participaciones de la sociedad dominante", "1140"),
    ("Reserva por capital amortizado", "1142"),
    ("Reserva por fondo de comercio", "1143"),
    ("Reservas por acciones propias aceptadas en garantía", "1144"),
    ("Reservas por pérdidas y ganancias actuariales y otros ajustes", "115"),
    ("Aportaciones de socios o propietarios", "118"),
    ("Diferencias por ajuste del capital a euros", "119"),
    ("RESULTADOS PENDIENTES DE APLICACIÓN", "12"),
    ("Remanente", "120"),
    ("Resultados negativos de ejercicios anteriores", "121"),
    ("Resultado del ejercicio", "129"),
    ("SUBVENCIONES, DONACIONES Y AJUSTES POR CAMBIOS DE VALOR", "13"),
    ("Subvenciones oficiales de capital", "130"),
    ("Donaciones y legados de capital", "131"),
    ("Otras subvenciones, donaciones y legados", "132"),
    ("Ajustes por valoración en activos financieros a valor razonable con cambios en el patrimonio neto", "133"),
    ("Operaciones de cobertura", "134"),
    ("Cobertura de flujos de efectivo", "1340"),
    ("Cobertura de una inversión neta en un negocio en el extranjero", "1341"),
    ("Diferencias de conversión", "135"),
    ("Ajustes por valoración en activos no corrientes y grupos enajenables de elementos, mantenidos para la venta", "136"),
    ("Ingresos fiscales a distribuir en varios ejercicios", "137"),
    ("Ingresos fiscales por diferencias permanentes a distribuir en varios ejercicios", "1370"),
    ("Ingresos fiscales por deducciones y bonificaciones a distribuir en varios ejercicios", "1371"),
    ("PROVISIONES", "14"),
    ("Provisión por retribuciones a largo plazo al personal", "140"),
    ("Provisión para impuestos", "141"),
    ("Provisión para otras responsabilidades", "142"),
    ("Provisión por desmantelamiento, retiro o rehabilitación del inmovilizado", "143"),
    ("Provisión para actuaciones medioambientales", "145"),
    ("Provisión para reestructuraciones", "146"),
    ("Provisión por transacciones con pagos basados en instrumentos de patrimonio", "147"),
    ("DEUDAS A LARGO PLAZO CON CARACTERÍSTICAS ESPECIALES", "15"),
    ("Acciones o participaciones a largo plazo consideradas como pasivos financieros", "150"),
    ("Desembolsos no exigidos por acciones o participaciones consideradas como pasivos financieros", "153"),
    ("Desembolsos no exigidos, empresas del grupo", "1533"),
    ("Desembolsos no exigidos, empresas asociadas", "1534"),
    ("Desembolsos no exigidos, otras partes vinculadas", "1535"),
    ("Otros desembolsos no exigidos", "1536"),
    ("Aportaciones no dinerarias pendientes por acciones o participaciones consideradas como pasivos financieros", "154"),
    ("Aportaciones no dinerarias pendientes, empresas del grupo", "1543"),
    ("Aportaciones no dinerarias pendientes, empresas asociadas", "1544"),
    ("Aportaciones no dinerarias pendientes, otras partes vinculadas", "1545"),
    ("Otras aportaciones no dinerarias pendientes", "1546"),
    ("DEUDAS A LARGO PLAZO CON PARTES VINCULADAS", "16"),
    ("Deudas a largo plazo con entidades de crédito vinculadas", "160"),
    ("Deudas a largo plazo con entidades de crédito, empresas del grupo", "1603"),
    ("Deudas a largo plazo con entidades de crédito, empresas asociadas", "1604"),
    ("Deudas a largo plazo con otras entidades de crédito vinculadas", "1605"),
    ("Proveedores de inmovilizado a largo plazo, partes vinculadas", "161"),
    ("Proveedores de inmovilizado a largo plazo, empresas del grupo", "1613"),
    ("Proveedores de inmovilizado a largo plazo, empresas asociadas", "1614"),
    ("Proveedores de inmovilizado a largo plazo, otras partes vinculadas", "1615"),
    ("Acreedores por arrendamiento financiero a largo plazo, partes vinculadas", "162"),
    ("Acreedores por arrendamiento financiero a largo plazo, empresas de grupo", "1623"),
    ("Acreedores por arrendamiento financiero a largo plazo, empresas asociadas", "1624"),
    ("Acreedores por arrendamiento financiero a largo plazo, otras partes vinculadas.", "1625"),
    ("Otras deudas a largo plazo con partes vinculadas", "163"),
    ("Otras deudas a largo plazo, empresas del grupo", "1633"),
    ("Otras deudas a largo plazo, empresas asociadas", "1634"),
    ("Otras deudas a largo plazo, con otras partes vinculadas", "1635"),
    ("DEUDAS A LARGO PLAZO POR PRÉSTAMOS RECIBIDOS, EMPRÉSTITOS Y OTROS CONCEPTOS", "17"),
    ("Deudas a largo plazo con entidades de crédito", "170"),
    ("Deudas a largo plazo", "171"),
    ("Deudas a largo plazo transformables en subvenciones, donaciones y legados", "172"),
    ("Proveedores de inmovilizado a largo plazo", "173"),
    ("Acreedores por arrendamiento financiero a largo plazo", "174"),
    ("Efectos a pagar a largo plazo", "175"),
    ("Pasivos por derivados financieros a largo plazo", "176"),
    ("Pasivos por derivados financieros a largo plazo, cartera de negociación", "1765"),
    ("Pasivos por derivados financieros a largo plazo, instrumentos de cobertura", "1768"),
    ("Obligaciones y bonos", "177"),
    ("Obligaciones y bonos convertibles", "178"),
    ("Deudas representadas en otros valores negociables", "179"),
    ("PASIVOS POR FIANZAS, GARANTÍAS Y OTROS CONCEPTOS A LARGO PLAZO", "18"),
    ("Fianzas recibidas a largo plazo", "180"),
    ("Anticipos recibidos por ventas o prestaciones de servicios a largo plazo", "181"),
    ("Depósitos recibidos a largo plazo", "185"),
    ("Garantías financieras a largo plazo", "189"),
    ("SITUACIONES TRANSITORIAS DE FINANCIACIÓN", "19"),
    ("Acciones o participaciones emitidas", "190"),
    ("Suscriptores de acciones", "192"),
    ("Capital emitido pendiente de inscripción", "194"),
    ("Acciones o participaciones emitidas consideradas como pasivos financieros", "195"),
    ("Suscriptores de acciones consideradas como pasivos financieros", "197"),
    ("Acciones o participaciones emitidas consideradas como pasivos financieros pendientes de inscripción.", "199"),
    ("INMOVILIZACIONES INTANGIBLES", "20"),
    ("Investigación", "200"),
    ("Desarrollo", "201"),
    ("Concesiones administrativas", "202"),
    ("Propiedad industrial", "203"),
    ("Fondo de comercio", "204"),
    ("Derechos de traspaso", "205"),
    ("Aplicaciones informáticas", "206"),
    ("Anticipos para inmovilizaciones intangibles", "209"),
    ("INMOVILIZACIONES MATERIALES", "21"),
    ("Terrenos y bienes naturales", "210"),
    ("Construcciones", "211"),
    ("Instalaciones técnicas", "212"),
    ("Maquinaria", "213"),
    ("Utillaje", "214"),
    ("Otras instalaciones", "215"),
    ("Mobiliario", "216"),
    ("Equipos para procesos de información", "217"),
    ("Elementos de transporte", "218"),
    ("Otro inmovilizado material", "219"),
    ("INVERSIONES INMOBILIARIAS", "22"),
    ("Inversiones en terrenos y bienes naturales", "220"),
    ("Inversiones en construcciones", "221"),
    ("INMOVILIZACIONES MATERIALES EN CURSO", "23"),
    ("Adaptación de terrenos y bienes naturales", "230"),
    ("Construcciones en curso", "231"),
    ("Instalaciones técnicas en montaje", "232"),
    ("Maquinaria en montaje", "233"),
    ("Equipos para procesos de información en montaje", "237"),
    ("Anticipos para inmovilizaciones materiales", "239"),
    ("INVERSIONES FINANCIERAS A LARGO PLAZO EN PARTES VINCULADAS", "24"),
    ("Participaciones a largo plazo en partes vinculadas", "240"),
    ("Participaciones a largo plazo en empresas del grupo", "2403"),
    ("Participaciones a largo plazo en empresas asociadas", "2404"),
    ("Participaciones a largo plazo en otras partes vinculadas", "2405"),
    ("Valores representativos de deuda a largo plazo de partes vinculadas", "241"),
    ("Valores representativos de deuda a largo plazo de empresas del grupo", "2413"),
    ("Valores representativos de deuda a largo plazo de empresas asociadas", "2414"),
    ("Valores representativos de deuda a largo plazo de otras partes vinculadas", "2415"),
    ("Créditos a largo plazo a partes vinculadas", "242"),
    ("Créditos a largo plazo a empresas del grupo", "2423"),
    ("Créditos a largo plazo a empresas asociadas", "2424"),
    ("Créditos a largo plazo a otras partes vinculadas", "2425"),
    ("Desembolsos pendientes sobre participaciones a largo plazo en partes vinculadas", "249"),
    ("Desembolsos pendientes sobre participaciones a largo plazo en empresas del grupo.", "2493"),
    ("Desembolsos pendientes sobre participaciones a largo plazo en empresas asociadas.", "2494"),
    ("Desembolsos pendientes sobre participaciones a largo plazo en otras partes vinculadas", "2495"),
    ("OTRAS INVERSIONES FINANCIERAS A LARGO PLAZO", "25"),
    ("Inversiones financieras a largo plazo en instrumentos de patrimonio", "250"),
    ("Valores representativos de deuda a largo plazo", "251"),
    ("Créditos a largo plazo", "252"),
    ("Créditos a largo plazo por enajenación de inmovilizado", "253"),
    ("Créditos a largo plazo al personal", "254"),
    ("Activos por derivados financieros a largo plazo", "255"),
    ("Activos por derivados financieros a largo plazo, cartera de negociación", "2550"),
    ("Activos por derivados financieros a largo plazo, instrumentos de cobertura", "2553"),
    ("Derechos de reembolso derivados de contratos de seguro relativos a retribuciones a largo plazo al personal", "257"),
    ("Imposiciones a largo plazo", "258"),
    ("Desembolsos pendientes sobre participaciones en el patrimonio neto a largo plazo", "259"),
    ("FIANZAS Y DEPÓSITOS CONSTITUIDOS A LARGO PLAZO", "26"),
    ("Fianzas constituidas a largo plazo", "260"),
    ("Depósitos constituidos a largo plazo", "265"),
    ("AMORTIZACIÓN ACUMULADA DEL INMOVILIZADO", "28"),
    ("Amortización acumulada del inmovilizado intangible", "280"),
    ("Amortización acumulada de investigación", "2800"),
    ("Amortización acumulada de desarrollo", "2801"),
    ("Amortización acumulada de concesiones administrativas", "2802"),
    ("Amortización acumulada de propiedad industrial", "2803"),
    ("Amortización acumulada de fondo de comercio", "2804"),
    ("Amortización acumulada de derechos de traspaso", "2805"),
    ("Amortización acumulada de aplicaciones informáticas", "2806"),
    ("Amortización acumulada del inmovilizado material", "281"),
    ("Amortización acumulada de construcciones", "2811"),
    ("Amortización acumulada de instalaciones técnicas", "2812"),
    ("Amortización acumulada de maquinaria", "2813"),
    ("Amortización acumulada de utillaje", "2814"),
    ("Amortización acumulada de otras instalaciones", "2815"),
    ("Amortización acumulada de mobiliario", "2816"),
    ("Amortización acumulada de equipos para procesos de información", "2817"),
    ("Amortización acumulada de elementos de transporte", "2818"),
    ("Amortización acumulada de otro inmovilizado material", "2819"),
    ("Amortización acumulada de las inversiones inmobiliarias", "282"),
    ("DETERIORO DE VALOR DE ACTIVOS NO CORRIENTES", "29"),
    ("Deterioro de valor del inmovilizado intangible", "290"),
    ("Deterioro de valor de investigación", "2900"),
    ("Deterioro del valor de desarrollo", "2901"),
    ("Deterioro de valor de concesiones administrativas", "2902"),
    ("Deterioro de valor de propiedad industrial", "2903"),
    ("Deterioro de valor de derechos de traspaso", "2905"),
    ("Deterioro de valor de aplicaciones informáticas", "2906"),
    ("Deterioro de valor del inmovilizado material", "291"),
    ("Deterioro de valor de terrenos y bienes naturales", "2910"),
    ("Deterioro de valor de construcciones", "2911"),
    ("Deterioro de valor de instalaciones técnicas", "2912"),
    ("Deterioro de valor de maquinaria", "2913"),
    ("Deterioro de valor de utillaje", "2914"),
    ("Deterioro de valor de otras instalaciones", "2915"),
    ("Deterioro de valor de mobiliario", "2916"),
    ("Deterioro de valor de equipos para procesos de información", "2917"),
    ("Deterioro de valor de elementos de transporte", "2918"),
    ("Deterioro de valor de otro inmovilizado material", "2919"),
    ("Deterioro de valor de las inversiones inmobiliarias", "292"),
    ("Deterioro de valor de los terrenos y bienes naturales", "2920"),
    ("Deterioro de valor de construcciones", "2921"),
    ("Deterioro de valor de participaciones a largo plazo", "293"),
    ("Deterioro de valor de participaciones a largo plazo en empresas del grupo", "2933"),
    ("Deterioro de valor de participaciones a largo plazo en empresas asociadas", "2934"),
    ("Deterioro de valor de participaciones a largo plazo en otras partes vinculadas", "2935"),
    ("Deterioro de valor de participaciones a largo plazo en otras empresas", "2936"),
    ("Deterioro de valor de valores representativos de deuda a largo plazo de partes vinculadas", "294"),
    ("Deterioro de valor de valores representativos de deuda a largo plazo de empresas del grupo", "2943"),
    ("Deterioro de valor de valores representativos de deuda a largo plazo de empresas asociadas", "2944"),
    ("Deterioro de valor de valores representativos de deuda a largo plazo de otras partes vinculadas", "2945"),
    ("Deterioro de valor de créditos a largo plazo a partes vinculadas", "295"),
    ("Deterioro de valor de créditos a largo plazo a empresas del grupo", "2953"),
    ("Deterioro de valor de créditos a largo plazo a empresas asociadas", "2954"),
    ("Deterioro de valor de créditos a largo plazo a otras partes vinculadas", "2955"),
    ("Deterioro de valor de valores representativos de deuda a largo plazo", "297"),
    ("Deterioro de valor de créditos a largo plazo", "298"),
    ("COMERCIALES", "30"),
    ("Mercaderías A", "300"),
    ("Mercaderías B", "301"),
    ("MATERIAS PRIMAS", "31"),
    ("Materias primas A", "310"),
    ("Materias primas B", "311"),
    ("OTROS APROVISIONAMIENTOS", "32"),
    ("Elementos y conjuntos incorporables", "320"),
    ("Combustibles", "321"),
    ("Repuestos", "322"),
    ("Materiales diversos", "325"),
    ("Embalajes", "326"),
    ("Envases", "327"),
    ("Material de oficina", "328"),
    ("PRODUCTOS EN CURSO", "33"),
    ("Productos en curso A", "330"),
    ("Productos en curso B", "331"),
    ("PRODUCTOS SEMITERMINADOS", "34"),
    ("Productos semiterminados A", "340"),
    ("Productos semiterminados B", "341"),
    ("PRODUCTOS TERMINADOS", "35"),
    ("Productos terminados A", "350"),
    ("Productos terminados B", "351"),
    ("SUBPRODUCTOS, RESIDUOS Y MATERIALES RECUPERADOS", "36"),
    ("Subproductos A", "360"),
    ("Subproductos B", "361"),
    ("Residuos A", "365"),
    ("Residuos B", "366"),
    ("Materiales recuperados A", "368"),
    ("Materiales recuperados B", "369"),
    ("DETERIORO DE VALOR DE LAS EXISTENCIAS", "39"),
    ("Deterioro de valor de las mercaderías", "390"),
    ("Deterioro de valor de las materias primas", "391"),
    ("Deterioro de valor de otros aprovisionamientos", "392"),
    ("Deterioro de valor de los productos en curso", "393"),
    ("Deterioro de valor de los productos semiterminados", "394"),
    ("Deterioro de valor de los productos terminados", "395"),
    ("Deterioro de valor de los subproductos, residuos y materiales recuperados", "396"),
    ("PROVEEDORES", "40"),
    ("Proveedores", "400"),
    ("Proveedores (euros)", "4000"),
    ("Proveedores (moneda extranjera)", "4004"),
    ("Proveedores, facturas pendientes de recibir o de formalizar", "4009"),
    ("Proveedores, efectos comerciales a pagar", "401"),
    ("Proveedores, empresas del grupo", "403"),
    ("Proveedores, empresas del grupo (euros)", "4030"),
    ("Efectos comerciales a pagar, empresas del grupo", "4031"),
    ("Proveedores, empresas del grupo (moneda extranjera)", "4034"),
    ("Envases y embalajes a devolver a proveedores, empresas del grupo", "4036"),
    ("Proveedores, empresas del grupo, facturas pendientes de recibir o de formalizar", "4039"),
    ("Proveedores, empresas asociadas", "404"),
    ("Proveedores, otras partes vinculadas", "405"),
    ("Envases y embalajes a devolver a proveedores", "406"),
    ("Anticipos a proveedores", "407"),
    ("ACREEDORES VARIOS", "41"),
    ("Acreedores por prestaciones de servicios", "410"),
    ("Acreedores por prestaciones de servicios (euros)", "4100"),
    ("Acreedores por prestaciones de servicios, (moneda extranjera)", "4104"),
    ("Acreedores por prestaciones de servicios, facturas pendientes de recibir o de formalizar", "4109"),
    ("Acreedores, efectos comerciales a pagar", "411"),
    ("Acreedores por operaciones en común", "419"),
    ("CLIENTES", "43"),
    ("Clientes", "430"),
    ("Clientes (euros)", "4300"),
    ("Clientes (moneda extranjera)", "4304"),
    ("Clientes, facturas pendientes de formalizar", "4309"),
    ("Clientes, efectos comerciales a cobrar", "431"),
    ("Efectos comerciales en cartera", "4310"),
    ("Efectos comerciales descontados", "4311"),
    ("Efectos comerciales en gestión de cobro", "4312"),
    ("Efectos comerciales impagados", "4315"),
    ("Clientes, operaciones de «factoring»", "432"),
    ("Clientes, empresas del grupo", "433"),
    ("Clientes empresas del grupo (euros)", "4330"),
    ("Efectos comerciales a cobrar, empresas del grupo", "4331"),
    ("Clientes empresas del grupo, operaciones de «factoring»", "4332"),
    ("Clientes empresas del grupo (moneda extranjera)", "4334"),
    ("Clientes empresas del grupo de dudoso cobro", "4336"),
    ("Envases y embalajes a devolver a clientes, empresas del grupo", "4337"),
    ("Clientes empresas del grupo, facturas pendientes de formalizar", "4339"),
    ("Clientes, empresas asociadas", "434"),
    ("Clientes, otras partes vinculadas", "435"),
    ("Clientes de dudoso cobro", "436"),
    ("Envases y embalajes a devolver por clientes", "437"),
    ("Anticipos de clientes", "438"),
    ("DEUDORES VARIOS", "44"),
    ("Deudores", "440"),
    ("Deudores (euros)", "4400"),
    ("Deudores (moneda extranjera)", "4404"),
    ("Deudores, facturas pendientes de formalizar", "4409"),
    ("Deudores, efectos comerciales a cobrar", "441"),
    ("Deudores, efectos comerciales en cartera", "4410"),
    ("Deudores, efectos comerciales descontados", "4411"),
    ("Deudores, efectos comerciales en gestión de cobro", "4412"),
    ("Deudores, efectos comerciales impagados", "4415"),
    ("Deudores de dudoso cobro", "446"),
    ("Deudores por operaciones en común", "449"),
    ("PERSONAL", "46"),
    ("Anticipos de remuneraciones", "460"),
    ("Remuneraciones pendientes de pago", "465"),
    ("Remuneraciones mediante sistemas de aportación definida pendientes de pago", "466"),
    ("ADMINISTRACIONES PÚBLICAS", "47"),
    ("Hacienda Pública, deudora por diversos conceptos", "470"),
    ("Hacienda Pública, deudora por IVA", "4700"),
    ("Hacienda Pública, deudora por subvenciones concedidas", "4708"),
    ("Hacienda Pública, deudora por devolución de impuestos", "4709"),
    ("Organismos de la Seguridad Social, deudores", "471"),
    ("Hacienda Pública, IVA soportado", "472"),
    ("Hacienda Pública, retenciones y pagos a cuenta", "473"),
    ("Activos por impuesto diferido", "474"),
    ("Activos por diferencias temporarias deducibles", "4740"),
    ("Derechos por deducciones y bonificaciones pendientes de aplicar", "4742"),
    ("Crédito por pérdidas a compensar del ejercicio", "4745"),
    ("Hacienda Pública, acreedora por conceptos fiscales", "475"),
    ("Hacienda Pública, acreedora por IVA", "4750"),
    ("Hacienda Pública, acreedora por retenciones practicadas", "4751"),
    ("Hacienda Pública, acreedora por impuesto sobre sociedades", "4752"),
    ("Hacienda Pública, acreedora por subvenciones a reintegrar", "4758"),
    ("Organismos de la Seguridad Social, acreedores", "476"),
    ("Hacienda Pública, IVA repercutido", "477"),
    ("Pasivos por diferencias temporarias imponibles", "479"),
    ("AJUSTES POR PERIODIFICACIÓN", "48"),
    ("Gastos anticipados", "480"),
    ("Ingresos anticipados", "485"),
    ("DETERIORO DE VALOR DE CRÉDITOS COMERCIALES Y PROVISIONES A CORTO PLAZO", "49"),
    ("Deterioro de valor de créditos por operaciones comerciales", "490"),
    ("Deterioro de valor de créditos por operaciones comerciales con partes vinculadas", "493"),
    ("Deterioro de valor de créditos por operaciones comerciales con empresas del grupo", "4933"),
    ("Deterioro de valor de créditos por operaciones comerciales con empresas asociadas", "4934"),
    ("Deterioro de valor de créditos por operaciones comerciales con otras partes vinculadas", "4935"),
    ("Provisiones por operaciones comerciales", "499"),
    ("Provisión por contratos onerosos", "4994"),
    ("Provisión para otras operaciones comerciales", "4999"),
    ("EMPRÉSTITOS, DEUDAS CON CARÁCTERÍSTICAS ESPECIALES Y OTRAS EMISIONES ANÁLOGAS A CORTO PLAZO", "50"),
    ("Obligaciones y bonos a corto plazo Obligaciones y bonos convertibles a corto plazo", "500"),
    ("Acciones o participaciones a corto plazo consideradas como pasivos financieros", "501"),
    ("Deudas representadas en otros valores negociables a corto plazo", "505"),
    ("Intereses a corto plazo de empréstitos y otras emisiones análogas", "506"),
    ("Dividendos de acciones o participaciones consideradas como pasivos financieros", "507"),
    ("Valores negociables amortizados", "509"),
    ("Obligaciones y bonos amortizados", "5090"),
    ("Obligaciones y bonos convertibles amortizados", "5091"),
    ("Otros valores negociables amortizados", "5095"),
    ("DEUDAS A CORTO PLAZO CON PARTES VINCULADAS", "51"),
    ("Deudas a corto plazo con entidades de crédito vinculadas", "510"),
    ("Deudas a corto plazo con entidades de crédito, empresas del grupo", "5103"),
    ("Deudas a corto plazo con entidades de crédito, empresas asociadas", "5104"),
    ("Deudas a corto plazo con otras entidades de crédito vinculadas", "5105"),
    ("Proveedores de inmovilizado a corto plazo, partes vinculadas", "511"),
    ("Proveedores de inmovilizado a corto plazo, empresas del grupo", "5113"),
    ("Proveedores de inmovilizado a corto plazo, empresas asociadas", "5114"),
    ("Proveedores de inmovilizado a corto plazo, otras partes vinculadas", "5115"),
    ("Acreedores por arrendamiento financiero a corto plazo, partes vinculadas.", "512"),
    ("Acreedores por arrendamiento financiero a corto plazo, empresas del grupo", "5123"),
    ("Acreedores por arrendamiento financiero a corto plazo, empresas asociadas", "5124"),
    ("Acreedores por arrendamiento financiero a corto plazo, otras partes vinculadas", "5125"),
    ("Otras deudas a corto plazo con partes vinculadas", "513"),
    ("Otras deudas a corto plazo con empresas del grupo", "5133"),
    ("Otras deudas a corto plazo con empresas asociadas", "5134"),
    ("Otras deudas a corto plazo con otras partes vinculadas", "5135"),
    ("Intereses a corto plazo de deudas con partes vinculadas", "514"),
    ("Intereses a corto plazo de deudas, empresas del grupo", "5143"),
    ("Intereses a corto plazo de deudas, empresas asociadas", "5144"),
    ("Intereses a corto plazo de deudas, otras partes vinculadas", "5145"),
    ("DEUDAS A CORTO PLAZO POR PRÉSTAMOS RECIBIDOS Y OTROS CONCEPTOS", "52"),
    ("Deudas a corto plazo con entidades de crédito", "520"),
    ("Préstamos a corto plazo de entidades de crédito", "5200"),
    ("Deudas a corto plazo por crédito dispuesto", "5201"),
    ("Deudas por efectos descontados", "5208"),
    ("Deudas por operaciones de «factoring»", "5209"),
    ("Deudas a corto plazo", "521"),
    ("Deudas a corto plazo transformables en subvenciones, donaciones y legados", "522"),
    ("Proveedores de inmovilizado a corto plazo", "523"),
    ("Acreedores por arrendamiento financiero a corto plazo", "524"),
    ("Efectos a pagar a corto plazo", "525"),
    ("Dividendo activo a pagar", "526"),
    ("Intereses a corto plazo de deudas con entidades de crédito", "527"),
    ("Intereses a corto plazo de deudas", "528"),
    ("Provisiones a corto plazo", "529"),
    ("Provisión a corto plazo por retribuciones al personal", "5290"),
    ("Provisión a corto plazo para impuestos", "5291"),
    ("Provisión a corto plazo para otras responsabilidades", "5292"),
    ("Provisión a corto plazo por desmantelamiento, retiro o rehabilitación del inmovilizado", "5293"),
    ("Provisión a corto plazo para actuaciones medioambientales", "5295"),
    ("Provisión a corto plazo para reestructuraciones", "5296"),
    ("Provisión a corto plazo por transacciones con pagos basados en instrumentos de patrimonio", "5297"),
    ("INVERSIONES FINANCIERAS A CORTO PLAZO EN PARTES VINCULADAS", "53"),
    ("Participaciones a corto plazo en partes vinculadas", "530"),
    ("Participaciones a corto plazo, en empresas del grupo", "5303"),
    ("Participaciones a corto plazo, en empresas asociadas", "5304"),
    ("Participaciones a corto plazo, en otras partes vinculadas", "5305"),
    ("Valores representativos de deuda a corto plazo de partes vinculadas", "531"),
    ("Valores representativos de deuda a corto plazo de empresas del grupo", "5313"),
    ("Valores representativos de deuda a corto plazo de empresas asociadas", "5314"),
    ("Valores representativos de deuda a corto plazo de otras partes vinculadas", "5315"),
    ("Créditos a corto plazo a partes vinculadas", "532"),
    ("Créditos a corto plazo a empresas del grupo", "5323"),
    ("Créditos a corto plazo a empresas asociadas", "5324"),
    ("Créditos a corto plazo a otras partes vinculadas", "5325"),
    ("Intereses a corto plazo de valores representativos de deuda de partes vinculadas", "533"),
    ("Intereses a corto plazo de valores representativos de deuda de empresas del grupo", "5333"),
    ("Intereses a corto plazo de valores representativos de deuda de empresas asociadas", "5334"),
    ("Intereses a corto plazo de valores representativos de deuda de otras partes vinculadas", "5335"),
    ("Intereses a corto plazo de créditos a partes vinculadas", "534"),
    ("Intereses a corto plazo de créditos a empresas del grupo", "5343"),
    ("Intereses a corto plazo de créditos a empresas asociadas", "5344"),
    ("Intereses a corto plazo de créditos a otras partes vinculadas", "5345"),
    ("Dividendo a cobrar de inversiones financieras en partes vinculadas", "535"),
    ("Dividendo a cobrar de empresas del grupo", "5353"),
    ("Dividendo a cobrar de empresas asociadas", "5354"),
    ("Dividendo a cobrar de otras partes vinculadas", "5355"),
    ("Desembolsos pendientes sobre participaciones a corto plazo en partes vinculadas", "539"),
    ("Desembolsos pendientes sobre participaciones a corto plazo en empresas del grupo.", "5393"),
    ("Desembolsos pendientes sobre participaciones a corto plazo en empresas asociadas.", "5394"),
    ("Desembolsos pendientes sobre participaciones a corto plazo en otras partes vinculadas", "5395"),
    ("OTRAS INVERSIONES FINANCIERAS A CORTO PLAZO", "54"),
    ("Inversiones financieras a corto plazo en instrumentos de patrimonio", "540"),
    ("Valores representativos de deuda a corto plazo", "541"),
    ("Créditos a corto plazo", "542"),
    ("Créditos a corto plazo por enajenación de inmovilizado", "543"),
    ("Créditos a corto plazo al personal", "544"),
    ("Dividendo a cobrar", "545"),
    ("Intereses a corto plazo de valores representativos de deudas", "546"),
    ("Intereses a corto plazo de créditos", "547"),
    ("Imposiciones a corto plazo", "548"),
    ("Desembolsos pendientes sobre participaciones en el patrimonio neto a corto plazo", "549"),
    ("OTRAS CUENTAS NO BANCARIAS", "55"),
    ("Titular de la explotación", "550"),
    ("Cuenta corriente con socios y administradores", "551"),
    ("Cuenta corriente con otras personas y entidades vinculadas", "552"),
    ("Cuenta corriente con empresas del grupo", "5523"),
    ("Cuenta corriente con empresas asociadas", "5524"),
    ("Cuenta corriente con otras partes vinculadas", "5525"),
    ("Cuentas corrientes en fusiones y escisiones", "553"),
    ("Socios de sociedad disuelta", "5530"),
    ("Socios, cuenta de fusión", "5531"),
    ("Socios de sociedad escindida", "5532"),
    ("Socios, cuenta de escisión", "5533"),
    ("Cuenta corriente con uniones temporales de empresas y comunidades de bienes", "554"),
    ("Partidas pendientes de aplicación", "555"),
    ("Desembolsos exigidos sobre participaciones en el patrimonio neto", "556"),
    ("Desembolsos exigidos sobre participaciones, empresas del grupo", "5563"),
    ("Desembolsos exigidos sobre participaciones, empresas asociadas", "5564"),
    ("Desembolsos exigidos sobre participaciones, otras partes vinculadas", "5565"),
    ("Desembolsos exigidos sobre participaciones de otras empresas", "5566"),
    ("Dividendo activo a cuenta", "557"),
    ("Socios por desembolsos exigidos", "558"),
    ("Socios por desembolsos exigidos sobre acciones o participaciones ordinarias", "5580"),
    ("Socios por desembolsos exigidos sobre acciones o participaciones consideradas como pasivos financieros", "5585"),
    ("Derivados financieros a corto plazo", "559"),
    ("Activos por derivados financieros a corto plazo, cartera de negociación", "5590"),
    ("Activos por derivados financieros a corto plazo, instrumentos de cobertura", "5593"),
    ("Pasivos por derivados financieros a corto plazo, cartera de negociación", "5595"),
    ("Pasivos por derivados financieros a corto plazo, instrumentos de cobertura", "5598"),
    ("FIANZAS Y DEPÓSITOS RECIBIDOS Y CONSTITUIDOS A CORTO PLAZO Y AJUSTES POR PERIODIFICACIÓN", "56"),
    ("Fianzas recibidas a corto plazo", "560"),
    ("Depósitos recibidos a corto plazo", "561"),
    ("Fianzas constituidas a corto plazo", "565"),
    ("Depósitos constituidos a corto plazo", "566"),
    ("Intereses pagados por anticipado", "567"),
    ("Intereses cobrados por anticipado", "568"),
    ("Garantías financieras a corto plazo", "569"),
    ("TESORERÍA", "57"),
    ("Caja, euros", "570"),
    ("Caja, moneda extranjera", "571"),
    ("Bancos e instituciones de crédito c/c vista, euros", "572"),
    ("Bancos e instituciones de crédito c/c vista, moneda extranjera", "573"),
    ("Bancos e instituciones de crédito, cuentas de ahorro, euros", "574"),
    ("Bancos e instituciones de crédito, cuentas de ahorro, moneda extranjera", "575"),
    ("Inversiones a corto plazo de gran liquidez", "576"),
    ("ACTIVOS NO CORRIENTES MANTENIDOS PARA LA VENTA Y ACTIVOS Y PASIVOS ASOCIADOS", "58"),
    ("Inmovilizado", "580"),
    ("Inversiones con personas y entidades vinculadas", "581"),
    ("Inversiones financieras", "582"),
    ("Existencias, deudores comerciales y otras cuentas a cobrar", "583"),
    ("Otros activos", "584"),
    ("Provisiones", "585"),
    ("Deudas con características especiales", "586"),
    ("Deudas con personas y entidades vinculadas", "587"),
    ("Acreedores comerciales y otras cuentas a pagar", "588"),
    ("Otros pasivos", "589"),
    ("DETERIORO DEL VALOR DE INVERSIONES FINANCIERAS A CORTO PLAZO Y DE ACTIVOS NO CORRIENTES MANTENIDOS PARA LA VENTA", "59"),
    ("Deterioro de valor de participaciones a corto plazo", "593"),
    ("Deterioro de valor de participaciones a corto plazo en empresas del grupo", "5933"),
    ("Deterioro de valor de participaciones a corto plazo en empresas asociadas", "5934"),
    ("Deterioro de valor de participaciones a corto plazo en otras partes vinculadas", "5935"),
    ("Deterioro de valor de participaciones a corto plazo en otras empresas.", "5936"),
    ("Deterioro de valor de valores representativos de deuda a corto plazo de partes vinculadas", "594"),
    ("Deterioro de valor de valores representativos de deuda a corto plazo de empresas del grupo", "5943"),
    ("Deterioro de valor de valores representativos de deuda a corto plazo de empresas asociadas", "5944"),
    ("Deterioro de valor de valores representativos de deuda a corto plazo de otras partes vinculadas", "5945"),
    ("Deterioro de valor de créditos a corto plazo a partes vinculadas", "595"),
    ("Deterioro de valor de créditos a corto plazo a empresas del grupo", "5953"),
    ("Deterioro de valor de créditos a corto plazo a empresas asociadas", "5954"),
    ("Deterioro de valor de créditos a corto plazo a otras partes vinculadas", "5955"),
    ("Deterioro de valor de valores representativos de deuda a corto plazo", "597"),
    ("Deterioro de valor de créditos a corto plazo", "598"),
    ("Deterioro de valor de activos no corrientes mantenidos para la venta", "599"),
    ("Deterioro de valor de inmovilizado no corriente mantenido para la venta", "5990"),
    ("Deterioro de valor de inversiones con personas y entidades vinculadas no corrientes mantenidas para la venta", "5991"),
    ("Deterioro de valor de inversiones financieras no corrientes mantenidas para la venta", "5992"),
    ("Deterioro de valor de existencias, deudores comerciales y otras cuentas a cobrar integrados en un grupo enajenable mantenido para la venta", "5993"),
    ("Deterioro de valor de otros activos mantenidos para la venta", "5994"),
    ("COMPRAS", "60"),
    ("Compras de mercaderías", "600"),
    ("Compras de materias primas", "601"),
    ("Compras de otros aprovisionamientos", "602"),
    ("Descuentos sobre compras por pronto pago", "606"),
    ("Descuentos sobre compras por pronto pago de mercaderías", "6060"),
    ("Descuentos sobre compras por pronto pago de materias primas", "6061"),
    ("Descuentos sobre compras por pronto pago de otros aprovisionamientos", "6062"),
    ("Trabajos realizados por otras empresas", "607"),
    ("Devoluciones de compras y operaciones similares", "608"),
    ("Devoluciones de compras de mercaderías", "6080"),
    ("Devoluciones de compras de materias primas", "6081"),
    ("Devoluciones de compras de otros aprovisionamientos", "6082"),
    ("«Rappels» por compras", "609"),
    ("«Rappels» por compras de mercaderías", "6090"),
    ("«Rappels» por compras de materias primas", "6091"),
    ("«Rappels» por compras de otros aprovisionamientos", "6092"),
    ("VARIACIÓN DE EXISTENCIAS", "61"),
    ("Variación de existencias de mercaderías", "610"),
    ("Variación de existencias de materias primas", "611"),
    ("Variación de existencias de otros aprovisionamientos", "612"),
    ("SERVICIOS EXTERIORES", "62"),
    ("Gastos en investigación y desarrollo del ejercicio", "620"),
    ("Arrendamientos y cánones", "621"),
    ("Reparaciones y conservación", "622"),
    ("Servicios de profesionales independientes", "623"),
    ("Transportes", "624"),
    ("Primas de seguros", "625"),
    ("Servicios bancarios y similares", "626"),
    ("Publicidad, propaganda y relaciones públicas", "627"),
    ("Suministros", "628"),
    ("Otros servicios", "629"),
    ("TRIBUTOS", "63"),
    ("Impuesto sobre beneficios", "630"),
    ("Impuesto corriente", "6300"),
    ("Impuesto diferido", "6301"),
    ("Otros tributos", "631"),
    ("Ajustes negativos en la imposición sobre beneficios", "633"),
    ("Ajustes negativos en la imposición indirecta", "634"),
    ("Ajustes negativos en IVA de activo corriente", "6341"),
    ("Ajustes negativos en IVA de inversiones", "6342"),
    ("Devolución de impuestos", "636"),
    ("Ajustes positivos en la imposición sobre beneficios", "638"),
    ("Ajustes positivos en la imposición indirecta", "639"),
    ("Ajustes positivos en IVA de activo corriente", "6391"),
    ("Ajustes positivos en IVA de inversiones", "6392"),
    ("GASTOS DE PERSONAL", "64"),
    ("Sueldos y salarios", "640"),
    ("Indemnizaciones", "641"),
    ("Seguridad Social a cargo de la empresa", "642"),
    ("Retribuciones a largo plazo mediante sistemas de aportación definida", "643"),
    ("Retribuciones a largo plazo mediante sistemas de prestación definida", "644"),
    ("Contribuciones anuales", "6440"),
    ("Otros costes", "6442"),
    ("Retribuciones al personal mediante instrumentos de patrimonio", "645"),
    ("Retribuciones al personal liquidados con instrumentos de patrimonio", "6450"),
    ("Retribuciones al personal liquidados en efectivo basado en instrumentos de patrimonio", "6457"),
    ("Otros gastos sociales", "649"),
    ("OTROS GASTOS DE GESTIÓN", "65"),
    ("Pérdidas de créditos comerciales incobrables", "650"),
    ("Resultados de operaciones en común", "651"),
    ("Beneficio transferido (gestor)", "6510"),
    ("Pérdida soportada (partícipe o asociado no gestor)", "6511"),
    ("Otras pérdidas en gestión corriente", "659"),
    ("GASTOS FINANCIEROS", "66"),
    ("Gastos financieros por actualización de provisiones", "660"),
    ("Intereses de obligaciones y bonos", "661"),
    ("Intereses de obligaciones y bonos a largo plazo, empresas del grupo", "6610"),
    ("Intereses de obligaciones y bonos a largo plazo, empresas asociadas", "6611"),
    ("Intereses de obligaciones y bonos a largo plazo, otras partes vinculadas", "6612"),
    ("Intereses de obligaciones y bonos a largo plazo, otras empresas", "6613"),
    ("Intereses de obligaciones y bonos a corto plazo, empresas del grupo", "6615"),
    ("Intereses de obligaciones y bonos a corto plazo, empresas asociadas", "6616"),
    ("Intereses de obligaciones y bonos a corto plazo, otras partes vinculadas", "6617"),
    ("Intereses de obligaciones y bonos a corto plazo, otras empresas", "6618"),
    ("Intereses de deudas", "662"),
    ("Intereses de deudas, empresas del grupo", "6620"),
    ("Intereses de deudas, empresas asociadas", "6621"),
    ("Intereses de deudas, otras partes vinculadas", "6622"),
    ("Intereses de deudas con entidades de crédito", "6623"),
    ("Intereses de deudas, otras empresas", "6624"),
    ("Pérdidas por valoración de instrumentos financieros por su valor razonable", "663"),
    ("Pérdidas de cartera de negociación", "6630"),
    ("Pérdidas de designados por la empresa", "6631"),
    ("Pérdidas de activos financieros a valor razonable con cambios en el patrimonio neto", "6632"),
    ("Pérdidas de instrumentos de cobertura", "6633"),
    ("Pérdidas de otros instrumentos financieros", "6634"),
    ("Gastos por dividendos de acciones o participaciones consideradas como pasivos financieros", "664"),
    ("Dividendos de pasivos, empresas del grupo", "6640"),
    ("Dividendos de pasivos, empresas asociadas", "6641"),
    ("Dividendos de pasivos, otras partes vinculadas", "6642"),
    ("Dividendos de pasivos, otras empresas", "6643"),
    ("Intereses por descuento de efectos y operaciones de «factoring»", "665"),
    ("Intereses por descuento de efectos en entidades de crédito del grupo", "6650"),
    ("Intereses por descuento de efectos en entidades de crédito asociadas", "6651"),
    ("Intereses por descuento de efectos en otras entidades de crédito vinculadas", "6652"),
    ("Intereses por descuento de efectos en otras entidades de crédito", "6653"),
    ("Intereses por operaciones de «factoring» con entidades de crédito del grupo", "6654"),
    ("Intereses por operaciones de «factoring» con otras entidades de crédito", "6657"),
    ("Pérdidas en participaciones y valores representativos de deuda", "666"),
    ("Pérdidas en valores representativos de deuda a largo plazo, empresas del grupo", "6660"),
    ("Pérdidas en valores representativos de deuda a largo plazo, empresas asociadas", "6661"),
    ("Pérdidas en valores representativos de deuda a largo plazo, otras partes vinculadas", "6662"),
    ("Pérdidas en participaciones y valores representativos de deuda a largo plazo, otras empresas", "6663"),
    ("Pérdidas en participaciones y valores representativos de deuda a corto plazo, empresas del grupo", "6665"),
    ("Pérdidas en participaciones y valores representativos de deuda a corto plazo, empresas asociadas", "6666"),
    ("Pérdidas en valores representativos de deuda a corto plazo, otras partes vinculadas", "6667"),
    ("Pérdidas en valores representativos de deuda a corto plazo, otras empresas", "6668"),
    ("Pérdidas de créditos no comerciales", "667"),
    ("Pérdidas de créditos a largo plazo, empresas del grupo", "6670"),
    ("Pérdidas de créditos a largo plazo, empresas asociadas", "6671"),
    ("Pérdidas de créditos a largo plazo, otras partes vinculadas", "6672"),
    ("Pérdidas de créditos a largo plazo, otras empresas", "6673"),
    ("Pérdidas de créditos a corto plazo, empresas del grupo", "6675"),
    ("Pérdidas de créditos a corto plazo, empresas asociadas", "6676"),
    ("Pérdidas de créditos a corto plazo, otras partes vinculadas", "6677"),
    ("Pérdidas de créditos a corto plazo, otras empresas", "6678"),
    ("Diferencias negativas de cambio", "668"),
    ("Otros gastos financieros", "669"),
    ("PÉRDIDAS PROCEDENTES DE ACTIVOS NO CORRIENTES Y GASTOS EXCEPCIONALES", "67"),
    ("Pérdidas procedentes del inmovilizado intangible", "670"),
    ("Pérdidas procedentes del inmovilizado material", "671"),
    ("Pérdidas procedentes de las inversiones inmobiliarias", "672"),
    ("Pérdidas procedentes de participaciones a largo plazo en partes vinculadas", "673"),
    ("Pérdidas procedentes de participaciones a largo plazo, empresas del grupo", "6733"),
    ("Pérdidas procedentes de participaciones a largo plazo, empresas asociadas", "6734"),
    ("Pérdidas procedentes de participaciones a largo plazo, otras partes vinculadas", "6735"),
    ("Pérdidas por operaciones con obligaciones propias", "675"),
    ("Gastos excepcionales", "678"),
    ("DOTACIONES PARA AMORTIZACIONES", "68"),
    ("Amortización del inmovilizado intangible", "680"),
    ("Amortización del inmovilizado material", "681"),
    ("Amortización de las inversiones inmobiliarias", "682"),
    ("PÉRDIDAS POR DETERIOROY OTRAS DOTACIONES", "69"),
    ("Pérdidas por deterioro del inmovilizado intangible", "690"),
    ("Pérdidas por deterioro del inmovilizado material", "691"),
    ("Pérdidas por deterioro de las inversiones inmobiliarias", "692"),
    ("Pérdidas por deterioro de existencias", "693"),
    ("Pérdidas por deterioro de productos terminados y en curso de fabricación", "6930"),
    ("Pérdidas por deterioro de mercaderías", "6931"),
    ("Pérdidas por deterioro de materias primas", "6932"),
    ("Pérdidas por deterioro de otros aprovisionamientos", "6933"),
    ("Pérdidas por deterioro de créditos por operaciones comerciales", "694"),
    ("Dotación a la provisión por operaciones comerciales", "695"),
    ("Dotación a la provisión por contratos onerosos", "6954"),
    ("Dotación a la provisión para otras operaciones comerciales", "6959"),
    ("Pérdidas por deterioro de participaciones y valores representativos de deuda a largo plazo", "696"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, empresas del grupo", "6960"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, empresas asociadas", "6961"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, otras partes vinculadas", "6962"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, otras empresas", "6963"),
    ("Pérdidas por deterioro en valores representativos de deuda a largo plazo, empresas del grupo", "6965"),
    ("Pérdidas por deterioro en valores representativos de deuda a largo plazo, empresas asociadas", "6966"),
    ("Pérdidas por deterioro en valores representativos de deuda a largo plazo, otras partes vinculadas", "6967"),
    ("Pérdidas por deterioro en valores representativos de deuda a largo plazo, de otras empresas", "6968"),
    ("Pérdidas por deterioro de créditos a largo plazo", "697"),
    ("Pérdidas por deterioro de créditos a largo plazo, empresas del grupo", "6970"),
    ("Pérdidas por deterioro de créditos a largo plazo, empresas asociadas", "6971"),
    ("Pérdidas por deterioro de créditos a largo plazo, otras partes vinculadas", "6972"),
    ("Pérdidas por deterioro de créditos a largo plazo, otras empresas", "6973"),
    ("Pérdidas por deterioro de participaciones y valores representativos de deuda a corto plazo", "698"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a corto plazo, empresas del grupo", "6980"),
    ("Pérdidas por deterioro de participaciones en instrumentos de patrimonio neto a corto plazo, empresas asociadas", "6981"),
    ("Pérdidas por deterioro en valores representativos de deuda a corto plazo, empresas del grupo", "6985"),
    ("Pérdidas por deterioro en valores representativos de deuda a corto plazo, empresas asociadas", "6986"),
    ("Pérdidas por deterioro en valores representativos de deuda a corto plazo, otras partes vinculadas", "6987"),
    ("Pérdidas por deterioro en valores representativos de deuda a corto plazo, de otras empresas", "6988"),
    ("Pérdidas por deterioro de créditos a corto plazo", "699"),
    ("Pérdidas por deterioro de créditos a corto plazo, empresas del grupo", "6990"),
    ("Pérdidas por deterioro de créditos a corto plazo, empresas asociadas", "6991"),
    ("Pérdidas por deterioro de créditos a corto plazo, otras partes vinculadas", "6992"),
    ("Pérdidas por deterioro de créditos a corto plazo, otras empresas", "6993"),
    ("VENTAS DE MERCADERÍAS, DE PRODUCCIÓN PROPIA, DE SERVICIOS, ETC", "70"),
    ("Ventas de mercaderías", "700"),
    ("Ventas de productos terminados", "701"),
    ("Ventas de productos semiterminados", "702"),
    ("Ventas de subproductos y residuos", "703"),
    ("Ventas de envases y embalajes", "704"),
    ("Prestaciones de servicios", "705"),
    ("Descuentos sobre ventas por pronto pago", "706"),
    ("Descuentos sobre ventas por pronto pago de mercaderías", "7060"),
    ("Descuentos sobre ventas por pronto pago de productos terminados", "7061"),
    ("Descuentos sobre ventas por pronto pago de productos semiterminados", "7062"),
    ("Descuentos sobre ventas por pronto pago de subproductos y residuos", "7063"),
    ("Devoluciones de ventas y operaciones similares", "708"),
    ("Devoluciones de ventas de mercaderías", "7080"),
    ("Devoluciones de ventas de productos terminados", "7081"),
    ("Devoluciones de ventas de productos semiterminados", "7082"),
    ("Devoluciones de ventas de subproductos y residuos", "7083"),
    ("Devoluciones de ventas de envases y embalajes", "7084"),
    ("«Rappels» sobre ventas", "709"),
    ("«Rappels» sobre ventas de mercaderías", "7090"),
    ("«Rappels» sobre ventas de productos terminados", "7091"),
    ("«Rappels» sobre ventas de productos semiterminados", "7092"),
    ("«Rappels» sobre ventas de subproductos y residuos", "7093"),
    ("«Rappels» sobre ventas de envases y embalajes", "7094"),
    ("VARIACIÓN DE EXISTENCIAS", "71"),
    ("Variación de existencias de productos en curso", "710"),
    ("Variación de existencias de productos semiterminados", "711"),
    ("Variación de existencias de productos terminados", "712"),
    ("Variación de existencias de subproductos, residuos y materiales recuperados", "713"),
    ("TRABAJOS REALIZADOS PARA LA EMPRESA", "73"),
    ("Trabajos realizados para el inmovilizado intangible", "730"),
    ("Trabajos realizados para el inmovilizado material", "731"),
    ("Trabajos realizados en inversiones inmobiliarias", "732"),
    ("Trabajos realizados para el inmovilizado material en curso", "733"),
    ("SUBVENCIONES, DONACIONESY LEGADOS", "74"),
    ("Subvenciones, donaciones y legados a la explotación", "740"),
    ("Subvenciones, donaciones y legados de capital transferidos al resultado del ejercicio", "746"),
    ("Otras subvenciones, donaciones y legados transferidos al resultado del ejercicio", "747"),
    ("OTROS INGRESOS DE GESTIÓN", "75"),
    ("Resultados de operaciones en común", "751"),
    ("Pérdida transferida (gestor)", "7510"),
    ("Beneficio atribuido (partícipe o asociado no gestor)", "7511"),
    ("Ingresos por arrendamientos", "752"),
    ("Ingresos de propiedad industrial cedida en explotación", "753"),
    ("Ingresos por comisiones", "754"),
    ("Ingresos por servicios al personal", "755"),
    ("Ingresos por servicios diversos", "759"),
    ("INGRESOS FINANCIEROS", "76"),
    ("Ingresos de participaciones en instrumentos de patrimonio", "760"),
    ("Ingresos de participaciones en instrumentos de patrimonio, empresas del grupo", "7600"),
    ("Ingresos de participaciones en instrumentos de patrimonio, empresas asociadas", "7601"),
    ("Ingresos de participaciones en instrumentos de patrimonio, otras partes vinculadas", "7602"),
    ("Ingresos de participaciones en instrumentos de patrimonio, otras empresas", "7603"),
    ("Ingresos de valores representativos de deuda", "761"),
    ("Ingresos de valores representativos de deuda, empresas del grupo", "7610"),
    ("Ingresos de valores representativos de deuda, empresas asociadas", "7611"),
    ("Ingresos de valores representativos de deuda, otras partes vinculadas", "7612"),
    ("Ingresos de valores representativos de deuda, otras empresas", "7613"),
    ("Ingresos de créditos", "762"),
    ("Ingresos de créditos a largo plazo", "7620"),
    ("Ingresos de créditos a largo plazo, empresas del grupo", "76200"),
    ("Ingresos de créditos a largo plazo, empresas asociadas", "76201"),
    ("Ingresos de créditos a largo plazo, otras partes vinculadas", "76202"),
    ("Ingresos de créditos a largo plazo, otras empresas", "76203"),
    ("Ingresos de créditos a corto plazo", "7621"),
    ("Ingresos de créditos a corto plazo, empresas del grupo", "76210"),
    ("Ingresos de créditos a corto plazo, empresas asociadas", "76211"),
    ("Ingresos de créditos a corto plazo, otras partes vinculadas", "76212"),
    ("Ingresos de créditos a corto plazo, otras empresas", "76213"),
    ("Beneficios por valoración de instrumentos financieros por su valor razonable", "763"),
    ("Beneficios de cartera de negociación", "7630"),
    ("Beneficios de designados por la empresa", "7631"),
    ("Beneficios de activos financieros a valor razonable con cambios en el patrimonio neto", "7632"),
    ("Beneficios de instrumentos de cobertura", "7633"),
    ("Beneficios de otros instrumentos financieros", "7634"),
    ("Beneficios en participaciones y valores representativos de deuda", "766"),
    ("Beneficios en valores representativos de deuda a largo plazo, empresas del grupo", "7660"),
    ("Beneficios en valores representativos de deuda a largo plazo, empresas asociadas", "7661"),
    ("Beneficios en valores representativos de deuda a largo plazo, otras partes vinculadas", "7662"),
    ("Beneficios en participaciones y valores representativos de deuda a largo plazo, otras empresas", "7663"),
    ("Beneficios en participaciones y valores representativos de deuda a corto plazo, empresas del grupo", "7665"),
    ("Beneficios en participaciones y valores representativos de deuda a corto plazo, empresas asociadas", "7666"),
    ("Beneficios en valores representativos de deuda a corto plazo, otras partes vinculadas", "7667"),
    ("Beneficios en valores representativos de deuda a corto plazo, otras empresas", "7668"),
    ("Ingresos de activos afectos y de derechos de reembolso relativos a retribuciones a largo plazo", "767"),
    ("Diferencias positivas de cambio", "768"),
    ("Otros ingresos financieros", "769"),
    ("BENEFICIOS PROCEDENTES DE ACTIVOS NO CORRIENTES E INGRESOS EXCEPCIONALES", "77"),
    ("Beneficios procedentes del inmovilizado intangible", "770"),
    ("Beneficios procedentes del inmovilizado material", "771"),
    ("Beneficios procedentes de las inversiones inmobiliarias", "772"),
    ("Beneficios procedentes de participaciones a largo plazo en partes vinculadas", "773"),
    ("Beneficios procedentes de participaciones a largo plazo, empresas del grupo", "7733"),
    ("Beneficios procedentes de participaciones a largo plazo, empresas asociadas", "7734"),
    ("Beneficios procedentes de participaciones a largo plazo, otras partes vinculadas", "7735"),
    ("Diferencia negativa en combinaciones de negocios", "774"),
    ("Beneficios por operaciones con obligaciones propias", "775"),
    ("Ingresos excepcionales", "778"),
    ("EXCESOS Y APLICACIONES DE PROVISIONES Y DE PÉRDIDAS POR DETERIORO", "79"),
    ("Reversión del deterioro del inmovilizado intangible", "790"),
    ("Reversión del deterioro del inmovilizado material", "791"),
    ("Reversión del deterioro de las inversiones inmobiliarias", "792"),
    ("Reversión del deterioro de existencias", "793"),
    ("Reversión del deterioro de productos terminados y en curso de fabricación", "7930"),
    ("Reversión del deterioro de mercaderías", "7931"),
    ("Reversión del deterioro de materias primas", "7932"),
    ("Reversión del deterioro de otros aprovisionamientos", "7933"),
    ("Reversión del deterioro de créditos por operaciones comerciales", "794"),
    ("Exceso de provisiones", "795"),
    ("Exceso de provisión por retribuciones al personal", "7950"),
    ("Exceso de provisión para impuestos", "7951"),
    ("Exceso de provisión para otras responsabilidades", "7952"),
    ("Exceso de provisión por operaciones comerciales", "7954"),
    ("Exceso de provisión por contratos onerosos", "79544"),
    ("Exceso de provisión para otras operaciones comerciales", "79549"),
    ("Exceso de provisión para actuaciones medioambientales", "7955"),
    ("Exceso de provisión para reestructuraciones", "7956"),
    ("Exceso de provisión por transacciones con pagos basados en instrumentos de patrimonio", "7957"),
    ("Reversión del deterioro de participaciones y valores representativos de deuda a largo plazo", "796"),
    ("Reversión del deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, empresas del grupo", "7960"),
    ("Reversión del deterioro de participaciones en instrumentos de patrimonio neto a largo plazo, empresas asociadas", "7961"),
    ("Reversión del deterioro de valores representativos de deuda a largo plazo, empresas del grupo", "7965"),
    ("Reversión del deterioro de valores representativos de deuda a largo plazo, empresas asociadas", "7966"),
    ("Reversión del deterioro de valores representativos de deuda a largo plazo, otras partes vinculadas", "7967"),
    ("Reversión del deterioro de valores representativos de deuda a largo plazo, otras empresas", "7968"),
    ("Reversión del deterioro de créditos a largo plazo", "797"),
    ("Reversión del deterioro de créditos a largo plazo, empresas del grupo", "7970"),
    ("Reversión del deterioro de créditos a largo plazo, empresas asociadas", "7971"),
    ("Reversión del deterioro de créditos a largo plazo, otras partes vinculadas", "7972"),
    ("Reversión del deterioro de créditos a largo plazo, otras empresas", "7973"),
    ("Reversión del deterioro de participaciones y valores representativos de deuda a corto plazo", "798"),
    ("Reversión del deterioro de participaciones en instrumentos de patrimonio neto a corto plazo, empresas del grupo", "7980"),
    ("Reversión del deterioro de participaciones en instrumentos de patrimonio neto a corto plazo, empresas asociadas", "7981"),
    ("Reversión del deterioro en valores representativos de deuda a corto plazo, empresas del grupo", "7985"),
    ("Reversión del deterioro en valores representativos de deuda a corto plazo, empresas asociadas", "7986"),
    ("Reversión del deterioro en valores representativos de deuda a corto plazo, otras partes vinculadas", "7987"),
    ("Reversión del deterioro en valores representativos de deuda a corto plazo, otras empresas", "7988"),
    ("Reversión del deterioro de créditos a corto plazo", "799"),
    ("Reversión del deterioro de créditos a corto plazo, empresas del grupo", "7990"),
    ("Reversión del deterioro de créditos a corto plazo, empresas asociadas", "7991"),
    ("Reversión del deterioro de créditos a corto plazo, otras partes vinculadas", "7992"),
    ("Reversión del deterioro de créditos a corto plazo, otras empresas", "7993"),
    ("GASTOS FINANCIEROS POR VALORACIÓN DE ACTIVOS Y PASIVOS", "80"),
    ("Pérdidas de activos financieros a valor razonable con cambios en el patrimonio neto", "800"),
    ("Transferencia de beneficios en activos financieros a valor razonable con cambios en el patrimonio neto", "802"),
    ("GASTOS EN OPERACIONES DE COBERTURA", "81"),
    ("Pérdidas por coberturas de flujos de efectivo", "810"),
    ("Pérdidas por coberturas de inversiones netas en un negocio en el extranjero", "811"),
    ("Transferencia de beneficios por coberturas de flujos de efectivo", "812"),
    ("Transferencia de beneficios por coberturas de inversiones netas en un negocio en el extranjero", "813"),
    ("GASTOS POR DIFERENCIAS DE CONVERSIÓN", "82"),
    ("Diferencias de conversión negativas", "820"),
    ("Transferencia de diferencias de conversión positivas", "821"),
    ("IMPUESTO SOBRE BENEFICIOS", "83"),
    ("Impuesto sobre beneficios", "830"),
    ("Impuesto corriente", "8300"),
    ("Impuesto diferido", "8301"),
    ("Ajustes negativos en la imposición sobre beneficios", "833"),
    ("Ingresos fiscales por diferencias permanentes", "834"),
    ("Ingresos fiscales por deducciones y bonificaciones", "835"),
    ("Transferencia de diferencias permanentes", "836"),
    ("Transferencia de deducciones y bonificaciones", "837"),
    ("Ajustes positivos en la imposición sobre beneficios", "838"),
    ("TRANSFERENCIAS DE SUBVENCIONES, DONACIONESY LEGADOS", "84"),
    ("Transferencia de subvenciones oficiales de capital", "840"),
    ("Transferencia de donaciones y legados de capital", "841"),
    ("Transferencia de otras subvenciones, donaciones y legados", "842"),
    ("GASTOS POR PÉRDIDAS ACTUARIALES Y AJUSTES EN LOS ACTIVOS POR RETRIBUCIONES A LARGO PLAZO DE PRESTACIÓN DEFINIDA", "85"),
    ("Pérdidas actuariales", "850"),
    ("Ajustes negativos en activos por retribuciones a largo plazo de prestación definida", "851"),
    ("GASTOS POR ACTIVOS NO CORRIENTES EN VENTA", "86"),
    ("Pérdidas en activos no corrientes y grupos enajenables de elementos mantenidos para la venta", "860"),
    ("Transferencia de beneficios en activos no corrientes y grupos enajenables de elementos mantenidos para la venta", "862"),
    ("GASTOS DE PARTICIPACIONES EN EMPRESAS DEL GRUPO O ASOCIADAS CON AJUSTES VALORATIVOS POSITIVOS PREVIOS", "89"),
    ("Deterioro de participaciones en el patrimonio, empresas del grupo", "891"),
    ("Deterioro de participaciones en el patrimonio, empresas asociadas", "892"),
    ("INGRESOS FINANCIEROS POR VALORACIÓN DE ACTIVOSY PASIVOS", "90"),
    ("Beneficios en activos financieros a valor razonable con cambios en el patrimonio neto", "900"),
    ("Transferencia de pérdidas de activos financieros a valor razonable con cambios en el patrimonio neto", "902"),
    ("INGRESOS EN OPERACIONES DE COBERTURA", "91"),
    ("Beneficios por coberturas de flujos de efectivo", "910"),
    ("Beneficios por coberturas de una inversión neta en un negocio en el extranjero", "911"),
    ("Transferencia de pérdidas por coberturas de flujos de efectivo", "912"),
    ("Transferencia de pérdidas por coberturas de una inversión neta en un negocio en el extranjero", "913"),
    ("INGRESOS POR DIFERENCIAS DE CONVERSIÓN", "92"),
    ("Diferencias de conversión positivas", "920"),
    ("Transferencia de diferencias de conversión negativas", "921"),
    ("INGRESOS POR SUBVENCIONES, DONACIONES Y LEGADOS", "94"),
    ("Ingresos de subvenciones oficiales de capital", "940"),
    ("Ingresos de donaciones y legados de capital", "941"),
    ("Ingresos de otras subvenciones, donaciones y legados", "942"),
    ("INGRESOS POR GANANCIAS ACTUARIALES Y AJUSTES EN LOS ACTIVOS POR RETRIBUCIONES A LARGO PLAZO DE PRESTACIÓN DEFINIDA", "95"),
    ("Ganancias actuariales", "950"),
    ("Ajustes positivos en activos por retribuciones a largo plazo de prestación definida", "951"),
    ("INGRESOS POR ACTIVOS NO CORRIENTES EN VENTA", "96"),
    ("Beneficios en activos no corrientes y grupos enajenables de elementos mantenidos para la venta", "960"),
    ("Transferencia de pérdidas en activos no corrientes y grupos enajenables de elementos mantenidos para la venta", "962"),
    ("INGRESOS DE PARTICIPACIONES EN EMPRESAS DEL GRUPO O ASOCIADAS CON AJUSTES VALORATIVOS NEGATIVOS PREVIOS", "99"),
    ("Recuperación de ajustes valorativos negativos previos, empresas del grupo", "991"),
    ("Recuperación de ajustes valorativos negativos previos, empresas asociadas", "992"),
    ("Transferencia por deterioro de ajustes valorativos negativos previos, empresas del grupo", "993"),
    ("Transferencia por deterioro de ajustes valorativos negativos previos, empresas asociadas", "994"),
];