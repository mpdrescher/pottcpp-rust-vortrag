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
    let chunklen = data.len() / THREADS + 1;
    data.chunks(chunklen)
        .map(|x: Chunks<&[f64]> | -> JoinHandle<Map<&[f64], Fn(&f64) -> f64>>{ //über slices
            //für jeden slice einen thread spawnen, der über den slice iteriert, und newton darauf anwendet.
            thread::spawn(move || x.map(|y| {
                newton(*y, NEWTON_ITER)
            })) //Rückgabewert: ThreadHandle<Map<f64>>
        })
        .map(|handle| handle.join().expect("failed to join"))
        .flat_map(|x| x)
        .collect()
}

/*fn map_data(data: Vec<f64>) -> Vec<f64> {
    let chunklen = data.len() / THREADS + 1;
    let chunks: Chunks<f64> = data.chunks(chunklen);
    chunks.map(|x: &[f64]| x); 
    vec!()
}*/