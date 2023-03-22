// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Belize

#[cfg(all(feature = "bz", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BZ;
    pub const ALPHA3: Alpha3 = Alpha3::BLZ;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 501;
    pub const CURRENCY_CODE: &str = "BZD";
    pub const GEC: Option<GEC> = Some(GEC::BH);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("BIZ");
    pub const ISO_SHORT_NAME: &str = "Belize";
    pub const ISO_LONG_NAME: &str = "Belize";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Belizean");
    pub const NUMBER: &str = "084";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "BZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Belize", "Belice", "ベリーズ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Belize"),
        ("af", "Belize"),
        ("ak", "Belize"),
        ("am", "ቤሑፈ"),
        ("an", "Belize"),
        ("ar", "بيليز"),
        ("as", "বেলিজ"),
        ("ay", "Belize"),
        ("az", "Beliz"),
        ("ba", "Belize"),
        ("be", "Беліз"),
        ("bg", "Белиз"),
        ("bi", "Belize"),
        ("bn", "বেলিজে"),
        ("bn_IN", "বেলিজে"),
        ("br", "Belize"),
        ("bs", "Belize"),
        ("ca", "Belize"),
        ("ce", "Белиз"),
        ("ch", "Belize"),
        ("cs", "Belize"),
        ("cv", "Белиз"),
        ("cy", "Belîs"),
        ("da", "Belize"),
        ("de", "Belize"),
        ("dv", "ބ\u{7ac}ލ\u{7a9}ޒ\u{7aa}"),
        ("dz", "བ\u{f7a}་ལ\u{f72}ཛ\u{f72}།"),
        ("ee", "Belize"),
        ("el", "Μπελίζ"),
        ("en", "Belize"),
        ("eo", "Belizo"),
        ("es", "Belice"),
        ("et", "Belize"),
        ("eu", "Belize"),
        ("fa", "بلیز"),
        ("ff", "Belize"),
        ("fi", "Belize"),
        ("fo", "Belis"),
        ("fr", "Belize"),
        ("fy", "Belize"),
        ("ga", "An Bheilís"),
        ("gl", "Belice"),
        ("gn", "Belize"),
        ("gu", "બ\u{ac7}લિઝ"),
        ("gv", "Yn Veleesh"),
        ("ha", "Belize"),
        ("he", "בליז"),
        ("hi", "ब\u{947}लीज\u{93c}"),
        ("hr", "Belize"),
        ("ht", "Beliz"),
        ("hu", "Belize"),
        ("hy", "Բելիզ"),
        ("ia", "Belize"),
        ("id", "Belize"),
        ("io", "Belize"),
        ("is", "Belís"),
        ("it", "Belize"),
        ("iu", "Belize"),
        ("ja", "ベリーズ"),
        ("ka", "ბელიზი"),
        ("ki", "Belize"),
        ("kk", "Белиз"),
        ("kl", "Belize"),
        ("km", "បេល\u{17b8}ហ\u{17d2}ស"),
        ("kn", "ಬ\u{cc6}ಲ\u{cbf}ಜ\u{cbf}"),
        ("ko", "벨리즈"),
        ("ku", "Belîze"),
        ("kv", "Belize"),
        ("kw", "Belisa"),
        ("ky", "Белиз"),
        ("lo", "Belize"),
        ("lt", "Belizas"),
        ("lv", "Beliza"),
        ("mi", "Belize"),
        ("mk", "Белизе"),
        ("ml", "ബെലൈസ\u{d4d}"),
        ("mn", "Белиз"),
        ("mr", "ब\u{947}लिझ"),
        ("ms", "Belize"),
        ("mt", "Beliże"),
        (
            "my",
            "ဘလ\u{102d}ဇ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Berij"),
        ("nb", "Belize"),
        ("ne", "ब\u{947}लिज"),
        ("nl", "Belize"),
        ("nn", "Belize"),
        ("nv", "Belize"),
        ("oc", "Belize"),
        ("or", "ବେଲ\u{b3f}ଜ"),
        ("pa", "ਬੀਲੀਜ\u{a3c}ੀ"),
        ("pi", "ब\u{947}लीज"),
        ("pl", "Belize"),
        ("ps", "بېلیز"),
        ("pt", "Belize"),
        ("pt_BR", "Belize"),
        ("ro", "Belize"),
        ("ru", "Белиз"),
        ("rw", "Belize"),
        ("sc", "Belize"),
        ("sd", "Belize"),
        ("si", "බ\u{dd2}ල\u{dd2}සේ"),
        ("sk", "Belize"),
        ("sl", "Belize"),
        ("so", "Belise"),
        ("sq", "Belize"),
        ("sr", "Белизе"),
        ("sv", "Belize"),
        ("sw", "Belize"),
        ("ta", "பெலிஸ\u{bcd}"),
        ("te", "బ\u{c47}ల\u{c3f}ఝ"),
        ("tg", "Белиз"),
        ("th", "เบล\u{e35}ซ"),
        ("ti", "ቤሊዝ"),
        ("tk", "Beliz"),
        ("tl", "Belize"),
        ("tr", "Belize"),
        ("tt", "Белиз"),
        ("ug", "بېلىز"),
        ("uk", "Беліз"),
        ("ur", "بیلیز"),
        ("uz", "Beliz"),
        ("ve", "Belize"),
        ("vi", "Bê-li-xê"),
        ("wa", "Belize"),
        ("wo", "Beliis"),
        ("xh", "Belize"),
        ("yo", "Bẹ\u{300}lísè"),
        ("zh_CN", "伯利兹"),
        ("zh_HK", "伯利兹"),
        ("zh_TW", "貝里斯"),
        ("zu", "Belize"),
    ];
    #[cfg(all(feature = "bz", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 17.189877;
        pub const LONGITUDE: f64 = -88.49765;
        pub const MAX_LATITUDE: f64 = 18.4959419;
        pub const MAX_LONGITUDE: f64 = -87.41269989999999;
        pub const MIN_LATITUDE: f64 = 15.8856189;
        pub const MIN_LONGITUDE: f64 = -89.22758789999999;
        pub const NORTHEAST_LATITUDE: f64 = 18.4959419;
        pub const NORTHEAST_LONGITUDE: f64 = -87.41269989999999;
        pub const SOUTHWEST_LATITUDE: f64 = 15.8856189;
        pub const SOUTHWEST_LONGITUDE: f64 = -89.22758789999999;
    }
}
#[cfg(all(feature = "bz", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 17.189877,
            longitude: -88.49765,
            max_latitude: 18.4959419,
            max_longitude: -87.41269989999999,
            min_latitude: 15.8856189,
            min_longitude: -89.22758789999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 18.4959419,
                    longitude: -87.41269989999999,
                },
                southwest: CountryGeoBound {
                    latitude: 15.8856189,
                    longitude: -89.22758789999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "bz", feature = "subdivisions"))]
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
                    "BZ",
                    Subdivision{
                        name: "BZ",
                        country_alpha2: Alpha2::BZ,
                        code: "BZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.5045661), longitude: Some(-88.1962133), max_latitude: Some(17.5353682), min_latitude: Some(17.4757965), max_longitude: Some(-88.18077389999999), min_longitude: Some(-88.2457925)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "акруга Беліз"), ("bg", "Белиз"), ("ca", "Districte de Belize"), ("ccp", "𑄝𑄬𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Belize District"), ("da", "Belize"), ("de", "Belize District"), ("en", "Belize"), ("es", "Distrito de Belice"), ("fa", "ناحیه بلیز"), ("fi", "Belize"), ("fr", "District de Belize"), ("gl", "Distrito de Belice"), ("hr", "Belize"), ("id", "Distrik Belize"), ("it", "distretto di Belize"), ("ja", "ベリーズ州"), ("ko", "벨리즈 구"), ("lt", "Belizo rajonas"), ("lv", "Belizas distrikts"), ("mr", "ब\u{947}लीझ जिल\u{94d}हा"), ("nb", "Belize"), ("nl", "Belize"), ("no", "Belize"), ("pl", "Dystrykt Belize"), ("pt", "Belize"), ("ru", "Белиз"), ("sv", "Belize"), ("uk", "Беліз"), ("ur", "بیلیز ضلع"), ("zh", "伯利兹区")]),
                        unofficial_name_list: ["Belize"].to_vec(),
                    }
                ),
                (
                    "CY",
                    Subdivision{
                        name: "CY",
                        country_alpha2: Alpha2::BZ,
                        code: "CY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.0984445), longitude: Some(-88.94138650000001), max_latitude: Some(17.5050549), min_latitude: Some(16.4052541), max_longitude: Some(-88.55081790000001), min_longitude: Some(-89.189365)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كايو"), ("bg", "Кайо"), ("bn", "ক\u{9be}য\u{9bc}ো জেল\u{9be}"), ("ca", "Districte de Cayo"), ("ccp", "𑄇\u{11133}𑄠𑄬𑄃\u{1112e}"), ("ceb", "Cayo District"), ("da", "Cayo"), ("de", "Cayo District"), ("el", "Κάγιο"), ("en", "Cayo"), ("es", "Distrito de Cayo"), ("fi", "Cayon kaupunginosa"), ("fr", "District de Cayo"), ("gu", "ક\u{ac7}યો જિલ\u{acd}લો"), ("hi", "कायो जिला"), ("hr", "Cayo (okrug)"), ("id", "Distrik Cayo"), ("it", "distretto di Cayo"), ("ja", "カヨ州"), ("jv", "Distrik Cayo"), ("kn", "ಕಯೋ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "카요 구"), ("lt", "Kajo rajonas"), ("lv", "Kajo distrikts"), ("mr", "काओ जिल\u{94d}हा"), ("ms", "Cayo District"), ("nb", "Cayo"), ("nl", "Cayo"), ("no", "Cayo"), ("pl", "Dystrykt Cayo"), ("pt", "Cayo"), ("ru", "Кайо"), ("si", "ක\u{dcf}යෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Cayo"), ("ta", "க\u{bbe}யோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}య\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "คาโย"), ("tr", "Cayo District"), ("uk", "Район Кайо"), ("ur", "کایو ضلع"), ("vi", "Quận Cayo"), ("zh", "卡約區")]),
                        unofficial_name_list: ["Cayo"].to_vec(),
                    }
                ),
                (
                    "CZL",
                    Subdivision{
                        name: "CZL",
                        country_alpha2: Alpha2::BZ,
                        code: "CZL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.1349238), longitude: Some(-88.24611829999999), max_latitude: Some(18.496557), min_latitude: Some(17.8932279), max_longitude: Some(-87.849937), min_longitude: Some(-88.59995289999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوروزال"), ("bg", "Коросал"), ("bn", "কোর\u{9c1}জ\u{9be}ল জেল\u{9be}"), ("ca", "Districte de Corozal"), ("ccp", "𑄇\u{11127}𑄢\u{1112e}𑄎𑄣\u{11134}"), ("ceb", "Corozal District"), ("da", "Corozal"), ("de", "Corozal District"), ("el", "Κορόζαλ"), ("en", "Corozal"), ("es", "Distrito de Corozal"), ("fa", "ناحیه کوروزال"), ("fi", "Corozalin kaupunginosa"), ("fr", "District de Corozal"), ("gu", "કોરોજલ જિલ\u{acd}લો"), ("hi", "कोरोज\u{93c}ल जिला"), ("hr", "Corozal (okrug)"), ("id", "Distrik Corozal"), ("it", "distretto di Corozal"), ("ja", "コロザル州"), ("kn", "ಕೊರೊಜಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "코로살 구"), ("lt", "Korosalio rajonas"), ("lv", "Korosalas distrikts"), ("mr", "कोरोजल जिल\u{94d}हा"), ("ms", "Corozal District"), ("nb", "Corozal"), ("nl", "Corozal"), ("no", "Corozal"), ("pl", "Dystrykt Corozal"), ("pt", "Corozal"), ("ru", "Коросаль"), ("si", "කොරෝසල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Corozal"), ("ta", "கோரோசல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c4b}ర\u{c4b}జ\u{c3e}ల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "โคโรซาล"), ("tr", "Corozal District"), ("uk", "Район Коросаль"), ("ur", "کوروزال ضلع"), ("vi", "Quận Corozal"), ("zh", "科羅薩爾區")]),
                        unofficial_name_list: ["Corozal"].to_vec(),
                    }
                ),
                (
                    "OW",
                    Subdivision{
                        name: "OW",
                        country_alpha2: Alpha2::BZ,
                        code: "OW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.760353), longitude: Some(-88.86469799999999), max_latitude: Some(18.2496241), min_latitude: Some(17.3420751), max_longitude: Some(-88.28445599999999), min_longitude: Some(-89.146957)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة اورانج ووك"), ("bg", "Ориндж Уолк"), ("bn", "ওরেঞ\u{9cd}জ ওয\u{9bc}\u{9be}ক জেল\u{9be}"), ("ca", "Districte d’Orange Walk"), ("ccp", "𑄃\u{11127}𑄢𑄬𑄚\u{11133}𑄎\u{11134} 𑄃\u{1112e}𑄠𑄇\u{11134}"), ("ceb", "Orange Walk District"), ("da", "Orange Walk"), ("de", "Orange Walk District"), ("el", "Όραντζ Γουόλκ"), ("en", "Orange Walk"), ("es", "Distrito de Orange Walk"), ("fa", "ناحیه آرنج واک"), ("fi", "Orange Walkin kaupunginosa"), ("fr", "District d’Orange Walk"), ("gu", "ઓર\u{ac7}ન\u{acd}જ વોક જિલ\u{acd}લો"), ("hi", "ऑर\u{947}\u{902}ज वाक जिला"), ("hr", "Orange Walk (okrug)"), ("id", "Distrik Orange Walk"), ("it", "distretto di Orange Walk"), ("ja", "オレンジウォーク州"), ("kn", "ಆರ\u{cc6}ಂಜ\u{ccd} ವಾಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "오렌지워크 구"), ("lt", "Orendž Volko rajonas"), ("lv", "Orindžvokas distrikts"), ("mr", "ऑर\u{947}\u{902}ज वॉल\u{94d}क जिल\u{94d}हा"), ("ms", "Orange Walk District"), ("nb", "Orange Walk"), ("nl", "Orange Walk"), ("no", "Orange Walk"), ("pl", "Dystrykt Orange Walk"), ("pt", "Orange Walk"), ("ru", "Ориндж-Уолк"), ("si", "ඔරේන\u{dca}ජ\u{dca} වෝක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Orange Walk"), ("ta", "ஆரஞ\u{bcd}சு வ\u{bbe}க\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఆర\u{c46}ంజ\u{c4d} వ\u{c3e}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องออเรนจ\u{e4c} วอล\u{e4c}ค"), ("tr", "Orange Walk District"), ("uk", "Район Ориндж-Волк"), ("ur", "اورنج واک ضلع"), ("vi", "Quận Orange Walk"), ("zh", "橘園區")]),
                        unofficial_name_list: ["Orange Walk"].to_vec(),
                    }
                ),
                (
                    "SC",
                    Subdivision{
                        name: "SC",
                        country_alpha2: Alpha2::BZ,
                        code: "SC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.8116631), longitude: Some(-88.4016041), max_latitude: Some(17.1271911), min_latitude: Some(16.5113349), max_longitude: Some(-88.0971759), min_longitude: Some(-88.774852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ستان كريك"), ("bg", "Стан Крийк"), ("bn", "স\u{9cd}ট\u{9be}ন জেল\u{9be}"), ("ca", "Districte de Stann Creek"), ("ccp", "𑄌\u{11133}𑄑𑄚\u{11134} 𑄇\u{11133}𑄢\u{11128}𑄇\u{11134}"), ("ceb", "Stann Creek District"), ("da", "Stann Creek"), ("de", "Stann Creek District"), ("el", "Σταν Κρικ"), ("en", "Stann Creek"), ("es", "Distrito de Stann Creek"), ("fa", "ناحیه استان کریک"), ("fi", "Stann Creekin kaupunginosa"), ("fr", "District de Stann Creek"), ("gu", "સ\u{acd}ટ\u{ac7}ન ક\u{acd}રીક જિલ\u{acd}લો"), ("hi", "स\u{94d}ट\u{947}न क\u{94d}रीक जिला"), ("hr", "Stann Creek (okrug)"), ("id", "Distrik Stann Creek"), ("it", "distretto di Stann Creek"), ("ja", "スタンクリーク州"), ("kn", "ಸ\u{ccd}ಟ\u{ccd}ಯಾನ\u{ccd}ಕ\u{ccd}ರೀಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "스탠크리크 구"), ("lt", "Stan Kryko rajonas"), ("lv", "Stankrīkas distrikts"), ("mr", "स\u{94d}ट\u{945}नन क\u{94d}रीक जिल\u{94d}हा"), ("ms", "Stann Creek District"), ("nb", "Stann Creek"), ("nl", "Stann Creek"), ("no", "Stann Creek"), ("pl", "Dystrykt Stann Creek"), ("pt", "Stann Creek"), ("ru", "Станн-Крик"), ("si", "ස\u{dca}ට\u{dcf}න\u{dca} ක\u{dca}\u{200d}ර\u{dd3}ක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Stann Creek"), ("ta", "ஸ\u{bcd}ட\u{bbe}ண\u{bcd} க\u{bcd}ர\u{bc0}க\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c4d}ట\u{c3e}న\u{c4d} క\u{c4d}ర\u{c40}క\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "สต\u{e31}นน\u{e4c}คร\u{e35}ก"), ("tr", "Stann Creek District"), ("uk", "Стан-Крик"), ("ur", "ستان کریک ضلع"), ("vi", "Quận Stann Creek"), ("zh", "斯坦克里克區")]),
                        unofficial_name_list: ["Stann Creek"].to_vec(),
                    }
                ),
                (
                    "TOL",
                    Subdivision{
                        name: "TOL",
                        country_alpha2: Alpha2::BZ,
                        code: "TOL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.2491923), longitude: Some(-88.86469799999999), max_latitude: Some(16.7002059), min_latitude: Some(15.889429), max_longitude: Some(-88.14487510000001), min_longitude: Some(-89.2248151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة توليدو"), ("bg", "Толедо"), ("bn", "তলেড\u{9c1} জেল\u{9be}"), ("ca", "Districte de Toledo"), ("ccp", "𑄑\u{11127}𑄣𑄬𑄓\u{1112e}"), ("ceb", "Toledo District"), ("da", "Toledo"), ("de", "Toledo"), ("el", "Τολέδο"), ("en", "Toledo"), ("es", "Distrito de Toledo"), ("fa", "ناحیه تولیدو"), ("fi", "Toledonin kaupunginosa"), ("fr", "District de Toledo"), ("gl", "Distrito de Toledo"), ("gu", "ટોલ\u{ac7}ડો જિલ\u{acd}લો"), ("hi", "टोलिडो जिला"), ("hr", "Toledo (okrug)"), ("id", "Distrik Toledo"), ("it", "distretto di Toledo"), ("ja", "トレド州"), ("kn", "ಟೋಲ\u{cc6}ಡೊ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "털리도 구"), ("lt", "Toledo rajonas"), ("lv", "Toledo distrikts"), ("mr", "टोल\u{947}डो जिल\u{94d}हा"), ("ms", "Toledo District"), ("nb", "Toledo"), ("nl", "Toledo"), ("no", "Toledo"), ("pl", "Dystrykt Toledo"), ("pt", "Toledo"), ("ru", "Толедо"), ("si", "ටොලේඩෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Toledo"), ("ta", "டோலிடோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c4b}ల\u{c46}డ\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อำเภอโตเลโด"), ("tr", "Toledo District"), ("uk", "Толедо"), ("ur", "ٹولڈو ضلع"), ("vi", "Quận Toledo"), ("zh", "托萊多區")]),
                        unofficial_name_list: ["Toledo"].to_vec(),
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
#[cfg(feature = "bz")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BZ,
        alpha3: Alpha3::BLZ,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 501,
        currency_code: "BZD",
        gec: Some(GEC::BH),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("BIZ"),
        iso_long_name: "Belize",
        iso_short_name: "Belize",
        official_language_list: ["en", "es"].to_vec(),
        spoken_language_list: ["en", "es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "0",
        nationality: Some("Belizean"),
        number: "084",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "BZ",
        unofficial_name_list: ["Belize", "Belice", "ベリーズ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Belize"),
            ("af", "Belize"),
            ("ak", "Belize"),
            ("am", "ቤሑፈ"),
            ("an", "Belize"),
            ("ar", "بيليز"),
            ("as", "বেলিজ"),
            ("ay", "Belize"),
            ("az", "Beliz"),
            ("ba", "Belize"),
            ("be", "Беліз"),
            ("bg", "Белиз"),
            ("bi", "Belize"),
            ("bn", "বেলিজে"),
            ("bn_IN", "বেলিজে"),
            ("br", "Belize"),
            ("bs", "Belize"),
            ("ca", "Belize"),
            ("ce", "Белиз"),
            ("ch", "Belize"),
            ("cs", "Belize"),
            ("cv", "Белиз"),
            ("cy", "Belîs"),
            ("da", "Belize"),
            ("de", "Belize"),
            ("dv", "ބ\u{7ac}ލ\u{7a9}ޒ\u{7aa}"),
            ("dz", "བ\u{f7a}་ལ\u{f72}ཛ\u{f72}།"),
            ("ee", "Belize"),
            ("el", "Μπελίζ"),
            ("en", "Belize"),
            ("eo", "Belizo"),
            ("es", "Belice"),
            ("et", "Belize"),
            ("eu", "Belize"),
            ("fa", "بلیز"),
            ("ff", "Belize"),
            ("fi", "Belize"),
            ("fo", "Belis"),
            ("fr", "Belize"),
            ("fy", "Belize"),
            ("ga", "An Bheilís"),
            ("gl", "Belice"),
            ("gn", "Belize"),
            ("gu", "બ\u{ac7}લિઝ"),
            ("gv", "Yn Veleesh"),
            ("ha", "Belize"),
            ("he", "בליז"),
            ("hi", "ब\u{947}लीज\u{93c}"),
            ("hr", "Belize"),
            ("ht", "Beliz"),
            ("hu", "Belize"),
            ("hy", "Բելիզ"),
            ("ia", "Belize"),
            ("id", "Belize"),
            ("io", "Belize"),
            ("is", "Belís"),
            ("it", "Belize"),
            ("iu", "Belize"),
            ("ja", "ベリーズ"),
            ("ka", "ბელიზი"),
            ("ki", "Belize"),
            ("kk", "Белиз"),
            ("kl", "Belize"),
            ("km", "បេល\u{17b8}ហ\u{17d2}ស"),
            ("kn", "ಬ\u{cc6}ಲ\u{cbf}ಜ\u{cbf}"),
            ("ko", "벨리즈"),
            ("ku", "Belîze"),
            ("kv", "Belize"),
            ("kw", "Belisa"),
            ("ky", "Белиз"),
            ("lo", "Belize"),
            ("lt", "Belizas"),
            ("lv", "Beliza"),
            ("mi", "Belize"),
            ("mk", "Белизе"),
            ("ml", "ബെലൈസ\u{d4d}"),
            ("mn", "Белиз"),
            ("mr", "ब\u{947}लिझ"),
            ("ms", "Belize"),
            ("mt", "Beliże"),
            (
                "my",
                "ဘလ\u{102d}ဇ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Berij"),
            ("nb", "Belize"),
            ("ne", "ब\u{947}लिज"),
            ("nl", "Belize"),
            ("nn", "Belize"),
            ("nv", "Belize"),
            ("oc", "Belize"),
            ("or", "ବେଲ\u{b3f}ଜ"),
            ("pa", "ਬੀਲੀਜ\u{a3c}ੀ"),
            ("pi", "ब\u{947}लीज"),
            ("pl", "Belize"),
            ("ps", "بېلیز"),
            ("pt", "Belize"),
            ("pt_BR", "Belize"),
            ("ro", "Belize"),
            ("ru", "Белиз"),
            ("rw", "Belize"),
            ("sc", "Belize"),
            ("sd", "Belize"),
            ("si", "බ\u{dd2}ල\u{dd2}සේ"),
            ("sk", "Belize"),
            ("sl", "Belize"),
            ("so", "Belise"),
            ("sq", "Belize"),
            ("sr", "Белизе"),
            ("sv", "Belize"),
            ("sw", "Belize"),
            ("ta", "பெலிஸ\u{bcd}"),
            ("te", "బ\u{c47}ల\u{c3f}ఝ"),
            ("tg", "Белиз"),
            ("th", "เบล\u{e35}ซ"),
            ("ti", "ቤሊዝ"),
            ("tk", "Beliz"),
            ("tl", "Belize"),
            ("tr", "Belize"),
            ("tt", "Белиз"),
            ("ug", "بېلىز"),
            ("uk", "Беліз"),
            ("ur", "بیلیز"),
            ("uz", "Beliz"),
            ("ve", "Belize"),
            ("vi", "Bê-li-xê"),
            ("wa", "Belize"),
            ("wo", "Beliis"),
            ("xh", "Belize"),
            ("yo", "Bẹ\u{300}lísè"),
            ("zh_CN", "伯利兹"),
            ("zh_HK", "伯利兹"),
            ("zh_TW", "貝里斯"),
            ("zu", "Belize"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
