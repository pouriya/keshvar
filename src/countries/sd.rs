// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of the Sudan

#[cfg(all(feature = "sd", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::SD;
    pub const ALPHA3: Alpha3 = Alpha3::SDN;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 249;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::SDG;
    pub const GEC: Option<GEC> = Some(GEC::SU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::SUD);
    pub const ISO_SHORT_NAME: &str = "Sudan";
    pub const ISO_LONG_NAME: &str = "The Republic of the Sudan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar", "en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar", "en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Sudanese");
    pub const NUMBER: &str = "729";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAfrica);
    pub const UN_LOCODE: &str = "SD";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Sudan", "السودان", "Soudan", "Sudán", "スーダン", "Soedan"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Sudan"),
        ("af", "Soedan"),
        ("ak", "Sudan"),
        ("am", "ሱ፣ን"),
        ("an", "Sudan"),
        ("ar", "السودان"),
        ("as", "ছ\u{9c1}দ\u{9be}ন"),
        ("ay", "Sudan"),
        ("az", "Sudan"),
        ("ba", "Sudan"),
        ("be", "Судан"),
        ("bg", "Судан"),
        ("bi", "Sudan"),
        ("bn", "স\u{9c1}দ\u{9be}ন"),
        ("bn_IN", "স\u{9c1}দ\u{9be}ন"),
        ("br", "Soudan"),
        ("bs", "Sudan"),
        ("ca", "Sudan"),
        ("ce", "Судан"),
        ("ch", "Sudan"),
        ("cs", "Súdán"),
        ("cv", "Судан"),
        ("cy", "Y Swdan"),
        ("da", "Sudan"),
        ("de", "Sudan"),
        ("dv", "ސ\u{7ab}ދ\u{7a7}ނ\u{7b0}"),
        ("dz", "ས\u{f74}་ད\u{f71}ན།"),
        ("ee", "Sudan"),
        ("el", "Σουδάν"),
        ("en", "Sudan"),
        ("eo", "Sudano"),
        ("es", "Sudán"),
        ("et", "Sudaan"),
        ("eu", "Sudan"),
        ("fa", "سودان"),
        ("ff", "Sudan"),
        ("fi", "Sudan"),
        ("fo", "Sudan"),
        ("fr", "Soudan"),
        ("fy", "Sûdan"),
        ("ga", "An tSúdáin"),
        ("gl", "Sudán"),
        ("gn", "Sudan"),
        ("gu", "સ\u{ac1}દાન"),
        ("gv", "Yn Toodaan"),
        ("ha", "Sudan"),
        ("he", "סודאן"),
        ("hi", "स\u{942}डान"),
        ("hr", "Sudan"),
        ("ht", "Soudan"),
        ("hu", "Szudán"),
        ("hy", "Սուդան"),
        ("ia", "Sudan"),
        ("id", "Sudan"),
        ("io", "Sudan"),
        ("is", "Súdan"),
        ("it", "Sudan"),
        ("iu", "Sudan"),
        ("ja", "スーダン"),
        ("ka", "სუდანი"),
        ("ki", "Sũdana"),
        ("kk", "Судан"),
        ("kl", "Sudan"),
        ("km", "ស\u{17ca}\u{17bc}ដង\u{17cb}"),
        ("kn", "ಸ\u{cc2}ಡಾನ\u{ccd}"),
        ("ko", "수단"),
        ("ku", "Sûdan"),
        ("kv", "Sudan"),
        ("kw", "Soudan"),
        ("ky", "Судан"),
        ("lo", "Sudan"),
        ("lt", "Sudanas"),
        ("lv", "Sudāna"),
        ("mi", "Hūtāne"),
        ("mk", "Судан"),
        ("ml", "സ\u{d41}ഡ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Судан"),
        ("mr", "स\u{941}दान"),
        ("ms", "Sudan"),
        ("mt", "Sudan"),
        (
            "my",
            "ဆ\u{1030}ဒန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tudan"),
        ("nb", "Sudan"),
        ("ne", "स\u{941}डान"),
        ("nl", "Soedan"),
        ("nn", "Sudan"),
        ("nv", "Ásáí Dineʼé Daalzhiní Bikéyah"),
        ("oc", "Sodan"),
        ("or", "ସ\u{b42}ଡ\u{b3e}ନ"),
        ("pa", "ਸ\u{a42}ਡਾਨ"),
        ("pi", "स\u{942}डान"),
        ("pl", "Sudan"),
        ("ps", "سوډان"),
        ("pt", "Sudão"),
        ("pt_BR", "Sudão"),
        ("ro", "Sudan"),
        ("ru", "Судан"),
        ("rw", "Sudani"),
        ("sc", "Sudàn"),
        ("sd", "Sudan"),
        ("si", "ස\u{dd4}ඩ\u{dcf}නය"),
        ("sk", "Sudán"),
        ("sl", "Sudan"),
        ("so", "Sudaan"),
        ("sq", "Sudan"),
        ("sr", "Судан"),
        ("sv", "Sudan"),
        ("sw", "Sudan"),
        ("ta", "சூட\u{bbe}ன\u{bcd}"),
        ("te", "సుడ\u{c3e}న\u{c4d}"),
        ("tg", "Судон"),
        ("th", "ซ\u{e39}ดาน"),
        ("ti", "ሱዳን"),
        ("tk", "Sudan"),
        ("tl", "Sudan"),
        ("tr", "Sudan"),
        ("tt", "Судан"),
        ("ug", "سۇدان"),
        ("uk", "Судан"),
        ("ur", "سوڈان"),
        ("uz", "Sudan"),
        ("ve", "Sudan"),
        ("vi", "Xu-đanh"),
        ("wa", "Soudan"),
        ("wo", "Suudaan"),
        ("xh", "Sudan"),
        ("yo", "Sudan"),
        ("zh_CN", "苏丹"),
        ("zh_HK", "蘇丹"),
        ("zh_TW", "蘇丹"),
        ("zu", "ISudan"),
    ];
    #[cfg(all(feature = "sd", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 12.862807;
        pub const LONGITUDE: f64 = 30.217636;
        pub const MAX_LATITUDE: f64 = 22.224918;
        pub const MAX_LONGITUDE: f64 = 38.69379989999999;
        pub const MIN_LATITUDE: f64 = 9.3472209;
        pub const MIN_LONGITUDE: f64 = 21.8146345;
        pub const NORTHEAST_LATITUDE: f64 = 22.224918;
        pub const NORTHEAST_LONGITUDE: f64 = 38.69379989999999;
        pub const SOUTHWEST_LATITUDE: f64 = 9.3472209;
        pub const SOUTHWEST_LONGITUDE: f64 = 21.8146345;
    }
}
#[cfg(all(feature = "sd", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 12.862807,
            longitude: 30.217636,
            max_latitude: 22.224918,
            max_longitude: 38.69379989999999,
            min_latitude: 9.3472209,
            min_longitude: 21.8146345,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 22.224918,
                    longitude: 38.69379989999999,
                },
                southwest: CountryGeoBound {
                    latitude: 9.3472209,
                    longitude: 21.8146345,
                },
            },
        }
    }
}

