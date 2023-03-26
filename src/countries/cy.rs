// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Cyprus

#[cfg(all(feature = "cy", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CY;
    pub const ALPHA3: Alpha3 = Alpha3::CYP;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 357;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::CY);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CYP);
    pub const ISO_SHORT_NAME: &str = "Cyprus";
    pub const ISO_LONG_NAME: &str = "The Republic of Cyprus";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["el", "hy", "tr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["el", "hy", "tr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Cypriot");
    pub const NUMBER: &str = "196";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "CY";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Cyprus", "Zypern", "Chypre", "Chipre", "キプロス"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Cyprus"),
        ("af", "Siprus"),
        ("ak", "Cyprus"),
        ("am", "ሳ፤ፕረስ"),
        ("an", "Cyprus"),
        ("ar", "قبرص"),
        ("as", "ছ\u{9be}ইপ\u{9cd}ৰ\u{9be}ছ"),
        ("ay", "Cyprus"),
        ("az", "Kipr"),
        ("ba", "Cyprus"),
        ("be", "Кіпр"),
        ("bg", "Кипър"),
        ("bi", "Cyprus"),
        ("bn", "স\u{9be}ইপ\u{9cd}র\u{9be}স"),
        ("bn_IN", "স\u{9be}ইপ\u{9cd}র\u{9be}স"),
        ("br", "Enez Chipr"),
        ("bs", "Kipar"),
        ("ca", "Xipre"),
        ("ce", "Кипр"),
        ("ch", "Cyprus"),
        ("cs", "Kypr"),
        ("cv", "Кипр"),
        ("cy", "Cyprus"),
        ("da", "Cypern"),
        ("de", "Zypern"),
        ("dv", "Cyprus"),
        ("dz", "སའ\u{f72}་པར\u{f71}ས\u{f72}།"),
        ("ee", "Cyprus"),
        ("el", "Κύπρος"),
        ("en", "Cyprus"),
        ("eo", "Kipro"),
        ("es", "Chipre"),
        ("et", "Küpros"),
        ("eu", "Zipre"),
        ("fa", "قبرس"),
        ("ff", "Kibris"),
        ("fi", "Kypros"),
        ("fo", "Kýpros"),
        ("fr", "Chypre"),
        ("fy", "Syprus"),
        ("ga", "An Chipir"),
        ("gl", "Chipre"),
        ("gn", "Cyprus"),
        ("gu", "સાયપ\u{acd}રસ"),
        ("gv", "Yn Cheeprey"),
        ("ha", "Cyprus"),
        ("he", "קפריסין"),
        ("hi", "साइप\u{94d}रस"),
        ("hr", "Cipar"),
        ("ht", "Chip"),
        ("hu", "Ciprus"),
        ("hy", "Կիպրոս"),
        ("ia", "Cypro"),
        ("id", "Siprus"),
        ("io", "Chipro"),
        ("is", "Kýpur"),
        ("it", "Cipro"),
        ("iu", "Cyprus"),
        ("ja", "キプロス"),
        ("ka", "კვიპროსი"),
        ("ki", "Cyprus"),
        ("kk", "Кипр"),
        ("kl", "Cyprus"),
        ("km", "ស\u{17ca}\u{17b8}ពរ\u{17cd}"),
        ("kn", "ಸೈಪ\u{ccd}ರಸ\u{ccd}"),
        ("ko", "키프로스"),
        ("ku", "Kibris"),
        ("kv", "Кипр"),
        ("kw", "Kobros"),
        ("ky", "Кипр"),
        ("lo", "ປະເທດຊ\u{eb5}ບ"),
        ("lt", "Kipras"),
        ("lv", "Kipra"),
        ("mi", "Haipara"),
        ("mk", "Кипар"),
        ("ml", "സൈപ\u{d4d}രസ\u{d4d}"),
        ("mn", "Кипр"),
        ("mr", "सायप\u{94d}रस"),
        ("ms", "Cyprus"),
        ("mt", "Ċipru"),
        (
            "my",
            "ဆ\u{102d}\u{102f}က\u{103a}ပရပ\u{103a}စ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Taiprus"),
        ("nb", "Kypros"),
        ("ne", "साइप\u{94d}रस"),
        ("nl", "Cyprus"),
        ("nn", "Kypros"),
        ("nv", "Béésh Łichíiʼii Bikéyah"),
        ("oc", "Chipre"),
        ("or", "ସ\u{b3e}ଇପ\u{b4d}ରସ"),
        ("pa", "ਕਿਊਰ\u{a42}ਸ"),
        ("pi", "सायप\u{94d}रस"),
        ("pl", "Cypr"),
        ("ps", "Cyprus"),
        ("pt", "Chipre"),
        ("pt_BR", "Chipre"),
        ("ro", "Cipru"),
        ("ru", "Кипр"),
        ("rw", "Shipure"),
        ("sc", "Tzipru"),
        ("sd", "Cyprus"),
        ("si", "සය\u{dd2}ප\u{dca}\u{200d}රස\u{dca}"),
        ("sk", "Cyprus"),
        ("sl", "Ciper"),
        ("so", "Qabrus"),
        ("sq", "Qipro"),
        ("sr", "Кипар"),
        ("sv", "Cypern"),
        ("sw", "Cyprus"),
        ("ta", "சைப\u{bcd}ரஸ\u{bcd}"),
        ("te", "స\u{c48}ప\u{c4d}రస\u{c4d}"),
        ("tg", "Кипр"),
        ("th", "ไซปร\u{e31}ส"),
        ("ti", "ሳይፕረስ"),
        ("tk", "Kipr"),
        ("tl", "Cyprus"),
        ("tr", "Kıbrıs"),
        ("tt", "Кипер"),
        ("ug", "سىپرۇس"),
        ("uk", "Кіпр"),
        ("ur", "قبرص"),
        ("uz", "Qibris"),
        ("ve", "Cyprus"),
        ("vi", "Síp"),
        ("wa", "Chipe"),
        ("wo", "Ciipër"),
        ("xh", "Cyprus"),
        ("yo", "Kíprù"),
        ("zh_CN", "塞浦路斯"),
        ("zh_HK", "賽浦路斯"),
        ("zh_TW", "賽普勒斯"),
        ("zu", "Cyprus"),
    ];
    #[cfg(all(feature = "cy", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 35.126413;
        pub const LONGITUDE: f64 = 33.429859;
        pub const MAX_LATITUDE: f64 = 35.7071999;
        pub const MAX_LONGITUDE: f64 = 34.60450000000001;
        pub const MIN_LATITUDE: f64 = 34.6304001;
        pub const MIN_LONGITUDE: f64 = 32.2459;
        pub const NORTHEAST_LATITUDE: f64 = 35.7071999;
        pub const NORTHEAST_LONGITUDE: f64 = 34.60450000000001;
        pub const SOUTHWEST_LATITUDE: f64 = 34.6304001;
        pub const SOUTHWEST_LONGITUDE: f64 = 32.2459;
    }
}
#[cfg(all(feature = "cy", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 35.126413,
            longitude: 33.429859,
            max_latitude: 35.7071999,
            max_longitude: 34.60450000000001,
            min_latitude: 34.6304001,
            min_longitude: 32.2459,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 35.7071999,
                    longitude: 34.60450000000001,
                },
                southwest: CountryGeoBound {
                    latitude: 34.6304001,
                    longitude: 32.2459,
                },
            },
        }
    }
}

