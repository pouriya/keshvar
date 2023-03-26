// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The State of Palestine

#[cfg(all(feature = "ps", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PS;
    pub const ALPHA3: Alpha3 = Alpha3::PSE;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 970;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::ILS;
    pub const GEC: Option<GEC> = Some(GEC::WE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::PLE);
    pub const ISO_SHORT_NAME: &str = "Palestine, State of";
    pub const ISO_LONG_NAME: &str = "The State of Palestine";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar", "en", "he"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar", "en", "he"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Palestinian");
    pub const NUMBER: &str = "275";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "PS";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Palestine",
        "فلسطين",
        "Palästina",
        "Palestina",
        "the Occupied Palestinian Territory",
        "パレスチナ",
        "Palestijnse gebieden",
        "Palestinian Territory Occupied",
        "Palestinian Authority",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Palestine, State of"),
        ("af", "Palestine, State of"),
        ("ak", "Palestine, State of"),
        ("am", "Palestine, State of"),
        ("an", "Palestine, State of"),
        ("ar", "فلسطين"),
        ("as", "Palestine, State of"),
        ("ay", "Palestine, State of"),
        ("az", "Palestine, State of"),
        ("ba", "Palestine, State of"),
        ("be", "Палесціна"),
        ("bg", "Палестина, държава"),
        ("bi", "Palestine, State of"),
        ("bn", "ফিলিস\u{9cd}তিন র\u{9be}ষ\u{9cd}ট\u{9cd}র"),
        ("bn_IN", "ফিলিস\u{9cd}তিন, র\u{9be}ষ\u{9cd}ট\u{9cd}র"),
        ("br", "Palestine, State of"),
        ("bs", "Palestine, State of"),
        ("ca", "Palestine, State of"),
        ("ce", "Palestine, State of"),
        ("ch", "Palestine, State of"),
        ("cs", "Palestinský stát"),
        ("cv", "Palestine, State of"),
        ("cy", "Palestina, Gwladwriaeth"),
        ("da", "Palæstina, staten"),
        ("de", "Palästina, Staat"),
        ("dv", "Palestine, State of"),
        ("dz", "Palestine, State of"),
        ("ee", "Palestine, State of"),
        ("el", "Παλαιστίνη"),
        ("en", "Palestine, State of"),
        ("eo", "Palestino, Ŝtato"),
        ("es", "Palestina, Estado de"),
        ("et", "Palestiina Riik"),
        ("eu", "Palestinako Estatua"),
        ("fa", "فلسطین"),
        ("ff", "Palestine, State of"),
        ("fi", "Palestine, State of"),
        ("fo", "Palestine, State of"),
        ("fr", "Palestine, État de"),
        ("fy", "Palestine, State of"),
        ("ga", "Palaistíne, Stát na"),
        ("gl", "Palestina, Estado de"),
        ("gn", "Palestine, State of"),
        ("gu", "પ\u{ac7}લ\u{ac7}સ\u{acd}ટાઇન, સ\u{acd}ટ\u{ac7}ટ ઓફ"),
        ("gv", "Palestine, State of"),
        ("ha", "Palestine, State of"),
        ("he", "פלסטין, מדינת"),
        ("hi", "प\u{948}ल\u{947}स\u{94d}टाइन, स\u{94d}ट\u{947}ट ऑफ़"),
        ("hr", "Palestina"),
        ("ht", "Palestine, State of"),
        ("hu", "Palesztina"),
        ("hy", "Պաղեստին, Պետություն"),
        ("ia", "Palestina"),
        ("id", "Negara Palestina"),
        ("io", "Palestine, State of"),
        ("is", "Palestína"),
        ("it", "Palestina, Stato di"),
        ("iu", "Palestine, State of"),
        ("ja", "パレスチナ"),
        ("ka", "Palestine, State of"),
        ("ki", "Palestine, State of"),
        ("kk", "Палестина мемлекеті"),
        ("kl", "Palestine, State of"),
        ("km", "រដ\u{17d2}ឋប\u{17c9}ាឡេស\u{17d2}ទ\u{17b8}ន"),
        ("kn", "Palestine, State of"),
        ("ko", "팔레스타인"),
        ("ku", "Palestine, State of"),
        ("kv", "Palestine, State of"),
        ("kw", "Palestine, State of"),
        ("ky", "Палестина Мамлекети"),
        ("lo", "Palestine, State of"),
        ("lt", "Palestinos valstybė"),
        ("lv", "Palestīnas valsts"),
        ("mi", "Palestine, State of"),
        ("mk", "Palestine, State of"),
        ("ml", "Palestine, State of"),
        ("mn", "Palestine, State of"),
        ("mr", "प\u{945}ल\u{947}स\u{94d}टाईन, राज\u{94d}य"),
        ("ms", "Palestine, State of"),
        ("mt", "Palestine, State of"),
        ("my", "Palestine, State of"),
        ("na", "Palestine, State of"),
        ("nb", "Palestina, staten"),
        ("ne", "Palestine, State of"),
        ("nl", "Palestina"),
        ("nn", "Palestina"),
        ("nv", "Palestine, State of"),
        ("oc", "Palestina, estat de"),
        ("or", "ପ\u{b3e}ଲେଷ\u{b4d}ଟ\u{b3e}ଇନ ର\u{b3e}ଜ\u{b4d}ୟ"),
        ("pa", "ਫਿਲਸਤੀਨ ਰਾਜ"),
        ("pi", "Palestine, State of"),
        ("pl", "Palestyna (państwo)"),
        ("ps", "Palestine, State of"),
        ("pt", "Palestina"),
        ("pt_BR", "Palestina, Estado da"),
        ("ro", "Palestina, Statul"),
        ("ru", "Палестина"),
        ("rw", "Palestine, State of"),
        ("sc", "Palestina, Istadu de"),
        ("sd", "Palestine, State of"),
        ("si", "Palestine, State of"),
        ("sk", "Palestína"),
        ("sl", "Palestina, država"),
        ("so", "Palestine, State of"),
        ("sq", "Palestinë, Shteti i"),
        ("sr", "Палестина, Држава"),
        ("sv", "Staten Palestina"),
        ("sw", "Palestine, State of"),
        ("ta", "Palestine, State of"),
        (
            "te",
            "ప\u{c3e}లస\u{c4d}త\u{c40}న\u{c4d}, స\u{c4d}ట\u{c47}ట\u{c4d} ఆఫ\u{c4d}",
        ),
        ("tg", "Ҷумҳурии Фаластин"),
        ("th", "ปาเลสไตน\u{e4c}, ร\u{e31}ฐ"),
        ("ti", "Palestine, State of"),
        ("tk", "Palestine, State of"),
        ("tl", "Palestine, State of"),
        ("tr", "Filistin Devleti"),
        ("tt", "Palestine, State of"),
        ("ug", "پەلەستىن دۆلىتى"),
        ("uk", "Палестина, Держава"),
        ("ur", "Palestine, State of"),
        ("uz", "Palestine, State of"),
        ("ve", "Palestine, State of"),
        ("vi", "Palestine, quốc gia"),
        ("wa", "Palestine, State of"),
        ("wo", "Palestine, State of"),
        ("xh", "Palestine, State of"),
        ("yo", "Palestine, State of"),
        ("zh_CN", "巴勒斯坦"),
        ("zh_HK", "巴勒斯坦"),
        ("zh_TW", "巴勒斯坦"),
        ("zu", "Palestine, State of"),
    ];
    #[cfg(all(feature = "ps", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 31.952162;
        pub const LONGITUDE: f64 = 35.233154;
        pub const MAX_LATITUDE: f64 = 32.5520999;
        pub const MAX_LONGITUDE: f64 = 35.5740521;
        pub const MIN_LATITUDE: f64 = 31.219691;
        pub const MIN_LONGITUDE: f64 = 34.21010010000001;
        pub const NORTHEAST_LATITUDE: f64 = 32.5520999;
        pub const NORTHEAST_LONGITUDE: f64 = 35.5740521;
        pub const SOUTHWEST_LATITUDE: f64 = 31.219691;
        pub const SOUTHWEST_LONGITUDE: f64 = 34.21010010000001;
    }
}
#[cfg(all(feature = "ps", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 31.952162,
            longitude: 35.233154,
            max_latitude: 32.5520999,
            max_longitude: 35.5740521,
            min_latitude: 31.219691,
            min_longitude: 34.21010010000001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 32.5520999,
                    longitude: 35.5740521,
                },
                southwest: CountryGeoBound {
                    latitude: 31.219691,
                    longitude: 34.21010010000001,
                },
            },
        }
    }
}

