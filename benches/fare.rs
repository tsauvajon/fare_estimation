extern crate fare_estimation;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fare_estimation::fare_estimation::estimate_fare;
use std::io;

pub fn bench_calculate_fares_small_file(c: &mut Criterion) {
    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("calc_fares_small_file", |b| {
        b.iter(|| {
            let input = std::fs::File::open("paths.csv").unwrap();
            tokio_rt
                .block_on(async { estimate_fare(black_box(input), io::sink()).await })
                .unwrap();
        })
    });
}

pub fn bench_calculate_fares_medium_file(c: &mut Criterion) {
    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("calc_fares_medium_file", |b| {
        b.iter(|| {
            let input = std::fs::File::open("pathsbig.csv").unwrap();
            tokio_rt
                .block_on(async { estimate_fare(black_box(input), io::sink()).await })
                .unwrap();
        })
    });
}

pub fn bench_calculate_fares_large_file(c: &mut Criterion) {
    let tokio_rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("calc_fares_large_file", |b| {
        b.iter(|| {
            let input = std::fs::File::open("large.csv").unwrap();
            tokio_rt
                .block_on(async { estimate_fare(black_box(input), io::sink()).await })
                .unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_calculate_fares_small_file,
    bench_calculate_fares_medium_file,
    bench_calculate_fares_large_file,
);
criterion_main!(benches);
