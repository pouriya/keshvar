// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Cuba

#[cfg(all(feature = "cu", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CU;
    pub const ALPHA3: Alpha3 = Alpha3::CUB;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 53;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::CUP;
    pub const GEC: Option<GEC> = Some(GEC::CU);
    pub const INTERNATIONAL_PREFIX: &str = "119";
    pub const IOC: Option<IOC> = Some(IOC::CUB);
    pub const ISO_SHORT_NAME: &str = "Cuba";
    pub const ISO_LONG_NAME: &str = "The Republic of Cuba";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Cuban");
    pub const NUMBER: &str = "192";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "CU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Cuba", "Kuba", "キューバ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Cuba"),
        ("af", "Kuba"),
        ("ak", "Cuba"),
        ("am", "Cuba"),
        ("an", "Cuba"),
        ("ar", "كوبا"),
        ("as", "কিউব\u{9be}"),
        ("ay", "Cuba"),
        ("az", "Kuba"),
        ("ba", "Cuba"),
        ("be", "Куба"),
        ("bg", "Куба"),
        ("bi", "Cuba"),
        ("bn", "কিউব\u{9be}"),
        ("bn_IN", "কিউব\u{9be}"),
        ("br", "Kuba"),
        ("bs", "Kuba"),
        ("ca", "Cuba"),
        ("ce", "Куба"),
        ("ch", "Cuba"),
        ("cs", "Kuba"),
        ("cv", "Куба"),
        ("cy", "Ciwba"),
        ("da", "Cuba"),
        ("de", "Kuba"),
        ("dv", "ކ\u{7a8}އ\u{7aa}ބ\u{7a7}"),
        ("dz", "ཀ\u{f74}ས་བ།"),
        ("ee", "Cuba"),
        ("el", "Κούβα"),
        ("en", "Cuba"),
        ("eo", "Kubo"),
        ("es", "Cuba"),
        ("et", "Kuuba"),
        ("eu", "Kuba"),
        ("fa", "کوبا"),
        ("ff", "Cuba"),
        ("fi", "Kuuba"),
        ("fo", "Kuba"),
        ("fr", "Cuba"),
        ("fy", "Kuba"),
        ("ga", "Cúba"),
        ("gl", "Cuba"),
        ("gn", "Cuba"),
        ("gu", "ક\u{acd}ય\u{ac1}બા"),
        ("gv", "Yn Choobey"),
        ("ha", "Cuba"),
        ("he", "קובה"),
        ("hi", "क\u{94d}य\u{942}बा"),
        ("hr", "Kuba"),
        ("ht", "Kiba"),
        ("hu", "Kuba"),
        ("hy", "Կուբա"),
        ("ia", "Cuba"),
        ("id", "Kuba"),
        ("io", "Kuba"),
        ("is", "Kúba"),
        ("it", "Cuba"),
        ("iu", "ᖂᐹ/quupaa"),
        ("ja", "キューバ"),
        ("ka", "კუბა"),
        ("ki", "Cuba"),
        ("kk", "Куба"),
        ("kl", "Cuba"),
        ("km", "គ\u{17bb}យបា"),
        ("kn", "ಕ\u{ccd}ಯ\u{cc2}ಬಾ"),
        ("ko", "쿠바"),
        ("ku", "Kûba"),
        ("kv", "Куба"),
        ("kw", "Kuba"),
        ("ky", "Куба"),
        ("lo", "Cuba"),
        ("lt", "Kuba"),
        ("lv", "Kuba"),
        ("mi", "Kūpā"),
        ("mk", "Куба"),
        ("ml", "ക\u{d4d}യ\u{d42}ബ"),
        ("mn", "Куба"),
        ("mr", "क\u{94d}य\u{942}बा"),
        ("ms", "Cuba"),
        ("mt", "Kuba"),
        ("my", "ကျ\u{1030}းဘားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Kiuba"),
        ("nb", "Cuba"),
        ("ne", "क\u{94d}य\u{942}बा"),
        ("nl", "Cuba"),
        ("nn", "Cuba"),
        ("nv", "Cuba"),
        ("oc", "Cuba"),
        ("or", "କ\u{b4d}ଯ\u{b41}ବ\u{b3e}"),
        ("pa", "ਕਿਊਬਾ"),
        ("pi", "क\u{94d}य\u{942}बा"),
        ("pl", "Kuba"),
        ("ps", "کیوبا"),
        ("pt", "Cuba"),
        ("pt_BR", "Cuba"),
        ("ro", "Cuba"),
        ("ru", "Куба"),
        ("rw", "Kiba"),
        ("sc", "Cuba"),
        ("sd", "ڪيوبا"),
        ("si", "ක\u{dd2}ය\u{dd4}බ\u{dcf}"),
        ("sk", "Kuba"),
        ("sl", "Kuba"),
        ("so", "Kuuba"),
        ("sq", "Kubë"),
        ("sr", "Куба"),
        ("sv", "Kuba"),
        ("sw", "Cuba"),
        ("ta", "க\u{bcd}யூப\u{bbe}"),
        ("te", "క\u{c4d}యూబ\u{c3e}"),
        ("tg", "Куба"),
        ("th", "ค\u{e34}วบา"),
        ("ti", "ኩባ"),
        ("tk", "Kuba"),
        ("tl", "Kuba"),
        ("tr", "Küba"),
        ("tt", "Куба"),
        ("ug", "كۇبا"),
        ("uk", "Куба"),
        ("ur", "کیوبا"),
        ("uz", "Kuba"),
        ("ve", "Cuba"),
        ("vi", "Cu-ba"),
        ("wa", "Cuba"),
        ("wo", "Kubaa"),
        ("xh", "Cuba"),
        ("yo", "Kúbà"),
        ("zh_CN", "古巴"),
        ("zh_HK", "古巴"),
        ("zh_TW", "古巴"),
        ("zu", "Cuba"),
    ];
    #[cfg(all(feature = "cu", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 21.521757;
        pub const LONGITUDE: f64 = -77.781167;
        pub const MAX_LATITUDE: f64 = 23.3776001;
        pub const MAX_LONGITUDE: f64 = -73.9545;
        pub const MIN_LATITUDE: f64 = 19.6529001;
        pub const MIN_LONGITUDE: f64 = -85.1715001;
        pub const NORTHEAST_LATITUDE: f64 = 23.3776001;
        pub const NORTHEAST_LONGITUDE: f64 = -73.9545;
        pub const SOUTHWEST_LATITUDE: f64 = 19.6529001;
        pub const SOUTHWEST_LONGITUDE: f64 = -85.1715001;
    }
}
#[cfg(all(feature = "cu", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 21.521757,
            longitude: -77.781167,
            max_latitude: 23.3776001,
            max_longitude: -73.9545,
            min_latitude: 19.6529001,
            min_longitude: -85.1715001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 23.3776001,
                    longitude: -73.9545,
                },
                southwest: CountryGeoBound {
                    latitude: 19.6529001,
                    longitude: -85.1715001,
                },
            },
        }
    }
}

