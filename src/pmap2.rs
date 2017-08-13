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
use std::iter::Map;
use std::slice::Chunks;
use std::sync::Arc;
use std::boxed::Box;
use std::sync::Mutex;

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
    let mut handles:Vec<JoinHandle<Vec<f64>>> = Vec::new();
    let mut data = data.chunks(data.len()/THREADS + 1).
                                        map(|x| Arc::new(x.to_owned()));
    println!("{}", data.len());
    for d in data {
        handles.push(thread::spawn(move || d.iter().map(|x| newton(*x, NEWTON_ITER)).collect()));
    }
    handles.into_iter().flat_map(|x| x.join().unwrap()).collect()
}
