// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Turkey

#[cfg(all(feature = "tr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::TR;
    pub const ALPHA3: Alpha3 = Alpha3::TUR;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 90;
    pub const CURRENCY_CODE: &str = "TRY";
    pub const GEC: Option<GEC> = Some(GEC::TU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("TUR");
    pub const ISO_SHORT_NAME: &str = "Turkey";
    pub const ISO_LONG_NAME: &str = "The Republic of Turkey";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["tr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["tr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Turkish");
    pub const NUMBER: &str = "792";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "TR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Turkey",
        "Türkei",
        "Turquie",
        "Turquía",
        "トルコ",
        "Turkije",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Turkey"),
        ("af", "Turkye"),
        ("ak", "Turkey"),
        ("am", "ቱሴጤ"),
        ("an", "Turkey"),
        ("ar", "تركي\u{651}ا"),
        ("as", "ত\u{9c1}ৰস\u{9cd}ক"),
        ("ay", "Turkey"),
        ("az", "Türkiyə"),
        ("ba", "Turkey"),
        ("be", "Турцыя"),
        ("bg", "Турция"),
        ("bi", "Turkey"),
        ("bn", "ত\u{9c1}র\u{9cd}কি"),
        ("bn_IN", "ত\u{9c1}র\u{9cd}কি"),
        ("br", "Turkia"),
        ("bs", "Turska"),
        ("ca", "Turquia"),
        ("ce", "Турци"),
        ("ch", "Turkey"),
        ("cs", "Turecko"),
        ("cv", "Турци"),
        ("cy", "Twrci"),
        ("da", "Tyrkiet"),
        ("de", "Türkei"),
        ("dv", "ތ\u{7aa}ރ\u{7aa}ކ\u{7a9}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}"),
        ("dz", "ཊར་ཀ\u{f72}།"),
        ("ee", "Turkey"),
        ("el", "Τουρκία"),
        ("en", "Turkey"),
        ("eo", "Turkio"),
        ("es", "Turquía"),
        ("et", "Türgi"),
        ("eu", "Turkia"),
        ("fa", "ترکیه"),
        ("ff", "Türkiye"),
        ("fi", "Turkki"),
        ("fo", "Turkaland"),
        ("fr", "Turquie"),
        ("fy", "Turkije"),
        ("ga", "An Tuirc"),
        ("gl", "Turquía"),
        ("gn", "Turkey"),
        ("gu", "ત\u{ac1}ર\u{acd}કી"),
        ("gv", "Yn Turkee"),
        ("ha", "Turkiyya"),
        ("he", "טורקיה"),
        ("hi", "त\u{941}र\u{94d}की"),
        ("hr", "Turska"),
        ("ht", "Tiki"),
        ("hu", "Törökország"),
        ("hy", "Թուրքիա"),
        ("ia", "Turchia"),
        ("id", "Turki"),
        ("io", "Turkia"),
        ("is", "Tyrkland"),
        ("it", "Turchia"),
        ("iu", "ᑑᕐᑭ"),
        ("ja", "トルコ"),
        ("ka", "თურქეთი"),
        ("ki", "Turkey"),
        ("kk", "Түркия"),
        ("kl", "Turkey"),
        ("km", "ទ\u{17bd}រគ\u{17b8}"),
        ("kn", "ಟರ\u{ccd}ಕ\u{cbf}"),
        ("ko", "터키"),
        ("ku", "Tirkiye"),
        ("kv", "Турция"),
        ("kw", "Turki"),
        ("ky", "Түркия Республикасы"),
        ("lo", "ປະເທດຕວກກ\u{eb5}"),
        ("lt", "Turkija"),
        ("lv", "Turcija"),
        ("mi", "Tākei"),
        ("mk", "Турција"),
        ("ml", "ത\u{d41}ര\u{d4d}\u{200d}ക\u{d4d}കി"),
        ("mn", "Турк"),
        ("mr", "त\u{941}र\u{94d}की"),
        ("ms", "Turki"),
        ("mt", "Turkija"),
        (
            "my",
            "တ\u{1030}ရက\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Terki"),
        ("nb", "Tyrkia"),
        ("ne", "टर\u{94d}की"),
        ("nl", "Turkije"),
        ("nn", "Tyrkia"),
        ("nv", "Tʼóok Bikéyah"),
        ("oc", "Turquia"),
        ("or", "ତ\u{b41}ର\u{b4d}କୀ"),
        ("pa", "ਤ\u{a41}ਰਕੀ"),
        ("pi", "त\u{941}र\u{94d}किय\u{947}"),
        ("pl", "Turcja"),
        ("ps", "تورکيه"),
        ("pt", "Turquia"),
        ("pt_BR", "Turquia"),
        ("ro", "Turcia"),
        ("ru", "Турция"),
        ("rw", "Turukiya"),
        ("sc", "Turchia"),
        ("sd", "Turkey"),
        ("si", "ත\u{dd4}ර\u{dca}ක\u{dd2}ය"),
        ("sk", "Turecko"),
        ("sl", "Turčija"),
        ("so", "Turki"),
        ("sq", "Turqi"),
        ("sr", "Турска"),
        ("sv", "Turkiet"),
        ("sw", "Turkey"),
        ("ta", "துருக\u{bcd}கி"),
        ("te", "టర\u{c4d}క\u{c40}"),
        ("tg", "Туркия"),
        ("th", "ต\u{e38}รก\u{e35}"),
        ("ti", "ቱርኪ"),
        ("tk", "Türk"),
        ("tl", "Turkey"),
        ("tr", "Türkiye"),
        ("tt", "Төркиә"),
        ("ug", "تۈركىيە"),
        ("uk", "Туреччина"),
        ("ur", "ترکی"),
        ("uz", "Turkiya"),
        ("ve", "Turkey"),
        ("vi", "Thổ Nhĩ Kỳ"),
        ("wa", "Turkeye"),
        ("wo", "Turki"),
        ("xh", "Turkey"),
        ("yo", "Túrkì"),
        ("zh_CN", "土耳其"),
        ("zh_HK", "土耳其"),
        ("zh_TW", "土耳其"),
        ("zu", "ITheki"),
    ];
    #[cfg(all(feature = "tr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 38.963745;
        pub const LONGITUDE: f64 = 35.243322;
        pub const MAX_LATITUDE: f64 = 42.3666999;
        pub const MAX_LONGITUDE: f64 = 44.8178449;
        pub const MIN_LATITUDE: f64 = 35.808592;
        pub const MIN_LONGITUDE: f64 = 25.5377;
        pub const NORTHEAST_LATITUDE: f64 = 42.3666999;
        pub const NORTHEAST_LONGITUDE: f64 = 44.8178449;
        pub const SOUTHWEST_LATITUDE: f64 = 35.808592;
        pub const SOUTHWEST_LONGITUDE: f64 = 25.5377;
    }
}
#[cfg(all(feature = "tr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 38.963745,
            longitude: 35.243322,
            max_latitude: 42.3666999,
            max_longitude: 44.8178449,
            min_latitude: 35.808592,
            min_longitude: 25.5377,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 42.3666999,
                    longitude: 44.8178449,
                },
                southwest: CountryGeoBound {
                    latitude: 35.808592,
                    longitude: 25.5377,
                },
            },
        }
    }
}

