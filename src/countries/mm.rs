// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of the Union of Myanmar

#[cfg(all(feature = "mm", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MM;
    pub const ALPHA3: Alpha3 = Alpha3::MMR;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 95;
    pub const CURRENCY_CODE: &str = "MMK";
    pub const GEC: Option<GEC> = Some(GEC::BM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("MYA");
    pub const ISO_SHORT_NAME: &str = "Myanmar";
    pub const ISO_LONG_NAME: &str = "The Republic of the Union of Myanmar";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["my"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["my"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Myanmarian");
    pub const NUMBER: &str = "104";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "MM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Myanmar (Burma)", "ミャンマー"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Myanmar"),
        ("af", "Mianmar"),
        ("ak", "Myanmar"),
        ("am", "ምየንማ"),
        ("an", "Myanmar"),
        ("ar", "ميانمار"),
        ("as", "ম\u{9be}য়\u{9be}ন\u{9cd}ম\u{9be}ৰ"),
        ("ay", "Myanmar"),
        ("az", "Myanmar"),
        ("ba", "Myanmar"),
        ("be", "М'янма"),
        ("bg", "Мянмар"),
        ("bi", "Myanmar"),
        ("bn", "ম\u{9be}য়\u{9be}নম\u{9be}র"),
        ("bn_IN", "ম\u{9be}য়\u{9be}নম\u{9be}র"),
        ("br", "Myanmar"),
        ("bs", "Myanmar"),
        ("ca", "Myanmar"),
        ("ce", "Мьянма"),
        ("ch", "Myanmar"),
        ("cs", "Myanmar"),
        ("cv", "Мьянма"),
        ("cy", "Myanmar"),
        ("da", "Burma"),
        ("de", "Myanmar"),
        ("dv", "ބ\u{7a6}ރ\u{7aa}މ\u{7a7}"),
        ("dz", "མ\u{f72}་ཡ\u{f71}ན་མར།"),
        ("ee", "Myanmar"),
        ("el", "Μιανμάρ"),
        ("en", "Myanmar"),
        ("eo", "Mjanmao"),
        ("es", "Birmania"),
        ("et", "Myanmar"),
        ("eu", "Myanmar"),
        ("fa", "میانمار"),
        ("ff", "Myanmar"),
        ("fi", "Myanmar"),
        ("fo", "Myanmar"),
        ("fr", "Birmanie"),
        ("fy", "Birma"),
        ("ga", "Maenmar"),
        ("gl", "Birmania"),
        ("gn", "Myanmar"),
        ("gu", "મ\u{acd}યાનમાર"),
        ("gv", "Myanmar"),
        ("ha", "Myanmar"),
        ("he", "מיאנמר"),
        ("hi", "म\u{94d}यान\u{94d}मार"),
        ("hr", "Mjanmar"),
        ("ht", "Bimani"),
        ("hu", "Mianmar"),
        ("hy", "Մյանմա"),
        ("ia", "Myanmar"),
        ("id", "Myanmar"),
        ("io", "Myanmar"),
        ("is", "Mjanmar"),
        ("it", "Birmania"),
        ("iu", "Myanmar"),
        ("ja", "ミャンマー"),
        ("ka", "მიანმარი"),
        ("ki", "Burma"),
        ("kk", "Мьянма"),
        ("kl", "Myanmar"),
        ("km", "ម\u{17b8}យ\u{17c9}ាន\u{17cb}ម\u{17c9}ា"),
        ("kn", "ಮ\u{cbf}ಯನ\u{ccd}ಮಾರ\u{ccd}"),
        ("ko", "미얀마"),
        ("ku", "Miyanmar"),
        ("kv", "Мьянма"),
        ("kw", "Byrmani"),
        ("ky", "Мьянма"),
        ("lo", "ປະເທດມຽນມາ"),
        ("lt", "Mianmaras"),
        ("lv", "Mjanma"),
        ("mi", "Pēma"),
        ("mk", "Мианмар"),
        ("ml", "മ\u{d4d}യ\u{d3e}ന\u{d4d}മ\u{d3e}ര\u{d4d}\u{200d}"),
        ("mn", "Мянмар"),
        ("mr", "म\u{94d}यानमार"),
        ("ms", "Myanmar"),
        ("mt", "Myanmar"),
        ("my", "မြန\u{103a}မာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Miyanmar"),
        ("nb", "Myanmar"),
        ("ne", "म\u{94d}यान\u{94d}मार"),
        ("nl", "Myanmar"),
        ("nn", "Myanmar"),
        ("nv", "Myanmar"),
        ("oc", "Birmania"),
        ("or", "ବର\u{b4d}ମ\u{b3e}/ମ\u{b4d}ଯ\u{b3e}ଂମ\u{b3e}ର"),
        ("pa", "ਮਿਆ\u{a02}ਮਾਰ"),
        ("pi", "मयन\u{94d}मार"),
        ("pl", "Mjanma"),
        ("ps", "ميانمار"),
        ("pt", "Birmânia"),
        ("pt_BR", "Myanmar"),
        ("ro", "Myanmar"),
        ("ru", "Мьянма"),
        ("rw", "Mayanimari"),
        ("sc", "Birmània"),
        ("sd", "ميانمار"),
        ("si", "ම\u{dd2}යන\u{dca}ම\u{dcf}රය"),
        ("sk", "Mjanmarsko"),
        ("sl", "Mjanmar"),
        ("so", "Myanmar"),
        ("sq", "Birmani"),
        ("sr", "Бурма"),
        ("sv", "Myanmar"),
        ("sw", "Myanmar"),
        ("ta", "மிய\u{bbe}ன\u{bcd}மர\u{bcd}"),
        ("te", "మయన\u{c4d}మ\u{c3e}ర\u{c4d}"),
        ("tg", "Мянмор"),
        ("th", "พม\u{e48}า"),
        ("ti", "Myanmar"),
        ("tk", "Birma"),
        ("tl", "Myanmar"),
        ("tr", "Myanmar"),
        ("tt", "Мианмар"),
        ("ug", "بىرما"),
        ("uk", "М’янма"),
        ("ur", "میانمار"),
        ("uz", "Birma"),
        ("ve", "Myanmar"),
        ("vi", "Miến Điện"),
        ("wa", "Birmaneye"),
        ("wo", "Miyanmaar"),
        ("xh", "Myanmar"),
        ("yo", "Myanmar"),
        ("zh_CN", "缅甸"),
        ("zh_HK", "緬甸"),
        ("zh_TW", "緬甸"),
        ("zu", "Myanmar"),
    ];
    #[cfg(all(feature = "mm", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 21.916221;
        pub const LONGITUDE: f64 = 95.955974;
        pub const MAX_LATITUDE: f64 = 28.5478351;
        pub const MAX_LONGITUDE: f64 = 101.1702717;
        pub const MIN_LATITUDE: f64 = 9.4518;
        pub const MIN_LONGITUDE: f64 = 92.171808;
        pub const NORTHEAST_LATITUDE: f64 = 28.5478351;
        pub const NORTHEAST_LONGITUDE: f64 = 101.1702717;
        pub const SOUTHWEST_LATITUDE: f64 = 9.4518;
        pub const SOUTHWEST_LONGITUDE: f64 = 92.171808;
    }
}
#[cfg(all(feature = "mm", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 21.916221,
            longitude: 95.955974,
            max_latitude: 28.5478351,
            max_longitude: 101.1702717,
            min_latitude: 9.4518,
            min_longitude: 92.171808,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 28.5478351,
                    longitude: 101.1702717,
                },
                southwest: CountryGeoBound {
                    latitude: 9.4518,
                    longitude: 92.171808,
                },
            },
        }
    }
}

