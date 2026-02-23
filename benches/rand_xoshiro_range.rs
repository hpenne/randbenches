use criterion::{criterion_group, criterion_main, Criterion};
use rand::distr::Uniform;
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng: SmallRng = rand::make_rng();
    let dist = Uniform::new(0, 42).unwrap();
    c.bench_function("rand Xoshiro256++ range", move |b| b.iter(|| dist.sample(&mut rng)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);