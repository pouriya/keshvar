// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of China

#[cfg(all(feature = "tw", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::TW;
    pub const ALPHA3: Alpha3 = Alpha3::TWN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 886;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::TWD;
    pub const GEC: Option<GEC> = Some(GEC::TW);
    pub const INTERNATIONAL_PREFIX: &str = "002";
    pub const IOC: Option<IOC> = Some(IOC::TPE);
    pub const ISO_SHORT_NAME: &str = "Taiwan, Province of China";
    pub const ISO_LONG_NAME: &str = "The Republic of China";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["zh"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["zh"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Taiwanese");
    pub const NUMBER: &str = "158";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}(?:\\d{2,3})?");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "TW";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Taiwan", "Taiwán", "台灣"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Taiwan"),
        ("af", "Taiwan"),
        ("ak", "Taiwan"),
        ("am", "ታ፤ሒን፥"),
        ("an", "Taiwan"),
        ("ar", "تايوان"),
        ("as", "ত\u{9be}ইৱ\u{9be}ন"),
        ("ay", "Taiwan"),
        ("az", "Tayvan"),
        ("ba", "Taiwan"),
        ("be", "Тайвань"),
        ("bg", "Тайван"),
        ("bi", "Taiwan"),
        ("bn", "ত\u{9be}ইওয়\u{9be}ন"),
        ("bn_IN", "ত\u{9be}ইওয়\u{9be}ন"),
        ("br", "Taiwan"),
        ("bs", "Tajvan"),
        ("ca", "Taiwan"),
        ("ce", "Тайвань"),
        ("ch", "Taiwan"),
        ("cs", "Tchaj-wan"),
        ("cv", "Тайвань"),
        ("cy", "Taiwan"),
        ("da", "Taiwan"),
        ("de", "Taiwan"),
        (
            "dv",
            "ޖ\u{7aa}މ\u{7b0}ހ\u{7ab}ރ\u{7a9} ޗ\u{7a6}އ\u{7a8}ނ\u{7a7}",
        ),
        ("dz", "ཏ་ཡ\u{f7a}་ཝ\u{f71}ན།"),
        ("ee", "Taiwan"),
        ("el", "Ταϊβάν"),
        ("en", "Taiwan"),
        ("eo", "Tajvano"),
        ("es", "Taiwán"),
        ("et", "Hiina Vabariik"),
        ("eu", "Taiwan"),
        ("fa", "تایوان"),
        ("ff", "Taiwan"),
        ("fi", "Taiwan"),
        ("fo", "Teivan"),
        ("fr", "Taïwan"),
        ("fy", "Taiwan"),
        ("ga", "An Téaváin"),
        ("gl", "Taiwán"),
        ("gn", "Taiwan"),
        ("gu", "તાઇવાન"),
        ("gv", "Pobblaght ny Sheen"),
        ("ha", "Taiwan"),
        ("he", "טאיוואן"),
        ("hi", "ताइवान"),
        ("hr", "Tajvan"),
        ("ht", "Taywann"),
        ("hu", "Tajvan"),
        ("hy", "Թայվան"),
        ("ia", "Taiwan"),
        ("id", "Taiwan"),
        ("io", "Republiko di Chinia"),
        ("is", "Tævan"),
        ("it", "Taiwan"),
        ("iu", "Taiwan"),
        ("ja", "台湾"),
        ("ka", "ტაივანი"),
        ("ki", "Taiwan"),
        ("kk", "Тайвань аралы"),
        ("kl", "Taiwan"),
        ("km", "តៃវ\u{17c9}ាន\u{17cb}"),
        ("kn", "ಟೈವಾನ\u{ccd}"),
        ("ko", "타이완"),
        ("ku", "Taywan"),
        ("kv", "Taiwan"),
        ("kw", "Taywan"),
        ("ky", "Тайвань"),
        ("lo", "Tâi-oân"),
        ("lt", "Taivanas"),
        ("lv", "Taivāna"),
        ("mi", "Taiwana"),
        ("mk", "Тајван"),
        ("ml", "ത\u{d3e}യ\u{d4d}\u{200c}വ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Тайван"),
        ("mr", "त\u{948}वान"),
        ("ms", "Taiwan"),
        ("mt", "Tajwan"),
        (
            "my",
            "တရ\u{102f}တ\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Republik Tsiene"),
        ("nb", "Taiwan"),
        ("ne", "ताइवान"),
        ("nl", "Taiwan"),
        ("nn", "Taiwan"),
        ("nv", "Taiwan"),
        ("oc", "Taiwan"),
        ("or", "ତ\u{b3e}ଇଓ\u{b4d}ବ\u{b3e}ନ"),
        ("pa", "ਤਾਈਵਾਨ"),
        ("pi", "त\u{948}वान"),
        ("pl", "Tajwan"),
        ("ps", "د چین جمهوریت"),
        ("pt", "Taiwan"),
        ("pt_BR", "Taiwan"),
        ("ro", "Taiwan"),
        ("ru", "Тайвань"),
        ("rw", "Tayiwani"),
        ("sc", "Taiwan"),
        ("sd", "Taiwan"),
        ("si", "ත\u{dcf}ය\u{dd2}ව\u{dcf}නය"),
        ("sk", "Taiwan"),
        ("sl", "Tajvan"),
        ("so", "Taiwan"),
        ("sq", "Tajvan"),
        ("sr", "Тајван"),
        ("sv", "Taiwan"),
        ("sw", "Taiwan"),
        ("ta", "த\u{bbe}ய\u{bcd}வ\u{bbe}ன\u{bcd}"),
        ("te", "త\u{c48}వ\u{c3e}న\u{c4d}"),
        ("tg", "Тайван"),
        ("th", "ไต\u{e49}หว\u{e31}น"),
        ("ti", "ታይዋን"),
        ("tk", "Taýwan"),
        ("tl", "Taiwan"),
        ("tr", "Tayvan"),
        ("tt", "Тайван"),
        ("ug", "تەيۋەن"),
        ("uk", "Тайвань"),
        ("ur", "جمہوریۂ چین"),
        ("uz", "Xitoy Respublikasi"),
        ("ve", "Taiwan"),
        ("vi", "Đài Loan"),
        ("wa", "Taiwan"),
        ("wo", "Taaywaan"),
        ("xh", "Taiwan"),
        ("yo", "Orílẹ\u{300}-èdè Olómìnira ilẹ\u{300} Ṣáínà"),
        ("zh_CN", "台湾"),
        ("zh_HK", "台灣"),
        ("zh_TW", "臺灣"),
        ("zu", "I-Tayiwani"),
    ];
    #[cfg(all(feature = "tw", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 23.69781;
        pub const LONGITUDE: f64 = 120.960515;
        pub const MAX_LATITUDE: f64 = 26.4545;
        pub const MAX_LONGITUDE: f64 = 123.5021012;
        pub const MIN_LATITUDE: f64 = 20.5170001;
        pub const MIN_LONGITUDE: f64 = 116.6665;
        pub const NORTHEAST_LATITUDE: f64 = 26.4545;
        pub const NORTHEAST_LONGITUDE: f64 = 123.5021012;
        pub const SOUTHWEST_LATITUDE: f64 = 20.5170001;
        pub const SOUTHWEST_LONGITUDE: f64 = 116.6665;
    }
}
#[cfg(all(feature = "tw", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 23.69781,
            longitude: 120.960515,
            max_latitude: 26.4545,
            max_longitude: 123.5021012,
            min_latitude: 20.5170001,
            min_longitude: 116.6665,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 26.4545,
                    longitude: 123.5021012,
                },
                southwest: CountryGeoBound {
                    latitude: 20.5170001,
                    longitude: 116.6665,
                },
            },
        }
    }
}

