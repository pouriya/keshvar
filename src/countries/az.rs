// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Azerbaijan

#[cfg(all(feature = "az", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AZ;
    pub const ALPHA3: Alpha3 = Alpha3::AZE;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 994;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::AZN;
    pub const GEC: Option<GEC> = Some(GEC::AJ);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::AZE);
    pub const ISO_SHORT_NAME: &str = "Azerbaijan";
    pub const ISO_LONG_NAME: &str = "The Republic of Azerbaijan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["az", "hy"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["az", "hy"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Azerbaijani");
    pub const NUMBER: &str = "031";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "AZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Azerbaijan",
        "Aserbaidschan",
        "Azerbaïdjan",
        "Azerbaiyán",
        "アゼルバイジャン",
        "Azerbeidzjan",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Azerbaijan"),
        ("af", "Azerbaidjan"),
        ("ak", "Azerbaijan"),
        ("am", "ጐፈሴባጃን"),
        ("an", "Azerbaichán"),
        ("ar", "أذربيجان"),
        ("as", "আজেৰ\u{9cd}বেইজ\u{9be}ন"),
        ("ay", "Azerbaijan"),
        ("az", "Azərbaycan"),
        ("ba", "Azerbaijan"),
        ("be", "Азербайджан"),
        ("bg", "Азербайджан"),
        ("bi", "Azerbaijan"),
        ("bn", "আজ\u{9be}রব\u{9be}ইজ\u{9be}ন"),
        ("bn_IN", "আজ\u{9be}রব\u{9be}ইজ\u{9be}ন"),
        ("br", "Azerbaidjan"),
        ("bs", "Azerbejdžan"),
        ("ca", "Azerbaitjan"),
        ("ce", "Азербайджан"),
        ("ch", "Azerbaijan"),
        ("cs", "Ázerbájdžán"),
        ("cv", "Азербайджан"),
        ("cy", "Azerbaijan"),
        ("da", "Aserbajdsjan"),
        ("de", "Aserbaidschan"),
        (
            "dv",
            "އ\u{7a6}ޒ\u{7a6}ރ\u{7aa}ބ\u{7a6}އ\u{7a8}ޖ\u{7a7}ނ\u{7b0}",
        ),
        ("dz", "ཨ་ཛར་བ་ཡ\u{f7a}་ཇ\u{f71}ན།"),
        ("ee", "Azerbaijan"),
        ("el", "Αζερμπαϊτζάν"),
        ("en", "Azerbaijan"),
        ("eo", "Azerbajĝano"),
        ("es", "Azerbaiyán"),
        ("et", "Aserbaidžaan"),
        ("eu", "Azerbaijan"),
        ("fa", "آذربایجان"),
        ("ff", "Aserbayjan"),
        ("fi", "Azerbaidžan"),
        ("fo", "Aserbajdsjan"),
        ("fr", "Azerbaïdjan"),
        ("fy", "Azerbeidzjan"),
        ("ga", "An Asarbaiseáin"),
        ("gl", "Acerbaixán"),
        ("gn", "Azerbaijan"),
        ("gu", "અઝરબ\u{ac8}ઝાન"),
        ("gv", "Yn Asserbajaan"),
        ("ha", "Azerbaijan"),
        ("he", "אזרבייג׳ן"),
        ("hi", "अज\u{93c}रब\u{948}जान"),
        ("hr", "Azerbajdžan"),
        ("ht", "Azerbaydjan"),
        ("hu", "Azerbajdzsán"),
        ("hy", "Ադրբեջան"),
        ("ia", "Azerbaijan"),
        ("id", "Azerbaijan"),
        ("io", "Azerbaijan"),
        ("is", "Aserbaídsjan"),
        ("it", "Azerbaigian"),
        ("iu", "Azerbaijan"),
        ("ja", "アゼルバイジャン"),
        ("ka", "აზერბაიჯანი"),
        ("ki", "Azerbaijan"),
        ("kk", "Әзірбайжан"),
        ("kl", "Azerbaijan"),
        ("km", "អាហ\u{17d2}ស\u{17ca}ែរបែហ\u{17d2}សង\u{17cb}"),
        ("kn", "ಅಜರ\u{ccd}ಬೈಜಾನ\u{ccd}"),
        ("ko", "아제르바이잔"),
        ("ku", "Azerbaycan"),
        ("kv", "Азербайджан"),
        ("kw", "Azerbayjan"),
        ("ky", "Азербайжан"),
        ("lo", "ປະເທດອາແຊກບາຍຊ\u{eb1}ງ"),
        ("lt", "Azerbaidžanas"),
        ("lv", "Azerbaidžāna"),
        ("mi", "Atepaihānia"),
        ("mk", "Азербејџан"),
        ("ml", "അസര\u{d4d}\u{200d}ബൈജ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Азербайжан"),
        ("mr", "अझ\u{947}र\u{94d}ब\u{947}जान"),
        ("ms", "Azerbaijan"),
        ("mt", "Ażerbajġan"),
        (
            "my",
            "အဇာဘ\u{102d}\u{102f}င\u{103a}ဂျန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Aderbaidjan"),
        ("nb", "Aserbajdsjan"),
        ("ne", "अजरब\u{948}जान"),
        ("nl", "Azerbeidzjan"),
        ("nn", "Aserbajdsjan"),
        ("nv", "Azerbaijan"),
        ("oc", "Azerbaitjan"),
        ("or", "ଆଜର\u{b4d}ବେଜ\u{b3e}ନ"),
        ("pa", "ਅਜ\u{a3c}ਰਬਾਈਜਾਨ"),
        ("pi", "अजर\u{94d}ब\u{948}जान"),
        ("pl", "Azerbejdżan"),
        ("ps", "آزربایجان"),
        ("pt", "Azerbaijão"),
        ("pt_BR", "Azerbaidjão"),
        ("ro", "Azerbaijan"),
        ("ru", "Азербайджан"),
        ("rw", "Azeribayijani"),
        ("sc", "Azerbaigiàn"),
        ("sd", "Azerbaijan"),
        ("si", "අසර\u{dca}බය\u{dd2}ජ\u{dcf}න\u{dca}"),
        ("sk", "Azerbajdžan"),
        ("sl", "Azerbajdžan"),
        ("so", "Aserbiijaan"),
        ("sq", "Azerbajxhan"),
        ("sr", "Азербејџан"),
        ("sv", "Azerbajdzjan"),
        ("sw", "Azerbaijan"),
        ("ta", "அசர\u{bcd}பைச\u{bbe}ன\u{bcd}"),
        ("te", "అఝ\u{c47}ర\u{c4d}బ\u{c47}జ\u{c3e}న"),
        ("tg", "Озарбойҷон"),
        ("th", "อาเซอร\u{e4c}ไบจาน"),
        ("ti", "ኣዘርባጃን"),
        ("tk", "Azerbeýjan"),
        ("tl", "Azerbaijan"),
        ("tr", "Azerbaycan"),
        ("tt", "Әзәрбайҗан"),
        ("ug", "ئەزەربەيجان"),
        ("uk", "Азербайджан"),
        ("ur", "آذربائیجان"),
        ("uz", "Ozarbayjon"),
        ("ve", "Azerbaijan"),
        ("vi", "Ai-xợ-bai-gianh"),
        ("wa", "Azerbaydjan"),
        ("wo", "Aserbayjaan"),
        ("xh", "Azerbaijan"),
        ("yo", "Azerbaijan"),
        ("zh_CN", "阿塞拜疆"),
        ("zh_HK", "亞塞拜彊"),
        ("zh_TW", "亞塞拜然"),
        ("zu", "I-Azerbayijani"),
    ];
    #[cfg(all(feature = "az", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 40.143105;
        pub const LONGITUDE: f64 = 47.576927;
        pub const MAX_LATITUDE: f64 = 41.9594999;
        pub const MAX_LONGITUDE: f64 = 50.7458001;
        pub const MIN_LATITUDE: f64 = 38.3922171;
        pub const MIN_LONGITUDE: f64 = 44.7632599;
        pub const NORTHEAST_LATITUDE: f64 = 41.9594999;
        pub const NORTHEAST_LONGITUDE: f64 = 50.7458001;
        pub const SOUTHWEST_LATITUDE: f64 = 38.3922171;
        pub const SOUTHWEST_LONGITUDE: f64 = 44.7632599;
    }
}
#[cfg(all(feature = "az", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 40.143105,
            longitude: 47.576927,
            max_latitude: 41.9594999,
            max_longitude: 50.7458001,
            min_latitude: 38.3922171,
            min_longitude: 44.7632599,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 41.9594999,
                    longitude: 50.7458001,
                },
                southwest: CountryGeoBound {
                    latitude: 38.3922171,
                    longitude: 44.7632599,
                },
            },
        }
    }
}

