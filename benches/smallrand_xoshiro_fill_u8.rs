use criterion::{criterion_group, criterion_main, Criterion};
use smallrand::{SmallRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::new();
    let mut buffer = [0u8; 256];
    c.bench_function("smallrand Xoshiro256++ fill", |b| b.iter(|| rng.fill_u8(&mut buffer)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
