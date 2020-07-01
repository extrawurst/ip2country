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
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    ops::Add,
    path::Path,
    str::FromStr,
};

pub type ShortCountryCode = [u8; 2];

// trait BaseType<T>: FromStr + From<u32> + PartialEq + Copy + Add<Output = T> {}

#[repr(packed(2))]
struct Asn<T> {
    start: T,
    code: ShortCountryCode,
}

const_assert_eq!(std::mem::size_of::<Asn<u32>>(), 4 + 2);
const_assert_eq!(std::mem::size_of::<Asn<u128>>(), 16 + 2);

impl<T> Asn<T>
where
    T: FromStr + From<u32> + PartialEq + Copy + Add<Output = T> + std::fmt::Debug,
{
    fn new(from: String, _last_end: Option<T>) -> Option<(Self, T)> {
        let mut components = from.split(',');

        let start = components.next().unwrap().parse::<T>().ok()?;
        let end = components.next().unwrap().parse::<T>().ok()?;
        let code_bytes = components.next().unwrap().as_bytes();

        // if let Some(last_end) = last_end {
        //     if last_end + T::from(1) != start {
        //         dbg!("{}+1 != {}", &last_end, &start);
        //         return None;
        //     }
        // }

        let mut code = [0, 0];
        code.copy_from_slice(code_bytes);

        Some((Self { code, start }, end))
    }
}

///
#[derive(Default)]
pub struct AsnDB {
    ip_db_v4: Vec<Asn<u32>>,
    ip_db_v6: Vec<Asn<u128>>,
}

impl AsnDB {
    ///
    #[must_use]
    pub fn load_ipv4(mut self, file: &str) -> Self {
        self.ip_db_v4 = Self::load_file(file);
        self
    }

    ///
    #[must_use]
    pub fn load_ipv6(mut self, file: &str) -> Self {
        self.ip_db_v6 = Self::load_file(file);
        self
    }

    ///
    #[must_use]
    pub fn lookup(&self, ip: IpAddr) -> Option<ShortCountryCode> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip),
            IpAddr::V6(ip) => self.lookup_ipv6(ip),
        }
    }

    ///
    #[inline]
    pub fn lookup_ipv4(&self, ip: Ipv4Addr) -> Option<ShortCountryCode> {
        Self::lookup_num::<u32>(&self.ip_db_v4, ip.into())
    }

    ///
    #[inline]
    pub fn lookup_ipv6(&self, ip: Ipv6Addr) -> Option<ShortCountryCode> {
        Self::lookup_num::<u128>(&self.ip_db_v6, ip.into())
    }

    ///
    fn lookup_num<T>(entries: &Vec<Asn<T>>, ip: T) -> Option<ShortCountryCode>
    where
        T: PartialOrd + Copy,
    {
        if entries.is_empty() {
            return None;
        }

        let len = entries.len();
        let first = entries[0].start;
        let last = entries[len - 1].start;

        if ip < first {
            return None;
        } else if ip > last {
            return Some(entries[len - 1].code);
        }

        Some(Self::recursive_search_num::<T>(entries, ip, 0, len))
    }

    fn recursive_search_num<T>(
        entries: &Vec<Asn<T>>,
        ip: T,
        min: usize,
        max: usize,
    ) -> ShortCountryCode
    where
        T: PartialOrd + Copy,
    {
        if max == min + 1 {
            return entries[min].code;
        }

        let mid = min + ((max - min) / 2);
        let mid_value = entries[mid].start;

        if ip >= mid_value {
            return Self::recursive_search_num(entries, ip, mid, max);
        } else {
            return Self::recursive_search_num(entries, ip, min, mid);
        }
    }

    ///
    #[must_use]
    pub fn lookup_str(&self, ip: IpAddr) -> Option<String> {
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

    fn load_file<T>(file: &str) -> Vec<Asn<T>>
    where
        T: FromStr + From<u32> + PartialEq + Copy + Add<Output = T> + std::fmt::Debug,
    {
        let mut entries = Vec::new();

        if let Ok(lines) = Self::read_lines(file) {
            let mut last_end = None;
            for line in lines {
                let line = line.unwrap();

                let entry = Asn::<T>::new(line, last_end).unwrap();

                last_end = Some(entry.1);

                entries.push(entry.0);
            }
        }

        entries.shrink_to_fit();

        entries
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smoke() {
        let db = AsnDB::default();

        assert_eq!(db.lookup_ipv4(16842752.into()), None);
    }

    #[test]
    fn test_asn_parse() {
        let v = Asn::<u32>::new(String::from("1234,1235,AA"), None)
            .unwrap()
            .0;

        let start = v.start;

        assert_eq!(start, 1234);
    }

    #[test]
    fn test_load_db() {
        let db = AsnDB::default().load_ipv4("test/example.csv");

        assert_eq!(db.ip_db_v4.len(), 78);
    }

    #[test]
    fn test_load_ipv4() {
        let db = AsnDB::load_file::<u32>("test/example.csv");

        assert_eq!(db.len(), 78);
    }

    #[test]
    fn test_lookup() {
        let db = AsnDB::default().load_ipv4("test/example.csv");

        assert_eq!(db.lookup_ipv4(16842752.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup_ipv4(16843007.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup_ipv4(16843008.into()).unwrap(), "AU".as_bytes());
    }

    #[test]
    fn test_lookup_fail() {
        let db = AsnDB::default().load_ipv4("test/example.csv");

        assert_eq!(db.lookup_ipv4(16777215.into()).is_none(), true);
    }

    #[test]
    fn test_lookup_last() {
        let db = AsnDB::default().load_ipv4("test/example.csv");

        assert_eq!(db.lookup_ipv4(28311551.into()).unwrap(), "TW".as_bytes());
    }
}
