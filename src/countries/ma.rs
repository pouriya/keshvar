// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Morocco

#[cfg(all(feature = "ma", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MA;
    pub const ALPHA3: Alpha3 = Alpha3::MAR;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 212;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::MAD;
    pub const GEC: Option<GEC> = Some(GEC::MO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::MAR);
    pub const ISO_SHORT_NAME: &str = "Morocco";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Morocco";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Moroccan");
    pub const NUMBER: &str = "504";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAfrica);
    pub const UN_LOCODE: &str = "MA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Morocco",
        "المغرب",
        "Marokko",
        "Maroc",
        "Marruecos",
        "モロッコ",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Morocco"),
        ("af", "Marokko"),
        ("ak", "Morocco"),
        ("am", "ሥስጥ"),
        ("an", "Morocco"),
        ("ar", "المغرب"),
        ("as", "মৰোক\u{9cd}কো"),
        ("ay", "Morocco"),
        ("az", "Mərakeş"),
        ("ba", "Morocco"),
        ("be", "Марока"),
        ("bg", "Мароко"),
        ("bi", "Morocco"),
        ("bn", "মরোক\u{9cd}কো"),
        ("bn_IN", "মরোক\u{9cd}কো"),
        ("br", "Maroko"),
        ("bs", "Maroko"),
        ("ca", "Marroc"),
        ("ce", "Марокко"),
        ("ch", "Morocco"),
        ("cs", "Maroko"),
        ("cv", "Марокко"),
        ("cy", "Moroco"),
        ("da", "Marokko"),
        ("de", "Marokko"),
        ("dv", "މ\u{7a6}އ\u{7aa}ރ\u{7a8}ބ\u{7aa}"),
        ("dz", "མ\u{f7c}་ར\u{f7c}ཀ་ཀ\u{f7c}།"),
        ("ee", "Morocco"),
        ("el", "Μαρόκο"),
        ("en", "Morocco"),
        ("eo", "Maroko"),
        ("es", "Marruecos"),
        ("et", "Maroko"),
        ("eu", "Maroko"),
        ("fa", "مراکش"),
        ("ff", "Morocco"),
        ("fi", "Marokko"),
        ("fo", "Marokko"),
        ("fr", "Maroc"),
        ("fy", "Marokko"),
        ("ga", "Maracó"),
        ("gl", "Marrocos"),
        ("gn", "Morocco"),
        ("gu", "મોરક\u{acd}કો"),
        ("gv", "Yn Varoc"),
        ("ha", "Morocco"),
        ("he", "מרוקו"),
        ("hi", "मोरक\u{94d}को"),
        ("hr", "Maroko"),
        ("ht", "Mawòk"),
        ("hu", "Marokkó"),
        ("hy", "Մարոկո"),
        ("ia", "Marocco"),
        ("id", "Maroko"),
        ("io", "Maroko"),
        ("is", "Marokkó"),
        ("it", "Marocco"),
        ("iu", "Morocco"),
        ("ja", "モロッコ"),
        ("ka", "მაროკო"),
        ("ki", "Morocco"),
        ("kk", "Марокко"),
        ("kl", "Morocco"),
        ("km", "ម\u{17c9}ារ\u{17c9}\u{17bb}ក"),
        ("kn", "ಮೊರೊಕ\u{ccd}ಕೊ"),
        ("ko", "모로코"),
        ("ku", "Fas"),
        ("kv", "Morocco"),
        ("kw", "Marokk"),
        ("ky", "Марокко"),
        ("lo", "Morocco"),
        ("lt", "Marokas"),
        ("lv", "Maroka"),
        ("mi", "Moroko"),
        ("mk", "Мароко"),
        ("ml", "മൊറോക\u{d4d}കോ"),
        ("mn", "Морокко"),
        ("mr", "मोरोक\u{94d}को"),
        ("ms", "Maghribi"),
        ("mt", "Marokk"),
        (
            "my",
            "မော\u{103a}ရ\u{102d}\u{102f}က\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Morocco"),
        ("nb", "Marokko"),
        ("ne", "मोरक\u{94d}को"),
        ("nl", "Marokko"),
        ("nn", "Marokko"),
        (
            "nv",
            "Eʼeʼaahjí Ghą\u{301}ą\u{301}ʼaskʼidii Biłikahii Bikéyah",
        ),
        ("oc", "Marròc"),
        ("or", "ମୋରକ\u{b4d}କୋ"),
        ("pa", "ਮ\u{a4b}ਰ\u{a4b}ਸ\u{a3c}ਸ\u{a4b}"),
        ("pi", "मोराको"),
        ("pl", "Maroko"),
        ("ps", "مراکش"),
        ("pt", "Marrocos"),
        ("pt_BR", "Marrocos"),
        ("ro", "Maroc"),
        ("ru", "Марокко"),
        ("rw", "Maroke"),
        ("sc", "Marocu"),
        ("sd", "Morocco"),
        ("si", "මොරොක\u{dca}කෝව"),
        ("sk", "Maroko"),
        ("sl", "Maroko"),
        ("so", "Marooko"),
        ("sq", "Marok"),
        ("sr", "Мароко"),
        ("sv", "Marocko"),
        ("sw", "Morocco"),
        ("ta", "மொரோக\u{bcd}கோ"),
        ("te", "మ\u{c4b}ర\u{c4b}క\u{c4d}క\u{c4b}"),
        ("tg", "Марокаш"),
        ("th", "โมร\u{e47}อกโก"),
        ("ti", "ሞሮኮ"),
        ("tk", "Morokko"),
        ("tl", "Morocco"),
        ("tr", "Fas"),
        ("tt", "Морокко"),
        ("ug", "ماراكەش"),
        ("uk", "Марокко"),
        ("ur", "مراکش"),
        ("uz", "Marokash"),
        ("ve", "Morocco"),
        ("vi", "Mo-ro-cô"),
        ("wa", "Marok"),
        ("wo", "Marook"),
        ("xh", "Morocco"),
        ("yo", "Mòrókò"),
        ("zh_CN", "摩洛哥"),
        ("zh_HK", "摩洛哥"),
        ("zh_TW", "摩洛哥"),
        ("zu", "IMorokho"),
    ];
    #[cfg(all(feature = "ma", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 31.791702;
        pub const LONGITUDE: f64 = -7.092619999999999;
        pub const MAX_LATITUDE: f64 = 35.9344;
        pub const MAX_LONGITUDE: f64 = -0.9969759;
        pub const MIN_LATITUDE: f64 = 27.6672693;
        pub const MIN_LONGITUDE: f64 = -13.3044001;
        pub const NORTHEAST_LATITUDE: f64 = 35.9344;
        pub const NORTHEAST_LONGITUDE: f64 = -0.9969759;
        pub const SOUTHWEST_LATITUDE: f64 = 27.6672693;
        pub const SOUTHWEST_LONGITUDE: f64 = -13.3044001;
    }
}
#[cfg(all(feature = "ma", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 31.791702,
            longitude: -7.092619999999999,
            max_latitude: 35.9344,
            max_longitude: -0.9969759,
            min_latitude: 27.6672693,
            min_longitude: -13.3044001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 35.9344,
                    longitude: -0.9969759,
                },
                southwest: CountryGeoBound {
                    latitude: 27.6672693,
                    longitude: -13.3044001,
                },
            },
        }
    }
}

