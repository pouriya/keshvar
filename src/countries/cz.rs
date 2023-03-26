// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Czech Republic

#[cfg(all(feature = "cz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::CZ;
    pub const ALPHA3: Alpha3 = Alpha3::CZE;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 420;
    pub const CURRENCY_CODE: &str = "CZK";
    pub const GEC: Option<GEC> = Some(GEC::EZ);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CZE);
    pub const ISO_SHORT_NAME: &str = "Czechia";
    pub const ISO_LONG_NAME: &str = "The Czech Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["cs", "sk"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["cs", "sk"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Czech");
    pub const NUMBER: &str = "203";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3} ?\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "CZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Czech Republic",
        "Tschechische Republik",
        "République Tchèque",
        "República Checa",
        "チェコ",
        "Tsjechië",
        "Czechia",
        "Česká republika",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Czechia"),
        ("af", "Tsjeggië"),
        ("ak", "Czechia"),
        ("am", "Czechia"),
        ("an", "Czechia"),
        ("ar", "التشيك"),
        ("as", "Czechia"),
        ("ay", "Czechia"),
        ("az", "Czechia"),
        ("ba", "Czechia"),
        ("be", "Чэхія"),
        ("bg", "Czechia"),
        ("bi", "Czechia"),
        ("bn", "চেকিয়\u{9be}"),
        ("bn_IN", "চেচিয\u{9bc}\u{9be}"),
        ("br", "Czechia"),
        ("bs", "Czechia"),
        ("ca", "Txèquia"),
        ("ce", "Czechia"),
        ("ch", "Czechia"),
        ("cs", "Česko"),
        ("cv", "Czechia"),
        ("cy", "Tsiecia"),
        ("da", "Tjekkiet"),
        ("de", "Tschechien"),
        ("dv", "Czechia"),
        ("dz", "Czechia"),
        ("ee", "Czechia"),
        ("el", "Τσεχία"),
        ("en", "Czechia"),
        ("eo", "Ĉeĥio"),
        ("es", "Chequia"),
        ("et", "Tšehhi"),
        ("eu", "Txekia"),
        ("fa", "چک"),
        ("ff", "Czechia"),
        ("fi", "Czechia"),
        ("fo", "Czechia"),
        ("fr", "Tchéquie"),
        ("fy", "Czechia"),
        ("ga", "Czechia"),
        ("gl", "Chequia"),
        ("gn", "Czechia"),
        ("gu", "ચ\u{ac7}કિયા"),
        ("gv", "Czechia"),
        ("ha", "Czechia"),
        ("he", "צ'כיה"),
        ("hi", "च\u{947}किया"),
        ("hr", "Češka"),
        ("ht", "Tchèki"),
        ("hu", "Csehország"),
        ("hy", "Czechia"),
        ("ia", "Cechia"),
        ("id", "Czechia"),
        ("io", "Czechia"),
        ("is", "Tékkland"),
        ("it", "Cechia"),
        ("iu", "Czechia"),
        ("ja", "Czechia"),
        ("ka", "Czechia"),
        ("ki", "Czechia"),
        ("kk", "Czechia"),
        ("kl", "Czechia"),
        ("km", "ឆេក"),
        ("kn", "Czechia"),
        ("ko", "체코"),
        ("ku", "Çekistan"),
        ("kv", "Czechia"),
        ("kw", "Czechia"),
        ("ky", "Чехия Республикасы"),
        ("lo", "Czechia"),
        ("lt", "Czechia"),
        ("lv", "Czechia"),
        ("mi", "Czechia"),
        ("mk", "Czechia"),
        ("ml", "Czechia"),
        ("mn", "Czechia"),
        ("mr", "झ\u{947}किया"),
        ("ms", "Czechia"),
        ("mt", "Czechia"),
        ("my", "Czechia"),
        ("na", "Czechia"),
        ("nb", "Tsjekkia"),
        ("ne", "Czechia"),
        ("nl", "Tsjechië"),
        ("nn", "Czechia"),
        ("nv", "Czechia"),
        ("oc", "Chequia"),
        ("or", "ଚେସ\u{b3f}ଆ"),
        ("pa", "ਚ\u{a47}ਚੀਆ"),
        ("pi", "Czechia"),
        ("pl", "Czechy"),
        ("ps", "Czechia"),
        ("pt", "República Checa"),
        ("pt_BR", "Chéquia"),
        ("ro", "Cehia"),
        ("ru", "Чехия"),
        ("rw", "Czechia"),
        ("sc", "Tzèchia"),
        ("sd", "Czechia"),
        ("si", "Czechia"),
        ("sk", "Česko"),
        ("sl", "Czechia"),
        ("so", "Czechia"),
        ("sq", "Çeki"),
        ("sr", "Чешка"),
        ("sv", "Tjeckien"),
        ("sw", "Czechia"),
        ("ta", "Czechia"),
        ("te", "Czechia"),
        ("tg", "Чехия"),
        ("th", "ประเทศเช\u{e47}กเก\u{e35}ย"),
        ("ti", "Czechia"),
        ("tk", "Czechia"),
        ("tl", "Czechia"),
        ("tr", "Çekya"),
        ("tt", "Czechia"),
        ("ug", "چېخ"),
        ("uk", "Чехія"),
        ("ur", "Czechia"),
        ("uz", "Czechia"),
        ("ve", "Czechia"),
        ("vi", "Czechia"),
        ("wa", "Czechia"),
        ("wo", "Czechia"),
        ("xh", "Czechia"),
        ("yo", "Czechia"),
        ("zh_CN", "捷克"),
        ("zh_HK", "捷克"),
        ("zh_TW", "捷克"),
        ("zu", "Czechia"),
    ];
    #[cfg(all(feature = "cz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 49.81749199999999;
        pub const LONGITUDE: f64 = 15.472962;
        pub const MAX_LATITUDE: f64 = 51.0557185;
        pub const MAX_LONGITUDE: f64 = 18.8592361;
        pub const MIN_LATITUDE: f64 = 48.5518081;
        pub const MIN_LONGITUDE: f64 = 12.090589;
        pub const NORTHEAST_LATITUDE: f64 = 51.0557185;
        pub const NORTHEAST_LONGITUDE: f64 = 18.8592361;
        pub const SOUTHWEST_LATITUDE: f64 = 48.5518081;
        pub const SOUTHWEST_LONGITUDE: f64 = 12.090589;
    }
}
#[cfg(all(feature = "cz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 49.81749199999999,
            longitude: 15.472962,
            max_latitude: 51.0557185,
            max_longitude: 18.8592361,
            min_latitude: 48.5518081,
            min_longitude: 12.090589,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 51.0557185,
                    longitude: 18.8592361,
                },
                southwest: CountryGeoBound {
                    latitude: 48.5518081,
                    longitude: 12.090589,
                },
            },
        }
    }
}

