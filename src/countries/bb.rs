// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Barbados

#[cfg(all(feature = "bb", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BB;
    pub const ALPHA3: Alpha3 = Alpha3::BRB;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: &str = "BBD";
    pub const GEC: Option<GEC> = Some(GEC::BB);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::BAR);
    pub const ISO_SHORT_NAME: &str = "Barbados";
    pub const ISO_LONG_NAME: &str = "Barbados";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Barbadian");
    pub const NUMBER: &str = "052";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("BB\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "BB";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Barbade", "Barbados", "バルバドス"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Barbados"),
        ("af", "Barbados"),
        ("ak", "Barbados"),
        ("am", "ባሴቤ፦ስ"),
        ("an", "Barbados"),
        ("ar", "بربادوس"),
        ("as", "ব\u{9be}ৰ\u{9cd}ব\u{9be}ডোছ"),
        ("ay", "Barbados"),
        ("az", "Barbados"),
        ("ba", "Barbados"),
        ("be", "Барбадас"),
        ("bg", "Барбадос"),
        ("bi", "Barbados"),
        ("bn", "ব\u{9be}র\u{9cd}ব\u{9be}ডোস"),
        ("bn_IN", "ব\u{9be}র\u{9cd}ব\u{9be}ডোস"),
        ("br", "Barbados"),
        ("bs", "Barbados"),
        ("ca", "Barbados"),
        ("ce", "Барбадос"),
        ("ch", "Barbados"),
        ("cs", "Barbados"),
        ("cv", "Барбадос"),
        ("cy", "Barbados"),
        ("da", "Barbados"),
        ("de", "Barbados"),
        ("dv", "ބ\u{7a7}ބ\u{7a6}ޑ\u{7ae}ސ\u{7b0}"),
        ("dz", "བར་བ་ཌ\u{f7c}ས\u{f72}།"),
        ("ee", "Barbados"),
        ("el", "Μπαρμπάντος"),
        ("en", "Barbados"),
        ("eo", "Barbado"),
        ("es", "Barbados"),
        ("et", "Barbados"),
        ("eu", "Barbados"),
        ("fa", "باربادوس"),
        ("ff", "Barbados"),
        ("fi", "Barbados"),
        ("fo", "Barbados"),
        ("fr", "Barbade"),
        ("fy", "Barbados"),
        ("ga", "Barbadós"),
        ("gl", "Barbados"),
        ("gn", "Barbados"),
        ("gu", "બાર\u{acd}બાડોસ"),
        ("gv", "Barbados"),
        ("ha", "Barbados"),
        ("he", "ברבדוס"),
        ("hi", "बारबाडोस"),
        ("hr", "Barbados"),
        ("ht", "Lababad"),
        ("hu", "Barbados"),
        ("hy", "Բարբադոս"),
        ("ia", "Barbados"),
        ("id", "Barbados"),
        ("io", "Barbados"),
        ("is", "Barbados"),
        ("it", "Barbados"),
        ("iu", "Barbados"),
        ("ja", "バルバドス"),
        ("ka", "ბარბადოსი"),
        ("ki", "Barbados"),
        ("kk", "Барбадос"),
        ("kl", "Barbados"),
        ("km", "បារបាដ\u{17bc}ស"),
        ("kn", "ಬಾರ\u{ccd}ಬಡಾಸ\u{ccd}"),
        ("ko", "바베이도스"),
        ("ku", "Barbados"),
        ("kv", "Барбадос"),
        ("kw", "Barbados"),
        ("ky", "Барбадос"),
        ("lo", "Barbados"),
        ("lt", "Barbadosas"),
        ("lv", "Barbadosa"),
        ("mi", "Barbados"),
        ("mk", "Барбадос"),
        ("ml", "ബ\u{d3e}ര\u{d4d}\u{200d}ബഡോസ\u{d4d}"),
        ("mn", "Барбадос"),
        ("mr", "बार\u{94d}ब\u{947}डॉस"),
        ("ms", "Barbados"),
        ("mt", "Barbados"),
        (
            "my",
            "ဘာဘေးဒ\u{102d}\u{102f}းစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Barbadot"),
        ("nb", "Barbados"),
        ("ne", "बारबाडोस"),
        ("nl", "Barbados"),
        ("nn", "Barbados"),
        ("nv", "Barbados"),
        ("oc", "Barbada"),
        ("or", "ବ\u{b3e}ର\u{b4d}ବ\u{b3e}ଡୋସ"),
        ("pa", "ਬਾਰਬਾਡਾਸ"),
        ("pi", "बार\u{94d}बाडोस"),
        ("pl", "Barbados"),
        ("ps", "باربادوس"),
        ("pt", "Barbados"),
        ("pt_BR", "Barbados"),
        ("ro", "Barbados"),
        ("ru", "Барбадос"),
        ("rw", "Barubadosi"),
        ("sc", "Barbados"),
        ("sd", "Barbados"),
        ("si", "බ\u{dcf}බඩෝස\u{dca}"),
        ("sk", "Barbados"),
        ("sl", "Barbados"),
        ("so", "Baarbadoos"),
        ("sq", "Barbados"),
        ("sr", "Барбадос"),
        ("sv", "Barbados"),
        ("sw", "Barbados"),
        ("ta", "ப\u{bbe}ர\u{bcd}பட\u{bbe}ஸ\u{bcd}"),
        ("te", "బ\u{c3e}ర\u{c4d}బ\u{c47}డ\u{c3e}స\u{c4d}"),
        ("tg", "Барбадос"),
        ("th", "บาร\u{e4c}เบโดส"),
        ("ti", "ባርቤዶስ"),
        ("tk", "Barbados"),
        ("tl", "Barbados"),
        ("tr", "Barbados"),
        ("tt", "Барбадос"),
        ("ug", "باربادوس"),
        ("uk", "Барбадос"),
        ("ur", "بارباڈوس"),
        ("uz", "Barbados"),
        ("ve", "Barbados"),
        ("vi", "Bă-ba-đôxợ"),
        ("wa", "Bårbades"),
        ("wo", "Barbados"),
        ("xh", "Barbados"),
        ("yo", "Bárbádọ\u{300}s"),
        ("zh_CN", "巴巴多斯"),
        ("zh_HK", "巴巴多斯"),
        ("zh_TW", "巴貝多"),
        ("zu", "I-Barbados"),
    ];
    #[cfg(all(feature = "bb", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 13.193887;
        pub const LONGITUDE: f64 = -59.543198;
        pub const MAX_LATITUDE: f64 = 13.3365093;
        pub const MAX_LONGITUDE: f64 = -59.4174957;
        pub const MIN_LATITUDE: f64 = 13.039844;
        pub const MIN_LONGITUDE: f64 = -59.6530151;
        pub const NORTHEAST_LATITUDE: f64 = 13.3365093;
        pub const NORTHEAST_LONGITUDE: f64 = -59.4174957;
        pub const SOUTHWEST_LATITUDE: f64 = 13.039844;
        pub const SOUTHWEST_LONGITUDE: f64 = -59.6530151;
    }
}
#[cfg(all(feature = "bb", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 13.193887,
            longitude: -59.543198,
            max_latitude: 13.3365093,
            max_longitude: -59.4174957,
            min_latitude: 13.039844,
            min_longitude: -59.6530151,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 13.3365093,
                    longitude: -59.4174957,
                },
                southwest: CountryGeoBound {
                    latitude: 13.039844,
                    longitude: -59.6530151,
                },
            },
        }
    }
}

