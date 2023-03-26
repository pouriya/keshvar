// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Uzbekistan

#[cfg(all(feature = "uz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::UZ;
    pub const ALPHA3: Alpha3 = Alpha3::UZB;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 998;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::UZS;
    pub const GEC: Option<GEC> = Some(GEC::UZ);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::UZB);
    pub const ISO_SHORT_NAME: &str = "Uzbekistan";
    pub const ISO_LONG_NAME: &str = "The Republic of Uzbekistan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ru", "uz"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ru", "uz"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Uzbekistani");
    pub const NUMBER: &str = "860";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAsia);
    pub const UN_LOCODE: &str = "UZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Uzbekistan",
        "Usbekistan",
        "Ouzbékistan",
        "Uzbekistán",
        "ウズベキスタン",
        "Oezbekistan",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Uzbekistan"),
        ("af", "Oesbekistan"),
        ("ak", "Uzbekistan"),
        ("am", "ፙፔበጡስታን"),
        ("an", "Uzbekistan"),
        ("ar", "أوزبكستان"),
        ("as", "উজ\u{9cd}বেকিস\u{9cd}ত\u{9be}ন"),
        ("ay", "Uzbekistan"),
        ("az", "Özbəkistan"),
        ("ba", "Uzbekistan"),
        ("be", "Узбекістан"),
        ("bg", "Узбекистан"),
        ("bi", "Uzbekistan"),
        ("bn", "উজবেকিস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "উজবেকিস\u{9cd}ত\u{9be}ন"),
        ("br", "Ouzbekistan"),
        ("bs", "Uzbekistan"),
        ("ca", "Uzbekistan"),
        ("ce", "Узбекистан"),
        ("ch", "Uzbekistan"),
        ("cs", "Uzbekistán"),
        ("cv", "Узбекистан"),
        ("cy", "Wsbecistan"),
        ("da", "Usbekistan"),
        ("de", "Usbekistan"),
        (
            "dv",
            "އ\u{7aa}ޒ\u{7b0}ބ\u{7ac}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}",
        ),
        ("dz", "ཨ\u{f74}ཛ་བ\u{f7a}་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Uzbekistan"),
        ("el", "Ουζμπεκιστάν"),
        ("en", "Uzbekistan"),
        ("eo", "Uzbekio"),
        ("es", "Uzbekistán"),
        ("et", "Usbekistan"),
        ("eu", "Uzbekistan"),
        ("fa", "ازبکستان"),
        ("ff", "Uzbekistan"),
        ("fi", "Uzbekistan"),
        ("fo", "Usbekistan"),
        ("fr", "Ouzbékistan"),
        ("fy", "Oezbekistan"),
        ("ga", "Úisbéiceastáin"),
        ("gl", "Uzbequistán"),
        ("gn", "Uzbekistan"),
        ("gu", "ઉઝબ\u{ac7}કિસ\u{acd}તાન"),
        ("gv", "Yn Oosbeckistaan"),
        ("ha", "Uzbekistan"),
        ("he", "אוזבקיסטן"),
        ("hi", "उज\u{93c}\u{94d}ब\u{947}किस\u{94d}तान"),
        ("hr", "Uzbekistan"),
        ("ht", "Ouzbekistan"),
        ("hu", "Üzbegisztán"),
        ("hy", "Ուզբեկստան"),
        ("ia", "Uzbekistan"),
        ("id", "Uzbekistan"),
        ("io", "Uzbekistan"),
        ("is", "Úsbekistan"),
        ("it", "Uzbekistan"),
        ("iu", "Uzbekistan"),
        ("ja", "ウズベキスタン"),
        ("ka", "უზბაკეთი"),
        ("ki", "Uzbekistan"),
        ("kk", "Өзбекстан"),
        ("kl", "Uzbekistan"),
        (
            "km",
            "អ\u{17ca}\u{17bc}ហ\u{17d2}សបេគ\u{17b8}ស\u{17d2}តង\u{17cb}",
        ),
        ("kn", "ಉಜ\u{ccd}ಬ\u{cc6}ಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
        ("ko", "우즈베키스탄"),
        ("ku", "Ozbekîstan"),
        ("kv", "Узбекистан"),
        ("kw", "Pow Ousbek"),
        ("ky", "Өзбекстан"),
        ("lo", "Uzbekistan"),
        ("lt", "Uzbekija"),
        ("lv", "Uzbekistāna"),
        ("mi", "Uhipeketāne"),
        ("mk", "Узбекистан"),
        (
            "ml",
            "ഉസ\u{d4d}ബേക\u{d4d}കിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}",
        ),
        ("mn", "узбекстан"),
        ("mr", "उझब\u{947}गिस\u{94d}तान"),
        ("ms", "Uzbekistan"),
        ("mt", "Użbekistan"),
        (
            "my",
            "ဥဇဘက\u{103a}ကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Uzbekistan"),
        ("nb", "Usbekistan"),
        ("ne", "उज\u{94d}ब\u{947}किस\u{94d}तान"),
        ("nl", "Oezbekistan"),
        ("nn", "Usbekistan"),
        ("nv", "Uzbekistan"),
        ("oc", "Ozbequistan"),
        ("or", "ଉଜବେକ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਉਜ਼\u{a47}ਬਕਸਤਾਨ"),
        ("pi", "उजब\u{947}किस\u{94d}थान"),
        ("pl", "Uzbekistan"),
        ("ps", "اوزبکستان"),
        ("pt", "Uzbequistão"),
        ("pt_BR", "Uzbequistão"),
        ("ro", "Uzbekistan"),
        ("ru", "Узбекистан"),
        ("rw", "Uzubekisitani"),
        ("sc", "Uzbèkistan"),
        ("sd", "ازبڪستان"),
        ("si", "උස\u{dca}බෙක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
        ("sk", "Uzbekistan"),
        ("sl", "Uzbekistan"),
        ("so", "Uzbekistan"),
        ("sq", "Uzbekistan"),
        ("sr", "Узбекистан"),
        ("sv", "Uzbekistan"),
        ("sw", "Uzbekistan"),
        ("ta", "உஸ\u{bcd}பெகிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
        ("te", "ఉజ\u{c4d}బ\u{c46}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Ӯзбекистон"),
        ("th", "อ\u{e38}ซเบก\u{e34}สถาน"),
        ("ti", "ኡዝበኪስታን"),
        ("tk", "Özbegistan"),
        ("tl", "Uzbekistan"),
        ("tr", "Özbekistan"),
        ("tt", "Өзбәкстан"),
        ("ug", "ئۆزبېكىستان"),
        ("uk", "Узбекистан"),
        ("ur", "ازبکستان"),
        ("uz", "Oʻzbekiston"),
        ("ve", "Uzbekistan"),
        ("vi", "U-xợ-bê-khi-xtanh"),
        ("wa", "Ouzbekistan"),
        ("wo", "Usbekistaan"),
        ("xh", "Uzbekistan"),
        ("yo", "Ùsbẹ\u{300}kìstán"),
        ("zh_CN", "乌兹别克斯坦"),
        ("zh_HK", "烏茲別克"),
        ("zh_TW", "烏茲別克"),
        ("zu", "Uzbekistan"),
    ];
    #[cfg(all(feature = "uz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 41.377491;
        pub const LONGITUDE: f64 = 64.585262;
        pub const MAX_LATITUDE: f64 = 45.590075;
        pub const MAX_LONGITUDE: f64 = 73.148946;
        pub const MIN_LATITUDE: f64 = 37.1722571;
        pub const MIN_LONGITUDE: f64 = 55.9982179;
        pub const NORTHEAST_LATITUDE: f64 = 45.590075;
        pub const NORTHEAST_LONGITUDE: f64 = 73.148946;
        pub const SOUTHWEST_LATITUDE: f64 = 37.1722571;
        pub const SOUTHWEST_LONGITUDE: f64 = 55.9982179;
    }
}
#[cfg(all(feature = "uz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 41.377491,
            longitude: 64.585262,
            max_latitude: 45.590075,
            max_longitude: 73.148946,
            min_latitude: 37.1722571,
            min_longitude: 55.9982179,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 45.590075,
                    longitude: 73.148946,
                },
                southwest: CountryGeoBound {
                    latitude: 37.1722571,
                    longitude: 55.9982179,
                },
            },
        }
    }
}

