// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Falkland Islands

#[cfg(all(feature = "fk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::FK;
    pub const ALPHA3: Alpha3 = Alpha3::FLK;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 500;
    pub const CURRENCY_CODE: &str = "FKP";
    pub const GEC: Option<GEC> = Some(GEC::FK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = None;
    pub const ISO_SHORT_NAME: &str = "Falkland Islands (Malvinas)";
    pub const ISO_LONG_NAME: &str = "The Falkland Islands";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[5];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Falkland Islander");
    pub const NUMBER: &str = "238";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("FIQQ 1ZZ");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "FK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Falkland Islands",
        "Falklandinseln",
        "Îles Malouines",
        "Islas Malvinas",
        "フォークランド（マルビナス）諸島",
        "Falklandeilanden [Islas Malvinas]",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Falkland Islands (Malvinas)"),
        ("af", "Falklandeilande (Malvinas)"),
        ("ak", "Falkland Islands (Malvinas)"),
        ("am", "Falkland Islands (Malvinas)"),
        ("an", "Falkland Islands (Malvinas)"),
        ("ar", "جزر فولكلاند (مالفيناس)"),
        ("as", "ফ\u{9be}কলেণ\u{9cd}ড দ\u{9cd}বীপ (ম\u{9be}ল\u{9cd}\u{200c}ভিন\u{9be}ছ)"),
        ("ay", "Falkland Islands (Malvinas)"),
        ("az", "Fokland Adaları (Malvin)"),
        ("ba", "Falkland Islands (Malvinas)"),
        ("be", "Фалклендскія астравы (Мальвіны)"),
        ("bg", "Фолклендски Острови (Малвини)"),
        ("bi", "Falkland Islands (Malvinas)"),
        ("bn", "ফল\u{9cd}কল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ (ম\u{9be}লভিন\u{9be}স)"),
        ("bn_IN", "ফল\u{9cd}কল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ (ম\u{9be}লভিন\u{9be}স)"),
        ("br", "Falkland Islands (Malvinas)"),
        ("bs", "Falklandsko otočje"),
        ("ca", "Illes Malvines (Falkland)"),
        ("ce", "Falkland Islands (Malvinas)"),
        ("ch", "Falkland Islands (Malvinas)"),
        ("cs", "Falkandské ostrovy (Malvíny)"),
        ("cv", "Falkland Islands (Malvinas)"),
        ("cy", "Ynysoedd y Falklands (Malvinas)"),
        ("da", "Falklandsøerne (Malvinas)"),
        ("de", "Falklandinseln (Malwinen)"),
        ("dv", "Falkland Islands (Malvinas)"),
        ("dz", "ཕ\u{f71}ལཀ་ལ\u{f7a}ནཌ\u{f72}་ མཚ\u{f7c}་ག\u{fb3}\u{f72}ང་ (མ\u{f71}ལ་ཝ\u{f72}་ན\u{f71}ས\u{f72})"),
        ("ee", "Falkland Islands (Malvinas)"),
        ("el", "Νήσοι Φώκλαντ (Μαλβίνες)"),
        ("en", "Falkland Islands (Malvinas)"),
        ("eo", "Falklandoj (Malvinoj)"),
        ("es", "Islas Falkland (Malvinas)"),
        ("et", "Falklandi saared"),
        ("eu", "Falkland uharteak (Malvina uharteak)"),
        ("fa", "جزایر فالکلند(مالویناس)"),
        ("ff", "Falkland Islands (Malvinas)"),
        ("fi", "Falklandinsaaret"),
        ("fo", "Falkland Islands (Malvinas)"),
        ("fr", "Malouines, Îles (Falkland)"),
        ("fy", "Falkland Islands (Malvinas)"),
        ("ga", "Oileáin Fháclainne (Malvinas)"),
        ("gl", "Illas Falkland (Malvinas)"),
        ("gn", "Falkland Islands (Malvinas)"),
        ("gu", "ફોકલ\u{ac7}ન\u{acd}ડ ટાપ\u{ac1}ઓ (માલ\u{acd}વિનસ)"),
        ("gv", "Falkland Islands (Malvinas)"),
        ("ha", "Falkland Islands (Malvinas)"),
        ("he", "איי פוקלנד"),
        ("hi", "फॉकल\u{948}\u{902}ड आइल\u{948}\u{902}ड\u{94d}स (मालविनास)"),
        ("hr", "Falklandski otoci"),
        ("ht", "Falkland Islands (Malvinas)"),
        ("hu", "Falkland-szigetek (Malvinas)"),
        ("hy", "Ֆոլկլոնդյան Կղզիներ"),
        ("ia", "Insulas Falkland (Malvinas)"),
        ("id", "Kepulauan Falkland (Malvinas)"),
        ("io", "Falkland Islands (Malvinas)"),
        ("is", "Falklandseyjar (Malvinaeyjar)"),
        ("it", "Isole Falkland (Malvine)"),
        ("iu", "Falkland Islands (Malvinas)"),
        ("ja", "フォークランド諸島 (マルビナス)"),
        ("ka", "ფოლკლენდის კუნძულები"),
        ("ki", "Falkland Islands (Malvinas)"),
        ("kk", "Фолкленд (Мальвин) аралдары"),
        ("kl", "Falkland Islands (Malvinas)"),
        ("km", "កោះ\u{200b}ហ\u{17d2}វ\u{17c9}កឡង\u{17cb} (ម\u{17c9}ាល\u{17cb}វ\u{17b8}ណា)"),
        ("kn", "ಫಾಕ\u{ccd}\u{200d}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ದ\u{ccd}ವೀಪಗಳು(ಮಾಲ\u{ccd}ವ\u{cbf}ನಾಸ\u{ccd})"),
        ("ko", "포클랜드 제도 (말비나스)"),
        ("ku", "Giravên Falkland (Malvin)"),
        ("kv", "Falkland Islands (Malvinas)"),
        ("kw", "Falkland Islands (Malvinas)"),
        ("ky", "Фолкленд (Мальвин) аралдары"),
        ("lo", "Falkland Islands (Malvinas)"),
        ("lt", "Folklando (Malvinų) salos"),
        ("lv", "Folklenda (Malvinu) salas"),
        ("mi", "Falkland Islands (Malvinas)"),
        ("mk", "Фокландски острови (Малвини)"),
        ("ml", "ഫ\u{d3e}ക\u{d4d}\u{200d}ല\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d} (മ\u{d3e}ള\u{d4d}\u{200d}വിന\u{d3e}സ\u{d4d})"),
        ("mn", "Falkland Islands (Malvinas)"),
        ("mr", "फाल\u{94d}कन आयल\u{945}\u{902}डस\u{94d}(माल\u{94d}विनास)"),
        ("ms", "Kepulauan Falkland (Malvinas)"),
        ("mt", "Falkland Islands (Malvinas)"),
        ("my", "Falkland Islands (Malvinas)"),
        ("na", "Falkland Islands (Malvinas)"),
        ("nb", "Falklandsøyene"),
        ("ne", "फकल\u{94d}याण\u{94d}ड टाप\u{941} (Malvinas)"),
        ("nl", "Falklandeilanden"),
        ("nn", "Falklandsøyane"),
        ("nv", "Falkland Islands (Malvinas)"),
        ("oc", "Illas Falkand (Malvinas)"),
        ("or", "ଫ\u{b3e}ଲ\u{b4d}କଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ ଦ\u{b4d}ବୀପ(ମ\u{b3e}ଲ\u{b4d}ଭ\u{b3f}ନ\u{b3e}ସ)"),
        ("pa", "ਫਾਕਲ\u{a48}\u{a02}ਡ ਟਾਪ\u{a42} (ਮਾਲਵੀਨਸ)"),
        ("pi", "Falkland Islands (Malvinas)"),
        ("pl", "Falklandy (Malwiny)"),
        ("ps", "Falkland Islands (Malvinas)"),
        ("pt", "Ilhas Falkland (Malvinas)"),
        ("pt_BR", "Ilhas Malvinas (Falkland)"),
        ("ro", "Insulele Falkland (Insulele Malvine)"),
        ("ru", "Фолклендские (Мальвинские) острова"),
        ("rw", "Ibirwa bya Folukilande (Maluvinasi)"),
        ("sc", "Ìsulas Falkland (Malvinas)"),
        ("sd", "Falkland Islands (Malvinas)"),
        ("si", "ෆෝක\u{dca}ලන\u{dca}ඩ\u{dca} ද\u{dd6}පත\u{dca} (මැල\u{dca}ව\u{dd2}න\u{dcf})"),
        ("sk", "Falklandy (Malvíny)"),
        ("sl", "Falklandski otoki"),
        ("so", "Falkland Islands (Malvinas)"),
        ("sq", "Ishujt Falkland (Malvinas)"),
        ("sr", "Фолкландска острва (Малвини)"),
        ("sv", "Falklandsöarna (Malvinas)"),
        ("sw", "Falkland Islands (Malvinas)"),
        ("ta", "ப\u{bbe}க\u{bcd}ல\u{bbe}ன\u{bcd}ட\u{bcd} த\u{bc0}வுகள\u{bcd} (ம\u{bbe}ல\u{bcd}வின\u{bbe}ஸ\u{bcd})"),
        ("te", "ఫ\u{c3e}ల\u{c4d}కలంద\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}(మ\u{c3e}ల\u{c4d}వ\u{c3f}న\u{c3e}స\u{c4d})"),
        ("tg", "Ҷазираҳои Фолкленд (Малвин)"),
        ("th", "หม\u{e39}\u{e48}เกาะฟอล\u{e4c}กแลนด\u{e4c} (มาลบ\u{e35}นาส)"),
        ("ti", "Falkland Islands (Malvinas)"),
        ("tk", "Folklend (Malwin) adalary"),
        ("tl", "Falkland Islands (Malvinas)"),
        ("tr", "Falkland Adaları (Malvinas)"),
        ("tt", "Фалкланд Утравлары (Малвиннар)"),
        ("ug", "فالكلاند تاقىم ئاراللىرى (مالۋىناس)"),
        ("uk", "Фолклендські острови (Британія)"),
        ("ur", "Falkland Islands (Malvinas)"),
        ("uz", "Falkland Islands (Malvinas)"),
        ("ve", "Falkland Islands (Malvinas)"),
        ("vi", "Quần Đảo Phoa-kh-lận-đợ (Man-vi-na)"),
        ("wa", "Iyes Malouwines"),
        ("wo", "Iil yu Falkland (Malvinas)"),
        ("xh", "Falkland Islands (Malvinas)"),
        ("yo", "Falkland Islands (Malvinas)"),
        ("zh_CN", "福克兰群岛(马尔维纳斯)"),
        ("zh_HK", "福克蘭羣島"),
        ("zh_TW", "福克蘭群島 (馬維娜斯)"),
        ("zu", "Falkland Islands (Malvinas)"),
];
    #[cfg(all(feature = "fk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -51.796253;
        pub const LONGITUDE: f64 = -59.523613;
        pub const MAX_LATITUDE: f64 = -50.9809115;
        pub const MAX_LONGITUDE: f64 = -57.6768495;
        pub const MIN_LATITUDE: f64 = -52.4744161;
        pub const MIN_LONGITUDE: f64 = -61.3792419;
        pub const NORTHEAST_LATITUDE: f64 = -50.9809115;
        pub const NORTHEAST_LONGITUDE: f64 = -57.6768495;
        pub const SOUTHWEST_LATITUDE: f64 = -52.4744161;
        pub const SOUTHWEST_LONGITUDE: f64 = -61.3792419;
    }
}
#[cfg(all(feature = "fk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -51.796253,
            longitude: -59.523613,
            max_latitude: -50.9809115,
            max_longitude: -57.6768495,
            min_latitude: -52.4744161,
            min_longitude: -61.3792419,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -50.9809115,
                    longitude: -57.6768495,
                },
                southwest: CountryGeoBound {
                    latitude: -52.4744161,
                    longitude: -61.3792419,
                },
            },
        }
    }
}

