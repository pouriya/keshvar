// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Islamic Republic of Pakistan

#[cfg(all(feature = "pk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PK;
    pub const ALPHA3: Alpha3 = Alpha3::PAK;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 92;
    pub const CURRENCY_CODE: &str = "PKR";
    pub const GEC: Option<GEC> = Some(GEC::PK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("PAK");
    pub const ISO_SHORT_NAME: &str = "Pakistan";
    pub const ISO_LONG_NAME: &str = "The Islamic Republic of Pakistan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "ur"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "ur"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Pakistani");
    pub const NUMBER: &str = "586";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "PK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Pakistan", "Paquistán", "パキスタン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Pakistan"),
        ("af", "Pakistan"),
        ("ak", "Pakistan"),
        ("am", "ፓኪስታን"),
        ("an", "Pakistan"),
        ("ar", "باكستان"),
        ("as", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
        ("ay", "Pakistan"),
        ("az", "Pakistan"),
        ("ba", "Pakistan"),
        ("be", "Пакістан"),
        ("bg", "Пакистан"),
        ("bi", "Pakistan"),
        ("bn", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
        ("br", "Pakistan"),
        ("bs", "Pakistan"),
        ("ca", "Pakistan"),
        ("ce", "Пакистан"),
        ("ch", "Pakistan"),
        ("cs", "Pákistán"),
        ("cv", "Пакистан"),
        ("cy", "Pacistan"),
        ("da", "Pakistan"),
        ("de", "Pakistan"),
        ("dv", "ޕ\u{7a7}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
        ("dz", "པ་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Pakistan"),
        ("el", "Πακιστάν"),
        ("en", "Pakistan"),
        ("eo", "Pakistano"),
        ("es", "Pakistán"),
        ("et", "Pakistan"),
        ("eu", "Pakistan"),
        ("fa", "پاکستان"),
        ("ff", "Pakistan"),
        ("fi", "Pakistan"),
        ("fo", "Pakistan"),
        ("fr", "Pakistan"),
        ("fy", "Pakistan"),
        ("ga", "An Phacastáin"),
        ("gl", "Paquistán"),
        ("gn", "Pakistan"),
        ("gu", "પાકિસ\u{acd}તાન"),
        ("gv", "Yn Phakistaan"),
        ("ha", "Pakistan"),
        ("he", "פקיסטן"),
        ("hi", "पाकिस\u{94d}तान"),
        ("hr", "Pakistan"),
        ("ht", "Pakistan"),
        ("hu", "Pakisztán"),
        ("hy", "Պակիստան"),
        ("ia", "Pakistan"),
        ("id", "Pakistan"),
        ("io", "Pakistan"),
        ("is", "Pakistan"),
        ("it", "Pakistan"),
        ("iu", "ᐸᑭᔅᑕᓐ"),
        ("ja", "パキスタン"),
        ("ka", "პაკისტანი"),
        ("ki", "Pakistan"),
        ("kk", "Пәкістан"),
        ("kl", "Pakistan"),
        ("km", "ប\u{17c9}ាគ\u{17b8}ស\u{17d2}ថាន"),
        ("kn", "ಪಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
        ("ko", "파키스탄"),
        ("ku", "Pakistan"),
        ("kv", "Пакистан"),
        ("kw", "Pakistan"),
        ("ky", "Пакистан"),
        ("lo", "Pakistan"),
        ("lt", "Pakistanas"),
        ("lv", "Pakistāna"),
        ("mi", "Pakitāne"),
        ("mk", "Пакистан"),
        ("ml", "പ\u{d3e}കിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Пакистан"),
        ("mr", "पाकिस\u{94d}तान"),
        ("ms", "Pakistan"),
        ("mt", "Pakistan"),
        (
            "my",
            "ပါကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Pakistan"),
        ("nb", "Pakistan"),
        ("ne", "पाकिस\u{94d}तान"),
        ("nl", "Pakistan"),
        ("nn", "Pakistan"),
        ("nv", "Eʼeʼaahjí Naakaii Dootłʼizhí Bikéyah"),
        ("oc", "Paquistan"),
        ("or", "ପ\u{b3e}କ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਪਾਕਿਸਤਾਨ"),
        ("pi", "पाकिस\u{94d}तान"),
        ("pl", "Pakistan"),
        ("ps", "پاکستان"),
        ("pt", "Paquistão"),
        ("pt_BR", "Paquistão"),
        ("ro", "Pakistan"),
        ("ru", "Пакистан"),
        ("rw", "Pakisitani"),
        ("sc", "Pàkistan"),
        ("sd", "پاڪستان"),
        ("si", "පක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
        ("sk", "Pakistan"),
        ("sl", "Pakistan"),
        ("so", "Bakistaan"),
        ("sq", "Pakistan"),
        ("sr", "Пакистан"),
        ("sv", "Pakistan"),
        ("sw", "Pakistan"),
        ("ta", "ப\u{bbe}கிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
        ("te", "ప\u{c3e}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Покистон"),
        ("th", "ปาก\u{e35}สถาน"),
        ("ti", "ፓኪስታን"),
        ("tk", "Pakystan"),
        ("tl", "Pakistan"),
        ("tr", "Pakistan"),
        ("tt", "Пәкстан"),
        ("ug", "پاكىستان"),
        ("uk", "Пакистан"),
        ("ur", "پاکستان"),
        ("uz", "Pokiston"),
        ("ve", "Pakistan"),
        ("vi", "Pa-ki-xợ-thănh"),
        ("wa", "Pakistan"),
        ("wo", "Pakistaan"),
        ("xh", "Pakistan"),
        ("yo", "Pakístàn"),
        ("zh_CN", "巴基斯坦"),
        ("zh_HK", "巴基斯坦"),
        ("zh_TW", "巴基斯坦"),
        ("zu", "IPakistani"),
    ];
    #[cfg(all(feature = "pk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 30.375321;
        pub const LONGITUDE: f64 = 69.34511599999999;
        pub const MAX_LATITUDE: f64 = 37.0841069;
        pub const MAX_LONGITUDE: f64 = 77.8231711;
        pub const MIN_LATITUDE: f64 = 23.6344999;
        pub const MIN_LONGITUDE: f64 = 60.8729721;
        pub const NORTHEAST_LATITUDE: f64 = 37.0841069;
        pub const NORTHEAST_LONGITUDE: f64 = 77.8231711;
        pub const SOUTHWEST_LATITUDE: f64 = 23.6344999;
        pub const SOUTHWEST_LONGITUDE: f64 = 60.8729721;
    }
}
#[cfg(all(feature = "pk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 30.375321,
            longitude: 69.34511599999999,
            max_latitude: 37.0841069,
            max_longitude: 77.8231711,
            min_latitude: 23.6344999,
            min_longitude: 60.8729721,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 37.0841069,
                    longitude: 77.8231711,
                },
                southwest: CountryGeoBound {
                    latitude: 23.6344999,
                    longitude: 60.8729721,
                },
            },
        }
    }
}

