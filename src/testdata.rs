pub const NEWTON_ITER: usize = 1000;
pub const SETLEN: usize = 100000;
pub const THREADS: usize = 4;

pub fn generate_data(count: usize) -> Vec<f64> {
    (0..count).map(|x| x as f64 / count as f64).collect::<Vec<f64>>()
}

pub fn newton(n: f64, iterations: usize) -> f64 {
    const START: f64 = 1.0;
    let mut x = START;
    for _ in 0..iterations {
        x = x - (x*x - n) / (2.0*x);
    }
    x
}