#[cfg(all(feature = "cu", feature = "subdivisions"))]
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
                        name: "Pinar del Río",
                        country_alpha2: Alpha2::CU,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.412222), longitude: Some(-83.671944), max_latitude: Some(22.4633914), min_latitude: Some(22.3613422), max_longitude: Some(-83.6413955), min_longitude: Some(-83.74414449999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pinar del Río"), ("ar", "محافظة بينار ديل ريو"), ("bg", "Пинар дел Рио"), ("bn", "পিন\u{9be}র ডেল রিও প\u{9cd}রদেশ"), ("ca", "Província de Pinar del Río"), ("ccp", "𑄛\u{11127}𑄚𑄢\u{11134} 𑄓𑄬𑄣\u{11134} 𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Pinar del Río"), ("cs", "Pinar del Río"), ("da", "Pinar del Río Provinsen"), ("de", "Provinz Pinar del Río"), ("el", "Πινάρ ντελ Ρίο"), ("en", "Pinar del Río"), ("es", "Provincia de Pinar del Río"), ("et", "Pinar del Río provints"), ("eu", "Pinar del Ríoko probintzia"), ("fa", "استان پینار دل ریو"), ("fi", "Pinar del Río"), ("fr", "province de Pinar del Río"), ("gl", "Provincia de Pinar del Río"), ("gu", "પિનાર ડ\u{ac7}લ રિયો પ\u{acd}રા\u{a82}ત"), ("hi", "पिनार ड\u{947}ल रियो प\u{94d}रा\u{902}त"), ("hr", "Pinar del Río"), ("hu", "Pinar del Río"), ("id", "Provinsi Pinar del Río"), ("it", "provincia di Pinar del Río"), ("ja", "ピナール・デル・リオ州"), ("ka", "პინარ-დელ-რიოს პროვინცია"), ("kn", "ಪ\u{cbf}ನಾರ\u{ccd} ಡ\u{cc6}ಲ\u{ccd} ರ\u{cbf}ಯೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "피나르델리오 주"), ("lt", "Pinar del Rijo provincija"), ("lv", "Pinaras del Rio province"), ("mk", "Пинар дел Рио"), ("mr", "पिनार ड\u{947}ल रिओ प\u{94d}रा\u{902}त"), ("ms", "Pinar del Río Province"), ("nb", "Pinar del Rio"), ("nl", "Pinar del Río"), ("no", "Pinar del Rio"), ("pl", "Prowincja Pinar del Río"), ("pt", "Pinar del Río"), ("ru", "Пинар-дель-Рио"), ("si", "ප\u{dd2}න\u{dcf}ර\u{dca} ඩෙල\u{dca} ර\u{dd2}යෝ පළ\u{dcf}ත"), ("sv", "Provincia de Pinar del Río"), ("ta", "பின\u{bcd}ன\u{bbe}ர\u{bcd} டெல\u{bcd} ரியோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3f}న\u{c3e}ర\u{c4d} డ\u{c46}ల\u{c4d} ర\u{c3f}య\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ป\u{e35}นาร\u{e4c}เดลร\u{e35}โอ"), ("tr", "Pinar del Río"), ("uk", "Пінар-дель-Ріо"), ("ur", "پینار دیل ریو صوبہ"), ("vi", "Pinar del Río"), ("zh", "比那尔德里奥省")]),
                        unofficial_name_list: ["Pinar del Río"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "La Habana",
                        country_alpha2: Alpha2::CU,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.159167), longitude: Some(-82.271056), max_latitude: Some(23.1812734), min_latitude: Some(22.9200574), max_longitude: Some(-82.0911311), min_longitude: Some(-82.5742877)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Гавана"), ("ca", "Província de l’Havana"), ("ccp", "𑄦𑄞𑄚"), ("de", "Provinz La Habana"), ("en", "Havana"), ("es", "Provincia de La Habana"), ("fr", "Ciudad de La Habana"), ("gl", "Provincia da Cidade da Habana"), ("hu", "Havanna"), ("ja", "ハバナ"), ("nl", "Ciudad de La Habana"), ("pt", "Ciudad de La Habana (província)"), ("ru", "Гавана"), ("sv", "Provincia de Ciudad de La Habana")]),
                        unofficial_name_list: ["La Habana"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "Matanzas",
                        country_alpha2: Alpha2::CU,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.5767123), longitude: Some(-81.3399414), max_latitude: Some(23.2767521), min_latitude: Some(22.0036335), max_longitude: Some(-80.47074099999999), min_longitude: Some(-82.16040799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Matanzas"), ("ar", "ماتنزاس"), ("be", "Правінцыя Матансас"), ("bg", "Матансас"), ("bn", "ম\u{9be}ত\u{9be}নজ\u{9be}স প\u{9cd}রদেশ"), ("ca", "Província de Matanzas"), ("ccp", "𑄟𑄑𑄚\u{11134}𑄎𑄌\u{11134}"), ("ceb", "Matanzas"), ("cs", "Matanzas"), ("da", "Matanzas Provinsen"), ("de", "Provinz Matanzas"), ("el", "Ματάνσας"), ("en", "Matanzas"), ("es", "Provincia de Matanzas"), ("eu", "Matanzasko probintzia"), ("fa", "استان ماتانزاس"), ("fi", "Matanzasin lääni"), ("fr", "province de Matanzas"), ("gl", "Provincia de Matanzas"), ("gu", "મતાઝાસ પ\u{acd}રા\u{a82}ત"), ("he", "מטנזס"), ("hi", "मटनजस प\u{94d}रा\u{902}त"), ("hr", "Matanzas"), ("hu", "Matanzas"), ("hy", "Մատանսաս"), ("id", "Provinsi Matanzas"), ("it", "provincia di Matanzas"), ("ja", "マタンサス州"), ("ka", "მატანსასის პროვინცია"), ("kn", "ಮಟಾಂಜಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마탄사스 주"), ("lt", "Matansaso provincija"), ("lv", "Matansasas province"), ("mk", "Матансас"), ("mr", "मटणजस प\u{94d}रा\u{902}त"), ("ms", "Matanzas Province"), ("nb", "Matanzas"), ("nl", "Matanzas"), ("no", "Matanzas"), ("pl", "Prowincja Matanzas"), ("pt", "Matanzas"), ("ru", "Матансас"), ("si", "මන\u{dca}ටන\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Matanzas"), ("ta", "மடன\u{bcd}சஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ట\u{c3e}ంజ\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาต\u{e31}นซ\u{e31}ส"), ("tr", "Matanzas ili"), ("uk", "Матансас"), ("ur", "ماتانساس صوبہ"), ("vi", "Matanzas"), ("zh", "马坦萨斯省")]),
                        unofficial_name_list: ["Matanzas"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "Villa Clara",
                        country_alpha2: Alpha2::CU,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.4937204), longitude: Some(-79.9192702), max_latitude: Some(23.1555759), min_latitude: Some(21.92908), max_longitude: Some(-79.3098399), min_longitude: Some(-80.740882)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Villa Clara"), ("ar", "محافظة فيلا كلارا"), ("be", "Правінцыя Вілья-Клара"), ("bg", "Виля Клара"), ("bn", "ভিল\u{9be} ক\u{9cd}ল\u{9be}র\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Villa Clara"), ("ccp", "𑄞\u{11128}𑄣 𑄇\u{11133}𑄣𑄢"), ("ceb", "Villa Clara"), ("cs", "Provincia de Villa Clara"), ("cy", "Villa Clara Province"), ("da", "Villa Clara Provinsen"), ("de", "Provinz Villa Clara"), ("el", "Βίγια Κλάρα"), ("en", "Villa Clara"), ("es", "Provincia de Villa Clara"), ("et", "Villa Clara provints"), ("eu", "Villa Clarako probintzia"), ("fa", "استان ویلا کلارا"), ("fi", "Villa Claran lääni"), ("fr", "Villa Clara"), ("gl", "Provincia de Villa Clara"), ("gu", "વિલા ક\u{acd}લ\u{ac7}રા પ\u{acd}રા\u{a82}ત"), ("he", "ויה קלרה"), ("hi", "विला क\u{94d}लारा प\u{94d}रा\u{902}त"), ("hr", "Villa Clara"), ("hu", "Villa Clara"), ("id", "Provinsi Villa Clara"), ("it", "provincia di Villa Clara"), ("ja", "ビジャ・クララ州"), ("ka", "ვილია-კლარას პროვინცია"), ("kn", "ವ\u{cbf}ಲ\u{ccd}ಲಾ ಕ\u{ccd}ಲಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비야클라라 주"), ("lt", "Vilja Klaros provincija"), ("lv", "Viljaklaras province"), ("mk", "Виља Клара"), ("mr", "व\u{94d}हिला क\u{94d}लारा प\u{94d}रा\u{902}त"), ("ms", "Villa Clara Province"), ("nb", "Villa Clara Provinsen"), ("nl", "Villa Clara"), ("no", "Villa Clara Provinsen"), ("pl", "Prowincja Villa Clara"), ("pt", "Villa Clara"), ("ru", "Вилья-Клара"), ("si", "ව\u{dd2}ල\u{dcf} ක\u{dca}ල\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sv", "Provincia de Villa Clara"), ("ta", "வில\u{bcd}ல\u{bbe} கிள\u{bbe}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3f}ల\u{c4d}ల\u{c3e} క\u{c4d}ల\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บ\u{e35}ยากลารา"), ("tr", "Villa Clara ili"), ("uk", "Вілья-Клара"), ("ur", "ویا کلارا صوبہ"), ("vi", "Villa Clara"), ("zh", "比亚克拉拉省")]),
                        unofficial_name_list: ["Villa Clara"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "Cienfuegos",
                        country_alpha2: Alpha2::CU,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.2379783), longitude: Some(-80.365865), max_latitude: Some(22.5832278), min_latitude: Some(21.817026), max_longitude: Some(-80.01658789999999), min_longitude: Some(-80.93570679999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cienfuegos"), ("ar", "سينفويغوس"), ("be", "Правінцыя Сьенфуэгас"), ("bg", "Сиенфуегос"), ("bn", "চিয\u{9bc}েনফ\u{9c1}য\u{9bc}েগোস প\u{9cd}রদেশ"), ("ccp", "𑄥\u{1112d}𑄠𑄚\u{11134}𑄜\u{11128}𑄅\u{1112a}𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Cienfuegos"), ("cs", "Cienfuegos"), ("da", "Cienfuegos Provinsen"), ("de", "Provinz Cienfuegos"), ("el", "Σιενφουέγος"), ("en", "Cienfuegos"), ("es", "Provincia de Cienfuegos"), ("eu", "Cienfuegosko probintzia"), ("fa", "استان سینفیوگوس"), ("fi", "Cienfuegos"), ("fr", "province de Cienfuegos"), ("gl", "Provincia de Cienfuegos"), ("gu", "સિએનફ\u{acd}ય\u{ac1}ગોસ પ\u{acd}રા\u{a82}ત"), ("hi", "सिएनफ\u{94d}य\u{942}गोस प\u{94d}रा\u{902}त"), ("hr", "Cienfuegos"), ("hu", "Cienfuegos"), ("id", "Provinsi Cienfuegos"), ("it", "provincia di Cienfuegos"), ("ja", "シエンフエーゴス州"), ("ka", "სიენფუეგოსის პროვინცია"), ("kn", "ಸ\u{cbf}ನ\u{ccd}ಫ\u{ccd}ಯ\u{cc2}ಗೊಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시엔푸에고스 주"), ("lt", "Sjenfuegoso provincija"), ("lv", "Sjenfuegosas province"), ("mk", "Сиенфуегос"), ("mr", "सिएनफ\u{94d}य\u{942}गोस प\u{94d}रा\u{902}त"), ("ms", "Cienfuegos Province"), ("nb", "Cienfuegos"), ("nl", "Cienfuegos"), ("no", "Cienfuegos"), ("pl", "Prowincja Cienfuegos"), ("pt", "Cienfuegos"), ("ru", "Сьенфуэгос"), ("si", "ස\u{dd2}යෙන\u{dca}ෆ\u{dd2}ය\u{dd4}ගොස\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Cienfuegos"), ("ta", "சிஏபியூயகோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}న\u{c4d}ఫ\u{c4d}యూగ\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเซ\u{e35}ยนฟวยโกส"), ("tr", "Cienfuegos ili"), ("uk", "Сьєнфуегос"), ("ur", "سینفویگوس صوبہ"), ("vi", "Cienfuegos"), ("zh", "西恩富戈斯省")]),
                        unofficial_name_list: ["Cienfuegos"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "Sancti Spíritus",
                        country_alpha2: Alpha2::CU,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.9938214), longitude: Some(-79.4703885), max_latitude: Some(22.6677195), min_latitude: Some(21.4506246), max_longitude: Some(-78.92516309999999), min_longitude: Some(-80.09424299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانكتي سبيريتوس"), ("bg", "Санкти Спиритус"), ("bn", "স\u{9be}নস\u{9cd}কি স\u{9cd}পিরিটস প\u{9cd}রদেশ"), ("ccp", "𑄥𑄚\u{11134}𑄑\u{11128} 𑄌\u{11133}𑄛\u{1112d}𑄢\u{11128}𑄑𑄌\u{11134}"), ("ceb", "Sancti Spíritus"), ("da", "Sancti Spíritus Provinsen"), ("de", "Provinz Sancti Spíritus"), ("el", "Σάνκτι Σπίριτους"), ("en", "Sancti Spíritus"), ("es", "Provincia de Sancti Spíritus"), ("eu", "Sancti Spiritusko probintzia"), ("fa", "استان سانکتی اسپیریتوس"), ("fi", "Sancti Spíritusin lääni"), ("fr", "province de Sancti Spíritus"), ("gl", "Provincia de Sancti Spíritus"), ("gu", "સ\u{ac7}\u{a82}ક\u{acd}ટી સ\u{acd}પિરિટસ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}\u{902}क\u{94d}टी स\u{94d}पिरिटस प\u{94d}रा\u{902}त"), ("hr", "Sancti Spíritus"), ("hu", "Sancti Spíritus"), ("id", "Provinsi Sancti Spíritus"), ("it", "provincia di Sancti Spíritus"), ("ja", "サンクティ・スピリトゥス州"), ("ka", "სანქტი-სპირიტუსის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾಂಕ\u{ccd}ಟ\u{cbf} ಸ\u{ccd}ಪೈರ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "상크티스피리투스 주"), ("lt", "Sankti Spirituso provincija"), ("lv", "Sanktispiritusas province"), ("mk", "Санкти Спиритус"), ("mr", "स\u{901}क\u{94d}टि स\u{94d}पिरिटस प\u{94d}रा\u{902}त"), ("ms", "Sancti Spiritus Province"), ("nb", "Sancti Spíritus"), ("nl", "Sancti Spíritus"), ("no", "Sancti Spíritus"), ("pl", "Prowincja Sancti Spíritus"), ("pt", "Sancti Spíritus"), ("ru", "Санкти-Спиритус"), ("si", "සැන\u{dca}ස\u{dd2}ට\u{dd2} ස\u{dca}ප\u{dca}ර\u{dd2}චස\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Sancti Spíritus"), ("ta", "ஸந\u{bcd}க\u{bcd}தி ஸ\u{bcd}பிரிட\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంక\u{c4d}ట\u{c3f} స\u{c4d}ప\u{c3f}ర\u{c3f}టస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแซนค\u{e4c}ต\u{e34} สป\u{e34}ร\u{e34}ท\u{e31}ส"), ("tr", "Sancti Spíritus ili"), ("uk", "Санкті-Спірітус"), ("ur", "سانکتی سپیریتوس صوبہ"), ("vi", "Sancti Spíritus"), ("zh", "圣斯皮里图斯省")]),
                        unofficial_name_list: ["Sancti Spíritus"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "Ciego de Ávila",
                        country_alpha2: Alpha2::CU,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.9329515), longitude: Some(-78.5660852), max_latitude: Some(22.6132385), min_latitude: Some(20.844875), max_longitude: Some(-78.067293), min_longitude: Some(-79.45208699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيغو دي أفيلا"), ("be", "Правінцыя Сьега-дэ-Авіла"), ("bg", "Сиего де Авила"), ("bn", "সিয\u{9bc}েগো দে আভিল\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄥\u{1112d}𑄠𑄬𑄉\u{1112e} 𑄓𑄬 𑄃𑄞\u{11128}𑄣"), ("ceb", "Ciego de Ávila"), ("da", "Ciego de Ávila Provinsen"), ("de", "Provinz Ciego de Ávila"), ("el", "Σιέγο ντε Άβιλα"), ("en", "Ciego de Ávila"), ("es", "Provincia de Ciego de Ávila"), ("eu", "Ciego de Ávilako probintzia"), ("fa", "استان سیه\u{200c}گو د آویلا"), ("fi", "Ciego de Ávila"), ("fr", "province de Ciego de Ávila"), ("gl", "Provincia de Ciego de Ávila"), ("gu", "સિએગો દ\u{ac7} એવીલા પ\u{acd}રા\u{a82}ત"), ("hi", "सिएगो डी एविला प\u{94d}रा\u{902}त"), ("hr", "Ciego de Ávila"), ("hu", "Ciego de Ávila"), ("id", "Provinsi Ciego de Ávila"), ("it", "provincia di Ciego de Ávila"), ("ja", "シエゴ・デ・アビラ州"), ("ka", "სიეგო-დე-ავილის პროვინცია"), ("kn", "ಸ\u{cbf}ಯೋಗೊ ಡ\u{cbf} ಆವ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시에고데아빌라 주"), ("lt", "Sjego de Avilos provincija"), ("lv", "Sjego de Avilas province"), ("mk", "Сиего де Авила"), ("mr", "सिगो डी एविला प\u{94d}रा\u{902}त"), ("ms", "Ciego de Avila Province"), ("nb", "Ciego de Avila"), ("nl", "Ciego de Ávila"), ("no", "Ciego de Avila"), ("pl", "Prowincja Ciego de Ávila"), ("pt", "Ciego de Ávila"), ("ru", "Сьего-де-Авила"), ("si", "ස\u{dd2}යෙගෝ ඩ\u{dd2} අව\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sv", "Provincia de Ciego de Ávila"), ("ta", "சிஎகோ டி அவில\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}య\u{c47}గ\u{c4b} \u{c4d} అవ\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซ\u{e35}ยโกเดอาบ\u{e35}ลา"), ("tr", "Ciego de Avila Province"), ("uk", "Сьєго-де-Авіла"), ("ur", "سیئگو دے ابیلا صوبہ"), ("vi", "Ciego de Ávila"), ("zh", "谢戈德阿维拉省")]),
                        unofficial_name_list: ["Ciego de Ávila"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "Camagüey",
                        country_alpha2: Alpha2::CU,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.3926035), longitude: Some(-77.9053182), max_latitude: Some(21.4397273), min_latitude: Some(21.3328497), max_longitude: Some(-77.8250886), min_longitude: Some(-77.9696273)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Camagüey (provinsie)"), ("ar", "كاماغوي"), ("be", "Правінцыя Камагуэй"), ("bg", "Камагуей"), ("bn", "ক\u{9be}ম\u{9be}গ\u{9c1}য\u{9bc}ে প\u{9cd}রদেশ"), ("ca", "Província de Camagüey"), ("ccp", "𑄇\u{11133}𑄠𑄟𑄉\u{1112d}\u{1112a}𑄠𑄬"), ("ceb", "Camagüey"), ("cs", "Camagüey"), ("da", "Camagüey Provinsen"), ("de", "Provinz Camagüey"), ("el", "Καμαγουέι"), ("en", "Camagüey"), ("es", "Provincia de Camagüey"), ("eu", "Camagüeyko probintzia"), ("fa", "استان کاماگوی"), ("fi", "Camagüey"), ("fr", "province de Camagüey"), ("gl", "Provincia de Camagüey"), ("gu", "ક\u{ac7}મગ\u{acd}ય\u{ac1}ઇ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{948}माग\u{94d}व\u{947} प\u{94d}रा\u{902}त"), ("hr", "Camagüey"), ("hu", "Camagüey"), ("id", "Provinsi Camagüey"), ("it", "provincia di Camagüey"), ("ja", "カマグエイ州"), ("ka", "კამაგუეის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಮಗು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카마구에이 주"), ("lt", "Kamagvėjaus provincija"), ("lv", "Kamagvejas province"), ("mk", "Камагвеј"), ("mr", "क\u{945}मग\u{94d}य\u{941}ई प\u{94d}रा\u{902}त"), ("ms", "Camaguey Province"), ("nb", "Camagüey"), ("nl", "Camagüey"), ("no", "Camagüey"), ("pl", "Prowincja Camagüey"), ("pt", "Camagüey"), ("ru", "Камагуэй"), ("si", "කැමග\u{dd4}ය\u{dd2} පළ\u{dcf}ත"), ("sv", "Provincia de Camagüey"), ("ta", "க\u{bbe}மகுஎய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}మ\u{c3e}గ\u{c4d}వ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องหลวงกามาก\u{e39}เอย\u{e4c}"), ("tr", "Camagüey ili"), ("uk", "Камагуей"), ("ur", "کاماگوئی صوبہ"), ("vi", "Camagüey"), ("zh", "卡马圭省")]),
                        unofficial_name_list: ["Camagüey"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "Las Tunas",
                        country_alpha2: Alpha2::CU,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.0605162), longitude: Some(-76.91820969999999), max_latitude: Some(21.4577833), min_latitude: Some(20.5049499), max_longitude: Some(-76.3107757), min_longitude: Some(-77.831835)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Las Tunas"), ("ar", "مقاطعة لاس توناس"), ("be", "Правінцыя Лас-Тунас"), ("bg", "Лас Тунас"), ("bn", "ল\u{9be}স ত\u{9c1}ন\u{9be}স প\u{9cd}রদেশ"), ("ca", "Província de Las Tunas"), ("ccp", "𑄣𑄌\u{11134} 𑄑\u{11128}𑄅\u{1112a}𑄚𑄌\u{11134}"), ("ceb", "Las Tunas"), ("da", "Las Tunas Provinsen"), ("de", "Provinz Las Tunas"), ("el", "Λας Τούνας"), ("en", "Las Tunas"), ("es", "Provincia de Las Tunas"), ("eu", "Las Tunasko probintzia"), ("fa", "استان لاس توناس"), ("fi", "Las Tunasin provinssi"), ("fr", "province de Las Tunas"), ("gl", "Provincia de Las Tunas"), ("gu", "લાસ ટ\u{acd}ય\u{ac1}નાસ પ\u{acd}રા\u{a82}ત"), ("hi", "लास ट\u{94d}य\u{942}नास प\u{94d}रा\u{902}त"), ("hr", "Las Tunas"), ("hu", "Las Tunas"), ("id", "Provinsi Las Tunas"), ("it", "provincia di Las Tunas"), ("ja", "ラス・トゥーナス州"), ("ka", "ლას-ტუნასის პროვინცია"), ("kn", "ಲಾಸ\u{ccd} ಟುನಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라스투나스 주"), ("lt", "Las Tunaso provincija"), ("lv", "Lastunasas province"), ("mk", "Лас Тунас"), ("mr", "लास ट\u{941}नस प\u{94d}रा\u{902}त"), ("ms", "Daerah Las Tunas"), ("nb", "Las Tunas"), ("nl", "Las Tunas"), ("no", "Las Tunas"), ("pl", "Prowincja Las Tunas"), ("pt", "Las Tunas"), ("ru", "Лас-Тунас"), ("si", "ල\u{dcf}ස\u{dca} ට\u{dd4}න\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Las Tunas"), ("ta", "ல\u{bbe}ஸ\u{bcd} டுன\u{bbe}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}స\u{c4d} టూన\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาสต\u{e39}วาส"), ("tr", "Las Tunas ili"), ("uk", "Лас-Тунас"), ("ur", "لاس ٹناس صوبہ"), ("vi", "Las Tunas"), ("zh", "拉斯图纳斯省")]),
                        unofficial_name_list: ["Las Tunas"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "Holguín",
                        country_alpha2: Alpha2::CU,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.7837893), longitude: Some(-75.8069082), max_latitude: Some(21.2499624), min_latitude: Some(20.392118), max_longitude: Some(-74.7155897), min_longitude: Some(-76.7217541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هولغوين"), ("az", "Olqin"), ("be", "Альгін"), ("bg", "Олгин"), ("bn", "হলগ\u{9c1}ইন প\u{9cd}রদেশ"), ("ca", "Província d’Holguín"), ("ccp", "𑄦\u{1112e}𑄣\u{11134}𑄉\u{1112a}𑄃\u{11128}𑄚\u{11134}"), ("ceb", "Holguín"), ("cs", "Holguín"), ("da", "Holguín Provinsen"), ("de", "Provinz Holguín"), ("el", "Ολγκίν"), ("en", "Holguín"), ("es", "Provincia de Holguín"), ("et", "Holguíni provints"), ("eu", "Holguíngo probintzia"), ("fa", "استان اولگین"), ("fi", "Holguín"), ("fr", "province de Holguín"), ("gl", "Provincia de Holguín"), ("gu", "હોલ\u{acd}ગિન પ\u{acd}રા\u{a82}ત"), ("hi", "होलग\u{94d}विन प\u{94d}रा\u{902}त"), ("hr", "Holguín"), ("hu", "Holguín"), ("id", "Provinsi Holguín"), ("it", "provincia di Holguín"), ("ja", "オルギン州"), ("ka", "ოლგინის პროვინცია"), ("kn", "ಹೊಲ\u{ccd}ಗುಯ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "올긴 주"), ("lt", "Holgino provincija"), ("lv", "Holginas province"), ("mk", "Олгин"), ("mr", "होल\u{94d}गिन प\u{94d}रा\u{902}त"), ("ms", "Daerah Holguín"), ("nb", "Holguín"), ("nl", "Holguín"), ("no", "Holguín"), ("pl", "Prowincja Holguín"), ("pt", "Holguín"), ("ru", "Ольгин"), ("si", "හොල\u{dca}ග\u{dd4}ය\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sr", "Покрајина Олгин"), ("sr_Latn", "Pokrajina Olgin"), ("sv", "Provincia de Holguín"), ("ta", "ஹொல\u{bcd}குயின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హ\u{c4b}ల\u{c4d}గ\u{c4d}వ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโฮลกว\u{e34}น"), ("tr", "Holguín ili"), ("uk", "Ольгін"), ("ur", "اولگین صوبہ"), ("vi", "Holguín"), ("zh", "奥尔金省")]),
                        unofficial_name_list: ["Holguín"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "Granma",
                        country_alpha2: Alpha2::CU,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.3844902), longitude: Some(-76.64127119999999), max_latitude: Some(20.7823959), min_latitude: Some(19.8280831), max_longitude: Some(-76.1810461), min_longitude: Some(-77.73912)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Granma"), ("ar", "غرانما"), ("be", "Правінцыя Гранма"), ("bg", "Гранма"), ("bn", "গ\u{9cd}র\u{9be}নম\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Granma"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11134}𑄟"), ("ceb", "Granma"), ("da", "Granma Provinsen"), ("de", "Provinz Granma"), ("el", "Γκράνμα"), ("en", "Granma"), ("es", "Provincia de Granma"), ("et", "Granma provints"), ("eu", "Granmako probintzia"), ("fa", "استان گرانما"), ("fi", "Granma"), ("fr", "province de Granma"), ("gl", "Provincia de Granma"), ("gu", "ગ\u{acd}રાનમા પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{94d}रानमा प\u{94d}रा\u{902}त"), ("hr", "Granma"), ("hu", "Granma"), ("id", "Provinsi Granma"), ("it", "provincia di Granma"), ("ja", "グランマ州"), ("ka", "გრანმის პროვინცია"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾನ\u{ccd}ಮಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "그란마 주"), ("lt", "Granmos provincija"), ("lv", "Granmas province"), ("mk", "Гранма"), ("mr", "ग\u{94d}र\u{945}नमा प\u{94d}रा\u{902}त"), ("ms", "Granma Province"), ("nb", "Granma"), ("nl", "Granma"), ("no", "Granma"), ("pl", "Prowincja Granma"), ("pt", "Granma"), ("ru", "Гранма"), ("si", "ග\u{dca}\u{200d}රන\u{dca}ම\u{dcf} පළ\u{dcf}ත"), ("sr", "Покрајина Гранма"), ("sr_Latn", "Pokrajina Granma"), ("sv", "Provincia Granma"), ("ta", "கிர\u{bbe}ன\u{bcd}ம\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}ర\u{c3e}న\u{c4d}మ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อนาลาแมนกา"), ("tr", "Granma ili"), ("uk", "Гранма"), ("ur", "گرانما صوبہ"), ("vi", "Granma"), ("zh", "格拉玛省")]),
                        unofficial_name_list: ["Granma"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "Santiago de Cuba",
                        country_alpha2: Alpha2::CU,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.01693), longitude: Some(-75.8301537), max_latitude: Some(20.0909996), min_latitude: Some(19.9578529), max_longitude: Some(-75.7598926), min_longitude: Some(-75.8923008)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتياغو دي كوبا"), ("bg", "Сантяго де Куба"), ("bn", "স\u{9be}ন\u{9cd}তিয\u{9bc}\u{9be}গো দে কিউব\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Santiago de Cuba"), ("ccp", "𑄥𑄚\u{11134}𑄑\u{11128}𑄠𑄉\u{1112e} 𑄓𑄬 𑄇\u{11128}𑄅\u{1112a}𑄝"), ("ceb", "Santiago de Cuba"), ("cs", "Santiago de Cuba"), ("da", "Santiago de Cuba Provinsen"), ("de", "Provinz Santiago de Cuba"), ("el", "Σαντιάγο ντε Κούβα"), ("en", "Santiago de Cuba"), ("es", "Provincia de Santiago de Cuba"), ("eu", "Santiago de Cubako probintzia"), ("fa", "استان سانتیاگو د کوبا"), ("fi", "Santiago de Cuba"), ("fr", "province de Santiago de Cuba"), ("gl", "Provincia de Santiago de Cuba"), ("gu", "સ\u{ac7}ન\u{acd}ટિયાગો ડ\u{ac7} ક\u{acd}ય\u{ac1}બા પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}\u{902}टियागो डी क\u{94d}य\u{942}बा प\u{94d}रा\u{902}त"), ("hr", "Santiago de Cuba"), ("id", "Provinsi Santiago de Cuba"), ("it", "provincia di Santiago di Cuba"), ("ja", "サンティアーゴ・デ・クーバ州"), ("ka", "სანტიაგო-დე-კუბის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾಂಟ\u{cbf}ಯಾಗೊ ಡ\u{cc6} ಕ\u{ccd}ಯ\u{cc2}ಬಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산티아고데쿠바 주"), ("lt", "Kubos Santjago provincija"), ("lv", "Santjago de Kubas province"), ("mk", "Сантјаго де Куба"), ("mr", "स\u{945}\u{902}टियागो द\u{947} क\u{94d}य\u{942}बा प\u{94d}रा\u{902}त"), ("ms", "Santiago de Cuba Province"), ("nb", "Santiago de Cuba"), ("nl", "Santiago de Cuba"), ("no", "Santiago de Cuba"), ("pl", "Prowincja Santiago de Cuba"), ("pt", "Santiago de Cuba"), ("ru", "Сантьяго-де-Куба"), ("si", "සැන\u{dca}ට\u{dd2}ය\u{dcf}ගෝ ඩ\u{dd2} ක\u{dd2}ය\u{dd4}බ\u{dcf}"), ("sv", "Provincia de Santiago de Cuba"), ("ta", "ச\u{bbe}ண\u{bcd}டிய\u{bbe}கோ டே குப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3f}య\u{c3e}గ\u{c4b} డ\u{c3f} క\u{c4d}యూబ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานเต\u{e35}ยโกเดก\u{e39}บา"), ("tr", "Santiago de Cuba ili"), ("uk", "Сантьяго-де-Куба"), ("ur", "سانتیاگو دے کیوبا صوبہ"), ("vi", "Santiago de Cuba"), ("zh", "圣地亚哥省")]),
                        unofficial_name_list: ["Santiago de Cuba"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "Guantánamo",
                        country_alpha2: Alpha2::CU,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.136667), longitude: Some(-75.213889), max_latitude: Some(20.1796493), min_latitude: Some(20.1143527), max_longitude: Some(-75.1662037), min_longitude: Some(-75.2367424)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غوانتانامو"), ("be", "Правінцыя Гуантанама"), ("bg", "Гуантанамо"), ("ca", "Província de Guantánamo"), ("ccp", "𑄉\u{1112a}𑄠𑄚\u{11134}𑄑𑄚𑄟\u{1112e}"), ("ceb", "Guantánamo"), ("da", "Guantanamo-provinsen"), ("de", "Provinz Guantánamo"), ("en", "Guantánamo"), ("es", "Provincia de Guantánamo"), ("eu", "Guantánamoko probintzia"), ("fa", "استان گوانتانامو"), ("fr", "province de Guantánamo"), ("gl", "Provincia de Guantánamo"), ("hr", "Guantánamo"), ("hu", "Guantánamo"), ("id", "Provinsi Guantánamo"), ("it", "provincia di Guantánamo"), ("ja", "グァンタナモ州"), ("ka", "გუანტანამოს პროვინცია"), ("ko", "관타나모 주"), ("lt", "Gvantanamo provincija"), ("mk", "Гвантанамо"), ("nb", "Guantánamo"), ("nl", "Guantánamo"), ("no", "Guantánamo"), ("pl", "Prowincja Guantánamo"), ("pt", "Guantánamo"), ("ru", "Гуантанамо"), ("sv", "Provincia de Guantánamo"), ("tr", "Guantánamo ili"), ("uk", "Гуантанамо"), ("ur", "گوانتانامو صوبہ"), ("vi", "Guantánamo"), ("zh", "关塔那摩省")]),
                        unofficial_name_list: ["Guantánamo"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "Artemisa",
                        country_alpha2: Alpha2::CU,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.7522903), longitude: Some(-82.99316069999999), max_latitude: Some(23.068672), min_latitude: Some(22.4866999), max_longitude: Some(-82.39349399999999), min_longitude: Some(-83.3889281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Artemisa"), ("ar", "أرتيميسا"), ("bg", "Артемиса"), ("bn", "আর\u{9cd}টেমিস\u{9be} প\u{9cd}রদেশ"), ("ca", "província d’Artemisa"), ("ccp", "𑄃𑄢\u{11134}𑄑𑄬𑄟\u{11128}𑄥"), ("ceb", "Artemisa"), ("da", "Artemisa Province"), ("de", "Provinz Artemisa"), ("el", "Αρτέμισα"), ("en", "Artemisa"), ("es", "Provincia de Artemisa"), ("eu", "Artemisako probintzia"), ("fa", "استان آرتمیسا"), ("fi", "Artemisa"), ("fr", "province d’Artemisa"), ("gu", "આર\u{acd}ટ\u{ac7}મિસા પ\u{acd}રા\u{a82}ત"), ("hi", "आर\u{94d}ट\u{947}मिस प\u{94d}रा\u{902}त"), ("hr", "Artemisa"), ("hu", "Artemisa"), ("id", "Provinsi Artemisa"), ("it", "provincia di Artemisa"), ("ja", "アルテミサ州"), ("ka", "არტემისის პროვინცია"), ("kn", "ಆರ\u{ccd}ಟ\u{cc6}ಮ\u{cbf}ಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아르테미사 주"), ("lt", "Artemisos provincija"), ("lv", "Artemisas province"), ("mk", "Артемиса"), ("mr", "आर\u{94d}टम\u{947}सा प\u{94d}रा\u{902}त"), ("ms", "Artemisa Province"), ("nb", "Artemisa provins"), ("nl", "Artemisa"), ("no", "Artemisa provins"), ("pl", "Prowincja Artemisa"), ("pt", "Artemisa"), ("ru", "Артемиса"), ("si", "අර\u{dca}ටෙම\u{dd2}ස\u{dcf} පළ\u{dcf}ත"), ("sv", "Artemisa"), ("ta", "ஆர\u{bcd}டெமிச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆర\u{c4d}ట\u{c46}మ\u{c40}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ท\u{e35}เม\u{e35}ย"), ("tr", "Artemisa ili"), ("uk", "Артеміса"), ("ur", "آرتیمیسا صوبہ"), ("vi", "Artemisa"), ("zh", "阿爾特米薩省")]),
                        unofficial_name_list: ["Artemisa"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "Mayabeque",
                        country_alpha2: Alpha2::CU,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.8926529), longitude: Some(-81.9534815), max_latitude: Some(23.1876818), min_latitude: Some(22.5725), max_longitude: Some(-81.58065789999999), min_longitude: Some(-82.45682599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mayabeque"), ("ar", "محافظة مايابيك"), ("bg", "Маябеке"), ("bn", "ম\u{9be}য\u{9bc}\u{9be}বিক প\u{9cd}রদেশ"), ("ccp", "𑄟𑄠𑄝𑄬𑄇\u{11134}"), ("ceb", "Provincia Mayabeque"), ("da", "Mayabeque Province"), ("de", "Provinz Mayabeque"), ("el", "Μαγιαμπέκ"), ("en", "Mayabeque"), ("es", "Provincia de Mayabeque"), ("eu", "Mayabequeko probintzia"), ("fa", "استان مایابکه"), ("fi", "Mayabequen lääni"), ("fr", "province de Mayabeque"), ("gu", "માયાબ\u{ac7}ક પ\u{acd}રા\u{a82}ત"), ("hi", "म\u{947}ब\u{947}क प\u{94d}रा\u{902}त"), ("hr", "Mayabeque"), ("hu", "Mayabeque"), ("id", "Provinsi Mayabeque"), ("it", "provincia di Mayabeque"), ("ja", "マヤベケ州"), ("ka", "მაიაბეკეს პროვინცია"), ("kn", "ಮಾಯಾಬ\u{cc6}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마야베케 주"), ("lt", "Majabekės provincija"), ("lv", "Majabekes province"), ("mk", "Мајабеке"), ("mr", "मयाब\u{947}क प\u{94d}रा\u{902}त"), ("ms", "Mayabeque Province"), ("nb", "Mayabeque provins"), ("nl", "Mayabeque"), ("no", "Mayabeque provins"), ("pl", "Prowincja Mayabeque"), ("pt", "Mayabeque"), ("ru", "Маябеке"), ("si", "මයබෙක\u{dd2}ය\u{dd4} පළ\u{dcf}ත"), ("sv", "Provincia Mayabeque"), ("ta", "மயபேயூ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c47}యబ\u{c46}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เดนก\u{e39}วเล"), ("tr", "Mayabeque ili"), ("uk", "Маябеке"), ("ur", "مایابیکوے صوبہ"), ("vi", "Mayabeque"), ("zh", "瑪雅貝克省")]),
                        unofficial_name_list: ["Mayabeque"].to_vec(),
                    }
                ),
                (
                    "99",
                    Subdivision{
                        name: "Isla de la Juventud",
                        country_alpha2: Alpha2::CU,
                        code: "99",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.6066405), longitude: Some(-82.82097399999999), max_latitude: Some(21.9435016), min_latitude: Some(21.4382502), max_longitude: Some(-82.54073369999999), min_longitude: Some(-83.1925464)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جزيرة الشباب"), ("be", "Востраў Хувентуд"), ("bg", "Хувентуд"), ("bn", "ইসল ডে ল\u{9be} জ\u{9c1}ভেন\u{9cd}ট\u{9c1}ড"), ("ca", "Illa de la Juventud"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄣 𑄓𑄬 𑄣 𑄎\u{1112a}𑄞𑄬𑄚\u{11134}𑄑𑄖\u{11134}"), ("ceb", "Isla de la Juventud"), ("cs", "ostrov Mládeže"), ("da", "Isla de la Juventud"), ("de", "Isla de la Juventud"), ("el", "Ίσλα δε λα Χουβεντούδ"), ("en", "Isla de la Juventud"), ("es", "Isla de la Juventud"), ("et", "Juventud"), ("eu", "Isla de la Juventud"), ("fa", "ایسلا د لا خوبنتود"), ("fi", "Isla de la Juventud"), ("fr", "île de la Jeunesse"), ("gl", "Illa da Juventud"), ("gu", "ઈસ\u{acd}લા ડ\u{ac7} લા જ\u{ac1}વ\u{ac7}ન\u{acd}ટ\u{ac1}ડ"), ("he", "איסלה דה לה חובנטוד"), ("hi", "आईला डी ला उव\u{947}\u{902}त\u{941}ड"), ("hr", "Isla de la Juventud"), ("hu", "Isla de la Juventud"), ("hy", "Խուվենտուդ"), ("id", "Isla de la Juventud"), ("it", "Isola della Gioventù"), ("ja", "青年の島"), ("ka", "ხუვენტუდი"), ("kk", "Хувентуд"), ("kn", "ಇಸ\u{ccd}ಲಾ ಡ\u{cbf} ಲಾ ಜುವ\u{cc6}ಂಡುಡ\u{ccd}"), ("ko", "후벤투드 섬"), ("lt", "Chuventudo sala"), ("lv", "Huventudas sala"), ("mk", "Исла де ла Хувентуд"), ("mr", "इस\u{94d}ला द\u{947} ला जाव\u{947}\u{902}ट\u{94d}य\u{942}ड"), ("ms", "Isla de la Juventud"), ("nb", "Isla de la Juventud"), ("nl", "Isla de la Juventud"), ("no", "Isla de la Juventud"), ("pl", "Isla de la Juventud"), ("pt", "Ilha da Juventude"), ("ru", "Хувентуд"), ("si", "ඉස\u{dca}ල\u{dcf} ඩෙ ල\u{dcf} ජ\u{dd4}වෙන\u{dca}ට\u{dd4}ඩ\u{dca}"), ("sv", "Isla de la Juventud"), ("sw", "Isla de la Juventud"), ("ta", "இஸ\u{bcd}ல\u{bbe} டி ல\u{bbe} ஜுவேண\u{bcd}டுட\u{bcd}"), ("te", "ఇస\u{c4d}ల\u{c3e} ద ల\u{c3e} జువ\u{c46}ంటుడ\u{c4d}"), ("th", "เทศบาลอ\u{e34}สลาเดลาค\u{e39}เบนต\u{e38}ด"), ("tr", "Isla de la Juventud"), ("uk", "Ісла-де-ла-Хувентуд"), ("ur", "ازلا دے لا خوبینتود"), ("vi", "Isla de la Juventud"), ("zh", "青年岛")]),
                        unofficial_name_list: ["Isla de la Juventud"].to_vec(),
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
#[cfg(feature = "cu")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CU,
        alpha3: Alpha3::CUB,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 53,
        currency_code: CurrencyCode::CUP,
        gec: Some(GEC::CU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "119",
        ioc: Some(IOC::CUB),
        iso_long_name: "The Republic of Cuba",
        iso_short_name: "Cuba",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Cuban"),
        number: "192",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "CU",
        unofficial_name_list: ["Cuba", "Kuba", "キューバ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Cuba"),
            ("af", "Kuba"),
            ("ak", "Cuba"),
            ("am", "Cuba"),
            ("an", "Cuba"),
            ("ar", "كوبا"),
            ("as", "কিউব\u{9be}"),
            ("ay", "Cuba"),
            ("az", "Kuba"),
            ("ba", "Cuba"),
            ("be", "Куба"),
            ("bg", "Куба"),
            ("bi", "Cuba"),
            ("bn", "কিউব\u{9be}"),
            ("bn_IN", "কিউব\u{9be}"),
            ("br", "Kuba"),
            ("bs", "Kuba"),
            ("ca", "Cuba"),
            ("ce", "Куба"),
            ("ch", "Cuba"),
            ("cs", "Kuba"),
            ("cv", "Куба"),
            ("cy", "Ciwba"),
            ("da", "Cuba"),
            ("de", "Kuba"),
            ("dv", "ކ\u{7a8}އ\u{7aa}ބ\u{7a7}"),
            ("dz", "ཀ\u{f74}ས་བ།"),
            ("ee", "Cuba"),
            ("el", "Κούβα"),
            ("en", "Cuba"),
            ("eo", "Kubo"),
            ("es", "Cuba"),
            ("et", "Kuuba"),
            ("eu", "Kuba"),
            ("fa", "کوبا"),
            ("ff", "Cuba"),
            ("fi", "Kuuba"),
            ("fo", "Kuba"),
            ("fr", "Cuba"),
            ("fy", "Kuba"),
            ("ga", "Cúba"),
            ("gl", "Cuba"),
            ("gn", "Cuba"),
            ("gu", "ક\u{acd}ય\u{ac1}બા"),
            ("gv", "Yn Choobey"),
            ("ha", "Cuba"),
            ("he", "קובה"),
            ("hi", "क\u{94d}य\u{942}बा"),
            ("hr", "Kuba"),
            ("ht", "Kiba"),
            ("hu", "Kuba"),
            ("hy", "Կուբա"),
            ("ia", "Cuba"),
            ("id", "Kuba"),
            ("io", "Kuba"),
            ("is", "Kúba"),
            ("it", "Cuba"),
            ("iu", "ᖂᐹ/quupaa"),
            ("ja", "キューバ"),
            ("ka", "კუბა"),
            ("ki", "Cuba"),
            ("kk", "Куба"),
            ("kl", "Cuba"),
            ("km", "គ\u{17bb}យបា"),
            ("kn", "ಕ\u{ccd}ಯ\u{cc2}ಬಾ"),
            ("ko", "쿠바"),
            ("ku", "Kûba"),
            ("kv", "Куба"),
            ("kw", "Kuba"),
            ("ky", "Куба"),
            ("lo", "Cuba"),
            ("lt", "Kuba"),
            ("lv", "Kuba"),
            ("mi", "Kūpā"),
            ("mk", "Куба"),
            ("ml", "ക\u{d4d}യ\u{d42}ബ"),
            ("mn", "Куба"),
            ("mr", "क\u{94d}य\u{942}बा"),
            ("ms", "Cuba"),
            ("mt", "Kuba"),
            ("my", "ကျ\u{1030}းဘားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Kiuba"),
            ("nb", "Cuba"),
            ("ne", "क\u{94d}य\u{942}बा"),
            ("nl", "Cuba"),
            ("nn", "Cuba"),
            ("nv", "Cuba"),
            ("oc", "Cuba"),
            ("or", "କ\u{b4d}ଯ\u{b41}ବ\u{b3e}"),
            ("pa", "ਕਿਊਬਾ"),
            ("pi", "क\u{94d}य\u{942}बा"),
            ("pl", "Kuba"),
            ("ps", "کیوبا"),
            ("pt", "Cuba"),
            ("pt_BR", "Cuba"),
            ("ro", "Cuba"),
            ("ru", "Куба"),
            ("rw", "Kiba"),
            ("sc", "Cuba"),
            ("sd", "ڪيوبا"),
            ("si", "ක\u{dd2}ය\u{dd4}බ\u{dcf}"),
            ("sk", "Kuba"),
            ("sl", "Kuba"),
            ("so", "Kuuba"),
            ("sq", "Kubë"),
            ("sr", "Куба"),
            ("sv", "Kuba"),
            ("sw", "Cuba"),
            ("ta", "க\u{bcd}யூப\u{bbe}"),
            ("te", "క\u{c4d}యూబ\u{c3e}"),
            ("tg", "Куба"),
            ("th", "ค\u{e34}วบา"),
            ("ti", "ኩባ"),
            ("tk", "Kuba"),
            ("tl", "Kuba"),
            ("tr", "Küba"),
            ("tt", "Куба"),
            ("ug", "كۇبا"),
            ("uk", "Куба"),
            ("ur", "کیوبا"),
            ("uz", "Kuba"),
            ("ve", "Cuba"),
            ("vi", "Cu-ba"),
            ("wa", "Cuba"),
            ("wo", "Kubaa"),
            ("xh", "Cuba"),
            ("yo", "Kúbà"),
            ("zh_CN", "古巴"),
            ("zh_HK", "古巴"),
            ("zh_TW", "古巴"),
            ("zu", "Cuba"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
