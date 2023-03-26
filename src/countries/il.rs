// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The State of Israel

#[cfg(all(feature = "il", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::IL;
    pub const ALPHA3: Alpha3 = Alpha3::ISR;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 972;
    pub const CURRENCY_CODE: &str = "ILS";
    pub const GEC: Option<GEC> = Some(GEC::IS);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ISR);
    pub const ISO_SHORT_NAME: &str = "Israel";
    pub const ISO_LONG_NAME: &str = "The State of Israel";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar", "he"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar", "he"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Israeli");
    pub const NUMBER: &str = "376";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}(?:\\d{2})?");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "IL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Israel", "Israël", "イスラエル"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Israel"),
        ("af", "Israel"),
        ("ak", "Israel"),
        ("am", "ጕስሲጔሔ"),
        ("an", "Israel"),
        ("ar", "إسرائيل"),
        ("as", "ইজ\u{9cd}ৰ\u{9be}ইল"),
        ("ay", "Israel"),
        ("az", "İsrail"),
        ("ba", "Israel"),
        ("be", "Ізраіль"),
        ("bg", "Израел"),
        ("bi", "Israel"),
        ("bn", "ইজর\u{9be}ইল"),
        ("bn_IN", "ইজর\u{9be}ইল"),
        ("br", "Israel"),
        ("bs", "Izrael"),
        ("ca", "Israel"),
        ("ce", "Израиль"),
        ("ch", "Israel"),
        ("cs", "Izrael"),
        ("cv", "Израиль"),
        ("cy", "Israel"),
        ("da", "Israel"),
        ("de", "Israel"),
        ("dv", "އ\u{7a8}ސ\u{7b0}ރ\u{7a7}އ\u{7a9}ލ\u{7aa}"),
        ("dz", "ཨ\u{f72}ཛ\u{f72}་ར\u{f7a}ལ།"),
        ("ee", "Israel"),
        ("el", "Ισραήλ"),
        ("en", "Israel"),
        ("eo", "Israelo"),
        ("es", "Israel"),
        ("et", "Iisrael"),
        ("eu", "Israel"),
        ("fa", "اسراییل"),
        ("ff", "Israel"),
        ("fi", "Israel"),
        ("fo", "Ísrael"),
        ("fr", "Israël"),
        ("fy", "Israel"),
        ("ga", "Iosrael"),
        ("gl", "Israel"),
        ("gn", "Israel"),
        ("gu", "ઇઝરાય\u{ac7}લ"),
        ("gv", "Israel"),
        ("ha", "Isra'ila"),
        ("he", "ישראל"),
        ("hi", "इज\u{93c}राइल"),
        ("hr", "Izrael"),
        ("ht", "Izrayèl"),
        ("hu", "Izrael"),
        ("hy", "Իսրայել"),
        ("ia", "Israel"),
        ("id", "Israel"),
        ("io", "Israel"),
        ("is", "Ísrael"),
        ("it", "Israele"),
        ("iu", "Israel"),
        ("ja", "イスラエル"),
        ("ka", "ისრაელი"),
        ("ki", "Israel"),
        ("kk", "Израиль"),
        ("kl", "Israel"),
        ("km", "អ\u{17ca}\u{17b8}ស\u{17d2}រាអែល"),
        ("kn", "ಇಸ\u{ccd}ರೇಲ\u{ccd}"),
        ("ko", "이스라엘"),
        ("ku", "Îsraîl"),
        ("kv", "Израиль"),
        ("kw", "Ysrael"),
        ("ky", "Израиль"),
        ("lo", "Israel"),
        ("lt", "Izraelis"),
        ("lv", "Izraēla"),
        ("mi", "Iharaira"),
        ("mk", "Израел"),
        ("ml", "ഇസ\u{d4d}ര\u{d3e}യേല\u{d4d}\u{200d}"),
        ("mn", "Израйл"),
        ("mr", "इस\u{94d}रायल"),
        ("ms", "Israel"),
        ("mt", "Iżrael"),
        ("my", "အစ\u{1039}စရေးန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Iteraer"),
        ("nb", "Israel"),
        ("ne", "इज\u{94d}रायल"),
        ("nl", "Israël"),
        ("nn", "Israel"),
        ("nv", "Ízrel Bikéyah"),
        ("oc", "Israel"),
        ("or", "ଇଜ\u{b4d}ର\u{b3e}ଇଲ"),
        ("pa", "ਇਜ਼ਰਾਈਲ"),
        ("pi", "इस\u{94d}र\u{948}ल"),
        ("pl", "Izrael"),
        ("ps", "اسرائیل"),
        ("pt", "Israel"),
        ("pt_BR", "Israel"),
        ("ro", "Israel"),
        ("ru", "Израиль"),
        ("rw", "Isirayeli"),
        ("sc", "Israele"),
        ("sd", "اسرائيل جي رياست"),
        ("si", "ඊශ\u{dca}\u{200d}ර\u{dcf}යලය"),
        ("sk", "Izrael"),
        ("sl", "Izrael"),
        ("so", "Israa'iil"),
        ("sq", "Izrael"),
        ("sr", "Израел"),
        ("sv", "Israel"),
        ("sw", "Israel"),
        ("ta", "இஸ\u{bcd}ரேல\u{bcd}"),
        ("te", "ఇస\u{c4d}ర\u{c3e}య\u{c3f}ల\u{c4d}"),
        ("tg", "Исроил"),
        ("th", "อ\u{e34}สราเอล"),
        ("ti", "እስራኤል"),
        ("tk", "Izrail"),
        ("tl", "Israel"),
        ("tr", "İsrail"),
        ("tt", "İсраел"),
        ("ug", "ئىسرائىلىيە"),
        ("uk", "Ізраїль"),
        ("ur", "اسرائیل"),
        ("uz", "Isroil"),
        ("ve", "Israel"),
        ("vi", "Do Thái"),
        ("wa", "Israyel"),
        ("wo", "Israayil"),
        ("xh", "Sirayeli"),
        ("yo", "Ísráẹ\u{301}lì"),
        ("zh_CN", "以色列"),
        ("zh_HK", "以色列"),
        ("zh_TW", "以色列"),
        ("zu", "Isreyili"),
    ];
    #[cfg(all(feature = "il", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 31.046051;
        pub const LONGITUDE: f64 = 34.851612;
        pub const MAX_LATITUDE: f64 = 33.33280500000001;
        pub const MAX_LONGITUDE: f64 = 35.896244;
        pub const MIN_LATITUDE: f64 = 29.47969999999999;
        pub const MIN_LONGITUDE: f64 = 34.2673871;
        pub const NORTHEAST_LATITUDE: f64 = 33.33280500000001;
        pub const NORTHEAST_LONGITUDE: f64 = 35.896244;
        pub const SOUTHWEST_LATITUDE: f64 = 29.47969999999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 34.2673871;
    }
}
#[cfg(all(feature = "il", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 31.046051,
            longitude: 34.851612,
            max_latitude: 33.33280500000001,
            max_longitude: 35.896244,
            min_latitude: 29.47969999999999,
            min_longitude: 34.2673871,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 33.33280500000001,
                    longitude: 35.896244,
                },
                southwest: CountryGeoBound {
                    latitude: 29.47969999999999,
                    longitude: 34.2673871,
                },
            },
        }
    }
}

