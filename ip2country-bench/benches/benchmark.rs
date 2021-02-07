use iai::black_box;
use ip2country::{AsnDB, ShortCountryCode};

fn iai_benchmark1() -> Option<ShortCountryCode> {
    let db = AsnDB::default()
        .load_ipv4("../ip2country/test/full.csv")
        .unwrap();
    db.lookup_ipv4(black_box(3108731904).into())
}

fn iai_benchmark2() -> Option<ShortCountryCode> {
    let db = AsnDB::default()
        .load_ipv4("../ip2country/test/full.csv")
        .unwrap();
    db.lookup_ipv4(black_box(3757834240).into())
}

iai::main!(iai_benchmark1, iai_benchmark2);
