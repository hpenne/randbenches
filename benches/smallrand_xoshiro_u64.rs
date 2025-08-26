use criterion::{criterion_group, criterion_main, Criterion};
use smallrand::{SmallRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::new();
    c.bench_function("smallrand Xoshiro256++ u64", move |b| b.iter(|| rng.random::<u64>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);