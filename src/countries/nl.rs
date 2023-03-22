// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of the Netherlands

#[cfg(all(feature = "nl", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::NL;
    pub const ALPHA3: Alpha3 = Alpha3::NLD;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 31;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::NL);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("NED");
    pub const ISO_SHORT_NAME: &str = "Netherlands";
    pub const ISO_LONG_NAME: &str = "The Kingdom of the Netherlands";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["nl"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["nl"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Dutch");
    pub const NUMBER: &str = "528";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4} ?[A-Z]{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "NL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Netherlands",
        "The Netherlands",
        "Niederlande",
        "Pays-Bas",
        "Países Bajos",
        "オランダ",
        "Nederland",
        "Нидерландия",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Netherlands"),
        ("af", "Nederland"),
        ("ak", "Netherlands"),
        ("am", "ኔፈሴሒን፥"),
        ("an", "Netherlands"),
        ("ar", "هولندا"),
        ("as", "নেড\u{9be}ৰলেণ\u{9cd}ড"),
        ("ay", "Netherlands"),
        ("az", "Hollandiya"),
        ("ba", "Netherlands"),
        ("be", "Нідэрланды"),
        ("bg", "Холандия"),
        ("bi", "Netherlands"),
        ("bn", "নেদ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "নেদ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Izelvroioù"),
        ("bs", "Nizozemska"),
        ("ca", "Països Baixos"),
        ("ce", "Нидерландаш"),
        ("ch", "Netherlands"),
        ("cs", "Nizozemsko"),
        ("cv", "Нидерландаш"),
        ("cy", "Yr Iseldiroedd"),
        ("da", "Holland"),
        ("de", "Niederlande"),
        ("dv", "ނ\u{7ac}ދ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
        ("dz", "ན\u{f7a}་དར་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།"),
        ("ee", "Netherlands"),
        ("el", "Ολλανδία"),
        ("en", "Netherlands"),
        ("eo", "Nederlando"),
        ("es", "Países Bajos"),
        ("et", "Holland"),
        ("eu", "Herbehereak"),
        ("fa", "هلند"),
        ("ff", "Holannda"),
        ("fi", "Alankomaat"),
        ("fo", "Niðurlond"),
        ("fr", "Pays-Bas"),
        ("fy", "Nederlân"),
        ("ga", "An Ísiltír"),
        ("gl", "Países Baixos"),
        ("gn", "Netherlands"),
        ("gu", "ન\u{ac7}ધરલ\u{ac7}ન\u{acd}ડ\u{acd}સ"),
        ("gv", "Yn Çheer Injil"),
        ("ha", "Netherlands"),
        ("he", "הולנד"),
        ("hi", "नीदरल\u{948}ण\u{94d}ड"),
        ("hr", "Nizozemska"),
        ("ht", "Peyiba"),
        ("hu", "Hollandia"),
        ("hy", "Նիդերլանդեր"),
        ("ia", "Pais Basse"),
        ("id", "Belanda"),
        ("io", "Nederlando"),
        ("is", "Holland"),
        ("it", "Paesi Bassi"),
        ("iu", "Netherlands"),
        ("ja", "オランダ"),
        ("ka", "ნიდერლანდები"),
        ("ki", "Netherlands"),
        ("kk", "Нидерланды"),
        ("kl", "Netherlands"),
        ("km", "\u{200b}ហ\u{17bc}ល\u{17d2}លង\u{17cb}"),
        ("kn", "ನ\u{cc6}ದರ\u{ccd}\u{200c}ಲ\u{ccd}ಯಾಂಡ\u{ccd}ಸ\u{ccd}"),
        ("ko", "네덜란드"),
        ("ku", "Holanda"),
        ("kv", "Нидерландъяс"),
        ("kw", "Iseldiryow"),
        ("ky", "Нидерландтар"),
        ("lo", "ປະເທດໂຮນລ\u{eb1}ງ"),
        ("lt", "Nyderlandai"),
        ("lv", "Nīderlande"),
        ("mi", "Hōrana"),
        ("mk", "Холандија"),
        ("ml", "നെതര\u{d4d}\u{200d}ലന\u{d4d}\u{200d}ഡ\u{d4d}സ\u{d4d}"),
        ("mn", "Нидерланд"),
        ("mr", "न\u{947}दरल\u{945}\u{902}डस\u{94d}"),
        ("ms", "Belanda"),
        ("mt", "Netherlands"),
        (
            "my",
            "နယ\u{103a}သာလန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Eben Eyong"),
        ("nb", "Nederland"),
        ("ne", "न\u{947}दरल\u{94d}याण\u{94d}ड"),
        ("nl", "Nederland"),
        ("nn", "Nederland"),
        ("nv", "Kéyah Wóyahgo Siʼánígíí"),
        ("oc", "Païses Basses"),
        ("or", "ନେଦରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਨੀ\u{a02}ਦਰਲ\u{a48}\u{a02}ਡ"),
        ("pi", "न\u{947}दरल\u{948}\u{902}ड\u{94d}स"),
        ("pl", "Holandia"),
        ("ps", "هالېنډ"),
        ("pt", "Países Baixos"),
        ("pt_BR", "Países Baixos"),
        ("ro", "Olanda"),
        ("ru", "Нидерланды"),
        ("rw", "Nederilande"),
        ("sc", "Paisos Bassos"),
        ("sd", "Netherlands"),
        ("si", "නෙදර\u{dca}ලන\u{dca}තය"),
        ("sk", "Holandsko"),
        ("sl", "Nizozemska"),
        ("so", "Netherlands"),
        ("sq", "Holandë"),
        ("sr", "Холандија"),
        ("sv", "Nederländerna"),
        ("sw", "Uholanzi"),
        ("ta", "நெதர\u{bcd}ல\u{bbe}ந\u{bcd}து"),
        ("te", "న\u{c47}దర\u{c4d}ల\u{c3e}ండ\u{c4d}స\u{c4d}"),
        ("tg", "Нидерландия"),
        ("th", "เนเธอร\u{e4c}แลนด\u{e4c}"),
        ("ti", "ኔዘርላንድ"),
        ("tk", "Niderland"),
        ("tl", "Netherlands"),
        ("tr", "Hollanda"),
        ("tt", "Нидерландлар"),
        ("ug", "گوللاندىيە"),
        ("uk", "Нідерланди"),
        ("ur", "نیدرلینڈز"),
        ("uz", "Niderlandlar"),
        ("ve", "Netherlands"),
        ("vi", "Hoà Lan"),
        ("wa", "Bas Payis"),
        ("wo", "Olaand"),
        ("xh", "Netherlands"),
        ("yo", "Nẹ\u{301}dálándì"),
        ("zh_CN", "荷兰"),
        ("zh_HK", "荷蘭"),
        ("zh_TW", "荷蘭"),
        ("zu", "I-Netherlands"),
    ];
    #[cfg(all(feature = "nl", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 52.132633;
        pub const LONGITUDE: f64 = 5.291265999999999;
        pub const MAX_LATITUDE: f64 = 53.6316;
        pub const MAX_LONGITUDE: f64 = 7.227510199999999;
        pub const MIN_LATITUDE: f64 = 50.75038379999999;
        pub const MIN_LONGITUDE: f64 = 3.3316001;
        pub const NORTHEAST_LATITUDE: f64 = 53.6316;
        pub const NORTHEAST_LONGITUDE: f64 = 7.227510199999999;
        pub const SOUTHWEST_LATITUDE: f64 = 50.75038379999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 3.3316001;
    }
}
#[cfg(all(feature = "nl", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 52.132633,
            longitude: 5.291265999999999,
            max_latitude: 53.6316,
            max_longitude: 7.227510199999999,
            min_latitude: 50.75038379999999,
            min_longitude: 3.3316001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 53.6316,
                    longitude: 7.227510199999999,
                },
                southwest: CountryGeoBound {
                    latitude: 50.75038379999999,
                    longitude: 3.3316001,
                },
            },
        }
    }
}

