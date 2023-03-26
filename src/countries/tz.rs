// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The United Republic of Tanzania

#[cfg(all(feature = "tz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::TZ;
    pub const ALPHA3: Alpha3 = Alpha3::TZA;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 255;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::TZS;
    pub const GEC: Option<GEC> = Some(GEC::TZ);
    pub const INTERNATIONAL_PREFIX: &str = "000";
    pub const IOC: Option<IOC> = Some(IOC::TAN);
    pub const ISO_SHORT_NAME: &str = "Tanzania, United Republic of";
    pub const ISO_LONG_NAME: &str = "The United Republic of Tanzania";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "sw"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "sw"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Tanzanian");
    pub const NUMBER: &str = "834";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4,5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "TZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Tanzania",
        "Tansania",
        "Tanzanie",
        "タンザニア",
        "Tanzania United Republic",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Tanzania"),
        ("af", "Tanzanië"),
        ("ak", "Tanzania"),
        ("am", "ታንዛኒያ"),
        ("an", "Tanzania"),
        ("ar", "تنزانيا"),
        ("as", "Tanzania"),
        ("ay", "Tanzania"),
        ("az", "Tanzania"),
        ("ba", "Tanzania"),
        ("be", "Танзанія"),
        ("bg", "Танзания"),
        ("bi", "Tanzania"),
        ("bn", "ত\u{9be}নজ\u{9be}নিয়\u{9be}"),
        ("bn_IN", "ত\u{9be}নজ\u{9be}নিয\u{9bc}\u{9be}"),
        ("br", "Tanzania"),
        ("bs", "Tanzanija"),
        ("ca", "Tanzània"),
        ("ce", "Танзани"),
        ("ch", "Tanzania"),
        ("cs", "Tanzánie"),
        ("cv", "Танзани"),
        ("cy", "Tansanïa"),
        ("da", "Tanzania"),
        ("de", "Tansania"),
        ("dv", "ޓ\u{7ac}ނ\u{7b0}ޒ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "Tanzania"),
        ("ee", "Tanzania"),
        ("el", "Τανζανία"),
        ("en", "Tanzania"),
        ("eo", "Tanzanio"),
        ("es", "Tanzania"),
        ("et", "Tansaania"),
        ("eu", "Tanzania"),
        ("fa", "تانزانیا"),
        ("ff", "Tanzania"),
        ("fi", "Tansanian yhdistynyt tasavalta"),
        ("fo", "Tanzania"),
        ("fr", "Tanzanie"),
        ("fy", "Tanzania"),
        ("ga", "An Tansáin"),
        ("gl", "Tanzania, República Unida de"),
        ("gn", "Tanzania"),
        ("gu", "ટાન\u{acd}ઝાનિયા"),
        ("gv", "Tanzania"),
        ("ha", "Tanzania"),
        ("he", "טנזניה"),
        ("hi", "त\u{902}ज\u{93c}ानिया"),
        ("hr", "Tanzanija"),
        ("ht", "Tanzani"),
        ("hu", "Tanzánia"),
        ("hy", "Tanzania"),
        ("ia", "Tanzania"),
        ("id", "Tanzania"),
        ("io", "Tanzania"),
        ("is", "Tansanía"),
        ("it", "Tanzania"),
        ("iu", "Tanzania"),
        ("ja", "タンザニア"),
        ("ka", "Tanzania"),
        ("ki", "Tanzania"),
        ("kk", "Tanzania"),
        ("kl", "Tanzania"),
        ("km", "តង\u{17cb}ហ\u{17d2}សេន\u{17b8}"),
        ("kn", "Tanzania"),
        ("ko", "탄자니아"),
        ("ku", "Tanzania"),
        ("kv", "Tanzania"),
        ("kw", "Tanzania"),
        ("ky", "Танзания"),
        ("lo", "Tanzania"),
        ("lt", "Tanzanijos Jungtinė Respublika"),
        ("lv", "Tanzānija"),
        ("mi", "Tānahia"),
        ("mk", "Tanzania"),
        ("ml", "Tanzania"),
        ("mn", "Танзани"),
        ("mr", "टा\u{902}झानिया"),
        ("ms", "Tanzania"),
        ("mt", "Tanżanija"),
        (
            "my",
            "တန\u{103a}ဇေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tanzania"),
        ("nb", "Tanzania"),
        ("ne", "Tanzania"),
        ("nl", "Tanzania"),
        ("nn", "Tanzania"),
        ("nv", "Tanzania"),
        ("oc", "Tanzania"),
        ("or", "ଟ\u{b3e}ନଯ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਤਨਜ\u{a3c}ਾਨੀਆ"),
        ("pi", "ट\u{902}जानिया"),
        ("pl", "Tanzania"),
        ("ps", "تانزانیه"),
        ("pt", "Tanzânia"),
        ("pt_BR", "Tanzânia"),
        ("ro", "Tanzania"),
        ("ru", "Танзания"),
        ("rw", "Tanzaniya"),
        ("sc", "Tanzània"),
        ("sd", "Tanzania"),
        ("si", "Tanzania"),
        ("sk", "Tanzánia"),
        ("sl", "Tanzanija"),
        ("so", "Tanzania"),
        ("sq", "Tanzani"),
        ("sr", "Танзанија"),
        ("sv", "Tanzania"),
        ("sw", "Tanzania"),
        ("ta", "Tanzania"),
        ("te", "Tanzania"),
        ("tg", "Танзания"),
        ("th", "แทนซาเน\u{e35}ย"),
        ("ti", "ታንዛንያ"),
        ("tk", "Tanzaniýa"),
        ("tl", "Tanzania"),
        ("tr", "Tanzanya"),
        ("tt", "Tanzania"),
        ("ug", "تانزانىيە"),
        ("uk", "Танзанія"),
        ("ur", "تنزانیہ"),
        ("uz", "Tanzaniya"),
        ("ve", "Tanzania"),
        ("vi", "Tanzania"),
        ("wa", "Tanzania"),
        ("wo", "Tansani"),
        ("xh", "Tanzania"),
        ("yo", "Tànsáníà"),
        ("zh_CN", "坦桑尼亚"),
        ("zh_HK", "坦桑尼亞"),
        ("zh_TW", "坦尚尼亞"),
        ("zu", "ITanzania"),
    ];
    #[cfg(all(feature = "tz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -6.369028;
        pub const LONGITUDE: f64 = 34.888822;
        pub const MAX_LATITUDE: f64 = -0.9843968000000001;
        pub const MAX_LONGITUDE: f64 = 40.6398;
        pub const MIN_LATITUDE: f64 = -11.7612539;
        pub const MIN_LONGITUDE: f64 = 29.34;
        pub const NORTHEAST_LATITUDE: f64 = -0.9843968000000001;
        pub const NORTHEAST_LONGITUDE: f64 = 40.6398;
        pub const SOUTHWEST_LATITUDE: f64 = -11.7612539;
        pub const SOUTHWEST_LONGITUDE: f64 = 29.34;
    }
}
#[cfg(all(feature = "tz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -6.369028,
            longitude: 34.888822,
            max_latitude: -0.9843968000000001,
            max_longitude: 40.6398,
            min_latitude: -11.7612539,
            min_longitude: 29.34,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -0.9843968000000001,
                    longitude: 40.6398,
                },
                southwest: CountryGeoBound {
                    latitude: -11.7612539,
                    longitude: 29.34,
                },
            },
        }
    }
}

