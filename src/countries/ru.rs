// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Russian Federation

#[cfg(all(feature = "ru", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{postalcode}} {{city}}\n{{street}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::RU;
    pub const ALPHA3: Alpha3 = Alpha3::RUS;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 7;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::RUB;
    pub const GEC: Option<GEC> = Some(GEC::RS);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::RUS);
    pub const ISO_SHORT_NAME: &str = "Russian Federation";
    pub const ISO_LONG_NAME: &str = "The Russian Federation";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ru"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ru"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Russian");
    pub const NUMBER: &str = "643";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "RU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Russia",
        "Russland",
        "Russie",
        "Rusia",
        "ロシア連邦",
        "Rusland",
        "Россия",
        "Расія",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Russian Federation"),
        ("af", "Russiese Federasie"),
        ("ak", "Russian Federation"),
        ("am", "ሲቁ።"),
        ("an", "Russian Federation"),
        ("ar", "الات\u{651}حاد الر\u{651}وسي"),
        ("as", "ৰ\u{9c1}চ য\u{9c1}ক\u{9cd}তৰ\u{9be}ষ\u{9cd}ট\u{9cd}ৰ"),
        ("ay", "Russian Federation"),
        ("az", "Rusiya Federasiyası"),
        ("ba", "Russian Federation"),
        ("be", "Расійская Федэрацыя"),
        ("bg", "Руска федерация"),
        ("bi", "Russian Federation"),
        ("bn", "র\u{9c1}শ য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র"),
        (
            "bn_IN",
            "র\u{9c1}শ য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
        ),
        ("br", "Russian Federation"),
        ("bs", "Ruska Federacija"),
        ("ca", "Federació Russa"),
        ("ce", "Russian Federation"),
        ("ch", "Russian Federation"),
        ("cs", "Ruská federace"),
        ("cv", "Russian Federation"),
        ("cy", "Ffederasiwn Rwsia"),
        ("da", "Russiske føderation"),
        ("de", "Russische Föderation"),
        ("dv", "Russian Federation"),
        (
            "dz",
            "ར་ཤ\u{f72}་ཡ\u{f71}ན་ ཕ\u{f7a}་ཌ\u{f72}་ར\u{f7a}་ཤ\u{f71}ན།",
        ),
        ("ee", "Russian Federation"),
        ("el", "Ρωσική Ομοσπονδία"),
        ("en", "Russian Federation"),
        ("eo", "Rusa Federacio"),
        ("es", "Federación Rusa"),
        ("et", "Venemaa Föderatsioon"),
        ("eu", "Errusiar Federakundea"),
        ("fa", "روسیه فدرال"),
        ("ff", "Russian Federation"),
        ("fi", "Venäjän federaatio"),
        ("fo", "Russian Federation"),
        ("fr", "Russie, Fédération de"),
        ("fy", "Russian Federation"),
        ("ga", "Cónaidhm na Rúise"),
        ("gl", "Federación Rusa"),
        ("gn", "Russian Federation"),
        ("gu", "રશિયન ફ\u{ac7}ડર\u{ac7}શન"),
        ("gv", "Russian Federation"),
        ("ha", "Russian Federation"),
        ("he", "הפדרציה הרוסית"),
        ("hi", "रशियन फ\u{947}ड\u{947}रशन"),
        ("hr", "Ruska Federacija"),
        ("ht", "Russian Federation"),
        ("hu", "Orosz Föderáció"),
        ("hy", "Ռուսաստանի Դաշնություն"),
        ("ia", "Federation Russe"),
        ("id", "Federasi Rusia"),
        ("io", "Russian Federation"),
        ("is", "Rússneska sambandið"),
        ("it", "Russia"),
        ("iu", "Russian Federation"),
        ("ja", "ロシア連邦"),
        ("ka", "რუსეთის ფედრაცია"),
        ("ki", "Russian Federation"),
        ("kk", "Ресей"),
        ("kl", "Russian Federation"),
        (
            "km",
            "សហព\u{17d0}ន\u{17d2}ធ\u{200b}រ\u{17bb}ស\u{17d2}ស\u{17ca}\u{17b8}",
        ),
        ("kn", "ರಶ\u{cbf}ಯನ\u{ccd} ಒಕ\u{ccd}ಕ\u{cc2}ಟ"),
        ("ko", "러시아 연방"),
        ("ku", "Federasyona Rûsî"),
        ("kv", "Russian Federation"),
        ("kw", "Russian Federation"),
        ("ky", "Орус Федерациясы"),
        ("lo", "Russian Federation"),
        ("lt", "Rusijos Federacija"),
        ("lv", "Krievijas Federācija"),
        ("mi", "Russian Federation"),
        ("mk", "Руска федерација"),
        ("ml", "റഷ\u{d4d}യന\u{d4d}\u{200d} ഫെഡറേഷന\u{d4d}\u{200d}"),
        ("mn", "Оросын холбооны улс"),
        ("mr", "रशियन स\u{902}घराज\u{94d}य"),
        ("ms", "Russian Federation"),
        ("mt", "Russian Federation"),
        ("my", "Russian Federation"),
        ("na", "Russian Federation"),
        ("nb", "Den russiske føderasjon"),
        ("ne", "रसियन स\u{902}घ"),
        ("nl", "Rusland"),
        ("nn", "Russland"),
        ("nv", "Russian Federation"),
        ("oc", "Federacion russa"),
        ("or", "ଋଷୀ ଫେଡରେଶନ"),
        ("pa", "ਰ\u{a42}ਸੀ ਗਣਰਾਜ"),
        ("pi", "Russian Federation"),
        ("pl", "Federacja Rosyjska"),
        ("ps", "Russian Federation"),
        ("pt", "Federação Russa"),
        ("pt_BR", "Federação Russa"),
        ("ro", "Federația Rusă"),
        ("ru", "Российская Федерация"),
        ("rw", "Ukwishyirahamwe kw'Uburusiya"),
        ("sc", "Federatzione Russa"),
        ("sd", "Russian Federation"),
        ("si", "ර\u{dd4}ස\u{dd2}ය\u{dcf}න\u{dd4} සංගමය"),
        ("sk", "Ruská federácia"),
        ("sl", "Ruska federacija"),
        ("so", "Ruush"),
        ("sq", "Federata Ruse"),
        ("sr", "Руска Федерација"),
        ("sv", "Ryssland"),
        ("sw", "Russian Federation"),
        ("ta", "ரஷ\u{bcd}யன\u{bcd} கூட\u{bcd}டமைப\u{bcd}பு"),
        ("te", "రష\u{c4d}యన\u{c4d} ఫ\u{c46}డర\u{c47}షన\u{c4d}"),
        ("tg", "Федератсияи Россия"),
        ("th", "สหพ\u{e31}นธร\u{e31}ฐร\u{e31}สเซ\u{e35}ย"),
        ("ti", "Russian Federation"),
        ("tk", "Russiýa Federasiýasy"),
        ("tl", "Pederasyong Ruso"),
        ("tr", "Rusya Federasyonu"),
        ("tt", "Урыс Патшаһлык"),
        ("ug", "رۇسىيە فېدېراتسىيەسى"),
        ("uk", "Російська Федерація"),
        ("ur", "Russian Federation"),
        ("uz", "Russian Federation"),
        ("ve", "Russian Federation"),
        ("vi", "Liên Bang Nga"),
        ("wa", "Rûsseye"),
        ("wo", "Federaasioŋ bu Riisi"),
        ("xh", "Russian Federation"),
        ("yo", "Russian Federation"),
        ("zh_CN", "俄罗斯"),
        ("zh_HK", "俄羅斯聯邦"),
        ("zh_TW", "俄羅斯聯邦"),
        ("zu", "Russian Federation"),
    ];
    #[cfg(all(feature = "ru", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 61.52401;
        pub const LONGITUDE: f64 = 105.318756;
        pub const MAX_LATITUDE: f64 = 82.1673907;
        pub const MAX_LONGITUDE: f64 = -168.97788;
        pub const MIN_LATITUDE: f64 = 41.185353;
        pub const MIN_LONGITUDE: f64 = 19.6160999;
        pub const NORTHEAST_LATITUDE: f64 = 82.1673907;
        pub const NORTHEAST_LONGITUDE: f64 = -168.97788;
        pub const SOUTHWEST_LATITUDE: f64 = 41.185353;
        pub const SOUTHWEST_LONGITUDE: f64 = 19.6160999;
    }
}
#[cfg(all(feature = "ru", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 61.52401,
            longitude: 105.318756,
            max_latitude: 82.1673907,
            max_longitude: -168.97788,
            min_latitude: 41.185353,
            min_longitude: 19.6160999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 82.1673907,
                    longitude: -168.97788,
                },
                southwest: CountryGeoBound {
                    latitude: 41.185353,
                    longitude: 19.6160999,
                },
            },
        }
    }
}