#[cfg(all(feature = "nl", feature = "subdivisions"))]
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
                    "AW",
                    Subdivision{
                        name: "AW",
                        country_alpha2: Alpha2::NL,
                        code: "AW",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄢\u{1112a}𑄝"), ("en", "Aruba")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BQ1",
                    Subdivision{
                        name: "BQ1",
                        country_alpha2: Alpha2::NL,
                        code: "BQ1",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bonaire"), ("ar", "بونير"), ("az", "Boneyr adası"), ("be", "Востраў Банайрэ"), ("bg", "Бонер"), ("bn", "বোন\u{9be}ইর"), ("ca", "Bonaire"), ("ccp", "𑄝\u{11127}𑄚\u{1112d}𑄠𑄢\u{11134}"), ("ceb", "Bonaire (pulo ug munisipyo espesyal)"), ("cs", "Bonaire"), ("cy", "Bonaire"), ("da", "Bonaire"), ("de", "Bonaire"), ("el", "Μποναίρ"), ("en", "Bonaire"), ("es", "Bonaire"), ("et", "Bonaire"), ("eu", "Bonaire"), ("fa", "بونیر"), ("fi", "Bonaire"), ("fr", "Bonaire"), ("gl", "Bonaire"), ("gu", "બોનાર\u{ac7}"), ("he", "בונייר"), ("hi", "बोन\u{947}य\u{947}र"), ("hr", "Bonaire"), ("hu", "Bonaire"), ("hy", "Բոնեյրե"), ("id", "Bonaire"), ("it", "Bonaire"), ("ja", "ボネール島"), ("ka", "ბონეირი"), ("kk", "Бонэйр"), ("kn", "ಬೊನೈರ\u{ccd}"), ("ko", "보네르 섬"), ("lt", "Boneras"), ("lv", "Bonaire"), ("mk", "Бонер"), ("mr", "बोनर\u{947}"), ("ms", "Bonaire"), ("nb", "Bonaire"), ("nl", "Bonaire"), ("no", "Bonaire"), ("pa", "ਬ\u{a4b}ਨ\u{a47}ਅਰ"), ("pl", "Bonaire"), ("pt", "Bonaire"), ("ro", "Bonaire"), ("ru", "Бонэйр"), ("si", "බොනය\u{dd2}රේ"), ("sk", "Bonaire"), ("sr", "Бонер"), ("sr_Latn", "Boner"), ("sv", "Bonaire"), ("sw", "Bonaire"), ("ta", "பொனெய\u{bcd}ர\u{bcd}"), ("te", "బ\u{c4b}న\u{c46}య\u{c3f}ర\u{c4d}"), ("th", "โบแนเรอ"), ("tr", "Bonaire"), ("uk", "Бонайре"), ("ur", "بونایر"), ("vi", "Bonaire"), ("yo", "Bonaire"), ("yo_BJ", "Bonaire"), ("yue", "博奈爾島"), ("yue_Hans", "博奈尔岛"), ("zh", "波内赫")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BQ2",
                    Subdivision{
                        name: "BQ2",
                        country_alpha2: Alpha2::NL,
                        code: "BQ2",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saba"), ("ar", "سابا"), ("az", "Saba"), ("be", "Востраў Саба"), ("bg", "Саба"), ("bn", "স\u{9be}ব\u{9be}"), ("bs", "Saba"), ("ca", "Illa de Saba"), ("ccp", "𑄥𑄝"), ("ceb", "Saba Island"), ("cs", "Saba"), ("da", "Saba"), ("de", "Saba"), ("el", "Σάμπα"), ("en", "Saba"), ("es", "Saba"), ("et", "Saba saar"), ("eu", "Saba"), ("fa", "سیبا"), ("fi", "Saba"), ("fr", "Saba"), ("gl", "Saba"), ("he", "סאבא"), ("hr", "Saba"), ("hu", "Saba"), ("hy", "Սաբա կղզի"), ("id", "Saba"), ("it", "Saba"), ("ja", "サバ島"), ("ka", "საბა"), ("kk", "Саба"), ("ko", "사바 섬"), ("lt", "Saba"), ("lv", "Saba"), ("mk", "Саба"), ("nb", "Saba"), ("nl", "Saba"), ("no", "Saba"), ("pa", "ਸਾਬਾ"), ("pl", "Saba"), ("pt", "Saba"), ("ro", "Saba"), ("ru", "Саба"), ("sr", "Саба"), ("sr_Latn", "Saba"), ("sv", "Saba"), ("sw", "Saba"), ("ta", "சேப\u{bbe}"), ("th", "ซาบา"), ("tr", "Saba"), ("uk", "Саба"), ("ur", "صبا"), ("vi", "Saba"), ("yue", "薩巴島"), ("yue_Hans", "萨巴岛"), ("zh", "薩巴")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BQ3",
                    Subdivision{
                        name: "BQ3",
                        country_alpha2: Alpha2::NL,
                        code: "BQ3",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sint Eustatius"), ("ar", "سينت أوستاتيوس"), ("az", "Sint-Estatius"), ("be", "Востраў Сінт-Эстаціус"), ("bg", "Свети Евстатиус"), ("bn", "সিন\u{9cd}ট এউস\u{9cd}ত\u{9be}তিউস"), ("ca", "Sint Eustatius"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄃\u{11128}𑄅\u{1112a}𑄌\u{11134}𑄑𑄑\u{11128}𑄠𑄌\u{11134}"), ("ceb", "Sint Eustatius"), ("cs", "Svatý Eustach"), ("da", "Sint Eustatius"), ("de", "Sint Eustatius"), ("el", "Άγιος Ευστάθιος"), ("en", "Sint Eustatius"), ("es", "San Eustaquio"), ("et", "Sint-Eustatius"), ("eu", "Sint Eustatius"), ("fa", "سینت یوستیشس"), ("fi", "Sint Eustatius"), ("fr", "Saint-Eustache"), ("gl", "San Eustaquio"), ("gu", "સિન\u{acd}ટ ઓસ\u{acd}ટ\u{ac7}ટિયસ"), ("he", "סנט אוסטטיוס"), ("hi", "सि\u{902}ट य\u{942}स\u{94d}ट\u{947}शियस"), ("hr", "Sveti Eustasius"), ("hu", "Sint Eustatius"), ("hy", "Սինտ Էվստատիուս"), ("id", "Sint Eustatius"), ("it", "Sint Eustatius"), ("ja", "シント・ユースタティウス島"), ("ka", "სინტ-ესტატიუსი"), ("kk", "Синт-Эстатиус"), ("kn", "ಸ\u{cbf}ಂಟ\u{ccd} ಯ\u{cc2}ಸ\u{ccd}ಟಾಟ\u{cbf}ಯಸ\u{ccd}"), ("ko", "신트외스타티위스 섬"), ("lt", "Sint Eustatijus"), ("lv", "Sintēstatiusa"), ("mk", "Свети Евстахиј"), ("mr", "सि\u{902}ट उस\u{94d}ताशिअस"), ("ms", "Sint Eustatius"), ("nb", "Sint Eustatius"), ("nl", "Sint Eustatius"), ("no", "Sint Eustatius"), ("pa", "ਸਿ\u{a70}ਟ ਯ\u{a42}ਸਟ\u{a47}ਸ\u{a3c}ਸ"), ("pl", "Sint Eustatius"), ("pt", "Santo Eustáquio"), ("ro", "Sint Eustatius"), ("ru", "Синт-Эстатиус"), ("si", "ස\u{dd2}න\u{dca}ට\u{dca} ඉය\u{dd4}ස\u{dca}ටේට\u{dd2}යස\u{dca}"), ("sr", "Свети Еустахије"), ("sr_Latn", "Sveti Eustahije"), ("sv", "Sint Eustatius"), ("sw", "Sint Eustatius"), ("ta", "சின\u{bcd}டு யுசுட\u{bbe}சியசு"), ("te", "స\u{c3f}ంట\u{c4d} యూస\u{c4d}ట\u{c47}ట\u{c3f}యస\u{c4d}"), ("th", "ซ\u{e34}นต\u{e4c}เอ\u{e34}สตาซ\u{e35}ย\u{e36}ส"), ("tr", "Sint Eustatius"), ("uk", "Сінт-Естатіус"), ("ur", "سینٹ ایوسٹائیس"), ("vi", "Sint Eustatius"), ("yue", "聖猶士坦島"), ("yue_Hans", "圣犹士坦岛"), ("zh", "圣尤斯特歇斯")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CW",
                    Subdivision{
                        name: "CW",
                        country_alpha2: Alpha2::NL,
                        code: "CW",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{11128}𑄅\u{1112a}𑄢𑄇𑄃\u{1112e}"), ("en", "Curaçao")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DR",
                    Subdivision{
                        name: "DR",
                        country_alpha2: Alpha2::NL,
                        code: "DR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.9476012), longitude: Some(6.623058599999999), max_latitude: Some(53.2037415), min_latitude: Some(52.6121857), max_longitude: Some(7.0929356), min_longitude: Some(6.1198503)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Drenthe"), ("ar", "درينته"), ("be", "Дрэнтэ"), ("bg", "Дренте"), ("bn", "ড\u{9cd}রেন\u{9cd}তে"), ("ca", "Drenthe"), ("ccp", "𑄓\u{11133}𑄢𑄬𑄚\u{11134}𑄗𑄬"), ("ceb", "Provincie Drenthe"), ("cs", "Drenthe"), ("cy", "Drenthe"), ("da", "Drenthe"), ("de", "Provinz Drente"), ("el", "Ντρέντε"), ("en", "Drenthe"), ("es", "Drente"), ("et", "Drenthe provints"), ("eu", "Drenthe"), ("fa", "درنته"), ("fi", "Drenthe"), ("fr", "Drenthe"), ("ga", "Drenthe"), ("gl", "Drenthe"), ("gu", "ડ\u{acd}ર\u{ac7}ન\u{acd}થ\u{ac7}"), ("he", "דרנתה"), ("hi", "ड\u{94d}र\u{947}\u{902}ठ\u{947}"), ("hu", "Drenthe"), ("hy", "Դրենթե"), ("id", "Drenthe"), ("is", "Drenthe"), ("it", "Drenthe"), ("ja", "ドレンテ州"), ("jv", "Drenthe"), ("ka", "დრენთე"), ("kn", "ಡ\u{ccd}ರ\u{cc6}ಂಟ\u{cc6}"), ("ko", "드렌터 주"), ("lt", "Drentė"), ("lv", "Drente"), ("mk", "Дренте"), ("mr", "द\u{94d}र\u{947}\u{902}थ"), ("ms", "Drenthe"), ("nb", "Drenthe"), ("ne", "ड\u{94d}र\u{947}न\u{94d}थ\u{947}"), ("nl", "Drenthe"), ("no", "Drenthe"), ("pl", "Drenthe"), ("pt", "Drente"), ("ro", "Drenthe"), ("ru", "Дренте"), ("si", "ඩ\u{dca}රෙන\u{dca}තේ"), ("sk", "Drenthe"), ("so", "Drenthe"), ("sq", "Drenthe"), ("sr", "Дренте"), ("sr_Latn", "Drente"), ("sv", "Drenthe"), ("sw", "Drenthe"), ("ta", "டிரென\u{bcd}த\u{bcd}"), ("te", "డ\u{c4d}ర\u{c46}ంత\u{c46}"), ("th", "จ\u{e31}งหว\u{e31}ดเดรนเทอ"), ("tr", "Drenthe"), ("uk", "Дренте"), ("ur", "درینتے"), ("vi", "Drenthe"), ("yue", "德倫特"), ("yue_Hans", "德伦特"), ("zh", "德伦特省")]),
                        unofficial_name_list: ["Drenthe"].to_vec(),
                    }
                ),
                (
                    "FL",
                    Subdivision{
                        name: "FL",
                        country_alpha2: Alpha2::NL,
                        code: "FL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5279781), longitude: Some(5.595350799999999), max_latitude: Some(52.8440468), min_latitude: Some(52.2526254), max_longitude: Some(6.011667099999999), min_longitude: Some(5.1221244)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Flevoland"), ("ar", "فليفولاند"), ("be", "Флеваланд"), ("bg", "Флеволанд"), ("bn", "ফ\u{9cd}রিলে ফ\u{9cd}লেভোল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Flevoland"), ("ccp", "𑄜\u{11133}𑄢𑄬𑄞\u{1112e}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Flevoland"), ("cs", "Flevoland"), ("cy", "Flevoland"), ("da", "Flevoland"), ("de", "Provinz Flevoland"), ("el", "Φλέβολαντ"), ("en", "Flevoland"), ("es", "Flevoland"), ("et", "Flevoland"), ("eu", "Flevoland"), ("fa", "فلیوولاند"), ("fi", "Flevoland"), ("fr", "Flevoland"), ("ga", "Flevoland"), ("gl", "Flevoland"), ("gu", "ફ\u{acd}લ\u{ac7}વોલ\u{ac7}ન\u{acd}ડ"), ("he", "פלבולנד"), ("hi", "फ\u{94d}ल\u{947}वोल\u{948}\u{902}ड"), ("hu", "Flevoland"), ("hy", "Ֆլևոլանդ"), ("id", "Flevoland"), ("is", "Flevoland"), ("it", "Flevoland"), ("ja", "フレヴォラント州"), ("jv", "Flevoland"), ("ka", "ფლევოლანდი"), ("kn", "ಫ\u{ccd}ಲೀವೋಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "플레볼란트 주"), ("lt", "Flevolandas"), ("lv", "Flevolande"), ("mk", "Флеволанд"), ("mr", "फ\u{94d}ल\u{947}व\u{94d}होला\u{902}ड"), ("ms", "Flevoland"), ("nb", "Flevoland"), ("ne", "फ\u{94d}ल\u{947}भोल\u{94d}यान\u{94d}ड"), ("nl", "Flevoland"), ("no", "Flevoland"), ("pl", "Flevoland"), ("pt", "Flevolândia"), ("ro", "Flevoland"), ("ru", "Флеволанд"), ("si", "ෆ\u{dca}ලෙවෝලන\u{dca}ඩ\u{dca}"), ("sk", "Flevoland"), ("so", "Flevoland"), ("sq", "Flevoland"), ("sr", "Флеволанд"), ("sr_Latn", "Flevoland"), ("sv", "Flevoland"), ("sw", "Flevoland"), ("ta", "ஃபிலொவோல\u{bbe}ந\u{bcd}து"), ("te", "ఫ\u{c4d}ర\u{c46}వ\u{c4b}ల\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฟลโฟล\u{e31}นด\u{e4c}"), ("tr", "Flevoland"), ("uk", "Флеволанд"), ("ur", "فلیولانت"), ("vi", "Flevoland"), ("yue", "傅利禾蘭"), ("yue_Hans", "傅利禾兰"), ("zh", "弗莱福兰省")]),
                        unofficial_name_list: ["Flevoland"].to_vec(),
                    }
                ),
                (
                    "FR",
                    Subdivision{
                        name: "FR",
                        country_alpha2: Alpha2::NL,
                        code: "FR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1641642), longitude: Some(5.7817542), max_latitude: Some(53.5145975), min_latitude: Some(52.8006928), max_longitude: Some(6.4274762), min_longitude: Some(4.8455655)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Friesland"), ("ar", "فرايزلاند"), ("be", "Фрысландыя"), ("bg", "Фризия"), ("bn", "ফ\u{9cd}র\u{9be}ইসল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Província de Frísia"), ("ccp", "𑄜\u{11133}𑄢\u{1112d}𑄠𑄬𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Friesland"), ("cs", "Frísko"), ("cy", "Fryslân"), ("da", "Frisland"), ("de", "Provinz Friesland"), ("el", "Φρίσλαντ"), ("en", "Friesland"), ("es", "Frisia"), ("et", "Friisimaa provints"), ("eu", "Frisia"), ("fa", "فریسلاند"), ("fi", "Friisinmaa"), ("fr", "Frise"), ("ga", "Friesland"), ("gl", "Frisia - Fryslân"), ("gu", "ફ\u{acd}રાઈસલ\u{ac7}ન\u{acd}ડ"), ("he", "פריסלנד"), ("hi", "फ\u{94d}राइजल\u{948}\u{902}ड"), ("hu", "Frízföld"), ("hy", "Ֆրիսլանդիա"), ("id", "Friesland"), ("is", "Frísland"), ("it", "Frisia"), ("ja", "フリースラント州"), ("jv", "Friesland"), ("ka", "ფრისლანდია"), ("kn", "ಫ\u{ccd}ರೈಸ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "프리슬란트 주"), ("lt", "Fryzija"), ("lv", "Frīzlande"), ("mk", "Фризија"), ("mr", "फ\u{94d}रीसल\u{902}ड"), ("ms", "Friesland"), ("nb", "Friesland"), ("ne", "फ\u{94d}राइजल\u{94d}यान\u{94d}ड"), ("nl", "Friesland"), ("no", "Friesland"), ("pl", "Fryzja"), ("pt", "Frísia"), ("ro", "Provincia Frizia"), ("ru", "Фрисландия"), ("si", "ෆ\u{dca}\u{200d}ර\u{dd3}ස\u{dca}ලන\u{dca}ත\u{dca}"), ("sk", "Frízsko"), ("so", "Friesland"), ("sq", "Friesland"), ("sr", "Фризија"), ("sr_Latn", "Frizija"), ("sv", "Friesland"), ("sw", "Friesland"), ("ta", "பிரைஸ\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "ఫ\u{c4d}ర\u{c48}స\u{c4d} ల\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟร\u{e35}สล\u{e31}นด\u{e4c}"), ("tr", "Frizya"), ("uk", "Фрисландія"), ("ur", "فریسلانت"), ("vi", "Friesland"), ("yue", "菲士蘭"), ("yue_Hans", "菲士兰"), ("zh", "弗里斯兰省")]),
                        unofficial_name_list: ["Friesland"].to_vec(),
                    }
                ),
                (
                    "GE",
                    Subdivision{
                        name: "GE",
                        country_alpha2: Alpha2::NL,
                        code: "GE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.045155), longitude: Some(5.871823399999999), max_latitude: Some(52.5220087), min_latitude: Some(51.7336076), max_longitude: Some(6.833041799999999), min_longitude: Some(4.9938809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gelderland"), ("ar", "خيلدرلند"), ("be", "правінцыя Гелдэрланд"), ("bg", "Гелдерланд"), ("bn", "গেল\u{9cd}ড\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Gelderland"), ("ccp", "𑄉𑄬𑄣\u{11134}𑄓𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Gelderland"), ("cs", "Gelderland"), ("cy", "Gelderland"), ("da", "Gelderland"), ("de", "Provinz Gelderland"), ("el", "Χέλντερλαντ"), ("en", "Gelderland"), ("es", "Güeldres"), ("et", "Gelderlandi provints"), ("eu", "Gelderland"), ("fa", "خلدرلاند"), ("fi", "Gelderland"), ("fr", "Gueldre"), ("ga", "Gelderland"), ("gl", "Gueldria - Gelderland"), ("gu", "ગ\u{ac7}લ\u{acd}ડરલ\u{ac7}ન\u{acd}ડ"), ("he", "חלדרלנד"), ("hi", "ग\u{947}ल\u{94d}डरल\u{948}\u{902}ड"), ("hr", "Gelderland"), ("hu", "Gelderland"), ("hy", "Խելդեռլանդ"), ("id", "Gelderland"), ("is", "Gelderland"), ("it", "Gheldria"), ("ja", "ヘルダーラント州"), ("jv", "Gelderland"), ("ka", "გელდერლანდი"), ("kn", "ಗ\u{cc6}ಲ\u{ccd}ಡರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "헬데를란트 주"), ("lt", "Gelderlandas"), ("lv", "Gelderlande"), ("mk", "Гелдерланд"), ("mn", "Гелдерланд"), ("mr", "ग\u{947}ल\u{94d}डरला\u{902}ड"), ("ms", "Gelderland"), ("nb", "Gelderland"), ("ne", "ग\u{947}ल\u{94d}डरल\u{94d}यान\u{94d}ड"), ("nl", "Gelderland"), ("no", "Gelderland"), ("pl", "Geldria"), ("pt", "Guéldria"), ("ro", "Gelderland"), ("ru", "Гелдерланд"), ("si", "ගෙලඩර\u{dca}ලන\u{dca}තය"), ("sk", "Gelderland"), ("so", "Gelderland"), ("sq", "Gelderland"), ("sr", "Хелдерланд"), ("sr_Latn", "Helderland"), ("sv", "Gelderland"), ("sw", "Gelderland"), ("ta", "கெல\u{bcd}டர\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "గ\u{c46}ల\u{c4d}డర\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกลเดอร\u{e4c}ล\u{e31}นด\u{e4c}"), ("tr", "Gelderland"), ("uk", "Гелдерланд"), ("ur", "خیلدرلنت"), ("vi", "Gelderland"), ("yue", "希德蘭"), ("yue_Hans", "希德兰"), ("zh", "海尔德兰省")]),
                        unofficial_name_list: ["Gueldre"].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::NL,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.2193835), longitude: Some(6.566501799999999), max_latitude: Some(53.26472589999999), min_latitude: Some(53.1786842), max_longitude: Some(6.6669044), min_longitude: Some(6.4625995)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Groningen"), ("ar", "مقاطعة خرونينغن"), ("az", "Xroninqen"), ("be", "Правінцыя Гронінген"), ("bg", "Гронинген"), ("bn", "গ\u{9cd}রোনিনজেন"), ("ca", "Província de Groningen"), ("ccp", "𑄉\u{11133}𑄢\u{1112e}𑄚\u{11128}𑄚\u{11134}𑄎𑄬𑄚\u{11134}"), ("ceb", "Provincie Groningen"), ("cs", "Groningen"), ("cy", "Groningen"), ("da", "Groningen"), ("de", "Provinz Groningen"), ("el", "Χρόνινγκεν"), ("en", "Groningen"), ("es", "Provincia de Groninga"), ("et", "Groningeni provints"), ("eu", "Groningen"), ("fa", "خرونینگن"), ("fi", "Groningen"), ("fr", "Groningue"), ("ga", "Groningen"), ("gl", "Provincia de Groninga - Provincie Groningen"), ("gu", "ગ\u{acd}રૉનિન\u{acd}જ\u{ac7}ન"), ("he", "חרונינגן"), ("hi", "ग\u{94d}रोनि\u{902}गन"), ("hu", "Groningen"), ("hy", "Խրոնինգեն"), ("id", "Groningen"), ("is", "Groningen"), ("it", "Groninga"), ("ja", "フローニンゲン州"), ("jv", "Provinsi Groningen"), ("ka", "გრონინგენის პროვინცია"), ("kn", "ಗ\u{ccd}ರೊನ\u{cbf}ನ\u{ccd}ಗ\u{cc6}ನ\u{ccd}"), ("ko", "흐로닝언 주"), ("lt", "Groningeno provincija"), ("lv", "Groningena"), ("mk", "Гронинген"), ("mr", "ग\u{94d}रोनि\u{902}गन (प\u{94d}रा\u{902}त)"), ("ms", "Groningen"), ("nb", "Groningen"), ("ne", "ग\u{94d}रोनिज\u{947}न"), ("nl", "Groningen"), ("no", "Groningen"), ("pl", "Groningen"), ("pt", "Groninga"), ("ro", "Groningen"), ("ru", "Гронинген"), ("si", "ග\u{dca}රෝන\u{dd2}න\u{dca}ජෙන\u{dca}"), ("sk", "Groningen"), ("so", "Groningen"), ("sq", "Groningen"), ("sr", "Гронинген"), ("sr_Latn", "Groningen"), ("sv", "Groningen"), ("sw", "Mkoa wa Groningen"), ("ta", "கிரோனிஞென\u{bcd}"), ("te", "గ\u{c4d}ర\u{c4b}న\u{c3f}ంగ\u{c46}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกรน\u{e34}งเง\u{e34}น"), ("tr", "Groningen"), ("uk", "Гронінген"), ("ur", "خرونیگین"), ("vi", "Groningen"), ("yue", "古羅寧亨"), ("yue_Hans", "古罗宁亨"), ("zh", "格罗宁根省")]),
                        unofficial_name_list: ["Groninga", "Groningue"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::NL,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4427238), longitude: Some(6.0608726), max_latitude: Some(51.778577), min_latitude: Some(50.75038379999999), max_longitude: Some(6.226801399999999), min_longitude: Some(5.5660666)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Limburg, Nederland"), ("ar", "ليمبورخ"), ("be", "Лімбург"), ("bg", "Лимбург"), ("bn", "লিম\u{9cd}ব\u{9be}র\u{9cd}গ"), ("ca", "Limburg"), ("ccp", "𑄣\u{11128}𑄟𑄝𑄢\u{11134}𑄉\u{11134}"), ("cs", "Limburg"), ("cy", "Limburg"), ("da", "Limburg (nederlandsk provins)"), ("de", "Provinz Limburg"), ("el", "Λιμβουργία"), ("en", "Limburg"), ("es", "Limburgo"), ("et", "Limburgi provints"), ("eu", "Limburg"), ("fa", "لیمبورخ"), ("fi", "Limburg"), ("fr", "Limbourg"), ("ga", "Limburg"), ("gl", "Limburgo - Limburg"), ("gu", "લિમ\u{acd}બર\u{acd}ગ"), ("he", "לימבורג"), ("hi", "लिम\u{94d}बर\u{94d}ग (नीदरल\u{948}\u{902}ड\u{94d}स)"), ("hr", "Limburg"), ("hu", "Limburg"), ("hy", "Լիմբուրխ"), ("id", "Limburg"), ("is", "Limburg"), ("it", "Limburgo"), ("ja", "リンブルフ州"), ("jv", "Limburg"), ("ka", "ლიმბურგის პროვინცია"), ("kn", "ಲ\u{cbf}ಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "림뷔르흐 주"), ("lt", "Limburgo provincija"), ("lv", "Limburga"), ("mk", "Лимбург"), ("mr", "लिमबर\u{94d}ग"), ("ms", "Limburg"), ("nb", "Limburg"), ("ne", "लिमबर\u{94d}ग"), ("nl", "Limburg"), ("no", "Limburg"), ("pl", "Limburgia"), ("pt", "Limburgo"), ("ro", "Limburg"), ("ru", "Лимбург"), ("si", "ල\u{dd2}ම\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Limbursko"), ("so", "Limburg"), ("sq", "Limburgu"), ("sr", "Лимбург"), ("sr_Latn", "Limburg"), ("sv", "Limburg"), ("sw", "Limburg, Uholanzi"), ("ta", "லிம\u{bcd}பெர\u{bcd}க\u{bcd}"), ("te", "ల\u{c3f}ంబర\u{c4d}గ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e34}มบ\u{e39}ร\u{e4c}ก"), ("tr", "Limburg, Hollanda"), ("uk", "Лімбург"), ("ur", "لمبرخ"), ("vi", "Limburg"), ("yue", "林堡"), ("yue_Hans", "林堡"), ("zh", "林堡省")]),
                        unofficial_name_list: ["Limburg"].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::NL,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4826537), longitude: Some(5.2321687), max_latitude: Some(51.8307142), min_latitude: Some(51.2209373), max_longitude: Some(6.047724), min_longitude: Some(4.190081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Brabant"), ("ar", "شمال برابنت"), ("az", "Şimali Brabant"), ("be", "Паўночны Брабант"), ("bg", "Северен Брабант"), ("bn", "উত\u{9cd}তর ব\u{9cd}র\u{9be}ব\u{9be}ন\u{9cd}ট"), ("ca", "Brabant Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄝\u{11133}𑄢𑄝𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Provincie Noord-Brabant"), ("cs", "Severní Brabantsko"), ("cy", "Noord-Brabant"), ("da", "Noord-Brabant"), ("de", "Provinz Nordbrabant"), ("el", "Βόρεια Βραβάντη"), ("en", "North Brabant"), ("es", "Brabante Septentrional"), ("et", "Põhja-Brabant"), ("eu", "Ipar Brabante"), ("fa", "برابانت شمالی"), ("fi", "Pohjois-Brabant"), ("fr", "Brabant-Septentrional"), ("ga", "Brabant Thuaidh"), ("gl", "Brabante do Norte - Noord Brabant"), ("gu", "નોર\u{acd}થ બ\u{acd}ર\u{ac7}બ\u{ac7}ન\u{acd}ટ"), ("he", "צפון בראבנט"), ("hi", "उत\u{94d}तरी ब\u{94d}रब\u{948}\u{902}ट"), ("hr", "Sjeverni Brabant"), ("hu", "Észak-Brabant"), ("hy", "Հյուսիսային Բրաբանդ"), ("id", "Brabant Utara"), ("is", "Norður-Brabant"), ("it", "Brabante Settentrionale"), ("ja", "北ブラバント州"), ("jv", "Brabant Lor"), ("ka", "ჩრდილოეთი ბრაბანტი"), ("kn", "ನಾರ\u{ccd}ತ\u{ccd} ಬ\u{ccd}ರಬಂಟ\u{ccd}"), ("ko", "노르트브라반트 주"), ("lt", "Šiaurės Brabantas"), ("lv", "Ziemeļbrabante"), ("mk", "Северен Брабант"), ("mn", "Умар Брабант"), ("mr", "न\u{942}र\u{94d}द-ब\u{94d}राबा\u{902}त"), ("ms", "Noord-Brabant"), ("nb", "Noord-Brabant"), ("nl", "Noord-Brabant"), ("no", "Noord-Brabant"), ("pl", "Brabancja Północna"), ("pt", "Brabante do Norte"), ("ro", "Brabantul de Nord"), ("ru", "Северный Брабант"), ("si", "උත\u{dd4}ර\u{dd4} බ\u{dca}\u{200d}රබන\u{dca}ට\u{dca}"), ("sk", "Severné Brabantsko"), ("so", "Waqooyiga Brabant"), ("sq", "Brabanta Veriore"), ("sr", "Северни Брабант"), ("sr_Latn", "Severni Brabant"), ("sv", "Noord-Brabant"), ("sw", "Noord-Brabant"), ("ta", "வடக\u{bcd}கு பிரப\u{bbe}ன\u{bcd}ட\u{bcd}"), ("te", "ఉత\u{c4d}తర బ\u{c4d}ర\u{c3e}బంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ทบราแบนต\u{e4c}"), ("tr", "Kuzey Brabant"), ("uk", "Північний Брабант"), ("ur", "شمالی برابانٹ"), ("vi", "Noord-Brabant"), ("yue", "北布拉班"), ("yue_Hans", "北布拉班"), ("zh", "北布拉班特省")]),
                        unofficial_name_list: ["Noord-Brabant"].to_vec(),
                    }
                ),
                (
                    "NH",
                    Subdivision{
                        name: "NH",
                        country_alpha2: Alpha2::NL,
                        code: "NH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5205869), longitude: Some(4.788474), max_latitude: Some(53.1833322), min_latitude: Some(52.1657716), max_longitude: Some(5.328279999999999), min_longitude: Some(4.4937415)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Holland"), ("ar", "شمال-هولندا"), ("az", "Şimali Hollandiya"), ("be", "Паўночная Галандыя"), ("bg", "Северна Холандия"), ("bn", "উত\u{9cd}তর হল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Holanda Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄦\u{1112e}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Noord-Holland"), ("cs", "Severní Holandsko"), ("cy", "Noord-Holland"), ("da", "Noord-Holland"), ("de", "Provinz Nordholland"), ("el", "Βόρεια Ολλανδία"), ("en", "North Holland"), ("es", "Holanda Septentrional"), ("et", "Põhja-Holland"), ("eu", "Ipar Holanda"), ("fa", "هلند شمالی"), ("fi", "Pohjois-Hollanti"), ("fr", "Hollande-Septentrionale"), ("ga", "An Ollainn Thuaidh"), ("gl", "Holanda Setentrional - Noord-Holland"), ("gu", "નોર\u{acd}થ હોલ\u{ac7}ન\u{acd}ડ"), ("he", "צפון הולנד"), ("hi", "उत\u{94d}तर हॉल\u{948}\u{902}ड"), ("hu", "Észak-Holland"), ("hy", "Հյուսիսային Հոլանդիա"), ("id", "Holland Utara"), ("is", "Norður-Holland"), ("it", "Olanda Settentrionale"), ("ja", "北ホラント州"), ("jv", "Holland Lor"), ("ka", "ჩრდილოეთი ჰოლანდია"), ("kn", "ಉತ\u{ccd}ತರ ಹಾಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "노르트홀란트 주"), ("lt", "Šiaurės Olandija"), ("lv", "Ziemeļholande"), ("mk", "Северна Холандија"), ("mn", "Умар Холланд"), ("mr", "न\u{942}र\u{94d}द-हॉल\u{902}ड"), ("ms", "Noord-Holland"), ("nb", "Noord-Holland"), ("ne", "उत\u{94d}तरी हल\u{94d}यान\u{94d}ड"), ("nl", "Noord-Holland"), ("no", "Noord-Holland"), ("pl", "Holandia Północna"), ("pt", "Holanda do Norte"), ("ro", "Olanda de Nord"), ("ru", "Северная Голландия"), ("si", "උත\u{dd4}ර\u{dd4} ඕලන\u{dca}දය"), ("sk", "Severný Holland"), ("sl", "Severna Holandija"), ("so", "Waqooyiga Holland"), ("sq", "Hollanda e Veriut"), ("sr", "Северна Холандија"), ("sr_Latn", "Severna Holandija"), ("sv", "Noord-Holland"), ("sw", "Noord-Holland"), ("ta", "வட ஹ\u{bbe}லந\u{bcd}து"), ("te", "ఉత\u{c4d}తర హ\u{c3e}ల\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ทฮอลแลนด\u{e4c}"), ("tr", "Kuzey Hollanda"), ("uk", "Північна Голландія"), ("ur", "شمالی ہولانت"), ("vi", "Noord-Holland"), ("yue", "北荷蘭"), ("yue_Hans", "北荷兰"), ("zh", "北荷兰省")]),
                        unofficial_name_list: ["Noord-Holland"].to_vec(),
                    }
                ),
                (
                    "OV",
                    Subdivision{
                        name: "OV",
                        country_alpha2: Alpha2::NL,
                        code: "OV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.4387814), longitude: Some(6.5016411), max_latitude: Some(52.85423489999999), min_latitude: Some(52.118023), max_longitude: Some(7.0727632), min_longitude: Some(5.7778252)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Overijssel"), ("ar", "أوفرايسل"), ("be", "Аверэйсел"), ("bg", "Оверейсел"), ("bn", "ওভেরিসসেল"), ("ca", "Overijssel"), ("ccp", "𑄃\u{1112e}𑄞𑄢\u{11128}𑄌\u{11134}𑄥𑄬𑄣\u{11134}"), ("ceb", "Provincie Overijssel"), ("cs", "Overijssel"), ("cy", "Overijssel"), ("da", "Overijssel"), ("de", "Provinz Overijssel"), ("el", "Οφεράισσελ"), ("en", "Overijssel"), ("es", "Overijssel"), ("et", "Overijsseli provints"), ("eu", "Overijssel"), ("fa", "افریسل"), ("fi", "Overijssel"), ("fr", "Overijssel"), ("ga", "Overijssel"), ("gl", "Overijssel"), ("gu", "ઓવરજ\u{acd}સ\u{ac7}લ"), ("he", "אוברייסל"), ("hi", "ओवरिस\u{94d}स\u{947}ल"), ("hu", "Overijssel"), ("hy", "Օվերեյսել"), ("id", "Overijssel"), ("is", "Overijssel"), ("it", "Overijssel"), ("ja", "オーファーアイセル州"), ("jv", "Overijssel"), ("ka", "ოვერეისელი"), ("kn", "ಓವರ\u{ccd}ಜ\u{cbf}ಸ\u{cc6}ಲ\u{ccd}"), ("ko", "오버레이설 주"), ("lt", "Overeiselis"), ("lv", "Overeisela"), ("mk", "Оверејсел"), ("mr", "ओव\u{94d}हराईजल"), ("ms", "Overijssel"), ("nb", "Overijssel"), ("ne", "ओभ\u{947}रिज\u{94d}स\u{94d}स\u{947}ल"), ("nl", "Overijssel"), ("no", "Overijssel"), ("pl", "Overijssel"), ("pt", "Overissel"), ("ro", "Overijssel"), ("ru", "Оверэйсел"), ("si", "ඔවෙර\u{dd2}ජෙස\u{dca}සෙල\u{dca}"), ("sk", "Overijssel"), ("so", "Overijssel"), ("sq", "Overijssel"), ("sr", "Оверејсел"), ("sr_Latn", "Overejsel"), ("sv", "Overijssel"), ("sw", "Overijssel"), ("ta", "ஓவர\u{bcd}ரிஜ\u{bcd}செல\u{bcd}"), ("te", "ఓవర\u{c3f}జ\u{c4d}స\u{c46}ల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอเฟอไรส\u{e4c}เซ\u{e34}ล"), ("tr", "Overijssel"), ("uk", "Оверейсел"), ("ur", "اوفریسل"), ("vi", "Overijssel"), ("yue", "上艾修"), ("yue_Hans", "上艾修"), ("zh", "上艾瑟尔省")]),
                        unofficial_name_list: ["Overijssel"].to_vec(),
                    }
                ),
                (
                    "SX",
                    Subdivision{
                        name: "SX",
                        country_alpha2: Alpha2::NL,
                        code: "SX",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{11128}𑄠𑄚\u{11134}𑄑\u{11134} 𑄟𑄢\u{11134}𑄑𑄬𑄚\u{11134}"), ("en", "Sint Maarten")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "UT",
                    Subdivision{
                        name: "UT",
                        country_alpha2: Alpha2::NL,
                        code: "UT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.09073739999999), longitude: Some(5.1214201), max_latitude: Some(52.1422624), min_latitude: Some(52.02611109999999), max_longitude: Some(5.1952072), min_longitude: Some(4.9697758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Utrecht"), ("ar", "مقاطعة أوترخت"), ("az", "Utrext vilayəti"), ("be", "правінцыя Утрэхт"), ("bg", "Утрехт"), ("bn", "উট\u{9cd}রেখট"), ("ca", "Província d’Utrecht"), ("ccp", "𑄅\u{1112a}𑄑\u{11133}𑄢𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Provincie Utrecht"), ("cs", "Utrecht"), ("cy", "Utrecht"), ("da", "Utrecht"), ("de", "Provinz Utrecht"), ("el", "Ουτρέχτη"), ("en", "Utrecht"), ("es", "Provincia de Utrecht"), ("et", "Utrechti provints"), ("eu", "Utrecht probintzia"), ("fa", "استان اوترخت"), ("fi", "Utrecht"), ("fr", "Utrecht"), ("ga", "Utrecht"), ("gl", "Provincia de Utrecht - Provincie Utrecht"), ("gu", "ઉટ\u{acd}ર\u{ac7}ક\u{acd}ટ"), ("he", "אוטרכט"), ("hi", "य\u{942}ट\u{94d}र\u{947}क\u{94d}ट"), ("hu", "Utrecht"), ("id", "Utrecht"), ("is", "Utrecht"), ("it", "Utrecht"), ("ja", "ユトレヒト州"), ("jv", "Provinsi Utrecht"), ("ka", "უტრეხტის პროვინცია"), ("kn", "ಉಟ\u{ccd}ರ\u{cc6}ಕ\u{ccd}ಟ\u{ccd}"), ("ko", "위트레흐트 주"), ("lt", "Utrechto provincija"), ("lv", "Utrehta"), ("mk", "Утрехт"), ("mr", "उट\u{94d}र\u{947}ख\u{94d}त"), ("ms", "Utrecht"), ("nb", "Utrecht"), ("ne", "उट\u{94d}र\u{947}च\u{94d}ट"), ("nl", "Utrecht"), ("no", "Utrecht"), ("pl", "Utrecht"), ("pt", "Utrecht"), ("ro", "Utrecht"), ("ru", "Утрехт"), ("si", "උට\u{dca}\u{200d}රෙච\u{dca}ට\u{dca}"), ("sk", "Utrecht"), ("sl", "Utrecht"), ("so", "Utrecht"), ("sq", "Utrecht"), ("sr", "Утрехт"), ("sr_Latn", "Utreht"), ("sv", "Utrecht"), ("sw", "Mkoa wa Utrecht"), ("ta", "யூடிரெக\u{bcd}ட\u{bcd}"), ("te", "యూట\u{c4d}ర\u{c46}క\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดย\u{e39}เทรกต\u{e4c}"), ("tr", "Utrecht"), ("uk", "Утрехт"), ("ur", "اوتریخت"), ("vi", "Utrecht"), ("yue", "宇德歷"), ("yue_Hans", "宇德历"), ("zh", "乌得勒支省")]),
                        unofficial_name_list: ["Utrecht"].to_vec(),
                    }
                ),
                (
                    "ZE",
                    Subdivision{
                        name: "ZE",
                        country_alpha2: Alpha2::NL,
                        code: "ZE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4940309), longitude: Some(3.8496815), max_latitude: Some(51.75842100000001), min_latitude: Some(51.2003318), max_longitude: Some(4.279691), min_longitude: Some(3.357962)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zeeland"), ("ar", "زيلند"), ("be", "Зеландыя"), ("bg", "Зеландия"), ("bn", "জিল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Zelanda"), ("ccp", "𑄎\u{11129}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Zeeland"), ("cs", "Zeeland"), ("cy", "Zeeland"), ("da", "Zeeland"), ("de", "Zeeland"), ("el", "Ζηλανδία"), ("en", "Zeeland"), ("es", "Zelanda"), ("et", "Zeelandi provints"), ("eu", "Zeelanda"), ("fa", "زیلاند"), ("fi", "Zeeland"), ("fr", "Zélande"), ("ga", "Zeeland"), ("gl", "Celandia - Zeeland"), ("gu", "ઝીલ\u{ac7}ન\u{acd}ડ"), ("he", "זיילנד"), ("hi", "ज\u{93c}ील\u{948}\u{902}ड"), ("hu", "Zeeland"), ("hy", "Զեյլանդ"), ("id", "Zeeland"), ("is", "Sjáland"), ("it", "Zelanda"), ("ja", "ゼーラント州"), ("ka", "ზელანდიის პროვინცია"), ("kn", "ಜೀಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "제일란트 주"), ("lt", "Zelandijos provincija"), ("lv", "Zēlande"), ("mk", "Зеланд"), ("mr", "झील\u{902}ड"), ("ms", "Zeeland"), ("nb", "Zeeland"), ("ne", "जिल\u{94d}यान\u{94d}ड"), ("nl", "Zeeland"), ("no", "Zeeland"), ("pl", "Zelandia"), ("pt", "Zelândia"), ("ro", "Zeelanda"), ("ru", "Зеландия"), ("si", "ස\u{dd3}ලන\u{dca}ඩ\u{dca}"), ("sk", "Zéland"), ("so", "Zeeland"), ("sq", "Zeeland"), ("sr", "Зеланд"), ("sr_Latn", "Zeland"), ("sv", "Zeeland"), ("sw", "Zeeland"), ("ta", "ச\u{bc0}ல\u{bbe}ந\u{bcd}து"), ("te", "జ\u{c40}ల\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเซล\u{e31}นด\u{e4c}"), ("tr", "Zelanda"), ("uk", "Зеландія"), ("ur", "زی لینڈ"), ("vi", "Zeeland"), ("yue", "西蘭"), ("yue_Hans", "西兰"), ("zh", "泽兰省")]),
                        unofficial_name_list: ["Zeeland"].to_vec(),
                    }
                ),
                (
                    "ZH",
                    Subdivision{
                        name: "ZH",
                        country_alpha2: Alpha2::NL,
                        code: "ZH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.0207975), longitude: Some(4.4937836), max_latitude: Some(52.3282742), min_latitude: Some(51.656067), max_longitude: Some(5.1492625), min_longitude: Some(3.8393205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Holland"), ("ar", "جنوب هولندا"), ("az", "Cənubi Hollandiya"), ("be", "Паўднёвая Галандыя"), ("bg", "Южна Холандия"), ("bn", "স\u{9be}উথ হল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Holanda Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄦\u{1112e}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Provincie Zuid-Holland"), ("cs", "Jižní Holandsko"), ("cy", "Zuid-Holland"), ("da", "Zuid-Holland"), ("de", "Provinz Südholland"), ("el", "Νότια Ολλανδία"), ("en", "South Holland"), ("es", "Holanda Meridional"), ("et", "Lõuna-Holland"), ("eu", "Hego Holanda"), ("fa", "هلند جنوبی"), ("fi", "Etelä-Hollanti"), ("fr", "Hollande-Méridionale"), ("ga", "An Ollainn Theas"), ("gl", "Holanda Meridional - Zuid-Holland"), ("gu", "સાઉથ હોલ\u{ac7}ન\u{acd}ડ"), ("he", "דרום הולנד"), ("hi", "दक\u{94d}षिण हॉल\u{948}\u{902}ड"), ("hu", "Dél-Holland"), ("hy", "Հարավային Հոլանդիա"), ("id", "Holland Selatan"), ("is", "Suður-Holland"), ("it", "Olanda Meridionale"), ("ja", "南ホラント州"), ("jv", "Holland Kidul"), ("ka", "სამხრეთი ჰოლანდია"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಹಾಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "자위트홀란트 주"), ("lt", "Pietų Olandija"), ("lv", "Dienvidholande"), ("mk", "Јужна Холандија"), ("mn", "Өмнө Холланд"), ("mr", "झाउड-हॉल\u{902}ड"), ("ms", "Zuid-Holland"), ("nb", "Zuid-Holland"), ("ne", "दक\u{94d}षिण हल\u{94d}यान\u{94d}ड"), ("nl", "Zuid-Holland"), ("no", "Zuid-Holland"), ("pl", "Holandia Południowa"), ("pt", "Holanda do Sul"), ("ro", "Olanda de Sud"), ("ru", "Южная Голландия"), ("si", "දක\u{dd4}ණ\u{dd4} ඕලන\u{dca}දය"), ("sk", "Južný Holland"), ("sl", "Južna Holandija"), ("so", "Koofurta Holland"), ("sq", "Hollanda e Jugut"), ("sr", "Јужна Холандија"), ("sr_Latn", "Južna Holandija"), ("sv", "Zuid-Holland"), ("sw", "Zuid-Holland"), ("ta", "தெற\u{bcd}கு ஹ\u{bbe}லந\u{bcd}து"), ("te", "ద7\u{c3f}ణ హ\u{c3e}లండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเซาท\u{e4c}ฮอลแลนด\u{e4c}"), ("tr", "Güney Hollanda"), ("uk", "Південна Голландія"), ("ur", "جنوبی ہولانت"), ("vi", "Zuid-Holland"), ("yue", "南荷蘭"), ("yue_Hans", "南荷兰"), ("zh", "南荷蘭省")]),
                        unofficial_name_list: ["Zuid-Holland"].to_vec(),
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
#[cfg(feature = "nl")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::NL,
        alpha3: Alpha3::NLD,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 31,
        currency_code: "EUR",
        gec: Some(GEC::NL),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("NED"),
        iso_long_name: "The Kingdom of the Netherlands",
        iso_short_name: "Netherlands",
        official_language_list: ["nl"].to_vec(),
        spoken_language_list: ["nl"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Dutch"),
        number: "528",
        postal_code: true,
        postal_code_format: Some("\\d{4} ?[A-Z]{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "NL",
        unofficial_name_list: [
            "Netherlands",
            "The Netherlands",
            "Niederlande",
            "Pays-Bas",
            "Países Bajos",
            "オランダ",
            "Nederland",
            "Нидерландия",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Netherlands"),
            ("af", "Nederland"),
            ("ak", "Netherlands"),
            ("am", "ኔፈሴሒን፥"),
            ("an", "Netherlands"),
            ("ar", "هولندا"),
            ("as", "নেড\u{9be}ৰলেণ\u{9cd}ড"),
            ("ay", "Netherlands"),
            ("az", "Hollandiya"),
            ("ba", "Netherlands"),
            ("be", "Нідэрланды"),
            ("bg", "Холандия"),
            ("bi", "Netherlands"),
            ("bn", "নেদ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("bn_IN", "নেদ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("br", "Izelvroioù"),
            ("bs", "Nizozemska"),
            ("ca", "Països Baixos"),
            ("ce", "Нидерландаш"),
            ("ch", "Netherlands"),
            ("cs", "Nizozemsko"),
            ("cv", "Нидерландаш"),
            ("cy", "Yr Iseldiroedd"),
            ("da", "Holland"),
            ("de", "Niederlande"),
            ("dv", "ނ\u{7ac}ދ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
            ("dz", "ན\u{f7a}་དར་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}།"),
            ("ee", "Netherlands"),
            ("el", "Ολλανδία"),
            ("en", "Netherlands"),
            ("eo", "Nederlando"),
            ("es", "Países Bajos"),
            ("et", "Holland"),
            ("eu", "Herbehereak"),
            ("fa", "هلند"),
            ("ff", "Holannda"),
            ("fi", "Alankomaat"),
            ("fo", "Niðurlond"),
            ("fr", "Pays-Bas"),
            ("fy", "Nederlân"),
            ("ga", "An Ísiltír"),
            ("gl", "Países Baixos"),
            ("gn", "Netherlands"),
            ("gu", "ન\u{ac7}ધરલ\u{ac7}ન\u{acd}ડ\u{acd}સ"),
            ("gv", "Yn Çheer Injil"),
            ("ha", "Netherlands"),
            ("he", "הולנד"),
            ("hi", "नीदरल\u{948}ण\u{94d}ड"),
            ("hr", "Nizozemska"),
            ("ht", "Peyiba"),
            ("hu", "Hollandia"),
            ("hy", "Նիդերլանդեր"),
            ("ia", "Pais Basse"),
            ("id", "Belanda"),
            ("io", "Nederlando"),
            ("is", "Holland"),
            ("it", "Paesi Bassi"),
            ("iu", "Netherlands"),
            ("ja", "オランダ"),
            ("ka", "ნიდერლანდები"),
            ("ki", "Netherlands"),
            ("kk", "Нидерланды"),
            ("kl", "Netherlands"),
            ("km", "\u{200b}ហ\u{17bc}ល\u{17d2}លង\u{17cb}"),
            ("kn", "ನ\u{cc6}ದರ\u{ccd}\u{200c}ಲ\u{ccd}ಯಾಂಡ\u{ccd}ಸ\u{ccd}"),
            ("ko", "네덜란드"),
            ("ku", "Holanda"),
            ("kv", "Нидерландъяс"),
            ("kw", "Iseldiryow"),
            ("ky", "Нидерландтар"),
            ("lo", "ປະເທດໂຮນລ\u{eb1}ງ"),
            ("lt", "Nyderlandai"),
            ("lv", "Nīderlande"),
            ("mi", "Hōrana"),
            ("mk", "Холандија"),
            ("ml", "നെതര\u{d4d}\u{200d}ലന\u{d4d}\u{200d}ഡ\u{d4d}സ\u{d4d}"),
            ("mn", "Нидерланд"),
            ("mr", "न\u{947}दरल\u{945}\u{902}डस\u{94d}"),
            ("ms", "Belanda"),
            ("mt", "Netherlands"),
            (
                "my",
                "နယ\u{103a}သာလန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Eben Eyong"),
            ("nb", "Nederland"),
            ("ne", "न\u{947}दरल\u{94d}याण\u{94d}ड"),
            ("nl", "Nederland"),
            ("nn", "Nederland"),
            ("nv", "Kéyah Wóyahgo Siʼánígíí"),
            ("oc", "Païses Basses"),
            ("or", "ନେଦରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
            ("pa", "ਨੀ\u{a02}ਦਰਲ\u{a48}\u{a02}ਡ"),
            ("pi", "न\u{947}दरल\u{948}\u{902}ड\u{94d}स"),
            ("pl", "Holandia"),
            ("ps", "هالېنډ"),
            ("pt", "Países Baixos"),
            ("pt_BR", "Países Baixos"),
            ("ro", "Olanda"),
            ("ru", "Нидерланды"),
            ("rw", "Nederilande"),
            ("sc", "Paisos Bassos"),
            ("sd", "Netherlands"),
            ("si", "නෙදර\u{dca}ලන\u{dca}තය"),
            ("sk", "Holandsko"),
            ("sl", "Nizozemska"),
            ("so", "Netherlands"),
            ("sq", "Holandë"),
            ("sr", "Холандија"),
            ("sv", "Nederländerna"),
            ("sw", "Uholanzi"),
            ("ta", "நெதர\u{bcd}ல\u{bbe}ந\u{bcd}து"),
            ("te", "న\u{c47}దర\u{c4d}ల\u{c3e}ండ\u{c4d}స\u{c4d}"),
            ("tg", "Нидерландия"),
            ("th", "เนเธอร\u{e4c}แลนด\u{e4c}"),
            ("ti", "ኔዘርላንድ"),
            ("tk", "Niderland"),
            ("tl", "Netherlands"),
            ("tr", "Hollanda"),
            ("tt", "Нидерландлар"),
            ("ug", "گوللاندىيە"),
            ("uk", "Нідерланди"),
            ("ur", "نیدرلینڈز"),
            ("uz", "Niderlandlar"),
            ("ve", "Netherlands"),
            ("vi", "Hoà Lan"),
            ("wa", "Bas Payis"),
            ("wo", "Olaand"),
            ("xh", "Netherlands"),
            ("yo", "Nẹ\u{301}dálándì"),
            ("zh_CN", "荷兰"),
            ("zh_HK", "荷蘭"),
            ("zh_TW", "荷蘭"),
            ("zu", "I-Netherlands"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
