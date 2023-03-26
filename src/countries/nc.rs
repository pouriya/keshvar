// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// New Caledonia

#[cfg(all(feature = "nc", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::NC;
    pub const ALPHA3: Alpha3 = Alpha3::NCL;
    pub const CONTINENT: Continent = Continent::Australia;
    pub const COUNTRY_CODE: usize = 687;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::XPF;
    pub const GEC: Option<GEC> = Some(GEC::NC);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "New Caledonia";
    pub const ISO_LONG_NAME: &str = "New Caledonia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("New Caledonian");
    pub const NUMBER: &str = "540";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("988\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Melanesia);
    pub const UN_LOCODE: &str = "NC";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "New Caledonia",
        "Neukaledonien",
        "Nouvelle-Calédonie",
        "Nueva Caledonia",
        "ニューカレドニア",
        "Nieuw-Caledonië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "New Caledonia"),
        ("af", "Nieu-Kaledonië"),
        ("ak", "New Caledonia"),
        ("am", "ኒፄ ጢሓ፦ኒ።"),
        ("an", "New Caledonia"),
        ("ar", "نيو قلدونيا"),
        ("as", "নিউ কেলেডনিয়\u{9be}"),
        ("ay", "New Caledonia"),
        ("az", "Yeni Kaledoniya"),
        ("ba", "New Caledonia"),
        ("be", "Новая Каледонія"),
        ("bg", "Нова Каледония"),
        ("bi", "New Caledonia"),
        ("bn", "নিউ ক\u{9cd}য\u{9be}লেডোনিয়\u{9be}"),
        ("bn_IN", "নিউ ক\u{9cd}য\u{9be}লেডোনিয়\u{9be}"),
        ("br", "Kaledonia-Nevez"),
        ("bs", "Nova Kaledonija"),
        ("ca", "Nova Caledònia"),
        ("ce", "Керла Каледони"),
        ("ch", "New Caledonia"),
        ("cs", "Nová Kaledonie"),
        ("cv", "Керла Каледони"),
        ("cy", "Caledonia Newydd"),
        ("da", "Ny Kaledonien"),
        ("de", "Neukaledonien"),
        (
            "dv",
            "ނ\u{7a8}އ\u{7aa} ކ\u{7ac}ލ\u{7ac}ޑ\u{7af}ނ\u{7a8}އ\u{7a7}",
        ),
        (
            "dz",
            "ན\u{f72}འ\u{f74}་ ཀ\u{f7a}་ལ\u{f72}་ཌ\u{f7c}་ན\u{f72}་ཡ།",
        ),
        ("ee", "New Caledonia"),
        ("el", "Νέα Καληδονία"),
        ("en", "New Caledonia"),
        ("eo", "Nov-Kaledonio"),
        ("es", "Nueva Caledonia"),
        ("et", "Uus-Kaledoonia"),
        ("eu", "Kaledonia Berria"),
        ("fa", "کالدونیای جدید"),
        ("ff", "New Caledonia"),
        ("fi", "Uusi-Kaledonia"),
        ("fo", "New Caledonia"),
        ("fr", "Nouvelle-Calédonie"),
        ("fy", "Nij-Kaledoanje"),
        ("ga", "An Nua-Chaladóin"),
        ("gl", "Nova Caledonia"),
        ("gn", "New Caledonia"),
        ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"),
        ("gv", "New Caledonia"),
        ("ha", "New Caledonia"),
        ("he", "קלדוניה החדשה"),
        ("hi", "नया क\u{948}ल\u{947}डोनिया"),
        ("hr", "Nova Kaledonija"),
        ("ht", "Nouvèl Kaledoni"),
        ("hu", "Új-Kaledónia"),
        ("hy", "Նոր Կալեդոնիա"),
        ("ia", "Nove Caledonia"),
        ("id", "Kaledonia Baru"),
        ("io", "Nova-Kaledonia"),
        ("is", "Nýja-Kaledónía"),
        ("it", "Nuova Caledonia"),
        ("iu", "New Caledonia"),
        ("ja", "ニューカレドニア"),
        ("ka", "ახალი კალედონია"),
        ("ki", "New Caledonia"),
        ("kk", "Жаңа Каледония"),
        ("kl", "New Caledonia"),
        ("km", "ន\u{17bc}វែលកាលេដ\u{17bc}ន\u{17b8}"),
        ("kn", "ಹೊಸ ಕ\u{cc6}ಲ\u{cc6}ಡೊನ\u{cbf}ಯಾ"),
        ("ko", "누벨칼레도니"),
        ("ku", "Kaledonyaya Nû"),
        ("kv", "New Caledonia"),
        ("kw", "Kelesoni Nowydh"),
        ("ky", "Жаңы Каледония"),
        ("lo", "New Caledonia"),
        ("lt", "Naujoji Kaledonija"),
        ("lv", "Jaunkaledonija"),
        ("mi", "New Caledonia"),
        ("mk", "Нова Каледонија"),
        ("ml", "ന\u{d4d}യ\u{d42} ക\u{d3e}ലിഡോമിയ"),
        ("mn", "Шинэ калдени"),
        ("mr", "न\u{94d}य\u{942} क\u{945}लिडोनिया"),
        ("ms", "New Caledonia"),
        ("mt", "Kaledonja l-Ġdida"),
        ("my", "New Caledonia"),
        ("na", "New Caledonia"),
        ("nb", "Ny-Caledonia"),
        ("ne", "नया\u{901} क\u{94d}याल\u{947}दोनिया"),
        ("nl", "Nieuw-Caledonië"),
        ("nn", "Ny-Caledonia"),
        ("nv", "New Caledonia"),
        ("oc", "Nòva Caledònia"),
        ("or", "ନ\u{b4d}ଯ\u{b41} କ\u{b3e}ଲେଡୋନ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਨਵਾ\u{a02} ਕਾਲੀਡ\u{a4b}ਨੀਆ\u{a02}"),
        ("pi", "New Caledonia"),
        ("pl", "Nowa Kaledonia"),
        ("ps", "New Caledonia"),
        ("pt", "Nova Caledónia"),
        ("pt_BR", "Nova Caledônia"),
        ("ro", "Noua Caledonie"),
        ("ru", "Новая Каледония"),
        ("rw", "Nuveli Kalidoniya"),
        ("sc", "Noa Caledònia"),
        ("sd", "New Caledonia"),
        ("si", "නව කැල\u{dd2}ඩෝන\u{dd2}ය\u{dcf}ව"),
        ("sk", "Nová Kaledónia"),
        ("sl", "Nova Kaledonija"),
        ("so", "New Caledonia"),
        ("sq", "Kaledonia e Re"),
        ("sr", "Нова Каледонија"),
        ("sv", "Nya Kaledonien"),
        ("sw", "New Caledonia"),
        ("ta", "புதிய கலிடோனிய\u{bbe}"),
        ("te", "న\u{c4d}యూ క\u{c46}ల\u{c3f}డ\u{c4b}న\u{c3f}య\u{c3e}"),
        ("tg", "Каледонияи Нав"),
        ("th", "น\u{e34}วแคล\u{e34}โดเน\u{e35}ย"),
        ("ti", "ኒው ካሌዶኒያ"),
        ("tk", "Täze Kaledoniýa"),
        ("tl", "Bagong Caledonia"),
        ("tr", "Yeni Kaledonya"),
        ("tt", "Яңа Каледониа"),
        ("ug", "يېڭى كالېدونىيە"),
        ("uk", "Нова Каледонія"),
        ("ur", "نیو کیلیڈونیا"),
        ("uz", "Yangi kaledoniya"),
        ("ve", "New Caledonia"),
        ("vi", "Niu Ca-lê-đô-ni-a"),
        ("wa", "Nouve Caledoneye"),
        ("wo", "Nuweel Kaledooni"),
        ("xh", "New Caledonia"),
        ("yo", "Kalẹdóníà Tuntun"),
        ("zh_CN", "新喀里多尼亚"),
        ("zh_HK", "新喀里多尼亞島"),
        ("zh_TW", "新喀里多尼亞"),
        ("zu", "New Caledonia"),
    ];
    #[cfg(all(feature = "nc", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -20.904305;
        pub const LONGITUDE: f64 = 165.618042;
        pub const MAX_LATITUDE: f64 = -19.1607355;
        pub const MAX_LONGITUDE: f64 = 168.3325194;
        pub const MIN_LATITUDE: f64 = -23.2514406;
        pub const MIN_LONGITUDE: f64 = 163.3557129;
        pub const NORTHEAST_LATITUDE: f64 = -19.1607355;
        pub const NORTHEAST_LONGITUDE: f64 = 168.3325194;
        pub const SOUTHWEST_LATITUDE: f64 = -23.2514406;
        pub const SOUTHWEST_LONGITUDE: f64 = 163.3557129;
    }
}
#[cfg(all(feature = "nc", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -20.904305,
            longitude: 165.618042,
            max_latitude: -19.1607355,
            max_longitude: 168.3325194,
            min_latitude: -23.2514406,
            min_longitude: 163.3557129,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -19.1607355,
                    longitude: 168.3325194,
                },
                southwest: CountryGeoBound {
                    latitude: -23.2514406,
                    longitude: 163.3557129,
                },
            },
        }
    }
}

