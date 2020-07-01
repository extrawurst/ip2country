use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ip2country::AsnDB;

fn benchmark_lookup(c: &mut Criterion) {
    let db = AsnDB::load("test/bigger.csv");

    let inputs = [16777472, 92785392, 635200168, 86453427, 16777475, 635217919];

    c.bench_function("lookup", |b| {
        b.iter(|| {
            inputs
                .iter()
                .map(|ip| db.lookup(black_box(*ip).into()))
                .flatten()
        })
    });
}

fn benchmark_load(c: &mut Criterion) {
    c.bench_function("load", |b| {
        b.iter(|| AsnDB::load(black_box("test/bigger.csv")))
    });
}

criterion_group!(benches, benchmark_lookup, benchmark_load);
criterion_main!(benches);
