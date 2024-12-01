//TODO:
// #![deny(clippy::result_unwrap_used)]

mod error;

use ascii::AsciiChar;
use error::Result;
use static_assertions::const_assert_eq;
use std::{
    fs::File,
    io::{self, BufRead},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    num::ParseIntError,
    ops::Add,
    str::FromStr,
};

pub type ShortCountryCode = [AsciiChar; 2];

#[repr(packed(1))]
struct Asn<T> {
    start: T,
    code: Option<ShortCountryCode>,
}

const_assert_eq!(std::mem::size_of::<AsciiChar>(), 1);
const_assert_eq!(std::mem::size_of::<Option<AsciiChar>>(), 1);
const_assert_eq!(std::mem::size_of::<ShortCountryCode>(), 2);
const_assert_eq!(std::mem::size_of::<Option<ShortCountryCode>>(), 2);
const_assert_eq!(std::mem::size_of::<Asn<u32>>(), 4 + 2);
const_assert_eq!(std::mem::size_of::<Asn<u128>>(), 16 + 2);

impl<T> Asn<T>
where
    T: FromStr + From<u32> + PartialEq + Copy + Add<Output = T> + std::fmt::Debug,
{
    fn new(
        from: &str,
        last_end: Option<T>,
    ) -> std::result::Result<(Option<Self>, Self, T), T::Err> {
        let mut components = from.split(',');

        let start = components.next().unwrap().parse::<T>()?;
        let end = components.next().unwrap().parse::<T>()?;
        let code_bytes = components.next().unwrap().as_bytes();

        let gap = last_end.and_then(|last_end| {
            if last_end + T::from(1) == start {
                None
            } else {
                Some(Self {
                    start: last_end + T::from(1),
                    code: None,
                })
            }
        });

        let mut code: [AsciiChar; 2] = [AsciiChar::Null, AsciiChar::Null];
        for (i, code) in code.iter_mut().enumerate() {
            *code = AsciiChar::from_ascii(code_bytes[i]).unwrap();
        }

        Ok((
            gap,
            Self {
                code: Some(code),
                start,
            },
            end,
        ))
    }
}

// converts byte array country code to string version of it
fn code_to_str(code: ShortCountryCode) -> Option<String> {
    let bytes = [code[0].as_byte(), code[1].as_byte()];
    std::str::from_utf8(bytes.as_ref()).map(String::from).ok()
}

/// stores thow lookup DBs, one for ipv4 and one for ipv6
#[derive(Default)]
pub struct AsnDB {
    ip_db_v4: Vec<Asn<u32>>,
    ip_db_v6: Vec<Asn<u128>>,
}

impl AsnDB {
    /// loads csv file of format: ip-range-start (v4),ip-range-end,short-country-code
    ///
    /// # Errors
    ///
    /// Will return `Err` if `file` does not exist or the user does not have
    /// permission to read it, or when the content was not in the correct format
    pub fn load_ipv4(mut self, file: &str) -> Result<Self> {
        self.ip_db_v4 = Self::from_reader(File::open(file)?)?;
        Ok(self)
    }

    /// loads csv file of format: ip-range-start (v4),ip-range-end,short-country-code
    /// from a reader
    pub fn load_ipv4_from_reader<R: std::io::Read>(mut self, reader: R) -> Result<Self> {
        self.ip_db_v4 = Self::from_reader(reader)?;
        Ok(self)
    }

    /// loads csv file of format: ip-range-start (v6),ip-range-end,short-country-code
    ///
    /// # Errors
    ///
    /// Will return `Err` if `file` does not exist or the user does not have
    /// permission to read it, or when the content was not in the correct format
    pub fn load_ipv6(mut self, file: &str) -> Result<Self> {
        self.ip_db_v6 = Self::from_reader(File::open(file)?)?;
        Ok(self)
    }

    /// loads csv file of format: ip-range-start (v4),ip-range-end,short-country-code
    /// from a reader
    pub fn load_ipv6_from_reader<R: std::io::Read>(mut self, reader: R) -> Result<Self> {
        self.ip_db_v6 = Self::from_reader(reader)?;
        Ok(self)
    }

    /// lookup ip to country and returning in the format of a `ShortCountryCode`
    #[must_use]
    pub fn lookup(&self, ip: IpAddr) -> Option<ShortCountryCode> {
        match ip {
            IpAddr::V4(ip) => self.lookup_ipv4(ip),
            IpAddr::V6(ip) => self.lookup_ipv6(ip),
        }
    }

