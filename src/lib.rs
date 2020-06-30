#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
//TODO:
// #![deny(clippy::result_unwrap_used)]

use static_assertions::const_assert_eq;
use std::{
    fs::File,
    io::{self, BufRead},
    net::Ipv4Addr,
    path::Path,
};

pub type ShortCountryCode = [u8; 2];

#[repr(packed(2))]
struct Asn<T> {
    start: T,
    code: ShortCountryCode,
}

const_assert_eq!(std::mem::size_of::<Asn<u32>>(), 4 + 2);

///
pub struct AsnDB {
    entries: Vec<Asn<u32>>,
}

impl AsnDB {
    ///
    #[must_use]
    pub fn load(file: &str) -> Self {
        let mut entries = Vec::new();

        if let Ok(lines) = Self::read_lines(file) {
            let mut last_end = None;
            for line in lines {
                let line = line.unwrap();
                let mut components = line.split(',');

                let start: u32 = components.next().unwrap().parse().unwrap();
                let end: u32 = components.next().unwrap().parse().unwrap();
                let code_bytes = components.next().unwrap().as_bytes();

                if let Some(ref mut last_end) = &mut last_end {
                    //fail with a result
                    assert_eq!(*last_end, start - 1);

                    *last_end = end;
                }

                let mut code = [0, 0];
                code.copy_from_slice(code_bytes);

                entries.push(Asn { start, code });
            }
        }

        entries.shrink_to_fit();

        Self { entries }
    }

    ///
    #[must_use]
    pub fn lookup(&self, ip: Ipv4Addr) -> Option<ShortCountryCode> {
        let ip: u32 = ip.into();
        let mut last_code = None;

        for entry in &self.entries {
            if entry.start > ip {
                break;
            }
            last_code = Some(entry.code);
        }

        last_code
    }

    ///
    #[must_use]
    pub fn lookup_str(&self, ip: Ipv4Addr) -> Option<String> {
        self.lookup(ip)
            .as_ref()
            .and_then(|r| std::str::from_utf8(r).ok())
            .map(String::from)
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load() {
        let db = AsnDB::load("test/example.csv");

        assert_eq!(db.entries.len(), 78);

        assert_eq!(std::mem::size_of::<Asn<u32>>(), 4 + 2);
    }

    #[test]
    fn test_lookup() {
        let db = AsnDB::load("test/example.csv");

        assert_eq!(db.lookup(16842752.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup(16843007.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup(16843008.into()).unwrap(), "AU".as_bytes());
    }
}
