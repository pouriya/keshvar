// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Territory of Guam

#[cfg(all(feature = "gu", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GU;
    pub const ALPHA3: Alpha3 = Alpha3::GUM;
    pub const CONTINENT: Continent = Continent::Australia;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::USD;
    pub const GEC: Option<GEC> = Some(GEC::GQ);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::GUM);
    pub const ISO_SHORT_NAME: &str = "Guam";
    pub const ISO_LONG_NAME: &str = "The Territory of Guam";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ch", "en", "es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ch", "en", "es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Guamanian");
    pub const NUMBER: &str = "316";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(969(?:[12]\\d|3[12]))(?:[ \\-](\\d{4}))?");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Micronesia);
    pub const UN_LOCODE: &str = "GU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Guam", "グアム"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Guam"),
        ("af", "Guam"),
        ("ak", "Guam"),
        ("am", "Guam"),
        ("an", "Guam"),
        ("ar", "جوام"),
        ("as", "গ\u{9c1}ৱ\u{9be}ম"),
        ("ay", "Guam"),
        ("az", "Quam"),
        ("ba", "Guam"),
        ("be", "Гуам"),
        ("bg", "Гуам"),
        ("bi", "Guam"),
        ("bn", "গ\u{9c1}য়\u{9be}ম"),
        ("bn_IN", "গ\u{9c1}য়\u{9be}ম"),
        ("br", "Guam"),
        ("bs", "Guam"),
        ("ca", "Guam"),
        ("ce", "Гуам"),
        ("ch", "Guåhån"),
        ("cs", "Guam"),
        ("cv", "Гуам"),
        ("cy", "Guam"),
        ("da", "Guam"),
        ("de", "Guam"),
        ("dv", "ގ\u{7aa}އ\u{7a7}މ\u{7aa}"),
        ("dz", "ག\u{f74}འམ།"),
        ("ee", "Guam"),
        ("el", "Γκουάμ"),
        ("en", "Guam"),
        ("eo", "Gvamo"),
        ("es", "Guam"),
        ("et", "Guam"),
        ("eu", "Guam"),
        ("fa", "گوام"),
        ("ff", "Guam"),
        ("fi", "Guam"),
        ("fo", "Guam"),
        ("fr", "Guam"),
        ("fy", "Gûam"),
        ("ga", "Guam"),
        ("gl", "Guam"),
        ("gn", "Guam"),
        ("gu", "ગ\u{ac1}આમ"),
        ("gv", "Guam"),
        ("ha", "Guam"),
        ("he", "גואם"),
        ("hi", "ग\u{941}आम"),
        ("hr", "Guam"),
        ("ht", "Guam"),
        ("hu", "Guam"),
        ("hy", "Գուամ"),
        ("ia", "Guam"),
        ("id", "Guam"),
        ("io", "Guam"),
        ("is", "Gvam"),
        ("it", "Guam"),
        ("iu", "Guam"),
        ("ja", "グアム"),
        ("ka", "გუამი"),
        ("ki", "Guam"),
        ("kk", "Гуам"),
        ("kl", "Guam"),
        ("km", "ហ\u{17d2}គាម"),
        ("kn", "ಗ\u{ccd}ವಾಮ\u{ccd}"),
        ("ko", "괌"),
        ("ku", "Guam"),
        ("kv", "Guam"),
        ("kw", "Guam"),
        ("ky", "Гуам"),
        ("lo", "Guam"),
        ("lt", "Guamas"),
        ("lv", "Guama"),
        ("mi", "Kuamu"),
        ("mk", "Гуам"),
        ("ml", "ഗ\u{d41}വ\u{d3e}ം"),
        ("mn", "Гуам"),
        ("mr", "ग\u{94d}वाम"),
        ("ms", "Guam"),
        ("mt", "Gwam"),
        ("my", "ဂ\u{1030}အမ\u{103a}ကျ\u{103d}န\u{103a}း"),
        ("na", "Guam"),
        ("nb", "Guam"),
        ("ne", "ग\u{94d}वाम"),
        ("nl", "Guam"),
        ("nn", "Guam"),
        ("nv", "Guam"),
        ("oc", "Guam"),
        ("or", "ଗ\u{b41}ଆମ"),
        ("pa", "ਗ\u{a41}ਆਮ"),
        ("pi", "Guam"),
        ("pl", "Guam"),
        ("ps", "Guam"),
        ("pt", "Guam"),
        ("pt_BR", "Guam"),
        ("ro", "Guam"),
        ("ru", "Гуам"),
        ("rw", "Gwami"),
        ("sc", "Guàm"),
        ("sd", "Guam"),
        ("si", "ග\u{dd4}ව\u{dcf}ම\u{dca}"),
        ("sk", "Guam"),
        ("sl", "Guam"),
        ("so", "Guam"),
        ("sq", "Guam"),
        ("sr", "Гиам"),
        ("sv", "Guam"),
        ("sw", "Guam"),
        ("ta", "குவ\u{bbe}ம\u{bcd}"),
        ("te", "గ\u{c4d}వ\u{c3e}మ\u{c4d}"),
        ("tg", "Гуам"),
        ("th", "กวม"),
        ("ti", "Guam"),
        ("tk", "Guam"),
        ("tl", "Gwam"),
        ("tr", "Guam"),
        ("tt", "Gуам"),
        ("ug", "گۇئام"),
        ("uk", "Гуам"),
        ("ur", "گوام"),
        ("uz", "Guam"),
        ("ve", "Guam"),
        ("vi", "Gu-ăm"),
        ("wa", "Gwam"),
        ("wo", "Guam"),
        ("xh", "Guam"),
        ("yo", "Guam"),
        ("zh_CN", "关岛"),
        ("zh_HK", "關島"),
        ("zh_TW", "關島"),
        ("zu", "Guam"),
    ];
    #[cfg(all(feature = "gu", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 13.444304;
        pub const LONGITUDE: f64 = 144.793731;
        pub const MAX_LATITUDE: f64 = 13.7994072;
        pub const MAX_LONGITUDE: f64 = 145.112915;
        pub const MIN_LATITUDE: f64 = 13.1022175;
        pub const MIN_LONGITUDE: f64 = 144.4647218;
        pub const NORTHEAST_LATITUDE: f64 = 13.7994072;
        pub const NORTHEAST_LONGITUDE: f64 = 145.112915;
        pub const SOUTHWEST_LATITUDE: f64 = 13.1022175;
        pub const SOUTHWEST_LONGITUDE: f64 = 144.4647218;
    }
}
#[cfg(all(feature = "gu", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 13.444304,
            longitude: 144.793731,
            max_latitude: 13.7994072,
            max_longitude: 145.112915,
            min_latitude: 13.1022175,
            min_longitude: 144.4647218,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 13.7994072,
                    longitude: 145.112915,
                },
                southwest: CountryGeoBound {
                    latitude: 13.1022175,
                    longitude: 144.4647218,
                },
            },
        }
    }
}

