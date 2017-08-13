mod testdata;
use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

use std::time::SystemTime;
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;

const THREADS: usize = 4;

fn main() {
    let data = generate_data(SETLEN);
    println!("generiere {} testdaten", SETLEN);
    let earlier = SystemTime::now();
    let _ = map_data(data);
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
    let mut handles: Vec<JoinHandle<Vec<f64>>> = Vec::new();
    let chunk_len = data.len()/THREADS + 1;
    let chunks = data.chunks(chunk_len).map(|x| Arc::new(x.to_owned()));
    for chunk in chunks {
        handles.push(thread::spawn(move || chunk.iter().map(|x| newton(*x, NEWTON_ITER)).collect()));
    }
    handles.into_iter().flat_map(|x| x.join().unwrap()).collect()
}