#[cfg(all(feature = "bb", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::BB,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.083735), longitude: Some(-59.52967419999999), max_latitude: Some(13.125974), min_latitude: Some(13.0449995), max_longitude: Some(-59.4719928), min_longitude: Some(-59.6110459)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كنيسة المسيح"), ("bn", "\"খ\u{9cd}রীষ\u{9cd}ট চ\u{9be}র\u{9cd}চ"), ("ca", "Christ Church"), ("ccp", "𑄇\u{11133}𑄢\u{11128}𑄥\u{11133}𑄑\u{11134} 𑄌𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Christ Church"), ("da", "Christ Church"), ("de", "Christchurch"), ("el", "Κράιστ Τσερτς"), ("en", "Christ Church"), ("es", "Parroquia de Christ Church"), ("fa", "کرایست چرچ، باربادوس"), ("fi", "Christ Church"), ("fr", "Christ Church"), ("gu", "ક\u{acd}રિસ\u{acd}ટ ચર\u{acd}ચ"), ("hi", "क\u{94d}राइस\u{94d}ट चर\u{94d}च"), ("id", "Christ Church"), ("it", "Christ Church"), ("ja", "クライスト・チャーチ (バルバドス)"), ("kn", "ಕ\u{ccd}ರೈಸ\u{ccd}ಟ\u{ccd} ಚರ\u{ccd}ಚ\u{ccd}"), ("ko", "크라이스트처치 교구"), ("lt", "Kristaus bažnyčios parapija"), ("lv", "Kraistčērčas pagasts"), ("mr", "क\u{94d}राइस\u{94d}ट चर\u{94d}च"), ("ms", "Kariah Christ Church"), ("nb", "Christ Church prestegjeld"), ("nl", "Christ Church"), ("no", "Christ Church prestegjeld"), ("pl", "Christ Church"), ("pt", "Christ Church"), ("ro", "Christ Church"), ("ru", "Крайстчерч"), ("si", "ක\u{dca}\u{200d}රය\u{dd2}ස\u{dca}ට\u{dca}චර\u{dca}ච\u{dca}"), ("sv", "Christ Church"), ("ta", "கிறிஸ\u{bcd}ட\u{bcd} சர\u{bcd}ச\u{bcd}"), ("te", "క\u{c4d}ర\u{c48}స\u{c4d}ట\u{c4d} చర\u{c4d}చ\u{c3f}"), ("th", "คร\u{e34}ส คร\u{e31}ซ"), ("tr", "Christ Kilisesi"), ("uk", "Крайст-Черч"), ("ur", "کرائسٹ چرچ، بارباڈوس"), ("vi", "Christ Church"), ("zh", "基督城教區")]),
                        unofficial_name_list: ["Christ Church"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::BB,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.2540083), longitude: Some(-59.5757599), max_latitude: Some(13.293989), min_latitude: Some(13.204939), max_longitude: Some(-59.5420721), min_longitude: Some(-59.60091989999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت أندرو"), ("bn", "সেন\u{9cd}ট অ\u{9cd}য\u{9be}ন\u{9cd}ড\u{9cd}র\u{9c1}"), ("ca", "Saint Andrew"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄃𑄬𑄚\u{11134}𑄓\u{11133}𑄢\u{1112a}"), ("ceb", "Saint Andrew"), ("da", "Saint Andrew"), ("de", "Saint Andrew"), ("el", "Άγιος Ανδρέας"), ("en", "Saint Andrew"), ("es", "Saint Andrew"), ("fa", "سنت آندرو، باربادوس"), ("fi", "Saint Andrew"), ("fr", "Saint Andrew"), ("gu", "સ\u{ac7}ન\u{acd}ટ એન\u{acd}ડ\u{acd}ર\u{ac1}"), ("he", "סנט אנדרו"), ("hi", "स\u{947}\u{902}ट ए\u{902}ड\u{94d}र\u{942}"), ("id", "Saint Andrew"), ("it", "Saint Andrew"), ("ja", "セント・アンドリュー (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಆಂಡ\u{ccd}ರ\u{ccd}ಯ\u{cc2}"), ("ko", "세인트앤드루 교구"), ("lt", "Šv. Andriejaus parapija"), ("lv", "Sentendrū pagasts"), ("mr", "स\u{947}\u{902}ट ऍन\u{94d}ड\u{94d}र\u{94d}य\u{942}"), ("ms", "Saint Andrew"), ("nb", "Saint Andrew prestegjeld"), ("nl", "Saint Andrew"), ("no", "Saint Andrew prestegjeld"), ("pl", "Saint Andrew"), ("pt", "Saint Andrew"), ("ro", "Saint Andrew"), ("ru", "Сент-Эндрю"), ("si", "ශ\u{dcf}න\u{dca}ත ඇන\u{dca}ඩ\u{dca}ර\u{dd6}"), ("sv", "Saint Andrew"), ("ta", "செயின\u{bcd}ட\u{bcd} அன\u{bcd}றெவ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఆండ\u{c4d}య\u{c4d}రూ"), ("th", "เซน แอนดร\u{e34}ว"), ("tr", "Saint Andrew"), ("uk", "Парафія Сент-Андрю"), ("ur", "سینٹ اینڈریو، بارباڈوس"), ("vi", "Saint Andrew"), ("zh", "聖安德魯區")]),
                        unofficial_name_list: ["Saint Andrew"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::BB,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.148292), longitude: Some(-59.55271499999999), max_latitude: Some(13.1815211), min_latitude: Some(13.099258), max_longitude: Some(-59.50622000000001), min_longitude: Some(-59.5823159)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت جورج، بربادوس"), ("bn", "সেন\u{9cd}ট জর\u{9cd}জ"), ("ca", "Saint George"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎\u{11127}𑄢\u{11133}𑄎\u{11134}"), ("ceb", "Saint George"), ("da", "Saint George"), ("de", "Saint George"), ("el", "Άγιος Γεώργιος"), ("en", "Saint George"), ("es", "Parroquia de Saint George"), ("fa", "سنت جورج، باربادوس"), ("fi", "Saint George"), ("fr", "Saint George"), ("gu", "સ\u{ac7}ન\u{acd}ટ જ\u{acd}યોર\u{acd}જ"), ("hi", "स\u{947}\u{902}ट जॉर\u{94d}ज"), ("id", "Saint George"), ("it", "Saint George"), ("ja", "セント・ジョージ (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಜಾರ\u{ccd}ಜ\u{ccd}"), ("ko", "세인트조지 교구"), ("lt", "Šv. Jurgio parapija"), ("lv", "Sentdžordža pagasts"), ("mk", "Сент Џорџ"), ("mr", "स\u{947}\u{902}ट जॉर\u{94d}ज"), ("ms", "Saint George"), ("nb", "Saint George prestegjeld"), ("nl", "Saint George"), ("no", "Saint George prestegjeld"), ("pl", "Saint George"), ("pt", "Saint George"), ("ro", "Saint George"), ("ru", "Приход Сент-Джордж"), ("si", "ශ\u{dcf}න\u{dca}ත ජෝජ\u{dca}"), ("sv", "Saint George"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜ\u{bbe}ர\u{bcd}ஜ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c3e}ర\u{c4d}జ\u{c4d}"), ("th", "เซนต\u{e4c}จอร\u{e4c}จ"), ("tr", "Saint George"), ("uk", "Парафія Сент-Джордж"), ("ur", "سینٹ جارج، بارباڈوس"), ("vi", "Saint George"), ("zh", "聖喬治教區")]),
                        unofficial_name_list: ["Saint George"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::BB,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1939911), longitude: Some(-59.6276259), max_latitude: Some(13.237683), min_latitude: Some(13.1381931), max_longitude: Some(-59.598465), min_longitude: Some(-59.64361599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت جيمس، بربادوس"), ("bn", "স\u{9cd}টিেন\u{9cd}ট জেমস, ব\u{9be}র\u{9cd}ব\u{9be}ডোস"), ("ca", "Saint James"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎𑄬𑄟\u{11134}𑄥\u{11134}"), ("ceb", "Saint James (parokya)"), ("da", "Saint James, Barbados"), ("de", "Saint James, Barbados"), ("el", "Άγιος Ιάκωβος, Μπαρμπάντος"), ("en", "Saint James"), ("es", "Parroquia de Saint James"), ("fa", "سنت جیمز، باربادوس"), ("fi", "Saint James, Barbados"), ("fr", "Saint James"), ("gu", "સ\u{ac7}ન\u{acd}ટ જ\u{ac7}મ\u{acd}સ, બાર\u{acd}બાડોસ"), ("he", "סנט ג׳יימס, ברבדוס"), ("hi", "स\u{947}\u{902}ट ज\u{947}म\u{94d}स, बारबाडोस"), ("id", "Saint James, Barbados"), ("it", "Saint James"), ("ja", "セント・ジェームズ (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಜೇಮ\u{ccd}ಸ\u{ccd}, ಬಾರ\u{ccd}ಬಡೋಸ\u{ccd}"), ("ko", "세인트제임스 교구"), ("lt", "Šv. Jokūbo parapija"), ("lv", "Sentdžeimsa, Barbadosa"), ("mr", "स\u{947}\u{902}ट ज\u{947}म\u{94d}स, बारबाडोस"), ("ms", "Saint James, Barbados"), ("nb", "Saint James prestegjeld"), ("nl", "Saint James"), ("no", "Saint James prestegjeld"), ("pl", "Saint James"), ("pt", "Saint James"), ("ro", "Saint James, Barbados"), ("ru", "Сент-Джеймс"), ("si", "ශ\u{dcf}න\u{dca}ත ජේම\u{dca}ස\u{dca} බ\u{dcf}ර\u{dca}බඩෝස\u{dca}"), ("sv", "Saint James (parish)"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜேம\u{bcd}ஸ\u{bcd}, ப\u{bbe}ர\u{bcd}பட\u{bbe}ஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c47}మ\u{c4d}స\u{c4d}, బ\u{c3e}ర\u{c4d}బడ\u{c4b}స\u{c4d}"), ("th", "เซนต\u{e4c}เจมส\u{e4c},บาร\u{e4c}บาดอส"), ("tr", "Saint James, Barbados"), ("uk", "Парафія Сент-Джеймс"), ("ur", "سینٹ جیمز، بارباڈوس"), ("vi", "Saint James, Barbados"), ("zh", "聖詹姆斯教區")]),
                        unofficial_name_list: ["Saint James"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::BB,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1833369), longitude: Some(-59.5066376), max_latitude: Some(13.210594), min_latitude: Some(13.1376361), max_longitude: Some(-59.45831699999999), min_longitude: Some(-59.54018499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت جون، بربادوس"), ("bn", "সেন\u{9cd}ট জন"), ("ca", "Saint John"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎\u{11127}𑄚\u{11134}"), ("ceb", "Saint John"), ("da", "St. John"), ("de", "Saint John"), ("el", "Άγιος Ιωάννης"), ("en", "Saint John"), ("es", "Parroquia de Saint John"), ("fa", "سنت جان، باربادوس"), ("fi", "Saint John"), ("fr", "Saint John"), ("gu", "સ\u{ac7}ન\u{acd}ટ જ\u{acd}હોન"), ("hi", "स\u{947}\u{902}ट जॉन, बारबाडोस"), ("id", "Saint John"), ("it", "Saint John"), ("ja", "セント・ジョン (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಜಾನ\u{ccd}"), ("ko", "세인트존 교구"), ("lt", "Šv. Jono parapija"), ("lv", "Sendžona"), ("mk", "Сент Џон"), ("mr", "स\u{947}\u{902}ट जॉन"), ("ms", "Saint John"), ("nb", "Saint John"), ("nl", "Saint John"), ("no", "Saint John"), ("pl", "Saint John"), ("pt", "Saint John"), ("ro", "Saint John"), ("ru", "Сент-Джон"), ("si", "ශ\u{dcf}න\u{dca}ත ජෝන\u{dca}"), ("sv", "Saint John"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜ\u{bbe}ன\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c3e}న\u{c4d}"), ("th", "เซนต\u{e4c}จอห\u{e4c}น"), ("tr", "Saint John"), ("uk", "Парафія Сент-Джон"), ("ur", "سینٹ جان، بارباڈوس"), ("vi", "Saint John"), ("zh", "聖約翰教區")]),
                        unofficial_name_list: ["Saint John"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::BB,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.2040088), longitude: Some(-59.54695439999999), max_latitude: Some(13.235171), min_latitude: Some(13.169218), max_longitude: Some(-59.51017799999999), min_longitude: Some(-59.575138)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت جوزيف، بربادوس"), ("bn", "সেন\u{9cd}ট জোসেফ, ব\u{9be}র\u{9cd}ব\u{9be}ডোস"), ("ca", "Saint Joseph"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄎\u{11127}𑄥𑄬𑄛\u{11134}"), ("ceb", "Saint Joseph"), ("da", "Saint Joseph, Barbados"), ("de", "Saint Joseph, Barbados"), ("el", "Άγιος Ιωσήφ, Μπαρμπάντος"), ("en", "Saint Joseph"), ("es", "Parroquia de Saint Joseph"), ("fa", "سنت جوزف، باربادوس"), ("fi", "Saint Joseph, Barbados"), ("fr", "Saint Joseph"), ("gu", "સ\u{ac7}ન\u{acd}ટ જોસ\u{ac7}ફ, બાર\u{acd}બાડોસ"), ("hi", "स\u{947}\u{902}ट जोस\u{947}फ, बारबाडोस"), ("id", "Saint Joseph, Barbados"), ("it", "Saint Joseph"), ("ja", "セント・ジョセフ (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಜೋಸ\u{cc6}ಫ\u{ccd}, ಬಾರ\u{ccd}ಬಡೋಸ\u{ccd}"), ("ko", "세인트조지프 교구"), ("lt", "Šv. Juozapo parapija"), ("lv", "Sentdžozefa pagasts, Barbadosa"), ("mk", "Сент Џозеф"), ("mr", "स\u{947}\u{902}ट जोस\u{947}फ, बार\u{94d}बाडोस"), ("ms", "Saint Joseph, Barbados"), ("nb", "Saint Joseph prestegjeld"), ("nl", "Saint Joseph"), ("no", "Saint Joseph prestegjeld"), ("pl", "Saint Joseph"), ("pt", "Saint Joseph"), ("ro", "Saint Joseph, Barbados"), ("ru", "Сент-Джозеф"), ("si", "ශ\u{dcf}න\u{dca}ත ජෝසප\u{dca},බ\u{dcf}ර\u{dca}බඩෝස\u{dca}"), ("sv", "Saint Joseph (parish i Barbados)"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜோசப\u{bcd} , ப\u{bbe}ர\u{bcd}பட\u{bbe}ஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c4b}స\u{c46}ఫ\u{c4d}, బ\u{c3e}ర\u{c4d}బడ\u{c4b}స\u{c4d}"), ("th", "เซนต\u{e4c} โจเซฟ"), ("tr", "Sain Joseph, Barbados"), ("uk", "Парафія Сент-Джозеф"), ("ur", "سینٹ جوسف، بارباڈوس"), ("vi", "Saint Joseph, Barbados"), ("zh", "聖約瑟夫區")]),
                        unofficial_name_list: ["Saint Joseph"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::BB,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.3011778), longitude: Some(-59.621862), max_latitude: Some(13.3350319), min_latitude: Some(13.2740421), max_longitude: Some(-59.57436509999999), min_longitude: Some(-59.650559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت لوسي"), ("bn", "সেন\u{9cd}ট ল\u{9c1}সি"), ("ca", "Saint Lucy"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄣\u{1112a}𑄌\u{11128}"), ("ceb", "Saint Lucy"), ("da", "Saint Lucy"), ("de", "Saint Lucy"), ("el", "Αγία Λουκία"), ("en", "Saint Lucy"), ("es", "Parroquia de Saint Lucy"), ("fa", "سنت لوسی، باربادوس"), ("fi", "Saint Lucy"), ("fr", "Saint Lucy"), ("gu", "સ\u{ac7}ન\u{acd}ટ લ\u{acd}ય\u{ac1}સી"), ("hi", "स\u{947}\u{902}ट ल\u{941}सी"), ("id", "Saint Lucy"), ("it", "Saint Lucy"), ("ja", "セント・ルーシー (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಲ\u{cc2}ಸ\u{cbf}"), ("ko", "세인트루시 교구"), ("lt", "Šv. Liucijos parapija"), ("lv", "Sentlūsijas pagasts"), ("mr", "स\u{947}\u{902}ट ल\u{941}सी"), ("ms", "Saint Lucy"), ("nb", "Saint Lucy prestegjeld"), ("nl", "Saint Lucy"), ("no", "Saint Lucy prestegjeld"), ("pl", "Saint Lucy"), ("pt", "Saint Lucy"), ("ro", "Saint Lucy"), ("ru", "Сент-Люси"), ("si", "ශ\u{dcf}න\u{dca}ත ල\u{dd4}ස\u{dd3}"), ("sv", "Saint Lucy"), ("ta", "செயின\u{bcd}ட\u{bcd} லூசி"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} లూస\u{c40}"), ("th", "เซนต\u{e4c} ล\u{e39}ซ\u{e35}\u{e48}"), ("tr", "Saint Lucy"), ("uk", "Сент-Люсі"), ("ur", "سینٹ لسی، بارباڈوس"), ("vi", "Saint Lucy"), ("zh", "聖露西教區")]),
                        unofficial_name_list: ["Saint Lucy"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::BB,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1132219), longitude: Some(-59.59880889999999), max_latitude: Some(13.158466), min_latitude: Some(13.0784939), max_longitude: Some(-59.567387), min_longitude: Some(-59.63719399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت مايكل"), ("be", "Сен-Мішэль"), ("bn", "সেন\u{9cd}ট ম\u{9be}ইকেল"), ("ca", "Saint Michael"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄟\u{1112d}𑄇𑄬𑄣\u{11134}"), ("ceb", "Saint Michael (parokya)"), ("cs", "Saint Michael"), ("cy", "Saint Michael"), ("da", "Saint Michael"), ("de", "Saint Michael"), ("el", "Άγιος Μιχαήλ"), ("en", "Saint Michael"), ("es", "Parroquia de Saint Michael"), ("fa", "سنت مایکل، باربادوس"), ("fi", "Saint Michael"), ("fr", "Saint Michael"), ("gu", "સ\u{ac7}\u{a82}ટ માઈકલ"), ("he", "סיינט מייקל"), ("hi", "स\u{947}\u{902}ट माइकल"), ("hy", "Սեն Միշել"), ("id", "Saint Michael"), ("it", "Saint Michael"), ("ja", "セント・マイケル"), ("kn", "ಸೇಂಟ\u{ccd} ಮೈಕ\u{cc6}ಲ\u{ccd}"), ("ko", "세인트마이클 교구"), ("lt", "Šv. Mykolo parapija"), ("lv", "Sentmišelas pagasts"), ("mr", "स\u{947}\u{902}ट मायक\u{947}ल"), ("ms", "Saint Michael"), ("nb", "Saint Michael prestegjeld"), ("nl", "Saint Michael"), ("no", "Saint Michael prestegjeld"), ("pl", "Saint Michael"), ("pt", "Saint Michael"), ("ro", "Saint Michael"), ("ru", "Сен-Мишель"), ("si", "ශ\u{dcf}න\u{dca}ත මය\u{dd2}කල\u{dca}"), ("sl", "Saint Michael"), ("sq", "Saint Michael"), ("sv", "St. Michael"), ("ta", "செயின\u{bcd}ட\u{bcd} மைகேல\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} మ\u{c48}క\u{c47}ల\u{c4d}"), ("th", "เซนต\u{e4c}ไมเค\u{e34}ล"), ("tr", "Saint Michael"), ("uk", "Парафія Сент-Майкл (Сент-МІшель)"), ("ur", "سینٹ مائیکل، بارباڈوس"), ("vi", "Saint Michael"), ("zh", "聖邁克爾區")]),
                        unofficial_name_list: ["Saint Michael"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::BB,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.2600459), longitude: Some(-59.621862), max_latitude: Some(13.299063), min_latitude: Some(13.2257179), max_longitude: Some(-59.572906), min_longitude: Some(-59.6469631)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت بيتر، بربادوس"), ("be", "Сент-Пітэр"), ("bn", "সেইন\u{9cd}ট পিট\u{9be}র ,ব\u{9be}রব\u{9be}ডোজ"), ("ca", "Saint Peter"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄛\u{11128}𑄑𑄢\u{11134}"), ("ceb", "Saint Peter (parokya sa Barbados)"), ("da", "St. Peter, Barbados"), ("de", "Saint Peter, Barbados"), ("el", "Άγιος Πέτρος, Μπαρμπάντος"), ("en", "Saint Peter"), ("es", "Parroquia de Saint Peter"), ("fa", "سنت پیتر، باربادوس"), ("fi", "Saint Peter, Barbados"), ("fr", "Saint Peter"), ("gu", "સ\u{ac7}ન\u{acd}ટ પીટર, બાર\u{acd}બાડોસ"), ("hi", "स\u{947}\u{902}ट पीटर, बारबाडोस"), ("id", "Saint Peter, Barbados"), ("it", "Saint Peter"), ("ja", "セント・ペーター (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಪೀಟರ\u{ccd}, ಬಾರ\u{ccd}ಬಡೋಸ\u{ccd}"), ("ko", "세인트피터 교구"), ("lt", "Šv. Petro parapija"), ("lv", "Sentpītera, Barbadosa"), ("mr", "स\u{947}\u{902}ट पीटर, बार\u{94d}बाडोस"), ("ms", "Saint Peter, Barbados"), ("nb", "Saint Peter prestegjeld"), ("nl", "Saint Peter"), ("no", "Saint Peter prestegjeld"), ("pl", "Saint Peter"), ("pt", "Saint Peter"), ("ro", "Saint Peter, Barbados"), ("ru", "Сент-Питер"), ("si", "ශ\u{dcf}න\u{dca}ත ප\u{dd3}තර ,බ\u{dcf}ර\u{dca}බඩෝස\u{dca}"), ("sv", "Saint Peter (parish i Barbados)"), ("ta", "செயின\u{bcd}ட\u{bcd} ப\u{bc0}ட\u{bcd}டர\u{bcd} , ப\u{bbe}ர\u{bcd}படோஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ప\u{c40}టర\u{c4d}, బ\u{c3e}ర\u{c4d}బడ\u{c4b}స\u{c4d}"), ("th", "เซนต\u{e4c}ป\u{e35}เตอร\u{e4c}บาร\u{e4c}เบโดส"), ("tr", "Saint Peter, Barbados"), ("uk", "Сент-Пітер"), ("ur", "سینٹ پیٹر، بارباڈوس"), ("vi", "Saint Peter, Barbados"), ("zh", "聖彼得教區")]),
                        unofficial_name_list: ["Saint Peter"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::BB,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1362782), longitude: Some(-59.4605768), max_latitude: Some(13.1829601), min_latitude: Some(13.0789943), max_longitude: Some(-59.4210709), min_longitude: Some(-59.51082299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت فيليب، باربادوس"), ("bn", "সেন\u{9cd}ট ফিলিপ"), ("ca", "Saint Philip"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄜\u{11128}𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Saint Philip (parokya sa Barbados)"), ("da", "Saint Philip"), ("de", "Saint Philip"), ("el", "Άγιος Φίλιππος"), ("en", "Saint Philip"), ("es", "Parroquia de Saint Philip"), ("fa", "سنت فیلیپ، باربادوس"), ("fi", "Saint Philip"), ("fr", "Saint Philip"), ("gu", "સ\u{ac7}ન\u{acd}ટ ફિલિપ"), ("hi", "स\u{947}\u{902}ट फिलिप, बारबाडोस"), ("id", "Saint Philip"), ("it", "Saint Philip"), ("ja", "セント・フィリップ (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಫ\u{cbf}ಲ\u{cbf}ಪ\u{ccd}"), ("ko", "세인트필립 교구"), ("lt", "Šv. Pilypo parapija"), ("lv", "Sentfilipa pagasts"), ("mr", "स\u{947}\u{902}ट फिलिप"), ("ms", "Saint Philip"), ("nb", "Saint Philip"), ("nl", "Saint Philip"), ("no", "Saint Philip"), ("pl", "Saint Philip"), ("pt", "Saint Philip"), ("ro", "Saint Philip"), ("ru", "Сент-Филип"), ("si", "ශ\u{dcf}න\u{dca}ත ප\u{dd2}ල\u{dd2}ප\u{dca}"), ("sv", "Saint Philip"), ("ta", "செயின\u{bcd}ட\u{bcd} பிலிப\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఫ\u{c3f}ల\u{c3f}ప\u{c4d}"), ("th", "เซนต\u{e4c} ฟ\u{e34}ลล\u{e34}ป"), ("tr", "Sain Philip"), ("uk", "Сент-Філіп"), ("ur", "سینٹ فلپ، بارباڈوس"), ("vi", "Saint Philip"), ("zh", "聖菲利普區")]),
                        unofficial_name_list: ["Saint Philip"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::BB,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.1748291), longitude: Some(-59.59880889999999), max_latitude: Some(13.216692), min_latitude: Some(13.1466249), max_longitude: Some(-59.55319399999999), min_longitude: Some(-59.617723)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت توماس، بربادوس"), ("bn", "সেন\u{9cd}ট থম\u{9be}স"), ("ca", "Saint Thomas"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄗\u{11127}𑄟𑄌\u{11134}"), ("ceb", "Saint Thomas"), ("da", "Saint Thomas"), ("de", "Saint Thomas"), ("el", "Άγιος Θωμάς"), ("en", "Saint Thomas"), ("es", "Parroquia de Saint Thomas"), ("fa", "سنت توماس، باربادوس"), ("fi", "Saint Thomas"), ("fr", "Saint Thomas"), ("gu", "સ\u{ac7}ન\u{acd}ટ થોમસ"), ("hi", "स\u{947}\u{902}ट थॉमस, बारबाडोस"), ("id", "Saint Thomas"), ("it", "Saint Thomas"), ("ja", "セント・トーマス (バルバドス)"), ("kn", "ಸೇಂಟ\u{ccd} ಥಾಮಸ\u{ccd}"), ("ko", "세인트토머스 교구"), ("lt", "Šv. Tomo parapija"), ("lv", "Senttomasa"), ("mk", "Сент Томас"), ("mr", "स\u{947}\u{902}ट थॉमस"), ("ms", "Saint Thomas"), ("nb", "Saint Thomas prestegjeld"), ("nl", "Saint Thomas"), ("no", "Saint Thomas prestegjeld"), ("pl", "Saint Thomas"), ("pt", "Saint Thomas"), ("ro", "Saint Thomas"), ("ru", "Сент-Томас"), ("si", "ශ\u{dcf}න\u{dca}ත තෝමස\u{dca}"), ("sv", "Saint Thomas"), ("ta", "செயின\u{bcd}ட\u{bcd} த\u{bbe}மஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} థ\u{c3e}మస\u{c4d}"), ("th", "เซนต\u{e4c}โทม\u{e31}ส"), ("tr", "Saint Thomas"), ("uk", "Парафія Сент-Томас"), ("ur", "سینٹ تھامس، بارباڈوس"), ("vi", "Saint Thomas"), ("zh", "聖托馬斯區")]),
                        unofficial_name_list: ["Saint Thomas"].to_vec(),
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
#[cfg(feature = "bb")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BB,
        alpha3: Alpha3::BRB,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: "BBD",
        gec: Some(GEC::BB),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::BAR),
        iso_long_name: "Barbados",
        iso_short_name: "Barbados",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Barbadian"),
        number: "052",
        postal_code: true,
        postal_code_format: Some("BB\\d{5}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "BB",
        unofficial_name_list: ["Barbade", "Barbados", "バルバドス"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Barbados"),
            ("af", "Barbados"),
            ("ak", "Barbados"),
            ("am", "ባሴቤ፦ስ"),
            ("an", "Barbados"),
            ("ar", "بربادوس"),
            ("as", "ব\u{9be}ৰ\u{9cd}ব\u{9be}ডোছ"),
            ("ay", "Barbados"),
            ("az", "Barbados"),
            ("ba", "Barbados"),
            ("be", "Барбадас"),
            ("bg", "Барбадос"),
            ("bi", "Barbados"),
            ("bn", "ব\u{9be}র\u{9cd}ব\u{9be}ডোস"),
            ("bn_IN", "ব\u{9be}র\u{9cd}ব\u{9be}ডোস"),
            ("br", "Barbados"),
            ("bs", "Barbados"),
            ("ca", "Barbados"),
            ("ce", "Барбадос"),
            ("ch", "Barbados"),
            ("cs", "Barbados"),
            ("cv", "Барбадос"),
            ("cy", "Barbados"),
            ("da", "Barbados"),
            ("de", "Barbados"),
            ("dv", "ބ\u{7a7}ބ\u{7a6}ޑ\u{7ae}ސ\u{7b0}"),
            ("dz", "བར་བ་ཌ\u{f7c}ས\u{f72}།"),
            ("ee", "Barbados"),
            ("el", "Μπαρμπάντος"),
            ("en", "Barbados"),
            ("eo", "Barbado"),
            ("es", "Barbados"),
            ("et", "Barbados"),
            ("eu", "Barbados"),
            ("fa", "باربادوس"),
            ("ff", "Barbados"),
            ("fi", "Barbados"),
            ("fo", "Barbados"),
            ("fr", "Barbade"),
            ("fy", "Barbados"),
            ("ga", "Barbadós"),
            ("gl", "Barbados"),
            ("gn", "Barbados"),
            ("gu", "બાર\u{acd}બાડોસ"),
            ("gv", "Barbados"),
            ("ha", "Barbados"),
            ("he", "ברבדוס"),
            ("hi", "बारबाडोस"),
            ("hr", "Barbados"),
            ("ht", "Lababad"),
            ("hu", "Barbados"),
            ("hy", "Բարբադոս"),
            ("ia", "Barbados"),
            ("id", "Barbados"),
            ("io", "Barbados"),
            ("is", "Barbados"),
            ("it", "Barbados"),
            ("iu", "Barbados"),
            ("ja", "バルバドス"),
            ("ka", "ბარბადოსი"),
            ("ki", "Barbados"),
            ("kk", "Барбадос"),
            ("kl", "Barbados"),
            ("km", "បារបាដ\u{17bc}ស"),
            ("kn", "ಬಾರ\u{ccd}ಬಡಾಸ\u{ccd}"),
            ("ko", "바베이도스"),
            ("ku", "Barbados"),
            ("kv", "Барбадос"),
            ("kw", "Barbados"),
            ("ky", "Барбадос"),
            ("lo", "Barbados"),
            ("lt", "Barbadosas"),
            ("lv", "Barbadosa"),
            ("mi", "Barbados"),
            ("mk", "Барбадос"),
            ("ml", "ബ\u{d3e}ര\u{d4d}\u{200d}ബഡോസ\u{d4d}"),
            ("mn", "Барбадос"),
            ("mr", "बार\u{94d}ब\u{947}डॉस"),
            ("ms", "Barbados"),
            ("mt", "Barbados"),
            (
                "my",
                "ဘာဘေးဒ\u{102d}\u{102f}းစ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Barbadot"),
            ("nb", "Barbados"),
            ("ne", "बारबाडोस"),
            ("nl", "Barbados"),
            ("nn", "Barbados"),
            ("nv", "Barbados"),
            ("oc", "Barbada"),
            ("or", "ବ\u{b3e}ର\u{b4d}ବ\u{b3e}ଡୋସ"),
            ("pa", "ਬਾਰਬਾਡਾਸ"),
            ("pi", "बार\u{94d}बाडोस"),
            ("pl", "Barbados"),
            ("ps", "باربادوس"),
            ("pt", "Barbados"),
            ("pt_BR", "Barbados"),
            ("ro", "Barbados"),
            ("ru", "Барбадос"),
            ("rw", "Barubadosi"),
            ("sc", "Barbados"),
            ("sd", "Barbados"),
            ("si", "බ\u{dcf}බඩෝස\u{dca}"),
            ("sk", "Barbados"),
            ("sl", "Barbados"),
            ("so", "Baarbadoos"),
            ("sq", "Barbados"),
            ("sr", "Барбадос"),
            ("sv", "Barbados"),
            ("sw", "Barbados"),
            ("ta", "ப\u{bbe}ர\u{bcd}பட\u{bbe}ஸ\u{bcd}"),
            ("te", "బ\u{c3e}ర\u{c4d}బ\u{c47}డ\u{c3e}స\u{c4d}"),
            ("tg", "Барбадос"),
            ("th", "บาร\u{e4c}เบโดส"),
            ("ti", "ባርቤዶስ"),
            ("tk", "Barbados"),
            ("tl", "Barbados"),
            ("tr", "Barbados"),
            ("tt", "Барбадос"),
            ("ug", "باربادوس"),
            ("uk", "Барбадос"),
            ("ur", "بارباڈوس"),
            ("uz", "Barbados"),
            ("ve", "Barbados"),
            ("vi", "Bă-ba-đôxợ"),
            ("wa", "Bårbades"),
            ("wo", "Barbados"),
            ("xh", "Barbados"),
            ("yo", "Bárbádọ\u{300}s"),
            ("zh_CN", "巴巴多斯"),
            ("zh_HK", "巴巴多斯"),
            ("zh_TW", "巴貝多"),
            ("zu", "I-Barbados"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
