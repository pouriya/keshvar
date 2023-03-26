// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of the Congo

#[cfg(all(feature = "cg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CG;
    pub const ALPHA3: Alpha3 = Alpha3::COG;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 242;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::XAF;
    pub const GEC: Option<GEC> = Some(GEC::CF);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CGO);
    pub const ISO_SHORT_NAME: &str = "Congo";
    pub const ISO_LONG_NAME: &str = "The Republic of the Congo";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr", "ln"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr", "ln"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Congolese");
    pub const NUMBER: &str = "178";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::MiddleAfrica);
    pub const UN_LOCODE: &str = "CG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Congo",
        "Kongo",
        "コンゴ共和国",
        "Congo [Republiek]",
        "Congo, Republic of",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Congo"),
        ("af", "Kongo"),
        ("ak", "Congo"),
        ("am", "ኮንጎ"),
        ("an", "Congo"),
        ("ar", "الكونغو"),
        ("as", "কঙ\u{9cd}গো"),
        ("ay", "Congo"),
        ("az", "Konqo"),
        ("ba", "Congo"),
        ("be", "Конга"),
        ("bg", "Конго"),
        ("bi", "Congo"),
        ("bn", "কঙ\u{9cd}গো"),
        ("bn_IN", "কঙ\u{9cd}গো"),
        ("br", "Congo"),
        ("bs", "Kongo"),
        ("ca", "Congo"),
        ("ce", "Congo"),
        ("ch", "Congo"),
        ("cs", "Kongo"),
        ("cv", "Congo"),
        ("cy", "Congo"),
        ("da", "Congo"),
        ("de", "Kongo"),
        ("dv", "Congo"),
        ("dz", "ཀ\u{f7c}ང་ག\u{f7c}།"),
        ("ee", "Congo"),
        ("el", "Κονγκό"),
        ("en", "Congo"),
        ("eo", "Kongo"),
        ("es", "Congo"),
        ("et", "Kongo Vabariik"),
        ("eu", "Kongo"),
        ("fa", "کونگو"),
        ("ff", "Congo"),
        ("fi", "Kongo"),
        ("fo", "Kongo"),
        ("fr", "République du Congo"),
        ("fy", "Congo"),
        ("ga", "An Congó"),
        ("gl", "Congo"),
        ("gn", "Congo"),
        ("gu", "કોન\u{acd}ગો"),
        ("gv", "Congo"),
        ("ha", "Congo"),
        ("he", "קונגו"),
        ("hi", "कॉ\u{902}गो"),
        ("hr", "Kongo"),
        ("ht", "Congo"),
        ("hu", "Kongó"),
        ("hy", "Կոնգո"),
        ("ia", "Congo"),
        ("id", "Congo"),
        ("io", "Congo"),
        ("is", "Kongó"),
        ("it", "Congo"),
        ("iu", "Congo"),
        ("ja", "コンゴ"),
        ("ka", "კონგო"),
        ("ki", "Congo"),
        ("kk", "Конго"),
        ("kl", "Congo"),
        ("km", "ក\u{17bb}ងហ\u{17d2}គោ"),
        ("kn", "ಕಾಂಗೋ"),
        ("ko", "콩고"),
        ("ku", "Kongo"),
        ("kv", "Congo"),
        ("kw", "Congo"),
        ("ky", "Конго"),
        ("lo", "Congo"),
        ("lt", "Kongas"),
        ("lv", "Kongo"),
        ("mi", "Kōngo"),
        ("mk", "Конго"),
        ("ml", "കോംഗോ"),
        ("mn", "Конго"),
        ("mr", "कॉ\u{902}गो"),
        ("ms", "Kongo"),
        ("mt", "Kongo"),
        ("my", "Congo"),
        ("na", "Congo"),
        ("nb", "Kongo"),
        ("ne", "कोङ\u{94d}गो"),
        ("nl", "Congo"),
        ("nn", "Kongo"),
        ("nv", "Congo"),
        ("oc", "Còngo"),
        ("or", "କଙ\u{b4d}ଗୋ"),
        ("pa", "ਕਾ\u{a02}ਗ\u{a4b}"),
        ("pi", "Congo"),
        ("pl", "Kongo"),
        ("ps", "Congo"),
        ("pt", "Congo"),
        ("pt_BR", "Congo"),
        ("ro", "Congo"),
        ("ru", "Конго"),
        ("rw", "Kongo"),
        ("sc", "Congo"),
        ("sd", "Congo"),
        ("si", "කොන\u{dca}ගෝව"),
        ("sk", "Kongo"),
        ("sl", "Kongo"),
        ("so", "Congo"),
        ("sq", "Kongo"),
        ("sr", "Конго"),
        ("sv", "Kongo"),
        ("sw", "Congo"),
        ("ta", "க\u{bbe}ங\u{bcd}கோ"),
        ("te", "క\u{c3e}ంగ\u{c4b}"),
        ("tg", "Конго"),
        ("th", "คองโก"),
        ("ti", "ኮንጎ"),
        ("tk", "Kongo"),
        ("tl", "Congo"),
        ("tr", "Kongo"),
        ("tt", "Конgо"),
        ("ug", "كونگو"),
        ("uk", "Конго"),
        ("ur", "Congo"),
        ("uz", "Congo"),
        ("ve", "Congo"),
        ("vi", "Công-gô"),
        ("wa", "Congo-Brazza"),
        ("wo", "Kongóo"),
        ("xh", "Congo"),
        ("yo", "Congo"),
        ("zh_CN", "刚果"),
        ("zh_HK", "剛果"),
        ("zh_TW", "剛果"),
        ("zu", "Congo"),
    ];
    #[cfg(all(feature = "cg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -0.228021;
        pub const LONGITUDE: f64 = 15.827659;
        pub const MAX_LATITUDE: f64 = 3.707791;
        pub const MAX_LONGITUDE: f64 = 18.650421;
        pub const MIN_LATITUDE: f64 = -5.0964;
        pub const MIN_LONGITUDE: f64 = 11.1182001;
        pub const NORTHEAST_LATITUDE: f64 = 3.707791;
        pub const NORTHEAST_LONGITUDE: f64 = 18.650421;
        pub const SOUTHWEST_LATITUDE: f64 = -5.0964;
        pub const SOUTHWEST_LONGITUDE: f64 = 11.1182001;
    }
}
#[cfg(all(feature = "cg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -0.228021,
            longitude: 15.827659,
            max_latitude: 3.707791,
            max_longitude: 18.650421,
            min_latitude: -5.0964,
            min_longitude: 11.1182001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 3.707791,
                    longitude: 18.650421,
                },
                southwest: CountryGeoBound {
                    latitude: -5.0964,
                    longitude: 11.1182001,
                },
            },
        }
    }
}

