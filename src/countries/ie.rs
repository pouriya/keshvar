// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Ireland

#[cfg(all(feature = "ie", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::IE;
    pub const ALPHA3: Alpha3 = Alpha3::IRL;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 353;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::EI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("IRL");
    pub const ISO_SHORT_NAME: &str = "Ireland";
    pub const ISO_LONG_NAME: &str = "Ireland";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "ga"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "ga"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Irish");
    pub const NUMBER: &str = "372";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("[\\dA-Z]{3} ?[\\dA-Z]{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "IE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Ireland",
        "Irland",
        "Irlande",
        "Irlanda",
        "アイルランド",
        "Ierland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Ireland"),
        ("af", "Ierland"),
        ("ak", "Ireland"),
        ("am", "ጐፘሴሒን፥"),
        ("an", "Ireland"),
        ("ar", "أيرلندا"),
        ("as", "আয়\u{9be}ৰ\u{9cd}লেণ\u{9cd}ড"),
        ("ay", "Ireland"),
        ("az", "İrlandiya"),
        ("ba", "Ireland"),
        ("be", "Ірландыя"),
        ("bg", "Ирландия"),
        ("bi", "Ireland"),
        ("bn", "আয়\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "আয়\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Iwerzhon"),
        ("bs", "Irska"),
        ("ca", "Irlanda"),
        ("ce", "Ирланди"),
        ("ch", "Ireland"),
        ("cs", "Irsko"),
        ("cv", "Ирланди"),
        ("cy", "Iwerddon"),
        ("da", "Irland"),
        ("de", "Irland"),
        ("dv", "އ\u{7a6}ޔ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}ގ\u{7ac} ޖ\u{7aa}މ\u{7b0}ހ\u{7ab}ރ\u{7a8}އ\u{7b0}ޔ\u{7a7}"),
        ("dz", "ཨའ\u{f72}་ར\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"),
        ("ee", "Ireland"),
        ("el", "Ιρλανδία"),
        ("en", "Ireland"),
        ("eo", "Irlando"),
        ("es", "Irlanda"),
        ("et", "Iirimaa"),
        ("eu", "Irlanda"),
        ("fa", "ایرلند"),
        ("ff", "Irlannda"),
        ("fi", "Irlanti"),
        ("fo", "Írland"),
        ("fr", "Irlande"),
        ("fy", "Ierlân"),
        ("ga", "Éire"),
        ("gl", "Irlanda"),
        ("gn", "Ireland"),
        ("gu", "આયરલ\u{ac7}ન\u{acd}ડ"),
        ("gv", "Pobblaght Nerin"),
        ("ha", "Ireland"),
        ("he", "אירלנד"),
        ("hi", "आयरल\u{948}ण\u{94d}ड"),
        ("hr", "Irska"),
        ("ht", "Ilann"),
        ("hu", "Írország"),
        ("hy", "Իռլանդիա"),
        ("ia", "Irlanda"),
        ("id", "Irlandia"),
        ("io", "Republiko di Irlando"),
        ("is", "Írland"),
        ("it", "Irlanda"),
        ("iu", "Ireland"),
        ("ja", "アイルランド"),
        ("ka", "ირლანდია"),
        ("ki", "Ireland"),
        ("kk", "Ирландия"),
        ("kl", "Ireland"),
        ("km", "អៀរឡង\u{17cb}"),
        ("kn", "ಐರ\u{ccd}ಲ\u{ccd}ಯಂಡ\u{ccd}"),
        ("ko", "아일랜드"),
        ("ku", "Îrlanda"),
        ("kv", "Ирландия"),
        ("kw", "Repoblek Wordhen"),
        ("ky", "Ирландия"),
        ("lo", "ປະເທດອຽກລ\u{eb1}ງ"),
        ("lt", "Airija"),
        ("lv", "Īrija"),
        ("mi", "Airangi"),
        ("mk", "Ирска"),
        ("ml", "അയര\u{d4d}\u{200d}ലണ\u{d4d}ട\u{d4d}"),
        ("mn", "Ирланд"),
        ("mr", "आयर\u{94d}ल\u{902}ड"),
        ("ms", "Ireland"),
        ("mt", "Irlanda"),
        ("my", "အ\u{102d}\u{102f}င\u{103a}ယာလန\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Ripubrikit Airerand"),
        ("nb", "Irland"),
        ("ne", "आयरल\u{94d}याण\u{94d}ड"),
        ("nl", "Ierland"),
        ("nn", "Irland"),
        ("nv", "Bitsiighaʼ Łichííʼí Bikéyah"),
        ("oc", "Irlanda"),
        ("or", "ଆୟରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਆਈਰਲ\u{a48}\u{a02}ਡ"),
        ("pi", "Ireland"),
        ("pl", "Irlandia"),
        ("ps", "د آيرلېنډ جمهوريت"),
        ("pt", "Irlanda"),
        ("pt_BR", "Irlanda"),
        ("ro", "Irlanda"),
        ("ru", "Ирландия"),
        ("rw", "Irilande"),
        ("sc", "Irlanda"),
        ("sd", "Ireland"),
        ("si", "අයර\u{dca}ලන\u{dca}තය"),
        ("sk", "Írsko"),
        ("sl", "Irska"),
        ("so", "Ayrlaanda"),
        ("sq", "Irlandë"),
        ("sr", "Ирска"),
        ("sv", "Irland"),
        ("sw", "Ireland"),
        ("ta", "அயர\u{bcd}ல\u{bbe}ந\u{bcd}து"),
        ("te", "ఐర\u{c4d}ల\u{c3e}ండ\u{c4d}"),
        ("tg", "Ирландия"),
        ("th", "ไอร\u{e4c}แลนด\u{e4c}"),
        ("ti", "አየርላንድ"),
        ("tk", "Irlandiýa"),
        ("tl", "Ireland"),
        ("tr", "İrlanda"),
        ("tt", "İреланд, İрландиа"),
        ("ug", "ئىرېلاندىيە"),
        ("uk", "Ірландія"),
        ("ur", "جمہوریہ آئرستان"),
        ("uz", "Irlandiya"),
        ("ve", "Ireland"),
        ("vi", "Ái Nhĩ Lan"),
        ("wa", "Irlande"),
        ("wo", "Irlaand"),
        ("xh", "Ireland"),
        ("yo", "Írẹ\u{301}lándì"),
        ("zh_CN", "爱尔兰"),
        ("zh_HK", "愛爾蘭"),
        ("zh_TW", "愛爾蘭"),
        ("zu", "I-Ayilendi"),
];
    #[cfg(all(feature = "ie", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 53.1423672;
        pub const LONGITUDE: f64 = -7.692053599999999;
        pub const MAX_LATITUDE: f64 = 55.38294149999999;
        pub const MAX_LONGITUDE: f64 = -5.431909999999999;
        pub const MIN_LATITUDE: f64 = 51.4475448;
        pub const MIN_LONGITUDE: f64 = -10.4800237;
        pub const NORTHEAST_LATITUDE: f64 = 55.38294149999999;
        pub const NORTHEAST_LONGITUDE: f64 = -5.431909999999999;
        pub const SOUTHWEST_LATITUDE: f64 = 51.4475448;
        pub const SOUTHWEST_LONGITUDE: f64 = -10.4800237;
    }
}
#[cfg(all(feature = "ie", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 53.1423672,
            longitude: -7.692053599999999,
            max_latitude: 55.38294149999999,
            max_longitude: -5.431909999999999,
            min_latitude: 51.4475448,
            min_longitude: -10.4800237,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 55.38294149999999,
                    longitude: -5.431909999999999,
                },
                southwest: CountryGeoBound {
                    latitude: 51.4475448,
                    longitude: -10.4800237,
                },
            },
        }
    }
}

