// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Virgin Islands

#[cfg(all(feature = "vg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::VG;
    pub const ALPHA3: Alpha3 = Alpha3::VGB;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::USD;
    pub const GEC: Option<GEC> = Some(GEC::VI);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::IVB);
    pub const ISO_SHORT_NAME: &str = "Virgin Islands (British)";
    pub const ISO_LONG_NAME: &str = "The Virgin Islands";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Virgin Islander");
    pub const NUMBER: &str = "092";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("VG\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "VG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "British Virgin Islands",
        "Britische Jungferninseln",
        "Îles Vierges britanniques",
        "Islas Vírgenes del Reino Unido",
        "イギリス領ヴァージン諸島",
        "Britse Maagdeneilanden",
        "Virgin Islands (British)",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Virgin Islands, British"),
        ("af", "Maagde-eilande, Britse"),
        ("ak", "Virgin Islands, British"),
        ("am", "ፘጕንጔሑፔ ፥ንጔሔ ፠ሴቶኄ"),
        ("an", "Virgin Islands, British"),
        ("ar", "فيرجن، جزر فيرجن البريطاني\u{651}ة"),
        ("as", "ব\u{9cd}ৰিটিশ\u{9cd}ব ভ\u{9be}ৰ\u{9cd}জিন দ\u{9cd}বীপ"),
        ("ay", "Virgin Islands, British"),
        ("az", "Virgin Adaları, Britaniya"),
        ("ba", "Virgin Islands, British"),
        ("be", "Віргінскія астравы Вялікабрытаніі"),
        ("bg", "Вирджински острови, Британски"),
        ("bi", "Virgin Islands, British"),
        ("bn", "ব\u{9cd}রিটিশ ভ\u{9be}র\u{9cd}জিন দ\u{9cd}বীপ"),
        ("bn_IN", "ব\u{9cd}রিটিশ ভ\u{9be}র\u{9cd}জিন দ\u{9cd}বীপ"),
        ("br", "Virgin Islands, British"),
        ("bs", "Djevičanska Ostrva (Britanska)"),
        ("ca", "Illes Verges, Britàniques"),
        ("ce", "Virgin Islands, British"),
        ("ch", "Virgin Islands, British"),
        ("cs", "Panenské ostrovy, britské"),
        ("cv", "Virgin Islands, British"),
        ("cy", "Ynysoedd Virgin, Prydeinig"),
        ("da", "Britiske Jomfruøer, De"),
        ("de", "Britische Jungferninseln"),
        ("dv", "Virgin Islands, British"),
        ("dz", "ཝར་ཇ\u{f72}ན་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}་ བ\u{f72}ར\u{f72}་ཊ\u{f72}ཤ\u{f72}།"),
        ("ee", "Virgin Islands, British"),
        ("el", "Παρθένοι Νήσοι, Βρετανικές"),
        ("en", "Virgin Islands, British"),
        ("eo", "Virgulininsuloj, Britaj"),
        ("es", "Islas Vírgenes, Británicas"),
        ("et", "Briti Neitsisaared"),
        ("eu", "Birjina uharteak, Britainiarrak"),
        ("fa", "جزایر ویرجین ، بریتانیا"),
        ("ff", "Virgin Islands, British"),
        ("fi", "Neitsytsaaret, Brittiläiset"),
        ("fo", "Virgin Islands, British"),
        ("fr", "Îles Vierges britanniques"),
        ("fy", "Virgin Islands, British"),
        ("ga", "Oileáin Bhriotanacha na Maighdean"),
        ("gl", "Illas Virxes Británicas"),
        ("gn", "Virgin Islands, British"),
        ("gu", "બ\u{acd}રિટિશ વર\u{acd}જીન ટાપ\u{ac1}ઓ"),
        ("gv", "Virgin Islands, British"),
        ("ha", "Virgin Islands, British"),
        ("he", "איי הבתולה (בריטיים)"),
        ("hi", "वर\u{94d}जिन आइल\u{948}\u{902}ड\u{94d}स, ब\u{94d}रिटिश"),
        ("hr", "Djevičanski Otoci, Britanski"),
        ("ht", "Virgin Islands, British"),
        ("hu", "Brit Virgin-szigetek"),
        ("hy", "Վիրջինյան կղզիներ, Բրիտանական"),
        ("ia", "Insulas Virgine, Britannic"),
        ("id", "Kepulauan Virgin Inggris"),
        ("io", "Virgin Islands, British"),
        ("is", "Jómfrúareyjar, bresku"),
        ("it", "Isole Vergini, Regno Unito"),
        ("iu", "Virgin Islands, British"),
        ("ja", "英領ヴァージン諸島"),
        ("ka", "ვირჯინის კუნძულები, ბრიტანეთის"),
        ("ki", "Virgin Islands, British"),
        ("kk", "Виргин аралдары (Ұлыбритания)"),
        ("kl", "Virgin Islands, British"),
        ("km", "កោះ\u{200b}វ\u{17b8}ជ\u{17b8}ន ចក\u{17d2}រភពអង\u{17cb}គ\u{17d2}លេស"),
        ("kn", "ವರ\u{ccd}ಜ\u{cbf}ನ\u{ccd} ಐಲಂಡ\u{ccd}ಸ\u{ccd}, ಬ\u{ccd}ರ\u{cbf}ಟ\u{cbf}ಷ\u{ccd}"),
        ("ko", "버진 제도, 영국령"),
        ("ku", "Giravên Virjin, Brîtanî"),
        ("kv", "Virgin Islands, British"),
        ("kw", "Virgin Islands, British"),
        ("ky", "Виргин аралдары (Улуу Британия)"),
        ("lo", "Virgin Islands, British"),
        ("lt", "Mergelių salos (Britų)"),
        ("lv", "Virdžīnas Salas, Britu"),
        ("mi", "Virgin Islands, British"),
        ("mk", "Девствени острови, Британски"),
        ("ml", "വിര\u{d4d}\u{200d}ജിന\u{d4d}\u{200d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}, ബ\u{d4d}രിട\u{d4d}ടീഷ\u{d4d}"),
        ("mn", "Виржиний британи арлууд"),
        ("mr", "व\u{94d}हर\u{94d}जिन आयल\u{945}\u{902}डस\u{94d}, ब\u{94d}रिटिश"),
        ("ms", "Virgin Islands, British"),
        ("mt", "Virgin Islands, British"),
        ("my", "Virgin Islands, British"),
        ("na", "Virgin Islands, British"),
        ("nb", "Jomfruøyene (Storbritannia)"),
        ("ne", "भर\u{94d}जिन टाप\u{941}, ब\u{947}लायत"),
        ("nl", "Maagdeneilanden, Britse"),
        ("nn", "Jomfruøyane (Storbritannia)"),
        ("nv", "Virgin Islands, British"),
        ("oc", "Illas Verges, britanicas"),
        ("or", "ଭର\u{b4d}ଜ\u{b3f}ନ ଦ\u{b4d}ବୀପପ\u{b41}ଞ\u{b4d}ଜ, ବ\u{b4d}ର\u{b3f}ଟୀଶ"),
        ("pa", "ਵਿਰਗਿਨ ਟਾਪ\u{a42} ਬਰਤਾਨੀਆ"),
        ("pi", "Virgin Islands, British"),
        ("pl", "Brytyjskie Wyspy Dziewicze"),
        ("ps", "Virgin Islands, British"),
        ("pt", "Ilhas Virgens Britânicas"),
        ("pt_BR", "Ilhas Virgens Britânicas"),
        ("ro", "Insulele virgine (britanice)"),
        ("ru", "Виргинские острова (Британия)"),
        ("rw", "Ibirwa bya Virigini, Nyongereza"),
        ("sc", "Ìsulas Vèrgines, Britànnicas"),
        ("sd", "Virgin Islands, British"),
        ("si", "බ\u{dca}\u{200d}ර\u{dd2}ත\u{dcf}න\u{dca}\u{200d}ය වර\u{dca}ජ\u{dd2}න\u{dca} ද\u{dd6}පත\u{dca}"),
        ("sk", "Panenské ostrovy, Britské"),
        ("sl", "Britanski Deviški otoki"),
        ("so", "Virgin Islands, British"),
        ("sq", "Ishujt e Virgjër, Britanikë"),
        ("sr", "Девичанска острва, Британска"),
        ("sv", "Jungfruöarna, brittiska"),
        ("sw", "Virgin Islands, British"),
        ("ta", "வெர\u{bcd}ஜின\u{bcd} த\u{bc0}வுகள\u{bcd}, ப\u{bcd}ரிட\u{bcd}ட\u{bc0}ஷ\u{bcd}"),
        ("te", "వర\u{c4d}జ\u{c3f}న\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}, బ\u{c4d}ర\u{c3f}ట\u{c3f}ష\u{c4d}"),
        ("tg", "Ҷазираҳои Виргини Британия"),
        ("th", "หม\u{e39}\u{e48}เกาะบร\u{e34}ต\u{e34}ชเวอร\u{e4c}จ\u{e34}น"),
        ("ti", "Virgin Islands, British"),
        ("tk", "Wirgin adalary, Britan"),
        ("tl", "Pulong Birhen, British"),
        ("tr", "İngiliz Virgin Adaları"),
        ("tt", "Вирgин Утравлары, Британ"),
        ("ug", "ئەنگلىيەگە قاراشلىق ۋىرجىن تاقىم ئاراللىرى"),
        ("uk", "Віргінські острови (Британія)"),
        ("ur", "Virgin Islands, British"),
        ("uz", "Virgin Islands, British"),
        ("ve", "Virgin Islands, British"),
        ("vi", "Quần Đảo Vơ-chin Anh"),
        ("wa", "Iyes Viedjes britanikes"),
        ("wo", "Virgin Islands, British"),
        ("xh", "Virgin Islands, British"),
        ("yo", "Virgin Islands, British"),
        ("zh_CN", "英属维尔京群岛"),
        ("zh_HK", "英屬處女羣島"),
        ("zh_TW", "英屬維京群島"),
        ("zu", "Virgin Islands, British"),
];
    #[cfg(all(feature = "vg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 18.420695;
        pub const LONGITUDE: f64 = -64.639968;
        pub const MAX_LATITUDE: f64 = 18.7539999;
        pub const MAX_LONGITUDE: f64 = -64.2651999;
        pub const MIN_LATITUDE: f64 = 18.2899998;
        pub const MIN_LONGITUDE: f64 = -64.8775;
        pub const NORTHEAST_LATITUDE: f64 = 18.7539999;
        pub const NORTHEAST_LONGITUDE: f64 = -64.2651999;
        pub const SOUTHWEST_LATITUDE: f64 = 18.2899998;
        pub const SOUTHWEST_LONGITUDE: f64 = -64.8775;
    }
}
#[cfg(all(feature = "vg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 18.420695,
            longitude: -64.639968,
            max_latitude: 18.7539999,
            max_longitude: -64.2651999,
            min_latitude: 18.2899998,
            min_longitude: -64.8775,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 18.7539999,
                    longitude: -64.2651999,
                },
                southwest: CountryGeoBound {
                    latitude: 18.2899998,
                    longitude: -64.8775,
                },
            },
        }
    }
}