    /// lookup a ipv4 address
    #[must_use]
    pub fn lookup_ipv4(&self, ip: Ipv4Addr) -> Option<ShortCountryCode> {
        Self::lookup_num::<u32>(&self.ip_db_v4, ip.into())
    }

    /// lookup a ipv6 address
    #[must_use]
    pub fn lookup_ipv6(&self, ip: Ipv6Addr) -> Option<ShortCountryCode> {
        Self::lookup_num::<u128>(&self.ip_db_v6, ip.into())
    }

    fn lookup_num<T>(entries: &[Asn<T>], ip: T) -> Option<ShortCountryCode>
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
            return entries[len - 1].code;
        }

        Self::recursive_search_num::<T>(entries, ip, 0, len)
    }

    fn recursive_search_num<T>(
        entries: &[Asn<T>],
        ip: T,
        min: usize,
        max: usize,
    ) -> Option<ShortCountryCode>
    where
        T: PartialOrd + Copy,
    {
        if max == min + 1 {
            return entries[min].code;
        }

        let middle = min + ((max - min) / 2);
        let mid_value = entries[middle].start;

        if ip >= mid_value {
            Self::recursive_search_num(entries, ip, middle, max)
        } else {
            Self::recursive_search_num(entries, ip, min, middle)
        }
    }

    /// does the lookup and converts the result to a `String`
    #[must_use]
    pub fn lookup_str(&self, ip: IpAddr) -> Option<String> {
        self.lookup(ip).and_then(code_to_str)
    }

    fn read_lines<R>(reader: R) -> io::Result<io::Lines<io::BufReader<R>>>
    where
        R: std::io::Read,
    {
        Ok(io::BufReader::new(reader).lines())
    }

    fn from_reader<T, R>(reader: R) -> Result<Vec<Asn<T>>>
    where
        R: std::io::Read,
        T: FromStr<Err = ParseIntError>
            + From<u32>
            + PartialEq
            + Copy
            + Add<Output = T>
            + std::fmt::Debug,
    {
        let mut entries = Vec::new();

        let lines = Self::read_lines(reader)?;

        let mut last_end = None;
        for line in lines {
            let line = line?;

            let (gap, entry, end) = Asn::<T>::new(&line, last_end)?;

            last_end = Some(end);

            if let Some(gap) = gap {
                entries.push(gap);
            }

            entries.push(entry);
        }

        entries.shrink_to_fit();

        Ok(entries)
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
        let v = Asn::<u32>::new("1234,1235,AA", None).unwrap().1;

        let start = v.start;

        assert_eq!(start, 1234);
    }

    #[test]
    fn test_load_db() {
        let db = AsnDB::default().load_ipv4("test/example.csv").unwrap();

        assert_eq!(db.ip_db_v4.len(), 78);
    }

    #[test]
    fn test_load_ipv4() {
        let db = AsnDB::from_reader::<u32>(File::open("test/example.csv").unwrap()).unwrap();

        assert_eq!(db.len(), 78);
    }

    #[test]
    fn test_lookup() {
        let db = AsnDB::default().load_ipv4("test/example.csv").unwrap();

        assert_eq!(db.lookup_ipv4(16842752.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup_ipv4(16843007.into()).unwrap(), "CN".as_bytes());
        assert_eq!(db.lookup_ipv4(16843008.into()).unwrap(), "AU".as_bytes());
    }

    #[test]
    fn test_lookup_fail() {
        let db = AsnDB::default().load_ipv4("test/example.csv").unwrap();

        assert_eq!(db.lookup_ipv4(16777215.into()).is_none(), true);
    }

    #[test]
    fn test_lookup_last() {
        let db = AsnDB::default().load_ipv4("test/example.csv").unwrap();

        assert_eq!(db.lookup_ipv4(28311551.into()).unwrap(), "TW".as_bytes());
    }

    #[test]
    fn test_gaps() {
        let db = AsnDB::default().load_ipv4("test/gap.csv").unwrap();

        assert_eq!(db.lookup_ipv4(16777470.into()).unwrap(), "AU".as_bytes());
        assert_eq!(db.lookup_ipv4(16777471.into()), None);
        assert_eq!(db.lookup_ipv4(16777472.into()).unwrap(), "CN".as_bytes());
    }
}
