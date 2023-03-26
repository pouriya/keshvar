// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Bahrain

#[cfg(all(feature = "bh", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::BH;
    pub const ALPHA3: Alpha3 = Alpha3::BHR;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 973;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::BHD;
    pub const GEC: Option<GEC> = Some(GEC::BA);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::BRN);
    pub const ISO_SHORT_NAME: &str = "Bahrain";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Bahrain";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Bahraini");
    pub const NUMBER: &str = "048";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(?:\\d|1[0-2])\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "BH";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Bahrain", "البحرين", "Bahreïn", "Bahrein", "バーレーン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bahrain"),
        ("af", "Bahrein"),
        ("ak", "Bahrain"),
        ("am", "ባህሳን"),
        ("an", "Bahrein"),
        ("ar", "البحرين"),
        ("as", "ব\u{9be}হৰেইন"),
        ("ay", "Bahrain"),
        ("az", "Bahreyn"),
        ("ba", "Bahrain"),
        ("be", "Бахрэйн"),
        ("bg", "Бахрейн"),
        ("bi", "Bahrain"),
        ("bn", "ব\u{9be}হরেইন"),
        ("bn_IN", "ব\u{9be}হরেইন"),
        ("br", "Bahrein"),
        ("bs", "Bahrein"),
        ("ca", "Bahrain"),
        ("ce", "Бахрейн"),
        ("ch", "Bahrain"),
        ("cs", "Bahrajn"),
        ("cv", "Бахрейн"),
        ("cy", "Bahrain"),
        ("da", "Bahrain"),
        ("de", "Bahrain"),
        ("dv", "ބ\u{7a6}ޙ\u{7b0}ރ\u{7a6}އ\u{7a8}ނ\u{7b0}"),
        ("dz", "བ\u{f71}་ར\u{f7a}ན།"),
        ("ee", "Bahrain"),
        ("el", "Μπαχρέιν"),
        ("en", "Bahrain"),
        ("eo", "Barejno"),
        ("es", "Baréin"),
        ("et", "Bahrein"),
        ("eu", "Bahrain"),
        ("fa", "بحرین"),
        ("ff", "Bahrain"),
        ("fi", "Bahrain"),
        ("fo", "Bahrain"),
        ("fr", "Bahreïn"),
        ("fy", "De Barein"),
        ("ga", "Bairéin"),
        ("gl", "Bahrein"),
        ("gn", "Bahrain"),
        ("gu", "બહ\u{ac7}રિન"),
        ("gv", "Bahrain"),
        ("ha", "Baharain"),
        ("he", "בחריין"),
        ("hi", "बहरीन"),
        ("hr", "Bahrein"),
        ("ht", "Barayn"),
        ("hu", "Bahrein"),
        ("hy", "Բահրեյն"),
        ("ia", "Bahrein"),
        ("id", "Bahrain"),
        ("io", "Bahrain"),
        ("is", "Barein"),
        ("it", "Bahrein"),
        ("iu", "Bahrain"),
        ("ja", "バーレーン"),
        ("ka", "ბაჰრეინი"),
        ("ki", "Bahrain"),
        ("kk", "Бахрейн"),
        ("kl", "Bahrain"),
        ("km", "បារ\u{17c9}ែន"),
        ("kn", "ಬಹರೈನ\u{ccd}"),
        ("ko", "바레인"),
        ("ku", "Bahrayîn"),
        ("kv", "Бахрейн"),
        ("kw", "Bahreyn"),
        ("ky", "Бахрейн"),
        ("lo", "Bahrain"),
        ("lt", "Bahreinas"),
        ("lv", "Bahreina"),
        ("mi", "Bahrain"),
        ("mk", "Бахреин"),
        ("ml", "ബഹറൈന\u{d4d}\u{200d}"),
        ("mn", "Бахрейн"),
        ("mr", "बहारिन"),
        ("ms", "Bahrain"),
        ("mt", "Baħrejn"),
        (
            "my",
            "ဘာရ\u{102d}န\u{103a}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Bahrain"),
        ("nb", "Bahrain"),
        ("ne", "बहराइन"),
        ("nl", "Bahrein"),
        ("nn", "Bahrain"),
        ("nv", "Bahrain"),
        ("oc", "Barein"),
        ("or", "ବ\u{b3e}ହ\u{b3e}ରୀନ"),
        ("pa", "ਬਹਿਰਾਨ"),
        ("pi", "बहर\u{948}न"),
        ("pl", "Bahrajn"),
        ("ps", "بحرین"),
        ("pt", "Barém"),
        ("pt_BR", "Barein"),
        ("ro", "Bahrein"),
        ("ru", "Бахрейн"),
        ("rw", "Bahirayini"),
        ("sc", "Bahrein"),
        ("sd", "بحرين"),
        ("si", "බහරේන\u{dca}"),
        ("sk", "Bahrajn"),
        ("sl", "Bahrajn"),
        ("so", "Baxrayn"),
        ("sq", "Bahrein"),
        ("sr", "Бахреин"),
        ("sv", "Bahrain"),
        ("sw", "Bahrain"),
        ("ta", "பஹ\u{bcd}ரெயின\u{bcd}"),
        ("te", "బహర\u{c3f}న\u{c4d}"),
        ("tg", "Баҳрайн"),
        ("th", "บาห\u{e4c}เรน"),
        ("ti", "ባህሬን"),
        ("tk", "Bahreýn"),
        ("tl", "Bahrain"),
        ("tr", "Bahreyn"),
        ("tt", "Баһрейн"),
        ("ug", "بەھرەين"),
        ("uk", "Бахрейн"),
        ("ur", "بحرین"),
        ("uz", "Bahrayn"),
        ("ve", "Bahrain"),
        ("vi", "Ba-rainh"),
        ("wa", "Bareyn"),
        ("wo", "Bahrayin"),
        ("xh", "Bahrain"),
        ("yo", "Báháráìnì"),
        ("zh_CN", "巴林"),
        ("zh_HK", "巴林"),
        ("zh_TW", "巴林"),
        ("zu", "Bahrain"),
    ];
    #[cfg(all(feature = "bh", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 26.0667;
        pub const LONGITUDE: f64 = 50.5577;
        pub const MAX_LATITUDE: f64 = 26.3469001;
        pub const MAX_LONGITUDE: f64 = 50.8509064;
        pub const MIN_LATITUDE: f64 = 25.5349999;
        pub const MIN_LONGITUDE: f64 = 50.324246;
        pub const NORTHEAST_LATITUDE: f64 = 26.3469001;
        pub const NORTHEAST_LONGITUDE: f64 = 50.8509064;
        pub const SOUTHWEST_LATITUDE: f64 = 25.5349999;
        pub const SOUTHWEST_LONGITUDE: f64 = 50.324246;
    }
}
#[cfg(all(feature = "bh", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 26.0667,
            longitude: 50.5577,
            max_latitude: 26.3469001,
            max_longitude: 50.8509064,
            min_latitude: 25.5349999,
            min_longitude: 50.324246,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 26.3469001,
                    longitude: 50.8509064,
                },
                southwest: CountryGeoBound {
                    latitude: 25.5349999,
                    longitude: 50.324246,
                },
            },
        }
    }
}

