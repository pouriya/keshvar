// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Guatemala

#[cfg(all(feature = "gt", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GT;
    pub const ALPHA3: Alpha3 = Alpha3::GTM;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 502;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::GTQ;
    pub const GEC: Option<GEC> = Some(GEC::GT);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::GUA);
    pub const ISO_SHORT_NAME: &str = "Guatemala";
    pub const ISO_LONG_NAME: &str = "The Republic of Guatemala";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Guatemalan");
    pub const NUMBER: &str = "320";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "GT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Guatemala", "グアテマラ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Guatemala"),
        ("af", "Guatemala"),
        ("ak", "Guatemala"),
        ("am", "Guatemala"),
        ("an", "Guatemala"),
        ("ar", "غواتيمالا"),
        ("as", "গ\u{9c1}ৱ\u{9be}টেম\u{9be}ল\u{9be}"),
        ("ay", "Guatemala"),
        ("az", "Quatemala"),
        ("ba", "Guatemala"),
        ("be", "Гватэмала"),
        ("bg", "Гватемала"),
        ("bi", "Guatemala"),
        ("bn", "গ\u{9c1}য়\u{9be}তেম\u{9be}ল\u{9be}"),
        ("bn_IN", "গ\u{9c1}য়\u{9be}তেম\u{9be}ল\u{9be}"),
        ("br", "Guatemala"),
        ("bs", "Gvatemala"),
        ("ca", "Guatemala"),
        ("ce", "Гватемала"),
        ("ch", "Guatemala"),
        ("cs", "Guatemala"),
        ("cv", "Гватемала"),
        ("cy", "Gwatemala"),
        ("da", "Guatemala"),
        ("de", "Guatemala"),
        ("dv", "ގ\u{7aa}އ\u{7a6}ޓ\u{7ac}މ\u{7a7}ލ\u{7a7}"),
        ("dz", "གའ\u{f74}་ཏ\u{f72}་མ་ལ།"),
        ("ee", "Guatemala"),
        ("el", "Γουατεμάλα"),
        ("en", "Guatemala"),
        ("eo", "Gvatemalo"),
        ("es", "Guatemala"),
        ("et", "Guatemala"),
        ("eu", "Guatemala"),
        ("fa", "گواتمالا"),
        ("ff", "Guatemala"),
        ("fi", "Guatemala"),
        ("fo", "Guatemala"),
        ("fr", "Guatemala"),
        ("fy", "Gûatemala"),
        ("ga", "Guatamala"),
        ("gl", "Guatemala"),
        ("gn", "Guatemala"),
        ("gu", "ગ\u{acd}વાટ\u{ac7}માલા"),
        ("gv", "Yn Ghuatemaley"),
        ("ha", "Guatemala"),
        ("he", "גואטמלה"),
        ("hi", "ग\u{94d}वाट\u{947}माला"),
        ("hr", "Gvatemala"),
        ("ht", "Gwatemala"),
        ("hu", "Guatemala"),
        ("hy", "Գվատեմալա"),
        ("ia", "Guatemala"),
        ("id", "Guatemala"),
        ("io", "Guatemala"),
        ("is", "Gvatemala"),
        ("it", "Guatemala"),
        ("iu", "Guatemala"),
        ("ja", "グアテマラ"),
        ("ka", "გვატემალა"),
        ("ki", "Guatemala"),
        ("kk", "Гватемала"),
        ("kl", "Guatemala"),
        ("km", "ហ\u{17d2}គាតេម\u{17c9}ាឡា"),
        ("kn", "ಗ\u{ccd}ವಾಟ\u{cc6}ಮಾಲಾ"),
        ("ko", "과테말라"),
        ("ku", "Guatemala"),
        ("kv", "Гватемала"),
        ("kw", "Gwatemala"),
        ("ky", "Гватемала"),
        ("lo", "ປະເທດກ\u{ebb}ວເຕມາລາ"),
        ("lt", "Gvatemala"),
        ("lv", "Gvatemala"),
        ("mi", "Guatemala"),
        ("mk", "Гватемала"),
        ("ml", "ഗ\u{d4d}വ\u{d3e}ട\u{d4d}ടിമ\u{d3e}ല"),
        ("mn", "Гватемал"),
        ("mr", "ग\u{94d}वाट\u{947}माला"),
        ("ms", "Guatemala"),
        ("mt", "Gwatemala"),
        (
            "my",
            "ဂ\u{103d}ါတ\u{102e}မာလာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Guatemara"),
        ("nb", "Guatemala"),
        ("ne", "ग\u{94d}वाट\u{947}माला"),
        ("nl", "Guatemala"),
        ("nn", "Guatemala"),
        ("nv", "Guatemala"),
        ("oc", "Guatemala"),
        ("or", "ଗ\u{b4d}ବ\u{b3e}ଟେମ\u{b3e}ଲ\u{b3e}"),
        ("pa", "ਗ\u{a41}ਆਟ\u{a47}ਮਾਲਾ"),
        ("pi", "ग\u{94d}वाट\u{947}माला"),
        ("pl", "Gwatemala"),
        ("ps", "ګواتیمالا"),
        ("pt", "Guatemala"),
        ("pt_BR", "Guatemala"),
        ("ro", "Guatemala"),
        ("ru", "Гватемала"),
        ("rw", "Gwatemala"),
        ("sc", "Guatemala"),
        ("sd", "Guatemala"),
        ("si", "ගෝතම\u{dcf}ල\u{dcf}ව"),
        ("sk", "Guatemala"),
        ("sl", "Gvatemala"),
        ("so", "Guatemala"),
        ("sq", "Guatemalë"),
        ("sr", "Гватемала"),
        ("sv", "Guatemala"),
        ("sw", "Guatemala"),
        ("ta", "குவ\u{bbe}தம\u{bbe}ல\u{bbe}"),
        ("te", "గ\u{c4d}వ\u{c3e}ట\u{c47}మ\u{c3e}ల\u{c3e}"),
        ("tg", "Гватемала"),
        ("th", "ก\u{e31}วเตมาลา"),
        ("ti", "ጓቲማላ"),
        ("tk", "Gwatemala"),
        ("tl", "Gwatemala"),
        ("tr", "Guatemala"),
        ("tt", "Gуатемала"),
        ("ug", "گىۋاتېمالا"),
        ("uk", "Гватемала"),
        ("ur", "گوئٹے مالا"),
        ("uz", "Gvatemala"),
        ("ve", "Guatemala"),
        ("vi", "Gua-tê-ma-la"),
        ("wa", "Gwatemala"),
        ("wo", "Guwaatemaala"),
        ("xh", "Guatemala"),
        ("yo", "Guatẹmálà"),
        ("zh_CN", "瓜地马拉"),
        ("zh_HK", "危地馬拉"),
        ("zh_TW", "瓜地馬拉"),
        ("zu", "I-Guwathemala"),
    ];
    #[cfg(all(feature = "gt", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 15.783471;
        pub const LONGITUDE: f64 = -90.23075899999999;
        pub const MAX_LATITUDE: f64 = 17.815697;
        pub const MAX_LONGITUDE: f64 = -88.1982001;
        pub const MIN_LATITUDE: f64 = 13.63;
        pub const MIN_LONGITUDE: f64 = -92.2714;
        pub const NORTHEAST_LATITUDE: f64 = 17.815697;
        pub const NORTHEAST_LONGITUDE: f64 = -88.1982001;
        pub const SOUTHWEST_LATITUDE: f64 = 13.63;
        pub const SOUTHWEST_LONGITUDE: f64 = -92.2714;
    }
}
#[cfg(all(feature = "gt", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 15.783471,
            longitude: -90.23075899999999,
            max_latitude: 17.815697,
            max_longitude: -88.1982001,
            min_latitude: 13.63,
            min_longitude: -92.2714,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 17.815697,
                    longitude: -88.1982001,
                },
                southwest: CountryGeoBound {
                    latitude: 13.63,
                    longitude: -92.2714,
                },
            },
        }
    }
}

