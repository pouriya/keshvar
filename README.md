# Keshvar
This library contains collection of all sorts of useful information for every country.

[Package](https://crates.io/crates/keshvar)   |   [Documentation](https://docs.rs/keshvar)   |   [Repository](https://github.com/pouriya/keshvar)

# Features
* Cargo features to support different countries, sub-regions, regions, continents, and world regions.  
* For every country it supports:
  * Short and long and unofficial names (parts of ISO 3166)
  * Currency code
  * Address format
  * Official and spoken languages
  * Nationality
  * Postal code format
  * Start of the week
  * Phone number (E.164)
  * ...
* Country Subdivisions. (Optional)
* GEO locations for countries and their subdivisions (Optional)
* Translations for countries and subdivisions (Optional)
* [`serde`](https://docs.rs/serde) integration (Optional)
* [`chrono`](https://docs.rs/chrono) integration (Optional)
* [`iso_currency (ISO 4217)`](https://docs.rs/iso_currency) integration (Optional)

# Installation
Add this library as a dependency (from commandline or do it manually) in your `Cargo.toml` file.
#### Commandline
Run bellow command inside root directory of your Cargo project:
```shell
cargo add keshvar 
```
#### Manual
Add `keshvar = "0.1"` under `dependencies` section inside your `Cargo.toml` file.  

Now you're ready to use it inside your Cargo project.

### Cargo features
By default, all countries are included. Additionally, you can add `subdivisions`, `translations`, `geo`, `serde-derive`, `chrono-integration`, and `iso-currency-integration` to your `Cargo.toml` file:
```toml
[dependencies]
# Include:
# - all countries (which is the default).
# - Translations for all country names.
# - Geo for all countries.
# - serde support for serializing/deserializing keshvar's structs and enums
keshvar = {version = "<VERSION>", features = ["translations", "geo", "serde-derive"]}
```
If you do not want to support all countries, You can disable default features and include what countries you want:
```toml
[dependencies]
# Include:
# - only USA and Englang.
# - Subdivisions for included countries (here only USA and Englang).
keshvar = {version = "<VERSION>", default-features = false, features = ["us", "gb", "subdivisions"]}
```
Additionally, You can only include countries for different continents, regions, subregions, and world regions:
```toml
[dependencies]
# Include:
# - Countries of `asia` continent.
# - Countries of `oceania` region.
# - Countries of `northern-africa` subregion.
keshvar = {version = "<VERSION>", default-features = false, features = ["asia", "oceania", "northern-africa"]}
```
[Continent](crate::Continent) feature names: `africa` | `antarctica` | `asia` | `australia` | `europe` | `north-america` | `south-america`

[Region](crate::Region) feature names: `americas` | `oceania` | `region-africa` | `region-antarctica` | `region-asia` | `region-europe`

[Subregion](crate::SubRegion) feature names: `australia-and-new-zealand` | `caribbean` | `central-america` | `central-asia` | `eastern-africa` | `eastern-asia` | `eastern-europe` | `melanesia` | `micronesia` | `middle-africa` | `northern-africa` | `northern-america` | `northern-europe` | `polynesia` | `south-eastern-asia` | `southern-africa` | `southern-asia` | `southern-europe` | `subregion-south-america` | `western-africa` | `western-asia` | `western-europe`

[World region](crate::WorldRegion) feature names: `amer` | `apac` | `emea`

# Examples

## Country struct
Main struct of this library is the [`Country`](crate::Country) struct. Most other types have a `to_country()` method, and we usually want to convert them to this struct.
```rust
use keshvar::{Country, CurrencyCode, WeekDay};

let country = Country::try_from("US").unwrap(); // "US" is its alpha2 code
assert_eq!("United States of America", country.iso_short_name());
assert_eq!("The United States of America", country.iso_long_name());
assert!(country.unofficial_name_list().contains(&"United States"));
assert_eq!(CurrencyCode::USD, country.currency_code());
assert_eq!(WeekDay::Sunday, country.start_of_week());
```
For more info see [`Country`](crate::Country).

## Iterating
```rust
use keshvar::{
  CountryIterator,   // To iterate over all included countries
  ContinentIterator, // To iterate over all supported continents (based on included countries)
  SubRegionIterator, // To iterate over all supported subregions (based on included countries)
};
use keshvar::{Alpha2, SubRegion, WeekDay, CurrencyCode};

let mut list = Vec::new();
// Doing iteration by for loop:
for country in CountryIterator::new() {
    let start_at_mon = country.start_of_week() == WeekDay::Monday;
    let use_usd_currency = country.currency_code() == CurrencyCode::USD;
    if start_at_mon && use_usd_currency {
        list.push(country)
    }
}
// Found 17 countries:
assert_eq!(17, list.len());
assert!(list.contains(&Alpha2::ZW.to_country())); // It contains The Republic of Zimbabwe (ZW alpha2 code)


// Use Any `Iterator` method you want:
let supported_continents = ContinentIterator::new().count();
assert_eq!(7, supported_continents);

// Doing iteration in a functional way:
let list: Vec<_> = SubRegionIterator::new()
    // Start filtering our supported subregions:
    .filter(
        |subregion| {
            subregion
                // Get alpha2 codes for countries of this subregion:
                .alpha2_list()
                // Iterate over them:
                .iter()
                // Try convert them to Alpha2 enum:
                .filter_map(|alpha2_str| Alpha2::try_from(*alpha2_str).ok())
                // Convert Alpha2 enums to Country structs:
                .map(|alpha2| alpha2.to_country())
                // Take countries that their start day of the week is sunday:
                .filter(|country| country.start_of_week() == WeekDay::Sunday)
                // Check if this filter has more than 4 output items:
                .count() > 4
        }
    ).collect();
// So there is just one subregion that contains more than 4 countries that
// their start day of the week is sunday:
assert_eq!(1, list.len());
assert_eq!(SubRegion::WesternAsia, list[0]);
```

## Subdivisions
Enable `subdivisions` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "<VERSION>", features = ["subdivisions"]}
```
##### Example
```rust
use std::collections::HashMap;
use keshvar::{GEC, Country, Subdivision, SubdivisionType, SubdivisionGeo};

// Load from GEC (Geopolitical Entities and Codes)
let country: Country = GEC::UK.to_country(); // England
// A hashmap containing string subdivision codes as keys and `Subdivision` structs as values:
let subdivisions: &HashMap<&str, Subdivision> = country.subdivisions();
let london = subdivisions.get("LND").unwrap();
assert_eq!("London, City of", london.name());
assert_eq!(SubdivisionType::CityCorporation, london.subdivision_type());

// If you enabled `translations` feature:
assert_eq!(Some(&"مدينة لندن"), london.translations().get("ar")); // Arabic

// If you enabled `geo` feature:
let geo = london.geo().unwrap();
assert_eq!(Some(51.5073509), geo.latitude());
assert_eq!(Some(-0.1277583), geo.longitude());
```

## Translations
Enable `translations` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "<VERSION>", features = ["translations"]}
```
##### Example
```rust
use std::collections::HashMap;
use keshvar::{Alpha2, CountryIterator};

// Load from alpha2 code
let country = Alpha2::CN.to_country(); // China
// A hashmap containing languages as keys and translations as values:
let translations: &HashMap<&str, &str> = country.translations();
assert_eq!(Some(&"Chine"), translations.get("fr")); // French
assert_eq!(Some(&"Китай"), translations.get("ru")); // Russian

// Find in all translations for country name "Ізраїль"
let search_text = "Ізраїль";
// Iterate over all included countries:
let country = CountryIterator::new()
    .find(
        |country| {
            //  Search for the given name in translations:
            country
                .translations()
                .values()
                .collect::<Vec<&&str>>()
                .contains(&&search_text)
        }
    ).unwrap();
// Actually "Ізраїль" is "Israel" in ukrainian language
assert_eq!("Israel", country.iso_short_name());
```

### GEO
Enable `geo` feature inside `Cargo.toml` file:
```toml
[dependencies]
keshvar = {version = "<VERSION>", features = ["geo"]}
```
##### Example
```rust
use keshvar::{IOC, Country, CountryGeo, CountryGeoBounds};

// Load from IOC (International Olympic Committee)
let country: Country = IOC::INA.to_country(); // The Republic of Indonesia (Asia)
let geo: CountryGeo = country.geo();
assert_eq!((-0.789275, 113.921327), (geo.latitude(), geo.longitude()));
assert_eq!((6.216999899999999, 141.0425), (geo.max_latitude(), geo.max_longitude()));
assert_eq!((-11.1082999, 94.7351), (geo.min_latitude(), geo.min_longitude()));
let bounds = geo.bounds();
assert_eq!((6.216999899999999, 141.0425), (bounds.northeast().latitude(), bounds.northeast().longitude()));
assert_eq!((-11.1082999, 94.7351), (bounds.southwest().latitude(), bounds.southwest().longitude()));
```

## Utility functions
```rust
use keshvar::{Alpha2, Alpha3, Region, SubRegion, Continent};
// Utility functions:
use keshvar::{find_by_iso_short_name, find_by_iso_long_name, find_by_code};

let country = find_by_iso_short_name("united states of america").unwrap();
assert_eq!(Some("American"), country.nationality());

let country = find_by_iso_long_name("ukraine").unwrap();
assert_eq!(Alpha2::UA, country.alpha2());
assert_eq!(Some(SubRegion::EasternEurope), country.subregion());

let country = find_by_code(971).unwrap(); // The United Arab Emirates (Asia)
assert_eq!(Alpha3::ARE, country.alpha3());
assert_eq!(Continent::Asia, country.continent());
```

# Contribution
See [**CONTRIBUTING.md** file](https://github.com/pouriya/keshvar/blob/master/CONTRIBUTING.md).

# License
`keshvar` source-code generator and the generated source are distributed under BSD-3-Clause license (See `LICENSE` file) but the data that is used to feed the generator is distributed under MIT License (See [`countries` license file](https://github.com/countries/countries/blob/master/LICENSE)).  