#[cfg(all(feature = "az", feature = "subdivisions"))]
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
                    "ABS",
                    Subdivision{
                        name: "ABS",
                        country_alpha2: Alpha2::AZ,
                        code: "ABS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.3629693), longitude: Some(49.2736815), max_latitude: Some(40.638273), min_latitude: Some(39.9478741), max_longitude: Some(49.908777), min_longitude: Some(48.843646)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة آبشوران"), ("az", "Abşeron"), ("bn", "আবশেরন জেল\u{9be}"), ("ccp", "𑄃𑄛\u{11134}𑄥𑄬𑄢\u{11127}𑄚\u{11134}"), ("ceb", "Absheron Rayon"), ("cs", "Apšeron"), ("da", "Absheron District"), ("de", "Abşeron"), ("el", "Αμπσερόν"), ("en", "Absheron"), ("es", "Abşeron"), ("fa", "شهرستان آب\u{200c}شوران"), ("fi", "Apšeronin piirikunta"), ("fr", "Abşeron"), ("gu", "એબ\u{acd}શ\u{ac7}રોન જિલ\u{acd}લો"), ("hi", "एबश\u{947}रन जिला"), ("hu", "Abşeroni járás"), ("hy", "Ապշերոնի շրջան"), ("id", "Abşeron"), ("it", "distretto di Abşeron"), ("ja", "アブシェロン県"), ("kn", "ಅಬ\u{ccd}ಷ\u{cc6}ರಾನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "압셰론 구"), ("lt", "Abšerono apskritis"), ("lv", "Abšeronas rajons"), ("mk", "Апшерон"), ("mn", "Абшерон аймаг"), ("mr", "अब\u{94d}बीरॉन जिल\u{94d}हा"), ("ms", "Abşeron"), ("nb", "Abşeron"), ("nl", "Abşeron"), ("no", "Abşeron"), ("pl", "Rejon Abşeron"), ("pt", "Absheron"), ("ru", "Апшеронский район"), ("si", "අබ\u{dca}ශේරොන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sl", "Abşeron"), ("sq", "Absheron"), ("sr", "Апшеронски рејон"), ("sr_Latn", "Apšeronski rejon"), ("sv", "Apsjeron"), ("ta", "அபிஷெரோன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అబ\u{c4d}\u{200c}ష\u{c46}ర\u{c3e}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตแอบช\u{e35}รอน"), ("tr", "Abşeron Rayonu"), ("uk", "Апшеронський район"), ("ur", "آب شیرون ضلع"), ("vi", "Absheron"), ("zh", "阿布歇隆區")]),
                        unofficial_name_list: ["Abseron"].to_vec(),
                    }
                ),
                (
                    "AGA",
                    Subdivision{
                        name: "AGA",
                        country_alpha2: Alpha2::AZ,
                        code: "AGA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.118889), longitude: Some(45.453889), max_latitude: Some(41.1373284), min_latitude: Some(41.09837), max_longitude: Some(45.48357), min_longitude: Some(45.4031037)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أقستافا"), ("az", "Ağstafa"), ("bn", "আগস\u{9cd}ত\u{9be}প\u{9be} জেল\u{9be}"), ("ccp", "𑄃𑄇\u{11134}𑄥\u{11133}𑄑𑄜"), ("ceb", "Aghstafa Rayon"), ("cs", "Agstafa"), ("da", "Agstafa District"), ("de", "Ağstafa"), ("el", "Αγκστάφα"), ("en", "Agstafa"), ("es", "Ağstafa"), ("fa", "شهرستان آق\u{200c}استفا"), ("fi", "Ağstafan piirikunta"), ("fr", "Aghstafa"), ("gu", "અગસ\u{acd}તાફા જિલ\u{acd}લો"), ("hi", "अस\u{94d}ताफा जिला"), ("hu", "Ağstafai járás"), ("hy", "Աղստևի շրջան"), ("id", "Agstafa"), ("it", "distretto di Ağstafa"), ("ja", "アグスタファ県"), ("kn", "ಅಗಸ\u{ccd}ಟಾ ಜ\u{cbf}ಲ\u{ccd}ಲಾ"), ("ko", "아흐스타파 구"), ("lt", "Akstafos apskritis"), ("lv", "Agstafas rajons"), ("mk", "Агстафа"), ("mn", "Акстафа аймаг"), ("mr", "अस\u{94d}तास\u{94d}ता जिल\u{94d}हा"), ("ms", "Daerah Agstafa"), ("nb", "Ağstafa"), ("nl", "Ağstafa"), ("no", "Ağstafa"), ("pl", "Rejon Ağstafa"), ("pt", "Agstafa"), ("ru", "Акстафинский район"), ("si", "අග\u{dca}ස\u{dca}ටෆ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Agstafa"), ("sr", "Акстафински рејон"), ("sr_Latn", "Akstafinski rejon"), ("sv", "Ağstafa Rayonu"), ("ta", "அஃஸ\u{bcd}த\u{bbe}ப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అగ\u{c4d}స\u{c4d}\u{200c}ట\u{c3e}ఫ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อ\u{e31}กสทาฟา"), ("tr", "Ağstafa Rayonu"), ("uk", "Агстафінський район"), ("ur", "اغستافا ضلع"), ("vi", "Aghstafa"), ("zh", "阿克斯塔法區")]),
                        unofficial_name_list: ["Agstafa"].to_vec(),
                    }
                ),
                (
                    "AGC",
                    Subdivision{
                        name: "AGC",
                        country_alpha2: Alpha2::AZ,
                        code: "AGC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.0257239), longitude: Some(47.3290937), max_latitude: Some(40.248541), min_latitude: Some(39.702849), max_longitude: Some(47.735411), min_longitude: Some(47.0597249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة آقجبدي"), ("az", "Ağcabədi"), ("bn", "আগজ\u{9be}ব\u{9be}দি জেল\u{9be}"), ("ccp", "𑄃𑄇\u{11134}𑄎𑄝𑄘\u{11128}"), ("ceb", "Aghjabadi Rayon"), ("cs", "Agdžabadi"), ("da", "Aghjabadi District"), ("de", "Ağcabədi"), ("el", "Αγκτζαμπάντι"), ("en", "Aghjabadi"), ("es", "Ağcabədi"), ("fa", "شهرستان آقجه\u{200c}بدی"), ("fi", "Ağcabədin piirikunta"), ("fr", "Ağcabədi"), ("gu", "અગ\u{acd}જાબાદી જિલ\u{acd}લો"), ("hi", "एग\u{94d}जाब\u{947}ड\u{93c}ी जिला"), ("hu", "Ağcabədi járás"), ("hy", "Աղջաբեդի շրջան"), ("id", "Agjabadi"), ("it", "distretto di Ağcabədi"), ("ja", "アグジャバディ県"), ("kn", "ಅಜ\u{ccd}ಜಬಾದ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아그자베디 구"), ("lt", "Agdžabadžio apskritis"), ("lv", "Agdžabedi rajons"), ("mk", "Агџабеди"), ("mn", "Агжабеди аймаг"), ("mr", "अगजाबादी जिल\u{94d}हा"), ("ms", "Daerah Agjabadi"), ("nb", "Aghjabadi distrikt"), ("nl", "Ağcabədi"), ("no", "Aghjabadi distrikt"), ("pl", "Rejon Ağcabədi"), ("pt", "Aghjabadi"), ("ru", "Агджабединский район"), ("si", "අග\u{dcf}හ\u{dca}ජබ\u{dcf}ද\u{dd3} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Aghjabadi"), ("sr", "Агџабедински рејон"), ("sr_Latn", "Agdžabedinski rejon"), ("sv", "Ağcabədi"), ("ta", "அஃஜபடி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అగ\u{c4d}జ\u{c3e}బ\u{c3e}ద\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e31}จจาบาด\u{e34}"), ("tr", "Ağcabedi Rayonu"), ("uk", "Агджабединський район"), ("ur", "اغجابادی ضلع"), ("vi", "Aghjabedi"), ("zh", "阿格賈貝迪區")]),
                        unofficial_name_list: ["Agcabädi"].to_vec(),
                    }
                ),
                (
                    "AGM",
                    Subdivision{
                        name: "AGM",
                        country_alpha2: Alpha2::AZ,
                        code: "AGM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5554533), longitude: Some(46.9507608), max_latitude: Some(39.5579602), min_latitude: Some(39.5526992), max_longitude: Some(46.95384989999999), min_longitude: Some(46.9491719)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة آقدام"), ("az", "Ağdam"), ("bn", "আগড\u{9be}ম জেল\u{9be}"), ("ccp", "𑄃𑄇\u{11134}𑄘𑄟\u{11134}"), ("ceb", "Aghdam Rayon"), ("cs", "Agdam"), ("da", "Agdam District"), ("de", "Ağdam"), ("el", "Αγκντάμ"), ("en", "Agdam"), ("es", "Ağdam"), ("fa", "شهرستان آق\u{200c}دام"), ("fi", "Ağdamin piirikunta"), ("fr", "Agdam"), ("gu", "અગામ જિલ\u{acd}લો"), ("hi", "एगड\u{947}म जिला"), ("hu", "Ağdami járás"), ("id", "Agdam"), ("it", "distretto di Ağdam"), ("ja", "アグダム県"), ("ka", "აღდამის რაიონი"), ("kn", "ಅಗಮ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아그담 구"), ("lt", "Agdamo provincija"), ("lv", "Agdamas rajons"), ("mk", "Агдам"), ("mn", "Агдам аймаг"), ("mr", "आगडम जिल\u{94d}हा"), ("ms", "Agdam"), ("nb", "Ağdam"), ("nl", "Ağdam"), ("no", "Ağdam"), ("pl", "Rejon Ağdam"), ("pt", "Agdam"), ("ru", "Агдамский район"), ("si", "අග\u{dca}ඩම\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Agdam"), ("sr", "Агдамски рејон"), ("sr_Latn", "Agdamski rejon"), ("sv", "Ağdam"), ("ta", "அக\u{bcd}த\u{bbe}ம\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఆగ\u{c4d}డమ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ควาน อ\u{e31}คด\u{e31}ม"), ("tr", "Ağdam Rayonu"), ("uk", "Агдамський район"), ("ur", "اغدام ضلع"), ("uz", "Agʻdam tumani"), ("vi", "Quận Agdam"), ("zh", "阿格達姆區")]),
                        unofficial_name_list: ["Agdam"].to_vec(),
                    }
                ),
                (
                    "AGS",
                    Subdivision{
                        name: "AGS",
                        country_alpha2: Alpha2::AZ,
                        code: "AGS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6335427), longitude: Some(47.467431), max_latitude: Some(40.6659586), min_latitude: Some(40.6229434), max_longitude: Some(47.5069858), min_longitude: Some(47.4376344)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أقداش"), ("az", "Ağdaş"), ("bn", "আগড\u{9be}স জেল\u{9be}"), ("ccp", "𑄃𑄇\u{11134}𑄘\u{11133}𑄠𑄌\u{11134}"), ("ceb", "Aghdash Rayon"), ("cs", "Agdaš"), ("da", "Agdash District"), ("de", "Ağdaş"), ("el", "Αγκντάς"), ("en", "Agdash"), ("es", "Ağdaş"), ("fa", "شهرستان آق\u{200c}داش"), ("fi", "Ağdaşin piirikunta"), ("fr", "Ağdaş"), ("gu", "અગદાશ જિલ\u{acd}લો"), ("hi", "एगड\u{948}श जिला"), ("hu", "Ağdaşi járás"), ("hy", "Աղդաշի շրջան"), ("id", "Agdash"), ("it", "distretto di Ağdaş"), ("ja", "アグダシュ県"), ("kn", "ಅಗ\u{ccd}ದಾಶ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아그다슈 구"), ("lt", "Agdašo apskritis"), ("lv", "Agdašas rajons"), ("mk", "Агдаш"), ("mn", "Агдаш аймаг"), ("mr", "आगादास जिल\u{94d}हा"), ("ms", "Daerah Agdash"), ("nb", "Ağdaş"), ("nl", "Ağdaş"), ("no", "Ağdaş"), ("pl", "Rejon Ağdaş"), ("pt", "Agdash"), ("ru", "Агдашский район"), ("si", "අග\u{dca}ද\u{dcf}ෂ\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Agdash"), ("sr", "Агдашки рејон"), ("sr_Latn", "Agdaški rejon"), ("sv", "Aghdasj"), ("ta", "அக\u{bcd}த\u{bbe}ஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అగ\u{c4d}ద\u{c3e}ష\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อ\u{e31}กแดช"), ("tr", "Ağdaş Rayonu"), ("uk", "Агдаський район"), ("ur", "آقداش ضلع"), ("vi", "Aghdash"), ("zh", "阿格達什區")]),
                        unofficial_name_list: ["Agdas"].to_vec(),
                    }
                ),
                (
                    "AGU",
                    Subdivision{
                        name: "AGU",
                        country_alpha2: Alpha2::AZ,
                        code: "AGU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.569167), longitude: Some(48.400833), max_latitude: Some(40.587429), min_latitude: Some(40.553135), max_longitude: Some(48.4195375), min_longitude: Some(48.3700133)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أقسو"), ("az", "Ağsu"), ("bn", "আগসো জেল\u{9be}"), ("ccp", "𑄃𑄇\u{11134}𑄥\u{1112a}"), ("ceb", "Aghsu Rayon"), ("cs", "Agsu"), ("da", "Agsu"), ("de", "Ağsu"), ("el", "Άγκσου"), ("en", "Agsu"), ("es", "Ağsu"), ("fa", "شهرستان آق\u{200c}سو"), ("fi", "Ağsun piirikunta"), ("fr", "Ağsu"), ("gu", "અગસ\u{ac1} જિલ\u{acd}લો"), ("hi", "एग\u{94d}स\u{942} जिला"), ("hy", "Աղսուի շրջան"), ("id", "Agsu"), ("it", "distretto di Ağsu"), ("ja", "アグス県"), ("ka", "აღსუს რაიონი"), ("kn", "ಅಗಸು ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아흐수 구"), ("lt", "Agsu apskritis"), ("lv", "Agsu rajons"), ("mk", "Агсу"), ("mn", "Агсу аймаг"), ("mr", "अग\u{941}स\u{941} जिल\u{94d}हा"), ("ms", "Agsu"), ("nb", "Ağsu"), ("nl", "Ağsu"), ("no", "Ağsu"), ("pl", "Rejon Ağsu"), ("pt", "Agsu"), ("ru", "Ахсуйский район"), ("si", "අග\u{dca}ස\u{dd4}"), ("sq", "Agsu"), ("sr", "Ахсујски рејон"), ("sr_Latn", "Ahsujski rejon"), ("sv", "Ağsu Rayonu"), ("ta", "அக\u{bcd}சு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అగ\u{c4d}సు జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e31}กซ\u{e39}"), ("tr", "Ağsu Rayonu"), ("uk", "Ахсуйський район"), ("ur", "اغسو ضلع"), ("vi", "Aghsu"), ("zh", "阿赫蘇區")]),
                        unofficial_name_list: ["Agsu"].to_vec(),
                    }
                ),
                (
                    "AST",
                    Subdivision{
                        name: "AST",
                        country_alpha2: Alpha2::AZ,
                        code: "AST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.45611100000001), longitude: Some(48.878611), max_latitude: Some(38.4936713), min_latitude: Some(38.4422285), max_longitude: Some(48.8811778), min_longitude: Some(48.8535404)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة آستارا"), ("az", "Astara"), ("bn", "আস\u{9cd}ত\u{9be}র\u{9be} জেল\u{9be}"), ("ca", "Districte d’Astara"), ("ccp", "𑄃𑄌\u{11134}𑄖𑄢"), ("ceb", "Astara District"), ("cs", "Astara"), ("da", "Astara District"), ("de", "Astara"), ("el", "Ασταρά"), ("en", "Astara"), ("es", "Astara"), ("fa", "شهرستان آستارا"), ("fi", "Astaran piirikunta"), ("fr", "Astara"), ("gu", "એસ\u{acd}ટારા જિલ\u{acd}લો"), ("hi", "अस\u{94d}तारा जिला"), ("hy", "Աստարայի շրջան"), ("id", "Astara"), ("it", "distretto di Astara"), ("ja", "アスタラ県"), ("kn", "ಅಸ\u{ccd}ತರಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아스타라 구"), ("lt", "Astaros apskritis"), ("lv", "Astaras rajons"), ("mk", "Астара"), ("mn", "Астара аймаг"), ("mr", "अतारा जिल\u{94d}हा"), ("ms", "Astara"), ("nb", "Astara"), ("nl", "Astara"), ("no", "Astara"), ("pl", "Rejon Astara"), ("pt", "Astara"), ("ru", "Астаринский район"), ("si", "අස\u{dca}ටර\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Astara"), ("sr", "Астарински рејон"), ("sr_Latn", "Astarinski rejon"), ("sv", "Astara (distrikt)"), ("ta", "அஸ\u{bcd}ட\u{bcd}ர\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అస\u{c4d}ట\u{c3e}ర\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องอ\u{e31}สทารา"), ("tr", "Astara Rayonu"), ("uk", "Астаринський район"), ("ur", "آستارا ضلع"), ("vi", "Astara"), ("zh", "阿斯塔拉區")]),
                        unofficial_name_list: ["Astara"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::AZ,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.40926169999999), longitude: Some(49.8670924), max_latitude: Some(40.486602), min_latitude: Some(40.3039993), max_longitude: Some(50.0561143), min_longitude: Some(49.6538049)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bakoe"), ("am", "ባኩ"), ("ar", "باكو"), ("az", "Bakı"), ("be", "Баку"), ("bg", "Баку"), ("bn", "ব\u{9be}ক\u{9c1}"), ("bs", "Baku"), ("ca", "Bakú"), ("ccp", "𑄝𑄇\u{1112a}"), ("ceb", "Baku City"), ("cs", "Baku"), ("cy", "Baku"), ("da", "Baku"), ("de", "Baku"), ("el", "Μπακού"), ("en", "Baku"), ("es", "Bakú"), ("et", "Bakuu"), ("eu", "Baku"), ("fa", "باکو"), ("fi", "Baku"), ("fr", "Bakou"), ("ga", "Bakı"), ("gl", "Bakú"), ("gu", "બાક\u{ac2}"), ("he", "באקו"), ("hi", "बाक\u{942}"), ("hr", "Baku"), ("hu", "Baku"), ("hy", "Բաքու"), ("id", "Baku"), ("is", "Bakú"), ("it", "Baku"), ("ja", "バクー"), ("jv", "Baku"), ("ka", "ბაქო"), ("kk", "Баку"), ("kn", "ಬಾಕು"), ("ko", "바쿠"), ("ky", "Баку"), ("lt", "Baku"), ("lv", "Baku"), ("mk", "Баку"), ("ml", "ബക\u{d4d}ക\u{d41}"), ("mn", "Баку"), ("mr", "बाक\u{942}"), ("ms", "Baku"), ("my", "ဗားက\u{1030}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Baku"), ("ne", "बाक\u{941}"), ("nl", "Bakoe"), ("no", "Baku"), ("or", "ବ\u{b3e}କ\u{b41}"), ("pa", "ਬਾਕ\u{a42}"), ("pl", "Baku"), ("ps", "باکو"), ("pt", "Baku"), ("ro", "Baku"), ("ru", "Баку"), ("si", "බ\u{dcf}ක\u{dd4}"), ("sk", "Baku"), ("sl", "Baku"), ("sq", "Baku"), ("sr", "Баку"), ("sr_Latn", "Baku"), ("sv", "Baku"), ("sw", "Baku"), ("ta", "ப\u{bbe}கு"), ("te", "బ\u{c3e}కూ"), ("th", "บาก\u{e39}"), ("tk", "Baku"), ("tr", "Bakü"), ("uk", "Баку"), ("ur", "باکو"), ("uz", "Boku"), ("vi", "Baku"), ("yo", "Baku"), ("yo_BJ", "Baku"), ("yue", "巴庫"), ("yue_Hans", "巴库"), ("zh", "巴库")]),
                        unofficial_name_list: ["Baki"].to_vec(),
                    }
                ),
                (
                    "BAB",
                    Subdivision{
                        name: "BAB",
                        country_alpha2: Alpha2::AZ,
                        code: "BAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.1507613), longitude: Some(45.4485368), max_latitude: Some(39.1675349), min_latitude: Some(39.1335888), max_longitude: Some(45.46829229999999), min_longitude: Some(45.4309559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بابيك"), ("az", "Babək"), ("bn", "ব\u{9be}বেক জেল\u{9be}"), ("ca", "Babək"), ("ccp", "𑄝𑄝𑄬𑄇\u{11134}"), ("ceb", "Babek Rayon"), ("da", "Babek District"), ("de", "Babək"), ("el", "Μπάμπεκ"), ("en", "Babek"), ("es", "Babek"), ("fa", "شهرستان بابک"), ("fi", "Babəkin piirikunta"), ("fr", "Babek"), ("gu", "બાબ\u{ac7}ક જિલ\u{acd}લો"), ("hi", "बाब\u{947}क जिला"), ("hy", "Բաբեկի շրջան"), ("id", "Babek"), ("it", "Distretto di Babak"), ("ja", "バベク県"), ("kn", "ಬಾಬ\u{cc6}ಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "바베크 구"), ("lt", "Babeko apskritis"), ("lv", "Babekas rajons"), ("mk", "Бабек"), ("mr", "बाम\u{94d}ब जिल\u{94d}हा"), ("ms", "Babek"), ("nb", "Babək"), ("nl", "Babək"), ("no", "Babək"), ("pl", "Rejon Babək"), ("pt", "Babek"), ("ru", "Бабекский район"), ("si", "බබෙක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Бабечки рејон"), ("sr_Latn", "Babečki rejon"), ("sv", "Babek Rayon"), ("ta", "ப\u{bbe}பேக\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c3e}బ\u{c46}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบาเบคร\u{e4c}"), ("tr", "Babek Rayonu"), ("uk", "Бабецький район"), ("ur", "بابک ضلع"), ("vi", "Babek"), ("zh", "巴貝克區")]),
                        unofficial_name_list: ["Babäk"].to_vec(),
                    }
                ),
                (
                    "BAL",
                    Subdivision{
                        name: "BAL",
                        country_alpha2: Alpha2::AZ,
                        code: "BAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.725833), longitude: Some(46.408333), max_latitude: Some(41.7560107), min_latitude: Some(41.6622043), max_longitude: Some(46.4600659), min_longitude: Some(46.3497733)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بالاكن"), ("az", "Balakən"), ("bn", "ব\u{9be}ল\u{9be}ক\u{9be}ন জেল\u{9be}"), ("ccp", "𑄝\u{11127}𑄣𑄇𑄚\u{11134}"), ("ceb", "Balakan Rayon"), ("cs", "Balakan"), ("da", "Balakan District"), ("de", "Balakən"), ("el", "Μπαλακάν"), ("en", "Balakan"), ("es", "Balakən"), ("fa", "شهرستان بالاکن"), ("fi", "Balakənin piirikunta"), ("fr", "Balakən"), ("gu", "બાલાકન જિલ\u{acd}લો"), ("hi", "बालकन ज\u{93c}िला"), ("hu", "Balakəni járás"), ("id", "Balakan"), ("it", "distretto di Balakən"), ("ja", "バラキャン県"), ("ka", "ბელაქანის რაიონი"), ("kn", "ಬಾಲಕನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "발라켄 구"), ("lt", "Balakano apskritis"), ("lv", "Balakenas rajons"), ("mk", "Балакен"), ("mn", "Балакен аймаг"), ("mr", "बालाकण जिल\u{94d}हा"), ("ms", "Balakan"), ("nb", "Balakən"), ("nl", "Balakən"), ("no", "Balakən"), ("pl", "Rejon Balakən"), ("pt", "Balakan"), ("ru", "Белоканский район"), ("si", "බලකන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Balakan"), ("sr", "Белокански рејон"), ("sr_Latn", "Belokanski rejon"), ("sv", "Balakən Rayonu"), ("ta", "ப\u{bbe}லகன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c3e}ల\u{c3e}కన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "บาลาคาน"), ("tr", "Balaken Rayonu"), ("uk", "Белоканський район"), ("ur", "بالاکن ضلع"), ("vi", "Balaken"), ("zh", "巴拉肯區")]),
                        unofficial_name_list: ["Balakän"].to_vec(),
                    }
                ),
                (
                    "BAR",
                    Subdivision{
                        name: "BAR",
                        country_alpha2: Alpha2::AZ,
                        code: "BAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.3667), longitude: Some(47.11669999999999), max_latitude: Some(40.405915), min_latitude: Some(40.3447124), max_longitude: Some(47.1718597), min_longitude: Some(47.09220879999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بردع"), ("az", "Bərdə"), ("bn", "ব\u{9be}রদ\u{9be} জেল\u{9be}"), ("ccp", "𑄝𑄢\u{11134}𑄘"), ("ceb", "Barda Rayon"), ("cs", "Barda"), ("da", "Barda District"), ("de", "Bərdə"), ("el", "Μπάρντα"), ("en", "Barda"), ("es", "Bərdə"), ("fa", "شهرستان بردع"), ("fi", "Bərdən piirikunta"), ("fr", "Bərdə"), ("gu", "બરડા જિલ\u{acd}લો"), ("hi", "बर\u{94d}दा जिला"), ("id", "Barda"), ("it", "distretto di Bərdə"), ("ja", "バルダ県"), ("ka", "ბარდის რაიონი"), ("kn", "ಬರ\u{ccd}ಡಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "베르데 구"), ("lt", "Bardos apskritis"), ("lv", "Bardas rajons"), ("mk", "Барда"), ("mn", "Барде аймаг"), ("mr", "बारडा जिल\u{94d}हा"), ("ms", "Barda"), ("nb", "Bərdə"), ("nl", "Bərdə"), ("no", "Bərdə"), ("pl", "Rejon Bərdə"), ("pt", "Barda"), ("ru", "Бардинский район"), ("si", "බර\u{dca}ඩ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Barda"), ("sr", "Бардински рејон"), ("sr_Latn", "Bardinski rejon"), ("sv", "Bərdə Rayonu"), ("ta", "பர\u{bcd}த\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c3e}ర\u{c4d}డ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบาด\u{e49}า"), ("tr", "Berde Rayonu"), ("uk", "Район Барда"), ("ur", "بردع ضلع"), ("vi", "Barda"), ("zh", "巴爾達區")]),
                        unofficial_name_list: ["Bärdä"].to_vec(),
                    }
                ),
                (
                    "BEY",
                    Subdivision{
                        name: "BEY",
                        country_alpha2: Alpha2::AZ,
                        code: "BEY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.766667), longitude: Some(47.616667), max_latitude: Some(39.7852244), min_latitude: Some(39.7545479), max_longitude: Some(47.6407099), min_longitude: Some(47.58363250000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيلقان"), ("az", "Beyləqan"), ("bn", "বেল\u{9be}গ\u{9be}ন জেল\u{9be}"), ("ccp", "𑄝𑄬𑄣\u{11127}𑄉\u{11127}𑄚\u{11134}"), ("ceb", "Beylagan Rayon"), ("cs", "Bejlagan"), ("da", "Beylagan District"), ("de", "Beyləqan"), ("el", "Μπεϊλαγκάν"), ("en", "Beylagan"), ("es", "Beyləqan"), ("fa", "شهرستان بیلقان"), ("fi", "Beyləqanin piirikunta"), ("fr", "Beyləqan"), ("gu", "બ\u{ac7}લ\u{ac7}ગન જિલ\u{acd}લો"), ("he", "ביילגאן"), ("hi", "ब\u{947}लागन जिला"), ("id", "Beylagan"), ("it", "distretto di Beyləqan"), ("ja", "ベイラガン県"), ("kn", "ಬೈಲಾಗನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "베일레간 구"), ("lt", "Bejlagano apskritis"), ("lv", "Bejleganas rajons"), ("mk", "Бејлеган"), ("mn", "Бейлеган аймаг"), ("mr", "ब\u{947}लागान जिल\u{94d}हा"), ("ms", "Beylagan"), ("nb", "Beyləqan"), ("nl", "Beyləqan"), ("no", "Beyləqan"), ("pl", "Rejon Beyləqan"), ("pt", "Beylagan"), ("ru", "Бейлаганский район"), ("si", "බෙල\u{dcf}ගන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Beylagan"), ("sr", "Бејлагански рејон"), ("sr_Latn", "Bejlaganski rejon"), ("sv", "Bejläqan"), ("ta", "பெய\u{bcd}லகன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c47}ల\u{c3e}గన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเบย\u{e4c}ลาแกน"), ("tr", "Beylegan Rayonu"), ("uk", "Бейлаганський район"), ("ur", "بیلقان ضلع"), ("vi", "Beilagan"), ("zh", "貝拉甘區")]),
                        unofficial_name_list: ["Beyläqan"].to_vec(),
                    }
                ),
                (
                    "BIL",
                    Subdivision{
                        name: "BIL",
                        country_alpha2: Alpha2::AZ,
                        code: "BIL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.45), longitude: Some(48.533333), max_latitude: Some(39.4795993), min_latitude: Some(39.4349998), max_longitude: Some(48.5670376), min_longitude: Some(48.5262895)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيلاسوار"), ("az", "Biləsuvar rayonu"), ("bn", "বিল\u{9be}সোব\u{9be}র জেল\u{9be}"), ("ccp", "𑄝\u{11128}𑄣𑄥\u{1112a}𑄞𑄢\u{11134}"), ("ceb", "Bilasuvar Rayon"), ("cs", "Bilasuvar"), ("da", "Bilasuvar District"), ("de", "Biləsuvar"), ("el", "Μπιλασουβάρ"), ("en", "Bilasuvar"), ("es", "Biləsuvar"), ("fa", "شهرستان بیله\u{200c}سوار"), ("fi", "Biləsuvarin piirikunta"), ("fr", "Biləsuvar"), ("gu", "બિલાસ\u{ac1}વર જિલ\u{acd}લો"), ("hi", "बिलास\u{941}वार जिला"), ("id", "Bilasuvar"), ("it", "distretto di Biləsuvar"), ("ja", "ビラスヴァル県"), ("kn", "ಬ\u{cbf}ಲಸುವರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "빌레수바르 구"), ("lt", "Bilasuvaro apskritis"), ("lv", "Bilesuvaras rajons"), ("mk", "Биљасувар"), ("mn", "Билесувар аймаг"), ("mr", "बिलास\u{941}वर जिल\u{94d}हा"), ("ms", "Bilasuvar"), ("nb", "Biləsuvar"), ("nl", "Biləsuvar"), ("no", "Biləsuvar"), ("pl", "Rejon Biləsuvar"), ("pt", "Bilasuvar"), ("ru", "Билясуварский район"), ("si", "බ\u{dd2}ලස\u{dd4}ව\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Биљасуварски рејон"), ("sr_Latn", "Biljasuvarski rejon"), ("sv", "Biläsuvar"), ("ta", "பிள\u{bbe}சுவர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c3f}ల\u{c3e}సువర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบ\u{e35}ละซ\u{e39}วาร\u{e4c}"), ("tr", "Bilesuvar rayonu"), ("uk", "Білясуварський район"), ("ur", "بیلاسوار ضلع"), ("vi", "Bilasuvar"), ("zh", "比利亞蘇瓦爾區")]),
                        unofficial_name_list: ["Biläsuvar"].to_vec(),
                    }
                ),
                (
                    "CAB",
                    Subdivision{
                        name: "CAB",
                        country_alpha2: Alpha2::AZ,
                        code: "CAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2645544), longitude: Some(46.9621561), max_latitude: Some(39.478561), min_latitude: Some(39.132511), max_longitude: Some(47.27655410000001), min_longitude: Some(46.692907)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جبرائيل"), ("az", "Cəbrayıl"), ("bn", "জ\u{9be}ব\u{9cd}র\u{9be}ইল জেল\u{9be}"), ("ccp", "𑄎𑄝\u{11133}𑄢\u{1112d}𑄣\u{11134}"), ("ceb", "Cəbrayıl Rayonu"), ("da", "Jabrayil District"), ("de", "Cəbrayıl"), ("el", "Τζαμπραγίλ"), ("en", "Jabrayil"), ("es", "Cəbrayıl"), ("fa", "شهرستان جبراییل"), ("fi", "Cəbrayılin piirikunta"), ("fr", "Jabrayil"), ("gu", "જબરાયિલ જિલ\u{acd}લો"), ("hi", "जाब\u{94d}र\u{947}इल जिला"), ("hu", "Cəbrayıl járás"), ("id", "Jabrayil"), ("it", "distretto di Cəbrayıl"), ("ja", "ジャブライル県"), ("kn", "ಜಬ\u{ccd}ರೇಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "제브라이을 구"), ("lt", "Džebrailo apskritis"), ("lv", "Džebrajilas rajons"), ("mk", "Џебраил"), ("mn", "Жабраил аймаг"), ("mr", "जाबरॉय जिल\u{94d}हा"), ("ms", "Jabrayil"), ("nb", "Cəbrayıl"), ("nl", "Cəbrayıl"), ("no", "Cəbrayıl"), ("pl", "Rejon Cəbrayıl"), ("pt", "Jabrayil"), ("ru", "Джебраильский район"), ("si", "ජබ\u{dca}\u{200d}රය\u{dd2}ල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Jabrayil"), ("sr", "Џебрајиљски рејон"), ("sr_Latn", "Džebrajiljski rejon"), ("sv", "Cəbrayıl Rayonu"), ("ta", "ஜப\u{bcd}ர\u{bbe}யில\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జ\u{c3e}బ\u{c4d}ర\u{c3e}య\u{c3f}ల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดจาบราย\u{e34}ล"), ("tr", "Cebrayıl Rayonu"), ("uk", "Джебраїльський район"), ("ur", "جبراییل ضلع"), ("vi", "Jebrail"), ("zh", "傑布拉伊爾區")]),
                        unofficial_name_list: ["Cäbrayil"].to_vec(),
                    }
                ),
                (
                    "CAL",
                    Subdivision{
                        name: "CAL",
                        country_alpha2: Alpha2::AZ,
                        code: "CAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.218442), longitude: Some(48.4295146), max_latitude: Some(39.408813), min_latitude: Some(38.98947), max_longitude: Some(48.754075), min_longitude: Some(48.1295969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جليل\u{200c} آباد"), ("az", "Cəlilabad"), ("bn", "জ\u{9be}লিল\u{9be} ব\u{9be}দ জেল\u{9be}"), ("ccp", "𑄎𑄣\u{11128}𑄣𑄝𑄖\u{11134}"), ("ceb", "Jalilabad"), ("cs", "Džalilabad"), ("da", "Jalilabad District"), ("de", "Cəlilabad"), ("el", "Τζαλιλαμπάντ"), ("en", "Jalilabad"), ("es", "Cəlilabad"), ("fa", "شهرستان جلیل\u{200c}آباد"), ("fi", "Cəlilabadin piirikunta"), ("fr", "Cəlilabad"), ("gu", "જલિલાબાદ જિલ\u{acd}લો"), ("hi", "जलीलाबाद जिला"), ("hu", "Cəlilabadi járás"), ("id", "Jalilabad"), ("it", "distretto di Cəlilabad"), ("ja", "ジャリラバド県"), ("kn", "ಜಲೀಲಾಬಾದ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "젤릴라바트 구"), ("lt", "Džalilabado apskritis"), ("lv", "Dželilabadas rajons"), ("mk", "Џелилабад"), ("mn", "Жалилабад аймаг"), ("mr", "जलालाबाद जिल\u{94d}हा"), ("ms", "Jalilabad"), ("nb", "Cəlilabab"), ("nl", "Cəlilabad"), ("no", "Cəlilabab"), ("pl", "Rejon Cəlilabad"), ("pt", "Jalilabad"), ("ru", "Джалилабадский район"), ("si", "ජලය\u{dd2}ල\u{dcf}බ\u{dcf}ද\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Jalilabad"), ("sr", "Џалилабадски рејон"), ("sr_Latn", "Džalilabadski rejon"), ("sv", "Cəlilabad (distrikt)"), ("ta", "ஜல\u{bc0}ல\u{bbe}ப\u{bbe}த\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జల\u{c40}ల\u{c3e}బ\u{c3e}ద\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตจาล\u{e34}ลาบ\u{e31}ด"), ("tr", "Celilabad Rayonu"), ("uk", "Джалілабад"), ("ur", "جلیل آباد ضلع (آذربائیجان)"), ("vi", "Jalilabad"), ("zh", "賈利拉巴德區")]),
                        unofficial_name_list: ["Cälilabab"].to_vec(),
                    }
                ),
                (
                    "CUL",
                    Subdivision{
                        name: "CUL",
                        country_alpha2: Alpha2::AZ,
                        code: "CUL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.955833), longitude: Some(45.630833), max_latitude: Some(38.9734231), min_latitude: Some(38.9447447), max_longitude: Some(45.6617546), min_longitude: Some(45.605641)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "حي جلفا"), ("az", "Culfa rayonu"), ("be", "Джульфінскі раён"), ("bn", "জ\u{9c1}লফ\u{9be} জেল\u{9be}"), ("ccp", "𑄎\u{1112a}𑄣\u{11134}𑄜"), ("ceb", "Julfa Rayon"), ("da", "Julfa District"), ("de", "Culfa"), ("el", "Τζούλφα"), ("en", "Julfa"), ("es", "Julfa"), ("fa", "شهرستان جلفا"), ("fi", "Culfan piirikunta"), ("fr", "Djoulfa"), ("gu", "જ\u{ac1}લ\u{acd}ફા જિલ\u{acd}લો"), ("hi", "ज\u{942}ल\u{94d}फा जिला"), ("hy", "Ջուղայի շրջան"), ("id", "Julfa"), ("it", "Distretto di Culfa"), ("ja", "ジュルファ県"), ("ka", "ჯულფის რაიონი"), ("kn", "ಜುಲ\u{ccd}ಫಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "줄파 구"), ("lt", "Džulfos apskritis"), ("lv", "Džulfas rajons"), ("mk", "Џулфа"), ("mr", "ज\u{941}ल\u{94d}फा जिल\u{94d}हा"), ("ms", "Julfa"), ("nb", "Culfa"), ("nl", "Culfa"), ("no", "Culfa"), ("pl", "Rejon Culfa"), ("pt", "Julfa"), ("ru", "Джульфинский район"), ("si", "ජ\u{dd4}ල\u{dca}ෆ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Џулфински рејон"), ("sr_Latn", "Džulfinski rejon"), ("sv", "Julfa Rayon"), ("ta", "ஜூலப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జుల\u{c4d}ఫ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตจ\u{e38}ลฟา"), ("tr", "Culfa Rayonu"), ("uk", "Джульфінський район"), ("ur", "جلفا ضلع"), ("vi", "Julfa"), ("zh", "朱利法區")]),
                        unofficial_name_list: ["Culfa"].to_vec(),
                    }
                ),
                (
                    "DAS",
                    Subdivision{
                        name: "DAS",
                        country_alpha2: Alpha2::AZ,
                        code: "DAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.5202257), longitude: Some(46.0779304), max_latitude: Some(40.5287404), min_latitude: Some(40.5079263), max_longitude: Some(46.09107969999999), min_longitude: Some(46.0622406)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة داشكاسان"), ("az", "Daşkəsən"), ("bn", "ড\u{9be}সক\u{9be}স\u{9be}ন জেল\u{9be}"), ("ccp", "𑄓\u{11133}𑄠𑄌\u{11134}𑄇𑄥𑄚\u{11134}"), ("ceb", "Dashkasan Rayon"), ("cs", "Daškasan"), ("da", "Dashkasan District"), ("de", "Daşkəsən"), ("el", "Ντασκασάν"), ("en", "Dashkasan"), ("es", "Daşkəsən"), ("fa", "شهرستان داش\u{200c}کسن"), ("fi", "Daşkəsənin piirikunta"), ("fr", "Daşkəsən"), ("gu", "દશ\u{ac7}કાસન જિલ\u{acd}લો"), ("hi", "दशकासन जिला"), ("hu", "Daşkəsəni járás"), ("hy", "Դաշկեսանի շրջան"), ("id", "Dashkasan"), ("it", "distretto di Daşkəsən"), ("ja", "ダシュキャサン県"), ("kn", "ದಶಕಾಸನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "다슈케센 구"), ("lt", "Daškesano apskritis"), ("lv", "Daškesenas rajons"), ("mk", "Дашкесен"), ("mn", "Дашкесан аймаг"), ("mr", "दशकासन जिल\u{94d}हा"), ("ms", "Dashkasan"), ("nb", "Daşkəsən"), ("nl", "Daşkəsən"), ("no", "Daşkəsən"), ("pl", "Rejon Daşkəsən"), ("pt", "Dashkasan"), ("ru", "Дашкесанский район"), ("si", "ඩෂ\u{dca}කසන\u{dca}"), ("sq", "Dashkasan"), ("sr", "Дашкесански рејон"), ("sr_Latn", "Daškesanski rejon"), ("sv", "Daşkəsən Rayonu"), ("ta", "த\u{bbe}ஸ\u{bcd}கசன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "దష\u{c4d}క\u{c3e}సన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "แดชคาซาน"), ("tr", "Daşkesen Rayonu"), ("uk", "Дашкесанський район"), ("ur", "داشکسن ضلع"), ("vi", "Dashkasan"), ("zh", "達什卡桑區")]),
                        unofficial_name_list: ["Daskäsän"].to_vec(),
                    }
                ),
                (
                    "FUZ",
                    Subdivision{
                        name: "FUZ",
                        country_alpha2: Alpha2::AZ,
                        code: "FUZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.6003), longitude: Some(47.1431), max_latitude: Some(39.6252594), min_latitude: Some(39.5692419), max_longitude: Some(47.1828459), min_longitude: Some(47.1216489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فضولي"), ("az", "Füzuli"), ("bn", "ফিজ\u{9c1}লি জেল\u{9be}"), ("ccp", "𑄜\u{1112d}𑄎\u{1112a}𑄣\u{11129}"), ("ceb", "Fizuli Rayon"), ("cs", "Füzuli"), ("da", "Fizuli District"), ("de", "Füzuli"), ("el", "Φιζούλι"), ("en", "Fizuli"), ("es", "Füzuli"), ("fa", "شهرستان فضولی"), ("fi", "Füzulin piirikunta"), ("fr", "Fizuli"), ("gu", "ફિઝ\u{ac1}લી જિલ\u{acd}લો"), ("hi", "फिज\u{93c}\u{942}ली जिला"), ("hu", "Füzuli járás"), ("hy", "Ֆիզուլիի շրջան"), ("id", "Fizuli"), ("it", "distretto di Füzuli"), ("ja", "フュズリ県"), ("kn", "ಫ\u{cbf}ಜುಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "퓌줄리 구"), ("lt", "Fuzulio apskritis"), ("lv", "Fuzuli rajons"), ("mk", "Физули"), ("mr", "फझ\u{941}ली जिल\u{94d}हा"), ("ms", "Daerah Fizuli"), ("nb", "Füzuli"), ("nl", "Füzuli"), ("no", "Füzuli"), ("pl", "Rejon Füzuli"), ("pt", "Fizuli"), ("ru", "Физулинский район"), ("si", "ෆ\u{dd2}ස\u{dd4}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Fizuli"), ("sr", "Физулински рејон"), ("sr_Latn", "Fizulinski rejon"), ("sv", "Füzuli Rayonu"), ("ta", "பிஸுலி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c3f}జూల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ฟ\u{e34}ซ\u{e39}ล\u{e35}"), ("tr", "Füzuli Rayonu"), ("uk", "Фізулінський район"), ("ur", "فضولی ضلع"), ("vi", "Fuzuli"), ("zh", "菲祖利區")]),
                        unofficial_name_list: ["Füzuli"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::AZ,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.682778), longitude: Some(46.360556), max_latitude: Some(40.7553196), min_latitude: Some(40.6333659), max_longitude: Some(46.4393805), min_longitude: Some(46.2979746)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غنجة"), ("az", "Gəncə"), ("be", "Гянджа"), ("bg", "Ганджа"), ("bn", "গ\u{9be}ঞ\u{9cd}জ\u{9be}"), ("ca", "Gandja"), ("ccp", "𑄉𑄚\u{11134}𑄎"), ("ceb", "Ganja"), ("cs", "Gandža"), ("cy", "Ganja"), ("da", "Gandja"), ("de", "Gəncə"), ("el", "Γκαντζά"), ("en", "Ganja"), ("es", "Ganja"), ("et", "Gəncə"), ("eu", "Gəncə"), ("fa", "گنجه"), ("fi", "Gəncə"), ("fr", "Gəncə"), ("gu", "ગ\u{a82}જા"), ("he", "גנג׳ה"), ("hi", "गा\u{902}जा"), ("hr", "Gäncä"), ("hu", "Gəncə"), ("hy", "Գանձակ"), ("id", "Ganja"), ("it", "Gäncä"), ("ja", "ギャンジャ"), ("ka", "განჯა"), ("kk", "Гәнжә"), ("kn", "ಗಂಜ"), ("ko", "간자"), ("lt", "Gendžė"), ("lv", "Gendže"), ("mn", "Гянжа"), ("mr", "गणजा"), ("ms", "Ganja"), ("nb", "Gandsja"), ("nl", "Gəncə"), ("no", "Gandsja"), ("pl", "Gandża"), ("pt", "Ganja"), ("ro", "Gandja"), ("ru", "Гянджа"), ("si", "ගන\u{dca}ජ\u{dcf}"), ("sk", "Gəncə"), ("sl", "Gandža"), ("sq", "Ganja"), ("sr", "Ганџа"), ("sr_Latn", "Gandža"), ("sv", "Gəncə"), ("ta", "கஞ\u{bcd}ச\u{bbe}"), ("te", "గంజ\u{c3e}"), ("th", "กานชา"), ("tr", "Gence"), ("uk", "Гянджа"), ("ur", "گنجہ"), ("uz", "Ganja"), ("vi", "Ganca"), ("zh", "占贾")]),
                        unofficial_name_list: ["Gäncä"].to_vec(),
                    }
                ),
                (
                    "GAD",
                    Subdivision{
                        name: "GAD",
                        country_alpha2: Alpha2::AZ,
                        code: "GAD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.5087904), longitude: Some(45.67288509999999), max_latitude: Some(40.821649), min_latitude: Some(40.319229), max_longitude: Some(45.915231), min_longitude: Some(45.3550831)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غدابيغ"), ("az", "Gədəbəy"), ("bn", "গেড\u{9be}বে"), ("ccp", "𑄉𑄘𑄝\u{11133}𑄠"), ("ceb", "Gadabay Rayon"), ("da", "Gedebey"), ("de", "Gədəbəy"), ("el", "Γκάντμπεϊ"), ("en", "Gadabay"), ("es", "Gədəbəy"), ("fa", "شهرستان گدابیگ"), ("fi", "Gədəbəyn piirikunta"), ("fr", "Gədəbəy"), ("gu", "ગાડબ\u{ac7}"), ("hi", "ग\u{947}ड\u{947}बी"), ("hu", "Gədəbəyi járás"), ("hy", "Գետաբեկի շրջան"), ("id", "Gadabay"), ("it", "distretto di Gədəbəy"), ("ja", "ガダバイ県"), ("ka", "გადაბეის რაიონი"), ("kn", "ಗಾಡ\u{ccd}ಬ\u{cc6}"), ("ko", "게데베이 구"), ("lt", "Gadabajaus apskritis"), ("lv", "Gedebejas rajons"), ("mk", "Гедебеј"), ("mn", "Кадебай аймаг"), ("mr", "गाडब\u{947}"), ("ms", "Gadabay"), ("nb", "Gədəbəy"), ("nl", "Gədəbəy"), ("no", "Gədəbəy"), ("pl", "Rejon Gədəbəy"), ("pt", "Gadabay"), ("ru", "Кедабекский район"), ("si", "ගෙඩෙබේ"), ("sq", "Gadabay"), ("sr", "Кедабечки рејон"), ("sr_Latn", "Kedabečki rejon"), ("sv", "Gädäbäj"), ("ta", "கெடிப\u{bbe}ய\u{bcd}"), ("te", "గ\u{c46}డ\u{c46}బ\u{c40}"), ("th", "กากาเบย\u{e4c}"), ("tr", "Gedebey Rayonu"), ("uk", "Кедабекський район"), ("ur", "گدابے ضلع"), ("vi", "Gadabey"), ("zh", "凱達貝克區")]),
                        unofficial_name_list: ["Gädäbäy"].to_vec(),
                    }
                ),
                (
                    "GOR",
                    Subdivision{
                        name: "GOR",
                        country_alpha2: Alpha2::AZ,
                        code: "GOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.610278), longitude: Some(46.789722), max_latitude: Some(40.6267217), min_latitude: Some(40.5915353), max_longitude: Some(46.8130016), min_longitude: Some(46.7551517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غورانبوي"), ("az", "Goranboy"), ("bn", "গোর\u{9be}নবয\u{9bc} জেল\u{9be}"), ("ccp", "𑄉\u{11127}𑄢𑄚\u{11134}𑄝\u{11127}𑄠\u{11134}"), ("ceb", "Goranboy Rayon"), ("da", "Goranboy"), ("de", "Goranboy"), ("el", "Γκορανμπόι"), ("en", "Goranboy"), ("es", "Goranboy"), ("fa", "شهرستان گورانبوی"), ("fi", "Goranboyn piirikunta"), ("fr", "Goranboy"), ("gu", "ગોરાનબોય જિલ\u{acd}લો"), ("he", "גורנבוי"), ("hi", "गोरान\u{94d}बोय जिला"), ("hy", "Գերանբոյի շրջան"), ("id", "Goranboy"), ("it", "distretto di Goranboy"), ("ja", "ゴランボイ県"), ("ka", "გორანბოის რაიონი"), ("kn", "ಗೋರನ\u{ccd}ಬಾಯ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "고란보이 구"), ("lt", "Goranbojaus apskritis"), ("lv", "Goranbojas rajons"), ("mk", "Горанбој"), ("mr", "गोरानबॉय जिल\u{94d}हा"), ("ms", "Goranboy"), ("nb", "Goranboy"), ("nl", "Goranboy"), ("no", "Goranboy"), ("pl", "Rejon Goranboy"), ("pt", "Goranboy"), ("ru", "Геранбойский район"), ("si", "ගොරන\u{dca}බෝය\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Goranboy"), ("sr", "Геранбојски рејон"), ("sr_Latn", "Geranbojski rejon"), ("sv", "Goranboj"), ("ta", "கோரன\u{bcd}போய\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "గ\u{c4b}రన\u{c4d}బ\u{c3e}య\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตกอแรนบอย"), ("tr", "Goranboy Rayonu"), ("uk", "Геранбойський район"), ("ur", "گورانبوئے ضلع"), ("vi", "Geranboy"), ("zh", "戈蘭博伊區")]),
                        unofficial_name_list: ["Goranboy"].to_vec(),
                    }
                ),
                (
                    "GOY",
                    Subdivision{
                        name: "GOY",
                        country_alpha2: Alpha2::AZ,
                        code: "GOY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.65309999999999), longitude: Some(47.7406), max_latitude: Some(40.6723061), min_latitude: Some(40.5915353), max_longitude: Some(47.78572080000001), min_longitude: Some(47.67448410000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غويتشاي"), ("az", "Göyçay"), ("bn", "গয\u{9bc}চ\u{9be}"), ("ccp", "𑄉\u{11127}𑄠\u{11134}𑄌\u{11127}𑄠\u{11134}"), ("ceb", "Goychay Rayon"), ("da", "Goychay"), ("de", "Göyçay"), ("el", "Γκοϊτσάι"), ("en", "Goychay"), ("es", "Göyçay"), ("fa", "شهرستان گوی\u{200c}چای"), ("fi", "Göyçayn piirikunta"), ("fr", "Göyçay"), ("gu", "ગોયચ\u{ac7}"), ("hi", "गोएचाए"), ("id", "Goychay"), ("it", "distretto di Göyçay"), ("ja", "ギョイチャイ県"), ("kn", "ಗೊಯ\u{ccd}ಚೇ"), ("ko", "괴이차이 구"), ("lt", "Geochajaus apskritis"), ("lv", "Gejčalas rajons"), ("mk", "Ѓојчај"), ("mr", "गोयचाय"), ("ms", "Goychay"), ("nb", "Göyçay"), ("nl", "Göyçay"), ("no", "Göyçay"), ("pl", "Rejon Göyçay"), ("pt", "Goychay"), ("ro", "Göyçay"), ("ru", "Гёйчайский район"), ("si", "ගොය\u{dd2}චේ"), ("sq", "Goychay"), ("sr", "Геокчајски рејон"), ("sr_Latn", "Geokčajski rejon"), ("sv", "Göjtjaj"), ("ta", "கோய\u{bcd}ச\u{bcd}சிய"), ("te", "గ\u{c4b}య\u{c4d}\u{200c}చ\u{c47}"), ("th", "กอยเชย\u{e4c}"), ("tr", "Göyçay Rayonu"), ("uk", "Гейчайський район"), ("ur", "گوئچے ضلع"), ("vi", "Goychay"), ("zh", "蓋奧克恰伊區")]),
                        unofficial_name_list: ["Göyçay"].to_vec(),
                    }
                ),
                (
                    "GYG",
                    Subdivision{
                        name: "GYG",
                        country_alpha2: Alpha2::AZ,
                        code: "GYG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Göygöl"), ("ccp", "𑄉\u{11127}𑄠\u{11134}𑄉\u{1112e}𑄣\u{11134}"), ("ceb", "Goygol Rayon"), ("cs", "Göygöl"), ("de", "Göygöl"), ("en", "Goygol"), ("es", "Göygöl"), ("fa", "شهرستان گوی\u{200c}گل"), ("fi", "Göygölin piirikunta"), ("fr", "Göygöl"), ("hy", "Գոյգյոլի շրջան"), ("id", "Khanlar"), ("it", "distretto di Göygöl"), ("ja", "ギョイギョル県"), ("ko", "괴이괼 구"), ("mk", "Ѓојѓол"), ("ms", "Khanlar"), ("nb", "Göygöl"), ("nl", "Göygöl"), ("no", "Göygöl"), ("pl", "Rejon Göygöl"), ("pt", "Goygol"), ("ru", "Гёйгёльский район"), ("sq", "Goygol"), ("sr", "Гогољски рејон"), ("sr_Latn", "Gogoljski rejon"), ("sv", "Xanlar Rayonu"), ("tr", "Göygöl Rayonu"), ("vi", "Goygol"), ("zh", "漢拉爾區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "HAC",
                    Subdivision{
                        name: "HAC",
                        country_alpha2: Alpha2::AZ,
                        code: "HAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.043333), longitude: Some(48.935556), max_latitude: Some(40.0635722), min_latitude: Some(40.00655500000001), max_longitude: Some(48.959273), min_longitude: Some(48.8856411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هايغابول"), ("az", "Hacıqabul"), ("bn", "হ\u{9be}জিগ\u{9be}বোল জেল\u{9be}"), ("ccp", "𑄦𑄎\u{11128}𑄉𑄝\u{1112a}𑄣\u{11134}"), ("ceb", "Hajigabul Rayon"), ("da", "Hajigabul District"), ("de", "Hacıqabul"), ("el", "Χατζιγκαμπούλ"), ("en", "Hajigabul"), ("es", "Hacıqabul"), ("fa", "شهرستان حاجی\u{200c}قبول"), ("fi", "Hacıqabulin piirikunta"), ("fr", "Hacıqabul"), ("gu", "હાજીગબ\u{ac1}લ જિલ\u{acd}લો"), ("hi", "हाजीगबल जिला"), ("hy", "Հաջիգաբուլի շրջան"), ("id", "Hajigabul"), ("it", "distretto di Hacıqabul"), ("ja", "ハジュガブル県"), ("kn", "ಹಜ\u{cbf}ಗಬುಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "하즈가불 구"), ("lt", "Hadžigabulo apskritis"), ("lv", "Hadžigabulas rajons"), ("mk", "Аџикабул"), ("mn", "Хажигабул аймаг"), ("mr", "हाजीगबल जिल\u{94d}हा"), ("ms", "Hajigabul"), ("nb", "Hacıqabul"), ("nl", "Hacıqabul"), ("no", "Hacıqabul"), ("pl", "Rejon Hacıqabul"), ("pt", "Hajigabul"), ("ro", "Hadjîgabul"), ("ru", "Аджикабулский район"), ("si", "හජ\u{dd2}ගබ\u{dd4}ල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Hajigabul"), ("sr", "Аџигабуљски рејон"), ("sr_Latn", "Adžigabuljski rejon"), ("sv", "Hadzjyqabul"), ("ta", "ஹ\u{bbe}ஜிகபூல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "హ\u{c3e}జ\u{c3f}గ\u{c3e}బుల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ฮาจ\u{e34}กาบ\u{e39}ร"), ("tr", "Hacıkabul Rayonu"), ("uk", "Район Гаджигабул"), ("ur", "حاجی قبول ضلع"), ("vi", "Hacuqabul"), ("zh", "卡齊穆罕默德區")]),
                        unofficial_name_list: ["Haciqabul"].to_vec(),
                    }
                ),
                (
                    "IMI",
                    Subdivision{
                        name: "IMI",
                        country_alpha2: Alpha2::AZ,
                        code: "IMI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.8692), longitude: Some(48.06), max_latitude: Some(39.8917536), min_latitude: Some(39.8378046), max_longitude: Some(48.1096519), min_longitude: Some(48.0193521)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إيميشلي"), ("az", "İmişli"), ("bn", "ইমিশি জেল\u{9be}"), ("ccp", "𑄃\u{11128}𑄟\u{11128}𑄌\u{11134}𑄣\u{11128}"), ("ceb", "Imishli Rayon"), ("da", "Imishli District"), ("de", "İmişli (Rayon)"), ("el", "Ιμισχλί"), ("en", "Imishli"), ("es", "İmişli (raión)"), ("et", "İmişli rajoon"), ("fa", "شهرستان ایمیشلی"), ("fi", "İmişlin piirikunta"), ("fr", "İmişli (raion)"), ("gu", "ઈમિશલી જિલ\u{acd}લો"), ("hi", "इमिशली जिला"), ("id", "Imishli"), ("it", "distretto di İmişli"), ("ja", "イミシュリ県"), ("kn", "ಇಮ\u{cbf}ಶ\u{ccd}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "이미슐리 구"), ("lt", "Išmišlio apskritis"), ("lv", "Imišli rajons"), ("mk", "Имишли (округ)"), ("mr", "इमिशिली जिल\u{94d}हा"), ("ms", "Imishli"), ("nb", "İmişli (distrikt)"), ("nl", "İmişli"), ("no", "İmişli (distrikt)"), ("pl", "Rejon İmişli"), ("pt", "Imishli"), ("ru", "Имишлинский район"), ("si", "ඉම\u{dd2}ශ\u{dca}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Imishli (rajon)"), ("sr", "Имишлински рејон"), ("sr_Latn", "Imišlinski rejon"), ("sv", "Imisjli"), ("ta", "இமிஷ\u{bcd}லி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఇమ\u{c3f}ష\u{c4d}ల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดประจวบค\u{e35}ร\u{e35}ข\u{e31}นธ\u{e4c}"), ("tr", "İmişli Rayonu"), ("uk", "Імішлінський район"), ("ur", "ایمیشلی ضلع"), ("vi", "Imishli (quận)"), ("zh", "伊米什利區")]),
                        unofficial_name_list: ["Imisli"].to_vec(),
                    }
                ),
                (
                    "ISM",
                    Subdivision{
                        name: "ISM",
                        country_alpha2: Alpha2::AZ,
                        code: "ISM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.79), longitude: Some(48.151944), max_latitude: Some(40.8119253), min_latitude: Some(40.7646814), max_longitude: Some(48.2034588), min_longitude: Some(48.13007349999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إسمايلي"), ("az", "İsmayıllı"), ("bn", "ইসম\u{9be}ইলি জেল\u{9be}"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄟\u{1112d}𑄣\u{11129}"), ("ceb", "Ismayilli Rayon"), ("da", "Ismailli District"), ("de", "İsmayıllı"), ("el", "Ισμαΐλι"), ("en", "Ismailli"), ("es", "İsmayıllı (raión)"), ("fa", "شهرستان اسماعیللی"), ("fi", "İsmayıllın piirikunta"), ("fr", "Ismailli (raion)"), ("gu", "ઇસ\u{acd}માઈલી જિલ\u{acd}લો"), ("hi", "इस\u{94d}माइलि जिला"), ("hu", "İsmayıllı járás"), ("id", "Ismailli"), ("it", "distretto di İsmayıllı"), ("ja", "イスマイル県"), ("kn", "ಇಸ\u{ccd}ಮಾಲ\u{ccd}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "이스마이을르 구"), ("lt", "Ismailio apskritis"), ("lv", "Ismajilli rajons"), ("mk", "Исмаили (округ)"), ("mr", "इस\u{94d}माइलि जिल\u{94d}हा"), ("ms", "Ismailli"), ("nb", "İsmayıllı (distrikt)"), ("nl", "İsmayıllı"), ("no", "İsmayıllı (distrikt)"), ("pl", "Rejon İsmayıllı"), ("pt", "Ismailli"), ("ru", "Исмаиллинский район"), ("si", "ඉස\u{dca}මය\u{dd2}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Ismailli (rajon)"), ("sr", "Исмајилински рејон"), ("sr_Latn", "Ismajilinski rejon"), ("sv", "Ismajylly"), ("ta", "இஸ\u{bcd}ம\u{bbe}யிலி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఇస\u{c4d}మ\u{c3e}య\u{c3f}ల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อ\u{e35}สเมลล\u{e35}"), ("tr", "İsmayıllı Rayonu"), ("uk", "Ісмаїллінський район"), ("ur", "اسماعیلی ضلع"), ("vi", "Ismailly (quận)"), ("zh", "伊斯梅爾雷區")]),
                        unofficial_name_list: ["Ismayilli"].to_vec(),
                    }
                ),
                (
                    "KAL",
                    Subdivision{
                        name: "KAL",
                        country_alpha2: Alpha2::AZ,
                        code: "KAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.1315615), longitude: Some(46.1674645), max_latitude: Some(40.31433699999999), min_latitude: Some(39.81310999999999), max_longitude: Some(46.759549), min_longitude: Some(45.6090569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كلبجر"), ("az", "Kəlbəcər"), ("bn", "ক\u{9be}লব\u{9be}জ\u{9be}র জেল\u{9be}"), ("ccp", "𑄇𑄣\u{11134}𑄝𑄎𑄢\u{11134}"), ("ceb", "Kəlbəcər Rayonu"), ("da", "Kalbajar"), ("de", "Kəlbəcər"), ("el", "Καλμπατζάρ"), ("en", "Kalbajar"), ("es", "Kəlbəcər"), ("fa", "شهرستان کلبجر"), ("fi", "Kəlbəcərin piirikunta"), ("fr", "Kelbadjar"), ("gu", "કલબજાર જિલ\u{acd}લો"), ("hi", "कालबाजर जिला"), ("hu", "Kəlbəcəri járás"), ("id", "Kalbajar"), ("it", "distretto di Kəlbəcər"), ("ja", "カルバジャル県"), ("kn", "ಕಲ\u{ccd}ಬಾಜರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "켈베제르 구"), ("lt", "Kalbadžaro apskritis"), ("lv", "Kelbedžeras rajons"), ("mk", "Келбеџер"), ("mr", "काळबाजर जिल\u{94d}हा"), ("ms", "Daerah Kalbajar"), ("nb", "Kəlbəcər"), ("nl", "Kəlbəcər"), ("no", "Kəlbəcər"), ("pl", "Rejon Kəlbəcər"), ("pt", "Kalbajar"), ("ro", "Kalbadjar"), ("ru", "Кельбаджарский район"), ("si", "කල\u{dca}ජබ\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Kalbajar"), ("sr", "Кељбаџарски рејон"), ("sr_Latn", "Keljbadžarski rejon"), ("sv", "Kälbädzjär"), ("ta", "கல\u{bcd}பஜ\u{bbe}ர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}ల\u{c4d}\u{200c}బజ\u{c3e}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตคาลบาจาร\u{e4c}"), ("tr", "Kelbecer Rayonu"), ("uk", "Кельбаджарський район"), ("ur", "کلباجار ضلع"), ("uz", "Kalbajar tumani"), ("vi", "Kelbajar"), ("zh", "克爾巴賈爾區")]),
                        unofficial_name_list: ["Kälbäcär"].to_vec(),
                    }
                ),
                (
                    "KAN",
                    Subdivision{
                        name: "KAN",
                        country_alpha2: Alpha2::AZ,
                        code: "KAN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كنجة"), ("az", "Kəngərli"), ("bn", "ক\u{9be}নগ\u{9be}রলি জেল\u{9be}"), ("ccp", "𑄇𑄋\u{11134}𑄉𑄢\u{11134}𑄣\u{11128}"), ("ceb", "Kangarli Rayon"), ("da", "Kangarli District"), ("de", "Kəngərli"), ("el", "Κανγκαρλί"), ("en", "Kangarli"), ("es", "Kangarli"), ("fa", "شهرستان کنگرلی"), ("fi", "Kəngərlin piirikunta"), ("fr", "Kangarli"), ("gu", "કાન\u{acd}ગરલી જિલ\u{acd}લો"), ("hi", "क\u{902}गारली जिला"), ("hy", "Քենգերլիի շրջան"), ("id", "Kangarli"), ("it", "Distretto di Kəngərli"), ("ja", "キャンギャルリ県"), ("kn", "ಕಾಂಗಾರ\u{ccd}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "켄겔리 구"), ("lt", "Kangarlio apskritis"), ("lv", "Kengerli rajons"), ("mk", "Кенгерли"), ("mr", "का\u{902}गारली जिल\u{94d}हा"), ("ms", "Kangarli"), ("nb", "Kəngərli"), ("nl", "Kəngərli"), ("no", "Kəngərli"), ("pl", "Rejon Kəngərli"), ("pt", "Kangarli"), ("ru", "Кенгерлинский район"), ("si", "කන\u{dca}ග\u{dcf}ර\u{dca}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Кенгерлински рејон"), ("sr_Latn", "Kengerlinski rejon"), ("sv", "Kangarli Rayon"), ("ta", "க\u{bbe}ங\u{bcd}கரலி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "కంగ\u{c3e}ర\u{c4d}ల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดมาน\u{e31}ส"), ("tr", "Kengerli Rayonu"), ("uk", "Кенгерлінський район"), ("ur", "کانجارلی ضلع"), ("vi", "Kangarli"), ("zh", "坎加利區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KUR",
                    Subdivision{
                        name: "KUR",
                        country_alpha2: Alpha2::AZ,
                        code: "KUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.34), longitude: Some(48.16), max_latitude: Some(40.3976794), min_latitude: Some(40.3172317), max_longitude: Some(48.2148742), min_longitude: Some(48.120718)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوردامير"), ("az", "Kürdəmir"), ("bn", "ক\u{9c1}রদ\u{9be}মির জেল\u{9be}"), ("ccp", "𑄇\u{1112a}𑄢\u{11134}𑄓𑄟\u{11128}𑄢\u{11134}"), ("ceb", "Kurdamir Rayon"), ("da", "Kurdamir District"), ("de", "Kürdəmir"), ("el", "Κουρνταμίρ"), ("en", "Kurdamir"), ("es", "Kürdəmir"), ("fa", "شهرستان کردامیر"), ("fi", "Kürdəmirin piirikunta"), ("fr", "Kürdəmir"), ("gu", "ક\u{ac1}ર\u{acd}દામિર જિલ\u{acd}લો"), ("hi", "क\u{941}र\u{94d}दमीर जिला"), ("id", "Kurdamir"), ("it", "distretto di Kürdəmir"), ("ja", "キュルダミル県"), ("kn", "ಕುರ\u{ccd}ದಾಮ\u{cbf}ರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "퀴르데미르 구"), ("lt", "Kiurdamiro apskritis"), ("lv", "Kurdemiras rajons"), ("mk", "Ќурдемир"), ("mr", "क\u{941}र\u{94d}दामिर जिल\u{94d}हा"), ("ms", "Kurdamir"), ("nb", "Kürdəmir"), ("nl", "Kürdəmir"), ("no", "Kürdəmir"), ("pl", "Rejon Kürdəmir"), ("pt", "Kurdamir"), ("ru", "Кюрдамирский район"), ("si", "ක\u{dd4}ර\u{dca}ද\u{dcf}ම\u{dd3}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}ර\u{dd2}ක\u{dca}කය"), ("sq", "Kurdamir"), ("sr", "Кјурдамирски рејон"), ("sr_Latn", "Kjurdamirski rejon"), ("sv", "Kürdämir"), ("ta", "குர\u{bcd}ட\u{bbe}மிர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "కుర\u{c4d}ద\u{c3e}మ\u{c3f}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e39}ดาเม\u{e35}ยร\u{e4c}"), ("tr", "Kürdemir Rayonu"), ("uk", "Кюрдамирський район"), ("ur", "کردامیر ضلع"), ("vi", "Kyurdamir"), ("zh", "丘爾達米爾區")]),
                        unofficial_name_list: ["Kürdämir"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::AZ,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.753611), longitude: Some(48.851111), max_latitude: Some(38.7975439), min_latitude: Some(38.7321757), max_longitude: Some(48.8669301), min_longitude: Some(48.8232207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لانكران"), ("az", "Lənkəran"), ("bn", "ল\u{9be}ংক\u{9be}র\u{9be}ন জেল\u{9be}"), ("ccp", "𑄣\u{11127}\u{11101}𑄇𑄢𑄚\u{11134}"), ("ceb", "Lankaran Rayon"), ("da", "Lankaran District"), ("de", "Lənkəran"), ("el", "Λάνκαραν"), ("en", "Lankaran"), ("es", "Lənkəran"), ("fa", "شهرستان لنکران"), ("fi", "Lənkəranin piirikunta"), ("fr", "Lənkəran"), ("gu", "લ\u{a82}કારણ જિલ\u{acd}લો"), ("hi", "ल\u{902}कारन जिला"), ("id", "Lankaran"), ("it", "distretto di Lənkəran"), ("ja", "レンキャラン県"), ("kn", "ಲಂಕರನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "렌케란 구"), ("lt", "Lankarano apskritis"), ("lv", "Lenkeranas rajons"), ("mk", "Ленкеран"), ("mn", "Ланкеран аймаг"), ("mr", "ल\u{902}कारण जिल\u{94d}हा"), ("ms", "Lankaran"), ("nb", "Lənkəran"), ("nl", "Lənkəran"), ("no", "Lənkəran"), ("pl", "Rejon Lenkoran"), ("pt", "Lankaran"), ("ru", "Ленкоранский район"), ("si", "ලන\u{dca}කරන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Lankaran"), ("sr", "Ленкорански рејон"), ("sr_Latn", "Lenkoranski rejon"), ("sv", "Lankaran"), ("ta", "லங\u{bcd}க\u{bbe}ரன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "లంక\u{c3e}రన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตล\u{e31}งคาร\u{e31}น"), ("tr", "Lenkeran Rayonu"), ("uk", "Ленкоранський район"), ("ur", "لنکران ضلع"), ("vi", "Lenkaran"), ("zh", "連科蘭區")]),
                        unofficial_name_list: ["Länkäran City"].to_vec(),
                    }
                ),
                (
                    "LAC",
                    Subdivision{
                        name: "LAC",
                        country_alpha2: Alpha2::AZ,
                        code: "LAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.633333), longitude: Some(46.55), max_latitude: Some(39.652359), min_latitude: Some(39.6167968), max_longitude: Some(46.5657878), min_longitude: Some(46.5265846)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لاتشين"), ("az", "Laçın"), ("ccp", "𑄣\u{11127}𑄌\u{11128}𑄚\u{11134}"), ("ceb", "Laçın Rayonu"), ("de", "Laçın"), ("en", "Lachin"), ("es", "Laçın"), ("fa", "شهرستان لاچین"), ("fi", "Laçınin piirikunta"), ("fr", "Latchin"), ("hu", "Laçıni járás"), ("id", "Lachin"), ("it", "distretto di Laçın"), ("ja", "ラチン県"), ("ko", "라츤 구"), ("mk", "Лачин"), ("ms", "Daerah Lachin"), ("nb", "Laçın"), ("nl", "Laçın"), ("no", "Laçın"), ("pl", "Rejon Laçın"), ("pt", "Lachin"), ("ru", "Лачинский район"), ("sq", "Lachin"), ("sr", "Лачински рејон"), ("sr_Latn", "Lačinski rejon"), ("sv", "Latjyn"), ("tr", "Laçın Rayonu"), ("ur", "لاچین ضلع"), ("vi", "Lachin"), ("zh", "拉欽區")]),
                        unofficial_name_list: ["Laçin"].to_vec(),
                    }
                ),
                (
                    "LAN",
                    Subdivision{
                        name: "LAN",
                        country_alpha2: Alpha2::AZ,
                        code: "LAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.753611), longitude: Some(48.851111), max_latitude: Some(38.7975439), min_latitude: Some(38.7321757), max_longitude: Some(48.8669301), min_longitude: Some(48.8232207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لنكران"), ("az", "Lənkəran²"), ("be", "Горад Ленкарань"), ("bg", "Ленкоран"), ("bn", "লঙ\u{9cd}ক\u{9be}র\u{9be}ন"), ("ca", "Lenkoran"), ("ccp", "𑄣\u{11127}\u{11101}𑄇𑄢𑄚\u{11134} 𑄎𑄬𑄣"), ("ceb", "Lankaran"), ("cs", "Lenkoran"), ("da", "Lankaran"), ("de", "Lənkəran²"), ("el", "Λανκαράν"), ("en", "Lankaran District"), ("es", "Lankaran"), ("et", "Lənkəran"), ("fa", "لنکران"), ("fi", "Lənkəran"), ("fr", "Lankaran"), ("gu", "લ\u{a82}કારણ"), ("he", "לנקראן"), ("hi", "ल\u{902}कारण"), ("hy", "Լենքորան"), ("id", "Lankaran²"), ("it", "Lənkəran"), ("ja", "ランカラン"), ("ka", "ლენქორანი"), ("kn", "ಲಂಕರನ\u{ccd}"), ("ko", "렌케란"), ("ky", "Ленкорань"), ("lt", "Lenkoranė"), ("lv", "Lenkorāna"), ("ml", "ലങ\u{d4d}കര\u{d3e}ൻ"), ("mn", "Ланкеран хот"), ("mr", "ल\u{902}करण"), ("ms", "Lankaran²"), ("nb", "Lənkəran²"), ("nl", "Lənkəran²"), ("no", "Lənkəran²"), ("pl", "Lenkoran"), ("pt", "Lankaran²"), ("ro", "Lankaran"), ("ru", "Ленкорань"), ("si", "ලන\u{dca}ක\u{dcf}රන\u{dca}"), ("sl", "Lankaran"), ("sq", "Lankaran²"), ("sv", "Lankaran²"), ("ta", "லங\u{bcd}க\u{bbe}ரன\u{bcd}"), ("te", "లంక\u{c3e}రన\u{c4d}"), ("th", "ล\u{e31}งคาร\u{e31}น"), ("tr", "Lenkeran"), ("uk", "Ленкорань"), ("ur", "لنکران"), ("uz", "Lenkoran"), ("vi", "Lankaran"), ("zh", "連科蘭")]),
                        unofficial_name_list: ["Länkäran"].to_vec(),
                    }
                ),
                (
                    "LER",
                    Subdivision{
                        name: "LER",
                        country_alpha2: Alpha2::AZ,
                        code: "LER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.7736192), longitude: Some(48.4151483), max_latitude: Some(38.786539), min_latitude: Some(38.7643235), max_longitude: Some(48.4290648), min_longitude: Some(48.3984231)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليريك"), ("az", "Lerik"), ("bn", "লেরিক জেল\u{9be}"), ("ccp", "𑄣𑄬𑄢\u{11128}𑄇\u{11134}"), ("ceb", "Lerik Rayon"), ("da", "Lerik District"), ("de", "Lerik"), ("el", "Λέρικ"), ("en", "Lerik"), ("es", "Lerik"), ("fa", "شهرستان لریک"), ("fi", "Lerikin piirikunta"), ("fr", "Lerik"), ("gu", "ઑડિશા"), ("hi", "ल\u{947}रिक जिला"), ("id", "Lerik"), ("it", "distretto di Lerik"), ("ja", "レリク県"), ("kn", "ಲ\u{cc6}ರ\u{cbf}ಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "레리크 구"), ("lt", "Leriko apskritis"), ("lv", "Lerikas rajons"), ("mk", "Лерик"), ("mr", "लरिक जिल\u{94d}हा"), ("ms", "Daerah Lerik"), ("nb", "Lerik"), ("nl", "Lerik"), ("no", "Lerik"), ("pl", "Rejon Lerik"), ("pt", "Lerik"), ("ru", "Лерикский район"), ("si", "ලෙර\u{dd2}ක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Lerik"), ("sr", "Лерички рејон"), ("sr_Latn", "Lerički rejon"), ("sv", "Lerik Rayonu"), ("ta", "ளெரிக\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c46}ర\u{c3f}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เลร\u{e34}ค"), ("tr", "Lerik Rayonu"), ("uk", "Лерікський район"), ("ur", "لیرک ضلع"), ("vi", "Lerik"), ("zh", "列里克區")]),
                        unofficial_name_list: ["Lerik"].to_vec(),
                    }
                ),
                (
                    "MAS",
                    Subdivision{
                        name: "MAS",
                        country_alpha2: Alpha2::AZ,
                        code: "MAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.033333), longitude: Some(48.65), max_latitude: Some(39.0481191), min_latitude: Some(39.0163495), max_longitude: Some(48.6902905), min_longitude: Some(48.6470318)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماساللي"), ("az", "Masallı"), ("bn", "ম\u{9be}স\u{9be}লি জেল\u{9be}"), ("ccp", "𑄟𑄥𑄣\u{11129}"), ("ceb", "Masally District"), ("da", "Masallı"), ("de", "Masallı"), ("el", "Μασάλι"), ("en", "Masally"), ("es", "Masallı"), ("fa", "شهرستان ماساللی"), ("fi", "Masallın piirikunta"), ("fr", "Masallı"), ("gu", "મસાલી જિલ\u{acd}લો"), ("hi", "म\u{947}ज\u{93c}ली जिला"), ("id", "Masally"), ("it", "distretto di Masallı"), ("ja", "マサッル県"), ("kn", "ಮಸಾಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "마살르 구"), ("lt", "Masalio apskritis"), ("lv", "Masalli rajons"), ("mk", "Масали"), ("mr", "मसाली जिल\u{94d}हा"), ("ms", "Masally"), ("nb", "Masallı"), ("nl", "Masallı"), ("no", "Masallı"), ("pl", "Rejon Masallı"), ("pt", "Masally"), ("ru", "Масаллинский район"), ("si", "මසල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Masally"), ("sr", "Масалински рејон"), ("sr_Latn", "Masalinski rejon"), ("sv", "Masallı Rayonu"), ("ta", "மஸல\u{bcd}லி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3e}స\u{c3e}ల\u{c40} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตมาซอลล\u{e34}"), ("tr", "Masallı Rayonu"), ("uk", "Масаллінський район"), ("ur", "ماسالی ضلع"), ("vi", "Masally"), ("zh", "馬薩雷區")]),
                        unofficial_name_list: ["Masalli"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "MI",
                        country_alpha2: Alpha2::AZ,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.7702563), longitude: Some(47.0496015), max_latitude: Some(40.7958107), min_latitude: Some(40.73093360000001), max_longitude: Some(47.1156406), min_longitude: Some(46.9520472)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مينجا تشيفير"), ("az", "Mingəçevir"), ("be", "Мінгечаур"), ("bg", "Мингечаур"), ("bn", "মিঙ\u{9cd}গ\u{9be}চেভির"), ("ccp", "𑄟\u{11128}\u{11101}𑄉𑄌𑄬𑄞\u{11128}𑄢\u{11134}"), ("ceb", "Mingacevir City"), ("cs", "Mingačevir"), ("da", "Mingachevir"), ("de", "Mingəçevir"), ("el", "Μινγκατσεβίρ"), ("en", "Mingachevir"), ("es", "Mingachevir"), ("et", "Mingəçevir"), ("fa", "مینگه\u{200c}چویر"), ("fi", "Mingəçevir"), ("fr", "Mingachevir"), ("gu", "મિ\u{a82}ગચ\u{ac7}વીર"), ("he", "מיגאצ׳ביר"), ("hi", "मि\u{902}गाश\u{947}वीर"), ("hy", "Մինգեչաուր"), ("id", "Mingachevir"), ("it", "Mingəçevir"), ("ja", "ミンゲチェヴィル"), ("ka", "მინგეჩაური"), ("kn", "ಮ\u{cbf}ಂಗಚ\u{cc6}ವೀರ\u{ccd}"), ("ko", "밍개체비르"), ("ky", "Мингечуар"), ("lt", "Mingečevyras"), ("lv", "Mingačevira"), ("mn", "Мингечевир хот"), ("mr", "मि\u{902}गच\u{947}व\u{94d}हीर"), ("ms", "Mingachevir"), ("nb", "Mingachevir"), ("nl", "Mingəçevir"), ("no", "Mingachevir"), ("pl", "Mingeczaur"), ("pt", "Mingachevir"), ("ro", "Mingacevir"), ("ru", "Мингечаур"), ("si", "ම\u{dd2}න\u{dcf}ශේව\u{dd2}ර\u{dca}"), ("sq", "Mingachevir"), ("sr", "Мингечевир"), ("sr_Latn", "Mingečevir"), ("sv", "Mingetjaur"), ("ta", "மிங\u{bcd}க\u{bbe}செவிற\u{bcd}"), ("te", "మ\u{c3f}ంగ\u{c3e}చ\u{c46}వ\u{c3f}ర\u{c4d}"), ("th", "ม\u{e34}นกาเชเวอ"), ("tr", "Mingeçevir"), ("uk", "Мінгечаур"), ("ur", "منگچویر"), ("vi", "Mingachevir"), ("zh", "明盖恰乌尔")]),
                        unofficial_name_list: ["Mingäçevir"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "NA",
                        country_alpha2: Alpha2::AZ,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.506667), longitude: Some(46.825), max_latitude: Some(40.5216943), min_latitude: Some(40.4947426), max_longitude: Some(46.8460464), min_longitude: Some(46.8092251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄚𑄛\u{11134}𑄑𑄣\u{11127}𑄚\u{11134}"), ("ceb", "Naftalan"), ("de", "Naftalan"), ("en", "Naftalan"), ("sv", "Naftalan Şəhəri")]),
                        unofficial_name_list: ["Naftalan"].to_vec(),
                    }
                ),
                (
                    "NEF",
                    Subdivision{
                        name: "NEF",
                        country_alpha2: Alpha2::AZ,
                        code: "NEF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.358611), longitude: Some(49.246944), max_latitude: Some(39.4222045), min_latitude: Some(39.3435909), max_longitude: Some(49.2725657), min_longitude: Some(49.2190934)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نيفتشالا"), ("az", "Neftçala"), ("bn", "নেফক\u{9be}ল\u{9be} জেল\u{9be}"), ("ccp", "𑄚𑄬𑄛\u{11134}𑄌\u{11127}𑄣"), ("ceb", "Neftchala Rayon"), ("da", "Neftchala District"), ("de", "Neftçala"), ("el", "Νεφτσάλα"), ("en", "Neftchala"), ("es", "Neftçala"), ("fa", "شهرستان نفت\u{200c}چاله"), ("fi", "Neftçalan piirikunta"), ("fr", "Neftçala"), ("gu", "ન\u{ac7}ફ\u{acd}ટચલા જિલ\u{acd}લો"), ("hi", "न\u{947}फचाला जिला"), ("id", "Neftchala"), ("it", "distretto di Neftçala"), ("ja", "ネフトチャラ県"), ("kn", "ನ\u{cc6}ಫ\u{ccd}ಚಾಲಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "네프찰라 구"), ("lt", "Neftchalos apskritis"), ("lv", "Neftčalas rajons"), ("mk", "Нефтчала"), ("mr", "न\u{947}फ\u{94d}ताळा जिल\u{94d}हा"), ("ms", "Daerah Neftchala"), ("nb", "Neftçala"), ("nl", "Neftçala"), ("no", "Neftçala"), ("pl", "Rejon Neftçala"), ("pt", "Neftchala"), ("ru", "Нефтечалинский район"), ("si", "නෙෆ\u{dca}ට\u{dca}චල\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Neftchala"), ("sr", "Нефтечалински рејон"), ("sr_Latn", "Neftečalinski rejon"), ("sv", "Nefttjala"), ("ta", "நேபிச\u{bcd}சல\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "న\u{c46}ఫ\u{c4d}ట\u{c4d}\u{200c}చ\u{c3e}ల\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเนฟท\u{e4c}ชาลา"), ("tr", "Neftçala Rayonu"), ("uk", "Район Нефтечала"), ("ur", "نفتچالا ضلع"), ("vi", "Neftchala"), ("zh", "涅夫捷恰拉區")]),
                        unofficial_name_list: ["Neftçala"].to_vec(),
                    }
                ),
                (
                    "NV",
                    Subdivision{
                        name: "NV",
                        country_alpha2: Alpha2::AZ,
                        code: "NV",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nachitsjewan"), ("ar", "نخجوان"), ("az", "Naxçıvan"), ("be", "Горад Нахічэвань"), ("bg", "Нахичеван"), ("bn", "ন\u{9be}খচিভ\u{9be}ন"), ("ca", "Naxçıvan"), ("ccp", "𑄚𑄇\u{11134}𑄌\u{11128}𑄞\u{11127}𑄚\u{11134}"), ("ceb", "Nakhchivan Autonomous Republic"), ("cs", "Nachičevan"), ("cy", "Nakhchivan"), ("da", "Nakhitjevan"), ("de", "Naxçıvan"), ("el", "Ναχιτσεβάν"), ("en", "Nakhchivan"), ("es", "Najicheván"), ("et", "Naxçıvan"), ("eu", "Nakhitxevan²"), ("fa", "نخجوان"), ("fi", "Nahitševan"), ("fr", "Nakhitchevan"), ("gl", "Nakhichevan"), ("gu", "નખચિવ\u{ac7}ન"), ("he", "נחיצ׳יבאן"), ("hi", "नख\u{93c}\u{94d}चीवा\u{902}"), ("hu", "Nahicseván"), ("hy", "Նախիջևան"), ("id", "Kota Nakhchivan"), ("it", "Naxçıvan"), ("ja", "ナヒチェヴァン"), ("ka", "ნახიჩევანი"), ("kn", "ನಖ\u{ccd}ಚ\u{cbf}ವನ\u{ccd}"), ("ko", "나히체반"), ("ky", "Нахичеван"), ("lt", "Nachičevanė"), ("lv", "Nahčivana"), ("ml", "ന\u{d3e}ഖ\u{d4d}ചിവൻ സിറ\u{d4d}റി"), ("mr", "नखचीव\u{902}"), ("ms", "Nakhchivan"), ("nb", "Nachitjevan"), ("nl", "Nachitsjevan"), ("no", "Nachitjevan"), ("pl", "Nachiczewan"), ("pt", "Naquichevão"), ("ro", "Naxcivan"), ("ru", "Нахичевань"), ("si", "නක\u{dca}ස\u{dd2}වන\u{dca}"), ("sk", "Nachičevan"), ("sr", "Нахчиван"), ("sr_Latn", "Nahčivan"), ("sv", "Nachitjevan"), ("ta", "நஃகுசிவ\u{bbe}ன\u{bcd}"), ("te", "న\u{c3e}క\u{c4d}చ\u{c3f}వన\u{c4d}"), ("th", "นาค\u{e35}ช\u{e35}ว\u{e31}น"), ("tr", "Nahçıvan"), ("uk", "Нахічевань"), ("ur", "ناخچیوان (شہر)"), ("uz", "Naxichevan (shahar)"), ("vi", "Nakhchivan"), ("zh", "納希切萬")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NX",
                    Subdivision{
                        name: "NX",
                        country_alpha2: Alpha2::AZ,
                        code: "NX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.208889), longitude: Some(45.412222), max_latitude: Some(39.228264), min_latitude: Some(39.1708288), max_longitude: Some(45.4377366), min_longitude: Some(45.3697586)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRepublic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nachitsjewan²"), ("ar", "جمهورية نخجوان الذاتية"), ("az", "Naxçıvan Muxtar Respublikası"), ("be", "Нахічэванская Аўтаномная Рэспубліка"), ("bg", "Нахичеванска автономна република"), ("bn", "ন\u{9be}খচিভ\u{9be}ন স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}বশ\u{9be}সিত প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"), ("bs", "Nahičevan"), ("ca", "Nakhtxivan"), ("ccp", "𑄚𑄇\u{11134}𑄌\u{11128}𑄞\u{11127}𑄚\u{11134} 𑄃𑄬𑄃𑄢\u{11134}"), ("ceb", "Nakhchivan Autonomous Republic²"), ("cs", "Nachičevanská autonomní republika"), ("da", "Nakhitjevan²"), ("de", "Autonome Republik Nachitschewan"), ("el", "Ναχιτσεβάν²"), ("en", "Nakhchivan AR"), ("es", "República Autónoma de Najicheván"), ("et", "Nahhitševan"), ("eu", "Nakhitxevan"), ("fa", "جمهوری خودمختار نخجوان"), ("fi", "Nahitševan²"), ("fr", "Nakhitchevan²"), ("gl", "Nakhichevan - Naxçıvan"), ("gu", "નાખચિવન ઓટોનોમસ રિપબ\u{acd}લિક"), ("he", "נחצ׳יבאן"), ("hi", "नकचिवन ऑटोनॉमस रिपब\u{94d}लिक"), ("hu", "Nahicseván²"), ("hy", "Նախիջևանի Ինքնավար Հանրապետություն"), ("id", "Nakhichevan"), ("it", "Repubblica Autonoma di Naxçıvan"), ("ja", "ナヒチェヴァン自治共和国"), ("ka", "ნახიჩევანის ავტონომიური რესპუბლიკა"), ("kk", "Нахшыван"), ("kn", "ನಖ\u{ccd}ಚ\u{cbf}ವನ\u{ccd} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಗಣರಾಜ\u{ccd}ಯ"), ("ko", "나히체반 자치공화국"), ("ky", "Нахичеван автономия республикасы"), ("lt", "Nachičevanės autonominė respublika"), ("lv", "Nahčivanas Autonomā Republika"), ("ml", "ന\u{d3e}ഖ\u{d4d}ചിവൻ സ\u{d4d}വയംഭരണ റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d}"), ("mn", "Нахчыван"), ("mr", "नखिवियन ऑटोनॉमस रिपब\u{94d}लिक"), ("ms", "Nakhichevan"), ("nb", "Nakhitsjevan"), ("nl", "Nachitsjevan²"), ("no", "Nakhitsjevan"), ("pl", "Nachiczewańska Republika Autonomiczna"), ("pt", "Nakichevan"), ("ro", "Naxcivan²"), ("ru", "Нахичеванская Автономная Республика"), ("si", "නක\u{dca}ච\u{dd2}වන\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd3}න ජනරජය"), ("sl", "Nahičevan"), ("sr", "Нахчиван²"), ("sr_Latn", "Nahčivan²"), ("sv", "Nachitjevan²"), ("ta", "நஃகுசிவ\u{bbe}ன\u{bcd} தன\u{bcd}ன\u{bbe}ட\u{bcd}சிக\u{bcd} குடியரசு"), ("te", "న\u{c3e}ఖ\u{c4d}చ\u{c3f}వ\u{c3e}న\u{c4d} అట\u{c3e}న\u{c3e}మస\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d}"), ("th", "นาค\u{e35}ช\u{e35}ว\u{e31}น²"), ("tk", "Nahçywan Awtonom Respublikasy"), ("tr", "Nahçıvan Özerk Cumhuriyeti"), ("uk", "Нахічеванська Автономна Республіка"), ("ur", "ناخچیوان خود مختار جمہوریہ"), ("uz", "Naxichevan"), ("vi", "Nakhchivan²"), ("zh", "纳希切万自治共和国")]),
                        unofficial_name_list: ["Naxçivan"].to_vec(),
                    }
                ),
                (
                    "OGU",
                    Subdivision{
                        name: "OGU",
                        country_alpha2: Alpha2::AZ,
                        code: "OGU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.070833), longitude: Some(47.458333), max_latitude: Some(41.0923544), min_latitude: Some(41.0595502), max_longitude: Some(47.4782753), min_longitude: Some(47.4506378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أغوز"), ("az", "Oğuz"), ("bn", "ওঘ\u{9c1}য জেল\u{9be}"), ("ccp", "𑄃\u{11127}𑄊\u{1112a}𑄌\u{11134}"), ("ceb", "Oghuz Rayon"), ("da", "Oghuz District"), ("de", "Oğuz"), ("el", "Επαρχία Ογκούζ"), ("en", "Oghuz"), ("es", "Oğuz"), ("fa", "شهرستان اغوز"), ("fi", "Oğuzin piirikunta"), ("fr", "Oğuz"), ("gu", "ઓઘ\u{ac1}ઝ જિલ\u{acd}લો"), ("hi", "ओग\u{93c}ज\u{93c} जिला"), ("hu", "Oğuzi járás"), ("hy", "Վարդաշենի շրջան"), ("id", "Oguz"), ("it", "distretto di Oğuz"), ("ja", "オグズ県"), ("kn", "ಒಘುಜ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "오구스 구"), ("lt", "Oguzo apskritis"), ("lv", "Oghužas rajons"), ("mk", "Огуз"), ("mr", "औघ\u{94d}ज जिल\u{94d}हा"), ("ms", "Daerah Oghuz"), ("nb", "Oğuz"), ("nl", "Oğuz"), ("no", "Oğuz"), ("pl", "Rejon Oğuz"), ("pt", "Oğuz"), ("ru", "Огузский район"), ("si", "ඔග\u{dd4}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Oghuz"), ("sr", "Огушки рејон"), ("sr_Latn", "Oguški rejon"), ("sv", "Oğuz"), ("ta", "ஓகுஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఓగుజ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e47}อกฮ\u{e31}ซ"), ("tr", "Oğuz Rayonu"), ("uk", "Огузький район"), ("ur", "اغوز ضلع"), ("vi", "Oghuz"), ("zh", "奧古茲區")]),
                        unofficial_name_list: ["Oguz"].to_vec(),
                    }
                ),
                (
                    "ORD",
                    Subdivision{
                        name: "ORD",
                        country_alpha2: Alpha2::AZ,
                        code: "ORD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.9), longitude: Some(46.033333), max_latitude: Some(38.9190856), min_latitude: Some(38.8915673), max_longitude: Some(46.0360623), min_longitude: Some(46.0086823)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوردوباد"), ("az", "Ordubad"), ("be", "Ардубадскі раён"), ("bn", "ওরডোব\u{9be}ড জেল\u{9be}"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄓\u{11128}𑄝𑄖\u{11134}"), ("ceb", "Ordubad Rayon"), ("da", "Ordubad District"), ("de", "Ordubad"), ("el", "Όρντουμπαντ"), ("en", "Ordubad"), ("es", "Ordubad"), ("fa", "شهرستان اردوباد"), ("fi", "Ordubadin piirikunta"), ("fr", "Ordubad"), ("gu", "ઓર\u{acd}ડ\u{ac1}બાદ જિલ\u{acd}લો"), ("hi", "ऑर\u{94d}ड\u{941}ब\u{948}ड जिला"), ("hy", "Որդուարի շրջան"), ("id", "Ordubad"), ("it", "Distretto di Ordubad"), ("ja", "オルドゥバド県"), ("ka", "ორდუბადის რაიონი"), ("kn", "ಒರ\u{ccd}ಡುಬಾದ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "오르두바트 구"), ("lt", "Ordubado apskritis"), ("lv", "Ordubadas rajons"), ("mk", "Ордубад"), ("mr", "ऑर\u{94d}ड\u{941}ब\u{945}ड जिल\u{94d}हा"), ("ms", "Daerah Ordubad"), ("nb", "Ordubad"), ("nl", "Ordubad"), ("no", "Ordubad"), ("pl", "Rejon Ordubad"), ("pt", "Ordubad"), ("ru", "Ордубадский район"), ("si", "ඔර\u{dca}ඩ\u{dd4}බ\u{dcf}ද\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Ордубадски рејон"), ("sr_Latn", "Ordubadski rejon"), ("sv", "Ordubad Rayon"), ("ta", "ஓடுப\u{bbe}டு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఓర\u{c4d}డుబ\u{c3e}డ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอรด\u{e39}แบด"), ("tr", "Ordubad Rayonu"), ("uk", "Ордубадський район"), ("ur", "اردوباد ضلع"), ("vi", "Ordubad"), ("zh", "奧爾杜巴德區")]),
                        unofficial_name_list: ["Ordubad"].to_vec(),
                    }
                ),
                (
                    "QAB",
                    Subdivision{
                        name: "QAB",
                        country_alpha2: Alpha2::AZ,
                        code: "QAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9981633), longitude: Some(47.8699826), max_latitude: Some(41.0042653), min_latitude: Some(40.9477997), max_longitude: Some(47.8774202), min_longitude: Some(47.8214693)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غابالا"), ("az", "Qəbələ"), ("bg", "Габалски район"), ("bn", "ক\u{9be}ব\u{9be}ল\u{9be} জেল\u{9be}"), ("ccp", "𑄇\u{11127}𑄝\u{11127}𑄣"), ("ceb", "Qabala Rayon"), ("cs", "Gabala"), ("da", "Qabala District"), ("de", "Qəbələ"), ("el", "Καμπάλα"), ("en", "Qabala"), ("es", "Qəbələ"), ("fa", "شهرستان قبله"), ("fi", "Qəbələn piirikunta"), ("fr", "Qabala"), ("gu", "કબાલા જિલ\u{acd}લો"), ("hi", "कबाला जिला"), ("hu", "Qəbələi járás"), ("id", "Qabala"), ("it", "distretto di Qəbələ"), ("ja", "ガバラ県"), ("kn", "ಖಬಲಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "게벨레 구"), ("lt", "Kabalos apskritis"), ("lv", "Gebeles rajons"), ("mk", "Кабала"), ("mr", "कबाला जिल\u{94d}हा"), ("ms", "Qabala"), ("nb", "Qəbələ"), ("nl", "Qəbələ"), ("no", "Qəbələ"), ("pl", "Rejon Qəbələ"), ("pt", "Qabala"), ("ro", "Raionul Qabala"), ("ru", "Габалинский район"), ("si", "කබල\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Qabala"), ("sr", "Габалински рејон"), ("sr_Latn", "Gabalinski rejon"), ("sv", "Qəbələ (distrikt)"), ("ta", "கில\u{bbe}ப\u{bbe}ல\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖ\u{c3e}బ\u{c3e}ల\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดกะบาลา"), ("tr", "Kabala Rayonu"), ("uk", "Габалинський район"), ("ur", "قابالا ضلع"), ("uz", "Qabala tumani"), ("vi", "Gabala"), ("zh", "蓋貝萊區")]),
                        unofficial_name_list: ["Qäbälä"].to_vec(),
                    }
                ),
                (
                    "QAX",
                    Subdivision{
                        name: "QAX",
                        country_alpha2: Alpha2::AZ,
                        code: "QAX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4225), longitude: Some(46.924167), max_latitude: Some(41.4429194), min_latitude: Some(41.4077159), max_longitude: Some(46.9906713), min_longitude: Some(46.89085009999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة قاخ"), ("az", "Qax"), ("bn", "ক\u{9be}হ জেল\u{9be}"), ("ccp", "𑄇\u{11127}𑄇\u{11134}"), ("ceb", "Qakh Rayon"), ("da", "Qakh District"), ("de", "Qax"), ("el", "Κάκχ"), ("en", "Qakh"), ("es", "Qax"), ("fa", "شهرستان قاخ"), ("fi", "Qaxin piirikunta"), ("fr", "Qax"), ("gu", "કાખ જિલ\u{acd}લો"), ("hi", "काख जिला"), ("hu", "Qaxi járás"), ("id", "Qakh"), ("it", "distretto di Qax"), ("ja", "ガフ県"), ("ka", "კახის რაიონი"), ("kn", "ಖಖ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "가흐 구"), ("lt", "Kako apskritis"), ("lv", "Gahas rajons"), ("mk", "Ках"), ("mr", "काख जिल\u{94d}हा"), ("ms", "Qakh"), ("nb", "Qax"), ("nl", "Qax"), ("no", "Qax"), ("pl", "Rejon Qax"), ("pt", "Qakh"), ("ru", "Кахский район"), ("si", "ක\u{dcf}ක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Qakh"), ("sr", "Кашки рејон"), ("sr_Latn", "Kaški rejon"), ("sv", "Qach"), ("ta", "கியக\u{bcd}ஹ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖ\u{c3e}ఖ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องคาร\u{e4c}ก"), ("tr", "Kah rayonu"), ("uk", "Гахський район"), ("ur", "کاخ ضلع"), ("vi", "Gakh"), ("zh", "卡希區")]),
                        unofficial_name_list: ["Qax"].to_vec(),
                    }
                ),
                (
                    "QAZ",
                    Subdivision{
                        name: "QAZ",
                        country_alpha2: Alpha2::AZ,
                        code: "QAZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.093333), longitude: Some(45.366111), max_latitude: Some(41.1163487), min_latitude: Some(41.0805481), max_longitude: Some(45.3892851), min_longitude: Some(45.3229809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة قازاخ"), ("az", "Qazax"), ("be", "Казахскі раён"), ("bn", "কোয\u{9bc}\u{9be}জ\u{9be}ক জেল\u{9be}"), ("ccp", "𑄇\u{11127}𑄎𑄇\u{11134}"), ("ceb", "Qazakh Rayon"), ("da", "Qazakh District"), ("de", "Qazax"), ("el", "Καζάκχ"), ("en", "Qazakh"), ("es", "Qazax"), ("fa", "شهرستان قازاخ"), ("fi", "Qazaxin piirikunta"), ("fr", "Qazax"), ("gu", "કઝાક જિલ\u{acd}લો"), ("hi", "कज\u{93c}ाख\u{93c} जिला"), ("hu", "Qazaxi járás"), ("hy", "Ղազախի շրջան"), ("id", "Qazakh"), ("it", "distretto di Qazax"), ("ja", "ガザフ県"), ("kk", "Қазақ ауданы"), ("kn", "ಖಜಾಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "가자흐 구"), ("lt", "Kazacho apskritis"), ("lv", "Gazahas rajons"), ("mk", "Казах"), ("mr", "कझाख जिल\u{94d}हा"), ("ms", "Qazakh"), ("nb", "Qazax"), ("nl", "Qazax"), ("no", "Qazax"), ("pl", "Rejon Qazax"), ("pt", "Qazakh"), ("ro", "Qazax"), ("ru", "Казахский район"), ("si", "කස\u{dcf}ක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Qazakh"), ("sr", "Казашки рејон"), ("sr_Latn", "Kazaški rejon"), ("sv", "Qazach"), ("ta", "கச\u{bbe}க\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖజ\u{c3e}ఖ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "กาซาฮ\u{e4c}"), ("tr", "Kazah Rayonu"), ("uk", "Газахський район"), ("ur", "قازاخ ضلع"), ("vi", "Gazakh"), ("zh", "哈薩克區")]),
                        unofficial_name_list: ["Qazax"].to_vec(),
                    }
                ),
                (
                    "QBA",
                    Subdivision{
                        name: "QBA",
                        country_alpha2: Alpha2::AZ,
                        code: "QBA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.359722), longitude: Some(48.5125), max_latitude: Some(41.3764865), min_latitude: Some(41.3494949), max_longitude: Some(48.557725), min_longitude: Some(48.4612941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قوبا"), ("az", "Quba"), ("bn", "কোব\u{9be} জেল\u{9be}"), ("ccp", "𑄇\u{1112a}𑄝"), ("ceb", "Quba Rayon"), ("da", "Quba District"), ("de", "Quba"), ("el", "Κούμπα"), ("en", "Quba"), ("es", "Quba"), ("fa", "شهرستان قبه"), ("fi", "Quban piirikunta"), ("fr", "Quba"), ("gu", "ક\u{acd}ય\u{ac1}બા જિલ\u{acd}લો"), ("hi", "क\u{94d}य\u{942}बा जिला"), ("hu", "Qubai járás"), ("id", "Quba"), ("it", "distretto di Quba"), ("ja", "クバ県"), ("ka", "ყუბის რაიონი"), ("kn", "ಖುಬಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "구바 구"), ("lt", "Kubos apskritis"), ("lv", "Gubas rajons"), ("mk", "Куба"), ("mr", "क\u{94d}य\u{942}बा जिल\u{94d}हा"), ("ms", "Quba"), ("nb", "Quba"), ("nl", "Quba"), ("no", "Quba"), ("pl", "Rejon Quba"), ("pt", "Quba"), ("ru", "Кубинский район"), ("si", "ක\u{dd4}බ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Quba"), ("sr", "Кубински рејон"), ("sr_Latn", "Kubinski rejon"), ("sv", "Quba Rayonu"), ("ta", "கியூப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c4d}యూబ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e39}บา"), ("tr", "Kuba Rayonu"), ("uk", "Куба"), ("ur", "قوبا ضلع"), ("uz", "Quba tumani"), ("vi", "Guba"), ("zh", "庫巴區")]),
                        unofficial_name_list: ["Quba"].to_vec(),
                    }
                ),
                (
                    "QBI",
                    Subdivision{
                        name: "QBI",
                        country_alpha2: Alpha2::AZ,
                        code: "QBI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2713996), longitude: Some(46.6354312), max_latitude: Some(39.489675), min_latitude: Some(39.120843), max_longitude: Some(46.829667), min_longitude: Some(46.3865889)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة قبادلي"), ("az", "Qubadlı"), ("bn", "ক\u{9be}\u{9c1}ব\u{9be}ডলি জেল\u{9be}"), ("ccp", "𑄇\u{1112a}𑄝𑄖\u{11134}𑄣\u{11128}"), ("ceb", "Qubadli Rayon"), ("da", "Qubadli District"), ("de", "Qubadlı"), ("el", "Επαρχία Κουμπάντλι"), ("en", "Qubadli"), ("es", "Qubadlı"), ("fa", "شهرستان قبادلی"), ("fi", "Qubadlın piirikunta"), ("fr", "Qubadli"), ("gu", "ક\u{acd}ય\u{ac1}બાડલી જિલ\u{acd}લો"), ("hi", "क\u{94d}व\u{948}डली जिला"), ("hu", "Qubadlı járás"), ("id", "Qubadli"), ("it", "distretto di Qubadlı"), ("ja", "グバドリ県"), ("ka", "გუბადლის რაიონი"), ("kn", "ಕ\u{ccd}ಯ\u{cc2}ಬಾದ\u{ccd}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "구바들르 구"), ("lt", "Kubadlio apskritis"), ("lv", "Gubadli rajons"), ("mk", "Кубадли"), ("mr", "क\u{94d}य\u{941}बाडली जिल\u{94d}हा"), ("ms", "Daerah Qubadli"), ("nb", "Qubadlı"), ("nl", "Qubadlı"), ("no", "Qubadlı"), ("pl", "Rejon Qubadlı"), ("pt", "Qubadli"), ("ru", "Кубатлинский район"), ("si", "ක\u{dd4}බ\u{dcf}ද\u{dca}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Qubadli"), ("sr", "Кубатлински рејон"), ("sr_Latn", "Kubatlinski rejon"), ("sv", "Qubadli"), ("ta", "குப\u{bbe}ட\u{bcd}லி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖుబ\u{c3e}డ\u{c4d}ల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e34}วแบดล\u{e34}"), ("tr", "Kubadlı Rayonu"), ("uk", "Кубатлинський район"), ("ur", "قوبادلی ضلع"), ("vi", "Gubadly"), ("zh", "庫巴特雷區")]),
                        unofficial_name_list: ["Qubadli"].to_vec(),
                    }
                ),
                (
                    "QOB",
                    Subdivision{
                        name: "QOB",
                        country_alpha2: Alpha2::AZ,
                        code: "QOB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.0877962), longitude: Some(49.4030219), max_latitude: Some(40.1079896), min_latitude: Some(40.0337922), max_longitude: Some(49.433756), min_longitude: Some(49.38769749999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غوبوستان"), ("az", "Qobustan"), ("bn", "গোব\u{9c1}স\u{9cd}ত\u{9be}ন জেল\u{9be}"), ("ccp", "𑄉\u{11127}𑄝\u{1112a}𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Gobustan Rayon"), ("cs", "Gobustan"), ("da", "Gobustan District"), ("de", "Qobustan"), ("el", "Γκομπουστάν"), ("en", "Gobustan"), ("es", "Qobustan"), ("fa", "شهرستان قبوستان"), ("fi", "Qobustanin piirikunta"), ("fr", "Qobustan"), ("gu", "ગોબ\u{acd}સ\u{acd}ટન જિલ\u{acd}લો"), ("hi", "गोबस\u{94d}टन जिला"), ("id", "Qobustan"), ("it", "distretto di Qobustan"), ("ja", "ゴブスタン県"), ("ka", "გობუსტანის რაიონი"), ("kn", "ಗೊಬಸ\u{ccd}ಟನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "고부스탄 구"), ("lt", "Gobustano apskritis"), ("lv", "Gobustanas rajons"), ("mk", "Гобустан"), ("mr", "गोब\u{942}स\u{94d}तान जिल\u{94d}हा"), ("ms", "Qobustan"), ("nb", "Qobustan"), ("nl", "Qobustan"), ("no", "Qobustan"), ("pl", "Rejon Qobustan"), ("pt", "Gobustão"), ("ru", "Гобустанский район"), ("si", "ගොබ\u{dd4}ස\u{dca}ත\u{dcf}න\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Gobustan"), ("sr", "Гобустански рејон"), ("sr_Latn", "Gobustanski rejon"), ("sv", "Qobustan Rayonu"), ("ta", "கோபிஸ\u{bcd}ட\u{bbe}ண\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "గ\u{c4b}బుస\u{c4d}త\u{c3e}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโกบ\u{e31}สต\u{e31}น"), ("tr", "Kobustan Rayonu"), ("uk", "Гобустанський район"), ("ur", "قوبوستان ضلع"), ("vi", "Gobustan"), ("zh", "戈布斯坦區")]),
                        unofficial_name_list: ["Qobustan"].to_vec(),
                    }
                ),
                (
                    "QUS",
                    Subdivision{
                        name: "QUS",
                        country_alpha2: Alpha2::AZ,
                        code: "QUS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.426389), longitude: Some(48.435556), max_latitude: Some(41.4406353), min_latitude: Some(41.4036923), max_longitude: Some(48.4615945), min_longitude: Some(48.3841753)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوسار"), ("az", "Qusar"), ("bn", "কোস\u{9be}র জেল\u{9be}"), ("ccp", "𑄇\u{1112a}𑄥𑄢\u{11134}"), ("ceb", "Qusar Rayon"), ("da", "Qusar District"), ("de", "Qusar"), ("el", "Κουσάρ"), ("en", "Qusar"), ("es", "Qusar"), ("fa", "شهرستان قوسار"), ("fi", "Qusarin piirikunta"), ("fr", "Qusar"), ("gu", "ક\u{ac1}સર જિલ\u{acd}લો"), ("hi", "क\u{941}सर जिला"), ("hu", "Qusari járás"), ("id", "Qusar"), ("it", "distretto di Qusar"), ("ja", "クサル県"), ("kn", "ಕುಸಾರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "구사르 구"), ("lt", "Kusaro provincija"), ("lv", "Gusaras rajons"), ("mk", "Кусар"), ("mr", "क\u{941}सर जिल\u{94d}हा"), ("ms", "Qusar"), ("nb", "Qusar"), ("nl", "Qusar"), ("no", "Qusar"), ("pl", "Rejon Qusar"), ("pt", "Qusar"), ("ru", "Кусарский район"), ("si", "ක\u{dd4}ස\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Qusar"), ("sr", "Кусарски рејон"), ("sr_Latn", "Kusarski rejon"), ("sv", "Qusar Rayonu"), ("ta", "குசர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖుస\u{c3e}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e39}ซาร\u{e4c}"), ("tr", "Kusar Rayonu"), ("uk", "Гусарський район"), ("ur", "قوسار ضلع"), ("uz", "Qusar tumani"), ("vi", "Gusar"), ("zh", "庫薩雷區")]),
                        unofficial_name_list: ["Qusar"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::AZ,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0489281), longitude: Some(-111.0937311), max_latitude: Some(37.0042599), min_latitude: Some(31.3321771), max_longitude: Some(-109.0452231), min_longitude: Some(-114.8165909)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شكي"), ("az", "Şəki"), ("be", "Шэкі"), ("bg", "Шеки"), ("bn", "শ\u{9be}কি"), ("ca", "Shaki"), ("ccp", "𑄥𑄇\u{11128}"), ("ceb", "Sheki"), ("cs", "Şəki"), ("da", "Shaki"), ("de", "Şəki"), ("el", "Σάκι"), ("en", "Shaki"), ("es", "Şəki"), ("et", "Şəki"), ("fa", "شکی"), ("fi", "Şəki"), ("fr", "Shaki"), ("gu", "શાકી"), ("he", "שקי"), ("hi", "शाकी"), ("hy", "Շաքի"), ("id", "Shaki"), ("it", "Şəki"), ("ja", "シャキ"), ("ka", "შაქი"), ("kn", "ಶಕೀ"), ("ko", "섀키"), ("lt", "Šekis"), ("lv", "Šeki"), ("mn", "Шеки"), ("mr", "शकी"), ("ms", "Shaki, Azerbaijan"), ("nb", "Şəki"), ("nl", "Şəki"), ("no", "Şəki"), ("pl", "Şəki"), ("pt", "Shaki"), ("ro", "Șaki"), ("ru", "Шеки"), ("si", "ශක\u{dd2}"), ("sl", "Şäki"), ("sq", "Shaki (qytet)"), ("sr", "Шаки"), ("sr_Latn", "Šaki"), ("sv", "Sjaki"), ("ta", "ஷ\u{bbe}கி"), ("te", "ష\u{c3e}క\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดเกลเดอร\u{e4c}ล\u{e31}นด\u{e4c}"), ("tr", "Şeki"), ("uk", "Шекі"), ("ur", "شکی"), ("uz", "Sheki"), ("vi", "Şəki"), ("zh", "舍基")]),
                        unofficial_name_list: ["Säki City"].to_vec(),
                    }
                ),
                (
                    "SAB",
                    Subdivision{
                        name: "SAB",
                        country_alpha2: Alpha2::AZ,
                        code: "SAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.012778), longitude: Some(48.478889), max_latitude: Some(40.0227507), min_latitude: Some(39.9682403), max_longitude: Some(48.5010339), min_longitude: Some(48.4441281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة صابرآباد"), ("az", "Sabirabad"), ("bg", "Сабирабад (район)"), ("bn", "স\u{9be}বির\u{9be}ব\u{9be}দ জেল\u{9be}"), ("ccp", "𑄥\u{11127}𑄝\u{11128}𑄢𑄝𑄖\u{11134}"), ("ceb", "Sabirabad Rayon"), ("cs", "Sabirabad"), ("da", "Sabirabad District"), ("de", "Sabirabad"), ("el", "Σαμπιραμπάντ"), ("en", "Sabirabad"), ("es", "Sabirabad"), ("et", "Sabirabadi rajoon"), ("fa", "شهرستان صابرآباد"), ("fi", "Sabirabadin piirikunta"), ("fr", "Sabirabad"), ("gu", "સબરાબાદ જિલ\u{acd}લો"), ("hi", "सबीराबाद जिला"), ("id", "Sabirabad"), ("it", "distretto di Sabirabad"), ("ja", "サビラバド県"), ("kn", "ಸಬೀರಾಬಾದ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "사비라바트 구"), ("lt", "Sabirabado apskritis"), ("lv", "Sabirabadas rajons"), ("mk", "Сабирабад"), ("ml", "സ\u{d3e}ബിറ\u{d3e}ബ\u{d3e}ദ\u{d4d} ജില\u{d4d}ല"), ("mr", "सबिराबाद जिल\u{94d}हा"), ("ms", "Sabirabad"), ("nb", "Sabirabad"), ("nl", "Sabirabad"), ("no", "Sabirabad"), ("pl", "Rejon Sabirabad"), ("pt", "Sabirabad"), ("ru", "Сабирабадский район"), ("si", "සබ\u{dd2}ර\u{dcf}බ\u{dcf}ද\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Sabirabad"), ("sr", "Сабирабадски рејон"), ("sr_Latn", "Sabirabadski rejon"), ("sv", "Sabirabad Rayonu"), ("ta", "ச\u{bbe}பிரபட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "సబ\u{c40}ర\u{c3e}బ\u{c3e}ద\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาบ\u{e35}ราบ\u{e31}ด"), ("tr", "Sabirabad Rayonu"), ("uk", "Сабірабадський район"), ("ur", "سبیر آباد ضلع"), ("vi", "Sabirabad"), ("zh", "薩比拉巴德區")]),
                        unofficial_name_list: ["Sabirabad"].to_vec(),
                    }
                ),
                (
                    "SAD",
                    Subdivision{
                        name: "SAD",
                        country_alpha2: Alpha2::AZ,
                        code: "SAD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7175), longitude: Some(44.876389), max_latitude: Some(39.7285114), min_latitude: Some(39.6878368), max_longitude: Some(44.9173451), min_longitude: Some(44.85340129999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساداراك"), ("az", "Sədərək"), ("bn", "স\u{9be}দ\u{9be}র\u{9be}ক জেল\u{9be}"), ("ccp", "𑄥𑄓𑄢𑄇\u{11134}"), ("ceb", "Sadarak Rayon"), ("cs", "Sadarak"), ("da", "Sadarak District"), ("de", "Sədərək"), ("el", "Σανταράκ"), ("en", "Sadarak"), ("es", "Sadarak"), ("fa", "شهرستان سدرک"), ("fi", "Sədərəkin piirikunta"), ("fr", "Sadarak"), ("gu", "સદરક જિલ\u{acd}લો"), ("hi", "सदरक जिला"), ("hy", "Սանդրուքի շրջան"), ("id", "Sadarak"), ("it", "Distretto di Sədərək"), ("ja", "サダラク県"), ("kn", "ಸದರಾಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "세데레크 구"), ("lt", "Sadarako apskritis"), ("lv", "Sederekas rajons"), ("mk", "Седерек"), ("mr", "सदरक जिल\u{94d}हा"), ("ms", "Sadarak"), ("nb", "Sədərək"), ("nl", "Sədərək"), ("no", "Sədərək"), ("pl", "Rejon Sədərək"), ("pt", "Sadarak"), ("ru", "Садаракский район"), ("si", "සදරක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Садарачки рејон"), ("sr_Latn", "Sadarački rejon"), ("sv", "Sadarak Rayon"), ("ta", "சடர\u{bbe}க\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "సడ\u{c3e}రక\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาดาร\u{e31}ค"), ("tr", "Sederek Rayonu"), ("uk", "Садарацький район"), ("ur", "صدرک ضلع"), ("vi", "Sedarak"), ("zh", "薩達拉克區")]),
                        unofficial_name_list: ["Sädäräk"].to_vec(),
                    }
                ),
                (
                    "SAH",
                    Subdivision{
                        name: "SAH",
                        country_alpha2: Alpha2::AZ,
                        code: "SAH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.407222), longitude: Some(45.57388900000001), max_latitude: Some(39.4185908), min_latitude: Some(39.39315759999999), max_longitude: Some(45.597639), min_longitude: Some(45.5522345)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شهبوز"), ("az", "Şahbuz"), ("bn", "শ\u{9be}হব\u{9be}জ জেল\u{9be}"), ("ccp", "𑄥𑄦\u{11134}𑄝\u{1112a}𑄌\u{11134}"), ("da", "Shahbuz District"), ("de", "Şahbuz (Rayon)"), ("el", "Σαχμπούζ"), ("en", "Shahbuz"), ("es", "Shahbuz"), ("fa", "شهرستان شاه\u{200c}بوز"), ("fi", "Şahbuzin piirikunta"), ("fr", "Shakhbuz (raion)"), ("gu", "શાહબ\u{ac1}ઝ જિલ\u{acd}લો"), ("hi", "शाहब\u{941}ज\u{93c} जिला"), ("hy", "Շահբուզի շրջան"), ("id", "Shahbuz"), ("it", "Distretto di Şahbuz"), ("ja", "シャフブズ県"), ("kn", "ಶಹಬುಜ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "샤흐부스 구"), ("lt", "Šabuzo apskritis"), ("lv", "Šahbuzas rajons"), ("mk", "Шахбуз (округ)"), ("mr", "शाहब\u{941}झ जिल\u{94d}हा"), ("ms", "Shahbuz"), ("nb", "Şahbuz (distrikt)"), ("nl", "Şahbuz"), ("no", "Şahbuz (distrikt)"), ("pl", "Rejon Şahbuz"), ("pt", "Shakhbuz"), ("ru", "Шахбузский район"), ("si", "ශ\u{dcf}බ\u{dd4}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Шахбушки рејон"), ("sr_Latn", "Šahbuški rejon"), ("sv", "Şahbuz (distrikt)"), ("ta", "ஷ\u{bbe}ஹபிஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ష\u{c3e}హబుజ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ชาฮ\u{e4c}บ\u{e38}ช"), ("tr", "Şahbuz Rayonu"), ("uk", "Шахбузький район"), ("ur", "شاحبوز ضلع"), ("vi", "Shahbuz (quận)"), ("zh", "沙赫布茲區")]),
                        unofficial_name_list: ["Sahbuz"].to_vec(),
                    }
                ),
                (
                    "SAK",
                    Subdivision{
                        name: "SAK",
                        country_alpha2: Alpha2::AZ,
                        code: "SAK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1134662), longitude: Some(47.13169269999999), max_latitude: Some(41.485622), min_latitude: Some(40.7506485), max_longitude: Some(47.6075521), min_longitude: Some(46.808805)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شاكي"), ("az", "Şəki²"), ("bn", "শ\u{9be}কি জেল\u{9be}"), ("ca", "Districte de Shaki"), ("ccp", "𑄥𑄇\u{11128} 𑄎𑄬𑄣"), ("ceb", "Shaki Rayon"), ("da", "Shaki District"), ("de", "Şəki²"), ("el", "Σάκι²"), ("en", "Shaki District"), ("es", "Şəki²"), ("fa", "شهرستان شکی"), ("fi", "Şəkin piirikunta"), ("fr", "Şəki"), ("gu", "શાખી જિલ\u{acd}લો"), ("hi", "शाकी जिला"), ("hu", "Şəki járás"), ("hy", "Շաքիի շրջան"), ("id", "Shaki²"), ("it", "distretto di Şəki"), ("ja", "シャキ県"), ("kn", "ಷಕೀ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "셰키 구"), ("lt", "Šakio apskritis"), ("lv", "Šeki rajons"), ("mk", "Шеки"), ("mr", "शाकी जिल\u{94d}हा"), ("ms", "Shaki"), ("nb", "Şəki²"), ("nl", "Şəki²"), ("no", "Şəki²"), ("pl", "Rejon Şəki"), ("pt", "Shaki²"), ("ru", "Шекинский район"), ("si", "ශක\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Shaki"), ("sr", "Шекински рејон"), ("sr_Latn", "Šekinski rejon"), ("sv", "Sjäki (distrikt)"), ("ta", "ஷ\u{bbe}கி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ష\u{c3e}క\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ซาร\u{e4c}ก\u{e35}\u{e49}"), ("tr", "Şeki Rayonu"), ("uk", "Шекинський район"), ("ur", "شکی ضلع"), ("vi", "Sheki"), ("zh", "舍基區")]),
                        unofficial_name_list: ["Säki"].to_vec(),
                    }
                ),
                (
                    "SAL",
                    Subdivision{
                        name: "SAL",
                        country_alpha2: Alpha2::AZ,
                        code: "SAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.595), longitude: Some(48.979167), max_latitude: Some(39.6451555), min_latitude: Some(39.5387341), max_longitude: Some(49.0261459), min_longitude: Some(48.91036039999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساليان"), ("az", "Salyan"), ("bn", "স\u{9be}লিয\u{9bc}\u{9be}ন জেল\u{9be}"), ("ccp", "𑄥𑄣\u{1112d}𑄚\u{11134}"), ("ceb", "Salyan Rayon"), ("da", "Salyan District"), ("de", "Salyan"), ("el", "Σαλγιάν"), ("en", "Salyan"), ("es", "Salyan"), ("fa", "شهرستان سالیان"), ("fi", "Salyanin piirikunta"), ("fr", "Salyan"), ("gu", "સાયલન જિલ\u{acd}લો"), ("hi", "सल\u{94d}यान जिला"), ("id", "Salyan"), ("it", "distretto di Salyan"), ("ja", "サルヤン県"), ("kn", "ಸಾಲ\u{ccd}ಯಾನ\u{ccd} ಡ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ\u{ccd}ಟ\u{ccd}"), ("ko", "살리안 구"), ("lt", "Saljano apskritis"), ("lv", "Saljanas rajons"), ("mk", "Саљан"), ("mr", "सल\u{94d}याण जिल\u{94d}हा"), ("ms", "Salyan"), ("nb", "Salyan"), ("nl", "Salyan"), ("no", "Salyan"), ("pl", "Rejon Salyan"), ("pt", "Salyan"), ("ru", "Сальянский район"), ("si", "සල\u{dca}ය\u{dcf}න\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Salyan"), ("sr", "Саљански рејон"), ("sr_Latn", "Saljanski rejon"), ("sv", "Saljan"), ("ta", "சல\u{bcd}யன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "సల\u{c4d}య\u{c3e}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาลยาน"), ("tr", "Salyan Rayonu"), ("uk", "Сальянський район"), ("ur", "سالیان ضلع"), ("vi", "Salyan"), ("zh", "薩利揚區")]),
                        unofficial_name_list: ["Salyan"].to_vec(),
                    }
                ),
                (
                    "SAR",
                    Subdivision{
                        name: "SAR",
                        country_alpha2: Alpha2::AZ,
                        code: "SAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5687523), longitude: Some(45.0823321), max_latitude: Some(39.785526), min_latitude: Some(39.3825), max_longitude: Some(45.2860981), min_longitude: Some(44.82024)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Şərur"), ("ccp", "𑄥𑄢\u{1112a}𑄢\u{11134}"), ("ceb", "Sharur Rayon"), ("de", "Şərur (Rayon)"), ("en", "Sharur"), ("es", "Sharur"), ("fa", "شهرستان شرور"), ("fi", "Şərurin piirikunta"), ("fr", "Sharur"), ("he", "סחרור"), ("hy", "Շարուրի շրջան"), ("id", "Sharur"), ("it", "Distretto di Şərur"), ("ja", "シャルル県"), ("ka", "შარურის რაიონი"), ("ko", "셰루르 구"), ("mk", "Шерур"), ("ms", "Sharur"), ("nb", "Şərur (distrikt)"), ("nl", "Şərur"), ("no", "Şərur (distrikt)"), ("pl", "Rejon Şərur"), ("pt", "Sharur"), ("ru", "Шарурский район"), ("sr", "Шарурски рејон"), ("sr_Latn", "Šarurski rejon"), ("sv", "Sharur Rayon"), ("tr", "Şerur Rayonu"), ("uk", "Шарурський район"), ("vi", "Sherur (quận)"), ("zh", "沙魯爾區")]),
                        unofficial_name_list: ["Särur"].to_vec(),
                    }
                ),
                (
                    "SAT",
                    Subdivision{
                        name: "SAT",
                        country_alpha2: Alpha2::AZ,
                        code: "SAT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.930833), longitude: Some(48.36944399999999), max_latitude: Some(39.9668589), min_latitude: Some(39.8768098), max_longitude: Some(48.4565735), min_longitude: Some(48.2874871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساتلي"), ("az", "Saatlı"), ("bn", "স\u{9be}টলি জেল\u{9be}"), ("ccp", "𑄥𑄖𑄣\u{11129}"), ("ceb", "Saatly Rayon"), ("da", "Saatly District"), ("de", "Saatlı"), ("el", "Σαάτλι"), ("en", "Saatly"), ("es", "Saatlı"), ("fa", "شهرستان ساعتلی"), ("fi", "Saatlın piirikunta"), ("fr", "Saatlı"), ("gu", "સાટલી જિલ\u{acd}લો"), ("hi", "सातली जिला"), ("id", "Saatly"), ("it", "distretto di Saatlı"), ("ja", "サアトル"), ("kn", "ಸಾಟ\u{ccd}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "사틀르 구"), ("lt", "Saatlio apskritis"), ("lv", "Saatli rajons"), ("mk", "Саатли"), ("mr", "साटली जिल\u{94d}हा"), ("ms", "Saatly"), ("nb", "Saatlı"), ("nl", "Saatlı"), ("no", "Saatlı"), ("pl", "Rejon Saatlı"), ("pt", "Saatly"), ("ru", "Саатлинский район"), ("si", "ස\u{dcf}ල\u{dca}ට\u{dca}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Saatly"), ("sr", "Сатлински рејон"), ("sr_Latn", "Satlinski rejon"), ("sv", "Saatlı Rayonu"), ("ta", "ச\u{bbe}ட\u{bcd}லய\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}ట\u{c4d}ల\u{c40} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซ\u{e4a}าทล\u{e35}\u{e48}"), ("tr", "Saatlı Rayonu"), ("uk", "Район Саатлі"), ("ur", "سااتلی ضلع"), ("vi", "Saatly"), ("zh", "薩特雷區")]),
                        unofficial_name_list: ["Saatli"].to_vec(),
                    }
                ),
                (
                    "SBN",
                    Subdivision{
                        name: "SBN",
                        country_alpha2: Alpha2::AZ,
                        code: "SBN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شابران"), ("az", "Şabran (rayon)"), ("bn", "স\u{9be}ব\u{9cd}র\u{9be}ন জেল\u{9be}"), ("ccp", "𑄥𑄛\u{11134}𑄢𑄚\u{11134}"), ("ceb", "Shabran Rayon"), ("cs", "Šabran"), ("da", "Shabran District"), ("de", "Şabran (Rayon)"), ("el", "Σάμπραν"), ("en", "Shabran"), ("es", "Şabran (raión)"), ("fa", "دوه\u{200c}چی"), ("fi", "Şabranin piirikunta"), ("fr", "Şabran (raion)"), ("gu", "શબ\u{acd}રાન જિલ\u{acd}લો"), ("hi", "शबरान जिला"), ("id", "Davachi"), ("it", "distretto di Şabran"), ("ja", "ダヴァヒ県"), ("kn", "ಶಬ\u{ccd}ರಾನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "샤브란 구"), ("lt", "Šabrano apskritis"), ("lv", "Šabranas rajons"), ("mk", "Шабран"), ("mn", "Шабран аймаг"), ("mr", "शबरण जिल\u{94d}हा"), ("ms", "Davachi"), ("nb", "Şabran"), ("nl", "Dəvəçi"), ("no", "Şabran"), ("pl", "Rejon Dəvəçi"), ("pt", "Shabran"), ("ro", "Șabran"), ("ru", "Шабранский район"), ("si", "ශබ\u{dca}රන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Шабрански рејон"), ("sr_Latn", "Šabranski rejon"), ("sv", "Shabran Rayon"), ("ta", "ச\u{bbe}ப\u{bcd}ர\u{bbe}ன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "షబ\u{c4d}రన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดคอปเปอเบล"), ("tr", "Şabran rayonu"), ("uk", "Шабранський район"), ("ur", "شابران ضلع"), ("vi", "Quận Shabran"), ("zh", "迪維奇區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SIY",
                    Subdivision{
                        name: "SIY",
                        country_alpha2: Alpha2::AZ,
                        code: "SIY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.0783833), longitude: Some(49.1118478), max_latitude: Some(41.09882270000001), min_latitude: Some(41.0619124), max_longitude: Some(49.1377259), min_longitude: Some(49.0923215)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيازن"), ("az", "Siyəzən"), ("bn", "সিজ\u{9be}ন জেল\u{9be}"), ("ccp", "𑄥\u{11128}𑄠𑄎\u{11127}𑄚\u{11134}"), ("ceb", "Siazan Rayon"), ("da", "Siazan District"), ("de", "Siyəzən"), ("el", "Σιαζάν"), ("en", "Siazan"), ("es", "Siyəzən"), ("fa", "شهرستان سیاه\u{200c}زن"), ("fi", "Siyəzənin piirikunta"), ("fr", "Siyəzən"), ("gu", "સિઆઝાન જિલ\u{acd}લો"), ("hi", "सिज\u{93c}ान जिला"), ("hu", "Siyəzəni járás"), ("id", "Siazan"), ("it", "distretto di Siyəzən"), ("ja", "シアザン県"), ("kn", "ಸ\u{cbf}ಯಾಜ\u{ccd}"), ("ko", "시예젠 구"), ("lt", "Siazano apskritis"), ("lv", "Sijezenas rajons"), ("mk", "Сијезен"), ("mr", "सिझन जिल\u{94d}हा"), ("ms", "Siazan"), ("nb", "Siyəzən"), ("nl", "Siyəzən"), ("no", "Siyəzən"), ("pl", "Rejon Siyəzən"), ("pt", "Siazan"), ("ru", "Сиазаньский район"), ("si", "ස\u{dd2}ය\u{dcf}ස\u{dcf}න\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Siazan"), ("sr", "Сијазански рејон"), ("sr_Latn", "Sijazanski rejon"), ("sv", "Siyəzən Rayonu"), ("ta", "சித\u{bbe}சன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3f}య\u{c3e}జన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เซ\u{e4a}ยแซน"), ("tr", "Siyezen Rayonu"), ("uk", "Район Сіазань"), ("ur", "سیاہ زن ضلع"), ("vi", "Siazan"), ("zh", "錫阿贊區")]),
                        unofficial_name_list: ["Siyäzän"].to_vec(),
                    }
                ),
                (
                    "SKR",
                    Subdivision{
                        name: "SKR",
                        country_alpha2: Alpha2::AZ,
                        code: "SKR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.88123969999999), longitude: Some(46.0179009), max_latitude: Some(41.129959), min_latitude: Some(40.570687), max_longitude: Some(46.323192), min_longitude: Some(45.713568)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شمكير"), ("az", "Şəmkir"), ("be", "Шамкірскі раён"), ("bn", "শ\u{9be}মকির জেল\u{9be}"), ("ccp", "𑄥𑄟\u{11134}𑄇\u{11128}𑄢\u{11134}"), ("ceb", "Shamkir Rayon"), ("da", "Shamkir District"), ("de", "Şəmkir"), ("el", "Επαρχία Σαμκίρ"), ("en", "Shamkir"), ("es", "Şəmkir"), ("fa", "شهرستان شمکیر"), ("fi", "Şəmkirin piirikunta"), ("fr", "Şəmkir"), ("gu", "શામકિર જિલ\u{acd}લો"), ("hi", "शामकीर जिला"), ("hu", "Şəmkiri járás"), ("hy", "Շամխորի շրջան"), ("id", "Shamkir"), ("it", "distretto di Şəmkir"), ("ja", "シャムキル県"), ("ka", "შამქირის რაიონი"), ("kn", "ಶಾಮ\u{ccd}ಕ\u{cbf}ರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "솀키르 구"), ("lt", "Šamkiro apskritis"), ("lv", "Šemkiras rajons"), ("mk", "Шемкир"), ("mr", "शामकीर जिल\u{94d}हा"), ("ms", "Shamkir"), ("nb", "Şəmkir"), ("nl", "Şəmkir"), ("no", "Şəmkir"), ("pl", "Rejon Şəmkir"), ("pt", "Shamkir"), ("ru", "Шамкирский район"), ("si", "ශ\u{dcf}ම\u{dca}ක\u{dd2}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Shamkir"), ("sr", "Шамкирски рејон"), ("sr_Latn", "Šamkirski rejon"), ("sv", "Sjämkir"), ("ta", "ஷ\u{bbe}ம\u{bcd}கிர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ష\u{c3e}ంక\u{c40}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตชามเคอ"), ("tr", "Şemkir Rayonu"), ("uk", "Шамкірський район"), ("ur", "شمخور ضلع"), ("vi", "Shamkir"), ("zh", "沙姆基爾區")]),
                        unofficial_name_list: ["Sämkir"].to_vec(),
                    }
                ),
                (
                    "SM",
                    Subdivision{
                        name: "SM",
                        country_alpha2: Alpha2::AZ,
                        code: "SM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.5854765), longitude: Some(49.6317411), max_latitude: Some(40.6572359), min_latitude: Some(40.5001047), max_longitude: Some(49.74102980000001), min_longitude: Some(49.5211315)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sumqayit"), ("ar", "سومقاييت"), ("az", "Sumqayıt"), ("be", "Сумгаіт"), ("bg", "Сумгаит"), ("bn", "স\u{9c1}মক\u{9be}য\u{9bc}িত"), ("ca", "Sumqayit"), ("ccp", "𑄥\u{1112a}𑄟𑄇\u{1112e}𑄃\u{11128}𑄖\u{11134}"), ("ceb", "Sumqayıt"), ("cs", "Sumqayıt"), ("cy", "Sumqayıt"), ("da", "Sumqayit"), ("de", "Sumqayıt"), ("el", "Σουμγκαγήτ"), ("en", "Sumqayit"), ("es", "Sumqayit"), ("et", "Sumqayıt"), ("eu", "Sumqayit"), ("fa", "سومقاییت"), ("fi", "Sumqayıt"), ("fr", "Sumqayıt"), ("gu", "સ\u{ac1}મક\u{acd}વાયિત"), ("he", "סומגאיט"), ("hi", "स\u{941}म\u{94d}क\u{93c}यित"), ("hu", "Sumqayit"), ("hy", "Սումգայիթ"), ("id", "Sumqayit"), ("is", "Sumqayit"), ("it", "Sumqayıt"), ("ja", "スムガイト"), ("ka", "სუმგაითი"), ("kn", "ಸುಮ\u{ccd}ಮಾಯತ\u{ccd}"), ("ko", "숨가이트"), ("lt", "Sumgajitas"), ("lv", "Sumgajita"), ("mk", "Сумгајит"), ("mn", "Сумгаит"), ("mr", "स\u{941}मगायीत"), ("ms", "Sumqayit"), ("nb", "Sumqayit"), ("nl", "Sumqayıt"), ("no", "Sumqayit"), ("pl", "Sumgait"), ("pt", "Sumqayit"), ("ro", "Sumqayıt"), ("ru", "Сумгаит"), ("si", "ස\u{dd4}ම\u{dca}කෙය\u{dd2}ට\u{dca}"), ("sl", "Sumgait"), ("sq", "Sumqayit"), ("sr", "Сумгајит"), ("sr_Latn", "Sumgajit"), ("sv", "Sumqayıt"), ("ta", "ஸும\u{bcd}ப\u{bcd}கயிட\u{bcd}"), ("te", "సంఖ\u{c3e}యత\u{c4d}"), ("th", "ซ\u{e31}มกาย\u{e34}ต"), ("tk", "Sumgait"), ("tr", "Sumgayıt"), ("uk", "Сумгаїт"), ("ur", "سومقاییت"), ("uz", "Sumgait"), ("vi", "Sumqayit"), ("zh", "苏姆盖特")]),
                        unofficial_name_list: ["Sumqayit"].to_vec(),
                    }
                ),
                (
                    "SMI",
                    Subdivision{
                        name: "SMI",
                        country_alpha2: Alpha2::AZ,
                        code: "SMI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.63187310000001), longitude: Some(48.6363801), max_latitude: Some(40.6541736), min_latitude: Some(40.6056446), max_longitude: Some(48.6714078), min_longitude: Some(48.6082792)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شاماخي"), ("az", "Şamaxı"), ("ca", "Districte de Şamaxı"), ("ccp", "𑄥𑄟𑄈\u{11128}"), ("ceb", "Shamakhi Rayon"), ("cs", "Šamachi"), ("de", "Şamaxı"), ("en", "Shamakhi"), ("es", "Şamaxı"), ("fa", "شهرستان شماخی"), ("fi", "Şamaxın piirikunta"), ("fr", "Şamaxı"), ("hu", "Şamaxı járás"), ("id", "Shamakhi"), ("it", "distretto di Şamaxı"), ("ja", "シャマフ県"), ("ka", "შემახის რაიონი"), ("ko", "샤마흐 구"), ("mk", "Шамахи"), ("ms", "Shamakhi"), ("nb", "Şamaxı"), ("nl", "Şamaxı"), ("no", "Şamaxı"), ("pl", "Rejon Şamaxı"), ("pt", "Shamakhi"), ("ru", "Шемахинский район"), ("sq", "Shamakhi"), ("sr", "Шемахински рејон"), ("sr_Latn", "Šemahinski rejon"), ("sv", "Sjamachy"), ("tr", "Şamahı Rayonu"), ("uk", "Шамахинський район"), ("vi", "Shamakhy"), ("zh", "沙馬基區")]),
                        unofficial_name_list: ["Samaxi"].to_vec(),
                    }
                ),
                (
                    "SMX",
                    Subdivision{
                        name: "SMX",
                        country_alpha2: Alpha2::AZ,
                        code: "SMX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.765833), longitude: Some(46.40888899999999), max_latitude: Some(40.7820362), min_latitude: Some(40.747452), max_longitude: Some(46.42375939999999), min_longitude: Some(46.3920022)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساموخ"), ("az", "Samux"), ("bn", "স\u{9be}মোক জেল\u{9be}"), ("ccp", "𑄥𑄟\u{1112a}𑄇\u{11134}"), ("ceb", "Samukh Rayon"), ("da", "Samukh District"), ("de", "Samux"), ("el", "Σαμούκ"), ("en", "Samukh"), ("es", "Samux"), ("fa", "شهرستان ساموخ"), ("fi", "Samuxin piirikunta"), ("fr", "Samux"), ("gu", "સમ\u{ac1}ખ જિલ\u{acd}લો"), ("hi", "सम\u{941}ख जिला"), ("hu", "Samuxi járás"), ("id", "Samukh"), ("it", "distretto di Samux"), ("ja", "サムフ県"), ("kn", "ಸಾಮುಖ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "사무흐 구"), ("lt", "Samuko apskritis"), ("lv", "Samuhas rajons"), ("mk", "Самух"), ("mr", "साम\u{941}ख जिल\u{94d}हा"), ("ms", "Samukh"), ("nb", "Samux"), ("nl", "Samux"), ("no", "Samux"), ("pl", "Rejon Samux"), ("pt", "Samukh"), ("ru", "Самухский район"), ("si", "සම\u{dd4}ඛ\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Samukh"), ("sr", "Самушки рејон"), ("sr_Latn", "Samuški rejon"), ("sv", "Samux Rayonu"), ("ta", "சமூக\u{bcd}ஹ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "సముఖ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ซาม\u{e38}ก"), ("tr", "Samuh Rayonu"), ("uk", "Самухський район"), ("ur", "ساموخ ضلع"), ("vi", "Samuh"), ("zh", "薩穆赫區")]),
                        unofficial_name_list: ["Samux"].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::AZ,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شيروان"), ("az", "Şirvan"), ("be", "Шырван"), ("ccp", "𑄥\u{11128}𑄢𑄞\u{11127}𑄚\u{11134}"), ("ceb", "Shirvan"), ("cs", "Širvan"), ("de", "Şirvan"), ("el", "Σιρβάν"), ("en", "Shirvan"), ("es", "Shirvan"), ("fa", "شیروان"), ("fi", "Şirvan"), ("fr", "Şirvan"), ("he", "שירוואן"), ("hy", "Շիրվան"), ("id", "Shirvan"), ("it", "Şirvan"), ("ja", "シルヴァン (市)"), ("ka", "შირვანი"), ("ko", "시르반"), ("lt", "Širvanas"), ("lv", "Širvana"), ("mk", "Ширван"), ("mn", "Азербайжаны Ширван"), ("ms", "Shirvan"), ("nl", "Şirvan"), ("pl", "Szyrwan"), ("pt", "Shirvan"), ("ro", "Șirvan"), ("ru", "Ширван"), ("sq", "Shirvan"), ("sr", "Ширван"), ("sr_Latn", "Širvan"), ("sv", "Shirvan"), ("tr", "Şirvan"), ("uk", "Ширван"), ("ur", "شیروان"), ("zh", "希爾萬")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SUS",
                    Subdivision{
                        name: "SUS",
                        country_alpha2: Alpha2::AZ,
                        code: "SUS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7537438), longitude: Some(46.7464755), max_latitude: Some(39.7725926), min_latitude: Some(39.7456061), max_longitude: Some(46.765623), min_longitude: Some(46.7307758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شوشا"), ("az", "Şuşa"), ("bn", "স\u{9c1}শ\u{9be} জেল\u{9be}"), ("ccp", "𑄥\u{1112a}𑄥"), ("da", "Shusha District"), ("de", "Şuşa"), ("el", "Σούσα"), ("en", "Shusha"), ("es", "Şuşa"), ("fa", "شهرستان شوشا"), ("fi", "Şuşan piirikunta"), ("fr", "Choucha"), ("gu", "શ\u{ac1}શા જિલ\u{acd}લો"), ("he", "שושה"), ("hi", "श\u{941}शा जिला"), ("hu", "Şuşai járás"), ("hy", "Շուշիի շրջան"), ("id", "Shusha"), ("it", "distretto di Şuşa"), ("ja", "シュシャ県"), ("kn", "ಶುಶಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "슈샤 구"), ("lt", "Šušos apskritis"), ("lv", "Šušas rajons"), ("mk", "Шуша"), ("mr", "श\u{941}सा विभाग"), ("ms", "Shusha"), ("nb", "Şuşa"), ("nl", "Şuşa"), ("no", "Şuşa"), ("pl", "Rejon Şuşa"), ("pt", "Shusha"), ("ru", "Шушинский район"), ("si", "ශ\u{dd4}ෂ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Shusha"), ("sr", "Шушински рејон"), ("sr_Latn", "Šušinski rejon"), ("sv", "Shusha (distrikt)"), ("ta", "ஷுஷ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "షూష\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซ\u{e39}ซา"), ("tr", "Şuşa Rayonu"), ("uk", "Шушинський район"), ("ur", "شوشا ضلع"), ("uz", "Shusha tumani"), ("vi", "Shusha"), ("zh", "舒沙區")]),
                        unofficial_name_list: ["Susa"].to_vec(),
                    }
                ),
                (
                    "TAR",
                    Subdivision{
                        name: "TAR",
                        country_alpha2: Alpha2::AZ,
                        code: "TAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.333333), longitude: Some(46.916667), max_latitude: Some(40.3576638), min_latitude: Some(40.3265895), max_longitude: Some(46.96801199999999), min_longitude: Some(46.9110202)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تارتار"), ("az", "Tərtər"), ("bn", "ত\u{9be}রত\u{9be}র জেল\u{9be}"), ("ccp", "𑄑𑄢\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Tartar Rayon"), ("da", "Tartar District"), ("de", "Tərtər"), ("el", "Ταρτάρ"), ("en", "Tartar"), ("es", "Tərtər"), ("fa", "شهرستان ترتر"), ("fi", "Tərtərin piirikunta"), ("fr", "Tartar"), ("gu", "ટર\u{acd}ટાર જિલ\u{acd}લો"), ("hi", "टारटर जिला"), ("hu", "Tərtəri járás"), ("hy", "Թարթառի շրջան"), ("id", "Tartar"), ("it", "distretto di Tərtər"), ("ja", "タルタル県"), ("kn", "ಟಾರ\u{ccd}ಟಾರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "테르테르 구"), ("lt", "Tartaro apskritis"), ("lv", "Terteras rajons"), ("mk", "Тертер"), ("mr", "टाटार जिल\u{94d}हा"), ("ms", "Tartar"), ("nb", "Tərtər"), ("nl", "Tərtər"), ("no", "Tərtər"), ("pl", "Rejon Tərtər"), ("pt", "Tartar"), ("ru", "Тертерский район"), ("si", "ටර\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Tartar"), ("sr", "Тертерски рејон"), ("sr_Latn", "Terterski rejon"), ("sv", "Tärtär"), ("ta", "ட\u{bbe}ர\u{bcd}ட\u{bcd}ட\u{bbe}ர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c3e}ర\u{c4d}ట\u{c3e}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ทาร\u{e4c}ทาร\u{e4c}"), ("tr", "Terter Rayonu"), ("uk", "Тертерський район"), ("ur", "تارتار ضلع"), ("vi", "Terter"), ("zh", "泰爾泰爾區")]),
                        unofficial_name_list: ["Tärtär"].to_vec(),
                    }
                ),
                (
                    "TOV",
                    Subdivision{
                        name: "TOV",
                        country_alpha2: Alpha2::AZ,
                        code: "TOV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9922), longitude: Some(45.62889999999999), max_latitude: Some(41.0084997), min_latitude: Some(40.9754255), max_longitude: Some(45.6436444), min_longitude: Some(45.5907513)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة توفوز"), ("az", "Tovuz"), ("bn", "ত\u{9c1}ভোজ জেল\u{9be}"), ("ccp", "𑄑\u{11127}𑄞\u{1112a}𑄌\u{11134}"), ("ceb", "Tovuz Rayon"), ("da", "Tovuz District"), ("de", "Tovuz"), ("el", "Τοβούζ"), ("en", "Tovuz"), ("es", "Tovuz"), ("fa", "شهرستان تووز"), ("fi", "Tovuzin piirikunta"), ("fr", "Tovuz"), ("gu", "તોવ\u{ac1}ઝ જિલ\u{acd}લો"), ("hi", "तोव\u{942}ज\u{93c} जिला"), ("hu", "Tovuzi járás"), ("hy", "Թովուզի շրջան"), ("id", "Tovuz"), ("it", "distretto di Tovuz"), ("ja", "トヴズ県"), ("kn", "ಟೋವಝ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "토부스 구"), ("lt", "Tovuzo apskritis"), ("lv", "Tovuzas rajons"), ("mk", "Товуз"), ("mr", "त\u{947}व\u{94d}हज जिल\u{94d}हा"), ("ms", "Tovuz"), ("nb", "Tovuz"), ("nl", "Tovuz"), ("no", "Tovuz"), ("pl", "Rejon Tovuz"), ("pt", "Tovuz"), ("ru", "Таузский район"), ("si", "ටෝව\u{dd4}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Tovuz"), ("sr", "Товушки рејон"), ("sr_Latn", "Tovuški rejon"), ("sv", "Tovuz Rayonu"), ("ta", "டோவுஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4b}వుజ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตทาว\u{e38}ช"), ("tr", "Tovuz Rayonu"), ("uk", "Товузький район"), ("ur", "توووز ضلع"), ("vi", "Tovuz"), ("zh", "塔烏茲區")]),
                        unofficial_name_list: ["Tovuz"].to_vec(),
                    }
                ),
                (
                    "UCA",
                    Subdivision{
                        name: "UCA",
                        country_alpha2: Alpha2::AZ,
                        code: "UCA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.518333), longitude: Some(47.654167), max_latitude: Some(40.5296863), min_latitude: Some(40.4896513), max_longitude: Some(47.6742697), min_longitude: Some(47.6283074)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أويار"), ("az", "Ucar"), ("bn", "উজ\u{9be}র জেল\u{9be}"), ("ccp", "𑄅\u{1112a}𑄎𑄢\u{11134}"), ("ceb", "Ujar Rayon"), ("da", "Ujar District"), ("de", "Ucar"), ("el", "Ούτζαρ"), ("en", "Ujar"), ("es", "Ucar"), ("fa", "شهرستان اوجار"), ("fi", "Ucarin piirikunta"), ("fr", "Ucar"), ("gu", "ઉજર જિલ\u{acd}લો"), ("hi", "उजर जिला"), ("id", "Ujar"), ("it", "distretto di Ucar"), ("ja", "ウジャル県"), ("kn", "ಉಜರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "우자르 구"), ("lt", "Udžaro apskritis"), ("lv", "Udžaras rajons"), ("mk", "Уџар"), ("mr", "उर\u{94d}जर जिल\u{94d}हा"), ("ms", "Ujar"), ("nb", "Ucar"), ("nl", "Ucar"), ("no", "Ucar"), ("pl", "Rejon Ucar"), ("pt", "Ujar"), ("ru", "Уджарский район"), ("si", "උජ\u{dcf}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Ujar"), ("sr", "Уџарски рејон"), ("sr_Latn", "Udžarski rejon"), ("sv", "Udzjar"), ("ta", "உஜ\u{bbe}ர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఉజర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e39}จาร\u{e4c}"), ("tr", "Ucar Rayonu"), ("uk", "Уджарський район"), ("ur", "اجڑ ضلع"), ("vi", "Ujar"), ("zh", "烏賈雷區")]),
                        unofficial_name_list: ["Ucar"].to_vec(),
                    }
                ),
                (
                    "XA",
                    Subdivision{
                        name: "XA",
                        country_alpha2: Alpha2::AZ,
                        code: "XA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.8196258), longitude: Some(46.7594431), max_latitude: Some(39.8520223), min_latitude: Some(39.7995519), max_longitude: Some(46.7890549), min_longitude: Some(46.7298317)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Stepanakert"), ("ar", "سيتباناكيرت"), ("az", "Xankəndi"), ("be", "Горад Сцепанакерт"), ("bg", "Степанакерт"), ("bn", "স\u{9cd}টেপ\u{9be}ন\u{9be}কেট"), ("bs", "Stepanakert"), ("ca", "Stepanakert"), ("ccp", "𑄌\u{11133}𑄑𑄬𑄛𑄚\u{11134}𑄇𑄢\u{11133}𑄑\u{11134}"), ("ceb", "Xankandi"), ("cs", "Stěpanakert"), ("da", "Stepanakert"), ("de", "Stepanakert"), ("el", "Στεπανακέρτ"), ("en", "Stepanakert"), ("es", "Stepanakert"), ("et", "Xankəndi"), ("eu", "Stepanakert"), ("fa", "خان\u{200c}کندی"), ("fi", "Stepanakert"), ("fr", "Stepanakert"), ("gl", "Stepanakert"), ("gu", "સ\u{acd}ટ\u{ac7}પનક\u{ac7}ર\u{acd}ટ"), ("he", "סטפנקרט"), ("hi", "सट\u{947}पणाक\u{947}र\u{94d}ट"), ("hr", "Stepanakert"), ("hu", "Sztepanakert"), ("hy", "Ստեփանակերտ"), ("id", "Stepanakert"), ("it", "Step’anakert"), ("ja", "ステパナケルト"), ("ka", "ხანქენდი"), ("kn", "ಸ\u{ccd}ಟ\u{cc6}ಟ\u{cc6}ನಾಕರ\u{ccd}ಟ\u{ccd}"), ("ko", "스테파나케르트"), ("ky", "Степанакерт"), ("lt", "Chankendi"), ("lv", "Stepanakerta"), ("mk", "Степанакерт"), ("mr", "स\u{94d}ट\u{947}पनाकर\u{94d}ट"), ("ms", "Stepanakert"), ("nb", "Stepanakert"), ("nl", "Stepanakert"), ("no", "Stepanakert"), ("pl", "Stepanakert"), ("pt", "Stepanakert"), ("ro", "Stepanakert"), ("ru", "Степанакерт"), ("si", "ස\u{dca}ටේපනකර\u{dca}ට\u{dca}"), ("sk", "Stepanakert"), ("sl", "Stepanakert"), ("sq", "Khankendi"), ("sv", "Stepanakert"), ("ta", "எசுடெப\u{bbe}னெகெத\u{bcd}"), ("te", "స\u{c4d}ట\u{c46}పన\u{c3e}కర\u{c4d}ట\u{c4d}"), ("th", "สเตพานาแกร\u{e4c}ต"), ("tr", "Hankendi"), ("uk", "Степанакерт"), ("ur", "خان کندی"), ("uz", "Xonkendi"), ("vi", "Khankendy"), ("yue", "斯捷潘納克特"), ("yue_Hans", "斯捷潘纳克特"), ("zh", "斯捷潘納克特")]),
                        unofficial_name_list: ["Xankändi"].to_vec(),
                    }
                ),
                (
                    "XAC",
                    Subdivision{
                        name: "XAC",
                        country_alpha2: Alpha2::AZ,
                        code: "XAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.470833), longitude: Some(48.809722), max_latitude: Some(41.4863344), min_latitude: Some(41.4395092), max_longitude: Some(48.8417817), min_longitude: Some(48.774619)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خاتشماز"), ("az", "Xaçmaz"), ("bn", "ক\u{9be}কহ\u{9be}ময জেল\u{9be}"), ("ccp", "𑄈𑄌\u{11134}𑄟𑄌\u{11134}"), ("ceb", "Khachmaz Rayon"), ("da", "Khachmaz District"), ("de", "Xaçmaz"), ("el", "Κατσμάζ"), ("en", "Khachmaz"), ("es", "Xaçmaz"), ("fa", "شهرستان خاچماز"), ("fi", "Xaçmazin piirikunta"), ("fr", "Xaçmaz"), ("gu", "ખાકમાઝ જિલ\u{acd}લો"), ("hi", "खाकमज जिला"), ("hu", "Xaçmazi járás"), ("id", "Khachmaz"), ("it", "distretto di Xaçmaz"), ("ja", "ハヒマズ県"), ("ka", "ხაჩმაზის რაიონი"), ("kn", "ಖಚ\u{ccd}ಮಾಜ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "하치마스 구"), ("lt", "Chačmazo apskritis"), ("lv", "Hačmazas rajons"), ("mk", "Хачмаз"), ("mr", "खाकमझ जिल\u{94d}हा"), ("ms", "Khachmaz"), ("nb", "Xaçmaz"), ("nl", "Xaçmaz"), ("no", "Xaçmaz"), ("pl", "Rejon Xaçmaz"), ("pt", "Khachmaz"), ("ru", "Хачмасский район"), ("si", "කච\u{dca}ම\u{dcf}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Khachmaz"), ("sr", "Хачмашки рејон"), ("sr_Latn", "Hačmaški rejon"), ("sv", "Khachmaz Rayon"), ("ta", "க\u{bbe}ச\u{bcd}ம\u{bbe}ஸ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖ\u{c3e}చ\u{c4d}మ\u{c3e}జ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ค\u{e31}ชมาช"), ("tr", "Haçmaz Rayonu"), ("uk", "Хачмазький район"), ("ur", "خاشماز ضلع"), ("vi", "Khachmaz"), ("zh", "哈奇馬斯區")]),
                        unofficial_name_list: ["Xaçmaz"].to_vec(),
                    }
                ),
                (
                    "XCI",
                    Subdivision{
                        name: "XCI",
                        country_alpha2: Alpha2::AZ,
                        code: "XCI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.9132553), longitude: Some(46.794305), max_latitude: Some(39.9206634), min_latitude: Some(39.9040514), max_longitude: Some(46.8147892), min_longitude: Some(46.770262)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خوجالي"), ("az", "Xocalı"), ("bn", "খ\u{9c1}জ\u{9be}লি জেল\u{9be}"), ("ccp", "𑄈\u{11127}𑄎\u{11127}𑄣\u{11128}"), ("ceb", "Xocalı Rayonu"), ("cs", "Chodžali"), ("da", "Khojali District"), ("de", "Xocalı"), ("el", "Κχοτζάλι"), ("en", "Khojali"), ("es", "Xocalı"), ("fa", "شهرستان خوجالی"), ("fi", "Xocalın piirikunta"), ("fr", "Khodjaly"), ("gu", "ખોઝાલી જિલ\u{acd}લો"), ("he", "חוג׳אלי"), ("hi", "खोजाली जिला"), ("hu", "Xocalı járás"), ("hy", "Խոջալուի շրջան"), ("id", "Khojali"), ("it", "distretto di Xocalı"), ("ja", "ホジャリ県"), ("ka", "ხოჯალის რაიონი"), ("kn", "ಖೋಜಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "호잘르 구"), ("lt", "Kojalio apskritis"), ("lv", "Hodžalinas rajons"), ("mk", "Хоџали"), ("mr", "खोजली जिल\u{94d}हा"), ("ms", "Daerah Khojali"), ("nb", "Xocalı"), ("nl", "Xocalı"), ("no", "Xocalı"), ("pl", "Rejon Xocalı"), ("pt", "Khojali"), ("ru", "Ходжалинский район"), ("si", "කොජ\u{dcf}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Khojali"), ("sr", "Хоџалински рејон"), ("sr_Latn", "Hodžalinski rejon"), ("sv", "Xocalı Rayonu"), ("ta", "க\u{bcd}ஹோஜ\u{bbe}லி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఖ\u{c4b}జ\u{c3e}ల\u{c40} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโคจาล\u{e34}"), ("tr", "Hocalı Rayonu"), ("uk", "Ходжалинський район"), ("ur", "خوجالی ضلع"), ("vi", "Khojaly"), ("zh", "霍賈雷區")]),
                        unofficial_name_list: ["Xocali"].to_vec(),
                    }
                ),
                (
                    "XIZ",
                    Subdivision{
                        name: "XIZ",
                        country_alpha2: Alpha2::AZ,
                        code: "XIZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9), longitude: Some(49.066667), max_latitude: Some(40.9251219), min_latitude: Some(40.8974897), max_longitude: Some(49.0894889), min_longitude: Some(49.0516375)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Xızı"), ("ccp", "𑄈\u{11128}𑄏\u{11128}"), ("ceb", "Khizi Rayon"), ("de", "Xızı"), ("en", "Khizi"), ("es", "Xızı"), ("fa", "شهرستان خیزی"), ("fi", "Xızın piirikunta"), ("fr", "Xızı"), ("hu", "Xızı járás"), ("id", "Khizi"), ("it", "distretto di Xızı"), ("ja", "ヒジ県"), ("ko", "흐즈 구"), ("mk", "Хизи"), ("ms", "Rayon Khizi"), ("nb", "Xızı"), ("nl", "Xızı"), ("no", "Xızı"), ("pl", "Rejon Xızı"), ("pt", "Khizi"), ("ru", "Хызинский район"), ("sq", "Khizi"), ("sr", "Хизински рејон"), ("sr_Latn", "Hizinski rejon"), ("sv", "Khizi Rayon"), ("tr", "Hızı Rayonu"), ("vi", "Khyzy"), ("zh", "基茲區")]),
                        unofficial_name_list: ["Xizi"].to_vec(),
                    }
                ),
                (
                    "XVD",
                    Subdivision{
                        name: "XVD",
                        country_alpha2: Alpha2::AZ,
                        code: "XVD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7060287), longitude: Some(47.064533), max_latitude: Some(39.883672), min_latitude: Some(39.3751951), max_longitude: Some(47.345414), min_longitude: Some(46.63206090000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Xocavənd"), ("ccp", "𑄈\u{11127}𑄎𑄞𑄬𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Xocavənd Rayonu"), ("de", "Xocavənd"), ("en", "Khojavend"), ("es", "Xocavənd"), ("fa", "شهرستان خواجه\u{200c}وند"), ("fi", "Xocavəndin piirikunta"), ("fr", "Khojavend"), ("he", "חוג׳אבנד"), ("hu", "Xocavəndi járás"), ("id", "Khojavend"), ("it", "distretto di Xocavənd"), ("ja", "ホジャヴェンド県"), ("ko", "호자벤트 구"), ("mk", "Хоџавенд"), ("ms", "Khojavend"), ("nb", "Xocavənd"), ("nl", "Xocavənd"), ("no", "Xocavənd"), ("pl", "Rejon Xocavənd"), ("pt", "Khojavend"), ("ru", "Ходжавендский район"), ("sq", "Khojavend"), ("sr", "Хоџавендски рејон"), ("sr_Latn", "Hodžavendski rejon"), ("sv", "Chodzjavänd"), ("tr", "Hocavend Rayonu"), ("vi", "Khojavend"), ("zh", "霍賈文德區")]),
                        unofficial_name_list: ["Xocavänd"].to_vec(),
                    }
                ),
                (
                    "YAR",
                    Subdivision{
                        name: "YAR",
                        country_alpha2: Alpha2::AZ,
                        code: "YAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.9058917), longitude: Some(48.24961270000001), max_latitude: Some(38.9156798), min_latitude: Some(38.886824), max_longitude: Some(48.2865858), min_longitude: Some(48.2148314)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يارديملي"), ("az", "Yardımlı"), ("bn", "ইয\u{9bc}\u{9be}র\u{9cd}ডিমলি জেল\u{9be}"), ("ccp", "𑄃\u{11128}𑄠𑄢\u{11134}𑄓\u{11128}𑄟\u{11134}𑄣\u{11128}"), ("ceb", "Yardymli Rayon"), ("cs", "Jardymli"), ("da", "Yardymli District"), ("de", "Yardımlı"), ("el", "Γιαρντιμλί"), ("en", "Yardymli"), ("es", "Yardımlı"), ("fa", "شهرستان یاردیملی"), ("fi", "Yardımlın piirikunta"), ("fr", "Yardımlı"), ("gu", "યાર\u{acd}દિમલી જિલ\u{acd}લો"), ("hi", "यार\u{94d}दिमली जिला"), ("id", "Yardymli"), ("it", "distretto di Yardımlı"), ("ja", "ヤルディムリ県"), ("kn", "ಯಾರ\u{ccd}ಡ\u{cbf}ಲ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "야르듬르 구"), ("lt", "Jardimlio apskritis"), ("lv", "Jardimli rajons"), ("mk", "Јардимли"), ("mr", "यार\u{94d}डिमली जिल\u{94d}हा"), ("ms", "Yardymli"), ("nb", "Yardımlı"), ("nl", "Yardımlı"), ("no", "Yardımlı"), ("pl", "Rejon Yardımlı"), ("pt", "Yardymli"), ("ru", "Ярдымлинский район"), ("si", "ය\u{dcf}ර\u{dca}ඩ\u{dd2}ම\u{dca}ල\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Yardymli"), ("sr", "Јардимлински рејон"), ("sr_Latn", "Jardimlinski rejon"), ("sv", "Jardymly"), ("ta", "ய\u{bbe}ர\u{bcd}ட\u{bcd}யம\u{bcd}லி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "య\u{c3e}ర\u{c4d}డ\u{c3f}మ\u{c4d}ల\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ยาด\u{e35}ไมล\u{e4c}"), ("tr", "Yardımlı Rayonu"), ("uk", "Ярдимлинський район"), ("ur", "یارڈیملی ضلع"), ("vi", "Yardymly"), ("zh", "亞爾德姆雷區")]),
                        unofficial_name_list: ["Yardimli"].to_vec(),
                    }
                ),
                (
                    "YE",
                    Subdivision{
                        name: "YE",
                        country_alpha2: Alpha2::AZ,
                        code: "YE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.617222), longitude: Some(47.15), max_latitude: Some(40.6331705), min_latitude: Some(40.5830618), max_longitude: Some(47.1835326), min_longitude: Some(47.1098042)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Yevlax"), ("be", "Еўлах"), ("ccp", "𑄃\u{11128}𑄠𑄬𑄛\u{11134}𑄣𑄇\u{11134}"), ("ceb", "Yevlakh"), ("cs", "Jevlach"), ("de", "Yevlax"), ("en", "Yevlakh"), ("es", "Yevlax"), ("fa", "یولاخ"), ("fi", "Yevlax"), ("fr", "Ievlakh"), ("he", "ייבלאח"), ("hu", "Yevlakh"), ("hy", "Եվլախ"), ("id", "Yevlakh"), ("it", "Yevlax"), ("ja", "イェヴラフ"), ("ko", "예블라흐"), ("lt", "Jevlachas"), ("mn", "Евлах"), ("ms", "Yevlakh"), ("nl", "Yevlax"), ("pl", "Yevlax"), ("ro", "Ievlah"), ("ru", "Евлах"), ("sl", "Jevlah"), ("sr", "Јевлах"), ("sr_Latn", "Jevlah"), ("sv", "Jevlach"), ("tr", "Yevlah"), ("uk", "Євлах"), ("ur", "یولاخ"), ("zh", "葉夫拉赫")]),
                        unofficial_name_list: ["Yevlax City"].to_vec(),
                    }
                ),
                (
                    "YEV",
                    Subdivision{
                        name: "YEV",
                        country_alpha2: Alpha2::AZ,
                        code: "YEV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.617222), longitude: Some(47.15), max_latitude: Some(40.6331705), min_latitude: Some(40.5830618), max_longitude: Some(47.1835326), min_longitude: Some(47.1098042)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يفلاخ"), ("az", "Yevlax²"), ("be", "Еўлахскі раён"), ("bn", "ইয\u{9bc}েভ\u{9cd}ল\u{9be}ক জেল\u{9be}"), ("ccp", "𑄃\u{11128}𑄠𑄬𑄛\u{11134}𑄣𑄇\u{11134} 𑄎𑄬𑄣"), ("ceb", "Yevlakh Rayon"), ("da", "Yevlax"), ("de", "Yevlax²"), ("el", "Γιεβλάκ"), ("en", "Yevlakh District"), ("es", "Yevlax²"), ("fa", "شهرستان یولاخ"), ("fi", "Yevlaxin piirikunta"), ("fr", "Yevlax"), ("gu", "ય\u{ac7}વલાખ જિલ\u{acd}લો"), ("hi", "यवलाख जिला"), ("id", "Yevlakh²"), ("it", "distretto di Yevlax"), ("ja", "イェヴラフ県"), ("kn", "ಯ\u{cc6}ವ\u{ccd}ಲಾಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "예블라흐 구"), ("lt", "Jevlako apskritis"), ("lv", "Jevlahas rajons"), ("mk", "Јевлах"), ("mr", "य\u{947}वलाख जिल\u{94d}हा"), ("ms", "Yevlakh²"), ("nb", "Yevlax"), ("nl", "Yevlax²"), ("no", "Yevlax"), ("pl", "Rejon Yevlax"), ("pt", "Yevlakh"), ("ru", "Евлахский район"), ("si", "යෙව\u{dca}ලක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Yevlakh"), ("sr", "Јевлашки рејон"), ("sr_Latn", "Jevlaški rejon"), ("sv", "Yevlakh (distrikt)"), ("ta", "எவ\u{bcd}லக\u{bcd}ஹ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "య\u{c46}వ\u{c4d}ల\u{c3e}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเยวลาค\u{e4c}"), ("tr", "Yevlah Rayonu"), ("uk", "Євлахський район"), ("ur", "ییولاخ ضلع"), ("vi", "Yevlakh"), ("zh", "葉夫拉赫區")]),
                        unofficial_name_list: ["Yevlax"].to_vec(),
                    }
                ),
                (
                    "ZAN",
                    Subdivision{
                        name: "ZAN",
                        country_alpha2: Alpha2::AZ,
                        code: "ZAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.0318935), longitude: Some(46.6265379), max_latitude: Some(39.224588), min_latitude: Some(38.872734), max_longitude: Some(46.8760261), min_longitude: Some(46.43833900000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زنغلان"), ("az", "Zəngilan"), ("bn", "জ\u{9be}ংগিল\u{9be}ন জেল\u{9be}"), ("ccp", "𑄎𑄋\u{11134}𑄉\u{11128}𑄣𑄚\u{11134}"), ("ceb", "Zangilan Rayon"), ("da", "Zəngilan"), ("de", "Zəngilan (Rayon)"), ("el", "Ζάνγκιλαν"), ("en", "Zangilan"), ("es", "Zəngilan (raión)"), ("fa", "شهرستان زنگلان"), ("fi", "Zəngilanin piirikunta"), ("fr", "Zangilan (raion)"), ("gu", "ઝા\u{a82}ગીલાન જિલ\u{acd}લો"), ("hi", "ज\u{93c}न\u{94d}गिलन जिला"), ("hu", "Zəngilani járás"), ("id", "Zangilan"), ("it", "distretto di Zəngilan"), ("ja", "ザンギラン県"), ("kn", "ಜಂಗ\u{cbf}ಲಾನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "젠길란 구"), ("lt", "Zangelano sritis"), ("lv", "Zengilanas rajons"), ("mk", "Зенгилан (округ)"), ("mr", "झा\u{902}गिलन जिल\u{94d}हा"), ("ms", "Daerah Zangilan"), ("nb", "Zəngilan (distrikt)"), ("nl", "Zəngilan"), ("no", "Zəngilan (distrikt)"), ("pl", "Rejon Zəngilan"), ("pt", "Zangilan"), ("ru", "Зангеланский район"), ("si", "සැන\u{dca}ග\u{dd2}ලන\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Zangilan (rajon)"), ("sr", "Зангелански рејон"), ("sr_Latn", "Zangelanski rejon"), ("sv", "Zängilan"), ("ta", "ச\u{bbe}ங\u{bcd}கிலன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జ\u{c3e}ంగ\u{c3f}ల\u{c3e}న\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตแซงก\u{e34}แลน"), ("tr", "Zengilan Rayonu"), ("uk", "Зангеланський район"), ("ur", "زنگلان ضلع"), ("vi", "Zangilan (quận)"), ("zh", "贊格蘭區")]),
                        unofficial_name_list: ["Zängilan"].to_vec(),
                    }
                ),
                (
                    "ZAQ",
                    Subdivision{
                        name: "ZAQ",
                        country_alpha2: Alpha2::AZ,
                        code: "ZAQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.63361099999999), longitude: Some(46.643333), max_latitude: Some(41.6469735), min_latitude: Some(41.5850192), max_longitude: Some(46.6757584), min_longitude: Some(46.6077804)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زقاتالا"), ("az", "Zaqatala"), ("bn", "জ\u{9be}ক\u{9be}ত\u{9be}ল\u{9be} জেল\u{9be}"), ("ccp", "𑄎𑄇𑄑𑄣\u{11134}"), ("ceb", "Zaqatala Rayon"), ("da", "Zaqatala District"), ("de", "Zaqatala"), ("el", "Ζακατάλα"), ("en", "Zaqatala"), ("es", "Zaqatala (raión)"), ("fa", "شهرستان زاقاتالا"), ("fi", "Zaqatalan piirikunta"), ("fr", "Zaqatala (raion)"), ("gu", "ઝાકાતાલા જિલ\u{acd}લો"), ("hi", "ज\u{93c}ाकाटला जिला"), ("hu", "Zaqatalai járás"), ("hy", "Զաքաթալայի շրջան"), ("id", "Zaqatala (rayon)"), ("it", "distretto di Zaqatala"), ("ja", "ザガタラ県"), ("ka", "ზაქათალის რაიონი"), ("kn", "ಜಕಾತಲಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "자가탈라 구"), ("lt", "Zakatala apskritis"), ("lv", "Zagatalas rajons"), ("mk", "Закатала (округ)"), ("mr", "जकातला जिल\u{94d}हा"), ("ms", "Zaqatala (rayon)"), ("nb", "Zaqatala (distrikt)"), ("nl", "Zaqatala"), ("no", "Zaqatala (distrikt)"), ("pl", "Rejon Zaqatala"), ("pt", "Zaqatala"), ("ru", "Закатальский район"), ("si", "සකටල\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Zaqatala (rajon)"), ("sr", "Закатаљски рејон"), ("sr_Latn", "Zakataljski rejon"), ("sv", "Zaqatala (distrikt)"), ("ta", "ச\u{bbe}கிய\u{bbe}தல\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జక\u{c3e}టల\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาดาคาลา"), ("tr", "Zakatala Rayonu"), ("uk", "Загатальський район"), ("ur", "زاقاتالا ضلع"), ("uz", "Zaqatala tumani"), ("vi", "Zagatala (quận)"), ("zh", "扎卡塔雷區")]),
                        unofficial_name_list: ["Zaqatala"].to_vec(),
                    }
                ),
                (
                    "ZAR",
                    Subdivision{
                        name: "ZAR",
                        country_alpha2: Alpha2::AZ,
                        code: "ZAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.218333), longitude: Some(47.708333), max_latitude: Some(40.2448614), min_latitude: Some(40.2012645), max_longitude: Some(47.73121829999999), min_longitude: Some(47.6942681)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Rayon,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زارداب"), ("az", "Zərdab"), ("bn", "জ\u{9be}রড\u{9be}ব জেল\u{9be}"), ("ccp", "𑄎𑄢\u{11134}𑄓𑄛\u{11134}"), ("ceb", "Zardab Rayon"), ("da", "Zardab District"), ("de", "Zərdab (Rayon)"), ("el", "Ζάρνταμπ"), ("en", "Zardab"), ("es", "Zərdab (raión)"), ("fa", "شهرستان زردآب"), ("fi", "Zərdabin piirikunta"), ("fr", "Zərdab (raion)"), ("gu", "ઝર\u{acd}દાબ જિલ\u{acd}લો"), ("hi", "ज\u{93c}ारदब जिला"), ("id", "Zardab"), ("it", "distretto di Zərdab"), ("ja", "ザルダブ県"), ("kn", "ಜರ\u{ccd}ದಾಬ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "제르다프 구"), ("lt", "Zardabo sritis"), ("lv", "Zerdabas rajons"), ("mk", "Зердаб"), ("mr", "झारदबा जिल\u{94d}हा"), ("ms", "Zardab"), ("nb", "Zərdab (distrikt)"), ("nl", "Zərdab"), ("no", "Zərdab (distrikt)"), ("pl", "Rejon Zərdab"), ("pt", "Zardab"), ("ru", "Зардобский район"), ("si", "සර\u{dca}ද\u{dcf}බ\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sq", "Zardab (rajon)"), ("sr", "Зердапски рејон"), ("sr_Latn", "Zerdapski rejon"), ("sv", "Zärdab"), ("ta", "ச\u{bbe}ர\u{bcd}த\u{bbe}ப\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జర\u{c4d}ద\u{c3e}బ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาร\u{e4c}แดบ"), ("tr", "Zerdab Rayonu"), ("uk", "Зердабський район"), ("ur", "زارداب ضلع"), ("vi", "Zerdab (quận)"), ("zh", "扎爾多布區")]),
                        unofficial_name_list: ["Zärdab"].to_vec(),
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
#[cfg(feature = "az")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AZ,
        alpha3: Alpha3::AZE,
        address_format: None,
        continent: Continent::Asia,
        country_code: 994,
        currency_code: CurrencyCode::AZN,
        gec: Some(GEC::AJ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::AZE),
        iso_long_name: "The Republic of Azerbaijan",
        iso_short_name: "Azerbaijan",
        official_language_list: ["az", "hy"].to_vec(),
        spoken_language_list: ["az", "hy"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "8",
        nationality: Some("Azerbaijani"),
        number: "031",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "AZ",
        unofficial_name_list: [
            "Azerbaijan",
            "Aserbaidschan",
            "Azerbaïdjan",
            "Azerbaiyán",
            "アゼルバイジャン",
            "Azerbeidzjan",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Azerbaijan"),
            ("af", "Azerbaidjan"),
            ("ak", "Azerbaijan"),
            ("am", "ጐፈሴባጃን"),
            ("an", "Azerbaichán"),
            ("ar", "أذربيجان"),
            ("as", "আজেৰ\u{9cd}বেইজ\u{9be}ন"),
            ("ay", "Azerbaijan"),
            ("az", "Azərbaycan"),
            ("ba", "Azerbaijan"),
            ("be", "Азербайджан"),
            ("bg", "Азербайджан"),
            ("bi", "Azerbaijan"),
            ("bn", "আজ\u{9be}রব\u{9be}ইজ\u{9be}ন"),
            ("bn_IN", "আজ\u{9be}রব\u{9be}ইজ\u{9be}ন"),
            ("br", "Azerbaidjan"),
            ("bs", "Azerbejdžan"),
            ("ca", "Azerbaitjan"),
            ("ce", "Азербайджан"),
            ("ch", "Azerbaijan"),
            ("cs", "Ázerbájdžán"),
            ("cv", "Азербайджан"),
            ("cy", "Azerbaijan"),
            ("da", "Aserbajdsjan"),
            ("de", "Aserbaidschan"),
            (
                "dv",
                "އ\u{7a6}ޒ\u{7a6}ރ\u{7aa}ބ\u{7a6}އ\u{7a8}ޖ\u{7a7}ނ\u{7b0}",
            ),
            ("dz", "ཨ་ཛར་བ་ཡ\u{f7a}་ཇ\u{f71}ན།"),
            ("ee", "Azerbaijan"),
            ("el", "Αζερμπαϊτζάν"),
            ("en", "Azerbaijan"),
            ("eo", "Azerbajĝano"),
            ("es", "Azerbaiyán"),
            ("et", "Aserbaidžaan"),
            ("eu", "Azerbaijan"),
            ("fa", "آذربایجان"),
            ("ff", "Aserbayjan"),
            ("fi", "Azerbaidžan"),
            ("fo", "Aserbajdsjan"),
            ("fr", "Azerbaïdjan"),
            ("fy", "Azerbeidzjan"),
            ("ga", "An Asarbaiseáin"),
            ("gl", "Acerbaixán"),
            ("gn", "Azerbaijan"),
            ("gu", "અઝરબ\u{ac8}ઝાન"),
            ("gv", "Yn Asserbajaan"),
            ("ha", "Azerbaijan"),
            ("he", "אזרבייג׳ן"),
            ("hi", "अज\u{93c}रब\u{948}जान"),
            ("hr", "Azerbajdžan"),
            ("ht", "Azerbaydjan"),
            ("hu", "Azerbajdzsán"),
            ("hy", "Ադրբեջան"),
            ("ia", "Azerbaijan"),
            ("id", "Azerbaijan"),
            ("io", "Azerbaijan"),
            ("is", "Aserbaídsjan"),
            ("it", "Azerbaigian"),
            ("iu", "Azerbaijan"),
            ("ja", "アゼルバイジャン"),
            ("ka", "აზერბაიჯანი"),
            ("ki", "Azerbaijan"),
            ("kk", "Әзірбайжан"),
            ("kl", "Azerbaijan"),
            ("km", "អាហ\u{17d2}ស\u{17ca}ែរបែហ\u{17d2}សង\u{17cb}"),
            ("kn", "ಅಜರ\u{ccd}ಬೈಜಾನ\u{ccd}"),
            ("ko", "아제르바이잔"),
            ("ku", "Azerbaycan"),
            ("kv", "Азербайджан"),
            ("kw", "Azerbayjan"),
            ("ky", "Азербайжан"),
            ("lo", "ປະເທດອາແຊກບາຍຊ\u{eb1}ງ"),
            ("lt", "Azerbaidžanas"),
            ("lv", "Azerbaidžāna"),
            ("mi", "Atepaihānia"),
            ("mk", "Азербејџан"),
            ("ml", "അസര\u{d4d}\u{200d}ബൈജ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Азербайжан"),
            ("mr", "अझ\u{947}र\u{94d}ब\u{947}जान"),
            ("ms", "Azerbaijan"),
            ("mt", "Ażerbajġan"),
            (
                "my",
                "အဇာဘ\u{102d}\u{102f}င\u{103a}ဂျန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Aderbaidjan"),
            ("nb", "Aserbajdsjan"),
            ("ne", "अजरब\u{948}जान"),
            ("nl", "Azerbeidzjan"),
            ("nn", "Aserbajdsjan"),
            ("nv", "Azerbaijan"),
            ("oc", "Azerbaitjan"),
            ("or", "ଆଜର\u{b4d}ବେଜ\u{b3e}ନ"),
            ("pa", "ਅਜ\u{a3c}ਰਬਾਈਜਾਨ"),
            ("pi", "अजर\u{94d}ब\u{948}जान"),
            ("pl", "Azerbejdżan"),
            ("ps", "آزربایجان"),
            ("pt", "Azerbaijão"),
            ("pt_BR", "Azerbaidjão"),
            ("ro", "Azerbaijan"),
            ("ru", "Азербайджан"),
            ("rw", "Azeribayijani"),
            ("sc", "Azerbaigiàn"),
            ("sd", "Azerbaijan"),
            ("si", "අසර\u{dca}බය\u{dd2}ජ\u{dcf}න\u{dca}"),
            ("sk", "Azerbajdžan"),
            ("sl", "Azerbajdžan"),
            ("so", "Aserbiijaan"),
            ("sq", "Azerbajxhan"),
            ("sr", "Азербејџан"),
            ("sv", "Azerbajdzjan"),
            ("sw", "Azerbaijan"),
            ("ta", "அசர\u{bcd}பைச\u{bbe}ன\u{bcd}"),
            ("te", "అఝ\u{c47}ర\u{c4d}బ\u{c47}జ\u{c3e}న"),
            ("tg", "Озарбойҷон"),
            ("th", "อาเซอร\u{e4c}ไบจาน"),
            ("ti", "ኣዘርባጃን"),
            ("tk", "Azerbeýjan"),
            ("tl", "Azerbaijan"),
            ("tr", "Azerbaycan"),
            ("tt", "Әзәрбайҗан"),
            ("ug", "ئەزەربەيجان"),
            ("uk", "Азербайджан"),
            ("ur", "آذربائیجان"),
            ("uz", "Ozarbayjon"),
            ("ve", "Azerbaijan"),
            ("vi", "Ai-xợ-bai-gianh"),
            ("wa", "Azerbaydjan"),
            ("wo", "Aserbayjaan"),
            ("xh", "Azerbaijan"),
            ("yo", "Azerbaijan"),
            ("zh_CN", "阿塞拜疆"),
            ("zh_HK", "亞塞拜彊"),
            ("zh_TW", "亞塞拜然"),
            ("zu", "I-Azerbayijani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
