// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Territory of Cocos (Keeling) Islands

#[cfg(all(feature = "cc", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CC;
    pub const ALPHA3: Alpha3 = Alpha3::CCK;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 61;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::AUD;
    pub const GEC: Option<GEC> = Some(GEC::CK);
    pub const INTERNATIONAL_PREFIX: &str = "0011";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Cocos (Keeling) Islands";
    pub const ISO_LONG_NAME: &str = "The Territory of Cocos (Keeling) Islands";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Cocos Islander");
    pub const NUMBER: &str = "166";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("6799");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::AustraliaAndNewZealand);
    pub const UN_LOCODE: &str = "CC";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Cocos (Keeling) Islands",
        "Kokosinseln",
        "ココス（キーリング）諸島",
        "Cocoseilanden",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Cocos (Keeling) Islands"),
        ("af", "Kokoseilande"),
        ("ak", "Cocos (Keeling) Islands"),
        ("am", "Cocos (Keeling) Islands"),
        ("an", "Cocos (Keeling) Islands"),
        ("ar", "جزر الكوكوس"),
        ("as", "কোকোচ (কিলিং) দ\u{9cd}বীপ"),
        ("ay", "Cocos (Keeling) Islands"),
        ("az", "Kokos Adası"),
        ("ba", "Cocos (Keeling) Islands"),
        ("be", "Какосавыя (Кілінг) астравы"),
        ("bg", "Кокосови острови"),
        ("bi", "Cocos (Keeling) Islands"),
        ("bn", "কোকোস (কিলিং) দ\u{9cd}বীপ"),
        ("bn_IN", "কোকোস (কিলিং) দ\u{9cd}বীপ"),
        ("br", "Inizi Cocos"),
        ("bs", "Kokosova (Kilingova) ostrva"),
        ("ca", "Illes Cocos (Keeling)"),
        ("ce", "Cocos (Keeling) Islands"),
        ("ch", "Cocos (Keeling) Islands"),
        ("cs", "Kokosové ostrovy"),
        ("cv", "Cocos (Keeling) Islands"),
        ("cy", "Ynysoedd Cocos (Keeling)"),
        ("da", "Cocosøerne (Keelingøerne)"),
        ("de", "Kokos-(Keeling-)Inseln"),
        ("dv", "Cocos (Keeling) Islands"),
        (
            "dz",
            "ཀ\u{f7c}་ཀ\u{f7c}ས\u{f72}་(ཀ\u{f72}་ལ\u{f72}ང་)ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།",
        ),
        ("ee", "Cocos (Keeling) Islands"),
        ("el", "Νήσοι Κόκος (Κήλινγκ)"),
        ("en", "Cocos (Keeling) Islands"),
        ("eo", "Kokosinsuloj"),
        ("es", "Islas Cocos (Keeling)"),
        ("et", "Kookossaared"),
        ("eu", "Cocos (Keeling) uharteak"),
        ("fa", "جزایر کوکوس"),
        ("ff", "Cocos (Keeling) Islands"),
        ("fi", "Kookossaaret"),
        ("fo", "Cocos (Keeling) Islands"),
        ("fr", "Cocos (Keeling), Îles"),
        ("fy", "Kokoseilannen"),
        ("ga", "Oileáin Cocos (Oileáin Keeling)"),
        ("gl", "Illas Cocos (Keeling)"),
        ("gn", "Cocos (Keeling) Islands"),
        ("gu", "કોકોસ (કીલી\u{a82}ગ) ટાપ\u{ac1}ઓ"),
        ("gv", "Ellanyn Cocos"),
        ("ha", "Cocos (Keeling) Islands"),
        ("he", "איי קוקוס"),
        ("hi", "कोकोस (कीलि\u{902}ग) द\u{94d}वीपसम\u{942}ह"),
        ("hr", "Cocos (Keeling) otoci"),
        ("ht", "Cocos (Keeling) Islands"),
        ("hu", "Kókusz (Keeling)-szigetek"),
        ("hy", "Կոկոսյան (Քիլինգ) կղզիներ"),
        ("ia", "Insulas Cocos (Keeling)"),
        ("id", "Kepulauan Cocos (Keeling)"),
        ("io", "Cocos (Keeling) Islands"),
        ("is", "Kókoseyjar (Keeling-eyjar)"),
        ("it", "Isole Cocos (Keeling)"),
        ("iu", "Cocos (Keeling) Islands"),
        ("ja", "ココス (キーリング) 諸島"),
        ("ka", "ქოქოსის კუნძული"),
        ("ki", "Cocos (Keeling) Islands"),
        ("kk", "Кокос аралдары"),
        ("kl", "Cocos (Keeling) Islands"),
        ("km", "កោះ\u{200b}ក\u{17bc}ក\u{17bc} (ឃ\u{17b8}ល\u{17b8}ង)"),
        ("kn", "ಕೋಕೋಸ\u{ccd}(ಕೀಲ\u{cbf}ಂಗ\u{ccd})ದ\u{ccd}ವೀಪಗಳು"),
        ("ko", "코코스 제도"),
        ("ku", "Grava Kokos (Kîlîng)"),
        ("kv", "Cocos (Keeling) Islands"),
        ("kw", "Ynysow Cocos"),
        ("ky", "Кокос аралдары"),
        ("lo", "Cocos (Keeling) Islands"),
        ("lt", "Kokosų (Kilingo) salos"),
        ("lv", "Kokosu (Kīlinga) salas"),
        ("mi", "Cocos (Keeling) Islands"),
        ("mk", "Кокос (Килинг) острови"),
        (
            "ml",
            "കൊകോസ\u{d4d} (കീലിങ\u{d4d}ങ\u{d4d}) ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}",
        ),
        ("mn", "Кокос (Кийлинг) Арлууд"),
        ("mr", "कोकस (कीलि\u{902}ग) आयल\u{945}\u{902}डस\u{94d}"),
        ("ms", "Kepulauan Cocos (Keeling)"),
        ("mt", "Gżejjer Kokos (Keeling)"),
        ("my", "Cocos (Keeling) Islands"),
        ("na", "Cocos (Keeling) Islands"),
        ("nb", "Kokosøyene"),
        ("ne", "कोकोस (किलिङ\u{94d}) टाप\u{941}"),
        ("nl", "Cocoseilanden"),
        ("nn", "Kokosøyane"),
        ("nv", "Cocos (Keeling) Islands"),
        ("oc", "Illas Cocos (Keeling)"),
        ("or", "କୋକୋସ (କ\u{b3f}ଲ\u{b3f}ଙ\u{b4d}ଗ) ଦ\u{b4d}ବୀପ"),
        ("pa", "ਕ\u{a4b}ਕ\u{a4b}ਸ(ਕਿਲਿ\u{a70}ਗ) ਟਾਪ\u{a42}"),
        ("pi", "Cocos (Keeling) Islands"),
        ("pl", "Wyspy Kokosowe (Wyspy Keelinga)"),
        ("ps", "Cocos (Keeling) Islands"),
        ("pt", "Ilhas Cocos"),
        ("pt_BR", "Ilhas Cocos"),
        ("ro", "Insulele Cocos (Keeling)"),
        ("ru", "Кокосовые острова"),
        ("rw", "Ibirwa bya Koko"),
        ("sc", "Ìsulas Cocos (Keeling)"),
        ("sd", "Cocos (Keeling) Islands"),
        ("si", "කොකෝව\u{dcf} (ක\u{dd3}ල\u{dd2}ං) ද\u{dd6}පත\u{dca}"),
        ("sk", "Kokosové ostrovy"),
        ("sl", "Kokosovi otoki"),
        ("so", "Cocos (Keeling) Islands"),
        ("sq", "Ishujt Kokos (Kiling)"),
        ("sr", "Кокосова острва"),
        ("sv", "Kokosöarna"),
        ("sw", "Kisiwa cha Cocos (Keeling)"),
        ("ta", "கோகோஸ\u{bcd}(க\u{bc0}லிங\u{bcd}) த\u{bc0}வு"),
        (
            "te",
            "క\u{c4b}కస\u{c4d} (క\u{c40}ల\u{c3f}ంగ\u{c4d}) ఐల\u{c3e}ండ\u{c4d}స\u{c4d}",
        ),
        ("tg", "Ҷазираҳои Кокос (Килинг)"),
        ("th", "หม\u{e39}\u{e48}เกาะโคโคส (ค\u{e35}ล\u{e34}ง)"),
        ("ti", "Cocos (Keeling) Islands"),
        ("tk", "Cocos (Keeling) Islands"),
        ("tl", "Kapuluang Cocos (Keeling)"),
        ("tr", "Cocos (Keeling) Adaları"),
        ("tt", "Кокос (Кеелинg) Утравлары"),
        ("ug", "كوكۇس (كېلىڭ)ئارىلى"),
        ("uk", "Кокосові (Кілінг) острови"),
        ("ur", "جزائر کوکوس"),
        ("uz", "Cocos (Keeling) Islands"),
        ("ve", "Cocos (Keeling) Islands"),
        ("vi", "Quần đảo Co-co-xợ (Khi-lịng)"),
        ("wa", "Iyes Cocos (Keeling)"),
        ("wo", "Cocos (Keeling) Islands"),
        ("xh", "Cocos (Keeling) Islands"),
        ("yo", "Àwọn Erékùṣù Kókósì"),
        ("zh_CN", "科科斯群岛"),
        ("zh_HK", "可可斯羣島"),
        ("zh_TW", "科科斯 (基林) 群島"),
        ("zu", "Cocos (Keeling) Islands"),
    ];
    #[cfg(all(feature = "cc", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -12.164165;
        pub const LONGITUDE: f64 = 96.87095599999999;
        pub const MAX_LATITUDE: f64 = -11.819973;
        pub const MAX_LONGITUDE: f64 = 96.93271639999999;
        pub const MIN_LATITUDE: f64 = -12.2118513;
        pub const MIN_LONGITUDE: f64 = 96.8134118;
        pub const NORTHEAST_LATITUDE: f64 = -11.819973;
        pub const NORTHEAST_LONGITUDE: f64 = 96.93271639999999;
        pub const SOUTHWEST_LATITUDE: f64 = -12.2118513;
        pub const SOUTHWEST_LONGITUDE: f64 = 96.8134118;
    }
}
#[cfg(all(feature = "cc", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -12.164165,
            longitude: 96.87095599999999,
            max_latitude: -11.819973,
            max_longitude: 96.93271639999999,
            min_latitude: -12.2118513,
            min_longitude: 96.8134118,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -11.819973,
                    longitude: 96.93271639999999,
                },
                southwest: CountryGeoBound {
                    latitude: -12.2118513,
                    longitude: 96.8134118,
                },
            },
        }
    }
}

