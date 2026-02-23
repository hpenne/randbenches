use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng: SmallRng = rand::make_rng();
    let mut buffer = [0u8; 256];
    c.bench_function("rand Xoshiro256++ fill_bytes", |b| b.iter(|| rng.fill_bytes(&mut buffer)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);