#[cfg(all(feature = "tr", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::TR,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.0), longitude: Some(35.321333), max_latitude: Some(37.065693), min_latitude: Some(36.929137), max_longitude: Some(35.4720578), min_longitude: Some(35.1040759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أضنة"), ("az", "Adana vilayəti"), ("be", "Правінцыя Адана"), ("bg", "Адана"), ("bn", "আদ\u{9be}ন\u{9be} প\u{9cd}রদেশ"), ("bs", "Adana"), ("ca", "Província d’Adana"), ("ccp", "𑄃𑄓𑄚"), ("ceb", "Adana (lalawigan)"), ("cs", "Adana"), ("cy", "Adana"), ("da", "Adana Province"), ("de", "Adana"), ("el", "Επαρχία Αδάνων"), ("en", "Adana"), ("es", "Provincia de Adana"), ("et", "Adana provints"), ("eu", "Adana probintzia"), ("fa", "استان آدانا"), ("fi", "Adanan maakunta"), ("fr", "Adana"), ("gl", "Provincia de Adana"), ("gu", "અદાના પ\u{acd}રા\u{a82}ત"), ("he", "אדנה"), ("hi", "अडाना प\u{94d}रा\u{902}त"), ("hr", "Adana"), ("hu", "Adana"), ("hy", "Ադանա"), ("id", "Provinsi Adana"), ("it", "provincia di Adana"), ("ja", "アダナ県"), ("jv", "Provinsi Adana"), ("ka", "ადანის პროვინცია"), ("kk", "Адана"), ("kn", "ಅದಾನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아다나 주"), ("lt", "Adanos provincija"), ("lv", "Adanas ils"), ("mk", "Адана"), ("mr", "अदना प\u{94d}रा\u{902}त"), ("ms", "Wilayah Adana"), ("nb", "Adana"), ("nl", "Adana"), ("no", "Adana"), ("pa", "ਅਦਾਨਾ"), ("pl", "Adana"), ("pt", "Adana"), ("ro", "Provincia Adana"), ("ru", "Адана"), ("si", "අදන\u{dcf} පළ\u{dcf}ත"), ("sl", "Adana"), ("sr", "Адана"), ("sr_Latn", "Adana"), ("sv", "Adana"), ("sw", "Mkoa wa Adana"), ("ta", "அதன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అడ\u{c3e}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาดานา"), ("tk", "Adana"), ("tr", "Adana"), ("uk", "Адана"), ("ur", "آدانا صوبہ"), ("uz", "Adana"), ("vi", "Adana"), ("yue", "阿達納省"), ("yue_Hans", "阿达纳省"), ("zh", "阿达纳省")]),
                        unofficial_name_list: ["Seyhan"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::TR,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.7647509), longitude: Some(38.278561), max_latitude: Some(37.793365), min_latitude: Some(37.741438), max_longitude: Some(38.320623), min_longitude: Some(38.2312051)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أديامان"), ("az", "Adıyaman vilayəti"), ("be", "Правінцыя Адыяман"), ("bg", "Адъяман"), ("bn", "আদিয\u{9bc}\u{9be}ম\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Adıyaman"), ("ca", "Província d’Adıyaman"), ("ccp", "𑄃\u{11128}𑄓\u{11128}𑄠𑄟\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Adıyaman (lalawigan)"), ("cs", "Adıyamanská provincie"), ("cy", "Adıyaman"), ("de", "Adıyaman"), ("el", "Επαρχία Αντιγιαμάν"), ("en", "Adıyaman"), ("es", "Provincia de Adıyaman"), ("et", "Adıyamani provints"), ("eu", "Adiyaman probintzia"), ("fa", "استان آدیامان"), ("fi", "Adıyamanin maakunta"), ("fr", "Adıyaman"), ("gl", "Provincia de Adıyaman"), ("he", "אדיאמן"), ("hi", "आदियामान प\u{94d}रा\u{902}त"), ("hu", "Adıyaman"), ("hy", "Ադըյաման"), ("id", "Provinsi Adıyaman"), ("it", "provincia di Adıyaman"), ("ja", "アドゥヤマン県"), ("ka", "ადიამანის პროვინცია"), ("kk", "Адыяман"), ("ko", "아디야만 주"), ("lv", "Adijamanas ils"), ("mk", "Адјаман"), ("mr", "आद\u{94d}यामान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Adıyaman"), ("nb", "Adıyaman"), ("nl", "Adıyaman"), ("no", "Adıyaman"), ("pa", "ਅਦਿਅਮਾਨ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Adıyaman"), ("pt", "Adıyaman"), ("ro", "Provincia Adıyaman"), ("ru", "Адыяман"), ("sl", "Adıyaman"), ("sq", "Adëjamani"), ("sr", "Адијаман"), ("sr_Latn", "Adijaman"), ("sv", "Adıyaman"), ("sw", "Mkoa wa Adıyaman"), ("th", "จ\u{e31}งหว\u{e31}ดอาด\u{e36}ยาม\u{e31}น"), ("tk", "Adyýaman"), ("tr", "Adıyaman"), ("uk", "Адияман"), ("ur", "آدیامان صوبہ"), ("uz", "Adıyaman"), ("vi", "Adıyaman"), ("yue", "阿德亞曼省"), ("yue_Hans", "阿德亚曼省"), ("zh", "阿德亚曼省")]),
                        unofficial_name_list: ["Adıyaman"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::TR,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.754167), longitude: Some(30.54027799999999), max_latitude: Some(38.796503), min_latitude: Some(38.716577), max_longitude: Some(30.5979509), min_longitude: Some(30.5121179)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أفيون"), ("az", "Afyonkarahisar vilayəti"), ("bg", "Афионкарахисар"), ("bn", "আফিয\u{9bc}নক\u{9be}র\u{9be}হিস\u{9be}র প\u{9cd}রদেশ"), ("bs", "Afyonkarahisar"), ("ca", "Província d’Afyonkarahisar"), ("ccp", "𑄃𑄜\u{11128}𑄠\u{11127}𑄚\u{11134}𑄖𑄢𑄝𑄦\u{11128}𑄥𑄢\u{11134}"), ("ceb", "Afyonkarahisar"), ("cs", "Provincie Afyonkarahisar"), ("cy", "Afyonkarahisar"), ("da", "Afyonkarahisar Province"), ("de", "Afyonkarahisar"), ("el", "Επαρχία Αφιόν Καραχισάρ"), ("en", "Afyonkarahisar"), ("es", "Provincia de Afyonkarahisar"), ("et", "Afyonkarahisari provints"), ("eu", "Afyonkarahisar probintzia"), ("fa", "استان افیون قره\u{200c}حصار"), ("fi", "Afyonkarahisarin maakunta"), ("fr", "Afyonkarahisar"), ("gl", "Provincia de Afyonkarahisar"), ("gu", "અફ\u{acd}યોનકારાહિસાર પ\u{acd}રા\u{a82}ત"), ("hi", "एफ\u{94d}यो\u{902}कराहिसर प\u{94d}रा\u{902}त"), ("hu", "Afyonkarahisar"), ("hy", "Աֆիոնկարահիսարի նահանգ"), ("id", "Provinsi Afyonkarahisar"), ("it", "provincia di Afyonkarahisar"), ("ja", "アフィヨンカラヒサール県"), ("ka", "აფიონ-ყარაჰისარის პროვინცია"), ("kk", "Афьонкарахисар"), ("kn", "ಅಫ\u{cbf}ಯಾನ\u{ccd}ಕಾರ\u{ccd}ಹ\u{cbf}ಸಾರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아피온카라히사르 주"), ("lt", "Afjonkarachisaro provincija"), ("lv", "Afjonas ils"), ("mk", "Афјонкарахисар"), ("mr", "आफ\u{94d}योनकाराहिसार प\u{94d}रा\u{902}त"), ("ms", "Wilayah Afyonkarahisar"), ("nb", "Afyonkarahisar"), ("nl", "Afyonkarahisar"), ("no", "Afyonkarahisar"), ("pa", "ਅਫ\u{a3c}ਿਓਨਕਾਰਾਹਿਸਾਰ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Afyonkarahisar"), ("pt", "Afyonkarahisar"), ("ro", "Provincia Afyonkarahisar"), ("ru", "ил Афьонкарахисар"), ("si", "අෆ\u{dca}යොන\u{dca}කරහ\u{dd2}ස\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sl", "Afyonkarahisar"), ("sr", "Афјонкарахисар"), ("sr_Latn", "Afjonkarahisar"), ("sv", "Afyonkarahisar"), ("sw", "Mkoa wa Afyonkarahisar"), ("ta", "அபியோன\u{bcd}கர\u{bcd}ஹிஷர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆఫ\u{c4d}య\u{c4b}ంక\u{c3e}ర\u{c3e}హ\u{c3f}సర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ฟยอนคาราฮ\u{e34}ซาร\u{e4c}"), ("tk", "Afýonkarahisar"), ("tr", "Afyonkarahisar"), ("uk", "Афьонкарахісар"), ("ur", "افیون قرہ حصار صوبہ"), ("uz", "Afyonkarahisar"), ("vi", "Afyonkarahisar"), ("yue", "阿菲永卡拉希薩爾省"), ("yue_Hans", "阿菲永卡拉希萨尔省"), ("zh", "阿菲永卡拉希萨尔省")]),
                        unofficial_name_list: ["Afyon"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::TR,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7188318), longitude: Some(43.0484269), max_latitude: Some(39.751162), min_latitude: Some(39.692784), max_longitude: Some(43.0799288), min_longitude: Some(42.9903562)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أغري"), ("az", "Ağrı vilayəti"), ("be", "Правінцыя Агры"), ("bg", "Агръ"), ("bn", "আরি প\u{9cd}রদেশ"), ("bs", "Ağrı"), ("ca", "Província d’Ağrı"), ("ccp", "𑄃𑄇\u{11134}𑄢\u{11128}"), ("ceb", "Ağrı"), ("cs", "Ağrıská provincie"), ("cy", "Ağrı"), ("da", "Ağrı Province"), ("de", "Ağrı"), ("el", "Άγκρι"), ("en", "Ağrı"), ("es", "Provincia de Ağrı"), ("et", "Ağrı provints"), ("eu", "Agri probintzia"), ("fa", "استان آغری"), ("fi", "Ağrın maakunta"), ("fr", "Ağrı"), ("gl", "Provincia de Ağrı"), ("gu", "આગ\u{acd}રી પ\u{acd}રા\u{a82}ત"), ("hi", "एग\u{94d}री प\u{94d}रा\u{902}त"), ("hu", "Ağrı"), ("hy", "Աղրի"), ("id", "Provinsi Ağrı"), ("it", "provincia di Ağrı"), ("ja", "アール県"), ("ka", "აღრის პროვინცია"), ("kk", "Ағры"), ("kn", "ಆಗ\u{ccd}ರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아리 주"), ("lt", "Agrio provincija"), ("lv", "Āgri ils"), ("mk", "Агри"), ("mr", "आर प\u{94d}रा\u{902}त"), ("ms", "Provinsi Ağrı"), ("nb", "Ağrı"), ("nl", "Ağrı"), ("no", "Ağrı"), ("pa", "ਆਗਰੀ ਸ\u{a42}ਬਾ"), ("pl", "Ağrı"), ("pt", "Ağrı"), ("ro", "Provincia Ağrı"), ("ru", "Агры"), ("si", "අග\u{dca}ර\u{dd2} පළ\u{dcf}ත"), ("sr", "Вилајет Агри"), ("sr_Latn", "Vilajet Agri"), ("sv", "Ağrı"), ("sw", "Mkoa wa Ağrı"), ("ta", "அக\u{bcd}ரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అగ\u{c4d}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e36}"), ("tk", "Agry"), ("tr", "Ağrı"), ("uk", "Агри"), ("ur", "آغری صوبہ"), ("uz", "Ağrı"), ("vi", "Ağrı"), ("yue", "阿勒省"), ("yue_Hans", "阿勒省"), ("zh", "阿勒省")]),
                        unofficial_name_list: ["Ağrı"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::TR,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.64991), longitude: Some(35.83532), max_latitude: Some(40.672106), min_latitude: Some(40.64035), max_longitude: Some(35.8556602), min_longitude: Some(35.788518)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أماصيا"), ("az", "Amasya vilayəti"), ("be", "Правінцыя Амасья"), ("bg", "Амасия"), ("bn", "আম\u{9be}সিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Amasya"), ("ca", "Província d’Amasya"), ("ccp", "𑄃𑄟𑄥\u{11128}𑄠"), ("ceb", "Amasya (lalawigan)"), ("cs", "Amasyjská provincie"), ("cy", "Amasya"), ("da", "Amasya Province"), ("de", "Amasya"), ("el", "Επαρχία Αμάσειας"), ("en", "Amasya"), ("es", "Provincia de Amasya"), ("et", "Amasya provints"), ("eu", "Amasya probintzia"), ("fa", "استان آماسیه"), ("fi", "Amasyan maakunta"), ("fr", "Amasya"), ("gl", "Provincia de Amasya"), ("gu", "અમાસ\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "अमासिया प\u{94d}रा\u{902}त"), ("hu", "Amasya"), ("hy", "Ամասիայի նահանգ"), ("id", "Provinsi Amasya"), ("it", "provincia di Amasya"), ("ja", "アマスィヤ県"), ("ka", "ამასიის პროვინცია"), ("kk", "Амасья"), ("kn", "ಅಮಸ\u{ccd}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아마시아 주"), ("lt", "Amasjos provincija"), ("lv", "Amasjas ils"), ("mk", "Амасија"), ("mr", "अमास\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Amasya"), ("nb", "Amasya"), ("nl", "Amasya"), ("no", "Amasya"), ("pa", "ਅਮਾਸਿਆ ਸ\u{a42}ਬਾ"), ("pl", "Amasya"), ("pt", "Amasya"), ("ro", "Provincia Amasya"), ("ru", "Амасья"), ("si", "අමස\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Амасија"), ("sr_Latn", "Amasija"), ("sv", "Amasya"), ("sw", "Mkoa wa Amasya"), ("ta", "அமஸ\u{bcd}ய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అమ\u{c3e}స\u{c4d}య ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอะม\u{e31}สยา"), ("tk", "Amasýa"), ("tr", "Amasya"), ("uk", "Амасья"), ("ur", "اماسیا صوبہ"), ("uz", "Amasya"), ("vi", "Amasya"), ("yue", "阿馬西亞省"), ("yue_Hans", "阿马西亚省"), ("zh", "阿马西亚省")]),
                        unofficial_name_list: ["Amasya"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::TR,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.92077), longitude: Some(32.85411), max_latitude: Some(40.076332), min_latitude: Some(39.730421), max_longitude: Some(33.0070561), min_longitude: Some(32.5286899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أنقرة"), ("az", "Ankara ili"), ("be", "Правінцыя Анкара"), ("bg", "Анкара"), ("bn", "আঙ\u{9cd}ক\u{9be}র\u{9be} প\u{9cd}রদেশ"), ("bs", "Ankara"), ("ca", "Província d’Ankara"), ("ccp", "𑄃\u{11101}𑄇𑄢"), ("ceb", "Ankara"), ("cs", "Ankarská provincie"), ("cy", "Ankara"), ("de", "Ankara"), ("el", "Επαρχία Άγκυρας"), ("en", "Ankara"), ("es", "Ankara"), ("et", "Ankara provints"), ("eu", "Ankara probintzia"), ("fa", "استان آنکارا"), ("fi", "Ankaran maakunta"), ("fr", "Ankara"), ("he", "אנקרה"), ("hr", "Ankara"), ("hu", "Ankara"), ("hy", "Անկարա"), ("id", "Provinsi Ankara"), ("it", "provincia di Ankara"), ("ja", "アンカラ県"), ("ka", "ანკარის პროვინცია"), ("ko", "앙카라 주"), ("lv", "Ankaras ils"), ("mk", "Анкара"), ("mr", "अ\u{902}कारा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ankara"), ("nb", "Ankara"), ("nl", "Ankara"), ("no", "Ankara"), ("pl", "Ankara"), ("pt", "Ancara"), ("ro", "Provincia Ankara"), ("ru", "Анкара"), ("sd", "انقره صوبو"), ("sl", "Ankara"), ("sr", "Анкарски вилајет"), ("sr_Latn", "Ankarski vilajet"), ("sv", "Ankara"), ("sw", "Mkoa wa Ankara"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}งการา"), ("tk", "Ankara"), ("tr", "Ankara"), ("uk", "Анкара"), ("ur", "انقرہ صوبہ"), ("uz", "Ankara"), ("vi", "Ankara"), ("yue", "安卡拉省"), ("yue_Hans", "安卡拉省"), ("zh", "安卡拉省")]),
                        unofficial_name_list: ["Ankara"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::TR,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.88414), longitude: Some(30.70563), max_latitude: Some(36.975586), min_latitude: Some(36.820289), max_longitude: Some(30.8552512), min_longitude: Some(30.5809589)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أنطاليا"), ("az", "Antalya vilayəti"), ("be", "Правінцыя Анталья"), ("bg", "Анталия"), ("bn", "আন\u{9cd}ত\u{9be}লিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Antalya"), ("ca", "Província d’Antalya"), ("ccp", "𑄃𑄚\u{11134}𑄑𑄣\u{11128}𑄠"), ("ceb", "Antalya (lalawigan)"), ("cs", "Antalijská provincie"), ("cy", "Antalya"), ("da", "Antalya"), ("de", "Antalya"), ("el", "Επαρχία Αττάλειας"), ("en", "Antalya"), ("es", "Provincia de Antalya"), ("et", "Antalya provints"), ("eu", "Antalya probintzia"), ("fa", "استان آنتالیا"), ("fi", "Antalyan maakunta"), ("fr", "Antalya"), ("gl", "Provincia de Antalya"), ("gu", "અ\u{a82}તાલ\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "अ\u{902}ताल\u{94d}या प\u{94d}रा\u{902}त"), ("hr", "Antalya"), ("hu", "Antalya"), ("hy", "Անթալիա"), ("id", "Provinsi Antalya"), ("it", "provincia di Adalia"), ("ja", "アンタルヤ県"), ("ka", "ანთალიის პროვინცია"), ("kk", "Анталья"), ("kn", "ಆಂತಲ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "안탈리아 주"), ("lt", "Antalijos provincija"), ("lv", "Antaljas ils"), ("mk", "Анталија"), ("mr", "अ\u{902}ताल\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Antalya"), ("nb", "Antalya"), ("nl", "Antalya"), ("no", "Antalya"), ("pa", "ਅ\u{a70}ਤਾਲਿਆ ਸ\u{a42}ਬਾ"), ("pl", "Antalya"), ("pt", "Antália (província)"), ("ro", "Provincia Antalya"), ("ru", "Анталья"), ("si", "අන\u{dca}ටල\u{dca}\u{200d}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Antalya"), ("sq", "Antalya"), ("sr", "Анталија"), ("sr_Latn", "Antalija"), ("sv", "Antalya"), ("sw", "Mkoa wa Antalya"), ("ta", "அண\u{bcd}டலிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అంట\u{c3e}ల\u{c4d}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}นต\u{e31}ลยา"), ("tk", "Antalýa"), ("tr", "Antalya"), ("uk", "Анталія"), ("ur", "انطالیہ صوبہ"), ("uz", "Antalya"), ("vi", "Antalya"), ("yue", "安塔利亞省"), ("yue_Hans", "安塔利亚省"), ("zh", "安塔利亚省")]),
                        unofficial_name_list: ["Antalya"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::TR,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.18277), longitude: Some(41.81829099999999), max_latitude: Some(41.191183), min_latitude: Some(41.168135), max_longitude: Some(41.8366858), min_longitude: Some(41.79922200000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أرتفين"), ("az", "Artvin vilayəti"), ("be", "Правінцыя Артвін"), ("bg", "Артвин"), ("bn", "আর\u{9cd}টভিন প\u{9cd}রদেশ"), ("bs", "Artvin"), ("ca", "Província d’Artvin"), ("ccp", "𑄃𑄢\u{11134}𑄑\u{11134}𑄞\u{11128}𑄚\u{11134}"), ("ceb", "Artvin"), ("cs", "Artvinská provincie"), ("cy", "Artvin"), ("da", "Artvin Province"), ("de", "Artvin"), ("el", "Επαρχία Αρτβίν"), ("en", "Artvin"), ("es", "Provincia de Artvin"), ("et", "Artvini provints"), ("eu", "Artvin probintzia"), ("fa", "استان آرتوین"), ("fi", "Artvinin maakunta"), ("fr", "Artvin"), ("gl", "Provincia de Artvin"), ("gu", "આર\u{acd}ટવિન પ\u{acd}રા\u{a82}ત"), ("he", "ארטווין"), ("hi", "आर\u{94d}टविन प\u{94d}रा\u{902}त"), ("hu", "Artvin"), ("hy", "Արդվինի նահանգ"), ("id", "Provinsi Artvin"), ("it", "provincia di Artvin"), ("ja", "アルトヴィン県"), ("ka", "ართვინის პროვინცია"), ("kk", "Артвин"), ("kn", "ಆರ\u{ccd}ಟ\u{ccd}ವ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아르트빈 주"), ("lt", "Artvino provincija"), ("lv", "Artvinas ils"), ("mk", "Артвин"), ("mr", "आर\u{94d}त\u{94d}विन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Artvin"), ("nb", "Artvin"), ("nl", "Artvin"), ("no", "Artvin"), ("pa", "ਅਰਤਵਿਨ"), ("pl", "Artvin"), ("pt", "Artvin"), ("ro", "Provincia Artvin"), ("ru", "Артвин"), ("si", "අර\u{dca}ට\u{dca}ව\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sq", "Artvin"), ("sr", "Артвин"), ("sr_Latn", "Artvin"), ("sv", "Artvin"), ("sw", "Mkoa wa Artvin"), ("ta", "அற\u{bcd}டவின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆర\u{c4d}ట\u{c4d}వ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ตว\u{e34}น"), ("tk", "Artwin"), ("tr", "Artvin ili"), ("uk", "Артвін"), ("ur", "آرتوین صوبہ"), ("uz", "Artvin"), ("vi", "Artvin"), ("yue", "阿爾特溫省"), ("yue_Hans", "阿尔特温省"), ("zh", "阿尔特温省")]),
                        unofficial_name_list: ["Artvin"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::TR,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.84805600000001), longitude: Some(27.845278), max_latitude: Some(37.867569), min_latitude: Some(37.803987), max_longitude: Some(27.9005062), min_longitude: Some(27.794327)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aydın"), ("ar", "أيدين"), ("az", "Aydın vilayəti"), ("be", "Правінцыя Айдын"), ("bg", "Айдън"), ("bn", "আইডিন প\u{9cd}রদেশ"), ("bs", "Aydın"), ("ca", "Província d’Aydın"), ("ccp", "𑄃\u{11128}𑄠𑄬𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Aydın (lalawigan)"), ("cs", "Aydınská provincie"), ("cy", "Aydın"), ("da", "Aydın"), ("de", "Aydın"), ("el", "Επαρχία Αϊδινίου"), ("en", "Aydın"), ("es", "Provincia de Aydin"), ("et", "Aydıni provints"), ("eu", "Aydin probintzia"), ("fa", "استان آیدین"), ("fi", "Aydınin maakunta"), ("fr", "Aydın"), ("ga", "Aydın"), ("gl", "Aydın"), ("gu", "એડીન પ\u{acd}રા\u{a82}ત"), ("hi", "आयदीन प\u{94d}रा\u{902}त"), ("hr", "Aydın"), ("hu", "Aydın"), ("hy", "Այդընի նահանգ"), ("id", "Provinsi Aydın"), ("is", "Aydın"), ("it", "provincia di Aydın"), ("ja", "アイドゥン県"), ("ka", "აიდინის პროვინცია"), ("kn", "ಆಯ\u{ccd}ದ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아이딘 주"), ("lt", "Aydın"), ("lv", "Ajdinas ils"), ("mk", "Ајдин"), ("mr", "आय\u{94d}दन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Aydın"), ("nb", "Aydın"), ("nl", "Aydın"), ("no", "Aydın"), ("pl", "Aydın"), ("pt", "Aydın"), ("ro", "Provincia Aydın"), ("ru", "Айдын"), ("si", "අය\u{dd2}ඩ\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sk", "Aydın"), ("sl", "Aydın"), ("sq", "Ajdëni"), ("sr", "Ајдин"), ("sr_Latn", "Ajdin"), ("sv", "Aydın"), ("sw", "Mkoa wa Aydın"), ("ta", "ஐடின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఐడ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไอด\u{e35}น"), ("tk", "Aýdyn"), ("tr", "Aydın"), ("uk", "Айдин"), ("ur", "آیدین صوبہ"), ("uz", "Aydın"), ("vi", "Aydın"), ("yue", "艾登省"), ("yue_Hans", "艾登省"), ("zh", "艾登省")]),
                        unofficial_name_list: ["Aydın"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::TR,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.648369), longitude: Some(27.88261), max_latitude: Some(39.683234), min_latitude: Some(39.612411), max_longitude: Some(27.9411919), min_longitude: Some(27.8383129)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باليكسير"), ("az", "Balıkəsir vilayəti"), ("be", "Правінцыя Балыкесір"), ("bg", "Балъкесир"), ("bn", "ব\u{9be}লিকেসির প\u{9cd}রদেশ"), ("bs", "Balıkesir"), ("ca", "Província de Balıkesir"), ("ccp", "𑄝𑄣\u{11128}𑄇𑄬𑄥\u{11128}𑄢\u{11134}"), ("ceb", "Balıkesir"), ("cs", "Balıkesirská provincie"), ("cy", "Balıkesir"), ("da", "Balıkesir Province"), ("de", "Balıkesir"), ("el", "Επαρχία Μπαλικεσίρ"), ("en", "Balıkesir"), ("es", "Provincia de Balıkesir"), ("et", "Balıkesiri provints"), ("eu", "Balikesir probintzia"), ("fa", "استان بالیکسیر"), ("fi", "Balıkesirin maakunta"), ("fr", "Balıkesir"), ("gl", "Provincia de Balıkesir"), ("gu", "બાલિક\u{ac7}સિર પ\u{acd}રા\u{a82}ત"), ("hi", "बालिक\u{947}सिर प\u{94d}रा\u{902}त"), ("hu", "Balıkesir"), ("hy", "Բալըքեսիր"), ("id", "Provinsi Balıkesir"), ("it", "provincia di Balıkesir"), ("ja", "バルケスィル県"), ("ka", "ბალიქესირის პროვინცია"), ("kk", "Балыкесир"), ("kn", "ಬಾಲ\u{cbf}ಕ\u{cc6}ಸರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "발리케시르 주"), ("lt", "Balikesiro provincija"), ("lv", "Balikesiras ils"), ("mk", "Балкесир"), ("mr", "बाल\u{94d}क\u{947}सिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Balıkesir"), ("nb", "Balıkesir"), ("nl", "Balıkesir"), ("no", "Balıkesir"), ("pa", "ਬਾਲਿਕ\u{a47}ਸਿਰ"), ("pl", "Balıkesir"), ("pt", "Balıkesir"), ("ro", "Provincia Balıkesir"), ("ru", "Балыкесир"), ("si", "බල\u{dd2}කෙස\u{dd3}ර\u{dca} පළ\u{dcf}ත"), ("sl", "Balıkesir"), ("sq", "Ballëkesiri"), ("sr", "Баликесир"), ("sr_Latn", "Balikesir"), ("sv", "Balıkesir"), ("sw", "Mkoa wa Balıkesir"), ("ta", "ப\u{bbe}லிகேஸிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ల\u{c3f}క\u{c47}స\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บาล\u{e35}เคซ\u{e35}ร\u{e4c}"), ("tk", "Balykesir"), ("tr", "Balıkesir"), ("uk", "Баликесір"), ("ur", "بالیکسیر صوبہ"), ("uz", "Balıkesir"), ("vi", "Balıkesir"), ("yue", "巴勒克埃西爾省"), ("yue_Hans", "巴勒克埃西尔省"), ("zh", "巴勒克埃西尔省")]),
                        unofficial_name_list: ["Balıkesir"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::TR,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.15013099999999), longitude: Some(29.983061), max_latitude: Some(40.2001), min_latitude: Some(40.12762), max_longitude: Some(29.991397), min_longitude: Some(29.957953)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيلجيك"), ("az", "Biləcik vilayəti"), ("be", "Правінцыя Біледжык"), ("bg", "Биледжик"), ("bn", "বিলেচিক প\u{9cd}রদেশ"), ("bs", "Bilecik"), ("ca", "Província de Bilecik"), ("ccp", "𑄝\u{11128}𑄣𑄬𑄥\u{11128}𑄇\u{11134}"), ("ceb", "Bilecik"), ("cs", "Bilecikská provincie"), ("da", "Bilecik Province"), ("de", "Bilecik"), ("el", "Επαρχία Μπιλετσίκ"), ("en", "Bilecik"), ("es", "Provincia de Bilecik"), ("et", "Bileciki provints"), ("eu", "Bilecik probintzia"), ("fa", "استان بیله\u{200c}جک"), ("fi", "Bilecikin maakunta"), ("fr", "Bilecik"), ("gl", "Provincia de Bilecik"), ("gu", "બિલ\u{ac7}સિક પ\u{acd}રા\u{a82}ત"), ("he", "בילג׳יק"), ("hi", "बिल\u{947}सिक प\u{94d}रा\u{902}त"), ("hu", "Bilecik"), ("hy", "Բիլեջիկ"), ("id", "Provinsi Bilecik"), ("it", "provincia di Bilecik"), ("ja", "ビレジク県"), ("ka", "ბილეჯიქის პროვინცია"), ("kk", "Билежик"), ("kn", "ಬ\u{cbf}ಲ\u{cc6}ಸ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "빌레지크 주"), ("lt", "Biledžiko provincija"), ("lv", "Biledžikas ils"), ("mk", "Билеџик"), ("mr", "बिल\u{947}चिक प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bilecik"), ("nb", "Bilecik"), ("nl", "Bilecik"), ("no", "Bilecik"), ("pl", "Bilecik"), ("pt", "Bilecik"), ("ro", "Provincia Bilecik"), ("ru", "Биледжик"), ("si", "බ\u{dd2}ලෙස\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sl", "Bilecik"), ("sq", "Bilecik"), ("sr", "Билеџик"), ("sr_Latn", "Biledžik"), ("sv", "Bilecik"), ("sw", "Mkoa wa Bilecik"), ("ta", "பிளேஸிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3f}ల\u{c46}స\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}เลช\u{e34}ก"), ("tk", "Bilejik"), ("tr", "Bilecik"), ("uk", "Біледжик"), ("ur", "بیلیجک صوبہ"), ("uz", "Bilecik"), ("vi", "Bilecik"), ("yue", "比萊吉克省"), ("yue_Hans", "比莱吉克省"), ("zh", "比莱吉克省")]),
                        unofficial_name_list: ["Bilecik"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::TR,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.885349), longitude: Some(40.49829099999999), max_latitude: Some(38.89169), min_latitude: Some(38.874678), max_longitude: Some(40.5216061), min_longitude: Some(40.4800891)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بينغول"), ("az", "Bingöl vilayəti"), ("be", "Правінцыя Бінгёль"), ("bg", "Бингьол"), ("bn", "বিঙ\u{9cd}গল প\u{9cd}রদেশ"), ("bs", "Bingöl"), ("ca", "Província de Bingöl"), ("ccp", "𑄝\u{11128}𑄚\u{11134}𑄉\u{1112e}𑄣\u{11134}"), ("ceb", "Bingöl"), ("cs", "Bingölská provincie"), ("cy", "Bingöl"), ("da", "Bingol"), ("de", "Bingöl"), ("el", "Μπινγκόλ"), ("en", "Bingöl"), ("es", "Provincia de Bingöl"), ("et", "Bingöli provints"), ("eu", "Bingol probintzia"), ("fa", "استان بینگول"), ("fi", "Bingölin maakunta"), ("fr", "Bingöl"), ("gl", "Provincia de Bingöl"), ("gu", "બિ\u{a82}ગોલ પ\u{acd}રા\u{a82}ત"), ("hi", "बि\u{902}गोल प\u{94d}रा\u{902}त"), ("hu", "Bingöl"), ("hy", "Բինգյոլ"), ("id", "Provinsi Bingöl"), ("it", "provincia di Bingöl"), ("ja", "ビンギョル県"), ("ka", "ბინგოლის პროვინცია"), ("kk", "Бингөл"), ("kn", "ಬ\u{cbf}ಂಗೊಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "빙괼 주"), ("lt", "Bingelo provincija"), ("lv", "Bingelas ils"), ("mk", "Бингол"), ("mr", "बि\u{902}गोल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bingöl"), ("nb", "Bingöl"), ("nl", "Bingöl"), ("no", "Bingöl"), ("pl", "Bingöl"), ("pt", "Bingöl"), ("ro", "Provincia Bingöl"), ("ru", "Бингёль"), ("si", "බ\u{dd2}න\u{dca}ගොල\u{dca} පළ\u{dcf}ත"), ("sr", "Бингол"), ("sr_Latn", "Bingol"), ("sv", "Bingöl"), ("sw", "Mkoa wa Bingöl"), ("ta", "பிங\u{bcd}கோள\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3f}ంగ\u{c4b}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}นกอล"), ("tk", "Bingöl"), ("tr", "Bingöl"), ("uk", "Бінґель"), ("ur", "بینگول صوبہ"), ("uz", "Bingöl"), ("vi", "Bingöl"), ("yue", "賓格爾省"), ("yue_Hans", "宾格尔省"), ("zh", "宾格尔省")]),
                        unofficial_name_list: ["Bingöl"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::TR,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.393799), longitude: Some(42.12318), max_latitude: Some(38.429029), min_latitude: Some(38.37997), max_longitude: Some(42.1383121), min_longitude: Some(42.0875269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بدليس"), ("az", "Bitlis vilayəti"), ("be", "Правінцыя Бітліс"), ("bg", "Битлис"), ("bn", "বিতলিস প\u{9cd}রদেশ"), ("bs", "Bitlis"), ("ca", "Província de Bitlis"), ("ccp", "𑄝\u{11128}𑄣\u{11134}𑄑\u{11128}𑄌\u{11134}"), ("ceb", "Bitlis"), ("cs", "Bitliská provincie"), ("da", "Bitlis Province"), ("de", "Bitlis"), ("el", "Μπίτλις"), ("en", "Bitlis"), ("es", "Provincia de Bitlis"), ("et", "Bitlisi provints"), ("eu", "Bitlis probintzia"), ("fa", "استان بتلیس"), ("fi", "Bitlisin maakunta"), ("fr", "Bitlis"), ("gu", "બિટલીસ પ\u{acd}રા\u{a82}ત"), ("he", "ביטליס"), ("hi", "बिटलिस प\u{94d}रा\u{902}त"), ("hu", "Bitlis"), ("hy", "Բիթլիս"), ("id", "Provinsi Bitlis"), ("it", "provincia di Bitlis"), ("ja", "ビトリス県"), ("ka", "ბითლისის პროვინცია"), ("kk", "Битлис"), ("kn", "ಬ\u{cbf}ಟ\u{ccd}ಲ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비틀리스 주"), ("lt", "Bitliso provincija"), ("lv", "Bitlisas ils"), ("mk", "Битлис"), ("mr", "बित\u{94d}लिस प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bitlis"), ("nb", "Bitlis"), ("nl", "Bitlis"), ("no", "Bitlis"), ("pl", "Bitlis"), ("pt", "Bitlis"), ("ro", "Provincia Bitlis"), ("ru", "Битлис"), ("si", "බ\u{dd2}ට\u{dca}ල\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Битлис"), ("sr_Latn", "Bitlis"), ("sv", "Bitlis"), ("sw", "Mkoa wa Bitlis"), ("ta", "பிட\u{bcd}டலிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3f}ట\u{c4d}ల\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}ตล\u{e34}ส"), ("tk", "Bitlis"), ("tr", "Bitlis"), ("uk", "Бітліс"), ("ur", "بتلیس صوبہ"), ("uz", "Bitlis"), ("vi", "Bitlis"), ("yue", "比特利斯省"), ("yue_Hans", "比特利斯省"), ("zh", "比特利斯省")]),
                        unofficial_name_list: ["Bitlis"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::TR,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.739479), longitude: Some(31.611561), max_latitude: Some(40.765095), min_latitude: Some(40.706339), max_longitude: Some(31.658418), min_longitude: Some(31.564409)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بولو"), ("az", "Bolu vilayəti"), ("be", "Правінцыя Балу"), ("bg", "Болу"), ("bn", "বল\u{9c1} প\u{9cd}রদেশ"), ("bs", "Bolu"), ("ca", "Província de Bolu"), ("ccp", "𑄝\u{1112e}𑄣\u{1112a}"), ("ceb", "Bolu (lalawigan)"), ("cs", "Bolská provincie"), ("cy", "Bolu"), ("da", "Bolu Province"), ("de", "Bolu"), ("el", "Επαρχία Μπολού"), ("en", "Bolu"), ("es", "Provincia de Bolu"), ("et", "Bolu provints"), ("eu", "Bolu probintzia"), ("fa", "استان بولی"), ("fi", "Bolun maakunta"), ("fr", "Bolu"), ("gl", "Provincia de Bolu"), ("gu", "બોલ\u{ac1} પ\u{acd}રા\u{a82}ત"), ("hi", "बोल\u{942} प\u{94d}रा\u{902}त"), ("hu", "Bolu"), ("hy", "Բոլուի նահանգ"), ("id", "Provinsi Bolu"), ("it", "provincia di Bolu"), ("ja", "ボル県"), ("ka", "ბოლუს პროვინცია"), ("kk", "Болу"), ("kn", "ಬೋಲು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "볼루 주"), ("lt", "Bolo provincija"), ("lv", "Bolu ils"), ("mk", "Болу"), ("mr", "बोल\u{942} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bolu"), ("nb", "Bolu"), ("nl", "Bolu"), ("no", "Bolu"), ("pl", "Bolu"), ("pt", "Bolu"), ("ro", "Provincia Bolu"), ("ru", "Болу"), ("si", "බොල\u{dd4} පළ\u{dcf}ත"), ("sq", "Bolu"), ("sr", "Болу"), ("sr_Latn", "Bolu"), ("sv", "Bolu"), ("sw", "Mkoa wa Bolu"), ("ta", "ப\u{bbe}லு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4b}లు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโบล\u{e39}"), ("tk", "Bolu"), ("tr", "Bolu"), ("uk", "Болу"), ("ur", "بولو صوبہ"), ("uz", "Bolu"), ("vi", "Bolu"), ("yue", "博盧省"), ("yue_Hans", "博卢省"), ("zh", "博卢省")]),
                        unofficial_name_list: ["Bolu"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::TR,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.726909), longitude: Some(30.288876), max_latitude: Some(37.760559), min_latitude: Some(37.704594), max_longitude: Some(30.3362129), min_longitude: Some(30.2286262)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوردور"), ("az", "Burdur vilayəti"), ("be", "Правінцыя Бурдур"), ("bg", "Бурдур"), ("bn", "ব\u{9c1}রদ\u{9c2}র প\u{9cd}রদেশ"), ("bs", "Burdur"), ("ca", "Província de Burdur"), ("ccp", "𑄝\u{1112a}𑄢\u{11134}𑄓\u{1112a}𑄢\u{11134}"), ("ceb", "Burdur"), ("cs", "Burdurská provincie"), ("da", "Burdur Province"), ("de", "Burdur"), ("el", "Επαρχία Μπουρντούρ"), ("en", "Burdur"), ("es", "Provincia de Burdur"), ("et", "Burduri provints"), ("eu", "Burdur probintzia"), ("fa", "استان بوردور"), ("fi", "Burdurin maakunta"), ("fr", "Burdur"), ("gu", "બ\u{ac1}ર\u{acd}ડ\u{ac1}ર પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{941}र\u{94d}द\u{942}र प\u{94d}रा\u{902}त"), ("hu", "Burdur"), ("hy", "Բուրդուր"), ("id", "Provinsi Burdur"), ("it", "provincia di Burdur"), ("ja", "ブルドゥル県"), ("ka", "ბურდურის პროვინცია"), ("kk", "Бурдур"), ("kn", "ಬ\u{cc2}ರ\u{ccd}ಡು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부르두르 주"), ("lt", "Burduro provincija"), ("lv", "Burduras ils"), ("mk", "Бурдур"), ("mr", "ब\u{941}र\u{94d}द\u{941}र प\u{94d}रा\u{902}त"), ("ms", "Wilayah Burdur"), ("nb", "Burdur"), ("ne", "ब\u{941}र\u{94d}ड\u{941}र\u{94d} क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Burdur"), ("no", "Burdur"), ("pl", "Burdur"), ("pt", "Burdur"), ("ro", "Provincia Burdur"), ("ru", "Бурдур (ил)"), ("si", "බර\u{dca}ඩර\u{dca} පළ\u{dcf}ත"), ("sq", "Burdur"), ("sr", "Бурдур"), ("sr_Latn", "Burdur"), ("sv", "Burdur"), ("sw", "Mkoa wa Burdur"), ("ta", "பர\u{bcd}துர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుర\u{c4d}డుర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e39}ร\u{e4c}ด\u{e39}ร\u{e4c}"), ("tk", "Burdur"), ("tr", "Burdur"), ("uk", "Бурдур"), ("ur", "بوردور صوبہ"), ("uz", "Burdur"), ("vi", "Burdur"), ("yue", "布林杜爾省"), ("yue_Hans", "布林杜尔省"), ("zh", "布尔杜尔省")]),
                        unofficial_name_list: ["Burdur"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::TR,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.266864), longitude: Some(29.0634479), max_latitude: Some(40.318955), min_latitude: Some(40.254332), max_longitude: Some(29.114446), min_longitude: Some(29.0490682)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بورصة"), ("az", "Bursa vilayəti"), ("be", "Правінцыя Бурса"), ("bg", "Бурса"), ("bn", "ব\u{9c1}রস\u{9be} প\u{9cd}রদেশ"), ("bs", "Bursa"), ("ca", "Província de Bursa"), ("ccp", "𑄝\u{1112a}𑄢\u{11134}𑄥"), ("ceb", "Bursa (lalawigan)"), ("cs", "Burská provincie"), ("cy", "Bursa"), ("da", "Bursa Province"), ("de", "Bursa"), ("el", "Επαρχία Προύσας"), ("en", "Bursa"), ("es", "Provincia de Bursa"), ("et", "Bursa provints"), ("eu", "Bursa probintzia"), ("fa", "استان بورسا"), ("fi", "Bursan maakunta"), ("fr", "Bursa"), ("gl", "Provincia de Bursa"), ("gu", "બ\u{ac3}સા પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{941}र\u{94d}सा प\u{94d}रा\u{902}त"), ("hr", "Bursa"), ("hu", "Bursa"), ("hy", "Բուրսա"), ("id", "Provinsi Bursa"), ("it", "provincia di Bursa"), ("ja", "ブルサ県"), ("ka", "ბურსის პროვინცია"), ("kn", "ಬುರ\u{ccd}ಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부르사 주"), ("lt", "Bursos provincija"), ("lv", "Bursas ils"), ("mk", "Бурса"), ("mr", "ब\u{941}र\u{94d}सा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bursa"), ("nb", "Bursa"), ("nl", "Bursa"), ("no", "Bursa"), ("pl", "Bursa"), ("pt", "Bursa"), ("ro", "Provincia Bursa"), ("ru", "Бурса"), ("si", "බර\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Bursa"), ("sq", "Brusa"), ("sr", "Бурса"), ("sr_Latn", "Bursa"), ("sv", "Bursa"), ("sw", "Mkoa wa Bursa"), ("ta", "பர\u{bcd}ஸ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుర\u{c4d}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e39}ร\u{e4c}ซา"), ("tk", "Bursa"), ("tr", "Bursa"), ("uk", "Бурса"), ("ur", "بورصہ صوبہ"), ("uz", "Bursa"), ("vi", "Bursa"), ("yue", "布爾薩省"), ("yue_Hans", "布尔萨省"), ("zh", "布尔萨省")]),
                        unofficial_name_list: ["Bursa"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::TR,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.147778), longitude: Some(26.405556), max_latitude: Some(40.17119599999999), min_latitude: Some(40.089921), max_longitude: Some(26.468447), min_longitude: Some(26.3857388)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة جاناكالي"), ("az", "Çanaqqala vilayəti"), ("be", "Правінцыя Чанакале"), ("bg", "Чанаккале"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9be}ক\u{9cd}ক\u{9be}লে প\u{9cd}রদেশ"), ("bs", "Çanakkale"), ("ca", "Província de Çanakkale"), ("ccp", "𑄇𑄬𑄚𑄇\u{11134}𑄇𑄣𑄬"), ("ceb", "Çanakkale"), ("cs", "Çanakkale (provincie)"), ("cy", "Çanakkale"), ("da", "Çanakkale Province"), ("de", "Çanakkale"), ("el", "Επαρχία Τσανάκκαλε"), ("en", "Çanakkale"), ("es", "Provincia de Çanakkale"), ("et", "Çanakkale provints"), ("eu", "Çanakkale probintzia"), ("fa", "استان چاناق\u{200c}قلعه"), ("fi", "Çanakkalen maakunta"), ("fr", "Çanakkale"), ("gl", "Provincia de Çanakkale"), ("gu", "કનકાલ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "कनक\u{94d}क\u{947}ल प\u{94d}रा\u{902}त"), ("hu", "Çanakkale"), ("hy", "Չանաքկալե"), ("id", "Provinsi Çanakkale"), ("it", "provincia di Çanakkale"), ("ja", "チャナッカレ県"), ("jv", "Provinsi Çanakkale"), ("ka", "ჩანაქკალეს პროვინცია"), ("kk", "Чанаккале"), ("kn", "ಕ\u{ccd}ಯಾನಕ\u{ccd}ಕಲ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "차나칼레 주"), ("lt", "Čanakalės provincija"), ("lv", "Čanakales ils"), ("mk", "Чанаккале"), ("mr", "चनाक\u{94d}काल\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Çanakkale"), ("nb", "Çanakkale"), ("nl", "Çanakkale"), ("no", "Çanakkale"), ("pl", "Çanakkale"), ("pt", "Çanakkale"), ("ro", "Provincia Çanakkale"), ("ru", "Чанаккале"), ("si", "කනක\u{dca}කලේ පළ\u{dcf}ත"), ("sl", "Çanakkale"), ("sr", "Чанакале"), ("sr_Latn", "Čanakale"), ("sv", "Çanakkale"), ("sw", "Mkoa wa Çanakkale"), ("ta", "கணுக\u{bcd}களே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c46}నక\u{c4d}క\u{c47}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ด คานากเคล"), ("tk", "Çanakkale"), ("tr", "Çanakkale"), ("uk", "Чанаккале"), ("ur", "چناق قلعہ صوبہ"), ("uz", "Çanakkale"), ("vi", "Çanakkale"), ("yue", "恰納卡萊省"), ("yue_Hans", "恰纳卡莱省"), ("zh", "恰纳卡莱省")]),
                        unofficial_name_list: ["Çanakkale"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::TR,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6013428), longitude: Some(33.6134213), max_latitude: Some(40.647551), min_latitude: Some(40.567176), max_longitude: Some(33.635831), min_longitude: Some(33.5933449)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جانقري"), ("az", "Çankırı ili"), ("be", "Правінцыя Чанкыры"), ("bg", "Чанкъръ (вилает)"), ("bn", "ক\u{9cd}লেয\u{9be}ন ক\u{9cd}য\u{9be}নকিরি প\u{9cd}রদেশ"), ("bs", "Çankiri"), ("ca", "Província de Çankırı"), ("ccp", "𑄇𑄚\u{11134}𑄇\u{11128}𑄢\u{11128}"), ("ceb", "Çankırı"), ("cs", "Çankırská provincie"), ("da", "Çankırı Province"), ("de", "Çankırı (Provinz)"), ("el", "Επαρχία Τσανκιρί"), ("en", "Çankırı"), ("es", "Provincia de Çankırı"), ("et", "Çankırı provints"), ("eu", "Çankırı probintzia"), ("fa", "استان چانقری"), ("fi", "Çankırın maakunta"), ("fr", "Çankırı (province)"), ("gu", "ક\u{ac7}ન\u{acd}કીરી પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{948}\u{902}कीरी प\u{94d}रा\u{902}त"), ("hu", "Çankırı (tartomány)"), ("id", "Provinsi Çankırı"), ("it", "provincia di Çankırı"), ("ja", "チャンクル県"), ("ka", "ჩანქირის პროვინცია"), ("kk", "Чанкыры"), ("kn", "ಚಾಂಕ\u{cbf}ರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "창키리 주"), ("lt", "Čankirio provincija"), ("lv", "Čankiri ils"), ("mk", "Чанкр (покраина)"), ("mr", "चा\u{902}कर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Çankırı"), ("nb", "Çankırı"), ("nl", "Çankırı"), ("no", "Çankırı"), ("pl", "Çankırı (prowincja)"), ("pt", "Çankırı (província)"), ("ro", "Provincia Çankırı"), ("ru", "Чанкыры (ил)"), ("si", "කන\u{dca}ක\u{dd2}ර\u{dd2} පළ\u{dcf}ත"), ("sr", "Чанкири (вилајет)"), ("sr_Latn", "Čankiri (vilajet)"), ("sv", "Çankırı (provins)"), ("sw", "Mkoa wa Çankırı"), ("ta", "க\u{bbe}ன\u{bcd}கிரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంక\u{c3f}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแคนค\u{e34}ร\u{e35}\u{e48}"), ("tk", "Çankyry (il)"), ("tr", "Çankırı (il)"), ("uk", "Чанкири (іл)"), ("ur", "چانقری صوبہ"), ("uz", "Çankırı (viloyat)"), ("vi", "Çankırı (tỉnh)"), ("yue", "昌克勒省"), ("yue_Hans", "昌克勒省"), ("zh", "昌克勒省")]),
                        unofficial_name_list: ["Çankırı"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::TR,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.554371), longitude: Some(34.963718), max_latitude: Some(40.583577), min_latitude: Some(40.486869), max_longitude: Some(34.9962619), min_longitude: Some(34.887546)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شوروم"), ("az", "Çorum ili"), ("be", "Правінцыя Чарум"), ("bg", "Чорум (вилает)"), ("bs", "Çorum"), ("ca", "Província de Çorum"), ("ccp", "𑄇𑄢\u{1112a}𑄟\u{11134}"), ("ceb", "Çorum"), ("cs", "Çorumská provincie"), ("cy", "Çorum"), ("de", "Çorum"), ("el", "Επαρχία Τσορούμ"), ("en", "Çorum"), ("es", "Provincia de Çorum"), ("et", "Çorumi provints"), ("eu", "Çorum probintzia"), ("fa", "استان چوروم"), ("fi", "Çorumin maakunta"), ("fr", "Çorum (province)"), ("hi", "कॉरम प\u{94d}रा\u{902}त"), ("hu", "Çorum (tartomány)"), ("hy", "Չորումի նահանգ"), ("id", "Provinsi Çorum"), ("it", "provincia di Çorum"), ("ja", "チョルム県"), ("ka", "ჩორუმის პროვინცია"), ("kk", "Чорум"), ("ko", "초룸 주"), ("lv", "Čorumas ils"), ("mk", "Чорум (покраина)"), ("mr", "चोर\u{941}म प\u{94d}रा\u{902}त"), ("ms", "Wilayah Çorum"), ("nb", "Çorum"), ("nl", "Çorum"), ("no", "Çorum"), ("pl", "Çorum (prowincja)"), ("pt", "Çorum (província)"), ("ro", "Provincia Çorum"), ("ru", "Чорум (ил)"), ("sr", "Чорум"), ("sr_Latn", "Čorum"), ("sv", "Çorum (provins)"), ("sw", "Mkoa wa Çorum"), ("th", "จ\u{e31}งหว\u{e31}ดโชร\u{e38}ม"), ("tk", "Çorum (il)"), ("tr", "Çorum (il)"), ("uk", "Чорум (іл)"), ("ur", "چوروم صوبہ"), ("uz", "Çorum (viloyat)"), ("vi", "Çorum (tỉnh)"), ("yue", "喬魯姆省"), ("yue_Hans", "乔鲁姆省"), ("zh", "乔鲁姆省")]),
                        unofficial_name_list: ["Çorum"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::TR,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.77652), longitude: Some(29.08639), max_latitude: Some(37.843464), min_latitude: Some(37.722339), max_longitude: Some(29.1701542), min_longitude: Some(28.9931389)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دينيزلي"), ("az", "Dənizli vilayəti"), ("be", "Правінцыя Дэнізлі"), ("bg", "Денизли"), ("bn", "দ\u{9be}নিজি প\u{9cd}রদেশ"), ("bs", "Denizli"), ("ca", "Província de Denizli"), ("ccp", "𑄓𑄬𑄚\u{11128}𑄌\u{11134}𑄣\u{11128}"), ("ceb", "Denizli"), ("cs", "Denizliská provincie"), ("da", "Denizli Province"), ("de", "Denizli"), ("el", "Επαρχία Ντενιζλί"), ("en", "Denizli"), ("es", "Provincia de Denizli"), ("et", "Denizli provints"), ("eu", "Denizli probintzia"), ("fa", "استان دنیزلی"), ("fi", "Denizlin maakunta"), ("fr", "Denizli"), ("gu", "ડ\u{ac7}નિઝ\u{acd}લી પ\u{acd}રા\u{a82}ત"), ("hi", "द\u{947}निज\u{94d}ली प\u{94d}रा\u{902}त"), ("hu", "Denizli"), ("hy", "Դենիզլիի նահանգ"), ("id", "Provinsi Denizli"), ("it", "provincia di Denizli"), ("ja", "デニズリ県"), ("ka", "დენიზლის პროვინცია"), ("kk", "Денизли"), ("kn", "ಡ\u{cc6}ನ\u{cbf}ಜ\u{ccd}ಲ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "데니즐리 주"), ("lt", "Denizlio provincija"), ("lv", "Denizli ils"), ("mk", "Денизли"), ("mr", "द\u{947}निझ\u{94d}ली प\u{94d}रा\u{902}त"), ("ms", "Wilayah Denizli"), ("nb", "Denizli"), ("nl", "Denizli"), ("no", "Denizli"), ("pl", "Denizli"), ("pt", "Denizli"), ("ro", "Provincia Denizli"), ("ru", "Денизли"), ("si", "ඩෙන\u{dd2}ස\u{dca}ල\u{dd2} පළ\u{dcf}ත"), ("sl", "Denizli"), ("sr", "Денизли"), ("sr_Latn", "Denizli"), ("sv", "Denizli"), ("sw", "Mkoa wa Denizli"), ("ta", "டெனிஸ\u{bcd}லி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c46}న\u{c3f}జ\u{c3f}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเดน\u{e34}ซล\u{e35}"), ("tk", "Denizli"), ("tr", "Denizli"), ("uk", "Денізлі"), ("ur", "دنیزلی صوبہ"), ("uz", "Denizli"), ("vi", "Denizli"), ("yue", "代尼茲利省"), ("yue_Hans", "代尼兹利省"), ("zh", "代尼兹利省")]),
                        unofficial_name_list: ["Denizli"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::TR,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9144099), longitude: Some(40.230629), max_latitude: Some(37.979755), min_latitude: Some(37.86583), max_longitude: Some(40.24692), min_longitude: Some(40.078436)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دياربكر"), ("az", "Diyarbəkir vilayəti"), ("be", "Правінцыя Дыярбакыр"), ("bg", "Диарбекир"), ("bn", "দিয\u{9bc}\u{9be}রব\u{9be}কির প\u{9cd}রদেশ"), ("bs", "Diyarbakır"), ("ca", "Província de Diyarbakır"), ("ccp", "𑄓\u{11128}𑄠𑄢\u{11134}𑄝𑄇\u{11128}𑄢\u{11134}"), ("ceb", "Diyarbakır"), ("cs", "Diyarbakırská provincie"), ("cy", "Diyarbakır"), ("da", "Diyarbakır Province"), ("de", "Diyarbakır"), ("el", "Επαρχία Ντιγιαρμπακίρ"), ("en", "Diyarbakır"), ("es", "Provincia de Diyarbakır"), ("et", "Diyarbakıri provints"), ("eu", "Diyarbakir probintzia"), ("fa", "استان دیاربکر"), ("fi", "Diyarbakırin maakunta"), ("fr", "Diyarbakır"), ("gu", "દિયાર\u{acd}બકીર પ\u{acd}રા\u{a82}ત"), ("he", "נפת דיארבקיר"), ("hi", "दिय\u{947}र\u{94d}बाकीर प\u{94d}रा\u{902}त"), ("hu", "Diyarbakır"), ("hy", "Դիարբեքիր"), ("id", "Provinsi Diyarbakır"), ("it", "provincia di Diyarbakır"), ("ja", "ディヤルバクル県"), ("ka", "დიარბაქირის პროვინცია"), ("kn", "ಡ\u{cbf}ಯರ\u{cbf}ಬಕ\u{cbf}ರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "디야르바키르 주"), ("lt", "Dijarbakiro provincija"), ("lv", "Dijarbakiras ils"), ("mk", "Дијарбекир"), ("mr", "दियाबाकर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Diyarbakır"), ("nb", "Diyarbakır"), ("nl", "Diyarbakır"), ("no", "Diyarbakır"), ("pl", "Diyarbakır"), ("pt", "Diyarbakır"), ("ro", "Provincia Diyarbakır"), ("ru", "Диярбакыр"), ("si", "ද\u{dd2}යර\u{dca}බක\u{dd2}ර\u{dca} පළ\u{dcf}ත"), ("sl", "Diyarbakır"), ("sq", "Dijarbakëri"), ("sr", "Дијарбакир"), ("sr_Latn", "Dijarbakir"), ("sv", "Diyarbakır"), ("sw", "Mkoa wa Diyarbakır"), ("ta", "டியர\u{bcd}பகிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c3f}య\u{c3e}ర\u{c4d}బకర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e35}ยาร\u{e4c}บาก\u{e36}ร\u{e4c}"), ("tk", "Diýarbakyr"), ("tr", "Diyarbakır"), ("uk", "Діярбакир"), ("ur", "دیار بکر صوبہ"), ("uz", "Diyarbakır"), ("vi", "Diyarbakır"), ("yue", "迪亞巴克爾省"), ("yue_Hans", "迪亚巴克尔省"), ("zh", "迪亚巴克尔省")]),
                        unofficial_name_list: ["Diyarbakır"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::TR,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.674444), longitude: Some(26.560833), max_latitude: Some(41.695851), min_latitude: Some(41.6419), max_longitude: Some(26.6036879), min_longitude: Some(26.5429609)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ادرنة"), ("az", "Ədirnə vilayəti"), ("be", "Правінцыя Эдзірне"), ("bg", "Одрин"), ("bn", "এডির\u{9cd}নে প\u{9cd}রদেশ"), ("bs", "Edirne"), ("ca", "Província d’Edirne"), ("ccp", "𑄃𑄬𑄓\u{11128}𑄢\u{11134}𑄚𑄬"), ("ceb", "Edirne"), ("cs", "Edirneská provincie"), ("cy", "Edirne"), ("da", "Edirne"), ("de", "Edirne"), ("el", "Επαρχία Αδριανούπολης"), ("en", "Edirne"), ("es", "Provincia de Edirne"), ("et", "Edirne provints"), ("eu", "Edirne probintzia"), ("fa", "استان ادرنه"), ("fi", "Edirnen maakunta"), ("fr", "Edirne"), ("gu", "એડિર\u{acd}ન\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "एडीर\u{94d}न\u{947} प\u{94d}रा\u{902}त"), ("hr", "Edirne"), ("hu", "Edirne"), ("hy", "Էդիրնե"), ("id", "Provinsi Edirne"), ("it", "provincia di Edirne"), ("ja", "エディルネ県"), ("ka", "ედირნეს პროვინცია"), ("kn", "ಎಡ\u{cbf}ರ\u{ccd}ನ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "에디르네 주"), ("lt", "Edirnės provincija"), ("lv", "Edirnes ils"), ("mk", "Одрин"), ("mr", "एदिर\u{94d}न\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Edirne"), ("nb", "Edirne"), ("ne", "एडिरन\u{947} क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Edirne"), ("no", "Edirne"), ("pl", "Edirne"), ("pt", "Edirne"), ("ro", "Provincia Edirne"), ("ru", "Эдирне"), ("si", "එඩ\u{dd2}ර\u{dca}නේ පළ\u{dcf}ත"), ("sl", "Odrin"), ("sq", "Edirne"), ("sr", "Једрене"), ("sr_Latn", "Jedrene"), ("sv", "Edirne"), ("sw", "Mkoa wa Edirne"), ("ta", "எதிர\u{bcd}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎడ\u{c3f}ర\u{c4d}న\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอเด\u{e35}ยร\u{e4c}เน"), ("tk", "Edirne"), ("tr", "Edirne"), ("uk", "Едірне"), ("ur", "ادرنہ صوبہ"), ("uz", "Edirne"), ("vi", "Edirne"), ("yue", "埃迪爾內省"), ("yue_Hans", "埃迪尔内省"), ("zh", "埃迪尔内省")]),
                        unofficial_name_list: ["Edirne"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::TR,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.680969), longitude: Some(39.226398), max_latitude: Some(38.697375), min_latitude: Some(38.631944), max_longitude: Some(39.27574010000001), min_longitude: Some(39.1205639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيلازيغ"), ("az", "Elazığ ili"), ("be", "Правінцыя Элязыг"), ("bg", "Елязъг"), ("bn", "এল\u{9be}জিগ প\u{9cd}রদেশ"), ("bs", "Elazığ"), ("ca", "Província d’Elâzığ"), ("ccp", "𑄃\u{11128}𑄣𑄎\u{11128}𑄇\u{11134}"), ("ceb", "Elazığ"), ("cs", "Elâzığ (provincie)"), ("da", "Elazığ Province"), ("de", "Elazığ"), ("el", "Επαρχία Ελαζίγκ"), ("en", "Elazığ"), ("es", "Provincia de Elazığ"), ("et", "Elazıği provints"), ("eu", "Elazığ probintzia"), ("fa", "استان الازیغ"), ("fi", "Elazığin maakunta"), ("fr", "Elâzığ"), ("gu", "એલાઝિગ પ\u{acd}રા\u{a82}ત"), ("he", "נפת אלזי"), ("hi", "एलाज\u{93c}िग"), ("hu", "Elazığ"), ("hy", "Խարբերդի նահանգ"), ("id", "Provinsi Elâzığ"), ("it", "provincia di Elâzığ"), ("ja", "エラズー県"), ("ka", "ელაზიღის პროვინცია"), ("kn", "ಎಲಾಝ\u{cbf}ಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엘라지 주"), ("lt", "Elazigo provincija"), ("lv", "Elazīgas ils"), ("mk", "Елазиг"), ("mr", "एलाझग प\u{94d}रा\u{902}त"), ("ms", "Wilayah Elâzığ"), ("nb", "Elazığ"), ("nl", "Elazığ"), ("no", "Elazığ"), ("pl", "Elazığ"), ("pt", "Elazığ"), ("ro", "Provincia Elazığ"), ("ru", "Элязыг"), ("si", "එලස\u{dd2}ග\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Ellazë"), ("sr", "Елазиг"), ("sr_Latn", "Elazig"), ("sv", "Elazığ"), ("sw", "Mkoa wa Elazığ"), ("ta", "எல\u{bbe}ஜிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎల\u{c3e}జ\u{c3f}గ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e31}ลซ\u{e34}ก"), ("tk", "Elazyg"), ("tr", "Elâzığ"), ("uk", "Елязиг"), ("ur", "الازیغ صوبہ"), ("uz", "Elazığ"), ("vi", "Elâzığ"), ("yue", "埃拉澤省"), ("yue_Hans", "埃拉泽省"), ("zh", "埃拉泽省")]),
                        unofficial_name_list: ["Elazığ"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::TR,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7399289), longitude: Some(39.504501), max_latitude: Some(39.781213), min_latitude: Some(39.712936), max_longitude: Some(39.547508), min_longitude: Some(39.4603087)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أرزنجان"), ("az", "Ərzincan ili"), ("be", "Правінцыя Эрзінджан"), ("bg", "Ерзинджан"), ("bn", "এরজিঙ\u{9cd}ক\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Erzincan"), ("ca", "Província d’Erzincan"), ("ccp", "𑄃\u{11128}𑄢\u{11134}𑄎\u{11128}𑄚\u{11134}𑄇𑄚\u{11134}"), ("ceb", "Erzincan"), ("cs", "Erzincanská provincie"), ("cy", "Erzincan"), ("da", "Erzincan Province"), ("de", "Erzincan"), ("el", "Ερζινκάν"), ("en", "Erzincan"), ("es", "Provincia de Erzincan"), ("et", "Erzincani provints"), ("eu", "Erzincan probintzia"), ("fa", "استان ارزنجان"), ("fi", "Erzincanin maakunta"), ("fr", "Erzincan"), ("gu", "એર\u{acd}ઝિનક\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "अर\u{94d}ज\u{93c}िनक\u{948}न प\u{94d}रा\u{902}त"), ("hu", "Erzincan"), ("hy", "Էրզինջանի նահանգ"), ("id", "Provinsi Erzincan"), ("it", "provincia di Erzincan"), ("ja", "エルズィンジャン県"), ("ka", "ერზინჯანის პროვინცია"), ("kn", "ಎರ\u{ccd}ಜ\u{cbf}ನ\u{ccd}ಸನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "에르진잔 주"), ("lt", "Erzindžano provincija"), ("lv", "Erzindžanas ils"), ("mk", "Ерзинџан"), ("mr", "एर\u{94d}झि\u{902}जान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Erzincan"), ("nb", "Erzincan"), ("nl", "Erzincan"), ("no", "Erzincan"), ("pl", "Erzincan"), ("pt", "Erzincan"), ("ro", "Provincia Erzincan"), ("ru", "Эрзинджан"), ("si", "එර\u{dca}ස\u{dd2}න\u{dca}කන\u{dca} පළ\u{dcf}ත"), ("sr", "Ерзинџан"), ("sr_Latn", "Erzindžan"), ("sv", "Erzincan"), ("sw", "Mkoa wa Erzincan"), ("ta", "இரசிங\u{bcd}கன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎర\u{c4d}జ\u{c3f}ంకన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอร\u{e4c}ซ\u{e34}นจ\u{e31}น"), ("tk", "Erzinjan"), ("tr", "Erzincan"), ("uk", "Ерзінджан"), ("ur", "ارزنجان صوبہ"), ("uz", "Erzincan"), ("vi", "Erzincan"), ("yue", "埃爾津詹省"), ("yue_Hans", "埃尔津詹省"), ("zh", "埃尔津詹省")]),
                        unofficial_name_list: ["Erzincan"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::TR,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.908611), longitude: Some(41.276944), max_latitude: Some(39.955924), min_latitude: Some(39.867876), max_longitude: Some(41.3105771), min_longitude: Some(41.2164562)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أرضروم"), ("az", "Ərzurum ili"), ("be", "Правінцыя Эрзурум"), ("bg", "Ерзурум"), ("bn", "এরজ\u{9c1}র\u{9be}ম প\u{9cd}রদেশ"), ("bs", "Erzurum"), ("ca", "Província d’Erzurum"), ("ccp", "𑄃\u{11128}𑄢\u{11134}𑄎\u{1112a}𑄢\u{1112a}𑄟\u{11134}"), ("ceb", "Erzurum"), ("cs", "Erzurum"), ("cy", "Erzurum"), ("da", "Erzurum"), ("de", "Erzurum"), ("el", "Ερζουρούμ"), ("en", "Erzurum"), ("es", "Provincia de Erzurum"), ("et", "Erzurumi provints"), ("eu", "Erzurum probintzia"), ("fa", "استان ارزروم"), ("fi", "Erzurumin maakunta"), ("fr", "Erzurum"), ("gu", "એર\u{acd}ઝ\u{ac2}ર\u{ac1}મ પ\u{acd}રા\u{a82}ત"), ("he", "ארזורום"), ("hi", "एर\u{94d}ज\u{93c}\u{941}र\u{941}म प\u{94d}रा\u{902}त"), ("hu", "Erzurum"), ("hy", "Էրզրում"), ("id", "Provinsi Erzurum"), ("it", "provincia di Erzurum"), ("ja", "エルズルム県"), ("ka", "ერზურუმის პროვინცია"), ("kn", "ಎರ\u{ccd}ಜುರುಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "에르주룸 주"), ("lt", "Erzurumo provincija"), ("lv", "Erzurumas ils"), ("mk", "Ерзурум"), ("mr", "एर\u{94d}झ\u{941}र\u{941}म प\u{94d}रा\u{902}त"), ("ms", "Wilayah Erzurum"), ("nb", "Erzurum"), ("nl", "Erzurum"), ("no", "Erzurum"), ("pa", "ਏਰਜ\u{a41}ਰਮ"), ("pl", "Erzurum"), ("pt", "Erzurum"), ("ro", "Provincia Erzurum"), ("ru", "Эрзурум"), ("si", "එර\u{dca}සරම\u{dca} පළ\u{dcf}ත"), ("sl", "Erzurum"), ("sq", "Erzurum"), ("sr", "Ерзурум"), ("sr_Latn", "Erzurum"), ("sv", "Erzurum"), ("sw", "Mkoa wa Erzurum"), ("ta", "ஏர\u{bcd}ஸுரும\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎర\u{c4d}జురుమ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเออซ\u{e39}ร\u{e31}ม"), ("tk", "Erzurum"), ("tr", "Erzurum"), ("uk", "Ерзурум"), ("ur", "ارض روم صوبہ"), ("uz", "Erzurum"), ("vi", "Erzurum (tỉnh)"), ("yue", "埃爾祖魯姆省"), ("yue_Hans", "埃尔祖鲁姆省"), ("zh", "埃尔祖鲁姆省")]),
                        unofficial_name_list: ["Erzurum"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::TR,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.776667), longitude: Some(30.520556), max_latitude: Some(39.828122), min_latitude: Some(39.702564), max_longitude: Some(30.6805741), min_longitude: Some(30.4090549)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إسكيشهر"), ("az", "Əskişəhər ili"), ("be", "Правінцыя Эскішэхір"), ("bg", "Ескишехир"), ("bn", "এস\u{9cd}কিসেহির প\u{9cd}রদেশ"), ("bs", "Eskişehir"), ("ca", "Província d’Eskişehir"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄇\u{11128}𑄥𑄬𑄦\u{11128}𑄢\u{11134}"), ("ceb", "Eskişehir"), ("cs", "Eskişehir"), ("da", "Eskişehir Province"), ("de", "Eskişehir"), ("el", "Επαρχία Εσκί Σεχίρ"), ("en", "Eskişehir"), ("es", "Provincia de Eskişehir"), ("et", "Eskişehiri provints"), ("eu", "Eskişehir probintzia"), ("fa", "استان اسکی\u{200c}شهر"), ("fi", "Eskişehirin maakunta"), ("fr", "Eskişehir"), ("gu", "એસ\u{acd}કિશ\u{ac7}હિર પ\u{acd}રા\u{a82}ત"), ("hi", "एस\u{94d}कीस\u{947}हिर प\u{94d}रा\u{902}त"), ("hu", "Eskişehir"), ("hy", "Էսքիշեհիր"), ("id", "Provinsi Eskişehir"), ("it", "provincia di Eskişehir"), ("ja", "エスキシェヒル県"), ("ka", "ესქიშეჰირის პროვინცია"), ("kn", "ಎಸ\u{ccd}ಕ\u{cbf}ಶೀರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "에스키셰히르 주"), ("lt", "Eskišehiro provincija"), ("lv", "Eskišehiras ils"), ("mk", "Ескишехир"), ("mr", "एस\u{94d}किश\u{947}हिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Eskişehir"), ("nb", "Eskişehir"), ("nl", "Eskişehir"), ("no", "Eskişehir"), ("pl", "Eskişehir"), ("pt", "Eskişehir"), ("ro", "Provincia Eskișehir"), ("ru", "Эскишехир"), ("si", "එස\u{dca}ක\u{dd2}සේහ\u{dd2}ර\u{dca} පළ\u{dcf}ත"), ("sr", "Ескишехир"), ("sr_Latn", "Eskišehir"), ("sv", "Eskişehir"), ("sw", "Mkoa wa Eskişehir"), ("ta", "எஸ\u{bcd}கிசோஹிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎస\u{c4d}క\u{c3f}స\u{c46}హ\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องหลวงเอสก\u{e34}ซ\u{e35}เฮ\u{e35}ยร\u{e4c}"), ("tk", "Eskişehir"), ("tr", "Eskişehir"), ("uk", "Ескішехір"), ("ur", "اسکی شہر صوبہ"), ("uz", "Eskişehir"), ("vi", "Eskişehir"), ("yue", "埃斯基謝希爾省"), ("yue_Hans", "埃斯基谢希尔省"), ("zh", "埃斯基谢希尔省")]),
                        unofficial_name_list: ["Eskişehir"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::TR,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.06622), longitude: Some(37.38332), max_latitude: Some(37.108291), min_latitude: Some(37.009252), max_longitude: Some(37.4567749), min_longitude: Some(37.3034992)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "عنتاب"), ("az", "Qaziantep ili"), ("be", "Правінцыя Газіянтэп"), ("bg", "Газиантеп"), ("bn", "গ\u{9be}জিয\u{9bc}\u{9be}ন\u{9cd}টেপ প\u{9cd}রদেশ"), ("bs", "Gaziantep"), ("ca", "Província de Gaziantep"), ("ccp", "𑄉\u{11133}𑄠𑄎\u{11128}𑄠𑄚\u{11134}𑄑𑄬𑄛\u{11134}"), ("ceb", "Gaziantep"), ("cs", "Gaziantepská provincie"), ("cy", "Gaziantep"), ("da", "Gaziantep Province"), ("de", "Gaziantep"), ("el", "Επαρχία Γκαζιαντέπ"), ("en", "Gaziantep"), ("es", "Provincia de Gaziantep"), ("et", "Gaziantepi provints"), ("eu", "Gaziantep probintzia"), ("fa", "استان غازی\u{200c}عینتاب"), ("fi", "Gaziantepin maakunta"), ("fr", "Gaziantep"), ("gu", "ગાઝયન\u{acd}ટ\u{ac7}પ પ\u{acd}રા\u{a82}ત"), ("hi", "गज\u{93c}िया\u{902}ट\u{947}प प\u{94d}रा\u{902}त"), ("hu", "Gaziantep"), ("hy", "Գազիանթեփ"), ("id", "Provinsi Gaziantep"), ("it", "provincia di Gaziantep"), ("ja", "ガズィアンテプ県"), ("ka", "გაზიანთეფის პროვინცია"), ("kn", "ಗಜ\u{cbf}ಯಾಂಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "가지안테프 주"), ("lt", "Gaziantepo provincija"), ("lv", "Gaziantepas ils"), ("mk", "Газиантеп"), ("mr", "गाझियान\u{94d}त\u{947}प प\u{94d}रा\u{902}त"), ("ms", "Wilayah Gaziantep"), ("nb", "Gaziantep"), ("nl", "Gaziantep"), ("no", "Gaziantep"), ("pl", "Gaziantep"), ("pt", "Gaziantep"), ("ro", "Provincia Gaziantep"), ("ru", "Газиантеп"), ("si", "ග\u{dcf}ස\u{dd2}යන\u{dca}ටෙප\u{dca} පළ\u{dcf}ත"), ("sr", "Газијантеп"), ("sr_Latn", "Gazijantep"), ("sv", "Gaziantep"), ("sw", "Mkoa wa Gaziantep"), ("ta", "கசியன\u{bcd}டேப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3e}జ\u{c3f}య\u{c3e}ంట\u{c46}ప\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e31}ล ชาฮ\u{e4c}เค\u{e35}ย โกเวอโนเรท"), ("tk", "Gaziantep"), ("tr", "Gaziantep"), ("uk", "Ґазіантеп"), ("ur", "غازی عینتاب صوبہ"), ("uz", "Gaziantep"), ("vi", "Gaziantep"), ("yue", "加濟安泰普省"), ("yue_Hans", "加济安泰普省"), ("zh", "加济安泰普省")]),
                        unofficial_name_list: ["Gaziantep"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::TR,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.912811), longitude: Some(38.38953), max_latitude: Some(40.92777), min_latitude: Some(40.880759), max_longitude: Some(38.4476309), min_longitude: Some(38.3108371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غيرسون"), ("az", "Girəsun ili"), ("be", "Правінцыя Гірэсун"), ("bg", "Гиресун"), ("bn", "গিরেস\u{9c1}ন প\u{9cd}রদেশ"), ("bs", "Giresun"), ("ca", "Província de Giresun"), ("ccp", "𑄉\u{11128}𑄠𑄬𑄢\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Giresun (lalawigan)"), ("cs", "Giresunská provincie"), ("cy", "Giresun"), ("da", "Giresun Province"), ("de", "Giresun"), ("el", "Επαρχία Κερασούντας"), ("en", "Giresun"), ("es", "Provincia de Giresun"), ("et", "Giresuni provints"), ("eu", "Giresun probintzia"), ("fa", "استان گره\u{200c}سون"), ("fi", "Giresunin maakunta"), ("fr", "Giresun"), ("gu", "ગિર\u{acd}સન પ\u{acd}રા\u{a82}ત"), ("hi", "गियरसन प\u{94d}रा\u{902}त"), ("hu", "Giresun"), ("hy", "Գիրեսուն"), ("id", "Provinsi Giresun"), ("it", "provincia di Giresun"), ("ja", "ギレスン県"), ("ka", "გირესუნის პროვინცია"), ("kn", "ಜ\u{cbf}ರಸುನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "기레순 주"), ("lt", "Giresuno provincija"), ("lv", "Giresunas ils"), ("mk", "Гиресун"), ("mr", "गिर\u{947}स\u{941}न प\u{94d}रा\u{902}त"), ("ms", "Wilayah Giresun"), ("nb", "Giresun"), ("nl", "Giresun"), ("no", "Giresun"), ("pl", "Giresun"), ("pt", "Giresun (província)"), ("ro", "Provincia Giresun"), ("ru", "Гиресун"), ("si", "ගරෙසන\u{dca} පළ\u{dcf}ත"), ("sq", "Giresun"), ("sr", "Гиресун"), ("sr_Latn", "Giresun"), ("sv", "Giresun"), ("sw", "Mkoa wa Giresun"), ("ta", "கிரேசன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3f}ర\u{c46}సున\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e34}เรซ\u{e38}น"), ("tk", "Giresun"), ("tr", "Giresun"), ("uk", "Ґіресун"), ("ur", "گریسون صوبہ"), ("uz", "Giresun"), ("vi", "Giresun"), ("yue", "吉雷松省"), ("yue_Hans", "吉雷松省"), ("zh", "吉雷松省")]),
                        unofficial_name_list: ["Giresun"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::TR,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.4385883), longitude: Some(39.5085557), max_latitude: Some(40.4684801), min_latitude: Some(40.4282392), max_longitude: Some(39.5207309), min_longitude: Some(39.4586489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غوموشخان"), ("az", "Gümüşxanə ili"), ("be", "Правінцыя Гюмюшханэ"), ("bg", "Гюмюшхане"), ("bs", "Gümüşhane"), ("ca", "Província de Gümüşhane"), ("ccp", "𑄉\u{1112a}𑄟\u{1112a}𑄥𑄚\u{11134}"), ("ceb", "Gümüşhane (lalawigan sa Turkiya)"), ("cs", "Gümüşhanská provincie"), ("cy", "Gümüşhane"), ("de", "Gümüşhane"), ("el", "Επαρχία Γκιουμούσχανε"), ("en", "Gümüşhane"), ("es", "Provincia de Gümüşhane"), ("et", "Gümüşhane provints"), ("eu", "Gümüşhane probintzia"), ("fa", "استان گوموش\u{200c}خانه"), ("fi", "Gümüşhanen maakunta"), ("fr", "Gümüşhane"), ("hu", "Gümüşhane"), ("hy", "Գյումյուշխանե"), ("id", "Provinsi Gümüşhane"), ("it", "provincia di Gümüşhane"), ("ja", "ギュミュシュハーネ県"), ("ka", "გუმუშჰანეს პროვინცია"), ("ko", "귀뮈샤네 주"), ("lv", "Gimišhanes ils"), ("mk", "Ѓумушхане"), ("mr", "ग\u{94d}य\u{941}म\u{941}शान\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Gümüşhane"), ("nb", "Gümüşhane"), ("nl", "Gümüşhane"), ("no", "Gümüşhane"), ("pl", "Gümüşhane"), ("pt", "Gümüşhane"), ("ro", "Provincia Gümüșhane"), ("ru", "Гюмюшхане"), ("sr", "Гумушхане"), ("sr_Latn", "Gumušhane"), ("sv", "Gümüşhane"), ("sw", "Mkoa wa Gümüşhane"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e37}อม\u{e36}ชฮาแน"), ("tk", "Gümüşhane"), ("tr", "Gümüşhane"), ("uk", "Ґюмюшхане"), ("ur", "گوموشخانے صوبہ"), ("uz", "Gümüşhane"), ("vi", "Gümüşhane"), ("yue", "居米什哈內省"), ("yue_Hans", "居米什哈内省"), ("zh", "居米什哈内省")]),
                        unofficial_name_list: ["Gümüşhane"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::TR,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.576389), longitude: Some(43.736667), max_latitude: Some(37.592381), min_latitude: Some(37.556175), max_longitude: Some(43.754594), min_longitude: Some(43.7149901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "حكاري"), ("az", "Hakkari vilayəti"), ("be", "Правінцыя Хаккары"), ("bg", "Хаккяри"), ("bn", "হ\u{9be}ক\u{9cd}ক\u{9be}রি প\u{9cd}রদেশ"), ("bs", "Hakkari"), ("ca", "Província de Hakkâri"), ("ccp", "𑄦𑄇\u{11133}𑄦𑄢\u{11128}"), ("cs", "Hakkarijská provincie"), ("cy", "Hakkâri"), ("da", "Hakkari Provins"), ("de", "Hakkâri"), ("el", "Χακάρι"), ("en", "Hakkâri"), ("es", "Provincia de Hakkâri"), ("et", "Hakkâri provints"), ("eu", "Hakkari probintzia"), ("fa", "استان حکاری"), ("fi", "Hakkarin maakunta"), ("fr", "Hakkari"), ("gu", "હક\u{acd}કારી પ\u{acd}રા\u{a82}ત"), ("he", "האקארי"), ("hi", "हक\u{94d}कारी प\u{94d}रोवि\u{902}स"), ("hu", "Hakkari"), ("hy", "Հակյարի"), ("id", "Provinsi Hakkari"), ("it", "provincia di Hakkâri"), ("ja", "ハッキャリ県"), ("ka", "ჰაქარის პროვინცია"), ("kn", "ಹಕ\u{ccd}ಕರ\u{cbf} ಪ\u{ccd}ರಾ೦ತ\u{ccd}ಯ"), ("ko", "하카리 주"), ("lt", "Hakario provincija"), ("lv", "Hakari ils"), ("mk", "Хаќари"), ("mr", "हक\u{94d}कारी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Hakkâri"), ("nb", "Hakkâri"), ("nl", "Hakkâri"), ("no", "Hakkâri"), ("pl", "Hakkari"), ("pt", "Hakkâri"), ("ro", "Provincia Hakkâri"), ("ru", "Хаккяри"), ("si", "හක\u{dca}ක\u{dcf}ර\u{dd3} පළ\u{dcf}ත"), ("sl", "Hakkâri"), ("sr", "Хакари"), ("sr_Latn", "Hakari"), ("sv", "Hakkari"), ("sw", "Mkoa wa Hakkâri"), ("ta", "ஹக\u{bcd}கரின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హక\u{c4d}క\u{c3e}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e31}คคาร\u{e34}"), ("tk", "Hakkari"), ("tr", "Hakkâri"), ("uk", "Хаккярі"), ("ur", "حکاری صوبہ"), ("uz", "Hakkari"), ("vi", "Hakkâri"), ("yue", "哈卡裡省"), ("yue_Hans", "哈卡里省"), ("zh", "哈卡里省")]),
                        unofficial_name_list: ["Hakkâri"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::TR,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.2), longitude: Some(36.166667), max_latitude: Some(36.27726), min_latitude: Some(36.16479), max_longitude: Some(36.231053), min_longitude: Some(36.1002049)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هتاي"), ("az", "Hatay vilayəti"), ("be", "Правінцыя Хатай"), ("bg", "Хатай"), ("bn", "হ\u{9be}ত\u{9be}য\u{9bc}"), ("bs", "Hatay"), ("ca", "Província de Hatay"), ("ccp", "𑄦𑄑𑄬"), ("ceb", "Hatay"), ("cs", "Hatayská provincie"), ("cy", "Hatay"), ("da", "Hatay"), ("de", "Hatay"), ("el", "Επαρχία Χατάι"), ("en", "Hatay"), ("es", "Provincia de Hatay"), ("et", "Hatay provints"), ("eu", "Hatay probintzia"), ("fa", "استان ختای"), ("fi", "Hatayn maakunta"), ("fr", "Hatay"), ("gu", "હટાય"), ("he", "נפת האטיי"), ("hi", "ह\u{947}ट\u{947}"), ("hu", "Hatay"), ("hy", "Հաթայ"), ("id", "Provinsi Hatay"), ("is", "Hatay"), ("it", "provincia di Hatay"), ("ja", "ハタイ県"), ("ka", "ჰათაის პროვინცია"), ("kn", "ಹತಾಯ\u{ccd}"), ("ko", "하타이 주"), ("lt", "Chatajus"), ("lv", "Hatajas ils"), ("mk", "Хатај"), ("mr", "हाताय प\u{94d}रा\u{902}त"), ("ms", "Wilayah Hatay"), ("nb", "Hatay"), ("nl", "Hatay"), ("no", "Hatay"), ("pa", "ਹਤਾਏ"), ("pl", "Hatay"), ("pt", "Hatay"), ("ro", "Provincia Hatay"), ("ru", "Хатай"), ("si", "හටේ"), ("sl", "Hatay"), ("sr", "Хатај"), ("sr_Latn", "Hataj"), ("sv", "Hatay"), ("sw", "Mkoa wa Hatay"), ("ta", "க\u{bbe}ட\u{bcd}ட\u{bbe}ய\u{bcd}"), ("te", "హట\u{c3e}య\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮาไต"), ("tk", "Hataý"), ("tr", "Hatay"), ("uk", "Хатай"), ("ur", "ہاتے صوبہ"), ("uz", "Hatay"), ("vi", "Hatay"), ("yue", "哈塔伊省"), ("yue_Hans", "哈塔伊省"), ("zh", "哈塔伊省")]),
                        unofficial_name_list: ["Hatay"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::TR,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.764771), longitude: Some(30.556561), max_latitude: Some(37.834092), min_latitude: Some(37.746192), max_longitude: Some(30.5919461), min_longitude: Some(30.5077259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إسبرطة"), ("az", "İsparta ili"), ("be", "Правінцыя Ыспарта"), ("bg", "Ъспарта"), ("bn", "ইস\u{9cd}প\u{9be}র\u{9cd}ট\u{9be} প\u{9cd}রদেশ"), ("bs", "Isparta"), ("ca", "Província d’Isparta"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄛𑄢\u{11134}𑄑"), ("ceb", "Isparta (lalawigan)"), ("cs", "Ispartská provincie"), ("da", "Isparta Province"), ("de", "Isparta"), ("el", "Επαρχία Ισπάρτα"), ("en", "Isparta"), ("es", "Provincia de Isparta"), ("et", "Isparta provints"), ("eu", "Isparta probintzia"), ("fa", "استان اسپارتا"), ("fi", "Ispartan maakunta"), ("fr", "Isparta"), ("gu", "ઇસ\u{acd}પાર\u{acd}તા પ\u{acd}રા\u{a82}ત"), ("hi", "इस\u{94d}पार\u{94d}टा प\u{94d}रा\u{902}त"), ("hu", "Isparta"), ("hy", "Ըսպարտայի նահանգ"), ("id", "Provinsi Isparta"), ("it", "provincia di Isparta"), ("ja", "ウスパルタ県"), ("ka", "ისპარტის პროვინცია"), ("kn", "ಇಸ\u{ccd}ಪಾರ\u{ccd}ಟಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "이스파르타 주"), ("lt", "Ispartos provincija"), ("lv", "Ispartas ils"), ("mk", "Испарта"), ("mr", "इस\u{94d}पार\u{94d}ता प\u{94d}रा\u{902}त"), ("ms", "Wilayah Isparta"), ("nb", "Isparta"), ("nl", "Isparta"), ("no", "Isparta"), ("pa", "ਇਸਪਾਰਤਾ ਸ\u{a42}ਬਾ"), ("pl", "Isparta"), ("pt", "Isparta"), ("ro", "Provincia Isparta"), ("ru", "Ыспарта"), ("si", "ඉස\u{dca}පර\u{dca}ට\u{dcf} කල\u{dcf}පය"), ("sl", "Isparta"), ("sr", "Испарта"), ("sr_Latn", "Isparta"), ("sv", "Isparta"), ("sw", "Mkoa wa Isparta"), ("ta", "இஸ\u{bcd}ப\u{bbe}ர\u{bcd}ட\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇస\u{c4d}ప\u{c3e}ర\u{c4d}ట\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}สปาร\u{e4c}ตา"), ("tr", "Isparta"), ("uk", "Испарта"), ("ur", "اسپارتا صوبہ"), ("uz", "Isparta"), ("vi", "Isparta"), ("yue", "伊斯帕爾塔省"), ("yue_Hans", "伊斯帕尔塔省"), ("zh", "伊斯帕尔塔省")]),
                        unofficial_name_list: ["Isparta"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::TR,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.6321731), longitude: Some(33.617577), max_latitude: Some(37.435227), min_latitude: Some(36.01681), max_longitude: Some(35.14063), min_longitude: Some(32.5361768)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مرسين"), ("az", "Mersin vilayəti"), ("be", "Правінцыя Мерсін"), ("bg", "Мерсин"), ("bn", "মেরসিন প\u{9cd}রদেশ"), ("bs", "Mersin"), ("ca", "Província de Mersin"), ("ccp", "𑄟𑄬𑄢\u{11134}𑄥\u{11128}𑄚\u{11134}"), ("cs", "Mersinská provincie"), ("da", "Mersin Province"), ("de", "Mersin"), ("el", "Επαρχία Μερσίν"), ("en", "Mersin"), ("es", "Provincia de Mersin"), ("et", "Mersini provints"), ("eu", "Mersin probintzia"), ("fa", "استان مرسین"), ("fi", "Mersinin maakunta"), ("fr", "Mersin"), ("gu", "મ\u{ac7}ર\u{acd}સિન પ\u{acd}રા\u{a82}ત"), ("hi", "मर\u{94d}सिन प\u{94d}रा\u{902}त"), ("hu", "Mersin"), ("hy", "Մերսին"), ("id", "Provinsi Mersin"), ("it", "provincia di Mersin"), ("ja", "メルスィン県"), ("ka", "მერსინის პროვინცია"), ("kn", "ಮ\u{cc6}ರ\u{ccd}ಸ\u{cbf}ನ\u{ccd}"), ("ko", "메르신 주"), ("lt", "Mersino provincija"), ("lv", "Mersinas ils"), ("mk", "Мерсин"), ("mr", "म\u{947}र\u{94d}सिन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mersin"), ("nb", "Mersin"), ("nl", "Mersin"), ("no", "Mersin"), ("pa", "ਮਰਸਿਨ"), ("pl", "Mersin"), ("pt", "Mersin"), ("ro", "Provincia Mersin"), ("ru", "Мерсин"), ("si", "මර\u{dca}ස\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sr", "Мерсин"), ("sr_Latn", "Mersin"), ("sv", "Mersin"), ("sw", "Mkoa wa Mersin"), ("ta", "மெர\u{bcd}ஸின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c46}ర\u{c3f}\u{c4d}స\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเมอซ\u{e34}น"), ("tk", "Mersin"), ("tr", "Mersin"), ("uk", "Мерсін"), ("ur", "مرسین صوبہ"), ("vi", "Mersin"), ("yue", "梅爾辛省"), ("yue_Hans", "梅尔辛省"), ("zh", "梅尔辛省")]),
                        unofficial_name_list: ["İçel"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::TR,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.00527), longitude: Some(28.97696), max_latitude: Some(41.199239), min_latitude: Some(40.811404), max_longitude: Some(29.4288052), min_longitude: Some(28.5955538)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إسطنبول"), ("az", "İstanbul ili"), ("be", "Правінцыя Стамбул"), ("bg", "Истанбул"), ("bs", "Istanbul"), ("ca", "Província d’Istanbul"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄑𑄚\u{11134}𑄝\u{1112a}𑄣\u{11134}"), ("cs", "Istanbulská provincie"), ("de", "Istanbul"), ("el", "Επαρχία Κωνσταντινούπολης"), ("en", "Istanbul"), ("es", "Provincia de Estambul"), ("et", "İstanbuli provints"), ("eu", "Istanbul probintzia"), ("fa", "استان استانبول"), ("fi", "İstanbulin maakunta"), ("fr", "Istanbul"), ("he", "איסטנבול"), ("hi", "इस\u{94d}ता\u{902}ब\u{941}ल प\u{94d}रा\u{902}त"), ("hu", "İstanbul"), ("hy", "Ստամբուլի նահանգ"), ("id", "Provinsi İstanbul"), ("it", "provincia di Istanbul"), ("ja", "イスタンブール県"), ("jv", "Provinsi İstanbul"), ("ka", "სტამბოლის პროვინცია"), ("ko", "이스탄불 주"), ("lt", "Stambulo provincija"), ("lv", "Stambulas ils"), ("mk", "Истанбул"), ("mr", "इस\u{94d}त\u{902}ब\u{942}ल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Istanbul"), ("nb", "İstanbul"), ("nl", "Istanboel"), ("no", "İstanbul"), ("pa", "ਇਸਤਾ\u{a02}ਬ\u{a41}ਲ ਸ\u{a42}ਬਾ"), ("pl", "Stambuł"), ("pt", "Istambul (província)"), ("ro", "Provincia Istanbul"), ("ru", "Стамбул"), ("sl", "İstanbul"), ("sr", "Истанбулски вилајет"), ("sr_Latn", "Istanbulski vilajet"), ("sv", "İstanbul"), ("sw", "Mkoa wa Istanbul"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}สต\u{e31}นบ\u{e39}ล"), ("tk", "Stambul"), ("tr", "İstanbul"), ("uk", "Стамбул"), ("ur", "استنبول صوبہ"), ("uz", "Istanbul"), ("vi", "Istanbul"), ("yue", "伊斯坦堡省"), ("yue_Hans", "伊斯坦堡省"), ("zh", "伊斯坦堡省")]),
                        unofficial_name_list: ["İstanbul"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::TR,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.41885), longitude: Some(27.12872), max_latitude: Some(38.545237), min_latitude: Some(38.290113), max_longitude: Some(27.3044851), min_longitude: Some(26.8549492)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ازمير"), ("az", "İzmir ili"), ("be", "Правінцыя Ізмір"), ("bg", "Измир (вилает)"), ("bn", "ইজমির প\u{9cd}রদেশ"), ("bs", "İzmir"), ("ca", "Província d’İzmir"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄟\u{11128}𑄢\u{11134}"), ("ceb", "İzmir"), ("cs", "Izmirská provincie"), ("cy", "İzmir"), ("da", "Izmir Province"), ("de", "Izmir"), ("el", "Επαρχία Σμύρνης"), ("en", "Izmir"), ("es", "Provincia de Esmirna"), ("et", "İzmiri provints"), ("eu", "Izmir probintzia"), ("fa", "استان ازمیر"), ("fi", "İzmirin maakunta"), ("fr", "Izmir"), ("gl", "Provincia de Esmirna"), ("gu", "ઇઝમિર પ\u{acd}રા\u{a82}ત"), ("hi", "इज\u{93c}मिर प\u{94d}रा\u{902}त"), ("hu", "İzmir (tartomány)"), ("hy", "Իզմիրի նահանգ"), ("id", "Provinsi İzmir"), ("it", "provincia di Smirne"), ("ja", "イズミル県"), ("ka", "იზმირის პროვინცია"), ("kn", "ಇಜ\u{ccd}ಮ\u{cbf}ರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "이즈미르 주"), ("lt", "Izmiro provincija"), ("lv", "Izmiras ils"), ("mk", "Измир (покраина)"), ("mr", "इझ\u{94d}मिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah İzmir"), ("nb", "İzmir"), ("nl", "İzmir"), ("no", "İzmir"), ("pl", "İzmir"), ("pt", "Esmirna (província)"), ("ro", "Provincia İzmir"), ("ru", "Измир"), ("si", "ඉස\u{dca}ම\u{dd3}ර\u{dca} පළ\u{dcf}ත"), ("sl", "İzmir (provinca)"), ("sq", "Smirnë"), ("sr", "Измир (вилајет)"), ("sr_Latn", "Izmir (vilajet)"), ("sv", "İzmir (provins)"), ("sw", "Mkoa wa İzmir"), ("ta", "இஸ\u{bcd}மிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇజ\u{c4d}మ\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}สเมอร\u{e4c}"), ("tk", "Izmir"), ("tr", "İzmir (il)"), ("uk", "Ізмір (іл)"), ("ur", "ازمیر صوبہ"), ("uz", "Izmir (viloyati)"), ("vi", "İzmir (tỉnh)"), ("yue", "伊茲密爾省"), ("yue_Hans", "伊兹密尔省"), ("zh", "伊兹密尔省")]),
                        unofficial_name_list: ["İzmir"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::TR,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.616667), longitude: Some(43.1), max_latitude: Some(40.623337), min_latitude: Some(40.570057), max_longitude: Some(43.1325241), min_longitude: Some(43.071051)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قارص"), ("az", "Qars vilayəti"), ("be", "Правінцыя Карс"), ("bg", "Карс"), ("bs", "Kars"), ("ca", "Província de Kars"), ("ccp", "𑄇𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Kars"), ("cs", "Karská provincie"), ("cy", "Kars"), ("de", "Kars"), ("el", "Επαρχία Καρς"), ("en", "Kars"), ("es", "Kars"), ("et", "Karsi provints"), ("eu", "Kars probintzia"), ("fa", "استان قارص"), ("fi", "Karsin maakunta"), ("fr", "Kars"), ("he", "קארס"), ("hi", "कार\u{94d}स प\u{94d}रा\u{902}त"), ("hu", "Kars"), ("hy", "Կարսի նահանգ"), ("id", "Provinsi Kars"), ("it", "Kars"), ("ja", "カルス県"), ("ka", "ყარსის პროვინცია"), ("ko", "카르스 주"), ("lv", "Karsas ils"), ("mk", "Карс"), ("mr", "कार\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kars"), ("nb", "Kars"), ("nl", "Kars"), ("no", "Kars"), ("pl", "Kars"), ("pt", "Kars"), ("ro", "Provincia Kars"), ("ru", "Карс"), ("sl", "Kars"), ("sr", "Карс"), ("sr_Latn", "Kars"), ("sv", "Kars"), ("sw", "Mkoa wa Kars"), ("th", "จ\u{e31}งหว\u{e31}ดคาร\u{e4c}ส"), ("tk", "Kars"), ("tr", "Kars"), ("uk", "Карс"), ("ur", "قارص صوبہ"), ("uz", "Kars"), ("vi", "Kars"), ("yue", "卡爾斯省"), ("yue_Hans", "卡尔斯省"), ("zh", "卡尔斯省")]),
                        unofficial_name_list: ["Kars"].to_vec(),
                    }
                ),
                (
                    "37",
                    Subdivision{
                        name: "37",
                        country_alpha2: Alpha2::TR,
                        code: "37",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.38871), longitude: Some(33.78273), max_latitude: Some(41.437135), min_latitude: Some(41.358148), max_longitude: Some(33.8178499), min_longitude: Some(33.7584892)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قسطموني"), ("az", "Kastamonu ili"), ("be", "Правінцыя Кастаману"), ("bg", "Кастамону"), ("bn", "ক\u{9be}স\u{9cd}ত\u{9be}মন\u{9c1} প\u{9cd}রদেশ"), ("bs", "Kastamonu"), ("ca", "Província de Kastamonu"), ("ccp", "𑄇𑄌\u{11134}𑄑𑄟\u{1112e}𑄚\u{1112a}"), ("ceb", "Kastamonu (lalawigan)"), ("cs", "Kastamonská provincie"), ("cy", "Kastamonu"), ("da", "Kastamonu Province"), ("de", "Kastamonu"), ("el", "Επαρχία Κασταμονής"), ("en", "Kastamonu"), ("es", "Provincia de Kastamonu"), ("et", "Kastamonu provints"), ("eu", "Kastamonu probintzia"), ("fa", "استان قسطمونی"), ("fi", "Kastamonun maakunta"), ("fr", "Kastamonu"), ("gu", "કાસ\u{acd}તમોનો પ\u{acd}રા\u{a82}ત"), ("hi", "कस\u{94d}तामोन\u{941} प\u{94d}रा\u{902}त"), ("hu", "Kastamonu"), ("hy", "Կաստամոնու"), ("id", "Provinsi Kastamonu"), ("it", "provincia di Kastamonu"), ("ja", "カスタモヌ県"), ("ka", "კასთამონუს პროვინცია"), ("kn", "ಕಸ\u{ccd}ತಮೋನು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카스타모누 주"), ("lt", "Kastamono provincija"), ("lv", "Kastamonu ils"), ("mk", "Кастамону"), ("mr", "कास\u{94d}तामोन\u{942} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kastamonu"), ("nb", "Kastamonu"), ("nl", "Kastamonu"), ("no", "Kastamonu"), ("pa", "ਕਾਸਤਾਮ\u{a4b}ਨ\u{a4b} ਸ\u{a42}ਬਾ"), ("pl", "Kastamonu"), ("pt", "Kastamonu"), ("ro", "Provincia Kastamonu"), ("ru", "Кастамону"), ("si", "කස\u{dca}ටමොන\u{dd4} පළ\u{dcf}ත"), ("sr", "Кастамону"), ("sr_Latn", "Kastamonu"), ("sv", "Kastamonu"), ("sw", "Mkoa wa Kastamonu"), ("ta", "கஷ\u{bcd}டமோனு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}స\u{c4d}ట\u{c3e}మ\u{c4b}ను ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาสตาโมน\u{e39}"), ("tk", "Kastamonu"), ("tr", "Kastamonu"), ("uk", "Кастамону"), ("ur", "کاستامونو صوبہ"), ("uz", "Kastamonu"), ("vi", "Kastamonu"), ("yue", "卡斯塔莫努省"), ("yue_Hans", "卡斯塔莫努省"), ("zh", "卡斯塔莫努省")]),
                        unofficial_name_list: ["Kastamonu"].to_vec(),
                    }
                ),
                (
                    "38",
                    Subdivision{
                        name: "38",
                        country_alpha2: Alpha2::TR,
                        code: "38",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.73122), longitude: Some(35.478729), max_latitude: Some(38.8248), min_latitude: Some(38.622002), max_longitude: Some(35.627675), min_longitude: Some(35.3126992)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قيصرية"), ("az", "Kayseri ili"), ("be", "Правінцыя Кайсеры"), ("bg", "Кайсери"), ("bn", "কিয\u{9bc}েসির প\u{9cd}রদেশ"), ("bs", "Kayseri"), ("ca", "Província de Kayseri"), ("ccp", "𑄇𑄬𑄠𑄥𑄬𑄢\u{11128}"), ("ceb", "Kayseri"), ("cs", "Kayseri"), ("da", "Kayseri Province"), ("de", "Kayseri"), ("el", "Επαρχία Καισάρειας"), ("en", "Kayseri"), ("es", "Provincia de Kayseri"), ("et", "Kayseri provints"), ("eu", "Kayseri probintzia"), ("fa", "استان قیصریه"), ("fi", "Kayserin maakunta"), ("fr", "Kayseri"), ("gu", "ક\u{ac7}સ\u{ac7}રી પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{947}स\u{947}री प\u{94d}रा\u{902}त"), ("hu", "Kayseri"), ("hy", "Կայսերի"), ("id", "Provinsi Kayseri"), ("it", "provincia di Kayseri"), ("ja", "カイセリ県"), ("ka", "კაისერის პროვინცია"), ("kn", "ಕಯ\u{ccd}ಸ\u{cc6}ರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카이세리 주"), ("lt", "Kaiserio provincija"), ("lv", "Kajseri ils"), ("mk", "Кајсери"), ("mr", "कायस\u{947}री प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kayseri"), ("nb", "Kayseri"), ("nl", "Kayseri"), ("no", "Kayseri"), ("pa", "ਕਾਇਸ\u{a47}ਰੀ"), ("pl", "Kayseri"), ("pt", "Kayseri"), ("ro", "Provincia Kayseri"), ("ru", "Кайсери"), ("si", "කෙසේර\u{dd2} පළ\u{dcf}ත"), ("sr", "Кајсери"), ("sr_Latn", "Kajseri"), ("sv", "Kayseri"), ("sw", "Mkoa wa Kayseri"), ("ta", "கைசேரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c47}స\u{c46}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไกเซร\u{e35}"), ("tk", "Kaýseri"), ("tr", "Kayseri"), ("uk", "Кайсері"), ("ur", "قیصری صوبہ"), ("uz", "Kayseri"), ("vi", "Kayseri"), ("yue", "開塞利省"), ("yue_Hans", "开塞利省"), ("zh", "开塞利省")]),
                        unofficial_name_list: ["Kayseri"].to_vec(),
                    }
                ),
                (
                    "39",
                    Subdivision{
                        name: "39",
                        country_alpha2: Alpha2::TR,
                        code: "39",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.742908), longitude: Some(27.226089), max_latitude: Some(41.760377), min_latitude: Some(41.712183), max_longitude: Some(27.244107), min_longitude: Some(27.1916528)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كيركلاريلي"), ("az", "Kırklareli ili"), ("be", "Правінцыя Кыркларэлі"), ("bg", "Лозенград"), ("bn", "কির\u{9cd}ক\u{9be}রেলি প\u{9cd}রদেশ"), ("bs", "Kırklareli"), ("ca", "Província de Kırklareli"), ("ccp", "𑄇\u{11128}𑄢\u{11134}𑄇\u{11134}𑄣𑄢𑄬𑄣\u{11128}"), ("ceb", "Kırklareli"), ("cs", "Kırklarelská provincie"), ("da", "Kırklareli Province"), ("de", "Kırklareli"), ("el", "Επαρχία Σαράντα Εκκλησιών"), ("en", "Kırklareli"), ("es", "Provincia de Kırklareli"), ("et", "Kırklareli provints"), ("eu", "Kirklareli probintzia"), ("fa", "استان قرقلرایلی"), ("fi", "Kırklarelin maakunta"), ("fr", "Kırklareli"), ("gu", "કિર\u{acd}કલર\u{ac7}લી પ\u{acd}રા\u{a82}ત"), ("hi", "किर\u{94d}कलार\u{947}ली प\u{94d}रा\u{902}त"), ("hu", "Kırklareli"), ("hy", "Քըրքլարելի"), ("id", "Provinsi Kırklareli"), ("it", "provincia di Kırklareli"), ("ja", "クルクラーレリ県"), ("ka", "ქირკლარელის პროვინცია"), ("kn", "ಕ\u{cbf}ರ\u{ccd}ಕ\u{ccd}ಲಾರ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키르클라렐리 주"), ("lt", "Kirklarelio provincija"), ("lv", "Kirkraleri ils"), ("mk", "Кркларели"), ("mr", "कर\u{94d}क\u{94d}लार\u{947}ली प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kırklareli"), ("nb", "Kırklareli"), ("nl", "Kırklareli"), ("no", "Kırklareli"), ("pl", "Kırklareli"), ("pt", "Kırklareli"), ("ro", "Provincia Kırklareli"), ("ru", "Кыркларели"), ("si", "කරක\u{dca}ලරේල\u{dd2} පළ\u{dcf}ත"), ("sl", "Kırklareli"), ("sq", "Kërklareli"), ("sr", "Киркларели"), ("sr_Latn", "Kirklareli"), ("sv", "Kırklareli"), ("sw", "Mkoa wa Kırklareli"), ("ta", "கிரக\u{bcd}ளரேலி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3f}ర\u{c4d}క\u{c4d}\u{200c}ల\u{c3e}ర\u{c47}ల\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เขตมานาฟวา"), ("tk", "Kyrklareli"), ("tr", "Kırklareli"), ("uk", "Киркларелі"), ("ur", "قرقلرایلی صوبہ"), ("vi", "Kırklareli"), ("yue", "克爾克拉雷利省"), ("yue_Hans", "克尔克拉雷利省"), ("zh", "克尔克拉雷利省")]),
                        unofficial_name_list: ["Kirklareli"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::TR,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.146), longitude: Some(34.1607), max_latitude: Some(39.199712), min_latitude: Some(39.083829), max_longitude: Some(34.21962), min_longitude: Some(34.119774)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قيرشهر"), ("az", "Kırşəhər vilayəti"), ("be", "Правінцыя Кыршэхір"), ("bg", "Кършехир"), ("bn", "কিরিসিয\u{9bc}\u{9be}র প\u{9cd}রদেশ"), ("bs", "Kırşehir"), ("ca", "Província de Kırşehir"), ("ccp", "𑄇\u{11128}𑄢\u{11134}𑄥𑄬𑄦\u{11128}𑄢\u{11134}"), ("ceb", "Kırşehir"), ("cs", "Kırşehirská provincie"), ("da", "Kırşehir Province"), ("de", "Kırşehir"), ("el", "Επαρχία Κιρσεχίρ"), ("en", "Kırşehir"), ("es", "Provincia de Kırşehir"), ("et", "Kırşehiri provints"), ("eu", "Kırşehir probintzia"), ("fa", "استان قرشهر"), ("fi", "Kırşehirin maakunta"), ("fr", "Kırşehir"), ("gu", "કિર\u{acd}શ\u{ac7}હિર પ\u{acd}રા\u{a82}ત"), ("hi", "किरिसहिर प\u{94d}रा\u{902}त"), ("hu", "Kırşehir"), ("hy", "Քըրշեհիր"), ("id", "Provinsi Kırşehir"), ("it", "provincia di Kırşehir"), ("ja", "クルシェヒル県"), ("ka", "ქირშეჰირის პროვინცია"), ("kn", "ಕ\u{cbf}ರ\u{ccd}ಶೈರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키르셰히르 주"), ("lt", "Kiršehiro provincija"), ("lv", "Kiršehiras ils"), ("mk", "Кршехир"), ("mr", "किर\u{94d}श\u{947}हिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kırşehir"), ("nb", "Kırşehir"), ("nl", "Kırşehir"), ("no", "Kırşehir"), ("pl", "Kırşehir"), ("pt", "Kırşehir"), ("ro", "Provincia Kırșehir"), ("ru", "Кыршехир"), ("si", "කස\u{dd2}යර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Киршехир"), ("sr_Latn", "Kiršehir"), ("sv", "Kırşehir"), ("sw", "Mkoa wa Kırşehir"), ("ta", "கிரஸ\u{bc0}ஹிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3f}ర\u{c4d}స\u{c46}హ\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e36}ร\u{e4c}เชฮ\u{e35}ร\u{e4c}"), ("tk", "Kyrşehir"), ("tr", "Kırşehir"), ("uk", "Киршехір"), ("ur", "قر شہر صوبہ"), ("vi", "Kırşehir"), ("yue", "克爾謝希爾省"), ("yue_Hans", "克尔谢希尔省"), ("zh", "克尔谢希尔省")]),
                        unofficial_name_list: ["Kırşehir"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::TR,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.8532704), longitude: Some(29.8815203), max_latitude: Some(41.20976), min_latitude: Some(40.521065), max_longitude: Some(30.3677431), min_longitude: Some(29.3331263)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوكالي"), ("az", "Kocaeli vilayəti"), ("be", "Правінцыя Каджаэлі"), ("bg", "Коджаели"), ("bn", "কোক\u{9be}ইলি প\u{9cd}রদেশ"), ("bs", "Kocaeli"), ("ca", "Província de Kocaeli"), ("ccp", "𑄇\u{1112e}𑄥𑄠𑄬𑄣\u{11128}"), ("ceb", "Kocaeli"), ("cs", "Kocaeliská provincie"), ("da", "Kocaeli Provins"), ("de", "Kocaeli"), ("el", "Επαρχία Κοτζαελί"), ("en", "Kocaeli"), ("es", "Provincia de Kocaeli"), ("et", "Kocaeli provints"), ("eu", "Kocaeli probintzia"), ("fa", "استان قوجاایلی"), ("fi", "Kocaelin maakunta"), ("fr", "Kocaeli"), ("gu", "કોક\u{ac7}લી પ\u{acd}રા\u{a82}ત"), ("hi", "कोकाएली"), ("hu", "Kocaeli"), ("hy", "Կոջաելի"), ("id", "Provinsi Kocaeli"), ("it", "provincia di Kocaeli"), ("ja", "コジャエリ県"), ("ka", "ქოჯაელის პროვინცია"), ("kn", "ಕೋಕೇಲ\u{cbf} ಪ\u{ccd}ರಾ೦ತ\u{ccd}ಯ"), ("ko", "코자엘리 주"), ("lt", "Kodžaelis"), ("lv", "Kodžaeli ils"), ("mk", "Коџаели"), ("mr", "कोच\u{947}ली प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kocaeli"), ("nb", "Kocaeli"), ("nl", "Kocaeli"), ("no", "Kocaeli"), ("pl", "Kocaeli"), ("pt", "Kocaeli"), ("ro", "Provincia Kocaeli"), ("ru", "Коджаэли"), ("si", "කොක\u{dcf}එල\u{dd2} පළ\u{dcf}ත"), ("sl", "Kocaeli"), ("sq", "Koxhaeli"), ("sr", "Коџаели"), ("sr_Latn", "Kodžaeli"), ("sv", "Kocaeli"), ("sw", "Mkoa wa Kocaeli"), ("ta", "கோச\u{bcd}சேலி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}స\u{c47}ల\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคเจล\u{e35}"), ("tk", "Kojaeli"), ("tr", "Kocaeli"), ("uk", "Коджаелі"), ("ur", "قوجائلی صوبہ"), ("uz", "Kocaeli"), ("vi", "Kocaeli"), ("yue", "科賈埃利省"), ("yue_Hans", "科贾埃利省"), ("zh", "科贾埃利省")]),
                        unofficial_name_list: ["Kocaeli"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::TR,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.866667), longitude: Some(32.483333), max_latitude: Some(38.034311), min_latitude: Some(37.722702), max_longitude: Some(32.647959), min_longitude: Some(32.3650058)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قونية"), ("az", "Konya vilayəti"), ("be", "Правінцыя Конья"), ("bg", "Кония"), ("bn", "কন\u{9cd}য\u{9be} প\u{9cd}রদেশ"), ("bs", "Konya"), ("ca", "Província de Konya"), ("ccp", "𑄇\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Konya"), ("cs", "Konyanská provincie"), ("da", "Konya Province"), ("de", "Konya"), ("el", "Επαρχία Ικονίου"), ("en", "Konya"), ("es", "Provincia de Konya"), ("et", "Konya provints"), ("eu", "Konya probintzia"), ("fa", "استان قونیه"), ("fi", "Konyan maakunta"), ("fr", "Konya"), ("gu", "કોન\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "कोन\u{94d}या प\u{94d}रा\u{902}त"), ("hu", "Konya"), ("hy", "Կոնիա"), ("id", "Provinsi Konya"), ("it", "provincia di Konya"), ("ja", "コンヤ県"), ("ka", "კონიის პროვინცია"), ("kn", "ಕೊನ\u{ccd}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코니아 주"), ("lt", "Konjos provincija"), ("lv", "Konjas ils"), ("mk", "Конија"), ("mr", "कोन\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Konya"), ("nb", "Konya"), ("nl", "Konya"), ("no", "Konya"), ("pl", "Konya"), ("pt", "Konya (província)"), ("ro", "Provincia Konya"), ("ru", "Конья"), ("si", "කොන\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Konya"), ("sq", "Konya"), ("sr", "Конија"), ("sr_Latn", "Konija"), ("sv", "Konya"), ("sw", "Mkoa wa Konya"), ("ta", "கொன\u{bcd}ய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}న\u{c4d}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคอนย\u{e48}า"), ("tk", "Konýa"), ("tr", "Konya"), ("uk", "Конья"), ("ur", "قونیہ صوبہ"), ("uz", "Konya"), ("vi", "Konya"), ("yue", "科尼亞省"), ("yue_Hans", "科尼亚省"), ("zh", "科尼亚省")]),
                        unofficial_name_list: ["Konya"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::TR,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.4167), longitude: Some(29.9833), max_latitude: Some(39.448665), min_latitude: Some(39.383705), max_longitude: Some(30.053083), min_longitude: Some(29.9209691)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوتاهيا"), ("az", "Kütahya vilayəti"), ("bg", "Кютахия"), ("bn", "ক\u{9c1}ট\u{9be}হিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Kütahya"), ("ca", "Província de Kütahya"), ("ccp", "𑄇\u{1112a}𑄑𑄦\u{11128}𑄠"), ("ceb", "Kütahya (lalawigan)"), ("cs", "Kütahyanská provincie"), ("da", "Kütahya Province"), ("de", "Kütahya"), ("el", "Επαρχία Κιουτάχειας"), ("en", "Kütahya"), ("es", "Provincia de Kütahya"), ("et", "Kütahya provints"), ("eu", "Kutahya probintzia"), ("fa", "استان کوتاهیه"), ("fi", "Kütahyan maakunta"), ("fr", "Kütahya"), ("gu", "ક\u{ac1}તાહ\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{941}ताह\u{94d}या प\u{94d}रा\u{902}त"), ("hu", "Kütahya"), ("hy", "Քյութահիայի նահանգ"), ("id", "Provinsi Kütahya"), ("it", "provincia di Kütahya"), ("ja", "キュタヒヤ県"), ("ka", "ქუთაჰიის პროვინცია"), ("kn", "ಕುಟ\u{ccd}ಟಹ\u{ccd}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "퀴타히아 주"), ("lt", "Kutachjos provincija"), ("lv", "Kitahjas ils"), ("mk", "Ќутахија"), ("mr", "क\u{941}ताह\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kütahya"), ("nb", "Kütahya"), ("nl", "Kütahya"), ("no", "Kütahya"), ("pl", "Kütahya"), ("pt", "Kütahya"), ("ro", "Provincia Kütahya"), ("ru", "Кютахья"), ("si", "ක\u{dd4}ටය\u{dcf} පළ\u{dcf}ත"), ("sq", "Qytahia"), ("sr", "Китахија"), ("sr_Latn", "Kitahija"), ("sv", "Kütahya"), ("sw", "Mkoa wa Kütahya"), ("ta", "குட\u{bcd}டஹய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కుట\u{c3e}హ\u{c4d}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e36}ตทายา"), ("tk", "Kütahýa"), ("tr", "Kütahya"), ("uk", "Кютахʼя"), ("ur", "کوتاہیا صوبہ"), ("uz", "Kütahya (viloyat)"), ("vi", "Kütahya"), ("yue", "屈塔希亞省"), ("yue_Hans", "屈塔希亚省"), ("zh", "屈塔希亚省")]),
                        unofficial_name_list: ["Kütahya"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::TR,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.35), longitude: Some(38.3), max_latitude: Some(38.392921), min_latitude: Some(38.312895), max_longitude: Some(38.3802232), min_longitude: Some(38.1716921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ملطية"), ("az", "Malatya vilayəti"), ("bg", "Малатия (вилает)"), ("bn", "ম\u{9be}ল\u{9be}টিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Malatya"), ("ca", "Província de Malatya"), ("ccp", "𑄟𑄣𑄑\u{11128}𑄠"), ("ceb", "Malatya (lalawigan)"), ("cs", "Malatya"), ("cy", "Malatya"), ("da", "Malatya Province"), ("de", "Malatya"), ("el", "Επαρχία Μαλάτειας"), ("en", "Malatya"), ("es", "Provincia de Malatya"), ("et", "Malatya provints"), ("eu", "Malatya probintzia"), ("fa", "استان ملطیه"), ("fi", "Malatyan maakunta"), ("fr", "Malatya"), ("gu", "માલાત\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "मालात\u{94d}या प\u{94d}रा\u{902}त"), ("hu", "Malatya"), ("hy", "Մալաթիա"), ("id", "Provinsi Malatya"), ("it", "provincia di Malatya"), ("ja", "マラティヤ県"), ("ka", "მალათიის პროვინცია"), ("kn", "ಮಲಾತ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "말라티아 주"), ("lt", "Malatajos provincija"), ("lv", "Malatjas ils"), ("mk", "Малатија"), ("mr", "मलात\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Malatya"), ("nb", "Malatya"), ("nl", "Malatya"), ("no", "Malatya"), ("pl", "Malatya"), ("pt", "Malatya"), ("ro", "Provincia Malatya"), ("ru", "Малатья"), ("si", "මලට\u{dca}\u{200d}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Малатија"), ("sr_Latn", "Malatija"), ("sv", "Malatya"), ("sw", "Mkoa wa Malatya"), ("ta", "மலடிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మల\u{c3e}ట\u{c4d}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "มาล\u{e31}ตยา"), ("tk", "Malatýa"), ("tr", "Malatya"), ("uk", "Малатья"), ("ur", "مالاطیہ صوبہ"), ("vi", "Malatya"), ("yue", "馬拉蒂亞省"), ("yue_Hans", "马拉蒂亚省"), ("zh", "马拉蒂亚省")]),
                        unofficial_name_list: ["Malatya"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::TR,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.619099), longitude: Some(27.428921), max_latitude: Some(38.645072), min_latitude: Some(38.602441), max_longitude: Some(27.4787759), min_longitude: Some(27.3085369)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مانيسا"), ("az", "Manisa vilayəti"), ("be", "Правінцыя Маніса"), ("bg", "Маниса"), ("bn", "ম\u{9be}নিশ\u{9be} প\u{9cd}রদেশ"), ("bs", "Manisa"), ("ca", "Província de Manisa"), ("ccp", "𑄟𑄚\u{11128}𑄥"), ("ceb", "Manisa"), ("cs", "Manisa"), ("da", "Manisa Province"), ("de", "Manisa"), ("el", "Επαρχία Μανίσας"), ("en", "Manisa"), ("es", "Provincia de Manisa"), ("et", "Manisa provints"), ("eu", "Manisa probintzia"), ("fa", "استان مانیسا"), ("fi", "Manisan maakunta"), ("fr", "Manisa"), ("gu", "મનિસા પ\u{acd}રા\u{a82}ત"), ("hi", "मनीसा प\u{94d}रा\u{902}त"), ("hu", "Manisa"), ("hy", "Մանիսայի նահանգ"), ("id", "Provinsi Manisa"), ("it", "provincia di Manisa"), ("ja", "マニサ県"), ("ka", "მანისის პროვინცია"), ("kn", "ಮನ\u{cbf}ಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마니사 주"), ("lt", "Manisos provincija"), ("lv", "Manisas ils"), ("mk", "Маниса"), ("mr", "मनिसा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Manisa"), ("nb", "Manisa"), ("nl", "Manisa"), ("no", "Manisa"), ("pl", "Manisa"), ("pt", "Manisa"), ("ro", "Provincia Manisa"), ("ru", "Маниса"), ("si", "මන\u{dd2}ස\u{dcf} පළ\u{dcf}ත"), ("sl", "Manisa"), ("sr", "Маниса"), ("sr_Latn", "Manisa"), ("sv", "Manisa"), ("sw", "Mkoa wa Manisa"), ("ta", "மனிஷ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}న\u{c3f}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแมน\u{e34}สซา"), ("tk", "Manisa"), ("tr", "Manisa"), ("uk", "Маніса"), ("ur", "مانیسا صوبہ"), ("vi", "Manisa"), ("yue", "馬尼薩省"), ("yue_Hans", "马尼萨省"), ("zh", "马尼萨省")]),
                        unofficial_name_list: ["Manisa"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::TR,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.583333), longitude: Some(36.933333), max_latitude: Some(37.610065), min_latitude: Some(37.500703), max_longitude: Some(37.0086919), min_longitude: Some(36.8144921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مرعش"), ("az", "Qəhrəmanmaraş ili"), ("be", "Правінцыя Кахраманмараш"), ("bg", "Кахраманмараш"), ("bn", "ক\u{9be}হ\u{9cd}র\u{9be}মম\u{9be}নম\u{9be}র\u{9be}স প\u{9cd}রদেশ"), ("bs", "Kahramanmaraş"), ("ca", "Província de Kahramanmaraş"), ("ccp", "𑄇𑄦\u{11134}𑄢𑄟𑄚\u{11134}𑄟𑄢𑄌\u{11134}"), ("ceb", "Kahramanmaraş"), ("cs", "Kahramanmaraşská provincie"), ("cy", "Kahramanmaraş"), ("da", "Kahramanmaraş Province"), ("de", "Kahramanmaraş"), ("el", "Επαρχία Καχραμανμαράς"), ("en", "Kahramanmaraş"), ("es", "Provincia de Kahramanmaraş"), ("et", "Kahramanmaraşi provints"), ("eu", "Kahramanmaras probintzia"), ("fa", "استان قهرمان مرعش"), ("fi", "Kahramanmaraşin maakunta"), ("fr", "Kahramanmaraş"), ("gu", "કાહરામમરાસ પ\u{acd}રા\u{a82}ત"), ("hi", "कहारामनमारास प\u{94d}रा\u{902}त"), ("hu", "Kahramanmaraş"), ("hy", "Մարաշի նահանգ"), ("id", "Provinsi Kahramanmaraş"), ("it", "provincia di Kahramanmaraş"), ("ja", "カフラマンマラシュ県"), ("ka", "ქაჰრამანმარაშის პროვინცია"), ("kn", "ಕಹ\u{ccd}ರಾಮರಾಮಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카라만마라슈 주"), ("lt", "Kahramanmarašo provincija"), ("lv", "Kahramanmarašas ils"), ("mk", "Кахраманмараш"), ("mr", "काहरामानमराश प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kahramanmaraş"), ("nb", "Kahramanmaraş"), ("nl", "Kahramanmaraş"), ("no", "Kahramanmaraş"), ("pl", "Kahramanmaraş"), ("pt", "Kahramanmaraş"), ("ro", "Provincia Kahramanmaraș"), ("ru", "Кахраманмараш (ил)"), ("si", "කහරමන\u{dca}මර\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Кахраманмараш"), ("sr_Latn", "Kahramanmaraš"), ("sv", "Kahramanmaraş"), ("sw", "Mkoa wa Kahramanmaraş"), ("ta", "கஹரம\u{bbe}ன\u{bcd}ம\u{bbe}ர\u{bcd}ஸ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కహ\u{c3e}ర\u{c3e}మన\u{c4d}మ\u{c3e}రస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาห\u{e4c}รามานมาร\u{e31}ส"), ("tk", "Kahramanmaraş"), ("tr", "Kahramanmaraş"), ("uk", "Кахраманмараш"), ("ur", "قہرمان مرعش صوبہ"), ("uz", "Kahramanmaraş"), ("vi", "Kahramanmaraş"), ("yue", "卡赫拉曼馬拉什省"), ("yue_Hans", "卡赫拉曼马拉什省"), ("zh", "卡赫拉曼马拉什省")]),
                        unofficial_name_list: ["Kahramanmaraş", "Maraş"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::TR,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.3211634), longitude: Some(40.7244774), max_latitude: Some(37.340449), min_latitude: Some(37.300656), max_longitude: Some(40.7663849), min_longitude: Some(40.6910659)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ماردين"), ("az", "Mardin vilayəti"), ("be", "Правінцыя Мардзін"), ("bg", "Мардин"), ("bn", "ম\u{9be}র\u{9cd}দিন প\u{9cd}রদেশ"), ("bs", "Mardin"), ("ca", "Província de Mardin"), ("ccp", "𑄟𑄢\u{11134}𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Mardin"), ("cs", "Mardinská provincie"), ("cy", "Mardin"), ("da", "Mardin Province"), ("de", "Mardin"), ("el", "Επαρχία Μαρντίν"), ("en", "Mardin"), ("es", "Provincia de Mardin"), ("et", "Mardini provints"), ("eu", "Mardin probintzia"), ("fa", "استان ماردین"), ("fi", "Mardinin maakunta"), ("fr", "Mardin"), ("gu", "માર\u{acd}ડિન પ\u{acd}રા\u{a82}ત"), ("hi", "मार\u{94d}दिन प\u{94d}रा\u{902}त"), ("hu", "Mardin"), ("hy", "Մարդին"), ("id", "Provinsi Mardin"), ("it", "provincia di Mardin"), ("ja", "マルディン県"), ("ka", "მარდინის პროვინცია"), ("kn", "ಮಾರ\u{ccd}ಡ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마르딘 주"), ("lt", "Mardino provincija"), ("lv", "Mardinas ils"), ("mk", "Мардин"), ("mr", "मार\u{94d}दिन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mardin"), ("nb", "Mardin"), ("nl", "Mardin"), ("no", "Mardin"), ("pl", "Mardin"), ("pt", "Mardin"), ("ro", "Provincia Mardin"), ("ru", "ил Мардин"), ("si", "ම\u{dcf}ර\u{dca}ද\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sr", "Мардин"), ("sr_Latn", "Mardin"), ("sv", "Mardin"), ("sw", "Mkoa wa Mardin"), ("ta", "ம\u{bbe}ர\u{bcd}டின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ర\u{c4d}డ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาร\u{e4c}ด\u{e34}น"), ("tk", "Mardin"), ("tr", "Mardin"), ("uk", "Мардін"), ("ur", "ماردین صوبہ"), ("vi", "Mardin"), ("yue", "瑪律丁省"), ("yue_Hans", "玛律丁省"), ("zh", "马尔丁省")]),
                        unofficial_name_list: ["Mardin"].to_vec(),
                    }
                ),
                (
                    "48",
                    Subdivision{
                        name: "48",
                        country_alpha2: Alpha2::TR,
                        code: "48",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.216667), longitude: Some(28.366667), max_latitude: Some(37.225165), min_latitude: Some(37.195114), max_longitude: Some(28.3861861), min_longitude: Some(28.342408)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "موغلا"), ("az", "Muğla vilayəti"), ("be", "Правінцыя Мугла"), ("bg", "Мугла"), ("bn", "ম\u{9c1}গ\u{9cd}ল\u{9be} প\u{9cd}রদেশ"), ("bs", "Muğla"), ("ca", "Província de Muğla"), ("ccp", "𑄟\u{1112a}𑄇\u{11134}𑄣"), ("ceb", "Muğla"), ("cs", "Muğlanská provincie"), ("da", "Muğla Province"), ("de", "Muğla"), ("el", "Επαρχία Μούγλων"), ("en", "Muğla"), ("es", "Provincia de Muğla"), ("et", "Muğla provints"), ("eu", "Muğla probintzia"), ("fa", "استان موغله"), ("fi", "Muğlan maakunta"), ("fr", "Muğla"), ("gl", "Provincia de Muğla"), ("gu", "મ\u{ac1}ગલા પ\u{acd}રા\u{a82}ત"), ("hi", "म\u{941}गला प\u{94d}रा\u{902}त"), ("hu", "Muğla"), ("hy", "Մուղլայի նահանգ"), ("id", "Provinsi Muğla"), ("it", "provincia di Muğla"), ("ja", "ムーラ県"), ("ka", "მუღლის პროვინცია"), ("kn", "ಮುಗ\u{ccd}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "물라 주"), ("lt", "Muglos provincija"), ("lv", "Muglas ils"), ("mk", "Мугла"), ("mr", "म\u{941}ला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Muğla"), ("nb", "Muğla"), ("nl", "Muğla"), ("no", "Muğla"), ("pa", "ਮ\u{a41}ਗਲਾ ਸ\u{a42}ਬਾ"), ("pl", "Muğla"), ("pt", "Muğla"), ("ro", "Provincia Muğla"), ("ru", "Мугла"), ("si", "ම\u{dd4}ග\u{dca}ල\u{dcf} පළ\u{dcf}ත"), ("sl", "Muğla"), ("sq", "Mugla"), ("sr", "Мугла"), ("sr_Latn", "Mugla"), ("sv", "Muğla"), ("sw", "Mkoa wa Muğla"), ("ta", "முகில\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ముగ\u{c4d}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอ\u{e47}ง"), ("tk", "Mugla"), ("tr", "Muğla"), ("uk", "Мугла"), ("ur", "موغلا صوبہ"), ("vi", "Muğla"), ("yue", "穆拉省"), ("yue_Hans", "穆拉省"), ("zh", "穆拉省")]),
                        unofficial_name_list: ["Muğla"].to_vec(),
                    }
                ),
                (
                    "49",
                    Subdivision{
                        name: "49",
                        country_alpha2: Alpha2::TR,
                        code: "49",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.733333), longitude: Some(41.491111), max_latitude: Some(38.783503), min_latitude: Some(38.718225), max_longitude: Some(41.540625), min_longitude: Some(41.47230589999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "موش"), ("az", "Muş ili"), ("be", "Правінцыя Муш"), ("bg", "Муш"), ("bn", "মোস প\u{9cd}রদেশ"), ("bs", "Muş"), ("ca", "Província de Muş"), ("ccp", "𑄟\u{1112a}𑄌\u{11134}"), ("ceb", "Muş"), ("cs", "Muşská provincie"), ("da", "Muş Province"), ("de", "Muş"), ("el", "Μους"), ("en", "Muş"), ("es", "Provincia de Muş"), ("et", "Muşi provints"), ("eu", "Muş probintzia"), ("fa", "استان موش"), ("fi", "Muşin maakunta"), ("fr", "Muş"), ("gu", "મ\u{ac1}સ પ\u{acd}રા\u{a82}ત"), ("he", "מוש (נפה)"), ("hi", "म\u{942}स प\u{94d}रा\u{902}त"), ("hu", "Muş"), ("hy", "Մուշ"), ("id", "Provinsi Muş"), ("it", "provincia di Muş"), ("ja", "ムシュ県"), ("ka", "მუშის პროვინცია"), ("kk", "Муш"), ("kn", "ಮುಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "무슈 주"), ("lt", "Mušo provincija"), ("lv", "Mušas ils"), ("mk", "Муш"), ("mr", "म\u{941}श प\u{94d}रा\u{902}त"), ("ms", "Wilayah Muş"), ("nb", "Muş"), ("nl", "Muş"), ("no", "Muş"), ("pa", "ਮ\u{a41}ਸ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Muş"), ("pt", "Muş"), ("ro", "Provincia Muș"), ("ru", "Муш"), ("si", "මස\u{dca} පළ\u{dcf}ත"), ("sl", "Muş"), ("sr", "Муш"), ("sr_Latn", "Muš"), ("sv", "Muş"), ("sw", "Mkoa wa Muş"), ("ta", "முஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ముస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e38}ส"), ("tk", "Muş"), ("tr", "Muş"), ("uk", "Муш"), ("ur", "موش صوبہ"), ("vi", "Muş"), ("yue", "穆什省"), ("yue_Hans", "穆什省"), ("zh", "穆什省")]),
                        unofficial_name_list: ["Muş"].to_vec(),
                    }
                ),
                (
                    "50",
                    Subdivision{
                        name: "50",
                        country_alpha2: Alpha2::TR,
                        code: "50",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.62442), longitude: Some(34.7239689), max_latitude: Some(38.676487), min_latitude: Some(38.589047), max_longitude: Some(34.7477789), min_longitude: Some(34.6697521)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نوشهر"), ("az", "Nevşəhər ili"), ("be", "Правінцыя Неўшэхір"), ("bg", "Невшехир"), ("bn", "নেভসেহির প\u{9cd}রদেশ"), ("bs", "Nevşehir"), ("ca", "Província de Nevşehir"), ("ccp", "𑄚𑄬𑄛\u{11134}𑄥𑄬𑄦\u{11128}𑄢\u{11134}"), ("ceb", "Nevşehir"), ("cs", "Nevşehirská provincie"), ("da", "Nevşehir Province"), ("de", "Nevşehir"), ("el", "Επαρχία Νεβσεχίρ"), ("en", "Nevşehir"), ("es", "Provincia de Nevşehir"), ("et", "Nevşehiri provints"), ("eu", "Nevşehir probintzia"), ("fa", "استان نوشهر"), ("fi", "Nevşehirin maakunta"), ("fr", "Nevşehir"), ("gu", "ન\u{ac7}વ\u{ac7}સીર પ\u{acd}રા\u{a82}ત"), ("hi", "न\u{947}विसिर प\u{94d}रा\u{902}त"), ("hu", "Nevşehir"), ("hy", "Նևշեհիր"), ("id", "Provinsi Nevşehir"), ("it", "provincia di Nevşehir"), ("ja", "ネヴシェヒル県"), ("ka", "ნევშეჰირის პროვინცია"), ("kn", "ನ\u{cc6}ವ\u{ccd}ಶ\u{cbf}ಹೀರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "네브셰히르 주"), ("lt", "Nevšechiro provincija"), ("lv", "Nevšehiras ils"), ("mk", "Невшехир"), ("mr", "न\u{947}वश\u{947}हिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Nevşehir"), ("nb", "Nevşehir"), ("nl", "Nevşehir"), ("no", "Nevşehir"), ("pa", "ਨਵਸ\u{a47}ਹਰ"), ("pl", "Nevşehir"), ("pt", "Nevşehir"), ("ro", "Provincia Nevșehir"), ("ru", "Невшехир"), ("si", "නෙව\u{dca}සෙහ\u{dd2}ර\u{dca} පළ\u{dcf}ත"), ("sr", "Невшехир"), ("sr_Latn", "Nevšehir"), ("sv", "Nevşehir"), ("sw", "Mkoa wa Nevşehir"), ("ta", "நெவ\u{bcd}செஹிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c46}వ\u{c4d}స\u{c46}హ\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเนฟเซไฮร\u{e4c}"), ("tk", "Newşehir"), ("tr", "Nevşehir"), ("uk", "Невшехір"), ("ur", "نو شہر صوبہ"), ("uz", "Nevşehir"), ("vi", "Nevşehir"), ("yue", "內夫謝希爾省"), ("yue_Hans", "内夫谢希尔省"), ("zh", "内夫谢希尔省")]),
                        unofficial_name_list: ["Nevşehir"].to_vec(),
                    }
                ),
                (
                    "51",
                    Subdivision{
                        name: "51",
                        country_alpha2: Alpha2::TR,
                        code: "51",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.97646), longitude: Some(34.69376), max_latitude: Some(38.00588500000001), min_latitude: Some(37.93935), max_longitude: Some(34.7289149), min_longitude: Some(34.633795)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نيغدة"), ("az", "Niğdə ili"), ("be", "Правінцыя Нійдэ"), ("bg", "Нигде"), ("bn", "নিগদে প\u{9cd}রদেশ"), ("bs", "Niğde"), ("ca", "Província de Niğde"), ("ccp", "𑄚\u{11128}𑄇\u{11134}𑄓𑄬"), ("ceb", "Niğde"), ("cs", "Niğdská provincie"), ("da", "Niğde Province"), ("de", "Niğde"), ("el", "Επαρχία Νίγδης"), ("en", "Niğde"), ("es", "Provincia de Niğde"), ("et", "Niğde provints"), ("eu", "Niğde probintzia"), ("fa", "استان نیغده"), ("fi", "Niğden maakunta"), ("fr", "Niğde"), ("gu", "નીગ\u{acd}દ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "निगड\u{947} प\u{94d}रा\u{902}त"), ("hu", "Niğde"), ("hy", "Նիղդե"), ("id", "Provinsi Niğde"), ("it", "provincia di Niğde"), ("ja", "ニーデ県"), ("ka", "ნიგდეს პროვინცია"), ("kn", "ನ\u{cbf}ಗ\u{ccd}ಡ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "니데 주"), ("lt", "Nigdės provincija"), ("lv", "Nīgdes ils"), ("mk", "Нигде"), ("mr", "नीद\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Niğde"), ("nb", "Niğde"), ("nl", "Niğde"), ("no", "Niğde"), ("pl", "Niğde (prowincja)"), ("pt", "Niğde (província)"), ("ro", "Provincia Niğde"), ("ru", "Нигде"), ("si", "න\u{dd2}ග\u{dca}ඩේ පළ\u{dcf}ත"), ("sr", "Нигде"), ("sr_Latn", "Nigde"), ("sv", "Niğde"), ("sw", "Mkoa wa Niğde"), ("ta", "நிகிடே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c3f}గ\u{c4d}డ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e34}จเด"), ("tk", "Nigde"), ("tr", "Niğde"), ("uk", "Нігде"), ("ur", "نیغدے صوبہ"), ("vi", "Niğde"), ("yue", "尼代省"), ("yue_Hans", "尼代省"), ("zh", "尼代省")]),
                        unofficial_name_list: ["Niğde"].to_vec(),
                    }
                ),
                (
                    "52",
                    Subdivision{
                        name: "52",
                        country_alpha2: Alpha2::TR,
                        code: "52",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.98387899999999), longitude: Some(37.876411), max_latitude: Some(41.038519), min_latitude: Some(40.948541), max_longitude: Some(38.0206169), min_longitude: Some(37.7917351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أردو"), ("az", "Ordu ili"), ("be", "Правінцыя Арду"), ("bg", "Орду"), ("bn", "অর\u{9cd}দ\u{9c1} প\u{9cd}রদেশ"), ("bs", "Ordu"), ("ca", "Província d’Ordu"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄓\u{1112a}"), ("ceb", "Ordu"), ("cs", "Orduská provincie"), ("cy", "Ordu"), ("da", "Ordu Province"), ("de", "Ordu"), ("el", "Επαρχία Ορντού"), ("en", "Ordu"), ("es", "Provincia de Ordu"), ("et", "Ordu provints"), ("eu", "Ordu probintzia"), ("fa", "استان اردو"), ("fi", "Ordun maakunta"), ("fr", "Ordu"), ("gu", "ઓર\u{acd}ડ\u{ac1} પ\u{acd}રા\u{a82}ત"), ("hi", "ओर\u{94d}द\u{942} प\u{94d}रा\u{902}त"), ("hu", "Ordu"), ("hy", "Օրդու"), ("id", "Provinsi Ordu"), ("it", "provincia di Ordu"), ("ja", "オルドゥ県"), ("ka", "ორდუს პროვინცია"), ("kn", "ಆರ\u{ccd}ಡು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오르두 주"), ("lt", "Ordu provincija"), ("lv", "Ordu ils"), ("mk", "Орду"), ("mr", "ओर\u{94d}द\u{942} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ordu"), ("nb", "Ordu"), ("nl", "Ordu"), ("no", "Ordu"), ("pa", "ਓਰਦ\u{a42}"), ("pl", "Ordu"), ("pt", "Ordu"), ("ro", "Provincia Ordu"), ("ru", "Орду"), ("si", "ඔර\u{dca}ද\u{dd4} පළ\u{dcf}ත"), ("sr", "Орду"), ("sr_Latn", "Ordu"), ("sv", "Ordu"), ("sw", "Mkoa wa Ordu"), ("ta", "போர\u{bcd}டு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓర\u{c4d}డు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอด\u{e39}"), ("tk", "Ordu"), ("tr", "Ordu"), ("uk", "Орду"), ("ur", "صوبہ اردو"), ("uz", "Ordu"), ("vi", "Ordu"), ("yue", "奧爾杜省"), ("yue_Hans", "奥尔杜省"), ("zh", "奥尔杜省")]),
                        unofficial_name_list: ["Ordu"].to_vec(),
                    }
                ),
                (
                    "53",
                    Subdivision{
                        name: "53",
                        country_alpha2: Alpha2::TR,
                        code: "53",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.02005), longitude: Some(40.523449), max_latitude: Some(41.052146), min_latitude: Some(41.010782), max_longitude: Some(40.6146409), min_longitude: Some(40.4785778)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريزه"), ("az", "Rizə ili"), ("be", "Правінцыя Рызэ"), ("bg", "Ризе"), ("bn", "রিজ প\u{9cd}রদেশ"), ("bs", "Rize"), ("ca", "Província de Rize"), ("ccp", "𑄢\u{1112d}𑄌\u{11134}"), ("ceb", "Rize"), ("cs", "Rizenská provincie"), ("cy", "Rize"), ("da", "Rize Province"), ("de", "Rize"), ("el", "Επαρχία Ριζούντας"), ("en", "Rize"), ("es", "Provincia de Rize"), ("et", "Rize provints"), ("eu", "Rize probintzia"), ("fa", "استان ریزه"), ("fi", "Rizen maakunta"), ("fr", "Rize"), ("gu", "રાઇઝ પ\u{acd}રા\u{a82}ત"), ("hi", "रिज\u{93c}\u{947} प\u{94d}रा\u{902}त"), ("hu", "Rize"), ("hy", "Ռիզե"), ("id", "Provinsi Rize"), ("it", "provincia di Rize"), ("ja", "リゼ県"), ("ka", "რიზეს პროვინცია"), ("kn", "ರೈಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리제 주"), ("lt", "Rizės provincija"), ("lv", "Rizes ils"), ("mk", "Ризе"), ("mr", "रिझ\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Rize"), ("nb", "Rize"), ("nl", "Rize"), ("no", "Rize"), ("pl", "Prowincja Rize"), ("pt", "Rize"), ("ro", "Provincia Rize"), ("ru", "Ризе"), ("si", "ර\u{dd2}සේ පළ\u{dcf}ත"), ("sr", "Ризе"), ("sr_Latn", "Rize"), ("sv", "Rize"), ("sw", "Mkoa wa Rize"), ("ta", "ரிஸி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c48}జ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e34}เซ"), ("tk", "Rize"), ("tr", "Rize"), ("uk", "Різе"), ("ur", "ریزہ صوبہ"), ("uz", "Rize"), ("vi", "Rize"), ("yue", "裡澤省"), ("yue_Hans", "里泽省"), ("zh", "里泽省")]),
                        unofficial_name_list: ["Rize"].to_vec(),
                    }
                ),
                (
                    "54",
                    Subdivision{
                        name: "54",
                        country_alpha2: Alpha2::TR,
                        code: "54",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.783333), longitude: Some(30.4), max_latitude: Some(40.816299), min_latitude: Some(40.68175), max_longitude: Some(30.439832), min_longitude: Some(30.3247559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساكاريا"), ("az", "Sakarya vilayəti"), ("bg", "Сакария"), ("bn", "স\u{9be}\u{9cd}ক\u{9be}য\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Sakarya"), ("ca", "Província de Sakarya"), ("ccp", "𑄥𑄇𑄢\u{11128}𑄠"), ("ceb", "Sakarya (lalawigan sa Turkiya)"), ("cs", "Sakaryjská provincie"), ("da", "Sakarya Province"), ("de", "Sakarya"), ("el", "Επαρχία Σαγγάριου"), ("en", "Sakarya"), ("es", "Provincia de Sakarya"), ("et", "Sakarya provints"), ("eu", "Sakarya probintzia"), ("fa", "استان سقاریه"), ("fi", "Sakaryan maakunta"), ("fr", "Sakarya"), ("gl", "Provincia de Sakarya"), ("gu", "સાકર\u{acd}યા પ\u{acd}રા\u{a82}ત"), ("hi", "सकरया प\u{94d}रोवि\u{902}स"), ("hu", "Sakarya"), ("hy", "Սակարիա"), ("id", "Provinsi Sakarya"), ("it", "provincia di Sakarya"), ("ja", "サカリヤ県"), ("ka", "საქარიის პროვინცია"), ("kn", "ಸಕರ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사카리아 주"), ("lt", "Sakarajos provincija"), ("lv", "Sakarjas ils"), ("mk", "Сакарија"), ("mr", "सकार\u{94d}या प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sakarya"), ("nb", "Sakarya"), ("nl", "Sakarya"), ("no", "Sakarya"), ("pa", "ਸਕਾਰੀਆ"), ("pl", "Sakarya"), ("pt", "Sakarya"), ("ro", "Provincia Sakarya"), ("ru", "Сакарья"), ("si", "සකර\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Sakarya"), ("sr", "Сакарија"), ("sr_Latn", "Sakarija"), ("sv", "Sakarya"), ("sw", "Mkoa wa Sakarya"), ("ta", "சக\u{bcd}கரிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సక\u{c3e}ర\u{c4d}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาการ\u{e4c}ยะ"), ("tk", "Sakarýa"), ("tr", "Sakarya"), ("uk", "Сакарʼя"), ("ur", "ساکاریا صوبہ"), ("uz", "Sakarya"), ("vi", "Sakarya"), ("yue", "薩卡裡亞省"), ("yue_Hans", "萨卡里亚省"), ("zh", "萨卡里亚省")]),
                        unofficial_name_list: ["Sakarya"].to_vec(),
                    }
                ),
                (
                    "55",
                    Subdivision{
                        name: "55",
                        country_alpha2: Alpha2::TR,
                        code: "55",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.292782), longitude: Some(36.33128), max_latitude: Some(41.343598), min_latitude: Some(41.239349), max_longitude: Some(36.3819319), min_longitude: Some(36.2428711)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سامسون"), ("az", "Samsun ili"), ("be", "Правінцыя Самсун"), ("bg", "Самсун"), ("bs", "Samsun"), ("ca", "Província de Samsun"), ("ccp", "𑄥𑄟\u{11134}𑄥\u{1112a}𑄚\u{11134}"), ("ceb", "Samsun"), ("cs", "Samsunská provincie"), ("de", "Samsun"), ("el", "Επαρχία Σαμψούντας"), ("en", "Samsun"), ("es", "Provincia de Samsun"), ("et", "Samsuni provints"), ("eu", "Samsun probintzia"), ("fa", "استان سامسون"), ("fi", "Samsunin maakunta"), ("fr", "Samsun"), ("hu", "Samsun"), ("hy", "Սամսուն"), ("id", "Provinsi Samsun"), ("it", "provincia di Samsun"), ("ja", "サムスン県"), ("ka", "სამსუნის პროვინცია"), ("ko", "삼순 주"), ("lv", "Samsunas ils"), ("mk", "Самсун"), ("mr", "साम\u{94d}स\u{941}न प\u{94d}रा\u{902}त"), ("ms", "Wilayah Samsun"), ("nb", "Samsun"), ("nl", "Samsun"), ("no", "Samsun"), ("pa", "ਸਾਮਸ\u{a42}ਨ ਸ\u{a42}ਬਾ"), ("pl", "Samsun"), ("pt", "Samsun"), ("ro", "Provincia Samsun"), ("ru", "Самсун"), ("sl", "Samsun"), ("sr", "Самсун"), ("sr_Latn", "Samsun"), ("sv", "Samsun"), ("sw", "Mkoa wa Samsun"), ("tk", "Samsun"), ("tr", "Samsun"), ("uk", "Самсун"), ("ur", "سامسون صوبہ"), ("uz", "Samsun"), ("vi", "Samsun"), ("yue", "森松省"), ("yue_Hans", "森松省"), ("zh", "萨姆松省")]),
                        unofficial_name_list: ["Samsun"].to_vec(),
                    }
                ),
                (
                    "56",
                    Subdivision{
                        name: "56",
                        country_alpha2: Alpha2::TR,
                        code: "56",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.93554710000001), longitude: Some(41.9368268), max_latitude: Some(37.949366), min_latitude: Some(37.911382), max_longitude: Some(41.9638651), min_longitude: Some(41.9029088)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سعرد"), ("az", "Siirt ili"), ("be", "Правінцыя Сіірт"), ("bg", "Сиирт"), ("bn", "স\u{9be}\u{981}র\u{9cd}ত প\u{9cd}রদেশ"), ("bs", "Siirt"), ("ca", "Província de Siirt"), ("ccp", "𑄥\u{11129}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Siirt"), ("cs", "Siirtská provincie"), ("da", "Siirt Province"), ("de", "Siirt"), ("el", "Επαρχία Σιίρτ"), ("en", "Siirt"), ("es", "Provincia de Siirt"), ("et", "Siirti provints"), ("eu", "Siirt probintzia"), ("fa", "استان سعرد"), ("fi", "Siirtin maakunta"), ("fr", "Siirt"), ("gu", "સીર\u{acd}ત પ\u{acd}રા\u{a82}ત"), ("hi", "सिर\u{94d}ट प\u{94d}रा\u{902}त"), ("hu", "Siirt"), ("hy", "Սղերթ"), ("id", "Provinsi Siirt"), ("it", "provincia di Siirt"), ("ja", "スィイルト県"), ("ka", "სიირთის პროვინცია"), ("kn", "ಸೀಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시이르트 주"), ("lt", "Siirto provincija"), ("lv", "Siirtas ils"), ("mk", "Сирт"), ("mr", "सीर\u{94d}त प\u{94d}रा\u{902}त"), ("ms", "Wilayah Siirt"), ("nb", "Siirt"), ("nl", "Siirt"), ("no", "Siirt"), ("pl", "Siirt"), ("pt", "Siirt"), ("ro", "Provincia Siirt"), ("ru", "Сиирт"), ("si", "ස\u{dd2}ර\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sr", "Сирт"), ("sr_Latn", "Sirt"), ("sv", "Siirt"), ("sw", "Mkoa wa Siirt"), ("ta", "சிறிட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c40}ర\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซเอ\u{e35}\u{e49}ยท"), ("tk", "Siirt"), ("tr", "Siirt"), ("uk", "Сіїрт"), ("ur", "سیرت صوبہ"), ("uz", "Siirt"), ("vi", "Siirt"), ("yue", "錫爾特省"), ("yue_Hans", "锡尔特省"), ("zh", "锡尔特省")]),
                        unofficial_name_list: ["Siirt"].to_vec(),
                    }
                ),
                (
                    "57",
                    Subdivision{
                        name: "57",
                        country_alpha2: Alpha2::TR,
                        code: "57",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.02314), longitude: Some(35.153069), max_latitude: Some(42.034432), min_latitude: Some(41.993839), max_longitude: Some(35.1922838), min_longitude: Some(35.0675269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سينوب"), ("az", "Sinop vilayəti"), ("be", "Правінцыя Сіноп"), ("bg", "Синоп"), ("bn", "সিনোপ প\u{9cd}রদেশ"), ("bs", "Sinop"), ("ca", "Província de Sinop"), ("ccp", "𑄥\u{11128}𑄚\u{1112e}𑄛\u{11134}"), ("ceb", "Sinop"), ("cs", "Sinop (provincie)"), ("cy", "Sinop"), ("da", "Sinop Province"), ("de", "Sinop"), ("el", "Επαρχία Σινώπης"), ("en", "Sinop"), ("es", "Provincia de Sinope"), ("et", "Sinopi provints"), ("eu", "Sinop probintzia"), ("fa", "استان سینوپ"), ("fi", "Sinopin maakunta"), ("fr", "Sinop"), ("gu", "સિનોપ પ\u{acd}રા\u{a82}ત"), ("hi", "सिनोप"), ("hu", "Sinop"), ("hy", "Սինոպ"), ("id", "Provinsi Sinop"), ("it", "provincia di Sinope"), ("ja", "スィノプ県"), ("ka", "სინოპის პროვინცია"), ("kn", "ಸ\u{cbf}ನೋಪ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시노프 주"), ("lt", "Sinopo provincija"), ("lv", "Sinopas ils"), ("mk", "Синоп"), ("mr", "सिनोप प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sinop"), ("nb", "Sinop"), ("nl", "Sinop"), ("no", "Sinop"), ("pl", "Synopa"), ("pt", "Sinop"), ("ro", "Provincia Sinop"), ("ru", "Синоп"), ("si", "ස\u{dd2}නොප\u{dca} පළ\u{dcf}ත"), ("sr", "Синоп"), ("sr_Latn", "Sinop"), ("sv", "Sinop"), ("sw", "Mkoa wa Sinop"), ("ta", "சினோப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}న\u{c4b}ప\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e35}นอป"), ("tr", "Sinop"), ("uk", "Синоп"), ("ur", "سینوپ صوبہ"), ("vi", "Sinop"), ("yue", "錫諾普省"), ("yue_Hans", "锡诺普省"), ("zh", "锡诺普省")]),
                        unofficial_name_list: ["Sinop"].to_vec(),
                    }
                ),
                (
                    "58",
                    Subdivision{
                        name: "58",
                        country_alpha2: Alpha2::TR,
                        code: "58",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.747662), longitude: Some(37.017879), max_latitude: Some(39.781486), min_latitude: Some(39.715307), max_longitude: Some(37.0647389), min_longitude: Some(36.9270799)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيواس"), ("az", "Sivas ili"), ("be", "Правінцыя Сівас"), ("bg", "Сивас"), ("bn", "সিভ\u{9be}স প\u{9cd}রদেশ"), ("bs", "Sivas"), ("ca", "Província de Sivas"), ("ccp", "𑄥\u{11128}𑄞𑄌\u{11134}"), ("ceb", "Sivas"), ("cs", "Sivaská provincie"), ("cy", "Sivas"), ("da", "Sivas Province"), ("de", "Sivas"), ("el", "Επαρχία Σεβάστειας"), ("en", "Sivas"), ("es", "Provincia de Sivas"), ("et", "Sivasi provints"), ("eu", "Sivas probintzia"), ("fa", "استان سیواس"), ("fi", "Sivasin maakunta"), ("fr", "Sivas"), ("gl", "Provincia de Sivas"), ("gu", "સિવાસ પ\u{acd}રા\u{a82}ત"), ("hi", "सिवास प\u{94d}रा\u{902}त"), ("hu", "Sivas"), ("hy", "Սիվաս"), ("id", "Provinsi Sivas"), ("it", "provincia di Sivas"), ("ja", "スィヴァス県"), ("ka", "სივასის პროვინცია"), ("kn", "ಶ\u{cbf}ವಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시바스 주"), ("lt", "Sivaso provincija"), ("lv", "Sivasas ils"), ("mk", "Сивас"), ("mr", "सिवास प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sivas"), ("nb", "Sivas"), ("nl", "Sivas"), ("no", "Sivas"), ("pl", "Sivas"), ("pt", "Sivas"), ("ro", "Provincia Sivas"), ("ru", "Сивас"), ("si", "ස\u{dd2}ව\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Сивас"), ("sr_Latn", "Sivas"), ("sv", "Sivas"), ("sw", "Mkoa wa Sivas"), ("ta", "சிவ\u{bbe}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "శ\u{c3f}వ\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e34}วาส"), ("tk", "Siwas"), ("tr", "Sivas"), ("uk", "Сівас"), ("ur", "سیواس صوبہ"), ("uz", "Sivas"), ("vi", "Sivas"), ("yue", "錫瓦斯省"), ("yue_Hans", "锡瓦斯省"), ("zh", "錫瓦斯省")]),
                        unofficial_name_list: ["Sivas"].to_vec(),
                    }
                ),
                (
                    "59",
                    Subdivision{
                        name: "59",
                        country_alpha2: Alpha2::TR,
                        code: "59",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.977778), longitude: Some(27.515278), max_latitude: Some(40.99954400000001), min_latitude: Some(40.920071), max_longitude: Some(27.6462518), min_longitude: Some(27.469809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تيكيرداغ"), ("az", "Təkirdağ vilayəti"), ("be", "Правінцыя Тэкірдаг"), ("bg", "Родосто"), ("bn", "তিকরদ\u{9be}গ প\u{9cd}রদেশ"), ("bs", "Tekirdağ"), ("ca", "Província de Tekirdağ"), ("ccp", "𑄑𑄬𑄇\u{11128}𑄢\u{11134}𑄓𑄇\u{11134}"), ("ceb", "Tekirdağ"), ("cs", "Tekirdağská provincie"), ("da", "Tekirdağ Province"), ("de", "Tekirdağ"), ("el", "Επαρχία Ραιδεστού"), ("en", "Tekirdağ"), ("es", "Provincia de Tekirdağ"), ("et", "Tekirdaği provints"), ("eu", "Tekirdağ probintzia"), ("fa", "استان تکیرداغ"), ("fi", "Tekirdağin maakunta"), ("fr", "Tekirdağ"), ("gl", "Provincia de Tekirdağ"), ("gu", "ત\u{ac7}કિરદાગ પ\u{acd}રા\u{a82}ત"), ("hi", "ट\u{947}कीरड\u{948}ग प\u{94d}रा\u{902}त"), ("hu", "Tekirdağ"), ("hy", "Թեքիրդաղ"), ("id", "Provinsi Tekirdağ"), ("it", "provincia di Tekirdağ"), ("ja", "テキルダー県"), ("ka", "თექირდაღის პროვინცია"), ("kn", "ಟ\u{cc6}ಕ\u{cbf}ರ\u{ccd}ಡಾಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "테키르다 주"), ("lt", "Tekirdago provincija"), ("lv", "Tekirdāgas ils"), ("mk", "Текирдаг"), ("mr", "त\u{947}किर\u{94d}दा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tekirdağ"), ("nb", "Tekirdağ"), ("nl", "Tekirdağ"), ("no", "Tekirdağ"), ("pl", "Tekirdağ"), ("pt", "Tekirdağ"), ("ro", "Provincia Tekirdağ"), ("ru", "Текирдаг"), ("si", "ටෙක\u{dd2}ර\u{dca}ද\u{dcf}ග\u{dca} පළ\u{dcf}ත"), ("sl", "Tekirdağ"), ("sq", "Tekirdagu"), ("sr", "Текирдаг"), ("sr_Latn", "Tekirdag"), ("sv", "Tekirdağ"), ("sw", "Mkoa wa Tekirdağ"), ("ta", "தெகிரித\u{bbe}கி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c46}క\u{c3f}ర\u{c4d}డ\u{c3e}గ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเตเกอร\u{e4c}แดก"), ("tk", "Tekirdag"), ("tr", "Tekirdağ"), ("uk", "Текірдаг"), ("ur", "تکیرداغ صوبہ"), ("vi", "Tekirdağ"), ("yue", "泰基爾達省"), ("yue_Hans", "泰基尔达省"), ("zh", "泰基尔达省")]),
                        unofficial_name_list: ["Tekirdağ"].to_vec(),
                    }
                ),
                (
                    "60",
                    Subdivision{
                        name: "60",
                        country_alpha2: Alpha2::TR,
                        code: "60",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.313889), longitude: Some(36.554167), max_latitude: Some(40.358932), min_latitude: Some(40.267868), max_longitude: Some(36.5932192), min_longitude: Some(36.4696621)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "توقات"), ("az", "Toqat ili"), ("be", "Правінцыя Такат"), ("bg", "Токат"), ("bn", "টোক\u{9be}ট প\u{9cd}রদেশ"), ("bs", "Tokat"), ("ca", "Província de Tokat"), ("ccp", "𑄑\u{1112e}𑄇𑄖\u{11134}"), ("ceb", "Tokat"), ("cs", "Tokatská provincie"), ("cy", "Tokat"), ("da", "Tokat Province"), ("de", "Tokat"), ("el", "Επαρχία Τοκάτ"), ("en", "Tokat"), ("es", "Provincia de Tokat"), ("et", "Tokati provints"), ("eu", "Tokat probintzia"), ("fa", "استان توقات"), ("fi", "Tokatin maakunta"), ("fr", "Tokat"), ("gl", "Provincia de Tokat"), ("gu", "ટોકટ પ\u{acd}રા\u{a82}ત"), ("he", "טוקט"), ("hi", "टोक\u{948}त प\u{94d}रा\u{902}त"), ("hu", "Tokat"), ("hy", "Թոկաթ"), ("id", "Provinsi Tokat"), ("it", "provincia di Tokat"), ("ja", "トカト県"), ("ka", "თოქათის პროვინცია"), ("kn", "ಟೋಕ\u{ccd}ಯಾಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "토카트 주"), ("lt", "Tokato provincija"), ("lv", "Tokatas ils"), ("mk", "Токат"), ("mr", "तोकात प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tokat"), ("nb", "Tokat"), ("nl", "Tokat"), ("no", "Tokat"), ("pl", "Tokat"), ("pt", "Tokat"), ("ro", "Provincia Tokat"), ("ru", "Токат"), ("si", "ටොකට\u{dca} පළ\u{dcf}ත"), ("sr", "Токат"), ("sr_Latn", "Tokat"), ("sv", "Tokat"), ("sw", "Mkoa wa Tokat"), ("ta", "டோக\u{bbe}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4b}క\u{c3e}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโตก\u{e31}ต"), ("tk", "Tokat"), ("tr", "Tokat"), ("uk", "Токат"), ("ur", "توقات صوبہ"), ("uz", "Tokat"), ("vi", "Tokat"), ("yue", "托卡特省"), ("yue_Hans", "托卡特省"), ("zh", "托卡特省")]),
                        unofficial_name_list: ["Tokat"].to_vec(),
                    }
                ),
                (
                    "61",
                    Subdivision{
                        name: "61",
                        country_alpha2: Alpha2::TR,
                        code: "61",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.00145), longitude: Some(39.7177999), max_latitude: Some(41.01237), min_latitude: Some(40.973513), max_longitude: Some(39.808463), min_longitude: Some(39.6524071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "طرابزون"), ("az", "Trabzon ili"), ("be", "Правінцыя Трабзон"), ("bg", "Трабзон"), ("bn", "ট\u{9cd}র\u{9be}ব\u{9cd}জন প\u{9cd}রদেশ"), ("bs", "Trabzon"), ("ca", "Província de Trabzon"), ("ccp", "𑄑\u{11133}𑄢𑄛\u{11134}𑄎\u{1112e}𑄚\u{11134}"), ("ceb", "Trabzon"), ("cs", "Trabzonská provincie"), ("cy", "Trabzon"), ("da", "Trabzon Province"), ("de", "Trabzon"), ("el", "Επαρχία Τραπεζούντας"), ("en", "Trabzon"), ("es", "Provincia de Trabzon"), ("et", "Trabzoni provints"), ("eu", "Trabzon probintzia"), ("fa", "استان ترابزون"), ("fi", "Trabzonin maakunta"), ("fr", "Trabzon"), ("gl", "Provincia de Trebizonda"), ("gu", "ટ\u{acd}ર\u{ac7}બઝોન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז טרבזון"), ("hi", "ट\u{94d}र\u{948}ब\u{94d}जन प\u{94d}रा\u{902}त"), ("hu", "Trabzon"), ("hy", "Տրապիզոն"), ("id", "Provinsi Trabzon"), ("it", "provincia di Trebisonda"), ("ja", "トラブゾン県"), ("ka", "ტრაპიზონის პროვინცია"), ("kn", "ಟ\u{ccd}ರಾಬ\u{ccd}ಜಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "트라브존 주"), ("lt", "Trabzono provincija"), ("lv", "Trabzonas ils"), ("mk", "Трабзон"), ("mr", "त\u{94d}राब\u{94d}झोन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Trabzon"), ("nb", "Trabzon"), ("nl", "Trabzon"), ("no", "Trabzon"), ("pl", "Trabzon"), ("pt", "Trebizonda"), ("ro", "Provincia Trabzon"), ("ru", "Трабзон"), ("si", "ට\u{dca}\u{200d}රබ\u{dca}සන\u{dca} පළ\u{dcf}ත"), ("sl", "Trabzon"), ("sq", "Trabzon"), ("sr", "Трабзон"), ("sr_Latn", "Trabzon"), ("sv", "Trabzon"), ("sw", "Mkoa wa Trabzon"), ("ta", "ட\u{bcd}ர\u{bbe}ப\u{bcd}ஸ\u{bcd}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e}బ\u{c4d}జ\u{c4b}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแทรปซอน"), ("tk", "Trabzon"), ("tr", "Trabzon"), ("uk", "Трабзон"), ("ur", "ترابزون صوبہ"), ("uz", "Trabzon"), ("vi", "Trabzon"), ("yue", "特拉布宗省"), ("yue_Hans", "特拉布宗省"), ("zh", "特拉布宗省")]),
                        unofficial_name_list: ["Trabzon"].to_vec(),
                    }
                ),
                (
                    "62",
                    Subdivision{
                        name: "62",
                        country_alpha2: Alpha2::TR,
                        code: "62",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.1058251), longitude: Some(39.5455166), max_latitude: Some(39.118647), min_latitude: Some(39.067998), max_longitude: Some(39.5571623), min_longitude: Some(39.5122501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تونجيلي"), ("az", "Tuncəli ili"), ("be", "Правінцыя Тунджэлі"), ("bg", "Тунджели"), ("bn", "ট\u{9c1}নসেলি প\u{9cd}রদেশ"), ("bs", "Tunceli"), ("ca", "Província de Tunceli"), ("ccp", "𑄑\u{1112a}𑄚\u{11134}𑄥𑄬𑄣\u{11128}"), ("ceb", "Tunceli"), ("cs", "Tuncelská provincie"), ("da", "Tunceli Province"), ("de", "Tunceli"), ("el", "Τούνκελι"), ("en", "Tunceli"), ("es", "Provincia de Tunceli"), ("et", "Tunceli provints"), ("eu", "Tunceli probintzia"), ("fa", "استان تونج\u{200c}ایلی"), ("fi", "Tuncelin maakunta"), ("fr", "Tunceli"), ("gl", "Provincia de Tunceli"), ("gu", "ટ\u{ac1}\u{a82}સ\u{ac7}લી પ\u{acd}રા\u{a82}ત"), ("hi", "ट\u{94d}य\u{942}नस\u{947}ली प\u{94d}रा\u{902}त"), ("hu", "Tunceli"), ("hy", "Թունջելի"), ("id", "Provinsi Tunceli"), ("it", "provincia di Tunceli"), ("ja", "トゥンジェリ県"), ("ka", "თუნჯელის პროვინცია"), ("kn", "ಟುನ\u{ccd}ಸ\u{cc6}ಲ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "툰젤리 주"), ("lt", "Tundželio provincija"), ("lv", "Tundželi ils"), ("mk", "Тунџели"), ("mr", "त\u{941}\u{902}ज\u{947}ली प\u{94d}रा\u{902}त"), ("ms", "Wilayah Tunceli"), ("nb", "Tunceli"), ("nl", "Tunceli"), ("no", "Tunceli"), ("pl", "Tunceli"), ("pt", "Tunceli"), ("ro", "Provincia Tunceli"), ("ru", "Тунджели"), ("si", "ට\u{dd4}න\u{dca}සෙල\u{dd2} පළ\u{dcf}ත"), ("sr", "Тунџели"), ("sr_Latn", "Tundželi"), ("sv", "Tunceli"), ("sw", "Mkoa wa Tunceli"), ("ta", "த\u{bbe}ன\u{bcd}செலி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టున\u{c4d}స\u{c46}ల\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e31}นเชล\u{e35}\u{e48}"), ("tk", "Tunjeli"), ("tr", "Tunceli"), ("uk", "Тунджелі"), ("ur", "تونجیلی صوبہ"), ("uz", "Tunceli"), ("vi", "Tunceli"), ("yue", "通傑利省"), ("yue_Hans", "通杰利省"), ("zh", "通杰利省")]),
                        unofficial_name_list: ["Tunceli"].to_vec(),
                    }
                ),
                (
                    "63",
                    Subdivision{
                        name: "63",
                        country_alpha2: Alpha2::TR,
                        code: "63",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.159149), longitude: Some(38.796909), max_latitude: Some(37.233371), min_latitude: Some(37.100904), max_longitude: Some(38.891956), min_longitude: Some(38.7599281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أورفة"), ("az", "Şanlıurfa ili"), ("be", "Правінцыя Шанлыўрфа"), ("bg", "Шанлъурфа (вилает)"), ("bn", "স\u{9cd}য\u{9be}নল\u{9c1}রফ\u{9be} প\u{9cd}রদেশ"), ("bs", "Şanlıurfa (provincija)"), ("ca", "Província de Şanlıurfa"), ("ccp", "𑄥𑄚\u{11134}𑄣\u{11128}𑄅\u{1112a}𑄢\u{11134}𑄜"), ("ceb", "Şanlıurfa"), ("cs", "Şanlıurfská provincie"), ("da", "Şanlıurfa Province"), ("de", "Şanlıurfa"), ("el", "Επαρχία Σανλιούρφα"), ("en", "Şanlıurfa"), ("es", "Provincia de Sanliurfa"), ("et", "Şanlıurfa provints"), ("eu", "Şanlıurfa probintzia"), ("fa", "استان شانلی\u{200c}اورفه"), ("fi", "Şanlıurfan maakunta"), ("fr", "Şanlıurfa"), ("gu", "સ\u{ac7}ન\u{acd}લ\u{ac1}ર\u{acd}ફા પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}न\u{94d}ल\u{94d}य\u{941}र\u{94d}फा"), ("hu", "Şanlıurfa (tartomány)"), ("hy", "Ուռհայի նահանգ"), ("id", "Provinsi Şanlıurfa"), ("it", "provincia di Şanlıurfa"), ("ja", "シャンルウルファ県"), ("ka", "ურფის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd}ಲ\u{cbf}ರ\u{ccd}ಫಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "샨리우르파 주"), ("lt", "Šanliurfos provincija"), ("lv", "Šanliurfas ils"), ("mk", "Шанлурфа (покраина)"), ("mr", "शानल\u{941}र\u{94d}फा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Şanlıurfa"), ("nb", "Şanlıurfa"), ("nl", "Şanlıurfa"), ("no", "Şanlıurfa"), ("pl", "Şanlıurfa (prowincja)"), ("pt", "Şanlıurfa (província)"), ("ro", "Provincia Șanlıurfa"), ("ru", "Шанлыурфа"), ("si", "සැන\u{dca}ල\u{dd2}ය\u{dd4}ර\u{dca}ෆ\u{dcf} පළ\u{dcf}ත"), ("sq", "Shanllëurfa"), ("sr", "Шанлијурфа"), ("sr_Latn", "Šanlijurfa"), ("sv", "Şanlıurfa (provins)"), ("sw", "Mkoa wa Şanlıurfa"), ("ta", "ஆன\u{bcd}லைனுரபிஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}న\u{c4d}లుర\u{c4d}ఫ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐบาเด\u{e34}น-เว\u{e37}อร\u{e4c}ทเท\u{e34}มแบร\u{e4c}ค"), ("tk", "Şanlyurfa (il)"), ("tr", "Şanlıurfa"), ("uk", "Шанлиурфа (іл)"), ("ur", "شانلیعرفا صوبہ"), ("uz", "Şanlıurfa (viloyat)"), ("vi", "Şanlıurfa (tỉnh)"), ("yue", "尚勒烏爾法省"), ("yue_Hans", "尚勒乌尔法省"), ("zh", "尚勒乌尔法省")]),
                        unofficial_name_list: ["Şanlıurfa"].to_vec(),
                    }
                ),
                (
                    "64",
                    Subdivision{
                        name: "64",
                        country_alpha2: Alpha2::TR,
                        code: "64",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.682301), longitude: Some(29.40818999999999), max_latitude: Some(38.705018), min_latitude: Some(38.633352), max_longitude: Some(29.456464), min_longitude: Some(29.3559361)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوشاك"), ("az", "Uşaq ili"), ("be", "Правінцыя Ушак"), ("bg", "Ушак"), ("bn", "উস\u{9be}ক প\u{9cd}রদেশ"), ("bs", "Uşak"), ("ca", "Província d’Uşak"), ("ccp", "𑄅\u{1112a}𑄥𑄇\u{11134}"), ("ceb", "Uşak (lalawigan)"), ("cs", "Uşacká provincie"), ("da", "Uşak Province"), ("de", "Uşak"), ("el", "Επαρχία Ουσάκ"), ("en", "Uşak"), ("es", "Provincia de Uşak"), ("et", "Uşaki provints"), ("eu", "Usak probintzia"), ("fa", "استان عشاق"), ("fi", "Uşakin maakunta"), ("fr", "Uşak"), ("gl", "Provincia de Uşak"), ("gu", "ઉસક પ\u{acd}રા\u{a82}ત"), ("hi", "उसाक प\u{94d}रा\u{902}त"), ("hu", "Uşak"), ("hy", "Ուշաքի նահանգ"), ("id", "Provinsi Uşak"), ("it", "provincia di Uşak"), ("ja", "ウシャク県"), ("ka", "უშაქის პროვინცია"), ("kn", "ಯುಸಾಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우샤크 주"), ("lt", "Ušako provincija"), ("lv", "Ušakas ils"), ("mk", "Ушак"), ("mr", "उशाक प\u{94d}रा\u{902}त"), ("ms", "Wilayah Uşak"), ("nb", "Uşak"), ("nl", "Uşak"), ("no", "Uşak"), ("pl", "Uşak"), ("pt", "Uşak"), ("ro", "Provincia Ușak"), ("ru", "Ушак"), ("si", "උස\u{dcf}ක\u{dca} පළ\u{dcf}ත"), ("sr", "Ушак"), ("sr_Latn", "Ušak"), ("sv", "Uşak"), ("sw", "Mkoa wa Uşak"), ("ta", "உசக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉస\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e39}ซ\u{e31}ค"), ("tk", "Uşak"), ("tr", "Uşak"), ("uk", "Ушак"), ("ur", "عشاق صوبہ"), ("uz", "Uşak"), ("vi", "Uşak"), ("yue", "烏沙克省"), ("yue_Hans", "乌沙克省"), ("zh", "乌沙克省")]),
                        unofficial_name_list: ["Uşak"].to_vec(),
                    }
                ),
                (
                    "65",
                    Subdivision{
                        name: "65",
                        country_alpha2: Alpha2::TR,
                        code: "65",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.494167), longitude: Some(43.38), max_latitude: Some(38.580314), min_latitude: Some(38.425492), max_longitude: Some(43.447585), min_longitude: Some(43.2695239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وان"), ("az", "Van vilayəti"), ("be", "Правінцыя Ван"), ("bg", "Ван"), ("bn", "ভয\u{9bc}\u{9be}ন ভ\u{9cd}য\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Van"), ("ca", "Província de Van"), ("ccp", "𑄞\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Van (lalawigan)"), ("cs", "Vanská provincie"), ("cy", "Van"), ("da", "Van Province"), ("de", "Van"), ("el", "Βαν"), ("en", "Van"), ("es", "Provincia de Van"), ("et", "Vani provints"), ("eu", "Van probintzia"), ("fa", "استان وان"), ("fi", "Vanin maakunta"), ("fr", "Van"), ("gl", "Provincia de Van"), ("gu", "વ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "ואן"), ("hi", "वान प\u{94d}रा\u{902}त"), ("hu", "Van"), ("hy", "Վան"), ("id", "Provinsi Van"), ("it", "provincia di Van"), ("ja", "ヴァン県"), ("ka", "ვანის პროვინცია"), ("kn", "ವ\u{ccd}ಯಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "반 주"), ("lt", "Vano provincija"), ("lv", "Vanas ils"), ("mk", "Ван"), ("mr", "वान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Van"), ("nb", "Van"), ("nl", "Van"), ("no", "Van"), ("pa", "ਵਾਨ"), ("pl", "Wan"), ("pt", "Van"), ("ro", "Provincia Van"), ("ru", "Ван"), ("si", "වැන\u{dca} පළ\u{dcf}ත"), ("sl", "Van"), ("sr", "Ван"), ("sr_Latn", "Van"), ("sv", "Van"), ("sw", "Mkoa wa Van"), ("ta", "வ\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3e}న\u{c4d}"), ("th", "มณฑลวาน"), ("tk", "Wan"), ("tr", "Van"), ("uk", "Ван"), ("ur", "وان صوبہ"), ("vi", "Van"), ("yue", "凡城省"), ("yue_Hans", "凡城省"), ("zh", "凡城省")]),
                        unofficial_name_list: ["Van"].to_vec(),
                    }
                ),
                (
                    "66",
                    Subdivision{
                        name: "66",
                        country_alpha2: Alpha2::TR,
                        code: "66",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.818081), longitude: Some(34.81469), max_latitude: Some(39.837859), min_latitude: Some(39.792581), max_longitude: Some(34.8495329), min_longitude: Some(34.7747827)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "يوزغات"), ("az", "Yozqat vilayəti"), ("be", "Правінцыя Ёзгат"), ("bg", "Йозгат"), ("bn", "ইয\u{9bc}জগ\u{9be}ট প\u{9cd}রদেশ"), ("bs", "Yozgat"), ("ca", "Província de Yozgat"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄉𑄖\u{11134}"), ("ceb", "Yozgat"), ("cs", "Yozgatská provincie"), ("da", "Yozgat Province"), ("de", "Yozgat"), ("el", "Επαρχία Γιοζγκάτ"), ("en", "Yozgat"), ("es", "Provincia de Yozgat"), ("et", "Yozgati provints"), ("eu", "Yozgat probintzia"), ("fa", "استان یوزگات"), ("fi", "Yozgatin maakunta"), ("fr", "Yozgat"), ("gl", "Provincia de Yozgat"), ("gu", "યોઝગાત પ\u{acd}રા\u{a82}ત"), ("hi", "योज\u{93c}\u{94d}गट प\u{94d}रा\u{902}त"), ("hu", "Yozgat"), ("hy", "Յոզգաթի նահանգ"), ("id", "Provinsi Yozgat"), ("it", "provincia di Yozgat"), ("ja", "ヨズガト県"), ("ka", "იოზგათის პროვინცია"), ("kk", "Йозгат"), ("kn", "ಯೊಜ\u{ccd}ಗತ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "요즈가트 주"), ("lt", "Jozgato provincija"), ("lv", "Jozgatas ils"), ("mk", "Јозгат"), ("mr", "योझ\u{94d}गात प\u{94d}रा\u{902}त"), ("ms", "Wilayah Yozgat"), ("nb", "Yozgat"), ("nl", "Yozgat"), ("no", "Yozgat"), ("pl", "Yozgat"), ("pt", "Yozgat"), ("ro", "Provincia Yozgat"), ("ru", "Йозгат"), ("si", "යෝස\u{dca}ගට\u{dca} පළ\u{dcf}ත"), ("sr", "Јозгат"), ("sr_Latn", "Jozgat"), ("sv", "Yozgat"), ("sw", "Mkoa wa Yozgat"), ("ta", "யோஸ\u{bcd}கட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "య\u{c4b}జ\u{c4d}గ\u{c3e}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดยอซแกท"), ("tk", "Ýozgat"), ("tr", "Yozgat"), ("uk", "Йозгат"), ("ur", "یوزگت علاقہ"), ("uz", "Yozgat"), ("vi", "Yozgat"), ("yue", "約茲加特省"), ("yue_Hans", "约兹加特省"), ("zh", "约兹加特省")]),
                        unofficial_name_list: ["Yozgat"].to_vec(),
                    }
                ),
                (
                    "67",
                    Subdivision{
                        name: "67",
                        country_alpha2: Alpha2::TR,
                        code: "67",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.456409), longitude: Some(31.798731), max_latitude: Some(41.495178), min_latitude: Some(41.42287), max_longitude: Some(31.8742409), min_longitude: Some(31.72608)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زانغولداك"), ("az", "Zonquldak ili"), ("be", "Правінцыя Зангулдак"), ("bg", "Зонгулдак (вилает)"), ("bn", "জঙ\u{9cd}গ\u{9c1}লড\u{9be}ক প\u{9cd}রদেশ"), ("bs", "Zonguldak (provincija)"), ("ca", "Província de Zonguldak"), ("ccp", "𑄎\u{11127}\u{11101}𑄉\u{1112a}𑄣\u{11134}𑄓𑄇\u{11134}"), ("ceb", "Zonguldak"), ("cs", "Zonguldakská provincie"), ("cy", "Zonguldak"), ("da", "Zonguldak Province"), ("de", "Zonguldak"), ("el", "Επαρχία Ζονγκουλντάκ"), ("en", "Zonguldak"), ("es", "Provincia de Zonguldak"), ("et", "Zonguldaki provints"), ("eu", "Zonguldak probintzia"), ("fa", "استان زونگولداغ"), ("fi", "Zonguldakin maakunta"), ("fr", "Zonguldak (province)"), ("gl", "Provincia de Zonguldak"), ("gu", "ઝો\u{a82}ગ\u{ac1}લડક પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{93c}ो\u{902}ग\u{941}लदक प\u{94d}रा\u{902}त"), ("hu", "Zonguldak (tartomány)"), ("id", "Provinsi Zonguldak"), ("it", "provincia di Zonguldak"), ("ja", "ゾングルダク県"), ("ka", "ზონგულდაქის პროვინცია"), ("kn", "ಝೊಂಗುಲ\u{ccd}ಡಾಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "종굴다크 주"), ("lt", "Zonguldako provincija"), ("lv", "Zonguldakas ils"), ("mk", "Зонгулдак (покраина)"), ("mr", "झो\u{902}ग\u{941}ल\u{94d}दाक प\u{94d}रा\u{902}त"), ("ms", "Wilayah Zonguldak"), ("nb", "Zonguldak"), ("nl", "Zonguldak"), ("no", "Zonguldak"), ("pa", "ਜ\u{a4b}\u{a02}ਗ\u{a41}ਲਡਕ"), ("pl", "Zonguldak (prowincja)"), ("pt", "Zonguldak (província)"), ("ro", "Provincia Zonguldak"), ("ru", "Зонгулдак (ил)"), ("si", "සොන\u{dca}ග\u{dd4}ල\u{dca}ඩක\u{dca} පළ\u{dcf}ත"), ("sr", "Зонгулдак"), ("sr_Latn", "Zonguldak"), ("sv", "Zonguldak (provins)"), ("sw", "Mkoa wa Zonguldak"), ("ta", "ஸ\u{bcd}ஓங\u{bcd}குலட\u{bbe}கி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c4b}ంగుల\u{c4d}డ\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซองก\u{e38}ลด\u{e31}ค"), ("tk", "Zonguldak (il)"), ("tr", "Zonguldak (il)"), ("uk", "Зонгулдак (іл)"), ("ur", "زانگولداک صوبہ"), ("uz", "Zonguldak (viloyat)"), ("vi", "Zonguldak (tỉnh)"), ("yue", "宗古爾達克省"), ("yue_Hans", "宗古尔达克省"), ("zh", "宗古尔达克省")]),
                        unofficial_name_list: ["Zonguldak"].to_vec(),
                    }
                ),
                (
                    "68",
                    Subdivision{
                        name: "68",
                        country_alpha2: Alpha2::TR,
                        code: "68",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.36869), longitude: Some(34.03698), max_latitude: Some(38.428766), min_latitude: Some(38.292834), max_longitude: Some(34.0744111), min_longitude: Some(33.9268219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آق سراي"), ("az", "Ağsaray vilayəti"), ("be", "Правінцыя Аксарай"), ("bg", "Аксарай"), ("bn", "আকস\u{9be}র প\u{9cd}রদেশ"), ("bs", "Aksaray"), ("ca", "Província d’Aksaray"), ("ccp", "𑄃𑄇\u{11134}𑄥𑄢𑄬"), ("ceb", "Aksaray"), ("cs", "Aksarayská provincie"), ("da", "Aksaray Province"), ("de", "Aksaray"), ("el", "Επαρχία Ακσαράι"), ("en", "Aksaray"), ("es", "Provincia de Aksaray"), ("et", "Aksaray provints"), ("eu", "Aksaray probintzia"), ("fa", "استان آق\u{200c}سرای"), ("fi", "Aksarayn maakunta"), ("fr", "Aksaray"), ("gl", "Provincia de Aksaray"), ("gu", "અક\u{acd}સારાય પ\u{acd}રા\u{a82}ત"), ("hi", "अकसरय प\u{94d}रा\u{902}त"), ("hu", "Aksaray"), ("hy", "Ակսարայ"), ("id", "Provinsi Aksaray"), ("it", "provincia di Aksaray"), ("ja", "アクサライ県"), ("ka", "აქსარაის პროვინცია"), ("kn", "ಅಕ\u{ccd}ಸರಾಯ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "악사라이 주"), ("lt", "Aksarajaus provincija"), ("lv", "Aksarajas ils"), ("mk", "Аксарај"), ("mr", "अक\u{94d}साराय प\u{94d}रा\u{902}त"), ("ms", "Wilayah Aksaray"), ("nb", "Aksaray"), ("nl", "Aksaray"), ("no", "Aksaray"), ("pl", "Aksaray"), ("pt", "Aksaray"), ("ro", "Provincia Aksaray"), ("ru", "Аксарай"), ("si", "අක\u{dca}සරේ පළ\u{dcf}ත"), ("sq", "Aksaray"), ("sr", "Аксарај"), ("sr_Latn", "Aksaraj"), ("sv", "Aksaray"), ("sw", "Mkoa wa Aksaray"), ("ta", "அக\u{bcd}ஸர\u{bbe}ய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అక\u{c4d}స\u{c3e}ర\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}กซาไร"), ("tk", "Aksaraý"), ("tr", "Aksaray"), ("uk", "Аксарай"), ("ur", "آق سرائے صوبہ"), ("uz", "Aksaray"), ("vi", "Aksaray"), ("yue", "阿克薩賴省"), ("yue_Hans", "阿克萨赖省"), ("zh", "阿克萨赖省")]),
                        unofficial_name_list: ["Aksaray"].to_vec(),
                    }
                ),
                (
                    "69",
                    Subdivision{
                        name: "69",
                        country_alpha2: Alpha2::TR,
                        code: "69",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.255169), longitude: Some(40.22488), max_latitude: Some(40.282803), min_latitude: Some(40.235426), max_longitude: Some(40.259813), min_longitude: Some(40.19289300000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بايبورت"), ("az", "Bayburt ili"), ("be", "Правінцыя Байбурт"), ("bg", "Байбурт"), ("bn", "বেবোর\u{9cd}ট প\u{9cd}রদেশ"), ("bs", "Bayburt"), ("ca", "Província de Bayburt"), ("ccp", "𑄝𑄬𑄝\u{1112a}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Bayburt (lalawigan)"), ("cs", "Bayburtská provincie"), ("cy", "Bayburt"), ("da", "Bayburt Province"), ("de", "Bayburt"), ("el", "Επαρχία Μπαϊμπούρτ"), ("en", "Bayburt"), ("es", "Provincia de Bayburt"), ("et", "Bayburti provints"), ("eu", "Bayburt probintzia"), ("fa", "استان بایبورت"), ("fi", "Bayburtin maakunta"), ("fr", "Bayburt"), ("gu", "બ\u{ac7}બર\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}बर\u{94d}ट प\u{94d}रा\u{902}त"), ("hu", "Bayburt"), ("hy", "Բայբուրթի նահանգ"), ("id", "Provinsi Bayburt"), ("it", "provincia di Bayburt"), ("ja", "バイブルト県"), ("ka", "ბაიბურთის პროვინცია"), ("kk", "Байбурт"), ("kn", "ಬೇಬಬರ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바이부르트 주"), ("lt", "Baiburto provincija"), ("lv", "Bajburtas ils"), ("mk", "Бајбурт"), ("mr", "बायब\u{941}र\u{94d}त प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bayburt"), ("nb", "Bayburt"), ("nl", "Bayburt"), ("no", "Bayburt"), ("pa", "ਬ\u{a47}ਬ\u{a41}ਰਤ ਸ\u{a42}ਬਾ"), ("pl", "Bayburt"), ("pt", "Bayburt"), ("ro", "Provincia Bayburt"), ("ru", "Байбурт"), ("si", "බෙබර\u{dca}ට\u{dca} පළ\u{dcf}ත"), ("sr", "Бајбурт"), ("sr_Latn", "Bajburt"), ("sv", "Bayburt"), ("sw", "Mkoa wa Bayburt"), ("ta", "பயபுரட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c47}బర\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบย\u{e4c}เบ\u{e34}ร\u{e4c}ต"), ("tk", "Baýburt"), ("tr", "Bayburt"), ("uk", "Байбурт"), ("ur", "بایبورت صوبہ"), ("uz", "Bayburt"), ("vi", "Bayburt"), ("yue", "巴伊布林特省"), ("yue_Hans", "巴伊布林特省"), ("zh", "巴伊布尔特省")]),
                        unofficial_name_list: ["Bayburt"].to_vec(),
                    }
                ),
                (
                    "70",
                    Subdivision{
                        name: "70",
                        country_alpha2: Alpha2::TR,
                        code: "70",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.17593), longitude: Some(33.228748), max_latitude: Some(37.212325), min_latitude: Some(37.146256), max_longitude: Some(33.271025), min_longitude: Some(33.179099)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كارامان"), ("az", "Karaman ili"), ("be", "Правінцыя Караман"), ("bg", "Караман"), ("bn", "ক\u{9cd}য\u{9be}র\u{9be}ম\u{9cd}য\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Vilajet Karaman"), ("ca", "Província de Karaman"), ("ccp", "𑄇𑄢𑄟𑄚\u{11134}"), ("ceb", "Karaman"), ("cs", "Karamanská provincie"), ("da", "Karaman Province"), ("de", "Karaman"), ("el", "Επαρχία Καραμάν"), ("en", "Karaman"), ("es", "Provincia de Karaman"), ("et", "Karamani provints"), ("eu", "Karaman probintzia"), ("fa", "استان قرامان"), ("fi", "Karamanin maakunta"), ("fr", "Karaman"), ("gu", "કારમ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "קאראמאן"), ("hi", "कारमन जिला"), ("hu", "Karaman"), ("hy", "Կարաման"), ("id", "Provinsi Karaman"), ("it", "provincia di Karaman"), ("ja", "カラマン県"), ("ka", "ქარამანის პროვინცია"), ("kn", "ಕರಮನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카라만 주"), ("lt", "Karamano provincija"), ("lv", "Karamanas ils"), ("mk", "Караман"), ("mr", "करामान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Karaman"), ("nb", "Karaman"), ("nl", "Karaman"), ("no", "Karaman"), ("pl", "Karaman"), ("pt", "Karaman (província)"), ("ro", "Provincia Karaman"), ("ru", "Караман"), ("si", "කරමන\u{dca} පළ\u{dcf}ත"), ("sr", "Караман"), ("sr_Latn", "Karaman"), ("sv", "Karaman"), ("sw", "Mkoa wa Karaman"), ("ta", "க\u{bbe}ரம\u{bbe}ன ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ర\u{c3e}మన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดการามาน"), ("tk", "Karaman"), ("tr", "Karaman"), ("uk", "Караман"), ("ur", "کارامان صوبہ"), ("uz", "Karaman"), ("vi", "Karaman"), ("yue", "卡拉曼省"), ("yue_Hans", "卡拉曼省"), ("zh", "卡拉曼省")]),
                        unofficial_name_list: ["Karaman"].to_vec(),
                    }
                ),
                (
                    "71",
                    Subdivision{
                        name: "71",
                        country_alpha2: Alpha2::TR,
                        code: "71",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.846821), longitude: Some(33.515251), max_latitude: Some(39.869203), min_latitude: Some(39.814633), max_longitude: Some(33.5761229), min_longitude: Some(33.4698919)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة قيريقكالي"), ("be", "Правінцыя Кырыкале"), ("bg", "Къръккале"), ("bn", "কিরিক\u{9cd}ক\u{9be}ল প\u{9cd}রদেশ"), ("bs", "Kırıkkale"), ("ca", "Província de Kırıkkale"), ("ccp", "𑄇\u{11128}𑄢\u{11128}𑄇\u{11134}𑄇𑄣𑄬"), ("ceb", "Kırıkkale"), ("cs", "Kırıkkalská provincie"), ("da", "Kırıkkale Province"), ("de", "Kırıkkale"), ("el", "Επαρχία Κιρίκαλε"), ("en", "Kırıkkale"), ("es", "Provincia de Kırıkkale"), ("et", "Kırıkkale provints"), ("eu", "Kırıkkale probintzia"), ("fa", "استان قیریق\u{200c}قلعه"), ("fi", "Kırıkkalen maakunta"), ("fr", "Kırıkkale"), ("gu", "કિનક\u{ac7}લ પ\u{acd}રા\u{a82}ત"), ("he", "נפת קרקקלה"), ("hi", "किरिकक\u{948}ल\u{947} प\u{94d}रा\u{902}त"), ("hu", "Kırıkkale"), ("hy", "Քըրըքկալե"), ("id", "Provinsi Kırıkkale"), ("it", "provincia di Kırıkkale"), ("ja", "クルッカレ県"), ("ka", "ქირიქკალეს პროვინცია"), ("kn", "ಕ\u{cbf}ರ\u{cbf}ಕಲ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키리칼레 주"), ("lt", "Kirikalės provincija"), ("lv", "Kirikales ils"), ("mk", "Крккале"), ("mr", "करक\u{94d}काल\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kırıkkale"), ("nb", "Kırıkkale"), ("nl", "Kırıkkale"), ("no", "Kırıkkale"), ("pl", "Kırıkkale"), ("pt", "Kırıkkale"), ("ro", "Provincia Kırıkkale"), ("ru", "Ил Кырыккале"), ("si", "කර\u{dd2}ක\u{dca}කලේ පළ\u{dcf}ත"), ("sr", "Кирикале"), ("sr_Latn", "Kirikale"), ("sv", "Kırıkkale"), ("sw", "Mkoa wa Kırıkkale"), ("ta", "க\u{bcd}ரிக\u{bcd}களே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3f}ర\u{c3f}క\u{c4d}క\u{c3e}ల\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องหลวงค\u{e34}ร\u{e34}คคาเล"), ("tk", "Kyrykkale"), ("tr", "Kırıkkale"), ("uk", "Кириккале"), ("ur", "قیریق قلعہ صوبہ"), ("uz", "Kırıkkale"), ("vi", "Kırıkkale"), ("yue", "克勒克卡萊省"), ("yue_Hans", "克勒克卡莱省"), ("zh", "克勒克卡莱省")]),
                        unofficial_name_list: ["Kırıkkale"].to_vec(),
                    }
                ),
                (
                    "72",
                    Subdivision{
                        name: "72",
                        country_alpha2: Alpha2::TR,
                        code: "72",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.881168), longitude: Some(41.13509), max_latitude: Some(37.934147), min_latitude: Some(37.85689199999999), max_longitude: Some(41.1744038), min_longitude: Some(41.08907019999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بطمان"), ("az", "Batman vilayəti"), ("be", "Правінцыя Батман"), ("bg", "Батман"), ("bn", "ব\u{9cd}য\u{9be}টম\u{9cd}য\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Batman"), ("ca", "Província de Batman"), ("ccp", "𑄝\u{11133}𑄠𑄖\u{11134}𑄟\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Batman"), ("cs", "Batman"), ("da", "Batman Province"), ("de", "Batman"), ("el", "Επαρχία Μπατμάν"), ("en", "Batman"), ("es", "Provincia de Batman"), ("et", "Batmani provints"), ("eu", "Batman probintzia"), ("fa", "استان باتمان"), ("fi", "Batmanin maakunta"), ("fr", "Batman"), ("gu", "બ\u{ac7}ટમ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "बतमान"), ("hu", "Batman"), ("hy", "Բաթման"), ("id", "Provinsi Batman"), ("it", "provincia di Batman"), ("ja", "バトマン県"), ("ka", "ბათმანის პროვინცია"), ("kk", "Батман"), ("kn", "ಬ\u{ccd}ಯಾಟ\u{ccd}ಮ\u{ccd}ಯಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바트만 주"), ("lt", "Betmeno provincija"), ("lv", "Batmanas ils"), ("mk", "Батман"), ("mr", "बात\u{94d}मान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Batman"), ("nb", "Batman"), ("nl", "Batman"), ("no", "Batman"), ("pa", "ਬਾਤਮਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Batman"), ("pt", "Batman"), ("ro", "Provincia Batman"), ("ru", "ил Батман"), ("si", "බැට\u{dca}මෑන\u{dca} පළ\u{dcf}ත"), ("sl", "Batman"), ("sq", "Batman"), ("sr", "Батман"), ("sr_Latn", "Batman"), ("sv", "Batman"), ("sw", "Mkoa wa Batman"), ("ta", "பேட\u{bcd}மேன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4d}య\u{c3e}ట\u{c4d}మన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บาตม\u{e31}น"), ("tk", "Batman"), ("tr", "Batman"), ("uk", "Батман"), ("ur", "باتمان صوبہ"), ("uz", "Batman"), ("vi", "Batman"), ("yue", "巴特曼省"), ("yue_Hans", "巴特曼省"), ("zh", "巴特曼省")]),
                        unofficial_name_list: ["Batman"].to_vec(),
                    }
                ),
                (
                    "73",
                    Subdivision{
                        name: "73",
                        country_alpha2: Alpha2::TR,
                        code: "73",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.516389), longitude: Some(42.461111), max_latitude: Some(37.53534), min_latitude: Some(37.50877699999999), max_longitude: Some(42.4721549), min_longitude: Some(42.440616)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شرناق"), ("az", "Şırnaq ili"), ("be", "Правінцыя Шырнак"), ("bg", "Шърнак"), ("bn", "সিন\u{9be}ক প\u{9cd}রদেশ"), ("bs", "Şırnak"), ("ca", "Província de Şırnak"), ("ccp", "𑄥\u{11128}𑄢\u{11134}𑄚𑄇\u{11134}"), ("ceb", "Şırnak"), ("cs", "Şırnakská provincie"), ("da", "Şırnak Province"), ("de", "Şırnak"), ("el", "Σίρνακ"), ("en", "Şırnak"), ("es", "Provincia de Şırnak"), ("et", "Şırnaki provints"), ("eu", "Şırnak probintzia"), ("fa", "استان شرناق"), ("fi", "Şırnakin maakunta"), ("fr", "Şırnak"), ("gu", "સિર\u{acd}નાક પ\u{acd}રા\u{a82}ત"), ("hi", "सिर\u{94d}नक प\u{94d}रा\u{902}त"), ("hu", "Şırnak"), ("hy", "Շըրնաք"), ("id", "Provinsi Şırnak"), ("it", "provincia di Şırnak"), ("ja", "シュルナク県"), ("ka", "შირნაქის პროვინცია"), ("kn", "ಸ\u{cc6}ರ\u{ccd}ನಾಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시르나크 주"), ("lt", "Širnako provincija"), ("lv", "Širnakas ils"), ("mk", "Шрнак"), ("mr", "शर\u{94d}नाक प\u{94d}रा\u{902}त"), ("ms", "Wilayah Şırnak"), ("nb", "Şırnak"), ("nl", "Şırnak"), ("no", "Şırnak"), ("pl", "Şırnak"), ("pt", "Şırnak"), ("ro", "Provincia Șırnak"), ("ru", "Ширнак"), ("si", "ස\u{dd2}ම\u{dcf}ක\u{dca} පළ\u{dcf}ත"), ("sq", "Shërnak"), ("sr", "Ширнак"), ("sr_Latn", "Širnak"), ("sv", "Şırnak"), ("sw", "Mkoa wa Şırnak"), ("ta", "சிரண\u{bbe}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}ర\u{c4d}నక\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซร\u{e34}น\u{e34}ก"), ("tk", "Şyrnak"), ("tr", "Şırnak"), ("uk", "Ширнак"), ("ur", "شرناق صوبہ"), ("uz", "Şırnak"), ("vi", "Şırnak"), ("yue", "舍爾納克省"), ("yue_Hans", "舍尔纳克省"), ("zh", "舍尔纳克省")]),
                        unofficial_name_list: ["Şırnak"].to_vec(),
                    }
                ),
                (
                    "74",
                    Subdivision{
                        name: "74",
                        country_alpha2: Alpha2::TR,
                        code: "74",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.641521), longitude: Some(32.345581), max_latitude: Some(41.654625), min_latitude: Some(41.604933), max_longitude: Some(32.37159310000001), min_longitude: Some(32.3046411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بارتين"), ("az", "Bartın vilayəti"), ("be", "Правінцыя Бартын"), ("bg", "Бартън"), ("bn", "ব\u{9be}রতিন প\u{9cd}রদেশ"), ("bs", "Bartın"), ("ca", "Província de Bartın"), ("ccp", "𑄝𑄢\u{11134}𑄑\u{11128}𑄚\u{11134}"), ("ceb", "Bartın (lalawigan)"), ("cs", "Bartınská provincie"), ("da", "Bartın Province"), ("de", "Bartın"), ("el", "Επαρχία Μπαρτίν"), ("en", "Bartın"), ("es", "Bartın"), ("et", "Bartıni provints"), ("eu", "Bartın probintzia"), ("fa", "استان بارتین"), ("fi", "Bartınin maakunta"), ("fr", "Bartın"), ("gu", "બાર\u{acd}ટિન પ\u{acd}રા\u{a82}ત"), ("hi", "बार\u{94d}टिन प\u{94d}रा\u{902}त"), ("hu", "Bartın"), ("hy", "Բարթըն"), ("id", "Provinsi Bartın"), ("it", "provincia di Bartın"), ("ja", "バルトゥン県"), ("ka", "ბართინის პროვინცია"), ("kk", "Бартын"), ("kn", "ಬಾರ\u{ccd}ಟ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바르틴 주"), ("lt", "Bartino provincija"), ("lv", "Bartinas ils"), ("mk", "Бартин"), ("mr", "बार\u{94d}तन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Bartın"), ("nb", "Bartın"), ("nl", "Bartın"), ("no", "Bartın"), ("pa", "ਬਾਰਤੀਨ ਸ\u{a42}ਬਾ"), ("pl", "Bartın"), ("pt", "Bartın"), ("ro", "Provincia Bartın"), ("ru", "Бартын"), ("si", "බ\u{dcf}ර\u{dca}ට\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sr", "Бартин"), ("sr_Latn", "Bartin"), ("sv", "Bartın"), ("sw", "Mkoa wa Bartın"), ("ta", "ப\u{bbe}ர\u{bcd}ட\u{bcd}டின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ర\u{c4d}ట\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาร\u{e4c}ต\u{e34}น"), ("tk", "Bartyn"), ("tr", "Bartın"), ("uk", "Бартин"), ("ur", "بارتین صوبہ"), ("uz", "Bartın"), ("vi", "Bartın"), ("yue", "巴爾滕省"), ("yue_Hans", "巴尔滕省"), ("zh", "巴尔滕省")]),
                        unofficial_name_list: ["Bartın"].to_vec(),
                    }
                ),
                (
                    "75",
                    Subdivision{
                        name: "75",
                        country_alpha2: Alpha2::TR,
                        code: "75",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.110481), longitude: Some(42.702171), max_latitude: Some(41.125951), min_latitude: Some(41.099811), max_longitude: Some(42.7281391), min_longitude: Some(42.6801308)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أرداهان"), ("az", "Ərdəhan vilayəti"), ("be", "Правінцыя Ардахан"), ("bg", "Ардахан"), ("bn", "আরদ\u{9be}হ\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Ardahan"), ("ca", "Província d’Ardahan"), ("ccp", "𑄃𑄢\u{11134}𑄝𑄦𑄚\u{11134}"), ("ceb", "Ardahan (lalawigan)"), ("cs", "Ardahanská provincie"), ("da", "Ardahan Province"), ("de", "Ardahan"), ("el", "Αρνταχάν"), ("en", "Ardahan"), ("es", "Provincia de Ardahan"), ("et", "Ardahani provints"), ("eu", "Ardahan probintzia"), ("fa", "استان اردهان"), ("fi", "Ardahanin maakunta"), ("fr", "Ardahan"), ("gu", "અર\u{acd}દાહન પ\u{acd}રા\u{a82}ત"), ("he", "ארדהאן"), ("hi", "अर\u{94d}दहन प\u{94d}रा\u{902}त"), ("hu", "Ardahan"), ("hy", "Արդահանի նահանգ"), ("id", "Provinsi Ardahan"), ("it", "provincia di Ardahan"), ("ja", "アルダハン県"), ("ka", "არტაანის პროვინცია"), ("kk", "Ардахан"), ("kn", "ಆರ\u{ccd}ಡಹಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아르다한 주"), ("lt", "Ardahano provincija"), ("lv", "Ardahanas ils"), ("mk", "Ардахан"), ("mr", "अर\u{94d}दाहान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ardahan"), ("nb", "Ardahan"), ("nl", "Ardahan"), ("no", "Ardahan"), ("pa", "ਅਰਦਹਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Ardahan"), ("pt", "Ardahan"), ("ro", "Provincia Ardahan"), ("ru", "Ардахан (ил)"), ("si", "අර\u{dca}දහන\u{dca} පළ\u{dcf}ත"), ("sl", "Ardahan"), ("sq", "Ardahan"), ("sr", "Ардахан"), ("sr_Latn", "Ardahan"), ("sv", "Ardahan"), ("sw", "Mkoa wa Ardahan"), ("ta", "ஆர\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అర\u{c4d}డ\u{c3e}హన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ดาฮาน"), ("tk", "Ardahan"), ("tr", "Ardahan"), ("uk", "Ардахан"), ("ur", "ارداہان صوبہ"), ("uz", "Ardahan"), ("vi", "Ardahan"), ("yue", "阿爾達漢省"), ("yue_Hans", "阿尔达汉省"), ("zh", "阿尔达汉省")]),
                        unofficial_name_list: ["Ardahan"].to_vec(),
                    }
                ),
                (
                    "76",
                    Subdivision{
                        name: "76",
                        country_alpha2: Alpha2::TR,
                        code: "76",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.91930000000001), longitude: Some(44.065498), max_latitude: Some(39.950163), min_latitude: Some(39.884109), max_longitude: Some(44.0928248), min_longitude: Some(43.9807291)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اغدير"), ("az", "İğdır vilayəti"), ("be", "Правінцыя Ыйдыр"), ("bg", "Ъгдър"), ("bs", "Iğdır"), ("ca", "Província d’Iğdır"), ("ccp", "𑄃\u{11128}𑄇\u{11134}𑄓𑄢\u{11134}"), ("ceb", "Iğdır"), ("cs", "Iğdırská provincie"), ("da", "Iğdır"), ("de", "Iğdır"), ("en", "Iğdır"), ("es", "Provincia de Iğdır"), ("et", "Iğdıri provints"), ("eu", "Iğdır probintzia"), ("fa", "استان ایغدیر"), ("fi", "Iğdırin maakunta"), ("fr", "Iğdır"), ("hu", "Iğdır"), ("hy", "Իգդիրի նահանգ"), ("id", "Provinsi Iğdır"), ("it", "provincia di Iğdır"), ("ja", "ウードゥル県"), ("ka", "იგდირის პროვინცია"), ("ko", "이디르 주"), ("lv", "Īgdiras ils"), ("mk", "Игдир"), ("mr", "इदिर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Iğdır"), ("nb", "Iğdır"), ("nl", "Iğdır"), ("no", "Iğdır"), ("pa", "ਇਗਦੀਰ ਸ\u{a42}ਬਾ"), ("pl", "Iğdır"), ("ps", "ايګدير"), ("pt", "Iğdır"), ("ro", "Provincia Iğdır"), ("ru", "Ыгдыр"), ("sl", "Iğdır"), ("sr", "Игдир"), ("sr_Latn", "Igdir"), ("sv", "Iğdır"), ("sw", "Mkoa wa Iğdır"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e37}อด\u{e37}อร\u{e4c}"), ("tk", "Ygdyr"), ("tr", "Iğdır"), ("uk", "Игдир"), ("ur", "اغدیر صوبہ"), ("vi", "Iğdır"), ("yue", "厄德爾省"), ("yue_Hans", "厄德尔省"), ("zh", "厄德尔省")]),
                        unofficial_name_list: ["Iğdır"].to_vec(),
                    }
                ),
                (
                    "77",
                    Subdivision{
                        name: "77",
                        country_alpha2: Alpha2::TR,
                        code: "77",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.65), longitude: Some(29.266667), max_latitude: Some(40.66593), min_latitude: Some(40.619459), max_longitude: Some(29.3150099), min_longitude: Some(29.2101232)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "يالوفا"), ("az", "Yalova ili"), ("be", "Правінцыя Ялава"), ("bg", "Ялова"), ("bn", "ইয\u{9bc}\u{9be}লোভ\u{9be} প\u{9cd}রদেশ"), ("bs", "Yalova"), ("ca", "Província de Yalova"), ("ccp", "𑄃\u{11128}𑄠𑄣\u{1112e}𑄞"), ("ceb", "Yalova"), ("cs", "Yalovská provincie"), ("da", "Yalova Province"), ("de", "Yalova"), ("el", "Επαρχία Γιάλοβας"), ("en", "Yalova"), ("es", "Provincia de Yalova"), ("et", "Yalova provints"), ("eu", "Yalova probintzia"), ("fa", "استان یالوا"), ("fi", "Yalovan maakunta"), ("fr", "Yalova"), ("gl", "Provincia de Yalova"), ("gu", "યાલોવા પ\u{acd}રા\u{a82}ત"), ("hi", "यलोवा प\u{94d}रा\u{902}त"), ("hu", "Yalova"), ("hy", "Յալովա"), ("id", "Provinsi Yalova"), ("it", "provincia di Yalova"), ("ja", "ヤロヴァ県"), ("ka", "იალოვის პროვინცია"), ("kn", "ಯಲೋವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "얄로바 주"), ("lt", "Jalovos provincija"), ("lv", "Jalovas ils"), ("mk", "Јалова"), ("mr", "यालोवा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Yalova"), ("nb", "Yalova"), ("nl", "Yalova"), ("no", "Yalova"), ("pl", "Yalova"), ("pt", "Yalova"), ("ro", "Provincia Yalova"), ("ru", "Ялова"), ("si", "යලොව\u{dcf} පළ\u{dcf}ත"), ("sl", "Yalova"), ("sq", "Jallova"), ("sr", "Јалова"), ("sr_Latn", "Jalova"), ("sv", "Yalova"), ("sw", "Mkoa wa Yalova"), ("ta", "யலோவ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "యల\u{c4b}వ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดยาโลวา"), ("tk", "Ýalowa"), ("tr", "Yalova"), ("uk", "Ялова"), ("ur", "یالووا صوبہ"), ("uz", "Yalova"), ("vi", "Yalova"), ("yue", "雅洛華省"), ("yue_Hans", "雅洛华省"), ("zh", "亚洛瓦省")]),
                        unofficial_name_list: ["Yalova"].to_vec(),
                    }
                ),
                (
                    "78",
                    Subdivision{
                        name: "78",
                        country_alpha2: Alpha2::TR,
                        code: "78",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2061), longitude: Some(32.62035), max_latitude: Some(41.239923), min_latitude: Some(41.172524), max_longitude: Some(32.7121672), min_longitude: Some(32.589157)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كارابوك"), ("az", "Karabük ili"), ("be", "Правінцыя Карабюк"), ("bg", "Карабюк"), ("bn", "ক\u{9be}র\u{9be}ব\u{9c1}ক প\u{9cd}রদেশ"), ("bs", "Karabük"), ("ca", "Província de Karabük"), ("ccp", "𑄇𑄢𑄝\u{1112a}𑄇\u{11134}"), ("ceb", "Karabük (lalawigan)"), ("cs", "Karabükská provincie"), ("cy", "Karabük"), ("da", "Karabuk Province"), ("de", "Karabük"), ("el", "Επαρχία Καραμπούκ"), ("en", "Karabük"), ("es", "Provincia de Karabük"), ("et", "Karabüki provints"), ("eu", "Karabük probintzia"), ("fa", "استان قره\u{200c}بوک"), ("fi", "Karabükin maakunta"), ("fr", "Karabük"), ("gu", "કરબ\u{ac1}ક પ\u{acd}રા\u{a82}ત"), ("hi", "कराब\u{941}क प\u{94d}रा\u{902}त"), ("hu", "Karabük"), ("hy", "Կարաբյուկ"), ("id", "Provinsi Karabük"), ("it", "provincia di Karabük"), ("ja", "カラビュック県"), ("ka", "ყარაბუქის პროვინცია"), ("kn", "ಕರಾಬುಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카라뷔크 주"), ("lt", "Karabjuko provincija"), ("lv", "Karabikas ils"), ("mk", "Карабук"), ("mr", "काराब\u{941}क प\u{94d}रा\u{902}त"), ("ms", "Wilayah Karabük"), ("nb", "Karabük"), ("nl", "Karabük"), ("no", "Karabük"), ("pl", "Karabük"), ("pt", "Karabük"), ("ro", "Provincia Karabük"), ("ru", "Карабюк"), ("si", "කරබ\u{dd4}ක\u{dca} පළ\u{dcf}ත"), ("sr", "Карабик"), ("sr_Latn", "Karabik"), ("sv", "Karabük"), ("sw", "Mkoa wa Karabük"), ("ta", "கர\u{bbe}புக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ర\u{c3e}బుక\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดการาบ\u{e38}ก"), ("tk", "Karabük"), ("tr", "Karabük"), ("uk", "Карабюк"), ("ur", "کارابوک صوبہ"), ("uz", "Karabük"), ("vi", "Karabük"), ("yue", "卡拉比克省"), ("yue_Hans", "卡拉比克省"), ("zh", "卡拉比克省")]),
                        unofficial_name_list: ["Karabük"].to_vec(),
                    }
                ),
                (
                    "79",
                    Subdivision{
                        name: "79",
                        country_alpha2: Alpha2::TR,
                        code: "79",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.718399), longitude: Some(37.12122), max_latitude: Some(36.736896), min_latitude: Some(36.70222), max_longitude: Some(37.1601428), min_longitude: Some(37.0916161)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كلس"), ("az", "Kilis vilayəti"), ("be", "Правінцыя Кіліс"), ("bg", "Килис"), ("bn", "কিলিস প\u{9cd}রদেশ"), ("bs", "Kilis"), ("ca", "Província de Kilis"), ("ccp", "𑄇\u{11128}𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Kilis"), ("cs", "Kiliská provincie"), ("da", "Kilis Province"), ("de", "Kilis"), ("el", "Επαρχία Κιλίς"), ("en", "Kilis"), ("es", "Provincia de Kilis"), ("et", "Kilisi provints"), ("eu", "Kilis probintzia"), ("fa", "استان کیلیس"), ("fi", "Kilisin maakunta"), ("fr", "Kilis"), ("gu", "કિલિસ પ\u{acd}રા\u{a82}ત"), ("hi", "किलिस प\u{94d}रा\u{902}त"), ("hu", "Kilis"), ("hy", "Քիլիս"), ("id", "Provinsi Kilis"), ("it", "provincia di Kilis"), ("ja", "キリス県"), ("ka", "ქილისის პროვინცია"), ("kn", "ಕ\u{cbf}ಲ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "킬리스 주"), ("lt", "Kilio provincija"), ("lv", "Kilisas ils"), ("mk", "Килис"), ("mr", "किलिस प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kilis"), ("nb", "Kilis"), ("nl", "Kilis"), ("no", "Kilis"), ("pl", "Kilis"), ("pt", "Kilis"), ("ro", "Provincia Kilis"), ("ru", "Килис"), ("si", "ක\u{dd2}ල\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Килис"), ("sr_Latn", "Kilis"), ("sv", "Kilis"), ("sw", "Mkoa wa Kilis"), ("ta", "கிளிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3f}ల\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e34}ล\u{e34}ส"), ("tk", "Kilis"), ("tr", "Kilis"), ("uk", "Кіліс"), ("ur", "کیلیس صوبہ"), ("vi", "Kilis"), ("yue", "基利斯省"), ("yue_Hans", "基利斯省"), ("zh", "基利斯省")]),
                        unofficial_name_list: ["Kilis"].to_vec(),
                    }
                ),
                (
                    "80",
                    Subdivision{
                        name: "80",
                        country_alpha2: Alpha2::TR,
                        code: "80",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.06805), longitude: Some(36.261589), max_latitude: Some(37.104061), min_latitude: Some(37.04694), max_longitude: Some(36.2902399), min_longitude: Some(36.2080339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "عثمانية"), ("az", "Osmaniyə ili"), ("be", "Правінцыя Асманіе"), ("bg", "Османие"), ("bn", "ওম\u{9be}নিয\u{9bc}ে প\u{9cd}রদেশ"), ("bs", "Osmaniye"), ("ca", "Província d’Osmaniye"), ("ccp", "𑄃\u{1112e}𑄌\u{11134}𑄟𑄚\u{11128}𑄠"), ("ceb", "Osmaniye"), ("cs", "Osmanijská provincie"), ("cy", "Osmaniye"), ("da", "Osmaniye Province"), ("de", "Osmaniye"), ("el", "Επαρχία Οσμανίγιε"), ("en", "Osmaniye"), ("es", "Provincia de Osmaniye"), ("et", "Osmaniye provints"), ("eu", "Osmaniye probintzia"), ("fa", "استان عثمانیه"), ("fi", "Osmaniyen maakunta"), ("fr", "Osmaniye"), ("gu", "ઓસ\u{acd}માનિયા પ\u{acd}રા\u{a82}ત"), ("hi", "उस\u{94d}मानिय\u{947} प\u{94d}रा\u{902}त"), ("hu", "Osmaniye"), ("hy", "Օսմանիյե"), ("id", "Provinsi Osmaniye"), ("it", "provincia di Osmaniye"), ("ja", "オスマニエ県"), ("ka", "ოსმანიეს პროვინცია"), ("kn", "ಓಸ\u{ccd}ಮಾನ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오스마니예 주"), ("lt", "Osmanijės provincija"), ("lv", "Osmanijes ils"), ("mk", "Османие"), ("mr", "ओस\u{94d}मानिय\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Osmaniye"), ("nb", "Osmaniye"), ("nl", "Osmaniye"), ("no", "Osmaniye"), ("pa", "ਓਸਮਾਨਿਏ"), ("pl", "Osmaniye"), ("pt", "Osmaniye"), ("ro", "Provincia Osmaniye"), ("ru", "Османие"), ("si", "ඔස\u{dca}මන\u{dd2}යේ පළ\u{dcf}ත"), ("sr", "Османије"), ("sr_Latn", "Osmanije"), ("sv", "Osmaniye"), ("sw", "Mkoa wa Osmaniye"), ("ta", "ஒஸ\u{bcd}ம\u{bbe}னியே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉస\u{c4d}మ\u{c3e}న\u{c3f}య\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดออสแมนน\u{e35}"), ("tk", "Osmaniýe"), ("tr", "Osmaniye"), ("uk", "Османіє"), ("ur", "عثمانیہ صوبہ"), ("uz", "Osmaniye"), ("vi", "Osmaniye"), ("yue", "奧斯曼尼耶省"), ("yue_Hans", "奥斯曼尼耶省"), ("zh", "奧斯曼尼耶省")]),
                        unofficial_name_list: ["Osmaniye"].to_vec(),
                    }
                ),
                (
                    "81",
                    Subdivision{
                        name: "81",
                        country_alpha2: Alpha2::TR,
                        code: "81",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.843849), longitude: Some(31.15654), max_latitude: Some(40.886112), min_latitude: Some(40.820133), max_longitude: Some(31.1883682), min_longitude: Some(31.116829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دوزجي"), ("az", "Düzcə ili"), ("be", "Правінцыя Дзюзджэ"), ("bg", "Дюздже"), ("bn", "দ\u{9c1}জচে প\u{9cd}রদেশ"), ("bs", "Düzce"), ("ca", "Província de Düzce"), ("ccp", "𑄓\u{1112a}𑄌\u{11134}𑄥𑄬"), ("cs", "Düzcská provincie"), ("cy", "Düzce"), ("da", "Duzce Province"), ("de", "Düzce"), ("el", "Επαρχία Ντούζτζε"), ("en", "Düzce"), ("es", "Provincia de Düzce"), ("et", "Düzce provints"), ("eu", "Düzce probintzia"), ("fa", "استان دوزجه"), ("fi", "Düzcen maakunta"), ("fr", "Düzce"), ("gu", "ડ\u{ac1}ઝ પ\u{acd}રા\u{a82}ત"), ("hi", "ड\u{942}ट\u{94d}स प\u{94d}रा\u{902}त"), ("hu", "Düzce"), ("hy", "Դյուզջեի նահանգ"), ("id", "Provinsi Düzce"), ("it", "provincia di Düzce"), ("ja", "デュズジェ県"), ("ka", "დუზჯეს პროვინცია"), ("kn", "ಡ\u{ccd}ಯ\u{cc2}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "뒤즈제 주"), ("lt", "Diuzdžės apskritis"), ("lv", "Dizdžes ils"), ("mk", "Дузџе"), ("mr", "द\u{941}झ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Düzce"), ("nb", "Düzce"), ("nl", "Düzce"), ("no", "Düzce"), ("pa", "ਦ\u{a41}ਜ\u{a3c}ਗ\u{a47}"), ("pl", "Düzce"), ("pt", "Düzce"), ("ro", "Provincia Düzce"), ("ru", "Дюздже"), ("si", "ඩස\u{dca}ක\u{dca} පළ\u{dcf}ත"), ("sq", "Dyzxhe"), ("sr", "Дузџе"), ("sr_Latn", "Duzdže"), ("sv", "Düzce"), ("sw", "Mkoa wa Düzce"), ("ta", "டியூஸ\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డూస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e38}ซเซ"), ("tk", "Düzje"), ("tr", "Düzce"), ("uk", "Дюздже"), ("ur", "دوزجے صوبہ"), ("uz", "Düzce"), ("vi", "Düzce"), ("yue", "迪茲傑省"), ("yue_Hans", "迪兹杰省"), ("zh", "迪兹杰省")]),
                        unofficial_name_list: ["Düzce"].to_vec(),
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
#[cfg(feature = "tr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::TR,
        alpha3: Alpha3::TUR,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 90,
        currency_code: "TRY",
        gec: Some(GEC::TU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("TUR"),
        iso_long_name: "The Republic of Turkey",
        iso_short_name: "Turkey",
        official_language_list: ["tr"].to_vec(),
        spoken_language_list: ["tr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "0",
        nationality: Some("Turkish"),
        number: "792",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "TR",
        unofficial_name_list: [
            "Turkey",
            "Türkei",
            "Turquie",
            "Turquía",
            "トルコ",
            "Turkije",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Turkey"),
            ("af", "Turkye"),
            ("ak", "Turkey"),
            ("am", "ቱሴጤ"),
            ("an", "Turkey"),
            ("ar", "تركي\u{651}ا"),
            ("as", "ত\u{9c1}ৰস\u{9cd}ক"),
            ("ay", "Turkey"),
            ("az", "Türkiyə"),
            ("ba", "Turkey"),
            ("be", "Турцыя"),
            ("bg", "Турция"),
            ("bi", "Turkey"),
            ("bn", "ত\u{9c1}র\u{9cd}কি"),
            ("bn_IN", "ত\u{9c1}র\u{9cd}কি"),
            ("br", "Turkia"),
            ("bs", "Turska"),
            ("ca", "Turquia"),
            ("ce", "Турци"),
            ("ch", "Turkey"),
            ("cs", "Turecko"),
            ("cv", "Турци"),
            ("cy", "Twrci"),
            ("da", "Tyrkiet"),
            ("de", "Türkei"),
            ("dv", "ތ\u{7aa}ރ\u{7aa}ކ\u{7a9}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}"),
            ("dz", "ཊར་ཀ\u{f72}།"),
            ("ee", "Turkey"),
            ("el", "Τουρκία"),
            ("en", "Turkey"),
            ("eo", "Turkio"),
            ("es", "Turquía"),
            ("et", "Türgi"),
            ("eu", "Turkia"),
            ("fa", "ترکیه"),
            ("ff", "Türkiye"),
            ("fi", "Turkki"),
            ("fo", "Turkaland"),
            ("fr", "Turquie"),
            ("fy", "Turkije"),
            ("ga", "An Tuirc"),
            ("gl", "Turquía"),
            ("gn", "Turkey"),
            ("gu", "ત\u{ac1}ર\u{acd}કી"),
            ("gv", "Yn Turkee"),
            ("ha", "Turkiyya"),
            ("he", "טורקיה"),
            ("hi", "त\u{941}र\u{94d}की"),
            ("hr", "Turska"),
            ("ht", "Tiki"),
            ("hu", "Törökország"),
            ("hy", "Թուրքիա"),
            ("ia", "Turchia"),
            ("id", "Turki"),
            ("io", "Turkia"),
            ("is", "Tyrkland"),
            ("it", "Turchia"),
            ("iu", "ᑑᕐᑭ"),
            ("ja", "トルコ"),
            ("ka", "თურქეთი"),
            ("ki", "Turkey"),
            ("kk", "Түркия"),
            ("kl", "Turkey"),
            ("km", "ទ\u{17bd}រគ\u{17b8}"),
            ("kn", "ಟರ\u{ccd}ಕ\u{cbf}"),
            ("ko", "터키"),
            ("ku", "Tirkiye"),
            ("kv", "Турция"),
            ("kw", "Turki"),
            ("ky", "Түркия Республикасы"),
            ("lo", "ປະເທດຕວກກ\u{eb5}"),
            ("lt", "Turkija"),
            ("lv", "Turcija"),
            ("mi", "Tākei"),
            ("mk", "Турција"),
            ("ml", "ത\u{d41}ര\u{d4d}\u{200d}ക\u{d4d}കി"),
            ("mn", "Турк"),
            ("mr", "त\u{941}र\u{94d}की"),
            ("ms", "Turki"),
            ("mt", "Turkija"),
            (
                "my",
                "တ\u{1030}ရက\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Terki"),
            ("nb", "Tyrkia"),
            ("ne", "टर\u{94d}की"),
            ("nl", "Turkije"),
            ("nn", "Tyrkia"),
            ("nv", "Tʼóok Bikéyah"),
            ("oc", "Turquia"),
            ("or", "ତ\u{b41}ର\u{b4d}କୀ"),
            ("pa", "ਤ\u{a41}ਰਕੀ"),
            ("pi", "त\u{941}र\u{94d}किय\u{947}"),
            ("pl", "Turcja"),
            ("ps", "تورکيه"),
            ("pt", "Turquia"),
            ("pt_BR", "Turquia"),
            ("ro", "Turcia"),
            ("ru", "Турция"),
            ("rw", "Turukiya"),
            ("sc", "Turchia"),
            ("sd", "Turkey"),
            ("si", "ත\u{dd4}ර\u{dca}ක\u{dd2}ය"),
            ("sk", "Turecko"),
            ("sl", "Turčija"),
            ("so", "Turki"),
            ("sq", "Turqi"),
            ("sr", "Турска"),
            ("sv", "Turkiet"),
            ("sw", "Turkey"),
            ("ta", "துருக\u{bcd}கி"),
            ("te", "టర\u{c4d}క\u{c40}"),
            ("tg", "Туркия"),
            ("th", "ต\u{e38}รก\u{e35}"),
            ("ti", "ቱርኪ"),
            ("tk", "Türk"),
            ("tl", "Turkey"),
            ("tr", "Türkiye"),
            ("tt", "Төркиә"),
            ("ug", "تۈركىيە"),
            ("uk", "Туреччина"),
            ("ur", "ترکی"),
            ("uz", "Turkiya"),
            ("ve", "Turkey"),
            ("vi", "Thổ Nhĩ Kỳ"),
            ("wa", "Turkeye"),
            ("wo", "Turki"),
            ("xh", "Turkey"),
            ("yo", "Túrkì"),
            ("zh_CN", "土耳其"),
            ("zh_HK", "土耳其"),
            ("zh_TW", "土耳其"),
            ("zu", "ITheki"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
