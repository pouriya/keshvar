// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Indonesia

#[cfg(all(feature = "id", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::ID;
    pub const ALPHA3: Alpha3 = Alpha3::IDN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 62;
    pub const CURRENCY_CODE: &str = "IDR";
    pub const GEC: Option<GEC> = Some(GEC::ID);
    pub const INTERNATIONAL_PREFIX: &str = "001";
    pub const IOC: Option<IOC> = Some(IOC::INA);
    pub const ISO_SHORT_NAME: &str = "Indonesia";
    pub const ISO_LONG_NAME: &str = "The Republic of Indonesia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["id"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["id"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9, 10, 11];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Indonesian");
    pub const NUMBER: &str = "360";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "ID";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Indonesia",
        "Indonesien",
        "Indonésie",
        "インドネシア",
        "Indonesië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Indonesia"),
        ("af", "Indonesië"),
        ("ak", "Indonesia"),
        ("am", "ጒን፦ኔፒ።"),
        ("an", "Indonesia"),
        ("ar", "إندونيسيا"),
        ("as", "ইণ\u{9cd}ডোনেছিয়\u{9be}"),
        ("ay", "Indonesia"),
        ("az", "İndoneziya"),
        ("ba", "Indonesia"),
        ("be", "Інданезія"),
        ("bg", "Индонезия"),
        ("bi", "Indonesia"),
        ("bn", "ইন\u{9cd}দোনেশিয়\u{9be}"),
        ("bn_IN", "ইন\u{9cd}দোনেশিয়\u{9be}"),
        ("br", "Indonezia"),
        ("bs", "Indonezija"),
        ("ca", "Indonèsia"),
        ("ce", "Индонези"),
        ("ch", "Indonesia"),
        ("cs", "Indonésie"),
        ("cv", "Индонези"),
        ("cy", "Indonesia"),
        ("da", "Indonesien"),
        ("de", "Indonesien"),
        ("dv", "އ\u{7a8}ނ\u{7b0}ޑ\u{7ae}ނ\u{7ad}ޝ\u{7a8}ޔ\u{7a7}"),
        ("dz", "ཨ\u{f72}ན་ཌ\u{f7c}་ན\u{f7a}་ཤ\u{f72}་ཡ།"),
        ("ee", "Indonesia"),
        ("el", "Ινδονησία"),
        ("en", "Indonesia"),
        ("eo", "Indonezio"),
        ("es", "Indonesia"),
        ("et", "Indoneesia"),
        ("eu", "Indonesia"),
        ("fa", "اندونزی"),
        ("ff", "Indonesia"),
        ("fi", "Indonesia"),
        ("fo", "Indonesia"),
        ("fr", "Indonésie"),
        ("fy", "Yndoneezje"),
        ("ga", "An Indinéis"),
        ("gl", "Indonesia"),
        ("gn", "Indonesia"),
        ("gu", "ઇન\u{acd}ડોન\u{ac7}શિયા"),
        ("gv", "Yn Indoneesh"),
        ("ha", "Indonesiya"),
        ("he", "אינדונזיה"),
        ("hi", "इ\u{902}डोन\u{947}शिया"),
        ("hr", "Indonezija"),
        ("ht", "Endonezi"),
        ("hu", "Indonézia"),
        ("hy", "Ինդոնեզիա"),
        ("ia", "Indonesia"),
        ("id", "Indonesia"),
        ("io", "Indonezia"),
        ("is", "Indónesía"),
        ("it", "Indonesia"),
        ("iu", "ᐄᓅᓯᐊ"),
        ("ja", "インドネシア"),
        ("ka", "ინდონეზია"),
        ("ki", "Indonesia"),
        ("kk", "Индонезия"),
        ("kl", "Indonesia"),
        ("km", "ឥណ\u{17d2}ឌ\u{17bc}នេស\u{17ca}\u{17b8}"),
        ("kn", "ಇಂಡೋನೇಶ\u{cbf}ಯಾ"),
        ("ko", "인도네시아"),
        ("ku", "Endonezya"),
        ("kv", "Индонезия"),
        ("kw", "Indonesi"),
        ("ky", "Индонезия"),
        ("lo", "ປະເທດອ\u{eb4}ນໂດເນເຊຍ"),
        ("lt", "Indonezija"),
        ("lv", "Indonēzija"),
        ("mi", "Initonīhia"),
        ("mk", "Индонезија"),
        ("ml", "ഇന\u{d4d}തോനേഷ\u{d4d}യ"),
        ("mn", "Индонез"),
        ("mr", "इ\u{902}डोन\u{947}शिया"),
        ("ms", "Indonesia"),
        ("mt", "Indoneżja"),
        (
            "my",
            "အင\u{103a}ဒ\u{102d}\u{102f}န\u{102e}းရ\u{103e}ားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Indonitsiya"),
        ("nb", "Indonesia"),
        ("ne", "इन\u{94d}डोन\u{947}सिया"),
        ("nl", "Indonesië"),
        ("nn", "Indonesia"),
        ("nv", "Kéyah Dah Ndaaʼeełí Łání"),
        ("oc", "Indonesia"),
        ("or", "ଇଣ\u{b4d}ଡୋନେଶ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਇ\u{a70}ਡ\u{a4b}ਨ\u{a47}ਸ\u{a3c}ੀਆ"),
        ("pi", "इन\u{94d}दोन\u{947}शिया"),
        ("pl", "Indonezja"),
        ("ps", "اندونیزیا"),
        ("pt", "Indonésia"),
        ("pt_BR", "Indonésia"),
        ("ro", "Indonezia"),
        ("ru", "Индонезия"),
        ("rw", "Indonesiya"),
        ("sc", "Indonèsia"),
        ("sd", "انڊونيشيا"),
        ("si", "ඉන\u{dca}ද\u{dd4}න\u{dd3}ස\u{dd2}ය\u{dcf}ව"),
        ("sk", "Indonézia"),
        ("sl", "Indonezija"),
        ("so", "Indoneesiya"),
        ("sq", "Indonezi"),
        ("sr", "Индонезија"),
        ("sv", "Indonesien"),
        ("sw", "Indonesia"),
        ("ta", "இந\u{bcd}தோனேசிய\u{bbe}"),
        ("te", "ఇండ\u{c4b}న\u{c47}ష\u{c3f}య\u{c3e}"),
        ("tg", "Индонезия"),
        ("th", "อ\u{e34}นโดน\u{e35}เซ\u{e35}ย"),
        ("ti", "ኢንዶኔዢያ"),
        ("tk", "Indoneziýa"),
        ("tl", "Indonesia"),
        ("tr", "Endonezya"),
        ("tt", "İндонесиа"),
        ("ug", "ھىندونېزىيە"),
        ("uk", "Індонезія"),
        ("ur", "انڈونیشیا"),
        ("uz", "Indoneziya"),
        ("ve", "Indonesia"),
        ("vi", "Nam Dương"),
        ("wa", "Indonezeye"),
        ("wo", "Indoneesi"),
        ("xh", "Indonesia"),
        ("yo", "Indonésíà"),
        ("zh_CN", "印度尼西亚"),
        ("zh_HK", "印尼"),
        ("zh_TW", "印度尼西亞"),
        ("zu", "I-Indonesia"),
    ];
    #[cfg(all(feature = "id", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -0.789275;
        pub const LONGITUDE: f64 = 113.921327;
        pub const MAX_LATITUDE: f64 = 6.216999899999999;
        pub const MAX_LONGITUDE: f64 = 141.0425;
        pub const MIN_LATITUDE: f64 = -11.1082999;
        pub const MIN_LONGITUDE: f64 = 94.7351;
        pub const NORTHEAST_LATITUDE: f64 = 6.216999899999999;
        pub const NORTHEAST_LONGITUDE: f64 = 141.0425;
        pub const SOUTHWEST_LATITUDE: f64 = -11.1082999;
        pub const SOUTHWEST_LONGITUDE: f64 = 94.7351;
    }
}
#[cfg(all(feature = "id", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -0.789275,
            longitude: 113.921327,
            max_latitude: 6.216999899999999,
            max_longitude: 141.0425,
            min_latitude: -11.1082999,
            min_longitude: 94.7351,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 6.216999899999999,
                    longitude: 141.0425,
                },
                southwest: CountryGeoBound {
                    latitude: -11.1082999,
                    longitude: 94.7351,
                },
            },
        }
    }
}