#[cfg(all(feature = "bh", feature = "subdivisions"))]
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([
            (
                "13",
                Subdivision {
                    name: "Al Manamah (Al ‘Asimah)",
                    country_alpha2: Alpha2::BH,
                    code: "13",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(26.2285161),
                        longitude: Some(50.5860497),
                        max_latitude: Some(26.247324),
                        min_latitude: Some(26.1945071),
                        max_longitude: Some(50.6259022),
                        min_longitude: Some(50.51370679999999),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Governorate,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "محافظة العاصمة"),
                        ("bg", "Столична провинция"),
                        ("ca", "Governació de la Capital"),
                        ("ccp", "𑄇\u{11133}𑄠𑄛\u{11128}𑄑𑄣\u{11134}"),
                        ("ceb", "Capital Governorate"),
                        ("de", "Hauptstadt-Gouvernement"),
                        ("en", "Capital"),
                        ("es", "Capital"),
                        ("eu", "Hiriburuaren eskualdea"),
                        ("fa", "استان عاصمه"),
                        ("fr", "Gouvernorat de la capitale"),
                        ("hu", "Főváros kormányzóság"),
                        ("id", "Kegubernuran Ibu Kota"),
                        ("it", "Governatorato della Capitale"),
                        ("ja", "首都県"),
                        ("ko", "수도 주"),
                        ("lt", "Sostinės muchafaza"),
                        ("nb", "Hovedstadsguvernementet"),
                        ("nl", "Hoofdstad"),
                        ("no", "Hovedstadsguvernementet"),
                        ("pl", "Muhafazat al-Asima"),
                        ("ps", "محافظه پلازمېنه(بحرين)"),
                        ("pt", "Província da Capital"),
                        ("ru", "Столичная мухафаза"),
                        ("sv", "Capital Governorate"),
                        ("uk", "Столична мухафаза"),
                        ("ur", "محافظہ دارالحکومت، بحرین"),
                        ("zh", "首都省"),
                    ]),
                    unofficial_name_list: ["Manama", "Manama", "Manama", "Manāmah", "al-Manāmah"]
                        .to_vec(),
                },
            ),
            (
                "14",
                Subdivision {
                    name: "Al Janubiyah",
                    country_alpha2: Alpha2::BH,
                    code: "14",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(25.9381018),
                        longitude: Some(50.5756887),
                        max_latitude: Some(26.138158),
                        min_latitude: Some(25.5798401),
                        max_longitude: Some(50.8223101),
                        min_longitude: Some(50.4545967),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Governorate,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "جنوبية"),
                        ("bg", "Южна провинция"),
                        ("ca", "Governació del Sud"),
                        ("ccp", "𑄥𑄅\u{1112a}𑄘𑄢\u{11133}𑄚\u{11134}"),
                        ("ceb", "Southern Governorate"),
                        ("en", "Southern"),
                        ("es", "Sur"),
                        ("eu", "Hegoaldeko eskualdea"),
                        ("fa", "جنوبیه"),
                        ("fr", "Gouvernorat méridional"),
                        ("hu", "Déli kormányzóság"),
                        ("hy", "Հարավային մուհաֆազա"),
                        ("id", "Kegubernuran Selatan"),
                        ("it", "Governatorato Meridionale"),
                        ("ja", "南部県"),
                        ("ko", "남부 주"),
                        ("lt", "Pietų muchafaza"),
                        ("nb", "Sørlige guvernement"),
                        ("nl", "Zuid"),
                        ("no", "Sørlige guvernement"),
                        ("pl", "Al-Muhafazat al-Dżanubijja"),
                        ("ps", "محافظه جنوبيه"),
                        ("pt", "Província do Sul"),
                        ("ru", "Южная мухафаза"),
                        ("sv", "Southern Governorate"),
                        ("tr", "Güney Valiliği"),
                        ("ur", "محافظہ جنوبیہ"),
                        ("zh", "南方省"),
                    ]),
                    unofficial_name_list: [
                        "Eastern",
                        "Hawa",
                        "Juzur H\u{328}awār",
                        "Southern",
                        "ash Sharqiyah",
                        "aš-Šarqīyah",
                    ]
                    .to_vec(),
                },
            ),
            (
                "15",
                Subdivision {
                    name: "Al Muharraq",
                    country_alpha2: Alpha2::BH,
                    code: "15",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(26.266941),
                        longitude: Some(50.63839),
                        max_latitude: None,
                        min_latitude: None,
                        max_longitude: None,
                        min_longitude: None,
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Governorate,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "محافظة المحرق"),
                        ("bg", "Мухарак"),
                        ("bn", "ম\u{9c1}হ\u{9be}রর\u{9be}ক গভর\u{9cd}নোরেট"),
                        ("ca", "Governació d’al-Muharraq"),
                        ("ccp", "𑄟\u{1112a}𑄦𑄢𑄇\u{11134}"),
                        ("ceb", "Muharraq Governorate"),
                        ("da", "Muharraq Governorate"),
                        ("de", "Gouvernement Muharraq"),
                        ("el", "Μουχαράκ"),
                        ("en", "Muharraq"),
                        ("es", "Muharraq"),
                        ("eu", "Muharraq eskualdea"),
                        ("fa", "استان محرق"),
                        ("fi", "Muharraqn kuvernoraatti"),
                        ("fr", "Gouvernorat de Muharraq"),
                        ("gu", "મ\u{ac1}હારક ગવર\u{acd}નોર\u{ac7}ટ"),
                        ("hi", "म\u{941}हरक गवर\u{94d}नर\u{947}ट"),
                        ("hu", "Muharrak kormányzóság"),
                        ("id", "Kegubernuran Al Muharraq"),
                        ("it", "Governatorato di Muharraq"),
                        ("ja", "ムハッラク県"),
                        ("kn", "ಮುಹರಾಕ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"),
                        ("ko", "무하라크 주"),
                        ("lt", "Muharako muchafaza"),
                        ("lv", "Muharrakas muhāfaza"),
                        ("mr", "म\u{941}हरक गव\u{94d}हर\u{94d}नोर\u{947}ट"),
                        ("ms", "Pentadbiran Al Muharraq"),
                        ("nb", "Muharraq guvernement"),
                        ("nl", "Muharraq Governorate"),
                        ("no", "Muharraq guvernement"),
                        ("pl", "Muhafazat al-Muharrak"),
                        ("ps", "محافظه محرق"),
                        ("pt", "Província de Muharraq"),
                        ("ru", "Мухаррак"),
                        ("si", "ම\u{dd4}හරක\u{dca} පළ\u{dcf}ත"),
                        ("sv", "Muharraq Governorate"),
                        ("ta", "முஹர\u{bcd}ரக\u{bcd} கோவெர\u{bcd}னோரே"),
                        (
                            "te",
                            "ముహ\u{c3e}ర\u{c4d}రక\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}",
                        ),
                        ("th", "เขตม\u{e38}ฮ\u{e31}รร\u{e47}อก"),
                        ("tr", "Muharraq Yönetimi"),
                        ("uk", "Муніципалітет Мухаррак"),
                        ("ur", "محافظہ محرق"),
                        ("vi", "Tỉnh Muharraq"),
                        ("zh", "穆哈拉格省"),
                    ]),
                    unofficial_name_list: ["Al Muharraq"].to_vec(),
                },
            ),
            (
                "17",
                Subdivision {
                    name: "Ash Shamaliyah",
                    country_alpha2: Alpha2::BH,
                    code: "17",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(26.1551914),
                        longitude: Some(50.4825173),
                        max_latitude: Some(26.235592),
                        min_latitude: Some(26.04768),
                        max_longitude: Some(50.5664624),
                        min_longitude: Some(50.3788254),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Governorate,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "شمالية"),
                        ("bg", "Северна провинция"),
                        ("ca", "Governació del Nord"),
                        ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄘𑄢\u{11133}𑄚\u{11134}"),
                        ("ceb", "Northern Governorate"),
                        ("en", "Northern"),
                        ("es", "Norte"),
                        ("eu", "Iparraldeko eskualdea"),
                        ("fa", "شمالیه"),
                        ("fr", "Gouvernorat septentrional"),
                        ("hu", "Északi kormányzóság"),
                        ("id", "Kegubernuran Utara"),
                        ("it", "Governatorato Settentrionale"),
                        ("ja", "北部県 (バーレーン)"),
                        ("ko", "북부 주"),
                        ("lt", "Šiaurės muchafaza"),
                        ("nb", "Nordlige guvernement"),
                        ("nl", "Noord"),
                        ("no", "Nordlige guvernement"),
                        ("pl", "Al-Muhafazat asz-Szimalijja"),
                        ("ps", "محافظه شماليه"),
                        ("pt", "Província do Norte"),
                        ("ru", "Северная мухафаза"),
                        ("sv", "Northern Governorate"),
                        ("tr", "Kuzey Valiliği"),
                        ("ur", "محافظہ شمالیہ"),
                        ("zh", "北方省"),
                    ]),
                    unofficial_name_list: [
                        "Northern",
                        "al-Mintaqa ash Shamaliyah",
                        "ash Shamaliyah",
                    ]
                    .to_vec(),
                },
            ),
        ])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "bh")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BH,
        alpha3: Alpha3::BHR,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 973,
        currency_code: CurrencyCode::BHD,
        gec: Some(GEC::BA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::BRN),
        iso_long_name: "The Kingdom of Bahrain",
        iso_short_name: "Bahrain",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Bahraini"),
        number: "048",
        postal_code: true,
        postal_code_format: Some("(?:\\d|1[0-2])\\d{2}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "BH",
        unofficial_name_list: ["Bahrain", "البحرين", "Bahreïn", "Bahrein", "バーレーン"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Bahrain"),
            ("af", "Bahrein"),
            ("ak", "Bahrain"),
            ("am", "ባህሳን"),
            ("an", "Bahrein"),
            ("ar", "البحرين"),
            ("as", "ব\u{9be}হৰেইন"),
            ("ay", "Bahrain"),
            ("az", "Bahreyn"),
            ("ba", "Bahrain"),
            ("be", "Бахрэйн"),
            ("bg", "Бахрейн"),
            ("bi", "Bahrain"),
            ("bn", "ব\u{9be}হরেইন"),
            ("bn_IN", "ব\u{9be}হরেইন"),
            ("br", "Bahrein"),
            ("bs", "Bahrein"),
            ("ca", "Bahrain"),
            ("ce", "Бахрейн"),
            ("ch", "Bahrain"),
            ("cs", "Bahrajn"),
            ("cv", "Бахрейн"),
            ("cy", "Bahrain"),
            ("da", "Bahrain"),
            ("de", "Bahrain"),
            ("dv", "ބ\u{7a6}ޙ\u{7b0}ރ\u{7a6}އ\u{7a8}ނ\u{7b0}"),
            ("dz", "བ\u{f71}་ར\u{f7a}ན།"),
            ("ee", "Bahrain"),
            ("el", "Μπαχρέιν"),
            ("en", "Bahrain"),
            ("eo", "Barejno"),
            ("es", "Baréin"),
            ("et", "Bahrein"),
            ("eu", "Bahrain"),
            ("fa", "بحرین"),
            ("ff", "Bahrain"),
            ("fi", "Bahrain"),
            ("fo", "Bahrain"),
            ("fr", "Bahreïn"),
            ("fy", "De Barein"),
            ("ga", "Bairéin"),
            ("gl", "Bahrein"),
            ("gn", "Bahrain"),
            ("gu", "બહ\u{ac7}રિન"),
            ("gv", "Bahrain"),
            ("ha", "Baharain"),
            ("he", "בחריין"),
            ("hi", "बहरीन"),
            ("hr", "Bahrein"),
            ("ht", "Barayn"),
            ("hu", "Bahrein"),
            ("hy", "Բահրեյն"),
            ("ia", "Bahrein"),
            ("id", "Bahrain"),
            ("io", "Bahrain"),
            ("is", "Barein"),
            ("it", "Bahrein"),
            ("iu", "Bahrain"),
            ("ja", "バーレーン"),
            ("ka", "ბაჰრეინი"),
            ("ki", "Bahrain"),
            ("kk", "Бахрейн"),
            ("kl", "Bahrain"),
            ("km", "បារ\u{17c9}ែន"),
            ("kn", "ಬಹರೈನ\u{ccd}"),
            ("ko", "바레인"),
            ("ku", "Bahrayîn"),
            ("kv", "Бахрейн"),
            ("kw", "Bahreyn"),
            ("ky", "Бахрейн"),
            ("lo", "Bahrain"),
            ("lt", "Bahreinas"),
            ("lv", "Bahreina"),
            ("mi", "Bahrain"),
            ("mk", "Бахреин"),
            ("ml", "ബഹറൈന\u{d4d}\u{200d}"),
            ("mn", "Бахрейн"),
            ("mr", "बहारिन"),
            ("ms", "Bahrain"),
            ("mt", "Baħrejn"),
            (
                "my",
                "ဘာရ\u{102d}န\u{103a}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Bahrain"),
            ("nb", "Bahrain"),
            ("ne", "बहराइन"),
            ("nl", "Bahrein"),
            ("nn", "Bahrain"),
            ("nv", "Bahrain"),
            ("oc", "Barein"),
            ("or", "ବ\u{b3e}ହ\u{b3e}ରୀନ"),
            ("pa", "ਬਹਿਰਾਨ"),
            ("pi", "बहर\u{948}न"),
            ("pl", "Bahrajn"),
            ("ps", "بحرین"),
            ("pt", "Barém"),
            ("pt_BR", "Barein"),
            ("ro", "Bahrein"),
            ("ru", "Бахрейн"),
            ("rw", "Bahirayini"),
            ("sc", "Bahrein"),
            ("sd", "بحرين"),
            ("si", "බහරේන\u{dca}"),
            ("sk", "Bahrajn"),
            ("sl", "Bahrajn"),
            ("so", "Baxrayn"),
            ("sq", "Bahrein"),
            ("sr", "Бахреин"),
            ("sv", "Bahrain"),
            ("sw", "Bahrain"),
            ("ta", "பஹ\u{bcd}ரெயின\u{bcd}"),
            ("te", "బహర\u{c3f}న\u{c4d}"),
            ("tg", "Баҳрайн"),
            ("th", "บาห\u{e4c}เรน"),
            ("ti", "ባህሬን"),
            ("tk", "Bahreýn"),
            ("tl", "Bahrain"),
            ("tr", "Bahreyn"),
            ("tt", "Баһрейн"),
            ("ug", "بەھرەين"),
            ("uk", "Бахрейн"),
            ("ur", "بحرین"),
            ("uz", "Bahrayn"),
            ("ve", "Bahrain"),
            ("vi", "Ba-rainh"),
            ("wa", "Bareyn"),
            ("wo", "Bahrayin"),
            ("xh", "Bahrain"),
            ("yo", "Báháráìnì"),
            ("zh_CN", "巴林"),
            ("zh_HK", "巴林"),
            ("zh_TW", "巴林"),
            ("zu", "Bahrain"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
