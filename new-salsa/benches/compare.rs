use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use new_salsa::{interned_length, length, Database, Input, InternedInput};
use salsa::Setter;

fn compare_interned_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("InternedInput");
    let db = Database::default();

    group.bench_function(BenchmarkId::new("new, identical", 1), |b| {
        b.iter(|| {
            let input: InternedInput = InternedInput::new(&db, "hello, world!".to_owned());
            interned_length(&db, input);
        })
    });

    group.bench_function(BenchmarkId::new("amortized", 1), |b| {
        let input: InternedInput = InternedInput::new(&db, "hello, world!".to_owned());
        let _ = interned_length(&db, input);

        b.iter(|| interned_length(&db, input));
    });

    group.finish();
}

fn compare_plain_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("Input");
    let mut db = Database::default();

    let base_string = "hello, world!".to_owned();
    let base_len = base_string.len();
    for n in &[10, 20, 30, 40, 50] {
        let string = std::iter::repeat(base_string.clone())
            .take(*n)
            .collect::<String>();
        let new_len = string.len();

        group.bench_function(BenchmarkId::new("mutating", n), |b| {
            b.iter(|| {
                let input = Input::new(&db, base_string.clone());
                let actual_len = length(&db, input);
                assert_eq!(base_len, actual_len);

                input.set_text(&mut db).to(string.clone());
                let actual_len = length(&db, input);
                assert_eq!(new_len, actual_len);
            })
        });
    }

    group.bench_function(BenchmarkId::new("new, identical", 1), |b| {
        b.iter(|| {
            let input = Input::new(&db, "hello, world!".to_owned());
            length(&db, input);
        })
    });

    group.bench_function(BenchmarkId::new("amortized", 1), |b| {
        let input = Input::new(&db, "hello, world!".to_owned());
        let _ = length(&db, input);

        b.iter(|| length(&db, input));
    });

    group.finish();
}

criterion_group!(benches, compare_interned_input, compare_plain_input);
criterion_main!(benches);
