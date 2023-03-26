// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Nation of Brunei, the Abode of Peace

#[cfg(all(feature = "bn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BN;
    pub const ALPHA3: Alpha3 = Alpha3::BRN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 673;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::BND;
    pub const GEC: Option<GEC> = Some(GEC::BX);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::BRU);
    pub const ISO_SHORT_NAME: &str = "Brunei Darussalam";
    pub const ISO_LONG_NAME: &str = "The Nation of Brunei, the Abode of Peace";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ms"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ms"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Bruneian");
    pub const NUMBER: &str = "096";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("[A-Z]{2} ?\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "BN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Brunei", "ブルネイ・ダルサラーム"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Brunei Darussalam"),
        ("af", "Broenei"),
        ("ak", "Brunei Darussalam"),
        ("am", "Brunei Darussalam"),
        ("an", "Brunei Darussalam"),
        ("ar", "بروناي دار الس\u{651}لام"),
        (
            "as",
            "ব\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9c1}নেই ড\u{9be}ৰ\u{9c1}ছ\u{9be}ল\u{9be}ম",
        ),
        ("ay", "Brunei Darussalam"),
        ("az", "Bruney Darüssəlam"),
        ("ba", "Brunei Darussalam"),
        ("be", "Бруней-Дарусалам"),
        ("bg", "Бруней"),
        ("bi", "Brunei Darussalam"),
        ("bn", "ব\u{9cd}র\u{9c1}নেই দ\u{9be}র\u{9c1}সল\u{9be}ম"),
        ("bn_IN", "ব\u{9cd}র\u{9c1}নেই দ\u{9be}র\u{9c1}সল\u{9be}ম"),
        ("br", "Brunei Darussalam"),
        ("bs", "Bruneji"),
        ("ca", "Brunei (Negara Brunei Darussalam)"),
        ("ce", "Brunei Darussalam"),
        ("ch", "Brunei Darussalam"),
        ("cs", "Brunej"),
        ("cv", "Brunei Darussalam"),
        ("cy", "Brunei Darussalam"),
        ("da", "Brunei"),
        ("de", "Brunei Darussalam"),
        ("dv", "Brunei Darussalam"),
        ("dz", "བ\u{f74}་ར\u{f74}་ན་ཡ\u{f7a}་ ད་ར\u{f74}་ས་ལམ།"),
        ("ee", "Brunei Darussalam"),
        ("el", "Μπρουνέι Νταρουσαλάμ"),
        ("en", "Brunei Darussalam"),
        ("eo", "Brunejo"),
        ("es", "Brunei Darussalam"),
        ("et", "Brunei Darussalami Riik"),
        ("eu", "Brunei Darussalam"),
        ("fa", "برونئی دارالسلام"),
        ("ff", "Brunei Darussalam"),
        ("fi", "Brunei Darussalamin valtio"),
        ("fo", "Brunei Darussalam"),
        ("fr", "Brunéi Darussalam"),
        ("fy", "Brunei Darussalam"),
        ("ga", "Brúiné Dárasalám"),
        ("gl", "Brunei Darussalam"),
        ("gn", "Brunei Darussalam"),
        ("gu", "બ\u{acd}ર\u{ac1}ન\u{ac7}ઇ દાર\u{ac1}સલ\u{ac7}મ"),
        ("gv", "Brunei Darussalam"),
        ("ha", "Brunei Darussalam"),
        ("he", "ברונאי דרוסלאלם"),
        ("hi", "ब\u{94d}र\u{941}न\u{947}ई दरउस\u{94d}सलाम"),
        ("hr", "Brunej Darussalam"),
        ("ht", "Brunei Darussalam"),
        ("hu", "Brunei Darussalam Állam"),
        ("hy", "Բրունեյ"),
        ("ia", "Brunei Darussalam"),
        ("id", "Brunei Darussalam"),
        ("io", "Brunei Darussalam"),
        ("is", "Brúnei Darussalam"),
        ("it", "Brunei"),
        ("iu", "Brunei Darussalam"),
        ("ja", "ブルネイ・ダルサラーム国"),
        ("ka", "ბრუნეი დარუსალამი"),
        ("ki", "Brunei Darussalam"),
        ("kk", "Бруней"),
        ("kl", "Brunei Darussalam"),
        (
            "km",
            "ប\u{17d2}រ\u{17ca}\u{17bb}យណេ\u{200b}ដារ\u{17bb}យសាឡ\u{17b9}ម",
        ),
        ("kn", "ಬ\u{ccd}ರ\u{cc2}ನೈ ದಾರುಸ\u{ccd}ಸಲಾಂ"),
        ("ko", "브루나이 다루살람"),
        ("ku", "Brûney Darûssalam"),
        ("kv", "Brunei Darussalam"),
        ("kw", "Brunei Darussalam"),
        ("ky", "Бруней-Даруссалам"),
        ("lo", "Brunei Darussalam"),
        ("lt", "Brunėjaus Darusalamas"),
        ("lv", "Brunejas Darusalamas Valsts"),
        ("mi", "Brunei Darussalam"),
        ("mk", "Брунеи Дарусалам"),
        (
            "ml",
            "ബ\u{d4d}ര\u{d42}ണെയ\u{d4d} ദ\u{d3e}റ\u{d41}സ\u{d4d}സല\u{d3e}ം",
        ),
        ("mn", "Брунейн вант улс"),
        ("mr", "ब\u{94d}र\u{941}नोइ दार\u{941}सलाम"),
        ("ms", "Brunei Darussalam"),
        ("mt", "Brunei Darussalam"),
        ("my", "Brunei Darussalam"),
        ("na", "Brunei Darussalam"),
        ("nb", "Brunei Darussalam"),
        ("ne", "ब\u{94d}र\u{941}नाई दार\u{941}सलाम"),
        ("nl", "Brunei"),
        ("nn", "Brunei"),
        ("nv", "Brunei Darussalam"),
        ("oc", "Brunei"),
        ("or", "ବ\u{b4d}ର\u{b41}ନେଈ ଦ\u{b3e}ଋସ\u{b3e}ଲମ"),
        ("pa", "ਬਰ\u{a42}ਨਈ ਡਾਰ\u{a42}ਸ਼ਲਾਮ"),
        ("pi", "Brunei Darussalam"),
        ("pl", "Państwo Brunei"),
        ("ps", "Brunei Darussalam"),
        ("pt", "Brunei"),
        ("pt_BR", "Brunei"),
        ("ro", "Brunei"),
        ("ru", "Бруней Даруссалам"),
        ("rw", "Buruneyi Darisalamu"),
        ("sc", "Brunei Darussalam"),
        ("sd", "Brunei Darussalam"),
        ("si", "බෲන\u{dcf}ය\u{dd2} දර\u{dd4}සලම\u{dca}"),
        ("sk", "Brunejsko-darussalamský štát"),
        ("sl", "Brunej"),
        ("so", "Brunei Darussalam"),
        ("sq", "Brunei"),
        ("sr", "Султанат Брунеји"),
        ("sv", "Brunei"),
        ("sw", "Brunei Darussalam"),
        ("ta", "ப\u{bcd}ரூனே தருசலம\u{bcd}"),
        ("te", "బ\u{c4d}రున\u{c4b}ఇ ద\u{c3e}రుసల\u{c3e}మ"),
        ("tg", "Ҷумҳурии Брунейи Доруссалом"),
        ("th", "บร\u{e39}ไนดาร\u{e38}สซาลาม"),
        ("ti", "Brunei Darussalam"),
        ("tk", "Bruneý Darussalam"),
        ("tl", "Brunei Darussalam"),
        ("tr", "Brunei Krallığı"),
        ("tt", "Брунеи Даруссалам"),
        ("ug", "بىرۇنېي دارېسسالام"),
        ("uk", "Бруней"),
        ("ur", "Brunei Darussalam"),
        ("uz", "Brunei Darussalam"),
        ("ve", "Brunei Darussalam"),
        ("vi", "Bợru-này Đa-ru-xa-làm"),
        ("wa", "Bruney"),
        ("wo", "Brunaay Daarussalaam"),
        ("xh", "Brunei Darussalam"),
        ("yo", "Brunei Darussalam"),
        ("zh_CN", "文莱"),
        ("zh_HK", "汶萊"),
        ("zh_TW", "汶萊"),
        ("zu", "Brunei Darussalam"),
    ];
    #[cfg(all(feature = "bn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 4.535277;
        pub const LONGITUDE: f64 = 114.727669;
        pub const MAX_LATITUDE: f64 = 5.0978001;
        pub const MAX_LONGITUDE: f64 = 115.3639552;
        pub const MIN_LATITUDE: f64 = 4.002460999999999;
        pub const MIN_LONGITUDE: f64 = 114.0752;
        pub const NORTHEAST_LATITUDE: f64 = 5.0978001;
        pub const NORTHEAST_LONGITUDE: f64 = 115.3639552;
        pub const SOUTHWEST_LATITUDE: f64 = 4.002460999999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 114.0752;
    }
}
#[cfg(all(feature = "bn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 4.535277,
            longitude: 114.727669,
            max_latitude: 5.0978001,
            max_longitude: 115.3639552,
            min_latitude: 4.002460999999999,
            min_longitude: 114.0752,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 5.0978001,
                    longitude: 115.3639552,
                },
                southwest: CountryGeoBound {
                    latitude: 4.002460999999999,
                    longitude: 114.0752,
                },
            },
        }
    }
}

