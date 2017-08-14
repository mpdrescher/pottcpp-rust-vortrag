mod testdata;
use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

mod map;
use map::map_data as original_map;

use std::time::SystemTime;
use std::thread;

fn main() {
    let mut data = generate_data(SETLEN);
    let data_copy = data.clone();
    println!("Es wurden {} Testdaten generiert.", SETLEN);
    let earlier = SystemTime::now();
    data = map_data(data);
    let dur = match SystemTime::now().duration_since(earlier) {
        Ok(v) => v,
        Err(e) => {
            println!("Messfehler: {}.", e);
            return;
        }
    };
    println!("Dauer: {}.{} secs", dur.as_secs(), dur.subsec_nanos());
    println!("Prüfe auf Gleichheit...");
    assert_eq!(original_map(data_copy), data);
    println!("Die Ergebnisse sind gleich.");
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