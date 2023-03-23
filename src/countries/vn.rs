// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Socialist Republic of Viet Nam

#[cfg(all(feature = "vn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::VN;
    pub const ALPHA3: Alpha3 = Alpha3::VNM;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 84;
    pub const CURRENCY_CODE: &str = "VND";
    pub const GEC: Option<GEC> = Some(GEC::VM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("VIE");
    pub const ISO_SHORT_NAME: &str = "Viet Nam";
    pub const ISO_LONG_NAME: &str = "The Socialist Republic of Viet Nam";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["vi"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["vi"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Vietnamese");
    pub const NUMBER: &str = "704";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}\\d?");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "VN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Vietnam", "ベトナム", "Viet Nam"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Vietnam"),
        ("af", "Viëtnam"),
        ("ak", "Vietnam"),
        ("am", "ቬት ናም"),
        ("an", "Vietnam"),
        ("ar", "الفيتنام"),
        ("as", "ভিয়েতন\u{9be}ম"),
        ("ay", "Vietnam"),
        ("az", "Vietnam"),
        ("ba", "Vietnam"),
        ("be", "В'етнам"),
        ("bg", "Виетнам"),
        ("bi", "Vietnam"),
        ("bn", "ভিয়েতন\u{9be}ম"),
        ("bn_IN", "ভিয়েতন\u{9be}ম"),
        ("br", "Viêt Nam"),
        ("bs", "Vijetnam"),
        ("ca", "Vietnam"),
        ("ce", "Вьетнам"),
        ("ch", "Vietnam"),
        ("cs", "Vietnam"),
        ("cv", "Вьетнам"),
        ("cy", "Fietnam"),
        ("da", "Vietnam"),
        ("de", "Vietnam"),
        ("dv", "ވ\u{7a8}އ\u{7ac}ޓ\u{7aa}ނ\u{7a7}މ\u{7aa}"),
        ("dz", "ཝ\u{f7a}ཊ\u{f72}་ནམ།"),
        ("ee", "Vietnam"),
        ("el", "Βιετνάμ"),
        ("en", "Vietnam"),
        ("eo", "Vjetnamio"),
        ("es", "Vietnam"),
        ("et", "Vietnam"),
        ("eu", "Vietnam"),
        ("fa", "ویتنام"),
        ("ff", "Vietnam"),
        ("fi", "Vietnam"),
        ("fo", "Vietnam"),
        ("fr", "Viêt Nam"),
        ("fy", "Fjetnam"),
        ("ga", "Vítneam"),
        ("gl", "Vietnam"),
        ("gn", "Vietnam"),
        ("gu", "વિય\u{ac7}તનામ"),
        ("gv", "Yn Vietnam"),
        ("ha", "Vietnam"),
        ("he", "וייטנאם"),
        ("hi", "वियतनाम"),
        ("hr", "Vijetnam"),
        ("ht", "Vyetnam"),
        ("hu", "Vietnám"),
        ("hy", "Վիետնամ"),
        ("ia", "Vietnam"),
        ("id", "Vietnam"),
        ("io", "Vietnam"),
        ("is", "Víetnam"),
        ("it", "Vietnam"),
        ("iu", "Vietnam"),
        ("ja", "ベトナム"),
        ("ka", "ვიეტ-ნამი"),
        ("ki", "Vietnam"),
        ("kk", "Вьетнам"),
        ("kl", "Vietnam"),
        ("km", "វៀតណាម"),
        ("kn", "ವ\u{cbf}ಯ\u{cc6}ಟ\u{ccd}ನಾಂ"),
        ("ko", "베트남"),
        ("ku", "Viyetnam"),
        ("kv", "Вьетнам"),
        ("kw", "Vietnam"),
        ("ky", "Вьетнам"),
        ("lo", "ປະເທດຫວຽດນາມ"),
        ("lt", "Vietnamas"),
        ("lv", "Vjetnama"),
        ("mi", "Whitināmu"),
        ("mk", "Виетнам"),
        ("ml", "വിയറ\u{d4d}റ\u{d4d}ന\u{d3e}ം"),
        ("mn", "Vietnam"),
        ("mr", "व\u{94d}हिय\u{947}तनाम"),
        ("ms", "Vietnam"),
        ("mt", "Vjetnam"),
        (
            "my",
            "ဗ\u{102e}ယက\u{103a}နမ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Bitinam"),
        ("nb", "Vietnam"),
        ("ne", "भिएतनाम"),
        ("nl", "Vietnam"),
        ("nn", "Vietnam"),
        ("nv", "Vietnam"),
        ("oc", "Vietnam"),
        ("or", "ଭ\u{b3f}ୟେତନ\u{b3e}ମ"),
        ("pa", "ਵੀਅਤਨਾਮ"),
        ("pi", "विय\u{947}तनाम"),
        ("pl", "Wietnam"),
        ("ps", "Vietnam"),
        ("pt", "Vietname"),
        ("pt_BR", "Vietnã"),
        ("ro", "Vietnam"),
        ("ru", "Вьетнам"),
        ("rw", "Viyetinamu"),
        ("sc", "Vietnam"),
        ("sd", "ويٽنام"),
        ("si", "ව\u{dd2}යෙට\u{dca}න\u{dcf}මය"),
        ("sk", "Vietnam"),
        ("sl", "Vietnam"),
        ("so", "Fiyetnaam"),
        ("sq", "Vietnam"),
        ("sr", "Вијетнам"),
        ("sv", "Vietnam"),
        ("sw", "Vietnam"),
        ("ta", "வியட\u{bcd}ன\u{bbe}ம\u{bcd}"),
        ("te", "వ\u{c3f}యత\u{c4d}న\u{c3e}మ\u{c4d}"),
        ("tg", "Ветнам"),
        ("th", "เว\u{e35}ยดนาม"),
        ("ti", "ቬትናም"),
        ("tk", "Wýetnam"),
        ("tl", "Viet Nam"),
        ("tr", "Vietnam"),
        ("tt", "Виет Нам"),
        ("ug", "ۋيېتنام"),
        ("uk", "В'єтнам"),
        ("ur", "ویتنام"),
        ("uz", "Vyetnam"),
        ("ve", "Viëtnam"),
        ("vi", "Việt Nam"),
        ("wa", "Vietnam"),
        ("wo", "Wiyet Naam"),
        ("xh", "Vietnam"),
        ("yo", "Fiẹtnám"),
        ("zh_CN", "越南"),
        ("zh_HK", "越南"),
        ("zh_TW", "越南"),
        ("zu", "IViyetnami"),
    ];
    #[cfg(all(feature = "vn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 14.058324;
        pub const LONGITUDE: f64 = 108.277199;
        pub const MAX_LATITUDE: f64 = 23.3926504;
        pub const MAX_LONGITUDE: f64 = 109.6765;
        pub const MIN_LATITUDE: f64 = 8.1952001;
        pub const MIN_LONGITUDE: f64 = 102.1439156;
        pub const NORTHEAST_LATITUDE: f64 = 23.3926504;
        pub const NORTHEAST_LONGITUDE: f64 = 109.6765;
        pub const SOUTHWEST_LATITUDE: f64 = 8.1952001;
        pub const SOUTHWEST_LONGITUDE: f64 = 102.1439156;
    }
}
#[cfg(all(feature = "vn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 14.058324,
            longitude: 108.277199,
            max_latitude: 23.3926504,
            max_longitude: 109.6765,
            min_latitude: 8.1952001,
            min_longitude: 102.1439156,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 23.3926504,
                    longitude: 109.6765,
                },
                southwest: CountryGeoBound {
                    latitude: 8.1952001,
                    longitude: 102.1439156,
                },
            },
        }
    }
}