#[cfg(all(feature = "mm", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::MM,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.9159702), longitude: Some(95.9621106), max_latitude: Some(21.9659415), min_latitude: Some(21.8696635), max_longitude: Some(96.0088348), min_longitude: Some(95.8791447)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ساجينغ"), ("bn", "স\u{9be}গ\u{9be}ইনিং অঞ\u{9cd}চল"), ("ca", "Divisió de Sagaing"), ("ccp", "𑄥𑄉𑄃\u{11128}\u{11101}"), ("ceb", "Sagaing Region"), ("cs", "Sakainská oblast"), ("da", "Sagaing Region"), ("de", "Sagaing-Region"), ("el", "Σαγκάινγκ"), ("en", "Sagaing"), ("es", "División de Sagaing"), ("et", "Sagaingi piirkond"), ("fa", "ناحیه زاگاین"), ("fi", "Sagaing"), ("fr", "Région de Sagaing"), ("gu", "સાગિ\u{a82}ગ પ\u{acd}રદ\u{ac7}શ"), ("hi", "सगाइ\u{902}ग मण\u{94d}डल"), ("id", "Region Sagaing"), ("it", "regione di Sagaing"), ("ja", "ザガイン管区"), ("kn", "ಸಾಗಾಂಗ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "사가잉 구"), ("lt", "Sagaino regionas"), ("lv", "Sikainas reģions"), ("mk", "Сагаин"), ("mr", "सागायनि\u{902}ग प\u{94d}रद\u{947}श"), ("ms", "Sagaing Region"), ("my", "စစ\u{103a}က\u{102d}\u{102f}င\u{103a}းတ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Sagaing"), ("nl", "Sagaing"), ("no", "Sagaing"), ("pl", "Sikong"), ("pt", "Sagaing"), ("ru", "Сикайн"), ("si", "සගය\u{dd2}න\u{dca} සගය\u{dd2}න\u{dca}ග\u{dca} කල\u{dcf}පය"), ("sv", "Sagaingregionen"), ("ta", "ச\u{bbe}கைங\u{bcd} பகுதி"), ("te", "స\u{c3e}గ\u{c46}య\u{c3f}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตสะกาย"), ("tr", "Sagaing Bölgesi"), ("uk", "Сікайн"), ("ur", "ساگاینگ علاقہ"), ("vi", "Vùng Sagaing"), ("zh", "實皆省")]),
                        unofficial_name_list: ["Sagaing"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::MM,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.333333), longitude: Some(96.483333), max_latitude: Some(17.3624761), min_latitude: Some(17.2299639), max_longitude: Some(96.51615849999999), min_longitude: Some(96.4299202)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باغو"), ("bn", "ব\u{9be}গো"), ("ca", "Divisió de Bago"), ("ccp", "𑄝𑄉\u{1112e}"), ("ceb", "Bago Region"), ("da", "Bago"), ("de", "Bago-Region"), ("el", "Μπάγκο"), ("en", "Bago"), ("es", "Bago (Birmania)"), ("et", "Bago piirkond"), ("fa", "ناحیه باگو"), ("fi", "Bago (alue)"), ("fr", "Région de Bago"), ("gu", "બગો"), ("hi", "बगो मण\u{94d}डल"), ("id", "Region Bago"), ("it", "regione di Bago"), ("ja", "バゴー管区"), ("kn", "ಬಾಗೋ"), ("ko", "바고 구"), ("lt", "Bago regionas"), ("lv", "Pegu"), ("mk", "Баго (округ)"), ("mr", "ब\u{947}गो"), ("ms", "Bago"), ("my", "ပ\u{1032}ခ\u{1030}းတ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Bago"), ("nl", "Bago"), ("no", "Bago"), ("pl", "Pegu (prowincja)"), ("pt", "Bago (divisão)"), ("ru", "Пегу (округ)"), ("si", "බගෝ"), ("sv", "Bagoregionen"), ("ta", "ப\u{bbe}கோ"), ("te", "బ\u{c3e}గ\u{c4b}"), ("th", "เขตหงสาวด\u{e35}"), ("tr", "Bago"), ("uk", "Пегу"), ("ur", "باگو علاقہ"), ("vi", "Vùng Bago"), ("zh", "勃固省")]),
                        unofficial_name_list: ["Pegu"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::MM,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.15), longitude: Some(94.94999999999999), max_latitude: Some(20.1782538), min_latitude: Some(20.1100737), max_longitude: Some(94.99182309999999), min_longitude: Some(94.9161388)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ماغواي"), ("bn", "ম\u{9be}গওয\u{9bc}ে অঞ\u{9cd}চল"), ("ca", "Divisió de Magwe"), ("ccp", "𑄟𑄇\u{11134}𑄃\u{1112e}𑄠𑄬"), ("ceb", "Magway Region"), ("cs", "Makweiská oblast"), ("da", "Magway Region"), ("de", "Magway-Region"), ("el", "Μαγκγουέι"), ("en", "Magway"), ("es", "Magway"), ("et", "Magway piirkond"), ("fa", "ناحیه ماگوای"), ("fi", "Magway"), ("fr", "Région de Magway"), ("gu", "મ\u{ac7}ગવ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("hi", "मगव\u{947} मण\u{94d}डल"), ("id", "Region Magway"), ("it", "regione di Magway"), ("ja", "マグウェ管区"), ("kn", "ಮ\u{ccd}ಯಾಗ\u{ccd}ವೇ ಪ\u{ccd}ರದೇಶ"), ("ko", "마궤 구"), ("lt", "Magvės regionas"), ("lv", "Magves reģions"), ("mk", "Магве"), ("mr", "म\u{945}गव\u{947} प\u{94d}रद\u{947}श"), ("ms", "Magway Region"), ("my", "မက\u{103d}ေးတ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Magway"), ("nl", "Magway"), ("no", "Magway"), ("pl", "Magwe"), ("pt", "Magway"), ("ru", "Магуэ"), ("si", "මැග\u{dca}වේ කල\u{dcf}පය"), ("sv", "Magwayregionen"), ("ta", "ம\u{bbe}குவே மண\u{bcd}டலம\u{bcd}"), ("te", "మ\u{c4d}య\u{c3e}గ\u{c4d}వ\u{c47} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตมาเกว"), ("tr", "Magway Bölgesi"), ("uk", "Магуе"), ("ur", "ماگاوے علاقہ"), ("vi", "Vùng Magway"), ("zh", "馬圭省")]),
                        unofficial_name_list: ["Magwe"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::MM,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.975), longitude: Some(96.083333), max_latitude: Some(22.0261394), min_latitude: Some(21.8545804), max_longitude: Some(96.14041019999999), min_longitude: Some(96.0112381)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ماندالاي"), ("be", "Мандалай"), ("bn", "ম\u{9be}ন\u{9cd}ড\u{9be}ল\u{9be}য\u{9bc} অঞ\u{9cd}চল"), ("ca", "Divisió de Mandalay"), ("ccp", "𑄟𑄚\u{11134}𑄓𑄣𑄬"), ("ceb", "Mandalay Region"), ("cs", "Mandalajská oblast"), ("da", "Mandalay Region"), ("de", "Mandalay-Region"), ("el", "Μάνταλεϊ"), ("en", "Mandalay"), ("es", "Región del Mandalay"), ("et", "Mandalay piirkond"), ("fa", "ایالت ماندلی"), ("fi", "Mandalay"), ("fr", "Région de Mandalay"), ("gu", "મા\u{a82}ડલ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("hi", "माण\u{94d}डल\u{947} मण\u{94d}डल"), ("id", "Region Mandalay"), ("it", "regione di Mandalay"), ("ja", "マンダレー管区"), ("kn", "ಮ\u{ccd}ಯಾಂಡಲ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "만달레이 구"), ("lt", "Mandalajaus regionas"), ("lv", "Mandalajas reģions"), ("mk", "Мандалеј (округ)"), ("mr", "म\u{945}\u{902}ड\u{947}ल\u{947} प\u{94d}रद\u{947}श"), ("ms", "Mandalay Region"), ("my", "မန\u{1039}တလေးတ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Mandalay"), ("nl", "Mandalay"), ("no", "Mandalay"), ("pl", "Mandalaj (prowincja)"), ("pt", "Mandalay (divisão)"), ("ru", "Мандалай (округ)"), ("si", "මැන\u{dca}ඩලේ කල\u{dcf}පය"), ("sv", "Mandalayregionen"), ("ta", "மண\u{bcd}டலே பகுதி"), ("te", "మండ\u{c3e}ల\u{c47} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตม\u{e31}ณฑะเลย\u{e4c}"), ("tr", "Mandalay Bölgesi"), ("uk", "Мандалай"), ("ur", "ماندالے علاقہ"), ("vi", "Vùng Mandalay"), ("zh", "曼德勒省")]),
                        unofficial_name_list: ["Mandalay"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::MM,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.0896327), longitude: Some(99.0115113), max_latitude: Some(12.0926189), min_latitude: Some(12.0778055), max_longitude: Some(99.020505), min_longitude: Some(99.0064103)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم تانينثاري"), ("bn", "ত\u{9be}নিনত\u{9be}র\u{9be}রি অঞ\u{9cd}চল"), ("ca", "Divisió de Tanintharyi"), ("ccp", "𑄑𑄚\u{11128}𑄚\u{11134}𑄗𑄢\u{11128}"), ("ceb", "Taninthayi Region"), ("cs", "Tanintharyi"), ("da", "Tanintharyi Region"), ("de", "Tanintharyi-Region"), ("el", "Τανινθάργι"), ("en", "Tanintharyi"), ("es", "Tanintharyi"), ("et", "Tanintharyi piirkond"), ("fa", "ناحیه تانینتاریی"), ("fi", "Tanintharyi"), ("fr", "Région de Tanintharyi"), ("gu", "તાનિન\u{acd}થાર\u{acd}યી પ\u{acd}રદ\u{ac7}શ"), ("hi", "तनीन\u{94d}थार\u{94d}यी मण\u{94d}डल"), ("id", "Region Tanintharyi"), ("it", "regione di Tanintharyi"), ("ja", "タニンダーリ管区"), ("kn", "ತನ\u{cbf}ಂಥರ\u{ccd}ಯ\u{ccd}ಈ ಪ\u{ccd}ರದೇಶ"), ("ko", "타닌타리 구"), ("lt", "Tanintajio regionas"), ("lv", "Tanintaji reģions"), ("mk", "Танинтаји"), ("mr", "तन\u{94d}था\u{902}रीय प\u{94d}रद\u{947}श"), ("ms", "Wilayah Tanintharyi"), ("my", "တနင\u{103a}\u{1039}သာရ\u{102e}တ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Tanintharyi"), ("nl", "Tanintharyi"), ("no", "Tanintharyi"), ("pa", "ਤਨੀ\u{a02}ਥਾਰਾਈ ਮ\u{a70}ਡਲ"), ("pl", "Taninthayi"), ("pt", "Tanintharyi"), ("ru", "Танинтайи"), ("si", "ටන\u{dd2}න\u{dca}තර\u{dd2} කල\u{dcf}පය"), ("sv", "Taninthayiregionen"), ("ta", "தணிந\u{bcd}தருயி பகுதி"), ("te", "ట\u{c3e}న\u{c3f}ంత\u{c3e}ర\u{c4d}య\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตตะนาวศร\u{e35}"), ("tr", "Tanintharyi Region"), ("uk", "Танінтаї"), ("ur", "تانینتھارئی علاقہ"), ("vi", "Vùng Tanintharyi"), ("zh", "德林达依")]),
                        unofficial_name_list: ["Tenasserim"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::MM,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.780833), longitude: Some(96.149722), max_latitude: Some(17.0788151), min_latitude: Some(16.7288268), max_longitude: Some(96.3432311), min_longitude: Some(95.9957885)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم يانغون"), ("be", "Янгон"), ("bn", "ইয\u{9bc}\u{9be}গন অঞ\u{9cd}চল"), ("ca", "Divisió de Yangon"), ("ccp", "𑄃\u{11128}𑄠\u{11101}𑄉\u{1112a}𑄚\u{11134}"), ("ceb", "Yangon Region"), ("cs", "Rangúnská oblast"), ("da", "Yangon Region"), ("de", "Yangon-Region"), ("el", "Γιάνγκον"), ("en", "Yangon"), ("es", "Región de Yangon"), ("et", "Yangoni piirkond"), ("fa", "ناحیه یانگون"), ("fi", "Yangon"), ("fr", "Région de Yangon"), ("gu", "યા\u{a82}ગોન પ\u{acd}રદ\u{ac7}શ"), ("hi", "या\u{902}गोन मण\u{94d}डल"), ("id", "Region Yangon"), ("it", "regione di Yangon"), ("ja", "ヤンゴン管区"), ("kn", "ಯಾಂಗೊನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "양곤 구"), ("lt", "Jangono regionas"), ("lv", "Jangonas reģions"), ("mk", "Јангон"), ("mr", "या\u{902}गॉन रिजन"), ("ms", "Yangon Region"), ("my", "ရန\u{103a}က\u{102f}န\u{103a}တ\u{102d}\u{102f}င\u{103a}း"), ("nb", "Yangon"), ("nl", "Rangoon"), ("no", "Yangon"), ("pa", "ਯਾ\u{a02}ਗ\u{a4b}ਨ ਖ\u{a47}ਤਰ"), ("pl", "Rangun"), ("pt", "Yangon"), ("ru", "Янгон"), ("si", "යැන\u{dca}ගොන\u{dca} කල\u{dcf}පය"), ("sv", "Yangonregionen"), ("ta", "யங\u{bcd}கொண\u{bcd} பகுதி"), ("te", "య\u{c3e}ంగ\u{c4b}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตย\u{e48}างก\u{e38}\u{e49}ง"), ("tr", "Yangon Bölgesi"), ("uk", "Янгон"), ("ur", "یانگون رگوں"), ("vi", "Vùng Yangon"), ("zh", "仰光省")]),
                        unofficial_name_list: ["Rangoun", "Rangun", "Rangún", "Yangon"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::MM,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.0342125), longitude: Some(95.22666749999999), max_latitude: Some(18.4690619), min_latitude: Some(13.9746695), max_longitude: Some(95.9665632), min_longitude: Some(93.22602839999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ayeyarwady Streek"), ("ar", "مقاطعة أيياروادي"), ("az", "İravadi dairəsi"), ("be", "Іравадзі"), ("bn", "আয\u{9bc}েয\u{9bc}\u{9be}র\u{9cd}ব\u{9cd}য\u{9be}দি অঞ\u{9cd}চল"), ("ca", "Divisió d’Ayeyarwady"), ("ccp", "𑄃\u{11127}𑄠𑄬𑄠𑄢\u{11134}𑄃\u{11128}𑄠𑄓\u{11128}"), ("ceb", "Ayeyawady Region"), ("cs", "oblast Iravádí"), ("da", "Ayeyarwady Region"), ("de", "Irawadi-Region"), ("el", "Επαρχία Αγιεγιαγουάντι"), ("en", "Ayeyarwady"), ("es", "Ayeyarwady"), ("et", "Ayeyarwady piirkond"), ("fa", "ناحیه آیه\u{200c}یاروادی"), ("fi", "Ayeyarwady"), ("fr", "Région d’Ayeyarwady"), ("gu", "આય\u{ac7}યાર\u{acd}વાડી પ\u{acd}રદ\u{ac7}શ"), ("hi", "इरावदी मण\u{94d}डल"), ("hy", "Իրավադի"), ("id", "Region Ayeyarwady"), ("it", "regione di Ayeyarwady"), ("ja", "エーヤワディ管区"), ("kn", "ಅಯ\u{ccd}ಯರ\u{ccd}ವಾಡ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "에야와디 구"), ("lt", "Ajejarvadžio regionas"), ("lv", "Ajejarvadžio reģions"), ("mk", "Иравади"), ("mr", "आयय\u{947}र\u{94d}वाडी प\u{94d}रद\u{947}श"), ("ms", "Daerah Ayeyarwady"), ("nb", "Ayeyarwady"), ("nl", "Ayeyarwady"), ("no", "Ayeyarwady"), ("pa", "ਏਯਾਰਵਾਡੀ ਮ\u{a70}ਡਲ"), ("pl", "Irawadi"), ("pt", "Ayeyarwady"), ("ru", "Иравади"), ("si", "අයෙය\u{dcf}ර\u{dca}වඩ\u{dd2} කල\u{dcf}පය"), ("sv", "Ayeyarwady (region)"), ("ta", "அயேயர\u{bcd}வ\u{bbe}டி பகுதி"), ("te", "అయ\u{c47}య\u{c3e}ర\u{c4d}వ\u{c3e}డ\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอ\u{e34}รวด\u{e35}"), ("tr", "Ayeryarwady Bölgesi"), ("uk", "Іраваді"), ("ur", "ایئیاروادی علاقہ"), ("vi", "Vùng Ayeyarwady"), ("yue", "愛耶也華地省"), ("yue_Hans", "爱耶也华地省"), ("zh", "伊洛瓦底省")]),
                        unofficial_name_list: ["Irrawaddy"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::MM,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.850904), longitude: Some(97.4381355), max_latitude: Some(28.543259), min_latitude: Some(23.641569), max_longitude: Some(98.78279099999999), min_longitude: Some(95.8360589)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كاشين"), ("as", "ক\u{9be}চিন"), ("bn", "ক\u{9be}ছিন র\u{9be}জ\u{9cd}য"), ("ca", "Kachin"), ("ccp", "𑄇𑄌\u{11128}𑄚\u{11134}"), ("ceb", "Kachin State"), ("cs", "Kačjinský stát"), ("cy", "Talaith Kachin"), ("da", "Kachin State"), ("de", "Kachin-Staat"), ("el", "Κατσίν"), ("en", "Kachin"), ("es", "Estado Kachin"), ("et", "Katšini osariik"), ("eu", "Katxin estatua"), ("fa", "ایالت کاچین"), ("fi", "Kachin"), ("fr", "État de Kachin"), ("gu", "કાચિન સ\u{acd}ટ\u{ac7}ટ"), ("hi", "काचीन राज\u{94d}य"), ("hr", "Država Kachin"), ("hy", "Կաչին"), ("id", "Negara Bagian Kachin"), ("it", "Stato Kachin"), ("ja", "カチン州"), ("kn", "ಕಚ\u{cbf}ನ\u{ccd}"), ("ko", "카친 주"), ("lt", "Kačinų valstija"), ("lv", "Kačinas pavalsts"), ("mk", "Качин"), ("mr", "काचीन राज\u{94d}य"), ("ms", "Kachin State"), ("my", "ကချင\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Kachin"), ("nl", "Kachin"), ("no", "Kachin"), ("pa", "ਕਾਚੀਨ ਸ\u{a42}ਬਾ"), ("pl", "Kaczin"), ("pt", "Kachin"), ("ro", "Statul Kachin"), ("ru", "Качин"), ("si", "කච\u{dd2}න\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Качин"), ("sr_Latn", "Kačin"), ("sv", "Kachin"), ("ta", "கொச\u{bcd}சின\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "కచ\u{c3f}న\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐกะฉ\u{e34}\u{e48}น"), ("tr", "Kachin Eyaleti"), ("uk", "Качин"), ("ur", "کاچین ریاست"), ("vi", "Kachin"), ("zh", "克钦邦")]),
                        unofficial_name_list: ["Kachin"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::MM,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.2342061), longitude: Some(97.26528580000002), max_latitude: Some(19.9939339), min_latitude: Some(18.4917349), max_longitude: Some(97.88789399999999), min_longitude: Some(96.833956)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كاياه"), ("bn", "ক\u{9be}য\u{9bc}\u{9be}হ অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Kayah"), ("ccp", "𑄇𑄬𑄠𑄦\u{11134}"), ("ceb", "Kayah State"), ("cs", "Kajaský stát"), ("da", "Kayah-regionen"), ("de", "Kayah-Staat"), ("el", "Κάγια"), ("en", "Kayah"), ("es", "Kayah"), ("et", "Kaja osariik"), ("fa", "ایالت کایا"), ("fi", "Kayah"), ("fr", "État de Kayah"), ("gu", "ક\u{ac7}યાહ સ\u{acd}ટ\u{ac7}ટ"), ("hi", "कयाह राज\u{94d}य"), ("hy", "Կայա"), ("id", "Negara Bagian Kayah"), ("it", "Stato Kayah"), ("ja", "カヤー州"), ("kn", "ಕರ\u{cc6}ನ\u{cbf} ರಾಜ\u{ccd}ಯ"), ("ko", "카야 주"), ("lt", "Kajahų valstija"), ("lv", "Kajas pavalsts"), ("mk", "Каја"), ("mr", "कायह राज\u{94d}य"), ("ms", "Kayah State"), ("my", "ကယားပြည\u{103a}နယ\u{103a}"), ("nb", "Kayah"), ("nl", "Kayah"), ("no", "Kayah"), ("pa", "ਕਾਇਆਹ ਰਾਜ"), ("pl", "Kaja"), ("pt", "Kayah"), ("ru", "Кая"), ("si", "කය\u{dcf} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Каја"), ("sr_Latn", "Kaja"), ("sv", "Kayahstaten"), ("ta", "க\u{bbe}ய ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c3e}య\u{c3e} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐกะยา"), ("tr", "Kayah Eyaleti"), ("uk", "Кая"), ("ur", "کایاہ ریاست"), ("vi", "Kayah"), ("zh", "克耶邦")]),
                        unofficial_name_list: ["Kayah"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::MM,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.9459346), longitude: Some(97.9592863), max_latitude: Some(19.4953029), min_latitude: Some(15.2195629), max_longitude: Some(98.91750309999999), min_longitude: Some(96.39982189999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كايين"), ("bn", "ক\u{9be}য\u{9bc}িন অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Karen"), ("ccp", "𑄇𑄬𑄠𑄃\u{11128}𑄚\u{11134}"), ("ceb", "Kayin State"), ("cs", "Karenský stát"), ("da", "Kayin State"), ("de", "Kayin-Staat"), ("el", "Καγίν"), ("en", "Kayin"), ("es", "Kayin"), ("et", "Kareni osariik"), ("fa", "ایالت کایین"), ("fi", "Kayin"), ("fr", "État Karen"), ("gu", "ક\u{ac7}યિન સ\u{acd}ટ\u{ac7}ટ"), ("hi", "कयिन राज\u{94d}य"), ("id", "Negara Bagian Kayin"), ("it", "Stato Karen"), ("ja", "カレン州"), ("kn", "ಕರ\u{cc6}ನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "카인 주"), ("lt", "Kajinų valstija"), ("lv", "Karenas pavalsts"), ("mk", "Карен"), ("mr", "कायिन राज\u{94d}य"), ("ms", "Kayin"), ("my", "ကရင\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Karen"), ("nl", "Kayin"), ("no", "Karen"), ("pa", "ਕਾਇਨ"), ("pl", "Karen"), ("pt", "Kayin"), ("ru", "Карен"), ("si", "කේ ඉන\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Карен"), ("sr_Latn", "Karen"), ("sv", "Karen"), ("ta", "க\u{bbe}யின\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c47}య\u{c3f}న\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐกะเหร\u{e35}\u{e48}ยง"), ("tr", "Kayin Eyaleti"), ("uk", "Карен"), ("ur", "کایئن ریاست"), ("vi", "Kayin"), ("zh", "克倫邦")]),
                        unofficial_name_list: ["Karen"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::MM,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.3896751), longitude: Some(93.5812692), max_latitude: Some(24.0915281), min_latitude: Some(20.6635458), max_longitude: Some(94.1857909), min_longitude: Some(92.6037597)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تشين"), ("bn", "চিন অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Xin"), ("ccp", "𑄌\u{11128}𑄚\u{11134}"), ("ceb", "Chin State"), ("cs", "Čjinský stát"), ("da", "Chin"), ("de", "Chin-Staat"), ("el", "Τσιν"), ("en", "Chin"), ("es", "Chin"), ("et", "Tšini osariik"), ("eu", "Chin estatua"), ("fa", "ایالت چین"), ("fi", "Chin"), ("fr", "État Chin"), ("gu", "ચિન સ\u{acd}ટ\u{ac7}ટ"), ("hi", "चिन राज\u{94d}य"), ("hy", "Չին"), ("id", "Negara Bagian Chin"), ("it", "Stato Chin"), ("ja", "チン州"), ("kn", "ಚ\u{cbf}ನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "친 주"), ("lt", "Činų valstija"), ("lv", "Čina"), ("mk", "Чин"), ("ml", "ചിൻ സംസ\u{d4d}ഥ\u{d3e}നം"), ("mr", "चिन राज\u{94d}य"), ("ms", "Chin State"), ("nb", "Chin"), ("nl", "Chin"), ("no", "Chin"), ("pl", "Czin"), ("pt", "Chin"), ("ro", "Statul Chin"), ("ru", "Чин"), ("si", "ච\u{dd2}න\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Чин"), ("sr_Latn", "Čin"), ("sv", "Chin"), ("ta", "சின\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "చ\u{c3f}న\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐช\u{e34}น"), ("tr", "Chin Eyaleti"), ("uk", "Чин"), ("ur", "چن ریاست"), ("vi", "Chin"), ("zh", "欽邦")]),
                        unofficial_name_list: ["Chin"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::MM,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.0626039), longitude: Some(97.35165579999999), max_latitude: Some(17.7052629), min_latitude: Some(14.884192), max_longitude: Some(98.23783879999999), min_longitude: Some(96.8755518)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية مون"), ("bn", "মন র\u{9be}জ\u{9cd}য"), ("ca", "Mon"), ("ccp", "𑄟\u{11127}𑄚\u{11134}"), ("ceb", "Mon State"), ("cs", "Monský stát"), ("da", "Mon State"), ("de", "Mon-Staat"), ("el", "Μον"), ("en", "Mon"), ("es", "Estado Mon"), ("et", "Moni osariik"), ("fa", "ایالت مون"), ("fi", "Mon"), ("fr", "État Môn"), ("gu", "મોન સ\u{acd}ટ\u{ac7}ટ"), ("hi", "मोन राज\u{94d}य"), ("hy", "Մոն"), ("id", "Mon State"), ("it", "Stato Mon"), ("ja", "モン州"), ("kn", "ಮೋನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "몬 주"), ("lt", "Monų valstija"), ("lv", "Monas pavalsts"), ("mk", "Мон"), ("mr", "मोन राज\u{94d}य"), ("ms", "Mon State"), ("my", "မ\u{103d}န\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Mon"), ("nl", "Mon"), ("no", "Mon"), ("pa", "ਮ\u{a4b}ਨ ਸਟ\u{a47}ਟ"), ("pl", "Mon"), ("pt", "Mon"), ("ru", "Мон"), ("si", "මොන\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Мон"), ("sr_Latn", "Mon"), ("sv", "Mon"), ("ta", "மோன\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "మ\u{c4b}న\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐมอญ"), ("tr", "Mon Eyaleti"), ("uk", "Мон"), ("ur", "مون ریاست"), ("vi", "Mon"), ("zh", "孟邦")]),
                        unofficial_name_list: ["Mon"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::MM,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.810093), longitude: Some(93.9878427), max_latitude: Some(21.4735741), min_latitude: Some(17.363218), max_longitude: Some(94.909111), min_longitude: Some(92.189278)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "راخين"), ("bn", "আর\u{9be}ক\u{9be}ন র\u{9be}জ\u{9cd}য"), ("ca", "Arakan"), ("ccp", "𑄢𑄈\u{1112d}𑄚\u{11134}"), ("ceb", "Rakhine State"), ("cs", "Rakhinský stát"), ("da", "Rakhine State"), ("de", "Rakhaing-Staat"), ("el", "Ρακίνε"), ("en", "Rakhine"), ("es", "Rakhine"), ("et", "Arakani osariik"), ("eu", "Rakhine estatua"), ("fa", "ایالت راخین"), ("fi", "Rakhine"), ("fr", "État d’Arakan"), ("gu", "રખીન સ\u{acd}ટ\u{ac7}ટ"), ("he", "ראקין"), ("hi", "अराकान"), ("id", "Negara Bagian Rakhine"), ("it", "Stato Rakhine"), ("ja", "ラカイン州"), ("jv", "Rakhine"), ("kn", "ಅರಕಾನ\u{ccd}"), ("ko", "라카인 주"), ("lt", "Rachinų valstija"), ("lv", "Rakhainas pavalsts"), ("mk", "Ракајн"), ("mr", "रखिन राज\u{94d}य"), ("ms", "Negeri Rakhine"), ("my", "ရခ\u{102d}\u{102f}င\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Rakhine"), ("nl", "Rakhine"), ("no", "Rakhine"), ("pl", "Arakan"), ("pt", "Arracão"), ("ru", "Ракхайн"), ("si", "රක\u{dd2}නේ ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Ракајн"), ("sr_Latn", "Rakajn"), ("sv", "Rakhinestaten"), ("sw", "Dola la Rakhaing"), ("ta", "ரஃஹின\u{bc0} ம\u{bbe}நிலம\u{bcd}"), ("te", "ర\u{c3e}ఖ\u{c3f}న\u{c46} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐยะไข\u{e48}"), ("tr", "Rakhine Eyaleti"), ("uk", "Ракхайн"), ("ur", "راخائن ریاست"), ("vi", "Rakhine"), ("zh", "若開邦")]),
                        unofficial_name_list: ["Arakan"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::MM,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.156717), longitude: Some(97.57347829999999), max_latitude: Some(21.1839317), min_latitude: Some(21.1287002), max_longitude: Some(97.59275430000001), min_longitude: Some(97.5539589)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية شان"), ("bn", "শ\u{9be}ন র\u{9be}জ\u{9cd}য"), ("ca", "Estat Shan"), ("ccp", "𑄥𑄚\u{11134}"), ("ceb", "Shan State"), ("cs", "Šanský stát"), ("da", "Shan State"), ("de", "Shan-Staat"), ("el", "Σαν"), ("en", "Shan"), ("es", "Shan"), ("et", "Šani osariik"), ("eu", "Shan estatua"), ("fa", "ایالت شان"), ("fi", "Shan"), ("fr", "État Shan"), ("gu", "શાન સ\u{acd}ટ\u{ac7}ટ"), ("hi", "शान राज\u{94d}य"), ("hy", "Շան"), ("id", "Negara Bagian Shan"), ("it", "Stato Shan"), ("ja", "シャン州"), ("kn", "ಶನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "샨 주"), ("lt", "Šanų valstija"), ("lv", "Šanas pavalsts"), ("mk", "Шан"), ("ml", "ഷ\u{d3e}ൻ സംസ\u{d4d}ഥ\u{d3e}നം"), ("mr", "शान राज\u{94d}य"), ("ms", "Shan State"), ("my", "ရ\u{103e}မ\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Shan"), ("nl", "Shan"), ("no", "Shan"), ("pl", "Szan"), ("pt", "Shan"), ("ru", "Шан"), ("si", "ශ\u{dcf}න\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("sr", "Шан"), ("sr_Latn", "Šan"), ("sv", "Shanstaten"), ("ta", "ஷ\u{bbe}ன\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "ష\u{c3e}న\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐฉาน"), ("tr", "Shan State"), ("uk", "Шан"), ("ur", "ریاست شان"), ("vi", "Shan"), ("yue", "撣邦"), ("yue_Hans", "掸邦"), ("zh", "掸邦")]),
                        unofficial_name_list: ["Shan"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::MM,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Territori de la Unió de Naypyidaw"), ("ccp", "𑄚𑄬𑄛\u{1112d}𑄃\u{11128}𑄓\u{11133}𑄦\u{11127}"), ("de", "Unionsterritorium Naypyidaw"), ("el", "Ενωσιακή Επικράτεια Νέπιντο"), ("en", "Naypyidaw"), ("es", "Territorio de la Unión de Naypyidaw"), ("fr", "Naypyidaw Territoire de l’Union"), ("it", "Territorio dell’Unione di Naypyidaw"), ("ja", "ネピドー連邦領"), ("ko", "네피도 연방구"), ("my", "နေပြည\u{103a}တော\u{103a} ပြည\u{103a}ထောင\u{103a}စ\u{102f}နယ\u{103a}မြေ"), ("nl", "Naypyidaw Union Territory"), ("pl", "Naypyidaw"), ("pt", "Território da União de Naypyidaw"), ("ta", "நைப\u{bcd}பியித\u{bbe} யூனியன\u{bcd} பிரதேசம\u{bcd}"), ("th", "ด\u{e34}นแดนสหภาพเนปย\u{e35}ดอ"), ("ur", "نیپیداو متحدہ خطہ"), ("vi", "Lãnh thổ Liên bang Naypyidaw"), ("zh", "内比都联邦区")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "mm")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MM,
        alpha3: Alpha3::MMR,
        address_format: None,
        continent: Continent::Asia,
        country_code: 95,
        currency_code: "MMK",
        gec: Some(GEC::BM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("MYA"),
        iso_long_name: "The Republic of the Union of Myanmar",
        iso_short_name: "Myanmar",
        official_language_list: ["my"].to_vec(),
        spoken_language_list: ["my"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "None",
        nationality: Some("Myanmarian"),
        number: "104",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "MM",
        unofficial_name_list: ["Myanmar (Burma)", "ミャンマー"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Myanmar"),
            ("af", "Mianmar"),
            ("ak", "Myanmar"),
            ("am", "ምየንማ"),
            ("an", "Myanmar"),
            ("ar", "ميانمار"),
            ("as", "ম\u{9be}য়\u{9be}ন\u{9cd}ম\u{9be}ৰ"),
            ("ay", "Myanmar"),
            ("az", "Myanmar"),
            ("ba", "Myanmar"),
            ("be", "М'янма"),
            ("bg", "Мянмар"),
            ("bi", "Myanmar"),
            ("bn", "ম\u{9be}য়\u{9be}নম\u{9be}র"),
            ("bn_IN", "ম\u{9be}য়\u{9be}নম\u{9be}র"),
            ("br", "Myanmar"),
            ("bs", "Myanmar"),
            ("ca", "Myanmar"),
            ("ce", "Мьянма"),
            ("ch", "Myanmar"),
            ("cs", "Myanmar"),
            ("cv", "Мьянма"),
            ("cy", "Myanmar"),
            ("da", "Burma"),
            ("de", "Myanmar"),
            ("dv", "ބ\u{7a6}ރ\u{7aa}މ\u{7a7}"),
            ("dz", "མ\u{f72}་ཡ\u{f71}ན་མར།"),
            ("ee", "Myanmar"),
            ("el", "Μιανμάρ"),
            ("en", "Myanmar"),
            ("eo", "Mjanmao"),
            ("es", "Birmania"),
            ("et", "Myanmar"),
            ("eu", "Myanmar"),
            ("fa", "میانمار"),
            ("ff", "Myanmar"),
            ("fi", "Myanmar"),
            ("fo", "Myanmar"),
            ("fr", "Birmanie"),
            ("fy", "Birma"),
            ("ga", "Maenmar"),
            ("gl", "Birmania"),
            ("gn", "Myanmar"),
            ("gu", "મ\u{acd}યાનમાર"),
            ("gv", "Myanmar"),
            ("ha", "Myanmar"),
            ("he", "מיאנמר"),
            ("hi", "म\u{94d}यान\u{94d}मार"),
            ("hr", "Mjanmar"),
            ("ht", "Bimani"),
            ("hu", "Mianmar"),
            ("hy", "Մյանմա"),
            ("ia", "Myanmar"),
            ("id", "Myanmar"),
            ("io", "Myanmar"),
            ("is", "Mjanmar"),
            ("it", "Birmania"),
            ("iu", "Myanmar"),
            ("ja", "ミャンマー"),
            ("ka", "მიანმარი"),
            ("ki", "Burma"),
            ("kk", "Мьянма"),
            ("kl", "Myanmar"),
            ("km", "ម\u{17b8}យ\u{17c9}ាន\u{17cb}ម\u{17c9}ា"),
            ("kn", "ಮ\u{cbf}ಯನ\u{ccd}ಮಾರ\u{ccd}"),
            ("ko", "미얀마"),
            ("ku", "Miyanmar"),
            ("kv", "Мьянма"),
            ("kw", "Byrmani"),
            ("ky", "Мьянма"),
            ("lo", "ປະເທດມຽນມາ"),
            ("lt", "Mianmaras"),
            ("lv", "Mjanma"),
            ("mi", "Pēma"),
            ("mk", "Мианмар"),
            ("ml", "മ\u{d4d}യ\u{d3e}ന\u{d4d}മ\u{d3e}ര\u{d4d}\u{200d}"),
            ("mn", "Мянмар"),
            ("mr", "म\u{94d}यानमार"),
            ("ms", "Myanmar"),
            ("mt", "Myanmar"),
            ("my", "မြန\u{103a}မာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Miyanmar"),
            ("nb", "Myanmar"),
            ("ne", "म\u{94d}यान\u{94d}मार"),
            ("nl", "Myanmar"),
            ("nn", "Myanmar"),
            ("nv", "Myanmar"),
            ("oc", "Birmania"),
            ("or", "ବର\u{b4d}ମ\u{b3e}/ମ\u{b4d}ଯ\u{b3e}ଂମ\u{b3e}ର"),
            ("pa", "ਮਿਆ\u{a02}ਮਾਰ"),
            ("pi", "मयन\u{94d}मार"),
            ("pl", "Mjanma"),
            ("ps", "ميانمار"),
            ("pt", "Birmânia"),
            ("pt_BR", "Myanmar"),
            ("ro", "Myanmar"),
            ("ru", "Мьянма"),
            ("rw", "Mayanimari"),
            ("sc", "Birmània"),
            ("sd", "ميانمار"),
            ("si", "ම\u{dd2}යන\u{dca}ම\u{dcf}රය"),
            ("sk", "Mjanmarsko"),
            ("sl", "Mjanmar"),
            ("so", "Myanmar"),
            ("sq", "Birmani"),
            ("sr", "Бурма"),
            ("sv", "Myanmar"),
            ("sw", "Myanmar"),
            ("ta", "மிய\u{bbe}ன\u{bcd}மர\u{bcd}"),
            ("te", "మయన\u{c4d}మ\u{c3e}ర\u{c4d}"),
            ("tg", "Мянмор"),
            ("th", "พม\u{e48}า"),
            ("ti", "Myanmar"),
            ("tk", "Birma"),
            ("tl", "Myanmar"),
            ("tr", "Myanmar"),
            ("tt", "Мианмар"),
            ("ug", "بىرما"),
            ("uk", "М’янма"),
            ("ur", "میانمار"),
            ("uz", "Birma"),
            ("ve", "Myanmar"),
            ("vi", "Miến Điện"),
            ("wa", "Birmaneye"),
            ("wo", "Miyanmaar"),
            ("xh", "Myanmar"),
            ("yo", "Myanmar"),
            ("zh_CN", "缅甸"),
            ("zh_HK", "緬甸"),
            ("zh_TW", "緬甸"),
            ("zu", "Myanmar"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}