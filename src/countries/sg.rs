// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Singapore

#[cfg(all(feature = "sg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SG;
    pub const ALPHA3: Alpha3 = Alpha3::SGP;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 65;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::SGD;
    pub const GEC: Option<GEC> = Some(GEC::SN);
    pub const INTERNATIONAL_PREFIX: &str = "001";
    pub const IOC: Option<IOC> = Some(IOC::SGP);
    pub const ISO_SHORT_NAME: &str = "Singapore";
    pub const ISO_LONG_NAME: &str = "The Republic of Singapore";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "ms", "ta"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "ms", "ta"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Singaporean");
    pub const NUMBER: &str = "702";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "SG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Singapore", "Singapur", "Singapour", "シンガポール"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Singapore"),
        ("af", "Singapoer"),
        ("ak", "Singapore"),
        ("am", "ሲንጒፖሴ"),
        ("an", "Singapore"),
        ("ar", "سنغافورة"),
        ("as", "ছিংগ\u{9be}প\u{9c1}ৰ"),
        ("ay", "Singapore"),
        ("az", "Sinqapur"),
        ("ba", "Singapore"),
        ("be", "Сінгапур"),
        ("bg", "Сингапур"),
        ("bi", "Singapore"),
        ("bn", "সিঙ\u{9cd}গ\u{9be}প\u{9c1}র"),
        ("bn_IN", "সিঙ\u{9cd}গ\u{9be}প\u{9c1}র"),
        ("br", "Singapour"),
        ("bs", "Singapur"),
        ("ca", "Singapur"),
        ("ce", "Сингапур"),
        ("ch", "Singapore"),
        ("cs", "Singapur"),
        ("cv", "Сингапур"),
        ("cy", "Singapore"),
        ("da", "Singapore"),
        ("de", "Singapur"),
        ("dv", "ސ\u{7a8}ނ\u{7b0}ގ\u{7a6}ޕ\u{7ab}ރ\u{7aa}"),
        ("dz", "ས\u{f72}ང་ག་པ\u{f7c}ར།"),
        ("ee", "Singapore"),
        ("el", "Σιγκαπούρη"),
        ("en", "Singapore"),
        ("eo", "Singapuro"),
        ("es", "Singapur"),
        ("et", "Singapur"),
        ("eu", "Singapur"),
        ("fa", "سنگاپور"),
        ("ff", "Sinngapuur"),
        ("fi", "Singapore"),
        ("fo", "Singapor"),
        ("fr", "Singapour"),
        ("fy", "Singapore"),
        ("ga", "Singeapór"),
        ("gl", "Singapur"),
        ("gn", "Singapore"),
        ("gu", "સિ\u{a82}ગાપ\u{ac1}ર"),
        ("gv", "Singapore"),
        ("ha", "Singapore"),
        ("he", "סינגפור"),
        ("hi", "सि\u{902}गाप\u{941}र"),
        ("hr", "Singapur"),
        ("ht", "Sengapou"),
        ("hu", "Szingapúr"),
        ("hy", "Սինգապուր"),
        ("ia", "Singapur"),
        ("id", "Singapura"),
        ("io", "Singapur"),
        ("is", "Singapúr"),
        ("it", "Singapore"),
        ("iu", "Singapore"),
        ("ja", "シンガポール"),
        ("ka", "სინგაპური"),
        ("ki", "Singapore"),
        ("kk", "Сингапур"),
        ("kl", "Singapore"),
        ("km", "សា\u{17c6}ងហ\u{17d2}គាព\u{17bd}រ"),
        ("kn", "ಸ\u{cbf}ಂಗಾಪುರ\u{ccd}"),
        ("ko", "싱가포르"),
        ("ku", "Sîngapur"),
        ("kv", "Сингапур"),
        ("kw", "Singapour"),
        ("ky", "Сингапур, мамлекет"),
        ("lo", "ປະເທດສ\u{eb4}ງກະໂປ"),
        ("lt", "Singapūras"),
        ("lv", "Singapūra"),
        ("mi", "Hingapoa"),
        ("mk", "Сингапур"),
        ("ml", "സിംഗപ\u{d4d}പ\u{d42}ര\u{d4d}\u{200d}"),
        ("mn", "Сингафур"),
        ("mr", "सि\u{902}गाप\u{942}र"),
        ("ms", "Singapura"),
        ("mt", "Singapor"),
        (
            "my",
            "စင\u{103a}ကာပ\u{1030}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tsingapoar"),
        ("nb", "Singapore"),
        ("ne", "सि\u{902}गाप\u{941}र"),
        ("nl", "Singapore"),
        ("nn", "Singapore"),
        ("nv", "Singapore"),
        ("oc", "Singapor"),
        ("or", "ସ\u{b3f}ଙ\u{b4d}ଗ\u{b3e}ପ\u{b41}ର"),
        ("pa", "ਸਿ\u{a70}ਘਾਪ\u{a41}ਰ"),
        ("pi", "सि\u{902}गापोर"),
        ("pl", "Singapur"),
        ("ps", "سينګاپور"),
        ("pt", "Singapura"),
        ("pt_BR", "Cingapura"),
        ("ro", "Singapore"),
        ("ru", "Сингапур"),
        ("rw", "Singapore"),
        ("sc", "Singapore"),
        ("sd", "سنگاپور"),
        ("si", "ස\u{dd2}ංගප\u{dca}ප\u{dd6}ර\u{dd4}ව"),
        ("sk", "Singapur"),
        ("sl", "Singapur"),
        ("so", "Singapore"),
        ("sq", "Singapor"),
        ("sr", "Сингапур"),
        ("sv", "Singapore"),
        ("sw", "Singapore"),
        ("ta", "சிங\u{bcd}கப\u{bcd}பூர\u{bcd}"),
        ("te", "స\u{c3f}ంగపూర\u{c4d}"),
        ("tg", "Сингапур"),
        ("th", "ส\u{e34}งคโปร\u{e4c}"),
        ("ti", "ሲንጋፖር"),
        ("tk", "Singapur"),
        ("tl", "Singapore"),
        ("tr", "Singapur"),
        ("tt", "Синgапур"),
        ("ug", "سىنگاپور"),
        ("uk", "Сінгапур"),
        ("ur", "سنگاپور"),
        ("uz", "Singapur"),
        ("ve", "Singapore"),
        ("vi", "Xin-ga-po"),
        ("wa", "Singapour"),
        ("wo", "Singapoor"),
        ("xh", "Singapore"),
        ("yo", "Singapore"),
        ("zh_CN", "新加坡"),
        ("zh_HK", "新加坡"),
        ("zh_TW", "新加坡"),
        ("zu", "Singapore"),
    ];
    #[cfg(all(feature = "sg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 1.352083;
        pub const LONGITUDE: f64 = 103.819836;
        pub const MAX_LATITUDE: f64 = 1.4784001;
        pub const MAX_LONGITUDE: f64 = 104.0945001;
        pub const MIN_LATITUDE: f64 = 1.1496;
        pub const MIN_LONGITUDE: f64 = 103.594;
        pub const NORTHEAST_LATITUDE: f64 = 1.4784001;
        pub const NORTHEAST_LONGITUDE: f64 = 104.0945001;
        pub const SOUTHWEST_LATITUDE: f64 = 1.1496;
        pub const SOUTHWEST_LONGITUDE: f64 = 103.594;
    }
}
#[cfg(all(feature = "sg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 1.352083,
            longitude: 103.819836,
            max_latitude: 1.4784001,
            max_longitude: 104.0945001,
            min_latitude: 1.1496,
            min_longitude: 103.594,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 1.4784001,
                    longitude: 104.0945001,
                },
                southwest: CountryGeoBound {
                    latitude: 1.1496,
                    longitude: 103.594,
                },
            },
        }
    }
}

