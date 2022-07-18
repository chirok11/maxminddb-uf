# MaxMindDB-UF
## _User-friendly library to interact with MaxMind City database_

This is simple wrapper around `maxminddb` crate.

Example:
```rust
let reader = maxminddb::Reader::open_readfile("./GeoLite2-City.mmdb").uwnrap();
let normalized_db = NormalizedDatabase::from(reader);
let addr = IpAddr::from_str("1.1.1.1").unwrap();

let record = normalized_db.lookup(addr); // Returns Result<NormalizedRecord, MaxMindDBError>
let record = record.unwrap();
record.postal_code(); // Option<&str>
record.city_name(language? (Option<&str>); // Option<String>
record.continent_name(language? (Option<&str>); // Option<String>
```
and more functions.

# Why this wrapper is created?
Just let me show you sample of code that you may use with this crate.
```rust
let country = record
                .country_name(None)
                .or(record.registered_country_name(None))
                .or(record.represented_country_name(None));
```
or
```rust
let localized_name = record
                    .country_name(Some("de"))
                    .or(record.country_name(None));
```
I have thoughts that looks better than.
```rust
let record = maxminddb.lookup(ip);
let c = record.country
        .and_then(|c| c.names)
        .and_then(|n| n.get("de"));
if let None = c {
    let c = record.registered_country
            .and_then(|c| c.names)
            .and_then(|n| n.get("en"));
}
```
Example with falling back through represented country and registered country would be bigger instead of few lines with this wrapper.
