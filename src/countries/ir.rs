// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Islamic Republic of Iran

#[cfg(all(feature = "ir", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::IR;
    pub const ALPHA3: Alpha3 = Alpha3::IRN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 98;
    pub const CURRENCY_CODE: &str = "IRR";
    pub const GEC: Option<GEC> = Some(GEC::IR);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("IRI");
    pub const ISO_SHORT_NAME: &str = "Iran (Islamic Republic of)";
    pub const ISO_LONG_NAME: &str = "The Islamic Republic of Iran";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fa"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fa"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Iranian");
    pub const NUMBER: &str = "364";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}-?\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Saturday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "IR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Iran",
        "Irán",
        "Iran (Islamic Republic Of)",
        "イラン・イスラム共和国",
        "Islamic Republic of Iran",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Iran, Islamic Republic of"),
        ("af", "Iran, Islamitiese Republiek van"),
        ("ak", "Iran, Islamic Republic of"),
        ("am", "Iran, Islamic Republic of"),
        ("an", "Iran, Islamic Republic of"),
        ("ar", "إيران، الجمهوري\u{651}ة الإسلامي\u{651}ة الإيراني\u{651}ة"),
        ("as", "ইৰ\u{9be}ন, ইচ\u{9cd}\u{200c}ল\u{9be}মিক প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"),
        ("ay", "Iran, Islamic Republic of"),
        ("az", "Iran, Islamic Republic of"),
        ("ba", "Iran, Islamic Republic of"),
        ("be", "Іран, Ісламская Рэспубліка"),
        ("bg", "Иран, Ислямска република"),
        ("bi", "Iran, Islamic Republic of"),
        ("bn", "ইর\u{9be}ন, ইসল\u{9be}মিক প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("bn_IN", "ইর\u{9be}ন, ইসল\u{9be}মিক প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("br", "Iran, Islamic Republic of"),
        ("bs", "Islamska Republika Iran"),
        ("ca", "Iran"),
        ("ce", "Iran, Islamic Republic of"),
        ("ch", "Iran, Islamic Republic of"),
        ("cs", "Írán, islámská republika"),
        ("cv", "Iran, Islamic Republic of"),
        ("cy", "Iran, Gweriniaeth Islamaidd"),
        ("da", "Iran, Den Islamiske Republik"),
        ("de", "Iran, Islamische Republik"),
        ("dv", "Iran, Islamic Republic of"),
        ("dz", "ཨ\u{f72}་ར\u{f71}ན་ ཨ\u{f72}ས་ལ་མ\u{f72}ཀ་ མ\u{f72}་ས\u{f7a}ར་ར\u{f92}\u{fb1}ལ་ཁབ།"),
        ("ee", "Iran, Islamic Republic of"),
        ("el", "Ιράν, Ισλαμική Δημοκρατία του"),
        ("en", "Iran"),
        ("eo", "Irano, Islama Respubliko"),
        ("es", "Irán, República islámica de"),
        ("et", "Iraan"),
        ("eu", "Iran, Islamiar Errepublika"),
        ("fa", "ایران"),
        ("ff", "Iran, Islamic Republic of"),
        ("fi", "Iranin islamilainen tasavalta"),
        ("fo", "Iran, Islamic Republic of"),
        ("fr", "Iran, République islamique d'"),
        ("fy", "Iran, Islamic Republic of"),
        ("ga", "An Iaráin"),
        ("gl", "Irán, República Islámica de"),
        ("gn", "Iran, Islamic Republic of"),
        ("gu", "ઇરાન, ઇસ\u{acd}લામિક રીપબ\u{acd}લિક ઓફ"),
        ("gv", "Iran, Islamic Republic of"),
        ("ha", "Iran, Islamic Republic of"),
        ("he", "אירן, הרפובליקה האיסלמית של"),
        ("hi", "ईरान, इस\u{94d}लामिक रिपब\u{94d}लिक ऑफ"),
        ("hr", "Iran, Islamska Republika"),
        ("ht", "Iran, Islamic Republic of"),
        ("hu", "Irán, Iszlám Köztársaság"),
        ("hy", "Իրան, -ի Իսլամական Հանրապետություն"),
        ("ia", "Iran"),
        ("id", "Iran, Republik Islam"),
        ("io", "Iran, Islamic Republic of"),
        ("is", "Íran, íslamska lýðveldið"),
        ("it", "Iran"),
        ("iu", "Iran, Islamic Republic of"),
        ("ja", "イラン・イスラム共和国"),
        ("ka", "ირანის ისლამური რესპუბლიკა"),
        ("ki", "Iran, Islamic Republic of"),
        ("kk", "Иран"),
        ("kl", "Iran, Islamic Republic of"),
        ("km", "\u{200b}អ\u{17ca}\u{17b8}រ\u{17c9}ង\u{17cb} សាធារណ\u{200b}រដ\u{17d2}ឋ\u{200b}អ\u{17ca}\u{17b8}ស\u{17d2}លាម\u{200b}នៃ"),
        ("kn", "ಇರಾನ\u{ccd} ಇಸ\u{ccd}ಲಾಮ\u{cbf}ಕ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"),
        ("ko", "이란 이슬람 공화국"),
        ("ku", "Îran, Komara Îslamî ya"),
        ("kv", "Iran, Islamic Republic of"),
        ("kw", "Iran, Islamic Republic of"),
        ("ky", "Иран"),
        ("lo", "Iran, Islamic Republic of"),
        ("lt", "Irano Islamo Respublika"),
        ("lv", "Irāna"),
        ("mi", "Iran, Islamic Republic of"),
        ("mk", "Иран"),
        ("ml", "ഇറ\u{d3e}ന\u{d4d}\u{200d}, ഇസ\u{d4d}ല\u{d3e}മിക\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"),
        ("mn", "Iran, Islamic Republic of"),
        ("mr", "इराण, इस\u{94d}लामिक रिपब\u{94d}लिक ऑफ"),
        ("ms", "Iran, Islamic Republic of"),
        ("mt", "Iran, Islamic Republic of"),
        ("my", "Iran, Islamic Republic of"),
        ("na", "Iran, Islamic Republic of"),
        ("nb", "Iran, Den islamske republikk"),
        ("ne", "इरान, इस\u{94d}लामिक गणराज\u{94d}य"),
        ("nl", "Iran"),
        ("nn", "Iran"),
        ("nv", "Iran, Islamic Republic of"),
        ("oc", "Republica Dominicana"),
        ("or", "ଇର\u{b3e}ନ, ଇସଲ\u{b3e}ମ\u{b3f}କ ଗଣତନ\u{b4d}ତ\u{b4d}ର"),
        ("pa", "ਈਰਾਨ, ਮ\u{a41}ਸਲਮਾਨ ਗਣਰਾਜ"),
        ("pi", "Iran, Islamic Republic of"),
        ("pl", "Iran"),
        ("ps", "Iran, Islamic Republic of"),
        ("pt", "Irão"),
        ("pt_BR", "Irã"),
        ("ro", "Iran, Republica islamică"),
        ("ru", "Иран"),
        ("rw", "Iran, Repubulika ya Kisilimu"),
        ("sc", "Iran, Repùblica Islàmica de"),
        ("sd", "Iran, Islamic Republic of"),
        ("si", "ඉර\u{dcf}න ඉස\u{dca}ල\u{dcf}ම\u{dd3}ය ජනරජය"),
        ("sk", "Iránska islamská republika"),
        ("sl", "Iran"),
        ("so", "Iiraan"),
        ("sq", "Iran, Republika Islamike"),
        ("sr", "Иран, Исламска Република"),
        ("sv", "Iran, islamiska republiken"),
        ("sw", "Iran, Islamic Republic of"),
        ("ta", "ஈர\u{bbe}ன\u{bcd}, இஸ\u{bcd}ல\u{bbe}மிக\u{bcd} குடியரசு"),
        ("te", "ఇర\u{c3e}న\u{c4d}, ఇస\u{c4d}ల\u{c3e}మ\u{c3f}క\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}"),
        ("tg", "Ҷумҳурии Исломии Эрон"),
        ("th", "อ\u{e34}หร\u{e48}าน, สาธารณร\u{e31}ฐอ\u{e34}สลาม"),
        ("ti", "Iran, Islamic Republic of"),
        ("tk", "Eýran, Yslam Respublikasy"),
        ("tl", "Iran, Republikang Islam ng"),
        ("tr", "İran İslâm Cumhuriyeti"),
        ("tt", "İран, İслам Җөмһүриәте"),
        ("ug", "ئىران ئىسلام جۇمھۇرىيىتى"),
        ("uk", "Іран"),
        ("ur", "Iran, Islamic Republic of"),
        ("uz", "Iran, Islamic Republic of"),
        ("ve", "Iran, Islamic Republic of"),
        ("vi", "Ba Tư, Cộng hoà Hồi giáo"),
        ("wa", "Iran"),
        ("wo", "Iraan, Republik Islaamik bu"),
        ("xh", "Iran, Islamic Republic of"),
        ("yo", "Iran, Islamic Republic of"),
        ("zh_CN", "伊朗"),
        ("zh_HK", "伊朗"),
        ("zh_TW", "伊朗伊斯蘭共和國"),
        ("zu", "Iran, Islamic Republic of"),
];
    #[cfg(all(feature = "ir", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 32.427908;
        pub const LONGITUDE: f64 = 53.688046;
        pub const MAX_LATITUDE: f64 = 39.782056;
        pub const MAX_LONGITUDE: f64 = 63.3333366;
        pub const MIN_LATITUDE: f64 = 24.8066999;
        pub const MIN_LONGITUDE: f64 = 44.0326949;
        pub const NORTHEAST_LATITUDE: f64 = 39.782056;
        pub const NORTHEAST_LONGITUDE: f64 = 63.3333366;
        pub const SOUTHWEST_LATITUDE: f64 = 24.8066999;
        pub const SOUTHWEST_LONGITUDE: f64 = 44.0326949;
    }
}
#[cfg(all(feature = "ir", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 32.427908,
            longitude: 53.688046,
            max_latitude: 39.782056,
            max_longitude: 63.3333366,
            min_latitude: 24.8066999,
            min_longitude: 44.0326949,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 39.782056,
                    longitude: 63.3333366,
                },
                southwest: CountryGeoBound {
                    latitude: 24.8066999,
                    longitude: 44.0326949,
                },
            },
        }
    }
}

