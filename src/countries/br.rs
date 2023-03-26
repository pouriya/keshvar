// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Federative Republic of Brazil

#[cfg(all(feature = "br", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::BR;
    pub const ALPHA3: Alpha3 = Alpha3::BRA;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 55;
    pub const CURRENCY_CODE: &str = "BRL";
    pub const GEC: Option<GEC> = Some(GEC::BR);
    pub const INTERNATIONAL_PREFIX: &str = "0014";
    pub const IOC: Option<IOC> = Some(IOC::BRA);
    pub const ISO_SHORT_NAME: &str = "Brazil";
    pub const ISO_LONG_NAME: &str = "The Federative Republic of Brazil";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10, 11];
    pub const NATIONAL_PREFIX: &str = "014";
    pub const NATIONALITY: Option<&str> = Some("Brazilian");
    pub const NUMBER: &str = "076";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}-?\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "BR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Brazil",
        "Brasilien",
        "Brésil",
        "Brasil",
        "ブラジル",
        "Brazilië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Brazil"),
        ("af", "Brasilië"),
        ("ak", "Brazil"),
        ("am", "ብሲፑሔ"),
        ("an", "Brasil"),
        ("ar", "البرازيل"),
        ("as", "ব\u{9cd}ৰ\u{9be}জিল"),
        ("ay", "Brazil"),
        ("az", "Braziliya"),
        ("ba", "Brazil"),
        ("be", "Бразілія"),
        ("bg", "Бразилия"),
        ("bi", "Brazil"),
        ("bn", "ব\u{9cd}র\u{9be}জিল"),
        ("bn_IN", "ব\u{9cd}র\u{9be}জিল"),
        ("br", "Brazil"),
        ("bs", "Brazil"),
        ("ca", "Brasil"),
        ("ce", "Бразили"),
        ("ch", "Brazil"),
        ("cs", "Brazílie"),
        ("cv", "Бразили"),
        ("cy", "Brasil"),
        ("da", "Brasilien"),
        ("de", "Brasilien"),
        ("dv", "ބ\u{7aa}ރ\u{7ac}ޒ\u{7a8}ލ\u{7b0}"),
        ("dz", "བ་ར་ཛ\u{f72}ལ།"),
        ("ee", "Brazil"),
        ("el", "Βραζιλία"),
        ("en", "Brazil"),
        ("eo", "Brazilo"),
        ("es", "Brasil"),
        ("et", "Brasiilia"),
        ("eu", "Brasil"),
        ("fa", "برزیل"),
        ("ff", "Barazil"),
        ("fi", "Brasilia"),
        ("fo", "Brasilia"),
        ("fr", "Brésil"),
        ("fy", "Brazylje"),
        ("ga", "An Bhrasaíl"),
        ("gl", "Brasil"),
        ("gn", "Brazil"),
        ("gu", "બ\u{acd}રાઝિલ"),
        ("gv", "Yn Vrasseel"),
        ("ha", "Brazil"),
        ("he", "ברזיל"),
        ("hi", "ब\u{94d}राज\u{93c}ील"),
        ("hr", "Brazil"),
        ("ht", "Brezil"),
        ("hu", "Brazília"),
        ("hy", "Բրազիլիա"),
        ("ia", "Brasil"),
        ("id", "Brasil"),
        ("io", "Brazilia"),
        ("is", "Brasilía"),
        ("it", "Brasile"),
        ("iu", "Brazil"),
        ("ja", "ブラジル"),
        ("ka", "ბრაზილია"),
        ("ki", "Brazil"),
        ("kk", "Бразилия"),
        ("kl", "Brazil"),
        ("km", "ប\u{17d2}រេស\u{17ca}\u{17b8}ល"),
        ("kn", "ಬ\u{ccd}ರಾಝ\u{cbf}ಲ\u{ccd}"),
        ("ko", "브라질"),
        ("ku", "Brezîlya"),
        ("kv", "Бразилия"),
        ("kw", "Brasil"),
        ("ky", "Бразилия"),
        ("lo", "ປະເທດບະເລຊ\u{eb4}ນ"),
        ("lt", "Brazilija"),
        ("lv", "Brazīlija"),
        ("mi", "Parīhi"),
        ("mk", "Бразил"),
        ("ml", "ബ\u{d4d}രസീല\u{d4d}\u{200d}"),
        ("mn", "Бразил"),
        ("mr", "ब\u{94d}राझिल"),
        ("ms", "Brazil"),
        ("mt", "Brażil"),
        ("my", "ဘရာဇ\u{102e}းန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Bradir"),
        ("nb", "Brasil"),
        ("ne", "ब\u{94d}राजिल"),
        ("nl", "Brazilië"),
        ("nn", "Brasil"),
        ("nv", "Kintaaʼanéhé Dineʼé Bikéyah"),
        ("oc", "Brasil"),
        ("or", "ବ\u{b4d}ର\u{b3e}ଜୀଲ"),
        ("pa", "ਬਰਾਜ\u{a3c}ੀਲ"),
        ("pi", "ब\u{94d}रासील"),
        ("pl", "Brazylia"),
        ("ps", "برازیل"),
        ("pt", "Brasil"),
        ("pt_BR", "Brasil"),
        ("ro", "Brazilia"),
        ("ru", "Бразилия"),
        ("rw", "Burezile"),
        ("sc", "Brasile"),
        ("sd", "برازيل"),
        ("si", "බ\u{dca}\u{200d}රස\u{dd2}ලය"),
        ("sk", "Brazília"),
        ("sl", "Brazilija"),
        ("so", "Braasiil"),
        ("sq", "Brazil"),
        ("sr", "Бразил"),
        ("sv", "Brasilien"),
        ("sw", "Brazil"),
        ("ta", "பிரேசில\u{bcd}"),
        ("te", "బ\u{c4d}ర\u{c46}జ\u{c3f}ల\u{c4d}"),
        ("tg", "Бразилия"),
        ("th", "บราซ\u{e34}ล"),
        ("ti", "ብራዚል"),
        ("tk", "Braziliýa"),
        ("tl", "Brasil"),
        ("tr", "Brezilya"),
        ("tt", "Бразил"),
        ("ug", "بىرازىلىيە"),
        ("uk", "Бразилія"),
        ("ur", "برازیل"),
        ("uz", "Braziliya"),
        ("ve", "Brazil"),
        ("vi", "Bra-xin"),
        ("wa", "Braezi"),
        ("wo", "Bareesil"),
        ("xh", "Brazil"),
        ("yo", "Brasil"),
        ("zh_CN", "巴西"),
        ("zh_HK", "巴西"),
        ("zh_TW", "巴西"),
        ("zu", "IBrazili"),
    ];
    #[cfg(all(feature = "br", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -14.235004;
        pub const LONGITUDE: f64 = -51.92528;
        pub const MAX_LATITUDE: f64 = 5.2717863;
        pub const MAX_LONGITUDE: f64 = -28.650543;
        pub const MIN_LATITUDE: f64 = -34.0891;
        pub const MIN_LONGITUDE: f64 = -73.9828169;
        pub const NORTHEAST_LATITUDE: f64 = 5.2717863;
        pub const NORTHEAST_LONGITUDE: f64 = -28.650543;
        pub const SOUTHWEST_LATITUDE: f64 = -34.0891;
        pub const SOUTHWEST_LONGITUDE: f64 = -73.9828169;
    }
}
#[cfg(all(feature = "br", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -14.235004,
            longitude: -51.92528,
            max_latitude: 5.2717863,
            max_longitude: -28.650543,
            min_latitude: -34.0891,
            min_longitude: -73.9828169,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 5.2717863,
                    longitude: -28.650543,
                },
                southwest: CountryGeoBound {
                    latitude: -34.0891,
                    longitude: -73.9828169,
                },
            },
        }
    }
}