#[cfg(all(feature = "nc", feature = "subdivisions"))]
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
#[cfg(feature = "nc")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::NC,
        alpha3: Alpha3::NCL,
        address_format: None,
        continent: Continent::Australia,
        country_code: 687,
        currency_code: CurrencyCode::XPF,
        gec: Some(GEC::NC),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "New Caledonia",
        iso_short_name: "New Caledonia",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [6].to_vec(),
        national_prefix: "None",
        nationality: Some("New Caledonian"),
        number: "540",
        postal_code: true,
        postal_code_format: Some("988\\d{2}"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Melanesia),
        un_locode: "NC",
        unofficial_name_list: [
            "New Caledonia",
            "Neukaledonien",
            "Nouvelle-Calédonie",
            "Nueva Caledonia",
            "ニューカレドニア",
            "Nieuw-Caledonië",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "New Caledonia"),
            ("af", "Nieu-Kaledonië"),
            ("ak", "New Caledonia"),
            ("am", "ኒፄ ጢሓ፦ኒ።"),
            ("an", "New Caledonia"),
            ("ar", "نيو قلدونيا"),
            ("as", "নিউ কেলেডনিয়\u{9be}"),
            ("ay", "New Caledonia"),
            ("az", "Yeni Kaledoniya"),
            ("ba", "New Caledonia"),
            ("be", "Новая Каледонія"),
            ("bg", "Нова Каледония"),
            ("bi", "New Caledonia"),
            ("bn", "নিউ ক\u{9cd}য\u{9be}লেডোনিয়\u{9be}"),
            ("bn_IN", "নিউ ক\u{9cd}য\u{9be}লেডোনিয়\u{9be}"),
            ("br", "Kaledonia-Nevez"),
            ("bs", "Nova Kaledonija"),
            ("ca", "Nova Caledònia"),
            ("ce", "Керла Каледони"),
            ("ch", "New Caledonia"),
            ("cs", "Nová Kaledonie"),
            ("cv", "Керла Каледони"),
            ("cy", "Caledonia Newydd"),
            ("da", "Ny Kaledonien"),
            ("de", "Neukaledonien"),
            (
                "dv",
                "ނ\u{7a8}އ\u{7aa} ކ\u{7ac}ލ\u{7ac}ޑ\u{7af}ނ\u{7a8}އ\u{7a7}",
            ),
            (
                "dz",
                "ན\u{f72}འ\u{f74}་ ཀ\u{f7a}་ལ\u{f72}་ཌ\u{f7c}་ན\u{f72}་ཡ།",
            ),
            ("ee", "New Caledonia"),
            ("el", "Νέα Καληδονία"),
            ("en", "New Caledonia"),
            ("eo", "Nov-Kaledonio"),
            ("es", "Nueva Caledonia"),
            ("et", "Uus-Kaledoonia"),
            ("eu", "Kaledonia Berria"),
            ("fa", "کالدونیای جدید"),
            ("ff", "New Caledonia"),
            ("fi", "Uusi-Kaledonia"),
            ("fo", "New Caledonia"),
            ("fr", "Nouvelle-Calédonie"),
            ("fy", "Nij-Kaledoanje"),
            ("ga", "An Nua-Chaladóin"),
            ("gl", "Nova Caledonia"),
            ("gn", "New Caledonia"),
            ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"),
            ("gv", "New Caledonia"),
            ("ha", "New Caledonia"),
            ("he", "קלדוניה החדשה"),
            ("hi", "नया क\u{948}ल\u{947}डोनिया"),
            ("hr", "Nova Kaledonija"),
            ("ht", "Nouvèl Kaledoni"),
            ("hu", "Új-Kaledónia"),
            ("hy", "Նոր Կալեդոնիա"),
            ("ia", "Nove Caledonia"),
            ("id", "Kaledonia Baru"),
            ("io", "Nova-Kaledonia"),
            ("is", "Nýja-Kaledónía"),
            ("it", "Nuova Caledonia"),
            ("iu", "New Caledonia"),
            ("ja", "ニューカレドニア"),
            ("ka", "ახალი კალედონია"),
            ("ki", "New Caledonia"),
            ("kk", "Жаңа Каледония"),
            ("kl", "New Caledonia"),
            ("km", "ន\u{17bc}វែលកាលេដ\u{17bc}ន\u{17b8}"),
            ("kn", "ಹೊಸ ಕ\u{cc6}ಲ\u{cc6}ಡೊನ\u{cbf}ಯಾ"),
            ("ko", "누벨칼레도니"),
            ("ku", "Kaledonyaya Nû"),
            ("kv", "New Caledonia"),
            ("kw", "Kelesoni Nowydh"),
            ("ky", "Жаңы Каледония"),
            ("lo", "New Caledonia"),
            ("lt", "Naujoji Kaledonija"),
            ("lv", "Jaunkaledonija"),
            ("mi", "New Caledonia"),
            ("mk", "Нова Каледонија"),
            ("ml", "ന\u{d4d}യ\u{d42} ക\u{d3e}ലിഡോമിയ"),
            ("mn", "Шинэ калдени"),
            ("mr", "न\u{94d}य\u{942} क\u{945}लिडोनिया"),
            ("ms", "New Caledonia"),
            ("mt", "Kaledonja l-Ġdida"),
            ("my", "New Caledonia"),
            ("na", "New Caledonia"),
            ("nb", "Ny-Caledonia"),
            ("ne", "नया\u{901} क\u{94d}याल\u{947}दोनिया"),
            ("nl", "Nieuw-Caledonië"),
            ("nn", "Ny-Caledonia"),
            ("nv", "New Caledonia"),
            ("oc", "Nòva Caledònia"),
            ("or", "ନ\u{b4d}ଯ\u{b41} କ\u{b3e}ଲେଡୋନ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਨਵਾ\u{a02} ਕਾਲੀਡ\u{a4b}ਨੀਆ\u{a02}"),
            ("pi", "New Caledonia"),
            ("pl", "Nowa Kaledonia"),
            ("ps", "New Caledonia"),
            ("pt", "Nova Caledónia"),
            ("pt_BR", "Nova Caledônia"),
            ("ro", "Noua Caledonie"),
            ("ru", "Новая Каледония"),
            ("rw", "Nuveli Kalidoniya"),
            ("sc", "Noa Caledònia"),
            ("sd", "New Caledonia"),
            ("si", "නව කැල\u{dd2}ඩෝන\u{dd2}ය\u{dcf}ව"),
            ("sk", "Nová Kaledónia"),
            ("sl", "Nova Kaledonija"),
            ("so", "New Caledonia"),
            ("sq", "Kaledonia e Re"),
            ("sr", "Нова Каледонија"),
            ("sv", "Nya Kaledonien"),
            ("sw", "New Caledonia"),
            ("ta", "புதிய கலிடோனிய\u{bbe}"),
            ("te", "న\u{c4d}యూ క\u{c46}ల\u{c3f}డ\u{c4b}న\u{c3f}య\u{c3e}"),
            ("tg", "Каледонияи Нав"),
            ("th", "น\u{e34}วแคล\u{e34}โดเน\u{e35}ย"),
            ("ti", "ኒው ካሌዶኒያ"),
            ("tk", "Täze Kaledoniýa"),
            ("tl", "Bagong Caledonia"),
            ("tr", "Yeni Kaledonya"),
            ("tt", "Яңа Каледониа"),
            ("ug", "يېڭى كالېدونىيە"),
            ("uk", "Нова Каледонія"),
            ("ur", "نیو کیلیڈونیا"),
            ("uz", "Yangi kaledoniya"),
            ("ve", "New Caledonia"),
            ("vi", "Niu Ca-lê-đô-ni-a"),
            ("wa", "Nouve Caledoneye"),
            ("wo", "Nuweel Kaledooni"),
            ("xh", "New Caledonia"),
            ("yo", "Kalẹdóníà Tuntun"),
            ("zh_CN", "新喀里多尼亚"),
            ("zh_HK", "新喀里多尼亞島"),
            ("zh_TW", "新喀里多尼亞"),
            ("zu", "New Caledonia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