#[cfg(all(feature = "ir", feature = "subdivisions"))]
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
                    "00",
                    Subdivision{
                        name: "00",
                        country_alpha2: Alpha2::IR,
                        code: "00",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.612305), longitude: Some(49.8547266), max_latitude: Some(35.710403), min_latitude: Some(33.5383001), max_longitude: Some(51.0862229), min_longitude: Some(48.889122)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مركزي"), ("az", "Mərkəzi ostanı"), ("bg", "Маркажи"), ("bn", "ম\u{9be}র\u{9cd}ক\u{9be}জি প\u{9cd}রদেশ"), ("ca", "Markazi"), ("ccp", "𑄟𑄢\u{11134}𑄇𑄎\u{11128}"), ("ceb", "Markazi"), ("cs", "Markazí"), ("cy", "Markazi"), ("da", "Markazi"), ("de", "Markazi"), ("el", "Μαρκαζί"), ("en", "Markazi"), ("es", "Markazi"), ("eu", "Markazi"), ("fa", "استان مرکزی"), ("fi", "Markazī"), ("fr", "Markazi"), ("gl", "Markazi"), ("gu", "માકાઝી પ\u{acd}રા\u{a82}ત"), ("he", "מרכזי"), ("hi", "मर\u{94d}कज\u{93c}ी प\u{94d}रा\u{902}त"), ("hr", "Markazi"), ("hu", "Markazi tartomány"), ("id", "Provinsi Markazi"), ("it", "regione di Markazi"), ("ja", "マルキャズィー州"), ("ka", "მარკაზი"), ("kn", "ಮಾರ\u{ccd}ಕಝ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마르카지 주"), ("lt", "Merkezio provincija"), ("lv", "Markazī"), ("mk", "Маркази"), ("mr", "मर\u{94d}काझी प\u{94d}रा\u{902}त"), ("ms", "Markazi"), ("nb", "Markazi"), ("nl", "Markazi"), ("no", "Markazi"), ("pa", "ਮਰਕਜ\u{a3c}ੀ ਸ\u{a42}ਬਾ"), ("pl", "Markazi"), ("ps", "مرکزي"), ("pt", "Markazi"), ("ro", "Provincia Markazi"), ("ru", "Центральный остан"), ("si", "මර\u{dca}ක\u{dcf}ස\u{dd2} පළ\u{dcf}ත"), ("sr", "Покрајина Маркази"), ("sr_Latn", "Pokrajina Markazi"), ("sv", "Markazi"), ("ta", "ம\u{bbe}ர\u{bcd}கசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ర\u{c4d}క\u{c3e}జ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาร\u{e4c}ซาก\u{e34}"), ("tr", "Merkezi Eyaleti"), ("uk", "Меркезі"), ("ur", "مرکزی"), ("vi", "Markazi"), ("zh", "中央省")]),
                        unofficial_name_list: ["Markazi"].to_vec(),
                    }
                ),
                (
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::IR,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.4651668), longitude: Some(49.4611697), max_latitude: Some(37.4669978), min_latitude: Some(37.4634536), max_longitude: Some(49.4642952), min_longitude: Some(49.4595517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غیلان"), ("az", "Gilan ostanı"), ("be", "Гілян"), ("bg", "Гилян"), ("bn", "গিলন প\u{9cd}রদেশ"), ("ca", "Gilan"), ("ccp", "𑄉\u{11128}𑄣𑄚\u{11134}"), ("ceb", "Gilān"), ("cs", "Gílán"), ("cy", "Gīlān"), ("da", "Gilan"), ("de", "Gilan"), ("el", "Γκιλάν"), ("en", "Gilan"), ("es", "Gilán"), ("eu", "Gilan"), ("fa", "استان گیلان"), ("fi", "Gīlān"), ("fr", "Gilan"), ("gu", "ગિલાન પ\u{acd}રા\u{a82}ત"), ("he", "גילאן"), ("hi", "गिलान प\u{94d}रा\u{902}त"), ("hr", "Gilan"), ("hu", "Gilán tartomány"), ("hy", "Գիլան"), ("id", "Provinsi Gīlān"), ("it", "regione di Gilan"), ("ja", "ギーラーン州"), ("ka", "გილანი"), ("kn", "ಗ\u{cbf}ಲಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "길란 주"), ("ky", "Гилян"), ("lt", "Gilano provincija"), ("lv", "Gīlāna"), ("mk", "Гилан"), ("mn", "Гилян муж"), ("mr", "गिलान प\u{94d}रा\u{902}त"), ("ms", "Gilan"), ("nb", "Gilan"), ("nl", "Gilan"), ("no", "Gilan"), ("pl", "Gilan"), ("ps", "گيلان"), ("pt", "Gilan"), ("ro", "Provincia Gilan"), ("ru", "Гилян"), ("si", "ග\u{dd2}ලන\u{dca} පළ\u{dcf}ත"), ("sl", "Ghelan"), ("sr", "Покрајина Гилан"), ("sr_Latn", "Pokrajina Gilan"), ("sv", "Gilan"), ("ta", "கிளன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3f}ల\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e35}ลาน"), ("tr", "Gilan Eyaleti"), ("uk", "Гілян"), ("ur", "صوبہ جیلان"), ("uz", "Gilon"), ("vi", "Gilan"), ("yue", "基蘭省"), ("yue_Hans", "基兰省"), ("zh", "吉蘭省")]),
                        unofficial_name_list: ["Gilan"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::IR,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.2262393), longitude: Some(52.5318604), max_latitude: Some(36.9641991), min_latitude: Some(35.774562), max_longitude: Some(54.23243799999999), min_longitude: Some(50.308859)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مازندران"), ("az", "Mazandaran ostanı"), ("be", "Астан Мазендэран"), ("bg", "Мазандаран"), ("bn", "মজ\u{9be}ন\u{9cd}দ\u{9be}রন প\u{9cd}রদেশ"), ("ca", "Mazanderan"), ("ccp", "𑄟𑄎𑄚\u{11134}𑄘𑄢𑄚\u{11134}"), ("ceb", "Māzandarān"), ("cs", "Mázandarán"), ("da", "Mazandaran"), ("de", "Māzandarān"), ("el", "Μαζανταράν"), ("en", "Mazandaran"), ("es", "Mazandarán"), ("et", "Māzandarān"), ("eu", "Mazandaran"), ("fa", "استان مازندران"), ("fi", "Māzandarān"), ("fr", "Mazandéran"), ("gu", "મઝાન\u{acd}ડરન પ\u{acd}રા\u{a82}ત"), ("he", "מאזנדראן"), ("hi", "माज\u{93c}\u{902}दरान प\u{94d}रा\u{902}त"), ("hr", "Mazandaran"), ("hu", "Mázandarán tartomány"), ("hy", "Մազանդարան"), ("id", "Provinsi Māzandarān"), ("it", "regione di Mazandaran"), ("ja", "マーザンダラーン州"), ("ka", "მაზანდარანი"), ("kn", "ಮಜಂದರನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마잔다란 주"), ("lt", "Mazenderano provincija"), ("lv", "Māzenderāna"), ("mk", "Мазендеран"), ("mr", "माझा\u{902}दारान प\u{94d}रा\u{902}त"), ("ms", "Mazandaran"), ("nb", "Mazandaran"), ("nl", "Mazandaran"), ("no", "Mazandaran"), ("pl", "Mazandaran"), ("ps", "مازندارن"), ("pt", "Mazandaran"), ("ru", "Мазендеран"), ("si", "මසන\u{dca}ඩරන\u{dca} පළ\u{dcf}ත"), ("sl", "Mazandaran"), ("sr", "Покрајина Мазандаран"), ("sr_Latn", "Pokrajina Mazandaran"), ("sv", "Mazandaran"), ("ta", "மசன\u{bcd}றன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మజందరన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}มะเนะ"), ("tr", "Mazenderan Eyaleti"), ("uk", "Мазендеран"), ("ur", "ماژندران"), ("uz", "Mozandaron"), ("vi", "Mazandaran"), ("zh", "馬贊德蘭省")]),
                        unofficial_name_list: ["Mazandaran"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::IR,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.4703541), longitude: Some(47.0571193), max_latitude: Some(38.473952), min_latitude: Some(38.46699479999999), max_longitude: Some(47.060772), min_longitude: Some(47.0525068)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أذربيجان الشرقية"), ("az", "Şərqi Azərbaycan ostanı"), ("be", "Усходні Азербайджан"), ("bg", "Източен Азербейджан"), ("bn", "প\u{9c2}র\u{9cd}ব আজ\u{9be}রব\u{9be}ইজন প\u{9cd}রদেশ"), ("ca", "Azerbaidjan Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄃𑄎𑄢\u{11134}𑄝\u{1112d}𑄎𑄚\u{11134}"), ("ceb", "Sidlakang Aserbayan"), ("cs", "Východní Ázerbájdžán"), ("da", "Øst-Aserbajdsjan"), ("de", "Ost-Aserbaidschan"), ("el", "Ανατολικό Αζερμπαϊτζάν"), ("en", "East Azerbaijan"), ("es", "Azerbaiyán Oriental"), ("et", "Ida-Aserbaidžaan"), ("eu", "Ekialdeko Azerbaijan"), ("fa", "استان آذربایجان شرقی"), ("fi", "Āzārbāyjān-e Sharqī"), ("fr", "Azerbaïdjan oriental"), ("gu", "પ\u{ac2}ર\u{acd}વ અઝરબ\u{ac8}જાન પ\u{acd}રા\u{a82}ત"), ("he", "מזרח אזרבייג׳ן"), ("hi", "प\u{942}र\u{94d}व अज\u{93c}रब\u{948}जान प\u{94d}रा\u{902}त"), ("hr", "Istočni Azarbajdžan"), ("hu", "Kelet-Azerbajdzsán tartomány"), ("hy", "Արևելյան Ադրբեջան"), ("id", "Provinsi Azarbaijan Timur"), ("it", "regione dell’Azarbaijan orientale"), ("ja", "東アーザルバーイジャーン州"), ("ka", "აღმოსავლეთი აზერბაიჯანი"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಅಜ\u{cc6}ರ\u{ccd}ಬೈಜಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아자르바이잔에샤르키 주"), ("lt", "Rytų Azerbaidžano provincija"), ("lv", "Austrumazerbaidžāna"), ("mk", "Источен Азербејџан"), ("mr", "प\u{942}र\u{94d}व अझरब\u{948}जान प\u{94d}रा\u{902}त"), ("ms", "Azarbaijan Timur"), ("nb", "Øst-Aserbajdsjan"), ("nl", "Oost-Azerbeidzjan"), ("no", "Øst-Aserbajdsjan"), ("pa", "ਪ\u{a42}ਰਬੀ ਅਜ\u{a3c}ਰਬਾਈਜਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Azerbejdżan Wschodni"), ("ps", "مشرقي ازربايجان"), ("pt", "Azerbaijão Oriental"), ("ro", "Provincia Azarbaidjanul de Est"), ("ru", "Восточный Азербайджан"), ("si", "නැගෙනහ\u{dd2}ර අසේර\u{dca}බජ\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Источни Азербејџан"), ("sr_Latn", "Pokrajina Istočni Azerbejdžan"), ("sv", "Östazarbaijan"), ("ta", "கிழக\u{bcd}கு அஜர\u{bcd}பைஜ\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు అజర\u{c4d} బ\u{c48}జ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเซอร\u{e4c}ไบจานตะว\u{e31}นออก"), ("tr", "Doğu Azerbaycan Eyaleti"), ("uk", "Східний Азербайджан"), ("ur", "صوبہ آذربائیجان شرقی"), ("uz", "Sharqiy Ozarbayjon ustoni"), ("vi", "Đông Azarbaijan"), ("yue", "東亞塞拜疆省"), ("yue_Hans", "东亚塞拜疆省"), ("zh", "東亞塞拜疆省")]),
                        unofficial_name_list: ["Azarbayjān-e Khavari", "East Azerbaijan"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::IR,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0489281), longitude: Some(-111.0937311), max_latitude: Some(37.0042599), min_latitude: Some(31.3321771), max_longitude: Some(-109.0452231), min_longitude: Some(-114.8165909)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أذربيجان الغربية"), ("az", "Qərbi Azərbaycan ostanı"), ("bg", "Западен Азербейджан"), ("bn", "পশ\u{9cd}চিম অজ\u{9be}রব\u{9be}ইজন প\u{9cd}রদেশ"), ("ca", "Azerbaidjan Oest"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄃𑄎𑄢\u{11134}𑄝\u{1112d}𑄎𑄚\u{11134}"), ("ceb", "Kasadpang Aserbayan"), ("cs", "Západní Ázerbájdžán"), ("da", "Vest-Aserbajdsjan"), ("de", "West-Aserbaidschan"), ("el", "Δυτικό Αζερμπαϊτζάν"), ("en", "West Azarbaijan"), ("es", "Azerbaiyán Occidental"), ("et", "Lääne-Aserbaidžaan"), ("eu", "Mendebaldeko Azerbaijan"), ("fa", "استان آذربایجان غربی"), ("fi", "Āzārbāyjān-e Gharbī"), ("fr", "Azerbaïdjan occidental"), ("gu", "પશ\u{acd}ચિમ અઝારબાઈજાન પ\u{acd}રા\u{a82}ત"), ("he", "מערב אזרבייג׳ן"), ("hi", "पश\u{94d}चिम अज\u{93c}रब\u{948}जान प\u{94d}रा\u{902}त"), ("hr", "Zapadni Azarbajdžan"), ("hu", "Nyugat-Azerbajdzsán tartomány"), ("hy", "Արևմտյան Ադրբեջան"), ("id", "Provinsi Azarbaijan Barat"), ("it", "regione dell’Azarbaijan occidentale"), ("ja", "西アーザルバーイジャーン州"), ("ka", "დასავლეთი აზერბაიჯანი"), ("kk", "Батыс Әзірбайжан"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಅಜರ\u{ccd}ಬೈಜಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아자르바이잔에가르비 주"), ("lt", "Vakarų Azerbaidžano provincija"), ("lv", "Rietumazerbaidžāna"), ("mk", "Западен Азербејџан"), ("mr", "पश\u{94d}चिम अझरब\u{948}जान प\u{94d}रा\u{902}त"), ("ms", "Azarbaijan Barat"), ("nb", "Vest-Aserbajdsjan"), ("nl", "West-Azerbeidzjan"), ("no", "Vest-Aserbajdsjan"), ("pl", "Azerbejdżan Zachodni"), ("ps", "مغربي ازربايجان"), ("pt", "Azerbaijão Ocidental"), ("ro", "Provincia Azerbaidjanul de Vest"), ("ru", "Западный Азербайджан"), ("si", "බටහ\u{dd2}ර අසර\u{dca}බජ\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Западни Азербејџан"), ("sr_Latn", "Pokrajina Zapadni Azerbejdžan"), ("sv", "Västazarbaijan"), ("ta", "மேற\u{bcd}கு அச\u{bbe}ர\u{bcd}பைஜ\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ అజర\u{c4d}\u{200c}బ\u{c48}జ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเซอร\u{e4c}ไบจานตะว\u{e31}นตก"), ("tr", "Batı Azerbaycan Eyaleti"), ("uk", "Західний Азербайджан"), ("ur", "صوبہ آذربائیجان غربی"), ("uz", "Gʻarbiy Ozarbayjon ustoni"), ("vi", "Tây Azerbaijan"), ("zh", "西亞塞拜然省")]),
                        unofficial_name_list: ["Azarbayjān-e Bakhtari", "West Azerbaijan"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::IR,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3135711), longitude: Some(47.0535126), max_latitude: Some(34.3188878), min_latitude: Some(34.3086925), max_longitude: Some(47.0581582), min_longitude: Some(47.0484325)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كرمانشاه"), ("az", "Kirmanşah ostanı"), ("be", "Керманшах"), ("bg", "Керманшах"), ("bn", "কের\u{9cd}মনশহ প\u{9cd}রদেশ"), ("ca", "Província de Kermanxah"), ("ccp", "𑄇𑄬𑄢\u{11134}𑄟𑄚\u{11134}𑄥𑄦\u{11134}"), ("ceb", "Kermānshāh"), ("cs", "Kermánšáh"), ("da", "Kermanshah"), ("de", "Kermānschāh"), ("el", "Κερμανσάχ"), ("en", "Kermanshah"), ("es", "Kermanshah"), ("eu", "Kermanshah"), ("fa", "استان کرمانشاه"), ("fi", "Kermānšāh"), ("fr", "province de Kermanshah"), ("gu", "કર\u{acd}માનશાહ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז כרמאנשאה"), ("hi", "करमानशाह प\u{94d}रा\u{902}त"), ("hr", "Kermanšah"), ("hy", "Քերմանշահի նահանգ"), ("id", "Provinsi Kermanshah"), ("it", "regione di Kermanshah"), ("ja", "ケルマーンシャー州"), ("ka", "ქირმანშაჰი"), ("kn", "ಕ\u{cc6}ರ\u{ccd}ಮನ\u{ccd}ಶಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "케르만샤 주"), ("lt", "Kermanšaho provincija"), ("lv", "Kermānšāha"), ("mk", "Керманшах"), ("mn", "Керманшах муж"), ("mr", "कर\u{94d}मानशाह प\u{94d}रा\u{902}त"), ("ms", "Kermanshah"), ("nb", "Kermanshah"), ("nl", "Kermanshah"), ("no", "Kermanshah"), ("pl", "Kermanszah"), ("ps", "کرمانشاه ولايت"), ("pt", "Kermanshah"), ("ro", "Provincia Kermanshah"), ("ru", "Керманшах"), ("si", "කර\u{dca}මන\u{dca}ශ\u{dcf} පළ\u{dcf}ත"), ("sr", "Покрајина Керманшах"), ("sr_Latn", "Pokrajina Kermanšah"), ("sv", "Kermanshah"), ("ta", "கெர\u{bcd}மன\u{bcd}ஷ\u{bbe}ஹ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}యల\u{c4d}ట\u{c40} ద\u{c40}వుల ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกอร\u{e4c}ม\u{e31}นซา"), ("tr", "Kirmanşah Eyaleti"), ("uk", "Керманшах"), ("ur", "صوبہ کرمانشاہ"), ("vi", "Kermanshah"), ("zh", "克爾曼沙汗省")]),
                        unofficial_name_list: ["Bakhtaran"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::IR,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.4360149), longitude: Some(49.041312), max_latitude: Some(33.09328), min_latitude: Some(29.9426961), max_longitude: Some(50.433708), min_longitude: Some(47.614482)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خوزستان"), ("az", "Xuzistan ostanı"), ("be", "Хузестан"), ("bg", "Хузестан"), ("bn", "খ\u{9c1}জেস\u{9cd}তন প\u{9cd}রদেশ"), ("ca", "Khuzestan"), ("ccp", "𑄈\u{1112a}𑄎𑄬𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Khuzestān"), ("cs", "Chúzistán"), ("cy", "Khūzestān"), ("da", "Khusistan"), ("de", "Chuzestan"), ("el", "Χουζεστάν"), ("en", "Khuzestan"), ("es", "Juzestán"), ("et", "Khūzestān"), ("eu", "Khuzestan"), ("fa", "استان خوزستان"), ("fi", "Khūzestān"), ("fr", "Khuzestan"), ("gu", "ખ\u{ac1}ઝ\u{ac7}સ\u{acd}તાન પ\u{acd}રા\u{a82}ત"), ("he", "ח׳וזסתאן"), ("hi", "ख\u{93c}\u{942}ज\u{93c}स\u{94d}तान"), ("hr", "Huzestan"), ("hu", "Huzesztán tartomány"), ("hy", "Խուզեստան"), ("id", "Provinsi Khūzestān"), ("it", "regione del Khūzestān"), ("ja", "フーゼスターン州"), ("ka", "ხუზისტანი"), ("kn", "ಖುಜ\u{cc6}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "후제스탄 주"), ("lt", "Chuzestano provincija"), ("lv", "Hūzestāna"), ("mk", "Кузестан"), ("mr", "ख\u{941}जस\u{94d}तान प\u{94d}रा\u{902}त"), ("ms", "Khuzestan"), ("nb", "Khuzestan"), ("nl", "Khuzestan"), ("no", "Khuzestan"), ("pa", "ਖ\u{a41}ਜਿਸਤਾਨ ਰਿਆਸਤ"), ("pl", "Chuzestan"), ("ps", "خوزستان"), ("pt", "Khuzistão"), ("ru", "Хузестан"), ("si", "ක\u{dd4}සේස\u{dca}ත\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sv", "Khuzestan"), ("ta", "க\u{bcd}ஹுஸிஸ\u{bcd}டன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖుజ\u{c46}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e39}เซสถาน"), ("tr", "Huzistan Eyaleti"), ("uk", "Хузестан"), ("ur", "صوبہ خوزستان"), ("uz", "Xuziston ustoni"), ("vi", "Khuzestan"), ("yue", "胡齊斯坦省"), ("yue_Hans", "胡齐斯坦省"), ("zh", "胡齊斯坦省")]),
                        unofficial_name_list: ["Arabistan"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::IR,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.1043813), longitude: Some(53.045893), max_latitude: Some(31.763714), min_latitude: Some(27.147872), max_longitude: Some(55.6170359), min_longitude: Some(50.58247799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فارس"), ("az", "Fars ostanı"), ("be", "Фарс"), ("bg", "Фарс (остан)"), ("bn", "ফর\u{9cd}স প\u{9cd}রদেশ"), ("bs", "Fars (provincija)"), ("ca", "Província de Fars"), ("ccp", "𑄜𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Fārs"), ("cs", "Fárs"), ("da", "Fars (provins)"), ("de", "Fars"), ("el", "Φαρς"), ("en", "Fars"), ("es", "Fars"), ("eu", "Fars"), ("fa", "استان فارس"), ("fi", "Fārs"), ("fr", "Fars"), ("gu", "ફાર\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז פארס"), ("hi", "फ\u{93c}ार\u{94d}स"), ("hr", "Fars"), ("hu", "Fársz tartomány"), ("hy", "Ֆարս"), ("id", "Provinsi Fars"), ("it", "regione di Fars"), ("ja", "ファールス州"), ("ka", "ფარსი (ოსტანი)"), ("kk", "Фарс облысы"), ("kn", "ಫಾರ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파르스 주"), ("lt", "Farso provincija"), ("lv", "Fārsa"), ("mk", "Фарс"), ("mr", "फार\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Fars"), ("nb", "Fars"), ("nl", "Fars"), ("no", "Fars"), ("pa", "ਫ\u{a3c}ਾਰਸ ਸ\u{a42}ਬਾ"), ("pl", "Fars"), ("ps", "د فارس ولايت"), ("pt", "Fars (província)"), ("ro", "Persida"), ("ru", "Фарс"), ("si", "ෆ\u{dcf}ර\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Фарс"), ("sr_Latn", "Pokrajina Fars"), ("sv", "Fars (provins)"), ("ta", "ப\u{bbe}ர\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఫ\u{c3e}ర\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟาร\u{e4c}ส"), ("tr", "Fars Eyaleti"), ("uk", "Фарс (остан)"), ("ur", "صوبہ فارس"), ("uz", "Fors"), ("vi", "Fars (tỉnh)"), ("yue", "法爾斯省"), ("yue_Hans", "法尔斯省"), ("zh", "法爾斯省")]),
                        unofficial_name_list: ["Fars"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::IR,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.2839379), longitude: Some(57.0833628), max_latitude: Some(30.3459171), min_latitude: Some(30.2001849), max_longitude: Some(57.1968843), min_longitude: Some(56.9356155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كرمان"), ("az", "Kirman ostanı"), ("bg", "Керман"), ("bn", "কের\u{9cd}মন প\u{9cd}রদেশ"), ("ca", "Província de Kerman"), ("ccp", "𑄇𑄬𑄢\u{11134}𑄟𑄚\u{11134}"), ("ceb", "Kermān"), ("cs", "Kermán"), ("da", "Kerman"), ("de", "Kerman"), ("el", "Κέρμαν"), ("en", "Kerman"), ("es", "Kermán"), ("et", "Kermāni provints"), ("eu", "Kerman"), ("fa", "استان کرمان"), ("fi", "Kermān"), ("fr", "province de Kerman"), ("gu", "કર\u{acd}મન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז כרמאן"), ("hi", "करमान प\u{94d}रा\u{902}त"), ("hr", "Kerman"), ("hu", "Kermán tartomány"), ("id", "Provinsi Kermān"), ("it", "regione di Kerman"), ("ja", "ケルマーン州"), ("ka", "ქერმანი"), ("kn", "ಕರ\u{ccd}ಮನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "케르만 주"), ("lt", "Kermano provincija"), ("lv", "Kermāna"), ("mk", "Керман"), ("mr", "कर\u{94d}मान प\u{94d}रा\u{902}त"), ("ms", "Kerman"), ("nb", "Kermān"), ("nl", "Kerman"), ("no", "Kermān"), ("pl", "Kerman"), ("ps", "د کرمان ولايت"), ("pt", "Kerman"), ("ro", "Kerman"), ("ru", "Керман"), ("si", "කර\u{dca}මන\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Керман"), ("sr_Latn", "Pokrajina Kerman"), ("sv", "Kerman"), ("ta", "கெர\u{bcd}மன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c46}ర\u{c4d}మన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เคอร\u{e4c}แมน"), ("tr", "Kirman Eyaleti"), ("uk", "Керман"), ("ur", "صوبہ کرمان"), ("vi", "Kerman"), ("zh", "克爾曼省")]),
                        unofficial_name_list: ["Kerman"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::IR,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1020253), longitude: Some(59.1041758), max_latitude: Some(37.7905918), min_latitude: Some(33.4001927), max_longitude: Some(61.282579), min_longitude: Some(56.4763309)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خراسان رضوي"), ("az", "Rəzəvi Xorasan ostanı"), ("bg", "Разави Хорасан"), ("bn", "র\u{9be}জ\u{9be}ভি খোরসন প\u{9cd}রদেশ"), ("ca", "Razavi Khorasan"), ("ccp", "𑄢𑄌\u{11134}𑄞\u{11128} 𑄈\u{1112e}𑄢\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Khorāsān-e Razavi"), ("cs", "Chorásán Razaví"), ("da", "Razavi Khorasan"), ("de", "Razavi-Chorasan"), ("el", "Ραζάβι Χορασάν"), ("en", "Razavi Khorasan"), ("es", "Jorasán Razaví"), ("et", "Razavi-Khorāsān"), ("eu", "Razavi Khorasan"), ("fa", "استان خراسان رضوی"), ("fi", "Khorāsān-e Razavi"), ("fr", "Khorasan-e-Razavi"), ("gu", "રઝાવી ખોરાસન પ\u{acd}રા\u{a82}ત"), ("he", "ח׳וראסאן רזאווי"), ("hi", "रज\u{93c}ावी ख\u{93c}ोरासान"), ("hr", "Razavi Horasan"), ("hu", "Razavi Horászán tartomány"), ("hy", "Ռազավի Խորասան"), ("id", "Provinsi Razavi Khorasan"), ("it", "regione di Razavi Khorasan"), ("ja", "ラザヴィー・ホラーサーン州"), ("ka", "ხორასან-რეზავის ოსტანი"), ("kn", "ರಝವ\u{cbf} ಖೊರಾಸನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "호라산에라자비 주"), ("lt", "Rezavio Chorasano provincija"), ("lv", "Rezāvi Horasāna"), ("mk", "Разави Корасан"), ("mr", "रझावी खोरासान प\u{94d}रा\u{902}त"), ("ms", "Razavi Khorasan"), ("nb", "Razavi-Khorasan"), ("nl", "Razavi-Khorasan"), ("no", "Razavi-Khorasan"), ("pl", "Chorasan-e Rezawi"), ("ps", "د رضوي خراسان ولايت"), ("pt", "Coração Razavi"), ("ru", "Хорасан-Резави"), ("si", "රසව\u{dd2} කොරසන\u{dca} පළ\u{dcf}ත"), ("sq", "Razavi Horasan"), ("sr", "Покрајина Разави Корасан"), ("sr_Latn", "Pokrajina Razavi Korasan"), ("sv", "Razavikhorasan"), ("ta", "ர\u{bbe}ச\u{bbe}வி க\u{bcd}ஹோரசன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3e}జ\u{c3e}వ\u{c3f} క\u{c4b}ర\u{c4b}సన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคราซานราซาว\u{e35}"), ("tr", "Razavi Horasan Eyaleti"), ("uk", "Хорасан-Резаві"), ("ur", "صوبہ خراسان رضوی"), ("uz", "Xurosoni Rizoviy ustoni"), ("vi", "Razavi Khorasan"), ("zh", "礼萨呼罗珊省")]),
                        unofficial_name_list: ["Khorasan-e Razavi"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::IR,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.6546275), longitude: Some(51.66798259999999), max_latitude: Some(32.8200277), min_latitude: Some(32.5045501), max_longitude: Some(51.8486024), min_longitude: Some(51.5254498)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أصفهان"), ("az", "İsfahan ostanı"), ("be", "астан Ісфахан"), ("bg", "Исфахан (остан)"), ("bn", "এসফ\u{9be}হন প\u{9cd}রদেশ"), ("bs", "Isfahanska pokrajina"), ("ca", "Província d’Isfahan"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄜𑄦𑄚\u{11134}"), ("ceb", "Esfahān"), ("cs", "Isfahán (provincie)"), ("da", "Isfahan (provins)"), ("de", "Isfahan"), ("el", "Ισφαχάν"), ("en", "Isfahan"), ("es", "Isfahán"), ("eu", "Ispahan probintzia"), ("fa", "استان اصفهان"), ("fi", "Eṣfahān (provinssi)"), ("fr", "province d’Ispahan"), ("gl", "Provincia de Esfahan"), ("gu", "ઇસ\u{acd}ફહન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אספהאן"), ("hi", "इस\u{94d}फ\u{93c}हान प\u{94d}रा\u{902}त"), ("hr", "Isfahanska pokrajina"), ("hu", "Iszfahán tartomány"), ("hy", "Իսֆահան"), ("id", "Provinsi Isfahan"), ("it", "regione di Esfahan"), ("ja", "エスファハーン州"), ("ka", "ისპაჰანი (ოსტანი)"), ("kk", "Исфаһан облысы"), ("kn", "ಇಸ\u{ccd}ಫಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "이스파한 주"), ("lt", "Isfahano provincija"), ("lv", "Isfahāna (ostāns)"), ("mk", "Исфахан"), ("mr", "इस\u{94d}फहान प\u{94d}रा\u{902}त"), ("ms", "Isfahan (wilayah)"), ("nb", "Isfahan (provins)"), ("nl", "Isfahan"), ("no", "Isfahan (provins)"), ("pl", "Isfahan"), ("ps", "د اصفهان ولايت"), ("pt", "Ispaão (província)"), ("ru", "Исфахан"), ("si", "ඉස\u{dca}ෆහන\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Исфахан"), ("sr_Latn", "Pokrajina Isfahan"), ("sv", "Esfahan (provins)"), ("ta", "இசுப\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇస\u{c4d}ఫహ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "มณฑลพราโฮวา"), ("tr", "İsfahan Eyaleti"), ("uk", "Ісфахан (остан)"), ("ur", "صوبہ اصفہان"), ("vi", "Isfahan (tỉnh)"), ("yue", "伊斯法罕省"), ("yue_Hans", "伊斯法罕省"), ("zh", "伊斯法罕省")]),
                        unofficial_name_list: ["Isfahan"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::IR,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.2001559), longitude: Some(60.6392527), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيستان وبلوشستان"), ("az", "Sistan və Bəlucistan ostanı"), ("bg", "Систан и Белуджистан"), ("bn", "সিস\u{9cd}তন ও ব\u{9be}ল\u{9c1}চেস\u{9cd}তন প\u{9cd}রদেশ"), ("ca", "Sistan i Balutxistan"), ("ccp", "𑄥\u{11128}𑄌\u{11134}𑄑𑄚\u{11134} 𑄃\u{11133}𑄃 𑄝𑄣\u{1112a}𑄌𑄬𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Sistān o Baluchestān"), ("cs", "Sístán a Balúčistán"), ("cy", "Sistan a Baluchestan"), ("da", "Sistan og Baluchistan"), ("de", "Sistan und Belutschistan"), ("el", "Σιστάν και Μπαλουτσεστάν"), ("en", "Sistan and Baluchestan"), ("es", "Sistán y Baluchistán"), ("et", "Sīstān va Balūchestān"), ("eu", "Sistan eta Balutxistan"), ("fa", "استان سیستان و بلوچستان"), ("fi", "Sīstān ja Balūchestān"), ("fr", "Sistan-et-Baloutchistan"), ("gu", "સિસ\u{acd}તાન એન\u{acd}ડ બલ\u{ac1}ચિસ\u{acd}તાન"), ("he", "סיסתאן ובלוצ׳סתאן"), ("hi", "सिस\u{94d}तान और बल\u{942}चिस\u{94d}तान"), ("hr", "Sistan i Beludžistan"), ("hu", "Szisztán és Beludzsisztán tartomány"), ("id", "Provinsi Sistan dan Baluchestan"), ("it", "regione del Sistan e Baluchistan"), ("ja", "スィースターン・バルーチェスターン州"), ("ka", "სისტანი და ბელუჯისტანი"), ("kn", "ಸ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಮತ\u{ccd}ತು ಬಾಲ\u{ccd}ಲುಚ\u{cc6}ಥಾನ\u{ccd}"), ("ko", "시스탄오발루체스탄 주"), ("lt", "Sistano ir Beludžistano provincija"), ("lv", "Sīstāna un Beludžistāna"), ("mk", "Систан и Балуџистан"), ("mr", "सिस\u{94d}तान व बल\u{941}चिस\u{94d}तान प\u{94d}रा\u{902}त"), ("ms", "Sistan dan Baluchestan"), ("nb", "Sistan og Balutsjistan"), ("nl", "Sistan en Beloetsjistan"), ("no", "Sistan og Balutsjistan"), ("pl", "Sistan i Beludżystan"), ("ps", "د سيستان او بلوچستان ولايت"), ("pt", "Sistan e Baluchistão"), ("ro", "Provincia Sistan-Baluchestan"), ("ru", "Систан и Белуджистан"), ("si", "ස\u{dd2}ස\u{dca}ටන\u{dca} සහ බල\u{dd4}චෙස\u{dca}ට\u{dcf}න\u{dca}"), ("sr", "Покрајина Систан и Белуџистан"), ("sr_Latn", "Pokrajina Sistan i Beludžistan"), ("sv", "Sistan och Baluchistan"), ("sw", "Mkoa wa Sistan na Baluchistan"), ("ta", "சிச\u{bcd}த\u{bbe}ன\u{bcd} மற\u{bcd}றும\u{bcd} பலுசிஸ\u{bcd}த\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}స\u{c4d}ట\u{c3e}న\u{c4d} మర\u{c3f}యు బలూచ\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e34}สถานและบาล\u{e39}จ\u{e34}สถาน"), ("tr", "Sistan ve Belucistan Eyaleti"), ("uk", "Систан і Белуджистан"), ("ur", "صوبہ سیستان و بلوچستان"), ("uz", "Siston va Balujiston ustoni"), ("vi", "Tỉnh Sistan và Baluchestan"), ("zh", "錫斯坦－俾路支斯坦省")]),
                        unofficial_name_list: ["Sistan-e Baluchistan"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::IR,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3135711), longitude: Some(47.0535126), max_latitude: Some(34.3188878), min_latitude: Some(34.3086925), max_longitude: Some(47.0581582), min_longitude: Some(47.0484325)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كردستان"), ("az", "Kürdüstan ostanı"), ("be", "Астан Курдыстан"), ("bg", "Кордестан"), ("bn", "কোর\u{9cd}দেস\u{9cd}তন প\u{9cd}রদেশ"), ("bs", "Kurdistanska pokrajina"), ("ca", "Província del Kurdistan"), ("ccp", "𑄇\u{1112a}𑄢\u{11134}𑄓\u{11128}𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Kordestān"), ("cs", "Kurdistán"), ("cy", "Cyrdistan"), ("da", "Kurdistan"), ("de", "Kordestān"), ("el", "Κουρδιστάν"), ("en", "Kurdistan"), ("es", "Kurdistán"), ("et", "Kurdistani provints"), ("eu", "Kurdistango probintzia"), ("fa", "استان کردستان"), ("fi", "Kordestān"), ("fr", "Province iranienne du Kurdistan"), ("gu", "ક\u{ac1}ર\u{acd}દિસ\u{acd}તાન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז כורדיסתאן"), ("hi", "क\u{941}र\u{94d}दिस\u{94d}तान प\u{94d}रा\u{902}त"), ("hr", "Kurdistan"), ("hu", "Kurdisztán tartomány"), ("id", "Provinsi Kurdistan"), ("it", "regione del Kurdistan"), ("ja", "コルデスターン州"), ("ka", "ქურთისტანის ოსტანი"), ("kn", "ಕುರ\u{ccd}ದ\u{cbf}ಸ\u{ccd}ತಾನ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코르데스탄 주"), ("lt", "Kurdistano provincija"), ("lv", "Kurdistāna"), ("mk", "Курдистан"), ("mr", "क\u{941}र\u{94d}दिस\u{94d}तान प\u{94d}रा\u{902}त"), ("ms", "Kurdistan"), ("nb", "Kurdistan"), ("nl", "Kordestan"), ("no", "Kurdistan"), ("pa", "ਕ\u{a41}ਰਦਿਸਤਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Kurdystan"), ("ps", "کردستان ولايت"), ("pt", "Curdistão"), ("ro", "Provincia Kurdistan"), ("ru", "Курдистан"), ("si", "ක\u{dd4}ර\u{dca}ද\u{dd2}ස\u{dca}ත\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Курдистан"), ("sr_Latn", "Kurdistan"), ("sv", "Kurdistan"), ("ta", "குர\u{bcd}டிஸ\u{bcd}ட\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కుర\u{c4d}ద\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเคอร\u{e4c}ด\u{e34}สถาน"), ("tr", "Kürdistan Eyaleti"), ("uk", "Курдистан"), ("ur", "صوبہ کردستان"), ("uz", "Kurdiston ustoni"), ("vi", "Kurdistan"), ("yue", "庫爾德斯坦省"), ("yue_Hans", "库尔德斯坦省"), ("zh", "庫爾德斯坦省")]),
                        unofficial_name_list: ["Kurdestan", "Kurdistan"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::IR,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.8), longitude: Some(48.516667), max_latitude: Some(34.8825162), min_latitude: Some(34.7421415), max_longitude: Some(48.5836458), min_longitude: Some(48.4364891)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "همدان"), ("az", "Həmədan ostanı"), ("be", "Хамадан"), ("bg", "Хамадан"), ("bn", "হ\u{9be}ম\u{9be}দন প\u{9cd}রদেশ"), ("ca", "Província d’Hamadan"), ("ccp", "𑄦𑄟\u{11134}𑄘𑄚\u{11134}"), ("ceb", "Hamadān"), ("cs", "Hamadán"), ("da", "Hamadan"), ("de", "Hamadan"), ("el", "Χαμαντάν"), ("en", "Hamadan"), ("es", "Hamadán"), ("eu", "Hamadan probintzia"), ("fa", "استان همدان"), ("fi", "Hamadān"), ("fr", "province de Hamedan"), ("gu", "હમાદાન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז המדאן"), ("hi", "हमादान प\u{94d}रा\u{902}त"), ("hr", "Hamadan"), ("hu", "Hamadán tartomány"), ("id", "Provinsi Hamadān"), ("it", "regione di Hamadan"), ("ja", "ハマダーン州"), ("ka", "ჰამადანი"), ("kn", "ಹಮಾದಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "하마단 주"), ("lt", "Hamadano provincija"), ("lv", "Hamadāna"), ("mk", "Хамадан"), ("mr", "हमादान प\u{94d}रा\u{902}त"), ("ms", "Hamedan"), ("nb", "Hamadan"), ("ne", "हामादान क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Hamadan"), ("no", "Hamadan"), ("pl", "Hamadan"), ("ps", "همدان ولايت"), ("pt", "Hamadã"), ("ro", "Provincia Hamadān"), ("ru", "Хамадан"), ("si", "හමඩන\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Хамадан"), ("sr_Latn", "Pokrajina Hamadan"), ("sv", "Hamadan"), ("ta", "அம\u{bbe}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హమదన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e31}ปเปอ ร\u{e34}เวอ ด\u{e34}ว\u{e34}ช\u{e31}\u{e48}น"), ("tr", "Hamedan Eyaleti"), ("uk", "Хамадан"), ("ur", "صوبہ ہمدان"), ("vi", "Hamadan"), ("yue", "哈馬丹省"), ("yue_Hans", "哈马丹省"), ("zh", "哈馬丹省")]),
                        unofficial_name_list: ["Hamedan"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::IR,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.9614348), longitude: Some(50.8456323), max_latitude: Some(32.700775), min_latitude: Some(31.112481), max_longitude: Some(51.3524481), min_longitude: Some(49.76081509999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تشهارمحال وبختياري"), ("az", "Çahar-Mahal və Bəxtiyari ostanı"), ("be", "Астан Чахармахаль і Бахціярыя"), ("bn", "চ\u{9be}হর-ম\u{9be}হ\u{9be}ল ও ব\u{9be}খতেয\u{9bc}রি প\u{9cd}রদেশ"), ("ca", "Chahar Mahall i Bakhtiari"), ("ccp", "𑄌𑄦𑄢\u{11134}𑄟\u{11127}𑄦\u{11127}𑄣\u{11134} 𑄃\u{11133}𑄃 𑄝𑄇\u{11134}𑄑\u{11128}𑄠𑄢\u{11128}"), ("ceb", "Chahārmahāl o Bakhtiyāri"), ("cs", "Čahármahál a Bachtijárí"), ("da", "Chahar Mahaal og Bakhtiari"), ("de", "Tschahār Mahāl und Bachtiyāri"), ("el", "Τσαχαρμαχάλ και Μπαχτιαρί"), ("en", "Chaharmahal and Bakhtiari"), ("es", "Chahar y Bajtiari"), ("et", "Chahār Maḩāll va Bakhtīārī"), ("eu", "Chahar eta Bakhtiari"), ("fa", "استان چهارمحال و بختیاری"), ("fi", "Chahār Mahāl ja Bakhtīārī"), ("fr", "Chahar Mahaal et Bakhtiari"), ("gu", "ચાહરમહલ એન\u{acd}ડ બખ\u{acd}તિઆરી પ\u{acd}રા\u{a82}ત"), ("he", "צ׳הארמחאל ובח׳תיארי"), ("hi", "चहार\u{94d}महाल और बाख\u{93c}\u{94d}तियारी प\u{94d}रा\u{902}त"), ("hr", "Čahar Mahal i Bahtijari"), ("hu", "Csahármahál és Bahtijári tartomány"), ("hy", "Չարմահալ և Բախտիյարի"), ("id", "Provinsi Chaharmahal dan Bakhtiari"), ("it", "regione di Chahar Mahal e Bakhtiari"), ("ka", "ჩახარ-მახალი და ბახთიარია"), ("kn", "ಚಹರ\u{ccd}ಮಹಲ\u{ccd} ಮತ\u{ccd}ತು ಬಖ\u{ccd}ತರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "차하르마할에바흐티아리 주"), ("lt", "Čeharmehalio ir Bachtiarijos provincija"), ("lv", "Čahārmahāla un Bahtijārī"), ("mk", "Чахар Махал и Бахтијари"), ("mr", "चहर\u{94d}महल अ\u{901}ड बख\u{94d}तीरी प\u{94d}रा\u{902}त"), ("ms", "Chaharmahal dan Bakhtiari"), ("nb", "Chahar Mahaal og Bakhtiari"), ("nl", "Chahar Mahaal en Bakhtiari"), ("no", "Chahar Mahaal og Bakhtiari"), ("pl", "Czahar Mahal wa Bachtijari"), ("ps", "چار محل و بختياري"), ("pt", "Chahar Mahaal e Bakhtiari"), ("ru", "Чехармехаль и Бахтиария"), ("si", "චහර\u{dca}මහල\u{dca} සහ බක\u{dca}ට\u{dd2}යර\u{dd2} පළ\u{dcf}ත"), ("sr", "Покрајина Чахар Махал и Бактијари"), ("sr_Latn", "Pokrajina Čahar Mahal i Baktijari"), ("sv", "Chahar Mahal och Bakhtiari"), ("ta", "சஹ\u{bcd}ரமஹ\u{bbe}ல\u{bcd} அண\u{bcd}ட\u{bcd} ப\u{bbe}க\u{bcd}ஹ\u{bcd}டிரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3e}హర\u{c4d}మహల\u{c4d} మర\u{c3f}యు బఖ\u{c4d}త\u{c3f}య\u{c3e}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชาฮาร\u{e4c}มาฮาลและบ\u{e31}คเต\u{e35}ยร\u{e35}"), ("tr", "Çaharmahal ve Bahtiyari Eyaleti"), ("uk", "Чехармехаль і Бахтиарія"), ("ur", "صوبہ چہارمحال و بختیاری"), ("vi", "Chaharmahal và Bakhtiari"), ("yue", "恰哈馬哈勒－巴赫蒂亞里省"), ("yue_Hans", "恰哈马哈勒－巴赫蒂亚里省"), ("zh", "恰哈馬哈勒－巴赫蒂亞里省")]),
                        unofficial_name_list: ["Chaharmahal Bakhtiari"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::IR,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5818394), longitude: Some(48.3988186), max_latitude: Some(34.412064), min_latitude: Some(32.643707), max_longitude: Some(50.0329071), min_longitude: Some(46.87204699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لرستان"), ("az", "Luristan ostanı"), ("bg", "Лурестан"), ("bn", "লোরেস\u{9cd}তন প\u{9cd}রদেশ"), ("ca", "Província de Lorestan"), ("ccp", "𑄣\u{1112e}𑄢𑄬𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Lorestān"), ("cs", "Lorestán"), ("cy", "Lorestān (talaith)"), ("da", "Luristan (provins)"), ("de", "Lorestan"), ("el", "Λορεστάν"), ("en", "Lorestan"), ("es", "Lorestán"), ("et", "Lorestān"), ("eu", "Lorestan"), ("fa", "استان لرستان"), ("fi", "Lorestān"), ("fr", "Lorestan"), ("gu", "લોર\u{ac7}સ\u{acd}તાન પ\u{acd}રા\u{a82}ત"), ("he", "לורסתאן"), ("hi", "ल\u{942}रिस\u{94d}तान"), ("hr", "Luristan"), ("hu", "Loresztán tartomány"), ("id", "Provinsi Lorestān"), ("it", "regione del Lorestan"), ("ja", "ロレスターン州"), ("ka", "ლურისტანი (ოსტანი)"), ("kn", "ಲೊರ\u{cc6}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로레스탄 주"), ("lt", "Lurestano provincija"), ("lv", "Luristāna"), ("mk", "Лорестан"), ("mr", "लोर\u{947}स\u{94d}टाइन प\u{94d}रा\u{902}त"), ("ms", "Lorestan"), ("nb", "Luristan"), ("nl", "Lorestan"), ("no", "Luristan"), ("pl", "Lorestan"), ("ps", "لرستان ولایت"), ("pt", "Lorestão"), ("ro", "Provincia Lorestan"), ("ru", "Лурестан"), ("si", "ලොරෙස\u{dca}ටන\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Лорестан"), ("sr_Latn", "Pokrajina Lorestan"), ("sv", "Lorestan"), ("ta", "லொரெஸ\u{bcd}டன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c4b}ర\u{c46}స\u{c4d}ట\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโลเรสตาน"), ("tr", "Luristan Eyaleti"), ("uk", "Лурестан"), ("ur", "صوبہ لرستان"), ("uz", "Luriston"), ("vi", "Lorestan (tỉnh)"), ("zh", "洛雷斯坦省")]),
                        unofficial_name_list: ["Lorestan"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::IR,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.6375), longitude: Some(46.422778), max_latitude: Some(33.6528069), min_latitude: Some(33.6210342), max_longitude: Some(46.44824269999999), min_longitude: Some(46.3797726)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "عيلام"), ("az", "İlam ostanı"), ("bg", "Илам"), ("bn", "ইলম প\u{9cd}রদেশ"), ("ccp", "𑄃\u{11128}𑄣𑄟\u{11134}"), ("ceb", "Ilām"), ("cs", "Ílám"), ("da", "Ilam"), ("de", "Ilam"), ("en", "Ilam"), ("es", "Ilam"), ("eu", "Ilam"), ("fa", "استان ایلام"), ("fi", "Ilām"), ("fr", "province d’Ilam"), ("he", "אילאם"), ("hi", "ईलम प\u{94d}रा\u{902}त"), ("hr", "Ilam"), ("hu", "Ilám tartomány"), ("hy", "Իլամ"), ("id", "Provinsi Īlām"), ("it", "regione di Ilam"), ("ja", "イーラーム州"), ("ka", "ილამი"), ("ko", "일람 주"), ("lt", "Ilamo provincija"), ("lv", "Īlāma"), ("mk", "Илам"), ("mr", "इलाम प\u{94d}रा\u{902}त"), ("ms", "Ilam"), ("nb", "Ilam"), ("nl", "Ilam"), ("no", "Ilam"), ("pl", "Ilam"), ("ps", "ايلام ولايت"), ("pt", "Ilam"), ("ro", "Provincia Īlām"), ("ru", "Илам"), ("sr", "Покрајина Илам"), ("sr_Latn", "Pokrajina Ilam"), ("sv", "Ilam"), ("ta", "இல\u{bbe}ம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "İlam Eyaleti"), ("uk", "Ілам"), ("ur", "صوبہ ایلام"), ("vi", "Ilam"), ("yue", "伊拉姆省"), ("yue_Hans", "伊拉姆省"), ("zh", "伊拉姆省")]),
                        unofficial_name_list: ["Ilam"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::IR,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.724586), longitude: Some(50.8456323), max_latitude: Some(31.5126911), min_latitude: Some(30.171534), max_longitude: Some(51.69622409999999), min_longitude: Some(49.913279)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كهكيلويه وبوير أحمد"), ("az", "Kohgiluyə və Boyer-Əhməd ostanı"), ("bg", "Кохкилюен и Байер Ахмед"), ("bn", "কোহগিল\u{9c1}ইয\u{9bc}ে ও ব\u{9c1}ইয\u{9bc}\u{9be}র আহম\u{9be}দ প\u{9cd}রদেশ"), ("ca", "Kohgiluyeh i Boyer-Ahmad"), ("ccp", "𑄇\u{1112e}𑄦\u{11134}𑄉\u{11128}𑄣\u{1112a}𑄠𑄬𑄦\u{11134} 𑄃\u{11133}𑄃 𑄝\u{11127}𑄠𑄢\u{11134}-𑄃𑄦\u{11134}𑄟𑄬𑄖\u{11134}"), ("ceb", "Kohgiluyeh o Boyer-Ahmad"), ("cs", "Kohgíluje a Bójer-Ahmad"), ("da", "Kohkiluyeh og Buyer Ahmad"), ("de", "Kohgiluye und Boyer Ahmad"), ("el", "Κογιλουγέ και Μπογέρ Αχμάντ"), ("en", "Kohgiluyeh and Boyer-Ahmad"), ("es", "Kohkiluyeh y Buyer Ahmad"), ("eu", "Kohkiluyeh eta Buyer Ahmad"), ("fa", "استان کهگیلویه و بویراحمد"), ("fi", "Boyer Ahmadī ja Kohkīlūyeh"), ("fr", "Kohkiluyeh et Buyer Ahmad"), ("gu", "કોગિલ\u{ac1}યહ એન\u{acd}ડ બોયર-અહમદ પ\u{acd}રા\u{a82}ત"), ("he", "כהגילויה ובויראחמד"), ("hi", "कोगिल\u{941}य\u{947} और बोयर-अख\u{93c}\u{94d}मद प\u{94d}रा\u{902}त"), ("hr", "Kuhgiluje i Bojer Ahmad"), ("hu", "Kohgiluje és Bujer Ahmad tartomány"), ("id", "Provinsi Kohgiluyeh dan Boyer-Ahmad"), ("it", "regione di Kohgiluyeh e Buyer Ahmad"), ("ka", "ქოხგილუე და ბოიერაჰმედი"), ("kn", "ಕೋಹ\u{cbf}ಲ\u{ccd}ಯುಯ\u{cc6}ಹ\u{ccd} ಮತ\u{ccd}ತು ಬೊಯ\u{cc6}ರ\u{ccd}-ಅಹ\u{ccd}ಮದ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코길루예부예르아마드 주"), ("lt", "Kohgilujės ir Bojerahmedo provincija"), ("lv", "Kohkīlūje un Būjerahmada"), ("mk", "Кохкилуех и Бујер Ахмад"), ("mr", "कोगिल\u{941}य\u{947} व बोय\u{947}र-अहमद प\u{94d}रा\u{902}त"), ("ms", "Kohkiluyeh dan Buyer Ahmad"), ("nb", "Kohkiluyeh og Buyer Ahmad"), ("nl", "Kohgiluyeh en Boyer Ahmad"), ("no", "Kohkiluyeh og Buyer Ahmad"), ("pl", "Kohgiluje wa Bujerahmad"), ("ps", "کهگلويه و بويراحمد"), ("pt", "Kohkiluyeh e Buyer Ahmad"), ("ru", "Кохгилуйе и Бойерахмед"), ("si", "කොග\u{dd2}ල\u{dd4}යේ සහ බෝයෙර\u{dca}-අහමඩ\u{dca} පළ\u{dcf}ත"), ("sq", "Kohgiluje dhe Bojer-Ahmad"), ("sr", "Покрајина Кохкилујех и Бујер Ахмад"), ("sr_Latn", "Pokrajina Kohkilujeh i Bujer Ahmad"), ("sv", "Kohgiluyeh och Buyer Ahmad"), ("ta", "கோஹ\u{bcd}கிலுஏஹ\u{bcd} அண\u{bcd}ட\u{bcd} போயர\u{bcd} -அஹ\u{bcd}மத\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}హ\u{c4d}\u{200c}గుల\u{c3f}య\u{c47} మర\u{c3f}యు బ\u{c4b}యర\u{c4d}-అహ\u{c4d}మద\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ด โคก\u{e34}โลเย และ โบเยอ อาฮ\u{e4c}ม\u{e31}ด"), ("tr", "Kohgiluye ve Buyer Ahmed Eyaleti"), ("uk", "Кохгілує і Бойєрахмед"), ("ur", "صوبہ کہگیلویہ و بویراحمد"), ("vi", "Tỉnh Kohgiluyeh và Boyer-Ahmad"), ("zh", "科吉盧耶－博韋艾哈邁德省")]),
                        unofficial_name_list: ["Boyer Ahmad-e Kohkiluyeh", "Kohkilouyeh"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::IR,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.97802549999999), longitude: Some(50.8379918), max_latitude: Some(28.9784922), min_latitude: Some(28.9775018), max_longitude: Some(50.8385061), min_longitude: Some(50.8374439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوشهر"), ("az", "Buşehr ostanı"), ("bg", "Бушехър"), ("bn", "ব\u{9c1}শেহর প\u{9cd}রদেশ"), ("ca", "Província de Buixehr"), ("ccp", "𑄝\u{1112a}𑄥𑄬𑄦\u{11134}𑄢\u{11134}"), ("ceb", "Bushehr"), ("cs", "Búšehr"), ("da", "Bushehr"), ("de", "Buschehr"), ("en", "Bushehr"), ("es", "Bushehr"), ("et", "Būshehri provints"), ("eu", "Bushehr"), ("fa", "استان بوشهر"), ("fi", "Būšehr"), ("fr", "province de Bouchehr"), ("he", "בושהר"), ("hi", "ब\u{941}शहर प\u{94d}रा\u{902}त"), ("hr", "Bušer"), ("hu", "Busehr tartomány"), ("hy", "Բուշեր"), ("id", "Provinsi Bushehr"), ("it", "regione di Bushehr"), ("ja", "ブーシェフル州"), ("ka", "ბუშირი"), ("kk", "Бушеһр"), ("ko", "부셰르 주"), ("lt", "Bušehro provincija"), ("lv", "Būšehra"), ("mk", "Бушер"), ("mr", "ब\u{941}शहर प\u{94d}रा\u{902}त"), ("ms", "Bushehr"), ("nb", "Bushehr"), ("nl", "Bushehr"), ("no", "Bushehr"), ("pa", "ਬ\u{a41}ਸ\u{a3c}ਹਿਰ ਸ\u{a42}ਬਾ"), ("pl", "Buszehr"), ("ps", "بوشهر ولايت"), ("pt", "Bushehr"), ("ru", "Бушир"), ("sr", "Покрајина Бушер"), ("sr_Latn", "Pokrajina Bušer"), ("sv", "Bushehr"), ("ta", "புஷெர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "Buşehr Eyaleti"), ("uk", "Бушер"), ("ur", "صوبہ بوشهر"), ("vi", "Bushehr"), ("yue", "布舍爾省"), ("yue_Hans", "布舍尔省"), ("zh", "布什爾省")]),
                        unofficial_name_list: ["Bushehr"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::IR,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.666667), longitude: Some(48.48333299999999), max_latitude: Some(36.726124), min_latitude: Some(36.6359171), max_longitude: Some(48.6016273), min_longitude: Some(48.384819)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زنجان"), ("az", "Zəncan ostanı"), ("be", "Зенджан"), ("bg", "Занян (остан)"), ("bn", "জ\u{9be}ন\u{9cd}\u{200c}জন প\u{9cd}রদেশ"), ("ca", "Província de Zanjan"), ("ccp", "𑄎𑄚\u{11134}𑄎𑄚\u{11134}"), ("ceb", "Zanjān"), ("cs", "Zandžán"), ("da", "Zanjan (provins)"), ("de", "Zandschan"), ("el", "Ζαντζάν"), ("en", "Zanjan"), ("es", "Provincia de Zanjan"), ("eu", "Zanjan"), ("fa", "استان زنجان"), ("fi", "Zanjān (provinssi)"), ("fr", "province de Zanjan"), ("he", "זנג׳אן"), ("hi", "ज\u{93c}\u{902}जन प\u{94d}रा\u{902}त"), ("hr", "Zandžan (pokrajina)"), ("hu", "Zandzsán tartomány"), ("hy", "Զանջան"), ("id", "Provinsi Zanjan"), ("it", "regione di Zanjan"), ("ja", "ザンジャーン州"), ("ka", "ზენჯანი (ოსტანი)"), ("ko", "잔잔 주"), ("lt", "Zendžano provincija"), ("lv", "Zandžāna"), ("mk", "Занџан"), ("mr", "झ\u{902}जान प\u{94d}रा\u{902}त"), ("ms", "Zanjan"), ("nb", "Zanjan"), ("nl", "Zanjan"), ("no", "Zanjan"), ("pa", "ਜ\u{a3c}ਨਜਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Zandżan (ostan)"), ("ps", "زنجان ولايت"), ("pt", "Zanjan (província)"), ("ro", "Provincia Zanjan"), ("ru", "Зенджан"), ("sr", "Покрајина Занџан"), ("sr_Latn", "Pokrajina Zandžan"), ("sv", "Zanjan (provins)"), ("tr", "Zencan Eyaleti"), ("uk", "Зенджан (остан)"), ("ur", "صوبہ زنجان"), ("uz", "Zanjon ustoni"), ("vi", "Zanjan (tỉnh)"), ("zh", "贊詹省")]),
                        unofficial_name_list: ["Zanjan"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::IR,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.572778), longitude: Some(53.397222), max_latitude: Some(35.6169769), min_latitude: Some(35.5383726), max_longitude: Some(53.436985), min_longitude: Some(53.3312416)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سمنان"), ("az", "Simnan ostanı"), ("bg", "Семнан"), ("bn", "সেম\u{9cd}\u{200c}নন প\u{9cd}রদেশ"), ("ca", "Província de Semnan"), ("ccp", "𑄥𑄬𑄟\u{11134}𑄚𑄚\u{11134}"), ("ceb", "Semnān"), ("cs", "Semnán"), ("cy", "Semnān"), ("da", "Semnan"), ("de", "Semnan"), ("el", "Επαρχία Σεμνάν"), ("en", "Semnan"), ("es", "Semnán"), ("eu", "Semnan"), ("fa", "استان سمنان"), ("fi", "Semnān"), ("fr", "province de Semnan"), ("gl", "Provincia de Semnan"), ("he", "מחוז סמנאן"), ("hi", "स\u{947}मनान प\u{94d}रा\u{902}त"), ("hr", "Semnan"), ("hu", "Szemnán tartomány"), ("hy", "Սեմնան"), ("id", "Provinsi Semnān"), ("is", "Semnan-hérað, Íran"), ("it", "regione di Semnan"), ("ja", "セムナーン州"), ("ka", "სემნანის ოსტანი"), ("ko", "셈난 주"), ("lt", "Semnano provincija"), ("lv", "Semnāna"), ("mk", "Семнан"), ("mr", "स\u{947}मनान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Semnan"), ("nb", "Semnan"), ("nl", "Semnan"), ("no", "Semnan"), ("pl", "Semnan"), ("ps", "سمنان ولايت"), ("pt", "Semnan"), ("ru", "Семнан"), ("sr", "Покрајина Семнан"), ("sr_Latn", "Pokrajina Semnan"), ("sv", "Semnan"), ("tr", "Simnan Eyaleti"), ("uk", "Семнан"), ("ur", "صوبہ سمنان"), ("zh", "塞姆南省")]),
                        unofficial_name_list: ["Semnan"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::IR,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.8974232), longitude: Some(54.3568562), max_latitude: Some(31.9654157), min_latitude: Some(31.7937011), max_longitude: Some(54.4530487), min_longitude: Some(54.22456740000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "يزد"), ("az", "Yəzd ostanı"), ("be", "Езд"), ("bg", "Язд (остан)"), ("bn", "ইয\u{9bc}\u{9be}জ\u{9cd}\u{200c}দ\u{9cd}\u{200c} প\u{9cd}রদেশ"), ("ca", "Província de Yazd"), ("ccp", "𑄃\u{11128}𑄠𑄌\u{11134}𑄓\u{11134}"), ("ceb", "Yazd"), ("cs", "Jazd"), ("da", "Yazd"), ("de", "Yazd"), ("el", "Επαρχία Γιαζντ"), ("en", "Yazd"), ("es", "Yazd"), ("et", "Yazdi provints"), ("eu", "Yazd"), ("fa", "استان یزد"), ("fi", "Yazd"), ("fr", "province de Yazd"), ("he", "מחוז יזד"), ("hi", "यज\u{93c}\u{94d}द प\u{94d}रा\u{902}त"), ("hr", "Jazd"), ("hu", "Jazd tartomány"), ("id", "Provinsi Yazd"), ("it", "regione di Yazd"), ("ja", "ヤズド州"), ("ka", "იაზდი"), ("ko", "야즈드 주"), ("lt", "Jezdo provincija"), ("lv", "Jezda"), ("mk", "Јазд"), ("mr", "याज प\u{94d}रा\u{902}त"), ("ms", "Yazd"), ("nb", "Yazd"), ("nl", "Yazd"), ("no", "Yazd"), ("pa", "ਯਜ\u{a3c}ਦ ਸ\u{a42}ਬਾ"), ("pl", "Jazd"), ("ps", "د یزد ولایت"), ("pt", "Yazd"), ("ru", "Йезд"), ("sr", "Покрајина Јазд"), ("sr_Latn", "Pokrajina Jazd"), ("sv", "Yazd"), ("ta", "இய\u{bbe}ஸ\u{bcd}த\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("tr", "Yezd Eyaleti"), ("uk", "Йєзд"), ("ur", "یزد علاقہ"), ("vi", "Yazd"), ("zh", "亞茲德省")]),
                        unofficial_name_list: ["Yazd"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::IR,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.1907825), longitude: Some(56.2635988), max_latitude: Some(27.1963352), min_latitude: Some(27.1667586), max_longitude: Some(56.2897182), min_longitude: Some(56.2467733)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hormozgan"), ("ar", "هرمزغان"), ("az", "Hörmüzgan ostanı"), ("bg", "Хормозган"), ("bn", "হোর\u{9cd}মোজগন প\u{9cd}রদেশ"), ("ca", "Província d’Hormozgan"), ("ccp", "𑄦\u{1112e}𑄢𑄟\u{1112e}𑄌\u{11134}𑄉\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Hormozgān"), ("cs", "Hormozgán"), ("da", "Hormozgan"), ("de", "Hormozgan"), ("en", "Hormozgan"), ("es", "Hormozgan"), ("eu", "Hormozgan"), ("fa", "استان هرمزگان"), ("fi", "Hormozgān"), ("fr", "Hormozgan"), ("he", "הורמוזגאן"), ("hi", "होर\u{94d}मोज\u{93c}\u{94d}गान प\u{94d}रा\u{902}त"), ("hr", "Hormuzgan"), ("hu", "Hormozgán tartomány"), ("id", "Provinsi Hormozgān"), ("it", "regione di Hormozgan"), ("ja", "ホルモズガーン州"), ("ka", "ჰორმაზაგანი"), ("ko", "호르모즈간 주"), ("lt", "Hormozgano provincija"), ("lv", "Hormozgāna"), ("mk", "Хормозган"), ("mr", "होर\u{94d}मोज\u{94d}गान प\u{94d}रा\u{902}त"), ("ms", "Hormozgan"), ("nb", "Hormozgan"), ("nl", "Hormozgan"), ("no", "Hormozgan"), ("pl", "Hormozgan"), ("ps", "هرمزگان"), ("pt", "Hormozgan"), ("ru", "Хормозган"), ("sr", "Покрајина Хормозган"), ("sr_Latn", "Pokrajina Hormozgan"), ("sv", "Hormozgan"), ("sw", "Mkoa wa Hormozgan"), ("ta", "ஹொர\u{bcd}மொஸ\u{bcd}க\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "Hürmüzgan Eyaleti"), ("uk", "Хормозган"), ("ur", "صوبہ ہرمزگان"), ("vi", "Hormozgan"), ("yue", "霍爾木茲甘省"), ("yue_Hans", "霍尔木兹甘省"), ("zh", "霍爾木茲甘省")]),
                        unofficial_name_list: ["Hormozgan"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::IR,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7006336), longitude: Some(51.4716563), max_latitude: Some(35.7107648), min_latitude: Some(35.6910776), max_longitude: Some(51.4861434), min_longitude: Some(51.4592261)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "طهران"), ("az", "Tehran ostanı"), ("be", "астан Тэгеран"), ("bg", "Техеран"), ("bn", "তেহরন প\u{9cd}রদেশ"), ("bs", "Teheranska pokrajina"), ("ca", "Província de Teheran"), ("ccp", "𑄖𑄬𑄦\u{11134}𑄢𑄚\u{11134}"), ("ceb", "Tehrān"), ("cs", "Teherán"), ("cy", "Tehran"), ("da", "Teheran"), ("de", "Teheran (Provinz)"), ("el", "Τεχράν"), ("en", "Tehran"), ("es", "Teherán"), ("eu", "Teherango probintzia"), ("fa", "استان تهران"), ("fi", "Tehrān"), ("fr", "province de Téhéran"), ("gu", "ત\u{ac7}હરાન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז טהראן"), ("hi", "त\u{947}हरान प\u{94d}रा\u{902}त"), ("hr", "Teheran"), ("hu", "Teherán tartomány"), ("hy", "Թեհրան"), ("id", "Provinsi Tehran"), ("it", "regione di Teheran"), ("ja", "テヘラン州"), ("ka", "თეირანი"), ("kn", "ಟ\u{cc6}ಹ\u{ccd}ರಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "테헤란 주"), ("lt", "Teherano provincija"), ("lv", "Teherāna"), ("mk", "Техеран"), ("mr", "त\u{947}हरान प\u{94d}रा\u{902}त"), ("ms", "Teheran"), ("nb", "Teheran"), ("nl", "Teheran"), ("no", "Teheran"), ("pl", "Teheran"), ("ps", "تهران ولايت"), ("pt", "Teerão"), ("ro", "Provincia Tehran"), ("ru", "Тегеран"), ("si", "ටෙහ\u{dca}ර\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Техеран"), ("sr_Latn", "Pokrajina Teheran"), ("sv", "Teheran"), ("ta", "தெஹ\u{bcd}ர\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c46}హ\u{c4d}ర\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเตหะราน"), ("tr", "Tahran Eyaleti"), ("uk", "Тегеран"), ("ur", "صوبہ تہران"), ("vi", "Tehran"), ("zh", "德黑蘭省")]),
                        unofficial_name_list: ["Teheran"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::IR,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.25), longitude: Some(48.283333), max_latitude: Some(38.3550081), min_latitude: Some(38.17842570000001), max_longitude: Some(48.4719989), min_longitude: Some(48.2100678)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أردبيل"), ("az", "Ərdəbil ostanı"), ("be", "Ардэбіль"), ("bg", "Ардабил"), ("bn", "আর\u{9cd}দ\u{9be}বিল প\u{9cd}রদেশ"), ("ca", "Província d’Ardabil"), ("ccp", "𑄃𑄢\u{11134}𑄓𑄝\u{11128}𑄣\u{11134}"), ("ceb", "Ardabil"), ("cs", "Ardabíl"), ("da", "Ardabil"), ("de", "Ardabil"), ("el", "Αρνταμπίλ"), ("en", "Ardabil"), ("es", "Provincia de Ardebil"), ("et", "Ardabīli provints"), ("eu", "Ardabil"), ("fa", "استان اردبیل"), ("fi", "Ardabīl"), ("fr", "province d’Ardabil"), ("gu", "અર\u{acd}ડાબિલ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז ארדביל"), ("hi", "अर\u{94d}दबील प\u{94d}रा\u{902}त"), ("hr", "Ardabil"), ("hu", "Ardabil tartomány"), ("hy", "Արդաբիլ"), ("id", "Provinsi Ardabil"), ("it", "regione di Ardabil"), ("ja", "アルダビール州"), ("ka", "არდაბილი"), ("kk", "Ардебил"), ("kn", "ಆರ\u{ccd}ಡಬ\u{cbf}ಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아르다빌 주"), ("lt", "Ardebilio provincija"), ("lv", "Ardabīla"), ("mk", "Ардабил"), ("mn", "Ардебиль муж"), ("mr", "अर\u{94d}दाबिल प\u{94d}रा\u{902}त"), ("ms", "Ardabil"), ("nb", "Ardabil"), ("nl", "Ardebil"), ("no", "Ardabil"), ("pl", "Ardabil"), ("ps", "اردبيل ولايت"), ("pt", "Província de Ardabil"), ("ro", "Provincia Ardabil"), ("ru", "Ардебиль"), ("si", "අර\u{dca}ඩබ\u{dd2}ල\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Ардабил"), ("sr_Latn", "Pokrajina Ardabil"), ("sv", "Ardabil"), ("ta", "அர\u{bcd}ட\u{bbe}பில\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అర\u{c4d}డ\u{c3e}బ\u{c3f}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ดะบ\u{e35}ล"), ("tr", "Erdebil Eyaleti"), ("uk", "Ардебіль"), ("ur", "صوبہ اردبیل"), ("vi", "Ardabil"), ("yue", "阿爾達比勒省"), ("yue_Hans", "阿尔达比勒省"), ("zh", "阿爾達比勒省")]),
                        unofficial_name_list: ["Ardabil", "Ardebil"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::IR,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6399443), longitude: Some(50.8759419), max_latitude: Some(34.8156355), min_latitude: Some(34.5336217), max_longitude: Some(50.9421444), min_longitude: Some(50.7619858)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قم"), ("az", "Qum ostanı"), ("bg", "Кум"), ("bn", "কোম প\u{9cd}রদেশ"), ("ca", "Província de Qom"), ("ccp", "𑄇\u{1112e}𑄠\u{1112e}𑄟\u{11134}"), ("ceb", "Qom"), ("cs", "Qom"), ("cy", "Qom"), ("da", "Qom"), ("de", "Ghom"), ("el", "Κομ"), ("en", "Qom"), ("es", "Qom"), ("et", "Qomi provints"), ("eu", "Qom"), ("fa", "استان قم"), ("fi", "Qom"), ("fr", "province de Qom"), ("gu", "કોમ પ\u{acd}રા\u{a82}ત"), ("he", "קום"), ("hi", "क\u{93c}ोम प\u{94d}रा\u{902}त"), ("hr", "Kom"), ("hu", "Kom tartomány"), ("hy", "Ղոմի նահանգ"), ("id", "Provinsi Qom"), ("it", "regione di Qom"), ("ja", "ゴム州"), ("ka", "ყუმი"), ("kn", "ಕ\u{ccd}ವೋಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿰 주"), ("lt", "Kumo provincija"), ("lv", "Kuma"), ("mk", "Ком"), ("mr", "कोम प\u{94d}रा\u{902}त"), ("ms", "Qom"), ("nb", "Qom"), ("nl", "Qom"), ("no", "Qom"), ("pl", "Kom"), ("ps", "د قم ولايت"), ("pt", "Qom"), ("ro", "Provincia Qom"), ("ru", "Кум"), ("si", "කොම\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Ком"), ("sr_Latn", "Pokrajina Kom"), ("sv", "Qom"), ("ta", "கியோம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c4b}మ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e38}ม"), ("tr", "Kum Eyaleti"), ("uk", "Кум"), ("ur", "صوبہ قم"), ("vi", "Qom"), ("zh", "庫姆省")]),
                        unofficial_name_list: ["Qom"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::IR,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.1840731), longitude: Some(50.06737769999999), max_latitude: Some(36.1854853), min_latitude: Some(36.1829514), max_longitude: Some(50.06737769999999), min_longitude: Some(50.067299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قزوين"), ("az", "Qəzvin ostanı"), ("be", "Астан Казвін"), ("bg", "Казвин (остан)"), ("bn", "ক\u{9be}জভিন প\u{9cd}রদেশ"), ("ca", "Província de Qazvin"), ("ccp", "𑄇\u{1112a}𑄠𑄌\u{11134}𑄞\u{11128}𑄚\u{11134}"), ("ceb", "Qazvin"), ("cs", "Kazvín"), ("da", "Qazvin (provins)"), ("de", "Qazvin"), ("el", "Καζβίν"), ("en", "Qazvin"), ("es", "Qazvin"), ("eu", "Qazvin"), ("fa", "استان قزوین"), ("fi", "Qazvīn (provinssi)"), ("fr", "province de Qazvin"), ("gu", "કાઝવિન પ\u{acd}રા\u{a82}ત"), ("he", "קזווין"), ("hi", "क\u{93c}ज\u{93c}\u{94d}वीन प\u{94d}रा\u{902}त"), ("hr", "Kazvinska pokrajina"), ("hu", "Kazvin tartomány"), ("id", "Provinsi Qazvīn"), ("it", "regione di Qazvin"), ("ja", "ガズヴィーン州"), ("ka", "ყაზვინი (ოსტანი)"), ("kn", "ಖಜ\u{ccd}ವ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카즈빈 주"), ("lt", "Kazvino provincija"), ("lv", "Kazvīna"), ("mk", "Газвин"), ("mr", "काझविन प\u{94d}रा\u{902}त"), ("ms", "Qazvin"), ("nb", "Qazvin (provins)"), ("nl", "Qazvin"), ("no", "Qazvin (provins)"), ("pl", "Kazwin (ostan)"), ("ps", "قزوين ولايت"), ("pt", "Qazvin (província)"), ("ro", "Provincia Qazvīn"), ("ru", "Казвин"), ("si", "කස\u{dca}ව\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sv", "Qazvin (provins)"), ("ta", "கிங\u{bcd}ஸ\u{bcd}க\u{bcd}வின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c3e}జ\u{c4d}వ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องกาซว\u{e34}น"), ("tr", "Kazvin Eyaleti"), ("uk", "Казвін (остан)"), ("ur", "صوبہ قزوین"), ("vi", "Qazvin (tỉnh)"), ("zh", "加茲溫省")]),
                        unofficial_name_list: ["Qazvin"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::IR,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3137505), longitude: Some(47.0536816), max_latitude: Some(34.3188878), min_latitude: Some(34.3086925), max_longitude: Some(47.0581582), min_longitude: Some(47.0484325)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غلستان"), ("az", "Gülüstan ostanı"), ("be", "Галестан"), ("bg", "Голестан"), ("bn", "গোলেস\u{9cd}তন প\u{9cd}রদেশ"), ("ca", "Província de Golestan"), ("ccp", "𑄉\u{1112e}𑄣𑄬𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Golestān"), ("cs", "Golestán"), ("da", "Golestan"), ("de", "Golestan"), ("el", "Γκολεστάν"), ("en", "Golestan"), ("es", "Golestán"), ("et", "Golestān"), ("eu", "Golestan"), ("fa", "استان گلستان"), ("fi", "Golestanin provinssi"), ("fr", "Golestan"), ("gu", "ગ\u{ac1}લ\u{ac7}સ\u{acd}ટ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "גולסתאן"), ("hi", "गोल\u{947}स\u{94d}तान"), ("hr", "Golestan"), ("hu", "Golesztán tartomány"), ("hy", "Գոլեստան"), ("id", "Provinsi Golestān"), ("it", "regione di Golestan"), ("ja", "ゴレスターン州"), ("ka", "გულისტანი"), ("kn", "ಗೋಲ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "골레스탄 주"), ("lt", "Golestano provincija"), ("lv", "Golestāna"), ("mk", "Голестан"), ("mr", "गोल\u{947}स\u{94d}तान प\u{94d}रा\u{902}त"), ("ms", "Golestan"), ("nb", "Golestan"), ("nl", "Golestan"), ("no", "Golestan"), ("pl", "Golestan"), ("ps", "گلستان ولایت"), ("pt", "Golestan"), ("ru", "Голестан"), ("si", "ගොලේස\u{dca}ත\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Голестан"), ("sr_Latn", "Pokrajina Golestan"), ("sv", "Golestan"), ("ta", "கோலெஸ\u{bcd}டன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4b}ల\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกเลสถาน"), ("tk", "Gülistan Welaýaty"), ("tr", "Gülistan Eyaleti"), ("uk", "Голестан"), ("ur", "صوبہ گلستان"), ("uz", "Guliston ustoni"), ("vi", "Golestan"), ("yue", "戈勒斯坦省"), ("yue_Hans", "戈勒斯坦省"), ("zh", "戈勒斯坦省")]),
                        unofficial_name_list: ["Golestan"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::IR,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.4710353), longitude: Some(57.10131879999999), max_latitude: Some(38.2725939), min_latitude: Some(36.6166539), max_longitude: Some(58.387249), min_longitude: Some(55.805023)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خراسان الشمالية"), ("az", "Şimali Xorasan ostanı"), ("bg", "Северен Хорасан"), ("bn", "উত\u{9cd}তর খোরসন প\u{9cd}রদেশ"), ("ca", "Khorasan Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄈\u{1112e}𑄢\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Amihanang Khorāsān"), ("cs", "Severní Chorásán"), ("da", "Nord-Khorasan"), ("de", "Nord-Chorasan"), ("el", "Βόρειο Χορασάν"), ("en", "North Khorasan"), ("es", "Jorasán del Norte"), ("et", "Põhja-Khorāsān"), ("eu", "Ipar Khorasan"), ("fa", "استان خراسان شمالی"), ("fi", "Khorāsān-e Shomali"), ("fr", "Khorasan-e-shomali"), ("gu", "ઉત\u{acd}તર ખોરાસન પ\u{acd}રા\u{a82}ત"), ("he", "צפון ח׳וראסאן"), ("hi", "उत\u{94d}तर ख\u{93c}ोरासान प\u{94d}रा\u{902}त"), ("hr", "Sjeverni Horasan"), ("hu", "Észak-Horászán tartomány"), ("id", "Provinsi Khorasan Utara"), ("it", "regione del Khorasan settentrionale"), ("ja", "北ホラーサーン州"), ("ka", "ჩრდილოეთი ხორასანი"), ("kn", "ಉತ\u{ccd}ತರ ಖೊರಾಸಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "호라산에쇼말리 주"), ("lt", "Šiaurės Chorasano provincija"), ("lv", "Ziemeļhorasāna"), ("mk", "Северен Корасан"), ("mr", "उत\u{94d}तर खोरासान प\u{94d}रा\u{902}त"), ("ms", "Khorasan Utara"), ("nb", "Nord-Khorasan"), ("nl", "Noord-Khorasan"), ("no", "Nord-Khorasan"), ("pa", "ਉ\u{a71}ਤਰੀ ਖ\u{a3c}\u{a41}ਰਾਸਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Chorasan Północny"), ("ps", "خوراسان، شمالي"), ("pt", "Coração do Norte"), ("ru", "Северный Хорасан"), ("si", "උත\u{dd4}ර\u{dd4} කොරස\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Северни Корасан"), ("sr_Latn", "Pokrajina Severni Korasan"), ("sv", "Nordkhorasan"), ("ta", "வடக\u{bcd}கு க\u{bcd}ஹோரசன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర ఖ\u{c4b}రస\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคราซานเหน\u{e37}อ"), ("tr", "Kuzey Horasan Eyaleti"), ("uk", "Північний Хорасан"), ("ur", "صوبہ خراسان شمالی"), ("uz", "Shimoliy Xuroson ustoni"), ("vi", "Tỉnh Bắc Khorasan"), ("zh", "北呼羅珊省")]),
                        unofficial_name_list: ["Khorasan-e Shemali"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::IR,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.5175643), longitude: Some(59.1041758), max_latitude: Some(35.1648275), min_latitude: Some(30.389156), max_longitude: Some(61.144714), min_longitude: Some(55.306599)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خراسان جنوبي"), ("az", "Cənubi Xorasan ostanı"), ("be", "Астан Паўднёвы Харасан"), ("bg", "Южен Хорасан"), ("bn", "দক\u{9cd}ষিণ খোরসন প\u{9cd}রদেশ"), ("ca", "Khorasan Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄈\u{1112e}𑄢\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Habagatang Khorāsān"), ("cs", "Jižní Chorásán"), ("da", "Syd-Khorasan"), ("de", "Süd-Chorasan"), ("el", "Νότιο Χορασάν"), ("en", "South Khorasan"), ("es", "Jorasán del Sur"), ("et", "Lõuna-Khorāsān"), ("eu", "Hego Khorasan"), ("fa", "استان خراسان جنوبی"), ("fi", "Khorāsān-e Junoubi"), ("fr", "Khorasan-e-jonubi"), ("gu", "દક\u{acd}ષિણ ખોરાસન પ\u{acd}રા\u{a82}ત"), ("he", "דרום ח׳וראסאן"), ("hi", "दक\u{94d}षिण ख\u{93c}ोरासान प\u{94d}रा\u{902}त"), ("hr", "Južni Horasan"), ("hu", "Dél-Horászán tartomány"), ("id", "Provinsi Khorasan Selatan"), ("it", "regione del Khorasan meridionale"), ("ja", "南ホラーサーン州"), ("ka", "სამხრეთი ხორასანი"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಖೊರಾಸಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "호라산에조누비 주"), ("lt", "Pietų Chorasano provincija"), ("lv", "Dienvidhorasāna"), ("mk", "Јужен Корасан"), ("mr", "दक\u{94d}षिण खोरासान प\u{94d}रा\u{902}त"), ("ms", "Khorasan Selatan"), ("nb", "Sør-Khorasan"), ("nl", "Zuid-Khorasan"), ("no", "Sør-Khorasan"), ("pl", "Chorasan Południowy"), ("ps", "د سهېلي خراسان ولايت"), ("pt", "Coração do Sul"), ("ru", "Южный Хорасан"), ("si", "දක\u{dd4}ණ\u{dd4} කොරස\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sq", "Horasani Jugor"), ("sr", "Покрајина Јужни Корасан"), ("sr_Latn", "Pokrajina Južni Korasan"), ("sv", "Sydkhorasan"), ("ta", "தெற\u{bcd}கு கஹோரசன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ఖ\u{c4b}ర\u{c3e}స\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคราซานใต\u{e49}"), ("tr", "Güney Horasan Eyaleti"), ("uk", "Південний Хорасан"), ("ur", "صوبہ خراسان جنوبی"), ("uz", "Janubiy Xuroson ustoni"), ("vi", "Tỉnh Nam Khorasan"), ("zh", "南呼羅珊省")]),
                        unofficial_name_list: ["Khorasan-e Janubi"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::IR,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ألبرز"), ("az", "Əlburz ostanı"), ("bg", "Алборз"), ("bn", "আলব\u{9c1}র\u{9cd}জ প\u{9cd}রদেশ"), ("ca", "província d’Alborz"), ("ccp", "𑄃𑄣\u{11134}𑄝\u{1112e}𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Alborz (lalawigan)"), ("cs", "Alborz"), ("da", "Alborz Province"), ("de", "Alborz"), ("el", "Αλμπόρτζ"), ("en", "Alborz"), ("es", "Provincia de Elburz"), ("et", "Alborzi provints"), ("eu", "Alborz"), ("fa", "استان البرز"), ("fi", "Ālborz"), ("fr", "province d’Alborz"), ("gl", "Provincia de Alborz"), ("gu", "આલ\u{acd}બોર\u{acd}ઝ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אלבורז"), ("hi", "अल\u{94d}बोर\u{94d}ज\u{93c} प\u{94d}रा\u{902}त"), ("hr", "Alborz"), ("hu", "Alborz tartomány"), ("id", "Provinsi Alborz"), ("it", "regione di Alborz"), ("ja", "アルボルズ州"), ("ka", "ალბორზის ოსტანი"), ("kn", "ಅಲ\u{ccd}ಬೋರ\u{ccd}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알보르즈 주"), ("lt", "Alborzo provincija"), ("lv", "Alborzas ostāns"), ("mk", "Алборз"), ("mn", "Альборз муж"), ("mr", "आल\u{94d}बोर\u{94d}ज प\u{94d}रा\u{902}त"), ("ms", "Alborz Province"), ("nb", "Alborz Kommune"), ("nl", "Alborz"), ("no", "Alborz Kommune"), ("pa", "ਅਲਬ\u{a41}ਰਜ\u{a3c} ਸ\u{a42}ਬਾ"), ("pl", "Alborz"), ("ps", "البرز ولايت"), ("pt", "Alborz"), ("ru", "Альборз"), ("si", "අල\u{dca}බොරෝස\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Алборз"), ("sr_Latn", "Pokrajina Alborz"), ("sv", "Alborz"), ("ta", "அல\u{bcd}போன\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆల\u{c4d}బ\u{c4b}ర\u{c4d}జ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e31}ลโบร\u{e4c}ซ"), ("tr", "Elburz Eyaleti"), ("uk", "Альборз"), ("ur", "صوبہ البرز"), ("vi", "Tỉnh Alborz"), ("yue", "厄爾布爾士省"), ("yue_Hans", "厄尔布尔士省"), ("zh", "厄尔布尔士省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "ir")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::IR,
        alpha3: Alpha3::IRN,
        address_format: None,
        continent: Continent::Asia,
        country_code: 98,
        currency_code: "IRR",
        gec: Some(GEC::IR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("IRI"),
        iso_long_name: "The Islamic Republic of Iran",
        iso_short_name: "Iran (Islamic Republic of)",
        official_language_list: ["fa"].to_vec(),
        spoken_language_list: ["fa"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "0",
        nationality: Some("Iranian"),
        number: "364",
        postal_code: true,
        postal_code_format: Some("\\d{5}-?\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Saturday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "IR",
        unofficial_name_list: ["Iran", "Irán", "Iran (Islamic Republic Of)", "イラン・イスラム共和国", "Islamic Republic of Iran"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Iran, Islamic Republic of"), ("af", "Iran, Islamitiese Republiek van"), ("ak", "Iran, Islamic Republic of"), ("am", "Iran, Islamic Republic of"), ("an", "Iran, Islamic Republic of"), ("ar", "إيران، الجمهوري\u{651}ة الإسلامي\u{651}ة الإيراني\u{651}ة"), ("as", "ইৰ\u{9be}ন, ইচ\u{9cd}\u{200c}ল\u{9be}মিক প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"), ("ay", "Iran, Islamic Republic of"), ("az", "Iran, Islamic Republic of"), ("ba", "Iran, Islamic Republic of"), ("be", "Іран, Ісламская Рэспубліка"), ("bg", "Иран, Ислямска република"), ("bi", "Iran, Islamic Republic of"), ("bn", "ইর\u{9be}ন, ইসল\u{9be}মিক প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bn_IN", "ইর\u{9be}ন, ইসল\u{9be}মিক প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("br", "Iran, Islamic Republic of"), ("bs", "Islamska Republika Iran"), ("ca", "Iran"), ("ce", "Iran, Islamic Republic of"), ("ch", "Iran, Islamic Republic of"), ("cs", "Írán, islámská republika"), ("cv", "Iran, Islamic Republic of"), ("cy", "Iran, Gweriniaeth Islamaidd"), ("da", "Iran, Den Islamiske Republik"), ("de", "Iran, Islamische Republik"), ("dv", "Iran, Islamic Republic of"), ("dz", "ཨ\u{f72}་ར\u{f71}ན་ ཨ\u{f72}ས་ལ་མ\u{f72}ཀ་ མ\u{f72}་ས\u{f7a}ར་ར\u{f92}\u{fb1}ལ་ཁབ།"), ("ee", "Iran, Islamic Republic of"), ("el", "Ιράν, Ισλαμική Δημοκρατία του"), ("en", "Iran"), ("eo", "Irano, Islama Respubliko"), ("es", "Irán, República islámica de"), ("et", "Iraan"), ("eu", "Iran, Islamiar Errepublika"), ("fa", "ایران"), ("ff", "Iran, Islamic Republic of"), ("fi", "Iranin islamilainen tasavalta"), ("fo", "Iran, Islamic Republic of"), ("fr", "Iran, République islamique d'"), ("fy", "Iran, Islamic Republic of"), ("ga", "An Iaráin"), ("gl", "Irán, República Islámica de"), ("gn", "Iran, Islamic Republic of"), ("gu", "ઇરાન, ઇસ\u{acd}લામિક રીપબ\u{acd}લિક ઓફ"), ("gv", "Iran, Islamic Republic of"), ("ha", "Iran, Islamic Republic of"), ("he", "אירן, הרפובליקה האיסלמית של"), ("hi", "ईरान, इस\u{94d}लामिक रिपब\u{94d}लिक ऑफ"), ("hr", "Iran, Islamska Republika"), ("ht", "Iran, Islamic Republic of"), ("hu", "Irán, Iszlám Köztársaság"), ("hy", "Իրան, -ի Իսլամական Հանրապետություն"), ("ia", "Iran"), ("id", "Iran, Republik Islam"), ("io", "Iran, Islamic Republic of"), ("is", "Íran, íslamska lýðveldið"), ("it", "Iran"), ("iu", "Iran, Islamic Republic of"), ("ja", "イラン・イスラム共和国"), ("ka", "ირანის ისლამური რესპუბლიკა"), ("ki", "Iran, Islamic Republic of"), ("kk", "Иран"), ("kl", "Iran, Islamic Republic of"), ("km", "\u{200b}អ\u{17ca}\u{17b8}រ\u{17c9}ង\u{17cb} សាធារណ\u{200b}រដ\u{17d2}ឋ\u{200b}អ\u{17ca}\u{17b8}ស\u{17d2}លាម\u{200b}នៃ"), ("kn", "ಇರಾನ\u{ccd} ಇಸ\u{ccd}ಲಾಮ\u{cbf}ಕ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "이란 이슬람 공화국"), ("ku", "Îran, Komara Îslamî ya"), ("kv", "Iran, Islamic Republic of"), ("kw", "Iran, Islamic Republic of"), ("ky", "Иран"), ("lo", "Iran, Islamic Republic of"), ("lt", "Irano Islamo Respublika"), ("lv", "Irāna"), ("mi", "Iran, Islamic Republic of"), ("mk", "Иран"), ("ml", "ഇറ\u{d3e}ന\u{d4d}\u{200d}, ഇസ\u{d4d}ല\u{d3e}മിക\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"), ("mn", "Iran, Islamic Republic of"), ("mr", "इराण, इस\u{94d}लामिक रिपब\u{94d}लिक ऑफ"), ("ms", "Iran, Islamic Republic of"), ("mt", "Iran, Islamic Republic of"), ("my", "Iran, Islamic Republic of"), ("na", "Iran, Islamic Republic of"), ("nb", "Iran, Den islamske republikk"), ("ne", "इरान, इस\u{94d}लामिक गणराज\u{94d}य"), ("nl", "Iran"), ("nn", "Iran"), ("nv", "Iran, Islamic Republic of"), ("oc", "Republica Dominicana"), ("or", "ଇର\u{b3e}ନ, ଇସଲ\u{b3e}ମ\u{b3f}କ ଗଣତନ\u{b4d}ତ\u{b4d}ର"), ("pa", "ਈਰਾਨ, ਮ\u{a41}ਸਲਮਾਨ ਗਣਰਾਜ"), ("pi", "Iran, Islamic Republic of"), ("pl", "Iran"), ("ps", "Iran, Islamic Republic of"), ("pt", "Irão"), ("pt_BR", "Irã"), ("ro", "Iran, Republica islamică"), ("ru", "Иран"), ("rw", "Iran, Repubulika ya Kisilimu"), ("sc", "Iran, Repùblica Islàmica de"), ("sd", "Iran, Islamic Republic of"), ("si", "ඉර\u{dcf}න ඉස\u{dca}ල\u{dcf}ම\u{dd3}ය ජනරජය"), ("sk", "Iránska islamská republika"), ("sl", "Iran"), ("so", "Iiraan"), ("sq", "Iran, Republika Islamike"), ("sr", "Иран, Исламска Република"), ("sv", "Iran, islamiska republiken"), ("sw", "Iran, Islamic Republic of"), ("ta", "ஈர\u{bbe}ன\u{bcd}, இஸ\u{bcd}ல\u{bbe}மிக\u{bcd} குடியரசு"), ("te", "ఇర\u{c3e}న\u{c4d}, ఇస\u{c4d}ల\u{c3e}మ\u{c3f}క\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}"), ("tg", "Ҷумҳурии Исломии Эрон"), ("th", "อ\u{e34}หร\u{e48}าน, สาธารณร\u{e31}ฐอ\u{e34}สลาม"), ("ti", "Iran, Islamic Republic of"), ("tk", "Eýran, Yslam Respublikasy"), ("tl", "Iran, Republikang Islam ng"), ("tr", "İran İslâm Cumhuriyeti"), ("tt", "İран, İслам Җөмһүриәте"), ("ug", "ئىران ئىسلام جۇمھۇرىيىتى"), ("uk", "Іран"), ("ur", "Iran, Islamic Republic of"), ("uz", "Iran, Islamic Republic of"), ("ve", "Iran, Islamic Republic of"), ("vi", "Ba Tư, Cộng hoà Hồi giáo"), ("wa", "Iran"), ("wo", "Iraan, Republik Islaamik bu"), ("xh", "Iran, Islamic Republic of"), ("yo", "Iran, Islamic Republic of"), ("zh_CN", "伊朗"), ("zh_HK", "伊朗"), ("zh_TW", "伊朗伊斯蘭共和國"), ("zu", "Iran, Islamic Republic of")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}