#[cfg(all(feature = "bn", feature = "subdivisions"))]
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from(
            [

                (
                    "BE",
                    Subdivision{
                        name: "Belait",
                        country_alpha2: Alpha2::BN,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.449758), longitude: Some(114.318703), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيلايت"), ("bg", "Белайт"), ("bn", "বেল\u{9be}তি জেল\u{9be}"), ("ca", "Districte de Belait"), ("ccp", "𑄝𑄬𑄣\u{1112d}𑄖\u{11134}"), ("ceb", "Belait District"), ("da", "Belait District"), ("de", "Belait"), ("el", "Μπελάιτ"), ("en", "Belait"), ("es", "Distrito de Belait"), ("eu", "Belait barrutia"), ("fa", "استان بلائیت"), ("fi", "Belait"), ("fr", "Belait"), ("gl", "Belait"), ("gu", "બ\u{ac7}લ\u{ac7}ટ\u{ac7} જિલ\u{acd}લો"), ("he", "בלייט"), ("hi", "ब\u{947}ल\u{948}त जिला"), ("id", "Distrik Belait"), ("it", "distretto di Belait"), ("ja", "ブライト地区"), ("kn", "ಬೇಲೈತ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "블라잇 구"), ("lt", "Belaito rajonas"), ("lv", "Belaitas distrikts"), ("mr", "ब\u{947}ल\u{947}ट जिल\u{94d}हा"), ("ms", "Daerah Belait"), ("nb", "Belait distrikt"), ("nl", "Belait"), ("no", "Belait distrikt"), ("pl", "Dystrykt Belait"), ("ps", "بيلائت ولسوالۍ"), ("pt", "Belait"), ("ru", "Белайт"), ("si", "බෙලය\u{dd2}ට\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Belait District"), ("ta", "பெட\u{bcd}லைட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c46}ల\u{c46}య\u{c3f}ట\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเบอไลต\u{e4c}"), ("tr", "Belait District"), ("uk", "Белайт"), ("ur", "بیلائت ضلع"), ("vi", "Quận Belait"), ("zh", "马来奕县")]),
                        unofficial_name_list: ["Belait"].to_vec(),
                    }
                ),
                (
                    "BM",
                    Subdivision{
                        name: "Brunei-Muara",
                        country_alpha2: Alpha2::BN,
                        code: "BM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.9311206), longitude: Some(114.9516869), max_latitude: Some(5.045009900000001), min_latitude: Some(4.7286589), max_longitude: Some(115.1262673), min_longitude: Some(114.771896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بروناي - موارا"), ("bg", "Бруней-Муара"), ("bn", "ব\u{9cd}র\u{9c1}নেই মোয\u{9bc}\u{9be}র\u{9be} জেল\u{9be}"), ("ca", "Districte de Brunei Muara"), ("ccp", "𑄝\u{11133}𑄢\u{1112a}𑄚𑄬\u{1112d}-𑄟\u{11127}𑄅\u{1112a}𑄢"), ("ceb", "Brunei and Muara District"), ("da", "Brunei-Muara District"), ("de", "Brunei-Muara"), ("el", "Μπρουνέι-Μουάρα"), ("en", "Brunei-Muara"), ("es", "Brunei y Muara"), ("eu", "Brunei eta Muara"), ("fa", "استان برونئی-موآرا"), ("fi", "Brunei ja Muara"), ("fr", "Brunei-Muara"), ("gl", "Brunei e Muara"), ("gu", "બ\u{acd}ર\u{ac1}ન\u{ac7}ઈ-મ\u{ac1}રા જિલ\u{acd}લો"), ("he", "ברוניי ומוארה"), ("hi", "ब\u{94d}र\u{941}न\u{947}ई-म\u{941}रा जिला"), ("id", "Distrik Brunei-Muara"), ("it", "distretto di Brunei-Muara"), ("ja", "ブルネイ・ムアラ地区"), ("kn", "ಬ\u{ccd}ರ\u{cc2}ನ\u{cbf}-ಮುವಾರಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "브루네이무아라 구"), ("lt", "Brunėjaus–Muaros rajonas"), ("lv", "Bruneimuaras distrikts"), ("mr", "ब\u{94d}र\u{941}न\u{947}ई-म\u{941}रा जिल\u{94d}हा"), ("ms", "Daerah Brunei Muara"), ("nb", "Brunei-Muara distrik"), ("nl", "Brunei en Muara"), ("no", "Brunei-Muara distrik"), ("pl", "Dystrykt Brunei i Muara"), ("ps", "بروناي-موارا ولسوالۍ"), ("pt", "Brunei e Muara"), ("ru", "Бруней-Муара"), ("si", "බ\u{dca}ර\u{dd4}නෙය\u{dd2}-ම\u{dd2}ය\u{dd4}ර\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Brunei and Muara District"), ("ta", "ப\u{bcd}ருனெய\u{bcd} -முர\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c4d}రూన\u{c40}-మువ\u{c3e}ర\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบร\u{e39}ไน-ม\u{e31}วรา"), ("tr", "Brunedi-Muara District"), ("uk", "Бруней-Муара"), ("ur", "برونائی-موارا ضلع"), ("vi", "Quận Brunei-Muara"), ("zh", "汶莱摩拉县")]),
                        unofficial_name_list: ["Brunei-Muara"].to_vec(),
                    }
                ),
                (
                    "TE",
                    Subdivision{
                        name: "Temburong",
                        country_alpha2: Alpha2::BN,
                        code: "TE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.6204128), longitude: Some(115.141484), max_latitude: Some(4.9088199), min_latitude: Some(4.296721), max_longitude: Some(115.359444), min_longitude: Some(115.022453)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيمبورونغ"), ("bg", "Тембуронг"), ("bn", "টেম\u{9cd}ব\u{9c1}র\u{9be}ং জেল\u{9be}"), ("ca", "Districte de Temburong"), ("ccp", "𑄑𑄬𑄟\u{11134}𑄝\u{1112a}𑄢\u{11127}\u{11101}"), ("ceb", "Temburong District"), ("cs", "Temburong"), ("da", "Temburong District"), ("de", "Temburong"), ("el", "Τεμπουρόνγκ"), ("en", "Temburong"), ("es", "Temburong"), ("eu", "Temburong barrutia"), ("fa", "استان تمبورونگ"), ("fi", "Temburong"), ("fr", "Temburong"), ("gl", "Temburong"), ("gu", "ટ\u{ac7}મ\u{acd}બ\u{ac1}રો\u{a82}ગ જિલ\u{acd}લો"), ("he", "טמבורונג"), ("hi", "ट\u{947}म\u{94d}ब\u{94d}य\u{941}रो\u{902}ग जिला"), ("id", "Distrik Temburong"), ("it", "distretto di Temburong"), ("ja", "テンブロン地区"), ("kn", "ಟ\u{cc6}ಂಬರ\u{ccd}ಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "템부롱 구"), ("lt", "Temburongo rajonas"), ("lv", "Temburonas distrikts"), ("mr", "ट\u{947}\u{902}ब\u{941}रौ\u{901}ग जिल\u{94d}हा"), ("ms", "Daerah Temburong"), ("nb", "Temburong distrikt"), ("nl", "Temburong"), ("no", "Temburong distrikt"), ("pl", "Dystrykt Temburong"), ("ps", "تيمبورونگ ولسوالۍ"), ("pt", "Temburong"), ("ru", "Тембуронг"), ("si", "ටෙම\u{dca}බ\u{dd4}රෝන\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Temburong District"), ("ta", "டெம\u{bcd}புரோங\u{bcd}கு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c46}ంబుర\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเต\u{e34}มบ\u{e39}รง"), ("tr", "Temburong District"), ("uk", "Тембуронґ"), ("ur", "تیمبورونگ ضلع"), ("vi", "Quận Temburong"), ("zh", "淡布隆县")]),
                        unofficial_name_list: ["Temburong"].to_vec(),
                    }
                ),
                (
                    "TU",
                    Subdivision{
                        name: "Tutong",
                        country_alpha2: Alpha2::BN,
                        code: "TU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.7140373), longitude: Some(114.6667939), max_latitude: Some(4.9333759), min_latitude: Some(4.3042129), max_longitude: Some(114.8825069), min_longitude: Some(114.5288525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة توتونغ"), ("bg", "Тутонг"), ("bn", "ত\u{9c1}তং জেল\u{9be}"), ("ca", "Districte de Tutong"), ("ccp", "𑄑\u{11128}𑄅\u{1112a}𑄖\u{11127}\u{11101}"), ("ceb", "Tutong District"), ("da", "Tutong District"), ("de", "Tutong"), ("el", "Τουτόνγκ"), ("en", "Tutong"), ("es", "Distrito de Tutong"), ("eu", "Tutong barrutia"), ("fa", "منطقه توتنگ"), ("fi", "Tutong"), ("fr", "Tutong"), ("gl", "Tutong"), ("gu", "ટ\u{ac1}ટૉ\u{a82}ગ જિલ\u{acd}લો"), ("he", "טוטונג"), ("hi", "ट\u{942}टो\u{902}ग जिला"), ("id", "Distrik Tutong"), ("it", "distretto di Tutong"), ("ja", "ツトン地区"), ("kn", "ಟುಟೊಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "투통 구"), ("lt", "Tutongo rajonas"), ("lv", "Tutonas distrikts"), ("mr", "त\u{941}टो\u{902}ग जिल\u{94d}हा"), ("ms", "Daerah Tutong"), ("nb", "Tutong"), ("nl", "Tutong"), ("no", "Tutong"), ("pl", "Dystrykt Tutong"), ("ps", "توتنگ ولسوالۍ"), ("pt", "Tutong"), ("ru", "Тутонг"), ("si", "ට\u{dd4}ටොන\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Tutong District"), ("ta", "டுடோங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4d}యుట\u{c3e}ంగ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ต\u{e39}ตง"), ("tr", "Tutong District"), ("uk", "Тутонг"), ("ur", "توتنگ ضلع"), ("vi", "Quận Tutong"), ("zh", "都东县")]),
                        unofficial_name_list: ["Tutong"].to_vec(),
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
#[cfg(feature = "bn")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BN,
        alpha3: Alpha3::BRN,
        address_format: None,
        continent: Continent::Asia,
        country_code: 673,
        currency_code: CurrencyCode::BND,
        gec: Some(GEC::BX),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::BRU),
        iso_long_name: "The Nation of Brunei, the Abode of Peace",
        iso_short_name: "Brunei Darussalam",
        official_language_list: ["ms"].to_vec(),
        spoken_language_list: ["ms"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "0",
        nationality: Some("Bruneian"),
        number: "096",
        postal_code: true,
        postal_code_format: Some("[A-Z]{2} ?\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "BN",
        unofficial_name_list: ["Brunei", "ブルネイ・ダルサラーム"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Brunei Darussalam"),
            ("af", "Broenei"),
            ("ak", "Brunei Darussalam"),
            ("am", "Brunei Darussalam"),
            ("an", "Brunei Darussalam"),
            ("ar", "بروناي دار الس\u{651}لام"),
            (
                "as",
                "ব\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9c1}নেই ড\u{9be}ৰ\u{9c1}ছ\u{9be}ল\u{9be}ম",
            ),
            ("ay", "Brunei Darussalam"),
            ("az", "Bruney Darüssəlam"),
            ("ba", "Brunei Darussalam"),
            ("be", "Бруней-Дарусалам"),
            ("bg", "Бруней"),
            ("bi", "Brunei Darussalam"),
            ("bn", "ব\u{9cd}র\u{9c1}নেই দ\u{9be}র\u{9c1}সল\u{9be}ম"),
            ("bn_IN", "ব\u{9cd}র\u{9c1}নেই দ\u{9be}র\u{9c1}সল\u{9be}ম"),
            ("br", "Brunei Darussalam"),
            ("bs", "Bruneji"),
            ("ca", "Brunei (Negara Brunei Darussalam)"),
            ("ce", "Brunei Darussalam"),
            ("ch", "Brunei Darussalam"),
            ("cs", "Brunej"),
            ("cv", "Brunei Darussalam"),
            ("cy", "Brunei Darussalam"),
            ("da", "Brunei"),
            ("de", "Brunei Darussalam"),
            ("dv", "Brunei Darussalam"),
            ("dz", "བ\u{f74}་ར\u{f74}་ན་ཡ\u{f7a}་ ད་ར\u{f74}་ས་ལམ།"),
            ("ee", "Brunei Darussalam"),
            ("el", "Μπρουνέι Νταρουσαλάμ"),
            ("en", "Brunei Darussalam"),
            ("eo", "Brunejo"),
            ("es", "Brunei Darussalam"),
            ("et", "Brunei Darussalami Riik"),
            ("eu", "Brunei Darussalam"),
            ("fa", "برونئی دارالسلام"),
            ("ff", "Brunei Darussalam"),
            ("fi", "Brunei Darussalamin valtio"),
            ("fo", "Brunei Darussalam"),
            ("fr", "Brunéi Darussalam"),
            ("fy", "Brunei Darussalam"),
            ("ga", "Brúiné Dárasalám"),
            ("gl", "Brunei Darussalam"),
            ("gn", "Brunei Darussalam"),
            ("gu", "બ\u{acd}ર\u{ac1}ન\u{ac7}ઇ દાર\u{ac1}સલ\u{ac7}મ"),
            ("gv", "Brunei Darussalam"),
            ("ha", "Brunei Darussalam"),
            ("he", "ברונאי דרוסלאלם"),
            ("hi", "ब\u{94d}र\u{941}न\u{947}ई दरउस\u{94d}सलाम"),
            ("hr", "Brunej Darussalam"),
            ("ht", "Brunei Darussalam"),
            ("hu", "Brunei Darussalam Állam"),
            ("hy", "Բրունեյ"),
            ("ia", "Brunei Darussalam"),
            ("id", "Brunei Darussalam"),
            ("io", "Brunei Darussalam"),
            ("is", "Brúnei Darussalam"),
            ("it", "Brunei"),
            ("iu", "Brunei Darussalam"),
            ("ja", "ブルネイ・ダルサラーム国"),
            ("ka", "ბრუნეი დარუსალამი"),
            ("ki", "Brunei Darussalam"),
            ("kk", "Бруней"),
            ("kl", "Brunei Darussalam"),
            (
                "km",
                "ប\u{17d2}រ\u{17ca}\u{17bb}យណេ\u{200b}ដារ\u{17bb}យសាឡ\u{17b9}ម",
            ),
            ("kn", "ಬ\u{ccd}ರ\u{cc2}ನೈ ದಾರುಸ\u{ccd}ಸಲಾಂ"),
            ("ko", "브루나이 다루살람"),
            ("ku", "Brûney Darûssalam"),
            ("kv", "Brunei Darussalam"),
            ("kw", "Brunei Darussalam"),
            ("ky", "Бруней-Даруссалам"),
            ("lo", "Brunei Darussalam"),
            ("lt", "Brunėjaus Darusalamas"),
            ("lv", "Brunejas Darusalamas Valsts"),
            ("mi", "Brunei Darussalam"),
            ("mk", "Брунеи Дарусалам"),
            (
                "ml",
                "ബ\u{d4d}ര\u{d42}ണെയ\u{d4d} ദ\u{d3e}റ\u{d41}സ\u{d4d}സല\u{d3e}ം",
            ),
            ("mn", "Брунейн вант улс"),
            ("mr", "ब\u{94d}र\u{941}नोइ दार\u{941}सलाम"),
            ("ms", "Brunei Darussalam"),
            ("mt", "Brunei Darussalam"),
            ("my", "Brunei Darussalam"),
            ("na", "Brunei Darussalam"),
            ("nb", "Brunei Darussalam"),
            ("ne", "ब\u{94d}र\u{941}नाई दार\u{941}सलाम"),
            ("nl", "Brunei"),
            ("nn", "Brunei"),
            ("nv", "Brunei Darussalam"),
            ("oc", "Brunei"),
            ("or", "ବ\u{b4d}ର\u{b41}ନେଈ ଦ\u{b3e}ଋସ\u{b3e}ଲମ"),
            ("pa", "ਬਰ\u{a42}ਨਈ ਡਾਰ\u{a42}ਸ਼ਲਾਮ"),
            ("pi", "Brunei Darussalam"),
            ("pl", "Państwo Brunei"),
            ("ps", "Brunei Darussalam"),
            ("pt", "Brunei"),
            ("pt_BR", "Brunei"),
            ("ro", "Brunei"),
            ("ru", "Бруней Даруссалам"),
            ("rw", "Buruneyi Darisalamu"),
            ("sc", "Brunei Darussalam"),
            ("sd", "Brunei Darussalam"),
            ("si", "බෲන\u{dcf}ය\u{dd2} දර\u{dd4}සලම\u{dca}"),
            ("sk", "Brunejsko-darussalamský štát"),
            ("sl", "Brunej"),
            ("so", "Brunei Darussalam"),
            ("sq", "Brunei"),
            ("sr", "Султанат Брунеји"),
            ("sv", "Brunei"),
            ("sw", "Brunei Darussalam"),
            ("ta", "ப\u{bcd}ரூனே தருசலம\u{bcd}"),
            ("te", "బ\u{c4d}రున\u{c4b}ఇ ద\u{c3e}రుసల\u{c3e}మ"),
            ("tg", "Ҷумҳурии Брунейи Доруссалом"),
            ("th", "บร\u{e39}ไนดาร\u{e38}สซาลาม"),
            ("ti", "Brunei Darussalam"),
            ("tk", "Bruneý Darussalam"),
            ("tl", "Brunei Darussalam"),
            ("tr", "Brunei Krallığı"),
            ("tt", "Брунеи Даруссалам"),
            ("ug", "بىرۇنېي دارېسسالام"),
            ("uk", "Бруней"),
            ("ur", "Brunei Darussalam"),
            ("uz", "Brunei Darussalam"),
            ("ve", "Brunei Darussalam"),
            ("vi", "Bợru-này Đa-ru-xa-làm"),
            ("wa", "Bruney"),
            ("wo", "Brunaay Daarussalaam"),
            ("xh", "Brunei Darussalam"),
            ("yo", "Brunei Darussalam"),
            ("zh_CN", "文莱"),
            ("zh_HK", "汶萊"),
            ("zh_TW", "汶萊"),
            ("zu", "Brunei Darussalam"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