#[cfg(all(feature = "vg", feature = "subdivisions"))]
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "vg")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::VG,
        alpha3: Alpha3::VGB,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: CurrencyCode::USD,
        gec: Some(GEC::VI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::IVB),
        iso_long_name: "The Virgin Islands",
        iso_short_name: "Virgin Islands (British)",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Virgin Islander"),
        number: "092",
        postal_code: true,
        postal_code_format: Some("VG\\d{4}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "VG",
        unofficial_name_list: ["British Virgin Islands", "Britische Jungferninseln", "Îles Vierges britanniques", "Islas Vírgenes del Reino Unido", "イギリス領ヴァージン諸島", "Britse Maagdeneilanden", "Virgin Islands (British)"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Virgin Islands, British"), ("af", "Maagde-eilande, Britse"), ("ak", "Virgin Islands, British"), ("am", "ፘጕንጔሑፔ ፥ንጔሔ ፠ሴቶኄ"), ("an", "Virgin Islands, British"), ("ar", "فيرجن، جزر فيرجن البريطاني\u{651}ة"), ("as", "ব\u{9cd}ৰিটিশ\u{9cd}ব ভ\u{9be}ৰ\u{9cd}জিন দ\u{9cd}বীপ"), ("ay", "Virgin Islands, British"), ("az", "Virgin Adaları, Britaniya"), ("ba", "Virgin Islands, British"), ("be", "Віргінскія астравы Вялікабрытаніі"), ("bg", "Вирджински острови, Британски"), ("bi", "Virgin Islands, British"), ("bn", "ব\u{9cd}রিটিশ ভ\u{9be}র\u{9cd}জিন দ\u{9cd}বীপ"), ("bn_IN", "ব\u{9cd}রিটিশ ভ\u{9be}র\u{9cd}জিন দ\u{9cd}বীপ"), ("br", "Virgin Islands, British"), ("bs", "Djevičanska Ostrva (Britanska)"), ("ca", "Illes Verges, Britàniques"), ("ce", "Virgin Islands, British"), ("ch", "Virgin Islands, British"), ("cs", "Panenské ostrovy, britské"), ("cv", "Virgin Islands, British"), ("cy", "Ynysoedd Virgin, Prydeinig"), ("da", "Britiske Jomfruøer, De"), ("de", "Britische Jungferninseln"), ("dv", "Virgin Islands, British"), ("dz", "ཝར་ཇ\u{f72}ན་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}ས\u{f72}་ བ\u{f72}ར\u{f72}་ཊ\u{f72}ཤ\u{f72}།"), ("ee", "Virgin Islands, British"), ("el", "Παρθένοι Νήσοι, Βρετανικές"), ("en", "Virgin Islands, British"), ("eo", "Virgulininsuloj, Britaj"), ("es", "Islas Vírgenes, Británicas"), ("et", "Briti Neitsisaared"), ("eu", "Birjina uharteak, Britainiarrak"), ("fa", "جزایر ویرجین ، بریتانیا"), ("ff", "Virgin Islands, British"), ("fi", "Neitsytsaaret, Brittiläiset"), ("fo", "Virgin Islands, British"), ("fr", "Îles Vierges britanniques"), ("fy", "Virgin Islands, British"), ("ga", "Oileáin Bhriotanacha na Maighdean"), ("gl", "Illas Virxes Británicas"), ("gn", "Virgin Islands, British"), ("gu", "બ\u{acd}રિટિશ વર\u{acd}જીન ટાપ\u{ac1}ઓ"), ("gv", "Virgin Islands, British"), ("ha", "Virgin Islands, British"), ("he", "איי הבתולה (בריטיים)"), ("hi", "वर\u{94d}जिन आइल\u{948}\u{902}ड\u{94d}स, ब\u{94d}रिटिश"), ("hr", "Djevičanski Otoci, Britanski"), ("ht", "Virgin Islands, British"), ("hu", "Brit Virgin-szigetek"), ("hy", "Վիրջինյան կղզիներ, Բրիտանական"), ("ia", "Insulas Virgine, Britannic"), ("id", "Kepulauan Virgin Inggris"), ("io", "Virgin Islands, British"), ("is", "Jómfrúareyjar, bresku"), ("it", "Isole Vergini, Regno Unito"), ("iu", "Virgin Islands, British"), ("ja", "英領ヴァージン諸島"), ("ka", "ვირჯინის კუნძულები, ბრიტანეთის"), ("ki", "Virgin Islands, British"), ("kk", "Виргин аралдары (Ұлыбритания)"), ("kl", "Virgin Islands, British"), ("km", "កោះ\u{200b}វ\u{17b8}ជ\u{17b8}ន ចក\u{17d2}រភពអង\u{17cb}គ\u{17d2}លេស"), ("kn", "ವರ\u{ccd}ಜ\u{cbf}ನ\u{ccd} ಐಲಂಡ\u{ccd}ಸ\u{ccd}, ಬ\u{ccd}ರ\u{cbf}ಟ\u{cbf}ಷ\u{ccd}"), ("ko", "버진 제도, 영국령"), ("ku", "Giravên Virjin, Brîtanî"), ("kv", "Virgin Islands, British"), ("kw", "Virgin Islands, British"), ("ky", "Виргин аралдары (Улуу Британия)"), ("lo", "Virgin Islands, British"), ("lt", "Mergelių salos (Britų)"), ("lv", "Virdžīnas Salas, Britu"), ("mi", "Virgin Islands, British"), ("mk", "Девствени острови, Британски"), ("ml", "വിര\u{d4d}\u{200d}ജിന\u{d4d}\u{200d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}, ബ\u{d4d}രിട\u{d4d}ടീഷ\u{d4d}"), ("mn", "Виржиний британи арлууд"), ("mr", "व\u{94d}हर\u{94d}जिन आयल\u{945}\u{902}डस\u{94d}, ब\u{94d}रिटिश"), ("ms", "Virgin Islands, British"), ("mt", "Virgin Islands, British"), ("my", "Virgin Islands, British"), ("na", "Virgin Islands, British"), ("nb", "Jomfruøyene (Storbritannia)"), ("ne", "भर\u{94d}जिन टाप\u{941}, ब\u{947}लायत"), ("nl", "Maagdeneilanden, Britse"), ("nn", "Jomfruøyane (Storbritannia)"), ("nv", "Virgin Islands, British"), ("oc", "Illas Verges, britanicas"), ("or", "ଭର\u{b4d}ଜ\u{b3f}ନ ଦ\u{b4d}ବୀପପ\u{b41}ଞ\u{b4d}ଜ, ବ\u{b4d}ର\u{b3f}ଟୀଶ"), ("pa", "ਵਿਰਗਿਨ ਟਾਪ\u{a42} ਬਰਤਾਨੀਆ"), ("pi", "Virgin Islands, British"), ("pl", "Brytyjskie Wyspy Dziewicze"), ("ps", "Virgin Islands, British"), ("pt", "Ilhas Virgens Britânicas"), ("pt_BR", "Ilhas Virgens Britânicas"), ("ro", "Insulele virgine (britanice)"), ("ru", "Виргинские острова (Британия)"), ("rw", "Ibirwa bya Virigini, Nyongereza"), ("sc", "Ìsulas Vèrgines, Britànnicas"), ("sd", "Virgin Islands, British"), ("si", "බ\u{dca}\u{200d}ර\u{dd2}ත\u{dcf}න\u{dca}\u{200d}ය වර\u{dca}ජ\u{dd2}න\u{dca} ද\u{dd6}පත\u{dca}"), ("sk", "Panenské ostrovy, Britské"), ("sl", "Britanski Deviški otoki"), ("so", "Virgin Islands, British"), ("sq", "Ishujt e Virgjër, Britanikë"), ("sr", "Девичанска острва, Британска"), ("sv", "Jungfruöarna, brittiska"), ("sw", "Virgin Islands, British"), ("ta", "வெர\u{bcd}ஜின\u{bcd} த\u{bc0}வுகள\u{bcd}, ப\u{bcd}ரிட\u{bcd}ட\u{bc0}ஷ\u{bcd}"), ("te", "వర\u{c4d}జ\u{c3f}న\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}, బ\u{c4d}ర\u{c3f}ట\u{c3f}ష\u{c4d}"), ("tg", "Ҷазираҳои Виргини Британия"), ("th", "หม\u{e39}\u{e48}เกาะบร\u{e34}ต\u{e34}ชเวอร\u{e4c}จ\u{e34}น"), ("ti", "Virgin Islands, British"), ("tk", "Wirgin adalary, Britan"), ("tl", "Pulong Birhen, British"), ("tr", "İngiliz Virgin Adaları"), ("tt", "Вирgин Утравлары, Британ"), ("ug", "ئەنگلىيەگە قاراشلىق ۋىرجىن تاقىم ئاراللىرى"), ("uk", "Віргінські острови (Британія)"), ("ur", "Virgin Islands, British"), ("uz", "Virgin Islands, British"), ("ve", "Virgin Islands, British"), ("vi", "Quần Đảo Vơ-chin Anh"), ("wa", "Iyes Viedjes britanikes"), ("wo", "Virgin Islands, British"), ("xh", "Virgin Islands, British"), ("yo", "Virgin Islands, British"), ("zh_CN", "英属维尔京群岛"), ("zh_HK", "英屬處女羣島"), ("zh_TW", "英屬維京群島"), ("zu", "Virgin Islands, British")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
