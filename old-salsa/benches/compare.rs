use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use old_salsa::TestDatabase;

fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare");
    let mut db = TestDatabase::default();

    for n in &[1] {
        group.bench_function(BenchmarkId::new("old_salsa_constant", n), |b| {
            b.iter(|| old_salsa::run_constant(&mut db))
        });
    }

    for n in &[1] {
        group.bench_function(BenchmarkId::new("old_salsa_length", n), |b| {
            let text = std::iter::repeat("A").take(*n).collect::<String>();
            b.iter(|| old_salsa::run_string_length(&mut db, text.clone()))
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
