use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ip2country::AsnDB;

fn benchmark_lookup(c: &mut Criterion) {
    let db = AsnDB::load("test/bigger.csv");

    let inputs = [16777472, 92785392, 635200168, 86453427, 635217919];

    c.bench_function("bench", |b| {
        b.iter(|| {
            for ip in &inputs {
                let ip: u32 = black_box(*ip);
                db.lookup(ip.into()).unwrap();
            }
        })
    });
}

criterion_group!(benches, benchmark_lookup);
criterion_main!(benches);