#[cfg(all(feature = "vn", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::VN,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.3686613), longitude: Some(103.3119085), max_latitude: Some(22.8214739), min_latitude: Some(21.6847368), max_longitude: Some(103.985241), min_longitude: Some(102.3274711)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لاي تشاو"), ("bg", "Лай Тяу"), ("bn", "ল\u{9be}ই চ\u{9be}ও"), ("ca", "Lai Châu"), ("ccp", "𑄣\u{1112d} 𑄌𑄅\u{1112a}"), ("ceb", "Tỉnh Lai Châu"), ("cs", "Lai Chau"), ("da", "Lai Châu"), ("de", "Lai Châu"), ("el", "Λάι Τσάου"), ("en", "Lai Châu"), ("es", "Lai Châu"), ("fa", "استان لای چو"), ("fi", "Lai Châu"), ("fr", "Province de Lai Châu"), ("gu", "લાઇ ચ\u{ac1}"), ("hi", "लाइ चाउ प\u{94d}रा\u{902}त"), ("id", "Provinsi Lai Chau"), ("it", "provincia di Lai Chau"), ("ja", "ライチャウ省"), ("kn", "ಲೈ ಚ\u{cbf}ಯು"), ("ko", "라이쩌우 성"), ("lt", "Lai Čau provincija"), ("lv", "Lajķou province"), ("mr", "लाइ चाउ"), ("ms", "Lai Chau"), ("nb", "La Chau"), ("nl", "Lai Châu"), ("no", "La Chau"), ("pl", "Prowincja Lai Châu"), ("pt", "Lai Chau"), ("ro", "Lai Châu"), ("ru", "Лайтяу"), ("si", "ල\u{dcf}ය\u{dd2} ච\u{dcf}උ"), ("sv", "Lai Chau"), ("sw", "Mkoa wa Lai Châu"), ("ta", "ல\u{bbe}ய\u{bcd} ச\u{bbe}வு"), ("te", "ల\u{c3e}య\u{c3f} చ\u{c3e}వూ"), ("th", "จ\u{e31}งหว\u{e31}ดลายเจ\u{e34}ว"), ("tr", "Lai Chau"), ("uk", "Лайтяу"), ("ur", "لائی چاو صوبہ"), ("vi", "Lai Châu"), ("yue", "萊州"), ("yue_Hans", "莱州"), ("zh", "萊州省")]),
                        unofficial_name_list: ["Lai Chau"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::VN,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.3380865), longitude: Some(104.1487055), max_latitude: Some(22.848793), min_latitude: Some(21.8772199), max_longitude: Some(104.626443), min_longitude: Some(103.529518)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لاو كاي"), ("bg", "Лао Кай"), ("bn", "ল\u{9be}ও ক\u{9be}য\u{9bc}"), ("ccp", "𑄣𑄃\u{1112e} 𑄥\u{1112d}"), ("ceb", "Tỉnh Lào Cai"), ("cs", "Provincie Lao Cai"), ("da", "Lào Cai"), ("de", "Lào Cai"), ("el", "Λάο Κάι"), ("en", "Lào Cai"), ("es", "Lào Cai"), ("fa", "استان لائو کای"), ("fi", "Lào Cai"), ("fr", "Province de Lào Cai"), ("gu", "લાઓ કાઈ"), ("hi", "लाओ काई प\u{94d}रा\u{902}त"), ("id", "Provinsi Lao Cai"), ("it", "provincia di Lao Cai"), ("ja", "ラオカイ省"), ("kn", "ಲಾಯ\u{ccd} ಕೈ"), ("ko", "라오까이 성"), ("lt", "Lao Kai provincija"), ("lv", "Laokajas province"), ("mn", "Лау Гаая"), ("mr", "लाओ काई"), ("ms", "Lao Cai"), ("nb", "Lao Cai"), ("nl", "Lào Cai"), ("no", "Lao Cai"), ("pl", "Prowincja Lào Cai"), ("pt", "Lao Cai"), ("ro", "Lào Cai"), ("ru", "Лаокай"), ("si", "ල\u{dcf}ඕ ක\u{dcf}ය\u{dd2}"), ("sv", "Lao Cai"), ("sw", "Mkoa wa Lào Cai"), ("ta", "ல\u{bbe}வோ க\u{bbe}ய\u{bcd}"), ("te", "ల\u{c3e}వ\u{c4b} క\u{c3e}య\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดหล\u{e48}าวกาย"), ("tr", "Lao Cai"), ("uk", "Лаокай"), ("ur", "لاو کائے صوبہ"), ("vi", "Lào Cai"), ("yue", "老街"), ("yue_Hans", "老街"), ("zh", "老街省")]),
                        unofficial_name_list: ["Lao Cai"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::VN,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.7662056), longitude: Some(104.9388853), max_latitude: Some(23.3888341), min_latitude: Some(22.166518), max_longitude: Some(105.5752411), min_longitude: Some(104.3361501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ها جيانج"), ("bg", "Ха Жианг"), ("bn", "হ\u{9be} জিয\u{9bc}\u{9be}ং"), ("ccp", "𑄦 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "Tỉnh Hà Giang"), ("cs", "Ha Giang (provincie)"), ("da", "Hà Giang"), ("de", "Hà Giang"), ("el", "Χα Γκιάνγκ"), ("en", "Hà Giang"), ("es", "Hà Giang"), ("fa", "استان ها گیانگ"), ("fi", "Hà Giang"), ("fr", "Province de Hà Giang"), ("gu", "હા ગિઆ\u{a82}ગ"), ("hi", "हा जिया\u{902}ग"), ("id", "Provinsi Ha Giang"), ("it", "provincia di Ha Giang"), ("ja", "ハザン省"), ("kn", "ಹ\u{ccd}ಯಾ ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "하장 성"), ("lt", "Chadžiangas"), ("lv", "Hazana"), ("mn", "Хай Жяан"), ("mr", "हा जिआ\u{902}ग"), ("ms", "Ha Giang"), ("nb", "Ha Giang"), ("nl", "Hà Giang"), ("no", "Ha Giang"), ("pl", "Prowincja Hà Giang"), ("pt", "Ha Giang"), ("ru", "Хазянг"), ("si", "හ\u{dcf} ජ\u{dd2}යැන\u{dca}ග\u{dca}"), ("sv", "Ha Giang"), ("sw", "Mkoa wa Hà Giang"), ("ta", "ஹ\u{bbe} ஜிய\u{bbe}ங\u{bcd}"), ("te", "హ\u{c3e} గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดห\u{e48}าซาง"), ("tr", "Ha Giang"), ("uk", "Хазянг"), ("ur", "ہا گیانگ صوبہ"), ("vi", "Hà Giang"), ("yue", "河江"), ("yue_Hans", "河江"), ("zh", "河江省")]),
                        unofficial_name_list: ["Ha Giang"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::VN,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.635689), longitude: Some(106.2522143), max_latitude: Some(23.1186219), min_latitude: Some(22.35741), max_longitude: Some(106.826317), min_longitude: Some(105.2724999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاو بانج"), ("bg", "Као Банг"), ("bn", "ক\u{9be}ও ব\u{9cd}য\u{9be}ং"), ("ccp", "𑄇𑄃\u{1112e} 𑄝\u{11101}"), ("ceb", "Tỉnh Cao Bằng"), ("cs", "Cao Bang"), ("da", "Cao Bằng"), ("de", "Cao Bằng"), ("el", "Κάο Μπάνγκ"), ("en", "Cao Bằng"), ("es", "Cao Bằng"), ("fa", "استان کائو بانگ"), ("fi", "Cao Bằng"), ("fr", "Province de Cao Bằng"), ("gu", "કાઓ બા\u{a82}ન\u{acd}ગ"), ("hi", "क\u{947}ओ ब\u{948}\u{902}ग"), ("id", "Provinsi Cao Bang"), ("it", "provincia di Cao Bang"), ("ja", "カオバン省"), ("kn", "ಕಾವೊ ಬ\u{ccd}ಯಾಂಗ\u{ccd}"), ("ko", "까오방 성"), ("lt", "Kao Bangas"), ("lv", "Kaobana"), ("mn", "Гау Бан"), ("mr", "काओ बि\u{902}ग"), ("ms", "Cao Bang"), ("nb", "Cao Bang"), ("nl", "Cao Bằng"), ("no", "Cao Bang"), ("pl", "Prowincja Cao Bằng"), ("pt", "Cao Bang"), ("ru", "Каобанг"), ("si", "කඕ බෑන\u{dca}ග\u{dca}"), ("sv", "Cao Bang"), ("sw", "Mkoa wa Cao Bằng"), ("ta", "கேயோ ப\u{bbe}ங\u{bcd}"), ("te", "క\u{c3e}వ\u{c4b} బ\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาวบ\u{e31}\u{e48}ง"), ("tr", "Cao Bằng ili"), ("uk", "Каобанг"), ("ur", "کاؤ بانگ صوبہ"), ("vi", "Cao Bằng"), ("yue", "高平"), ("yue_Hans", "高平"), ("zh", "高平省")]),
                        unofficial_name_list: ["Cao Bang"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::VN,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.3270341), longitude: Some(103.9141288), max_latitude: Some(21.4172762), min_latitude: Some(21.2237815), max_longitude: Some(104.0360641), min_longitude: Some(103.8084413)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سن لا"), ("bg", "Сон Ла"), ("bn", "সন ল\u{9be}"), ("ccp", "𑄥\u{11127}𑄚\u{11134} 𑄣"), ("ceb", "Tỉnh Sơn La"), ("cs", "Son La"), ("da", "Sơn La"), ("de", "Sơn La"), ("el", "Σον Λα"), ("en", "Sơn La"), ("es", "Sơn La"), ("fa", "استان سون لا"), ("fi", "Sơn La"), ("fr", "Province de Sơn La"), ("gu", "સોન લા"), ("hi", "सॉन ला"), ("id", "Provinsi Son La"), ("it", "provincia di Son La"), ("ja", "ソンラ省"), ("kn", "ಸೋನ\u{ccd} ಲಾ"), ("ko", "선라 성"), ("lt", "Sonla"), ("lv", "Sonla"), ("mr", "सोन ला"), ("ms", "Son La"), ("nb", "Son La"), ("nl", "Sơn La"), ("no", "Son La"), ("pl", "Prowincja Sơn La"), ("pt", "Son La"), ("ro", "Sơn La"), ("ru", "Шонла"), ("si", "සෝන\u{dca} ල\u{dcf}"), ("sr", "Сон Ла"), ("sr_Latn", "Son La"), ("sv", "Son La"), ("sw", "Mkoa wa Sơn La"), ("ta", "சன\u{bcd} ல\u{bbe}"), ("te", "స\u{c4b}న\u{c4d} ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดเซ\u{e34}นลา"), ("tr", "Son la"), ("uk", "Шонла"), ("ur", "سون لا صوبہ"), ("vi", "Sơn La"), ("yue", "山羅"), ("yue_Hans", "山罗"), ("zh", "山羅省")]),
                        unofficial_name_list: ["Son La"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::VN,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.6837923), longitude: Some(104.4551361), max_latitude: Some(22.291081), min_latitude: Some(21.3273449), max_longitude: Some(105.100925), min_longitude: Some(103.887402)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة وين باي"), ("bg", "Йен Бай"), ("bn", "ইয\u{9bc}েন ব\u{9be}ই"), ("ccp", "𑄃\u{11128}𑄠𑄬𑄚\u{11134} 𑄝\u{1112d}"), ("ceb", "Tỉnh Yên Bái"), ("cs", "Yen Bai"), ("da", "Yen Bai"), ("de", "Yên Bái"), ("el", "Γιέν Μπάι"), ("en", "Yên Bái"), ("es", "Yên Bái"), ("fa", "استان ین بای"), ("fi", "Yên Bái"), ("fr", "Province de Yên Bái"), ("gu", "ય\u{ac7}ન બ\u{ac8}"), ("hi", "य\u{947}न बाई प\u{94d}रा\u{902}त"), ("id", "Provinsi Yen Bai"), ("it", "provincia di Yen Bai"), ("ja", "イエンバイ省"), ("kn", "ಯ\u{cc6}ನ\u{ccd} ಬಾಯ\u{ccd}"), ("ko", "옌바이 성"), ("lt", "Jen Bėjus"), ("lv", "Jenbajas province"), ("mn", "Иэньбай"), ("mr", "य\u{947}न बाई"), ("ms", "Yen Bai"), ("nb", "Yen Bai"), ("nl", "Yên Bái"), ("no", "Yen Bai"), ("pl", "Prowincja Yên Bái"), ("pt", "Yen Bai"), ("ro", "Yên Bái"), ("ru", "Йенбай"), ("si", "යේන\u{dca} බ\u{dcf}ය\u{dd2}"), ("sv", "Yen Bai"), ("sw", "Mkoa wa Yên Bái"), ("ta", "ஏன\u{bcd} பை"), ("te", "య\u{c46}న\u{c4d} బ\u{c47}"), ("th", "จ\u{e31}งหว\u{e31}ดเอ\u{e35}ยนบ\u{e4a}าย"), ("tr", "Yen Bai"), ("uk", "Єнбай"), ("ur", "یین با پراونس"), ("vi", "Yên Bái"), ("yue", "安沛"), ("yue_Hans", "安沛"), ("zh", "安沛省")]),
                        unofficial_name_list: ["Yen Bai"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::VN,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.1726708), longitude: Some(105.3131185), max_latitude: Some(22.694384), min_latitude: Some(21.501763), max_longitude: Some(105.597397), min_longitude: Some(104.848572)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة توين كوانج"), ("bg", "Туйен Куанг"), ("bn", "ত\u{9c1}য\u{9bc}েন ক\u{9c1}য\u{9bc}\u{9be}ং"), ("ccp", "𑄑\u{1112a}𑄠𑄬𑄚\u{11134} 𑄇\u{1112a}𑄠𑄬𑄚\u{11134}"), ("ceb", "Tỉnh Tuyên Quang"), ("da", "Tuyên Quang"), ("de", "Tuyên Quang"), ("el", "Τουγιέν Κουάνγκ"), ("en", "Tuyên Quang"), ("es", "Tuyên Quang"), ("fa", "استان توین کوانگ"), ("fi", "Tuyên Quang"), ("fr", "Province de Tuyên Quang"), ("gu", "ત\u{ac1}એન ક\u{acd}વા\u{a82}ગ"), ("hi", "त\u{941}एन क\u{948}\u{902}ग"), ("id", "Provinsi Tuyen Quang"), ("it", "provincia di Tuyen Quang"), ("ja", "トゥエンクアン省"), ("kn", "ತುಯ\u{cc6}ನ\u{ccd} ಕ\u{ccd}ವಾಂಗ\u{ccd}"), ("ko", "뚜옌꽝 성"), ("lt", "Tujen Kvango provincija"), ("lv", "Tujenkuana"), ("mn", "Туэньгуан"), ("mr", "त\u{941}य\u{947}न क\u{941}आ\u{902}ग"), ("ms", "Tuyen Quang"), ("nb", "Tuyenn Quang"), ("nl", "Tuyên Quang"), ("no", "Tuyenn Quang"), ("pl", "Prowincja Tuyên Quang"), ("pt", "Tuyen Quang"), ("ro", "Tuyên Quang (provincie)"), ("ru", "Туенкуанг"), ("si", "ට\u{dd4}යෙන\u{dca} ක\u{dd4}ව\u{dcf}න\u{dca}ග\u{dca}"), ("sr", "Тујен Кванг"), ("sr_Latn", "Tujen Kvang"), ("sv", "Tuyen Quang"), ("sw", "Mkoa wa Tuyên Quang"), ("ta", "த\u{bbe}யின\u{bcd} குஅங\u{bcd}"), ("te", "టుయ\u{c46}న\u{c4d} క\u{c4d}వ\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเตว\u{e35}ยนกวาง"), ("tr", "Tuyen Quang"), ("uk", "Туєнкуанг (провінція)"), ("ur", "توین قوانگ صوبہ"), ("vi", "Tuyên Quang"), ("yue", "宣光"), ("yue_Hans", "宣光"), ("zh", "宣光省")]),
                        unofficial_name_list: ["Tuyen Quang"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::VN,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.8563705), longitude: Some(106.6291304), max_latitude: Some(22.4613169), min_latitude: Some(21.3245939), max_longitude: Some(107.370491), min_longitude: Some(106.0948229)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لانغ صن"), ("bg", "Ланг Сон"), ("bn", "ল\u{9cd}য\u{9be}ং সন"), ("ccp", "𑄣\u{11101} 𑄥\u{11127}𑄚\u{11134}"), ("ceb", "Tỉnh Lạng Sơn"), ("cs", "Lang Son"), ("da", "Lang Son"), ("de", "Lạng Sơn"), ("el", "Λανγκ Σον"), ("en", "Lạng Sơn"), ("es", "Lạng Sơn"), ("fa", "استان لانگ سون"), ("fi", "Lạng Sơn"), ("fr", "Province de Lạng Sơn"), ("gu", "લા\u{a82}ગ સોન"), ("hi", "ला\u{902}ग सोन"), ("id", "Provinsi Lang Son"), ("it", "provincia di Lang Son"), ("ja", "ランソン省"), ("kn", "ಲಾಂಗ\u{ccd} ಸೋನ\u{ccd}"), ("ko", "랑선 성"), ("lt", "Langšonas"), ("lv", "Lanšona"), ("mn", "Ланшонь"), ("mr", "ला\u{901}ग सोन"), ("ms", "Lang Son"), ("nb", "Lang Son"), ("nl", "Lạng Sơn"), ("no", "Lang Son"), ("pl", "Prowincja Lạng Sơn"), ("pt", "Lang Son"), ("ro", "Lạng Sơn"), ("ru", "Лангшон"), ("si", "ලැන\u{dca}ග\u{dca} සෝන\u{dca}"), ("sr", "Ланг Сон"), ("sr_Latn", "Lang Son"), ("sv", "Lang Son"), ("sw", "Mkoa wa Lạng Sơn"), ("ta", "ல\u{bbe}ங\u{bcd} சன\u{bcd}"), ("te", "ల\u{c3e}ంగ\u{c4d} స\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดหล\u{e31}\u{e48}งเซ\u{e34}น"), ("tr", "Lang Son"), ("uk", "Лангшон"), ("ur", "لانگ سون صوبہ"), ("vi", "Lạng Sơn"), ("yue", "諒山"), ("yue_Hans", "谅山"), ("zh", "諒山省")]),
                        unofficial_name_list: ["Lang Son"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::VN,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.006382), longitude: Some(107.2925144), max_latitude: Some(21.6654891), min_latitude: Some(20.7164602), max_longitude: Some(108.0736009), min_longitude: Some(106.439682)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوانج ننه"), ("bg", "Куанг Нин"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ং নিন"), ("ccp", "𑄇\u{1112a}𑄠\u{11101} 𑄚\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Quảng Ninh"), ("cs", "Quang Ninh"), ("da", "Quảng Ninh"), ("de", "Quảng Ninh"), ("el", "Κουάνγκ Νινχ"), ("en", "Quảng Ninh"), ("es", "Quảng Ninh"), ("fa", "استان کوانگ نین"), ("fi", "Quảng Ninh"), ("fr", "Province de Quảng Ninh"), ("gu", "ક\u{acd}વા\u{a82}ગ નિ\u{a82}હ"), ("hi", "क\u{94d}व\u{948}\u{902}ग निन\u{94d}ह"), ("id", "Provinsi Quang Ninh"), ("it", "provincia di Quang Ninh"), ("ja", "クアンニン省"), ("kn", "ಕ\u{ccd}ವಾಂಗ\u{ccd} ನ\u{cbf}ನ\u{ccd}ಹ\u{ccd}"), ("ko", "꽝닌 성"), ("lt", "Kuang Nino provincija"), ("lv", "Kuangniņa"), ("mn", "Гуан Нэн"), ("mr", "क\u{94d}वि\u{902}ग निन\u{94d}ह"), ("ms", "Quang Ninh"), ("nb", "Quang Ninh"), ("nl", "Quảng Ninh"), ("no", "Quang Ninh"), ("pl", "Prowincja Quảng Ninh"), ("pt", "Quang Ninh"), ("ro", "Quảng Ninh"), ("ru", "Куангнинь"), ("si", "ක\u{dd4}ආන\u{dca}ග\u{dca} න\u{dd2}න\u{dca}හ\u{dca}"), ("sr", "Кванг Нин"), ("sr_Latn", "Kvang Nin"), ("sv", "Quang Ninh"), ("sw", "Mkoa wa Quảng Ninh"), ("ta", "குணங\u{bcd} நின\u{bcd}ஹ\u{bcd}"), ("te", "క\u{c4d}వ\u{c3e}ంంగ\u{c4d} న\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกว\u{e4b}างน\u{e34}ญ"), ("tr", "Quang Ninh"), ("uk", "Куангнінь"), ("ur", "قوانگ ننہ صوبہ"), ("vi", "Quảng Ninh"), ("yue", "廣寧"), ("yue_Hans", "广宁"), ("zh", "廣寧省")]),
                        unofficial_name_list: ["Quang Ninh"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::VN,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.6861265), longitude: Some(105.3131185), max_latitude: Some(21.1126179), min_latitude: Some(20.3047901), max_longitude: Some(105.8611979), min_longitude: Some(104.8349999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هوا بنه"), ("bg", "Хоа Бин"), ("bn", "হোয\u{9bc}\u{9be} বিহ\u{9cd}ন"), ("ccp", "𑄦\u{1112e}𑄠 𑄝\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Hòa Bình"), ("cs", "Hoa Binh"), ("da", "Hòa Bình"), ("de", "Hòa Bình"), ("el", "Χόα Μπινχ"), ("en", "Hòa Bình"), ("es", "Hòa Bình"), ("fa", "استان هوا بین"), ("fi", "Hòa Bình"), ("fr", "Province de Hòa Bình"), ("gu", "હો બિન\u{acd}હ"), ("hi", "हो बिन\u{94d}ह"), ("id", "Provinsi Hoa Binh"), ("it", "provincia di Hoa Binh"), ("ja", "ホアビン省"), ("kn", "ಹ\u{ccc} ಬ\u{cbf}ನ\u{ccd}"), ("ko", "호아빈 성"), ("lt", "Hoa Binas"), ("lv", "Hoabiņa"), ("mn", "Хуа Бэн"), ("mr", "हो बिन"), ("ms", "Hoa Binh"), ("nb", "Hoa Binh"), ("nl", "Hòa Bình"), ("no", "Hoa Binh"), ("pl", "Prowincja Hoà Bình"), ("pt", "Hoa Binh"), ("ru", "Хоабинь"), ("si", "හොආ බ\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Hoa Binh"), ("sw", "Mkoa wa Hòa Bình"), ("ta", "ஹோஆ பின\u{bcd}ஹ\u{bcd}"), ("te", "హ\u{c4b}వ\u{c3e} బ\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮหว\u{e48}าบ\u{e34}\u{e48}ญ"), ("tr", "Hoa Binh"), ("uk", "Хоабінь"), ("ur", "ہوا بنہ صوبہ"), ("vi", "Hòa Bình"), ("yue", "和平"), ("yue_Hans", "和平"), ("zh", "和平省")]),
                        unofficial_name_list: ["Hoa Binh"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::VN,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.2129969), longitude: Some(105.92299), max_latitude: Some(20.4552341), min_latitude: Some(19.9628219), max_longitude: Some(106.1685398), min_longitude: Some(105.5424731)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ninh Bình"), ("ar", "مقاطعة ننه بنه"), ("bg", "Нин Бин"), ("bn", "নিহ\u{9cd}ন বিহ\u{9cd}ন"), ("ccp", "𑄚\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄝\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Ninh Bình"), ("cs", "Ninh Binh"), ("da", "Ninh Bình"), ("de", "Ninh Bình"), ("el", "Νινχ Μπινχ"), ("en", "Ninh Bình"), ("es", "Ninh Binh"), ("fa", "استان نین بین"), ("fi", "Ninh Bình"), ("fr", "Province de Ninh Bình"), ("gu", "નિન\u{acd}હ બિ\u{a82}હ"), ("hi", "निन\u{94d}ह बिन\u{94d}ह"), ("id", "Provinsi Ninh Bình"), ("is", "Ninh Bình"), ("it", "provincia di Ninh Binh"), ("ja", "ニンビン省"), ("kn", "ನ\u{cbf}ನ\u{ccd}ಹ\u{ccd} ಬ\u{cbf}ನ\u{ccd}ಹ\u{ccd}"), ("ko", "닌빈 성"), ("lt", "Nin Binas"), ("lv", "Niņbiņa"), ("mn", "Нэн Бэн"), ("mr", "निन\u{94d}ह बिन\u{94d}ह"), ("ms", "Ninh Binh"), ("nb", "Ninh Bình"), ("nl", "Ninh Bình"), ("no", "Ninh Bình"), ("pl", "Prowincja Ninh Bình"), ("pt", "Ninh Binh"), ("ro", "Ninh Bình"), ("ru", "Ниньбинь"), ("si", "න\u{dd2}න\u{dca}හ\u{dca} බ\u{dd2}න\u{dca}හ\u{dca}"), ("sr", "Нин Бин"), ("sr_Latn", "Nin Bin"), ("sv", "Ninh Binh"), ("sw", "Mkoa wa Ninh Bình"), ("ta", "நின\u{bcd}ஹ\u{bcd} பின\u{bcd}ஹ\u{bcd}"), ("te", "న\u{c3f}న\u{c4d}హ\u{c4d} బ\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e34}ญบ\u{e34}\u{e48}ญ"), ("tr", "Ninh Bình"), ("uk", "Ніньбінь"), ("ur", "ننہ بنہ صوبہ"), ("vi", "Ninh Bình"), ("yue", "寧平"), ("yue_Hans", "宁平"), ("zh", "寧平省")]),
                        unofficial_name_list: ["Ninh Binh"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::VN,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.4463471), longitude: Some(106.3365828), max_latitude: Some(20.5060988), min_latitude: Some(20.3997838), max_longitude: Some(106.3930608), min_longitude: Some(106.2962436)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ثاي بنه"), ("bg", "Тхай Бин"), ("bn", "ত\u{9be}ই বিন"), ("ca", "Thái Bình"), ("ccp", "𑄗\u{1112d} 𑄝\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Thái Bình"), ("cs", "Thai Binh"), ("da", "Thai Bình"), ("de", "Thái Bình"), ("el", "Τάι Μπινχ"), ("en", "Thái Bình"), ("es", "Thái Bình"), ("fa", "استان تای بین"), ("fi", "Thái Bình"), ("fr", "Province de Thái Bình"), ("gu", "થાઈ બિ\u{a82}હ"), ("hi", "थाई बिन प\u{94d}रा\u{902}त"), ("id", "Provinsi Thai Binh"), ("it", "provincia di Thai Binh"), ("ja", "タイビン省"), ("kn", "ಥ\u{cbf} ಬ\u{cc6}ನ\u{ccd}"), ("ko", "타이빈 성"), ("lt", "Tai Binas"), ("lv", "Thajbiņas province"), ("mn", "Таая Бэн"), ("mr", "थाई बिन\u{94d}ह"), ("ms", "Thai Binh"), ("nb", "Thái Bình"), ("nl", "Thái Bình"), ("no", "Thái Bình"), ("pl", "Prowincja Thái Bình"), ("pt", "Thai Binh"), ("ro", "Thái Bình"), ("ru", "Тхайбинь"), ("si", "ත\u{dcf}ය\u{dd2} බ\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Thai Binh"), ("sw", "Mkoa wa Thái Bình"), ("ta", "த\u{bbe}ய\u{bcd} பிநஹ\u{bcd}"), ("te", "థ\u{c3e}\u{c3e}య\u{c3f} బ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e49}ายบ\u{e34}\u{e48}ญ"), ("tr", "Tha, Binh"), ("uk", "Тхайбінь"), ("ur", "تھائی بنہ صوبہ"), ("vi", "Thái Bình"), ("yue", "太平"), ("yue_Hans", "太平"), ("zh", "太平省")]),
                        unofficial_name_list: ["Thai Binh"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::VN,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.1291279), longitude: Some(105.3131185), max_latitude: Some(20.6708141), min_latitude: Some(19.2866772), max_longitude: Some(106.0758351), min_longitude: Some(104.378349)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تان هوا"), ("bg", "Тхан Хоа"), ("bn", "থ\u{9be}হ\u{9cd}ন হোয\u{9bc}\u{9be}"), ("ccp", "𑄗𑄚\u{11134}𑄦\u{11134} 𑄦\u{1112e}𑄠"), ("ceb", "Tỉnh Thanh Hóa"), ("cs", "Thanh Hoa"), ("da", "Thanh Hóa"), ("de", "Thanh Hóa"), ("el", "Θανχ Χόα"), ("en", "Thanh Hóa"), ("es", "Thanh Hóa"), ("fa", "تان هوا (استان)"), ("fi", "Thanh Hóa"), ("fr", "Province de Thanh Hóa"), ("gu", "થાન હોયા"), ("hi", "था\u{902}ह होआ"), ("id", "Provinsi Thanh Hoa"), ("it", "provincia di Thanh Hoa"), ("ja", "タインホア省"), ("kn", "ಥಾನ\u{ccd} ಹೊಯಾ"), ("ko", "타인호아 성"), ("lt", "Tan Choa provincija"), ("lv", "Thaņhoa province"), ("mr", "थान हो"), ("ms", "Thanh Hoa"), ("nb", "Thanh Hoa"), ("nl", "Thanh Hóa"), ("no", "Thanh Hoa"), ("pl", "Prowincja Thanh Hóa"), ("pt", "Thanh Hoa"), ("ro", "Thanh Hóa"), ("ru", "Тханьхоа"), ("si", "තනහ\u{dca} හොආ"), ("sv", "Thanh Hoa"), ("sw", "Mkoa wa Thanh Hóa"), ("ta", "தன\u{bcd}ஹ ஹோஆ"), ("te", "త\u{c3e}న\u{c4d}హ\u{c4d} హ\u{c4b}వ\u{c3e}"), ("th", "ธานห\u{e4c}โฮ"), ("tr", "Thanh Hóa"), ("uk", "Тханьхоа"), ("ur", "تھان ہوا صوبہ"), ("vi", "Thanh Hóa"), ("yue", "清化"), ("yue_Hans", "清化"), ("zh", "清化省")]),
                        unofficial_name_list: ["Thanh Hoa"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::VN,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.2342489), longitude: Some(104.9200365), max_latitude: Some(19.999296), min_latitude: Some(18.5531651), max_longitude: Some(105.806644), min_longitude: Some(103.876259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ني أن"), ("bg", "Нге Ан"), ("bn", "নেহেয\u{9bc}\u{9be}ন"), ("ccp", "𑄊𑄬 𑄃𑄚\u{11134}"), ("ceb", "Tỉnh Nghệ An"), ("cs", "Nghệ An"), ("da", "Nghệ An"), ("de", "Nghệ An"), ("el", "Νγκχε Αν"), ("en", "Nghệ An"), ("es", "Nghệ An"), ("fa", "استان نه آن"), ("fi", "Nghệ An"), ("fr", "Province de Nghệ An"), ("gu", "એન\u{acd}ઘ\u{ac7} એન"), ("hi", "नाघिए एन"), ("hy", "Նգեան"), ("id", "Provinsi Nghệ An"), ("it", "provincia di Nghe An"), ("ja", "ゲアン省"), ("kn", "ನ\u{ccd}ಘೇ ಆನ\u{ccd}"), ("ko", "응에안 성"), ("lt", "Ngeanas"), ("lv", "Ngeana"), ("mk", "Нге Ан"), ("mr", "नघ\u{947} अन"), ("ms", "Nghe An"), ("nb", "Nghe An"), ("nl", "Nghệ An"), ("no", "Nghe An"), ("pl", "Prowincja Nghệ An"), ("pt", "Nghe An"), ("ro", "Nghệ An"), ("ru", "Нгеан"), ("si", "එන\u{dca}ඝේ අන\u{dca}"), ("sl", "Nghe An"), ("sv", "Nghe An"), ("sw", "Mkoa wa Nghệ An"), ("ta", "நஃஹ\u{bc0} அன\u{bcd}"), ("te", "న\u{c47}గ\u{c4d} ఆన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเหงะอาน"), ("tr", "Nghe An"), ("uk", "Нгеан"), ("ur", "نگہ آن صوبہ"), ("vi", "Nghệ An"), ("yue", "乂安"), ("yue_Hans", "乂安"), ("zh", "乂安省")]),
                        unofficial_name_list: ["Nghe An"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::VN,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.2943776), longitude: Some(105.6745247), max_latitude: Some(18.7626158), min_latitude: Some(17.915977), max_longitude: Some(106.5042068), min_longitude: Some(105.108635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ها تنه"), ("bg", "Ха Тин"), ("bn", "হ\u{9be} তিহ\u{9cd}ন"), ("ccp", "𑄦 𑄑\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Hà Tĩnh"), ("cs", "Ha Tinh"), ("da", "Hà Tĩnh"), ("de", "Hà Tĩnh"), ("el", "Χα Τινχ"), ("en", "Hà Tĩnh"), ("es", "Hà Tĩnh"), ("fa", "استان ها تین"), ("fi", "Hà Tĩnh"), ("fr", "Province de Hà Tĩnh"), ("gu", "હા તિન\u{acd}હ"), ("hi", "ह\u{947} तिन\u{94d}ह"), ("id", "Provinsi Ha Tinh"), ("it", "provincia di Ha Tinh"), ("ja", "ハティン省"), ("kn", "ಹಾ ಟನ\u{ccd}ಹ\u{ccd}"), ("ko", "하띤 성"), ("lt", "Natino provincija"), ("lv", "Hatiņas province"), ("mr", "हा तिन\u{94d}ह"), ("ms", "Ha Tinh"), ("nb", "Ha Tinh"), ("nl", "Hà Tĩnh"), ("no", "Ha Tinh"), ("pl", "Prowincja Hà Tĩnh"), ("pt", "Ha Tinh"), ("ru", "Хатинь"), ("si", "හ\u{dcf} ට\u{dd2}න\u{dca}හ\u{dca}"), ("sr", "Ха Тин"), ("sr_Latn", "Ha Tin"), ("sv", "Ha Tinh"), ("sw", "Mkoa wa Hà Tĩnh"), ("ta", "ஹ\u{bbe} டின\u{bcd}ஹ\u{bcd}"), ("te", "హ\u{c3e} ట\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดห\u{e48}าต\u{e34}\u{e4b}ญ"), ("tr", "Hà Tĩnh"), ("uk", "Хатінь"), ("ur", "صوبہ ہاتنہ"), ("vi", "Hà Tĩnh"), ("yue", "河靜"), ("yue_Hans", "河静"), ("zh", "河靜省")]),
                        unofficial_name_list: ["Ha Tinh"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::VN,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.6102715), longitude: Some(106.3487474), max_latitude: Some(18.089871), min_latitude: Some(16.924024), max_longitude: Some(106.995214), min_longitude: Some(105.617928)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Quang Binh"), ("am", "ኳንግ ቢን ክፍላገር"), ("ar", "محافظة كوانغ بنه"), ("be", "Куангбінь"), ("bg", "Куанг Бин"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ং বিন প\u{9cd}রদেশ"), ("ca", "Província de Quảng Bình"), ("ccp", "𑄇\u{1112a}𑄠\u{11101} 𑄝\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Quang Binh"), ("cs", "Quang Binh"), ("da", "Quang Binh"), ("de", "Quảng Bình"), ("el", "Κουάνγκ Μπινχ"), ("en", "Quảng Bình"), ("es", "Quảng Bình"), ("et", "Quảng Bìnhi provints"), ("eu", "Quang Binh probintzia"), ("fa", "استان کوانگ\u{200c}بن"), ("fi", "Quảng Bình"), ("fr", "Province de Quảng Bình"), ("ga", "Quang Binh"), ("gl", "Provincia de Quảng Bình"), ("gu", "ક\u{acd}વા\u{a82}ગ બિ\u{a82}હ"), ("he", "קוואנג בין"), ("hi", "क\u{948}\u{902}ग बिन\u{94d}ह"), ("hr", "Quảng Bình"), ("hu", "Quang Binh"), ("id", "Provinsi Quang Binh"), ("it", "provincia di Quang Binh"), ("ja", "クアンビン省"), ("jv", "Provinsi Quang Binh"), ("kn", "ಕ\u{ccd}ವಾಂಗ\u{ccd} ಬ\u{cbf}ನ\u{ccd}"), ("ko", "꽝빈 성"), ("lo", "ແຂວງກວາງບ\u{eb4}ນ"), ("lt", "Kvangbinio provincija"), ("lv", "Kuanbiņa"), ("mr", "क\u{94d}वि\u{902}ग बिन"), ("ms", "Quang Binh"), ("nb", "Quang Binh"), ("nl", "Quảng Bình"), ("no", "Quang Binh"), ("pa", "ਕ\u{a42}ਏ\u{a02}ਗ ਬਿਨਾਹ ਸ\u{a42}ਬਾ"), ("pl", "Prowincja Quảng Bình"), ("pt", "Quang Binh"), ("ro", "Quảng Bình"), ("ru", "Куангбинь"), ("si", "ක\u{dca}ව\u{dcf}න\u{dca}ග\u{dca} බ\u{dd2}න\u{dca}හ\u{dca}"), ("sk", "Quảng Bình"), ("sl", "Quảng Bình"), ("so", "Quang Binh"), ("sq", "Provinca Quang Binh"), ("sr", "Куангбин"), ("sr_Latn", "Kuangbin"), ("sv", "Quang Binh"), ("sw", "Quang Binh"), ("ta", "குஅங\u{bcd} பிநஹ\u{bcd}"), ("te", "క\u{c4d}వ\u{c3e}ంగ\u{c4d} బ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกว\u{e4b}างบ\u{e34}\u{e48}ญ"), ("tr", "Quang Binh"), ("uk", "Куангбінь"), ("ur", "صوبہ کوانگ بن"), ("vi", "Quảng Bình"), ("yue", "廣平省"), ("yue_Hans", "广平省"), ("zh", "廣平省")]),
                        unofficial_name_list: ["Quang Binh"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::VN,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.7943472), longitude: Some(106.963409), max_latitude: Some(17.165551), min_latitude: Some(16.3023949), max_longitude: Some(107.3883289), min_longitude: Some(106.553429)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوانج تري"), ("be", "Куангчы"), ("bg", "Куанг Чи"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ং ত\u{9cd}রি"), ("ccp", "𑄇\u{1112a}𑄠\u{11101} 𑄑\u{11133}𑄢\u{1112d}"), ("ceb", "Tỉnh Quảng Trị"), ("cs", "Quang Tri"), ("da", "Quảng Trị"), ("de", "Quảng Trị"), ("el", "Κουάνγκ Τρι"), ("en", "Quảng Trị"), ("es", "Quảng Trị"), ("fa", "استان کوانگ تری"), ("fi", "Quảng Trị"), ("fr", "Province de Quảng Trị"), ("gu", "ક\u{acd}યા\u{a82}ગ ટ\u{acd}રી"), ("hi", "क\u{94d}वा\u{902}ग ट\u{94d}राय"), ("id", "Provinsi Quang Tri"), ("it", "provincia di Quang Tri"), ("ja", "クアンチ省"), ("kn", "ಕ\u{ccd}ವಾಂಗ\u{ccd} ಟ\u{ccd}ರುಟೊ"), ("ko", "꽝찌 성"), ("lt", "Kvangčio provincija"), ("lv", "Kuanči province"), ("mr", "क\u{94d}य\u{942}अ\u{902}ग ट\u{94d}री"), ("ms", "Quang Trị"), ("nb", "Quang Tri"), ("nl", "Quảng Trị"), ("no", "Quang Tri"), ("pl", "Prowincja Quảng Trị"), ("pt", "Quang Tri"), ("ro", "Quảng Trị"), ("ru", "Куангчи"), ("si", "ක\u{dca}ව\u{dcf}න\u{dca}ග\u{dca} ට\u{dca}\u{200d}ර\u{dd2}"), ("sl", "provinca Quảng Trị"), ("sv", "Quang Tri"), ("sw", "Mkoa wa Quảng Trị"), ("ta", "குங\u{bcd}க ட\u{bcd}ரி"), ("te", "క\u{c4d}వ\u{c3e}ంగ\u{c4d} ట\u{c4d}ర\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดกว\u{e4b}างจ\u{e34}"), ("tr", "Quảng Trị"), ("uk", "Куангчі"), ("ur", "قوانگ تری صوبہ"), ("vi", "Quảng Trị"), ("yue", "廣治"), ("yue_Hans", "广治"), ("zh", "廣治省")]),
                        unofficial_name_list: ["Quang Tri"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::VN,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.467397), longitude: Some(107.5905326), max_latitude: Some(16.741354), min_latitude: Some(15.994803), max_longitude: Some(108.1925689), min_longitude: Some(107.0167731)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ثوا ثن هوي"), ("bg", "Тхуа Тхиен-Хюе"), ("bn", "থ\u{9c1}\u{9c1}য\u{9bc}\u{9be} থিয\u{9bc}েন-হ\u{9c1}"), ("ccp", "𑄗\u{1112a}𑄠 𑄗\u{11128}𑄠𑄬𑄚\u{11134}-𑄦\u{1112a}𑄠𑄬"), ("ceb", "Tỉnh Thừa Thiên-Huế"), ("cs", "Thua Thien-Hue"), ("da", "Huế"), ("de", "Thừa Thiên-Huế"), ("el", "Θούα Θιέν-Χουέ"), ("en", "Thừa Thiên–Huế"), ("es", "Thừa Thiên-Huế"), ("fi", "Thừa Thiên-Huế"), ("fr", "Province de Thừa Thiên-Huế"), ("gu", "થ\u{ac1}આ થિએન-હ\u{ac1}એ"), ("hi", "थ\u{941}रा थिएन-ह\u{941}अए"), ("id", "Provinsi Thua Thien-Hue"), ("it", "provincia di Thua Thien-Hue"), ("kn", "ಥ\u{cbf}ಯಾ ಥ\u{cbf}ನ\u{ccd}-ಹ\u{cc2}"), ("ko", "투아티엔후에 성"), ("lt", "TchiaTjenchujaus provincija"), ("lv", "Thiathjenas-Hue province"), ("mr", "थाय थिएन-ह\u{941}ए\u{902}ग"), ("ms", "Thua Thien-Hue"), ("nb", "Thua Thien-Hue"), ("nl", "Thừa Thiên-Huế"), ("no", "Thua Thien-Hue"), ("pl", "Prowincja Thừa Thiên-Huế"), ("pt", "Thua Thien-Hue"), ("ro", "Thừa Thiên - Huế"), ("ru", "Тхыатхьен-Хюэ"), ("si", "ට\u{dca}ඨ\u{dd4}ව\u{dcf} ත\u{dd2}යන\u{dca} හ\u{dd2}ය\u{dd4}"), ("sv", "Thua Thien-Hué"), ("sw", "Mkoa wa Thừa Thiên - Huế"), ("ta", "துஆ த\u{bc0}யின\u{bcd}-ஹுய\u{bcd}"), ("te", "తుర\u{c3e} త\u{c3f}య\u{c46}న\u{c4d}-హ\u{c4d}యూ"), ("th", "จ\u{e31}งหว\u{e31}ดเถ\u{e37}\u{e48}อเท\u{e35}ยน-เว\u{e49}"), ("tr", "Thừa Thiên - Huế"), ("uk", "Тхиатхьєн-Хюе"), ("ur", "تھوا تھیئن-ہوائے صوبہ"), ("vi", "Thừa Thiên - Huế"), ("yue", "承天順化"), ("yue_Hans", "承天顺化"), ("zh", "承天順化省")]),
                        unofficial_name_list: ["Thua Thien-Hue"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::VN,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.5393538), longitude: Some(108.019102), max_latitude: Some(16.066077), min_latitude: Some(14.951885), max_longitude: Some(108.7379948), min_longitude: Some(107.217789)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوانج نام"), ("bg", "Куанг Нам"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ং ন\u{9be}ম"), ("ccp", "𑄇\u{1112a}𑄠\u{11101} 𑄚𑄟\u{11134}"), ("ceb", "Tỉnh Quảng Nam"), ("cs", "Quảng Nam"), ("da", "Quang Nam"), ("de", "Quảng Nam"), ("el", "Κουάνγκ Ναμ"), ("en", "Quảng Nam"), ("es", "Quảng Nam"), ("fa", "استان کوانگ نام"), ("fi", "Quảng Nam"), ("fr", "Province de Quảng Nam"), ("gu", "ક\u{ac1}\u{a82}ગ નામ"), ("hi", "क\u{948}\u{902}ग नाम"), ("id", "Provinsi Quang Nam"), ("it", "provincia di Quang Nam"), ("ja", "クアンナム省"), ("kn", "ಕ\u{ccd}ವಾಂಗ\u{ccd} ನಾಮ\u{ccd}"), ("ko", "꽝남 성"), ("lt", "Kvangnamas"), ("lv", "Kuannama"), ("mr", "क\u{94d}\u{902}ग नाम"), ("ms", "Quang Nam"), ("nb", "Quang Nam"), ("nl", "Quảng Nam"), ("no", "Quang Nam"), ("pl", "Prowincja Quảng Nam"), ("pt", "Quang Nam"), ("ro", "Quảng Nam"), ("ru", "Куангнам"), ("si", "ක\u{dca}වන\u{dca}ග\u{dca} නම\u{dca}"), ("sr", "Кванг Нам"), ("sr_Latn", "Kvang Nam"), ("sv", "Quang Nam"), ("sw", "Mkoa wa Quảng Nam"), ("ta", "குணங\u{bcd} நம\u{bcd}"), ("te", "క\u{c4d}వ\u{c3e}ంగ\u{c4d} న\u{c3e}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกว\u{e4b}างนาม"), ("tr", "Quang Nam"), ("uk", "Куангнам"), ("ur", "قوانگ نام صوبہ"), ("vi", "Quảng Nam"), ("yue", "廣南"), ("yue_Hans", "广南"), ("zh", "廣南省")]),
                        unofficial_name_list: ["Quang Nam"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::VN,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.3497403), longitude: Some(108.0004606), max_latitude: Some(14.4549609), min_latitude: Some(14.2307742), max_longitude: Some(108.088045), min_longitude: Some(107.8523969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كون توم"), ("bg", "Кон Тум"), ("bn", "কন\u{9cd} ত\u{9c1}ম"), ("ccp", "𑄇\u{1112e}𑄚\u{11134} 𑄑\u{1112a}𑄟\u{11134}"), ("ceb", "Kon Tum"), ("cs", "Kon Tum"), ("da", "Kon Tum"), ("de", "Kon Tum"), ("el", "Κον Τουμ"), ("en", "Kon Tum"), ("es", "Kon Tum"), ("fa", "استان کون توم"), ("fi", "Kon Tum"), ("fr", "Province de Kon Tum"), ("gu", "કોન ત\u{ac1}મ"), ("hi", "कॉन टम प\u{94d}रा\u{902}त"), ("id", "Provinsi Kon Tum"), ("it", "provincia di Kon Tum"), ("ja", "コントゥム省"), ("kn", "ಕಾನ\u{ccd} ತುಮ\u{ccd}"), ("ko", "꼰뚬 성"), ("lt", "Kon Tumo provincija"), ("lv", "Kontumas province"), ("mr", "कोण ट\u{942}म"), ("ms", "Kon Tum"), ("nb", "Kon Tum (provins)"), ("nl", "Kon Tum"), ("no", "Kon Tum (provins)"), ("pl", "Prowincja Kon Tum"), ("pt", "Kon Tum"), ("ro", "Kon Tum"), ("ru", "Контум"), ("si", "කොන\u{dca} ට\u{dd4}ම\u{dca}"), ("sr", "Контум"), ("sr_Latn", "Kontum"), ("sv", "Kon Tum"), ("sw", "Mkoa wa Kon Tum"), ("ta", "கொண\u{bcd} டும\u{bcd}"), ("te", "క\u{c4b}న\u{c4d} టుమ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกอนต\u{e39}ม"), ("tr", "Kon Tum"), ("uk", "Контум"), ("ur", "کون تم صوبہ"), ("vi", "Kon Tum"), ("yue", "崑嵩"), ("yue_Hans", "昆嵩"), ("zh", "崑嵩省")]),
                        unofficial_name_list: ["Kon Tum"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::VN,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.1213873), longitude: Some(108.8044145), max_latitude: Some(15.216273), min_latitude: Some(15.0926163), max_longitude: Some(108.9229524), min_longitude: Some(108.7603999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوانج نجاي"), ("be", "Куангнгай"), ("bg", "Куанг Нгай"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ং গ\u{9be}ই"), ("ccp", "𑄇\u{1112a}𑄠\u{11101} 𑄉\u{1112d}"), ("ceb", "Tỉnh Quảng Ngãi"), ("cs", "Quảng Ngãi"), ("da", "Quảng Ngãi"), ("de", "Quảng Ngãi"), ("el", "Κουάνγκ Νγκάι"), ("en", "Quảng Ngãi"), ("es", "Quảng Ngãi"), ("fa", "استان کوانگ نگای"), ("fi", "Quảng Ngãi"), ("fr", "Province de Quảng Ngãi"), ("gu", "ક\u{ac1}આ\u{a82}ગ નગાઈ"), ("hi", "क\u{948}\u{902}ग गाई"), ("hy", "Կուանգնգայ"), ("id", "Provinsi Quang Ngai"), ("it", "provincia di Quang Ngai"), ("ja", "クアンガイ省"), ("kn", "ಕ\u{ccd}ವಾಂಗ\u{ccd} ನ\u{ccd}ಗಾಯ\u{cbf}"), ("ko", "꽝응아이 성"), ("lt", "Kvang Ngajus"), ("lv", "Kuanngaja"), ("mr", "क\u{941}आ\u{902}ग नगाई"), ("ms", "Quang Ngai"), ("nb", "Quang Ngai"), ("nl", "Quảng Ngãi"), ("no", "Quang Ngai"), ("pl", "Prowincja Quảng Ngãi"), ("pt", "Quang Ngai"), ("ro", "Quảng Ngãi"), ("ru", "Куангнгай"), ("si", "ක\u{dca}වන\u{dca}ග\u{dca} එන\u{dca}ග\u{dcf}ය\u{dd2}"), ("sr", "Кванг Нгај"), ("sr_Latn", "Kvang Ngaj"), ("sv", "Quang Ngai"), ("sw", "Mkoa wa Quảng Ngãi"), ("te", "క\u{c4d}వ\u{c3e}ంగ\u{c4d} ఎన\u{c4d}గ\u{c3e}య\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกว\u{e4b}างหงาย"), ("tr", "Quang Ngai"), ("uk", "Куангнгай"), ("ur", "قوانگ نگائی صوبہ"), ("vi", "Quảng Ngãi"), ("yue", "廣刈省"), ("yue_Hans", "广刈省"), ("zh", "廣義省")]),
                        unofficial_name_list: ["Quang Ngai"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::VN,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.8078943), longitude: Some(108.109375), max_latitude: Some(14.602364), min_latitude: Some(12.9962269), max_longitude: Some(108.8727541), min_longitude: Some(107.3392181)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة زا لاي"), ("bg", "Жиа Лай"), ("bn", "জিয\u{9bc}\u{9be} লিয\u{9bc}\u{9be}"), ("ccp", "𑄎\u{11128}𑄠\u{1112d} 𑄣\u{1112d}"), ("ceb", "Gia Lai"), ("cs", "Gia Lai"), ("da", "Gia Lai"), ("de", "Gia Lai"), ("el", "Γκία Λέι"), ("en", "Gia Lai"), ("es", "Gia Lai"), ("fa", "استان گیا لای"), ("fi", "Gia Lai"), ("fr", "Province de Gia Lai"), ("gu", "જીઆ લાઈ"), ("hi", "जिया लाई प\u{94d}रा\u{902}त"), ("id", "Provinsi Gia Lai"), ("it", "provincia di Gia Lai"), ("ja", "ザライ省"), ("kn", "ಜ\u{cbf}ಯಾ ಲೈ"), ("ko", "잘라이 성"), ("lt", "Baklėjus"), ("lv", "Zalaja"), ("ml", "ഗിയ ല\u{d3e}യ\u{d4d} പ\u{d4d}രൊവിൻസ\u{d4d}"), ("mr", "जिया लाइ"), ("ms", "Gia Lai"), ("nb", "Gia Lai"), ("nl", "Gia Lai"), ("no", "Gia Lai"), ("pl", "Prowincja Gia Lai"), ("pt", "Gia Lai"), ("ru", "Зялай"), ("si", "ජ\u{dd2}ය\u{dcf} ල\u{dcf}ය\u{dd2}"), ("sr", "Ђа Лај"), ("sr_Latn", "Đa Laj"), ("sv", "Gia Lai"), ("sw", "Mkoa wa Gia Lai"), ("ta", "கிய\u{bbe} ல\u{bbe}ய\u{bcd}"), ("te", "గ\u{c3f}య\u{c3e} ల\u{c3e}య\u{c4d}"), ("th", "ยาลาย"), ("tr", "Gia Lai"), ("uk", "Зялай"), ("ur", "گیا لائی صوبہ"), ("vi", "Gia Lai"), ("yue", "嘉萊"), ("yue_Hans", "嘉莱"), ("zh", "嘉萊省")]),
                        unofficial_name_list: ["Gia Lai"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::VN,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.7829673), longitude: Some(109.2196634), max_latitude: Some(13.899993), min_latitude: Some(13.669688), max_longitude: Some(109.3000072), min_longitude: Some(109.1325188)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بنه دنه"), ("bg", "Бин Дин"), ("bn", "বিহ\u{9cd}ন দিহ\u{9cd}ন"), ("ccp", "𑄝\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄓\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Bình Định"), ("cs", "Binh Dinh"), ("da", "Bình Dinh"), ("de", "Bình Định"), ("el", "Μπινχ Ντινχ"), ("en", "Bình Định"), ("es", "Bình Định"), ("fa", "استان بین دین"), ("fi", "Bình Định"), ("fr", "Province de Bình Định"), ("gu", "બિ\u{a82}હ દિ\u{a82}હ"), ("hi", "बिन\u{94d}ह दिन\u{94d}ह"), ("id", "Provinsi Binh Dinh"), ("it", "provincia di Binh Dinh"), ("ja", "ビンディン省"), ("kn", "ಬೈನ\u{ccd} ಧೀನ\u{ccd}ಹ\u{ccd}"), ("ko", "빈딘 성"), ("lt", "Bindino provincija"), ("lv", "Biņdiņas province"), ("mr", "बिनह ड\u{901}न\u{94d}ह"), ("ms", "Binh Dinh"), ("nb", "Binh Dinh"), ("nl", "Bình Định"), ("no", "Binh Dinh"), ("pl", "Prowincja Bình Định"), ("pt", "Binh Dinh"), ("ru", "Биньдинь"), ("si", "බ\u{dd2}න\u{dca}හ\u{dca} ඩ\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Binh Dinh"), ("sw", "Mkoa wa Bình Định"), ("ta", "பிநஹ\u{bcd} டிநஹ\u{bcd}"), ("te", "బ\u{c3f}న\u{c4d}హ\u{c4d} డ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}\u{e48}ญด\u{e34}\u{e48}ญ"), ("tr", "Binh Dinh"), ("uk", "Біньдінь"), ("ur", "بنہ دینہ صوبہ"), ("vi", "Bình Định"), ("yue", "平定"), ("yue_Hans", "平定"), ("zh", "平定省")]),
                        unofficial_name_list: ["Binh Dinh"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::VN,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.0881861), longitude: Some(109.0928764), max_latitude: Some(13.694343), min_latitude: Some(12.705112), max_longitude: Some(109.4588245), min_longitude: Some(108.672809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فو أين"), ("bg", "Фу Йен"), ("bn", "ফ\u{9c1} ইয\u{9bc}েন"), ("ca", "Phú Yên"), ("ccp", "𑄜\u{1112a} 𑄃\u{11128}𑄠𑄬𑄚\u{11134}"), ("ceb", "Tỉnh Phú Yên"), ("cs", "Phu Yen"), ("da", "Phú Yên"), ("de", "Phú Yên"), ("el", "Φου Γιέν"), ("en", "Phú Yên"), ("es", "Phú Yên"), ("fa", "استان فو ین"), ("fi", "Phú Yên"), ("fr", "Province de Phú Yên"), ("gu", "ફ\u{ac1} ય\u{ac7}ન"), ("hi", "फ\u{941} य\u{947}न प\u{94d}रा\u{902}त"), ("id", "Provinsi Phu Yen"), ("it", "provincia di Phu Yen"), ("ja", "フーイエン省"), ("kn", "ಫು ಯ\u{cc6}ನ\u{ccd}"), ("ko", "푸옌 성"), ("lt", "Fujenas"), ("lv", "Fujena"), ("mr", "फ\u{942} य\u{947}\u{902}"), ("ms", "Phu Yen"), ("nb", "Phu Yen"), ("nl", "Phú Yên"), ("no", "Phu Yen"), ("pl", "Prowincja Phú Yên"), ("pt", "Phu Yen"), ("ro", "Phú Yên"), ("ru", "Фуйен"), ("si", "ෆ\u{dd4} යෙන\u{dca}"), ("sv", "Phu Yen"), ("sw", "Mkoa wa Phú Yên"), ("ta", "ப\u{bcd}ஹு என\u{bcd}"), ("te", "ఫ\u{c4d}యూ య\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพ\u{e39} เหย\u{e34}น"), ("tr", "Phu Yen"), ("uk", "Фуєн"), ("ur", "فو ین صوبہ"), ("vi", "Phú Yên"), ("yue", "富安"), ("yue_Hans", "富安"), ("zh", "富安省")]),
                        unofficial_name_list: ["Phu Yen"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::VN,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.7100116), longitude: Some(108.2377519), max_latitude: Some(13.4162268), min_latitude: Some(12.16056), max_longitude: Some(108.994509), min_longitude: Some(107.4892809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة داك لاك"), ("bg", "Дак Лак"), ("bn", "ড\u{9be}ক ল\u{9be}ক"), ("ccp", "𑄓𑄇\u{11134} 𑄣𑄇\u{11134}"), ("cs", "Dak Lak"), ("da", "Đắk Lắk"), ("de", "Đắk Lắk"), ("el", "Ντακ Λακ"), ("en", "Đắk Lắk"), ("es", "Đắk Lắk"), ("fa", "استان داک لاک"), ("fi", "Đắk Lắk"), ("fr", "Đắk Lắk"), ("gu", "ડાક લાક"), ("hi", "डाक लाक"), ("id", "Provinsi Dak Lak"), ("it", "provincia di Dak Lak"), ("ja", "ダクラク省"), ("kn", "ಡಕ\u{ccd} ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "닥락 성"), ("lt", "Daklakas"), ("lv", "Daklaka"), ("mr", "डाक लाक"), ("ms", "Dak Lak"), ("nb", "Dak Lak"), ("nl", "Đắk Lắk"), ("no", "Dak Lak"), ("pl", "Prowincja Đăk Lăk"), ("pt", "Dac Lac"), ("ru", "Даклак"), ("si", "ඩ\u{dcf}ක\u{dca} ල\u{dcf}ක\u{dca}"), ("sr", "Дак Лак"), ("sr_Latn", "Dak Lak"), ("sv", "Dak Lak"), ("sw", "Mkoa wa Đắk Lắk"), ("ta", "டக\u{bcd} ல\u{bbe}க\u{bcd}"), ("te", "డ\u{c3e}క\u{c3e}\u{c4d} ల\u{c3e}క\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e31}\u{e4a}กล\u{e31}ก"), ("tr", "Dak Lak"), ("uk", "Даклак"), ("ur", "داک لاک صوبہ"), ("vi", "Đắk Lắk"), ("yue", "多樂"), ("yue_Hans", "多乐"), ("zh", "多樂省")]),
                        unofficial_name_list: ["Dac Lac"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::VN,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.2585098), longitude: Some(109.0526076), max_latitude: Some(12.8655891), min_latitude: Some(11.8045669), max_longitude: Some(109.4615432), min_longitude: Some(108.671521)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كان هوا"), ("bg", "Кхан Хоа"), ("bn", "খ\u{9be}ন হোয\u{9bc}\u{9be}"), ("ca", "Khánh Hòa"), ("ccp", "𑄈𑄚\u{11134}𑄦\u{11134} 𑄦\u{1112e}𑄠"), ("ceb", "Tỉnh Khánh Hòa"), ("cs", "Khanh Hoa"), ("da", "Khánh Hòa"), ("de", "Khánh Hòa"), ("el", "Κανχ Χόα"), ("en", "Khánh Hòa"), ("es", "Khánh Hòa"), ("fa", "استان خانح هوآ"), ("fi", "Khánh Hòa"), ("fr", "Province de Khánh Hòa"), ("gu", "ખા\u{a82}હ હોઆ"), ("hi", "खा\u{902}ह हो"), ("id", "Provinsi Khanh Hoa"), ("it", "provincia di Khanh Hoa"), ("ja", "カインホア省"), ("kn", "ಖಾನ\u{ccd}ಹ\u{ccd} ಹೋ"), ("ko", "카인호아 성"), ("lt", "Kanchoa"), ("lv", "Haņhoa"), ("mr", "खानह हो"), ("ms", "Khanh Hoa"), ("nb", "Khanh Hoa"), ("nl", "Khánh Hòa"), ("no", "Khanh Hoa"), ("pl", "Prowincja Khánh Hòa"), ("pt", "Khanh Hoa"), ("ro", "provincia Khánh Hòa"), ("ru", "Кханьхоа"), ("si", "ඛ\u{dcf}න\u{dca} හොආ"), ("sr", "Кан Хоа"), ("sr_Latn", "Kan Hoa"), ("sv", "Khanh Hoa"), ("sw", "Mkoa wa Khánh Hòa"), ("ta", "க\u{bbe}ன\u{bcd}ஹ\u{bcd} ஹோஆ"), ("te", "ఖ\u{c3e}న\u{c4d} హ\u{c4b}వ\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e31}\u{e49}ญฮหว\u{e48}า"), ("tr", "Khanh Hoa"), ("uk", "Кханьхоа"), ("ur", "خانھ ہوا صوبہ"), ("vi", "Khánh Hòa"), ("yue", "慶和"), ("yue_Hans", "庆和"), ("zh", "慶和省")]),
                        unofficial_name_list: ["Khanh Hoa"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::VN,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.9404192), longitude: Some(108.4583132), max_latitude: Some(12.002635), min_latitude: Some(11.8051867), max_longitude: Some(108.5906696), min_longitude: Some(108.3107758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لام دونغ"), ("bg", "Лам Донг"), ("bn", "ল\u{9be}ম ডং"), ("ccp", "𑄣𑄟\u{11134} 𑄓\u{11127}\u{11101}"), ("ceb", "Tỉnh Lâm Đồng"), ("cs", "Lam Dong"), ("da", "Lâm Đồng"), ("de", "Lâm Đồng"), ("el", "Λαμ Ντόνγκ"), ("en", "Lâm Đồng"), ("es", "Lâm Đồng"), ("fa", "استان لام دونگ"), ("fi", "Lâm Đồng"), ("fr", "Province de Lâm Đồng"), ("gu", "લ\u{ac7}મ ડો\u{a82}ગ"), ("hi", "लाम दो\u{902}ग"), ("id", "Provinsi Lam Dong"), ("it", "provincia di Lam Dong"), ("ja", "ラムドン省"), ("kn", "ಲಮ\u{ccd} ಡಾಂಗ\u{ccd}"), ("ko", "럼동 성"), ("lt", "Lamdongo provincija"), ("lv", "Lomdonas province"), ("mr", "ल\u{945}म डा\u{901}ग"), ("ms", "Lam Dong"), ("nb", "Lam Dong"), ("nl", "Lâm Đồng"), ("no", "Lam Dong"), ("pl", "Prowincja Lâm Đồng"), ("pt", "Lam Dong"), ("ro", "Lâm Đồng"), ("ru", "Ламдонг"), ("si", "ල\u{dcf}ම\u{dca} ඩෝන\u{dca}ග\u{dca}"), ("sr", "Лам Донг"), ("sr_Latn", "Lam Dong"), ("sv", "Lam Dong"), ("sw", "Mkoa wa Lâm Đồng"), ("ta", "ல\u{bbe}ம\u{bcd} டோங\u{bcd}"), ("te", "ల\u{c3e}మ\u{c4d} డ\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเล\u{e34}มด\u{e48}ง"), ("tr", "Lâm Đồng"), ("uk", "Ламдонг"), ("ur", "لام ڈونگ صوبہ"), ("vi", "Lâm Đồng"), ("yue", "林同"), ("yue_Hans", "林同"), ("zh", "林同省")]),
                        unofficial_name_list: ["Lam Dong"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::VN,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.6738767), longitude: Some(108.8629572), max_latitude: Some(12.163288), min_latitude: Some(11.3070363), max_longitude: Some(109.2379444), min_longitude: Some(108.55301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ننه توان"), ("bg", "Нин Тхуан"), ("bn", "নিহ\u{9cd}ন থ\u{9c1}য\u{9bc}\u{9be}ন"), ("ccp", "𑄚\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄒\u{1112a}𑄠𑄚\u{11134}"), ("ceb", "Tỉnh Ninh Thuận"), ("cs", "Ninh Thuan"), ("da", "Ninh Thuận"), ("de", "Ninh Thuận"), ("el", "Νινχ Θουάν"), ("en", "Ninh Thuận"), ("es", "Ninh Thuận"), ("fa", "استان نین توان"), ("fi", "Ninh Thuận"), ("fr", "Province de Ninh Thuận"), ("gu", "નિ\u{a82}હ થ\u{ac1}આન"), ("hi", "निन\u{94d}ह थ\u{941}आन"), ("id", "Provinsi Ninh Thuan"), ("it", "provincia di Ninh Thuan"), ("ja", "ニントゥアン省"), ("kn", "ನ\u{cbf}ನ\u{ccd}ಹ\u{ccd} ಥುವಾನ\u{ccd}"), ("ko", "닌투언 성"), ("lt", "Nintchuanas"), ("lv", "Niņthuona"), ("mr", "निन\u{94d}ह थ\u{941}\u{902}न"), ("ms", "Ninh Thuan"), ("nb", "Ninh Thuận"), ("nl", "Ninh Thuận"), ("no", "Ninh Thuận"), ("pl", "Prowincja Ninh Thuận"), ("pt", "Ninh Thuan"), ("ro", "Ninh Thuận"), ("ru", "Ниньтхуан"), ("si", "න\u{dd2}න\u{dca}හ\u{dca} ත\u{dd4}ව\u{dcf}න\u{dca}"), ("sl", "provinca Ninh Thuận"), ("sv", "Ninh Thuan"), ("sw", "Mkoa wa Ninh Thuận"), ("ta", "நின\u{bcd}ஹ\u{bcd} துத\u{bbe}ன\u{bcd}"), ("te", "న\u{c3f}న\u{c4d}హ\u{c4d} తువ\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e34}ญถ\u{e48}วน"), ("tr", "Ninh Thuận"), ("uk", "Ніньтхуан"), ("ur", "ننہ تھوان صوبہ"), ("vi", "Ninh Thuận"), ("yue", "寧順"), ("yue_Hans", "宁顺"), ("zh", "寧順省")]),
                        unofficial_name_list: ["Ninh Thuan"].to_vec(),
                    }
                ),
                (
                    "37",
                    Subdivision{
                        name: "37",
                        country_alpha2: Alpha2::VN,
                        code: "37",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.3675415), longitude: Some(106.1192802), max_latitude: Some(11.4389323), min_latitude: Some(11.2912109), max_longitude: Some(106.1909722), min_longitude: Some(106.0719681)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تاي ننه"), ("bg", "Тай Нин"), ("bn", "ত\u{9be}য\u{9bc} নিহ\u{9cd}ন"), ("ccp", "𑄑𑄬 𑄚\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Tây Ninh"), ("cs", "Tay Ninh"), ("da", "Tay Ninh"), ("de", "Tây Ninh"), ("el", "Τάι Νινχ"), ("en", "Tây Ninh"), ("es", "Tây Ninh"), ("fa", "استان تای نینها"), ("fi", "Tây Ninh"), ("fr", "Province de Tây Ninh"), ("gu", "તાય નિન\u{acd}હ"), ("hi", "ताय निन\u{94d}ह"), ("id", "Provinsi Tay Ninh"), ("it", "provincia di Tay Ninh"), ("ja", "タイニン省"), ("km", "ខេត\u{17d2}តរោងដ\u{17c6}រ\u{17b8}"), ("kn", "ಟೈ ನ\u{cbf}ನ\u{ccd}ಹ\u{ccd}"), ("ko", "떠이닌 성"), ("lt", "Teinino provincija"), ("lv", "Tojniņa"), ("mr", "ताय निन\u{94d}ह"), ("ms", "Tay Ninh"), ("nb", "Tay Ninh"), ("nl", "Tây Ninh"), ("no", "Tay Ninh"), ("pl", "Prowincja Tây Ninh"), ("pt", "Tay Ninh"), ("ro", "Tây Ninh"), ("ru", "Тэйнинь"), ("si", "ටේ න\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Tay Ninh"), ("sw", "Mkoa wa Tây Ninh"), ("ta", "டேய\u{bcd} நின\u{bcd}ஹ\u{bcd}"), ("te", "ట\u{c47} న\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเต\u{e47}ยน\u{e34}ญ"), ("tr", "Tây Ninh"), ("uk", "Тейнінь"), ("ur", "تاے ننہ صوبہ"), ("vi", "Tây Ninh"), ("yue", "西寧"), ("yue_Hans", "西宁"), ("zh", "西寧省")]),
                        unofficial_name_list: ["Tay Ninh"].to_vec(),
                    }
                ),
                (
                    "39",
                    Subdivision{
                        name: "39",
                        country_alpha2: Alpha2::VN,
                        code: "39",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.0686305), longitude: Some(107.1675976), max_latitude: Some(11.5814941), min_latitude: Some(10.582153), max_longitude: Some(107.5747849), min_longitude: Some(106.7527479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دونج ناي"), ("bg", "Донг Най"), ("ccp", "𑄓\u{11127}\u{11101} 𑄚\u{1112d}"), ("ceb", "Tỉnh Đồng Nai"), ("cs", "Dong Nai"), ("de", "Đồng Nai"), ("en", "Đồng Nai"), ("es", "Đồng Nai"), ("fa", "استان دونگ نای"), ("fi", "Đồng Nai"), ("fr", "Province de Đồng Nai"), ("ga", "Dong Nai"), ("hi", "डौ\u{902}ग नाय प\u{94d}रान\u{94d}त"), ("id", "Provinsi Dong Nai"), ("it", "provincia di Dong Nai"), ("ja", "ドンナイ省"), ("ko", "동나이 성"), ("nl", "Đồng Nai"), ("pl", "Prowincja Đồng Nai"), ("pt", "Dong Nai"), ("ru", "Донгнай"), ("sr", "Донг Нај"), ("sr_Latn", "Dong Naj"), ("sv", "Dong Nai"), ("sw", "Mkoa wa Đồng Nai"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e48}งนาย"), ("uk", "Донгнай"), ("ur", "دونگ نائی صوبہ"), ("vi", "Đồng Nai"), ("yue", "同奈"), ("yue_Hans", "同奈"), ("zh", "同奈省")]),
                        unofficial_name_list: ["Dong Nai"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::VN,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.933333), longitude: Some(108.1), max_latitude: Some(11.0238032), min_latitude: Some(10.7712849), max_longitude: Some(108.3558984), min_longitude: Some(107.9904427)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بنه ثوان"), ("bg", "Бин Тхуан"), ("bn", "হ\u{9be}বিহ\u{9cd}ন থ\u{9c1}য\u{9bc}\u{9be}ন"), ("ccp", "𑄝\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄗\u{1112a}𑄠𑄚\u{11134}"), ("ceb", "Tỉnh Bình Thuận"), ("cs", "Binh Thuan"), ("da", "Bình Thuận"), ("de", "Bình Thuận"), ("el", "Μπινχ Θουάν"), ("en", "Bình Thuận"), ("es", "Bình Thuận"), ("fa", "استان بین توآن"), ("fi", "Bình Thuận"), ("fr", "Bình Thuận"), ("gu", "બિન\u{acd}હ થ\u{ac2}આન"), ("hi", "बिन\u{94d}ह थ\u{941}आन"), ("id", "Provinsi Binh Thuan"), ("it", "provincia di Binh Thuan"), ("ja", "ビントゥアン省"), ("kn", "ಬೈನ\u{ccd} ಥುನ\u{ccd}ನ\u{ccd}"), ("ko", "빈투언 성"), ("lt", "Bin Tuano provincija"), ("lv", "Biņthuanas province"), ("mr", "बिन\u{94d}स थ\u{941}\u{902}न"), ("ms", "Binh Thuan"), ("nb", "Binh Thuan"), ("nl", "Bình Thuận"), ("no", "Binh Thuan"), ("pl", "Prowincja Bình Thuận"), ("pt", "Binh Thuan"), ("ru", "Биньтхуан"), ("si", "බ\u{dd2}න\u{dca}හ\u{dca} ත\u{dd4}ව\u{dcf}න\u{dca}"), ("sv", "Binh Thuan"), ("sw", "Mkoa wa Bình Thuận"), ("ta", "பின\u{bcd}ஹ\u{bcd} த\u{bbe}ன\u{bcd}"), ("te", "బ\u{c3f}న\u{c4d}హ\u{c4d} తుయ\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}\u{e48}ญถ\u{e48}วน"), ("tr", "Binh Thuan"), ("uk", "Біньтхуан"), ("ur", "بنہ تھوان صوبہ"), ("vi", "Bình Thuận"), ("yue", "平順"), ("yue_Hans", "平顺"), ("zh", "平順省")]),
                        unofficial_name_list: ["Binh Thuan"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::VN,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.5330098), longitude: Some(106.4052541), max_latitude: Some(10.5754034), min_latitude: Some(10.4757429), max_longitude: Some(106.4600945), min_longitude: Some(106.346712)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لونغ أن"), ("bg", "Лонг Ан"), ("bn", "লং আন"), ("ccp", "𑄣\u{11127}\u{11101} 𑄃𑄚\u{11134}"), ("ceb", "Long An"), ("cs", "Long An"), ("da", "Long An"), ("de", "Long An"), ("el", "Λονγκ Άν"), ("en", "Long An"), ("es", "Long An"), ("fa", "استان لونگ آن"), ("fi", "Long An"), ("fr", "Province de Long An"), ("gu", "લો\u{a82}ગ એન"), ("hi", "लॉन\u{94d}ग एन"), ("id", "Provinsi Long An"), ("it", "provincia di Long An"), ("ja", "ロンアン省"), ("km", "ខេត\u{17d2}តក\u{17c6}ពង\u{17cb}គោ"), ("kn", "ಲಾಂಗ\u{ccd} ಆನ\u{ccd}"), ("ko", "롱안 성"), ("lt", "Long Anas"), ("lv", "Lonana"), ("mr", "लॉन\u{94d}ग एन"), ("ms", "Long An"), ("nb", "Long An"), ("nl", "Long An"), ("no", "Long An"), ("pl", "Prowincja Long An"), ("pt", "Long An"), ("ro", "Long An"), ("ru", "Лонган"), ("si", "ලෝන\u{dca}ග\u{dcf}න\u{dca}"), ("sr", "Лонг Ан"), ("sr_Latn", "Long An"), ("sv", "Long An"), ("sw", "Mkoa wa Long An"), ("ta", "ல\u{bbe}ங\u{bcd} அன\u{bcd}"), ("te", "ల\u{c3e}ంగ\u{c4d} ఆన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e47}องอาน"), ("tr", "Long An"), ("uk", "Лонган"), ("ur", "لونگ آن صوبہ"), ("vi", "Long An"), ("yue", "隆安"), ("yue_Hans", "隆安"), ("zh", "隆安省")]),
                        unofficial_name_list: ["Long An"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::VN,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.5417397), longitude: Some(107.2429976), max_latitude: Some(10.8039479), min_latitude: Some(10.3202097), max_longitude: Some(107.5830259), min_longitude: Some(106.9980384)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة با ريا فونج تاو"), ("bg", "Ба Зя-Вунг Тау"), ("bn", "ব\u{9be} রিয\u{9bc}\u{9be} ভ\u{9c1}ং ত\u{9be}ও"), ("ccp", "𑄝 𑄢\u{11128}𑄃\u{1112e}-𑄞\u{11101} 𑄑𑄅\u{1112a}"), ("ceb", "Tỉnh Bà Rịa-Vũng Tàu"), ("cs", "Ba Ria-Vung Tau"), ("da", "Bà Rịa–Vũng Tàu"), ("de", "Bà Rịa-Vũng Tàu"), ("el", "Μπα Ρία-Βούνγκ Τάου"), ("en", "Bà Rịa–Vũng Tàu"), ("es", "Bà Rịa-Vũng Tàu"), ("fa", "استان با ریا-وونگ تائو"), ("fi", "Bà Rịa-Vũng Tàu"), ("fr", "Province de Bà Rịa-Vũng Tàu"), ("gu", "બા રિયા-વા\u{a82}ગ તાઉ"), ("hi", "बा रिया-व\u{941}\u{902}ग तौ"), ("id", "Provinsi Bà Rịa–Vũng Tàu"), ("it", "provincia di Ba Ria-Vung Tau"), ("kn", "ಬಾ ರ\u{cbf}ಜಾ-ವಂಗ\u{ccd} ತ\u{ccc}"), ("ko", "바리어붕따우 성"), ("lt", "Barijos-Vungtau provincija"), ("lv", "Baria-Vungtau"), ("mr", "बा रा\u{902}ग-वा\u{902}ग ट\u{941}"), ("ms", "Ba Ria–Vung Tau"), ("nb", "Ba Ria Vung Tau"), ("nl", "Bà Rịa-Vũng Tàu"), ("no", "Ba Ria Vung Tau"), ("pl", "Prowincja Bà Rịa-Vũng Tàu"), ("pt", "Ba Ria-Vung Tau"), ("ro", "Bà Rịa - Vũng Tàu"), ("ru", "Бариа-Вунгтау"), ("si", "බ\u{dcf} ර\u{dd2}ය\u{dcf}-ව\u{dd4}න\u{dca}ග\u{dca} ට\u{dcf}උ"), ("sv", "Ba Ria-Vung Tau"), ("sw", "Mkoa wa Bà Rịa - Vũng Tàu"), ("ta", "ப\u{bbe} ரிய\u{bbe}–உங\u{bcd} ட\u{bbe}"), ("te", "బ\u{c3e} ర\u{c3f}య\u{c3e}-వుంగ\u{c4d} త\u{c3e}వ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e48}าเส\u{e35}ยะ–หว\u{e38}งเต\u{e48}า"), ("tr", "Bà Rịa-Vũng Tàu"), ("uk", "Барія-Вунгтау"), ("ur", "با ریا-وؤنگ تاو صوبہ"), ("vi", "Bà Rịa - Vũng Tàu"), ("yue", "巴地頭頓"), ("yue_Hans", "巴地头顿"), ("zh", "巴地頭頓省")]),
                        unofficial_name_list: ["Ba Ria - Vung Tau"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::VN,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.5215836), longitude: Some(105.1258955), max_latitude: Some(10.9559351), min_latitude: Some(10.1824999), max_longitude: Some(105.5719318), min_longitude: Some(104.7775209)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة آن جيانج"), ("bg", "Ан Жианг"), ("bn", "আন জ\u{9be}ং"), ("ccp", "𑄃𑄚\u{11134} 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "An Giang"), ("cs", "An Giang"), ("da", "An Giang"), ("de", "An Giang"), ("el", "Αν Γκιάνγκ"), ("en", "An Giang"), ("es", "An Giang"), ("et", "An Giang"), ("fa", "استان ان گینگ"), ("fi", "An Giang"), ("fr", "Province d’An Giang"), ("gu", "અન જિઆ\u{a82}ગ"), ("hi", "एन गिया\u{902}ग"), ("id", "Provinsi An Giang"), ("it", "provincia di An Giang"), ("ja", "アンザン省"), ("km", "ខេត\u{17d2}តមាត\u{17cb}ជ\u{17d2}រ\u{17bc}ក"), ("kn", "ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "안장 성"), ("lt", "Anziango provincija"), ("lv", "Anzana"), ("mr", "अन जिआ\u{902}ग"), ("ms", "Wilayah An Giang"), ("nb", "An Giang"), ("nl", "An Giang"), ("no", "An Giang"), ("pl", "Prowincja An Giang"), ("pt", "An Giang"), ("ro", "An Giang"), ("ru", "Анзянг"), ("si", "අන\u{dca} ග\u{dd2}ය\u{dcf}න\u{dca}ග\u{dca}"), ("sv", "An Giang"), ("sw", "Mkoa wa An Giang"), ("ta", "அன\u{bcd} ஜிய\u{bbe}ங\u{bcd}"), ("te", "ఆన\u{c4d} గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอานซาง"), ("tr", "An Giang"), ("uk", "Анзянг"), ("ur", "آن گیانگ صوبہ"), ("vi", "An Giang"), ("yue", "安江省"), ("yue_Hans", "安江省"), ("zh", "安江省")]),
                        unofficial_name_list: ["An Giang"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::VN,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.4937989), longitude: Some(105.6881788), max_latitude: Some(10.9664691), min_latitude: Some(10.1387694), max_longitude: Some(105.944197), min_longitude: Some(105.1887371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دون تاب"), ("bg", "Донг Тхап"), ("bn", "দোং থ\u{9be}প প\u{9cd}রদেশ"), ("ccp", "𑄓\u{11127}\u{11101} 𑄗𑄛\u{11134}"), ("ceb", "Tỉnh Đồng Tháp"), ("cs", "Dong Thap"), ("da", "Dong Thap Province"), ("de", "Đồng Tháp"), ("el", "Ντονγκ Θαπ"), ("en", "Đồng Tháp"), ("es", "Đồng Tháp"), ("fa", "استان دونگ تاپ"), ("fi", "Đồng Tháp"), ("fr", "Đồng Tháp"), ("gu", "ડો\u{a82}ગ થાપ પ\u{acd}રા\u{a82}ત"), ("hi", "दा\u{902}ग थाप प\u{94d}रा\u{902}त"), ("id", "Provinsi Dong Thap"), ("it", "provincia di Dong Thap"), ("ja", "ドンタップ省"), ("km", "ខេត\u{17d2}តផ\u{17d2}សារដែក"), ("kn", "ಡಾಂಗ\u{ccd} ಥ\u{ccd}ಯಾಪ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "동탑 성"), ("lt", "Dongtchapo provincija"), ("lv", "Donthapas province"), ("mr", "डो\u{902}ग थाप प\u{94d}रा\u{902}त"), ("ms", "Dong Thap Province"), ("nb", "Dong Thap provins"), ("nl", "Đồng Tháp"), ("no", "Dong Thap provins"), ("pl", "Prowincja Đồng Tháp"), ("pt", "Dong Thap"), ("ru", "Донгтхап"), ("si", "ඩොන\u{dca}ග\u{dca} ත\u{dcf}ප\u{dca} පළ\u{dcf}ත"), ("sv", "Dong Thap"), ("sw", "Mkoa wa Đồng Tháp"), ("ta", "டோங\u{bcd} தப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c3e}ంగ\u{c4d} థ\u{c3e}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e48}งท\u{e49}าป"), ("tr", "Đồng Tháp"), ("uk", "Донгтхап"), ("ur", "دونگ تھاپ صوبہ"), ("vi", "Tỉnh Đồng Tháp"), ("yue", "同塔"), ("yue_Hans", "同塔"), ("zh", "同塔省")]),
                        unofficial_name_list: ["Dong Thap"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::VN,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.4493324), longitude: Some(106.3420504), max_latitude: Some(10.5871), min_latitude: Some(10.213442), max_longitude: Some(106.788528), min_longitude: Some(105.8196079)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تين جيانج"), ("bg", "Тиен Жианг"), ("bn", "তিয\u{9bc}েন গিয\u{9bc}\u{9be}ং"), ("ccp", "𑄑\u{11128}𑄠𑄬𑄚\u{11134} 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "Tỉnh Tiền Giang"), ("cs", "Tien Giang"), ("da", "Tien Giang"), ("de", "Tiền Giang"), ("el", "Τιέν Γκιάνγκ"), ("en", "Tiền Giang"), ("es", "Tiền Giang"), ("fa", "استان تین گیانگ"), ("fi", "Tiền Giang"), ("fr", "Province de Tiền Giang"), ("gu", "ટીએન જિઆ\u{a82}ગ"), ("hi", "तिएन जीआ\u{902}ग"), ("id", "Provinsi Tien Giang"), ("it", "provincia di Tien Giang"), ("ja", "ティエンザン省"), ("kn", "ತ\u{cbf}ಯಾನ\u{ccd} ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "띠엔장 성"), ("lt", "Tien Giangas"), ("lv", "Thenzana"), ("mr", "तिएन जिआ\u{902}ग"), ("ms", "Tien Giang"), ("nb", "Tein Giang"), ("nl", "Tiền Giang"), ("no", "Tein Giang"), ("pl", "Prowincja Tiền Giang"), ("pt", "Tien Giang"), ("ro", "Tiền Giang"), ("ru", "Тьензянг"), ("si", "ට\u{dd2}යෙන\u{dca} ග\u{dd2}යන\u{dca}ග\u{dca}"), ("sv", "Tien Giang"), ("sw", "Mkoa wa Tiền Giang"), ("ta", "டைன\u{bcd} ஜிய\u{bbe}ங\u{bcd}"), ("te", "ట\u{c3f}య\u{c46}న\u{c4d} గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเต\u{e35}\u{e48}ยนซาง"), ("tr", "Tiền Giang"), ("uk", "Тьєнзянг"), ("ur", "تیئن گیانگ صوبہ"), ("vi", "Tiền Giang"), ("yue", "前江"), ("yue_Hans", "前江"), ("zh", "前江省")]),
                        unofficial_name_list: ["Tien Giang"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::VN,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.8249587), longitude: Some(105.1258955), max_latitude: Some(10.538596), min_latitude: Some(9.381122999999999), max_longitude: Some(105.538959), min_longitude: Some(104.3223179)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كن زانغ"), ("bg", "Киен Жианг"), ("bn", "কিয\u{9bc}েন গিয\u{9bc}\u{9be}ং"), ("ccp", "𑄇\u{11128}𑄠𑄬𑄚\u{11134} 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "Tỉnh Kiến Giang"), ("cs", "Kien Giang"), ("da", "Kiên Giang"), ("de", "Kiên Giang"), ("el", "Κιέν Γκιάνγκ"), ("en", "Kiên Giang"), ("es", "Kiên Giang"), ("fa", "استان کین گیانگ"), ("fi", "Kiên Giang"), ("fr", "Province de Kiên Giang"), ("gu", "કીન ગિઆ\u{a82}ગ"), ("hi", "कीन गिया\u{902}ग"), ("id", "Provinsi Kien Giang"), ("it", "provincia di Kien Giang"), ("ja", "キエンザン省"), ("km", "ខេត\u{17d2}តក\u{17d2}រម\u{17bd}នស"), ("kn", "ಕೀನ\u{ccd} ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "끼엔장 성"), ("lt", "Kjengiangas"), ("lv", "Kjenzana"), ("mr", "क\u{947}आन गिया\u{902}ग"), ("ms", "Kien Giang"), ("nb", "Kien Giang"), ("nl", "Kiên Giang"), ("no", "Kien Giang"), ("pl", "Prowincja Kiên Giang"), ("pt", "Kien Giang"), ("ru", "Кьензянг"), ("si", "ක\u{dd2}යෙන\u{dca} ජ\u{dd2}යැන\u{dca}ග\u{dca}"), ("sv", "Kien Giang"), ("sw", "Mkoa wa Kiên Giang"), ("ta", "கின\u{bcd} ஜிய\u{bbe}ங\u{bcd}"), ("te", "క\u{c3f}య\u{c46}న\u{c4d} గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเก\u{e35}ยนซาง"), ("tr", "Kien Giang"), ("uk", "Кьєнзянг"), ("ur", "کیئن گیانگ صوبہ"), ("vi", "Kiên Giang"), ("yue", "堅江"), ("yue_Hans", "坚江"), ("zh", "堅江省")]),
                        unofficial_name_list: ["Kien Giang"].to_vec(),
                    }
                ),
                (
                    "49",
                    Subdivision{
                        name: "49",
                        country_alpha2: Alpha2::VN,
                        code: "49",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.2448442), longitude: Some(105.958865), max_latitude: Some(10.2759884), min_latitude: Some(10.2191458), max_longitude: Some(105.9974669), min_longitude: Some(105.8777602)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فنه لونج"), ("bg", "Вин Лонг"), ("bn", "ভিন লং"), ("ccp", "𑄞\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄣\u{11127}\u{11101}"), ("ceb", "Tỉnh Vĩnh Long"), ("cs", "Vinh Long"), ("da", "Vĩnh Long"), ("de", "Vĩnh Long"), ("el", "Βινχ Λονγκ"), ("en", "Vĩnh Long"), ("es", "Vĩnh Long"), ("fa", "استان وین لونگ"), ("fi", "Vĩnh Long"), ("fr", "Province de Vĩnh Long"), ("gu", "વિ\u{a82}હ લો\u{a82}ગ"), ("hi", "विन\u{94d}ह लॉन\u{94d}ग"), ("id", "Provinsi Vinh Long"), ("it", "provincia di Vinh Long"), ("ja", "ヴィンロン省"), ("km", "ខេត\u{17d2}តលង\u{17cb}ហោរ"), ("kn", "ವ\u{cbf}ನ\u{ccd} ಲಾಂಗ\u{ccd}"), ("ko", "빈롱 성"), ("lt", "Vin Longas"), ("lv", "Viņlona"), ("mr", "विन\u{94d}ह लो\u{902}ग"), ("ms", "Vinh Long"), ("nb", "Ving Long"), ("nl", "Vĩnh Long"), ("no", "Ving Long"), ("pl", "Prowincja Vĩnh Long"), ("pt", "Vinh Long"), ("ro", "Vĩnh Long"), ("ru", "Виньлонг"), ("si", "ව\u{dd2}න\u{dca}හ\u{dca} ලෝන\u{dca}ග\u{dca}"), ("sr", "Вињ Лонг"), ("sr_Latn", "Vinj Long"), ("sv", "Vinh Long"), ("sw", "Mkoa wa Vĩnh Long"), ("ta", "வின\u{bcd}ஹ\u{bcd} ல\u{bbe}ங\u{bcd}"), ("te", "వ\u{c3f}న\u{c4d} ల\u{c3e}ంగ\u{c4d}"), ("th", "ว\u{e34}นฮ\u{e4c} ลอง"), ("tr", "Ving Long"), ("uk", "Віньлонг"), ("ur", "وینہ لونگ صوبہ"), ("vi", "Vĩnh Long"), ("yue", "永隆"), ("yue_Hans", "永隆"), ("zh", "永隆省")]),
                        unofficial_name_list: ["Vinh Long"].to_vec(),
                    }
                ),
                (
                    "50",
                    Subdivision{
                        name: "50",
                        country_alpha2: Alpha2::VN,
                        code: "50",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.1081553), longitude: Some(106.4405872), max_latitude: Some(10.3373171), min_latitude: Some(9.808341), max_longitude: Some(106.7976299), min_longitude: Some(106.0147733)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بن تشي"), ("bg", "Бен Че"), ("bn", "বেন ত\u{9cd}রে"), ("ccp", "𑄝𑄬𑄚\u{11134} 𑄑\u{11133}𑄢𑄬"), ("ceb", "Tỉnh Bến Tre"), ("cs", "Ben Tre"), ("da", "Bến Tre"), ("de", "Bến Tre"), ("el", "Μπεν Τρε"), ("en", "Bến Tre"), ("es", "Bến Tre"), ("fa", "استان بن تر"), ("fi", "Bến Tre (maakunta)"), ("fr", "Province de Bến Tre"), ("gu", "બ\u{ac7}ન ટ\u{acd}ર\u{ac7}"), ("hi", "ब\u{948}न ट\u{94d}र\u{947}"), ("id", "Provinsi Ben Tre"), ("it", "provincia di Ben Tre"), ("ja", "ベンチェ省"), ("kn", "ಬ\u{cc6}ನ\u{ccd} ಟ\u{ccd}ರ\u{cc6}"), ("ko", "벤째 성"), ("lt", "Benčės provincija"), ("lv", "Benčes province"), ("mr", "ब\u{947}न ट\u{94d}री"), ("ms", "Ben Tre"), ("nb", "Ben Tre"), ("nl", "Bến Tre"), ("no", "Ben Tre"), ("pl", "Prowincja Bến Tre"), ("pt", "Ben Tre"), ("ru", "Бенче"), ("si", "බේන\u{dca} ට\u{dca}\u{200d}රේ"), ("sr", "Бен Че"), ("sr_Latn", "Ben Če"), ("sv", "Ben Tre"), ("sw", "Mkoa wa Bến Tre"), ("ta", "பெண\u{bcd} ட\u{bcd}ரே"), ("te", "బ\u{c46}న\u{c4d} ట\u{c4d}ర\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดเบนแตร"), ("tr", "Ben Tre"), ("uk", "Бенче"), ("ur", "بئن تر صوبہ"), ("vi", "Bến Tre"), ("yue", "檳椥"), ("yue_Hans", "槟椥"), ("zh", "檳椥省")]),
                        unofficial_name_list: ["Ben Tre"].to_vec(),
                    }
                ),
                (
                    "51",
                    Subdivision{
                        name: "51",
                        country_alpha2: Alpha2::VN,
                        code: "51",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.933333), longitude: Some(106.35), max_latitude: Some(10.0126486), min_latitude: Some(9.8867569), max_longitude: Some(106.3883399), min_longitude: Some(106.3002563)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ترا فنه"), ("be", "Чавінь"), ("bg", "Ча Вин"), ("bn", "ত\u{9cd}র\u{9be} ভিহ\u{9cd}ন"), ("ccp", "𑄑\u{11133}𑄢 𑄞\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Trà Vinh"), ("cs", "Tra Vinh"), ("da", "Tra Vinh"), ("de", "Trà Vinh"), ("el", "Τρα Βινχ"), ("en", "Trà Vinh"), ("es", "Trà Vinh"), ("fa", "استان ترا وین"), ("fi", "Trà Vinh"), ("fr", "Province de Trà Vinh"), ("gu", "ટ\u{acd}રા વિન\u{acd}હ"), ("hi", "ट\u{94d}रा विन प\u{94d}रा\u{902}त"), ("id", "Provinsi Tra Vinh"), ("it", "provincia di Tra Vinh"), ("ja", "チャーヴィン省"), ("km", "ខេត\u{17d2}តព\u{17d2}រះត\u{17d2}រពា\u{17c6}ង"), ("kn", "ತ\u{ccd}ರ\u{cbf}ಕ ವ\u{cbf}ನ\u{ccd}"), ("ko", "짜빈 성"), ("lt", "Čavinas"), ("lv", "Čaviņas province"), ("mr", "ट\u{94d}रा विन\u{94d}ह"), ("ms", "Tra Vinh"), ("nb", "Tra Vinh"), ("nl", "Trà Vinh"), ("no", "Tra Vinh"), ("pl", "Prowincja Trà Vinh"), ("pt", "Tra Vinh"), ("ro", "Trà Vinh"), ("ru", "Чавинь"), ("si", "ට\u{dca}\u{200d}ර\u{dcf} ව\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Tra Vinh"), ("sw", "Mkoa wa Trà Vinh"), ("ta", "ட\u{bcd}ர\u{bbe} விந\u{bcd}த\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3e} వ\u{c3f}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดจ\u{e48}าว\u{e34}ญ"), ("tr", "Tra Vinh"), ("uk", "Чавінь"), ("ur", "ترا وینہ صوبہ"), ("vi", "Trà Vinh"), ("yue", "茶榮"), ("yue_Hans", "茶荣"), ("zh", "茶榮省")]),
                        unofficial_name_list: ["Tra Vinh"].to_vec(),
                    }
                ),
                (
                    "52",
                    Subdivision{
                        name: "52",
                        country_alpha2: Alpha2::VN,
                        code: "52",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.6003688), longitude: Some(105.9599539), max_latitude: Some(9.9332116), min_latitude: Some(9.2386673), max_longitude: Some(106.293053), min_longitude: Some(105.5439898)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سك ترانج"), ("bg", "Сок Чанг"), ("bn", "সখ ত\u{9be}ং"), ("ccp", "𑄥\u{1112e}𑄇\u{11134} 𑄑\u{11133}𑄢\u{11101}"), ("ceb", "Tỉnh Sóc Trăng"), ("cs", "Soc Trang"), ("da", "Sóc Trăng"), ("de", "Sóc Trăng"), ("el", "Σοκ Τρανγκ"), ("en", "Sóc Trăng"), ("es", "Sóc Trăng"), ("fa", "استان سوک ترانگ"), ("fi", "Sóc Trăng"), ("fr", "Province de Sóc Trăng"), ("gu", "સોક ટ\u{acd}રા\u{a82}ગ"), ("hi", "सोक ट\u{94d}रा\u{902}ग"), ("id", "Provinsi Soc Trang"), ("it", "provincia di Soc Trang"), ("ja", "ソクチャン省"), ("km", "ខេត\u{17d2}តឃ\u{17d2}លា\u{17c6}ង"), ("kn", "ಸೊಕ\u{ccd} ಟ\u{ccd}ರಾಂಗ\u{ccd}"), ("ko", "속짱 성"), ("lt", "Sok Čiangas"), ("lv", "Šokčana"), ("mr", "सोक ट\u{94d}र\u{901}ग"), ("ms", "Soc Trang"), ("nb", "Soc Trang"), ("nl", "Sóc Trăng"), ("no", "Soc Trang"), ("pl", "Prowincja Sóc Trăng"), ("pt", "Soc Trang"), ("ro", "Sóc Trăng"), ("ru", "Шокчанг"), ("si", "සොක\u{dca} ට\u{dca}ර\u{dcf}න\u{dca}ග\u{dca}"), ("sr", "Сок Транг"), ("sr_Latn", "Sok Trang"), ("sv", "Soc Trang"), ("sw", "Mkoa wa Sóc Trăng"), ("ta", "சொக\u{bcd} ட\u{bcd}ரங\u{bcd}"), ("te", "ఎస\u{c4d}ఓస\u{c3f} ట\u{c4d}ర\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e49}อกจ\u{e31}ง"), ("tr", "Soc Trang"), ("uk", "Шокчанг"), ("ur", "سوک ترانگ صوبہ"), ("vi", "Sóc Trăng"), ("yue", "朔莊"), ("yue_Hans", "朔庄"), ("zh", "朔莊省")]),
                        unofficial_name_list: ["Soc Trang"].to_vec(),
                    }
                ),
                (
                    "53",
                    Subdivision{
                        name: "53",
                        country_alpha2: Alpha2::VN,
                        code: "53",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.1329032), longitude: Some(105.8407722), max_latitude: Some(22.2115634), min_latitude: Some(22.051914), max_longitude: Some(105.9308625), min_longitude: Some(105.7767105)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باك كان"), ("bg", "Бак Кан"), ("ccp", "𑄝𑄇\u{11134} 𑄇𑄚\u{11134}"), ("ceb", "Tỉnh Bắc Kạn"), ("cs", "Bac Kan"), ("de", "Bắc Kạn"), ("en", "Bắc Kạn"), ("es", "Bắc Kạn"), ("fa", "استان باک کان"), ("fi", "Bắc Kạn"), ("fr", "Province de Bắc Kạn"), ("hi", "बाक क\u{948}न प\u{94d}रान\u{94d}त"), ("id", "Provinsi Bac Kan"), ("it", "provincia di Bac Kan"), ("ja", "バックカン省"), ("ko", "박깐 성"), ("ml", "ബ\u{d3e}ക\u{d4d} ക\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mn", "Бак Гайн"), ("nl", "Bắc Kạn"), ("pl", "Prowincja Bắc Kạn"), ("pt", "Bac Kan"), ("ro", "Bắc Kạn"), ("ru", "Баккан"), ("sv", "Bac Kan"), ("sw", "Mkoa wa Bắc Kạn"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}\u{e4a}กก\u{e31}\u{e48}น"), ("uk", "Баккан"), ("ur", "باک کان صوبہ"), ("vi", "Bắc Kạn"), ("yue", "北𣴓"), ("yue_Hans", "北𣴓"), ("zh", "北𣴓省")]),
                        unofficial_name_list: ["Bac Can"].to_vec(),
                    }
                ),
                (
                    "54",
                    Subdivision{
                        name: "54",
                        country_alpha2: Alpha2::VN,
                        code: "54",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.3014947), longitude: Some(106.6291304), max_latitude: Some(21.6256549), min_latitude: Some(21.121762), max_longitude: Some(107.033035), min_longitude: Some(105.881726)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة باك زانغ"), ("bg", "Бак Жианг"), ("bn", "ব\u{9be}ক গিয\u{9bc}\u{9be}ং"), ("ca", "Bắc Giang"), ("ccp", "𑄝𑄇\u{11134} 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "Tỉnh Bắc Giang"), ("cs", "Bac Giang"), ("da", "Bắc Giang"), ("de", "Bắc Giang"), ("el", "Μπακ Γκιάνγκ"), ("en", "Bắc Giang"), ("es", "Bắc Giang"), ("fa", "استان باک گیانگ"), ("fi", "Bắc Giang"), ("fr", "Province de Bắc Giang"), ("gu", "બ\u{ac7}ક જિઆ\u{a82}ગ"), ("hi", "बीक गिया\u{902}ग"), ("id", "Provinsi Bac Giang"), ("it", "provincia di Bac Giang"), ("ja", "バクザン省"), ("kn", "ಬ\u{cbf}ಗ\u{ccd} ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "박장 성"), ("lt", "Bakdžiango provincija"), ("lv", "Bakzanas province"), ("mn", "Бак Жяан"), ("mr", "ब\u{945}क जिआ\u{902}ग"), ("ms", "Bac Giang"), ("nb", "Bac Giang"), ("nl", "Bắc Giang"), ("no", "Bac Giang"), ("pl", "Prowincja Bắc Giang"), ("pt", "Bac Giang"), ("ru", "Бакзянг"), ("si", "බ\u{dcf}ක\u{dca} ග\u{dd2}යන\u{dca}ග\u{dca}"), ("sv", "Bac Giang"), ("sw", "Mkoa wa Bắc Giang"), ("ta", "ப\u{bbe}க\u{bcd} ஜிய\u{bbe}ங\u{bcd}"), ("te", "బ\u{c3e}క\u{c4d} గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}\u{e4a}กซาง"), ("tr", "Bắc Giang"), ("uk", "Бакзянг"), ("ur", "باک گیانگ صوبہ"), ("vi", "Bắc Giang"), ("yue", "北江"), ("yue_Hans", "北江"), ("zh", "北江省")]),
                        unofficial_name_list: ["Bac Giang"].to_vec(),
                    }
                ),
                (
                    "55",
                    Subdivision{
                        name: "55",
                        country_alpha2: Alpha2::VN,
                        code: "55",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.251555500000002), longitude: Some(105.5136472), max_latitude: Some(9.637118899999999), min_latitude: Some(8.595139200000002), max_longitude: Some(106.7433943), min_longitude: Some(105.2332641)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باك ليو"), ("bg", "Бак Лиеу"), ("bn", "ব\u{9be}ক লিও"), ("ccp", "𑄝𑄇\u{11134} 𑄣\u{11128}𑄅\u{1112a}"), ("ceb", "Tỉnh Bạc Liêu"), ("cs", "Bac Lieu"), ("da", "Bạc Liêu"), ("de", "Bạc Liêu"), ("el", "Μπακ Λιέου"), ("en", "Bạc Liêu"), ("es", "Bạc Liêu"), ("fa", "استان باک لیئو"), ("fi", "Bạc Liêu"), ("fr", "Province de Bạc Liêu"), ("gu", "બાક લીઉ"), ("hi", "ब\u{948}क लिऊ"), ("id", "Provinsi Bac Lieu"), ("it", "provincia di Bac Lieu"), ("ja", "バクリエウ省"), ("km", "ខេត\u{17d2}តពលលាវ"), ("kn", "ಬಾಕ\u{ccd} ಲ\u{cbf}ಯು"), ("ko", "박리에우 성"), ("lt", "Bakleu"), ("lv", "Bakljeu"), ("mr", "बा\u{902}ग लीए"), ("ms", "Bac Lieu"), ("nb", "Bac Lieu"), ("nl", "Bạc Liêu"), ("no", "Bac Lieu"), ("pl", "Prowincja Bạc Liêu"), ("pt", "Bac Lieu"), ("ru", "Бакльеу"), ("si", "බක\u{dca} ල\u{dd2}ය\u{dd4}"), ("sv", "Bac Lieu"), ("sw", "Mkoa wa Bạc Liêu"), ("ta", "ப\u{bbe}க\u{bcd} ல\u{bc0}ஐ"), ("te", "బ\u{c3e}క\u{c4d} లూ"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}กเล\u{e35}ยว"), ("tr", "Bac Lieu"), ("uk", "Бакльєу"), ("ur", "باک لیئو صوبہ"), ("vi", "Bạc Liêu"), ("yue", "薄遼"), ("yue_Hans", "薄辽"), ("zh", "薄遼省")]),
                        unofficial_name_list: ["Bac Lieu"].to_vec(),
                    }
                ),
                (
                    "56",
                    Subdivision{
                        name: "56",
                        country_alpha2: Alpha2::VN,
                        code: "56",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.121444), longitude: Some(106.1110501), max_latitude: Some(21.263603), min_latitude: Some(20.969552), max_longitude: Some(106.3089509), min_longitude: Some(105.9041419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باك ننه"), ("bg", "Бак Нин"), ("bn", "ব\u{9cd}য\u{9be}ক নিহ\u{9cd}ন"), ("ca", "Bac Ninh"), ("ccp", "𑄝𑄇\u{11134} 𑄚\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Bắc Ninh"), ("cs", "Bac Ninh"), ("da", "Bắc Ninh"), ("de", "Bắc Ninh"), ("el", "Μπακ Νινχ"), ("en", "Bắc Ninh"), ("es", "Bắc Ninh"), ("fa", "استان باک نین"), ("fi", "Bắc Ninh"), ("fr", "Province de Bắc Ninh"), ("gu", "બ\u{ac7}ક નિ\u{a82}હ"), ("hi", "बक निन\u{94d}ह"), ("id", "Provinsi Bac Ninh"), ("it", "provincia di Bac Ninh"), ("ja", "バクニン省"), ("kn", "ಬ\u{cbf}ಕ\u{ccd} ನ\u{cbf}ನ\u{ccd}ಹ\u{ccd}"), ("ko", "박닌 성"), ("lt", "Baknino provincija"), ("lv", "Bakniņas province"), ("mn", "Бак Нэн"), ("mr", "ब\u{945}क निन\u{94d}ह"), ("ms", "Wilayah Bac Ninh"), ("nb", "Bac Ninh"), ("nl", "Bắc Ninh"), ("no", "Bac Ninh"), ("pl", "Prowincja Bắc Ninh"), ("pt", "Bac Ninh"), ("ru", "Бакнинь"), ("si", "බැක\u{dca} න\u{dd2}න\u{dca}හ\u{dca}"), ("sr", "Бак Нин"), ("sr_Latn", "Bak Nin"), ("sv", "Bac Ninh"), ("sw", "Mkoa wa Bắc Ninh"), ("ta", "ப\u{bbe}க\u{bcd} நின\u{bcd}ஹ\u{bcd}"), ("te", "బ\u{c3e}క\u{c4d} న\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}\u{e4a}กน\u{e34}ญ"), ("tr", "Bắc Ninh"), ("uk", "Бакнінь"), ("ur", "باک ننہ صوبہ"), ("vi", "Bắc Ninh"), ("yue", "北寧"), ("yue_Hans", "北宁"), ("zh", "北寧省")]),
                        unofficial_name_list: ["Bac Ninh"].to_vec(),
                    }
                ),
                (
                    "57",
                    Subdivision{
                        name: "57",
                        country_alpha2: Alpha2::VN,
                        code: "57",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.3254024), longitude: Some(106.477017), max_latitude: Some(11.5000023), min_latitude: Some(10.8636351), max_longitude: Some(106.9676759), min_longitude: Some(106.335735)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بنه ديونج"), ("bg", "Бин Дуонг"), ("bn", "বিহ\u{9cd}ন ড\u{9c1}ওং"), ("ccp", "𑄝\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄓\u{1112f}𑄃\u{11127}\u{11101}"), ("ceb", "Tỉnh Bình Dương"), ("cs", "Binh Duong"), ("da", "Bình Dương"), ("de", "Bình Dương"), ("el", "Μπινχ Ντούονγκ"), ("en", "Bình Dương"), ("es", "Bình Dương"), ("fa", "استان بین دونگ"), ("fi", "Bình Dương"), ("fr", "Bình Dương"), ("gu", "બિન\u{acd}હ ડ\u{acd}ય\u{ac1}રો\u{a82}ગ"), ("hi", "बिन\u{94d}ह ड\u{94d}य\u{942}ओ\u{902}ग"), ("id", "Provinsi Bình Dương"), ("it", "provincia di Binh Duong"), ("ja", "ビンズオン省"), ("kn", "ಬ\u{cbf}ನ\u{ccd}ಹ\u{ccd} ಡಂಗ\u{ccd}ಂಗ\u{ccd}"), ("ko", "빈즈엉 성"), ("lt", "Bin Dongo provincija"), ("lv", "Biņziona"), ("mr", "बिन\u{94d}ह द\u{941}ओ\u{902}ग"), ("ms", "Binh Duong"), ("nb", "Binh Duong"), ("nl", "Bình Dương"), ("no", "Binh Duong"), ("pl", "Prowincja Bình Dương"), ("pt", "Binh Duong"), ("ru", "Биньзыонг"), ("si", "බ\u{dd2}න\u{dca}හ\u{dca} ඩ\u{dd4} ඔන\u{dca}ග\u{dca}"), ("sv", "Binh Duong"), ("sw", "Mkoa wa Bình Dương"), ("ta", "பின\u{bcd}ஹ\u{bcd} டுயோங\u{bcd}"), ("te", "బ\u{c3f}న\u{c4d}హ\u{c4d} డుర\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}\u{e48}ญเซ\u{e37}อง"), ("tr", "Binh Duonb"), ("uk", "Біньзионг"), ("ur", "بنہ دیونگ صوبہ"), ("vi", "Bình Dương"), ("yue", "平陽省"), ("yue_Hans", "平阳省"), ("zh", "平陽省")]),
                        unofficial_name_list: ["Binh Duong"].to_vec(),
                    }
                ),
                (
                    "58",
                    Subdivision{
                        name: "58",
                        country_alpha2: Alpha2::VN,
                        code: "58",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.7511894), longitude: Some(106.7234639), max_latitude: Some(12.29071), min_latitude: Some(11.300705), max_longitude: Some(107.4282521), min_longitude: Some(106.417961)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بنه فوك"), ("bg", "Бин Фуок"), ("bn", "বিন পো"), ("ccp", "𑄝\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄜\u{1112a}𑄠\u{1112e}𑄇\u{11134}"), ("ceb", "Tỉnh Bình Phước"), ("cs", "Binh Phuoc"), ("da", "Bình Phuoc"), ("de", "Bình Phước"), ("el", "Μπινχ Φουόκ"), ("en", "Bình Phước"), ("es", "Bình Phước"), ("fa", "استان بین فوک"), ("fi", "Bình Phước"), ("fr", "Bình Phước"), ("gu", "બિ\u{a82}હ ફ\u{ac2}ઓક"), ("hi", "बिन\u{94d}ह फ\u{941}‘ओक प\u{94d}रा\u{902}त"), ("id", "Provinsi Binh Phuoc"), ("it", "provincia di Binh Phuoc"), ("ja", "ビンフオック省"), ("kn", "ಬ\u{cbf}ನ\u{ccd}ಹ\u{ccd} ಫುಂಗ\u{ccd}"), ("ko", "빈프억 성"), ("lt", "Binfijokas"), ("lv", "Biņfioka"), ("mr", "बिन\u{94d}ह फ\u{941}ऊक"), ("ms", "Binh Phuoc"), ("nb", "Binh Phuroc"), ("nl", "Bình Phước"), ("no", "Binh Phuroc"), ("pl", "Prowincja Bình Phước"), ("pt", "Binh Phuoc"), ("ru", "Биньфыок"), ("si", "බ\u{dd2}න\u{dca}හ\u{dca} ප\u{dd4}ඔක\u{dca}"), ("sv", "Binh Phuoc"), ("sw", "Mkoa wa Bình Phước"), ("ta", "பிநஹ\u{bcd} ப\u{bcd}ஹுஅ"), ("te", "బ\u{c3f}న\u{c4d}హ\u{c4d} ఫ\u{c4d}యూర\u{c4b}క\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}\u{e48}ญเฟ\u{e37}\u{e49}อก"), ("tr", "Bình Phước"), ("uk", "Біньфиок"), ("ur", "بنہ فووک صوبہ"), ("vi", "Bình Phước"), ("yue", "平福"), ("yue_Hans", "平福"), ("zh", "平福省")]),
                        unofficial_name_list: ["Binh Phuoc"].to_vec(),
                    }
                ),
                (
                    "59",
                    Subdivision{
                        name: "59",
                        country_alpha2: Alpha2::VN,
                        code: "59",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.962409899999999), longitude: Some(105.1258955), max_latitude: Some(9.55968), min_latitude: Some(8.4127295), max_longitude: Some(105.4185013), min_longitude: Some(104.5229252)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كا ماو"), ("bg", "Ка Мау"), ("bn", "ক\u{9cd}য\u{9be} ম\u{9be}উ"), ("ccp", "𑄇 𑄟𑄅\u{1112a}"), ("ceb", "Tỉnh Cà Mau"), ("cs", "Cà Mau"), ("da", "Ca Mau"), ("de", "Cà Mau"), ("el", "Κα Μάου"), ("en", "Cà Mau"), ("es", "Cà Mau"), ("fa", "ایالت کا مائو"), ("fi", "Cà Mau"), ("fr", "Province de Cà Mau"), ("gu", "કા માઉ"), ("hi", "का माउ प\u{94d}रा\u{902}त"), ("id", "Provinsi Ca Mau"), ("it", "provincia di Ca Mau"), ("ja", "カマウ省"), ("km", "ខេត\u{17d2}តទ\u{17b9}កខ\u{17d2}មៅ"), ("kn", "ಕಾ ಮ\u{ccc}"), ("ko", "까마우 성"), ("lt", "Kamau"), ("lv", "Kamau"), ("mr", "सीए मऊ"), ("ms", "Wilayah Ca Mau"), ("nb", "Ca Mau"), ("nl", "Cà Mau"), ("no", "Ca Mau"), ("pl", "Prowincja Cà Mau"), ("pt", "Cà Mau"), ("ru", "Камау"), ("si", "ක\u{dcf} මෞ"), ("sr", "Ка Мау"), ("sr_Latn", "Ka Mau"), ("sv", "Ca Mau"), ("sw", "Mkoa wa Cà Mau"), ("ta", "க\u{bbe} ம\u{bbe}வ\u{bcd}"), ("te", "స\u{c3f}ఏ మ\u{c3e}వ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e48}าเมา"), ("tr", "Ca Mau"), ("uk", "Камау"), ("ur", "کآ ماو صوبہ"), ("vi", "Cà Mau"), ("yue", "金甌"), ("yue_Hans", "金瓯"), ("zh", "金甌省")]),
                        unofficial_name_list: ["Ca Mau"].to_vec(),
                    }
                ),
                (
                    "61",
                    Subdivision{
                        name: "61",
                        country_alpha2: Alpha2::VN,
                        code: "61",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.9385958), longitude: Some(106.3206861), max_latitude: Some(21.231167), min_latitude: Some(20.691178), max_longitude: Some(106.6127538), min_longitude: Some(106.126308)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هاي دونج"), ("bg", "Хай Дуонг"), ("bn", "হ\u{9be}ই দ\u{9c1}ওং"), ("ccp", "𑄦\u{1112d} 𑄓\u{1112f}𑄃\u{11127}\u{11101}"), ("ceb", "Tỉnh Hải Dương"), ("da", "Hải Dương"), ("de", "Hải Dương"), ("el", "Χάι Ντουόνγκ"), ("en", "Hải Dương"), ("es", "Hải Dương"), ("fa", "استان های دونگ"), ("fi", "Hải Dương"), ("fr", "Province de Hải Dương"), ("gu", "હાઈ ડ\u{acd}ય\u{ac1}ઓ\u{a82}ગ"), ("hi", "हाई द\u{941}रो\u{902}ग"), ("id", "Provinsi Hai Duong"), ("it", "provincia di Hai Duong"), ("ja", "ハイズオン省"), ("kn", "ಹಾಯ\u{ccd} ಡ\u{ccc}ಂಗ\u{ccd}"), ("ko", "하이즈엉 성"), ("lt", "Chaisiongas"), ("lv", "Hajziona"), ("mn", "Хайя Зыон"), ("mr", "हाई ड\u{94d}य\u{942}यॉन\u{94d}ग"), ("ms", "Hai Duong"), ("nb", "Hai Doung"), ("nl", "Hải Dương"), ("no", "Hai Doung"), ("pl", "Prowincja Hải Dương"), ("pt", "Hai Duong"), ("ru", "Хайзыонг"), ("si", "හ\u{dcf}ය\u{dd2} ඩ\u{dd4}ඔන\u{dca}ග\u{dca}"), ("sv", "Hai Duong"), ("sw", "Mkoa wa Hải Dương"), ("ta", "ஹை டுவ\u{bcd}ங\u{bcd}"), ("te", "హ\u{c3e}య\u{c3f} డుర\u{c4b}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดหายเซ\u{e37}อง"), ("tr", "Hai Duong"), ("uk", "Хайзионг"), ("ur", "ہائی دیونگ صوبہ"), ("vi", "Hải Dương"), ("yue", "海陽"), ("yue_Hans", "海阳"), ("zh", "海陽省")]),
                        unofficial_name_list: ["Hai Duong"].to_vec(),
                    }
                ),
                (
                    "63",
                    Subdivision{
                        name: "63",
                        country_alpha2: Alpha2::VN,
                        code: "63",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.5835196), longitude: Some(105.92299), max_latitude: Some(20.703745), min_latitude: Some(20.3626781), max_longitude: Some(106.183102), min_longitude: Some(105.7697231)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ها نام"), ("bg", "Хай Дуонг²"), ("bn", "হ\u{9cd}য ন\u{9be}ম"), ("ccp", "𑄦 𑄚𑄟\u{11134}"), ("ceb", "Tỉnh Hà Nam"), ("cs", "Ha Nam"), ("da", "Hà Nam"), ("de", "Hà Nam"), ("el", "Χα Ναμ"), ("en", "Hà Nam"), ("es", "Hà Nam"), ("fa", "استان ها نام"), ("fi", "Hà Nam"), ("fr", "Hà Nam"), ("gu", "હા નામ"), ("he", "אה נם"), ("hi", "हा नाम"), ("id", "Provinsi Ha Nam"), ("it", "provincia di Ha Nam"), ("ja", "ハナム省"), ("kn", "ಹ\u{ccd}ಯಾ ನ\u{ccd}ಯಾಮ\u{ccd}"), ("ko", "하남 성"), ("lt", "Chanamo provincija"), ("lv", "Hanamas province"), ("mn", "Хай Наам"), ("mr", "हा नाम"), ("ms", "Ha Nam"), ("nb", "Ha Nam"), ("nl", "Hà Nam"), ("no", "Ha Nam"), ("pl", "Prowincja Hà Nam"), ("pt", "Hà Nam"), ("ru", "Ханам"), ("si", "හ\u{dcf} නම\u{dca}"), ("sv", "Ha Nam"), ("sw", "Mkoa wa Hà Nam"), ("ta", "ஹ\u{bbe} நம\u{bcd}"), ("te", "హ\u{c3e} న\u{c3e}మ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดห\u{e48}านาม"), ("tr", "Ha Nam"), ("uk", "Ханам"), ("ur", "صوبہ ہانام"), ("vi", "Hà Nam"), ("yue", "河南"), ("yue_Hans", "河南"), ("zh", "河南省")]),
                        unofficial_name_list: ["Ha Nam"].to_vec(),
                    }
                ),
                (
                    "66",
                    Subdivision{
                        name: "66",
                        country_alpha2: Alpha2::VN,
                        code: "66",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.8525711), longitude: Some(106.0169971), max_latitude: Some(21.006161), min_latitude: Some(20.602892), max_longitude: Some(106.269346), min_longitude: Some(105.8954829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هونج ين"), ("bg", "Хунг Йен"), ("ca", "Província de Hưng Yên"), ("ccp", "𑄦\u{1112a}𑄠\u{11101} 𑄃\u{11128}𑄠𑄬𑄚\u{11134}"), ("ceb", "Tỉnh Hưng Yên"), ("cs", "Hung Yen"), ("de", "Hưng Yên"), ("en", "Hưng Yên"), ("es", "Hưng Yên"), ("fa", "استان هونگ ین"), ("fi", "Hưng Yên"), ("fr", "Province de Hưng Yên"), ("hi", "ह\u{941}\u{902}ग य\u{947}न प\u{94d}रान\u{94d}त"), ("id", "Provinsi Hung Yen"), ("it", "provincia di Hung Yen"), ("ja", "フンイエン省"), ("ko", "흥옌 성"), ("mn", "Хыниэнь"), ("nl", "Hưng Yên"), ("pl", "Prowincja Hưng Yên"), ("pt", "Hung Yen"), ("ru", "Хынгйен"), ("sv", "Hung Yen"), ("sw", "Mkoa wa Hưng Yên"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e36}งเอ\u{e35}ยน"), ("uk", "Хингʼєн"), ("ur", "ہونگ ین صوبہ"), ("vi", "Hưng Yên"), ("yue", "興安"), ("yue_Hans", "兴安"), ("zh", "興安省")]),
                        unofficial_name_list: ["Hung Yen"].to_vec(),
                    }
                ),
                (
                    "67",
                    Subdivision{
                        name: "67",
                        country_alpha2: Alpha2::VN,
                        code: "67",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.2791804), longitude: Some(106.2051484), max_latitude: Some(20.4996661), min_latitude: Some(19.968296), max_longitude: Some(106.5649637), min_longitude: Some(105.924061)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة نام دنه"), ("bg", "Нам Дин"), ("bn", "ন\u{9be}ম দিহ\u{9cd}ন"), ("ccp", "𑄚𑄟\u{11134} 𑄓\u{11128}𑄚\u{11134}𑄦\u{11134}"), ("ceb", "Tỉnh Nam Định"), ("cs", "Nam Định"), ("da", "Nam Dinh"), ("de", "Nam Định"), ("el", "Ναμ Ντίνχ"), ("en", "Nam Định"), ("es", "Nam Định"), ("fa", "استان نام دین"), ("fi", "Nam Định"), ("fr", "Province de Nam Định"), ("gu", "નામ દિ\u{a82}હ"), ("hi", "नाम दिन\u{94d}ह"), ("id", "Provinsi Nam Định"), ("it", "provincia di Nam Dinh"), ("ja", "ナムディン省"), ("kn", "ನಾಮ\u{ccd} ಧೀನ\u{ccd}ಹ\u{ccd}"), ("ko", "남딘 성"), ("lt", "Namdinas"), ("lv", "Namdiņa"), ("mn", "Наам Дэн"), ("mr", "नाम ड\u{901}न\u{94d}ह"), ("ms", "Nam Dinh"), ("nb", "Nam Dinh"), ("nl", "Nam Định"), ("no", "Nam Dinh"), ("pl", "Prowincja Nam Định"), ("pt", "Nam Dinh"), ("ro", "Nam Định"), ("ru", "Намдинь"), ("si", "නම\u{dca} ද\u{dd2}න\u{dca}හ\u{dca}"), ("sv", "Nam Dinh"), ("sw", "Mkoa wa Nam Định"), ("ta", "நம\u{bcd} டின\u{bcd}ஹ\u{bcd}"), ("te", "న\u{c3e}మ డ\u{c3f}న\u{c4d}హ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนามด\u{e34}\u{e48}ญ"), ("tr", "Nam Dinh"), ("uk", "Намдінь"), ("ur", "نام دینہ صوبہ"), ("vi", "Nam Định"), ("yue", "南定"), ("yue_Hans", "南定"), ("zh", "南定省")]),
                        unofficial_name_list: ["Nam Dinh"].to_vec(),
                    }
                ),
                (
                    "68",
                    Subdivision{
                        name: "68",
                        country_alpha2: Alpha2::VN,
                        code: "68",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.268443), longitude: Some(105.2045573), max_latitude: Some(21.719738), min_latitude: Some(20.917282), max_longitude: Some(105.4579171), min_longitude: Some(104.8163571)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فو تاه"), ("bg", "Фу Тхо"), ("bn", "ফ\u{9c1} থ"), ("ccp", "𑄜\u{1112a} 𑄗\u{1112e}"), ("cs", "Phu Tho"), ("da", "Phu Tho"), ("de", "Phú Thọ"), ("el", "Φου Θο"), ("en", "Phú Thọ"), ("es", "Phú Thọ"), ("fa", "استان فو تو"), ("fi", "Phú Thọ"), ("fr", "Province de Phú Thọ"), ("gu", "ફ\u{ac1} થા"), ("hi", "फ\u{942} ठो"), ("id", "Provinsi Phu Tho"), ("it", "provincia di Phu Tho"), ("ja", "フート省"), ("kn", "ಫು ಥೋ"), ("ko", "푸토 성"), ("lt", "Futo"), ("lv", "Futuo"), ("mn", "Фү Тоо"), ("mr", "फ\u{942} थ\u{945}"), ("ms", "Phe Thọ"), ("nb", "Phu Tho"), ("nl", "Phú Thọ"), ("no", "Phu Tho"), ("pl", "Prowincja Phú Thọ"), ("pt", "Phu Tho"), ("ro", "Phú Thọ"), ("ru", "Футхо"), ("si", "ෆ\u{dd4} -තෝ"), ("sv", "Phu Tho"), ("sw", "Mkoa wa Phú Thọ"), ("ta", "ப\u{bcd}ஹு தொ"), ("te", "ఫూ త\u{c4b}"), ("th", "จ\u{e31}งหว\u{e31}ดฟ\u{e39}\u{e49}เถาะ"), ("tr", "Phu Tho"), ("uk", "Футхо"), ("ur", "فو تھو صوبہ"), ("vi", "Phú Thọ"), ("yue", "富壽"), ("yue_Hans", "富寿"), ("zh", "富壽省")]),
                        unofficial_name_list: ["Phu Tho"].to_vec(),
                    }
                ),
                (
                    "69",
                    Subdivision{
                        name: "69",
                        country_alpha2: Alpha2::VN,
                        code: "69",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.5613771), longitude: Some(105.876004), max_latitude: Some(22.047269), min_latitude: Some(21.324607), max_longitude: Some(106.23657), min_longitude: Some(105.4771269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ثاي نجوين"), ("bg", "Тай Нгуен"), ("bn", "তৈইয\u{9bc}\u{9c2}আন"), ("ccp", "𑄗\u{1112d} 𑄉\u{1112a}𑄠𑄬𑄚\u{11134}"), ("ceb", "Tỉnh Thái Nguyên"), ("cs", "Thai Nguyen"), ("da", "Thái Nguyên"), ("de", "Thái Nguyên"), ("el", "Τάι Νγκουγιέν"), ("en", "Thái Nguyên"), ("es", "Thái Nguyên"), ("fa", "استان تای نگوین"), ("fi", "Thái Nguyên"), ("fr", "Province de Thái Nguyên"), ("gu", "થાઈ એનગ\u{acd}ય\u{ac1}એન"), ("hi", "थाई ग\u{94d}य\u{941}एन प\u{94d}रा\u{902}त"), ("id", "Provinsi Thai Nguyen"), ("it", "provincia di Thai Nguyen"), ("ja", "タイグエン省"), ("kn", "ಥ\u{cbf} ನ\u{ccd}ಗುಯ\u{cc6}ನ\u{ccd}"), ("ko", "타이응우옌 성"), ("lt", "Tai Ngujenas"), ("lv", "Thajngujenas province"), ("mn", "Тайнюэнь"), ("mr", "थ\u{948} नग\u{941}य\u{947}न"), ("ms", "Thai Nguyen"), ("nb", "Thai Nguyen"), ("nl", "Thái Nguyên"), ("no", "Thai Nguyen"), ("pl", "Prowincja Thái Nguyên"), ("pt", "Thái Nguyên"), ("ro", "Thái Nguyên"), ("ru", "Тхайнгуен"), ("si", "ත\u{dcf}ය\u{dd2} එන\u{dca}ග\u{dd4}යෙන\u{dca}"), ("sr", "Тај Нгујен"), ("sr_Latn", "Taj Ngujen"), ("sv", "Thai Nguyen"), ("sw", "Mkoa wa Thái Nguyên"), ("ta", "த\u{bbe}ய\u{bcd} ங\u{bcd}குயின\u{bcd}"), ("te", "థ\u{c3e}య\u{c3f} గుయ\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดท\u{e49}ายเงว\u{e35}ยน"), ("tr", "Thai Nguyen"), ("uk", "Тхайнгуєн"), ("ur", "تھائی نگوین صوبہ"), ("vi", "Thái Nguyên"), ("yue", "太原"), ("yue_Hans", "太原"), ("zh", "太原省")]),
                        unofficial_name_list: ["Central Highlands"].to_vec(),
                    }
                ),
                (
                    "70",
                    Subdivision{
                        name: "70",
                        country_alpha2: Alpha2::VN,
                        code: "70",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.3608805), longitude: Some(105.5474373), max_latitude: Some(21.573348), min_latitude: Some(21.10479), max_longitude: Some(105.7897179), min_longitude: Some(105.322971)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فنه فو"), ("bg", "Вин Фук"), ("bn", "ভিহ\u{9cd}ন ফ\u{9c1}ক"), ("ccp", "𑄞\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄜\u{1112a}𑄇\u{11134}"), ("ceb", "Tỉnh Vĩnh Phúc"), ("cs", "Vinh Phuc"), ("da", "Vĩnh Phúc"), ("de", "Vĩnh Phúc"), ("el", "Βινχ Φουκ"), ("en", "Vĩnh Phúc"), ("es", "Vĩnh Phúc"), ("fa", "استان وین فوک"), ("fi", "Vĩnh Phúc"), ("fr", "Province de Vĩnh Phúc"), ("gu", "વિન\u{acd}હ ફ\u{ac2}ક\u{ac1}"), ("hi", "विन\u{94d}ह फ\u{942}क"), ("id", "Provinsi Vinh Phuc"), ("it", "provincia di Vinh Phuc"), ("ja", "ヴィンフック省"), ("kn", "ವನ\u{ccd}ಹ\u{ccd} ಫುಕ\u{ccd}"), ("ko", "빈푹 성"), ("lt", "Vinfuko provincija"), ("lv", "Viņfukas province"), ("mn", "Вэн Фүк"), ("mr", "विनह फ\u{941}क"), ("ms", "Vinh Phuc"), ("nb", "Vinh Phuc"), ("nl", "Vĩnh Phúc"), ("no", "Vinh Phuc"), ("pl", "Prowincja Vĩnh Phúc"), ("pt", "Vinh Phuc"), ("ro", "Vĩnh Phúc"), ("ru", "Виньфук"), ("si", "ව\u{dd2}න\u{dca}හ\u{dca} ප\u{dd4}ක\u{dca}"), ("sv", "Vinh Phuc"), ("sw", "Mkoa wa Vĩnh Phúc"), ("ta", "வின\u{bcd}ஹ\u{bcd} பிஹுக\u{bcd}"), ("te", "వ\u{c3f}న\u{c4d} ఫుక\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดหว\u{e34}ญฟ\u{e38}ก"), ("tr", "Vĩnh Phúc"), ("uk", "Віньфук"), ("ur", "وینہ فوک صوبہ"), ("vi", "Vĩnh Phúc"), ("yue", "永福"), ("yue_Hans", "永福"), ("zh", "永福省")]),
                        unofficial_name_list: ["Vinh Phuc"].to_vec(),
                    }
                ),
                (
                    "71",
                    Subdivision{
                        name: "71",
                        country_alpha2: Alpha2::VN,
                        code: "71",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.8042309), longitude: Some(103.1076525), max_latitude: Some(22.5563429), min_latitude: Some(20.869232), max_longitude: Some(103.6003289), min_longitude: Some(102.1482091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دين بين"), ("bg", "Диен Биен"), ("bn", "দিয\u{9bc}েন বিয\u{9bc}েন"), ("ccp", "𑄓\u{11128}𑄠𑄬𑄚\u{11134} 𑄝\u{1112d}𑄠𑄬𑄚\u{11134}"), ("ceb", "Tỉnh Ðiện Biên"), ("cs", "Dien Bien"), ("da", "Dien Bien"), ("de", "Điện Biên"), ("el", "Ντιέν Μπιέν"), ("en", "Điện Biên"), ("es", "Điện Biên"), ("fa", "استان دین\u{200c}بین"), ("fi", "Điện Biên"), ("fr", "Province de Điện Biên"), ("gu", "ડીએન બીએન"), ("hi", "दिएन बिएन प\u{94d}रा\u{902}त"), ("id", "Provinsi Dien Bien"), ("it", "provincia di Dien Bien"), ("ja", "ディエンビエン省"), ("kn", "ಡ\u{cbf}ಯ\u{cc6}ನ\u{ccd} ಬ\u{cbf}ಯ\u{cc6}ನ\u{ccd}"), ("ko", "디엔비엔 성"), ("lt", "Djenbjenas"), ("lv", "Djenbjenas province"), ("mr", "डीएन बिएन"), ("ms", "Dien Bien"), ("nb", "Dien Bien"), ("nl", "Điện Biên"), ("no", "Dien Bien"), ("pl", "Prowincja Điện Biên"), ("pt", "Dien Bien"), ("ru", "Дьенбьен"), ("si", "ඩ\u{dd2}යෙන\u{dca} බ\u{dd2}යෙන\u{dca}"), ("sr", "Дијен Бијен"), ("sr_Latn", "Dijen Bijen"), ("sv", "Dien Bien"), ("sw", "Mkoa wa Điện Biên"), ("ta", "டின\u{bcd} பியன\u{bcd}"), ("te", "డ\u{c40}న\u{c4d} బ\u{c40}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเด\u{e35}\u{e48}ยนเบ\u{e35}ยน"), ("tr", "Dien Bien"), ("uk", "Дьєнбʼєн"), ("ur", "دیئن بیئن صوبہ"), ("vi", "Điện Biên"), ("yue", "奠邊"), ("yue_Hans", "奠边"), ("zh", "奠邊省")]),
                        unofficial_name_list: ["Dien Bien"].to_vec(),
                    }
                ),
                (
                    "72",
                    Subdivision{
                        name: "72",
                        country_alpha2: Alpha2::VN,
                        code: "72",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.2646476), longitude: Some(107.609806), max_latitude: Some(12.8117129), min_latitude: Some(11.748865), max_longitude: Some(108.115932), min_longitude: Some(107.2079091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة داك نانج"), ("bg", "Дак Нонг"), ("bn", "ড\u{9be}ক নোং"), ("ccp", "𑄓𑄇\u{11134} 𑄚\u{11127}\u{11101}"), ("ceb", "Ðắk Nông"), ("cs", "Đắk Nông"), ("da", "Dak Nong"), ("de", "Đắk Nông"), ("el", "Ντακ Νονγκ"), ("en", "Đắk Nông"), ("es", "Đăk Nông"), ("fa", "استان داک نونگ"), ("fi", "Đắk Nông"), ("fr", "Đắk Nông"), ("gu", "ડક નો\u{a82}ગ"), ("hi", "डाक नो\u{902}ग"), ("id", "Provinsi Dak Nong"), ("it", "provincia di Dak Nong"), ("ja", "ダクノン省"), ("kn", "ಡಕ\u{ccd} ನಾಂಗ\u{ccd}"), ("ko", "닥농 성"), ("lt", "Daknongas"), ("lv", "Daknona"), ("mr", "डक नॉन\u{94d}ग"), ("ms", "Dak Nong"), ("nb", "Dak Nong"), ("nl", "Đắk Nông"), ("no", "Dak Nong"), ("pl", "Prowincja Đăk Nông"), ("pt", "Dak Nong"), ("ru", "Дакнонг"), ("si", "ඩක\u{dca} නොන\u{dca}ග\u{dca}"), ("sv", "Dak Nong"), ("sw", "Mkoa wa Đắk Nông"), ("ta", "ட\u{bbe}க\u{bcd} ந\u{bbe}ங\u{bcd}"), ("te", "డ\u{c3e}క\u{c4d} న\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e31}กโนง"), ("tr", "Dak Nong"), ("uk", "Дакнонг"), ("ur", "داک نونگ صوبہ"), ("vi", "Đắk Nông"), ("yue", "得農"), ("yue_Hans", "得农"), ("zh", "得農省")]),
                        unofficial_name_list: ["Dak Nong"].to_vec(),
                    }
                ),
                (
                    "73",
                    Subdivision{
                        name: "73",
                        country_alpha2: Alpha2::VN,
                        code: "73",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.757897999999999), longitude: Some(105.6412527), max_latitude: Some(9.9928138), min_latitude: Some(9.5820831), max_longitude: Some(105.8934326), min_longitude: Some(105.328687)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هو زانغ"), ("bg", "Хау Жианг"), ("bn", "হ\u{9be}\u{9c1} গিয\u{9bc}\u{9be}ং"), ("ccp", "𑄦\u{11127}𑄅\u{1112a} 𑄎\u{11128}𑄠\u{11101}"), ("ceb", "Hau Giang"), ("da", "Hậu Giang"), ("de", "Hậu Giang"), ("el", "Χο Γκιάνγκ"), ("en", "Hậu Giang"), ("es", "Hậu Giang"), ("fa", "استان هائو گیانگ"), ("fi", "Hậu Giang"), ("fr", "Province de Hậu Giang"), ("gu", "હ\u{ac1}ઉ ગિઆ\u{a82}ગ"), ("hi", "हाउ जिए\u{902}ग"), ("id", "Provinsi Hau Giang"), ("it", "provincia di Hau Giang"), ("ja", "ハウザン省"), ("kn", "ಹು ಜ\u{cbf}ಯಾಂಗ\u{ccd}"), ("ko", "하우장 성"), ("lt", "Haudžiango provincija"), ("lv", "Houzana"), ("mr", "ह\u{94d}य\u{942} गिया\u{902}ग"), ("ms", "Hau Giang"), ("nb", "Hau Giang"), ("nl", "Hậu Giang"), ("no", "Hau Giang"), ("pl", "Prowincja Hậu Giang"), ("pt", "Hau Giang"), ("ru", "Хаузянг"), ("si", "හෞ ජ\u{dd2}යැන\u{dca}ග\u{dca}"), ("sv", "Hau Giang"), ("sw", "Mkoa wa Hậu Giang"), ("te", "హ\u{c3e}వు గ\u{c3f}య\u{c3e}ంగ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e39}เซ"), ("tr", "Hau Giang"), ("uk", "Хаузянг"), ("ur", "ہآو گیانگ صوبہ"), ("vi", "Hậu Giang"), ("yue", "後江"), ("yue_Hans", "后江"), ("zh", "後江省")]),
                        unofficial_name_list: ["Hau Giang"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "CT",
                        country_alpha2: Alpha2::VN,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.0451618), longitude: Some(105.7468535), max_latitude: Some(10.0746025), min_latitude: Some(9.993702899999999), max_longitude: Some(105.7959312), min_longitude: Some(105.7170582)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Can Tho"), ("ar", "كان ثو"), ("be", "Кантхо"), ("bg", "Кан Тхо"), ("bn", "ক\u{9be}ন থো"), ("ca", "Cần Thơ"), ("ccp", "𑄇𑄚\u{11134} 𑄗\u{1112e}"), ("ceb", "Thành Phố Cần Thơ (lalawigan sa Vietnam)"), ("cs", "Can Tho"), ("da", "Can Tho"), ("de", "Cần Thơ"), ("el", "Καν Θο"), ("en", "Can Tho"), ("es", "Cần Thơ"), ("eu", "Can Tho"), ("fa", "کان تو"), ("fi", "Cần Thơ"), ("fr", "Cần Thơ"), ("gl", "Can Tho"), ("gu", "કાન થો"), ("ha", "Can Tho"), ("ha_NE", "Can Tho"), ("he", "קאנטחו"), ("hi", "क\u{948}न थो"), ("hu", "Cần Thơ"), ("id", "Cần Thơ"), ("it", "Cần Thơ"), ("ja", "カントー"), ("km", "ទ\u{17b8}ក\u{17d2}រ\u{17bb}ងព\u{17d2}រែកឫស\u{17d2}ស\u{17b8}"), ("kn", "ಸ\u{cbf}ನ\u{ccd} ತೊ"), ("ko", "껀터"), ("ky", "Кантхо"), ("lt", "Kantas"), ("lv", "Kontho"), ("mn", "Кантхо"), ("mr", "कॉ थो"), ("ms", "Can Tho"), ("nb", "Can Tho"), ("nl", "Cần Thơ"), ("no", "Can Tho"), ("pl", "Cần Thơ"), ("pt", "Can Tho"), ("ru", "Кантхо"), ("si", "ක\u{dcf}න\u{dca} තො"), ("sr", "Кантхо"), ("sr_Latn", "Kantho"), ("sv", "Can Tho"), ("sw", "Can Tho"), ("ta", "கேன\u{bcd} தொ"), ("te", "క\u{c3e}న\u{c4d} త\u{c4b}"), ("th", "เก\u{e34}\u{e48}นเทอ"), ("tr", "Cần Thơ"), ("uk", "Кантхо"), ("ur", "کآن تھؤ"), ("vi", "Cần Thơ"), ("yue", "芹苴"), ("yue_Hans", "芹苴"), ("zh", "芹苴市")]),
                        unofficial_name_list: ["Can Tho"].to_vec(),
                    }
                ),
                (
                    "DN",
                    Subdivision{
                        name: "DN",
                        country_alpha2: Alpha2::VN,
                        code: "DN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.0544068), longitude: Some(108.2021667), max_latitude: Some(16.0941455), min_latitude: Some(15.999203), max_longitude: Some(108.2354165), min_longitude: Some(108.1779956)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Da Nang"), ("am", "ዳ ናንግ"), ("ar", "دا نانغ"), ("az", "Dananq"), ("be", "Горад Дананг"), ("bg", "Дананг"), ("bn", "ড\u{9be}ন\u{9be}ং"), ("ca", "Da Nang"), ("ccp", "𑄓 𑄚\u{11127}\u{11101}"), ("ceb", "Da Nang"), ("cs", "Danang"), ("cy", "Da Nang"), ("da", "Da Nang"), ("de", "Đà Nẵng"), ("el", "Ντα Νανγκ"), ("en", "Da Nang"), ("es", "Đà Nẵng"), ("et", "Đà Nẵng"), ("eu", "Da Nang"), ("fa", "دانانگ"), ("fi", "Đà Nẵng"), ("fr", "Đà Nẵng"), ("gl", "Đà Nẵng"), ("gu", "દાના\u{a82}ગ"), ("ha", "Da Nang"), ("ha_NE", "Da Nang"), ("he", "דננג"), ("hi", "दा ना\u{902}ग"), ("hr", "Đà Nẵng"), ("hu", "Đà Nẵng"), ("hy", "Դանանգ"), ("id", "Đà Nẵng"), ("ig", "Da Nang"), ("it", "Da Nang"), ("ja", "ダナン"), ("jv", "Da Nang"), ("kn", "ದಾನಂಗ\u{ccd}"), ("ko", "다낭"), ("lt", "Danangas"), ("lv", "Dananga"), ("mk", "Да Нанг"), ("mn", "Данан"), ("mr", "दाना\u{902}ग"), ("ms", "Da Nang"), ("my", "ဒါနန\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Da Nang"), ("nl", "Đà Nẵng"), ("no", "Da Nang"), ("pl", "Đà Nẵng"), ("pt", "Da Nang"), ("ro", "Da Nang"), ("ru", "Дананг"), ("si", "ඩනන\u{dca}"), ("sk", "Đà Nẵng"), ("sq", "Da Nang"), ("sr", "Да Нанг"), ("sr_Latn", "Da Nang"), ("sv", "Da Nang"), ("sw", "Da Nang"), ("ta", "த\u{bbe} ந\u{bbe}ங\u{bcd}"), ("te", "డ\u{c3e}న\u{c3e}ంగ\u{c4d}"), ("th", "ดาน\u{e31}ง"), ("tk", "Da Nang"), ("tr", "Đà Nẵng"), ("uk", "Дананг"), ("ur", "دا نانگ"), ("uz", "Danang"), ("vi", "Đà Nẵng"), ("yue", "峴港市"), ("yue_Hans", "岘港市"), ("zh", "岘港市"), ("zu", "IDanang")]),
                        unofficial_name_list: ["Da Nang, thanh pho"].to_vec(),
                    }
                ),
                (
                    "HN",
                    Subdivision{
                        name: "HN",
                        country_alpha2: Alpha2::VN,
                        code: "HN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.0277644), longitude: Some(105.8341598), max_latitude: Some(21.0502942), min_latitude: Some(20.9950991), max_longitude: Some(105.8764459), min_longitude: Some(105.7974815)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hanoi"), ("am", "ሀኖይ"), ("ar", "هانوي"), ("az", "Hanoy"), ("be", "Горад Ханой"), ("bg", "Ханой"), ("bn", "হ\u{9cd}য\u{9be}নয\u{9bc}"), ("bs", "Hanoi"), ("ca", "Hanoi"), ("ccp", "𑄦𑄚\u{11130}"), ("ceb", "Hanoi"), ("cs", "Hanoj"), ("cy", "Hanoi"), ("da", "Hanoi"), ("de", "Hanoi"), ("el", "Ανόι"), ("en", "Hanoi"), ("es", "Hanói"), ("et", "Hanoi"), ("eu", "Hanoi"), ("fa", "هانوی"), ("fi", "Hanoi"), ("fr", "Hanoï"), ("ga", "Ha Noi"), ("gl", "Hanoi"), ("gu", "હનોઈ"), ("ha", "Hanoi"), ("ha_NE", "Hanoi"), ("he", "האנוי"), ("hi", "हनोई"), ("hr", "Hanoi"), ("hu", "Hanoi"), ("hy", "Հանոյ"), ("id", "Hanoi"), ("is", "Hanoí"), ("it", "Hanoi"), ("ja", "ハノイ"), ("jv", "Hanoi"), ("ka", "ჰანოი"), ("kk", "Ханой"), ("km", "ទ\u{17b8}ក\u{17d2}រ\u{17bb}ងហាន\u{17bc}យ"), ("kn", "ಹಾನೊಯ\u{ccd}"), ("ko", "하노이"), ("ky", "Ханой"), ("lo", "ຮ\u{ec8}າໂນ\u{ec9}ຍ"), ("lt", "Hanojus"), ("lv", "Hanoja"), ("mk", "Ханој"), ("ml", "ഹ\u{d3e}നോയ\u{d4d}"), ("mn", "Ханой"), ("mr", "हनोई"), ("ms", "Hanoi"), ("my", "ဟန\u{103d}\u{102d}\u{102f}င\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Hanoi"), ("ne", "हनोइ"), ("nl", "Hanoi"), ("no", "Hanoi"), ("or", "ହ\u{b3e}ନୋଇ"), ("pa", "ਹਨ\u{a4b}ਈ"), ("pl", "Hanoi"), ("pt", "Hanói"), ("ro", "Hanoi"), ("ru", "Ханой"), ("si", "හැනෝය\u{dd2}"), ("sk", "Hanoj"), ("sl", "Hanoj"), ("sq", "Hanoi"), ("sr", "Ханој"), ("sr_Latn", "Hanoj"), ("sv", "Hanoi"), ("sw", "Hanoi"), ("ta", "ஹனோய\u{bcd}"), ("te", "హన\u{c4b}య\u{c4d}"), ("th", "ฮานอย"), ("tk", "Hanoýi"), ("tr", "Hanoi"), ("uk", "Ханой"), ("ur", "ہنوئی"), ("uz", "Xanoy"), ("vi", "Hà Nội"), ("yo", "Hanoi"), ("yo_BJ", "Hanoi"), ("yue", "河內"), ("yue_Hans", "河内"), ("zh", "河內市")]),
                        unofficial_name_list: ["Ha Noi, thu do"].to_vec(),
                    }
                ),
                (
                    "HP",
                    Subdivision{
                        name: "HP",
                        country_alpha2: Alpha2::VN,
                        code: "HP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.8449115), longitude: Some(106.6880841), max_latitude: Some(20.8792627), min_latitude: Some(20.814211), max_longitude: Some(106.759901), min_longitude: Some(106.6375924)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hai Phong"), ("ar", "هايفونغ"), ("be", "Горад Хайфон"), ("bg", "Хайфонг"), ("bn", "হ\u{9be}ইফোং"), ("ca", "Hai Phong"), ("ccp", "𑄦\u{1112d}𑄜\u{11127}\u{11101}"), ("ceb", "Haiphong"), ("cs", "Haiphong"), ("da", "Hai Phong"), ("de", "Hải Phòng"), ("el", "Χάι Φονγκ"), ("en", "Haiphong"), ("es", "Hải Phòng"), ("et", "Hải Phòng"), ("eu", "Hai Phong"), ("fa", "هایفونگ"), ("fi", "Hải Phòng"), ("fr", "Hải Phòng"), ("gu", "હ\u{ac8}ફ\u{a82}ગ"), ("ha", "Haiphong"), ("ha_NE", "Haiphong"), ("he", "היפונג"), ("hi", "हाईफो\u{902}ग"), ("hu", "Hải Phòng"), ("hy", "Հայֆոն"), ("id", "Hải Phòng"), ("it", "Haiphong"), ("ja", "ハイフォン"), ("kn", "ಹಾಫ\u{cbf}ಂಗ\u{ccd}"), ("ko", "하이퐁"), ("lt", "Haifongas"), ("lv", "Haifona"), ("ml", "ഹൈ ഫോങ\u{d4d}"), ("mr", "हाय फा\u{901}ग"), ("ms", "Haiphong"), ("nb", "Haiphong"), ("nl", "Hải Phòng"), ("no", "Haiphong"), ("or", "ହ\u{b3e}ଈ ଫୋଙ\u{b4d}ଗ"), ("pl", "Hajfong"), ("pt", "Haiphong"), ("ru", "Хайфон"), ("si", "හය\u{dd2} ෆොන\u{dca}ග\u{dca}"), ("sr", "Хајфонг"), ("sr_Latn", "Hajfong"), ("sv", "Hai Phong"), ("sw", "Hai Phong"), ("ta", "ஹ\u{bbe}ய\u{bcd} ப\u{bbe}ங\u{bcd}"), ("te", "హ\u{c48}ఫ\u{c4b}ంగ\u{c4d}"), ("th", "ไฮฟอง"), ("tr", "Hải Phòng"), ("uk", "Хайфонг"), ("ur", "ہائیفونگ"), ("vi", "Hải Phòng"), ("yue", "海防"), ("yue_Hans", "海防"), ("zh", "海防市")]),
                        unofficial_name_list: ["Hai Phong, thanh pho"].to_vec(),
                    }
                ),
                (
                    "SG",
                    Subdivision{
                        name: "SG",
                        country_alpha2: Alpha2::VN,
                        code: "SG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.8230989), longitude: Some(106.6296638), max_latitude: Some(11.1602136), min_latitude: Some(10.3766885), max_longitude: Some(107.0248468), min_longitude: Some(106.3638784)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ho Chi Minh-stad"), ("am", "ሆ ቺ ሚን ከተማ"), ("ar", "هو تشي منه"), ("az", "Hoşimin"), ("be", "Горад Хашымін"), ("bg", "Хошимин"), ("bn", "হো চি মিন সিটি"), ("bs", "Ho Ši Min Grad"), ("ca", "Ciutat Ho Chi Minh"), ("ccp", "𑄦\u{1112e} 𑄌\u{11128} 𑄟\u{11128}𑄚\u{11134}𑄦\u{11134} 𑄌\u{11128}𑄑\u{11128}"), ("ceb", "Dakbayan sa Ho Chi Minh"), ("cs", "Ho Či Minovo Město"), ("cy", "Dinas Ho Chi Minh"), ("da", "Ho Chi Minh-byen"), ("de", "Ho-Chi-Minh-Stadt"), ("el", "Χο Τσι Μινχ"), ("en", "Ho Chi Minh City"), ("es", "Ciudad Ho Chi Minh"), ("et", "Hồ Chí Minh"), ("eu", "Ho Chi Minh Hiria"), ("fa", "هوشی\u{200c}مین"), ("fi", "Hồ Chí Minhin kaupunki"), ("fr", "Hô-Chi-Minh-Ville"), ("ga", "Cathair Ho Chi Minh"), ("gl", "Cidade Ho Chi Minh"), ("gu", "હો ચી મિન સિટી"), ("ha", "Birnin Ho Chi Minh"), ("ha_NE", "Birnin Ho Chi Minh"), ("he", "הו צ׳י מין סיטי"), ("hi", "हो ची मिन\u{94d}ह शहर"), ("hr", "Ho Ši Min"), ("hu", "Ho Si Minh-város"), ("hy", "Հոշիմին"), ("id", "Kota Hồ Chí Minh"), ("ig", "Ho Chi Minh City"), ("is", "Ho Chi Minh-borg"), ("it", "Ho Chi Minh"), ("ja", "ホーチミン市"), ("jv", "Kutha Ho Chi Minh"), ("ka", "ხოშიმინი"), ("kk", "Хошимин"), ("km", "ក\u{17d2}រ\u{17bb}ងព\u{17d2}រៃនគរ"), ("kn", "ಹೊ ಚ\u{cbf} ಮ\u{cbf}ನ\u{ccd} ನಗರ"), ("ko", "호찌민 시"), ("ky", "Хошимин"), ("lt", "Hošiminas"), ("lv", "Hošimina"), ("mk", "Хо Ши Мин"), ("ml", "ഹോ ചി മിൻ നഗരം"), ("mn", "Хо Ши Мин хот"), ("mr", "हो चि मिन\u{94d}ह सिटी"), ("ms", "Bandar Raya Ho Chi Minh"), ("my", "ဟ\u{102d}\u{102f}ချ\u{102e}မင\u{103a}းစ\u{102e}းတ\u{102e}း"), ("nb", "Ho Chi Minh-byen"), ("nl", "Ho Chi Minhstad"), ("no", "Ho Chi Minh-byen"), ("or", "ସ\u{b3e}ଇଗନ"), ("pl", "Ho Chi Minh"), ("pt", "Cidade de Ho Chi Minh"), ("ro", "Ho Și Min"), ("ru", "Хошимин"), ("si", "හෝ ච\u{dd2} ම\u{dd2}න\u{dca} නගරය"), ("sk", "Hočiminovo Mesto"), ("sl", "Hošiminh"), ("so", "Ho Chi Minh City"), ("sq", "Ho-Chi-Minh-Qytet"), ("sr", "Хо Ши Мин"), ("sr_Latn", "Ho Ši Min"), ("sv", "Ho Chi Minh-staden"), ("sw", "Mji wa Ho Chi Minh"), ("ta", "ஹோ சி மின\u{bcd} நகரம\u{bcd}"), ("te", "హ\u{c4b}చ\u{c3f}మ\u{c3f}న\u{c4d} స\u{c3f}ట\u{c40}"), ("th", "นครโฮจ\u{e34}ม\u{e34}นห\u{e4c}"), ("tk", "Ho Şi Miň Şäheri"), ("tr", "Ho Chi Minh Kenti"), ("uk", "Хошимін"), ("ur", "ہو چی من"), ("uz", "Xoshimin"), ("vi", "Thành phố Hồ Chí Minh"), ("yue", "胡志明市"), ("yue_Hans", "胡志明市"), ("zh", "胡志明市"), ("zu", "IHochiminh")]),
                        unofficial_name_list: ["Sai Gon"].to_vec(),
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
#[cfg(feature = "vn")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::VN,
        alpha3: Alpha3::VNM,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 84,
        currency_code: "VND",
        gec: Some(GEC::VM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("VIE"),
        iso_long_name: "The Socialist Republic of Viet Nam",
        iso_short_name: "Viet Nam",
        official_language_list: ["vi"].to_vec(),
        spoken_language_list: ["vi"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Vietnamese"),
        number: "704",
        postal_code: true,
        postal_code_format: Some("\\d{5}\\d?"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "VN",
        unofficial_name_list: ["Vietnam", "ベトナム", "Viet Nam"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Vietnam"),
            ("af", "Viëtnam"),
            ("ak", "Vietnam"),
            ("am", "ቬት ናም"),
            ("an", "Vietnam"),
            ("ar", "الفيتنام"),
            ("as", "ভিয়েতন\u{9be}ম"),
            ("ay", "Vietnam"),
            ("az", "Vietnam"),
            ("ba", "Vietnam"),
            ("be", "В'етнам"),
            ("bg", "Виетнам"),
            ("bi", "Vietnam"),
            ("bn", "ভিয়েতন\u{9be}ম"),
            ("bn_IN", "ভিয়েতন\u{9be}ম"),
            ("br", "Viêt Nam"),
            ("bs", "Vijetnam"),
            ("ca", "Vietnam"),
            ("ce", "Вьетнам"),
            ("ch", "Vietnam"),
            ("cs", "Vietnam"),
            ("cv", "Вьетнам"),
            ("cy", "Fietnam"),
            ("da", "Vietnam"),
            ("de", "Vietnam"),
            ("dv", "ވ\u{7a8}އ\u{7ac}ޓ\u{7aa}ނ\u{7a7}މ\u{7aa}"),
            ("dz", "ཝ\u{f7a}ཊ\u{f72}་ནམ།"),
            ("ee", "Vietnam"),
            ("el", "Βιετνάμ"),
            ("en", "Vietnam"),
            ("eo", "Vjetnamio"),
            ("es", "Vietnam"),
            ("et", "Vietnam"),
            ("eu", "Vietnam"),
            ("fa", "ویتنام"),
            ("ff", "Vietnam"),
            ("fi", "Vietnam"),
            ("fo", "Vietnam"),
            ("fr", "Viêt Nam"),
            ("fy", "Fjetnam"),
            ("ga", "Vítneam"),
            ("gl", "Vietnam"),
            ("gn", "Vietnam"),
            ("gu", "વિય\u{ac7}તનામ"),
            ("gv", "Yn Vietnam"),
            ("ha", "Vietnam"),
            ("he", "וייטנאם"),
            ("hi", "वियतनाम"),
            ("hr", "Vijetnam"),
            ("ht", "Vyetnam"),
            ("hu", "Vietnám"),
            ("hy", "Վիետնամ"),
            ("ia", "Vietnam"),
            ("id", "Vietnam"),
            ("io", "Vietnam"),
            ("is", "Víetnam"),
            ("it", "Vietnam"),
            ("iu", "Vietnam"),
            ("ja", "ベトナム"),
            ("ka", "ვიეტ-ნამი"),
            ("ki", "Vietnam"),
            ("kk", "Вьетнам"),
            ("kl", "Vietnam"),
            ("km", "វៀតណាម"),
            ("kn", "ವ\u{cbf}ಯ\u{cc6}ಟ\u{ccd}ನಾಂ"),
            ("ko", "베트남"),
            ("ku", "Viyetnam"),
            ("kv", "Вьетнам"),
            ("kw", "Vietnam"),
            ("ky", "Вьетнам"),
            ("lo", "ປະເທດຫວຽດນາມ"),
            ("lt", "Vietnamas"),
            ("lv", "Vjetnama"),
            ("mi", "Whitināmu"),
            ("mk", "Виетнам"),
            ("ml", "വിയറ\u{d4d}റ\u{d4d}ന\u{d3e}ം"),
            ("mn", "Vietnam"),
            ("mr", "व\u{94d}हिय\u{947}तनाम"),
            ("ms", "Vietnam"),
            ("mt", "Vjetnam"),
            (
                "my",
                "ဗ\u{102e}ယက\u{103a}နမ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Bitinam"),
            ("nb", "Vietnam"),
            ("ne", "भिएतनाम"),
            ("nl", "Vietnam"),
            ("nn", "Vietnam"),
            ("nv", "Vietnam"),
            ("oc", "Vietnam"),
            ("or", "ଭ\u{b3f}ୟେତନ\u{b3e}ମ"),
            ("pa", "ਵੀਅਤਨਾਮ"),
            ("pi", "विय\u{947}तनाम"),
            ("pl", "Wietnam"),
            ("ps", "Vietnam"),
            ("pt", "Vietname"),
            ("pt_BR", "Vietnã"),
            ("ro", "Vietnam"),
            ("ru", "Вьетнам"),
            ("rw", "Viyetinamu"),
            ("sc", "Vietnam"),
            ("sd", "ويٽنام"),
            ("si", "ව\u{dd2}යෙට\u{dca}න\u{dcf}මය"),
            ("sk", "Vietnam"),
            ("sl", "Vietnam"),
            ("so", "Fiyetnaam"),
            ("sq", "Vietnam"),
            ("sr", "Вијетнам"),
            ("sv", "Vietnam"),
            ("sw", "Vietnam"),
            ("ta", "வியட\u{bcd}ன\u{bbe}ம\u{bcd}"),
            ("te", "వ\u{c3f}యత\u{c4d}న\u{c3e}మ\u{c4d}"),
            ("tg", "Ветнам"),
            ("th", "เว\u{e35}ยดนาม"),
            ("ti", "ቬትናም"),
            ("tk", "Wýetnam"),
            ("tl", "Viet Nam"),
            ("tr", "Vietnam"),
            ("tt", "Виет Нам"),
            ("ug", "ۋيېتنام"),
            ("uk", "В'єтнам"),
            ("ur", "ویتنام"),
            ("uz", "Vyetnam"),
            ("ve", "Viëtnam"),
            ("vi", "Việt Nam"),
            ("wa", "Vietnam"),
            ("wo", "Wiyet Naam"),
            ("xh", "Vietnam"),
            ("yo", "Fiẹtnám"),
            ("zh_CN", "越南"),
            ("zh_HK", "越南"),
            ("zh_TW", "越南"),
            ("zu", "IViyetnami"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}