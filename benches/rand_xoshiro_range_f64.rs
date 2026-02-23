use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng: SmallRng = rand::make_rng();
    c.bench_function("rand Xoshiro256++ range f64", move |b| b.iter(|| rng.random_range(0_f64..42_f64)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);