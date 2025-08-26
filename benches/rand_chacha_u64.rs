use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::from_os_rng();
    c.bench_function("rand ChaCha12 u64", move |b| b.iter(|| rng.random::<u64>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);