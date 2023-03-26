// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Faroe Islands

#[cfg(all(feature = "fo", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::FO;
    pub const ALPHA3: Alpha3 = Alpha3::FRO;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 298;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::DKK;
    pub const GEC: Option<GEC> = Some(GEC::FO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::FRO);
    pub const ISO_SHORT_NAME: &str = "Faroe Islands";
    pub const ISO_LONG_NAME: &str = "The Faroe Islands";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fo"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fo"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Faroese");
    pub const NUMBER: &str = "234";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "FO";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Faroe Islands",
        "Färöer-Inseln",
        "Îles Féroé",
        "Islas Faroe",
        "フェロー諸島",
        "Faeröer",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Faroe Islands"),
        ("af", "Faroëreilande"),
        ("ak", "Faroe Islands"),
        ("am", "Faroe Islands"),
        ("an", "Faroe Islands"),
        ("ar", "جزر الفارو"),
        ("as", "ফ\u{9be}ৰো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
        ("ay", "Faroe Islands"),
        ("az", "Faro Adaları"),
        ("ba", "Faroe Islands"),
        ("be", "Фарэрскія астравы"),
        ("bg", "Острови Фаро"),
        ("bi", "Faroe Islands"),
        ("bn", "ফ\u{9cd}য\u{9be}রো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
        ("bn_IN", "ফ\u{9cd}য\u{9be}রো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
        ("br", "Faero"),
        ("bs", "Farska ostrva"),
        ("ca", "Illes Fèroe"),
        ("ce", "Фарерийн гІайренаш"),
        ("ch", "Faroe Islands"),
        ("cs", "Faerské ostrovy"),
        ("cv", "Фарерийн гІайренаш"),
        ("cy", "Ynysoedd y Ffaro"),
        ("da", "Færøerne"),
        ("de", "Färöer-Inseln"),
        ("dv", "ފ\u{7a6}ރ\u{7ae}އ\u{7ad} ޖ\u{7a6}ޒ\u{7a9}ރ\u{7a7}"),
        (
            "dz",
            "ཕ་ར\u{f7c}འ\u{f72}་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།",
        ),
        ("ee", "Faroe Islands"),
        ("el", "Νησιά Φερόε"),
        ("en", "Faroe Islands"),
        ("eo", "Ferooj"),
        ("es", "Islas Feroe"),
        ("et", "Fääri saared"),
        ("eu", "Faroe uharteak"),
        ("fa", "جزایر فارو"),
        ("ff", "Faroe Islands"),
        ("fi", "Färsaaret"),
        ("fo", "Føroyar"),
        ("fr", "îles Féroé"),
        ("fy", "Faeröer"),
        ("ga", "Oileáin Fharó"),
        ("gl", "Illas Feroe"),
        ("gn", "Faroe Islands"),
        ("gu", "ફારો ટાપ\u{ac1}ઓ"),
        ("gv", "Ellanyn ny Geyrragh"),
        ("ha", "Faroe Islands"),
        ("he", "איי פארו"),
        ("hi", "फ\u{93c}रो द\u{94d}वीपसम\u{942}ह"),
        ("hr", "Farski otoci"),
        ("ht", "Faroe Islands"),
        ("hu", "Feröer"),
        ("hy", "Ֆարերյան կղզիներ"),
        ("ia", "Insulas Feroe"),
        ("id", "Kepulauan Faroe"),
        ("io", "Faero"),
        ("is", "Færeyjar"),
        ("it", "Isole Fær Øer"),
        ("iu", "Faroe Islands"),
        ("ja", "フェロー諸島"),
        ("ka", "ფაროეს კუნძულები"),
        ("ki", "Faroe Islands"),
        ("kk", "Фарер аралдары"),
        ("kl", "Faroe Islands"),
        ("km", "កោះ\u{200b}ហ\u{17d2}វារ\u{17c9}\u{17bc}"),
        ("kn", "ಪರೋ ದ\u{ccd}ವೀಪಗಳು"),
        ("ko", "페로 제도"),
        ("ku", "Giravên Faroye"),
        ("kv", "Фарер діяс"),
        ("kw", "Ynysow Faroe"),
        ("ky", "Фарер аралдары"),
        ("lo", "Faroe Islands"),
        ("lt", "Farerų salos"),
        ("lv", "Fēru salas"),
        ("mi", "Moutere Faroe"),
        ("mk", "Фаројски острови"),
        ("ml", "ഫറവോ ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}"),
        ("mn", "Фарерын арлууд"),
        ("mr", "फ\u{947}रो आयल\u{945}\u{902}डस\u{94d}"),
        ("ms", "Kepulauan Faroe"),
        ("mt", "Gżejjer Faroe"),
        ("my", "Faroe Islands"),
        ("na", "Faroe Islands"),
        ("nb", "Færøyene"),
        ("ne", "फारो टाप\u{941}"),
        ("nl", "Faeröer"),
        ("nn", "Færøyane"),
        ("nv", "Faroe Islands"),
        ("oc", "Illas Feròe"),
        ("or", "ଫ\u{b3e}ରୋ ଦ\u{b4d}ବୀପପ\u{b41}ଞ\u{b4d}ଜ"),
        ("pa", "ਫਾਰੀਓ ਟਾਪ\u{a42}"),
        ("pi", "Faroe Islands"),
        ("pl", "Wyspy Owcze"),
        ("ps", "Faroe Islands"),
        ("pt", "Ilhas Faroé"),
        ("pt_BR", "Ilhas Faroe"),
        ("ro", "Insulele Feroe"),
        ("ru", "Фарерские острова"),
        ("rw", "Ibirwa bya Farowe"),
        ("sc", "Ìsulas Føroyar"),
        ("sd", "Faroe Islands"),
        ("si", "ෆ\u{dcf}රෝ ද\u{dd6}පත\u{dca}"),
        ("sk", "Faerské ostrovy"),
        ("sl", "Ferski otoki"),
        ("so", "Jasiiradaha Feroe"),
        ("sq", "Ishujt Faroe"),
        ("sr", "Фарска острва"),
        ("sv", "Färöarna"),
        ("sw", "Faroe Islands"),
        ("ta", "பேரோ த\u{bc0}வுகள\u{bcd}"),
        ("te", "ఫ\u{c47}ర\u{c4b} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}"),
        ("tg", "Ҷазираҳои Форуй"),
        ("th", "หม\u{e39}\u{e48}เกาะแฟโร"),
        ("ti", "Faroe Islands"),
        ("tk", "Farer adalary"),
        ("tl", "Faroe Islands"),
        ("tr", "Faroe Adaları"),
        ("tt", "Фарое Утравлары"),
        ("ug", "فائېرو تاقىم ئاراللىرى (دانىيە)"),
        ("uk", "Фарерські острови"),
        ("ur", "جزائرفارو"),
        ("uz", "Farer orollari"),
        ("ve", "Faroe Islands"),
        ("vi", "Quần đảo Pha-rô"),
        ("wa", "Iyes Faeroyé"),
        ("wo", "Iil yu Faarow"),
        ("xh", "Faroe Islands"),
        ("yo", "Àwọn Erékùṣù Fàróè"),
        ("zh_CN", "法罗群岛"),
        ("zh_HK", "法羅羣島"),
        ("zh_TW", "法羅群島"),
        ("zu", "Faroe Islands"),
    ];
    #[cfg(all(feature = "fo", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 61.89263500000001;
        pub const LONGITUDE: f64 = -6.9118061;
        pub const MAX_LATITUDE: f64 = 62.4310742;
        pub const MAX_LONGITUDE: f64 = -6.190796;
        pub const MIN_LATITUDE: f64 = 61.3677776;
        pub const MIN_LONGITUDE: f64 = -7.7178956;
        pub const NORTHEAST_LATITUDE: f64 = 62.4310742;
        pub const NORTHEAST_LONGITUDE: f64 = -6.190796;
        pub const SOUTHWEST_LATITUDE: f64 = 61.3677776;
        pub const SOUTHWEST_LONGITUDE: f64 = -7.7178956;
    }
}
#[cfg(all(feature = "fo", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 61.89263500000001,
            longitude: -6.9118061,
            max_latitude: 62.4310742,
            max_longitude: -6.190796,
            min_latitude: 61.3677776,
            min_longitude: -7.7178956,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 62.4310742,
                    longitude: -6.190796,
                },
                southwest: CountryGeoBound {
                    latitude: 61.3677776,
                    longitude: -7.7178956,
                },
            },
        }
    }
}

