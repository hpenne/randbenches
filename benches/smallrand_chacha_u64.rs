use criterion::{criterion_group, criterion_main, Criterion};
use smallrand::StdRng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::new();
    c.bench_function("smallrand ChaCha12 u64", move |b| b.iter(|| rng.random::<u64>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);