// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Hong Kong Special Administrative Region of China

#[cfg(all(feature = "hk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::HK;
    pub const ALPHA3: Alpha3 = Alpha3::HKG;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 852;
    pub const CURRENCY_CODE: &str = "HKD";
    pub const GEC: Option<GEC> = Some(GEC::HK);
    pub const INTERNATIONAL_PREFIX: &str = "001";
    pub const IOC: Option<&str> = Some("HKG");
    pub const ISO_SHORT_NAME: &str = "Hong Kong";
    pub const ISO_LONG_NAME: &str = "The Hong Kong Special Administrative Region of China";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "zh"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "zh"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Hong Kongese");
    pub const NUMBER: &str = "344";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "HK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Hong Kong", "香港", "Hongkong"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Hong Kong"),
        ("af", "Hongkong"),
        ("ak", "Hong Kong"),
        ("am", "ሆንጔ ጥንጔ"),
        ("an", "Hong Kong"),
        ("ar", "هونغ كونغ"),
        ("as", "হংকং"),
        ("ay", "Hong Kong"),
        ("az", "Honq Konq"),
        ("ba", "Hong Kong"),
        ("be", "Ганконг"),
        ("bg", "Хонг Конг"),
        ("bi", "Hong Kong"),
        ("bn", "হং কং"),
        ("bn_IN", "হং কং"),
        ("br", "Hong Kong"),
        ("bs", "Hong Kong"),
        ("ca", "Hong Kong"),
        ("ce", "Гонконг"),
        ("ch", "Hong Kong"),
        ("cs", "Hongkong"),
        ("cv", "Гонконг"),
        ("cy", "Hong Kong"),
        ("da", "Hongkong"),
        ("de", "Hongkong"),
        ("dv", "ހ\u{7ae}ނ\u{7b0}ކ\u{7ae}ނ\u{7b0}ގ\u{7aa}"),
        ("dz", "ཧ\u{f7c}ང་ཀ\u{f7c}ང་"),
        ("ee", "Hong Kong"),
        ("el", "Χονγκ Κονγκ"),
        ("en", "Hong Kong"),
        ("eo", "Honkongo"),
        ("es", "Hong Kong"),
        ("et", "Hongkong"),
        ("eu", "Hong Kong"),
        ("fa", "هنگ کنگ"),
        ("ff", "Hong Kong"),
        ("fi", "Hong Kong"),
        ("fo", "Hong Kong"),
        ("fr", "Hong Kong"),
        ("fy", "Hongkong"),
        ("ga", "Hong Cong"),
        ("gl", "Hong Kong"),
        ("gn", "Hong Kong"),
        ("gu", "હો\u{a82}ગ કો\u{a82}ગ"),
        ("gv", "Hong Kong"),
        ("ha", "Hong Kong"),
        ("he", "הונג קונג"),
        ("hi", "हा\u{902}गका\u{902}ग"),
        ("hr", "Hong Kong"),
        ("ht", "Hong Kong"),
        ("hu", "Hongkong"),
        ("hy", "Հոնգ Կոնգ"),
        ("ia", "Hong Kong"),
        ("id", "Hong Kong"),
        ("io", "Hong Kong"),
        ("is", "Hong Kong"),
        ("it", "Hong Kong"),
        ("iu", "Hong Kong"),
        ("ja", "香港"),
        ("ka", "ჰონკონგი"),
        ("ki", "Hong Kong"),
        ("kk", "Сянган (Гонконг)"),
        ("kl", "Hong Kong"),
        ("km", "ហ\u{17bb}ងក\u{17bb}ង"),
        ("kn", "ಹಾಂಗ\u{ccd} ಕಾಂಗ\u{ccd}"),
        ("ko", "홍콩"),
        ("ku", "Hong Kong"),
        ("kv", "Hong Kong"),
        ("kw", "Hong Kong"),
        ("ky", "Гонконг"),
        ("lo", "ຮ\u{ebb}ງກ\u{ebb}ງ"),
        ("lt", "Honkongas"),
        ("lv", "Honkonga"),
        ("mi", "Hongipua"),
        ("mk", "Хонк Конг"),
        ("ml", "ഹോങ\u{d4d} കോങ\u{d4d}"),
        ("mn", "Хонконг"),
        ("mr", "हॉ\u{902}गकॉ\u{902}ग"),
        ("ms", "Hong Kong"),
        ("mt", "Ħong Kong"),
        ("my", "ဟောင\u{103a}ကောင\u{103a}"),
        ("na", "Hong Kong"),
        ("nb", "Hongkong"),
        ("ne", "हङकङ"),
        ("nl", "Hongkong"),
        ("nn", "Hongkong"),
        ("nv", "Hong Kong"),
        ("oc", "Hong Kong"),
        ("or", "ହଙ\u{b4d}ଗ କଙ\u{b4d}ଗ"),
        ("pa", "ਹਾ\u{a02}ਗ ਕਾ\u{a02}ਗ"),
        ("pi", "Hong Kong"),
        ("pl", "Hongkong"),
        ("ps", "هانګ کانګ"),
        ("pt", "Hong Kong"),
        ("pt_BR", "Hong Kong"),
        ("ro", "Hong Kong"),
        ("ru", "Гонконг"),
        ("rw", "Hongo Kongo"),
        ("sc", "Hong Kong"),
        ("sd", "Hong Kong"),
        ("si", "හොංග\u{dca} කොංග\u{dca}"),
        ("sk", "Hongkong"),
        ("sl", "Hong Kong"),
        ("so", "Hong Kong"),
        ("sq", "Hong Kong"),
        ("sr", "Хонг Конг"),
        ("sv", "Hongkong"),
        ("sw", "Hong Kong"),
        ("ta", "ஹ\u{bbe}ங\u{bcd}க\u{bbe}ங\u{bcd}"),
        ("te", "హ\u{c56}ంగ\u{c4d}\u{200c}క\u{c3e}ంగ\u{c4d}"),
        ("tg", "Ҳонгконг"),
        ("th", "ฮ\u{e48}องกง"),
        ("ti", "ሆንግ ኮንግ"),
        ("tk", "Gonkong"),
        ("tl", "Hong Kong"),
        ("tr", "Hong Kong"),
        ("tt", "Һонg Конg"),
        ("ug", "شياڭگاڭ"),
        ("uk", "Гонконг"),
        ("ur", "ہانگ کانگ"),
        ("uz", "Gonkong"),
        ("ve", "Hong Kong"),
        ("vi", "Hông Kông"),
        ("wa", "Hong Kong"),
        ("wo", "Ooŋ Koŋ"),
        ("xh", "Hong Kong"),
        ("yo", "Họ\u{301}ng Kọng"),
        ("zh_CN", "香港"),
        ("zh_HK", "香港"),
        ("zh_TW", "香港"),
        ("zu", "Hong Kong"),
    ];
    #[cfg(all(feature = "hk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 22.3193039;
        pub const LONGITUDE: f64 = 114.1693611;
        pub const MAX_LATITUDE: f64 = 22.5619469;
        pub const MAX_LONGITUDE: f64 = 114.4294999;
        pub const MIN_LATITUDE: f64 = 22.1435;
        pub const MIN_LONGITUDE: f64 = 113.8259001;
        pub const NORTHEAST_LATITUDE: f64 = 22.5619469;
        pub const NORTHEAST_LONGITUDE: f64 = 114.4294999;
        pub const SOUTHWEST_LATITUDE: f64 = 22.1435;
        pub const SOUTHWEST_LONGITUDE: f64 = 113.8259001;
    }
}
#[cfg(all(feature = "hk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 22.3193039,
            longitude: 114.1693611,
            max_latitude: 22.5619469,
            max_longitude: 114.4294999,
            min_latitude: 22.1435,
            min_longitude: 113.8259001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 22.5619469,
                    longitude: 114.4294999,
                },
                southwest: CountryGeoBound {
                    latitude: 22.1435,
                    longitude: 113.8259001,
                },
            },
        }
    }
}

