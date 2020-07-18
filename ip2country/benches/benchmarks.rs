use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ip2country::AsnDB;

fn benchmark_lookup(c: &mut Criterion) {
    let db = AsnDB::default().load_ipv4("test/full.csv").unwrap();

    let inputs = [
        3108731904, 16777216, 16908288, 3274341888, 1045158206, 3758096383, 3758096128, 3715741696,
    ];

    c.bench_function("lookup", |b| {
        b.iter(|| {
            inputs
                .iter()
                .filter_map(|ip| db.lookup_ipv4(black_box(*ip).into()))
                .count()
        })
    });
}

fn benchmark_load(c: &mut Criterion) {
    c.bench_function("load", |b| {
        b.iter(|| AsnDB::default().load_ipv4(black_box("test/full.csv")))
    });
}

criterion_group!(benches, benchmark_lookup, benchmark_load);
criterion_main!(benches);