#[cfg(all(feature = "br", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::BR,
                        code: "AC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.0237964), longitude: Some(-70.81199529999999), max_latitude: Some(-7.111826799999999), min_latitude: Some(-11.1452221), max_longitude: Some(-66.62407089999999), min_longitude: Some(-73.9915154)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Acre"), ("ar", "أكري"), ("az", "Akri"), ("be", "Штат Акры"), ("bg", "Акри"), ("bn", "একর"), ("bs", "Acre"), ("ca", "Estat d’Acre"), ("ccp", "𑄃𑄬𑄇\u{11127}𑄢\u{11134}"), ("ceb", "Acre"), ("cs", "Acre"), ("cy", "Acre"), ("da", "Acre"), ("de", "Acre"), ("el", "Άκρε"), ("en", "Acre"), ("es", "Estado de Acre"), ("et", "Acre osariik"), ("eu", "Acre"), ("fa", "اکری"), ("fi", "Acre"), ("fr", "Acre"), ("ga", "Acre"), ("gl", "Acre"), ("gu", "એકર"), ("he", "אקרי"), ("hi", "आक\u{94d}री"), ("hr", "Acre"), ("hu", "Acre"), ("id", "Acre"), ("is", "Acre"), ("it", "Acre"), ("ja", "アクレ州"), ("ka", "აკრი"), ("kn", "ಎಕರ\u{cc6}"), ("ko", "아크리 주"), ("lt", "Akrė"), ("lv", "Akri"), ("mk", "Акре"), ("ml", "അക\u{d4d}രേ"), ("mr", "आक\u{94d}र\u{947}"), ("ms", "Acre"), ("nb", "Acre"), ("nl", "Acre"), ("no", "Acre"), ("pl", "Acre"), ("pt", "Acre"), ("ro", "Acre"), ("ru", "Акри"), ("si", "අක\u{dca}රේ"), ("sk", "Acre"), ("sq", "Acre"), ("sr", "Акре"), ("sr_Latn", "Akre"), ("sv", "Acre"), ("sw", "Acre"), ("ta", "ஆக\u{bcd}ரி"), ("te", "ఏకర\u{c4d}"), ("th", "ร\u{e31}ฐอากร\u{e35}"), ("tr", "Acre"), ("uk", "Акрі"), ("ur", "اکری"), ("uz", "Akri"), ("vi", "Acre"), ("yo", "Acre"), ("yo_BJ", "Acre"), ("yue", "阿克里州"), ("yue_Hans", "阿克里州"), ("zh", "阿克里州")]),
                        unofficial_name_list: ["Acre"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "AL",
                        country_alpha2: Alpha2::BR,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.5713058), longitude: Some(-36.7819505), max_latitude: Some(-8.8131293), min_latitude: Some(-10.5037567), max_longitude: Some(-35.1522142), min_longitude: Some(-38.2273267)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alagoas"), ("ar", "ألاغواس"), ("az", "Alaqoas"), ("be", "штат Алагоас"), ("bg", "Алагоас"), ("bn", "আল\u{9be}গোয\u{9bc}\u{9be}স"), ("bs", "Alagoas"), ("ca", "Alagoas"), ("ccp", "𑄃𑄣𑄉𑄃\u{1112e}𑄌\u{11134}"), ("ceb", "Alagoas"), ("cs", "Alagoas"), ("cy", "Alagoas"), ("da", "Alagoas"), ("de", "Alagoas"), ("el", "Αλαγκόας"), ("en", "Alagoas"), ("es", "Alagoas"), ("et", "Alagoase osariik"), ("eu", "Alagoas"), ("fa", "آلاگواس"), ("fi", "Alagoas"), ("fr", "Alagoas"), ("ga", "Alagoas"), ("gl", "Alagoas"), ("gu", "આલાગોઆસ"), ("he", "אלגואס"), ("hi", "अलागोआस"), ("hr", "Alagoas"), ("hu", "Alagoas"), ("hy", "Ալագոաս"), ("id", "Alagoas"), ("is", "Alagoas"), ("it", "Alagoas"), ("ja", "アラゴアス州"), ("ka", "ალაგოასი"), ("kk", "Алагоас"), ("kn", "ಅಲಾಗೊಸ\u{ccd}"), ("ko", "알라고아스 주"), ("lt", "Alagoasas"), ("lv", "Alagoasa"), ("mk", "Алагоас"), ("mr", "आलागोआस"), ("ms", "Alagoas"), ("nb", "Alagoas"), ("ne", "अलागोआस"), ("nl", "Alagoas"), ("no", "Alagoas"), ("pl", "Alagoas"), ("pt", "Alagoas"), ("ro", "Alagoas"), ("ru", "Алагоас"), ("si", "අලගොආස\u{dca}"), ("sk", "Alagoas"), ("sq", "Alagoas"), ("sr", "Алагоас"), ("sr_Latn", "Alagoas"), ("sv", "Alagoas"), ("sw", "Alagoas"), ("ta", "அலகொல\u{bbe}ஸ\u{bcd}"), ("te", "ఆల\u{c3e}గ\u{c4b}వ\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐอาลาโกอ\u{e31}ส"), ("tr", "Alagoas"), ("uk", "Алагоас"), ("ur", "الاگواس"), ("uz", "Alagoas"), ("vi", "Alagoas"), ("yo", "Alagoas"), ("yo_BJ", "Alagoas"), ("yue", "阿拉瓜斯州"), ("yue_Hans", "阿拉瓜斯州"), ("zh", "阿拉戈斯州")]),
                        unofficial_name_list: ["Alagoas"].to_vec(),
                    }
                ),
                (
                    "AM",
                    Subdivision{
                        name: "AM",
                        country_alpha2: Alpha2::BR,
                        code: "AM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.4168427), longitude: Some(-65.8560646), max_latitude: Some(2.246628), min_latitude: Some(-9.8180491), max_longitude: Some(-56.0975519), min_longitude: Some(-73.8015533)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Amazonas"), ("ar", "الأمازون"), ("az", "Amazonas"), ("be", "Штат Амазонас"), ("bg", "Амазонас"), ("bn", "আম\u{9be}জোন\u{9be}স"), ("bs", "Amazonas"), ("ca", "Estat de l’Amazones"), ("ccp", "𑄃𑄟𑄎\u{11127}𑄚𑄌\u{11134}"), ("ceb", "Amazonas"), ("cs", "Amazonas"), ("cy", "Amazonas"), ("da", "Amazonas"), ("de", "Amazonas"), ("el", "Αμαζόνας"), ("en", "Amazonas"), ("es", "Estado de Amazonas"), ("et", "Amazonase osariik"), ("eu", "Amazonas"), ("fa", "آمازوناس"), ("fi", "Amazonas"), ("fr", "Amazonas"), ("ga", "Amazonas"), ("gl", "Estado do Amazonas"), ("gu", "એમ\u{ac7}ઝોનાઝ"), ("he", "אמזונאס"), ("hi", "आम\u{947}ज\u{93c}ोनास"), ("hr", "Amazonas"), ("hu", "Amazonas"), ("hy", "Ամազոնաս"), ("id", "Amazonas"), ("is", "Amazonas"), ("it", "Amazonas"), ("ja", "アマゾナス州"), ("ka", "ამაზონასი"), ("kn", "ಅಮ\u{cc6}ಜೋನಾಸ\u{ccd}"), ("ko", "아마조나스 주"), ("lt", "Amazonė"), ("lv", "Amazonasa"), ("mk", "Амазон"), ("mr", "अम\u{947}झोनास"), ("ms", "Amazonas"), ("nb", "Amazonas"), ("nl", "Amazonas"), ("no", "Amazonas"), ("pl", "Amazonas"), ("pt", "Amazonas"), ("ro", "Amazonas"), ("ru", "Амазонас"), ("si", "ඇමසෝන\u{dcf}ස\u{dca}"), ("sk", "Amazonas"), ("sq", "Amazonas"), ("sr", "Амазонас"), ("sr_Latn", "Amazonas"), ("sv", "Amazonas"), ("sw", "Amazonas"), ("ta", "அமேசோன\u{bbe}சு"), ("te", "అమ\u{c46}జ\u{c4b}న\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐอามาโซน\u{e31}ส"), ("tr", "Amazonas"), ("uk", "Амазонас"), ("ur", "ایمازوناس"), ("uz", "Amazonas"), ("vi", "Amazonas"), ("yo", "Amazonas"), ("yo_BJ", "Amazonas"), ("yue", "亞馬遜州"), ("yue_Hans", "亚马逊州"), ("zh", "亚马孙州")]),
                        unofficial_name_list: ["Amazonas"].to_vec(),
                    }
                ),
                (
                    "AP",
                    Subdivision{
                        name: "AP",
                        country_alpha2: Alpha2::BR,
                        code: "AP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.0447397), longitude: Some(-50.78742219999999), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Amapá"), ("ar", "أمابا"), ("az", "Amapa"), ("be", "Штат Амапа"), ("bg", "Амапа"), ("bn", "আম\u{9be}প\u{9be}"), ("bs", "Amapá"), ("ca", "Estat d’Amapá"), ("ccp", "𑄃𑄟\u{11134}𑄛"), ("ceb", "Amapá (estado)"), ("cs", "Amapá"), ("cy", "Amapá"), ("da", "Amapá"), ("de", "Amapá"), ("el", "Αμαπά"), ("en", "Amapá"), ("es", "Amapá"), ("et", "Amapá osariik"), ("eu", "Amapá"), ("fa", "آماپا"), ("fi", "Amapá"), ("fr", "Amapá"), ("ga", "Amapá"), ("gl", "Amapá"), ("gu", "અમાપા"), ("he", "אמאפה"), ("hi", "अमापा"), ("hr", "Amapá"), ("hu", "Amapá"), ("hy", "Ամապա"), ("id", "Amapá"), ("it", "Amapá"), ("ja", "アマパー州"), ("ka", "ამაპა"), ("kk", "Амапа"), ("kn", "ಅಮಪಾ"), ("ko", "아마파 주"), ("lt", "Amapa"), ("lv", "Amapa"), ("mk", "Амапа"), ("mr", "अमापा"), ("ms", "Amapá"), ("nb", "Amapá"), ("nl", "Amapá"), ("no", "Amapá"), ("pl", "Amapá"), ("pt", "Amapá"), ("ro", "Amapá"), ("ru", "Амапа"), ("si", "අමප\u{dcf}"), ("sk", "Amapá"), ("sq", "Amapá"), ("sr", "Амапа"), ("sr_Latn", "Amapa"), ("sv", "Amapá"), ("sw", "Amapá"), ("ta", "ஆமப\u{bcd}ப\u{bbe}"), ("te", "అమ\u{c3e}ప\u{c3e}"), ("th", "ร\u{e31}ฐอามาปา"), ("tr", "Amapá"), ("uk", "Амапа"), ("ur", "اماپا"), ("vi", "Amapá"), ("yo", "Amapá"), ("yo_BJ", "Amapá"), ("yue", "阿馬帕州"), ("yue_Hans", "阿马帕州"), ("zh", "阿马帕")]),
                        unofficial_name_list: ["Amapá"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::BR,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.579738), longitude: Some(-41.7007272), max_latitude: Some(-8.5328229), min_latitude: Some(-18.3359162), max_longitude: Some(-37.3484113), min_longitude: Some(-46.6170906)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bahia"), ("ar", "باهيا"), ("az", "Baiya"), ("be", "штат Баія"), ("bg", "Баия"), ("bn", "ব\u{9be}হিয\u{9bc}\u{9be}"), ("bs", "Bahia"), ("ca", "Estat de Bahia"), ("ccp", "𑄝𑄦\u{11128}𑄠"), ("ceb", "Bahia (estado)"), ("cs", "Bahia"), ("cy", "Bahia"), ("da", "Bahia"), ("de", "Bahia"), ("el", "Μπαΐα"), ("en", "Bahia"), ("es", "Estado de Bahía"), ("et", "Bahia osariik"), ("eu", "Bahiako estatua"), ("fa", "باهیا"), ("fi", "Bahia"), ("fr", "Bahia"), ("ga", "Bahia"), ("gl", "Bahia"), ("gu", "બહિઆ"), ("he", "באהיה"), ("hi", "बाहिया"), ("hr", "Bahia"), ("hu", "Bahia"), ("hy", "Իտամարաժու"), ("id", "Bahia"), ("is", "Bahia"), ("it", "Bahia"), ("ja", "バイーア州"), ("ka", "ბაია"), ("kk", "Баия"), ("kn", "ಬಹ\u{cbf}ಯ"), ("ko", "바이아 주"), ("lt", "Bahija"), ("lv", "Baija"), ("mk", "Баија"), ("mr", "बाईया"), ("ms", "Bahia"), ("nb", "Bahia"), ("nl", "Bahia"), ("no", "Bahia"), ("pa", "ਬਾਹੀਆ"), ("pl", "Bahia"), ("pt", "Bahia"), ("ro", "Bahia"), ("ru", "Баия"), ("si", "බහ\u{dd2}ය\u{dcf}"), ("sk", "Bahia"), ("sq", "Bahia"), ("sr", "Баија"), ("sr_Latn", "Baija"), ("sv", "Bahia"), ("sw", "Bahia"), ("ta", "ப\u{bbe}கைய\u{bbe}"), ("te", "బ\u{c3e}హ\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐบาเอ\u{e35}ย"), ("tr", "Bahia"), ("uk", "Баїя"), ("ur", "باہیا"), ("uz", "Baiya"), ("vi", "Bahia"), ("yo", "Bahia"), ("yo_BJ", "Bahia"), ("yue", "巴伊亞州"), ("yue_Hans", "巴伊亚州"), ("zh", "巴伊亚")]),
                        unofficial_name_list: ["Bahia"].to_vec(),
                    }
                ),
                (
                    "CE",
                    Subdivision{
                        name: "CE",
                        country_alpha2: Alpha2::BR,
                        code: "CE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.4983977), longitude: Some(-39.3206241), max_latitude: Some(-2.7844329), min_latitude: Some(-7.858185100000001), max_longitude: Some(-37.2530223), min_longitude: Some(-41.4235126)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ceará"), ("ar", "سيارا"), ("az", "Seara"), ("be", "Штат Сеара"), ("bg", "Сеара"), ("bn", "সিয\u{9bc}\u{9be}র\u{9be}"), ("bs", "Ceará"), ("ca", "Ceará"), ("ccp", "𑄥\u{11128}𑄠𑄬𑄢"), ("ceb", "Ceará (estado sa Brasil)"), ("cs", "Ceará"), ("cy", "Ceará"), ("da", "Ceará"), ("de", "Ceará"), ("el", "Σεαρά"), ("en", "Ceará"), ("es", "Ceará"), ("et", "Ceará osariik"), ("eu", "Ceará"), ("fa", "سئارا"), ("fi", "Ceará"), ("fr", "Ceará"), ("ga", "Ceará"), ("gl", "Ceará"), ("gu", "સિયારા"), ("he", "סיארה"), ("hi", "सियारा"), ("hr", "Ceará"), ("hu", "Ceará"), ("id", "Ceará"), ("is", "Ceará"), ("it", "Ceará"), ("ja", "セアラー州"), ("ka", "სეარა"), ("kk", "Сеара"), ("kn", "ಸೀರಾ"), ("ko", "세아라 주"), ("lt", "Seara"), ("lv", "Seara"), ("mk", "Сеара"), ("mr", "सियारा"), ("ms", "Ceará"), ("nb", "Ceará"), ("nl", "Ceará"), ("no", "Ceará"), ("pl", "Ceará"), ("pt", "Ceará"), ("ro", "Ceará"), ("ru", "Сеара"), ("si", "කෙය\u{dcf}ර\u{dcf}"), ("sk", "Ceará"), ("sr", "Сеара"), ("sr_Latn", "Seara"), ("sv", "Ceará"), ("sw", "Ceará"), ("ta", "சிய\u{bbe}ர\u{bbe}"), ("te", "స\u{c3f}య\u{c3e}ర\u{c3e}"), ("th", "ร\u{e31}ฐเซอารา"), ("tr", "Ceará"), ("uk", "Сеара"), ("ur", "سئیرا"), ("uz", "Seara"), ("vi", "Ceará"), ("yo", "Ceará"), ("yo_BJ", "Ceará"), ("yue", "塞阿拉州"), ("yue_Hans", "塞阿拉州"), ("zh", "塞阿腊")]),
                        unofficial_name_list: ["Ceará"].to_vec(),
                    }
                ),
                (
                    "DF",
                    Subdivision{
                        name: "DF",
                        country_alpha2: Alpha2::BR,
                        code: "DF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.826691), longitude: Some(-47.92182039999999), max_latitude: Some(-15.5001712), min_latitude: Some(-16.0517624), max_longitude: Some(-47.3081926), min_longitude: Some(-48.2870947)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FederalDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Federale Distrik"), ("ar", "القطاع الفدرالي البرازيلي"), ("az", "Federal dairə"), ("be", "Федэральная акруга Бразілія"), ("bg", "Федерален окръг"), ("bn", "ফেড\u{9be}রেল জেল\u{9be}"), ("ca", "Districte Federal del Brasil"), ("ccp", "𑄜𑄬𑄓𑄬𑄢𑄣\u{11134} 𑄎𑄬𑄣"), ("ceb", "Federal District"), ("cs", "Distrito Federal do Brasil"), ("cy", "Distrito Federal"), ("da", "Distrito Federal"), ("de", "Bundesdistrikt von Brasilien"), ("el", "Φεντεράλ"), ("en", "Federal District"), ("es", "Distrito Federal"), ("et", "Liiduringkond"), ("eu", "Barruti Federala"), ("fa", "ناحیه فدرال (برزیل)"), ("fi", "Distrito Federal"), ("fr", "District Fédéral"), ("gl", "Distrito Federal"), ("gu", "ફ\u{ac7}ડરલ જિલ\u{acd}લો"), ("he", "המחוז הפדרלי של ברזיל"), ("hi", "फ\u{947}डरल डिस\u{94d}ट\u{94d}रिक\u{94d}ट (ब\u{94d}राज\u{93c}ील)"), ("hr", "Brazilski federalni distrikt"), ("hu", "Szövetségi kerület"), ("id", "Distrik Federal Brasil"), ("it", "Distretto Federale"), ("ja", "ブラジリア連邦直轄区"), ("ka", "ფედერალური ოლქი"), ("kn", "ಫ\u{cc6}ಡರಲ\u{ccd} ಡ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ\u{ccd}ಟ\u{ccd}"), ("ko", "연방구"), ("lt", "Federalinė sritis"), ("lv", "Federālais distrikts"), ("mk", "Федерален округ"), ("mr", "शासकीय जिल\u{94d}हा"), ("ms", "Wilayah Persekutuan Brazil"), ("nb", "Distrito Federal"), ("nl", "Federaal District"), ("no", "Distrito Federal"), ("pl", "Dystrykt Federalny"), ("pt", "Distrito Federal"), ("ro", "Districtul Federal"), ("ru", "Федеральный округ"), ("si", "ෆෙඩරල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Distrito Federal do Brasil"), ("sq", "Distrikti Federal i Brazilit"), ("sr", "Савезни дистрикт Бразила"), ("sr_Latn", "Savezni distrikt Brazila"), ("sv", "Brasiliens federala distrikt"), ("ta", "கூட\u{bcd}டரசு ம\u{bbe}நிலம\u{bcd}"), ("te", "ఫ\u{c46}డరల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเฟอเดอร\u{e31}ล"), ("tr", "Federal Bölge"), ("uk", "Федеральний округ у Бразилії"), ("ur", "وفاقی ضلع"), ("uz", "Federal okrug"), ("vi", "Quận liên bang Brasil"), ("yo", "Brazilian Federal District"), ("yo_BJ", "Brazilian Federal District"), ("yue", "聯邦區"), ("yue_Hans", "联邦区"), ("zh", "聯邦區")]),
                        unofficial_name_list: ["Distrito Federal"].to_vec(),
                    }
                ),
                (
                    "ES",
                    Subdivision{
                        name: "ES",
                        country_alpha2: Alpha2::BR,
                        code: "ES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-19.1834229), longitude: Some(-40.3088626), max_latitude: Some(-17.891947), min_latitude: Some(-21.3017845), max_longitude: Some(-28.8359374), min_longitude: Some(-41.8797894)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Espírito Santo"), ("ar", "إسبيريتو سانتو"), ("az", "Espiritu-Santu"), ("be", "Штат Эспірыту-Санту"), ("bg", "Еспирито Санто"), ("bn", "এস\u{9cd}পিরিতো স\u{9cd}য\u{9be}ন\u{9cd}টো"), ("bs", "Espírito Santo"), ("ca", "Estat d’Espírito Santo"), ("ccp", "𑄃𑄬𑄌\u{11134}𑄛\u{11128}𑄢\u{11128}𑄑\u{1112e} 𑄥𑄚\u{11134}𑄑\u{1112e}"), ("ceb", "Espírito Santo (estado)"), ("cs", "Espírito Santo"), ("cy", "Espírito Santo"), ("da", "Espírito Santo"), ("de", "Espírito Santo"), ("el", "Εσπιρίτο Σάντο"), ("en", "Espírito Santo"), ("es", "Estado de Espírito Santo"), ("et", "Espírito Santo osariik"), ("eu", "Espírito Santo"), ("fa", "اسپیریتو سانتو"), ("fi", "Espírito Santo"), ("fr", "Espírito Santo"), ("ga", "Espírito Santo"), ("gl", "Espírito Santo"), ("gu", "એસપીરિટો સાન\u{acd}ટો"), ("he", "אספיריטו סאנטו"), ("hi", "एस\u{94d}पिरितो सान\u{94d}तो"), ("hr", "Espírito Santo"), ("hu", "Espírito Santo"), ("hy", "Էսպիրիտո Սանտո"), ("id", "Espírito Santo"), ("it", "Espírito Santo"), ("ja", "エスピリトサント州"), ("ka", "ესპირიტუ-სანტუ"), ("kn", "ಎಸ\u{ccd}ಪ\u{cbf}ರ\u{cbf}ಟೋ ಸ\u{ccd}ಯಾಂಟೋ"), ("ko", "이스피리투산투 주"), ("lt", "Espirito Santas"), ("lv", "Espiritu Santu"), ("mk", "Еспирито Санто"), ("mr", "एस\u{94d}पिरितो सा\u{902}तो"), ("ms", "Espírito Santo"), ("nb", "Espírito Santo"), ("nl", "Espírito Santo"), ("no", "Espírito Santo"), ("pl", "Espírito Santo"), ("pt", "Espírito Santo"), ("ro", "Espírito Santo"), ("ru", "Эспириту-Санту"), ("si", "එස\u{dca}ප\u{dd2}ර\u{dd2}ටෝ සැන\u{dca}ටෝ"), ("sk", "Espírito Santo"), ("sr", "Еспирито Санто"), ("sr_Latn", "Espirito Santo"), ("sv", "Espírito Santo"), ("sw", "Espírito Santo"), ("ta", "ஏசுபிரிட\u{bcd}டோ சந\u{bcd}தோ"), ("te", "ఎస\u{c4d}ప\u{c3f}ర\u{c3f}ట\u{c4b} స\u{c3e}ంట\u{c4b}"), ("th", "ร\u{e31}ฐเอสป\u{e35}ร\u{e35}ต\u{e39}ซานต\u{e39}"), ("tr", "Espírito Santo"), ("uk", "Еспіріту-Санту"), ("ur", "اسپیریتو سانتو"), ("uz", "Espiritu-Santu"), ("vi", "Espírito Santo"), ("yo", "Espírito Santo"), ("yo_BJ", "Espírito Santo"), ("yue", "聖埃斯皮里圖州"), ("yue_Hans", "圣埃斯皮里图州"), ("zh", "圣埃斯皮里图州")]),
                        unofficial_name_list: ["Espírito Santo"].to_vec(),
                    }
                ),
                (
                    "GO",
                    Subdivision{
                        name: "GO",
                        country_alpha2: Alpha2::BR,
                        code: "GO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.93397), longitude: Some(-50.1403832), max_latitude: Some(-15.9224628), min_latitude: Some(-15.9523406), max_longitude: Some(-50.1251296), min_longitude: Some(-50.1606536)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Goiás"), ("ar", "غوياس"), ("az", "Qoyas"), ("be", "Штат Гаяс"), ("bg", "Гояс"), ("bn", "গৌয\u{9bc}\u{9be}স"), ("bs", "Goiás"), ("ca", "Estat de Goiás"), ("ccp", "𑄉\u{1112e}𑄠𑄌\u{11134}"), ("ceb", "Goiás (estado)"), ("cs", "Goiás"), ("cy", "Goiás"), ("da", "Goiás"), ("de", "Goiás"), ("el", "Γκοϊάς"), ("en", "Goiás"), ("es", "Estado de Goiás"), ("et", "Goiási osariik"), ("eu", "Goiás"), ("fa", "گوییاس"), ("fi", "Goiás"), ("fr", "Goiás"), ("ga", "Goiás"), ("gl", "Goiás"), ("gu", "ગોયાસ"), ("he", "גויאס"), ("hi", "गोइयास"), ("hr", "Goiás"), ("hu", "Goiás"), ("hy", "Գոյաս"), ("id", "Goiás"), ("it", "Goiás"), ("ja", "ゴイアス州"), ("ka", "გოიასი"), ("kn", "ಗೋಯಾಸ\u{ccd}"), ("ko", "고이아스 주"), ("lt", "Gojasas"), ("lv", "Gojasa"), ("mk", "Гојас"), ("mr", "गोयाएस"), ("ms", "Goiás"), ("nb", "Goiás"), ("nl", "Goiás"), ("no", "Goiás"), ("pl", "Goiás"), ("pt", "Goiás"), ("ro", "Goiás"), ("ru", "Гояс"), ("si", "ගොය\u{dcf}ස\u{dca}"), ("sk", "Goiás"), ("sr", "Гојас"), ("sr_Latn", "Gojas"), ("sv", "Goiás"), ("sw", "Goiás"), ("ta", "கோய\u{bcd}அஸ\u{bcd}"), ("te", "గ\u{c4b}య\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐโกยาส"), ("tr", "Goiás"), ("uk", "Гояс"), ("ur", "گوئیاس"), ("uz", "Goyas"), ("vi", "Goiás"), ("yo", "Goiás"), ("yo_BJ", "Goiás"), ("yue", "戈亞斯州"), ("yue_Hans", "戈亚斯州"), ("zh", "戈亚斯")]),
                        unofficial_name_list: ["Goiás"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::BR,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.9609498), longitude: Some(-45.2744159), max_latitude: Some(-1.049999), min_latitude: Some(-10.2617676), max_longitude: Some(-41.7958785), min_longitude: Some(-48.7551446)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Maranhão"), ("ar", "مارانهاو"), ("az", "Maranyan"), ("be", "Штат Мараньян"), ("bg", "Мараняо"), ("bn", "মহ\u{9be}নহো"), ("bs", "Maranhão"), ("ca", "Estat de Maranhão"), ("ccp", "𑄟𑄢𑄚\u{11134}𑄦𑄃\u{1112e}"), ("ceb", "Maranhão (estado)"), ("cs", "Maranhão"), ("cy", "Maranhão"), ("da", "Maranhão"), ("de", "Maranhão"), ("el", "Μαρανιάο"), ("en", "Maranhão"), ("es", "Maranhão"), ("et", "Maranhão osariik"), ("eu", "Maranhão"), ("fa", "مارانیائو"), ("fi", "Maranhão"), ("fr", "Maranhão"), ("ga", "Maranhão"), ("gl", "Marañón - Maranhão"), ("gu", "મારાનહાઓ"), ("he", "מרניאו"), ("hi", "मरान\u{94d}हाओ"), ("hr", "Maranhão"), ("hu", "Maranhão"), ("hy", "Մարանյան"), ("id", "Maranhão"), ("it", "Maranhão"), ("ja", "マラニョン州"), ("ka", "მარანიანი"), ("kk", "Мараньян"), ("kn", "ಮಾರನ\u{ccd}ಹಾವೊ"), ("ko", "마라냥 주"), ("lt", "Maranjanas"), ("lv", "Maraņauna"), ("mk", "Марањао"), ("mr", "मरान\u{94d}याव"), ("ms", "Maranhão"), ("nb", "Maranhão"), ("nl", "Maranhão"), ("no", "Maranhão"), ("pl", "Maranhão"), ("pt", "Maranhão"), ("ro", "Maranhão"), ("ru", "Мараньян"), ("si", "මරන\u{dca}හ\u{dcf}ඕ"), ("sk", "Maranhão"), ("sr", "Марањао"), ("sr_Latn", "Maranjao"), ("sv", "Maranhão"), ("sw", "Maranhão"), ("ta", "ம\u{bbe}ரஞ\u{bcd}ஞோ"), ("te", "మర\u{c3e}న\u{c4d}హ\u{c4b}"), ("th", "ร\u{e31}ฐมาร\u{e31}นเยา"), ("tr", "Maranhão"), ("uk", "Мараньян"), ("ur", "مارانہاؤ"), ("uz", "Maranyan"), ("vi", "Maranhão"), ("yo", "Maranhão"), ("yo_BJ", "Maranhão"), ("yue", "馬拉尼昂州"), ("yue_Hans", "马拉尼昂州"), ("zh", "马拉尼昂州")]),
                        unofficial_name_list: ["Maranhão"].to_vec(),
                    }
                ),
                (
                    "MG",
                    Subdivision{
                        name: "MG",
                        country_alpha2: Alpha2::BR,
                        code: "MG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-18.512178), longitude: Some(-44.5550308), max_latitude: Some(-14.233183), min_latitude: Some(-22.9227576), max_longitude: Some(-39.8568263), min_longitude: Some(-51.0460748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Minas Gerais"), ("ar", "ميناس جيرايس"), ("az", "Minas-Jerays"), ("be", "штат Мінас-Жэрайс"), ("bg", "Минас Жерайс"), ("bn", "মিন\u{9be}স জের\u{9be}সিয\u{9bc}\u{9be}"), ("bs", "Minas Gerais"), ("ca", "Minas Gerais"), ("ccp", "𑄟\u{1112d}𑄚𑄌\u{11134} 𑄎𑄬𑄢\u{1112d}𑄌\u{11134}"), ("ceb", "Minas Gerais"), ("cs", "Minas Gerais"), ("cy", "Minas Gerais"), ("da", "Minas Gerais"), ("de", "Minas Gerais"), ("el", "Μίνας Ζεράις"), ("en", "Minas Gerais"), ("es", "Estado de Minas Gerais"), ("et", "Minas Geraisi osariik"), ("eu", "Minas Gerais"), ("fa", "میناس گرایس"), ("fi", "Minas Gerais"), ("fr", "Minas Gerais"), ("ga", "Minas Gerais"), ("gl", "Minas Xerais - Minas Gerais"), ("gu", "મિનાસ ગ\u{ac7}રઈસ"), ("he", "מינאס ז׳ראיס"), ("hi", "मिनास ज\u{947}रायज\u{93c}"), ("hr", "Minas Gerais"), ("hu", "Minas Gerais"), ("hy", "Մինաս Ժերաիս"), ("id", "Minas Gerais"), ("is", "Minas Gerais"), ("it", "Minas Gerais"), ("ja", "ミナスジェライス州"), ("ka", "მინას-ჟერაისი"), ("kk", "Минас-Жерайс"), ("kn", "ಮ\u{cbf}ನಾಸ\u{ccd} ಗ\u{cc6}ರೈಸ\u{ccd}"), ("ko", "미나스제라이스 주"), ("lt", "Minas Žeraisas"), ("lv", "Minasžeraisa"), ("mk", "Минас Жераис"), ("mr", "मिनास ज\u{947}राईस"), ("ms", "Minas Gerais"), ("nb", "Minas Gerais"), ("nl", "Minas Gerais"), ("no", "Minas Gerais"), ("pl", "Minas Gerais"), ("pt", "Minas Gerais"), ("ro", "Minas Gerais"), ("ru", "Минас-Жерайс"), ("si", "ම\u{dd2}න\u{dcf}ස\u{dca} ගෙර\u{dcf}ය\u{dd2}ස\u{dca}"), ("sk", "Minas Gerais"), ("sq", "Minas Gerais"), ("sr", "Минас Жераис"), ("sr_Latn", "Minas Žerais"), ("sv", "Minas Gerais"), ("sw", "Minas Gerais"), ("ta", "மின\u{bbe}ஸ\u{bcd} ஜெரைசு"), ("te", "మ\u{c3f}న\u{c3e}స\u{c4d} జ\u{c46}ర\u{c3e}య\u{c3f}స\u{c4d}"), ("th", "ร\u{e31}ฐม\u{e35}น\u{e31}สเชไรส\u{e4c}"), ("tr", "Minas Gerais"), ("uk", "Мінас-Жерайс"), ("ur", "میناس گیرائس"), ("uz", "Minas-jerais"), ("vi", "Minas Gerais"), ("yo", "Minas Gerais"), ("yo_BJ", "Minas Gerais"), ("yue", "米納吉拉斯州"), ("yue_Hans", "米纳吉拉斯州"), ("zh", "米纳斯吉拉斯")]),
                        unofficial_name_list: ["Minas Gerais"].to_vec(),
                    }
                ),
                (
                    "MS",
                    Subdivision{
                        name: "MS",
                        country_alpha2: Alpha2::BR,
                        code: "MS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-20.7722295), longitude: Some(-54.7851531), max_latitude: Some(-17.1666336), min_latitude: Some(-24.0561184), max_longitude: Some(-50.9229023), min_longitude: Some(-58.16705729999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mato Grosso do Sul"), ("ar", "ماتو غروسو دو سول"), ("az", "Matu-Qrosu-du-Sul"), ("be", "штат Мату-Гросу-ду-Сул"), ("bg", "Мато Гросо до Сул"), ("bn", "ম\u{9be}তো গ\u{9cd}রোস দো সৌল"), ("bs", "Mato Grosso do Sul"), ("ca", "Mato Grosso do Sul"), ("ccp", "𑄟𑄑\u{1112e} 𑄉\u{11133}𑄢𑄥\u{1112e} 𑄓\u{1112e} 𑄥\u{1112a}𑄣\u{11134}"), ("ceb", "Mato Grosso do Sul"), ("cs", "Mato Grosso do Sul"), ("cy", "Mato Grosso do Sul"), ("da", "Mato Grosso do Sul"), ("de", "Mato Grosso do Sul"), ("el", "Μάτο Γκρόσσο ντο Σούλ"), ("en", "Mato Grosso do Sul"), ("es", "Mato Grosso del Sur"), ("et", "Mato Grosso do Suli osariik"), ("eu", "Mato Grosso do Sul"), ("fa", "ماتوگروسو جنوبی"), ("fi", "Mato Grosso do Sul"), ("fr", "Mato Grosso do Sul"), ("ga", "Mato Grosso do Sul"), ("gl", "Mato Groso do Sur - Mato Grosso do Sul"), ("gu", "માટો ગ\u{acd}રોસો દો સ\u{ac1}લ"), ("he", "מאטו גרוסו דו סול"), ("hi", "मातो ग\u{94d}रोसो दो स\u{941}ल"), ("hr", "Mato Grosso do Sul"), ("hu", "Mato Grosso do Sul"), ("hy", "Մատու Գրոսու դու Սուլ"), ("id", "Mato Grosso do Sul"), ("it", "Mato Grosso do Sul"), ("ja", "マットグロッソ・ド・スル州"), ("ka", "მატუ-გროსუ-დუ-სული"), ("kn", "ಮ\u{ccd}ಯಾಟೊ ಗ\u{ccd}ರೊಸೊ ಡೊ ಸುಲ\u{ccd}"), ("ko", "마투그로수두술 주"), ("lt", "Pietų Mato Grosas"), ("lv", "Matugrosu du Sula"), ("mk", "Јужно Мато Гросо"), ("mr", "मातो ग\u{94d}रोसो दो स\u{941}ल"), ("ms", "Mato Grosso do Sul"), ("nb", "Mato Grosso do Sul"), ("nl", "Mato Grosso do Sul"), ("no", "Mato Grosso do Sul"), ("pl", "Mato Grosso do Sul"), ("pt", "Mato Grosso do Sul"), ("ro", "Mato Grosso do Sul"), ("ru", "Мату-Гросу-ду-Сул"), ("si", "මටෝ ග\u{dca}රෝසෝ ඩො සල\u{dca}"), ("sk", "Mato Grosso do Sul"), ("sq", "Mato Grosso do Sul"), ("sr", "Мато Гросо до Сул"), ("sr_Latn", "Mato Groso do Sul"), ("sv", "Mato Grosso do Sul"), ("sw", "Mato Grosso do Sul"), ("ta", "ம\u{bbe}டோ கிரோஸோ டூ சூல\u{bcd}"), ("te", "మ\u{c3e}ట\u{c4b} గ\u{c4d}ర\u{c3e}స\u{c4b} డ\u{c4b} సూల\u{c4d}"), ("th", "ร\u{e31}ฐมาต\u{e39}โกรสซ\u{e39}ด\u{e39}ซ\u{e39}ล"), ("tr", "Mato Grosso do Sul"), ("uk", "Мату-Гросу-ду-Сул"), ("ur", "جنوبی ماتو گروسو"), ("uz", "Matu-Grosu-du-Sul"), ("vi", "Mato Grosso do Sul"), ("yo", "Mato Grosso do Sul"), ("yo_BJ", "Mato Grosso do Sul"), ("yue", "南馬托格羅索州"), ("yue_Hans", "南马托格罗索州"), ("zh", "南马托格罗索州")]),
                        unofficial_name_list: ["Mato Grosso do Sul"].to_vec(),
                    }
                ),
                (
                    "MT",
                    Subdivision{
                        name: "MT",
                        country_alpha2: Alpha2::BR,
                        code: "MT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.6818712), longitude: Some(-56.921099), max_latitude: Some(-7.349037199999999), min_latitude: Some(-18.0420517), max_longitude: Some(-50.2247999), min_longitude: Some(-61.63319459999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mato Grosso"), ("ar", "ماتو غروسو"), ("az", "Matu-Qrosu"), ("be", "Штат Мату-Гросу"), ("bg", "Мато Гросо"), ("bn", "ম\u{9be}তো গ\u{9cd}রসো"), ("bs", "Mato Grosso"), ("ca", "Mato Grosso"), ("ccp", "𑄟𑄑\u{1112e} 𑄉\u{11133}𑄢𑄥\u{1112e}"), ("ceb", "Mato Grosso"), ("cs", "Mato Grosso"), ("cy", "Mato Grosso"), ("da", "Mato Grosso"), ("de", "Mato Grosso"), ("el", "Μάτου Γκρόσου"), ("en", "Mato Grosso"), ("es", "Mato Grosso"), ("et", "Mato Grosso osariik"), ("eu", "Mato Grosso"), ("fa", "ماتو گروسو"), ("fi", "Mato Grosso"), ("fr", "Mato Grosso"), ("ga", "Mato Grosso"), ("gl", "Mato Groso - Mato Grosso"), ("gu", "મ\u{ac7}ટો ગ\u{acd}રોસો"), ("he", "מאטו גרוסו"), ("hi", "मातो ग\u{94d}रोसो"), ("hr", "Mato Grosso"), ("hu", "Mato Grosso"), ("hy", "Մատու Գրոսու"), ("id", "Mato Grosso"), ("it", "Mato Grosso"), ("ja", "マットグロッソ州"), ("ka", "მატუ-გროსუ"), ("kk", "Мату-Гроссу"), ("kn", "ಮ\u{ccd}ಯಾಟೊ ಗ\u{ccd}ರೊಸೊ"), ("ko", "마투그로수 주"), ("lt", "Mato Grosas"), ("lv", "Matugrosu"), ("mk", "Мато Гросо"), ("mr", "मातो ग\u{94d}रोसो"), ("ms", "Mato Grosso"), ("nb", "Mato Grosso"), ("nl", "Mato Grosso"), ("no", "Mato Grosso"), ("pl", "Mato Grosso"), ("pt", "Mato Grosso"), ("ro", "Mato Grosso"), ("ru", "Мату-Гросу"), ("si", "මටෝ ග\u{dca}රෝසෝ"), ("sk", "Mato Grosso"), ("sr", "Мато Гросо"), ("sr_Latn", "Mato Groso"), ("sv", "Mato Grosso"), ("sw", "Mato Grosso"), ("ta", "மடோ குரோசோ"), ("te", "మ\u{c3e}ట\u{c4b} గ\u{c4d}ర\u{c4b}స\u{c4b}"), ("th", "ร\u{e31}ฐมาต\u{e39}โกรสซ\u{e39}"), ("tr", "Mato Grosso"), ("uk", "Мату-Гросу"), ("ur", "ماتو گروسو"), ("uz", "Matu-Grosu"), ("vi", "Mato Grosso"), ("yo", "Mato Grosso"), ("yo_BJ", "Mato Grosso"), ("yue", "馬托格羅索州"), ("yue_Hans", "马托格罗索州"), ("zh", "马托格罗索州")]),
                        unofficial_name_list: ["Mato Grosso"].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "PA",
                        country_alpha2: Alpha2::BR,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.9981271), longitude: Some(-54.9306152), max_latitude: Some(2.5910246), min_latitude: Some(-9.8411565), max_longitude: Some(-46.0643195), min_longitude: Some(-58.89827700000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pará"), ("ar", "بارا"), ("az", "Para"), ("be", "Пара"), ("bg", "Пара"), ("bn", "প\u{9cd}য\u{9be}\u{9be}র\u{9be}"), ("bs", "Pará"), ("ca", "Pará"), ("ccp", "𑄛𑄢"), ("ceb", "Pará (estado)"), ("cs", "Pará"), ("cy", "Pará"), ("da", "Pará"), ("de", "Pará"), ("el", "Παρά"), ("en", "Pará"), ("es", "Pará"), ("et", "Pará osariik"), ("eu", "Pará"), ("fa", "پارا"), ("fi", "Pará"), ("fr", "Pará"), ("ga", "Pará"), ("gl", "Pará"), ("gu", "પારા"), ("he", "פארה"), ("hi", "पारा"), ("hr", "Pará"), ("hu", "Pará"), ("hy", "Պարա"), ("id", "Pará"), ("it", "Pará"), ("ja", "パラー州"), ("ka", "პარა"), ("kn", "ಪ\u{ccd}ಯಾರಾ"), ("ko", "파라 주"), ("lt", "Para"), ("lv", "Para"), ("mk", "Пара"), ("mr", "पारा"), ("ms", "Pará"), ("nb", "Pará"), ("nl", "Pará"), ("no", "Pará"), ("pa", "ਪਾਰਾ"), ("pl", "Pará"), ("pt", "Pará"), ("ro", "Pará"), ("ru", "Пара"), ("si", "පර\u{dcf}"), ("sk", "Pará"), ("sq", "Pará"), ("sr", "Пара"), ("sr_Latn", "Para"), ("sv", "Pará"), ("sw", "Pará"), ("ta", "ப\u{bbe}ர\u{bbe}"), ("te", "పర\u{c3e}"), ("th", "ร\u{e31}ฐปารา"), ("tr", "Pará"), ("uk", "Пара"), ("ur", "پارا"), ("uz", "Para"), ("vi", "Pará"), ("yo", "Pará"), ("yo_BJ", "Pará"), ("yue", "帕拉州"), ("yue_Hans", "帕拉州"), ("zh", "帕拉")]),
                        unofficial_name_list: ["Pará"].to_vec(),
                    }
                ),
                (
                    "PB",
                    Subdivision{
                        name: "PB",
                        country_alpha2: Alpha2::BR,
                        code: "PB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.239960900000001), longitude: Some(-36.7819505), max_latitude: Some(-6.0259143), min_latitude: Some(-8.3029572), max_longitude: Some(-34.7931433), min_longitude: Some(-38.7656069)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Paraíba"), ("ar", "بارايبا"), ("az", "Paraiba"), ("be", "Штат Параіба"), ("bg", "Параиба"), ("bn", "প\u{9be}র\u{9be}য\u{9bc}ব\u{9be}"), ("bs", "Paraíba"), ("ca", "Paraíba"), ("ccp", "𑄛\u{11127}𑄢\u{1112d}𑄝"), ("ceb", "Paraíba"), ("cs", "Paraíba"), ("cy", "Paraíba"), ("da", "Paraíba"), ("de", "Paraíba"), ("el", "Παραΐμπα"), ("en", "Paraíba"), ("es", "Paraíba"), ("et", "Paraíba osariik"), ("eu", "Paraíba"), ("fa", "پارائیبا"), ("fi", "Paraíba"), ("fr", "Paraíba"), ("ga", "Paraíba"), ("gl", "Paraíba"), ("gu", "પ\u{ac7}રિબા"), ("he", "פאראיבה"), ("hi", "पर\u{947}बा"), ("hr", "Paraíba"), ("hu", "Paraíba"), ("hy", "Պարաիբա"), ("id", "Paraíba"), ("is", "Paraíba"), ("it", "Paraíba"), ("ja", "パライバ州"), ("ka", "პარაიბა"), ("kk", "Параиба"), ("kn", "ಪ\u{ccd}ಯಾರ\u{cbf}ಬಾ"), ("ko", "파라이바 주"), ("lt", "Paraiba"), ("lv", "Paraiba"), ("mk", "Параиба"), ("mr", "पर\u{948}बा"), ("ms", "Paraiba"), ("nb", "Paraíba"), ("nl", "Paraíba"), ("no", "Paraíba"), ("pl", "Paraíba"), ("pt", "Paraíba"), ("ro", "Paraíba"), ("ru", "Параиба"), ("si", "පරය\u{dd2}බ\u{dcf}"), ("sk", "Paraíba"), ("sr", "Параиба"), ("sr_Latn", "Paraiba"), ("sv", "Paraíba"), ("sw", "Paraíba"), ("ta", "பறைப\u{bbe}"), ("te", "ప\u{c3e}ర\u{c3e}య\u{c3f}బ\u{c3e}"), ("th", "ร\u{e31}ฐปาราอ\u{e35}บา"), ("tr", "Paraíba"), ("uk", "Параїба"), ("ur", "پارائیبا"), ("uz", "Paraiba"), ("vi", "Paraíba"), ("yo", "Paraíba"), ("yo_BJ", "Paraíba"), ("yue", "帕拉巴州"), ("yue_Hans", "帕拉巴州"), ("zh", "帕拉伊巴")]),
                        unofficial_name_list: ["Paraíba"].to_vec(),
                    }
                ),
                (
                    "PE",
                    Subdivision{
                        name: "PE",
                        country_alpha2: Alpha2::BR,
                        code: "PE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.8137173), longitude: Some(-36.954107), max_latitude: Some(-3.830501599999999), min_latitude: Some(-9.482900599999999), max_longitude: Some(-32.3918587), min_longitude: Some(-41.3583307)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pernambuco"), ("ar", "بيرنامبوكو"), ("az", "Pernambuku"), ("be", "штат Пернамбуку"), ("bg", "Пернамбуко"), ("bn", "পেরন\u{9be}ব\u{9c1}ক\u{9c1}"), ("bs", "Pernambuco"), ("ca", "Pernambuco"), ("ccp", "𑄛𑄢\u{11134}𑄚𑄟\u{11134}𑄝\u{1112a}𑄇\u{1112e}"), ("ceb", "Pernambuco"), ("cs", "Pernambuco"), ("cy", "Pernambuco"), ("da", "Pernambuco"), ("de", "Pernambuco"), ("el", "Περναμπούκο"), ("en", "Pernambuco"), ("es", "Pernambuco"), ("et", "Pernambuco osariik"), ("eu", "Pernambuco"), ("fa", "پرنامبوکو"), ("fi", "Pernambuco"), ("fr", "Pernambouc"), ("ga", "Pernambuco"), ("gl", "Pernambuco"), ("gu", "પરનામ\u{acd}બ\u{ac1}કો"), ("he", "פרנמבוקו"), ("hi", "प\u{947}रनाम\u{94d}ब\u{941}को"), ("hr", "Pernambuco"), ("hu", "Pernambuco"), ("hy", "Պերնամբուկու"), ("id", "Pernambuco"), ("is", "Pernambuco"), ("it", "Pernambuco"), ("ja", "ペルナンブーコ州"), ("ka", "პერნამბუკუ"), ("kk", "Пернамбуку"), ("kn", "ಪ\u{cc6}ರ\u{ccd}ನಂಬುಕೊ"), ("ko", "페르남부쿠 주"), ("lt", "Pernambukas"), ("lv", "Pernambuku"), ("mk", "Пернамбуко"), ("mr", "पर\u{94d}ना\u{902}ब\u{941}को"), ("ms", "Pernambuco"), ("nb", "Pernambuco"), ("nl", "Pernambuco"), ("no", "Pernambuco"), ("pl", "Pernambuco"), ("pt", "Pernambuco"), ("ro", "Pernambuco"), ("ru", "Пернамбуку"), ("si", "පර\u{dca}නම\u{dca}බ\u{dd4}කෝ"), ("sk", "Pernambuco"), ("sl", "Pernambuco, Brazilija"), ("sq", "Pernambuco"), ("sr", "Пернамбуко"), ("sr_Latn", "Pernambuko"), ("sv", "Pernambuco"), ("sw", "Pernambuco"), ("ta", "பெர\u{bcd}னம\u{bcd}புகோ"), ("te", "ప\u{c46}ర\u{c4d}న\u{c3e}ంబుక\u{c4b}"), ("th", "ร\u{e31}ฐเปร\u{e4c}น\u{e31}มบ\u{e39}ก\u{e39}"), ("tr", "Pernambuco"), ("uk", "Пернамбуку"), ("ur", "پرنامبوکو"), ("uz", "Pernambuku"), ("vi", "Pernambuco"), ("yo", "Pernambuco"), ("yo_BJ", "Pernambuco"), ("yue", "帕南布哥州"), ("yue_Hans", "帕南布哥州"), ("zh", "伯南布哥")]),
                        unofficial_name_list: ["Pernambuco"].to_vec(),
                    }
                ),
                (
                    "PI",
                    Subdivision{
                        name: "PI",
                        country_alpha2: Alpha2::BR,
                        code: "PI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.718340100000001), longitude: Some(-42.72892359999999), max_latitude: Some(-2.7473161), min_latitude: Some(-10.9287593), max_longitude: Some(-40.3705066), min_longitude: Some(-45.9942887)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Piauí"), ("ar", "بياوي"), ("az", "Piaui"), ("be", "Штат Піауі"), ("bg", "Пиауи"), ("bn", "পিয\u{9bc}\u{9be}উই"), ("bs", "Piauí"), ("ca", "Piauí"), ("ccp", "𑄛\u{1112d}𑄅\u{1112a}𑄃\u{11128}"), ("ceb", "Piauí"), ("cs", "Piauí"), ("cy", "Piauí"), ("da", "Piauí"), ("de", "Piauí"), ("el", "Πιοΐ"), ("en", "Piauí"), ("es", "Piauí"), ("et", "Piauí osariik"), ("eu", "Piauí"), ("fa", "پیاوی"), ("fi", "Piauí"), ("fr", "Piauí"), ("ga", "Piauí"), ("gl", "Piauí"), ("gu", "પિઆઉઈ"), ("he", "פיאאוי"), ("hi", "पियाउई"), ("hr", "Piauí"), ("hu", "Piauí"), ("hy", "Պիաուի"), ("id", "Piauí"), ("is", "Piauí"), ("it", "Piauí"), ("ja", "ピアウイ州"), ("ka", "პიაუი"), ("kk", "Пиауи"), ("kn", "ಪ\u{cbf}ಯಾಯ\u{cbf}"), ("ko", "피아우이 주"), ("lt", "Piaujis"), ("lv", "Pjaui"), ("mk", "Пјауи"), ("mr", "पिआवी"), ("ms", "Piauí"), ("nb", "Piauí"), ("nl", "Piauí"), ("no", "Piauí"), ("pl", "Piauí"), ("pt", "Piauí"), ("ro", "Piauí"), ("ru", "Пиауи"), ("si", "ප\u{dd2}යෞය\u{dd2}"), ("sk", "Piauí"), ("sr", "Пјауи"), ("sr_Latn", "Pjaui"), ("sv", "Piauí"), ("sw", "Piauí"), ("ta", "பிஒய\u{bcd}"), ("te", "ప\u{c3f}య\u{c3e}వ\u{c3e}"), ("th", "ร\u{e31}ฐป\u{e35}เอาอ\u{e35}"), ("tr", "Piauí"), ("uk", "Піауї"), ("ur", "پیاوی"), ("uz", "Piaui"), ("vi", "Piauí"), ("yo", "Piauí"), ("yo_BJ", "Piauí"), ("yue", "皮奧伊州"), ("yue_Hans", "皮奥伊州"), ("zh", "皮奧伊州")]),
                        unofficial_name_list: ["Piauí"].to_vec(),
                    }
                ),
                (
                    "PR",
                    Subdivision{
                        name: "PR",
                        country_alpha2: Alpha2::BR,
                        code: "PR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-25.2520888), longitude: Some(-52.0215415), max_latitude: Some(-22.5166644), min_latitude: Some(-26.7172983), max_longitude: Some(-48.0235303), min_longitude: Some(-54.6192979)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Paraná"), ("ar", "بارانا"), ("az", "Parana"), ("be", "штат Парана"), ("bg", "Парана"), ("bn", "প\u{9be}র\u{9be}ন\u{9be}"), ("bs", "Paraná"), ("ca", "Estat de Paranà"), ("ccp", "𑄛\u{11127}𑄢𑄚\u{11133}𑄦"), ("ceb", "Paraná (estado)"), ("cs", "Paraná"), ("cy", "Paraná"), ("da", "Paraná"), ("de", "Paraná"), ("el", "Παρανά"), ("en", "Paraná"), ("es", "Paraná"), ("et", "Paraná osariik"), ("eu", "Paraná"), ("fa", "پارانا، برزیل"), ("fi", "Paraná"), ("fr", "Paraná"), ("ga", "Paraná"), ("gl", "Estado do Paraná"), ("gu", "પરાના"), ("he", "פרנה"), ("hi", "पाराना"), ("hr", "Paraná"), ("hu", "Paraná"), ("hy", "Պարանա"), ("id", "Paraná"), ("is", "Paraná"), ("it", "Paraná"), ("ja", "パラナ州"), ("ka", "პარანა"), ("kn", "ಪರನಾ"), ("ko", "파라나 주"), ("lt", "Parana"), ("lv", "Parana"), ("mk", "Парана"), ("mr", "पाराना"), ("ms", "Paraná"), ("nb", "Paraná"), ("nl", "Paraná"), ("no", "Paraná"), ("pl", "Parana"), ("pt", "Paraná"), ("ro", "Paraná"), ("ru", "Парана"), ("si", "පරන\u{dcf}"), ("sk", "Paraná"), ("sr", "Парана"), ("sr_Latn", "Parana"), ("sv", "Paraná"), ("sw", "Paraná"), ("ta", "பர\u{bbe}ன\u{bbe}"), ("te", "పర\u{c3e}న\u{c3e}"), ("th", "ร\u{e31}ฐปารานา"), ("tr", "Paraná"), ("uk", "Парана"), ("ur", "پارانا"), ("uz", "Parana"), ("vi", "Paraná"), ("yo", "Paraná"), ("yo_BJ", "Paraná"), ("yue", "巴拉那州"), ("yue_Hans", "巴拉那州"), ("zh", "巴拉那州")]),
                        unofficial_name_list: ["Paraná"].to_vec(),
                    }
                ),
                (
                    "RJ",
                    Subdivision{
                        name: "RJ",
                        country_alpha2: Alpha2::BR,
                        code: "RJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-22.9068467), longitude: Some(-43.1728965), max_latitude: Some(-22.7461987), min_latitude: Some(-23.0763469), max_longitude: Some(-43.1018358), min_longitude: Some(-43.7950599)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rio de Janeiro"), ("ar", "ريو دي جانيرو"), ("az", "Rio-de-Janeyro"), ("be", "штат Рыа-дэ-Жанэйра"), ("bg", "Рио де Жанейро"), ("bs", "Rio de Janeiro"), ("ca", "Estat de Rio de Janeiro"), ("ccp", "𑄢𑄃\u{1112e} 𑄓\u{11128} 𑄎𑄬𑄚𑄬\u{1112d}𑄢\u{1112e}"), ("ceb", "Rio de Janeiro (estado)"), ("cs", "Rio de Janeiro"), ("cy", "Rio de Janeiro"), ("da", "Rio de Janeiro"), ("de", "Rio de Janeiro"), ("el", "Ρίο ντε Τζανέιρο"), ("en", "Rio de Janeiro"), ("es", "Estado de Río de Janeiro"), ("et", "Rio de Janeiro osariik"), ("eu", "Rio de Janeiroko estatua"), ("fa", "ایالت ریو د ژانیرو"), ("fi", "Rio de Janeiro"), ("fr", "État de Rio de Janeiro"), ("ga", "Rio de Janeiro"), ("gl", "Estado de Río de Xaneiro"), ("he", "ריו דה ז׳ניירו"), ("hi", "रियो डि ज\u{947}न\u{947}रो, प\u{94d}रा\u{902}त"), ("hr", "Rio de Janeiro"), ("hu", "Rio de Janeiro"), ("hy", "Ռիո դե Ժանեյրո"), ("id", "Rio de Janeiro"), ("it", "Rio de Janeiro"), ("ja", "リオデジャネイロ州"), ("ka", "რიო-დე-ჟანეიროს შტატი"), ("ko", "리우데자네이루 주"), ("lt", "Rio de Žaneiras"), ("lv", "Riodežaneiro"), ("mk", "Рио де Жанеиро"), ("mr", "रियो दि जान\u{947}रो"), ("ms", "Rio de Janeiro"), ("nb", "Rio de Janeiro"), ("nl", "Rio de Janeiro"), ("no", "Rio de Janeiro"), ("pl", "Rio de Janeiro"), ("pt", "Rio de Janeiro"), ("ro", "Rio de Janeiro"), ("ru", "Рио-де-Жанейро"), ("sk", "Rio de Janeiro"), ("sr", "Рио де Жанеиро"), ("sr_Latn", "Rio de Žaneiro"), ("sv", "Rio de Janeiro"), ("sw", "Rio de Janeiro"), ("ta", "இரியோ டி சென\u{bc0}ரோ"), ("th", "ร\u{e31}ฐร\u{e35}โอเดจาเนโร"), ("tr", "Rio de Janeiro"), ("uk", "Ріо-де-Жанейро"), ("ur", "ریو دے جینیرو"), ("uz", "Rio-de-Janeyro"), ("vi", "Rio de Janeiro"), ("yo", "Ìpínlẹ\u{300} Rio de Janeiro"), ("yo_BJ", "Ìpínlɛ\u{300} Rio de Janeiro"), ("yue", "里約熱內盧州"), ("yue_Hans", "里约热内卢州"), ("zh", "里約熱內盧州")]),
                        unofficial_name_list: ["Rio de Janeiro"].to_vec(),
                    }
                ),
                (
                    "RN",
                    Subdivision{
                        name: "RN",
                        country_alpha2: Alpha2::BR,
                        code: "RN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.402580299999999), longitude: Some(-36.954107), max_latitude: Some(-4.8317963), min_latitude: Some(-6.9827372), max_longitude: Some(-34.9687549), min_longitude: Some(-38.5821039)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rio Grande do Norte"), ("ar", "ريو غراندي دو نورتي"), ("az", "Riu-Qrandi-du-Norti"), ("be", "Штат Рыу-Гранды-ду-Норты"), ("bg", "Рио Гранди до Норти"), ("bn", "রিও গ\u{9cd}র\u{9be}ন\u{9cd}ডে ড\u{9c1} নর\u{9cd}টে"), ("bs", "Rio Grande do Norte"), ("ca", "Rio Grande do Norte"), ("ccp", "𑄢𑄃\u{1112e} 𑄉\u{11133}𑄢𑄚\u{11134}𑄓𑄬 𑄓\u{1112e} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Rio Grande do Norte"), ("cs", "Rio Grande do Norte"), ("cy", "Rio Grande do Norte"), ("da", "Rio Grande do Norte"), ("de", "Rio Grande do Norte"), ("el", "Ρίο Γκράντε ντο Νόρτε"), ("en", "Rio Grande do Norte"), ("es", "Río Grande del Norte"), ("et", "Rio Grande do Norte osariik"), ("eu", "Rio Grande do Norte"), ("fa", "ریوگرانده دو نورتی"), ("fi", "Rio Grande do Norte"), ("fr", "Rio Grande do Norte"), ("ga", "Rio Grande do Norte"), ("gl", "Río Grande do Norte - Rio Grande do Norte"), ("gu", "રિયો ગ\u{acd}રાન\u{acd}ડ\u{ac7} ડો નોર\u{acd}ટ"), ("he", "ריו גראנדה דו נורטה"), ("hi", "रियो ग\u{94d}रा\u{902}ड\u{947} दो नोर\u{94d}ट\u{947}"), ("hr", "Rio Grande do Norte"), ("hu", "Rio Grande do Norte"), ("hy", "Ռիու Գրանդի դու Նորտի"), ("id", "Rio Grande do Norte"), ("is", "Rio Grande do Norte"), ("it", "Rio Grande do Norte"), ("ja", "リオグランデ・ド・ノルテ州"), ("ka", "რიუ-გრანდი-დუ-ნორტი"), ("kn", "ರ\u{cbf}ಯೊ ಗ\u{ccd}ರಾಂಡ\u{cc6} ಡು ನಾರ\u{ccd}ಟ\u{cc6}"), ("ko", "히우그란지두노르치 주"), ("lt", "Šiaurės Rio Grandė"), ("lv", "Riugrandi du Norti"), ("mk", "Северно Рио Гранде"), ("mr", "रियो ग\u{94d}रा\u{902}द\u{947} दो नॉर\u{94d}त\u{947}"), ("ms", "Rio Grande do Norte"), ("nb", "Rio Grande do Norte"), ("nl", "Rio Grande do Norte"), ("no", "Rio Grande do Norte"), ("pl", "Rio Grande do Norte"), ("pt", "Rio Grande do Norte"), ("ro", "Rio Grande do Norte"), ("ru", "Риу-Гранди-ду-Норти"), ("si", "ර\u{dd2}යෝ ග\u{dca}\u{200d}රන\u{dca}ඩේ ඩො මොර\u{dca}ටෙ"), ("sk", "Rio Grande do Norte"), ("sq", "Rio Grande do Norte"), ("sr", "Рио Гранде до Норте"), ("sr_Latn", "Rio Grande do Norte"), ("sv", "Rio Grande do Norte"), ("sw", "Rio Grande do Norte"), ("ta", "இரியோ கிர\u{bbe}ண\u{bcd}டு டோ ந\u{bbe}ர\u{bcd}த\u{bcd}"), ("te", "ర\u{c3f}య\u{c4b} గ\u{c4d}ర\u{c3e}ండ\u{c4d} డు న\u{c3e}ర\u{c4d}ట\u{c47}"), ("th", "ร\u{e31}ฐร\u{e35}อ\u{e39}กร\u{e31}นด\u{e35}ด\u{e39}นอร\u{e4c}ต\u{e35}"), ("tr", "Rio Grande do Norte"), ("uk", "Ріу-Гранді-ду-Норті"), ("ur", "شمالی ریو گرانڈی"), ("uz", "Riu Grandi du Norti"), ("vi", "Rio Grande do Norte"), ("yo", "Rio Grande do Norte"), ("yo_BJ", "Rio Grande do Norte"), ("yue", "北大河州"), ("yue_Hans", "北大河州"), ("zh", "北里约格朗德")]),
                        unofficial_name_list: ["Rio Grande do Norte"].to_vec(),
                    }
                ),
                (
                    "RO",
                    Subdivision{
                        name: "RO",
                        country_alpha2: Alpha2::BR,
                        code: "RO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.5057341), longitude: Some(-63.580611), max_latitude: Some(-7.9692976), min_latitude: Some(-13.6937038), max_longitude: Some(-59.77434709999999), min_longitude: Some(-66.8102456)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rondônia"), ("ar", "روندونيا"), ("az", "Rondoniya"), ("be", "Штат Рандонія"), ("bg", "Рондония"), ("bn", "রন\u{9cd}ডোনিয\u{9bc}\u{9be}"), ("bs", "Rondônia"), ("ca", "Rondônia"), ("ccp", "𑄢\u{11127}𑄚\u{11133}𑄓\u{11134}𑄓\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Rondônia"), ("cs", "Rondônia"), ("cy", "Rondônia"), ("da", "Rondônia"), ("de", "Rondônia"), ("el", "Ροντόνια"), ("en", "Rondônia"), ("es", "Rondonia"), ("et", "Rondônia osariik"), ("eu", "Rondônia"), ("fa", "روندونیا"), ("fi", "Rondônia"), ("fr", "Rondônia"), ("ga", "Rondônia"), ("gl", "Rondonia - Rondônia"), ("gu", "રોન\u{acd}ડોનિયા"), ("he", "רונדוניה"), ("hi", "रोन\u{94d}डोनिया"), ("hr", "Rondônia"), ("hu", "Rondônia"), ("hy", "Ռոնդոնիա"), ("id", "Rondônia"), ("it", "Rondônia"), ("ja", "ロンドニア州"), ("ka", "რონდონია"), ("kn", "ರೊಂಡೊನ\u{cbf}ಯಾ"), ("ko", "혼도니아 주"), ("lt", "Rondonija"), ("lv", "Rondonija"), ("mk", "Рондонија"), ("mr", "रोन\u{94d}द\u{94d}योनिया"), ("ms", "Rondônia"), ("nb", "Rondônia"), ("nl", "Rondônia"), ("no", "Rondônia"), ("pl", "Rondônia"), ("pt", "Rondônia"), ("ro", "Rondônia"), ("ru", "Рондония"), ("si", "රැන\u{dca}ඩෝන\u{dd2}ය\u{dcf}"), ("sk", "Rondônia"), ("sr", "Рондонија"), ("sr_Latn", "Rondonija"), ("sv", "Rondônia"), ("sw", "Rondônia"), ("ta", "ரொண\u{bcd}டோனிய\u{bbe}"), ("te", "ర\u{c3e}ండ\u{c4b}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐรอนโดเน\u{e35}ย"), ("tr", "Rondônia"), ("uk", "Рондонія"), ("ur", "روندونیا"), ("uz", "Rondoniya"), ("vi", "Rondônia"), ("yo", "Rondônia"), ("yo_BJ", "Rondônia"), ("yue", "朗多尼亞州"), ("yue_Hans", "朗多尼亚州"), ("zh", "朗多尼亚州")]),
                        unofficial_name_list: ["Rondônia"].to_vec(),
                    }
                ),
                (
                    "RR",
                    Subdivision{
                        name: "RR",
                        country_alpha2: Alpha2::BR,
                        code: "RR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.7375971), longitude: Some(-62.0750998), max_latitude: Some(5.2718389), min_latitude: Some(-1.5806358), max_longitude: Some(-58.88687820000001), min_longitude: Some(-64.8252444)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Roraima"), ("ar", "رورايما"), ("az", "Rorayma"), ("be", "Штат Рарайма"), ("bg", "Рорайма"), ("bn", "রোর\u{9be}ইম\u{9be} প\u{9cd}রদেশ"), ("bs", "Roraima"), ("ca", "Roraima"), ("ccp", "𑄢\u{11127}𑄢\u{1112d}𑄟"), ("ceb", "Roraima (estado sa Brasil)"), ("cs", "Roraima"), ("cy", "Roraima"), ("da", "Roraima"), ("de", "Roraima"), ("el", "Ροράιμα"), ("en", "Roraima"), ("es", "Roraima"), ("et", "Roraima osariik"), ("eu", "Roraima"), ("fa", "رورایما"), ("fi", "Roraima"), ("fr", "Roraima"), ("ga", "Roraima"), ("gl", "Roraima"), ("gu", "રોર\u{ac7}મા"), ("he", "רוריימה"), ("hi", "रोर\u{948}मा"), ("hr", "Roraima"), ("hu", "Roraima"), ("id", "Roraima"), ("is", "Roraima"), ("it", "Roraima"), ("ja", "ロライマ州"), ("ka", "რორაიმა"), ("kn", "ರೋರೈಮಾ"), ("ko", "호라이마 주"), ("lt", "Roraima"), ("lv", "Roraima"), ("mk", "Рораима"), ("ml", "റൊറൈമ പർവ\u{d4d}വതം"), ("mr", "रोराईमा"), ("ms", "Roraima"), ("nb", "Roraima"), ("nl", "Roraima"), ("no", "Roraima"), ("pl", "Roraima"), ("pt", "Roraima"), ("ro", "Roraima"), ("ru", "Рорайма"), ("si", "රෝරය\u{dd2}ම\u{dcf}"), ("sk", "Roraima"), ("sq", "Roraima"), ("sr", "Рораима"), ("sr_Latn", "Roraima"), ("sv", "Roraima"), ("sw", "Roraima"), ("ta", "றோரைம\u{bbe}"), ("te", "ర\u{c4b}ర\u{c47}మ\u{c3e}"), ("th", "ร\u{e31}ฐโรไรมา"), ("tr", "Roraima"), ("uk", "Рорайма"), ("ur", "رورائیما"), ("uz", "Rorayma"), ("vi", "Roraima"), ("yo", "Roraima"), ("yo_BJ", "Roraima"), ("yue", "羅賴馬州"), ("yue_Hans", "罗赖马州"), ("zh", "羅賴馬州")]),
                        unofficial_name_list: ["Roraima"].to_vec(),
                    }
                ),
                (
                    "RS",
                    Subdivision{
                        name: "RS",
                        country_alpha2: Alpha2::BR,
                        code: "RS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-30.0346316), longitude: Some(-51.2176986), max_latitude: Some(-27.0805987), min_latitude: Some(-33.752084), max_longitude: Some(-49.6916345), min_longitude: Some(-57.64928399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rio Grande do Sul"), ("ar", "ريو غراندي دو سول"), ("az", "Riu-Qrandi-du-Sul"), ("be", "штат Рыу-Гранды-ду-Сул"), ("bg", "Рио Гранди до Сул"), ("bn", "রিও গ\u{9cd}র\u{9be}ন\u{9cd}ডে দ\u{9c1} সোল"), ("bs", "Rio Grande do Sul"), ("ca", "Rio Grande do Sul"), ("ccp", "𑄢\u{11128}𑄃\u{1112e} 𑄉\u{11133}𑄢𑄚\u{11134}𑄓𑄬 𑄓\u{1112e} 𑄥\u{1112a}𑄣\u{11134}"), ("ceb", "Rio Grande do Sul (estado sa Brasil)"), ("cs", "Rio Grande do Sul"), ("cy", "Rio Grande do Sul"), ("da", "Rio Grande do Sul"), ("de", "Rio Grande do Sul"), ("el", "Ρίο Γκράντε ντο Σουρ"), ("en", "Rio Grande do Sul"), ("es", "Río Grande del Sur"), ("et", "Rio Grande do Suli osariik"), ("eu", "Rio Grande do Sul"), ("fa", "ریو گرانده دو سول"), ("fi", "Rio Grande do Sul"), ("fr", "Rio Grande do Sul"), ("ga", "Rio Grande do Sul"), ("gl", "Río Grande do Sur - Rio Grande do Sul"), ("gu", "રિયો ગ\u{acd}રાન\u{acd}દ\u{ac7} દો સ\u{ac1}લ"), ("he", "ריו גראנדה דו סול"), ("hi", "रियो ग\u{94d}रा\u{902}ड\u{947} दो स\u{941}ल"), ("hr", "Rio Grande do Sul"), ("hu", "Río Grande del Sur"), ("hy", "Ռիու Գրանդի դու Սուլ"), ("id", "Rio Grande do Sul"), ("ig", "BR-RS"), ("is", "Rio Grande do Sul"), ("it", "Rio Grande do Sul"), ("ja", "リオグランデ・ド・スル州"), ("jv", "BR-RS"), ("ka", "რიუ-გრანდი-დუ-სული"), ("kk", "Риу-Гранди-ду-Сул"), ("kn", "ರ\u{cbf}ಯೊ ಗ\u{ccd}ರಾಂಡ\u{cc6} ಡು ಸುಲ\u{ccd}"), ("ko", "히우그란지두술 주"), ("lt", "Pietų Rio Grandė"), ("lv", "Riugrandi du Sula"), ("mk", "Јужно Рио Гранде"), ("mr", "रियो ग\u{94d}रा\u{902}द\u{947} दो स\u{941}ल"), ("ms", "Rio Grande do Sul"), ("nb", "Rio Grande do Sul"), ("nl", "Rio Grande do Sul"), ("no", "Rio Grande do Sul"), ("pl", "Rio Grande do Sul"), ("pt", "Rio Grande do Sul"), ("ro", "Rio Grande do Sul"), ("ru", "Риу-Гранди-ду-Сул"), ("si", "ර\u{dd2}යෝ ග\u{dca}\u{200d}රන\u{dca}ඩේ ඩ\u{dd4} සල\u{dca}"), ("sk", "Rio Grande do Sul"), ("sl", "BR-RS"), ("so", "BR-RS"), ("sq", "BR-RS"), ("sr", "Рио Гранде до Сул"), ("sr_Latn", "Rio Grande do Sul"), ("sv", "Rio Grande do Sul"), ("sw", "Rio Grande do Sul"), ("ta", "இரியோ கிர\u{bbe}ண\u{bcd}டு டொ சுல\u{bcd}"), ("te", "ర\u{c3f}య\u{c4b} గ\u{c4d}ర\u{c3e}ండ\u{c4d} డు సల\u{c4d}"), ("th", "ร\u{e31}ฐร\u{e35}อ\u{e39}กร\u{e31}นด\u{e35}ด\u{e39}ซ\u{e39}ล"), ("tr", "Rio Grande do Sul"), ("uk", "Ріу-Гранді-ду-Сул"), ("ur", "جنوبی ریو گرانڈی"), ("uz", "Riu Grandi du Sul"), ("vi", "Rio Grande do Sul"), ("yo", "Rio Grande do Sul"), ("yo_BJ", "Rio Grande do Sul"), ("yue", "南大河州"), ("yue_Hans", "南大河州"), ("zh", "南里奥格兰德州"), ("zu", "BR-RS")]),
                        unofficial_name_list: ["Rio Grande do Sul"].to_vec(),
                    }
                ),
                (
                    "SC",
                    Subdivision{
                        name: "SC",
                        country_alpha2: Alpha2::BR,
                        code: "SC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-27.2423392), longitude: Some(-50.2188556), max_latitude: Some(-25.9559588), min_latitude: Some(-29.351441), max_longitude: Some(-48.35680809999999), min_longitude: Some(-53.83635870000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Santa Catarina"), ("ar", "سانتا كاتارينا"), ("az", "Santa-Katarina"), ("be", "штат Санта-Катарына"), ("bg", "Санта Катарина"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9cd}য\u{9be}ট\u{9be}রিন\u{9be}"), ("bs", "Santa Catarina"), ("ca", "Estat de Santa Catarina"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄠𑄑𑄢\u{11128}𑄚"), ("ceb", "Santa Catarina"), ("cs", "Santa Catarina"), ("cy", "Santa Catarina"), ("da", "Santa Catarina"), ("de", "Santa Catarina"), ("el", "Σάντα Καταρίνα"), ("en", "Santa Catarina"), ("es", "Santa Catarina"), ("et", "Santa Catarina osariik"), ("eu", "Santa Catarina"), ("fa", "سانتا کاتارینا"), ("fi", "Santa Catarina"), ("fr", "Santa Catarina"), ("ga", "Santa Catarina"), ("gl", "Santa Catarina"), ("gu", "સા\u{a82}તા ક\u{ac7}ટરિના"), ("he", "סנטה קטרינה"), ("hi", "सा\u{902}ता कातारीना"), ("hr", "Santa Catarina"), ("hu", "Santa Catarina"), ("hy", "Սանտա Կատարինա"), ("id", "Santa Catarina"), ("is", "Santa Catarina"), ("it", "Santa Catarina"), ("ja", "サンタカタリーナ州"), ("ka", "სანტა-კატარინა"), ("kk", "Санта Катарина"), ("kn", "ಸಾಂಟಾ ಕ\u{ccd}ಯಾಟರ\u{cbf}ನಾ"), ("ko", "산타카타리나 주"), ("lt", "Santa Katarina"), ("lv", "Santakatarina"), ("mk", "Санта Катарина"), ("mr", "सा\u{902}ता कातारिना"), ("ms", "Santa Catarina"), ("nb", "Santa Catarina"), ("nl", "Santa Catarina"), ("no", "Santa Catarina"), ("pl", "Santa Catarina"), ("pt", "Santa Catarina"), ("ro", "Santa Catarina"), ("ru", "Санта-Катарина"), ("si", "සැන\u{dca}ට\u{dcf} කැතර\u{dd2}න\u{dcf}"), ("sk", "Santa Catarina"), ("sr", "Санта Катарина"), ("sr_Latn", "Santa Katarina"), ("sv", "Santa Catarina"), ("sw", "Santa Catarina"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கதறின\u{bbe}"), ("te", "స\u{c3e}ంట\u{c3e} క\u{c3e}టర\u{c40}న\u{c3e}"), ("th", "ร\u{e31}ฐซานตากาตาร\u{e35}นา"), ("tr", "Santa Catarina"), ("uk", "Санта-Катаріна"), ("ur", "سانتا کاتارینا"), ("uz", "Santa-Katarina"), ("vi", "Santa Catarina"), ("yo", "Santa Catarina"), ("yo_BJ", "Santa Catarina"), ("yue", "聖卡塔蓮娜州"), ("yue_Hans", "圣卡塔莲娜州"), ("zh", "圣卡塔琳娜州")]),
                        unofficial_name_list: ["Santa Catarina"].to_vec(),
                    }
                ),
                (
                    "SE",
                    Subdivision{
                        name: "SE",
                        country_alpha2: Alpha2::BR,
                        code: "SE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.5740934), longitude: Some(-37.3856581), max_latitude: Some(-9.5150294), min_latitude: Some(-11.5685288), max_longitude: Some(-36.3998681), min_longitude: Some(-38.2456472)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sergipe"), ("ar", "سيرجيبي"), ("az", "Serjipi"), ("be", "штат Сержыпі"), ("bg", "Сержипи"), ("bn", "স\u{9be}রজিপে"), ("bs", "Sergipe"), ("ca", "Sergipe"), ("ccp", "𑄥𑄬𑄢\u{11134}𑄉\u{1112d}𑄛\u{11134}"), ("ceb", "Sergipe"), ("cs", "Sergipe"), ("cy", "Sergipe"), ("da", "Sergipe"), ("de", "Sergipe"), ("el", "Σερζίπε"), ("en", "Sergipe"), ("es", "Sergipe"), ("et", "Sergipe osariik"), ("eu", "Sergipe"), ("fa", "سرژیپه"), ("fi", "Sergipe"), ("fr", "Sergipe"), ("ga", "Sergipe"), ("gl", "Sergipe"), ("gu", "સર\u{acd}ગીપ\u{ac7}"), ("he", "סרז׳יפה"), ("hi", "सर\u{94d}जिप\u{947}"), ("hr", "Sergipe"), ("hu", "Sergipe"), ("hy", "Սերժիպի"), ("id", "Sergipe"), ("is", "Sergipe"), ("it", "Sergipe"), ("ja", "セルジッペ州"), ("ka", "სერჟიპი"), ("kk", "Сержипи"), ("kn", "ಸ\u{cc6}ರ\u{ccd}ಗ\u{cbf}ಪ\u{cc6}"), ("ko", "세르지피 주"), ("lt", "Seržipė"), ("lv", "Seržipi"), ("mk", "Сержипе"), ("mr", "सर\u{94d}जिप\u{947}"), ("ms", "Sergipe"), ("nb", "Sergipe"), ("nl", "Sergipe"), ("no", "Sergipe"), ("pl", "Sergipe"), ("pt", "Sergipe"), ("ro", "Sergipe"), ("ru", "Сержипи"), ("si", "සර\u{dca}ග\u{dd2}පේ"), ("sk", "Sergipe"), ("sr", "Сержипе"), ("sr_Latn", "Seržipe"), ("sv", "Sergipe"), ("sw", "Sergipe"), ("ta", "சேர\u{bcd}க\u{bcd}கபே"), ("te", "స\u{c46}ర\u{c4d}జయ\u{c3f}ప\u{c4d}"), ("th", "ร\u{e31}ฐเซร\u{e4c}ช\u{e35}ป\u{e35}"), ("tr", "Sergipe"), ("uk", "Сержипі"), ("ur", "سرژیپی"), ("uz", "Serjipi"), ("vi", "Sergipe"), ("yo", "Sergipe"), ("yo_BJ", "Sergipe"), ("yue", "沙治比州"), ("yue_Hans", "沙治比州"), ("zh", "塞尔希培州")]),
                        unofficial_name_list: ["Sergipe"].to_vec(),
                    }
                ),
                (
                    "SP",
                    Subdivision{
                        name: "SP",
                        country_alpha2: Alpha2::BR,
                        code: "SP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-23.5505199), longitude: Some(-46.63330939999999), max_latitude: Some(-23.3566039), min_latitude: Some(-24.0082209), max_longitude: Some(-46.3650844), min_longitude: Some(-46.825514)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "São Paulo"), ("ar", "ساو باولو"), ("az", "San-Paulo"), ("be", "штат Сан-Паўлу"), ("bg", "Сао Пауло"), ("bn", "স\u{9be}\u{981}ও প\u{9be}ওলো"), ("bs", "São Paulo (država)"), ("ca", "Estat de São Paulo"), ("ccp", "𑄥𑄃\u{1112e} 𑄛𑄅\u{1112a}𑄣\u{1112e}"), ("ceb", "São Paulo (estado)"), ("cs", "São Paulo"), ("cy", "São Paulo"), ("da", "São Paulo"), ("de", "São Paulo"), ("el", "Σάο Πάολο"), ("en", "São Paulo"), ("es", "São Paulo"), ("et", "São Paulo osariik"), ("eu", "São Pauloko estatua"), ("fa", "ایالت سائوپائولو"), ("fi", "São Paulo"), ("fr", "État de São Paulo"), ("ga", "São Paulo"), ("gl", "Estado de San Paulo - São Paulo"), ("gu", "સાઓ પાઉલો"), ("he", "סאו פאולו"), ("hi", "साओ पाउलो"), ("hr", "São Paulo"), ("hu", "São Paulo"), ("hy", "Սան Պաուլու"), ("id", "São Paulo"), ("is", "São Paulo"), ("it", "San Paolo"), ("ja", "サンパウロ州"), ("ka", "სან-პაულუ"), ("kn", "ಸಾವೊ ಪಾಲೊ"), ("ko", "상파울루 주"), ("lt", "San Paulas"), ("lv", "Sanpaulu"), ("mk", "Сао Паоло"), ("mn", "Сан-Паулу муж"), ("mr", "साओ पाउलो"), ("ms", "São Paulo"), ("nb", "São Paulo"), ("ne", "सा\u{901}ओ पाउलो"), ("nl", "São Paulo"), ("no", "São Paulo"), ("pl", "São Paulo"), ("pt", "São Paulo"), ("ro", "São Paulo"), ("ru", "Сан-Паулу"), ("si", "ස\u{dcf}ඕ පව\u{dd4}ලෝ"), ("sk", "São Paulo"), ("so", "São Paulo"), ("sq", "Sao Paulo"), ("sr", "Сао Пауло"), ("sr_Latn", "Sao Paulo"), ("sv", "São Paulo"), ("sw", "São Paulo"), ("ta", "ச\u{bbe}வோ ப\u{bbe}லோ"), ("te", "స\u{c3e}వ\u{c4b} ప\u{c3e}ల\u{c4b}"), ("th", "ร\u{e31}ฐเซาเปาล\u{e39}"), ("tr", "São Paulo"), ("uk", "Сан-Паулу"), ("ur", "ساؤ پاؤلو"), ("uz", "San Paulu"), ("vi", "São Paulo"), ("yo", "Sao Paulo"), ("yo_BJ", "Sao Paulo"), ("yue", "聖保羅州"), ("yue_Hans", "圣保罗州"), ("zh", "圣保罗州")]),
                        unofficial_name_list: ["São Paulo"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::BR,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.17528), longitude: Some(-48.2982474), max_latitude: Some(-5.1683828), min_latitude: Some(-13.4677157), max_longitude: Some(-45.6960778), min_longitude: Some(-50.74205809999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tocantins"), ("ar", "توكانتينس"), ("az", "Tokantins"), ("be", "Штат Такантынс"), ("bg", "Токантинс"), ("bn", "টোক\u{9cd}য\u{9be}ন\u{9cd}টিন"), ("bs", "Tocantins"), ("ca", "Estat de Tocantins"), ("ccp", "𑄑\u{11127}𑄇\u{11133}𑄠𑄚\u{11134}𑄑\u{11128}𑄚\u{11134}"), ("ceb", "Tocantins (estado)"), ("cs", "Tocantins"), ("cy", "Tocantins"), ("da", "Tocantins"), ("de", "Tocantins"), ("el", "Τοκάντινς"), ("en", "Tocantins"), ("es", "Tocantins"), ("et", "Tocantinsi osariik"), ("eu", "Tocantins"), ("fa", "توکانتینس"), ("fi", "Tocantins"), ("fr", "Tocantins"), ("ga", "Tocantins"), ("gl", "Tocantins"), ("gu", "ટોક\u{ac7}ન\u{acd}ટિન\u{acd}સ"), ("he", "טוקנטינס"), ("hi", "टोकाचिस"), ("hr", "Tocantins"), ("hu", "Tocatins"), ("id", "Tocantins"), ("it", "Tocantins"), ("ja", "トカンティンス州"), ("ka", "ტოკანტინსი"), ("kk", "Токантинс"), ("kn", "ಟೊಕಂಟ\u{cbf}ನ\u{ccd}ಸ\u{ccd}"), ("ko", "토칸칭스 주"), ("lt", "Tokantinsas"), ("lv", "Tokantinsa"), ("mk", "Токантинс"), ("mr", "तोका\u{902}तिन\u{94d}स"), ("ms", "Tocantins"), ("nb", "Tocantins"), ("nl", "Tocantins"), ("no", "Tocantins"), ("pl", "Tocantins"), ("pt", "Tocantins"), ("ro", "Tocantins"), ("ru", "Токантинс"), ("si", "ටෝකන\u{dca}ට\u{dd2}න\u{dca}ස\u{dca}"), ("sk", "Tocantins"), ("sr", "Токантинс"), ("sr_Latn", "Tokantins"), ("sv", "Tocantins"), ("sw", "Tocantins"), ("ta", "டோக\u{bbe}ண\u{bcd}டின\u{bcd}ஸ\u{bcd}"), ("te", "ట\u{c4a}క\u{c3e}ంట\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐโตก\u{e31}นช\u{e35}นส\u{e4c}"), ("tr", "Tocantins"), ("uk", "Токантінс"), ("ur", "توکانتینس"), ("uz", "Tokantins"), ("vi", "Tocantins"), ("yo", "Tocantins"), ("yo_BJ", "Tocantins"), ("yue", "托坎廷斯州"), ("yue_Hans", "托坎廷斯州"), ("zh", "托坎廷斯")]),
                        unofficial_name_list: ["Tocantins"].to_vec(),
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
#[cfg(feature = "br")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BR,
        alpha3: Alpha3::BRA,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::SouthAmerica,
        country_code: 55,
        currency_code: "BRL",
        gec: Some(GEC::BR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "0014",
        ioc: Some(IOC::BRA),
        iso_long_name: "The Federative Republic of Brazil",
        iso_short_name: "Brazil",
        official_language_list: ["pt"].to_vec(),
        spoken_language_list: ["pt"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10, 11].to_vec(),
        national_prefix: "014",
        nationality: Some("Brazilian"),
        number: "076",
        postal_code: true,
        postal_code_format: Some("\\d{5}-?\\d{3}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "BR",
        unofficial_name_list: [
            "Brazil",
            "Brasilien",
            "Brésil",
            "Brasil",
            "ブラジル",
            "Brazilië",
        ]
        .to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Brazil"),
            ("af", "Brasilië"),
            ("ak", "Brazil"),
            ("am", "ብሲፑሔ"),
            ("an", "Brasil"),
            ("ar", "البرازيل"),
            ("as", "ব\u{9cd}ৰ\u{9be}জিল"),
            ("ay", "Brazil"),
            ("az", "Braziliya"),
            ("ba", "Brazil"),
            ("be", "Бразілія"),
            ("bg", "Бразилия"),
            ("bi", "Brazil"),
            ("bn", "ব\u{9cd}র\u{9be}জিল"),
            ("bn_IN", "ব\u{9cd}র\u{9be}জিল"),
            ("br", "Brazil"),
            ("bs", "Brazil"),
            ("ca", "Brasil"),
            ("ce", "Бразили"),
            ("ch", "Brazil"),
            ("cs", "Brazílie"),
            ("cv", "Бразили"),
            ("cy", "Brasil"),
            ("da", "Brasilien"),
            ("de", "Brasilien"),
            ("dv", "ބ\u{7aa}ރ\u{7ac}ޒ\u{7a8}ލ\u{7b0}"),
            ("dz", "བ་ར་ཛ\u{f72}ལ།"),
            ("ee", "Brazil"),
            ("el", "Βραζιλία"),
            ("en", "Brazil"),
            ("eo", "Brazilo"),
            ("es", "Brasil"),
            ("et", "Brasiilia"),
            ("eu", "Brasil"),
            ("fa", "برزیل"),
            ("ff", "Barazil"),
            ("fi", "Brasilia"),
            ("fo", "Brasilia"),
            ("fr", "Brésil"),
            ("fy", "Brazylje"),
            ("ga", "An Bhrasaíl"),
            ("gl", "Brasil"),
            ("gn", "Brazil"),
            ("gu", "બ\u{acd}રાઝિલ"),
            ("gv", "Yn Vrasseel"),
            ("ha", "Brazil"),
            ("he", "ברזיל"),
            ("hi", "ब\u{94d}राज\u{93c}ील"),
            ("hr", "Brazil"),
            ("ht", "Brezil"),
            ("hu", "Brazília"),
            ("hy", "Բրազիլիա"),
            ("ia", "Brasil"),
            ("id", "Brasil"),
            ("io", "Brazilia"),
            ("is", "Brasilía"),
            ("it", "Brasile"),
            ("iu", "Brazil"),
            ("ja", "ブラジル"),
            ("ka", "ბრაზილია"),
            ("ki", "Brazil"),
            ("kk", "Бразилия"),
            ("kl", "Brazil"),
            ("km", "ប\u{17d2}រេស\u{17ca}\u{17b8}ល"),
            ("kn", "ಬ\u{ccd}ರಾಝ\u{cbf}ಲ\u{ccd}"),
            ("ko", "브라질"),
            ("ku", "Brezîlya"),
            ("kv", "Бразилия"),
            ("kw", "Brasil"),
            ("ky", "Бразилия"),
            ("lo", "ປະເທດບະເລຊ\u{eb4}ນ"),
            ("lt", "Brazilija"),
            ("lv", "Brazīlija"),
            ("mi", "Parīhi"),
            ("mk", "Бразил"),
            ("ml", "ബ\u{d4d}രസീല\u{d4d}\u{200d}"),
            ("mn", "Бразил"),
            ("mr", "ब\u{94d}राझिल"),
            ("ms", "Brazil"),
            ("mt", "Brażil"),
            ("my", "ဘရာဇ\u{102e}းန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Bradir"),
            ("nb", "Brasil"),
            ("ne", "ब\u{94d}राजिल"),
            ("nl", "Brazilië"),
            ("nn", "Brasil"),
            ("nv", "Kintaaʼanéhé Dineʼé Bikéyah"),
            ("oc", "Brasil"),
            ("or", "ବ\u{b4d}ର\u{b3e}ଜୀଲ"),
            ("pa", "ਬਰਾਜ\u{a3c}ੀਲ"),
            ("pi", "ब\u{94d}रासील"),
            ("pl", "Brazylia"),
            ("ps", "برازیل"),
            ("pt", "Brasil"),
            ("pt_BR", "Brasil"),
            ("ro", "Brazilia"),
            ("ru", "Бразилия"),
            ("rw", "Burezile"),
            ("sc", "Brasile"),
            ("sd", "برازيل"),
            ("si", "බ\u{dca}\u{200d}රස\u{dd2}ලය"),
            ("sk", "Brazília"),
            ("sl", "Brazilija"),
            ("so", "Braasiil"),
            ("sq", "Brazil"),
            ("sr", "Бразил"),
            ("sv", "Brasilien"),
            ("sw", "Brazil"),
            ("ta", "பிரேசில\u{bcd}"),
            ("te", "బ\u{c4d}ర\u{c46}జ\u{c3f}ల\u{c4d}"),
            ("tg", "Бразилия"),
            ("th", "บราซ\u{e34}ล"),
            ("ti", "ብራዚል"),
            ("tk", "Braziliýa"),
            ("tl", "Brasil"),
            ("tr", "Brezilya"),
            ("tt", "Бразил"),
            ("ug", "بىرازىلىيە"),
            ("uk", "Бразилія"),
            ("ur", "برازیل"),
            ("uz", "Braziliya"),
            ("ve", "Brazil"),
            ("vi", "Bra-xin"),
            ("wa", "Braezi"),
            ("wo", "Bareesil"),
            ("xh", "Brazil"),
            ("yo", "Brasil"),
            ("zh_CN", "巴西"),
            ("zh_HK", "巴西"),
            ("zh_TW", "巴西"),
            ("zu", "IBrazili"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
