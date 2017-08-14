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
use std::thread::{
    self,
    JoinHandle
};
use std::sync::Arc;

const THREADS: usize = 4;

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
    let mut handles: Vec<JoinHandle<Vec<f64>>> = Vec::new();
    let chunk_len = data.len()/THREADS + 1;
    let chunks = data.chunks(chunk_len).map(|x| Arc::new(x.to_owned()));
    for chunk in chunks {
        handles.push(thread::spawn(move || chunk.iter().map(|x| newton(*x, NEWTON_ITER)).collect()));
    }
    handles.into_iter().flat_map(|x| x.join().unwrap()).collect()
}