#[cfg(all(feature = "ma", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from(
            [

                (
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::MA,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "طنجة - تطوان - الحسيمة")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::MA,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الشرق")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::MA,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فاس-مكناس")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::MA,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الرباط-سلا-القنيطرة")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::MA,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بني ملال - خنيفرة")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::MA,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الدار البيضاء - سطات")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::MA,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مراكش آسفي")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::MA,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "درعة - تافيلالت")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::MA,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سوس ماسة")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::MA,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ڭلميم-وادي نون")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::MA,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "العيون - الساقية الحمراء")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::MA,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الداخلة - وادي الذهب")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "AGD",
                    Subdivision{
                        name: "AGD",
                        country_alpha2: Alpha2::MA,
                        code: "AGD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.4277547), longitude: Some(-9.5981072), max_latitude: Some(30.4702104), min_latitude: Some(30.3692836), max_longitude: Some(-9.4871341), min_longitude: Some(-9.6682679)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أكادير"), ("be", "Агадыр"), ("bg", "Агадир"), ("bn", "আগ\u{9be}দির"), ("ca", "Agadir"), ("ccp", "𑄃𑄉𑄓\u{11128}𑄢\u{11134}-𑄃\u{11128}𑄓 𑄃\u{1112f} 𑄑𑄚𑄚𑄬"), ("ceb", "Agadir"), ("cs", "Agadir"), ("cy", "Agadir"), ("da", "Agadir"), ("de", "Agadir"), ("el", "Αγαδίρ"), ("en", "Agadir-Ida Ou Tanane"), ("es", "Agadir"), ("et", "Agadir"), ("eu", "Agadir"), ("fa", "اگادیر"), ("fi", "Agadir"), ("fr", "Agadir"), ("gu", "અગાદિર"), ("he", "אגאדיר"), ("hi", "अगाडिर"), ("hr", "Agadir"), ("hu", "Agadir"), ("id", "Agadir"), ("it", "Agadir"), ("ja", "アガディール"), ("ka", "აგადირი"), ("kk", "Агадир қаласы"), ("kn", "ಅಗಡ\u{cbf}ರ\u{ccd}"), ("ko", "아가디르"), ("lt", "Agadiras"), ("lv", "Agādīra"), ("mk", "Агадир"), ("mr", "अगादिर"), ("ms", "Agadir"), ("nb", "Agadir"), ("nl", "Agadir"), ("no", "Agadir"), ("pl", "Agadir"), ("pt", "Agadir"), ("ro", "Agadir"), ("ru", "Агадир"), ("si", "අගඩ\u{dd2}ර\u{dca}"), ("sk", "Agadir"), ("sl", "Agadir"), ("sq", "Agadir"), ("sr", "Агадир"), ("sr_Latn", "Agadir"), ("sv", "Agadir"), ("ta", "அகதிர\u{bcd}"), ("te", "అగ\u{c3e}డ\u{c3f}ర\u{c4d}"), ("th", "อกาด\u{e35}ร\u{e4c}"), ("tr", "Agadir"), ("uk", "Агадір"), ("ur", "اغادیر"), ("uz", "Agadir"), ("vi", "Agadir"), ("zh", "阿加迪尔")]),
                        unofficial_name_list: ["Agadir*"].to_vec(),
                    }
                ),
                (
                    "AOU",
                    Subdivision{
                        name: "AOU",
                        country_alpha2: Alpha2::MA,
                        code: "AOU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.3920338), longitude: Some(-9.564496499999999), max_latitude: Some(30.3923136), min_latitude: Some(30.39160919999999), max_longitude: Some(-9.5620805), min_longitude: Some(-9.5660522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أوسرد"), ("ca", "Província d’Auserd"), ("ccp", "𑄃𑄃\u{1112f}𑄥𑄬𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Aousserd (lalawigan)"), ("de", "Aousserd"), ("en", "Aousserd"), ("es", "Prefectura de Auserd"), ("fr", "province d’Aousserd"), ("it", "Prefettura di Aousserd"), ("ja", "アウサード州"), ("nl", "Aousserd"), ("sv", "Aousserd (provins)"), ("zh", "奧塞爾德省")]),
                        unofficial_name_list: ["Aousserd"].to_vec(),
                    }
                ),
                (
                    "ASZ",
                    Subdivision{
                        name: "ASZ",
                        country_alpha2: Alpha2::MA,
                        code: "ASZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.1402395), longitude: Some(-9.7232673), max_latitude: Some(28.8133365), min_latitude: Some(27.662115), max_longitude: Some(-8.670276), min_longitude: Some(-10.7308961)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم آسا الزاك"), ("ca", "Província d’Assa-Zag"), ("ccp", "𑄃𑄌\u{11133}𑄦\u{11134}-𑄎𑄇\u{11134}"), ("ceb", "Assa-Zag"), ("de", "Assa-Zag"), ("en", "Assa-Zag"), ("es", "Provincia de Assa-Zag"), ("fr", "province d’Assa-Zag"), ("hu", "Assa-Zag"), ("it", "Provincia di Assa-Zag"), ("ja", "アサ・ザグ州"), ("nl", "Assa-Zag"), ("pt", "Assa-Zag"), ("sv", "Assa-Zag"), ("zh", "阿薩-扎格省")]),
                        unofficial_name_list: ["Assa-Zag"].to_vec(),
                    }
                ),
                (
                    "AZI",
                    Subdivision{
                        name: "AZI",
                        country_alpha2: Alpha2::MA,
                        code: "AZI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.966944), longitude: Some(-6.569444), max_latitude: Some(31.9825259), min_latitude: Some(31.9387247), max_longitude: Some(-6.5521861), min_longitude: Some(-6.6000795)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أزيلال"), ("ca", "Província d’Azilal"), ("ccp", "𑄃𑄎\u{11128}𑄣𑄣\u{11134}"), ("ceb", "Azilal Province"), ("de", "Azilal"), ("en", "Azilal"), ("es", "Provincia de Azilal"), ("fr", "province d’Azilal"), ("it", "Provincia di Azilal"), ("ja", "アジラル州"), ("nl", "Azilal"), ("pt", "Azilal (província)"), ("sv", "Azilal Province"), ("zh", "艾濟拉勒省")]),
                        unofficial_name_list: ["Azilal"].to_vec(),
                    }
                ),
                (
                    "BAH",
                    Subdivision{
                        name: "BAH",
                        country_alpha2: Alpha2::MA,
                        code: "BAH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.0688758), longitude: Some(-9.152534), max_latitude: Some(30.0738757), min_latitude: Some(30.0648089), max_longitude: Some(-9.1488218), min_longitude: Some(-9.166395699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Aït Baha")]),
                        unofficial_name_list: ["Aït Baha"].to_vec(),
                    }
                ),
                (
                    "BEM",
                    Subdivision{
                        name: "BEM",
                        country_alpha2: Alpha2::MA,
                        code: "BEM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.339444), longitude: Some(-6.360833), max_latitude: Some(32.367638), min_latitude: Some(32.2941706), max_longitude: Some(-6.3162803), min_longitude: Some(-6.4301777)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بني ملال"), ("ca", "Província de Béni Mellal"), ("ccp", "𑄝𑄬𑄚\u{11128}-𑄟𑄬𑄣𑄣\u{11134}"), ("ceb", "Beni-Mellal"), ("de", "Béni Mellal (Provinz)"), ("en", "Béni-Mellal"), ("es", "Provincia de Beni Melal"), ("fr", "province de Béni-Mellal"), ("it", "Provincia di Béni-Mellal"), ("ja", "ベニ・メラル州"), ("nl", "Béni-Mellal"), ("pt", "Beni Mellal (província)"), ("sv", "Beni-Mellal"), ("zh", "貝尼邁拉勒省")]),
                        unofficial_name_list: ["Beni Mellal"].to_vec(),
                    }
                ),
                (
                    "BER",
                    Subdivision{
                        name: "BER",
                        country_alpha2: Alpha2::MA,
                        code: "BER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.916667), longitude: Some(-2.316667), max_latitude: Some(34.9551815), min_latitude: Some(34.8987438), max_longitude: Some(-2.2930527), min_longitude: Some(-2.3526192)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بركان"), ("ca", "Província de Berkane"), ("ccp", "𑄝𑄬𑄢\u{11134}𑄇𑄚\u{11134}"), ("ceb", "Berkane-Taourirt"), ("de", "Berkane (Provinz)"), ("en", "Berkane"), ("es", "Provincia de Berkan"), ("fa", "استان برکان"), ("fr", "province de Berkane"), ("it", "Provincia di Berkane"), ("ja", "ベルカンヌ州"), ("nl", "Berkane"), ("sv", "Berkane-Taourirt"), ("ur", "برکان صوبہ"), ("zh", "貝爾坎省")]),
                        unofficial_name_list: ["Berkane"].to_vec(),
                    }
                ),
                (
                    "BES",
                    Subdivision{
                        name: "BES",
                        country_alpha2: Alpha2::MA,
                        code: "BES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.616667), longitude: Some(-7.116667), max_latitude: Some(33.6338072), min_latitude: Some(33.5937142), max_longitude: Some(-7.1013782), min_longitude: Some(-7.1562243)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بنسليمان"), ("ca", "Província de Benslimane"), ("ccp", "𑄝𑄬𑄚\u{11134} 𑄥\u{11133}𑄣\u{11128}𑄟𑄚\u{11134}"), ("ceb", "Benslimane (lalawigan)"), ("de", "Ben Slimane"), ("en", "Ben Slimane"), ("es", "Provincia de Benslimane"), ("fr", "province de Benslimane"), ("it", "Provincia di Ben Slimane"), ("ja", "ベン・スリマン州"), ("nl", "Ben Slimane"), ("ru", "Бен-Слиман"), ("sv", "Benslimane (provins)"), ("zh", "本蘇萊曼省")]),
                        unofficial_name_list: ["Ben Slimane"].to_vec(),
                    }
                ),
                (
                    "BOD",
                    Subdivision{
                        name: "BOD",
                        country_alpha2: Alpha2::MA,
                        code: "BOD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.8521664), longitude: Some(-12.1632718), max_latitude: Some(28.2162268), min_latitude: Some(27.6665817), max_longitude: Some(-11.50715), min_longitude: Some(-13.1729662)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بوجدور"), ("ca", "Bojador"), ("ccp", "𑄝\u{1112f}𑄌\u{11134}𑄓\u{1112f}𑄢\u{11134}"), ("ceb", "Boujdour (lalawigan)"), ("de", "Boujdour (Provinz)"), ("en", "Boujdour"), ("es", "Provincia de Bojador"), ("fr", "province de Boujdour"), ("it", "Provincia di Boujdour"), ("ja", "ブジュール州"), ("nl", "Boujdour"), ("sv", "Boujdour (provins)"), ("zh", "布支杜爾省")]),
                        unofficial_name_list: ["Boujdour (EH)"].to_vec(),
                    }
                ),
                (
                    "BOM",
                    Subdivision{
                        name: "BOM",
                        country_alpha2: Alpha2::MA,
                        code: "BOM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.3625159), longitude: Some(-4.7303397), max_latitude: Some(33.3740471), min_latitude: Some(33.3536884), max_longitude: Some(-4.720602), min_longitude: Some(-4.7400855)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بولمان"), ("ca", "Província de Boulemane"), ("ccp", "𑄝\u{1112f}𑄣𑄬𑄟𑄚\u{11134}"), ("ceb", "Boulemane (lalawigan)"), ("de", "Boulemane (Provinz)"), ("en", "Boulemane"), ("es", "Provincia de Bulmán"), ("fr", "province de Boulemane"), ("it", "Provincia di Boulemane"), ("ja", "ブルマーヌ州"), ("nl", "Boulmane"), ("sv", "Boulemane (provins)"), ("zh", "布勒曼省")]),
                        unofficial_name_list: ["Boulemane"].to_vec(),
                    }
                ),
                (
                    "BRR",
                    Subdivision{
                        name: "BRR",
                        country_alpha2: Alpha2::MA,
                        code: "BRR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Berrechid")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CAS",
                    Subdivision{
                        name: "CAS",
                        country_alpha2: Alpha2::MA,
                        code: "CAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5731104), longitude: Some(-7.589843399999999), max_latitude: Some(33.649659), min_latitude: Some(33.495279), max_longitude: Some(-7.4582757), min_longitude: Some(-7.7164613)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Casablanca"), ("am", "ካሳብላንካ"), ("ar", "الدار البيضاء"), ("az", "Kasablanka"), ("be", "Касабланка"), ("bg", "Казабланка"), ("bn", "ক\u{9be}স\u{9be}ব\u{9cd}ল\u{9be}ংক\u{9be}"), ("bs", "Casablanca"), ("ca", "Casablanca"), ("ccp", "𑄇𑄥𑄝\u{11133}𑄣\u{11101}𑄇"), ("cs", "Casablanca"), ("cy", "Casablanca"), ("da", "Casablanca"), ("de", "Casablanca"), ("el", "Καζαμπλάνκα"), ("en", "Casablanca"), ("es", "Casablanca"), ("et", "Casablanca"), ("eu", "Casablanca"), ("fa", "کازابلانکا"), ("fi", "Casablanca"), ("fr", "Casablanca"), ("gl", "Casablanca"), ("gu", "ક\u{ac8}સાબ\u{acd}લાન\u{acd}કા"), ("ha", "Casablanca"), ("ha_NE", "Casablanca"), ("he", "קזבלנקה"), ("hi", "कासाब\u{94d}ला\u{902}का"), ("hr", "Casablanca"), ("hu", "Casablanca"), ("hy", "Կասաբլանկա"), ("id", "Casablanca"), ("is", "Casablanca"), ("it", "Casablanca"), ("ja", "カサブランカ"), ("ka", "კასაბლანკა"), ("kk", "Касабланка"), ("kn", "ಕಾಸಾಬ\u{ccd}ಲಾಂಕಾ"), ("ko", "카사블랑카"), ("ky", "Касабланка"), ("lo", "ກາຊາບ\u{eb1}ງກາ"), ("lt", "Kasablanka"), ("lv", "Kasablanka"), ("mk", "Казабланка"), ("ml", "ക\u{d3e}സബ\u{d4d}ലങ\u{d4d}ക"), ("mn", "Касабланка"), ("mr", "कासाब\u{94d}ला\u{902}का"), ("ms", "Casablanca"), ("my", "ကာဆာဘလန\u{103a}ကာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Casablanca"), ("nl", "Casablanca"), ("no", "Casablanca"), ("pa", "ਕਾਸਾਬਲਾ\u{a02}ਕਾ"), ("pl", "Casablanca"), ("pt", "Casablanca"), ("ro", "Casablanca"), ("ru", "Касабланка"), ("si", "කැසබ\u{dca}ලැන\u{dca}ක\u{dcf}"), ("sk", "Casablanca"), ("sl", "Casablanca"), ("sq", "Kasablanka"), ("sr", "Казабланка"), ("sr_Latn", "Kazablanka"), ("sv", "Casablanca"), ("sw", "Casablanca"), ("ta", "க\u{bbe}ச\u{bbe}பிள\u{bbe}ங\u{bcd}க\u{bbe}"), ("te", "క\u{c3e}స\u{c3e}బ\u{c4d}ల\u{c3e}ంక\u{c3e}"), ("th", "กาซาบล\u{e47}องกา"), ("tk", "Kasablanka"), ("tr", "Kazablanka"), ("uk", "Касабланка"), ("ur", "دار البیضاء"), ("uz", "Kasablanka"), ("vi", "Casablanca"), ("yo", "Casablanca"), ("yo_BJ", "Casablanca"), ("yue", "卡薩布蘭卡"), ("yue_Hans", "卡萨布兰卡"), ("zh", "卡萨布兰卡")]),
                        unofficial_name_list: ["Casablanca [Dar el Beïda]"].to_vec(),
                    }
                ),
                (
                    "CHE",
                    Subdivision{
                        name: "CHE",
                        country_alpha2: Alpha2::MA,
                        code: "CHE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.171389), longitude: Some(-5.269722000000001), max_latitude: Some(35.185559), min_latitude: Some(35.152179), max_longitude: Some(-5.2556705), min_longitude: Some(-5.287148999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شفشاون"), ("ca", "Província de Xauen"), ("ccp", "𑄌𑄬𑄛\u{11134}𑄌𑄬𑄠\u{1112f}𑄠𑄬𑄚\u{11134}"), ("ceb", "Chefchaouen Province"), ("de", "Chefchaouen (Provinz)"), ("en", "Chefchaouen"), ("es", "Provincia de Chauen"), ("fr", "province de Chefchaouen"), ("it", "Provincia di Chefchaouen"), ("ja", "シャウエン州"), ("nl", "Chefchaouen"), ("sv", "Chefchaouen Province"), ("ur", "شفشاون صوبہ"), ("zh", "謝夫沙萬省")]),
                        unofficial_name_list: ["Chefchaouene"].to_vec(),
                    }
                ),
                (
                    "CHI",
                    Subdivision{
                        name: "CHI",
                        country_alpha2: Alpha2::MA,
                        code: "CHI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.533333), longitude: Some(-8.766667), max_latitude: Some(31.5541463), min_latitude: Some(31.5162154), max_longitude: Some(-8.7405252), min_longitude: Some(-8.779620999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شيشاوة"), ("ca", "Província de Chichaoua"), ("ccp", "𑄌\u{11128}𑄌\u{1112f}𑄤"), ("ceb", "Chichaoua"), ("de", "Chichaoua (Provinz)"), ("en", "Chichaoua"), ("es", "Provincia de Chichaoua"), ("fa", "استان شیشاوه"), ("fr", "province de Chichaoua"), ("it", "Provincia di Chichaoua"), ("ja", "シカウア州"), ("nl", "Chichaoua"), ("pt", "Chichaoua (província)"), ("sv", "Chichaoua (provins)"), ("ur", "شیشاوہ صوبہ"), ("zh", "希沙瓦省")]),
                        unofficial_name_list: ["Chichaoua"].to_vec(),
                    }
                ),
                (
                    "CHT",
                    Subdivision{
                        name: "CHT",
                        country_alpha2: Alpha2::MA,
                        code: "CHT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.0688758), longitude: Some(-9.152534), max_latitude: Some(30.0738757), min_latitude: Some(30.0648089), max_longitude: Some(-9.1488218), min_longitude: Some(-9.166395699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شتوكة آيت باها"), ("ca", "Província de Chtouka-Aït Baha"), ("ccp", "𑄑\u{1112e}𑄇 𑄃𑄃\u{11128}𑄖\u{11134} 𑄝𑄦"), ("ceb", "Chtouka-Ait-Baha"), ("de", "Chtouka-Aït Baha"), ("en", "Chtouka Aït Baha"), ("es", "Provincia de Chtouka-Aït Baha"), ("eu", "Chtouka Aït Baha probintzia"), ("fr", "province de Chtouka-Aït Baha"), ("it", "Provincia di Chtouka-Aït Baha"), ("ja", "チュトウカ・アイト・バハ州"), ("nl", "Chtouka-Aït Baha"), ("pt", "Chtouka-Aït Baha"), ("sv", "Chtouka-Ait-Baha"), ("zh", "希圖卡阿伊特巴哈省")]),
                        unofficial_name_list: ["Chtouka-Ait Baha"].to_vec(),
                    }
                ),
                (
                    "DRI",
                    Subdivision{
                        name: "DRI",
                        country_alpha2: Alpha2::MA,
                        code: "DRI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Driouch")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ERR",
                    Subdivision{
                        name: "ERR",
                        country_alpha2: Alpha2::MA,
                        code: "ERR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.931944), longitude: Some(-4.424443999999999), max_latitude: Some(31.9623573), min_latitude: Some(31.9077638), max_longitude: Some(-4.3753052), min_longitude: Some(-4.4768859)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الرشيدية"), ("ca", "Província d’Errachidia"), ("ccp", "𑄃\u{11129}𑄢𑄌\u{11128}𑄘\u{11128}𑄠"), ("ceb", "Errachidia"), ("de", "Errachidia (Provinz)"), ("en", "Errachidia"), ("es", "Provincia de Errachidia"), ("fa", "استان رشیدیه"), ("fr", "province d’Errachidia"), ("it", "Provincia di al-Rashidiyya"), ("ja", "エルラシディア州"), ("nl", "Errachidia"), ("sv", "Errachidia"), ("zh", "拉希迪耶省")]),
                        unofficial_name_list: ["Errachidia"].to_vec(),
                    }
                ),
                (
                    "ESI",
                    Subdivision{
                        name: "ESI",
                        country_alpha2: Alpha2::MA,
                        code: "ESI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.5084926), longitude: Some(-9.7595041), max_latitude: Some(31.5268173), min_latitude: Some(31.4950698), max_longitude: Some(-9.7492435), min_longitude: Some(-9.7755215)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الصويرة"), ("ca", "Província d’Essaouira"), ("ccp", "𑄃\u{11128}𑄥\u{1112f}𑄃\u{11128}𑄢"), ("ceb", "Essaouira"), ("de", "Essaouira (Provinz)"), ("en", "Essaouira"), ("es", "Provincia de Esauira"), ("fa", "استان صویره"), ("fi", "Essaouiran provinssi"), ("fr", "province d’Essaouira"), ("it", "Provincia di Essaouira"), ("ja", "エッサウィラ州"), ("nl", "Essaouira"), ("pt", "Essaouira (província)"), ("ur", "صویرہ صوبہ"), ("zh", "索維拉省")]),
                        unofficial_name_list: ["Essaouira"].to_vec(),
                    }
                ),
                (
                    "ESM",
                    Subdivision{
                        name: "ESM",
                        country_alpha2: Alpha2::MA,
                        code: "ESM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.7082053), longitude: Some(-9.5450974), max_latitude: Some(30.4073509), min_latitude: Some(27.6666665), max_longitude: Some(-6.263147), min_longitude: Some(-11.1281028)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم السمارة"), ("ca", "Província d’Es-Semara"), ("ccp", "𑄃\u{11128}𑄌\u{11134} 𑄥𑄬𑄟𑄢"), ("ceb", "Es-Semara (lalawigan)"), ("de", "Es Semara (Provinz)"), ("en", "Es Semara"), ("es", "Provincia de Esmara"), ("fr", "province d’Es-Semara"), ("it", "Provincia di Smara"), ("ja", "スマラ州"), ("nl", "Es-Semara"), ("sv", "Es-Semara (provins)"), ("zh", "塞馬拉省")]),
                        unofficial_name_list: ["Es Smara (EH)"].to_vec(),
                    }
                ),
                (
                    "FAH",
                    Subdivision{
                        name: "FAH",
                        country_alpha2: Alpha2::MA,
                        code: "FAH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.757668), longitude: Some(-5.8109107), max_latitude: Some(35.7583864), min_latitude: Some(35.757346), max_longitude: Some(-5.8102681), min_longitude: Some(-5.8115019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فحص أنجرة"), ("ca", "Província de Fahs-Anjra"), ("ccp", "𑄜𑄦\u{11134}𑄥\u{11134}-𑄝𑄬𑄚\u{11128} 𑄟𑄖𑄓"), ("ceb", "Fahs-Anjra"), ("de", "Fahs-Anjra"), ("en", "Fahs-Beni Makada"), ("es", "Prefectura de Fahs Anjra"), ("fa", "استان فحص انجره"), ("fr", "province de Fahs-Anjra"), ("it", "Prefettura di Fahs Anjra"), ("ja", "ファフス・アンジュラ州"), ("nl", "Fahs-Bni Mkada"), ("pl", "prowincja Fahs Anjra"), ("sv", "Fahs-Anjra"), ("ur", "فاہس انجرا"), ("zh", "法斯-安傑拉省")]),
                        unofficial_name_list: ["Fahs-Beni Makada"].to_vec(),
                    }
                ),
                (
                    "FES",
                    Subdivision{
                        name: "FES",
                        country_alpha2: Alpha2::MA,
                        code: "FES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.033333), longitude: Some(-5.0), max_latitude: Some(34.075377), min_latitude: Some(33.9725487), max_longitude: Some(-4.927883), min_longitude: Some(-5.0755978)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Fes"), ("ar", "فاس"), ("az", "Fəs"), ("be", "Горад Фес"), ("bg", "Фес"), ("bs", "Fes"), ("ca", "Fes"), ("ccp", "𑄜𑄬𑄌\u{11134}-𑄓𑄢\u{11134}-𑄝\u{11128}𑄝𑄬𑄇\u{11134}"), ("ceb", "Fès"), ("cs", "Fás"), ("cy", "Fès"), ("da", "Fez"), ("de", "Fès"), ("el", "Φεζ"), ("en", "Fès-Dar-Dbibegh"), ("es", "Fez"), ("et", "Fès"), ("eu", "Fez"), ("fa", "فاس"), ("fi", "Fès"), ("fr", "Fès"), ("gl", "Fez"), ("he", "פס"), ("hi", "फ\u{947}ज\u{93c}"), ("hr", "Fes"), ("hu", "Fez"), ("hy", "Ֆես"), ("id", "Fez"), ("is", "Fez"), ("it", "Fes"), ("ja", "フェズ"), ("ka", "ფესი"), ("kk", "Фес (Фез) қаласы"), ("ko", "페스"), ("ky", "Фес"), ("lt", "Fesas"), ("lv", "Fēsa"), ("mk", "Фес"), ("ml", "ഫെസ\u{d4d}, മൊറോക\u{d4d}കോ"), ("ms", "Fes"), ("nb", "Fès"), ("ne", "फ\u{947}स एल बाली"), ("nl", "Fez"), ("no", "Fès"), ("pa", "ਫ\u{a3c}ਾਸ"), ("pl", "Fez"), ("pt", "Fez"), ("ro", "Fès"), ("ru", "Фес"), ("sl", "Fes"), ("sr", "Фес"), ("sr_Latn", "Fes"), ("sv", "Fès"), ("sw", "Fes"), ("th", "แฟ\u{e47}ส"), ("tr", "Fes"), ("uk", "Фес"), ("ur", "فاس"), ("vi", "Fes"), ("zh", "非斯")]),
                        unofficial_name_list: ["Fès"].to_vec(),
                    }
                ),
                (
                    "FIG",
                    Subdivision{
                        name: "FIG",
                        country_alpha2: Alpha2::MA,
                        code: "FIG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.1092613), longitude: Some(-1.229806), max_latitude: Some(32.1282694), min_latitude: Some(32.082611), max_longitude: Some(-1.1983252), min_longitude: Some(-1.2536429)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فكيك"), ("ca", "Província de Figuig"), ("ccp", "𑄜\u{11128}𑄉\u{1112a}𑄃\u{11128}𑄇\u{11134}"), ("ceb", "Figuig (lalawigan)"), ("de", "Figuig"), ("en", "Figuig"), ("es", "Provincia de Figuig"), ("fa", "استان فکیک"), ("fr", "province de Figuig"), ("it", "Provincia di Figuig"), ("ja", "フィギグ州"), ("nl", "Figuig"), ("sv", "Figuig"), ("ur", "فکیک صوبہ"), ("zh", "菲吉格省")]),
                        unofficial_name_list: ["Figuig"].to_vec(),
                    }
                ),
                (
                    "FQH",
                    Subdivision{
                        name: "FQH",
                        country_alpha2: Alpha2::MA,
                        code: "FQH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.791702), longitude: Some(-7.092619999999999), max_latitude: Some(35.9344), min_latitude: Some(27.6672693), max_longitude: Some(-0.9969759), min_longitude: Some(-13.3044001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Fquih Ben Salah")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GUE",
                    Subdivision{
                        name: "GUE",
                        country_alpha2: Alpha2::MA,
                        code: "GUE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.98333299999999), longitude: Some(-10.066667), max_latitude: Some(29.0155338), min_latitude: Some(28.9644067), max_longitude: Some(-10.0353242), min_longitude: Some(-10.0787115)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كلميم"), ("bn", "গ\u{9c1}য\u{9bc}েল\u{9cd}মিম প\u{9cd}রদেশ"), ("ca", "Província de Guelmim"), ("ccp", "𑄉\u{1112a}𑄠𑄬𑄣\u{11134}𑄟\u{11128}𑄟\u{11134}"), ("ceb", "Guelmim (lalawigan)"), ("da", "Guelmim"), ("de", "Guelmim"), ("el", "Γκουελμίμ"), ("en", "Guelmim"), ("es", "Provincia de Guelmim"), ("fi", "Guelmim Province"), ("fr", "province de Guelmim"), ("gu", "ગ\u{ac7}લમીમ પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{941}लमीम प\u{94d}रोवि\u{902}स"), ("id", "Guelmim Province"), ("it", "Provincia di Guelmim"), ("ja", "ゲルミン州"), ("kn", "ಗುಲ\u{ccd}ಮ\u{cbf}ಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "구엘밈 프로빈스"), ("lt", "Guelmimo provincija"), ("lv", "Guelmimas province"), ("mr", "ग\u{947}लमिक प\u{94d}रा\u{902}त"), ("ms", "Guelmim Province"), ("nb", "Guelmim Kommune"), ("nl", "Guelmim"), ("no", "Guelmim Kommune"), ("pl", "Prowincja Guelmim"), ("pt", "Província de Guelmim"), ("ru", "Гельмим"), ("si", "ග\u{dd4}ලෙම\u{dd2}ම\u{dca} පළ\u{dcf}ත"), ("sv", "Guelmim"), ("ta", "குயல\u{bcd}ம\u{bc0}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}యుల\u{c4d}మ\u{c3f}మ\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రభ\u{c3e}గం"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e39}ลม\u{e34}ม"), ("tr", "Guelmim Province"), ("uk", "Гельмім"), ("ur", "جویلمیم صوبہ"), ("vi", "Tỉnh Guelmim"), ("zh", "蓋勒敏省")]),
                        unofficial_name_list: ["Guelmim"].to_vec(),
                    }
                ),
                (
                    "GUF",
                    Subdivision{
                        name: "GUF",
                        country_alpha2: Alpha2::MA,
                        code: "GUF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5059889), longitude: Some(2.5307101), max_latitude: Some(39.5128789), min_latitude: Some(39.5016927), max_longitude: Some(2.5327309), min_longitude: Some(2.5269251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Guercif")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "HAJ",
                    Subdivision{
                        name: "HAJ",
                        country_alpha2: Alpha2::MA,
                        code: "HAJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.692778), longitude: Some(-5.371111), max_latitude: Some(33.7026764), min_latitude: Some(33.6618291), max_longitude: Some(-5.358757), min_longitude: Some(-5.3847214)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الحاجب"), ("ca", "Província d’El Hajeb"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄝𑄎\u{11128}𑄛\u{11134}"), ("ceb", "El-Hajeb"), ("de", "El Hajeb"), ("en", "El Hajeb"), ("es", "Provincia de El Hayeb"), ("fr", "province d’El Hajeb"), ("it", "Provincia di El Hajeb"), ("ja", "ハジェブ州"), ("nl", "El Hajeb"), ("zh", "哈傑卜省")]),
                        unofficial_name_list: ["El Hajeb"].to_vec(),
                    }
                ),
                (
                    "HAO",
                    Subdivision{
                        name: "HAO",
                        country_alpha2: Alpha2::MA,
                        code: "HAO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.2956729), longitude: Some(-7.872159999999999), max_latitude: Some(31.7052412), min_latitude: Some(30.862742), max_longitude: Some(-7.0889282), min_longitude: Some(-8.5580064)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الحوز"), ("ca", "Província d’Al Haouz"), ("ccp", "𑄃𑄣\u{11134} 𑄦𑄅\u{1112a}𑄌\u{11134}"), ("ceb", "Al-Haouz"), ("de", "Al Haouz"), ("en", "Al Haouz"), ("es", "Provincia de Al Hauz"), ("fa", "اقلیم الحوز"), ("fr", "province d’Al Haouz"), ("it", "Provincia di Al Haouz"), ("ja", "ハオウズ"), ("nl", "Al Haouz"), ("pl", "prowincja Al Haouz"), ("pt", "Al Haouz"), ("sv", "Al-Haouz"), ("ur", "الحوز صوبہ"), ("zh", "豪茲省")]),
                        unofficial_name_list: ["Al Haouz"].to_vec(),
                    }
                ),
                (
                    "HOC",
                    Subdivision{
                        name: "HOC",
                        country_alpha2: Alpha2::MA,
                        code: "HOC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.25), longitude: Some(-3.933333), max_latitude: Some(35.2618515), min_latitude: Some(35.2178035), max_longitude: Some(-3.9094573), min_longitude: Some(-3.9659865)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الحسيمة"), ("ca", "Província d’Al Hoceima"), ("ccp", "𑄃𑄣\u{11134} 𑄦\u{1112e}𑄥𑄬\u{1112d}𑄟"), ("ceb", "Al-Hoceima"), ("de", "Al Hoceïma"), ("en", "Al Hoceïma"), ("es", "Provincia de Alhucemas"), ("fr", "province d’Al Hoceïma"), ("gl", "Provincia de Al-Hoceima"), ("it", "Provincia di Al-Hoseyma"), ("ja", "アル・ホセイマ州"), ("nl", "Al Hoceima"), ("ru", "Эль-Хосейма (провинция)"), ("sv", "Al-Hoceima (provins i Marocko)"), ("ur", "الحسیمہ صوبہ"), ("zh", "胡塞馬省")]),
                        unofficial_name_list: ["Al Hoceïma"].to_vec(),
                    }
                ),
                (
                    "IFR",
                    Subdivision{
                        name: "IFR",
                        country_alpha2: Alpha2::MA,
                        code: "IFR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5228062), longitude: Some(-5.1109552), max_latitude: Some(33.5549144), min_latitude: Some(33.4804938), max_longitude: Some(-5.086154899999999), min_longitude: Some(-5.169239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم إفران"), ("ca", "Província d’Ifrane"), ("ccp", "𑄃\u{11128}𑄜\u{11133}𑄢\u{1112d}𑄚\u{11134}"), ("ceb", "Ifrane (lalawigan)"), ("de", "Ifrane"), ("en", "Ifrane"), ("es", "Provincia de Ifrane"), ("fr", "province d’Ifrane"), ("it", "Provincia di Ifrane"), ("ja", "イフレン州"), ("nl", "Ifrane"), ("ru", "Ифран (провинция)"), ("sv", "Ifrane"), ("zh", "伊夫蘭省")]),
                        unofficial_name_list: ["Ifrane"].to_vec(),
                    }
                ),
                (
                    "INE",
                    Subdivision{
                        name: "INE",
                        country_alpha2: Alpha2::MA,
                        code: "INE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "عمالة إنزكان آيت ملول"), ("ca", "Prefectura d’Inezgane-Aït Melloul"), ("ccp", "𑄃\u{11128}𑄚𑄬𑄌\u{11134}𑄉𑄚\u{11134}-𑄃𑄃\u{11128}𑄖\u{11134} 𑄟𑄣𑄃\u{1112f}𑄣\u{11134}"), ("ceb", "Inezgane-Ait Mellou"), ("de", "Inezgane-Aït Melloul"), ("en", "Inezgane-Aït Melloul"), ("es", "Prefectura Inezgane-Aït Melloul"), ("eu", "Inezgane-Ait Melloul prefektura"), ("fr", "préfecture d’Inezgane-Aït Melloul"), ("it", "Prefettura di Inezgane-Aït Melloul"), ("ja", "イネガーヌ・アイト・メロウル県"), ("nl", "Inezgane-Aït Melloul"), ("pt", "Inezgane-Aït Melloul"), ("sv", "Inezgane-Ait Mellou"), ("ur", "انزکان آیت ملول"), ("zh", "因茲甘-阿伊特邁盧勒省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JDI",
                    Subdivision{
                        name: "JDI",
                        country_alpha2: Alpha2::MA,
                        code: "JDI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.233333), longitude: Some(-8.5), max_latitude: Some(33.2641124), min_latitude: Some(33.202792), max_longitude: Some(-8.4659775), min_longitude: Some(-8.5413515)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الجديدة"), ("ca", "Província d’El Jadida"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄎𑄓\u{11128}𑄘"), ("ceb", "El-Jadida (lalawigan)"), ("de", "El Jadida"), ("en", "El Jadida"), ("es", "Provincia de El Yadida"), ("fr", "province d’El Jadida"), ("it", "Provincia di El Jadida"), ("ja", "アル・ジャディーダ州"), ("nl", "El Jadida"), ("sv", "El-Jadida"), ("zh", "傑迪代省")]),
                        unofficial_name_list: ["El Jadida"].to_vec(),
                    }
                ),
                (
                    "JRA",
                    Subdivision{
                        name: "JRA",
                        country_alpha2: Alpha2::MA,
                        code: "JRA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.311667), longitude: Some(-2.163611), max_latitude: Some(34.3264968), min_latitude: Some(34.2897364), max_longitude: Some(-2.1487284), min_longitude: Some(-2.2159766)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جرادة"), ("ca", "Província de Jerada"), ("ccp", "𑄎𑄬𑄢𑄓"), ("ceb", "Jerada (lalawigan)"), ("de", "Jerada (Provinz)"), ("en", "Jerada"), ("es", "Provincia de Yerada"), ("fa", "استان جراده"), ("fr", "province de Jerada"), ("it", "Provincia di Jerada"), ("ja", "ジェラダ州"), ("nl", "Jerada"), ("sv", "Jerada (provins)"), ("ur", "جرادہ صوبہ"), ("zh", "傑拉達省")]),
                        unofficial_name_list: ["Jerada"].to_vec(),
                    }
                ),
                (
                    "KEN",
                    Subdivision{
                        name: "KEN",
                        country_alpha2: Alpha2::MA,
                        code: "KEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.25), longitude: Some(-6.583333), max_latitude: Some(34.310032), min_latitude: Some(34.2233904), max_longitude: Some(-6.5178107), min_longitude: Some(-6.682949)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم القنيطرة"), ("ca", "Província de Kénitra"), ("ccp", "𑄇𑄬𑄚\u{11128}𑄑\u{11133}𑄢"), ("ceb", "Kenitra Province"), ("de", "Kénitra"), ("en", "Kénitra"), ("es", "Provincia de Kenitra"), ("fa", "استان قنیطره"), ("fr", "province de Kénitra"), ("it", "Provincia di Kenitra"), ("ja", "ケニトラ州"), ("nl", "Kénitra"), ("ru", "провинция Кенитра"), ("sv", "Kenitra Province"), ("ur", "قنیطرہ صوبہ"), ("zh", "蓋尼特拉省")]),
                        unofficial_name_list: ["Kénitra"].to_vec(),
                    }
                ),
                (
                    "KES",
                    Subdivision{
                        name: "KES",
                        country_alpha2: Alpha2::MA,
                        code: "KES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.048056), longitude: Some(-7.408333000000001), max_latitude: Some(32.0710322), min_latitude: Some(32.0310284), max_longitude: Some(-7.3746578), min_longitude: Some(-7.431478500000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم قلعة السراغنة"), ("ca", "Província d’El Kelâa des Sraghna"), ("ccp", "𑄇𑄬𑄣𑄖\u{11134} 𑄥\u{11133}𑄢𑄇\u{11134}𑄚"), ("ceb", "Kelaa-Des-Sraghna"), ("de", "El Kelaâ des Sraghna"), ("en", "Kelaat Sraghna"), ("es", "Provincia de El Kelaa des Sraghna"), ("fa", "استان قلعه سراغنه"), ("fr", "province d’El Kelaâ des Sraghna"), ("it", "Provincia di El Kelâat Es-Sraghna"), ("ja", "エル・ケッラ・デ・スラーナ州"), ("nl", "Kelâat Es-Sraghna"), ("pt", "El Kelâat Es-Sraghna"), ("sv", "Kelaa-Des-Sraghna"), ("ur", "القعہ سراغنہ صوبہ"), ("zh", "斯拉格奈堡省")]),
                        unofficial_name_list: ["Kelaat Sraghna"].to_vec(),
                    }
                ),
                (
                    "KHE",
                    Subdivision{
                        name: "KHE",
                        country_alpha2: Alpha2::MA,
                        code: "KHE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.816667), longitude: Some(-6.066667), max_latitude: Some(33.848522), min_latitude: Some(33.7989952), max_longitude: Some(-6.0419214), min_longitude: Some(-6.1040727)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم الخميسات"), ("ca", "Província de Khémisset"), ("ccp", "𑄈𑄬𑄟\u{11128}𑄥𑄬𑄖\u{11134}"), ("ceb", "Khemisset (lalawigan)"), ("de", "Khémisset (Provinz)"), ("en", "Khemisset"), ("es", "Provincia de Jemisset"), ("fr", "province de Khémisset"), ("it", "Provincia di Khemisset"), ("ja", "ケミセット州"), ("nl", "Khémisset"), ("sv", "Khemisset (provins i Marocko)"), ("zh", "海米薩特省")]),
                        unofficial_name_list: ["Khemisset"].to_vec(),
                    }
                ),
                (
                    "KHN",
                    Subdivision{
                        name: "KHN",
                        country_alpha2: Alpha2::MA,
                        code: "KHN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.939444), longitude: Some(-5.6675), max_latitude: Some(32.9613621), min_latitude: Some(32.9063255), max_longitude: Some(-5.6215668), min_longitude: Some(-5.691261300000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم خنيفرة"), ("ca", "Província de Khénifra"), ("ccp", "𑄈𑄬𑄚\u{11128}𑄜\u{11133}𑄢"), ("ceb", "Khenifra (lalawigan)"), ("de", "Khénifra"), ("en", "Khénifra"), ("es", "Provincia de Jenifra"), ("fr", "province de Khénifra"), ("it", "Provincia di Khenifra"), ("ja", "ヘニフラ州"), ("nl", "Khénifra"), ("zh", "海尼夫拉省")]),
                        unofficial_name_list: ["Khenifra"].to_vec(),
                    }
                ),
                (
                    "KHO",
                    Subdivision{
                        name: "KHO",
                        country_alpha2: Alpha2::MA,
                        code: "KHO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.886023), longitude: Some(-6.9208655), max_latitude: Some(32.9182217), min_latitude: Some(32.8434018), max_longitude: Some(-6.8709799), min_longitude: Some(-6.963607000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم خريبكة"), ("ca", "Província de Khouribga"), ("ccp", "𑄈\u{1112f}𑄢\u{11128}𑄛\u{11134}𑄉"), ("ceb", "Khouribga Province"), ("de", "Khouribga (Provinz)"), ("en", "Khouribga"), ("es", "Provincia de Juribga"), ("fr", "province de Khouribga"), ("it", "Provincia di Khouribga"), ("ja", "クーリブカ州"), ("nl", "Khouribga (provincie)"), ("sv", "Khouribga Province"), ("zh", "胡里卜蓋省")]),
                        unofficial_name_list: ["Khouribga"].to_vec(),
                    }
                ),
                (
                    "LAA",
                    Subdivision{
                        name: "LAA",
                        country_alpha2: Alpha2::MA,
                        code: "LAA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.1252867), longitude: Some(-13.1625005), max_latitude: Some(27.1669433), min_latitude: Some(27.0811371), max_longitude: Some(-13.0969906), min_longitude: Some(-13.2541466)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم العيون"), ("ca", "Província d’Al-Aaiun"), ("ccp", "𑄣𑄬𑄃\u{1112f}𑄚\u{11134}"), ("ceb", "Laayoune (lalawigan)"), ("de", "Laâyoune"), ("en", "Laâyoune"), ("es", "Provincia de El Aaiún"), ("fr", "province de Laâyoune"), ("it", "Provincia di Laâyoune"), ("ja", "アイウン州"), ("nl", "Laâyoune"), ("pt", "Laâyoune"), ("sv", "Laayoune (provins)"), ("zh", "阿尤恩省")]),
                        unofficial_name_list: ["Laâyoune* (EH)"].to_vec(),
                    }
                ),
                (
                    "LAR",
                    Subdivision{
                        name: "LAR",
                        country_alpha2: Alpha2::MA,
                        code: "LAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.183333), longitude: Some(-6.15), max_latitude: Some(35.2061451), min_latitude: Some(35.1456698), max_longitude: Some(-6.129512800000001), min_longitude: Some(-6.1814405)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم العرائش"), ("be", "Лараш"), ("ca", "Província de Larraix"), ("ccp", "𑄣𑄢\u{11134}𑄥𑄬"), ("ceb", "Larache (lalawigan)"), ("de", "Larache"), ("en", "Larache"), ("es", "Larache"), ("fr", "Larache"), ("it", "Provincia di Larache"), ("ja", "アライシュ州"), ("nl", "Larache"), ("ru", "Лараш"), ("sv", "Larache (provins)"), ("ur", "العرائش صوبہ"), ("zh", "拉臘什省")]),
                        unofficial_name_list: ["Larache"].to_vec(),
                    }
                ),
                (
                    "MAR",
                    Subdivision{
                        name: "MAR",
                        country_alpha2: Alpha2::MA,
                        code: "MAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.63), longitude: Some(-8.008889), max_latitude: Some(31.7162668), min_latitude: Some(31.5529761), max_longitude: Some(-7.887625799999999), min_longitude: Some(-8.1280804)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Marrakech")]),
                        unofficial_name_list: ["Marrakech"].to_vec(),
                    }
                ),
                (
                    "MDF",
                    Subdivision{
                        name: "MDF",
                        country_alpha2: Alpha2::MA,
                        code: "MDF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5521196), longitude: Some(-7.4780258), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "M’diq-Fnideq")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MED",
                    Subdivision{
                        name: "MED",
                        country_alpha2: Alpha2::MA,
                        code: "MED",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.4525), longitude: Some(-7.514722), max_latitude: Some(33.4622099), min_latitude: Some(33.44092), max_longitude: Some(-7.508552099999999), min_longitude: Some(-7.526265299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مديونة"), ("ca", "Província de Médiouna"), ("ccp", "𑄟𑄬𑄓\u{11128}𑄃\u{1112f}𑄚"), ("ceb", "Mediouna"), ("de", "Médiouna (Provinz)"), ("en", "Médiouna"), ("es", "Provincia de Mediuna"), ("fr", "province de Médiouna"), ("it", "Provincia di Mediouna"), ("ja", "メディウナ州"), ("nl", "Médiouna"), ("zh", "梅久那省")]),
                        unofficial_name_list: ["Médiouna"].to_vec(),
                    }
                ),
                (
                    "MEK",
                    Subdivision{
                        name: "MEK",
                        country_alpha2: Alpha2::MA,
                        code: "MEK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.895), longitude: Some(-5.554722), max_latitude: Some(33.9281185), min_latitude: Some(33.8338915), max_longitude: Some(-5.466414599999999), min_longitude: Some(-5.6096258)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مكناس"), ("be", "Горад Мекнес"), ("bg", "Мекнес"), ("bn", "মেকন\u{9be}স"), ("ca", "Meknès"), ("ccp", "𑄟𑄬𑄇\u{11134}𑄚\u{11128}𑄌\u{11134}"), ("ceb", "Meknès"), ("cs", "Meknes"), ("cy", "Meknès"), ("da", "Meknès"), ("de", "Meknès"), ("el", "Μεκνές"), ("en", "Meknès"), ("es", "Mequinez"), ("eu", "Meknes"), ("fa", "مکناس"), ("fi", "Meknes"), ("fr", "Meknès"), ("gu", "મ\u{ac7}કન\u{ac7}સ"), ("he", "מקנס"), ("hi", "म\u{947}कन\u{947}स"), ("hr", "Meknes"), ("hu", "Meknesz"), ("hy", "Մեկնես"), ("id", "Meknes"), ("it", "Meknes"), ("ja", "メクネス"), ("ka", "მეკნესი"), ("kk", "Мекнес қаласы"), ("kn", "ಮ\u{cc6}ಕ\u{ccd}ನ\u{cc6}ಸ\u{ccd}"), ("ko", "메크네스"), ("lt", "Meknesas"), ("lv", "Meknesa"), ("mk", "Мекнес"), ("mr", "म\u{947}कन\u{947}स"), ("ms", "Meknes"), ("nb", "Meknes"), ("ne", "म\u{947}कन\u{947}स"), ("nl", "Meknes"), ("no", "Meknes"), ("pl", "Meknes"), ("pt", "Meknès"), ("ro", "Meknès"), ("ru", "Мекнес"), ("si", "මෙක\u{dca}නෙස\u{dca}"), ("sr", "Мекнес"), ("sr_Latn", "Meknes"), ("sv", "Meknès"), ("ta", "மெக\u{bcd}ன\u{bbe}ஸ\u{bcd}"), ("te", "మ\u{c46}క\u{c4d}న\u{c46}స\u{c4d}"), ("th", "แม\u{e47}กแน\u{e47}ส"), ("tr", "Meknes"), ("uk", "Мекнес"), ("ur", "مکناس"), ("uz", "Miknas"), ("vi", "Meknes"), ("zh", "梅克内斯")]),
                        unofficial_name_list: ["Meknès*"].to_vec(),
                    }
                ),
                (
                    "MEL",
                    Subdivision{
                        name: "MEL",
                        country_alpha2: Alpha2::MA,
                        code: "MEL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.334167), longitude: Some(-9.497221999999999), max_latitude: Some(30.3755411), min_latitude: Some(30.3045765), max_longitude: Some(-9.4401741), min_longitude: Some(-9.5496081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Aït Melloul")]),
                        unofficial_name_list: ["Aït Melloul"].to_vec(),
                    }
                ),
                (
                    "MID",
                    Subdivision{
                        name: "MID",
                        country_alpha2: Alpha2::MA,
                        code: "MID",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.791702), longitude: Some(-7.092619999999999), max_latitude: Some(35.9344), min_latitude: Some(27.6672693), max_longitude: Some(-0.9969759), min_longitude: Some(-13.3044001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Midelt")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MMD",
                    Subdivision{
                        name: "MMD",
                        country_alpha2: Alpha2::MA,
                        code: "MMD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Marrakesj"), ("ar", "مراكش"), ("be", "Маракеш"), ("bg", "Маракеш"), ("bn", "ম\u{9be}র\u{9be}ক\u{9cd}কেশ"), ("ca", "Marràqueix"), ("ccp", "𑄟𑄢𑄇𑄬𑄌\u{11134}-𑄟𑄬𑄓\u{11128}𑄚"), ("cs", "Marrákeš"), ("da", "Marrakech"), ("de", "Marrakesch"), ("el", "Μαρακές"), ("en", "Marrakech-Medina"), ("es", "Marrakech"), ("et", "Marrakech"), ("eu", "Marrakex"), ("fa", "مراکش"), ("fi", "Marrakech"), ("fr", "Marrakech"), ("gl", "Marrakech"), ("gu", "મારાક\u{ac7}શ"), ("he", "מרקש"), ("hi", "मराक\u{947}श"), ("hr", "Marrakech"), ("hu", "Marrákes"), ("hy", "Մառակեշ"), ("id", "Marrakesh"), ("is", "Marrakess"), ("it", "Marrakech"), ("ja", "マラケシュ"), ("ka", "მარაკეში"), ("kn", "ಮಾರಕೇಶ\u{ccd}"), ("ko", "마라케시"), ("lt", "Marakešas"), ("lv", "Marrākeša"), ("ml", "മര\u{d3e}ക\u{d4d}കേഷ\u{d4d}"), ("mr", "माराक\u{947}श"), ("ms", "Marrakesh"), ("nb", "Marrakech"), ("ne", "मर\u{94d}राक\u{947}शको मदिना"), ("nl", "Marrakesh"), ("no", "Marrakech"), ("pl", "Marrakesz"), ("pt", "Marraquexe"), ("ro", "Marrakech"), ("ru", "Марракеш"), ("si", "මරකෙෂ\u{dca}"), ("sk", "Murrákuš"), ("sl", "Marakeš, Maroko"), ("sr", "Маракеш"), ("sr_Latn", "Marakeš"), ("sv", "Marrakech"), ("ta", "மரர\u{bbe}கேஷ\u{bcd}"), ("te", "మర\u{c3e}క\u{c47}శ\u{c4d}"), ("th", "มาร\u{e4c}ราค\u{e34}ช"), ("tr", "Marakeş"), ("uk", "Марракеш"), ("ur", "مراکش (شہر)"), ("vi", "Marrakech"), ("zh", "马拉喀什")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MMN",
                    Subdivision{
                        name: "MMN",
                        country_alpha2: Alpha2::MA,
                        code: "MMN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Marrakesj²"), ("ar", "مراكش²"), ("be", "Маракеш²"), ("bg", "Маракеш²"), ("bn", "ম\u{9be}র\u{9be}ক\u{9cd}কেশ²"), ("ca", "Marràqueix²"), ("ccp", "𑄟𑄢𑄇𑄬𑄌\u{11134}-𑄟𑄬𑄚𑄢"), ("cs", "Marrákeš²"), ("da", "Marrakech²"), ("de", "Marrakesch²"), ("el", "Μαρακές²"), ("en", "Marrakech-Menara"), ("es", "Marrakech²"), ("et", "Marrakech²"), ("eu", "Marrakex²"), ("fa", "مراکش²"), ("fi", "Marrakech²"), ("fr", "Marrakech²"), ("gl", "Marrakech²"), ("gu", "મારાક\u{ac7}શ²"), ("he", "מרקש²"), ("hi", "मराक\u{947}श²"), ("hr", "Marrakech²"), ("hu", "Marrákes²"), ("hy", "Մառակեշ²"), ("id", "Marrakesh²"), ("is", "Marrakess²"), ("it", "Marrakech²"), ("ja", "マラケシュ²"), ("ka", "მარაკეში²"), ("kn", "ಮಾರಕೇಶ\u{ccd}²"), ("ko", "마라케시²"), ("lt", "Marakešas²"), ("lv", "Marrākeša²"), ("ml", "മര\u{d3e}ക\u{d4d}കേഷ\u{d4d}²"), ("mr", "माराक\u{947}श²"), ("ms", "Marrakesh²"), ("nb", "Marrakech²"), ("ne", "मर\u{94d}राक\u{947}शको मदिना²"), ("nl", "Marrakesh²"), ("no", "Marrakech²"), ("pl", "Marrakesz²"), ("pt", "Marraquexe²"), ("ro", "Marrakech²"), ("ru", "Марракеш²"), ("si", "මරකෙෂ\u{dca}²"), ("sk", "Murrákuš²"), ("sl", "Marakeš, Maroko²"), ("sr", "Маракеш²"), ("sr_Latn", "Marakeš²"), ("sv", "Marrakech²"), ("ta", "மரர\u{bbe}கேஷ\u{bcd}²"), ("te", "మర\u{c3e}క\u{c47}శ\u{c4d}²"), ("th", "มาร\u{e4c}ราค\u{e34}ช²"), ("tr", "Marakeş²"), ("uk", "Марракеш²"), ("ur", "مراکش (شہر)²"), ("vi", "Marrakech²"), ("zh", "马拉喀什²")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MOH",
                    Subdivision{
                        name: "MOH",
                        country_alpha2: Alpha2::MA,
                        code: "MOH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المحمدية"), ("az", "Məhəmmədiyyə"), ("bn", "মোহ\u{9be}ম\u{9cd}ম\u{9be}দিয\u{9bc}\u{9be}"), ("ca", "Mohammedia"), ("ccp", "𑄟\u{1112e}𑄦𑄟\u{11134}𑄟\u{11127}𑄘\u{11128}𑄠"), ("ceb", "Mohammedia"), ("cy", "Mohammédia"), ("da", "Mohammédia"), ("de", "Mohammedia"), ("el", "Μοχαμέντια"), ("en", "Mohammedia"), ("es", "Mohammédia"), ("eu", "Mohammedia"), ("fa", "محمدیه"), ("fi", "Mohammedia"), ("fr", "Mohammédia"), ("gu", "મોહમ\u{acd}મદિયા"), ("he", "מוחמדיה"), ("hi", "मोहम\u{94d}मदिया"), ("id", "Mohammedia"), ("it", "Mohammedia"), ("ja", "フェドハラ（モハメディア）"), ("kn", "ಮೊಹಮ\u{ccd}ಮದ\u{cbf}ಯಾ"), ("ko", "모하메디아"), ("lt", "Mohamedija"), ("lv", "Mohamedija"), ("mr", "मोहम\u{94d}मदिया"), ("ms", "Mohammedia"), ("nb", "Mohammedia"), ("nl", "Mohammedia"), ("no", "Mohammedia"), ("pl", "Al-Muhammadijja"), ("pt", "Mohammedia"), ("ro", "Mohammedia"), ("ru", "Мохаммедия"), ("si", "මෝහමෙද\u{dd2}ය\u{dcf}"), ("sq", "Mohammedia"), ("sv", "Mohammedia"), ("ta", "மொஹம\u{bcd}ம\u{bc0}டிய\u{bbe}"), ("te", "మ\u{c4a}హమ\u{c4d}మ\u{c46}ద\u{c3f}య\u{c3e}"), ("th", "ม\u{e39}ฮ\u{e31}มเมด\u{e34}อา"), ("tr", "Muhammediye"), ("uk", "Мохаммедія"), ("ur", "محمدیہ"), ("vi", "Mohammedia"), ("zh", "穆罕默迪耶")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MOU",
                    Subdivision{
                        name: "MOU",
                        country_alpha2: Alpha2::MA,
                        code: "MOU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0878), longitude: Some(-5.1811), max_latitude: Some(34.0926153), min_latitude: Some(34.0821661), max_longitude: Some(-5.1716422), min_longitude: Some(-5.1857186)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مولاي يعقوب"), ("ca", "Província de Moulay Yaâcoub"), ("ccp", "𑄟\u{1112f}𑄣𑄬 𑄃\u{11128}𑄠𑄇\u{1112a}𑄛\u{11134}"), ("ceb", "Moulay-Yacoub"), ("de", "Moulay Yacoub (Provinz)"), ("en", "Moulay Yacoub"), ("es", "Provincia de Mulay Yacub"), ("fr", "province de Moulay Yaâcoub"), ("it", "Prefettura di Moulay Yacoub"), ("ja", "ムーレイ・ヤコブ州"), ("nl", "Moulay Yacoub"), ("sv", "Moulay-Yacoub"), ("zh", "穆雷省")]),
                        unofficial_name_list: ["Moulay Yacoub"].to_vec(),
                    }
                ),
                (
                    "NAD",
                    Subdivision{
                        name: "NAD",
                        country_alpha2: Alpha2::MA,
                        code: "NAD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.166667), longitude: Some(-2.933333), max_latitude: Some(35.2065182), min_latitude: Some(35.1237717), max_longitude: Some(-2.9002642), min_longitude: Some(-2.9779407)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nador (provinsie)"), ("ar", "إقليم الناظور"), ("ca", "Província de Nador"), ("ccp", "𑄚𑄓\u{11127}𑄢\u{11134}"), ("ceb", "Nador (lalawigan)"), ("de", "Nador"), ("el", "Επαρχία του Ναντόρ"), ("en", "Nador"), ("es", "Provincia de Nador"), ("fa", "استان ناظور"), ("fr", "province de Nador"), ("it", "Provincia di Nador"), ("ja", "ナドール州"), ("nl", "Nador"), ("ru", "Надор"), ("sv", "Nador (provins)"), ("ur", "ناظور صوبہ"), ("zh", "納祖爾省")]),
                        unofficial_name_list: ["Nador"].to_vec(),
                    }
                ),
                (
                    "NOU",
                    Subdivision{
                        name: "NOU",
                        country_alpha2: Alpha2::MA,
                        code: "NOU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.3721834), longitude: Some(-7.547006599999999), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم النواصر"), ("ca", "Província de Nouaceur"), ("ccp", "𑄚\u{1112f}𑄃𑄥𑄬𑄅\u{1112a}𑄢\u{11134}"), ("ceb", "Nouaceur (lalawigan)"), ("de", "Nouaceur"), ("en", "Nouaceur"), ("es", "Provincia de Nouaceur"), ("fr", "province de Nouaceur"), ("it", "Provincia di Nouaceur"), ("ja", "ノウアセウル州"), ("nl", "Nouaceur"), ("ur", "صوبہ نواصر"), ("zh", "諾瓦瑟爾省")]),
                        unofficial_name_list: ["Nouaceur"].to_vec(),
                    }
                ),
                (
                    "OUA",
                    Subdivision{
                        name: "OUA",
                        country_alpha2: Alpha2::MA,
                        code: "OUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.9335436), longitude: Some(-6.937016), max_latitude: Some(30.9557508), min_latitude: Some(30.915294), max_longitude: Some(-6.8729387), min_longitude: Some(-6.99224)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ورززات"), ("ca", "Província de Ouarzazate"), ("ccp", "𑄇\u{1112f}𑄢\u{11134}𑄎𑄎𑄖\u{11134}"), ("ceb", "Ouarzazate"), ("de", "Ouarzazate"), ("en", "Ouarzazate"), ("es", "Provincia de Uarzazate"), ("fa", "استان ورززات"), ("fr", "province de Ouarzazate"), ("it", "Provincia di Ouarzazate"), ("ja", "ワルザザート州"), ("nl", "Ouarzazate"), ("pl", "prowincja Ouarzazate"), ("pt", "Ouarzazate"), ("zh", "瓦爾扎扎特省")]),
                        unofficial_name_list: ["Ouarzazate"].to_vec(),
                    }
                ),
                (
                    "OUD",
                    Subdivision{
                        name: "OUD",
                        country_alpha2: Alpha2::MA,
                        code: "OUD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.369212), longitude: Some(-16.9570741), max_latitude: Some(21.3696659), min_latitude: Some(21.3686192), max_longitude: Some(-16.9564694), min_longitude: Some(-16.9578964)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم وادي الذهب"), ("bn", "ওউড এড-ড\u{9be}হ\u{9be}ব প\u{9cd}রদেশ"), ("ca", "Província d’Oued Ed-Dahab"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄖\u{11134} 𑄃\u{11128}𑄖\u{11134}-𑄓𑄦𑄛\u{11134}"), ("ceb", "Oued Ed-Dahab-Lagouira"), ("da", "Oued Ed-Dahab Province"), ("de", "Oued ed Dahab"), ("el", "Ουέντ Εντ-Νταχάμπ"), ("en", "Oued Ed-Dahab"), ("es", "Provincia de Río de Oro-Dajla"), ("fi", "Oued Ed-Dahabn lääni"), ("fr", "province d’Oued Ed-Dahab"), ("gu", "ઓઉડ એડ-દાહાબ પ\u{acd}રા\u{a82}ત"), ("hi", "ओयड एड-दाहब प\u{94d}रा\u{902}त"), ("id", "Provinsi Oued Ed-Dahab"), ("it", "Provincia di Oued Ed-Dahab"), ("kn", "ಓಯಡ\u{ccd} ಎಡ\u{ccd}-ದಹಾಬ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우에드 에드 다합 주"), ("lt", "Ued Ed-Dahabas"), ("lv", "Oued Ed-Dahabas province"), ("mr", "ओएड एड-दाहाब प\u{94d}रा\u{902}त"), ("ms", "Oued Ed-Dahab Province"), ("nb", "Oued ed-Dahab provins"), ("nl", "Oued ed Dahab"), ("no", "Oued ed-Dahab provins"), ("pl", "Prowincja Oued Ed-Dahab"), ("pt", "Oued ed-Dahab provins"), ("ru", "Уэд Эд-Дахаб"), ("si", "ඖඑඩ\u{dca}-ඩහබ\u{dca} පළ\u{dcf}ත"), ("sv", "Oued ed-Dahab (provins)"), ("ta", "யத\u{bcd} எட\u{bcd} -டஹ\u{bbe}ப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}వ\u{c46}డ\u{c4d} ఎడ\u{c4d}-ద\u{c3e}హబ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกด เอด ดาอ\u{e31}บ"), ("tr", "Oued Ed-Dahab Province"), ("uk", "Провінція Уед-Ед-Дахаб"), ("ur", "ووید ید-داحاب صوبہ"), ("vi", "Oued Ed-Dahab Tỉnh"), ("zh", "黃金谷地省")]),
                        unofficial_name_list: ["Oued ed Dahab (EH)"].to_vec(),
                    }
                ),
                (
                    "OUJ",
                    Subdivision{
                        name: "OUJ",
                        country_alpha2: Alpha2::MA,
                        code: "OUJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.686667), longitude: Some(-1.911389), max_latitude: Some(34.7273116), min_latitude: Some(34.6418294), max_longitude: Some(-1.8517972), min_longitude: Some(-1.963159)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وجدة"), ("be", "Горад Уджда"), ("bg", "Уджда"), ("bn", "ওজদ\u{9be}"), ("ca", "Oujda"), ("ccp", "𑄃\u{1112f}𑄌\u{11134}𑄓-𑄃𑄚\u{11134}𑄉𑄖\u{11134}"), ("ceb", "Oujda (kapital sa rehiyon)"), ("cs", "Oujda"), ("cy", "Oujda"), ("da", "Oujda"), ("de", "Oujda"), ("el", "Ούζντα"), ("en", "Oujda-Angad"), ("es", "Uchda"), ("eu", "Ujda"), ("fa", "وجده"), ("fi", "Oujda"), ("fr", "Oujda"), ("gl", "Oujda"), ("gu", "ઔજદા"), ("he", "אוג׳דה"), ("hi", "उजडा"), ("hr", "Oujda"), ("hu", "Oujda"), ("id", "Oujda"), ("it", "Oujda"), ("ja", "ウジダ"), ("kk", "Уджда қаласы"), ("kn", "ಔಜದ"), ("ko", "우지다"), ("lt", "Udžda"), ("lv", "Udždza"), ("mk", "Уџда"), ("mr", "औजदा"), ("ms", "Oujda"), ("nb", "Oujda"), ("nl", "Oujda"), ("no", "Oujda"), ("pl", "Wadżda"), ("pt", "Oujda"), ("ro", "Oujda"), ("ru", "Уджда"), ("si", "ඖජ\u{dcf}"), ("sl", "Oujda"), ("sq", "Oujda"), ("sv", "Oujda"), ("ta", "அவுஜ\u{bcd}த\u{bbe}"), ("te", "ఊజ\u{c4d}డ\u{c3e}"), ("th", "อ\u{e38}จดา"), ("tr", "Ucda"), ("uk", "Уджда"), ("ur", "وجدہ"), ("vi", "Oujda"), ("zh", "乌季达")]),
                        unofficial_name_list: ["Oujda"].to_vec(),
                    }
                ),
                (
                    "OUZ",
                    Subdivision{
                        name: "OUZ",
                        country_alpha2: Alpha2::MA,
                        code: "OUZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.791702), longitude: Some(-7.092619999999999), max_latitude: Some(35.9344), min_latitude: Some(27.6672693), max_longitude: Some(-0.9969759), min_longitude: Some(-13.3044001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Ouezzane")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RAB",
                    Subdivision{
                        name: "RAB",
                        country_alpha2: Alpha2::MA,
                        code: "RAB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rabat"), ("am", "ራባት"), ("ar", "الرباط"), ("az", "Rabat"), ("be", "Горад Рабат"), ("bg", "Рабат"), ("bn", "র\u{9be}ব\u{9be}ত"), ("bs", "Rabat"), ("ca", "Rabat"), ("ccp", "𑄢\u{11127}𑄝𑄖\u{11134}"), ("cs", "Rabat"), ("cy", "Rabat"), ("da", "Rabat"), ("de", "Rabat"), ("el", "Ραμπάτ"), ("en", "Rabat"), ("es", "Rabat"), ("et", "Rabat"), ("eu", "Rabat"), ("fa", "رباط"), ("fi", "Rabat"), ("fr", "Rabat"), ("ga", "Rabat"), ("gl", "Rabat"), ("gu", "રબાટ"), ("ha", "Rabat"), ("ha_NE", "Rabat"), ("he", "רבאט"), ("hi", "रबत"), ("hr", "Rabat"), ("hu", "Rabat"), ("hy", "Ռաբաթ"), ("id", "Rabat"), ("is", "Rabat"), ("it", "Rabat"), ("ja", "ラバト"), ("jv", "Rabat"), ("ka", "რაბატი"), ("kn", "ರಬಾಟ\u{ccd}"), ("ko", "라바트"), ("ky", "Рабат"), ("lt", "Rabatas"), ("lv", "Rabāta"), ("mk", "Рабат"), ("ml", "റ\u{d3e}ബത\u{d4d}ത\u{d4d}"), ("mn", "Рабат"), ("mr", "रबात"), ("ms", "Rabat"), ("nb", "Rabat"), ("ne", "रबाट"), ("nl", "Rabat"), ("no", "Rabat"), ("or", "ରବୋଟ"), ("pa", "ਰਬਾਤ"), ("pl", "Rabat"), ("ps", "ربات"), ("pt", "Rabat"), ("ro", "Rabat"), ("ru", "Рабат"), ("si", "රබ\u{dcf}ත\u{dca}"), ("sk", "Rabat"), ("sl", "Rabat"), ("so", "Rabat"), ("sq", "Rabati"), ("sr", "Рабат"), ("sr_Latn", "Rabat"), ("sv", "Rabat"), ("sw", "Rabat"), ("ta", "ரப\u{bbe}த\u{bcd}"), ("te", "ర\u{c3e}బట\u{c4d}"), ("th", "ราบ\u{e31}ต"), ("tk", "Rabat"), ("tr", "Rabat"), ("uk", "Рабат"), ("ur", "رباط"), ("uz", "Rabot"), ("vi", "Rabat"), ("yo", "Rabat"), ("yo_BJ", "Rabat"), ("yue", "剌八"), ("yue_Hans", "剌八"), ("zh", "拉巴特")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "REH",
                    Subdivision{
                        name: "REH",
                        country_alpha2: Alpha2::MA,
                        code: "REH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Rehamna")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SAF",
                    Subdivision{
                        name: "SAF",
                        country_alpha2: Alpha2::MA,
                        code: "SAF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.3008151), longitude: Some(-9.2272033), max_latitude: Some(32.3430063), min_latitude: Some(32.2431484), max_longitude: Some(-9.1936504), min_longitude: Some(-9.2796346)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم آسفي"), ("ca", "Província de Safi"), ("ccp", "𑄥𑄜\u{11128}"), ("ceb", "Safi (lalawigan)"), ("de", "Safi"), ("el", "επαρχία Σάφι"), ("en", "Safi"), ("es", "Provincia de Safí"), ("fr", "province de Safi"), ("it", "Provincia di Safi"), ("ja", "サフィ州"), ("nl", "Safi"), ("sv", "Safi (provins)"), ("zh", "薩非省")]),
                        unofficial_name_list: ["Safi"].to_vec(),
                    }
                ),
                (
                    "SAL",
                    Subdivision{
                        name: "SAL",
                        country_alpha2: Alpha2::MA,
                        code: "SAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0336969), longitude: Some(-6.770813800000001), max_latitude: Some(34.0964405), min_latitude: Some(33.9791132), max_longitude: Some(-6.709358600000001), min_longitude: Some(-6.834943)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سلا"), ("be", "Горад Сале"), ("bg", "Сале"), ("bn", "স\u{9be}লে"), ("ca", "Salé"), ("ccp", "𑄥𑄣𑄬"), ("ceb", "Sale"), ("cs", "Salé"), ("cy", "Salé"), ("da", "Salé"), ("de", "Salé"), ("el", "Σαλέ"), ("en", "Salé"), ("es", "Salé"), ("eu", "Sale"), ("fa", "سلا"), ("fi", "Salé"), ("fr", "Salé"), ("gu", "સ\u{ac7}લ\u{ac7}"), ("he", "סלא"), ("hi", "साल\u{947}"), ("hr", "Salé"), ("hu", "Szale"), ("id", "Salé"), ("it", "Salé"), ("ja", "サレ"), ("kk", "Сале"), ("kn", "ಸಲ\u{cc6}"), ("ko", "살레"), ("lt", "Salė"), ("lv", "Sale"), ("mk", "Сале"), ("mr", "स\u{947}ल"), ("ms", "Sale"), ("nb", "Sale"), ("nl", "Salé"), ("no", "Sale"), ("pl", "Sala"), ("pt", "Salé"), ("ro", "Salé"), ("ru", "Сале"), ("si", "ස\u{dcf}ලේ"), ("sr", "Сале"), ("sr_Latn", "Sale"), ("sv", "Salé"), ("sw", "Sale"), ("ta", "சேல\u{bcd}"), ("te", "స\u{c3e}ల\u{c47}"), ("th", "ซาเล\u{e47}ม"), ("tr", "Salé"), ("uk", "Сале"), ("ur", "سلا"), ("vi", "Salé"), ("zh", "塞拉")]),
                        unofficial_name_list: ["Salé"].to_vec(),
                    }
                ),
                (
                    "SEF",
                    Subdivision{
                        name: "SEF",
                        country_alpha2: Alpha2::MA,
                        code: "SEF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.8305244), longitude: Some(-4.835315400000001), max_latitude: Some(33.8492472), min_latitude: Some(33.8098898), max_longitude: Some(-4.802141199999999), min_longitude: Some(-4.854927099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم صفرو"), ("ca", "Província de Sufruy"), ("ccp", "𑄥𑄬𑄜\u{11133}𑄢\u{11127}\u{1112e}"), ("ceb", "Sefrou (lalawigan)"), ("de", "Sefrou"), ("en", "Sefrou"), ("es", "Provincia de Sefrú"), ("fr", "province de Séfrou"), ("it", "Provincia di Sefrou"), ("ja", "セフル州"), ("nl", "Sefrou"), ("sv", "Sefrou (provins)"), ("zh", "塞夫勞省")]),
                        unofficial_name_list: ["Sefrou"].to_vec(),
                    }
                ),
                (
                    "SET",
                    Subdivision{
                        name: "SET",
                        country_alpha2: Alpha2::MA,
                        code: "SET",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.0), longitude: Some(-7.6167), max_latitude: Some(33.0173469), min_latitude: Some(32.9683838), max_longitude: Some(-7.575590699999998), min_longitude: Some(-7.663727499999998)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سطات"), ("ca", "Província de Settat"), ("ccp", "𑄥𑄬𑄖\u{11134}𑄑𑄖\u{11134}"), ("ceb", "Settat Province"), ("de", "Settat"), ("en", "Settat"), ("es", "Provincia de Settat"), ("fr", "province de Settat"), ("it", "Provincia di Settat"), ("ja", "セタト州"), ("nl", "Settat"), ("sv", "Settat Province"), ("zh", "塞塔特省")]),
                        unofficial_name_list: ["Settat"].to_vec(),
                    }
                ),
                (
                    "SIB",
                    Subdivision{
                        name: "SIB",
                        country_alpha2: Alpha2::MA,
                        code: "SIB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Sidi Bennour")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SIF",
                    Subdivision{
                        name: "SIF",
                        country_alpha2: Alpha2::MA,
                        code: "SIF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.660768), longitude: Some(49.8393179), max_latitude: Some(26.6660987), min_latitude: Some(26.6544679), max_longitude: Some(49.8596116), min_longitude: Some(49.81823420000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Sidi Ifni")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SIK",
                    Subdivision{
                        name: "SIK",
                        country_alpha2: Alpha2::MA,
                        code: "SIK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.216667), longitude: Some(-5.7), max_latitude: Some(34.2702685), min_latitude: Some(34.2092469), max_longitude: Some(-5.6877422), min_longitude: Some(-5.7565785)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سيدي قاسم"), ("ca", "Província de Sidi Kacem"), ("ccp", "𑄥\u{11128}𑄓\u{11128} 𑄇𑄥𑄬𑄟\u{11134}"), ("ceb", "Sidi-Kacem (lalawigan)"), ("de", "Sidi Kacem"), ("en", "Sidi Kacem"), ("es", "Provincia de Sidi Kacem"), ("fa", "استان سیدی قاسم"), ("fr", "province de Sidi Kacem"), ("it", "Provincia di Sidi Kacem"), ("ja", "シディ・カセム州"), ("nl", "Sidi Kacem"), ("sv", "Sidi-Kacem (provins i Marocko)"), ("ur", "سیدی قاسم صوبہ"), ("zh", "西迪卡塞姆省")]),
                        unofficial_name_list: ["Sidi Kacem"].to_vec(),
                    }
                ),
                (
                    "SIL",
                    Subdivision{
                        name: "SIL",
                        country_alpha2: Alpha2::MA,
                        code: "SIL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.147821), longitude: Some(-8.5584885), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Sidi Slimane")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SKH",
                    Subdivision{
                        name: "SKH",
                        country_alpha2: Alpha2::MA,
                        code: "SKH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.9278354), longitude: Some(-6.9051819), max_latitude: Some(33.9586252), min_latitude: Some(33.8619702), max_longitude: Some(-6.877097999999999), min_longitude: Some(-7.0053398)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تمارة"), ("bn", "তেম\u{9be}র\u{9be}"), ("ca", "Témara"), ("ccp", "𑄈\u{11128}𑄢𑄖\u{11134}-𑄑𑄬𑄟𑄢"), ("ceb", "Temara (kapital sa munisipyo)"), ("cs", "Temara"), ("da", "Témara"), ("de", "Témara"), ("el", "Τεμάρα"), ("en", "Skhirat-Témara"), ("es", "Temara"), ("fa", "تماره"), ("fi", "Temara"), ("fr", "Témara"), ("gu", "ત\u{ac7}મારા"), ("hi", "त\u{948}मारा"), ("id", "Temara"), ("it", "Temara"), ("ja", "テマラ"), ("kn", "ತ\u{cc6}ಮಾರಾ"), ("ko", "테마라"), ("lt", "Temara"), ("lv", "Temara"), ("mr", "त\u{947}मारा"), ("ms", "Temara"), ("nb", "Témara"), ("nl", "Témara"), ("no", "Témara"), ("pl", "Temara"), ("pt", "Temara"), ("ru", "Темара"), ("si", "ටෙම\u{dcf}ර\u{dcf}"), ("sv", "Témara"), ("ta", "டெம\u{bbe}ர"), ("te", "త\u{c47}మ\u{c3e}ర"), ("th", "เตมารา"), ("tr", "Temara"), ("uk", "Темара"), ("ur", "تمارہ"), ("vi", "Temara"), ("zh", "特马拉")]),
                        unofficial_name_list: ["Skhirate-Témara"].to_vec(),
                    }
                ),
                (
                    "SYB",
                    Subdivision{
                        name: "SYB",
                        country_alpha2: Alpha2::MA,
                        code: "SYB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.6084373), longitude: Some(-7.965306699999999), max_latitude: Some(31.6256135), min_latitude: Some(31.5528848), max_longitude: Some(-7.920498799999999), min_longitude: Some(-7.978788499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Marrakesj³"), ("ar", "عمالة سيدي يوسف بن علي"), ("be", "Маракеш³"), ("bg", "Маракеш³"), ("bn", "ম\u{9be}র\u{9be}ক\u{9cd}কেশ³"), ("ca", "Marràqueix³"), ("ccp", "𑄥\u{11128}𑄓\u{11128} 𑄃\u{11128}𑄅\u{1112a}𑄥𑄬𑄛\u{11134} 𑄝𑄬𑄚\u{11134} 𑄃𑄣\u{11128}"), ("cs", "Marrákeš³"), ("da", "Marrakech³"), ("de", "Marrakesch³"), ("el", "Μαρακές³"), ("en", "Sidi Youssef Ben Ali"), ("es", "Marrakech³"), ("et", "Marrakech³"), ("eu", "Marrakex³"), ("fa", "مراکش³"), ("fi", "Marrakech³"), ("fr", "Marrakech³"), ("gl", "Marrakech³"), ("gu", "મારાક\u{ac7}શ³"), ("he", "מרקש³"), ("hi", "मराक\u{947}श³"), ("hr", "Marrakech³"), ("hu", "Marrákes³"), ("hy", "Մառակեշ³"), ("id", "Marrakesh³"), ("is", "Marrakess³"), ("it", "Marrakech³"), ("ja", "マラケシュ³"), ("ka", "მარაკეში³"), ("kn", "ಮಾರಕೇಶ\u{ccd}³"), ("ko", "마라케시³"), ("lt", "Marakešas³"), ("lv", "Marrākeša³"), ("ml", "മര\u{d3e}ക\u{d4d}കേഷ\u{d4d}³"), ("mr", "माराक\u{947}श³"), ("ms", "Marrakesh³"), ("nb", "Marrakech³"), ("ne", "मर\u{94d}राक\u{947}शको मदिना³"), ("nl", "Marrakesh³"), ("no", "Marrakech³"), ("pl", "Marrakesz³"), ("pt", "Marraquexe³"), ("ro", "Marrakech³"), ("ru", "Марракеш³"), ("si", "මරකෙෂ\u{dca}³"), ("sk", "Murrákuš³"), ("sl", "Marakeš, Maroko³"), ("sr", "Маракеш³"), ("sr_Latn", "Marakeš³"), ("sv", "Marrakech³"), ("ta", "மரர\u{bbe}கேஷ\u{bcd}³"), ("te", "మర\u{c3e}క\u{c47}శ\u{c4d}³"), ("th", "มาร\u{e4c}ราค\u{e34}ช³"), ("tr", "Marakeş³"), ("uk", "Марракеш³"), ("ur", "مراکش (شہر)³"), ("vi", "Marrakech³"), ("zh", "马拉喀什³")]),
                        unofficial_name_list: ["Sidi Youssef Ben Ali"].to_vec(),
                    }
                ),
                (
                    "TAF",
                    Subdivision{
                        name: "TAF",
                        country_alpha2: Alpha2::MA,
                        code: "TAF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.6360026), longitude: Some(-8.057429899999999), max_latitude: Some(31.72042849999999), min_latitude: Some(31.5600339), max_longitude: Some(-8.0045699), min_longitude: Some(-8.1314279)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Tarfaya (EH-partial)")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TAI",
                    Subdivision{
                        name: "TAI",
                        country_alpha2: Alpha2::MA,
                        code: "TAI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3983716), longitude: Some(-2.8935028), max_latitude: Some(34.4237263), min_latitude: Some(34.3875042), max_longitude: Some(-2.8746414), min_longitude: Some(-2.9234147)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تاوريرت"), ("ca", "Província de Taourirt"), ("ccp", "𑄑\u{1112f}𑄢\u{11128}𑄢\u{11134}𑄑\u{11134}"), ("de", "Taourirt"), ("en", "Taourirt"), ("es", "Provincia de Taurirt"), ("fa", "استان تاوریرت"), ("fr", "province de Taourirt"), ("it", "Provincia di Taourirt"), ("ja", "タウリル州"), ("nl", "Taourirt"), ("pl", "Taurirt"), ("ur", "تاوریرت صوبہ"), ("zh", "陶里爾特省")]),
                        unofficial_name_list: ["Taourirt"].to_vec(),
                    }
                ),
                (
                    "TAO",
                    Subdivision{
                        name: "TAO",
                        country_alpha2: Alpha2::MA,
                        code: "TAO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.535833), longitude: Some(-4.64), max_latitude: Some(34.555448), min_latitude: Some(34.5246897), max_longitude: Some(-4.6242773), min_longitude: Some(-4.6587909)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تاونات"), ("ca", "Província de Taounate"), ("ccp", "𑄑\u{1112f}𑄚𑄖\u{11134}"), ("ceb", "Taounate"), ("de", "Taounate"), ("en", "Taounate"), ("es", "Provincia de Taunat"), ("fr", "province de Taounate"), ("it", "Provincia di Taounate"), ("ja", "タウナト州"), ("nl", "Taounate"), ("sv", "Taounate (provins)"), ("zh", "陶納特省")]),
                        unofficial_name_list: ["Taounate"].to_vec(),
                    }
                ),
                (
                    "TAR",
                    Subdivision{
                        name: "TAR",
                        country_alpha2: Alpha2::MA,
                        code: "TAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.466944), longitude: Some(-8.879999999999999), max_latitude: Some(30.497608), min_latitude: Some(30.459809), max_longitude: Some(-8.8509464), min_longitude: Some(-8.9028739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تارودانت"), ("bg", "Тароудант"), ("ca", "Província de Taroudant"), ("ccp", "𑄑𑄢\u{1112f}𑄓𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Taroudannt"), ("de", "Taroudant"), ("en", "Taroudant"), ("es", "Provincia de Tarudant"), ("fr", "province de Taroudannt"), ("it", "Provincia di Taroudant"), ("ja", "タルーダント州"), ("nl", "Taroudant"), ("pt", "Tarudante"), ("sv", "Taroudannt (provins)"), ("zh", "塔魯丹特省")]),
                        unofficial_name_list: ["Taroudannt"].to_vec(),
                    }
                ),
                (
                    "TAT",
                    Subdivision{
                        name: "TAT",
                        country_alpha2: Alpha2::MA,
                        code: "TAT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.742778), longitude: Some(-7.972499999999998), max_latitude: Some(29.7831515), min_latitude: Some(29.7337501), max_longitude: Some(-7.9531574), min_longitude: Some(-7.996587799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم طاطا"), ("ca", "Província de Tata"), ("ccp", "𑄑𑄑"), ("ceb", "Tata"), ("de", "Tata"), ("en", "Tata"), ("es", "Provincia de Tata"), ("fr", "province de Tata"), ("it", "Provincia di Tata"), ("ja", "タタ州"), ("nl", "Tata"), ("sv", "Tata (provins)"), ("zh", "塔塔省")]),
                        unofficial_name_list: ["Tata"].to_vec(),
                    }
                ),
                (
                    "TAZ",
                    Subdivision{
                        name: "TAZ",
                        country_alpha2: Alpha2::MA,
                        code: "TAZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.216667), longitude: Some(-4.016667), max_latitude: Some(34.2452976), min_latitude: Some(34.1945525), max_longitude: Some(-3.9336206), min_longitude: Some(-4.0451144)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تازة"), ("ca", "Província de Taza"), ("ccp", "𑄑𑄎"), ("ceb", "Taza (lalawigan)"), ("de", "Taza"), ("en", "Taza"), ("es", "Provincia de Taza"), ("fr", "province de Taza"), ("it", "Provincia di Taza"), ("ja", "ターザ州"), ("nl", "Taza"), ("sv", "Taza (provins)"), ("zh", "塔扎省")]),
                        unofficial_name_list: ["Taza"].to_vec(),
                    }
                ),
                (
                    "TET",
                    Subdivision{
                        name: "TET",
                        country_alpha2: Alpha2::MA,
                        code: "TET",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.566667), longitude: Some(-5.366667), max_latitude: Some(35.6139067), min_latitude: Some(35.5562292), max_longitude: Some(-5.2952385), min_longitude: Some(-5.4377174)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تطوان"), ("ca", "Província de Tetuan"), ("ccp", "𑄑𑄬𑄑\u{1112f}𑄠𑄚\u{11134}"), ("ceb", "Tetouan"), ("de", "Tétouan"), ("en", "Tétouan"), ("es", "Prefectura de Tetuán"), ("fr", "province de Tétouan"), ("it", "Prefettura di Tétouan"), ("ja", "テトゥアン州"), ("nl", "Tétouan"), ("sv", "Tetouan (provins i Marocko)"), ("ur", "تطوان صوبہ"), ("zh", "得土安省")]),
                        unofficial_name_list: ["Tétouan*"].to_vec(),
                    }
                ),
                (
                    "TIN",
                    Subdivision{
                        name: "TIN",
                        country_alpha2: Alpha2::MA,
                        code: "TIN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.9263125), longitude: Some(-6.9072676), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Tinghir")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TIZ",
                    Subdivision{
                        name: "TIZ",
                        country_alpha2: Alpha2::MA,
                        code: "TIZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.693392), longitude: Some(-9.732156999999999), max_latitude: Some(29.7290494), min_latitude: Some(29.67264399999999), max_longitude: Some(-9.702920899999999), min_longitude: Some(-9.7581326)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تيزنيت"), ("ca", "Província de Tiznit"), ("ccp", "𑄑\u{11128}𑄌\u{11134}𑄚\u{11128}𑄖\u{11134}"), ("ceb", "Tiznit (lalawigan)"), ("de", "Tiznit"), ("en", "Tiznit"), ("es", "Provincia de Tiznit"), ("fr", "province de Tiznit"), ("it", "Provincia di Tiznit"), ("ja", "ティーズニート州"), ("nl", "Tiznit"), ("pt", "Tiznit"), ("ru", "Тизнит"), ("sv", "Tiznit (provins)"), ("zh", "提茲尼特省")]),
                        unofficial_name_list: ["Tiznit"].to_vec(),
                    }
                ),
                (
                    "TNG",
                    Subdivision{
                        name: "TNG",
                        country_alpha2: Alpha2::MA,
                        code: "TNG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7594651), longitude: Some(-5.833954299999999), max_latitude: Some(35.8257368), min_latitude: Some(35.7008015), max_longitude: Some(-5.718555500000001), min_longitude: Some(-5.9487488)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tangier"), ("ar", "طنجة"), ("az", "Tanjer"), ("be", "Танжэр"), ("bg", "Танжер"), ("bn", "ত\u{9be}নজ\u{9be}হ"), ("bs", "Tanger"), ("ca", "Tànger"), ("ccp", "𑄑𑄚\u{11134}𑄉\u{11128}𑄠𑄢\u{11134}-𑄃𑄥\u{11128}𑄣𑄦\u{11134}"), ("ceb", "Tanger-Assilah"), ("cs", "Tanger"), ("cy", "Tanger"), ("da", "Tanger"), ("de", "Tanger"), ("el", "Ταγγέρη"), ("en", "Tangier-Assilah"), ("es", "Tánger"), ("et", "Tanger"), ("eu", "Tanger"), ("fa", "تانگیر"), ("fi", "Tanger"), ("fr", "Tanger"), ("gl", "Tánxer"), ("gu", "ટ\u{ac7}\u{a82}જિયર"), ("he", "טנג׳יר"), ("hi", "ट\u{902}ग\u{947}र"), ("hr", "Tanger"), ("hu", "Tanger"), ("hy", "Տանժեր"), ("id", "Tangier"), ("it", "Tangeri"), ("ja", "タンジェ"), ("ka", "ტანჟერი"), ("kk", "Танжер қаласы"), ("kn", "ಟ\u{ccd}ಯಾಂಜ\u{cbf}ಯರ\u{ccd}"), ("ko", "탕헤르"), ("lt", "Tanžeras"), ("lv", "Tanžera"), ("mk", "Тангер"), ("ml", "ട\u{d3e}ൻജീർ"), ("mn", "Танжер"), ("mr", "ट\u{901}जियर"), ("ms", "Tangier"), ("my", "တန\u{103a}ဂျ\u{102e}းယားမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Tanger"), ("nl", "Tanger"), ("no", "Tanger"), ("pl", "Tanger"), ("pt", "Tânger"), ("ro", "Tanger"), ("ru", "Танжер"), ("si", "ටැන\u{dca}ජයර\u{dca}"), ("sl", "Tanger"), ("sr", "Тангер"), ("sr_Latn", "Tanger"), ("sv", "Tanger"), ("ta", "டன\u{bcd}கிஏர\u{bcd}"), ("te", "ట\u{c4d}య\u{c3e}ంజ\u{c3f}యర\u{c4d}"), ("th", "แทนเก\u{e35}ยร\u{e4c}"), ("tr", "Tanca"), ("uk", "Танжер"), ("ur", "طنجہ"), ("vi", "Tangier"), ("yue", "丹吉爾"), ("yue_Hans", "丹吉尔"), ("zh", "丹吉尔")]),
                        unofficial_name_list: ["Tanger"].to_vec(),
                    }
                ),
                (
                    "TNT",
                    Subdivision{
                        name: "TNT",
                        country_alpha2: Alpha2::MA,
                        code: "TNT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.4380408), longitude: Some(-11.0987374), max_latitude: Some(28.4523924), min_latitude: Some(28.4089164), max_longitude: Some(-11.0713005), min_longitude: Some(-11.125803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم طانطان"), ("ca", "Província de Tan-Tan"), ("ccp", "𑄑𑄚\u{11134}-𑄑𑄚\u{11134}"), ("ceb", "Tan-Tan (lalawigan)"), ("de", "Tan-Tan"), ("en", "Tan-Tan"), ("es", "Provincia de Tan-Tan"), ("fr", "province de Tan-Tan"), ("it", "Provincia di Tan-Tan"), ("ja", "タンタン州"), ("nl", "Tan-Tan"), ("ru", "Тан-Тан"), ("sv", "Tan-Tan (provins)"), ("zh", "坦坦省")]),
                        unofficial_name_list: ["Tan-Tan"].to_vec(),
                    }
                ),
                (
                    "X1~",
                    Subdivision{
                        name: "X1~",
                        country_alpha2: Alpha2::MA,
                        code: "X1~",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.8521664), longitude: Some(-12.1632718), max_latitude: Some(28.2162268), min_latitude: Some(27.6665817), max_longitude: Some(-11.50715), min_longitude: Some(-13.1729662)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Laayoune-Boujdour-Sakia El Hamra")]),
                        unofficial_name_list: ["Laayoune-Boujdour-Sakia El Hamra"].to_vec(),
                    }
                ),
                (
                    "YUS",
                    Subdivision{
                        name: "YUS",
                        country_alpha2: Alpha2::MA,
                        code: "YUS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Youssoufia")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ZAG",
                    Subdivision{
                        name: "ZAG",
                        country_alpha2: Alpha2::MA,
                        code: "ZAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.330556), longitude: Some(-5.838056), max_latitude: Some(30.3685246), min_latitude: Some(30.3173955), max_longitude: Some(-5.8262927), min_longitude: Some(-5.8603477)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم زاكورة"), ("ca", "Província de Zagora"), ("ccp", "𑄎𑄉\u{1112e}𑄢"), ("ceb", "Zagora (lalawigan)"), ("de", "Zagora"), ("en", "Zagora"), ("es", "Provincia de Zagora"), ("fr", "province de Zagora"), ("it", "Provincia di Zagora"), ("ja", "ザゴラ州"), ("nl", "Zagora"), ("pt", "Zagora"), ("zh", "扎古拉省")]),
                        unofficial_name_list: ["Zagora"].to_vec(),
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
#[cfg(feature = "ma")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MA,
        alpha3: Alpha3::MAR,
        address_format: None,
        continent: Continent::Africa,
        country_code: 212,
        currency_code: CurrencyCode::MAD,
        gec: Some(GEC::MO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::MAR),
        iso_long_name: "The Kingdom of Morocco",
        iso_short_name: "Morocco",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Moroccan"),
        number: "504",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernAfrica),
        un_locode: "MA",
        unofficial_name_list: [
            "Morocco",
            "المغرب",
            "Marokko",
            "Maroc",
            "Marruecos",
            "モロッコ",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Morocco"),
            ("af", "Marokko"),
            ("ak", "Morocco"),
            ("am", "ሥስጥ"),
            ("an", "Morocco"),
            ("ar", "المغرب"),
            ("as", "মৰোক\u{9cd}কো"),
            ("ay", "Morocco"),
            ("az", "Mərakeş"),
            ("ba", "Morocco"),
            ("be", "Марока"),
            ("bg", "Мароко"),
            ("bi", "Morocco"),
            ("bn", "মরোক\u{9cd}কো"),
            ("bn_IN", "মরোক\u{9cd}কো"),
            ("br", "Maroko"),
            ("bs", "Maroko"),
            ("ca", "Marroc"),
            ("ce", "Марокко"),
            ("ch", "Morocco"),
            ("cs", "Maroko"),
            ("cv", "Марокко"),
            ("cy", "Moroco"),
            ("da", "Marokko"),
            ("de", "Marokko"),
            ("dv", "މ\u{7a6}އ\u{7aa}ރ\u{7a8}ބ\u{7aa}"),
            ("dz", "མ\u{f7c}་ར\u{f7c}ཀ་ཀ\u{f7c}།"),
            ("ee", "Morocco"),
            ("el", "Μαρόκο"),
            ("en", "Morocco"),
            ("eo", "Maroko"),
            ("es", "Marruecos"),
            ("et", "Maroko"),
            ("eu", "Maroko"),
            ("fa", "مراکش"),
            ("ff", "Morocco"),
            ("fi", "Marokko"),
            ("fo", "Marokko"),
            ("fr", "Maroc"),
            ("fy", "Marokko"),
            ("ga", "Maracó"),
            ("gl", "Marrocos"),
            ("gn", "Morocco"),
            ("gu", "મોરક\u{acd}કો"),
            ("gv", "Yn Varoc"),
            ("ha", "Morocco"),
            ("he", "מרוקו"),
            ("hi", "मोरक\u{94d}को"),
            ("hr", "Maroko"),
            ("ht", "Mawòk"),
            ("hu", "Marokkó"),
            ("hy", "Մարոկո"),
            ("ia", "Marocco"),
            ("id", "Maroko"),
            ("io", "Maroko"),
            ("is", "Marokkó"),
            ("it", "Marocco"),
            ("iu", "Morocco"),
            ("ja", "モロッコ"),
            ("ka", "მაროკო"),
            ("ki", "Morocco"),
            ("kk", "Марокко"),
            ("kl", "Morocco"),
            ("km", "ម\u{17c9}ារ\u{17c9}\u{17bb}ក"),
            ("kn", "ಮೊರೊಕ\u{ccd}ಕೊ"),
            ("ko", "모로코"),
            ("ku", "Fas"),
            ("kv", "Morocco"),
            ("kw", "Marokk"),
            ("ky", "Марокко"),
            ("lo", "Morocco"),
            ("lt", "Marokas"),
            ("lv", "Maroka"),
            ("mi", "Moroko"),
            ("mk", "Мароко"),
            ("ml", "മൊറോക\u{d4d}കോ"),
            ("mn", "Морокко"),
            ("mr", "मोरोक\u{94d}को"),
            ("ms", "Maghribi"),
            ("mt", "Marokk"),
            (
                "my",
                "မော\u{103a}ရ\u{102d}\u{102f}က\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Morocco"),
            ("nb", "Marokko"),
            ("ne", "मोरक\u{94d}को"),
            ("nl", "Marokko"),
            ("nn", "Marokko"),
            (
                "nv",
                "Eʼeʼaahjí Ghą\u{301}ą\u{301}ʼaskʼidii Biłikahii Bikéyah",
            ),
            ("oc", "Marròc"),
            ("or", "ମୋରକ\u{b4d}କୋ"),
            ("pa", "ਮ\u{a4b}ਰ\u{a4b}ਸ\u{a3c}ਸ\u{a4b}"),
            ("pi", "मोराको"),
            ("pl", "Maroko"),
            ("ps", "مراکش"),
            ("pt", "Marrocos"),
            ("pt_BR", "Marrocos"),
            ("ro", "Maroc"),
            ("ru", "Марокко"),
            ("rw", "Maroke"),
            ("sc", "Marocu"),
            ("sd", "Morocco"),
            ("si", "මොරොක\u{dca}කෝව"),
            ("sk", "Maroko"),
            ("sl", "Maroko"),
            ("so", "Marooko"),
            ("sq", "Marok"),
            ("sr", "Мароко"),
            ("sv", "Marocko"),
            ("sw", "Morocco"),
            ("ta", "மொரோக\u{bcd}கோ"),
            ("te", "మ\u{c4b}ర\u{c4b}క\u{c4d}క\u{c4b}"),
            ("tg", "Марокаш"),
            ("th", "โมร\u{e47}อกโก"),
            ("ti", "ሞሮኮ"),
            ("tk", "Morokko"),
            ("tl", "Morocco"),
            ("tr", "Fas"),
            ("tt", "Морокко"),
            ("ug", "ماراكەش"),
            ("uk", "Марокко"),
            ("ur", "مراکش"),
            ("uz", "Marokash"),
            ("ve", "Morocco"),
            ("vi", "Mo-ro-cô"),
            ("wa", "Marok"),
            ("wo", "Marook"),
            ("xh", "Morocco"),
            ("yo", "Mòrókò"),
            ("zh_CN", "摩洛哥"),
            ("zh_HK", "摩洛哥"),
            ("zh_TW", "摩洛哥"),
            ("zu", "IMorokho"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