#[cfg(all(feature = "tw", feature = "subdivisions"))]
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
                    "CHA",
                    Subdivision{
                        name: "CHA",
                        country_alpha2: Alpha2::TW,
                        code: "CHA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.0716583), longitude: Some(120.5624474), max_latitude: Some(24.1139398), min_latitude: Some(24.0377972), max_longitude: Some(120.6258758), min_longitude: Some(120.4997914)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشانغوا"), ("bg", "Чангхуа"), ("bn", "চ\u{9cd}য\u{9be}ংহ\u{9c1}য\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄌𑄚\u{11134}𑄊\u{1112a}𑄠"), ("ceb", "Changhua"), ("cs", "Okres Čang-chua"), ("cy", "Sir Changhua"), ("da", "Changhua County"), ("de", "Landkreis Changhua"), ("el", "Τσάνγκουα"), ("en", "Changhua"), ("es", "Condado de Changhua"), ("eu", "Changhua konderria"), ("fi", "Changhuan kunta"), ("fr", "Comté de Changhua"), ("gu", "ચા\u{a82}ગહ\u{ac1}આ કાઉન\u{acd}ટી"), ("hi", "चान\u{94d}ग\u{941}आ काउ\u{902}टी"), ("id", "Kabupaten Changhua"), ("it", "contea di Changhua"), ("ja", "彰化県"), ("kn", "ಚಂಗ\u{ccd}ವಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "장화 현"), ("lt", "Džanghua apskritis"), ("lv", "Džanhua apriņķis"), ("mn", "Жанхуа"), ("mr", "च\u{902}गह\u{941}आ काउ\u{902}टी"), ("ms", "Changhua County"), ("nb", "Changhua Fylke"), ("nl", "Changhua"), ("no", "Changhua Fylke"), ("pl", "Zhanghua"), ("pt", "Condado de Changhua"), ("ru", "Чжанхуа"), ("si", "චන\u{dca}ග\u{dca}හ\u{dd4}ආ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Changhus (ort)"), ("ta", "ச\u{bbe}ங\u{bcd}உஆ கவுண\u{bcd}டி"), ("te", "చ\u{c3e}ంగువ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "จางฮว\u{e48}า"), ("tr", "Changhua"), ("uk", "Округ Чжанхуа"), ("ur", "چانگہوا کاؤنٹی"), ("vi", "Chương Hóa"), ("yue", "彰化縣"), ("yue_Hans", "彰化县"), ("zh", "彰化縣")]),
                        unofficial_name_list: ["Changhua"].to_vec(),
                    }
                ),
                (
                    "CYI",
                    Subdivision{
                        name: "CYI",
                        country_alpha2: Alpha2::TW,
                        code: "CYI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.4871898), longitude: Some(120.4516708), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شياي"), ("bn", "চিয\u{9bc}\u{9be}য\u{9bc}ি ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄌\u{11128}𑄠𑄃\u{11128} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Chiayi"), ("cs", "Okres Ťia-i"), ("cy", "Sir Chiayi"), ("da", "Chiayi County"), ("de", "Landkreis Chiayi"), ("el", "Κομητεία Τσιαγί"), ("en", "Chiayi County"), ("es", "Condado del Chiayai"), ("eu", "Chiayi konderria"), ("fi", "Chiayin kunta"), ("fr", "Comté de Chiayi"), ("gu", "ચીઆયી કાઉન\u{acd}ટી"), ("hi", "चाय\u{947}ई काउ\u{902}टी"), ("id", "Kabupaten Chiayi"), ("it", "contea di Chiayi"), ("ja", "嘉義県"), ("kn", "ಚ\u{cbf}ಯಾಯ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "자이 현"), ("lt", "Dziaji apskritis"), ("lv", "Czai apgabals"), ("mn", "Жяи шянь"), ("mr", "चिआई काउ\u{902}टी"), ("ms", "Daerah Chiayi"), ("nb", "Chiayi Fylke"), ("nl", "Chiayi"), ("no", "Chiayi Fylke"), ("pl", "Jiayi"), ("pt", "Condado de Chiayi"), ("ru", "Цзяи"), ("si", "ච\u{dd2}ය\u{dcf}ය\u{dd2} පළ\u{dcf}ත"), ("sv", "Chiayi"), ("ta", "சிய\u{bbe}யி கவுண\u{bcd}டி"), ("te", "చ\u{c3f}య\u{c3e}య\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เจ\u{e35}ยอ\u{e35}\u{e49}"), ("tr", "Chiayi Country"), ("uk", "Округ Цзяі"), ("ur", "چیائی کاؤنٹی"), ("vi", "Gia Nghĩa"), ("yue", "嘉義縣"), ("yue_Hans", "嘉义县"), ("zh", "嘉義縣")]),
                        unofficial_name_list: ["Chiai"].to_vec(),
                    }
                ),
                (
                    "CYQ",
                    Subdivision{
                        name: "CYQ",
                        country_alpha2: Alpha2::TW,
                        code: "CYQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.4518428), longitude: Some(120.2554615), max_latitude: Some(23.6359268), min_latitude: Some(23.2148), max_longitude: Some(120.9576495), min_longitude: Some(120.1174411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شيا يي"), ("bn", "সিয\u{9bc}\u{9be}ই শহর"), ("ccp", "𑄌\u{11128}𑄠𑄃\u{11128}"), ("ceb", "Chiayi (lalawigan sa Republika sa Tsina, lat 23,48, long 120,45)"), ("cs", "Ťia-i"), ("da", "Chiayi"), ("de", "Chiayi"), ("el", "Τσιάγι"), ("en", "Chiayi"), ("es", "Chiayi"), ("eu", "Chiayi"), ("fa", "چیایای"), ("fi", "Chiayi City"), ("fr", "Chiayi"), ("gu", "ચિઆઇ શહ\u{ac7}ર"), ("he", "ג׳יאיי"), ("hi", "छिअई सिटी"), ("hu", "Csiaji"), ("id", "Kota Chiayi"), ("it", "Chiayi"), ("ja", "嘉義市"), ("kn", "ಚ\u{cbf}ಯಾಯ\u{cbf}"), ("ko", "자이 시"), ("lt", "Dziaji"), ("lv", "Dzjaji"), ("mn", "Жяи"), ("mr", "चिआई शहर"), ("ms", "Chiayi"), ("nb", "Chiayi"), ("nl", "Chiayi²"), ("no", "Chiayi"), ("pl", "Jiayi²"), ("pt", "Chiayi"), ("ru", "Цзяи²"), ("si", "ච\u{dd2}ය\u{dcf}ය\u{dd2} ස\u{dd2}ට\u{dd2}"), ("sv", "Jiayi"), ("ta", "சிய\u{bbe}இ நகரம\u{bcd}"), ("te", "చ\u{c3f}య\u{c3e}ంయ\u{c3f} స\u{c3f}ట\u{c40}"), ("th", "นครเจ\u{e35}ยอ\u{e35}\u{e49}"), ("tr", "Chiayi"), ("uk", "Цзяї"), ("ur", "چیایی شہر"), ("vi", "Gia Nghĩa²"), ("yue", "嘉義市"), ("yue_Hans", "嘉义市"), ("zh", "嘉義市")]),
                        unofficial_name_list: ["Chiayi"].to_vec(),
                    }
                ),
                (
                    "HSQ",
                    Subdivision{
                        name: "HSQ",
                        country_alpha2: Alpha2::TW,
                        code: "HSQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.8066493), longitude: Some(120.9669257), max_latitude: Some(24.8569956), min_latitude: Some(24.7125478), max_longitude: Some(121.0335449), min_longitude: Some(120.8794077)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هسينشو"), ("bn", "হেসিনচ\u{9c1} ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄥\u{11128}𑄚\u{11134}𑄌\u{1112a} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("cs", "Okres Sin-ču"), ("cy", "Sir Hsinchu"), ("da", "Hsinchu County"), ("de", "Landkreis Hsinchu"), ("el", "Χσίντσου"), ("en", "Hsinchu County"), ("es", "Condado de Hsinchu"), ("eu", "Hsinchu konderria"), ("fi", "Hsinchun kunta"), ("fr", "Comté de Hsinchu"), ("gu", "સિન\u{acd}ચ\u{ac1} કાઉન\u{acd}ટી"), ("hi", "सि\u{902}च\u{942} काउ\u{902}टी"), ("id", "Kabupaten Hsinchu"), ("it", "Contea di Hsinchu"), ("ja", "新竹県"), ("kn", "ಹ\u{cbf}ನ\u{ccd}ಚು ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "신주 현"), ("lt", "Sindžu apskritis"), ("lv", "Sjiņdžu apriņķis"), ("ml", "ഹ\u{d4d}സിഞ\u{d4d}ച\u{d41} ക\u{d57}ണ\u{d4d}ടി"), ("mn", "Шиньжү шянь"), ("mr", "सि\u{902}च\u{942} काउ\u{902}टी"), ("ms", "Hsinchu County"), ("nb", "Hsinchu Fylke"), ("nl", "Hsinchu"), ("no", "Hsinchu Fylke"), ("pl", "Xinzhu"), ("pt", "Condado de Hsinchu"), ("ru", "Синьчжу"), ("si", "ස\u{dd2}න\u{dca}ච\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Hsinchu (Län)"), ("ta", "ஹசிஞ\u{bcd}சு கவுண\u{bcd}டி"), ("te", "హ\u{c3f}స\u{c3f}ంచు క\u{c4c}ంట\u{c40}"), ("th", "ซ\u{e34}นจ\u{e39}\u{e4b}"), ("tr", "Hsinchu County"), ("uk", "Округ Сіньчжу"), ("ur", "ہسینچو کاؤنٹی"), ("vi", "Tân Trúc"), ("yue", "新竹縣"), ("yue_Hans", "新竹县"), ("zh", "新竹縣")]),
                        unofficial_name_list: ["Hsinchu"].to_vec(),
                    }
                ),
                (
                    "HSZ",
                    Subdivision{
                        name: "HSZ",
                        country_alpha2: Alpha2::TW,
                        code: "HSZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.9330791), longitude: Some(121.0093188), max_latitude: Some(24.9337171), min_latitude: Some(24.932182), max_longitude: Some(121.0100775), min_longitude: Some(121.0086104)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سين شو"), ("be", "Сіньчжу"), ("bn", "সিনসো"), ("ca", "Hsinchu"), ("ccp", "𑄥\u{11128}𑄚\u{11134}𑄌\u{1112a}"), ("ceb", "Hsinchu (kapital sa lalawigan)"), ("cs", "Sin-ču"), ("da", "Hsinchu"), ("de", "Hsinchu"), ("el", "Χσιντσού"), ("en", "Hsinchu"), ("es", "Hsinchu"), ("fa", "سینچو"), ("fi", "Hsinchu"), ("fr", "Hsinchu"), ("gu", "સિન\u{acd}ચ\u{ac1}"), ("he", "שינג׳ו"), ("hi", "सि\u{902}च\u{941}"), ("hu", "Hszincsu"), ("id", "Kota Hsinchu"), ("it", "Hsinchu"), ("ja", "新竹市"), ("kk", "Синьчжу"), ("kn", "ಶ\u{cbf}ಂಚು"), ("ko", "신주 시"), ("lt", "Sindžu"), ("lv", "Sjiņdžu"), ("mn", "Шиньжү"), ("mr", "ह\u{94d}सि\u{902}च\u{941}"), ("ms", "Hsinchu"), ("nb", "Hsinchu"), ("nl", "Hsinchu²"), ("no", "Hsinchu"), ("pl", "Xinzhu²"), ("pt", "Hsinchu"), ("ru", "Синьчжу²"), ("si", "ස\u{dd2}න\u{dca}ච\u{dd4}"), ("sv", "Hsinchu"), ("ta", "ஹெசின\u{bcd}ச\u{bcd}சூ"), ("te", "శ\u{c3f}ంచు"), ("th", "นครซ\u{e34}นจ\u{e39}\u{e4b}"), ("tr", "Hsinchu"), ("uk", "Сіньчжу"), ("ur", "ہسینچو"), ("vi", "Tân Trúc²"), ("yue", "新竹市"), ("yue_Hans", "新竹市"), ("zh", "新竹市")]),
                        unofficial_name_list: ["Hsinchu Municipality"].to_vec(),
                    }
                ),
                (
                    "HUA",
                    Subdivision{
                        name: "HUA",
                        country_alpha2: Alpha2::TW,
                        code: "HUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.9499886), longitude: Some(121.5518908), max_latitude: Some(24.37056), min_latitude: Some(23.0977983), max_longitude: Some(121.7714431), min_longitude: Some(120.9866267)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هوالين"), ("bn", "হ\u{9c1}য\u{9bc}\u{9be}লিয\u{9bc}েন ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄦\u{1112a}𑄠𑄣\u{11128}𑄠𑄬𑄚\u{11134}"), ("ceb", "Hualien (lalawigan sa Republika sa Tsina)"), ("cs", "Okres Chua-lien"), ("cy", "Sir Hualien"), ("da", "Hualien County"), ("de", "Landkreis Hualien"), ("el", "Χουάλιεν"), ("en", "Hualien"), ("es", "Condado de Hualien"), ("eu", "Hualien konderria"), ("fa", "هوالاین"), ("fi", "Hualienin kunta"), ("fr", "Comté d’Hualien"), ("gu", "હ\u{ac1}આલિયન કાઉન\u{acd}ટી"), ("hi", "ह\u{941}अलिएन काउ\u{902}टी"), ("id", "Kabupaten Hualien"), ("it", "Contea di Hualien"), ("ja", "花蓮県"), ("kn", "ಹುಲ\u{ccd}ಲ\u{cbf}ಯನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "화롄 현"), ("lt", "Hualiano apskritis"), ("lv", "Hualiaņas apriņķis"), ("mn", "Хуалянь"), ("mr", "ह\u{941}अलिएन काउ\u{902}टी"), ("ms", "Hualien County"), ("nb", "Hualien Fylke"), ("nl", "Hualien"), ("no", "Hualien Fylke"), ("pl", "Hualian"), ("pt", "Condado de Hualien"), ("ru", "Хуалянь"), ("si", "හ\u{dd4}ආල\u{dd2}යන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Hualien (län)"), ("ta", "ஹப\u{bcd}களின\u{bcd} கவுண\u{bcd}டி"), ("te", "హువ\u{c3e}ల\u{c3f}య\u{c46}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ฮวาเหล\u{e35}ยน"), ("tr", "Hualien"), ("uk", "Округ Гуалянь"), ("ur", "ہوالیئن کاؤنٹی"), ("vi", "Hoa Liên"), ("yue", "花蓮縣"), ("yue_Hans", "花莲县"), ("zh", "花蓮縣")]),
                        unofficial_name_list: ["Hualian"].to_vec(),
                    }
                ),
                (
                    "ILA",
                    Subdivision{
                        name: "ILA",
                        country_alpha2: Alpha2::TW,
                        code: "ILA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.7021073), longitude: Some(121.7377502), max_latitude: Some(25.7517095), min_latitude: Some(24.3094447), max_longitude: Some(123.4934282), min_longitude: Some(121.3167219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ييلان"), ("bn", "ইল\u{9be}ন ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄃\u{11128}𑄣𑄚\u{11134}"), ("ceb", "Yilan"), ("cs", "Okres I-lan"), ("cy", "Sir Yilan"), ("da", "Yilan County"), ("de", "Landkreis Ilan"), ("el", "Γιλάν"), ("en", "Yilan"), ("es", "Condado de Yilan"), ("eu", "Yilan konderria"), ("fa", "یایلان"), ("fi", "Yilanin kunta"), ("fr", "comté de Yilan"), ("gu", "યિલાન કાઉન\u{acd}ટી"), ("hi", "यिलान काउ\u{902}टी"), ("id", "Kabupaten Yilan"), ("it", "Contea di Yilan"), ("ja", "宜蘭県"), ("kn", "ಯ\u{cbf}ಲಾನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "이란 현"), ("lt", "Jilano apskritis"), ("lv", "Ilaņas distrikts"), ("mn", "Илань"), ("mr", "इलान काउ\u{902}टी"), ("ms", "Yilan County"), ("nb", "Yilan Fylke"), ("nl", "Yilan"), ("no", "Yilan Fylke"), ("pl", "Yilan"), ("pt", "Condado de Yilan"), ("ru", "Илань"), ("si", "ය\u{dd2}ලන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Yilan (län)"), ("ta", "இலன\u{bcd} கவுண\u{bcd}டி"), ("te", "య\u{c3f}ల\u{c3e}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "อ\u{e35}\u{e4b}หลาน"), ("tr", "Yilan"), ("uk", "Округ Ілань"), ("ur", "یلن کاؤنٹی، تائیوان"), ("vi", "Nghi Lan"), ("yue", "宜蘭縣"), ("yue_Hans", "宜兰县"), ("zh", "宜蘭縣")]),
                        unofficial_name_list: ["Ilan"].to_vec(),
                    }
                ),
                (
                    "KEE",
                    Subdivision{
                        name: "KEE",
                        country_alpha2: Alpha2::TW,
                        code: "KEE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.1313701), longitude: Some(121.7488675), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كي لنغ"), ("az", "Kiilunq"), ("bn", "কেল\u{9c1}ং"), ("ccp", "𑄇\u{11129}𑄣\u{1112a}\u{11101}"), ("ceb", "Keelung"), ("cs", "Ťi-lung"), ("da", "Keelung"), ("de", "Keelung"), ("el", "Κίλανγκ"), ("en", "Keelung"), ("es", "Keelung"), ("eu", "Keelung"), ("fa", "کیلانگ"), ("fi", "Keelung"), ("fr", "Chilung"), ("gu", "કીલ\u{a82}ગ"), ("he", "ג׳ילונג"), ("hi", "कील\u{902}ग"), ("hu", "Csilung"), ("id", "Kota Keelung"), ("it", "Keelung"), ("ja", "基隆市"), ("kn", "ಕೀಲುಂಗ\u{ccd}"), ("ko", "지룽 시"), ("lt", "Dzilongas"), ("lv", "Kēlunga"), ("mn", "Жилун"), ("mr", "किल\u{941}\u{902}ग"), ("ms", "Keelung"), ("nb", "Chilung"), ("nl", "Keelung"), ("no", "Chilung"), ("pl", "Keelung"), ("pt", "Keelung"), ("ru", "Цзилун"), ("si", "ක\u{dd3}ලං"), ("sv", "Chilung"), ("ta", "க\u{bc0}ளுங\u{bcd}"), ("te", "క\u{c40}లంగ\u{c4d}"), ("th", "นครจ\u{e35}หลง"), ("tr", "Keelung"), ("uk", "Цзилун"), ("ur", "کیلونگ"), ("vi", "Cơ Long"), ("yue", "基隆"), ("yue_Hans", "基隆"), ("zh", "基隆市")]),
                        unofficial_name_list: ["Chilung Shih"].to_vec(),
                    }
                ),
                (
                    "KHH",
                    Subdivision{
                        name: "KHH",
                        country_alpha2: Alpha2::TW,
                        code: "KHH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.6272784), longitude: Some(120.3014353), max_latitude: Some(23.4717267), min_latitude: Some(20.5862202), max_longitude: Some(121.0490147), min_longitude: Some(116.7118602)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kaohsiung"), ("ar", "كاوهسيونغ"), ("bg", "Гаосюн"), ("bn", "ত\u{9be}ইওয\u{9bc}\u{9be}ন শহর"), ("ca", "Kaohsiung"), ("ccp", "𑄇𑄃\u{1112e}𑄦\u{11134}𑄥\u{11128}𑄅\u{1112a}\u{11101}"), ("ceb", "Kaohsiung (kapital sa lalawigan)"), ("cs", "Kao-siung"), ("cy", "Dinas Kaohsiung"), ("da", "Kaohsiung"), ("de", "Kaohsiung"), ("el", "Καοσιούνγκ"), ("en", "Kaohsiung"), ("es", "Kaohsiung"), ("eu", "Kaohsiung"), ("fa", "کاوهسیونگ"), ("fi", "Kaohsiung"), ("fr", "Kaohsiung"), ("gl", "Kaohsiung"), ("gu", "કાઊશ\u{ac1}\u{a82}ગ શહ\u{ac7}ર"), ("he", "גאושיונג"), ("hi", "काऊश\u{941}\u{902}ग सिटी"), ("hu", "Kaohsziung"), ("hy", "Գաոսյուն"), ("id", "Kota Kaohsiung"), ("it", "Kaohsiung"), ("ja", "高雄市"), ("ka", "კაოჰსიუნგი"), ("kn", "ಕಾಒಹ\u{ccd}ಸ\u{ccd}ಯುಂಗ\u{ccd}"), ("ko", "가오슝 시"), ("ky", "Гаосун"), ("lt", "Gaosiongas"), ("lv", "Gaosjunas speciālā municipalitāte"), ("mk", "Гаосјунг"), ("mn", "Гаошюн"), ("mr", "काऊश\u{941}\u{902}ग शहर"), ("ms", "Kaohsiung"), ("my", "ကောင\u{103a}းရ\u{103e}\u{102f}\u{1036}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Kaohsiung"), ("nl", "Kaohsiung"), ("no", "Kaohsiung"), ("pa", "ਕਾਓਸੀਅ\u{a70}ਗ"), ("pl", "Kaohsiung"), ("pt", "Kaohsiung"), ("ro", "Kaohsiung"), ("ru", "Гаосюн"), ("si", "කොස\u{dd2}ය\u{dd4}න\u{dca}ග\u{dca}"), ("sk", "Kao-siung"), ("sv", "Kaohsiung"), ("sw", "Kaohsiung"), ("ta", "கோஷிங\u{bcd} நகரம\u{bcd}"), ("te", "క\u{c3e}వ\u{c4b}\u{c4b}స\u{c3f}యంగ\u{c4d} స\u{c3f}ట\u{c40}"), ("th", "เกาส\u{e3a}ยง"), ("tr", "Kaohsiung"), ("uk", "Гаосюн"), ("ur", "کائوسیونگ شہر"), ("vi", "Cao Hùng"), ("yue", "高雄市"), ("yue_Hans", "高雄市"), ("zh", "高雄市")]),
                        unofficial_name_list: ["Kaohsiung Special Municipality"].to_vec(),
                    }
                ),
                (
                    "KIN",
                    Subdivision{
                        name: "KIN",
                        country_alpha2: Alpha2::TW,
                        code: "KIN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كنمن"), ("az", "Szinmınsyundao"), ("bn", "কোয\u{9bc}েময\u{9bc}"), ("ca", "Kinmen"), ("ccp", "𑄇\u{11128}𑄚\u{11134}𑄟𑄬𑄚\u{11134}"), ("ceb", "Kinmen (lalawigan sa Republika sa Tsina)"), ("cs", "Okres Ťin-men"), ("da", "Kinmen"), ("de", "Kinmen"), ("el", "Κίνμεν"), ("en", "Kinmen"), ("es", "Kinmen"), ("et", "Jinmen"), ("eu", "Kinmen"), ("fa", "کینمن"), ("fi", "Kinmen"), ("fr", "Jinmen"), ("gu", "કિનમ\u{ac7}ન"), ("he", "קואמוי"), ("hi", "किन\u{94d}म\u{947}न"), ("hu", "Csinmen-szigetek"), ("id", "Kabupaten Kinmen"), ("it", "Kinmen"), ("ja", "金門県"), ("kn", "ಕ\u{cbf}ನ\u{ccd}ಮ\u{cc6}ನ\u{ccd}"), ("ko", "진먼 현"), ("lt", "Dzinmenas"), ("lv", "Dzjiņmeņa"), ("ml", "കിന\u{d4d}മെൻ"), ("mn", "Жиньмэнь"), ("mr", "किम\u{947}न"), ("ms", "Kinmen"), ("nb", "Kinmen"), ("nl", "Kinmen"), ("no", "Kinmen"), ("pl", "Kinmen"), ("pt", "Kinmen"), ("ru", "Цзиньмэнь"), ("si", "ක\u{dd2}න\u{dca}මෙන\u{dca}"), ("sv", "Kinmen"), ("ta", "கின\u{bcd}மேன\u{bcd}"), ("te", "క\u{c3f}న\u{c4d}మ\u{c46}న\u{c4d}"), ("th", "หม\u{e39}\u{e48}เกาะจ\u{e34}นเหม\u{e34}น"), ("tr", "Kinmen"), ("uk", "Цзіньмень"), ("ur", "کنمن"), ("vi", "Kim Môn"), ("yo", "Kinmen"), ("yo_BJ", "Kinmen"), ("yue", "金門"), ("yue_Hans", "金门"), ("zh", "金門縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LIE",
                    Subdivision{
                        name: "LIE",
                        country_alpha2: Alpha2::TW,
                        code: "LIE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄌\u{11128}𑄠𑄬\u{11101}"), ("ceb", "Lienchiang"), ("cs", "Okres Lien-ťiang"), ("cy", "Sir Lienchiang"), ("en", "Lienchiang"), ("fr", "comté de Lienchiang"), ("id", "Kepulauan Lienchiang"), ("ja", "連江県"), ("ko", "롄장 현"), ("yue", "連江"), ("yue_Hans", "连江"), ("zh", "連江縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MIA",
                    Subdivision{
                        name: "MIA",
                        country_alpha2: Alpha2::TW,
                        code: "MIA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.560159), longitude: Some(120.8214265), max_latitude: Some(24.741082), min_latitude: Some(24.2885293), max_longitude: Some(121.2626344), min_longitude: Some(120.6223738)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مياولي"), ("bn", "মিয\u{9bc}\u{9be}ওলি ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄟\u{1112d}𑄃\u{1112e}𑄣\u{11128}"), ("ceb", "Miaoli (lalawigan)"), ("cs", "Okres Miao-li"), ("cy", "Sir Miaoli"), ("da", "Miaoli County"), ("de", "Landkreis Miaoli"), ("el", "Μιαολί"), ("en", "Miaoli"), ("es", "Comté de Miaoli"), ("eu", "Miaoli konderria"), ("fi", "Miaolin kunta"), ("fr", "Comté de Miaoli"), ("gu", "મિયાઓલી કાઉન\u{acd}ટી"), ("hi", "मियाओली काउ\u{902}टी"), ("id", "Kabupaten Miaoli"), ("it", "contea di Miaoli"), ("ja", "苗栗県"), ("kn", "ಮ\u{cbf}ಯಾಲ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "먀오리 현"), ("lt", "Miaoli apskritis"), ("lv", "Miaoli apriņķis"), ("ml", "മിയോലി ക\u{d57}ണ\u{d4d}ടി"), ("mn", "Мяоли"), ("mr", "मियाओली काउ\u{902}टी"), ("ms", "Daerah Miaoli"), ("nb", "Miaoli Fylke"), ("nl", "Miaoli"), ("no", "Miaoli Fylke"), ("pl", "Miaoli"), ("pt", "Condado de Miaoli"), ("ru", "Мяоли"), ("si", "ම\u{dd2}ය\u{dcf}ඔල\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Miaoli County"), ("ta", "மிய\u{bbe}வ\u{bcd}லி கவுண\u{bcd}டி"), ("te", "మ\u{c3f}య\u{c3e}వ\u{c4b}ల\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เหม\u{e35}ยวล\u{e35}\u{e48}"), ("tr", "Miaoli County"), ("uk", "Округ Мяолі"), ("ur", "میاولی کاؤنٹی"), ("vi", "Miêu Lật"), ("yue", "苗栗縣"), ("yue_Hans", "苗栗县"), ("zh", "苗栗縣")]),
                        unofficial_name_list: ["Miaoli"].to_vec(),
                    }
                ),
                (
                    "NAN",
                    Subdivision{
                        name: "NAN",
                        country_alpha2: Alpha2::TW,
                        code: "NAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.9609981), longitude: Some(120.9718638), max_latitude: Some(24.2463089), min_latitude: Some(23.435385), max_longitude: Some(121.3494752), min_longitude: Some(120.6155614)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نانتو"), ("bn", "ন\u{9be}ন\u{9cd}ত\u{9c1} ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄚𑄚\u{11134}𑄑\u{1112f}"), ("ceb", "Nantou (lalawigan)"), ("cs", "Okres Nan-tchou"), ("cy", "Sir Nantou"), ("da", "Nantou County"), ("de", "Landkreis Nantou"), ("el", "Ναντού"), ("en", "Nantou"), ("es", "Condado del Nantou"), ("eu", "Nantou konderria"), ("fi", "Nantoun kunta"), ("fr", "Comté de Nantou"), ("gu", "નાન\u{acd}તોઉ કાઉન\u{acd}ટી"), ("hi", "न\u{948}नटो काउ\u{902}टी"), ("id", "Kabupaten Nantou"), ("it", "contea di Nantou"), ("ja", "南投県"), ("kn", "ನಾಂಟ\u{ccc} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "난터우 현"), ("lt", "Nantou apskritis"), ("lv", "Naņtou apriņķis"), ("mn", "Наньтоу"), ("mr", "न\u{945}नटोउ काउ\u{902}टी"), ("ms", "Nantou County"), ("nb", "Nantou fylk"), ("nl", "Nantou"), ("no", "Nantou fylk"), ("pl", "Nantou"), ("pt", "Condado de Nantou"), ("ru", "Наньтоу"), ("si", "නන\u{dca}ටෝඋ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Nantou (län)"), ("ta", "நண\u{bcd}டோவு கவுண\u{bcd}டி"), ("te", "న\u{c3e}ంట\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "หนานโถว"), ("tr", "Nantou"), ("uk", "Округ Наньтоу"), ("ur", "نانتوو کاؤنٹی"), ("vi", "Nam Đầu"), ("yue", "南投縣"), ("yue_Hans", "南投县"), ("zh", "南投縣")]),
                        unofficial_name_list: ["Nantou"].to_vec(),
                    }
                ),
                (
                    "NWT",
                    Subdivision{
                        name: "NWT",
                        country_alpha2: Alpha2::TW,
                        code: "NWT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄑\u{1112d}𑄛𑄬"), ("en", "New Taipei")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PEN",
                    Subdivision{
                        name: "PEN",
                        country_alpha2: Alpha2::TW,
                        code: "PEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.5711899), longitude: Some(119.5793157), max_latitude: Some(23.775773), min_latitude: Some(23.1865723), max_longitude: Some(119.7269552), min_longitude: Some(119.3144141)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بسكادورز"), ("ccp", "𑄛𑄬𑄚\u{11134}𑄊\u{1112a}"), ("cs", "Okres Pcheng-chu"), ("cy", "Sir Penghu"), ("da", "Pescadorerne"), ("de", "Landkreis Penghu"), ("en", "Penghu"), ("es", "Islas Pescadores"), ("fa", "پنگهو"), ("fi", "Peskadorit"), ("fr", "comté de Penghu"), ("id", "Kabupaten Penghu"), ("is", "Penghu"), ("it", "contea di Penghu"), ("ja", "澎湖県"), ("ko", "펑후 현"), ("ml", "പെൻഘ\u{d41}"), ("mn", "Пөнхү"), ("ms", "Penghu"), ("nl", "Pescadores"), ("pt", "Penghu"), ("ru", "Пэнху"), ("th", "เผ\u{e34}งห\u{e39}"), ("uk", "Пенху"), ("vi", "Bành Hồ"), ("yo", "Penghu"), ("yo_BJ", "Penghu"), ("yue", "澎湖縣"), ("yue_Hans", "澎湖县"), ("zh", "澎湖縣")]),
                        unofficial_name_list: ["Penghu"].to_vec(),
                    }
                ),
                (
                    "PIF",
                    Subdivision{
                        name: "PIF",
                        country_alpha2: Alpha2::TW,
                        code: "PIF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.5519759), longitude: Some(120.5487597), max_latitude: Some(22.8851844), min_latitude: Some(21.8968211), max_longitude: Some(120.9042007), min_longitude: Some(120.3527512)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بينغتونغ"), ("be", "Піндун"), ("bg", "Пингтунг"), ("bn", "পিংট\u{9c1}ং ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄛\u{11128}\u{11101}𑄑\u{11101}"), ("ceb", "Pingtung"), ("cs", "Okres Pching-tung"), ("cy", "Sir Pingtung"), ("da", "Pingtung County"), ("de", "Landkreis Pingtung"), ("el", "Πίνγκτουνγκ"), ("en", "Pingtung"), ("es", "Condado de Pingtung"), ("eu", "Pingtung konderria"), ("fa", "پینگتانگ"), ("fi", "Pingtungin kunta"), ("fr", "Comté de Pingtung"), ("gu", "પિ\u{a82}ગટ\u{a82}ગ કાઉન\u{acd}ટી"), ("hi", "पि\u{902}गट\u{942}\u{902}ग काउ\u{902}टी"), ("id", "Kabupaten Pingtung"), ("it", "Contea di Pingtung"), ("ja", "屏東県"), ("kn", "ಪ\u{cbf}ಂಗ\u{ccd}ಟಂಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "핑둥 현"), ("lt", "Pingdongo apskritis"), ("lv", "Pindunas apriņķis"), ("ml", "പിങ\u{d4d}ടങ\u{d4d} ക\u{d57}ണ\u{d4d}ടി"), ("mn", "Пиндун"), ("mr", "पि\u{902}गत\u{941}\u{902}ग काउ\u{902}टी"), ("ms", "Pingtung County"), ("nb", "Pingtung fylke"), ("nl", "Pingtung"), ("no", "Pingtung fylke"), ("pl", "Pingdong"), ("pt", "Condado de Pingtung"), ("ru", "Пиндун"), ("si", "ප\u{dd2}න\u{dca}ග\u{dca}ට\u{dd4}න\u{dca}ග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Pingtung (län)"), ("ta", "பிங\u{bcd}டுங\u{bcd} கவுண\u{bcd}டி"), ("te", "ప\u{c3f}ంగ\u{c4d}\u{200c}టుంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ผ\u{e34}งตง"), ("tr", "Pingtung"), ("uk", "Округ Пінтун"), ("ur", "پنگتونگ کاؤنٹی"), ("vi", "Bình Đông"), ("yue", "屏東縣"), ("yue_Hans", "屏东县"), ("zh", "屏東縣")]),
                        unofficial_name_list: ["Pingtung"].to_vec(),
                    }
                ),
                (
                    "TAO",
                    Subdivision{
                        name: "TAO",
                        country_alpha2: Alpha2::TW,
                        code: "TAO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.9936281), longitude: Some(121.3009798), max_latitude: Some(25.123137), min_latitude: Some(24.5864837), max_longitude: Some(121.4798128), min_longitude: Some(120.9845663)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Taoyuan"), ("ar", "مدينة تاويون"), ("bn", "ত\u{9be}ওউয\u{9bc}\u{9be}ন শহর"), ("ccp", "𑄑𑄃\u{1112e}𑄅\u{1112a}𑄠𑄚\u{11134}"), ("ceb", "Taoyuan"), ("cs", "Okres Tchao-jüan"), ("da", "Taoyuan City"), ("de", "Taoyuan"), ("el", "Ταογιουάν"), ("en", "Taoyuan"), ("es", "Taoyuan"), ("eu", "Taoyuang konderria"), ("fa", "تاویوان"), ("fi", "Taoyuan City"), ("fr", "comté de Taoyuan"), ("gu", "તાઓય\u{ac1}આન શહ\u{ac7}ર"), ("hi", "ताओय\u{941}आन सिटी"), ("hu", "Taojüan megye"), ("id", "Kabupaten Taoyuan"), ("it", "Taoyuan"), ("ja", "桃園市"), ("kn", "ತಾವೋಯ\u{cc2}ನ\u{ccd}"), ("ko", "타오위안 시"), ("lt", "Taojuano apskritis"), ("lv", "Taojuaņas speciālā municipalitāte"), ("mk", "Тоајуен"), ("mn", "Таоюань"), ("mr", "ताओय\u{941}आन शहर"), ("ms", "Taoyuan"), ("nb", "Toayuan By"), ("nl", "Taoyuan"), ("no", "Toayuan By"), ("pl", "Taoyuan"), ("pt", "Taoyuan"), ("ru", "Таоюань"), ("si", "ටොය\u{dd4}ආන\u{dca}"), ("sv", "Taoyuan"), ("ta", "த\u{bbe}வோயூய\u{bbe}ன\u{bcd} நகரம\u{bcd}"), ("te", "ట\u{c48}య\u{c4b}వ\u{c3e}న\u{c4d} నగరం"), ("th", "เถาหยวน"), ("tr", "Taoyuan"), ("uk", "Таоюань"), ("ur", "تاویوان"), ("vi", "Đào Viên"), ("yue", "桃園縣"), ("yue_Hans", "桃园县"), ("zh", "桃園市")]),
                        unofficial_name_list: ["Taoyuan"].to_vec(),
                    }
                ),
                (
                    "TNN",
                    Subdivision{
                        name: "TNN",
                        country_alpha2: Alpha2::TW,
                        code: "TNN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.0038497), longitude: Some(120.212227), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tainan"), ("ar", "تاينان"), ("be", "Горад Тайнань"), ("bn", "ত\u{9be}ইন\u{9be}ন"), ("ca", "Tainan"), ("ccp", "𑄑\u{1112d}𑄚𑄚\u{11134}"), ("cs", "Tchaj-nan"), ("da", "Tainan"), ("de", "Tainan"), ("el", "Ταϊνάν"), ("en", "Tainan"), ("es", "Tainan"), ("eu", "Tainan"), ("fa", "تاینان"), ("fi", "Tainan"), ("fr", "Tainan"), ("ga", "Tainan"), ("gu", "ત\u{ac8}નન"), ("he", "טאינאן"), ("hi", "ताइनान"), ("hu", "Tajnan"), ("hy", "Թայնան"), ("id", "Kota Tainan"), ("it", "Tainan"), ("ja", "台南市"), ("kn", "ತೈನ\u{cc6}ನ\u{ccd}"), ("ko", "타이난 시"), ("lt", "Tainanas"), ("lv", "Tainana"), ("mk", "Тајнан"), ("mn", "Тайнань"), ("mr", "ताईनान"), ("ms", "Tainan"), ("nb", "Tainan"), ("nl", "Tainan"), ("no", "Tainan"), ("pl", "Tainan"), ("pt", "Tainan"), ("ru", "Тайнань"), ("si", "ටය\u{dd2}නන\u{dca}"), ("sv", "Tainan"), ("ta", "டைன\u{bbe}ன\u{bcd}"), ("te", "ట\u{c3e}య\u{c3f}న\u{c3e}న\u{c4d}"), ("th", "ไถหน\u{e31}น"), ("tr", "Tainan"), ("uk", "Тайнань"), ("ur", "تاینان"), ("vi", "Đài Nam"), ("yue", "臺南"), ("yue_Hans", "台南"), ("zh", "臺南市")]),
                        unofficial_name_list: ["Tainan Municipality"].to_vec(),
                    }
                ),
                (
                    "TPE",
                    Subdivision{
                        name: "TPE",
                        country_alpha2: Alpha2::TW,
                        code: "TPE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.0329694), longitude: Some(121.5654177), max_latitude: Some(25.2443731), min_latitude: Some(24.7900797), max_longitude: Some(121.7300824), min_longitude: Some(121.2826735)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Taipei"), ("am", "ታይፔ"), ("ar", "تايبيه"), ("az", "Taybey"), ("be", "Тайбэй"), ("bg", "Тайпей"), ("bn", "ত\u{9be}ইপে"), ("bs", "Taipei"), ("ca", "Taipei"), ("ccp", "𑄑\u{1112d}𑄛𑄬"), ("ceb", "Taipei (ulohang dakbayan)"), ("cs", "Tchaj-pej"), ("cy", "Taipei"), ("da", "Taipei"), ("de", "Taipeh"), ("el", "Ταϊπέι"), ("en", "Taipei"), ("es", "Taipéi"), ("et", "Taipei"), ("eu", "Taipei"), ("fa", "تایپه"), ("fi", "Taipei"), ("fr", "Taipei"), ("ga", "Taipei"), ("gl", "Taipei"), ("gu", "તાયપ\u{ac7}ઈ"), ("ha", "Taipei"), ("ha_NE", "Taipei"), ("he", "טאיפיי"), ("hi", "ताइप\u{947}"), ("hr", "Taipei"), ("hu", "Tajpej"), ("hy", "Թայբեյ"), ("id", "Kota Taipei"), ("is", "Taípei"), ("it", "Taipei"), ("ja", "台北市"), ("jv", "Taipei"), ("ka", "ტაიბეი"), ("kk", "Тайбэй"), ("kn", "ತೈಪ\u{cc6}"), ("ko", "타이베이 시"), ("ky", "Тайбэй"), ("lt", "Taipėjus"), ("lv", "Taibei"), ("mk", "Тајпеј"), ("ml", "ത\u{d3e}യ\u{d4d}\u{200c}പെയ\u{d4d}"), ("mn", "Тайбэй"), ("mr", "ताइप\u{947}इ"), ("ms", "Taipei"), ("my", "ထ\u{102d}\u{102f}င\u{103a}ပေမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Taipei"), ("ne", "ताइप\u{947}इ"), ("nl", "Taipei"), ("no", "Taipei"), ("or", "ତ\u{b3e}ଇପେ"), ("pa", "ਤਾਈਪਈ"), ("pl", "Tajpej"), ("pt", "Taipé"), ("ro", "Taipei"), ("ru", "Тайбэй"), ("si", "ත\u{dcf}ය\u{dd2}පේ"), ("sk", "Tchaj-pej"), ("sl", "Tajpej"), ("so", "Taipei"), ("sq", "Taipei"), ("sr", "Тајпеј"), ("sr_Latn", "Tajpej"), ("sv", "Taipei"), ("sw", "Taipei"), ("ta", "த\u{bbe}ய\u{bcd}பெய\u{bcd}"), ("te", "త\u{c3e}య\u{c3f}ప\u{c46}య\u{c4d}"), ("th", "ไทเป"), ("tk", "Taýpeý"), ("tr", "Taipei"), ("uk", "Тайбей"), ("ur", "تائی پے"), ("uz", "Taypey"), ("vi", "Đài Bắc"), ("yo", "Taipei"), ("yo_BJ", "Taipei"), ("yue", "臺北市"), ("yue_Hans", "台北市"), ("zh", "臺北市")]),
                        unofficial_name_list: ["Taipei Special Municipality"].to_vec(),
                    }
                ),
                (
                    "TTT",
                    Subdivision{
                        name: "TTT",
                        country_alpha2: Alpha2::TW,
                        code: "TTT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.7972447), longitude: Some(121.0713702), max_latitude: Some(23.443723), min_latitude: Some(22.0009719), max_longitude: Some(121.601229), min_longitude: Some(120.7390581)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تايتونغ"), ("bg", "Таитунг"), ("bn", "ট\u{9be}ইট\u{9c1}ং ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄑\u{1112d}𑄑\u{1112a}\u{11101}"), ("ceb", "Taitung"), ("cs", "Okres Tchaj-tung"), ("cy", "Sir Taitung"), ("da", "Taitung County"), ("de", "Landkreis Taitung"), ("el", "Ταϊτούνγκ"), ("en", "Taitung"), ("es", "Condado del Taitung"), ("eu", "Taitung konderria"), ("fi", "Taitungn kunta"), ("fr", "Comté de Taitung"), ("gu", "ત\u{ac8}ત\u{ac1}\u{a82}ગ કાઉન\u{acd}ટી"), ("hi", "ताइत\u{941}\u{902}ग काउ\u{902}टी"), ("id", "Kabupaten Taitung"), ("it", "Contea di Taitung"), ("ja", "台東県"), ("kn", "ತೈಟಂಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "타이둥 현"), ("lt", "Taidongo apskritis"), ("lv", "Taidunas apriņķis"), ("mn", "Тайдун"), ("mr", "तायत\u{941}ग काउ\u{902}टी"), ("ms", "Taitung County"), ("nb", "Taitung Fylke"), ("nl", "Taitung"), ("no", "Taitung Fylke"), ("pl", "Taidong"), ("pt", "Condado de Taitung"), ("ru", "Тайдун"), ("si", "ටය\u{dd2}ට\u{dd4}න\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Taitung (län)"), ("ta", "டைட\u{bcd}டுங\u{bcd} கவுண\u{bcd}டி"), ("te", "ట\u{c48}టుంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ไถตง"), ("tr", "Taitung County"), ("uk", "Округ Тайдун"), ("ur", "تائیتونگ کاؤنٹی"), ("vi", "Đài Đông"), ("yue", "臺東縣"), ("yue_Hans", "台东县"), ("zh", "臺東縣")]),
                        unofficial_name_list: ["Taidong"].to_vec(),
                    }
                ),
                (
                    "TXG",
                    Subdivision{
                        name: "TXG",
                        country_alpha2: Alpha2::TW,
                        code: "TXG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.301983), longitude: Some(120.5854674), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Taichung"), ("ar", "تاي شانغ"), ("bn", "ত\u{9be}ইচোং"), ("ca", "Taichung"), ("ccp", "𑄑\u{1112d}𑄌\u{1112a}\u{11101}"), ("cs", "Tchaj-čung"), ("da", "Taichung"), ("de", "Taichung"), ("el", "Ταϊτσούνγκ"), ("en", "Taichung"), ("es", "Taichung"), ("fa", "تایچونگ"), ("fi", "Taichung"), ("fr", "Taichung"), ("ga", "Taichung"), ("gu", "તાઈચ\u{ac1}\u{a82}ગ"), ("he", "טאיג׳ונג"), ("hi", "ताइच\u{941}\u{902}ग"), ("hu", "Tajcshung"), ("id", "Kota Taichung"), ("it", "Taichung"), ("ja", "台中市"), ("kn", "ತಾಯ\u{ccd}ಚುಂಗ\u{ccd}"), ("ko", "타이중 시"), ("ky", "Тайчжун"), ("lt", "Taičungas"), ("lv", "Taidžuna"), ("mk", "Тајџунг"), ("mn", "Тайжун"), ("mr", "ताईच\u{941}\u{902}ग"), ("ms", "Taichung"), ("nb", "Taichung"), ("nl", "Taichung"), ("no", "Taichung"), ("pl", "Taizhong"), ("pt", "Taichung"), ("ro", "Taichung"), ("ru", "Тайчжун"), ("si", "ට\u{dcf}ය\u{dd2}චන\u{dca}ග\u{dca}"), ("sv", "Taichung"), ("ta", "த\u{bbe}ய\u{bcd}சங\u{bcd}"), ("te", "ట\u{c3e}య\u{c3f}చుంగ\u{c4d}"), ("th", "ไทจง"), ("tr", "Taichung"), ("uk", "Тайчжун"), ("ur", "ٹائچونگ"), ("vi", "Đài Trung"), ("yue", "臺中市"), ("yue_Hans", "台中市"), ("zh", "臺中市")]),
                        unofficial_name_list: ["Taichung Municipality"].to_vec(),
                    }
                ),
                (
                    "YUN",
                    Subdivision{
                        name: "YUN",
                        country_alpha2: Alpha2::TW,
                        code: "YUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.7092033), longitude: Some(120.4313373), max_latitude: Some(23.8421451), min_latitude: Some(23.5043147), max_longitude: Some(120.7362057), min_longitude: Some(120.132978)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يونلين"), ("bg", "Юнлин"), ("bn", "ইউনলিন ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄃\u{11128}𑄅\u{1112a}𑄚\u{11134}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Yunlin"), ("cs", "Okres Jün-lin"), ("cy", "Sir Yunlin"), ("da", "Yunlin County"), ("de", "Landkreis Yunlin"), ("el", "Γιούνλιν"), ("en", "Yunlin"), ("es", "Condado de Yunlin"), ("eu", "Yunlin konderria"), ("fi", "Yunlinin kunta"), ("fr", "comté de Yunlin"), ("gu", "ય\u{ac1}નલીન કાઉન\u{acd}ટી"), ("hi", "य\u{942}नलिन काउ\u{902}टी"), ("id", "Kabupaten Yunlin"), ("it", "contea di Yunlin"), ("ja", "雲林県"), ("kn", "ಯುನ\u{ccd}ಲ\u{cbf}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "윈린 현"), ("lt", "Junlino apskritis"), ("lv", "Junlinas distrikts"), ("mn", "Юньлинь"), ("mr", "य\u{941}निन काउ\u{902}टी"), ("ms", "Yunlin County"), ("nb", "Yunlin County"), ("nl", "Yunlin"), ("no", "Yunlin County"), ("pl", "Yunlin"), ("pt", "Condado de Yunlin"), ("ru", "Юньлинь"), ("si", "ය\u{dd4}න\u{dca}ල\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Yunlin County"), ("ta", "யூன\u{bcd}லின\u{bcd} கவுண\u{bcd}டி"), ("te", "యున\u{c4d}ల\u{c3f}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "หย\u{e38}นหล\u{e34}น"), ("tr", "Yunlin County"), ("uk", "Округ Юньлінь"), ("ur", "یونلن کاؤنٹی"), ("vi", "Vân Lâm"), ("yue", "雲林縣"), ("yue_Hans", "云林县"), ("zh", "雲林縣")]),
                        unofficial_name_list: ["Yunlin"].to_vec(),
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
#[cfg(feature = "tw")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::TW,
        alpha3: Alpha3::TWN,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 886,
        currency_code: CurrencyCode::TWD,
        gec: Some(GEC::TW),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "002",
        ioc: Some(IOC::TPE),
        iso_long_name: "The Republic of China",
        iso_short_name: "Taiwan, Province of China",
        official_language_list: ["zh"].to_vec(),
        spoken_language_list: ["zh"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "0",
        nationality: Some("Taiwanese"),
        number: "158",
        postal_code: true,
        postal_code_format: Some("\\d{3}(?:\\d{2,3})?"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "TW",
        unofficial_name_list: ["Taiwan", "Taiwán", "台灣"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Taiwan"),
            ("af", "Taiwan"),
            ("ak", "Taiwan"),
            ("am", "ታ፤ሒን፥"),
            ("an", "Taiwan"),
            ("ar", "تايوان"),
            ("as", "ত\u{9be}ইৱ\u{9be}ন"),
            ("ay", "Taiwan"),
            ("az", "Tayvan"),
            ("ba", "Taiwan"),
            ("be", "Тайвань"),
            ("bg", "Тайван"),
            ("bi", "Taiwan"),
            ("bn", "ত\u{9be}ইওয়\u{9be}ন"),
            ("bn_IN", "ত\u{9be}ইওয়\u{9be}ন"),
            ("br", "Taiwan"),
            ("bs", "Tajvan"),
            ("ca", "Taiwan"),
            ("ce", "Тайвань"),
            ("ch", "Taiwan"),
            ("cs", "Tchaj-wan"),
            ("cv", "Тайвань"),
            ("cy", "Taiwan"),
            ("da", "Taiwan"),
            ("de", "Taiwan"),
            (
                "dv",
                "ޖ\u{7aa}މ\u{7b0}ހ\u{7ab}ރ\u{7a9} ޗ\u{7a6}އ\u{7a8}ނ\u{7a7}",
            ),
            ("dz", "ཏ་ཡ\u{f7a}་ཝ\u{f71}ན།"),
            ("ee", "Taiwan"),
            ("el", "Ταϊβάν"),
            ("en", "Taiwan"),
            ("eo", "Tajvano"),
            ("es", "Taiwán"),
            ("et", "Hiina Vabariik"),
            ("eu", "Taiwan"),
            ("fa", "تایوان"),
            ("ff", "Taiwan"),
            ("fi", "Taiwan"),
            ("fo", "Teivan"),
            ("fr", "Taïwan"),
            ("fy", "Taiwan"),
            ("ga", "An Téaváin"),
            ("gl", "Taiwán"),
            ("gn", "Taiwan"),
            ("gu", "તાઇવાન"),
            ("gv", "Pobblaght ny Sheen"),
            ("ha", "Taiwan"),
            ("he", "טאיוואן"),
            ("hi", "ताइवान"),
            ("hr", "Tajvan"),
            ("ht", "Taywann"),
            ("hu", "Tajvan"),
            ("hy", "Թայվան"),
            ("ia", "Taiwan"),
            ("id", "Taiwan"),
            ("io", "Republiko di Chinia"),
            ("is", "Tævan"),
            ("it", "Taiwan"),
            ("iu", "Taiwan"),
            ("ja", "台湾"),
            ("ka", "ტაივანი"),
            ("ki", "Taiwan"),
            ("kk", "Тайвань аралы"),
            ("kl", "Taiwan"),
            ("km", "តៃវ\u{17c9}ាន\u{17cb}"),
            ("kn", "ಟೈವಾನ\u{ccd}"),
            ("ko", "타이완"),
            ("ku", "Taywan"),
            ("kv", "Taiwan"),
            ("kw", "Taywan"),
            ("ky", "Тайвань"),
            ("lo", "Tâi-oân"),
            ("lt", "Taivanas"),
            ("lv", "Taivāna"),
            ("mi", "Taiwana"),
            ("mk", "Тајван"),
            ("ml", "ത\u{d3e}യ\u{d4d}\u{200c}വ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Тайван"),
            ("mr", "त\u{948}वान"),
            ("ms", "Taiwan"),
            ("mt", "Tajwan"),
            (
                "my",
                "တရ\u{102f}တ\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Republik Tsiene"),
            ("nb", "Taiwan"),
            ("ne", "ताइवान"),
            ("nl", "Taiwan"),
            ("nn", "Taiwan"),
            ("nv", "Taiwan"),
            ("oc", "Taiwan"),
            ("or", "ତ\u{b3e}ଇଓ\u{b4d}ବ\u{b3e}ନ"),
            ("pa", "ਤਾਈਵਾਨ"),
            ("pi", "त\u{948}वान"),
            ("pl", "Tajwan"),
            ("ps", "د چین جمهوریت"),
            ("pt", "Taiwan"),
            ("pt_BR", "Taiwan"),
            ("ro", "Taiwan"),
            ("ru", "Тайвань"),
            ("rw", "Tayiwani"),
            ("sc", "Taiwan"),
            ("sd", "Taiwan"),
            ("si", "ත\u{dcf}ය\u{dd2}ව\u{dcf}නය"),
            ("sk", "Taiwan"),
            ("sl", "Tajvan"),
            ("so", "Taiwan"),
            ("sq", "Tajvan"),
            ("sr", "Тајван"),
            ("sv", "Taiwan"),
            ("sw", "Taiwan"),
            ("ta", "த\u{bbe}ய\u{bcd}வ\u{bbe}ன\u{bcd}"),
            ("te", "త\u{c48}వ\u{c3e}న\u{c4d}"),
            ("tg", "Тайван"),
            ("th", "ไต\u{e49}หว\u{e31}น"),
            ("ti", "ታይዋን"),
            ("tk", "Taýwan"),
            ("tl", "Taiwan"),
            ("tr", "Tayvan"),
            ("tt", "Тайван"),
            ("ug", "تەيۋەن"),
            ("uk", "Тайвань"),
            ("ur", "جمہوریۂ چین"),
            ("uz", "Xitoy Respublikasi"),
            ("ve", "Taiwan"),
            ("vi", "Đài Loan"),
            ("wa", "Taiwan"),
            ("wo", "Taaywaan"),
            ("xh", "Taiwan"),
            ("yo", "Orílẹ\u{300}-èdè Olómìnira ilẹ\u{300} Ṣáínà"),
            ("zh_CN", "台湾"),
            ("zh_HK", "台灣"),
            ("zh_TW", "臺灣"),
            ("zu", "I-Tayiwani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
