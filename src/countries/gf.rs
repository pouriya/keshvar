// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Guyane

#[cfg(all(feature = "gf", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GF;
    pub const ALPHA3: Alpha3 = Alpha3::GUF;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 594;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::FG);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "French Guiana";
    pub const ISO_LONG_NAME: &str = "Guyane";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("French Guianan");
    pub const NUMBER: &str = "254";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("9[78]3\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "GF";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "French Guiana",
        "Französisch Guyana",
        "Guayana Francesa",
        "フランス領ギアナ",
        "Frans-Guyana",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "French Guiana"),
        ("af", "Frans-Guyana"),
        ("ak", "French Guiana"),
        ("am", "ፘፈረንሳ፤ ጉፁጐና"),
        ("an", "French Guiana"),
        ("ar", "غيانا الفرنسي\u{651}ة"),
        ("as", "ফ\u{9cd}ৰেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
        ("ay", "French Guiana"),
        ("az", "Fransız Gvineyası"),
        ("ba", "French Guiana"),
        ("be", "Французская Гвіяна"),
        ("bg", "Френска Гвиана"),
        ("bi", "French Guiana"),
        ("bn", "ফ\u{9cd}রেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
        ("bn_IN", "ফ\u{9cd}রেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
        ("br", "Gwiana c'hall"),
        ("bs", "Francuska Gvajana"),
        ("ca", "Guaiana Francesa"),
        ("ce", "Французийн Гвиана"),
        ("ch", "French Guiana"),
        ("cs", "Francouzská Guayana"),
        ("cv", "Французийн Гвиана"),
        ("cy", "Guiana Ffrangeg"),
        ("da", "Fransk Guyana"),
        ("de", "Französisch-Guyana"),
        (
            "dv",
            "ފ\u{7a6}ރ\u{7a6}ނ\u{7b0}ސ\u{7ad}ސ\u{7a8} ގ\u{7a9}ނ\u{7a7}",
        ),
        ("dz", "ཕ\u{f7a}ར\u{f7a}ནཆ\u{f72}་ ག\u{f74}འ\u{f72}་ན།"),
        ("ee", "French Guiana"),
        ("el", "Γαλλική Γουιάνα"),
        ("en", "French Guiana"),
        ("eo", "Franca Gviano"),
        ("es", "Guayana Francesa"),
        ("et", "Prantsuse Guajaana"),
        ("eu", "Guyana Frantsesa"),
        ("fa", "گویان فرانسه"),
        ("ff", "French Guiana"),
        ("fi", "Ranskan Guayana"),
        ("fo", "French Guiana"),
        ("fr", "Guyane française"),
        ("fy", "Frânsk Guyana"),
        ("ga", "An Ghuáin Fhrancach"),
        ("gl", "Güiana Francesa"),
        ("gn", "French Guiana"),
        ("gu", "ફ\u{acd}ર\u{ac7}ન\u{acd}ચ ગ\u{ac1}એના"),
        ("gv", "French Guiana"),
        ("ha", "French Guiana"),
        ("he", "גיאנה הצרפתית"),
        ("hi", "फ\u{93c}\u{94d}रान\u{94d}सीसी ग\u{941}याना"),
        ("hr", "Francuska Gijana"),
        ("ht", "Giyàn franse"),
        ("hu", "Francia Guyana"),
        ("hy", "Ֆրանսիական Գվիանա"),
        ("ia", "Guiana Francese"),
        ("id", "Guyana Perancis"),
        ("io", "French Guiana"),
        ("is", "Franska Gvæjana"),
        ("it", "Guyana francese"),
        ("iu", "French Guiana"),
        ("ja", "仏領ギアナ"),
        ("ka", "ფრანგული გუიანა"),
        ("ki", "French Guyana"),
        ("kk", "Француз Гвианасы"),
        ("kl", "French Guiana"),
        (
            "km",
            "ហ\u{17d2}គ\u{17bc}អ\u{17ca}\u{17b8}យ\u{17c9}ាណា\u{200b}បារា\u{17c6}ង",
        ),
        ("kn", "ಫ\u{ccd}ರ\u{cc6}ಂಚ\u{ccd} ಗಯಾನಾ"),
        ("ko", "프랑스령 기아나"),
        ("ku", "Gîneya Fransî"),
        ("kv", "French Guiana"),
        ("kw", "Gwayana Frynkek"),
        ("ky", "Гвиана"),
        ("lo", "French Guiana"),
        ("lt", "Prancūzijos Gviana"),
        ("lv", "Gviāna"),
        ("mi", "Kaiana Wīwī"),
        ("mk", "Француска Гвинеја"),
        ("ml", "ഫ\u{d4d}രഞ\u{d4d}ച\u{d4d} ഗയ\u{d3e}ന"),
        ("mn", "Франц гана"),
        ("mr", "फ\u{94d}र\u{947}\u{902}च गियाना"),
        ("ms", "Guiana Perancis"),
        ("mt", "Gujana Franċiża"),
        ("my", "French Guiana"),
        ("na", "French Guiana"),
        ("nb", "Fransk Guyana"),
        ("ne", "फ\u{94d}र\u{947}न\u{94d}च जिनिया"),
        ("nl", "Frans-Guyana"),
        ("nn", "Fransk Guyana"),
        ("nv", "French Guiana"),
        ("oc", "Guaiana francesa"),
        ("or", "ଫ\u{b4d}ରେଞ\u{b4d}ଚ ଗ\u{b41}ଆନ\u{b3e}"),
        ("pa", "ਫਰ\u{a48}\u{a02}ਚ ਗ\u{a41}ਆਨਾ"),
        ("pi", "French Guiana"),
        ("pl", "Gujana Francuska"),
        ("ps", "French Guiana"),
        ("pt", "Guiana Francesa"),
        ("pt_BR", "Guiana Francesa"),
        ("ro", "Guiana Franceză"),
        ("ru", "Французская Гвиана"),
        ("rw", "Guyane Nyamfaransa"),
        ("sc", "Guyana Frantzesa"),
        ("sd", "French Guiana"),
        ("si", "ප\u{dca}\u{200d}රංශ ග\u{dd2}න\u{dd2}ය\u{dcf}ව"),
        ("sk", "Francúzska Guyana"),
        ("sl", "Francoska Gvajana"),
        ("so", "Faransiis Guyana"),
        ("sq", "Guajana Frënge"),
        ("sr", "Француска Гвајана"),
        ("sv", "Franska Guyana"),
        ("sw", "French Guiana"),
        ("ta", "ஃப\u{bcd}ரெஞ\u{bcd}ச\u{bcd} கைய\u{bbe}ன\u{bbe}"),
        ("te", "ఫ\u{c4d}ర\u{c46}ంచ\u{c4d} గ\u{c3f}య\u{c3e}న\u{c3e}"),
        ("tg", "Гвианаи Фаронса"),
        ("th", "เฟรนช\u{e4c}เก\u{e35}ยนา"),
        ("ti", "የፈረንሳይ ጉዊአና"),
        ("tk", "Fransuz Gwiana"),
        ("tl", "French Guiana"),
        ("tr", "Fransız Guyanası"),
        ("tt", "Франс Gуиана"),
        ("ug", "فىرانسىيەگە قاراشلىق گىۋىيانا"),
        ("uk", "Французька Гвіана"),
        ("ur", "فرانسیسی گیانا"),
        ("uz", "Fransiya Gvianasi"),
        ("ve", "French Guiana"),
        ("vi", "Ghi-a-na Pháp"),
        ("wa", "Guyane francesse"),
        ("wo", "Guyaana Faraañse"),
        ("xh", "French Guiana"),
        ("yo", "Gùyánà Fránsì"),
        ("zh_CN", "法属圭亚那"),
        ("zh_HK", "法屬圭亞那"),
        ("zh_TW", "法屬蓋亞那"),
        ("zu", "French Guiana"),
    ];
    #[cfg(all(feature = "gf", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 3.933889;
        pub const LONGITUDE: f64 = -53.125782;
        pub const MAX_LATITUDE: f64 = 5.9548;
        pub const MAX_LONGITUDE: f64 = -51.6164491;
        pub const MIN_LATITUDE: f64 = 2.109287;
        pub const MIN_LONGITUDE: f64 = -54.5544379;
        pub const NORTHEAST_LATITUDE: f64 = 5.9548;
        pub const NORTHEAST_LONGITUDE: f64 = -51.6164491;
        pub const SOUTHWEST_LATITUDE: f64 = 2.109287;
        pub const SOUTHWEST_LONGITUDE: f64 = -54.5544379;
    }
}
#[cfg(all(feature = "gf", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 3.933889,
            longitude: -53.125782,
            max_latitude: 5.9548,
            max_longitude: -51.6164491,
            min_latitude: 2.109287,
            min_longitude: -54.5544379,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 5.9548,
                    longitude: -51.6164491,
                },
                southwest: CountryGeoBound {
                    latitude: 2.109287,
                    longitude: -54.5544379,
                },
            },
        }
    }
}