#[cfg(all(feature = "uz", feature = "subdivisions"))]
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
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::UZ,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.8153561), longitude: Some(72.28375), max_latitude: Some(40.8773096), min_latitude: Some(40.6814191), max_longitude: Some(72.4216927), min_longitude: Some(72.2175981)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أنديجان"), ("az", "Əndican vilayəti"), ("be", "Андыжанскі вілает"), ("bg", "Андижанска област"), ("bn", "আন\u{9cd}ডিজ\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Província d’Andijan"), ("ccp", "𑄃𑄚\u{11134}𑄓\u{11128}𑄎𑄚\u{11134}"), ("da", "Andijan Region"), ("de", "Provinz Andijon"), ("el", "Αντιτζάν"), ("en", "Andijan"), ("es", "Provincia de Andillán"), ("et", "Andijoni vilajett"), ("eu", "Andijan probintzia"), ("fa", "استان اندیجان"), ("fi", "Andižanin alue"), ("fr", "province d’Andijan"), ("gl", "Provincia de Andijon"), ("gu", "એન\u{acd}ડિજાન પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז אנדיז׳ן"), ("hi", "अन\u{94d}दीझ\u{93c}ान प\u{94d}रान\u{94d}त"), ("id", "Provinsi Andijan"), ("it", "Regione di Andijan"), ("ja", "アンディジャン州"), ("jv", "Andijon"), ("kk", "Андижан облысы"), ("kn", "ಆಂಡ\u{cbf}ಜನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "안디잔 주"), ("ky", "Андижан облусу"), ("lt", "Andižano vilajetas"), ("lv", "Andidžonas vilojats"), ("mk", "Андиџанска област"), ("mr", "अ\u{902}दिजोन विलायती"), ("ms", "Wilayah Andijan"), ("nb", "Andijan region"), ("nl", "Andizan"), ("no", "Andijan region"), ("pa", "ਅ\u{a70}ਦੀਜਾਨ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Wilajet andiżański"), ("pt", "Andijan"), ("ru", "Андижанская область"), ("si", "අන\u{dca}ඩ\u{dd2}ජන\u{dca} කල\u{dcf}පය"), ("sv", "Andizjan"), ("ta", "ஆண\u{bcd}டிஜன\u{bcd} பகுதி"), ("te", "ఆండ\u{c3f}జ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แอนด\u{e34}จาน"), ("tr", "Andican ili"), ("uk", "Андижанська область"), ("ur", "اندیجان صوبہ"), ("uz", "Andijon"), ("vi", "Andijon"), ("zh", "安集延州")]),
                        unofficial_name_list: ["Andijon", "Andizhan", "Andižan"].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::UZ,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7680827), longitude: Some(64.4555769), max_latitude: Some(39.8241068), min_latitude: Some(39.7309658), max_longitude: Some(64.5017624), min_longitude: Some(64.3438338)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بخارى"), ("az", "Buxara vilayəti"), ("bg", "Бухарска област"), ("bn", "ব\u{9c1}কহ\u{9be}র\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄝\u{1112a}𑄈𑄢"), ("ceb", "Bukhara Province"), ("da", "Bukhara Region"), ("de", "Provinz Buxoro"), ("el", "Μπουχάρα"), ("en", "Bukhara"), ("es", "Provincia de Bujara"), ("et", "Buxoro vilajett"), ("eu", "Bukhara probintzia"), ("fa", "استان بخارا"), ("fi", "Buharan alue"), ("fr", "province de Boukhara"), ("gl", "Provincia de Buxoro"), ("gu", "બ\u{ac1}ખારા પ\u{acd}રદ\u{ac7}શ"), ("hi", "ब\u{941}ख\u{93c}ारा प\u{94d}रान\u{94d}त"), ("hy", "Բուխարայի մարզ"), ("id", "Provinsi Buxoro"), ("it", "Regione di Bukhara"), ("ja", "ブハラ州"), ("jv", "Buxoro"), ("kk", "Бұхара облысы"), ("kn", "ಬುಖರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "부하라 주"), ("ky", "Бухара облусу"), ("lt", "Bucharos vilajetas"), ("lv", "Buhāras vilojats"), ("mk", "Бухарска област"), ("mr", "ब\u{941}झोरो विलायती"), ("ms", "Wilayah Buxoro"), ("nb", "Bukhara region"), ("nl", "Buxoro"), ("no", "Bukhara region"), ("pa", "ਬ\u{a41}ਖਾਰਾ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Wilajet bucharski"), ("pt", "Bukhara"), ("ru", "Бухарская область"), ("si", "බ\u{dd4}ක\u{dca}හර\u{dcf} කල\u{dcf}පය"), ("sv", "Buchara"), ("ta", "புக\u{bbe}ர\u{bbe} பகுதி"), ("te", "బుఖ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "บ\u{e39}โคโร"), ("tr", "Buhara ili"), ("uk", "Бухарська область"), ("ur", "بخارا صوبہ"), ("uz", "Buxoro viloyati"), ("vi", "Buxoro"), ("zh", "布哈拉州")]),
                        unofficial_name_list: ["Boukhara", "Buchara", "Buhara", "Buhoro", "Bukhara", "Bukhoro"].to_vec(),
                    }
                ),
                (
                    "FA",
                    Subdivision{
                        name: "FA",
                        country_alpha2: Alpha2::UZ,
                        code: "FA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.386389), longitude: Some(71.786389), max_latitude: Some(40.4268914), min_latitude: Some(40.3326582), max_longitude: Some(71.8614006), min_longitude: Some(71.7195224)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية فرغانة"), ("az", "Fərqanə vilayəti"), ("be", "Ферганскі вілает"), ("bg", "Ферганска област"), ("bn", "ফেরগ\u{9be}ন\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄜𑄢\u{11134}𑄉𑄚"), ("ceb", "Fergana"), ("da", "Fergana Region"), ("de", "Provinz Fargʻona"), ("el", "Φεργκάνα"), ("en", "Fergana"), ("es", "Provincia de Fergana"), ("eu", "Fergana probintzia"), ("fa", "استان فرغانه"), ("fi", "Ferganan alue"), ("fr", "province de Ferghana"), ("gl", "Provincia de Fergana"), ("gu", "ફ\u{ac7}રગના પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז פרגנה"), ("hi", "फ\u{93c}रग\u{93c}ना प\u{94d}रान\u{94d}त"), ("id", "Provinsi Fergana"), ("it", "Regione di Fergana"), ("ja", "フェルガナ州"), ("ka", "ფერღანის ოლქი"), ("kn", "ಫ\u{cc6}ರ\u{ccd}ಗಾನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "페르가나 주"), ("ky", "Фергана облусу"), ("lt", "Ferganos vilajetas"), ("lv", "Fergānas vilojats"), ("mk", "Ферганска област"), ("mr", "फर\u{94d}गोना विलायती"), ("ms", "Wilayah Fergana"), ("nb", "Fergana region"), ("nl", "Fargʻona"), ("no", "Fergana region"), ("pa", "ਫ\u{a3c}ਰਗਨਾ ਖ\u{a47}ਤਰ"), ("pl", "Wilajet fergański"), ("pt", "Fergana"), ("ro", "Regiunea Fergana"), ("ru", "Ферганская область"), ("si", "ෆෙර\u{dca}ගන\u{dcf} කල\u{dcf}පය"), ("sv", "Fergana"), ("ta", "பெர\u{bcd}கன\u{bbe} பகுதி"), ("te", "ఫ\u{c46}ర\u{c4d}గ\u{c3e}న\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ลาก\u{e39}เนส"), ("tr", "Fergana ili"), ("uk", "Ферганська область"), ("ur", "فرغانہ صوبہ"), ("uz", "Fargʻona viloyati"), ("vi", "Farg’ona"), ("zh", "費爾干納州")]),
                        unofficial_name_list: ["Farghona", "Fergana", "Ferghana", "Ferghona"].to_vec(),
                    }
                ),
                (
                    "JI",
                    Subdivision{
                        name: "JI",
                        country_alpha2: Alpha2::UZ,
                        code: "JI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.11583299999999), longitude: Some(67.84222199999999), max_latitude: Some(40.1917254), min_latitude: Some(40.0458832), max_longitude: Some(67.9324151), min_longitude: Some(67.7763748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية جيزك"), ("az", "Cizzəx vilayəti"), ("be", "Джызакская вобласць"), ("bg", "Джизакска област"), ("bn", "জিজ\u{9be}ক অঞ\u{9cd}চল"), ("ccp", "𑄎\u{11128}𑄎\u{11133}𑄦𑄇\u{11134}"), ("ceb", "Jizzakh Province"), ("da", "Jizzakh Region"), ("de", "Provinz Jizzax"), ("el", "Επαρχία Τζιζάκ"), ("en", "Jizzakh"), ("es", "Provincia de Djizaks"), ("et", "Jizzaxi vilajett"), ("fa", "استان جیزک"), ("fi", "Jizzaxin alue"), ("fr", "province de Djizak"), ("gl", "Provincia de Jizzax"), ("gu", "જીઝાખ પ\u{acd}રદ\u{ac7}શ"), ("hi", "जिज\u{93c}ाख\u{93c} प\u{94d}रान\u{94d}त"), ("id", "Provinsi Jizzakh"), ("it", "Regione di Djizak"), ("ja", "ジザフ州"), ("kn", "ಜ\u{cbf}ಝಾಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "지자흐 주"), ("ky", "Жызак облусу"), ("lt", "Džizako vilajetas"), ("lv", "Džizahas vilojats"), ("mk", "Џизачка област"), ("mr", "जिझाक\u{94d}स विलायती"), ("ms", "Daerah Jizzakh"), ("nb", "Jizzakh region"), ("nl", "Jizzax"), ("no", "Jizzakh region"), ("pa", "ਜਿਜ\u{a3c}ਾਖ ਸ\u{a42}ਬਾ"), ("pl", "Wilajet dżyzacki"), ("pt", "Jizzakh"), ("ru", "Джизакская область"), ("si", "ජ\u{dd2}ස\u{dcf}ක\u{dca} කල\u{dcf}පය"), ("sv", "Dzjizzach"), ("ta", "ஜிஸ\u{bcd}ச\u{bbe}க\u{bcd}ஹ\u{bcd} பகுதி"), ("te", "జ\u{c3f}జక\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตย\u{e34}ซซ\u{e31}ค"), ("tr", "Cizzak ili"), ("uk", "Джиззацька область"), ("ur", "جیزخ صوبہ"), ("uz", "Jizzax viloyati"), ("vi", "Jizzax"), ("zh", "吉扎克州")]),
                        unofficial_name_list: ["Cizah", "Cizak", "Djizak", "Dzhizak", "Džizak", "Jizzakh"].to_vec(),
                    }
                ),
                (
                    "NG",
                    Subdivision{
                        name: "NG",
                        country_alpha2: Alpha2::UZ,
                        code: "NG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.001111), longitude: Some(71.66833299999999), max_latitude: Some(41.0572407), min_latitude: Some(40.8836945), max_longitude: Some(71.7551683), min_longitude: Some(71.5340768)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نمنغان"), ("az", "Namangan Vilayəti"), ("be", "Наманганскі вілает"), ("bg", "Наманганска област"), ("bn", "ন\u{9be}ম\u{9be}নগ\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Província de Namangan"), ("ccp", "𑄚𑄟𑄚\u{11134}𑄉𑄚\u{11134}"), ("ceb", "Namangan Province"), ("da", "Namangan Region"), ("de", "Provinz Namangan"), ("el", "Νάμανγκαν"), ("en", "Namangan"), ("es", "Provincia de Namangán"), ("et", "Namangani vilajett"), ("fa", "استان نمنگان"), ("fi", "Namanganin alue"), ("fr", "province de Namangan"), ("gl", "Provincia de Namangan"), ("gu", "નામાનગન પ\u{acd}રદ\u{ac7}શ"), ("hi", "नमन\u{94d}गान प\u{94d}रान\u{94d}त"), ("id", "Provinsi Namangan"), ("it", "Regione di Namangan"), ("ja", "ナマンガン州"), ("kn", "ನಾಮಾಂಗನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "나망간 주"), ("ky", "Наманган облусу"), ("lt", "Namangano vilajetas"), ("lv", "Namanganas vilojats"), ("mk", "Наманганска област"), ("mr", "नमनगन विलायती"), ("ms", "Wilayah Namangan"), ("nb", "Namangan region"), ("nl", "Namangan"), ("no", "Namangan region"), ("pa", "ਨਮਾਗਾਨ ਵਿਲ\u{a4b}ਇਤੀ"), ("pl", "Wilajet namangański"), ("pt", "Namangan"), ("ro", "Regiunea Namangan"), ("ru", "Наманганская область"), ("si", "නමන\u{dca}ගන\u{dca} කල\u{dcf}පය"), ("sv", "Namangan"), ("ta", "ந\u{bbe}மங\u{bcd}கன\u{bcd} பகுதி"), ("te", "న\u{c3e}మంగ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตนาม\u{e31}นก\u{e31}น"), ("tr", "Namangan ili"), ("uk", "Наманганська область"), ("ur", "نمنگان صوبہ"), ("uz", "Namangan"), ("vi", "Namangan"), ("zh", "納曼干州")]),
                        unofficial_name_list: ["Namangan", "Namaņgan"].to_vec(),
                    }
                ),
                (
                    "NW",
                    Subdivision{
                        name: "NW",
                        country_alpha2: Alpha2::UZ,
                        code: "NW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.084444), longitude: Some(65.379167), max_latitude: Some(40.130427), min_latitude: Some(40.0655921), max_longitude: Some(65.4370594), min_longitude: Some(65.3061677)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نواوي"), ("az", "Nəvai vilayəti"), ("be", "Навойскі вілает"), ("bg", "Навойска област"), ("bn", "ন\u{9be}ভ\u{9c1}ই অঞ\u{9cd}চল"), ("ca", "Província de Navoiy"), ("ccp", "𑄚𑄞\u{11130}𑄠\u{11128}"), ("ceb", "Navoiy Province"), ("da", "Navoiy Region"), ("de", "Provinz Navoiy"), ("el", "Ναβόιγ"), ("en", "Navoiy"), ("es", "Provincia de Navoi"), ("et", "Navoiy vilajett"), ("fa", "استان نوایی"), ("fi", "Navoiyn alue"), ("fr", "province de Navoï"), ("gl", "Provincia de Navoiy"), ("gu", "નવોય પ\u{acd}રદ\u{ac7}શ"), ("hi", "नवोई प\u{94d}रान\u{94d}त"), ("id", "Provinsi Navoiy"), ("it", "Regione di Navoiy"), ("ja", "ナヴァーイー州"), ("kk", "Науаи облысы"), ("kn", "ನವೋಯ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "나보이 주"), ("ky", "Навоий облусу"), ("lt", "Navojo vilajetas"), ("lv", "Navoji vilojats"), ("mk", "Навојска област"), ("mr", "नावोयी विलायती"), ("ms", "Wilayah Navoiy"), ("nb", "Navoiy region"), ("nl", "Navoiy"), ("no", "Navoiy region"), ("pa", "ਨਵ\u{a4b}ਈ ਵਿਲ\u{a4b}ਇਤੀ"), ("pl", "Wilajet nawojski"), ("pt", "Navoiy"), ("ru", "Навоийская область"), ("si", "නවොය\u{dd2} කල\u{dcf}පය"), ("sv", "Navoi"), ("ta", "நவோய\u{bcd} பகுதி"), ("te", "న\u{c3e}వ\u{c4b}య\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "นาวอยย\u{e4c}"), ("tr", "Navoiy ili"), ("uk", "Навоїйська область"), ("ur", "نوائی صوبہ"), ("uz", "Navoiy viloyati"), ("vi", "Navoiy"), ("zh", "納沃伊州")]),
                        unofficial_name_list: ["Navoi", "Navoj", "Navoy"].to_vec(),
                    }
                ),
                (
                    "QA",
                    Subdivision{
                        name: "QA",
                        country_alpha2: Alpha2::UZ,
                        code: "QA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.8986231), longitude: Some(66.0463534), max_latitude: Some(39.5323751), min_latitude: Some(38.0266721), max_longitude: Some(67.683914), min_longitude: Some(64.331024)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية قشقداريا"), ("az", "Qaşqadərya vilayəti"), ("bg", "Кашкадаринска област"), ("bn", "ক\u{9be}\u{9be}শক\u{9be}ড\u{9cd}র\u{9be}য\u{9bc}ো অঞ\u{9cd}চল"), ("ccp", "𑄇\u{11127}𑄠𑄌\u{11134}𑄇𑄓𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Qashqadaryo Province"), ("da", "Qashqadaryo Region"), ("de", "Provinz Qashqadaryo"), ("el", "Κασκαντάριο"), ("en", "Qashqadaryo"), ("es", "Provincia de Kashkadar"), ("et", "Qashqadaryo vilajett"), ("fa", "استان قشقه\u{200c}دریا"), ("fi", "Qashqadaryon alue"), ("fr", "province de Kachkadaria"), ("gl", "Provincia de Qashqadaryo"), ("gu", "કશકડાર\u{acd}યો પ\u{acd}રદ\u{ac7}શ"), ("hi", "क\u{93c}श\u{94d}क\u{93c}ादरिया प\u{94d}रान\u{94d}त"), ("hy", "Կաշկադարյայի մարզ"), ("id", "Provinsi Qashqadaryo"), ("it", "Regione di Kashkadarya"), ("ja", "カシュカダリヤ州"), ("kn", "ಖಶ\u{ccd}ಕಡರ\u{cbf}ಯೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "카슈카다리야 주"), ("ky", "Кашкадарыя облусу"), ("lt", "Kaškadarjos vilajetas"), ("lv", "Kaškadarjas vilojats"), ("mk", "Кашкадарјанска област"), ("mr", "काशकादर\u{94d}यो विलायती"), ("ms", "Wilayah Qashqadaryo"), ("nb", "Qashqadryo region"), ("nl", "Qashqadaryo"), ("no", "Qashqadryo region"), ("pa", "ਕਸ\u{a3c}ਕਾਦਰਯ\u{a4b} ਵਿਲ\u{a4b}ਇਤੀ"), ("pl", "Wilajet kaszkadaryjski"), ("pt", "Qashqadaryo"), ("ro", "Regiunea Kașkadaria"), ("ru", "Кашкадарьинская область"), ("si", "කෂ\u{dca}කදර\u{dca}යෝ කල\u{dcf}පය"), ("sv", "Qasjqadarja"), ("ta", "கஷ\u{bcd}கிஆடயோ பகுதி"), ("te", "క\u{c3e}ష\u{c4d}క\u{c3e}డ\u{c3e}ర\u{c4d}య\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เทศมณฑลล\u{e35}ทร\u{e34}ม"), ("tr", "Kaşkaderya"), ("uk", "Кашкадарʼїнська область"), ("ur", "قشقہ دریا صوبہ"), ("uz", "Qashqadaryo viloyati"), ("vi", "Qashqadaryo"), ("zh", "卡什卡達里亞州")]),
                        unofficial_name_list: ["Kashkadar", "Kashkadaria", "Kashkadarya", "Kaskadar", "Qashqadar", "Qasqadar", "Qaşqadar", "Ķaşķadar"].to_vec(),
                    }
                ),
                (
                    "QR",
                    Subdivision{
                        name: "QR",
                        country_alpha2: Alpha2::UZ,
                        code: "QR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8041334), longitude: Some(59.4457988), max_latitude: Some(45.60519), min_latitude: Some(40.993829), max_longitude: Some(62.37159389999999), min_longitude: Some(55.996635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Karakalpakië"), ("ar", "قرقل باغستان"), ("az", "Qaraqalpaqıstan Respublikası"), ("be", "Каракалпакстан"), ("bg", "Каракалпакстан"), ("bn", "ক\u{9be}র\u{9be}ক\u{9be}লপ\u{9be}কত\u{9be}ন"), ("ca", "Karakalpakistan"), ("ccp", "𑄇𑄢𑄇𑄣\u{11134}𑄛𑄇\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Karakalpakstan"), ("cs", "Karakalpakstán"), ("da", "Karakalpakstan"), ("de", "Karakalpakistan"), ("el", "Καρακαλπακστάν"), ("en", "Karakalpakstan"), ("es", "Karakalpakia"), ("et", "Karakalpakkia"), ("eu", "Karakalpakstan"), ("fa", "قره\u{200c}قالپاقستان"), ("fi", "Karakalpakia"), ("fr", "Karakalpakistan"), ("gu", "કરકાલપાકસ\u{acd}તાન"), ("he", "קרקלפקסטן"), ("hi", "क\u{93c}ाराक\u{93c}ालपाक\u{93c}स\u{94d}तान"), ("hu", "Karakalpaksztán"), ("id", "Karakalpakstan"), ("it", "Karakalpakstan"), ("ja", "カラカルパクスタン共和国"), ("ka", "ყარაყალპაკეთის რესპუბლიკა"), ("kk", "Қарақалпақстан"), ("kn", "ಕರಕಲ\u{ccd}ಪಕ\u{ccd}ಸ\u{ccd}ತಾನ\u{ccd}"), ("ko", "카라칼파크스탄 공화국"), ("ky", "Каракалпакстан Республикасы"), ("lt", "Karakalpakija"), ("lv", "Karakalpakstāna"), ("mk", "Каракалпакстан"), ("ml", "കര\u{d3e}കല\u{d4d}പക\u{d4d}സ\u{d4d}ഥ\u{d3e}ൻ"), ("mn", "Хархалпак орон"), ("mr", "करकालपाकस\u{94d}तान"), ("ms", "Karakalpakistan"), ("nb", "Karakalpakstan"), ("nl", "Karakalpakië"), ("no", "Karakalpakstan"), ("pa", "ਕਰਾਕਲਪਕਸਤਾਨ"), ("pl", "Karakałpacja"), ("pt", "Caracalpaquistão"), ("ro", "Karakalpakstan"), ("ru", "Каракалпакстан"), ("si", "කරකල\u{dca}පක\u{dca}ස\u{dca}ථ\u{dcf}නය"), ("sk", "Karakalpacko"), ("sr", "Каракалпакија"), ("sr_Latn", "Karakalpakija"), ("sv", "Karakalpakstan"), ("ta", "க\u{bbe}ரகல\u{bcd}பக\u{bcd}ஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "క\u{c3e}ర\u{c3e}కల\u{c4d}పక\u{c4d}\u{200c}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "การาก\u{e31}ลป\u{e31}กสถาน"), ("tk", "Garagalpagystan"), ("tr", "Karakalpakistan"), ("uk", "Каракалпакстан"), ("ur", "کاراکالپکستان"), ("uz", "Qoraqalpogʻiston"), ("vi", "Qaraqalpaqstan"), ("zh", "卡拉卡爾帕克斯坦自治共和國")]),
                        unofficial_name_list: ["Karakalpakistan", "Qoraqalpoghiston", "Qoraqalpogiston"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::UZ,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.6542), longitude: Some(66.9597), max_latitude: Some(39.7405258), min_latitude: Some(39.5408509), max_longitude: Some(67.1334059), min_longitude: Some(66.8023682)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سمرقند"), ("az", "Səmərqənd vilayəti"), ("be", "Самаркандская вобласць"), ("bg", "Самаркандска област"), ("bn", "স\u{9be}ম\u{9be}রক\u{9be}ন\u{9cd}ড অঞ\u{9cd}চল"), ("ca", "Província de Samarcanda"), ("ccp", "𑄥𑄟𑄢\u{11134}𑄇𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Samarqand Viloyati"), ("da", "Samarqand Region"), ("de", "Provinz Samarqand"), ("el", "Σαμαρκάντ"), ("en", "Samarqand"), ("es", "Provincia de Samarcanda"), ("et", "Samarqandi vilajett"), ("eu", "Samarkanda probintzia"), ("fa", "استان سمرقند"), ("fi", "Samarkandin alue"), ("fr", "province de Samarcande"), ("gl", "Provincia de Samarcanda"), ("gu", "સમરક\u{a82}દ પ\u{acd}રદ\u{ac7}શ"), ("hi", "समरक\u{93c}न\u{94d}द प\u{94d}रान\u{94d}त"), ("id", "Provinsi Samarqand"), ("it", "Regione di Samarcanda"), ("ja", "サマルカンド州"), ("kk", "Самарқан облысы"), ("kn", "ಸಮರ\u{ccd}ಖಂಡ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "사마르칸트 주"), ("ky", "Самарканд облусу"), ("lt", "Samarkando vilajetas"), ("lv", "Samarkandas vilojats"), ("mk", "Самаркандска област"), ("mr", "समरक\u{902}द विलायती"), ("ms", "Wilayah Samarqand"), ("nb", "Samarqand region"), ("nl", "Samarkand"), ("no", "Samarqand region"), ("pa", "ਸਮਰਕ\u{a70}ਦ ਵਿਲ\u{a4b}ਇਤੀ"), ("pl", "Wilajet samarkandzki"), ("pt", "Samarcanda"), ("ro", "Regiunea Samarkand"), ("ru", "Самаркандская область"), ("si", "සමර\u{dca}කන\u{dca}ද\u{dca} කල\u{dcf}පය"), ("sv", "Samarkand"), ("ta", "சமர\u{bcd}கின\u{bcd}ட\u{bcd} பகுதி"), ("te", "సముర\u{c4d}\u{200c}ఖండ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซามาร\u{e4c}แคนด\u{e4c}"), ("tr", "Semerkand ili"), ("uk", "Самаркандська область"), ("ur", "سمرقند صوبہ"), ("uz", "Samarqand viloyati"), ("vi", "Samarqand"), ("zh", "撒馬爾罕州")]),
                        unofficial_name_list: ["Samarkand", "Samarkand", "Samarqand", "Samarķand"].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::UZ,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.85), longitude: Some(68.666667), max_latitude: Some(40.8751355), min_latitude: Some(40.8027001), max_longitude: Some(68.6985825), min_longitude: Some(68.642063)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سرداريا"), ("az", "Sırdərya vilayəti"), ("bg", "Сърдаринска област"), ("bn", "সিড\u{9cd}র\u{9be}য\u{9bc}ো অঞ\u{9cd}চল"), ("ccp", "𑄥\u{11128}𑄢\u{11134}𑄓𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Sirdaryo (lalawigan)"), ("da", "Sirdaryo Region"), ("de", "Provinz Sirdaryo"), ("el", "Σιρντάριο"), ("en", "Sirdaryo"), ("es", "Provincia de Sirdarín"), ("et", "Sirdaryo vilajett"), ("eu", "Sirdaryo probintzia"), ("fa", "استان سیردریا"), ("fi", "Sirdaryon alue"), ("fr", "province de Syr-Daria"), ("gu", "શિરદાર\u{acd}યો પ\u{acd}રદ\u{ac7}શ"), ("hi", "सिरदरिया प\u{94d}रान\u{94d}त"), ("id", "Provinsi Sirdaryo"), ("it", "Regione di Sirdarya"), ("ja", "シルダリヤ州"), ("kn", "ಸ\u{cbf}ರ\u{ccd}ಡೇರ\u{cbf}ಯೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "시르다리야 주"), ("ky", "Сырдарыя облусу"), ("lt", "Syrdarjos vilajetas"), ("lv", "Sirdarjas vilojats"), ("mk", "Сирдарјанска област"), ("mr", "सीरदर\u{94d}यो विलायती"), ("ms", "Wilayah Sirdaryo"), ("nb", "Sirdarya region"), ("nl", "Sirdaryo"), ("no", "Sirdarya region"), ("pa", "ਸਿਰਦਾਰਿਓ ਸ\u{a42}ਬਾ"), ("pl", "Wilajet syrdaryjski"), ("pt", "Sirdaryo"), ("ro", "Regiunea Sârdaria"), ("ru", "Сырдарьинская область"), ("si", "සර\u{dca}දර\u{dca}යෝ කල\u{dcf}පය"), ("sv", "Syr-Darja"), ("ta", "ச\u{bc0}ர\u{bcd}தரயோ பகுதி"), ("te", "స\u{c3f}ర\u{c4d}డ\u{c3e}ర\u{c4d}య\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซ\u{e35}ร\u{e4c}ดาร\u{e34}โอ"), ("tr", "Sirderya ili"), ("uk", "Сирдарʼїнська область"), ("ur", "سیر دریا صوبہ"), ("uz", "Sirdaryo viloyati"), ("vi", "Sirdaryo"), ("zh", "錫爾河州")]),
                        unofficial_name_list: ["Sirdare", "Syrdarja", "Syrdarya"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::UZ,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9409005), longitude: Some(67.57085359999999), max_latitude: Some(39.078876), min_latitude: Some(37.1843299), max_longitude: Some(68.402641), min_longitude: Some(66.51777609999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية صرخنداريا"), ("az", "Surxandərya vilayəti"), ("bg", "Сурхандаринска област"), ("bn", "স\u{9be}ক\u{9cd}সোনদ\u{9be}রিয\u{9bc}ো অঞ\u{9cd}চল"), ("ca", "Surjandarin"), ("ccp", "𑄥𑄢\u{11134}𑄎\u{11127}𑄚\u{11134}𑄓𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Surxondaryo Viloyati"), ("cs", "Surchandarijský kraj"), ("da", "Surxondaryo Region"), ("de", "Provinz Surxondaryo"), ("el", "Επαρχία Σουρξοντάριο"), ("en", "Surxondaryo"), ("es", "Provincia de Surjandarín"), ("et", "Surxondaryo vilajett"), ("fa", "استان سرخان\u{200c}دریا"), ("fi", "Surxondaryon alue"), ("fr", "province de Sourkhan-Daria"), ("gu", "સર\u{acd}કસો\u{a82}ડરયો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סורח׳אן-דריא"), ("hi", "स\u{941}रख\u{93c}ानदरिया प\u{94d}रान\u{94d}त"), ("hy", "Սուրխանդարյայի մարզ"), ("id", "Provinsi Surxondaryo"), ("it", "Regione di Surkhandarya"), ("ja", "スルハンダリヤ州"), ("ka", "სურხანდარიის ოლქი"), ("kn", "ಸರ\u{ccd}ಕ\u{ccd}ಸಾಂಡರ\u{cbf}ಯೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "수르한다리야 주"), ("ky", "Сурхандарыя облусу"), ("lt", "Surchandarjos vilajetas"), ("lv", "Surhondarjas vilojats"), ("mk", "Сурхандарјанска област"), ("mr", "स\u{941}र\u{94d}झोनदर\u{94d}यो विलायती"), ("ms", "Wilayah Surxondaryo"), ("nb", "Surxondaryo region"), ("nl", "Surxondaryo"), ("no", "Surxondaryo region"), ("pa", "ਸ\u{a41}ਰਖਾਨਦਰਿਆ ਸ\u{a42}ਬਾ"), ("pl", "Wilajet surchandaryjski"), ("pt", "Surxondaryo"), ("ro", "Regiunea Surhandaria"), ("ru", "Сурхандарьинская область"), ("si", "සර\u{dca}කොන\u{dca}ඩර\u{dca}යෝ කල\u{dcf}පය"), ("sv", "Surchondarja"), ("ta", "சூர\u{bcd}ஸோன\u{bcd}றயோ பகுதி"), ("te", "సురక\u{c4d}స\u{c3e}ండ\u{c3e}ర\u{c4d}య\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตซ\u{e39}ร\u{e4c}คอนดาร\u{e34}โอ"), ("tr", "Surhanderya"), ("uk", "Сурхандарʼїнська область"), ("ur", "سرخان دریا صوبہ"), ("uz", "Surxondaryo viloyati"), ("vi", "Surxondaryo"), ("zh", "蘇爾漢河州")]),
                        unofficial_name_list: ["Surkhondar"].to_vec(),
                    }
                ),
                (
                    "TK",
                    Subdivision{
                        name: "TK",
                        country_alpha2: Alpha2::UZ,
                        code: "TK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.266667), longitude: Some(69.216667), max_latitude: Some(41.3985579), min_latitude: Some(41.166637), max_longitude: Some(69.41222189999999), min_longitude: Some(69.1465116)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tasjkent"), ("am", "ታሽኬንት"), ("ar", "طشقند"), ("az", "Daşkənd"), ("be", "Ташкент"), ("bg", "Ташкент"), ("bn", "ত\u{9be}শখন\u{9cd}দ"), ("bs", "Taškent"), ("ca", "Taixkent"), ("ccp", "𑄑𑄌\u{11134}𑄇𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Toshkent Shahri"), ("cs", "Taškent"), ("cy", "Tashkent"), ("da", "Tasjkent"), ("de", "Taschkent"), ("el", "Τασκένδη"), ("en", "Tashkent"), ("es", "Taskent"), ("et", "Toshkent"), ("eu", "Tashkent"), ("fa", "تاشکند"), ("fi", "Taškent"), ("fr", "Tachkent"), ("ga", "Tashkent"), ("gl", "Tashkent"), ("gu", "તાશક\u{ac7}ન\u{acd}ટ"), ("he", "טשקנט"), ("hi", "ताशकन\u{94d}द"), ("hr", "Taškent"), ("hu", "Taskent"), ("hy", "Տաշքենդ"), ("id", "Tashkent"), ("is", "Taskent"), ("it", "Tashkent"), ("ja", "タシュケント"), ("ka", "ტაშკენტი"), ("kk", "Ташкент"), ("kn", "ತಾಷ\u{ccd}ಕ\u{cc6}ಂಟ\u{ccd}"), ("ko", "타슈켄트"), ("ky", "Ташкен"), ("lt", "Taškentas"), ("lv", "Taškenta"), ("mk", "Ташкент"), ("ml", "ത\u{d3e}ഷ\u{d4d}കന\u{d4d}റ\u{d4d}"), ("mn", "Ташкент"), ("mr", "ताश\u{94d}क\u{902}द"), ("ms", "Tashkent"), ("nb", "Tasjkent"), ("ne", "ट\u{94d}यासक\u{947}न\u{94d}ट"), ("nl", "Tasjkent"), ("no", "Tasjkent"), ("pa", "ਤਾਸ\u{a3c}ਕ\u{a70}ਦ"), ("pl", "Taszkent"), ("ps", "تاشکند"), ("pt", "Tashkent"), ("ro", "Tașkent"), ("ru", "Ташкент"), ("si", "ටෂ\u{dca}කෙන\u{dca}ට\u{dca}"), ("sk", "Taškent"), ("sl", "Taškent"), ("sq", "Tashkenti"), ("sr", "Ташкент"), ("sr_Latn", "Taškent"), ("sv", "Tasjkent"), ("sw", "Tashkent"), ("ta", "த\u{bbe}ஷ\u{bcd}கந\u{bcd}து"), ("te", "త\u{c3e}ష\u{c4d}క\u{c46}ంట\u{c4d}"), ("th", "ทาชเคนต\u{e4c}"), ("tk", "Daşkent"), ("tr", "Taşkent"), ("uk", "Ташкент"), ("ur", "تاشقند"), ("uz", "Toshkent"), ("vi", "Tashkent"), ("yo", "Tashkent"), ("yo_BJ", "Tashkent"), ("yue", "塔什干"), ("yue_Hans", "塔什干"), ("zh", "塔什干")]),
                        unofficial_name_list: ["Toshkent City"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::UZ,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.266667), longitude: Some(69.216667), max_latitude: Some(41.3985579), min_latitude: Some(41.166637), max_longitude: Some(69.41222189999999), min_longitude: Some(69.1465116)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية طشقند"), ("az", "Daşkənd vilayəti"), ("be", "Ташкенцкі вілает"), ("bg", "Ташкентска област"), ("bn", "ত\u{9be}সকেন\u{9cd}ট অঞ\u{9cd}চল"), ("ca", "Regió de Taixkent"), ("ccp", "𑄑𑄌\u{11134}𑄇𑄬𑄚\u{11134}𑄑\u{11134} 𑄛\u{11133}𑄢\u{1112e}𑄞\u{11128}𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Toshkent Viloyati"), ("da", "Tashkent Region"), ("de", "Provinz Taschkent"), ("el", "Τασκέντ"), ("en", "Tashkent Province"), ("es", "Provincia de Taskent"), ("et", "Toshkendi vilajett"), ("eu", "Toshkent probintzia"), ("fa", "استان تاشکند"), ("fi", "Taškentin alue"), ("fr", "province de Tachkent"), ("gl", "Provincia de Tashkent"), ("gu", "તાશ\u{acd}ક\u{ac7}\u{a82}ટ પ\u{acd}રદ\u{ac7}શ"), ("hi", "ताशक\u{947}\u{902}त प\u{94d}रान\u{94d}त"), ("hu", "Taskenti terület"), ("id", "Provinsi Toshkent"), ("it", "Regione di Tashkent"), ("ja", "タシュケント州"), ("kn", "ತಾಷ\u{ccd}ಕ\u{cc6}ಂಟ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "타슈켄트 주"), ("ky", "Ташкент облусу"), ("lt", "Taškento vilajetas"), ("lv", "Taškentas vilojats"), ("mk", "Ташкентска област"), ("mr", "तोश\u{94d}क\u{947}\u{902}त विलायती"), ("ms", "Wilayah Toshkent"), ("nb", "Tashkent region"), ("nl", "Tasjkent²"), ("no", "Tashkent region"), ("pa", "ਤਾਸ\u{a3c}ਕ\u{a47}\u{a02}ਤ ਸ\u{a42}ਬਾ"), ("pl", "Wilajet taszkencki"), ("pt", "Tashkent²"), ("ro", "Regiunea Tașkent"), ("ru", "Ташкентская область"), ("si", "ටශ\u{dca}කෙන\u{dca}ට\u{dca} කල\u{dcf}පය"), ("sv", "Tasjkent²"), ("ta", "த\u{bbe}ஷ\u{bcd}கண\u{bcd}ட\u{bcd} பகுதி"), ("te", "ట\u{c3e}ష\u{c4d}క\u{c46}ంట\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ทอชเคนต\u{e4c}"), ("tr", "Taşkent ili"), ("uk", "Ташкентська область"), ("ur", "تاشقند صوبہ"), ("uz", "Toshkent viloyati"), ("vi", "Toshkent"), ("zh", "塔什干州")]),
                        unofficial_name_list: ["Tachkent", "Taschkent", "Taškent", "Toshkent", "Toshkent", "Toşkent"].to_vec(),
                    }
                ),
                (
                    "XO",
                    Subdivision{
                        name: "XO",
                        country_alpha2: Alpha2::UZ,
                        code: "XO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.3565336), longitude: Some(60.8566686), max_latitude: Some(42.004252), min_latitude: Some(40.564991), max_longitude: Some(62.35982399999999), min_longitude: Some(60.06019589999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية خوارزم"), ("az", "Xarəzm vilayəti"), ("be", "Харэзмская вобласць"), ("bg", "Хорезъмска област"), ("bn", "জোর\u{9be}ম অঞ\u{9cd}চল"), ("ccp", "𑄎\u{1112e}𑄢𑄌\u{11134}𑄟\u{11134}"), ("ceb", "Xorazm Viloyati"), ("da", "Xorazm Region"), ("de", "Provinz Xorazm"), ("el", "Ξοράζμ"), ("en", "Xorazm"), ("es", "Provincia de Corasmia"), ("et", "Xorazm"), ("fa", "استان خوارزم"), ("fi", "Xorazmin alue"), ("fr", "province de Khorezm"), ("gl", "Provincia de Corasmia"), ("gu", "ઝોરજ\u{acd}મ પ\u{acd}રદ\u{ac7}શ"), ("hi", "ख\u{93c}ोरज\u{93c}\u{94d}म प\u{94d}रान\u{94d}त"), ("hy", "Խորեզմի մարզ"), ("id", "Provinsi Xorazm"), ("it", "Regione di Khorezm"), ("ja", "ホラズム州"), ("kk", "Хорезм облысы"), ("kn", "ಝೊರಾಝಮ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "호레즘 주"), ("ky", "Хорезм облусу"), ("lt", "Chorezmo vilajetas"), ("lv", "Horezmas vilojats"), ("mk", "Хорезмска област"), ("mr", "झोराझ\u{94d}म विलायती"), ("ms", "Wilayah Xorazm"), ("nb", "Xorazm region"), ("nl", "Xorazm"), ("no", "Xorazm region"), ("pa", "ਖ\u{a4b}ਰ\u{a47}ਜਮ ਸ\u{a42}ਬਾ"), ("pl", "Wilajet chorezmijski"), ("pt", "Khorazm"), ("ru", "Хорезмская область"), ("si", "ක\u{dca}සොර\u{dcf}ස\u{dca}ම\u{dca} කල\u{dcf}පය"), ("sv", "Khwarezm"), ("ta", "ஸோர\u{bbe}ஸ\u{bcd}ம\u{bcd} பகுதி"), ("te", "జ\u{c4b}ర\u{c3e}జమ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โซร\u{e31}ซ"), ("tr", "Harezm ili"), ("uk", "Хорезмська область"), ("ur", "خوارزم صوبہ"), ("uz", "Xorazm viloyati"), ("vi", "Xorazm"), ("zh", "花拉子模州")]),
                        unofficial_name_list: ["Khorazm"].to_vec(),
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
#[cfg(feature = "uz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::UZ,
        alpha3: Alpha3::UZB,
        address_format: None,
        continent: Continent::Asia,
        country_code: 998,
        currency_code: CurrencyCode::UZS,
        gec: Some(GEC::UZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::UZB),
        iso_long_name: "The Republic of Uzbekistan",
        iso_short_name: "Uzbekistan",
        official_language_list: ["ru", "uz"].to_vec(),
        spoken_language_list: ["ru", "uz"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "8",
        nationality: Some("Uzbekistani"),
        number: "860",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAsia),
        un_locode: "UZ",
        unofficial_name_list: [
            "Uzbekistan",
            "Usbekistan",
            "Ouzbékistan",
            "Uzbekistán",
            "ウズベキスタン",
            "Oezbekistan",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Uzbekistan"),
            ("af", "Oesbekistan"),
            ("ak", "Uzbekistan"),
            ("am", "ፙፔበጡስታን"),
            ("an", "Uzbekistan"),
            ("ar", "أوزبكستان"),
            ("as", "উজ\u{9cd}বেকিস\u{9cd}ত\u{9be}ন"),
            ("ay", "Uzbekistan"),
            ("az", "Özbəkistan"),
            ("ba", "Uzbekistan"),
            ("be", "Узбекістан"),
            ("bg", "Узбекистан"),
            ("bi", "Uzbekistan"),
            ("bn", "উজবেকিস\u{9cd}ত\u{9be}ন"),
            ("bn_IN", "উজবেকিস\u{9cd}ত\u{9be}ন"),
            ("br", "Ouzbekistan"),
            ("bs", "Uzbekistan"),
            ("ca", "Uzbekistan"),
            ("ce", "Узбекистан"),
            ("ch", "Uzbekistan"),
            ("cs", "Uzbekistán"),
            ("cv", "Узбекистан"),
            ("cy", "Wsbecistan"),
            ("da", "Usbekistan"),
            ("de", "Usbekistan"),
            (
                "dv",
                "އ\u{7aa}ޒ\u{7b0}ބ\u{7ac}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}",
            ),
            ("dz", "ཨ\u{f74}ཛ་བ\u{f7a}་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
            ("ee", "Uzbekistan"),
            ("el", "Ουζμπεκιστάν"),
            ("en", "Uzbekistan"),
            ("eo", "Uzbekio"),
            ("es", "Uzbekistán"),
            ("et", "Usbekistan"),
            ("eu", "Uzbekistan"),
            ("fa", "ازبکستان"),
            ("ff", "Uzbekistan"),
            ("fi", "Uzbekistan"),
            ("fo", "Usbekistan"),
            ("fr", "Ouzbékistan"),
            ("fy", "Oezbekistan"),
            ("ga", "Úisbéiceastáin"),
            ("gl", "Uzbequistán"),
            ("gn", "Uzbekistan"),
            ("gu", "ઉઝબ\u{ac7}કિસ\u{acd}તાન"),
            ("gv", "Yn Oosbeckistaan"),
            ("ha", "Uzbekistan"),
            ("he", "אוזבקיסטן"),
            ("hi", "उज\u{93c}\u{94d}ब\u{947}किस\u{94d}तान"),
            ("hr", "Uzbekistan"),
            ("ht", "Ouzbekistan"),
            ("hu", "Üzbegisztán"),
            ("hy", "Ուզբեկստան"),
            ("ia", "Uzbekistan"),
            ("id", "Uzbekistan"),
            ("io", "Uzbekistan"),
            ("is", "Úsbekistan"),
            ("it", "Uzbekistan"),
            ("iu", "Uzbekistan"),
            ("ja", "ウズベキスタン"),
            ("ka", "უზბაკეთი"),
            ("ki", "Uzbekistan"),
            ("kk", "Өзбекстан"),
            ("kl", "Uzbekistan"),
            (
                "km",
                "អ\u{17ca}\u{17bc}ហ\u{17d2}សបេគ\u{17b8}ស\u{17d2}តង\u{17cb}",
            ),
            ("kn", "ಉಜ\u{ccd}ಬ\u{cc6}ಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
            ("ko", "우즈베키스탄"),
            ("ku", "Ozbekîstan"),
            ("kv", "Узбекистан"),
            ("kw", "Pow Ousbek"),
            ("ky", "Өзбекстан"),
            ("lo", "Uzbekistan"),
            ("lt", "Uzbekija"),
            ("lv", "Uzbekistāna"),
            ("mi", "Uhipeketāne"),
            ("mk", "Узбекистан"),
            (
                "ml",
                "ഉസ\u{d4d}ബേക\u{d4d}കിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}",
            ),
            ("mn", "узбекстан"),
            ("mr", "उझब\u{947}गिस\u{94d}तान"),
            ("ms", "Uzbekistan"),
            ("mt", "Użbekistan"),
            (
                "my",
                "ဥဇဘက\u{103a}ကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Uzbekistan"),
            ("nb", "Usbekistan"),
            ("ne", "उज\u{94d}ब\u{947}किस\u{94d}तान"),
            ("nl", "Oezbekistan"),
            ("nn", "Usbekistan"),
            ("nv", "Uzbekistan"),
            ("oc", "Ozbequistan"),
            ("or", "ଉଜବେକ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
            ("pa", "ਉਜ਼\u{a47}ਬਕਸਤਾਨ"),
            ("pi", "उजब\u{947}किस\u{94d}थान"),
            ("pl", "Uzbekistan"),
            ("ps", "اوزبکستان"),
            ("pt", "Uzbequistão"),
            ("pt_BR", "Uzbequistão"),
            ("ro", "Uzbekistan"),
            ("ru", "Узбекистан"),
            ("rw", "Uzubekisitani"),
            ("sc", "Uzbèkistan"),
            ("sd", "ازبڪستان"),
            ("si", "උස\u{dca}බෙක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
            ("sk", "Uzbekistan"),
            ("sl", "Uzbekistan"),
            ("so", "Uzbekistan"),
            ("sq", "Uzbekistan"),
            ("sr", "Узбекистан"),
            ("sv", "Uzbekistan"),
            ("sw", "Uzbekistan"),
            ("ta", "உஸ\u{bcd}பெகிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
            ("te", "ఉజ\u{c4d}బ\u{c46}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
            ("tg", "Ӯзбекистон"),
            ("th", "อ\u{e38}ซเบก\u{e34}สถาน"),
            ("ti", "ኡዝበኪስታን"),
            ("tk", "Özbegistan"),
            ("tl", "Uzbekistan"),
            ("tr", "Özbekistan"),
            ("tt", "Өзбәкстан"),
            ("ug", "ئۆزبېكىستان"),
            ("uk", "Узбекистан"),
            ("ur", "ازبکستان"),
            ("uz", "Oʻzbekiston"),
            ("ve", "Uzbekistan"),
            ("vi", "U-xợ-bê-khi-xtanh"),
            ("wa", "Ouzbekistan"),
            ("wo", "Usbekistaan"),
            ("xh", "Uzbekistan"),
            ("yo", "Ùsbẹ\u{300}kìstán"),
            ("zh_CN", "乌兹别克斯坦"),
            ("zh_HK", "烏茲別克"),
            ("zh_TW", "烏茲別克"),
            ("zu", "Uzbekistan"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
