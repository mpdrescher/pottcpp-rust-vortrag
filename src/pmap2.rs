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

const THREADS: usize = 4;

fn main() {
    let mut data = generate_data(SETLEN);
    println!("generiere {} testdaten", SETLEN);
    let earlier = SystemTime::now();
    map_data(&mut data);
    let dur = match SystemTime::now().duration_since(earlier) {
        Ok(v) => v,
        Err(e) => {
            println!("messfehler: {}", e);
            return;
        }
    };
    println!("duration: {}.{} secs", dur.as_secs(), dur.subsec_nanos());
}

fn map_data(data: &mut Vec<f64>) {
    let chunk_size = data.len() / THREADS + 1;
    *data = {
        data.chunks(chunk_size)
            .map(|x| x.to_owned())
            .map(|x| {
                thread::spawn(move || {
                    x.into_iter().map(|x| newton(x, NEWTON_ITER))
                })
            })
            .map(|x| x.join().expect("failed to join"))
            .flat_map(|x| x)
            .collect::<Vec<f64>>()
    }
}