#[cfg(all(feature = "ru", feature = "subdivisions"))]
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
                    "AD",
                    Subdivision{
                        name: "AD",
                        country_alpha2: Alpha2::RU,
                        code: "AD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.8229155), longitude: Some(40.1754463), max_latitude: Some(45.21684), min_latitude: Some(43.757678), max_longitude: Some(40.7744969), min_longitude: Some(38.6818529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Adigea"), ("am", "አድጌያ"), ("ar", "أديغيا"), ("az", "Adıgey"), ("be", "Адыгея"), ("bg", "Адигея"), ("bn", "আদিগিয\u{9bc}\u{9be} রিপ\u{9be}বলিক"), ("bs", "Adigeja"), ("ca", "Adiguèsia"), ("ccp", "𑄃𑄓\u{11128}𑄉\u{11128}𑄠"), ("ceb", "Respublika Adygeya"), ("cs", "Adygsko"), ("cy", "Adygea"), ("da", "Adygejien"), ("de", "Adygeja"), ("el", "Δημοκρατία της Αντιγκέα"), ("en", "Adygea"), ("es", "Adigueya"), ("et", "Adõgee"), ("eu", "Adigea"), ("fa", "آدیغیه"), ("fi", "Adygeia"), ("fr", "Adyguée"), ("ga", "Adygea"), ("gl", "Adiguesia"), ("gu", "રિપબ\u{acd}લિક ઓફ અદિગ\u{ac7}અ"), ("he", "אדיגיה"), ("hi", "आदिग\u{947}या"), ("hr", "Adigejska"), ("hu", "Adigeföld"), ("hy", "Ադիգեա"), ("id", "Adygea"), ("is", "Adigea"), ("it", "Adighezia"), ("ja", "アディゲ共和国"), ("ka", "ადიღე"), ("kk", "Адыгея"), ("kn", "ಆಡ\u{cbf}ಜ\u{cbf}ಯಾ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "아디게야 공화국"), ("ky", "Адыгэ Республикасы"), ("lt", "Adygėja"), ("lv", "Adigeja"), ("mk", "Адигеја"), ("mn", "Адыгей"), ("mr", "अदिग\u{947}या प\u{94d}रजासत\u{94d}ताक"), ("ms", "Adygea"), ("nb", "Adygia"), ("nl", "Adygea"), ("no", "Adygia"), ("pl", "Adygeja"), ("pt", "Adiguésia"), ("ro", "Adîgheia"), ("ru", "Адыгея"), ("si", "අඩ\u{dd2}ග\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Adygejsko"), ("sl", "Adigeja"), ("sq", "Adigjeja"), ("sr", "Адигеја"), ("sr_Latn", "Adigeja"), ("sv", "Adygeiska republiken"), ("sw", "Adygea"), ("ta", "அடிகேய\u{bbe}"), ("te", "అడ\u{c40}గ\u{c3f}య\u{c3e} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐอะด\u{e35}เกยา"), ("tr", "Adıge Cumhuriyeti"), ("uk", "Адигея"), ("ur", "ادیگیا"), ("uz", "Adigeya"), ("vi", "Adygea"), ("yue", "阿迪格"), ("yue_Hans", "阿迪格"), ("zh", "阿迪格共和国")]),
                        unofficial_name_list: ["Adygei Republic", "Adygeja"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "AL",
                        country_alpha2: Alpha2::RU,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7671943), longitude: Some(37.606158), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Altai"), ("ar", "جمهورية ألطاي"), ("az", "Altay Respublikası"), ("be", "Рэспубліка Алтай"), ("bg", "Алтай"), ("bn", "আলত\u{9be}ই প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Altajska republika"), ("ca", "República de l’Altai"), ("ccp", "𑄃𑄣\u{11134}𑄑\u{1112d}"), ("ceb", "Respublika Altay"), ("cs", "Republika Altaj"), ("cy", "Gweriniaeth Altai"), ("da", "Altai"), ("de", "Republik Altai"), ("el", "Δημοκρατία των Αλτάι"), ("en", "Altai"), ("es", "Altái"), ("et", "Altai Vabariik"), ("eu", "Altaiko Errepublika"), ("fa", "جمهوری آلتایی"), ("fi", "Altain tasavalta"), ("fr", "République de l’Altaï"), ("ga", "Poblacht Altae"), ("gl", "República de Altai"), ("gu", "અલ\u{acd}તાઇ રિપબ\u{acd}લિક"), ("he", "אלטאי"), ("hi", "अल\u{94d}ताई गणराज\u{94d}य"), ("hr", "Altajska"), ("hu", "Altaj Köztársaság"), ("hy", "Ալթայի հանրապետություն"), ("id", "Republik Altai"), ("it", "Repubblica dell’Altaj"), ("ja", "アルタイ共和国"), ("ka", "ალთაის რესპუბლიკა"), ("kk", "Алтай Республикасы"), ("kn", "ಆಲ\u{ccd}ಟಾಯ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "알타이 공화국"), ("ky", "Алтай Республикасы"), ("lt", "Altajaus Respublika"), ("lv", "Altaja Republika"), ("mk", "Република Алтај"), ("mn", "Алтайн Бүгд Найрамдах Улс"), ("mr", "आल\u{94d}ताय प\u{94d}रजासत\u{94d}ताक"), ("ms", "Republik Altai"), ("nb", "Altaj"), ("nl", "Altaj"), ("no", "Altaj"), ("pl", "Republika Ałtaju"), ("pt", "Altai (república)"), ("ro", "Republica Altai"), ("ru", "Республика Алтай"), ("si", "අල\u{dca}ට\u{dcf}ය\u{dd2} ජනරජය"), ("sk", "Altajsko"), ("sl", "Republika Altaj"), ("sr", "Алтај"), ("sr_Latn", "Altaj"), ("sv", "Altajrepubliken"), ("sw", "Jamhuri ya Altai"), ("ta", "அல\u{bcd}த\u{bcd}த\u{bbe}ய\u{bcd} குடியரசு"), ("te", "అల\u{c4d}ట\u{c3e}య\u{c3f} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "อ\u{e31}ลไต"), ("tr", "Altay Cumhuriyeti"), ("uk", "Республіка Алтай"), ("ur", "التائی جمہوریہ"), ("uz", "Oltoy Respublikasi"), ("vi", "Cộng hòa Altai"), ("yue", "阿爾泰"), ("yue_Hans", "阿尔泰"), ("zh", "阿尔泰共和国")]),
                        unofficial_name_list: ["Altaj", "Altay", "Gorno-Altaj"].to_vec(),
                    }
                ),
                (
                    "ALT",
                    Subdivision{
                        name: "ALT",
                        country_alpha2: Alpha2::RU,
                        code: "ALT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.7936298), longitude: Some(82.6758596), max_latitude: Some(54.45086), min_latitude: Some(50.6395136), max_longitude: Some(87.16908889999999), min_longitude: Some(77.8891551)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Altai-krai"), ("ar", "ألطاي كراي"), ("az", "Altay diyarı"), ("be", "Алтайскі край"), ("bg", "Алтайски край"), ("bn", "আলত\u{9be}ই ক\u{9cd}র\u{9be}ই"), ("bs", "Altajski kraj"), ("ca", "Territori de l’Altai"), ("ccp", "𑄃𑄣\u{11134}𑄑\u{1112d} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Altayskiy Kray"), ("cs", "Altajský kraj"), ("cy", "Crai Altai"), ("da", "Altaj kraj"), ("de", "Region Altai"), ("el", "Κράι Αλτάι"), ("en", "Altai Krai"), ("es", "Krai de Altái"), ("et", "Altai krai"), ("eu", "Altai kraia"), ("fa", "سرزمین آلتایی"), ("fi", "Altain aluepiiri"), ("fr", "kraï de l’Altaï"), ("ga", "Críoch Altae"), ("gu", "અલ\u{acd}તાઇ ક\u{acd}રાઇ"), ("he", "מחוז אלטאי"), ("hi", "अल\u{94d}ताई क\u{94d}राय"), ("hr", "Altajski kraj"), ("hu", "Altaji határterület"), ("hy", "Ալթայի երկրամաս"), ("id", "Krai Altai"), ("it", "Territorio dell’Altaj"), ("ja", "アルタイ地方"), ("ka", "ალთაის მხარე"), ("kn", "ಆಲ\u{ccd}ಟಾಯ\u{ccd} ಕ\u{ccd}ರೈ"), ("ko", "알타이 지방"), ("ky", "Алтай крайы"), ("lt", "Altajaus kraštas"), ("lv", "Altaja novads"), ("mk", "Алтајски крај"), ("mn", "Алтайн хязгаар"), ("mr", "आल\u{94d}ताय क\u{94d}राय"), ("ms", "Jajahan Altai"), ("nb", "Altaj²"), ("nl", "Kraj Altaj"), ("no", "Altaj²"), ("pa", "ਅਲਤਾਈ ਕ\u{a4d}ਰਾਈ"), ("pl", "Kraj Ałtajski"), ("pt", "Krai de Altai"), ("ro", "Ținutul Altai"), ("ru", "Алтайский край"), ("si", "අල\u{dca}ට\u{dcf}ය\u{dd2} ක\u{dca}\u{200d}රය\u{dd2}"), ("sk", "Altajský kraj"), ("sr", "Алтајска Покрајина"), ("sr_Latn", "Altajska Pokrajina"), ("sv", "Altaj kraj"), ("sw", "Altai Krai"), ("ta", "அல\u{bcd}த\u{bcd}த\u{bbe}ய\u{bcd} பிரதேசம\u{bcd}"), ("te", "అల\u{c4d}ట\u{c3e}య\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "อ\u{e31}ลไตไคร"), ("tr", "Altay Krayı"), ("uk", "Алтайський край"), ("ur", "التائی کرائی"), ("uz", "Oltoy oʻlkasi"), ("vi", "Altai"), ("yue", "阿爾泰邊疆區"), ("yue_Hans", "阿尔泰边疆区"), ("zh", "阿尔泰边疆区")]),
                        unofficial_name_list: ["Altai Kray", "Altaj", "Altajskij Kray", "Altajskiy Kraj"].to_vec(),
                    }
                ),
                (
                    "AMU",
                    Subdivision{
                        name: "AMU",
                        country_alpha2: Alpha2::RU,
                        code: "AMU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.60350649999999), longitude: Some(127.4801721), max_latitude: Some(57.0585329), min_latitude: Some(48.852265), max_longitude: Some(134.920255), min_longitude: Some(119.66795)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Amoer-oblast"), ("ar", "أمور أوبلاست"), ("az", "Amur vilayəti"), ("be", "Амурская вобласць"), ("bg", "Амурска област"), ("bn", "আম\u{9c1}র ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Amurska oblast"), ("ca", "Província de l’Amur"), ("ccp", "𑄃𑄟\u{1112a}𑄢\u{11134}"), ("ceb", "Amurskaya Oblast’"), ("cs", "Amurská oblast"), ("cy", "Oblast Amur"), ("da", "Amur oblast"), ("de", "Oblast Amur"), ("el", "Περιφέρεια Αμούρ"), ("en", "Amur"), ("es", "Amur"), ("et", "Amuuri oblast"), ("eu", "Amur oblasta"), ("fa", "استان آمور"), ("fi", "Amurin alue"), ("fr", "Oblast d’Amour"), ("ga", "Cúige Amur"), ("gl", "Óblast de Amur"), ("gu", "અમ\u{ac1}ર ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אמור"), ("hi", "आम\u{941}र ओब\u{94d}लास\u{94d}ट"), ("hr", "Amurska oblast"), ("hu", "Amuri terület"), ("hy", "Ամուրի մարզ"), ("id", "Oblast Amur"), ("is", "Amúrfylki"), ("it", "Oblast’ dell’Amur"), ("ja", "アムール州"), ("ka", "ამურის ოლქი"), ("kn", "ಅಮುರ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "아무르 주"), ("lt", "Amūro sritis"), ("lv", "Amūras apgabals"), ("mk", "Амурска област"), ("mr", "आम\u{942}र ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Amur"), ("nb", "Amur"), ("nl", "Oblast Amoer"), ("no", "Amur"), ("pl", "Obwód amurski"), ("pt", "Oblast de Amur"), ("ro", "Regiunea Amur"), ("ru", "Амурская область"), ("si", "අම\u{dd4}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Amurská oblasť"), ("sl", "Amurska oblast"), ("sr", "Амурска област"), ("sr_Latn", "Amurska oblast"), ("sv", "Amur oblast"), ("sw", "Amur Oblast"), ("ta", "அமுர\u{bcd} ஓபல\u{bbe}சுத\u{bcd}து"), ("te", "అమూర\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นปกครองอาม\u{e39}ร\u{e4c}"), ("tr", "Amur Oblastı"), ("uk", "Амурська область"), ("ur", "آمور اوبلاست"), ("uz", "Amur viloyati"), ("vi", "Amur"), ("yue", "阿穆爾州"), ("yue_Hans", "阿穆尔州"), ("zh", "阿穆尔州")]),
                        unofficial_name_list: ["Amurskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "ARK",
                    Subdivision{
                        name: "ARK",
                        country_alpha2: Alpha2::RU,
                        code: "ARK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(63.2852803), longitude: Some(42.5884191), max_latitude: Some(81.8581221), min_latitude: Some(60.63212300000001), max_longitude: Some(69.04823499999999), min_longitude: Some(35.502945)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Archangelsk-oblast"), ("ar", "أرخانجيلسك أوبلاست"), ("az", "Arxangelsk vilayəti"), ("be", "Архангельская вобласць"), ("bg", "Архангелска област"), ("bn", "আরখ\u{9be}নগেলস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Arhangelska oblast"), ("ca", "Província d’Arkhànguelsk"), ("ccp", "𑄃𑄢\u{11134}𑄈𑄚\u{11134}𑄉𑄬𑄣\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Arkhangel’skaya Oblast’"), ("cs", "Archangelská oblast"), ("cy", "Oblast Arkhangelsk"), ("da", "Arkhangelsk oblast"), ("de", "Oblast Archangelsk"), ("el", "Περιφέρεια Αρχάγγελσκ"), ("en", "Arkhangelsk"), ("es", "Óblast de Arjánguelsk"), ("et", "Arhangelski oblast"), ("eu", "Arkhangelsk oblasta"), ("fa", "استان آرخانگلسک"), ("fi", "Arkangelin alue"), ("fr", "Oblast d’Arkhangelsk"), ("ga", "Cúige Arkhangelsk"), ("gl", "Óblast de Arjánguelsk"), ("gu", "અર\u{acd}ખા\u{a82}ગ\u{ac7}લસ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ארכנגלסק"), ("hi", "अर\u{94d}खा\u{902}ग\u{947}ल\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Arhangelska oblast"), ("hu", "Arhangelszki terület"), ("hy", "Արխանգելսկի մարզ"), ("id", "Oblast Arkhangelsk"), ("is", "Arkangelskfylki"), ("it", "Oblast’ di Arcangelo"), ("ja", "アルハンゲリスク州"), ("ka", "არხანგელსკის ოლქი"), ("kk", "Архангельск облысы"), ("kn", "ಆರ\u{ccd}ಖಾಂಗ\u{cc6}ಲ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "아르한겔스크 주"), ("lt", "Archangelsko sritis"), ("lv", "Arhangeļskas apgabals"), ("mk", "Архангелска област"), ("mn", "Архангельск муж"), ("mr", "अर\u{94d}खा\u{902}ग\u{947}ल\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Arkhangelsk"), ("nb", "Arkhangelsk"), ("nl", "Oblast Archangelsk"), ("no", "Arkhangelsk"), ("pl", "Obwód archangielski"), ("pt", "Oblast de Arkhangelsk"), ("ro", "Regiunea Arhanghelsk"), ("ru", "Архангельская область"), ("si", "අර\u{dca}කන\u{dca}ගේල\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Archangeľská oblasť"), ("sl", "Arhangelska oblast"), ("sr", "Архангелска област"), ("sr_Latn", "Arhangelska oblast"), ("sv", "Archangelsk oblast"), ("sw", "Arkhangelsk Oblast"), ("ta", "ஆர\u{bcd}க\u{bcd}க\u{bbe}ங\u{bcd}கெல\u{bcd}சிக\u{bcd} ஓபல\u{bbe}சுத\u{bcd}து"), ("te", "అర\u{c4d}క\u{c3e}జ\u{c46}ల\u{c4d}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "อาร\u{e4c}ค\u{e31}นเกลสค\u{e4c}"), ("tr", "Arhangelsk Oblastı"), ("uk", "Архангельська область"), ("ur", "آرخانگلسک اوبلاست"), ("uz", "Arxangel viloyati"), ("vi", "Arkhangelsk"), ("yue", "阿爾漢格爾斯克州"), ("yue_Hans", "阿尔汉格尔斯克州"), ("zh", "阿尔汉格尔斯克州")]),
                        unofficial_name_list: ["Arhangelsk", "Arhangelskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "AST",
                    Subdivision{
                        name: "AST",
                        country_alpha2: Alpha2::RU,
                        code: "AST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1321166), longitude: Some(48.0610115), max_latitude: Some(48.8653439), min_latitude: Some(45.17255590000001), max_longitude: Some(49.279398), min_longitude: Some(44.9707619)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Astrachan-oblast"), ("ar", "أوبلاست أستراخان"), ("az", "Həştərxan vilayəti"), ("be", "Астраханская вобласць"), ("bg", "Астраханска област"), ("bn", "আস\u{9cd}ট\u{9cd}রখ\u{9be}ন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Astrahanska oblast"), ("ca", "Província d’Àstrakhan"), ("ccp", "𑄃𑄌\u{11134}𑄑\u{11133}𑄢𑄈𑄚\u{11134}"), ("ceb", "Astrakhanskaya Oblast’"), ("cs", "Astrachaňská oblast"), ("cy", "Oblast Astrakhan"), ("da", "Astrakhan oblast"), ("de", "Oblast Astrachan"), ("el", "Περιφέρεια Άστραχαν"), ("en", "Astrakhan"), ("es", "Astracán"), ("et", "Astrahani oblast"), ("eu", "Astrakhan oblasta"), ("fa", "استان آستراخان"), ("fi", "Astrahanin alue"), ("fr", "Oblast d’Astrakhan"), ("ga", "Cúige na hAstracháine"), ("gl", "Óblast de Astrakán"), ("gu", "એસ\u{acd}ટ\u{acd}રાખાનન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אסטרחן"), ("hi", "अस\u{94d}त\u{94d}राखान ओब\u{94d}लास\u{94d}ट"), ("hr", "Astrahanska oblast"), ("hu", "Asztraháni terület"), ("hy", "Աստրախանի մարզ"), ("id", "Oblast Astrakhan"), ("is", "Astrakanfylki"), ("it", "Oblast’ di Astrachan’"), ("ja", "アストラハン州"), ("ka", "ასტრახანის ოლქი"), ("kk", "Астрахан облысы"), ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರಾಖಾನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "아스트라한 주"), ("lt", "Astrachanės sritis"), ("lv", "Astrahaņas apgabals"), ("mk", "Астраханска област"), ("mr", "आस\u{94d}त\u{94d}राखान ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Astrakhan"), ("nb", "Astrakhan"), ("nl", "Oblast Astrachan"), ("no", "Astrakhan"), ("pl", "Obwód astrachański"), ("pt", "Oblast de Astracã"), ("ro", "Regiunea Astrahan"), ("ru", "Астраханская область"), ("si", "අස\u{dca}ට\u{dca}\u{200d}ර\u{dcf}ඛ\u{dcf}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Astrachánska oblasť"), ("sl", "Astrahanska oblast"), ("sr", "Астраханска област"), ("sr_Latn", "Astrahanska oblast"), ("sv", "Astrachan oblast"), ("sw", "Astrakhan Oblast"), ("ta", "அஸ\u{bcd}த\u{bcd}ரக\u{bbe}ன\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఆస\u{c4d}ట\u{c4d}ర\u{c3e}ఖ\u{c3e}న\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แอสทร\u{e31}คฮาน โอแบลส"), ("tr", "Astrahan Oblastı"), ("uk", "Астраханська область"), ("ur", "استراخان اوبلاست"), ("uz", "Astraxan viloyati"), ("vi", "Astrakhan"), ("yue", "阿斯特拉罕州"), ("yue_Hans", "阿斯特拉罕州"), ("zh", "阿斯特拉罕州")]),
                        unofficial_name_list: ["Astrahan", "Astrahanska Oblast"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::RU,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.2312172), longitude: Some(56.1645257), max_latitude: Some(56.53352), min_latitude: Some(51.57122589999999), max_longitude: Some(60.00295010000001), min_longitude: Some(53.1579969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Basjkortostan"), ("am", "ባሽኮርቶስታን"), ("ar", "باشكورستان"), ("az", "Başqırdıstan"), ("be", "Башкартастан"), ("bg", "Башкирия"), ("bn", "রিপ\u{9be}বলিক অফ ব\u{9be}শক\u{9c1}রত\u{9c1}স\u{9cd}ত\u{9be}ন"), ("bs", "Baškortostan"), ("ca", "Baixkíria"), ("ccp", "𑄝𑄌\u{11134}𑄇\u{11127}𑄢\u{11134}𑄑\u{11127}𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Bashkortostan"), ("cs", "Baškortostán"), ("cy", "Bashkortostan"), ("da", "Basjkortostan"), ("de", "Baschkortostan"), ("el", "Δημοκρατία του Μπασκορτοστάν"), ("en", "Bashkortostan"), ("es", "Bashkortostán"), ("et", "Baškortostan"), ("eu", "Baxkortostan"), ("fa", "باشقیرستان"), ("fi", "Baškortostan"), ("fr", "Bachkirie"), ("ga", "An Bhaisceartastáin"), ("gl", "Bashkortostán"), ("gu", "રિપબ\u{acd}લિક ઓફ બાશ\u{acd}કોર\u{acd}ટોસ\u{acd}તાન"), ("he", "בשקורטוסטן"), ("hi", "बश\u{94d}कोरतोस\u{94d}तान"), ("hr", "Baškirska"), ("hu", "Baskíria"), ("hy", "Բաշկորտոստան"), ("id", "Bashkortostan"), ("is", "Basjkortostan"), ("it", "Baschiria"), ("ja", "バシコルトスタン共和国"), ("ka", "ბაშკირეთი"), ("kk", "Башқұртстан"), ("kn", "ಬ\u{ccd}ಯಾಷ\u{ccd}ಕಾರ\u{ccd}ಟೊಸ\u{ccd}ಟಾನ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "바시키르 공화국"), ("ky", "Башкортостан"), ("lt", "Baškirija"), ("lv", "Baškortostāna"), ("mk", "Башкортостан"), ("ml", "ബ\u{d3e}ഷ\u{d4d}കോർടോസ\u{d4d}ഥ\u{d3e}ൻ"), ("mn", "Башкортостан"), ("mr", "बाश\u{94d}कोर\u{94d}तोस\u{94d}तान प\u{94d}रजासत\u{94d}ताक"), ("ms", "Bashkortostan"), ("nb", "Basjkortostan"), ("nl", "Basjkirostan"), ("no", "Basjkortostan"), ("pa", "ਬਸ\u{a3c}ਕ\u{a4b}ਰਤ\u{a4b}ਸਤਾਨ"), ("pl", "Baszkiria"), ("pt", "Bascortostão"), ("ro", "Bașchiria"), ("ru", "Башкортостан"), ("si", "බ\u{dcf}ෂ\u{dca}කොර\u{dca}ටොස\u{dca}ට\u{dcf}න\u{dca} ජනරජය"), ("sk", "Baškirsko"), ("sl", "Baškortostan"), ("sq", "Bashkortostani"), ("sr", "Башкортостан"), ("sr_Latn", "Baškortostan"), ("sv", "Basjkirien"), ("sw", "Bashkortostan"), ("ta", "ப\u{bbe}ஷ\u{bcd}கொர\u{bcd}டொஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "బష\u{c4d}క\u{c4a}ర\u{c4d}త\u{c4b}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "สาธารณร\u{e31}ฐบ\u{e31}ชคอร\u{e4c}โตสถาน"), ("tk", "Başgyrdystan"), ("tr", "Başkurdistan"), ("uk", "Башкортостан"), ("ur", "باشکورتوستان"), ("uz", "Boshqirdiston"), ("vi", "Bashkortostan"), ("yue", "巴什科爾托斯坦"), ("yue_Hans", "巴什科尔托斯坦"), ("zh", "巴什科尔托斯坦共和国")]),
                        unofficial_name_list: ["Baškortostan"].to_vec(),
                    }
                ),
                (
                    "BEL",
                    Subdivision{
                        name: "BEL",
                        country_alpha2: Alpha2::RU,
                        code: "BEL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.71069259999999), longitude: Some(37.7533377), max_latitude: Some(51.4325619), min_latitude: Some(49.796403), max_longitude: Some(39.2751271), min_longitude: Some(35.3285271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Belgorod-oblast"), ("ar", "أوبلاست بيلغورود"), ("az", "Belqorod vilayəti"), ("be", "Белгародская вобласць"), ("bg", "Белгородска област"), ("bn", "বেল\u{9cd}গ\u{9cd}রোদ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Belgorodska oblast"), ("ca", "Província de Bèlgorod"), ("ccp", "𑄝𑄬𑄣\u{11134}𑄉\u{11127}𑄢\u{11127}𑄖\u{11134}"), ("ceb", "Belgorodskaya Oblast’"), ("cs", "Belgorodská oblast"), ("cy", "Oblast Belgorod"), ("da", "Belgorod oblast"), ("de", "Oblast Belgorod"), ("el", "Περιφέρεια Μπέλγκοροντ"), ("en", "Belgorod"), ("es", "Óblast de Bélgorod"), ("et", "Belgorodi oblast"), ("eu", "Belgorod oblasta"), ("fa", "استان بلگورود"), ("fi", "Belgorodin alue"), ("fr", "oblast de Belgorod"), ("ga", "Cúige Bhelgorod"), ("gu", "બ\u{ac7}લ\u{acd}ગોરોડ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז בלגורוד"), ("hi", "ब\u{947}ल\u{94d}गोरोद ओब\u{94d}लास\u{94d}त"), ("hr", "Belgorodska oblast"), ("hu", "Belgorodi terület"), ("hy", "Բելգորոդի մարզ"), ("id", "Oblast Belgorod"), ("is", "Belgorodfylki"), ("it", "Oblast’ di Belgorod"), ("ja", "ベルゴロド州"), ("ka", "ბელგოროდის ოლქი"), ("kn", "ಬ\u{cc6}ಲ\u{ccd}ಗೊರೊಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "벨고로드 주"), ("lt", "Belgorodo sritis"), ("lv", "Belgorodas apgabals"), ("mk", "Белгородска област"), ("mr", "ब\u{947}ल\u{94d}गोरोद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Belgorod"), ("nb", "Belgorod"), ("nl", "Oblast Belgorod"), ("no", "Belgorod"), ("pl", "Obwód biełgorodzki"), ("pt", "Oblast de Belgorod"), ("ro", "Regiunea Belgorod"), ("ru", "Белгородская область"), ("si", "බෙල\u{dca}ගොරඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Belgorodská oblasť"), ("sl", "Belgorodska oblast"), ("sr", "Белгородска област"), ("sr_Latn", "Belgorodska oblast"), ("sv", "Belgorod oblast"), ("sw", "Belgorod Oblast"), ("ta", "பெல\u{bcd}கோரத\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "బ\u{c46}ల\u{c4d}గ\u{c4b}ర\u{c4b}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นเบลโกรอด"), ("tr", "Belgorod Oblastı"), ("uk", "Бєлгородська область"), ("ur", "بلگورود اوبلاست"), ("uz", "Belgorod viloyati"), ("vi", "Belgorod"), ("yue", "別爾哥羅德州"), ("yue_Hans", "别尔哥罗德州"), ("zh", "别尔哥罗德州")]),
                        unofficial_name_list: ["Belgorodskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "BRY",
                    Subdivision{
                        name: "BRY",
                        country_alpha2: Alpha2::RU,
                        code: "BRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.0408599), longitude: Some(33.2690899), max_latitude: Some(54.03629910000001), min_latitude: Some(51.844038), max_longitude: Some(35.32127810000001), min_longitude: Some(31.24210489999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Brjansk-oblast"), ("ar", "بريانسك أوبلاست"), ("az", "Bryansk vilayəti"), ("be", "Бранская вобласць"), ("bg", "Брянска област"), ("bn", "ব\u{9cd}রিয\u{9bc}\u{9be}নস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Brjanska oblast"), ("ca", "Província de Briansk"), ("ccp", "𑄝\u{11133}𑄢\u{1112d}𑄠𑄚\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Bryanskaya Oblast’"), ("cs", "Brjanská oblast"), ("cy", "Oblast Bryansk"), ("da", "Brjansk oblast"), ("de", "Oblast Brjansk"), ("el", "Περιφέρεια Μπριάνσκ"), ("en", "Bryansk"), ("es", "Óblast de Briansk"), ("et", "Brjanski oblast"), ("eu", "Briansk oblasta"), ("fa", "استان بریانسک"), ("fi", "Brjanskin alue"), ("fr", "Oblast de Briansk"), ("ga", "Cúige Bryansk"), ("gu", "બ\u{acd}રાયનસ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז בריאנסק"), ("hi", "ब\u{94d}रिया\u{902}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Brjanska oblast"), ("hu", "Brjanszki terület"), ("hy", "Բրյանսկի մարզ"), ("id", "Oblast Bryansk"), ("it", "Oblast’ di Brjansk"), ("ja", "ブリャンスク州"), ("ka", "ბრიანსკის ოლქი"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಯಾನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "브랸스크 주"), ("lt", "Briansko sritis"), ("lv", "Brjanskas apgabals"), ("mk", "Брјанска област"), ("mn", "Брянск муж"), ("mr", "ब\u{94d}र\u{94d}यान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Bryansk"), ("nb", "Brjansk"), ("ne", "ब\u{94d}रान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("nl", "Oblast Brjansk"), ("no", "Brjansk"), ("pl", "Obwód briański"), ("pt", "Oblast de Briansk"), ("ro", "Regiunea Briansk"), ("ru", "Брянская область"), ("si", "බ\u{dca}\u{200d}රයන\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Brianska oblasť"), ("sl", "Brjanska oblast"), ("sr", "Брјанска област"), ("sr_Latn", "Brjanska oblast"), ("sv", "Brjansk oblast"), ("sw", "Bryansk Oblast"), ("ta", "ப\u{bcd}ரையன\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "బ\u{c4d}రయ\u{c3e}ంక\u{c4d}స\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นบร\u{e35}อ\u{e31}นสค\u{e4c}"), ("tr", "Bryansk Oblastı"), ("uk", "Брянська область"), ("ur", "بریانسک اوبلاست"), ("uz", "Bryansk viloyati"), ("vi", "Bryansk"), ("yue", "布良斯克州"), ("yue_Hans", "布良斯克州"), ("zh", "布良斯克州")]),
                        unofficial_name_list: ["Brjansk", "Brjanskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::RU,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7685161), longitude: Some(37.6425744), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Boerjatië"), ("am", "ቡርያቲያ"), ("ar", "بورياتيا"), ("az", "Buryatiya"), ("be", "Бурація"), ("bg", "Бурятия"), ("bn", "ব\u{9c1}রিয\u{9bc}\u{9be}ত রিপ\u{9be}বলিক"), ("bs", "Burjatija"), ("ca", "Buriàtia"), ("ccp", "𑄝\u{1112a}𑄢\u{11128}𑄠𑄖\u{11134}"), ("ceb", "Respublika Buryatiya"), ("cs", "Burjatsko"), ("da", "Burjatien"), ("de", "Burjatien"), ("el", "Δημοκρατία της Μπουργιατίας"), ("en", "Buryat"), ("es", "Buriatia"), ("et", "Burjaatia"), ("eu", "Buriatia"), ("fa", "بوریاتیا"), ("fi", "Burjatia"), ("fr", "Bouriatie"), ("ga", "An Bhuiriáit"), ("gl", "Buriatia"), ("gu", "બ\u{ac1}રયટ રિપબ\u{acd}લિક"), ("he", "בוריאטיה"), ("hi", "ब\u{941}र\u{94d}यातिया"), ("hr", "Burjatska"), ("hu", "Burjátföld"), ("hy", "Բուրյաթիա"), ("id", "Buryatia"), ("it", "Buriazia"), ("ja", "ブリヤート共和国"), ("ka", "ბურიატეთი"), ("kk", "Бурятия"), ("kn", "ಬುರ\u{cbf}ತ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "부랴트 공화국"), ("ky", "Бурятия"), ("lt", "Buriatija"), ("lv", "Burjatija"), ("mk", "Бурјатија"), ("mn", "Бүгд Найрамдах Буриад Улс"), ("mr", "ब\u{941}र\u{94d}यातिया"), ("ms", "Republik Buryatia"), ("nb", "Burjatia"), ("nl", "Boerjatië"), ("no", "Burjatia"), ("pl", "Buriacja"), ("pt", "Buriácia"), ("ro", "Buriația"), ("ru", "Бурятия"), ("si", "බර\u{dca}යට\u{dca} ජනරජය"), ("sk", "Buriatsko"), ("sl", "Burjatija"), ("sq", "Burjatia"), ("sr", "Бурјатија"), ("sr_Latn", "Burjatija"), ("sv", "Burjatien"), ("sw", "Buryatia"), ("ta", "புரிய\u{bbe}த\u{bcd}திய\u{bbe}"), ("te", "బుర\u{c4d}య\u{c3e}ట\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐบ\u{e39}เร\u{e35}ยต\u{e35}ยา"), ("tr", "Buryatya"), ("uk", "Бурятія"), ("ur", "بوریاتیا"), ("uz", "Buryatiya"), ("vi", "Buryatia"), ("yue", "布里亞特"), ("yue_Hans", "布里亚特"), ("zh", "布里亞特共和國")]),
                        unofficial_name_list: ["Burjatija", "Buryat Republic"].to_vec(),
                    }
                ),
                (
                    "CE",
                    Subdivision{
                        name: "CE",
                        country_alpha2: Alpha2::RU,
                        code: "CE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7521689), longitude: Some(37.5886438), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjetsjnië"), ("am", "ቸችኒያ"), ("ar", "الشيشان"), ("az", "Çeçenistan"), ("be", "Чачня"), ("bg", "Чеченска република"), ("bn", "চেচনিয\u{9bc}\u{9be}"), ("bs", "Čečenija"), ("ca", "Txetxènia"), ("ccp", "𑄌𑄬𑄌𑄬𑄚\u{11134}"), ("ceb", "Chechenskaya Respublika"), ("cs", "Čečensko"), ("cy", "Tsietsnia"), ("da", "Tjetjenien"), ("de", "Tschetschenien"), ("el", "Δημοκρατία της Τσετσενίας"), ("en", "Chechen"), ("es", "Chechenia"), ("et", "Tšetšeeni Vabariik"), ("eu", "Txetxenia"), ("fa", "چچن"), ("fi", "Tšetšenia"), ("fr", "Tchétchénie"), ("ga", "An tSeisnia"), ("gu", "ચ\u{ac7}ચન રિપબ\u{acd}લિક"), ("he", "צ׳צ׳ניה"), ("hi", "च\u{947}चन\u{94d}या"), ("hr", "Čečenija"), ("hu", "Csecsenföld"), ("hy", "Չեչնիա"), ("id", "Chechnya"), ("is", "Téténía"), ("it", "Cecenia"), ("ja", "チェチェン共和国"), ("jv", "Chechnya"), ("ka", "ჩეჩნეთი"), ("kn", "ಚ\u{cc6}ಚ\u{cc6}ನ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "체첸 공화국"), ("ky", "Чеченстан"), ("lt", "Čečėnija"), ("lv", "Čečenija"), ("mk", "Чеченија"), ("ml", "ചെച\u{d4d}\u{200c}നിയ"), ("mn", "Чечень"), ("mr", "च\u{947}चन\u{94d}या"), ("ms", "Chechnya"), ("nb", "Tsjetsjenia"), ("nl", "Tsjetsjenië"), ("no", "Tsjetsjenia"), ("pa", "ਚ\u{a48}ਚਨੀਆ"), ("pl", "Czeczenia"), ("pt", "Chechênia"), ("ro", "Cecenia"), ("ru", "Чечня"), ("sd", "چيچنيا"), ("si", "චෙච\u{dca}න\u{dd2}ය\u{dcf}"), ("sk", "Čečensko"), ("sl", "Čečenija"), ("so", "Jeejniya"), ("sq", "Çeçenia"), ("sr", "Чеченија"), ("sr_Latn", "Čečenija"), ("sv", "Tjetjenien"), ("sw", "Chechnya"), ("ta", "செச\u{bcd}சினிய\u{bbe}"), ("te", "చ\u{c46}చ\u{c46}న\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐเชเชน"), ("tr", "Çeçenistan"), ("uk", "Чечня"), ("ur", "شیشان"), ("uz", "Checheniston"), ("vi", "Chechnya"), ("yue", "車臣共和國"), ("yue_Hans", "车臣共和国"), ("zh", "车臣共和国")]),
                        unofficial_name_list: ["Chechen", "Chechenia", "Ichkeria", "Ičkeria", "Čečenija", "Čečens", "Čečenskaja Respublika"].to_vec(),
                    }
                ),
                (
                    "CHE",
                    Subdivision{
                        name: "CHE",
                        country_alpha2: Alpha2::RU,
                        code: "CHE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.43194219999999), longitude: Some(60.87889629999999), max_latitude: Some(56.3648829), min_latitude: Some(51.990638), max_longitude: Some(63.3492892), min_longitude: Some(57.1300707)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjeljabinsk-oblast"), ("ar", "أوبلاست تشيليابنسك"), ("az", "Çelyabinsk vilayəti"), ("be", "Чалябінская вобласць"), ("bg", "Челябинска област"), ("bn", "চেলিয\u{9bc}\u{9be}বিনস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Čeljabinska oblast"), ("ca", "Província de Txeliàbinsk"), ("ccp", "𑄌𑄬𑄣\u{11128}𑄠𑄝\u{11128}𑄚\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Chelyabinskaya Oblast’"), ("cs", "Čeljabinská oblast"), ("cy", "Oblast Chelyabinsk"), ("da", "Tjeljabinsk oblast"), ("de", "Oblast Tscheljabinsk"), ("el", "Περιφέρεια Τσελιάμπινσκ"), ("en", "Chelyabinsk"), ("es", "Cheliábinsk"), ("et", "Tšeljabinski oblast"), ("eu", "Txeliabinskeko oblasta"), ("fa", "استان چلیابینسک"), ("fi", "Tšeljabinskin alue"), ("fr", "Oblast de Tcheliabinsk"), ("ga", "Cúige Chelyabinsk"), ("gu", "ચ\u{ac7}લ\u{acd}યાબિન\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז צ׳ליאבינסק"), ("hi", "च\u{947}ल\u{94d}याबिन\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("hr", "Čeljabinska oblast"), ("hu", "Cseljabinszki terület"), ("hy", "Չելյաբինսկի մարզ"), ("id", "Oblast Chelyabinsk"), ("it", "Oblast’ di Čeljabinsk"), ("ja", "チェリャビンスク州"), ("ka", "ჩელიაბინსკის ოლქი"), ("kk", "Челябі облысы"), ("kn", "ಚ\u{cc6}ಲ\u{ccd}ಯಾಬ\u{cbf}ನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "첼랴빈스크 주"), ("lt", "Čeliabinsko sritis"), ("lv", "Čeļabinskas apgabals"), ("mk", "Чељабинска област"), ("mn", "Челябинск муж"), ("mr", "च\u{947}लियाबिन\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Chelyabinsk"), ("nb", "Tsjeljabinsk"), ("nl", "Oblast Tsjeljabinsk"), ("no", "Tsjeljabinsk"), ("pl", "Obwód czelabiński"), ("pt", "Oblast de Cheliabinsk"), ("ro", "Regiunea Celiabinsk"), ("ru", "Челябинская область"), ("si", "චෙල\u{dd2}අබ\u{dd2}න\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Čeľabinská oblasť"), ("sl", "Čeljabinska oblast"), ("sr", "Чељабинска област"), ("sr_Latn", "Čeljabinska oblast"), ("sv", "Tjeljabinsk oblast"), ("sw", "Chelyabinsk Oblast"), ("ta", "செல\u{bcd}யபின\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "చ\u{c46}ల\u{c4d}య\u{c3e}బ\u{c3f}న\u{c4d}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นเชเล\u{e35}ยบ\u{e34}นสค\u{e4c}"), ("tr", "Çelyabinsk Oblastı"), ("uk", "Челябінська область"), ("ur", "چیلیابنسک اوبلاست"), ("uz", "Chelyabinsk viloyati"), ("vi", "Chelyabinsk"), ("yue", "車里雅賓斯克州"), ("yue_Hans", "车里雅宾斯克州"), ("zh", "车里雅宾斯克州")]),
                        unofficial_name_list: ["Cheljabinsk", "Čeljabinsk", "Čeljabinskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "CHU",
                    Subdivision{
                        name: "CHU",
                        country_alpha2: Alpha2::RU,
                        code: "CHU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(65.6298355), longitude: Some(171.6952159), max_latitude: Some(71.59401129999999), min_latitude: Some(61.8096659), max_longitude: Some(-168.9996789), min_longitude: Some(157.732108)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjoekotka"), ("ar", "أوكروغ تشوكوتكا الذاتية"), ("az", "Çukotka Muxtar Dairəsi"), ("be", "Чукоцкая аўтаномная акруга"), ("bg", "Чукотски автономен окръг"), ("bn", "চ\u{9c1}কোটক\u{9be} স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Čukotski autonomni okrug"), ("ca", "Txukotka"), ("ccp", "𑄌\u{1112a}𑄇\u{1112e}𑄖\u{11134}𑄇 𑄃\u{1112e}𑄇\u{11134}𑄢\u{1112a}𑄇\u{11134}"), ("ceb", "Chukotskiy Avtonomnyy Okrug"), ("cs", "Čukotský autonomní okruh"), ("cy", "Ocrwg Ymreolaethol Chukotka"), ("da", "Tjuktjien"), ("de", "Autonomer Kreis der Tschuktschen"), ("el", "Αυτόνομος θύλακας Τσουκότκα"), ("en", "Chukotka Okrug"), ("es", "Chukotka"), ("et", "Tšuktšimaa"), ("eu", "Txukotk"), ("fa", "چوکوتکا"), ("fi", "Tšukotka"), ("fr", "Tchoukotka"), ("ga", "Ceantar Féinrialaitheach na Siuicsíoch"), ("gl", "Chukotka"), ("gu", "ચ\u{ac1}કોત\u{acd}કા ઓટોનોમસ ઓક\u{acd}રગ"), ("he", "צ׳וקוטקה"), ("hi", "च\u{941}कोतका स\u{94d}वायत\u{94d}त ऑक\u{94d}रग"), ("hr", "Čukotski autonomni okrug"), ("hu", "Csukcsföld"), ("hy", "Չուկոտկայի ինքնավար օկրուգ"), ("id", "Okrug otonom Chukotka"), ("it", "Circondario autonomo della Čukotka"), ("ja", "チュクチ自治管区"), ("ka", "ჩუკოტკის ავტონომიური ოკრუგი"), ("kn", "ಚುಕೊಟ\u{ccd}ಕಾ ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಒಕ\u{ccd}ರುಗ\u{ccd}"), ("ko", "축치 자치구"), ("lt", "Čiukčių autonominė apygarda"), ("lv", "Čukotkas autonomais apvidus"), ("mk", "Чукотски автономен округ"), ("mr", "च\u{941}कोत\u{94d}का स\u{94d}वायत\u{94d}त ऑक\u{94d}र\u{942}ग"), ("ms", "Negeri autonomi Chukotka"), ("nb", "Tsjukotka"), ("nl", "Tsjoekotka"), ("no", "Tsjukotka"), ("pl", "Czukocki Okręg Autonomiczny"), ("pt", "Chukotka"), ("ro", "Districtul autonom Ciukotka"), ("ru", "Чукотский автономный округ"), ("si", "ච\u{dd4}කොට\u{dca}ක\u{dcf} ස\u{dca}ව\u{dcf}ධ\u{dd3}න පළ\u{dcf}ත"), ("sk", "Čukotka"), ("sl", "Čukotka"), ("sr", "Чукотка"), ("sr_Latn", "Čukotka"), ("sv", "Tjuktjien"), ("sw", "Okrug huru ya Chukotka"), ("ta", "சுகோத\u{bcd}க\u{bbe} தன\u{bcd}ன\u{bbe}ட\u{bcd}சி வட\u{bcd}ட\u{bbe}ரம\u{bcd}"), ("te", "చుక\u{c4b}ట\u{c4d}క\u{c3e} అట\u{c3e}నమస\u{c4d} ఓకుర\u{c4d}గ\u{c4d}"), ("th", "เขตปกครองตนเองช\u{e39}คอตคา"), ("tr", "Çukotka Özerk Okrugu"), ("uk", "Чукотський автономний округ"), ("ur", "چوکوتکا خود مختار آکرگ"), ("uz", "Chukotka muhtor okrugi"), ("vi", "Khu tự trị Chukotka"), ("yue", "楚科奇自治區"), ("yue_Hans", "楚科奇自治区"), ("zh", "楚科奇自治区")]),
                        unofficial_name_list: ["Chuckchi", "Čukotka", "Čukotskij Avtonomnyj Okrug", "Čukči"].to_vec(),
                    }
                ),
                (
                    "CU",
                    Subdivision{
                        name: "CU",
                        country_alpha2: Alpha2::RU,
                        code: "CU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.55959920000001), longitude: Some(46.9283536), max_latitude: Some(56.3299659), min_latitude: Some(54.6260479), max_longitude: Some(48.416765), min_longitude: Some(45.91057199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjoewasjië"), ("am", "ቹቫሺያ"), ("ar", "تشوفاشيا"), ("az", "Çuvaşıstan"), ("be", "Чувашыя"), ("bg", "Чувашия"), ("bn", "চ\u{9c1}ভ\u{9be}স রিপ\u{9be}বলিক"), ("bs", "Čuvašija"), ("ca", "Txuvàixia"), ("ccp", "𑄌\u{1112a}𑄞𑄌\u{11134}"), ("ceb", "Chuvashskaya Respublika"), ("cs", "Čuvašsko"), ("da", "Tjuvasjien"), ("de", "Tschuwaschien"), ("el", "Τσουβασία"), ("en", "Chuvash"), ("es", "Chuvasia"), ("et", "Tšuvaššia"), ("eu", "Txuvaxia"), ("fa", "چواشستان"), ("fi", "Tšuvassia"), ("fr", "Tchouvachie"), ("ga", "An tSuvais"), ("gl", "Chuvashia"), ("gu", "ચ\u{ac1}વાશ રિપબ\u{acd}લિક"), ("he", "צ׳ובשיה"), ("hi", "च\u{941}व\u{948}श गणत\u{902}त\u{94d}र"), ("hr", "Čuvaška"), ("hu", "Csuvasföld"), ("hy", "Չուվաշիա"), ("id", "Chuvashia"), ("it", "Ciuvascia"), ("ja", "チュヴァシ共和国"), ("ka", "ჩუვაშეთი"), ("kn", "ಚುವಾಶ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "추바시 공화국"), ("lt", "Čiuvašija"), ("lv", "Čuvašija"), ("mk", "Чувашија"), ("mn", "Чуваш"), ("mr", "च\u{941}वाशिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Chuvashia"), ("nb", "Tsjuvasjia"), ("nl", "Tsjoevasjië"), ("no", "Tsjuvasjia"), ("pl", "Czuwaszja"), ("pt", "Chuváchia"), ("ro", "Ciuvașia"), ("ru", "Чувашия"), ("si", "ච\u{dd4}ව\u{dcf}ෂ\u{dca} ජනරජය"), ("sk", "Čuvašsko"), ("sl", "Čuvašija"), ("sq", "Çuvashia"), ("sr", "Чувашија"), ("sr_Latn", "Čuvašija"), ("sv", "Tjuvasjien"), ("sw", "Chuvashia"), ("ta", "சுவ\u{bbe}சிய\u{bbe}"), ("te", "చువ\u{c3e}శ\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐช\u{e39}ว\u{e31}ช"), ("tk", "Çuwaşystan"), ("tr", "Çuvaşistan"), ("uk", "Чуваська Республіка"), ("ur", "چوواشیا"), ("uz", "Chuvashiya"), ("vi", "Chuvashia"), ("yue", "楚瓦什共和國"), ("yue_Hans", "楚瓦什共和国"), ("zh", "楚瓦什共和国")]),
                        unofficial_name_list: ["Chuvash Republic", "Chuvashskaya Respublika", "Čuvašija", "Čuvašskaja Respublika"].to_vec(),
                    }
                ),
                (
                    "DA",
                    Subdivision{
                        name: "DA",
                        country_alpha2: Alpha2::RU,
                        code: "DA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7599606), longitude: Some(37.6504362), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Dagestan"), ("am", "ዳገስታን"), ("ar", "داغستان"), ("az", "Dağıstan"), ("be", "Дагестан"), ("bg", "Дагестан"), ("bn", "রিপ\u{9be}বলিক অফ ড\u{9be}গেস\u{9cd}ট\u{9be}ন"), ("bs", "Dagestan"), ("ca", "Daguestan"), ("ccp", "𑄓𑄉𑄬𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Dagestan"), ("cs", "Dagestán"), ("da", "Dagestan"), ("de", "Dagestan"), ("el", "Δημοκρατία του Νταγκεστάν"), ("en", "Dagestan"), ("es", "Daguestán"), ("et", "Dagestan"), ("eu", "Dagestan"), ("fa", "داغستان"), ("fi", "Dagestan"), ("fr", "Daghestan"), ("ga", "An Dagastáin"), ("gu", "રિપબ\u{acd}લિક ઓફ ડ\u{ac5}ગ\u{ac7}સ\u{acd}ટન"), ("he", "דאגסטן"), ("hi", "दाग\u{93c}िस\u{94d}तान"), ("hr", "Dagestan"), ("hu", "Dagesztán"), ("hy", "Դաղստան"), ("id", "Dagestan"), ("is", "Dagestan"), ("it", "Daghestan"), ("ja", "ダゲスタン共和国"), ("jv", "Dagestan"), ("ka", "დაღესტანი"), ("kk", "Дағыстан"), ("kn", "ಡಾಗ\u{cc6}ಸ\u{ccd}ತಾನ\u{ccd}ನ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "다게스탄 공화국"), ("ky", "Дагстан"), ("lt", "Dagestanas"), ("lv", "Dagestāna"), ("mk", "Дагестан"), ("ml", "ദ\u{d3e}ഗസ\u{d4d}ത\u{d3e}ൻ"), ("mn", "Дагестан"), ("mr", "दागिस\u{94d}तान प\u{94d}रजासत\u{94d}ताक"), ("ms", "Dagestan"), ("nb", "Dagestan"), ("nl", "Dagestan"), ("no", "Dagestan"), ("pa", "ਦਾਗਿਸਤਾਨ"), ("pl", "Dagestan"), ("pt", "Daguestão"), ("ro", "Daghestan"), ("ru", "Дагестан"), ("si", "ඩගේස\u{dca}ට\u{dcf}න\u{dca} ජනරජය"), ("sk", "Dagestan"), ("sl", "Dagestan"), ("sq", "Dagestani"), ("sr", "Дагестан"), ("sr_Latn", "Dagestan"), ("sv", "Dagestan"), ("sw", "Dagestan"), ("ta", "த\u{bbe}கெஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d} డ\u{c3e}గ\u{c46}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "เม\u{e37}องจวนพ\u{e34}ล"), ("tk", "Dagystan"), ("tr", "Dağıstan"), ("uk", "Дагестан"), ("ur", "داغستان"), ("uz", "Dogʻiston"), ("vi", "Dagestan"), ("yue", "達吉斯坦"), ("yue_Hans", "达吉斯坦"), ("zh", "达吉斯坦共和国")]),
                        unofficial_name_list: ["Dagestan, Respublika"].to_vec(),
                    }
                ),
                (
                    "IN",
                    Subdivision{
                        name: "IN",
                        country_alpha2: Alpha2::RU,
                        code: "IN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.4051698), longitude: Some(44.82029989999999), max_latitude: Some(43.6107959), min_latitude: Some(42.614831), max_longitude: Some(45.1902339), min_longitude: Some(44.4769308)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ingoesjetië"), ("am", "ኢንጉሼቲያ"), ("ar", "إنغوشيتيا"), ("az", "İnquşetiya"), ("be", "Інгушэція"), ("bg", "Ингушетия"), ("bn", "ইঙ\u{9cd}গ\u{9c1}শেতিয\u{9bc}\u{9be}"), ("bs", "Ingušetija"), ("ca", "Ingúixia"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄉\u{1112a}𑄥𑄬𑄑\u{11128}𑄠"), ("cs", "Ingušsko"), ("da", "Ingusjien"), ("de", "Inguschetien"), ("el", "Δημοκρατία της Ινγκουσετίας"), ("en", "Ingushetia"), ("es", "Ingusetia"), ("et", "Inguššia"), ("eu", "Inguxetia"), ("fa", "اینگوشتیا"), ("fi", "Ingušia"), ("fr", "Ingouchie"), ("ga", "An Iongúiséit"), ("gl", "Ingusetia"), ("he", "אינגושטיה"), ("hi", "इन\u{94d}ग\u{941}श\u{947}तिया"), ("hr", "Inguška"), ("hu", "Ingusföld"), ("hy", "Ինգուշեթիա"), ("id", "Ingushetia"), ("it", "Inguscezia"), ("ja", "イングーシ共和国"), ("jv", "Ingushetia"), ("ka", "ინგუშეთი"), ("kk", "Ингушетия"), ("ko", "인구시 공화국"), ("ky", "Ингушстан"), ("lt", "Ingušija"), ("lv", "Ingušija"), ("mk", "Ингушетија"), ("mn", "Ингушет"), ("mr", "इ\u{902}ग\u{941}श\u{947}तिया"), ("ms", "Ingushetia"), ("nb", "Ingusjetia"), ("nl", "Ingoesjetië"), ("no", "Ingusjetia"), ("pl", "Inguszetia"), ("pt", "Inguchétia"), ("ro", "Ingușetia"), ("ru", "Ингушетия"), ("sk", "Ingušsko"), ("sl", "Ingušetija"), ("sr", "Ингушетија"), ("sr_Latn", "Ingušetija"), ("sv", "Ingusjien"), ("sw", "Ingushetia"), ("ta", "இங\u{bcd}குசேத\u{bcd}திய\u{bbe}"), ("tr", "İnguşetya"), ("uk", "Інгушетія"), ("ur", "انگوشتیا"), ("uz", "Ingushiya"), ("vi", "Ingushetiya"), ("yue", "印古什"), ("yue_Hans", "印古什"), ("zh", "印古什共和国")]),
                        unofficial_name_list: ["Ingushetija", "Ingušetija"].to_vec(),
                    }
                ),
                (
                    "IRK",
                    Subdivision{
                        name: "IRK",
                        country_alpha2: Alpha2::RU,
                        code: "IRK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.13214199999999), longitude: Some(103.948625), max_latitude: Some(64.31695599999999), min_latitude: Some(51.1373171), max_longitude: Some(119.1306879), min_longitude: Some(95.65773999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Irkoetsk-oblast"), ("ar", "إركوتسك أوبلاست"), ("az", "İrkutsk vilayəti"), ("be", "Іркуцкая вобласць"), ("bg", "Иркутска област"), ("bn", "ইরক\u{9c1}টস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Irkutska oblast"), ("ca", "Província d’Irkutsk"), ("ccp", "𑄃\u{11128}𑄢\u{11134}𑄎\u{1112a}𑄖\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Irkutskaya Oblast’"), ("cs", "Irkutská oblast"), ("cy", "Oblast Irkutsk"), ("da", "Irkutsk oblast"), ("de", "Oblast Irkutsk"), ("el", "Περιφέρεια Ιρκούτσκ"), ("en", "Irkutsk"), ("es", "Irkutsk"), ("et", "Irkutski oblast"), ("eu", "Irkutsk oblasta"), ("fa", "استان ایرکوتسک"), ("fi", "Irkutskin alue"), ("fr", "Oblast d’Irkoutsk"), ("ga", "Cúige Irkutsk"), ("gu", "ઇર\u{acd}ક\u{acd}ટ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אירקוטסק"), ("hi", "इरक\u{941}त\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("hr", "Irkutska oblast"), ("hu", "Irkutszki terület"), ("hy", "Իրկուտսկի մարզ"), ("id", "Oblast Irkutsk"), ("it", "Oblast’ di Irkutsk"), ("ja", "イルクーツク州"), ("ka", "ირკუტსკის ოლქი"), ("kn", "ಇರ\u{ccd}ಕುಟ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "이르쿠츠크 주"), ("lt", "Irkutsko sritis"), ("lv", "Irkutskas apgabals"), ("mk", "Иркутска област"), ("mn", "Эрхүү муж"), ("mr", "इरक\u{941}त\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Irkutsk"), ("nb", "Irkutsk"), ("nl", "Oblast Irkoetsk"), ("no", "Irkutsk"), ("pl", "Obwód irkucki"), ("pt", "Oblast de Irkutsk"), ("ro", "Regiunea Irkutsk"), ("ru", "Иркутская область"), ("si", "ඉර\u{dca}ක\u{dd4}ට\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Irkutská oblasť"), ("sl", "Irkutska oblast"), ("sr", "Иркутска област"), ("sr_Latn", "Irkutska oblast"), ("sv", "Irkutsk oblast"), ("sw", "Irkutsk Oblast"), ("ta", "இர\u{bcd}கூத\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఇర\u{c4d}కుట\u{c4d}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นอ\u{e35}ร\u{e4c}ค\u{e38}ตสค\u{e4c}"), ("tr", "İrkutsk Oblastı"), ("uk", "Іркутська область"), ("ur", "ارکتسک اوبلاست"), ("uz", "Irkutsk viloyati"), ("vi", "Irkutsk"), ("yue", "伊爾庫茨克州"), ("yue_Hans", "伊尔库茨克州"), ("zh", "伊尔库茨克州")]),
                        unofficial_name_list: ["Irkutskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "IVA",
                    Subdivision{
                        name: "IVA",
                        country_alpha2: Alpha2::RU,
                        code: "IVA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.1056854), longitude: Some(41.4830084), max_latitude: Some(57.74272199999999), min_latitude: Some(56.3503961), max_longitude: Some(43.3058696), min_longitude: Some(39.3779529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Iwanowo-oblast"), ("ar", "إيفانوفو أوبلاست"), ("az", "İvanovo vilayəti"), ("be", "Іванаўская вобласць"), ("bg", "Ивановска област"), ("bn", "ইভ\u{9be}নোভো ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Ivanovska oblast"), ("ca", "Província d’Ivànovo"), ("ccp", "𑄃\u{11128}𑄞𑄚\u{1112e}𑄞\u{1112e}"), ("ceb", "Ivanovskaya Oblast’"), ("cs", "Ivanovská oblast"), ("cy", "Oblast Ivanovo"), ("da", "Ivanovo oblast"), ("de", "Oblast Iwanowo"), ("el", "Περιφέρεια Ιβάνοβο"), ("en", "Ivanovo"), ("es", "Óblast de Ivánovo"), ("et", "Ivanovo oblast"), ("eu", "Ivanovo oblasta"), ("fa", "استان ایوانوف"), ("fi", "Ivanovon alue"), ("fr", "Oblast d’Ivanovo"), ("ga", "Cúige Ivanovo"), ("gu", "ઇવાનહો ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז איבנובו"), ("hi", "इवानोवो ओब\u{94d}लास\u{94d}ट"), ("hr", "Ivanovska oblast"), ("hu", "Ivanovói terület"), ("hy", "Իվանովոյի մարզ"), ("id", "Oblast Ivanovo"), ("it", "Oblast’ di Ivanovo"), ("ja", "イヴァノヴォ州"), ("ka", "ივანოვოს ოლქი"), ("kn", "ಇವಾನೊವೊ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "이바노보 주"), ("lt", "Ivanovo sritis"), ("lv", "Ivanovas apgabals"), ("mk", "Ивановска област"), ("mn", "Иваново муж"), ("mr", "इवानोवो ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Ivanovo"), ("nb", "Ivanovo"), ("nl", "Oblast Ivanovo"), ("no", "Ivanovo"), ("pl", "Obwód iwanowski"), ("pt", "Oblast de Ivanovo"), ("ro", "Regiunea Ivanovo"), ("ru", "Ивановская область"), ("si", "ඉවනොවෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ivanovská oblasť"), ("sl", "Ivanovska oblast"), ("sr", "Ивановска област"), ("sr_Latn", "Ivanovska oblast"), ("sv", "Ivanovo oblast"), ("sw", "Ivanovo Oblast"), ("ta", "இவ\u{bbe}னோவ\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఇవ\u{c3e}న\u{c4b}వ\u{c4b} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นอ\u{e35}วาโนโว"), ("tr", "İvanovo Oblastı"), ("uk", "Івановська область"), ("ur", "ایوانوو اوبلاست"), ("uz", "Ivanovo viloyati"), ("vi", "Ivanovo"), ("yue", "伊凡諾禾州"), ("yue_Hans", "伊凡诺禾州"), ("zh", "伊万诺沃州")]),
                        unofficial_name_list: ["Ivanovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KAM",
                    Subdivision{
                        name: "KAM",
                        country_alpha2: Alpha2::RU,
                        code: "KAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.743583), longitude: Some(37.5880195), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kamtsjatka-krai"), ("ar", "كراي كامشاتكا"), ("az", "Kamçatka diyarı"), ("be", "Камчацкі край"), ("bg", "Камчатски край"), ("bn", "ক\u{9be}মচ\u{9be}টক\u{9be}ক\u{9cd}র\u{9be}ই"), ("bs", "Kamčatski kraj"), ("ca", "Territori de Kamtxatka"), ("ccp", "𑄇𑄟\u{11134}𑄌𑄖\u{11134}𑄇 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Kamtchatski Kray"), ("cs", "Kamčatský kraj"), ("cy", "Crai Kamchatka"), ("da", "Kamtjatka kraj"), ("de", "Region Kamtschatka"), ("el", "Κράι Καμτσάτκα"), ("en", "Kamchatka Krai"), ("es", "Kamchatka"), ("et", "Kamtšatka krai"), ("eu", "Kamtxatka kraia"), ("fa", "سرزمین کامچاتکا"), ("fi", "Kamtšatkan aluepiiri"), ("fr", "Kraï du Kamtchatka"), ("ga", "Críoch Kamchatka"), ("gu", "કામચાટ\u{acd}કા ક\u{acd}રાઇ"), ("he", "מחוז קמצ׳טקה"), ("hi", "कमचातका क\u{94d}राय"), ("hr", "Kamčatski kraj"), ("hu", "Kamcsatkai határterület"), ("id", "Krai Kamchatka"), ("is", "Kamtsjatka skagi"), ("it", "Territorio della Kamčatka"), ("ja", "カムチャツカ地方"), ("ka", "კამჩატკა"), ("kn", "ಕಂಚಟ\u{ccd}ಕಾ ಕ\u{ccd}ರಾಯ\u{cbf}"), ("ko", "캄차카 지방"), ("ky", "Камчатка крайы"), ("lt", "Kamčiatkos kraštas"), ("lv", "Kamčatkas novads"), ("mk", "Камчатски крај"), ("mn", "Камчаткын Хязгаар"), ("mr", "कामचत\u{94d}का क\u{94d}राय"), ("ms", "Jajahan Kamchatka"), ("nb", "Kamtsjatka"), ("nl", "Kraj Kamtsjatka"), ("no", "Kamtsjatka"), ("pl", "Kraj Kamczacki"), ("pt", "Krai de Kamtchatka"), ("ro", "Ținutul Kamceatka"), ("ru", "Камчатский край"), ("si", "කම\u{dca}චත\u{dca}ක\u{dcf} ක\u{dca}\u{200d}ර\u{dcf}ය\u{dd2}"), ("sk", "Kamčatský kraj"), ("sl", "Kamčatski kraj"), ("sq", "Kamçatka Krai"), ("sr", "Камчатска Покрајина"), ("sr_Latn", "Kamčatska Pokrajina"), ("sv", "Kamtjatka kraj"), ("sw", "Kamchatka Krai"), ("ta", "கம\u{bcd}ச\u{bbe}த\u{bcd}க\u{bbe} பிரதேசம\u{bcd}"), ("te", "కమ\u{c4d}చత\u{c4d}క\u{c3e} క\u{c4d}ర\u{c3e}య\u{c4d}"), ("th", "ด\u{e34}นแดนค\u{e31}มช\u{e31}ตคา"), ("tk", "Kamçatka Kraý"), ("tr", "Kamçatka Krayı"), ("uk", "Камчатський край"), ("ur", "کامچاٹکا کرائی"), ("uz", "Kamchatka oʻlkasi"), ("vi", "Kamchatka"), ("yue", "琴察加邊疆區"), ("yue_Hans", "琴察加边疆区"), ("zh", "堪察加邊疆區")]),
                        unofficial_name_list: ["Kamchatskaya Oblast", "Kamčatka", "Kamčatskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KB",
                    Subdivision{
                        name: "KB",
                        country_alpha2: Alpha2::RU,
                        code: "KB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.3932469), longitude: Some(43.5628498), max_latitude: Some(44.0228141), min_latitude: Some(42.8891), max_longitude: Some(44.47055100000001), min_longitude: Some(42.3981201)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kabardino-Balkarië"), ("am", "ካባርዲኖ-ባልካሪያ"), ("ar", "قبردينو - بلقاريا"), ("az", "Kabardin-Balkariya"), ("be", "Кабардзіна-Балкарыя"), ("bg", "Кабардино-Балкария"), ("bn", "ক\u{9be}ব\u{9be}র\u{9cd}ডিনো-বল\u{9cd}ক\u{9be}র রিপ\u{9be}বলিক"), ("bs", "Kabardino-Balkarija"), ("ca", "Kabardino-Balkària"), ("ccp", "𑄇𑄚𑄢\u{11134}𑄓\u{11128}𑄚\u{1112e}-𑄝𑄣\u{11134}𑄇𑄢\u{11134}"), ("ceb", "Kabardino-Balkarskaya Respublika"), ("cs", "Kabardsko-Balkarsko"), ("cy", "Kabardino-Balkaria"), ("da", "Kabardino-Balkarien"), ("de", "Kabardino-Balkarien"), ("el", "Δημοκρατία της Καμπαρντίνο - Μπαλκάρια"), ("en", "Kabardino-Balkar"), ("es", "Kabardia-Balkaria"), ("et", "Kabardi-Balkaaria"), ("eu", "Kabardino-Balkaria"), ("fa", "کاباردینو-بالکاریا"), ("fi", "Kabardi-Balkaria"), ("fr", "Kabardino-Balkarie"), ("ga", "An Chabairdín-Bhalcáir"), ("gl", "Kabardino-Balkaria"), ("gu", "કબાર\u{acd}ડિનો બાલ\u{acd}કર રિપબ\u{acd}લિક"), ("he", "קברדינו-בלקריה"), ("hi", "काबारदीनो-बल\u{94d}कारिया"), ("hr", "Kabardsko-Balkarska"), ("hu", "Kabard- és Balkárföld"), ("hy", "Կաբարդա-Բալկարիա"), ("id", "Kabardino-Balkaria"), ("it", "Cabardino-Balcaria"), ("ja", "カバルダ・バルカル共和国"), ("ka", "ყაბარდო-ბალყარეთი"), ("kk", "Қабарда-Балқария"), ("kn", "ಕಬಾರ\u{ccd}ಡ\u{cbf}ನ-ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "카바르디노발카르 공화국"), ("ky", "Кабарда-Балкария"), ("lt", "Kabarda-Balkarija"), ("lv", "Kabarda-Balkārija"), ("mk", "Кабардино-Балкарија"), ("mn", "Кабардино-Балкар"), ("mr", "काबार\u{94d}डिनो-बाल\u{94d}कारिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Kabardino-Balkaria"), ("nb", "Kabardino-Balkaria"), ("nl", "Kabardië-Balkarië"), ("no", "Kabardino-Balkaria"), ("pl", "Kabardo-Bałkaria"), ("pt", "Cabárdia-Balcária"), ("ro", "Cabardino-Balcaria"), ("ru", "Кабардино-Балкария"), ("si", "කබර\u{dca}ද\u{dd2}නෝ -බල\u{dca}ක\u{dcf}ර\u{dca} ජනරජය"), ("sk", "Kabardsko-Balkarsko"), ("sl", "Kabardino-Balkarija"), ("sr", "Кабардино-Балкарија"), ("sr_Latn", "Kabardino-Balkarija"), ("sv", "Kabardinien-Balkarien"), ("sw", "Kabardino-Balkaria"), ("ta", "கபர\u{bcd}தினோ-பல\u{bcd}கர\u{bc0}ய\u{bbe}"), ("te", "కబ\u{c3e}ర\u{c4d}డ\u{c3f}న\u{c4b}-బ\u{c3e}ల\u{c4d}కర\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐ คาบาด\u{e34}โน-บอลเกอร\u{e4c}"), ("tr", "Kabardino-Balkarya"), ("uk", "Кабардино-Балкарія"), ("ur", "کباردینو-بالکاریا جمہوریہ"), ("uz", "Kabarda-balkariya"), ("vi", "Kabardino-Balkaria"), ("yue", "卡巴爾達-巴爾卡爾"), ("yue_Hans", "卡巴尔达-巴尔卡尔"), ("zh", "卡巴爾達-巴爾卡爾共和國")]),
                        unofficial_name_list: ["Kabardino-Balkarian Republic", "Kabardino-Balkarija", "Kabardino-Balkarskaja Respublika"].to_vec(),
                    }
                ),
                (
                    "KC",
                    Subdivision{
                        name: "KC",
                        country_alpha2: Alpha2::RU,
                        code: "KC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8845143), longitude: Some(41.730394), max_latitude: Some(44.49697889999999), min_latitude: Some(43.187437), max_longitude: Some(42.6780321), min_longitude: Some(40.6833698)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Karatsjai-Tsjerkessië"), ("am", "ካራቻይ-ቸርከሢያ"), ("ar", "قراتشاي - تشيركيسيا"), ("az", "Qaraçay-Çerkesiya"), ("be", "Карачаева-Чаркесія"), ("bg", "Карачаево-Черкезия"), ("bn", "ক\u{9be}র\u{9be}ক\u{9be}য\u{9bc} চেরকিস রিপ\u{9be}বলিক"), ("bs", "Karačajevo-Čerkezija"), ("ca", "Karatxai-Txerkèssia"), ("ccp", "𑄇𑄢𑄌𑄬-𑄇𑄢\u{11134}𑄇𑄬𑄌\u{11134}"), ("ceb", "Karachayevo-Cherkesiya"), ("cs", "Karačajevsko-Čerkesko"), ("da", "Karatjajevo-Tjerkessien"), ("de", "Karatschai-Tscherkessien"), ("el", "Δημοκρατία των Καρατσάι - Τσερκεσίων"), ("en", "Karachay-Cherkess"), ("es", "Karacháyevo-Cherkesia"), ("et", "Karatšai-Tšerkessia"), ("eu", "Karatxai-Txerkesia"), ("fa", "قره\u{200c}چای و چرکس"), ("fi", "Karatšai-Tšerkessia"), ("fr", "Karatchaïévo-Tcherkessie"), ("ga", "An Charaitsé-Shiorcáis"), ("gl", "Karachay-Cherkessia"), ("gu", "કરાચાય-ચ\u{ac7}ર\u{acd}ક\u{ac7}સ રિપબ\u{acd}લિક"), ("he", "קאראצ׳אי-צ׳רקסיה"), ("hi", "काराचाए-चरकस\u{94d}सिया"), ("hr", "Karačajevsko-Čerkeska"), ("hu", "Karacsáj- és Cserkeszföld"), ("hy", "Կարաչայ-Չերքեզիա"), ("id", "Karachay-Cherkessia"), ("it", "Karačaj-Circassia"), ("ja", "カラチャイ・チェルケス共和国"), ("ka", "ყარაჩაი-ჩერქეზეთი"), ("kk", "Қарашай-Черкесия"), ("kn", "ಚಾರ\u{ccd}ಕ\u{cc6}ಸ\u{ccd}-ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "카라차예보체르케스카야 공화국"), ("ky", "Карачай-Черкисия"), ("lt", "Karačiajų Čerkesija"), ("lv", "Karačaja-Čerkesija"), ("mk", "Карачаево-Черкесија"), ("mn", "Карачай-Черкесс"), ("mr", "काराचाय-च\u{947}र\u{94d}क\u{947}स प\u{94d}रजासत\u{94d}ताक"), ("ms", "Karachay-Cherkessia"), ("nb", "Karatsjajevo-Tsjerkessia"), ("nl", "Karatsjaj-Tsjerkessië"), ("no", "Karatsjajevo-Tsjerkessia"), ("pl", "Karaczajo-Czerkiesja"), ("pt", "Carachai-Circássia"), ("ro", "Karaciai-Cerchezia"), ("ru", "Карачаево-Черкесия"), ("si", "කරචේ-චෙර\u{dca}කෙස\u{dca} ජනරජය"), ("sk", "Karačajsko-Čerkesko"), ("sl", "Karačaj-Čerkezija"), ("sr", "Карачајево-Черкезија"), ("sr_Latn", "Karačajevo-Čerkezija"), ("sv", "Karatjajen-Tjerkessien"), ("sw", "Karachaevo-Cherkesia"), ("ta", "க\u{bbe}ர\u{bbe}ச\u{bbe}ய\u{bcd}-செர\u{bcd}கெஸ\u{bcd}ஸிய\u{bbe}"), ("te", "కర\u{c3e}చ\u{c47}-చ\u{c46}ర\u{c4d}క\u{c46}స\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "การาชาย-ชาร\u{e4c}คเคส"), ("tr", "Karaçay-Çerkesya"), ("uk", "Карачаєво-Черкесія"), ("ur", "کراچائے-چرکیسیا"), ("uz", "Qorachoy-Cherkasiya"), ("vi", "Karachay-Cherkessia"), ("yue", "卡拉恰伊-切爾克斯共和國"), ("yue_Hans", "卡拉恰伊-切尔克斯共和国"), ("zh", "卡拉恰伊-切尔克斯共和国")]),
                        unofficial_name_list: ["Karachay-Cherkessian", "Karačaj-Čerkessija", "Karačajevo-Čerkesskaja Respublika"].to_vec(),
                    }
                ),
                (
                    "KDA",
                    Subdivision{
                        name: "KDA",
                        country_alpha2: Alpha2::RU,
                        code: "KDA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6415289), longitude: Some(39.7055977), max_latitude: Some(46.8802379), min_latitude: Some(43.3848639), max_longitude: Some(41.7476441), min_longitude: Some(36.5980539)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Krasnodar-krai"), ("ar", "كراسنودار كراي"), ("az", "Krasnodar diyarı"), ("be", "Краснадарскі край"), ("bg", "Краснодарски край"), ("bn", "ক\u{9be}রস\u{9cd}ন\u{9c1}দ\u{9be}র ক\u{9be}রি"), ("bs", "Krasnodarski kraj"), ("ca", "Territori de Krasnodar"), ("ccp", "𑄇\u{11133}𑄢𑄌\u{11134}𑄚\u{1112e}𑄓𑄢\u{11134} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Krasnodarskiy Kray"), ("cs", "Krasnodarský kraj"), ("cy", "Crai Krasnodar"), ("da", "Krasnodar kraj"), ("de", "Region Krasnodar"), ("el", "Κράι Κρασνοντάρ"), ("en", "Krasnodar Krai"), ("es", "Krai de Krasnodar"), ("et", "Krasnodari krai"), ("eu", "Krasnodar kraia"), ("fa", "سرزمین کراسنودار"), ("fi", "Krasnodarin aluepiiri"), ("fr", "Kraï de Krasnodar"), ("ga", "Críoch Krasnodar"), ("gl", "Krai de Krasnodar"), ("gu", "ક\u{acd}રાન\u{acd}સ\u{acd}નોદર ક\u{acd}રાઇ"), ("he", "מחוז קרסנודאר"), ("hi", "क\u{94d}रास\u{94d}नोदार क\u{94d}राय"), ("hr", "Krasnodarski kraj"), ("hu", "Krasznodari határterület"), ("hy", "Կրասնոդարի երկրամաս"), ("id", "Krai Krasnodar"), ("is", "Krasnodarfylki"), ("it", "Territorio di Krasnodar"), ("ja", "クラスノダール地方"), ("ka", "კრასნოდარის მხარე"), ("kn", "ಕ\u{ccd}ರಾಸ\u{ccd}ನೋಡರ\u{ccd} ಕ\u{ccd}ರೈ"), ("ko", "크라스노다르 지방"), ("ky", "Краснодар крайы"), ("lt", "Krasnodaro kraštas"), ("lv", "Krasnodaras novads"), ("mk", "Краснодарски крај"), ("mn", "Краснодарын хязгаар"), ("mr", "क\u{94d}रास\u{94d}नोदर क\u{94d}राय"), ("ms", "Jajahan Krasnodar"), ("nb", "Krasnodar"), ("nl", "Kraj Krasnodar"), ("no", "Krasnodar"), ("pl", "Kraj Krasnodarski"), ("pt", "Krai de Krasnodar"), ("ro", "Ținutul Krasnodar"), ("ru", "Краснодарский край"), ("si", "ක\u{dca}\u{200d}රස\u{dca}නොද\u{dcf}ර\u{dca} ක\u{dca}\u{200d}රය\u{dd2}"), ("sk", "Krasnodarský kraj"), ("sl", "Krasnodarski kraj"), ("sr", "Краснодарска Покрајина"), ("sr_Latn", "Krasnodarska Pokrajina"), ("sv", "Krasnodar kraj"), ("sw", "Krasnodar Krai"), ("ta", "கிர\u{bbe}ஸ\u{bcd}னத\u{bbe}ர\u{bcd} பிரதேசம\u{bcd}"), ("te", "క\u{c4d}ర\u{c3e}స\u{c4d}న\u{c4b}డర\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "คราสโนดา ไคร"), ("tr", "Krasnodar Krayı"), ("uk", "Краснодарський край"), ("ur", "کریسنوڈار کرائی"), ("uz", "Krasnodar oʻlkasi"), ("vi", "Krasnodar"), ("yue", "克拉斯諾達爾邊疆區"), ("yue_Hans", "克拉斯诺达尔边疆区"), ("zh", "克拉斯诺达尔边疆区")]),
                        unofficial_name_list: ["Krasnodarskij Kraj"].to_vec(),
                    }
                ),
                (
                    "KEM",
                    Subdivision{
                        name: "KEM",
                        country_alpha2: Alpha2::RU,
                        code: "KEM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.7574648), longitude: Some(87.4055288), max_latitude: Some(56.83512), min_latitude: Some(52.1605449), max_longitude: Some(89.399602), min_longitude: Some(84.450098)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kemerowo-oblast"), ("ar", "كيميروفو أوبلاست"), ("az", "Kemerovo vilayəti"), ("be", "Кемераўская вобласць"), ("bg", "Кемеровска област"), ("bn", "কেমেরোভো ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kemerovska oblast"), ("ca", "Província de Kèmerovo"), ("ccp", "𑄇𑄬𑄟𑄬𑄢\u{1112e}𑄞\u{1112e}"), ("ceb", "Kemerovskaya Oblast’"), ("cs", "Kemerovská oblast"), ("cy", "Oblast Kemerovo"), ("da", "Kemerovo oblast"), ("de", "Oblast Kemerowo"), ("el", "Περιφέρεια Κεμέροβο"), ("en", "Kemerovo"), ("es", "Kémerovo"), ("et", "Kemerovo oblast"), ("eu", "Kemerovo oblasta"), ("fa", "استان کمروو"), ("fi", "Kemerovon alue"), ("fr", "oblast de Kemerovo"), ("ga", "Cúige Kemerovo"), ("gu", "ક\u{ac7}મ\u{ac7}રોવો ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קמרובו"), ("hi", "क\u{947}म\u{947}रोवो ओब\u{94d}लास\u{94d}ट"), ("hr", "Kemerovska oblast"), ("hu", "Kemerovói terület"), ("hy", "Կեմերովոյի մարզ"), ("id", "Oblast Kemerovo"), ("it", "Oblast’ di Kemerovo"), ("ja", "ケメロヴォ州"), ("ka", "კემეროვოს ოლქი"), ("kn", "ಕ\u{cc6}ಮ\u{cc6}ರೊ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "케메로보 주"), ("lt", "Kemerovo sritis"), ("lv", "Kemerovas apgabals"), ("mk", "Кемеровска област"), ("mn", "Кемерово муж"), ("mr", "क\u{947}म\u{947}रोवो ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kemerovo"), ("nb", "Kemerovo"), ("nl", "Oblast Kemerovo"), ("no", "Kemerovo"), ("pl", "Obwód kemerowski"), ("pt", "Oblast de Kemerovo"), ("ro", "Regiunea Kemerovo"), ("ru", "Кемеровская область"), ("si", "කෙමෙරොවෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kemerovská oblasť"), ("sl", "Kemerovska oblast"), ("sr", "Кемеровска област"), ("sr_Latn", "Kemerovska oblast"), ("sv", "Kemerovo oblast"), ("sw", "Kemerovo Oblast"), ("ta", "கெமரோவோ ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "క\u{c46}మ\u{c46}ర\u{c4b}వ\u{c4b} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลเคเมโรโว"), ("tr", "Kemerovo Oblastı"), ("uk", "Кемеровська область"), ("ur", "کیمیروو اوبلاست"), ("uz", "Kemerovo viloyati"), ("vi", "Kemerovo"), ("yue", "克麥羅禾州"), ("yue_Hans", "克麦罗禾州"), ("zh", "科麦罗沃州")]),
                        unofficial_name_list: ["Kemerovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KGD",
                    Subdivision{
                        name: "KGD",
                        country_alpha2: Alpha2::RU,
                        code: "KGD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.8235292), longitude: Some(21.4816163), max_latitude: Some(55.2944458), min_latitude: Some(54.318845), max_longitude: Some(22.886888), min_longitude: Some(19.6388525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kaliningrad-oblast"), ("am", "ካሊኒንግራድ ኦብላስት"), ("ar", "أوبلاست كالينينغرادسكايا"), ("az", "Kalininqrad vilayəti"), ("be", "Калінінградская вобласць"), ("bg", "Калининградска област"), ("bn", "ক\u{9be}লিনিনগ\u{9cd}র\u{9be}দ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kalinjingradska oblast"), ("ca", "Província de Kaliningrad"), ("ccp", "𑄇𑄣\u{11128}𑄚\u{11128}𑄚\u{11134}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Kaliningradskaya Oblast’"), ("cs", "Kaliningradská oblast"), ("cy", "Oblast Kaliningrad"), ("da", "Kaliningrad oblast"), ("de", "Oblast Kaliningrad"), ("el", "Περιφέρεια Καλίνινγκραντ"), ("en", "Kaliningrad"), ("es", "Óblast de Kaliningrado"), ("et", "Kaliningradi oblast"), ("eu", "Kaliningrad oblasta"), ("fa", "استان کالینینگراد"), ("fi", "Kaliningradin alue"), ("fr", "Oblast de Kaliningrad"), ("ga", "Cúige Kaliningrad"), ("gl", "Óblast de Kaliningrado"), ("gu", "ક\u{ac7}લિનિનગ\u{acd}ર\u{ac7}ડ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קלינינגרד"), ("hi", "कलिनिनग\u{94d}र\u{948}ड ओब\u{94d}लास\u{94d}ट"), ("hr", "Kalinjingradska oblast"), ("hu", "Kalinyingrádi terület"), ("hy", "Կալինինգրադի մարզ"), ("id", "Oblast Kaliningrad"), ("it", "oblast’ di Kaliningrad"), ("ja", "カリーニングラード州"), ("ka", "კალინინგრადის ოლქი"), ("kn", "ಕಲ\u{cbf}ನ\u{cbf}ನ\u{ccd}ಗ\u{ccd}ರಾಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "칼리닌그라드 주"), ("lt", "Karaliaučiaus sritis"), ("lv", "Kaļiņingradas apgabals"), ("mk", "Калининградска област"), ("ml", "കലീന\u{d4d}യിൻഗ\u{d4d}ര\u{d3e}ഡ\u{d4d} ഒബ\u{d4d}ല\u{d3e}സ\u{d4d}റ\u{d4d}റ\u{d4d}"), ("mn", "Калининград муж"), ("mr", "कालिनिनग\u{94d}राद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kaliningrad"), ("nb", "Kaliningrad"), ("nl", "Oblast Kaliningrad"), ("no", "Kaliningrad"), ("pl", "Obwód kaliningradzki"), ("pt", "Oblast de Kaliningrado"), ("ro", "Regiunea Kaliningrad"), ("ru", "Калининградская область"), ("si", "ක\u{dcf}ල\u{dd2}න\u{dd2}න\u{dd2}ග\u{dca}රඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kaliningradská oblasť"), ("sl", "Kaliningrajska oblast"), ("sr", "Калињинградска област"), ("sr_Latn", "Kalinjingradska oblast"), ("sv", "Kaliningrad oblast"), ("sw", "Kaliningrad Oblast"), ("ta", "கலினின\u{bcd}கிர\u{bbe}ட\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "క\u{c3e}ల\u{c3f}న\u{c3f}ంగ\u{c4d}ర\u{c3e}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "คาล\u{e34}น\u{e34}นกราดโอบลาสต\u{e4c}"), ("tr", "Kaliningrad Oblastı"), ("uk", "Калінінградська область"), ("ur", "کیلننگراڈ اوبلاست"), ("uz", "Kaliningrad viloyati"), ("vi", "Kaliningrad"), ("yo", "Kaliningrad Oblast"), ("yo_BJ", "Kaliningrad Oblast"), ("yue", "加里寧格勒州"), ("yue_Hans", "加里宁格勒州"), ("zh", "加里宁格勒州")]),
                        unofficial_name_list: ["Kaliningradskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KGN",
                    Subdivision{
                        name: "KGN",
                        country_alpha2: Alpha2::RU,
                        code: "KGN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.4481548), longitude: Some(65.11809749999999), max_latitude: Some(56.8420819), min_latitude: Some(54.1874291), max_longitude: Some(68.7217089), min_longitude: Some(61.9661031)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Koergan-oblast"), ("ar", "أوبلاست كورغان"), ("az", "Kurqan vilayəti"), ("be", "Курганская вобласць"), ("bg", "Курганска област"), ("bn", "ক\u{9c1}রগ\u{9cd}য\u{9be}ন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kurganska oblast"), ("ca", "Província de Kurgan"), ("ccp", "𑄇\u{1112a}𑄢\u{11134}𑄉\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Kurganskaya Oblast’"), ("cs", "Kurganská oblast"), ("cy", "Oblast Kurgan"), ("da", "Kurgan oblast"), ("de", "Oblast Kurgan"), ("el", "Περιφέρεια Κουργκάν"), ("en", "Kurgan"), ("es", "Kurgán"), ("et", "Kurgani oblast"), ("eu", "Kurgan oblasta"), ("fa", "استان کورگان"), ("fi", "Kurganin alue"), ("fr", "Oblast de Kourgan"), ("ga", "Cúige Kurgan"), ("gu", "ક\u{ac1}ર\u{acd}ગન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קורגן"), ("hi", "क\u{941}र\u{94d}गन ओब\u{94d}लास\u{94d}ट"), ("hr", "Kurganska oblast"), ("hu", "Kurgani terület"), ("hy", "Կուրգանի մարզ"), ("id", "Oblast Kurgan"), ("it", "Oblast’ di Kurgan"), ("ja", "クルガン州"), ("ka", "კურგანის ოლქი"), ("kk", "Қорған облысы"), ("kn", "ಕುರ\u{ccd}ಗನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "쿠르간 주"), ("lt", "Kurgano sritis"), ("lv", "Kurgānas apgabals"), ("mk", "Курганска област"), ("mn", "Курган муж"), ("mr", "क\u{941}र\u{94d}गान ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kurgan"), ("nb", "Kurgan"), ("nl", "Oblast Koergan"), ("no", "Kurgan"), ("pl", "Obwód kurgański"), ("pt", "Oblast de Kurgan"), ("ro", "Regiunea Kurgan"), ("ru", "Курганская область"), ("si", "කර\u{dca}ගන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kurganská oblasť"), ("sl", "Kurganska oblast"), ("sr", "Курганска област"), ("sr_Latn", "Kurganska oblast"), ("sv", "Kurgan oblast"), ("sw", "Kurgan Oblast"), ("ta", "குர\u{bcd}கன\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "కుర\u{c4d}గన\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ค\u{e39}แกน อบลาส"), ("tr", "Kurgan Oblastı"), ("uk", "Курганська область"), ("ur", "کورگان اوبلاست"), ("uz", "Koʻrgan viloyati"), ("vi", "Kurgan"), ("yue", "庫爾幹州"), ("yue_Hans", "库尔干州"), ("zh", "库尔干州")]),
                        unofficial_name_list: ["Kurganskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KHA",
                    Subdivision{
                        name: "KHA",
                        country_alpha2: Alpha2::RU,
                        code: "KHA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.5888431), longitude: Some(135.0), max_latitude: Some(62.5246119), min_latitude: Some(46.6347189), max_longitude: Some(147.2038518), min_longitude: Some(130.3886411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chabarofsk-krai"), ("ar", "خاباروفسك كراي"), ("az", "Xabarovsk diyarı"), ("be", "Хабараўскі край"), ("bg", "Хабаровски край"), ("bn", "খ\u{9be}ব\u{9be}রোভস\u{9cd}ক ক\u{9cd}র\u{9be}ই"), ("bs", "Habarovski kraj"), ("ca", "Territori de Khabàrovsk"), ("ccp", "𑄈𑄝𑄢\u{1112e}𑄛\u{11134}𑄥\u{11134}𑄇 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Khabarovskiy Kray"), ("cs", "Chabarovský kraj"), ("cy", "Crai Khabarovsk"), ("da", "Khabarovsk kraj"), ("de", "Region Chabarowsk"), ("el", "Κράι Χαμπάροφσκ"), ("en", "Khabarovsk Krai"), ("es", "Jabárovsk"), ("et", "Habarovski krai"), ("eu", "Khabarovsk kraia"), ("fa", "سرزمین خاباروفسک"), ("fi", "Habarovskin aluepiiri"), ("fr", "Kraï de Khabarovsk"), ("ga", "Críoch Khabarovsk"), ("gl", "Krai de Khabarovsk"), ("gu", "ખાબરોવ\u{acd}સ\u{acd}ક ક\u{acd}રાઇ"), ("he", "מחוז חברובסק"), ("hi", "ख\u{93c}ाबारोव\u{94d}स\u{94d}क क\u{94d}राय"), ("hr", "Habarovski kraj"), ("hu", "Habarovszki határterület"), ("hy", "Խաբարովսկի երկրամաս"), ("id", "Krai Khabarovsk"), ("it", "Territorio di Chabarovsk"), ("ja", "ハバロフスク地方"), ("ka", "ხაბაროვსკის მხარე"), ("kn", "ಖಬರೋವ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಕ\u{ccd}ರಾಯ\u{cbf}"), ("ko", "하바롭스크 지방"), ("ky", "Хабаров крайы"), ("lt", "Chabarovsko kraštas"), ("lv", "Habarovskas novads"), ("mk", "Хабаровски крај"), ("mn", "Хабаровскийн хязгаар"), ("mr", "खबारोव\u{94d}स\u{94d}क क\u{94d}राय"), ("ms", "Jajahan Khabarovsk"), ("nb", "Khabarovsk"), ("ne", "खाबारोभ\u{94d}स\u{94d}क क\u{94d}राइ"), ("nl", "Kraj Chabarovsk"), ("no", "Khabarovsk"), ("pl", "Kraj Chabarowski"), ("pt", "Krai de Khabarovsk"), ("ro", "Ținutul Habarovsk"), ("ru", "Хабаровский край"), ("si", "කම\u{dca}බ\u{dcf}රොව\u{dca}ස\u{dca}ක\u{dca} ක\u{dca}\u{200d}ර\u{dcf}ය\u{dd2}"), ("sk", "Chabarovský kraj"), ("sr", "Хабаровска Покрајина"), ("sr_Latn", "Habarovska Pokrajina"), ("sv", "Chabarovsk kraj"), ("sw", "Khabarovsk Krai"), ("ta", "கபரோவ\u{bcd}சுக\u{bcd} பிரதேசம\u{bcd}"), ("te", "ఖబ\u{c3e}ర\u{c4b}వస\u{c4d}క\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "เขตบาซาราเบอ\u{e31}สกา"), ("tr", "Habarovsk Krayı"), ("uk", "Хабаровський край"), ("ur", "خابارووسک کرائی"), ("uz", "Xabarovsk oʻlkasi"), ("vi", "Khabarovsk"), ("yue", "哈巴羅夫斯克邊疆區"), ("yue_Hans", "哈巴罗夫斯克边疆区"), ("zh", "哈巴罗夫斯克边疆区")]),
                        unofficial_name_list: ["Habarovsk", "Habarovskij Kray"].to_vec(),
                    }
                ),
                (
                    "KHM",
                    Subdivision{
                        name: "KHM",
                        country_alpha2: Alpha2::RU,
                        code: "KHM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(62.2287062), longitude: Some(70.6410058), max_latitude: Some(65.71070100000001), min_latitude: Some(58.5764188), max_longitude: Some(85.9728444), min_longitude: Some(59.19747880000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chanto-Mansië"), ("ar", "أوكروغ خانتي-مانسي ذاتية الحكم"), ("az", "Xantı-Mansi Muxtar Dairəsi"), ("be", "Ханты-Мансійская аўтаномная акруга — Югра"), ("bg", "Ханти-Мансийски автономен окръг - Югра"), ("bn", "ক\u{9be}ন\u{9cd}তি-ম\u{9be}ন\u{9cd}সি স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Hantijsko-Mansijski autonomni okrug"), ("ca", "Khàntia-Mànsia"), ("ccp", "𑄈𑄚\u{11134}𑄑\u{11128}-𑄟𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Khanty-Mansiyskiy Avtonomnyy Okrug-Yugra"), ("cs", "Chantymansijský autonomní okruh - Jugra"), ("cy", "Ocrwg Ymreolaethol Khanty-Mansi"), ("da", "Hanti-Mansi autonome okrug"), ("de", "Autonomer Kreis der Chanten und Mansen/Jugra"), ("el", "Αυτόνομος θύλακας της Χαντίας - Μανσίας"), ("en", "Khanty-Mansi"), ("es", "Janti-Mansi"), ("et", "Handi-Mansimaa"), ("eu", "Khanti-Mansi"), ("fa", "خانتی-مانسی"), ("fi", "Hanti-Mansia"), ("fr", "Khantys-Mansis"), ("ga", "Ceantar Féinrialaitheach na gCántach is na Mainseach"), ("gu", "ખ\u{a82}તી-માનસી ઓટોનોમસ ઓર\u{acd}કગ"), ("he", "המחוז האוטונומי חנטי ומנסי"), ("hi", "खा\u{902}ति-मानसी स\u{94d}वायत\u{94d}त ऑक\u{94d}रग"), ("hr", "Hantijsko-Mansijski autonomni okrug — Jugra"), ("hu", "Hanti- és Manysiföld"), ("hy", "Խանտի-Մանսիական Ինքնավար Շրջան"), ("id", "Khantia-Mansia"), ("it", "Circondario autonomo degli Chanty-Mansi-Jugra"), ("ja", "ハンティ・マンシ自治管区・ユグラ"), ("ka", "ხანტი-მანსის ავტონომიური ოკრუგი"), ("kn", "ಖಂತ\u{cbf}-ಮನ\u{ccd}ಸ\u{cbf} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಒಕ\u{ccd}ರುಗ\u{ccd}"), ("ko", "한티만시 자치구"), ("ky", "Ханты-Манси автономия округу"), ("lt", "Chantų Mansija"), ("lv", "Hantu-mansu autonomais apvidus-Jugra"), ("mk", "Ханти-Мансиски автономен округ"), ("mr", "खान\u{94d}ती-मान\u{94d}सी स\u{94d}वायत\u{94d}त ऑक\u{94d}र\u{942}ग"), ("ms", "Negeri autonomi Khanty-Mansi"), ("nb", "Khanty-Mansia"), ("nl", "Chanto-Mansië"), ("no", "Khanty-Mansia"), ("pl", "Chanty-Mansyjski Okręg Autonomiczny - Jugra"), ("pt", "Khantia-Mansia"), ("ro", "Districtul autonom Hantî-Mansi"), ("ru", "Ханты-Мансийский автономный округ — Югра"), ("si", "ඛන\u{dca}ට\u{dd2}-මන\u{dca}ස\u{dd2} ස\u{dca}ව\u{dcf}ධ\u{dd2}න කල\u{dcf}පය"), ("sk", "Chantyjsko-Mansijsko"), ("sr", "Хантија-Мансија"), ("sr_Latn", "Hantija-Mansija"), ("sv", "Chantien-Mansien"), ("sw", "Mkoa Huru wa Hanty-Mansi"), ("ta", "க\u{bbe}ன\u{bcd}டி-ம\u{bbe}ன\u{bcd}ஸி தன\u{bcd}ன\u{bbe}ட\u{bcd}சி வட\u{bcd}ட\u{bbe}ரம\u{bcd}"), ("te", "ఖ\u{c3e}ంట\u{c40}-మ\u{c3e}న\u{c4d}స\u{c3f} అట\u{c3e}నమస\u{c4d} ఓక\u{c4d}రుగ\u{c4d}"), ("th", "ฮ\u{e31}นต\u{e35}-ม\u{e31}นซ\u{e35}"), ("tr", "Hantı-Mansi Özerk Okrugu"), ("uk", "Ханти-Мансійський автономний округ — Югра"), ("ur", "خانتی-مانسی خود مختار آکرگ"), ("uz", "Xanti-Mansilar muxtor okrugi"), ("vi", "Khantia-Mansia"), ("yue", "漢特-曼西"), ("yue_Hans", "汉特-曼西"), ("zh", "汉特-曼西自治区")]),
                        unofficial_name_list: ["Hanty-Mansija", "Hanty-Mansijskij Avtonomnyj Okrug"].to_vec(),
                    }
                ),
                (
                    "KIR",
                    Subdivision{
                        name: "KIR",
                        country_alpha2: Alpha2::RU,
                        code: "KIR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.4198529), longitude: Some(50.2097248), max_latitude: Some(61.0629159), min_latitude: Some(56.061146), max_longitude: Some(53.9431012), min_longitude: Some(46.2618118)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kirof-oblast"), ("ar", "أوبلاست كيروف"), ("az", "Kirov vilayəti"), ("be", "Кіраўская вобласць"), ("bg", "Кировска област"), ("bn", "কিরোভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kirovska oblast"), ("ca", "Província de Kírov"), ("ccp", "𑄇\u{11128}𑄢\u{1112e}𑄛\u{11134}"), ("ceb", "Kirovskaya Oblast’"), ("cs", "Kirovská oblast"), ("cy", "Oblast Kirov"), ("da", "Kirov oblast"), ("de", "Oblast Kirow"), ("el", "Περιφέρεια Κίροβ"), ("en", "Kirov"), ("es", "Kírov"), ("et", "Kirovi oblast"), ("eu", "Kirov oblasta"), ("fa", "استان کیروف"), ("fi", "Kirovin alue"), ("fr", "Oblast de Kirov"), ("ga", "Cúige Kirov"), ("gu", "કિરોવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קירוב"), ("hi", "किरोव ओब\u{94d}लास\u{94d}ट"), ("hr", "Kirovska oblast"), ("hu", "Kirovi terület"), ("hy", "Կիրովի մարզ"), ("id", "Oblast Kirov"), ("it", "Oblast’ di Kirov"), ("ja", "キーロフ州"), ("ka", "კიროვის ოლქი"), ("kn", "ಕ\u{cbf}ರೊವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "키로프 주"), ("lt", "Kirovo sritis"), ("lv", "Kirovas apgabals"), ("mk", "Кировска област"), ("mn", "Киров муж"), ("mr", "किरोव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kirov"), ("nb", "Kirov"), ("nl", "Oblast Kirov"), ("no", "Kirov"), ("pl", "Obwód kirowski"), ("pt", "Oblast de Kirov"), ("ro", "Regiunea Kirov"), ("ru", "Кировская область"), ("si", "ක\u{dd2}රෝව\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}කය"), ("sk", "Kirovská oblasť"), ("sl", "Kirovska oblast"), ("sr", "Кировска област"), ("sr_Latn", "Kirovska oblast"), ("sv", "Kirov oblast"), ("sw", "Kirov Oblast"), ("ta", "க\u{bc0}ரோவ\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "క\u{c3f}ర\u{c4b}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นปกครองค\u{e34}โรฟ"), ("tr", "Kirov Oblastı"), ("uk", "Кіровська область"), ("ur", "کیروف اوبلاست"), ("uz", "Kirov viloyati"), ("vi", "Kirov"), ("yue", "基洛夫州"), ("yue_Hans", "基洛夫州"), ("zh", "基洛夫州")]),
                        unofficial_name_list: ["Kirovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KK",
                    Subdivision{
                        name: "KK",
                        country_alpha2: Alpha2::RU,
                        code: "KK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.0452281), longitude: Some(90.3982145), max_latitude: Some(55.43188689999999), min_latitude: Some(51.28421179999999), max_longitude: Some(91.9249129), min_longitude: Some(87.8758369)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chakassië"), ("ar", "خقاسيا"), ("az", "Xakasiya"), ("be", "Хакасія"), ("bg", "Хакасия"), ("bn", "রিপ\u{9be}\u{9be}বলিক অফ খ\u{9be}কসিয\u{9bc}\u{9be}"), ("bs", "Hakasija"), ("ca", "Khakàssia"), ("ccp", "𑄈𑄇𑄥\u{11128}𑄠"), ("ceb", "Respublika Khakasiya"), ("cs", "Chakasie"), ("da", "Khakasien"), ("de", "Chakassien"), ("el", "Δημοκρατία της Χακασίας"), ("en", "Khakassia"), ("es", "Jakasia"), ("et", "Hakassia"), ("eu", "Khakasia"), ("fa", "خاکاسیا"), ("fi", "Hakassia"), ("fr", "Khakassie"), ("ga", "An Chacáis"), ("gl", "Khakassia"), ("gu", "ખાકાસિયા રિપબ\u{acd}લિક"), ("he", "חקסיה"), ("hi", "ख\u{93c}कासिया"), ("hr", "Hakasija"), ("hu", "Hakaszföld"), ("hy", "Խակասիա"), ("id", "Khakassia"), ("it", "Chakassia"), ("ja", "ハカス共和国"), ("ka", "ხაკასეთი"), ("kk", "Хақас Республикасы"), ("kn", "ಖಕಾಸ\u{ccd}ಸ\u{cbf}ಯಾ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "하카스 공화국"), ("ky", "Хакасия"), ("lt", "Chakasija"), ("lv", "Hakasija"), ("mk", "Хакасија"), ("mn", "Хакас"), ("mr", "खाकाशिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Khakassia"), ("nb", "Khakasia"), ("nl", "Chakassië"), ("no", "Khakasia"), ("pl", "Chakasja"), ("pt", "Cacássia"), ("ro", "Hacasia"), ("ru", "Хакасия"), ("si", "ඛකස\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Chakasko"), ("sl", "Hakasija"), ("sr", "Хакасија"), ("sr_Latn", "Hakasija"), ("sv", "Chakassien"), ("sw", "Khakasia"), ("ta", "அக\u{bcd}க\u{bbe}சிய\u{bbe}"), ("te", "ఖక\u{c3e}స\u{c3f}య\u{c3e} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "เม\u{e37}องคาคาซเซ\u{e35}ย"), ("tr", "Hakasya"), ("uk", "Хакасія"), ("ur", "خاکاسیا"), ("uz", "Xakasiya"), ("vi", "Khakassia"), ("yue", "哈卡斯"), ("yue_Hans", "哈卡斯"), ("zh", "哈卡斯共和国")]),
                        unofficial_name_list: ["Hakasija", "Khakass Republic"].to_vec(),
                    }
                ),
                (
                    "KL",
                    Subdivision{
                        name: "KL",
                        country_alpha2: Alpha2::RU,
                        code: "KL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5676845), longitude: Some(45.7731614), max_latitude: Some(48.2743179), min_latitude: Some(44.7639349), max_longitude: Some(47.601117), min_longitude: Some(41.6327159)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kalmikië"), ("am", "ካልምኪያ"), ("ar", "كالميكيا"), ("az", "Kalmıkiya"), ("be", "Калмыкія"), ("bg", "Калмикия"), ("bn", "ক\u{9be}ল\u{9cd}মিয\u{9bc}\u{9be}কিয\u{9bc}\u{9be} রিপ\u{9be}বলিক"), ("bs", "Kalmikija"), ("ca", "Calmúquia"), ("ccp", "𑄇𑄣\u{11134}𑄟\u{11128}𑄠𑄇\u{11128}𑄠"), ("ceb", "Kalmykiya"), ("cs", "Kalmycko"), ("cy", "Gweriniaeth Kalmykia"), ("da", "Kalmykien"), ("de", "Kalmückien"), ("el", "Δημοκρατία της Καλμίκια"), ("en", "Kalmykia"), ("es", "Kalmukia"), ("et", "Kalmõkkia"), ("eu", "Kalmukia"), ("fa", "قالموقستان"), ("fi", "Kalmukia"), ("fr", "Kalmoukie"), ("ga", "An Chailmíc"), ("gl", "Kalmykia"), ("gu", "કાલ\u{acd}મીકિયા રિપબ\u{acd}લિક"), ("he", "קלמיקיה"), ("hi", "कालमिकिया"), ("hr", "Kalmička"), ("hu", "Kalmükföld"), ("hy", "Կալմիկիա"), ("id", "Kalmykia"), ("it", "Calmucchia"), ("ja", "カルムイク共和国"), ("ka", "ყალმუხეთი"), ("kk", "Қалмақстан"), ("kn", "ಕಲ\u{ccd}ಮೀಕ\u{cbf}ಯಾ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "칼미크 공화국"), ("ky", "Калмакстан"), ("lt", "Kalmukija"), ("lv", "Kalmikija"), ("mk", "Калмикија"), ("mn", "Бүгд Найрамдах Халимаг Улс"), ("mr", "काल\u{94d}मिकिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Kalmykia"), ("nb", "Kalmykia"), ("nl", "Kalmukkië"), ("no", "Kalmykia"), ("pl", "Kałmucja"), ("pt", "Calmúquia"), ("ro", "Calmîchia"), ("ru", "Калмыкия"), ("si", "කල\u{dca}ම\u{dd3}ක\u{dd2}ය\u{dcf}"), ("sk", "Kalmycko"), ("sl", "Kalmikija"), ("sr", "Калмикија"), ("sr_Latn", "Kalmikija"), ("sv", "Kalmuckien"), ("sw", "Kalmykia"), ("ta", "க\u{bbe}ல\u{bcd}ம\u{bc0}க\u{bcd}கிய\u{bbe}"), ("te", "ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆప\u{c4d} క\u{c3e}ల\u{c4d}మ\u{c48}క\u{c3f}య\u{c3e}"), ("th", "สาธารณร\u{e31}ฐค\u{e31}ลม\u{e37}ยค\u{e35}ยา"), ("tr", "Kalmukya"), ("uk", "Калмикія"), ("ur", "کلمیکیا"), ("uz", "Qalmogʻiston"), ("vi", "Kalmykia"), ("yue", "卡爾梅克"), ("yue_Hans", "卡尔梅克"), ("zh", "卡尔梅克共和国")]),
                        unofficial_name_list: ["Halmg-Tangč", "Kalmyk Republic", "Kalmykija", "Khalmg-Tangch"].to_vec(),
                    }
                ),
                (
                    "KLU",
                    Subdivision{
                        name: "KLU",
                        country_alpha2: Alpha2::RU,
                        code: "KLU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.3872666), longitude: Some(35.1889094), max_latitude: Some(55.3402639), min_latitude: Some(53.2759969), max_longitude: Some(37.2756489), min_longitude: Some(33.4465778)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kaloega-oblast"), ("ar", "كالوغا أوبلاست"), ("az", "Kaluqa vilayəti"), ("be", "Калужская вобласць"), ("bg", "Калужка област"), ("bn", "ক\u{9be}ল\u{9c1}গ\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kaluška oblast"), ("ca", "Província de Kaluga"), ("ccp", "𑄇𑄣\u{1112a}𑄉"), ("ceb", "Kaluzhskaya Oblast’"), ("cs", "Kalužská oblast"), ("cy", "Oblast Kaluga"), ("da", "Kaluga oblast"), ("de", "Oblast Kaluga"), ("el", "Περιφέρεια Καλούγκα"), ("en", "Kaluga"), ("es", "Kaluga"), ("et", "Kaluga oblast"), ("eu", "Kalugako oblasta"), ("fa", "استان کالوگا"), ("fi", "Kalugan alue"), ("fr", "Oblast de Kalouga"), ("ga", "Cúige Kaluga"), ("gu", "કાલ\u{ac1}ગો ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קלוגה"), ("hi", "कल\u{941}गा ओब\u{94d}लास\u{94d}ट"), ("hr", "Kaluška oblast"), ("hu", "Kalugai terület"), ("hy", "Կալուգայի մարզ"), ("id", "Oblast Kaluga"), ("it", "Oblast’ di Kaluga"), ("ja", "カルーガ州"), ("ka", "კალუგის ოლქი"), ("kn", "ಕಲುಗು ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "칼루가 주"), ("lt", "Kalugos sritis"), ("lv", "Kalugas apgabals"), ("mk", "Калушка област"), ("mr", "काल\u{941}गा ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kaluga"), ("nb", "Kaluga"), ("nl", "Oblast Kaloega"), ("no", "Kaluga"), ("pl", "Obwód kałuski"), ("pt", "Oblast de Kaluga"), ("ro", "Regiunea Kaluga"), ("ru", "Калужская область"), ("si", "කළ\u{dd4}ග\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kalužská oblasť"), ("sl", "Kaluška oblast"), ("sr", "Калушка област"), ("sr_Latn", "Kaluška oblast"), ("sv", "Kaluga oblast"), ("sw", "Kaluga Oblast"), ("ta", "களுக\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "క\u{c3e}లుగ\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เขตคาล\u{e39}กา"), ("tr", "Kaluga Oblastı"), ("uk", "Калузька область"), ("ur", "کالوگا اوبلاست"), ("uz", "Kaluga oblasti"), ("vi", "Kaluga"), ("yue", "卡盧加州"), ("yue_Hans", "卡卢加州"), ("zh", "卡卢加州")]),
                        unofficial_name_list: ["Kaluzhskaya Oblast", "Kalužskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KO",
                    Subdivision{
                        name: "KO",
                        country_alpha2: Alpha2::RU,
                        code: "KO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(63.8630539), longitude: Some(54.8312689), max_latitude: Some(68.42287689999999), min_latitude: Some(59.196101), max_longitude: Some(66.2523211), min_longitude: Some(45.4046569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Republiek van Komi"), ("am", "የኮሚ ሪፐብሊክ"), ("ar", "جمهورية كومي"), ("az", "Komi Respublikası"), ("be", "Рэспубліка Комі"), ("bg", "Коми"), ("bn", "কোমি রিপ\u{9be}বলিক"), ("bs", "Komi"), ("ca", "República de Komi"), ("ccp", "𑄇\u{1112e}𑄟\u{11128}"), ("ceb", "Komi"), ("cs", "Republika Komi"), ("da", "Republikken Komi"), ("de", "Republik Komi"), ("el", "Δημοκρατία των Κόμι"), ("en", "Komi"), ("es", "Komi"), ("et", "Komi"), ("eu", "Komi"), ("fa", "کومی"), ("fi", "Komin tasavalta"), ("fr", "République des Komis"), ("ga", "Poblacht Choimí"), ("gl", "República de Komi"), ("gu", "કોમી રિપબ\u{acd}લિક"), ("he", "רפובליקת קומי"), ("hi", "कोमी गणराज\u{94d}य"), ("hr", "Komi"), ("hu", "Komiföld"), ("hy", "Կոմի Հանրապետություն"), ("id", "Republik Komi"), ("it", "Repubblica dei Komi"), ("ja", "コミ共和国"), ("ka", "კომის რესპუბლიკა"), ("kk", "Коми Республикасы"), ("kn", "ಕೋಮ\u{cbf} ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "코미 공화국"), ("ky", "Коми Республикасы"), ("lt", "Komijos Respublika"), ("lv", "Komi"), ("mk", "Коми"), ("mn", "Коми"), ("mr", "कोमी प\u{94d}रजासत\u{94d}ताक"), ("ms", "Republik Komi"), ("nb", "Komi"), ("nl", "Komi"), ("no", "Komi"), ("pl", "Republika Komi"), ("pt", "República de Komi"), ("ro", "Republica Komi"), ("ru", "Республика Коми"), ("si", "කෝම\u{dd2} ජනරජය"), ("sk", "Komijsko"), ("sl", "Komi"), ("sr", "Коми"), ("sr_Latn", "Komi"), ("sv", "Komi"), ("sw", "Jamhuri ya Komi"), ("ta", "கோமி"), ("te", "క\u{c4b}మ\u{c3f} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "โคม\u{e34}"), ("tr", "Komi Cumhuriyeti"), ("uk", "Республіка Комі"), ("ur", "کومی جمہوریہ"), ("uz", "Komi"), ("vi", "Cộng hòa Komi"), ("yue", "科密"), ("yue_Hans", "科密"), ("zh", "科米共和国")]),
                        unofficial_name_list: ["Komi, Respublika"].to_vec(),
                    }
                ),
                (
                    "KOS",
                    Subdivision{
                        name: "KOS",
                        country_alpha2: Alpha2::RU,
                        code: "KOS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.5501069), longitude: Some(43.9541103), max_latitude: Some(59.62038), min_latitude: Some(57.277828), max_longitude: Some(47.6446471), min_longitude: Some(40.3997611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kostroma-oblast"), ("ar", "كوستروما أوبلاست"), ("az", "Kostroma vilayəti"), ("be", "Кастрамская вобласць"), ("bg", "Костромска област"), ("bn", "কস\u{9cd}ট\u{9cd}রোম\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kostromska oblast"), ("ca", "Província de Kostromà"), ("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄑\u{11133}𑄢\u{11127}𑄟"), ("ceb", "Kostromskaya Oblast’"), ("cs", "Kostromská oblast"), ("cy", "Oblast Kostroma"), ("da", "Kostroma oblast"), ("de", "Oblast Kostroma"), ("el", "Περιφέρεια Κοστρομά"), ("en", "Kostroma"), ("es", "Óblast de Kostromá"), ("et", "Kostroma oblast"), ("eu", "Kostroma oblasta"), ("fa", "استان کوستروما"), ("fi", "Kostroman alue"), ("fr", "Oblast de Kostroma"), ("ga", "Cúige Kostroma"), ("gu", "કોસ\u{acd}ટ\u{acd}રોમા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קוסטרומה"), ("hi", "कोस\u{94d}ट\u{94d}रोमा ओब\u{94d}लास\u{94d}ट"), ("hr", "Kostromska oblast"), ("hu", "Kosztromai terület"), ("hy", "Կոստրոմայի մարզ"), ("id", "Oblast Kostroma"), ("it", "Oblast’ di Kostroma"), ("ja", "コストロマ州"), ("ka", "კოსტრომის ოლქი"), ("kn", "ಕೋಸ\u{ccd}ಟ\u{ccd}ರೋಮಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "코스트로마 주"), ("lt", "Kostromos sritis"), ("lv", "Kostromas apgabals"), ("mk", "Костромска област"), ("mr", "कोस\u{94d}त\u{94d}रोमा ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kostroma"), ("nb", "Kostroma"), ("ne", "कोस\u{94d}त\u{94d}रोमा ओब\u{94d}लास\u{94d}ट"), ("nl", "Oblast Kostroma"), ("no", "Kostroma"), ("pl", "Obwód kostromski"), ("pt", "Oblast de Kostroma"), ("ro", "Regiunea Kostroma"), ("ru", "Костромская область"), ("si", "කොස\u{dca}ට\u{dca}\u{200d}රොම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kostromská oblasť"), ("sl", "Kostromska oblast"), ("sr", "Костромска област"), ("sr_Latn", "Kostromska oblast"), ("sv", "Kostroma oblast"), ("sw", "Kostroma Oblast"), ("ta", "கொஸ\u{bcd}ட\u{bcd}ரோம\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "క\u{c3e}స\u{c4d}ట\u{c4d}ర\u{c4b}మ\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "โคลโตรมา โอแบลส"), ("tr", "Kostroma Oblastı"), ("uk", "Костромська область"), ("ur", "کوستروما اوبلاست"), ("uz", "Kostroma viloyati"), ("vi", "Kostroma"), ("yue", "科斯特羅馬州"), ("yue_Hans", "科斯特罗马州"), ("zh", "科斯特罗马州")]),
                        unofficial_name_list: ["Kostromskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KR",
                    Subdivision{
                        name: "KR",
                        country_alpha2: Alpha2::RU,
                        code: "KR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(63.15587019999999), longitude: Some(32.9905552), max_latitude: Some(66.6732639), min_latitude: Some(60.68576710000001), max_longitude: Some(37.9320522), min_longitude: Some(29.3102922)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Republiek van Karelië"), ("am", "የካሬሊያ ሪፐብሊክ"), ("ar", "جمهورية كاريليا"), ("az", "Kareliya Respublikası"), ("be", "Карэлія"), ("bg", "Република Карелия"), ("bn", "ক\u{9be}রেলিয\u{9bc}\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Karelija"), ("ca", "República de Carèlia"), ("ccp", "𑄇𑄢𑄬𑄣\u{11128}𑄠"), ("ceb", "Respublika Kareliya"), ("cs", "Republika Karélie"), ("da", "Karelen"), ("de", "Republik Karelien"), ("el", "Δημοκρατία της Καρελίας"), ("en", "Karelia"), ("es", "República de Carelia"), ("et", "Karjala Vabariik"), ("eu", "Kareliako Errepublika"), ("fa", "جمهوری کارلیا"), ("fi", "Karjalan tasavalta"), ("fr", "République de Carélie"), ("ga", "Poblacht na Cairéile"), ("gl", "República de Carelia"), ("gu", "કર\u{ac7}લિયા રિપબ\u{acd}લિક"), ("he", "קרליה"), ("hi", "कार\u{947}लिया गणत\u{902}त\u{94d}र"), ("hr", "Karelija"), ("hu", "Karélia"), ("hy", "Կարելիայի Հանրապետություն"), ("id", "Republik Karelia"), ("it", "Repubblica di Carelia"), ("ja", "カレリア共和国"), ("ka", "კარელიის რესპუბლიკა"), ("kk", "Карелия Республикасы"), ("kn", "ಕರೀಲ\u{cbf}ಯಾ"), ("ko", "카렐리야 공화국"), ("ky", "Карилия"), ("lt", "Karelijos Respublika"), ("lv", "Karēlijas Republika"), ("mk", "Република Карелија"), ("mn", "Бүгд Найрамдах Карель Улс"), ("mr", "क\u{945}र\u{947}लिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Republik Karelia"), ("nb", "Karelia"), ("nl", "Karelië"), ("no", "Karelia"), ("pl", "Karelia"), ("pt", "República da Carélia"), ("ro", "Republica Carelia"), ("ru", "Республика Карелия"), ("si", "කරෙල\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Karelsko"), ("sl", "Republika Karelija"), ("sq", "Karelia"), ("sr", "Карелија"), ("sr_Latn", "Karelija"), ("sv", "Karelska republiken"), ("sw", "Karelia"), ("ta", "கரேலிய\u{bbe}"), ("te", "ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d} క\u{c3e}ర\u{c47}ల\u{c3f}య\u{c3e}"), ("th", "สาธารณร\u{e31}ฐคาเรเล\u{e35}ย"), ("tr", "Karelya Cumhuriyeti"), ("uk", "Республіка Карелія"), ("ur", "جمہوریہ کریلیا"), ("uz", "Kareliya"), ("vi", "Cộng hòa Kareliya"), ("yue", "卡累利阿共和國"), ("yue_Hans", "卡累利阿共和国"), ("zh", "卡累利阿共和国")]),
                        unofficial_name_list: ["Karelian Republic", "Karelija"].to_vec(),
                    }
                ),
                (
                    "KRS",
                    Subdivision{
                        name: "KRS",
                        country_alpha2: Alpha2::RU,
                        code: "KRS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.76340260000001), longitude: Some(35.3811812), max_latitude: Some(52.4405099), min_latitude: Some(50.9034249), max_longitude: Some(38.5257578), min_longitude: Some(34.0821881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Koersk-oblast"), ("ar", "كورسك أوبلاست"), ("az", "Kursk vilayəti"), ("be", "Курская вобласць"), ("bg", "Курска област"), ("bn", "ক\u{9c1}রস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kurska oblast"), ("ca", "Província de Kursk"), ("ccp", "𑄇𑄢\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Kurskaya Oblast’"), ("cs", "Kurská oblast"), ("cy", "Oblast Kursk"), ("da", "Kursk oblast"), ("de", "Oblast Kursk"), ("el", "Περιφέρεια Κουρσκ"), ("en", "Kursk"), ("es", "óblast de Kursk"), ("et", "Kurski oblast"), ("eu", "Kursk oblasta"), ("fa", "استان کورسک"), ("fi", "Kurskin alue"), ("fr", "Oblast de Koursk"), ("ga", "Cúige Kursk"), ("gu", "ક\u{ac1}ર\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קורסק"), ("hi", "क\u{941}र\u{94d}सक ओब\u{94d}लास\u{94d}ट"), ("hr", "Kurska oblast"), ("hu", "Kurszki terület"), ("hy", "Կուրսկի մարզ"), ("id", "Oblast Kursk"), ("it", "Oblast’ di Kursk"), ("ja", "クルスク州"), ("ka", "კურსკის ოლქი"), ("kn", "ಕುರ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "쿠르스크 주"), ("lt", "Kursko sritis"), ("lv", "Kurskas apgabals"), ("mk", "Курска област"), ("mr", "क\u{941}र\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Kursk"), ("nb", "Kursk"), ("nl", "Oblast Koersk"), ("no", "Kursk"), ("pl", "Obwód kurski"), ("pt", "Oblast de Kursk"), ("ro", "Regiunea Kursk"), ("ru", "Курская область"), ("si", "කර\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kurská oblasť"), ("sl", "Kurska oblast"), ("sr", "Курска област"), ("sr_Latn", "Kurska oblast"), ("sv", "Kursk oblast"), ("sw", "Kursk Oblast"), ("ta", "கூர\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "కుర\u{c4d}సుక\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เค\u{e34}กส\u{e4c} โอแบลส"), ("tr", "Kursk Oblastı"), ("uk", "Курська область"), ("ur", "کورسک اوبلاست"), ("uz", "Kursk viloyati"), ("vi", "Kursk"), ("yue", "庫爾斯克州"), ("yue_Hans", "库尔斯克州"), ("zh", "库尔斯克州")]),
                        unofficial_name_list: ["Kurskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "KYA",
                    Subdivision{
                        name: "KYA",
                        country_alpha2: Alpha2::RU,
                        code: "KYA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(64.24797579999999), longitude: Some(95.11041759999999), max_latitude: Some(81.2663089), min_latitude: Some(51.7734569), max_longitude: Some(113.9162833), min_longitude: Some(76.1117517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Krasnojarsk-krai"), ("ar", "كراسنويارسك كراي"), ("az", "Krasnoyarsk diyarı"), ("be", "Краснаярскі край"), ("bg", "Красноярски край"), ("bn", "ক\u{9cd}র\u{9be}স\u{9cd}নোয\u{9bc}\u{9be}রস\u{9cd}ক ক\u{9cd}র\u{9be}"), ("bs", "Krasnojarski kraj"), ("ca", "Territori de Krasnoiarsk"), ("ccp", "𑄇\u{11133}𑄢𑄌\u{11134}𑄚\u{11131}𑄢\u{11134}𑄥\u{11134}𑄇\u{11134} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Krasnoyarskiy Kray"), ("cs", "Krasnojarský kraj"), ("cy", "Crai Krasnoyarsk"), ("da", "Krasnojarsk kraj"), ("de", "Region Krasnojarsk"), ("el", "Κράι Κρασνογιάρσκ"), ("en", "Krasnoyarsk Krai"), ("es", "Krasnoyarsk"), ("et", "Krasnojarski krai"), ("eu", "Krasnoiarsk kraia"), ("fa", "سرزمین کراسنویارسک"), ("fi", "Krasnojarskin aluepiiri"), ("fr", "Kraï de Krasnoïarsk"), ("ga", "Críoch Krasnoyarsk"), ("gu", "ક\u{acd}રાસ\u{acd}નોયાર\u{acd}સ\u{acd}ક ક\u{acd}ર\u{ac7}ઇ"), ("he", "מחוז קרסנויארסק"), ("hi", "क\u{94d}रस\u{94d}नोयार\u{94d}स\u{94d}क क\u{94d}राय"), ("hr", "Krasnojarski kraj"), ("hu", "Krasznojarszki határterület"), ("hy", "Կրասնոյարսկի երկրամաս"), ("id", "Krai Krasnoyarsk"), ("it", "Territorio di Krasnojarsk"), ("ja", "クラスノヤルスク地方"), ("ka", "კრასნოიარსკის მხარე"), ("kn", "ಕ\u{ccd}ರಾಸ\u{ccd}ನೊಯಾರ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಕ\u{ccd}ರೈ"), ("ko", "크라스노야르스크 지방"), ("ky", "Краснояр крайы"), ("lt", "Krasnojarsko kraštas"), ("lv", "Krasnojarskas novads"), ("mk", "Краснојарски крај"), ("mn", "Красноярскийн хязгаар"), ("mr", "क\u{94d}रास\u{94d}नोयार\u{94d}स\u{94d}क क\u{94d}राय"), ("ms", "Jajahan Krasnoyarsk"), ("nb", "Krasnojarsk"), ("nl", "Kraj Krasnojarsk"), ("no", "Krasnojarsk"), ("pl", "Kraj Krasnojarski"), ("pt", "Krai de Krasnoiarsk"), ("ro", "Krasnoiarsk"), ("ru", "Красноярский край"), ("si", "ජජහන\u{dca} ක\u{dca}\u{200d}රස\u{dca}නොය\u{dcf}ර\u{dca}ස\u{dca}ක\u{dca}"), ("sk", "Krasnojarský kraj"), ("sl", "Krasnojarski kraj"), ("sr", "Краснојарска Покрајина"), ("sr_Latn", "Krasnojarska Pokrajina"), ("sv", "Krasnojarsk kraj"), ("sw", "Krasnoyarsk Krai"), ("ta", "கிர\u{bbe}ஸ\u{bcd}னய\u{bbe}ர\u{bcd}சுக\u{bcd} பிரதேசம\u{bcd}"), ("te", "క\u{c4d}ర\u{c3e}స\u{c4d}న\u{c4b}య\u{c3e}ర\u{c4d}క\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "ด\u{e34}นแดนคร\u{e31}สโนยาสค\u{e4c}"), ("tr", "Krasnoyarsk Krayı"), ("uk", "Красноярський край"), ("ur", "کراسنویارسک کرائی"), ("uz", "Krasnoyarsk oʻlkasi"), ("vi", "Krasnoyarsk"), ("yue", "克拉斯諾亞爾斯克邊疆區"), ("yue_Hans", "克拉斯诺亚尔斯克边疆区"), ("zh", "克拉斯諾亞爾斯克邊疆區")]),
                        unofficial_name_list: ["Krasnojarsk", "Krasnojarsk", "Krasnojarskij Kraj"].to_vec(),
                    }
                ),
                (
                    "LEN",
                    Subdivision{
                        name: "LEN",
                        country_alpha2: Alpha2::RU,
                        code: "LEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(60.0793208), longitude: Some(31.8926644), max_latitude: Some(61.3297682), min_latitude: Some(58.4147179), max_longitude: Some(35.6959784), min_longitude: Some(27.740038)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Leningrad-oblast"), ("ar", "لينينغراد أوبلاست"), ("az", "Leninqrad vilayəti"), ("be", "Ленінградская вобласць"), ("bg", "Ленинградска област"), ("bn", "লেনিনগ\u{9cd}র\u{9be}ড ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Lenjingradska oblast"), ("ca", "Província de Leningrad"), ("ccp", "𑄣𑄬𑄣\u{11128}𑄚\u{11134}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Leningradskaya Oblast’"), ("cs", "Leningradská oblast"), ("cy", "Oblast Leningrad"), ("da", "Leningrad oblast"), ("de", "Oblast Leningrad"), ("el", "Περιφέρεια Λένινγκραντ"), ("en", "Leningrad"), ("es", "Óblast de Leningrado"), ("et", "Leningradi oblast"), ("eu", "Leningrad oblasta"), ("fa", "استان لنینگراد"), ("fi", "Leningradin alue"), ("fr", "Oblast de Léningrad"), ("ga", "Cúige Leningrad"), ("gu", "લ\u{ac7}નિનગ\u{acd}રાડ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז לנינגרד"), ("hi", "ल\u{947}निनग\u{94d}राद ओब\u{94d}लास\u{94d}ट"), ("hr", "Lenjingradska oblast"), ("hu", "Leningrádi terület"), ("hy", "Լենինգրադի մարզ"), ("id", "Oblast Leningrad"), ("is", "Leníngradfylki"), ("it", "Oblast’ di Leningrado"), ("ja", "レニングラード州"), ("ka", "ლენინგრადის ოლქი"), ("kn", "ಲ\u{cc6}ನ\u{cbf}ನ\u{ccd}ಗ\u{ccd}ರಾಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "레닌그라드 주"), ("lt", "Leningrado sritis"), ("lv", "Ļeņingradas apgabals"), ("mk", "Ленинградска област"), ("mn", "Ленинград муж"), ("mr", "ल\u{947}निनग\u{94d}राद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Leningrad"), ("nb", "Leningrad"), ("ne", "ल\u{947}लिनग\u{94d}राड ओब\u{94d}लास\u{94d}ट"), ("nl", "Oblast Leningrad"), ("no", "Leningrad"), ("pa", "ਲ\u{a48}ਨਿਨਗ\u{a4d}ਰਾਦ ਓਬਲਾਸਤ"), ("pl", "Obwód leningradzki"), ("pt", "Oblast de Leningrado"), ("ro", "Regiunea Leningrad"), ("ru", "Ленинградская область"), ("si", "ලේන\u{dd2}න\u{dca}ග\u{dca}රඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Leningradská oblasť"), ("sl", "Leningrajska oblast"), ("sr", "Лењинградска област"), ("sr_Latn", "Lenjingradska oblast"), ("sv", "Leningrad oblast"), ("sw", "Leningrad Oblast"), ("ta", "லெனின\u{bcd}கிர\u{bbe}டு ஓப\u{bcd}லசுது"), ("te", "ల\u{c46}న\u{c3f}న\u{c4d}\u{200c}గ\u{c4d}ర\u{c3e}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นเลน\u{e34}นกราด"), ("tr", "Leningrad eyaleti"), ("uk", "Ленінградська область"), ("ur", "لیننگراڈ اوبلاست"), ("uz", "Leningrad viloyati"), ("vi", "Leningrad"), ("yue", "列寧格勒州"), ("yue_Hans", "列宁格勒州"), ("zh", "列宁格勒州")]),
                        unofficial_name_list: ["Leningradskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "LIP",
                    Subdivision{
                        name: "LIP",
                        country_alpha2: Alpha2::RU,
                        code: "LIP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5264702), longitude: Some(39.2032269), max_latitude: Some(53.5848638), min_latitude: Some(51.8867799), max_longitude: Some(40.764882), min_longitude: Some(37.7224471)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lipetsk-oblast"), ("ar", "ليبيتسك أوبلاست"), ("az", "Lipetsk vilayəti"), ("be", "Ліпецкая вобласць"), ("bg", "Липецка област"), ("bn", "লিপেটস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Lipecka oblast"), ("ca", "Província de Lípetsk"), ("ccp", "𑄣\u{11128}𑄛𑄬𑄖\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Lipetskaya Oblast’"), ("cs", "Lipecká oblast"), ("cy", "Oblast Lipetsk"), ("da", "Lipetsk oblast"), ("de", "Oblast Lipezk"), ("el", "Περιφέρεια Λίπετσκ"), ("en", "Lipetsk"), ("es", "Óblast de Lípetsk"), ("et", "Lipetski oblast"), ("eu", "Lipetsk oblasta"), ("fa", "استان لیپتسک"), ("fi", "Lipetskin alue"), ("fr", "oblast de Lipetsk"), ("ga", "Cúige Lipetsk"), ("gl", "Provincia de Lipetsk"), ("gu", "લિપ\u{ac7}ટ\u{acd}સક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ליפצק"), ("hi", "लिप\u{947}ट\u{94d}सक ओब\u{94d}लास\u{94d}ट"), ("hr", "Lipecka oblast"), ("hu", "Lipecki terület"), ("hy", "Լիպեցկի մարզ"), ("id", "Oblast Lipetsk"), ("it", "Oblast’ di Lipeck"), ("ja", "リペツク州"), ("ka", "ლიპეცკის ოლქი"), ("kk", "Липецк облысы"), ("kn", "ಲ\u{cbf}ಪ\u{cc6}ಟ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "리페츠크 주"), ("ky", "Липецкий областы"), ("lt", "Lipecko sritis"), ("lv", "Ļipeckas apgabals"), ("mk", "Липецка област"), ("mr", "लिप\u{947}त\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Lipetsk"), ("nb", "Lipetsk"), ("nl", "Oblast Lipetsk"), ("no", "Lipetsk"), ("pl", "Obwód lipiecki"), ("pt", "Oblast de Lipetsk"), ("ro", "Regiunea Lipețk"), ("ru", "Липецкая область"), ("si", "ල\u{dd2}පෙට\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Lipecká oblasť"), ("sl", "Lipecka oblast"), ("sr", "Липецка област"), ("sr_Latn", "Lipecka oblast"), ("sv", "Lipetsk oblast"), ("sw", "Lipetsk Oblast"), ("ta", "லிபெட\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ల\u{c3f}ప\u{c46}ట\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เม\u{e37}องล\u{e35}เปตสค\u{e4c}"), ("tr", "Lipetsk Oblastı"), ("uk", "Липецька область"), ("ur", "لیپٹسک اوبلاست"), ("uz", "Lipetsk viloyati"), ("vi", "Lipetsk"), ("yue", "利佩茨克州"), ("yue_Hans", "利佩茨克州"), ("zh", "利佩茨克州")]),
                        unofficial_name_list: ["Lipeck", "Lipeckaja Oblast"].to_vec(),
                    }
                ),
                (
                    "MAG",
                    Subdivision{
                        name: "MAG",
                        country_alpha2: Alpha2::RU,
                        code: "MAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(62.66434169999999), longitude: Some(153.9149909), max_latitude: Some(66.33609200000001), min_latitude: Some(58.83734889999999), max_longitude: Some(163.4827849), min_longitude: Some(144.722207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Magadan-oblast"), ("ar", "ماغادان أوبلاست"), ("az", "Maqadan vilayəti"), ("be", "Магаданская вобласць"), ("bg", "Магаданска област"), ("bn", "ম\u{9be}গ\u{9be}দ\u{9be}ন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Magadanska oblast"), ("ca", "Província de Magadan"), ("ccp", "𑄟𑄉𑄓𑄚\u{11134}"), ("ceb", "Magadanskaya Oblast’"), ("cs", "Magadanská oblast"), ("cy", "Oblast Magadan"), ("da", "Magadan oblast"), ("de", "Oblast Magadan"), ("el", "Περιφέρεια Μαγκαντάν"), ("en", "Magadan"), ("es", "Magadán"), ("et", "Magadani oblast"), ("eu", "Magadan oblasta"), ("fa", "استان ماگادان"), ("fi", "Magadanin alue"), ("fr", "Oblast de Magadan"), ("ga", "Cúige Mhagadan"), ("gu", "મ\u{ac7}ગ\u{ac7}ડન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז מגדן"), ("hi", "मागादान ओब\u{94d}लास\u{94d}त"), ("hr", "Magadanska oblast"), ("hu", "Magadani terület"), ("hy", "Մագադանի մարզ"), ("id", "Oblast Magadan"), ("is", "Magadanfylki"), ("it", "Oblast’ di Magadan"), ("ja", "マガダン州"), ("ka", "მაგადანის ოლქი"), ("kn", "ಮಗಾಡನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "마가단 주"), ("lt", "Magadano sritis"), ("lv", "Magadanas apgabals"), ("mk", "Магаданска област"), ("mn", "Магадан муж"), ("mr", "मागादान ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Magadan"), ("nb", "Magadan"), ("nl", "Oblast Magadan"), ("no", "Magadan"), ("pl", "Obwód magadański"), ("pt", "Oblast de Magadan"), ("ro", "Regiunea Magadan"), ("ru", "Магаданская область"), ("si", "මගඩන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Magadanská oblasť"), ("sl", "Magadanska oblast"), ("sr", "Магаданска област"), ("sr_Latn", "Magadanska oblast"), ("sv", "Magadan oblast"), ("sw", "Magadan Oblast"), ("ta", "மகதன\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "మగ\u{c3e}డ\u{c3e}న\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เขตมากาดาน"), ("tr", "Magadan Oblastı"), ("uk", "Магаданська область"), ("ur", "ماگادان اوبلاست"), ("uz", "Magadan viloyati"), ("vi", "Magadan"), ("yue", "馬加丹州"), ("yue_Hans", "马加丹州"), ("zh", "马加丹州")]),
                        unofficial_name_list: ["Magadanskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "ME",
                    Subdivision{
                        name: "ME",
                        country_alpha2: Alpha2::RU,
                        code: "ME",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.438457), longitude: Some(47.9607757), max_latitude: Some(57.3436309), min_latitude: Some(55.8266931), max_longitude: Some(50.2000648), min_longitude: Some(45.6197337)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mari El"), ("ar", "ماري إل"), ("az", "Mari El"), ("be", "Марый Эл"), ("bg", "Марий Ел"), ("bn", "ম\u{9be}র\u{9be}ি এল প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Marij El"), ("ca", "Marí El"), ("ccp", "𑄟𑄢\u{11128} 𑄃\u{11128}𑄣\u{11134}"), ("ceb", "Respublika Mariy-El"), ("cs", "Marij El"), ("cy", "Mari El"), ("da", "Mari El"), ("de", "Mari El"), ("el", "Δημοκρατία της Μαρί Ελ"), ("en", "Mari El"), ("es", "Mari-El"), ("et", "Marimaa"), ("eu", "Mari El"), ("fa", "ماری ال"), ("fi", "Marin tasavalta"), ("fr", "République des Maris"), ("ga", "Mairí El"), ("gl", "Mari El"), ("gu", "મારી અલ રિપબ\u{acd}લિક"), ("he", "מארי אל"), ("hi", "मरी ऍल"), ("hr", "Marijska"), ("hu", "Mariföld"), ("hy", "Մարիյ Էլ"), ("id", "Mari El"), ("it", "Repubblica dei Mari"), ("ja", "マリ・エル共和国"), ("ka", "მარი ელი"), ("kk", "Марий Эл"), ("kn", "ಮಾರ\u{cbf} ಎಲ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "마리옐 공화국"), ("ky", "Марий Эл Республикасы"), ("lt", "Marių Respublika"), ("lv", "Marijela"), ("mk", "Мариј Ел"), ("mn", "Мари Эл"), ("mr", "मारी एल प\u{94d}रजासत\u{94d}ताक"), ("ms", "Mari El"), ("nb", "Mari El"), ("nl", "Mari El"), ("no", "Mari El"), ("pl", "Mari El"), ("pt", "Mari El"), ("ro", "Mari El"), ("ru", "Марий Эл"), ("si", "ම\u{dcf}ර\u{dd2} එල\u{dca} ජනරජය"), ("sk", "Marijsko"), ("sl", "Marij El"), ("sr", "Мариј Ел"), ("sr_Latn", "Marij El"), ("sv", "Marij El"), ("sw", "Mari El"), ("ta", "ம\u{bbe}ர\u{bc0} எல\u{bcd}"), ("te", "మ\u{c3e}ర\u{c3f}ఎల\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "มาร\u{e34} เอล"), ("tr", "Mari El"), ("uk", "Марій Ел"), ("ur", "ماری ال"), ("uz", "Mariy el"), ("vi", "Cộng hòa Mari El"), ("yue", "馬里埃爾"), ("yue_Hans", "马里埃尔"), ("zh", "马里埃尔共和国")]),
                        unofficial_name_list: ["Marij El", "Mariy El"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::RU,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.2369441), longitude: Some(44.0683969), max_latitude: Some(55.1882439), min_latitude: Some(53.6542969), max_longitude: Some(46.7113749), min_longitude: Some(42.170382)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mordowië"), ("ar", "موردوفيا"), ("az", "Mordoviya"), ("be", "Мардовія"), ("bg", "Мордовия"), ("bn", "মরডোভিয\u{9bc}\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Mordovija"), ("ca", "Mordòvia"), ("ccp", "𑄟\u{11127}𑄢\u{11134}𑄓\u{1112e}𑄞\u{11128}𑄠"), ("ceb", "Respublika Mordoviya"), ("cs", "Mordvinsko"), ("da", "Mordva"), ("de", "Mordwinien"), ("el", "Δημοκρατία της Μορδοβίας"), ("en", "Mordovia"), ("es", "Mordovia"), ("et", "Mordva"), ("eu", "Mordovia"), ("fa", "موردوویا"), ("fi", "Mordva"), ("fr", "Mordovie"), ("ga", "An Mhordóiv"), ("gl", "Mordovia"), ("gu", "મોર\u{acd}દોવિઆ રિપબ\u{acd}લિક"), ("he", "מורדוביה"), ("hi", "मॉर\u{94d}डोविया गणराज\u{94d}य"), ("hr", "Mordvinska"), ("hu", "Mordvinföld"), ("hy", "Մորդովիա"), ("id", "Mordovia"), ("it", "Mordovia"), ("ja", "モルドヴィア共和国"), ("ka", "მორდვეთი"), ("kk", "Мордовия"), ("kn", "ಮೊರ\u{ccd}ಡೋವ\u{cbf}ಯ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "모르도바 공화국"), ("ky", "Мордва республикасы"), ("lt", "Mordvija"), ("lv", "Mordovija"), ("mk", "Мордовија"), ("mn", "Мордова"), ("mr", "मोर\u{94d}दोविया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Mordovia"), ("nb", "Mordovia"), ("nl", "Mordovië"), ("no", "Mordovia"), ("pl", "Mordowia"), ("pt", "Mordóvia"), ("ro", "Mordovia"), ("ru", "Мордовия"), ("si", "මොර\u{dca}ඩොව\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Mordviansko"), ("sl", "Mordovija"), ("sr", "Мордовија"), ("sr_Latn", "Mordovija"), ("sv", "Mordvinien"), ("sw", "Mordovia"), ("ta", "மொர\u{bcd}தோவிய\u{bbe}"), ("te", "మ\u{c4b}ర\u{c4d}డ\u{c4b}వ\u{c3f}య\u{c3e} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "มอร\u{e4c}โดว\u{e34}ยา"), ("tr", "Mordovya"), ("uk", "Мордовія"), ("ur", "موردوویا"), ("uz", "Mordoviya"), ("vi", "Mordovia"), ("yue", "莫爾多瓦"), ("yue_Hans", "莫尔多瓦"), ("zh", "莫尔多瓦共和国")]),
                        unofficial_name_list: ["Mordovian Republic", "Mordovija"].to_vec(),
                    }
                ),
                (
                    "MOS",
                    Subdivision{
                        name: "MOS",
                        country_alpha2: Alpha2::RU,
                        code: "MOS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.340396), longitude: Some(38.2917651), max_latitude: Some(56.962834), min_latitude: Some(54.254261), max_longitude: Some(40.2060071), min_longitude: Some(35.149022)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Moskou-oblast"), ("ar", "محافظة موسكو"), ("az", "Moskva vilayəti"), ("be", "Маскоўская вобласць"), ("bg", "Московска област"), ("bn", "মস\u{9cd}কো ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Moskovska oblast"), ("ca", "Província de Moscou"), ("ccp", "𑄟\u{11127}𑄌\u{11134}𑄇\u{1112e} 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Moscow Oblast"), ("cs", "Moskevská oblast"), ("cy", "Oblast Moscfa"), ("da", "Moskva oblast"), ("de", "Oblast Moskau"), ("el", "Περιφέρεια Μόσχας"), ("en", "Moscow Province"), ("es", "Óblast de Moscú"), ("et", "Moskva oblast"), ("eu", "Moskuko oblasta"), ("fa", "استان مسکو"), ("fi", "Moskovan alue"), ("fr", "oblast de Moscou"), ("ga", "Cúige Mhoscó"), ("gu", "મોસ\u{acd}કો ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז מוסקבה"), ("hi", "मास\u{94d}को ओब\u{94d}लास\u{94d}ट"), ("hr", "Moskovska oblast"), ("hu", "Moszkvai terület"), ("hy", "Մոսկվայի մարզ"), ("id", "Oblast Moskwa"), ("it", "Oblast’ di Mosca"), ("ja", "モスクワ州"), ("ka", "მოსკოვის ოლქი"), ("kn", "ಮಾಸ\u{ccd}ಕೋ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "모스크바 주"), ("ky", "Москва областы"), ("lt", "Maskvos sritis"), ("lv", "Maskavas apgabals"), ("mk", "Московска област"), ("mn", "Москва муж"), ("mr", "मॉस\u{94d}को ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Moscow"), ("nb", "Moskva oblast"), ("nl", "Oblast Moskou"), ("no", "Moskva"), ("pl", "Obwód moskiewski"), ("pt", "Oblast de Moscou"), ("ro", "regiunea Moscova"), ("ru", "Московская область"), ("si", "මොස\u{dca}කව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Moskovská oblasť"), ("sl", "Moskovska oblast"), ("sr", "Московска област"), ("sr_Latn", "Moskovska oblast"), ("sv", "Moskva oblast"), ("sw", "Moscow Oblast"), ("ta", "ம\u{bbe}ஸ\u{bcd}கோ ஓப\u{bcd}லஸ\u{bcd}து"), ("te", "మ\u{c3e}స\u{c4d}క\u{c4b} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เขตด\u{e34}อ\u{e31}วร\u{e4c}เบล"), ("tr", "Moskova Oblastı"), ("uk", "Московська область"), ("ur", "ماسکو اوبلاست"), ("uz", "Moskva oblasti"), ("vi", "Moskva"), ("yue", "莫斯科州"), ("yue_Hans", "莫斯科州"), ("zh", "莫斯科州")]),
                        unofficial_name_list: ["Moskovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "MOW",
                    Subdivision{
                        name: "MOW",
                        country_alpha2: Alpha2::RU,
                        code: "MOW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.755826), longitude: Some(37.6173), max_latitude: Some(56.009657), min_latitude: Some(55.48992699999999), max_longitude: Some(37.9456611), min_longitude: Some(37.3193288)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Moskou"), ("am", "ሞስኮ"), ("ar", "موسكو"), ("az", "Moskva"), ("be", "Масква"), ("bg", "Москва"), ("bn", "মস\u{9cd}কো"), ("bs", "Moskva"), ("ca", "Moscou"), ("ccp", "𑄟\u{11127}𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Moscow"), ("cs", "Moskva"), ("cy", "Moscfa"), ("da", "Moskva"), ("de", "Moskau"), ("el", "Μόσχα"), ("en", "Moscow"), ("es", "Moscú"), ("et", "Moskva"), ("eu", "Mosku"), ("fa", "مسکو"), ("fi", "Moskova"), ("fr", "Moscou"), ("ga", "Moscó"), ("gl", "Moscova"), ("gu", "મોસ\u{acd}કો"), ("ha", "Moscow"), ("ha_NE", "Moscow"), ("he", "מוסקבה"), ("hi", "मास\u{94d}को"), ("hr", "Moskva"), ("hu", "Moszkva"), ("hy", "Մոսկվա"), ("id", "Moskwa"), ("is", "Moskva"), ("it", "Mosca"), ("ja", "モスクワ"), ("jv", "Moskwa"), ("ka", "მოსკოვი"), ("kk", "Мәскеу"), ("kn", "ಮಾಸ\u{ccd}ಕೋ"), ("ko", "모스크바"), ("ky", "Москва"), ("lt", "Maskva"), ("lv", "Maskava"), ("mk", "Москва"), ("ml", "മോസ\u{d4d}കോ"), ("mn", "Москва"), ("mr", "मॉस\u{94d}को"), ("ms", "Moscow"), ("my", "မော\u{103a}စက\u{102d}\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Moskva²"), ("ne", "मस\u{94d}को"), ("nl", "Moskou"), ("no", "Moskva²"), ("or", "ମସ\u{b4d}କୋ"), ("pa", "ਮਾਸਕ\u{a4b}"), ("pl", "Moskwa"), ("ps", "مسکو"), ("pt", "Moscovo"), ("ro", "Moscova"), ("ru", "Москва"), ("sd", "ماسڪو"), ("si", "මොස\u{dca}කව\u{dca}"), ("sk", "Moskva"), ("sl", "Moskva"), ("so", "Moskow"), ("sq", "Moska"), ("sr", "Москва"), ("sr_Latn", "Moskva"), ("sv", "Moskva"), ("sw", "Moscow"), ("ta", "ம\u{bbe}ஸ\u{bcd}கோ"), ("te", "మ\u{c3e}స\u{c4d}క\u{c4b}"), ("th", "มอสโก"), ("tk", "Moskwa"), ("tr", "Moskova"), ("uk", "Москва"), ("ur", "ماسکو"), ("uz", "Moskva"), ("vi", "Moskva²"), ("yo", "Mọsko"), ("yo_BJ", "Mɔsko"), ("yue", "莫斯科"), ("yue_Hans", "莫斯科"), ("zh", "莫斯科"), ("zu", "IMoskwa")]),
                        unofficial_name_list: ["Moscou", "Moscú", "Moskau", "Moskva"].to_vec(),
                    }
                ),
                (
                    "MUR",
                    Subdivision{
                        name: "MUR",
                        country_alpha2: Alpha2::RU,
                        code: "MUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(67.84426739999999), longitude: Some(35.0884101), max_latitude: Some(69.9520907), min_latitude: Some(66.0565385), max_longitude: Some(41.4017088), min_longitude: Some(28.4163852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Moermansk-oblast"), ("ar", "مورمانسك أوبلاست"), ("az", "Murmansk vilayəti"), ("be", "Мурманская вобласць"), ("bg", "Мурманска област"), ("bn", "ম\u{9c1}রম\u{9be}নস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Murmanska oblast"), ("ca", "Província de Múrmansk"), ("ccp", "𑄟𑄢\u{11134}𑄟\u{11133}𑄠𑄚\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Murmanskaya Oblast’"), ("cs", "Murmanská oblast"), ("cy", "Oblast Murmansk"), ("da", "Murmansk oblast"), ("de", "Oblast Murmansk"), ("el", "Περιφέρεια Μούρμανσκ"), ("en", "Murmansk"), ("es", "Óblast de Múrmansk"), ("et", "Murmanski oblast"), ("eu", "Murmansk oblasta"), ("fa", "استان مورمانسک"), ("fi", "Murmanskin alue"), ("fr", "Oblast de Mourmansk"), ("ga", "Cúige Mhurmansk"), ("gl", "Óblast de Múrmansk"), ("gu", "મર\u{acd}માન\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז מורמנסק"), ("hi", "म\u{942}रमान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("hr", "Murmanska oblast"), ("hu", "Murmanszki terület"), ("hy", "Մուրմանսկի մարզ"), ("id", "Oblast Murmansk"), ("it", "Oblast’ di Murmansk"), ("ja", "ムルマンスク州"), ("ka", "მურმანსკის ოლქი"), ("kn", "ಮರ\u{ccd}ಮನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "무르만스크 주"), ("lt", "Murmansko sritis"), ("lv", "Murmanskas apgabals"), ("mk", "Мурманска област"), ("mn", "Мурманск муж"), ("mr", "म\u{941}र\u{94d}मान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Murmansk"), ("nb", "Murmansk"), ("nl", "Oblast Moermansk"), ("no", "Murmansk"), ("pl", "Obwód murmański"), ("pt", "Oblast de Murmansk"), ("ro", "Regiunea Murmansk"), ("ru", "Мурманская область"), ("si", "මර\u{dca}මන\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Murmanská oblasť"), ("sl", "Murmanska oblast"), ("sr", "Мурманска област"), ("sr_Latn", "Murmanska oblast"), ("sv", "Murmansk oblast"), ("sw", "Murmansk Oblast"), ("ta", "முர\u{bcd}ம\u{bbe}ன\u{bcd}சுக\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ముర\u{c4d}మ\u{c3e}న\u{c4d}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เมอร\u{e4c}แมนสกายา"), ("tr", "Murmansk Oblastı"), ("uk", "Мурманська область"), ("ur", "مورمانسک اوبلاست"), ("uz", "Murmansk viloyati"), ("vi", "Murmansk"), ("yue", "莫曼斯克州"), ("yue_Hans", "莫曼斯克州"), ("zh", "摩爾曼斯克州")]),
                        unofficial_name_list: ["Murmanskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "NEN",
                    Subdivision{
                        name: "NEN",
                        country_alpha2: Alpha2::RU,
                        code: "NEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(67.6078337), longitude: Some(57.63383309999999), max_latitude: Some(70.4649036), min_latitude: Some(65.823471), max_longitude: Some(65.6777838), min_longitude: Some(43.2703527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nenetsië"), ("ar", "أوكروغ نينيتس الذاتية"), ("az", "Nenets Muxtar Dairəsi"), ("be", "Ненецкая аўтаномная акруга"), ("bg", "Ненецки автономен окръг"), ("bn", "নেনেটস স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত অক\u{9cd}র\u{9c1}গ"), ("bs", "Nenečki autonomni okrug"), ("ca", "Nenètsia"), ("ccp", "𑄚𑄚𑄬𑄖\u{11134}𑄌\u{11134}"), ("ceb", "Nenetskiy Avtonomnyy Okrug"), ("cs", "Něnecký autonomní okruh"), ("cy", "Ocrwg Ymreolaethol Nenets"), ("da", "Nenets autonome okrug"), ("de", "Autonomer Kreis der Nenzen"), ("el", "Αυτόνομος θύλακας της Νενετσίας"), ("en", "Nenets"), ("es", "Nenetsia"), ("et", "Neenetsimaa"), ("eu", "Nenetsia"), ("fa", "ننتسیا"), ("fi", "Nenetsia"), ("fr", "Nénétsie"), ("ga", "Ceantar Féinrialaitheach na Neinéitseach"), ("gu", "ન\u{ac7}ન\u{ac7}ટ\u{acd}સ ઓટોનોમસ ઓક\u{acd}રગ"), ("he", "הניינץ"), ("hi", "न\u{947}न\u{947}ट\u{94d}स औटोनोमस ओक\u{94d}र\u{941}ग"), ("hr", "Nenečki autonomni okrug"), ("hu", "Nyenyecföld"), ("hy", "Նենեցյան ինքնավար օկրուգ"), ("id", "Nenetsia"), ("it", "Circondario autonomo dei Nenec"), ("ja", "ネネツ自治管区"), ("ka", "ნენთა ავტონომიური ოკრუგი"), ("kn", "ನ\u{cc6}ನ\u{cc6}ಟ\u{ccd}ಸ\u{ccd} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಒಕ\u{ccd}ರುಗ\u{ccd}"), ("ko", "네네츠 자치구"), ("ky", "Ненец автономия округу"), ("lt", "Nencų autonominė apygarda"), ("lv", "Ņencu autonomais apvidus"), ("mk", "Ненецки автономен округ"), ("mr", "न\u{947}न\u{947}त\u{94d}स स\u{94d}वायत\u{94d}त ऑक\u{94d}र\u{942}ग"), ("ms", "Negeri autonomi Nenets"), ("nb", "Nenetsk"), ("nl", "Nenetsië"), ("no", "Nenetsk"), ("pl", "Nieniecki Okręg Autonomiczny"), ("pt", "Nenetsia"), ("ro", "Districtul autonom Neneția"), ("ru", "Ненецкий автономный округ"), ("si", "නෙනේට\u{dca}ස\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd2}න කල\u{dcf}පය"), ("sk", "Nenecko"), ("sr", "Ненеција"), ("sr_Latn", "Nenecija"), ("sv", "Nentsien"), ("sw", "Okrug Huru wa Nenets"), ("ta", "நெனெத\u{bcd}து தன\u{bcd}ன\u{bbe}ட\u{bcd}சி வட\u{bcd}ட\u{bbe}ரம\u{bcd}"), ("te", "న\u{c46}న\u{c47}ట\u{c4d}స\u{c4d} ఆట\u{c3e}నమస\u{c4d} ఓక\u{c4d}రుగ\u{c4d}"), ("th", "เนเน\u{e47}ท ออโตโนม\u{e31}ส โอคร\u{e31}ค"), ("tr", "Nenets Özerk Okrugu"), ("uk", "Ненецький автономний округ"), ("ur", "نینیتس خود مختار آکرگ"), ("uz", "Nenetslar muxtor okrugi"), ("vi", "Nenetsia"), ("yue", "涅涅茨自治區"), ("yue_Hans", "涅涅茨自治区"), ("zh", "涅涅茨自治区")]),
                        unofficial_name_list: ["Nenetskij Avtonomnyj Okrug"].to_vec(),
                    }
                ),
                (
                    "NGR",
                    Subdivision{
                        name: "NGR",
                        country_alpha2: Alpha2::RU,
                        code: "NGR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.2427552), longitude: Some(32.5665191), max_latitude: Some(59.44515209999999), min_latitude: Some(56.917883), max_longitude: Some(36.2195299), min_longitude: Some(29.62305709999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nowgorod-oblast"), ("ar", "نوفغورود أوبلاست"), ("az", "Novqorod vilayəti"), ("be", "Наўгародская вобласць"), ("bg", "Новгородска област"), ("bn", "নোভগরদ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Novgorodska oblast"), ("ca", "Província de Nóvgorod"), ("ccp", "𑄚\u{1112e}𑄛\u{11134}𑄉\u{1112e}𑄢\u{11127}𑄖\u{11134}"), ("ceb", "Novgorodskaya Oblast’"), ("cs", "Novgorodská oblast"), ("cy", "Oblast Novgorod"), ("da", "Novgorod oblast"), ("de", "Oblast Nowgorod"), ("el", "Περιφέρεια Νόβγκοροντ"), ("en", "Novgorod"), ("es", "Óblast de Nóvgorod"), ("et", "Novgorodi oblast"), ("eu", "Novgorod oblasta"), ("fa", "استان نووگورود"), ("fi", "Novgorodin alue"), ("fr", "oblast de Novgorod"), ("ga", "Cúige Novgorod"), ("gu", "નોવગોરોડ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז נובגורוד"), ("hi", "नोवगोरोड ओब\u{94d}लास\u{94d}ट"), ("hr", "Novgorodska oblast"), ("hu", "Novgorodi terület"), ("hy", "Նովգորոդի մարզ"), ("id", "Oblast Novgorod"), ("it", "Oblast’ di Novgorod"), ("ja", "ノヴゴロド州"), ("ka", "ნოვგოროდის ოლქი"), ("kn", "ನವ\u{ccd}ಗೊರೊಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "노브고로드 주"), ("lt", "Novgorodo sritis"), ("lv", "Novgorodas apgabals"), ("mk", "Новгородска област"), ("mr", "नॉवगोरोद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Novgorod"), ("nb", "Novgorod"), ("nl", "Oblast Novgorod"), ("no", "Novgorod"), ("pl", "Obwód nowogrodzki"), ("pt", "Oblast de Novgorod"), ("ro", "Regiunea Novgorod"), ("ru", "Новгородская область"), ("si", "නොව\u{dca}ගොරොඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Novgorodská oblasť"), ("sl", "Novgorodska oblast"), ("sr", "Новгородска област"), ("sr_Latn", "Novgorodska oblast"), ("sv", "Novgorod oblast"), ("sw", "Novgorod Oblast"), ("ta", "நொவ\u{bcd}கோரோட\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "న\u{c3e}వ\u{c4d}\u{200c}గ\u{c4a}ర\u{c4b}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "นอวกอรอด โอแบลส"), ("tr", "Novgorod Oblastı"), ("uk", "Новгородська область"), ("ur", "نووگورود اوبلاست"), ("uz", "Novgorod viloyati"), ("vi", "Novgorod"), ("yue", "諾夫哥羅德州"), ("yue_Hans", "诺夫哥罗德州"), ("zh", "諾夫哥羅德州")]),
                        unofficial_name_list: ["Novgorodskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "NIZ",
                    Subdivision{
                        name: "NIZ",
                        country_alpha2: Alpha2::RU,
                        code: "NIZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7995159), longitude: Some(44.0296769), max_latitude: Some(58.0889949), min_latitude: Some(54.47186689999999), max_longitude: Some(47.7473761), min_longitude: Some(41.775117)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nizjni Nowgorod-oblast"), ("ar", "نيجني نوفغورود أوبلاست"), ("az", "Nijeqorod vilayəti"), ("be", "Ніжагародская вобласць"), ("bg", "Нижегородска област"), ("bn", "নিঝনি ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Nižnjenovgorodska oblast"), ("ca", "Província de Nijni Nóvgorod"), ("ccp", "𑄚\u{11128}𑄌\u{11134}𑄚\u{11128} 𑄚\u{1112e}𑄛\u{11134}𑄉\u{1112e}𑄢\u{11127}𑄖\u{11134}"), ("ceb", "Nizhegorodskaya Oblast’"), ("cs", "Nižněnovgorodská oblast"), ("cy", "Oblast Nizhny Novgorod"), ("da", "Nisjnij Novgorod oblast"), ("de", "Oblast Nischni Nowgorod"), ("el", "Περιφέρεια Νίζνι Νόβγκοροντ"), ("en", "Nizhny Novgorod"), ("es", "Óblast de Nizhni Nóvgorod"), ("et", "Nižni Novgorodi oblast"), ("eu", "Nizhni Novgorod oblasta"), ("fa", "استان نیژنی نووگورود"), ("fi", "Nižni Novgorodin alue"), ("fr", "oblast de Nijni Novgorod"), ("ga", "Cúige Nizhny Novgorod"), ("gu", "નિઝની નોવ\u{acd}ગોરોડ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ניז׳ני נובגורוד"), ("hi", "निज\u{93c}नी नोवोगोरोड ओब\u{94d}लास\u{94d}ट"), ("hr", "Nižnjenovgorodska oblast"), ("hu", "Nyizsnyij Novgorod-i terület"), ("hy", "Նիժնի Նովգորոդի մարզ"), ("id", "Oblast Nizhny Novgorod"), ("it", "Oblast’ di Nižnij Novgorod"), ("ja", "ニジニ・ノヴゴロド州"), ("ka", "ნიჟნი-ნოვგოროდის ოლქი"), ("kn", "ನ\u{cbf}ಜ\u{ccd}ನ\u{cbf} ನವ\u{ccd}ಗೊರೊಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "니즈니노브고로드 주"), ("lt", "Žemutinio Naugardo sritis"), ("lv", "Ņižņijnovgorodas apgabals"), ("mk", "Нижегородска област"), ("mn", "Нижегород муж"), ("mr", "निज\u{94d}नी नॉवगोरोद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Nizhny Novgorod"), ("nb", "Nizjnij Novgorod"), ("nl", "Oblast Nizjni Novgorod"), ("no", "Nizjnij Novgorod"), ("pl", "Obwód niżnonowogrodzki"), ("pt", "Oblast de Níjni Novgorod"), ("ro", "Regiunea Nijni Novgorod"), ("ru", "Нижегородская область"), ("si", "න\u{dd2}ස\u{dca}න\u{dd2} නොව\u{dca}ගොරොඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Nižnonovgorodská oblasť"), ("sl", "Niženovgorodska oblast"), ("sr", "Нижегородска област"), ("sr_Latn", "Nižegorodska oblast"), ("sv", "Nizjnij Novgorod oblast"), ("sw", "Nizhny Novgorod Oblast"), ("ta", "ந\u{bc0}ஷ\u{bcd}னி நொவ\u{bcd}கோரோட\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "న\u{c3f}జ\u{c4d}న\u{c40} న\u{c4b}వ\u{c4d}\u{200c}గ\u{c4b}ర\u{c4b}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "น\u{e34}จน\u{e35}นอฟโกรอด"), ("tr", "Nijniy Novgorod Oblastı"), ("uk", "Нижньогородська область"), ("ur", "نزہنی نووگورود اوبلاست"), ("uz", "Nijegorod viloyati"), ("vi", "Nizhny Novgorod"), ("yue", "下諾夫哥羅德州"), ("yue_Hans", "下诺夫哥罗德州"), ("zh", "下诺夫哥罗德州")]),
                        unofficial_name_list: ["Gorki", "Gorkij", "Gorky", "Nizhegorodskaya Oblast", "Nižegorodskaja Oblast", "Nižnij Novgorod"].to_vec(),
                    }
                ),
                (
                    "NVS",
                    Subdivision{
                        name: "NVS",
                        country_alpha2: Alpha2::RU,
                        code: "NVS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.4467133), longitude: Some(80.1043924), max_latitude: Some(57.236193), min_latitude: Some(53.2909191), max_longitude: Some(85.11756299999999), min_longitude: Some(75.08785499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nowosibirsk-oblast"), ("ar", "نوفوسيبيرسك أوبلاست"), ("az", "Novosibirsk vilayəti"), ("be", "Новасібірская вобласць"), ("bg", "Новосибирска област"), ("bn", "নভোসিবিরস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Novosibirska oblast"), ("ca", "Província de Novosibirsk"), ("ccp", "𑄚\u{11127}𑄞\u{1112e}𑄥\u{11128}𑄝\u{11128}𑄢\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Novosibirskaya Oblast’"), ("cs", "Novosibirská oblast"), ("cy", "Oblast Novosibirsk"), ("da", "Novosibirsk oblast"), ("de", "Oblast Nowosibirsk"), ("el", "Περιφέρεια Νοβοσιμπίρσκ"), ("en", "Novosibirsk"), ("es", "Novosibirsk"), ("et", "Novosibirski oblast"), ("eu", "Novosibirsk oblasta"), ("fa", "استان نووسیبیرسک"), ("fi", "Novosibirskin alue"), ("fr", "Oblast de Novossibirsk"), ("ga", "Cúige Novosibirsk"), ("gu", "નોવોસિબિર\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז נובוסיבירסק"), ("hi", "नोवोसिबिर\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Novosibirska oblast"), ("hu", "Novoszibirszki terület"), ("hy", "Նովոսիբիրսկի մարզ"), ("id", "Oblast Novosibirsk"), ("it", "Oblast’ di Novosibirsk"), ("ja", "ノヴォシビルスク州"), ("ka", "ნოვოსიბირსკის ოლქი"), ("kk", "Новосібір облысы"), ("kn", "ನೋವೊಸ\u{cbf}ಬ\u{cbf}ರ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "노보시비르스크 주"), ("lt", "Novosibirsko sritis"), ("lv", "Novosibirskas apgabals"), ("mk", "Новосибирска област"), ("mr", "नोवोसिबिर\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Novosibirsk"), ("nb", "Novosibirsk"), ("nl", "Oblast Novosibirsk"), ("no", "Novosibirsk"), ("pl", "Obwód nowosybirski"), ("pt", "Oblast de Novosibirsk"), ("ro", "Regiunea Novosibirsk"), ("ru", "Новосибирская область"), ("si", "නොස\u{dd2}බ\u{dd2}ර\u{dd2}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Novosibirská oblasť"), ("sl", "Novosibirska oblast"), ("sr", "Новосибирска област"), ("sr_Latn", "Novosibirska oblast"), ("sv", "Novosibirsk oblast"), ("sw", "Novosibirsk Oblast"), ("ta", "நோவசிப\u{bc0}ர\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "న\u{c4b}వ\u{c3e}\u{c4b}స\u{c3f}బ\u{c4d}ర\u{c3f}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นโนโวซ\u{e35}บ\u{e35}สค\u{e4c}"), ("tr", "Novosibirsk Oblastı"), ("uk", "Новосибірська область"), ("ur", "نووسیبرسک اوبلاست"), ("uz", "Novosibirsk viloyati"), ("vi", "Novosibirsk"), ("yue", "新西伯利亞州"), ("yue_Hans", "新西伯利亚州"), ("zh", "新西伯利亚州")]),
                        unofficial_name_list: ["Novosibirskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "OMS",
                    Subdivision{
                        name: "OMS",
                        country_alpha2: Alpha2::RU,
                        code: "OMS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.0554669), longitude: Some(73.3167343), max_latitude: Some(58.5741489), min_latitude: Some(53.43512399999999), max_longitude: Some(76.3037678), min_longitude: Some(70.35473710000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Omsk-oblast"), ("ar", "أوبلاست أومسك"), ("az", "Omsk vilayəti"), ("be", "Омская вобласць"), ("bg", "Омска област"), ("bn", "ওমস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Omska oblast"), ("ca", "Província d’Omsk"), ("ccp", "𑄃\u{1112e}𑄟𑄌\u{11134}𑄇\u{11134}"), ("ceb", "Omskaya Oblast’"), ("cs", "Omská oblast"), ("cy", "Oblast Omsk"), ("da", "Omsk oblast"), ("de", "Oblast Omsk"), ("el", "Περιφέρεια Ομσκ"), ("en", "Omsk"), ("es", "Omsk"), ("et", "Omski oblast"), ("eu", "Omsk oblasta"), ("fa", "استان اومسک"), ("fi", "Omskin alue"), ("fr", "Oblast d’Omsk"), ("ga", "Cúige Omsk"), ("gu", "ઓમ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אומסק"), ("hi", "ओम\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Omska oblast"), ("hu", "Omszki terület"), ("hy", "Օմսկի մարզ"), ("id", "Oblast Omsk"), ("it", "Oblast’ di Omsk"), ("ja", "オムスク州"), ("ka", "ომსკის ოლქი"), ("kk", "Омбы облысы"), ("kn", "ಓಮ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "옴스크 주"), ("ky", "Омь областы"), ("lt", "Omsko sritis"), ("lv", "Omskas apgabals"), ("mk", "Омска област"), ("mr", "ओम\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Omsk"), ("nb", "Omsk"), ("nl", "Oblast Omsk"), ("no", "Omsk"), ("pl", "Obwód omski"), ("pt", "Oblast de Omsk"), ("ro", "Regiunea Omsk"), ("ru", "Омская область"), ("si", "ඔම\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Omská oblasť"), ("sl", "Omska oblast"), ("sr", "Омска област"), ("sr_Latn", "Omska oblast"), ("sv", "Omsk oblast"), ("sw", "Omsk Oblast"), ("ta", "ஒம\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఓమస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลโอมสค\u{e4c}"), ("tr", "Omsk Oblastı"), ("uk", "Омська область"), ("ur", "اومسک اوبلاست"), ("uz", "Omsk viloyati"), ("vi", "Omsk"), ("yue", "鄂木斯克州"), ("yue_Hans", "鄂木斯克州"), ("zh", "鄂木斯克州")]),
                        unofficial_name_list: ["Omskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "ORE",
                    Subdivision{
                        name: "ORE",
                        country_alpha2: Alpha2::RU,
                        code: "ORE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.76340260000001), longitude: Some(54.6188188), max_latitude: Some(54.36583), min_latitude: Some(50.494961), max_longitude: Some(61.6907461), min_longitude: Some(50.7683845)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Orenburg-oblast"), ("ar", "أورنبرغ أوبلاست"), ("az", "Orenburq vilayəti"), ("be", "Арэнбургская вобласць"), ("bg", "Оренбургска област"), ("bn", "ওরেনব\u{9be}র\u{9cd}গ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Orenburška oblast"), ("ca", "Província d’Orenburg"), ("ccp", "𑄃\u{11127}𑄢𑄬𑄚\u{11134}𑄝𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Orenburgskaya Oblast’"), ("cs", "Orenburská oblast"), ("cy", "Oblast Orenburg"), ("da", "Orenburg oblast"), ("de", "Oblast Orenburg"), ("el", "Περιφέρεια Ορενμπούργκ"), ("en", "Orenburg"), ("es", "Óblast de Oremburgo"), ("et", "Orenburgi oblast"), ("eu", "Orenburg oblasta"), ("fa", "استان ارنبورگ"), ("fi", "Orenburgin alue"), ("fr", "Oblast d’Orenbourg"), ("ga", "Cúige Orenburg"), ("gu", "ઓર\u{ac7}નબર\u{acd}ગ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אורנבורג"), ("hi", "ओर\u{947}नब\u{942}र\u{94d}ग ओब\u{94d}लास\u{94d}त"), ("hr", "Orenburška oblast"), ("hu", "Orenburgi terület"), ("hy", "Օրենբուրգի մարզ"), ("id", "Oblast Orenburg"), ("it", "Oblast’ di Orenburg"), ("ja", "オレンブルク州"), ("ka", "ორენბურგის ოლქი"), ("kk", "Орынбор облысы"), ("kn", "ಓರ\u{cc6}ನ\u{ccd}ಬರ\u{ccd}ಗ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "오렌부르크 주"), ("ky", "Оренбург областы"), ("lt", "Orenburgo sritis"), ("lv", "Orenburgas apgabals"), ("mk", "Оренбуршка област"), ("mn", "Оренбург муж"), ("mr", "ओर\u{947}नबर\u{94d}ग ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Orenburg"), ("nb", "Orenburg"), ("nl", "Oblast Orenburg"), ("no", "Orenburg"), ("pl", "Obwód orenburski"), ("pt", "Oblast de Oremburgo"), ("ro", "Regiunea Orenburg"), ("ru", "Оренбургская область"), ("si", "ඔරෙන\u{dca}බර\u{dca}ග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Orenburská oblasť"), ("sl", "Orenburška oblast"), ("sr", "Оренбуршка област"), ("sr_Latn", "Orenburška oblast"), ("sv", "Orenburg oblast"), ("sw", "Orenburg Oblast"), ("ta", "ஒரன\u{bcd}பூர\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఓర\u{c46}\u{c46}ంబర\u{c4d}గ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "โอเรนบ\u{e38}ร\u{e4c}ก"), ("tr", "Orenburg Oblastı"), ("uk", "Оренбурзька область"), ("ur", "اورنبرگ اوبلاست"), ("uz", "Orenburg viloyati"), ("vi", "Orenburg"), ("yue", "奧倫堡州"), ("yue_Hans", "奥伦堡州"), ("zh", "奧倫堡州")]),
                        unofficial_name_list: ["Orenburgskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "ORL",
                    Subdivision{
                        name: "ORL",
                        country_alpha2: Alpha2::RU,
                        code: "ORL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.7450189), longitude: Some(36.4849627), max_latitude: Some(53.6372159), min_latitude: Some(51.9367449), max_longitude: Some(38.0644791), min_longitude: Some(34.7917679)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Orjol-oblast"), ("ar", "أوريول أوبلاست"), ("az", "Orlov vilayəti"), ("be", "Арлоўская вобласць"), ("bg", "Орловска област"), ("bn", "ওরিওল ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Orelska oblast"), ("ca", "Província d’Oriol"), ("ccp", "𑄃\u{11127}𑄢\u{11128}𑄠\u{1112e}𑄣\u{11134}"), ("ceb", "Orlovskaya Oblast’"), ("cs", "Orelská oblast"), ("cy", "Oblast Oryol"), ("da", "Orjol oblast"), ("de", "Oblast Orjol"), ("el", "Περιφέρεια Οριόλ"), ("en", "Oryol"), ("es", "Oriol"), ("et", "Orjoli oblast"), ("eu", "Oriol oblasta"), ("fa", "استان اریول"), ("fi", "Orjolin alue"), ("fr", "Oblast d’Orel"), ("ga", "Cúige Oryol"), ("gu", "ઓરીઓલ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אוריול"), ("hi", "ओर\u{94d}योल ओब\u{94d}लास\u{94d}ट"), ("hr", "Orelska oblast"), ("hu", "Orjoli terület"), ("hy", "Օրյոլի մարզ"), ("id", "Oblast Oryol"), ("it", "Oblast’ di Orël"), ("ja", "オリョール州"), ("ka", "ორიოლის ოლქი"), ("kn", "ಓರೊಲ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "오룔 주"), ("lt", "Oriolo sritis"), ("lv", "Orlas apgabals"), ("mk", "Орловска област"), ("mn", "Орёл муж"), ("mr", "ओरियोल ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Oryol"), ("nb", "Orjol"), ("nl", "Oblast Orjol"), ("no", "Orjol"), ("pl", "Obwód orłowski"), ("pt", "Oblast de Oriol"), ("ro", "Regiunea Oriol"), ("ru", "Орловская область"), ("si", "ඔර\u{dca}යෝල\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Oriolská oblasť"), ("sl", "Orjolska oblast"), ("sr", "Орелска област"), ("sr_Latn", "Orelska oblast"), ("sv", "Orjol oblast"), ("sw", "Oryol Oblast"), ("ta", "ஒர\u{bcd}யோல\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ఓర\u{c4d}య\u{c4b}ల\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ออยอล โอแบลส"), ("tr", "Oryol Oblastı"), ("uk", "Орловська область"), ("ur", "اوریول اوبلاست"), ("uz", "Oryol viloyati"), ("vi", "Oryol"), ("yue", "奧廖爾州"), ("yue_Hans", "奥廖尔州"), ("zh", "奥廖尔州")]),
                        unofficial_name_list: ["Orjol", "Orlovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "PER",
                    Subdivision{
                        name: "PER",
                        country_alpha2: Alpha2::RU,
                        code: "PER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.00000000000001), longitude: Some(56.316667), max_latitude: Some(58.176955), min_latitude: Some(57.864017), max_longitude: Some(56.65680709999999), min_longitude: Some(55.8117439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Perm-krai"), ("ar", "بيرم كراي"), ("az", "Perm diyarı"), ("be", "Пермскі край"), ("bg", "Пермски край"), ("bn", "পেরম ক\u{9cd}র\u{9be}য\u{9bc}"), ("bs", "Permski kraj"), ("ca", "Territori de Perm"), ("ccp", "𑄛𑄢\u{11134}𑄟\u{11134} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Perm Krai"), ("cs", "Permský kraj"), ("cy", "Crai Perm"), ("da", "Perm kraj"), ("de", "Region Perm"), ("el", "Κράι Περμ"), ("en", "Perm Krai"), ("es", "Perm"), ("et", "Permi krai"), ("eu", "Perm kraia"), ("fa", "سرزمین پرم"), ("fi", "Permin aluepiiri"), ("fr", "Kraï de Perm"), ("ga", "Críoch Pherm"), ("gu", "પર\u{acd}મ ક\u{acd}ર\u{ac7}ઇ"), ("he", "מחוז פרם"), ("hi", "प\u{947}र\u{94d}म क\u{94d}राय"), ("hr", "Permski kraj"), ("hu", "Permi határterület"), ("hy", "Պերմի երկրամաս"), ("id", "Krai Perm"), ("it", "Territorio di Perm’"), ("ja", "ペルミ地方"), ("ka", "პერმის მხარე"), ("kn", "ಪ\u{cc6}ರ\u{ccd}ಮ\u{ccd} ಕ\u{ccd}ರೇ"), ("ko", "페름 지방"), ("lt", "Permės kraštas"), ("lv", "Permas novads"), ("mk", "Пермски крај"), ("mr", "पर\u{94d}म क\u{94d}राय"), ("ms", "Jajahan Perm"), ("nb", "Perm"), ("nl", "Kraj Perm"), ("no", "Perm"), ("pl", "Kraj Permski"), ("pt", "Krai de Perm"), ("ro", "Ținutul Perm"), ("ru", "Пермский край"), ("si", "පර\u{dca}ම\u{dca} ක\u{dca}\u{200d}රය\u{dd2}"), ("sk", "Permský kraj"), ("sr", "Пермска Покрајина"), ("sr_Latn", "Permska Pokrajina"), ("sv", "Perm kraj"), ("sw", "Perm Krai"), ("ta", "பேர\u{bcd}ம\u{bcd} பிரதேசம\u{bcd}"), ("te", "పర\u{c4d}మ\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "เพ\u{e34}ร\u{e4c}ม ไกร"), ("tr", "Perm Krayı"), ("uk", "Пермський край"), ("ur", "پیرم کرائی"), ("uz", "Permskiy oʻlkasi"), ("vi", "Perm"), ("yue", "彼爾姆邊疆區"), ("yue_Hans", "彼尔姆边疆区"), ("zh", "彼爾姆邊疆區")]),
                        unofficial_name_list: ["Permskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "PNZ",
                    Subdivision{
                        name: "PNZ",
                        country_alpha2: Alpha2::RU,
                        code: "PNZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1412105), longitude: Some(44.0940048), max_latitude: Some(54.0281089), min_latitude: Some(52.3048359), max_longitude: Some(46.985298), min_longitude: Some(42.091744)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Penza-oblast"), ("ar", "بانزا أوبلاست"), ("az", "Penza vilayəti"), ("be", "Пензенская вобласць"), ("bg", "Пензенска област"), ("bn", "পেঞ\u{9cd}জ\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Penzjanska oblast"), ("ca", "Província de Penza"), ("ccp", "𑄛𑄬𑄚\u{11134}𑄎"), ("ceb", "Penzenskaya Oblast’"), ("cs", "Penzenská oblast"), ("cy", "Oblast Penza"), ("da", "Penza oblast"), ("de", "Oblast Pensa"), ("el", "Περιφέρεια Πένζα"), ("en", "Penza"), ("es", "Óblast de Penza"), ("et", "Penza oblast"), ("eu", "Penza oblasta"), ("fa", "استان پنزا"), ("fi", "Penzan alue"), ("fr", "oblast de Penza"), ("ga", "Cúige Phenza"), ("gu", "પ\u{ac7}ન\u{acd}ઝા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז פנזה"), ("hi", "प\u{947}न\u{94d}ज\u{93c}ा ओब\u{94d}लास\u{94d}ट"), ("hr", "Penzjanska oblast"), ("hu", "Penzai terület"), ("hy", "Պենզայի մարզ"), ("id", "Oblast Penza"), ("it", "Oblast’ di Penza"), ("ja", "ペンザ州"), ("ka", "პენზის ოლქი"), ("kn", "ಪ\u{cc6}ನ\u{ccd}ಜಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "펜자 주"), ("lt", "Penzos sritis"), ("lv", "Penzas apgabals"), ("mk", "Пензенска област"), ("mr", "प\u{947}न\u{94d}झा ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Penza"), ("nb", "Penza"), ("nl", "Oblast Penza"), ("no", "Penza"), ("pl", "Obwód penzeński"), ("pt", "Oblast de Penza"), ("ro", "Regiunea Penza"), ("ru", "Пензенская область"), ("si", "පෙන\u{dca}ස\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Penzianska oblasť"), ("sl", "Penzenska oblast"), ("sr", "Пензенска област"), ("sr_Latn", "Penzenska oblast"), ("sv", "Penza oblast"), ("sw", "Penza Oblast"), ("ta", "பென\u{bcd}ச\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ప\u{c46}ంజ\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เพนซา"), ("tr", "Penza Oblastı"), ("uk", "Пензенська область"), ("ur", "پینزا اوبلاست"), ("uz", "Penza viloyati"), ("vi", "Penza"), ("yue", "奔薩州"), ("yue_Hans", "奔萨州"), ("zh", "奔萨州")]),
                        unofficial_name_list: ["Penzenskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "PRI",
                    Subdivision{
                        name: "PRI",
                        country_alpha2: Alpha2::RU,
                        code: "PRI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0525641), longitude: Some(135.0), max_latitude: Some(48.4587049), min_latitude: Some(42.2883814), max_longitude: Some(139.021501), min_longitude: Some(130.39465)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Primorski-krai"), ("ar", "بريمورسكي كراي"), ("az", "Primorsk diyarı"), ("be", "Прыморскі край"), ("bg", "Приморски край"), ("bn", "প\u{9cd}র\u{9be}ইমিরস\u{9cd}কি ক\u{9cd}র\u{9be}ই"), ("bs", "Primorski kraj"), ("ca", "Territori de Primórie"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄟\u{1112e}𑄢\u{11134}𑄌\u{11133}𑄇\u{1112d} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Primorskiy Kray"), ("cs", "Přímořský kraj"), ("cy", "Crai Primorsky"), ("da", "Primorskij kraj"), ("de", "Primorje"), ("el", "Κράι Πριμόρσκι"), ("en", "Primorsky Krai"), ("es", "Primorie"), ("et", "Primorje krai"), ("eu", "Primorski kraia"), ("fa", "سرزمین پریمورسکی"), ("fi", "Primorjen aluepiiri"), ("fr", "Kraï du Primorie"), ("ga", "Críoch Phrimorsky"), ("gu", "પ\u{acd}રિમોર\u{acd}સ\u{acd}કી ક\u{acd}રાઇ"), ("he", "פרימוריה"), ("hi", "प\u{94d}रिमोर\u{94d}स\u{94d}की क\u{94d}राय"), ("hr", "Primorski kraj"), ("hu", "Tengermelléki határterület"), ("hy", "Պրիմորիեի երկրամաս"), ("id", "Krai Primorsky"), ("is", "Prímorja"), ("it", "Territorio del Litorale"), ("ja", "沿海地方"), ("ka", "პრიმორიე"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ಮಾರ\u{ccd}ಸ\u{ccd}ಕ\u{cbf} ಕ\u{ccd}ರೈ"), ("ko", "프리모르스키 지방"), ("ky", "Приморье крайы"), ("lt", "Primorės kraštas"), ("lv", "Piejūras novads"), ("mk", "Приморски крај"), ("mr", "प\u{94d}रिमोर\u{94d}स\u{94d}की क\u{94d}राय"), ("ms", "Jajahan Primorsky"), ("nb", "Primorskij"), ("nl", "Kraj Primorski"), ("no", "Primorskij"), ("pl", "Kraj Nadmorski"), ("pt", "Krai do Litoral"), ("ro", "Ținutul Primorie"), ("ru", "Приморский край"), ("si", "ප\u{dca}ර\u{dd2}මොර\u{dca}ස\u{dca}ක\u{dd2} ක\u{dca}\u{200d}ර\u{dcf}ය\u{dd2}"), ("sk", "Prímorský kraj"), ("sl", "Primorski kraj"), ("sr", "Приморска Покрајина"), ("sr_Latn", "Primorska Pokrajina"), ("sv", "Primorje kraj"), ("sw", "Primorsky Krai"), ("ta", "பிறிமோர\u{bcd}சுக\u{bcd}கி நிலப\u{bcd}பரப\u{bcd}பு"), ("te", "ప\u{c4d}ర\u{c48}మ\u{c4b}ర\u{c4d}స\u{c4d}క\u{c40} క\u{c4d}ర\u{c47}"), ("th", "เขตการปกครองปร\u{e34}มอร\u{e4c}สก\u{e35}"), ("tr", "Primorskiy Krayı"), ("uk", "Приморський край"), ("ur", "پریمورسکی کرائی"), ("uz", "Primorsk oʻlkasi"), ("vi", "Primorsky"), ("yue", "濱海邊疆區"), ("yue_Hans", "滨海边疆区"), ("zh", "滨海边疆区")]),
                        unofficial_name_list: ["Primorje", "Primorskij", "Primorskij Kraj"].to_vec(),
                    }
                ),
                (
                    "PSK",
                    Subdivision{
                        name: "PSK",
                        country_alpha2: Alpha2::RU,
                        code: "PSK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.7708599), longitude: Some(29.094009), max_latitude: Some(59.01885189999999), min_latitude: Some(55.5896019), max_longitude: Some(31.51626409999999), min_longitude: Some(27.323293)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pskof-oblast"), ("ar", "بسكوف أوبلاست"), ("az", "Pskov vilayəti"), ("be", "Пскоўская вобласць"), ("bg", "Псковска област"), ("bn", "পেস\u{9cd}কোভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Pskovska oblast"), ("ca", "Província de Pskov"), ("ccp", "𑄌\u{11133}𑄇\u{11127}𑄛\u{11134}"), ("ceb", "Pskovskaya Oblast’"), ("cs", "Pskovská oblast"), ("cy", "Oblast Pskov"), ("da", "Pskov oblast"), ("de", "Oblast Pskow"), ("el", "Περιφέρεια Πσκοβ"), ("en", "Pskov"), ("es", "Óblast de Pskov"), ("et", "Pihkva oblast"), ("eu", "Pskov oblasta"), ("fa", "استان پسکوف"), ("fi", "Pihkovan alue"), ("fr", "Oblast de Pskov"), ("ga", "Cúige Pskov"), ("gu", "પસ\u{acd}કોવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז פסקוב"), ("hi", "प\u{94d}सकोव ओब\u{94d}लास\u{94d}ट"), ("hr", "Pskovska oblast"), ("hu", "Pszkovi terület"), ("hy", "Պսկովի մարզ"), ("id", "Oblast Pskov"), ("it", "Oblast’ di Pskov"), ("ja", "プスコフ州"), ("ka", "ფსკოვის ოლქი"), ("kn", "ಪ\u{ccd}ಸ\u{ccd}ಕೋವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "프스코프 주"), ("lt", "Pskovo sritis"), ("lv", "Pleskavas apgabals"), ("mk", "Псковска област"), ("mr", "प\u{94d}स\u{94d}कोव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Pskov"), ("nb", "Pskov"), ("nl", "Oblast Pskov"), ("no", "Pskov"), ("pl", "Obwód pskowski"), ("pt", "Oblast de Pskov"), ("ro", "Regiunea Pskov"), ("ru", "Псковская область"), ("si", "ප\u{dca}ස\u{dca}කොව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Pskovská oblasť"), ("sl", "Pskovska oblast"), ("sr", "Псковска област"), ("sr_Latn", "Pskovska oblast"), ("sv", "Pskov oblast"), ("sw", "Pskov Oblast"), ("ta", "பிசுக\u{bcd}கோவ\u{bcd} ஒப\u{bcd}ல\u{bbe}சுது"), ("te", "ప\u{c4d}స\u{c3e}క\u{c4b}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ไซโคฟ โอเบลส"), ("tr", "Pskov Oblastı"), ("uk", "Псковська область"), ("ur", "پسکوف اوبلاست"), ("uz", "Pskov viloyati"), ("vi", "Pskov"), ("yue", "普斯科夫州"), ("yue_Hans", "普斯科夫州"), ("zh", "普斯科夫州")]),
                        unofficial_name_list: ["Pihkva", "Pleskau", "Pskovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "ROS",
                    Subdivision{
                        name: "ROS",
                        country_alpha2: Alpha2::RU,
                        code: "ROS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6853247), longitude: Some(41.8258952), max_latitude: Some(50.2123279), min_latitude: Some(45.9521199), max_longitude: Some(44.3225439), min_longitude: Some(38.2223739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rostof-oblast"), ("ar", "روستوف أوبلاست"), ("az", "Rostov vilayəti"), ("be", "Растоўская вобласць"), ("bg", "Ростовска област"), ("bn", "রসতোভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Rostovska oblast"), ("ca", "Província de Rostov"), ("ccp", "𑄢\u{1112e}𑄌\u{11134}𑄑\u{1112e}𑄛\u{11134}"), ("ceb", "Rostovskaya Oblast’"), ("cs", "Rostovská oblast"), ("cy", "Oblast Rostov"), ("da", "Rostov oblast"), ("de", "Oblast Rostow"), ("el", "Περιφέρεια Ροστόβ"), ("en", "Rostov"), ("es", "Rostov"), ("et", "Rostovi oblast"), ("eu", "Rostov oblasta"), ("fa", "استان روستوف"), ("fi", "Rostovin alue"), ("fr", "Oblast de Rostov"), ("ga", "Cúige Rostov"), ("gl", "Óblast de Rostov"), ("gu", "રૉસ\u{acd}ટોવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז רוסטוב"), ("hi", "रोस\u{94d}तोव ओब\u{94d}लास\u{94d}ट"), ("hr", "Rostovska oblast"), ("hu", "Rosztovi terület"), ("hy", "Ռոստովի մարզ"), ("id", "Oblast Rostov"), ("it", "Oblast’ di Rostov"), ("ja", "ロストフ州"), ("ka", "როსტოვის ოლქი"), ("kn", "ರೊಸ\u{ccd}ತೊವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "로스토프 주"), ("lt", "Rostovo sritis"), ("lv", "Rostovas apgabals"), ("mk", "Ростовска област"), ("mn", "Ростов муж"), ("mr", "रोस\u{94d}तोव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Rostov"), ("nb", "Rostov"), ("nl", "Oblast Rostov"), ("no", "Rostov"), ("pl", "Obwód rostowski"), ("pt", "Oblast de Rostov"), ("ro", "Regiunea Rostov"), ("ru", "Ростовская область"), ("si", "රස\u{dca}ටෝව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Rostovská oblasť"), ("sl", "Rostovska oblast"), ("sr", "Ростовска област"), ("sr_Latn", "Rostovska oblast"), ("sv", "Rostov oblast"), ("sw", "Rostov Oblast"), ("ta", "ரஸ\u{bcd}தோவ\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ర\u{c4b}స\u{c4d}ట\u{c4b}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลราสทอฟ"), ("tk", "Rostow oblasty"), ("tr", "Rostov Oblastı"), ("uk", "Ростовська область"), ("ur", "روستوف اوبلاست"), ("uz", "Rostov viloyati"), ("vi", "Rostov"), ("yue", "羅斯托夫州"), ("yue_Hans", "罗斯托夫州"), ("zh", "罗斯托夫州")]),
                        unofficial_name_list: ["Rostovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "RYA",
                    Subdivision{
                        name: "RYA",
                        country_alpha2: Alpha2::RU,
                        code: "RYA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.3875964), longitude: Some(41.259566), max_latitude: Some(55.3661119), min_latitude: Some(53.3123849), max_longitude: Some(42.694238), min_longitude: Some(38.6651351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rjazan-oblast"), ("ar", "ريازان أوبلاست"), ("az", "Ryazan vilayəti"), ("be", "Разанская вобласць"), ("bg", "Рязанска област"), ("bn", "রেয\u{9bc}\u{9be}জ\u{9be}ন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Rjazanjska oblast"), ("ca", "Província de Riazan"), ("ccp", "𑄢\u{11128}𑄠𑄎𑄚\u{11134}"), ("ceb", "Ryazanskaya Oblast’"), ("cs", "Rjazaňská oblast"), ("cy", "Oblast Ryazan"), ("da", "Rjasan oblast"), ("de", "Oblast Rjasan"), ("el", "Περιφέρεια Ριαζάν"), ("en", "Ryazan"), ("es", "Óblast de Riazán"), ("et", "Rjazani oblast"), ("eu", "Riazan oblasta"), ("fa", "استان ریازان"), ("fi", "Rjazanin alue"), ("fr", "Oblast de Riazan"), ("ga", "Cúige Ryazan"), ("gu", "રિયાઝન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ריאזאן"), ("hi", "रियाज\u{93c}ान ओब\u{94d}लास\u{94d}ट"), ("hr", "Rjazanjska oblast"), ("hu", "Rjazanyi terület"), ("hy", "Ռյազանի մարզ"), ("id", "Oblast Ryazan"), ("it", "Oblast’ di Rjazan’"), ("ja", "リャザン州"), ("ka", "რიაზანის ოლქი"), ("kn", "ರ\u{cbf}ಯಾಜಾನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "랴잔 주"), ("lt", "Riazanės sritis"), ("lv", "Rjazaņas apgabals"), ("mk", "Рјазањска област"), ("mn", "Рязань муж"), ("mr", "रायझन ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Ryazan"), ("nb", "Rjazan"), ("nl", "Oblast Rjazan"), ("no", "Rjazan"), ("pl", "Obwód riazański"), ("pt", "Oblast de Riazan"), ("ro", "Regiunea Reazan"), ("ru", "Рязанская область"), ("si", "රයස\u{dcf}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Riazanská oblasť"), ("sl", "Rjazanska oblast"), ("sr", "Рјазањска област"), ("sr_Latn", "Rjazanjska oblast"), ("sv", "Rjazan oblast"), ("sw", "Ryazan Oblast"), ("ta", "ரய\u{bbe}சன\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "రయ\u{c3e}జన\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาซ\u{e31}นดะรอน"), ("tr", "Ryazan Oblastı"), ("uk", "Рязанська область"), ("ur", "ریازان اوبلاست"), ("uz", "Ryazan viloyati"), ("vi", "Ryazan"), ("yue", "梁贊州"), ("yue_Hans", "梁赞州"), ("zh", "梁赞州")]),
                        unofficial_name_list: ["Rjazan", "Rjazanskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::RU,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(66.7613451), longitude: Some(124.1237531), max_latitude: Some(76.7581309), min_latitude: Some(55.4906601), max_longitude: Some(162.858423), min_longitude: Some(105.529348)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jakoetië"), ("am", "ሳኻ ሪፐብሊክ"), ("ar", "ياقوتيا"), ("az", "Saxa Respublikası"), ("be", "Якуція"), ("bg", "Якутия"), ("bn", "স\u{9be}খ\u{9be} রিপ\u{9be}বলিক"), ("bs", "Republika Saha"), ("ca", "Sakhà"), ("ccp", "𑄥𑄈"), ("ceb", "Respublika Sakha"), ("cs", "Sacha"), ("cy", "Gweriniaeth Sakha"), ("da", "Sakha"), ("de", "Sacha"), ("el", "Δημοκρατία των Σαχά"), ("en", "Sakha"), ("es", "Sajá"), ("et", "Sahha"), ("eu", "Yakutia"), ("fa", "یاقوتستان"), ("fi", "Sahan tasavalta"), ("fr", "République de Sakha"), ("ga", "Poblacht Shácha"), ("gl", "República de Sakha"), ("gu", "સખા રિપબ\u{acd}લિક"), ("he", "רפובליקת סאחה-יקוטיה"), ("hi", "साख\u{93c}ा गणत\u{902}त\u{94d}र"), ("hr", "Jakutska"), ("hu", "Jakutföld"), ("hy", "Սախա-Յակուտիայի հանրապետություն"), ("id", "Sakha"), ("is", "Saka"), ("it", "Sacha-Jacuzia"), ("ja", "サハ共和国"), ("ka", "სახა"), ("kk", "Саха Республикасы"), ("kn", "ಸಖ ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "사하 공화국"), ("ky", "Якутия"), ("lt", "Jakutija"), ("lv", "Saha"), ("mk", "Јакутија"), ("mn", "Саха"), ("mr", "साखा प\u{94d}रजासत\u{94d}ताक"), ("ms", "Republik Sakha"), ("nb", "Sakha"), ("nl", "Jakoetië"), ("no", "Sakha"), ("pa", "ਸਾਖਾ ਗਣਰਾਜ"), ("pl", "Jakucja"), ("pt", "Iacútia"), ("ro", "Iacuția"), ("ru", "Якутия"), ("si", "සඛ\u{dcf} ජනරජය"), ("sk", "Jakutsko"), ("sl", "Jakutija"), ("sq", "Sahaja"), ("sr", "Јакутија"), ("sr_Latn", "Jakutija"), ("sv", "Sacha"), ("sw", "Yakutia"), ("ta", "சக\u{bbe} குடியரசு"), ("te", "స\u{c3e}ఖ\u{c3e} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐซาฮา"), ("tr", "Yakutistan"), ("uk", "Республіка Саха"), ("ur", "سخا جمہوریہ"), ("uz", "Saxa"), ("vi", "Cộng hòa Sakha"), ("yue", "薩哈"), ("yue_Hans", "萨哈"), ("zh", "萨哈共和国")]),
                        unofficial_name_list: ["Jakutija", "Saha", "Sakha", "Yakutsk-Sakha"].to_vec(),
                    }
                ),
                (
                    "SAK",
                    Subdivision{
                        name: "SAK",
                        country_alpha2: Alpha2::RU,
                        code: "SAK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.9807847), longitude: Some(143.3738129), max_latitude: Some(54.416035), min_latitude: Some(43.3608536), max_longitude: Some(156.5116784), min_longitude: Some(141.1964501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sachalin-oblast"), ("ar", "ساخالين أوبلاست"), ("az", "Saxalin vilayəti"), ("be", "Сахалінская вобласць"), ("bg", "Сахалинска област"), ("bn", "শ\u{9be}খ\u{9be}লিন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Sahalinska oblast"), ("ca", "Província de Sakhalín"), ("ccp", "𑄥𑄈𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Sakhalinskaya Oblast’"), ("cs", "Sachalinská oblast"), ("cy", "Oblast Sakhalin"), ("da", "Sakhalin oblast"), ("de", "Oblast Sachalin"), ("el", "Περιφέρεια Σαχαλίνης"), ("en", "Sakhalin"), ("es", "Sajalín"), ("et", "Sahhalini oblast"), ("eu", "Sakhalin oblasta"), ("fa", "استان ساخالین"), ("fi", "Sahalinin alue"), ("fr", "oblast de Sakhaline"), ("ga", "Cúige Shakhalin"), ("gl", "Óblast de Sakhalin"), ("gu", "સાખાલિન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז סחלין"), ("hi", "सखलिन ओब\u{94d}लास\u{94d}ट"), ("hr", "Sahalinska oblast"), ("hu", "Szahalini terület"), ("hy", "Սախալինի մարզ"), ("id", "Oblast Sakhalin"), ("is", "Sakalínfylki"), ("it", "Oblast’ di Sachalin"), ("ja", "サハリン州"), ("ka", "სახალინის ოლქი"), ("kn", "ಸಖಾಲ\u{cbf}ನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "사할린 주"), ("lt", "Sachalino sritis"), ("lv", "Sahalīnas apgabals"), ("mk", "Сахалинска област"), ("mn", "Сахалин муж"), ("mr", "साखालिन ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Sakhalin"), ("nb", "Sakhalin"), ("nl", "Oblast Sachalin"), ("no", "Sakhalin"), ("pa", "ਸਾਖਾਲਿਨ ਓਬਲਾਸਤ"), ("pl", "Obwód sachaliński"), ("pt", "Oblast de Sacalina"), ("ro", "Regiunea Sahalin"), ("ru", "Сахалинская область"), ("si", "සකහල\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sachalinská oblasť"), ("sl", "Sahalinska oblast"), ("sr", "Сахалинска област"), ("sr_Latn", "Sahalinska oblast"), ("sv", "Sachalin oblast"), ("sw", "Sakhalin Oblast"), ("ta", "சக\u{bbe}லின\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "సఖ\u{c3e}ల\u{c3f}న\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ซาคาล\u{e34}น"), ("tr", "Sahalin Oblastı"), ("uk", "Сахалінська область"), ("ur", "سخالن اوبلاست"), ("uz", "Saxalin viloyati"), ("vi", "Sakhalin"), ("yue", "薩哈林州"), ("yue_Hans", "萨哈林州"), ("zh", "萨哈林州")]),
                        unofficial_name_list: ["Sahalin", "Sahalinskaya Oblast"].to_vec(),
                    }
                ),
                (
                    "SAM",
                    Subdivision{
                        name: "SAM",
                        country_alpha2: Alpha2::RU,
                        code: "SAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.41838389999999), longitude: Some(50.4725528), max_latitude: Some(54.678024), min_latitude: Some(51.773957), max_longitude: Some(52.555451), min_longitude: Some(47.9246881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Samara-oblast"), ("ar", "سمارا أوبلاست"), ("az", "Samara vilayəti"), ("be", "Самарская вобласць"), ("bg", "Самарска област"), ("bn", "স\u{9be}ম\u{9be}র\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Samarska oblast"), ("ca", "Província de Samara"), ("ccp", "𑄥𑄟𑄢"), ("ceb", "Samarskaya Oblast’"), ("cs", "Samarská oblast"), ("cy", "Oblast Samara"), ("da", "Samara oblast"), ("de", "Oblast Samara"), ("el", "Περιφέρεια Σαμάρα"), ("en", "Samara"), ("es", "Óblast de Samara"), ("et", "Samara oblast"), ("eu", "Samarako oblasta"), ("fa", "استان سامارا"), ("fi", "Samaran alue"), ("fr", "Oblast de Samara"), ("ga", "Cúige Shamara"), ("gu", "સમારા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז סמרה"), ("hi", "समारा ओब\u{94d}लास\u{94d}त"), ("hr", "Samarska oblast"), ("hu", "Szamarai terület"), ("hy", "Սամարայի մարզ"), ("id", "Oblast Samara"), ("it", "Oblast’ di Samara"), ("ja", "サマラ州"), ("ka", "სამარის ოლქი"), ("kn", "ಸಮರ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "사마라 주"), ("lt", "Samaros sritis"), ("lv", "Samaras apgabals"), ("mk", "Самарска област"), ("mr", "समारा ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Samara"), ("nb", "Samara"), ("nl", "Oblast Samara"), ("no", "Samara"), ("pl", "Obwód samarski"), ("pt", "Oblast de Samara"), ("ro", "Regiunea Samara"), ("ru", "Самарская область"), ("si", "සමර\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Samarská oblasť"), ("sl", "Samarska oblast"), ("sr", "Самарска област"), ("sr_Latn", "Samarska oblast"), ("sv", "Samara oblast"), ("sw", "Samara Oblast"), ("ta", "சம\u{bbe}ர\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "సమ\u{c3e}ర\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เม\u{e37}องซามารา"), ("tr", "Samara Oblastı"), ("uk", "Самарська область"), ("ur", "سمارا اوبلاست"), ("uz", "Samara viloyati"), ("vi", "Samara"), ("yue", "薩馬拉州"), ("yue_Hans", "萨马拉州"), ("zh", "萨马拉州")]),
                        unofficial_name_list: ["Samarskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "SAR",
                    Subdivision{
                        name: "SAR",
                        country_alpha2: Alpha2::RU,
                        code: "SAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.83692629999999), longitude: Some(46.7539397), max_latitude: Some(52.814547), min_latitude: Some(49.800981), max_longitude: Some(50.8330918), min_longitude: Some(42.5132981)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saratof-oblast"), ("ar", "ساراتوف أوبلاست"), ("az", "Saratov vilayəti"), ("be", "Саратаўская вобласць"), ("bg", "Саратовска област"), ("bn", "স\u{9be}র\u{9be}তোভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Saratovska oblast"), ("ca", "Província de Saràtov"), ("ccp", "𑄥𑄢𑄑\u{1112e}𑄛\u{11134}"), ("ceb", "Saratovskaya Oblast’"), ("cs", "Saratovská oblast"), ("cy", "Oblast Saratov"), ("da", "Saratov oblast"), ("de", "Oblast Saratow"), ("el", "Περιφέρεια Σαράτοβ"), ("en", "Saratov"), ("es", "Óblast de Sarátov"), ("et", "Saratovi oblast"), ("eu", "Saratov oblasta"), ("fa", "استان ساراتوف"), ("fi", "Saratovin alue"), ("fr", "Oblast de Saratov"), ("ga", "Cúige Sharatov"), ("gu", "સારાટોવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז סראטוב"), ("hi", "साराटोव ओब\u{94d}लास\u{94d}ट"), ("hr", "Saratovska oblast"), ("hu", "Szaratovi terület"), ("hy", "Սարատովի մարզ"), ("id", "Oblast Saratov"), ("it", "Oblast’ di Saratov"), ("ja", "サラトフ州"), ("ka", "სარატოვის ოლქი"), ("kn", "ಸಾರಾಟೊವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "사라토프 주"), ("lt", "Saratovo sritis"), ("lv", "Saratovas apgabals"), ("mk", "Саратовска област"), ("mr", "सारातोव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Saratov"), ("nb", "Saratov"), ("nl", "Oblast Saratov"), ("no", "Saratov"), ("pl", "Obwód saratowski"), ("pt", "Oblast de Saratov"), ("ro", "Regiunea Saratov"), ("ru", "Саратовская область"), ("si", "සරතොව\u{dca} ඔබ\u{dca}ල\u{dcf}ස\u{dca}ට\u{dca}"), ("sk", "Saratovská oblasť"), ("sl", "Saratovska oblast"), ("sr", "Саратовска област"), ("sr_Latn", "Saratovska oblast"), ("sv", "Saratov oblast"), ("sw", "Saratov Oblast"), ("ta", "சர\u{bbe}த\u{bcd}தவ\u{bcd} ஓப\u{bcd}லஸ\u{bcd}து"), ("te", "సర\u{c3e}ట\u{c4b}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นปกครองซาลาตอฟ"), ("tr", "Saratov Oblastı"), ("uk", "Саратовська область"), ("ur", "ساراتوو اوبلاست"), ("uz", "Saratov viloyati"), ("vi", "Saratov"), ("yue", "薩拉托夫州"), ("yue_Hans", "萨拉托夫州"), ("zh", "萨拉托夫州")]),
                        unofficial_name_list: ["Saratovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "SE",
                    Subdivision{
                        name: "SE",
                        country_alpha2: Alpha2::RU,
                        code: "SE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0451302), longitude: Some(44.2870972), max_latitude: Some(43.839755), min_latitude: Some(42.5470799), max_longitude: Some(44.956779), min_longitude: Some(43.4103983)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Ossetië-Alanië"), ("am", "ስሜን ኦሤትያ-አላኒያ"), ("ar", "أوسيتيا الشمالية-ألانيا"), ("az", "Şimali Osetiya"), ("be", "Паўночная Асеція — Аланія"), ("bg", "Северна Осетия"), ("bn", "উত\u{9cd}তর ওশেতিয\u{9bc}\u{9be}-আল\u{9be}নিয\u{9bc}\u{9be}"), ("bs", "Sjeverna Osetija-Alanija"), ("ca", "Ossètia del Nord - Alània"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄃\u{11127}𑄥𑄬𑄑\u{11128}𑄠-𑄃𑄣𑄚\u{11128}𑄠"), ("ceb", "North Ossetia"), ("cs", "Severní Osetie-Alanie"), ("cy", "Gogledd Ossetia"), ("da", "Nordossetien"), ("de", "Nordossetien-Alanien"), ("el", "Δημοκρατία της Βόρειας Οσετίας - Αλανίας"), ("en", "North Ossetia-Alania"), ("es", "Osetia del Norte - Alania"), ("et", "Põhja-Osseetia"), ("eu", "Ipar Osetia-Alania"), ("fa", "اوستیای شمالی-آلانیا"), ("fi", "Pohjois-Ossetia-Alania"), ("fr", "Ossétie-du-Nord-Alanie"), ("ga", "An Oiséit Thuaidh-An Aláin"), ("gl", "Osetia do Norte-Alania"), ("gu", "રીપબ\u{acd}લિક ઓફ નોર\u{acd}થ ઓસ\u{ac7}ટીયા-એલાનિયા"), ("he", "צפון אוסטיה - אלניה"), ("hi", "उत\u{94d}तर ओस\u{947}तिया-आलानिया"), ("hr", "Sjeverna Osetija-Alanija"), ("hu", "Észak-Oszétia"), ("hy", "Հյուսիսային Օսիա"), ("id", "Ossetia Utara-Alania"), ("is", "Norður-Ossetía"), ("it", "Ossezia Settentrionale-Alania"), ("ja", "北オセチア共和国"), ("ka", "ჩრდილოეთი ოსეთი"), ("kn", "ಉತ\u{ccd}ತರ ಒಸ\u{ccd}ಸ\u{cc6}ಟ\u{cbf}ಯಾ-ಅಲನ\u{cbf}ಯ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "세베로오세티야 공화국"), ("lt", "Šiaurės Osetija"), ("lv", "Ziemeļosetija-Alānija"), ("mk", "Северна Осетија-Аланија"), ("mn", "Хойд Осет-Алани"), ("mr", "उत\u{94d}तर ओस\u{947}शिया-अलानिया"), ("ms", "Ossetia Utara-Alania"), ("nb", "Nord-Ossetia"), ("nl", "Noord-Ossetië"), ("no", "Nord-Ossetia"), ("pl", "Osetia Północna"), ("pt", "Ossétia do Norte-Alânia"), ("ro", "Osetia de Nord"), ("ru", "Республика Северная Осетия-Алания"), ("si", "උත\u{dd4}ර\u{dd4} ඔසේට\u{dd2}ය\u{dcf}-ඇලන\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Severné Osetsko"), ("sl", "Severna Osetija-Alanija"), ("sq", "Osetia e Veriut-Alania"), ("sr", "Северна Осетија — Аланија"), ("sr_Latn", "Severna Osetija — Alanija"), ("sv", "Nordossetien"), ("sw", "Kaskazi Ossetia-Alania"), ("ta", "வடக\u{bcd}கு ஒசேத\u{bcd}திய-அலன\u{bc0}ய\u{bbe}"), ("te", "ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d} న\u{c3e}ర\u{c4d}త\u{c4d} ఒస\u{c47}ట\u{c3f}య\u{c3e}-అల\u{c47}న\u{c3f}య\u{c3e}"), ("th", "โอเซเท\u{e35}ย อลาเน\u{e35}ย เหน\u{e37}อ"), ("tr", "Kuzey Osetya-Alanya"), ("uk", "Північна Осетія"), ("ur", "شمالی اوسیشیا-الانیا"), ("uz", "Shimoliy Osetiya Alaniya"), ("vi", "Bắc Osetiya-Alaniya"), ("yue", "北奧塞梯-阿蘭"), ("yue_Hans", "北奥塞梯-阿兰"), ("zh", "北奥塞梯-阿兰共和国")]),
                        unofficial_name_list: ["Alania", "Alanija", "North Ossetian Republic", "Osetija"].to_vec(),
                    }
                ),
                (
                    "SMO",
                    Subdivision{
                        name: "SMO",
                        country_alpha2: Alpha2::RU,
                        code: "SMO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.9882994), longitude: Some(32.6677378), max_latitude: Some(56.07094379999999), min_latitude: Some(53.414513), max_longitude: Some(35.3920447), min_longitude: Some(30.7486752)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Smolensk-oblast"), ("ar", "أوبلاست سمولينسك"), ("az", "Smolensk vilayəti"), ("be", "Смаленская вобласць"), ("bg", "Смоленска област"), ("bn", "সি\u{9cd}মো স\u{9cd}মোলেনস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Smolenska oblast"), ("ca", "Província de Smolensk"), ("ccp", "𑄌\u{11133}𑄟\u{11127}𑄣𑄬𑄚\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Smolenskaya Oblast’"), ("cs", "Smolenská oblast"), ("cy", "Oblast Smolensk"), ("da", "Smolensk oblast"), ("de", "Oblast Smolensk"), ("el", "Περιφέρεια Σμολένσκ"), ("en", "Smolensk"), ("es", "Smolensk"), ("et", "Smolenski oblast"), ("eu", "Smolensk oblasta"), ("fa", "استان اسمولنسک"), ("fi", "Smolenskin alue"), ("fr", "Oblast de Smolensk"), ("ga", "Cúige Smolensk"), ("gu", "સ\u{acd}મોલ\u{ac7}ન\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "סמולנסק"), ("hi", "स\u{94d}मोल\u{947}\u{902}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Smolenska oblast"), ("hu", "Szmolenszki terület"), ("hy", "Սմոլենսկի մարզ"), ("id", "Oblast Smolensk"), ("it", "Oblast’ di Smolensk"), ("ja", "スモレンスク州"), ("ka", "სმოლენსკის ოლქი"), ("kn", "ಸ\u{ccd}ಮೊಲ\u{cc6}ನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "스몰렌스크 주"), ("lt", "Smolensko sritis"), ("lv", "Smoļenskas apgabals"), ("mk", "Смоленска област"), ("mr", "स\u{94d}मोल\u{947}न\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Smolensk"), ("nb", "Smolensk"), ("nl", "Oblast Smolensk"), ("no", "Smolensk"), ("pl", "Obwód smoleński"), ("pt", "Oblast de Smolensk"), ("ro", "Regiunea Smolensk"), ("ru", "Смоленская область"), ("si", "ස\u{dca}මොලේන\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Smolenská oblasť"), ("sl", "Smolenska oblast"), ("sr", "Смоленска област"), ("sr_Latn", "Smolenska oblast"), ("sv", "Smolensk oblast"), ("sw", "Smolensk Oblast"), ("ta", "ஸ\u{bcd}மொலென\u{bcd}ஸ\u{bcd}க\u{bcd} ஒபில\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "స\u{c4d}మ\u{c4b}ల\u{c46}న\u{c4d}స\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลสมอเลนสค\u{e4c}"), ("tr", "Smolensk Oblastı"), ("uk", "Смоленська область"), ("ur", "سمولنسک اوبلاست"), ("uz", "Smolensk viloyati"), ("vi", "Smolensk"), ("yue", "斯摩棱斯克州"), ("yue_Hans", "斯摩棱斯克州"), ("zh", "斯摩棱斯克州")]),
                        unofficial_name_list: ["Smolenskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "SPE",
                    Subdivision{
                        name: "SPE",
                        country_alpha2: Alpha2::RU,
                        code: "SPE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.9342802), longitude: Some(30.3350986), max_latitude: Some(60.089675), min_latitude: Some(59.74521590000001), max_longitude: Some(30.559783), min_longitude: Some(30.090332)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sint Petersburg"), ("am", "ሳንክት ፔቴርቡርግ"), ("ar", "سانت بطرسبرغ"), ("az", "Sankt-Peterburq"), ("be", "Санкт-Пецярбург"), ("bg", "Санкт Петербург"), ("bn", "সেন\u{9cd}ট পিট\u{9be}র\u{9cd}সব\u{9be}র\u{9cd}গ"), ("bs", "Sankt Peterburg"), ("ca", "Sant Petersburg"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄛\u{11128}𑄑𑄢\u{11134}𑄝𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Saint Petersburg"), ("cs", "Petrohrad"), ("cy", "St Petersburg"), ("da", "Sankt Petersborg"), ("de", "Sankt Petersburg"), ("el", "Αγία Πετρούπολη"), ("en", "Saint Petersburg"), ("es", "San Petersburgo"), ("et", "Peterburi"), ("eu", "San Petersburgo"), ("fa", "سن پترزبورگ"), ("fi", "Pietari"), ("fr", "Saint-Pétersbourg"), ("ga", "Cathair Pheadair"), ("gl", "San Petersburgo"), ("gu", "સ\u{ac7}ન\u{acd}ટ પીટર\u{acd}સબર\u{acd}ગ"), ("ha", "Saint-Petersburg"), ("ha_NE", "Saint-Petersburg"), ("he", "סנקט פטרבורג"), ("hi", "स\u{947}\u{902}ट पीटर\u{94d}सबर\u{94d}ग"), ("hr", "Sankt Peterburg"), ("hu", "Szentpétervár"), ("hy", "Սանկտ Պետերբուրգ"), ("id", "St. Petersburg"), ("is", "Sankti Pétursborg"), ("it", "San Pietroburgo"), ("ja", "サンクトペテルブルク"), ("jv", "St. Petersburg"), ("ka", "სანქტ-პეტერბურგი"), ("kk", "Санкт-Петербург"), ("kn", "ಸೇಂಟ\u{ccd} ಪೀಟರ\u{ccd}ಸ\u{ccd}\u{200c}ಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "상트페테르부르크"), ("ky", "Санкт-Петербург"), ("lt", "Sankt Peterburgas"), ("lv", "Sanktpēterburga"), ("mk", "Санкт Петербург"), ("ml", "സെന\u{d4d}റ\u{d4d} പീറ\u{d4d}റേഴ\u{d4d}സ\u{d4d}ബർഗ\u{d4d}"), ("mn", "Санкт-Петербург"), ("mr", "स\u{947}\u{902}ट पीटर\u{94d}सबर\u{94d}ग"), ("ms", "Saint Petersburg"), ("my", "စ\u{102d}န\u{1037}\u{103a}ပ\u{102e}တာစဘတ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "St. Petersburg"), ("ne", "स\u{947}न\u{94d}ट पिटर\u{94d}सवर\u{94d}ग"), ("nl", "Sint-Petersburg"), ("no", "St. Petersburg"), ("pa", "ਸ\u{a47}\u{a02}ਟ ਪੀਟਰਸਬਰਗ"), ("pl", "Petersburg"), ("pt", "São Petersburgo"), ("ro", "Sankt Petersburg"), ("ru", "Санкт-Петербург"), ("si", "ස\u{dcf}න\u{dca}ත ප\u{dd3}ටර\u{dca}ස\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Petrohrad"), ("sl", "Sankt Peterburg"), ("so", "Saint Petersburg"), ("sq", "Shën Petersburgu"), ("sr", "Санкт Петербург"), ("sr_Latn", "Sankt Peterburg"), ("sv", "Sankt Petersburg"), ("sw", "Sankt Peterburg"), ("ta", "சென\u{bcd} ப\u{bc0}ட\u{bcd}டர\u{bcd}ஸ\u{bcd}பேர\u{bcd}க\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ప\u{c40}టర\u{c4d}స\u{c4d}\u{200c}బర\u{c4d}గ\u{c4d}"), ("th", "เซนต\u{e4c}ป\u{e35}เตอร\u{e4c}สเบ\u{e34}ร\u{e4c}ก"), ("tk", "Sankt-Peterburg"), ("tr", "Sankt Petersburg"), ("uk", "Санкт-Петербург"), ("ur", "سینٹ پیٹرز برگ"), ("uz", "Sankt-Peterburg"), ("vi", "Sankt-Peterburg"), ("yo", "Saint Petersburg"), ("yo_BJ", "Saint Petersburg"), ("yue", "聖彼得堡"), ("yue_Hans", "圣彼得堡"), ("zh", "圣彼得堡")]),
                        unofficial_name_list: ["Saint-Pétersbourg", "San Petersburgo", "San Pietroburgo", "Sankt Petersburg", "Sankt-Peterburg"].to_vec(),
                    }
                ),
                (
                    "STA",
                    Subdivision{
                        name: "STA",
                        country_alpha2: Alpha2::RU,
                        code: "STA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6680993), longitude: Some(43.520214), max_latitude: Some(46.2299116), min_latitude: Some(43.658068), max_longitude: Some(45.7189749), min_longitude: Some(40.8430621)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Stawropol-krai"), ("ar", "ستافروبول كراي"), ("az", "Stavropol diyarı"), ("be", "Стаўрапольскі край"), ("bg", "Ставрополски край"), ("bn", "স\u{9cd}ট\u{9cd}য\u{9be}ভরোপোল ক\u{9cd}র\u{9be}ই"), ("bs", "Stavropoljski kraj"), ("ca", "Territori de Stàvropol"), ("ccp", "𑄌\u{11133}𑄑𑄛\u{11134}𑄢\u{1112e}𑄛\u{1112e}𑄣\u{11134} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Stavropol’skiy Kray"), ("cs", "Stavropolský kraj"), ("cy", "Crai Stavropol"), ("da", "Stavropol kraj"), ("de", "Region Stawropol"), ("el", "Κράι Σταυρούπολης"), ("en", "Stavropol Krai"), ("es", "Stávropol"), ("et", "Stavropoli krai"), ("eu", "Stavropol kraia"), ("fa", "سرزمین استاوروپول"), ("fi", "Stavropolin aluepiiri"), ("fr", "Kraï de Stavropol"), ("ga", "Críoch Stavropol"), ("gl", "Krai de Stavropol"), ("he", "מחוז סטברופול"), ("hi", "स\u{94d}ताव\u{94d}रोपोल क\u{94d}राय"), ("hr", "Stavropoljski kraj"), ("hu", "Sztavropoli határterület"), ("hy", "Ստավրոպոլի երկրամաս"), ("id", "Krai Stavropol"), ("it", "Territorio di Stavropol’"), ("ja", "スタヴロポリ地方"), ("ka", "სტავროპოლის მხარე"), ("kn", "ಸ\u{ccd}ಟಾವ\u{ccd}ರೋಪೋಲ\u{ccd} ಕ\u{ccd}ರೇ"), ("ko", "스타브로폴 지방"), ("lt", "Stavropolio kraštas"), ("lv", "Stavropoles novads"), ("mk", "Ставрополски крај"), ("mn", "Ставрополийн хязгаар"), ("mr", "स\u{94d}ताव\u{94d}रोपोल क\u{94d}राय"), ("ms", "Jajahan Stavropol"), ("nb", "Stavropol"), ("nl", "Kraj Stavropol"), ("no", "Stavropol"), ("pl", "Kraj Stawropolski"), ("pt", "Krai de Stavropol"), ("ro", "Ținutul Stavropol"), ("ru", "Ставропольский край"), ("si", "ස\u{dca}ටව\u{dca}රෝපොල\u{dca} ක\u{dca}\u{200d}ර\u{dcf}ය\u{dd2}"), ("sk", "Stavropoľský kraj"), ("sr", "Ставропољска Покрајина"), ("sr_Latn", "Stavropoljska Pokrajina"), ("sv", "Stavropol kraj"), ("sw", "Stavropol Krai"), ("ta", "இசுத\u{bbe}வ\u{bcd}ரபோல\u{bcd} நிலப\u{bcd}பரப\u{bcd}பு"), ("te", "స\u{c4d}ట\u{c3e}వర\u{c4b}ప\u{c4b}ల\u{c4d} క\u{c4d}ర\u{c47}"), ("th", "เขตสต\u{e31}ฟโรปอล"), ("tr", "Stavropol Krayı"), ("uk", "Ставропольський край"), ("ur", "سٹاوروپول کرائی"), ("uz", "Stavropol oʻlkasi"), ("vi", "Stavropol"), ("yue", "斯塔夫羅波爾邊疆區"), ("yue_Hans", "斯塔夫罗波尔边疆区"), ("zh", "斯塔夫罗波尔边疆区")]),
                        unofficial_name_list: ["Stavropolskij Kraj"].to_vec(),
                    }
                ),
                (
                    "SVE",
                    Subdivision{
                        name: "SVE",
                        country_alpha2: Alpha2::RU,
                        code: "SVE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.007735), longitude: Some(61.9316226), max_latitude: Some(61.94590299999999), min_latitude: Some(56.0562339), max_longitude: Some(66.178652), min_longitude: Some(57.2360151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Swerdlofsk-oblast"), ("ar", "أوبلاست سفردلوفسك"), ("az", "Sverdlovsk vilayəti"), ("be", "Свярдлоўская вобласць"), ("bg", "Свердловска област"), ("bn", "ভ\u{9be}র\u{9cd}দলোভস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Sverdlovska oblast"), ("ca", "Província de Sverdlovsk"), ("ccp", "𑄌\u{11133}𑄞𑄬𑄢\u{11134}𑄓\u{11134}𑄣\u{1112e}𑄛\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Sverdlovskaya Oblast’"), ("cs", "Sverdlovská oblast"), ("cy", "Oblast Sverdlovsk"), ("da", "Sverdlovsk oblast"), ("de", "Oblast Swerdlowsk"), ("el", "Περιφέρεια Σβερντλόβσκ"), ("en", "Sverdlovsk"), ("es", "Sverdlovsk"), ("et", "Sverdlovski oblast"), ("eu", "Sverdlovsk oblasta"), ("fa", "استان سوردلوفسک"), ("fi", "Sverdlovskin alue"), ("fr", "Oblast de Sverdlovsk"), ("ga", "Cúige Sverdlovsk"), ("gu", "સ\u{acd}વ\u{ac7}ર\u{acd}ડલોવ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז סברדלובסק"), ("hi", "स\u{94d}वर\u{94d}दलोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Sverdlovska oblast"), ("hu", "Szverdlovszki terület"), ("hy", "Սվերդլովսկի մարզ"), ("id", "Oblast Sverdlovsk"), ("it", "Oblast’ di Sverdlovsk"), ("ja", "スヴェルドロフスク州"), ("ka", "სვერდლოვსკის ოლქი"), ("kk", "Свердлов облысы"), ("kn", "ಸ\u{ccd}ವ\u{cc6}ರ\u{ccd}ಡ\u{ccd}ಲೋವ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "스베르들롭스크 주"), ("lt", "Sverdlovsko sritis"), ("lv", "Sverdlovskas apgabals"), ("mk", "Свердловска област"), ("mr", "स\u{94d}व\u{947}र\u{94d}दलोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Sverdlovsk"), ("nb", "Sverdlovsk"), ("nl", "Oblast Sverdlovsk"), ("no", "Sverdlovsk"), ("pl", "Obwód swierdłowski"), ("pt", "Oblast de Sverdlovsk"), ("ro", "Regiunea Sverdlovsk"), ("ru", "Свердловская область"), ("si", "ස\u{dca}වෙර\u{dca}ද\u{dca}ලොව\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sverdlovská oblasť"), ("sl", "Sverdlovska oblast"), ("sr", "Свердловска област"), ("sr_Latn", "Sverdlovska oblast"), ("sv", "Sverdlovsk oblast"), ("sw", "Sverdlovsk Oblast"), ("ta", "ஸ\u{bcd}வெர\u{bcd}த\u{bcd}லோவ\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "స\u{c4d}వ\u{c46}ర\u{c4d}డ\u{c4d}\u{200c}ల\u{c4b}వస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "สเวอรลอฟ โอแบลส"), ("tr", "Sverdlovsk Oblastı"), ("uk", "Свердловська область"), ("ur", "سوردلووسک اوبلاست"), ("uz", "Sverdlovsk viloyati"), ("vi", "Sverdlovsk"), ("yue", "斯維爾德洛夫斯克州"), ("yue_Hans", "斯维尔德洛夫斯克州"), ("zh", "斯維爾德洛夫斯克州")]),
                        unofficial_name_list: ["Sverdlovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::RU,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.1802364), longitude: Some(50.7263945), max_latitude: Some(56.6772159), min_latitude: Some(53.976026), max_longitude: Some(54.2602891), min_longitude: Some(47.2586476)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tartarstan"), ("am", "ታታርስታን"), ("ar", "تتارستان"), ("az", "Tatarıstan"), ("be", "Татарстан"), ("bg", "Татарстан"), ("bn", "ত\u{9be}ত\u{9be}রস\u{9cd}ত\u{9be}ন"), ("bs", "Tatarstan"), ("ca", "Tatarstan"), ("ccp", "𑄑𑄑𑄢\u{11134}𑄌\u{11133}𑄑𑄚\u{11134}"), ("ceb", "Tatarstan"), ("cs", "Tatarstán"), ("da", "Tatarstan"), ("de", "Tatarstan"), ("el", "Δημοκρατία του Ταταρστάν"), ("en", "Tatarstan"), ("es", "Tartaristán"), ("et", "Tatarstan"), ("eu", "Tatarstan"), ("fa", "تاتارستان"), ("fi", "Tatarstan"), ("fr", "Tatarstan"), ("ga", "An Tatarstáin"), ("gl", "Tartaristán"), ("gu", "રિપબ\u{acd}લિક ઓફ તાતરસ\u{acd}તાન"), ("he", "טטרסטן"), ("hi", "तातारस\u{94d}तान"), ("hr", "Tatarstan"), ("hu", "Tatárföld"), ("hy", "Թաթարստան"), ("id", "Tatarstan"), ("is", "Tatarstan"), ("it", "Tatarstan"), ("ja", "タタールスタン共和国"), ("ka", "თათრეთი"), ("kk", "Татарстан"), ("kn", "ಟಾಟರ\u{ccd}ಸ\u{ccd}ತಾನ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "타타르 공화국"), ("ky", "Татарстан"), ("lt", "Tatarstanas"), ("lv", "Tatarstāna"), ("mk", "Татарстан"), ("ml", "ട\u{d3e}ട\u{d4d}ടർസ\u{d4d}ത\u{d3e}ൻ"), ("mn", "Татарстан"), ("mr", "तातरस\u{94d}तान"), ("ms", "Tatarstan"), ("nb", "Tatarstan"), ("nl", "Tatarije"), ("no", "Tatarstan"), ("pl", "Tatarstan"), ("pt", "Tartaristão"), ("ro", "Tatarstan"), ("ru", "Татарстан"), ("si", "ටට\u{dcf}ර\u{dca}ස\u{dca}ථ\u{dcf}න\u{dca}"), ("sk", "Tatársko"), ("sl", "Tatarstan"), ("sq", "Tataristani"), ("sr", "Татарстан"), ("sr_Latn", "Tatarstan"), ("sv", "Tatarstan"), ("sw", "Tatarstan"), ("ta", "தர\u{bcd}த\u{bbe}ரிஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "ట\u{c3e}టర\u{c4d} స\u{c4d}ట\u{c3e}న\u{c4d}"), ("th", "สาธารณร\u{e31}ฐตาตาร\u{e4c}สถาน"), ("tk", "Tatarystan"), ("tr", "Tataristan"), ("uk", "Татарстан"), ("ur", "تاتارستان"), ("uz", "Tatariston"), ("vi", "Tatarstan"), ("yue", "韃靼斯坦共和國"), ("yue_Hans", "鞑靼斯坦共和国"), ("zh", "鞑靼斯坦共和国")]),
                        unofficial_name_list: ["Tatarstan, Respublika"].to_vec(),
                    }
                ),
                (
                    "TAM",
                    Subdivision{
                        name: "TAM",
                        country_alpha2: Alpha2::RU,
                        code: "TAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6416589), longitude: Some(41.4216451), max_latitude: Some(53.8228889), min_latitude: Some(51.589638), max_longitude: Some(43.244815), min_longitude: Some(39.9170968)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tambof-oblast"), ("ar", "تامبوف أوبلاست"), ("az", "Tambov vilayəti"), ("be", "Тамбоўская вобласць"), ("bg", "Тамбовска област"), ("bn", "ত\u{9be}ম\u{9cd}বোভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Tambovska oblast"), ("ca", "Província de Tambov"), ("ccp", "𑄑𑄟\u{11134}𑄝\u{1112e}𑄛\u{11134}"), ("ceb", "Tambovskaya Oblast’"), ("cs", "Tambovská oblast"), ("cy", "Oblast Tambov"), ("da", "Tambov oblast"), ("de", "Oblast Tambow"), ("el", "Περιφέρεια Ταμπόβ"), ("en", "Tambov"), ("es", "Tambov"), ("et", "Tambovi oblast"), ("eu", "Tambov oblasta"), ("fa", "استان تامبوف"), ("fi", "Tambovin alue"), ("fr", "oblast de Tambov"), ("ga", "Cúige Thambov"), ("gu", "ત\u{ac7}મ\u{acd}બો ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז טמבוב"), ("hi", "ता\u{902}बोव ओब\u{94d}लास\u{94d}ट"), ("hr", "Tambovska oblast"), ("hu", "Tambovi terület"), ("hy", "Տամբովի մարզ"), ("id", "Oblast Tambov"), ("it", "Oblast’ di Tambov"), ("ja", "タンボフ州"), ("ka", "ტამბოვის ოლქი"), ("kn", "ಟಾಂಬೊವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "탐보프 주"), ("lt", "Tambovo sritis"), ("lv", "Tambovas apgabals"), ("mk", "Тамбовска област"), ("mr", "ता\u{902}बोव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Tambov"), ("nb", "Tambov"), ("nl", "Oblast Tambov"), ("no", "Tambov"), ("pl", "Obwód tambowski"), ("pt", "Oblast de Tambov"), ("ro", "Regiunea Tambov"), ("ru", "Тамбовская область"), ("si", "ටම\u{dca}බොව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tambovská oblasť"), ("sl", "Tambovska oblast"), ("sr", "Тамбовска област"), ("sr_Latn", "Tambovska oblast"), ("sv", "Tambov oblast"), ("sw", "Tambov Oblast"), ("ta", "தம\u{bcd}போவ\u{bcd} ஓப\u{bcd}லஸ\u{bcd}து"), ("te", "ట\u{c3e}ంబ\u{c4b}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ท\u{e31}มโบว โอแบลส"), ("tr", "Tambov Oblastı"), ("uk", "Тамбовська область"), ("ur", "تیمبوف اوبلاست"), ("uz", "Tambov viloyati"), ("vi", "Tambov"), ("yue", "坦波夫州"), ("yue_Hans", "坦波夫州"), ("zh", "坦波夫州")]),
                        unofficial_name_list: ["Tambovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "TOM",
                    Subdivision{
                        name: "TOM",
                        country_alpha2: Alpha2::RU,
                        code: "TOM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.8969882), longitude: Some(82.67654999999999), max_latitude: Some(61.0335298), min_latitude: Some(55.66864), max_longitude: Some(89.37532709999999), min_longitude: Some(75.0574591)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tomsk-oblast"), ("ar", "تومسك أوبلاست"), ("az", "Tomsk vilayəti"), ("be", "Томская вобласць"), ("bg", "Томска област"), ("bn", "টোমস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Tomska oblast"), ("ca", "Província de Tomsk"), ("ccp", "𑄑\u{11127}𑄟\u{11133}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Tomskaya Oblast’"), ("cs", "Tomská oblast"), ("cy", "Oblast Tomsk"), ("da", "Tomsk oblast"), ("de", "Oblast Tomsk"), ("el", "Περιφέρεια Τομσκ"), ("en", "Tomsk"), ("es", "Tomsk"), ("et", "Tomski oblast"), ("eu", "Tomsk oblasta"), ("fa", "استان تومسک"), ("fi", "Tomskin alue"), ("fr", "Oblast de Tomsk"), ("ga", "Cúige Thomsk"), ("gu", "ટોમ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז טומסק"), ("hi", "टॉमस\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Tomska oblast"), ("hu", "Tomszki terület"), ("hy", "Տոմսկի մարզ"), ("id", "Oblast Tomsk"), ("it", "Oblast’ di Tomsk"), ("ja", "トムスク州"), ("ka", "ტომსკის ოლქი"), ("kn", "ಟಾಮ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "톰스크 주"), ("lt", "Tomsko sritis"), ("lv", "Tomskas apgabals"), ("mk", "Томска област"), ("mr", "तोम\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Tomsk"), ("nb", "Tomsk"), ("nl", "Oblast Tomsk"), ("no", "Tomsk"), ("pl", "Obwód tomski"), ("pt", "Oblast de Tomsk"), ("ro", "Regiunea Tomsk"), ("ru", "Томская область"), ("si", "ටෝම\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tomská oblasť"), ("sl", "Tomska oblast"), ("sr", "Томска област"), ("sr_Latn", "Tomska oblast"), ("sv", "Tomsk oblast"), ("sw", "Tomsk Oblast"), ("ta", "தோம\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ట\u{c3e}మస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ทอมส\u{e4c}ค โอแบลส"), ("tr", "Tomsk Oblastı"), ("uk", "Томська область"), ("ur", "تومسک اوبلاست"), ("uz", "Tomsk viloyati"), ("vi", "Tomsk"), ("yue", "托木斯克州"), ("yue_Hans", "托木斯克州"), ("zh", "托木斯克州")]),
                        unofficial_name_list: ["Tomskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "TUL",
                    Subdivision{
                        name: "TUL",
                        country_alpha2: Alpha2::RU,
                        code: "TUL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.163768), longitude: Some(37.5649507), max_latitude: Some(54.8505721), min_latitude: Some(52.9556001), max_longitude: Some(38.952968), min_longitude: Some(35.8963269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Toela-oblast"), ("ar", "تولا أوبلاست"), ("az", "Tula vilayəti"), ("be", "Тульская вобласць"), ("bg", "Тулска област"), ("bn", "ত\u{9c1}ল\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Tulska oblast"), ("ca", "Província de Tula"), ("ccp", "𑄑\u{1112a}𑄣"), ("ceb", "Tul’skaya Oblast’"), ("cs", "Tulská oblast"), ("cy", "Oblast Tula"), ("da", "Tula oblast"), ("de", "Oblast Tula"), ("el", "Περιφέρεια Τούλα"), ("en", "Tula"), ("es", "Óblast de Tula"), ("et", "Tula oblast"), ("eu", "Tula oblasta"), ("fa", "استان تولا"), ("fi", "Tulan alue"), ("fr", "Oblast de Toula"), ("ga", "Cúige Thula"), ("gu", "ત\u{ac1}લા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "טולה"), ("hi", "ट\u{942}ला ओब\u{94d}लास\u{94d}ट"), ("hr", "Tulska oblast"), ("hu", "Tulai terület"), ("hy", "Տուլայի մարզ"), ("id", "Oblast Tula"), ("it", "Oblast’ di Tula"), ("ja", "トゥーラ州"), ("ka", "ტულის ოლქი"), ("kn", "ತುಲಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "툴라 주"), ("lt", "Tulos sritis"), ("lv", "Tulas apgabals"), ("mk", "Тулска област"), ("mn", "Тула муж"), ("mr", "त\u{941}ला ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Tula"), ("nb", "Tula"), ("nl", "Oblast Toela"), ("no", "Tula"), ("pl", "Obwód tulski"), ("pt", "Oblast de Tula"), ("ro", "Regiunea Tula"), ("ru", "Тульская область"), ("si", "ට\u{dd4}ල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tuľská oblasť"), ("sl", "Tulska oblast"), ("sr", "Тулска област"), ("sr_Latn", "Tulska oblast"), ("sv", "Tula oblast"), ("sw", "Tula Oblast"), ("ta", "தூல\u{bbe} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "టూల\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลต\u{e39}ลา"), ("tr", "Tula Oblastı"), ("uk", "Тульська область"), ("ur", "تولا اوبلاست"), ("uz", "Tula viloyati"), ("vi", "Tula"), ("yue", "圖拉州"), ("yue_Hans", "图拉州"), ("zh", "图拉州")]),
                        unofficial_name_list: ["Tulskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "TVE",
                    Subdivision{
                        name: "TVE",
                        country_alpha2: Alpha2::RU,
                        code: "TVE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.0021654), longitude: Some(33.9853142), max_latitude: Some(58.8721109), min_latitude: Some(55.6322698), max_longitude: Some(38.31839), min_longitude: Some(30.7768341)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Twer-oblast"), ("ar", "تفير أوبلاست"), ("az", "Tver vilayəti"), ("be", "Цвярская вобласць"), ("bg", "Тверска област"), ("bn", "টেভ\u{9be}র ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Tverska oblast"), ("ca", "Província de Tver"), ("ccp", "𑄞𑄬𑄢\u{11134}"), ("ceb", "Tverskaya Oblast’"), ("cs", "Tverská oblast"), ("cy", "Oblast Tver"), ("da", "Tver oblast"), ("de", "Oblast Twer"), ("el", "Περιφέρεια Τβερ"), ("en", "Tver"), ("es", "Óblast de Tver"), ("et", "Tveri oblast"), ("eu", "Tver oblasta"), ("fa", "استان تور"), ("fi", "Tverin alue"), ("fr", "Oblast de Tver"), ("ga", "Cúige Tver"), ("gu", "ટવર ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז טבר"), ("hi", "त\u{94d}व\u{947}र ओब\u{94d}लास\u{94d}ट"), ("hr", "Tverska oblast"), ("hu", "Tveri terület"), ("hy", "Տվերի մարզ"), ("id", "Oblast Tver"), ("it", "Oblast’ di Tver’"), ("ja", "トヴェリ州"), ("ka", "ტვერის ოლქი"), ("kn", "ಟ\u{ccd}ವ\u{cc6}ರ\u{ccd}ಸ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "트베리 주"), ("lt", "Tverės sritis"), ("lv", "Tveras apgabals"), ("mk", "Тверска област"), ("mn", "Тверь муж"), ("mr", "त\u{94d}व\u{947}र ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Tver"), ("nb", "Tver"), ("nl", "Oblast Tver"), ("no", "Tver"), ("pl", "Obwód twerski"), ("pt", "Oblast de Tver"), ("ro", "Regiunea Tver"), ("ru", "Тверская область"), ("si", "ට\u{dd2}වර\u{dca}-ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tverská oblasť"), ("sl", "Tverska oblast"), ("sr", "Тверска област"), ("sr_Latn", "Tverska oblast"), ("sv", "Tver oblast"), ("sw", "Tver Oblast"), ("ta", "வெர\u{bcd} ஒபில\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ట\u{c4d}వర\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดจ\u{e31}มบ\u{e35}"), ("tr", "Tver Oblastı"), ("uk", "Тверська область"), ("ur", "توور اوبلاست"), ("uz", "Tver viloyati"), ("vi", "Tver"), ("yue", "特維爾州"), ("yue_Hans", "特维尔州"), ("zh", "特维尔州")]),
                        unofficial_name_list: ["Tverskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "TY",
                    Subdivision{
                        name: "TY",
                        country_alpha2: Alpha2::RU,
                        code: "TY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8872669), longitude: Some(95.62601719999999), max_latitude: Some(53.727431), min_latitude: Some(49.7417159), max_longitude: Some(99.269666), min_longitude: Some(88.7985341)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tiwa"), ("am", "ቱቫ"), ("ar", "توفا"), ("az", "Tıva"), ("be", "Тува"), ("bg", "Тува"), ("bn", "ত\u{9c1}ভ\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Tuva"), ("ca", "Tuvà"), ("ccp", "𑄑\u{1112a}𑄞"), ("ceb", "Respublika Tyva"), ("cs", "Tuva"), ("da", "Tuva"), ("de", "Tuwa"), ("el", "Δημοκρατία της Τιβά"), ("en", "Tuva"), ("es", "Tuvá"), ("et", "Tõva"), ("eu", "Tuva"), ("fa", "تووا"), ("fi", "Tuva"), ("fr", "Touva"), ("ga", "Túva"), ("gl", "Tuva"), ("gu", "ત\u{ac1}વા રિપબ\u{acd}લિક"), ("he", "טובה"), ("hi", "त\u{942}वा"), ("hr", "Tuva"), ("hu", "Tuva"), ("hy", "Տիվա"), ("id", "Tuva"), ("it", "Tuva"), ("ja", "トゥヴァ共和国"), ("ka", "ტუვა"), ("kk", "Тыуа"), ("kn", "ತುವಾ ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "투바 공화국"), ("ky", "Тыва Республикасы"), ("lt", "Tuva"), ("lv", "Tuva"), ("mk", "Тува"), ("mn", "Бүгд Найрамдах Тува Улс"), ("mr", "त\u{941}वा प\u{94d}रजासत\u{94d}ताक"), ("ms", "Tuva"), ("nb", "Tuva"), ("ne", "ट\u{941}भा गणतन\u{94d}त\u{94d}र"), ("nl", "Toeva"), ("no", "Tuva"), ("pl", "Tuwa"), ("pt", "Tuva"), ("ro", "Tuva"), ("ru", "Тыва"), ("si", "ත\u{dd4}ව\u{dcf} ජනර\u{dcf}ජය"), ("sk", "Tuviansko"), ("sl", "Tuva"), ("sq", "Tëvaja"), ("sr", "Тива"), ("sr_Latn", "Tiva"), ("sv", "Tuva"), ("sw", "Tuva"), ("ta", "டுவ\u{bbe}"), ("te", "టువ\u{c3e} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐต\u{e39}วา"), ("tr", "Tuva Cumhuriyeti"), ("uk", "Тива"), ("ur", "تووا"), ("uz", "Tiva"), ("vi", "Tuva"), ("yue", "圖瓦"), ("yue_Hans", "图瓦"), ("zh", "图瓦共和国")]),
                        unofficial_name_list: ["Tuva"].to_vec(),
                    }
                ),
                (
                    "TYU",
                    Subdivision{
                        name: "TYU",
                        country_alpha2: Alpha2::RU,
                        code: "TYU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.9634387), longitude: Some(66.948278), max_latitude: Some(59.9896339), min_latitude: Some(55.145467), max_longitude: Some(75.19419900000001), min_longitude: Some(64.8277999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tjoemen-oblast"), ("ar", "تيومين أوبلاست"), ("az", "Tümen vilayəti"), ("be", "Цюменская вобласць"), ("bg", "Тюменска област"), ("bn", "তিউমেন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Tjumenjska oblast"), ("ca", "Província de Tiumén"), ("ccp", "𑄑\u{1112d}𑄠\u{1112a}𑄟𑄬𑄚\u{11134}"), ("ceb", "Tyumenskaya Oblast’"), ("cs", "Ťumeňská oblast"), ("cy", "Oblast Tyumen"), ("da", "Tjumen oblast"), ("de", "Oblast Tjumen"), ("el", "Περιφέρεια Τιουμέν"), ("en", "Tyumen"), ("es", "Tiumén"), ("et", "Tjumeni oblast"), ("eu", "Tiumen oblasta"), ("fa", "استان تیومن"), ("fi", "Tjumenin alue"), ("fr", "Oblast de Tioumen"), ("ga", "Cúige Tyumen"), ("gu", "ટ\u{acd}ય\u{ac1}મ\u{ac7}ન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז טיומן"), ("hi", "ट\u{94d}य\u{941}म\u{947}न ओब\u{94d}लास\u{94d}ट"), ("hr", "Tjumenjska oblast"), ("hu", "Tyumenyi terület"), ("hy", "Տյումենի մարզ"), ("id", "Oblast Tyumen"), ("it", "Oblast’ di Tjumen’"), ("ja", "チュメニ州"), ("ka", "ტიუმენის ოლქი"), ("kk", "Түмен облысы"), ("kn", "ತ\u{ccd}ಯುಮ\u{cc6}ನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "튜멘 주"), ("lt", "Tiumenės sritis"), ("lv", "Tjumeņas apgabals"), ("mk", "Тјумењска област"), ("mr", "त\u{94d}य\u{941}म\u{947}न ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Tyumen"), ("nb", "Tjumen"), ("nl", "Oblast Tjoemen"), ("no", "Tjumen"), ("pl", "Obwód tiumeński"), ("pt", "Oblast de Tiumen"), ("ro", "Regiunea Tiumen"), ("ru", "Тюменская область"), ("si", "ට\u{dca}ය\u{dd4}මෙන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ťumenská oblasť"), ("sl", "Tjumenska oblast"), ("sr", "Тјумењска област"), ("sr_Latn", "Tjumenjska oblast"), ("sv", "Tiumen oblast"), ("sw", "Tyumen Oblast"), ("ta", "தியூமென\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "ట\u{c4d}యూమన\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลต\u{e39}ย\u{e4c}เมน"), ("tr", "Tümen Oblastı"), ("uk", "Тюменська область"), ("ur", "تیومن اوبلاست"), ("uz", "Tyumen viloyati"), ("vi", "Tyumen"), ("yue", "秋明州"), ("yue_Hans", "秋明州"), ("zh", "秋明州")]),
                        unofficial_name_list: ["Tjumen", "Tjumenskaja Oblast", "Tumen"].to_vec(),
                    }
                ),
                (
                    "UD",
                    Subdivision{
                        name: "UD",
                        country_alpha2: Alpha2::RU,
                        code: "UD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.0670218), longitude: Some(53.0277948), max_latitude: Some(58.545039), min_latitude: Some(55.8642715), max_longitude: Some(54.42754619999999), min_longitude: Some(51.1226019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oedmoertië"), ("am", "ኡድሙርቲያ"), ("ar", "أودمورتيا"), ("az", "Udmurtiya"), ("be", "Удмуртыя"), ("bg", "Удмуртия"), ("bn", "আডম\u{9c1}র\u{9cd}ত রিপ\u{9be}বলিক"), ("bs", "Udmurtija"), ("ca", "Udmúrtia"), ("ccp", "𑄅\u{1112a}𑄖\u{11134}𑄟𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Udmurtskaya Respublika"), ("cs", "Udmurtsko"), ("da", "Udmurtien"), ("de", "Udmurtien"), ("el", "Δημοκρατία των Ουντμούρτ"), ("en", "Udmurt"), ("es", "Udmurtia"), ("et", "Udmurtia"), ("eu", "Udmurtia"), ("fa", "اودمورتیا"), ("fi", "Udmurtia"), ("fr", "Oudmourtie"), ("ga", "An Udmairt"), ("gl", "Udmurtia"), ("gu", "ઉદમ\u{ac1}ર\u{acd}ત રિપબ\u{acd}લિક"), ("he", "אודמורטיה"), ("hi", "उदम\u{942}र\u{94d}तिया"), ("hr", "Udmurtska"), ("hu", "Udmurtföld"), ("hy", "Ուդմուրտիա"), ("id", "Udmurtia"), ("is", "Údmúrtía"), ("it", "Udmurtia"), ("ja", "ウドムルト共和国"), ("ka", "უდმურტეთი"), ("kn", "ಉಡ\u{ccd}ಮರ\u{ccd}ಟ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "우드무르트 공화국"), ("ky", "Удмуртия"), ("lt", "Udmurtija"), ("lv", "Udmurtija"), ("mk", "Удмуртија"), ("mn", "Удмурт"), ("mr", "उद\u{94d}म\u{941}र\u{94d}तिया प\u{94d}रजासत\u{94d}ताक"), ("ms", "Udmurtia"), ("nb", "Udmurtia"), ("nl", "Oedmoertië"), ("no", "Udmurtia"), ("pl", "Udmurcja"), ("pt", "Udmúrtia"), ("ro", "Udmurtia"), ("ru", "Удмуртия"), ("si", "උඩ\u{dca}ම\u{dd4}ර\u{dca}ට\u{dca} ජනරජය"), ("sk", "Udmurtsko"), ("sl", "Udmurtija"), ("sr", "Удмуртија"), ("sr_Latn", "Udmurtija"), ("sv", "Udmurtien"), ("sw", "Udmurtia"), ("ta", "உத\u{bcd}மூர\u{bcd}த\u{bcd}திய\u{bbe}"), ("te", "ఉడ\u{c4d}మర\u{c4d}ట\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "สาธารณร\u{e31}ฐอ\u{e39}มอร\u{e4c}ท"), ("tr", "Udmurtya"), ("uk", "Удмуртія"), ("ur", "ادمورتیا"), ("uz", "Udmurtiya"), ("vi", "Udmurtia"), ("yue", "烏德穆爾特"), ("yue_Hans", "乌德穆尔特"), ("zh", "乌德穆尔特共和国")]),
                        unofficial_name_list: ["Udmurt Republic", "Udmurtija", "Udmurtskaya Respublika"].to_vec(),
                    }
                ),
                (
                    "ULY",
                    Subdivision{
                        name: "ULY",
                        country_alpha2: Alpha2::RU,
                        code: "ULY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.9793357), longitude: Some(47.7762426), max_latitude: Some(54.891972), min_latitude: Some(52.5380879), max_longitude: Some(50.2391649), min_longitude: Some(45.7949271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oeljanofsk-oblast"), ("ar", "أوليانوفسك أوبلاست"), ("az", "Ulyanovsk vilayəti"), ("be", "Ульянаўская вобласць"), ("bg", "Уляновска област"), ("bn", "উলিয\u{9bc}\u{9be}নভস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Uljanovska oblast"), ("ca", "Província d’Uliànovsk"), ("ccp", "𑄅\u{1112a}𑄣\u{1112d}𑄠𑄚\u{1112e}𑄛\u{11134}𑄥\u{11134}𑄇\u{11134}"), ("ceb", "Ulyanovsk Oblast"), ("cs", "Uljanovská oblast"), ("cy", "Oblast Ulyanovsk"), ("da", "Uljanovsk oblast"), ("de", "Oblast Uljanowsk"), ("el", "Περιφέρεια Ουλιάνοβσκ"), ("en", "Ulyanovsk"), ("es", "Uliánovsk"), ("et", "Uljanovski oblast"), ("eu", "Ulianovsk oblasta"), ("fa", "استان اولیانوفسک"), ("fi", "Uljanovskin alue"), ("fr", "Oblast d’Oulianovsk"), ("ga", "Cúige Ulyanovsk"), ("gu", "ઉલ\u{acd}યાનોવ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אולייאנובסק"), ("hi", "उल\u{94d}यानोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Uljanovska oblast"), ("hu", "Uljanovszki terület"), ("hy", "Ուլյանովսկի մարզ"), ("id", "Oblast Ulyanovsk"), ("it", "Oblast’ di Ul’janovsk"), ("ja", "ウリヤノフスク州"), ("ka", "ულიანოვსკის ოლქი"), ("kn", "ಉಲ\u{ccd}ಯನೋವ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "울리야놉스크 주"), ("lt", "Uljanovsko sritis"), ("lv", "Uļjanovskas apgabals"), ("mk", "Уљјановска област"), ("mn", "Ульяновск муж"), ("mr", "उल\u{94d}यानोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Ulyanovsk"), ("nb", "Uljanovsk"), ("nl", "Oblast Oeljanovsk"), ("no", "Uljanovsk"), ("pl", "Obwód uljanowski"), ("pt", "Oblast de Ulianovsk"), ("ro", "Regiunea Ulianovsk"), ("ru", "Ульяновская область"), ("si", "වැලේ ඩ\u{dd4} බන\u{dca}ඩම\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Ulianovská oblasť"), ("sl", "Uljanovska oblast"), ("sr", "Уљановска област"), ("sr_Latn", "Uljanovska oblast"), ("sv", "Uljanovsk oblast"), ("sw", "Ulyanovsk Oblast"), ("ta", "உலிய\u{bbe}னவ\u{bcd}ஸ\u{bcd}க\u{bcd} ஓப\u{bcd}லஸ\u{bcd}து"), ("te", "ఉల\u{c4d}యన\u{c4b}వస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "อ\u{e38}ลยานอฟสค\u{e4c} โอแบลส"), ("tr", "Ulyanovsk Oblastı"), ("uk", "Ульяновська область"), ("ur", "اولیانووسک اوبلاست"), ("uz", "Ulyanov viloyati"), ("vi", "Ulyanovsk"), ("yue", "烏里揚諾夫斯克州"), ("yue_Hans", "乌里扬诺夫斯克州"), ("zh", "乌里扬诺夫斯克州")]),
                        unofficial_name_list: ["Uljanovsk", "Uljanovskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "VGG",
                    Subdivision{
                        name: "VGG",
                        country_alpha2: Alpha2::RU,
                        code: "VGG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.7604522), longitude: Some(45.0), max_latitude: Some(51.2443039), min_latitude: Some(47.44145899999999), max_longitude: Some(47.4312876), min_longitude: Some(41.167564)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wolgograd-oblast"), ("ar", "فولغوغراد أوبلاست"), ("az", "Volqoqrad vilayəti"), ("be", "Валгаградская вобласць"), ("bg", "Волгоградска област"), ("bn", "ভল\u{9cd}গোগ\u{9cd}র\u{9be}ড ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Volgogradska oblast"), ("ca", "Província de Volgograd"), ("ccp", "𑄞\u{1112e}𑄣\u{11134}𑄉\u{1112e}𑄛\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Volgogradskaya Oblast’"), ("cs", "Volgogradská oblast"), ("cy", "Oblast Volgograd"), ("da", "Volgograd oblast"), ("de", "Oblast Wolgograd"), ("el", "Περιφέρεια Βολγκογκράντ"), ("en", "Volgograd"), ("es", "Volgogrado"), ("et", "Volgogradi oblast"), ("eu", "Volgograd oblasta"), ("fa", "استان ولگوگراد"), ("fi", "Volgogradin alue"), ("fr", "oblast de Volgograd"), ("ga", "Cúige Volgograd"), ("gu", "વોલ\u{acd}ગોગ\u{acd}રાદ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "וולגוגרד"), ("hi", "वोल\u{94d}गोग\u{94d}राद ओब\u{94d}लास\u{94d}ट"), ("hr", "Volgogradska oblast"), ("hu", "Volgográdi terület"), ("hy", "Վոլգոգրադի մարզ"), ("id", "Oblast Volgograd"), ("it", "Oblast’ di Volgograd"), ("ja", "ヴォルゴグラード州"), ("ka", "ვოლგოგრადის ოლქი"), ("kk", "Волгоград облысы"), ("kn", "ವೋಲ\u{ccd}ಗೊಗ\u{ccd}ರಾಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "볼고그라드 주"), ("ky", "Волгоград областы"), ("lt", "Volgogrado sritis"), ("lv", "Volgogradas apgabals"), ("mk", "Волгоградска област"), ("mr", "वोल\u{94d}गोग\u{94d}राद ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Volgograd"), ("nb", "Volgograd"), ("nl", "Oblast Wolgograd"), ("no", "Volgograd"), ("pl", "Obwód wołgogradzki"), ("pt", "Oblast de Volgogrado"), ("ro", "Regiunea Volgograd"), ("ru", "Волгоградская область"), ("si", "වෝල\u{dca}ගොග\u{dca}රඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Volgogradská oblasť"), ("sl", "Volgograjska oblast"), ("sr", "Волгоградска област"), ("sr_Latn", "Volgogradska oblast"), ("sv", "Volgograd oblast"), ("sw", "Volgograd Oblast"), ("ta", "வோல\u{bcd}கோகிர\u{bbe}ட\u{bcd} வட\u{bcd}ட\u{bbe}ரம\u{bcd}"), ("te", "వ\u{c4b}ల\u{c4d}గ\u{c4b}గ\u{c4d}ర\u{c3e}డ\u{c4d}"), ("th", "วอลโกกราด"), ("tr", "Volgograd Oblastı"), ("uk", "Волгоградська область"), ("ur", "وولگوگراڈ اوبلاست"), ("uz", "Volgograd viloyati"), ("vi", "Volgograd"), ("yue", "伏爾加格勒州"), ("yue_Hans", "伏尔加格勒州"), ("zh", "伏尔加格勒州")]),
                        unofficial_name_list: ["Volgogradskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "VLA",
                    Subdivision{
                        name: "VLA",
                        country_alpha2: Alpha2::RU,
                        code: "VLA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.1553465), longitude: Some(40.5926685), max_latitude: Some(56.81697), min_latitude: Some(55.103193), max_longitude: Some(42.977175), min_longitude: Some(38.272862)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wladimir-oblast"), ("ar", "فلاديمير أوبلاست"), ("az", "Vladimir vilayəti"), ("be", "Уладзімірская вобласць"), ("bg", "Владимирска област"), ("bn", "ভ\u{9cd}ল\u{9be}দিমির ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Vladimirska oblast"), ("ca", "Província de Vladímir"), ("ccp", "𑄞\u{11133}𑄣𑄓\u{11128}𑄟\u{11128}𑄢\u{11134}"), ("ceb", "Vladimirskaya Oblast’"), ("cs", "Vladimirská oblast"), ("cy", "Oblast Vladimir"), ("da", "Vladimir oblast"), ("de", "Oblast Wladimir"), ("el", "Περιφέρεια Βλαντίμιρ"), ("en", "Vladimir"), ("es", "Óblast de Vladímir"), ("et", "Vladimiri oblast"), ("eu", "Vladimir oblasta"), ("fa", "استان ولادیمیر"), ("fi", "Vladimirin alue"), ("fr", "Oblast de Vladimir"), ("ga", "Cúige Vladimir"), ("gu", "વ\u{acd}લાદિમીર ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "ולדימיר"), ("hi", "व\u{94d}लादिमीर ओब\u{94d}लास\u{94d}ट"), ("hr", "Vladimirska oblast"), ("hu", "Vlagyimiri terület"), ("hy", "Վլադիմիրի մարզ"), ("id", "Oblast Vladimir"), ("it", "Oblast’ di Vladimir"), ("ja", "ヴラジーミル州"), ("ka", "ვლადიმირის ოლქი"), ("kn", "ವ\u{ccd}ಲಾಡ\u{cbf}ಮ\u{cbf}ರ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "블라디미르 주"), ("ky", "Владимир областы"), ("lt", "Vladimiro sritis"), ("lv", "Vladimiras apgabals"), ("mk", "Владимирска област"), ("mn", "Владимир муж"), ("mr", "व\u{94d}लादिमिर ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Vladimir"), ("nb", "Vladimir"), ("nl", "Oblast Vladimir"), ("no", "Vladimir"), ("pl", "Obwód włodzimierski"), ("pt", "Oblast de Vladimir"), ("ro", "Regiunea Vladimir"), ("ru", "Владимирская область"), ("si", "ව\u{dca}ලම\u{dd2}දර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vladimírska oblasť"), ("sl", "Vladimirska oblast"), ("sr", "Владимирска област"), ("sr_Latn", "Vladimirska oblast"), ("sv", "Vladimir oblast"), ("sw", "Vladimir Oblast"), ("ta", "விள\u{bbe}டிமிர\u{bcd} ஒபில\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "వ\u{c4d}ల\u{c3e}ద\u{c3f}మ\u{c40}ర\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "วลาด\u{e35}ม\u{e35}ร\u{e4c} โอแบลส"), ("tr", "Vladimir Oblastı"), ("uk", "Владимирська область"), ("ur", "ولادیمیر اوبلاست"), ("uz", "Vladimir viloyati"), ("vi", "Vladimir"), ("yue", "伏拉迪米爾州"), ("yue_Hans", "伏拉迪米尔州"), ("zh", "弗拉基米尔州")]),
                        unofficial_name_list: ["Vladimirskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "VLG",
                    Subdivision{
                        name: "VLG",
                        country_alpha2: Alpha2::RU,
                        code: "VLG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.8706711), longitude: Some(40.6555411), max_latitude: Some(61.607277), min_latitude: Some(58.4831769), max_longitude: Some(47.1578849), min_longitude: Some(34.7161178)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wologda-oblast"), ("ar", "فولوغدا أوبلاست"), ("az", "Voloqda vilayəti"), ("be", "Валагодская вобласць"), ("bg", "Вологодска област"), ("bn", "ভোলোগদ\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Vologodska oblast"), ("ca", "Província de Vólogda"), ("ccp", "𑄞\u{1112e}𑄣\u{1112e}𑄇\u{11134}𑄓"), ("ceb", "Vologodskaya Oblast’"), ("cs", "Vologdská oblast"), ("cy", "Oblast Vologda"), ("da", "Vologda oblast"), ("de", "Oblast Wologda"), ("el", "Περιφέρεια Βόλογκντα"), ("en", "Vologda"), ("es", "Óblast de Vólogda"), ("et", "Vologda oblast"), ("eu", "Vologda oblasta"), ("fa", "استان ولوگدا"), ("fi", "Vologdan alue"), ("fr", "Oblast de Vologda"), ("ga", "Cúige Vologda"), ("gu", "વોલોગ\u{acd}ડા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז וולוגדה"), ("hi", "वोलोडा ओब\u{94d}लास\u{94d}ट"), ("hr", "Vologodska oblast"), ("hu", "Vologdai terület"), ("hy", "Վոլոգդայի մարզ"), ("id", "Oblast Vologda"), ("is", "Vologdafylki"), ("it", "Oblast’ di Vologda"), ("ja", "ヴォログダ州"), ("ka", "ვოლოგდის ოლქი"), ("kn", "ವೊಲೊಗ\u{ccd}ಡಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "볼로그다 주"), ("ky", "Вологда областы"), ("lt", "Vologdos sritis"), ("lv", "Vologdas apgabals"), ("mk", "Вологодска област"), ("mr", "वोलोग\u{94d}दा ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Vologda"), ("nb", "Vologda"), ("nl", "Oblast Vologda"), ("no", "Vologda"), ("pl", "Obwód wołogodzki"), ("pt", "Oblast de Vologda"), ("ro", "Regiunea Vologda"), ("ru", "Вологодская область"), ("si", "වොලෝග\u{dca}ඩ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vologdská oblasť"), ("sl", "Vologdska oblast"), ("sr", "Вологдска област"), ("sr_Latn", "Vologdska oblast"), ("sv", "Vologda oblast"), ("sw", "Vologda Oblast"), ("ta", "வொலக\u{bcd}த\u{bbe} ஓப\u{bcd}லஸ\u{bcd}து"), ("te", "వ\u{c4b}\u{c4b}ల\u{c4b}గ\u{c4d}డ\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลโวลอคดา"), ("tr", "Vologda Oblastı"), ("uk", "Вологодська область"), ("ur", "ولوگدا اوبلاست"), ("uz", "Vologda viloyati"), ("vi", "Vologda"), ("yue", "禾洛格達州"), ("yue_Hans", "禾洛格达州"), ("zh", "沃洛格达州")]),
                        unofficial_name_list: ["Vologodskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "VOR",
                    Subdivision{
                        name: "VOR",
                        country_alpha2: Alpha2::RU,
                        code: "VOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.8589713), longitude: Some(39.8644375), max_latitude: Some(52.102429), min_latitude: Some(49.556056), max_longitude: Some(42.944786), min_longitude: Some(38.13708)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woronezj-oblast"), ("ar", "فورونيج أوبلاست"), ("az", "Voronej vilayəti"), ("be", "Варонежская вобласць"), ("bg", "Воронежка област"), ("bn", "ভোরওনেঝ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Voronješka oblast"), ("ca", "Província de Vorónej"), ("ccp", "𑄞\u{1112e}𑄢\u{1112e}𑄚𑄬𑄌\u{11134}"), ("ceb", "Voronezhskaya Oblast’"), ("cs", "Voroněžská oblast"), ("cy", "Oblast Voronezh"), ("da", "Voronezj oblast"), ("de", "Oblast Woronesch"), ("el", "Περιφέρεια Βορόνεζ"), ("en", "Voronezh"), ("es", "Óblast de Vorónezh"), ("et", "Voroneži oblast"), ("eu", "Voronezh oblasta"), ("fa", "استان ورونژ"), ("fi", "Voronežin alue"), ("fr", "oblast de Voronej"), ("ga", "Cúige Voronezh"), ("gu", "વોરોન\u{ac7}ઝ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז וורונז׳"), ("hi", "वोरोन\u{947}ज\u{93c} ओब\u{94d}लास\u{94d}ट"), ("hr", "Voronješka oblast"), ("hu", "Voronyezsi terület"), ("hy", "Վորոնեժի մարզ"), ("id", "Oblast Voronezh"), ("it", "Oblast’ di Voronež"), ("ja", "ヴォロネジ州"), ("ka", "ვორონეჟის ოლქი"), ("kn", "ವೊರೊನ\u{cc6}ಜ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "보로네시 주"), ("ky", "Воронеж областы"), ("lt", "Voronežo sritis"), ("lv", "Voroņežas apgabals"), ("mk", "Воронешка област"), ("mn", "Воронеж муж"), ("mr", "वोरोन\u{947}झ ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Voronezh"), ("nb", "Voronezj"), ("nl", "Oblast Voronezj"), ("no", "Voronezj"), ("pl", "Obwód woroneski"), ("pt", "Oblast de Voronej"), ("ro", "Regiunea Voronej"), ("ru", "Воронежская область"), ("si", "වොරෝනෙස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Voronežská oblasť"), ("sl", "Voroneška oblast"), ("sr", "Вороњешка област"), ("sr_Latn", "Voronješka oblast"), ("sv", "Voronezj oblast"), ("sw", "Voronezh Oblast"), ("ta", "வரனியோஷ\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "వ\u{c4b}ర\u{c4b}న\u{c46}జ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลโวรอนเนซ"), ("tr", "Voronej Oblastı"), ("uk", "Воронезька область"), ("ur", "ورونیش اوبلاست"), ("uz", "Voronej viloyati"), ("vi", "Voronezh"), ("yue", "佛洛尼茲州"), ("yue_Hans", "佛洛尼兹州"), ("zh", "沃罗涅日州")]),
                        unofficial_name_list: ["Voronež", "Voronežskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "YAN",
                    Subdivision{
                        name: "YAN",
                        country_alpha2: Alpha2::RU,
                        code: "YAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(66.0653057), longitude: Some(76.9345194), max_latitude: Some(73.5224935), min_latitude: Some(62.1900029), max_longitude: Some(86.01397010000001), min_longitude: Some(62.0121153)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jamalië"), ("ar", "أوكروغ يامالو-نينيتس الذاتية"), ("az", "Yamalo-Nenets Muxtar Dairəsi"), ("be", "Ямала-Ненецкая аўтаномная акруга"), ("bg", "Ямало-Ненецки автономен окръг"), ("bn", "ইয\u{9bc}\u{9be}ম\u{9be}লো-নেনেটস স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত অক\u{9cd}র\u{9c1}গ"), ("bs", "Jamalskonenečki autonomni okrug"), ("ca", "Iamàlia"), ("ccp", "𑄃\u{11128}𑄠𑄟\u{11134}𑄣\u{1112e}-𑄚𑄚𑄬𑄖\u{11134}𑄌\u{11134} 𑄃\u{1112e}𑄇\u{11134}𑄢𑄇\u{11134}"), ("ceb", "Yamalo-Nenetskiy Avtonomnyy Okrug"), ("cs", "Jamalo-něnecký autonomní okruh"), ("cy", "Ocrwg Ymreolaethol Yamalo-Nenets"), ("da", "Jamalo-Nenets autonome okrug"), ("de", "Autonomer Kreis der Jamal-Nenzen"), ("el", "Αυτόνομος θύλακας των Γιαμάλων Νένετς"), ("en", "Yamalo-Nenets Okrug"), ("es", "Yamalia-Nenetsia"), ("et", "Jamali Neenetsi autonoomne ringkond"), ("eu", "Jamalia-Nenetsia"), ("fa", "یامالو-ننتس"), ("fi", "Jamalin Nenetsia"), ("fr", "Iamalie"), ("ga", "Ceantar Féinrialaitheach Neinéitseach Iamáil"), ("gu", "યમાલો-ન\u{ac7}ન\u{ac7}ટ\u{acd}સ ઓટોનોમસ ઓર\u{acd}કગ"), ("he", "ימלו-ננץ"), ("hi", "यामालो-न\u{947}न\u{947}ट ऑटोनॉमस ऑक\u{94d}रग"), ("hr", "Jamalskonenečki autonomni okrug"), ("hu", "Jamali Nyenyecföld"), ("hy", "Յամալ-Նենեցյան ինքնավար օկրուգ"), ("id", "Yamalia"), ("it", "Circondario autonomo Jamalo-Nenec"), ("ja", "ヤマロ・ネネツ自治管区"), ("ka", "იამალ-ნენთა ავტონომიური ოკრუგი"), ("kn", "ಯಮಲೊ-ನ\u{cc6}ನ\u{cc6}ಟ\u{ccd}ಸ\u{ccd} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಒಕ\u{ccd}ರುಗ\u{ccd}"), ("ko", "야말로네네츠 자치구"), ("ky", "Ямал-Ненец автономиялуу округу"), ("lt", "Jamalo Nencų autonominė apygarda"), ("lv", "Jamalas Ņencu autonomais apvidus"), ("mk", "Јамало-Ненецки автономен округ"), ("mr", "यम\u{947}लो-न\u{947}न\u{947}त\u{94d}स स\u{94d}वायत\u{94d}त ऑक\u{94d}र\u{942}ग"), ("ms", "Negeri autonomi Yamalo-Nenets"), ("nb", "Jamalo-Nenetsk"), ("nl", "Jamalië"), ("no", "Jamalo-Nenetsk"), ("pl", "Jamalsko-Nieniecki Okręg Autonomiczny"), ("pt", "Iamália"), ("ro", "Districtul autonom Iamalia-Neneția"), ("ru", "Ямало-Ненецкий автономный округ"), ("si", "යම\u{dcf}ලෝ-නෙනෙට\u{dca}ස\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd2}න ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jamalsko"), ("sr", "Јамалија"), ("sr_Latn", "Jamalija"), ("sv", "Jamalo-Nentsien"), ("sw", "Mkoa Huru wa Yamalo-Nenets"), ("ta", "யமலோ-நெனெத\u{bcd}து தன\u{bcd}ன\u{bbe}ட\u{bcd}சி வட\u{bcd}ட\u{bbe}ரம\u{bcd}"), ("te", "యమ\u{c3e}ల\u{c4b}-న\u{c46}న\u{c46}ట\u{c4d}స\u{c4d} అట\u{c3e}నమస\u{c4d} ఓక\u{c4d}రుగ\u{c4d}"), ("th", "ยามาโล เนเนท ออโตโนม\u{e31}ส โอเคร\u{e34}ก"), ("tr", "Yamalo-Nenets Özerk Okrugu"), ("uk", "Ямало-Ненецький автономний округ"), ("ur", "یامالو-نینیتس خود مختار آکرگ"), ("uz", "Yamal Nenetslari muxtor okrugi"), ("vi", "Yamalo-Nenets"), ("yue", "亞馬爾-涅涅茨"), ("yue_Hans", "亚马尔-涅涅茨"), ("zh", "亚马尔-涅涅茨自治区")]),
                        unofficial_name_list: ["Jamalija", "Jamalo-Nenets", "Jamalo-Nenetskij Avtonomnyj Okrug"].to_vec(),
                    }
                ),
                (
                    "YAR",
                    Subdivision{
                        name: "YAR",
                        country_alpha2: Alpha2::RU,
                        code: "YAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.8991523), longitude: Some(38.8388633), max_latitude: Some(58.95002100000001), min_latitude: Some(56.5401059), max_longitude: Some(41.178673), min_longitude: Some(37.3243239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jaroslawl-oblast"), ("ar", "ياروسلافل أوبلاست"), ("az", "Yaroslavl vilayəti"), ("be", "Яраслаўская вобласць"), ("bg", "Ярославска област"), ("bn", "ইয\u{9bc}রোস\u{9cd}ল\u{9be}ভ\u{9cd}ল ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Jaroslavljska oblast"), ("ca", "Província de Iaroslavl"), ("ccp", "𑄃\u{11128}𑄠𑄢\u{1112e}𑄌\u{11134}𑄣𑄛\u{11134}"), ("ceb", "Yaroslavskaya Oblast’"), ("cs", "Jaroslavská oblast"), ("cy", "Oblast Yaroslavl"), ("da", "Jaroslavl oblast"), ("de", "Oblast Jaroslawl"), ("el", "Περιφέρεια Γιαροσλάβλ"), ("en", "Yaroslavl"), ("es", "Óblast de Yaroslavl"), ("et", "Jaroslavli oblast"), ("eu", "Jaroslavl oblasta"), ("fa", "استان یاروسلاول"), ("fi", "Jaroslavlin alue"), ("fr", "Oblast de Iaroslavl"), ("ga", "Cúige Yaroslavl"), ("gu", "યારોસ\u{acd}લાવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "ירוסלבל"), ("hi", "यरोस\u{94d}लावी ओब\u{94d}लास\u{94d}ट"), ("hr", "Jaroslavska oblast"), ("hu", "Jaroszlavli terület"), ("hy", "Յարոսլավլի մարզ"), ("id", "Oblast Yaroslavl"), ("it", "Oblast’ di Jaroslavl’"), ("ja", "ヤロスラヴリ州"), ("ka", "იაროსლავლის ოლქი"), ("kn", "ಯಾರೊಸ\u{ccd}ಲಾವ\u{ccd}ಬ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "야로슬라블 주"), ("lt", "Jaroslavlio sritis"), ("lv", "Jaroslavļas apgabals"), ("mk", "Јарославска област"), ("mr", "यारोस\u{94d}लाव ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah Yaroslavl"), ("nb", "Jaroslavl"), ("nl", "Oblast Jaroslavl"), ("no", "Jaroslavl"), ("pl", "Obwód jarosławski"), ("pt", "Oblast de Iaroslavl"), ("ro", "Regiunea Iaroslavl"), ("ru", "Ярославская область"), ("si", "යෝරෝස\u{dca}ල\u{dcf}ව\u{dd3} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jaroslavlianska oblasť"), ("sl", "Jaroslaveljska oblast"), ("sr", "Јарославска област"), ("sr_Latn", "Jaroslavska oblast"), ("sv", "Jaroslavl oblast"), ("sw", "Yaroslavl Oblast"), ("ta", "ய\u{bbe}ரோஸ\u{bcd}ல\u{bbe}வ\u{bcd} ஒப\u{bcd}ல\u{bbe}ஸ\u{bcd}து"), ("te", "య\u{c3e}ర\u{c3e}స\u{c4d}ల\u{c3e}వ\u{c3f} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลยาดรสลาฟ"), ("tr", "Yaroslavl Oblastı"), ("uk", "Ярославська область"), ("ur", "یاروسلاول اوبلاست"), ("uz", "Yaroslavl viloyati"), ("vi", "Yaroslavl"), ("yue", "雅羅斯拉夫爾州"), ("yue_Hans", "雅罗斯拉夫尔州"), ("zh", "雅羅斯拉夫爾州")]),
                        unofficial_name_list: ["Jaroslavl", "Jaroslavskaja Oblast"].to_vec(),
                    }
                ),
                (
                    "YEV",
                    Subdivision{
                        name: "YEV",
                        country_alpha2: Alpha2::RU,
                        code: "YEV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.4808147), longitude: Some(131.7657367), max_latitude: Some(49.4938831), min_latitude: Some(47.65943799999999), max_longitude: Some(134.9953569), min_longitude: Some(130.5212439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Joodse Outonome Oblast"), ("ar", "الأوبلاست اليهودية الذاتية"), ("az", "Yəhudi Muxtar Vilayəti"), ("be", "Яўрэйская аўтаномная вобласць"), ("bg", "Еврейска автономна област"), ("bn", "জ\u{9c1}য\u{9bc}িশ স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Jevrejska autonomna oblast"), ("ca", "Província Autònoma dels Hebreus"), ("ccp", "𑄎𑄬𑄅\u{1112a}𑄠\u{11128}𑄌\u{11134}"), ("ceb", "Yevreyskaya Avtonomnaya Oblast’"), ("cs", "Židovská autonomní oblast"), ("cy", "Oblast Ymreolaethol Iddewig"), ("da", "Jødiske autonome oblast"), ("de", "Jüdische Autonome Oblast"), ("el", "Εβραϊκή Αυτόνομη Περιφέρεια"), ("en", "Jewish"), ("es", "Óblast Autónomo Hebreo"), ("et", "Juudi autonoomne oblast"), ("eu", "Juduen Probintzia Autonomoa"), ("fa", "استان خودگردان یهودی"), ("fi", "Juutalaisten autonominen alue"), ("fr", "Oblast autonome juif"), ("ga", "Cúige Féinrialaitheach na nGiúdach"), ("gl", "Oblast Autónomo Hebreo"), ("gu", "જ\u{ac7}વીશ ઔટોનોમસ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "המחוז היהודי האוטונומי"), ("hi", "ज\u{94d}य\u{942}इश ऑटोनौमस ओब\u{94d}लास\u{94d}ट"), ("hr", "Židovska autonomna oblast"), ("hu", "Zsidó autonóm terület"), ("hy", "Հրեական ինքնավար մարզ"), ("id", "Oblast Otonom Yahudi"), ("is", "Hebreska sjálfstjórnarfylkið"), ("it", "Oblast’ autonoma ebraica"), ("ja", "ユダヤ自治州"), ("ka", "ებრაელთა ავტონომიური ოლქი"), ("kk", "Еврей Автономиялы Облысы"), ("kn", "ಯಹ\u{cc2}ದ\u{cbf} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "유대인 자치주"), ("ky", "Еврей автономия облусу"), ("lt", "Žydų autonominė sritis"), ("lv", "Ebreju autonomais apgabals"), ("mk", "Еврејска автономна област"), ("mr", "ज\u{94d}य\u{942}ईश स\u{94d}वायत\u{94d}त ओब\u{94d}लास\u{94d}त"), ("ms", "Wilayah autonomi Yahudi"), ("nb", "Den jødiske autonome oblast"), ("nl", "Joodse Autonome Oblast"), ("no", "Den jødiske autonome oblast"), ("pl", "Żydowski Obwód Autonomiczny"), ("pt", "Oblast Autônomo Judaico"), ("ro", "Regiunea Autonomă Evreiască"), ("ru", "Еврейская автономная область"), ("si", "ජෙව\u{dd2}ෂ\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd2}න ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Židovská autonómna oblasť"), ("sr", "Јеврејска аутономна област"), ("sr_Latn", "Jevrejska autonomna oblast"), ("sv", "Judiska autonoma länet"), ("sw", "Oblast huru ya Kiyahudi"), ("ta", "யூதர\u{bcd}களின\u{bcd} தன\u{bcd}ன\u{bbe}ட\u{bcd}சி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c4d}యూవ\u{c3f}ష\u{c4d} అట\u{c3e}నమస\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e34}วว\u{e34}ช ออโตโนม\u{e31}ส โอแบลส"), ("tr", "Yahudi Özerk Oblastı"), ("uk", "Єврейська автономна область"), ("ur", "یہودی خود مختار اوبلاست"), ("uz", "Yahudiylar muxtor viloyati"), ("vi", "Tỉnh tự trị Do Thái"), ("yue", "猶太自治州"), ("yue_Hans", "犹太自治州"), ("zh", "犹太自治州")]),
                        unofficial_name_list: ["Jevrej", "Jevrejskaja Oblast", "Jevrejskaja Respublika", "Jewish Autonomous Oblast", "Yevreyskaya Oblast"].to_vec(),
                    }
                ),
                (
                    "ZAB",
                    Subdivision{
                        name: "ZAB",
                        country_alpha2: Alpha2::RU,
                        code: "ZAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.0928771), longitude: Some(116.967656), max_latitude: Some(58.435147), min_latitude: Some(49.1551025), max_longitude: Some(122.1303228), min_longitude: Some(107.736142)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zabaikalski-krai"), ("ar", "زابايكالسكي كراي"), ("az", "Zabaykalsk diyarı"), ("be", "Забайкальскі край"), ("bg", "Забайкалски край"), ("bn", "জ\u{9be}ব\u{9be}য\u{9bc}ক\u{9be}লস\u{9cd}কি ক\u{9cd}র\u{9be}"), ("bs", "Zabajkalski kraj"), ("ca", "Territori de Zabaikal"), ("ccp", "𑄎𑄝𑄬𑄇𑄣\u{11134}𑄌\u{11133}𑄇\u{1112d} 𑄇\u{11133}𑄢\u{1112d}"), ("ceb", "Zabaykal’skiy Kray"), ("cs", "Zabajkalský kraj"), ("cy", "Crai Zabaykalsky"), ("da", "Zabajkalskij kraj"), ("de", "Region Transbaikalien"), ("el", "Κράι Υπερβαϊκάλης"), ("en", "Zabaykalsky Krai"), ("es", "Zabaikalie"), ("et", "Taga-Baikali krai"), ("eu", "Zabaikalski kraia"), ("fa", "سرزمین زابایکالسکی"), ("fi", "Taka-Baikalian aluepiiri"), ("fr", "Kraï de Transbaïkalie"), ("ga", "Zabaykalsky Krai"), ("gu", "ઝાબ\u{ac7}કલ\u{acd}સકી ક\u{acd}ર\u{ac7}ઇ"), ("he", "מחוז עבר הבאיקל"), ("hi", "ज\u{93c}बायकाल\u{94d}स\u{94d}की क\u{94d}राय"), ("hr", "Zabajkalski kraj"), ("hu", "Bajkálontúli határterület"), ("hy", "Անդրբայկալյան երկրամաս"), ("id", "Krai Zabaykalsky"), ("it", "Territorio della Transbajkalia"), ("ja", "ザバイカリエ地方"), ("ka", "იმიერბაიკალეთის მხარე"), ("kn", "ಝಬಾಯಕಲ\u{ccd}ಕ\u{cbf} ಕ\u{ccd}ರೈ"), ("ko", "자바이칼 지방"), ("lt", "Užbaikalės kraštas"), ("lv", "Aizbaikāla novads"), ("mk", "Забајкалски крај"), ("mn", "Өвөр Байгалын хязгаар"), ("mr", "झबायकल\u{94d}स\u{94d}की क\u{94d}राय"), ("ms", "Jajahan Zabaykalsky"), ("nb", "Zabajkalskij"), ("nl", "Kraj Transbaikal"), ("no", "Zabajkalskij"), ("pl", "Kraj Zabajkalski"), ("pt", "Krai de Zabaykalsky"), ("ro", "Regiunea Transbaikal"), ("ru", "Забайкальский край"), ("si", "සබෙකල\u{dca}ස\u{dca}ක\u{dd2} ක\u{dca}\u{200d}රය\u{dd2}"), ("sk", "Zabajkalský kraj"), ("sr", "Забајкалска Покрајина"), ("sr_Latn", "Zabajkalska Pokrajina"), ("sv", "Zabajkalskij kraj"), ("sw", "Zabaykalsky Krai"), ("ta", "சபைக\u{bcd}க\u{bbe}ல\u{bcd}சுக\u{bcd}கி பிரதேசம\u{bcd}"), ("te", "జ\u{c3e}బ\u{c47}క\u{c3e}ల\u{c4d}స\u{c4d}క\u{c40} క\u{c4d}ర\u{c47}"), ("th", "ซาเบคาลสก\u{e35} ไกร"), ("tr", "Zabaykalskiy Krayı"), ("uk", "Забайкальський край"), ("ur", "زابایکالسکی کرائی"), ("uz", "Zabaykal oʼlkasi"), ("vi", "Zabaykalsky"), ("yue", "外貝加爾邊疆區"), ("yue_Hans", "外贝加尔边疆区"), ("zh", "外貝加爾邊疆區")]),
                        unofficial_name_list: ["Zabajkal'skij", "Zabajkal'skij kraj", "Zabaykal'skij kray"].to_vec(),
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
#[cfg(feature = "ru")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::RU,
        alpha3: Alpha3::RUS,
        address_format: Some("{{recipient}}\n{{postalcode}} {{city}}\n{{street}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 7,
        currency_code: CurrencyCode::RUB,
        gec: Some(GEC::RS),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::RUS),
        iso_long_name: "The Russian Federation",
        iso_short_name: "Russian Federation",
        official_language_list: ["ru"].to_vec(),
        spoken_language_list: ["ru"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "8",
        nationality: Some("Russian"),
        number: "643",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "RU",
        unofficial_name_list: [
            "Russia",
            "Russland",
            "Russie",
            "Rusia",
            "ロシア連邦",
            "Rusland",
            "Россия",
            "Расія",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Russian Federation"),
            ("af", "Russiese Federasie"),
            ("ak", "Russian Federation"),
            ("am", "ሲቁ።"),
            ("an", "Russian Federation"),
            ("ar", "الات\u{651}حاد الر\u{651}وسي"),
            ("as", "ৰ\u{9c1}চ য\u{9c1}ক\u{9cd}তৰ\u{9be}ষ\u{9cd}ট\u{9cd}ৰ"),
            ("ay", "Russian Federation"),
            ("az", "Rusiya Federasiyası"),
            ("ba", "Russian Federation"),
            ("be", "Расійская Федэрацыя"),
            ("bg", "Руска федерация"),
            ("bi", "Russian Federation"),
            ("bn", "র\u{9c1}শ য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র"),
            (
                "bn_IN",
                "র\u{9c1}শ য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
            ),
            ("br", "Russian Federation"),
            ("bs", "Ruska Federacija"),
            ("ca", "Federació Russa"),
            ("ce", "Russian Federation"),
            ("ch", "Russian Federation"),
            ("cs", "Ruská federace"),
            ("cv", "Russian Federation"),
            ("cy", "Ffederasiwn Rwsia"),
            ("da", "Russiske føderation"),
            ("de", "Russische Föderation"),
            ("dv", "Russian Federation"),
            (
                "dz",
                "ར་ཤ\u{f72}་ཡ\u{f71}ན་ ཕ\u{f7a}་ཌ\u{f72}་ར\u{f7a}་ཤ\u{f71}ན།",
            ),
            ("ee", "Russian Federation"),
            ("el", "Ρωσική Ομοσπονδία"),
            ("en", "Russian Federation"),
            ("eo", "Rusa Federacio"),
            ("es", "Federación Rusa"),
            ("et", "Venemaa Föderatsioon"),
            ("eu", "Errusiar Federakundea"),
            ("fa", "روسیه فدرال"),
            ("ff", "Russian Federation"),
            ("fi", "Venäjän federaatio"),
            ("fo", "Russian Federation"),
            ("fr", "Russie, Fédération de"),
            ("fy", "Russian Federation"),
            ("ga", "Cónaidhm na Rúise"),
            ("gl", "Federación Rusa"),
            ("gn", "Russian Federation"),
            ("gu", "રશિયન ફ\u{ac7}ડર\u{ac7}શન"),
            ("gv", "Russian Federation"),
            ("ha", "Russian Federation"),
            ("he", "הפדרציה הרוסית"),
            ("hi", "रशियन फ\u{947}ड\u{947}रशन"),
            ("hr", "Ruska Federacija"),
            ("ht", "Russian Federation"),
            ("hu", "Orosz Föderáció"),
            ("hy", "Ռուսաստանի Դաշնություն"),
            ("ia", "Federation Russe"),
            ("id", "Federasi Rusia"),
            ("io", "Russian Federation"),
            ("is", "Rússneska sambandið"),
            ("it", "Russia"),
            ("iu", "Russian Federation"),
            ("ja", "ロシア連邦"),
            ("ka", "რუსეთის ფედრაცია"),
            ("ki", "Russian Federation"),
            ("kk", "Ресей"),
            ("kl", "Russian Federation"),
            (
                "km",
                "សហព\u{17d0}ន\u{17d2}ធ\u{200b}រ\u{17bb}ស\u{17d2}ស\u{17ca}\u{17b8}",
            ),
            ("kn", "ರಶ\u{cbf}ಯನ\u{ccd} ಒಕ\u{ccd}ಕ\u{cc2}ಟ"),
            ("ko", "러시아 연방"),
            ("ku", "Federasyona Rûsî"),
            ("kv", "Russian Federation"),
            ("kw", "Russian Federation"),
            ("ky", "Орус Федерациясы"),
            ("lo", "Russian Federation"),
            ("lt", "Rusijos Federacija"),
            ("lv", "Krievijas Federācija"),
            ("mi", "Russian Federation"),
            ("mk", "Руска федерација"),
            ("ml", "റഷ\u{d4d}യന\u{d4d}\u{200d} ഫെഡറേഷന\u{d4d}\u{200d}"),
            ("mn", "Оросын холбооны улс"),
            ("mr", "रशियन स\u{902}घराज\u{94d}य"),
            ("ms", "Russian Federation"),
            ("mt", "Russian Federation"),
            ("my", "Russian Federation"),
            ("na", "Russian Federation"),
            ("nb", "Den russiske føderasjon"),
            ("ne", "रसियन स\u{902}घ"),
            ("nl", "Rusland"),
            ("nn", "Russland"),
            ("nv", "Russian Federation"),
            ("oc", "Federacion russa"),
            ("or", "ଋଷୀ ଫେଡରେଶନ"),
            ("pa", "ਰ\u{a42}ਸੀ ਗਣਰਾਜ"),
            ("pi", "Russian Federation"),
            ("pl", "Federacja Rosyjska"),
            ("ps", "Russian Federation"),
            ("pt", "Federação Russa"),
            ("pt_BR", "Federação Russa"),
            ("ro", "Federația Rusă"),
            ("ru", "Российская Федерация"),
            ("rw", "Ukwishyirahamwe kw'Uburusiya"),
            ("sc", "Federatzione Russa"),
            ("sd", "Russian Federation"),
            ("si", "ර\u{dd4}ස\u{dd2}ය\u{dcf}න\u{dd4} සංගමය"),
            ("sk", "Ruská federácia"),
            ("sl", "Ruska federacija"),
            ("so", "Ruush"),
            ("sq", "Federata Ruse"),
            ("sr", "Руска Федерација"),
            ("sv", "Ryssland"),
            ("sw", "Russian Federation"),
            ("ta", "ரஷ\u{bcd}யன\u{bcd} கூட\u{bcd}டமைப\u{bcd}பு"),
            ("te", "రష\u{c4d}యన\u{c4d} ఫ\u{c46}డర\u{c47}షన\u{c4d}"),
            ("tg", "Федератсияи Россия"),
            ("th", "สหพ\u{e31}นธร\u{e31}ฐร\u{e31}สเซ\u{e35}ย"),
            ("ti", "Russian Federation"),
            ("tk", "Russiýa Federasiýasy"),
            ("tl", "Pederasyong Ruso"),
            ("tr", "Rusya Federasyonu"),
            ("tt", "Урыс Патшаһлык"),
            ("ug", "رۇسىيە فېدېراتسىيەسى"),
            ("uk", "Російська Федерація"),
            ("ur", "Russian Federation"),
            ("uz", "Russian Federation"),
            ("ve", "Russian Federation"),
            ("vi", "Liên Bang Nga"),
            ("wa", "Rûsseye"),
            ("wo", "Federaasioŋ bu Riisi"),
            ("xh", "Russian Federation"),
            ("yo", "Russian Federation"),
            ("zh_CN", "俄罗斯"),
            ("zh_HK", "俄羅斯聯邦"),
            ("zh_TW", "俄羅斯聯邦"),
            ("zu", "Russian Federation"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
