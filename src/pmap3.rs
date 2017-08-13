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
    let future_vec = data.into_iter()
        .map(|x| pool.spawn_fn(move || -> Result<f64, ()> {Ok(newton(x, NEWTON_ITER))}))
        .collect::<Vec<_>>();
    future_vec.into_iter().map(|x| x.wait().unwrap()).collect::<Vec<_>>()
}