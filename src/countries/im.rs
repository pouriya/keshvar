// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Isle of Man

#[cfg(all(feature = "im", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::IM;
    pub const ALPHA3: Alpha3 = Alpha3::IMN;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 44;
    pub const CURRENCY_CODE: &str = "GBP";
    pub const GEC: Option<GEC> = Some(GEC::IM);
    pub const INTERNATIONAL_PREFIX: &str = "";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Isle of Man";
    pub const ISO_LONG_NAME: &str = "The Isle of Man";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "gv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "gv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_PREFIX: &str = "";
    pub const NATIONALITY: Option<&str> = Some("Manx");
    pub const NUMBER: &str = "833";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("IM\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "IM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Isle of Man",
        "Insel Man",
        "Île de Man",
        "Isla de Man",
        "マン島",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Isle of Man"),
        ("af", "Eiland Man"),
        ("ak", "Isle of Man"),
        ("am", "Isle of Man"),
        ("an", "Isle of Man"),
        ("ar", "آيزل أف مان"),
        ("as", "আইল অফ মেন"),
        ("ay", "Isle of Man"),
        ("az", "Isle of Man"),
        ("ba", "Isle of Man"),
        ("be", "Востраў Мэн"),
        ("bg", "Айл ъф мен"),
        ("bi", "Isle of Man"),
        ("bn", "আইল অফ ম\u{9cd}য\u{9be}ন"),
        ("bn_IN", "আইল অফ ম\u{9cd}য\u{9be}ন"),
        ("br", "Manav"),
        ("bs", "Ostrvo Man"),
        ("ca", "Illa de Man"),
        ("ce", "Isle of Man"),
        ("ch", "Isle of Man"),
        ("cs", "Ostrov Man"),
        ("cv", "Isle of Man"),
        ("cy", "Ynys Manaw"),
        ("da", "Isle of Man"),
        ("de", "Insel Man"),
        (
            "dv",
            "އ\u{7a6}އ\u{7a8}ޒ\u{7a6}ލ\u{7b0} އ\u{7ae}ފ\u{7b0} މ\u{7ad}ނ\u{7b0}",
        ),
        ("dz", "ཨ་ཡ\u{f72}ལ་ཨ\u{f7c}ཕ་མ\u{f7a}ན།"),
        ("ee", "Isle of Man"),
        ("el", "Νήσος του Μαν"),
        ("en", "Isle of Man"),
        ("eo", "Manksinsulo"),
        ("es", "Isla de Man"),
        ("et", "Man"),
        ("eu", "Man uhartea"),
        ("fa", "جزیره من"),
        ("ff", "Isle of Man"),
        ("fi", "Mansaari"),
        ("fo", "Isle of Man"),
        ("fr", "Île de Man"),
        ("fy", "Man"),
        ("ga", "Oileán Mhanann"),
        ("gl", "Illa de Man"),
        ("gn", "Isle of Man"),
        ("gu", "આઇસ\u{acd}લ\u{ac7} ઓફ મ\u{ac7}ન"),
        ("gv", "Mannin"),
        ("ha", "Isle of Man"),
        ("he", "האי מאן"),
        ("hi", "मन\u{941}ष\u{94d}य का टाप\u{942}"),
        ("hr", "Otok Man"),
        ("ht", "Isle of Man"),
        ("hu", "Man"),
        ("hy", "Մեն Կղզի"),
        ("ia", "Insula de Man"),
        ("id", "Pulau Man"),
        ("io", "Man-Insulo"),
        ("is", "Eyjan Mön"),
        ("it", "Isola di Man"),
        ("iu", "Isle of Man"),
        ("ja", "マン島"),
        ("ka", "მენის კუნძული"),
        ("ki", "Isle of Man"),
        ("kk", "Мэн аралы"),
        ("kl", "Isle of Man"),
        ("km", "Isle of Man"),
        ("kn", "Isle of Man"),
        ("ko", "맨 섬"),
        ("ku", "Girava Man"),
        ("kv", "Isle of Man"),
        ("kw", "Ynys Manow"),
        ("ky", "Мэн аралы"),
        ("lo", "Isle of Man"),
        ("lt", "Meno sala"),
        ("lv", "Menas sala"),
        ("mi", "Motu o Man"),
        ("mk", "Ман остров"),
        ("ml", "ഐല\u{d4d}\u{200d} ഓഫ\u{d4d} മ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Isle of Man"),
        ("mr", "आइल ऑफ म\u{945}न"),
        ("ms", "Isle of Man"),
        ("mt", "Gżira ta' Man"),
        ("my", "Isle of Man"),
        ("na", "Isle of Man"),
        ("nb", "Man"),
        ("ne", "मानव द\u{94d}वीप"),
        ("nl", "Eiland Man"),
        ("nn", "Øya Man"),
        ("nv", "Isle of Man"),
        ("oc", "Illa de Man"),
        ("or", "ଇସ\u{b4d}ଲେ ଅଫ ମେନ"),
        ("pa", "ਇਸਲ\u{a47} ਆਫ ਮ\u{a48}ਨ"),
        ("pi", "Isle of Man"),
        ("pl", "Wyspa Man"),
        ("ps", "Isle of Man"),
        ("pt", "Ilha de Man"),
        ("pt_BR", "Ilha de Man"),
        ("ro", "Insula Man"),
        ("ru", "Остров Мэн"),
        ("rw", "Ikirwa cya Man"),
        ("sc", "Ìsula de Man"),
        ("sd", "Isle of Man"),
        ("si", "ම\u{dcf}න\u{dca} ද\u{dd2}වය\u{dd2}න"),
        ("sk", "Man"),
        ("sl", "Otok Man"),
        ("so", "Isle of Man"),
        ("sq", "Ishulli Man"),
        ("sr", "Острво Ман"),
        ("sv", "Isle of Man"),
        ("sw", "Isle of Man"),
        ("ta", "ஐல\u{bcd} ஆஃப\u{bcd} மேன\u{bcd}"),
        ("te", "ఐల\u{c46} ఆఫ\u{c4d} మ\u{c3e}న\u{c4d}"),
        ("tg", "Ҷазираи Мэн"),
        ("th", "เกาะแมน"),
        ("ti", "Isle of Man"),
        ("tk", "Isle of Man"),
        ("tl", "Pulo ng Man"),
        ("tr", "Man Adası"),
        ("tt", "Isle of Man"),
        ("ug", "مەن ئارىلى"),
        ("uk", "Острів Мен"),
        ("ur", "آئل آف مین"),
        ("uz", "Isle of Man"),
        ("ve", "Isle of Man"),
        ("vi", "Đảo Man"),
        ("wa", "Iye di Man"),
        ("wo", "Isle of Man"),
        ("xh", "Isle of Man"),
        ("yo", "Erékùṣù ilẹ\u{300} Man"),
        ("zh_CN", "曼岛"),
        ("zh_HK", "萌島"),
        ("zh_TW", "曼島"),
        ("zu", "Isle of Man"),
    ];
    #[cfg(all(feature = "im", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 54.236107;
        pub const LONGITUDE: f64 = -4.548056;
        pub const MAX_LATITUDE: f64 = 54.4369363;
        pub const MAX_LONGITUDE: f64 = -4.270618199999999;
        pub const MIN_LATITUDE: f64 = 54.0186764;
        pub const MIN_LONGITUDE: f64 = -4.8736609;
        pub const NORTHEAST_LATITUDE: f64 = 54.4369363;
        pub const NORTHEAST_LONGITUDE: f64 = -4.270618199999999;
        pub const SOUTHWEST_LATITUDE: f64 = 54.0186764;
        pub const SOUTHWEST_LONGITUDE: f64 = -4.8736609;
    }
}
#[cfg(all(feature = "im", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 54.236107,
            longitude: -4.548056,
            max_latitude: 54.4369363,
            max_longitude: -4.270618199999999,
            min_latitude: 54.0186764,
            min_longitude: -4.8736609,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 54.4369363,
                    longitude: -4.270618199999999,
                },
                southwest: CountryGeoBound {
                    latitude: 54.0186764,
                    longitude: -4.8736609,
                },
            },
        }
    }
}

