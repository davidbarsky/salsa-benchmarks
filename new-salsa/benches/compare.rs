use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use new_salsa::Database;

fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare");
    let mut db = Database::default();

    let input = new_salsa::Input::new(&db, String::new());

    for n in &[1] {
        group.bench_function(BenchmarkId::new("new_salsa_constant", n), |b| {
            // let text = std::iter::repeat("A").take(*n).collect::<String>();
            b.iter(|| new_salsa::run_constant(&mut db))
        });
    }

    for n in &[1] {
        group.bench_function(BenchmarkId::new("new_salsa_length", n), |b| {
            let text = std::iter::repeat("A").take(*n).collect::<String>();
            b.iter(|| new_salsa::run_length(&mut db, input, text.clone()))
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
