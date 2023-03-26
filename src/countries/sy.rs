// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Syrian Arab Republic

#[cfg(all(feature = "sy", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SY;
    pub const ALPHA3: Alpha3 = Alpha3::SYR;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 963;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::SYP;
    pub const GEC: Option<GEC> = Some(GEC::SY);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::SYR);
    pub const ISO_SHORT_NAME: &str = "Syrian Arab Republic";
    pub const ISO_LONG_NAME: &str = "The Syrian Arab Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Syrian");
    pub const NUMBER: &str = "760";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "SY";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Syria",
        "سوريا",
        "سورية",
        "Syrien",
        "Syrie",
        "Siria",
        "シリア・アラブ共和国",
        "Syrië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Syrian Arab Republic"),
        ("af", "Siriese Arabiese Republiek"),
        ("ak", "Syrian Arab Republic"),
        ("am", "ሲሱ።"),
        ("an", "Syrian Arab Republic"),
        ("ar", "سوريا"),
        ("as", "চিৰিয়\u{9be}ন আৰব প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"),
        ("ay", "Syrian Arab Republic"),
        ("az", "Syrian Arab Republic"),
        ("ba", "Syrian Arab Republic"),
        ("be", "Сірыйская Арабская Рэспубліка"),
        ("bg", "Сирийска арабска република"),
        ("bi", "Syrian Arab Republic"),
        ("bn", "সিরিয়\u{9be}ন আরব প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("bn_IN", "সিরিয়\u{9be}ন আরব প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("br", "Syrian Arab Republic"),
        ("bs", "Sirijska Arapska Republika"),
        ("ca", "República Àrab Síria"),
        ("ce", "Syrian Arab Republic"),
        ("ch", "Syrian Arab Republic"),
        ("cs", "Syrská arabská republika"),
        ("cv", "Syrian Arab Republic"),
        ("cy", "Gweriniaeth Arabaidd Syria"),
        ("da", "Syriske Arabiske Republik"),
        ("de", "Syrien, Arabische Republik"),
        ("dv", "Syrian Arab Republic"),
        ("dz", "ས\u{f72}་ར\u{f7a}ན་ ཨ་རབ་ མ\u{f72}་ས\u{f7a}ར་ར\u{f92}\u{fb1}ལ་ཁབ།"),
        ("ee", "Syrian Arab Republic"),
        ("el", "Αραβική Δημοκρατία της Συρίας"),
        ("en", "Syrian Arab Republic"),
        ("eo", "Siria Araba Respubliko"),
        ("es", "República árabe de Siria"),
        ("et", "Süüria Araabia Vabariik"),
        ("eu", "Siriako Arabiar Errepublika"),
        ("fa", "جمهوری عربی سوریه"),
        ("ff", "Syrian Arab Republic"),
        ("fi", "Syyrian arabitasavalta"),
        ("fo", "Syrian Arab Republic"),
        ("fr", "Syrienne, République arabe"),
        ("fy", "Syrian Arab Republic"),
        ("ga", "Poblacht Arabach na Siria"),
        ("gl", "República Árabe de Siria"),
        ("gn", "Syrian Arab Republic"),
        ("gu", "સીરીયન આરબ રીપબ\u{acd}લિક"),
        ("gv", "Syrian Arab Republic"),
        ("ha", "Syrian Arab Republic"),
        ("he", "הרפובליקה הערבית הסורית"),
        ("hi", "सीरियन अरब रिपब\u{94d}लिक"),
        ("hr", "Sirijska Arapska Republika"),
        ("ht", "Syrian Arab Republic"),
        ("hu", "Szíriai Arab Köztársaság"),
        ("hy", "Սիրիայի Արաբական Հանրապետություն"),
        ("ia", "Republica Arabe Syrie"),
        ("id", "Republik Arab Syria"),
        ("io", "Syrian Arab Republic"),
        ("is", "Sýrlenska arabalýðveldið"),
        ("it", "Siria"),
        ("iu", "Syrian Arab Republic"),
        ("ja", "シリア・アラブ共和国"),
        ("ka", "სირიის არაბული რესპუბლიკა"),
        ("ki", "Syrian Arab Republic"),
        ("kk", "Сирия Араб Республикасы"),
        ("kl", "Syrian Arab Republic"),
        ("km", "សាធារណរដ\u{17d2}ឋ\u{200b}ស\u{17ca}\u{17b8}រ\u{17b8}យ\u{17c9}ា\u{200b}\u{200b}អារាប\u{17cb}"),
        ("kn", "ಸ\u{cbf}ರ\u{cbf}ಯನ\u{ccd} ಅರಬ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"),
        ("ko", "시리아 아랍 공화국"),
        ("ku", "Sûriye"),
        ("kv", "Syrian Arab Republic"),
        ("kw", "Syrian Arab Republic"),
        ("ky", "Сирия Араб Республикасы"),
        ("lo", "Syrian Arab Republic"),
        ("lt", "Sirijos Arabų Respublika"),
        ("lv", "Sīrija"),
        ("mi", "Syrian Arab Republic"),
        ("mk", "Сирија арапска република"),
        ("ml", "സിറിയന\u{d4d}\u{200d} അറബ\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d}"),
        ("mn", "Syrian Arab Republic"),
        ("mr", "सिरियन अरब रिपब\u{94d}लिक"),
        ("ms", "Syrian Arab Republic"),
        ("mt", "Syrian Arab Republic"),
        ("my", "Syrian Arab Republic"),
        ("na", "Syrian Arab Republic"),
        ("nb", "Den arabiske republikk Syria"),
        ("ne", "सिरियन अरब गणराज\u{94d}य"),
        ("nl", "Syrië"),
        ("nn", "Syria"),
        ("nv", "Syrian Arab Republic"),
        ("oc", "Republica Dominicana"),
        ("or", "ସ\u{b3f}ର\u{b3f}ଆନ ଆରବ ଗଣତନ\u{b4d}ତ\u{b4d}ର"),
        ("pa", "ਸੀਰੀਅਨ ਅਰਬ ਗਣਰਾਜ"),
        ("pi", "Syrian Arab Republic"),
        ("pl", "Syryjska Republika Arabska"),
        ("ps", "Syrian Arab Republic"),
        ("pt", "República Árabe Síria"),
        ("pt_BR", "República Árabe da Síria"),
        ("ro", "Republica Araba Siria"),
        ("ru", "Сирийская Арабская Республика"),
        ("rw", "Repubulika Nyarabu ya Siriya"),
        ("sc", "Repùblica Àraba de Sìria"),
        ("sd", "Syrian Arab Republic"),
        ("si", "ස\u{dd2}ර\u{dd2}ය\u{dcf}න\u{dd4} අර\u{dcf}බ\u{dd2} ජනරජය"),
        ("sk", "Sýrska arabská republika"),
        ("sl", "Sirska arabska republika"),
        ("so", "Suuriya"),
        ("sq", "Republika Arabe e Sirisë"),
        ("sr", "Сиријска Арапска Република"),
        ("sv", "Syriska arabrepubliken"),
        ("sw", "Syrian Arab Republic"),
        ("ta", "சிரியன\u{bcd} அரபு குடியரசு"),
        ("te", "స\u{c3f}ర\u{c3f}యన\u{c4d} అరబ\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"),
        ("tg", "Ҷумҳурии Сурияи Араб"),
        ("th", "สาธารณร\u{e31}ฐอาหร\u{e31}บซ\u{e35}เร\u{e35}ย"),
        ("ti", "Syrian Arab Republic"),
        ("tk", "Siriýa Arap Respublikasy"),
        ("tl", "Syrian Arab Republika"),
        ("tr", "Suriye Arap Cumhuriyeti"),
        ("tt", "Гәрәп Сүриә Җөмһүриәте"),
        ("ug", "سۈرىيە ئەرەب جۇمھۇرىيىتى"),
        ("uk", "Сирійська Арабська Республіка"),
        ("ur", "Syrian Arab Republic"),
        ("uz", "Syrian Arab Republic"),
        ("ve", "Syrian Arab Republic"),
        ("vi", "Cộng hoà A-rập Xi-ri-a"),
        ("wa", "Sireye"),
        ("wo", "Republik Araab bu Siiri"),
        ("xh", "Syrian Arab Republic"),
        ("yo", "Syrian Arab Republic"),
        ("zh_CN", "叙利亚"),
        ("zh_HK", "阿拉伯敍利亞共和國"),
        ("zh_TW", "敘利亞阿拉伯共和國"),
        ("zu", "Syrian Arab Republic"),
];
    #[cfg(all(feature = "sy", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 34.80207499999999;
        pub const LONGITUDE: f64 = 38.996815;
        pub const MAX_LATITUDE: f64 = 37.318693;
        pub const MAX_LONGITUDE: f64 = 42.376309;
        pub const MIN_LATITUDE: f64 = 32.311136;
        pub const MIN_LONGITUDE: f64 = 35.62869999999999;
        pub const NORTHEAST_LATITUDE: f64 = 37.318693;
        pub const NORTHEAST_LONGITUDE: f64 = 42.376309;
        pub const SOUTHWEST_LATITUDE: f64 = 32.311136;
        pub const SOUTHWEST_LONGITUDE: f64 = 35.62869999999999;
    }
}
#[cfg(all(feature = "sy", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 34.80207499999999,
            longitude: 38.996815,
            max_latitude: 37.318693,
            max_longitude: 42.376309,
            min_latitude: 32.311136,
            min_longitude: 35.62869999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 37.318693,
                    longitude: 42.376309,
                },
                southwest: CountryGeoBound {
                    latitude: 32.311136,
                    longitude: 35.62869999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "sy", feature = "subdivisions"))]
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
                    "DI",
                    Subdivision{
                        name: "Dimashq",
                        country_alpha2: Alpha2::SY,
                        code: "DI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.513), longitude: Some(36.292), max_latitude: Some(33.5651386), min_latitude: Some(33.4498124), max_longitude: Some(36.3678997), min_longitude: Some(36.1978912)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دمشق"), ("be", "Мухафаза Дамаск"), ("bg", "Дамаск"), ("ca", "Governació de Damasc"), ("ccp", "𑄓𑄟𑄌\u{11134}𑄇𑄌\u{11134}"), ("ceb", "Damascus Governorate"), ("de", "Gouvernement Damaskus"), ("el", "Δαμασκός"), ("en", "Damascus"), ("es", "Gobernación de Damasco"), ("eu", "Damasko eskualdea"), ("fa", "استان دمشق"), ("fi", "Damaskoksen maakunta"), ("fr", "Gouvernorat de Damas"), ("hi", "दमिश\u{94d}क\u{93c} प\u{94d}रान\u{94d}त"), ("hy", "Դամասկոսի մարզ"), ("id", "Kegubernuran Damaskus"), ("it", "governatorato di Damasco"), ("ja", "ダマスカス県"), ("ka", "დამასკის მუჰაფაზა"), ("ko", "다마스쿠스 주"), ("nb", "Damaskus"), ("nl", "Damascus Governorate"), ("no", "Damaskus"), ("pl", "Damaszek-Miasto"), ("pt", "Damasco"), ("ro", "Guvernoratul Damasc"), ("ru", "Дамаск"), ("tr", "Şam"), ("ur", "محافظہ دمشق"), ("zh", "大馬士革省 (敘利亞)")]),
                        unofficial_name_list: ["Damas", "Damaskus", "Dimashq", "Madīnat Dimašq"].to_vec(),
                    }
                ),
                (
                    "DR",
                    Subdivision{
                        name: "Dar'ā",
                        country_alpha2: Alpha2::SY,
                        code: "DR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.625278), longitude: Some(36.106111), max_latitude: Some(32.6466746), min_latitude: Some(32.5963599), max_longitude: Some(36.1277676), min_longitude: Some(36.0721494)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة درعا"), ("bg", "Дараа"), ("ca", "Governació de Daraa"), ("ccp", "𑄓𑄢\u{11133}𑄃"), ("ceb", "Daraa Governorate"), ("cs", "Guvernorát Dar’á"), ("de", "Gouvernement Dar’a"), ("el", "Νταράα"), ("en", "Daraa"), ("es", "Gobernación de Dar’a"), ("eu", "Daraa eskualdea"), ("fa", "استان درعا"), ("fi", "Dar’an maakunta"), ("fr", "Gouvernorat de Deraa"), ("he", "מחוז דרעא"), ("hi", "दरआ प\u{94d}रान\u{94d}त"), ("hy", "Դառաայի մարզ"), ("id", "Kegubernuran Daraa"), ("it", "governatorato di Dar’a"), ("ja", "ダルアー県"), ("ka", "დარის მუჰაფაზა"), ("ko", "다라 주"), ("lt", "Daros muchafaza"), ("nl", "Daraa"), ("pl", "Dara"), ("pt", "Dara"), ("ro", "Guvernoratul Daraa"), ("ru", "Деръа"), ("sr", "Дара (покрајина)"), ("sr_Latn", "Dara (pokrajina)"), ("sv", "Dar’a"), ("tr", "Dera"), ("uk", "Дара"), ("ur", "محافظہ درعا"), ("vi", "Daraa"), ("zh", "德拉省")]),
                        unofficial_name_list: ["Dara", "Darā", "Darʿa", "Deraa", "Dārayyā"].to_vec(),
                    }
                ),
                (
                    "DY",
                    Subdivision{
                        name: "Dayr az Zawr",
                        country_alpha2: Alpha2::SY,
                        code: "DY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.336), longitude: Some(40.145), max_latitude: Some(35.3734271), min_latitude: Some(35.2968173), max_longitude: Some(40.1946509), min_longitude: Some(40.0813007)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دير الزور"), ("az", "Deyr əz-Zor mühafazası"), ("bg", "Дейр ез-Зор"), ("bn", "ডিএর এজ-জর গভর\u{9cd}নোরেট"), ("ca", "Governació de Deir ez-Zor"), ("ccp", "𑄓𑄠𑄬𑄢\u{11134} 𑄃𑄬𑄌\u{11134}-𑄎\u{1112e}𑄢\u{11134}"), ("ceb", "Deir ez-Zor Governorate"), ("cs", "Guvernorát Dajr az-Zaur"), ("da", "Deir ez-Zor Governorate"), ("de", "Gouvernement Deir ez-Zor"), ("el", "Ντέιρ εζ-Ζορ"), ("en", "Deir ez-Zor"), ("es", "Gobernación de Deir ez-Zor"), ("eu", "Dayr az Zawr eskualdea"), ("fa", "استان دیرالزور"), ("fi", "Dayr al-Zawrin maakunta"), ("fr", "Gouvernorat de Deir ez-Zor"), ("gu", "ડ\u{ac7}ઇર ઇઝ-ઝોર ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז דיר א-זור"), ("hi", "द\u{947}इर अज\u{93c}-ज\u{93c}ोर प\u{94d}रान\u{94d}त"), ("hu", "Dajr ez-Zaur kormányzóság"), ("hy", "Դեյր-էլ-Զորի մարզ"), ("id", "Kegubernuran Dayr az-Zawr"), ("it", "governatorato di Deir el-Zor"), ("ja", "デリゾール県"), ("ka", "დეირ-ელ-ზორის მუჰაფაზა"), ("kn", "ಡ\u{cbf}ಯರ\u{ccd} ಇಜ\u{ccd}-ಝೋರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "데이르에조르 주"), ("lt", "Deir az Zoro muchafaza"), ("lv", "Deir ez Zoras muhāfaza"), ("mr", "द\u{947}अर एझ-झोर गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Deir ez-Zor Governorate"), ("nb", "Dayr az-Zawr"), ("nl", "Deir ez-Zor"), ("no", "Dayr az-Zawr"), ("pl", "Dajr az-Zaur"), ("pt", "Deir ez-Zor"), ("ro", "Guvernoratul Deir ez-Zor"), ("ru", "Дейр-эз-Зор"), ("si", "ඩය\u{dd2}ර\u{dca} එස\u{dca}-සෝර\u{dca} පළ\u{dcf}ත"), ("sr", "Дајр ез Заур (покрајина)"), ("sr_Latn", "Dajr ez Zaur (pokrajina)"), ("sv", "Dayr az-Zawr"), ("ta", "டேய\u{bbe}ர\u{bcd} எஸ\u{bcd}-சொற\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "డ\u{c3f}యర\u{c4d} ఈజ\u{c40}-జ\u{c4b}ర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ไดร\u{e4c}อ\u{e31}ซเซาร\u{e4c}"), ("tr", "Deyrizor"), ("uk", "Дайр-аз-Заур"), ("ur", "محافظہ دیر الزور"), ("vi", "Deir ez-Zor"), ("zh", "代尔祖尔省")]),
                        unofficial_name_list: ["Deir El-Zor", "Deir-ez-Zor"].to_vec(),
                    }
                ),
                (
                    "HA",
                    Subdivision{
                        name: "Al Ḩasakah",
                        country_alpha2: Alpha2::SY,
                        code: "HA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.405515), longitude: Some(40.7969149), max_latitude: Some(37.319145), min_latitude: Some(35.5298381), max_longitude: Some(42.3850397), min_longitude: Some(39.452599)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الحسكة"), ("az", "Əl-Həsəkə mühafazası"), ("bg", "Ал-Хасеке"), ("bn", "আল-হ\u{9be}স\u{9be}ক\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació d’Al-Hasakah"), ("ccp", "𑄃𑄣\u{11134}-𑄝𑄥𑄇\u{11134}𑄦\u{11134}"), ("ceb", "Al-Hasakah Governorate"), ("cs", "Guvernorát Hasaka"), ("da", "Al-Hasakah Governorate"), ("de", "Gouvernement al-Hasaka"), ("el", "Αλ Χασακά"), ("en", "Al-Hasakah"), ("es", "Gobernación de Hasaka"), ("eu", "Al-Hasakah eskualdea"), ("fa", "استان حسکه"), ("fi", "Al-Hasakan maakunta"), ("fr", "Gouvernorat d’al-Hasaka"), ("gu", "અલ-હસાકાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז אל-חסכה"), ("hi", "अल-हसकाह प\u{94d}रान\u{94d}त"), ("hu", "Haszaka kormányzóság"), ("hy", "Ալ-Հասաքայի մարզ"), ("id", "Kegubernuran Al-Hasakah"), ("it", "governatorato di al-Hasaka"), ("ja", "ハサカ県"), ("ka", "ალ-ჰასაქის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd}-ಹಸಾಕ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "하사카 주"), ("lt", "Hasekės muchafaza"), ("lv", "Hasekas muhāfaza"), ("mr", "अल-हसाका गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al-Hasakah Governorate"), ("nb", "Al-Hasakah"), ("nl", "Al-Hasakah"), ("no", "Al-Hasakah"), ("pl", "Al-Hasaka"), ("pt", "Al-Hasakah"), ("ro", "Guvernoratul Al-Hasaka"), ("ru", "Хасеке"), ("si", "අල\u{dca} හසක\u{dcf} පළ\u{dcf}ත"), ("sr", "Хасака (покрајина)"), ("sr_Latn", "Hasaka (pokrajina)"), ("sv", "Al-Hasakah"), ("ta", "அல\u{bcd}-ஹஸக\u{bcd}க\u{bbe}ஹ\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "అల\u{c4d}-హసక\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลฮะซะกะห\u{e4c}"), ("tr", "Hasiçi"), ("uk", "Аль-Хасака"), ("ur", "محافظہ الحسکہ"), ("vi", "Al-Hasakah"), ("zh", "哈塞克省")]),
                        unofficial_name_list: ["El Haseke", "Hassetche", "H\u{328}asakah", "al-Hasakah"].to_vec(),
                    }
                ),
                (
                    "HI",
                    Subdivision{
                        name: "Ḩimş",
                        country_alpha2: Alpha2::SY,
                        code: "HI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.7348116), longitude: Some(-117.9947778), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حمص"), ("az", "Hims mühafazası"), ("be", "Хомс"), ("bg", "Хомс"), ("bn", "হোমস গভর\u{9cd}নোরেট"), ("ca", "Governació d’Homs"), ("ccp", "𑄦\u{11127}𑄟\u{11134}𑄌\u{11134}"), ("ceb", "Homs Governorate"), ("cs", "Guvernorát Homs"), ("da", "Homs"), ("de", "Gouvernement Homs"), ("el", "Χομς"), ("en", "Homs"), ("es", "Gobernación de Homs"), ("eu", "Homs eskualdea"), ("fa", "استان حمص"), ("fi", "Homsin maakunta"), ("fr", "Gouvernorat de Homs"), ("gu", "હોમ\u{acd}સ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז חומס"), ("hi", "होम\u{94d}स प\u{94d}रान\u{94d}त"), ("hu", "Homsz kormányzóság"), ("hy", "Հոմսի մարզ"), ("id", "Kegubernuran Homs"), ("is", "Homshérað"), ("it", "governatorato di Homs"), ("ja", "ホムス県"), ("ka", "ჰომსის მუჰაფაზა"), ("kn", "ಹಾಮ\u{ccd}ಸ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "홈스 주"), ("lt", "Homso muchafaza"), ("lv", "Himsas munāfaza"), ("mr", "होम\u{94d}स गोव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Homs"), ("nb", "Homs"), ("nl", "Homs"), ("no", "Homs"), ("pl", "Hims"), ("pt", "Homs"), ("ro", "Guvernoratul Homs"), ("ru", "Хомс"), ("si", "හෝම\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Хомс (покрајина)"), ("sr_Latn", "Homs (pokrajina)"), ("sv", "Homs"), ("ta", "ஹோம\u{bcd}ஸ\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "హ\u{c3e}మ\u{c4d}స\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองฮ\u{e34}มส\u{e4c}"), ("tr", "Humus"), ("uk", "Хомс"), ("ur", "محافظہ حمص"), ("vi", "Homs"), ("zh", "霍姆斯省")]),
                        unofficial_name_list: ["Hims", "Homs"].to_vec(),
                    }
                ),
                (
                    "HL",
                    Subdivision{
                        name: "Ḩalab",
                        country_alpha2: Alpha2::SY,
                        code: "HL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.9166667), longitude: Some(38.3), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حلب"), ("az", "Hələb mühafazası"), ("be", "Халеб"), ("bg", "Халеб"), ("ca", "Governació d’Alep"), ("ccp", "𑄃𑄣\u{11128}𑄛\u{11133}𑄦\u{11127}"), ("ceb", "Aleppo Governorate"), ("cs", "Aleppská muháfaza"), ("da", "Aleppo"), ("de", "Gouvernement Aleppo"), ("el", "Αλέππο"), ("en", "Aleppo"), ("es", "Gobernación de Alepo"), ("eu", "Alepo eskualdea"), ("fa", "استان حلب"), ("fi", "Aleppon maakunta"), ("fr", "Gouvernorat d’Alep"), ("he", "מחוז חלב"), ("hi", "हलब प\u{94d}रान\u{94d}त"), ("hu", "Aleppó kormányzóság"), ("hy", "Հալեպի մարզ"), ("id", "Kegubernuran Aleppo"), ("it", "governatorato di Aleppo"), ("ja", "アレッポ県"), ("ka", "ალეპოს მუჰაფაზა"), ("ko", "알레포 주"), ("lt", "Alepo muchafaza"), ("nb", "Aleppo"), ("nl", "Aleppo"), ("no", "Aleppo"), ("pl", "Aleppo"), ("pt", "Alepo"), ("ro", "Guvernoratul Alep"), ("ru", "Халеб"), ("sr", "Алеп (покрајина)"), ("sr_Latn", "Alep (pokrajina)"), ("sv", "Aleppo"), ("tr", "Halep"), ("uk", "Халеб"), ("ur", "محافظہ حلب"), ("vi", "Aleppo"), ("zh", "阿勒颇省")]),
                        unofficial_name_list: ["Alep", "Halab", "Haleb", "H\u{328}alab"].to_vec(),
                    }
                ),
                (
                    "HM",
                    Subdivision{
                        name: "Ḩamāh",
                        country_alpha2: Alpha2::SY,
                        code: "HM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.8947222), longitude: Some(37.12527780000001), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حماة"), ("az", "Həma mühafazası"), ("bg", "Хама"), ("bn", "হ\u{9be}ম\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació d’Hama"), ("ccp", "𑄦𑄟"), ("ceb", "Hama Governorate"), ("cs", "Hamá"), ("da", "Hama Governorate"), ("de", "Gouvernement Hama"), ("el", "Χάμα"), ("en", "Hama"), ("es", "Gobernación de Hama"), ("eu", "Hama eskualdea"), ("fa", "استان حمات"), ("fi", "Haman maakunta"), ("fr", "Gouvernorat de Hama"), ("gu", "હમા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז חמה"), ("hi", "हमा प\u{94d}रान\u{94d}त"), ("hu", "Hamá kormányzóság"), ("hy", "Համայի մարզ"), ("id", "Kegubernuran Hama"), ("is", "Hamahérað"), ("it", "governatorato di Hama"), ("ja", "ハマー県"), ("ka", "ჰამის მუჰაფაზა"), ("kn", "ಹಮಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "하마 주"), ("lt", "Hamos muchafaza"), ("lv", "Hamas muhāfaza"), ("mr", "हमा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Hama Governorate"), ("nb", "Guvernementet Hama"), ("nl", "Hama"), ("no", "Guvernementet Hama"), ("pl", "Hama"), ("pt", "Hama"), ("ro", "Guvernoratul Hama"), ("ru", "Хама"), ("si", "හම\u{dcf} පළ\u{dcf}ත"), ("sr", "Хама (покрајина)"), ("sr_Latn", "Hama (pokrajina)"), ("sv", "Guvernementet Hamah"), ("ta", "ஹம\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "హమ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ฮามาห\u{e4c}"), ("tr", "Hama"), ("uk", "Хама"), ("ur", "محافظہ حماہ"), ("vi", "Hama"), ("zh", "哈馬省")]),
                        unofficial_name_list: ["Hama", "Hamah"].to_vec(),
                    }
                ),
                (
                    "ID",
                    Subdivision{
                        name: "Idlib",
                        country_alpha2: Alpha2::SY,
                        code: "ID",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.933333), longitude: Some(36.633333), max_latitude: Some(35.9471799), min_latitude: Some(35.90606289999999), max_longitude: Some(36.6674555), min_longitude: Some(36.6036813)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة إدلب"), ("az", "İdlib mühafazası"), ("bg", "Идлиб"), ("bn", "ইদলিব গভর\u{9cd}নোরেট"), ("ca", "Governació d’Idlib"), ("ccp", "𑄃\u{11128}𑄖\u{11134}𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Idlib Governorate"), ("cs", "Guvernorát Idlib"), ("da", "Idlib Governorate"), ("de", "Gouvernement Idlib"), ("el", "Ιντλίμπ"), ("en", "Idlib"), ("es", "Gobernación de Idlib"), ("eu", "Idlib eskualdea"), ("fa", "استان ادلب"), ("fi", "Idlibin maakunta"), ("fr", "Gouvernorat d’Idleb"), ("gu", "ઇડલિબ ગ\u{ac3}હારીત"), ("he", "מחוז אדלב"), ("hi", "इदलिब प\u{94d}रान\u{94d}त"), ("hu", "Idlib kormányzóság"), ("hy", "Իդլիբի մարզ"), ("id", "Kegubernuran Idlib"), ("it", "governatorato di Idlib"), ("ja", "イドリブ県"), ("ka", "იდლიბის მუჰაფაზა"), ("kn", "ಇಡ\u{ccd}ಲ\u{cbf}ಬ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "이들리브 주"), ("lt", "Idlibo muchafaza"), ("lv", "Idlibas munāfaza"), ("mr", "इडलीब गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Idlib Governorate"), ("nb", "Idlib"), ("nl", "Idlib"), ("no", "Idlib"), ("pl", "Idlib"), ("pt", "Idlib"), ("ro", "Guvernoratul Idlib"), ("ru", "Идлиб"), ("si", "ඉද\u{dca}ල\u{dd2}බ\u{dca} පළ\u{dcf}ත"), ("sr", "Идлиб (покрајина)"), ("sr_Latn", "Idlib (pokrajina)"), ("sv", "Idlib"), ("ta", "இட\u{bcd}லிப\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ఇడ\u{c4d}ల\u{c3f}బ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e34}ดล\u{e34}บ"), ("tr", "İdlip"), ("uk", "Ідліб"), ("ur", "محافظہ ادلب"), ("vi", "Idlib"), ("zh", "伊德利卜省")]),
                        unofficial_name_list: ["Idlib"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "Al Ladhiqiyah",
                        country_alpha2: Alpha2::SY,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.6129791), longitude: Some(36.0023225), max_latitude: Some(35.940891), min_latitude: Some(35.194528), max_longitude: Some(36.2848139), min_longitude: Some(35.7272339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة اللاذقية"), ("az", "Latakiya mühafazası"), ("bg", "Латакия"), ("ca", "Governació de Latakia"), ("ccp", "𑄣𑄑𑄇\u{11128}𑄠"), ("ceb", "Latakia Governorate"), ("cs", "Guvernorát Latákia"), ("de", "Gouvernement Latakia"), ("el", "Λατάκεια"), ("en", "Latakia"), ("es", "Gobernación de Latakia"), ("eu", "Latakia eskualdea"), ("fa", "استان لاذقیه"), ("fi", "Lattakian maakunta"), ("fr", "Gouvernorat de Lattaquié"), ("he", "מחוז לטקיה"), ("hi", "लातक\u{93c}िया प\u{94d}रान\u{94d}त"), ("hu", "Latakia kormányzóság"), ("hy", "Լաթակիայի մարզ"), ("id", "Kegubernuran Latakia"), ("it", "governatorato di Latakia"), ("ja", "ラタキア県"), ("ka", "ლათაკიის მუჰაფაზა"), ("ko", "라타키아 주"), ("lt", "Latakijos muchafaza"), ("nb", "Latakia"), ("nl", "Latakia"), ("no", "Latakia"), ("pl", "Latakia"), ("pt", "Latakia"), ("ro", "Guvernoratul Latakia"), ("ru", "Латакия"), ("sr", "Латакија (покрајина)"), ("sr_Latn", "Latakija (pokrajina)"), ("sv", "Latakia"), ("tr", "Lazkiye"), ("uk", "Латакія"), ("ur", "اللاذقیة"), ("vi", "Latakia"), ("zh", "拉塔基亚省")]),
                        unofficial_name_list: ["Latakia", "Lattakia", "Lattaquié", "Lādhiqīyah", "al-Ladhiqiyah"].to_vec(),
                    }
                ),
                (
                    "QU",
                    Subdivision{
                        name: "Al Qunaytirah",
                        country_alpha2: Alpha2::SY,
                        code: "QU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.0776318), longitude: Some(35.8934136), max_latitude: Some(33.247301), min_latitude: Some(32.836052), max_longitude: Some(36.003361), min_longitude: Some(35.8478781)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة القنيطرة"), ("az", "Əl-Qüneytrə mühafazası"), ("bg", "Кунейтра"), ("bn", "ক\u{9c1}নিত\u{9cd}র\u{9be} গভর\u{9cd}নোরেট"), ("ca", "governació de Quneitra"), ("ccp", "𑄇\u{1112a}𑄠𑄚\u{11134}𑄃\u{1112d}𑄑\u{11133}𑄢"), ("ceb", "Quneitra Governorate"), ("cs", "Guvernorát Kunejtra"), ("da", "Al-Qunaytirah"), ("de", "Gouvernement al-Quneitra"), ("el", "Κουνέιτρα"), ("en", "Quneitra"), ("es", "Gobernación de Quneitra"), ("eu", "Al Qunaytirah eskualdea"), ("fa", "استان قنیطره"), ("fi", "Al-Qunaytran maakunta"), ("fr", "Gouvernorat de Qouneitra"), ("gu", "ક\u{acd}ય\u{ac1}ન\u{ac7}ટ\u{acd}રા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז קוניטרה"), ("hy", "Ալ-Կունեյտրայի մարզ"), ("id", "Kegubernuran Quneitra"), ("it", "governatorato di Quneitra"), ("ja", "クネイトラ県"), ("ka", "ალ-კუნეიტრის მუჰაფაზა"), ("kn", "ಖುನೈತ\u{ccd}ರ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "쿠네이트라 주"), ("lt", "Kuneitros muchafaza"), ("lv", "Kunaitiras muhāfaza"), ("mr", "क\u{94d}य\u{942}नइ\u{902}ट\u{94d}रा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Quneitra Governorate"), ("nb", "Al-Qunaytirah"), ("nl", "Quneitra"), ("no", "Al-Qunaytirah"), ("pl", "Al-Kunajtira"), ("pt", "Quneitra"), ("ro", "Guvernoratul Quneitra"), ("ru", "Эль-Кунейтра"), ("si", "ක\u{dd4}නේඉට\u{dca}\u{200d}ර\u{dcf} පළ\u{dcf}ත"), ("sr", "Кунејтра (покрајина)"), ("sr_Latn", "Kunejtra (pokrajina)"), ("sv", "al-Qunaytirah"), ("ta", "குனிற\u{bcd}ற\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "క\u{c4d}వ\u{c3f}న\u{c40}ట\u{c4d}ర\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ค\u{e39}เนทร\u{e48}า โกเวอโนเรท"), ("tr", "Kuneytire"), ("uk", "Ель-Кунейтра"), ("ur", "محافظہ قنیطرہ"), ("vi", "Quneitra"), ("zh", "库奈特拉省")]),
                        unofficial_name_list: ["Quneitra", "al-Qunaytirah"].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "Ar Raqqah",
                        country_alpha2: Alpha2::SY,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.9594106), longitude: Some(38.9981052), max_latitude: Some(35.9821736), min_latitude: Some(35.9272857), max_longitude: Some(39.0682411), min_longitude: Some(38.9259338)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الرقة"), ("az", "Ər-Rəqqə mühafazası"), ("bg", "Ар-Ракка"), ("ca", "Governació d’Ar-Raqqà"), ("ccp", "𑄃𑄢\u{11134}-𑄢𑄇\u{11133}𑄦"), ("ceb", "Ar-Raqqah Governorate"), ("cs", "Guvernorát Rakka"), ("de", "Gouvernement ar-Raqqa"), ("el", "Αρ Ρακκά"), ("en", "Ar-Raqqah"), ("es", "Gobernación de Ar-Raqqa"), ("eu", "Ar-Raqqa eskualdea"), ("fa", "استان رقه"), ("fi", "Al-Raqqan maakunta"), ("fr", "Gouvernorat de Racca"), ("he", "מחוז א-רקה"), ("hu", "Rakka kormányzóság"), ("hy", "Ալ-Ռակկայի մարզ"), ("id", "Kegubernuran Ar-Raqqah"), ("it", "governatorato di al-Raqqa"), ("ja", "ラッカ県"), ("ka", "ალ-რაკის მუჰაფაზა"), ("ko", "락까 주"), ("lt", "Rakos muchafaza"), ("mk", "Рака"), ("nb", "Ar-Raqqah"), ("nl", "Ar-Raqqah"), ("no", "Ar-Raqqah"), ("pl", "Ar-Rakka"), ("pt", "Ar-Raqqah"), ("ro", "Guvernoratul Ar-Raqqa"), ("ru", "Ракка"), ("sr", "Рака (покрајина)"), ("sr_Latn", "Raka (pokrajina)"), ("sv", "Ar-Raqqah"), ("tr", "Rakka"), ("uk", "Ар-Ракка"), ("ur", "محافظہ الرقہ"), ("vi", "Ar-Raqqah"), ("zh", "拉卡省")]),
                        unofficial_name_list: ["Raqqah", "al-Rakka"].to_vec(),
                    }
                ),
                (
                    "RD",
                    Subdivision{
                        name: "Rif Dimashq",
                        country_alpha2: Alpha2::SY,
                        code: "RD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5167289), longitude: Some(36.954107), max_latitude: Some(34.2120558), min_latitude: Some(32.68313), max_longitude: Some(38.313671), min_longitude: Some(35.834707)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ريف دمشق"), ("az", "Dəməşq mühafazası"), ("bg", "Кунейтра²"), ("bn", "রিফ দ\u{9be}ম\u{9be}স\u{9cd}ক গভর\u{9cd}নোরেট"), ("ca", "Governació de Damasc Rural"), ("ccp", "𑄢\u{11128}𑄛\u{11134} 𑄓\u{11128}𑄟𑄌\u{11134}𑄇\u{11134}"), ("cs", "Ríf Dimašq"), ("da", "Rif Dimashq Governorate"), ("de", "Gouvernement Rif Dimaschq"), ("el", "Ριφ Ντιμάσκ"), ("en", "Rif Dimashq"), ("es", "Gobernación de la campiña de Damasco"), ("eu", "Rif Dimashq eskualdea"), ("fa", "استان ریف دمشق"), ("fi", "Rif Dimašqin maakunta"), ("fr", "Gouvernorat de Rif Dimachq"), ("gu", "રિફ ડિમાશક ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "ריף דמשק"), ("hi", "रीफ\u{93c} दिमश\u{94d}क\u{93c} प\u{94d}रान\u{94d}त"), ("hu", "Ríf Dimask kormányzóság"), ("hy", "Ռիֆ Դիմաշկ"), ("id", "Kegubernuran Rif Dimashq"), ("it", "governatorato del Rif di Damasco"), ("ja", "ダマスカス郊外県"), ("ka", "რიფ-დიმაშკის მუჰაფაზა"), ("kn", "ರ\u{cbf}ಫ\u{ccd} ದ\u{cbf}ಮಾಶ\u{ccd}ಕ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "리프디마슈크 주"), ("lt", "Damasko muchafaza"), ("lv", "Damaskas muhāfaza"), ("mr", "रिफ डिमशक गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Rif Dimashq Governorate"), ("nb", "Rif Dimashq"), ("nl", "Rif Dimashq"), ("no", "Rif Dimashq"), ("pl", "Damaszek"), ("pt", "Rif Dimashq"), ("ro", "Guvernoratul Rif Dimashq"), ("ru", "Дамаск²"), ("si", "ර\u{dd2}ෆ\u{dca} ඩ\u{dd2}ම\u{dcf}ශ\u{dca}ක\u{dca} පළ\u{dcf}ත"), ("sr", "Дамаск (покрајина)"), ("sr_Latn", "Damask (pokrajina)"), ("sv", "Rif Dimashq"), ("ta", "ரிஃ டிம\u{bbe}ஷ\u{bcd}க\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ర\u{c3f}ఫ\u{c4d} డ\u{c3f}మ\u{c3e}ష\u{c4d}ఖ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ร\u{e34}ฟด\u{e34}ม\u{e31}ชก\u{e4c}"), ("tr", "Rif Şam"), ("uk", "Дамаск"), ("ur", "محافظہ ریف دمشق"), ("vi", "Rif Dimashq"), ("zh", "大马士革农村省")]),
                        unofficial_name_list: ["Damas", "Damascus", "Damaskus", "Dimashq", "Dimašq"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "As Suwayda'",
                        country_alpha2: Alpha2::SY,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.7), longitude: Some(36.566667), max_latitude: Some(32.7348733), min_latitude: Some(32.6688949), max_longitude: Some(36.6076899), min_longitude: Some(36.5449906)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة السويداء"), ("az", "Əs-Süveydə mühafazası"), ("bg", "Ас-Суейда"), ("bn", "আস-স\u{9c1}ব\u{9be}য\u{9bc}দ\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació d’As-Suwayda"), ("ccp", "𑄃𑄌\u{11134}-𑄥\u{1112a}𑄠𑄬𑄓"), ("ceb", "As-Suwayda Governorate"), ("cs", "Guvernorát Suvajda"), ("da", "As-Suwayda"), ("de", "Gouvernement as-Suwaida"), ("el", "Ας Σουγουαΰντα"), ("en", "As-Suwayda"), ("es", "Gobernación de Sueida"), ("eu", "As Suwayda eskualdea"), ("fa", "استان سویدا"), ("fi", "Al-Suwaydanin maakunta"), ("fr", "Gouvernorat de Suweyda"), ("gu", "અસ-સ\u{ac1}વ\u{ac7}દા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז א-סווידא"), ("hi", "अस-स\u{941}व\u{948}दा प\u{94d}रान\u{94d}त"), ("hy", "Ալ-Սուեյդայի մարզ"), ("id", "Kegubernuran As-Suwayda"), ("it", "governatorato di al-Suwayda"), ("ja", "スワイダー県"), ("ka", "ელ-სუვაიდის მუჰაფაზა"), ("kn", "ಸುವೇದಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "수와이다 주"), ("lt", "Suveidos muchafaza"), ("lv", "Suveidas muhāfaza"), ("mr", "अस-स\u{941}व\u{947}डा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "As-Suwayda Governorate"), ("nb", "as-Suwayda"), ("nl", "As-Suwayda"), ("no", "as-Suwayda"), ("pl", "As-Suwajda"), ("pt", "As-Suwayda"), ("ro", "Guvernoratul As-Suwayda"), ("ru", "Эс-Сувейда"), ("si", "අස\u{dca}-ස\u{dd4}වේද\u{dcf} පළ\u{dcf}ත"), ("sr", "Сувајда (покрајина)"), ("sr_Latn", "Suvajda (pokrajina)"), ("sv", "As-Suwayda’"), ("ta", "அஸ\u{bcd} -சுவ\u{bbe}யட கோவெர\u{bcd}னோரே"), ("te", "అస\u{c4d}-సువ\u{c47} డ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}สซ\u{e38}เวย\u{e4c}ดา"), ("tr", "Süveyde"), ("uk", "Ас-Сувайда"), ("ur", "محافظہ السویداء"), ("vi", "As-Suwayda"), ("zh", "苏韦达省")]),
                        unofficial_name_list: ["as-Suwayda"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "Tartus",
                        country_alpha2: Alpha2::SY,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.883333), longitude: Some(35.883333), max_latitude: Some(34.9235482), min_latitude: Some(34.8481038), max_longitude: Some(35.9136459), min_longitude: Some(35.8542896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة طرطوس"), ("az", "Tərtus mühafazası"), ("bg", "Тартус"), ("bn", "ট\u{9be}রট\u{9be}স গভর\u{9cd}নোরেট"), ("ca", "Governació de Tartus"), ("ccp", "𑄑𑄢\u{11134}𑄑𑄌\u{11134}"), ("ceb", "Tartus Governorate"), ("cs", "Guvernorát Tartús"), ("da", "Tartus Governorate"), ("de", "Gouvernement Tartus"), ("el", "Ταρτούς"), ("en", "Tartus"), ("es", "Gobernación de Tartus"), ("eu", "Tartus eskualdea"), ("fa", "استان طرطوس"), ("fi", "Tartusin maakunta"), ("fr", "Gouvernorat de Tartous"), ("gu", "ટાર\u{acd}ટસ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז טרטוס"), ("hi", "तरत\u{942}स प\u{94d}रान\u{94d}त"), ("hu", "Tartúsz kormányzóság"), ("hy", "Տարտուսի մարզ"), ("id", "Kegubernuran Tartus"), ("it", "governatorato di Tartus"), ("ja", "タルトゥース県"), ("ka", "ტარტუსის მუჰაფაზა"), ("kn", "ಟಾರ\u{ccd}ಟಸ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "타르투스 주"), ("lt", "Tartuso muchafaza"), ("lv", "Tartūsas muhāfaza"), ("mr", "तार\u{94d}टास गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Tartus Governorate"), ("nb", "Tartus"), ("nl", "Tartous"), ("no", "Tartus"), ("pl", "Tartus"), ("pt", "Tartus"), ("ro", "Guvernoratul Tartus"), ("ru", "Тартус"), ("si", "ට\u{dcf}ර\u{dca}ටස\u{dca} පර\u{dd2}ප\u{dcf}ලන ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}කය"), ("sr", "Тартус (покрајина)"), ("sr_Latn", "Tartus (pokrajina)"), ("sv", "Tartus"), ("ta", "ட\u{bbe}ர\u{bcd}ட\u{bcd}ஸ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ట\u{c3e}ర\u{c4d}టస\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตอร\u{e4c}ต\u{e39}ส"), ("tr", "Tartus"), ("uk", "Тартус"), ("ur", "محافظہ طرطوس"), ("vi", "Tartus"), ("zh", "塔尔图斯省")]),
                        unofficial_name_list: ["Tartoûs", "Tartus"].to_vec(),
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
#[cfg(feature = "sy")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::SY,
        alpha3: Alpha3::SYR,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 963,
        currency_code: CurrencyCode::SYP,
        gec: Some(GEC::SY),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::SYR),
        iso_long_name: "The Syrian Arab Republic",
        iso_short_name: "Syrian Arab Republic",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "0",
        nationality: Some("Syrian"),
        number: "760",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "SY",
        unofficial_name_list: ["Syria", "سوريا", "سورية", "Syrien", "Syrie", "Siria", "シリア・アラブ共和国", "Syrië"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Syrian Arab Republic"), ("af", "Siriese Arabiese Republiek"), ("ak", "Syrian Arab Republic"), ("am", "ሲሱ።"), ("an", "Syrian Arab Republic"), ("ar", "سوريا"), ("as", "চিৰিয়\u{9be}ন আৰব প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"), ("ay", "Syrian Arab Republic"), ("az", "Syrian Arab Republic"), ("ba", "Syrian Arab Republic"), ("be", "Сірыйская Арабская Рэспубліка"), ("bg", "Сирийска арабска република"), ("bi", "Syrian Arab Republic"), ("bn", "সিরিয়\u{9be}ন আরব প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bn_IN", "সিরিয়\u{9be}ন আরব প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("br", "Syrian Arab Republic"), ("bs", "Sirijska Arapska Republika"), ("ca", "República Àrab Síria"), ("ce", "Syrian Arab Republic"), ("ch", "Syrian Arab Republic"), ("cs", "Syrská arabská republika"), ("cv", "Syrian Arab Republic"), ("cy", "Gweriniaeth Arabaidd Syria"), ("da", "Syriske Arabiske Republik"), ("de", "Syrien, Arabische Republik"), ("dv", "Syrian Arab Republic"), ("dz", "ས\u{f72}་ར\u{f7a}ན་ ཨ་རབ་ མ\u{f72}་ས\u{f7a}ར་ར\u{f92}\u{fb1}ལ་ཁབ།"), ("ee", "Syrian Arab Republic"), ("el", "Αραβική Δημοκρατία της Συρίας"), ("en", "Syrian Arab Republic"), ("eo", "Siria Araba Respubliko"), ("es", "República árabe de Siria"), ("et", "Süüria Araabia Vabariik"), ("eu", "Siriako Arabiar Errepublika"), ("fa", "جمهوری عربی سوریه"), ("ff", "Syrian Arab Republic"), ("fi", "Syyrian arabitasavalta"), ("fo", "Syrian Arab Republic"), ("fr", "Syrienne, République arabe"), ("fy", "Syrian Arab Republic"), ("ga", "Poblacht Arabach na Siria"), ("gl", "República Árabe de Siria"), ("gn", "Syrian Arab Republic"), ("gu", "સીરીયન આરબ રીપબ\u{acd}લિક"), ("gv", "Syrian Arab Republic"), ("ha", "Syrian Arab Republic"), ("he", "הרפובליקה הערבית הסורית"), ("hi", "सीरियन अरब रिपब\u{94d}लिक"), ("hr", "Sirijska Arapska Republika"), ("ht", "Syrian Arab Republic"), ("hu", "Szíriai Arab Köztársaság"), ("hy", "Սիրիայի Արաբական Հանրապետություն"), ("ia", "Republica Arabe Syrie"), ("id", "Republik Arab Syria"), ("io", "Syrian Arab Republic"), ("is", "Sýrlenska arabalýðveldið"), ("it", "Siria"), ("iu", "Syrian Arab Republic"), ("ja", "シリア・アラブ共和国"), ("ka", "სირიის არაბული რესპუბლიკა"), ("ki", "Syrian Arab Republic"), ("kk", "Сирия Араб Республикасы"), ("kl", "Syrian Arab Republic"), ("km", "សាធារណរដ\u{17d2}ឋ\u{200b}ស\u{17ca}\u{17b8}រ\u{17b8}យ\u{17c9}ា\u{200b}\u{200b}អារាប\u{17cb}"), ("kn", "ಸ\u{cbf}ರ\u{cbf}ಯನ\u{ccd} ಅರಬ\u{ccd} ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "시리아 아랍 공화국"), ("ku", "Sûriye"), ("kv", "Syrian Arab Republic"), ("kw", "Syrian Arab Republic"), ("ky", "Сирия Араб Республикасы"), ("lo", "Syrian Arab Republic"), ("lt", "Sirijos Arabų Respublika"), ("lv", "Sīrija"), ("mi", "Syrian Arab Republic"), ("mk", "Сирија арапска република"), ("ml", "സിറിയന\u{d4d}\u{200d} അറബ\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d}"), ("mn", "Syrian Arab Republic"), ("mr", "सिरियन अरब रिपब\u{94d}लिक"), ("ms", "Syrian Arab Republic"), ("mt", "Syrian Arab Republic"), ("my", "Syrian Arab Republic"), ("na", "Syrian Arab Republic"), ("nb", "Den arabiske republikk Syria"), ("ne", "सिरियन अरब गणराज\u{94d}य"), ("nl", "Syrië"), ("nn", "Syria"), ("nv", "Syrian Arab Republic"), ("oc", "Republica Dominicana"), ("or", "ସ\u{b3f}ର\u{b3f}ଆନ ଆରବ ଗଣତନ\u{b4d}ତ\u{b4d}ର"), ("pa", "ਸੀਰੀਅਨ ਅਰਬ ਗਣਰਾਜ"), ("pi", "Syrian Arab Republic"), ("pl", "Syryjska Republika Arabska"), ("ps", "Syrian Arab Republic"), ("pt", "República Árabe Síria"), ("pt_BR", "República Árabe da Síria"), ("ro", "Republica Araba Siria"), ("ru", "Сирийская Арабская Республика"), ("rw", "Repubulika Nyarabu ya Siriya"), ("sc", "Repùblica Àraba de Sìria"), ("sd", "Syrian Arab Republic"), ("si", "ස\u{dd2}ර\u{dd2}ය\u{dcf}න\u{dd4} අර\u{dcf}බ\u{dd2} ජනරජය"), ("sk", "Sýrska arabská republika"), ("sl", "Sirska arabska republika"), ("so", "Suuriya"), ("sq", "Republika Arabe e Sirisë"), ("sr", "Сиријска Арапска Република"), ("sv", "Syriska arabrepubliken"), ("sw", "Syrian Arab Republic"), ("ta", "சிரியன\u{bcd} அரபு குடியரசு"), ("te", "స\u{c3f}ర\u{c3f}యన\u{c4d} అరబ\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("tg", "Ҷумҳурии Сурияи Араб"), ("th", "สาธารณร\u{e31}ฐอาหร\u{e31}บซ\u{e35}เร\u{e35}ย"), ("ti", "Syrian Arab Republic"), ("tk", "Siriýa Arap Respublikasy"), ("tl", "Syrian Arab Republika"), ("tr", "Suriye Arap Cumhuriyeti"), ("tt", "Гәрәп Сүриә Җөмһүриәте"), ("ug", "سۈرىيە ئەرەب جۇمھۇرىيىتى"), ("uk", "Сирійська Арабська Республіка"), ("ur", "Syrian Arab Republic"), ("uz", "Syrian Arab Republic"), ("ve", "Syrian Arab Republic"), ("vi", "Cộng hoà A-rập Xi-ri-a"), ("wa", "Sireye"), ("wo", "Republik Araab bu Siiri"), ("xh", "Syrian Arab Republic"), ("yo", "Syrian Arab Republic"), ("zh_CN", "叙利亚"), ("zh_HK", "阿拉伯敍利亞共和國"), ("zh_TW", "敘利亞阿拉伯共和國"), ("zu", "Syrian Arab Republic")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