#[cfg(all(feature = "sg", feature = "subdivisions"))]
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from(
            [

                (
                    "01",
                    Subdivision{
                        name: "Central Singapore",
                        country_alpha2: Alpha2::SG,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.3418329), longitude: Some(103.8608764), max_latitude: Some(1.3948174), min_latitude: Some(1.2774146), max_longitude: Some(103.86428), min_longitude: Some(103.8230134)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄥\u{11128}\u{11101}𑄉𑄛\u{1112a}𑄢\u{11134}"), ("ceb", "Central Singapore Community Development Council"), ("de", "Central Singapore District"), ("en", "Central Singapore"), ("es", "Consejo Central (Singapur)"), ("fr", "Singapour central"), ("ja", "シンガポール中央地区"), ("ko", "중앙싱가포르 지구 사회 발전 이사회"), ("ur", "مرکزی سنگاپور کمیونٹی ڈیولپمنٹ کونسل"), ("zh", "中区社区发展理事会")]),
                        unofficial_name_list: ["Central Singapore"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "North East",
                        country_alpha2: Alpha2::SG,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.352083), longitude: Some(103.819836), max_latitude: Some(1.4707592), min_latitude: Some(1.1587023), max_longitude: Some(104.0884808), min_longitude: Some(103.6055448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{1112a}𑄇\u{11134}"), ("ceb", "North East Community Development Region"), ("en", "North East"), ("es", "Consejo del Noreste (Singapur)"), ("fr", "District du Nord-Est"), ("it", "Distretto Nordorientale"), ("ja", "北東地区 (シンガポール)"), ("ko", "북동부 지구 사회 발전 이사회"), ("nl", "North East Community Development Council"), ("ur", "شمال مشرقی کمیونٹی ڈیولپمنٹ کونسل"), ("zh", "东北社区发展理事会")]),
                        unofficial_name_list: ["North East"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "North West",
                        country_alpha2: Alpha2::SG,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.3537604), longitude: Some(103.7107653), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}"), ("ceb", "North West Community Development Council"), ("en", "North West"), ("es", "Consejo del Noroeste (Singapur)"), ("fr", "District du Nord-Ouest"), ("ja", "北西地区 (シンガポール)"), ("ko", "북서부 지구 사회 발전 이사회"), ("ur", "شمال مغربی کمیونٹی ڈیولپمنٹ کونسل"), ("zh", "西北社区发展理事会")]),
                        unofficial_name_list: ["North West"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "South East",
                        country_alpha2: Alpha2::SG,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.352083), longitude: Some(103.819836), max_latitude: Some(1.4707592), min_latitude: Some(1.1587023), max_longitude: Some(104.0884808), min_longitude: Some(103.6055448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄛\u{1112a}𑄇\u{11134}"), ("ceb", "South East Community Development Council"), ("de", "Südost"), ("en", "South East"), ("es", "Consejo del Sudeste (Singapur)"), ("fr", "District du Sud-Est"), ("ja", "南東地区"), ("ko", "남동부 지구 사회 발전 이사회"), ("ur", "جنوب مشرقی کمیونٹی ڈیولپمنٹ کونسل"), ("zh", "东南社区发展理事会")]),
                        unofficial_name_list: ["South East"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "South West",
                        country_alpha2: Alpha2::SG,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.352083), longitude: Some(103.819836), max_latitude: Some(1.4707592), min_latitude: Some(1.1587023), max_longitude: Some(104.0884808), min_longitude: Some(103.6055448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}"), ("ceb", "South West Community Development Council"), ("de", "South West Singapore District"), ("en", "South West"), ("es", "Consejo del Sudoeste (Singapur)"), ("fr", "District du Sud-Ouest"), ("ja", "南西地区 (シンガポール)"), ("ko", "남서부 지구 사회 발전 이사회"), ("ur", "جنوب مغربی کمیونٹی ڈیولپمنٹ کونسل"), ("zh", "西南社区发展理事会")]),
                        unofficial_name_list: ["South West"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "sg")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SG,
        alpha3: Alpha3::SGP,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}} {{postalcode}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 65,
        currency_code: CurrencyCode::SGD,
        gec: Some(GEC::SN),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "001",
        ioc: Some(IOC::SGP),
        iso_long_name: "The Republic of Singapore",
        iso_short_name: "Singapore",
        official_language_list: ["en", "ms", "ta"].to_vec(),
        spoken_language_list: ["en", "ms", "ta"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "None",
        nationality: Some("Singaporean"),
        number: "702",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "SG",
        unofficial_name_list: ["Singapore", "Singapur", "Singapour", "シンガポール"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Singapore"),
            ("af", "Singapoer"),
            ("ak", "Singapore"),
            ("am", "ሲንጒፖሴ"),
            ("an", "Singapore"),
            ("ar", "سنغافورة"),
            ("as", "ছিংগ\u{9be}প\u{9c1}ৰ"),
            ("ay", "Singapore"),
            ("az", "Sinqapur"),
            ("ba", "Singapore"),
            ("be", "Сінгапур"),
            ("bg", "Сингапур"),
            ("bi", "Singapore"),
            ("bn", "সিঙ\u{9cd}গ\u{9be}প\u{9c1}র"),
            ("bn_IN", "সিঙ\u{9cd}গ\u{9be}প\u{9c1}র"),
            ("br", "Singapour"),
            ("bs", "Singapur"),
            ("ca", "Singapur"),
            ("ce", "Сингапур"),
            ("ch", "Singapore"),
            ("cs", "Singapur"),
            ("cv", "Сингапур"),
            ("cy", "Singapore"),
            ("da", "Singapore"),
            ("de", "Singapur"),
            ("dv", "ސ\u{7a8}ނ\u{7b0}ގ\u{7a6}ޕ\u{7ab}ރ\u{7aa}"),
            ("dz", "ས\u{f72}ང་ག་པ\u{f7c}ར།"),
            ("ee", "Singapore"),
            ("el", "Σιγκαπούρη"),
            ("en", "Singapore"),
            ("eo", "Singapuro"),
            ("es", "Singapur"),
            ("et", "Singapur"),
            ("eu", "Singapur"),
            ("fa", "سنگاپور"),
            ("ff", "Sinngapuur"),
            ("fi", "Singapore"),
            ("fo", "Singapor"),
            ("fr", "Singapour"),
            ("fy", "Singapore"),
            ("ga", "Singeapór"),
            ("gl", "Singapur"),
            ("gn", "Singapore"),
            ("gu", "સિ\u{a82}ગાપ\u{ac1}ર"),
            ("gv", "Singapore"),
            ("ha", "Singapore"),
            ("he", "סינגפור"),
            ("hi", "सि\u{902}गाप\u{941}र"),
            ("hr", "Singapur"),
            ("ht", "Sengapou"),
            ("hu", "Szingapúr"),
            ("hy", "Սինգապուր"),
            ("ia", "Singapur"),
            ("id", "Singapura"),
            ("io", "Singapur"),
            ("is", "Singapúr"),
            ("it", "Singapore"),
            ("iu", "Singapore"),
            ("ja", "シンガポール"),
            ("ka", "სინგაპური"),
            ("ki", "Singapore"),
            ("kk", "Сингапур"),
            ("kl", "Singapore"),
            ("km", "សា\u{17c6}ងហ\u{17d2}គាព\u{17bd}រ"),
            ("kn", "ಸ\u{cbf}ಂಗಾಪುರ\u{ccd}"),
            ("ko", "싱가포르"),
            ("ku", "Sîngapur"),
            ("kv", "Сингапур"),
            ("kw", "Singapour"),
            ("ky", "Сингапур, мамлекет"),
            ("lo", "ປະເທດສ\u{eb4}ງກະໂປ"),
            ("lt", "Singapūras"),
            ("lv", "Singapūra"),
            ("mi", "Hingapoa"),
            ("mk", "Сингапур"),
            ("ml", "സിംഗപ\u{d4d}പ\u{d42}ര\u{d4d}\u{200d}"),
            ("mn", "Сингафур"),
            ("mr", "सि\u{902}गाप\u{942}र"),
            ("ms", "Singapura"),
            ("mt", "Singapor"),
            (
                "my",
                "စင\u{103a}ကာပ\u{1030}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tsingapoar"),
            ("nb", "Singapore"),
            ("ne", "सि\u{902}गाप\u{941}र"),
            ("nl", "Singapore"),
            ("nn", "Singapore"),
            ("nv", "Singapore"),
            ("oc", "Singapor"),
            ("or", "ସ\u{b3f}ଙ\u{b4d}ଗ\u{b3e}ପ\u{b41}ର"),
            ("pa", "ਸਿ\u{a70}ਘਾਪ\u{a41}ਰ"),
            ("pi", "सि\u{902}गापोर"),
            ("pl", "Singapur"),
            ("ps", "سينګاپور"),
            ("pt", "Singapura"),
            ("pt_BR", "Cingapura"),
            ("ro", "Singapore"),
            ("ru", "Сингапур"),
            ("rw", "Singapore"),
            ("sc", "Singapore"),
            ("sd", "سنگاپور"),
            ("si", "ස\u{dd2}ංගප\u{dca}ප\u{dd6}ර\u{dd4}ව"),
            ("sk", "Singapur"),
            ("sl", "Singapur"),
            ("so", "Singapore"),
            ("sq", "Singapor"),
            ("sr", "Сингапур"),
            ("sv", "Singapore"),
            ("sw", "Singapore"),
            ("ta", "சிங\u{bcd}கப\u{bcd}பூர\u{bcd}"),
            ("te", "స\u{c3f}ంగపూర\u{c4d}"),
            ("tg", "Сингапур"),
            ("th", "ส\u{e34}งคโปร\u{e4c}"),
            ("ti", "ሲንጋፖር"),
            ("tk", "Singapur"),
            ("tl", "Singapore"),
            ("tr", "Singapur"),
            ("tt", "Синgапур"),
            ("ug", "سىنگاپور"),
            ("uk", "Сінгапур"),
            ("ur", "سنگاپور"),
            ("uz", "Singapur"),
            ("ve", "Singapore"),
            ("vi", "Xin-ga-po"),
            ("wa", "Singapour"),
            ("wo", "Singapoor"),
            ("xh", "Singapore"),
            ("yo", "Singapore"),
            ("zh_CN", "新加坡"),
            ("zh_HK", "新加坡"),
            ("zh_TW", "新加坡"),
            ("zu", "Singapore"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
