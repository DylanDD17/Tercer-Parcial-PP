use std::time::Instant;

// Cálculo de regresión lineal: y = m*x + b
fn linear_regression(xs: &Vec<f64>, ys: &Vec<f64>) -> (f64, f64) {
    let n = xs.len() as f64;

    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = xs.iter().map(|x| x * x).sum();

    let m = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let b = (sum_y - m * sum_x) / n;

    (m, b)
}

fn main() {
    // Datos grandes para medir desempeño
    let n = 2_000_000;
    let xs: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let ys: Vec<f64> = xs.iter().map(|x| 3.2 * x + 5.0).collect();

    let start = Instant::now();
    let (m, b) = linear_regression(&xs, &ys);
    let duration = start.elapsed();

    println!("m = {}", m);
    println!("b = {}", b);
    println!("Tiempo de ejecución en RUST: {:?}", duration);
}
