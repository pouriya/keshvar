// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The State of Kuwait

#[cfg(all(feature = "kw", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::KW;
    pub const ALPHA3: Alpha3 = Alpha3::KWT;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 965;
    pub const CURRENCY_CODE: &str = "KWD";
    pub const GEC: Option<GEC> = Some(GEC::KU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::KUW);
    pub const ISO_SHORT_NAME: &str = "Kuwait";
    pub const ISO_LONG_NAME: &str = "The State of Kuwait";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Kuwaiti");
    pub const NUMBER: &str = "414";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "KW";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Kuwait", "الكويت", "Koweït", "クウェート", "Koeweit"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Kuwait"),
        ("af", "Koeweit"),
        ("ak", "Kuwait"),
        ("am", "ጤፃት"),
        ("an", "Kuwait"),
        ("ar", "الكويت"),
        ("as", "ক\u{9c1}ৱেইট"),
        ("ay", "Kuwait"),
        ("az", "Küveyt"),
        ("ba", "Kuwait"),
        ("be", "Кувейт"),
        ("bg", "Кувейт"),
        ("bi", "Kuwait"),
        ("bn", "ক\u{9c1}য়েত"),
        ("bn_IN", "ক\u{9c1}য়েত"),
        ("br", "Koweit"),
        ("bs", "Kuvajt"),
        ("ca", "Kuwait"),
        ("ce", "Кувейт"),
        ("ch", "Kuwait"),
        ("cs", "Kuvajt"),
        ("cv", "Кувейт"),
        ("cy", "Coweit"),
        ("da", "Kuwait"),
        ("de", "Kuwait"),
        ("dv", "ކ\u{7aa}ވ\u{7ac}އ\u{7a8}ތ\u{7aa}"),
        ("dz", "ཀ\u{f74}་ཝ\u{f7a}ཊ\u{f72}།"),
        ("ee", "Kuwait"),
        ("el", "Κουβέιτ"),
        ("en", "Kuwait"),
        ("eo", "Kuvajto"),
        ("es", "Kuwait"),
        ("et", "Kuveit"),
        ("eu", "Kuwait"),
        ("fa", "کویت"),
        ("ff", "Kuwait"),
        ("fi", "Kuwait"),
        ("fo", "Kuvait"),
        ("fr", "Koweït"),
        ("fy", "Koeweit"),
        ("ga", "Cuáit"),
        ("gl", "Kuvait"),
        ("gn", "Kuwait"),
        ("gu", "ક\u{ac1}વ\u{ac8}ત"),
        ("gv", "Yn Choowait"),
        ("ha", "Kuwait"),
        ("he", "כווית"),
        ("hi", "क\u{941}व\u{948}त"),
        ("hr", "Kuvajt"),
        ("ht", "Kowet"),
        ("hu", "Kuvait"),
        ("hy", "Քուվեյթ"),
        ("ia", "Kuwait"),
        ("id", "Kuwait"),
        ("io", "Kuwait"),
        ("is", "Kúveit"),
        ("it", "Kuwait"),
        ("iu", "Kuwait"),
        ("ja", "クウェート"),
        ("ka", "კუვეიტი"),
        ("ki", "Kuwait"),
        ("kk", "Кувейт"),
        ("kl", "Kuwait"),
        ("km", "គ\u{17bb}យវ\u{17c9}ែត"),
        ("kn", "ಕುವೈತ\u{ccd}"),
        ("ko", "쿠웨이트"),
        ("ku", "Kûveyt"),
        ("kv", "Кувейт"),
        ("kw", "Koweyt"),
        ("ky", "Кувейт"),
        ("lo", "Kuwait"),
        ("lt", "Kuveitas"),
        ("lv", "Kuveita"),
        ("mi", "Kuwait"),
        ("mk", "Кувајт"),
        ("ml", "ക\u{d41}വൈത\u{d4d}ത\u{d4d}"),
        ("mn", "Кувейт"),
        ("mr", "क\u{941}व\u{948}त"),
        ("ms", "Kuwait"),
        ("mt", "Kuwajt"),
        (
            "my",
            "က\u{1030}ဝ\u{102d}တ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Kuwait"),
        ("nb", "Kuwait"),
        ("ne", "क\u{941}व\u{947}त"),
        ("nl", "Koeweit"),
        ("nn", "Kuwait"),
        ("nv", "Kuwait"),
        ("oc", "Koweit"),
        ("or", "କ\u{b41}ଏତ"),
        ("pa", "ਕ\u{a41}ਵ\u{a48}ਤ"),
        ("pi", "क\u{941}व\u{948}त"),
        ("pl", "Kuwejt"),
        ("ps", "کویټ"),
        ("pt", "Kuwait"),
        ("pt_BR", "Kuwait"),
        ("ro", "Kuweit"),
        ("ru", "Кувейт"),
        ("rw", "Koweti"),
        ("sc", "Kuwait"),
        ("sd", "Kuwait"),
        ("si", "ක\u{dd4}වේට\u{dca}"),
        ("sk", "Kuvajt"),
        ("sl", "Kuvajt"),
        ("so", "Kuwayt"),
        ("sq", "Kuvajt"),
        ("sr", "Кувајт"),
        ("sv", "Kuwait"),
        ("sw", "Kuwait"),
        ("ta", "குவைத\u{bcd}"),
        ("te", "కువ\u{c48}ట\u{c4d}"),
        ("tg", "Қувайт"),
        ("th", "ค\u{e39}เวต"),
        ("ti", "ክዌት"),
        ("tk", "Kuweýt"),
        ("tl", "Kuwait"),
        ("tr", "Kuveyt"),
        ("tt", "Күвәйт"),
        ("ug", "كۇۋەيت"),
        ("uk", "Кувейт"),
        ("ur", "کویت"),
        ("uz", "Quvayt"),
        ("ve", "Kuwait"),
        ("vi", "Cu-ouai-thợ"),
        ("wa", "Kuweyt"),
        ("wo", "Kuwet"),
        ("xh", "Kuwait"),
        ("yo", "Kuwaiti"),
        ("zh_CN", "科威特"),
        ("zh_HK", "科威特"),
        ("zh_TW", "科威特"),
        ("zu", "Kuwait"),
    ];
    #[cfg(all(feature = "kw", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 29.31166;
        pub const LONGITUDE: f64 = 47.481766;
        pub const MAX_LATITUDE: f64 = 30.1036993;
        pub const MAX_LONGITUDE: f64 = 48.5184;
        pub const MIN_LATITUDE: f64 = 28.5244463;
        pub const MIN_LONGITUDE: f64 = 46.55303989999999;
        pub const NORTHEAST_LATITUDE: f64 = 30.1036993;
        pub const NORTHEAST_LONGITUDE: f64 = 48.5184;
        pub const SOUTHWEST_LATITUDE: f64 = 28.5244463;
        pub const SOUTHWEST_LONGITUDE: f64 = 46.55303989999999;
    }
}
#[cfg(all(feature = "kw", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 29.31166,
            longitude: 47.481766,
            max_latitude: 30.1036993,
            max_longitude: 48.5184,
            min_latitude: 28.5244463,
            min_longitude: 46.55303989999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 30.1036993,
                    longitude: 48.5184,
                },
                southwest: CountryGeoBound {
                    latitude: 28.5244463,
                    longitude: 46.55303989999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "kw", feature = "subdivisions"))]
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
                    "AH",
                    Subdivision{
                        name: "AH",
                        country_alpha2: Alpha2::KW,
                        code: "AH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.083333), longitude: Some(48.083333), max_latitude: Some(29.1115398), min_latitude: Some(29.0593959), max_longitude: Some(48.0997057), min_longitude: Some(48.0440711)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الأحمدي"), ("bg", "Ал-Ахмади"), ("bn", "আল আহম\u{9be}দি গভর\u{9cd}নোরেট"), ("ca", "Governació d’Ahmadí"), ("ccp", "𑄃𑄣\u{11134} 𑄃𑄦\u{11134}𑄟\u{11127}𑄘\u{11128}"), ("da", "Al Ahmadi Governorate"), ("de", "Gouvernement Ahmadi"), ("el", "Αλ Αχμάντι Γκοβερνοράτε"), ("en", "Al Ahmadi"), ("es", "Ahmadí"), ("fa", "استان احمدی"), ("fi", "Al Ahmadin kuvernoraatti"), ("fr", "Al Ahmadi (gouvernorat)"), ("gu", "અલ અહમદી ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अल अहमदी गवर\u{94d}नर\u{947}ट"), ("hu", "Ahmadi kormányzóság"), ("id", "Kegubernuran Al-Ahmadi"), ("it", "Al-Ahmad"), ("ja", "アハマディ県"), ("kn", "ಅಲ\u{ccd} ಅಹ\u{ccd}ಮದ\u{cbf} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "알아마디 주"), ("lt", "Al Achmadžio muchafaza"), ("lv", "Ahmedi muhāfaza"), ("mr", "अल अहमदी गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Ahmadi Governorate"), ("nb", "Al Ahmadi Governorate"), ("nl", "Ahmadi"), ("no", "Al Ahmadi Governorate"), ("pl", "Al-Ahmadi (muhafaza)"), ("pt", "Al Ahmadi (província)"), ("ru", "Эль-Ахмади"), ("si", "අල\u{dca} අහමද\u{dd2} පළ\u{dcf}ත"), ("ta", "அல\u{bcd} அஹ\u{bcd}மடி கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "అల\u{c4d} అహ\u{c4d}మద\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลอาห\u{e4c}มาด\u{e35}"), ("tr", "El Ahmedî Valiliği"), ("uk", "Муніципалітет Ель-Ахмаді"), ("ur", "محافظہ احمدی"), ("vi", "Tỉnh Al Ahmadi"), ("zh", "艾哈迈迪省")]),
                        unofficial_name_list: ["Ahmadi"].to_vec(),
                    }
                ),
                (
                    "FA",
                    Subdivision{
                        name: "FA",
                        country_alpha2: Alpha2::KW,
                        code: "FA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.2428445), longitude: Some(47.9416114), max_latitude: Some(29.305629), min_latitude: Some(29.180285), max_longitude: Some(48.0104), min_longitude: Some(47.8128219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الفروانية"), ("bg", "Фаруания"), ("bn", "আল ফ\u{9be}রব\u{9be}নিয\u{9bc}\u{9be}হ গভর\u{9cd}নোরেট"), ("ccp", "𑄃𑄣\u{11134} 𑄜𑄢\u{11134}𑄤𑄚\u{11128}𑄠𑄦\u{11134}"), ("da", "Al Farwaniyah Governorate"), ("de", "Gouvernement Al Farwaniya"), ("el", "Αλ Φαρβανιγιά Γκοβερνοράτε"), ("en", "Al Farwaniyah"), ("es", "Al Farwaniyah"), ("fa", "استان فروانیه"), ("fi", "Al Farwaniyahn kuvernoraatti"), ("fr", "Al Farwaniyah"), ("gu", "અલ ફરવાનિયાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "अल फरवानिया गवर\u{94d}नर\u{947}ट"), ("hu", "Farvánijja kormányzóság"), ("hy", "Էլ Ֆարվանիյահ"), ("id", "Kegubernuran Al-Farwaniyah"), ("it", "Al-Farwaniyah"), ("ja", "ファルワーニーヤ県"), ("kn", "ಅಲ\u{ccd} ಫರ\u{ccd}ವಾನ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "알파르와니야 주"), ("lt", "Al Farvanijos muchafaza"), ("lv", "Farvānijas muhāfaza"), ("mr", "अल फरविनिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Farwaniyah Governorate"), ("nb", "Al Farwaniyah"), ("nl", "Farwaniya"), ("no", "Al Farwaniyah"), ("pl", "Al-Farwanijja"), ("pt", "Al Farwaniyah"), ("ru", "Эль-Фарвания"), ("si", "අල\u{dca} ෆව\u{dcf}න\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sv", "Al Farwaniyah"), ("ta", "அல\u{bcd} ப\u{bbe}ர\u{bcd}வ\u{bbe}னிய கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} ఫర\u{c4d}వ\u{c3e}న\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตอ\u{e31}ลฟาร\u{e4c}วะน\u{e35}ยะห\u{e4c}"), ("tr", "El Fervaniye Valiliği"), ("uk", "Меніципалітет Ель-Фарванія"), ("ur", "محافظہ فروانیہ"), ("vi", "Tỉnh Al Farwaniyah"), ("zh", "費爾瓦尼耶省")]),
                        unofficial_name_list: ["Farwaniyah"].to_vec(),
                    }
                ),
                (
                    "HA",
                    Subdivision{
                        name: "HA",
                        country_alpha2: Alpha2::KW,
                        code: "HA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.333), longitude: Some(48.029), max_latitude: Some(29.34970509999999), min_latitude: Some(29.3230583), max_longitude: Some(48.0381103), min_longitude: Some(48.0009217)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حولي"), ("bg", "Хавалли"), ("bn", "হ\u{9be}ওয\u{9bc}\u{9be}ল\u{9cd}যি গভর\u{9cd}নোরেট"), ("ccp", "𑄦𑄤𑄣\u{11128}"), ("da", "Hawalli Governorate"), ("de", "Gouvernement Hawalli"), ("el", "Χαβάλι"), ("en", "Hawalli"), ("es", "Hawalli"), ("fa", "استان حولی"), ("fi", "Hawalli"), ("fr", "Hawalli"), ("gu", "હવાલી ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "हवाली गवर\u{94d}नर\u{947}ट"), ("hr", "Hawalli"), ("hu", "Havalli kormányzóság"), ("hy", "Հավալի"), ("id", "Kegubernuran Hawalli"), ("it", "Hawalli"), ("ja", "ハワリ県"), ("kn", "ಹವಾಲ\u{cbf} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "하왈리 주"), ("lt", "Havalio muchafaza"), ("lv", "Havalli muhāfaza"), ("mr", "ह\u{941}वली गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Hawalli Governorate"), ("nb", "Hawalli"), ("nl", "Hawalli"), ("no", "Hawalli"), ("pl", "Hawalli"), ("pt", "Hawalli"), ("ru", "Хавалли"), ("si", "හව\u{dcf}ය\u{dd2} පළ\u{dcf}ත"), ("ta", "ஹவல\u{bcd}லி கோவெர\u{bcd}னோரே"), ("te", "హవ\u{c3e}ల\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตฮะว\u{e31}ลล\u{e35}"), ("tr", "Hawalli Yönetimi"), ("uk", "Хаваллі"), ("ur", "محافظہ حولی"), ("vi", "Tỉnh Hawalli"), ("zh", "哈瓦利省")]),
                        unofficial_name_list: ["Hawalli"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::KW,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.3365728), longitude: Some(47.6755291), max_latitude: Some(29.3745747), min_latitude: Some(29.2811337), max_longitude: Some(47.7926787), min_longitude: Some(47.6289382)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الجهراء"), ("bg", "Джахра"), ("ca", "Governació d’Al Jahra"), ("ccp", "𑄃𑄣\u{11134} 𑄎𑄦\u{11134}𑄢"), ("de", "Gouvernement al-Dschahra"), ("en", "Al Jahra"), ("es", "Yahra"), ("fa", "استان جهرا"), ("fr", "Al Jahra"), ("hr", "Al Jahra"), ("hu", "Dzsahrá kormányzóság"), ("hy", "Էլ Ջահրա"), ("id", "Kegubernuran Al-Jahra"), ("it", "Al-Jahra"), ("ja", "ジャハラー県"), ("ko", "알자라 주"), ("lt", "Al Džahros muchafaza"), ("nl", "Jahra"), ("pl", "Al-Dżahra"), ("pt", "Al Jahra"), ("ru", "Эль-Джахра"), ("tr", "El Cehrâ"), ("ur", "محافظہ جہراء"), ("zh", "傑赫拉省")]),
                        unofficial_name_list: ["Al Jahra", "Jahra", "Jahrah", "al-Jahra", "al-Jahraʿ"].to_vec(),
                    }
                ),
                (
                    "KU",
                    Subdivision{
                        name: "KU",
                        country_alpha2: Alpha2::KW,
                        code: "KU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.375859), longitude: Some(47.9774052), max_latitude: Some(29.3933479), min_latitude: Some(29.3587818), max_longitude: Some(48.0058081), min_longitude: Some(47.95796259999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة العاصمة"), ("bg", "Ал-Асима"), ("ccp", "𑄃𑄣\u{11134} 𑄃𑄥\u{11128}𑄟𑄦\u{11134}"), ("ceb", "Al Asimah Governorate"), ("el", "Κυβερνείο Πρωτεύουσας"), ("en", "Al Asimah"), ("es", "Capital"), ("fa", "استان عاصمه"), ("fr", "Al Asimah"), ("hr", "Al Asimah"), ("hu", "Főváros kormányzóság (Kuvait)"), ("hy", "Ալ Ասիմա"), ("id", "Kegubernuran Al-Asimah"), ("it", "Governatorato della Capitale"), ("ja", "アースィマ県"), ("ko", "알아시마 주"), ("lt", "Al Asimos muchafaza"), ("mr", "अल असिमाह गव\u{94d}हर\u{94d}नर\u{947}ट"), ("nb", "Al Asimah"), ("nl", "Al-Asimah"), ("no", "Al Asimah"), ("pl", "Al-Asima"), ("pt", "Al Asimah"), ("ru", "Эль-Асима"), ("sv", "Al Asimahguvernementet"), ("tr", "El Asime Valiliği"), ("ur", "محافظہ العاصمہ (کویت)"), ("zh", "科威特省")]),
                        unofficial_name_list: ["Capital", "Koweit", "Kuwait", "Kuwayt", "al-Kuwayt"].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::KW,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.21224), longitude: Some(48.0605108), max_latitude: Some(29.2734889), min_latitude: Some(29.172518), max_longitude: Some(48.1176184), min_longitude: Some(47.983255)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مبارك الكبير"), ("bg", "Мубарак ал-Кабир"), ("bn", "ম\u{9c1}ব\u{9be}রক আল-ক\u{9be}বির গভর\u{9cd}নোরেট"), ("ccp", "𑄟\u{1112a}𑄝𑄢\u{11127}𑄇\u{11134} 𑄃𑄣\u{11134}-𑄇𑄝\u{11128}𑄢\u{11134}"), ("da", "Mubarak Al-Kabeer Governorate"), ("de", "Gouvernement Mubarak Al-Kabeer"), ("el", "Μουμπάρακ Αλ-Καμπίρ"), ("en", "Mubarak Al-Kabeer"), ("es", "Gobernación de Mubarak el Grande"), ("fa", "استان مبارک\u{200c}الکبیر"), ("fi", "Mubarak Al-Kabeern kuvernoraatti"), ("fr", "Gouvernorat de Mubarak Al-Kabeer"), ("gu", "મ\u{ac1}બારક અલ-કબીર ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "म\u{941}बारक अल-कबीर गवर\u{94d}नर\u{947}ट"), ("hr", "Mubarak Al-Kabeer"), ("hu", "Mubárak el-Kabír kormányzóság"), ("hy", "Մուբարաք Էլ Քաբիր"), ("id", "Kegubernuran Mubarak Al-Kabeer"), ("it", "Mobarak al-Kabir"), ("kn", "ಮುಬಾರಕ\u{ccd} ಅಲ\u{ccd}-ಕಬೀರ\u{ccd} ಗವರ\u{ccd}ನರ\u{ccd}"), ("ko", "무바라크알카비르 주"), ("lt", "Mubarak Al Kabiro muchafaza"), ("lv", "Mubarāk el Kebīras muhāfaza"), ("mr", "म\u{941}बारक अल-कबीर गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Mubarak Al-Kabeer Governorate"), ("nb", "Mubarak Al-Kabeer"), ("nl", "Mubarak Al-Kabier"), ("no", "Mubarak Al-Kabeer"), ("pl", "Mubarak al-Kabir"), ("pt", "Mubarak Al-Kabeer"), ("ru", "Мубарак аль-Кабир"), ("si", "ම\u{dd4}බ\u{dcf}රක\u{dca} අල\u{dca}-කබ\u{dd3}ර\u{dca} පළ\u{dcf}ත"), ("ta", "முப\u{bbe}ரக\u{bcd} அழ -கப\u{bc0}ர\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ముబ\u{c3e}రక\u{c4d} అల\u{c4d}-కబ\u{c40}ర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ม\u{e39}บาร\u{e31}ค อ\u{e31}ล คาเบ\u{e35}ย"), ("tr", "Mübarek El Kebir Valiliği"), ("uk", "Муніципалітет Мубарак-аль-Кабір"), ("ur", "محافظہ مبارک الکبیر"), ("vi", "Tỉnh Mubarak Al-Kabeer"), ("zh", "大穆巴拉克省")]),
                        unofficial_name_list: ["Mubarak al-Kabir"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "kw")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::KW,
        alpha3: Alpha3::KWT,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 965,
        currency_code: "KWD",
        gec: Some(GEC::KU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::KUW),
        iso_long_name: "The State of Kuwait",
        iso_short_name: "Kuwait",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Kuwaiti"),
        number: "414",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "KW",
        unofficial_name_list: ["Kuwait", "الكويت", "Koweït", "クウェート", "Koeweit"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Kuwait"),
            ("af", "Koeweit"),
            ("ak", "Kuwait"),
            ("am", "ጤፃት"),
            ("an", "Kuwait"),
            ("ar", "الكويت"),
            ("as", "ক\u{9c1}ৱেইট"),
            ("ay", "Kuwait"),
            ("az", "Küveyt"),
            ("ba", "Kuwait"),
            ("be", "Кувейт"),
            ("bg", "Кувейт"),
            ("bi", "Kuwait"),
            ("bn", "ক\u{9c1}য়েত"),
            ("bn_IN", "ক\u{9c1}য়েত"),
            ("br", "Koweit"),
            ("bs", "Kuvajt"),
            ("ca", "Kuwait"),
            ("ce", "Кувейт"),
            ("ch", "Kuwait"),
            ("cs", "Kuvajt"),
            ("cv", "Кувейт"),
            ("cy", "Coweit"),
            ("da", "Kuwait"),
            ("de", "Kuwait"),
            ("dv", "ކ\u{7aa}ވ\u{7ac}އ\u{7a8}ތ\u{7aa}"),
            ("dz", "ཀ\u{f74}་ཝ\u{f7a}ཊ\u{f72}།"),
            ("ee", "Kuwait"),
            ("el", "Κουβέιτ"),
            ("en", "Kuwait"),
            ("eo", "Kuvajto"),
            ("es", "Kuwait"),
            ("et", "Kuveit"),
            ("eu", "Kuwait"),
            ("fa", "کویت"),
            ("ff", "Kuwait"),
            ("fi", "Kuwait"),
            ("fo", "Kuvait"),
            ("fr", "Koweït"),
            ("fy", "Koeweit"),
            ("ga", "Cuáit"),
            ("gl", "Kuvait"),
            ("gn", "Kuwait"),
            ("gu", "ક\u{ac1}વ\u{ac8}ત"),
            ("gv", "Yn Choowait"),
            ("ha", "Kuwait"),
            ("he", "כווית"),
            ("hi", "क\u{941}व\u{948}त"),
            ("hr", "Kuvajt"),
            ("ht", "Kowet"),
            ("hu", "Kuvait"),
            ("hy", "Քուվեյթ"),
            ("ia", "Kuwait"),
            ("id", "Kuwait"),
            ("io", "Kuwait"),
            ("is", "Kúveit"),
            ("it", "Kuwait"),
            ("iu", "Kuwait"),
            ("ja", "クウェート"),
            ("ka", "კუვეიტი"),
            ("ki", "Kuwait"),
            ("kk", "Кувейт"),
            ("kl", "Kuwait"),
            ("km", "គ\u{17bb}យវ\u{17c9}ែត"),
            ("kn", "ಕುವೈತ\u{ccd}"),
            ("ko", "쿠웨이트"),
            ("ku", "Kûveyt"),
            ("kv", "Кувейт"),
            ("kw", "Koweyt"),
            ("ky", "Кувейт"),
            ("lo", "Kuwait"),
            ("lt", "Kuveitas"),
            ("lv", "Kuveita"),
            ("mi", "Kuwait"),
            ("mk", "Кувајт"),
            ("ml", "ക\u{d41}വൈത\u{d4d}ത\u{d4d}"),
            ("mn", "Кувейт"),
            ("mr", "क\u{941}व\u{948}त"),
            ("ms", "Kuwait"),
            ("mt", "Kuwajt"),
            (
                "my",
                "က\u{1030}ဝ\u{102d}တ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Kuwait"),
            ("nb", "Kuwait"),
            ("ne", "क\u{941}व\u{947}त"),
            ("nl", "Koeweit"),
            ("nn", "Kuwait"),
            ("nv", "Kuwait"),
            ("oc", "Koweit"),
            ("or", "କ\u{b41}ଏତ"),
            ("pa", "ਕ\u{a41}ਵ\u{a48}ਤ"),
            ("pi", "क\u{941}व\u{948}त"),
            ("pl", "Kuwejt"),
            ("ps", "کویټ"),
            ("pt", "Kuwait"),
            ("pt_BR", "Kuwait"),
            ("ro", "Kuweit"),
            ("ru", "Кувейт"),
            ("rw", "Koweti"),
            ("sc", "Kuwait"),
            ("sd", "Kuwait"),
            ("si", "ක\u{dd4}වේට\u{dca}"),
            ("sk", "Kuvajt"),
            ("sl", "Kuvajt"),
            ("so", "Kuwayt"),
            ("sq", "Kuvajt"),
            ("sr", "Кувајт"),
            ("sv", "Kuwait"),
            ("sw", "Kuwait"),
            ("ta", "குவைத\u{bcd}"),
            ("te", "కువ\u{c48}ట\u{c4d}"),
            ("tg", "Қувайт"),
            ("th", "ค\u{e39}เวต"),
            ("ti", "ክዌት"),
            ("tk", "Kuweýt"),
            ("tl", "Kuwait"),
            ("tr", "Kuveyt"),
            ("tt", "Күвәйт"),
            ("ug", "كۇۋەيت"),
            ("uk", "Кувейт"),
            ("ur", "کویت"),
            ("uz", "Quvayt"),
            ("ve", "Kuwait"),
            ("vi", "Cu-ouai-thợ"),
            ("wa", "Kuweyt"),
            ("wo", "Kuwet"),
            ("xh", "Kuwait"),
            ("yo", "Kuwaiti"),
            ("zh_CN", "科威特"),
            ("zh_HK", "科威特"),
            ("zh_TW", "科威特"),
            ("zu", "Kuwait"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
