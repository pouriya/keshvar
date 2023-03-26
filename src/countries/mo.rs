// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Macao Special Administrative Region of China

#[cfg(all(feature = "mo", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MO;
    pub const ALPHA3: Alpha3 = Alpha3::MAC;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 853;
    pub const CURRENCY_CODE: &str = "MOP";
    pub const GEC: Option<GEC> = Some(GEC::MC);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Macao";
    pub const ISO_LONG_NAME: &str = "The Macao Special Administrative Region of China";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pt", "zh"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pt", "zh"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Chinese");
    pub const NUMBER: &str = "446";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "MO";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Macao", "Macau", "マカオ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Macao"),
        ("af", "Macao"),
        ("ak", "Macao"),
        ("am", "ሢጢፅ"),
        ("an", "Macao"),
        ("ar", "مك\u{651}او"),
        ("as", "মেক\u{9be}ও"),
        ("ay", "Macao"),
        ("az", "Makao"),
        ("ba", "Macao"),
        ("be", "Макаа"),
        ("bg", "Макао"),
        ("bi", "Macao"),
        ("bn", "ম\u{9cd}য\u{9be}ক\u{9be}ও"),
        ("bn_IN", "ম\u{9cd}য\u{9be}ক\u{9be}ও"),
        ("br", "Macao"),
        ("bs", "Makau"),
        ("ca", "Macau"),
        ("ce", "Macao"),
        ("ch", "Macao"),
        ("cs", "Macao"),
        ("cv", "Macao"),
        ("cy", "Macao"),
        ("da", "Macao"),
        ("de", "Macao"),
        ("dv", "Macao"),
        ("dz", "མ་ཀའ\u{f7c}།"),
        ("ee", "Macao"),
        ("el", "Μακάο"),
        ("en", "Macao"),
        ("eo", "Makao"),
        ("es", "Macao"),
        ("et", "Aomen"),
        ("eu", "Macau"),
        ("fa", "ماکائو"),
        ("ff", "Macao"),
        ("fi", "Macao"),
        ("fo", "Macao"),
        ("fr", "Macau"),
        ("fy", "Macao"),
        ("ga", "Macao"),
        ("gl", "Macau"),
        ("gn", "Macao"),
        ("gu", "મકાઉ"),
        ("gv", "Macao"),
        ("ha", "Macao"),
        ("he", "מאקאו"),
        ("hi", "मकाउ"),
        ("hr", "Makao"),
        ("ht", "Macao"),
        ("hu", "Makaó"),
        ("hy", "Աոմին"),
        ("ia", "Macao"),
        ("id", "Macao"),
        ("io", "Macao"),
        ("is", "Makaó"),
        ("it", "Macao"),
        ("iu", "Macao"),
        ("ja", "マカオ"),
        ("ka", "მაკაო"),
        ("ki", "Macao"),
        ("kk", "Макао"),
        ("kl", "Macao"),
        ("km", "ម\u{17c9}ាកាវ"),
        ("kn", "ಮಕಾವು"),
        ("ko", "마카오"),
        ("ku", "Makao"),
        ("kv", "Macao"),
        ("kw", "Macao"),
        ("ky", "Макао"),
        ("lo", "Macao"),
        ("lt", "Makao"),
        ("lv", "Makao"),
        ("mi", "Macao"),
        ("mk", "Макао"),
        ("ml", "മക\u{d3e}വോ"),
        ("mn", "Макао"),
        ("mr", "म\u{945}काव\u{94d}ह"),
        ("ms", "Macao"),
        ("mt", "Macao"),
        ("my", "Macao"),
        ("na", "Macao"),
        ("nb", "Macao"),
        ("ne", "मकाओ"),
        ("nl", "Macau"),
        ("nn", "Macao"),
        ("nv", "Macao"),
        ("oc", "Macau"),
        ("or", "ମ\u{b3e}କ\u{b3e}ଓ"),
        ("pa", "ਮ\u{a48}ਕ\u{a4b}"),
        ("pi", "Macao"),
        ("pl", "Makau"),
        ("ps", "Macao"),
        ("pt", "Macau"),
        ("pt_BR", "Macau"),
        ("ro", "Macao"),
        ("ru", "Макао"),
        ("rw", "Makawo"),
        ("sc", "Macao"),
        ("sd", "Macao"),
        ("si", "මැක\u{dcf}වෝ"),
        ("sk", "Macao"),
        ("sl", "Makao"),
        ("so", "Macao"),
        ("sq", "Makao"),
        ("sr", "Макао"),
        ("sv", "Macao"),
        ("sw", "Macao"),
        ("ta", "மக\u{bbe}வோ"),
        ("te", "మక\u{c4b}"),
        ("tg", "Макао"),
        ("th", "มาเก\u{e4a}า"),
        ("ti", "Macao"),
        ("tk", "Makao"),
        ("tl", "Makaw"),
        ("tr", "Makao"),
        ("tt", "Макао"),
        ("ug", "ئاۋمېن"),
        ("uk", "Макао"),
        ("ur", "Macao"),
        ("uz", "Macao"),
        ("ve", "Macao"),
        ("vi", "Ma-cao"),
        ("wa", "Macao"),
        ("wo", "Makaaw"),
        ("xh", "Macao"),
        ("yo", "Macao"),
        ("zh_CN", "澳门"),
        ("zh_HK", "澳門"),
        ("zh_TW", "澳門"),
        ("zu", "Macao"),
    ];
    #[cfg(all(feature = "mo", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 22.198745;
        pub const LONGITUDE: f64 = 113.543873;
        pub const MAX_LATITUDE: f64 = 22.2170639;
        pub const MAX_LONGITUDE: f64 = 113.6127001;
        pub const MIN_LATITUDE: f64 = 22.1066001;
        pub const MIN_LONGITUDE: f64 = 113.5276053;
        pub const NORTHEAST_LATITUDE: f64 = 22.2170639;
        pub const NORTHEAST_LONGITUDE: f64 = 113.6127001;
        pub const SOUTHWEST_LATITUDE: f64 = 22.1066001;
        pub const SOUTHWEST_LONGITUDE: f64 = 113.5276053;
    }
}
#[cfg(all(feature = "mo", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 22.198745,
            longitude: 113.543873,
            max_latitude: 22.2170639,
            max_longitude: 113.6127001,
            min_latitude: 22.1066001,
            min_longitude: 113.5276053,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 22.2170639,
                    longitude: 113.6127001,
                },
                southwest: CountryGeoBound {
                    latitude: 22.1066001,
                    longitude: 113.5276053,
                },
            },
        }
    }
}

