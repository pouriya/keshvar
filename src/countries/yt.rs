// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Department of Mayotte

#[cfg(all(feature = "yt", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::YT;
    pub const ALPHA3: Alpha3 = Alpha3::MYT;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 262;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::MF);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Mayotte";
    pub const ISO_LONG_NAME: &str = "The Department of Mayotte";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("French");
    pub const NUMBER: &str = "175";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("976\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "YT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Mayotte", "マヨット"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Mayotte"),
        ("af", "Mayotte"),
        ("ak", "Mayotte"),
        ("am", "Mayotte"),
        ("an", "Mayotte"),
        ("ar", "مايوت"),
        ("as", "মেওতে"),
        ("ay", "Mayotte"),
        ("az", "Mayot"),
        ("ba", "Mayotte"),
        ("be", "Маёта"),
        ("bg", "Майот"),
        ("bi", "Mayotte"),
        ("bn", "মেওতে"),
        ("bn_IN", "মেওতে"),
        ("br", "Mayotte"),
        ("bs", "Mayotte"),
        ("ca", "Mayotte"),
        ("ce", "Майотта"),
        ("ch", "Mayotte"),
        ("cs", "Mayotte"),
        ("cv", "Майотта"),
        ("cy", "Mayotte"),
        ("da", "Mayotte"),
        ("de", "Mayotte"),
        ("dv", "Mayotte"),
        ("dz", "མ་ཡ\u{f7c}་ཊ\u{f72}།"),
        ("ee", "Mayotte"),
        ("el", "Μαγιότ"),
        ("en", "Mayotte"),
        ("eo", "Majoto"),
        ("es", "Mayotte"),
        ("et", "Mayotte"),
        ("eu", "Mayotte"),
        ("fa", "مایوت"),
        ("ff", "Mayotte"),
        ("fi", "Mayotte"),
        ("fo", "Mayotte"),
        ("fr", "Mayotte"),
        ("fy", "Majot"),
        ("ga", "Mayotte"),
        ("gl", "Maiote"),
        ("gn", "Mayotte"),
        ("gu", "માયોટ\u{ac7}"),
        ("gv", "Mayotte"),
        ("ha", "Mayotte"),
        ("he", "מיוט"),
        ("hi", "म\u{947}योट"),
        ("hr", "Mayotte"),
        ("ht", "Mayotte"),
        ("hu", "Mayotte"),
        ("hy", "Մայոտ"),
        ("ia", "Mayotte"),
        ("id", "Mayotte"),
        ("io", "Mayotte"),
        ("is", "Mayott"),
        ("it", "Mayotte"),
        ("iu", "Mayotte"),
        ("ja", "マヨット"),
        ("ka", "მეიოტი"),
        ("ki", "Mayotte"),
        ("kk", "Мариотт"),
        ("kl", "Mayotte"),
        ("km", "ម\u{17c9}ាយ\u{17bb}ត"),
        ("kn", "Mayotte"),
        ("ko", "마요트"),
        ("ku", "Mayote"),
        ("kv", "Mayotte"),
        ("kw", "Mayotte"),
        ("ky", "Майот"),
        ("lo", "Mayotte"),
        ("lt", "Majotas"),
        ("lv", "Majota"),
        ("mi", "Mayotte"),
        ("mk", "Мајоте"),
        ("ml", "മയോട\u{d4d}ടെ"),
        ("mn", "Майотте"),
        ("mr", "मायोट"),
        ("ms", "Mayotte"),
        ("mt", "Majotte"),
        ("my", "Mayotte"),
        ("na", "Mayotte"),
        ("nb", "Mayotte"),
        ("ne", "मायोट\u{94d}ट\u{947}"),
        ("nl", "Mayotte"),
        ("nn", "Mayotte"),
        ("nv", "Mayotte"),
        ("oc", "Maiòta"),
        ("or", "ମେୟୋଟ"),
        ("pa", "ਮਾਈਟੀ"),
        ("pi", "Mayotte"),
        ("pl", "Majotta"),
        ("ps", "Mayotte"),
        ("pt", "Mayotte"),
        ("pt_BR", "Maiote"),
        ("ro", "Mayotte"),
        ("ru", "Майот"),
        ("rw", "Mayoti"),
        ("sc", "Mayotte"),
        ("sd", "Mayotte"),
        ("si", "මයොට\u{dca}ටේ"),
        ("sk", "Mayotte"),
        ("sl", "Mayotte"),
        ("so", "Mayotte"),
        ("sq", "Majote"),
        ("sr", "Мајот"),
        ("sv", "Mayotte"),
        ("sw", "Mayotte"),
        ("ta", "மயோதி"),
        ("te", "మ\u{c3e}య\u{c4b}ట\u{c46}"),
        ("tg", "Майотта"),
        ("th", "มายอต"),
        ("ti", "Mayotte"),
        ("tk", "Maýotta"),
        ("tl", "Mayotte"),
        ("tr", "Mayotte"),
        ("tt", "Маётте"),
        ("ug", "مايوتتې"),
        ("uk", "Майотта"),
        ("ur", "مایوٹ"),
        ("uz", "Mayotta"),
        ("ve", "Mayotte"),
        ("vi", "May-o-thợ"),
        ("wa", "Mayote"),
        ("wo", "Mayoot"),
        ("xh", "Mayotte"),
        ("yo", "Mayotte"),
        ("zh_CN", "马约特"),
        ("zh_HK", "美亞特"),
        ("zh_TW", "馬約特"),
        ("zu", "IMayotte"),
    ];
    #[cfg(all(feature = "yt", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -12.8275;
        pub const LONGITUDE: f64 = 45.166244;
        pub const MAX_LATITUDE: f64 = -12.5772665;
        pub const MAX_LONGITUDE: f64 = 45.32014849999999;
        pub const MIN_LATITUDE: f64 = -13.0358332;
        pub const MIN_LONGITUDE: f64 = 44.9914169;
        pub const NORTHEAST_LATITUDE: f64 = -12.5772665;
        pub const NORTHEAST_LONGITUDE: f64 = 45.32014849999999;
        pub const SOUTHWEST_LATITUDE: f64 = -13.0358332;
        pub const SOUTHWEST_LONGITUDE: f64 = 44.9914169;
    }
}
#[cfg(all(feature = "yt", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -12.8275,
            longitude: 45.166244,
            max_latitude: -12.5772665,
            max_longitude: 45.32014849999999,
            min_latitude: -13.0358332,
            min_longitude: 44.9914169,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -12.5772665,
                    longitude: 45.32014849999999,
                },
                southwest: CountryGeoBound {
                    latitude: -13.0358332,
                    longitude: 44.9914169,
                },
            },
        }
    }
}

