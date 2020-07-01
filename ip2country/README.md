# ip2country

provides loading of geo+whois+asn+country `csv` files (see: https://github.com/sapics/ip-location-db) and provides fast and lightweight ip lookup.

the file format hast to be in this format:
```
ip-range-start,ip-range-end,short-country-code
```

the lines need to be sorted ascending by the `ip-range-start` column and the ranges need to be free of gaps.

# TODO

* [x] ipv6 support
* [ ] gap support