#[cfg(all(feature = "pk", feature = "subdivisions"))]
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
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::PK,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.4907332), longitude: Some(65.0957792), max_latitude: Some(32.064602), min_latitude: Some(24.890663), max_longitude: Some(70.259517), min_longitude: Some(60.87859700000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلوشستان"), ("az", "Bəlucistan"), ("be", "Правінцыя Белуджыстан"), ("bg", "Белуджистан"), ("bn", "বেল\u{9c1}চিস\u{9cd}ত\u{9be}ন"), ("ca", "Balutxistan Oriental"), ("ccp", "𑄝𑄣\u{1112a}𑄌\u{11128}𑄌\u{11134}𑄑𑄚\u{11134}"), ("cs", "Balúčistán"), ("da", "Baluchistan"), ("de", "Belutschistan"), ("el", "Μπαλοτσιστάν"), ("en", "Balochistan"), ("es", "Baluchistán"), ("et", "Belutšistani provints"), ("eu", "Balutxistan"), ("fa", "ایالت بلوچستان پاکستان"), ("fi", "Balochistan"), ("fr", "Baloutchistan"), ("ga", "An Bhalúcastáin"), ("gl", "Baluchistán"), ("gu", "બલ\u{ac1}ચિસ\u{acd}તાન"), ("he", "בלוצ׳יסטן"), ("hi", "बल\u{942}चिस\u{94d}तान"), ("hu", "Beludzsisztán"), ("id", "Balochistan"), ("it", "Belucistan"), ("ja", "バローチスターン州"), ("ka", "ბელუჯისტანი"), ("kn", "ಬಲ\u{cc2}ಚ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"), ("ko", "발루치스탄 주"), ("lt", "Beludžistanas"), ("lv", "Beludžistāna"), ("ml", "ബല\u{d42}ചിസ\u{d4d}ഥ\u{d3e}ൻ"), ("mn", "Балучистан"), ("mr", "बल\u{941}चिस\u{94d}तान"), ("ms", "Balochistan"), ("nb", "Balutsjistan"), ("ne", "बल\u{941}चिस\u{94d}तान"), ("nl", "Beloetsjistan"), ("no", "Balutsjistan"), ("pa", "ਬਲ\u{a4b}ਚਿਸਤਾਨ"), ("pl", "Beludżystan"), ("ps", "بلوچستان"), ("pt", "Baluchistão"), ("ro", "Belucistan"), ("ru", "Белуджистан"), ("sd", "بلوچستان"), ("si", "බලොස\u{dca}ච\u{dd2}ස\u{dca}ථ\u{dcf}නය"), ("sr", "Белуџистан"), ("sr_Latn", "Beludžistan"), ("sv", "Baluchistan"), ("ta", "பலூசிஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "బలూచ\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "บาลอนคร\u{e34}สต\u{e31}น"), ("tr", "Belucistan Eyaleti"), ("uk", "Белуджистан"), ("ur", "بلوچستان"), ("uz", "Balujiston"), ("vi", "Balochistan"), ("yue", "俾路支省"), ("yue_Hans", "俾路支省"), ("zh", "俾路支省")]),
                        unofficial_name_list: ["Baluchistan (en)"].to_vec(),
                    }
                ),
                (
                    "GB",
                    Subdivision{
                        name: "GB",
                        country_alpha2: Alpha2::PK,
                        code: "GB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::PakistanAdministeredArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غلغت-بلتستان"), ("az", "Gilgit-Baltistan"), ("be", "Гілгіт-Балтыстан"), ("bg", "Северна провинция"), ("bn", "গিলগিত-ব\u{9be}লতিস\u{9cd}ত\u{9be}ন"), ("ca", "Gilgit-Baltistan"), ("ccp", "𑄉\u{11128}𑄣\u{11134}𑄉\u{11128}𑄖\u{11134}-𑄝𑄣\u{11134}𑄑\u{11128}𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Northern Areas"), ("cs", "Gilgit - Baltistán"), ("cy", "Ardaloedd y Gogledd"), ("da", "Gilgit-Baltistan"), ("de", "Gilgit-Baltistan"), ("el", "Γκιλγκίτ-Μπαλτιστάν"), ("en", "Gilgit-Baltistan"), ("es", "Gilgit-Baltistán"), ("et", "Põhjaalad"), ("eu", "Iparraldeko Eremuak"), ("fa", "گلگت-بلتستان"), ("fi", "Gilgit-Baltistan"), ("fr", "Gilgit-Baltistan"), ("gu", "ગિલગીટ-બાલ\u{acd}તિસ\u{acd}તાન"), ("he", "גילגיט-בלטיסטן"), ("hi", "गिलगित-बाल\u{94d}तिस\u{94d}तान"), ("id", "Gilgit–Baltistan"), ("it", "Gilgit-Baltistan"), ("ja", "ギルギット・バルティスタン州"), ("ka", "გილგის-ბალტისტანი"), ("kn", "ಗ\u{cbf}ಲ\u{ccd}ಗ\u{cbf}ಟ\u{ccd}-ಬಾಲ\u{ccd}ಟ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"), ("ko", "길기트발티스탄 주"), ("lt", "Gilgitas-Baltistanas"), ("lv", "Gilgita-Baltistāna"), ("ml", "ഗിൽഗിറ\u{d4d}റ\u{d4d}-ബ\u{d3e}ൾട\u{d4d}ടിസ\u{d4d}ഥ\u{d3e}ൻ"), ("mr", "गिलगीट-बाल\u{94d}टिस\u{94d}तान"), ("ms", "Gilgit-Baltistan"), ("nb", "Gilgit-Baltistan"), ("ne", "गिल\u{94d}गित-बाल\u{94d}तिस\u{94d}तान"), ("nl", "Gilgit-Baltistan"), ("no", "Gilgit-Baltistan"), ("pa", "ਗਿਲਗਿਤ-ਬਾਲਤਿਸਤਾਨ"), ("pl", "Gilgit-Baltistan"), ("ps", "گېلگېت بالتستان"), ("pt", "Gilgit-Baltistão"), ("ru", "Гилгит-Балтистан"), ("sd", "گلگت بلتستان"), ("si", "ග\u{dd2}ල\u{dca}ග\u{dd2}ට\u{dca}-බල\u{dca}ට\u{dd2}ස\u{dca}ට\u{dcf}න\u{dca}"), ("sr", "Гилгит-Балтистан"), ("sr_Latn", "Gilgit-Baltistan"), ("sv", "Gilgit-Baltistan"), ("ta", "வடக\u{bcd}கு நிலங\u{bcd}கள\u{bcd}"), ("te", "గ\u{c3f}ల\u{c4d}జ\u{c3f}త\u{c4d}-బ\u{c3e}ల\u{c4d}ట\u{c3f}స\u{c4d}ట\u{c3e}న\u{c4d}"), ("th", "ก\u{e34}ลก\u{e34}ต บ\u{e31}ลท\u{e34}สสถาน"), ("tr", "Gilgit-Baltistan"), ("uk", "Гілгіт-Балтистан"), ("ur", "گلگت بلتستان"), ("vi", "Gilgit-Baltistan"), ("zh", "吉尔吉特-巴尔蒂斯坦")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "IS",
                    Subdivision{
                        name: "IS",
                        country_alpha2: Alpha2::PK,
                        code: "IS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.7293882), longitude: Some(73.0931461), max_latitude: Some(33.7566789), min_latitude: Some(33.5993931), max_longitude: Some(73.1771194), min_longitude: Some(72.7928352)}),
                        comments: None,
                        subdivision_type: SubdivisionType::FederalCapitalTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ناحية بايتختي اسلام\u{200c}آباد"), ("be", "Федэральная сталічная тэрыторыя"), ("bg", "Федерално управляеми племенни територии"), ("bn", "ইসল\u{9be}ম\u{9be}ব\u{9be}দ র\u{9be}জধ\u{9be}নী অঞ\u{9cd}চল"), ("ca", "Territori Capital Islamabad"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄣𑄟𑄝𑄖\u{11134}"), ("ceb", "Islāmābād Capital Territory"), ("de", "Hauptstadtterritorium Islamabad"), ("en", "Islamabad"), ("fa", "ناحیه پایتختی اسلام\u{200c}آباد"), ("fr", "Territoire fédéral d’Islamabad"), ("hi", "इस\u{94d}लामाबाद राजधानी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Teritorij glavnoga grada Islamabada"), ("id", "Wilayah Ibu Kota Islamabad"), ("it", "Territorio della capitale Islamabad"), ("ja", "イスラーマーバード首都圏"), ("ka", "ისლამაბადის ფედერალური ტერიტორია"), ("ko", "이슬라마바드 수도권"), ("ml", "ഇസ\u{d4d}ല\u{d3e}മബ\u{d3e}ദ\u{d4d} തലസ\u{d4d}ഥ\u{d3e}ന പ\u{d4d}രദേശം"), ("nb", "Islamabad hovedstadsterritorium"), ("nl", "Hoofdstedelijk Territorium Islamabad"), ("no", "Islamabad hovedstadsterritorium"), ("pl", "Stołeczne Terytorium Islamabadu"), ("pt", "Território da Capital Islamabad"), ("ru", "Федеральная столичная территория"), ("ta", "இசுல\u{bbe}ம\u{bbe}ப\u{bbe}த\u{bcd} தலைநகர ஆட\u{bcd}புலம\u{bcd}"), ("th", "ด\u{e34}นแดนนครหลวงอ\u{e34}สลามาบ\u{e31}ด"), ("tr", "İslamabad Başkent Bölgesi"), ("uk", "Федеральна столична територія"), ("ur", "وفاقی دارالحکومت"), ("vi", "Lãnh thổ Thủ đô Islamabad"), ("zh", "伊斯兰堡首都区")]),
                        unofficial_name_list: ["Islamabad"].to_vec(),
                    }
                ),
                (
                    "JK",
                    Subdivision{
                        name: "JK",
                        country_alpha2: Alpha2::PK,
                        code: "JK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.8866588), longitude: Some(73.93598209999999), max_latitude: Some(35.1311), min_latitude: Some(32.766542), max_longitude: Some(75.2642401), min_longitude: Some(73.394989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::PakistanAdministeredArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آزاد كشمير"), ("az", "Azad Kəşmir"), ("be", "Азад Кашмір"), ("bg", "Азад Кашмир"), ("bn", "আজ\u{9be}দ ক\u{9be}শ\u{9cd}মীর"), ("ca", "Azad Kashmir"), ("ccp", "𑄃𑄎𑄖\u{11134} 𑄇𑄌\u{11134}𑄟\u{11128}𑄢\u{11134}"), ("ceb", "Azad Kashmir"), ("cs", "Ázád Kašmír"), ("da", "Azad Kashmir"), ("de", "Asad Jammu und Kaschmir"), ("el", "Αζάντ Κασμίρ"), ("en", "Azad Kashmir"), ("es", "Azad Cachemira"), ("eu", "Azad Kaxmir"), ("fa", "کشمیر آزاد"), ("fi", "Azad Kashmir"), ("fr", "Azad Cachemire"), ("gu", "આઝાદ કાશ\u{acd}મીર"), ("he", "אזאד קשמיר"), ("hi", "आज\u{93c}ाद कश\u{94d}मीर"), ("hy", "Ազադ Քաշմիր"), ("id", "Azad Kashmir"), ("it", "Azad Kashmir"), ("ja", "アザド・カシミール"), ("ka", "აზად-ქაშმირი"), ("kn", "ಆಜಾದ\u{ccd} ಕಾಶ\u{ccd}ಮೀರ"), ("ko", "아자드 카슈미르 주"), ("ky", "Азад Кашмир"), ("lt", "Laisvasis Kašmyras"), ("lv", "Azadkašmīra"), ("ml", "ആസ\u{d3e}ദ\u{d4d} കശ\u{d4d}മീർ"), ("mr", "पाकव\u{94d}याप\u{94d}त काश\u{94d}मीर"), ("ms", "Azad Kashmir"), ("nb", "Azad Kashmir"), ("ne", "आजाद कश\u{94d}मीर"), ("nl", "Azad Kasjmir"), ("no", "Azad Kashmir"), ("pa", "ਅਜ\u{a3c}ਾਦ ਕਸ\u{a3c}ਮੀਰ"), ("pl", "Azad Dżammu i Kaszmir"), ("ps", "آزاد کشمير"), ("pt", "Caxemira Livre"), ("ru", "Азад Кашмир"), ("sd", "آزاد ڪشمير"), ("si", "අස\u{dcf}ද\u{dca} ක\u{dcf}ශ\u{dca}ම\u{dd3}ර\u{dca}"), ("sr", "Азад Кашмир"), ("sr_Latn", "Azad Kašmir"), ("sv", "Azad Kashmir"), ("ta", "ஆச\u{bbe}த\u{bcd} க\u{bbe}ஷ\u{bcd}ம\u{bc0}ர\u{bcd}"), ("te", "ఆజ\u{c3e}ద\u{c4d} కశ\u{c4d}మ\u{c40}ర\u{c4d}"), ("th", "อาซาดแคชเม\u{e35}ยร\u{e4c}"), ("tr", "Azad Keşmir"), ("uk", "Азад Кашмір"), ("ur", "آزاد کشمیر"), ("vi", "Azad Kashmir"), ("zh", "自由克什米爾")]),
                        unofficial_name_list: ["Azad Kashmir"].to_vec(),
                    }
                ),
                (
                    "KP",
                    Subdivision{
                        name: "KP",
                        country_alpha2: Alpha2::PK,
                        code: "KP",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خیبر بختونخوا"), ("be", "Хайбер-Пахтунхва"), ("bg", "Северозападна погранична провинция"), ("bn", "খ\u{9be}ইব\u{9be}র প\u{9be}খত\u{9c1}নখ\u{9be}ওয\u{9bc}\u{9be}"), ("ca", "Província de la Frontera del Nord-oest"), ("ccp", "𑄈\u{1112d}𑄝𑄬𑄢\u{11134} 𑄛𑄇\u{11134}𑄑\u{1112a}𑄚\u{11134}𑄈\u{11127}𑄤"), ("ceb", "Khyber Pakhtunkhwa Province"), ("cs", "Chajbar Paštúnchwá"), ("cy", "Khyber Pakhtunkhwa"), ("da", "Khyber Pakhtunkhwa"), ("de", "Khyber Pakhtunkhwa"), ("en", "Khyber Pakhtunkhwa"), ("es", "Jaiber Pastunjuá"), ("et", "Loodepiiriprovints"), ("eu", "Khyber Pakhtunkhwa"), ("fa", "خیبر پختونخوا"), ("fi", "Khyber Pakhtunkhwa"), ("fr", "Khyber Pakhtunkhwa"), ("he", "ח׳ייבר פח׳טונח׳ווה"), ("hi", "ख\u{93c}\u{948}बर-पख\u{93c}\u{94d}त\u{942}नख\u{93c}\u{94d}वा"), ("hu", "Hajber-Pahtunhva"), ("id", "Khyber Pakhtunkhwa"), ("it", "Khyber Pakhtunkhwa"), ("ja", "カイバル・パクトゥンクワ州"), ("ka", "ხაიბერ-პახტუნხვა"), ("ko", "카이베르파크툰크와 주"), ("lt", "Chaiber Pachtunchva"), ("lv", "Haibara Pahtūnhva"), ("ml", "ഖൈബർ പഖ\u{d4d}ത\u{d41}ൻഖ\u{d4d}വ"), ("mr", "ख\u{948}बर पख\u{94d}त\u{942}नख\u{94d}वा"), ("ms", "Khyber Pakhtunkhwa"), ("nb", "Khyber Pakhtunkhwa"), ("ne", "ख\u{948}बर पख\u{94d}त\u{941}नख\u{94d}वा"), ("nl", "Khyber-Pakhtunkhwa"), ("no", "Khyber Pakhtunkhwa"), ("pa", "ਖ\u{a3c}\u{a48}ਬਰ ਪਖ\u{a3c}ਤ\u{a4b}ਨਖ\u{a3c}ਵਾ"), ("pl", "Chajber Pachtunchwa"), ("ps", "خيبر پښتونخوا"), ("pt", "Khyber Pakhtunkhwa"), ("ru", "Хайбер-Пахтунхва"), ("sd", "خيبر پختونخوا"), ("sr", "Хајбер-Пахтунва"), ("sr_Latn", "Hajber-Pahtunva"), ("sv", "Khyber Pukhtunkhwa"), ("ta", "வடமேற\u{bcd}கு எல\u{bcd}லைப\u{bcd}புற ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("th", "แคว\u{e49}นไคเบอร\u{e4c}ป\u{e31}คต\u{e39}นควา"), ("tr", "Kuzeybatı Sınır Eyaleti"), ("uk", "Хайбер-Пахтунхва"), ("ur", "خیبر پختونخوا"), ("uz", "Shimoliy-gʻarbiy chegara viloyati"), ("vi", "Khyber Pakhtunkhwa"), ("yue", "西北邊境省"), ("yue_Hans", "西北边境省"), ("zh", "开伯尔－普赫图赫瓦省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PB",
                    Subdivision{
                        name: "PB",
                        country_alpha2: Alpha2::PK,
                        code: "PB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.1704063), longitude: Some(72.7097161), max_latitude: Some(34.016032), min_latitude: Some(27.705111), max_longitude: Some(75.3818661), min_longitude: Some(69.33357699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "البنجاب"), ("az", "Pəncab"), ("be", "Пенджаб"), ("bg", "Пенджаб"), ("bn", "প\u{9be}ঞ\u{9cd}জ\u{9be}ব, প\u{9be}কিস\u{9cd}ত\u{9be}ন"), ("ca", "Panjab"), ("ccp", "𑄛𑄚\u{11134}𑄎𑄛\u{11134}"), ("ceb", "Punjab"), ("cs", "Paňdžáb"), ("cy", "Punjab"), ("da", "Punjab"), ("de", "Punjab"), ("el", "Παντζάμπ"), ("en", "Punjab"), ("es", "Punyab"), ("et", "Pandžab"), ("eu", "Punjab"), ("fa", "پنجاب"), ("fi", "Punjab"), ("fr", "Pendjab"), ("gl", "Punjab"), ("gu", "પ\u{a82}જાબ"), ("hi", "प\u{902}जाब"), ("hu", "Pandzsáb"), ("hy", "Փենջաբ"), ("id", "Punjab"), ("it", "Punjab"), ("ja", "パンジャーブ州"), ("ka", "პენჯაბი"), ("kn", "ಪಂಜಾಬ\u{ccd}"), ("ko", "펀자브 주"), ("lt", "Pandžabas"), ("lv", "Pendžāba"), ("ml", "പഞ\u{d4d}ച\u{d3e}ബ\u{d4d}"), ("mn", "Панжаб"), ("mr", "प\u{902}जाब"), ("ms", "Punjab"), ("nb", "Punjab"), ("ne", "प\u{902}जाब (पाकिस\u{94d}तान)"), ("nl", "Punjab"), ("no", "Punjab"), ("or", "ପଞ\u{b4d}ଜ\u{b3e}ବ"), ("pa", "ਪ\u{a70}ਜਾਬ"), ("pl", "Pendżab"), ("ps", "پنجاب"), ("pt", "Punjab"), ("ro", "Punjab"), ("ru", "Пенджаб"), ("si", "පන\u{dca}ජ\u{dcf}බ\u{dca}"), ("sk", "Pandžáb (Pakistan)"), ("sq", "Panxhab, Pakistan"), ("sr", "Панџаб"), ("sr_Latn", "Pandžab"), ("sv", "Punjab"), ("ta", "பஞ\u{bcd}ச\u{bbe}ப\u{bcd}"), ("te", "పంజ\u{c3e}బ\u{c4d}"), ("th", "แคว\u{e49}นป\u{e31}ญจาบ"), ("tr", "Pencap Eyaleti"), ("uk", "Пенджаб"), ("ur", "پنجاب"), ("vi", "Punjab"), ("yue", "頻渣別省"), ("yue_Hans", "频渣别省"), ("zh", "旁遮普省")]),
                        unofficial_name_list: ["Punjab"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::PK,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.8943018), longitude: Some(68.52471489999999), max_latitude: Some(28.5015481), min_latitude: Some(23.6946945), max_longitude: Some(71.12440509999999), min_longitude: Some(66.6546894)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "السند"), ("be", "Правінцыя Сінд"), ("bg", "Синд"), ("bn", "সিন\u{9cd}ধ\u{9c1} প\u{9cd}রদেশ"), ("ca", "Sind"), ("ccp", "𑄥\u{11128}𑄚\u{11134}𑄙\u{11134}"), ("ceb", "Sindh"), ("cs", "Sindh"), ("da", "Sindh"), ("de", "Sindh"), ("el", "Σιντ"), ("en", "Sindh"), ("es", "Sind"), ("et", "Sindh"), ("eu", "Sindh"), ("fa", "استان سند"), ("fi", "Sindh"), ("fr", "Sind"), ("ga", "An tSind"), ("gl", "Sind"), ("gu", "સિ\u{a82}ધ"), ("he", "סינד"), ("hi", "सि\u{902}ध"), ("hr", "Sind"), ("hu", "Szindh"), ("hy", "Սինդ"), ("id", "Sindh"), ("is", "Sind"), ("it", "Sindh"), ("ja", "シンド州"), ("ka", "სინდის პროვინცია"), ("kn", "ಸ\u{cbf}ಂಧ\u{ccd}"), ("ko", "신드 주"), ("lt", "Sindas"), ("lv", "Sinda"), ("mk", "Синд"), ("ml", "സിന\u{d4d}ധ\u{d4d}"), ("mr", "सि\u{902}ध"), ("ms", "Sindh"), ("nb", "Sind"), ("ne", "सिन\u{94d}ध"), ("nl", "Sindh"), ("no", "Sind"), ("pa", "ਸਿ\u{a70}ਧ"), ("pl", "Sindh"), ("ps", "سند"), ("pt", "Sind"), ("ro", "Sindh"), ("ru", "Синд"), ("sd", "سنڌ"), ("si", "ස\u{dd2}න\u{dca}ධ\u{dca}"), ("sl", "Sindh"), ("so", "Sind"), ("sq", "Sind"), ("sr", "Синд"), ("sr_Latn", "Sind"), ("sv", "Sindh"), ("ta", "சிந\u{bcd}து ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}ంధ\u{c4d}"), ("th", "แคว\u{e49}นส\u{e34}นธ\u{e4c}"), ("tr", "Sind Eyaleti"), ("uk", "Сінд"), ("ur", "سندھ"), ("uz", "Sind"), ("vi", "Sindh"), ("yo", "Sindh"), ("yo_BJ", "Sindh"), ("yue", "信德省"), ("yue_Hans", "信德省"), ("zh", "信德省")]),
                        unofficial_name_list: ["Sind (en)"].to_vec(),
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
#[cfg(feature = "pk")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PK,
        alpha3: Alpha3::PAK,
        address_format: None,
        continent: Continent::Asia,
        country_code: 92,
        currency_code: "PKR",
        gec: Some(GEC::PK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("PAK"),
        iso_long_name: "The Islamic Republic of Pakistan",
        iso_short_name: "Pakistan",
        official_language_list: ["en", "ur"].to_vec(),
        spoken_language_list: ["en", "ur"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Pakistani"),
        number: "586",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "PK",
        unofficial_name_list: ["Pakistan", "Paquistán", "パキスタン"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Pakistan"),
            ("af", "Pakistan"),
            ("ak", "Pakistan"),
            ("am", "ፓኪስታን"),
            ("an", "Pakistan"),
            ("ar", "باكستان"),
            ("as", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
            ("ay", "Pakistan"),
            ("az", "Pakistan"),
            ("ba", "Pakistan"),
            ("be", "Пакістан"),
            ("bg", "Пакистан"),
            ("bi", "Pakistan"),
            ("bn", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
            ("bn_IN", "প\u{9be}কিস\u{9cd}ত\u{9be}ন"),
            ("br", "Pakistan"),
            ("bs", "Pakistan"),
            ("ca", "Pakistan"),
            ("ce", "Пакистан"),
            ("ch", "Pakistan"),
            ("cs", "Pákistán"),
            ("cv", "Пакистан"),
            ("cy", "Pacistan"),
            ("da", "Pakistan"),
            ("de", "Pakistan"),
            ("dv", "ޕ\u{7a7}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
            ("dz", "པ་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
            ("ee", "Pakistan"),
            ("el", "Πακιστάν"),
            ("en", "Pakistan"),
            ("eo", "Pakistano"),
            ("es", "Pakistán"),
            ("et", "Pakistan"),
            ("eu", "Pakistan"),
            ("fa", "پاکستان"),
            ("ff", "Pakistan"),
            ("fi", "Pakistan"),
            ("fo", "Pakistan"),
            ("fr", "Pakistan"),
            ("fy", "Pakistan"),
            ("ga", "An Phacastáin"),
            ("gl", "Paquistán"),
            ("gn", "Pakistan"),
            ("gu", "પાકિસ\u{acd}તાન"),
            ("gv", "Yn Phakistaan"),
            ("ha", "Pakistan"),
            ("he", "פקיסטן"),
            ("hi", "पाकिस\u{94d}तान"),
            ("hr", "Pakistan"),
            ("ht", "Pakistan"),
            ("hu", "Pakisztán"),
            ("hy", "Պակիստան"),
            ("ia", "Pakistan"),
            ("id", "Pakistan"),
            ("io", "Pakistan"),
            ("is", "Pakistan"),
            ("it", "Pakistan"),
            ("iu", "ᐸᑭᔅᑕᓐ"),
            ("ja", "パキスタン"),
            ("ka", "პაკისტანი"),
            ("ki", "Pakistan"),
            ("kk", "Пәкістан"),
            ("kl", "Pakistan"),
            ("km", "ប\u{17c9}ាគ\u{17b8}ស\u{17d2}ថាន"),
            ("kn", "ಪಾಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
            ("ko", "파키스탄"),
            ("ku", "Pakistan"),
            ("kv", "Пакистан"),
            ("kw", "Pakistan"),
            ("ky", "Пакистан"),
            ("lo", "Pakistan"),
            ("lt", "Pakistanas"),
            ("lv", "Pakistāna"),
            ("mi", "Pakitāne"),
            ("mk", "Пакистан"),
            ("ml", "പ\u{d3e}കിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Пакистан"),
            ("mr", "पाकिस\u{94d}तान"),
            ("ms", "Pakistan"),
            ("mt", "Pakistan"),
            (
                "my",
                "ပါကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Pakistan"),
            ("nb", "Pakistan"),
            ("ne", "पाकिस\u{94d}तान"),
            ("nl", "Pakistan"),
            ("nn", "Pakistan"),
            ("nv", "Eʼeʼaahjí Naakaii Dootłʼizhí Bikéyah"),
            ("oc", "Paquistan"),
            ("or", "ପ\u{b3e}କ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
            ("pa", "ਪਾਕਿਸਤਾਨ"),
            ("pi", "पाकिस\u{94d}तान"),
            ("pl", "Pakistan"),
            ("ps", "پاکستان"),
            ("pt", "Paquistão"),
            ("pt_BR", "Paquistão"),
            ("ro", "Pakistan"),
            ("ru", "Пакистан"),
            ("rw", "Pakisitani"),
            ("sc", "Pàkistan"),
            ("sd", "پاڪستان"),
            ("si", "පක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
            ("sk", "Pakistan"),
            ("sl", "Pakistan"),
            ("so", "Bakistaan"),
            ("sq", "Pakistan"),
            ("sr", "Пакистан"),
            ("sv", "Pakistan"),
            ("sw", "Pakistan"),
            ("ta", "ப\u{bbe}கிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
            ("te", "ప\u{c3e}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
            ("tg", "Покистон"),
            ("th", "ปาก\u{e35}สถาน"),
            ("ti", "ፓኪስታን"),
            ("tk", "Pakystan"),
            ("tl", "Pakistan"),
            ("tr", "Pakistan"),
            ("tt", "Пәкстан"),
            ("ug", "پاكىستان"),
            ("uk", "Пакистан"),
            ("ur", "پاکستان"),
            ("uz", "Pokiston"),
            ("ve", "Pakistan"),
            ("vi", "Pa-ki-xợ-thănh"),
            ("wa", "Pakistan"),
            ("wo", "Pakistaan"),
            ("xh", "Pakistan"),
            ("yo", "Pakístàn"),
            ("zh_CN", "巴基斯坦"),
            ("zh_HK", "巴基斯坦"),
            ("zh_TW", "巴基斯坦"),
            ("zu", "IPakistani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}