#[cfg(all(feature = "il", feature = "subdivisions"))]
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
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::IL,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.829562), longitude: Some(35.0388164), max_latitude: Some(31.8779508), min_latitude: Some(29.4906471), max_longitude: Some(35.4549186), min_longitude: Some(34.2673871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suidelike distrik"), ("ar", "المنطقة الجنوبية"), ("az", "İsrail cənub dairəsi"), ("be", "Паўднёвая акруга Ізраіля"), ("bg", "Южен окръг"), ("ca", "Districte del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄎𑄬𑄣"), ("ceb", "Southern District"), ("cs", "Jižní distrikt"), ("da", "Syddistriktet"), ("de", "Südbezirk"), ("en", "Southern District"), ("es", "Distrito Meridional"), ("et", "Lõunaringkond"), ("eu", "Hegoaldeko barrutia"), ("fa", "استان جنوب"), ("fi", "Eteläinen hallintoalue"), ("fr", "district sud"), ("he", "מחוז הדרום"), ("hu", "Déli körzet"), ("hy", "Հարավային մարզ"), ("id", "Distrik Selatan"), ("it", "distretto Sud"), ("ja", "南部地区"), ("ka", "სამხრეთი რაიონი"), ("kk", "Израильдің оңтүстік округі"), ("ko", "남부 구"), ("lt", "Pietų apskritis"), ("lv", "Dienvidu apgabals"), ("mk", "Јужен округ"), ("ms", "Daerah Selatan"), ("nb", "Sørdistriktet"), ("nl", "Zuid"), ("no", "Sørdistriktet"), ("pl", "Dystrykt Południowy"), ("pt", "Distrito Sul"), ("ro", "Districtul de Sud"), ("ru", "Южный округ Израиля"), ("sr", "Јужни округ"), ("sr_Latn", "Južni okrug"), ("sv", "Södra distriktet"), ("tr", "Güney Bölgesi"), ("uk", "Південний округ"), ("ur", "جنوبی ضلع (اسرائیل)"), ("vi", "Quận Nam"), ("zh", "南部区")]),
                        unofficial_name_list: ["Southern"].to_vec(),
                    }
                ),
                (
                    "HA",
                    Subdivision{
                        name: "HA",
                        country_alpha2: Alpha2::IL,
                        code: "HA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.7940463), longitude: Some(34.989571), max_latitude: Some(32.842681), min_latitude: Some(32.7565638), max_longitude: Some(35.079493), min_longitude: Some(34.954059)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Haifa distrik"), ("ar", "منطقة حيفا"), ("az", "Hayfa dairəsi"), ("be", "Хайфская акруга"), ("bg", "Хайфски окръг"), ("bn", "হ\u{9be}ইফ\u{9be} জেল\u{9be}"), ("ca", "Districte de Haifa"), ("ccp", "𑄦\u{1112d}𑄜 𑄎𑄬𑄣"), ("ceb", "Haifa (distrito)"), ("cs", "Haifský distrikt"), ("da", "Haifa-distriktet"), ("de", "Bezirk Haifa"), ("el", "Χαΐφα"), ("en", "Haifa District"), ("es", "Distrito de Haifa"), ("et", "Haifa ringkond"), ("eu", "Haifa barrutia"), ("fa", "استان حیفا"), ("fi", "Haifan hallintoalue"), ("fr", "district de Haïfa"), ("gu", "હ\u{ac8}ફા જિલ\u{acd}લો"), ("he", "מחוז חיפה"), ("hi", "ह\u{948}फा जिला"), ("hr", "Okrug Haifa"), ("hu", "Haifai körzet"), ("hy", "Հայֆայի մարզ"), ("id", "Distrik Haifa"), ("it", "distretto di Haifa"), ("ja", "ハイファ地区"), ("ka", "ხაიფის რაიონი"), ("kk", "Хайфа округі"), ("kn", "ಹೈಫಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "하이파 구"), ("lt", "Haifos apskritis"), ("lv", "Haifas apgabals"), ("mk", "Хаифа"), ("mr", "ह\u{948}फा जिल\u{94d}हा"), ("ms", "Daerah Haifa"), ("nb", "Haifa-distriktet"), ("nl", "Haifa"), ("no", "Haifa-distriktet"), ("pl", "Dystrykt Hajfy"), ("pt", "Haifa"), ("ro", "Districtul Haifa"), ("ru", "Хайфский округ"), ("si", "හය\u{dd2}ෆ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Хаифа"), ("sr_Latn", "Haifa"), ("sv", "Haifa"), ("ta", "ஹேப\u{bcd}பிய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "హ\u{c48}ఫ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตไฮฟา"), ("tr", "Hayfa Bölgesi"), ("uk", "Хайфський округ"), ("ur", "حیفا ضلع"), ("vi", "Quận Haifa"), ("zh", "海法区")]),
                        unofficial_name_list: ["Haifa", "Haifa", "Haifa", "Hefa", "H\u{331}efa"].to_vec(),
                    }
                ),
                (
                    "JM",
                    Subdivision{
                        name: "JM",
                        country_alpha2: Alpha2::IL,
                        code: "JM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.768319), longitude: Some(35.21371), max_latitude: Some(31.8829601), min_latitude: Some(31.7096771), max_longitude: Some(35.2652869), min_longitude: Some(35.0854311)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jerusalem distrik"), ("ar", "منطقة القدس"), ("az", "Qüds dairəsi"), ("be", "Іерусалімская акруга"), ("bg", "Йерусалимски окръг"), ("ca", "Districte de Jerusalem"), ("ccp", "𑄎𑄬𑄢\u{1112a}𑄎𑄣𑄬𑄟\u{11134}"), ("ceb", "Jerusalem"), ("cs", "Jeruzalémský distrikt"), ("da", "Jerusalem-distriktet"), ("de", "Bezirk Jerusalem"), ("en", "Jerusalem"), ("es", "Distrito de Jerusalén"), ("et", "Jeruusalemma ringkond"), ("eu", "Jerusalem barrutia"), ("fa", "استان اورشلیم"), ("fi", "Jerusalemin hallintoalue"), ("fr", "district de Jérusalem"), ("gl", "Distrito de Xerusalén"), ("he", "מחוז ירושלים"), ("hu", "Jeruzsálemi körzet"), ("hy", "Երուսաղեմի մարզ"), ("id", "Distrik Yerusalem"), ("it", "distretto di Gerusalemme"), ("ja", "エルサレム地区"), ("kk", "Иерусалим округі"), ("ko", "예루살렘 구"), ("lt", "Jeruzalės apskritis"), ("lv", "Jeruzalemes apgabals"), ("mk", "Ерусалим"), ("ms", "Daerah Jerusalem"), ("nb", "Jerusalem-distriktet"), ("nl", "Jeruzalem"), ("no", "Jerusalem-distriktet"), ("pl", "Dystrykt Jerozolimy"), ("pt", "Jerusalém"), ("ro", "Districtul Ierusalim"), ("ru", "Иерусалимский округ"), ("sr", "Јерусалим"), ("sr_Latn", "Jerusalim"), ("sv", "Jerusalem"), ("tr", "Kudüs Bölgesi"), ("uk", "Єрусалимський округ"), ("ur", "یروشلم ضلع"), ("vi", "Quận Jerusalem"), ("zh", "耶路撒冷区")]),
                        unofficial_name_list: ["Jerusalem", "Jerusalén", "Jérusalem", "Yerushalayim", "al-Quds"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::IL,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.9521108), longitude: Some(34.906551), max_latitude: Some(32.4126018), min_latitude: Some(31.7571918), max_longitude: Some(35.051422), min_longitude: Some(34.66654279999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sentrale distrik"), ("ar", "المنطقة الوسطى"), ("az", "İsrail mərkəzi dairəsi"), ("be", "Цэнтральная акруга"), ("bg", "Централен окръг"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল জেল\u{9be}"), ("ca", "Districte Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄎𑄬𑄣"), ("ceb", "Central District"), ("cs", "Centrální distrikt"), ("da", "Centraldistriktet"), ("de", "Zentralbezirk"), ("el", "Σέντραλ Ντίστρικτ, Ισραήλ"), ("en", "Central District"), ("es", "Distrito Central"), ("et", "Keskringkond"), ("eu", "Erdialdeko barrutia"), ("fa", "استان مرکز"), ("fi", "Keski-Israelin hallintoalue"), ("fr", "district centre"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ જિલ\u{acd}લો"), ("he", "מחוז המרכז"), ("hi", "मध\u{94d}य जिला (इज\u{93c}राइल)"), ("hu", "Központi körzet"), ("hy", "Կենտրոնական մարզ"), ("id", "Distrik Tengah"), ("it", "distretto Centrale"), ("ja", "中央地区"), ("ka", "ცენტრალური რაიონი"), ("kk", "Израильдің орталық округі"), ("kn", "ಕೇಂದ\u{ccd}ರ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "중부 구"), ("lt", "Centrinė apskritis"), ("lv", "Centra apgabals"), ("mk", "Централен округ"), ("mr", "मध\u{94d}यवर\u{94d}ती जिल\u{94d}हा"), ("ms", "Daerah Tengah"), ("nb", "Sentraldistriktet"), ("nl", "Centrum"), ("no", "Sentraldistriktet"), ("pl", "Dystrykt Centralny"), ("pt", "Distrito Central"), ("ro", "Districtul Central"), ("ru", "Центральный округ"), ("si", "මද\u{dca}\u{200d}යම ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Централни округ"), ("sr_Latn", "Centralni okrug"), ("sv", "Centrala distriktet"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเซนทร\u{e31}ล"), ("tr", "Merkez Bölge"), ("uk", "Центральний округ"), ("ur", "مرکزی ضلع (اسرائیل)"), ("vi", "Quận Trung"), ("zh", "中央区")]),
                        unofficial_name_list: ["Central"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::IL,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.0852999), longitude: Some(34.78176759999999), max_latitude: Some(32.1465073), min_latitude: Some(32.0292531), max_longitude: Some(34.8519761), min_longitude: Some(34.7425159)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tel Aviv distrik"), ("ar", "منطقة تل أبيب"), ("az", "Təl-Əviv dairəsi"), ("be", "Тэль-Авіўская акруга"), ("bg", "Телавивски окръг"), ("ca", "Districte de Tel Aviv"), ("ccp", "𑄑𑄬𑄣\u{11134} 𑄃𑄞\u{11128}𑄛\u{11134} 𑄎𑄬𑄣"), ("ceb", "Tel Aviv District"), ("cs", "Telavivský distrikt"), ("da", "Tel Aviv-distriktet"), ("de", "Bezirk Tel Aviv"), ("el", "Διαμέρισμα του Τελ Αβίβ"), ("en", "Tel Aviv District"), ("es", "Distrito de Tel Aviv"), ("et", "Tel Avivi ringkond"), ("eu", "Tel Aviv barrutia"), ("fa", "استان تل\u{200c}آویو"), ("fr", "district de Tel Aviv"), ("he", "מחוז תל אביב"), ("hu", "Tel-Avivi járás"), ("hy", "Թել Ավիվի մարզ"), ("id", "Distrik Tel Aviv"), ("it", "distretto di Tel Aviv"), ("ja", "テルアビブ地区"), ("kk", "Тель-Авив округі"), ("ko", "텔아비브 구"), ("lt", "Tel Avivo apskritis"), ("lv", "Telavivas apgabals"), ("mk", "Тел Авив"), ("ms", "Daerah Tel Aviv"), ("nb", "Tel Aviv-distriktet"), ("nl", "Tel Aviv"), ("no", "Tel Aviv-distriktet"), ("pl", "Dystrykt Tel Awiwu"), ("pt", "Tel Aviv"), ("ro", "Districtul Tel Aviv"), ("ru", "Тель-Авивский округ"), ("sr", "Тел Авив"), ("sr_Latn", "Tel Aviv"), ("sv", "Tel Aviv"), ("tr", "Tel Aviv Bölgesi"), ("uk", "Тель-Авівський округ"), ("ur", "تل ابیب ضلع"), ("vi", "Quận Tel Aviv"), ("zh", "特拉维夫区")]),
                        unofficial_name_list: ["Tel-Aviv"].to_vec(),
                    }
                ),
                (
                    "Z",
                    Subdivision{
                        name: "Z",
                        country_alpha2: Alpha2::IL,
                        code: "Z",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.8972246), longitude: Some(35.3027226), max_latitude: Some(33.33280500000001), min_latitude: Some(32.3869671), max_longitude: Some(35.896244), min_longitude: Some(35.0272979)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordelike distrik"), ("ar", "المنطقة الشمالية"), ("az", "İsrail şimal dairəsi"), ("be", "Паўночная акруга"), ("bg", "Северен окръг"), ("bn", "নর\u{9cd}দ\u{9be}ন জেল\u{9be}"), ("ca", "Districte del Nord"), ("ccp", "𑄅\u{11127}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄎𑄬𑄣"), ("ceb", "Northern District"), ("cs", "Severní distrikt"), ("da", "Norddistriktet"), ("de", "Nordbezirk"), ("el", "Βόρεια Περιοχή, Ισραήλ"), ("en", "Northern District"), ("es", "Distrito Norte"), ("et", "Põhjaringkond"), ("eu", "Iparraldeko barrutia"), ("fa", "استان شمال"), ("fi", "Pohjoinen hallintoalue"), ("fr", "district nord"), ("gu", "ઉત\u{acd}તરીય જિલ\u{acd}લો"), ("he", "מחוז הצפון"), ("hi", "उत\u{94d}तरी जिला"), ("hu", "Északi körzet"), ("hy", "Հյուսիսային մարզ"), ("id", "Distrik Utara"), ("it", "distretto Nord"), ("ja", "北部地区"), ("ka", "ჩრდილოეთი რაიონი"), ("kk", "Израильдің солтүстік округі"), ("kn", "ಉತ\u{ccd}ತರ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "북부 구"), ("lt", "Šiaurės apskritis"), ("lv", "Ziemeļu apgabals"), ("mk", "Северен округ"), ("mr", "उत\u{94d}तर जिल\u{94d}हा"), ("ms", "Daerah Utara"), ("nb", "Norddistriktet"), ("nl", "Noord"), ("no", "Norddistriktet"), ("pl", "Dystrykt Północny"), ("pt", "Distrito Norte"), ("ro", "Districtul de Nord"), ("ru", "Северный округ Израиля"), ("si", "උත\u{dd4}ර\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Северни округ"), ("sr_Latn", "Severni okrug"), ("sv", "Norra distriktet"), ("ta", "வடக\u{bcd}கு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఉత\u{c4d}తర జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "นอร\u{e4c}ทเท\u{e34}ร\u{e4c}น ด\u{e34}สทร\u{e34}ค"), ("tr", "Kuzey Bölgesi"), ("uk", "Північний округ"), ("ur", "شمالی ضلع (اسرائیل)"), ("vi", "Quận Bắc"), ("zh", "北部区")]),
                        unofficial_name_list: ["Northern"].to_vec(),
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
#[cfg(feature = "il")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::IL,
        alpha3: Alpha3::ISR,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 972,
        currency_code: "ILS",
        gec: Some(GEC::IS),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ISR),
        iso_long_name: "The State of Israel",
        iso_short_name: "Israel",
        official_language_list: ["ar", "he"].to_vec(),
        spoken_language_list: ["ar", "he"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Israeli"),
        number: "376",
        postal_code: true,
        postal_code_format: Some("\\d{5}(?:\\d{2})?"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "IL",
        unofficial_name_list: ["Israel", "Israël", "イスラエル"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Israel"),
            ("af", "Israel"),
            ("ak", "Israel"),
            ("am", "ጕስሲጔሔ"),
            ("an", "Israel"),
            ("ar", "إسرائيل"),
            ("as", "ইজ\u{9cd}ৰ\u{9be}ইল"),
            ("ay", "Israel"),
            ("az", "İsrail"),
            ("ba", "Israel"),
            ("be", "Ізраіль"),
            ("bg", "Израел"),
            ("bi", "Israel"),
            ("bn", "ইজর\u{9be}ইল"),
            ("bn_IN", "ইজর\u{9be}ইল"),
            ("br", "Israel"),
            ("bs", "Izrael"),
            ("ca", "Israel"),
            ("ce", "Израиль"),
            ("ch", "Israel"),
            ("cs", "Izrael"),
            ("cv", "Израиль"),
            ("cy", "Israel"),
            ("da", "Israel"),
            ("de", "Israel"),
            ("dv", "އ\u{7a8}ސ\u{7b0}ރ\u{7a7}އ\u{7a9}ލ\u{7aa}"),
            ("dz", "ཨ\u{f72}ཛ\u{f72}་ར\u{f7a}ལ།"),
            ("ee", "Israel"),
            ("el", "Ισραήλ"),
            ("en", "Israel"),
            ("eo", "Israelo"),
            ("es", "Israel"),
            ("et", "Iisrael"),
            ("eu", "Israel"),
            ("fa", "اسراییل"),
            ("ff", "Israel"),
            ("fi", "Israel"),
            ("fo", "Ísrael"),
            ("fr", "Israël"),
            ("fy", "Israel"),
            ("ga", "Iosrael"),
            ("gl", "Israel"),
            ("gn", "Israel"),
            ("gu", "ઇઝરાય\u{ac7}લ"),
            ("gv", "Israel"),
            ("ha", "Isra'ila"),
            ("he", "ישראל"),
            ("hi", "इज\u{93c}राइल"),
            ("hr", "Izrael"),
            ("ht", "Izrayèl"),
            ("hu", "Izrael"),
            ("hy", "Իսրայել"),
            ("ia", "Israel"),
            ("id", "Israel"),
            ("io", "Israel"),
            ("is", "Ísrael"),
            ("it", "Israele"),
            ("iu", "Israel"),
            ("ja", "イスラエル"),
            ("ka", "ისრაელი"),
            ("ki", "Israel"),
            ("kk", "Израиль"),
            ("kl", "Israel"),
            ("km", "អ\u{17ca}\u{17b8}ស\u{17d2}រាអែល"),
            ("kn", "ಇಸ\u{ccd}ರೇಲ\u{ccd}"),
            ("ko", "이스라엘"),
            ("ku", "Îsraîl"),
            ("kv", "Израиль"),
            ("kw", "Ysrael"),
            ("ky", "Израиль"),
            ("lo", "Israel"),
            ("lt", "Izraelis"),
            ("lv", "Izraēla"),
            ("mi", "Iharaira"),
            ("mk", "Израел"),
            ("ml", "ഇസ\u{d4d}ര\u{d3e}യേല\u{d4d}\u{200d}"),
            ("mn", "Израйл"),
            ("mr", "इस\u{94d}रायल"),
            ("ms", "Israel"),
            ("mt", "Iżrael"),
            ("my", "အစ\u{1039}စရေးန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Iteraer"),
            ("nb", "Israel"),
            ("ne", "इज\u{94d}रायल"),
            ("nl", "Israël"),
            ("nn", "Israel"),
            ("nv", "Ízrel Bikéyah"),
            ("oc", "Israel"),
            ("or", "ଇଜ\u{b4d}ର\u{b3e}ଇଲ"),
            ("pa", "ਇਜ਼ਰਾਈਲ"),
            ("pi", "इस\u{94d}र\u{948}ल"),
            ("pl", "Izrael"),
            ("ps", "اسرائیل"),
            ("pt", "Israel"),
            ("pt_BR", "Israel"),
            ("ro", "Israel"),
            ("ru", "Израиль"),
            ("rw", "Isirayeli"),
            ("sc", "Israele"),
            ("sd", "اسرائيل جي رياست"),
            ("si", "ඊශ\u{dca}\u{200d}ර\u{dcf}යලය"),
            ("sk", "Izrael"),
            ("sl", "Izrael"),
            ("so", "Israa'iil"),
            ("sq", "Izrael"),
            ("sr", "Израел"),
            ("sv", "Israel"),
            ("sw", "Israel"),
            ("ta", "இஸ\u{bcd}ரேல\u{bcd}"),
            ("te", "ఇస\u{c4d}ర\u{c3e}య\u{c3f}ల\u{c4d}"),
            ("tg", "Исроил"),
            ("th", "อ\u{e34}สราเอล"),
            ("ti", "እስራኤል"),
            ("tk", "Izrail"),
            ("tl", "Israel"),
            ("tr", "İsrail"),
            ("tt", "İсраел"),
            ("ug", "ئىسرائىلىيە"),
            ("uk", "Ізраїль"),
            ("ur", "اسرائیل"),
            ("uz", "Isroil"),
            ("ve", "Israel"),
            ("vi", "Do Thái"),
            ("wa", "Israyel"),
            ("wo", "Israayil"),
            ("xh", "Sirayeli"),
            ("yo", "Ísráẹ\u{301}lì"),
            ("zh_CN", "以色列"),
            ("zh_HK", "以色列"),
            ("zh_TW", "以色列"),
            ("zu", "Isreyili"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
