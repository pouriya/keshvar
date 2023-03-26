// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Colombia

#[cfg(all(feature = "co", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CO;
    pub const ALPHA3: Alpha3 = Alpha3::COL;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 57;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::COP;
    pub const GEC: Option<GEC> = Some(GEC::CO);
    pub const INTERNATIONAL_PREFIX: &str = "005";
    pub const IOC: Option<IOC> = Some(IOC::COL);
    pub const ISO_SHORT_NAME: &str = "Colombia";
    pub const ISO_LONG_NAME: &str = "The Republic of Colombia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 10];
    pub const NATIONAL_PREFIX: &str = "05";
    pub const NATIONALITY: Option<&str> = Some("Colombian");
    pub const NUMBER: &str = "170";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "CO";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Colombia", "Kolumbien", "Colombie", "コロンビア"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Colombia"),
        ("af", "Colombië"),
        ("ak", "Colombia"),
        ("am", "ጥሕሤቢ።"),
        ("an", "Colombia"),
        ("ar", "كولومبيا"),
        ("as", "কোলোম\u{9cd}বিয়\u{9be}"),
        ("ay", "Colombia"),
        ("az", "Kolumbiya"),
        ("ba", "Colombia"),
        ("be", "Калумбія"),
        ("bg", "Колумбия"),
        ("bi", "Colombia"),
        ("bn", "কোলোম\u{9cd}বিয়\u{9be}"),
        ("bn_IN", "কোলোম\u{9cd}বিয়\u{9be}"),
        ("br", "Kolombia"),
        ("bs", "Kolumbija"),
        ("ca", "Colòmbia"),
        ("ce", "Колумби"),
        ("ch", "Colombia"),
        ("cs", "Kolumbie"),
        ("cv", "Колумби"),
        ("cy", "Colombia"),
        ("da", "Colombia"),
        ("de", "Kolumbien"),
        ("dv", "ކ\u{7ae}ލ\u{7a6}ނ\u{7b0}ބ\u{7a8}އ\u{7a7}"),
        ("dz", "ཀ\u{f7c}་ལ\u{f7c}མ་བ\u{f72}་ཡ།"),
        ("ee", "Colombia"),
        ("el", "Κολομβία"),
        ("en", "Colombia"),
        ("eo", "Kolombio"),
        ("es", "Colombia"),
        ("et", "Colombia"),
        ("eu", "Kolonbia"),
        ("fa", "کلمبیا"),
        ("ff", "Colombia"),
        ("fi", "Kolumbia"),
        ("fo", "Kolombia"),
        ("fr", "Colombie"),
        ("fy", "Kolombia"),
        ("ga", "An Cholóim"),
        ("gl", "Colombia"),
        ("gn", "Colombia"),
        ("gu", "કોલોમ\u{acd}બિયા"),
        ("gv", "Yn Cholombey"),
        ("ha", "Colombia"),
        ("he", "קולומביה"),
        ("hi", "कोलम\u{94d}बिया"),
        ("hr", "Kolumbija"),
        ("ht", "Kolonbi"),
        ("hu", "Kolumbia"),
        ("hy", "Կոլումբիա"),
        ("ia", "Colombia"),
        ("id", "Kolombia"),
        ("io", "Kolumbia"),
        ("is", "Kólumbía"),
        ("it", "Colombia"),
        ("iu", "Colombia"),
        ("ja", "コロンビア"),
        ("ka", "კოლუმბია"),
        ("ki", "Colombia"),
        ("kk", "Колумбия"),
        ("kl", "Colombia"),
        ("km", "ក\u{17bc}ឡ\u{17bb}\u{17c6}ប\u{17ca}\u{17b8}"),
        ("kn", "ಕೋಲಂಬ\u{cbf}ಯಾ"),
        ("ko", "콜롬비아"),
        ("ku", "Kolombiya"),
        ("kv", "Colombia"),
        ("kw", "Kolombi"),
        ("ky", "Колумбия"),
        ("lo", "ປະເທດໂກລ\u{ebb}ມບ\u{eb5}"),
        ("lt", "Kolumbija"),
        ("lv", "Kolumbija"),
        ("mi", "Koromōpia"),
        ("mk", "Колумбија"),
        ("ml", "കൊളംബിയ"),
        ("mn", "Колумб"),
        ("mr", "कोल\u{902}बिया"),
        ("ms", "Colombia"),
        ("mt", "Colombia"),
        (
            "my",
            "က\u{102d}\u{102f}လ\u{1036}ဘ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Korombiya"),
        ("nb", "Colombia"),
        ("ne", "कोलम\u{94d}बिया"),
        ("nl", "Colombia"),
        ("nn", "Colombia"),
        ("nv", "Colombia"),
        ("oc", "Colombia"),
        ("or", "କୋଲୋମ\u{b4d}ବ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਕ\u{a4b}ਲ\u{a70}ਬੀਆ"),
        ("pi", "कोलोम\u{94d}बिया"),
        ("pl", "Kolumbia"),
        ("ps", "کولمبیا"),
        ("pt", "Colômbia"),
        ("pt_BR", "Colômbia"),
        ("ro", "Columbia"),
        ("ru", "Колумбия"),
        ("rw", "Kolombiya"),
        ("sc", "Colòmbia"),
        ("sd", "Colombia"),
        ("si", "කොලොම\u{dca}බ\u{dd2}ය\u{dcf}ව"),
        ("sk", "Kolumbia"),
        ("sl", "Kolumbija"),
        ("so", "Kolombiya"),
        ("sq", "Kolumbi"),
        ("sr", "Колумбија"),
        ("sv", "Colombia"),
        ("sw", "Kolombia"),
        ("ta", "கொலம\u{bcd}பிய\u{bbe}"),
        ("te", "క\u{c4b}లంబ\u{c3f}య\u{c3e}"),
        ("tg", "Колумбия"),
        ("th", "โคลอมเบ\u{e35}ย"),
        ("ti", "ኮሎምብያ"),
        ("tk", "Kolumbiýa"),
        ("tl", "Kolombya"),
        ("tr", "Kolombiya"),
        ("tt", "Коломбиа"),
        ("ug", "كولومبىيە"),
        ("uk", "Колумбія"),
        ("ur", "کولمبیا"),
        ("uz", "Kolumbiya"),
        ("ve", "Colombia"),
        ("vi", "Cô-lôm-bi-a"),
        ("wa", "Colombeye"),
        ("wo", "Koloombi"),
        ("xh", "Colombia"),
        ("yo", "Kòlómbìà"),
        ("zh_CN", "哥伦比亚"),
        ("zh_HK", "哥倫比亞"),
        ("zh_TW", "哥倫比亞"),
        ("zu", "IKolombiya"),
    ];
    #[cfg(all(feature = "co", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 4.570868;
        pub const LONGITUDE: f64 = -74.297333;
        pub const MAX_LATITUDE: f64 = 13.5177999;
        pub const MAX_LONGITUDE: f64 = -66.8463122;
        pub const MIN_LATITUDE: f64 = -4.227109899999999;
        pub const MIN_LONGITUDE: f64 = -81.8317;
        pub const NORTHEAST_LATITUDE: f64 = 13.5177999;
        pub const NORTHEAST_LONGITUDE: f64 = -66.8463122;
        pub const SOUTHWEST_LATITUDE: f64 = -4.227109899999999;
        pub const SOUTHWEST_LONGITUDE: f64 = -81.8317;
    }
}
#[cfg(all(feature = "co", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 4.570868,
            longitude: -74.297333,
            max_latitude: 13.5177999,
            max_longitude: -66.8463122,
            min_latitude: -4.227109899999999,
            min_longitude: -81.8317,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 13.5177999,
                    longitude: -66.8463122,
                },
                southwest: CountryGeoBound {
                    latitude: -4.227109899999999,
                    longitude: -81.8317,
                },
            },
        }
    }
}