#[cfg(all(feature = "fo", feature = "subdivisions"))]
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
#[cfg(feature = "fo")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::FO,
        alpha3: Alpha3::FRO,
        address_format: None,
        continent: Continent::Europe,
        country_code: 298,
        currency_code: CurrencyCode::DKK,
        gec: Some(GEC::FO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::FRO),
        iso_long_name: "The Faroe Islands",
        iso_short_name: "Faroe Islands",
        official_language_list: ["fo"].to_vec(),
        spoken_language_list: ["fo"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [6].to_vec(),
        national_prefix: "None",
        nationality: Some("Faroese"),
        number: "234",
        postal_code: true,
        postal_code_format: Some("\\d{3}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "FO",
        unofficial_name_list: [
            "Faroe Islands",
            "Färöer-Inseln",
            "Îles Féroé",
            "Islas Faroe",
            "フェロー諸島",
            "Faeröer",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Faroe Islands"),
            ("af", "Faroëreilande"),
            ("ak", "Faroe Islands"),
            ("am", "Faroe Islands"),
            ("an", "Faroe Islands"),
            ("ar", "جزر الفارو"),
            ("as", "ফ\u{9be}ৰো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
            ("ay", "Faroe Islands"),
            ("az", "Faro Adaları"),
            ("ba", "Faroe Islands"),
            ("be", "Фарэрскія астравы"),
            ("bg", "Острови Фаро"),
            ("bi", "Faroe Islands"),
            ("bn", "ফ\u{9cd}য\u{9be}রো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
            ("bn_IN", "ফ\u{9cd}য\u{9be}রো দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
            ("br", "Faero"),
            ("bs", "Farska ostrva"),
            ("ca", "Illes Fèroe"),
            ("ce", "Фарерийн гІайренаш"),
            ("ch", "Faroe Islands"),
            ("cs", "Faerské ostrovy"),
            ("cv", "Фарерийн гІайренаш"),
            ("cy", "Ynysoedd y Ffaro"),
            ("da", "Færøerne"),
            ("de", "Färöer-Inseln"),
            ("dv", "ފ\u{7a6}ރ\u{7ae}އ\u{7ad} ޖ\u{7a6}ޒ\u{7a9}ރ\u{7a7}"),
            (
                "dz",
                "ཕ་ར\u{f7c}འ\u{f72}་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།",
            ),
            ("ee", "Faroe Islands"),
            ("el", "Νησιά Φερόε"),
            ("en", "Faroe Islands"),
            ("eo", "Ferooj"),
            ("es", "Islas Feroe"),
            ("et", "Fääri saared"),
            ("eu", "Faroe uharteak"),
            ("fa", "جزایر فارو"),
            ("ff", "Faroe Islands"),
            ("fi", "Färsaaret"),
            ("fo", "Føroyar"),
            ("fr", "îles Féroé"),
            ("fy", "Faeröer"),
            ("ga", "Oileáin Fharó"),
            ("gl", "Illas Feroe"),
            ("gn", "Faroe Islands"),
            ("gu", "ફારો ટાપ\u{ac1}ઓ"),
            ("gv", "Ellanyn ny Geyrragh"),
            ("ha", "Faroe Islands"),
            ("he", "איי פארו"),
            ("hi", "फ\u{93c}रो द\u{94d}वीपसम\u{942}ह"),
            ("hr", "Farski otoci"),
            ("ht", "Faroe Islands"),
            ("hu", "Feröer"),
            ("hy", "Ֆարերյան կղզիներ"),
            ("ia", "Insulas Feroe"),
            ("id", "Kepulauan Faroe"),
            ("io", "Faero"),
            ("is", "Færeyjar"),
            ("it", "Isole Fær Øer"),
            ("iu", "Faroe Islands"),
            ("ja", "フェロー諸島"),
            ("ka", "ფაროეს კუნძულები"),
            ("ki", "Faroe Islands"),
            ("kk", "Фарер аралдары"),
            ("kl", "Faroe Islands"),
            ("km", "កោះ\u{200b}ហ\u{17d2}វារ\u{17c9}\u{17bc}"),
            ("kn", "ಪರೋ ದ\u{ccd}ವೀಪಗಳು"),
            ("ko", "페로 제도"),
            ("ku", "Giravên Faroye"),
            ("kv", "Фарер діяс"),
            ("kw", "Ynysow Faroe"),
            ("ky", "Фарер аралдары"),
            ("lo", "Faroe Islands"),
            ("lt", "Farerų salos"),
            ("lv", "Fēru salas"),
            ("mi", "Moutere Faroe"),
            ("mk", "Фаројски острови"),
            ("ml", "ഫറവോ ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}"),
            ("mn", "Фарерын арлууд"),
            ("mr", "फ\u{947}रो आयल\u{945}\u{902}डस\u{94d}"),
            ("ms", "Kepulauan Faroe"),
            ("mt", "Gżejjer Faroe"),
            ("my", "Faroe Islands"),
            ("na", "Faroe Islands"),
            ("nb", "Færøyene"),
            ("ne", "फारो टाप\u{941}"),
            ("nl", "Faeröer"),
            ("nn", "Færøyane"),
            ("nv", "Faroe Islands"),
            ("oc", "Illas Feròe"),
            ("or", "ଫ\u{b3e}ରୋ ଦ\u{b4d}ବୀପପ\u{b41}ଞ\u{b4d}ଜ"),
            ("pa", "ਫਾਰੀਓ ਟਾਪ\u{a42}"),
            ("pi", "Faroe Islands"),
            ("pl", "Wyspy Owcze"),
            ("ps", "Faroe Islands"),
            ("pt", "Ilhas Faroé"),
            ("pt_BR", "Ilhas Faroe"),
            ("ro", "Insulele Feroe"),
            ("ru", "Фарерские острова"),
            ("rw", "Ibirwa bya Farowe"),
            ("sc", "Ìsulas Føroyar"),
            ("sd", "Faroe Islands"),
            ("si", "ෆ\u{dcf}රෝ ද\u{dd6}පත\u{dca}"),
            ("sk", "Faerské ostrovy"),
            ("sl", "Ferski otoki"),
            ("so", "Jasiiradaha Feroe"),
            ("sq", "Ishujt Faroe"),
            ("sr", "Фарска острва"),
            ("sv", "Färöarna"),
            ("sw", "Faroe Islands"),
            ("ta", "பேரோ த\u{bc0}வுகள\u{bcd}"),
            ("te", "ఫ\u{c47}ర\u{c4b} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}"),
            ("tg", "Ҷазираҳои Форуй"),
            ("th", "หม\u{e39}\u{e48}เกาะแฟโร"),
            ("ti", "Faroe Islands"),
            ("tk", "Farer adalary"),
            ("tl", "Faroe Islands"),
            ("tr", "Faroe Adaları"),
            ("tt", "Фарое Утравлары"),
            ("ug", "فائېرو تاقىم ئاراللىرى (دانىيە)"),
            ("uk", "Фарерські острови"),
            ("ur", "جزائرفارو"),
            ("uz", "Farer orollari"),
            ("ve", "Faroe Islands"),
            ("vi", "Quần đảo Pha-rô"),
            ("wa", "Iyes Faeroyé"),
            ("wo", "Iil yu Faarow"),
            ("xh", "Faroe Islands"),
            ("yo", "Àwọn Erékùṣù Fàróè"),
            ("zh_CN", "法罗群岛"),
            ("zh_HK", "法羅羣島"),
            ("zh_TW", "法羅群島"),
            ("zu", "Faroe Islands"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