#[cfg(all(feature = "cy", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "Lefkosia",
                        country_alpha2: Alpha2::CY,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.166667), longitude: Some(33.366667), max_latitude: Some(35.2323172), min_latitude: Some(35.1516702), max_longitude: Some(33.4762216), min_longitude: Some(33.3186151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة نيقوسيا"), ("be", "Нікосія"), ("bg", "Никозия"), ("ca", "Districte de Nicòsia"), ("ccp", "𑄚\u{11128}𑄇\u{1112e}𑄥\u{11128}𑄠"), ("ceb", "Eparchía Lefkosías"), ("cs", "Nikósie"), ("de", "Bezirk Nikosia"), ("el", "Επαρχία Λευκωσίας"), ("en", "Nicosia"), ("es", "Nicosia"), ("fa", "ناحیه نیکوزیا"), ("fi", "Nikosian alue"), ("fr", "district de Nicosie"), ("he", "מחוז ניקוסיה"), ("hu", "Nikosia kerület"), ("hy", "Նիկոսիայի շրջան"), ("it", "distretto di Nicosia"), ("ja", "ニコシア地区"), ("ka", "ნიქოზიის რაიონი"), ("ko", "니코시아 구"), ("lt", "Nikosijos apygarda"), ("ms", "Daerah Nicosia"), ("nb", "Nikosia"), ("ne", "नोकोसिया जिल\u{94d}ला"), ("nl", "Nicosia"), ("no", "Nikosia"), ("pl", "Dystrykt Nikozja"), ("pt", "Nicósia"), ("ro", "Districtul Nicosia"), ("ru", "Никосия"), ("sr", "Никозија"), ("sr_Latn", "Nikozija"), ("sv", "Eparchía Lefkosías"), ("th", "เขตน\u{e34}โคเซ\u{e35}ย"), ("tr", "Lefkoşa kazası"), ("uk", "Нікосія"), ("ur", "ضلع نیکوسیا"), ("vi", "Nicosia"), ("zh", "尼科西亚区")]),
                        unofficial_name_list: ["Lefkosia"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "Lemesos",
                        country_alpha2: Alpha2::CY,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.7071301), longitude: Some(33.0226174), max_latitude: Some(34.7400842), min_latitude: Some(34.6433942), max_longitude: Some(33.0705513), min_longitude: Some(32.986418)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة ليماسول"), ("bg", "Лимасол"), ("bn", "লিম\u{9be}সোল জেল\u{9be}"), ("ca", "Districte de Limassol"), ("ccp", "𑄣\u{11128}𑄟𑄥\u{1112e}𑄣\u{11134}"), ("ceb", "Eparchía Lemesoú"), ("cs", "Lemesos (distrikt)"), ("da", "Limassol District"), ("de", "Bezirk Limassol"), ("el", "Επαρχία Λεμεσού"), ("en", "Limassol"), ("es", "Limasol"), ("fa", "ناحیه لیماسول"), ("fi", "Limassolin alue"), ("fr", "district de Limassol"), ("gu", "લિમાસોલ જિલ\u{acd}લો"), ("he", "מחוז לימסול"), ("hi", "लिमासोल जिला"), ("hu", "Limassol kerület"), ("hy", "Լիմասոլի շրջան"), ("id", "Distrik Limassol"), ("it", "distretto di Limassol"), ("ja", "リマソール地区"), ("ka", "ლიმასოლის რაიონი"), ("kn", "ಲ\u{cbf}ಮಾಸಾಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "리마솔 구"), ("lt", "Limasolio apygarda"), ("lv", "Limasolas eparhija"), ("mr", "लिमासोल जिल\u{94d}हा"), ("ms", "Daerah Limassol"), ("nb", "Limassol"), ("nl", "Limasol"), ("no", "Limassol"), ("pl", "Dystrykt Limassol"), ("pt", "Limassol"), ("ro", "Districtul Limassol"), ("ru", "Лимасол"), ("si", "ල\u{dd2}මස\u{dca}සොල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Лимасол"), ("sr_Latn", "Limasol"), ("sv", "Eparchía Lemesoú"), ("ta", "லிமஸ\u{bcd}ஸோல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3f}మ\u{c3e}స\u{c4b}ల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตล\u{e35}มาซอล"), ("tr", "Limasol Bölgesi"), ("uk", "Лімасол"), ("ur", "ضلع لیماسول"), ("vi", "Quận Limassol"), ("zh", "利马索尔区")]),
                        unofficial_name_list: ["Lemesos"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "Larnaka",
                        country_alpha2: Alpha2::CY,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.8507206), longitude: Some(33.4831906), max_latitude: Some(35.1124949), min_latitude: Some(34.7075399), max_longitude: Some(33.791904), min_longitude: Some(33.125832)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة لارنكا"), ("bg", "Ларнака"), ("bn", "ল\u{9be}র\u{9cd}ন\u{9be}ক\u{9be} জেল\u{9be}"), ("ca", "Districte de Làrnaca"), ("ccp", "𑄣𑄢\u{11134}𑄚𑄇"), ("ceb", "Eparchía Lárnakas"), ("cs", "Larnaka"), ("da", "Larnaca District"), ("de", "Bezirk Larnaka"), ("el", "Επαρχία Λάρνακας"), ("en", "Larnaca"), ("es", "Lárnaca"), ("fa", "ناحیه لارناکا"), ("fi", "Larnakan alue"), ("fr", "district de Larnaca"), ("gu", "લાર\u{acd}નાકા જિલ\u{acd}લો"), ("he", "מחוז לרנקה"), ("hi", "लार\u{94d}नाका जिला"), ("hu", "Lárnaka kerület"), ("hy", "Լարնակայի շրջան"), ("id", "Distrik Larnaca"), ("it", "distretto di Larnaca"), ("ja", "ラルナカ地区"), ("ka", "ლარნაკის რაიონი"), ("kn", "ಲಾರ\u{ccd}ನಕ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "라르나카 구"), ("lt", "Larnakos apygarda"), ("lv", "Larnakas eparhija"), ("mr", "लार\u{94d}नाका जिल\u{94d}हा"), ("ms", "Daerah Larnaca"), ("nb", "Larnaka"), ("nl", "Larnaca"), ("no", "Larnaka"), ("pl", "Dystrykt Larnaka"), ("pt", "Larnaca"), ("ro", "Larnaca"), ("ru", "Ларнака"), ("si", "ලමක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Ларнака"), ("sr_Latn", "Larnaka"), ("sv", "Eparchía Lárnakas"), ("ta", "ல\u{bbe}ர\u{bcd}நக ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3e}ర\u{c4d}న\u{c3e}క\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตลาร\u{e4c}นากา"), ("tr", "Larnaka kazası"), ("uk", "Ларнака"), ("ur", "ضلع لارناکا"), ("vi", "Quận Larnaca"), ("zh", "拉纳卡区")]),
                        unofficial_name_list: ["Larnaka"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "Ammochostos",
                        country_alpha2: Alpha2::CY,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1149116), longitude: Some(33.919245), max_latitude: Some(35.1608203), min_latitude: Some(35.0780555), max_longitude: Some(33.9829874), min_longitude: Some(33.8847113)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة فاماغوستا"), ("bg", "Фамагуста"), ("ca", "Districte de Famagusta"), ("ccp", "𑄜𑄬𑄟𑄉𑄌\u{11134}𑄑"), ("ceb", "Eparchía Ammochóstou"), ("cs", "Gazimağusa"), ("de", "Bezirk Famagusta"), ("el", "Επαρχία Αμμοχώστου"), ("en", "Famagusta"), ("es", "Famagusta"), ("fa", "ناحیه فاماگوستا"), ("fi", "Famagustan alue"), ("fr", "district de Famagouste"), ("hu", "Famagusta kerület"), ("hy", "Ֆամագուստայի շրջան"), ("it", "distretto di Famagosta"), ("ja", "ファマグスタ地区"), ("ka", "ფამაგუსტის რაიონი"), ("ko", "파마구스타 구"), ("lt", "Famagustos apygarda"), ("ms", "Daerah Famagusta"), ("nb", "Famagusta"), ("nl", "Famagusta"), ("no", "Famagusta"), ("pl", "Dystrykt Famagusta"), ("pt", "Famagusta"), ("ro", "Districtul Famagusta"), ("ru", "Фамагуста"), ("si", "ෆැමග\u{dd4}ස\u{dca}ට\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Mağusa"), ("sr", "Амохостос (округ)"), ("sr_Latn", "Amohostos (okrug)"), ("sv", "Famagusta"), ("th", "เขตแฟมาก\u{e38}สตา"), ("tr", "Mağusa kazası"), ("uk", "Фамагуста"), ("ur", "ضلع فاماگوستا"), ("zh", "法马古斯塔区")]),
                        unofficial_name_list: ["Ammochostos"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "Pafos",
                        country_alpha2: Alpha2::CY,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.7720133), longitude: Some(32.4297369), max_latitude: Some(34.8011783), min_latitude: Some(34.7442135), max_longitude: Some(32.4666024), min_longitude: Some(32.3999583)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة بافوس"), ("bg", "Пафос"), ("ca", "Districte de Pafos"), ("ccp", "𑄛𑄜\u{1112e}𑄌\u{11134}"), ("ceb", "Eparchía Páfou"), ("cs", "Pafos"), ("de", "Bezirk Paphos"), ("el", "Επαρχία Πάφου"), ("en", "Paphos"), ("es", "Pafos"), ("fa", "ناحیه پافوس"), ("fi", "Pafoksen alue"), ("fr", "district de Paphos"), ("he", "מחוז פאפוס"), ("hu", "Paphos kerület"), ("hy", "Պաֆոսի շրջան"), ("it", "distretto di Pafo"), ("ja", "パフォス地区"), ("ka", "პაფოსის რაიონი"), ("ko", "파포스 구"), ("lt", "Pafoso apygarda"), ("ms", "Daerah Paphos"), ("nb", "Pafos"), ("nl", "Paphos"), ("no", "Pafos"), ("pl", "Dystrykt Pafos"), ("pt", "Pafos"), ("ro", "Districtul Paphos"), ("ru", "Пафос"), ("sr", "Пафос"), ("sr_Latn", "Pafos"), ("sv", "Eparchía Páfou"), ("th", "เขตแพฟอส"), ("tr", "Baf kazası"), ("uk", "Пафос"), ("ur", "ضلع پافوس"), ("zh", "帕福斯区")]),
                        unofficial_name_list: ["Pafos"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "Keryneia",
                        country_alpha2: Alpha2::CY,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.0586943), longitude: Some(33.9582783), max_latitude: Some(35.072638), min_latitude: Some(35.0423389), max_longitude: Some(34.0006899), min_longitude: Some(33.9222729)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة كيرينيا"), ("bg", "Кирения"), ("bn", "ক\u{9be}য\u{9bc}রেনিয\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Districte de Kerínia"), ("ccp", "𑄇\u{1112d}𑄢𑄬𑄚\u{11128}𑄠"), ("ceb", "Eparchía Kerýneias"), ("cs", "Kyrenia"), ("da", "Kyrenia District"), ("de", "Bezirk Kyrenia"), ("el", "Επαρχία Κερύνειας"), ("en", "Kyrenia"), ("es", "Girne"), ("fa", "ناحیه گیرنه"), ("fi", "Kyrenian alue"), ("fr", "district de Kyrenia"), ("gu", "કીર\u{ac7}નિયા જિલ\u{acd}લો"), ("hi", "कायर\u{947}निया जिला"), ("hu", "Kyrenia kerület"), ("hy", "Կիրենիայի շրջան"), ("id", "Distrik Kyrenia"), ("it", "distretto di Kyrenia"), ("ja", "キレニア地区"), ("ka", "კირენიის რაიონი"), ("kn", "ಕ\u{cbf}ರ\u{cc6}ನ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "키레니아 구"), ("lt", "Kirenijos apygarda"), ("lv", "Kerinijas eparhija"), ("mr", "कायर\u{947}निया जिल\u{94d}हा"), ("ms", "Daerah Kyrenia"), ("nb", "Kyrenia"), ("nl", "Kyrenia"), ("no", "Kyrenia"), ("pl", "Dystrykt Kirenia"), ("pt", "Kyrenia"), ("ro", "Districtul Kyrenia"), ("ru", "Кирения"), ("si", "කය\u{dd2}රෙන\u{dd2}ය\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Girne"), ("sr", "Керинеја"), ("sr_Latn", "Kerineja"), ("sv", "Kyrenia (distrikt)"), ("ta", "கியரேனிய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c48}ర\u{c47}న\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e35}ร\u{e35}เน\u{e35}ย"), ("tr", "Girne kazası"), ("uk", "Киренія"), ("ur", "ضلع کیرینیہ"), ("vi", "Huyện Kyrenia"), ("zh", "凯里尼亚区")]),
                        unofficial_name_list: ["Keryneia"].to_vec(),
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
#[cfg(feature = "cy")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CY,
        alpha3: Alpha3::CYP,
        address_format: None,
        continent: Continent::Asia,
        country_code: 357,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::CY),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CYP),
        iso_long_name: "The Republic of Cyprus",
        iso_short_name: "Cyprus",
        official_language_list: ["el", "hy", "tr"].to_vec(),
        spoken_language_list: ["el", "hy", "tr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Cypriot"),
        number: "196",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "CY",
        unofficial_name_list: ["Cyprus", "Zypern", "Chypre", "Chipre", "キプロス"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Cyprus"),
            ("af", "Siprus"),
            ("ak", "Cyprus"),
            ("am", "ሳ፤ፕረስ"),
            ("an", "Cyprus"),
            ("ar", "قبرص"),
            ("as", "ছ\u{9be}ইপ\u{9cd}ৰ\u{9be}ছ"),
            ("ay", "Cyprus"),
            ("az", "Kipr"),
            ("ba", "Cyprus"),
            ("be", "Кіпр"),
            ("bg", "Кипър"),
            ("bi", "Cyprus"),
            ("bn", "স\u{9be}ইপ\u{9cd}র\u{9be}স"),
            ("bn_IN", "স\u{9be}ইপ\u{9cd}র\u{9be}স"),
            ("br", "Enez Chipr"),
            ("bs", "Kipar"),
            ("ca", "Xipre"),
            ("ce", "Кипр"),
            ("ch", "Cyprus"),
            ("cs", "Kypr"),
            ("cv", "Кипр"),
            ("cy", "Cyprus"),
            ("da", "Cypern"),
            ("de", "Zypern"),
            ("dv", "Cyprus"),
            ("dz", "སའ\u{f72}་པར\u{f71}ས\u{f72}།"),
            ("ee", "Cyprus"),
            ("el", "Κύπρος"),
            ("en", "Cyprus"),
            ("eo", "Kipro"),
            ("es", "Chipre"),
            ("et", "Küpros"),
            ("eu", "Zipre"),
            ("fa", "قبرس"),
            ("ff", "Kibris"),
            ("fi", "Kypros"),
            ("fo", "Kýpros"),
            ("fr", "Chypre"),
            ("fy", "Syprus"),
            ("ga", "An Chipir"),
            ("gl", "Chipre"),
            ("gn", "Cyprus"),
            ("gu", "સાયપ\u{acd}રસ"),
            ("gv", "Yn Cheeprey"),
            ("ha", "Cyprus"),
            ("he", "קפריסין"),
            ("hi", "साइप\u{94d}रस"),
            ("hr", "Cipar"),
            ("ht", "Chip"),
            ("hu", "Ciprus"),
            ("hy", "Կիպրոս"),
            ("ia", "Cypro"),
            ("id", "Siprus"),
            ("io", "Chipro"),
            ("is", "Kýpur"),
            ("it", "Cipro"),
            ("iu", "Cyprus"),
            ("ja", "キプロス"),
            ("ka", "კვიპროსი"),
            ("ki", "Cyprus"),
            ("kk", "Кипр"),
            ("kl", "Cyprus"),
            ("km", "ស\u{17ca}\u{17b8}ពរ\u{17cd}"),
            ("kn", "ಸೈಪ\u{ccd}ರಸ\u{ccd}"),
            ("ko", "키프로스"),
            ("ku", "Kibris"),
            ("kv", "Кипр"),
            ("kw", "Kobros"),
            ("ky", "Кипр"),
            ("lo", "ປະເທດຊ\u{eb5}ບ"),
            ("lt", "Kipras"),
            ("lv", "Kipra"),
            ("mi", "Haipara"),
            ("mk", "Кипар"),
            ("ml", "സൈപ\u{d4d}രസ\u{d4d}"),
            ("mn", "Кипр"),
            ("mr", "सायप\u{94d}रस"),
            ("ms", "Cyprus"),
            ("mt", "Ċipru"),
            (
                "my",
                "ဆ\u{102d}\u{102f}က\u{103a}ပရပ\u{103a}စ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Taiprus"),
            ("nb", "Kypros"),
            ("ne", "साइप\u{94d}रस"),
            ("nl", "Cyprus"),
            ("nn", "Kypros"),
            ("nv", "Béésh Łichíiʼii Bikéyah"),
            ("oc", "Chipre"),
            ("or", "ସ\u{b3e}ଇପ\u{b4d}ରସ"),
            ("pa", "ਕਿਊਰ\u{a42}ਸ"),
            ("pi", "सायप\u{94d}रस"),
            ("pl", "Cypr"),
            ("ps", "Cyprus"),
            ("pt", "Chipre"),
            ("pt_BR", "Chipre"),
            ("ro", "Cipru"),
            ("ru", "Кипр"),
            ("rw", "Shipure"),
            ("sc", "Tzipru"),
            ("sd", "Cyprus"),
            ("si", "සය\u{dd2}ප\u{dca}\u{200d}රස\u{dca}"),
            ("sk", "Cyprus"),
            ("sl", "Ciper"),
            ("so", "Qabrus"),
            ("sq", "Qipro"),
            ("sr", "Кипар"),
            ("sv", "Cypern"),
            ("sw", "Cyprus"),
            ("ta", "சைப\u{bcd}ரஸ\u{bcd}"),
            ("te", "స\u{c48}ప\u{c4d}రస\u{c4d}"),
            ("tg", "Кипр"),
            ("th", "ไซปร\u{e31}ส"),
            ("ti", "ሳይፕረስ"),
            ("tk", "Kipr"),
            ("tl", "Cyprus"),
            ("tr", "Kıbrıs"),
            ("tt", "Кипер"),
            ("ug", "سىپرۇس"),
            ("uk", "Кіпр"),
            ("ur", "قبرص"),
            ("uz", "Qibris"),
            ("ve", "Cyprus"),
            ("vi", "Síp"),
            ("wa", "Chipe"),
            ("wo", "Ciipër"),
            ("xh", "Cyprus"),
            ("yo", "Kíprù"),
            ("zh_CN", "塞浦路斯"),
            ("zh_HK", "賽浦路斯"),
            ("zh_TW", "賽普勒斯"),
            ("zu", "Cyprus"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