#[cfg(all(feature = "im", feature = "subdivisions"))]
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
#[cfg(feature = "im")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::IM,
        alpha3: Alpha3::IMN,
        address_format: None,
        continent: Continent::Europe,
        country_code: 44,
        currency_code: "GBP",
        gec: Some(GEC::IM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "",
        ioc: None,
        iso_long_name: "The Isle of Man",
        iso_short_name: "Isle of Man",
        official_language_list: ["en", "gv"].to_vec(),
        spoken_language_list: ["en", "gv"].to_vec(),
        national_destination_code_length_list: [].to_vec(),
        national_number_length_list: [].to_vec(),
        national_prefix: "",
        nationality: Some("Manx"),
        number: "833",
        postal_code: true,
        postal_code_format: Some("IM\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "IM",
        unofficial_name_list: [
            "Isle of Man",
            "Insel Man",
            "Île de Man",
            "Isla de Man",
            "マン島",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Isle of Man"),
            ("af", "Eiland Man"),
            ("ak", "Isle of Man"),
            ("am", "Isle of Man"),
            ("an", "Isle of Man"),
            ("ar", "آيزل أف مان"),
            ("as", "আইল অফ মেন"),
            ("ay", "Isle of Man"),
            ("az", "Isle of Man"),
            ("ba", "Isle of Man"),
            ("be", "Востраў Мэн"),
            ("bg", "Айл ъф мен"),
            ("bi", "Isle of Man"),
            ("bn", "আইল অফ ম\u{9cd}য\u{9be}ন"),
            ("bn_IN", "আইল অফ ম\u{9cd}য\u{9be}ন"),
            ("br", "Manav"),
            ("bs", "Ostrvo Man"),
            ("ca", "Illa de Man"),
            ("ce", "Isle of Man"),
            ("ch", "Isle of Man"),
            ("cs", "Ostrov Man"),
            ("cv", "Isle of Man"),
            ("cy", "Ynys Manaw"),
            ("da", "Isle of Man"),
            ("de", "Insel Man"),
            (
                "dv",
                "އ\u{7a6}އ\u{7a8}ޒ\u{7a6}ލ\u{7b0} އ\u{7ae}ފ\u{7b0} މ\u{7ad}ނ\u{7b0}",
            ),
            ("dz", "ཨ་ཡ\u{f72}ལ་ཨ\u{f7c}ཕ་མ\u{f7a}ན།"),
            ("ee", "Isle of Man"),
            ("el", "Νήσος του Μαν"),
            ("en", "Isle of Man"),
            ("eo", "Manksinsulo"),
            ("es", "Isla de Man"),
            ("et", "Man"),
            ("eu", "Man uhartea"),
            ("fa", "جزیره من"),
            ("ff", "Isle of Man"),
            ("fi", "Mansaari"),
            ("fo", "Isle of Man"),
            ("fr", "Île de Man"),
            ("fy", "Man"),
            ("ga", "Oileán Mhanann"),
            ("gl", "Illa de Man"),
            ("gn", "Isle of Man"),
            ("gu", "આઇસ\u{acd}લ\u{ac7} ઓફ મ\u{ac7}ન"),
            ("gv", "Mannin"),
            ("ha", "Isle of Man"),
            ("he", "האי מאן"),
            ("hi", "मन\u{941}ष\u{94d}य का टाप\u{942}"),
            ("hr", "Otok Man"),
            ("ht", "Isle of Man"),
            ("hu", "Man"),
            ("hy", "Մեն Կղզի"),
            ("ia", "Insula de Man"),
            ("id", "Pulau Man"),
            ("io", "Man-Insulo"),
            ("is", "Eyjan Mön"),
            ("it", "Isola di Man"),
            ("iu", "Isle of Man"),
            ("ja", "マン島"),
            ("ka", "მენის კუნძული"),
            ("ki", "Isle of Man"),
            ("kk", "Мэн аралы"),
            ("kl", "Isle of Man"),
            ("km", "Isle of Man"),
            ("kn", "Isle of Man"),
            ("ko", "맨 섬"),
            ("ku", "Girava Man"),
            ("kv", "Isle of Man"),
            ("kw", "Ynys Manow"),
            ("ky", "Мэн аралы"),
            ("lo", "Isle of Man"),
            ("lt", "Meno sala"),
            ("lv", "Menas sala"),
            ("mi", "Motu o Man"),
            ("mk", "Ман остров"),
            ("ml", "ഐല\u{d4d}\u{200d} ഓഫ\u{d4d} മ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Isle of Man"),
            ("mr", "आइल ऑफ म\u{945}न"),
            ("ms", "Isle of Man"),
            ("mt", "Gżira ta' Man"),
            ("my", "Isle of Man"),
            ("na", "Isle of Man"),
            ("nb", "Man"),
            ("ne", "मानव द\u{94d}वीप"),
            ("nl", "Eiland Man"),
            ("nn", "Øya Man"),
            ("nv", "Isle of Man"),
            ("oc", "Illa de Man"),
            ("or", "ଇସ\u{b4d}ଲେ ଅଫ ମେନ"),
            ("pa", "ਇਸਲ\u{a47} ਆਫ ਮ\u{a48}ਨ"),
            ("pi", "Isle of Man"),
            ("pl", "Wyspa Man"),
            ("ps", "Isle of Man"),
            ("pt", "Ilha de Man"),
            ("pt_BR", "Ilha de Man"),
            ("ro", "Insula Man"),
            ("ru", "Остров Мэн"),
            ("rw", "Ikirwa cya Man"),
            ("sc", "Ìsula de Man"),
            ("sd", "Isle of Man"),
            ("si", "ම\u{dcf}න\u{dca} ද\u{dd2}වය\u{dd2}න"),
            ("sk", "Man"),
            ("sl", "Otok Man"),
            ("so", "Isle of Man"),
            ("sq", "Ishulli Man"),
            ("sr", "Острво Ман"),
            ("sv", "Isle of Man"),
            ("sw", "Isle of Man"),
            ("ta", "ஐல\u{bcd} ஆஃப\u{bcd} மேன\u{bcd}"),
            ("te", "ఐల\u{c46} ఆఫ\u{c4d} మ\u{c3e}న\u{c4d}"),
            ("tg", "Ҷазираи Мэн"),
            ("th", "เกาะแมน"),
            ("ti", "Isle of Man"),
            ("tk", "Isle of Man"),
            ("tl", "Pulo ng Man"),
            ("tr", "Man Adası"),
            ("tt", "Isle of Man"),
            ("ug", "مەن ئارىلى"),
            ("uk", "Острів Мен"),
            ("ur", "آئل آف مین"),
            ("uz", "Isle of Man"),
            ("ve", "Isle of Man"),
            ("vi", "Đảo Man"),
            ("wa", "Iye di Man"),
            ("wo", "Isle of Man"),
            ("xh", "Isle of Man"),
            ("yo", "Erékùṣù ilẹ\u{300} Man"),
            ("zh_CN", "曼岛"),
            ("zh_HK", "萌島"),
            ("zh_TW", "曼島"),
            ("zu", "Isle of Man"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