#[cfg(all(feature = "ps", feature = "subdivisions"))]
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
                    "BTH",
                    Subdivision{
                        name: "BTH",
                        country_alpha2: Alpha2::PS,
                        code: "BTH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بيت لحم"), ("be", "Віфлеем"), ("bn", "বেথেলহ\u{9be}ম গভর\u{9cd}নোরেট"), ("ca", "Governació de Betlem"), ("ccp", "𑄝𑄬𑄗𑄬𑄣\u{11134}𑄦𑄟\u{11134}"), ("da", "Bethlehem Governorate"), ("de", "Bethlehem Gouvernement"), ("el", "Βηθλεέμ Γκοβερνορατόρε"), ("en", "Bethlehem"), ("es", "Gobernación de Belén"), ("fa", "استان بیت\u{200c}لحم"), ("fi", "Bethlehemin kuvernoraatti"), ("fr", "Gouvernorat de Bethléem"), ("gu", "બ\u{ac7}થલ\u{ac7}હ\u{ac7}મ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "בית לחם"), ("hi", "ब\u{947}थल\u{947}हम गवर\u{94d}नर\u{947}ट"), ("hu", "Betlehem kormányzóság"), ("id", "Bethlehem Governorate"), ("it", "Governatorato di Betlemme"), ("ja", "ベツレヘム"), ("kn", "ಬ\u{cc6}ಥ\u{ccd} ಲ\u{cc6}ಹ\u{cc6}ಮ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "베들레헴 주"), ("lt", "Betliejaus muchafaza"), ("lv", "Bētlemes muhāfaza"), ("mr", "ब\u{947}थलह\u{947}म गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Bethlehem Governorate"), ("nb", "Bethlehem Governorate"), ("nl", "Bethlehem"), ("no", "Bethlehem Governorate"), ("pl", "Gubernatorstwo Betlejem"), ("pt", "Governamento de Belém"), ("ro", "Guvernoratul Betleem"), ("ru", "Вифлеем"), ("si", "බෙත\u{dca}ලෙහෙම\u{dca} පළ\u{dcf}ත"), ("sv", "Bethlehem Governorate"), ("ta", "பெத\u{bcd}லகேம\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "బ\u{c46}త\u{c4d}ల\u{c46}హ\u{c3e}మ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองเบธเลเฮม"), ("tr", "Bethlehem Yönetimi"), ("uk", "Вифлеєм (провінція)"), ("ur", "محافظہ بیت لحم"), ("vi", "Tỉnh Bethlehem"), ("zh", "伯利恒省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DEB",
                    Subdivision{
                        name: "DEB",
                        country_alpha2: Alpha2::PS,
                        code: "DEB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الوسطى"), ("bn", "দেইর আল-ব\u{9be}ল\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació de Deir al-Balah"), ("ccp", "𑄓𑄬𑄃\u{11128}𑄢\u{11134} 𑄃𑄣\u{11134} 𑄝𑄣𑄦\u{11134}"), ("ceb", "Deir Al Balah"), ("da", "Deir al-Balah Governorate"), ("de", "Gouvernement Dair al-Balah"), ("el", "Ντέιρ Αλ Μπαλάχ"), ("en", "Deir al-Balah"), ("es", "Gobernación de Deir el-Balah"), ("fa", "استان دیرالبلح"), ("fi", "Deir al-Balahn kuvernoraatti"), ("fr", "Gouvernorat de Deir Al-Balah"), ("gu", "દ\u{ac7}ઇર અલ-બાલાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "דיר אל-בלח (נפה)"), ("hi", "द\u{947}िर अल-बालाह गवर\u{94d}नर\u{947}ट"), ("hu", "Dejr el-Balah kormányzóság"), ("id", "Kegubernuran Deir al-Balah"), ("it", "Governatorato di Deir al-Balah"), ("ja", "ディール・バラフ"), ("kn", "ಡ\u{cbf}ಯರ\u{ccd} ಅಲ\u{ccd}-ಬಾಲಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "데이르알발라 주"), ("lt", "Deir al-Bachalo gubernija"), ("lv", "Deirelbelehas muhāfaza"), ("mr", "द\u{947}र अल-बालाह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Deir al-Balah Governorate"), ("nl", "Deir el-Balah"), ("pl", "Gubernatorstwo Deir al-Balah"), ("ps", "محافظه دير البلح"), ("ro", "Guvernoratul Deir al-Balah"), ("ru", "Дейр эль-Балах"), ("si", "ඩයර\u{dca} අල\u{dca}-බල\u{dcf} පළ\u{dcf}ත"), ("sv", "Deir al-Balah"), ("ta", "டேஇரு அல\u{bcd}-பல\u{bbe}ஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "డ\u{c46}య\u{c3f}ర\u{c4d} అల\u{c4d}-బల\u{c3e}హ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "แดร\u{e4c} อ\u{e31}ล บาลา"), ("tr", "Deir al-Balah Yönetimi"), ("uk", "Муніципалітет Дейр-ель-Балах"), ("ur", "محافظہ دیر البلح"), ("vi", "Tỉnh Deir al-Balah"), ("zh", "代尔拜莱赫省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GZA",
                    Subdivision{
                        name: "GZA",
                        country_alpha2: Alpha2::PS,
                        code: "GZA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة غزة"), ("ca", "Governació de Gaza"), ("ccp", "𑄉𑄎"), ("de", "Gouvernement Gaza"), ("en", "Gaza"), ("es", "Gobernación de Gaza"), ("fa", "استان غزه"), ("fr", "Gouvernorat de Gaza"), ("he", "עזה (נפה)"), ("hu", "Gáza kormányzóság"), ("id", "Kegubernuran Gaza"), ("it", "governatorato di Gaza"), ("ja", "ガザ県"), ("ko", "가자 주"), ("nl", "Gaza"), ("pl", "Gaza"), ("ps", "محافظه غزه"), ("ro", "Guvernoratul Gaza"), ("ru", "Газа"), ("tr", "Gazze valiliği"), ("uk", "мугафаза Ґаза"), ("ur", "محافظہ غزہ"), ("zh", "加沙省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "HBN",
                    Subdivision{
                        name: "HBN",
                        country_alpha2: Alpha2::PS,
                        code: "HBN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الخليل"), ("bn", "হেবরন গভর\u{9cd}নোরেট"), ("ca", "Governació d’Hebron"), ("ccp", "𑄦𑄬𑄝\u{11133}𑄢\u{11127}𑄚\u{11134}"), ("da", "Hebron Governorate"), ("de", "Hebron Governorate"), ("el", "Χέμπρον"), ("en", "Hebron"), ("es", "Gobernación de Hebrón"), ("fa", "استان الخلیل"), ("fi", "Hebronn kuvernoraatti"), ("fr", "Gouvernorat de Hébron"), ("gu", "હ\u{ac7}બ\u{acd}રોન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "חברון"), ("hi", "ह\u{947}ब\u{94d}रोन गवर\u{94d}नर\u{947}ट"), ("hu", "Hebron kormányzóság"), ("id", "Hebron Governorate"), ("it", "Governatorato di Hebron"), ("ja", "ヘブロン"), ("kn", "ಹ\u{cc6}ಬ\u{ccd}ರೋನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "헤브론 주"), ("lt", "Hebrono muchafaza"), ("lv", "Hebronas muhāfaza"), ("mr", "ह\u{947}ब\u{94d}रोन गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Hebron Governorate"), ("nb", "Herbron Governorate"), ("nl", "Hebron"), ("no", "Herbron Governorate"), ("pl", "Gubernatorstwo Hebron"), ("pt", "Governamento de Hebron"), ("ro", "Guvernoratul Hebron"), ("ru", "Хеврон"), ("si", "හෙබ\u{dca}\u{200d}රොන\u{dca} පළ\u{dcf}ත"), ("sv", "Herbron Governorate"), ("ta", "ஹெப\u{bcd}ரோன\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "హ\u{c46}బ\u{c4d}రన\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ฮ\u{e35}บรอน"), ("tr", "Hebron Valiliği"), ("uk", "Хеврон (провінція)"), ("ur", "محافظہ الخلیل"), ("vi", "Tỉnh Hebron"), ("zh", "希布伦省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JEM",
                    Subdivision{
                        name: "JEM",
                        country_alpha2: Alpha2::PS,
                        code: "JEM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "القدس"), ("be", "Іерусалім"), ("ca", "Governació de Jerusalem"), ("ccp", "𑄎𑄬𑄢\u{1112a}𑄎𑄣𑄬𑄟\u{11134}"), ("de", "Gouvernement Jerusalem"), ("el", "Κυβερνείο Ιερουσαλήμ"), ("en", "Jerusalem"), ("es", "Gobernación de Jerusalén"), ("fa", "استان قدس"), ("fr", "gouvernorat de Jérusalem"), ("he", "אל-קודס"), ("hu", "Jeruzsálem kormányzóság"), ("it", "Governatorato di Gerusalemme"), ("ja", "エルサレム県"), ("ko", "예루살렘 주"), ("lt", "Jeruzalės muchafaza"), ("nl", "Jeruzalem"), ("pl", "Jerozolima"), ("ro", "Guvernoratul Ierusalim"), ("ru", "Иерусалим"), ("sv", "Jerusalems provins"), ("ur", "محافظہ یروشلم"), ("zh", "耶路撒冷省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JEN",
                    Subdivision{
                        name: "JEN",
                        country_alpha2: Alpha2::PS,
                        code: "JEN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنين"), ("ca", "Governació de Jenin"), ("ccp", "𑄎𑄬𑄚\u{11128}𑄚\u{11134}"), ("de", "Jenin Governorate"), ("en", "Jenin"), ("es", "Gobernación de Yenín"), ("fa", "استان جنین"), ("fr", "Gouvernorat de Jénine"), ("he", "ג׳נין"), ("hu", "Dzsenín kormányzóság"), ("id", "Kegubernuran Jenin"), ("it", "Governatorato di Jenin"), ("ja", "ジェニーン"), ("ko", "제닌 주"), ("lt", "Dženino muchafaza"), ("nl", "Jenin"), ("pl", "Dżanin"), ("pt", "Província de Jenin"), ("ro", "Guvernoratul Jenin"), ("ru", "Джанин"), ("tr", "Cenin valiliği"), ("ur", "محافظہ جنین"), ("zh", "杰宁省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JRH",
                    Subdivision{
                        name: "JRH",
                        country_alpha2: Alpha2::PS,
                        code: "JRH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أريحا"), ("ca", "Governació de Jericó"), ("ccp", "𑄎𑄬𑄢\u{11128}𑄇\u{1112e}"), ("en", "Jericho"), ("es", "Gobernación de Jericó"), ("fa", "استان اریحا"), ("fr", "Gouvernorat de Jéricho"), ("he", "יריחו"), ("hu", "Jerikó kormányzóság"), ("it", "Governatorato di Gerico"), ("ja", "エリコ"), ("ko", "예리코 주"), ("lt", "Jericho muchafaza"), ("nl", "Jericho"), ("pl", "Jerycho"), ("ro", "Guvernoratul Ierihon"), ("ru", "Иерихон"), ("ur", "محافظہ اریحا"), ("zh", "杰里科省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KYS",
                    Subdivision{
                        name: "KYS",
                        country_alpha2: Alpha2::PS,
                        code: "KYS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة خان يونس"), ("ca", "Governació de Khan Yunis"), ("ccp", "𑄈𑄚\u{11134} 𑄃\u{11128}𑄅\u{1112a}𑄚\u{1112a}𑄌\u{11134}"), ("ceb", "Khan Yunis Governorate"), ("de", "Gouvernement Chan Yunis"), ("en", "Khan Yunis"), ("es", "Gobernación de Jan Yunis"), ("fa", "استان خان\u{200c}یونس"), ("fr", "Gouvernorat de Khan Younès"), ("he", "ח׳אן יונס (נפה)"), ("hu", "Hán Júnisz kormányzóság"), ("id", "Kegubernuran Khan Yunis"), ("it", "governatorato di Khan Yunis"), ("ja", "ハーン・ユーニス"), ("ko", "칸유니스 주"), ("nl", "Khan Yunis"), ("pl", "Chan Junus"), ("ps", "محافظه خان يونس"), ("ro", "Guvernoratul Khan Yunis"), ("ru", "Хан-Юнис"), ("tr", "Han Yunus valiliği"), ("ur", "محافظہ خان یونس"), ("zh", "汉尤尼斯省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NBS",
                    Subdivision{
                        name: "NBS",
                        country_alpha2: Alpha2::PS,
                        code: "NBS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة نابلس"), ("be", "Наблус"), ("bn", "ন\u{9be}ব\u{9cd}ল\u{9c1}স গভর\u{9cd}নোরেট"), ("ca", "Governació de Nablus"), ("ccp", "𑄚𑄛\u{11134}𑄣𑄌\u{11134}"), ("da", "Nablus Governorate"), ("de", "Gouvernement Nablus"), ("el", "Νάμπλους Γκοβερνοράτε"), ("en", "Nablus"), ("es", "Gobernación de Naplusa"), ("fa", "استان نابلس"), ("fi", "Nablusin kuvernoraatti"), ("fr", "Gouvernorat de Naplouse"), ("gu", "નાબલ\u{ac1}સ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "שכם"), ("hi", "नबलस गवर\u{94d}नर\u{947}ट"), ("hu", "Náblusz kormányzóság"), ("id", "Nablus Governorate"), ("it", "Governatorato di Nablus"), ("ja", "ナーブルス"), ("kn", "ನಬ\u{ccd}ಲಸ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "나블루스 주"), ("lt", "Nabluso muchafaza"), ("lv", "Nablusas muhāfaza"), ("mr", "नाबल\u{942}स गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Nablus Governorate"), ("nb", "Nablus Governorate"), ("nl", "Nablus"), ("no", "Nablus Governorate"), ("pl", "Gubernatorstwo Nablus"), ("pt", "Província de Nablus"), ("ro", "Guvernoratul Nablus"), ("ru", "Наблус"), ("si", "නබ\u{dca}ලස\u{dca} පළ\u{dcf}ත"), ("sv", "Nablus Governorate"), ("ta", "நப\u{bcd}லஸ\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "న\u{c3e}బ\u{c4d}లస\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองแนบล\u{e39}ส"), ("tr", "Nablus Yönetimi"), ("uk", "Муніципалітет Наблус"), ("ur", "محافظہ نابلس"), ("vi", "Tỉnh Nablus"), ("zh", "纳布卢斯省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NGZ",
                    Subdivision{
                        name: "NGZ",
                        country_alpha2: Alpha2::PS,
                        code: "NGZ",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شمال غزة"), ("bn", "উত\u{9cd}তর গ\u{9be}জ\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació de Gaza Nord"), ("ccp", "𑄅\u{11127}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄉𑄎"), ("da", "North Gaza Governorate"), ("de", "Gouvernement Nordgaza"), ("el", "Βόρεια Γάζα"), ("en", "North Gaza"), ("es", "Gobernación de Gaza del Norte"), ("fa", "استان شمال غزه"), ("fi", "North Gazan kuvernoraatti"), ("fr", "Gouvernorat de Gaza-Nord"), ("gu", "નોર\u{acd}થ ગાઝા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "צפון עזה (נפה)"), ("hi", "उत\u{94d}तर गाजा गवर\u{94d}नर\u{947}ट"), ("hu", "Észak-Gáza kormányzóság"), ("id", "Kegubernuran Gaza Utara"), ("it", "Governatorato di Nord Gaza"), ("ja", "北ガザ県"), ("kn", "ಉತ\u{ccd}ತರ ಗಾಜಾ ಗವರ\u{ccd}ನರ\u{ccd}"), ("ko", "북가자 주"), ("lt", "Šiaurinė Gazos gubernija"), ("lv", "Ziemeļgazas muhāfaza"), ("mr", "नॉर\u{94d}थ गाझा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "North Gaza Governorate"), ("nb", "Nord Gaza Governorate"), ("nl", "Noord Gaza"), ("no", "Nord Gaza Governorate"), ("pl", "Gubernatorstwo Północnej Gazy"), ("ps", "محافظه شمالي غزه"), ("pt", "Governamento Nord de Gaza"), ("ro", "Guvernoratul Gaza de Nord"), ("ru", "Северная Газа"), ("si", "උත\u{dd4}ර\u{dd4} ග\u{dcf}ස\u{dcf} පළ\u{dcf}ත"), ("sv", "Norra Gaza Governorate"), ("ta", "வடக\u{bcd}கு க\u{bbe}ச\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "ఉత\u{c4d}తర గ\u{c3e}జ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "กาซาเหน\u{e37}อ"), ("tr", "Kuzey Gazze valiliği"), ("uk", "Муніципалітет Північна Газа"), ("ur", "محافظہ شمالی غزہ"), ("vi", "Tỉnh Bắc Gaza"), ("zh", "北加沙省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "QQA",
                    Subdivision{
                        name: "QQA",
                        country_alpha2: Alpha2::PS,
                        code: "QQA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة قلقيلية"), ("bn", "ক\u{9be}ল\u{9cd}কিলিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació de Qalqilya"), ("ccp", "𑄇\u{1112a}𑄠𑄣\u{11134}𑄇\u{1112a}𑄣\u{11128}𑄠"), ("da", "Qalqilya Governorate"), ("de", "Gouvernement Qalqilya"), ("el", "Καλκίλγια"), ("en", "Qalqilya"), ("es", "Gobernación de Calquelia"), ("fa", "استان قلقیلیه"), ("fi", "Qalqilyan kuvernoraatti"), ("fr", "Gouvernorat de Qalqilya"), ("gu", "ક\u{ac7}લકીલ\u{acd}યા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "קלקיליה"), ("hi", "कालकिया गवर\u{94d}नर\u{947}ट"), ("hu", "Kalkílija kormányzóság"), ("id", "Kegubernuran Qalqilya"), ("it", "Governatorato di Qalqilya"), ("ja", "カルキーリヤ"), ("kn", "ಕಲ\u{ccd}ಕ\u{cbf}ಲ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "칼킬리야 주"), ("lt", "Kalkiljos muchafaza"), ("lv", "Kalkīljas muhāfaza"), ("mr", "क\u{945}लकियला गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Qalqilya Governorate"), ("nb", "Qalqilya"), ("nl", "Qalqilya"), ("no", "Qalqilya"), ("pl", "Gubernatorstwo Qalqilya"), ("pt", "Qalqilya"), ("ro", "Guvernoratul Qalqilya"), ("ru", "Калькилия"), ("si", "කල\u{dca}ක\u{dd2}ල\u{dca}\u{200d}ය\u{dcf} පළ\u{dcf}ත"), ("sv", "Qalqilya"), ("ta", "கிய\u{bbe}ழக\u{bcd}கிலிய கோவெர\u{bcd}னோரே"), ("te", "ఖ\u{c3e}ల\u{c4d}గ\u{c3f}ల\u{c4d}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "คาลค\u{e34}ลยา"), ("tr", "Qalqilya District"), ("uk", "Муніципалітет Калькілія"), ("ur", "محافظہ قلقیلیہ"), ("vi", "Tỉnh Qalqilya"), ("zh", "盖勒吉利耶省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RBH",
                    Subdivision{
                        name: "RBH",
                        country_alpha2: Alpha2::PS,
                        code: "RBH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "رام الله والبيرة"), ("bn", "র\u{9be}ম\u{9be}ল\u{9cd}ল\u{9be}হ ও আল-বিরেহ গভর\u{9cd}নোরেট"), ("ca", "Governació de Ramal·lah i al-Bireh"), ("ccp", "𑄢𑄟𑄣\u{11133}𑄣𑄦\u{11134} 𑄃\u{11133}𑄃 𑄃𑄣\u{11134}-𑄝\u{11128}𑄢𑄬𑄦\u{11134}"), ("da", "Ramallah and al-Bireh Governorate"), ("de", "Gouvernement Ramallah und al-Bireh"), ("el", "Ραμάλα και αλ-Μπίρεχ"), ("en", "Ramallah and al-Bireh"), ("es", "Gobernación de Ramala y Al Bireh"), ("fa", "استان رام\u{200c}الله و البیره"), ("fi", "Ramallah ja al-Birehn kuvernoraatti"), ("fr", "Gouvernorat de Ramallah et Al-Bireh"), ("gu", "રામલ\u{acd}લાહ એન\u{acd}ડ અલ-બિર\u{ac7}હ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "נפת רמאללה ואל-בירה"), ("hi", "रामल\u{94d}लाह और अल-बिर\u{947}ह गवर\u{94d}नर\u{947}ट"), ("hu", "Rámalláh és el-Bíra kormányzóság"), ("id", "Ramallah and al-Bireh Governorate"), ("it", "Governatorato di Ramallah e al-Bireh"), ("kn", "ರಾಮಾಲ\u{ccd}ಲಾಹ\u{ccd} ಮತ\u{ccd}ತು ಅಲ\u{ccd}-ಬೈರ\u{cc6} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "라말라알비레 주"), ("lt", "Ramalos ir Biros muchafaza"), ("lv", "Rāmallas un Bīras muhāfaza"), ("mr", "रामाल\u{94d}ला अ\u{901}ड अल-बिर\u{947}हच\u{947} गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ramallah and al-Bireh Governorate"), ("nb", "Ramallah og al-Bireh"), ("nl", "Ramallah & Al-Bireh"), ("no", "Ramallah og al-Bireh"), ("pl", "Gubernatorstwo Ramallah and al-Bireh"), ("pt", "Ramallah og al-Bireh"), ("ro", "Guvernoratul Ramallah ṣi al-Bireh"), ("ru", "Рамалла и эль-Бира"), ("si", "රමල\u{dca}ල\u{dcf} සහ අල\u{dca}-බ\u{dd2}රේ පළ\u{dcf}ත"), ("sv", "Ramallah och al-Bireh"), ("ta", "ர\u{bbe}மல\u{bcd}ல\u{bbe}ஹ\u{bcd} அண\u{bcd}ட\u{bcd} அல\u{bcd} -பிரெஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ర\u{c3e}మల\u{c4d}ల\u{c3e} మర\u{c3f}యు అల\u{c4d}-బ\u{c3f}ర\u{c47}హ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ราม\u{e31}ลลาฮ\u{e4c} แอนด\u{e4c} อ\u{e31}ล ไบเรฮ\u{e4c} โกเวอโนเรท"), ("tr", "Ramallah ve al-Bireh Valiliği"), ("uk", "Муніципалітет Рамалла та ель-Біра"), ("ur", "محافظہ رام الله اور البیرہ"), ("vi", "Tỉnh Ramallah và al-Bireh"), ("zh", "拉马拉和比雷赫省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RFH",
                    Subdivision{
                        name: "RFH",
                        country_alpha2: Alpha2::PS,
                        code: "RFH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة رفح الفلسطينية"), ("ca", "Governació de Rafah"), ("ccp", "𑄢𑄜𑄦\u{11134}"), ("de", "Gouvernement Rafah"), ("en", "Rafah"), ("es", "Gobernación de Rafah"), ("et", "Rafaḩi kubernerkond"), ("fa", "استان رفح"), ("fr", "Gouvernorat de Rafah"), ("he", "רפיח"), ("hu", "Rafah kormányzóság"), ("id", "Kegubernuran Rafah"), ("it", "governatorato di Rafah"), ("ja", "ラファハ"), ("ko", "라파 주"), ("nl", "Rafah"), ("pl", "Rafah"), ("ps", "محافظه رفح"), ("ro", "Guvernoratul Rafah"), ("ru", "Рафах"), ("uk", "Рафах"), ("ur", "محافظہ رفح"), ("zh", "拉法省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SLT",
                    Subdivision{
                        name: "SLT",
                        country_alpha2: Alpha2::PS,
                        code: "SLT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سلفيت"), ("bn", "স\u{9be}লফিত গভর\u{9cd}নোরেট"), ("ca", "Governació de Salfit"), ("ccp", "𑄥𑄣\u{11134}𑄜\u{11128}𑄖\u{11134}"), ("da", "Salfit Governorate"), ("de", "Gouvernement Salfit"), ("el", "Σαλφίτ Γκοβερνοράτε"), ("en", "Salfit"), ("es", "Gobernación de Salfit"), ("fa", "استان سلفیت"), ("fi", "Salfitin kuvernoraatti"), ("fr", "Gouvernorat de Salfit"), ("gu", "સ\u{ac7}લ\u{acd}ફિટ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "סלפית"), ("hi", "साल\u{94d}फीट गवर\u{94d}नर\u{947}ट"), ("hu", "Szalfít kormányzóság"), ("id", "Salfit Governorate"), ("it", "Governatorato di Salfit"), ("ja", "サルフィート"), ("kn", "ಸಫೀಟ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "살피트 주"), ("lt", "Salfito muchafaza"), ("lv", "Selfītas muhāfaza"), ("mr", "सलट\u{947}ट गव\u{94d}हनर\u{94d}ट\u{947}ट"), ("ms", "Salfit Governorate"), ("nb", "Salfit Governorate"), ("nl", "Salfit"), ("no", "Salfit Governorate"), ("pl", "Gubernatorstwo Salfit"), ("pt", "Província de Salfit"), ("ro", "Guvernoratul Salfit"), ("ru", "Сальфит"), ("si", "සල\u{dca}ෆ\u{dd2}ට\u{dca} පළ\u{dcf}ත"), ("sv", "Salfit Governorate"), ("ta", "ச\u{bbe}லபிட\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "స\u{c3e}ల\u{c4d}ఫ\u{c3f}ట\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ฟรอเร\u{e35}ยนา"), ("tr", "Salfint Yönetimi"), ("uk", "Муніципалітет Салфіт"), ("ur", "محافظہ سلفیت"), ("vi", "Tỉnh Salfit"), ("zh", "萨尔费特省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TBS",
                    Subdivision{
                        name: "TBS",
                        country_alpha2: Alpha2::PS,
                        code: "TBS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة طوباس"), ("ca", "Governació de Tubas"), ("ccp", "𑄑\u{1112a}𑄝𑄌\u{11134}"), ("en", "Tubas"), ("es", "Gobernación de Tubas"), ("fa", "استان طوباس"), ("fr", "Gouvernorat de Tubas"), ("he", "טובאס"), ("hu", "Túbász kormányzóság"), ("id", "Kegubernuran Tubas"), ("it", "governatorato di Tubas"), ("ja", "トゥーバース"), ("ko", "투바스 주"), ("lt", "Tubaso muchafaza"), ("nl", "Tubas"), ("pl", "Tubas"), ("pt", "Tubas Governorate"), ("ro", "Guvernoratul Tubas"), ("ru", "Тубас"), ("tr", "Tubas valiliği"), ("ur", "محافظہ طوباس"), ("zh", "图巴斯省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TKM",
                    Subdivision{
                        name: "TKM",
                        country_alpha2: Alpha2::PS,
                        code: "TKM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة طولكرم"), ("bn", "ত\u{9c1}ল\u{9cd}ক\u{9be}রম গভর\u{9cd}নোরেট"), ("ca", "Governació de Tulkarem"), ("ccp", "𑄑\u{1112a}𑄣\u{11134}𑄇𑄢\u{11134}𑄟\u{11134}"), ("da", "Tulkarm Governorate"), ("de", "Gouvernement Tulkarm"), ("el", "Τουλκάρμ"), ("en", "Tulkarm"), ("es", "Gobernación de Tulcarem"), ("fa", "استان طولکرم"), ("fi", "Tulkarmin kuvernoraatti"), ("fr", "Gouvernorat de Tulkarem"), ("gu", "ટ\u{ac1}લકાર\u{acd}મ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "טולכרם"), ("hi", "त\u{941}लकार\u{94d}म गवर\u{94d}नर\u{947}ट"), ("hu", "Túlkarm kormányzóság"), ("id", "Tulkarm Governorate"), ("it", "Governatorato di Tulkarm"), ("ja", "トゥールカリム"), ("kn", "ತುಲ\u{ccd}ಕಾರಮ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "툴카름 주"), ("lt", "Tulkarmo muchafaza"), ("lv", "Tūlkarmas muhāfaza"), ("mr", "त\u{941}ळकर\u{94d}म गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Tulkarm Governorate"), ("nb", "Tulkarm"), ("nl", "Tulkarm"), ("no", "Tulkarm"), ("pl", "Gubernatorstwo Tulkarm"), ("pt", "Tulkarm"), ("ro", "Guvernoratul Tulkarm"), ("ru", "Тулькарм"), ("si", "ට\u{dd4}ල\u{dca}ක\u{dcf}ර\u{dca}ම\u{dca} පළ\u{dcf}ත"), ("sv", "Tulkarm"), ("ta", "துல\u{bcd}க\u{bbe}ர\u{bcd}ம\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "టుల\u{c4d}క\u{c3e}మ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ท\u{e31}ลคาม โกเวอโนเรท"), ("tr", "Tulkarm Yönetimi"), ("uk", "Муніципалітет Тулькарм"), ("ur", "محافظہ طولکرم"), ("vi", "Tỉnh Tulkarm"), ("zh", "图勒凯尔姆省")]),
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
#[cfg(feature = "ps")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PS,
        alpha3: Alpha3::PSE,
        address_format: None,
        continent: Continent::Asia,
        country_code: 970,
        currency_code: CurrencyCode::ILS,
        gec: Some(GEC::WE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::PLE),
        iso_long_name: "The State of Palestine",
        iso_short_name: "Palestine, State of",
        official_language_list: ["ar", "en", "he"].to_vec(),
        spoken_language_list: ["ar", "en", "he"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Palestinian"),
        number: "275",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "PS",
        unofficial_name_list: [
            "Palestine",
            "فلسطين",
            "Palästina",
            "Palestina",
            "the Occupied Palestinian Territory",
            "パレスチナ",
            "Palestijnse gebieden",
            "Palestinian Territory Occupied",
            "Palestinian Authority",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Palestine, State of"),
            ("af", "Palestine, State of"),
            ("ak", "Palestine, State of"),
            ("am", "Palestine, State of"),
            ("an", "Palestine, State of"),
            ("ar", "فلسطين"),
            ("as", "Palestine, State of"),
            ("ay", "Palestine, State of"),
            ("az", "Palestine, State of"),
            ("ba", "Palestine, State of"),
            ("be", "Палесціна"),
            ("bg", "Палестина, държава"),
            ("bi", "Palestine, State of"),
            ("bn", "ফিলিস\u{9cd}তিন র\u{9be}ষ\u{9cd}ট\u{9cd}র"),
            ("bn_IN", "ফিলিস\u{9cd}তিন, র\u{9be}ষ\u{9cd}ট\u{9cd}র"),
            ("br", "Palestine, State of"),
            ("bs", "Palestine, State of"),
            ("ca", "Palestine, State of"),
            ("ce", "Palestine, State of"),
            ("ch", "Palestine, State of"),
            ("cs", "Palestinský stát"),
            ("cv", "Palestine, State of"),
            ("cy", "Palestina, Gwladwriaeth"),
            ("da", "Palæstina, staten"),
            ("de", "Palästina, Staat"),
            ("dv", "Palestine, State of"),
            ("dz", "Palestine, State of"),
            ("ee", "Palestine, State of"),
            ("el", "Παλαιστίνη"),
            ("en", "Palestine, State of"),
            ("eo", "Palestino, Ŝtato"),
            ("es", "Palestina, Estado de"),
            ("et", "Palestiina Riik"),
            ("eu", "Palestinako Estatua"),
            ("fa", "فلسطین"),
            ("ff", "Palestine, State of"),
            ("fi", "Palestine, State of"),
            ("fo", "Palestine, State of"),
            ("fr", "Palestine, État de"),
            ("fy", "Palestine, State of"),
            ("ga", "Palaistíne, Stát na"),
            ("gl", "Palestina, Estado de"),
            ("gn", "Palestine, State of"),
            ("gu", "પ\u{ac7}લ\u{ac7}સ\u{acd}ટાઇન, સ\u{acd}ટ\u{ac7}ટ ઓફ"),
            ("gv", "Palestine, State of"),
            ("ha", "Palestine, State of"),
            ("he", "פלסטין, מדינת"),
            ("hi", "प\u{948}ल\u{947}स\u{94d}टाइन, स\u{94d}ट\u{947}ट ऑफ़"),
            ("hr", "Palestina"),
            ("ht", "Palestine, State of"),
            ("hu", "Palesztina"),
            ("hy", "Պաղեստին, Պետություն"),
            ("ia", "Palestina"),
            ("id", "Negara Palestina"),
            ("io", "Palestine, State of"),
            ("is", "Palestína"),
            ("it", "Palestina, Stato di"),
            ("iu", "Palestine, State of"),
            ("ja", "パレスチナ"),
            ("ka", "Palestine, State of"),
            ("ki", "Palestine, State of"),
            ("kk", "Палестина мемлекеті"),
            ("kl", "Palestine, State of"),
            ("km", "រដ\u{17d2}ឋប\u{17c9}ាឡេស\u{17d2}ទ\u{17b8}ន"),
            ("kn", "Palestine, State of"),
            ("ko", "팔레스타인"),
            ("ku", "Palestine, State of"),
            ("kv", "Palestine, State of"),
            ("kw", "Palestine, State of"),
            ("ky", "Палестина Мамлекети"),
            ("lo", "Palestine, State of"),
            ("lt", "Palestinos valstybė"),
            ("lv", "Palestīnas valsts"),
            ("mi", "Palestine, State of"),
            ("mk", "Palestine, State of"),
            ("ml", "Palestine, State of"),
            ("mn", "Palestine, State of"),
            ("mr", "प\u{945}ल\u{947}स\u{94d}टाईन, राज\u{94d}य"),
            ("ms", "Palestine, State of"),
            ("mt", "Palestine, State of"),
            ("my", "Palestine, State of"),
            ("na", "Palestine, State of"),
            ("nb", "Palestina, staten"),
            ("ne", "Palestine, State of"),
            ("nl", "Palestina"),
            ("nn", "Palestina"),
            ("nv", "Palestine, State of"),
            ("oc", "Palestina, estat de"),
            ("or", "ପ\u{b3e}ଲେଷ\u{b4d}ଟ\u{b3e}ଇନ ର\u{b3e}ଜ\u{b4d}ୟ"),
            ("pa", "ਫਿਲਸਤੀਨ ਰਾਜ"),
            ("pi", "Palestine, State of"),
            ("pl", "Palestyna (państwo)"),
            ("ps", "Palestine, State of"),
            ("pt", "Palestina"),
            ("pt_BR", "Palestina, Estado da"),
            ("ro", "Palestina, Statul"),
            ("ru", "Палестина"),
            ("rw", "Palestine, State of"),
            ("sc", "Palestina, Istadu de"),
            ("sd", "Palestine, State of"),
            ("si", "Palestine, State of"),
            ("sk", "Palestína"),
            ("sl", "Palestina, država"),
            ("so", "Palestine, State of"),
            ("sq", "Palestinë, Shteti i"),
            ("sr", "Палестина, Држава"),
            ("sv", "Staten Palestina"),
            ("sw", "Palestine, State of"),
            ("ta", "Palestine, State of"),
            (
                "te",
                "ప\u{c3e}లస\u{c4d}త\u{c40}న\u{c4d}, స\u{c4d}ట\u{c47}ట\u{c4d} ఆఫ\u{c4d}",
            ),
            ("tg", "Ҷумҳурии Фаластин"),
            ("th", "ปาเลสไตน\u{e4c}, ร\u{e31}ฐ"),
            ("ti", "Palestine, State of"),
            ("tk", "Palestine, State of"),
            ("tl", "Palestine, State of"),
            ("tr", "Filistin Devleti"),
            ("tt", "Palestine, State of"),
            ("ug", "پەلەستىن دۆلىتى"),
            ("uk", "Палестина, Держава"),
            ("ur", "Palestine, State of"),
            ("uz", "Palestine, State of"),
            ("ve", "Palestine, State of"),
            ("vi", "Palestine, quốc gia"),
            ("wa", "Palestine, State of"),
            ("wo", "Palestine, State of"),
            ("xh", "Palestine, State of"),
            ("yo", "Palestine, State of"),
            ("zh_CN", "巴勒斯坦"),
            ("zh_HK", "巴勒斯坦"),
            ("zh_TW", "巴勒斯坦"),
            ("zu", "Palestine, State of"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