#[cfg(all(feature = "hk", feature = "subdivisions"))]
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
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "hk")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::HK,
        alpha3: Alpha3::HKG,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 852,
        currency_code: "HKD",
        gec: Some(GEC::HK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "001",
        ioc: Some("HKG"),
        iso_long_name: "The Hong Kong Special Administrative Region of China",
        iso_short_name: "Hong Kong",
        official_language_list: ["en", "zh"].to_vec(),
        spoken_language_list: ["en", "zh"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Hong Kongese"),
        number: "344",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "HK",
        unofficial_name_list: ["Hong Kong", "香港", "Hongkong"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Hong Kong"),
            ("af", "Hongkong"),
            ("ak", "Hong Kong"),
            ("am", "ሆንጔ ጥንጔ"),
            ("an", "Hong Kong"),
            ("ar", "هونغ كونغ"),
            ("as", "হংকং"),
            ("ay", "Hong Kong"),
            ("az", "Honq Konq"),
            ("ba", "Hong Kong"),
            ("be", "Ганконг"),
            ("bg", "Хонг Конг"),
            ("bi", "Hong Kong"),
            ("bn", "হং কং"),
            ("bn_IN", "হং কং"),
            ("br", "Hong Kong"),
            ("bs", "Hong Kong"),
            ("ca", "Hong Kong"),
            ("ce", "Гонконг"),
            ("ch", "Hong Kong"),
            ("cs", "Hongkong"),
            ("cv", "Гонконг"),
            ("cy", "Hong Kong"),
            ("da", "Hongkong"),
            ("de", "Hongkong"),
            ("dv", "ހ\u{7ae}ނ\u{7b0}ކ\u{7ae}ނ\u{7b0}ގ\u{7aa}"),
            ("dz", "ཧ\u{f7c}ང་ཀ\u{f7c}ང་"),
            ("ee", "Hong Kong"),
            ("el", "Χονγκ Κονγκ"),
            ("en", "Hong Kong"),
            ("eo", "Honkongo"),
            ("es", "Hong Kong"),
            ("et", "Hongkong"),
            ("eu", "Hong Kong"),
            ("fa", "هنگ کنگ"),
            ("ff", "Hong Kong"),
            ("fi", "Hong Kong"),
            ("fo", "Hong Kong"),
            ("fr", "Hong Kong"),
            ("fy", "Hongkong"),
            ("ga", "Hong Cong"),
            ("gl", "Hong Kong"),
            ("gn", "Hong Kong"),
            ("gu", "હો\u{a82}ગ કો\u{a82}ગ"),
            ("gv", "Hong Kong"),
            ("ha", "Hong Kong"),
            ("he", "הונג קונג"),
            ("hi", "हा\u{902}गका\u{902}ग"),
            ("hr", "Hong Kong"),
            ("ht", "Hong Kong"),
            ("hu", "Hongkong"),
            ("hy", "Հոնգ Կոնգ"),
            ("ia", "Hong Kong"),
            ("id", "Hong Kong"),
            ("io", "Hong Kong"),
            ("is", "Hong Kong"),
            ("it", "Hong Kong"),
            ("iu", "Hong Kong"),
            ("ja", "香港"),
            ("ka", "ჰონკონგი"),
            ("ki", "Hong Kong"),
            ("kk", "Сянган (Гонконг)"),
            ("kl", "Hong Kong"),
            ("km", "ហ\u{17bb}ងក\u{17bb}ង"),
            ("kn", "ಹಾಂಗ\u{ccd} ಕಾಂಗ\u{ccd}"),
            ("ko", "홍콩"),
            ("ku", "Hong Kong"),
            ("kv", "Hong Kong"),
            ("kw", "Hong Kong"),
            ("ky", "Гонконг"),
            ("lo", "ຮ\u{ebb}ງກ\u{ebb}ງ"),
            ("lt", "Honkongas"),
            ("lv", "Honkonga"),
            ("mi", "Hongipua"),
            ("mk", "Хонк Конг"),
            ("ml", "ഹോങ\u{d4d} കോങ\u{d4d}"),
            ("mn", "Хонконг"),
            ("mr", "हॉ\u{902}गकॉ\u{902}ग"),
            ("ms", "Hong Kong"),
            ("mt", "Ħong Kong"),
            ("my", "ဟောင\u{103a}ကောင\u{103a}"),
            ("na", "Hong Kong"),
            ("nb", "Hongkong"),
            ("ne", "हङकङ"),
            ("nl", "Hongkong"),
            ("nn", "Hongkong"),
            ("nv", "Hong Kong"),
            ("oc", "Hong Kong"),
            ("or", "ହଙ\u{b4d}ଗ କଙ\u{b4d}ଗ"),
            ("pa", "ਹਾ\u{a02}ਗ ਕਾ\u{a02}ਗ"),
            ("pi", "Hong Kong"),
            ("pl", "Hongkong"),
            ("ps", "هانګ کانګ"),
            ("pt", "Hong Kong"),
            ("pt_BR", "Hong Kong"),
            ("ro", "Hong Kong"),
            ("ru", "Гонконг"),
            ("rw", "Hongo Kongo"),
            ("sc", "Hong Kong"),
            ("sd", "Hong Kong"),
            ("si", "හොංග\u{dca} කොංග\u{dca}"),
            ("sk", "Hongkong"),
            ("sl", "Hong Kong"),
            ("so", "Hong Kong"),
            ("sq", "Hong Kong"),
            ("sr", "Хонг Конг"),
            ("sv", "Hongkong"),
            ("sw", "Hong Kong"),
            ("ta", "ஹ\u{bbe}ங\u{bcd}க\u{bbe}ங\u{bcd}"),
            ("te", "హ\u{c56}ంగ\u{c4d}\u{200c}క\u{c3e}ంగ\u{c4d}"),
            ("tg", "Ҳонгконг"),
            ("th", "ฮ\u{e48}องกง"),
            ("ti", "ሆንግ ኮንግ"),
            ("tk", "Gonkong"),
            ("tl", "Hong Kong"),
            ("tr", "Hong Kong"),
            ("tt", "Һонg Конg"),
            ("ug", "شياڭگاڭ"),
            ("uk", "Гонконг"),
            ("ur", "ہانگ کانگ"),
            ("uz", "Gonkong"),
            ("ve", "Hong Kong"),
            ("vi", "Hông Kông"),
            ("wa", "Hong Kong"),
            ("wo", "Ooŋ Koŋ"),
            ("xh", "Hong Kong"),
            ("yo", "Họ\u{301}ng Kọng"),
            ("zh_CN", "香港"),
            ("zh_HK", "香港"),
            ("zh_TW", "香港"),
            ("zu", "Hong Kong"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}