#[cfg(all(feature = "gt", feature = "subdivisions"))]
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
                    "AV",
                    Subdivision{
                        name: "AV",
                        country_alpha2: Alpha2::GT,
                        code: "AV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.5942883), longitude: Some(-90.14949879999999), max_latitude: Some(16.0719059), min_latitude: Some(15.137356), max_longitude: Some(-89.372641), min_longitude: Some(-90.7980479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ألتا فيراباز"), ("bg", "Горен Верапас"), ("bn", "আল\u{9cd}ট\u{9be} ভের\u{9be}প\u{9be}জ বিভ\u{9be}গ"), ("bs", "Alta Verapaz"), ("ca", "Alta Verapaz"), ("ccp", "𑄃𑄣\u{11134}𑄑 𑄞𑄢\u{11134}𑄛𑄌\u{11134}"), ("ceb", "Departamento de Alta Verapaz"), ("da", "Alta Verapaz Department"), ("de", "Departamento Alta Verapaz"), ("el", "Άλτα Βεραπάζ"), ("en", "Alta Verapaz"), ("es", "Alta Verapaz"), ("eu", "Alta Verapaz"), ("fa", "حوزه التا وراپاز"), ("fi", "Alta Verapaz"), ("fr", "département d’Alta Verapaz"), ("gl", "Departamento de Alta Verapaz"), ("gu", "અલ\u{acd}ટા વ\u{ac7}રાપાઝ વિભાગ"), ("hi", "ऑल\u{94d}टा व\u{947}रापाज\u{93c} विभाग"), ("hr", "Alta Verapaz"), ("hu", "Alta Verapaz megye"), ("id", "Alta Verapaz"), ("it", "dipartimento di Alta Verapaz"), ("ja", "アルタ・ベラパス県"), ("ka", "ალტა-ვერაპასის დეპარტამენტი"), ("kn", "ಅಲ\u{ccd}ಟಾ ವ\u{cc6}ರಾಪಾಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "알타베라파스 주"), ("lt", "Aukštutinio Verapaso departamentas"), ("lv", "Altaverapasas departaments"), ("mr", "अल\u{94d}टा व\u{947}रापाझ विभाग"), ("ms", "Alta Verapaz Department"), ("nb", "Alta Verapaz"), ("nl", "Alta Verapaz"), ("no", "Alta Verapaz"), ("pl", "Alta Verapaz"), ("pt", "Alta Verapaz"), ("ru", "Альта-Верапас"), ("si", "අල\u{dca}ට\u{dcf} වෙරප\u{dcf}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Alta Verapaz"), ("sv", "Departamento de Alta Verapaz"), ("ta", "அல\u{bcd}ட\u{bbe} வெர\u{bcd}ப\u{bcd}பஸ\u{bcd} துறை"), ("te", "ఆల\u{c4d}ట\u{c3e} వ\u{c46}ర\u{c3e}ప\u{c3e}జ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องอ\u{e31}ลตา เวราปาซ"), ("tr", "Alta Verapaz Departmanı"), ("uk", "Альта-Верапас"), ("ur", "آلتا ویراپاس محکمہ"), ("vi", "Alta Verapaz"), ("zh", "上維拉帕斯省")]),
                        unofficial_name_list: ["Alta Verapaz"].to_vec(),
                    }
                ),
                (
                    "BV",
                    Subdivision{
                        name: "BV",
                        country_alpha2: Alpha2::GT,
                        code: "BV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.0787498), longitude: Some(-90.4125181), max_latitude: Some(15.282989), min_latitude: Some(14.865581), max_longitude: Some(-89.894313), min_longitude: Some(-90.8599491)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بايا فيراباز"), ("bg", "Долен Верапас"), ("bn", "ব\u{9be}জ\u{9be} ভ\u{9be}র\u{9be}প\u{9cd}য\u{9be}জ বিভ\u{9be}গ"), ("bs", "Baja Verapaz"), ("ca", "Baja Verapaz"), ("ccp", "𑄝𑄎 𑄞𑄢\u{11134}𑄛𑄌\u{11134}"), ("ceb", "Departamento de Baja Verapaz"), ("da", "Baja Verapaz Department"), ("de", "Departamento Baja Verapaz"), ("el", "Μπάχα Βεραπάζ"), ("en", "Baja Verapaz"), ("es", "Baja Verapaz"), ("eu", "Baja Verapaz"), ("fi", "Baja Verapazin departementti"), ("fr", "département de Baja Verapaz"), ("gl", "Departamento de Baja Verapaz"), ("gu", "બાજા વ\u{ac7}રાપાઝ વિભાગ"), ("hi", "बाखा व\u{947}रापाज\u{93c} विभाग"), ("hr", "Baja Verapaz"), ("hu", "Baja Verapaz megye"), ("id", "Baja Verapaz"), ("it", "dipartimento di Baja Verapaz"), ("ja", "バハ・ベラパス県"), ("ka", "ბახა-ვერაპასის დეპარტამენტი"), ("kn", "ಬಾಜಾ ವ\u{cc6}ರಾಪಾಜ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "바하베라파스 주"), ("lt", "Žemutinio Verapaso departamentas"), ("lv", "Bahaverapasas departaments"), ("mr", "बाजा व\u{947}रापाझ विभाग"), ("ms", "Baja Verapaz Department"), ("nb", "Baja Verapaz"), ("nl", "Baja Verapaz"), ("no", "Baja Verapaz"), ("pl", "Baja Verapaz"), ("pt", "Baja Verapaz"), ("ru", "Баха-Верапас"), ("si", "බජ\u{dcf} වෙරප\u{dcf}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Baja Verapaz"), ("sv", "Departamento de Baja Verapaz"), ("ta", "ப\u{bbe}ஜ\u{bbe} வேர\u{bcd}ப\u{bcd}பஸ\u{bcd} துறை"), ("te", "బ\u{c3e}జ\u{c3e}వరప\u{c3e}జ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ลตาเบราป\u{e31}ซ"), ("tr", "Baja Verapaz"), ("uk", "Баха-Верапас"), ("ur", "باخا ویراپاس محکمہ"), ("vi", "Baja Verapaz"), ("zh", "下維拉帕斯省")]),
                        unofficial_name_list: ["Baja Verapaz"].to_vec(),
                    }
                ),
                (
                    "CM",
                    Subdivision{
                        name: "CM",
                        country_alpha2: Alpha2::GT,
                        code: "CM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.662222), longitude: Some(-90.820833), max_latitude: Some(14.6773026), min_latitude: Some(14.6391056), max_longitude: Some(-90.7975388), min_longitude: Some(-90.8541442)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة تشيمالتينانغو"), ("bg", "Чималтенанго"), ("bn", "কিম\u{9be}লতেন\u{9be}গো বিভ\u{9be}গ"), ("bs", "Chimaltenango"), ("ca", "Departament de Chimaltenango"), ("ccp", "𑄌\u{11128}𑄟𑄣\u{11134}𑄑𑄬𑄚𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Departamento de Chimaltenango"), ("da", "Chimaltenango Department"), ("de", "Departamento Chimaltenango"), ("el", "Τσιμαλτενάνγκο"), ("en", "Chimaltenango"), ("es", "Chimaltenango"), ("eu", "Chimaltenango"), ("fi", "Chimaltenangon departmentti"), ("fr", "département de Chimaltenango"), ("gl", "Departamento de Chimaltenango"), ("gu", "ચિમલાટાલ\u{ac7}ન\u{acd}ગો વિભાગ"), ("hi", "चिमाल\u{94d}त\u{947}ना\u{902}गो विभाग"), ("hr", "Chimaltenango"), ("hu", "Chimaltenango megye"), ("id", "Departemen Chimaltenango"), ("it", "dipartimento di Chimaltenango"), ("ja", "チマルテナンゴ県"), ("ka", "ჩიმალტენანგოს დეპარტამენტი"), ("kn", "ಚ\u{cbf}ಮಾಲ\u{ccd}ಟ\u{cc6}ನಾಂಗೊ ಇಲಾಖ\u{cc6}"), ("ko", "치말테낭고 주"), ("lt", "Čimaltenango departamentas"), ("lv", "Čimeltenango departaments"), ("mr", "चिमाल\u{94d}टगो विभाग"), ("ms", "Chimaltenango Department"), ("nb", "Chimaltenango"), ("nl", "Chimaltenango"), ("no", "Chimaltenango"), ("pl", "Chimaltenango"), ("pt", "Chimaltenango"), ("ru", "Чимальтенанго"), ("si", "ච\u{dd2}මල\u{dca}ටෙනන\u{dca}ගෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Chimaltenango"), ("sv", "Departamento de Chimaltenango"), ("ta", "ச\u{bc0}ம\u{bbe}ல\u{bcd}டேணங\u{bcd}கொ துறை"), ("te", "చ\u{c3f}మ\u{c3e}ల\u{c4d}ట\u{c46}న\u{c3e}ంగ\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ช\u{e35}ม\u{e31}ลเตน\u{e31}งโก"), ("tr", "Chimaltenango District"), ("uk", "Чимальтенанго"), ("ur", "چیمالتینانگو محکمہ"), ("vi", "Khu vực hành chính Chimaltenango"), ("zh", "奇馬爾特南戈省")]),
                        unofficial_name_list: ["Chimaltenango"].to_vec(),
                    }
                ),
                (
                    "CQ",
                    Subdivision{
                        name: "CQ",
                        country_alpha2: Alpha2::GT,
                        code: "CQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.783333), longitude: Some(-89.533333), max_latitude: Some(14.8176195), min_latitude: Some(14.7463474), max_longitude: Some(-89.51448309999999), min_longitude: Some(-89.56295019999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة تشيكيمولا"), ("bg", "Чикимула"), ("bn", "কিক\u{9c1}ইমোল\u{9be} বিভ\u{9be}গ"), ("bs", "Chiquimula"), ("ca", "Departament de Chiquimula"), ("ccp", "𑄌\u{11128}𑄇\u{1112d}\u{1112a}𑄟\u{11128}𑄅\u{1112a}𑄣"), ("da", "Chiquimula Department"), ("de", "Departamento Chiquimula"), ("el", "Τσικουιμούλα"), ("en", "Chiquimula"), ("es", "Chiquimula"), ("eu", "Chiquimula"), ("fi", "Chiquimula"), ("fr", "département de Chiquimula"), ("gl", "Departamento de Chiquimula"), ("gu", "ચિક\u{acd}ય\u{ac1}મ\u{ac1}લા વિભાગ"), ("hi", "चिकिम\u{941}ला विभाग"), ("hu", "Chiquimula megye"), ("id", "Departemen Chiquimula"), ("it", "dipartimento di Chiquimula"), ("ja", "チキムラ県"), ("ka", "ჩიკიმულის დეპარტამენტი"), ("kn", "ಚ\u{cbf}ಕ\u{ccd}ವ\u{cbf}ಮುಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "치키물라 주"), ("lt", "Čikimulos departamentas"), ("lv", "Čikimulas departaments"), ("mr", "चीक\u{94d}य\u{941}इम\u{941}ल\u{94d} विभाग"), ("ms", "Chiquimula Department"), ("nb", "Chiquimula"), ("nl", "Chiquimula"), ("no", "Chiquimula"), ("pl", "Chiquimula"), ("pt", "Chiquimula"), ("ru", "Чикимула"), ("si", "ච\u{dca}ක\u{dd4}ය\u{dd2}ම\u{dd4}ල\u{dcf} දෙප\u{dcf}ර\u{dca}තෙමේන\u{dca}ත\u{dd4}ව"), ("sk", "Chiquimula"), ("sv", "Chiquimula"), ("ta", "சிக\u{bcd}விமுல\u{bbe} துறை"), ("te", "చ\u{c3f}క\u{c3f}ముల\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e35}ก\u{e35}ม\u{e39}ลา"), ("tr", "Chiquimula Departmanı"), ("uk", "Чикімула"), ("ur", "چیکویمولا محکمہ"), ("vi", "Chiquimula"), ("zh", "奇基穆拉省")]),
                        unofficial_name_list: ["Chiquimula"].to_vec(),
                    }
                ),
                (
                    "ES",
                    Subdivision{
                        name: "ES",
                        country_alpha2: Alpha2::GT,
                        code: "ES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.3009389), longitude: Some(-90.7882163), max_latitude: Some(14.474574), min_latitude: Some(13.912029), max_longitude: Some(-90.49225009999999), min_longitude: Some(-91.538004)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إسكوينتلا"), ("bg", "Ескуинтла"), ("bn", "এস\u{9cd}ক\u{9c1}ইন\u{9cd}ট\u{9be}ল বিভ\u{9be}গ"), ("bs", "Escuintla"), ("ca", "Departament d’Escuintla"), ("ccp", "𑄃𑄬𑄌\u{11134}𑄇\u{1112a}𑄃\u{11128}𑄚\u{11134}𑄑\u{11134}𑄣"), ("ceb", "Departamento de Escuintla"), ("da", "Escuintla Department"), ("de", "Departamento Escuintla"), ("el", "Εσκουίντλα"), ("en", "Escuintla"), ("es", "Escuintla"), ("eu", "Escuintla"), ("fi", "Escuintlan departmentti"), ("fr", "Departement d’Escuintla"), ("gl", "Departamento de Escuintla"), ("gu", "એસક\u{ac1}ઇન\u{acd}ત\u{acd}લા વિભાગ"), ("hi", "एसक\u{94d}य\u{941}इ\u{902}टला"), ("hu", "Escuintla megye"), ("id", "Departemen Escuintla"), ("it", "Escuintla"), ("ja", "エスクィントラ県"), ("ka", "ესკუინტლას დეპარტამენტი"), ("kn", "ಎಸ\u{ccd}ಕುಂಟ\u{cbf}ಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "에스쿠인틀라 주"), ("lt", "Eskuintlos departamentas"), ("lv", "Eskuintlas departaments"), ("mr", "एसकइन\u{94d}टला विभाग"), ("ms", "Escuintla Department"), ("nb", "Escuintla"), ("nl", "Escuintla"), ("no", "Escuintla"), ("pl", "Escuintla"), ("pt", "Escuintla"), ("ru", "Эскуинтла"), ("si", "එස\u{dca}ක\u{dd4}ය\u{dd2}න\u{dca}ට\u{dca}ල\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Escuintla"), ("sv", "Departamento de Escuintla"), ("ta", "ஏசுகியின\u{bcd}ட\u{bcd}ல துறை"), ("te", "ఎస\u{c4d}క\u{c4d}వ\u{c3f}ంట\u{c4d}ల\u{c3e}"), ("th", "เอสกว\u{e34}นตลา"), ("tr", "Escuintla Departmano"), ("uk", "Ескуінтла"), ("ur", "ایسکوینتلا محکمہ"), ("vi", "Escuintla"), ("zh", "埃斯昆特拉省")]),
                        unofficial_name_list: ["Escuintla"].to_vec(),
                    }
                ),
                (
                    "GU",
                    Subdivision{
                        name: "GU",
                        country_alpha2: Alpha2::GT,
                        code: "GU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.613333), longitude: Some(-90.53527799999999), max_latitude: Some(14.7012545), min_latitude: Some(14.5511856), max_longitude: Some(-90.39645209999999), min_longitude: Some(-90.5886688)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة غواتيمالا"), ("be", "дэпартамент Гватэмала"), ("bg", "Гватемала"), ("bn", "গোয\u{9bc}\u{9be}তেম\u{9be}ল\u{9be} বিভ\u{9be}গ"), ("bs", "Guatemala"), ("ca", "Departament de Guatemala"), ("ccp", "𑄉\u{1112a}𑄠𑄑𑄬𑄟𑄣"), ("ceb", "Departamento de Guatemala"), ("da", "Guatemala Department"), ("de", "Departamento Guatemala"), ("el", "Γουατεμάλα"), ("en", "Guatemala"), ("es", "Guatemala"), ("eu", "Guatemala"), ("fa", "استان گواتمالا"), ("fi", "Guatemala"), ("fr", "département de Guatemala"), ("gl", "Departamento de Guatemala"), ("gu", "ગ\u{acd}વાટ\u{ac7}માલા વિભાગ"), ("hi", "ग\u{94d}वाट\u{947}माला विभाग"), ("hr", "Razgovor:Guatemala"), ("hu", "Guatemala megye"), ("id", "Departemen Guatemala"), ("it", "dipartimento di Guatemala"), ("ja", "グアテマラ県"), ("ka", "გვატემალის დეპარტამენტი"), ("kn", "ಗ\u{ccd}ವಾಟ\u{cc6}ಮಾಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "과테말라 주"), ("lt", "Gvatemalos departamentas"), ("lv", "Gvatemalas departements"), ("mr", "ग\u{94d}वात\u{947}माला विभाग"), ("ms", "Guatemala Department"), ("nb", "Guatemala"), ("nl", "Guatemala"), ("no", "Guatemala"), ("pl", "Gwatemala"), ("pt", "Guatemala"), ("ru", "Гватемала"), ("si", "ගෝතම\u{dcf}ල\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Guatemala"), ("sv", "Guatemala"), ("ta", "குட\u{bcd}டேமலை துறை"), ("te", "గ\u{c4d}వ\u{c3e}ట\u{c46}మ\u{c3e}ల\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ก\u{e31}วเตมาลา"), ("tr", "Guatemala Departmanı"), ("uk", "Гватемала"), ("ur", "گواتیمالا محکمہ"), ("vi", "Guatemala"), ("zh", "瓜地馬拉省")]),
                        unofficial_name_list: ["Guatemala"].to_vec(),
                    }
                ),
                (
                    "HU",
                    Subdivision{
                        name: "HU",
                        country_alpha2: Alpha2::GT,
                        code: "HU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.314722), longitude: Some(-91.47611099999999), max_latitude: Some(15.3472653), min_latitude: Some(15.2881177), max_longitude: Some(-91.44650449999999), min_longitude: Some(-91.5365411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة هيويتينانغو"), ("bg", "Уеуетенанго"), ("bn", "হিউইউইটেনংগো বিভ\u{9be}গ"), ("bs", "Huehuetenango"), ("ca", "Departament de Huehuetenango"), ("ccp", "𑄦\u{11128}𑄅\u{1112a}𑄦\u{11128}𑄅\u{1112a}𑄑𑄬𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Departamento de Huehuetenango"), ("da", "Huehuetenango Department"), ("de", "Departamento Huehuetenango"), ("el", "Χουεχουετενάνγκο"), ("en", "Huehuetenango"), ("es", "Huehuetenango"), ("eu", "Huehuetenango"), ("fi", "Huehuetenangon departmentti"), ("fr", "département de Huehuetenango"), ("gl", "Departamento de Huehuetenango"), ("gu", "હ\u{ac1}એહ\u{ac1}એટ\u{ac7}ના\u{a82}ગો વિભાગ"), ("hi", "ह\u{94d}य\u{942}एह\u{941}त\u{947}न\u{947}\u{902}गो विभाग"), ("hu", "Huehuetenango megye"), ("id", "Departemen Huehuetenango"), ("it", "dipartimento di Huehuetenango"), ("ja", "ウェウェテナンゴ県"), ("ka", "უეუეტენანგოს დეპარტამენტი"), ("kn", "ಹ\u{ccd}ಯುಹ\u{cc6}ಟ\u{cc6}ನಾಂಗೊ ಇಲಾಖ\u{cc6}"), ("ko", "우에우에테낭고 주"), ("lt", "Huehuetenango departamentas"), ("lv", "Venetenango departaments"), ("mr", "ह\u{94d}व\u{947}ह\u{941}त\u{947}न\u{947}\u{902}गो विभाग"), ("ms", "Huehuetenango Department"), ("nb", "Huehuetenango"), ("nl", "Huehuetenango"), ("no", "Huehuetenango"), ("pl", "Huehuetenango"), ("pt", "Huehuetenango"), ("ru", "Уэуэтенанго"), ("si", "හ\u{dd2}ය\u{dd4}හ\u{dd2}ය\u{dd4}ටෙනන\u{dca}ගෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Huehuetenango"), ("sv", "Departamento de Huehuetenango"), ("ta", "ஹஎதுஎட\u{bcd}டெனங\u{bcd}கோ துறை"), ("te", "హుయ\u{c46}హుయ\u{c46}ట\u{c46}న\u{c3e}ంగ\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวเวเตน\u{e31}งโก"), ("tr", "Huehuetenango"), ("uk", "Уеуетенанго"), ("ur", "ہویہویتینانگو محکمہ"), ("vi", "Huehuetenango"), ("zh", "韋韋特南戈省")]),
                        unofficial_name_list: ["Huehuetenango"].to_vec(),
                    }
                ),
                (
                    "IZ",
                    Subdivision{
                        name: "IZ",
                        country_alpha2: Alpha2::GT,
                        code: "IZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4036847), longitude: Some(-89.1384441), max_latitude: Some(15.4070216), min_latitude: Some(15.4006451), max_longitude: Some(-89.13182259999999), min_longitude: Some(-89.1429805)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة إيزابال"), ("bg", "Исабал"), ("bn", "ইজ\u{9be}ব\u{9be}ল বিভ\u{9be}গ"), ("bs", "Izabal"), ("ca", "Izabal"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄝𑄣\u{11134}"), ("ceb", "Departamento de Izabal"), ("da", "Izabal Department"), ("de", "Departamento Izabal"), ("el", "Ιζαμπάλ"), ("en", "Izabal"), ("es", "Izabal"), ("eu", "Izabal"), ("fi", "Izabal"), ("fr", "département d’Izabal"), ("gl", "Departamento de Izabal"), ("gu", "ઇઝાબ\u{ac7}લ વિભાગ"), ("hi", "इसाब\u{947}ल विभाग"), ("hu", "Izabal megye"), ("id", "Departemen Izabal"), ("it", "dipartimento di Izabal"), ("ja", "イサバル県"), ("ka", "ისაბალის დეპარტამენტი"), ("kn", "ಇಝಾಬಾಲ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "이사발 주"), ("lt", "Isabalio departamentas"), ("lv", "Isabalas departaments"), ("mr", "इझाबल विभाग"), ("ms", "Izabal Department"), ("nb", "Izabal"), ("nl", "Izabal"), ("no", "Izabal"), ("pl", "Izabal"), ("pt", "Izabal"), ("ru", "Исабаль"), ("si", "ඉසබල\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Izabal"), ("sv", "Departamento de Izabal"), ("ta", "இசபெல\u{bcd} துறை"), ("te", "ఇజ\u{c3e}బల\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ไอซาเบล ด\u{e35}พาทเม\u{e49}น"), ("tr", "Izabel Departmanı"), ("uk", "Ісабаль"), ("ur", "اسابال محکمہ"), ("vi", "Izabal"), ("zh", "伊薩瓦爾省")]),
                        unofficial_name_list: ["Izabal"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::GT,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.5655154), longitude: Some(-89.9253233), max_latitude: Some(14.8639619), min_latitude: Some(14.421895), max_longitude: Some(-89.624861), min_longitude: Some(-90.261681)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة جالابا"), ("bg", "Халапа"), ("bn", "জ\u{9cd}য\u{9be}\u{9be}ল\u{9be}প\u{9be} বিভ\u{9be}গ"), ("bs", "Jalapa"), ("ca", "Departament de Jalapa"), ("ccp", "𑄎𑄣𑄛"), ("ceb", "Departamento de Jalapa"), ("da", "Jalapa Department"), ("de", "Departamento Jalapa"), ("el", "Τζαλάπα"), ("en", "Jalapa"), ("es", "Jalapa"), ("eu", "Jalapa"), ("fi", "Jalapan Departementti"), ("fr", "département de Jalapa"), ("gl", "Departamento de Jalapa"), ("gu", "જલાપા વિભાગ"), ("hi", "जालपा विभाग"), ("hu", "Jalapa megye"), ("id", "Departemen Jalapa"), ("it", "dipartimento di Jalapa"), ("ja", "ハラパ県"), ("ka", "ხალაპის დეპარტამენტი"), ("kn", "ಜಲಪ ಇಲಾಖ\u{cc6}"), ("ko", "할라파 주"), ("lt", "Chalapos departamentas"), ("lv", "Halapas departaments"), ("mr", "जलपा विभाग"), ("ms", "Jabatan Jalapa"), ("nb", "Jalapa"), ("nl", "Jalapa"), ("no", "Jalapa"), ("pl", "Jalapa"), ("pt", "Jalapa"), ("ru", "Халапа"), ("si", "ජලප\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Jalapa"), ("sv", "Departamento de Jalapa"), ("ta", "ஜலப\u{bbe} துறை"), ("te", "జల\u{c3e}ప\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ฮลาปา"), ("tr", "Jalapa Departmanı"), ("uk", "Халапа"), ("ur", "خالاپا محکمہ"), ("vi", "Jalapa"), ("zh", "哈拉帕省")]),
                        unofficial_name_list: ["Jalapa"].to_vec(),
                    }
                ),
                (
                    "JU",
                    Subdivision{
                        name: "JU",
                        country_alpha2: Alpha2::GT,
                        code: "JU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.282778), longitude: Some(-89.8925), max_latitude: Some(14.3246006), min_latitude: Some(14.2331868), max_longitude: Some(-89.83949659999999), min_longitude: Some(-89.9115944)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة خوتيابا"), ("bg", "Хутиапа"), ("bn", "জ\u{9c1}তিপ\u{9be} বিভ\u{9be}গ"), ("bs", "Jutiapa"), ("ca", "Departament de Jutiapa"), ("ccp", "𑄎\u{1112a}𑄑\u{11128}𑄠𑄛"), ("ceb", "Departamento de Jutiapa"), ("da", "Jutiapa"), ("de", "Departamento Jutiapa"), ("el", "Τζουτιάπα"), ("en", "Jutiapa"), ("es", "Jutiapa"), ("eu", "Jutiapa"), ("fi", "Jutiapan depatermentti"), ("fr", "département de Jutiapa"), ("gl", "Departamento de Jutiapa"), ("gu", "જ\u{ac1}ટાયપા વિભાગ"), ("hi", "ज\u{941}तियापा विभाग"), ("hr", "Jutiapa"), ("hu", "Jutiapa megye"), ("id", "Jutiapa Department"), ("it", "dipartimento di Jutiapa"), ("ja", "フティアパ県"), ("ka", "ხუტიაპის დეპარტამენტი"), ("kn", "ಜುಟ\u{cbf}ಯಾ ಇಲಾಖ\u{cc6}"), ("ko", "후티아파 주"), ("lt", "Chutiapos departamentas"), ("lv", "Hutjapas departaments"), ("mr", "ज\u{941}टाआप विभाग"), ("ms", "Jutiapa Department"), ("nb", "Jutiapa"), ("nl", "Jutiapa"), ("no", "Jutiapa"), ("pl", "Jutiapa"), ("pt", "Jutiapa"), ("ru", "Хутьяпа"), ("si", "ජ\u{dd4}ට\u{dd2}යප\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Jutiapa"), ("sv", "Departamento de Jutiapa"), ("ta", "ஜூடிப\u{bcd}ப\u{bbe} துறை"), ("te", "జుట\u{c3f}య\u{c3e}ప\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e39}เต\u{e35}ยปา"), ("tr", "Jutiapa Department"), ("uk", "Хутьяпа"), ("ur", "خوتیاپا محکمہ"), ("vi", "Jutiapa"), ("zh", "胡蒂亞帕省")]),
                        unofficial_name_list: ["Jutiapa"].to_vec(),
                    }
                ),
                (
                    "PE",
                    Subdivision{
                        name: "PE",
                        country_alpha2: Alpha2::GT,
                        code: "PE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.912033), longitude: Some(-90.2995785), max_latitude: Some(17.8152201), min_latitude: Some(15.833072), max_longitude: Some(-89.14571409999999), min_longitude: Some(-91.437546)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بيتين"), ("bg", "Петен"), ("bn", "পেটেন"), ("bs", "Petén"), ("ca", "El Petén"), ("ccp", "𑄛𑄑𑄬𑄚\u{11134}"), ("ceb", "Departamento del Petén"), ("cs", "Petén"), ("da", "El Petén"), ("de", "Departamento Petén"), ("el", "Πετέν"), ("en", "Petén"), ("es", "Petén"), ("eu", "Petén"), ("fi", "Petén"), ("fr", "département du Petén"), ("gl", "Departamento de El Petén"), ("gu", "પ\u{ac7}ટ\u{ac7}ન વિભાગ"), ("he", "מחוז פטן"), ("hi", "पीटन विभाग"), ("hu", "Petén megye"), ("id", "Departemen El Petén"), ("it", "dipartimento di Petén"), ("ja", "ペテン県"), ("ka", "პეტენის დეპარტამენტი"), ("kn", "ಪ\u{cc6}ಟ\u{cc6}ನ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "페텐 주"), ("lt", "Peteno departamentas"), ("lv", "Petenas departaments"), ("mr", "प\u{947}ट\u{947}न विभाग"), ("ms", "Peten Department"), ("nb", "Petén"), ("nl", "Petén"), ("no", "Petén"), ("pl", "Petén"), ("pt", "El Petén"), ("ru", "Петен"), ("si", "පෙටේන\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Petén"), ("sv", "Petén"), ("ta", "பேட\u{bcd}டன\u{bcd} துறை"), ("te", "ప\u{c46}ట\u{c46}న\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "พ\u{e35}เท\u{e47}น ด\u{e35}พาทเม\u{e49}น"), ("tr", "Peten Departmanı"), ("uk", "Петен"), ("ur", "پیتین محکمہ"), ("vi", "Khu vực hành chính Petén"), ("zh", "貝登省")]),
                        unofficial_name_list: ["Petén"].to_vec(),
                    }
                ),
                (
                    "PR",
                    Subdivision{
                        name: "PR",
                        country_alpha2: Alpha2::GT,
                        code: "PR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.86527), longitude: Some(-90.02031699999999), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة البروغريسو"), ("bg", "Ел Прогресо"), ("bn", "এল প\u{9cd}রোগ\u{9cd}রেসো বিভ\u{9be}গ"), ("bs", "El Progreso"), ("ca", "El Progreso"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄛\u{11133}𑄢\u{11127}\u{1112e}𑄉\u{11133}𑄢𑄬𑄥\u{1112e}"), ("ceb", "Departamento de El Progreso"), ("da", "El Progreso Department"), ("de", "Departamento El Progreso"), ("el", "Ελ Προγκρέσο"), ("en", "El Progreso"), ("es", "El Progreso"), ("eu", "El Progreso"), ("fi", "El Progreson departmentti"), ("fr", "département d’El Progreso"), ("gl", "Departamento de El Progreso"), ("gu", "અલ પ\u{acd}રોગ\u{acd}ર\u{ac7}સો વિભાગ"), ("hi", "एल प\u{94d}रोग\u{94d}र\u{947}सो विभाग"), ("hu", "El Progreso megye"), ("id", "Departemen El Progreso"), ("it", "dipartimento di El Progreso"), ("ja", "エル・プログレソ県"), ("ka", "ელ-პროგერსოს დეპარტამენტი"), ("kn", "ಎಲ\u{ccd} ಪ\u{ccd}ರೋಗ\u{ccd}ರ\u{cc6}ಸ\u{ccd}ಸೋ ಇಲಾಖ\u{cc6}"), ("ko", "엘프로그레소 주"), ("lt", "El Progreso departamentas"), ("lv", "Elprogreso departaments"), ("mr", "एल प\u{94d}रोग\u{94d}र\u{947}स विभाग"), ("ms", "El Progreso Department"), ("nb", "El Progreso"), ("nl", "El Progreso"), ("no", "El Progreso"), ("pl", "El Progreso"), ("pt", "El Progreso"), ("ru", "Эль-Прогресо"), ("si", "එල\u{dca} ප\u{dca}රෝග\u{dca}රෙසෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "El Progreso"), ("sv", "Departamento de El Progreso"), ("ta", "எல\u{bcd} ப\u{bcd}ரோக\u{bcd}ரேசோ துறை"), ("te", "ఎల\u{c4d} ప\u{c4d}ర\u{c4b}గ\u{c4d}ర\u{c46}స\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เอลโปรเกโซ"), ("tr", "El Progreso Departmanı"), ("uk", "Ель-Прогресо"), ("ur", "ایل پروگریسو محکمہ"), ("vi", "El Progreso"), ("zh", "普羅格雷索省")]),
                        unofficial_name_list: ["El Progreso"].to_vec(),
                    }
                ),
                (
                    "QC",
                    Subdivision{
                        name: "QC",
                        country_alpha2: Alpha2::GT,
                        code: "QC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4983808), longitude: Some(-90.9820668), max_latitude: Some(16.0730049), min_latitude: Some(14.7989009), max_longitude: Some(-90.412424), min_longitude: Some(-91.3238779)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كويتشي"), ("bg", "Ел Киче"), ("bn", "কিচে বিভ\u{9be}গ"), ("bs", "Quiché"), ("ca", "El Quiché"), ("ccp", "𑄇\u{1112a}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Departamento del Quiché"), ("da", "Quiché Department"), ("de", "Departamento Quiché"), ("el", "Κουιτσέ"), ("en", "Quiché"), ("es", "Quiché"), ("eu", "Quiché"), ("fi", "Quichén departmentti"), ("fr", "département du Quiché"), ("gl", "Departamento de El Quiché"), ("gu", "ક\u{ac1}ચ\u{ac7} વિભાગ"), ("hi", "कीश विभाग"), ("hu", "Quiché megye"), ("id", "Departemen El Quiché"), ("it", "dipartimento di Quiché"), ("ja", "キチェ県"), ("ka", "კიჩეს დეპარტამენტი"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ಚ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "키체 주"), ("lt", "Kičės departamentas"), ("lv", "Kičes departaments"), ("mr", "क\u{94d}य\u{942}इच\u{947} विभाग"), ("ms", "Quiche Department"), ("nb", "Quiché"), ("nl", "Quiché"), ("no", "Quiché"), ("pl", "Quiché"), ("pt", "El Quiché"), ("ru", "Киче"), ("si", "ක\u{dd4}ය\u{dd2}චේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Quiché"), ("sv", "Departamento del Quiché"), ("ta", "குய\u{bcd}ச\u{bcd}சே துறை"), ("te", "క\u{c4d}వ\u{c3f}చ\u{c46} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "กาฮ\u{e4c}เซค"), ("tr", "Quiche Departmanı"), ("uk", "Кіче"), ("ur", "کیتشے محکمہ"), ("vi", "Quiché"), ("zh", "基切省")]),
                        unofficial_name_list: ["Quiché"].to_vec(),
                    }
                ),
                (
                    "QZ",
                    Subdivision{
                        name: "QZ",
                        country_alpha2: Alpha2::GT,
                        code: "QZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.833333), longitude: Some(-91.516667), max_latitude: Some(14.896184), min_latitude: Some(14.7787831), max_longitude: Some(-91.46487239999999), min_longitude: Some(-91.58203119999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قسم كويتزالتينانغو"), ("bg", "Кецалтенанго"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}েটজ\u{9be}লেন\u{9be}ং বিভ\u{9be}গ"), ("bs", "Quetzaltenango"), ("ca", "Departament de Quetzaltenango"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄖\u{11134}𑄎𑄣\u{11134}𑄑𑄬𑄚𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Departamento de Quetzaltenango"), ("da", "Quetzaltenango Department"), ("de", "Departamento Quetzaltenango"), ("el", "Κουετζαλτενάνγκο"), ("en", "Quetzaltenango"), ("es", "Quetzaltenango"), ("eu", "Quetzaltenango"), ("fa", "استان کتسالتنانگو"), ("fi", "Quetzaltenangon departementti"), ("fr", "département de Quetzaltenango"), ("gl", "Departamento de Quetzaltenango"), ("gu", "ક\u{acd}વ\u{ac7}ટઝાલ\u{acd}ત\u{ac7}ના\u{a82}ગો વિભાગ"), ("hi", "क\u{94d}व\u{947}टज\u{93c}ाल\u{94d}ट\u{947}न\u{948}\u{902}गो विभाग"), ("hr", "Quetzaltenango (departman)"), ("hu", "Quetzaltenango megye"), ("id", "Departemen Quetzaltenango"), ("it", "dipartimento di Quetzaltenango"), ("ja", "ケツァルテナンゴ県"), ("ka", "კეცალტენანგოს დეპარტამენტი"), ("kn", "ಕ\u{ccd}ವ\u{cc6}ಟ\u{ccd}ಝಲ\u{ccd}ಟ\u{cc6}ನಾಂಗೊ ಇಲಾಖ\u{cc6}"), ("ko", "케트살테낭고 주"), ("lt", "Kecaltenango departamentas"), ("lv", "Kesaltenango departaments"), ("mr", "क\u{94d}व\u{947}ट\u{94d}झलात\u{947}ना\u{902}गो विभाग"), ("ms", "Quetzaltenango Department"), ("nb", "Quetzaltenango"), ("nl", "Quetzaltenango"), ("no", "Quetzaltenango"), ("pl", "Quetzaltenango"), ("pt", "Quetzaltenango"), ("ru", "Кесальтенанго"), ("si", "කෙට\u{dca}සල\u{dca}ටේනංගෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Quetzaltenango"), ("sv", "Departamento de Quetzaltenango"), ("ta", "யூதசல\u{bcd}டெணங\u{bcd}கொ துறை"), ("te", "క\u{c4d}వ\u{c46}ట\u{c4d}జ\u{c3e}ల\u{c4d}ట\u{c46}న\u{c3e}ంగ\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกตซ\u{e31}ลเตน\u{e31}งโก"), ("tr", "Quetzaltenango"), ("uk", "Кесальтенанго"), ("ur", "کویتسالتینانگو محکمہ"), ("vi", "Quetzaltenango"), ("zh", "克薩爾特南戈省")]),
                        unofficial_name_list: ["Quetzaltenango"].to_vec(),
                    }
                ),
                (
                    "RE",
                    Subdivision{
                        name: "RE",
                        country_alpha2: Alpha2::GT,
                        code: "RE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.533333), longitude: Some(-91.68333299999999), max_latitude: Some(14.555921), min_latitude: Some(14.5086458), max_longitude: Some(-91.661768), min_longitude: Some(-91.7044258)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ريتاليوليو"), ("bg", "Реталулеу"), ("bn", "রেত\u{9be}লহোলে বিভ\u{9be}গ"), ("bs", "Retalhuleu"), ("ca", "Departament de Retalhuleu"), ("ccp", "𑄢𑄬𑄑𑄣\u{11134}𑄦\u{1112a}𑄣\u{11128}𑄅\u{1112a}"), ("ceb", "Departamento de Retalhuleu"), ("da", "Retalhuleu"), ("de", "Departamento Retalhuleu"), ("el", "Ρεταλουλέου"), ("en", "Retalhuleu"), ("es", "Retalhuleu"), ("eu", "Retalhuleu"), ("fi", "Retalhuleun departmentti"), ("fr", "département de Retalhuleu"), ("gl", "Departamento de Retalhuleu"), ("gu", "ર\u{ac7}ટાલહ\u{ac1}લ\u{ac7}ઉ વિભાગ"), ("hi", "र\u{947}तलह\u{941}ल\u{941} विभाग"), ("hr", "Retalhuleu (departman)"), ("hu", "Retalhuleu megye"), ("id", "Departemen Retalhuleu"), ("it", "dipartimento di Retalhuleu"), ("ja", "レタルレウ県"), ("ka", "რეტალულეუს დეპარტამენტი"), ("kn", "ರ\u{cc6}ಟಲ\u{ccd}ಹ\u{ccd}ಯ\u{cc2}ಲು ಇಲಾಖ\u{cc6}"), ("ko", "레탈룰레우 주"), ("lt", "Retalhuleu departamentas"), ("lv", "Retaluleu departaments"), ("mr", "र\u{947}टलहॉल\u{942} विभाग"), ("ms", "Retalhuleu Department"), ("nb", "Retalhuleu"), ("nl", "Retalhuleu"), ("no", "Retalhuleu"), ("pl", "Retalhuleu"), ("pt", "Retalhuleu"), ("ru", "Реталулеу"), ("si", "රෙටල\u{dca}හ\u{dd4}ලෙව\u{dd6} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Retalhuleu"), ("sv", "Departamento de Retalhuleu"), ("ta", "ரெட\u{bcd}ட\u{bbe}ல\u{bcd}ஹலேயு துறை"), ("te", "ర\u{c3f}ట\u{c3e}ల\u{c4d}హుల\u{c47}వు డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ร\u{e35}ท\u{e31}ลค\u{e39}เลย\u{e39} ด\u{e35}พาทเม\u{e49}น"), ("tr", "Retalhuleu Departmanı"), ("uk", "Реталулеу"), ("ur", "ریتالولیو محکمہ"), ("vi", "Retalhuleu"), ("zh", "雷塔盧萊烏省")]),
                        unofficial_name_list: ["Retalhuleu"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::GT,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.6110576), longitude: Some(-90.7152749), max_latitude: Some(14.771409), min_latitude: Some(14.3943199), max_longitude: Some(-90.59746299999999), min_longitude: Some(-90.876255)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساكاتيبيكيز"), ("bg", "Сакатепекес"), ("bn", "স\u{9be}ক\u{9be}তেপেক\u{9c1}য\u{9bc}েজ বিভ\u{9be}গ"), ("bs", "Sacatepéquez"), ("ca", "Sacatepéquez"), ("ccp", "𑄥𑄇𑄑𑄬𑄛\u{11134}𑄇\u{1112a}𑄠𑄬𑄌\u{11134}"), ("ceb", "Departamento de Sacatepéquez"), ("da", "Sacatepéquez Department"), ("de", "Sacatepéquez"), ("el", "Σακατεπέκεζ"), ("en", "Sacatepéquez"), ("es", "Sacatepéquez"), ("eu", "Sacatepéquez"), ("fa", "استان ساکاتپکس"), ("fi", "Sacatepéquezin departementti"), ("fr", "département de Sacatepéquez"), ("gl", "Departamento de Sacatepéquez"), ("gu", "સાકાટ\u{ac7}પ\u{ac7}ક\u{acd}વિઝ વિભાગ"), ("hi", "सकाट\u{947}प\u{947}क\u{947}ज\u{93c}"), ("hr", "Razgovor:Sacatepéquez"), ("hu", "Sacatepéquez megye"), ("id", "Departemen Sacatepéquez"), ("it", "dipartimento di Sacatepéquez"), ("ja", "サカテペケス県"), ("ka", "საკატეპეკესის დეპარტამენტი"), ("kn", "ಸಕ\u{cc6}ಟ\u{cc6}ಪ\u{cc6}ಕ\u{ccd}ವ\u{cc6}ಜ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "사카테페케스 주"), ("lt", "Sakatepekeso departamentas"), ("lv", "Sakatepekeso departaments"), ("mr", "सिक\u{94d}व\u{947}ट\u{947}प\u{947}क\u{94d}य\u{942}झ विभाग"), ("ms", "Sacatepequez Department"), ("nb", "Sacatepéquez"), ("nl", "Sacatepéquez"), ("no", "Sacatepéquez"), ("pl", "Sacatepéquez"), ("pt", "Sacatepéquez"), ("ru", "Сакатепекес"), ("si", "සකටෙපේක\u{dca}වෙස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Sacatepéquez"), ("sv", "Departamento de Sacatepéquez"), ("ta", "ச\u{bbe}க\u{bcd}க\u{bbe}டேபெக\u{bcd}ஸ\u{bcd} துறை"), ("te", "స\u{c3e}క\u{c4b}జ\u{c46}పక\u{c40}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซากาเตเปเกซ"), ("tr", "Sacatepeques District"), ("uk", "Сакатепекес"), ("ur", "ساکاتیپیکیس محکمہ"), ("vi", "Sacatepéquez"), ("zh", "薩卡特佩克斯省")]),
                        unofficial_name_list: ["Sacatepéquez"].to_vec(),
                    }
                ),
                (
                    "SM",
                    Subdivision{
                        name: "SM",
                        country_alpha2: Alpha2::GT,
                        code: "SM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.9309569), longitude: Some(-91.9099238), max_latitude: Some(15.432205), min_latitude: Some(14.4866105), max_longitude: Some(-91.58420199999999), min_longitude: Some(-92.2414908)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سان ماركوس"), ("bg", "Сан Маркос"), ("bn", "স\u{9be}ন ম\u{9be}র\u{9cd}কো বিভ\u{9be}গ"), ("bs", "San Marcos"), ("ca", "Departament de San Marcos"), ("ccp", "𑄥𑄚\u{11134} 𑄟𑄢\u{11134}𑄇\u{1112e}𑄌\u{11134}"), ("ceb", "Departamento de San Marcos"), ("da", "San Marcos Department"), ("de", "Departamento San Marcos"), ("el", "Σαν Μάρκος"), ("en", "San Marcos"), ("es", "San Marcos"), ("eu", "San Marcos"), ("fi", "San Marcosin departmentti"), ("fr", "département de San Marcos"), ("gl", "Departamento de San Marcos"), ("gu", "સ\u{ac7}ન માર\u{acd}કોસ વિભાગ"), ("hi", "स\u{948}न मार\u{94d}कोस विभाग"), ("hu", "San Marcos megye"), ("id", "Departemen San Marcos"), ("it", "dipartimento di San Marcos"), ("ja", "サン・マルコス県"), ("ka", "სან-მარკოსის დეპარტამენტი"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಮಾರ\u{ccd}ಕೊಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "산마르코스 주"), ("lt", "San Markoso departamentas"), ("lv", "Sanmarkosas departaments"), ("mr", "स\u{945}न मार\u{94d}कोस विभाग"), ("ms", "San Marcos Department"), ("nb", "San Marcos (departement)"), ("nl", "San Marcos"), ("no", "San Marcos (departement)"), ("pl", "San Marcos"), ("pt", "San Marcos"), ("ru", "Сан-Маркос"), ("si", "සැන\u{dca} ම\u{dcf}ර\u{dca}කෝස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "San Marcos"), ("sv", "Departamento de San Marcos"), ("ta", "ச\u{bbe}ன\u{bcd} ம\u{bbe}ர\u{bcd}கோஸ\u{bcd} துறை"), ("te", "స\u{c3e}న\u{c4d} మ\u{c3e}ర\u{c4d}క\u{c4b}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตซ\u{e31}นมาร\u{e4c}โกส"), ("tr", "San Marcos Departmanı"), ("uk", "Сан-Маркос"), ("ur", "سان مارکوس محکمہ"), ("vi", "San Marcos"), ("zh", "聖馬科斯省")]),
                        unofficial_name_list: ["San Marcos"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::GT,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.773889), longitude: Some(-91.1875), max_latitude: Some(14.78758), min_latitude: Some(14.7473686), max_longitude: Some(-91.1657524), min_longitude: Some(-91.1936903)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سولولا"), ("bg", "Солола"), ("bn", "স\u{9cd}লিয\u{9bc}েম\u{9be}"), ("bs", "Sololá"), ("ca", "Departament de Sololá"), ("ccp", "𑄥\u{11127}𑄣\u{1112e}𑄣"), ("ceb", "Departamento de Sololá"), ("da", "Solola Department"), ("de", "Departamento Sololá"), ("el", "Σολολά"), ("en", "Sololá"), ("es", "Sololá"), ("eu", "Sololá"), ("fi", "Sololán departmentti"), ("fr", "département de Sololá"), ("gl", "Departamento de Sololá"), ("gu", "સોલોલા વિભાગ"), ("hi", "सोलोला विभाग"), ("hu", "Sololá megye"), ("id", "Departemen Sololá"), ("it", "dipartimento di Sololá"), ("ja", "ソロラ県"), ("ka", "სოლოლის დეპარტამენტი"), ("kn", "ಸೊಲೊಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "솔롤라 주"), ("lt", "Sololos departamentas"), ("lv", "Solosas departaments"), ("mr", "सोलोवा विभाग"), ("ms", "Solola Department"), ("nb", "Sololá"), ("nl", "Sololá"), ("no", "Sololá"), ("pl", "Sololá"), ("pt", "Sololá"), ("ru", "Солола"), ("si", "සොලොල\u{dcf} ඩ\u{dd2}ප\u{dcf}ර\u{dca}ට\u{dca}මන\u{dca}ට\u{dca}"), ("sk", "Sololá"), ("sv", "Departamento de Sololá"), ("ta", "ஸோலோல துறை"), ("te", "స\u{c4b}ల\u{c4b}ల\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตโซโลลา"), ("tr", "Solola Departmanı"), ("uk", "Солола"), ("ur", "سولولا محکمہ"), ("vi", "Sololá"), ("zh", "索洛拉省")]),
                        unofficial_name_list: ["Sololá"].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::GT,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.1928003), longitude: Some(-90.37483540000001), max_latitude: Some(14.501664), min_latitude: Some(13.7795542), max_longitude: Some(-90.01661399999999), min_longitude: Some(-90.606461)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سانتا روزا"), ("bg", "Санта Роса"), ("bn", "স\u{9be}ন\u{9cd}ত\u{9be} রোস\u{9be} বিভ\u{9be}গ"), ("bs", "Santa Rosa"), ("ca", "Departament de Santa Rosa"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄢\u{1112e}𑄥"), ("ceb", "Departamento de Santa Rosa"), ("da", "Santa Rosa Department"), ("de", "Departamento Santa Rosa"), ("el", "Σάντα Ρόζα"), ("en", "Santa Rosa"), ("es", "Santa Rosa"), ("eu", "Santa Rosa"), ("fa", "استان سانتا روسا، گواتمالا"), ("fi", "Santa Rosan departmentti"), ("fr", "département de Santa Rosa"), ("gl", "Departamento de Santa Rosa"), ("gu", "સાન\u{acd}તા રોઝા વિભાગ"), ("hi", "सा\u{902}ता रोजा विभाग"), ("hu", "Santa Rosa megye"), ("hy", "Սանտա Ռոսա"), ("id", "Departemen Santa Rosa"), ("it", "dipartimento di Santa Rosa"), ("ja", "サンタ・ローサ県"), ("ka", "სანტა-როსას დეპარტამენტი"), ("kn", "ಸಾಂಟಾ ರೋಸಾ ಇಲಾಖ\u{cc6}"), ("ko", "산타로사 주"), ("lt", "Santa Rosos departamentas"), ("lv", "Santarosas departaments"), ("mr", "सा\u{902}ता रोसा विभाग"), ("ms", "Santa Rosa Department"), ("nb", "Santa Rosa (departement)"), ("nl", "Santa Rosa"), ("no", "Santa Rosa (departement)"), ("pl", "Santa Rosa"), ("pt", "Santa Rosa"), ("ru", "Санта-Роса"), ("si", "සැන\u{dca}ට\u{dcf} රෝස\u{dcf} ඩ\u{dd2}ප\u{dcf}ර\u{dca}ට\u{dca}මන\u{dca}ට\u{dca}"), ("sk", "Santa Rosa"), ("sv", "Departamento de Santa Rosa (departement)"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} ரோச\u{bbe} துறை"), ("te", "స\u{c3e}ంట\u{c3e} ర\u{c4b}స\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ซ\u{e31}นตาโรซา"), ("tr", "Santa Rosa Departman"), ("uk", "Санта-Роса"), ("ur", "سانتا روسا محکمہ، گواتیمالا"), ("vi", "Santa Rosa"), ("zh", "聖羅薩省")]),
                        unofficial_name_list: ["Santa Rosa"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::GT,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.4215982), longitude: Some(-91.4048249), max_latitude: Some(14.733114), min_latitude: Some(14.044629), max_longitude: Some(-91.07959009999999), min_longitude: Some(-91.6621132)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سوشيتبيكيز"), ("bg", "Сучитепекес"), ("bn", "স\u{9c1}খীতেপেক\u{9c1}য\u{9bc}েজ বিভ\u{9be}গ"), ("bs", "Suchitepéquez"), ("ca", "Suchitepéquez"), ("ccp", "𑄥\u{1112a}𑄌\u{11128}𑄚\u{11134}𑄑𑄬𑄛\u{11134}𑄇\u{1112a}𑄠\u{11128}𑄌\u{11134}"), ("ceb", "Departamento de Suchitepéquez"), ("da", "Suchitepéquez Department"), ("de", "Departamento Suchitepéquez"), ("el", "Σουτσιτεπέκεζ"), ("en", "Suchitepéquez"), ("es", "Suchitepéquez"), ("eu", "Suchitepéquez"), ("fi", "Suchitepéquezin departmentti"), ("fr", "département de Suchitepéquez"), ("gl", "Departamento de Suchitepéquez"), ("gu", "સ\u{ac1}ચિટ\u{ac7}પ\u{ac7}ક\u{acd}વિઝ વિભાગ"), ("hi", "स\u{941}चित\u{948}प\u{947}क\u{94d}य\u{942}ज\u{93c} विभाग"), ("hu", "Suchitepéquez megye"), ("id", "Departemen Suchitepequez"), ("it", "dipartimento di Suchitepéquez"), ("ja", "スチテペケス県"), ("ka", "სუჩიტეპეკესის დეპარტამენტი"), ("kn", "ಸಸ\u{cbf}ಟ\u{cc6}ಪ\u{cc6}ಕ\u{ccd}ವ\u{cc6}ಜ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "수치테페케스 주"), ("lt", "Sučitepekeso departamentas"), ("lv", "Sučitepekesas departaments"), ("mr", "स\u{942}ईट\u{947}प\u{947}क\u{94d}य\u{942}झ विभाग"), ("ms", "Suchitepequez Department"), ("nb", "Suchitepéquez"), ("nl", "Suchitepéquez"), ("no", "Suchitepéquez"), ("pl", "Suchitepéquez"), ("pt", "Suchitepéquez"), ("ru", "Сучитепекес"), ("si", "ස\u{dd4}ච\u{dd2}ටෙපෙක\u{dca}වෙස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Suchitepéquez"), ("sv", "Departamento de Suchitepéquez"), ("ta", "சுச\u{bcd}சிட\u{bcd}டேபேயூஸ\u{bcd} துறை"), ("te", "సుష\u{c3f}ట\u{c46}ప\u{c46}క\u{c4d}వ\u{c46}జ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ช\u{e35}เตเปเกซ"), ("tr", "Suchitepéquez District"), ("uk", "Сучитепекес"), ("ur", "سوتشیتیپیکیس محکمہ"), ("vi", "Suchitepéquez"), ("zh", "蘇奇特佩克斯省")]),
                        unofficial_name_list: ["Suchitepéquez"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::GT,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.910833), longitude: Some(-91.360556), max_latitude: Some(14.9257106), min_latitude: Some(14.9010778), max_longitude: Some(-91.347971), min_longitude: Some(-91.3731193)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة توتونيكابان"), ("bg", "Тотоникапан"), ("bn", "ত\u{9c1}ত\u{9c1}নিক\u{9be}প\u{9be}ন বিভ\u{9be}গ"), ("bs", "Totonicapán"), ("ca", "Departament de Totonicapán"), ("ccp", "𑄑\u{1112e}𑄑\u{1112e}𑄚\u{11128}𑄇𑄛𑄚\u{11134}"), ("ceb", "Departamento de Totonicapán"), ("da", "Totonicapán Department"), ("de", "Departamento Totonicapán"), ("el", "Τοτονικαπάν"), ("en", "Totonicapán"), ("es", "Totonicapán"), ("eu", "Totonicapán"), ("fi", "Totonicapánin departmentti"), ("fr", "département de Totonicapán"), ("gl", "Departamento de Totonicapán"), ("gu", "ટોટનિક\u{ac7}કન વિભાગ"), ("hi", "टोटोनिक\u{948}पान विभाग"), ("hu", "Totonicapán megye"), ("id", "Totonicapán Department"), ("it", "dipartimento di Totonicapán"), ("ja", "トトニカパン県"), ("ka", "ტოტონიკაპანის დეპარტამენტი"), ("kn", "ಟೊಟೊನ\u{cbf}ಕಾಪಾನ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "토토니카판 주"), ("lt", "Totonikapano departamentas"), ("lv", "Totonipakana departaments"), ("mr", "टोटोनिक\u{945}पण विभाग"), ("ms", "Totonicapan Department"), ("nb", "Totonicapán (departement)"), ("nl", "Totonicapán"), ("no", "Totonicapán (departement)"), ("pl", "Totonicapán"), ("pt", "Totonicapán"), ("ru", "Тотоникапан"), ("si", "ටෝටෝන\u{dd2}කපන\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Totonicapán"), ("sv", "Departamento de Totonicapán"), ("ta", "டோடோனிக\u{bcd}கப\u{bcd}பன\u{bcd} துறை"), ("te", "ట\u{c4b}ట\u{c4b}న\u{c3f}క\u{c3e}పన\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โตโตน\u{e35}กาป\u{e31}ง ด\u{e35}พาร\u{e4c}ทเม\u{e49}น"), ("tr", "Totonicapán"), ("uk", "Тотонікапан"), ("ur", "توتونیکاپان محکمہ"), ("vi", "Totonicapán"), ("zh", "托托尼卡潘省")]),
                        unofficial_name_list: ["Totonicapán"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::GT,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.0784265), longitude: Some(-89.436391), max_latitude: Some(15.296872), min_latitude: Some(14.773436), max_longitude: Some(-89.129655), min_longitude: Some(-89.849375)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة زاكابا"), ("bg", "Сакапа"), ("bn", "জ\u{9be}ক\u{9be}প\u{9be} বিভ\u{9be}গ"), ("bs", "Zacapa"), ("ca", "Departament de Zacapa"), ("ccp", "𑄎𑄇𑄛"), ("ceb", "Departamento de Zacapa"), ("da", "Zacapa Department"), ("de", "Departamento Zacapa"), ("el", "Ζάκαπα"), ("en", "Zacapa"), ("es", "Zacapa"), ("eu", "Zacapa"), ("fi", "Zacapan depatermentti"), ("fr", "département de Zacapa"), ("gl", "Departamento de Zacapa"), ("gu", "ઝાકાપા વિભાગ"), ("hi", "ज\u{93c}ाकापा विभाग"), ("hu", "Zacapa megye"), ("id", "Departemen Zacapa"), ("it", "dipartimento di Zacapa"), ("ja", "サカパ県"), ("ka", "საკაპის დეპარტამენტი"), ("kn", "ಜಕಪಾ ಇಲಾಖ\u{cc6}"), ("ko", "사카파 주"), ("lt", "Sakapos departamentas"), ("lv", "Sakapas departaments"), ("mr", "झ\u{945}कपा विभाग"), ("ms", "Zacapa Department"), ("nb", "Zacapa"), ("nl", "Zacapa"), ("no", "Zacapa"), ("pl", "Zacapa"), ("pt", "Zacapa"), ("ru", "Сакапа"), ("si", "සකප\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Zacapa"), ("sv", "Departamento de Zacapa"), ("ta", "சக\u{bbe}ப\u{bbe} துறை"), ("te", "జక\u{c3e}\u{c3e}ప\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซากาปา"), ("tr", "Zacapa Department"), ("uk", "Сакапа"), ("ur", "ساکاپا محکمہ"), ("vi", "Zacapa"), ("zh", "薩卡帕省")]),
                        unofficial_name_list: ["Zacapa"].to_vec(),
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
#[cfg(feature = "gt")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GT,
        alpha3: Alpha3::GTM,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 502,
        currency_code: CurrencyCode::GTQ,
        gec: Some(GEC::GT),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::GUA),
        iso_long_name: "The Republic of Guatemala",
        iso_short_name: "Guatemala",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Guatemalan"),
        number: "320",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "GT",
        unofficial_name_list: ["Guatemala", "グアテマラ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Guatemala"),
            ("af", "Guatemala"),
            ("ak", "Guatemala"),
            ("am", "Guatemala"),
            ("an", "Guatemala"),
            ("ar", "غواتيمالا"),
            ("as", "গ\u{9c1}ৱ\u{9be}টেম\u{9be}ল\u{9be}"),
            ("ay", "Guatemala"),
            ("az", "Quatemala"),
            ("ba", "Guatemala"),
            ("be", "Гватэмала"),
            ("bg", "Гватемала"),
            ("bi", "Guatemala"),
            ("bn", "গ\u{9c1}য়\u{9be}তেম\u{9be}ল\u{9be}"),
            ("bn_IN", "গ\u{9c1}য়\u{9be}তেম\u{9be}ল\u{9be}"),
            ("br", "Guatemala"),
            ("bs", "Gvatemala"),
            ("ca", "Guatemala"),
            ("ce", "Гватемала"),
            ("ch", "Guatemala"),
            ("cs", "Guatemala"),
            ("cv", "Гватемала"),
            ("cy", "Gwatemala"),
            ("da", "Guatemala"),
            ("de", "Guatemala"),
            ("dv", "ގ\u{7aa}އ\u{7a6}ޓ\u{7ac}މ\u{7a7}ލ\u{7a7}"),
            ("dz", "གའ\u{f74}་ཏ\u{f72}་མ་ལ།"),
            ("ee", "Guatemala"),
            ("el", "Γουατεμάλα"),
            ("en", "Guatemala"),
            ("eo", "Gvatemalo"),
            ("es", "Guatemala"),
            ("et", "Guatemala"),
            ("eu", "Guatemala"),
            ("fa", "گواتمالا"),
            ("ff", "Guatemala"),
            ("fi", "Guatemala"),
            ("fo", "Guatemala"),
            ("fr", "Guatemala"),
            ("fy", "Gûatemala"),
            ("ga", "Guatamala"),
            ("gl", "Guatemala"),
            ("gn", "Guatemala"),
            ("gu", "ગ\u{acd}વાટ\u{ac7}માલા"),
            ("gv", "Yn Ghuatemaley"),
            ("ha", "Guatemala"),
            ("he", "גואטמלה"),
            ("hi", "ग\u{94d}वाट\u{947}माला"),
            ("hr", "Gvatemala"),
            ("ht", "Gwatemala"),
            ("hu", "Guatemala"),
            ("hy", "Գվատեմալա"),
            ("ia", "Guatemala"),
            ("id", "Guatemala"),
            ("io", "Guatemala"),
            ("is", "Gvatemala"),
            ("it", "Guatemala"),
            ("iu", "Guatemala"),
            ("ja", "グアテマラ"),
            ("ka", "გვატემალა"),
            ("ki", "Guatemala"),
            ("kk", "Гватемала"),
            ("kl", "Guatemala"),
            ("km", "ហ\u{17d2}គាតេម\u{17c9}ាឡា"),
            ("kn", "ಗ\u{ccd}ವಾಟ\u{cc6}ಮಾಲಾ"),
            ("ko", "과테말라"),
            ("ku", "Guatemala"),
            ("kv", "Гватемала"),
            ("kw", "Gwatemala"),
            ("ky", "Гватемала"),
            ("lo", "ປະເທດກ\u{ebb}ວເຕມາລາ"),
            ("lt", "Gvatemala"),
            ("lv", "Gvatemala"),
            ("mi", "Guatemala"),
            ("mk", "Гватемала"),
            ("ml", "ഗ\u{d4d}വ\u{d3e}ട\u{d4d}ടിമ\u{d3e}ല"),
            ("mn", "Гватемал"),
            ("mr", "ग\u{94d}वाट\u{947}माला"),
            ("ms", "Guatemala"),
            ("mt", "Gwatemala"),
            (
                "my",
                "ဂ\u{103d}ါတ\u{102e}မာလာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Guatemara"),
            ("nb", "Guatemala"),
            ("ne", "ग\u{94d}वाट\u{947}माला"),
            ("nl", "Guatemala"),
            ("nn", "Guatemala"),
            ("nv", "Guatemala"),
            ("oc", "Guatemala"),
            ("or", "ଗ\u{b4d}ବ\u{b3e}ଟେମ\u{b3e}ଲ\u{b3e}"),
            ("pa", "ਗ\u{a41}ਆਟ\u{a47}ਮਾਲਾ"),
            ("pi", "ग\u{94d}वाट\u{947}माला"),
            ("pl", "Gwatemala"),
            ("ps", "ګواتیمالا"),
            ("pt", "Guatemala"),
            ("pt_BR", "Guatemala"),
            ("ro", "Guatemala"),
            ("ru", "Гватемала"),
            ("rw", "Gwatemala"),
            ("sc", "Guatemala"),
            ("sd", "Guatemala"),
            ("si", "ගෝතම\u{dcf}ල\u{dcf}ව"),
            ("sk", "Guatemala"),
            ("sl", "Gvatemala"),
            ("so", "Guatemala"),
            ("sq", "Guatemalë"),
            ("sr", "Гватемала"),
            ("sv", "Guatemala"),
            ("sw", "Guatemala"),
            ("ta", "குவ\u{bbe}தம\u{bbe}ல\u{bbe}"),
            ("te", "గ\u{c4d}వ\u{c3e}ట\u{c47}మ\u{c3e}ల\u{c3e}"),
            ("tg", "Гватемала"),
            ("th", "ก\u{e31}วเตมาลา"),
            ("ti", "ጓቲማላ"),
            ("tk", "Gwatemala"),
            ("tl", "Gwatemala"),
            ("tr", "Guatemala"),
            ("tt", "Gуатемала"),
            ("ug", "گىۋاتېمالا"),
            ("uk", "Гватемала"),
            ("ur", "گوئٹے مالا"),
            ("uz", "Gvatemala"),
            ("ve", "Guatemala"),
            ("vi", "Gua-tê-ma-la"),
            ("wa", "Gwatemala"),
            ("wo", "Guwaatemaala"),
            ("xh", "Guatemala"),
            ("yo", "Guatẹmálà"),
            ("zh_CN", "瓜地马拉"),
            ("zh_HK", "危地馬拉"),
            ("zh_TW", "瓜地馬拉"),
            ("zu", "I-Guwathemala"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