#[cfg(all(feature = "id", feature = "subdivisions"))]
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
                    "AC",
                    Subdivision{
                        name: "AC",
                        country_alpha2: Alpha2::ID,
                        code: "AC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.131231499999999), longitude: Some(106.7329234), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aceh"), ("ar", "آتشيه"), ("be", "Правінцыя Ачэх"), ("bg", "Ачех"), ("bn", "আচেহ"), ("ca", "Aceh"), ("ccp", "𑄃𑄌𑄬𑄦\u{11134}"), ("ceb", "Nanggroe Aceh Darussalam Province"), ("cs", "Aceh"), ("cy", "Aceh"), ("da", "Aceh"), ("de", "Aceh"), ("el", "Άτσεχ"), ("en", "Aceh"), ("es", "Aceh"), ("et", "Aceh"), ("eu", "Aceh"), ("fa", "آچه"), ("fi", "Aceh"), ("fr", "Aceh"), ("ga", "Aceh"), ("gl", "Aceh"), ("gu", "અસ\u{ac7}હ"), ("he", "אצ׳ה"), ("hi", "आच\u{947}"), ("hr", "Aceh"), ("hu", "Aceh"), ("id", "Aceh"), ("is", "Aceh"), ("it", "Aceh"), ("ja", "アチェ州"), ("jv", "Acèh"), ("ka", "აჩეჰი"), ("kn", "ಆಚೇ"), ("ko", "아체 주"), ("lt", "Ačehas"), ("lv", "Ačeha"), ("mk", "Ачех"), ("ml", "അക\u{d4d}കെ"), ("mn", "Аче"), ("mr", "आच\u{947}"), ("ms", "Aceh"), ("nb", "Aceh"), ("ne", "अाच\u{947}"), ("nl", "Atjeh"), ("no", "Aceh"), ("pl", "Aceh"), ("pt", "Achém"), ("ro", "Aceh"), ("ru", "Ачех"), ("si", "අකෙහ\u{dca}"), ("sk", "Aceh"), ("sl", "Aceh"), ("sq", "Aqeh"), ("sr", "Аћех"), ("sr_Latn", "Aćeh"), ("sv", "Aceh"), ("sw", "Aceh"), ("ta", "அச\u{bcd}சே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఏస\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดอาเจะฮ\u{e4c}"), ("tr", "Açe"), ("uk", "Ачех"), ("ur", "آچے"), ("uz", "Achex"), ("vi", "Aceh"), ("yo", "Aceh"), ("yo_BJ", "Aceh"), ("yue", "亞齊"), ("yue_Hans", "亚齐"), ("zh", "亞齊"), ("zu", "Aceh")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::ID,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.4095178), longitude: Some(115.188916), max_latitude: Some(-8.061681799999999), min_latitude: Some(-8.849261), max_longitude: Some(115.7115281), min_longitude: Some(114.431626)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bali"), ("ca", "Bali"), ("ccp", "𑄝𑄣\u{11128}"), ("ceb", "Provinsi Bali"), ("cs", "Bali"), ("cy", "Bali"), ("da", "Bali"), ("de", "Provinz Bali"), ("en", "Bali"), ("es", "provincia de Bali"), ("et", "Bali provints"), ("eu", "Bali"), ("fi", "Bali"), ("fr", "province de Bali"), ("ga", "Bali"), ("gl", "Bali"), ("hr", "Bali"), ("hu", "Bali"), ("id", "Bali"), ("is", "Bali"), ("it", "provincia di Bali"), ("ja", "バリ州"), ("jv", "Provinsi Bali"), ("ko", "발리주"), ("ms", "Bali"), ("nb", "Bali"), ("nl", "Bali"), ("no", "Bali"), ("pl", "prowincja Bali"), ("pt", "Bali"), ("ro", "Bali"), ("ru", "Бали"), ("sk", "Bali"), ("sl", "Bali"), ("sv", "Bali"), ("sw", "Bali"), ("uk", "Балі"), ("vi", "Bali"), ("zu", "Bali")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BB",
                    Subdivision{
                        name: "BB",
                        country_alpha2: Alpha2::ID,
                        code: "BB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.2392759), longitude: Some(116.8727314), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بانغكا - بليتونغ"), ("bg", "Бангка Белитунг"), ("bn", "ব\u{9be}ঙ\u{9cd}ক\u{9be} বেলিট\u{9c1}ং দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"), ("ca", "Bangka-Belitung"), ("ccp", "𑄝\u{11101}𑄇-𑄝𑄬𑄣\u{11128}𑄑\u{11101} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Kepulauan Bangka Belitung"), ("cs", "Bangka-Belitung"), ("da", "Bangka-Belitung"), ("de", "Bangka-Belitung"), ("el", "Μπάνγκα Μπελίτουνγκ"), ("en", "Bangka–Belitung Islands"), ("es", "Bangka-Belitung"), ("eu", "Bangka-Belitung"), ("fa", "جزایر بانگکا-بلیتونگ"), ("fi", "Bangka-Belitung"), ("fr", "Îles Bangka Belitung"), ("gu", "બા\u{a82}ગ\u{acd}કા બ\u{ac7}લિટ\u{ac1}\u{a82}ગ આઇલ\u{ac7}ન\u{acd}ડ\u{acd}સ"), ("he", "באנגקה-בליטונג"), ("hi", "बा\u{902}का-ब\u{947}लित\u{941}\u{902}ग द\u{94d}वीपसम\u{942}ह"), ("hu", "Bangka-Belitung"), ("id", "Kepulauan Bangka Belitung"), ("it", "Bangka-Belitung"), ("ja", "バンカ・ブリトゥン州"), ("jv", "Kapuloan Bangka-Belitung"), ("ka", "ბანკა-ბელიტუნგი"), ("kn", "ಬ\u{ccd}ಯಾಂಗ\u{ccd}ಕಾ ಬ\u{cc6}ಲ\u{cbf}ಟುಂಗ\u{ccd} ದ\u{ccd}ವೀಪಗಳು"), ("ko", "방카블리퉁 제도"), ("lt", "Bankos-Belitungo salos"), ("lv", "Banka-Belituna"), ("ml", "ബങ\u{d4d}ക ബലിറ\u{d4d}റങ\u{d4d} ദ\u{d4d}വീപ\u{d41}കൾ"), ("mr", "बा\u{902}का-ब\u{947}लित\u{941}\u{902}ग द\u{94d}वीपसम\u{942}ह"), ("ms", "Kepulauan Bangka Belitung"), ("nb", "Bangka-Belitung"), ("nl", "Banka-Billiton"), ("no", "Bangka-Belitung"), ("pl", "Wyspy Bangka i Belitung"), ("pt", "Bangka-Belitung"), ("ru", "Банка-Белитунг"), ("si", "බන\u{dca}ග\u{dca}ක\u{dcf} බෙල\u{dd2}ට\u{dd4}න\u{dca}ග\u{dca} ද\u{dd4}පත\u{dca}"), ("sr", "Бангка Белитунг"), ("sr_Latn", "Bangka Belitung"), ("sv", "Bangka-Belitung"), ("ta", "பங\u{bcd}க\u{bbe}-பெலித\u{bcd}துங\u{bcd} த\u{bc0}வுகள\u{bcd}"), ("te", "బ\u{c3e}ంగ\u{c4d}క\u{c3e}-బ\u{c46}ల\u{c3f}టంగ\u{c4d} ద\u{c40}వులు"), ("th", "หม\u{e39}\u{e48}เกาะบ\u{e31}งกาเบล\u{e35}ต\u{e38}ง"), ("tr", "Bangka-Belitung"), ("uk", "Банка-Белітунг"), ("ur", "بانگکا بیلیٹنگ"), ("uz", "Bangka-Belitung"), ("vi", "Quần đảo Bangka-Belitung"), ("yue", "邦加-勿里洞"), ("yue_Hans", "邦加-勿里洞"), ("zh", "邦加-勿里洞省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::ID,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بنغكولو"), ("be", "Правінцыя Бенгкулу"), ("bg", "Бенгкулу"), ("bn", "বেংক\u{9c1}ল\u{9c1} প\u{9cd}রদেশ"), ("ca", "Bengkulu"), ("ccp", "𑄝𑄬\u{11101}𑄇\u{1112a}𑄣\u{1112a}"), ("ceb", "Propinsi Bengkulu"), ("cs", "Bengkulu"), ("cy", "Bengkulu"), ("da", "Bengkulu"), ("de", "Bengkulu"), ("el", "Μπενγκούλου"), ("en", "Bengkulu"), ("es", "Bengkulu"), ("eu", "Bengkulu"), ("fa", "بنگکولو"), ("fi", "Bengkulu"), ("fr", "Bengkulu (province)"), ("gu", "બ\u{ac7}ન\u{acd}ગક\u{ac1}લ\u{ac1} પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}\u{902}क\u{941}ल\u{942}"), ("hu", "Bengkulu"), ("id", "Bengkulu"), ("it", "Bengkulu"), ("ja", "ブンクル州"), ("jv", "Bengkulu"), ("ka", "ბენკულუ"), ("kn", "ಬ\u{cc6}ಂಕುಲು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "븡쿨루 주"), ("lt", "Benkulus"), ("lv", "Benkulu"), ("ml", "ബെങ\u{d4d}ക\u{d41}ള\u{d41}"), ("mr", "ब\u{947}\u{902}क\u{941}ल\u{942}"), ("ms", "Bengkulu"), ("nb", "Bengkulu"), ("nl", "Bengkulu"), ("no", "Bengkulu"), ("pl", "Bengkulu"), ("pt", "Bengkulu"), ("ro", "Bengkulu"), ("ru", "Бенкулу"), ("si", "බෙන\u{dca}ක\u{dd4}ල\u{dd4} පළ\u{dcf}ත"), ("sr", "Бенгкулу"), ("sr_Latn", "Bengkulu"), ("sv", "Bengkulu"), ("ta", "பெங\u{bcd}குளு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}ంగ\u{c4d}\u{200c}కులు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบ\u{e34}งก\u{e39}ล\u{e39}"), ("tr", "Bengkulu"), ("uk", "Бенгкулу (провінція)"), ("ur", "بنگکولو"), ("vi", "Bengkulu"), ("yue", "明古魯"), ("yue_Hans", "明古鲁"), ("zh", "明古魯省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BT",
                    Subdivision{
                        name: "BT",
                        country_alpha2: Alpha2::ID,
                        code: "BT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.789275), longitude: Some(113.921327), max_latitude: Some(6.216999899999999), min_latitude: Some(-11.1082999), max_longitude: Some(141.0425), min_longitude: Some(94.7351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Banten"), ("ar", "بنتن"), ("be", "Правінцыя Бантэн"), ("bg", "Бантен"), ("bn", "ব\u{9cd}য\u{9be}ন\u{9cd}টেন প\u{9cd}রদেশ"), ("ca", "Banten"), ("ccp", "𑄝𑄚\u{11134}𑄑𑄬𑄚\u{11134}"), ("ceb", "Banten (lalawigan sa Indonesia)"), ("cs", "Banten"), ("cy", "Banten"), ("da", "Banten"), ("de", "Banten"), ("el", "Μπάντεν"), ("en", "Banten"), ("es", "Bantén"), ("eu", "Banten"), ("fa", "بانتن"), ("fi", "Banten"), ("fr", "Banten"), ("gu", "બ\u{ac5}ન\u{acd}ટ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "באנטן"), ("hi", "बा\u{902}त\u{947}न"), ("hy", "Բանտեն"), ("id", "Banten"), ("it", "Banten"), ("ja", "バンテン州"), ("jv", "Banten"), ("ka", "ბანტენი"), ("kn", "ಬಂಟೇನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "반텐 주"), ("lt", "Bantenas"), ("lv", "Bantena"), ("ml", "ബ\u{d3e}ൻറൻ"), ("mn", "Бантен"), ("mr", "बा\u{902}त\u{947}न"), ("ms", "Banten"), ("nb", "Banten"), ("nl", "Bantam"), ("no", "Banten"), ("pl", "Banten"), ("pt", "Banten"), ("ru", "Бантен"), ("si", "බන\u{dca}ටෙන\u{dca} පළ\u{dcf}ත"), ("sq", "Banten"), ("sr", "Бантен"), ("sr_Latn", "Banten"), ("sv", "Banten"), ("ta", "பந\u{bcd}தன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ంట\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}นเต\u{e34}น"), ("tr", "Banten"), ("uk", "Бантен"), ("ur", "بانٹین"), ("vi", "Banten"), ("yue", "萬丹"), ("yue_Hans", "万丹"), ("zh", "万丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GO",
                    Subdivision{
                        name: "GO",
                        country_alpha2: Alpha2::ID,
                        code: "GO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.789275), longitude: Some(113.921327), max_latitude: Some(6.216999899999999), min_latitude: Some(-11.1082999), max_longitude: Some(141.0425), min_longitude: Some(94.7351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غورونتالو"), ("be", "Правінцыя Гарантала"), ("bg", "Горонтало"), ("bn", "গোরন\u{9cd}ত\u{9be}লো প\u{9cd}রদেশ"), ("ca", "Gorontalo"), ("ccp", "𑄉\u{1112e}𑄢\u{1112e}𑄚\u{11134}𑄑𑄣\u{1112e}"), ("ceb", "Provinsi Gorontalo"), ("cs", "Gorontalo"), ("da", "Gorontalo Province"), ("de", "Gorontalo"), ("el", "Γκοροντάλο"), ("en", "Gorontalo"), ("es", "Gorontalo"), ("eu", "Gorontalo"), ("fa", "گورونتالو"), ("fi", "Gorontalo"), ("fr", "Gorontalo"), ("gu", "ગોરોન\u{acd}ટાલો પ\u{acd}રા\u{a82}ત"), ("hi", "गोरो\u{902}तालो"), ("id", "Gorontalo"), ("it", "Provincia di Gorontalo"), ("ja", "ゴロンタロ州"), ("jv", "Gorontalo"), ("ka", "გორონტალო"), ("kn", "ಗೊರಾಂಟೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "고론탈로 주"), ("lt", "Gorontalas"), ("lv", "Gorontalo province"), ("mk", "Горонтало"), ("ml", "ഗൊറ\u{d3e}ന\u{d4d}റ\u{d3e}ലോ"), ("mr", "गोरोन\u{94d}गस\u{94d}टो प\u{94d}रा\u{902}त"), ("ms", "Gorontalo"), ("nb", "Gorontalo"), ("nl", "Gorontalo"), ("no", "Gorontalo"), ("pl", "Gorontalo"), ("pt", "Gorontalo"), ("ru", "Горонтало"), ("si", "ගොරෝන\u{dca}ටලෝ පළ\u{dcf}ත"), ("sr", "Горонтало"), ("sr_Latn", "Gorontalo"), ("sv", "Gorontalo"), ("ta", "கொரொந\u{bcd}த\u{bbe}ளோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4b}ర\u{c4a}ంట\u{c3e}ల\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกรอนทาโล"), ("tr", "Gorontalo"), ("uk", "Горонтало"), ("ur", "گورونٹالو (صوبہ)"), ("vi", "Gorontalo"), ("yue", "哥倫打洛"), ("yue_Hans", "哥伦打洛"), ("zh", "哥伦打洛省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::ID,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.4851831), longitude: Some(102.4380581), max_latitude: Some(-0.7357379), min_latitude: Some(-2.7700678), max_longitude: Some(104.4574329), min_longitude: Some(101.1286029)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جمبي"), ("bg", "Джамби"), ("bn", "জ\u{9be}ম\u{9cd}বি প\u{9cd}রদেশে"), ("ca", "Jambi"), ("ccp", "𑄎𑄟\u{11134}𑄝\u{11128}"), ("ceb", "Provinsi Jambi"), ("cs", "Jambi"), ("cy", "Jambi"), ("da", "Jambi"), ("de", "Jambi"), ("el", "Τζάμπι"), ("en", "Jambi"), ("es", "Jambi"), ("eu", "Jambi"), ("fa", "جامبی"), ("fi", "Jambi"), ("fr", "Jambi"), ("gu", "જા\u{a82}બી પ\u{acd}રા\u{a82}ત"), ("hi", "जा\u{902}बी"), ("id", "Jambi"), ("it", "Jambi"), ("ja", "ジャンビ州"), ("jv", "Jambi"), ("ka", "ჯამბი"), ("kn", "ಜಂಬ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "잠비 주"), ("lt", "Džambis"), ("lv", "Džambi province"), ("ml", "ജംബി പട\u{d4d}ടണം"), ("mn", "Жамби"), ("mr", "जा\u{902}बी"), ("ms", "Jambi"), ("nb", "Jambi"), ("nl", "Jambi"), ("no", "Jambi"), ("pl", "Jambi"), ("pt", "Jambi"), ("ro", "Jambi"), ("ru", "Джамби"), ("si", "ජම\u{dca}බ\u{dd2} පළ\u{dcf}ත"), ("sr", "Џамби"), ("sr_Latn", "Džambi"), ("sv", "Jambi"), ("ta", "ஜம\u{bcd}பி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c3e}ంబ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดจ\u{e31}มบ\u{e35}"), ("tr", "Jambi"), ("uk", "Джамбі"), ("ur", "جمبی"), ("vi", "Jambi"), ("yue", "占碑"), ("yue_Hans", "占碑"), ("zh", "占碑省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JB",
                    Subdivision{
                        name: "JB",
                        country_alpha2: Alpha2::ID,
                        code: "JB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.446736199999999), longitude: Some(109.2390841), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جاوة الغربية"), ("az", "Qərbi Yava"), ("bg", "Западна Ява"), ("bn", "পশ\u{9cd}চিম জ\u{9be}ভ\u{9be} প\u{9cd}রদেশ"), ("ca", "Java Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄎𑄞"), ("ceb", "Jawa Barat"), ("cs", "Západní Jáva"), ("cy", "Gorllewin Jawa"), ("da", "Vestjava"), ("de", "Jawa Barat"), ("el", "Γουέστ Τζάβα"), ("en", "West Java"), ("es", "Java Occidental"), ("eu", "Mendebaldeko Java"), ("fa", "جاوه غربی"), ("fi", "Länsi-Jaava"), ("fr", "Java occidental"), ("gu", "પશ\u{acd}ચિમ જાવા પ\u{acd}રા\u{a82}ત"), ("he", "מערב ג׳אווה"), ("hi", "पश\u{94d}चिम जावा"), ("id", "Jawa Barat"), ("it", "Giava Occidentale"), ("ja", "西ジャワ州"), ("jv", "Jawa Wetan"), ("ka", "დასავლეთი იავა"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಜಾವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "자와바랏 주"), ("lt", "Vakarų Java"), ("lv", "Rietumjava"), ("ml", "പടിഞ\u{d4d}ഞ\u{d3e}റൻ ജ\u{d3e}വ"), ("mn", "Өрнө Жава"), ("mr", "पश\u{94d}चिम जावा"), ("ms", "Jawa Barat"), ("nb", "Jawa Barat"), ("nl", "West-Java"), ("no", "Jawa Barat"), ("pl", "Jawa Zachodnia"), ("pt", "Java Ocidental"), ("ro", "Provincia Java de Vest"), ("ru", "Западная Ява"), ("si", "බටහ\u{dd2}ර ජ\u{dcf}ව\u{dcf} පළ\u{dcf}ත"), ("sr", "Западна Јава"), ("sr_Latn", "Zapadna Java"), ("sv", "Jawa Barat"), ("ta", "மேற\u{bcd}குச\u{bcd} ச\u{bbe}வக ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ జ\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชวาตะว\u{e31}นตก"), ("tr", "Batı Cava"), ("uk", "Західна Ява"), ("ur", "مغربی جاوا"), ("vi", "Tây Java"), ("yue", "西爪哇"), ("yue_Hans", "西爪哇"), ("zh", "西爪哇省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JI",
                    Subdivision{
                        name: "JI",
                        country_alpha2: Alpha2::ID,
                        code: "JI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.5360639), longitude: Some(112.2384017), max_latitude: Some(-5.048857), min_latitude: Some(-8.780560999999999), max_longitude: Some(116.267761), min_longitude: Some(110.8987199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جاوة الشرقية"), ("bg", "Източна Ява"), ("bn", "প\u{9c2}র\u{9cd}ব জ\u{9be}ভ\u{9be}"), ("ca", "Java Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄎𑄞"), ("ceb", "Jawa Timur"), ("cs", "Východní Jáva"), ("cy", "Dwyrain Jawa"), ("da", "Østjava"), ("de", "Jawa Timur"), ("el", "Ιστ Τζάβα"), ("en", "East Java"), ("es", "Java Oriental"), ("eu", "Ekialdeko Java"), ("fa", "جاوه شرقی"), ("fi", "Itä-Jaava"), ("fr", "Java oriental"), ("gu", "ઇસ\u{acd}ટ જાવા"), ("he", "מזרח ג׳אווה"), ("hi", "प\u{942}र\u{94d}व जावा"), ("id", "Jawa Timur"), ("it", "Giava Orientale"), ("ja", "東ジャワ州"), ("jv", "Jawa Wétan"), ("ka", "აღმოსავლეთი იავა"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಜಾವಾ"), ("ko", "자와티무르 주"), ("lt", "Rytų Java"), ("lv", "Austrumjava"), ("ml", "കിഴക\u{d4d}കൻ ജ\u{d3e}വ"), ("mn", "Дорно Жава"), ("mr", "प\u{942}र\u{94d}व जावा"), ("ms", "Jawa Timur"), ("nb", "Jawa Timur"), ("ne", "प\u{942}र\u{94d}वी जाभा क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Oost-Java"), ("no", "Jawa Timur"), ("pl", "Jawa Wschodnia"), ("pt", "Java Oriental"), ("ro", "Provincia Java de Est"), ("ru", "Восточная Ява"), ("si", "නැගෙනහ\u{dd2}ර ජ\u{dcf}ව\u{dcf}"), ("sl", "Vzhodna Java"), ("sr", "Источна Јава"), ("sr_Latn", "Istočna Java"), ("sv", "Jawa Timur"), ("ta", "கிழக\u{bcd}குச\u{bcd} ச\u{bbe}வக ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు జ\u{c3e}వ\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดชวาตะว\u{e31}นออก"), ("tr", "Doğu Cava"), ("uk", "Східна Ява"), ("ur", "مشرقی جاوا"), ("vi", "Đông Java"), ("yue", "東爪哇"), ("yue_Hans", "东爪哇"), ("zh", "东爪哇省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JK",
                    Subdivision{
                        name: "JK",
                        country_alpha2: Alpha2::ID,
                        code: "JK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.180495), longitude: Some(106.8283415), max_latitude: Some(-5.1843219), min_latitude: Some(-6.3708331), max_longitude: Some(106.972825), min_longitude: Some(106.3831259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Djakarta"), ("am", "ጃካርታ"), ("ar", "جاكرتا"), ("as", "জ\u{9be}ক\u{9be}ৰ\u{9cd}ট\u{9be}"), ("az", "Cakarta"), ("be", "Горад Джакарта"), ("bg", "Джакарта"), ("bn", "জ\u{9be}ক\u{9be}র\u{9cd}ত\u{9be}"), ("bs", "Džakarta"), ("ca", "Jakarta"), ("ccp", "𑄎𑄇𑄢\u{11134}𑄖"), ("ceb", "Daerah Khusus Ibukota Jakarta"), ("cs", "Jakarta"), ("cy", "Jakarta"), ("da", "Jakarta"), ("de", "Jakarta"), ("el", "Τζακάρτα"), ("en", "Jakarta"), ("es", "Yakarta"), ("et", "Jakarta"), ("eu", "Jakarta"), ("fa", "جاکارتا"), ("fi", "Jakarta"), ("fr", "Jakarta"), ("ga", "Iacárta"), ("gl", "Iacarta"), ("gu", "જાકાર\u{acd}તા"), ("ha", "Jakarta"), ("ha_NE", "Jakarta"), ("he", "ג׳קרטה"), ("hi", "जकार\u{94d}ता"), ("hr", "Jakarta"), ("hu", "Jakarta"), ("hy", "Ջակարտա"), ("id", "Daerah Khusus Ibukota Jakarta"), ("is", "Djakarta"), ("it", "Giacarta"), ("ja", "ジャカルタ"), ("jv", "Jakarta"), ("ka", "ჯაკარტა"), ("kk", "Джакарта"), ("km", "ចាការតា"), ("kn", "ಜಕಾರ\u{ccd}ತ"), ("ko", "자카르타"), ("ky", "Жакарта"), ("lt", "Džakarta"), ("lv", "Džakarta"), ("mk", "Џакарта"), ("ml", "ജക\u{d4d}ക\u{d3e}ർത\u{d4d}ത"), ("mn", "Жакарта"), ("mr", "जकार\u{94d}ता"), ("ms", "Jakarta"), ("my", "ဂျကာတာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Jakarta"), ("ne", "जाकार\u{94d}ता"), ("nl", "Jakarta"), ("no", "Jakarta"), ("or", "ଜ\u{b3e}କର\u{b4d}ତ\u{b3e}"), ("pa", "ਜਕਾਰਤਾ"), ("pl", "Dżakarta"), ("ps", "جکارتا"), ("pt", "Jacarta"), ("ro", "Jakarta"), ("ru", "Джакарта"), ("si", "ජක\u{dcf}ර\u{dca}ත\u{dcf}"), ("sk", "Jakarta"), ("sl", "Džakarta"), ("sq", "Xhakarta"), ("sr", "Џакарта"), ("sr_Latn", "Džakarta"), ("sv", "Jakarta"), ("sw", "Jakarta"), ("ta", "ஜக\u{bbe}ர\u{bcd}த\u{bcd}த\u{bbe}"), ("te", "జక\u{c3e}ర\u{c4d}త\u{c3e}"), ("th", "จาการ\u{e4c}ตา"), ("tk", "Jakarta"), ("tr", "Cakarta"), ("uk", "Джакарта"), ("ur", "جکارتہ"), ("uz", "Jakarta"), ("vi", "Jakarta"), ("yo", "Jakarta"), ("yo_BJ", "Jakarta"), ("yue", "耶加達"), ("yue_Hans", "耶加达"), ("zh", "雅加达")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JT",
                    Subdivision{
                        name: "JT",
                        country_alpha2: Alpha2::ID,
                        code: "JT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.150975), longitude: Some(110.1402594), max_latitude: Some(-5.725698), min_latitude: Some(-8.2116361), max_longitude: Some(111.6914889), min_longitude: Some(108.555502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jawa Tengah"), ("ar", "جاوة الوسطى"), ("be", "Цэнтральная Ява"), ("bg", "Централна Ява"), ("bn", "কেন\u{9cd}দ\u{9cd}রীয\u{9bc} জ\u{9be}ভ\u{9be} প\u{9cd}রদেশ"), ("ca", "Java Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄎𑄞"), ("ceb", "Provinsi Jawa Tengah"), ("cs", "Střední Jáva"), ("cy", "Canolbarth Jawa"), ("da", "Centraljava"), ("de", "Jawa Tengah"), ("el", "Σέντραλ Τζάβα"), ("en", "Central Java"), ("es", "Java Central"), ("et", "Jawa Tengah"), ("eu", "Jawa Tengah"), ("fa", "جاوه مرکزی"), ("fi", "Keski-Jaava"), ("fr", "Java central"), ("ga", "Jawa Tengah"), ("gl", "Jawa Tengah"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ જાવા પ\u{acd}રા\u{a82}ત"), ("he", "יאוה המרכזית"), ("hi", "मध\u{94d}य जावा"), ("hr", "Jawa Tengah"), ("hu", "Jawa Tengah"), ("id", "Jawa Tengah"), ("is", "Jawa Tengah"), ("it", "Giava Centrale"), ("ja", "中部ジャワ州"), ("jv", "Jawa Tengah"), ("ka", "ცენტრალური იავა"), ("kn", "ಮಧ\u{ccd}ಯ ಜಾವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "자와틍아 주"), ("lt", "Centrinė Java"), ("lv", "Centrālās Javas province"), ("ml", "മദ\u{d4d}ധ\u{d4d}യ ജ\u{d3e}വ"), ("mn", "Төв Жава"), ("mr", "मध\u{94d}य जावा"), ("ms", "Jawa Tengah"), ("nb", "Jawa Tengah"), ("nl", "Midden-Java"), ("no", "Jawa Tengah"), ("pl", "Jawa Środkowa"), ("pt", "Java Central"), ("ro", "Provincia Java Centrală"), ("ru", "Центральная Ява"), ("si", "මධ\u{dca}\u{200d}යම ජ\u{dcf}ව\u{dcf} පළ\u{dcf}ත"), ("sk", "Jawa Tengah"), ("sl", "Jawa Tengah"), ("sr", "Централна Јава"), ("sr_Latn", "Centralna Java"), ("sv", "Jawa Tengah"), ("sw", "Jawa Tengah"), ("ta", "நடுச\u{bcd} ச\u{bbe}வக ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} జ\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชวากลาง"), ("tr", "Orta Cava"), ("uk", "Центральна Ява"), ("ur", "وسطی جاوا"), ("vi", "Trung Java"), ("yue", "中爪哇"), ("yue_Hans", "中爪哇"), ("zh", "中爪哇省"), ("zu", "Jawa Tengah")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JW",
                    Subdivision{
                        name: "JW",
                        country_alpha2: Alpha2::ID,
                        code: "JW",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Java"), ("am", "ጃዋ"), ("ar", "جاوة"), ("az", "Yava"), ("be", "Востраў Ява"), ("bg", "Ява"), ("bn", "জ\u{9be}ভ\u{9be} দ\u{9cd}বীপ"), ("bs", "Java"), ("ca", "Java"), ("ccp", "𑄎𑄞"), ("ceb", "Java"), ("cs", "Jáva"), ("cy", "Jawa"), ("da", "Java"), ("de", "Java"), ("el", "Ιάβα"), ("en", "Java"), ("es", "Java"), ("et", "Jaava"), ("eu", "Java"), ("fa", "جاوه"), ("fi", "Jaava"), ("fr", "île de Java"), ("ga", "Iáva"), ("gl", "Xava"), ("gu", "જાવા"), ("ha", "Java"), ("ha_NE", "Java"), ("he", "ג׳אווה"), ("hi", "जावा"), ("hr", "Java"), ("hu", "Jáva"), ("hy", "Ճավա"), ("id", "Jawa"), ("is", "Java"), ("it", "Giava"), ("ja", "ジャワ島"), ("jv", "Jawa"), ("ka", "იავა"), ("kk", "Ява"), ("km", "កោះជ\u{17d2}វា"), ("kn", "ಜಾವಾ"), ("ko", "자와 섬"), ("ky", "Ява"), ("lt", "Java"), ("lv", "Java"), ("mk", "Јава"), ("ml", "ജ\u{d3e}വ (ദ\u{d4d}വീപ\u{d4d})"), ("mn", "Жава"), ("mr", "जावा"), ("ms", "Jawa"), ("my", "ဂျားဗားကျ\u{103d}န\u{103a}း"), ("nb", "Java"), ("ne", "जावा द\u{94d}वीप"), ("nl", "Java"), ("no", "Java"), ("or", "ଜ\u{b3e}ଭ\u{b3e}"), ("pa", "ਜਾਵਾ ਟਾਪ\u{a42}"), ("pl", "Jawa"), ("pt", "Java"), ("ro", "Insula Java"), ("ru", "Ява"), ("si", "ජ\u{dcf}ව\u{dcf}"), ("sk", "Jáva"), ("sl", "Java"), ("sr", "Јава"), ("sr_Latn", "Java"), ("sv", "Java"), ("sw", "Java"), ("ta", "ச\u{bbe}வகம\u{bcd}"), ("te", "జ\u{c3e}వ\u{c3e}"), ("th", "เกาะชวา"), ("tr", "Cava Adası"), ("uk", "Ява"), ("ur", "جاوا"), ("uz", "Yava"), ("vi", "Java"), ("yue", "爪哇"), ("yue_Hans", "爪哇"), ("zh", "爪哇岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "KA",
                        country_alpha2: Alpha2::ID,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.9790674), longitude: Some(112.6358005), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كليمنتان"), ("be", "Інданезійскі Калімантан"), ("bg", "Калимантан"), ("bn", "ক\u{9be}লিম\u{9be}ন\u{9cd}ত\u{9be}ন"), ("ca", "Kalimantan"), ("ccp", "𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("da", "Kalimantan"), ("de", "Kalimantan"), ("en", "Kalimantan"), ("es", "Kalimantan"), ("eu", "Kalimantan"), ("fa", "کالیمانتان"), ("fi", "Kalimantan"), ("fr", "Kalimantan"), ("he", "קלימנטן"), ("hi", "कालिम\u{902}तान"), ("id", "Kalimantan"), ("it", "Kalimantan"), ("ja", "カリマンタン"), ("km", "កាល\u{17b8}ម\u{17c9}ាន\u{17cb}តាន\u{17cb}"), ("ko", "칼리만탄"), ("lt", "Kalimantanas"), ("mk", "Калимантан"), ("ml", "കലിമന\u{d4d}ത\u{d3e}ൻ\u{200c}"), ("ms", "Kalimantan"), ("nb", "Kalimantan"), ("nl", "Kalimantan"), ("no", "Kalimantan"), ("pl", "Kalimantan"), ("pt", "Kalimantan"), ("ru", "Индонезийский Калимантан"), ("sv", "Kalimantan"), ("sw", "Kalimantan"), ("ta", "கலிமந\u{bcd}தன\u{bcd}"), ("tr", "Kalimantan"), ("uk", "Індонезійський Калімантан"), ("ur", "کالیمانتان"), ("uz", "Kalimantan"), ("vi", "Kalimantan"), ("yue", "婆羅洲"), ("yue_Hans", "婆罗洲"), ("zh", "加里曼丹")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KB",
                    Subdivision{
                        name: "KB",
                        country_alpha2: Alpha2::ID,
                        code: "KB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.2787808), longitude: Some(111.4752851), max_latitude: Some(2.0815391), min_latitude: Some(-3.0391839), max_longitude: Some(114.2022952), min_longitude: Some(108.020782)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كالمنتان الغربية"), ("bg", "Западен Калимантан"), ("bn", "ক\u{9be}ল\u{9be}িম\u{9be}নত\u{9be}ন ব\u{9be}র\u{9be}ত\u{9be}"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Provinsi Kalimantan Barat"), ("cs", "Západní Kalimantan"), ("da", "Kalimantan Barat"), ("de", "Kalimantan Barat"), ("el", "Γουέστ Καλιμάνταν"), ("en", "West Kalimantan"), ("es", "Borneo Occidental"), ("et", "Lääne-Kalimantan"), ("eu", "Mendebaldeko Kalimantan"), ("fa", "کالیمانتان غربی"), ("fi", "Länsi-Kalimantan"), ("fr", "Kalimantan occidental"), ("gu", "કાલિમ\u{a82}તાન બારાટ"), ("he", "מערב קלימנטן"), ("hi", "पश\u{94d}चिम कालिम\u{902}तान"), ("id", "Kalimantan Barat"), ("it", "Kalimantan Occidentale"), ("ja", "西カリマンタン州"), ("jv", "Kalimantan Kulon"), ("ka", "დასავლეთი კალიმანტანი"), ("kn", "ಕಲ\u{cbf}ಮಾಂತನ\u{ccd} ಬರಾಟ\u{ccd}"), ("ko", "칼리만탄바랏 주"), ("lt", "Vakarų Kalimantanas"), ("lv", "Rietumkalimantāna"), ("ml", "പശ\u{d4d}ചിമ കലിമന\u{d4d}ത\u{d3e}ൻ"), ("mn", "Өрнө Калимантан"), ("mr", "पश\u{94d}चिम कालिमा\u{902}तान"), ("ms", "Kalimantan Barat"), ("nb", "Kalimantan Barat"), ("nl", "West-Kalimantan"), ("no", "Kalimantan Barat"), ("pl", "Borneo Zachodnie"), ("pt", "Kalimantan Ocidental"), ("ru", "Западный Калимантан"), ("si", "කල\u{dd2}මන\u{dca}ටන\u{dca} බ\u{dcf}රට\u{dca}"), ("sr", "Западни Калимантан"), ("sr_Latn", "Zapadni Kalimantan"), ("sv", "Kalimantan Barat"), ("ta", "மேற\u{bcd}கு களிமந\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ల\u{c3f}మంటన\u{c4d} బ\u{c3e}ర\u{c3e}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาล\u{e35}ม\u{e31}นต\u{e31}นตะว\u{e31}นตก"), ("tr", "Batı Kalimantan"), ("uk", "Західний Калімантан"), ("ur", "مغربی کالیمانتان"), ("vi", "Tây Kalimantan"), ("yue", "西加里曼丹"), ("yue_Hans", "西加里曼丹"), ("zh", "西加里曼丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KI",
                    Subdivision{
                        name: "KI",
                        country_alpha2: Alpha2::ID,
                        code: "KI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.5386586), longitude: Some(116.419389), max_latitude: Some(2.6065559), min_latitude: Some(-2.409401), max_longitude: Some(119.025528), min_longitude: Some(113.8366543)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كالمنتان الشرقية"), ("be", "Правінцыя Усходні Калімантан"), ("bg", "Източен Калимантан"), ("bn", "কিলিম\u{9be}ন\u{9cd}ত\u{9be}ন তিম\u{9c1}র"), ("ca", "Kalimantan oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Provinsi Kalimantan Timur"), ("cs", "Východní Kalimantan"), ("da", "Kalimantan Timur"), ("de", "Kalimantan Timur"), ("el", "Ανατολικό Καλιμαντάν"), ("en", "East Kalimantan"), ("es", "Kalimantan Oriental"), ("eu", "Ekialdeko Kalimantan"), ("fa", "کالیمانتان شرقی"), ("fi", "Itä-Kalimantan"), ("fr", "Kalimantan oriental"), ("gu", "કાલિમ\u{a82}તન ત\u{ac8}મ\u{ac1}ર"), ("hi", "प\u{942}र\u{94d}व कालिम\u{902}तान"), ("id", "Kalimantan Timur"), ("it", "Kalimantan Orientale"), ("ja", "東カリマンタン州"), ("jv", "Kalimantan Wétan"), ("ka", "აღმოსავლეთი კალიმანტანი"), ("kn", "ಕಲ\u{cbf}ಮಾಂತನ\u{ccd} ತ\u{cbf}ಮುರ\u{ccd}"), ("ko", "칼리만탄티무르 주"), ("lt", "Rytų Kalimantanas"), ("lv", "Austrumu Kalimantāna"), ("ml", "കിഴക\u{d4d}കൻ കലിമന\u{d4d}ത\u{d3e}ൻ"), ("mr", "प\u{942}र\u{94d}व कालिमा\u{902}तान"), ("ms", "Kalimantan Timur"), ("my", "အရ\u{103e}ေ\u{1037}ကာလ\u{102e}မန\u{103a}တန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Kalimantan Timur"), ("nl", "Oost-Kalimantan"), ("no", "Kalimantan Timur"), ("pl", "Borneo Wschodnie"), ("pt", "Kalimantan Oriental"), ("ro", "Kalimantanul de Est"), ("ru", "Восточный Калимантан"), ("si", "කල\u{dd2}මන\u{dca}ටන\u{dca} ට\u{dd2}ම\u{dd4}ර\u{dca}"), ("sr", "Источни Калимантан"), ("sr_Latn", "Istočni Kalimantan"), ("sv", "Kalimantan Timur"), ("ta", "கிழக\u{bcd}குக\u{bcd} களிமந\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ల\u{c3f}మంటన\u{c4d} ట\u{c3f}మూర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาล\u{e35}ม\u{e31}นต\u{e31}นตะว\u{e31}นออก"), ("tr", "Doğu Kalimantan"), ("uk", "Східний Калімантан"), ("ur", "مشرقی کالیمانتان"), ("vi", "Đông Kalimantan"), ("yue", "東加里曼丹"), ("yue_Hans", "东加里曼丹"), ("zh", "東加里曼丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KR",
                    Subdivision{
                        name: "KR",
                        country_alpha2: Alpha2::ID,
                        code: "KR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.4429142), longitude: Some(107.0115729), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جزر رياو"), ("be", "Астравы Рыау"), ("bg", "Острови Риау"), ("bn", "কেপ\u{9c1}ল\u{9be}উয\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Illes Riau"), ("ccp", "𑄢\u{1112d}𑄅\u{1112a} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Provinsi Kepulauan Riau"), ("cs", "Ostrovy Riau"), ("cy", "Ynysoedd Riau"), ("da", "Kepulauan Riau"), ("de", "Kepulauan Riau"), ("el", "Νησιά Ριάου"), ("en", "Riau Islands"), ("es", "Islas Riau"), ("eu", "Riau uhartedi"), ("fa", "جزایر ریائو"), ("fi", "Riausaaret"), ("fr", "Îles Riau"), ("gu", "ક\u{ac7}પ\u{ac1}લઉઆન રીઉ"), ("he", "איי ריאו"), ("hi", "रियाउ द\u{94d}वीपसम\u{942}ह"), ("id", "Kepulauan Riau"), ("it", "Isole Riau"), ("ja", "リアウ諸島州"), ("jv", "Kapuloan Riau"), ("ka", "რიაუს კუნძულების პროვინცია"), ("kn", "ಕ\u{cc6}ಪುಲ\u{ccc}ನ\u{ccd} ರ\u{cbf}ಯು"), ("ko", "리아우 제도 주"), ("lt", "Riau salos"), ("lv", "Riau salas"), ("ml", "റിയ\u{d3e}വ\u{d41} ദ\u{d4d}വീപ\u{d41}കൾ"), ("mn", "Риау арлуудын муж"), ("mr", "रियाउ द\u{94d}वीपसम\u{942}ह"), ("ms", "Kepulauan Riau"), ("nb", "Kepulauan Riau"), ("nl", "Riouwarchipel"), ("no", "Kepulauan Riau"), ("pl", "Wyspy Riau"), ("pt", "Ilhas Riau"), ("ru", "Острова Риау"), ("si", "කෙප\u{dd4}ලඋආන\u{dca} ර\u{dd2}යව\u{dd6}"), ("sr", "Острва Ријау"), ("sr_Latn", "Ostrva Rijau"), ("sv", "Kepulauan Riau"), ("ta", "ரிய\u{bbe}வு த\u{bc0}வுகள\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}\u{c47}పుల\u{c3e}వన\u{c4d} ర\u{c3f}య\u{c3e}వు"), ("th", "หม\u{e39}\u{e48}เกาะเร\u{e35}ยว"), ("tr", "Riau Adaları"), ("uk", "Острови Ріау"), ("ur", "ریاو جزائر صوبہ"), ("vi", "Quần đảo Riau"), ("yue", "廖內群島省"), ("yue_Hans", "廖内群岛省"), ("zh", "廖内群岛省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KS",
                    Subdivision{
                        name: "KS",
                        country_alpha2: Alpha2::ID,
                        code: "KS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.0926415), longitude: Some(115.2837585), max_latitude: Some(-1.315037), min_latitude: Some(-4.744812899999999), max_longitude: Some(116.555961), min_longitude: Some(114.345859)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كليمنتان الجنوبية"), ("bg", "Южен Калимантан"), ("bn", "ক\u{9be}লিম\u{9be}ন\u{9cd}ট\u{9be}ন সেল\u{9be}ট\u{9be}ন"), ("ca", "Kalimantan del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Provinsi Kalimantan Selatan"), ("cs", "Jižní Kalimantan"), ("da", "Kalimantan Selatan"), ("de", "Kalimantan Selatan"), ("el", "Καλιμαντάν"), ("en", "South Kalimantan"), ("es", "Borneo Meridional"), ("eu", "Hego Kalimantan"), ("fa", "کالیمانتان جنوبی"), ("fi", "Etelä-Kalimantan"), ("fr", "Kalimantan du Sud"), ("gu", "કાલિમ\u{a82}ટન સ\u{ac7}લાટન"), ("hi", "दक\u{94d}षिण कालिम\u{902}तान"), ("id", "Kalimantan Selatan"), ("it", "Kalimantan Meridionale"), ("ja", "南カリマンタン州"), ("jv", "Kalimantan Kidul"), ("ka", "სამხრეთი კალიმანტანი"), ("kn", "ಕಲ\u{cbf}ಮಾಂತನ\u{ccd} ಸ\u{cc6}ಲಟಾನ\u{ccd}"), ("ko", "칼리만탄슬라탄 주"), ("lt", "Pietų Kalimantanas"), ("lv", "Dienvidkalimantāna"), ("ml", "ദക\u{d4d}ഷിണ കലിമന\u{d4d}ത\u{d3e}ൻ"), ("mn", "Өмнө Калимантан"), ("mr", "दक\u{94d}षिण कालिमा\u{902}तान"), ("ms", "Kalimantan Selatan"), ("nb", "Kalimantan Selatan"), ("nl", "Zuid-Kalimantan"), ("no", "Kalimantan Selatan"), ("pl", "Borneo Południowe"), ("pt", "Kalimantan do Sul"), ("ru", "Южный Калимантан"), ("si", "කල\u{dd2}මන\u{dca}ටන\u{dca} සෙලටන\u{dca}"), ("sr", "Јужни Калимантан"), ("sr_Latn", "Južni Kalimantan"), ("sv", "Kalimantan Selatan"), ("ta", "தெற\u{bcd}குக\u{bcd} களிமந\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ల\u{c3f}మంటన\u{c4d} స\u{c46}ల\u{c3e}టన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาล\u{e34}ม\u{e31}นต\u{e31}น"), ("tr", "Güney Kalimantan"), ("uk", "Південний Калімантан"), ("ur", "جنوبی کالیمانتان"), ("vi", "Nam Kalimantan"), ("yue", "南加里曼丹"), ("yue_Hans", "南加里曼丹"), ("zh", "南加里曼丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KT",
                    Subdivision{
                        name: "KT",
                        country_alpha2: Alpha2::ID,
                        code: "KT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.6814878), longitude: Some(113.3823545), max_latitude: Some(0.793256), min_latitude: Some(-3.5618859), max_longitude: Some(115.84722), min_longitude: Some(110.7336648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كالمنتان الوسطى"), ("bg", "Централен Калимантан"), ("bn", "ক\u{9be}লিম\u{9be}ন\u{9cd}টন টেঙ\u{9cd}গ\u{9be}হ"), ("ca", "Kalimantan central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Provinsi Kalimantan Tengah"), ("cs", "Střední Kalimantan"), ("da", "Centralkalimantan"), ("de", "Kalimantan Tengah"), ("el", "Κεντρικό Καλιμαντάν"), ("en", "Central Kalimantan"), ("es", "Borneo Central"), ("eu", "Erdialdeko Kalimantan"), ("fa", "کالیمانتان مرکزی"), ("fi", "Keski-Kalimantan"), ("fr", "Kalimantan central"), ("gu", "કાલીમ\u{a82}તાન ટ\u{ac7}ન\u{acd}ગાહ"), ("hi", "मध\u{94d}य कालिम\u{902}तान"), ("id", "Kalimantan Tengah"), ("it", "Kalimantan Centrale"), ("ja", "中部カリマンタン州"), ("jv", "Kalimantan Tengah"), ("ka", "ცენტრალური კალიმანტანი"), ("kn", "ಕಲ\u{cbf}ಮಾಂತನ\u{ccd} ತ\u{cc6}ಂಗಾಹ\u{ccd}"), ("ko", "칼리만탄틍아 주"), ("lt", "Centrinis Kalimantanas"), ("lv", "Centrālā Kalimantāna"), ("ml", "മദ\u{d4d}ധ\u{d4d}യ കലിമന\u{d4d}ത\u{d3e}ൻ"), ("mn", "Төв Калимантан"), ("mr", "मध\u{94d}य कालिमा\u{902}तान"), ("ms", "Kalimantan Tengah"), ("nb", "Kalimantan Tengah"), ("nl", "Midden-Kalimantan"), ("no", "Kalimantan Tengah"), ("pl", "Borneo Środkowe"), ("pt", "Kalimantan Central"), ("ru", "Центральный Калимантан"), ("si", "කල\u{dd2}මන\u{dca}ටන\u{dca} ටෙන\u{dca}ග\u{dcf}"), ("sk", "Stredný Kalimantan"), ("sr", "Централни Калимантан"), ("sr_Latn", "Centralni Kalimantan"), ("sv", "Kalimantan Tengah"), ("ta", "நடுக\u{bcd} களிமந\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ల\u{c3f}మంటన\u{c4d} ట\u{c46}ంగ\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดกาล\u{e35}ม\u{e31}นต\u{e31}นกลาง"), ("tr", "Orta Kalimantan"), ("uk", "Центральний Калімантан"), ("ur", "وسطی کالیمانتان"), ("vi", "Trung Kalimantan"), ("yue", "中加里曼丹"), ("yue_Hans", "中加里曼丹"), ("zh", "中加里曼丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KU",
                    Subdivision{
                        name: "KU",
                        country_alpha2: Alpha2::ID,
                        code: "KU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.1718272), longitude: Some(106.8440285), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كالمنتان الشمالية"), ("be", "Паўночны Калімантан"), ("ca", "Kalimantan Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄇𑄣\u{11128}𑄟𑄚\u{11134}𑄑𑄚\u{11134}"), ("ceb", "North Kalimantan"), ("cs", "Severní Kalimantan"), ("de", "Kalimantan Utara"), ("en", "North Kalimantan"), ("es", "Kalimantán Septentrional"), ("eu", "Ipar Kalimantan"), ("fa", "کالیمانتان شمالی"), ("fi", "Pohjois-Kalimantan"), ("fr", "Nord Kalimantan"), ("hi", "उत\u{94d}तर कालिम\u{902}तान"), ("id", "Kalimantan Utara"), ("it", "Kalimantan Utara"), ("ja", "北カリマンタン州"), ("jv", "Kalimantan Lor"), ("ko", "칼리만탄우타라 주"), ("mr", "उत\u{94d}तर कालिमा\u{902}तान"), ("ms", "Kalimantan Utara"), ("my", "မြောက\u{103a}ကာလ\u{102e}မန\u{103a}တန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nl", "Noord-Kalimantan"), ("pl", "Borneo Północne"), ("pt", "Kalimantan Setentrional"), ("ru", "Северный Калимантан"), ("sr", "Северни Калимантан"), ("sr_Latn", "Severni Kalimantan"), ("ta", "வடக\u{bcd}குக\u{bcd} களிமந\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("th", "จ\u{e31}งหว\u{e31}ดกาล\u{e35}ม\u{e31}นต\u{e31}นเหน\u{e37}อ"), ("tr", "Kuzey Kalimantan"), ("uk", "Північний Калімантан"), ("ur", "شمالی کالیمانتان"), ("vi", "Bắc Kalimantan"), ("yue", "北加里曼丹"), ("yue_Hans", "北加里曼丹"), ("zh", "北加里曼丹省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::ID,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.636759399999999), longitude: Some(115.2067686), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لامبونغ"), ("bg", "Лампунг"), ("bn", "ল\u{9cd}য\u{9be}ম\u{9cd}প\u{9c1}ং প\u{9cd}রদেশ"), ("ca", "Lampung"), ("ccp", "𑄣𑄟\u{11134}𑄛\u{1112a}\u{11101}"), ("ceb", "Provinsi Lampung"), ("cs", "Lampung"), ("cy", "Lampung"), ("da", "Lampung"), ("de", "Lampung"), ("el", "Λαμπούνγκ"), ("en", "Lampung"), ("es", "Lampung"), ("eu", "Lampung"), ("fa", "لامپونگ"), ("fi", "Lampung"), ("fr", "Lampung"), ("gu", "લ\u{ac7}મ\u{acd}પ\u{ac1}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "ला\u{902}प\u{941}\u{902}ग"), ("hy", "Լամպունգ"), ("id", "Lampung"), ("it", "Lampung"), ("ja", "ランプン州"), ("jv", "Lampung"), ("ka", "ლამპუნგი"), ("kn", "ಲ\u{ccd}ಯಾಂಪಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "람풍 주"), ("lt", "Lampungas"), ("lv", "Lampungas province"), ("mn", "Лампун"), ("mr", "ला\u{902}प\u{941}\u{902}ग"), ("ms", "Lampung"), ("nb", "Lampung"), ("ne", "ल\u{94d}याम\u{94d}प\u{941}ङ क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Lampung"), ("no", "Lampung"), ("pl", "Lampung"), ("pt", "Lampung"), ("ro", "Lampung"), ("ru", "Лампунг"), ("si", "ලැම\u{dca}පන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sq", "Lampung"), ("sr", "Лампунг"), ("sr_Latn", "Lampung"), ("sv", "Lampung"), ("ta", "இளம\u{bcd}புங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}ంపుంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลำปาง"), ("tr", "Lampung"), ("uk", "Лампунг"), ("ur", "لامپونگ"), ("vi", "Lampung"), ("yue", "楠榜"), ("yue_Hans", "楠榜"), ("zh", "楠榜省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::ID,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4072107), longitude: Some(-71.3824374), max_latitude: Some(42.88679), min_latitude: Some(41.18705300000001), max_longitude: Some(-69.85886099999999), min_longitude: Some(-73.5081419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مالوكو"), ("be", "Правінцыя Малуку"), ("bg", "Молуку"), ("bn", "ম\u{9be}ল\u{9be}ক\u{9c1} প\u{9cd}রদেশ"), ("ca", "Província de les Moluques"), ("ccp", "𑄟𑄣\u{1112a}𑄇\u{1112a}"), ("ceb", "Provinsi Maluku"), ("cs", "Moluky"), ("cy", "Maluku"), ("da", "Maluku Province"), ("de", "Maluku"), ("el", "Μαλούκου"), ("en", "Maluku"), ("es", "Molucas"), ("et", "Maluku provints"), ("eu", "Molukak"), ("fa", "ملوک"), ("fi", "Molukit"), ("fr", "Moluques"), ("gu", "માલ\u{ac1}ક\u{ac1} પ\u{acd}રા\u{a82}ત"), ("he", "מאלוקו"), ("hi", "माल\u{941}क\u{942}"), ("id", "Maluku"), ("it", "Provincia di Maluku"), ("ja", "マルク州"), ("jv", "Maluku"), ("ka", "მალუკუ"), ("kn", "ಮಲುಕು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "말루쿠 주"), ("lt", "Molukai"), ("lv", "Maluku province"), ("mk", "Малуку"), ("mr", "माल\u{941}क\u{942} प\u{94d}रा\u{902}त"), ("ms", "Maluku"), ("nb", "Maluku Kommune"), ("nl", "Maluku"), ("no", "Maluku Kommune"), ("pl", "Moluki"), ("pt", "Molucas (província)"), ("ru", "Малуку"), ("si", "මල\u{dd4}ක\u{dd4} පළ\u{dcf}ත"), ("sr", "Молуци"), ("sr_Latn", "Moluci"), ("sv", "Moluckerna"), ("ta", "மளுக\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}లుకు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาล\u{e39}ก\u{e39}"), ("tr", "Maluku"), ("uk", "Малуку"), ("ur", "مالوکو (صوبہ)"), ("vi", "Maluku"), ("yue", "摩鹿加"), ("yue_Hans", "摩鹿加"), ("zh", "马鲁古省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ML",
                    Subdivision{
                        name: "ML",
                        country_alpha2: Alpha2::ID,
                        code: "ML",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.789275), longitude: Some(113.921327), max_latitude: Some(6.216999899999999), min_latitude: Some(-11.1082999), max_longitude: Some(141.0425), min_longitude: Some(94.7351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Molukke"), ("ar", "جزر الملوك"), ("be", "Малукскія астравы"), ("bg", "Молукски острови"), ("bs", "Molučka ostrva"), ("ca", "Moluques"), ("ccp", "𑄟𑄣\u{1112a}𑄇\u{1112a} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Molucas"), ("cs", "Moluky²"), ("cy", "Maluku²"), ("da", "Molukkerne"), ("de", "Molukken"), ("el", "Μολούκες"), ("en", "Maluku Islands"), ("es", "Islas Molucas"), ("et", "Maluku saared"), ("eu", "Molukak²"), ("fa", "جزایر ملوک"), ("fi", "Molukit²"), ("fr", "Moluques²"), ("ga", "Na Molacaí"), ("gl", "Molucas"), ("he", "איי מאלוקו"), ("hi", "माल\u{941}क\u{942} द\u{94d}वीपसम\u{942}ह"), ("hr", "Molučki otoci"), ("hu", "Maluku-szigetek"), ("hy", "Մոլուքային կղզիներ"), ("id", "Kepulauan Maluku"), ("is", "Mólúkkaeyjar"), ("it", "Molucche"), ("ja", "モルッカ諸島"), ("jv", "Kapuloan Maluku"), ("ka", "მოლუკის კუნძულები"), ("kk", "Молукка аралдары"), ("ko", "말루쿠 제도"), ("ky", "Молукка аралдары"), ("lt", "Molukų salos"), ("lv", "Moluku salas"), ("mk", "Молучки Острови"), ("ml", "മല\u{d41}ക\u{d41} ദ\u{d4d}വീപ\u{d41}കൾ"), ("ms", "Kepulauan Maluku"), ("nb", "Molukkene"), ("nl", "Molukken"), ("no", "Molukkene"), ("pl", "Moluki²"), ("pt", "Ilhas Molucas"), ("ro", "Insulele Moluce"), ("ru", "Молуккские острова"), ("sk", "Moluky"), ("sr", "Молучка острва"), ("sr_Latn", "Molučka ostrva"), ("sv", "Moluckerna²"), ("ta", "மலுக\u{bcd}கு த\u{bc0}வுகள\u{bcd}"), ("th", "หม\u{e39}\u{e48}เกาะโมล\u{e38}กกะ"), ("tr", "Maluku Adaları"), ("uk", "Молуккські острови"), ("ur", "جزائر ملوک"), ("uz", "Molukka orollari"), ("vi", "Quần đảo Maluku"), ("yue", "摩鹿加群島"), ("yue_Hans", "摩鹿加群岛"), ("zh", "摩鹿加群岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::ID,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.5709993), longitude: Some(127.8087693), max_latitude: Some(2.645167), min_latitude: Some(-2.477640000000001), max_longitude: Some(129.6574671), min_longitude: Some(124.2830601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مالوكو الشمالية"), ("be", "Паўночнае Малуку"), ("bg", "Северно Молуку"), ("bn", "উত\u{9cd}তর ম\u{9be}ল\u{9c1}ক\u{9be} প\u{9cd}রদেশ"), ("ca", "Moluques Septentrionals"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄟𑄣\u{1112a}𑄇\u{1112a}"), ("ceb", "Provinsi Maluku Utara"), ("cs", "Severní Moluky"), ("cy", "Gogledd Maluku"), ("da", "North Maluku Province"), ("de", "Nordmolukken"), ("el", "Βόρειο Μαλούκου"), ("en", "North Maluku"), ("es", "Molucas Septentrional"), ("eu", "Ipar Molukak"), ("fa", "ملوک شمالی"), ("fi", "Pohjois-Molukit"), ("fr", "Moluques du Nord"), ("gu", "ઉત\u{acd}તર માલ\u{ac1}ક\u{ac1} પ\u{acd}રા\u{a82}ત"), ("he", "צפון מאלוקו"), ("hi", "उत\u{94d}तर माल\u{941}क\u{942}"), ("id", "Maluku Utara"), ("it", "Maluku Settentrionale"), ("ja", "北マルク州"), ("jv", "Maluku Lor"), ("ka", "ჩრდილოეთი მალუკუ"), ("kn", "ಉತ\u{ccd}ತರ ಮಲುಕು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "말루쿠우타라 주"), ("lt", "Šiaurės Molukai"), ("lv", "Ziemeļu Moluku province"), ("ml", "വടക\u{d4d}കൻ മല\u{d41}ക\u{d41}"), ("mr", "उत\u{94d}तर माल\u{941}क\u{942} प\u{94d}रा\u{902}त"), ("ms", "Maluku Utara"), ("nb", "Nord Maluku provins"), ("nl", "Noord-Molukken"), ("no", "Nord Maluku provins"), ("pl", "Moluki Północne"), ("pt", "Molucas do Norte"), ("ru", "Северное Малуку"), ("si", "උත\u{dd4}ර\u{dd4} මල\u{dd4}ක\u{dd4} පළ\u{dcf}ත"), ("sr", "Северни Молуци"), ("sr_Latn", "Severni Moluci"), ("sv", "Maluku Utara"), ("ta", "வடக\u{bcd}கு மளுக\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర మ\u{c3e}లుకు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาล\u{e39}ก\u{e39}เหน\u{e37}อ"), ("tr", "Kuzey Maluku"), ("uk", "Північне Малуку"), ("ur", "شمالی مالوکو"), ("vi", "Bắc Maluku"), ("yue", "北馬魯古省"), ("yue_Hans", "北马鲁古省"), ("zh", "北马鲁古省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::ID,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.249008), longitude: Some(106.776638), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نوسا تنقارا الغربية"), ("bg", "Западни Малки Зондски острови"), ("bn", "ওয\u{9bc}েস\u{9cd}ট ন\u{9c1}স\u{9be} তেনগ\u{9be}র\u{9be}"), ("ca", "Illes Petites de la Sonda Occidentals"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄚\u{1112a}𑄥 𑄑𑄬𑄚\u{11134}𑄉𑄢"), ("ceb", "West Nusa Tenggara"), ("cs", "Západní Nusa Tenggara"), ("cy", "Gorllewin Nusa Tenggara"), ("da", "West Nusa Tenggara"), ("de", "Nusa Tenggara Barat"), ("el", "Γουέστ Νούσα Τενγκάρα"), ("en", "West Nusa Tenggara"), ("es", "Nusatenggara Occidental"), ("eu", "Mendebaldeko Nusa Tenggara"), ("fa", "سوندای غربی"), ("fi", "Läntinen Nusa Tenggara"), ("fr", "Petites Îles de la Sonde occidentales"), ("gu", "વ\u{ac7}સ\u{acd}ટ ન\u{ac1}સા ત\u{ac7}\u{a82}ગારા"), ("hi", "पश\u{94d}चिम न\u{941}सा त\u{947}\u{902}गारा"), ("id", "Nusa Tenggara Barat"), ("it", "Nusa Tenggara Occidentale"), ("ja", "西ヌサ・トゥンガラ州"), ("jv", "Nusa Tenggara Kulon"), ("ka", "დასავლეთი მცირე ზონდის კუნძულები"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ನುಸಾ ತ\u{cc6}ಂಗ\u{ccd}ಗರಾ"), ("ko", "누사틍가라바랏 주"), ("lt", "Vakarų Nusa Tengara"), ("lv", "Rietumu Nusa Tengara"), ("mk", "Западни Мали Сундски острови"), ("ml", "പടിഞ\u{d4d}ഞ\u{d3e}റൻ ന\u{d41}സ ടെങ\u{d4d}ക\u{d3e}ര"), ("mr", "पश\u{94d}चिम न\u{941}सा त\u{947}\u{902}गारा"), ("ms", "Nusa Tenggara Barat"), ("nb", "West Nusa Tenggara"), ("nl", "West-Nusa Tenggara"), ("no", "West Nusa Tenggara"), ("pl", "Małe Wyspy Sundajskie Zachodnie"), ("pt", "Sonda Ocidental"), ("ru", "Западные Малые Зондские острова"), ("si", "බටහ\u{dd2}ර න\u{dd4}ස\u{dcf} ටෙන\u{dca}ග\u{dca}ගර\u{dcf}"), ("sq", "Nusa Tengara Perëndimore"), ("sr", "Западна Нуса Тенгара"), ("sr_Latn", "Zapadna Nusa Tengara"), ("sv", "Nusa Tenggara Barat"), ("ta", "மேற\u{bcd}கு நுச\u{bbe} தெங\u{bcd}க\u{bbe}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ నూస\u{c3e} ట\u{c46}ంగ\u{c3e}ర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e39}ซาเต\u{e47}งการาตะว\u{e31}นตก"), ("tr", "Batı Nusa Tenggara"), ("uk", "Західна Південно-Східна Нуса"), ("ur", "مغربی نوسا ٹنگارہ"), ("vi", "Nusa Tenggara Barat"), ("yue", "西努沙登加拉"), ("yue_Hans", "西努沙登加拉"), ("zh", "西努沙登加拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NT",
                    Subdivision{
                        name: "NT",
                        country_alpha2: Alpha2::ID,
                        code: "NT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.657381899999999), longitude: Some(121.0793705), max_latitude: Some(-8.063516), min_latitude: Some(-11.0075679), max_longitude: Some(125.192625), min_longitude: Some(118.927002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نوسا تنقارا الشرقية"), ("bg", "Източни Малки Зондски острови"), ("bn", "প\u{9c2}র\u{9cd}ব ন\u{9c1}স\u{9be} টেঙ\u{9cd}গ\u{9be}র\u{9be} প\u{9cd}রদেশ"), ("ca", "Illes Petites de la Sonda Orientals"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄚\u{1112a}𑄥 𑄑𑄬𑄚\u{11134}𑄉𑄢"), ("ceb", "Provinsi Nusa Tenggara Timur"), ("cs", "Východní Nusa Tenggara"), ("cy", "Dwyrain Nusa Tenggara"), ("da", "East Nusa Tenggara Province"), ("de", "Nusa Tenggara Timur"), ("el", "Ήστ Νούσα Τενγκάρα"), ("en", "East Nusa Tenggara"), ("es", "Nusatenggara Oriental"), ("eu", "Nusa Tenggara Timur"), ("fa", "سوندای شرقی"), ("fi", "Itäinen Nusa Tenggara"), ("fr", "Petites Îles de la Sonde orientales"), ("gu", "પ\u{ac2}ર\u{acd}વ ન\u{ac1}સા ત\u{ac7}\u{a82}ગારા પ\u{acd}રા\u{a82}ત"), ("he", "מזרח נוסה טנגרה"), ("hi", "प\u{942}र\u{94d}वी न\u{941}सा त\u{947}\u{902}गारा"), ("hr", "Istočni Mali sundski otoci"), ("id", "Nusa Tenggara Timur"), ("it", "Nusa Tenggara Orientale"), ("ja", "東ヌサ・トゥンガラ州"), ("jv", "Nusa Tenggara Wétan"), ("ka", "აღმოსავლეთი მცირე ზონდის კუნძულები"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ನುಸಾ ಟ\u{cc6}ಂಗ\u{ccd}ಗರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "누사틍가라티무르 주"), ("lt", "Rytų Nusa Tengara"), ("lv", "Austrumu Nusa Tengara"), ("mk", "Источни Мали Сундски Острови"), ("ml", "കിഴക\u{d4d}കൻ ന\u{d41}സ ടെങ\u{d4d}ക\u{d3e}ര"), ("mr", "प\u{942}र\u{94d}व न\u{941}सा त\u{947}\u{902}गारा"), ("ms", "Nusa Tenggara Timur"), ("nb", "Nusa Tenggara Timur"), ("nl", "Oost-Nusa Tenggara"), ("no", "Nusa Tenggara Timur"), ("pl", "Małe Wyspy Sundajskie Wschodnie"), ("pt", "Sonda Oriental"), ("ru", "Восточные Малые Зондские острова"), ("si", "නැගෙනහ\u{dd2}ර ටෙන\u{dca}ග\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sr", "Источна Нуса Тенгара"), ("sr_Latn", "Istočna Nusa Tengara"), ("sv", "Nusa Tenggara Timur"), ("ta", "கிழக\u{bcd}கு நுச\u{bbe} தெங\u{bcd}க\u{bbe}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు న\u{c4d}యూస\u{c3e} ట\u{c46}ంగ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "น\u{e39}ซา เตงการา ตะว\u{e31}นออก"), ("tr", "Doğu Nusa Tenggara"), ("uk", "Східна Південно-Східна Нуса"), ("ur", "مشرقی نوسا ٹنگارہ"), ("vi", "Đông Nusa Tenggara"), ("yue", "東努沙登加拉"), ("yue_Hans", "东努沙登加拉"), ("zh", "東努沙登加拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NU",
                    Subdivision{
                        name: "NU",
                        country_alpha2: Alpha2::ID,
                        code: "NU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Klein Sunda-eilande"), ("ar", "جزر سوندا الصغرى"), ("az", "Kiçik Zond adaları"), ("be", "Малыя Зондскія астравы"), ("bg", "Малки Зондски острови"), ("bs", "Mala Sundska ostrva"), ("ca", "Illes Petites de la Sonda"), ("ccp", "𑄣𑄬𑄥𑄢\u{11134} 𑄥\u{1112a}𑄚\u{11134}𑄓 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Lesser Sunda Islands"), ("cs", "Malé Sundy"), ("cy", "Ynysoedd Swnda Lleiaf"), ("da", "De små Sundaøer"), ("de", "Kleine Sundainseln"), ("el", "Νήσοι Μικρές Σούνδες"), ("en", "Lesser Sunda Islands"), ("es", "Islas menores de la Sonda"), ("et", "Väikesed Sunda saared"), ("eu", "Sondako Uharte Txikiak"), ("fa", "جزایر سوندای کوچک"), ("fi", "Pienet Sundasaaret"), ("fr", "Petites îles de la Sonde"), ("gl", "Illas menores da Sonda"), ("he", "איי סונדה הקטנים"), ("hi", "छोटा स\u{941}न\u{94d}दा द\u{94d}वीप सम\u{942}ह"), ("hr", "Mali sundski otoci"), ("hu", "Kis-Szunda-szigetek"), ("id", "Kepulauan Nusa Tenggara"), ("is", "Litlu-Sundaeyjar"), ("it", "Piccole Isole della Sonda"), ("ja", "小スンダ列島"), ("jv", "Kapuloan Nusa Tenggara"), ("ka", "მცირე ზონდის კუნძულები"), ("ko", "소순다 열도"), ("ky", "Кичи зонд аралдары"), ("lt", "Mažosios Sundos salos"), ("lv", "Mazās Zunda salas"), ("mk", "Мали Сундски Острови"), ("ml", "ലെസ\u{d4d}സർ സന\u{d4d}റ ദ\u{d4d}വീപ\u{d4d} സമ\u{d42}ഹം"), ("mn", "Бага Зондын арлууд"), ("ms", "Kepulauan Sunda Kecil"), ("nb", "De små Sundaøyer"), ("nl", "Kleine Soenda-eilanden"), ("no", "De små Sundaøyer"), ("pl", "Małe Wyspy Sundajskie"), ("pt", "Pequenas Ilhas da Sonda"), ("ru", "Малые Зондские острова"), ("sk", "Malé Sundy"), ("sr", "Мала Сундска острва"), ("sr_Latn", "Mala Sundska ostrva"), ("sv", "Små Sundaöarna"), ("ta", "சுந\u{bcd}த\u{bbe} சிறு த\u{bc0}வுகள\u{bcd}"), ("th", "หม\u{e39}\u{e48}เกาะซ\u{e38}นดาน\u{e49}อย"), ("tr", "Küçük Sunda Adaları"), ("uk", "Малі Зондські острови"), ("ur", "سونڈای جزائر کوچک"), ("uz", "Kichik zond orollari"), ("vi", "Quần đảo Sunda Nhỏ"), ("yue", "小巽他群島"), ("yue_Hans", "小巽他群岛"), ("zh", "小巽他群岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "PA",
                        country_alpha2: Alpha2::ID,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2033216), longitude: Some(-77.1945247), max_latitude: Some(42.5141658), min_latitude: Some(39.7197989), max_longitude: Some(-74.6895018), min_longitude: Some(-80.51989499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Papoea"), ("ar", "بابوا"), ("be", "Правінцыя Папуа"), ("bn", "প\u{9be}প\u{9c1}য\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Papua"), ("ccp", "𑄛𑄛\u{1112a}𑄠"), ("ceb", "Provinsi Papua"), ("cs", "Papua"), ("cy", "Papua"), ("da", "Papua Province"), ("de", "Papua"), ("el", "Παπούα"), ("en", "Papua"), ("es", "Papúa"), ("eu", "Papua"), ("fa", "پاپوآ"), ("fi", "Papua"), ("fr", "Papouasie"), ("gu", "પાપ\u{ac1}આ પ\u{acd}રા\u{a82}ત"), ("hi", "पाप\u{941}आ"), ("id", "Papua"), ("it", "Papua"), ("ja", "パプア州"), ("jv", "Papua"), ("ka", "პაპუა"), ("kn", "ಪಪುವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파푸아 주"), ("lt", "Papua"), ("lv", "Papua"), ("mk", "Папуа"), ("ml", "പപ\u{d41}വ"), ("mr", "पाप\u{941}आ"), ("ms", "Papua"), ("nb", "Papua"), ("nl", "Papoea"), ("no", "Papua"), ("pl", "Papua"), ("pt", "Papua"), ("ru", "Папуа"), ("si", "පැප\u{dd4}ව\u{dcf} පළ\u{dcf}ත"), ("sk", "Papua"), ("sr", "Папуа"), ("sr_Latn", "Papua"), ("sv", "Papua"), ("ta", "பப\u{bcd}புவ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}పువ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาป\u{e31}ว"), ("tr", "Papua"), ("uk", "Папуа"), ("ur", "پاپوا (صوبہ)"), ("vi", "Papua"), ("yue", "巴布亞"), ("yue_Hans", "巴布亚"), ("zh", "巴布亚省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PB",
                    Subdivision{
                        name: "PB",
                        country_alpha2: Alpha2::ID,
                        code: "PB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.789275), longitude: Some(113.921327), max_latitude: Some(6.216999899999999), min_latitude: Some(-11.1082999), max_longitude: Some(141.0425), min_longitude: Some(94.7351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بابوا الغربية"), ("bg", "Западна Папуа"), ("bn", "পশ\u{9cd}চিম প\u{9be}প\u{9c1}য\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Papua Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄛𑄛\u{1112a}𑄠"), ("ceb", "Provinsi Papua Barat"), ("cs", "Západní Papua"), ("cy", "Gorllewin Papua"), ("da", "West Papua Province"), ("de", "Papua Barat"), ("el", "Επαρχία Δυτικής Παπούα"), ("en", "West Papua"), ("es", "Papúa Occidental"), ("eu", "Mendebaldeko Papua"), ("fa", "پاپوآی غربی"), ("fi", "Länsi-Papua"), ("fr", "Papouasie occidentale"), ("gu", "પશ\u{acd}ચિમ પપ\u{ac1}આ પ\u{acd}રા\u{a82}ત"), ("he", "פפואה המערבית"), ("hi", "पश\u{94d}चिम पाप\u{941}आ"), ("id", "Papua Barat"), ("it", "Papua Occidentale"), ("ja", "西パプア州"), ("jv", "Papua Kulon"), ("ka", "დასავლეთი პაპუა"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಪಪುವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파푸아바랏 주"), ("lt", "Vakarų Papua"), ("lv", "Rietumpapua province"), ("mk", "Западна Папуа"), ("ml", "വെസ\u{d4d}റ\u{d4d}റ\u{d4d} പപ\u{d41}വ"), ("mr", "पश\u{94d}चिम पाप\u{941}आ"), ("ms", "Papua Barat"), ("nb", "Papua Barat"), ("nl", "West-Papoea"), ("no", "Papua Barat"), ("pl", "Papua Zachodnia"), ("pt", "Papua Ocidental"), ("ru", "Западное Папуа"), ("si", "බටහ\u{dd2}ර පැප\u{dd4}ව\u{dcf} පළ\u{dcf}ත"), ("sr", "Западна Папуа"), ("sr_Latn", "Zapadna Papua"), ("sv", "Papua Barat"), ("ta", "மேற\u{bcd}குப\u{bcd} பப\u{bcd}புவ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ ప\u{c3e}పువ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาป\u{e31}วตะว\u{e31}นตก"), ("tr", "Batı Papua"), ("uk", "Західне Папуа"), ("ur", "مغربی پاپوا (صوبہ)"), ("vi", "Tây Papua"), ("yue", "西巴布亞"), ("yue_Hans", "西巴布亚"), ("zh", "西巴布亞省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PP",
                    Subdivision{
                        name: "PP",
                        country_alpha2: Alpha2::ID,
                        code: "PP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.9084233), longitude: Some(109.136415), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غرب غينيا الجديدة"), ("bg", "Западна Нова Гвинея"), ("bn", "পশ\u{9cd}চিম প\u{9be}প\u{9c1}য\u{9bc}\u{9be}"), ("ca", "Papua Occidental"), ("ccp", "𑄛𑄛\u{1112a}𑄠 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("cs", "Západní Nová Guinea"), ("da", "West Papua"), ("de", "Westneuguinea"), ("el", "Πάπουα"), ("en", "Papua Islands"), ("es", "Nueva Guinea Occidental"), ("eu", "Mendebaldeko Papua²"), ("fa", "پاپوآی غربی²"), ("fi", "Länsi-Uusi-Guinea"), ("fr", "Nouvelle-Guinée occidentale"), ("gu", "વ\u{ac7}સ\u{acd}ટ પપ\u{ac1}આ"), ("he", "מערב גינאה החדשה"), ("hi", "पश\u{94d}चिम पाप\u{941}आ²"), ("hy", "Արևմտյան Իրիան"), ("id", "Papua bagian barat"), ("is", "Vestur-Nýja-Gínea"), ("it", "Nuova Guinea Occidentale"), ("ja", "イリアンジャヤ"), ("ka", "დასავლეთი ახალი გვინეა"), ("kk", "Батыс Ириан"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಪಪುವಾ"), ("ko", "서뉴기니"), ("lt", "Vakarų Naujoji Gvinėja"), ("lv", "Rietumpapua"), ("ml", "വെസ\u{d4d}റ\u{d4d}റേൺ ന\u{d4d}യ\u{d42} ഗിനിയ"), ("mn", "Папуа-Шинэ Гвиней"), ("mr", "व\u{947}स\u{94d}ट पाप\u{941}आ"), ("ms", "West Papua"), ("nb", "Vest-Papua"), ("nl", "Irian Jaya"), ("no", "Vest-Papua"), ("pl", "Irian Zachodni"), ("pt", "Nova Guiné Ocidental"), ("ru", "Западная Новая Гвинея"), ("si", "බටහ\u{dd2}ර පැප\u{dd4}ව\u{dcf}"), ("sv", "Västpapua"), ("ta", "மேற\u{bcd}கு ப\u{bbe}புஆ"), ("te", "పశ\u{c4d}చ\u{c3f}మ ప\u{c3e}పువ\u{c3e}"), ("th", "น\u{e34}วก\u{e34}น\u{e35}ตะว\u{e31}นตก"), ("tr", "Batı Papua²"), ("uk", "Західна Нова Гвінея"), ("ur", "مغربی پاپوا (علاقہ)"), ("vi", "Tây New Guinea"), ("yue", "西新畿內亞"), ("yue_Hans", "西新畿内亚"), ("zh", "西新几内亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::ID,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.91266), longitude: Some(107.642042), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "رياو"), ("be", "Правінцыя Рыау"), ("bg", "Риау"), ("bn", "রিয\u{9bc}\u{9be}উ প\u{9cd}রদেশ"), ("ca", "Riau"), ("ccp", "𑄢\u{1112d}𑄅\u{1112a}"), ("ceb", "Provinsi Riau"), ("cs", "Riau"), ("cy", "Riau"), ("da", "Riau"), ("de", "Riau"), ("el", "Ριάου"), ("en", "Riau"), ("es", "Riau"), ("eu", "Riau"), ("fa", "ریائو"), ("fi", "Riau"), ("fr", "Riau"), ("gu", "રિયાઉ પ\u{acd}રા\u{a82}ત"), ("hi", "रियाउ"), ("id", "Riau"), ("it", "Riau"), ("ja", "リアウ州"), ("jv", "Riau"), ("ka", "რიაუ"), ("kn", "ರ\u{cbf}ಯು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리아우 주"), ("lt", "Riau"), ("lv", "Riau province"), ("ml", "റിയ\u{d3e}വ\u{d41}"), ("mn", "Риау"), ("mr", "रियाउ"), ("ms", "Riau"), ("nb", "Riau"), ("nl", "Riau"), ("no", "Riau"), ("pl", "Riau"), ("pt", "Riau"), ("ru", "Риау"), ("si", "ර\u{dd2}යෞ පළ\u{dcf}ත"), ("sr", "Ријау"), ("sr_Latn", "Rijau"), ("sv", "Riau"), ("ta", "ரிய\u{bbe}வு"), ("te", "ర\u{c3f}య\u{c3e}యూ ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเร\u{e35}ยว"), ("tr", "Riau"), ("uk", "Ріау"), ("ur", "ریاو"), ("vi", "Riau"), ("yue", "廖內"), ("yue_Hans", "廖内"), ("zh", "廖內省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::ID,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.7199596), longitude: Some(115.1752526), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولاوسي الشمالية"), ("be", "Правінцыя Паўночны Сулавесі"), ("bg", "Северно Сулавеси"), ("bn", "উত\u{9cd}তর স\u{9c1}ল\u{9be}বেসি প\u{9cd}রদেশ"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄥\u{1112a}𑄣𑄠𑄬𑄥\u{11128}"), ("ceb", "Sulawesi Utara"), ("cs", "Severní Sulawesi"), ("da", "North Sulawesi Province"), ("de", "Sulawesi Utara"), ("el", "Βόρειο Σουλαβέσι"), ("en", "North Sulawesi"), ("es", "Célebes Septentrional"), ("eu", "Ipar Sulawesi"), ("fa", "سولاوسی شمالی"), ("fi", "Pohjois-Sulawesi"), ("fr", "Sulawesi du Nord"), ("gu", "ઉત\u{acd}તર સ\u{ac1}લાવ\u{ac7}સી પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तर स\u{941}लाव\u{947}सी"), ("id", "Sulawesi Utara"), ("it", "Sulawesi Settentrionale"), ("ja", "北スラウェシ州"), ("jv", "Sulawesi Lor"), ("ka", "ჩრდილოეთი სულავესი"), ("kn", "ಉತ\u{ccd}ತರ ಸುಲಾವ\u{cc6}ಸ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "술라웨시우타라 주"), ("lt", "Šiaurės Sulavesis"), ("lv", "Ziemeļsulavesi province"), ("ml", "വടക\u{d4d}കൻ സ\u{d41}ലവേസി"), ("mn", "Умар Сулавеси"), ("mr", "उत\u{94d}तर स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi Utara"), ("nb", "Nord Sulawesi provins"), ("nl", "Noord-Celebes"), ("no", "Nord Sulawesi provins"), ("pl", "Celebes Północny"), ("pt", "Celebes do Norte"), ("ru", "Северный Сулавеси"), ("si", "උත\u{dd4}ර\u{dd4} ස\u{dd4}ලවෙස\u{dd2} පළ\u{dcf}ත"), ("sr", "Северни Сулавеси"), ("sr_Latn", "Severni Sulavesi"), ("sv", "Sulawesi Utara"), ("ta", "வடக\u{bcd}குச\u{bcd} சுள\u{bbe}வெசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర సుల\u{c3e}వ\u{c47}స\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ลาเวซ\u{e35}ใต\u{e49}"), ("tr", "Kuzey Sulawesi"), ("uk", "Північне Сулавесі"), ("ur", "شمالی سولاویسی"), ("vi", "Bắc Sulawesi"), ("yue", "北蘇拉威西"), ("yue_Hans", "北苏拉威西"), ("zh", "北苏拉威西省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SB",
                    Subdivision{
                        name: "SB",
                        country_alpha2: Alpha2::ID,
                        code: "SB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.7399397), longitude: Some(100.8000051), max_latitude: Some(0.907389), min_latitude: Some(-4.020091), max_longitude: Some(101.8929181), min_longitude: Some(98.59692190000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سومطرة الغربية"), ("be", "Правінцыя Заходняя Суматра"), ("bg", "Западна Суматра"), ("bn", "পশ\u{9cd}চিম স\u{9c1}ম\u{9be}ত\u{9cd}র\u{9be}"), ("ca", "Sumatra Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄥\u{1112a}𑄟𑄖\u{11133}𑄢\u{11134}"), ("ceb", "Provinsi Sumatera Barat"), ("cs", "Západní Sumatra"), ("cy", "Gorllewin Sumatra"), ("da", "Vestsumatra"), ("de", "Sumatera Barat"), ("el", "Δυτική Σουμάτρα (Γουέστ Σουμάτρα)"), ("en", "West Sumatra"), ("es", "Sumatra Occidental"), ("eu", "Mendebaldeko Sumatra"), ("fa", "سوماترای غربی"), ("fi", "Länsi-Sumatra"), ("fr", "Sumatra occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ સ\u{ac1}માત\u{acd}રા"), ("he", "מערב סומטרה"), ("hi", "पश\u{94d}चिम स\u{941}मात\u{94d}रा"), ("id", "Sumatera Barat"), ("it", "Sumatra Occidentale"), ("ja", "西スマトラ州"), ("jv", "Sumatra Kulon"), ("ka", "დასავლეთი სუმატრა"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಸುಮಾತ\u{ccd}ರ"), ("ko", "수마트라바랏 주"), ("lt", "Vakarų Sumatra"), ("lv", "Rietumsumatra"), ("ml", "പടിഞ\u{d4d}ഞ\u{d3e}റൻ സ\u{d41}മ\u{d3e}ത\u{d4d}ര"), ("mn", "Өрнө Суматра"), ("mr", "पश\u{94d}चिम स\u{941}मात\u{94d}रा"), ("ms", "Sumatera Barat"), ("nb", "Sumatera Barat"), ("nl", "West-Sumatra"), ("no", "Sumatera Barat"), ("pl", "Sumatra Zachodnia"), ("pt", "Sumatra Ocidental"), ("ru", "Западная Суматра"), ("si", "බස\u{dca}න\u{dcf}හ\u{dd2}ර ස\u{dd4}ම\u{dcf}ත\u{dca}\u{200d}ර\u{dcf}"), ("sr", "Западна Суматра"), ("sr_Latn", "Zapadna Sumatra"), ("sv", "Sumatera Barat"), ("ta", "மேற\u{bcd}குச\u{bcd} சும\u{bbe}த\u{bcd}திர\u{bbe}"), ("te", "పశ\u{c4d}చ\u{c3f}మ సుమత\u{c4d}ర"), ("th", "จ\u{e31}งหว\u{e31}ดส\u{e38}มาตราตะว\u{e31}นตก"), ("tr", "Batı Sumatra"), ("uk", "Західна Суматра"), ("ur", "مغربی سماٹرا"), ("vi", "Tây Sumatera"), ("yue", "西蘇門答臘"), ("yue_Hans", "西苏门答腊"), ("zh", "西苏门答腊省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SG",
                    Subdivision{
                        name: "SG",
                        country_alpha2: Alpha2::ID,
                        code: "SG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولاوسي الجنوبية الشرقية"), ("be", "Правінцыя Паўднёва-Усходні Сулавесі"), ("bg", "Югоизточно Сулавеси"), ("bn", "দক\u{9cd}ষিণ প\u{9c2}র\u{9cd}ব স\u{9c1}লভিসী প\u{9cd}রদেশ"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥\u{1112a}𑄣𑄠𑄬𑄥\u{11128}"), ("ceb", "Sulawesi Tenggara"), ("cs", "Jihovýchodní Sulawesi"), ("da", "Sydøstsulawesi provins"), ("de", "Sulawesi Tenggara"), ("el", "Νοτιοανατολικό Σουλαβέσι"), ("en", "Southeast Sulawesi"), ("es", "Célebes Suroriental"), ("eu", "Hego-ekialdeko Sulawesi"), ("fa", "سولاوسی جنوب شرقی"), ("fi", "Kaakkois-Sulawesi"), ("fr", "Sulawesi du Sud-Est"), ("gu", "દક\u{acd}ષિણ પ\u{ac2}ર\u{acd}વ સ\u{ac1}લાવ\u{ac7}સી પ\u{acd}રા\u{a82}ત"), ("hi", "आग\u{94d}न\u{947}य स\u{941}लाव\u{947}सी"), ("id", "Sulawesi Tenggara"), ("it", "Sulawesi Sudorientale"), ("ja", "南東スラウェシ州"), ("jv", "Sulawesi Kidul-wétan"), ("ka", "სამხრეთ-აღმოსავლეთი სულავესი"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಈಸ\u{ccd}ಟ\u{ccd} ಸುಲಾವ\u{cc6}ಸ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "술라웨시틍가라 주"), ("lt", "Pietryčių Sulavesis"), ("lv", "Dienvidaustrumu Suvalesi"), ("mr", "आग\u{94d}न\u{947}य स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi Tenggara"), ("nb", "South East Sulawesi Provins"), ("nl", "Zuidoost-Celebes"), ("no", "South East Sulawesi Provins"), ("pl", "Celebes Południowo-Wschodni"), ("pt", "Celebes do Sudeste"), ("ru", "Юго-Восточный Сулавеси"), ("si", "දක\u{dd4}ණ\u{dd4} නැගෙනහ\u{dd2}ර ස\u{dd4}ලවෙස\u{dd2} පළ\u{dcf}ත"), ("sr", "Југоисточни Сулавеси"), ("sr_Latn", "Jugoistočni Sulavesi"), ("sv", "Sulawesi Tenggara"), ("ta", "தென\u{bcd}கிழக\u{bcd}குச\u{bcd} சுள\u{bbe}வெசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ తూర\u{c4d}పు సులవ\u{c47}స\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ส\u{e38}ลาเวส\u{e35}เตงการาตะว\u{e31}นออกเฉ\u{e35}ยงใต\u{e49}"), ("tr", "Güneydoğu Sulawesi"), ("uk", "Південно-Східне Сулавесі"), ("ur", "جنوب مشرقی سولاویسی"), ("vi", "Đông Nam Sulawesi"), ("yue", "東南蘇拉威西"), ("yue_Hans", "东南苏拉威西"), ("zh", "东南苏拉威西省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SL",
                    Subdivision{
                        name: "SL",
                        country_alpha2: Alpha2::ID,
                        code: "SL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sulawesi"), ("ar", "سولاوسي"), ("az", "Sulavesi"), ("be", "Востраў Сулавесі"), ("bg", "Сулавеси"), ("bs", "Sulawesi"), ("ca", "Cèlebes"), ("ccp", "𑄥\u{1112a}𑄣𑄃\u{1112e}𑄠𑄬𑄥\u{11128}"), ("ceb", "Sulawesi"), ("cs", "Celebes"), ("cy", "Sulawesi"), ("da", "Sulawesi"), ("de", "Sulawesi"), ("el", "Σουλαουέζι"), ("en", "Sulawesi"), ("es", "Célebes"), ("et", "Sulawesi"), ("eu", "Sulawesi"), ("fa", "سولاوسی"), ("fi", "Sulawesi"), ("fr", "Célèbes"), ("ga", "An Cheilibéis"), ("gl", "Célebes"), ("ha", "Sulawesi"), ("ha_NE", "Sulawesi"), ("he", "סולאווסי"), ("hi", "स\u{941}लाव\u{947}सी"), ("hr", "Sulawesi"), ("hu", "Celebesz"), ("hy", "Ցելեբես Սուլավեսի"), ("id", "Sulawesi"), ("is", "Súlavesí"), ("it", "Sulawesi"), ("ja", "スラウェシ島"), ("jv", "Sulawesi"), ("ka", "სულავესი"), ("kk", "Сулавеси"), ("ko", "술라웨시 섬"), ("lt", "Sulavesis"), ("lv", "Sulavesi"), ("mk", "Сулавеси"), ("ml", "സ\u{d41}ലവേസി"), ("mr", "स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi"), ("my", "ဆ\u{1030}လာဝေစ\u{102e}ကျ\u{103d}န\u{103a}း"), ("nb", "Sulawesi"), ("nl", "Celebes"), ("no", "Sulawesi"), ("pa", "ਸ\u{a41}ਲਾਵ\u{a47}ਸੀ"), ("pl", "Celebes"), ("pt", "Celebes"), ("ro", "Sulawesi"), ("ru", "Сулавеси"), ("sk", "Sulawesi"), ("sl", "Sulavezi"), ("sr", "Сулавеси"), ("sr_Latn", "Sulavesi"), ("sv", "Sulawesi"), ("sw", "Sulawesi"), ("ta", "சுல\u{bbe}வெசி"), ("th", "เกาะซ\u{e39}ลาเวซ\u{e35}"), ("tr", "Sulawesi"), ("uk", "Сулавесі"), ("ur", "سولاویسی"), ("uz", "Sulavesi"), ("vi", "Sulawesi"), ("yue", "蘇拉威西"), ("yue_Hans", "苏拉威西"), ("zh", "苏拉威西岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SM",
                    Subdivision{
                        name: "SM",
                        country_alpha2: Alpha2::ID,
                        code: "SM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sumatra"), ("am", "ሱማትራ"), ("ar", "سومطرة"), ("az", "Sumatra"), ("be", "Востраў Суматра"), ("bg", "Суматра"), ("bn", "স\u{9c1}ম\u{9be}ত\u{9cd}র\u{9be}"), ("bs", "Sumatra"), ("ca", "Sumatra"), ("ccp", "𑄥\u{1112a}𑄟𑄖\u{11133}𑄢\u{11134}"), ("ceb", "Sumatra"), ("cs", "Sumatra"), ("cy", "Sumatera"), ("da", "Sumatra"), ("de", "Sumatra"), ("el", "Σουμάτρα"), ("en", "Sumatra"), ("es", "Sumatra"), ("et", "Sumatra"), ("eu", "Sumatra"), ("fa", "سوماترا"), ("fi", "Sumatra"), ("fr", "Sumatra"), ("ga", "Sumatra"), ("gl", "Sumatra"), ("ha", "Sumatra"), ("ha_NE", "Sumatra"), ("he", "סומטרה"), ("hi", "स\u{941}मात\u{94d}रा"), ("hr", "Sumatra"), ("hu", "Szumátra"), ("hy", "Սումատրա"), ("id", "Sumatera"), ("is", "Súmatra"), ("it", "Sumatra"), ("ja", "スマトラ島"), ("jv", "Sumatra"), ("ka", "სუმატრა"), ("kk", "Суматра"), ("km", "កោះស\u{17ca}\u{17bc}ម\u{17c9}ាត\u{17d2}រា"), ("ko", "수마트라 섬"), ("ky", "Суматра"), ("lt", "Sumatra"), ("lv", "Sumatra"), ("mk", "Суматра"), ("ml", "സ\u{d41}മ\u{d3e}ത\u{d4d}ര"), ("mr", "स\u{941}मात\u{94d}रा"), ("ms", "Sumatera"), ("my", "ဆ\u{1030}မတြာကျ\u{103d}န\u{103a}း"), ("nb", "Sumatra"), ("ne", "स\u{941}मात\u{94d}रा"), ("nl", "Sumatra"), ("no", "Sumatra"), ("or", "ସ\u{b41}ମ\u{b3e}ତ\u{b4d}ର\u{b3e}"), ("pl", "Sumatra"), ("pt", "Sumatra"), ("ro", "Sumatra"), ("ru", "Суматра"), ("sk", "Sumatra"), ("sl", "Sumatra"), ("sq", "Sumatra"), ("sr", "Суматра"), ("sr_Latn", "Sumatra"), ("sv", "Sumatra"), ("sw", "Sumatra"), ("ta", "சும\u{bbe}த\u{bcd}திர\u{bbe}"), ("th", "เกาะส\u{e38}มาตรา"), ("tr", "Sumatra"), ("uk", "Суматра"), ("ur", "سماٹرا"), ("uz", "Sumatra"), ("vi", "Sumatra"), ("yue", "蘇門答臘"), ("yue_Hans", "苏门答腊"), ("zh", "蘇門答臘")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::ID,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.6687994), longitude: Some(119.9740534), max_latitude: Some(-1.8952359), min_latitude: Some(-7.758941000000001), max_longitude: Some(121.8402099), min_longitude: Some(117.038483)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولاوسي الجنوبية"), ("be", "Правінцыя Паўднёвы Сулавесі"), ("bg", "Южно Сулавеси"), ("bn", "দক\u{9cd}ষিন স\u{9c1}ল\u{9be}বেসি"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄥\u{1112a}𑄣𑄠𑄬𑄥\u{11128}"), ("ceb", "Provinsi Sulawesi Selatan"), ("cs", "Jižní Sulawesi"), ("da", "South Sulawesi"), ("de", "Sulawesi Selatan"), ("el", "Νότιο Σουλαβέσι"), ("en", "South Sulawesi"), ("es", "Célebes Meridional"), ("eu", "Hego Sulawesi"), ("fa", "سولاوسی جنوبی"), ("fi", "Etelä-Sulawesi"), ("fr", "Sulawesi du Sud"), ("gl", "Célebes Meridional"), ("gu", "સાઉથ સ\u{ac1}લાવ\u{ac7}સી"), ("hi", "दक\u{94d}षिण स\u{941}लाव\u{947}सी"), ("id", "Sulawesi Selatan"), ("it", "Sulawesi Meridionale"), ("ja", "南スラウェシ州"), ("jv", "Sulawesi Kidul"), ("ka", "სამხრეთი სულავესი"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಸುಲಾವ\u{cc6}ಸ\u{cbf}"), ("ko", "술라웨시슬라탄 주"), ("lt", "Pietų Sulavesis"), ("lv", "Dienvidsulavesi"), ("ml", "തെക\u{d4d}കൻ സ\u{d41}ലവേസി"), ("mr", "दक\u{94d}षिण स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi Selatan"), ("nb", "Sør Sulawesi"), ("nl", "Zuid-Celebes"), ("no", "Sør Sulawesi"), ("pl", "Celebes Południowy"), ("pt", "Celebes do Sul"), ("ru", "Южный Сулавеси"), ("si", "දක\u{dd4}ණ\u{dd4} ස\u{dd4}ලවෙස\u{dd2}"), ("sr", "Јужни Сулавеси"), ("sr_Latn", "Južni Sulavesi"), ("sv", "Sulawesi Selatan"), ("ta", "தெற\u{bcd}குச\u{bcd} சுள\u{bbe}வெசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ సులవ\u{c47}స\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดส\u{e38}ลาเวส\u{e35}"), ("tr", "Güney Sulawesi"), ("uk", "Південне Сулавесі"), ("ur", "جنوبی سولاویسی"), ("vi", "Nam Sulawesi"), ("yue", "南蘇拉威西"), ("yue_Hans", "南苏拉威西"), ("zh", "南苏拉威西省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::ID,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.926822599999999), longitude: Some(107.6068221), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولاوسي الغربية"), ("be", "Правінцыя Заходні Сулавесі"), ("bg", "Западно Сулавеси"), ("bn", "ওয\u{9bc}েস\u{9cd}ট স\u{9c1}ল\u{9be}বেসি"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄥\u{1112a}𑄣𑄠𑄬𑄥\u{11128}"), ("ceb", "Provinsi Sulawesi Barat"), ("cs", "Západní Sulawesi"), ("da", "West Sulawesi (Vestsulawesi)"), ("de", "Westsulawesi"), ("el", "Γουέστ Σουλαβέσι"), ("en", "West Sulawesi"), ("es", "Célebes Occidental"), ("eu", "Mendebaldeko Sulawesi"), ("fa", "سولاوسی غربی"), ("fi", "Länsi-Sulawesi"), ("fr", "Sulawesi occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ સ\u{ac1}લાવ\u{ac7}સી"), ("hi", "पश\u{94d}चिम स\u{941}लाव\u{947}सी"), ("id", "Sulawesi Barat"), ("it", "Sulawesi Occidentale"), ("ja", "西スラウェシ州"), ("jv", "Sulawesi Kulon"), ("ka", "დასავლეთი სულავესი"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಸುಲಾವ\u{cc6}ಸ\u{cbf}"), ("ko", "술라웨시바랏 주"), ("lt", "Vakarų Sulavesis"), ("lv", "Rietumsulavesi"), ("mn", "Өрнө Сулавеси"), ("mr", "पश\u{94d}चिम स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi Barat"), ("nb", "West Sulawesi"), ("nl", "West-Celebes"), ("no", "West Sulawesi"), ("pl", "Celebes Zachodni"), ("pt", "Celebes Ocidental"), ("ru", "Западный Сулавеси"), ("si", "බටහ\u{dd2}ර ස\u{dd4}ලවෙස\u{dd2}"), ("sr", "Западни Сулавеси"), ("sr_Latn", "Zapadni Sulavesi"), ("sv", "Sulawesi Barat"), ("ta", "மேற\u{bcd}குச\u{bcd} சுள\u{bbe}வெசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ సులవ\u{c47}స\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ลาเวซ\u{e35}ตะว\u{e31}นตก"), ("tr", "Batı Sulawesi"), ("uk", "Західне Сулавесі"), ("ur", "مغربی سولاویسی"), ("vi", "Tây Sulawesi"), ("yue", "西蘇拉威西"), ("yue_Hans", "西苏拉威西"), ("zh", "西苏拉威西省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "SS",
                        country_alpha2: Alpha2::ID,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Sumatra"), ("ar", "سومطرة الجنوبية"), ("bg", "Южна Суматра"), ("bn", "দক\u{9cd}ষিণ স\u{9c1}ম\u{9be}ত\u{9cd}র\u{9be}র"), ("ca", "Sumatra Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄥\u{1112a}𑄟𑄖\u{11133}𑄢\u{11134}"), ("ceb", "Sumatera Selatan"), ("cs", "Jižní Sumatra"), ("cy", "De Sumatra"), ("da", "Sydsumatra"), ("de", "Sumatera Selatan"), ("el", "Νότια Σουμάτρα"), ("en", "South Sumatra"), ("es", "Sumatra Meridional"), ("eu", "Hego Sumatra"), ("fa", "سوماترای جنوبی"), ("fi", "Etelä-Sumatra"), ("fr", "Sumatra du Sud"), ("gl", "Sumatra Meridional"), ("gu", "સાઉથ સ\u{ac1}માત\u{acd}રા"), ("he", "דרום סומטרה"), ("hi", "दक\u{94d}षिण स\u{941}मात\u{94d}रा"), ("id", "Sumatera Selatan"), ("it", "Sumatra Meridionale"), ("ja", "南スマトラ州"), ("jv", "Sumatra Kidul"), ("ka", "სამხრეთი სუმატრა"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಸುಮಾತ\u{ccd}ರ"), ("ko", "수마트라슬라탄 주"), ("lt", "Pietų Sumatra"), ("lv", "Dienvidsumatra"), ("ml", "തെക\u{d4d}കൻ സ\u{d41}മ\u{d3e}ത\u{d4d}ര"), ("mn", "Өмнө Суматра"), ("mr", "दक\u{94d}षिण स\u{941}मात\u{94d}रा"), ("ms", "Sumatera Selatan"), ("nb", "Sumatera Selatan"), ("nl", "Zuid-Sumatra"), ("no", "Sumatera Selatan"), ("pl", "Sumatra Południowa"), ("pt", "Sumatra do Sul"), ("ru", "Южная Суматра"), ("si", "දක\u{dd4}ණ\u{dd4} ස\u{dd4}ම\u{dcf}ත\u{dca}\u{200d}ර\u{dcf}"), ("sr", "Јужна Суматра"), ("sr_Latn", "Južna Sumatra"), ("sv", "Sumatera Selatan"), ("ta", "தெற\u{bcd}குச\u{bcd} சும\u{bbe}த\u{bcd}திர\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ సుమత\u{c4d}ర"), ("th", "จ\u{e31}งหว\u{e31}ดส\u{e38}มาตราใต\u{e49}"), ("tr", "Güney Sumatra"), ("uk", "Південна Суматра"), ("ur", "جنوبی سماٹرا"), ("vi", "Nam Sumatera"), ("yue", "南蘇門答臘"), ("yue_Hans", "南苏门答腊"), ("zh", "南苏门答腊省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ST",
                    Subdivision{
                        name: "ST",
                        country_alpha2: Alpha2::ID,
                        code: "ST",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سولاوسي الوسطى"), ("be", "Правінцыя Цэнтральны Сулавесі"), ("bg", "Централно Сулавеси"), ("bn", "কেন\u{9cd}দ\u{9cd}রীয\u{9bc} স\u{9c1}ল\u{9be}বেসি প\u{9cd}রদেশ"), ("ca", "Celebas del Centre"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄥\u{1112a}𑄣𑄠𑄬𑄥\u{11128}"), ("ceb", "Sulawesi Tengah"), ("cs", "Střední Sulawesi"), ("da", "Central Sulawesi Province"), ("de", "Sulawesi Tengah"), ("el", "Κεντρικό Σουλαβέσι"), ("en", "Central Sulawesi"), ("es", "Célebes Central"), ("eu", "Erdialdeko Sulawesi"), ("fa", "سولاوسی مرکزی"), ("fi", "Keski-Sulawesi"), ("fr", "Sulawesi central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ સ\u{ac1}લાવ\u{ac7}સી પ\u{acd}રા\u{a82}ત"), ("hi", "मध\u{94d}य स\u{941}लाव\u{947}सी"), ("id", "Sulawesi Tengah"), ("it", "Sulawesi Centrale"), ("ja", "中部スラウェシ州"), ("jv", "Sulawesi Tengah"), ("ka", "ცენტრალური სულავესი"), ("kn", "ಕೇಂದ\u{ccd}ರ ಸುಲಾವ\u{cc6}ಸ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "술라웨시틍아 주"), ("lt", "Centrinis Sulavesis"), ("lv", "Centrālā Sulavesi"), ("ml", "മദ\u{d4d}ധ\u{d4d}യ സ\u{d41}ലവേസി"), ("mn", "Төв Сулавеси"), ("mr", "मध\u{94d}य स\u{941}लाव\u{947}सी"), ("ms", "Sulawesi Tengah"), ("nb", "Sentral Sulaweisi provins"), ("nl", "Midden-Celebes"), ("no", "Sentral Sulaweisi provins"), ("pl", "Celebes Środkowy"), ("pt", "Celebes Central"), ("ru", "Центральный Сулавеси"), ("si", "මද\u{dca}\u{200d}යම ස\u{dd4}ලවෙස\u{dd2} පළ\u{dcf}ත"), ("sr", "Централни Сулавеси"), ("sr_Latn", "Centralni Sulavesi"), ("sv", "Sulawesi Tengah"), ("ta", "நடுச\u{bcd} சுள\u{bbe}வெசி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} సుల\u{c3e}వ\u{c47}స\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ลาเวซ\u{e35}กลาง"), ("tr", "Orta Sulawesi"), ("uk", "Центральне Сулавесі"), ("ur", "وسطی سولاویسی"), ("vi", "Trung Sulawesi"), ("yue", "中蘇拉威西"), ("yue_Hans", "中苏拉威西"), ("zh", "中苏拉威西省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::ID,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.1153547), longitude: Some(99.54509739999999), max_latitude: Some(4.3013449), min_latitude: Some(-0.6387609), max_longitude: Some(100.4257811), min_longitude: Some(97.0575619)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سومطرة الشمالية"), ("be", "Правінцыя Паўночная Суматра"), ("bg", "Северна Суматра"), ("bn", "উত\u{9cd}তর স\u{9c1}ম\u{9be}ত\u{9cd}র\u{9be} প\u{9cd}রদেশ"), ("ca", "Sumatra Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄥\u{1112a}𑄟𑄖\u{11133}𑄢\u{11134}"), ("ceb", "Sumatera Utara"), ("cs", "Severní Sumatra"), ("cy", "Gogledd Sumatra"), ("da", "Nordsumatra"), ("de", "Sumatera Utara"), ("el", "Βόρεια Σουμάτρα"), ("en", "North Sumatra"), ("es", "Sumatra Septentrional"), ("eu", "Ipar Sumatra"), ("fa", "سوماترای شمالی"), ("fi", "Pohjois-Sumatra"), ("fr", "Sumatra du Nord"), ("gu", "ઉત\u{acd}તર સ\u{ac1}માત\u{acd}રા પ\u{acd}રા\u{a82}ત"), ("he", "צפון סומטרה"), ("hi", "उत\u{94d}तर स\u{941}मात\u{94d}रा"), ("id", "Sumatera Utara"), ("it", "Sumatra Settentrionale"), ("ja", "北スマトラ州"), ("jv", "Sumatra Lor"), ("ka", "ჩრდილოეთი სუმატრა"), ("kn", "ಉತ\u{ccd}ತರ ಸುಮಾತ\u{ccd}ರ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "수마트라우타라 주"), ("lt", "Šiaurės Sumatra"), ("lv", "Ziemeļsumatra"), ("ml", "വടക\u{d4d}കൻ സ\u{d41}മ\u{d3e}ത\u{d4d}ര"), ("mn", "Умар Суматра"), ("mr", "उत\u{94d}तर स\u{941}मात\u{94d}रा"), ("ms", "Sumatera Utara"), ("nb", "Sumatera Utara"), ("nl", "Noord-Sumatra"), ("no", "Sumatera Utara"), ("pl", "Sumatra Północna"), ("pt", "Sumatra do Norte"), ("ru", "Северная Суматра"), ("si", "උත\u{dd4}ර\u{dd4} ස\u{dd4}ම\u{dcf}ත\u{dca}\u{200d}ර\u{dcf} පළ\u{dcf}ත"), ("sr", "Северна Суматра"), ("sr_Latn", "Severna Sumatra"), ("sv", "Sumatera Utara"), ("ta", "வடக\u{bcd}கு சும\u{bbe}த\u{bcd}திர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర సుమత\u{c4d}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดส\u{e38}มาตราเหน\u{e37}อ"), ("tr", "Kuzey Sumatra"), ("uk", "Північна Суматра"), ("ur", "شمالی سماٹرا"), ("uz", "Shimoliy Sumatra"), ("vi", "Bắc Sumatera"), ("yue", "北蘇門答臘"), ("yue_Hans", "北苏门答腊"), ("zh", "北苏门答腊省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "YO",
                    Subdivision{
                        name: "YO",
                        country_alpha2: Alpha2::ID,
                        code: "YO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.875384899999999), longitude: Some(110.4262088), max_latitude: Some(-7.541901900000001), min_latitude: Some(-8.204191999999999), max_longitude: Some(110.8346329), min_longitude: Some(110.013942)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yogyakarta"), ("ar", "يوجياكرتا"), ("bg", "Джокякарта"), ("bn", "ইয\u{9bc}গ\u{9be}ক\u{9be}র\u{9be}ত\u{9be}-র বিশেষ অঞ\u{9cd}চল"), ("ca", "Yogyakarta"), ("ccp", "𑄡\u{1112e}𑄉𑄇𑄢\u{11134}𑄖"), ("ceb", "Daerah Istimewa Yogyakarta"), ("cs", "Yogyakarta"), ("cy", "Yogyakarta"), ("da", "Yogyakarta"), ("de", "Yogyakarta"), ("el", "Γιογκιακάρτα"), ("en", "Yogyakarta"), ("es", "Yogyakarta"), ("et", "Yogyakarta"), ("eu", "Yogyakarta"), ("fa", "یوگیاکارتا"), ("fi", "Yogyakarta"), ("fr", "Territoire spécial de Yogyakarta"), ("ga", "Yogyakarta"), ("gl", "Yogyakarta"), ("gu", "યોગ\u{acd}યાકર\u{acd}તાનો સ\u{acd}પ\u{ac7}શિયલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "योग\u{94d}यकर\u{94d}ता"), ("hr", "Yogyakarta"), ("hu", "Yogyakarta"), ("id", "Yogyakarta"), ("is", "Yogyakarta"), ("it", "Yogyakarta"), ("ja", "ジョグジャカルタ特別州"), ("jv", "Dhaérah Istiméwa Yogyakarta"), ("ka", "ჯოკიაკარტის სპეციალური რეგიონი"), ("kn", "ಯೋಗ\u{ccd}ಯಕಾರ\u{ccd}ಟಾದ ವ\u{cbf}ಶೇಷ ಪ\u{ccd}ರದೇಶ"), ("ko", "욕야카르타"), ("lt", "Džokjakarta"), ("lv", "Džojakarta speciālais reģions"), ("ml", "സ\u{d4d}പെഷ\u{d4d}യൽ റീജിയൻ ഓഫ\u{d4d} യോഗ\u{d4d}യ\u{d3e}കർത\u{d4d}ത"), ("mr", "योग\u{94d}यताकाचा विश\u{947}ष प\u{94d}रद\u{947}श"), ("ms", "Daerah Istimewa Yogyakarta"), ("nb", "Yogyakarta"), ("nl", "Jogjakarta"), ("no", "Yogyakarta"), ("pl", "Yogyakarta"), ("pt", "Yogyakarta"), ("ro", "Yogyakarta"), ("ru", "Джокьякарта"), ("si", "යෝග\u{dca}\u{200d}යකර\u{dca}ට\u{dcf} ව\u{dd2}ශේෂ\u{dd2}ත කල\u{dcf}පය"), ("sk", "Yogyakarta"), ("sl", "Yogyakarta"), ("sr", "Џогџакарта"), ("sr_Latn", "Džogdžakarta"), ("sv", "Yogyakarta"), ("sw", "Yogyakarta"), ("ta", "யோக\u{bcd}யகர\u{bcd}த\u{bcd}த\u{bbe} சிறப\u{bcd}புப\u{bcd} பகுதி"), ("te", "స\u{c4d}ప\u{c46}షల\u{c4d} ర\u{c40}జ\u{c3f}యన\u{c4d} ఆఫ\u{c4d} య\u{c4b}గ\u{c4d}యక\u{c3e}ర\u{c4d}ట\u{c3e}"), ("th", "เขตปกครองพ\u{e34}เศษย\u{e47}อกยาการ\u{e4c}ตา"), ("tr", "Yogyakarta Özel Bölgesi"), ("uk", "Джокʼякарта"), ("ur", "خصوصی علاقہ یوگیاکارتا"), ("vi", "Yogyakarta"), ("yue", "日惹特區"), ("yue_Hans", "日惹特区"), ("zh", "日惹特区"), ("zu", "Yogyakarta")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "id")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::ID,
        alpha3: Alpha3::IDN,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}} {{postalcode}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 62,
        currency_code: "IDR",
        gec: Some(GEC::ID),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "001",
        ioc: Some(IOC::INA),
        iso_long_name: "The Republic of Indonesia",
        iso_short_name: "Indonesia",
        official_language_list: ["id"].to_vec(),
        spoken_language_list: ["id"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9, 10, 11].to_vec(),
        national_prefix: "0",
        nationality: Some("Indonesian"),
        number: "360",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "ID",
        unofficial_name_list: ["Indonesia", "Indonesien", "Indonésie", "インドネシア", "Indonesië"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Indonesia"), ("af", "Indonesië"), ("ak", "Indonesia"), ("am", "ጒን፦ኔፒ።"), ("an", "Indonesia"), ("ar", "إندونيسيا"), ("as", "ইণ\u{9cd}ডোনেছিয়\u{9be}"), ("ay", "Indonesia"), ("az", "İndoneziya"), ("ba", "Indonesia"), ("be", "Інданезія"), ("bg", "Индонезия"), ("bi", "Indonesia"), ("bn", "ইন\u{9cd}দোনেশিয়\u{9be}"), ("bn_IN", "ইন\u{9cd}দোনেশিয়\u{9be}"), ("br", "Indonezia"), ("bs", "Indonezija"), ("ca", "Indonèsia"), ("ce", "Индонези"), ("ch", "Indonesia"), ("cs", "Indonésie"), ("cv", "Индонези"), ("cy", "Indonesia"), ("da", "Indonesien"), ("de", "Indonesien"), ("dv", "އ\u{7a8}ނ\u{7b0}ޑ\u{7ae}ނ\u{7ad}ޝ\u{7a8}ޔ\u{7a7}"), ("dz", "ཨ\u{f72}ན་ཌ\u{f7c}་ན\u{f7a}་ཤ\u{f72}་ཡ།"), ("ee", "Indonesia"), ("el", "Ινδονησία"), ("en", "Indonesia"), ("eo", "Indonezio"), ("es", "Indonesia"), ("et", "Indoneesia"), ("eu", "Indonesia"), ("fa", "اندونزی"), ("ff", "Indonesia"), ("fi", "Indonesia"), ("fo", "Indonesia"), ("fr", "Indonésie"), ("fy", "Yndoneezje"), ("ga", "An Indinéis"), ("gl", "Indonesia"), ("gn", "Indonesia"), ("gu", "ઇન\u{acd}ડોન\u{ac7}શિયા"), ("gv", "Yn Indoneesh"), ("ha", "Indonesiya"), ("he", "אינדונזיה"), ("hi", "इ\u{902}डोन\u{947}शिया"), ("hr", "Indonezija"), ("ht", "Endonezi"), ("hu", "Indonézia"), ("hy", "Ինդոնեզիա"), ("ia", "Indonesia"), ("id", "Indonesia"), ("io", "Indonezia"), ("is", "Indónesía"), ("it", "Indonesia"), ("iu", "ᐄᓅᓯᐊ"), ("ja", "インドネシア"), ("ka", "ინდონეზია"), ("ki", "Indonesia"), ("kk", "Индонезия"), ("kl", "Indonesia"), ("km", "ឥណ\u{17d2}ឌ\u{17bc}នេស\u{17ca}\u{17b8}"), ("kn", "ಇಂಡೋನೇಶ\u{cbf}ಯಾ"), ("ko", "인도네시아"), ("ku", "Endonezya"), ("kv", "Индонезия"), ("kw", "Indonesi"), ("ky", "Индонезия"), ("lo", "ປະເທດອ\u{eb4}ນໂດເນເຊຍ"), ("lt", "Indonezija"), ("lv", "Indonēzija"), ("mi", "Initonīhia"), ("mk", "Индонезија"), ("ml", "ഇന\u{d4d}തോനേഷ\u{d4d}യ"), ("mn", "Индонез"), ("mr", "इ\u{902}डोन\u{947}शिया"), ("ms", "Indonesia"), ("mt", "Indoneżja"), ("my", "အင\u{103a}ဒ\u{102d}\u{102f}န\u{102e}းရ\u{103e}ားန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Indonitsiya"), ("nb", "Indonesia"), ("ne", "इन\u{94d}डोन\u{947}सिया"), ("nl", "Indonesië"), ("nn", "Indonesia"), ("nv", "Kéyah Dah Ndaaʼeełí Łání"), ("oc", "Indonesia"), ("or", "ଇଣ\u{b4d}ଡୋନେଶ\u{b3f}ୟ\u{b3e}"), ("pa", "ਇ\u{a70}ਡ\u{a4b}ਨ\u{a47}ਸ\u{a3c}ੀਆ"), ("pi", "इन\u{94d}दोन\u{947}शिया"), ("pl", "Indonezja"), ("ps", "اندونیزیا"), ("pt", "Indonésia"), ("pt_BR", "Indonésia"), ("ro", "Indonezia"), ("ru", "Индонезия"), ("rw", "Indonesiya"), ("sc", "Indonèsia"), ("sd", "انڊونيشيا"), ("si", "ඉන\u{dca}ද\u{dd4}න\u{dd3}ස\u{dd2}ය\u{dcf}ව"), ("sk", "Indonézia"), ("sl", "Indonezija"), ("so", "Indoneesiya"), ("sq", "Indonezi"), ("sr", "Индонезија"), ("sv", "Indonesien"), ("sw", "Indonesia"), ("ta", "இந\u{bcd}தோனேசிய\u{bbe}"), ("te", "ఇండ\u{c4b}న\u{c47}ష\u{c3f}య\u{c3e}"), ("tg", "Индонезия"), ("th", "อ\u{e34}นโดน\u{e35}เซ\u{e35}ย"), ("ti", "ኢንዶኔዢያ"), ("tk", "Indoneziýa"), ("tl", "Indonesia"), ("tr", "Endonezya"), ("tt", "İндонесиа"), ("ug", "ھىندونېزىيە"), ("uk", "Індонезія"), ("ur", "انڈونیشیا"), ("uz", "Indoneziya"), ("ve", "Indonesia"), ("vi", "Nam Dương"), ("wa", "Indonezeye"), ("wo", "Indoneesi"), ("xh", "Indonesia"), ("yo", "Indonésíà"), ("zh_CN", "印度尼西亚"), ("zh_HK", "印尼"), ("zh_TW", "印度尼西亞"), ("zu", "I-Indonesia")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
