
fn main() {
    let mut cuentitas:[(&str, f64, f64); 2] = [("Movistar", 170.00, 170.00), ("Supermercados", 350.00, 274.35)];
    cuentitas.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap() );
    for (nombre, presupuestado, importe) in cuentitas.into_iter() {
        let porcentaje_consumido = importe * 100.00/presupuestado;
        let grafica = (porcentaje_consumido/5.00).round();
        println!("{:<20}|{:>10} €|{:>10} €|{:>10.2} %|{:-<20}", nombre, importe, presupuestado, porcentaje_consumido, "#".repeat(grafica as usize));
    }
}