#[cfg(all(feature = "mo", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "mo")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MO,
        alpha3: Alpha3::MAC,
        address_format: None,
        continent: Continent::Asia,
        country_code: 853,
        currency_code: "MOP",
        gec: Some(GEC::MC),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "The Macao Special Administrative Region of China",
        iso_short_name: "Macao",
        official_language_list: ["pt", "zh"].to_vec(),
        spoken_language_list: ["pt", "zh"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Chinese"),
        number: "446",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "MO",
        unofficial_name_list: ["Macao", "Macau", "マカオ"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Macao"),
            ("af", "Macao"),
            ("ak", "Macao"),
            ("am", "ሢጢፅ"),
            ("an", "Macao"),
            ("ar", "مك\u{651}او"),
            ("as", "মেক\u{9be}ও"),
            ("ay", "Macao"),
            ("az", "Makao"),
            ("ba", "Macao"),
            ("be", "Макаа"),
            ("bg", "Макао"),
            ("bi", "Macao"),
            ("bn", "ম\u{9cd}য\u{9be}ক\u{9be}ও"),
            ("bn_IN", "ম\u{9cd}য\u{9be}ক\u{9be}ও"),
            ("br", "Macao"),
            ("bs", "Makau"),
            ("ca", "Macau"),
            ("ce", "Macao"),
            ("ch", "Macao"),
            ("cs", "Macao"),
            ("cv", "Macao"),
            ("cy", "Macao"),
            ("da", "Macao"),
            ("de", "Macao"),
            ("dv", "Macao"),
            ("dz", "མ་ཀའ\u{f7c}།"),
            ("ee", "Macao"),
            ("el", "Μακάο"),
            ("en", "Macao"),
            ("eo", "Makao"),
            ("es", "Macao"),
            ("et", "Aomen"),
            ("eu", "Macau"),
            ("fa", "ماکائو"),
            ("ff", "Macao"),
            ("fi", "Macao"),
            ("fo", "Macao"),
            ("fr", "Macau"),
            ("fy", "Macao"),
            ("ga", "Macao"),
            ("gl", "Macau"),
            ("gn", "Macao"),
            ("gu", "મકાઉ"),
            ("gv", "Macao"),
            ("ha", "Macao"),
            ("he", "מאקאו"),
            ("hi", "मकाउ"),
            ("hr", "Makao"),
            ("ht", "Macao"),
            ("hu", "Makaó"),
            ("hy", "Աոմին"),
            ("ia", "Macao"),
            ("id", "Macao"),
            ("io", "Macao"),
            ("is", "Makaó"),
            ("it", "Macao"),
            ("iu", "Macao"),
            ("ja", "マカオ"),
            ("ka", "მაკაო"),
            ("ki", "Macao"),
            ("kk", "Макао"),
            ("kl", "Macao"),
            ("km", "ម\u{17c9}ាកាវ"),
            ("kn", "ಮಕಾವು"),
            ("ko", "마카오"),
            ("ku", "Makao"),
            ("kv", "Macao"),
            ("kw", "Macao"),
            ("ky", "Макао"),
            ("lo", "Macao"),
            ("lt", "Makao"),
            ("lv", "Makao"),
            ("mi", "Macao"),
            ("mk", "Макао"),
            ("ml", "മക\u{d3e}വോ"),
            ("mn", "Макао"),
            ("mr", "म\u{945}काव\u{94d}ह"),
            ("ms", "Macao"),
            ("mt", "Macao"),
            ("my", "Macao"),
            ("na", "Macao"),
            ("nb", "Macao"),
            ("ne", "मकाओ"),
            ("nl", "Macau"),
            ("nn", "Macao"),
            ("nv", "Macao"),
            ("oc", "Macau"),
            ("or", "ମ\u{b3e}କ\u{b3e}ଓ"),
            ("pa", "ਮ\u{a48}ਕ\u{a4b}"),
            ("pi", "Macao"),
            ("pl", "Makau"),
            ("ps", "Macao"),
            ("pt", "Macau"),
            ("pt_BR", "Macau"),
            ("ro", "Macao"),
            ("ru", "Макао"),
            ("rw", "Makawo"),
            ("sc", "Macao"),
            ("sd", "Macao"),
            ("si", "මැක\u{dcf}වෝ"),
            ("sk", "Macao"),
            ("sl", "Makao"),
            ("so", "Macao"),
            ("sq", "Makao"),
            ("sr", "Макао"),
            ("sv", "Macao"),
            ("sw", "Macao"),
            ("ta", "மக\u{bbe}வோ"),
            ("te", "మక\u{c4b}"),
            ("tg", "Макао"),
            ("th", "มาเก\u{e4a}า"),
            ("ti", "Macao"),
            ("tk", "Makao"),
            ("tl", "Makaw"),
            ("tr", "Makao"),
            ("tt", "Макао"),
            ("ug", "ئاۋمېن"),
            ("uk", "Макао"),
            ("ur", "Macao"),
            ("uz", "Macao"),
            ("ve", "Macao"),
            ("vi", "Ma-cao"),
            ("wa", "Macao"),
            ("wo", "Makaaw"),
            ("xh", "Macao"),
            ("yo", "Macao"),
            ("zh_CN", "澳门"),
            ("zh_HK", "澳門"),
            ("zh_TW", "澳門"),
            ("zu", "Macao"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
