extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

mod testdata;
use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

use std::time::SystemTime;
use std::sync::Arc;

const THREADS: usize = 8;

fn main() {
    let pool = CpuPool::new_num_cpus();
    let data = generate_data(SETLEN);
    println!("generiere {} testdaten", SETLEN);
    let earlier = SystemTime::now();
    let _ = map_data(data, &pool);
    let dur = match SystemTime::now().duration_since(earlier) {
        Ok(v) => v,
        Err(e) => {
            println!("messfehler: {}", e);
            return;
        }
    };
    println!("duration: {}.{} secs", dur.as_secs(), dur.subsec_nanos());
}

fn map_data(data: Vec<f64>, pool: &CpuPool) -> Vec<f64> {
    let mut handles: Vec<_> = Vec::new();
    let chunk_len = data.len()/THREADS + 1;
    let chunks = data.chunks(chunk_len).map(|x| Arc::new(x.to_owned()));
    for chunk in chunks {
        handles.push(pool.spawn_fn(move || -> Result<_, ()> {
            Ok(chunk.iter().map(|x| newton(*x, NEWTON_ITER)).collect::<Vec<_>>())
        }));
    }
    handles.into_iter().flat_map(|x| x.wait().unwrap()).collect()
}