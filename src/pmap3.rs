extern crate testdata;
extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

mod map;
use map::map_data as original_map;

use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN
};

use std::time::SystemTime;

fn main() {
    let pool = CpuPool::new_num_cpus();
    let mut data = generate_data(SETLEN);
    let data_copy = data.clone();
    println!("Es wurden {} Testdaten generiert.", SETLEN);
    let earlier = SystemTime::now();
    data = map_data(data, &pool);
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

fn map_data(data: Vec<f64>, pool: &CpuPool) -> Vec<f64> {
    let future_vec = data.into_iter()
        .map(|x| pool.spawn_fn(move || -> Result<f64, ()> {Ok(newton(x, NEWTON_ITER))}))
        .collect::<Vec<_>>();
    future_vec.into_iter().map(|x| x.wait().unwrap()).collect::<Vec<_>>()
}