#[cfg(all(feature = "cg", feature = "subdivisions"))]
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
                    "11",
                    Subdivision{
                        name: "Bouenza",
                        country_alpha2: Alpha2::CG,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.1128079), longitude: Some(13.7289167), max_latitude: Some(-3.45881), min_latitude: Some(-4.78104), max_longitude: Some(14.383406), min_longitude: Some(12.601334)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بوينزا"), ("bg", "Буенза"), ("bn", "ব\u{9c1}য\u{9bc}েঞ\u{9cd}জ\u{9be} বিভ\u{9be}গ"), ("ccp", "𑄝\u{1112f}𑄠𑄬𑄚\u{11134}𑄎"), ("ceb", "Région de la Bouenza"), ("da", "Bouenza Department"), ("de", "Bouenza"), ("el", "Μπουενζά"), ("en", "Bouenza"), ("es", "Bouenza"), ("fi", "Bouenzan departmentti"), ("fr", "Bouenza"), ("gu", "બોઉ\u{a82}એન\u{acd}ઝા વિભાગ"), ("hi", "ब\u{941}ए\u{901}ज\u{93c}ा विभाग"), ("id", "Bouenza"), ("it", "Dipartimento di Bouenza"), ("ja", "ブエンザ地方"), ("ka", "ბუენზის დეპარტამენტი"), ("kn", "ಬೋವ\u{cc6}ಂಜ ಇಲಾಖ\u{cc6}"), ("ko", "부엔자 주"), ("lt", "Buenzos regionas"), ("lv", "Buenzas departaments"), ("mr", "बॉऊएन\u{94d}झा विभाग"), ("ms", "Bouenza"), ("nb", "Bouenza"), ("nl", "Bouenza"), ("no", "Bouenza"), ("pl", "Departament Bouenza"), ("pt", "Bouenza"), ("ro", "Bouenza"), ("ru", "Буэнза"), ("si", "බෝඑන\u{dca}ස\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Région de la Bouenza"), ("ta", "போன\u{bcd}ஜ\u{bbe} துறை"), ("te", "బ\u{c4b}య\u{c46}ంజ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "บ\u{e39}อองซา"), ("tr", "Bouenza"), ("uk", "Регіон Буенза"), ("ur", "بؤینزا محکمہ"), ("vi", "Khu vực hành chính Bouenza"), ("zh", "布恩扎省")]),
                        unofficial_name_list: ["Bouénza"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "Pool",
                        country_alpha2: Alpha2::CG,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.420088), longitude: Some(16.1816196), max_latitude: Some(2.2295692), min_latitude: Some(-6.1148617), max_longitude: Some(25.1980562), min_longitude: Some(12.3650857)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بول"), ("bg", "Пол"), ("bn", "প\u{9c1}ল বিভ\u{9be}গ"), ("ca", "Pool"), ("ccp", "𑄛\u{1112a}𑄣\u{11134}"), ("ceb", "Pool (departamento)"), ("da", "Pool Department"), ("de", "Pool"), ("el", "Πουλ"), ("en", "Pool"), ("es", "Departamento de Pool"), ("fi", "Poolin departmentti"), ("fr", "Pool"), ("gu", "પ\u{ac2}લ વિભાગ"), ("hi", "प\u{942}ल विभाग"), ("id", "Pool Department"), ("it", "Dipartimento di Pool"), ("ja", "プール地方"), ("ka", "პოოლის დეპარტამენტი"), ("kn", "ಪ\u{cc2}ಲ\u{ccd} ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "풀 주"), ("lt", "Baseino regionas"), ("lv", "Pulas departaments"), ("mr", "प\u{942}ल विभाग"), ("ms", "Pool Department"), ("nb", "Pool"), ("nl", "Pool"), ("no", "Pool"), ("pl", "Departament Pool"), ("pt", "Pool"), ("ro", "Departamentul Pool"), ("ru", "Пул"), ("si", "ප\u{dd6}ල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Pool (departement)"), ("ta", "பூல\u{bcd} துறை"), ("te", "పూల\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตพล\u{e39}"), ("tr", "Pool"), ("uk", "Регіон Пул"), ("ur", "پول محکمہ"), ("vi", "Khu vực hành chính Pool"), ("zh", "普爾省")]),
                        unofficial_name_list: ["Pool"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "Sangha",
                        country_alpha2: Alpha2::CG,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.4662328), longitude: Some(15.4068079), max_latitude: Some(2.685291), min_latitude: Some(0.031058), max_longitude: Some(17.014767), min_longitude: Some(13.130887)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة سانغا"), ("bg", "Санга"), ("bn", "স\u{9be}ংহ\u{9be} বিভ\u{9be}গ"), ("ccp", "𑄥\u{11101}𑄦"), ("ceb", "Sangha"), ("da", "Sangha Department"), ("de", "Sangha"), ("el", "Σανγκά"), ("en", "Sangha"), ("es", "Departamento de Sangha"), ("fi", "Sangha"), ("fr", "Sangha"), ("gu", "સા\u{a82}ઘા વિભાગ"), ("hi", "स\u{902}घ विभाग"), ("id", "Sangha Department"), ("it", "Dipartimento di Sangha"), ("ja", "サンガ地方"), ("ka", "სანგის დეპარტამენტი"), ("kn", "ಸಂಘ ಇಲಾಖ\u{cc6}"), ("ko", "상가 주"), ("lt", "Sangos regionas"), ("lv", "Sangas departaments"), ("mr", "स\u{902}घ विभाग"), ("ms", "Sangha Department"), ("nb", "Sangha"), ("nl", "Sangha"), ("no", "Sangha"), ("pl", "Departament Sangha"), ("pt", "Sangha"), ("ro", "Sangha"), ("ru", "Санга"), ("si", "සැන\u{dca}ග\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Sangha (departement)"), ("ta", "சங\u{bcd}க துறை"), ("te", "సంఘ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ซ\u{e31}งกา"), ("tr", "Sangha"), ("uk", "Регіон Санга"), ("ur", "سانگا محکمہ"), ("vi", "Khu vực hành chính Sangha"), ("zh", "桑加省")]),
                        unofficial_name_list: ["Sangha"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "Plateaux",
                        country_alpha2: Alpha2::CG,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.0680088), longitude: Some(15.4068079), max_latitude: Some(-0.939503), min_latitude: Some(-3.098674), max_longitude: Some(16.6609534), min_longitude: Some(14.0830219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بلاتيوكس"), ("bg", "Плато"), ("bn", "প\u{9cd}লেট\u{9c1}ক\u{9cd}স বিভ\u{9be}গ"), ("ccp", "𑄛\u{11133}𑄣\u{11127}𑄑\u{1112e}𑄠𑄇\u{11134}"), ("ceb", "Plateaux (departamento sa Republika sa Congo)"), ("da", "Plateaux Department"), ("de", "Plateaux"), ("el", "Πλατώ"), ("en", "Plateaux"), ("es", "Plateaux"), ("fi", "Plateauxin departmentti"), ("fr", "Plateaux"), ("gu", "પ\u{acd}લ\u{ac7}ટોક\u{acd}સ વિભાગ"), ("hi", "प\u{94d}ल\u{947}टो विभाग"), ("id", "Plateaux Department"), ("it", "Dipartimento degli Altopiani"), ("ja", "プラトー地方"), ("ka", "პლატოს დეპარტამენტი"), ("kn", "ಪ\u{ccd}ಲೇಟ\u{cc6}ಕ\u{ccd}ಸ\u{ccd} ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "플라토 주"), ("lt", "Plato regionas"), ("lv", "Plato departaments"), ("mr", "प\u{94d}ल\u{947}टोक\u{94d}स विभाग"), ("ms", "Plateaux Department"), ("nb", "Plateaux"), ("nl", "Plateaux"), ("no", "Plateaux"), ("pl", "Departament Plateaux"), ("pt", "Plateaux"), ("ro", "Departamentul Plateaux, Republica Congo"), ("ru", "Плато"), ("si", "ප\u{dca}ලටේඋක\u{dca}ස\u{dca}\u{200c} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Plateaux (departement i Kongo-Brazzaville)"), ("ta", "பிலட\u{bc0}ஸ\u{bcd} துறை"), ("te", "ప\u{c4d}ల\u{c3e}టూక\u{c4d}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "พลาโต ด\u{e35}พาทเม\u{e49}น"), ("tr", "Plateaux"), ("uk", "Регіон Плато"), ("ur", "پلاتو محکمہ"), ("vi", "Khu vực hành chính Plateaux"), ("yue", "高原省"), ("yue_Hans", "高原省"), ("zh", "高原省")]),
                        unofficial_name_list: ["Plateaux"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "Cuvette-Ouest",
                        country_alpha2: Alpha2::CG,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.144755), longitude: Some(14.4723301), max_latitude: Some(1.3734011), min_latitude: Some(-1.283519), max_longitude: Some(15.387269), min_longitude: Some(13.88563)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أدارة كوفيت-كويست"), ("bg", "Кювет Уест"), ("bn", "ক\u{9c1}বেতে বিভ\u{9be}গ"), ("ccp", "𑄇\u{11128}𑄅\u{1112a}𑄞𑄑𑄬-𑄃\u{11127}𑄅\u{1112a}𑄠𑄬𑄌\u{11134}"), ("ceb", "Cuvette-Ouest"), ("da", "Cuvette-Ouest Department"), ("de", "Cuvette-Ouest"), ("el", "Κυβέτ-Ουέστ"), ("en", "Cuvette-Ouest"), ("es", "Cuvette-Oeste"), ("fi", "Cuvette-Ouestn departmentti"), ("fr", "Cuvette-Ouest"), ("gu", "ક\u{ac1}વ\u{ac7}ટ- ક\u{acd}પશ\u{acd}ચિમ વિભાગ"), ("he", "קובט מערב"), ("hi", "क\u{941}व\u{947}ट\u{947}-आउस\u{94d}ट विभाग"), ("id", "Cuvette-Ouest"), ("it", "Dipartimento di Cuvette-Ovest"), ("ja", "西キュヴェト地方"), ("ka", "დასავლეთ კიუვეტის დეპარტამენტი"), ("kn", "ಕ\u{ccd}ವ\u{cc6}ವ\u{cc6}ಟ\u{ccd}-ಔಯ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "퀴베트우에스트 주"), ("lt", "Vakarų Kuvetės regionas"), ("lv", "Rietumkivetas departaments"), ("mr", "क\u{94d}य\u{941}व\u{94d}ह\u{947}टी-ऑस\u{94d}ट विभाग"), ("ms", "Cuvette-Ouest Department"), ("nb", "Cuvette-Ouest"), ("nl", "Cuvette-Ouest"), ("no", "Cuvette-Ouest"), ("pl", "Departament Cuvette-Ouest"), ("pt", "Cuvette-Ouest"), ("ro", "Cuvette-Ouest"), ("ru", "Западный Кювет"), ("si", "ක\u{dd4}වෙටේ-අව\u{dd4}ස\u{dca}ට\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Cuvette-Ouest"), ("ta", "சுவேட\u{bcd}டே -ஆயுஸ\u{bcd}ட\u{bcd} துறை"), ("te", "కవ\u{c46}ట\u{c4d}-క\u{c4d}వ\u{c46}స\u{c4d}ట\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ค\u{e39}เวทเท\u{e48} เออส"), ("tr", "Batı Cuvette"), ("uk", "Регіон Західний Кювет"), ("ur", "کؤیت-مغربی محکمہ"), ("vi", "Khu vực hành chính Cuvette-Ouest"), ("zh", "西盆地省")]),
                        unofficial_name_list: ["Cuvette Ouest"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "Pointe-Noire",
                        country_alpha2: Alpha2::CG,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{11127}𑄠𑄬𑄚\u{11133}𑄑\u{11134}-𑄚\u{11127}𑄠\u{11128}𑄢\u{11134}"), ("ceb", "Pointe-Noire (distrito)"), ("en", "Pointe-Noire"), ("sv", "Pointe-Noire")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "2",
                    Subdivision{
                        name: "Lékoumou",
                        country_alpha2: Alpha2::CG,
                        code: "2",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.170382), longitude: Some(13.3587288), max_latitude: Some(-2.1133809), min_latitude: Some(-4.0506931), max_longitude: Some(14.3262581), min_longitude: Some(12.6057329)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ليكومو"), ("bg", "Лекуму"), ("bn", "লেক\u{9c1}মৌ বিভ\u{9be}গ"), ("ccp", "𑄣𑄬𑄇\u{1112f}𑄟\u{1112f}"), ("ceb", "Lékoumou (departamento)"), ("da", "Lekoumou Department"), ("de", "Lékoumou"), ("el", "Λεκουμού"), ("en", "Lékoumou"), ("es", "Lékoumou"), ("fi", "Lékoumoun departmentti"), ("fr", "Lékoumou"), ("gu", "લ\u{ac7}કૌમો વિભાગ"), ("hi", "लिक\u{942}म\u{942} विभाग"), ("id", "Lékoumou"), ("it", "Dipartimento di Lékoumou"), ("ja", "レクム地方"), ("ka", "ლეკუმუს დეპარტამენტი"), ("kn", "ಲ\u{cc6}ಕ\u{ccc}ಮ ಇಲಾಖ\u{cc6}"), ("ko", "레쿠무 주"), ("lt", "Lekumu regionas"), ("lv", "Lekumu novads"), ("mr", "ल\u{947}कौम\u{941}ऊ विभाग"), ("ms", "Jabatan Lekoumou"), ("nb", "Lékoumou"), ("nl", "Lékoumou"), ("no", "Lékoumou"), ("pl", "Departament Lékoumou"), ("pt", "Lékoumou"), ("ro", "Departamentul Lékoumou"), ("ru", "Лекуму"), ("si", "ලෙකෞමෞ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Lékoumou"), ("ta", "லெகோமோஉ துறை"), ("te", "ల\u{c46}క\u{c4b}మూ డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องเลก\u{e39}ม\u{e39}"), ("tr", "Lékoumou"), ("uk", "Лекуму"), ("ur", "لیکؤمؤ محکمہ"), ("vi", "Khu hành chính Lékoumou"), ("yue", "萊庫穆省"), ("yue_Hans", "莱库穆省"), ("zh", "萊庫穆省")]),
                        unofficial_name_list: ["Lékoumou"].to_vec(),
                    }
                ),
                (
                    "5",
                    Subdivision{
                        name: "Kouilou",
                        country_alpha2: Alpha2::CG,
                        code: "5",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.1428413), longitude: Some(11.8891721), max_latitude: Some(-3.512756), min_latitude: Some(-5.0283128), max_longitude: Some(12.7600829), min_longitude: Some(11.2050089)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كويلو"), ("bg", "Куилу"), ("bn", "ক\u{9c1}ইল\u{9c1} বিভ\u{9be}গ"), ("ccp", "𑄇\u{1112f}𑄃\u{11128}𑄣\u{1112f}"), ("ceb", "Région du Kouilou"), ("da", "Kouilou Department"), ("de", "Kouilou"), ("el", "Κουιλού"), ("en", "Kouilou"), ("es", "Kouilou"), ("fi", "Kouiloun depatermentti"), ("fr", "Kouilou"), ("gu", "કૌઇલૌ વિભાગ"), ("hi", "क\u{941}इल\u{942} विभाग"), ("id", "Kouilou"), ("it", "Dipartimento di Kouilou"), ("ja", "クイル地方"), ("ka", "კუილუს დეპარტამენტი"), ("kn", "ಕ\u{ccc}ಯ\u{cbf}ಲೊ ಇಲಾಖ\u{cc6}"), ("ko", "쿠일루 주"), ("lt", "Kvilu regionas"), ("lv", "Kuilu departaments"), ("mr", "कौइलोऊ विभाग"), ("ms", "Kouilou Department"), ("nb", "Kouilou"), ("nl", "Kouilou"), ("no", "Kouilou"), ("pl", "Departament Kouilou"), ("pt", "Kouilou"), ("ro", "Kouilou"), ("ru", "Куилу"), ("si", "කොය\u{dd2}ල\u{dd4} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Région du Kouilou"), ("ta", "கோயிலூ துறை"), ("te", "క\u{c4b}య\u{c3f}ల\u{c4b}వ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องค\u{e39}ล\u{e38}ย"), ("tr", "Kouilou"), ("uk", "Куілу"), ("ur", "کؤیلؤ محکمہ"), ("vi", "Khu vực hành chính Kouilou"), ("zh", "奎盧省")]),
                        unofficial_name_list: ["Kouilou"].to_vec(),
                    }
                ),
                (
                    "7",
                    Subdivision{
                        name: "Likouala",
                        country_alpha2: Alpha2::CG,
                        code: "7",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.043924000000001), longitude: Some(17.668887), max_latitude: Some(3.7030821), min_latitude: Some(-0.7595563), max_longitude: Some(18.649839), min_longitude: Some(16.435879)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة ليكوالا"), ("bg", "Ликуала"), ("bn", "লিক\u{9c1}উল\u{9be} বিভ\u{9be}গ"), ("ccp", "𑄣\u{11128}𑄇\u{1112f}𑄠𑄣"), ("ceb", "Likouala"), ("da", "Likouala Department"), ("de", "Likouala"), ("el", "Λικουαλά"), ("en", "Likouala"), ("es", "Likouala"), ("fa", "لیکوالا"), ("fi", "Likouala"), ("fr", "Likouala"), ("gu", "લિકૌલા વિભાગ"), ("hi", "लिकौला विभाग"), ("id", "Likouala Department"), ("it", "Dipartimento di Likouala"), ("ja", "リクアラ地方"), ("ka", "ლიკუალის დეპარტამენტი"), ("kn", "ಲ\u{cbf}ಕುವಾಲಾ ಇಲಾಖ\u{cc6}"), ("ko", "리쿠알라 주"), ("lt", "Likualos regionas"), ("lv", "Likvalas departaments"), ("mr", "लिकौला विभाग"), ("ms", "Likouala Department"), ("nb", "Likouala"), ("nl", "Likouala"), ("no", "Likouala"), ("pl", "Departament Likouala"), ("pt", "Likouala"), ("ro", "Likouala"), ("ru", "Ликуала"), ("si", "ල\u{dd2}කොඅල\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Likouala"), ("ta", "லிகோல\u{bbe} துறை"), ("te", "ల\u{c3f}కువ\u{c3e}ల\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ล\u{e35}ก\u{e39}อาลา"), ("tr", "Likouala"), ("uk", "Регіон Лікуала"), ("ur", "لیکوالا محکمہ"), ("vi", "Khu vực hành chính Likouala"), ("zh", "利夸拉省")]),
                        unofficial_name_list: ["Likouala"].to_vec(),
                    }
                ),
                (
                    "8",
                    Subdivision{
                        name: "Cuvette",
                        country_alpha2: Alpha2::CG,
                        code: "8",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.2877446), longitude: Some(16.1580937), max_latitude: Some(0.453031), min_latitude: Some(-2.028305), max_longitude: Some(17.5505448), min_longitude: Some(14.349458)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كوفيت"), ("bg", "Кювет"), ("bn", "ক\u{9c1}বেতে বিভ\u{9be}গ²"), ("ccp", "𑄇\u{11128}𑄅\u{1112a}𑄞𑄑𑄬"), ("ceb", "Cuvette"), ("da", "Cuvette Department"), ("de", "Cuvette"), ("el", "Κυβέτ"), ("en", "Cuvette"), ("es", "Departamento de Cuvette"), ("fi", "Cuvetten departmentti"), ("fr", "Cuvette"), ("gu", "ક\u{acd}ય\u{ac1}વ\u{ac7}ટ વિભાગ"), ("hi", "क\u{94d}य\u{942}व\u{947}ट विभाग"), ("id", "Wilayah Cuvette"), ("it", "Dipartimento di Cuvette"), ("ja", "キュヴェト地方"), ("ka", "კიუვეტის დეპარტამენტი"), ("kn", "ಕುವ\u{cc6}ಟ\u{ccd}ಟ\u{cc6} ಇಲಾಖ\u{cc6}"), ("ko", "퀴베트 주"), ("lt", "Kuvetės regionas"), ("lv", "Kivetas departaments"), ("mr", "क\u{94d}य\u{941}व\u{94d}ह\u{947}ट विभाग"), ("ms", "Wilayah Cuvette"), ("nb", "Cuvette"), ("nl", "Cuvette"), ("no", "Cuvette"), ("pl", "Departament Cuvette"), ("pt", "Cuvette"), ("ro", "Cuvette"), ("ru", "Кювет"), ("si", "ක\u{dd4}වෙට\u{dca}ටේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Cuvette"), ("ta", "சுவேட\u{bcd}டே துறை"), ("te", "కువ\u{c46}ట\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตก\u{e39}แวต"), ("tr", "Cuvette"), ("uk", "Регіон Кувет"), ("ur", "کؤیت محکمہ"), ("vi", "Khu vực hành chính Cuvette"), ("yue", "盆地省"), ("yue_Hans", "盆地省"), ("zh", "盆地省")]),
                        unofficial_name_list: ["Cuvette"].to_vec(),
                    }
                ),
                (
                    "9",
                    Subdivision{
                        name: "Niari",
                        country_alpha2: Alpha2::CG,
                        code: "9",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.18427), longitude: Some(12.2547919), max_latitude: Some(-1.858259), min_latitude: Some(-4.919864), max_longitude: Some(13.609751), min_longitude: Some(11.607122)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة نياري"), ("bg", "Ниари"), ("bn", "নিয\u{9bc}\u{9be}রি বিভ\u{9be}গ"), ("ccp", "𑄚\u{1112d}𑄢\u{11128}"), ("ceb", "Région du Niari"), ("da", "Niari Department"), ("de", "Niari (Kongo)"), ("el", "Νιαρί"), ("en", "Niari"), ("es", "Niari"), ("fi", "Niarin hallintoalue"), ("fr", "Niari"), ("gu", "નિઆરી વિભાગ"), ("hi", "नियारी विभाग"), ("id", "Niari Department"), ("it", "Dipartimento di Niari"), ("ja", "ニアリ地方"), ("ka", "ნიარის დეპარტამენტი"), ("kn", "ನ\u{cbf}ಯಾರ\u{cbf} ಇಲಾಖ\u{cc6}"), ("ko", "니아리 주"), ("lt", "Niario regionas"), ("lv", "Niari departaments"), ("mr", "नियारी विभाग"), ("ms", "Niari Department"), ("nb", "Niari"), ("nl", "Niari"), ("no", "Niari"), ("pl", "Departament Niari"), ("pt", "Niari"), ("ro", "Niari"), ("ru", "Ниари"), ("si", "න\u{dd2}ය\u{dcf}ර\u{dd2} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Région du Niari"), ("ta", "ந\u{bc0}அறி துறை"), ("te", "న\u{c3f}య\u{c3e}ర\u{c3f} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "น\u{e35}ยาร\u{e35}"), ("tr", "Niari"), ("uk", "Ніарі"), ("ur", "نیاری محکمہ"), ("vi", "Khu vực hành chính Niari"), ("yue", "尼阿里省"), ("yue_Hans", "尼阿里省"), ("zh", "尼阿里省")]),
                        unofficial_name_list: ["Niari"].to_vec(),
                    }
                ),
                (
                    "BZV",
                    Subdivision{
                        name: "Brazzaville",
                        country_alpha2: Alpha2::CG,
                        code: "BZV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.267778), longitude: Some(15.291944), max_latitude: Some(-4.1233473), min_latitude: Some(-4.3710429), max_longitude: Some(15.3173448), min_longitude: Some(15.1371001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Brazzaville"), ("am", "ብራዛቪል"), ("ar", "برازافيل"), ("az", "Brazzavil"), ("be", "Бразавіль"), ("bg", "Бразавил"), ("bn", "ব\u{9cd}র\u{9be}জ\u{9be}ভিল"), ("bs", "Brazzaville"), ("ca", "Brazzaville"), ("ccp", "𑄝\u{11133}𑄢𑄎𑄞\u{11128}𑄣𑄬"), ("ceb", "Brazzaville (ulohang dakbayan sa Republika sa Congo)"), ("cs", "Brazzaville"), ("cy", "Brazzaville"), ("da", "Brazzaville"), ("de", "Brazzaville"), ("el", "Μπραζαβίλ"), ("en", "Brazzaville"), ("es", "Brazzaville"), ("et", "Brazzaville"), ("eu", "Brazzaville"), ("fa", "برازاویل"), ("fi", "Brazzaville"), ("fr", "Brazzaville"), ("ga", "Brazzaville"), ("gl", "Brazzaville"), ("gu", "બ\u{acd}રાઝાવિલ"), ("ha", "Brazzaville"), ("ha_NE", "Brazzaville"), ("he", "ברזוויל"), ("hi", "ब\u{94d}राजाविल"), ("hr", "Brazzaville"), ("hu", "Brazzaville"), ("hy", "Բրազավիլ"), ("id", "Brazzaville"), ("is", "Brazzaville"), ("it", "Brazzaville"), ("ja", "ブラザヴィル"), ("jv", "Brazzaville"), ("ka", "ბრაზავილი"), ("kk", "Браззавиль"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಜಾವ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "브라자빌"), ("ky", "Браззавиль"), ("lt", "Brazavilis"), ("lv", "Brazavila"), ("mk", "Бразавил"), ("ml", "ബ\u{d4d}ര\u{d3e}സവില\u{d4d}ലെ"), ("mr", "ब\u{94d}राझाव\u{94d}हिल"), ("ms", "Brazzaville"), ("my", "ဘရာဇာဗ\u{102e}းလ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Brazzaville"), ("nl", "Brazzaville"), ("no", "Brazzaville"), ("or", "ବ\u{b4d}ର\u{b3e}ଜ\u{b3e}ଭ\u{b3f}ଲ"), ("pa", "ਬ\u{a4d}ਰਾਜ\u{a3c}ਾਵਿਲ"), ("pl", "Brazzaville"), ("pt", "Brazzaville"), ("ro", "Brazzaville"), ("ru", "Браззавиль"), ("si", "බ\u{dca}\u{200d}රසව\u{dd2}ල\u{dd2}"), ("sk", "Brazzaville"), ("sl", "Brazzaville"), ("so", "Barasafille"), ("sq", "Brazzaville"), ("sr", "Бразавил"), ("sr_Latn", "Brazavil"), ("sv", "Brazzaville"), ("sw", "Brazzaville"), ("ta", "பிர\u{bbe}சவில\u{bcd}லி"), ("te", "బ\u{c4d}ర\u{c47}జ\u{c3e}వ\u{c3f}ల\u{c4d}ల\u{c46}"), ("th", "บราซาว\u{e35}ล"), ("tr", "Brazzaville"), ("uk", "Браззавіль"), ("ur", "برازاویلے"), ("uz", "Brazzavil"), ("vi", "Brazzaville"), ("yo", "Brazzaville"), ("yo_BJ", "Brazzaville"), ("yue", "布拉柴維爾"), ("yue_Hans", "布拉柴维尔"), ("zh", "布拉柴维尔")]),
                        unofficial_name_list: ["Brazzaville"].to_vec(),
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
#[cfg(feature = "cg")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CG,
        alpha3: Alpha3::COG,
        address_format: None,
        continent: Continent::Africa,
        country_code: 242,
        currency_code: CurrencyCode::XAF,
        gec: Some(GEC::CF),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CGO),
        iso_long_name: "The Republic of the Congo",
        iso_short_name: "Congo",
        official_language_list: ["fr", "ln"].to_vec(),
        spoken_language_list: ["fr", "ln"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Congolese"),
        number: "178",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::MiddleAfrica),
        un_locode: "CG",
        unofficial_name_list: [
            "Congo",
            "Kongo",
            "コンゴ共和国",
            "Congo [Republiek]",
            "Congo, Republic of",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Congo"),
            ("af", "Kongo"),
            ("ak", "Congo"),
            ("am", "ኮንጎ"),
            ("an", "Congo"),
            ("ar", "الكونغو"),
            ("as", "কঙ\u{9cd}গো"),
            ("ay", "Congo"),
            ("az", "Konqo"),
            ("ba", "Congo"),
            ("be", "Конга"),
            ("bg", "Конго"),
            ("bi", "Congo"),
            ("bn", "কঙ\u{9cd}গো"),
            ("bn_IN", "কঙ\u{9cd}গো"),
            ("br", "Congo"),
            ("bs", "Kongo"),
            ("ca", "Congo"),
            ("ce", "Congo"),
            ("ch", "Congo"),
            ("cs", "Kongo"),
            ("cv", "Congo"),
            ("cy", "Congo"),
            ("da", "Congo"),
            ("de", "Kongo"),
            ("dv", "Congo"),
            ("dz", "ཀ\u{f7c}ང་ག\u{f7c}།"),
            ("ee", "Congo"),
            ("el", "Κονγκό"),
            ("en", "Congo"),
            ("eo", "Kongo"),
            ("es", "Congo"),
            ("et", "Kongo Vabariik"),
            ("eu", "Kongo"),
            ("fa", "کونگو"),
            ("ff", "Congo"),
            ("fi", "Kongo"),
            ("fo", "Kongo"),
            ("fr", "République du Congo"),
            ("fy", "Congo"),
            ("ga", "An Congó"),
            ("gl", "Congo"),
            ("gn", "Congo"),
            ("gu", "કોન\u{acd}ગો"),
            ("gv", "Congo"),
            ("ha", "Congo"),
            ("he", "קונגו"),
            ("hi", "कॉ\u{902}गो"),
            ("hr", "Kongo"),
            ("ht", "Congo"),
            ("hu", "Kongó"),
            ("hy", "Կոնգո"),
            ("ia", "Congo"),
            ("id", "Congo"),
            ("io", "Congo"),
            ("is", "Kongó"),
            ("it", "Congo"),
            ("iu", "Congo"),
            ("ja", "コンゴ"),
            ("ka", "კონგო"),
            ("ki", "Congo"),
            ("kk", "Конго"),
            ("kl", "Congo"),
            ("km", "ក\u{17bb}ងហ\u{17d2}គោ"),
            ("kn", "ಕಾಂಗೋ"),
            ("ko", "콩고"),
            ("ku", "Kongo"),
            ("kv", "Congo"),
            ("kw", "Congo"),
            ("ky", "Конго"),
            ("lo", "Congo"),
            ("lt", "Kongas"),
            ("lv", "Kongo"),
            ("mi", "Kōngo"),
            ("mk", "Конго"),
            ("ml", "കോംഗോ"),
            ("mn", "Конго"),
            ("mr", "कॉ\u{902}गो"),
            ("ms", "Kongo"),
            ("mt", "Kongo"),
            ("my", "Congo"),
            ("na", "Congo"),
            ("nb", "Kongo"),
            ("ne", "कोङ\u{94d}गो"),
            ("nl", "Congo"),
            ("nn", "Kongo"),
            ("nv", "Congo"),
            ("oc", "Còngo"),
            ("or", "କଙ\u{b4d}ଗୋ"),
            ("pa", "ਕਾ\u{a02}ਗ\u{a4b}"),
            ("pi", "Congo"),
            ("pl", "Kongo"),
            ("ps", "Congo"),
            ("pt", "Congo"),
            ("pt_BR", "Congo"),
            ("ro", "Congo"),
            ("ru", "Конго"),
            ("rw", "Kongo"),
            ("sc", "Congo"),
            ("sd", "Congo"),
            ("si", "කොන\u{dca}ගෝව"),
            ("sk", "Kongo"),
            ("sl", "Kongo"),
            ("so", "Congo"),
            ("sq", "Kongo"),
            ("sr", "Конго"),
            ("sv", "Kongo"),
            ("sw", "Congo"),
            ("ta", "க\u{bbe}ங\u{bcd}கோ"),
            ("te", "క\u{c3e}ంగ\u{c4b}"),
            ("tg", "Конго"),
            ("th", "คองโก"),
            ("ti", "ኮንጎ"),
            ("tk", "Kongo"),
            ("tl", "Congo"),
            ("tr", "Kongo"),
            ("tt", "Конgо"),
            ("ug", "كونگو"),
            ("uk", "Конго"),
            ("ur", "Congo"),
            ("uz", "Congo"),
            ("ve", "Congo"),
            ("vi", "Công-gô"),
            ("wa", "Congo-Brazza"),
            ("wo", "Kongóo"),
            ("xh", "Congo"),
            ("yo", "Congo"),
            ("zh_CN", "刚果"),
            ("zh_HK", "剛果"),
            ("zh_TW", "剛果"),
            ("zu", "Congo"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