#[cfg(all(feature = "fk", feature = "subdivisions"))]
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
#[cfg(feature = "fk")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::FK,
        alpha3: Alpha3::FLK,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 500,
        currency_code: "FKP",
        gec: Some(GEC::FK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "The Falkland Islands",
        iso_short_name: "Falkland Islands (Malvinas)",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [5].to_vec(),
        national_prefix: "None",
        nationality: Some("Falkland Islander"),
        number: "238",
        postal_code: true,
        postal_code_format: Some("FIQQ 1ZZ"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "FK",
        unofficial_name_list: ["Falkland Islands", "Falklandinseln", "Îles Malouines", "Islas Malvinas", "フォークランド（マルビナス）諸島", "Falklandeilanden [Islas Malvinas]"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Falkland Islands (Malvinas)"), ("af", "Falklandeilande (Malvinas)"), ("ak", "Falkland Islands (Malvinas)"), ("am", "Falkland Islands (Malvinas)"), ("an", "Falkland Islands (Malvinas)"), ("ar", "جزر فولكلاند (مالفيناس)"), ("as", "ফ\u{9be}কলেণ\u{9cd}ড দ\u{9cd}বীপ (ম\u{9be}ল\u{9cd}\u{200c}ভিন\u{9be}ছ)"), ("ay", "Falkland Islands (Malvinas)"), ("az", "Fokland Adaları (Malvin)"), ("ba", "Falkland Islands (Malvinas)"), ("be", "Фалклендскія астравы (Мальвіны)"), ("bg", "Фолклендски Острови (Малвини)"), ("bi", "Falkland Islands (Malvinas)"), ("bn", "ফল\u{9cd}কল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ (ম\u{9be}লভিন\u{9be}স)"), ("bn_IN", "ফল\u{9cd}কল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ (ম\u{9be}লভিন\u{9be}স)"), ("br", "Falkland Islands (Malvinas)"), ("bs", "Falklandsko otočje"), ("ca", "Illes Malvines (Falkland)"), ("ce", "Falkland Islands (Malvinas)"), ("ch", "Falkland Islands (Malvinas)"), ("cs", "Falkandské ostrovy (Malvíny)"), ("cv", "Falkland Islands (Malvinas)"), ("cy", "Ynysoedd y Falklands (Malvinas)"), ("da", "Falklandsøerne (Malvinas)"), ("de", "Falklandinseln (Malwinen)"), ("dv", "Falkland Islands (Malvinas)"), ("dz", "ཕ\u{f71}ལཀ་ལ\u{f7a}ནཌ\u{f72}་ མཚ\u{f7c}་ག\u{fb3}\u{f72}ང་ (མ\u{f71}ལ་ཝ\u{f72}་ན\u{f71}ས\u{f72})"), ("ee", "Falkland Islands (Malvinas)"), ("el", "Νήσοι Φώκλαντ (Μαλβίνες)"), ("en", "Falkland Islands (Malvinas)"), ("eo", "Falklandoj (Malvinoj)"), ("es", "Islas Falkland (Malvinas)"), ("et", "Falklandi saared"), ("eu", "Falkland uharteak (Malvina uharteak)"), ("fa", "جزایر فالکلند(مالویناس)"), ("ff", "Falkland Islands (Malvinas)"), ("fi", "Falklandinsaaret"), ("fo", "Falkland Islands (Malvinas)"), ("fr", "Malouines, Îles (Falkland)"), ("fy", "Falkland Islands (Malvinas)"), ("ga", "Oileáin Fháclainne (Malvinas)"), ("gl", "Illas Falkland (Malvinas)"), ("gn", "Falkland Islands (Malvinas)"), ("gu", "ફોકલ\u{ac7}ન\u{acd}ડ ટાપ\u{ac1}ઓ (માલ\u{acd}વિનસ)"), ("gv", "Falkland Islands (Malvinas)"), ("ha", "Falkland Islands (Malvinas)"), ("he", "איי פוקלנד"), ("hi", "फॉकल\u{948}\u{902}ड आइल\u{948}\u{902}ड\u{94d}स (मालविनास)"), ("hr", "Falklandski otoci"), ("ht", "Falkland Islands (Malvinas)"), ("hu", "Falkland-szigetek (Malvinas)"), ("hy", "Ֆոլկլոնդյան Կղզիներ"), ("ia", "Insulas Falkland (Malvinas)"), ("id", "Kepulauan Falkland (Malvinas)"), ("io", "Falkland Islands (Malvinas)"), ("is", "Falklandseyjar (Malvinaeyjar)"), ("it", "Isole Falkland (Malvine)"), ("iu", "Falkland Islands (Malvinas)"), ("ja", "フォークランド諸島 (マルビナス)"), ("ka", "ფოლკლენდის კუნძულები"), ("ki", "Falkland Islands (Malvinas)"), ("kk", "Фолкленд (Мальвин) аралдары"), ("kl", "Falkland Islands (Malvinas)"), ("km", "កោះ\u{200b}ហ\u{17d2}វ\u{17c9}កឡង\u{17cb} (ម\u{17c9}ាល\u{17cb}វ\u{17b8}ណា)"), ("kn", "ಫಾಕ\u{ccd}\u{200d}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ದ\u{ccd}ವೀಪಗಳು(ಮಾಲ\u{ccd}ವ\u{cbf}ನಾಸ\u{ccd})"), ("ko", "포클랜드 제도 (말비나스)"), ("ku", "Giravên Falkland (Malvin)"), ("kv", "Falkland Islands (Malvinas)"), ("kw", "Falkland Islands (Malvinas)"), ("ky", "Фолкленд (Мальвин) аралдары"), ("lo", "Falkland Islands (Malvinas)"), ("lt", "Folklando (Malvinų) salos"), ("lv", "Folklenda (Malvinu) salas"), ("mi", "Falkland Islands (Malvinas)"), ("mk", "Фокландски острови (Малвини)"), ("ml", "ഫ\u{d3e}ക\u{d4d}\u{200d}ല\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d} (മ\u{d3e}ള\u{d4d}\u{200d}വിന\u{d3e}സ\u{d4d})"), ("mn", "Falkland Islands (Malvinas)"), ("mr", "फाल\u{94d}कन आयल\u{945}\u{902}डस\u{94d}(माल\u{94d}विनास)"), ("ms", "Kepulauan Falkland (Malvinas)"), ("mt", "Falkland Islands (Malvinas)"), ("my", "Falkland Islands (Malvinas)"), ("na", "Falkland Islands (Malvinas)"), ("nb", "Falklandsøyene"), ("ne", "फकल\u{94d}याण\u{94d}ड टाप\u{941} (Malvinas)"), ("nl", "Falklandeilanden"), ("nn", "Falklandsøyane"), ("nv", "Falkland Islands (Malvinas)"), ("oc", "Illas Falkand (Malvinas)"), ("or", "ଫ\u{b3e}ଲ\u{b4d}କଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ ଦ\u{b4d}ବୀପ(ମ\u{b3e}ଲ\u{b4d}ଭ\u{b3f}ନ\u{b3e}ସ)"), ("pa", "ਫਾਕਲ\u{a48}\u{a02}ਡ ਟਾਪ\u{a42} (ਮਾਲਵੀਨਸ)"), ("pi", "Falkland Islands (Malvinas)"), ("pl", "Falklandy (Malwiny)"), ("ps", "Falkland Islands (Malvinas)"), ("pt", "Ilhas Falkland (Malvinas)"), ("pt_BR", "Ilhas Malvinas (Falkland)"), ("ro", "Insulele Falkland (Insulele Malvine)"), ("ru", "Фолклендские (Мальвинские) острова"), ("rw", "Ibirwa bya Folukilande (Maluvinasi)"), ("sc", "Ìsulas Falkland (Malvinas)"), ("sd", "Falkland Islands (Malvinas)"), ("si", "ෆෝක\u{dca}ලන\u{dca}ඩ\u{dca} ද\u{dd6}පත\u{dca} (මැල\u{dca}ව\u{dd2}න\u{dcf})"), ("sk", "Falklandy (Malvíny)"), ("sl", "Falklandski otoki"), ("so", "Falkland Islands (Malvinas)"), ("sq", "Ishujt Falkland (Malvinas)"), ("sr", "Фолкландска острва (Малвини)"), ("sv", "Falklandsöarna (Malvinas)"), ("sw", "Falkland Islands (Malvinas)"), ("ta", "ப\u{bbe}க\u{bcd}ல\u{bbe}ன\u{bcd}ட\u{bcd} த\u{bc0}வுகள\u{bcd} (ம\u{bbe}ல\u{bcd}வின\u{bbe}ஸ\u{bcd})"), ("te", "ఫ\u{c3e}ల\u{c4d}కలంద\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}(మ\u{c3e}ల\u{c4d}వ\u{c3f}న\u{c3e}స\u{c4d})"), ("tg", "Ҷазираҳои Фолкленд (Малвин)"), ("th", "หม\u{e39}\u{e48}เกาะฟอล\u{e4c}กแลนด\u{e4c} (มาลบ\u{e35}นาส)"), ("ti", "Falkland Islands (Malvinas)"), ("tk", "Folklend (Malwin) adalary"), ("tl", "Falkland Islands (Malvinas)"), ("tr", "Falkland Adaları (Malvinas)"), ("tt", "Фалкланд Утравлары (Малвиннар)"), ("ug", "فالكلاند تاقىم ئاراللىرى (مالۋىناس)"), ("uk", "Фолклендські острови (Британія)"), ("ur", "Falkland Islands (Malvinas)"), ("uz", "Falkland Islands (Malvinas)"), ("ve", "Falkland Islands (Malvinas)"), ("vi", "Quần Đảo Phoa-kh-lận-đợ (Man-vi-na)"), ("wa", "Iyes Malouwines"), ("wo", "Iil yu Falkland (Malvinas)"), ("xh", "Falkland Islands (Malvinas)"), ("yo", "Falkland Islands (Malvinas)"), ("zh_CN", "福克兰群岛(马尔维纳斯)"), ("zh_HK", "福克蘭羣島"), ("zh_TW", "福克蘭群島 (馬維娜斯)"), ("zu", "Falkland Islands (Malvinas)")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
