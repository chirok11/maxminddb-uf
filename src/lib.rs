pub struct NormalizedDatabase {
    inner: maxminddb::Reader<Vec<u8>>,
}

#[allow(unused)]
impl NormalizedDatabase {
    pub fn lookup(
        &self,
        ip: std::net::IpAddr,
    ) -> Result<NormalizedCityRecord, maxminddb::MaxMindDBError> {
        Ok(NormalizedCityRecord {
            inner: self.inner.lookup(ip)?,
        })
    }
}

pub struct NormalizedCityRecord<'a> {
    inner: maxminddb::geoip2::City<'a>,
}

#[allow(unused)]
impl<'a> NormalizedCityRecord<'a> {
    /// Returns the country iso code
    pub fn country_code(&self) -> Option<&str> {
        self.inner.country.as_ref()?.iso_code
    }

    /// Returns the country name
    pub fn country_name(&self, language: Option<&'a str>) -> Option<String> {
        self.inner
            .country
            .as_ref()?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the registered country iso code of record [`NormalizedCityRecord`].
    pub fn registered_country_iso_code(&self) -> Option<&str> {
        self.inner.registered_country.as_ref()?.iso_code
    }

    /// Returns the registered country name of record [`NormalizedCityRecord`].
    /// Accepts a language code.
    /// Returns `None` if the country is not available.
    pub fn registered_country_name(&self, language: Option<&'a str>) -> Option<String> {
        self.inner
            .registered_country
            .as_ref()?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the registered country name of record [`NormalizedCityRecord`].
    /// Returns `None` if the country iso code is not available.
    pub fn represented_country_iso_code(&self) -> Option<&str> {
        self.inner.represented_country.as_ref()?.iso_code
    }

    /// Returns the registered country name of record [`NormalizedCityRecord`].
    /// Accepts a language code.
    /// Returns `None` if the country is not available.
    pub fn represented_country_name(&self, language: Option<&'a str>) -> Option<String> {
        self.inner
            .represented_country
            .as_ref()?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the city name of record [`NormalizedCityRecord`].
    /// Accepts a language code.
    /// Returns `None` if the city is not available.
    pub fn city_name(&self, language: Option<&'a str>) -> Option<String> {
        self.inner
            .city
            .as_ref()?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the city geoname id of record [`NormalizedCityRecord`].
    /// Returns `None` if the city geoname id is not available.
    pub fn city_geoname_id(&self) -> Option<u32> {
        self.inner.city.as_ref()?.geoname_id
    }

    /// Returns the subdivision geoname id of record [`NormalizedCityRecord`].
    /// Accepts index of subdivision.
    /// Returns `None` if the subdivision geoname id is not available.
    pub fn subdivision_geoname_id(&self, idx: usize) -> Option<u32> {
        self.inner.subdivisions.as_ref()?.get(idx)?.geoname_id
    }

    /// Returns the subdivision name of record [`NormalizedCityRecord`].
    /// Accepts index of subdivision and preferred language.
    /// Returns `None` if the subdivision name is not available.
    pub fn subdivision_name(&self, idx: usize, language: Option<&'a str>) -> Option<String> {
        self.inner
            .subdivisions
            .as_ref()?
            .get(idx)?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the subdivision iso code of record [`NormalizedCityRecord`].
    /// Accepts index of subdivision.
    /// Returns `None` if the subdivision iso code is not available.
    pub fn subdivision_iso_code(&self, idx: usize) -> Option<&str> {
        self.inner.subdivisions.as_ref()?.get(idx)?.iso_code
    }

    /// Returns the continent code of record [`NormalizedCityRecord`].
    /// Returns `None` if the continent code is not available.

    pub fn continent_code(&self) -> Option<&str> {
        self.inner.continent.as_ref()?.code
    }

    /// Returns the continent geoname id of record [`NormalizedCityRecord`].
    /// Returns `None` if the continent geoname id is not available.
    pub fn continent_geoname_id(&self) -> Option<u32> {
        self.inner.continent.as_ref()?.geoname_id
    }

    /// Returns the continent name of record [`NormalizedCityRecord`].
    /// Accepts a language code.
    /// Returns `None` if the continent name is not available.
    pub fn continent_name(&self, language: Option<&'a str>) -> Option<String> {
        self.inner
            .continent
            .as_ref()?
            .names
            .as_ref()?
            .get(language.unwrap_or("en"))
            .map(|s| s.to_string())
    }

    /// Returns the postal code of record [`NormalizedCityRecord`].
    /// Returns `None` if the postal code is not available.
    pub fn postal_code(&self) -> Option<&str> {
        self.inner.postal.as_ref()?.code
    }

    /// Returns the timezone of record [`NormalizedCityRecord`].
    /// Returns `None` if the timezone is not available.
    pub fn time_zone(&self) -> Option<&str> {
        self.inner.location.as_ref()?.time_zone
    }

    /// Returns the [`Option<(f64, f64)>`] of record [`NormalizedCityRecord`].
    /// Returns `None` if the latitude is not available.
    pub fn lon_and_lat(&self) -> Option<(f64, f64)> {
        Some((
            self.inner.location.as_ref()?.longitude?,
            self.inner.location.as_ref()?.latitude?,
        ))
    }
}

impl<'a> From<maxminddb::geoip2::City<'a>> for NormalizedCityRecord<'a> {
    fn from(city: maxminddb::geoip2::City<'a>) -> Self {
        NormalizedCityRecord { inner: city }
    }
}

impl From<maxminddb::Reader<Vec<u8>>> for NormalizedDatabase {
    fn from(reader: maxminddb::Reader<Vec<u8>>) -> Self {
        NormalizedDatabase { inner: reader }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::NormalizedDatabase;

    #[test]
    fn lookup_ip() {
        let db = maxminddb::Reader::open_readfile("./GeoLite2-City.mmdb").unwrap();
        let ndb = NormalizedDatabase::from(db);
        let addr = std::net::IpAddr::from_str("1.1.1.1").unwrap();

        let record = ndb.lookup(addr);

        assert!(record.is_ok());
    }

    #[test]
    fn validate_results() {
        let db = maxminddb::Reader::open_readfile("./GeoLite2-City.mmdb").unwrap();
        let ndb = NormalizedDatabase::from(db);
        let addr = std::net::IpAddr::from_str("8.8.8.8").unwrap();

        let record = ndb.lookup(addr);
        assert!(record.is_ok());

        let record = record.unwrap();
        assert_eq!(Some("America/Chicago"), record.time_zone());
        assert_eq!(None, record.postal_code());
        assert_eq!(None, record.city_name(None));
        assert_eq!(
            Some("North America".to_string()),
            record.continent_name(None)
        );
        assert_eq!(Some("US"), record.registered_country_iso_code());
        assert_eq!(
            Some("United States".to_string()),
            record.registered_country_name(None)
        );
    }
}