#[cfg(all(feature = "gf", feature = "subdivisions"))]
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
#[cfg(feature = "gf")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GF,
        alpha3: Alpha3::GUF,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 594,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::FG),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "Guyane",
        iso_short_name: "French Guiana",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "None",
        nationality: Some("French Guianan"),
        number: "254",
        postal_code: true,
        postal_code_format: Some("9[78]3\\d{2}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "GF",
        unofficial_name_list: [
            "French Guiana",
            "Französisch Guyana",
            "Guayana Francesa",
            "フランス領ギアナ",
            "Frans-Guyana",
        ]
        .to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "French Guiana"),
            ("af", "Frans-Guyana"),
            ("ak", "French Guiana"),
            ("am", "ፘፈረንሳ፤ ጉፁጐና"),
            ("an", "French Guiana"),
            ("ar", "غيانا الفرنسي\u{651}ة"),
            ("as", "ফ\u{9cd}ৰেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
            ("ay", "French Guiana"),
            ("az", "Fransız Gvineyası"),
            ("ba", "French Guiana"),
            ("be", "Французская Гвіяна"),
            ("bg", "Френска Гвиана"),
            ("bi", "French Guiana"),
            ("bn", "ফ\u{9cd}রেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
            ("bn_IN", "ফ\u{9cd}রেঞ\u{9cd}চ গ\u{9be}য়\u{9be}ন\u{9be}"),
            ("br", "Gwiana c'hall"),
            ("bs", "Francuska Gvajana"),
            ("ca", "Guaiana Francesa"),
            ("ce", "Французийн Гвиана"),
            ("ch", "French Guiana"),
            ("cs", "Francouzská Guayana"),
            ("cv", "Французийн Гвиана"),
            ("cy", "Guiana Ffrangeg"),
            ("da", "Fransk Guyana"),
            ("de", "Französisch-Guyana"),
            (
                "dv",
                "ފ\u{7a6}ރ\u{7a6}ނ\u{7b0}ސ\u{7ad}ސ\u{7a8} ގ\u{7a9}ނ\u{7a7}",
            ),
            ("dz", "ཕ\u{f7a}ར\u{f7a}ནཆ\u{f72}་ ག\u{f74}འ\u{f72}་ན།"),
            ("ee", "French Guiana"),
            ("el", "Γαλλική Γουιάνα"),
            ("en", "French Guiana"),
            ("eo", "Franca Gviano"),
            ("es", "Guayana Francesa"),
            ("et", "Prantsuse Guajaana"),
            ("eu", "Guyana Frantsesa"),
            ("fa", "گویان فرانسه"),
            ("ff", "French Guiana"),
            ("fi", "Ranskan Guayana"),
            ("fo", "French Guiana"),
            ("fr", "Guyane française"),
            ("fy", "Frânsk Guyana"),
            ("ga", "An Ghuáin Fhrancach"),
            ("gl", "Güiana Francesa"),
            ("gn", "French Guiana"),
            ("gu", "ફ\u{acd}ર\u{ac7}ન\u{acd}ચ ગ\u{ac1}એના"),
            ("gv", "French Guiana"),
            ("ha", "French Guiana"),
            ("he", "גיאנה הצרפתית"),
            ("hi", "फ\u{93c}\u{94d}रान\u{94d}सीसी ग\u{941}याना"),
            ("hr", "Francuska Gijana"),
            ("ht", "Giyàn franse"),
            ("hu", "Francia Guyana"),
            ("hy", "Ֆրանսիական Գվիանա"),
            ("ia", "Guiana Francese"),
            ("id", "Guyana Perancis"),
            ("io", "French Guiana"),
            ("is", "Franska Gvæjana"),
            ("it", "Guyana francese"),
            ("iu", "French Guiana"),
            ("ja", "仏領ギアナ"),
            ("ka", "ფრანგული გუიანა"),
            ("ki", "French Guyana"),
            ("kk", "Француз Гвианасы"),
            ("kl", "French Guiana"),
            (
                "km",
                "ហ\u{17d2}គ\u{17bc}អ\u{17ca}\u{17b8}យ\u{17c9}ាណា\u{200b}បារា\u{17c6}ង",
            ),
            ("kn", "ಫ\u{ccd}ರ\u{cc6}ಂಚ\u{ccd} ಗಯಾನಾ"),
            ("ko", "프랑스령 기아나"),
            ("ku", "Gîneya Fransî"),
            ("kv", "French Guiana"),
            ("kw", "Gwayana Frynkek"),
            ("ky", "Гвиана"),
            ("lo", "French Guiana"),
            ("lt", "Prancūzijos Gviana"),
            ("lv", "Gviāna"),
            ("mi", "Kaiana Wīwī"),
            ("mk", "Француска Гвинеја"),
            ("ml", "ഫ\u{d4d}രഞ\u{d4d}ച\u{d4d} ഗയ\u{d3e}ന"),
            ("mn", "Франц гана"),
            ("mr", "फ\u{94d}र\u{947}\u{902}च गियाना"),
            ("ms", "Guiana Perancis"),
            ("mt", "Gujana Franċiża"),
            ("my", "French Guiana"),
            ("na", "French Guiana"),
            ("nb", "Fransk Guyana"),
            ("ne", "फ\u{94d}र\u{947}न\u{94d}च जिनिया"),
            ("nl", "Frans-Guyana"),
            ("nn", "Fransk Guyana"),
            ("nv", "French Guiana"),
            ("oc", "Guaiana francesa"),
            ("or", "ଫ\u{b4d}ରେଞ\u{b4d}ଚ ଗ\u{b41}ଆନ\u{b3e}"),
            ("pa", "ਫਰ\u{a48}\u{a02}ਚ ਗ\u{a41}ਆਨਾ"),
            ("pi", "French Guiana"),
            ("pl", "Gujana Francuska"),
            ("ps", "French Guiana"),
            ("pt", "Guiana Francesa"),
            ("pt_BR", "Guiana Francesa"),
            ("ro", "Guiana Franceză"),
            ("ru", "Французская Гвиана"),
            ("rw", "Guyane Nyamfaransa"),
            ("sc", "Guyana Frantzesa"),
            ("sd", "French Guiana"),
            ("si", "ප\u{dca}\u{200d}රංශ ග\u{dd2}න\u{dd2}ය\u{dcf}ව"),
            ("sk", "Francúzska Guyana"),
            ("sl", "Francoska Gvajana"),
            ("so", "Faransiis Guyana"),
            ("sq", "Guajana Frënge"),
            ("sr", "Француска Гвајана"),
            ("sv", "Franska Guyana"),
            ("sw", "French Guiana"),
            ("ta", "ஃப\u{bcd}ரெஞ\u{bcd}ச\u{bcd} கைய\u{bbe}ன\u{bbe}"),
            ("te", "ఫ\u{c4d}ర\u{c46}ంచ\u{c4d} గ\u{c3f}య\u{c3e}న\u{c3e}"),
            ("tg", "Гвианаи Фаронса"),
            ("th", "เฟรนช\u{e4c}เก\u{e35}ยนา"),
            ("ti", "የፈረንሳይ ጉዊአና"),
            ("tk", "Fransuz Gwiana"),
            ("tl", "French Guiana"),
            ("tr", "Fransız Guyanası"),
            ("tt", "Франс Gуиана"),
            ("ug", "فىرانسىيەگە قاراشلىق گىۋىيانا"),
            ("uk", "Французька Гвіана"),
            ("ur", "فرانسیسی گیانا"),
            ("uz", "Fransiya Gvianasi"),
            ("ve", "French Guiana"),
            ("vi", "Ghi-a-na Pháp"),
            ("wa", "Guyane francesse"),
            ("wo", "Guyaana Faraañse"),
            ("xh", "French Guiana"),
            ("yo", "Gùyánà Fránsì"),
            ("zh_CN", "法属圭亚那"),
            ("zh_HK", "法屬圭亞那"),
            ("zh_TW", "法屬蓋亞那"),
            ("zu", "French Guiana"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
