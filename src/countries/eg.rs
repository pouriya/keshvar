// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Arab Republic of Egypt

#[cfg(all(feature = "eg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::EG;
    pub const ALPHA3: Alpha3 = Alpha3::EGY;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 20;
    pub const CURRENCY_CODE: &str = "EGP";
    pub const GEC: Option<GEC> = Some(GEC::EG);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("EGY");
    pub const ISO_SHORT_NAME: &str = "Egypt";
    pub const ISO_LONG_NAME: &str = "The Arab Republic of Egypt";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Egyptian");
    pub const NUMBER: &str = "818";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAfrica);
    pub const UN_LOCODE: &str = "EG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Egypt",
        "مصر",
        "Ägypten",
        "Égypte",
        "Egipto",
        "エジプト",
        "Egypte",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Egypt"),
        ("af", "Egipte"),
        ("ak", "Egypt"),
        ("am", "ጔብፄ"),
        ("an", "Egypt"),
        ("ar", "مصر"),
        ("as", "মিশ\u{9cd}ৰ"),
        ("ay", "Egypt"),
        ("az", "Misir"),
        ("ba", "Egypt"),
        ("be", "Егіпет"),
        ("bg", "Египет"),
        ("bi", "Egypt"),
        ("bn", "মিশর"),
        ("bn_IN", "মিশর"),
        ("br", "Egipt"),
        ("bs", "Egipat"),
        ("ca", "Egipte"),
        ("ce", "Мисар"),
        ("ch", "Egypt"),
        ("cs", "Egypt"),
        ("cv", "Мисар"),
        ("cy", "Yr Aifft"),
        ("da", "Egypten"),
        ("de", "Ägypten"),
        ("dv", "މ\u{7a8}ޞ\u{7b0}ރ\u{7aa}"),
        ("dz", "ཨ\u{f72}་ཇ\u{f72}བཊ\u{f72}།"),
        ("ee", "Egypt"),
        ("el", "Αίγυπτος"),
        ("en", "Egypt"),
        ("eo", "Egiptio"),
        ("es", "Egipto"),
        ("et", "Egiptus"),
        ("eu", "Egipto"),
        ("fa", "مصر"),
        ("ff", "Egypt"),
        ("fi", "Egypti"),
        ("fo", "Egyptaland"),
        ("fr", "Égypte"),
        ("fy", "Egypte"),
        ("ga", "An Éigipt"),
        ("gl", "Exipto"),
        ("gn", "Egypt"),
        ("gu", "ઇજીપ\u{acd}ત"),
        ("gv", "Yn Egypt"),
        ("ha", "Misra"),
        ("he", "מצרים"),
        ("hi", "मिस\u{94d}र"),
        ("hr", "Egipat"),
        ("ht", "Ejip"),
        ("hu", "Egyiptom"),
        ("hy", "Եգիպտոս"),
        ("ia", "Egypto"),
        ("id", "Mesir"),
        ("io", "Egiptia"),
        ("is", "Egyptaland"),
        ("it", "Egitto"),
        ("iu", "Egypt"),
        ("ja", "エジプト"),
        ("ka", "ეგვიპტე"),
        ("ki", "Egypt"),
        ("kk", "Мысыр"),
        ("kl", "Egypt"),
        ("km", "អេហ\u{17d2}ស\u{17ca}\u{17b8}ប"),
        ("kn", "ಈಜ\u{cbf}ಪ\u{ccd}ಟ\u{ccd}"),
        ("ko", "이집트"),
        ("ku", "Misir"),
        ("kv", "Египет Араб Республика"),
        ("kw", "Ejyp"),
        ("ky", "Египет"),
        ("lo", "Egypt"),
        ("lt", "Egiptas"),
        ("lv", "Ēģipte"),
        ("mi", "Egypt"),
        ("mk", "Египет"),
        ("ml", "ഈജിപ\u{d4d}ത\u{d4d}"),
        ("mn", "Египт"),
        ("mr", "इजिप\u{94d}त"),
        ("ms", "Mesir"),
        ("mt", "Eġittu"),
        (
            "my",
            "အ\u{102e}ဂျစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Idjipt"),
        ("nb", "Egypt"),
        ("ne", "इजिप\u{94d}ट"),
        ("nl", "Egypte"),
        ("nn", "Egypt"),
        ("nv", "Egypt"),
        ("oc", "Egipte"),
        ("or", "ମ\u{b3f}ଶର"),
        ("pa", "ਮਿਸਰ"),
        ("pi", "ईजिप\u{94d}ट"),
        ("pl", "Egipt"),
        ("ps", "مصر"),
        ("pt", "Egito"),
        ("pt_BR", "Egito"),
        ("ro", "Egipt"),
        ("ru", "Египет"),
        ("rw", "Misiri"),
        ("sc", "Egitu"),
        ("sd", "مصر"),
        ("si", "ඊජ\u{dd2}ප\u{dca}ත\u{dd4}ව"),
        ("sk", "Egypt"),
        ("sl", "Egipt"),
        ("so", "Masar"),
        ("sq", "Egjipt"),
        ("sr", "Египат"),
        ("sv", "Egypten"),
        ("sw", "Egypt"),
        ("ta", "எகிப\u{bcd}து"),
        ("te", "ఈజ\u{c3f}ప\u{c4d}ట\u{c4d}"),
        ("tg", "Миср"),
        ("th", "อ\u{e35}ย\u{e34}ปต\u{e4c}"),
        ("ti", "ግብጺ"),
        ("tk", "Egypt"),
        ("tl", "Ehipto"),
        ("tr", "Mısır"),
        ("tt", "Мысыр"),
        ("ug", "مىسىر"),
        ("uk", "Єгипет"),
        ("ur", "مصر"),
        ("uz", "Misr"),
        ("ve", "Egypt"),
        ("vi", "Ai Cập"),
        ("wa", "Edjipe"),
        ("wo", "Ejipt"),
        ("xh", "Egypt"),
        ("yo", "Ẹ\u{301}gíptì"),
        ("zh_CN", "埃及"),
        ("zh_HK", "埃及"),
        ("zh_TW", "埃及"),
        ("zu", "IGibhithe"),
    ];
    #[cfg(all(feature = "eg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 26.820553;
        pub const LONGITUDE: f64 = 30.802498;
        pub const MAX_LATITUDE: f64 = 31.8122;
        pub const MAX_LONGITUDE: f64 = 37.0569;
        pub const MIN_LATITUDE: f64 = 21.9999999;
        pub const MIN_LONGITUDE: f64 = 24.696775;
        pub const NORTHEAST_LATITUDE: f64 = 31.8122;
        pub const NORTHEAST_LONGITUDE: f64 = 37.0569;
        pub const SOUTHWEST_LATITUDE: f64 = 21.9999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 24.696775;
    }
}
#[cfg(all(feature = "eg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 26.820553,
            longitude: 30.802498,
            max_latitude: 31.8122,
            max_longitude: 37.0569,
            min_latitude: 21.9999999,
            min_longitude: 24.696775,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 31.8122,
                    longitude: 37.0569,
                },
                southwest: CountryGeoBound {
                    latitude: 21.9999999,
                    longitude: 24.696775,
                },
            },
        }
    }
}