#[cfg(all(feature = "sd", feature = "subdivisions"))]
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
                    "DC",
                    Subdivision{
                        name: "DC",
                        country_alpha2: Alpha2::SD,
                        code: "DC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وسط دار فور"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল দ\u{9be}রফ\u{9c1}র"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄓𑄢\u{11134}𑄜\u{1112a}𑄢\u{11134}"), ("ceb", "Central Darfur State"), ("da", "Central Darfur"), ("de", "Wasat Darfur"), ("el", "Κεντρικό Νταρφούρ"), ("en", "Central Darfur"), ("es", "Darfur Central"), ("fa", "دارفور مرکزی"), ("fi", "Central Darfur"), ("fr", "Darfour-Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ દારફ\u{ac1}ર"), ("hi", "क\u{947}\u{902}द\u{94d}रीय दारफ\u{941}र"), ("id", "Central Darfur"), ("it", "Central Darfur"), ("ja", "中部ダルフール州"), ("ka", "ცენტრალური დარფური"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಡಾರ\u{ccd}ಫರ\u{ccd}"), ("ko", "중앙다르푸르 주"), ("lt", "Vidurio Darfūras"), ("lv", "Centrālā Dārfūra"), ("mr", "स\u{947}\u{902}ट\u{94d}रल दार\u{941}र"), ("ms", "Central Darfur"), ("nb", "Darfur senter"), ("nl", "Darfoer"), ("no", "Darfur senter"), ("pl", "Darfur Środkowy"), ("pt", "Darfur Central"), ("ro", "Statul Darfur Central"), ("ru", "Центральный Дарфур"), ("si", "මධ\u{dca}\u{200d}යම ඩ\u{dcf}ෆ\u{dd4}ර\u{dca}"), ("sv", "Centrala Dafur"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ட\u{bbe}ர\u{bcd}பர\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} డ\u{c3e}ర\u{c4d}ఫుర\u{c4d}"), ("th", "ร\u{e31}ฐดาร\u{e4c}ฟ\u{e31}วกลาง"), ("tr", "Orta Darfur"), ("uk", "Центральний Дарфур"), ("ur", "وسطی دارفور"), ("vi", "Miền Trung Darfur"), ("zh", "中達爾富爾")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DE",
                    Subdivision{
                        name: "DE",
                        country_alpha2: Alpha2::SD,
                        code: "DE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية شرق دارفور"), ("bn", "ইস\u{9cd}ট দ\u{9be}রফ\u{9c1}র"), ("ca", "Darfur de l’Est"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄓𑄢\u{11134}𑄜\u{1112a}𑄢\u{11134}"), ("ceb", "East Darfur"), ("da", "East Darfur"), ("de", "Scharq Darfur"), ("el", "Ανατολικό Νταρφούρ (Ήστ Νταρφούρ)"), ("en", "East Darfur"), ("es", "Darfur del Este"), ("fa", "دارفور شرقی"), ("fi", "Itä-Darfur"), ("fr", "Darfour-Oriental"), ("gu", "ઇસ\u{acd}ટ દારફ\u{ac1}ર"), ("hi", "प\u{942}र\u{94d}वी दारफ\u{941}र"), ("id", "East Darfur"), ("it", "East Darfur"), ("ja", "東ダルフール州"), ("ka", "აღმოსავლეთი დარფური"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಡಾರ\u{ccd}ಫ\u{cc2}ರ\u{ccd}"), ("ko", "동다르푸르 주"), ("lt", "Rytų Darfūras"), ("lv", "Austrumdārfūra"), ("mr", "प\u{942}र\u{94d}व दारफ\u{941}र"), ("ms", "East Darfur"), ("nb", "Vest Darfur"), ("nl", "Oost Darfoer"), ("no", "Vest Darfur"), ("pl", "Darfur Wschodni"), ("pt", "Darfur Este"), ("ro", "Statul Darfur de Est"), ("ru", "Восточный Дарфур"), ("si", "නැගෙනහ\u{dd2}ර ඩ\u{dcf}ර\u{dca}ෆ\u{dd4}ර\u{dca}"), ("sv", "Västra Dafur"), ("ta", "கிழக\u{bcd}கு ட\u{bbe}ர\u{bcd}பர\u{bcd}"), ("te", "తూర\u{c4d}పు డ\u{c3e}ర\u{c4d}ఫుర\u{c4d}"), ("th", "เอ\u{e34}ร\u{e4c}ท ดาฟ\u{e39}ล"), ("tr", "East Darfur"), ("uk", "Східний Дарфур"), ("ur", "مشرقی دارفور"), ("vi", "Đông Darfur"), ("zh", "东達爾富爾")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DN",
                    Subdivision{
                        name: "DN",
                        country_alpha2: Alpha2::SD,
                        code: "DN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.7661969), longitude: Some(24.9042208), max_latitude: Some(20.0064945), min_latitude: Some(11.7224969), max_longitude: Some(27.532602), min_longitude: Some(22.75691)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية شمال دارفور"), ("bg", "Северен Дарфур"), ("bn", "দক\u{9cd}ষিন দ\u{9be}রফ\u{9c1}র"), ("ca", "Shamal Darfur"), ("ccp", "𑄅\u{1112a}𑄦\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄓𑄢\u{11134}𑄜\u{1112a}𑄢\u{11134}"), ("ceb", "North Darfur"), ("da", "Norddarfur"), ("de", "Schamal Darfur"), ("el", "Βόρειο Νταρφούρ"), ("en", "North Darfur"), ("es", "Darfur del Norte"), ("fa", "دارفور شمالی"), ("fi", "Pohjois-Darfur"), ("fr", "Darfour du Nord"), ("gu", "નોર\u{acd}થ ડાર\u{acd}ફ\u{ac1}ર"), ("he", "צפון דארפור"), ("hi", "उत\u{94d}तरी दार\u{94d}फ\u{941}र"), ("id", "Darfur Utara"), ("it", "Darfur Settentrionale"), ("ja", "北ダルフール州"), ("ka", "ჩრდილოეთი დარფური"), ("kn", "ಉತ\u{ccd}ತರ ಡಾರ\u{ccd}ಫರ\u{ccd}"), ("ko", "북다르푸르 주"), ("lt", "Šiaurės Darfūras"), ("lv", "Ziemeļdārfūra"), ("mr", "नॉर\u{94d}थ दारफ\u{941}र"), ("ms", "North Darfur"), ("nb", "Nord-Darfur"), ("nl", "Shamal-Darfur"), ("no", "Nord-Darfur"), ("pl", "Darfur Północny"), ("pt", "Darfur do Norte"), ("ro", "Statul Darfur de Nord"), ("ru", "Северный Дарфур"), ("si", "උත\u{dd4}ර\u{dd4} ඩ\u{dcf}ර\u{dca}ෆ\u{dd4}ර\u{dca}"), ("sr", "Северни Дарфур"), ("sr_Latn", "Severni Darfur"), ("sv", "Shamal Darfur"), ("sw", "Kaskazini Darfur"), ("ta", "வடக\u{bcd}கு ட\u{bbe}ர\u{bcd}பர\u{bcd}"), ("te", "ఉత\u{c4d}తర డ\u{c3e}ర\u{c4d}ఫూర\u{c4d}"), ("th", "ดาร\u{e4c}ฟ\u{e39}ร\u{e4c}เหน\u{e37}อ"), ("tr", "Kuzey Darfur Eyaleti"), ("uk", "Північний Дарфур"), ("ur", "شمالی دارفور"), ("vi", "Bắc Darfur"), ("zh", "北達爾富爾省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DS",
                    Subdivision{
                        name: "DS",
                        country_alpha2: Alpha2::SD,
                        code: "DS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.6488639), longitude: Some(24.9042208), max_latitude: Some(13.133373), min_latitude: Some(8.6366421), max_longitude: Some(27.908629), min_longitude: Some(23.047403)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب دارفور"), ("bg", "Южен Дарфур"), ("bn", "দক\u{9cd}ষিণ দ\u{9be}রফ\u{9c1}র"), ("ca", "Janob Darfur"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄓𑄢\u{11134}𑄜\u{1112a}𑄢\u{11134}"), ("ceb", "South Darfur"), ("da", "Syddarfur"), ("de", "Dschanub Darfur"), ("el", "Νότιο Νταρφούρ"), ("en", "South Darfur"), ("es", "Darfur del Sur"), ("fa", "دارفور جنوبی"), ("fi", "Etelä-Darfur"), ("fr", "Darfour du Sud"), ("gu", "સાઉથ ડાર\u{acd}ફ\u{ac1}ર"), ("hi", "दक\u{94d}षिण दारफ\u{941}र"), ("hy", "Հարավային Դարֆուր"), ("id", "Darfur Selatan"), ("it", "Darfur Meridionale"), ("ja", "南ダルフール州"), ("ka", "სამხრეთი დარფური"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಡಾರ\u{ccd}ಫರ\u{ccd}"), ("ko", "남다르푸르 주"), ("lt", "Pietų Darfūras"), ("lv", "Dienviddārfūra"), ("mr", "साऊथ दारफ\u{941}र"), ("ms", "South Darfur"), ("nb", "Sør Darfur"), ("nl", "Janub-Darfur"), ("no", "Sør Darfur"), ("pl", "Darfur Południowy"), ("pt", "Darfur do Sul"), ("ro", "Statul Darfur de Sud"), ("ru", "Южный Дарфур"), ("si", "දක\u{dd4}ණ\u{dd4} ඩ\u{dcf}ර\u{dca}ෆ\u{dd4}ර\u{dca}"), ("sr", "Јужни Дарфур"), ("sr_Latn", "Južni Darfur"), ("sv", "Janub Darfur"), ("sw", "Kusini Darfur"), ("ta", "தெற\u{bcd}கு ட\u{bbe}ர\u{bcd}பர\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ డ\u{c3e}ర\u{c4d}ఫుర\u{c4d}"), ("th", "เซาท\u{e4c} ดาร\u{e4c}ฟ\u{e39}ล"), ("tr", "Güney Darfur Eyaleti"), ("uk", "Південний Дарфур"), ("ur", "جنوبی دارفور"), ("vi", "Nam Darfur"), ("zh", "南达尔富尔省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DW",
                    Subdivision{
                        name: "DW",
                        country_alpha2: Alpha2::SD,
                        code: "DW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.8463561), longitude: Some(23.0011989), max_latitude: Some(14.972213), min_latitude: Some(10.694536), max_longitude: Some(24.720192), min_longitude: Some(21.838947)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غرب دارفور"), ("bg", "Западен Дарфур"), ("bn", "পশ\u{9cd}চিম দ\u{9be}রফ\u{9c1}র"), ("ca", "Gharb Darfur"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄓𑄢\u{11134}𑄜\u{1112a}𑄢\u{11134}"), ("ceb", "West Darfur"), ("da", "Vestdarfur"), ("de", "Gharb Darfur"), ("el", "Δυτικό Νταρφούρ"), ("en", "West Darfur"), ("es", "Darfur del Oeste"), ("fa", "دارفور غربی"), ("fi", "Länsi-Darfur"), ("fr", "Darfour-Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ ડર\u{acd}ફર"), ("hi", "पश\u{94d}चिम दारफ\u{941}र"), ("id", "Darfur Barat"), ("it", "Darfur Occidentale"), ("ja", "西ダルフール州"), ("ka", "დასავლეთი დარფური"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಡಾರ\u{ccd}ಫರ\u{ccd}"), ("ko", "서다르푸르 주"), ("lt", "Vakarų Darfūras"), ("lv", "Rietumdārfūra"), ("mr", "पश\u{94d}चिम दारफ\u{941}र"), ("ms", "West Darfur"), ("nb", "Vest Dafur"), ("nl", "Gharb-Darfur"), ("no", "Vest Dafur"), ("pl", "Darfur Zachodni"), ("pt", "Darfur Ocidental"), ("ro", "Statul Darfur de Vest"), ("ru", "Западный Дарфур"), ("si", "බටහ\u{dd2}ර ඩ\u{dcf}ර\u{dca}ෆ\u{dd4}ර\u{dca}"), ("sr", "Западни Дарфур"), ("sr_Latn", "Zapadni Darfur"), ("sv", "Gharb Darfur"), ("sw", "Magharibi Darfur"), ("ta", "மேற\u{bcd}கு ட\u{bbe}ர\u{bcd}பர\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ డ\u{c3e}ర\u{c4d}ఫూర\u{c4d}"), ("th", "ร\u{e31}ฐดาร\u{e4c}ฟ\u{e31}วร\u{e4c}ตะว\u{e31}นตก"), ("tr", "Batı Darfur Eyaleti"), ("uk", "Західний Дарфур"), ("ur", "مغربی دارفور"), ("vi", "Bang West Darfur"), ("zh", "西达尔富尔省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GD",
                    Subdivision{
                        name: "GD",
                        country_alpha2: Alpha2::SD,
                        code: "GD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.024307), longitude: Some(35.3685679), max_latitude: Some(14.0808871), min_latitude: Some(13.9824623), max_longitude: Some(35.4345988), min_longitude: Some(35.3189849)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "القضارف"), ("bg", "Ал Кадариф"), ("bn", "আল ক\u{9be}দ\u{9be}রিফ"), ("ca", "Estat de Gedarif"), ("ccp", "𑄃𑄣\u{11134} 𑄇\u{1112a}𑄠𑄓𑄢\u{11128}𑄛\u{11134}"), ("ceb", "Al Qadarif State"), ("cs", "Al Qadarif"), ("da", "Al Qadarif"), ("de", "Al-Qadarif"), ("el", "Αλ Κανταρίφ"), ("en", "Al Qadarif"), ("es", "Gadarif"), ("fa", "القضارف"), ("fi", "Al Qadarif"), ("fr", "Al Qadarif"), ("gu", "અલ કાદારીફ"), ("hi", "अल कादरीफ"), ("id", "Al Qadarif"), ("it", "Gadaref"), ("ja", "ガダーレフ州"), ("ka", "ალ-ქადარიფის შტატი"), ("kk", "Гедареф қаласы"), ("kn", "ಅಲ\u{ccd} ಖದರ\u{cbf}ಫ\u{ccd}"), ("ko", "알카다리프 주"), ("lt", "Kadarifas"), ("lv", "Gadārifa"), ("mr", "अल कादरीफ"), ("ms", "Al Qadarif"), ("nb", "Al-Qadarif"), ("nl", "Al-Qadarif"), ("no", "Al-Qadarif"), ("pl", "Al-Kadarif"), ("pt", "Gadarife"), ("ro", "Statul Gedarif"), ("ru", "Гедареф"), ("si", "අල\u{dca} කඩර\u{dd2}ෆ\u{dca}"), ("sv", "Al-Qadarif"), ("sw", "Al Qadarif"), ("ta", "அல\u{bcd} கிடரிப\u{bcd}பி"), ("te", "అల\u{c4d} ఖడ\u{c3e}ర\u{c3f}ఫ\u{c4d}"), ("th", "อ\u{e31}ล คาดาร\u{e34}ฟ"), ("tr", "El Kadarif Eyaleti"), ("uk", "Гедареф"), ("ur", "القضارف (ریاست)"), ("vi", "Al Qadarif"), ("zh", "加达里夫省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GK",
                    Subdivision{
                        name: "GK",
                        country_alpha2: Alpha2::SD,
                        code: "GK",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غرب كردفان"), ("ca", "Kordofan de l’Oest"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄇\u{1112a}𑄢\u{11134}𑄓\u{1112a}𑄜𑄚\u{11134}"), ("de", "Gharb Kurdufan"), ("en", "West Kurdufan"), ("es", "Kordofán del Oeste"), ("fa", "غرب کردفان"), ("fr", "Kordofan-Occidental"), ("id", "Kurdufan Barat"), ("it", "Kordofan Occidentale"), ("ja", "西コルドファン州"), ("ko", "서쿠르두판 주"), ("nl", "Gharb-Kordofan"), ("pl", "Kordofan Zachodni"), ("pt", "Cordofão Ocidental"), ("ro", "Statul Kordofan de Vest"), ("ru", "Западный Кордофан"), ("sw", "Magharibi Kurdufan"), ("uk", "Західний Кордофан"), ("ur", "مغربی کردفان"), ("zh", "西科尔多凡州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GZ",
                    Subdivision{
                        name: "GZ",
                        country_alpha2: Alpha2::SD,
                        code: "GZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.8859611), longitude: Some(33.438353), max_latitude: Some(15.4767249), min_latitude: Some(13.604472), max_longitude: Some(34.3057649), min_longitude: Some(32.415869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الجزيرة"), ("bg", "Ал-Джазира"), ("bn", "আল জ\u{9be}জির\u{9be}"), ("ca", "Gezira"), ("ccp", "𑄃𑄣\u{11134} 𑄎𑄢\u{11128}𑄢𑄦\u{11134}"), ("ceb", "Al Jazirah State"), ("da", "Al Jazirah"), ("de", "Al-Dschazira"), ("el", "Αλ Τζαζίρα"), ("en", "Al Jazirah"), ("es", "Gezira"), ("fa", "جزیره"), ("fi", "Al Jazirah"), ("fr", "Al Jazirah"), ("gu", "અલ જાઝીરાહ"), ("he", "אל-ג׳זירה"), ("hi", "अल जज\u{93c}ीरा (राज\u{94d}य)"), ("id", "Al Jazirah"), ("it", "Gezira"), ("ja", "ジャジーラ州"), ("ka", "ალ-ჯაზირის შტატი"), ("kn", "ಅಲ\u{ccd} ಜಝ\u{cbf}ರಾಹ\u{ccd}"), ("ko", "알자지라 주"), ("lt", "Džezira"), ("lv", "Džazīra"), ("mr", "अल जाजीराह"), ("ms", "Al Jazirah"), ("nb", "Al Jazirah"), ("nl", "Al-Jazirah"), ("no", "Al Jazirah"), ("pl", "Al-Dżazira"), ("pt", "Al Jazirah (estado)"), ("ro", "Statul Al Jazirah"), ("ru", "Эль-Гезира"), ("si", "අල\u{dca} ජස\u{dd3}ර\u{dcf}"), ("sv", "Al-Jazirah"), ("sw", "Al Jazirah"), ("ta", "அல\u{bcd} ஜஸ\u{bc0}ர\u{bbe}"), ("te", "అల\u{c4d} జజ\u{c40}ర\u{c3e}"), ("th", "อ\u{e31}ลจาซ\u{e35}รา"), ("tr", "El Cezire Eyaleti"), ("uk", "Ель-Гезіра"), ("ur", "الجزیرہ (ریاست)"), ("vi", "Al Jazirah"), ("zh", "杰济拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "KA",
                        country_alpha2: Alpha2::SD,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4581332), longitude: Some(36.4039629), max_latitude: Some(15.4994253), min_latitude: Some(15.4122089), max_longitude: Some(36.4511558), min_longitude: Some(36.3763976)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كسلا"), ("bg", "Касала"), ("bn", "ক\u{9be}স\u{9be}ল\u{9be}"), ("ca", "Estat de Kassala"), ("ccp", "𑄇𑄥𑄣"), ("ceb", "Kassala State"), ("da", "Kassala (stat)"), ("de", "Kassala"), ("el", "Κάσσαλα"), ("en", "Kassala"), ("es", "Kasala (estado)"), ("fa", "استان کسلا"), ("fi", "Kassala (osavaltio)"), ("fr", "Kassala"), ("gu", "કસાલા"), ("hi", "कसाला (राज\u{94d}य)"), ("id", "Kassala"), ("it", "Cassala"), ("ja", "カッサラ州"), ("ka", "კასალის შტატი"), ("kn", "ಕಸ\u{ccd}ಸಲಾ"), ("ko", "카살라 주"), ("lt", "Kasala"), ("lv", "Kasala"), ("mr", "कासला"), ("ms", "Kassala"), ("nb", "Kassala"), ("nl", "Kassala"), ("no", "Kassala"), ("pl", "Kassala"), ("pt", "Kassala"), ("ro", "Kassala"), ("ru", "Кассала"), ("si", "කස\u{dca}සල\u{dcf}"), ("sv", "Kassala"), ("sw", "Kassala"), ("ta", "கஸ\u{bcd}ஸல\u{bbe}"), ("te", "కస\u{c4d}స\u{c3e}ల\u{c3e}"), ("th", "คาสซาลา"), ("tr", "Kassala Eyaleti"), ("uk", "Кассала"), ("ur", "کسلا (ریاست)"), ("vi", "Kassala"), ("zh", "卡萨拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KH",
                    Subdivision{
                        name: "KH",
                        country_alpha2: Alpha2::SD,
                        code: "KH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.5006544), longitude: Some(32.5598994), max_latitude: Some(15.615997), min_latitude: Some(15.3870932), max_longitude: Some(32.6859455), min_longitude: Some(32.4592475)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الخرطوم"), ("be", "Хартум"), ("bg", "Хартум"), ("bn", "খ\u{9be}র\u{9cd}ত\u{9c1}ম"), ("ccp", "𑄈𑄢\u{11134}𑄑\u{1112a}𑄠\u{1112e}𑄟\u{11134}"), ("ceb", "Khartoum"), ("da", "Khartoum"), ("de", "Al-Chartum"), ("el", "Χαρτούμ"), ("en", "Khartoum"), ("es", "Jartum"), ("fa", "استان خارطوم"), ("fi", "Khartoum"), ("fr", "Khartoum"), ("gu", "ખાર\u{acd}ટ\u{ac2}મ"), ("he", "ח׳רטום (מדינה)"), ("hi", "खार\u{94d}त\u{942}म"), ("id", "Khartum"), ("it", "Khartum"), ("ja", "ハルツーム州"), ("ka", "ხარტუმის შტატი"), ("kn", "ಖಾರ\u{ccd}ಟ\u{ccc}ಮ\u{ccd}"), ("ko", "하르툼 주"), ("lt", "Chartumas"), ("lv", "Hartūma"), ("mr", "खार\u{94d}ट\u{942}म"), ("ms", "Khartoum"), ("nb", "Khartoum"), ("nl", "Khartoem"), ("no", "Khartoum"), ("pl", "Chartum"), ("pt", "Cartum"), ("ro", "Statul Khartoum"), ("ru", "Хартум"), ("si", "ක\u{dcf}ර\u{dca}ටෝඋම\u{dca}"), ("sv", "Khartoum"), ("sw", "Khartoum"), ("ta", "க\u{bbe}ர\u{bcd}டூம\u{bcd}"), ("te", "ఖ\u{c3e}ర\u{c4d}ట\u{c4b}మ\u{c4d}"), ("th", "คาร\u{e4c}ท\u{e39}ม"), ("tr", "Hartum Eyaleti"), ("uk", "Хартум"), ("ur", "خرطوم (ریاست)"), ("vi", "Khartoum"), ("zh", "喀土穆省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KN",
                    Subdivision{
                        name: "KN",
                        country_alpha2: Alpha2::SD,
                        code: "KN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية شمال كردفان"), ("bg", "Северен Курдуфан"), ("bn", "উত\u{9cd}তর ক\u{9c1}র\u{9cd}দফ\u{9be}ন"), ("ca", "Kordofan del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄇\u{1112a}𑄢\u{11134}𑄓\u{1112a}𑄜𑄚\u{11134}"), ("ceb", "North Kordofan"), ("da", "Nordkordofan"), ("de", "Schamal Kurdufan"), ("el", "Βόρειο Κορντοφάν"), ("en", "North Kurdufan"), ("es", "Kordofán del Norte"), ("fa", "کردفان شمالی"), ("fi", "North Kurdufan"), ("fr", "Kordofan du Nord"), ("gu", "નોર\u{acd}થ ક\u{ac1}ર\u{acd}દ\u{ac2}ફાન"), ("hi", "उत\u{94d}तर क\u{941}र\u{94d}द\u{941}फान"), ("id", "Kurdufan Utara"), ("it", "Kordofan Settentrionale"), ("ja", "北コルドファン州"), ("ka", "ჩრდილოეთი კორდოფანი"), ("kn", "ಉತ\u{ccd}ತರ ಕರ\u{ccd}ಡುಫನ\u{ccd}"), ("ko", "북쿠르두판 주"), ("lt", "Šiaurės Kordofanas"), ("lv", "Ziemeļkordofāna"), ("mr", "नॉर\u{94d}थ क\u{941}र\u{94d}द\u{941}फान"), ("ms", "North Kurdufan"), ("nb", "Nord Kurdufan"), ("nl", "Shamal-Kordofan"), ("no", "Nord Kurdufan"), ("pl", "Kordofan Północny"), ("pt", "Cordofão do Norte"), ("ro", "Statul Kordofan de Nord"), ("ru", "Северный Кордофан"), ("si", "උත\u{dd4}ර\u{dd4} ක\u{dd4}ර\u{dca}ද\u{dd4}ෆ\u{dcf}න\u{dca}"), ("sv", "Shamal Kurdufan"), ("sw", "Kaskazini Kurdufan"), ("ta", "வடக\u{bcd}கு குர\u{bcd}துப\u{bbe}ன\u{bcd}"), ("te", "ఉత\u{c4d}తరం కుర\u{c4d}దుఫ\u{c3e}న\u{c4d}"), ("th", "นอร\u{e4c}ท เคอด\u{e39}แฟน"), ("tr", "Kuzey Kordofan Eyaleti"), ("uk", "Північний Кордофан"), ("ur", "شمالی کردفان"), ("vi", "Bắc Kurdufan"), ("zh", "北科尔多凡省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KS",
                    Subdivision{
                        name: "KS",
                        country_alpha2: Alpha2::SD,
                        code: "KS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.1990192), longitude: Some(29.4179324), max_latitude: Some(12.7500065), min_latitude: Some(9.345832), max_longitude: Some(32.473745), min_longitude: Some(27.2556731)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب كردفان"), ("bg", "Южен Курдуфан"), ("bn", "স\u{9be}উথ কোদোফ\u{9cd}য\u{9be}ন"), ("ca", "Kordofan del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇\u{1112a}𑄢\u{11134}𑄓\u{1112a}𑄜𑄚\u{11134}"), ("ceb", "South Kordofan"), ("cs", "Jižní Kordofán"), ("da", "Sydkordofan"), ("de", "Dschanub Kurdufan"), ("el", "Νότιο Κορντοφάν"), ("en", "South Kurdufan"), ("es", "Kordofán del Sur"), ("fa", "کردفان جنوبی"), ("fi", "Etelä-Kordofan"), ("fr", "Kordofan du Sud"), ("gu", "સાઉથ કોર\u{acd}ડોફ\u{ac7}ન"), ("hi", "दक\u{94d}षिणी कोर\u{94d}डोफ\u{948}न"), ("hu", "Dél-Kordofán"), ("hy", "Հարավային Կորդոֆան"), ("id", "Kurdufan Selatan"), ("it", "Kordofan Meridionale"), ("ja", "南コルドファン州"), ("ka", "სამხრეთი კორდოფანი"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಕೊರ\u{ccd}ಡೊಫಾನ\u{ccd}"), ("ko", "남쿠르두판 주"), ("lt", "Pietų Kordofanas"), ("lv", "Dienvidkordofāna"), ("mr", "साऊथ कॉर\u{94d}डोफ\u{945}न"), ("ms", "South Kordofan"), ("nb", "Sør Kordofan"), ("nl", "Janub-Kordofan"), ("no", "Sør Kordofan"), ("pl", "Kordofan Południowy"), ("pt", "Cordofão do Sul"), ("ro", "Statul Kordofan de Sud"), ("ru", "Южный Кордофан"), ("si", "දක\u{dd4}ණ\u{dd4} කොර\u{dca}ඩෝෆ\u{dcf}න\u{dca}"), ("sr", "Јужни Кордофан"), ("sr_Latn", "Južni Kordofan"), ("sv", "Janub Kurdufan"), ("sw", "Kusini Kurdufan"), ("ta", "தெற\u{bcd}கு கோர\u{bcd}டோப\u{bbe}ன\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ క\u{c4b}ర\u{c4d}డ\u{c4b}ఫ\u{c3e}న\u{c4d}"), ("th", "คอร\u{e37}โดแฟนใต\u{e49}"), ("tr", "Güney Kordofan Eyaleti"), ("uk", "Південний Кордофан"), ("ur", "جنوبی کردفان"), ("vi", "Nam Kordofan"), ("zh", "南科尔多凡省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::SD,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5860078), longitude: Some(34.1531947), max_latitude: Some(12.56568), min_latitude: Some(9.500347999999999), max_longitude: Some(35.09243), min_longitude: Some(33.1359769)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "النيل الأزرق"), ("bg", "Сини Нил"), ("bn", "ব\u{9cd}ল\u{9c1} নীল"), ("ca", "Nil Blau"), ("ccp", "𑄝\u{11133}𑄣\u{1112a} 𑄚\u{1112d}𑄣\u{11134}"), ("ceb", "Blue Nile (estado)"), ("da", "Blå Nil"), ("de", "An-Nil al-azraq"), ("el", "Μπλε Νείλος"), ("en", "Blue Nile"), ("es", "Nilo Azul"), ("fa", "نیل آبی"), ("fi", "Blue Nile"), ("fr", "Nil Bleu"), ("gu", "બ\u{acd}લ\u{ac1} નાઇલ"), ("hi", "ब\u{94d}ल\u{942} नाइल (राज\u{94d}य)"), ("id", "Nil Biru"), ("it", "Nilo Azzurro"), ("ja", "青ナイル州"), ("ka", "ცისფერი ნილოსის შტატი"), ("kn", "ಬ\u{ccd}ಲ\u{cc2} ನೈಲ\u{ccd}"), ("ko", "청나일 주"), ("lt", "Žydrojo Nilo vilajetas"), ("lv", "Zilās Nīlas vilājs"), ("mr", "ब\u{94d}ल\u{942} नाईल"), ("ms", "Blue Nile"), ("nb", "An-Nil al-Azraq"), ("nl", "An-Nil-al-Azraq"), ("no", "An-Nil al-Azraq"), ("pl", "Nil Błękitny"), ("pt", "Nilo Azul"), ("ro", "Statul Nilul Albastru"), ("ru", "Голубой Нил"), ("si", "න\u{dd2}ල\u{dca} නය\u{dd2}ල\u{dca}"), ("sr", "Плави Нил"), ("sr_Latn", "Plavi Nil"), ("sv", "An-Nil al-Azraq"), ("sw", "Bluu Nile"), ("ta", "ப\u{bcd}ளூ நில"), ("te", "బ\u{c4d}లూ న\u{c48}ల\u{c4d}"), ("th", "บล\u{e39}นาย"), ("tr", "Mavi Nil Eyaleti"), ("uk", "Блакитний Ніл"), ("ur", "نیل ازرق (ریاست)"), ("vi", "Nin Xanh"), ("zh", "青尼罗省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NO",
                    Subdivision{
                        name: "NO",
                        country_alpha2: Alpha2::SD,
                        code: "NO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.4448963), longitude: Some(30.1589303), max_latitude: Some(22.225082), min_latitude: Some(16.511393), max_longitude: Some(32.637459), min_longitude: Some(25.0)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الشمالية"), ("bg", "Северна провинция"), ("bn", "নর\u{9cd}দ\u{9be}ন"), ("ca", "Estat del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Northern"), ("da", "Nordlige Sudan"), ("de", "Asch-Schamaliyya"), ("el", "Νόρθερν"), ("en", "Northern"), ("es", "Norte"), ("eu", "Iparraldea"), ("fa", "شمالیه"), ("fi", "Northern"), ("fr", "Nord"), ("gu", "નોર\u{acd}થન"), ("hi", "उत\u{94d}तरी"), ("id", "Asy-Syamaliyah"), ("it", "Nord"), ("ja", "北部州"), ("ka", "ჩრდილოეთის შტატი"), ("kn", "ಉತ\u{ccd}ತರದ"), ("ko", "북부 주"), ("lt", "Šiaurės vilajetas"), ("lv", "Ziemeļu vilājs"), ("mr", "नॉर\u{94d}थर\u{94d}न"), ("ms", "Wilayah Utara"), ("nb", "Nord"), ("nl", "Ash-Shamaliyah"), ("no", "Nord"), ("pl", "Prowincja Północna"), ("pt", "Estado do Norte"), ("ro", "Statul de Nord"), ("ru", "Северный штат"), ("si", "නොදර\u{dca}න\u{dca}"), ("sv", "Ash-Shamaliyya"), ("sw", "Kaskazini"), ("ta", "வடக\u{bcd}கு"), ("te", "ఉత\u{c4d}తర"), ("th", "ร\u{e31}ฐซ\u{e39}ดานเหน\u{e37}อ"), ("tr", "Kuzey Eyaleti"), ("uk", "Північний штат"), ("ur", "شمالی (ریاست)"), ("vi", "Khu vực Phía Bắc"), ("zh", "北部省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NR",
                    Subdivision{
                        name: "NR",
                        country_alpha2: Alpha2::SD,
                        code: "NR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.1142529), longitude: Some(33.7964613), max_latitude: Some(22.006193), min_latitude: Some(15.9642631), max_longitude: Some(35.69729090000001), min_longitude: Some(31.8534791)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نهر النيل"), ("bg", "Река Нил"), ("bn", "নিল নদ"), ("ca", "Nahr an-Nil"), ("ccp", "𑄢\u{11128}𑄞𑄢\u{11134} 𑄚\u{1112d}𑄣\u{11134}"), ("ceb", "River Nile"), ("da", "Nilen"), ("de", "Nahr an-Nil"), ("el", "Ποταμός Νείλος"), ("en", "River Nile"), ("es", "Río Nilo"), ("fi", "River Nile"), ("fr", "Nil"), ("gu", "રિવર નાઇલ"), ("hi", "नील नदी"), ("id", "Sungai Nil"), ("it", "Nilo"), ("ja", "ナイル川州"), ("ka", "ნილოსის შტატი"), ("kn", "ನೈಲ\u{ccd} ನದ\u{cbf}"), ("ko", "나일 주"), ("lt", "Nilo Upės vilajetas"), ("lv", "Nīlas vilājs"), ("mr", "रिव\u{94d}हर नाईल"), ("ms", "River Nile"), ("nb", "Nil elven"), ("nl", "Nahr-an-Nil"), ("no", "Nil elven"), ("pl", "Nil"), ("pt", "Rio Nilo"), ("ro", "Statul Râul Nil"), ("ru", "Нил"), ("si", "නය\u{dd2}ල\u{dca} ගංග\u{dcf}ව"), ("sv", "Nahr an-Nil"), ("sw", "Mto Nile"), ("ta", "ரிவேர\u{bcd} நில\u{bcd}"), ("te", "ర\u{c3f}వర\u{c4d} న\u{c48}ల\u{c4d}"), ("th", "ร\u{e34}เวอร\u{e4c}ไนล\u{e4c}"), ("tr", "Nil Nehri Eyaleti"), ("uk", "Ніл"), ("ur", "دریائے نیل (ریاست)"), ("vi", "Sông Nin"), ("zh", "尼羅省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NW",
                    Subdivision{
                        name: "NW",
                        country_alpha2: Alpha2::SD,
                        code: "NW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.2403881), longitude: Some(32.5372741), max_latitude: Some(15.250874), min_latitude: Some(11.9465411), max_longitude: Some(33.2549571), min_longitude: Some(31.525822)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "النيل الأبيض"), ("bg", "Бели Нил"), ("bn", "হোয\u{9bc}\u{9be}ইট নিল"), ("ccp", "𑄘\u{1112a}𑄛\u{11134} 𑄚\u{1112d}𑄣\u{11134}"), ("ceb", "White Nile"), ("da", "Hvide Nil"), ("de", "An-Nil al-abyad"), ("el", "Λευκός Νείλος"), ("en", "White Nile"), ("es", "Nilo Blanco"), ("fa", "نیل سفید"), ("fi", "White Nile"), ("fr", "Nil Blanc"), ("gu", "વ\u{acd}હાઇટ નીલ"), ("hi", "सफ\u{93c}\u{947}द नील"), ("id", "Nil Putih"), ("it", "Nilo Bianco"), ("ja", "白ナイル州"), ("ka", "თეთრი ნილოსის შტატი"), ("kn", "ವೈಟ\u{ccd} ನೈಲ\u{ccd}"), ("ko", "백나일 주"), ("lt", "Baltojo Nilo vilajetas"), ("lv", "Baltās Nīlas vilājs"), ("mr", "व\u{94d}हाईट नील"), ("ms", "White Nile"), ("nb", "Hvite nil"), ("nl", "An-Nil-al-Abyad"), ("no", "Hvite nil"), ("pl", "Nil Biały"), ("pt", "Nilo Branco"), ("ro", "Statul Nilul Alb"), ("ru", "Белый Нил"), ("si", "ස\u{dd4}ද\u{dd4} නය\u{dd2}ල\u{dca}"), ("sv", "An-Nil al-Abyad"), ("sw", "Nile nyeupe"), ("ta", "வைட\u{bcd} நிலே"), ("te", "వ\u{c48}ట\u{c4d} న\u{c48}ల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกต-ดาร\u{e4c}มอร\u{e4c}"), ("tr", "Beyaz Nil Eyaleti"), ("uk", "Білий Ніл"), ("ur", "نیل ابیض (ریاست)"), ("vi", "Nin Trắng"), ("zh", "白尼罗省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RS",
                    Subdivision{
                        name: "RS",
                        country_alpha2: Alpha2::SD,
                        code: "RS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.4556063), longitude: Some(35.2148469), max_latitude: Some(22.0063784), min_latitude: Some(16.9985761), max_longitude: Some(38.580036), min_longitude: Some(33.2534589)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "البحر الأحمر"), ("bg", "Червено море"), ("bn", "লোহিত স\u{9be}গর"), ("ca", "Estat de la Mar Roja"), ("ccp", "𑄢𑄬𑄖\u{11134} 𑄥\u{11128}"), ("ceb", "Red Sea (estado)"), ("da", "Røde Hav"), ("de", "Al-Bahr al-ahmar"), ("el", "Ερυθρά Θάλασσα"), ("en", "Red Sea"), ("es", "Mar Rojo"), ("et", "Al-Baḩr al-Aḩmari provints"), ("fa", "دریای سرخ (استان سودان)"), ("fi", "Al-Bahr al-Ahmar"), ("fr", "Mer Rouge"), ("gu", "ર\u{ac7}ડ સી"), ("he", "אל-בחר אל-אחמר"), ("hi", "लाल सागर"), ("id", "Laut Merah"), ("it", "Mar Rosso"), ("ja", "紅海州"), ("ka", "წითელი ზღვის შტატი"), ("kn", "ಕ\u{cc6}ಂಪು ಸಮುದ\u{ccd}ರ"), ("ko", "홍해 주"), ("lt", "Raudonosios Jūros vilajetas"), ("lv", "Sarkanā jūra"), ("mr", "र\u{947}ड सी"), ("ms", "Laut Merah"), ("nb", "Rød sjøen"), ("nl", "Al-Bahr-al-Ahmar"), ("no", "Rød sjøen"), ("pl", "Prowincja Morza Czerwonego"), ("pt", "Mar Vermelho"), ("ro", "Statul Marea Roșie"), ("ru", "Красное море"), ("si", "රත\u{dd4} ම\u{dd4}හ\u{dd4}ද"), ("sv", "Al-Bahr al-Ahmar"), ("sw", "Bahari ya Shamu"), ("ta", "ரெட\u{bcd} ஸ\u{bc0}"), ("te", "ఎర\u{c4d}ర సముద\u{c4d}రం"), ("th", "ทะเลแดง"), ("tr", "Kızıl Deniz Eyaleti"), ("uk", "Червоне море"), ("ur", "بحیرہ احمر (ریاست)"), ("vi", "Biển Đỏ"), ("zh", "紅海省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::SD,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.0317108), longitude: Some(33.9750018), max_latitude: Some(14.106393), min_latitude: Some(11.7343758), max_longitude: Some(35.70768), min_longitude: Some(32.924994)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ስናር"), ("ar", "سنار"), ("bg", "Сенар"), ("bn", "সেন\u{9be}র"), ("ccp", "𑄥𑄬𑄚𑄢\u{11134}"), ("ceb", "Sinnar State"), ("da", "Sennar"), ("de", "Sannar"), ("el", "Σένναρ"), ("en", "Sennar"), ("es", "Sennar"), ("fa", "سنار"), ("fi", "Sennar"), ("fr", "Sannar"), ("gu", "સ\u{ac7}નાર"), ("hi", "स\u{947}नणार"), ("id", "Sannar"), ("it", "Sennar"), ("ja", "センナール州"), ("ka", "სენარის შტატი"), ("kn", "ಸ\u{cc6}ನ\u{ccd}ನರ\u{ccd}"), ("ko", "센나르 주"), ("lt", "Senaras"), ("lv", "Senara"), ("mr", "स\u{947}न\u{94d}नार"), ("ms", "Sennar"), ("nb", "Sennar"), ("nl", "Sennar"), ("no", "Sennar"), ("pl", "Sannar"), ("pt", "Sennar"), ("ro", "Statul Sennar"), ("ru", "Сеннар"), ("si", "සෙන\u{dca}නර\u{dca}"), ("sv", "Sennar"), ("ta", "சென\u{bcd}ன\u{bbe}ர\u{bcd}"), ("te", "స\u{c46}న\u{c4d}న\u{c3e}ర\u{c4d}"), ("th", "ซานนาร\u{e4c}"), ("tr", "Sennar Eyaleti"), ("uk", "Сеннар"), ("ur", "سنار (ریاست)"), ("vi", "Sennar"), ("zh", "森纳尔省")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "sd")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SD,
        alpha3: Alpha3::SDN,
        address_format: None,
        continent: Continent::Africa,
        country_code: 249,
        currency_code: CurrencyCode::SDG,
        gec: Some(GEC::SU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::SUD),
        iso_long_name: "The Republic of the Sudan",
        iso_short_name: "Sudan",
        official_language_list: ["ar", "en"].to_vec(),
        spoken_language_list: ["ar", "en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Sudanese"),
        number: "729",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernAfrica),
        un_locode: "SD",
        unofficial_name_list: ["Sudan", "السودان", "Soudan", "Sudán", "スーダン", "Soedan"]
            .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Sudan"),
            ("af", "Soedan"),
            ("ak", "Sudan"),
            ("am", "ሱ፣ን"),
            ("an", "Sudan"),
            ("ar", "السودان"),
            ("as", "ছ\u{9c1}দ\u{9be}ন"),
            ("ay", "Sudan"),
            ("az", "Sudan"),
            ("ba", "Sudan"),
            ("be", "Судан"),
            ("bg", "Судан"),
            ("bi", "Sudan"),
            ("bn", "স\u{9c1}দ\u{9be}ন"),
            ("bn_IN", "স\u{9c1}দ\u{9be}ন"),
            ("br", "Soudan"),
            ("bs", "Sudan"),
            ("ca", "Sudan"),
            ("ce", "Судан"),
            ("ch", "Sudan"),
            ("cs", "Súdán"),
            ("cv", "Судан"),
            ("cy", "Y Swdan"),
            ("da", "Sudan"),
            ("de", "Sudan"),
            ("dv", "ސ\u{7ab}ދ\u{7a7}ނ\u{7b0}"),
            ("dz", "ས\u{f74}་ད\u{f71}ན།"),
            ("ee", "Sudan"),
            ("el", "Σουδάν"),
            ("en", "Sudan"),
            ("eo", "Sudano"),
            ("es", "Sudán"),
            ("et", "Sudaan"),
            ("eu", "Sudan"),
            ("fa", "سودان"),
            ("ff", "Sudan"),
            ("fi", "Sudan"),
            ("fo", "Sudan"),
            ("fr", "Soudan"),
            ("fy", "Sûdan"),
            ("ga", "An tSúdáin"),
            ("gl", "Sudán"),
            ("gn", "Sudan"),
            ("gu", "સ\u{ac1}દાન"),
            ("gv", "Yn Toodaan"),
            ("ha", "Sudan"),
            ("he", "סודאן"),
            ("hi", "स\u{942}डान"),
            ("hr", "Sudan"),
            ("ht", "Soudan"),
            ("hu", "Szudán"),
            ("hy", "Սուդան"),
            ("ia", "Sudan"),
            ("id", "Sudan"),
            ("io", "Sudan"),
            ("is", "Súdan"),
            ("it", "Sudan"),
            ("iu", "Sudan"),
            ("ja", "スーダン"),
            ("ka", "სუდანი"),
            ("ki", "Sũdana"),
            ("kk", "Судан"),
            ("kl", "Sudan"),
            ("km", "ស\u{17ca}\u{17bc}ដង\u{17cb}"),
            ("kn", "ಸ\u{cc2}ಡಾನ\u{ccd}"),
            ("ko", "수단"),
            ("ku", "Sûdan"),
            ("kv", "Sudan"),
            ("kw", "Soudan"),
            ("ky", "Судан"),
            ("lo", "Sudan"),
            ("lt", "Sudanas"),
            ("lv", "Sudāna"),
            ("mi", "Hūtāne"),
            ("mk", "Судан"),
            ("ml", "സ\u{d41}ഡ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Судан"),
            ("mr", "स\u{941}दान"),
            ("ms", "Sudan"),
            ("mt", "Sudan"),
            (
                "my",
                "ဆ\u{1030}ဒန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tudan"),
            ("nb", "Sudan"),
            ("ne", "स\u{941}डान"),
            ("nl", "Soedan"),
            ("nn", "Sudan"),
            ("nv", "Ásáí Dineʼé Daalzhiní Bikéyah"),
            ("oc", "Sodan"),
            ("or", "ସ\u{b42}ଡ\u{b3e}ନ"),
            ("pa", "ਸ\u{a42}ਡਾਨ"),
            ("pi", "स\u{942}डान"),
            ("pl", "Sudan"),
            ("ps", "سوډان"),
            ("pt", "Sudão"),
            ("pt_BR", "Sudão"),
            ("ro", "Sudan"),
            ("ru", "Судан"),
            ("rw", "Sudani"),
            ("sc", "Sudàn"),
            ("sd", "Sudan"),
            ("si", "ස\u{dd4}ඩ\u{dcf}නය"),
            ("sk", "Sudán"),
            ("sl", "Sudan"),
            ("so", "Sudaan"),
            ("sq", "Sudan"),
            ("sr", "Судан"),
            ("sv", "Sudan"),
            ("sw", "Sudan"),
            ("ta", "சூட\u{bbe}ன\u{bcd}"),
            ("te", "సుడ\u{c3e}న\u{c4d}"),
            ("tg", "Судон"),
            ("th", "ซ\u{e39}ดาน"),
            ("ti", "ሱዳን"),
            ("tk", "Sudan"),
            ("tl", "Sudan"),
            ("tr", "Sudan"),
            ("tt", "Судан"),
            ("ug", "سۇدان"),
            ("uk", "Судан"),
            ("ur", "سوڈان"),
            ("uz", "Sudan"),
            ("ve", "Sudan"),
            ("vi", "Xu-đanh"),
            ("wa", "Soudan"),
            ("wo", "Suudaan"),
            ("xh", "Sudan"),
            ("yo", "Sudan"),
            ("zh_CN", "苏丹"),
            ("zh_HK", "蘇丹"),
            ("zh_TW", "蘇丹"),
            ("zu", "ISudan"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
