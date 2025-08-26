use criterion::{criterion_group, criterion_main, Criterion};
use smallrand::{StdRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::new();
    let mut buffer = [0u8; 256];
    c.bench_function("smallrand ChaCha12 fill", |b| b.iter(|| rng.fill_u8(&mut buffer)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
