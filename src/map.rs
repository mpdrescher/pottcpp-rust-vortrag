use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

use std::time::SystemTime;

fn main() {
    let data = generate_data(SETLEN);
    println!("Es wurden {} Testdaten generiert.", SETLEN);
    let earlier = SystemTime::now();
    let _new_data = map_data(data);
    let dur = match SystemTime::now().duration_since(earlier) {
        Ok(v) => v,
        Err(e) => {
            println!("Messfehler: {}.", e);
            return;
        }
    };
    println!("Dauer: {}.{} secs", dur.as_secs(), dur.subsec_nanos());
}

pub fn map_data(data: Vec<f64>) -> Vec<f64> {
    data.into_iter().map(|x| newton(x, NEWTON_ITER)).collect::<Vec<f64>>()
}