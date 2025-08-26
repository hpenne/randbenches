use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fastrand range", move |b| b.iter(|| fastrand::u64(0_u64..42)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);