#[cfg(all(feature = "tz", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::TZ,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.372301), longitude: Some(36.6944136), max_latitude: Some(-3.3338411), min_latitude: Some(-3.4624464), max_longitude: Some(36.7436028), min_longitude: Some(36.6108228)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أروشا"), ("bg", "Аруша"), ("bn", "আরোশ\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Arusha"), ("ccp", "𑄃𑄢\u{1112a}𑄥"), ("ceb", "Arusha Region"), ("da", "Arusha"), ("de", "Arusha"), ("el", "Περιφέρεια Αρούσα"), ("en", "Arusha"), ("es", "Región de Arusha"), ("et", "Arusha piirkond"), ("fa", "استان آروشا"), ("fi", "Arushan alue"), ("fr", "Région d’Arusha"), ("gl", "Rexión de Arusha"), ("gu", "અર\u{ac1}શા પ\u{acd}રદ\u{ac7}શ"), ("hi", "अर\u{941}शा प\u{94d}रद\u{947}श"), ("hr", "Aruša"), ("id", "Wilayah Arusha"), ("it", "regione di Arusha"), ("ja", "アルーシャ州"), ("ka", "არუშის ოლქი"), ("kn", "ಅರುಶ ಪ\u{ccd}ರದೇಶ"), ("ko", "아루샤 주"), ("lt", "Arušos regionas"), ("lv", "Arušas reģions"), ("mr", "अर\u{941}षा प\u{94d}रद\u{947}श"), ("ms", "Arusha Region"), ("nb", "Arusha"), ("nl", "Arusha"), ("no", "Arusha"), ("pl", "Arusza"), ("pt", "Arusha"), ("ro", "Regiunea Arusha"), ("ru", "Аруша"), ("si", "අර\u{dd4}ෂ\u{dcf} කල\u{dcf}පය"), ("sr", "Аруша"), ("sr_Latn", "Aruša"), ("sv", "Arusha"), ("sw", "Mkoa wa Arusha"), ("ta", "அரூஷ\u{bbe} பகுதி"), ("te", "ఆరుష\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอะร\u{e39}ชา"), ("tr", "Arusha Region"), ("uk", "Аруша"), ("ur", "اروشا ریجن"), ("vi", "Arusha"), ("yo", "Agbègbè Arusha"), ("yo_BJ", "Agbègbè Arusha"), ("yue", "阿魯沙區"), ("yue_Hans", "阿鲁沙区"), ("zh", "阿鲁沙区")]),
                        unofficial_name_list: ["Arusha"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::TZ,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.8), longitude: Some(39.283333), max_latitude: Some(-6.5891538), min_latitude: Some(-6.950965), max_longitude: Some(39.4815816), min_longitude: Some(39.0268707)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم دار السلام"), ("bg", "Дар ес Салаам"), ("bn", "দ\u{9be}র\u{9c1}স-স\u{9be}ল\u{9be}ম অঞ\u{9cd}চল"), ("ca", "Regió de Dar al-Salaam"), ("ccp", "𑄓𑄢\u{11134} 𑄃𑄬𑄌\u{11134} 𑄥𑄣\u{11133}𑄃𑄟\u{11134}"), ("ceb", "Dar es Salaam Region"), ("da", "Dar-es-Salaam"), ("de", "Region Daressalam"), ("el", "Περιφέρεια Νταρ ες Σαλαάμ"), ("en", "Dar es Salaam"), ("es", "Región de Dar es-Salam"), ("fa", "استان دارالسلام"), ("fi", "Dar es Salaam"), ("fr", "Dar es Salaam"), ("gl", "Rexión de Dar es Salaam"), ("gu", "દર એસ સલામ પ\u{acd}રદ\u{ac7}શ"), ("hi", "दार अस सलाम क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Dar-es-Salaam"), ("id", "Wilayah Dar es Salaam"), ("it", "regione di Dar es Salaam"), ("ja", "ダルエスサラーム州"), ("ka", "დარ-ეს-სალამის ოლქი"), ("kn", "ಡಾರ\u{ccd} ಎಸ\u{ccd} ಸಲಾಮ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "다르에스살람 주"), ("lt", "Dar es Salamo regionas"), ("lv", "Dāresalāmas reģions"), ("mk", "Дар ес Салам"), ("mr", "दार एस सलाम प\u{94d}रद\u{947}श"), ("ms", "Dar es Salaam Region"), ("nb", "Dar-es-Salaam"), ("nl", "Dar es Salaam"), ("no", "Dar-es-Salaam"), ("pl", "Dar es Salaam"), ("pt", "Dar es Salaam"), ("ro", "Regiunea Dar es Salaam"), ("ru", "Дар-эс-Салам"), ("si", "ඩ\u{dcf}ර\u{dca} එස\u{dca} සල\u{dcf}ම\u{dca} කල\u{dcf}පය"), ("sr", "Дар ес Салам"), ("sr_Latn", "Dar es Salam"), ("sv", "Dar-es-Salaam (region)"), ("sw", "Mkoa wa Dar es Salaam"), ("ta", "ட\u{bbe}ர\u{bcd} எஸ\u{bcd} ஸல\u{bbe}ம\u{bcd} பகுதி"), ("te", "డ\u{c3e}ర\u{c4d} ఎస\u{c4d} సల\u{c3e}మ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตดาเรสซาราม"), ("tr", "Dar es Salaam Bölgesi"), ("uk", "Регіон Дар-ес-Салам"), ("ur", "د\u{64e}ر اس سلام ریجن"), ("vi", "Dar es Salaam"), ("yo", "Agbègbè Dar es Salaam"), ("yo_BJ", "Agbègbè Dar es Salaam"), ("zh", "达累斯萨拉姆区")]),
                        unofficial_name_list: ["Daressalam"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::TZ,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.173056), longitude: Some(35.741944), max_latitude: Some(-6.120853100000001), min_latitude: Some(-6.2163048), max_longitude: Some(35.8233639), min_longitude: Some(35.6944152)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Додома"), ("ca", "Regió de Dodoma"), ("ccp", "𑄓\u{1112e}𑄓\u{1112e}𑄟"), ("ceb", "Dodoma Region"), ("da", "Dodoma"), ("de", "Dodoma"), ("el", "Περιφέρεια Ντοντόμα"), ("en", "Dodoma"), ("es", "Región de Dodoma"), ("et", "Dodoma piirkond"), ("fa", "استان دودوما"), ("fi", "Dodoman alue"), ("fr", "Dodoma"), ("gl", "Rexión de Dodoma"), ("hi", "दोदोमा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Dodoma"), ("it", "regione di Dodoma"), ("ja", "ドドマ州"), ("ka", "დოდომის ოლქი"), ("ko", "도도마 주"), ("mk", "Додома"), ("nb", "Dodoma"), ("nl", "Dodoma"), ("no", "Dodoma"), ("pl", "Dodoma"), ("pt", "Dodoma"), ("ro", "Regiunea Dodoma"), ("ru", "Додома"), ("sr", "Додома"), ("sr_Latn", "Dodoma"), ("sv", "Dodoma"), ("sw", "Mkoa wa Dodoma"), ("tr", "Dodoma"), ("uk", "Додома (регіон)"), ("vi", "Dodoma"), ("yo", "Agbègbè Dodoma"), ("yo_BJ", "Agbègbè Dodoma"), ("yue", "杜篤瑪區"), ("yue_Hans", "杜笃玛区"), ("zh", "多多馬區")]),
                        unofficial_name_list: ["Dodoma"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::TZ,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.77), longitude: Some(35.69), max_latitude: Some(-7.723390999999999), min_latitude: Some(-7.815317299999999), max_longitude: Some(35.7300982), min_longitude: Some(35.6575782)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم إيرينغا"), ("bg", "Иринга"), ("bn", "ইরিংগ\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Iringa"), ("ccp", "𑄃\u{11128}𑄢\u{11128}𑄚\u{11134}𑄉"), ("ceb", "Iringa Region"), ("da", "Iringa"), ("de", "Iringa"), ("el", "Περιφέρεια Ιρίνγκα"), ("en", "Iringa"), ("es", "Región de Iringa"), ("et", "Iringa piirkond"), ("fa", "استان ایرینگا"), ("fi", "Iringan alue"), ("fr", "Iringa"), ("gl", "Rexión de Iringa"), ("gu", "ઈરિ\u{a82}ગા પ\u{acd}રદ\u{ac7}શ"), ("hi", "इरि\u{902}गा प\u{94d}रद\u{947}श"), ("hr", "Iringa"), ("id", "Wilayah Iringa"), ("it", "regione di Iringa"), ("ja", "イリンガ州"), ("ka", "ირინგის ოლქი"), ("kn", "ಇರ\u{cbf}ಂಗ ಪ\u{ccd}ರದೇಶ"), ("ko", "이링가 주"), ("lt", "Iringos regionas"), ("lv", "Iringas reģions"), ("mr", "इरीगा प\u{94d}रद\u{947}श"), ("ms", "Iringa Region"), ("nb", "Iringa"), ("nl", "Iringa"), ("no", "Iringa"), ("pl", "Iringa"), ("pt", "Iringa"), ("ro", "Regiunea Iringa"), ("ru", "Иринга"), ("si", "ඉර\u{dd2}න\u{dca}ග\u{dcf} පළ\u{dcf}ත"), ("sr", "Иринга"), ("sr_Latn", "Iringa"), ("sv", "Iringa"), ("sw", "Mkoa wa Iringa"), ("ta", "ரிங\u{bcd}க\u{bbe} பகுதி"), ("te", "ఇర\u{c3f}ంగ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอ\u{e35}ร\u{e34}งกา"), ("tr", "Iringa"), ("uk", "Ірінґа (регіон)"), ("ur", "یرینجا ریجن"), ("vi", "Iringa"), ("yo", "Agbègbè Iringa"), ("yo_BJ", "Agbègbè Iringa"), ("yue", "伊林加區"), ("yue_Hans", "伊林加区"), ("zh", "伊林加區")]),
                        unofficial_name_list: ["Iringa"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::TZ,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.3751124), longitude: Some(31.2626366), max_latitude: Some(-0.990736), min_latitude: Some(-3.3440899), max_longitude: Some(32.6189474), min_longitude: Some(30.4033911)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كاجيرا"), ("bg", "Кагера"), ("bn", "ক\u{9be}গের\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Kagera"), ("ccp", "𑄇𑄉𑄬𑄢"), ("ceb", "Kagera Region"), ("da", "Kagera"), ("de", "Kagera"), ("el", "Καγκέρα"), ("en", "Kagera"), ("es", "Región de Kagera"), ("et", "Kagera piirkond"), ("fa", "استان کاگرا"), ("fi", "Kageran alue"), ("fr", "Kagera"), ("gl", "Rexión de Kagera"), ("gu", "ક\u{ac7}ગ\u{ac7}રા પ\u{acd}રદ\u{ac7}શ"), ("hi", "कग\u{947}रा प\u{94d}रद\u{947}श"), ("hr", "Kagera"), ("id", "Region Kagera"), ("it", "regione del Kagera"), ("ja", "カゲラ州"), ("ka", "კაგერის ოლქი"), ("kn", "ಕಗ\u{cc6}ರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "카게라 주"), ("lt", "Kageros regionas"), ("lv", "Kageras reģions"), ("mr", "क\u{947}ग\u{947}रा प\u{94d}रद\u{947}श"), ("ms", "Kagera Region"), ("nb", "Kagera"), ("nl", "Kagera"), ("no", "Kagera"), ("pl", "Kagera"), ("pt", "Kagera"), ("ro", "Regiunea Kagera"), ("ru", "Кагера"), ("si", "කගෙර\u{dcf} කල\u{dcf}පය"), ("sr", "Кагера"), ("sr_Latn", "Kagera"), ("sv", "Kagera"), ("sw", "Mkoa wa Kagera"), ("ta", "கஜ\u{bc0}ர\u{bbe} ர\u{bc0}ஜியன\u{bcd}"), ("te", "క\u{c3e}గ\u{c47}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาเกรา ร\u{e35}\u{e31}เจ\u{e35}\u{e49}ยน"), ("tr", "Kagera Bölgesi"), ("uk", "Регіон Кагера"), ("ur", "کاگیرا علاقہ"), ("vi", "Kagera"), ("yo", "Agbègbè Kagera"), ("yo_BJ", "Agbègbè Kagera"), ("yue", "卡蓋拉區"), ("yue_Hans", "卡盖拉区"), ("zh", "卡蓋拉區")]),
                        unofficial_name_list: ["Akagera", "Ziwa Magharibi"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::TZ,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.031935199999999), longitude: Some(39.7755571), max_latitude: Some(-4.8667473), min_latitude: Some(-5.2071645), max_longitude: Some(39.875565), min_longitude: Some(39.6152071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شمال بيمبا"), ("bg", "Пемба-север"), ("bn", "নর\u{9cd}থ পেম\u{9cd}ব\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Pemba del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛𑄬𑄟𑄝"), ("ceb", "Pemba North Region"), ("da", "Pemba Kaskazini"), ("de", "Pemba Kaskazini"), ("el", "Πέμπα Νορθ Ρίτζιον"), ("en", "North Pemba"), ("es", "Región de Pemba North"), ("fa", "استان پمبای شمالی"), ("fi", "North Pemban maakunta"), ("fr", "Pemba Nord"), ("gl", "Pemba Norte"), ("gu", "ઉત\u{acd}તર પ\u{ac7}મ\u{acd}બા પ\u{acd}રદ\u{ac7}શ"), ("hi", "उत\u{94d}तरी पिम\u{94d}बा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Pemba Sjever"), ("id", "Wilayah Pemba Utara"), ("it", "regione di Pemba Nord"), ("ja", "ペンバ北部州"), ("ka", "ჩრდილოეთი პემბა"), ("kn", "ಉತ\u{ccd}ತರ ಪ\u{cc6}ಂಬಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "펨바 북부 주"), ("lt", "Šiaurės Pembos regionas"), ("lv", "Ziemeļpembas reģions"), ("mr", "उत\u{94d}तर प\u{947}म\u{94d}बा प\u{94d}रद\u{947}श"), ("ms", "North Pemba Region"), ("nb", "Pemba Kaskazini"), ("nl", "Noord-Pemba"), ("no", "Pemba Kaskazini"), ("pl", "Pemba Północna"), ("pt", "Pemba Norte"), ("ro", "Regiunea Pemba North"), ("ru", "Пемба Северная"), ("si", "උත\u{dd4}ර\u{dd4} පෙම\u{dca}බ\u{dcf} කල\u{dcf}පය"), ("sr", "Пемба север"), ("sr_Latn", "Pemba sever"), ("sv", "Norra Pemba"), ("sw", "Mkoa wa Pemba Kaskazini"), ("ta", "நோர\u{bcd}த\u{bcd} பெம\u{bcd}ப\u{bbe} பகுதி"), ("te", "ఉత\u{c4d}తతర ప\u{c46}ంబ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "นอร\u{e4c}ท เปมบา"), ("tr", "Kuzey Pemba Bölgesi"), ("uk", "Регіон Пемба Північна"), ("ur", "پیمبا شمالی علاقہ"), ("vi", "Pemba North"), ("yo", "Agbègbè Pemba Àríwá"), ("yo_BJ", "Agbègbè Pemba Àríwá"), ("yue", "北奔巴區"), ("yue_Hans", "北奔巴区"), ("zh", "北奔巴區")]),
                        unofficial_name_list: ["Kaskazini Pemba"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::TZ,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.9395093), longitude: Some(39.2791011), max_latitude: Some(-5.721938), min_latitude: Some(-6.083642999999999), max_longitude: Some(39.409218), min_longitude: Some(39.186363)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شمال زنجبار"), ("bg", "Северен Занзибар (регион)"), ("bn", "জ\u{9be}ঞ\u{9cd}জিব\u{9be}র উত\u{9cd}তর অঞ\u{9cd}চল"), ("ca", "Regió de Zanzibar Nord"), ("ccp", "𑄎𑄚\u{11134}𑄎\u{11128}𑄝𑄢\u{11134} 𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}"), ("ceb", "Zanzibar North Region"), ("da", "Unguja Kaskazini"), ("de", "Zanzibar North"), ("el", "Ζανζιμπάρ Νορθ Ρίτζιον"), ("en", "Zanzibar North"), ("es", "Región de Zanzibar North"), ("fa", "استان زنگبار شمالی"), ("fi", "Zanzibar Northn maakunta"), ("fr", "Unguja Nord"), ("gl", "Zanzíbar Norte"), ("gu", "ઝા\u{a82}ઝીબાર ઉત\u{acd}તર પ\u{acd}રદ\u{ac7}શ"), ("hi", "ज\u{902}ज\u{93c}ीबार उत\u{94d}तरी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Zanzibar Sjever"), ("id", "Wilayah Utara Zanzibar"), ("it", "regione di Zanzibar Nord"), ("ja", "ザンジバル北部州"), ("ka", "ჩრდილოეთი ზანზიბარი"), ("kn", "ಜಂಜ\u{cbf}ಬಾರ\u{ccd} ನಾರ\u{ccd}ತ\u{ccd} ರೀಜನ\u{ccd}"), ("ko", "잔지바르 북부 주"), ("lt", "Šiaurinis Zanzibaro regionas"), ("lv", "Ziemeļzanzibāras reģions"), ("mk", "Занзибар Север"), ("mr", "झा\u{902}झिबार उत\u{94d}तर प\u{94d}रद\u{947}श"), ("ms", "Zanzibar North Region"), ("nb", "Unguja Kaskazini"), ("nl", "Noord-Zanzibar"), ("no", "Unguja Kaskazini"), ("pl", "Zanzibar Północny"), ("pt", "Zanzibar North (região)"), ("ro", "Regiunea Zanzibar North"), ("ru", "Занзибар Северный"), ("si", "සන\u{dca}ස\u{dd2}බ\u{dcf}ර\u{dca} උත\u{dd4}ර\u{dd4} කල\u{dcf}පය"), ("sr", "Занзибар север"), ("sr_Latn", "Zanzibar sever"), ("sv", "Norra Zanzibar"), ("sw", "Mkoa wa Unguja Kaskazini"), ("ta", "சன\u{bcd}னஜிப\u{bcd}பர\u{bcd} நோர\u{bcd}த\u{bcd} பகுதி"), ("te", "జ\u{c3e}ంజ\u{c3f}బ\u{c3e}ర\u{c4d} న\u{c3e}ర\u{c4d}త\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซานซ\u{e34}บา นอร\u{e4c}ท"), ("tr", "Kuzey Zanzibar Bölgesi"), ("uk", "Регіон Північний Занзібар"), ("ur", "زانزیبر نارتھ ریجن"), ("vi", "Zanzibar North (vùng)"), ("yo", "Agbègbè Zanzibar Àríwá"), ("yo_BJ", "Agbègbè Zanzibar Àríwá"), ("yue", "北安古迦區"), ("yue_Hans", "北安古迦区"), ("zh", "尚西巴北區")]),
                        unofficial_name_list: ["Unguja"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::TZ,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.883332999999999), longitude: Some(29.633333), max_latitude: Some(-4.8398476), min_latitude: Some(-4.9241075), max_longitude: Some(29.7011989), min_longitude: Some(29.5995595)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كيغوما"), ("bg", "Кигома"), ("bn", "কিগ\u{9c1}ম\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Kigoma"), ("ccp", "𑄇\u{11128}𑄉\u{1112e}𑄟"), ("ceb", "Kigoma Region"), ("da", "Kigoma"), ("de", "Kigoma"), ("el", "Περιφέρεια Κιγκόμα"), ("en", "Kigoma"), ("es", "Región de Kigoma"), ("et", "Kigoma piirkond"), ("fa", "استان کیگوما"), ("fi", "Kigoman alue"), ("fr", "Région de Kigoma"), ("gl", "Rexión de Kigoma"), ("gu", "કિગોમા પ\u{acd}રદ\u{ac7}શ"), ("he", "קיגומה"), ("hi", "किगोमा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Kigoma"), ("id", "Wilayah Kigoma"), ("it", "regione di Kigoma"), ("ja", "キゴマ州"), ("ka", "კიგომის ოლქი"), ("kn", "ಕ\u{cbf}ಗೊಮಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "키고마 주"), ("lt", "Kigomos regionas"), ("lv", "Kigomas reģions"), ("mr", "किगोमा प\u{94d}रद\u{947}श"), ("ms", "Kigoma Region"), ("nb", "Kigoma"), ("nl", "Kigoma"), ("no", "Kigoma"), ("pl", "Kigoma"), ("pt", "Kigoma"), ("ro", "Regiunea Kigoma"), ("ru", "Кигома"), ("si", "ක\u{dd2}ගොම\u{dcf} කල\u{dcf}පය"), ("sr", "Кигома"), ("sr_Latn", "Kigoma"), ("sv", "Kigoma"), ("sw", "Mkoa wa Kigoma"), ("ta", "கிகோம\u{bbe} பகுதி"), ("te", "క\u{c3f}గ\u{c4b}మ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ค\u{e34}โกมา"), ("tr", "Kigoma Bölgesi"), ("uk", "Регіон Кігома"), ("ur", "کیگوما علاقہ"), ("vi", "Kigoma"), ("yo", "Agbègbè Kigoma"), ("yo_BJ", "Agbègbè Kigoma"), ("yue", "基戈馬區"), ("yue_Hans", "基戈马区"), ("zh", "基戈馬區")]),
                        unofficial_name_list: ["Kigoma"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::TZ,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.1336927), longitude: Some(37.8087693), max_latitude: Some(-2.8484581), min_latitude: Some(-4.623959999999999), max_longitude: Some(38.471599), min_longitude: Some(36.97385)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كليمنجارو"), ("bg", "Килиманджаро"), ("bn", "কিলিম\u{9be}ঞ\u{9cd}জ\u{9be}রো অঞ\u{9cd}চল"), ("ca", "Regió de Kilimanjaro"), ("ccp", "𑄇\u{11128}𑄣\u{11128}𑄟𑄚\u{11134}𑄎𑄢\u{1112e}"), ("ceb", "Kilimanjaro Region"), ("da", "Kilimanjaro"), ("de", "Kilimandscharo"), ("el", "Περιφέρεια Κιλιμαντζάρο"), ("en", "Kilimanjaro"), ("es", "Región de Kilimanjaro"), ("et", "Kilimanjaro piirkond"), ("fa", "استان کلیمانجارو"), ("fi", "Kilimanjaron alue"), ("fr", "Kilimandjaro"), ("gl", "Rexión do Kilimanjaro"), ("gu", "કિલીમ\u{a82}જારો પ\u{acd}રદ\u{ac7}શ"), ("hi", "किलीम\u{902}जारो प\u{94d}रद\u{947}श"), ("hr", "Kilimandžaro"), ("id", "Wilayah Kilimanjaro"), ("it", "regione del Kilimangiaro"), ("ja", "キリマンジャロ州"), ("ka", "კილიმანჯაროს ოლქი"), ("kn", "ಕ\u{cbf}ಲ\u{cbf}ಮಾಂಜರೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "킬리만자로 주"), ("lt", "Kilimandžaro regionas"), ("lv", "Kilimandžaro reģions"), ("mr", "किलीम\u{902}जारो प\u{94d}रद\u{947}श"), ("ms", "Daerah Kilimanjaro"), ("nb", "Kilimanjaro"), ("nl", "Kilimanjaro"), ("no", "Kilimanjaro"), ("pl", "Kilimandżaro"), ("pt", "Kilimanjaro"), ("ro", "Regiunea Kilimanjaro"), ("ru", "Килиманджаро"), ("si", "ක\u{dd2}ල\u{dd2}මන\u{dca}ජ\u{dcf}රෝ කල\u{dcf}පය"), ("sr", "Килиманџаро"), ("sr_Latn", "Kilimandžaro"), ("sv", "Kilimanjaro"), ("sw", "Mkoa wa Kilimanjaro"), ("ta", "கிளிமஞ\u{bcd}ச\u{bbe}ரோ பகுதி"), ("te", "క\u{c3f}ల\u{c3f}మంజ\u{c3e}ర\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ค\u{e34}ล\u{e34}ม\u{e31}นจาโร"), ("tr", "Kilimanjaro Bölgesi"), ("uk", "Кіліманджаро (регіон)"), ("ur", "کلیمنجارو علاقہ"), ("vi", "Kilimanjaro"), ("yo", "Agbègbè Kilimanjaro"), ("yo_BJ", "Agbègbè Kilimanjaro"), ("yue", "乞力馬扎羅區"), ("yue_Hans", "乞力马扎罗区"), ("zh", "乞力馬扎羅區")]),
                        unofficial_name_list: ["Kilimanjaro"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::TZ,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.3146961), longitude: Some(39.7549511), max_latitude: Some(-5.1784691), min_latitude: Some(-5.476286), max_longitude: Some(39.8530017), min_longitude: Some(39.580599)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جنوب بيمبا"), ("bg", "Пемба-юг"), ("bn", "স\u{9be}উথ পেম\u{9cd}ব\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Pemba Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄛𑄬𑄟\u{11134}𑄝"), ("ceb", "Pemba South Region"), ("da", "Pemba Kusini"), ("de", "Pemba Kusini"), ("el", "Πέμπα"), ("en", "South Pemba"), ("es", "Región de Pemba South"), ("fa", "استان پمبای جنوبی"), ("fi", "South Pemban maakunta"), ("fr", "Pemba Sud"), ("gl", "Pemba Sur"), ("gu", "દક\u{acd}ષિણ પ\u{ac7}મ\u{acd}બા પ\u{acd}રદ\u{ac7}શ"), ("hi", "दक\u{94d}षिण प\u{947}म\u{94d}बा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Pemba Jug"), ("id", "Wilayah Pemba Selatan"), ("it", "regione di Pemba Sud"), ("ja", "ペンバ南部州"), ("ka", "სამხრეთი პემბა"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಪ\u{cc6}ಂಬಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "펨바 남부 주"), ("lt", "Pietinis Pembos regionas"), ("lv", "Dienvidpembas reģions"), ("mr", "दक\u{94d}षिण प\u{947}म\u{94d}बा प\u{94d}रद\u{947}श"), ("ms", "South Pemba Region"), ("nb", "Pemba Kusini"), ("nl", "Zuid-Pemba"), ("no", "Pemba Kusini"), ("pl", "Pemba Południowa"), ("pt", "Pemba Sul"), ("ro", "Regiunea Pemba South"), ("ru", "Пемба Южная"), ("si", "දක\u{dd4}ණ\u{dd4} පෙම\u{dca}බ\u{dcf} කල\u{dcf}පය"), ("sr", "Пемба југ"), ("sr_Latn", "Pemba jug"), ("sv", "Södra Pemba"), ("sw", "Mkoa wa Pemba Kusini"), ("ta", "தெற\u{bcd}கு பெம\u{bcd}ப\u{bbe} பகுதி"), ("te", "దక\u{c4d}ష\u{c3f}ణ ప\u{c46}ంబ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เซาต\u{e4c} เพมบา"), ("tr", "South Pemba Bölgesi"), ("uk", "Регіон Південна Пемба"), ("ur", "پیمبا جنوبی علاقہ"), ("vi", "Pemba South"), ("yo", "Agbègbè Pemba Gúúsù"), ("yo_BJ", "Agbègbè Pemba Gúúsù"), ("yue", "南奔巴區"), ("yue_Hans", "南奔巴区"), ("zh", "南奔巴區")]),
                        unofficial_name_list: ["Kusini Pemba"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::TZ,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.2036563), longitude: Some(39.3621196), max_latitude: Some(-6.0552399), min_latitude: Some(-6.476134999999999), max_longitude: Some(39.579086), min_longitude: Some(39.26983999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Централен/Южен Занзибар"), ("ca", "Regió de Zanzibar Central-Sud"), ("ccp", "𑄎𑄚\u{11134}𑄎\u{11128}𑄝𑄢\u{11134} 𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134}/𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}"), ("ceb", "Zanzibar Central/South Region"), ("da", "Unguja Kusini"), ("de", "Zanzibar Central/South"), ("en", "Zanzibar Central/South"), ("es", "Región de Zanzibar Central/South"), ("fa", "استان زنگبارمرکزی-جنوبی"), ("fr", "Unguja Sud et Central"), ("gl", "Zanzíbar Central/Sur"), ("hr", "Zanzibar Centar/Jug"), ("it", "regione di Zanzibar Centro-Sud"), ("ja", "ザンジバル中部・南部州"), ("ka", "ცენტრალური სამხრეთის ზანზიბარი"), ("ko", "잔지바르 중부·남부 주"), ("nb", "Unguja Kusini"), ("nl", "Centraal- en Zuid-Zanzibar"), ("no", "Unguja Kusini"), ("pl", "Zanzibar Południowy"), ("pt", "Zanzibar Central/South"), ("ro", "Regiunea Zanzibar South and Central"), ("ru", "Занзибар Центрально-Южный"), ("sv", "Södra Zanzibar"), ("sw", "Mkoa wa Unguja Kusini"), ("uk", "Занзібар Південний"), ("yo", "Agbègbè Zanzibar Àrin/Gúúsù"), ("yo_BJ", "Agbègbè Zanzibar Àrin/Gúúsù"), ("yue", "南安古迦區"), ("yue_Hans", "南安古迦区"), ("zh", "尚西巴南區")]),
                        unofficial_name_list: ["Kusini Unguja"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::TZ,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.996944), longitude: Some(39.714444), max_latitude: Some(-9.9624893), min_latitude: Some(-10.0248502), max_longitude: Some(39.7192051), min_longitude: Some(39.6923375)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ليندي"), ("bg", "Линди"), ("bn", "লিন\u{9cd}ডি অঞ\u{9cd}চল"), ("ca", "Regió de Lindi"), ("ccp", "𑄣\u{11128}𑄚\u{11134}𑄓\u{11128}"), ("ceb", "Lindi Region"), ("da", "Lindi"), ("de", "Lindi"), ("el", "Περιφέρεια Λίντι"), ("en", "Lindi"), ("es", "Región de Lindi"), ("et", "Lindi piirkond"), ("fa", "استان لیندی"), ("fi", "Lindin alue"), ("fr", "Lindi"), ("gl", "Rexión de Lindi"), ("gu", "લીન\u{acd}દી પ\u{acd}રદ\u{ac7}શ"), ("he", "לינדי"), ("hi", "लि\u{902}डी प\u{94d}रद\u{947}श"), ("hr", "Lindi"), ("id", "Wilayah Lindi"), ("it", "regione di Lindi"), ("ja", "リンディ州"), ("ka", "ლინდის ოლქი"), ("kn", "ಲ\u{cbf}ಂಡ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "린디 주"), ("lt", "Lindžio regionas"), ("lv", "Lindi reģions"), ("mr", "लि\u{902}डी प\u{94d}रद\u{947}श"), ("ms", "Lindi Region"), ("nb", "Lindi"), ("nl", "Lindi"), ("no", "Lindi"), ("pl", "Lindi"), ("pt", "Lindi"), ("ro", "Regiunea Lindi"), ("ru", "Линди"), ("si", "ල\u{dd2}න\u{dca}ඩ\u{dd2} කල\u{dcf}පය"), ("sr", "Линди"), ("sr_Latn", "Lindi"), ("sv", "Lindi"), ("sw", "Mkoa wa Lindi"), ("ta", "லினடி பகுதி"), ("te", "ల\u{c3f}ండ\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตล\u{e34}นด\u{e34}"), ("tr", "Lindi Region"), ("uk", "Регіон Лінді"), ("ur", "لیندی علاقہ"), ("vi", "Lindi"), ("yo", "Agbègbè Lindi"), ("yo_BJ", "Agbègbè Lindi"), ("yue", "蓮迪區"), ("yue_Hans", "莲迪区"), ("zh", "林迪區")]),
                        unofficial_name_list: ["Lindi"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::TZ,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.7753538), longitude: Some(34.1531947), max_latitude: Some(-1.0252941), min_latitude: Some(-2.528427), max_longitude: Some(35.32159), min_longitude: Some(32.7954015)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم مارا"), ("be", "Мара"), ("bg", "Мара"), ("bn", "ম\u{9be}র\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Mara"), ("ccp", "𑄟𑄢"), ("ceb", "Mara Region"), ("da", "Mara"), ("de", "Mara"), ("el", "Περιφέρεια Μάρα"), ("en", "Mara"), ("es", "Región de Mara"), ("fa", "استان مارا"), ("fi", "Maran alue"), ("fr", "Mara"), ("gl", "Rexión de Mara"), ("gu", "મારા પ\u{acd}રદ\u{ac7}શ"), ("hi", "मारा प\u{94d}रद\u{947}श"), ("hr", "Mara"), ("id", "Wilayah Mara"), ("it", "regione del Mara"), ("ja", "マラ州"), ("ka", "მარის ოლქი"), ("kn", "ಮಾರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "마라 주"), ("lt", "Maros regionas"), ("lv", "Mara reģions"), ("mr", "मरा प\u{94d}रद\u{947}श"), ("ms", "Mara Region"), ("nb", "Mara"), ("nl", "Mara"), ("no", "Mara"), ("pl", "Mara"), ("pt", "Mara"), ("ro", "Regiunea Mara"), ("ru", "Мара"), ("si", "මර\u{dcf} කල\u{dcf}පය"), ("sr", "Мара"), ("sr_Latn", "Mara"), ("sv", "Mara"), ("sw", "Mkoa wa Mara"), ("ta", "ம\u{bbe}ர\u{bbe} பகுதி"), ("te", "మ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตมารา"), ("tr", "Mara Bölgesi"), ("uk", "Регіон Мара"), ("ur", "مارا علاقہ"), ("vi", "Mara"), ("yo", "Agbègbè Mara"), ("yo_BJ", "Agbègbè Mara"), ("yue", "馬拉區"), ("yue_Hans", "马拉区"), ("zh", "馬拉區")]),
                        unofficial_name_list: ["Mara"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::TZ,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.9), longitude: Some(33.45), max_latitude: Some(-8.8526556), min_latitude: Some(-8.9592026), max_longitude: Some(33.5545536), min_longitude: Some(33.3601812)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مبيا"), ("bg", "Мбея"), ("bn", "এমবেয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Mbeya"), ("ccp", "𑄝𑄬𑄠"), ("ceb", "Mbeya Region"), ("da", "Mbeya"), ("de", "Mbeya"), ("el", "Μπέγια"), ("en", "Mbeya"), ("es", "Región de Mbeya"), ("et", "Mbeya piirkond"), ("fa", "استان امبیا"), ("fi", "Mbeyan alue"), ("fr", "Mbeya"), ("gl", "Rexión de Mbeya"), ("gu", "મ\u{acd}બ\u{ac7}યા પ\u{acd}રદ\u{ac7}શ"), ("hi", "म\u{947}बिया प\u{94d}रद\u{947}श"), ("hr", "Mbeya"), ("id", "Wilayah Mbeya"), ("it", "regione di Mbeya"), ("ja", "ムベヤ州"), ("ka", "მბეის ოლქი"), ("kn", "ಮ\u{cc6}ಬ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "므베야 주"), ("lt", "Mbejos regionas"), ("lv", "Mbejas reģions"), ("mr", "माब\u{947}या प\u{94d}रद\u{947}श"), ("ms", "Mbeya Region"), ("nb", "Mbeya"), ("nl", "Mbeya"), ("no", "Mbeya"), ("pl", "Mbeya"), ("pt", "Mbeya"), ("ro", "Regiunea Mbeya"), ("ru", "Мбея"), ("si", "එම\u{dca}බෙය\u{dcf} කල\u{dcf}පය"), ("sr", "Мбеја"), ("sr_Latn", "Mbeja"), ("sv", "Mbeya"), ("sw", "Mkoa wa Mbeya"), ("ta", "ம\u{bcd}பேய\u{bbe} பகுதி"), ("te", "ఎంబ\u{c47}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มเบยา"), ("tr", "Mbeya Bölgesi"), ("uk", "Мбея"), ("ur", "مبیا علاقہ"), ("vi", "Mbeya"), ("yo", "Agbègbè Mbeya"), ("yo_BJ", "Agbègbè Mbeya"), ("yue", "姆貝亞區"), ("yue_Hans", "姆贝亚区"), ("zh", "姆貝亞區")]),
                        unofficial_name_list: ["Mbeya"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::TZ,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.229813600000001), longitude: Some(39.2583293), max_latitude: Some(-6.076521199999999), min_latitude: Some(-6.3821774), max_longitude: Some(39.2991794), min_longitude: Some(39.1856115)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Западен Занзибар"), ("bn", "জ\u{9be}\u{9be}ঞ\u{9cd}জিব\u{9be}র আরব\u{9be}ন/ওয\u{9bc}েস\u{9cd}ট অঞ\u{9cd}চল"), ("ca", "Regió de Zanzibar Urbà-Oest"), ("ccp", "𑄎𑄚\u{11134}𑄎\u{11128}𑄝𑄢\u{11134} 𑄅\u{1112a}𑄢\u{11134}𑄝𑄚\u{11134}/𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}"), ("ceb", "Zanzibar Urban/West Region"), ("da", "Unguja Mjini Magharibi"), ("de", "Zanzibar Urban/West"), ("el", "Μτζίνι Μαγκχαρίμπι"), ("en", "Zanzibar Urban/West"), ("es", "Región de Zanzibar Urban/West"), ("fa", "استان زنگبارشهری-غربی"), ("fi", "Zanzibar Urban/West Region"), ("fr", "Unguja Ville et Ouest"), ("gl", "Zanzíbar Urbano/Oeste"), ("gu", "ઝા\u{a82}ઝીબાર અર\u{acd}બન / પશ\u{acd}ચિમ પ\u{acd}રદ\u{ac7}શ"), ("hr", "Zanzibar Grad/Zapad"), ("hy", "Արևմտյան Զանզիբար"), ("id", "Wilayah Zanzibar Urban/Barat"), ("it", "regione di Zanzibar Urbana-Ovest"), ("ja", "ザンジバル都市部・西部州"), ("ka", "დასავლეთ ზანზიბარი"), ("kn", "ಜಂಜ\u{cbf}ಬಾರ\u{ccd} ಅರ\u{ccd}ಬನ\u{ccd} / ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ರೀಜನ\u{ccd}"), ("ko", "잔지바르 도시·서부 주"), ("lt", "Zanzibaro vakarinis regionas"), ("lv", "Rietumzanzibāras reģions"), ("mr", "झा\u{902}झिबार शहरी/पश\u{94d}चिम प\u{94d}रद\u{947}श"), ("ms", "Mjini Magharibi Region"), ("nb", "Unguja Mjini Magharibi"), ("nl", "Stedelijk- en West-Zanzibar"), ("no", "Unguja Mjini Magharibi"), ("pl", "Zanzibar Zachodni"), ("pt", "Zanzibar Urban/West"), ("ro", "Regiunea Zanzibar West"), ("ru", "Занзибар Западный"), ("si", "සන\u{dca}ස\u{dd2}බ\u{dcf}ර\u{dca} න\u{dcf}ගර\u{dd2}ක /බස\u{dca}න\u{dcf}හ\u{dd2}ර කල\u{dcf}පය"), ("sv", "Västra Zanzibar"), ("sw", "Mkoa wa Unguja Mjini Magharibi"), ("ta", "சன\u{bcd}னஜிப\u{bcd}பர\u{bcd} அர\u{bcd}பன\u{bcd} /மேற\u{bcd}கு பகுதி"), ("th", "เม\u{e37}องแซนซ\u{e34}บาร\u{e4c}/เวส ร\u{e35}เจ\u{e35}\u{e49}ยน"), ("tr", "Zanzibar Kırsal/Batı Bölgesi"), ("uk", "Регіон Західний Занзібар"), ("ur", "مجینی مغربی علاقہ"), ("vi", "Zanzibar West"), ("yo", "Agbègbè Zanzibar Ìlú/Ìwọ\u{300}orùn"), ("yo_BJ", "Agbègbè Zanzibar Ìlú/Ìwɔ\u{300}orùn"), ("yue", "桑給巴爾西區"), ("yue_Hans", "桑给巴尔西区"), ("zh", "尚西巴西區")]),
                        unofficial_name_list: ["Mjini Magharibi"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::TZ,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.816667000000001), longitude: Some(37.666667), max_latitude: Some(-6.792040699999999), min_latitude: Some(-6.8526342), max_longitude: Some(37.69769670000001), min_longitude: Some(37.623539)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم موروغورو"), ("bg", "Морогоро"), ("bn", "ম\u{9c1}রোগ\u{9c1}রো অঞ\u{9cd}চল"), ("ca", "Regió de Morogoro"), ("ccp", "𑄟\u{1112e}𑄢\u{1112e}𑄉\u{1112e}𑄢\u{1112e}"), ("ceb", "Morogoro Region"), ("da", "Morogoro"), ("de", "Morogoro"), ("el", "Περιφέρεια Μορογκόρο"), ("en", "Morogoro"), ("es", "Región de Morogoro"), ("et", "Morogoro piirkond"), ("fa", "استان موروگورو"), ("fi", "Morogoron alue"), ("fr", "Morogoro"), ("gl", "Rexión de Morogoro"), ("gu", "મોરોગોરો પ\u{acd}રદ\u{ac7}શ"), ("hi", "मोरोगोरो क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Morogoro"), ("id", "Wilayah Morogoro"), ("it", "regione di Morogoro"), ("ja", "モロゴロ州"), ("ka", "მოროგოროს ოლქი"), ("kn", "ಮೊರೊಗೊರೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "모로고로 주"), ("lt", "Morogoro regionas"), ("lv", "Morogoro reģions"), ("mr", "मोरोगोरो प\u{94d}रा\u{902}त"), ("ms", "Morogoro Region"), ("nb", "Morogoro"), ("nl", "Morogoro"), ("no", "Morogoro"), ("pl", "Morogoro"), ("pt", "Morogoro"), ("ro", "Regiunea Morogoro"), ("ru", "Морогоро"), ("si", "මොරෝගොරෝ කල\u{dcf}පය"), ("sr", "Морогоро"), ("sr_Latn", "Morogoro"), ("sv", "Morogoro"), ("sw", "Mkoa wa Morogoro"), ("ta", "மொரோகோரோ பகுதி"), ("te", "మ\u{c4b}ర\u{c4b}గ\u{c4b}ర\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โมโรโกโร"), ("tr", "Morogora Bölgesi"), ("uk", "Морогоро"), ("ur", "موروگورو علاقہ"), ("vi", "Morogoro"), ("yo", "Agbègbè Morogoro"), ("yo_BJ", "Agbègbè Morogoro"), ("yue", "莫羅戈羅區"), ("yue_Hans", "莫罗戈罗区"), ("zh", "莫罗戈罗区")]),
                        unofficial_name_list: ["Morogoro"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::TZ,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.273611), longitude: Some(40.182778), max_latitude: Some(-10.2536284), min_latitude: Some(-10.382298), max_longitude: Some(40.212965), min_longitude: Some(40.1423263)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم متوارا"), ("be", "Мтвара"), ("bg", "Мтвара"), ("bn", "এমত\u{9be}ওয\u{9bc}\u{9be}র\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Mtwara"), ("ccp", "𑄑𑄠𑄢"), ("ceb", "Mtwara Region"), ("da", "Mtwara"), ("de", "Mtwara"), ("el", "Περιφέρεια Μτουάρα"), ("en", "Mtwara"), ("es", "Región de Mtwara"), ("fa", "استان امتوارا"), ("fi", "Mtwaran alue"), ("fr", "Mtwara"), ("gl", "Rexión de Mtwara"), ("gu", "મતવારા પ\u{acd}રદ\u{ac7}શ"), ("hi", "मत\u{94d}तवार क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Mtwara"), ("id", "Wilayah Mtwara"), ("it", "regione di Mtwara"), ("ja", "ムトワラ州"), ("ka", "მტვარის ოლქი"), ("kn", "ಮ\u{ccc}ಂಟ\u{ccd}ರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "므트와라 주"), ("lt", "Mtvaros regionas"), ("lv", "Mtvaras reģions"), ("mr", "मत\u{94d}तवार प\u{94d}रद\u{947}श"), ("ms", "Mtwara Region"), ("nb", "Mtwara"), ("nl", "Mtwara"), ("no", "Mtwara"), ("pl", "Mtwara"), ("pt", "Mtwara"), ("ro", "Regiunea Mtwara"), ("ru", "Мтвара"), ("si", "ම\u{dca}ට\u{dca}වර\u{dcf} කල\u{dcf}පය"), ("sr", "Мтвара"), ("sr_Latn", "Mtvara"), ("sv", "Mtwara"), ("sw", "Mkoa wa Mtwara"), ("ta", "மத\u{bcd}வ\u{bbe}ர\u{bbe} பகுதி"), ("te", "మత\u{c4d}వ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มวารา"), ("tr", "Mtwara Bölgesi"), ("uk", "Регіон Мтвара"), ("ur", "متوارا علاقہ"), ("vi", "Mtwara"), ("yo", "Agbègbè Mtwara"), ("yo_BJ", "Agbègbè Mtwara"), ("yue", "姆特華拉區"), ("yue_Hans", "姆特华拉区"), ("zh", "姆特瓦拉區")]),
                        unofficial_name_list: ["Mtware"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::TZ,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.516667), longitude: Some(32.9), max_latitude: Some(-2.4342968), min_latitude: Some(-2.5725184), max_longitude: Some(32.9982823), min_longitude: Some(32.8744311)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم موانزا"), ("bg", "Муанза"), ("bn", "এমওয\u{9bc}\u{9be}ঞ\u{9cd}জ\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Mwanza"), ("ccp", "𑄤𑄚\u{11134}𑄎"), ("ceb", "Mwanza Region"), ("da", "Mwanza"), ("de", "Mwanza"), ("el", "Περιφέρεια Μουάνζα"), ("en", "Mwanza"), ("es", "Región de Mwanza"), ("et", "Mwanza piirkond"), ("fa", "استان موانزا"), ("fi", "Mwanzan alue"), ("fr", "Mwanza"), ("gl", "Rexión de Mwanza"), ("gu", "મ\u{acd}વાન\u{acd}ઝા પ\u{acd}રદ\u{ac7}શ"), ("hi", "मवान\u{94d}ज\u{93c}ा प\u{94d}रद\u{947}श"), ("hr", "Mwanza"), ("id", "Wilayah Mwanza"), ("it", "regione di Mwanza"), ("ja", "ムワンザ州"), ("ka", "მვანზის ოლქი"), ("kn", "ಮುವಾಝಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "므완자 주"), ("lt", "Mvangos regionas"), ("lv", "Mvanzas reģions"), ("mr", "म\u{94d}वा\u{902}झ\u{942} प\u{94d}रद\u{947}श"), ("ms", "Mwanza Region"), ("nb", "Mwanza"), ("nl", "Mwanza"), ("no", "Mwanza"), ("pl", "Mwanza"), ("pt", "Mwanza"), ("ro", "Regiunea Mwanza"), ("ru", "Мванза"), ("si", "ම\u{dca}වන\u{dca}ස\u{dcf} කල\u{dcf}පය"), ("sr", "Мванза"), ("sr_Latn", "Mvanza"), ("sv", "Mwanza"), ("sw", "Mkoa wa Mwanza"), ("ta", "மவன\u{bcd}ச பகுதி"), ("te", "మవ\u{c3e}ంజ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มว\u{e31}นซา"), ("tr", "Mwanza Bölgesi"), ("uk", "Регіон Мванза"), ("ur", "موانزا علاقہ"), ("vi", "Mwanza"), ("yo", "Agbègbè Mwanza"), ("yo_BJ", "Agbègbè Mwanza"), ("yue", "姆宛札區"), ("yue_Hans", "姆宛札区"), ("zh", "姆萬紮區")]),
                        unofficial_name_list: ["Mwanza"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::TZ,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.323771400000001), longitude: Some(38.8205454), max_latitude: Some(-5.828001), min_latitude: Some(-8.4510141), max_longitude: Some(39.910326), min_longitude: Some(37.8144299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بواني"), ("be", "Пвані"), ("bg", "Пвани"), ("bn", "পউয\u{9bc}\u{9be}নি অঞ\u{9cd}চল"), ("ca", "Regió de Pwani"), ("ccp", "𑄤𑄚\u{11128}"), ("ceb", "Coast Region"), ("da", "Pwani"), ("de", "Pwani"), ("el", "Περιφέρεια Πουάνι"), ("en", "Pwani"), ("es", "Región de Pwani"), ("fa", "استان پوانی"), ("fi", "Pwanin alue"), ("fr", "Pwani"), ("gl", "Pwani"), ("gu", "પવાની પ\u{acd}રદ\u{ac7}શ"), ("hi", "पवानी प\u{94d}रद\u{947}श"), ("hr", "Pwani"), ("id", "Wilayah Pwani"), ("it", "regione di Pwani"), ("ja", "プワニ州"), ("ka", "პვანის ოლქი"), ("kn", "ಪ\u{ccd}ವಾನ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "프와니 주"), ("lt", "Pvanio regionas"), ("lv", "Pvani reģions"), ("mr", "पवाणी प\u{94d}रद\u{947}श"), ("ms", "Pwani Region"), ("nb", "Pwani"), ("nl", "Pwani"), ("no", "Pwani"), ("pl", "Pwani"), ("pt", "Pwani"), ("ro", "Regiunea Pwani"), ("ru", "Пвани"), ("si", "ප\u{dca}වන\u{dd2} කල\u{dcf}පය"), ("sr", "Пвани"), ("sr_Latn", "Pvani"), ("sv", "Pwani"), ("sw", "Mkoa wa Pwani"), ("ta", "பவ\u{bbe}னி பகுதி"), ("te", "ప\u{c4d}వ\u{c3e}న\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตปวาน\u{e34}"), ("tr", "Pwani Region"), ("uk", "Регіон Пвані"), ("ur", "پوانی علاقہ"), ("vi", "Pwani"), ("yo", "Agbègbè Pwani"), ("yo_BJ", "Agbègbè Pwani"), ("yue", "濱海區"), ("yue_Hans", "滨海区"), ("zh", "濱海區")]),
                        unofficial_name_list: ["Coast"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::TZ,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.965368499999999), longitude: Some(31.2626366), max_latitude: Some(-5.182548100000001), min_latitude: Some(-9.076138), max_longitude: Some(32.7445109), min_longitude: Some(29.95819109999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم روكوا"), ("bg", "Руква"), ("bn", "র\u{9c1}কওয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Rukwa"), ("ccp", "𑄢\u{1112a}𑄇\u{11127}𑄤"), ("ceb", "Rukwa Region"), ("da", "Rukwa"), ("de", "Rukwa"), ("el", "Περιφέρεια Ρούκουα"), ("en", "Rukwa"), ("es", "Región de Rukwa"), ("fa", "استان روکاوا"), ("fi", "Rukwan alue"), ("fr", "Rukwa"), ("gl", "Rukwa"), ("gu", "ર\u{ac1}કવા પ\u{acd}રદ\u{ac7}શ"), ("hi", "र\u{941}क\u{94d}वा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Rukwa"), ("id", "Wilayah Rukwa"), ("it", "regione del Rukwa"), ("ja", "ルクワ州"), ("ka", "რუკვის ოლქი"), ("kn", "ರುಕ\u{ccd}ವಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "루콰 주"), ("lt", "Rukvos regionas"), ("lv", "Rukvas reģions"), ("mr", "र\u{941}क\u{94d}वा प\u{94d}रद\u{947}श"), ("ms", "Rukwa Region"), ("nb", "Rukwa"), ("nl", "Rukwa"), ("no", "Rukwa"), ("pl", "Rukwa"), ("pt", "Rukwa"), ("ro", "Regiunea Rukwa"), ("ru", "Руква"), ("si", "ර\u{dd4}ක\u{dca}ව\u{dcf} කල\u{dcf}පය"), ("sr", "Руква"), ("sr_Latn", "Rukva"), ("sv", "Rukwa"), ("sw", "Mkoa wa Rukwa"), ("ta", "ருக\u{bcd}கவ\u{bbe} பகுதி"), ("te", "రుక\u{c4d}వ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ร\u{e38}กวา"), ("tr", "Rukwa Bölgesi"), ("uk", "Регіон Руква"), ("ur", "رکوا ریجن"), ("vi", "Rukwa"), ("yo", "Agbègbè Rukwa"), ("yo_BJ", "Agbègbè Rukwa"), ("yue", "魯夸區"), ("yue_Hans", "鲁夸区"), ("zh", "魯夸區")]),
                        unofficial_name_list: ["Rukwa"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::TZ,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.6878717), longitude: Some(36.2630846), max_latitude: Some(-9.1864479), min_latitude: Some(-11.745696), max_longitude: Some(38.071091), min_longitude: Some(34.5682455)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم روفوما"), ("bg", "Рувума"), ("bn", "র\u{9c1}ভ\u{9c1}ম\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Ruvuma"), ("ccp", "𑄢\u{1112a}𑄞\u{1112a}𑄟"), ("ceb", "Ruvuma Region"), ("da", "Ruvuma"), ("de", "Ruvuma"), ("el", "Ρουβούμα"), ("en", "Ruvuma"), ("es", "Región de Ruvuma"), ("fa", "استان روووما"), ("fi", "Ruvuman alue"), ("fr", "Ruvuma"), ("gl", "Rexión de Ruvuma"), ("gu", "ર\u{ac1}વ\u{ac1}મા પ\u{acd}રદ\u{ac7}શ"), ("hi", "र\u{942}व\u{941}मा प\u{94d}रद\u{947}श"), ("hr", "Ruvuma"), ("id", "Wilayah Ruvuma"), ("it", "regione del Ruvuma"), ("ja", "ルヴマ州"), ("ka", "რუვუმის ოლქი"), ("kn", "ರುವುಮಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "루부마 주"), ("lt", "Ruvumos regionas"), ("lv", "Ruvumas reģions"), ("mk", "Рувума"), ("mr", "र\u{941}वामा प\u{94d}रद\u{947}श"), ("ms", "Ruvuma Region"), ("nb", "Ruvuma"), ("nl", "Ruvuma"), ("no", "Ruvuma"), ("pl", "Ruvuma"), ("pt", "Ruvuma"), ("ro", "Regiunea Ruvuma"), ("ru", "Рувума"), ("si", "ර\u{dd4}ව\u{dd4}ම\u{dcf} කල\u{dcf}පය"), ("sr", "Рувума"), ("sr_Latn", "Ruvuma"), ("sv", "Ruvuma"), ("sw", "Mkoa wa Ruvuma"), ("ta", "ருவும\u{bbe} பகுதி"), ("te", "రువూమ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ร\u{e39}ว\u{e39}มา"), ("tr", "Ruvuma Region"), ("uk", "Регіон Рувума"), ("ur", "روووما ریجن"), ("vi", "Ruvuma"), ("yo", "Agbègbè Ruvuma"), ("yo_BJ", "Agbègbè Ruvuma"), ("yue", "魯伍馬區"), ("yue_Hans", "鲁伍马区"), ("zh", "魯伍馬區")]),
                        unofficial_name_list: ["Ruvuma"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::TZ,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.661944), longitude: Some(33.423056), max_latitude: Some(-3.650936499999999), min_latitude: Some(-3.6935938), max_longitude: Some(33.4494835), min_longitude: Some(33.3919132)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شينيانغا"), ("bg", "Шинянга"), ("bn", "শিনিয\u{9bc}\u{9be}ঙ\u{9cd}গ\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Shinyanga"), ("ccp", "𑄥\u{11128}𑄚\u{11128}𑄠\u{11101}𑄉"), ("ceb", "Shinyanga Region"), ("da", "Shinyanga"), ("de", "Shinyanga"), ("el", "Περιφέρεια Σινυάνγκα"), ("en", "Shinyanga"), ("es", "Región de Shinyanga"), ("fa", "استان شینیانگا"), ("fi", "Shinyangan alue"), ("fr", "Shinyanga"), ("gl", "Rexión de Shinyanga"), ("gu", "શિન\u{acd}ય\u{a82}ગા પ\u{acd}રદ\u{ac7}શ"), ("hi", "शिन\u{94d}या\u{902}गा प\u{94d}रद\u{947}श"), ("hr", "Šinjanga"), ("id", "Wilayah Shinyanga"), ("it", "regione di Shinyanga"), ("ja", "シニャンガ州"), ("ka", "შინიანგის ოლქი"), ("kn", "ಶ\u{cbf}ನ\u{ccd}ಯಾಂಗ ಪ\u{ccd}ರದೇಶ"), ("ko", "시니앙가 주"), ("lt", "Šinjangos regionas"), ("lv", "Šiņangas reģions"), ("mr", "शिनी\u{902}गा प\u{94d}रा\u{902}त"), ("ms", "Shinyanga Region"), ("nb", "Shinyanga"), ("nl", "Shinyanga"), ("no", "Shinyanga"), ("pl", "Shinyanga"), ("pt", "Shinyanga"), ("ro", "Regiunea Shinyanga"), ("ru", "Шиньянга"), ("si", "ශ\u{dd2}න\u{dca}යන\u{dca}ග\u{dcf} කල\u{dcf}පය"), ("sr", "Шињанга"), ("sr_Latn", "Šinjanga"), ("sv", "Shinyanga"), ("sw", "Mkoa wa Shinyanga"), ("ta", "ஷின\u{bcd}ய\u{bbe}ங\u{bcd}க பகுதி"), ("te", "ష\u{c3f}న\u{c4d}య\u{c3e}ంగ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โลการ\u{e4c}"), ("tr", "Shinyanga Bölgesi"), ("uk", "Регіон Шиньянга"), ("ur", "شینیانگا علاقہ"), ("vi", "Shinyanga"), ("yo", "Agbègbè Shinyanga"), ("yo_BJ", "Agbègbè Shinyanga"), ("yue", "辛陽賈區"), ("yue_Hans", "辛阳贾区"), ("zh", "欣延加區")]),
                        unofficial_name_list: ["Shinyanga"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::TZ,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.819999999999999), longitude: Some(34.73999999999999), max_latitude: Some(-4.777412600000001), min_latitude: Some(-4.8474173), max_longitude: Some(34.7794962), min_longitude: Some(34.706626)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم سينغيدا"), ("bg", "Сингида"), ("bn", "সিনগিড\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Singida"), ("ccp", "𑄥\u{11128}𑄚\u{11134}𑄉\u{11128}𑄓"), ("ceb", "Singida Region"), ("da", "Singida"), ("de", "Singida"), ("el", "Περιφέρεια Σινγκίντα"), ("en", "Singida"), ("es", "Región de Singida"), ("fa", "استان سینگیدا"), ("fi", "Singidan alue"), ("fr", "Singida"), ("gl", "Rexión de Singida"), ("gu", "સિ\u{a82}ગીડા પ\u{acd}રદ\u{ac7}શ"), ("hi", "सि\u{902}गिदा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Singida"), ("id", "Wilayah Singida"), ("it", "regione di Singida"), ("ja", "シンギダ州"), ("ka", "სინგიდის ოლქი"), ("kn", "ಸ\u{cbf}ಂಗ\u{cbf}ಡಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "싱기다 주"), ("lt", "Singidos regionas"), ("lv", "Singidas reģions"), ("mr", "सि\u{902}गदा प\u{94d}रद\u{947}श"), ("ms", "Singida Region"), ("nb", "Singida"), ("nl", "Singida"), ("no", "Singida"), ("pl", "Singida"), ("pt", "Singida"), ("ro", "Regiunea Singida"), ("ru", "Сингида"), ("si", "ස\u{dd2}න\u{dca}ග\u{dd2}ඩ\u{dcf} කල\u{dcf}පය"), ("sr", "Сингида"), ("sr_Latn", "Singida"), ("sv", "Singida"), ("sw", "Mkoa wa Singida"), ("ta", "சிங\u{bcd}கிட\u{bbe} பகுதி"), ("te", "స\u{c3f}ంగ\u{c3f}డ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซ\u{e34}นก\u{e34}ด\u{e49}า"), ("tr", "Singida Bölgesi"), ("uk", "Регіон Сингіда"), ("ur", "سنگیدا علاقہ"), ("vi", "Singida"), ("yo", "Agbègbè Singida"), ("yo_BJ", "Agbègbè Singida"), ("yue", "辛基達區"), ("yue_Hans", "辛基达区"), ("zh", "辛吉達區")]),
                        unofficial_name_list: ["Singida"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::TZ,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.016667), longitude: Some(32.8), max_latitude: Some(-4.9860158), min_latitude: Some(-5.0897659), max_longitude: Some(32.8532632), min_longitude: Some(32.7742009)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تابورا"), ("bg", "Табора"), ("bn", "ত\u{9be}ব\u{9c1}র\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Tabora"), ("ccp", "𑄑𑄝\u{1112e}𑄢"), ("ceb", "Tabora Region"), ("da", "Tabora"), ("de", "Tabora"), ("el", "Περιφέρεια Ταμπόρα"), ("en", "Tabora"), ("es", "Región de Tabora"), ("et", "Tabora piirkond"), ("fa", "استان تابورا"), ("fi", "Taboran alue"), ("fr", "Tabora (région)"), ("gl", "Rexión de Tabora"), ("gu", "તબોરા પ\u{acd}રદ\u{ac7}શ"), ("hi", "तबोरा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Tabora"), ("id", "Wilayah Tabora"), ("it", "regione di Tabora"), ("ja", "タボーラ州"), ("ka", "ტაბორის ოლქი"), ("kn", "ಟಾಬರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "타보라 주"), ("lt", "Taboros regionas"), ("lv", "Taboras reģions"), ("mr", "ताबोरा प\u{94d}रद\u{947}श"), ("ms", "Tabora Region"), ("nb", "Tabora"), ("nl", "Tabora"), ("no", "Tabora"), ("pl", "Tabora"), ("pt", "Tabora"), ("ro", "Regiunea Tabora"), ("ru", "Табора"), ("si", "ටබෝර\u{dcf} කල\u{dcf}පය"), ("sr", "Табора"), ("sr_Latn", "Tabora"), ("sv", "Tabora"), ("sw", "Mkoa wa Tabora"), ("ta", "தம\u{bcd}போர\u{bbe} பகுதி"), ("te", "టబ\u{c4b}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ทาบอรา"), ("tr", "Tabora Region"), ("uk", "Регіон Табора"), ("ur", "تابورا علاقہ"), ("vi", "Tabora"), ("yo", "Agbègbè Tabora"), ("yo_BJ", "Agbègbè Tabora"), ("yue", "塔波拉區"), ("yue_Hans", "塔波拉区"), ("zh", "塔波拉區")]),
                        unofficial_name_list: ["Tabora"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::TZ,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.0888751), longitude: Some(39.1023228), max_latitude: Some(-5.0548382), min_latitude: Some(-5.1261043), max_longitude: Some(39.1312501), min_longitude: Some(39.0567078)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تانغا"), ("bg", "Танга"), ("bn", "ত\u{9be}ংগ\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Tanga"), ("ccp", "𑄑𑄚\u{11134}𑄉"), ("ceb", "Tanga Region"), ("da", "Tanga"), ("de", "Tanga"), ("el", "Περιφέρεια Τάνγκα"), ("en", "Tanga"), ("es", "Región de Tanga"), ("et", "Tanga piirkond"), ("eu", "Tanga eskualdea"), ("fa", "استان تانگا"), ("fi", "Tangan alue"), ("fr", "Tanga"), ("gl", "Rexión de Tanga"), ("gu", "ત\u{a82}ગા પ\u{acd}રદ\u{ac7}શ"), ("hi", "ट\u{902}गा क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Tanga"), ("id", "Wilayah Tanga"), ("it", "regione di Tanga"), ("ja", "タンガ州"), ("ka", "ტანგის ოლქი"), ("kn", "ಟಂಗಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "탕가 주"), ("lt", "Tangos regionas"), ("lv", "Tangas reģions"), ("mr", "त\u{902}गा प\u{94d}रद\u{947}श"), ("ms", "Tanga Region"), ("nb", "Tanga"), ("nl", "Tanga"), ("no", "Tanga"), ("pl", "Tanga"), ("pt", "Tanga"), ("ro", "Regiunea Tanga"), ("ru", "Танга"), ("si", "ටැන\u{dca}ග\u{dcf} කල\u{dcf}පය"), ("sr", "Танга"), ("sr_Latn", "Tanga"), ("sv", "Tanga"), ("sw", "Mkoa wa Tanga"), ("ta", "தங\u{bcd}க\u{bbe} பகுதி"), ("te", "ట\u{c3e}ంగ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ท\u{e31}งกา"), ("tr", "Tanga Region"), ("uk", "Танга"), ("ur", "ٹانگہ ریجن"), ("vi", "Tanga"), ("yo", "Agbègbè Tanga"), ("yo_BJ", "Agbègbè Tanga"), ("yue", "坦噶區"), ("yue_Hans", "坦噶区"), ("zh", "坦噶区")]),
                        unofficial_name_list: ["Tanga"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::TZ,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.3150058), longitude: Some(36.954107), max_latitude: Some(-3.391186), min_latitude: Some(-5.980841), max_longitude: Some(38.0391009), min_longitude: Some(34.78854)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مانيارا"), ("bg", "Маняра"), ("bn", "ম\u{9cd}য\u{9be}নিয\u{9bc}\u{9be}র\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Manyara"), ("ccp", "𑄟𑄚\u{11128}𑄠𑄢"), ("ceb", "Manyara Region"), ("da", "Manyara"), ("de", "Manyara"), ("el", "Περιφέρεια Μανυάρα"), ("en", "Manyara"), ("es", "Región de Manyara"), ("et", "Manyara piirkond"), ("fa", "استان مانیارا"), ("fi", "Manyaran alue"), ("fr", "Manyara"), ("gl", "Manyara"), ("gu", "માન\u{acd}યારા પ\u{acd}રા\u{a82}ત"), ("hi", "मन\u{94d}यारा प\u{94d}रद\u{947}श"), ("hr", "Manyara"), ("id", "Wilayah Manyara"), ("it", "regione del Manyara"), ("ja", "マニャラ州"), ("ka", "მანიარის ოლქი"), ("kn", "ಮನ\u{ccd}ಯಾರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "마냐라 주"), ("lt", "Manjaros regionas"), ("lv", "Manjaras reģions"), ("mr", "अन\u{947}कारा प\u{94d}रद\u{947}श"), ("ms", "Manyara Region"), ("nb", "Manyara"), ("nl", "Manyara"), ("no", "Manyara"), ("pl", "Manyara"), ("pt", "Manyara"), ("ro", "Regiunea Manyara"), ("ru", "Маньяра"), ("si", "මන\u{dca}යර\u{dcf} කල\u{dcf}පය"), ("sr", "Мањара"), ("sr_Latn", "Manjara"), ("sv", "Manyara"), ("sw", "Mkoa wa Manyara"), ("ta", "ம\u{bbe}ன\u{bcd}யர\u{bbe} பகுதி"), ("te", "మన\u{c4d}య\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ม\u{e31}นยารา"), ("tr", "Manyara Bölgesi"), ("uk", "Регіон Маньяра"), ("ur", "مانیارا علاقہ"), ("vi", "Manyara"), ("yo", "Agbègbè Manyara"), ("yo_BJ", "Agbègbè Manyara"), ("yue", "曼亞拉區"), ("yue_Hans", "曼亚拉区"), ("zh", "曼亞拉區")]),
                        unofficial_name_list: ["Manyara"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::TZ,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جيتا"), ("bn", "গীত\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Geita"), ("ccp", "𑄉\u{11128}𑄠𑄬𑄑"), ("ceb", "Geita Region"), ("da", "Geita Region"), ("de", "Region Geita"), ("el", "Γκέιτα"), ("en", "Geita"), ("es", "Región Geita"), ("et", "Geita piirkond"), ("fa", "استان گیتا"), ("fi", "Geitan alue"), ("fr", "Région de Geita"), ("gu", "ગ\u{ac7}ઇતા પ\u{acd}રદ\u{ac7}શ"), ("hi", "गीता क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Geita"), ("id", "Wilayah Geita"), ("it", "Regione di Geita"), ("ja", "ゲイタ州"), ("ka", "გეიტის ოლქი"), ("kn", "ಗೀತಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "게이타 주"), ("lt", "Geitos regionas"), ("lv", "Geitas reģions"), ("mr", "ग\u{947}ईता प\u{94d}रद\u{947}श"), ("ms", "Geita Region"), ("nb", "Geita"), ("nl", "Geita Regio"), ("no", "Geita"), ("pl", "Region Geita"), ("pt", "Geita"), ("ru", "Гейта"), ("si", "ගෙය\u{dd2}ට\u{dcf} කල\u{dcf}පය"), ("sv", "Geita"), ("sw", "Mkoa wa Geita"), ("ta", "ஜெய\u{bcd}த\u{bcd}த\u{bbe} பகுதி"), ("te", "గ\u{c46}య\u{c3f}ట\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เกยตา"), ("tr", "Geita Bölgesi"), ("uk", "Регіон Гейта"), ("ur", "گئیتا علاقہ"), ("vi", "Khu vực Geita"), ("yue", "蓋塔區"), ("yue_Hans", "盖塔区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::TZ,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كاتافي"), ("bn", "ক\u{9be}ট\u{9be}ভি অঞ\u{9cd}চল"), ("ca", "Regió de Katavi"), ("ccp", "𑄇𑄑𑄞\u{11128}"), ("ceb", "Katavi Region"), ("da", "Katavi Region"), ("de", "Region Katavi"), ("el", "Περιφέρεια Κατάβι"), ("en", "Katavi"), ("es", "Región del Katavi"), ("fi", "Katavin alue"), ("fr", "Région de Katavi"), ("gu", "કાતાવી પ\u{acd}રદ\u{ac7}શ"), ("hi", "कातावी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Katavi"), ("id", "Wilayah Katavi"), ("it", "Regione del Katavi"), ("ja", "カタヴィ州"), ("ka", "კატავის ოლქი"), ("kn", "ಕಟಾವ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "카타비 주"), ("lt", "Katavio regionas"), ("lv", "Katavi reģions"), ("mr", "कातावी प\u{94d}रद\u{947}श"), ("ms", "Katavi Region"), ("nb", "Katavi"), ("nl", "Katavi"), ("no", "Katavi"), ("pl", "Region Katavi"), ("pt", "Katavi"), ("ru", "Катави"), ("si", "කටව\u{dd2} කල\u{dcf}පය"), ("sv", "Katavi (region)"), ("sw", "Mkoa wa Katavi"), ("ta", "தடவி பகுதி"), ("te", "కట\u{c3e}వ\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาทาว\u{e35}"), ("tr", "Katavi Bölgesi"), ("uk", "Регіон Катаві"), ("ur", "کاتاوی علاقہ"), ("vi", "Khu vực Katavi"), ("yue", "卡塔維區"), ("yue_Hans", "卡塔维区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::TZ,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم نجومبي"), ("bn", "জম\u{9cd}বে অঞ\u{9cd}চল"), ("ca", "Regió de Njombe"), ("ccp", "𑄎\u{1112e}𑄟\u{11134}𑄝\u{11128}"), ("ceb", "Njombe Region"), ("da", "Njombe Region"), ("de", "Region Njombe"), ("el", "Περιφέρεια Ντζόμπε"), ("en", "Njombe"), ("es", "Región del Njombe"), ("fi", "Njomben alue"), ("fr", "Région de Njombe"), ("gu", "ન\u{acd}જોમ\u{acd}બ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("hi", "नियोम\u{94d}ब\u{947} क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Njombe"), ("id", "Wilayah Njombe"), ("it", "Regione di Njombe"), ("ja", "ンジョンベ州"), ("ka", "ნჯომბეს ოლქი"), ("kn", "ನಜೋಬ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "느좀베 주"), ("lt", "Njombės regionas"), ("lv", "Nģombes reģions"), ("mr", "नोजोम\u{94d} प\u{94d}रद\u{947}श"), ("ms", "Daerah Njombe"), ("nb", "Njombe"), ("nl", "Njombe Region"), ("no", "Njombe"), ("pl", "Region Njombe"), ("pt", "Região de Njombe"), ("ru", "Нджомб"), ("si", "එන\u{dca}ජෝම\u{dca}බේ කල\u{dcf}පය"), ("sv", "Njombe"), ("sw", "Mkoa wa Njombe"), ("ta", "ஞ\u{bcd}சோம\u{bcd}பே பகுதி"), ("te", "ఎన\u{c4d}జ\u{c4b}ంబ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตจอมบ\u{e34}"), ("tr", "Nhombe Bölgesi"), ("uk", "Регіон Нджомбе"), ("ur", "نجومبے علاقہ"), ("vi", "Khu vực Njombe"), ("yue", "恩瓊貝區"), ("yue_Hans", "恩琼贝区"), ("zh", "恩瓊貝區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::TZ,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم سيميو"), ("bn", "সিমিয\u{9bc}ো অঞ\u{9cd}চল"), ("ca", "Regió de Simiyu"), ("ccp", "𑄥\u{11128}𑄟\u{11128}𑄠\u{1112a}"), ("ceb", "Simiyu Region"), ("da", "Simiya"), ("de", "Region Simiyu"), ("el", "Περιφέρεια Σιμίγιου"), ("en", "Simiyu"), ("es", "Región del Simiyu"), ("fi", "Simiyun alue"), ("fr", "Simiyu"), ("gu", "સિમિય\u{ac1} પ\u{acd}રદ\u{ac7}શ"), ("hi", "सिमिऊ प\u{94d}रद\u{947}श"), ("hr", "Simiju"), ("id", "Wilayah Simiyu"), ("it", "Regione del Simiyu"), ("ja", "シミユ州"), ("ka", "სიმიუს ოლქი"), ("kn", "ಸ\u{cbf}ಮ\u{cbf}ಯು ಪ\u{ccd}ರದೇಶ"), ("ko", "시미유 주"), ("lt", "Simijo regionas"), ("lv", "Simiju reģions"), ("mr", "सिमिय\u{941} प\u{94d}रद\u{947}श"), ("ms", "Simiyu Region"), ("nb", "Simiyu"), ("nl", "Simiya"), ("no", "Simiyu"), ("pl", "Region Simiyu"), ("pt", "Região de Simiyu"), ("ru", "Симию"), ("si", "ස\u{dd2}ම\u{dd2}ය\u{dd4} කල\u{dcf}පය"), ("sv", "Simiyu (region)"), ("sw", "Mkoa wa Simiyu"), ("ta", "சிமியு பகுதி"), ("te", "స\u{c3f}మ\u{c3f}యు ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตซ\u{e34}ม\u{e34}ย\u{e39}"), ("tr", "Simiyu Region"), ("uk", "Регіон Сімію"), ("ur", "سیمیو ریجن"), ("vi", "Khu vực Simiyu"), ("yue", "錫米尤區"), ("yue_Hans", "锡米尤区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::TZ,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.369028), longitude: Some(34.888822), max_latitude: Some(-0.9843968000000001), min_latitude: Some(-11.7612539), max_longitude: Some(40.6398), min_longitude: Some(29.34)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Songwe")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "tz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::TZ,
        alpha3: Alpha3::TZA,
        address_format: None,
        continent: Continent::Africa,
        country_code: 255,
        currency_code: CurrencyCode::TZS,
        gec: Some(GEC::TZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "000",
        ioc: Some(IOC::TAN),
        iso_long_name: "The United Republic of Tanzania",
        iso_short_name: "Tanzania, United Republic of",
        official_language_list: ["en", "sw"].to_vec(),
        spoken_language_list: ["en", "sw"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Tanzanian"),
        number: "834",
        postal_code: true,
        postal_code_format: Some("\\d{4,5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "TZ",
        unofficial_name_list: [
            "Tanzania",
            "Tansania",
            "Tanzanie",
            "タンザニア",
            "Tanzania United Republic",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Tanzania"),
            ("af", "Tanzanië"),
            ("ak", "Tanzania"),
            ("am", "ታንዛኒያ"),
            ("an", "Tanzania"),
            ("ar", "تنزانيا"),
            ("as", "Tanzania"),
            ("ay", "Tanzania"),
            ("az", "Tanzania"),
            ("ba", "Tanzania"),
            ("be", "Танзанія"),
            ("bg", "Танзания"),
            ("bi", "Tanzania"),
            ("bn", "ত\u{9be}নজ\u{9be}নিয়\u{9be}"),
            ("bn_IN", "ত\u{9be}নজ\u{9be}নিয\u{9bc}\u{9be}"),
            ("br", "Tanzania"),
            ("bs", "Tanzanija"),
            ("ca", "Tanzània"),
            ("ce", "Танзани"),
            ("ch", "Tanzania"),
            ("cs", "Tanzánie"),
            ("cv", "Танзани"),
            ("cy", "Tansanïa"),
            ("da", "Tanzania"),
            ("de", "Tansania"),
            ("dv", "ޓ\u{7ac}ނ\u{7b0}ޒ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "Tanzania"),
            ("ee", "Tanzania"),
            ("el", "Τανζανία"),
            ("en", "Tanzania"),
            ("eo", "Tanzanio"),
            ("es", "Tanzania"),
            ("et", "Tansaania"),
            ("eu", "Tanzania"),
            ("fa", "تانزانیا"),
            ("ff", "Tanzania"),
            ("fi", "Tansanian yhdistynyt tasavalta"),
            ("fo", "Tanzania"),
            ("fr", "Tanzanie"),
            ("fy", "Tanzania"),
            ("ga", "An Tansáin"),
            ("gl", "Tanzania, República Unida de"),
            ("gn", "Tanzania"),
            ("gu", "ટાન\u{acd}ઝાનિયા"),
            ("gv", "Tanzania"),
            ("ha", "Tanzania"),
            ("he", "טנזניה"),
            ("hi", "त\u{902}ज\u{93c}ानिया"),
            ("hr", "Tanzanija"),
            ("ht", "Tanzani"),
            ("hu", "Tanzánia"),
            ("hy", "Tanzania"),
            ("ia", "Tanzania"),
            ("id", "Tanzania"),
            ("io", "Tanzania"),
            ("is", "Tansanía"),
            ("it", "Tanzania"),
            ("iu", "Tanzania"),
            ("ja", "タンザニア"),
            ("ka", "Tanzania"),
            ("ki", "Tanzania"),
            ("kk", "Tanzania"),
            ("kl", "Tanzania"),
            ("km", "តង\u{17cb}ហ\u{17d2}សេន\u{17b8}"),
            ("kn", "Tanzania"),
            ("ko", "탄자니아"),
            ("ku", "Tanzania"),
            ("kv", "Tanzania"),
            ("kw", "Tanzania"),
            ("ky", "Танзания"),
            ("lo", "Tanzania"),
            ("lt", "Tanzanijos Jungtinė Respublika"),
            ("lv", "Tanzānija"),
            ("mi", "Tānahia"),
            ("mk", "Tanzania"),
            ("ml", "Tanzania"),
            ("mn", "Танзани"),
            ("mr", "टा\u{902}झानिया"),
            ("ms", "Tanzania"),
            ("mt", "Tanżanija"),
            (
                "my",
                "တန\u{103a}ဇေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tanzania"),
            ("nb", "Tanzania"),
            ("ne", "Tanzania"),
            ("nl", "Tanzania"),
            ("nn", "Tanzania"),
            ("nv", "Tanzania"),
            ("oc", "Tanzania"),
            ("or", "ଟ\u{b3e}ନଯ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਤਨਜ\u{a3c}ਾਨੀਆ"),
            ("pi", "ट\u{902}जानिया"),
            ("pl", "Tanzania"),
            ("ps", "تانزانیه"),
            ("pt", "Tanzânia"),
            ("pt_BR", "Tanzânia"),
            ("ro", "Tanzania"),
            ("ru", "Танзания"),
            ("rw", "Tanzaniya"),
            ("sc", "Tanzània"),
            ("sd", "Tanzania"),
            ("si", "Tanzania"),
            ("sk", "Tanzánia"),
            ("sl", "Tanzanija"),
            ("so", "Tanzania"),
            ("sq", "Tanzani"),
            ("sr", "Танзанија"),
            ("sv", "Tanzania"),
            ("sw", "Tanzania"),
            ("ta", "Tanzania"),
            ("te", "Tanzania"),
            ("tg", "Танзания"),
            ("th", "แทนซาเน\u{e35}ย"),
            ("ti", "ታንዛንያ"),
            ("tk", "Tanzaniýa"),
            ("tl", "Tanzania"),
            ("tr", "Tanzanya"),
            ("tt", "Tanzania"),
            ("ug", "تانزانىيە"),
            ("uk", "Танзанія"),
            ("ur", "تنزانیہ"),
            ("uz", "Tanzaniya"),
            ("ve", "Tanzania"),
            ("vi", "Tanzania"),
            ("wa", "Tanzania"),
            ("wo", "Tansani"),
            ("xh", "Tanzania"),
            ("yo", "Tànsáníà"),
            ("zh_CN", "坦桑尼亚"),
            ("zh_HK", "坦桑尼亞"),
            ("zh_TW", "坦尚尼亞"),
            ("zu", "ITanzania"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