#[cfg(all(feature = "ie", feature = "subdivisions"))]
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
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::IE,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8968917), longitude: Some(-8.4863157), max_latitude: Some(51.93586), min_latitude: Some(51.8561501), max_longitude: Some(-8.380579899999999), min_longitude: Some(-8.54552)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوناكت"), ("be", "Конахт"), ("bg", "Конахт"), ("ca", "Connacht"), ("ccp", "𑄇\u{1112e}𑄚𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Connaught"), ("cs", "Connacht"), ("cy", "Connacht"), ("da", "Connacht"), ("de", "Connacht"), ("el", "Κόναχτ"), ("en", "Connacht"), ("es", "Connacht"), ("et", "Connachti provints"), ("eu", "Connacht"), ("fa", "کوناکت"), ("fi", "Connacht"), ("fr", "Connacht"), ("ga", "Cúige Chonnacht"), ("gl", "Connacht - Cúige Connachta"), ("he", "קונכט"), ("hr", "Cúige Chonnacht"), ("hu", "Connacht"), ("hy", "Կոննախտ"), ("id", "Connacht"), ("it", "Connacht"), ("ja", "コノート"), ("ka", "კონაქტი"), ("ko", "코노트"), ("lt", "Konachtas"), ("mk", "Конахт"), ("nb", "Connacht"), ("nl", "Connacht"), ("no", "Connacht"), ("pl", "Connacht"), ("pt", "Connacht"), ("ru", "Коннахт"), ("sk", "Connacht"), ("sr", "Конот"), ("sr_Latn", "Konot"), ("sv", "Connacht"), ("tr", "Connacht"), ("uk", "Коннахт"), ("ur", "کوناکٹ"), ("vi", "Connacht"), ("yue", "干諾省"), ("yue_Hans", "干诺省"), ("zh", "康諾特省")]),
                        unofficial_name_list: ["Connaught"].to_vec(),
                    }
                ),
                (
                    "CE",
                    Subdivision{
                        name: "CE",
                        country_alpha2: Alpha2::IE,
                        code: "CE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.86245280000001), longitude: Some(-9.0471645), max_latitude: Some(53.1686759), min_latitude: Some(52.5545323), max_longitude: Some(-8.2839238), min_longitude: Some(-9.9386938)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كلير"), ("be", "графства Клэр"), ("bg", "Клеър"), ("bn", "ক\u{9be}উন\u{9cd}টি ক\u{9cd}লেয\u{9bc}\u{9be}র"), ("ca", "Comtat de Clare"), ("ccp", "𑄇\u{11133}𑄣𑄢𑄬"), ("ceb", "An Clár"), ("cs", "Hrabství Clare"), ("cy", "Swydd Clare"), ("da", "County Clare"), ("de", "County Clare"), ("el", "Κομητεία Κλέιρ"), ("en", "Clare"), ("es", "Condado de Clare"), ("et", "Clare’i krahvkond"), ("eu", "Clareko konderria"), ("fa", "شهرستان کلر"), ("fi", "Claren kreivikunta"), ("fr", "Comté de Clare"), ("ga", "Contae an Chláir"), ("gl", "Condado de Clare"), ("gu", "કાઉન\u{acd}ટી ક\u{acd}લ\u{ac7}ર"), ("he", "מחוז קלייר"), ("hi", "काउ\u{902}टी क\u{94d}ल\u{947}यर"), ("hu", "Clare megye"), ("id", "County Clare"), ("is", "County Clare"), ("it", "Clare"), ("ja", "クレア州"), ("ka", "კლერის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕ\u{ccd}ಲೇರ\u{ccd}"), ("ko", "클래어 주"), ("lt", "Klero grafystė"), ("lv", "Klēras grāfiste"), ("mk", "Клер"), ("mr", "काउ\u{902}टी क\u{94d}ल\u{947}अर"), ("ms", "County Clare"), ("nb", "Clare"), ("nl", "County Clare"), ("no", "Clare"), ("pl", "Clare (hrabstwo)"), ("pt", "Condado de Clare"), ("ro", "Comitatul Clare"), ("ru", "Клэр"), ("si", "ක\u{dca}ලෙය\u{dcf}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Clare"), ("sr", "Клер"), ("sr_Latn", "Kler"), ("sv", "Clare"), ("sw", "Wilaya ya Clare"), ("ta", "கவுண\u{bcd}டி கலர\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} క\u{c4d}ల\u{c47}ర\u{c4d}"), ("th", "แคลร\u{e4c}"), ("tr", "Clare County"), ("uk", "Клер"), ("ur", "کاؤنٹی کلیئر"), ("vi", "Hạt Clare"), ("zh", "克莱尔郡")]),
                        unofficial_name_list: ["An Clár"].to_vec(),
                    }
                ),
                (
                    "CN",
                    Subdivision{
                        name: "CN",
                        country_alpha2: Alpha2::IE,
                        code: "CN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.9897186), longitude: Some(-7.3633319), max_latitude: Some(54.00315670000001), min_latitude: Some(53.9700812), max_longitude: Some(-7.3168015), min_longitude: Some(-7.394294299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كافان"), ("be", "графства Каван"), ("bg", "Каван"), ("bn", "ক\u{9be}উন\u{9cd}টি কভেন"), ("ca", "Comtat de Cavan"), ("ccp", "𑄇\u{11133}𑄠𑄞\u{11127}𑄚\u{11134}"), ("ceb", "An Cabhán"), ("cs", "Hrabství Cavan"), ("cy", "Swydd Cavan"), ("da", "County Cavan"), ("de", "County Cavan"), ("el", "Κομητεία Κάβαν"), ("en", "Cavan"), ("es", "Condado de Cavan"), ("et", "Cavani krahvkond"), ("eu", "Cavan konderria"), ("fa", "شهرستان کاوان"), ("fi", "Cavanin kreivikunta"), ("fr", "comté de Cavan"), ("ga", "Contae an Chabháin"), ("gl", "Condado de Cavan"), ("gu", "કાઉન\u{acd}ટી કાવાન"), ("he", "מחוז קאוואן"), ("hi", "काउ\u{902}टी क\u{948}वन"), ("hu", "Cavan megye"), ("id", "County Cavan"), ("it", "Cavan"), ("ja", "キャバン州"), ("ka", "კავანის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕ\u{ccd}ಯಾವನ\u{ccd}"), ("ko", "캐번 주"), ("lt", "Kavano grafystė"), ("lv", "Kevanas grāfiste"), ("mk", "Каван"), ("mr", "काउ\u{902}टी कावन"), ("ms", "County Cavan"), ("nb", "Cavan"), ("nl", "County Cavan"), ("no", "Cavan"), ("pl", "Cavan"), ("pt", "Condado de Cavan"), ("ro", "Comitatul Cavan"), ("ru", "Каван"), ("si", "කැවන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Cavan"), ("sl", "okrožje Cavan, Irska"), ("sr", "Каван"), ("sr_Latn", "Kavan"), ("sv", "Cavan"), ("sw", "Wilaya ya Cavan"), ("ta", "கவுண\u{bcd}டி க\u{bbe}வல\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} క\u{c47}వ\u{c3e}న\u{c4d}"), ("th", "เม\u{e37}องหลวงคาวาน"), ("tr", "Cavan County"), ("uk", "Каван"), ("ur", "کاؤنٹی کاوان"), ("vi", "Hạt Cavan"), ("zh", "卡文郡")]),
                        unofficial_name_list: ["An Cabhán"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::IE,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8985), longitude: Some(8.4756), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كورك"), ("be", "Графства Корк"), ("bg", "Корк"), ("bn", "ক\u{9be}উন\u{9cd}টি কর\u{9cd}ক"), ("ca", "Comtat de Cork"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄇\u{11134}"), ("ceb", "County Cork"), ("cs", "Hrabství Cork"), ("cy", "Swydd Corc"), ("da", "County Cork"), ("de", "County Cork"), ("el", "Κομητεία Κορκ"), ("en", "Cork"), ("es", "Condado de Cork"), ("et", "Corki krahvkond"), ("eu", "Corkeko konderria"), ("fa", "شهرستان کورک"), ("fi", "Corkin kreivikunta"), ("fr", "comté de Cork"), ("ga", "Contae Chorcaí"), ("gl", "Condado de Cork - Chorcaí"), ("gu", "કાઉન\u{acd}ટી કૉર\u{acd}ક"), ("he", "מחוז קורק"), ("hi", "काउ\u{902}टी कॉर\u{94d}क"), ("hu", "Cork megye"), ("hy", "Կորկ"), ("id", "County Cork"), ("is", "County Cork"), ("it", "Cork"), ("ja", "コーク州"), ("ka", "კორკის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕಾರ\u{ccd}ಕ\u{ccd}"), ("ko", "코크 주"), ("lt", "Korko grafystė"), ("lv", "Korkas grāfiste"), ("mk", "Корк"), ("mr", "काउ\u{902}टी कॉर\u{94d}क"), ("ms", "County Cork"), ("nb", "Cork"), ("nl", "County Cork"), ("no", "Cork"), ("pl", "Cork (hrabstwo)"), ("pt", "Condado de Cork"), ("ro", "Comitatul Cork"), ("ru", "Корк"), ("si", "කොර\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Contae Chorcaí"), ("sl", "County Cork"), ("sq", "Qarku Cork"), ("sr", "Корк"), ("sr_Latn", "Kork"), ("sv", "Cork"), ("sw", "Wilaya ya Cork"), ("ta", "கவுண\u{bcd}டி க\u{bbe}ர\u{bcd}க\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} క\u{c3e}ర\u{c4d}క\u{c4d}"), ("th", "เม\u{e37}องหลวงคอร\u{e4c}ก"), ("tr", "Cork Kontluğu"), ("uk", "Корк"), ("ur", "کاؤنٹی کورک"), ("vi", "Hạt Cork"), ("zh", "科克郡")]),
                        unofficial_name_list: ["Corcaigh", "County Cork"].to_vec(),
                    }
                ),
                (
                    "CW",
                    Subdivision{
                        name: "CW",
                        country_alpha2: Alpha2::IE,
                        code: "CW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.7189747), longitude: Some(-6.8503703), max_latitude: Some(52.9179628), min_latitude: Some(52.4638685), max_longitude: Some(-6.5049107), min_longitude: Some(-7.107934699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كارلو"), ("be", "графства Карлау"), ("bg", "Карлоу"), ("bn", "ক\u{9be}উন\u{9cd}টি ক\u{9be}রলো"), ("ca", "Comtat de Carlow"), ("ccp", "𑄇𑄢\u{11134}𑄣\u{1112e}"), ("ceb", "County Carlow"), ("cs", "Hrabství Carlow"), ("cy", "Swydd Carlow"), ("da", "County Carlow"), ("de", "County Carlow"), ("el", "Κομητεία Κάρλοου"), ("en", "Carlow"), ("es", "Condado de Carlow"), ("et", "Carlow’ krahvkond"), ("eu", "Carlowko konderria"), ("fa", "کارلو ( ایرلند)"), ("fi", "Carlow’n kreivikunta"), ("fr", "Comté de Carlow"), ("ga", "Contae Cheatharlach"), ("gl", "Condado de Carlow"), ("gu", "કાઉન\u{acd}ટી કાર\u{acd}લો"), ("he", "מחוז קרלו"), ("hi", "कार\u{94d}लो काउ\u{902}टी"), ("hu", "Carlow megye"), ("id", "County Carlow"), ("it", "Carlow"), ("ja", "カーロウ州"), ("ka", "კარლოუს საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕಾರ\u{ccd}ಲೊ"), ("ko", "칼로 주"), ("lt", "Karlou grafystė"), ("lv", "Kārlovas grāfiste"), ("mk", "Карлоу"), ("mr", "काउ\u{902}टी कार\u{94d}लो"), ("ms", "County Carlow"), ("nb", "Carlow"), ("nl", "County Carlow"), ("no", "Carlow"), ("pl", "Carlow (hrabstwo)"), ("pt", "Condado de Carlow"), ("ro", "Comitatul Carlow"), ("ru", "Карлоу"), ("si", "ක\u{dcf}ර\u{dca}ලෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Contae Cheatharlach"), ("sl", "grofija Carlow"), ("sq", "Carlow"), ("sr", "Карлоу"), ("sr_Latn", "Karlou"), ("sv", "Carlow"), ("sw", "Wilaya ya Carlow"), ("ta", "கவுண\u{bcd}டி கர\u{bcd}லோவ\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}ల\u{c4b}వ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "โคโซวสกา คาเมน\u{e34}ซา"), ("tr", "Carlow County"), ("uk", "Карлоу"), ("ur", "کاؤنٹی کارلو"), ("vi", "Hạt Carlow"), ("zh", "卡洛郡")]),
                        unofficial_name_list: ["Ceatharlach"].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::IE,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.3498053), longitude: Some(-6.2603097), max_latitude: Some(53.42521010000001), min_latitude: Some(53.22343009999999), max_longitude: Some(-6.05255), min_longitude: Some(-6.450839999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دبلن"), ("be", "Дублін"), ("bg", "Дъблин"), ("ca", "Comtat de Dublín"), ("ccp", "𑄓\u{1112a}𑄛\u{11134}𑄣\u{11128}𑄚\u{11134}"), ("cs", "Hrabství Dublin"), ("cy", "Swydd Dulyn"), ("da", "County Dublin"), ("de", "County Dublin"), ("el", "Κομητεία Δουβλίνου"), ("en", "Dublin"), ("es", "Condado de Dublín"), ("et", "Dublini krahvkond"), ("eu", "Dublingo konderria"), ("fa", "شهرستان دوبلین"), ("fi", "Dublinin kreivikunta"), ("fr", "Comté de Dublin"), ("ga", "Contae Bhaile Átha Cliath"), ("gl", "Condado de Dublín - Bhaile Átha Cliath"), ("he", "מחוז דבלין"), ("hy", "Դուբլին"), ("id", "County Dublin"), ("is", "County Dublin"), ("it", "Dublino"), ("ja", "ダブリン州"), ("ka", "დუბლინის საგრაფო"), ("ko", "더블린 주"), ("lt", "Dublino grafystė"), ("mk", "Даблин"), ("nb", "Dublin"), ("nl", "County Dublin"), ("no", "Dublin"), ("pl", "Dublin (hrabstwo)"), ("pt", "Condado de Dublin"), ("ro", "Comitatul Dublin"), ("ru", "Дублин"), ("sk", "Dublin"), ("sr", "Даблин"), ("sr_Latn", "Dablin"), ("sv", "Dublin"), ("tr", "Dublin ili"), ("uk", "Дублін"), ("ur", "کاؤنٹی ڈبلن"), ("zh", "都柏林地區")]),
                        unofficial_name_list: ["Baile Átha Cliath", "Átha Cliath"].to_vec(),
                    }
                ),
                (
                    "DL",
                    Subdivision{
                        name: "DL",
                        country_alpha2: Alpha2::IE,
                        code: "DL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.6541972), longitude: Some(-8.110546), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دونيجال"), ("be", "графства Донегал"), ("bg", "Донигал"), ("bn", "ক\u{9be}উন\u{9cd}টি ডোনেগ\u{9be}ল"), ("ca", "Comtat de Donegal"), ("ccp", "𑄓\u{11127}𑄚\u{11134}𑄉\u{11133}𑄠𑄣\u{11134}"), ("ceb", "County Donegal"), ("cs", "Hrabství Donegal"), ("cy", "Swydd Donegal"), ("da", "County Donegal"), ("de", "County Donegal"), ("el", "Κομητεία Ντονεγκάλ"), ("en", "Donegal"), ("es", "Condado de Donegal"), ("et", "Donegali krahvkond"), ("eu", "Donegalgo konderria"), ("fa", "شهرستان دانیگول"), ("fi", "Donegalin kreivikunta"), ("fr", "Comté de Donegal"), ("ga", "Contae Dhún na nGall"), ("gl", "Condado de Donegal"), ("gu", "કાઉન\u{acd}ટી ડોન\u{ac7}ગલ"), ("he", "מחוז דונגל"), ("hi", "काउ\u{902}टी डोन\u{947}गल"), ("hr", "Dhún na nGall"), ("hu", "Donegal megye"), ("id", "County Donegal"), ("it", "Donegal"), ("ja", "ドニゴール州"), ("ka", "დონეგალის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಡೊನ\u{cc6}ಗಲ\u{ccd}"), ("ko", "도니골 주"), ("lt", "Donegolo grafystė"), ("lv", "Donegolas grāfiste"), ("mk", "Донегол"), ("mr", "काउ\u{902}टी डोन\u{947}गल"), ("ms", "County Donegal"), ("nb", "Donegal"), ("nl", "County Donegal"), ("no", "Donegal"), ("pl", "Donegal (hrabstwo)"), ("pt", "Condado de Donegal"), ("ro", "Comitatul Donegal"), ("ru", "Донегол"), ("si", "ඩොනේගල\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Donegal"), ("sl", "okrožje Donegal, Irska"), ("sr", "Донегол"), ("sr_Latn", "Donegol"), ("sv", "Donegal"), ("sw", "Wilaya ya Donegal"), ("ta", "கவுண\u{bcd}டி டோனெகல\u{bcd}"), ("te", "డ\u{c4a}న\u{c46}గల\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ดอน\u{e34}กอล"), ("tr", "Donegal Kontluğu"), ("uk", "Донегол"), ("ur", "کاؤنٹی ڈانیگول"), ("vi", "Hạt Donegal"), ("zh", "多尼戈爾郡")]),
                        unofficial_name_list: ["Dún na nGall"].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "G",
                        country_alpha2: Alpha2::IE,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.270668), longitude: Some(-9.0567905), max_latitude: Some(53.31947049999999), min_latitude: Some(53.2485394), max_longitude: Some(-8.954820699999999), min_longitude: Some(-9.1426873)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جلوي"), ("be", "графства Голуэй"), ("bg", "Голуей"), ("bn", "ক\u{9be}উন\u{9cd}টি গলওয\u{9bc}ে"), ("ca", "Comtat de Galway"), ("ccp", "𑄉\u{11127}𑄣\u{11134}𑄃\u{1112e}𑄠𑄬"), ("ceb", "County Galway"), ("cs", "Hrabství Galway"), ("cy", "Swydd Galway"), ("da", "County Galway"), ("de", "County Galway"), ("el", "Κομητεία Γκόλγουεϊ"), ("en", "Galway"), ("es", "Condado de Galway"), ("eu", "Galwayko konderria"), ("fa", "شهرستان گالوی"), ("fi", "Galwayn kreivikunta"), ("fr", "Comté de Galway"), ("ga", "Contae na Gaillimhe"), ("gl", "Condado de Galway"), ("gu", "કાઉન\u{acd}ટી ગ\u{ac7}લવ\u{ac7}"), ("he", "מחוז גולוויי"), ("hi", "काउ\u{902}टी गॉलव\u{947}"), ("hu", "Galway megye"), ("id", "County Galway"), ("is", "County Galway"), ("it", "Contea di Galway"), ("ja", "ゴールウェイ州"), ("ka", "გოლუეის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಗಾಲ\u{ccd}ವೇ"), ("ko", "골웨이 주"), ("lt", "Golvėjaus grafystė"), ("lv", "Golvejas grāfiste"), ("mk", "Голвеј"), ("mr", "काउ\u{902}टी ग\u{945}लव\u{947}"), ("ms", "County Galway"), ("nb", "Galway"), ("nl", "County Galway"), ("no", "Galway"), ("pl", "Galway"), ("pt", "Condado de Galway"), ("ro", "Comitatul Galway"), ("ru", "Голуэй"), ("si", "ගල\u{dca}වේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Galway"), ("sl", "Grofija Galway"), ("sr", "Голвеј"), ("sr_Latn", "Golvej"), ("sv", "Galway"), ("sw", "Wilaya ya Galway"), ("ta", "கவுண\u{bcd}டி க\u{bbe}ல\u{bcd}வே"), ("te", "క\u{c4c}ంట\u{c40} గ\u{c3e}ల\u{c4d}వ\u{c47}"), ("th", "เม\u{e37}องหลวงก\u{e31}ลเวย\u{e4c}"), ("tr", "Galway County"), ("uk", "Голвей"), ("ur", "کاؤنٹی گالوے"), ("vi", "Hạt Galway"), ("zh", "戈尔韦郡")]),
                        unofficial_name_list: ["Gaillimh"].to_vec(),
                    }
                ),
                (
                    "KE",
                    Subdivision{
                        name: "KE",
                        country_alpha2: Alpha2::IE,
                        code: "KE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1589342), longitude: Some(-6.9095683), max_latitude: Some(53.1828601), min_latitude: Some(53.15196), max_longitude: Some(-6.8829899), min_longitude: Some(-6.92259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Graafskap Kildare"), ("ar", "مقاطعة كيلدير"), ("be", "Графства Кілдэр"), ("bg", "Килдеър"), ("bn", "ক\u{9be}উন\u{9cd}টি কিল\u{9cd}ড\u{9be}র"), ("ca", "Comtat de Kildare"), ("ccp", "𑄇\u{11128}𑄣\u{11134}𑄓𑄢𑄬"), ("ceb", "Kildare (kondado)"), ("cs", "Hrabství Kildare"), ("cy", "Swydd Kildare"), ("da", "County Kildare"), ("de", "County Kildare"), ("el", "Κομητεία Κιλντέιρ"), ("en", "Kildare"), ("es", "Condado de Kildare"), ("et", "Kildare’i krahvkond"), ("eu", "Kildareko konderria"), ("fa", "شهراستان کیلدر"), ("fi", "Kildaren kreivikunta"), ("fr", "Comté de Kildare"), ("ga", "Contae Chill Dara"), ("gl", "Condado de Kildare"), ("gu", "કાઉન\u{acd}ટી કિલ\u{acd}ડર"), ("he", "מחוז קילדייר"), ("hi", "काउ\u{902}टी किल\u{94d}ड\u{947}र\u{947}"), ("hu", "Kildare megye"), ("id", "County Kildare"), ("it", "Kildare"), ("ja", "キルデア州"), ("ka", "კილდერის საგრაფო"), ("kk", "Килдэйр"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕ\u{cbf}ಲ\u{ccd}ಡೇರ\u{ccd}"), ("ko", "킬데어 주"), ("lt", "Kildero grafystė"), ("lv", "Kildēras grāfiste"), ("mk", "Килдер"), ("mr", "काउ\u{902}टी क\u{947}ल\u{94d}ल\u{94d}डर"), ("ms", "County Kildare"), ("nb", "Kildare"), ("nl", "County Kildare"), ("no", "Kildare"), ("pl", "Kildare (hrabstwo)"), ("pt", "Condado de Kildare"), ("ro", "Comitatul Kildare"), ("ru", "Килдэр"), ("si", "ක\u{dd2}ල\u{dca}දරේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kildare"), ("sr", "Килдер"), ("sr_Latn", "Kilder"), ("sv", "Kildare"), ("sw", "Wilaya ya Kildare"), ("ta", "கவுண\u{bcd}டி கில\u{bcd}டர\u{bcd}"), ("te", "క\u{c3f}ల\u{c4d}డ\u{c3e}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องค\u{e34}ลแดร\u{e4c}"), ("tr", "County Kildare"), ("uk", "Кілдер"), ("ur", "کاؤنٹی کلڈیئر"), ("vi", "Hạt Kildare"), ("zh", "基尔代尔郡")]),
                        unofficial_name_list: ["Cill Dara"].to_vec(),
                    }
                ),
                (
                    "KK",
                    Subdivision{
                        name: "KK",
                        country_alpha2: Alpha2::IE,
                        code: "KK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6541454), longitude: Some(-7.2447879), max_latitude: Some(52.67698000000001), min_latitude: Some(52.63111), max_longitude: Some(-7.20547), min_longitude: Some(-7.2731599)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كيكني"), ("be", "Кілкені"), ("bg", "Килкени"), ("bn", "ক\u{9be}উন\u{9cd}টি কিলকেনি"), ("ca", "Comtat de Kilkenny"), ("ccp", "𑄇\u{11128}𑄣\u{11134}𑄇𑄬𑄚\u{11128}"), ("cs", "Hrabství Kilkenny"), ("cy", "Swydd Kilkenny"), ("da", "County Kilkenny"), ("de", "County Kilkenny"), ("el", "Κομητεία Κίλκενι"), ("en", "Kilkenny"), ("es", "Condado de Kilkenny"), ("et", "Kilkenny krahvkond"), ("eu", "Kilkennyko konderria"), ("fa", "شهرستان کیلکنی"), ("fi", "Kilkennyn kreivikunta"), ("fr", "comté de Kilkenny"), ("ga", "Contae Chill Chainnigh"), ("gl", "Condado de Kilkenny"), ("gu", "કાઉન\u{acd}ટી કિલક\u{ac7}ની"), ("he", "מחוז קילקני"), ("hi", "काउ\u{902}टी किल\u{94d}क\u{947}नी"), ("hu", "Kilkenny megye"), ("hy", "Կիլկենի"), ("id", "County Kilkenny"), ("it", "Kilkenny"), ("ja", "キルケニー州"), ("ka", "კილკენის საგრაფო"), ("kk", "Килкенни"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕ\u{cbf}ಲ\u{ccd}ಕ\u{cc6}ನ\u{cbf}"), ("ko", "킬케니 주"), ("lt", "Kilkenio grafystė"), ("lv", "Kilkeni grāfiste"), ("mk", "Килкени"), ("mr", "काउ\u{902}टी किलक\u{947}नी"), ("ms", "County Kilkenny"), ("nb", "Kilkenny"), ("nl", "County Kilkenny"), ("no", "Kilkenny"), ("pl", "Kilkenny (hrabstwo)"), ("pt", "Condado de Kilkenny"), ("ro", "Comitatul Kilkenny"), ("ru", "Килкенни"), ("si", "ක\u{dd2}ල\u{dca}කෙන\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kilkenny"), ("sl", "okrožje Kilkenny"), ("sr", "Килкени"), ("sr_Latn", "Kilkeni"), ("sv", "Kilkenny"), ("sw", "Wilaya ya Kilkenny"), ("ta", "கவுண\u{bcd}டி கில\u{bcd}கென\u{bcd}னி"), ("te", "క\u{c3f}ల\u{c4d}క\u{c46}న\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องหลวงค\u{e34}ลเค\u{e47}นน\u{e35}"), ("tr", "Kilkenny County"), ("uk", "Кілкенні"), ("ur", "کاؤنٹی کلکینی"), ("vi", "Hạt Kilkenny"), ("zh", "基爾肯尼郡")]),
                        unofficial_name_list: ["Cill Chainnigh"].to_vec(),
                    }
                ),
                (
                    "KY",
                    Subdivision{
                        name: "KY",
                        country_alpha2: Alpha2::IE,
                        code: "KY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.15446069999999), longitude: Some(-9.5668632), max_latitude: Some(52.59184), min_latitude: Some(51.6883223), max_longitude: Some(-9.118786), min_longitude: Some(-10.6183626)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كري"), ("bg", "Кери"), ("bn", "ক\u{9be}উন\u{9cd}টি কেরি"), ("ca", "Comtat de Kerry"), ("ccp", "𑄇𑄬𑄢\u{11128}"), ("ceb", "Ciarraí"), ("cs", "Hrabství Kerry"), ("cy", "Swydd Kerry"), ("da", "County Kerry"), ("de", "Kerry"), ("el", "Κομητεία Κέρι"), ("en", "Kerry"), ("es", "Condado de Kerry"), ("et", "Kerry krahvkond"), ("eu", "Kerryko konderria"), ("fa", "شهرستان کری"), ("fi", "Kerryn kreivikunta"), ("fr", "comté de Kerry"), ("ga", "Contae Chiarraí"), ("gl", "Condado de Kerry"), ("gu", "કાઉન\u{acd}ટી ક\u{ac7}રી"), ("he", "מחוז קרי"), ("hi", "काउ\u{902}टी क\u{947}री"), ("hu", "Kerry megye"), ("id", "County Kerry"), ("it", "Kerry"), ("ja", "ケリー州"), ("ka", "კერის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಕ\u{cc6}ರ\u{ccd}ರ\u{cbf}"), ("ko", "케리 주"), ("lt", "Kerio grafystė"), ("lv", "Kerri grāfiste"), ("mk", "Кери"), ("mr", "काउ\u{902}टी क\u{947}री"), ("ms", "County Kerry"), ("nb", "Kerry"), ("nl", "County Kerry"), ("no", "Kerry"), ("pl", "Kerry"), ("pt", "Condado de Kerry"), ("ro", "Comitatul Kerry"), ("ru", "Керри"), ("si", "කෙර\u{dd3} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Contae Chiarraí"), ("sl", "Okrožje Kerry"), ("sr", "Кери"), ("sr_Latn", "Keri"), ("sv", "Kerry"), ("sw", "Wilaya ya Kerry"), ("ta", "கவுண\u{bcd}டி கெர\u{bcd}ரி"), ("te", "క\u{c46}ర\u{c4d}ర\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เคาต\u{e35} เคอร\u{e4c}ร\u{e35}"), ("tr", "Kerry County"), ("uk", "Керрі"), ("ur", "کاؤنٹی کیری"), ("vi", "Hạt Kerry"), ("zh", "凱里郡")]),
                        unofficial_name_list: ["Ciarraighe", "Ciarraí"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::IE,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاينستر"), ("be", "Ленстэр"), ("bg", "Ленстър"), ("ca", "Leinster"), ("ccp", "𑄣𑄬\u{1112d}𑄚\u{11134}𑄌\u{11133}𑄑𑄢\u{11134}"), ("ceb", "Leinster"), ("cs", "Leinster"), ("cy", "Laighin"), ("da", "Leinster"), ("de", "Leinster"), ("el", "Λένστερ"), ("en", "Leinster"), ("es", "Leinster"), ("et", "Leinsteri provints"), ("eu", "Leinster"), ("fa", "لینستر"), ("fi", "Leinster"), ("fr", "Leinster"), ("ga", "Cúige Laighean"), ("gl", "Leinster - Cúige Laighean"), ("he", "לנסטר"), ("hr", "Cúige Laighean"), ("hy", "Լենստեր"), ("id", "Leinster"), ("it", "Leinster"), ("ja", "レンスター"), ("ka", "ლენსტერი"), ("ko", "렌스터"), ("lt", "Lensteris"), ("lv", "Lenstera"), ("mk", "Ленстер"), ("nb", "Leinster"), ("nl", "Leinster"), ("no", "Leinster"), ("pl", "Leinster"), ("pt", "Leinster"), ("ru", "Ленстер"), ("sk", "Leinster"), ("sq", "Leinsteri"), ("sr", "Ленстер"), ("sr_Latn", "Lenster"), ("sv", "Leinster"), ("tr", "Leinster"), ("uk", "Ленстер"), ("ur", "لینسٹر"), ("vi", "Leinster"), ("yue", "利揚省"), ("yue_Hans", "利扬省"), ("zh", "倫斯特省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LD",
                    Subdivision{
                        name: "LD",
                        country_alpha2: Alpha2::IE,
                        code: "LD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.7275564), longitude: Some(-7.793108999999999), max_latitude: Some(53.9420806), min_latitude: Some(53.5269053), max_longitude: Some(-7.374903300000001), min_longitude: Some(-8.0358602)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لونجفورد"), ("be", "графства Лонгфард"), ("bg", "Лонгфорд"), ("bn", "ক\u{9be}উন\u{9cd}টি লংফোর\u{9cd}ড"), ("ca", "Comtat de Longford"), ("ccp", "𑄣\u{11127}\u{11101}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "An Longfort (kondado sa Ireland)"), ("cs", "Hrabství Longford"), ("cy", "Swydd Longfoirt"), ("da", "County Longford"), ("de", "County Longford"), ("el", "Κομητεία Λόνγκφορντ"), ("en", "Longford"), ("es", "Condado de Longford"), ("eu", "Longfordeko konderria"), ("fa", "شهرستان لانگفورد"), ("fi", "Longfordin kreivikunta"), ("fr", "Comté de Longford"), ("ga", "Contae an Longfoirt"), ("gl", "Condado de Longford"), ("gu", "કાઉન\u{acd}ટી લો\u{a82}ગફોર\u{acd}ડ"), ("he", "מחוז לונגפורד"), ("hi", "लो\u{902}ग\u{94d}फोर\u{94d}ड काउ\u{902}टी"), ("hu", "Longford megye"), ("id", "County Longford"), ("it", "Longford"), ("ja", "ロングフォード州"), ("ka", "ლონგფორდის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಲಾಂಗ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd}"), ("ko", "롱퍼드 주"), ("lt", "Longfordo grafystė"), ("lv", "Longfordas grāfiste"), ("mk", "Лонгфорд"), ("mr", "काउ\u{902}टी लॉ\u{902}गफोर\u{94d}ड"), ("ms", "County Longford"), ("nb", "Longford"), ("nl", "County Longford"), ("no", "Longford"), ("pl", "Longford (hrabstwo)"), ("pt", "Condado de Longford"), ("ro", "Comitatul Longford"), ("ru", "Лонгфорд"), ("si", "ලෝන\u{dca}ග\u{dca}ෆර\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Longford (grófstvo)"), ("sl", "Okrožje Longford"), ("sr", "Лонгфорд"), ("sr_Latn", "Longford"), ("sv", "Longford"), ("sw", "Wilaya ya Longford"), ("ta", "கவுண\u{bcd}டி லோங\u{bcd}போர\u{bcd}ட"), ("te", "ల\u{c3e}ంగ\u{c4d}\u{200c}ఫ\u{c4b}ర\u{c4d}డ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ลองฟอร\u{e4c}ด"), ("tr", "Longford County"), ("uk", "Лонгфорд"), ("ur", "کاؤنٹی لونگفرڈ"), ("vi", "Hạt Longford"), ("zh", "朗福德郡")]),
                        unofficial_name_list: ["Longphort", "Longphuirt"].to_vec(),
                    }
                ),
                (
                    "LH",
                    Subdivision{
                        name: "LH",
                        country_alpha2: Alpha2::IE,
                        code: "LH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.8969758), longitude: Some(-6.467097099999999), max_latitude: Some(54.1139513), min_latitude: Some(53.6984916), max_longitude: Some(-6.1030107), min_longitude: Some(-6.695343200000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لاوث"), ("az", "Laut"), ("be", "графства Лаўт"), ("bg", "Лаут"), ("bn", "ক\u{9be}উন\u{9cd}টি লৌথ"), ("ca", "Comtat de Louth"), ("ccp", "𑄣\u{1112e}𑄖\u{11134}"), ("ceb", "Lú"), ("cs", "Hrabství Louth"), ("cy", "Swydd Louth"), ("da", "County Louth"), ("de", "County Louth"), ("el", "Κομητεία Λάουθ"), ("en", "Louth"), ("es", "Condado de Louth"), ("et", "Louthi krahvkond"), ("eu", "Loutheko konderria"), ("fa", "شهرستان لوث"), ("fi", "Louth"), ("fr", "comté de Louth"), ("ga", "Contae Lú"), ("gl", "Condado de Louth"), ("gu", "કાઉન\u{acd}ટી લાઉથ"), ("he", "מחוז לאות׳"), ("hi", "काउ\u{902}टी लाउथ"), ("hu", "Louth megye"), ("id", "County Louth"), ("it", "Louth"), ("ja", "ラウス州"), ("ka", "ლაუტის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಲ\u{ccc}ತ\u{ccd}"), ("ko", "라우스 주"), ("lt", "Lauto grafystė"), ("lv", "Lautas grāfiste"), ("mk", "Лаут"), ("mr", "काउ\u{902}टी लोऊथ"), ("ms", "County Louth"), ("nb", "Louth"), ("nl", "County Louth"), ("no", "Louth"), ("pl", "Louth (hrabstwo)"), ("pt", "Condado de Louth"), ("ro", "Comitatul Louth"), ("ru", "Лаут"), ("si", "ලව\u{dd4}ත\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Grófstvo Louth"), ("sr", "Лауд"), ("sr_Latn", "Laud"), ("sv", "Louth"), ("sw", "Wilaya ya Louth"), ("ta", "கவுண\u{bcd}டி லூத\u{bcd}"), ("te", "ల\u{c4c}త\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ล\u{e39}ธ"), ("tr", "County Louth"), ("uk", "Лаут"), ("ur", "کاؤنٹی لاوتھ"), ("vi", "Hạt Louth"), ("zh", "劳斯郡")]),
                        unofficial_name_list: ["Lughbhadh", "Lú"].to_vec(),
                    }
                ),
                (
                    "LK",
                    Subdivision{
                        name: "LK",
                        country_alpha2: Alpha2::IE,
                        code: "LK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6680204), longitude: Some(-8.630497499999999), max_latitude: Some(52.6886401), min_latitude: Some(52.6143899), max_longitude: Some(-8.5703399), min_longitude: Some(-8.68994)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لمريك"), ("be", "графства Лімерык"), ("bg", "Лимерик"), ("bn", "ক\u{9be}উন\u{9cd}টি লিমেরিক"), ("ca", "Comtat de Limerick"), ("ccp", "𑄣\u{11128}𑄟𑄬𑄢\u{11128}𑄇\u{11134}"), ("ceb", "County Limerick"), ("cs", "Hrabství Limerick"), ("cy", "Swydd Limerick"), ("da", "County Limerick"), ("de", "County Limerick"), ("el", "Κομητεία Λίμερικ"), ("en", "Limerick"), ("es", "Condado de Limerick"), ("et", "Limericki krahvkond"), ("eu", "Limerickeko konderria"), ("fa", "شهرستان لیمریک"), ("fi", "Limerickin kreivikunta"), ("fr", "comté de Limerick"), ("ga", "Contae Luimnigh"), ("gl", "Condado de Limerick"), ("gu", "કાઉન\u{acd}ટી લિમરિક"), ("he", "מחוז לימריק"), ("hi", "काउ\u{902}टी लिमरिक"), ("hu", "Limerick megye"), ("hy", "Լիմերիկ"), ("id", "County Limerick"), ("it", "Limerick"), ("ja", "リムリック州"), ("ka", "ლიმერიკის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಲ\u{cbf}ಮರ\u{cbf}ಕ\u{ccd}"), ("ko", "리머릭 주"), ("lt", "Limeriko grafystė"), ("lv", "Limerikas grāfiste"), ("mk", "Лимерик"), ("mr", "काउ\u{902}टी लिमरिक"), ("ms", "County Limerick"), ("nb", "Limerick"), ("nl", "County Limerick"), ("no", "Limerick"), ("pl", "Limerick (hrabstwo)"), ("pt", "Condado de Limerick"), ("ro", "Comitatul Limerick"), ("ru", "Лимерик"), ("si", "ල\u{dd2}ම\u{dd2}රෙක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Limerick"), ("sr", "Лимерик"), ("sr_Latn", "Limerik"), ("sv", "Limerick"), ("sw", "Wilaya ya Limerick"), ("ta", "கவுண\u{bcd}டி லிமரிக\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} ల\u{c3f}మర\u{c3f}క\u{c4d}"), ("th", "เทศมณฑลล\u{e34}เมร\u{e34}ค"), ("tr", "County Limerick"), ("uk", "Лімерик"), ("ur", "کاؤنٹی لیمرک"), ("vi", "Hạt Limerick"), ("zh", "利默里克郡")]),
                        unofficial_name_list: ["Luimneach"].to_vec(),
                    }
                ),
                (
                    "LM",
                    Subdivision{
                        name: "LM",
                        country_alpha2: Alpha2::IE,
                        code: "LM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.9926007), longitude: Some(-8.0655852), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليتريم"), ("be", "графства Літрым"), ("bg", "Лийтрим"), ("bn", "ক\u{9be}উন\u{9cd}টি লিত\u{9cd}রম"), ("ca", "Comtat de Leitrim"), ("ccp", "𑄣𑄬\u{1112d}𑄑\u{11133}𑄢\u{11128}𑄟\u{11134}"), ("ceb", "County Leitrim"), ("cs", "Hrabství Leitrim"), ("cy", "Swydd Leitrim"), ("da", "County Leitrim"), ("de", "Leitrim"), ("el", "Κομητεία Λέιτριμ"), ("en", "Leitrim"), ("es", "Condado de Leitrim"), ("eu", "Leitrimeko konderria"), ("fa", "شهرستان لیتریم"), ("fi", "Leitrimin kreivikunta"), ("fr", "Comté de Leitrim"), ("ga", "Contae Liatroma"), ("gl", "Condado de Leitrim"), ("gu", "કાઉન\u{acd}ટી લ\u{ac7}ટ\u{acd}રીમ"), ("he", "מחוז ליטרים"), ("hi", "काउ\u{902}टी लीट\u{94d}रिम"), ("id", "County Leitrim"), ("is", "County Leitrim"), ("it", "Leitrim"), ("ja", "リートリム州"), ("ka", "ლიტრიმის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಲೀಟ\u{ccd}ರ\u{cbf}ಮ\u{ccd}"), ("ko", "리트림 주"), ("lt", "Leitrimo grafystė"), ("lv", "Litrimas grāfiste"), ("mk", "Литрим"), ("mr", "काउ\u{902}टी लीट\u{94d}रिम"), ("ms", "County Leitrim"), ("nb", "Leitrim"), ("nl", "County Leitrim"), ("no", "Leitrim"), ("pl", "Leitrim (hrabstwo)"), ("pt", "Condado de Leitrim"), ("ro", "Comitatul Leitrim"), ("ru", "Литрим"), ("si", "ලේය\u{dd2}ට\u{dca}\u{200d}ර\u{dd3}ම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Leitrim"), ("sr", "Литрим"), ("sr_Latn", "Litrim"), ("sv", "Leitrim"), ("sw", "Wilaya ya Leitrim"), ("ta", "கவுண\u{bcd}டி லேய\u{bcd}த\u{bcd}ரிம\u{bcd}"), ("te", "ల\u{c40}ట\u{c4d}ర\u{c3f}మ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "คาสก\u{e49}า"), ("tr", "Leitrim County"), ("uk", "Літрім"), ("ur", "کاؤنٹی لیٹریم"), ("vi", "Hạt Leitrim"), ("zh", "利特里姆郡")]),
                        unofficial_name_list: ["Liathdroim"].to_vec(),
                    }
                ),
                (
                    "LS",
                    Subdivision{
                        name: "LS",
                        country_alpha2: Alpha2::IE,
                        code: "LS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.994295), longitude: Some(-7.332300699999999), max_latitude: Some(53.2156522), min_latitude: Some(52.7812693), max_longitude: Some(-6.9321155), min_longitude: Some(-7.7346495)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليش"), ("be", "графтсва Лііш"), ("bg", "Лийш"), ("bn", "ক\u{9be}উন\u{9cd}টি ল\u{9be}ওইস"), ("ca", "Comtat de Laois"), ("ccp", "𑄣\u{1112d}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Laois"), ("cs", "Hrabství Laois"), ("cy", "Swydd Laois"), ("da", "County Laois"), ("de", "County Laois"), ("el", "Κομητεία Λέιοαϊς"), ("en", "Laois"), ("es", "Condado de Laois"), ("eu", "Laoiseko konderria"), ("fa", "شهرستان لیش"), ("fi", "Laois"), ("fr", "comté de Laois"), ("ga", "Contae Laoise"), ("gl", "Condado de Laois"), ("gu", "કાઉન\u{acd}ટી લાઓઇસ"), ("he", "מחוז לייש"), ("hi", "काउ\u{902}टी लाओइस"), ("hu", "Laois megye"), ("id", "County Laois"), ("it", "Laois"), ("ja", "ラオース州"), ("ka", "ლიშის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಲಾವೋಯ\u{cbf}ಸ\u{ccd}"), ("ko", "레이시 주"), ("lt", "Lišo grafystė"), ("lv", "Lišas grāfiste"), ("mk", "Лиш"), ("mr", "काउ\u{902}टी लॉईस"), ("ms", "County Laois"), ("nb", "Laois"), ("nl", "County Laois"), ("no", "Laois"), ("pl", "Laois (hrabstwo)"), ("pt", "Condado de Laois"), ("ro", "Comitatul Laois"), ("ru", "Лиишь"), ("si", "ල\u{dcf}ඕය\u{dd2}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Grófstvo Laois"), ("sr", "Лиш"), ("sr_Latn", "Liš"), ("sv", "Laois"), ("sw", "Wilaya ya Laois"), ("ta", "கவுண\u{bcd}டி லவ\u{bcd}ய\u{bcd}ஸ\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} ల\u{c3e}వ\u{c4b}య\u{c3f}స\u{c4d}"), ("th", "ลาวอ\u{e34}ส ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "County Laois"), ("uk", "Ліїш"), ("ur", "کاؤنٹی لیش"), ("vi", "Hạt Laois"), ("zh", "萊伊什郡")]),
                        unofficial_name_list: ["Laoighis", "Queenʿs"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::IE,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.2218), longitude: Some(8.5567), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مونستر"), ("be", "Манстэр"), ("bg", "Мънстър"), ("ca", "Munster"), ("ccp", "𑄟\u{1112a}𑄚\u{11134}𑄌\u{11133}𑄑𑄢\u{11134}"), ("ceb", "Munster (lalawigan)"), ("cs", "Munster"), ("cy", "Munster"), ("da", "Munster"), ("de", "Munster"), ("el", "Μάνστερ"), ("en", "Munster"), ("es", "Munster"), ("et", "Munsteri provints"), ("eu", "Munster"), ("fa", "مونستر (ایرلند)"), ("fi", "Munster"), ("fr", "Munster"), ("ga", "Cúige Mumhan"), ("gl", "Munster - Cúige Mumhan"), ("he", "מנסטר"), ("hr", "Cúige Mumhan"), ("hy", "Մանսթեր"), ("id", "Munster"), ("it", "Munster"), ("ja", "マンスター"), ("ka", "მანსტერი"), ("kk", "Мунстер"), ("ko", "먼스터"), ("lt", "Mansteris"), ("lv", "Manstera"), ("mk", "Манстер"), ("nb", "Munster"), ("nl", "Munster"), ("no", "Munster"), ("pl", "Munster"), ("pt", "Munster"), ("ru", "Манстер"), ("sk", "Munster"), ("sq", "Munsteri"), ("sr", "Манстер"), ("sr_Latn", "Manster"), ("sv", "Munster"), ("tr", "Munster"), ("uk", "Манстер"), ("ur", "مونسٹر"), ("vi", "Munster"), ("yue", "莫雲省"), ("yue_Hans", "莫云省"), ("zh", "芒斯特省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MH",
                    Subdivision{
                        name: "MH",
                        country_alpha2: Alpha2::IE,
                        code: "MH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.60554800000001), longitude: Some(-6.6564169), max_latitude: Some(53.9176662), min_latitude: Some(53.38186640000001), max_longitude: Some(-6.212610499999999), min_longitude: Some(-7.33552)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ميث"), ("be", "графства Міт"), ("bg", "Мийт"), ("bn", "ক\u{9be}উন\u{9cd}টি মিথ"), ("ca", "Comtat de Meath"), ("ccp", "𑄟𑄬𑄖\u{11134}"), ("ceb", "An Mhí"), ("cs", "Hrabství Meath"), ("cy", "Swydd Meath"), ("da", "County Meath"), ("de", "County Meath"), ("el", "Κομητεία Μιθ"), ("en", "Meath"), ("es", "Condado de Meath"), ("eu", "Meatheko konderria"), ("fa", "شهرستان میث"), ("fi", "Meath"), ("fr", "comté de Meath"), ("ga", "Contae na Mí"), ("gl", "Condado de Meath"), ("gu", "કાઉન\u{acd}ટી મીથ"), ("he", "מחוז מית׳"), ("hi", "काउ\u{902}टी मीथ"), ("id", "County Meath"), ("it", "Meath"), ("ja", "ミース州"), ("ka", "მითის საგრაფო"), ("kk", "Мит (графтық)"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಮೀಥ\u{ccd}"), ("ko", "미스 주"), ("lt", "Mido grafystė"), ("lv", "Mītas grāfiste"), ("mk", "Мит"), ("mr", "काउ\u{902}टी मिथ"), ("ms", "County Meath"), ("nb", "Meath"), ("nl", "County Meath"), ("no", "Meath"), ("pl", "Meath (hrabstwo)"), ("pt", "Condado de Meath"), ("ro", "Comitatul Meath"), ("ru", "Мит"), ("si", "ම\u{dd2}ත\u{dca}\u{200d} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Meath"), ("sr", "Мид"), ("sr_Latn", "Mid"), ("sv", "Meath"), ("sw", "Wilaya ya Meath"), ("ta", "கவுண\u{bcd}டி மெத\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} మ\u{c3f}య\u{c3e}త\u{c4d}"), ("th", "ค\u{e31}นทร\u{e35}\u{e48} ม\u{e35}ทฮ\u{e4c}"), ("tr", "County Meath"), ("uk", "Міт"), ("ur", "کاؤنٹی میدھ"), ("vi", "Hạt Meath"), ("yue", "中土郡"), ("yue_Hans", "中土郡"), ("zh", "米斯郡")]),
                        unofficial_name_list: ["An Mhí", "An Mhídhe"].to_vec(),
                    }
                ),
                (
                    "MN",
                    Subdivision{
                        name: "MN",
                        country_alpha2: Alpha2::IE,
                        code: "MN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.2492046), longitude: Some(-6.9683132), max_latitude: Some(54.4213907), min_latitude: Some(53.9006798), max_longitude: Some(-6.5497282), min_longitude: Some(-7.339505999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة موناغان"), ("bg", "Монахан"), ("bn", "ক\u{9be}উন\u{9cd}টি মোন\u{9be}ঘ\u{9be}ন"), ("ca", "Comtat de Monaghan"), ("ccp", "𑄟\u{11127}𑄚𑄊𑄚\u{11134}"), ("ceb", "County Monaghan"), ("cs", "Hrabství Monaghan"), ("cy", "Swydd Monaghan"), ("da", "County Monaghan"), ("de", "Monaghan"), ("el", "Κομητεία Μόναχαν"), ("en", "Monaghan"), ("es", "Condado de Monaghan"), ("et", "Monaghani krahvkond"), ("eu", "Monaghaneko konderria"), ("fa", "مانهن"), ("fi", "Monaghanin kreivikunta"), ("fr", "Comté de Monaghan"), ("ga", "Contae Mhuineacháin"), ("gl", "Condado de Monaghan"), ("gu", "કાઉન\u{acd}ટી મોનાઘન"), ("he", "מחוז מונאהן"), ("hi", "काउ\u{902}टी मोनाघन"), ("hu", "Monaghan megye"), ("id", "County Monaghan"), ("it", "Monaghan"), ("ja", "モナハン州"), ("ka", "მონაჰანის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಮೊನಾಘನ\u{ccd}"), ("ko", "모나한 주"), ("lt", "Monachano grafystė"), ("lv", "Monahanas grāfiste"), ("mk", "Монахан"), ("mr", "काउ\u{902}टी मोनाघन"), ("ms", "County Monaghan"), ("nb", "Monaghan"), ("nl", "County Monaghan"), ("no", "Monaghan"), ("pl", "Monaghan (hrabstwo)"), ("pt", "Condado de Monaghan"), ("ro", "Comitatul Monaghan"), ("ru", "Монахан"), ("si", "මොනගන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Monaghan"), ("sr", "Монахан"), ("sr_Latn", "Monahan"), ("sv", "Monaghan"), ("sw", "Wilaya ya Monaghan"), ("ta", "கவுண\u{bcd}டி மோனக\u{bbe}ண\u{bcd}"), ("te", "మ\u{c4b}న\u{c3e}గన\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องโมน\u{e31}กฮาน"), ("tr", "Monaghan County"), ("uk", "Монахан"), ("ur", "کاؤنٹی مونیہین"), ("vi", "Hạt Monaghan"), ("zh", "莫纳亨郡")]),
                        unofficial_name_list: ["Muineachán"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::IE,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.93458099999999), longitude: Some(-9.351645600000001), max_latitude: Some(54.3454008), min_latitude: Some(53.4719262), max_longitude: Some(-8.5828617), min_longitude: Some(-10.2510017)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مايو"), ("be", "Мэё"), ("bg", "Мейо"), ("bn", "ক\u{9be}উন\u{9cd}টি ম\u{9be}য\u{9bc}ো"), ("ca", "Comtat de Mayo"), ("ccp", "𑄟𑄬𑄠\u{1112e}"), ("ceb", "Maigh Eo"), ("cs", "Hrabství Mayo"), ("cy", "Swydd Mayo"), ("da", "County Mayo"), ("de", "County Mayo"), ("el", "Κομητεία Μέιο"), ("en", "Mayo"), ("es", "Condado de Mayo"), ("eu", "Mayoko konderria"), ("fa", "شهرستان مایو"), ("fi", "Mayon kreivikunta"), ("fr", "Comté de Mayo"), ("ga", "Contae Mhaigh Eo"), ("gl", "Condado de Mayo"), ("gu", "કાઉન\u{acd}ટી મ\u{ac7}યો"), ("he", "מחוז מאיו"), ("hi", "म\u{947}यो काउ\u{902}टी"), ("hu", "Mayo megye"), ("hy", "Մեյո"), ("id", "County Mayo"), ("is", "County Mayo"), ("it", "Mayo"), ("ja", "メイヨー州"), ("ka", "მეიოს საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಮೇಯೊ"), ("ko", "메이요 주"), ("lt", "Mėjo grafystė"), ("lv", "Mejo grāfiste"), ("mk", "Мејо"), ("mr", "काउ\u{902}टी म\u{947}यो"), ("ms", "County Mayo"), ("nb", "Mayo"), ("nl", "County Mayo"), ("no", "Mayo"), ("pl", "Mayo (hrabstwo)"), ("pt", "Condado de Mayo"), ("ro", "Comitatul Mayo"), ("ru", "Мейо"), ("si", "ම\u{dcf}යෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mayo"), ("sr", "Мејо"), ("sr_Latn", "Mejo"), ("sv", "Mayo"), ("sw", "Wilaya ya Mayo"), ("ta", "கவுண\u{bcd}டி ம\u{bbe}யோ"), ("te", "మ\u{c3e}య\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลเมโย"), ("tr", "Mayo COunty"), ("uk", "Мейо"), ("ur", "کاؤنٹی میو"), ("vi", "Hạt Mayo"), ("zh", "梅奧郡")]),
                        unofficial_name_list: ["Maigh Eo"].to_vec(),
                    }
                ),
                (
                    "OY",
                    Subdivision{
                        name: "OY",
                        country_alpha2: Alpha2::IE,
                        code: "OY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.2356871), longitude: Some(-7.7122229), max_latitude: Some(53.424279), min_latitude: Some(52.8481718), max_longitude: Some(-6.9777077), min_longitude: Some(-8.0838722)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوفالي"), ("be", "графства Офалі"), ("bg", "Офали"), ("bn", "ক\u{9be}উন\u{9cd}টি ওফ\u{9cd}য\u{9be}লি"), ("ca", "Comtat d’Offaly"), ("ccp", "𑄃\u{11127}𑄜𑄣\u{11128}"), ("ceb", "Uíbh Fhailí"), ("cs", "Hrabství Offaly"), ("cy", "Swydd Offaly"), ("da", "County Offaly"), ("de", "County Offaly"), ("el", "Κομητεία Όφαλι"), ("en", "Offaly"), ("es", "Condado de Offaly"), ("eu", "Offalyko konderria"), ("fa", "افلی"), ("fi", "Offalyn kreivikunta"), ("fr", "Comté d’Offaly"), ("ga", "Contae Uíbh Fhailí"), ("gl", "Condado de Offaly"), ("gu", "કાઉન\u{acd}ટી ઓફ\u{ac7}લી"), ("he", "מחוז אופאלי"), ("hi", "काउ\u{902}टी ऑफ\u{947}ली"), ("id", "County Offaly"), ("it", "Offaly"), ("ja", "オファリー州"), ("ka", "ოფალის საგრაფო"), ("kk", "Оффали (графтық)"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಆಫಲ\u{cbf}"), ("ko", "오펄리 주"), ("lt", "Ofalio grafystė"), ("lv", "Ofali grāfiste"), ("mk", "Офали"), ("mr", "काउ\u{902}टी ऑफली"), ("ms", "County Offaly"), ("nb", "Offaly"), ("nl", "County Offaly"), ("no", "Offaly"), ("pl", "Offaly"), ("pt", "Condado de Offaly"), ("ro", "Comitatul Offaly"), ("ru", "Оффали"), ("si", "ඔෆලේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Offaly"), ("sl", "okrožje Offaly, Irska"), ("sr", "Офали"), ("sr_Latn", "Ofali"), ("sv", "Offaly"), ("sw", "Wilaya ya Offaly"), ("ta", "கவுண\u{bcd}டி ஆஃப\u{bcd}லய\u{bcd}"), ("te", "ఓఫ\u{c3e}ల\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลออฟฟาล\u{e35}"), ("tr", "Offaly County"), ("uk", "Оффалі"), ("ur", "کاؤنٹی اوفلی"), ("vi", "Hạt Offaly"), ("zh", "奧法利郡")]),
                        unofficial_name_list: ["Kingʿs", "Kingʿs County", "Ua Uíbh Fhailí", "Uí Fáilghe"].to_vec(),
                    }
                ),
                (
                    "RN",
                    Subdivision{
                        name: "RN",
                        country_alpha2: Alpha2::IE,
                        code: "RN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.6275906), longitude: Some(-8.189095499999999), max_latitude: Some(53.63941999999999), min_latitude: Some(53.6117699), max_longitude: Some(-8.16418), min_longitude: Some(-8.20713)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة روسكومون"), ("be", "графства Раскоман"), ("bg", "Роскомън"), ("bn", "ক\u{9be}উন\u{9cd}টি রসকমন"), ("ca", "Comtat de Roscommon"), ("ccp", "𑄢\u{1112e}𑄌\u{11134}𑄇\u{1112e}𑄟\u{11127}𑄚\u{11134}"), ("ceb", "Roscommon"), ("cs", "Hrabství Roscommon"), ("cy", "Swydd Roscommon"), ("da", "County Roscommon"), ("de", "County Roscommon"), ("el", "Κομητεία Ροσκόμον"), ("en", "Roscommon"), ("es", "Condado de Roscommon"), ("eu", "Roscommoneko konderria"), ("fa", "شهرستان رسکومون"), ("fi", "Roscommonin kreivikunta"), ("fr", "Comté de Roscommon"), ("ga", "Contae Ros Comáin"), ("gl", "Condado de Roscommon"), ("gu", "કાઉન\u{acd}ટી રોસકોમન"), ("he", "מחוז רוסקומון"), ("hi", "रौसकॉमन काउ\u{902}टी"), ("hy", "Ռոսքոմոն"), ("id", "County Roscommon"), ("is", "County Roscommon"), ("it", "Roscommon"), ("ja", "ロスコモン州"), ("ka", "როსკომონის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ರಾಸ\u{ccd}ಕಾಮನ\u{ccd}"), ("ko", "로스커먼 주"), ("lt", "Roskomono grafystė"), ("lv", "Roskomono grāfiste"), ("mk", "Роскомон"), ("mr", "काउ\u{902}टी रॉसकॉमन"), ("ms", "County Roscommon"), ("nb", "Roscommon"), ("nl", "County Roscommon"), ("no", "Roscommon"), ("pl", "Roscommon (hrabstwo)"), ("pt", "Condado de Roscommon"), ("ro", "Comitatul Roscommon"), ("ru", "Роскоммон"), ("si", "රොස\u{dca}කොමන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Contae Ros Comáin"), ("sr", "Роскомон"), ("sr_Latn", "Roskomon"), ("sv", "Roscommon"), ("sw", "Wilaya ya Roscommon"), ("ta", "கவுண\u{bcd}டி ரோஸ\u{bcd}கோம\u{bcd}மோன\u{bcd}"), ("te", "ర\u{c3e}స\u{c4b}క\u{c3e}మన\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "รอสส\u{e4c}คอมม\u{e31}น"), ("tr", "County Roscommon"), ("uk", "Роскоммон"), ("ur", "کاؤنٹی راسکومن"), ("vi", "Hạt Roscommon"), ("zh", "羅斯康芒郡")]),
                        unofficial_name_list: ["Ros Comáin"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::IE,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.27661029999999), longitude: Some(-8.4760888), max_latitude: Some(54.2872399), min_latitude: Some(54.24951), max_longitude: Some(-8.44213), min_longitude: Some(-8.5193301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سليجو"), ("be", "графства Слайга"), ("bg", "Слайгоу"), ("bn", "ক\u{9be}উন\u{9cd}টি স\u{9cd}লিগো"), ("ca", "Comtat de Sligo"), ("ccp", "𑄥\u{11133}𑄣\u{11128}𑄉\u{1112e}"), ("ceb", "Sligo (kondado)"), ("cs", "Hrabství Sligo"), ("cy", "Swydd Sligo"), ("da", "County Sligo"), ("de", "County Sligo"), ("el", "Κομητεία Σλάιγκο"), ("en", "Sligo"), ("es", "Condado de Sligo"), ("eu", "Sligoko konderria"), ("fa", "شهرستان اسلایگو"), ("fi", "Sligon kreivikunta"), ("fr", "Comté de Sligo"), ("ga", "Contae Shligigh"), ("gl", "Condado de Sligo"), ("gu", "કાઉન\u{acd}ટી સ\u{acd}લાઇગો"), ("he", "מחוז סלייגו"), ("hi", "काउ\u{902}टी स\u{94d}लीगो"), ("hu", "Sligo megye"), ("hy", "Սլայգո"), ("id", "County Sligo"), ("is", "County Sligo"), ("it", "Sligo"), ("ja", "スライゴ州"), ("ka", "სლაიგოს საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಸ\u{ccd}ಲ\u{cbf}ಗೊ"), ("ko", "슬라이고 주"), ("lt", "Slaigo grafystė"), ("lv", "Slaigo grāfiste"), ("mk", "Слајго"), ("mr", "काउ\u{902}टी स\u{94d}लिगो"), ("ms", "County Sligo"), ("nb", "Sligo"), ("nl", "County Sligo"), ("no", "Sligo"), ("pl", "Sligo (hrabstwo)"), ("pt", "Condado de Sligo"), ("ro", "Comitatul Sligo"), ("ru", "Слайго"), ("si", "ස\u{dca}ල\u{dd2}ගෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sligo"), ("sl", "grofija Sligo, Irska"), ("sr", "Слајго"), ("sr_Latn", "Slajgo"), ("sv", "Sligo"), ("sw", "Wilaya ya Sligo"), ("ta", "கவுண\u{bcd}டி ஸ\u{bcd}லிகொ"), ("te", "క\u{c4c}ంట\u{c40} స\u{c4d}ల\u{c3f}గ\u{c4b}"), ("th", "เทศมณฑลสล\u{e34}โก"), ("tr", "County Sligo"), ("uk", "Слайго"), ("ur", "کاؤنٹی سلایگوہ"), ("vi", "Hạt Sligo"), ("zh", "斯萊戈郡")]),
                        unofficial_name_list: ["Sligeach"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::IE,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.47378940000001), longitude: Some(-8.1618514), max_latitude: Some(53.1675822), min_latitude: Some(52.2020144), max_longitude: Some(-7.372055800000001), min_longitude: Some(-8.4800793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيبيراري"), ("be", "графства Тыперэры"), ("bg", "Типърари"), ("bn", "ক\u{9be}উন\u{9cd}টি টিপের\u{9be}রি"), ("ca", "Comtat de Tipperary"), ("ccp", "𑄑\u{11128}𑄛𑄢\u{11134}𑄢\u{11128}"), ("cs", "Hrabství Tipperary"), ("cy", "Swydd Tipperary"), ("da", "County Tipperary"), ("de", "County Tipperary"), ("el", "Κομητεία Τιπερέρι"), ("en", "Tipperary"), ("es", "Condado de Tipperary"), ("et", "Tipperary krahvkond"), ("eu", "Tipperaryko konderria"), ("fa", "شهرستان تیپراری"), ("fi", "Tipperaryn kreivikunta"), ("fr", "comté de Tipperary"), ("ga", "Contae Thiobraid Árann"), ("gl", "Condado de Tipperary"), ("gu", "કાઉન\u{acd}ટી ટિપર\u{ac7}રી"), ("he", "מחוז טיפררי"), ("hi", "टिपर\u{947}री काउ\u{902}टी"), ("id", "County Tipperary"), ("it", "Tipperary"), ("ja", "ティペラリー州"), ("ka", "ტიპერარის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಟ\u{cbf}ಪ\u{cc6}ರರ\u{cbf}"), ("ko", "티퍼레리 주"), ("lt", "Tipererio grafystė"), ("lv", "Tiperēri grāfiste"), ("mk", "Типерери"), ("mr", "काउ\u{902}टी टिपर\u{947}री"), ("ms", "County Tipperary"), ("nb", "Tipperary"), ("nl", "County Tipperary"), ("no", "Tipperary"), ("pl", "Tipperary (hrabstwo)"), ("pt", "Condado de Tipperary"), ("ro", "Comitatul Tipperary"), ("ru", "Типперэри"), ("si", "ට\u{dd2}පෙරේර\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tipperary"), ("sr", "Типерари"), ("sr_Latn", "Tiperari"), ("sv", "Tipperary"), ("sw", "Wilaya ya Tipperary"), ("ta", "கவுண\u{bcd}டி டிப\u{bcd}பெயரி"), ("te", "ట\u{c3f}ప\u{c4d}పరర\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "ท\u{e34}ปเปอร\u{e4c}แรร\u{e35}"), ("tr", "County Tipperary"), ("uk", "Тіпперері"), ("ur", "کاؤنٹی ٹپاریری"), ("vi", "Hạt Tipperary"), ("zh", "蒂珀雷里郡")]),
                        unofficial_name_list: ["Tiobraid Árann"].to_vec(),
                    }
                ),
                (
                    "U",
                    Subdivision{
                        name: "U",
                        country_alpha2: Alpha2::IE,
                        code: "U",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.7617), longitude: Some(6.9612), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ulster"), ("ar", "أولستر"), ("az", "Olster"), ("be", "Ольстэр"), ("bg", "Ълстър"), ("bs", "Ulster"), ("ca", "Ulster"), ("ccp", "𑄃𑄣\u{11134}𑄌\u{11133}𑄑𑄢\u{11134}"), ("ceb", "Ulster"), ("cs", "Ulster"), ("cy", "Ulster"), ("da", "Ulster"), ("de", "Provinz Ulster"), ("el", "Άλστερ"), ("en", "Ulster"), ("es", "Úlster"), ("et", "Ulster"), ("eu", "Ulster"), ("fa", "اولستر"), ("fi", "Ulster"), ("fr", "Ulster"), ("ga", "Cúige Uladh"), ("gl", "Úlster - Cúige Uladh"), ("he", "אלסטר"), ("hr", "Cúige Uladh"), ("hy", "Օլսթեր"), ("id", "Ulster"), ("is", "Ulster"), ("it", "Ulster"), ("ja", "アルスター"), ("ka", "ოლსტერი"), ("ko", "얼스터"), ("lt", "Alsteris"), ("lv", "Olstera"), ("mk", "Алстер"), ("nb", "Ulster"), ("nl", "Ulster"), ("no", "Ulster"), ("pl", "Ulster"), ("pt", "Ulster"), ("ro", "Ulster"), ("ru", "Ольстер"), ("sk", "Ulster"), ("sr", "Алстер"), ("sr_Latn", "Alster"), ("sv", "Ulster"), ("th", "อ\u{e31}ลสเตอร\u{e4c}"), ("tr", "Ulster"), ("uk", "Ольстер"), ("ur", "السٹر"), ("vi", "Ulster"), ("yue", "烏勒省"), ("yue_Hans", "乌勒省"), ("zh", "阿爾斯特省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WD",
                    Subdivision{
                        name: "WD",
                        country_alpha2: Alpha2::IE,
                        code: "WD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.256667), longitude: Some(-7.129167), max_latitude: Some(52.27019989999999), min_latitude: Some(52.22481), max_longitude: Some(-7.0548699), min_longitude: Some(-7.171189999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة وترفورد"), ("be", "графства Уотэрфард"), ("bg", "Уотърфорд"), ("bn", "ক\u{9be}উন\u{9cd}টি ওয\u{9bc}\u{9be}ট\u{9be}রফোর\u{9cd}ড"), ("ca", "Comtat de Waterford"), ("ccp", "𑄃\u{1112e}𑄠𑄑𑄢\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Waterford (kondado)"), ("cs", "Hrabství Waterford"), ("cy", "Swydd Waterford"), ("da", "County Waterford"), ("de", "County Waterford"), ("el", "Κομητεία Ουότερφορντ"), ("en", "Waterford"), ("es", "Waterford"), ("et", "Waterfordi krahvkond"), ("eu", "Waterfordeko konderria"), ("fa", "شهرستان واترفورد"), ("fi", "Waterfordin kreivikunta"), ("fr", "comté de Waterford"), ("ga", "Contae Phort Láirge"), ("gl", "Condado de Waterford"), ("gu", "કાઉન\u{acd}ટી વૉટરફોર\u{acd}ડ"), ("he", "מחוז ווטרפורד"), ("hi", "काउ\u{902}टी वॉटरफोर\u{94d}ड"), ("hu", "Waterford megye"), ("id", "County Waterford"), ("it", "Waterford"), ("ja", "ウォーターフォード州"), ("ka", "უოტერფორდის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ವಾಟರ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd}"), ("ko", "워터퍼드 주"), ("lt", "Voterfordo grafystė"), ("lv", "Voterfordas grāfiste"), ("mk", "Вотерфорд"), ("mr", "काउ\u{902}टी वॉटरफोर\u{94d}ड"), ("ms", "County Waterford"), ("nb", "Waterford"), ("nl", "County Waterford"), ("no", "Waterford"), ("pl", "Waterford (hrabstwo)"), ("pt", "Condado de Waterford"), ("ro", "Comitatul Waterford"), ("ru", "Уотерфорд"), ("si", "වොටර\u{dca}ෆොර\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Waterford"), ("sr", "Вотерфорд"), ("sr_Latn", "Voterford"), ("sv", "Waterford"), ("sw", "Wilaya ya Waterford"), ("ta", "கவுண\u{bcd}டி வ\u{bbe}ட\u{bcd}டரிபோர\u{bcd}ட"), ("te", "వ\u{c3e}టర\u{c4d}ఫ\u{c4b}ర\u{c4d}డ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "วอเตอร\u{e4c}ฟอร\u{e4c}ด"), ("tr", "Waterford County"), ("uk", "Вотерфорд"), ("ur", "کاؤنٹی واٹرفرڈ"), ("vi", "Hạt Waterford"), ("zh", "沃特福德郡")]),
                        unofficial_name_list: ["Port Láirge"].to_vec(),
                    }
                ),
                (
                    "WH",
                    Subdivision{
                        name: "WH",
                        country_alpha2: Alpha2::IE,
                        code: "WH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5345308), longitude: Some(-7.4653217), max_latitude: Some(53.7879599), min_latitude: Some(53.31796259999999), max_longitude: Some(-6.9547842), min_longitude: Some(-7.9729531)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة وستميث"), ("be", "графства Уэстміт"), ("bg", "Уестмийт"), ("bn", "ক\u{9be}উন\u{9cd}টি ওয\u{9bc}েস\u{9cd}টমিদ"), ("ca", "Comtat de Westmeath"), ("ccp", "𑄃\u{11127}𑄠𑄬𑄌\u{11134}𑄟𑄬𑄖\u{11134}"), ("ceb", "An Iarmhí"), ("cs", "Hrabství Westmeath"), ("cy", "Swydd Westmeath"), ("da", "County Westmeath"), ("de", "County Westmeath"), ("el", "Κομητεία Ουέστμιθ"), ("en", "Westmeath"), ("es", "Condado de Westmeath"), ("eu", "Westmeatheko konderria"), ("fa", "وست میث"), ("fi", "Westmeath"), ("fr", "Comté de Westmeath"), ("ga", "Contae na hIarmhí"), ("gl", "Condado de Westmeath"), ("gu", "કાઉન\u{acd}ટી વ\u{ac7}સ\u{acd}ટમ\u{ac7}થ"), ("he", "מחוז וסטמית׳"), ("hi", "काउ\u{902}टी व\u{947}स\u{94d}टम\u{947}थ"), ("id", "County Westmeath"), ("it", "Westmeath"), ("ja", "ウェストミース州"), ("ka", "უესტმითის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}ಮೀಥ\u{ccd}"), ("ko", "웨스트미스 주"), ("lt", "Vestmido grafystė"), ("lv", "Vestmītas grāfiste"), ("mk", "Вестмит"), ("mr", "काउ\u{902}टी व\u{947}स\u{94d}टम\u{947}थ"), ("ms", "County Westmeath"), ("nb", "Westmeath"), ("nl", "County Westmeath"), ("no", "Westmeath"), ("pl", "Westmeath"), ("pt", "Condado de Westmeath"), ("ro", "Comitatul Westmeath"), ("ru", "Уэстмит"), ("si", "වෙස\u{dca}ට\u{dca}ම\u{dd2}ත\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Westmeath"), ("sr", "Вестмид"), ("sr_Latn", "Vestmid"), ("sv", "Westmeath"), ("sw", "Wilaya ya Westmeath"), ("ta", "கவுண\u{bcd}டி வெஸ\u{bcd}டமேத\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d}\u{200c}మ\u{c40}త\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ร\u{e31}ฐโอไฮโอ"), ("tr", "County Westmeath"), ("uk", "Західний Міт"), ("ur", "کاؤنٹی ویسٹمیدھ"), ("vi", "Hạt Westmeath"), ("zh", "韋斯特米斯郡")]),
                        unofficial_name_list: ["An Iarmhidhe"].to_vec(),
                    }
                ),
                (
                    "WW",
                    Subdivision{
                        name: "WW",
                        country_alpha2: Alpha2::IE,
                        code: "WW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.9808207), longitude: Some(-6.044588999999999), max_latitude: Some(52.99892), min_latitude: Some(52.9661001), max_longitude: Some(-6.0154599), min_longitude: Some(-6.07004)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ويكلاو"), ("be", "графства Уіклау"), ("bg", "Уиклоу"), ("bn", "ক\u{9be}উন\u{9cd}টি উইকলো"), ("ca", "Comtat de Wicklow"), ("ccp", "𑄅\u{1112a}𑄃\u{11128}𑄇\u{11134}𑄣\u{1112e}"), ("ceb", "Wicklow (kondado)"), ("cs", "Hrabství Wicklow"), ("cy", "Swydd Wicklow"), ("da", "County Wicklow"), ("de", "County Wicklow"), ("el", "Κομητεία Ουΐκλοου"), ("en", "Wicklow"), ("es", "Condado de Wicklow"), ("eu", "Wicklowko konderria"), ("fa", "شهرستان ویکلو"), ("fi", "Wicklow’n kreivikunta"), ("fr", "Comté de Wicklow"), ("ga", "Contae Chill Mhantáin"), ("gl", "Condado de Wicklow"), ("gu", "કાઉન\u{acd}ટી વિકલો"), ("he", "מחוז ויקלו"), ("hi", "काउ\u{902}टी विकलो"), ("hu", "Wicklow megye"), ("hy", "Ուիկլոու"), ("id", "County Wicklow"), ("it", "Wicklow"), ("ja", "ウィックロー州"), ("ka", "უიკლოუს საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ವ\u{cbf}ಕ\u{ccd}ಲೊ"), ("ko", "위클로 주"), ("lt", "Viklou grafystė"), ("lv", "Viklovas grāfiste"), ("mk", "Виклоу"), ("mr", "काउ\u{902}टी विस\u{94d}कॉ"), ("ms", "County Wicklow"), ("nb", "Wicklow"), ("nl", "County Wicklow"), ("no", "Wicklow"), ("pl", "Wicklow (hrabstwo)"), ("pt", "Condado de Wicklow"), ("ro", "Comitatul Wicklow"), ("ru", "Уиклоу"), ("si", "ව\u{dd2}ක\u{dca}ලෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Wicklow"), ("sl", "Okrožje Wicklow"), ("sr", "Виклоу"), ("sr_Latn", "Viklou"), ("sv", "Wicklow"), ("sw", "Wilaya ya Wicklow"), ("ta", "கவுண\u{bcd}டி விக\u{bcd}ள\u{bbe}"), ("te", "వ\u{c3f}క\u{c4d}ల\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "ค\u{e31}นทร\u{e35}\u{e48}ว\u{e34}คโลว"), ("tr", "County Wicklow"), ("uk", "Віклоу"), ("ur", "کاؤنٹی ویکلو"), ("vi", "Hạt Wicklow"), ("zh", "威克洛郡")]),
                        unofficial_name_list: ["Cill Maintain", "Cill Mhanntáin"].to_vec(),
                    }
                ),
                (
                    "WX",
                    Subdivision{
                        name: "WX",
                        country_alpha2: Alpha2::IE,
                        code: "WX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.336916), longitude: Some(-6.4633381), max_latitude: Some(52.3470099), min_latitude: Some(52.32044), max_longitude: Some(-6.4464301), min_longitude: Some(-6.49995)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة وكسفورد"), ("be", "графства Уэксфард"), ("bg", "Уексфорд"), ("bn", "ক\u{9be}উন\u{9cd}টি ওয\u{9bc}েক\u{9cd}সফোর\u{9cd}ড"), ("ca", "Comtat de Wexford"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄇\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Loch Garman (kondado)"), ("cs", "Hrabství Wexford"), ("cy", "Swydd Wexford"), ("da", "County Wexford"), ("de", "County Wexford"), ("el", "Κομητεία Ουέξφορντ"), ("en", "Wexford"), ("es", "Condado de Wexford"), ("eu", "Wexfordeko konderria"), ("fa", "شهرستان وکسفورد"), ("fi", "Wexfordin kreivikunta"), ("fr", "Comté de Wexford"), ("ga", "Contae Loch Garman"), ("gl", "Condado de Wexford"), ("gu", "કાઉન\u{acd}ટી વ\u{ac7}ક\u{acd}સફોર\u{acd}ડ"), ("he", "מחוז ווקספורד"), ("hi", "व\u{947}क\u{94d}सफोर\u{94d}ड काउ\u{902}टी"), ("hu", "Wexford megye"), ("id", "County Wexford"), ("it", "Wexford"), ("ja", "ウェックスフォード州"), ("ka", "უექსფორდის საგრაფო"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ವ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}ಫರ\u{ccd}ಡ\u{ccd}"), ("ko", "웩스퍼드 주"), ("lt", "Veksfordo grafystė"), ("lv", "Veksfordas grāfiste"), ("mk", "Вексфорд"), ("mr", "काउ\u{902}टी व\u{947}क\u{94d}सफोर\u{94d}ड"), ("ms", "County Wexford"), ("nb", "Wexford"), ("nl", "County Wexford"), ("no", "Wexford"), ("pl", "Wexford (hrabstwo)"), ("pt", "Condado de Wexford"), ("ro", "Comitatul Wexford"), ("ru", "Уэксфорд"), ("si", "වෙක\u{dca}ස\u{dca}ෆොර\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Wexford"), ("sr", "Вексфорд"), ("sr_Latn", "Veksford"), ("sv", "Wexford"), ("sw", "Wilaya ya Wexford"), ("ta", "கவுண\u{bcd}டி வெஸ\u{bcd}போர\u{bcd}ட"), ("te", "వ\u{c46}క\u{c4d}స\u{c4d} ఫ\u{c4b}ర\u{c4d}డ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}อกแวกฟอร\u{e4c}ด"), ("tr", "Wexford Countt"), ("uk", "Вексфорд"), ("ur", "کاؤنٹی ویکسفرڈ"), ("vi", "Wexford"), ("zh", "韦克斯福德郡")]),
                        unofficial_name_list: ["Loch Garman"].to_vec(),
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
#[cfg(feature = "ie")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::IE,
        alpha3: Alpha3::IRL,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 353,
        currency_code: "EUR",
        gec: Some(GEC::EI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("IRL"),
        iso_long_name: "Ireland",
        iso_short_name: "Ireland",
        official_language_list: ["en", "ga"].to_vec(),
        spoken_language_list: ["en", "ga"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Irish"),
        number: "372",
        postal_code: true,
        postal_code_format: Some("[\\dA-Z]{3} ?[\\dA-Z]{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "IE",
        unofficial_name_list: ["Ireland", "Irland", "Irlande", "Irlanda", "アイルランド", "Ierland"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Ireland"), ("af", "Ierland"), ("ak", "Ireland"), ("am", "ጐፘሴሒን፥"), ("an", "Ireland"), ("ar", "أيرلندا"), ("as", "আয়\u{9be}ৰ\u{9cd}লেণ\u{9cd}ড"), ("ay", "Ireland"), ("az", "İrlandiya"), ("ba", "Ireland"), ("be", "Ірландыя"), ("bg", "Ирландия"), ("bi", "Ireland"), ("bn", "আয়\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bn_IN", "আয়\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("br", "Iwerzhon"), ("bs", "Irska"), ("ca", "Irlanda"), ("ce", "Ирланди"), ("ch", "Ireland"), ("cs", "Irsko"), ("cv", "Ирланди"), ("cy", "Iwerddon"), ("da", "Irland"), ("de", "Irland"), ("dv", "އ\u{7a6}ޔ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}ގ\u{7ac} ޖ\u{7aa}މ\u{7b0}ހ\u{7ab}ރ\u{7a8}އ\u{7b0}ޔ\u{7a7}"), ("dz", "ཨའ\u{f72}་ར\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"), ("ee", "Ireland"), ("el", "Ιρλανδία"), ("en", "Ireland"), ("eo", "Irlando"), ("es", "Irlanda"), ("et", "Iirimaa"), ("eu", "Irlanda"), ("fa", "ایرلند"), ("ff", "Irlannda"), ("fi", "Irlanti"), ("fo", "Írland"), ("fr", "Irlande"), ("fy", "Ierlân"), ("ga", "Éire"), ("gl", "Irlanda"), ("gn", "Ireland"), ("gu", "આયરલ\u{ac7}ન\u{acd}ડ"), ("gv", "Pobblaght Nerin"), ("ha", "Ireland"), ("he", "אירלנד"), ("hi", "आयरल\u{948}ण\u{94d}ड"), ("hr", "Irska"), ("ht", "Ilann"), ("hu", "Írország"), ("hy", "Իռլանդիա"), ("ia", "Irlanda"), ("id", "Irlandia"), ("io", "Republiko di Irlando"), ("is", "Írland"), ("it", "Irlanda"), ("iu", "Ireland"), ("ja", "アイルランド"), ("ka", "ირლანდია"), ("ki", "Ireland"), ("kk", "Ирландия"), ("kl", "Ireland"), ("km", "អៀរឡង\u{17cb}"), ("kn", "ಐರ\u{ccd}ಲ\u{ccd}ಯಂಡ\u{ccd}"), ("ko", "아일랜드"), ("ku", "Îrlanda"), ("kv", "Ирландия"), ("kw", "Repoblek Wordhen"), ("ky", "Ирландия"), ("lo", "ປະເທດອຽກລ\u{eb1}ງ"), ("lt", "Airija"), ("lv", "Īrija"), ("mi", "Airangi"), ("mk", "Ирска"), ("ml", "അയര\u{d4d}\u{200d}ലണ\u{d4d}ട\u{d4d}"), ("mn", "Ирланд"), ("mr", "आयर\u{94d}ल\u{902}ड"), ("ms", "Ireland"), ("mt", "Irlanda"), ("my", "အ\u{102d}\u{102f}င\u{103a}ယာလန\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Ripubrikit Airerand"), ("nb", "Irland"), ("ne", "आयरल\u{94d}याण\u{94d}ड"), ("nl", "Ierland"), ("nn", "Irland"), ("nv", "Bitsiighaʼ Łichííʼí Bikéyah"), ("oc", "Irlanda"), ("or", "ଆୟରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"), ("pa", "ਆਈਰਲ\u{a48}\u{a02}ਡ"), ("pi", "Ireland"), ("pl", "Irlandia"), ("ps", "د آيرلېنډ جمهوريت"), ("pt", "Irlanda"), ("pt_BR", "Irlanda"), ("ro", "Irlanda"), ("ru", "Ирландия"), ("rw", "Irilande"), ("sc", "Irlanda"), ("sd", "Ireland"), ("si", "අයර\u{dca}ලන\u{dca}තය"), ("sk", "Írsko"), ("sl", "Irska"), ("so", "Ayrlaanda"), ("sq", "Irlandë"), ("sr", "Ирска"), ("sv", "Irland"), ("sw", "Ireland"), ("ta", "அயர\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "ఐర\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("tg", "Ирландия"), ("th", "ไอร\u{e4c}แลนด\u{e4c}"), ("ti", "አየርላንድ"), ("tk", "Irlandiýa"), ("tl", "Ireland"), ("tr", "İrlanda"), ("tt", "İреланд, İрландиа"), ("ug", "ئىرېلاندىيە"), ("uk", "Ірландія"), ("ur", "جمہوریہ آئرستان"), ("uz", "Irlandiya"), ("ve", "Ireland"), ("vi", "Ái Nhĩ Lan"), ("wa", "Irlande"), ("wo", "Irlaand"), ("xh", "Ireland"), ("yo", "Írẹ\u{301}lándì"), ("zh_CN", "爱尔兰"), ("zh_HK", "愛爾蘭"), ("zh_TW", "愛爾蘭"), ("zu", "I-Ayilendi")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}