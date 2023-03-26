// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Réunion

#[cfg(all(feature = "re", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::RE;
    pub const ALPHA3: Alpha3 = Alpha3::REU;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 262;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::RE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Réunion";
    pub const ISO_LONG_NAME: &str = "Réunion";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("French");
    pub const NUMBER: &str = "638";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("9[78]4\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "RE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Réunion", "Reunión", "Reunion", "レユニオン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Réunion"),
        ("af", "Réunion"),
        ("ak", "Réunion"),
        ("am", "ሬዩንዮን"),
        ("an", "Réunion"),
        ("ar", "ريونيون"),
        ("as", "ৰিইউনিয়\u{9be}ন"),
        ("ay", "Réunion"),
        ("az", "Reyunion"),
        ("ba", "Réunion"),
        ("be", "Рэюньён"),
        ("bg", "Риюниън"),
        ("bi", "Réunion"),
        ("bn", "রিইউনিয়\u{9be}ন"),
        ("bn_IN", "রিইউনিয়\u{9be}ন"),
        ("br", "Enez ar Reunion"),
        ("bs", "Réunion"),
        ("ca", "Reunió"),
        ("ce", "Реюньон"),
        ("ch", "Réunion"),
        ("cs", "Réunion"),
        ("cv", "Реюньон"),
        ("cy", "Réunion"),
        ("da", "Réunion"),
        ("de", "Réunion"),
        ("dv", "ރ\u{7a8}ޔ\u{7ab}ނ\u{7a8}އ\u{7a6}ނ\u{7b0}"),
        ("dz", "ས\u{fb3}ར་བས\u{fa1}\u{f7c}མ།"),
        ("ee", "Réunion"),
        ("el", "Ρεϋνιόν"),
        ("en", "Réunion"),
        ("eo", "Reunio"),
        ("es", "Reunión"),
        ("et", "Réunion"),
        ("eu", "Réunion"),
        ("fa", "رئونیون"),
        ("ff", "Réunion"),
        ("fi", "Réunion"),
        ("fo", "Réunion"),
        ("fr", "Réunion, Île de la"),
        ("fy", "Réunion"),
        ("ga", "Réunion"),
        ("gl", "Reunión"),
        ("gn", "Réunion"),
        ("gu", "રીય\u{ac1}નિયન"),
        ("gv", "Réunion"),
        ("ha", "Réunion"),
        ("he", "ראוניון"),
        ("hi", "र\u{947}य\u{942}नियो\u{902}"),
        ("hr", "Réunion"),
        ("ht", "La Reyinyon"),
        ("hu", "Réunion"),
        ("hy", "Ռեյունյոն"),
        ("ia", "Reunion"),
        ("id", "Réunion"),
        ("io", "Reunion"),
        ("is", "Réunion"),
        ("it", "Riunione"),
        ("iu", "Réunion"),
        ("ja", "レユニオン"),
        ("ka", "რეუნიონი"),
        ("ki", "Réunion"),
        ("kk", "Реюньон"),
        ("kl", "Réunion"),
        ("km", "រេអ\u{17bb}យញ\u{17c9}\u{17bb}ង"),
        ("kn", "Réunion"),
        ("ko", "레위니옹"),
        ("ku", "Reûnyon"),
        ("kv", "Réunion"),
        ("kw", "Réunion"),
        ("ky", "Реюньон департаменти"),
        ("lo", "Réunion"),
        ("lt", "Reunjonas"),
        ("lv", "Reinjona"),
        ("mi", "Réunion"),
        ("mk", "Реунион"),
        ("ml", "റീയ\u{d42}ണിയന\u{d4d}\u{200d}"),
        ("mn", "Реюньон"),
        ("mr", "रिय\u{941}नियन"),
        ("ms", "Réunion"),
        ("mt", "Réunion"),
        ("my", "Réunion"),
        ("na", "Réunion"),
        ("nb", "Réunion"),
        ("ne", "रिय\u{942}नियन"),
        ("nl", "Réunion"),
        ("nn", "Réunion"),
        ("nv", "Réunion"),
        ("oc", "Réunion"),
        ("or", "ର\u{b3f}ୟ\u{b41}ନ\u{b3f}ୟନ"),
        ("pa", "ਰੀਯ\u{a41}ਨੀਅਨ"),
        ("pi", "Réunion"),
        ("pl", "Reunion"),
        ("ps", "Réunion"),
        ("pt", "Ilha Reunião"),
        ("pt_BR", "Reunião"),
        ("ro", "Réunion"),
        ("ru", "Реюньон"),
        ("rw", "Reyiniyo"),
        ("sc", "Riunione"),
        ("sd", "Réunion"),
        ("si", "ප\u{dca}\u{200d}රත\u{dd2}සංව\u{dd2}ධ\u{dcf}නය"),
        ("sk", "Réunion"),
        ("sl", "Reunion"),
        ("so", "Réunion"),
        ("sq", "Réunion"),
        ("sr", "Реунион"),
        ("sv", "Réunion"),
        ("sw", "Réunion"),
        ("ta", "ர\u{bc0}யூனியன\u{bcd}"),
        ("te", "ర\u{c3f}యూన\u{c3f}యన\u{c4d}"),
        ("tg", "Реюнон"),
        ("th", "เรอ\u{e39}น\u{e35}ยง"),
        ("ti", "Réunion"),
        ("tk", "Reýunon"),
        ("tl", "Réunion"),
        ("tr", "Réunion"),
        ("tt", "Рéунион"),
        ("ug", "رېئونىيون"),
        ("uk", "Реюньйон"),
        ("ur", "غے یونیوں"),
        ("uz", "Réunion"),
        ("ve", "Réunion"),
        ("vi", "Rê-u-ni-ợnh"),
        ("wa", "Reyunion"),
        ("wo", "Reiñõo"),
        ("xh", "Réunion"),
        ("yo", "Réunion"),
        ("zh_CN", "留尼汪"),
        ("zh_HK", "留尼旺"),
        ("zh_TW", "留尼旺島"),
        ("zu", "IRiyunion"),
    ];
    #[cfg(all(feature = "re", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -21.115141;
        pub const LONGITUDE: f64 = 55.536384;
        pub const MAX_LATITUDE: f64 = -20.8671529;
        pub const MAX_LONGITUDE: f64 = 55.84487919999999;
        pub const MIN_LATITUDE: f64 = -21.4035321;
        pub const MIN_LONGITUDE: f64 = 55.209732;
        pub const NORTHEAST_LATITUDE: f64 = -20.8671529;
        pub const NORTHEAST_LONGITUDE: f64 = 55.84487919999999;
        pub const SOUTHWEST_LATITUDE: f64 = -21.4035321;
        pub const SOUTHWEST_LONGITUDE: f64 = 55.209732;
    }
}
#[cfg(all(feature = "re", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -21.115141,
            longitude: 55.536384,
            max_latitude: -20.8671529,
            max_longitude: 55.84487919999999,
            min_latitude: -21.4035321,
            min_longitude: 55.209732,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -20.8671529,
                    longitude: 55.84487919999999,
                },
                southwest: CountryGeoBound {
                    latitude: -21.4035321,
                    longitude: 55.209732,
                },
            },
        }
    }
}

