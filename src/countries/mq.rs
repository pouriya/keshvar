// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Martinique

#[cfg(all(feature = "mq", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MQ;
    pub const ALPHA3: Alpha3 = Alpha3::MTQ;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 596;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::MB);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Martinique";
    pub const ISO_LONG_NAME: &str = "Martinique";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("French");
    pub const NUMBER: &str = "474";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("9[78]2\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "MQ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Martinique", "Martinica", "マルティニーク"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Martinique"),
        ("af", "Martinique"),
        ("ak", "Martinique"),
        ("am", "Martinique"),
        ("an", "Martinique"),
        ("ar", "مارتينيك"),
        ("as", "ম\u{9be}ৰ\u{9cd}টিনিক"),
        ("ay", "Martinique"),
        ("az", "Martiniq"),
        ("ba", "Martinique"),
        ("be", "Марцініка"),
        ("bg", "Мартиника"),
        ("bi", "Martinique"),
        ("bn", "ম\u{9be}র\u{9cd}টিনিক"),
        ("bn_IN", "ম\u{9be}র\u{9cd}টিনিক"),
        ("br", "Martinik"),
        ("bs", "Martinik"),
        ("ca", "Martinica"),
        ("ce", "Мартиника"),
        ("ch", "Martinique"),
        ("cs", "Martinik"),
        ("cv", "Мартиника"),
        ("cy", "Martinique"),
        ("da", "Martinique"),
        ("de", "Martinique"),
        ("dv", "މ\u{7a7}ޓ\u{7a8}ނ\u{7a9}ކ\u{7ab}"),
        ("dz", "མར་ཊ\u{f72}་ན\u{f72}ཀ།"),
        ("ee", "Martinique"),
        ("el", "Μαρτινίκα"),
        ("en", "Martinique"),
        ("eo", "Martiniko"),
        ("es", "Martinica"),
        ("et", "Martinique"),
        ("eu", "Martinika"),
        ("fa", "مارتینیک"),
        ("ff", "Martinique"),
        ("fi", "Martinique"),
        ("fo", "Martinique"),
        ("fr", "Martinique"),
        ("fy", "Martinique"),
        ("ga", "Martainíc"),
        ("gl", "Martinica"),
        ("gn", "Martinique"),
        ("gu", "માર\u{acd}ટિનિક"),
        ("gv", "Martinique"),
        ("ha", "Martinique"),
        ("he", "מרטיניק"),
        ("hi", "मार\u{94d}टीनिक"),
        ("hr", "Martinik"),
        ("ht", "Matinik"),
        ("hu", "Martinique"),
        ("hy", "Մարտինիկա"),
        ("ia", "Martinica"),
        ("id", "Martinik"),
        ("io", "Martinik"),
        ("is", "Martiník"),
        ("it", "Martinica"),
        ("iu", "Martinique"),
        ("ja", "マルティニーク"),
        ("ka", "მარტინიკი"),
        ("ki", "Martinique"),
        ("kk", "Мартиника"),
        ("kl", "Martinique"),
        ("km", "ម\u{17c9}ារទ\u{17b8}ន\u{17b8}គ"),
        ("kn", "Martinique"),
        ("ko", "마르티니크"),
        ("ku", "Martinîk"),
        ("kv", "Martinique"),
        ("kw", "Martinik"),
        ("ky", "Мартиника"),
        ("lo", "Martinique"),
        ("lt", "Martinika"),
        ("lv", "Martinika"),
        ("mi", "Martinique"),
        ("mk", "Мартиник"),
        ("ml", "മ\u{d3e}ര\u{d4d}\u{200d}ട\u{d4d}ടിനിക\u{d4d}ക\u{d4d}"),
        ("mn", "Martinique"),
        ("mr", "मार\u{94d}टिनिक"),
        ("ms", "Martinique"),
        ("mt", "Martinik"),
        ("my", "Martinique"),
        ("na", "Martinique"),
        ("nb", "Martinique"),
        ("ne", "मार\u{94d}टिनिक"),
        ("nl", "Martinique"),
        ("nn", "Martinique"),
        ("nv", "Martinique"),
        ("oc", "Martinica"),
        ("or", "ମ\u{b3e}ର\u{b4d}ଟ\u{b3f}ନୀକ"),
        ("pa", "ਮਾਰਟੀਨੀਕਿਊ"),
        ("pi", "Martinique"),
        ("pl", "Martynika"),
        ("ps", "Martinique"),
        ("pt", "Martinica"),
        ("pt_BR", "Martinica"),
        ("ro", "Martinica"),
        ("ru", "Мартиника"),
        ("rw", "Maritike"),
        ("sc", "Martinica"),
        ("sd", "Martinique"),
        ("si", "ම\u{dcf}ට\u{dd2}න\u{dd2}ක\u{dd2}"),
        ("sk", "Martinik"),
        ("sl", "Martinik"),
        ("so", "Martinique"),
        ("sq", "Martinikë"),
        ("sr", "Мартиник"),
        ("sv", "Martinique"),
        ("sw", "Martinique"),
        ("ta", "ம\u{bbe}ர\u{bcd}டின\u{bc0}க\u{bcd}கு"),
        ("te", "మ\u{c3e}ర\u{c4d}ట\u{c3f}న\u{c3f}క\u{c4d}"),
        ("tg", "Мартиника"),
        ("th", "มาร\u{e4c}ต\u{e34}น\u{e35}ก"),
        ("ti", "Martinique"),
        ("tk", "Martinika"),
        ("tl", "Martiniko"),
        ("tr", "Martinique"),
        ("tt", "Мартиник"),
        ("ug", "مارتىنىكا"),
        ("uk", "Мартиніка"),
        ("ur", "مارٹینیک"),
        ("uz", "Martinique"),
        ("ve", "Martinique"),
        ("vi", "Ma-thi-ni-khợ"),
        ("wa", "Martinike"),
        ("wo", "Martinik"),
        ("xh", "Martinique"),
        ("yo", "Mártíníkì"),
        ("zh_CN", "马提尼克"),
        ("zh_HK", "法屬馬丁尼克"),
        ("zh_TW", "馬丁尼克"),
        ("zu", "Martinique"),
    ];
    #[cfg(all(feature = "mq", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 14.641528;
        pub const LONGITUDE: f64 = -61.024174;
        pub const MAX_LATITUDE: f64 = 14.8973451;
        pub const MAX_LONGITUDE: f64 = -60.7856368;
        pub const MIN_LATITUDE: f64 = 14.370834;
        pub const MIN_LONGITUDE: f64 = -61.24191279999999;
        pub const NORTHEAST_LATITUDE: f64 = 14.8973451;
        pub const NORTHEAST_LONGITUDE: f64 = -60.7856368;
        pub const SOUTHWEST_LATITUDE: f64 = 14.370834;
        pub const SOUTHWEST_LONGITUDE: f64 = -61.24191279999999;
    }
}
#[cfg(all(feature = "mq", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 14.641528,
            longitude: -61.024174,
            max_latitude: 14.8973451,
            max_longitude: -60.7856368,
            min_latitude: 14.370834,
            min_longitude: -61.24191279999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 14.8973451,
                    longitude: -60.7856368,
                },
                southwest: CountryGeoBound {
                    latitude: 14.370834,
                    longitude: -61.24191279999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "mq", feature = "subdivisions"))]
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
#[cfg(feature = "mq")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MQ,
        alpha3: Alpha3::MTQ,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 596,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::MB),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "Martinique",
        iso_short_name: "Martinique",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "None",
        nationality: Some("French"),
        number: "474",
        postal_code: true,
        postal_code_format: Some("9[78]2\\d{2}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "MQ",
        unofficial_name_list: ["Martinique", "Martinica", "マルティニーク"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Martinique"),
            ("af", "Martinique"),
            ("ak", "Martinique"),
            ("am", "Martinique"),
            ("an", "Martinique"),
            ("ar", "مارتينيك"),
            ("as", "ম\u{9be}ৰ\u{9cd}টিনিক"),
            ("ay", "Martinique"),
            ("az", "Martiniq"),
            ("ba", "Martinique"),
            ("be", "Марцініка"),
            ("bg", "Мартиника"),
            ("bi", "Martinique"),
            ("bn", "ম\u{9be}র\u{9cd}টিনিক"),
            ("bn_IN", "ম\u{9be}র\u{9cd}টিনিক"),
            ("br", "Martinik"),
            ("bs", "Martinik"),
            ("ca", "Martinica"),
            ("ce", "Мартиника"),
            ("ch", "Martinique"),
            ("cs", "Martinik"),
            ("cv", "Мартиника"),
            ("cy", "Martinique"),
            ("da", "Martinique"),
            ("de", "Martinique"),
            ("dv", "މ\u{7a7}ޓ\u{7a8}ނ\u{7a9}ކ\u{7ab}"),
            ("dz", "མར་ཊ\u{f72}་ན\u{f72}ཀ།"),
            ("ee", "Martinique"),
            ("el", "Μαρτινίκα"),
            ("en", "Martinique"),
            ("eo", "Martiniko"),
            ("es", "Martinica"),
            ("et", "Martinique"),
            ("eu", "Martinika"),
            ("fa", "مارتینیک"),
            ("ff", "Martinique"),
            ("fi", "Martinique"),
            ("fo", "Martinique"),
            ("fr", "Martinique"),
            ("fy", "Martinique"),
            ("ga", "Martainíc"),
            ("gl", "Martinica"),
            ("gn", "Martinique"),
            ("gu", "માર\u{acd}ટિનિક"),
            ("gv", "Martinique"),
            ("ha", "Martinique"),
            ("he", "מרטיניק"),
            ("hi", "मार\u{94d}टीनिक"),
            ("hr", "Martinik"),
            ("ht", "Matinik"),
            ("hu", "Martinique"),
            ("hy", "Մարտինիկա"),
            ("ia", "Martinica"),
            ("id", "Martinik"),
            ("io", "Martinik"),
            ("is", "Martiník"),
            ("it", "Martinica"),
            ("iu", "Martinique"),
            ("ja", "マルティニーク"),
            ("ka", "მარტინიკი"),
            ("ki", "Martinique"),
            ("kk", "Мартиника"),
            ("kl", "Martinique"),
            ("km", "ម\u{17c9}ារទ\u{17b8}ន\u{17b8}គ"),
            ("kn", "Martinique"),
            ("ko", "마르티니크"),
            ("ku", "Martinîk"),
            ("kv", "Martinique"),
            ("kw", "Martinik"),
            ("ky", "Мартиника"),
            ("lo", "Martinique"),
            ("lt", "Martinika"),
            ("lv", "Martinika"),
            ("mi", "Martinique"),
            ("mk", "Мартиник"),
            ("ml", "മ\u{d3e}ര\u{d4d}\u{200d}ട\u{d4d}ടിനിക\u{d4d}ക\u{d4d}"),
            ("mn", "Martinique"),
            ("mr", "मार\u{94d}टिनिक"),
            ("ms", "Martinique"),
            ("mt", "Martinik"),
            ("my", "Martinique"),
            ("na", "Martinique"),
            ("nb", "Martinique"),
            ("ne", "मार\u{94d}टिनिक"),
            ("nl", "Martinique"),
            ("nn", "Martinique"),
            ("nv", "Martinique"),
            ("oc", "Martinica"),
            ("or", "ମ\u{b3e}ର\u{b4d}ଟ\u{b3f}ନୀକ"),
            ("pa", "ਮਾਰਟੀਨੀਕਿਊ"),
            ("pi", "Martinique"),
            ("pl", "Martynika"),
            ("ps", "Martinique"),
            ("pt", "Martinica"),
            ("pt_BR", "Martinica"),
            ("ro", "Martinica"),
            ("ru", "Мартиника"),
            ("rw", "Maritike"),
            ("sc", "Martinica"),
            ("sd", "Martinique"),
            ("si", "ම\u{dcf}ට\u{dd2}න\u{dd2}ක\u{dd2}"),
            ("sk", "Martinik"),
            ("sl", "Martinik"),
            ("so", "Martinique"),
            ("sq", "Martinikë"),
            ("sr", "Мартиник"),
            ("sv", "Martinique"),
            ("sw", "Martinique"),
            ("ta", "ம\u{bbe}ர\u{bcd}டின\u{bc0}க\u{bcd}கு"),
            ("te", "మ\u{c3e}ర\u{c4d}ట\u{c3f}న\u{c3f}క\u{c4d}"),
            ("tg", "Мартиника"),
            ("th", "มาร\u{e4c}ต\u{e34}น\u{e35}ก"),
            ("ti", "Martinique"),
            ("tk", "Martinika"),
            ("tl", "Martiniko"),
            ("tr", "Martinique"),
            ("tt", "Мартиник"),
            ("ug", "مارتىنىكا"),
            ("uk", "Мартиніка"),
            ("ur", "مارٹینیک"),
            ("uz", "Martinique"),
            ("ve", "Martinique"),
            ("vi", "Ma-thi-ni-khợ"),
            ("wa", "Martinike"),
            ("wo", "Martinik"),
            ("xh", "Martinique"),
            ("yo", "Mártíníkì"),
            ("zh_CN", "马提尼克"),
            ("zh_HK", "法屬馬丁尼克"),
            ("zh_TW", "馬丁尼克"),
            ("zu", "Martinique"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
