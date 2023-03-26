// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Kazakhstan

#[cfg(all(feature = "kz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::KZ;
    pub const ALPHA3: Alpha3 = Alpha3::KAZ;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 7;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::KZT;
    pub const GEC: Option<GEC> = Some(GEC::KZ);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::KAZ);
    pub const ISO_SHORT_NAME: &str = "Kazakhstan";
    pub const ISO_LONG_NAME: &str = "The Republic of Kazakhstan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["kk", "ru"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["kk", "ru"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Kazakhstani");
    pub const NUMBER: &str = "398";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAsia);
    pub const UN_LOCODE: &str = "KZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Kazakhstan",
        "Kasachstan",
        "Kazajistán",
        "カザフスタン",
        "Kazachstan",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Kazakhstan"),
        ("af", "Kazakstan"),
        ("ak", "Kazakhstan"),
        ("am", "ካዛክስታን"),
        ("an", "Kazakhstan"),
        ("ar", "كازاخستان"),
        ("as", "ক\u{9be}জ\u{9be}খিস\u{9cd}ত\u{9be}ন"),
        ("ay", "Kazakhstan"),
        ("az", "Qazaxstan"),
        ("ba", "Kazakhstan"),
        ("be", "Казахстан"),
        ("bg", "Казахстан"),
        ("bi", "Kazakhstan"),
        ("bn", "ক\u{9be}জ\u{9be}খস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "ক\u{9be}জ\u{9be}খস\u{9cd}ত\u{9be}ন"),
        ("br", "Kazakstan"),
        ("bs", "Kazahstan"),
        ("ca", "Kazakhstan"),
        ("ce", "Кхазакхстан"),
        ("ch", "Kazakhstan"),
        ("cs", "Kazachstán"),
        ("cv", "Кхазакхстан"),
        ("cy", "Kazakhstan"),
        ("da", "Kasakhstan"),
        ("de", "Kasachstan"),
        ("dv", "ކ\u{7a6}ޒ\u{7a6}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
        ("dz", "ཀ་ཛཀས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Kazakhstan"),
        ("el", "Καζακστάν"),
        ("en", "Kazakhstan"),
        ("eo", "Kazaĥio"),
        ("es", "Kazajistán"),
        ("et", "Kasahstan"),
        ("eu", "Kazakhstan"),
        ("fa", "قزاقستان"),
        ("ff", "Kazakhstan"),
        ("fi", "Kazakstan"),
        ("fo", "Kasakstan"),
        ("fr", "Kazakhstan"),
        ("fy", "Kazakstan"),
        ("ga", "An Chasacstáin"),
        ("gl", "Kazakhstán"),
        ("gn", "Kazakhstan"),
        ("gu", "કઝાખસ\u{acd}તાન"),
        ("gv", "Yn Chassaghstaan"),
        ("ha", "Kazakystan"),
        ("he", "קזחסטן"),
        ("hi", "कज\u{93c}ाख\u{93c}िस\u{94d}तान"),
        ("hr", "Kazahstan"),
        ("ht", "Kazakstan"),
        ("hu", "Kazahsztán"),
        ("hy", "Ղազախստան"),
        ("ia", "Kazakhstan"),
        ("id", "Kazakhstan"),
        ("io", "Kazakstan"),
        ("is", "Kasakstan"),
        ("it", "Kazakistan"),
        ("iu", "ᑲᓴᒃᖢᓇ ᓄᓇ"),
        ("ja", "カザフスタン"),
        ("ka", "ყაზახეთი"),
        ("ki", "Kazakhstan"),
        ("kk", "Қазақстан"),
        ("kl", "Kazakhstan"),
        ("km", "កាហ\u{17d2}សាក\u{17cb}ស\u{17d2}តង\u{17cb}"),
        ("kn", "ಕಝಕಸ\u{ccd}ತಾನ\u{ccd}"),
        ("ko", "카자흐스탄"),
        ("ku", "Kazaxstan"),
        ("kv", "Казахстан"),
        ("kw", "Pow Kazagh"),
        ("ky", "Казакстан"),
        ("lo", "Kazakhstan"),
        ("lt", "Kazachstanas"),
        ("lv", "Kazahstāna"),
        ("mi", "Katatānga"),
        ("mk", "Казакстан"),
        ("ml", "കസ\u{d3e}ഖിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Казакстан"),
        ("mr", "कझाकिस\u{94d}तान"),
        ("ms", "Kazakhstan"),
        ("mt", "Każakistan"),
        (
            "my",
            "ကာဇက\u{103a}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Kadaketan"),
        ("nb", "Kasakhstan"),
        ("ne", "काजखास\u{94d}तान"),
        ("nl", "Kazachstan"),
        ("nn", "Kasakhstan"),
        ("nv", "Bilį\u{301}į\u{301}ʼ Ńdeiltihii Dineʼé Bikéyah"),
        ("oc", "Cazacstan"),
        ("or", "କ\u{b3e}ଜ\u{b3e}ଖସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਕਜ\u{a3c}ਾਕਸਤਾਨ"),
        ("pi", "कजाकस\u{94d}थान"),
        ("pl", "Kazachstan"),
        ("ps", "قزاقستان"),
        ("pt", "Cazaquistão"),
        ("pt_BR", "Cazaquistão"),
        ("ro", "Kazakhstan"),
        ("ru", "Казахстан"),
        ("rw", "Kazakisitani"),
        ("sc", "Kazakistan"),
        ("sd", "قازڪستان"),
        ("si", "කසකස\u{dca}ත\u{dcf}නය"),
        ("sk", "Kazachstan"),
        ("sl", "Kazahstan"),
        ("so", "Kasaakhistaan"),
        ("sq", "Kazakistan"),
        ("sr", "Казахстан"),
        ("sv", "Kazakstan"),
        ("sw", "Kazakhstan"),
        ("ta", "கசகிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
        ("te", "కఝక\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Қазоқистон"),
        ("th", "คาซ\u{e31}คสถาน"),
        ("ti", "Kazakhstan"),
        ("tk", "Gazagystan"),
        ("tl", "Kazakhstan"),
        ("tr", "Kazakistan"),
        ("tt", "Казакстан"),
        ("ug", "قازاقىستان"),
        ("uk", "Казахстан"),
        ("ur", "قازقستان"),
        ("uz", "Qozogʻiston"),
        ("ve", "Kazakhstan"),
        ("vi", "Kha-xa-kh-x-thanh"),
        ("wa", "Kazaxhtan"),
        ("wo", "Kaasakestaan"),
        ("xh", "Kazakhstan"),
        ("yo", "Kàsàkstán"),
        ("zh_CN", "哈萨克斯坦"),
        ("zh_HK", "哈薩克"),
        ("zh_TW", "哈薩克"),
        ("zu", "Kazakhstan"),
    ];
    #[cfg(all(feature = "kz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 48.019573;
        pub const LONGITUDE: f64 = 66.923684;
        pub const MAX_LATITUDE: f64 = 55.4419839;
        pub const MAX_LONGITUDE: f64 = 87.315415;
        pub const MIN_LATITUDE: f64 = 40.5685841;
        pub const MIN_LONGITUDE: f64 = 46.493672;
        pub const NORTHEAST_LATITUDE: f64 = 55.4419839;
        pub const NORTHEAST_LONGITUDE: f64 = 87.315415;
        pub const SOUTHWEST_LATITUDE: f64 = 40.5685841;
        pub const SOUTHWEST_LONGITUDE: f64 = 46.493672;
    }
}
#[cfg(all(feature = "kz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 48.019573,
            longitude: 66.923684,
            max_latitude: 55.4419839,
            max_longitude: 87.315415,
            min_latitude: 40.5685841,
            min_longitude: 46.493672,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 55.4419839,
                    longitude: 87.315415,
                },
                southwest: CountryGeoBound {
                    latitude: 40.5685841,
                    longitude: 46.493672,
                },
            },
        }
    }
}

