use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_example(c: &mut Criterion) {
    c.bench_function("example", |b| {
        b.iter(|| {
            // benchmark code here
        })
    });
}

criterion_group!(benches, benchmark_example);
criterion_main!(benches);