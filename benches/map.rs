#![feature(test)]
extern crate test;
extern crate testdata;
extern crate futures;
extern crate futures_cpupool;
extern crate rayon;


use test::Bencher;
use std::sync::Arc;
use std::thread::{
    self,
    JoinHandle
};
use futures::Future;
use futures_cpupool::CpuPool;
use rayon::prelude::*;

use testdata::{
    newton,
    generate_data,
    NEWTON_ITER,
    SETLEN,
    THREADS
};

#[bench]
fn sequential(b: &mut Bencher) {
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = map_data(data.clone());
        test::black_box(new_data);
    });
}

#[bench]
fn thread_per_item(b: &mut Bencher) {
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap1_data(data.clone());
        test::black_box(new_data);
    });
}

#[bench]
fn fixed_threads(b: &mut Bencher) {
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap2_data(data.clone());
        test::black_box(new_data);
    });
}

#[bench]
fn future_threadpool(b: &mut Bencher) {
    let pool = CpuPool::new_num_cpus();
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap3_data(data.clone(), &pool);
        test::black_box(new_data);
    });
}

#[bench]
fn future_threadpool_chunked(b: &mut Bencher) {
    let pool = CpuPool::new_num_cpus();
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap4_data(data.clone(), &pool);
        test::black_box(new_data);
    });
}

#[bench]
fn fixed_threads_no_arc(b: &mut Bencher) {
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap5_data(data.clone());
        test::black_box(new_data);
    });
}

#[bench]
fn rayon(b: &mut Bencher) {
    let data = generate_data(SETLEN);

    b.iter(|| {
        let new_data = pmap6_data(data.clone());
        test::black_box(new_data);
    });
}

fn map_data(data: Vec<f64>) -> Vec<f64> {
    data.into_iter().map(|x| newton(x, NEWTON_ITER)).collect::<Vec<f64>>()
}

fn pmap1_data(data: Vec<f64>) -> Vec<f64> {
    data.into_iter()
        .map(|x| {
            thread::spawn(move || newton(x, NEWTON_ITER))
        })
        .map(|x| {
            x.join().expect("failed to join")
        })
        .collect::<Vec<f64>>()
}

fn pmap2_data(data: Vec<f64>) -> Vec<f64> {
    let mut handles: Vec<JoinHandle<Vec<f64>>> = Vec::new();
    let chunk_len = data.len()/THREADS + 1;
    let chunks = data.chunks(chunk_len).map(|x| Arc::new(x.to_owned()));
    for chunk in chunks {
        handles.push(thread::spawn(move || chunk.iter().map(|x| newton(*x, NEWTON_ITER)).collect()));
    }
    handles.into_iter().flat_map(|x| x.join().unwrap()).collect()
}

fn pmap3_data(data: Vec<f64>, pool: &CpuPool) -> Vec<f64> {
    let future_vec = data.into_iter()
        .map(|x| pool.spawn_fn(move || -> Result<f64, ()> {Ok(newton(x, NEWTON_ITER))}))
        .collect::<Vec<_>>();
    future_vec.into_iter().map(|x| x.wait().unwrap()).collect::<Vec<_>>()
}

fn pmap4_data(data: Vec<f64>, pool: &CpuPool) -> Vec<f64> {
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

fn pmap5_data(data: Vec<f64>) -> Vec<f64> {
    let mut handles: Vec<JoinHandle<Vec<f64>>> = Vec::new();
    let chunk_len = data.len()/THREADS + 1;
    let chunks = data.chunks(chunk_len).map(|x| x.to_owned());
    for chunk in chunks {
        handles.push(thread::spawn(move || chunk.into_iter().map(|x| newton(x, NEWTON_ITER)).collect()));
    }
    handles.into_iter().flat_map(|x| x.join().unwrap()).collect()
}

fn pmap6_data(data: Vec<f64>) -> Vec<f64> {
    data.par_iter().map(|&x| newton(x, NEWTON_ITER)).collect()
}