#[cfg(all(feature = "gu", feature = "subdivisions"))]
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "gu")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GU,
        alpha3: Alpha3::GUM,
        address_format: None,
        continent: Continent::Australia,
        country_code: 1,
        currency_code: CurrencyCode::USD,
        gec: Some(GEC::GQ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::GUM),
        iso_long_name: "The Territory of Guam",
        iso_short_name: "Guam",
        official_language_list: ["ch", "en", "es"].to_vec(),
        spoken_language_list: ["ch", "en", "es"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Guamanian"),
        number: "316",
        postal_code: true,
        postal_code_format: Some("(969(?:[12]\\d|3[12]))(?:[ \\-](\\d{4}))?"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Micronesia),
        un_locode: "GU",
        unofficial_name_list: ["Guam", "グアム"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Guam"),
            ("af", "Guam"),
            ("ak", "Guam"),
            ("am", "Guam"),
            ("an", "Guam"),
            ("ar", "جوام"),
            ("as", "গ\u{9c1}ৱ\u{9be}ম"),
            ("ay", "Guam"),
            ("az", "Quam"),
            ("ba", "Guam"),
            ("be", "Гуам"),
            ("bg", "Гуам"),
            ("bi", "Guam"),
            ("bn", "গ\u{9c1}য়\u{9be}ম"),
            ("bn_IN", "গ\u{9c1}য়\u{9be}ম"),
            ("br", "Guam"),
            ("bs", "Guam"),
            ("ca", "Guam"),
            ("ce", "Гуам"),
            ("ch", "Guåhån"),
            ("cs", "Guam"),
            ("cv", "Гуам"),
            ("cy", "Guam"),
            ("da", "Guam"),
            ("de", "Guam"),
            ("dv", "ގ\u{7aa}އ\u{7a7}މ\u{7aa}"),
            ("dz", "ག\u{f74}འམ།"),
            ("ee", "Guam"),
            ("el", "Γκουάμ"),
            ("en", "Guam"),
            ("eo", "Gvamo"),
            ("es", "Guam"),
            ("et", "Guam"),
            ("eu", "Guam"),
            ("fa", "گوام"),
            ("ff", "Guam"),
            ("fi", "Guam"),
            ("fo", "Guam"),
            ("fr", "Guam"),
            ("fy", "Gûam"),
            ("ga", "Guam"),
            ("gl", "Guam"),
            ("gn", "Guam"),
            ("gu", "ગ\u{ac1}આમ"),
            ("gv", "Guam"),
            ("ha", "Guam"),
            ("he", "גואם"),
            ("hi", "ग\u{941}आम"),
            ("hr", "Guam"),
            ("ht", "Guam"),
            ("hu", "Guam"),
            ("hy", "Գուամ"),
            ("ia", "Guam"),
            ("id", "Guam"),
            ("io", "Guam"),
            ("is", "Gvam"),
            ("it", "Guam"),
            ("iu", "Guam"),
            ("ja", "グアム"),
            ("ka", "გუამი"),
            ("ki", "Guam"),
            ("kk", "Гуам"),
            ("kl", "Guam"),
            ("km", "ហ\u{17d2}គាម"),
            ("kn", "ಗ\u{ccd}ವಾಮ\u{ccd}"),
            ("ko", "괌"),
            ("ku", "Guam"),
            ("kv", "Guam"),
            ("kw", "Guam"),
            ("ky", "Гуам"),
            ("lo", "Guam"),
            ("lt", "Guamas"),
            ("lv", "Guama"),
            ("mi", "Kuamu"),
            ("mk", "Гуам"),
            ("ml", "ഗ\u{d41}വ\u{d3e}ം"),
            ("mn", "Гуам"),
            ("mr", "ग\u{94d}वाम"),
            ("ms", "Guam"),
            ("mt", "Gwam"),
            ("my", "ဂ\u{1030}အမ\u{103a}ကျ\u{103d}န\u{103a}း"),
            ("na", "Guam"),
            ("nb", "Guam"),
            ("ne", "ग\u{94d}वाम"),
            ("nl", "Guam"),
            ("nn", "Guam"),
            ("nv", "Guam"),
            ("oc", "Guam"),
            ("or", "ଗ\u{b41}ଆମ"),
            ("pa", "ਗ\u{a41}ਆਮ"),
            ("pi", "Guam"),
            ("pl", "Guam"),
            ("ps", "Guam"),
            ("pt", "Guam"),
            ("pt_BR", "Guam"),
            ("ro", "Guam"),
            ("ru", "Гуам"),
            ("rw", "Gwami"),
            ("sc", "Guàm"),
            ("sd", "Guam"),
            ("si", "ග\u{dd4}ව\u{dcf}ම\u{dca}"),
            ("sk", "Guam"),
            ("sl", "Guam"),
            ("so", "Guam"),
            ("sq", "Guam"),
            ("sr", "Гиам"),
            ("sv", "Guam"),
            ("sw", "Guam"),
            ("ta", "குவ\u{bbe}ம\u{bcd}"),
            ("te", "గ\u{c4d}వ\u{c3e}మ\u{c4d}"),
            ("tg", "Гуам"),
            ("th", "กวม"),
            ("ti", "Guam"),
            ("tk", "Guam"),
            ("tl", "Gwam"),
            ("tr", "Guam"),
            ("tt", "Gуам"),
            ("ug", "گۇئام"),
            ("uk", "Гуам"),
            ("ur", "گوام"),
            ("uz", "Guam"),
            ("ve", "Guam"),
            ("vi", "Gu-ăm"),
            ("wa", "Gwam"),
            ("wo", "Guam"),
            ("xh", "Guam"),
            ("yo", "Guam"),
            ("zh_CN", "关岛"),
            ("zh_HK", "關島"),
            ("zh_TW", "關島"),
            ("zu", "Guam"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