#[cfg(all(feature = "co", feature = "subdivisions"))]
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
                    "AMA",
                    Subdivision{
                        name: "Amazonas",
                        country_alpha2: Alpha2::CO,
                        code: "AMA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.4429123), longitude: Some(-71.5723953), max_latitude: Some(0.11057), min_latitude: Some(-4.22727), max_longitude: Some(-69.39555), min_longitude: Some(-74.39015)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أمازوناس"), ("az", "Amasonas (Kolumbiya)"), ("bg", "Амасонас"), ("bn", "আম\u{9be}জ\u{9c1}ন\u{9be}স বিভ\u{9be}গ"), ("ca", "Departament de l’Amazones"), ("ccp", "𑄃𑄟𑄎\u{11127}𑄚𑄌\u{11134}"), ("ceb", "Departamento del Amazonas"), ("cs", "Amazonas"), ("da", "Amazonas Department"), ("de", "Departamento de Amazonas"), ("el", "Διαμέρισμα Αμαζόνιου"), ("en", "Amazonas"), ("es", "Amazonas"), ("et", "Amazonase departemang"), ("eu", "Amazonas"), ("fa", "حوزه امازوناس"), ("fi", "Amazonas"), ("fr", "Amazonas"), ("ga", "Amazonas"), ("gl", "Amazonas"), ("gu", "એમ\u{ac7}ઝોનાઝ વિભાગ"), ("he", "אמסונאס"), ("hi", "अम\u{947}ज\u{93c}ोनास विभाग"), ("hr", "Amazonas"), ("hu", "Amazonas megye"), ("hy", "Ամազոնաս"), ("id", "Departemen Amazonas"), ("it", "dipartimento di Amazonas"), ("ja", "アマソナス県"), ("ka", "ამასონასის დეპარტამენტი"), ("kn", "ಅಮ\u{cc6}ಜೋನಾಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "아마소나스 주"), ("lt", "Amazonės departamentas"), ("lv", "Amazonasas departaments"), ("mr", "अम\u{945}झॉन\u{94d}स विभाग"), ("ms", "Pentadbiran Amazonas"), ("nb", "Amazonas"), ("nl", "Amazonas"), ("no", "Amazonas"), ("pl", "Amazonas"), ("pt", "Amazonas"), ("ro", "Departamentul Amazonas"), ("ru", "Амасонас"), ("si", "ඇමසෝන\u{dcf}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Amazonas (department v Kolumbii)"), ("sv", "Amazonas"), ("ta", "அம\u{bbe}சோனஸ\u{bcd} துறை"), ("te", "అమ\u{c46}జ\u{c3e}న\u{c4b}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตอะเมโซนาส"), ("tr", "Amazonas Departmantı"), ("uk", "Амасонас"), ("ur", "ایمازوناس محکمہ"), ("vi", "Amazonas"), ("yo", "Amazonas Department"), ("yo_BJ", "Amazonas Department"), ("zh", "亚马孙省")]),
                        unofficial_name_list: ["Amazonas"].to_vec(),
                    }
                ),
                (
                    "ANT",
                    Subdivision{
                        name: "Antioquia",
                        country_alpha2: Alpha2::CO,
                        code: "ANT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.1986064), longitude: Some(-75.34121789999999), max_latitude: Some(8.889778699999999), min_latitude: Some(5.41826), max_longitude: Some(-73.88748), min_longitude: Some(-77.12709)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Antioquia"), ("ar", "أنتيوكيا"), ("az", "Antokiya"), ("be", "Анцьёкія"), ("bg", "Антиокия"), ("bn", "আন\u{9cd}টিক\u{9c1}ইয\u{9bc}\u{9be} বিভ\u{9be}গ"), ("ca", "Departament d’Antioquia"), ("ccp", "𑄃𑄚\u{11134}𑄑\u{11128}𑄃\u{1112e}𑄇\u{1112d}\u{1112a}𑄠"), ("ceb", "Departamento de Antioquia"), ("cs", "Antioquia"), ("da", "Antioquia Department"), ("de", "Departamento de Antioquia"), ("el", "Αντιόχεια Κολομβίας"), ("en", "Antioquia"), ("es", "Antioquia"), ("et", "Antioquia departemang"), ("eu", "Antioquia"), ("fa", "شهرستان انتیوکیا"), ("fi", "Antioquia"), ("fr", "Antioquia"), ("ga", "Antioquia"), ("gl", "Antioquia"), ("gu", "એન\u{acd}ટિઓક\u{acd}વિઆ વિભાગ"), ("he", "אנטיוקיה"), ("hi", "ए\u{902}टियोक\u{94d}विया विभाग"), ("hr", "Antioquia"), ("hu", "Antioquia megye"), ("hy", "Անտիոքիա"), ("id", "Departemen Antioquia"), ("it", "dipartimento di Antioquia"), ("ja", "アンティオキア県"), ("ka", "ანტიოკიის დეპარტამენტი"), ("kn", "ಅಂಟ\u{cbf}ಯೋಕ\u{ccd}ವ\u{cbf}ಯಾ ಇಲಾಖ\u{cc6}"), ("ko", "안티오키아 주"), ("lt", "Antiokijos departamentas"), ("lv", "Antjokijas departaments"), ("mk", "Антиокија"), ("mr", "एन\u{94d}टिऑक\u{94d}विया विभाग"), ("ms", "Pentadbiran Antioquia"), ("nb", "Antioquia"), ("nl", "Antioquia"), ("no", "Antioquia"), ("pl", "Antioquia"), ("pt", "Antioquia"), ("ro", "Departamentul Antioquia"), ("ru", "Антьокия"), ("si", "අන\u{dca}ට\u{dd2}යෝක\u{dd4}ය\u{dd2}ය\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Antioquia"), ("sv", "Antioquia"), ("ta", "அண\u{bcd}டியகுஐய\u{bbe} துறை"), ("te", "ఆంట\u{c3f}య\u{c4b}క\u{c3f}య\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "แอนต\u{e34}โอก\u{e31}ว"), ("tr", "Antioquia"), ("uk", "Антіокія"), ("ur", "انتیوکیا محکمہ"), ("vi", "Antioquia"), ("yo", "Antioquia Department"), ("yo_BJ", "Antioquia Department"), ("yue", "晏托卡省"), ("yue_Hans", "晏托卡省"), ("zh", "安蒂奥基亚省")]),
                        unofficial_name_list: ["Antioquia"].to_vec(),
                    }
                ),
                (
                    "ARA",
                    Subdivision{
                        name: "Arauca",
                        country_alpha2: Alpha2::CO,
                        code: "ARA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.079371), longitude: Some(-70.758377), max_latitude: Some(7.10438), min_latitude: Some(6.48352), max_longitude: Some(-69.76795), min_longitude: Some(-71.20248)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أروكا"), ("az", "Arauka (departament)"), ("bg", "Араука"), ("bn", "আরোক\u{9be} বিভ\u{9be}গ"), ("ca", "Departament d’Arauca"), ("ccp", "𑄃𑄢𑄅\u{1112a}𑄇"), ("ceb", "Departamento de Arauca"), ("cs", "Arauca"), ("da", "Arauca"), ("de", "Departamento del Arauca"), ("el", "Αράουκα"), ("en", "Arauca"), ("es", "Arauca"), ("eu", "Arauca"), ("fa", "شهرستان آرائوکا"), ("fi", "Arauca"), ("fr", "Arauca"), ("ga", "Arauca"), ("gl", "Arauca"), ("gu", "અરૌકા વિભાગ"), ("he", "אראוקה (מחוז)"), ("hi", "अराउका विभाग"), ("hr", "Arauca"), ("hu", "Arauca megye"), ("id", "Departemen Arauca"), ("it", "dipartimento di Arauca"), ("ja", "アラウカ県"), ("ka", "არაუკის დეპარტამენტი"), ("kn", "ಅರ\u{ccc}ಕ ಇಲಾಖ\u{cc6}"), ("ko", "아라우카 주"), ("lt", "Araukos departamentas"), ("lv", "Araukas departments"), ("mr", "अरौका विभाग"), ("ms", "Pentadbiran Arauca"), ("nb", "Arauca"), ("nl", "Arauca"), ("no", "Arauca"), ("pl", "Arauca"), ("pt", "Arauca"), ("ro", "Departamentul Arauca"), ("ru", "Араука"), ("si", "අරව\u{dd4}ක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Arauca"), ("sv", "Arauca"), ("ta", "அறவுக\u{bcd}க\u{bbe} துறை"), ("te", "అర\u{c4c}క\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอราวกา"), ("tr", "Arauca"), ("uk", "Араука"), ("ur", "آراوکا محکمہ"), ("vi", "Arauca"), ("yo", "Arauca Department"), ("yo_BJ", "Arauca Department"), ("zh", "阿劳卡省")]),
                        unofficial_name_list: ["Arauca"].to_vec(),
                    }
                ),
                (
                    "ATL",
                    Subdivision{
                        name: "Atlántico",
                        country_alpha2: Alpha2::CO,
                        code: "ATL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.6966159), longitude: Some(-74.8741045), max_latitude: Some(11.1067254), min_latitude: Some(10.2557104), max_longitude: Some(-74.71324), min_longitude: Some(-75.26715)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أتلانتيكو"), ("az", "Atlantiko"), ("be", "дэпартамент Атлантыка"), ("bg", "Атлантико"), ("bn", "আটল\u{9be}ন\u{9cd}টিকো বিভ\u{9be}গ"), ("ca", "Departament de l’Atlàntic"), ("ccp", "𑄃𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11134}𑄑\u{11128}𑄇\u{1112e}"), ("ceb", "Departamento del Atlántico"), ("cs", "Atlántico"), ("da", "Atlántico Department"), ("de", "Departamento del Atlántico"), ("el", "Διαμέρισμα Ατλαντικού"), ("en", "Atlántico"), ("es", "Atlántico"), ("eu", "Atlantikoko departamendua"), ("fa", "شهرستان آتلانتیکو"), ("fi", "Atlántico"), ("fr", "Atlántico"), ("ga", "Atlántico"), ("gl", "Atlántico"), ("gu", "એટલાન\u{acd}ટિકો વિભાગ"), ("he", "אטלאנטיקו"), ("hi", "अटला\u{902}टिको विभाग"), ("hr", "Atlántico"), ("hu", "Atlántico megye"), ("id", "Departemen Atlántico"), ("it", "dipartimento dell’Atlantico"), ("ja", "アトランティコ県"), ("ka", "ატლანტიკოს დეპარტამენტი"), ("kn", "ಅಟ\u{ccd}ಲಾಂಟ\u{cbf}ಕೊ ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "아틀란티코 주"), ("lt", "Atlantiko departamentas"), ("lv", "Atlantiko departaments"), ("mr", "अटला\u{902}टिको विभाग"), ("ms", "Pentadbiran Atlántico"), ("nb", "Atlántico"), ("nl", "Atlántico"), ("no", "Atlántico"), ("pl", "Atlántico"), ("pt", "Atlántico"), ("ro", "Departamentul Atlántico"), ("ru", "Атлантико"), ("si", "ඇට\u{dca}ලන\u{dca}ට\u{dd2}කෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Atlántico"), ("sv", "Atlántico"), ("ta", "அட\u{bcd}ல\u{bbe}ன\u{bcd}டிகோ துறை"), ("te", "అట\u{c4d}ల\u{c3e}ంట\u{c3f}క\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "แอตแลนต\u{e34}โก"), ("tr", "Atlántico"), ("uk", "Атлантіко"), ("ur", "اتلانتیکو محکمہ"), ("vi", "Atlántico"), ("yo", "Atlántico Department"), ("yo_BJ", "Atlántico Department"), ("zh", "大西洋省")]),
                        unofficial_name_list: ["Atlántico"].to_vec(),
                    }
                ),
                (
                    "BOL",
                    Subdivision{
                        name: "Bolívar",
                        country_alpha2: Alpha2::CO,
                        code: "BOL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.6704382), longitude: Some(-74.0300122), max_latitude: Some(10.8013601), min_latitude: Some(6.99981), max_longitude: Some(-73.74835), min_longitude: Some(-75.70403019999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بوليفار"), ("az", "Bolivar (departament)"), ("bg", "Боливар"), ("bn", "বলিভ\u{9be}র বিভ\u{9be}গ"), ("ca", "Departament de Bolívar"), ("ccp", "𑄝\u{11127}𑄣\u{11128}𑄞𑄢\u{11134}"), ("ceb", "Departamento de Bolívar"), ("cs", "Bolívar"), ("da", "Bolívar Department"), ("de", "Departamento de Bolívar"), ("el", "Διαμέρισμα Μπολίβαρ"), ("en", "Bolívar"), ("es", "Bolívar"), ("eu", "Bolívar"), ("fa", "شهرستان بولیوار، کلمبیا"), ("fi", "Bolívar"), ("fr", "Bolívar"), ("gl", "Bolívar"), ("gu", "બોલિવર વિભાગ"), ("he", "בוליבר"), ("hi", "बोलिवार विभाग"), ("hr", "Bolívar"), ("hu", "Bolívar megye"), ("hy", "Բոլիվար"), ("id", "Departemen Bolívar"), ("it", "dipartimento di Bolívar"), ("ja", "ボリーバル県"), ("ka", "ბოლივარის დეპარტამენტი"), ("kn", "ಬೋಲ\u{cbf}ವಾರ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "볼리바르 주"), ("lt", "Bolivaro departamentas"), ("lv", "Bolivara departaments"), ("mr", "बोलिवर विभाग"), ("ms", "Pentadbiran Bolívar"), ("nb", "Bolívar"), ("nl", "Bolívar"), ("no", "Bolívar"), ("pl", "Bolívar"), ("pt", "Bolívar"), ("ro", "Departamentul Bolívar"), ("ru", "Боливар"), ("si", "බොල\u{dd2}වර\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Bolívar"), ("ta", "பொலிவ\u{bbe}ர\u{bcd} துறை"), ("te", "బ\u{c4b}ల\u{c3f}వర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โบล\u{e35}วาร\u{e4c}"), ("tr", "Bolívar"), ("uk", "Болівар"), ("ur", "بولیوار محکمہ"), ("vi", "Bolívar"), ("yo", "Bolívar Department"), ("yo_BJ", "Bolívar Department"), ("zh", "玻利瓦尔省")]),
                        unofficial_name_list: ["Bolívar"].to_vec(),
                    }
                ),
                (
                    "BOY",
                    Subdivision{
                        name: "Boyacá",
                        country_alpha2: Alpha2::CO,
                        code: "BOY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.45), longitude: Some(-73.35), max_latitude: Some(5.481), min_latitude: Some(5.40465), max_longitude: Some(-73.33131999999999), min_longitude: Some(-73.43772)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بوياكا"), ("az", "Boyaka"), ("bg", "Бояка"), ("bn", "বয\u{9bc}\u{9be}ক\u{9be} বিভ\u{9be}গ"), ("ca", "Boyacá"), ("ccp", "𑄝\u{11127}𑄠𑄇"), ("ceb", "Departamento de Boyacá"), ("cs", "Boyacá"), ("da", "Boyacá Department"), ("de", "Departamento de Boyacá"), ("el", "Μπογιακά"), ("en", "Boyacá"), ("es", "Boyacá"), ("eu", "Boyacá"), ("fa", "شهرستان بویاکا"), ("fi", "Boyacá"), ("fr", "Boyacá"), ("ga", "Boyacá"), ("gl", "Boyacá"), ("gu", "બોયાકા વિભાગ"), ("he", "בויאקה"), ("hi", "बोयाका विभाग"), ("hr", "Boyacá"), ("hu", "Boyacá megye"), ("id", "Departemen Boyacá"), ("it", "dipartimento di Boyacá"), ("ja", "ボヤカ県"), ("ka", "ბოიაკის დეპარტამენტი"), ("kn", "ಬಾಯ\u{ccd}ಕಾ ಇಲಾಖ\u{cc6}"), ("ko", "보야카 주"), ("lt", "Bojakos departamentas"), ("lv", "Bojasas departaments"), ("mr", "बॉयका विभाग"), ("ms", "Pentadbiran Boyacá"), ("nb", "Boyacá"), ("nl", "Boyacá"), ("no", "Boyacá"), ("pl", "Boyacá"), ("pt", "Boyacá"), ("ro", "Departamentul Boyacá"), ("ru", "Бояка"), ("si", "බොයක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Boyacá"), ("ta", "போயக\u{bcd}க\u{bbe} துறை"), ("te", "బ\u{c4b}య\u{c3e}క\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโบยากา"), ("tr", "Boyacá"), ("uk", "Бояка"), ("ur", "بویاکا محکمہ"), ("vi", "Khu vực hành chính Boyacá"), ("yo", "Boyacá Department"), ("yo_BJ", "Boyacá Department"), ("zh", "博亚卡省")]),
                        unofficial_name_list: ["Boyacá"].to_vec(),
                    }
                ),
                (
                    "CAL",
                    Subdivision{
                        name: "Caldas",
                        country_alpha2: Alpha2::CO,
                        code: "CAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.4815477), longitude: Some(-75.21144079999999), max_latitude: Some(5.4854088), min_latitude: Some(5.4777194), max_longitude: Some(-75.2068233), min_longitude: Some(-75.21738049999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كالداس"), ("az", "Kaldas"), ("bg", "Калдас"), ("bn", "ক\u{9be}লড\u{9be}স বিভ\u{9be}গ"), ("ca", "Departament de Caldas"), ("ccp", "𑄇𑄣\u{11134}𑄓𑄌\u{11134}"), ("ceb", "Departamento de Caldas"), ("cs", "Caldas"), ("da", "Caldas Department"), ("de", "Departamento de Caldas"), ("el", "Διαμέρισμα Κάλδα"), ("en", "Caldas"), ("es", "Caldas"), ("eu", "Caldas"), ("fa", "شهرستان کالداس"), ("fi", "Caldas"), ("fr", "Caldas"), ("gl", "Caldas"), ("gu", "કાલ\u{acd}ડસ વિભાગ"), ("he", "קלדס"), ("hi", "क\u{948}लडस विभाग"), ("hr", "Caldas"), ("hu", "Caldas megye"), ("id", "Departemen Caldas"), ("it", "dipartimento di Caldas"), ("ja", "カルダス県"), ("ka", "კალდასის დეპარტამენტი"), ("kn", "ಕ\u{ccd}ಯಾಲ\u{ccd}ಡಾಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "칼다스 주"), ("lt", "Kaldaso departamentas"), ("lv", "Kaldasas departaments"), ("mr", "क\u{945}लडस विभाग"), ("ms", "Pentadbiran Caldas"), ("nb", "Caldas"), ("nl", "Caldas"), ("no", "Caldas"), ("pl", "Caldas"), ("pt", "Caldas"), ("ro", "Departamentul Caldas"), ("ru", "Кальдас"), ("si", "කල\u{dca}ඩ\u{dcf}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Caldas"), ("ta", "க\u{bbe}ல\u{bcd}டஸ\u{bcd} துறை"), ("te", "క\u{c3e}ల\u{c4d}డ\u{c3e}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ดาลดาส ด\u{e35}พาทเม\u{e49}น"), ("tr", "Caldas"), ("uk", "Кальдас"), ("ur", "کالداس محکمہ"), ("vi", "Caldas"), ("yo", "Caldas Department"), ("yo_BJ", "Caldas Department"), ("zh", "卡尔达斯省")]),
                        unofficial_name_list: ["Caldas"].to_vec(),
                    }
                ),
                (
                    "CAQ",
                    Subdivision{
                        name: "Caquetá",
                        country_alpha2: Alpha2::CO,
                        code: "CAQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.869892), longitude: Some(-73.8419063), max_latitude: Some(2.9332), min_latitude: Some(-0.7065199999999999), max_longitude: Some(-71.32021999999999), min_longitude: Some(-76.30578)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كاكيتا"), ("bg", "Какета"), ("bn", "ক\u{9be}ক\u{9c1}য\u{9bc}েট\u{9be} বিভ\u{9be}গ"), ("ca", "Caquetá"), ("ccp", "𑄇𑄬𑄇\u{1112a}𑄠𑄬𑄑"), ("ceb", "Departamento del Caquetá"), ("cs", "Caquetá"), ("da", "Caquetá Department"), ("de", "Departamento de Caquetá"), ("el", "Κακουετά"), ("en", "Caquetá"), ("es", "Caquetá"), ("eu", "Caquetá"), ("fa", "شهرستان کاکئتا"), ("fi", "Caquetá"), ("fr", "Caquetá"), ("gl", "Caquetá"), ("gu", "ક\u{ac7}ક\u{acd}વ\u{ac7}ટા વિભાગ"), ("he", "קקטה"), ("hi", "क\u{948}क\u{947}टा विभाग"), ("hr", "Caquetá"), ("hu", "Caquetá megye"), ("id", "Departemen Caquetá"), ("it", "dipartimento di Caquetá"), ("ja", "カケタ県"), ("ka", "კაკეტის დეპარტამენტი"), ("kn", "ಕಾಕ\u{ccd}ವ\u{cc6}ಟಾ ಇಲಾಖ\u{cc6}"), ("ko", "카케타 주"), ("lt", "Kaketos departamentas"), ("lv", "Kaketas departaments"), ("mr", "क\u{945}काटा विभाग"), ("ms", "Pentadbiran Caquetá"), ("nb", "Caquetá"), ("nl", "Caquetá"), ("no", "Caquetá"), ("pl", "Caquetá"), ("pt", "Caquetá"), ("ro", "Departamentul Caquetá"), ("ru", "Какета"), ("si", "කක\u{dca}වෙට\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Caquetá"), ("ta", "சயூட\u{bbe} துறை"), ("te", "క\u{c3e}క\u{c4d}వ\u{c46}ట\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "คาควอต\u{e49}า ด\u{e34}พาร\u{e4c}ทเมนต\u{e4c}"), ("tr", "Caquetá"), ("uk", "Какета"), ("ur", "کاکیتا محکمہ"), ("vi", "Caquetá"), ("yo", "Caquetá Department"), ("yo_BJ", "Caquetá Department"), ("zh", "卡克塔省")]),
                        unofficial_name_list: ["Caquetá"].to_vec(),
                    }
                ),
                (
                    "CAS",
                    Subdivision{
                        name: "Casanare",
                        country_alpha2: Alpha2::CO,
                        code: "CAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.7589269), longitude: Some(-71.5723953), max_latitude: Some(6.30716), min_latitude: Some(4.286), max_longitude: Some(-69.8356), min_longitude: Some(-73.07694)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كاساناري"), ("bg", "Касанаре"), ("bn", "ক\u{9be}স\u{9be}নরে বিভ\u{9be}গ"), ("ca", "Casanare"), ("ccp", "𑄇𑄥𑄚𑄢𑄬"), ("ceb", "Departamento de Casanare"), ("cs", "Casanare"), ("da", "Casanare Department"), ("de", "Departamento de Casanare"), ("el", "Κασανάρε"), ("en", "Casanare"), ("es", "Casanare"), ("eu", "Casanare"), ("fa", "شهرستان کاساناره"), ("fi", "Casanare"), ("fr", "Casanare"), ("gl", "Casanare"), ("gu", "કાસન\u{ac7}ર વિભાગ"), ("he", "קסנרה"), ("hi", "कासन\u{947}र\u{947} विभाग"), ("hr", "Casanare"), ("hu", "Casanare megye"), ("id", "Departemen Casanare"), ("it", "dipartimento di Casanare"), ("ja", "カサナレ県"), ("ka", "კასანარეს დეპარტამენტი"), ("kn", "ಕ\u{ccd}ಯಾಸನಾರ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "카사나레 주"), ("lt", "Kasanarės departamentas"), ("lv", "Kasanares departaments"), ("mr", "कासन\u{947}र विभाग"), ("ms", "Pentadbiran Casanare"), ("nb", "Casanare"), ("nl", "Casanare"), ("no", "Casanare"), ("pl", "Casanare"), ("pt", "Casanare"), ("ru", "Касанаре"), ("si", "කසනේය\u{dcf}ර\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Casanare"), ("ta", "கேசன\u{bbe}ரே துறை"), ("te", "క\u{c3e}సన\u{c3e}ర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "คาซาแนร\u{e4c}"), ("tr", "Casanare"), ("uk", "Касанаре"), ("ur", "کاسانارے محکمہ"), ("vi", "Casanare"), ("yo", "Casanare Department"), ("yo_BJ", "Casanare Department"), ("zh", "卡萨纳雷省")]),
                        unofficial_name_list: ["Casanare"].to_vec(),
                    }
                ),
                (
                    "CAU",
                    Subdivision{
                        name: "Cauca",
                        country_alpha2: Alpha2::CO,
                        code: "CAU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.5725523), longitude: Some(-76.7783748), max_latitude: Some(2.5752548), min_latitude: Some(2.5690383), max_longitude: Some(-76.7718601), min_longitude: Some(-76.78168769999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كاوكا"), ("be", "дэпартамент Каўка"), ("bg", "Каука"), ("bn", "ক\u{9be}উক\u{9be} বিভ\u{9be}গ"), ("ca", "Departament del Cauca"), ("ccp", "𑄇\u{11127}𑄅\u{1112a}𑄇"), ("ceb", "Departamento del Cauca"), ("cs", "Cauca"), ("da", "Cauca Department"), ("de", "Departamento de Cauca"), ("el", "Διαμέρισμα Κάουκας"), ("en", "Cauca"), ("es", "Cauca"), ("eu", "Cauca"), ("fa", "شهرستان کائوکا"), ("fi", "Cauca"), ("fr", "Cauca"), ("gl", "Cauca"), ("gu", "કોઉકા વિભાગ"), ("he", "קאוקה"), ("hi", "काऊका विभाग"), ("hr", "Cauca"), ("hu", "Cauca megye"), ("hy", "Կաուկա"), ("id", "Departemen Cauca"), ("it", "dipartimento di Cauca"), ("ja", "カウカ県"), ("ka", "კაუკის დეპარტამენტი"), ("kn", "ಕ\u{ccc}ಕಾ ಇಲಾಖ\u{cc6}"), ("ko", "카우카 주"), ("lt", "Kaukos departamentas"), ("lv", "Kaukas departaments"), ("mr", "काऊच\u{902} विभाग"), ("ms", "Pentadbiran Cauca"), ("nb", "Cauca"), ("nl", "Cauca"), ("no", "Cauca"), ("pl", "Cauca"), ("pt", "Cauca"), ("ro", "Departamentul Cauca"), ("ru", "Каука"), ("si", "කව\u{dd4}ක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Cauca"), ("ta", "கவுக\u{bcd}க துறை"), ("te", "క\u{c4c}క\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เคาคา ด\u{e35}พาทเม\u{e49}น"), ("tr", "Cauca"), ("uk", "Каука"), ("ur", "کاؤکا محکمہ"), ("vi", "Cauca"), ("yo", "Cauca Department"), ("yo_BJ", "Cauca Department"), ("zh", "考卡省")]),
                        unofficial_name_list: ["Cauca"].to_vec(),
                    }
                ),
                (
                    "CES",
                    Subdivision{
                        name: "Cesar",
                        country_alpha2: Alpha2::CO,
                        code: "CES",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.3372948), longitude: Some(-73.65362089999999), max_latitude: Some(10.86722), min_latitude: Some(7.676399999999999), max_longitude: Some(-72.88257), min_longitude: Some(-74.13982)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سيزار"), ("az", "Sesar (departament)"), ("bg", "Сесар"), ("bn", "সিজ\u{9be}র বিভ\u{9be}গ"), ("ca", "Departament del Cesar"), ("ccp", "𑄥𑄬𑄥𑄢\u{11134}"), ("ceb", "Departamento del Cesar"), ("cs", "Cesar"), ("da", "Cesar Department"), ("de", "Departamento del Cesar"), ("el", "Κέσαρ"), ("en", "Cesar"), ("es", "Cesar"), ("eu", "Cesar"), ("fa", "شهرستان سزار"), ("fi", "Cesar"), ("fr", "Cesar"), ("gl", "Cesar"), ("gu", "સીઝર વિભાગ"), ("he", "ססאר"), ("hi", "सीसर विभाग"), ("hr", "Cesar"), ("hu", "Cesar megye"), ("id", "Departemen Cesar"), ("it", "dipartimento di Cesar"), ("ja", "セサール県"), ("ka", "სესარის დეპარტამენტი"), ("kn", "ಸೀಜರ ಇಲಾಖ\u{cc6}"), ("ko", "세사르 주"), ("lt", "Sesaro departamentas"), ("lv", "Sesaras departaments"), ("mr", "सीझर विभाग"), ("ms", "Pentadbiran Cesar"), ("nb", "Cesar"), ("nl", "Cesar"), ("no", "Cesar"), ("pl", "Cesar"), ("pt", "Cesar"), ("ru", "Сесар"), ("si", "ස\u{dd3}සර\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Cesar"), ("ta", "ச\u{bc0}சர\u{bcd} துறை"), ("te", "స\u{c40}సర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ซ\u{e35}ซาร\u{e4c}"), ("tr", "Cesar"), ("uk", "Сесар"), ("ur", "سیزار محکمہ"), ("vi", "Khu vực hành chính Cesar"), ("yo", "Cesar Department"), ("yo_BJ", "Cesar Department"), ("zh", "塞萨尔省")]),
                        unofficial_name_list: ["Cesar"].to_vec(),
                    }
                ),
                (
                    "CHO",
                    Subdivision{
                        name: "Chocó",
                        country_alpha2: Alpha2::CO,
                        code: "CHO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.2528033), longitude: Some(-76.8259652), max_latitude: Some(8.67598), min_latitude: Some(3.96467), max_longitude: Some(-76.0026), min_longitude: Some(-77.88727999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة تشوكو"), ("be", "дэпартамент Чако"), ("bg", "Чоко"), ("bn", "ক\u{9c1}কো বিভ\u{9be}গ"), ("ca", "Chocó"), ("ccp", "𑄌\u{1112e}𑄇\u{1112e}"), ("ceb", "Departamento del Chocó"), ("cs", "Chocó"), ("da", "Chocó Department"), ("de", "Departamento del Chocó"), ("el", "Τσοκό"), ("en", "Chocó"), ("es", "Chocó"), ("et", "Chocó departemang"), ("eu", "Chocó"), ("fa", "شهرستان چوکو"), ("fi", "Chocó"), ("fr", "Chocó"), ("gl", "Chocó"), ("gu", "ચોકો વિભાગ"), ("he", "צ׳וקו"), ("hi", "चोको विभाग"), ("hr", "Chocó"), ("hu", "Chocó megye"), ("hy", "Չոկո"), ("id", "Departemen Chocó"), ("it", "dipartimento di Chocó"), ("ja", "チョコ県"), ("ka", "ჩოკოს დეპარტამენტი"), ("kn", "ಚೊಕೊ ಇಲಾಖ\u{cc6}"), ("ko", "초코 주"), ("lt", "Čioko departamentas"), ("lv", "Čoko departaments"), ("mr", "चोको विभाग"), ("ms", "Pentadbiran Chocó"), ("nb", "Chocó"), ("nl", "Chocó"), ("no", "Chocó"), ("pl", "Chocó"), ("pt", "Chocó"), ("ru", "Чоко"), ("si", "චෝකෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Chocó"), ("ta", "ச\u{bbe}கோ துறை"), ("te", "చ\u{c4b}క\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโชโก"), ("tr", "Chocó"), ("uk", "Чоко"), ("ur", "چوکو محکمہ"), ("vi", "Chocó"), ("yo", "Chocó Department"), ("yo_BJ", "Chocó Department"), ("zh", "乔科省")]),
                        unofficial_name_list: ["Chocó"].to_vec(),
                    }
                ),
                (
                    "COR",
                    Subdivision{
                        name: "Córdoba",
                        country_alpha2: Alpha2::CO,
                        code: "COR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.4029253), longitude: Some(-75.89986739999999), max_latitude: Some(9.447802399999999), min_latitude: Some(7.347090000000001), max_longitude: Some(-74.78514), min_longitude: Some(-76.50993)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة قرطبة"), ("bg", "Кордоба"), ("bn", "কর\u{9cd}ডোব\u{9be} বিভ\u{9be}গ"), ("ca", "Departament de Córdoba"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄓\u{1112e}𑄝"), ("ceb", "Departamento de Córdoba"), ("cs", "Córdoba"), ("da", "Córdoba Department"), ("de", "Departamento de Córdoba"), ("el", "Διαμέρισμα Κόρδοβα"), ("en", "Córdoba"), ("es", "Córdoba"), ("et", "Córdoba departemang"), ("eu", "Córdoba"), ("fa", "شهرستان کوردوبا"), ("fi", "Córdoba"), ("fr", "Córdoba"), ("gl", "Córdoba"), ("gu", "કોર\u{acd}ડોબા વિભાગ"), ("he", "קורדובה (קולומביה)"), ("hi", "कॉर\u{94d}दोबा विभाग"), ("hr", "Córdoba"), ("hu", "Córdoba megye"), ("id", "Departemen Córdoba"), ("it", "Córdoba"), ("ja", "コルドバ県"), ("ka", "კორდობის დეპარტამენტი"), ("kn", "ಕೊರ\u{ccd}ಡೊಬಾ ಇಲಾಖ\u{cc6}"), ("ko", "코르도바 주"), ("lt", "Kordobos departamentas"), ("lv", "Kordobas departaments"), ("mr", "कोर\u{94d}डोबा विभाग"), ("ms", "Pentadbiran Córdoba"), ("nb", "Córdoba"), ("nl", "Córdoba"), ("no", "Córdoba"), ("pl", "Córdoba"), ("pt", "Córdoba (departamento)"), ("ru", "Кордова"), ("si", "කොර\u{dca}ඩොබ\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Córdoba"), ("ta", "கோர\u{bcd}டோப\u{bbe} துறை"), ("te", "క\u{c4b}ర\u{c4d}డ\u{c4b}బ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคอร\u{e4c}โดบา"), ("tr", "Córdoba"), ("uk", "Кордова"), ("ur", "کوردوبا محکمہ"), ("vi", "Khu vực hành chính Cordoba"), ("yo", "Córdoba Department"), ("yo_BJ", "Córdoba Department"), ("zh", "科爾多瓦省 (哥倫比亞)")]),
                        unofficial_name_list: ["Córdoba"].to_vec(),
                    }
                ),
                (
                    "CUN",
                    Subdivision{
                        name: "Cundinamarca",
                        country_alpha2: Alpha2::CO,
                        code: "CUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.026002999999999), longitude: Some(-74.0300122), max_latitude: Some(5.8367), min_latitude: Some(3.7269), max_longitude: Some(-73.05084), min_longitude: Some(-74.89067)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كونديناماركا"), ("bg", "Кундинамарка"), ("bn", "ক\u{9c1}ন\u{9cd}ডিন\u{9be}ম\u{9be}ক\u{9be} বিভ\u{9be}গ"), ("ca", "Cundinamarca"), ("ccp", "𑄇\u{1112a}𑄚\u{11134}𑄓\u{11128}𑄚𑄟𑄢\u{11134}𑄇"), ("ceb", "Departamento de Cundinamarca"), ("cs", "Cundinamarca"), ("da", "Cundinamarca Department"), ("de", "Cundinamarca"), ("el", "Διαμέρισμα Κουντιναμάρκα"), ("en", "Cundinamarca"), ("es", "Cundinamarca"), ("eu", "Cundinamarca"), ("fa", "شهرستان کوندینامارکا"), ("fi", "Cundinamarca"), ("fr", "Cundinamarca"), ("gl", "Cundinamarca"), ("gu", "ક\u{ac1}ન\u{acd}ડિનામાર\u{acd}કા વિભાગ"), ("he", "קונדינמרקה"), ("hi", "क\u{941}\u{902}डीनामार\u{94d}का विभाग"), ("hr", "Cundinamarca"), ("hu", "Cundinamarca megye"), ("id", "Departemen Kundinamarka"), ("it", "dipartimento di Cundinamarca"), ("ja", "クンディナマルカ県"), ("ka", "კუნდინამარკის დეპარტამენტი"), ("kn", "ಕುಂಡ\u{cbf}ನಮಾರ\u{ccd}ಕ ಇಲಾಖ\u{cc6}"), ("ko", "쿤디나마르카 주"), ("lt", "Kundinamarkos departamentas"), ("lv", "Kundinamarkas departaments"), ("mr", "क\u{941}\u{902}डीनमरका विभाग"), ("ms", "Pentadbiran Kundinamarka"), ("nb", "Cundinamarca"), ("nl", "Cundinamarca"), ("no", "Cundinamarca"), ("pl", "Cundinamarca"), ("pt", "Cundinamarca"), ("ru", "Кундинамарка"), ("si", "ක\u{dd4}න\u{dca}ඩ\u{dd2}න\u{dcf}මර\u{dca}ක\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sk", "Cundinamarca"), ("sv", "Cundinamarca"), ("ta", "சுண\u{bcd}டினம\u{bbe}ர\u{bcd}க\u{bcd}க துறை"), ("te", "కుండ\u{c3f}న\u{c3e}మ\u{c3e}ర\u{c4d}స\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e38}นด\u{e35}นามาร\u{e4c}กา"), ("tr", "Cundinamarca"), ("uk", "Кундінамарка"), ("ur", "کوندینامارکا محکمہ"), ("vi", "Cundinamarca"), ("yo", "Cundinamarca Department"), ("yo_BJ", "Cundinamarca Department"), ("zh", "昆迪納馬卡省")]),
                        unofficial_name_list: ["Cundinamarca"].to_vec(),
                    }
                ),
                (
                    "DC",
                    Subdivision{
                        name: "Distrito Capital de Bogotá",
                        country_alpha2: Alpha2::CO,
                        code: "DC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.598056000000001), longitude: Some(-74.075833), max_latitude: Some(4.8371), min_latitude: Some(3.72977), max_longitude: Some(-73.99631), min_longitude: Some(-74.45177)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bogotá, DC"), ("am", "ቦጎታ"), ("ar", "بوغوتا"), ("az", "Boqota"), ("be", "Багата"), ("bg", "Богота"), ("bn", "বোগোত\u{9be}"), ("bs", "Bogotá"), ("ca", "Bogotà"), ("ccp", "𑄢𑄌\u{11134}𑄙𑄚\u{11129} 𑄎𑄬𑄣"), ("ceb", "Bogotá"), ("cs", "Bogotá"), ("cy", "Bogotá"), ("da", "Bogotá"), ("de", "Bogotá"), ("el", "Μπογκοτά"), ("en", "Capital District"), ("es", "Bogotá"), ("et", "Bogotá"), ("eu", "Bogotá"), ("fa", "بوگوتا"), ("fi", "Bogotá"), ("fr", "Bogota"), ("ga", "Bogotá"), ("gl", "Bogotá"), ("gu", "બોગોટા"), ("he", "בוגוטה"), ("hi", "बोगोटा"), ("hr", "Bogotá"), ("hu", "Bogotá"), ("hy", "Բոգոտա"), ("id", "Bogotá"), ("is", "Bógóta"), ("it", "Bogotá, DC"), ("ja", "ボゴタ"), ("jv", "Bogotá"), ("ka", "ბოგოტა"), ("kk", "Богота"), ("km", "ទ\u{17b8}ក\u{17d2}រ\u{17bb}ងប\u{17bc}ក\u{17bc}តា"), ("kn", "ಬೊಗೋಟ"), ("ko", "보고타"), ("lt", "Bogota"), ("lv", "Bogota"), ("mk", "Богота"), ("ml", "ബൊഗോട\u{d4d}ട"), ("mn", "Богота"), ("mr", "बोगोता"), ("ms", "Bogotá"), ("my", "ဘ\u{102d}\u{102f}ဂ\u{102d}\u{102f}တာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Bogotá"), ("nl", "Bogota"), ("no", "Bogotá"), ("or", "ବୋଗୋଟ\u{b3e}"), ("pa", "ਬ\u{a4b}ਗ\u{a4b}ਤਾ"), ("pl", "Bogota"), ("pt", "Bogotá, DC"), ("ro", "Bogotá"), ("ru", "Богота"), ("si", "බොගෝට\u{dcf}"), ("sk", "Bogota"), ("sl", "Bogota"), ("so", "Bogota"), ("sq", "Bogotá"), ("sr", "Богота"), ("sr_Latn", "Bogota"), ("sv", "Bogotá"), ("sw", "Bogota"), ("ta", "பொகோட\u{bcd}ட\u{bbe}"), ("te", "బ\u{c4b}గ\u{c4b}ట\u{c3e}"), ("th", "โบโกตา"), ("tk", "Bogota"), ("tr", "Bogotá"), ("uk", "Богота"), ("ur", "بوگوتا"), ("uz", "Bogota"), ("vi", "Bogotá"), ("yo", "Bogotá"), ("yo_BJ", "Bogotá"), ("yue", "波哥大"), ("yue_Hans", "波哥大"), ("zh", "波哥大"), ("zu", "IBogotaa")]),
                        unofficial_name_list: ["Santafé de Bogotá Distrito Capital"].to_vec(),
                    }
                ),
                (
                    "GUA",
                    Subdivision{
                        name: "Guainía",
                        country_alpha2: Alpha2::CO,
                        code: "GUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.585393), longitude: Some(-68.52471489999999), max_latitude: Some(4.04741), min_latitude: Some(1.16702), max_longitude: Some(-66.84722), min_longitude: Some(-70.9425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة غواينيا"), ("bg", "Гуайния"), ("bn", "গোয\u{9bc}\u{9be}ইনিয\u{9bc}\u{9be} বিভ\u{9be}গ"), ("ca", "Guainía"), ("ccp", "𑄉\u{1112d}\u{1112a}𑄃\u{11128}𑄚\u{11128}𑄠"), ("ceb", "Departamento del Guainía"), ("da", "Guainía Department"), ("de", "Guainía"), ("el", "Γκουαϊνία"), ("en", "Guainía"), ("es", "Guainía"), ("eu", "Guainía"), ("fa", "شهرستان گواینیا"), ("fi", "Guainía"), ("fr", "Guainía"), ("gl", "Guainía"), ("gu", "ગ\u{acd}ય\u{ac1}એનિઆ વિભાગ"), ("he", "גואאיניה"), ("hi", "ग\u{941}ऐनिआ विभाग"), ("hr", "Guainía"), ("hu", "Guainía megye"), ("id", "Departemen Guainía"), ("it", "dipartimento di Guainía"), ("ja", "グアイニア県"), ("ka", "გუაინიის დეპარტამენტი"), ("kn", "ಗ\u{cbf}ನ\u{cbf}ಯಾ ಇಲಾಖ\u{cc6}"), ("ko", "과이니아 주"), ("lt", "Guainijos departamentas"), ("lv", "Gvainijas departaments"), ("mr", "ग\u{94d}व\u{947}निआ विभाग"), ("ms", "Pentadbiran Guainía"), ("nb", "Guainía"), ("nl", "Guainía"), ("no", "Guainía"), ("pl", "Guainía"), ("pt", "Guainía"), ("ru", "Гуайния"), ("si", "ගය\u{dd2}න\u{dd2}ය\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Guainía"), ("ta", "குயினிய\u{bbe} துறை"), ("te", "గ\u{c4d}వ\u{c46}య\u{c3f}న\u{c3f}య\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องก\u{e31}วเน\u{e35}ย"), ("tr", "Guainía"), ("uk", "Ґуайнія"), ("ur", "گواینیا محکمہ"), ("vi", "Khu hành chính Guainía"), ("yo", "Guainía Department"), ("yo_BJ", "Guainía Department"), ("zh", "瓜伊尼亚省")]),
                        unofficial_name_list: ["Guainía"].to_vec(),
                    }
                ),
                (
                    "GUV",
                    Subdivision{
                        name: "Guaviare",
                        country_alpha2: Alpha2::CO,
                        code: "GUV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.0619181), longitude: Some(-73.2558952), max_latitude: Some(1.0722202), min_latitude: Some(1.051796), max_longitude: Some(-73.24516299999999), min_longitude: Some(-73.27159879999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة غوافياري"), ("az", "Quavyare (departament)"), ("bg", "Гуавиаре"), ("bn", "গোয\u{9bc}\u{9be}ভিরে বিভ\u{9be}গ"), ("ca", "Guaviare"), ("ccp", "𑄉\u{1112a}𑄠𑄞𑄠𑄢\u{11134}"), ("ceb", "Departamento del Guaviare"), ("da", "Guaviare Department"), ("de", "Guaviare"), ("el", "Γκουαβιάρε"), ("en", "Guaviare"), ("es", "Guaviare"), ("eu", "Guaviare"), ("fa", "شهرستان گواویره"), ("fi", "Guaviare"), ("fr", "Guaviare"), ("gl", "Guaviare"), ("gu", "ગ\u{ac1}આવિર\u{ac7} વિભાગ"), ("he", "גואביארה"), ("hi", "ग\u{94d}वाविआर\u{947} विभाग"), ("hr", "Guaviare"), ("hu", "Guaviare megye"), ("id", "Departemen Guaviare"), ("it", "dipartimento di Guaviare"), ("ja", "グアビアーレ県"), ("ka", "გუავიარეს დეპარტამენტი"), ("kn", "ಗುವಾವ\u{cbf}ರ\u{cc6} ಇಲಾಖ\u{cc6}"), ("ko", "과비아레 주"), ("lt", "Guavjarės departamentas"), ("lv", "Guavjares departaments"), ("mr", "ग\u{941}आविर\u{947} विभाग"), ("ms", "Pentadbiran Guaviare"), ("nb", "Guaviare"), ("nl", "Guaviare"), ("no", "Guaviare"), ("pl", "Guaviare"), ("pt", "Guaviare"), ("ru", "Гуавьяре"), ("si", "ගව\u{dd2}යරේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Guaviare"), ("ta", "குவைய\u{bbe}ரே துறை"), ("te", "గ\u{c4d}వ\u{c3e}వ\u{c47}ర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ก\u{e31}วเว\u{e35}ยเร\u{e48} ด\u{e35}พาทเม\u{e49}น"), ("tr", "Guaviare"), ("uk", "Ґуавʼяре"), ("ur", "گوابیارے محکمہ"), ("vi", "Khu vực hành chính Guaviare"), ("yo", "Guaviare Department"), ("yo_BJ", "Guaviare Department"), ("zh", "瓜维亚雷省")]),
                        unofficial_name_list: ["Guaviare"].to_vec(),
                    }
                ),
                (
                    "HUI",
                    Subdivision{
                        name: "Huila",
                        country_alpha2: Alpha2::CO,
                        code: "HUI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.5359349), longitude: Some(-75.52766989999999), max_latitude: Some(3.84485), min_latitude: Some(1.55547), max_longitude: Some(-74.41284999999999), min_longitude: Some(-76.62425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة هويلا"), ("az", "Uila (departament)"), ("bg", "Уила"), ("bn", "হ\u{9c1}ইল\u{9be} বিভ\u{9be}গ"), ("ca", "Huila"), ("ccp", "𑄦\u{1112d}\u{1112a}𑄣"), ("ceb", "Departamento del Huila"), ("da", "Huila Department"), ("de", "Departamento de Huila"), ("el", "Χουίλα"), ("en", "Huila"), ("es", "Huila"), ("eu", "Huila"), ("fa", "شهرستان هویلا"), ("fi", "Huila"), ("fr", "Huila"), ("gl", "Huila"), ("gu", "હ\u{ac1}ઈલા વિભાગ"), ("he", "וילה (מחוז)"), ("hi", "ह\u{941}इला विभाग"), ("hr", "Huila"), ("hu", "Huila megye"), ("id", "Departemen Huila"), ("it", "dipartimento di Huila"), ("ja", "ウイラ県"), ("ka", "უილის დეპარტამენტი"), ("kn", "ಹುಯ\u{cbf}ಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "우일라 주"), ("lt", "Uilos departamentas"), ("lv", "Uilas departaments"), ("mr", "ह\u{941}ईला विभाग"), ("ms", "Pentadbiran Huila"), ("nb", "Huila"), ("nl", "Huila"), ("no", "Huila"), ("pl", "Huila"), ("pt", "Huila"), ("ru", "Уила"), ("si", "හ\u{dd4}ය\u{dd2}ල\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Huila"), ("ta", "ஹுஇல\u{bbe} துறை"), ("te", "హుయ\u{c3f}ల\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ฮ\u{e38}ยลา ด\u{e35}พาทเม\u{e49}น"), ("tr", "Huila"), ("uk", "Уїла"), ("ur", "ہوئلا محکمہ"), ("vi", "Khu vực hành chính Huila"), ("yo", "Huila Department"), ("yo_BJ", "Huila Department"), ("zh", "乌伊拉省")]),
                        unofficial_name_list: ["Huila"].to_vec(),
                    }
                ),
                (
                    "LAG",
                    Subdivision{
                        name: "La Guajira",
                        country_alpha2: Alpha2::CO,
                        code: "LAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.3547743), longitude: Some(-72.5204827), max_latitude: Some(12.4584622), min_latitude: Some(10.39699), max_longitude: Some(-71.1131716), min_longitude: Some(-73.66418999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة لا غواخيرا"), ("bg", "Ла Гуахира"), ("bn", "ল\u{9be} গোয\u{9bc}\u{9be}জির\u{9be} বিভ\u{9be}গ"), ("ca", "La Guajira"), ("ccp", "𑄣 𑄉\u{1112a}𑄠𑄎\u{11128}𑄢"), ("ceb", "Departamento de La Guajira"), ("da", "La Guajira Department"), ("de", "La Guajira"), ("el", "Λα Γκουατζίρα"), ("en", "La Guajira"), ("es", "La Guajira"), ("eu", "La Guajira"), ("fa", "شهرستان لا گواخیرا"), ("fi", "Guajira"), ("fr", "La Guajira"), ("gl", "La Guajira"), ("gu", "લા ગ\u{ac1}જિરા વિભાગ"), ("he", "לה גואחירה"), ("hi", "ला ग\u{94d}वाजिरा विभाग"), ("hr", "La Guajira"), ("hu", "La Guajira megye"), ("id", "Departemen Guajira"), ("it", "dipartimento di La Guajira"), ("ja", "ラ・グアヒーラ県"), ("ka", "ლა-გუახირის დეპარტამენტი"), ("kn", "ಲಾ ಗುಜ\u{cbf}ರಾ ಇಲಾಖ\u{cc6}"), ("ko", "라과히라 주"), ("lt", "Gvachiros departamentas"), ("lv", "Lagvahiras departaments"), ("mr", "ला ग\u{941}जिरा विभाग"), ("ms", "Pentadbiran Guajira"), ("nb", "La Guajira"), ("nl", "La Guajira"), ("no", "La Guajira"), ("pl", "La Guajira"), ("pt", "Guajira"), ("ru", "Гуахира"), ("si", "ල\u{dcf} ග\u{dd4}ආ ජ\u{dd2}ර\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "La Guajira"), ("ta", "ல\u{bbe} குஅஜ\u{bc0}ர\u{bbe} துறை"), ("te", "ల\u{c3e} గ\u{c4d}వ\u{c3e}జ\u{c3f}ర\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาก\u{e31}วฮ\u{e34}รา"), ("tr", "La Guajira"), ("uk", "Гуахіра"), ("ur", "لا گواجیرا دیپارٹمنٹ"), ("vi", "Khu vực hành chính La Guajira"), ("yo", "La Guajira Department"), ("yo_BJ", "La Guajira Department"), ("zh", "瓜希拉省")]),
                        unofficial_name_list: ["La Guajira"].to_vec(),
                    }
                ),
                (
                    "MAG",
                    Subdivision{
                        name: "Magdalena",
                        country_alpha2: Alpha2::CO,
                        code: "MAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.4113014), longitude: Some(-74.4056612), max_latitude: Some(11.3489296), min_latitude: Some(8.9183801), max_longitude: Some(-73.54199), min_longitude: Some(-74.94542299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ماجدالينا"), ("az", "Maqdalena (departament)"), ("be", "Магдалена"), ("bg", "Магдалена"), ("bn", "ম\u{9be}গড\u{9be}লেন\u{9be} বিভ\u{9be}গ"), ("ca", "Departament del Magdalena"), ("ccp", "𑄟𑄇\u{11134}𑄓𑄬𑄣𑄬𑄚"), ("ceb", "Departamento del Magdalena"), ("cs", "Magdalena"), ("da", "Magdalena Department"), ("de", "Departamento del Magdalena"), ("el", "Διαμέρισμα Μαγκνταλένας"), ("en", "Magdalena"), ("es", "Magdalena"), ("eu", "Magdalena"), ("fa", "شهرستان ماگدالنا"), ("fi", "Magdalena"), ("fr", "Magdalena"), ("gl", "Magdalena"), ("gu", "માગ\u{acd}દાલ\u{ac7}ના વિભાગ"), ("he", "מגדלנה"), ("hi", "म\u{947}गड\u{947}लिना विभाग"), ("hr", "Magdalena"), ("hu", "Magdalena megye"), ("hy", "Մագդալենա շրջան"), ("id", "Departemen Magdalena"), ("it", "dipartimento di Magdalena"), ("ja", "マグダレーナ県"), ("ka", "მაგდალენას დეპარტამენტი"), ("kn", "ಮ\u{ccd}ಯಾಗ\u{ccd}ಡಲೇನಾ ಇಲಾಖ\u{cc6}"), ("ko", "마그달레나 주"), ("lt", "Magdalenos departamentas"), ("lv", "Magdalenas departaments"), ("mr", "म\u{945}ग\u{94d}डाल\u{947}ना विभाग"), ("ms", "Pentadbiran Magdalena"), ("nb", "Magdalena"), ("nl", "Magdalena"), ("no", "Magdalena"), ("pl", "Magdalena"), ("pt", "Magdalena"), ("ro", "Departamentul Magdalena"), ("ru", "Магдалена"), ("si", "මගල\u{dca}දේන\u{dcf} දෙප\u{dcf}ර\u{dca}තමන\u{dca}ත\u{dd4}ව"), ("sr", "Магдалена регион"), ("sr_Latn", "Magdalena region"), ("sv", "Magdalena"), ("ta", "மக\u{bcd}த\u{bbe}லென துறை"), ("te", "మ\u{c3e}గ\u{c4d}డ\u{c3e}ల\u{c46}న\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ม\u{e31}กดาเลนา"), ("tr", "Magdalena"), ("uk", "Маґдалена"), ("ur", "ماگدالینا محکمہ"), ("vi", "Magdalena"), ("yo", "Magdalena Department"), ("yo_BJ", "Magdalena Department"), ("zh", "马格达莱纳省")]),
                        unofficial_name_list: ["Magdalena"].to_vec(),
                    }
                ),
                (
                    "MET",
                    Subdivision{
                        name: "Meta",
                        country_alpha2: Alpha2::CO,
                        code: "MET",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.2719904), longitude: Some(-73.087749), max_latitude: Some(4.92514), min_latitude: Some(1.60126), max_longitude: Some(-71.0774), min_longitude: Some(-74.93392)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ميتا"), ("az", "Meta (departament)"), ("bg", "Мета"), ("bn", "মেট\u{9be}"), ("ca", "Departament del Meta"), ("ccp", "𑄟𑄬𑄑"), ("ceb", "Departamento del Meta"), ("da", "Meta"), ("de", "Meta"), ("el", "Μέτα"), ("en", "Meta"), ("es", "Meta"), ("eu", "Meta"), ("fa", "شهرستان متا"), ("fi", "Meta"), ("fr", "Meta"), ("gl", "Meta"), ("gu", "મ\u{ac7}ટા"), ("he", "מטה (מחוז)"), ("hi", "म\u{947}टा"), ("hr", "Meta"), ("hu", "Meta megye"), ("id", "Departemen Meta"), ("it", "dipartimento di Meta"), ("ja", "メタ県"), ("ka", "მეტის დეპარტამენტი"), ("kn", "ಮ\u{cc6}ಟಾ"), ("ko", "메타 주"), ("lt", "Metos departamentas"), ("lv", "Metas departaments"), ("mr", "म\u{947}टा"), ("ms", "Pentadbiran Meta"), ("nb", "Meta"), ("nl", "Meta"), ("no", "Meta"), ("pl", "Meta"), ("pt", "Meta"), ("ru", "Ме\u{301}та"), ("si", "මෙට\u{dcf}"), ("sv", "Meta"), ("ta", "மெட\u{bcd}ட\u{bbe}"), ("te", "మ\u{c46}ట\u{c3e}"), ("th", "เมตา"), ("tr", "Meta"), ("uk", "Мета"), ("ur", "میتا محکمہ"), ("vi", "Meta"), ("yo", "Meta Department"), ("yo_BJ", "Meta Department"), ("zh", "梅塔省")]),
                        unofficial_name_list: ["Meta"].to_vec(),
                    }
                ),
                (
                    "NAR",
                    Subdivision{
                        name: "Nariño",
                        country_alpha2: Alpha2::CO,
                        code: "NAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.289822), longitude: Some(-77.35861190000001), max_latitude: Some(1.35415), min_latitude: Some(1.22328), max_longitude: Some(-77.30888999999999), min_longitude: Some(-77.38336)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة نارينيو"), ("az", "Narino"), ("bg", "Нариньо"), ("bn", "ন\u{9be}রিনো বিভ\u{9be}গ"), ("ca", "Nariño"), ("ccp", "𑄚𑄢\u{11128}𑄚\u{1112e}"), ("ceb", "Departamento de Nariño"), ("da", "Narino Department"), ("de", "Departamento de Nariño"), ("el", "Ναρίνο"), ("en", "Nariño"), ("es", "Nariño"), ("et", "Nariño departemang"), ("eu", "Nariño"), ("fa", "شهرستان نارینیو"), ("fi", "Nariño"), ("fr", "Nariño"), ("gl", "Nariño"), ("gu", "નરીનો વિભાગ"), ("he", "נריניו"), ("hi", "नारिनो विभाग"), ("hr", "Nariño"), ("hu", "Nariño megye"), ("id", "Departemen Nariño"), ("it", "dipartimento di Nariño"), ("ja", "ナリーニョ県"), ("ka", "ნარინიოს დეპარტამენტი"), ("kn", "ನ\u{ccd}ಯಾರ\u{cbf}ನೋ ಇಲಾಖ\u{cc6}"), ("ko", "나리뇨 주"), ("lt", "Narinjo departamentas"), ("lv", "Narinjo departaments"), ("mk", "Нарињо"), ("mr", "नर\u{947}न विभाग"), ("ms", "Pentadbiran Nariño"), ("nb", "Nariño"), ("nl", "Nariño"), ("no", "Nariño"), ("pl", "Nariño"), ("pt", "Nariño"), ("ro", "Nariño"), ("ru", "Нариньо"), ("si", "නර\u{dd2}නෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Nariño"), ("ta", "நரினோ துறை"), ("te", "న\u{c3e}ర\u{c3f}న\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "นาร\u{e34}โน"), ("tr", "Nariño"), ("uk", "Наріньйо"), ("ur", "نارینو محکمہ"), ("vi", "Nariño"), ("yo", "Nariño Department"), ("yo_BJ", "Nariño Department"), ("zh", "纳里尼奥省")]),
                        unofficial_name_list: ["Nariño"].to_vec(),
                    }
                ),
                (
                    "NSA",
                    Subdivision{
                        name: "Norte de Santander",
                        country_alpha2: Alpha2::CO,
                        code: "NSA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.9462831), longitude: Some(-72.8988069), max_latitude: Some(9.29051), min_latitude: Some(6.87306), max_longitude: Some(-72.04813), min_longitude: Some(-73.63743)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة نورتي دي سانتاندر"), ("az", "Şimali Santander"), ("be", "дэпартамент Нортэ-дэ-Сантандэр"), ("bg", "Северен Сантандер"), ("bn", "নর\u{9cd}থে ডি স\u{9be}ন\u{9cd}ত\u{9be}ন\u{9cd}ডের বিভ\u{9be}গ"), ("ca", "Norte de Santander"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄑𑄬 𑄓𑄬 𑄥𑄚\u{11134}𑄑𑄚\u{11134}𑄓𑄢\u{11134}"), ("ceb", "Departamento de Norte de Santander"), ("da", "Norte de Santander Department"), ("de", "Departamento de Norte de Santander"), ("el", "Νόρτε ντε Σαντάντερ"), ("en", "Norte de Santander"), ("es", "Norte de Santander"), ("eu", "Norte de Santander"), ("fa", "شهرستان نورته د سانتاندر"), ("fi", "Norte de Santander"), ("fr", "Norte de Santander"), ("gl", "Norte de Santander"), ("gu", "નોર\u{acd}ટ દ\u{ac7} સ\u{ac7}ન\u{acd}ટ\u{ac7}ન\u{acd}ડર વિભાગ"), ("he", "נורטה דה סנטנדר"), ("hi", "नोर\u{94d}ट\u{947} डी स\u{948}नट\u{947}\u{902}डर विभाग"), ("hr", "Norte de Santander"), ("hu", "Észak-Santander megye"), ("id", "Departemen Norte de Santander"), ("it", "dipartimento di Norte de Santander"), ("ja", "ノルテ・デ・サンタンデール県"), ("ka", "ნორტე-დე-სანტანდერის დეპარტამენტი"), ("kn", "ನಾರ\u{ccd}ಟ\u{cc6} ಡ\u{cc6} ಸ\u{ccd}ಯಾಂಟಾಂಡರ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "노르테데산탄데르 주"), ("lt", "Šiaurės Santandero departamentas"), ("lv", "Ziemeļsantanderas departaments"), ("mr", "नॉर\u{94d}ट\u{947} डी स\u{945}न\u{94d}त\u{947}\u{902}डर विभाग"), ("ms", "Pentadbiran Norte de Santander"), ("nb", "Norte de Santander"), ("nl", "Norte de Santander"), ("no", "Norte de Santander"), ("pl", "Norte de Santander"), ("pt", "Norte de Santander"), ("ru", "Северный Сантандер"), ("si", "නොර\u{dca}ටේ ඩ\u{dd2} සන\u{dca}ටර\u{dca}ඩෙන\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Norte de Santander"), ("ta", "நோர\u{bcd}ட\u{bcd} டி சந\u{bcd}த\u{bbe}ண\u{bcd}டேர\u{bcd} துறை"), ("te", "న\u{c3e}ర\u{c4d}ట\u{c4d} డ\u{c3f} స\u{c3e}ంట\u{c3e}ండర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "นอร\u{e4c}เท เด ซานทานเดอร\u{e4c}"), ("tr", "Norte de Santander"), ("uk", "Норте-де-Сантандер"), ("ur", "شمالی سانتاندر محکمہ"), ("vi", "Norte de Santander"), ("yo", "Norte de Santander Department"), ("yo_BJ", "Norte de Santander Department"), ("zh", "北桑坦德省")]),
                        unofficial_name_list: ["Norte de Santander"].to_vec(),
                    }
                ),
                (
                    "PUT",
                    Subdivision{
                        name: "Putumayo",
                        country_alpha2: Alpha2::CO,
                        code: "PUT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.4359506), longitude: Some(-75.52766989999999), max_latitude: Some(1.46781), min_latitude: Some(-0.56002), max_longitude: Some(-73.84129999999999), min_longitude: Some(-77.18681)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بوتومايو"), ("az", "Putumayo (departament)"), ("bg", "Путумайо"), ("bn", "প\u{9c1}তোম\u{9be}ইয\u{9bc}ো বিভ\u{9be}গ"), ("ca", "Putumayo"), ("ccp", "𑄛\u{1112a}𑄑\u{1112a}𑄟\u{1112d}𑄠\u{1112e}"), ("ceb", "Departamento del Putumayo"), ("da", "Putumayo Department"), ("de", "Departamento de Putumayo"), ("el", "Πουτουμάγιο"), ("en", "Putumayo"), ("es", "Putumayo"), ("eu", "Putumayo"), ("fa", "شهرستان پوتومایو"), ("fi", "Putumayo"), ("fr", "Putumayo"), ("gl", "Putumayo"), ("gu", "પ\u{ac1}ટ\u{ac1}મ\u{ac7}યો વિભાગ"), ("he", "פוטומיו"), ("hi", "प\u{941}ट\u{941}माओ विभाग"), ("hr", "Putumayo"), ("hu", "Putumayo megye"), ("id", "Departemen Putumayo"), ("it", "dipartimento di Putumayo"), ("ja", "プトゥマヨ県"), ("ka", "პუტუმაიოს დეპარტამენტი"), ("kn", "ಪುಟುಮಾಯ\u{cbf} ಇಲಾಖ\u{cc6}"), ("ko", "푸투마요 주"), ("lt", "Putumajo departamentas"), ("lv", "Putumajo departaments"), ("mr", "प\u{941}त\u{94d}त\u{941}मा विभाग"), ("ms", "Pentadbiran Putumayo"), ("nb", "Putumayo"), ("nl", "Putumayo"), ("no", "Putumayo"), ("pl", "Putumayo"), ("pt", "Putumayo"), ("ru", "Путумайо"), ("si", "ප\u{dd4}ට\u{dd4}ම\u{dcf}යෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Putumayo"), ("ta", "புதுமையோ துறை"), ("te", "పుటుమ\u{c3e}య\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e39}ต\u{e39}มาโย"), ("tr", "Putumayo"), ("uk", "Путумайо"), ("ur", "پوتامایو محکمہ"), ("vi", "Putumayo"), ("yo", "Putumayo Department"), ("yo_BJ", "Putumayo Department"), ("zh", "普图马约省")]),
                        unofficial_name_list: ["Putumayo"].to_vec(),
                    }
                ),
                (
                    "QUI",
                    Subdivision{
                        name: "Quindío",
                        country_alpha2: Alpha2::CO,
                        code: "QUI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.4610191), longitude: Some(-75.667356), max_latitude: Some(4.71822), min_latitude: Some(4.07391), max_longitude: Some(-75.38454), min_longitude: Some(-75.89562)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كوينديو"), ("az", "Kindio"), ("bg", "Киндио"), ("bn", "কিনদিও বিভ\u{9be}গ"), ("ca", "Quindío"), ("ccp", "𑄇\u{1112a}𑄃\u{11128}𑄚\u{11134}𑄓\u{11128}𑄃\u{1112e}"), ("ceb", "Quindío Department"), ("da", "Quindío Department"), ("de", "Departamento del Quindío"), ("el", "Κουίντιο"), ("en", "Quindío"), ("es", "Quindío"), ("eu", "Quindío"), ("fa", "شهرستان کیندیو"), ("fi", "Quindío"), ("fr", "Quindío"), ("gl", "Quindío"), ("gu", "ક\u{acd}વિન\u{acd}ડો વિભાગ"), ("he", "קינדיו"), ("hi", "किनडियो विभाग"), ("hr", "Quindío"), ("hu", "Quindío megye"), ("id", "Departemen Quindío"), ("it", "dipartimento di Quindío"), ("ja", "キンディオ県"), ("ka", "კინდიოს დეპარტამენტი"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ಂಡ\u{cbf}ಯೋ ಇಲಾಖ\u{cc6}"), ("ko", "킨디오 주"), ("lt", "Kindijo departamentas"), ("lv", "Kindio departaments"), ("mr", "क\u{94d}वि\u{902}न\u{94d}डियो विभाग"), ("ms", "Pentadbiran Quindío"), ("nb", "Quindío"), ("nl", "Quindío"), ("no", "Quindío"), ("pl", "Quindío"), ("pt", "Quindío"), ("ru", "Киндио"), ("si", "ක\u{dd4}ය\u{dd2}න\u{dca}ඩ\u{dd2}යෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Quindío"), ("ta", "குஇந\u{bcd}தியோ துறை"), ("te", "క\u{c4d}వ\u{c3f}ండ\u{c3f}య\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "คว\u{e34}นเด\u{e35}ยว ด\u{e35}พาทเม\u{e49}น"), ("tr", "Quindío"), ("uk", "Кіндіо"), ("ur", "قیندیو محکمہ"), ("vi", "Quindío"), ("yo", "Quindío Department"), ("yo_BJ", "Quindío Department"), ("zh", "金迪奥省")]),
                        unofficial_name_list: ["Quindío"].to_vec(),
                    }
                ),
                (
                    "RIS",
                    Subdivision{
                        name: "Risaralda",
                        country_alpha2: Alpha2::CO,
                        code: "RIS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.99243), longitude: Some(-76.01866), max_latitude: Some(5.568049999999999), min_latitude: Some(4.66464), max_longitude: Some(-75.37608999999999), min_longitude: Some(-76.21154)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ريزارالدا"), ("az", "Risaralda"), ("bg", "Рисаралда"), ("bn", "রিস\u{9be}র\u{9be}লড\u{9be} বিভ\u{9be}গ"), ("ca", "Risaralda"), ("ccp", "𑄢\u{11128}𑄥𑄢𑄣\u{11134}𑄓"), ("ceb", "Departamento de Risaralda"), ("da", "Risaralda Department"), ("de", "Departamento de Risaralda"), ("el", "Ρισαράλντα"), ("en", "Risaralda"), ("es", "Risaralda"), ("eu", "Risaralda"), ("fa", "شهرستان ریسارالدا"), ("fi", "Risaralda"), ("fr", "Risaralda"), ("gl", "Risaralda"), ("gu", "રિસારલડા વિભાગ"), ("he", "ריסרלדה"), ("hi", "रिसाराल\u{94d}डा विभाग"), ("hr", "Risaralda"), ("hu", "Risaralda megye"), ("id", "Departemen Risaralda"), ("it", "dipartimento di Risaralda"), ("ja", "リサラルダ県"), ("ka", "რისარალდის დეპარტამენტი"), ("kn", "ರ\u{cbf}ಸರಾಲ\u{ccd}ಡಾ ಇಲಾಖ\u{cc6}"), ("ko", "리사랄다 주"), ("lt", "Risaraldos departamentas"), ("lv", "Risaraldas departaments"), ("mr", "रियारलादा विभाग"), ("ms", "Pentadbiran Risaralda"), ("nb", "Risaralda"), ("nl", "Risaralda"), ("no", "Risaralda"), ("pl", "Risaralda"), ("pt", "Risaralda"), ("ru", "Рисаральда"), ("si", "ර\u{dd2}සරල\u{dca}ඩ\u{dcf}"), ("sv", "Risaralda"), ("ta", "ரிசரளட\u{bbe} துறை"), ("te", "ర\u{c3f}స\u{c3e}ర\u{c3e}ల\u{c4d}డ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "บาไบเท"), ("tr", "Risaralda"), ("uk", "Рісаральда"), ("ur", "ریسارالدا محکمہ"), ("vi", "Risaralda"), ("yo", "Risaralda Department"), ("yo_BJ", "Risaralda Department"), ("zh", "里萨拉尔达省")]),
                        unofficial_name_list: ["Risaralda"].to_vec(),
                    }
                ),
                (
                    "SAN",
                    Subdivision{
                        name: "Santander",
                        country_alpha2: Alpha2::CO,
                        code: "SAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.6437076), longitude: Some(-73.65362089999999), max_latitude: Some(8.1410123), min_latitude: Some(5.70671), max_longitude: Some(-72.4764), min_longitude: Some(-74.52665999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتاندير"), ("az", "Santander (departament)"), ("bg", "Сантандер"), ("bn", "স\u{9cd}য\u{9be}নট\u{9be}নড\u{9be}র বিভ\u{9be}গ"), ("ca", "Departament de Santander"), ("ccp", "𑄥𑄚\u{11134}𑄑𑄚\u{11134}𑄓𑄢\u{11134}"), ("ceb", "Departamento de Santander"), ("da", "Santander Department"), ("de", "Departamento de Santander"), ("el", "Διαμέρισμα Σανταντέρ"), ("en", "Santander"), ("es", "Santander"), ("eu", "Santander"), ("fa", "شهرستان سانتاندر"), ("fi", "Santander"), ("fr", "Santander"), ("gl", "Santander"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{ac7}ન\u{acd}ડર વિભાગ"), ("he", "סנטנדר"), ("hi", "स\u{902}तान\u{94d}दर विभाग"), ("hu", "Santander megye"), ("id", "Departemen Santander"), ("it", "dipartimento di Santander"), ("ja", "サンタンデール県"), ("ka", "სანტანდერის დეპარტამენტი"), ("kn", "ಸ\u{ccd}ಯಾಂಟ\u{ccd}ಯಾಂಡರ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "산탄데르 주"), ("lt", "Santandero departamentas"), ("lv", "Santanderas departaments"), ("mr", "स\u{945}नट\u{945}नडर विभाग"), ("ms", "Pentadbiran Santander"), ("nb", "Santander"), ("nl", "Santander"), ("no", "Santander"), ("pl", "Santander"), ("pt", "Santander"), ("ru", "Сантандер"), ("si", "සන\u{dca}ටන\u{dca}ඩෙර\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Santander"), ("ta", "சந\u{bcd}தண\u{bcd}டெர\u{bcd} துறை"), ("te", "స\u{c3e}ంట\u{c3e}ండర\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e31}นต\u{e31}นเดร\u{e4c}"), ("tr", "Santander"), ("uk", "Сантандер"), ("ur", "سانتاندر محکمہ"), ("vi", "Khu vực hành chính Santander"), ("yo", "Santander Department"), ("yo_BJ", "Santander Department"), ("zh", "桑坦德省")]),
                        unofficial_name_list: ["Santander"].to_vec(),
                    }
                ),
                (
                    "SAP",
                    Subdivision{
                        name: "San Andrés, Providencia y Santa Catalina",
                        country_alpha2: Alpha2::CO,
                        code: "SAP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.35395), longitude: Some(-81.37127), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ارخبيل سان اندريس"), ("az", "San Andres və Providensiya"), ("bg", "Сан Андрес и Провиденсия"), ("bn", "দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ সেন\u{9cd}ট আন\u{9cd}ড\u{9cd}রেজ, প\u{9cd}রভিডেন\u{9cd}স এবং সেন\u{9cd}ট ক\u{9be}ত\u{9be}লিন\u{9be}"), ("ca", "San Andrés y Providencia"), ("ccp", "𑄥𑄚\u{11134} 𑄃𑄚\u{11134}𑄓\u{11133}𑄢𑄬 & 𑄛\u{11133}𑄢\u{11127}\u{1112e}𑄞\u{1112d}𑄓𑄬𑄚\u{11128}𑄠"), ("ceb", "Providencia y Santa Catalina"), ("cs", "San Andrés a Providencia"), ("da", "San Andrés y Providencia"), ("de", "San Andrés und Providencia"), ("el", "Διαμέρισμα Άγιου Ανδρέα και Προβιντένσιας"), ("en", "San Andrés & Providencia"), ("es", "Archipiélago de San Andrés, Providencia y Santa Catalina"), ("eu", "San Andrés eta Providencia"), ("fa", "مجمع\u{200c}الجزایر سن آندرس، پروویدنسیا و سانتا کاتالینا"), ("fi", "San Andrés, Providencia y Santa Catalina"), ("fr", "Archipel de San Andrés, Providencia et Santa Catalina"), ("gl", "Arquipélago de San Andrés, Providencia e Santa Catalina"), ("gu", "આર\u{acd}ચિપ\u{ac7}લાગો ઓફ સ\u{ac7}\u{a82}ટ આન\u{acd}દ\u{acd}ર\u{ac7}સ, પ\u{acd}રોવિડ\u{ac7}ન\u{acd}સ , એન\u{acd}ડ સ\u{ac7}ન\u{acd}ટ ક\u{ac7}ટિલાના"), ("he", "סן אנדרס ופרובידנסיה"), ("hi", "स\u{947}\u{902}ट ए\u{902}ड\u{94d}र\u{942}स, प\u{94d}रोविड\u{947}\u{902}स और स\u{947}\u{902}ट क\u{948}टलीना आर\u{94d}कीपल\u{948}गो"), ("hu", "San Andrés y Providencia megye"), ("id", "Departemen San Andrés dan Providencia"), ("it", "dipartimento dell’Arcipelago di San Andrés, Providencia e Santa Catalina"), ("ja", "サン・アンドレス・イ・プロビデンシア県"), ("ka", "სან-ანდრეს-ი-პროვიდენსია"), ("kn", "ಸಂತ ಆಂಡ\u{ccd}ರ\u{cc6}ವ\u{ccd}ಸ\u{ccd}, ಪ\u{ccd}ರಾವ\u{cbf}ಡ\u{cc6}ನ\u{ccd}ಸ\u{ccd} ಮತ\u{ccd}ತು ಸೇಂಟ\u{ccd} ಕ\u{ccd}ಯಾಟಲ\u{cbf}ನಾದ ದ\u{ccd}ವೀಪಸಮ\u{cc2}ಹ"), ("ko", "산안드레스 이 프로비덴시아 주"), ("lt", "San Andresas ir Providensija"), ("lv", "Sanandresas, Providensijas un Santakatalinas arhipelāgs"), ("mr", "स\u{947}\u{902}ट अ\u{901}ण\u{94d}ड\u{94d}र\u{94d}सच\u{947} अर\u{94d}पलीगोगो, प\u{94d}रोविड\u{947}\u{902}स अ\u{945}न\u{94d}ड स\u{947}\u{902}ट कातालिना"), ("ms", "Pentadbiran San Andrés dan Providencia"), ("nb", "San Andrés y Providencia"), ("nl", "San Andrés en Providencia"), ("no", "San Andrés y Providencia"), ("pa", "ਸਾਨ ਆ\u{a02}ਦਰ\u{a47}ਸ, ਪ\u{a4d}ਰ\u{a4b}ਵੀਦ\u{a48}\u{a02}ਸੀਆ ਅਤ\u{a47} ਸਾ\u{a02}ਤਾ ਕਾਤਾਲੀਨਾ ਟਾਪ\u{a42}-ਸਮ\u{a42}ਹ"), ("pl", "San Andrés i Providencia"), ("pt", "Santo André, Providência e Santa Catarina"), ("ru", "Сан-Андрес-и-Провиденсия"), ("si", "ශ\u{dcf}න\u{dca}ත ඇන\u{dca}ඩ\u{dca}ර\u{dd4} වල අර\u{dca}ච\u{dca}පෙලගෝ,ශ\u{dcf}න\u{dca}ත කැටල\u{dd2}න\u{dcf} පළ\u{dcf}ත"), ("sr", "Сан Андрес и Провиденсија"), ("sr_Latn", "San Andres i Providensija"), ("sv", "San Andrés och Providencia"), ("sw", "San Andrés"), ("ta", "அர\u{bcd}ச\u{bcd}சிபெலகோ ஆப\u{bcd} செயின\u{bcd}ட\u{bcd} அந\u{bcd}திரேவ\u{bcd}ஸ\u{bcd} , ப\u{bcd}ரொவிடென\u{bcd}ஸ\u{bcd} அண\u{bcd}ட\u{bcd} செயின\u{bcd}ட\u{bcd} கேட\u{bcd}டலின\u{bbe}"), ("te", "ఆర\u{c4d}చ\u{c3f}ప\u{c46}ల\u{c3e}గ\u{c4b} ఆఫ\u{c4d} స\u{c46}య\u{c3f}ంట\u{c4d} ఆండ\u{c4d}రూస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}డ\u{c46}న\u{c4d}స\u{c4d} అండ\u{c4d} స\u{c46}య\u{c3f}ంట\u{c4d} క\u{c3e}టల\u{c40}న\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e31}นอ\u{e31}นเดรสและโปรบ\u{e35}เดนเซ\u{e35}ย"), ("tr", "San Andrés, Providencia ve Santa Catalina Takımadaları"), ("uk", "Сан-Андрес і Провіденсія"), ("ur", "مجموعہ الجزائر سان اینڈریس، پرووڈینسیا و سانتا کاتالینا"), ("vi", "Archipelago ở Saint Andréws, Providence và Saint Catalina"), ("yo", "San Andrés, Providencia àti Santa Catalina"), ("yo_BJ", "San Andrés, Providencia àti Santa Catalina"), ("zh", "聖安德列斯-普羅維登西亞和聖卡塔利娜群島省")]),
                        unofficial_name_list: ["San Andrés, Providencia y Santa Catalina"].to_vec(),
                    }
                ),
                (
                    "SUC",
                    Subdivision{
                        name: "Sucre",
                        country_alpha2: Alpha2::CO,
                        code: "SUC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.813889999999999), longitude: Some(-74.72529999999999), max_latitude: Some(9.04574), min_latitude: Some(8.568349999999999), max_longitude: Some(-74.55743), min_longitude: Some(-74.89497999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سوكري"), ("az", "Sukre (departament)"), ("bg", "Сукре"), ("bn", "স\u{9c1}ক\u{9cd}রে বিভ\u{9be}গ"), ("ca", "Departament de Sucre"), ("ccp", "𑄥\u{1112a}𑄇\u{11133}𑄢𑄬"), ("ceb", "Departamento de Sucre"), ("da", "Sucre Department"), ("de", "Departamento de Sucre"), ("el", "Σούκρε"), ("en", "Sucre"), ("es", "Sucre"), ("eu", "Sucre departamendua"), ("fa", "شهرستان سوکره"), ("fi", "Sucre"), ("fr", "Sucre"), ("gl", "Sucre"), ("gu", "સ\u{ac1}ક\u{acd}ર\u{ac7} વિભાગ"), ("he", "סוקרה"), ("hi", "स\u{942}क\u{94d}र विभाग"), ("hu", "Sucre megye"), ("id", "Departemen Sucre"), ("it", "dipartimento di Sucre"), ("ja", "スクレ県"), ("ka", "სუკრეს დეპარტამენტი"), ("kn", "ಸುಕ\u{ccd}ರ\u{cc6} ಇಲಾಖ\u{cc6}"), ("ko", "수크레 주"), ("lt", "Sukrės departamentas"), ("lv", "Sukres departaments"), ("mr", "स\u{941}क\u{94d}र\u{947} विभाग"), ("ms", "Pentadbiran Sucre"), ("nb", "Sucre"), ("nl", "Sucre"), ("no", "Sucre"), ("pl", "Sucre"), ("pt", "Sucre"), ("ru", "Сукре"), ("si", "සක\u{dca}රේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Sucre"), ("ta", "சுகிரே துறை"), ("te", "సుక\u{c4d}ర\u{c46} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ซ\u{e39}เกร"), ("tr", "Sucre"), ("uk", "Сукре"), ("ur", "سوکرے محکمہ"), ("vi", "Khu vực hành chính Sucre"), ("yo", "Sucre Department"), ("yo_BJ", "Sucre Department"), ("zh", "苏克雷省")]),
                        unofficial_name_list: ["Sucre"].to_vec(),
                    }
                ),
                (
                    "TOL",
                    Subdivision{
                        name: "Tolima",
                        country_alpha2: Alpha2::CO,
                        code: "TOL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.092516799999999), longitude: Some(-75.1545381), max_latitude: Some(5.316949999999999), min_latitude: Some(2.8705), max_longitude: Some(-74.47525), min_longitude: Some(-76.10852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة توليما"), ("az", "Tolima"), ("bg", "Толима"), ("bn", "টোলিম\u{9be} বিভ\u{9be}গ"), ("ca", "Tolima"), ("ccp", "𑄑\u{11127}𑄣\u{11128}𑄟"), ("ceb", "Departamento de Tolima"), ("cs", "Tolima"), ("da", "Tolima Department"), ("de", "Tolima"), ("el", "Διαμέρισμα Τολίμα"), ("en", "Tolima"), ("es", "Tolima"), ("eu", "Tolima"), ("fa", "شهرستان تولیما"), ("fi", "Tolima"), ("fr", "Tolima"), ("gl", "Tolima"), ("gu", "તોલિમા વિભાગ"), ("he", "טולימה"), ("hi", "तोलिमा विभाग"), ("hu", "Tolima megye"), ("id", "Departemen Tolima"), ("it", "dipartimento di Tolima"), ("ja", "トリマ県"), ("ka", "ტოლიმის დეპარტამენტი"), ("kn", "ಟಾಲ\u{cbf}ಮಾ ಇಲಾಖ\u{cc6}"), ("ko", "톨리마 주"), ("lt", "Tolimos departamentas"), ("lv", "Tolimas departaments"), ("mr", "तोलिमा विभाग"), ("ms", "Pentadbiran Tolima"), ("nb", "Tolima"), ("nl", "Tolima"), ("no", "Tolima"), ("pl", "Tolima"), ("pt", "Tolima"), ("ru", "Толима"), ("si", "ටෝල\u{dd2}ම\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Tolima"), ("ta", "டோலிம\u{bbe} துறை"), ("te", "ట\u{c4b}ల\u{c3f}మ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โตล\u{e34}มา"), ("tr", "Tolima"), ("uk", "Толіма"), ("ur", "تولیما محکمہ"), ("vi", "Khu vực hành chính Tolima"), ("yo", "Tolima Department"), ("yo_BJ", "Tolima Department"), ("zh", "托利马省")]),
                        unofficial_name_list: ["Tolima"].to_vec(),
                    }
                ),
                (
                    "VAC",
                    Subdivision{
                        name: "Valle del Cauca",
                        country_alpha2: Alpha2::CO,
                        code: "VAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.8008893), longitude: Some(-76.64127119999999), max_latitude: Some(5.03687), min_latitude: Some(3.09083), max_longitude: Some(-75.70488), min_longitude: Some(-77.5440788)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة فالي ديل كاوكا"), ("az", "Valye Del Kauka"), ("be", "дэпартамент Валье-дэль-Каўка"), ("bg", "Вале дел Каука"), ("bn", "ভ\u{9be}ল ডেল ক\u{9be}উক\u{9be} বিভ\u{9be}গ"), ("ca", "Valle del Cauca"), ("ccp", "𑄞𑄣𑄬 𑄓𑄬𑄣\u{11134} 𑄇\u{1112f}𑄇"), ("ceb", "Departamento del Valle del Cauca"), ("cs", "Valle del Cauca"), ("da", "Valle del Cauca Department"), ("de", "Valle del Cauca"), ("el", "Διαμέρισμα Βάλιε δελ Κάουκα"), ("en", "Valle del Cauca"), ("es", "Valle del Cauca"), ("eu", "Valle del Cauca"), ("fa", "شهرستان بایه دل کائوکا"), ("fi", "Valle del Cauca"), ("fr", "Valle del Cauca"), ("gl", "Valle del Cauca"), ("gu", "વ\u{ac7}લ\u{ac7} ડ\u{ac7}લ કૌકા વિભાગ"), ("he", "ואייה דל קאוקה"), ("hi", "व\u{948}ल\u{947} ड\u{947}ल काऊका विभाग"), ("hu", "Valle del Cauca megye"), ("id", "Departemen Valle del Cauca"), ("it", "dipartimento di Valle del Cauca"), ("ja", "バジェ・デル・カウカ県"), ("ka", "ვალიე-დელ-კაუკის დეპარტამენტი"), ("kn", "ವ\u{ccd}ಯಾಲ\u{cc6} ಡ\u{cc6}ಲ\u{ccd} ಕ\u{ccc}ಕಾ ಇಲಾಖ\u{cc6}"), ("ko", "바예델카우카 주"), ("lt", "Valje del Kaukos departamentas"), ("lv", "Valjes del Kaukas departaments"), ("mr", "व\u{945}ल\u{947} ड\u{947}ल काऊका विभाग"), ("ms", "Pentadbiran Valle del Cauca"), ("nb", "Valle del Cauca"), ("nl", "Valle del Cauca"), ("no", "Valle del Cauca"), ("pl", "Valle del Cauca"), ("pt", "Valle del Cauca"), ("ru", "Валье-дель-Каука"), ("si", "වැල\u{dd2} ඩෙල\u{dca} ක\u{dcf}ව\u{dd4}ක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Valle del Cauca"), ("ta", "வல\u{bcd}லே டேல\u{bcd} க\u{bbe}க\u{bcd}க\u{bbe} துறை"), ("te", "వల\u{c4d}ల\u{c46} డ\u{c46}ల\u{c4d} క\u{c3e}క\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดบาเยเดลเกากา"), ("tr", "Valle del Cauca"), ("uk", "Вальє-дель-Каука"), ("ur", "ویے دیل کاؤکا محکمہ"), ("vi", "Valle del Cauca"), ("yo", "Valle del Cauca Department"), ("yo_BJ", "Valle del Cauca Department"), ("zh", "考卡山谷省")]),
                        unofficial_name_list: ["Valle del Cauca"].to_vec(),
                    }
                ),
                (
                    "VAU",
                    Subdivision{
                        name: "Vaupés",
                        country_alpha2: Alpha2::CO,
                        code: "VAU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.8553561), longitude: Some(-70.81199529999999), max_latitude: Some(2.08011), min_latitude: Some(-1.22768), max_longitude: Some(-69.11563), min_longitude: Some(-72.03437)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة فاوبيس"), ("az", "Vaupes"), ("bg", "Ваупес"), ("bn", "ভ\u{9c1}প\u{9cd}স বিভ\u{9be}গ"), ("ca", "Vaupés"), ("ccp", "𑄞𑄅\u{1112a}𑄛𑄬𑄌\u{11134}"), ("ceb", "Departamento del Vaupés"), ("cs", "Vaupés"), ("da", "Vaupés Department"), ("de", "Departamento del Vaupés"), ("el", "Βοπές"), ("en", "Vaupés"), ("es", "Vaupés"), ("eu", "Vaupés"), ("fa", "واوپس دپارتمنت"), ("fi", "Vaupés"), ("fr", "Vaupés"), ("gl", "Vaupés"), ("gu", "વોપ\u{ac7}સ વિભાગ"), ("he", "ואופס"), ("hi", "वॉप\u{947}स विभाग"), ("hu", "Vaupés megye"), ("id", "Departemen Vaupés"), ("it", "dipartimento di Vaupés"), ("ja", "バウペス県"), ("ka", "ვაუპესის დეპარტამენტი"), ("kn", "ವ\u{cc2}ಪ\u{cc6}ಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "바우페스 주"), ("lt", "Vaupeso departamentas"), ("lv", "Vaupesas departaments"), ("mr", "व\u{941}प\u{94d}स विभाग"), ("ms", "Pentadbiran Vaupés"), ("nb", "Vaupés"), ("nl", "Vaupés"), ("no", "Vaupés"), ("pl", "Vaupés"), ("pt", "Vaupés"), ("ru", "Ваупес"), ("si", "වෞපේස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Vaupés"), ("ta", "வ\u{bbe}யுப\u{bcd}ஸ\u{bcd} துறை"), ("te", "వ\u{c3e}ప\u{c46}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "าย\u{e38}เปส ด\u{e35}พาทเม\u{e49}น"), ("tr", "Vaupés"), ("uk", "Ваупес"), ("ur", "واوپیس محکمہ"), ("vi", "Khu vực hành chính Vaupés"), ("yo", "Vaupés Department"), ("yo_BJ", "Vaupés Department"), ("zh", "沃佩斯省")]),
                        unofficial_name_list: ["Vaupés"].to_vec(),
                    }
                ),
                (
                    "VID",
                    Subdivision{
                        name: "Vichada",
                        country_alpha2: Alpha2::CO,
                        code: "VID",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.4234452), longitude: Some(-69.2877535), max_latitude: Some(6.32433), min_latitude: Some(2.7371), max_longitude: Some(-67.40982), min_longitude: Some(-71.07766)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة فيتشادا"), ("az", "Viçada"), ("bg", "Вичада"), ("bn", "ভিক\u{9be}দ\u{9be} বিভ\u{9be}গ"), ("ca", "Vichada"), ("ccp", "𑄞\u{1112d}𑄥𑄓"), ("ceb", "Departamento del Vichada"), ("da", "Vichada Department"), ("de", "Vichada"), ("el", "Βιτσάντα"), ("en", "Vichada"), ("es", "Vichada"), ("eu", "Vichada"), ("fa", "شهرستان ویچادا"), ("fi", "Vichada"), ("fr", "Vichada"), ("gl", "Vichada"), ("gu", "વિચાડા વિભાગ"), ("he", "ויצ׳דה"), ("hi", "विकाडा विभाग"), ("hu", "Vichada megye"), ("id", "Departemen Vichada"), ("it", "dipartimento di Vichada"), ("ja", "ビチャーダ県"), ("ka", "ვიჩადის დეპარტამენტი"), ("kn", "ವ\u{cbf}ಕಾಡಾ ಇಲಾಖ\u{cc6}"), ("ko", "비차다 주"), ("lt", "Vičados departamentas"), ("lv", "Vičadas departaments"), ("mr", "वीच\u{945}ड विभाग"), ("ms", "Pentadbiran Vichada"), ("nb", "Vichada"), ("nl", "Vichada"), ("no", "Vichada"), ("pl", "Vichada"), ("pt", "Vichada"), ("ru", "Вичада"), ("si", "ව\u{dd2}ක\u{dca}හඩ\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Vichada"), ("ta", "விச\u{bcd}ச\u{bbe}ட\u{bbe} துறை"), ("te", "వ\u{c3f}చ\u{c3e}డ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ว\u{e34}ชาดา ด\u{e35}พาทเม\u{e49}น"), ("tr", "Vichada"), ("uk", "Вічада"), ("ur", "بیچادا محکمہ"), ("vi", "Khu vực hành chính Vichada"), ("yo", "Vichada Department"), ("yo_BJ", "Vichada Department"), ("zh", "比查达省")]),
                        unofficial_name_list: ["Vichada"].to_vec(),
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
#[cfg(feature = "co")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CO,
        alpha3: Alpha3::COL,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 57,
        currency_code: CurrencyCode::COP,
        gec: Some(GEC::CO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "005",
        ioc: Some(IOC::COL),
        iso_long_name: "The Republic of Colombia",
        iso_short_name: "Colombia",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9, 10].to_vec(),
        national_prefix: "05",
        nationality: Some("Colombian"),
        number: "170",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "CO",
        unofficial_name_list: ["Colombia", "Kolumbien", "Colombie", "コロンビア"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Colombia"),
            ("af", "Colombië"),
            ("ak", "Colombia"),
            ("am", "ጥሕሤቢ።"),
            ("an", "Colombia"),
            ("ar", "كولومبيا"),
            ("as", "কোলোম\u{9cd}বিয়\u{9be}"),
            ("ay", "Colombia"),
            ("az", "Kolumbiya"),
            ("ba", "Colombia"),
            ("be", "Калумбія"),
            ("bg", "Колумбия"),
            ("bi", "Colombia"),
            ("bn", "কোলোম\u{9cd}বিয়\u{9be}"),
            ("bn_IN", "কোলোম\u{9cd}বিয়\u{9be}"),
            ("br", "Kolombia"),
            ("bs", "Kolumbija"),
            ("ca", "Colòmbia"),
            ("ce", "Колумби"),
            ("ch", "Colombia"),
            ("cs", "Kolumbie"),
            ("cv", "Колумби"),
            ("cy", "Colombia"),
            ("da", "Colombia"),
            ("de", "Kolumbien"),
            ("dv", "ކ\u{7ae}ލ\u{7a6}ނ\u{7b0}ބ\u{7a8}އ\u{7a7}"),
            ("dz", "ཀ\u{f7c}་ལ\u{f7c}མ་བ\u{f72}་ཡ།"),
            ("ee", "Colombia"),
            ("el", "Κολομβία"),
            ("en", "Colombia"),
            ("eo", "Kolombio"),
            ("es", "Colombia"),
            ("et", "Colombia"),
            ("eu", "Kolonbia"),
            ("fa", "کلمبیا"),
            ("ff", "Colombia"),
            ("fi", "Kolumbia"),
            ("fo", "Kolombia"),
            ("fr", "Colombie"),
            ("fy", "Kolombia"),
            ("ga", "An Cholóim"),
            ("gl", "Colombia"),
            ("gn", "Colombia"),
            ("gu", "કોલોમ\u{acd}બિયા"),
            ("gv", "Yn Cholombey"),
            ("ha", "Colombia"),
            ("he", "קולומביה"),
            ("hi", "कोलम\u{94d}बिया"),
            ("hr", "Kolumbija"),
            ("ht", "Kolonbi"),
            ("hu", "Kolumbia"),
            ("hy", "Կոլումբիա"),
            ("ia", "Colombia"),
            ("id", "Kolombia"),
            ("io", "Kolumbia"),
            ("is", "Kólumbía"),
            ("it", "Colombia"),
            ("iu", "Colombia"),
            ("ja", "コロンビア"),
            ("ka", "კოლუმბია"),
            ("ki", "Colombia"),
            ("kk", "Колумбия"),
            ("kl", "Colombia"),
            ("km", "ក\u{17bc}ឡ\u{17bb}\u{17c6}ប\u{17ca}\u{17b8}"),
            ("kn", "ಕೋಲಂಬ\u{cbf}ಯಾ"),
            ("ko", "콜롬비아"),
            ("ku", "Kolombiya"),
            ("kv", "Colombia"),
            ("kw", "Kolombi"),
            ("ky", "Колумбия"),
            ("lo", "ປະເທດໂກລ\u{ebb}ມບ\u{eb5}"),
            ("lt", "Kolumbija"),
            ("lv", "Kolumbija"),
            ("mi", "Koromōpia"),
            ("mk", "Колумбија"),
            ("ml", "കൊളംബിയ"),
            ("mn", "Колумб"),
            ("mr", "कोल\u{902}बिया"),
            ("ms", "Colombia"),
            ("mt", "Colombia"),
            (
                "my",
                "က\u{102d}\u{102f}လ\u{1036}ဘ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Korombiya"),
            ("nb", "Colombia"),
            ("ne", "कोलम\u{94d}बिया"),
            ("nl", "Colombia"),
            ("nn", "Colombia"),
            ("nv", "Colombia"),
            ("oc", "Colombia"),
            ("or", "କୋଲୋମ\u{b4d}ବ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਕ\u{a4b}ਲ\u{a70}ਬੀਆ"),
            ("pi", "कोलोम\u{94d}बिया"),
            ("pl", "Kolumbia"),
            ("ps", "کولمبیا"),
            ("pt", "Colômbia"),
            ("pt_BR", "Colômbia"),
            ("ro", "Columbia"),
            ("ru", "Колумбия"),
            ("rw", "Kolombiya"),
            ("sc", "Colòmbia"),
            ("sd", "Colombia"),
            ("si", "කොලොම\u{dca}බ\u{dd2}ය\u{dcf}ව"),
            ("sk", "Kolumbia"),
            ("sl", "Kolumbija"),
            ("so", "Kolombiya"),
            ("sq", "Kolumbi"),
            ("sr", "Колумбија"),
            ("sv", "Colombia"),
            ("sw", "Kolombia"),
            ("ta", "கொலம\u{bcd}பிய\u{bbe}"),
            ("te", "క\u{c4b}లంబ\u{c3f}య\u{c3e}"),
            ("tg", "Колумбия"),
            ("th", "โคลอมเบ\u{e35}ย"),
            ("ti", "ኮሎምብያ"),
            ("tk", "Kolumbiýa"),
            ("tl", "Kolombya"),
            ("tr", "Kolombiya"),
            ("tt", "Коломбиа"),
            ("ug", "كولومبىيە"),
            ("uk", "Колумбія"),
            ("ur", "کولمبیا"),
            ("uz", "Kolumbiya"),
            ("ve", "Colombia"),
            ("vi", "Cô-lôm-bi-a"),
            ("wa", "Colombeye"),
            ("wo", "Koloombi"),
            ("xh", "Colombia"),
            ("yo", "Kòlómbìà"),
            ("zh_CN", "哥伦比亚"),
            ("zh_HK", "哥倫比亞"),
            ("zh_TW", "哥倫比亞"),
            ("zu", "IKolombiya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