#[cfg(all(feature = "cz", feature = "subdivisions"))]
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
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::CZ,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.0755381), longitude: Some(14.4378005), max_latitude: Some(50.177403), min_latitude: Some(49.94193629999999), max_longitude: Some(14.7067945), min_longitude: Some(14.2244533)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Praag"), ("am", "ፕራግ"), ("ar", "براغ"), ("az", "Praqa"), ("be", "Прага"), ("bg", "Прага"), ("bn", "প\u{9cd}র\u{9be}গ"), ("bs", "Prag"), ("ca", "Praga"), ("ccp", "𑄛\u{11133}𑄢\u{11127}𑄉\u{1112a} 𑄦\u{11133}𑄣𑄞𑄚\u{1112d} 𑄟𑄬𑄌\u{11134}𑄑\u{1112e}"), ("ceb", "Praga"), ("cs", "Praha"), ("cy", "Prag"), ("da", "Prag"), ("de", "Prag"), ("el", "Πράγα"), ("en", "Prague, Hlavní mešto"), ("es", "Praga"), ("et", "Praha"), ("eu", "Praga"), ("fa", "پراگ"), ("fi", "Praha"), ("fr", "Prague"), ("ga", "Prág"), ("gl", "Praga"), ("gu", "પ\u{acd}રાગ"), ("ha", "Prag"), ("ha_NE", "Prag"), ("he", "פראג"), ("hi", "प\u{94d}राग"), ("hr", "Prag"), ("hu", "Prága"), ("hy", "Պրահա"), ("id", "Praha"), ("is", "Prag"), ("it", "Praga"), ("ja", "プラハ"), ("jv", "Praha"), ("ka", "პრაღა"), ("kk", "Прага"), ("km", "ទ\u{17b8}ក\u{17d2}រ\u{17bb}ងប\u{17d2}រាក"), ("kn", "ಪ\u{ccd}ರಾಗ\u{ccd}"), ("ko", "프라하"), ("ky", "Прага"), ("lo", "ປຣາກ"), ("lt", "Praha"), ("lv", "Prāga"), ("mk", "Прага"), ("ml", "പ\u{d4d}ര\u{d3e}ഗ\u{d4d}"), ("mn", "Прага"), ("mr", "प\u{94d}राग"), ("ms", "Praha"), ("my", "ပရက\u{103a}ဂ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Praha"), ("ne", "प\u{94d}राग"), ("nl", "Praag"), ("no", "Praha"), ("or", "ପ\u{b4d}ର\u{b3e}ଗ"), ("pa", "ਪ\u{a4d}ਰਾਗ"), ("pl", "Praga"), ("ps", "پراګ"), ("pt", "Praga"), ("ro", "Praga"), ("ru", "Прага"), ("sd", "پراگ"), ("si", "ප\u{dca}\u{200d}ර\u{dcf}ග\u{dca}"), ("sk", "Praha"), ("sl", "Praga"), ("sq", "Praga"), ("sr", "Праг"), ("sr_Latn", "Prag"), ("sv", "Prag"), ("sw", "Praha"), ("ta", "பிர\u{bbe}க\u{bbe}"), ("te", "ప\u{c4d}ర\u{c3e}హ\u{c3e}"), ("th", "ปราก"), ("tk", "Praga"), ("tr", "Prag"), ("uk", "Прага"), ("ur", "پراگ"), ("uz", "Praga"), ("vi", "Praha"), ("yo", "Prague"), ("yo_BJ", "Prague"), ("yue", "布拉格"), ("yue_Hans", "布拉格"), ("zh", "布拉格"), ("zu", "IPraha")]),
                        unofficial_name_list: ["Hlavní město Praha", "Prag", "Prague", "Praha"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::CZ,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.8782223), longitude: Some(14.9362955), max_latitude: Some(50.6190994), min_latitude: Some(49.50133659999999), max_longitude: Some(15.5345257), min_longitude: Some(13.3973366)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بوهيميا الوسطى"), ("az", "Orta Çex diyarı"), ("be", "Сярэднячэшскі край"), ("bg", "Среднобохемски край"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল ব\u{9c1}হেমিয\u{9bc}\u{9be}ন অঞ\u{9cd}চল"), ("bs", "Srednjočeški kraj"), ("ca", "Regió de Bohèmia Central"), ("ccp", "𑄃𑄬𑄌\u{11134}𑄑\u{11133}𑄢𑄬𑄓\u{1112e}𑄥𑄬𑄌\u{11134}𑄇\u{11128}"), ("ceb", "Středočeský kraj"), ("cs", "Středočeský kraj"), ("da", "Centralbøhmen"), ("de", "Středočeský kraj"), ("el", "Περιφέρεια Κεντρικής Βοημίας"), ("en", "Středočeský"), ("es", "Bohemia Central"), ("et", "Kesk-Tšehhi maakond"), ("eu", "Erdialdeko Bohemiako eskualdea"), ("fa", "منطقه بوهمی مرکزی"), ("fi", "Keski-Böömin lääni"), ("fr", "Bohême centrale"), ("gl", "Bohemia Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ બોહ\u{ac7}મિયન પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז מרכז בוהמיה"), ("hi", "क\u{947}\u{902}द\u{94d}रीय बोह\u{947}मियन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Središnja Češka"), ("hu", "Közép-csehországi kerület"), ("hy", "Կենտրոնաչեխական շրջան"), ("id", "Daerah Bohemia Tengah"), ("it", "Boemia centrale"), ("ja", "中央ボヘミア州"), ("ka", "ცენტრალური ბოჰემიის მხარე"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಬೋಹೀಮ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "중앙보헤미아 주"), ("lt", "Vidurio Čekijos kraštas"), ("lv", "Centrālčehijas apgabals"), ("mk", "Средночешки крај"), ("mr", "स\u{947}\u{902}ट\u{94d}रल बोह\u{947}मियन प\u{94d}रद\u{947}श"), ("ms", "Daerah Bohemia Tengah"), ("nb", "Sentralbøhmen region"), ("nl", "Midden-Bohemen"), ("no", "Sentralbøhmen region"), ("pl", "Kraj środkowoczeski"), ("pt", "Boêmia Central"), ("ro", "Regiunea Boemia Centrală"), ("ru", "Среднечешский край"), ("si", "මධ\u{dca}\u{200d}යම බොහ\u{dd2}ම\u{dd2}යන\u{dca} කල\u{dcf}පය"), ("sk", "Stredočeský kraj"), ("sr", "Средњочешки крај"), ("sr_Latn", "Srednjočeški kraj"), ("sv", "Mellersta Böhmen"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} பொஹ\u{bc0}மிய\u{bbe}ன\u{bcd} பகுதி"), ("te", "మధ\u{c4d}య బ\u{c4a}హ\u{c40}మ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతము"), ("th", "เขตปกครองโบฮ\u{e35}เม\u{e35}ยนกลาง"), ("tr", "Orta Bohemia"), ("uk", "Центральночеський край"), ("ur", "مرکزی بوہیمیائی علاقہ"), ("vi", "Trung Bohemia"), ("zh", "中波希米亚州")]),
                        unofficial_name_list: ["Central Bohemia", "Mittelböhmen", "Středočeský"].to_vec(),
                    }
                ),
                (
                    "201",
                    Subdivision{
                        name: "201",
                        country_alpha2: Alpha2::CZ,
                        code: "201",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بينيشوف"), ("be", "раён Бенешаў"), ("bg", "Бенешов (окръг)"), ("ca", "Districte de Benešov"), ("ccp", "𑄝𑄬𑄚𑄬𑄥\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Benešov"), ("cs", "okres Benešov"), ("de", "Okres Benešov"), ("en", "Benešov"), ("es", "Distrito de Benešov"), ("eu", "Benešoveko barrutia"), ("fa", "شهرستان بنشوف"), ("fr", "District de Benešov"), ("he", "מחוז משנה בנשוב"), ("hu", "Benešovi járás"), ("hy", "Խեբի շրջան"), ("it", "Distretto di Benešov"), ("ja", "ベネショフ郡"), ("ka", "ბენეშოვის რაიონი"), ("lt", "Benešovo rajonas"), ("lv", "Benešovas apriņķis"), ("ms", "Daerah Benešov"), ("nl", "Okres Benešov"), ("pl", "Powiat Benešov"), ("pt", "Benešov (distrito)"), ("ro", "Benešov"), ("ru", "Бенешов"), ("sk", "Okres Benešov"), ("sr", "Округ Бенешов"), ("sr_Latn", "Okrug Benešov"), ("sv", "Benešov (distrikt)"), ("tr", "Benešov ilçesi"), ("uk", "Бенешов"), ("ur", "بینیشوف ضلع"), ("vi", "Benešov"), ("zh", "貝內紹夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "202",
                    Subdivision{
                        name: "202",
                        country_alpha2: Alpha2::CZ,
                        code: "202",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيرون"), ("be", "Бераўн"), ("bg", "Бероун"), ("ca", "Districte de Beroun"), ("ccp", "𑄝𑄬𑄢\u{1112f}𑄚\u{11134}"), ("ceb", "Okres Beroun"), ("cs", "okres Beroun"), ("de", "Okres Beroun"), ("en", "Beroun"), ("es", "Distrito de Beroun"), ("eu", "Beroungo barrutia"), ("fa", "شهرستان برون"), ("fr", "District de Beroun"), ("he", "מחוז משנה ברואון"), ("hu", "Berouni járás"), ("hy", "Կարլովի Վարիի շրջան"), ("it", "distretto di Beroun"), ("ka", "ბეროუნის რაიონი"), ("lt", "Berouno rajonas"), ("lv", "Berounas apriņķis"), ("mk", "Бероун"), ("ms", "Daerah Beroun"), ("nl", "Okres Beroun"), ("pl", "Powiat Beroun"), ("pt", "Beroun"), ("ru", "Бероун"), ("sk", "Okres Beroun"), ("sr", "Округ Бероун"), ("sr_Latn", "Okrug Beroun"), ("sv", "Beroun"), ("tr", "Beroun ilçesi"), ("uk", "Бероун (округ)"), ("ur", "بیرون ضلع"), ("vi", "Beroun"), ("zh", "貝龍縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "203",
                    Subdivision{
                        name: "203",
                        country_alpha2: Alpha2::CZ,
                        code: "203",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كلادنو"), ("be", "Кладна"), ("bg", "Кладно"), ("ca", "Districte de Kladno"), ("ccp", "𑄇\u{11133}𑄣𑄖\u{11134}𑄚\u{1112e}"), ("ceb", "Okres Kladno"), ("cs", "okres Kladno"), ("de", "Okres Kladno"), ("en", "Kladno"), ("es", "Distrito de Kladno"), ("fa", "شهرستان کلادنو"), ("fr", "District de Kladno"), ("he", "מחוז משנה קלדנו"), ("hu", "Kladnói járás"), ("hy", "Սոկոլովի շրջան"), ("it", "Distretto di Kladno"), ("ka", "კლადნოს რაიონი"), ("lt", "Kladno rajonas"), ("lv", "Kladno apriņķis"), ("ms", "Daerah Kladno"), ("nl", "Okres Kladno"), ("pl", "Powiat Kladno"), ("pt", "Kladno"), ("ro", "Districtul Kladno"), ("ru", "Кладно"), ("sk", "Okres Kladno"), ("sr", "Округ Кладно"), ("sr_Latn", "Okrug Kladno"), ("sv", "Kladno"), ("tr", "Kladno ilçesi"), ("uk", "Кладно"), ("ur", "کلاندو ضلع"), ("vi", "Kladno"), ("zh", "克拉德諾縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "204",
                    Subdivision{
                        name: "204",
                        country_alpha2: Alpha2::CZ,
                        code: "204",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كولين"), ("bg", "Колин"), ("ca", "Districte de Kolín"), ("ccp", "𑄇\u{11127}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Okres Kolín"), ("cs", "okres Kolín"), ("de", "Okres Kolín"), ("en", "Kolín"), ("es", "Distrito de Kolín"), ("eu", "Kolíngo barrutia"), ("fa", "شهرستان کولین"), ("fr", "District de Kolín"), ("he", "מחוז משנה קולין"), ("hu", "Kolíni járás"), ("hy", "Բենեշովի շրջան"), ("it", "Distretto di Kolín"), ("ka", "კოლინის რაიონი"), ("lt", "Kolyno rajonas"), ("lv", "Kolīnas apriņķis"), ("ms", "Daerah Kolín"), ("nl", "Okres Kolín"), ("pl", "Powiat Kolín"), ("pt", "Kolín"), ("ru", "Колин"), ("sk", "Okres Kolín"), ("sr", "Округ Колин"), ("sr_Latn", "Okrug Kolin"), ("sv", "Kolín"), ("tr", "Kolín ilçesi"), ("uk", "Колін"), ("ur", "کولین ضلع"), ("vi", "Kolín"), ("zh", "科林縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "205",
                    Subdivision{
                        name: "205",
                        country_alpha2: Alpha2::CZ,
                        code: "205",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوتنا هورا"), ("be", "раён Кутна-Гора"), ("bg", "Кутна Хора"), ("ca", "Districte de Kutná Hora"), ("ccp", "𑄇\u{1112a}𑄖\u{11134}𑄚 𑄦\u{1112e}𑄢"), ("ceb", "Okres Kutná Hora"), ("cs", "okres Kutná Hora"), ("de", "Okres Kutná Hora"), ("en", "Kutná Hora"), ("es", "Distrito de Kutná Hora"), ("eu", "Kutná Horako barrutia"), ("fa", "شهرستان کوتنا هورا"), ("fr", "District de Kutná Hora"), ("he", "מחוז משנה קוטנה הורה"), ("hu", "Kutná Hora-i járás"), ("hy", "Բերոունի շրջան"), ("it", "Distretto di Kutná Hora"), ("ka", "კუტნა-ჰორის რაიონი"), ("lt", "Kutna Horos rajonas"), ("lv", "Kutna Horas apriņķis"), ("mk", "Кутна Хора"), ("ms", "Daerah Kutná Hora"), ("nl", "Okres Kutná Hora"), ("pl", "Powiat Kutná Hora"), ("pt", "Kutná Hora"), ("ru", "Кутна-Гора"), ("sk", "Okres Kutná Hora"), ("sr", "Округ Кутна Хора"), ("sr_Latn", "Okrug Kutna Hora"), ("sv", "Kutná Hora"), ("tr", "Kutná Hora ilçesi"), ("uk", "Кутна Гора (округ)"), ("ur", "کوتنا ہورا ضلع"), ("vi", "Kutná Hora"), ("zh", "庫特納霍拉縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "206",
                    Subdivision{
                        name: "206",
                        country_alpha2: Alpha2::CZ,
                        code: "206",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة منيلنيك"), ("be", "Мельнік"), ("bg", "Мелник"), ("ca", "Districte de Mělník"), ("ccp", "𑄟𑄬𑄣\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Okres Mělník"), ("cs", "okres Mělník"), ("de", "Okres Mělník"), ("en", "Mělník"), ("es", "Distrito de Mělník"), ("fa", "شهرستان میلنیک"), ("fr", "District de Mělník"), ("he", "מחוז משנה מלניק"), ("hu", "Mělníki járás"), ("hy", "Կլադնոյի շրջան"), ("it", "Distretto di Mělník"), ("ja", "ムニェルニーク地区"), ("ka", "მელნიკის რაიონი"), ("lt", "Melnyko rajonas"), ("lv", "Melnīkas apriņķis"), ("ms", "Daerah Mělník"), ("nl", "Okres Mělník"), ("pl", "Powiat Mielnik"), ("pt", "Mělník"), ("ru", "Мельник"), ("sk", "Okres Mělník"), ("sr", "Округ Мјелњик"), ("sr_Latn", "Okrug Mjelnjik"), ("sv", "Mělník"), ("tr", "Mělník ilçesi"), ("uk", "Мельник"), ("ur", "میلنک ضلع"), ("vi", "Mělník"), ("zh", "梅爾尼克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "207",
                    Subdivision{
                        name: "207",
                        country_alpha2: Alpha2::CZ,
                        code: "207",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ملادا بوليسلاف"), ("be", "Млада Болеслаў"), ("bg", "Млада Болеслав"), ("ca", "Districte de Mladá Boleslav"), ("ccp", "𑄟\u{11133}𑄣𑄓 𑄝\u{1112e}𑄣𑄬𑄌\u{11134}𑄣𑄛\u{11134}"), ("ceb", "Okres Mladá Boleslav"), ("cs", "okres Mladá Boleslav"), ("de", "Okres Mladá Boleslav"), ("en", "Mladá Boleslav"), ("es", "Distrito de Mladá Boleslav"), ("eu", "Mladá Boleslaveko barrutia"), ("fa", "شهرستان ملادا بولسلاو"), ("fr", "District de Mladá Boleslav"), ("he", "מחוז משנה מלאדה בולסלאב"), ("hu", "Mladá Boleslav-i járás"), ("hy", "Կոլինի շրջան"), ("it", "Distretto di Mladá Boleslav"), ("ka", "მლადა-ბოლესლავის რაიონი"), ("lt", "Mlada Boleslavo rajonas"), ("lv", "Mlada Boleslavas apriņķis"), ("ms", "Daerah Mladá Boleslav"), ("nl", "Okres Mladá Boleslav"), ("pl", "Powiat Mladá Boleslav"), ("pt", "Mladá Boleslav"), ("ru", "Млада-Болеслав"), ("sk", "Okres Mladá Boleslav"), ("sr", "Округ Млада Болеслав"), ("sr_Latn", "Okrug Mlada Boleslav"), ("sv", "Mladá Boleslav"), ("tr", "Mladá Boleslav ilçesi"), ("uk", "Млада-Болеслав"), ("ur", "ملادا بولسلاو ضلع"), ("vi", "Mladá Boleslav"), ("zh", "姆拉達-博萊斯拉夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "208",
                    Subdivision{
                        name: "208",
                        country_alpha2: Alpha2::CZ,
                        code: "208",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نيمبورك"), ("be", "раён Німбурк"), ("bg", "Нимбурк"), ("ca", "Districte de Nymburk"), ("ccp", "𑄚\u{11128}𑄠𑄟\u{11134}𑄝𑄢\u{11133}𑄇\u{11134}"), ("ceb", "Okres Nymburk"), ("cs", "okres Nymburk"), ("de", "Okres Nymburk"), ("en", "Nymburk"), ("es", "Distrito de Nymburk"), ("fa", "شهرستان نیمبورک"), ("fr", "District de Nymburk"), ("he", "מחוז משנה נימבורק"), ("hu", "Nymburki járás"), ("hy", "Կուտնա Հորայի շրջան"), ("it", "Distretto di Nymburk"), ("ka", "ნიმბურკის რაიონი"), ("lt", "Nimburko rajonas"), ("lv", "Nimburkas apriņķis"), ("ms", "Daerah Nymburk"), ("nl", "Okres Nymburk"), ("pl", "Powiat Nymburk"), ("pt", "Nymburk"), ("ru", "Нимбурк"), ("sk", "Okres Nymburk"), ("sr", "Округ Нимбурк"), ("sr_Latn", "Okrug Nimburk"), ("sv", "Nymburk"), ("uk", "Нимбурк (округ)"), ("ur", "نمبورک ضلع"), ("vi", "Nymburk"), ("zh", "寧布爾克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "209",
                    Subdivision{
                        name: "209",
                        country_alpha2: Alpha2::CZ,
                        code: "209",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة براغ-شرق"), ("be", "Прага-ўсход"), ("bg", "Прага-изток"), ("ca", "Districte de Praga-Est"), ("ccp", "𑄛\u{11133}𑄢𑄉\u{1112a}𑄠𑄬-𑄛\u{1112a}𑄇\u{11134}"), ("ceb", "Okres Praha-Východ"), ("cs", "okres Praha-východ"), ("de", "Okres Praha-východ"), ("en", "Prague-East"), ("es", "Distrito de Praga-Este"), ("eu", "Praga-Ekialdeko barrutia"), ("fa", "شهرستان پراگ-شرق"), ("fr", "District de Prague-est"), ("he", "מחוז משנה פראג-מזרח"), ("hu", "Kelet-prágai járás"), ("hy", "Պրահայի արևելք"), ("it", "Distretto di Praha-východ"), ("ka", "პრაღა-ვიხოდის რაიონი"), ("lt", "Prahos rytinis rajonas"), ("lv", "Prāgas Austrumu apriņķis"), ("ms", "Daerah Prague-Timur"), ("nl", "Okres Praha-východ"), ("pl", "Powiat Praga Wschód"), ("ru", "Прага-восток"), ("sk", "Okres Praha-východ"), ("sr", "Округ Праг-исток"), ("sr_Latn", "Okrug Prag-istok"), ("sv", "Praha-východ"), ("uk", "Прага-схід"), ("ur", "پراگ-شرقی ضلع"), ("vi", "Praha Đông"), ("zh", "東布拉格縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "20A",
                    Subdivision{
                        name: "20A",
                        country_alpha2: Alpha2::CZ,
                        code: "20A",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة براغ-غرب"), ("bg", "Прага-запад"), ("ca", "Districte de Praga-Oest"), ("ccp", "𑄛\u{11133}𑄢𑄉\u{1112a}𑄠𑄬-𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}"), ("ceb", "Okres Praha-Západ"), ("cs", "okres Praha-západ"), ("de", "Okres Praha-západ"), ("en", "Prague-West"), ("es", "Distrito de Praga-Oeste"), ("fa", "شهرستان پراگ-غرب"), ("fr", "District de Prague-ouest"), ("he", "מחוז משנה פראג-מערב"), ("hu", "Nyugat-prágai járás"), ("it", "Distretto di Praha-západ"), ("ka", "პრაღა-ზაპადის რაიონი"), ("lt", "Prahos vakarinis rajonas"), ("ms", "Daerah Prague-Barat"), ("nl", "Okres Praha-západ"), ("pl", "Powiat Praga Zachód"), ("ru", "Прага-запад"), ("sk", "Okres Praha-západ"), ("sr", "Округ Праг-запад"), ("sr_Latn", "Okrug Prag-zapad"), ("sv", "Praha-západ"), ("uk", "Прага-захід"), ("ur", "پراگ-غربی ضلع"), ("vi", "Praha Tây"), ("zh", "西布拉格縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "20B",
                    Subdivision{
                        name: "20B",
                        country_alpha2: Alpha2::CZ,
                        code: "20B",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بشيبرام"), ("bg", "Пршибрам"), ("ca", "Districte de Příbram"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄝\u{11133}𑄢𑄟\u{11134}"), ("ceb", "Okres Příbram"), ("cs", "okres Příbram"), ("de", "Okres Příbram"), ("en", "Příbram"), ("es", "Distrito de Příbram"), ("eu", "Příbramgo barrutia"), ("fa", "شهرستان پرژیبرام"), ("fr", "District de Příbram"), ("he", "מחוז משנה פריברם"), ("hu", "Příbrami járás"), ("hy", "Նիմբուրկի շրջան"), ("it", "Distretto di Příbram"), ("ka", "პრშიბრამის რაიონი"), ("lt", "Pršybramo rajonas"), ("mk", "Пшибрам"), ("ms", "Daerah Příbram"), ("nl", "Okres Příbram"), ("pl", "powiat Przybram"), ("pt", "Příbram"), ("ru", "Пршибрам"), ("sk", "Okres Příbram"), ("sr", "Округ Прибрам"), ("sr_Latn", "Okrug Pribram"), ("sv", "Příbram"), ("tr", "Příbram ilçesi"), ("uk", "Пржибрам"), ("ur", "پشیبرام ضلع"), ("vi", "Příbram"), ("zh", "普日布拉姆縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "20C",
                    Subdivision{
                        name: "20C",
                        country_alpha2: Alpha2::CZ,
                        code: "20C",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة راكوفنيك"), ("bg", "Раковник"), ("ca", "Districte de Rakovník"), ("ccp", "𑄢\u{11127}𑄇\u{1112e}𑄛\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Okres Rakovník"), ("cs", "okres Rakovník"), ("de", "Okres Rakovník"), ("en", "Rakovník"), ("es", "Distrito de Rakovník"), ("eu", "Rakovníkeko barrutia"), ("fa", "شهرستان راکوونیک"), ("fr", "District de Rakovník"), ("he", "מחוז משנה ראקובניק"), ("hu", "Rakovníki járás"), ("hy", "Արևելապրահյան շրջան"), ("it", "Distretto di Rakovník"), ("ka", "რაკოვნიკის რაიონი"), ("lt", "Rakovnyko rajonas"), ("ms", "Daerah Rakovník"), ("nl", "Okres Rakovník"), ("pl", "Powiat Rakovník"), ("pt", "Rakovník"), ("ru", "Раковник"), ("sk", "Okres Rakovník"), ("sr", "Округ Раковњик"), ("sr_Latn", "Okrug Rakovnjik"), ("sv", "Rakovník"), ("tr", "Rakovník ilçesi"), ("uk", "Раковнік"), ("ur", "راکوونیک ضلع"), ("vi", "Rakovník"), ("zh", "拉科夫尼克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::CZ,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.9457789), longitude: Some(14.4416055), max_latitude: Some(49.6212548), min_latitude: Some(48.5518081), max_longitude: Some(15.6041293), min_longitude: Some(13.5363203)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جنوب بوهيميا"), ("az", "Cənubi Çex diyarı"), ("be", "Паўднёвачэшскі край"), ("bg", "Южнобохемски край"), ("bn", "স\u{9be}উথ ব\u{9c1}হেমিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("bs", "Južnočeški kraj"), ("ca", "Regió de Bohèmia Meridional"), ("ccp", "𑄎\u{11128}𑄦\u{1112e}𑄥𑄬𑄌\u{11134}𑄇\u{11128}"), ("ceb", "Jihočeský kraj"), ("cs", "Jihočeský kraj"), ("da", "Sydbøhmen"), ("de", "Jihočeský kraj"), ("el", "Περιφέρεια Νότιας Βοημίας"), ("en", "Jihočeský"), ("es", "Región de Bohemia Meridional"), ("et", "Lõuna-Tšehhi maakond"), ("eu", "Hegoaldeko Bohemiako eskualdea"), ("fa", "منطقه بوهمی جنوبی"), ("fi", "Etelä-Böömin lääni"), ("fr", "Bohême-du-Sud"), ("gu", "દક\u{acd}ષિણ બોહ\u{ac7}મિયા પ\u{acd}રદ\u{ac7}શ"), ("he", "דרום בוהמיה"), ("hi", "दक\u{94d}षिण बोह\u{947}मियाई क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Južna Češka"), ("hu", "Dél-csehországi kerület"), ("id", "Daerah Bohemia Selatan"), ("it", "Boemia meridionale"), ("ja", "南ボヘミア州"), ("ka", "სამხრეთ ბოჰემიის მხარე"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಬೊಹ\u{cc6}ಮ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "남보헤미아 주"), ("lt", "Pietų Čekijos kraštas"), ("lv", "Dienvidčehijas apgabals"), ("mk", "Јужночешки крај"), ("mr", "दक\u{94d}षिण बोह\u{947}मिया प\u{94d}रद\u{947}श"), ("ms", "Daerah Bohemia Selatan"), ("nb", "Sydbøhmen region"), ("nl", "Zuid-Bohemen"), ("no", "Sydbøhmen region"), ("pl", "Kraj południowoczeski"), ("pt", "Boêmia do Sul"), ("ro", "Regiunea Boemia de Sud"), ("ru", "Южночешский край"), ("si", "දක\u{dd4}ණ\u{dd4} බොහ\u{dd2}ම\u{dd2}ය\u{dcf} කල\u{dcf}පය"), ("sk", "Juhočeský kraj"), ("sr", "Јужночешки крај"), ("sr_Latn", "Južnočeški kraj"), ("sv", "Södra Böhmen"), ("ta", "தெற\u{bcd}கு பொஹ\u{bc0}மிய\u{bbe} பகுதி"), ("te", "దక\u{c4d}ష\u{c3f}ణ బ\u{c4a}హ\u{c46}మ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตโบฮ\u{e35}เม\u{e35}ยนตอนใต\u{e49}"), ("tr", "Güney Bohemya ili"), ("uk", "Південночеський край"), ("ur", "جنوبی بوہیمیائی علاقہ"), ("vi", "Nam Bohemia"), ("zh", "南波希米亚州")]),
                        unofficial_name_list: ["Budějovický", "Českobudějovický"].to_vec(),
                    }
                ),
                (
                    "311",
                    Subdivision{
                        name: "311",
                        country_alpha2: Alpha2::CZ,
                        code: "311",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيسكي بوديوفيتسه"), ("be", "Раён Чэске-Будзеёвіцы"), ("bg", "Ческе Будейовице"), ("ca", "Districte de České Budějovice"), ("ccp", "𑄇𑄬𑄌\u{11134}𑄇𑄬 𑄝𑄓𑄬𑄎\u{1112e}𑄞\u{1112d}𑄌\u{11134}"), ("ceb", "Okres České Budějovice"), ("cs", "okres České Budějovice"), ("de", "Okres České Budějovice"), ("en", "České Budějovice"), ("es", "Distrito de České Budějovice"), ("eu", "České Budějoviceko barrutia"), ("fa", "شهرستان چسکه بودیوویتسه"), ("fr", "District de České Budějovice"), ("he", "מחוז משנה צ׳סקה בודייוביצה"), ("hu", "České Budějovice-i járás"), ("hy", "Չեսկե Բուդեյովիցեի շրջան"), ("it", "Distretto di České Budějovice"), ("ka", "ჩესკე-ბუდეიოვიცეს რაიონი"), ("lt", "Česke Budejovicų rajonas"), ("ms", "Daerah České Budějovice"), ("nb", "České Budějovice"), ("nl", "Okres České Budějovice"), ("no", "České Budějovice"), ("pl", "Powiat Czeskie Budziejowice"), ("pt", "České Budějovice"), ("ro", "Okres České Budějovice"), ("ru", "Ческе-Будеёвице"), ("sk", "Okres České Budějovice"), ("sr", "Округ Чешке Будјејовице"), ("sr_Latn", "Okrug Češke Budjejovice"), ("sv", "České Budějovice"), ("tr", "České Budějovice ilçesi"), ("uk", "Чеське Будейовіце (округ)"), ("ur", "چسکے بودجیووسکے ضلع"), ("vi", "České Budějovice"), ("zh", "捷克布傑約維采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "312",
                    Subdivision{
                        name: "312",
                        country_alpha2: Alpha2::CZ,
                        code: "312",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيسكي كروملوف"), ("be", "раён Чэскі-Крумлаў"), ("bg", "Чески Крумлов (окръг)"), ("ca", "Districte de Český Krumlov"), ("ccp", "𑄇𑄬𑄌\u{11134}𑄇\u{11128} 𑄇\u{11133}𑄢\u{1112a}𑄟𑄣\u{11127}𑄛\u{11134}"), ("ceb", "Okres Český Krumlov"), ("cs", "okres Český Krumlov"), ("de", "Okres Český Krumlov"), ("en", "Český Krumlov"), ("es", "Distrito de Český Krumlov"), ("eu", "Český Krumloveko barrutia"), ("fa", "شهرستان چسکی کروملوف"), ("fr", "District de Český Krumlov"), ("he", "מחוז משנה צ׳סקי קרומלוב"), ("hu", "Český Krumlov-i járás"), ("hy", "Չեսկե Կրումլովի շրջան"), ("it", "Distretto di Český Krumlov"), ("ja", "チェスキー・クルムロフ郡"), ("ka", "ჩესკი-კრუმლოვის რაიონი"), ("lt", "Česky Krumlovo rajonas"), ("ms", "Daerah Český Krumlov"), ("nb", "Český Krumlov"), ("nl", "Okres Český Krumlov"), ("no", "Český Krumlov"), ("pl", "Powiat Český Krumlov"), ("pt", "Český Krumlov"), ("ro", "Okres Český Krumlov"), ("ru", "Чески-Крумлов"), ("sk", "Okres Český Krumlov"), ("sr", "Округ Чешки Крумлов"), ("sr_Latn", "Okrug Češki Krumlov"), ("sv", "Český Krumlov"), ("tr", "Český Krumlov ilçesi"), ("uk", "Чеський Крумлов"), ("ur", "چیسکی کروملوف ضلع"), ("vi", "Český Krumlov"), ("zh", "捷克克魯姆洛夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "313",
                    Subdivision{
                        name: "313",
                        country_alpha2: Alpha2::CZ,
                        code: "313",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يندريخوف هرادتس"), ("bg", "Индржихув Храдец (окръг)"), ("ca", "Districte de Jindřichův Hradec"), ("ccp", "𑄎\u{11128}𑄚\u{11134}𑄘\u{11133}𑄢\u{1112d}𑄇\u{1112a}𑄛\u{11134} 𑄢𑄓𑄬𑄇\u{11134}"), ("ceb", "Okres Jindřichův Hradec"), ("cs", "okres Jindřichův Hradec"), ("de", "Okres Jindřichův Hradec"), ("en", "Jindřichův Hradec"), ("es", "Distrito de Jindřichův Hradec"), ("eu", "Jindřichův Hradeceko barrutia"), ("fa", "شهرستان ییندرژیخوف هرادتس"), ("fr", "District de Jindřichův Hradec"), ("he", "מחוז משנה יינדריחוב הרדץ"), ("hu", "Jindřichův Hradec-i járás"), ("hy", "Յինդրիխուվ Հրադեցի շրջան"), ("it", "Distretto di Jindřichův Hradec"), ("ja", "インジフーフ・フラデツ郡"), ("ka", "იინდრჟიხუვ-ჰრადეცის რაიონი"), ("lt", "Jindržichūv Hradeco rajonas"), ("ms", "Daerah Jindřichův Hradec"), ("nb", "Jindřichův Hradec"), ("nl", "Okres Jindřichův Hradec"), ("no", "Jindřichův Hradec"), ("pl", "Powiat Jindřichův Hradec"), ("pt", "Jindřichův Hradec"), ("ro", "Okres Jindřichův Hradec"), ("ru", "Йиндржихув-Градец"), ("sk", "Okres Jindřichův Hradec"), ("sr", "Округ Јиндрихув Храдец"), ("sr_Latn", "Okrug Jindrihuv Hradec"), ("sv", "Jindřichův Hradec"), ("tr", "Jindřichův Hradec ilçesi"), ("uk", "Їндржіхув Градець"), ("ur", "جندرشیخوو ہاردک ضلع"), ("vi", "Jindřichův Hradec"), ("zh", "因德日赫城堡縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "314",
                    Subdivision{
                        name: "314",
                        country_alpha2: Alpha2::CZ,
                        code: "314",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيسك"), ("be", "Пісек"), ("bg", "Писек"), ("ca", "Districte de Písek"), ("ccp", "𑄛\u{1112d}𑄥𑄬𑄇\u{11134}"), ("ceb", "Okres Písek"), ("cs", "okres Písek"), ("de", "Okres Písek"), ("en", "Písek"), ("es", "Distrito de Písek"), ("eu", "Písekeko barrutia"), ("fa", "شهرستان پیسک"), ("fr", "District de Písek"), ("he", "מחוז משנה פיסק"), ("hu", "Píseki járás"), ("hy", "Պիսեկի շրջան"), ("it", "Distretto di Písek"), ("ka", "პისეკის რაიონი"), ("lt", "Piseko rajonas"), ("ms", "Daerah Písek"), ("nb", "Písek"), ("nl", "Okres Písek"), ("no", "Písek"), ("pl", "Powiat Písek"), ("pt", "Písek"), ("ru", "Писек"), ("sk", "Okres Písek"), ("sr", "Округ Писек"), ("sr_Latn", "Okrug Pisek"), ("sv", "Písek"), ("tr", "Písek ilçesi"), ("uk", "Пісек"), ("ur", "پیسک ضلع"), ("vi", "Písek"), ("zh", "皮塞克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "315",
                    Subdivision{
                        name: "315",
                        country_alpha2: Alpha2::CZ,
                        code: "315",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة براخاتيتسه"), ("be", "Прахаціцы"), ("bg", "Прахатице"), ("ca", "Districte de Prachatice"), ("ccp", "𑄛\u{11133}𑄢𑄌𑄑\u{1112d}𑄌\u{11134}"), ("ceb", "Okres Prachatice"), ("cs", "okres Prachatice"), ("de", "Okres Prachatice"), ("el", "Επαρχία Πρατσάτιτσε"), ("en", "Prachatice"), ("es", "Distrito de Prachatic"), ("eu", "Prachaticeko barrutia"), ("fa", "شهرستان پراخاتیتسه"), ("fr", "District de Prachatice"), ("hu", "Prachaticei járás"), ("hy", "Պրախատիցեի շրջան"), ("it", "Distretto di Prachatice"), ("ka", "პრახატიცეს რაიონი"), ("lt", "Prachaticių rajonas"), ("ms", "Daerah Prachatice"), ("nb", "Prachatice"), ("nl", "Okres Prachatice"), ("no", "Prachatice"), ("pl", "Powiat Prachatice"), ("pt", "Prachatice"), ("ru", "Прахатице"), ("sk", "Okres Prachatice"), ("sr", "Округ Прахатице"), ("sr_Latn", "Okrug Prahatice"), ("sv", "Okres Prachatice"), ("tr", "Prachatice İlçesi"), ("uk", "Прахатіце"), ("ur", "پراخاتسے ضلع"), ("vi", "Prachatice"), ("zh", "普拉哈季采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "316",
                    Subdivision{
                        name: "316",
                        country_alpha2: Alpha2::CZ,
                        code: "316",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ستراكونيتسه"), ("bg", "Страконице"), ("ca", "Districte de Strakonice"), ("ccp", "𑄌\u{11133}𑄑\u{11133}𑄢\u{11133}𑄢\u{11133}𑄢𑄇\u{1112e}𑄚\u{1112d}𑄌\u{11134}"), ("ceb", "Okres Strakonice"), ("cs", "okres Strakonice"), ("de", "Okres Strakonice"), ("en", "Strakonice"), ("es", "Distrito de Strakonice"), ("eu", "Strakoniceko barrutia"), ("fa", "شهرستان استراکونیتسه"), ("fr", "District de Strakonice"), ("he", "מחוז משנה סטרוקניצה"), ("hu", "Strakonicei járás"), ("hy", "Ստրակոնիցեի շրջան"), ("it", "Distretto di Strakonice"), ("ka", "სტრაკონიცეს რაიონი"), ("lt", "Strakonicių rajonas"), ("ms", "Daerah Strakonice"), ("nb", "Strakonice"), ("nl", "Okres Strakonice"), ("no", "Strakonice"), ("pl", "Powiat Strakonice"), ("pt", "Strakonice"), ("ro", "Okres Strakonice"), ("ru", "Страконице"), ("sk", "Okres Strakonice"), ("sr", "Округ Стракоњице"), ("sr_Latn", "Okrug Strakonjice"), ("sv", "Okres Strakonice"), ("tr", "Strakonice ilçesi"), ("uk", "Страконіце"), ("ur", "ستراکونیسے ضلع"), ("vi", "Strakonice"), ("zh", "斯特拉科尼采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "317",
                    Subdivision{
                        name: "317",
                        country_alpha2: Alpha2::CZ,
                        code: "317",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تابور"), ("be", "Табар"), ("bg", "Табор"), ("ca", "Districte de Tábor"), ("ccp", "𑄑𑄝\u{11127}𑄢\u{11134}"), ("ceb", "Okres Tábor"), ("cs", "okres Tábor"), ("de", "Okres Tábor"), ("en", "Tábor"), ("es", "Distrito de Tábor"), ("eu", "Táborko barrutia"), ("fa", "منطقه تابور"), ("fr", "District de Tábor"), ("he", "מחוז משנה טאבור"), ("hu", "Tábori járás"), ("hy", "Տաբորի շրջան"), ("it", "Distretto di Tábor"), ("ka", "ტაბორის რაიონი"), ("lt", "Taboro rajonas"), ("ms", "Daerah Tábor"), ("nb", "Tábor"), ("nl", "Okres Tábor"), ("no", "Tábor"), ("pl", "Powiat Tabor"), ("ru", "Табор"), ("sk", "Okres Tábor"), ("sr", "Округ Табор"), ("sr_Latn", "Okrug Tabor"), ("sv", "Okres Tábor"), ("tr", "Tábor ilçesi"), ("uk", "Табор"), ("ur", "تابور ضلع"), ("vi", "Tábor"), ("zh", "塔博爾縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::CZ,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.4134812), longitude: Some(13.3157246), max_latitude: Some(50.1033335), min_latitude: Some(48.9412671), max_longitude: Some(13.835103), min_longitude: Some(12.400522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بلزن"), ("az", "Plzen diyarı"), ("be", "Пльзенскі край"), ("bg", "Пилзенски край"), ("bn", "প\u{9be}জেন অঞ\u{9cd}চল"), ("bs", "Plzenjski kraj"), ("ca", "Regió de Plzeň"), ("ccp", "𑄛\u{11133}𑄣𑄬𑄚\u{11134}𑄥\u{11134}𑄇\u{11129}"), ("ceb", "Plzeňský kraj"), ("cs", "Plzeňský kraj"), ("da", "Plzeň"), ("de", "Plzeňský kraj"), ("el", "Περιφέρεια Πίλσεν"), ("en", "Plzeňský"), ("es", "Región de Pilsen"), ("et", "Plzeňi maakond"), ("eu", "Pilsen eskualdea"), ("fa", "منطقه پلزن"), ("fi", "Plzeňin lääni"), ("fr", "Région de Plzeň"), ("gu", "પીલ\u{acd}ઝ\u{ac7}ન પ\u{acd}રદ\u{ac7}શ"), ("he", "פלזן (מחוז)"), ("hi", "प\u{94d}ल\u{947}ज\u{947}न प\u{94d}रद\u{947}श"), ("hr", "Plzeňski kraj"), ("hu", "Plzeňi kerület"), ("id", "Daerah Plzeň"), ("it", "regione di Plzeň"), ("ja", "プルゼニ州"), ("ka", "პლზენის მხარე"), ("kn", "ಪ\u{ccd}ಲ\u{cc6}ಝ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "플젠 주"), ("lt", "Pilzeno kraštas"), ("lv", "Plzeņas apgabals"), ("mr", "प\u{94d}ल\u{945}ज\u{93c}\u{947}न प\u{94d}रद\u{947}श"), ("ms", "Daerah Plzeň"), ("nb", "Plzeň region"), ("nl", "Pilsen"), ("no", "Plzeň region"), ("pl", "Kraj pilzneński"), ("pt", "Plzeň"), ("ro", "Regiunea Plzeň"), ("ru", "Пльзенский край"), ("si", "ප\u{dca}ල\u{dca}සෙන\u{dca} කල\u{dcf}පය"), ("sk", "Plzenský kraj"), ("sr", "Плзењски крај"), ("sr_Latn", "Plzenjski kraj"), ("sv", "Plzeň"), ("ta", "பில\u{bcd}ஜென\u{bcd} பகுதி"), ("te", "ప\u{c4d}ల\u{c3f}జ\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ภ\u{e39}ม\u{e34}ภาคพล\u{e35}เซน"), ("tr", "Plzen Region"), ("uk", "Пльзенський край"), ("ur", "پلزین علاقہ"), ("vi", "Plzeň"), ("zh", "比爾森州")]),
                        unofficial_name_list: ["Plzeňský kraj"].to_vec(),
                    }
                ),
                (
                    "321",
                    Subdivision{
                        name: "321",
                        country_alpha2: Alpha2::CZ,
                        code: "321",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوماجليتسه"), ("bg", "Домажлице"), ("ca", "Districte de Domažlice"), ("ccp", "𑄓\u{1112e}𑄟𑄬𑄌\u{11134}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Okres Domažlice"), ("cs", "okres Domažlice"), ("de", "Okres Domažlice"), ("en", "Domažlice"), ("es", "Distrito de Domažlice"), ("eu", "Domažliceko barrutia"), ("fa", "شهرستان دوماژلیتسه"), ("fr", "District de Domažlice"), ("he", "מחוז משנה דומזליצה"), ("hu", "Domažlicei járás"), ("hy", "Դոմաժլիցեի շրջան"), ("it", "Distretto di Domažlice"), ("ka", "დომაჟლიცეს რაიონი"), ("lt", "Domažlicių rajonas"), ("ms", "Daerah Domažlice"), ("nl", "Okres Domažlice"), ("pl", "Powiat Domažlice"), ("pt", "Domažlice (distrito)"), ("ru", "Домажлице"), ("sk", "Okres Domažlice"), ("sr", "Округ Домажлице"), ("sr_Latn", "Okrug Domažlice"), ("sv", "Okres Domažlice"), ("tr", "Domažlice ilçesi"), ("uk", "Домажліце"), ("ur", "دوماشزلیسے ضلع"), ("vi", "Domažlice"), ("zh", "多馬日利采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "322",
                    Subdivision{
                        name: "322",
                        country_alpha2: Alpha2::CZ,
                        code: "322",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كلاتوفي"), ("be", "раён Клатаві"), ("bg", "Клатови"), ("ca", "Districte de Klatovy"), ("ccp", "𑄇\u{11133}𑄣𑄑\u{1112e}𑄞\u{11128}"), ("ceb", "Okres Klatovy"), ("cs", "okres Klatovy"), ("de", "Okres Klatovy"), ("en", "Klatovy"), ("es", "Distrito de Klatovy"), ("eu", "Klatovyko barrutia"), ("fa", "شهرستان کلاتووی"), ("fr", "District de Klatovy"), ("he", "מחוז משנה קלאטובי"), ("hu", "Klatovy-i járás"), ("hy", "Կլատովի շրջան"), ("it", "Distretto di Klatovy"), ("ka", "კლატოვის რაიონი"), ("lt", "Klatovų rajonas"), ("ms", "Daerah Klatovy"), ("nl", "Okres Klatovy"), ("pl", "Powiat Klatovy"), ("pt", "Klatovy"), ("ru", "Клатови"), ("sk", "Okres Klatovy"), ("sr", "Округ Клатови"), ("sr_Latn", "Okrug Klatovi"), ("sv", "Okres Klatovy"), ("tr", "Klatovy ilçesi"), ("uk", "Клатови"), ("ur", "کلاتووی ضلع"), ("vi", "Klatovy"), ("zh", "克拉托維縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "323",
                    Subdivision{
                        name: "323",
                        country_alpha2: Alpha2::CZ,
                        code: "323",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلزن-المدينة"), ("be", "акруга Пльзень-горад"), ("bg", "Пилзен-град"), ("ca", "Districte de Plzeň-město"), ("ccp", "𑄛\u{11133}𑄣𑄎𑄬𑄚\u{11134}"), ("ceb", "Okres Plzeň-Město"), ("cs", "okres Plzeň-město"), ("de", "Okres Plzeň-město"), ("en", "Plzeň"), ("es", "Distrito de Ciudad de Pilsen"), ("fa", "شهرستان پلزن-میستو"), ("fr", "District de Plzeň-město"), ("he", "מחוז משנה פלזן מסטו"), ("hu", "Plzeň városi járás"), ("it", "Distretto di Plzeň-město"), ("lt", "Pilzeno miesto rajonas"), ("ms", "Daerah Bandaraya Plzeň"), ("nl", "Okres Plzeň-město"), ("pt", "Distrito de Plzeň-Ciudad"), ("ro", "districtul orașului Plzeň"), ("ru", "Пльзень-город"), ("sk", "Okres Plzeň-mesto"), ("sr", "Округ Плзењ-град"), ("sr_Latn", "Okrug Plzenj-grad"), ("sv", "Okres Plzeň-Město"), ("uk", "Пльзень-місто"), ("ur", "پلزین-شہر ضلع"), ("zh", "比爾森城縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "324",
                    Subdivision{
                        name: "324",
                        country_alpha2: Alpha2::CZ,
                        code: "324",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلزن-جنوب"), ("bg", "Пилзен-юг"), ("ca", "Districte de Plzeň-jih"), ("ccp", "𑄛\u{11133}𑄣𑄎𑄬𑄚\u{11134}-𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}"), ("ceb", "Okres Plzeň-Jih"), ("cs", "okres Plzeň-jih"), ("de", "Okres Plzeň-jih"), ("en", "Plzeň-South"), ("es", "Distrito de Pilsen Sur"), ("eu", "Plzeň-Hegoaldea barrutia"), ("fa", "ناحیه پلزن-جنوبی"), ("fr", "District de Plzeň-jih"), ("hu", "Dél-plzeňi járás"), ("it", "Distretto di Plzeň-jih"), ("ka", "პლზენ-იიჰის რაიონი"), ("lt", "Pilzeno pietinis rajonas"), ("ms", "Daerah Plzeň-Selatan"), ("nl", "Okres Plzeň-jih"), ("pl", "Powiat Pilzno Południe"), ("pt", "Distrito de Plzeň-Sul"), ("ru", "Пльзень-юг"), ("sk", "Okres Plzeň-juh"), ("sr", "Округ Плзењ-југ"), ("sr_Latn", "Okrug Plzenj-jug"), ("sv", "Okres Plzeň-Jih"), ("uk", "Пльзень-південь"), ("ur", "پلزین-جنوبی ضلع"), ("vi", "Plzeň-Nam"), ("zh", "南比爾森縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "325",
                    Subdivision{
                        name: "325",
                        country_alpha2: Alpha2::CZ,
                        code: "325",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلزن-شمال"), ("bg", "Пилзен-север"), ("ca", "Districte de Plzeň-sever"), ("ccp", "𑄛\u{11133}𑄣𑄎𑄬𑄚\u{11134}-𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}"), ("ceb", "Okres Plzeň-Sever"), ("cs", "okres Plzeň-sever"), ("de", "Okres Plzeň-sever"), ("en", "Plzeň-North"), ("es", "Distrito de Pilsen Norte"), ("eu", "Plzeň-Iparraldeko barrutia"), ("fa", "ناحیه پلزن-شمالی"), ("fr", "District de Plzeň-sever"), ("hu", "Észak-plzeňi járás"), ("it", "Distretto di Plzeň-sever"), ("ka", "პლზენ-სევერის რაიონი"), ("lt", "Pilzeno šiaurinis rajonas"), ("ms", "Daerah Plzeň-Utara"), ("nl", "Okres Plzeň-sever"), ("pl", "Powiat Pilzno Północ"), ("ru", "Пльзень-север"), ("sk", "Okres Plzeň-sever"), ("sr", "Округ Плзењ-север"), ("sr_Latn", "Okrug Plzenj-sever"), ("sv", "Okres Plzeň-Sever"), ("uk", "Пльзень-північ"), ("ur", "پلزین-شمالی ضلع"), ("vi", "Plzeň-Bắc"), ("zh", "北比爾森縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "326",
                    Subdivision{
                        name: "326",
                        country_alpha2: Alpha2::CZ,
                        code: "326",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة روكيتساني"), ("bg", "Рокицани"), ("ca", "Districte de Rokycany"), ("ccp", "𑄢\u{11127}𑄇\u{11128}𑄇\u{11133}𑄠𑄚\u{11128}"), ("ceb", "Okres Rokycany"), ("cs", "okres Rokycany"), ("de", "Okres Rokycany"), ("en", "Rokycany"), ("es", "Distrito de Rokycany"), ("eu", "Rokycanyko barrutia"), ("fa", "شهرستان روکیتسانی"), ("fr", "District de Rokycany"), ("he", "מחוז משנה רוקיצאני"), ("hu", "Rokycany-i járás"), ("hy", "Ռոկիցանկի շրջան"), ("it", "Distretto di Rokycany"), ("ka", "როკიცანის რაიონი"), ("lt", "Rokicanų rajonas"), ("ms", "Daerah Rokycany"), ("nl", "Okres Rokycany"), ("pl", "Powiat Rokycany"), ("pt", "Rokycany"), ("ru", "Рокицани"), ("sk", "Okres Rokycany"), ("sr", "Округ Рокицани"), ("sr_Latn", "Okrug Rokicani"), ("sv", "Okres Rokycany"), ("tr", "Rokycany ilçesi"), ("uk", "Рокицани"), ("ur", "روکیسانی ضلع"), ("vi", "Rokycany"), ("zh", "羅基察尼縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "327",
                    Subdivision{
                        name: "327",
                        country_alpha2: Alpha2::CZ,
                        code: "327",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تاخوف"), ("bg", "Тахов"), ("ca", "Districte de Tachov"), ("ccp", "𑄑𑄝\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Tachov"), ("cs", "okres Tachov"), ("de", "Okres Tachov"), ("en", "Tachov"), ("es", "Distrito de Tachov"), ("eu", "Tachoveko barrutia"), ("fa", "شهرستان تاخوف"), ("fr", "District de Tachov"), ("he", "מחוז משנה טכוב"), ("hu", "Tachovi járás"), ("hy", "Տախովի շրջան"), ("it", "Distretto di Tachov"), ("ja", "タホフ郡"), ("ka", "ტახოვის რაიონი"), ("lt", "Tachovo rajonas"), ("ms", "Daerah Tachov"), ("nl", "Okres Tachov"), ("pl", "Powiat Tachov"), ("pt", "Tachov"), ("ru", "Тахов"), ("sk", "Okres Tachov"), ("sr", "Округ Тахов"), ("sr_Latn", "Okrug Tahov"), ("sv", "Okres Tachov"), ("tr", "Tachov ilçesi"), ("uk", "Тахов"), ("ur", "تاخوف ضلع"), ("vi", "Tachov"), ("zh", "塔霍夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::CZ,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.1435), longitude: Some(12.7501899), max_latitude: Some(50.4602991), min_latitude: Some(49.891137), max_longitude: Some(13.3011974), min_longitude: Some(12.0906263)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كارلوفي فاري"), ("az", "Karlovı Varı diyarı"), ("be", "Карлаварскі край"), ("bg", "Карловарски край"), ("bn", "ক\u{9be}র\u{9cd}লোভি ভেরি অঞ\u{9cd}চল"), ("bs", "Karlovarski kraj"), ("ca", "Regió de Karlovy Vary"), ("ccp", "𑄇𑄢\u{11134}𑄣\u{1112e}𑄞𑄢\u{11134}𑄇\u{11129}"), ("ceb", "Karlovarský kraj"), ("cs", "Karlovarský kraj"), ("da", "Karlovy Vary"), ("de", "Karlovarský kraj"), ("el", "Περιφέρεια Κάρλοβυ Βάρυ"), ("en", "Karlovarský"), ("es", "Región de Karlovy Vary"), ("et", "Karlovy Vary maakond"), ("eu", "Karlovy Vary eskualdea"), ("fa", "منطقه کارلووی واری"), ("fi", "Karlovy Varyn lääni"), ("fr", "Région de Karlovy Vary"), ("gu", "કાર\u{acd}લોવી વ\u{ac7}રી પ\u{acd}રદ\u{ac7}શ"), ("he", "קרלובי וארי"), ("hi", "कार\u{94d}लोवी व\u{947}री प\u{94d}रद\u{947}श"), ("hr", "Karlovarski kraj"), ("hu", "Karlovy Vary-i kerület"), ("id", "Daerah Karlovy Vary"), ("it", "regione di Karlovy Vary"), ("ja", "カルロヴィ・ヴァリ州"), ("ka", "კარლოვი-ვარის მხარე"), ("kn", "ಕಾರ\u{ccd}ಲೋವ\u{cbf} ವೇರ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "카를로비바리 주"), ("lt", "Karlovi Varų kraštas"), ("lv", "Karlovi Varu reģions"), ("mk", "Карловарски крај"), ("mr", "कार\u{94d}लोव\u{94d}ही व\u{947}र\u{947} प\u{94d}रद\u{947}श"), ("ms", "Daerah Karlovy Vary"), ("nb", "Karlovy Vary region"), ("nl", "Karlsbad (regio)"), ("no", "Karlovy Vary region"), ("pl", "Kraj karlowarski"), ("pt", "Karlovy Vary"), ("ro", "Regiunea Karlovy Vary"), ("ru", "Карловарский край"), ("si", "කර\u{dca}ලොව\u{dd3} වෙර\u{dd2} කල\u{dcf}පය"), ("sk", "Karlovarský kraj"), ("sr", "Карловарски крај"), ("sr_Latn", "Karlovarski kraj"), ("sv", "Karlovy Vary"), ("ta", "க\u{bbe}ர\u{bcd}லோவ\u{bcd}ய\u{bcd} வ\u{bbe}ரி பகுதி"), ("te", "క\u{c3e}ర\u{c4d}ల\u{c4b}వ\u{c40} వ\u{c47}ర\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาร\u{e4c}โลว\u{e35}\u{e48} เวร\u{e35}\u{e48} ร\u{e35}เจ\u{e35}\u{e49}ยน"), ("tr", "Karlovy Vary ili"), ("uk", "Карловарський край"), ("ur", "کارلووی واری علاقہ"), ("vi", "Karlovy Vary"), ("zh", "卡羅維發利州")]),
                        unofficial_name_list: ["Karlovarský kraj"].to_vec(),
                    }
                ),
                (
                    "411",
                    Subdivision{
                        name: "411",
                        country_alpha2: Alpha2::CZ,
                        code: "411",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خب"), ("be", "Хеб"), ("bg", "Хеб"), ("ca", "Districte de Cheb"), ("ccp", "𑄌𑄬𑄛\u{11134}"), ("ceb", "Okres Cheb"), ("cs", "okres Cheb"), ("de", "Okres Cheb"), ("en", "Cheb"), ("es", "Distrito de Cheb"), ("eu", "Chebeko barrutia"), ("fa", "شهرستان خب"), ("fr", "District de Cheb"), ("he", "מחוז משנה חב"), ("hu", "Chebi járás"), ("hy", "Նախոդի շրջան"), ("it", "Distretto di Cheb"), ("ka", "ხების რაიონი"), ("lt", "Chebo rajonas"), ("mk", "Хеб"), ("ms", "Daerah Cheb"), ("nl", "Okres Cheb"), ("pl", "Powiat Cheb"), ("pt", "Cheb"), ("ro", "Okres Cheb"), ("ru", "Хеб"), ("sk", "Okres Cheb"), ("sr", "Округ Хеб"), ("sr_Latn", "Okrug Heb"), ("sv", "Cheb"), ("tr", "Cheb ilçesi"), ("uk", "округ Хеб"), ("ur", "خیب ضلع"), ("vi", "Cheb"), ("zh", "海布縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "412",
                    Subdivision{
                        name: "412",
                        country_alpha2: Alpha2::CZ,
                        code: "412",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كارلوفي فاري"), ("be", "Раён Карлавы Вары"), ("bg", "Карлови Вари"), ("ca", "Districte de Karlovy Vary"), ("ccp", "𑄇𑄢\u{11134}𑄣\u{11127}𑄞\u{11128} 𑄞𑄢\u{11129}"), ("ceb", "Okres Karlovy Vary"), ("cs", "okres Karlovy Vary"), ("de", "Okres Karlovy Vary"), ("en", "Karlovy Vary"), ("es", "Distrito de Karlovy Vary"), ("eu", "Karlovy Varyko barrutia"), ("fa", "ناحیه کارلووی واری"), ("fr", "District de Karlovy Vary"), ("he", "מחוז משנה קרלובי וארי"), ("hu", "Karlovy Vary-i járás"), ("hy", "Ռիխնով-նադ-Կնեժնոուի շրջան"), ("it", "Distretto di Karlovy Vary"), ("ka", "კარლოვი-ვარის რაიონი"), ("lt", "Karlovi Varų rajonas"), ("ms", "Daerah Karlovy Vary²"), ("nl", "Okres Karlsbad"), ("pl", "Powiat Karlowe Wary"), ("pt", "Karlovy Vary²"), ("ru", "Карловы Вары"), ("sk", "Okres Karlove Vary"), ("sr", "Округ Карлове Вари"), ("sr_Latn", "Okrug Karlove Vari"), ("sv", "Karlovy Vary²"), ("tr", "Karlovy Vary ilçesi"), ("uk", "Округ Карлові Вари"), ("ur", "کارلووی واری ضلع"), ("vi", "Karlovy Vary²"), ("zh", "卡羅維發利縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "413",
                    Subdivision{
                        name: "413",
                        country_alpha2: Alpha2::CZ,
                        code: "413",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سوكولوف"), ("bg", "Соколов"), ("ca", "Districte de Sokolov"), ("ccp", "𑄥\u{11127}𑄇\u{1112e}𑄣\u{11127}𑄛\u{11134}"), ("ceb", "Okres Sokolov"), ("cs", "okres Sokolov"), ("de", "Okres Sokolov"), ("en", "Sokolov"), ("es", "Distrito de Sokolov"), ("eu", "Sokoloveko barrutia"), ("fa", "شهرستان سوکولوف"), ("fr", "District de Sokolov"), ("he", "מחוז משנה סוקולוב"), ("hu", "Sokolovi járás"), ("hy", "Տրուտնովի շրջան"), ("it", "Distretto di Sokolov"), ("ja", "ソコロフ郡"), ("ka", "სოკოლოვის რაიონი"), ("lt", "Sokolovo rajonas"), ("ms", "Daerah Sokolov"), ("nl", "Okres Sokolov"), ("pl", "Powiat Sokolov"), ("pt", "Sokolov"), ("ru", "Соколов"), ("sk", "Okres Sokolov"), ("sr", "Округ Соколов"), ("sr_Latn", "Okrug Sokolov"), ("sv", "Sokolov"), ("tr", "Sokolov ilçesi"), ("uk", "Соколов"), ("ur", "سوکولوف ضلع"), ("vi", "Sokolov"), ("zh", "索洛洛夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::CZ,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.6119037), longitude: Some(13.7870086), max_latitude: Some(51.0557757), min_latitude: Some(50.0774318), max_longitude: Some(14.6528592), min_longitude: Some(12.940246)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أوستي ناد لابم"), ("az", "Ustetskiy diyarı"), ("be", "Усцецкі край"), ("bg", "Устецки край"), ("bn", "উস\u{9cd}টি ন\u{9be}ড ল\u{9be}বেম অঞ\u{9cd}চল"), ("bs", "Ustečki kraj"), ("ca", "Regió d’Ústí nad Labem"), ("ccp", "𑄅\u{1112a}𑄌\u{11134}𑄑𑄬𑄇\u{11129}"), ("ceb", "Ústecký kraj"), ("cs", "Ústecký kraj"), ("da", "Ústí nad Labem"), ("de", "Ústecký kraj"), ("el", "Περιφέρεια Ούστι ναντ Λάμπεμ"), ("en", "Ústecký"), ("es", "Región de Ústí nad Labem"), ("et", "Ústí maakond"), ("eu", "Ústí nad Labem eskualdea"), ("fa", "منطقه اوستی ناد لابم"), ("fi", "Ústín lääni"), ("fr", "Région d’Aussig-sur-Elbe"), ("gu", "ઉસ\u{acd}તી નાદ લાબ\u{ac7}મ પ\u{acd}રદ\u{ac7}શ"), ("he", "אוסטי נד לבם (מחוז)"), ("hi", "उस\u{94d}ती नाड लाब\u{947}म क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Ústečki kraj"), ("hu", "Ústí nad Labem-i kerület"), ("id", "Daerah Ústí nad Labem"), ("it", "regione di Ústí nad Labem"), ("ja", "ウースチー州"), ("ka", "უსტეცის მხარე"), ("kn", "ಯುಸ\u{ccd}ಟ\u{cbf} ನಾಡ\u{ccd} ಲ\u{ccd}ಯಾಬ\u{cc6}ಮ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "우스티 주"), ("lt", "Ūsčio kraštas"), ("lv", "Ūstu pie Labas apriņķis"), ("mk", "Устечки крај"), ("mr", "उस\u{94d}ति नाद लाब\u{947}म प\u{94d}रद\u{947}श"), ("ms", "Daerah Ústí nad Labem"), ("nb", "Ústí nad Labem region"), ("nl", "Ústí nad Labem"), ("no", "Ústí nad Labem region"), ("pl", "Kraj ustecki"), ("pt", "Ústí nad Labem"), ("ro", "Regiunea Ústí nad Labem"), ("ru", "Устецкий край"), ("si", "උස\u{dca}ට\u{dd2} නඩ\u{dca} ලබෙම\u{dca} කල\u{dcf}පය"), ("sk", "Úsťanský kraj"), ("sr", "Устечки крај"), ("sr_Latn", "Ustečki kraj"), ("sv", "Ústí nad Labem"), ("ta", "ஸ\u{bcd}டி ந\u{bbe}ட\u{bcd} ல\u{bbe}பென\u{bcd} பகுதி"), ("te", "ఉస\u{c4d}ట\u{c3f} న\u{c3e}డ\u{c4d} ల\u{c3e}బ\u{c46}మ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แถบอ\u{e31}สต\u{e34} นาดลาเบม"), ("tr", "Ústí nad Labem"), ("uk", "Устецький край"), ("ur", "اوستی ناد لاہم علاقہ"), ("vi", "Ústí nad Labem"), ("zh", "烏斯季州")]),
                        unofficial_name_list: ["Ústecký kraj"].to_vec(),
                    }
                ),
                (
                    "421",
                    Subdivision{
                        name: "421",
                        country_alpha2: Alpha2::CZ,
                        code: "421",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ديتشين"), ("be", "Дзечын"), ("bg", "Дечин"), ("ca", "Districte de Děčín"), ("ccp", "𑄓𑄬𑄥\u{11128}𑄚\u{11134}"), ("ceb", "Okres Děčín"), ("cs", "okres Děčín"), ("de", "Okres Děčín"), ("en", "Děčín"), ("es", "Distrito de Děčín"), ("eu", "Děčíngo barrutia"), ("fa", "شهرستان دیه\u{200c}چین"), ("fr", "District de Děčín"), ("he", "מחוז משנה דצ׳ין"), ("hu", "Děčíni járás"), ("hy", "Դեչինի շրջան"), ("it", "Distretto di Děčín"), ("ka", "დეჩინის რაიონი"), ("lt", "Dečyno rajonas"), ("ms", "Daerah Děčín"), ("nl", "Okres Děčín"), ("pl", "Powiat Děčín"), ("pt", "Děčín"), ("ru", "Дечин"), ("sk", "okres Děčín"), ("sr", "Округ Дјечин"), ("sr_Latn", "Okrug Dječin"), ("sv", "Okres Děčín"), ("tr", "Děčín ilçesi"), ("uk", "Дечин"), ("ur", "دیچین ضلع"), ("vi", "Děčín"), ("zh", "傑欽縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "422",
                    Subdivision{
                        name: "422",
                        country_alpha2: Alpha2::CZ,
                        code: "422",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خوموتوف"), ("bg", "Хомутов"), ("ca", "Districte de Chomutov"), ("ccp", "𑄇\u{1112e}𑄟\u{1112a}𑄑\u{11127}𑄛\u{11134}"), ("ceb", "Okres Chomutov"), ("cs", "okres Chomutov"), ("de", "Okres Chomutov"), ("en", "Chomutov"), ("es", "Distrito de Chomutov"), ("eu", "Chomutoveko barrutia"), ("fa", "شهرستان خوموتوف"), ("fr", "District de Chomutov"), ("he", "מחוז משנה חומוטוב"), ("hu", "Chomutovi járás"), ("hy", "Խոմուտովի շրջան"), ("it", "Distretto di Chomutov"), ("ka", "ხომუტოვის რაიონი"), ("lt", "Chomutovo rajonas"), ("ms", "Daerah Chomutov"), ("nl", "Okres Chomutov"), ("pl", "Powiat Chomutov"), ("pt", "Chomutov"), ("ru", "Хомутов"), ("sk", "Okres Chomutov"), ("sr", "Округ Хомутов"), ("sr_Latn", "Okrug Homutov"), ("sv", "Okres Chomutov"), ("tr", "Chomutov ilçesi"), ("uk", "Хомутов (округ)"), ("ur", "خومتو ضلع"), ("vi", "Chomutov"), ("zh", "霍穆托夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "423",
                    Subdivision{
                        name: "423",
                        country_alpha2: Alpha2::CZ,
                        code: "423",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليتوميريتسه"), ("be", "Літамержыцэ"), ("bg", "Литомержице"), ("ca", "Districte de Litoměřice"), ("ccp", "𑄣\u{11128}𑄑\u{1112e}𑄟𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Okres Litoměřice"), ("cs", "okres Litoměřice"), ("de", "Okres Litoměřice"), ("en", "Litoměřice"), ("es", "Distrito de Litoměřice"), ("eu", "Litoměřiceko barrutia"), ("fa", "شهرستان لیتومیه\u{200c}رژیتسه"), ("fr", "District de Litoměřice"), ("he", "מחוז משנה ליטומריצה"), ("hu", "Litoměřicei járás"), ("hy", "Լիտոմերիցեի շրջան"), ("it", "Distretto di Litoměřice"), ("ka", "ლიტომერჟიცეს რაიონი"), ("lt", "Litomeržicių rajonas"), ("ms", "Daerah Litoměřice"), ("nl", "Okres Litoměřice"), ("pl", "Powiat Litomierzyce"), ("pt", "Litoměřice"), ("ru", "Литомержице"), ("sk", "Okres Litoměřice"), ("sr", "Округ Литомјержице"), ("sr_Latn", "Okrug Litomjeržice"), ("sv", "Okres Litoměřice"), ("tr", "Litoměřice ilçesi"), ("uk", "Літомержице"), ("ur", "لیتومیرشسے ضلع"), ("vi", "Litoměřice"), ("zh", "利托梅日采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "424",
                    Subdivision{
                        name: "424",
                        country_alpha2: Alpha2::CZ,
                        code: "424",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوني"), ("be", "раён Лоўні"), ("bg", "Лоуни"), ("ca", "Districte de Louny"), ("ccp", "𑄣𑄅\u{1112a}𑄚\u{11128}"), ("ceb", "Okres Louny"), ("cs", "okres Louny"), ("de", "Okres Louny"), ("en", "Louny"), ("es", "Distrito de Louny"), ("eu", "Lounyko barrutia"), ("fa", "شهرستان لوونی"), ("fr", "District de Louny"), ("he", "מחוז משנה לואוני"), ("hu", "Louny-i járás"), ("hy", "Լոունի շրջան"), ("it", "Distretto di Louny"), ("ka", "ლოუნის რაიონი"), ("lt", "Lounų rajonas"), ("ms", "Daerah Louny"), ("nl", "Okres Louny"), ("pl", "Powiat Louny"), ("pt", "Louny"), ("ru", "Лоуни"), ("sk", "Okres Louny"), ("sr", "Округ Лоуни"), ("sr_Latn", "Okrug Louni"), ("sv", "Okres Louny"), ("tr", "Louny ilçesi"), ("uk", "Лоуни"), ("ur", "لونی ضلع"), ("vi", "Louny"), ("zh", "洛烏尼縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "425",
                    Subdivision{
                        name: "425",
                        country_alpha2: Alpha2::CZ,
                        code: "425",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة موست"), ("bg", "Мост"), ("ca", "Districte de Most"), ("ccp", "𑄟\u{1112e}𑄥\u{11133}𑄑\u{11134}"), ("ceb", "Okres Most"), ("cs", "okres Most"), ("de", "Okres Most"), ("en", "Most"), ("es", "Distrito de Most"), ("fa", "شهرستان موست"), ("fr", "District de Most"), ("he", "מחוז משנה מוסט"), ("hu", "Mosti járás"), ("hy", "Մոստի շրջան"), ("it", "Distretto di Most"), ("ka", "მოსტის რაიონი"), ("lt", "Mosto rajonas"), ("ms", "Daerah Most"), ("nl", "Okres Most"), ("pl", "Powiat Most"), ("pt", "Most"), ("ru", "Мост"), ("sk", "Okres Most"), ("sr", "Округ Мост"), ("sr_Latn", "Okrug Most"), ("sv", "Okres Most"), ("uk", "Мост"), ("ur", "موست ضلع"), ("vi", "Most"), ("zh", "莫斯特縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "426",
                    Subdivision{
                        name: "426",
                        country_alpha2: Alpha2::CZ,
                        code: "426",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تبليتسه"), ("be", "Цепліцы"), ("bg", "Теплице"), ("ca", "Districte de Teplice"), ("ccp", "𑄑𑄬𑄛\u{11134}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Okres Teplice"), ("cs", "okres Teplice"), ("de", "Okres Teplice"), ("en", "Teplice"), ("es", "Distrito de Teplice"), ("fa", "شهرستان تپلیتسه"), ("fr", "District de Teplice"), ("he", "מחוז משנה טפליצה"), ("hu", "Teplicei járás"), ("hy", "Տեպլիցեի շրջան"), ("it", "Distretto di Teplice"), ("ka", "ტეპლიცეს რაიონი"), ("lt", "Teplicės rajonas"), ("ms", "Daerah Teplice"), ("nl", "Okres Teplice"), ("pl", "Powiat Cieplice"), ("pt", "Teplice"), ("ru", "Теплице"), ("sk", "Okres Teplice"), ("sr", "Округ Теплице"), ("sr_Latn", "Okrug Teplice"), ("sv", "Okres Teplice"), ("tr", "Teplice ilçesi"), ("uk", "Тепліце"), ("ur", "تپلیتسے ضلع"), ("vi", "Teplice"), ("zh", "特普利采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "427",
                    Subdivision{
                        name: "427",
                        country_alpha2: Alpha2::CZ,
                        code: "427",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوستي ناد لابم"), ("be", "Усці-над-Лабем"), ("bg", "Усти над Лабе"), ("ca", "Districte d’Ústí nad Labem"), ("ccp", "𑄅\u{1112a}𑄌\u{11134}𑄑\u{11128} 𑄚𑄖\u{11134} 𑄣𑄝𑄬𑄟\u{11134}"), ("ceb", "Okres Ústí nad Labem"), ("cs", "okres Ústí nad Labem"), ("de", "Okres Ústí nad Labem"), ("en", "Ústí nad Labem"), ("es", "Distrito de Ústí nad Labem"), ("fr", "District d’Ústí nad Labem"), ("he", "מחוז משנה אוסטי נד לבם"), ("hu", "Ústí nad Labem-i járás"), ("hy", "Ուստի-նադ-Լաբեմի շրջան"), ("it", "Distretto di Ústí nad Labem"), ("ka", "უსტი-ნად-ლაბემის რაიონი"), ("lt", "Ūsčio prie Labės rajonas"), ("ms", "Daerah Ústí nad Labem²"), ("nl", "Okres Ústí nad Labem"), ("pl", "Powiat Uście nad Łabą"), ("pt", "Ústí nad Labem²"), ("ru", "Усти-над-Лабем"), ("sk", "Okres Ústí nad Labem"), ("sr", "Округ Усти на Лаби"), ("sr_Latn", "Okrug Usti na Labi"), ("sv", "Ústí nad Labem²"), ("uk", "Усті-над-Лабем"), ("ur", "اوستی ناد لاہم ضلع"), ("vi", "Ústí nad Labem²"), ("zh", "拉貝河畔烏斯季縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "51",
                    Subdivision{
                        name: "51",
                        country_alpha2: Alpha2::CZ,
                        code: "51",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.65942399999999), longitude: Some(14.7632424), max_latitude: Some(51.0231974), min_latitude: Some(50.4720971), max_longitude: Some(15.6330293), min_longitude: Some(14.3435357)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ليبيريتس"), ("az", "Liberets diyarı"), ("be", "Ліберацкі край"), ("bg", "Либерецки край"), ("bn", "ল\u{9be}ইবেরেক অঞ\u{9cd}চল"), ("bs", "Liberecki kraj"), ("ca", "Regió de Liberec"), ("ccp", "𑄣\u{1112d}𑄝𑄬𑄢𑄬𑄇\u{11128}"), ("ceb", "Liberecký kraj"), ("cs", "Liberecký kraj"), ("da", "Liberec"), ("de", "Liberecký kraj"), ("el", "Περιφέρεια Λίμπερετς"), ("en", "Liberecký"), ("es", "Región de Liberec"), ("et", "Libereci maakond"), ("eu", "Liberec eskualdea"), ("fa", "منطقه لیبرتس"), ("fi", "Liberecin lääni"), ("fr", "Région de Liberec"), ("gu", "લિબ\u{acd}ર\u{ac7}ક પ\u{acd}રદ\u{ac7}શ"), ("he", "ליברץ (מחוז)"), ("hi", "लिब\u{947}र\u{947}क प\u{94d}रद\u{947}श"), ("hr", "Liberečki kraj"), ("hu", "Libereci kerület"), ("id", "Daerah Liberec"), ("it", "regione di Liberec"), ("ja", "リベレツ州"), ("ka", "ლიბერეცის მხარე"), ("kn", "ಲ\u{cbf}ಬರ\u{cc6}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "리베레츠 주"), ("lt", "Libereco kraštas"), ("lv", "Liberecas apriņķis"), ("mk", "Либеречки крај"), ("mr", "लिबर\u{947}क प\u{94d}रद\u{947}श"), ("ms", "Daerah Liberec"), ("nb", "Liberec"), ("nl", "Liberec"), ("no", "Liberec"), ("pl", "Kraj liberecki"), ("pt", "Liberec"), ("ro", "Regiunea Liberec"), ("ru", "Либерецкий край"), ("si", "ල\u{dd2}බෙරෙක\u{dca} කල\u{dcf}පය"), ("sk", "Liberecký kraj"), ("sr", "Либеречки крај"), ("sr_Latn", "Liberečki kraj"), ("sv", "Liberec"), ("ta", "லிபெர\u{bcd}க\u{bcd} பகுதி"), ("te", "ల\u{c3f}బర\u{c46}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ล\u{e34}เบเรช"), ("tr", "Liberec Bölgesi"), ("uk", "Ліберецький край"), ("ur", "لیبرتس علاقہ"), ("vi", "Liberec"), ("zh", "利贝雷茨州")]),
                        unofficial_name_list: ["Liberecký kraj"].to_vec(),
                    }
                ),
                (
                    "511",
                    Subdivision{
                        name: "511",
                        country_alpha2: Alpha2::CZ,
                        code: "511",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيسكا ليبا"), ("bg", "Ческа Липа"), ("ca", "Districte de Česká Lípa"), ("ccp", "𑄇𑄬𑄌\u{11134}𑄇 𑄣\u{11128}𑄛"), ("ceb", "Okres Česká Lípa"), ("cs", "okres Česká Lípa"), ("de", "Okres Česká Lípa"), ("en", "Česká Lípa"), ("es", "Distrito de Česká Lípa"), ("fa", "شهرستان چسکا لیپا"), ("fr", "District de Česká Lípa"), ("he", "מחוז משנה צסקה ליפה"), ("hu", "Česká Lípa-i járás"), ("hy", "Չեսկա Լիպայի շրջան"), ("it", "Distretto di Česká Lípa"), ("ka", "ჩესკა-ლიპის რაიონი"), ("lt", "Česka Lypos rajonas"), ("ms", "Daerah Česká Lípa"), ("nl", "Okres Česká Lípa"), ("pl", "Powiat Czeska Lipa"), ("ro", "districtul Česká Lípa"), ("ru", "Ческа-Липа"), ("sk", "Okres Česká Lípa"), ("sr", "Округ Чешка Липа"), ("sr_Latn", "Okrug Češka Lipa"), ("sv", "Česká Lípa"), ("tr", "Česká Lípa ilçesi"), ("uk", "Чеська Липа"), ("ur", "چیسکا لیپا ضلع"), ("vi", "Česká Lípa"), ("zh", "捷克利帕縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "512",
                    Subdivision{
                        name: "512",
                        country_alpha2: Alpha2::CZ,
                        code: "512",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يابلونتس ناد نيسو"), ("bg", "Яблонец над Ниса"), ("ca", "Districte de Jablonec nad Nisou"), ("ccp", "𑄎𑄛\u{11134}𑄣\u{1112e}𑄚𑄬𑄇\u{11134} 𑄚𑄖\u{11134} 𑄚\u{11128}𑄥𑄅\u{1112a}"), ("ceb", "Okres Jablonec nad Nisou"), ("cs", "okres Jablonec nad Nisou"), ("de", "Okres Jablonec nad Nisou"), ("en", "Jablonec nad Nisou"), ("es", "Distrito de Jablonec nad Nisou"), ("eu", "Jablonec nad Nisouko barrutia"), ("fa", "شهرستان یابلونتس ناد نیسوو"), ("fr", "District de Jablonec nad Nisou"), ("he", "מחוז משנה יבלונץ נד ניסו"), ("hu", "Jablonec nad Nisou-i járás"), ("hy", "Յաբլոնեց-նադ-Նիսոուի շրջան"), ("it", "Distretto di Jablonec nad Nisou"), ("ka", "იაბლონეც-ნად-ნისოუს რაიონი"), ("lt", "Jabloneco prie Nisos rajonas"), ("ms", "Daerah Jablonec nad Nisou"), ("nl", "Okres Jablonec nad Nisou"), ("pl", "Powiat Jablonec nad Nysą"), ("pt", "Jablonec nad Nisou"), ("ru", "Яблонец-над-Нисоу"), ("sk", "Okres Jablonec nad Nisou"), ("sr", "Округ Јаблонец на Ниси"), ("sr_Latn", "Okrug Jablonec na Nisi"), ("sv", "Jablonec nad Nisou"), ("tr", "Jablonec nad Nisou ilçesi"), ("uk", "Яблонець-над-Нисою"), ("ur", "جبلونیک ناد نسو ضلع"), ("vi", "Jablonec nad Nisou"), ("zh", "尼斯河畔亞布洛內茨縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "513",
                    Subdivision{
                        name: "513",
                        country_alpha2: Alpha2::CZ,
                        code: "513",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليبيريتس"), ("bg", "Либерец"), ("ca", "Districte de Liberec"), ("ccp", "𑄣\u{11128}𑄝𑄢𑄬𑄇\u{11134}"), ("ceb", "Okres Liberec"), ("cs", "okres Liberec"), ("de", "Okres Liberec"), ("en", "Liberec"), ("es", "Distrito de Liberec"), ("fa", "شهرستان لیبرتس"), ("fr", "District de Liberec"), ("he", "מחוז משנה ליברץ"), ("hu", "Libereci járás"), ("hy", "Լիբերեցի շրջան"), ("it", "Distretto di Liberec"), ("ka", "ლიბერეცის რაიონი"), ("lt", "Libereco rajonas"), ("ms", "Daerah Liberec²"), ("nl", "Okres Liberec"), ("pl", "Powiat Liberec"), ("pt", "Liberec²"), ("ro", "Liberec"), ("ru", "Либерец"), ("sk", "Okres Liberec"), ("sr", "Округ Либерец"), ("sr_Latn", "Okrug Liberec"), ("sv", "Liberec²"), ("tr", "Liberec ilçesi"), ("uk", "округ Ліберець"), ("ur", "لیبرتس ضلع"), ("vi", "Liberec²"), ("zh", "利貝雷茨縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "514",
                    Subdivision{
                        name: "514",
                        country_alpha2: Alpha2::CZ,
                        code: "514",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيميلي"), ("be", "раён Семілі"), ("bg", "Семили"), ("ca", "Districte de Semily"), ("ccp", "𑄥𑄬𑄟\u{11128}𑄣\u{11128}"), ("ceb", "Okres Semily"), ("cs", "okres Semily"), ("de", "Okres Semily"), ("en", "Semily"), ("es", "Distrito de Semily"), ("eu", "Semilyko barrutia"), ("fa", "شهرستان سمیلی"), ("fr", "District de Semily"), ("he", "מחוז משנה סמילי"), ("hu", "Semily-i járás"), ("hy", "Սեմիլիի շրջան"), ("it", "Distretto di Semily"), ("ka", "სემილის რაიონი"), ("lt", "Semilų rajonas"), ("ms", "Daerah Semily"), ("nb", "Semily"), ("nl", "Okres Semily"), ("no", "Semily"), ("pl", "Powiat Semily"), ("pt", "Semily"), ("ru", "Семили"), ("sk", "Okres Semily"), ("sr", "Округ Семили"), ("sr_Latn", "Okrug Semili"), ("sv", "Semily"), ("tr", "Semily ilçesi"), ("uk", "Семіли"), ("vi", "Semily"), ("zh", "塞米利縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "52",
                    Subdivision{
                        name: "52",
                        country_alpha2: Alpha2::CZ,
                        code: "52",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.3512484), longitude: Some(15.7976459), max_latitude: Some(50.7804396), min_latitude: Some(50.0388059), max_longitude: Some(16.5855247), min_longitude: Some(15.1051615)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم هرادتس كرالوفه"), ("az", "Kraloveqradetskiy diyarı"), ("be", "Кралавеградзецкі край"), ("bg", "Краловохрадецки край"), ("bn", "হ\u{9be}রডেক ক\u{9cd}র\u{9be}লোভ অঞ\u{9cd}চল"), ("bs", "Kralovehradecki kraj"), ("ca", "Regió de Hradec Králové"), ("ccp", "𑄇\u{11133}𑄢𑄣\u{1112e}𑄞𑄬𑄦\u{11134}𑄢𑄓𑄬𑄇\u{11128}"), ("ceb", "Královéhradecký kraj"), ("cs", "Královéhradecký kraj"), ("da", "Hradec Králové"), ("de", "Královéhradecký kraj"), ("el", "Περιφέρεια Χράντετς Κράλοβε"), ("en", "Královéhradecký"), ("es", "Región de Hradec Králové"), ("et", "Hradec Králové maakond"), ("eu", "Hradec Králové eskualdea"), ("fa", "منطقه هرادتس کرالووه"), ("fi", "Hradec Královén lääni"), ("fr", "Région de Hradec Králové"), ("gu", "હ\u{acd}રાડ\u{ac7}ક ક\u{acd}ર\u{ac7}લોવ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("he", "הראדץ קראלובה (מחוז)"), ("hi", "ह\u{94d}रद\u{947}क क\u{94d}रालोव प\u{94d}रद\u{947}श"), ("hr", "Královéhradečki kraj"), ("hu", "Hradec Králové-i kerület"), ("id", "Daerah Hradec Kralove"), ("it", "regione di Hradec Králové"), ("ja", "フラデツ・クラーロヴェー州"), ("ka", "ჰრადეც-კრალოვეს მხარე"), ("kn", "ಹಾರ\u{ccd}ಡ\u{cc6}ಕ\u{ccd} ಕ\u{ccd}ರಾಲೋವ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "흐라데츠크랄로베 주"), ("lt", "Hradec Kralovės kraštas"), ("lv", "Hradeckrāloves apgabals"), ("mk", "Краловохрадечки крај"), ("mr", "ह\u{94d}राड\u{947}क क\u{94d}रॉल\u{947}व\u{94d}ह प\u{94d}रद\u{947}श"), ("ms", "Daerah Hradec Kralove"), ("nb", "Plzeň region"), ("nl", "Hradec Králové"), ("no", "Hradec Králové"), ("pl", "Kraj hradecki"), ("pt", "Hradec Králové"), ("ro", "Regiunea Hradec Králové"), ("ru", "Краловеградецкий край"), ("si", "හ\u{dca}රඩෙක\u{dca} ක\u{dca}ර\u{dcf}ලොවේ කල\u{dcf}පය"), ("sk", "Královohradecký kraj"), ("sr", "Краловехрадечки крај"), ("sr_Latn", "Kralovehradečki kraj"), ("sv", "Hradec Králové"), ("ta", "ஹர\u{bbe}டெக\u{bcd} க\u{bcd}ரலோவே பகுதி"), ("te", "ర\u{c3e}డ\u{c46}క\u{c4d} క\u{c4d}ర\u{c3e}ల\u{c4b}వ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตฮราเด\u{e47}ค คราลอเว"), ("tr", "Hradec Králové ili"), ("uk", "Краловоградецький край"), ("ur", "ہاردک کارلوف علاقہ"), ("vi", "Hradec Králové"), ("zh", "赫拉德茨-克拉洛韋州")]),
                        unofficial_name_list: ["Královéhradecký kraj"].to_vec(),
                    }
                ),
                (
                    "521",
                    Subdivision{
                        name: "521",
                        country_alpha2: Alpha2::CZ,
                        code: "521",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هرادتس كرالوفه"), ("be", "раён Градзец-Кралавэ"), ("bg", "Храдец Кралове"), ("ca", "Districte de Hradec Králové"), ("ccp", "𑄦\u{11133}𑄢𑄓𑄬𑄇\u{11134} 𑄇\u{11133}𑄢𑄣\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Hradec Králové"), ("cs", "okres Hradec Králové"), ("de", "Okres Hradec Králové"), ("en", "Hradec Králové"), ("es", "Distrito de Hradec Králové"), ("eu", "Hradec Královéko barrutia"), ("fa", "شهرستان هرادتس کرالووه"), ("fr", "District de Hradec Králové"), ("he", "מחוז משנה הראדץ קראלובה"), ("hu", "Hradec Králové-i járás"), ("hy", "Արևմտապրահյան շրջան"), ("it", "Distretto di Hradec Králové"), ("ja", "フラデツ・クラーロヴェー郡"), ("ka", "ჰრადეც-კრალოვეს რაიონი"), ("lt", "Hradec Kralovės rajonas"), ("ms", "Daerah Hradec Králové"), ("nl", "Okres Hradec Králové"), ("pl", "Powiat Hradec Králové"), ("pt", "Hradec Králové²"), ("ru", "Градец-Кралове"), ("sk", "Okres Hradec Králové"), ("sr", "Округ Храдец Кралове"), ("sr_Latn", "Okrug Hradec Kralove"), ("sv", "Hradec Králové²"), ("tr", "Hradec Králové ilçesi"), ("uk", "Градець-Кралове"), ("ur", "ہاردک کارلوف ضلع"), ("vi", "Hradec Králové²"), ("zh", "赫拉德茨克拉洛韋縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "522",
                    Subdivision{
                        name: "522",
                        country_alpha2: Alpha2::CZ,
                        code: "522",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يتشين"), ("bg", "Ичин"), ("ca", "Districte de Jičín"), ("ccp", "𑄎\u{11128}𑄌\u{11128}𑄚\u{11134}"), ("ceb", "Okres Jičín"), ("cs", "okres Jičín"), ("de", "Okres Jičín"), ("en", "Jičín"), ("es", "Distrito de Jičín"), ("eu", "Jičíngo barrutia"), ("fa", "شهرستان ییچین"), ("fr", "District de Jičín"), ("he", "מחוז משנה ייצין"), ("hu", "Jičíni járás"), ("hy", "Պրիբրամի շրջան"), ("it", "Distretto di Jičín"), ("ka", "იიჩინის რაიონი"), ("lt", "Jičyno rajonas"), ("ms", "Daerah Jičín"), ("nl", "Okres Jičín"), ("pl", "Powiat Jiczyn"), ("pt", "Jičín"), ("ru", "Йичин"), ("sk", "Okres Jičín"), ("sr", "Округ Јичин"), ("sr_Latn", "Okrug Jičin"), ("sv", "Jičín"), ("tr", "Jičín ilçesi"), ("uk", "Їчін"), ("ur", "جیچن ضلع"), ("vi", "Jičín"), ("zh", "伊欽縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "523",
                    Subdivision{
                        name: "523",
                        country_alpha2: Alpha2::CZ,
                        code: "523",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ناخود"), ("bg", "Наход"), ("ca", "Districte de Náchod"), ("ccp", "𑄚𑄌\u{1112e}𑄖\u{11134}"), ("ceb", "Okres Náchod"), ("cs", "okres Náchod"), ("da", "Náchod"), ("de", "Okres Náchod"), ("en", "Náchod"), ("es", "Distrito de Náchod"), ("eu", "Náchodeko barrutia"), ("fa", "شهرستان ناخود"), ("fr", "District de Náchod"), ("he", "מחוז משנה נאחוד"), ("hu", "Náchodi járás"), ("hy", "Ռակովնիկի շրջան"), ("it", "Distretto di Náchod"), ("ka", "ნახოდის რაიონი"), ("lt", "Nachodo rajonas"), ("ms", "Daerah Náchod"), ("nl", "Okres Náchod"), ("pl", "Powiat Náchod"), ("pt", "Náchod"), ("ru", "Наход"), ("sk", "Okres Náchod"), ("sr", "Округ Наход"), ("sr_Latn", "Okrug Nahod"), ("sv", "Náchod"), ("tr", "Náchod ilçesi"), ("uk", "Наход"), ("ur", "ناخود ضلع"), ("vi", "Náchod"), ("zh", "納霍德縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "524",
                    Subdivision{
                        name: "524",
                        country_alpha2: Alpha2::CZ,
                        code: "524",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريخنوف ناد كنيجنو"), ("be", "раён Рыхнаў над Кнежнау"), ("bg", "Рихнов над Кнежноу"), ("ca", "Districte de Rychnov nad Kněžnou"), ("ccp", "𑄢\u{1112d}𑄇\u{11133}𑄚\u{11127}𑄛\u{11134} 𑄚𑄖\u{11134} 𑄇\u{11133}𑄚𑄬𑄌\u{11134}𑄚𑄅\u{1112a}"), ("ceb", "Okres Rychnov nad Kněžnou"), ("cs", "okres Rychnov nad Kněžnou"), ("de", "Okres Rychnov nad Kněžnou"), ("en", "Rychnov nad Kněžnou"), ("es", "Distrito de Rychnov nad Kněžnou"), ("eu", "Rychnov nad Kněžnouko barrutia"), ("fa", "شهرستان ریخنوف ناد کنیژنوو"), ("fr", "District de Rychnov nad Kněžnou"), ("he", "מחוז משנה ריחנוב נד קניז׳נואו"), ("hu", "Rychnov nad Kněžnou-i járás"), ("hy", "Հրադեց Կրալովեի շրջան"), ("it", "Distretto di Rychnov nad Kněžnou"), ("ja", "リフノフ・ナド・クニェジュノウ郡"), ("ka", "რიხნოვი-ნად-კნეჟნოუს რაიონი"), ("lt", "Richnovo prie Knežnos rajonas"), ("ms", "Daerah Rychnov nad Kněžnou"), ("nl", "Okres Rychnov nad Kněžnou"), ("pl", "Powiat Rychnov nad Kněžnou"), ("ru", "Рихнов-над-Кнежноу"), ("sk", "Okres Rychnov nad Kněžnou"), ("sr", "Округ Рихнов на Књежној"), ("sr_Latn", "Okrug Rihnov na Knježnoj"), ("sv", "Rychnov nad Kněžnou"), ("tr", "Rychnov nad Kněžnou ilçesi"), ("uk", "Рихнов-над-Кнєжной"), ("ur", "ریخنوف ناد کنیزشنو ضلع"), ("vi", "Rychnov nad Kněžnou"), ("zh", "科內日諾河畔里赫諾夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "525",
                    Subdivision{
                        name: "525",
                        country_alpha2: Alpha2::CZ,
                        code: "525",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ترتنوف"), ("be", "раён Трутнаў"), ("bg", "Трутнов"), ("ca", "Districte de Trutnov"), ("ccp", "𑄑\u{11133}𑄢\u{1112a}𑄑𑄚\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Trutnov"), ("cs", "okres Trutnov"), ("de", "Okres Trutnov"), ("en", "Trutnov"), ("es", "Distrito de Trutnov"), ("eu", "Trutnoveko barrutia"), ("fa", "منطقه تروتنوف"), ("fr", "District de Trutnov"), ("he", "מחוז משנה טרונטוב"), ("hu", "Trutnovi járás"), ("hy", "Յիչինի շրջան"), ("it", "Distretto di Trutnov"), ("ka", "ტრუტნოვის რაიონი"), ("lt", "Trutnovo rajonas"), ("ms", "Daerah Trutnov"), ("nb", "Trutnov"), ("nl", "Okres Trutnov"), ("no", "Trutnov"), ("pl", "Powiat Trutnov"), ("pt", "Trutnov"), ("ro", "Okres Trutnov"), ("ru", "Трутнов"), ("sk", "Okres Trutnov"), ("sr", "Округ Трутнов"), ("sr_Latn", "Okrug Trutnov"), ("sv", "Trutnov"), ("tr", "Trutnov İlçesi"), ("uk", "Трутнов"), ("ur", "ترنتوو ضلع"), ("vi", "Trutnov"), ("zh", "特魯特諾夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "53",
                    Subdivision{
                        name: "53",
                        country_alpha2: Alpha2::CZ,
                        code: "53",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.9444479), longitude: Some(16.2856916), max_latitude: Some(50.20763729999999), min_latitude: Some(49.5720141), max_longitude: Some(16.8665452), min_longitude: Some(15.3630765)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم باردوبيتسه"), ("az", "Pardubitskiy diyarı"), ("be", "Пардубіцкі край"), ("bg", "Пардубицки край"), ("bn", "প\u{9be}রডোবিচে অঞ\u{9cd}চল"), ("bs", "Pardubički kraj"), ("ca", "Regió de Pardubice"), ("ccp", "𑄛𑄢\u{11134}𑄓\u{1112a}𑄝\u{1112d}𑄇\u{11128}"), ("ceb", "Pardubický kraj"), ("cs", "Pardubický kraj"), ("da", "Pardubice"), ("de", "Pardubický kraj"), ("el", "Περιφέρεια Παρντούμπιτσε"), ("en", "Pardubický"), ("es", "Región de Pardubice"), ("et", "Pardubice maakond"), ("eu", "Pardubice eskualdea"), ("fa", "منطقه پاردوبیتسه"), ("fi", "Pardubicen lääni"), ("fr", "Région de Pardubice"), ("gu", "પરડ\u{ac1}બિસ પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז פרדוביצה"), ("hi", "पार\u{94d}द\u{941}बिस प\u{94d}रद\u{947}श"), ("hr", "Pardubički kraj"), ("hu", "Pardubicei kerület"), ("id", "Daerah Pardubice"), ("it", "regione di Pardubice"), ("ja", "パルドゥビツェ州"), ("ka", "პარდუბიცეს მხარე"), ("kn", "ಪಾರ\u{ccd}ಡ\u{cc2}ಬ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "파르두비체 주"), ("lt", "Pardubicių kraštas"), ("lv", "Pardubices apgabals"), ("mk", "Пардубички крај"), ("mr", "परद\u{941}बीस प\u{94d}रद\u{947}श"), ("ms", "Daerah Pardubice"), ("nb", "Pardubice region"), ("nl", "Pardubice"), ("no", "Pardubice region"), ("pl", "Kraj pardubicki"), ("pt", "Pardubice"), ("ro", "Regiunea Pardubice"), ("ru", "Пардубицкий край"), ("si", "පඩ\u{dd4}බය\u{dd2}ස\u{dca} කල\u{dcf}පය"), ("sk", "Pardubický kraj"), ("sr", "Пардубички крај"), ("sr_Latn", "Pardubički kraj"), ("sv", "Pardubice"), ("ta", "படுப\u{bc0}ஸ\u{bcd} பகுதி"), ("te", "ప\u{c3e}ర\u{c4d}డుబ\u{c48}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตปาด\u{e39}บ\u{e34}ซ"), ("tr", "Pardubice Bölgesi"), ("uk", "Пардубицький край"), ("ur", "پاردوبیتسے علاقہ"), ("vi", "Pardubice"), ("zh", "帕爾杜比采州")]),
                        unofficial_name_list: ["Pardubický kraj"].to_vec(),
                    }
                ),
                (
                    "531",
                    Subdivision{
                        name: "531",
                        country_alpha2: Alpha2::CZ,
                        code: "531",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خروديم"), ("be", "Хрудзім"), ("bg", "Хрудим"), ("ca", "Districte de Chrudim"), ("ccp", "𑄇\u{11133}𑄢\u{1112a}𑄓\u{11128}𑄟\u{11134}"), ("ceb", "Okres Chrudim"), ("cs", "okres Chrudim"), ("de", "Okres Chrudim"), ("en", "Chrudim"), ("es", "Distrito de Chrudim"), ("fa", "شهرستان خرودیم"), ("fr", "District de Chrudim"), ("he", "מחוז משנה חרודים"), ("hu", "Chrudimi járás"), ("hy", "Խրուդիմի շրջան"), ("it", "Distretto di Chrudim"), ("ka", "ხრუდიმის რაიონი"), ("lt", "Chrudimo rajonas"), ("lv", "Hrudimas apriņķis"), ("ms", "Daerah Chrudim"), ("nl", "Okres Chrudim"), ("pl", "Powiat Chrudim"), ("pt", "Chrudim"), ("ru", "Хрудим"), ("sk", "Okres Chrudim"), ("sr", "Округ Хрудим"), ("sr_Latn", "Okrug Hrudim"), ("sv", "Chrudim"), ("tr", "Chrudim ilçesi"), ("uk", "Хрудім"), ("ur", "خودیم ضلع"), ("vi", "Chrudim"), ("zh", "赫魯迪姆縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "532",
                    Subdivision{
                        name: "532",
                        country_alpha2: Alpha2::CZ,
                        code: "532",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باردوبيتسه"), ("be", "раён Пардубіцы"), ("bg", "Пардубице"), ("ca", "Districte de Pardubice"), ("ccp", "𑄛𑄢\u{11134}𑄓\u{1112a}𑄝\u{1112d}𑄌\u{11134}"), ("ceb", "Okres Pardubice"), ("cs", "okres Pardubice"), ("de", "Okres Pardubice"), ("en", "Pardubice"), ("es", "Distrito de Pardubice"), ("eu", "Pardubiceko barrutia"), ("fa", "ناحیه پاردوبیتسه"), ("fr", "District de Pardubice"), ("he", "מחוז משנה פרדוביצה"), ("hu", "Pardubicei járás"), ("hy", "Պարուբիցեի շրջան"), ("it", "Distretto di Pardubice"), ("ka", "პარდუბიცეს რაიონი"), ("lt", "Pardubicių rajonas"), ("lv", "Pardubices apriņķis"), ("ms", "Daerah Pardubice²"), ("nl", "Okres Pardubice"), ("pl", "Powiat Pardubice"), ("pt", "Pardubice²"), ("ru", "Пардубице"), ("sk", "Okres Pardubice"), ("sr", "Округ Пардубице"), ("sr_Latn", "Okrug Pardubice"), ("sv", "Pardubice²"), ("tr", "Pardubice ilçesi"), ("uk", "Пардубице"), ("ur", "پاردوبیتسے ضلع"), ("vi", "Pardubice²"), ("zh", "帕爾杜比采縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "533",
                    Subdivision{
                        name: "533",
                        country_alpha2: Alpha2::CZ,
                        code: "533",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سفيتافي"), ("be", "раён Світаві"), ("bg", "Свитави"), ("ca", "Districte de Svitavy"), ("ccp", "𑄞\u{11128}𑄑𑄞\u{11128}"), ("ceb", "Okres Svitavy"), ("cs", "okres Svitavy"), ("de", "Okres Svitavy"), ("en", "Svitavy"), ("es", "Distrito de Svitavy"), ("eu", "Svitavyko barrutia"), ("fa", "شهرستان سویتاوی"), ("fr", "District de Svitavy"), ("he", "מחוז משנה סביטאבי"), ("hu", "Svitavy-i járás"), ("hy", "Սվիտավիի շրջան"), ("it", "Distretto di Svitavy"), ("ja", "スヴィタヴィ郡"), ("ka", "სვიტავის რაიონი"), ("lt", "Svitavų rajonas"), ("lv", "Svitavu apriņķis"), ("ms", "Daerah Svitavy"), ("nl", "Okres Svitavy"), ("pl", "Powiat Svitavy"), ("pt", "Svitavy"), ("ru", "Свитави"), ("sk", "Okres Svitavy"), ("sr", "Округ Свитави"), ("sr_Latn", "Okrug Svitavi"), ("sv", "Okres Svitavy"), ("tr", "Svitavy ilçesi"), ("uk", "Світави"), ("ur", "سویتاوی ضلع"), ("vi", "Svitavy"), ("zh", "斯維塔維縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "534",
                    Subdivision{
                        name: "534",
                        country_alpha2: Alpha2::CZ,
                        code: "534",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوستي ناد أورليتسي"), ("be", "Усці-над-Орліцай"), ("bg", "Усти над Орлици"), ("ca", "Districte d’Ústí nad Orlicí"), ("ccp", "𑄅\u{1112a}𑄌\u{11134}𑄑\u{11128} 𑄚𑄖\u{11134} 𑄃\u{11127}𑄢\u{11134}𑄣\u{11128}𑄌\u{11128}"), ("ceb", "Okres Ústí nad Orlicí"), ("cs", "okres Ústí nad Orlicí"), ("de", "Okres Ústí nad Orlicí"), ("en", "Ústí nad Orlicí"), ("es", "Distrito de Ústí nad Orlicí"), ("eu", "Ústí nad Orlicíko barrutia"), ("fa", "شهرستان اوستی ناد اورلیتسی"), ("fr", "District d’Ústí nad Orlicí"), ("he", "מחוז משנה אוסטי נד אורליצי"), ("hu", "Ústí nad Orlicí-i járás"), ("hy", "Ուստի-նադ-Օրլիցի շրջան"), ("it", "Distretto di Ústí nad Orlicí"), ("ja", "ウスティ・ナドゥ・オーリシ"), ("ka", "უსტი-ნად-ორლიცის რაიონი"), ("lt", "Ūsčio prie Orlicės rajonas"), ("lv", "Ūstu pie Orlices apriņķis"), ("ms", "Daerah Ústí nad Orlicí"), ("nb", "Ústí nad Orlicí"), ("nl", "Okres Ústí nad Orlicí"), ("no", "Ústí nad Orlicí"), ("pl", "Powiat Uście nad Orlicą"), ("ru", "Усти-над-Орлици"), ("sk", "Okres Ústí nad Orlicí"), ("sr", "Округ Усти на Орлици"), ("sr_Latn", "Okrug Usti na Orlici"), ("sv", "Okres Ústí nad Orlicí"), ("tr", "Ústí nad Orlicí ilçesi"), ("uk", "Усті-над-Орлиццю"), ("ur", "اوستی ناد اورلیسی ضلع"), ("vi", "Ústí nad Orlicí"), ("zh", "奧爾利采河畔烏斯季縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "63",
                    Subdivision{
                        name: "63",
                        country_alpha2: Alpha2::CZ,
                        code: "63",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.44900519999999), longitude: Some(15.6405934), max_latitude: Some(49.8611195), min_latitude: Some(48.9375446), max_longitude: Some(16.4184401), min_longitude: Some(14.8887667)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فيسوتشنا"), ("az", "Vısoçina diyarı"), ("be", "Край Высачына"), ("bg", "Височински край"), ("bn", "ভ\u{9be}ইসোচিন\u{9be} অঞ\u{9cd}চল"), ("bs", "Visočinski kraj"), ("ca", "Regió de Vysočina"), ("ccp", "𑄞\u{11128}𑄥\u{1112e}𑄥\u{11128}𑄚"), ("ceb", "Kraj Vysočina"), ("cs", "Kraj Vysočina"), ("da", "Vysočina"), ("de", "Kraj Vysočina"), ("el", "Περιφέρεια Βίσοτσινα"), ("en", "Vysočina"), ("es", "Región de Vysočina"), ("et", "Vysočina"), ("eu", "Vysocina"), ("fa", "استان ویسوچینا"), ("fi", "Vysočinan lääni"), ("fr", "Région de Vysočina"), ("gu", "વિસોસિના પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז ויסוצ׳ינה"), ("hi", "विसोकिना क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Vysočina"), ("hu", "Vysočina kerület"), ("hy", "Վիսոչինի ծայրամաս"), ("id", "Daerah Vysočina"), ("it", "regione di Vysočina"), ("ja", "ヴィソチナ州"), ("ka", "ვისოჩინის მხარე"), ("kn", "ವೈಸೋಸ\u{cbf}ನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "비소치나 주"), ("lt", "Vysočinos kraštas"), ("lv", "Visočinas apgabals"), ("mk", "Височински крај"), ("mr", "व\u{94d}हिस\u{94d}च\u{947}ना प\u{94d}रद\u{947}श"), ("ms", "Daerah Vysočina"), ("nb", "Vysočina region"), ("nl", "Vysočina"), ("no", "Vysočina region"), ("pl", "Kraj Vysočina"), ("pt", "Vysočina"), ("ro", "Regiunea Vysočina"), ("ru", "Край Высочина"), ("si", "වය\u{dd2}සොක\u{dd2}න\u{dcf} කල\u{dcf}පය"), ("sk", "Kraj Vysočina"), ("sr", "Крај Височина"), ("sr_Latn", "Kraj Visočina"), ("sv", "Vysočina"), ("ta", "வயசோஸின\u{bbe} பகுதி"), ("te", "వ\u{c48}స\u{c4b}స\u{c3f}న\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ว\u{e34}โซซ\u{e34}น\u{e48}า"), ("tr", "Vysocina Bölgesi"), ("uk", "Край Височіна"), ("ur", "ویسوچیانا علاقہ"), ("vi", "Vysočina"), ("zh", "維索基納州")]),
                        unofficial_name_list: ["Jihlavský"].to_vec(),
                    }
                ),
                (
                    "631",
                    Subdivision{
                        name: "631",
                        country_alpha2: Alpha2::CZ,
                        code: "631",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هافليتشكوف برود"), ("be", "раён Гаўлічкаў Брод"), ("bg", "Хавличкув Брод"), ("ca", "Districte de Havlíčkův Brod"), ("ccp", "𑄦𑄛\u{11134}𑄣\u{11128}𑄇\u{1112a}𑄛\u{11134} 𑄝\u{11133}𑄢\u{11127}𑄖\u{11134}"), ("ceb", "Okres Havlíčkův Brod"), ("cs", "okres Havlíčkův Brod"), ("de", "Okres Havlíčkův Brod"), ("en", "Havlíčkův Brod"), ("es", "Distrito de Havlíčkův Brod"), ("eu", "Havlíčkův Brodeko barrutia"), ("fa", "ناحیه هاولیچکوف برود"), ("fr", "District de Havlíčkův Brod"), ("he", "מחוז משנה הבליצקוב ברוד"), ("hu", "Havlíčkův Brod-i járás"), ("hy", "Հավլիչկուվ Բրոդի շրջան"), ("it", "Distretto di Havlíčkův Brod"), ("ka", "ჰავლიცკუვ-ბროდის რაიონი"), ("lt", "Havličkūv Brodo rajonas"), ("lv", "Havlīčkūvbrodas apriņķis"), ("ms", "Daerah Havlíčkův Brod"), ("nb", "Havlíčkův Brod"), ("nl", "Okres Havlíčkův Brod"), ("no", "Havlíčkův Brod"), ("pl", "Powiat Havlíčkův Brod"), ("ru", "Гавличкув-Брод"), ("sk", "Okres Havlíčkův Brod"), ("sr", "Округ Хавличкув Брод"), ("sr_Latn", "Okrug Havličkuv Brod"), ("sv", "Okres Havlíčkův Brod"), ("tr", "Havlíčkův Brod ilçesi"), ("uk", "Гавличкув-Брод"), ("ur", "ہاولیچکورف برود ضلع"), ("vi", "Havlíčkův Brod"), ("zh", "哈夫利奇庫夫布羅德縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "632",
                    Subdivision{
                        name: "632",
                        country_alpha2: Alpha2::CZ,
                        code: "632",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يهلافا"), ("be", "Раён Йіглава"), ("bg", "Ихлава"), ("ca", "Districte de Jihlava"), ("ccp", "𑄎\u{11128}𑄦\u{11134}𑄣𑄛\u{11134}"), ("ceb", "Okres Jihlava"), ("cs", "okres Jihlava"), ("de", "Okres Jihlava"), ("el", "κομητεία Γίσλαβα"), ("en", "Jihlava"), ("es", "Distrito de Jihlava"), ("eu", "Jihlavako barrutia"), ("fa", "شهرستان ییهلاوا"), ("fr", "District de Jihlava"), ("he", "מחוז משנה ייהלבה"), ("hu", "Jihlavai járás"), ("hy", "Յիհլավայի շրջան"), ("it", "Distretto di Jihlava"), ("ka", "იიჰლავის რაიონი"), ("lt", "Jihlavos rajonas"), ("lv", "Jihlavas apriņķis"), ("ms", "Daerah Jihlava"), ("nb", "Jihlava"), ("nl", "Okres Jihlava"), ("no", "Jihlava"), ("pl", "Powiat Igława"), ("pt", "Jihlava"), ("ru", "Йиглава"), ("sk", "Okres Jihlava"), ("sr", "Округ Јихлава"), ("sr_Latn", "Okrug Jihlava"), ("sv", "Okres Jihlava"), ("tr", "Jihlava ilçesi"), ("uk", "Їглава"), ("ur", "ایہاوا ضلع"), ("vi", "Jihlava"), ("zh", "伊赫拉瓦縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "633",
                    Subdivision{
                        name: "633",
                        country_alpha2: Alpha2::CZ,
                        code: "633",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلهريموف"), ("be", "Пельгржымаў"), ("bg", "Пелхржимов"), ("ca", "Districte de Pelhřimov"), ("ccp", "𑄛𑄬𑄣\u{11134}𑄦\u{11133}𑄢\u{11128}𑄟\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Pelhřimov"), ("cs", "okres Pelhřimov"), ("de", "Okres Pelhřimov"), ("en", "Pelhřimov"), ("es", "Distrito de Pelhřimov"), ("eu", "Pelhřimoveko barrutia"), ("fa", "شهرستان پلهرژیموف"), ("fr", "District de Pelhřimov"), ("he", "מחוז משנה פלהרימוב"), ("hu", "Pelhřimovi járás"), ("hy", "Պելհրիմովի շրջան"), ("it", "Distretto di Pelhřimov"), ("ka", "პელჰრჟიმოვის რაიონი"), ("lt", "Pelhržimovo rajonas"), ("lv", "Pelhržimovas apriņķis"), ("ms", "Daerah Pelhřimov"), ("nb", "Pelhřimov"), ("nl", "Okres Pelhřimov"), ("no", "Pelhřimov"), ("pl", "Powiat Pelhřimov"), ("pt", "Pelhřimov"), ("ru", "Пельгржимов"), ("sk", "Okres Pelhřimov"), ("sr", "Округ Пелхримов"), ("sr_Latn", "Okrug Pelhrimov"), ("sv", "Okres Pelhřimov"), ("tr", "Pelhřimov ilçesi"), ("uk", "Пельгржимов"), ("ur", "پیلہریموف ضلع"), ("vi", "Pelhřimov"), ("zh", "佩爾赫日莫夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "634",
                    Subdivision{
                        name: "634",
                        country_alpha2: Alpha2::CZ,
                        code: "634",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تربيتش"), ("be", "раён Тршэбіч"), ("bg", "Тршебич"), ("ca", "Districte de Třebíč"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄝\u{11128}𑄇\u{11134}"), ("ceb", "Okres Třebíč"), ("cs", "okres Třebíč"), ("de", "Okres Třebíč"), ("en", "Třebíč"), ("es", "Distrito de Třebíč"), ("eu", "Třebíčeko barrutia"), ("fa", "منطقه تربیچ"), ("fr", "District de Třebíč"), ("he", "מחוז משנה טשביץ׳"), ("hu", "Třebíči járás"), ("hy", "Տրեբիչի շրջան"), ("it", "Distretto di Třebíč"), ("ka", "ტრჟებიცის რაიონი"), ("lt", "Tršebyčo rajonas"), ("lv", "Tršebīčas apriņķis"), ("ms", "Daerah Třebíč"), ("nb", "Třebíč"), ("nl", "Okres Třebíč"), ("no", "Třebíč"), ("pl", "Powiat Třebíč"), ("pt", "Třebíč (distrito)"), ("ro", "Třebíč"), ("ru", "Тршебич"), ("sk", "Okres Třebíč"), ("sr", "Округ Требич"), ("sr_Latn", "Okrug Trebič"), ("sv", "Okres Třebíč"), ("tr", "Třebíč ilçesi"), ("uk", "Тршебич"), ("ur", "تشربیچ ضلع"), ("vi", "Třebíč"), ("zh", "特熱比奇縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "635",
                    Subdivision{
                        name: "635",
                        country_alpha2: Alpha2::CZ,
                        code: "635",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جديار ناد سازافو"), ("bg", "Ждяр над Сазавоу"), ("ca", "Districte de Žďár nad Sázavou"), ("ccp", "𑄓𑄢\u{11134} 𑄚𑄖\u{11134} 𑄥𑄎𑄞\u{1112e}𑄅\u{1112a}"), ("ceb", "Okres Žďár nad Sázavou"), ("cs", "okres Žďár nad Sázavou"), ("de", "Okres Žďár nad Sázavou"), ("en", "Žďár nad Sázavou"), ("es", "Distrito de Žďár nad Sázavou"), ("eu", "Žďár nad Sázavouko barrutia"), ("fa", "ناحیه ژدار ناد سازاوو"), ("fr", "District de Žďár nad Sázavou"), ("he", "מחוז משנה ז׳דאר נד סאזאבואו"), ("hu", "Žďár nad Sázavou-i járás"), ("hy", "Զդար-նադ-Սազավոուի շրջան"), ("it", "Distretto di Žďár nad Sázavou"), ("ka", "ჟდიარ-ნად-საზავოის რაიონი"), ("lt", "Ždiaro prie Sazavos rajonas"), ("lv", "Ždjāras pie Sāzavas apriņķis"), ("ms", "Daerah Žďár nad Sázavou"), ("nb", "Žďár nad Sázavou"), ("nl", "Okres Žďár nad Sázavou"), ("no", "Žďár nad Sázavou"), ("pl", "Powiat Zdziar nad Sazawą"), ("pt", "Žďár nad Sázavou (distrito)"), ("ru", "Ждяр-над-Сазавоу"), ("sk", "Okres Žďár nad Sázavou"), ("sr", "Округ Ждјар на Сазави"), ("sr_Latn", "Okrug Ždjar na Sazavi"), ("sv", "Okres Žďár nad Sázavou"), ("tr", "Žďár nad Sázavou ilçesi"), ("uk", "Ждяр-над-Сазавою"), ("ur", "زشدار ناد سازوو ضلع"), ("vi", "Žďár nad Sázavou"), ("zh", "薩扎瓦河畔日賈爾縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "64",
                    Subdivision{
                        name: "64",
                        country_alpha2: Alpha2::CZ,
                        code: "64",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.9544528), longitude: Some(16.7676899), max_latitude: Some(49.63325469999999), min_latitude: Some(48.61653769999999), max_longitude: Some(17.6458148), min_longitude: Some(15.5424287)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مورافيا الجنوبية"), ("az", "Cənubi Moraviya diyarı"), ("be", "Паўднёвамараўскі край"), ("bg", "Южноморавски край"), ("bn", "স\u{9be}উথ ম\u{9c1}র\u{9be}ভিয\u{9bc}\u{9be}ন অঞ\u{9cd}চল"), ("bs", "Južnomoravski kraj"), ("ca", "Regió de Moràvia Meridional"), ("ccp", "𑄎\u{11128}𑄦\u{1112e}𑄟\u{11127}𑄢𑄛\u{11134}𑄇\u{11128}"), ("ceb", "Jihomoravský kraj"), ("cs", "Jihomoravský kraj"), ("da", "Sydmähren"), ("de", "Jihomoravský kraj"), ("el", "Περιφέρεια Νότιας Μοραβίας"), ("en", "Jihomoravský"), ("es", "Región de Moravia Meridional"), ("et", "Lõuna-Morava maakond"), ("eu", "Hego Moravia"), ("fa", "منطقه موراویای جنوبی"), ("fi", "Etelä-Määrin lääni"), ("fr", "Moravie du Sud"), ("gl", "Rexión de Moravia Meridional"), ("gu", "દક\u{acd}ષિણ મોરાવિયન પ\u{acd}રદ\u{ac7}શ"), ("he", "מורביה הדרומית"), ("hi", "दक\u{94d}षिण मोरावियन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Južna Moravska"), ("hu", "Dél-morvaországi kerület"), ("hy", "Հարավային Մորավիայի շրջան"), ("id", "Wilayah Moravia Selatan"), ("it", "Moravia meridionale"), ("ja", "南モラヴィア州"), ("ka", "სამხრეთ მორავიის მხარე"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಮೊರವ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "남모라바 주"), ("lt", "Pietų Moravijos kraštas"), ("lv", "Dienvidmorāvijas apgabals"), ("mk", "Јужноморавски крај"), ("mr", "दक\u{94d}षिण मोरावियन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Moravia Selatan"), ("nb", "Sydmähriske region"), ("nl", "Zuid-Moravië"), ("no", "Sydmähriske region"), ("pl", "Kraj południowomorawski"), ("pt", "Morávia do Sul"), ("ro", "Regiunea Moravia de Sud"), ("ru", "Южноморавский край"), ("si", "දක\u{dd4}ණ\u{dd4} මොරව\u{dd2}යන\u{dca} කල\u{dcf}පය"), ("sk", "Juhomoravský kraj"), ("sr", "Јужноморавски крај"), ("sr_Latn", "Južnomoravski kraj"), ("sv", "Södra Mähren"), ("ta", "தெற\u{bcd}கு மொர\u{bbe}வின\u{bcd} பகுதி"), ("te", "దక\u{c4d}ష\u{c3f}ణ మ\u{c4a}ర\u{c3e}వ\u{c3f}యన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เซาท\u{e4c} โมราเว\u{e35}ยน"), ("tr", "Güney Moravya ili"), ("uk", "Південноморавський край"), ("ur", "جنوبی موراویائی علاقہ"), ("vi", "Nam Moravia"), ("zh", "南摩拉維亞州")]),
                        unofficial_name_list: ["Brněnský"].to_vec(),
                    }
                ),
                (
                    "641",
                    Subdivision{
                        name: "641",
                        country_alpha2: Alpha2::CZ,
                        code: "641",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلانسكو"), ("be", "Раён Бланска"), ("bg", "Бланско"), ("ca", "Districte de Blansko"), ("ccp", "𑄇\u{11133}𑄣𑄚\u{11134}𑄇\u{1112e}"), ("ceb", "Okres Blansko"), ("cs", "okres Blansko"), ("de", "Okres Blansko"), ("en", "Blansko"), ("es", "Distrito de Blansko"), ("eu", "Blanskoko barrutia"), ("fa", "شهرستان بلانسکو"), ("fr", "District de Blansko"), ("he", "מחוז משנה בלנסקו"), ("hr", "Okrug Blansko"), ("hu", "Blanskói járás"), ("hy", "Բլանսկոյի շրջան"), ("it", "Distretto di Blansko"), ("ka", "ბლანსკოს რაიონი"), ("lt", "Blansko rajonas"), ("lv", "Blansko apriņķis"), ("ms", "Daerah Blansko"), ("nl", "Okres Blansko"), ("pl", "Powiat Blansko"), ("pt", "Blansko (distrito)"), ("ru", "Бланско"), ("sk", "Okres Blansko"), ("sr", "Округ Бланско"), ("sr_Latn", "Okrug Blansko"), ("sv", "Okres Blansko"), ("tr", "Blansko ilçesi"), ("uk", "Бланско"), ("ur", "بلانسکو ضلع"), ("vi", "Blansko"), ("zh", "布蘭斯科縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "642",
                    Subdivision{
                        name: "642",
                        country_alpha2: Alpha2::CZ,
                        code: "642",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برنو-المدينة"), ("be", "Брно-горад"), ("bg", "Бърно-град"), ("ca", "Districte de Brno-město"), ("ccp", "𑄝\u{11133}𑄢𑄚\u{1112e}-𑄟𑄬𑄌\u{11134}𑄑\u{1112e}"), ("ceb", "Město Brno"), ("cs", "okres Brno-město"), ("de", "Okres Brno-město"), ("en", "Brno-město"), ("fa", "شهرستان برنو-میستو"), ("fr", "District de Brno-město"), ("hu", "Brno városi járás"), ("it", "Distretto di Brno-město"), ("lt", "Brno miesto rajonas"), ("mk", "Брно-град"), ("ms", "Daerah Bandaraya Brno"), ("nl", "Okres Brno-město"), ("pt", "Brno (distrito)"), ("ru", "Брно-город"), ("sk", "Okres Brno-mesto"), ("sr", "Округ Брно-град"), ("sr_Latn", "Okrug Brno-grad"), ("sv", "Město Brno"), ("uk", "Брно-місто"), ("ur", "برنو-شہر ضلع"), ("vi", "Brno"), ("zh", "布爾諾城縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "643",
                    Subdivision{
                        name: "643",
                        country_alpha2: Alpha2::CZ,
                        code: "643",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برنو-الريف"), ("be", "Брно-прыгарад"), ("bg", "Бърно-район"), ("ca", "Districte de Brno-venkov"), ("ccp", "𑄝\u{11133}𑄢𑄚\u{1112e}-𑄞𑄬𑄚\u{11134}𑄇\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Brno-Venkov"), ("cs", "okres Brno-venkov"), ("de", "Okres Brno-venkov"), ("el", "Επαρχία Μπρνο"), ("en", "Brno-venkov"), ("es", "Distrito de Brno"), ("eu", "Brno-Landako barrutia"), ("fa", "شهرستان برنو-ونکوو"), ("fr", "District de Brno-venkov"), ("hu", "Brno-vidéki járás"), ("hy", "Բրնո-վենկովի շրջան"), ("it", "Distretto di Brno-venkov"), ("ka", "ბრნო-ვენკოვის რაიონი"), ("lt", "Brno savivaldybės rajonas"), ("lv", "Brno lauku apriņķis"), ("mk", "Брно-предградие"), ("ms", "Daerah Brno-Country"), ("nl", "Okres Brno-venkov"), ("pl", "Powiat Brno"), ("pt", "Brno-Venkov"), ("ru", "Брно-пригород"), ("sk", "Okres Brno-venkov"), ("sr", "Округ Брно-околина"), ("sr_Latn", "Okrug Brno-okolina"), ("sv", "Brno-venkov"), ("uk", "Брно-околиця"), ("ur", "برنو-کنٹری ضلع"), ("vi", "Brno-venkov"), ("zh", "布爾諾郊縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "644",
                    Subdivision{
                        name: "644",
                        country_alpha2: Alpha2::CZ,
                        code: "644",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بريتسلاف"), ("be", "Раён Бржэцлаў"), ("bg", "Бржецлав"), ("ca", "Districte de Břeclav"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄇\u{11133}𑄣\u{11133}𑄠𑄛\u{11134}"), ("ceb", "Okres Břeclav"), ("cs", "okres Břeclav"), ("de", "Okres Břeclav"), ("en", "Břeclav"), ("es", "Distrito de Břeclav"), ("eu", "Břeclaveko barrutia"), ("fa", "شهرستان برژتسلاو"), ("fr", "District de Břeclav"), ("he", "מחוז משנה ברצלב"), ("hu", "Břeclavi járás"), ("hy", "Բրեցլավի շրջան"), ("it", "Distretto di Břeclav"), ("ja", "ブジェツラフ郡"), ("ka", "ბრჟეცლავის რაიონი"), ("lt", "Bržeclavo rajonas"), ("lv", "Bržeclavas apriņķis"), ("ms", "Daerah Břeclav"), ("nl", "Okres Břeclav"), ("pl", "Powiat Brzecław"), ("pt", "Břeclav"), ("ru", "Бржецлав"), ("sk", "Okres Břeclav"), ("sr", "Округ Брецлав"), ("sr_Latn", "Okrug Breclav"), ("sv", "Okres Břeclav"), ("tr", "Břeclav ilçesi"), ("uk", "Бржецлав"), ("ur", "بشرسلاف ضلع"), ("vi", "Břeclav"), ("zh", "布熱茨拉夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "645",
                    Subdivision{
                        name: "645",
                        country_alpha2: Alpha2::CZ,
                        code: "645",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هودونين"), ("bg", "Ходонин"), ("ca", "Districte de Hodonín"), ("ccp", "𑄦\u{11127}𑄓\u{11127}𑄚\u{11128}𑄚\u{11134}"), ("ceb", "Okres Hodonín"), ("cs", "okres Hodonín"), ("de", "Okres Hodonín"), ("en", "Hodonín"), ("es", "Distrito de Hodonín"), ("eu", "Hodoníngo barrutia"), ("fa", "ناحیه هودونین"), ("fr", "District de Hodonín"), ("he", "מחוז משנה הודונין"), ("hu", "Hodoníni járás"), ("hy", "Հոդոնինի շրջան"), ("it", "Distretto di Hodonín"), ("ka", "ჰოდონინის რაიონი"), ("lt", "Hodonyno rajonas"), ("lv", "Hodonīnas apriņķis"), ("ms", "Daerah Hodonín"), ("nl", "Okres Hodonín"), ("pl", "Powiat Hodonín"), ("pt", "Hodonín (distrito)"), ("ru", "Годонин"), ("sk", "Okres Hodonín"), ("sr", "Округ Ходоњин"), ("sr_Latn", "Okrug Hodonjin"), ("sv", "Okres Hodonín"), ("tr", "Hodonín ilçesi"), ("uk", "Годонін"), ("ur", "ہودونین ضلع"), ("vi", "Hodonín"), ("zh", "霍多寧縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "646",
                    Subdivision{
                        name: "646",
                        country_alpha2: Alpha2::CZ,
                        code: "646",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيشكوف"), ("be", "Раён Вішкаў"), ("bg", "Вишков"), ("ca", "Districte de Vyškov"), ("ccp", "𑄞\u{11128}𑄌\u{11134}𑄇\u{11127}𑄛\u{11134}"), ("ceb", "Okres Vyškov"), ("cs", "okres Vyškov"), ("de", "Okres Vyškov"), ("en", "Vyškov"), ("es", "Distrito de Vyškov"), ("fa", "شهرستان ویشکوف"), ("fr", "District de Vyškov"), ("he", "מחוז משנה ויסקוב"), ("hu", "Vyškovi járás"), ("hy", "Վիշկովի շրջան"), ("it", "Distretto di Vyškov"), ("ka", "ვისკოვის რაიონი"), ("lt", "Viškovo rajonas"), ("lv", "Viškovas apriņķis"), ("ms", "Daerah Vyškov"), ("nl", "Okres Vyškov"), ("pl", "Powiat Vyškov"), ("ru", "Вишков"), ("sk", "Okres Vyškov"), ("sr", "Округ Вишков"), ("sr_Latn", "Okrug Viškov"), ("sv", "Okres Vyškov"), ("tr", "Vyškov ilçesi"), ("uk", "Вишков"), ("ur", "ویشکوف ضلع"), ("vi", "Vyškov"), ("zh", "維什科夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "647",
                    Subdivision{
                        name: "647",
                        country_alpha2: Alpha2::CZ,
                        code: "647",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زنويمو"), ("be", "Раён Знойма"), ("bg", "Зноймо"), ("ca", "Districte de Znojmo"), ("ccp", "𑄚\u{11127}𑄌\u{11134}𑄟\u{1112e}"), ("ceb", "Okres Znojmo"), ("cs", "okres Znojmo"), ("de", "Okres Znojmo"), ("en", "Znojmo"), ("es", "Distrito de Znojmo"), ("eu", "Znojmoko barrutia"), ("fa", "شهرستان زنویمو"), ("fr", "District de Znojmo"), ("hu", "Znojmói járás"), ("hy", "Զնոյմոյի շրջան"), ("it", "Distretto di Znojmo"), ("ka", "ზნოიმოს რაიონი"), ("lt", "Znoimo rajonas"), ("lv", "Znojmo apriņķis"), ("ms", "Daerah Znojmo"), ("nl", "Okres Znojmo"), ("pl", "Powiat Znojmo"), ("ru", "Зноймо"), ("sk", "Okres Znojmo"), ("sr", "Округ Знојмо"), ("sr_Latn", "Okrug Znojmo"), ("sv", "Okres Znojmo"), ("tr", "Znojmo ilçesi"), ("uk", "Зноймо"), ("ur", "زنویمو ضلع"), ("vi", "Znojmo"), ("zh", "茲諾伊莫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "71",
                    Subdivision{
                        name: "71",
                        country_alpha2: Alpha2::CZ,
                        code: "71",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.65865489999999), longitude: Some(17.0811406), max_latitude: Some(50.4494346), min_latitude: Some(49.2669759), max_longitude: Some(17.9172214), min_longitude: Some(16.7115968)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أولوموتس"), ("az", "Olomoutskiy diyarı"), ("be", "Оламаўцкі край"), ("bg", "Оломоуцки край"), ("bn", "ওলোম\u{9c1}ক অঞ\u{9cd}চল"), ("bs", "Olomoucki kraj"), ("ca", "Regió d’Olomouc"), ("ccp", "𑄃\u{1112e}𑄣\u{1112e}𑄟\u{1112f}𑄇\u{11128}"), ("ceb", "Olomoucký kraj"), ("cs", "Olomoucký kraj"), ("da", "Olomouc"), ("de", "Olomoucký kraj"), ("el", "Περιφέρεια Όλομουτς"), ("en", "Olomoucký"), ("es", "Región de Olomouc"), ("et", "Olomouci maakond"), ("eu", "Olomouc eskualdea"), ("fa", "منطقه اولوموتس"), ("fi", "Olomoucin lääni"), ("fr", "Région d’Olomouc"), ("gu", "ઓલોમૌક પ\u{acd}રદ\u{ac7}શ"), ("he", "אולומואוץ (מחוז)"), ("hi", "ओलोमौक क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Olomoučki kraj"), ("hu", "Olomouci kerület"), ("id", "Daerah Olomouc"), ("it", "regione di Olomouc"), ("ja", "オロモウツ州"), ("ka", "ოლომოუცის მხარე"), ("kn", "ಓಲೋಮ\u{ccc}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "올로모우츠 주"), ("lt", "Olomouco kraštas"), ("lv", "Olomoucas apgabals"), ("mk", "Оломоучки крај"), ("mr", "ऑलोकिक प\u{94d}रद\u{947}श"), ("ms", "Daerah Olomouc"), ("nb", "Olomouc region"), ("nl", "Olomouc"), ("no", "Olomouc region"), ("pl", "Kraj ołomuniecki"), ("pt", "Olomouc"), ("ro", "Regiunea Olomouc"), ("ru", "Оломоуцкий край"), ("si", "ඔලොමොඋක\u{dca} කල\u{dcf}පය"), ("sk", "Olomoucký kraj"), ("sr", "Оломоуцки крај"), ("sr_Latn", "Olomoucki kraj"), ("sv", "Olomouc"), ("ta", "ஒலோமெக\u{bcd} பகுதி"), ("te", "ఓల\u{c4b}మ\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โอลม\u{e38}ก โบเกซ\u{e35}"), ("tr", "Olomuc Bölgesi"), ("uk", "Оломоуцький край"), ("ur", "اولوموک علاقہ"), ("vi", "Olomouc"), ("zh", "奧洛穆克州")]),
                        unofficial_name_list: ["Olomoucký kraj"].to_vec(),
                    }
                ),
                (
                    "711",
                    Subdivision{
                        name: "711",
                        country_alpha2: Alpha2::CZ,
                        code: "711",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يسينيك"), ("bg", "Есеник"), ("ca", "Districte de Jeseník"), ("ccp", "𑄎𑄬𑄥𑄬𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Okres Jeseník"), ("cs", "okres Jeseník"), ("de", "Okres Jeseník"), ("en", "Jeseník"), ("es", "Distrito de Jeseník"), ("eu", "Jeseníkeko barrutia"), ("fa", "شهرستان یسنیک"), ("fr", "District de Jeseník"), ("he", "מחוז משנה יסניק"), ("hu", "Jeseníki járás"), ("hy", "Յեսենիկի շրջան"), ("it", "Distretto di Jeseník"), ("ka", "ესენიკის რაიონი"), ("lt", "Jesenyko rajonas"), ("lv", "Jesenīkas apriņķis"), ("ms", "Daerah Jeseník"), ("nl", "Okres Jeseník"), ("pl", "Powiat Jesionik"), ("pt", "Jeseník"), ("ru", "Есеник"), ("sk", "Okres Jeseník"), ("sr", "Округ Јесењик"), ("sr_Latn", "Okrug Jesenjik"), ("sv", "Jeseník"), ("tr", "Jeseník ilçesi"), ("uk", "Єсенік"), ("ur", "یسنک ضلع"), ("vi", "Jeseník"), ("zh", "耶塞尼克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "712",
                    Subdivision{
                        name: "712",
                        country_alpha2: Alpha2::CZ,
                        code: "712",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أولوموتس"), ("be", "Оламаўц"), ("bg", "Оломоуц"), ("ca", "Districte d’Olomouc"), ("ccp", "𑄃\u{11127}𑄣\u{1112e}𑄟𑄅\u{1112a}𑄇\u{11134}"), ("ceb", "Okres Olomouc"), ("cs", "okres Olomouc"), ("de", "Okres Olomouc"), ("en", "Olomouc"), ("es", "Distrito de Olomouc"), ("eu", "Olomouceko barrutia"), ("fa", "شهرستان اولوموتس"), ("fr", "District d’Olomouc"), ("he", "מחוז משנה אולומואוץ"), ("hu", "Olomouci járás"), ("hy", "Օլոմոուցի շրջան"), ("it", "Distretto di Olomouc"), ("ka", "ოლომოუცის რაიონი"), ("lt", "Olomouco rajonas"), ("lv", "Olomoucas apriņķis"), ("ms", "Daerah Olomouc²"), ("nl", "Okres Olomouc"), ("pl", "Powiat Ołomuniec"), ("pt", "Olomouc²"), ("ru", "Оломоуц"), ("sk", "Okres Olomouc"), ("sr", "Округ Оломоуц"), ("sr_Latn", "Okrug Olomouc"), ("sv", "Olomouc²"), ("tr", "Olomouc ilçesi"), ("uk", "Оломоуц"), ("ur", "اولوموک ضلع"), ("vi", "Olomouc²"), ("zh", "奧洛莫烏茨縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "713",
                    Subdivision{
                        name: "713",
                        country_alpha2: Alpha2::CZ,
                        code: "713",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بروستيوف"), ("be", "Просцеёў"), ("bg", "Простейов"), ("ca", "Districte de Prostějov"), ("ccp", "𑄛\u{11133}𑄢\u{1112e}𑄌\u{11134}𑄑𑄬𑄎\u{1112e}𑄛\u{11134}"), ("ceb", "Okres Prostějov"), ("cs", "okres Prostějov"), ("de", "Okres Prostějov"), ("en", "Prostějov"), ("es", "Distrito de Prostějov"), ("eu", "Prostějoveko barrutia"), ("fa", "شهرستان پروستییوف"), ("fr", "district de Prostějov"), ("he", "מחוז משנה פרוסטיוב"), ("hu", "Prostějovi járás"), ("hy", "Պրոստեյովի շրջան"), ("it", "Distretto di Prostějov"), ("ja", "プロスチェヨフ郡"), ("ka", "პროსტეჟოვის რაიონი"), ("lt", "Prostejovo rajonas"), ("lv", "Prostejovas apriņķis"), ("ms", "Daerah Prostějov"), ("nl", "Okres Prostějov"), ("pl", "Powiat Prościejów"), ("pt", "Prostějov"), ("ru", "Простеёв"), ("sk", "Okres Prostějov"), ("sr", "Округ Простјејов"), ("sr_Latn", "Okrug Prostjejov"), ("sv", "Prostějov"), ("tr", "Prostějov ilçesi"), ("uk", "Простейов"), ("ur", "پروستیو ضلع"), ("vi", "Prostějov"), ("zh", "普羅斯捷約夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "714",
                    Subdivision{
                        name: "714",
                        country_alpha2: Alpha2::CZ,
                        code: "714",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برشيروف"), ("be", "раён Пршэраў"), ("bg", "Пршеров"), ("ca", "Districte de Přerov"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄢\u{11127}𑄛\u{11134}"), ("ceb", "Okres Přerov"), ("cs", "okres Přerov"), ("de", "Okres Přerov"), ("en", "Přerov"), ("es", "Distrito de Přerov"), ("fa", "شهرستان پرژروف"), ("fr", "District de Přerov"), ("he", "מחוז משנה פררוב"), ("hu", "Přerovi járás"), ("hy", "Պշերովի շրջան"), ("it", "Distretto di Přerov"), ("ja", "プルジェロフ郡"), ("ka", "პრშეროვის რაიონი"), ("lt", "Pršerovo rajonas"), ("lv", "Pršerovas apriņķis"), ("ms", "Daerah Přerov"), ("nl", "Okres Přerov"), ("pl", "Powiat Przerów"), ("pt", "Přerov"), ("ru", "Пршеров"), ("sk", "Okres Přerov"), ("sr", "Округ Преров"), ("sr_Latn", "Okrug Prerov"), ("sv", "Přerov"), ("tr", "Přerov ilçesi"), ("uk", "Пршеров"), ("ur", "پژیرو ضلع"), ("vi", "Přerov"), ("zh", "普熱羅夫縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "715",
                    Subdivision{
                        name: "715",
                        country_alpha2: Alpha2::CZ,
                        code: "715",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شومبيرك"), ("bg", "Шумперк"), ("ca", "Districte de Šumperk"), ("ccp", "𑄥𑄟\u{11134}𑄛𑄢\u{11133}𑄇\u{11134}"), ("ceb", "Okres Šumperk"), ("cs", "okres Šumperk"), ("de", "Okres Šumperk"), ("en", "Šumperk"), ("es", "Distrito de Šumperk"), ("fa", "شهرستان شومپرک"), ("fr", "District de Šumperk"), ("he", "מחוז משנה סומפרק"), ("hu", "Šumperki járás"), ("hy", "Շումպերկի շրջան"), ("it", "Distretto di Šumperk"), ("ka", "შუმპერკის რაიონი"), ("lt", "Šumperko rajonas"), ("lv", "Šumperkas apriņķis"), ("ms", "Daerah Šumperk"), ("nl", "Okres Šumperk"), ("pl", "Powiat Šumperk"), ("pt", "Šumperk (distrito)"), ("ru", "Шумперк"), ("sk", "Okres Šumperk"), ("sr", "Шумперски округ"), ("sr_Latn", "Šumperski okrug"), ("sv", "Šumperk"), ("tr", "Šumperk İlçesi"), ("uk", "Шумперк"), ("ur", "شومپرک ضلع"), ("vi", "Šumperk"), ("zh", "順佩爾克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "72",
                    Subdivision{
                        name: "72",
                        country_alpha2: Alpha2::CZ,
                        code: "72",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.2162296), longitude: Some(17.7720353), max_latitude: Some(49.5399786), min_latitude: Some(48.8542302), max_longitude: Some(18.4155308), min_longitude: Some(17.1108595)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم زلين"), ("az", "Zlin diyarı"), ("be", "Злінскі край"), ("bg", "Злински край"), ("bn", "জিন অঞ\u{9cd}চল"), ("bs", "Zlinski kraj"), ("ca", "Regió de Zlín"), ("ccp", "𑄎\u{11133}𑄣\u{11128}𑄚\u{11134}𑄇\u{11128}"), ("ceb", "Zlínský kraj"), ("cs", "Zlínský kraj"), ("da", "Zlín"), ("de", "Zlínský kraj"), ("el", "Περιφέρεια Ζλιν"), ("en", "Zlínský"), ("es", "Región de Zlín"), ("et", "Zlíni maakond"), ("eu", "Zlín eskualdea"), ("fa", "منطقه زلین"), ("fi", "Zlínin lääni"), ("fr", "Région de Zlín"), ("gu", "ઝ\u{acd}લીન પ\u{acd}રદ\u{ac7}શ"), ("he", "זלין (מחוז)"), ("hi", "ज\u{93c}लिन प\u{94d}रद\u{947}श"), ("hr", "Zlínski kraj"), ("hu", "Zlíni kerület"), ("hy", "Զլինի երկրամաս"), ("id", "Daerah Zlín"), ("it", "regione di Zlín"), ("ja", "ズリーン州"), ("ka", "ზლინის მხარე"), ("kn", "ಝ\u{ccd}ಲ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "즐린 주"), ("lt", "Zlyno kraštas"), ("lv", "Zlīnas apgabals"), ("mk", "Злински крај"), ("mr", "झ\u{94d}लिन प\u{94d}रद\u{947}श"), ("ms", "Daerah Zlín"), ("nb", "Zlín"), ("nl", "Zlín"), ("no", "Zlín"), ("pl", "Kraj zliński"), ("pt", "Zlín"), ("ro", "Regiunea Zlín"), ("ru", "Злинский край"), ("si", "ස\u{dca}ල\u{dd2}න\u{dca} කල\u{dcf}පය"), ("sk", "Zlínsky kraj"), ("sr", "Злински крај"), ("sr_Latn", "Zlinski kraj"), ("sv", "Zlín"), ("ta", "ஜில\u{bcd}ன\u{bcd} பகுதி"), ("te", "జ\u{c4d}ల\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซล\u{e34}น"), ("tr", "Zlin Bölgesi"), ("uk", "Злінський край"), ("ur", "زلین علاقہ"), ("vi", "Zlín"), ("zh", "兹林州")]),
                        unofficial_name_list: ["Zlínský kraj"].to_vec(),
                    }
                ),
                (
                    "721",
                    Subdivision{
                        name: "721",
                        country_alpha2: Alpha2::CZ,
                        code: "721",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كروميشريش"), ("be", "Кромержыж"), ("bg", "Кромержиж (окръг)"), ("ca", "Districte de Kroměříž"), ("ccp", "𑄇\u{11133}𑄢\u{1112e}𑄟𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Okres Kroměříž"), ("cs", "okres Kroměříž"), ("de", "Okres Kroměříž"), ("en", "Kroměříž"), ("es", "Distrito de Kroměříž"), ("eu", "Kroměřížko barrutia"), ("fa", "شهرستان کرومیه\u{200c}رژیژ"), ("fr", "District de Kroměříž"), ("he", "מחוז משנה קרומניירי"), ("hu", "Kroměříži járás"), ("hy", "Կրոմերիժի շրջան"), ("it", "Distretto di Kroměříž"), ("ja", "クロミェジージュ郡"), ("ka", "კრომერჟიჟის რაიონი"), ("lt", "Kromeržyžo rajonas"), ("lv", "Kromeržīžas apriņķis"), ("ms", "Daerah Kroměříž"), ("nl", "Okres Kroměříž"), ("pl", "Powiat Kromieryż"), ("pt", "Kroměříž (distrito)"), ("ru", "Кромержиж"), ("sk", "Okres Kroměříž"), ("sr", "Округ Кромјержиж"), ("sr_Latn", "Okrug Kromjeržiž"), ("sv", "Okres Kroměříž"), ("tr", "Kroměříž ilçesi"), ("uk", "Кромержиж (округ)"), ("ur", "کرومیرشیزش ضلع"), ("vi", "Kroměříž"), ("zh", "克羅梅日什縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "722",
                    Subdivision{
                        name: "722",
                        country_alpha2: Alpha2::CZ,
                        code: "722",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوهرسكه هراديشتيه"), ("bg", "Ухерске Храдище"), ("ca", "Districte d’Uherské Hradiště"), ("ccp", "𑄅\u{1112a}𑄦𑄬𑄢\u{11134}𑄇𑄬 𑄦\u{11133}𑄢𑄓\u{11128}𑄌\u{11134}𑄑𑄬"), ("ceb", "Okres Uherské Hradiště"), ("cs", "okres Uherské Hradiště"), ("de", "Okres Uherské Hradiště"), ("en", "Uherské Hradiště"), ("es", "Distrito de Uherské Hradiště"), ("eu", "Uherské Hradištěko barrutia"), ("fa", "ناحیه اوهرسکه هرادیشتی"), ("fr", "District d’Uherské Hradiště"), ("he", "מחוז משנה אוהרסקי הראדיץ׳"), ("hu", "Uherské Hradiště-i járás"), ("hy", "Ուհարսյե Հրադանիշտեի շրջան"), ("it", "Distretto di Uherské Hradiště"), ("ka", "უჰერსკე-ჰრადიშტეს რაიონი"), ("lt", "Uherske Hradištės rajonas"), ("lv", "Uherske Hradištes apriņķis"), ("ms", "Daerah Uherské Hradiště"), ("nl", "Okres Uherské Hradiště"), ("pl", "Powiat Uherské Hradiště"), ("pt", "Uherské Hradiště (distrito)"), ("ru", "Угерске-Градиште"), ("sk", "Okres Uherské Hradiště"), ("sr", "Округ Ухерско Храдиште"), ("sr_Latn", "Okrug Uhersko Hradište"), ("sv", "Okres Uherské Hradiště"), ("tr", "Uherské Hradiště ilçesi"), ("uk", "Угерске Градіште"), ("ur", "اوہیرسکے ہاردیشتے ضلع"), ("vi", "Uherské Hradiště"), ("zh", "烏赫爾堡縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "723",
                    Subdivision{
                        name: "723",
                        country_alpha2: Alpha2::CZ,
                        code: "723",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فسيتين"), ("be", "раён Усецін"), ("bg", "Всетин"), ("ca", "Districte de Vsetín"), ("ccp", "𑄞\u{11128}𑄥𑄬𑄑\u{11128}𑄚\u{11134}"), ("ceb", "Okres Vsetín"), ("cs", "okres Vsetín"), ("de", "Okres Vsetín"), ("en", "Vsetín"), ("es", "Distrito de Vsetín"), ("fa", "شهرستان وستین"), ("fr", "District de Vsetín"), ("he", "מחוז משנה וסטין"), ("hu", "Vsetíni járás"), ("hy", "Վսետինի շրջան"), ("it", "Distretto di Vsetín"), ("ja", "フセチーン郡"), ("ka", "ვსეტინის რაიონი"), ("lt", "Vsetyno rajonas"), ("lv", "Vsetīnas apriņķis"), ("ms", "Daerah Vsetín"), ("nl", "Okres Vsetín"), ("pl", "Powiat Vsetín"), ("ru", "Всетин"), ("sk", "Okres Vsetín"), ("sr", "Округ Всетин"), ("sr_Latn", "Okrug Vsetin"), ("sv", "Okres Vsetín"), ("tr", "Vsetín ilçesi"), ("uk", "Всетін"), ("ur", "وسیتین ضلع"), ("vi", "Vsetín"), ("zh", "弗塞廷縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "724",
                    Subdivision{
                        name: "724",
                        country_alpha2: Alpha2::CZ,
                        code: "724",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زلين"), ("be", "Злін"), ("bg", "Злин"), ("ca", "Districte de Zlín"), ("ccp", "𑄎\u{11133}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Okres Zlín"), ("cs", "okres Zlín"), ("de", "Okres Zlín"), ("en", "Zlín"), ("es", "Distrito de Zlín"), ("fa", "شهرستان زلین"), ("fr", "District de Zlín"), ("he", "מחוז משנה זלין"), ("hu", "Zlíni járás"), ("hy", "Զլինիի շրջան"), ("it", "Distretto di Zlín"), ("ja", "ズリン"), ("ka", "ზლინის რაიონი"), ("lt", "Zlyno rajonas"), ("lv", "Zlīnas apriņķis"), ("ms", "Daerah Zlín²"), ("nl", "Okres Zlín"), ("pl", "Powiat Zlin"), ("pt", "Zlín (distrito)"), ("ro", "Districtul Zlín"), ("ru", "Злин"), ("sk", "Okres Zlín"), ("sr", "Округ Злин"), ("sr_Latn", "Okrug Zlin"), ("sv", "Okres Zlín"), ("tr", "Zlín ilçesi"), ("uk", "Злін"), ("ur", "زلین ضلع"), ("vi", "Zlín²"), ("zh", "茲林縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "80",
                    Subdivision{
                        name: "80",
                        country_alpha2: Alpha2::CZ,
                        code: "80",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.7305327), longitude: Some(18.2332637), max_latitude: Some(50.3279579), min_latitude: Some(49.392608), max_longitude: Some(18.8592548), min_longitude: Some(17.1462629)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مورافيا-سيليزيا"), ("az", "Moraviya-Sileziya diyarı"), ("be", "Мараўскасілезскі край"), ("bg", "Моравско-силезки край"), ("bn", "ম\u{9c1}র\u{9be}ভিয\u{9bc}\u{9be}ন অঞ\u{9cd}চল"), ("bs", "Moravskošleski kraj"), ("ca", "Regió de Moràvia i Silèsia"), ("ccp", "𑄟\u{11127}𑄢\u{11127}𑄛\u{11134}𑄇\u{1112e}𑄌\u{11134}𑄣𑄬𑄌\u{11134}𑄇\u{11128}"), ("ceb", "Moravskoslezský kraj"), ("cs", "Moravskoslezský kraj"), ("da", "Mähren-Schlesien"), ("de", "Moravskoslezský kraj"), ("el", "Περιφέρεια Μοραβίας Σιλεσίας"), ("en", "Moravskoslezský"), ("es", "Región de Moravia-Silesia"), ("et", "Morava-Sileesia maakond"), ("eu", "Moravia-Silesia eskualdea"), ("fa", "منطقه موراوی-سیلزی"), ("fi", "Määrin-Sleesian lääni"), ("fr", "Région de Moravie-Silésie"), ("gl", "Rexión de Moravia-Silesia"), ("gu", "મોરાવિયન-સિલ\u{ac7}સિઅન પ\u{acd}રદ\u{ac7}શ"), ("he", "מורביה-שלזיה (מחוז)"), ("hi", "मोरावियन-सिल\u{947}सियन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Moravsko-šleski kraj"), ("hu", "Morva-sziléziai kerület"), ("id", "Daerah Moravia-Silesia"), ("it", "regione di Moravia-Slesia"), ("ja", "モラヴィア・スレスコ州"), ("ka", "მორავია-სილეზიის მხარე"), ("kn", "ಮೊರಾವ\u{cbf}ಯನ\u{ccd}-ಸ\u{cbf}ಲೇಶ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "모라바슬레스코 주"), ("lt", "Moravijos-Silezijos kraštas"), ("lv", "Morāvijas-Silēzijas apgabals"), ("mk", "Моравскошлески крај"), ("mr", "मोरावियन-सिल\u{947}सियन प\u{94d}रद\u{947}श"), ("ms", "Daerah Moravia-Silesia"), ("nb", "Mähren-Schlesien region"), ("nl", "Moravië-Silezië"), ("no", "Mähren-Schlesien region"), ("pl", "Kraj morawsko-śląski"), ("pt", "Morávia-Silésia"), ("ro", "Regiunea Moravia-Silezia"), ("ru", "Моравскосилезский край"), ("si", "මොරව\u{dd2}යන\u{dca}-ස\u{dd2}ලෙස\u{dd2}යන\u{dca} කල\u{dcf}පය"), ("sk", "Moravsko-sliezsky kraj"), ("sr", "Моравско-Шлески крај"), ("sr_Latn", "Moravsko-Šleski kraj"), ("sv", "Mähren-Schlesien"), ("ta", "மொர\u{bbe}வின\u{bcd} -சிலேசிய\u{bbe}ன\u{bcd} பகுதி"), ("te", "మ\u{c4b}ర\u{c3e}వ\u{c3f}యన\u{c4d}-స\u{c3f}ల\u{c47}స\u{c3f}యన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โมราเว\u{e35}ยน ซ\u{e35}เลเซ\u{e35}ยน"), ("tr", "Moravya-Silezya Bölgesi"), ("uk", "Мораво-Сілезький край"), ("ur", "موراویائی سیلیسیائی علاقہ"), ("vi", "Moravia–Silesia"), ("zh", "摩拉維亞-西里西亞州")]),
                        unofficial_name_list: ["Ostravský"].to_vec(),
                    }
                ),
                (
                    "801",
                    Subdivision{
                        name: "801",
                        country_alpha2: Alpha2::CZ,
                        code: "801",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برونتال"), ("bg", "Брунтал"), ("ca", "Districte de Bruntál"), ("ccp", "𑄝\u{11133}𑄢\u{1112a}𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Okres Bruntál"), ("cs", "okres Bruntál"), ("de", "Okres Bruntál"), ("en", "Bruntál"), ("es", "Distrito de Bruntál"), ("eu", "Bruntálgo barrutia"), ("fa", "شهرستان برونتال"), ("fr", "District de Bruntál"), ("he", "מחוז משנה ברונטל"), ("hu", "Bruntáli járás"), ("hy", "Բրունտալի շրջան"), ("it", "Distretto di Bruntál"), ("ka", "ბრუნტალის რაიონი"), ("lt", "Bruntalo rajonas"), ("lv", "Bruntālas apriņķis"), ("ms", "Daerah Bruntál"), ("nl", "Okres Bruntál"), ("pl", "Powiat Bruntál"), ("ru", "Брунталь (район)"), ("sk", "Okres Bruntál"), ("sr", "Округ Брунтал"), ("sr_Latn", "Okrug Bruntal"), ("sv", "Bruntál (distrikt)"), ("uk", "Округ Брунталь"), ("ur", "بروتال ضلع"), ("vi", "Bruntál"), ("zh", "布倫塔爾縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "802",
                    Subdivision{
                        name: "802",
                        country_alpha2: Alpha2::CZ,
                        code: "802",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فريدك-ميستك"), ("bg", "Фридек-Мистек"), ("ca", "Districte de Frýdek-Místek"), ("ccp", "𑄜\u{11133}𑄢\u{1112d}𑄓𑄬𑄇\u{11134}-𑄟\u{11128}𑄌\u{11134}𑄑𑄬𑄇\u{11134}"), ("ceb", "Okres Frýdek-Místek"), ("cs", "okres Frýdek-Místek"), ("de", "Okres Frýdek-Místek"), ("en", "Frýdek-Místek"), ("es", "Distrito de Frýdek-Místek"), ("eu", "Frýdek-Místekeko barrutia"), ("fa", "شهرستان فریدک-میستک"), ("fr", "District de Frýdek-Místek"), ("he", "מחוז משנה פרידק-מיסטק"), ("hu", "Frýdek-místeki járás"), ("hy", "Ֆրիդեկ-Միստեկի շրջան"), ("it", "Distretto di Frýdek-Místek"), ("ka", "ფრიდეკ-მისტეკის რაიონი"), ("lt", "Frydek Mysteko rajonas"), ("lv", "Frīdekmīstekas apriņķis"), ("ms", "Daerah Frýdek-Místek"), ("nl", "Okres Frýdek-Místek"), ("pl", "Powiat Frydek-Mistek"), ("pt", "Frýdek-Místek"), ("ro", "Okresul Frýdek-Místek"), ("ru", "Фридек-Мистек"), ("sk", "Okres Frýdek-Místek"), ("sr", "Округ Фридек-Мистек"), ("sr_Latn", "Okrug Fridek-Mistek"), ("sv", "Frýdek-Místek"), ("tr", "Frýdek-Místek ilçesi"), ("uk", "Фридек-Містек"), ("ur", "فریدک-میستک ضلع"), ("vi", "Frýdek-Místek"), ("zh", "弗里代克-米斯泰克縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "803",
                    Subdivision{
                        name: "803",
                        country_alpha2: Alpha2::CZ,
                        code: "803",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كارفينا"), ("be", "раён Карвіна"), ("bg", "Карвина"), ("ca", "Districte de Karviná"), ("ccp", "𑄇𑄢\u{11134}𑄞\u{11128}𑄚"), ("ceb", "Okres Karviná"), ("cs", "okres Karviná"), ("de", "Okres Karviná"), ("en", "Karviná"), ("es", "Distrito de Karviná"), ("eu", "Karvináko barrutia"), ("fa", "شهرستان کاروینا"), ("fr", "District de Karviná"), ("he", "מחוז משנה קרבינה"), ("hu", "Karvinái járás"), ("hy", "Կարվինայի շրջան"), ("it", "Distretto di Karviná"), ("ka", "კარვინის რაიონი"), ("lt", "Karvinos rajonas"), ("lv", "Karvinas apriņķis"), ("ms", "Daerah Karviná"), ("nl", "Okres Karviná"), ("pl", "Powiat Karwina"), ("ru", "Карвина"), ("sk", "Okres Karviná"), ("sr", "Округ Карвина"), ("sr_Latn", "Okrug Karvina"), ("sv", "Karviná"), ("tr", "Karviná ilçesi"), ("uk", "Карвіна"), ("ur", "کاروینا ضلع"), ("vi", "Karviná"), ("zh", "卡爾維納縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "804",
                    Subdivision{
                        name: "804",
                        country_alpha2: Alpha2::CZ,
                        code: "804",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نوفي يتشين"), ("be", "Новы-Йічын"), ("bg", "Нови Ичин"), ("ca", "Districte de Nový Jičín"), ("ccp", "𑄚\u{1112e}𑄞\u{11128} 𑄎\u{11128}𑄌\u{11128}𑄚\u{11134}"), ("ceb", "Okres Nový Jičín"), ("cs", "okres Nový Jičín"), ("de", "Okres Nový Jičín"), ("en", "Nový Jičín"), ("es", "Distrito de Nový Jičín"), ("eu", "Nový Jičíngo barrutia"), ("fa", "شهرستان نووی ییچین"), ("fr", "District de Nový Jičín"), ("he", "מחוז משנה נובי ייצין"), ("hu", "Nový Jičín-i járás"), ("hy", "Նովի Յիչինի շրջան"), ("it", "Distretto di Nový Jičín"), ("ka", "ნოვი-იიჩინის რაიონი"), ("lt", "Novy Jičyno rajonas"), ("lv", "Novi Jičīnas apriņķis"), ("ms", "Daerah Nový Jičín"), ("nl", "Okres Nový Jičín"), ("pl", "Powiat Nowy Jiczyn"), ("ro", "Okres Nový Jičín"), ("ru", "Нови-Йичин"), ("sk", "Okres Nový Jičín"), ("sr", "Округ Нови Јичин"), ("sr_Latn", "Okrug Novi Jičin"), ("sv", "Nový Jičín"), ("tr", "Nový Jičín ilçesi"), ("uk", "Новий Їчин"), ("ur", "نووی جیچن ضلع"), ("vi", "Nový Jičín"), ("zh", "新伊欽縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "805",
                    Subdivision{
                        name: "805",
                        country_alpha2: Alpha2::CZ,
                        code: "805",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوبافا"), ("be", "раён Опава"), ("bg", "Опава"), ("ca", "Districte d’Opava"), ("ccp", "𑄃\u{11127}𑄛𑄞"), ("ceb", "Okres Opava"), ("cs", "okres Opava"), ("de", "Okres Opava"), ("en", "Opava"), ("es", "Distrito de Opava"), ("fa", "شهرستان اوپاوا"), ("fr", "District d’Opava"), ("he", "מחוז משנה אופאבה"), ("hu", "Opavai járás"), ("hy", "Օպավայի շրջան"), ("it", "Distretto di Opava"), ("ka", "ოპავის რაიონი"), ("lt", "Opavos rajonas"), ("lv", "Opavas apriņķis"), ("ms", "Daerah Opava"), ("nl", "Okres Opava"), ("pl", "Powiat Opawa"), ("pt", "Opava"), ("ro", "Districtul Opava"), ("ru", "Опава"), ("sk", "Okres Opava"), ("sr", "Округ Опава"), ("sr_Latn", "Okrug Opava"), ("sv", "Opava"), ("tr", "Opava ilçesi"), ("uk", "Опава"), ("ur", "اوپاوا ضلع"), ("vi", "Opava"), ("zh", "奧帕瓦縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "806",
                    Subdivision{
                        name: "806",
                        country_alpha2: Alpha2::CZ,
                        code: "806",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوسترافا-المدينة"), ("be", "Острава-горад"), ("bg", "Острава-град"), ("ca", "Districte d’Ostrava-město"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢𑄞"), ("ceb", "Okres Ostrava-Město"), ("cs", "okres Ostrava-město"), ("de", "Okres Ostrava-město"), ("en", "Ostrava"), ("es", "Distrito de Ostrava"), ("fa", "شهرستان استراوا-میستو"), ("fr", "District d’Ostrava-město"), ("hu", "Ostrava városi járás"), ("hy", "Օստրավայի շրջան"), ("it", "Distretto di Ostrava"), ("lt", "Ostravos miesto rajonas"), ("lv", "Ostravas-pilsētas apriņķis"), ("ms", "Daerah Bandaraya Ostrava"), ("nl", "Okres Ostrava-město"), ("pl", "Powiat Ostrawa"), ("pt", "Ostrava"), ("ru", "Острава-город"), ("sk", "Okres Ostrava-mesto"), ("sr", "Округ Острава-град"), ("sr_Latn", "Okrug Ostrava-grad"), ("sv", "Ostravas stadsdistrikt"), ("uk", "Острава-місто"), ("ur", "اوستراوا-شہر ضلع"), ("vi", "Ostrava"), ("zh", "俄斯特拉發城縣")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "cz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CZ,
        alpha3: Alpha3::CZE,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 420,
        currency_code: "CZK",
        gec: Some(GEC::EZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CZE),
        iso_long_name: "The Czech Republic",
        iso_short_name: "Czechia",
        official_language_list: ["cs", "sk"].to_vec(),
        spoken_language_list: ["cs", "sk"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "None",
        nationality: Some("Czech"),
        number: "203",
        postal_code: true,
        postal_code_format: Some("\\d{3} ?\\d{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "CZ",
        unofficial_name_list: [
            "Czech Republic",
            "Tschechische Republik",
            "République Tchèque",
            "República Checa",
            "チェコ",
            "Tsjechië",
            "Czechia",
            "Česká republika",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Czechia"),
            ("af", "Tsjeggië"),
            ("ak", "Czechia"),
            ("am", "Czechia"),
            ("an", "Czechia"),
            ("ar", "التشيك"),
            ("as", "Czechia"),
            ("ay", "Czechia"),
            ("az", "Czechia"),
            ("ba", "Czechia"),
            ("be", "Чэхія"),
            ("bg", "Czechia"),
            ("bi", "Czechia"),
            ("bn", "চেকিয়\u{9be}"),
            ("bn_IN", "চেচিয\u{9bc}\u{9be}"),
            ("br", "Czechia"),
            ("bs", "Czechia"),
            ("ca", "Txèquia"),
            ("ce", "Czechia"),
            ("ch", "Czechia"),
            ("cs", "Česko"),
            ("cv", "Czechia"),
            ("cy", "Tsiecia"),
            ("da", "Tjekkiet"),
            ("de", "Tschechien"),
            ("dv", "Czechia"),
            ("dz", "Czechia"),
            ("ee", "Czechia"),
            ("el", "Τσεχία"),
            ("en", "Czechia"),
            ("eo", "Ĉeĥio"),
            ("es", "Chequia"),
            ("et", "Tšehhi"),
            ("eu", "Txekia"),
            ("fa", "چک"),
            ("ff", "Czechia"),
            ("fi", "Czechia"),
            ("fo", "Czechia"),
            ("fr", "Tchéquie"),
            ("fy", "Czechia"),
            ("ga", "Czechia"),
            ("gl", "Chequia"),
            ("gn", "Czechia"),
            ("gu", "ચ\u{ac7}કિયા"),
            ("gv", "Czechia"),
            ("ha", "Czechia"),
            ("he", "צ'כיה"),
            ("hi", "च\u{947}किया"),
            ("hr", "Češka"),
            ("ht", "Tchèki"),
            ("hu", "Csehország"),
            ("hy", "Czechia"),
            ("ia", "Cechia"),
            ("id", "Czechia"),
            ("io", "Czechia"),
            ("is", "Tékkland"),
            ("it", "Cechia"),
            ("iu", "Czechia"),
            ("ja", "Czechia"),
            ("ka", "Czechia"),
            ("ki", "Czechia"),
            ("kk", "Czechia"),
            ("kl", "Czechia"),
            ("km", "ឆេក"),
            ("kn", "Czechia"),
            ("ko", "체코"),
            ("ku", "Çekistan"),
            ("kv", "Czechia"),
            ("kw", "Czechia"),
            ("ky", "Чехия Республикасы"),
            ("lo", "Czechia"),
            ("lt", "Czechia"),
            ("lv", "Czechia"),
            ("mi", "Czechia"),
            ("mk", "Czechia"),
            ("ml", "Czechia"),
            ("mn", "Czechia"),
            ("mr", "झ\u{947}किया"),
            ("ms", "Czechia"),
            ("mt", "Czechia"),
            ("my", "Czechia"),
            ("na", "Czechia"),
            ("nb", "Tsjekkia"),
            ("ne", "Czechia"),
            ("nl", "Tsjechië"),
            ("nn", "Czechia"),
            ("nv", "Czechia"),
            ("oc", "Chequia"),
            ("or", "ଚେସ\u{b3f}ଆ"),
            ("pa", "ਚ\u{a47}ਚੀਆ"),
            ("pi", "Czechia"),
            ("pl", "Czechy"),
            ("ps", "Czechia"),
            ("pt", "República Checa"),
            ("pt_BR", "Chéquia"),
            ("ro", "Cehia"),
            ("ru", "Чехия"),
            ("rw", "Czechia"),
            ("sc", "Tzèchia"),
            ("sd", "Czechia"),
            ("si", "Czechia"),
            ("sk", "Česko"),
            ("sl", "Czechia"),
            ("so", "Czechia"),
            ("sq", "Çeki"),
            ("sr", "Чешка"),
            ("sv", "Tjeckien"),
            ("sw", "Czechia"),
            ("ta", "Czechia"),
            ("te", "Czechia"),
            ("tg", "Чехия"),
            ("th", "ประเทศเช\u{e47}กเก\u{e35}ย"),
            ("ti", "Czechia"),
            ("tk", "Czechia"),
            ("tl", "Czechia"),
            ("tr", "Çekya"),
            ("tt", "Czechia"),
            ("ug", "چېخ"),
            ("uk", "Чехія"),
            ("ur", "Czechia"),
            ("uz", "Czechia"),
            ("ve", "Czechia"),
            ("vi", "Czechia"),
            ("wa", "Czechia"),
            ("wo", "Czechia"),
            ("xh", "Czechia"),
            ("yo", "Czechia"),
            ("zh_CN", "捷克"),
            ("zh_HK", "捷克"),
            ("zh_TW", "捷克"),
            ("zu", "Czechia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
