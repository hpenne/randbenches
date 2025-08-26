use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rngs::SmallRng::from_os_rng();
    c.bench_function("rand Xoshiro256++ range", move |b| b.iter(|| rng.random_range(0_u64..42_u64)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);