#[cfg(all(feature = "yt", feature = "subdivisions"))]
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
#[cfg(feature = "yt")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::YT,
        alpha3: Alpha3::MYT,
        address_format: None,
        continent: Continent::Africa,
        country_code: 262,
        currency_code: "EUR",
        gec: Some(GEC::MF),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "The Department of Mayotte",
        iso_short_name: "Mayotte",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("French"),
        number: "175",
        postal_code: true,
        postal_code_format: Some("976\\d{2}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "YT",
        unofficial_name_list: ["Mayotte", "マヨット"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Mayotte"),
            ("af", "Mayotte"),
            ("ak", "Mayotte"),
            ("am", "Mayotte"),
            ("an", "Mayotte"),
            ("ar", "مايوت"),
            ("as", "মেওতে"),
            ("ay", "Mayotte"),
            ("az", "Mayot"),
            ("ba", "Mayotte"),
            ("be", "Маёта"),
            ("bg", "Майот"),
            ("bi", "Mayotte"),
            ("bn", "মেওতে"),
            ("bn_IN", "মেওতে"),
            ("br", "Mayotte"),
            ("bs", "Mayotte"),
            ("ca", "Mayotte"),
            ("ce", "Майотта"),
            ("ch", "Mayotte"),
            ("cs", "Mayotte"),
            ("cv", "Майотта"),
            ("cy", "Mayotte"),
            ("da", "Mayotte"),
            ("de", "Mayotte"),
            ("dv", "Mayotte"),
            ("dz", "མ་ཡ\u{f7c}་ཊ\u{f72}།"),
            ("ee", "Mayotte"),
            ("el", "Μαγιότ"),
            ("en", "Mayotte"),
            ("eo", "Majoto"),
            ("es", "Mayotte"),
            ("et", "Mayotte"),
            ("eu", "Mayotte"),
            ("fa", "مایوت"),
            ("ff", "Mayotte"),
            ("fi", "Mayotte"),
            ("fo", "Mayotte"),
            ("fr", "Mayotte"),
            ("fy", "Majot"),
            ("ga", "Mayotte"),
            ("gl", "Maiote"),
            ("gn", "Mayotte"),
            ("gu", "માયોટ\u{ac7}"),
            ("gv", "Mayotte"),
            ("ha", "Mayotte"),
            ("he", "מיוט"),
            ("hi", "म\u{947}योट"),
            ("hr", "Mayotte"),
            ("ht", "Mayotte"),
            ("hu", "Mayotte"),
            ("hy", "Մայոտ"),
            ("ia", "Mayotte"),
            ("id", "Mayotte"),
            ("io", "Mayotte"),
            ("is", "Mayott"),
            ("it", "Mayotte"),
            ("iu", "Mayotte"),
            ("ja", "マヨット"),
            ("ka", "მეიოტი"),
            ("ki", "Mayotte"),
            ("kk", "Мариотт"),
            ("kl", "Mayotte"),
            ("km", "ម\u{17c9}ាយ\u{17bb}ត"),
            ("kn", "Mayotte"),
            ("ko", "마요트"),
            ("ku", "Mayote"),
            ("kv", "Mayotte"),
            ("kw", "Mayotte"),
            ("ky", "Майот"),
            ("lo", "Mayotte"),
            ("lt", "Majotas"),
            ("lv", "Majota"),
            ("mi", "Mayotte"),
            ("mk", "Мајоте"),
            ("ml", "മയോട\u{d4d}ടെ"),
            ("mn", "Майотте"),
            ("mr", "मायोट"),
            ("ms", "Mayotte"),
            ("mt", "Majotte"),
            ("my", "Mayotte"),
            ("na", "Mayotte"),
            ("nb", "Mayotte"),
            ("ne", "मायोट\u{94d}ट\u{947}"),
            ("nl", "Mayotte"),
            ("nn", "Mayotte"),
            ("nv", "Mayotte"),
            ("oc", "Maiòta"),
            ("or", "ମେୟୋଟ"),
            ("pa", "ਮਾਈਟੀ"),
            ("pi", "Mayotte"),
            ("pl", "Majotta"),
            ("ps", "Mayotte"),
            ("pt", "Mayotte"),
            ("pt_BR", "Maiote"),
            ("ro", "Mayotte"),
            ("ru", "Майот"),
            ("rw", "Mayoti"),
            ("sc", "Mayotte"),
            ("sd", "Mayotte"),
            ("si", "මයොට\u{dca}ටේ"),
            ("sk", "Mayotte"),
            ("sl", "Mayotte"),
            ("so", "Mayotte"),
            ("sq", "Majote"),
            ("sr", "Мајот"),
            ("sv", "Mayotte"),
            ("sw", "Mayotte"),
            ("ta", "மயோதி"),
            ("te", "మ\u{c3e}య\u{c4b}ట\u{c46}"),
            ("tg", "Майотта"),
            ("th", "มายอต"),
            ("ti", "Mayotte"),
            ("tk", "Maýotta"),
            ("tl", "Mayotte"),
            ("tr", "Mayotte"),
            ("tt", "Маётте"),
            ("ug", "مايوتتې"),
            ("uk", "Майотта"),
            ("ur", "مایوٹ"),
            ("uz", "Mayotta"),
            ("ve", "Mayotte"),
            ("vi", "May-o-thợ"),
            ("wa", "Mayote"),
            ("wo", "Mayoot"),
            ("xh", "Mayotte"),
            ("yo", "Mayotte"),
            ("zh_CN", "马约特"),
            ("zh_HK", "美亞特"),
            ("zh_TW", "馬約特"),
            ("zu", "IMayotte"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