#[cfg(all(feature = "kz", feature = "subdivisions"))]
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
                    "AKM",
                    Subdivision{
                        name: "AKM",
                        country_alpha2: Alpha2::KZ,
                        code: "AKM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.916532), longitude: Some(69.4110493), max_latitude: Some(53.708545), min_latitude: Some(50.111386), max_longitude: Some(74.19755889999999), min_longitude: Some(65.2571501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس أكمولا"), ("az", "Aqmola vilayəti"), ("be", "Акмалінская вобласць"), ("bg", "Акмолинска област"), ("bn", "আকম\u{9c1}ল\u{9be} অঞ\u{9cd}চল"), ("ca", "Província d’Akmolà"), ("ccp", "𑄃𑄇\u{11134}𑄟\u{11127}𑄣"), ("ceb", "Aqmola Oblysy"), ("cs", "Akmolská oblast"), ("da", "Akmola Region"), ("de", "Aqmola"), ("el", "Ακμόλα"), ("en", "Akmola"), ("es", "Provincia de Akmola"), ("et", "Akmola oblast"), ("eu", "Aqmola"), ("fa", "استان اکمولا"), ("fi", "Aqmolan alue"), ("fr", "Oblys d’Aqmola"), ("gu", "અક\u{acd}મોલા પ\u{acd}રદ\u{ac7}શ"), ("he", "אקמולה (מחוז)"), ("hi", "अकमोला प\u{94d}रा\u{902}त"), ("hy", "Աքմոլայի մարզ"), ("id", "Provinsi Aqmola"), ("is", "Aqmolafylki"), ("it", "Regione di Aqmola"), ("ja", "アクモラ州"), ("ka", "აკმოლის ოლქი"), ("kk", "Ақмола облысы"), ("kn", "ಅಕ\u{ccd}ಮೋಲಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "아크몰라 주"), ("ky", "Акмоло облусу"), ("lt", "Akmolos sritis"), ("lv", "Akmolas apgabals"), ("mk", "Акмола"), ("mr", "अकमोला"), ("ms", "Wilayah Akmola"), ("nb", "Akmola region"), ("nl", "Oblast Aqmola"), ("no", "Akmola region"), ("pl", "Obwód akmolski"), ("pt", "Aqmola"), ("ro", "Provincia Akmola"), ("ru", "Акмолинская область"), ("si", "අක\u{dca}මොල\u{dcf} කල\u{dcf}පය"), ("sr", "Акмолинска област"), ("sr_Latn", "Akmolinska oblast"), ("sv", "Aqmola"), ("ta", "அஃமோல\u{bbe} பகுதி"), ("te", "అక\u{c4d}మ\u{c4b}ల\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}คโมลา"), ("tr", "Akmola Eyaleti"), ("uk", "Акмолинська область"), ("ur", "اقمولا صوبہ"), ("uz", "Akmola viloyati"), ("vi", "Akmola"), ("zh", "阿克莫拉州")]),
                        unofficial_name_list: ["Akmola", "Akmolinsk", "Aķmola", "Celinograd", "Tselinograd"].to_vec(),
                    }
                ),
                (
                    "AKT",
                    Subdivision{
                        name: "AKT",
                        country_alpha2: Alpha2::KZ,
                        code: "AKT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.7797078), longitude: Some(57.9974377), max_latitude: Some(51.339557), min_latitude: Some(45.162695), max_longitude: Some(64.1807611), min_longitude: Some(53.5261182)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس أكتوبي"), ("az", "Aqtöbe vilayəti"), ("be", "Акцюбінская вобласць"), ("bg", "Актобенска област"), ("bn", "আকত\u{9c1}বে অঞ\u{9cd}চল"), ("ca", "Província d’Aktobé"), ("ccp", "𑄃𑄇\u{11134}𑄑\u{1112e}𑄝𑄬"), ("ceb", "Aktyubinskaya Oblast’"), ("cs", "Akťubinská oblast"), ("da", "Aktjubinsk oblast"), ("de", "Aqtöbe"), ("el", "Περιφέρεια Ακτόμπε"), ("en", "Aktobe"), ("es", "Provincia de Aktobe"), ("et", "Aktöbe oblast"), ("eu", "Aktobe probintzia"), ("fa", "استان اکتوب"), ("fi", "Aqtöben alue"), ("fr", "Oblys d’Aqtöbe"), ("gu", "અક\u{acd}ટોબ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("he", "אקטובה (מחוז)"), ("hi", "अकतोब\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Aqtöbe"), ("is", "Aktöbefylki"), ("it", "Regione di Aqtöbe"), ("ja", "アクトベ州"), ("ka", "აქტობეს ოლქი"), ("kk", "Ақтөбе облысы"), ("kn", "ಅಕ\u{ccd}ಟೋಬ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "악퇴베 주"), ("ky", "Актөбө облусу"), ("lt", "Aktobės sritis"), ("lv", "Aktebes apgabals"), ("mk", "Актобе"), ("mr", "अक\u{94d}तोब\u{947}"), ("ms", "Wilayah Aktobe"), ("nb", "Aqtöbe"), ("nl", "Oblast Aqtöbe"), ("no", "Aqtöbe"), ("pl", "Obwód aktobski"), ("pt", "Aqtöbe"), ("ro", "Provincia Aktobe"), ("ru", "Актюбинская область"), ("si", "අක\u{dca}ටෝබේ කල\u{dcf}පය"), ("sr", "Актјубинска област"), ("sr_Latn", "Aktjubinska oblast"), ("sv", "Aqtöbe"), ("ta", "அக\u{bcd}டொபே பகுதி"), ("te", "అక\u{c4d}ట\u{c4b}బ\u{c46} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}กโตเบ"), ("tr", "Aktöbe eyaleti"), ("uk", "Актюбінська область"), ("ur", "اقتوبے صوبہ"), ("uz", "Aqtoʻbe viloyati"), ("vi", "Aktobe"), ("zh", "阿克托貝州")]),
                        unofficial_name_list: ["Aktjubinsk", "Aktyubinsk", "Aktöbe", "Aktʿubinsk"].to_vec(),
                    }
                ),
                (
                    "ALA",
                    Subdivision{
                        name: "ALA",
                        country_alpha2: Alpha2::KZ,
                        code: "ALA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2220146), longitude: Some(76.8512485), max_latitude: Some(43.4057021), min_latitude: Some(43.0287453), max_longitude: Some(77.1467686), min_longitude: Some(76.7415618)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Almaty"), ("am", "አልማቲ"), ("ar", "ألماتي"), ("az", "Almatı"), ("be", "Алма-Ата"), ("bg", "Алмати"), ("bn", "আলমেটি"), ("bs", "Almaty"), ("ca", "Almati"), ("ccp", "𑄃𑄣\u{11134}𑄟𑄑\u{11128}"), ("ceb", "Almaty (kapital sa lalawigan)"), ("cs", "Almaty"), ("cy", "Almaty"), ("da", "Almaty"), ("de", "Almaty"), ("el", "Αλμάτι"), ("en", "Almaty"), ("es", "Almatý"), ("et", "Almatõ"), ("eu", "Almaty"), ("fa", "آلماآتی"), ("fi", "Almaty"), ("fr", "Almaty"), ("ga", "Almaty"), ("gl", "Almati"), ("gu", "અલ\u{acd}માટી"), ("he", "אלמטי"), ("hi", "अलमाती"), ("hr", "Almati"), ("hu", "Almati"), ("hy", "Ալմաթի"), ("id", "Almaty"), ("is", "Almaty"), ("it", "Almaty"), ("ja", "アルマトイ"), ("ka", "ალმათი"), ("kk", "Алматы"), ("kn", "ಅಲ\u{ccd}ಮಾಟ\u{cbf}"), ("ko", "알마티"), ("ky", "Алматы"), ("lt", "Almata"), ("lv", "Almati"), ("mk", "Алмати"), ("ml", "അൽമ\u{d3e}ട\u{d4d}ടി"), ("mn", "Алматы"), ("mr", "अल\u{94d}माटी"), ("ms", "Almaty"), ("nb", "Almaty"), ("nl", "Alma-Ata"), ("no", "Almaty"), ("pa", "ਅਲਮਾਟੀ"), ("pl", "Ałmaty"), ("pt", "Almaty"), ("ro", "Almatî"), ("ru", "Алматы"), ("si", "අල\u{dca}මට\u{dd2}"), ("sk", "Alma-Ata"), ("sl", "Almati"), ("sq", "Almati"), ("sr", "Алмати"), ("sr_Latn", "Almati"), ("sv", "Almaty"), ("sw", "Almaty"), ("ta", "அல\u{bcd}ம\u{bbe}த\u{bcd}தி"), ("te", "ఆల\u{c4d}మ\u{c3e}ట\u{c40}"), ("th", "อ\u{e31}ลมาต\u{e35}"), ("tk", "Almaty"), ("tr", "Almatı"), ("uk", "Алмати"), ("ur", "الماتی"), ("uz", "Almati"), ("vi", "Almaty"), ("yo", "Almaty"), ("yo_BJ", "Almaty"), ("yue", "阿拉木圖"), ("yue_Hans", "阿拉木图"), ("zh", "阿拉木圖"), ("zu", "Almaty")]),
                        unofficial_name_list: ["Almati Oblasti", "Almaty Oblasty", "Almatı"].to_vec(),
                    }
                ),
                (
                    "ALM",
                    Subdivision{
                        name: "ALM",
                        country_alpha2: Alpha2::KZ,
                        code: "ALM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0119227), longitude: Some(78.4229392), max_latitude: Some(47.329847), min_latitude: Some(42.202049), max_longitude: Some(82.63085889999999), min_longitude: Some(74.0447233)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس ألماطي"), ("az", "Almatı vilayəti"), ("be", "Алмацінская вобласць"), ("bg", "Алматинска област"), ("bn", "আলম\u{9be}টি অঞ\u{9cd}চল"), ("ca", "Província d’Almati"), ("ccp", "𑄃𑄣\u{11134}𑄟𑄑\u{11128} 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Almaty Qalasy"), ("cs", "Almatinská oblast"), ("da", "Almaty Region"), ("de", "Almaty²"), ("el", "Επαρχία Αλμάτι"), ("en", "Almaty Region"), ("es", "Provincia de Almaty"), ("et", "Almatõ oblast"), ("eu", "Almatyko probintzia"), ("fa", "استان الماتی"), ("fi", "Almaty maakunta"), ("fr", "Oblys d’Almaty"), ("gl", "Provincia de Almati"), ("gu", "અલ\u{acd}માટી પ\u{acd}રદ\u{ac7}શ"), ("he", "אלמטי (מחוז)"), ("hi", "अलमाती प\u{94d}रा\u{902}त"), ("hy", "Ալմաթի շրջան"), ("id", "Provinsi Almaty"), ("is", "Almatyfylki"), ("it", "Regione di Almaty"), ("ja", "アルマトイ州"), ("ka", "ალმათის ოლქი"), ("kk", "Алматы облысы"), ("kn", "ಅಲ\u{ccd}ಮಾಟ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "알마티 주"), ("ky", "Алматы облусу"), ("lt", "Almatos sritis"), ("lv", "Almati apgabals"), ("mk", "Алмати²"), ("mr", "अल\u{94d}माटी²"), ("ms", "Wilayah Almaty"), ("nb", "Almaty region"), ("nl", "Oblast Almaty"), ("no", "Almaty region"), ("pa", "ਅਲਮਾਟੀ ਸ\u{a42}ਬਾ"), ("pl", "Obwód ałmacki"), ("pt", "Almaty²"), ("ro", "Provincia Almatî"), ("ru", "Алматинская область"), ("si", "අල\u{dca}මෙට\u{dd2} කල\u{dcf}පය"), ("sr", "Алматинска област"), ("sr_Latn", "Almatinska oblast"), ("sv", "Almaty²"), ("ta", "அள\u{bcd}ம\u{bbe}டி பகுதி"), ("te", "ఆల\u{c4d}మ\u{c3e}ట\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}ลมาต\u{e35}²"), ("tr", "Almatı Eyaleti"), ("uk", "Алматинська область"), ("ur", "الماتی صوبہ"), ("uz", "Almati²"), ("vi", "Almaty²"), ("zh", "阿拉木圖州")]),
                        unofficial_name_list: ["Almaty oblysy"].to_vec(),
                    }
                ),
                (
                    "AST",
                    Subdivision{
                        name: "AST",
                        country_alpha2: Alpha2::KZ,
                        code: "AST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.16052269999999), longitude: Some(71.4703558), max_latitude: Some(51.2903453), min_latitude: Some(51.0055461), max_longitude: Some(71.7427397), min_longitude: Some(71.2160397)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Astana"), ("am", "አስታና"), ("ar", "أستانا"), ("az", "Astana"), ("be", "Горад Астана"), ("bg", "Астана"), ("bn", "আস\u{9cd}ত\u{9be}ন\u{9be}"), ("bs", "Astana"), ("ca", "Astanà"), ("ccp", "𑄃𑄌\u{11134}𑄑𑄚"), ("ceb", "Astana"), ("cs", "Astana"), ("cy", "Astana"), ("da", "Astana"), ("de", "Astana"), ("el", "Αστανά"), ("en", "Astana"), ("es", "Astaná"), ("et", "Astana"), ("eu", "Astana"), ("fa", "آستانه"), ("fi", "Astana"), ("fr", "Astana"), ("ga", "Astana"), ("gl", "Astana"), ("gu", "અસટાના"), ("he", "אסטנה"), ("hi", "अस\u{94d}ताना"), ("hr", "Astana"), ("hu", "Asztana"), ("hy", "Աստանա"), ("id", "Astana, Kazakhstan"), ("is", "Astana"), ("it", "Astana"), ("ja", "アスタナ"), ("jv", "Astana, Kazakhstan"), ("ka", "ასტანა"), ("kk", "Астана қаласы"), ("kn", "ಅಸ\u{ccd}ತಾನ"), ("ko", "아스타나"), ("ky", "Астана"), ("lt", "Astana"), ("lv", "Astana"), ("mk", "Астана"), ("ml", "അസ\u{d4d}ത\u{d3e}ന"), ("mn", "Астана"), ("mr", "अस\u{94d}ताना"), ("ms", "Astana"), ("nb", "Astana"), ("ne", "अस\u{94d}टाना"), ("nl", "Astana"), ("no", "Astana"), ("or", "ଅସ\u{b4d}ତ\u{b3e}ନ\u{b3e}"), ("pa", "ਅਸਤਾਨਾ"), ("pl", "Astana"), ("ps", "استانه"), ("pt", "Astana"), ("ro", "Astana"), ("ru", "Астана"), ("si", "අස\u{dca}ත\u{dcf}න\u{dcf}"), ("sk", "Astana"), ("sl", "Astana"), ("sq", "Astana"), ("sr", "Астана"), ("sr_Latn", "Astana"), ("sv", "Astana"), ("sw", "Astana"), ("ta", "அஸ\u{bcd}த\u{bbe}ன\u{bbe}"), ("te", "అస\u{c4d}త\u{c3e}న\u{c3e}"), ("th", "อ\u{e31}สตานา"), ("tk", "Astana"), ("tr", "Astana"), ("uk", "Астана"), ("ur", "آستانہ"), ("uz", "Ostona"), ("vi", "Astana"), ("yo", "Astana"), ("yo_BJ", "Astana"), ("yue", "阿斯塔納"), ("yue_Hans", "阿斯塔纳"), ("zh", "阿斯塔納")]),
                        unofficial_name_list: ["Astana"].to_vec(),
                    }
                ),
                (
                    "ATY",
                    Subdivision{
                        name: "ATY",
                        country_alpha2: Alpha2::KZ,
                        code: "ATY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.9053152), longitude: Some(51.3780767), max_latitude: Some(49.270703), min_latitude: Some(46.16998), max_longitude: Some(56.37093609999999), min_longitude: Some(46.994003)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس أتييرو"), ("az", "Atırau vilayəti"), ("be", "Атыраўская вобласць"), ("bg", "Атърауска област"), ("bn", "আতির\u{9be}উ প\u{9cd}রদেশ"), ("ca", "Província d’Atyrau"), ("ccp", "𑄃𑄑\u{11128}𑄠𑄢𑄅\u{1112a}"), ("ceb", "Atyraū Oblysy"), ("cs", "Atyrauská oblast"), ("da", "Atyrau Province"), ("de", "Atyrau"), ("el", "Περιφέρεια Ατιράου"), ("en", "Atyrau"), ("es", "Atyrau"), ("et", "Atõrau oblast"), ("eu", "Atyrau probintzia"), ("fa", "استان آتیرائو"), ("fi", "Atıraw’n alue"), ("fr", "Oblys d’Atyraou"), ("gu", "અતિરાઉ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אטיראו"), ("hy", "Արիտաու մարզ"), ("id", "Provinsi Atyrau"), ("is", "Atýráfylki"), ("it", "Regione di Atyrau"), ("ja", "アティラウ州"), ("ka", "ატირაუს ოლქი"), ("kk", "Атырау облысы"), ("kn", "ಆತ\u{cbf}ರಾವ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아티라우 주"), ("ky", "Атырау облусу"), ("lt", "Atyrau sritis"), ("lv", "Atirau apgabals"), ("mk", "Атирау"), ("mr", "अतिरौ"), ("ms", "Wilayah Atyrau"), ("nb", "Atyrau oblast"), ("nl", "Oblast Atıraw"), ("no", "Atyrau oblast"), ("pl", "Obwód atyrauski"), ("pt", "Atyrau"), ("ro", "Provincia Atîrau"), ("ru", "Атырауская область"), ("si", "අට\u{dd2}ය\u{dcf}ර\u{dd4} පළ\u{dcf}ත"), ("sr", "Атирауска област"), ("sr_Latn", "Atirauska oblast"), ("sv", "Atyraw"), ("ta", "அடிர\u{bbe}வ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అట\u{c3f}ర\u{c3e}వ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e39}ลา"), ("tr", "Atırav Eyaleti"), ("uk", "Атирауська область"), ("ur", "اتیراؤ صوبہ"), ("uz", "Atirau viloyati"), ("vi", "Atyrau"), ("zh", "阿特勞州")]),
                        unofficial_name_list: ["Ateransk", "Aterau", "Atirau", "Atırau", "Gurjev", "Guryev"].to_vec(),
                    }
                ),
                (
                    "KAR",
                    Subdivision{
                        name: "KAR",
                        country_alpha2: Alpha2::KZ,
                        code: "KAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.90221820000001), longitude: Some(71.77068059999999), max_latitude: Some(51.38928199999999), min_latitude: Some(46.001395), max_longitude: Some(77.6211869), min_longitude: Some(62.6005359)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كراغندي"), ("az", "Karaqanda vilayəti"), ("be", "Карагандзінская вобласць"), ("bg", "Карагандинска област"), ("bn", "ক\u{9be}র\u{9be}গ\u{9be}ন\u{9cd}ডি অঞ\u{9cd}চল"), ("ca", "Província de Karaganda"), ("ccp", "𑄇𑄢𑄉𑄚\u{11134}𑄓\u{11128}"), ("ceb", "Qaraghandy Oblysy"), ("cs", "Karagandská oblast"), ("da", "Karagandy Region"), ("de", "Qaraghandy"), ("el", "Περιφέρεια Καραγκάντα"), ("en", "Karagandy"), ("es", "Provincia de Karagandá"), ("et", "Karagandõ oblast"), ("eu", "Karagandy probintzia"), ("fa", "استان قراغندی"), ("fi", "Karakandan alue"), ("fr", "Oblys de Karaganda"), ("gu", "કારાગ\u{a82}ડી પ\u{acd}રદ\u{ac7}શ"), ("he", "קרגנדה (מחוז)"), ("hi", "काराग\u{93c}ान\u{94d}दी प\u{94d}रा\u{902}त"), ("hu", "Karagandi terület"), ("hy", "Կարագանդայի մարզ"), ("id", "Provinsi Qaraghandy"), ("is", "Karagandyfylki"), ("it", "Regione di Qaraǧandy"), ("ja", "カラガンダ州"), ("ka", "ყარაღანდის ოლქი"), ("kk", "Қарағанды облысы"), ("kn", "ಕರಾಗಾಂಡ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "카라간다 주"), ("ky", "Караганды облусу"), ("lt", "Karagandos sritis"), ("lv", "Karaghandi apgabals"), ("mk", "Караганди"), ("mr", "काराग\u{902}डी"), ("ms", "Wilayah Karagandy"), ("nb", "Karagandy region"), ("nl", "Oblast Qarağandı"), ("no", "Karagandy region"), ("pl", "Obwód karagandzki"), ("pt", "Qaraghandy"), ("ro", "Provincia Karagandî"), ("ru", "Карагандинская область"), ("si", "කරගන\u{dca}ඩ\u{dd2} කල\u{dcf}පය"), ("sr", "Карагандинска област"), ("sr_Latn", "Karagandinska oblast"), ("sv", "Qaraghandy"), ("ta", "க\u{bbe}ரகண\u{bcd}டி பகுதி"), ("te", "క\u{c3e}రగ\u{c3e}ండ\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "การาก\u{e31}นด\u{e35}"), ("tr", "Karagandı Eyaleti"), ("uk", "Карагандинська область"), ("ur", "قاراغاندی صوبہ"), ("uz", "Qaragʻandi viloyati"), ("vi", "Karagandy"), ("zh", "卡拉干達州")]),
                        unofficial_name_list: ["Karaganda", "Karagandi", "Karagandy", "Qaraghandy", "Ķaragandı"].to_vec(),
                    }
                ),
                (
                    "KUS",
                    Subdivision{
                        name: "KUS",
                        country_alpha2: Alpha2::KZ,
                        code: "KUS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5077096), longitude: Some(64.04790729999999), max_latitude: Some(54.7125819), min_latitude: Some(48.197313), max_longitude: Some(68.02896299999999), min_longitude: Some(60.0529098)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كوستناي"), ("az", "Qostanay vilayəti"), ("be", "Кастанайская вобласць"), ("bg", "Костанайска област"), ("bn", "ক\u{9c1}স\u{9cd}ট\u{9be}নি অঞ\u{9cd}চল"), ("ca", "província de Kostanay"), ("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄑𑄚𑄬"), ("ceb", "Qostanay Oblysy"), ("cs", "Kostanajská oblast"), ("da", "Kostanay Region"), ("de", "Qostanai"), ("el", "Κοστανέι"), ("en", "Kostanay"), ("es", "Provincia de Kostanay"), ("et", "Kostanaj oblast"), ("eu", "Kostanay probintzia"), ("fa", "استان قوستانای"), ("fi", "Qostanayn alue"), ("fr", "Oblys de Kostanaï"), ("gu", "કોસ\u{acd}તાનાય પ\u{acd}રદ\u{ac7}શ"), ("he", "קוסטנאי (מחוז)"), ("hi", "कोस\u{94d}तानय प\u{94d}रा\u{902}त"), ("hu", "Kosztanaji terület"), ("hy", "Կոստանայ շրջան"), ("id", "Provinsi Qostanay"), ("is", "Kóstanæfylki"), ("it", "Regione di Qostanay"), ("ja", "コスタナイ州"), ("ka", "კოსტანაის ოლქი"), ("kk", "Қостанай облысы"), ("kn", "ಕೊಸ\u{ccd}ಟನೇ ಪ\u{ccd}ರದೇಶ"), ("ko", "코스타나이 주"), ("ky", "Костанай облусу"), ("lt", "Kostanajaus sritis"), ("lv", "Kostanajas apgabals"), ("mk", "Костанај"), ("mr", "कोस\u{94d}तानय"), ("ms", "Wilayah Kostanay"), ("nb", "Kostanay region"), ("nl", "Oblast Qostanay"), ("no", "Kostanay region"), ("pa", "ਕ\u{a4b}ਸਤਾਨਾਏ ਸ\u{a42}ਬਾ"), ("pl", "Obwód kustanajski"), ("pt", "Qostanay"), ("ro", "Provincia Kostanai"), ("ru", "Костанайская область"), ("si", "කොස\u{dca}ටනේ කල\u{dcf}පය"), ("sr", "Костанајска област"), ("sr_Latn", "Kostanajska oblast"), ("sv", "Qostanaj"), ("ta", "கோஸ\u{bcd}ட\u{bbe}ன\u{bbe}ய\u{bcd} பகுதி"), ("te", "క\u{c4b}స\u{c4d}ట\u{c3e}న\u{c47} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คอสตาไน"), ("tr", "Kostanay Eyaleti"), ("uk", "Костанайська область"), ("ur", "قوستانای صوبہ"), ("uz", "Qoʻstanoy viloyati"), ("vi", "Kostanay"), ("zh", "庫斯塔奈州")]),
                        unofficial_name_list: ["Kostanay", "Kustanai", "Kustanaj", "Kustanay", "Ķostanay"].to_vec(),
                    }
                ),
                (
                    "KZY",
                    Subdivision{
                        name: "KZY",
                        country_alpha2: Alpha2::KZ,
                        code: "KZY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6922613), longitude: Some(62.6571885), max_latitude: Some(47.854687), min_latitude: Some(42.421028), max_longitude: Some(68.0135679), min_longitude: Some(58.7882902)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كيزيلوردا"), ("az", "Qızılorda vilayəti"), ("be", "Кызылардзінская вобласць"), ("bg", "Къзълординска област"), ("bn", "কিজিলোর\u{9cd}দ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Khizilordà"), ("ccp", "𑄇\u{1112d}𑄎\u{1112d}𑄣\u{11127}𑄢\u{11134}𑄓"), ("ceb", "Qyzylorda Oblysy"), ("cs", "Kyzylordská oblast"), ("da", "Kyzylorda Province"), ("de", "Qysylorda"), ("el", "Περιφέρεια Κιζιλόρντα"), ("en", "Kyzylorda"), ("es", "Provincia de Kyzylorda"), ("et", "Kõzõlorda oblast"), ("eu", "Kyzylorda probintzia"), ("fa", "استان قیزیل\u{200c}اوردا"), ("fi", "Kyzyl-Ordan alue"), ("fr", "Oblys de Kyzylorda"), ("gu", "કીઝીલોર\u{acd}ડા પ\u{acd}રા\u{a82}ત"), ("he", "קזילאורדה (מחוז)"), ("hi", "किज\u{93c}िलओरदा प\u{94d}रा\u{902}त"), ("hy", "Կզըլ-Օրդայի մարզ"), ("id", "Provinsi Qyzylorda"), ("is", "Kusulórdafylki"), ("it", "Regione di Qyzylorda"), ("ja", "クズロルダ州"), ("ka", "ყიზილორდის ოლქი"), ("kk", "Қызылорда облысы"), ("kn", "ಕ\u{cbf}ಜ\u{cbf}ಲೋರ\u{ccd}ಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키질로르다 주"), ("ky", "Кызыл-Ордо облусу"), ("lt", "Kyzylordos sritis"), ("lv", "Kizilordas apgabals"), ("mk", "Кизилорда"), ("mr", "किझिलोर\u{94d}दा"), ("ms", "Wilayah Kyzylorda"), ("nb", "Qızılorda"), ("nl", "Oblast Qızılorda"), ("no", "Qızılorda"), ("pl", "Obwód kyzyłordyński"), ("pt", "Qyzylorda"), ("ro", "Provincia Kîzîlorda"), ("ru", "Кызылординская область"), ("si", "ක\u{dd2}ස\u{dd2}ලොර\u{dca}ඩ\u{dcf} පළ\u{dcf}ත"), ("sr", "Кизилординска област"), ("sr_Latn", "Kizilordinska oblast"), ("sv", "Qyzylorda"), ("ta", "க\u{bbe}ய\u{bcd}ஸ\u{bcd}யல\u{bbe}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c48}జ\u{c3f}ల\u{c4b}ర\u{c4d}డ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e35}ซ\u{e34}ลอร\u{e4c}ดา"), ("tr", "Kızılorda Eyaleti"), ("uk", "Кизилординська область"), ("ur", "قیزیلوردا صوبہ"), ("uz", "Qiziloʼrda viloyati"), ("vi", "Kyzylorda"), ("zh", "克孜勒奧爾達州")]),
                        unofficial_name_list: ["Ak-Mechet", "Kizilorda", "Kyzyl-Orda", "Kyzylorda", "Kzyl-Orda", "Qyzylorda", "Ķızılorda"].to_vec(),
                    }
                ),
                (
                    "MAN",
                    Subdivision{
                        name: "MAN",
                        country_alpha2: Alpha2::KZ,
                        code: "MAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.590802), longitude: Some(53.84995079999999), max_latitude: Some(46.461712), min_latitude: Some(41.240135), max_longitude: Some(56.696327), min_longitude: Some(50.0048314)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس مانكيستاو"), ("az", "Manğıstau vilayəti"), ("be", "Мангістаўская вобласць"), ("bg", "Мангистауска област"), ("bn", "মঙ\u{9cd}গিস\u{9cd}টৌ অঞ\u{9cd}চল"), ("ca", "Mangghystau"), ("ccp", "𑄟\u{11101}𑄉\u{11128}𑄌\u{11134}𑄑𑄅\u{1112a}"), ("ceb", "Mangistauskaya Oblast’"), ("cs", "Mangystauská oblast"), ("da", "Mangystau Region"), ("de", "Mangghystau"), ("el", "Περιφέρεια Μανγκιστάου"), ("en", "Mangystau"), ("es", "Mangystau"), ("et", "Manggõstau oblast"), ("eu", "Mangystau"), ("fa", "استان مانغیستاو"), ("fi", "Mañğıstaw’n alue"), ("fr", "Oblys de Manguistaou"), ("gu", "મા\u{a82}ગિસ\u{acd}તૌ પ\u{acd}રદ\u{ac7}શ"), ("he", "מנגוסטאו (מחוז)"), ("id", "Provinsi Mangghystau"), ("is", "Mangystáfylki"), ("it", "Regione di Mangghystau"), ("ja", "マンギスタウ州"), ("ka", "მანგისთაუს ოლქი"), ("kk", "Маңғыстау облысы"), ("kn", "ಮಂಗೈಸ\u{ccd}ಟ\u{ccc} ಪ\u{ccd}ರದೇಶ"), ("ko", "망기스타우 주"), ("ky", "Маңгыстоо областы"), ("lt", "Mangyštau sritis"), ("lv", "Manghistau apgabals"), ("mk", "Мангистау"), ("mr", "मा\u{902}गिस\u{94d}तौ"), ("ms", "Wilayah Mangystau"), ("nb", "Mangystau oblast"), ("nl", "Oblast Mañğıstaw"), ("no", "Mangystau oblast"), ("pa", "ਮਾ\u{a02}ਗਿਸਤ\u{a4c}"), ("pl", "Obwód mangystauski"), ("pt", "Mangghystau"), ("ro", "Provincia Mangîstau"), ("ru", "Мангистауская область"), ("si", "මැන\u{dca}ග\u{dd2}ස\u{dca}ටව\u{dd4} කල\u{dcf}පය"), ("sr", "Мангистауска област"), ("sr_Latn", "Mangistauska oblast"), ("sv", "Mangghystaw"), ("ta", "மஞ\u{bcd}சிஸ\u{bcd}ட\u{bbe}வு பகுதி"), ("te", "మ\u{c47}ంజ\u{c3f}స\u{c4d}ట\u{c3e}వ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ม\u{e31}งก\u{e37}ย\u{e4c}สตาอ\u{e39}"), ("tr", "Mangistav Eyaleti"), ("uk", "Мангистауська область"), ("ur", "مانغیستاؤ صوبہ"), ("uz", "Mangʻistov viloyati"), ("vi", "Mangistau"), ("zh", "曼格斯套州")]),
                        unofficial_name_list: ["Mangghystau", "Mangistau", "Mangkistau", "Mangqystau"].to_vec(),
                    }
                ),
                (
                    "PAV",
                    Subdivision{
                        name: "PAV",
                        country_alpha2: Alpha2::KZ,
                        code: "PAV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.65085440000001), longitude: Some(76.7773224), max_latitude: Some(54.4579159), min_latitude: Some(50.023726), max_longitude: Some(79.590622), min_longitude: Some(73.282956)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس بافلودار"), ("az", "Pavlodar vilayəti"), ("be", "Паўладарская вобласць"), ("bg", "Павлодарска област"), ("bn", "প\u{9be}ভলোদ\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Pavlodar"), ("ccp", "𑄛𑄛\u{11134}𑄣\u{1112e}𑄓𑄢\u{11134}"), ("ceb", "Pavlodar Oblysy"), ("cs", "Pavlodarská oblast"), ("da", "Pavlodar Province"), ("de", "Pawlodar"), ("el", "Περιφέρεια Παβλοντάρ"), ("en", "Pavlodar"), ("es", "Provincia de Pavlodar"), ("et", "Pavlodari oblast"), ("eu", "Pavlodar probintzia"), ("fa", "استان پاولودار"), ("fi", "Pavlodarin alue"), ("fr", "Oblys de Pavlodar"), ("gu", "પાવ\u{acd}લોડોર પ\u{acd}રા\u{a82}ત"), ("he", "פבלודר (מחוז)"), ("hi", "पाव\u{94d}लोदार प\u{94d}रा\u{902}त"), ("hy", "Պավլոդարի մարզ"), ("id", "Provinsi Pavlodar"), ("is", "Pavlódarfylki"), ("it", "Regione di Pavlodar"), ("ja", "パブロダール州"), ("ka", "პავლოდარის ოლქი"), ("kk", "Павлодар облысы"), ("kn", "ಪವ\u{ccd}ಲೋಡರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파블로다르 주"), ("ky", "Павлодар областы"), ("lt", "Pavlodaro sritis"), ("lv", "Pavlodaras apgabals"), ("mk", "Павлодар"), ("mr", "पाव\u{94d}लोदर"), ("ms", "Wilayah Pavlodar"), ("nb", "Pavlodar provins"), ("nl", "Oblast Pavlodar"), ("no", "Pavlodar provins"), ("pa", "ਪਾਵਲ\u{a4b}ਦਰ"), ("pl", "Obwód pawłodarski"), ("pt", "Pavlodar"), ("ro", "Provincia Pavlodar"), ("ru", "Павлодарская область"), ("si", "පව\u{dca}ලොද\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sr", "Павлодарска област"), ("sr_Latn", "Pavlodarska oblast"), ("sv", "Pavlodar"), ("ta", "ப\u{bbe}வ\u{bcd}லோடர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}వ\u{c4d}ల\u{c4b}డర\u{c4d}"), ("th", "ป\u{e31}ฟโลดาร\u{e4c}"), ("tr", "Pavlodar Eyaleti"), ("uk", "Павлодарська область"), ("ur", "پاؤلودار صوبہ"), ("uz", "Pavlodar viloyati"), ("vi", "Pavlodar"), ("zh", "巴甫洛達爾州")]),
                        unofficial_name_list: ["Pavlodar oblysy"].to_vec(),
                    }
                ),
                (
                    "SEV",
                    Subdivision{
                        name: "SEV",
                        country_alpha2: Alpha2::KZ,
                        code: "SEV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.2948229), longitude: Some(69.4047872), max_latitude: Some(53.3444028), min_latitude: Some(53.2533834), max_longitude: Some(69.4638061), min_longitude: Some(69.35711859999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كازاخستان الشمالي"), ("az", "Şimalı Qazaxıstan vilayəti"), ("be", "Паўночна-Казахстанская вобласць"), ("bg", "Североказахстанска област"), ("bn", "উত\u{9cd}তর ক\u{9be}জ\u{9be}খস\u{9cd}ত\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província del Kazakhstan Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄇𑄎𑄈𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Severo-Kazakhstanskaya Oblast’"), ("cs", "Severokazašská oblast"), ("da", "North Kazakhstan Province"), ("de", "Nordkasachstan"), ("el", "Περιφέρεια Βορείου Καζακστάν"), ("en", "North Kazakhstan"), ("es", "Provincia de Kazajistán Septentrional"), ("et", "Põhja-Kasahstani oblast"), ("eu", "Iparraldeko Kazakhstan"), ("fa", "استان قزاقستان شمالی"), ("fi", "Pohjois-Kazakstanin alue"), ("fr", "Kazakhstan septentrional"), ("gu", "ઉત\u{acd}તર કઝાકસ\u{acd}તાન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז הצפון (קזחסטן)"), ("hi", "उत\u{94d}तर क\u{93c}ाज\u{93c}ाक\u{93c}स\u{94d}तान प\u{94d}रा\u{902}त"), ("hy", "Հյուսիս-Ղազախստանյան մարզ"), ("id", "Provinsi Kazakhstan Utara"), ("is", "Norður-Kasakstanfylki"), ("it", "Regione del Kazakistan Settentrionale"), ("ja", "北カザフスタン州"), ("ka", "ჩრდილოეთ ყაზახეთის ოლქი"), ("kk", "Солтүстік Қазақстан облысы"), ("kn", "ಉತ\u{ccd}ತರ ಕಝಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "솔튀스틱카작스탄 주"), ("lt", "Šiaurės Kazachstano sritis"), ("lv", "Ziemeļkazahstānas apgabals"), ("mk", "Северен Казахстан"), ("mr", "उत\u{94d}तर कझाकस\u{94d}तान"), ("ms", "Wilayah Kazakhstan Utara"), ("nb", "Nord Kazakhstan provins"), ("nl", "Oblast Noord-Kazachstan"), ("no", "Nord Kazakhstan provins"), ("pa", "ਉ\u{a71}ਤਰੀ ਕਜ\u{a3c}ਾਖ\u{a3c}ਿਸਤਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Obwód północnokazachstański"), ("pt", "Cazaquistão do Norte"), ("ro", "Provincia Kazahstanul de Nord"), ("ru", "Северо-Казахстанская область"), ("si", "උත\u{dd4}ර\u{dd4} කසකස\u{dca}ත\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Северноказахстанска област"), ("sr_Latn", "Severnokazahstanska oblast"), ("sv", "Nordkazakstan"), ("ta", "வடக\u{bcd}கு கஜகஸ\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర కజక\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "นอร\u{e4c}ท คาซ\u{e31}คสถาน"), ("tr", "Kuzey Kazakistan Eyaleti"), ("uk", "Північно-Казахстанська область"), ("ur", "شمالی قازقستان صوبہ"), ("uz", "Shimoliy Qozogʻiston viloyati"), ("vi", "Bắc Kazakhstan"), ("zh", "北哈薩克斯坦州")]),
                        unofficial_name_list: ["Northern Kazakhstan", "Soltüstik Kazakstan", "Soltüstik Qazaqstan"].to_vec(),
                    }
                ),
                (
                    "SHY",
                    Subdivision{
                        name: "SHY",
                        country_alpha2: Alpha2::KZ,
                        code: "SHY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.019573), longitude: Some(66.923684), max_latitude: Some(55.4419839), min_latitude: Some(40.5685841), max_longitude: Some(87.315415), min_longitude: Some(46.493672)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Shymkent")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "VOS",
                    Subdivision{
                        name: "VOS",
                        country_alpha2: Alpha2::KZ,
                        code: "VOS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.4884873), longitude: Some(82.7103185), max_latitude: Some(51.4008139), min_latitude: Some(45.57064099999999), max_longitude: Some(87.3126599), min_longitude: Some(76.76924199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كازاخستان الشرقي"), ("az", "Şərqi Qazaxıstan əyaləti"), ("be", "Усходне-Казахстанская вобласць"), ("bg", "Източноказахстанска област"), ("bn", "ইস\u{9cd}ট ক\u{9be}জ\u{9be}ক\u{9be}স\u{9cd}ত\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Província del Kazakhstan Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄇𑄎𑄈𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Shyghys Qazaqstan Oblysy"), ("cs", "Východokazašská oblast"), ("da", "East Kazakhstan Region"), ("de", "Ostkasachstan"), ("el", "Περιφέρεια Ανατολικού Καζακστάν"), ("en", "East Kazakhstan"), ("es", "Provincia de Kazajistán Oriental"), ("et", "Ida-Kasahstani oblast"), ("eu", "Ekialdeko Kazakhstan"), ("fa", "استان قزاقستان شرقی"), ("fi", "Itä-Kazakstanin alue"), ("fr", "Kazakhstan oriental"), ("gu", "પ\u{ac2}ર\u{acd}વ કઝાખસ\u{acd}તાન પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז המזרח (קזחסטן)"), ("hi", "प\u{942}र\u{94d}व क\u{93c}ाज\u{93c}ाक\u{93c}स\u{94d}तान प\u{94d}रा\u{902}त"), ("hy", "Արևելաղազախական շրջան"), ("id", "Provinsi Kazakhstan Timur"), ("is", "Austur-Kasakstanfylki"), ("it", "Regione del Kazakistan Orientale"), ("ja", "東カザフスタン州"), ("ka", "აღმოსავლეთ ყაზახეთის ოლქი"), ("kk", "Шығыс Қазақстан облысы"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಕಝಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "시기스카작스탄 주"), ("lt", "Rytų Kazachstano sritis"), ("lv", "Austrumkazahstānas apgabals"), ("mk", "Источен Казахстан"), ("mr", "प\u{942}र\u{94d}व कझाकस\u{94d}तान"), ("ms", "Wilayah Kazakhstan Timur"), ("nb", "Vest Kazakhstan region"), ("nl", "Oblast Oost-Kazachstan"), ("no", "Vest Kazakhstan region"), ("pl", "Obwód wschodniokazachstański"), ("pt", "Cazaquistão Oriental"), ("ro", "Provincia Kazahstanul de Est"), ("ru", "Восточно-Казахстанская область"), ("si", "නැගෙනහ\u{dd2}ර කසකස\u{dca}ත\u{dcf}න\u{dca} කල\u{dcf}පය"), ("sr", "Источноказахстанска област"), ("sr_Latn", "Istočnokazahstanska oblast"), ("sv", "Östkazakstan"), ("ta", "கிழக\u{bcd}கு கஜகஸ\u{bcd}த\u{bbe}ன\u{bcd} பகுதி"), ("te", "తూర\u{c4d}పు కజక\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาซ\u{e31}คสถานตะว\u{e31}นออก"), ("tr", "Doğu Kazakistan Eyaleti"), ("uk", "Східно-Казахстанська область"), ("ur", "مشرقی قازقستان صوبہ"), ("uz", "Sharqiy Qozogʻiston viloyati"), ("vi", "Đông Kazakhstan"), ("zh", "東哈薩克斯坦州")]),
                        unofficial_name_list: ["Eastern Kazakhstan", "Shyghys Qazaqstan", "Sigis Kazakstan"].to_vec(),
                    }
                ),
                (
                    "YUZ",
                    Subdivision{
                        name: "YUZ",
                        country_alpha2: Alpha2::KZ,
                        code: "YUZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.56797270000001), longitude: Some(50.8066617), max_latitude: Some(51.76659799999999), min_latitude: Some(48.002499), max_longitude: Some(54.5574649), min_longitude: Some(46.4918561)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس كازاخستان الجنوبي"), ("az", "Cənubi Qazaxıstan əyaləti"), ("be", "Паўднёва-Казахстанская вобласць"), ("bg", "Южноказахстанска област"), ("bn", "স\u{9be}উথ ক\u{9be}জ\u{9be}ক\u{9be}স\u{9cd}ত\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Província del Kazakhstan Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇𑄎𑄈𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Yuzhno-Kazakhstanskaya Oblast’"), ("cs", "Jihokazašská oblast"), ("da", "South Kazakhstan Region"), ("de", "Südkasachstan"), ("el", "Περιφέρεια Νότιου Καζακστάν"), ("en", "South Kazakhstan"), ("es", "Provincia de Kazajistán Meridional"), ("et", "Lõuna-Kasahstani oblast"), ("eu", "Hegoaldeko Kazakhstan"), ("fa", "استان قزاقستان جنوبی"), ("fi", "Etelä-Kazakstanin alue"), ("fr", "Kazakhstan méridional"), ("gu", "દક\u{acd}ષિણ કઝાકિસ\u{acd}તાન પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז הדרום (קזחסטן)"), ("hi", "दक\u{94d}षिण क\u{93c}ाज\u{93c}ाक\u{93c}स\u{94d}तान प\u{94d}रा\u{902}त"), ("hy", "Հարավ-Ղազախստանյան մարզ"), ("id", "Provinsi Kazakhstan Selatan"), ("is", "Suður-Kasakstanfylki"), ("it", "Regione del Kazakistan Meridionale"), ("ja", "南カザフスタン州"), ("ka", "სამხრეთ ყაზახეთის ოლქი"), ("kk", "Түркістан облысы"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಕಝಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "옹튀스틱카작스탄 주"), ("lt", "Pietų Kazachstano sritis"), ("lv", "Dienvidkazahstānas apgabals"), ("mk", "Јужен Казахстан"), ("mr", "दक\u{94d}षिण कझाकस\u{94d}तान"), ("ms", "Wilayah Kazakhstan Selatan"), ("nb", "Sør Kazakhstan region"), ("nl", "Oblast Zuid-Kazachstan"), ("no", "Sør Kazakhstan region"), ("pa", "ਦ\u{a71}ਖਣੀ ਕਜ\u{a3c}ਾਖ\u{a3c}ਸਤਾਨ"), ("pl", "Obwód południowokazachstański"), ("pt", "Cazaquistão do Sul"), ("ro", "Provincia Kazahstanul de Sud"), ("ru", "Южно-Казахстанская область"), ("si", "දක\u{dd4}ණ\u{dd4} කසකස\u{dca}ත\u{dcf}නය"), ("sl", "Južno-kazahstanska oblast"), ("sr", "Јужно-казахстанска област"), ("sr_Latn", "Južno-kazahstanska oblast"), ("sv", "Sydkazakstan"), ("ta", "தெற\u{bcd}கு கஜகஸ\u{bcd}த\u{bbe}ன\u{bcd} பகுதி"), ("te", "దక\u{c4d}ష\u{c3f}ణ కజక\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาซ\u{e31}คสถานใต\u{e49}"), ("tr", "Güney Kazakistan Eyaleti"), ("uk", "Південно-Казахстанська область"), ("ur", "جنوبی قازقستان صوبہ"), ("uz", "Turkiston viloyati"), ("vi", "Nam Kazakhstan"), ("zh", "南哈薩克斯坦州")]),
                        unofficial_name_list: ["Ongtüstik Qazaqstan", "Ongtüstük Kazakstan", "Southern Kazakhstan"].to_vec(),
                    }
                ),
                (
                    "ZAP",
                    Subdivision{
                        name: "ZAP",
                        country_alpha2: Alpha2::KZ,
                        code: "ZAP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.56797270000001), longitude: Some(50.8066617), max_latitude: Some(51.76659799999999), min_latitude: Some(48.002499), max_longitude: Some(54.5574649), min_longitude: Some(46.4918561)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كازاخستان الغربية"), ("az", "Qərbi Qazaxıstan əyaləti"), ("be", "Заходне-Казахстанская вобласць"), ("bg", "Западноказахстанска област"), ("bn", "পশ\u{9cd}চি\u{9cd}চিম ক\u{9be}জ\u{9be}কস\u{9cd}ত\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Kazakhstan Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄇𑄎𑄈𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Batys Qazaqstan Oblysy"), ("cs", "Západokazašská oblast"), ("da", "Vest-Kasakhstan oblast"), ("de", "Westkasachstan"), ("el", "Περιφέρεια Δυτικού Καζακστάν"), ("en", "West Kazakhstan"), ("es", "Provincia de Kazajistán Occidental"), ("et", "Lääne-Kasahstani oblast"), ("eu", "Mendebaldeko Kazakhstan"), ("fa", "استان قزاقستان غربی"), ("fi", "Länsi-Kazakstanin alue"), ("fr", "Kazakhstan occidental"), ("gu", "પશ\u{acd}ચિમ કઝાખસ\u{acd}તાન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז המערב (קזחסטן)"), ("hi", "पश\u{94d}चिम क\u{93c}ाज\u{93c}ाक\u{93c}स\u{94d}तान प\u{94d}रा\u{902}त"), ("hy", "Արևմտյան Ղազախստանի մարզ"), ("id", "Provinsi Kazakhstan Barat"), ("is", "Vestur-Kasakstanfylki"), ("it", "Regione del Kazakistan Occidentale"), ("ja", "西カザフスタン州"), ("ka", "დასავლეთ ყაზახეთის ოლქი"), ("kk", "Батыс Қазақстан облысы"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಕಝಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바티스카작스탄 주"), ("ky", "Батыш Казакстан облусу"), ("lt", "Vakarų Kazachstano sritis"), ("lv", "Rietumkazahstānas apgabals"), ("mk", "Западноказахстанска област"), ("mr", "पश\u{94d}चिम कझाकस\u{94d}तान"), ("ms", "Wilayah Kazakhstan Barat"), ("nb", "Vest-Kasakhstan oblast"), ("nl", "Oblast Batıs Qazaqstan"), ("no", "Vest-Kasakhstan oblast"), ("pl", "Obwód zachodniokazachstański"), ("pt", "Cazaquistão Ocidental"), ("ro", "Provincia Kazahstanul de Vest"), ("ru", "Западно-Казахстанская область"), ("si", "බටහ\u{dd2}ර කසකස\u{dca}ත\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Западноказахстанска област"), ("sr_Latn", "Zapadnokazahstanska oblast"), ("sv", "Västkazakstan"), ("ta", "மேற\u{bcd}கு கஜகஸ\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ కజక\u{c3f}స\u{c4d}త\u{c3e}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "คาซ\u{e31}คสถานตะว\u{e31}นตก"), ("tr", "Batı Kazakistan Eyaleti"), ("uk", "Західно-Казахстанська область"), ("ur", "مغربی قازقستان صوبہ"), ("uz", "Gʻarbiy Qozogʻiston viloyati"), ("vi", "Tây Kazakhstan"), ("zh", "西哈薩克斯坦州")]),
                        unofficial_name_list: ["Batis Kazakstan", "Batys Qazaqstan", "Uralsk", "Uralskaya Oblast", "Western Kazakhstan"].to_vec(),
                    }
                ),
                (
                    "ZHA",
                    Subdivision{
                        name: "ZHA",
                        country_alpha2: Alpha2::KZ,
                        code: "ZHA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.2220308), longitude: Some(72.36579669999999), max_latitude: Some(46.0366224), min_latitude: Some(42.2212301), max_longitude: Some(75.77342999999999), min_longitude: Some(68.9939081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبليس جامبيل"), ("az", "Cambul vilayəti"), ("be", "Жамбылская вобласць"), ("bg", "Жамбълска област"), ("ca", "Província de Jàmbil"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{11128}𑄣\u{11134}"), ("ceb", "Zhambyl Oblysy"), ("cs", "Žambylská oblast"), ("de", "Schambyl"), ("el", "Περιφέρεια Τζαμπίλ"), ("en", "Jambyl"), ("es", "Provincia de Zhambyl"), ("et", "Žambõli oblast"), ("eu", "Zhambyl"), ("fa", "استان ژمبیل"), ("fi", "Jambılin alue"), ("fr", "Oblys de Djamboul"), ("he", "דז׳אמבול (מחוז)"), ("hi", "झ\u{93c}ामबिल प\u{94d}रा\u{902}त"), ("id", "Provinsi Zhambyl"), ("is", "Djambýlfylki"), ("it", "Regione di Žambyl"), ("ja", "ジャンブール州"), ("ka", "ჟამბილის ოლქი"), ("kk", "Жамбыл облысы"), ("ko", "잠빌 주"), ("ky", "Жамбыл облусу"), ("lt", "Žambylo sritis"), ("lv", "Žambilas apgabals"), ("mk", "Жамбил"), ("mr", "झा\u{902}बील"), ("ms", "Wilayah Zhambyl"), ("nl", "Oblast Jambıl"), ("pl", "Obwód żambylski"), ("pt", "Jambyl"), ("ro", "Provincia Jambîl"), ("ru", "Жамбылская область"), ("sr", "Жамбилска област"), ("sr_Latn", "Žambilska oblast"), ("sv", "Zjambyl"), ("tr", "Jambıl Eyaleti"), ("uk", "Жамбилська область"), ("ur", "جمبیل پراونس"), ("uz", "Jambul viloyati"), ("vi", "Zhambyl"), ("zh", "江布爾州")]),
                        unofficial_name_list: ["Aulie-Ata", "Auliye-Ata", "Cambil", "Cambıl", "Djambul", "Dzhambul", "Zhambul", "Zhambyl", "Zhambül"].to_vec(),
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
#[cfg(feature = "kz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::KZ,
        alpha3: Alpha3::KAZ,
        address_format: None,
        continent: Continent::Asia,
        country_code: 7,
        currency_code: CurrencyCode::KZT,
        gec: Some(GEC::KZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::KAZ),
        iso_long_name: "The Republic of Kazakhstan",
        iso_short_name: "Kazakhstan",
        official_language_list: ["kk", "ru"].to_vec(),
        spoken_language_list: ["kk", "ru"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "8",
        nationality: Some("Kazakhstani"),
        number: "398",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAsia),
        un_locode: "KZ",
        unofficial_name_list: [
            "Kazakhstan",
            "Kasachstan",
            "Kazajistán",
            "カザフスタン",
            "Kazachstan",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Kazakhstan"),
            ("af", "Kazakstan"),
            ("ak", "Kazakhstan"),
            ("am", "ካዛክስታን"),
            ("an", "Kazakhstan"),
            ("ar", "كازاخستان"),
            ("as", "ক\u{9be}জ\u{9be}খিস\u{9cd}ত\u{9be}ন"),
            ("ay", "Kazakhstan"),
            ("az", "Qazaxstan"),
            ("ba", "Kazakhstan"),
            ("be", "Казахстан"),
            ("bg", "Казахстан"),
            ("bi", "Kazakhstan"),
            ("bn", "ক\u{9be}জ\u{9be}খস\u{9cd}ত\u{9be}ন"),
            ("bn_IN", "ক\u{9be}জ\u{9be}খস\u{9cd}ত\u{9be}ন"),
            ("br", "Kazakstan"),
            ("bs", "Kazahstan"),
            ("ca", "Kazakhstan"),
            ("ce", "Кхазакхстан"),
            ("ch", "Kazakhstan"),
            ("cs", "Kazachstán"),
            ("cv", "Кхазакхстан"),
            ("cy", "Kazakhstan"),
            ("da", "Kasakhstan"),
            ("de", "Kasachstan"),
            ("dv", "ކ\u{7a6}ޒ\u{7a6}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
            ("dz", "ཀ་ཛཀས\u{f72}་ཏ\u{f71}ན།"),
            ("ee", "Kazakhstan"),
            ("el", "Καζακστάν"),
            ("en", "Kazakhstan"),
            ("eo", "Kazaĥio"),
            ("es", "Kazajistán"),
            ("et", "Kasahstan"),
            ("eu", "Kazakhstan"),
            ("fa", "قزاقستان"),
            ("ff", "Kazakhstan"),
            ("fi", "Kazakstan"),
            ("fo", "Kasakstan"),
            ("fr", "Kazakhstan"),
            ("fy", "Kazakstan"),
            ("ga", "An Chasacstáin"),
            ("gl", "Kazakhstán"),
            ("gn", "Kazakhstan"),
            ("gu", "કઝાખસ\u{acd}તાન"),
            ("gv", "Yn Chassaghstaan"),
            ("ha", "Kazakystan"),
            ("he", "קזחסטן"),
            ("hi", "कज\u{93c}ाख\u{93c}िस\u{94d}तान"),
            ("hr", "Kazahstan"),
            ("ht", "Kazakstan"),
            ("hu", "Kazahsztán"),
            ("hy", "Ղազախստան"),
            ("ia", "Kazakhstan"),
            ("id", "Kazakhstan"),
            ("io", "Kazakstan"),
            ("is", "Kasakstan"),
            ("it", "Kazakistan"),
            ("iu", "ᑲᓴᒃᖢᓇ ᓄᓇ"),
            ("ja", "カザフスタン"),
            ("ka", "ყაზახეთი"),
            ("ki", "Kazakhstan"),
            ("kk", "Қазақстан"),
            ("kl", "Kazakhstan"),
            ("km", "កាហ\u{17d2}សាក\u{17cb}ស\u{17d2}តង\u{17cb}"),
            ("kn", "ಕಝಕಸ\u{ccd}ತಾನ\u{ccd}"),
            ("ko", "카자흐스탄"),
            ("ku", "Kazaxstan"),
            ("kv", "Казахстан"),
            ("kw", "Pow Kazagh"),
            ("ky", "Казакстан"),
            ("lo", "Kazakhstan"),
            ("lt", "Kazachstanas"),
            ("lv", "Kazahstāna"),
            ("mi", "Katatānga"),
            ("mk", "Казакстан"),
            ("ml", "കസ\u{d3e}ഖിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Казакстан"),
            ("mr", "कझाकिस\u{94d}तान"),
            ("ms", "Kazakhstan"),
            ("mt", "Każakistan"),
            (
                "my",
                "ကာဇက\u{103a}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Kadaketan"),
            ("nb", "Kasakhstan"),
            ("ne", "काजखास\u{94d}तान"),
            ("nl", "Kazachstan"),
            ("nn", "Kasakhstan"),
            ("nv", "Bilį\u{301}į\u{301}ʼ Ńdeiltihii Dineʼé Bikéyah"),
            ("oc", "Cazacstan"),
            ("or", "କ\u{b3e}ଜ\u{b3e}ଖସ\u{b4d}ତ\u{b3e}ନ"),
            ("pa", "ਕਜ\u{a3c}ਾਕਸਤਾਨ"),
            ("pi", "कजाकस\u{94d}थान"),
            ("pl", "Kazachstan"),
            ("ps", "قزاقستان"),
            ("pt", "Cazaquistão"),
            ("pt_BR", "Cazaquistão"),
            ("ro", "Kazakhstan"),
            ("ru", "Казахстан"),
            ("rw", "Kazakisitani"),
            ("sc", "Kazakistan"),
            ("sd", "قازڪستان"),
            ("si", "කසකස\u{dca}ත\u{dcf}නය"),
            ("sk", "Kazachstan"),
            ("sl", "Kazahstan"),
            ("so", "Kasaakhistaan"),
            ("sq", "Kazakistan"),
            ("sr", "Казахстан"),
            ("sv", "Kazakstan"),
            ("sw", "Kazakhstan"),
            ("ta", "கசகிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
            ("te", "కఝక\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
            ("tg", "Қазоқистон"),
            ("th", "คาซ\u{e31}คสถาน"),
            ("ti", "Kazakhstan"),
            ("tk", "Gazagystan"),
            ("tl", "Kazakhstan"),
            ("tr", "Kazakistan"),
            ("tt", "Казакстан"),
            ("ug", "قازاقىستان"),
            ("uk", "Казахстан"),
            ("ur", "قازقستان"),
            ("uz", "Qozogʻiston"),
            ("ve", "Kazakhstan"),
            ("vi", "Kha-xa-kh-x-thanh"),
            ("wa", "Kazaxhtan"),
            ("wo", "Kaasakestaan"),
            ("xh", "Kazakhstan"),
            ("yo", "Kàsàkstán"),
            ("zh_CN", "哈萨克斯坦"),
            ("zh_HK", "哈薩克"),
            ("zh_TW", "哈薩克"),
            ("zu", "Kazakhstan"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