#[cfg(all(feature = "re", feature = "subdivisions"))]
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
#[cfg(feature = "re")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::RE,
        alpha3: Alpha3::REU,
        address_format: None,
        continent: Continent::Africa,
        country_code: 262,
        currency_code: "EUR",
        gec: Some(GEC::RE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "Réunion",
        iso_short_name: "Réunion",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "None",
        nationality: Some("French"),
        number: "638",
        postal_code: true,
        postal_code_format: Some("9[78]4\\d{2}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "RE",
        unofficial_name_list: ["Réunion", "Reunión", "Reunion", "レユニオン"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Réunion"),
            ("af", "Réunion"),
            ("ak", "Réunion"),
            ("am", "ሬዩንዮን"),
            ("an", "Réunion"),
            ("ar", "ريونيون"),
            ("as", "ৰিইউনিয়\u{9be}ন"),
            ("ay", "Réunion"),
            ("az", "Reyunion"),
            ("ba", "Réunion"),
            ("be", "Рэюньён"),
            ("bg", "Риюниън"),
            ("bi", "Réunion"),
            ("bn", "রিইউনিয়\u{9be}ন"),
            ("bn_IN", "রিইউনিয়\u{9be}ন"),
            ("br", "Enez ar Reunion"),
            ("bs", "Réunion"),
            ("ca", "Reunió"),
            ("ce", "Реюньон"),
            ("ch", "Réunion"),
            ("cs", "Réunion"),
            ("cv", "Реюньон"),
            ("cy", "Réunion"),
            ("da", "Réunion"),
            ("de", "Réunion"),
            ("dv", "ރ\u{7a8}ޔ\u{7ab}ނ\u{7a8}އ\u{7a6}ނ\u{7b0}"),
            ("dz", "ས\u{fb3}ར་བས\u{fa1}\u{f7c}མ།"),
            ("ee", "Réunion"),
            ("el", "Ρεϋνιόν"),
            ("en", "Réunion"),
            ("eo", "Reunio"),
            ("es", "Reunión"),
            ("et", "Réunion"),
            ("eu", "Réunion"),
            ("fa", "رئونیون"),
            ("ff", "Réunion"),
            ("fi", "Réunion"),
            ("fo", "Réunion"),
            ("fr", "Réunion, Île de la"),
            ("fy", "Réunion"),
            ("ga", "Réunion"),
            ("gl", "Reunión"),
            ("gn", "Réunion"),
            ("gu", "રીય\u{ac1}નિયન"),
            ("gv", "Réunion"),
            ("ha", "Réunion"),
            ("he", "ראוניון"),
            ("hi", "र\u{947}य\u{942}नियो\u{902}"),
            ("hr", "Réunion"),
            ("ht", "La Reyinyon"),
            ("hu", "Réunion"),
            ("hy", "Ռեյունյոն"),
            ("ia", "Reunion"),
            ("id", "Réunion"),
            ("io", "Reunion"),
            ("is", "Réunion"),
            ("it", "Riunione"),
            ("iu", "Réunion"),
            ("ja", "レユニオン"),
            ("ka", "რეუნიონი"),
            ("ki", "Réunion"),
            ("kk", "Реюньон"),
            ("kl", "Réunion"),
            ("km", "រេអ\u{17bb}យញ\u{17c9}\u{17bb}ង"),
            ("kn", "Réunion"),
            ("ko", "레위니옹"),
            ("ku", "Reûnyon"),
            ("kv", "Réunion"),
            ("kw", "Réunion"),
            ("ky", "Реюньон департаменти"),
            ("lo", "Réunion"),
            ("lt", "Reunjonas"),
            ("lv", "Reinjona"),
            ("mi", "Réunion"),
            ("mk", "Реунион"),
            ("ml", "റീയ\u{d42}ണിയന\u{d4d}\u{200d}"),
            ("mn", "Реюньон"),
            ("mr", "रिय\u{941}नियन"),
            ("ms", "Réunion"),
            ("mt", "Réunion"),
            ("my", "Réunion"),
            ("na", "Réunion"),
            ("nb", "Réunion"),
            ("ne", "रिय\u{942}नियन"),
            ("nl", "Réunion"),
            ("nn", "Réunion"),
            ("nv", "Réunion"),
            ("oc", "Réunion"),
            ("or", "ର\u{b3f}ୟ\u{b41}ନ\u{b3f}ୟନ"),
            ("pa", "ਰੀਯ\u{a41}ਨੀਅਨ"),
            ("pi", "Réunion"),
            ("pl", "Reunion"),
            ("ps", "Réunion"),
            ("pt", "Ilha Reunião"),
            ("pt_BR", "Reunião"),
            ("ro", "Réunion"),
            ("ru", "Реюньон"),
            ("rw", "Reyiniyo"),
            ("sc", "Riunione"),
            ("sd", "Réunion"),
            ("si", "ප\u{dca}\u{200d}රත\u{dd2}සංව\u{dd2}ධ\u{dcf}නය"),
            ("sk", "Réunion"),
            ("sl", "Reunion"),
            ("so", "Réunion"),
            ("sq", "Réunion"),
            ("sr", "Реунион"),
            ("sv", "Réunion"),
            ("sw", "Réunion"),
            ("ta", "ர\u{bc0}யூனியன\u{bcd}"),
            ("te", "ర\u{c3f}యూన\u{c3f}యన\u{c4d}"),
            ("tg", "Реюнон"),
            ("th", "เรอ\u{e39}น\u{e35}ยง"),
            ("ti", "Réunion"),
            ("tk", "Reýunon"),
            ("tl", "Réunion"),
            ("tr", "Réunion"),
            ("tt", "Рéунион"),
            ("ug", "رېئونىيون"),
            ("uk", "Реюньйон"),
            ("ur", "غے یونیوں"),
            ("uz", "Réunion"),
            ("ve", "Réunion"),
            ("vi", "Rê-u-ni-ợnh"),
            ("wa", "Reyunion"),
            ("wo", "Reiñõo"),
            ("xh", "Réunion"),
            ("yo", "Réunion"),
            ("zh_CN", "留尼汪"),
            ("zh_HK", "留尼旺"),
            ("zh_TW", "留尼旺島"),
            ("zu", "IRiyunion"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
