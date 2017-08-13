mod testdata;
use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

use std::time::SystemTime;
use std::thread;

fn main() {
    let data = generate_data(SETLEN/10);
    println!("generiere {} testdaten", SETLEN/10);
    let earlier = SystemTime::now();
    let _new_data = map_data(data);
    let dur = match SystemTime::now().duration_since(earlier) {
        Ok(v) => v,
        Err(e) => {
            println!("messfehler: {}", e);
            return;
        }
    };
    println!("duration: {}.{} secs", dur.as_secs(), dur.subsec_nanos());
}

fn map_data(data: Vec<f64>) -> Vec<f64> {
    data.into_iter()
        .map(|x| {
            thread::spawn(move || newton(x, NEWTON_ITER))
        })
        .map(|x| {
            x.join().expect("failed to join") 
        })
        .collect::<Vec<f64>>()
}