#[cfg(all(feature = "cc", feature = "subdivisions"))]
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
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "cc")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::CC,
        alpha3: Alpha3::CCK,
        address_format: None,
        continent: Continent::Asia,
        country_code: 61,
        currency_code: CurrencyCode::AUD,
        gec: Some(GEC::CK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "0011",
        ioc: None,
        iso_long_name: "The Territory of Cocos (Keeling) Islands",
        iso_short_name: "Cocos (Keeling) Islands",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Cocos Islander"),
        number: "166",
        postal_code: true,
        postal_code_format: Some("6799"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::AustraliaAndNewZealand),
        un_locode: "CC",
        unofficial_name_list: ["Cocos (Keeling) Islands", "Kokosinseln", "ココス（キーリング）諸島", "Cocoseilanden"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Cocos (Keeling) Islands"), ("af", "Kokoseilande"), ("ak", "Cocos (Keeling) Islands"), ("am", "Cocos (Keeling) Islands"), ("an", "Cocos (Keeling) Islands"), ("ar", "جزر الكوكوس"), ("as", "কোকোচ (কিলিং) দ\u{9cd}বীপ"), ("ay", "Cocos (Keeling) Islands"), ("az", "Kokos Adası"), ("ba", "Cocos (Keeling) Islands"), ("be", "Какосавыя (Кілінг) астравы"), ("bg", "Кокосови острови"), ("bi", "Cocos (Keeling) Islands"), ("bn", "কোকোস (কিলিং) দ\u{9cd}বীপ"), ("bn_IN", "কোকোস (কিলিং) দ\u{9cd}বীপ"), ("br", "Inizi Cocos"), ("bs", "Kokosova (Kilingova) ostrva"), ("ca", "Illes Cocos (Keeling)"), ("ce", "Cocos (Keeling) Islands"), ("ch", "Cocos (Keeling) Islands"), ("cs", "Kokosové ostrovy"), ("cv", "Cocos (Keeling) Islands"), ("cy", "Ynysoedd Cocos (Keeling)"), ("da", "Cocosøerne (Keelingøerne)"), ("de", "Kokos-(Keeling-)Inseln"), ("dv", "Cocos (Keeling) Islands"), ("dz", "ཀ\u{f7c}་ཀ\u{f7c}ས\u{f72}་(ཀ\u{f72}་ལ\u{f72}ང་)ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།"), ("ee", "Cocos (Keeling) Islands"), ("el", "Νήσοι Κόκος (Κήλινγκ)"), ("en", "Cocos (Keeling) Islands"), ("eo", "Kokosinsuloj"), ("es", "Islas Cocos (Keeling)"), ("et", "Kookossaared"), ("eu", "Cocos (Keeling) uharteak"), ("fa", "جزایر کوکوس"), ("ff", "Cocos (Keeling) Islands"), ("fi", "Kookossaaret"), ("fo", "Cocos (Keeling) Islands"), ("fr", "Cocos (Keeling), Îles"), ("fy", "Kokoseilannen"), ("ga", "Oileáin Cocos (Oileáin Keeling)"), ("gl", "Illas Cocos (Keeling)"), ("gn", "Cocos (Keeling) Islands"), ("gu", "કોકોસ (કીલી\u{a82}ગ) ટાપ\u{ac1}ઓ"), ("gv", "Ellanyn Cocos"), ("ha", "Cocos (Keeling) Islands"), ("he", "איי קוקוס"), ("hi", "कोकोस (कीलि\u{902}ग) द\u{94d}वीपसम\u{942}ह"), ("hr", "Cocos (Keeling) otoci"), ("ht", "Cocos (Keeling) Islands"), ("hu", "Kókusz (Keeling)-szigetek"), ("hy", "Կոկոսյան (Քիլինգ) կղզիներ"), ("ia", "Insulas Cocos (Keeling)"), ("id", "Kepulauan Cocos (Keeling)"), ("io", "Cocos (Keeling) Islands"), ("is", "Kókoseyjar (Keeling-eyjar)"), ("it", "Isole Cocos (Keeling)"), ("iu", "Cocos (Keeling) Islands"), ("ja", "ココス (キーリング) 諸島"), ("ka", "ქოქოსის კუნძული"), ("ki", "Cocos (Keeling) Islands"), ("kk", "Кокос аралдары"), ("kl", "Cocos (Keeling) Islands"), ("km", "កោះ\u{200b}ក\u{17bc}ក\u{17bc} (ឃ\u{17b8}ល\u{17b8}ង)"), ("kn", "ಕೋಕೋಸ\u{ccd}(ಕೀಲ\u{cbf}ಂಗ\u{ccd})ದ\u{ccd}ವೀಪಗಳು"), ("ko", "코코스 제도"), ("ku", "Grava Kokos (Kîlîng)"), ("kv", "Cocos (Keeling) Islands"), ("kw", "Ynysow Cocos"), ("ky", "Кокос аралдары"), ("lo", "Cocos (Keeling) Islands"), ("lt", "Kokosų (Kilingo) salos"), ("lv", "Kokosu (Kīlinga) salas"), ("mi", "Cocos (Keeling) Islands"), ("mk", "Кокос (Килинг) острови"), ("ml", "കൊകോസ\u{d4d} (കീലിങ\u{d4d}ങ\u{d4d}) ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}"), ("mn", "Кокос (Кийлинг) Арлууд"), ("mr", "कोकस (कीलि\u{902}ग) आयल\u{945}\u{902}डस\u{94d}"), ("ms", "Kepulauan Cocos (Keeling)"), ("mt", "Gżejjer Kokos (Keeling)"), ("my", "Cocos (Keeling) Islands"), ("na", "Cocos (Keeling) Islands"), ("nb", "Kokosøyene"), ("ne", "कोकोस (किलिङ\u{94d}) टाप\u{941}"), ("nl", "Cocoseilanden"), ("nn", "Kokosøyane"), ("nv", "Cocos (Keeling) Islands"), ("oc", "Illas Cocos (Keeling)"), ("or", "କୋକୋସ (କ\u{b3f}ଲ\u{b3f}ଙ\u{b4d}ଗ) ଦ\u{b4d}ବୀପ"), ("pa", "ਕ\u{a4b}ਕ\u{a4b}ਸ(ਕਿਲਿ\u{a70}ਗ) ਟਾਪ\u{a42}"), ("pi", "Cocos (Keeling) Islands"), ("pl", "Wyspy Kokosowe (Wyspy Keelinga)"), ("ps", "Cocos (Keeling) Islands"), ("pt", "Ilhas Cocos"), ("pt_BR", "Ilhas Cocos"), ("ro", "Insulele Cocos (Keeling)"), ("ru", "Кокосовые острова"), ("rw", "Ibirwa bya Koko"), ("sc", "Ìsulas Cocos (Keeling)"), ("sd", "Cocos (Keeling) Islands"), ("si", "කොකෝව\u{dcf} (ක\u{dd3}ල\u{dd2}ං) ද\u{dd6}පත\u{dca}"), ("sk", "Kokosové ostrovy"), ("sl", "Kokosovi otoki"), ("so", "Cocos (Keeling) Islands"), ("sq", "Ishujt Kokos (Kiling)"), ("sr", "Кокосова острва"), ("sv", "Kokosöarna"), ("sw", "Kisiwa cha Cocos (Keeling)"), ("ta", "கோகோஸ\u{bcd}(க\u{bc0}லிங\u{bcd}) த\u{bc0}வு"), ("te", "క\u{c4b}కస\u{c4d} (క\u{c40}ల\u{c3f}ంగ\u{c4d}) ఐల\u{c3e}ండ\u{c4d}స\u{c4d}"), ("tg", "Ҷазираҳои Кокос (Килинг)"), ("th", "หม\u{e39}\u{e48}เกาะโคโคส (ค\u{e35}ล\u{e34}ง)"), ("ti", "Cocos (Keeling) Islands"), ("tk", "Cocos (Keeling) Islands"), ("tl", "Kapuluang Cocos (Keeling)"), ("tr", "Cocos (Keeling) Adaları"), ("tt", "Кокос (Кеелинg) Утравлары"), ("ug", "كوكۇس (كېلىڭ)ئارىلى"), ("uk", "Кокосові (Кілінг) острови"), ("ur", "جزائر کوکوس"), ("uz", "Cocos (Keeling) Islands"), ("ve", "Cocos (Keeling) Islands"), ("vi", "Quần đảo Co-co-xợ (Khi-lịng)"), ("wa", "Iyes Cocos (Keeling)"), ("wo", "Cocos (Keeling) Islands"), ("xh", "Cocos (Keeling) Islands"), ("yo", "Àwọn Erékùṣù Kókósì"), ("zh_CN", "科科斯群岛"), ("zh_HK", "可可斯羣島"), ("zh_TW", "科科斯 (基林) 群島"), ("zu", "Cocos (Keeling) Islands")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