#[cfg(all(feature = "eg", feature = "subdivisions"))]
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
                    "ALX",
                    Subdivision{
                        name: "ALX",
                        country_alpha2: Alpha2::EG,
                        code: "ALX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.2000924), longitude: Some(29.9187387), max_latitude: Some(31.330904), min_latitude: Some(31.1173177), max_longitude: Some(30.0864016), min_longitude: Some(29.8233701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الإسكندرية"), ("be", "Мухафаза Александрыя"), ("bg", "Александрия"), ("bn", "আলেক\u{9cd}স\u{9be}ন\u{9cd}দ\u{9cd}রিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Aleksandrija"), ("ca", "Governació d’Alexandria"), ("ccp", "𑄃𑄣𑄬𑄇\u{11134}𑄎𑄚\u{11134}𑄓\u{11133}𑄢\u{11128}𑄠"), ("cs", "Alexandrie"), ("da", "Alexandria"), ("de", "al-Iskandariyya"), ("el", "Κυβερνείο Αλεξάνδρειας"), ("en", "Alexandria"), ("es", "Alejandría"), ("et", "Aleksandria kubernerkond"), ("fa", "استان اسکندریه"), ("fi", "Al-Iskandariyyan kuvernoraatti"), ("fr", "Gouvernorat d’Alexandrie"), ("gu", "એલ\u{ac7}ક\u{acd}ઝાન\u{acd}ડ\u{acd}રિયા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אלכסנדריה"), ("hi", "सिकन\u{94d}दरिया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Alexandria kormányzóság"), ("hy", "Ալեքսանդրիա"), ("id", "Kegubernuran Al Iskandariyah"), ("it", "Governatorato di Alessandria"), ("ja", "アレクサンドリア県"), ("ka", "ალექსანდრიის მუჰაფაზა"), ("kn", "ಅಲ\u{cc6}ಕ\u{ccd}ಸಾಂಡ\u{ccd}ರ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "알렉산드리아 주"), ("lt", "Aleksandrijos muchafaza"), ("lv", "Aleksandrijas muhāfaza"), ("mr", "अल\u{947}ग\u{94d}ज\u{93c}\u{945}\u{902}ड\u{94d}रिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Alexandria Governorate"), ("nb", "Guvernementet Al Iskandariyah"), ("nl", "Alexandrië"), ("no", "Guvernementet Al Iskandariyah"), ("pl", "Aleksandria"), ("pt", "Alexandria (província egípcia)"), ("ro", "Al Iskandariyah"), ("ru", "Александрия"), ("si", "ඇලෙක\u{dca}සැන\u{dca}ඩ\u{dca}\u{200d}ර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Александрија"), ("sr_Latn", "Aleksandrija"), ("sv", "Guvernementet Alexandria"), ("sw", "Mkoa wa Aleksandria"), ("ta", "அலெக\u{bcd}ச\u{bbe}ண\u{bcd}ட\u{bcd}ரிய\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "ఆల\u{c46}క\u{c4d}స\u{c3e}ండ\u{c4d}ర\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลอ\u{e34}สก\u{e31}นดาร\u{e34}ยาห\u{e4c}"), ("tr", "İskenderiye"), ("uk", "Александрія"), ("ur", "محافظہ اسکندریہ"), ("vi", "Alexandria"), ("zh", "亞歷山大省")]),
                        unofficial_name_list: ["Alexandria", "Alexandria", "Alexandrie", "El Iskandariya", "al-Iskandariyah", "al-Iskandarīyah"].to_vec(),
                    }
                ),
                (
                    "ASN",
                    Subdivision{
                        name: "ASN",
                        country_alpha2: Alpha2::EG,
                        code: "ASN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.088938), longitude: Some(32.8998293), max_latitude: Some(24.1131931), min_latitude: Some(24.0714761), max_longitude: Some(32.9207739), min_longitude: Some(32.8794385)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أسوان"), ("be", "Мухафаза Асуан"), ("bg", "Асуан"), ("bs", "Asuan"), ("ca", "Governació d’Assuan"), ("ccp", "𑄃𑄥\u{1112e}𑄠𑄚\u{11134}"), ("cs", "Asuán"), ("da", "Aswan"), ("de", "Aswan"), ("el", "Κυβερνείο Ασουάν"), ("en", "Aswan"), ("es", "Asuán"), ("et", "Aswāni kubernerkond"), ("fa", "استان اسوان"), ("fi", "Assuanin kuvernoraatti"), ("fr", "Gouvernorat d’Assouan"), ("hi", "असवान म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Asszuán kormányzóság"), ("hy", "Ասուան"), ("id", "Kegubernuran Aswan"), ("it", "Governatorato di Assuan"), ("ja", "アスワン県"), ("ka", "ასუანის მუჰაფაზა"), ("ko", "아스완 주"), ("lt", "Asuano muchafaza"), ("nb", "Aswan"), ("nl", "Aswan"), ("no", "Aswan"), ("pl", "Asuan"), ("pt", "Assuão"), ("ro", "Guvernoratul Aswan"), ("ru", "Асуан"), ("sr", "Асуан"), ("sr_Latn", "Asuan"), ("sv", "Guvernementet Assuan"), ("sw", "Mkoa wa Aswan"), ("tr", "Asvan"), ("uk", "Асуан"), ("ur", "محافظہ اسوان"), ("vi", "Aswan"), ("zh", "阿斯旺省")]),
                        unofficial_name_list: ["Assouan", "Assuan", "Aswān"].to_vec(),
                    }
                ),
                (
                    "AST",
                    Subdivision{
                        name: "AST",
                        country_alpha2: Alpha2::EG,
                        code: "AST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.1783117), longitude: Some(31.1859257), max_latitude: Some(27.1960162), min_latitude: Some(27.1580509), max_longitude: Some(31.2121548), min_longitude: Some(31.1566813)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أسيوط"), ("be", "Мухафаза Асьют"), ("bg", "Асют"), ("bn", "আস\u{9c1}ত গভর\u{9cd}নোরেট"), ("bs", "Asyut"), ("ca", "Governació d’Asyut"), ("ccp", "𑄃𑄥\u{1112d}𑄠\u{1112a}𑄖\u{11134}"), ("cs", "Asijút"), ("da", "Asyut"), ("de", "Asyut"), ("el", "Κυβερνείο Ασιούτ"), ("en", "Asyut"), ("es", "Asiut"), ("et", "Asyūţi kubernerkond"), ("fa", "استان اسیوط"), ("fi", "Asyutin kuvernoraatti"), ("fr", "Gouvernorat d’Assiout"), ("gu", "અસ\u{acd}ય\u{ac1}ત ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אסיוט"), ("hi", "असय\u{942}त म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Aszjút kormányzóság"), ("hy", "Ասյութ"), ("id", "Asyut Governorate"), ("it", "Governatorato di Asyut"), ("ja", "アシュート県"), ("ka", "ასიუტის მუჰაფაზა"), ("kn", "ಅಸ\u{ccd}ಯುತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "아시우트 주"), ("lt", "Asjuto muchafaza"), ("lv", "Asjūtas muhāfaza"), ("mr", "अस\u{94d}य\u{941}त गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Asyut Governorate"), ("nb", "Guvernementet Asyut"), ("nl", "Assioet"), ("no", "Guvernementet Asyut"), ("pl", "Asjut"), ("pt", "Assiut"), ("ro", "Guvernoratul Asyut"), ("ru", "Асьют"), ("si", "අස\u{dca}ය\u{dd4}ට\u{dca} පළ\u{dcf}ත"), ("sr", "Асјут"), ("sr_Latn", "Asjut"), ("sv", "Guvernementet Asyut"), ("sw", "Mkoa wa Asyut"), ("ta", "அஸ\u{bcd}யூட\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "అస\u{c4d}యుత\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}สย\u{e39}ต"), ("tr", "Asyut"), ("uk", "Асьют"), ("ur", "محافظہ اسیوط"), ("vi", "Tỉnh Asyut"), ("zh", "艾斯尤特省")]),
                        unofficial_name_list: ["Asiut", "Assiout", "Assiut", "Assyût", "Asyūţ", "Siut"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::EG,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.1076838), longitude: Some(33.7964613), max_latitude: Some(29.3923378), min_latitude: Some(21.9996377), max_longitude: Some(36.8945141), min_longitude: Some(30.741445)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "البحر الأحمر"), ("bg", "Червено море"), ("bn", "লোহিত স\u{9be}গর গভর\u{9cd}নোরেট"), ("bs", "Crveno more"), ("ca", "Governació de la Mar Roja"), ("ccp", "𑄢𑄬𑄖\u{11134}-𑄥\u{11128}"), ("ceb", "Red Sea Governorate"), ("cs", "Al-Bahri al-Ahmari"), ("da", "Al-Bahr al-Ahmar"), ("de", "al-Bahr al-ahmar"), ("el", "Κυβερνείο Ερυθράς Θάλασσας"), ("en", "Red Sea"), ("es", "Mar Rojo"), ("et", "Al-Baḩr al-Aḩmari kubernerkond"), ("fa", "استان بحرالاحمر"), ("fi", "Al-Bahr al-Ahmar"), ("fr", "Gouvernorat de la Mer-Rouge"), ("gu", "ર\u{ac7}ડ સી ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "הים האדום (מחוז)"), ("hi", "लाल सागर म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Կարմիր ծով"), ("id", "Kegubernuran Al-Bahr al-Ahmar"), ("it", "Governatorato del Mar Rosso"), ("ja", "紅海県"), ("ka", "წითელი ზღვის მუჰაფაზა"), ("kn", "ಕ\u{cc6}ಂಪು ಸಮುದ\u{ccd}ರ ಗವರ\u{ccd}ನರ\u{ccd}"), ("ko", "홍해 주"), ("lt", "Raudonosios Jūros muchafaza"), ("lv", "Sarkanās jūras muhāfaza"), ("mr", "र\u{947}ड सी गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Red Sea"), ("nb", "Guvernementet Al-Bahr al-Ahmar"), ("nl", "Rode Zee"), ("no", "Guvernementet Al-Bahr al-Ahmar"), ("pl", "Prowincja Morza Czerwonego"), ("pt", "Mar Vermelho"), ("ro", "Guvernoratul Al Bahr al Ahmar"), ("ru", "Красное Море"), ("si", "රත\u{dd4} ම\u{dd4}හ\u{dd4}ද පළ\u{dcf}ත"), ("sk", "Al-Bahr al-Ahmar"), ("sr", "Црвено море"), ("sr_Latn", "Crveno more"), ("sv", "Guvernementet Al-Bahr al-Ahmar"), ("sw", "Mkoa wa Bahari ya Shamu"), ("ta", "ரெட\u{bcd} செபட கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "ర\u{c46}డ\u{c4d} స\u{c40} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองเรดซ\u{e35}"), ("tr", "Kızıldeniz"), ("uk", "Червоне Море"), ("ur", "محافظہ بحر احمر"), ("vi", "Biển Đỏ"), ("zh", "红海省")]),
                        unofficial_name_list: ["El Bahr el Ahmar"].to_vec(),
                    }
                ),
                (
                    "BH",
                    Subdivision{
                        name: "BH",
                        country_alpha2: Alpha2::EG,
                        code: "BH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.8480986), longitude: Some(30.3435506), max_latitude: Some(31.4683007), min_latitude: Some(29.9339286), max_longitude: Some(30.8513688), min_longitude: Some(29.5338788)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة البحيرة"), ("be", "Мухафаза Бухейра"), ("bg", "Бухайра"), ("bn", "বেহির\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Beheira"), ("ca", "Governació de Buhayra"), ("ccp", "𑄝𑄬𑄦𑄬𑄃\u{11128}𑄢"), ("ceb", "Beheira Governorate"), ("cs", "Buhajra"), ("da", "Al Buhayrah"), ("de", "al-Buhaira"), ("el", "Κυβερνείο Μπεχέιρα"), ("en", "Beheira"), ("es", "Behera"), ("et", "Al-Buḩayrāti kubernerkond"), ("fa", "استان بحیره"), ("fi", "Al-Buhaira"), ("fr", "Gouvernorat de Beheira"), ("gu", "બીહ\u{ac7}ઇરા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אל-בחירה"), ("hi", "ब\u{947}ह\u{947}इरा म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hr", "Beheira"), ("hy", "Բուհեյրա"), ("id", "Kegubernuran Al Buhayrah"), ("it", "Governatorato di Buhayra"), ("ja", "ブハイラ県"), ("ka", "ბეჰეირის მუჰაფაზა"), ("kn", "ಬ\u{cc6}ಹ\u{cbf}ರಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "베헤이라 주"), ("lt", "Beheiros muchafaza"), ("lv", "Buheirātas muhāfaza"), ("mr", "ब\u{947}ह\u{947}रा गव\u{94d}हरन\u{947}ट"), ("ms", "Beheira Governorate"), ("nb", "Guvernementet Al Buhayrah"), ("nl", "Al Buhayrah"), ("no", "Guvernementet Al Buhayrah"), ("pl", "Al-Buhajra"), ("pt", "Al-Buhaira"), ("ro", "Guvernoratul Beheira"), ("ru", "Бухейра"), ("si", "බෙහෙර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Al-Buhajra"), ("sr", "Бухејра"), ("sr_Latn", "Buhejra"), ("sv", "Guvernementet Beheira"), ("sw", "Mkoa wa Beheira"), ("ta", "பெஹெயிர\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "బ\u{c46}హ\u{c40}ర\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เบเฮยรา โกเวอโรเนท"), ("tr", "Buheyre"), ("uk", "Бухейра"), ("ur", "محافظہ بحیرہ"), ("vi", "Tỉnh Beheira"), ("zh", "布海拉省")]),
                        unofficial_name_list: ["El Buhayra", "al-Buh\u{328}ayrah"].to_vec(),
                    }
                ),
                (
                    "BNS",
                    Subdivision{
                        name: "BNS",
                        country_alpha2: Alpha2::EG,
                        code: "BNS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.0661274), longitude: Some(31.0993845), max_latitude: Some(29.0958994), min_latitude: Some(29.0058847), max_longitude: Some(31.1423418), min_longitude: Some(31.0616271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بني سويف"), ("be", "Мухафаза Бені-Суэйф"), ("bg", "Бени Суеф"), ("bn", "ব\u{9be}নি স\u{9c1}য\u{9bc}েফ গভর\u{9cd}নোরেট"), ("bs", "Beni Suef"), ("ca", "Governació de Beni Suef"), ("ccp", "𑄝𑄬𑄚\u{11128} 𑄥\u{1112a}𑄠𑄬𑄛\u{11134}"), ("cs", "Bání Suvajf"), ("da", "Bani Suwayf"), ("de", "Bani Suwaif"), ("el", "Κυβερνείο Μπένι Σουέφ"), ("en", "Beni Suef"), ("es", "Beni Suef"), ("fa", "استان بنی\u{200c}سویف"), ("fi", "Bani Suwaifin kuvernoraatti"), ("fr", "Gouvernorat de Beni Souef"), ("gu", "બ\u{ac7}ની સ\u{ac1}એફ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז בני סויף"), ("hi", "बनी स\u{941}एफ\u{93c} म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Բենի Սուեյֆ"), ("id", "Kegubernuran Bani Suwayf"), ("it", "Governatorato di Beni Suef"), ("ja", "ベニ・スエフ県"), ("ka", "ბანი-სუაფის მუჰაფაზა"), ("kn", "ಬ\u{cc6}ನ\u{cbf} ಸುಯ\u{cc6}ಫ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "베니수에프 주"), ("lt", "Beni Sueifo muchafaza"), ("lv", "Benī Sueifas muhāfaza"), ("ml", "ബനീ സ\u{d41}വൈഫ\u{d4d}"), ("mr", "ब\u{947}नी स\u{941}एफ गव\u{94d}हरन\u{947}ट\u{947}ट"), ("ms", "Beni Suef Governorate"), ("nb", "Guvernementet Bani Suwayf"), ("nl", "Beni Suef"), ("no", "Guvernementet Bani Suwayf"), ("pl", "Bani Suwajf"), ("pt", "Beni Suef"), ("ro", "Bani Suwayf"), ("ru", "Бени-Суэйф"), ("si", "බෙන\u{dd2} ස\u{dd4}එෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Бени Суејф"), ("sr_Latn", "Beni Suejf"), ("sv", "Guvernementet Beni Suef"), ("sw", "Mkoa wa Beni Suef"), ("ta", "பெனி சுயபி கோவெர\u{bcd}னோரே"), ("te", "బ\u{c47}న\u{c3f} సుయ\u{c46}ఫ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เบน\u{e35} เซ\u{e34}ฟ กอฟเวอโนเลท"), ("tr", "Beni Suveyf"), ("uk", "Бені-Суейф"), ("ur", "محافظہ بنی سیوف"), ("vi", "Tỉnh Beni Suef"), ("zh", "贝尼苏韦夫省")]),
                        unofficial_name_list: ["Bani Suwayf", "Banī Suwayf"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::EG,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.0444196), longitude: Some(31.2357116), max_latitude: Some(30.1106024), min_latitude: Some(30.0083745), max_longitude: Some(31.3019729), min_longitude: Some(31.2149558)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة القاهرة"), ("be", "Мухафаза Каір"), ("bg", "Кайро"), ("bn", "ক\u{9be}য\u{9bc}রো গভর\u{9cd}নোরেট"), ("bs", "Kairo"), ("ca", "Governació del Caire"), ("ccp", "𑄇\u{1112d}𑄢\u{1112e}"), ("cs", "Káhira"), ("da", "Al Qahirah"), ("de", "al-Qahira"), ("el", "Κυβερνείο του Καΐρου"), ("en", "Cairo"), ("es", "El Cairo"), ("fa", "استان قاهره"), ("fi", "Kairon kuvernoraatti"), ("fr", "Gouvernorat du Caire"), ("gu", "ક\u{ac8}રો ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז קהיר"), ("hi", "क\u{93c}ाहिरा म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Կահիրե"), ("id", "Kegubernuran Al Qahirah"), ("it", "Governatorato del Cairo"), ("ja", "カイロ県"), ("ka", "კაიროს მუჰაფაზა"), ("kn", "ಕೈರೋ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "카이로 주"), ("lt", "Kairo muchafaza"), ("lv", "Kairas muhāfaza"), ("mr", "क\u{948}रो गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Cairo Governorate"), ("nb", "Guvernementet Al Qahirah"), ("nl", "Caïro"), ("no", "Guvernementet Al Qahirah"), ("pl", "Kair"), ("pt", "Cairo"), ("ro", "Al Qahirah"), ("ru", "Каир"), ("si", "කය\u{dd2}රෝ පළ\u{dcf}ත"), ("sk", "Káhira"), ("sr", "Каиро"), ("sr_Latn", "Kairo"), ("sv", "Guvernementet Kairo"), ("sw", "Mkoa wa Kairo"), ("ta", "கெய\u{bcd}ரோ கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "క\u{c46}య\u{c3f}ర\u{c4b} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลกาฮ\u{e34}ราห\u{e4c}"), ("tr", "Kahire"), ("uk", "Каїр"), ("ur", "محافظہ قاہرہ"), ("vi", "Cairo"), ("zh", "开罗省")]),
                        unofficial_name_list: ["Caire", "Cairo", "El Qahira", "Kairo", "Le Caire-sur-Mer", "al-Qāhirah"].to_vec(),
                    }
                ),
                (
                    "DK",
                    Subdivision{
                        name: "DK",
                        country_alpha2: Alpha2::EG,
                        code: "DK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.0832017), longitude: Some(31.4913182), max_latitude: Some(31.5452581), min_latitude: Some(30.5769786), max_longitude: Some(32.0835976), min_longitude: Some(31.2086064)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الدقهلية"), ("be", "Дакахлія"), ("bg", "Дакахлия"), ("bn", "ড\u{9be}ক\u{9be}হ\u{9cd}লিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Dakahlia"), ("ca", "Governació d’Al-Daqahliyah"), ("ccp", "𑄓𑄇𑄦\u{11134}𑄣\u{11128}𑄠"), ("cs", "Dachílija"), ("da", "Ad Daqahliyah"), ("de", "ad-Daqahliyya"), ("el", "Κυβερνείο Ντακάχλια"), ("en", "Dakahlia"), ("es", "Dacalia"), ("fa", "استان دقهلیه"), ("fi", "Al-Daqahliyya"), ("fr", "Gouvernorat de Dakahleya"), ("gu", "ડાકાહલિઆ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "א-דקהלייה"), ("hi", "दक\u{93c}\u{948}हल\u{947}या म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Դակահլիա"), ("id", "Kegubernuran Ad Daqahliyah"), ("it", "Governatorato di Daqahliyya"), ("ja", "ダカリーヤ県"), ("ka", "დაკაჰლიის მუჰაფაზა"), ("kn", "ಡಕಾಹ\u{ccd}ಲ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다칼리야 주"), ("lt", "Dakahlijos muchafaza"), ("lv", "Dekahlījas muhāfaza"), ("mr", "डाख\u{94d}लिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Dakahlia Governorate"), ("nb", "Ad Daqahliyah"), ("nl", "Ad Daqahliyah"), ("no", "Ad Daqahliyah"), ("pl", "Ad-Dakahlijja"), ("pt", "Dakahlia"), ("ro", "Ad Daqahliyah"), ("ru", "Дакахлия"), ("si", "ඩකහ\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sr", "Дакахлија"), ("sr_Latn", "Dakahlija"), ("sv", "Ad-Daqahliyya"), ("sw", "Mkoa wa Dakahlia"), ("te", "డ\u{c3e}క\u{c3e}హ\u{c3f}ల\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนครสวรรค\u{e4c}"), ("tr", "Dekahliye"), ("uk", "Дакахлія"), ("ur", "محافظہ دقہلیہ"), ("vi", "Tỉnh Dakahlia"), ("zh", "代蓋赫利耶省")]),
                        unofficial_name_list: ["Dakahlia", "El Daqahliya", "ad-Daqahliyah"].to_vec(),
                    }
                ),
                (
                    "DT",
                    Subdivision{
                        name: "DT",
                        country_alpha2: Alpha2::EG,
                        code: "DT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.4175388), longitude: Some(31.81444339999999), max_latitude: Some(31.4417822), min_latitude: Some(31.3934849), max_longitude: Some(31.8280067), min_longitude: Some(31.7816078)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دمياط"), ("bg", "Дамиета"), ("bn", "দ\u{9be}মিয\u{9bc}েট\u{9cd}ট\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Damietta"), ("ca", "Governació de Damietta"), ("ccp", "𑄓𑄟\u{1112d}𑄠𑄬𑄑"), ("cs", "Dimját"), ("da", "Dumyat"), ("de", "Dumyat"), ("el", "Κυβερνείο Δαμιέτης"), ("en", "Damietta"), ("es", "Damieta"), ("fa", "استان دمیاط"), ("fi", "Dumjatin kuvernoraatti"), ("fr", "Gouvernorat de Damiette"), ("gu", "ડ\u{ac7}મિએટા ગવર\u{acd}નોર\u{ac7}ટ"), ("hi", "द\u{941}मयात म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Դումյաթ"), ("id", "Kegubernuran Dumyat"), ("it", "Governatorato di Damietta"), ("ja", "ディムヤート県"), ("ka", "დუმიატის მუჰაფაზა"), ("kn", "ಡಾಮ\u{cbf}ತಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다미에타 주"), ("lt", "Dumjato muchafaza"), ("lv", "Dumjātas muhāfaza"), ("mr", "ड\u{945}मिएटटा गव\u{94d}हरन\u{947}ट\u{947}ट"), ("ms", "Damietta Governorate"), ("nb", "Guvernementet Dumyat"), ("nl", "Damietta"), ("no", "Guvernementet Dumyat"), ("pl", "Damietta (muhafaza)"), ("pt", "Damieta"), ("ro", "Dumyat"), ("ru", "Думьят"), ("si", "ඩම\u{dd3}එට\u{dca}ට\u{dcf} පළ\u{dcf}ත"), ("sr", "Дамијета"), ("sr_Latn", "Damijeta"), ("sv", "Guvernementet Damietta"), ("sw", "Mkoa wa Damietta"), ("ta", "தமியேட\u{bcd}ட\u{bbe} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "డ\u{c3e}మ\u{c3f}య\u{c46}ట\u{c4d}ట\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ด\u{e31}มย\u{e31}ต"), ("tr", "Dimyat"), ("uk", "Думʼят"), ("ur", "محافظہ دمیاط"), ("vi", "Tỉnh Damietta"), ("zh", "杜姆亞特省")]),
                        unofficial_name_list: ["Damiat", "Damiette", "Dumyat", "Dumyāţ"].to_vec(),
                    }
                ),
                (
                    "FYM",
                    Subdivision{
                        name: "FYM",
                        country_alpha2: Alpha2::EG,
                        code: "FYM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.3084021), longitude: Some(30.8428497), max_latitude: Some(29.3299673), min_latitude: Some(29.2918259), max_longitude: Some(30.86388549999999), min_longitude: Some(30.8206875)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الفيوم"), ("bg", "Фаюм"), ("bs", "Faiyum"), ("ca", "Governació d’el Faium"), ("ccp", "𑄜\u{1112d}𑄠\u{1112a}𑄟\u{11134}"), ("cs", "Fajjúm"), ("da", "Al Fayyum"), ("de", "al-Fayyum"), ("el", "Κυβερνείο Φαγιούμ"), ("en", "Faiyum"), ("es", "Fayún"), ("eu", "Faiyum"), ("fa", "استان فیوم"), ("fi", "Al-Fayyumin kuvernoraatti"), ("fr", "Gouvernorat du Fayoum"), ("he", "אל-פיום"), ("hi", "फ\u{93c}य\u{94d}य\u{942}म म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Ալ-Ֆայում"), ("it", "Governatorato di Faiyum"), ("ja", "ファイユーム県"), ("ka", "ელ-ფაიუმის მუჰაფაზა"), ("ko", "파이윰 주"), ("lt", "Fajumo muchafaza"), ("nb", "Guvernementet Al Fayyum"), ("nl", "Fajoem"), ("no", "Guvernementet Al Fayyum"), ("pl", "Fajum"), ("pt", "Faium"), ("ro", "Guvernoratul Al Fayyum"), ("ru", "Эль-Файюм"), ("sr", "Фајум"), ("sr_Latn", "Fajum"), ("sv", "Guvernementet Faijum"), ("sw", "Mkoa wa Faiyum"), ("tr", "Feyyum"), ("uk", "Ель-Файюм"), ("ur", "محافظہ فیوم"), ("zh", "法尤姆省")]),
                        unofficial_name_list: ["El Faiyūm", "al-Fayyum", "al-Fayyūm"].to_vec(),
                    }
                ),
                (
                    "GH",
                    Subdivision{
                        name: "GH",
                        country_alpha2: Alpha2::EG,
                        code: "GH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.8753556), longitude: Some(31.03351), max_latitude: Some(31.1583532), min_latitude: Some(30.5747929), max_longitude: Some(31.3105264), min_longitude: Some(30.7465372)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الغربية"), ("bg", "Гарбия"), ("bn", "ঘ\u{9be}রবিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Gharbia"), ("ca", "Governació d’Al-Gharbiya"), ("ccp", "𑄊𑄢\u{11134}𑄝\u{11128}𑄠"), ("cs", "Gharbíja"), ("cy", "Gharbia Governorate"), ("da", "Al Gharbiyah"), ("de", "al-Gharbiyya"), ("el", "Κυβερνείο Γαρμπία"), ("en", "Gharbia"), ("es", "Occidental"), ("fa", "استان غربیه"), ("fi", "Al-Gharbiyyan kuvernoraatti"), ("fr", "Gouvernorat de Gharbeya"), ("gu", "ઘારબીયા ગવર\u{acd}નમ\u{ac7}\u{a82}ટ"), ("he", "מחוז אל-ע׳רביה"), ("hi", "ग\u{93c}रबीया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Ղարբիա"), ("id", "Kegubernuran Al Gharbiyah"), ("it", "Governatorato di Gharbiyya"), ("ja", "ガルビーヤ県"), ("ka", "გარბიის მუჰაფაზა"), ("kn", "ಘರ\u{ccd}ಬ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "가르비야 주"), ("lt", "Garbijos muchafaza"), ("lv", "Garbijas muhāfaza"), ("mr", "घर\u{94d}बिया गव\u{94d}हरन\u{947}ट\u{947}ट"), ("ms", "Gharbia Governorate"), ("nb", "Guvernementet Al Gharbiyah"), ("nl", "Al Gharbiyah"), ("no", "Guvernementet Al Gharbiyah"), ("pl", "Prowincja Zachodnia"), ("pt", "Garbia"), ("ro", "Al Gharbiyah"), ("ru", "Гарбия"), ("si", "ඝර\u{dca}බ\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Гарбија"), ("sr_Latn", "Garbija"), ("sv", "Guvernementet Al-Gharbiyya"), ("sw", "Mkoa wa Gharbia"), ("ta", "க\u{bbe}ரப\u{bc0}ஆ கோவெர\u{bcd}னோரே"), ("te", "గ\u{c3e}ర\u{c4d}బ\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "กาฮ\u{e4c}เบ\u{e35}ย โกเวอโนเรท"), ("tr", "Garbiya"), ("uk", "Гарбія"), ("ur", "محافظہ غربیہ"), ("vi", "Tỉnh Gharbia"), ("zh", "西部省")]),
                        unofficial_name_list: ["El Gharbiya", "Gharbiya", "al-Garbiyah", "al-Ġarbīyah"].to_vec(),
                    }
                ),
                (
                    "GZ",
                    Subdivision{
                        name: "GZ",
                        country_alpha2: Alpha2::EG,
                        code: "GZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.0130557), longitude: Some(31.2088526), max_latitude: Some(30.0714063), min_latitude: Some(29.9621703), max_longitude: Some(31.2320457), min_longitude: Some(31.1472406)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الجيزة"), ("be", "Эль-Гіза"), ("bg", "Гиза"), ("bn", "গিজ\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Giza"), ("ca", "Governació de Gizeh"), ("ccp", "𑄎\u{11128}𑄎"), ("cs", "Gíza"), ("da", "Al Jizah"), ("de", "al-Dschiza"), ("el", "Κυβερνείο Γκίζας"), ("en", "Giza"), ("es", "Guiza"), ("et", "Giza kubernerkond"), ("eu", "Gizako gobernaketa"), ("fa", "استان جیزه"), ("fi", "Al-Gizan kuvernoraatti"), ("fr", "Gouvernorat de Gizeh"), ("gu", "ગિઝા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-גיזה"), ("hi", "गीज\u{93c}ा म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Ալ-Գիզա"), ("id", "Kegubernuran Al Jizah"), ("it", "Governatorato di Giza"), ("ja", "ギーザ県"), ("ka", "გიზის მუჰაფაზა"), ("kn", "ಗೀಜಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "기자 주"), ("lt", "Gizos muchafaza"), ("lv", "Gīzas muhāfaza"), ("mr", "गिझा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Giza Governorate"), ("nb", "Guvernementet Al Jizah"), ("nl", "Gizeh"), ("no", "Guvernementet Al Jizah"), ("pl", "Giza"), ("pt", "Guizé"), ("ro", "Al Jizah"), ("ru", "Эль-Гиза"), ("si", "ග\u{dd3}ස\u{dcf} පළ\u{dcf}ත"), ("sr", "Гиза"), ("sr_Latn", "Giza"), ("sv", "Guvernementet Giza"), ("sw", "Mkoa wa Giza"), ("ta", "கிச\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "గ\u{c40}జ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลจ\u{e34}ซาห\u{e4c}"), ("tr", "Gize"), ("uk", "Гіза"), ("ur", "محافظہ جیزہ"), ("vi", "Giza"), ("zh", "吉薩省")]),
                        unofficial_name_list: ["El Giza", "Gise", "Giseh", "Giza", "Gizah", "Gîza", "Jiza", "Jizah", "al-Jīzah"].to_vec(),
                    }
                ),
                (
                    "IS",
                    Subdivision{
                        name: "IS",
                        country_alpha2: Alpha2::EG,
                        code: "IS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.5830934), longitude: Some(32.2653887), max_latitude: Some(31.0059587), min_latitude: Some(30.2436079), max_longitude: Some(32.7520118), min_longitude: Some(31.6954456)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الإسماعيلية"), ("bg", "Исмаилия"), ("bn", "ইস\u{9cd}ম\u{9be}ইলিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Ismailija"), ("ca", "Governació d’Ismailiyah"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄟\u{1112d}𑄣\u{11128}𑄠"), ("ceb", "Ismailia Governorate"), ("cs", "Ismá’ílíja"), ("da", "Al Isma’iliyah"), ("el", "Κυβερνείο Ισμαηλίας"), ("en", "Ismailia"), ("es", "Ismailia"), ("fa", "استان اسماعیلیه"), ("fi", "Al-Isma’iliyyan kuvernoraatti"), ("fr", "Gouvernorat d’Ismaïlia"), ("gu", "ઇસ\u{acd}માઇલિયા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אל-אסמאעיליה"), ("hi", "इस\u{94d}माईलिया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Իսմայիլիա"), ("id", "Kegubernuran Al Isma’iliyah"), ("it", "Governatorato di Ismailia"), ("ja", "イスマイリア県"), ("ka", "ისმაილიის მუჰაფაზა"), ("kn", "ಇಸ\u{ccd}ಮೇಲ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "이스마일리아 주"), ("lt", "Ismailijos muchafaza"), ("lv", "Ismaīlījas muhāfaza"), ("mr", "इस\u{94d}माईलिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ismailia Governorate"), ("nb", "Guvernementet Al Isma’iliyah"), ("nl", "Ismaïlia"), ("no", "Guvernementet Al Isma’iliyah"), ("pl", "Ismailia"), ("pt", "Ismaília"), ("ro", "Al Isma’iliyah"), ("ru", "Исмаилия"), ("si", "ඉස\u{dca}මය\u{dd2}ල\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Al-Ismá’ílíja"), ("sr", "Исмаилија"), ("sr_Latn", "Ismailija"), ("sv", "Guvernementet Ismailia"), ("sw", "Mkoa wa Ismailia"), ("ta", "இஸ\u{bcd}ம\u{bbe}யிலிஆ கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "ఇస\u{c4d}మ\u{c3e}య\u{c3f}ల\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e34}สมาเล\u{e35}ย กอฟเวอโนเรท"), ("tr", "İsmailiye"), ("uk", "Ісмаїлія"), ("ur", "محافظہ اسماعیلیہ"), ("vi", "Tỉnh Ismailia"), ("zh", "伊斯梅利亚省")]),
                        unofficial_name_list: ["El Ismailia", "Ismaʿiliya", "al-Ismailiyah", "al-Ismāīlīyah"].to_vec(),
                    }
                ),
                (
                    "JS",
                    Subdivision{
                        name: "JS",
                        country_alpha2: Alpha2::EG,
                        code: "JS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.3101828), longitude: Some(34.1531947), max_latitude: Some(29.9494465), min_latitude: Some(27.7240277), max_longitude: Some(34.9076691), min_longitude: Some(32.6094505)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب سيناء"), ("bg", "Южен Синай"), ("bn", "দক\u{9cd}ষিন সিন\u{9be}ই গভর\u{9cd}নোরেট"), ("bs", "Južni Sinaj"), ("ca", "Governació del Sinaí del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄥\u{11128}𑄚\u{1112d}"), ("ceb", "South Sinai Governorate"), ("cs", "Jižní Sinaj"), ("da", "Janub Sina’"), ("de", "Dschanub Sina"), ("el", "Κυβερνείο Νοτίου Σινά"), ("en", "South Sinai"), ("es", "Sinaí del Sur"), ("et", "Lõuna-Siinai kubernerkond"), ("fa", "استان سینای جنوبی"), ("fi", "Janub Sina’"), ("fr", "Gouvernorat du Sinaï Sud"), ("gu", "સાઉથ સિનાઇ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "דרום סיני"), ("hi", "दक\u{94d}षिण सीनाई म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hr", "Južni Sinaj"), ("hy", "Հարավային Սինայ"), ("id", "Janub Sina’"), ("it", "Governatorato del Sinai del Sud"), ("ja", "南シナイ県"), ("ka", "სამხრეთ სინაის მუჰაფაზა"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಸ\u{cbf}ನೈ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "자누브시나 주"), ("lt", "Pietų Sinajaus muchafaza"), ("lv", "Dienvidsīnāja muhāfaza"), ("mk", "Јужен Синај"), ("mr", "दक\u{94d}षिण सिनाई गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Sinai Selatan"), ("nb", "Guvernementet Janub Sina’"), ("nl", "Zuid-Sinaï"), ("no", "Guvernementet Janub Sina’"), ("pl", "Synaj Południowy"), ("pt", "Sinai do Sul"), ("ro", "Janub Sina’"), ("ru", "Южный Синай"), ("si", "දක\u{dd4}ණ\u{dd4} ස\u{dd2}නය\u{dd2} පළ\u{dcf}ත"), ("sr", "Јужни Синај"), ("sr_Latn", "Južni Sinaj"), ("sv", "Guvernementet Sina al-Janubiyya"), ("sw", "Mkoa wa Sinai Kusini"), ("ta", "தெற\u{bcd}கு சின\u{bbe}ய\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "దక\u{c4d}ష\u{c3f}ణ స\u{c3f}న\u{c3e}య\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เม\u{e37}องดาโชก\u{e38}ซ"), ("tr", "Güney Sina"), ("uk", "Південний Синай"), ("ur", "محافظہ جنوبی سینا"), ("vi", "Tỉnh Nam Sinai"), ("zh", "南西奈省")]),
                        unofficial_name_list: ["Sina al-Janūbīyah", "Sinai al-Janubiyah", "South Sinai"].to_vec(),
                    }
                ),
                (
                    "KB",
                    Subdivision{
                        name: "KB",
                        country_alpha2: Alpha2::EG,
                        code: "KB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.3292368), longitude: Some(31.2168466), max_latitude: Some(30.6076595), min_latitude: Some(30.1040552), max_longitude: Some(31.4157302), min_longitude: Some(31.0542669)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة القليوبية"), ("bg", "Калюбия"), ("bn", "ক\u{9be}ল\u{9c1}বিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Qalyubia"), ("ca", "Governació de Qalyubia"), ("ccp", "𑄇\u{11127}𑄠𑄣\u{1112d}𑄠\u{1112a}𑄝\u{11128}𑄠"), ("cs", "Kaljúbíja"), ("da", "Al Qalyubiyah"), ("de", "al-Qalyubiyya"), ("el", "Κυβερνείο Καλγιούμπια"), ("en", "Qalyubia"), ("es", "Caliubia"), ("eu", "Qaliubiya"), ("fa", "استان قلیوبیه"), ("fi", "Al-Qalyubiyya"), ("fr", "Gouvernorat de Qalyubiya"), ("gu", "ક\u{acd}વાલિબિયા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אל-קליוביה"), ("hi", "क\u{93c}ल\u{94d}य\u{942}बीया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("id", "Kegubernuran Al Qalyubiyah"), ("it", "Governatorato di al-Qalyūbiyya"), ("ja", "カリュービーヤ県"), ("ka", "კალიუბიის მუჰაფაზა"), ("kn", "ಕ\u{ccd}ಲ\u{ccd}ಯಾಲ\u{cbf}ಯಾಬ\u{cbf} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "칼리우비야 주"), ("lt", "Kaljubijos muchafaza"), ("lv", "Kaljūbījas muhāfaza"), ("mr", "कल\u{94d}याबिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Qalyubia Governorate"), ("nb", "Al Qalyubiyah"), ("nl", "Al Qalyubiyah"), ("no", "Al Qalyubiyah"), ("pl", "Al-Kaljubijja"), ("pt", "Qaliubia"), ("ro", "Al Qalyubiyah"), ("ru", "Кальюбия"), ("si", "කල\u{dd2}ය\u{dd4}බ\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Каљубија"), ("sr_Latn", "Kaljubija"), ("sv", "Al-Qalyubiyya"), ("ta", "கிப\u{bcd}லயுப\u{bc0}ஆ கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "ఖ\u{c3e}ల\u{c4d}యుబ\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลกอลย\u{e38}บ\u{e35}ยะห\u{e4c}"), ("tr", "Kalyubiye"), ("uk", "Кальюбія"), ("ur", "محافظہ قلیوبیہ"), ("vi", "Tỉnh Qalyubia"), ("zh", "蓋盧比尤省")]),
                        unofficial_name_list: ["El Qalubiya", "al-Qalyubiyah"].to_vec(),
                    }
                ),
                (
                    "KFS",
                    Subdivision{
                        name: "KFS",
                        country_alpha2: Alpha2::EG,
                        code: "KFS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.3085444), longitude: Some(30.8039474), max_latitude: Some(31.6016217), min_latitude: Some(30.9942465), max_longitude: Some(31.3122298), min_longitude: Some(30.3653976)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كفر الشيخ"), ("bg", "Кафър ел-Шейх"), ("bn", "ক\u{9be}ফ\u{9cd}র এল-শেইখ গভর\u{9cd}নোরেট"), ("bs", "Kafr el-Sheikh"), ("ca", "Governació de Kafr al-Sheikh"), ("ccp", "𑄇𑄛\u{11134}𑄢 𑄃𑄬𑄣\u{11134}-𑄥𑄬𑄇\u{11134}"), ("ceb", "Kafr ash Shaykh"), ("cs", "Kafr aš-Šajch"), ("da", "Kafr ash Shaykh"), ("de", "Kafr asch-Schaich"), ("el", "Κυβερνείο Καφρ ελ Σεΐχ"), ("en", "Kafr el-Sheikh"), ("es", "Kafr el Sheij"), ("fa", "استان کفرالشیخ"), ("fi", "Kafr al-Šaykh"), ("fr", "Gouvernorat de Kafr el-Cheik"), ("gu", "કાફર અલ-શ\u{ac7}ખ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז כפר א שיח׳"), ("hi", "कफ\u{93c}\u{94d}र अल-श\u{947}ख\u{93c} म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("id", "Kegubernuran Kafr asy Syaykh"), ("it", "Governatorato di Kafr el-Sheikh"), ("ja", "カフル・アッシャイフ県"), ("ka", "ქაფრ-ელ-შაიხის მუჰაფაზა"), ("kn", "ಕಾಫ\u{ccd}ರ\u{ccd} ಎಲ\u{ccd}-ಶೇಖ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "카프르엘셰이크 주"), ("lt", "Kafr eš Šeicho muchafaza"), ("lv", "Kefr eš Šeihas muhāfaza"), ("mr", "कफर एल-श\u{947}ख गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Kafr el-Sheikh Governorate"), ("nb", "Kafr ash Shaykh"), ("nl", "Kafr el Sheikh"), ("no", "Kafr ash Shaykh"), ("pl", "Kafr asz-Szajch"), ("pt", "Kafr el-Sheikh"), ("ro", "Guvernoratul Kafr el-Sheikh"), ("ru", "Кафр-эш-Шейх"), ("si", "කෆ\u{dca}ර\u{dca} එල\u{dca}-ෂෙය\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sk", "Kafr aš-Šajch"), ("sr", "Кафр еш Шејх"), ("sr_Latn", "Kafr eš Šejh"), ("sv", "Kafr el-Sheikh"), ("ta", "க\u{bbe}பிர\u{bcd} எல\u{bcd} -ஷேக\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "క\u{c3e}ఫర\u{c4d} ఎల\u{c4d}-ష\u{c47}క\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "คาฟร\u{e4c}เอลเชก"), ("tr", "Kafrü’ş-Şeyh"), ("uk", "Кафр-еш-Шейх"), ("ur", "محافظہ کفر الشیخ"), ("vi", "Tỉnh Kafr el-Sheikh"), ("zh", "謝赫村省")]),
                        unofficial_name_list: ["Kafr-ash-Shaykh", "Kafr-aš-Šayh\u{331}"].to_vec(),
                    }
                ),
                (
                    "KN",
                    Subdivision{
                        name: "KN",
                        country_alpha2: Alpha2::EG,
                        code: "KN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.155061), longitude: Some(32.7160121), max_latitude: Some(26.2104418), min_latitude: Some(26.1320478), max_longitude: Some(32.7601361), min_longitude: Some(32.6828284)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة قنا"), ("be", "Кена"), ("bg", "Кена"), ("bn", "কেন\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Qena"), ("ca", "Governació de Qena"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄚"), ("cs", "Kená"), ("da", "Qina"), ("de", "Qina"), ("el", "Κυβερνείο Κένα"), ("en", "Qena"), ("es", "Quena"), ("et", "Qinā kubernerkond"), ("fa", "استان قنا"), ("fi", "Qinan kuvernoraatti"), ("fr", "gouvernorat de Qena"), ("gu", "ક\u{acd}વ\u{ac7}ના ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז קינא"), ("hi", "क\u{93c}\u{947}ना म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("id", "Kegubernuran Qina"), ("it", "Governatorato di Qena"), ("ja", "ケナ県"), ("ka", "კენის მუჰაფაზა"), ("kn", "ಕ\u{ccd}ಯ\u{cc2}ನಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "케나 주"), ("lt", "Kenos muchafaza"), ("lv", "Kinas muhāfaza"), ("mr", "कायद\u{947} गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Qena Governorate"), ("nb", "Guvernementet Qina"), ("nl", "Qina"), ("no", "Guvernementet Qina"), ("pl", "Kina"), ("pt", "Qina"), ("ro", "Guvernoratul Qena"), ("ru", "Кена"), ("si", "කෙන\u{dcf} පළ\u{dcf}ත"), ("sr", "Кена"), ("sr_Latn", "Kena"), ("sv", "Guvernementet Qena"), ("sw", "Mkoa wa Qena"), ("ta", "கியேன\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "ఖ\u{c47}న\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ก\u{e34}นะ"), ("tr", "Kina"), ("uk", "Кена"), ("ur", "محافظہ قنا"), ("vi", "Tỉnh Qena"), ("zh", "基納省")]),
                        unofficial_name_list: ["Qina", "Qinā"].to_vec(),
                    }
                ),
                (
                    "LX",
                    Subdivision{
                        name: "LX",
                        country_alpha2: Alpha2::EG,
                        code: "LX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.6872431), longitude: Some(32.6396357), max_latitude: Some(25.7358046), min_latitude: Some(25.6540777), max_longitude: Some(32.6964268), min_longitude: Some(32.6225071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الأقصر"), ("be", "Луксор"), ("bg", "Луксор"), ("bs", "Luxor"), ("ca", "Governació de Luxor"), ("ccp", "𑄣𑄇\u{11134}𑄎\u{1112e}𑄢\u{11134}"), ("cs", "Luxor"), ("da", "Luxor"), ("de", "al-Uqsur"), ("el", "Κυβερνείο Λούξορ"), ("en", "Luxor"), ("es", "Lúxor"), ("et", "Luxori kubernerkond"), ("fa", "استان اقصر"), ("fi", "Luxorin kuvernoraatti"), ("fr", "Gouvernorat de Louxor"), ("he", "מחוז לוקסור"), ("hi", "लक\u{94d}सर म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Լուքսոր"), ("it", "Governatorato di Luxor"), ("ja", "ルクソール県"), ("ka", "ლუქსორის მუჰაფაზა"), ("ko", "룩소르 주"), ("lt", "Luksoro muchafaza"), ("nb", "Guvernementet Luxor"), ("nl", "Luxor"), ("no", "Guvernementet Luxor"), ("pl", "Luksor"), ("pt", "Luxor"), ("ro", "Guvernoratul Luxor"), ("ru", "Луксор"), ("sr", "Луксор"), ("sr_Latn", "Luksor"), ("sv", "Guvernementet Luxor"), ("sw", "Mkoa wa Luxor"), ("tr", "Luksor"), ("uk", "Луксор"), ("ur", "محافظہ الاقصر"), ("zh", "盧克索省")]),
                        unofficial_name_list: ["Louxor", "Luxor", "al-Uqsur", "al-Uqşur"].to_vec(),
                    }
                ),
                (
                    "MN",
                    Subdivision{
                        name: "MN",
                        country_alpha2: Alpha2::EG,
                        code: "MN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.0870967), longitude: Some(30.7618397), max_latitude: Some(28.1318404), min_latitude: Some(28.058606), max_longitude: Some(30.7843728), min_longitude: Some(30.7170696)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنيا"), ("be", "Эль-Мінья"), ("bg", "Миня"), ("bn", "মিন\u{9cd}য\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Minya"), ("ca", "Governació d’Al-Minya"), ("ccp", "𑄟\u{11128}𑄚\u{11128}𑄠"), ("cs", "Minjá"), ("da", "Al Minya"), ("de", "al-Minya"), ("el", "Κυβερνείο Μίνια"), ("en", "Minya"), ("es", "Menia"), ("fa", "استان منیا"), ("fi", "Al-Minyan kuvernoraatti"), ("fr", "Gouvernorat de Minya"), ("gu", "મિન\u{acd}યા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מיניה"), ("hi", "मिन\u{94d}या म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Էլ Մինյա"), ("id", "Al Minya"), ("it", "Governatorato di Minya"), ("ja", "ミニヤー県"), ("ka", "ელ-მინიის მუჰაფაზა"), ("kn", "ಮ\u{cbf}ನ\u{ccd}ಯ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "미니아 주"), ("ky", "Минья"), ("lt", "Minjos muchafaza"), ("lv", "Minjas muhāfaza"), ("mr", "मिनिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Minya"), ("nb", "Guvernementet Al Minya"), ("nl", "Minya"), ("no", "Guvernementet Al Minya"), ("pl", "Al-Minja (muhafaza)"), ("pt", "Minya"), ("ro", "Al Minya"), ("ru", "Эль-Минья"), ("si", "ම\u{dd2}න\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Миња"), ("sr_Latn", "Minja"), ("sv", "Guvernementet Al-Minya"), ("sw", "Mkoa wa Minya"), ("ta", "மின\u{bcd}னிய\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "మ\u{c3f}న\u{c4d}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลม\u{e34}เน\u{e35}ย"), ("tr", "Minye"), ("uk", "Мінья (губернаторство)"), ("ur", "محافظہ منیا"), ("vi", "Tỉnh Minya"), ("zh", "明亞省")]),
                        unofficial_name_list: ["El Minya", "Minia", "al-Minya", "al-Minyā"].to_vec(),
                    }
                ),
                (
                    "MNF",
                    Subdivision{
                        name: "MNF",
                        country_alpha2: Alpha2::EG,
                        code: "MNF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.5972455), longitude: Some(30.9876321), max_latitude: Some(30.7480879), min_latitude: Some(30.1829009), max_longitude: Some(31.2559327), min_longitude: Some(30.4745556)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة المنوفية"), ("be", "Мінуфія"), ("bg", "Минуфия"), ("bn", "মন\u{9c1}ফিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Monufia"), ("ca", "Governació de Menufeya"), ("ccp", "𑄟\u{11127}𑄚\u{1112a}𑄜\u{11128}𑄠"), ("cs", "Minúfija"), ("da", "Al Minufiyah"), ("de", "al-Minufiyya"), ("el", "Κυβερνείο Μενούφια"), ("en", "Monufia"), ("es", "Menufia"), ("fa", "استان منوفیه"), ("fi", "Al-Minufiyya"), ("fr", "Gouvernorat de Menufeya"), ("gu", "મોન\u{ac1}ફિયા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-מ\u{5b4}נופיה"), ("hi", "मोन\u{942}फ\u{93c}िया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Մինուֆիա"), ("id", "Kegubernuran Al Minufiyah"), ("it", "Governatorato di al-Manufiyya"), ("ja", "ミヌーフィーヤ県"), ("ka", "მონუფიის მუჰაფაზა"), ("kn", "ಮೊನುಫ\u{cbf}ಯ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "무누피아 주"), ("lt", "Minufijos muchafaza"), ("lv", "Minūfījas muhāfaza"), ("mr", "मोन\u{942}फिया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Monufia Governorate"), ("nb", "Guvernementet Al Minufiyah"), ("nl", "Al Minufiyah"), ("no", "Guvernementet Al Minufiyah"), ("pl", "Al-Minufijja"), ("pt", "Monufia"), ("ro", "Al Minufiyah"), ("ru", "Минуфия"), ("si", "මොන\u{dd4}ෆ\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Минуфија"), ("sr_Latn", "Minufija"), ("sv", "Guvernementet Al-Minufiyya"), ("sw", "Mkoa wa Monufia"), ("ta", "மொநுபிய\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "మ\u{c4b}నూఫ\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "โมน\u{e39}เฟ\u{e35}ย โกเวอโนเรท"), ("tr", "Minufiye"), ("uk", "Мінуфія"), ("ur", "محافظہ منوفیہ"), ("vi", "Tỉnh Monufia"), ("zh", "米努夫省")]),
                        unofficial_name_list: ["El Minufiya", "Menufiya", "al-Minufiyah", "al-Minūfīyah"].to_vec(),
                    }
                ),
                (
                    "MT",
                    Subdivision{
                        name: "MT",
                        country_alpha2: Alpha2::EG,
                        code: "MT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.3543445), longitude: Some(27.2373159), max_latitude: Some(31.3761768), min_latitude: Some(31.2970958), max_longitude: Some(27.3544331), min_longitude: Some(27.156232)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مطروح"), ("bg", "Матрух"), ("bn", "ম\u{9be}ত\u{9cd}র\u{9c1}হ গভর\u{9cd}নোরেট"), ("bs", "Matrouh"), ("ca", "Governació de Matruh"), ("ccp", "𑄟𑄖\u{11133}𑄢\u{1112e}𑄦\u{11134}"), ("cs", "Matrúh"), ("da", "Matruh"), ("de", "Matruh"), ("el", "Κυβερνείο Ματρούχ"), ("en", "Matrouh"), ("es", "Matrú"), ("fa", "استان مطروح"), ("fi", "Matruhin kuvernoraatti"), ("fr", "Gouvernorat de Marsa-Matruh"), ("gu", "મટ\u{acd}રોહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז מטרוח"), ("hi", "मत\u{94d}र\u{942}ह म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Matrúh"), ("id", "Kegubernuran Matruh"), ("it", "Governatorato di Matruh"), ("ja", "マトルーフ県"), ("ka", "მატრუჰის მუჰაფაზა"), ("kn", "ಮಾಥ\u{ccc}ರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "마트루 주"), ("lt", "Matruho muchafaza"), ("lv", "Matrūhas muhāfaza"), ("mk", "Матрух"), ("ml", "മത\u{d4d}രോഹ\u{d4d} ഗവർണ\u{d4d}ണറേറ\u{d4d}റ\u{d4d}"), ("mr", "म\u{945}टरफ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Matrouh Governorate"), ("nb", "Guvernementet Matruh"), ("nl", "Matruh"), ("no", "Guvernementet Matruh"), ("pl", "Matruh"), ("pt", "Matruh"), ("ro", "Matruh"), ("ru", "Матрух"), ("si", "මට\u{dca}\u{200d}රෝ පළ\u{dcf}ත"), ("sr", "Матрух"), ("sr_Latn", "Matruh"), ("sv", "Guvernementet Matruh"), ("sw", "Mkoa wa Matruh"), ("ta", "மற\u{bcd}றொருஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "మ\u{c3e}ట\u{c4d}ర\u{c4b} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตม\u{e31}ฏร\u{e39}ห\u{e4c}"), ("tr", "Matruh"), ("uk", "Матрух"), ("ur", "محافظہ مطروح"), ("vi", "Tỉnh Matrouh"), ("zh", "馬特魯省")]),
                        unofficial_name_list: ["Matrah", "Matrūh"].to_vec(),
                    }
                ),
                (
                    "PTS",
                    Subdivision{
                        name: "PTS",
                        country_alpha2: Alpha2::EG,
                        code: "PTS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.2652893), longitude: Some(32.3018661), max_latitude: Some(31.2784705), min_latitude: Some(31.1988856), max_longitude: Some(32.3213102), min_longitude: Some(32.2490287)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بورسعيد"), ("be", "Мухафаза Порт-Саід"), ("bg", "Порт Саид"), ("bn", "পোর\u{9cd}ট সঈদ গভর\u{9cd}নোরেট"), ("bs", "Port Said"), ("ca", "Governació de Port Saïd"), ("ccp", "𑄛\u{11127}𑄢\u{11133}𑄑\u{11134} 𑄥𑄬𑄃\u{11128}𑄖\u{11134}"), ("cs", "Port Said"), ("da", "Bur Sa’id"), ("el", "Κυβερνείο Πορτ Σάιντ"), ("en", "Port Said"), ("es", "Puerto Saíd"), ("fa", "استان پورت\u{200c}سعید"), ("fi", "Bur Sa’idin kuvernoraatti"), ("fr", "Gouvernorat de Port-Saïd"), ("gu", "પોર\u{acd}ટ સ\u{ac7}ડ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז פורט סעיד"), ("hi", "पोर\u{94d}ट सईद म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("id", "Kegubernuran Bur Sa’id"), ("it", "Governatorato di Porto Said"), ("ja", "ポートサイド県"), ("ka", "პორტ-საიდის მუჰაფაზა"), ("kn", "ಪೋರ\u{ccd}ಟ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd} ಸೇಡ\u{ccd}"), ("ko", "포트사이드 주"), ("lt", "Port Saido muchafaza"), ("lv", "Portsaīdas muhāfaza"), ("mr", "पोर\u{94d}ट स\u{947}ड गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Port Said Governorate"), ("nb", "Bur Sa’id"), ("nl", "Port Said"), ("no", "Bur Sa’id"), ("pl", "Port Said"), ("pt", "Governamento de Porto Said"), ("ro", "Bur Sa’id"), ("ru", "Порт-Саид"), ("si", "පොර\u{dca}ට\u{dca} සෙඩ\u{dca} පළ\u{dcf}ත"), ("sr", "Порт Саид"), ("sr_Latn", "Port Said"), ("sv", "Port Said"), ("sw", "Mkoa wa Port Said"), ("ta", "போர\u{bcd}ட\u{bcd} சேட\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4d} స\u{c46}య\u{c3f}ద\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เม\u{e37}องพอร\u{e4c}ตซาอ\u{e34}ด"), ("tr", "Port Said"), ("uk", "Порт-Саїд"), ("ur", "محافظہ پورٹ سعید"), ("vi", "Tỉnh Port Said"), ("zh", "塞得港省")]),
                        unofficial_name_list: ["Bur Said", "Būr Saʿīd"].to_vec(),
                    }
                ),
                (
                    "SHG",
                    Subdivision{
                        name: "SHG",
                        country_alpha2: Alpha2::EG,
                        code: "SHG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.501348), longitude: Some(31.76513619999999), max_latitude: Some(26.9343956), min_latitude: Some(26.1518194), max_longitude: Some(32.1707085), min_longitude: Some(31.3415418)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سوهاج"), ("bg", "Сухадж"), ("bn", "সোহ\u{9be}গ গভর\u{9cd}নোরেট"), ("bs", "Sohag"), ("ca", "Governació de Sohag"), ("ccp", "𑄥\u{1112e}𑄦𑄇\u{11134}"), ("cs", "Suhag"), ("da", "Sohag Governorate"), ("de", "Sauhadsch"), ("el", "Κυβερνείο Σοχάγκ"), ("en", "Sohag"), ("es", "Suhag"), ("fa", "استان سوهاج"), ("fi", "Suhaj"), ("fr", "Gouvernorat de Sohag"), ("gu", "સોહગ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז סוהאג"), ("hi", "सोहाग म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hu", "Szohág kormányzóság"), ("id", "Kegubernuran Suhaj"), ("it", "Governatorato di Sohag"), ("ja", "ソハーグ県"), ("ka", "სუჰაგის მუჰაფაზა"), ("kn", "ಸೊಹಾಗ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "소하그 주"), ("lt", "Sohago muchafaza"), ("lv", "Sauhāgas muhāfaza"), ("mr", "सोहाग गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Sohag Governorate"), ("nb", "Suhaj"), ("nl", "Suhaj"), ("no", "Suhaj"), ("pl", "Sauhadż"), ("pt", "Sohag"), ("ro", "Guvernoratul Sohag"), ("ru", "Сохаг"), ("si", "සොහ\u{dcf}ග\u{dca} පළ\u{dcf}ත"), ("sk", "Sawhádž"), ("sr", "Сохаг"), ("sr_Latn", "Sohag"), ("sv", "Guvernementet Sohag"), ("sw", "Mkoa wa Sohag"), ("ta", "சோகக\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "స\u{c4b}హ\u{c3e}గ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ซ\u{e38}ฮ\u{e31}จ"), ("tr", "Sevhac"), ("uk", "Согаґ"), ("ur", "محافظہ سوہاج"), ("vi", "Tỉnh Sohag"), ("zh", "索哈傑省")]),
                        unofficial_name_list: ["Sawhaj", "Sawhāj", "Suhag", "Suhaj", "Sūhaj"].to_vec(),
                    }
                ),
                (
                    "SHR",
                    Subdivision{
                        name: "SHR",
                        country_alpha2: Alpha2::EG,
                        code: "SHR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.7326622), longitude: Some(31.7195459), max_latitude: Some(31.168795), min_latitude: Some(30.2886886), max_longitude: Some(32.1782565), min_longitude: Some(31.2607672)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الشرقية"), ("bg", "Шаркия"), ("bn", "আল শ\u{9be}রকিয\u{9bc}\u{9be} গভর\u{9cd}নোরেট"), ("bs", "Sharqia"), ("ca", "Governació de Sharqia"), ("ccp", "𑄃𑄣\u{11134} 𑄥𑄢\u{11134}𑄇\u{1112d}\u{1112a}𑄠"), ("cs", "Šarkíja"), ("da", "Ash Sharqiyah"), ("de", "asch-Scharqiyya"), ("el", "Ας Σαρκίγια"), ("en", "Al Sharqia"), ("es", "Oriental"), ("fa", "استان شرقیه"), ("fi", "Al-Šarqiyya"), ("fr", "Ach-Charqiya"), ("gu", "અલ શારકિયા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "א-שרקיה"), ("hi", "अल-शरक\u{93c}िया म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Շարկիա"), ("id", "Kegubernuran Asy Syarqiyah"), ("it", "Governatorato di Sharqiyya"), ("ja", "シャルキーヤ県"), ("ka", "შარკიის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಶಾರ\u{ccd}ಖ\u{cbf}ಯ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "샤르키야 주"), ("lt", "Šarkijos muchafaza"), ("lv", "Šerkījas muhāfaza"), ("mr", "अल शर\u{94d}किया गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Sharqia Governorate"), ("nb", "Ash Sharqiyah"), ("nl", "Ash Sharqiyah"), ("no", "Ash Sharqiyah"), ("pl", "Prowincja Wschodnia"), ("pt", "Xarqia"), ("ro", "Ash Sharqiyah"), ("ru", "Шаркия"), ("si", "අල\u{dca} ශක\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Шаркија"), ("sr_Latn", "Šarkija"), ("sv", "Ash-Sharqiyya"), ("sw", "Mkoa wa Sharqia"), ("ta", "அல\u{bcd} ஷ\u{bbe}ர\u{bcd}க\u{bcd}கிய\u{bbe} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "అల\u{c4d} షర\u{c4d}ఖ\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เบโรโว"), ("tr", "Şarkiye"), ("uk", "Шаркія"), ("ur", "محافظہ الشرقیہ"), ("vi", "Tỉnh Al Sharqia"), ("zh", "東部省")]),
                        unofficial_name_list: ["ash-Sharqiyah", "aš-Šarqīyah"].to_vec(),
                    }
                ),
                (
                    "SIN",
                    Subdivision{
                        name: "SIN",
                        country_alpha2: Alpha2::EG,
                        code: "SIN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.6084723), longitude: Some(33.617577), max_latitude: Some(31.3314092), min_latitude: Some(29.5932723), max_longitude: Some(34.8774622), min_longitude: Some(32.554564)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شمال سيناء"), ("bg", "Северен Синай"), ("bn", "নর\u{9cd}থ সিন\u{9be}ই গভর\u{9cd}নোরেট"), ("bs", "Sjeverni Sinaj"), ("ca", "Governació del Sinaí del Nord"), ("ccp", "𑄃\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄥\u{11128}𑄚\u{1112d}"), ("cs", "Severní Sinaj"), ("da", "Shamal Sina’"), ("de", "Schimal Sina"), ("el", "Κυβερνείο Βορείου Σινά"), ("en", "North Sinai"), ("es", "Sinaí del Norte"), ("et", "Põhja-Siinai kubernerkond"), ("fa", "استان سینای شمالی"), ("fi", "Šamal Sina’"), ("fr", "Gouvernorat du Sinaï Nord"), ("gu", "નોર\u{acd}થ સિનાઇ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "צפון סיני"), ("hi", "उत\u{94d}तर सीनाई म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Հյուսիսային Սինայ"), ("id", "Kegubernuran Syamal Sina’"), ("it", "Governatorato del Sinai del Nord"), ("ja", "北シナイ県"), ("ka", "ჩრდილოეთ სინაის მუჰაფაზა"), ("kn", "ಉತ\u{ccd}ತರ ಸ\u{cbf}ನೈ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "샤말시나 주"), ("lt", "Šiaurės Sinajaus muchafaza"), ("lv", "Ziemeļu Sinajas muhāfaza"), ("mk", "Северен Синај"), ("mr", "नॉर\u{94d}थ सिनाई गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "North Sinai Governorate"), ("nb", "Guvernementet Shamal Sina’"), ("nl", "Noord-Sinaï"), ("no", "Guvernementet Shamal Sina’"), ("pl", "Synaj Północny"), ("pt", "Sinai do Norte"), ("ro", "Shamal Sina’"), ("ru", "Северный Синай"), ("si", "උත\u{dd4}ර\u{dd4} ස\u{dd2}නය\u{dd2} පළ\u{dcf}ත"), ("sr", "Северни Синај"), ("sr_Latn", "Severni Sinaj"), ("sv", "Guvernementet Sina ash-Shamaliyya"), ("sw", "Mkoa wa Sinai Kaskazini"), ("ta", "வடக\u{bcd}கு சின\u{bbe}ய\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} స\u{c3f}న\u{c3e}య\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ชามาลซ\u{e34}นา"), ("tr", "Kuzey Sina"), ("uk", "Північний Синай"), ("ur", "محافظہ شمالی سینا"), ("vi", "Tỉnh Bắc Sinai"), ("zh", "北西奈省")]),
                        unofficial_name_list: ["Shamal Sina", "Sina aš-Šamālīyah", "Sinai ash-Shamaliyah", "Šamāl Sīna"].to_vec(),
                    }
                ),
                (
                    "SUZ",
                    Subdivision{
                        name: "SUZ",
                        country_alpha2: Alpha2::EG,
                        code: "SUZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.9668343), longitude: Some(32.5498069), max_latitude: Some(30.0211813), min_latitude: Some(29.9329452), max_longitude: Some(32.5854957), min_longitude: Some(32.4374941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "السويس"), ("bg", "Суец"), ("bn", "স\u{9c1}য\u{9bc}েজ গভর\u{9cd}নোরেট"), ("bs", "Suez"), ("ca", "Governació de Suez"), ("ccp", "𑄥\u{1112a}𑄠𑄬𑄌\u{11134}"), ("ceb", "As Suways"), ("cs", "Suez"), ("da", "Suez Governorate"), ("de", "as-Suwais"), ("el", "Κυβερνείο Σουέζ"), ("en", "Suez"), ("es", "Gobernación de Suez"), ("fa", "استان سوئز"), ("fi", "Al-Suwaisin kuvernoraatti"), ("fr", "Gouvernorat de Suez"), ("gu", "સ\u{ac1}એઝ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "סואץ"), ("hi", "स\u{941}एज\u{93c} म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hy", "Սուեզ"), ("id", "Kegubernuran As Suways"), ("it", "Governatorato di Suez"), ("ja", "スエズ県"), ("ka", "სუეცის მუჰაფაზა"), ("kn", "ಸುಯ\u{cc6}ಜ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "수에즈 주"), ("lt", "Sueco muchafaza"), ("lv", "Suecas muhāfaza"), ("mr", "स\u{941}एझ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Suez"), ("nb", "As Suways"), ("nl", "Suez"), ("no", "As Suways"), ("pl", "Suez"), ("pt", "Suez"), ("ro", "As Suways"), ("ru", "Суэц"), ("si", "ස\u{dd4}එස\u{dca} පළ\u{dcf}ත"), ("sr", "Суец"), ("sr_Latn", "Suec"), ("sv", "Suez"), ("sw", "Mkoa wa Suez"), ("te", "సూయ\u{c46}జ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองซ\u{e31}ลซ\u{e4c}"), ("tr", "Süveyş"), ("uk", "Суец"), ("ur", "محافظہ سوئیز"), ("vi", "Tỉnh Suez"), ("zh", "蘇伊士省")]),
                        unofficial_name_list: ["El Suweiz", "as-Suways"].to_vec(),
                    }
                ),
                (
                    "WAD",
                    Subdivision{
                        name: "WAD",
                        country_alpha2: Alpha2::EG,
                        code: "WAD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.5455638), longitude: Some(27.1735316), max_latitude: Some(27.6966292), min_latitude: Some(21.9915523), max_longitude: Some(32.9224299), min_longitude: Some(25.0003101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الوادي الجديد"), ("bg", "Уади ал Джадид"), ("bn", "নিউ ভ\u{9cd}য\u{9be}লি গভর\u{9cd}নোরেট"), ("bs", "Nova Dolina"), ("ca", "Governació de Wadi al Jadid"), ("ccp", "𑄚\u{11131} 𑄞𑄬𑄣\u{11128}"), ("cs", "Al-Vádí al-Gadíd"), ("da", "New Valley Governorate"), ("de", "al-Wadi al-dschadid"), ("el", "Νιου Βάλεϊ"), ("en", "New Valley"), ("es", "Nuevo Valle"), ("et", "Al-Wādī al-Jadīdi kubernerkond"), ("fa", "استان وادی\u{200c}الجدید"), ("fi", "Al-Wadi al-Gadid"), ("fr", "Gouvernorat de la Nouvelle-Vallée"), ("gu", "ન\u{acd}ય\u{ac2} વ\u{ac7}લી ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אל-ואדי אל-ג׳דיד"), ("hi", "वादी अल-जदीद म\u{941}हाफ\u{93c}ज\u{93c}ाह"), ("hr", "Al Wadi al Jadid"), ("id", "Al Wadi al Jadid"), ("it", "Governatorato di Wadi al-Jadid"), ("ja", "ニューバレー県"), ("ka", "ახალი ველის მუჰაფაზა"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ವ\u{ccd}ಯಾಲ\u{cbf} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "알와디알자디드 주"), ("lt", "Naujojo Slėnio muchafaza"), ("lv", "Vādīelgedīdas muhāfaza"), ("mr", "अल वाडी अल जदिद प\u{94d}रद\u{947}श"), ("ms", "New Valley Governorate"), ("nb", "Guvernementet Al Wadi al Jadid"), ("nl", "Nieuwe Vallei"), ("no", "Guvernementet Al Wadi al Jadid"), ("pl", "Nowa Dolina"), ("pt", "Vale Novo"), ("ro", "Al Wadi al Jadid"), ("ru", "Новая Долина"), ("si", "න\u{dd2}ව\u{dca} වැල\u{dd2} පළ\u{dcf}ත"), ("sk", "Al-Wádí al-Džadíd"), ("sr", "Нова Долина"), ("sr_Latn", "Nova Dolina"), ("sv", "Guvernementet Al-Wadi al-Jadid"), ("sw", "Mkoa wa Bonde la Mpya"), ("ta", "நியூ வ\u{bbe}லே கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "న\u{c4d}యూ వ\u{c4d}య\u{c3e}ల\u{c40} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตอ\u{e31}ลวะด\u{e35}อ\u{e31}ลจะด\u{e34}ด"), ("tr", "Yeni Vadi"), ("uk", "Нова Долина"), ("ur", "محافظہ وادی جدید"), ("vi", "Tỉnh New Valley"), ("zh", "新河谷省")]),
                        unofficial_name_list: ["El Wadi el Jadid", "El Wadi el Jedid"].to_vec(),
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
#[cfg(feature = "eg")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::EG,
        alpha3: Alpha3::EGY,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Africa,
        country_code: 20,
        currency_code: "EGP",
        gec: Some(GEC::EG),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("EGY"),
        iso_long_name: "The Arab Republic of Egypt",
        iso_short_name: "Egypt",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Egyptian"),
        number: "818",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::NorthernAfrica),
        un_locode: "EG",
        unofficial_name_list: [
            "Egypt",
            "مصر",
            "Ägypten",
            "Égypte",
            "Egipto",
            "エジプト",
            "Egypte",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Egypt"),
            ("af", "Egipte"),
            ("ak", "Egypt"),
            ("am", "ጔብፄ"),
            ("an", "Egypt"),
            ("ar", "مصر"),
            ("as", "মিশ\u{9cd}ৰ"),
            ("ay", "Egypt"),
            ("az", "Misir"),
            ("ba", "Egypt"),
            ("be", "Егіпет"),
            ("bg", "Египет"),
            ("bi", "Egypt"),
            ("bn", "মিশর"),
            ("bn_IN", "মিশর"),
            ("br", "Egipt"),
            ("bs", "Egipat"),
            ("ca", "Egipte"),
            ("ce", "Мисар"),
            ("ch", "Egypt"),
            ("cs", "Egypt"),
            ("cv", "Мисар"),
            ("cy", "Yr Aifft"),
            ("da", "Egypten"),
            ("de", "Ägypten"),
            ("dv", "މ\u{7a8}ޞ\u{7b0}ރ\u{7aa}"),
            ("dz", "ཨ\u{f72}་ཇ\u{f72}བཊ\u{f72}།"),
            ("ee", "Egypt"),
            ("el", "Αίγυπτος"),
            ("en", "Egypt"),
            ("eo", "Egiptio"),
            ("es", "Egipto"),
            ("et", "Egiptus"),
            ("eu", "Egipto"),
            ("fa", "مصر"),
            ("ff", "Egypt"),
            ("fi", "Egypti"),
            ("fo", "Egyptaland"),
            ("fr", "Égypte"),
            ("fy", "Egypte"),
            ("ga", "An Éigipt"),
            ("gl", "Exipto"),
            ("gn", "Egypt"),
            ("gu", "ઇજીપ\u{acd}ત"),
            ("gv", "Yn Egypt"),
            ("ha", "Misra"),
            ("he", "מצרים"),
            ("hi", "मिस\u{94d}र"),
            ("hr", "Egipat"),
            ("ht", "Ejip"),
            ("hu", "Egyiptom"),
            ("hy", "Եգիպտոս"),
            ("ia", "Egypto"),
            ("id", "Mesir"),
            ("io", "Egiptia"),
            ("is", "Egyptaland"),
            ("it", "Egitto"),
            ("iu", "Egypt"),
            ("ja", "エジプト"),
            ("ka", "ეგვიპტე"),
            ("ki", "Egypt"),
            ("kk", "Мысыр"),
            ("kl", "Egypt"),
            ("km", "អេហ\u{17d2}ស\u{17ca}\u{17b8}ប"),
            ("kn", "ಈಜ\u{cbf}ಪ\u{ccd}ಟ\u{ccd}"),
            ("ko", "이집트"),
            ("ku", "Misir"),
            ("kv", "Египет Араб Республика"),
            ("kw", "Ejyp"),
            ("ky", "Египет"),
            ("lo", "Egypt"),
            ("lt", "Egiptas"),
            ("lv", "Ēģipte"),
            ("mi", "Egypt"),
            ("mk", "Египет"),
            ("ml", "ഈജിപ\u{d4d}ത\u{d4d}"),
            ("mn", "Египт"),
            ("mr", "इजिप\u{94d}त"),
            ("ms", "Mesir"),
            ("mt", "Eġittu"),
            (
                "my",
                "အ\u{102e}ဂျစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Idjipt"),
            ("nb", "Egypt"),
            ("ne", "इजिप\u{94d}ट"),
            ("nl", "Egypte"),
            ("nn", "Egypt"),
            ("nv", "Egypt"),
            ("oc", "Egipte"),
            ("or", "ମ\u{b3f}ଶର"),
            ("pa", "ਮਿਸਰ"),
            ("pi", "ईजिप\u{94d}ट"),
            ("pl", "Egipt"),
            ("ps", "مصر"),
            ("pt", "Egito"),
            ("pt_BR", "Egito"),
            ("ro", "Egipt"),
            ("ru", "Египет"),
            ("rw", "Misiri"),
            ("sc", "Egitu"),
            ("sd", "مصر"),
            ("si", "ඊජ\u{dd2}ප\u{dca}ත\u{dd4}ව"),
            ("sk", "Egypt"),
            ("sl", "Egipt"),
            ("so", "Masar"),
            ("sq", "Egjipt"),
            ("sr", "Египат"),
            ("sv", "Egypten"),
            ("sw", "Egypt"),
            ("ta", "எகிப\u{bcd}து"),
            ("te", "ఈజ\u{c3f}ప\u{c4d}ట\u{c4d}"),
            ("tg", "Миср"),
            ("th", "อ\u{e35}ย\u{e34}ปต\u{e4c}"),
            ("ti", "ግብጺ"),
            ("tk", "Egypt"),
            ("tl", "Ehipto"),
            ("tr", "Mısır"),
            ("tt", "Мысыр"),
            ("ug", "مىسىر"),
            ("uk", "Єгипет"),
            ("ur", "مصر"),
            ("uz", "Misr"),
            ("ve", "Egypt"),
            ("vi", "Ai Cập"),
            ("wa", "Edjipe"),
            ("wo", "Ejipt"),
            ("xh", "Egypt"),
            ("yo", "Ẹ\u{301}gíptì"),
            ("zh_CN", "埃及"),
            ("zh_HK", "埃及"),
            ("zh_TW", "埃及"),
            ("zu", "IGibhithe"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}