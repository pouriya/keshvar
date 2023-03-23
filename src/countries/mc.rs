// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Principality of Monaco

#[cfg(all(feature = "mc", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MC;
    pub const ALPHA3: Alpha3 = Alpha3::MCO;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 377;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::MN);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("MON");
    pub const ISO_SHORT_NAME: &str = "Monaco";
    pub const ISO_LONG_NAME: &str = "The Principality of Monaco";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Monegasque");
    pub const NUMBER: &str = "492";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("980\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "MC";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Monaco", "Mónaco", "モナコ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Monaco"),
        ("af", "Monaco"),
        ("ak", "Monaco"),
        ("am", "ሞናኮ"),
        ("an", "Monaco"),
        ("ar", "موناكو"),
        ("as", "মোন\u{9be}কো"),
        ("ay", "Monaco"),
        ("az", "Monako"),
        ("ba", "Monaco"),
        ("be", "Манака"),
        ("bg", "Монако"),
        ("bi", "Monaco"),
        ("bn", "মোন\u{9be}কো"),
        ("bn_IN", "মোন\u{9be}কো"),
        ("br", "Monako"),
        ("bs", "Monako"),
        ("ca", "Mònaco"),
        ("ce", "Монако"),
        ("ch", "Monaco"),
        ("cs", "Monako"),
        ("cv", "Монако"),
        ("cy", "Monaco"),
        ("da", "Monaco"),
        ("de", "Monaco"),
        ("dv", "މ\u{7ae}ނ\u{7a7}ކ\u{7af}"),
        ("dz", "མ\u{f7c}་ན་ཀ\u{f7c}"),
        ("ee", "Monaco"),
        ("el", "Μονακό"),
        ("en", "Monaco"),
        ("eo", "Monako"),
        ("es", "Mónaco"),
        ("et", "Monaco"),
        ("eu", "Monako"),
        ("fa", "موناکو"),
        ("ff", "Monako"),
        ("fi", "Monaco"),
        ("fo", "Monako"),
        ("fr", "Monaco"),
        ("fy", "Monako"),
        ("ga", "Monacó"),
        ("gl", "Mónaco"),
        ("gn", "Monaco"),
        ("gu", "મોન\u{ac7}કો"),
        ("gv", "Monaco"),
        ("ha", "Monaco"),
        ("he", "מונקו"),
        ("hi", "मोन\u{948}को"),
        ("hr", "Monako"),
        ("ht", "Monako"),
        ("hu", "Monaco"),
        ("hy", "Մոնակո"),
        ("ia", "Monaco"),
        ("id", "Monako"),
        ("io", "Monako"),
        ("is", "Mónakó"),
        ("it", "Monaco"),
        ("iu", "Monaco"),
        ("ja", "モナコ"),
        ("ka", "მონაკო"),
        ("ki", "Monaco"),
        ("kk", "Монако"),
        ("kl", "Monaco"),
        ("km", "ម\u{17c9}\u{17bc}ណាក\u{17bc}"),
        ("kn", "ಮೊನಾಕೋ"),
        ("ko", "모나코"),
        ("ku", "Monako"),
        ("kv", "Монако"),
        ("kw", "Monako"),
        ("ky", "Монако"),
        ("lo", "ປະເທດໂມນາໂກ"),
        ("lt", "Monakas"),
        ("lv", "Monako"),
        ("mi", "Manako"),
        ("mk", "Монако"),
        ("ml", "മൊണ\u{d3e}ക\u{d4d}കോ"),
        ("mn", "Монако"),
        ("mr", "मोन\u{945}को"),
        ("ms", "Monaco"),
        ("mt", "Monako"),
        (
            "my",
            "မ\u{102d}\u{102f}နာက\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Monako"),
        ("nb", "Monaco"),
        ("ne", "मोनाको"),
        ("nl", "Monaco"),
        ("nn", "Monaco"),
        ("nv", "Monaco"),
        ("oc", "Mónegue"),
        ("or", "ମୋନ\u{b3e}କୋ"),
        ("pa", "ਮ\u{a4b}ਨਕ\u{a4b}"),
        ("pi", "मोनाको"),
        ("pl", "Monako"),
        ("ps", "Monaco"),
        ("pt", "Mónaco"),
        ("pt_BR", "Mônaco"),
        ("ro", "Monaco"),
        ("ru", "Монако"),
        ("rw", "Monako"),
        ("sc", "Mònaco"),
        ("sd", "Monaco"),
        ("si", "මොන\u{dcf}කෝ"),
        ("sk", "Monako"),
        ("sl", "Monako"),
        ("so", "Moonako"),
        ("sq", "Monako"),
        ("sr", "Монако"),
        ("sv", "Monaco"),
        ("sw", "Monaco"),
        ("ta", "மொன\u{bbe}க\u{bcd}கோ"),
        ("te", "మ\u{c4b}న\u{c3e}క\u{c4b}"),
        ("tg", "Монако"),
        ("th", "โมนาโก"),
        ("ti", "Monaco"),
        ("tk", "Monako"),
        ("tl", "Monaco"),
        ("tr", "Monako"),
        ("tt", "Манако"),
        ("ug", "موناكو"),
        ("uk", "Монако"),
        ("ur", "موناکو"),
        ("uz", "Monako"),
        ("ve", "Monaco"),
        ("vi", "Mo-na-cô"),
        ("wa", "Monaco"),
        ("wo", "Monaako"),
        ("xh", "Monaco"),
        ("yo", "Mónakò"),
        ("zh_CN", "摩纳哥"),
        ("zh_HK", "摩納哥"),
        ("zh_TW", "摩納哥"),
        ("zu", "Monaco"),
    ];
    #[cfg(all(feature = "mc", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 43.73841760000001;
        pub const LONGITUDE: f64 = 7.424615799999999;
        pub const MAX_LATITUDE: f64 = 43.7519029;
        pub const MAX_LONGITUDE: f64 = 7.4426;
        pub const MIN_LATITUDE: f64 = 43.7237999;
        pub const MIN_LONGITUDE: f64 = 7.4091049;
        pub const NORTHEAST_LATITUDE: f64 = 43.7519029;
        pub const NORTHEAST_LONGITUDE: f64 = 7.4426;
        pub const SOUTHWEST_LATITUDE: f64 = 43.7237999;
        pub const SOUTHWEST_LONGITUDE: f64 = 7.4091049;
    }
}
#[cfg(all(feature = "mc", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 43.73841760000001,
            longitude: 7.424615799999999,
            max_latitude: 43.7519029,
            max_longitude: 7.4426,
            min_latitude: 43.7237999,
            min_longitude: 7.4091049,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 43.7519029,
                    longitude: 7.4426,
                },
                southwest: CountryGeoBound {
                    latitude: 43.7237999,
                    longitude: 7.4091049,
                },
            },
        }
    }
}

#[cfg(all(feature = "mc", feature = "subdivisions"))]
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
                    "CL",
                    Subdivision{
                        name: "CL",
                        country_alpha2: Alpha2::MC,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "La Còla"), ("ccp", "𑄣 𑄇\u{1112e}𑄣\u{11128}"), ("da", "La Colle"), ("de", "La Colle"), ("en", "La Colle"), ("es", "La Colle"), ("fr", "La Colle"), ("gl", "La Colle, Mónaco"), ("it", "La Colle"), ("ko", "라콜"), ("nl", "La Colle"), ("pt", "La Colle"), ("ru", "Ла-Колле"), ("sv", "La Colle"), ("tr", "La Colle, Monako"), ("ur", "لا کولی، موناکو"), ("zh", "拉科尔")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::MC,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "La-Kondamin"), ("be", "Ла-Кандамін"), ("ca", "La Condamina"), ("ccp", "𑄣 𑄇\u{11127}𑄚\u{11134}𑄓𑄟\u{1112d}𑄚\u{11134}"), ("ceb", "La Condamine"), ("cs", "La Condamine"), ("de", "La Condamine"), ("en", "La Condamine"), ("es", "La Condamine"), ("fa", "لا کوندامین"), ("fr", "La Condamine"), ("gl", "La Condamine"), ("hu", "La Condamine"), ("hy", "Լա Կոնդամին"), ("it", "La Condamine"), ("ja", "ラ・コンダミーヌ地区"), ("ko", "라콩다민"), ("lt", "La Kondaminas"), ("nl", "La Condamine"), ("pl", "La Condamine"), ("pt", "La Condamine"), ("ru", "Ла-Кондамин"), ("sv", "La Condamine"), ("tr", "La Condamine"), ("uk", "Ла-Кондамін"), ("ur", "لا کونڈامینی"), ("zh", "拉康达明")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "FO",
                    Subdivision{
                        name: "FO",
                        country_alpha2: Alpha2::MC,
                        code: "FO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Fòntvielha"), ("ccp", "𑄜\u{11127}𑄚\u{11134}𑄑\u{11134}𑄞\u{11128}𑄣𑄬"), ("ceb", "Fontvieille"), ("de", "Fontvieille"), ("el", "Φοντβιέγ"), ("en", "Fontvieille"), ("es", "Fontvieille"), ("fa", "فونوی، موناکو"), ("fr", "Fontvieille"), ("gl", "Fontvieille, Mónaco"), ("he", "פונטוייל"), ("hu", "Fontvieille"), ("hy", "Ֆոնվյեյ"), ("it", "Fontvieille"), ("ja", "フォンヴィエイユ"), ("ko", "퐁비에유"), ("lt", "Fonvjėjus"), ("nb", "Fontvieille"), ("nl", "Fontvieille"), ("no", "Fontvieille"), ("pl", "Fontvieille"), ("pt", "Fontvieille"), ("ru", "Фонвьей"), ("sv", "Fontvieille (del av en befolkad plats)"), ("tr", "Fontvieille, Monako"), ("uk", "Фонтвілль"), ("zh", "芳特维耶")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::MC,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄣 𑄉𑄢𑄬"), ("en", "La Gare")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JE",
                    Subdivision{
                        name: "JE",
                        country_alpha2: Alpha2::MC,
                        code: "JE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎𑄢\u{11134}𑄓\u{11128}𑄚\u{11134} 𑄃𑄬𑄇\u{11134}𑄎\u{1112e}𑄑\u{11128}𑄇\u{11134} 𑄓𑄬 𑄟\u{11127}𑄚\u{11134}𑄇\u{1112e}"), ("de", "Exotischer Garten von Monaco"), ("en", "Jardin Exotique de Monaco"), ("es", "Jardín Exótico de Mónaco"), ("fr", "jardin exotique de Monaco"), ("it", "Giardino esotico di Monaco"), ("ja", "熱帯公園"), ("nl", "Jardin Exotique de Monaco"), ("pl", "Jardin exotique de Monaco"), ("uk", "Екзотичний сад Монако"), ("zh", "摩納哥熱帶公園")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::MC,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Larvotto"), ("ca", "Larvot e Molins Bas"), ("ccp", "𑄣𑄢\u{11134}𑄞\u{1112e}𑄑\u{1112e}"), ("de", "Larvotto"), ("en", "Larvotto"), ("es", "Larvotto"), ("fr", "Larvotto/Bas Moulins"), ("gl", "Larvotto"), ("it", "Larvotto"), ("ja", "ラルヴォット"), ("ko", "라르보토"), ("nl", "Larvotto"), ("pt", "Larvotto"), ("ru", "Ларвотто"), ("tr", "Larvotto"), ("ur", "لارووٹو"), ("zh", "拉沃托")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::MC,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄣\u{11134}𑄝\u{1112f}𑄇\u{1112e}𑄠𑄬𑄖\u{11134}"), ("en", "Malbousquet")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::MC,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Monte Carlo"), ("ar", "مونت كارلو"), ("az", "Monte Karlo"), ("be", "Монтэ-Карла"), ("bg", "Монте Карло"), ("bs", "Monte Carlo"), ("ca", "Montcarles"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑𑄬 𑄇𑄢\u{11134}𑄣\u{1112e}"), ("ceb", "Monte-Carlo"), ("cs", "Monte Carlo"), ("cy", "Monte-Carlo"), ("da", "Monte Carlo"), ("de", "Monte-Carlo"), ("el", "Μόντε Κάρλο"), ("en", "Monte Carlo"), ("es", "Montecarlo"), ("et", "Monte Carlo"), ("eu", "Monte-Carlo"), ("fa", "مونت\u{200c}کارلو"), ("fi", "Monte Carlo"), ("fr", "Monte-Carlo"), ("ga", "Monte Carlo"), ("gl", "Monte Carlo"), ("he", "מונטה קרלו"), ("hi", "मो\u{902}टी कार\u{94d}लो"), ("hr", "Monte Carlo"), ("hu", "Monte-Carlo"), ("hy", "Մոնտե Կառլո"), ("id", "Monte Carlo"), ("is", "Monte Carlo"), ("it", "Monte Carlo"), ("ja", "モンテカルロ"), ("ka", "მონტე-კარლო"), ("kk", "Монте-Карло"), ("ko", "몬테카를로"), ("lt", "Monte Karlas"), ("lv", "Montekarlo"), ("mk", "Монте Карло"), ("ms", "Monte Carlo"), ("my", "မ\u{103d}န\u{103a}တ\u{102e}ကာလ\u{102d}\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Monte Carlo"), ("nl", "Monte Carlo"), ("no", "Monte Carlo"), ("pl", "Monte Carlo"), ("pt", "Monte Carlo"), ("ro", "Monte Carlo"), ("ru", "Монте-Карло"), ("sk", "Monte Carlo"), ("sl", "Monte Carlo"), ("sq", "Monte Karlo"), ("sr", "Монте Карло"), ("sr_Latn", "Monte Karlo"), ("sv", "Monte Carlo"), ("ta", "ம\u{bbe}ன\u{bcd}டே க\u{bbe}ர\u{bcd}லோ"), ("tr", "Monte Carlo"), ("uk", "Монте-Карло"), ("ur", "مونٹی کارلو"), ("uz", "Montekarlo"), ("vi", "Monte Carlo"), ("yue", "蒙地卡羅"), ("yue_Hans", "蒙地卡罗"), ("zh", "蒙特卡洛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MG",
                    Subdivision{
                        name: "MG",
                        country_alpha2: Alpha2::MC,
                        code: "MG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Moneqetti"), ("bg", "Монегхети"), ("ca", "Moneguet"), ("ccp", "𑄟\u{11127}𑄚𑄬𑄊𑄬𑄑\u{11128}"), ("ceb", "Moneghetti"), ("de", "Moneghetti"), ("en", "Moneghetti"), ("es", "Monegeti"), ("fr", "Moneghetti"), ("gl", "Moneghetti"), ("hu", "Moneghetti"), ("it", "Moneghetti"), ("ko", "모네게티"), ("nl", "Moneghetti"), ("pt", "Moneghetti"), ("ru", "Монегетти"), ("sv", "Moneghetti"), ("zh", "莫内盖蒂")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::MC,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "موناكو فيل"), ("az", "Monako Vill"), ("be", "Горад Манака"), ("ca", "Vila de Mònaco"), ("ccp", "𑄟\u{11127}𑄚𑄇\u{1112e}-𑄞\u{11128}𑄣𑄬"), ("ceb", "Commune de Monaco"), ("cs", "Monaco-Ville"), ("da", "Monaco-Ville"), ("de", "Monaco-Ville"), ("en", "Monaco-Ville"), ("es", "Mónaco"), ("et", "Monaco-Ville"), ("eu", "Monako"), ("fa", "موناکوویل"), ("fr", "Monaco-Ville"), ("gl", "Monaco-Ville"), ("he", "מונקו-ויל"), ("hu", "Monaco"), ("hy", "Մոնակո"), ("it", "Monaco Vecchia"), ("ja", "モナコ・ヴィル"), ("kk", "Монако"), ("ko", "모나코빌"), ("lt", "Monakas"), ("nl", "Monaco-Ville"), ("pl", "Monaco-Ville"), ("pt", "Monaco-Ville"), ("ru", "Монако"), ("sv", "Monaco-Ville"), ("te", "మ\u{c4a}న\u{c3e}క\u{c4b}-వ\u{c3f}ల\u{c4d}"), ("tr", "Monaco-Ville"), ("uk", "Монако"), ("ur", "موناکو شہر"), ("uz", "Monako"), ("zh", "摩纳哥城")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::MC,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟\u{1112f}𑄣\u{11128}𑄚\u{11134}"), ("en", "Moulins")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "PH",
                    Subdivision{
                        name: "PH",
                        country_alpha2: Alpha2::MC,
                        code: "PH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134} 𑄦𑄢\u{11134}𑄇\u{11128}𑄅\u{1112a}𑄣𑄌\u{11134}"), ("ceb", "Port Hercule"), ("de", "Port Hercule"), ("en", "Port Hercules"), ("es", "Puerto Hércules"), ("fr", "Port Hercule"), ("it", "Port Hercule"), ("ja", "ポートエルキュール"), ("nl", "Port Hercules"), ("sv", "Port Hercule")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::MC,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134}-𑄓\u{11128}𑄞\u{1112e}𑄖\u{11134} 𑄇𑄛𑄬𑄣\u{11134}"), ("en", "Sainte-Dévote Chapel"), ("es", "iglesia de Santa Devota"), ("fr", "église Sainte-Dévote"), ("nl", "Église Sainte-Dévote")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::MC,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄣 𑄥\u{1112e}𑄢\u{11134}𑄌\u{11134}"), ("en", "La Source")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SP",
                    Subdivision{
                        name: "SP",
                        country_alpha2: Alpha2::MC,
                        code: "SP",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄌\u{11133}𑄛𑄬𑄣\u{11128}𑄅\u{1112a}𑄉𑄌\u{11134}"), ("en", "Spélugues")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::MC,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Sen-Roman"), ("ca", "La Rossa e Sant Roman"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄢\u{11127}𑄟𑄚\u{11134}"), ("ceb", "Saint-Roman"), ("de", "La Rousse"), ("en", "Saint Roman"), ("es", "La Rousse/San Roman"), ("fr", "La Rousse"), ("gl", "Saint Roman"), ("it", "La Rousse/Saint-Roman"), ("ko", "생로망"), ("nl", "Saint Roman, Monaco"), ("pt", "La Rousse/Saint-Roman"), ("ru", "Сен-Роман"), ("sv", "Saint-Roman, Monaco"), ("ur", "سینٹ رومن، موناکو"), ("zh", "圣罗曼")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "VR",
                    Subdivision{
                        name: "VR",
                        country_alpha2: Alpha2::MC,
                        code: "VR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Quarter,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄞𑄣\u{11127}𑄚\u{11134} 𑄓𑄬 𑄣 𑄢\u{1112f}𑄌\u{11134}"), ("en", "Vallon de la Rousse"), ("nl", "La Rousse")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "mc")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MC,
        alpha3: Alpha3::MCO,
        address_format: None,
        continent: Continent::Europe,
        country_code: 377,
        currency_code: "EUR",
        gec: Some(GEC::MN),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("MON"),
        iso_long_name: "The Principality of Monaco",
        iso_short_name: "Monaco",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Monegasque"),
        number: "492",
        postal_code: true,
        postal_code_format: Some("980\\d{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "MC",
        unofficial_name_list: ["Monaco", "Mónaco", "モナコ"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Monaco"),
            ("af", "Monaco"),
            ("ak", "Monaco"),
            ("am", "ሞናኮ"),
            ("an", "Monaco"),
            ("ar", "موناكو"),
            ("as", "মোন\u{9be}কো"),
            ("ay", "Monaco"),
            ("az", "Monako"),
            ("ba", "Monaco"),
            ("be", "Манака"),
            ("bg", "Монако"),
            ("bi", "Monaco"),
            ("bn", "মোন\u{9be}কো"),
            ("bn_IN", "মোন\u{9be}কো"),
            ("br", "Monako"),
            ("bs", "Monako"),
            ("ca", "Mònaco"),
            ("ce", "Монако"),
            ("ch", "Monaco"),
            ("cs", "Monako"),
            ("cv", "Монако"),
            ("cy", "Monaco"),
            ("da", "Monaco"),
            ("de", "Monaco"),
            ("dv", "މ\u{7ae}ނ\u{7a7}ކ\u{7af}"),
            ("dz", "མ\u{f7c}་ན་ཀ\u{f7c}"),
            ("ee", "Monaco"),
            ("el", "Μονακό"),
            ("en", "Monaco"),
            ("eo", "Monako"),
            ("es", "Mónaco"),
            ("et", "Monaco"),
            ("eu", "Monako"),
            ("fa", "موناکو"),
            ("ff", "Monako"),
            ("fi", "Monaco"),
            ("fo", "Monako"),
            ("fr", "Monaco"),
            ("fy", "Monako"),
            ("ga", "Monacó"),
            ("gl", "Mónaco"),
            ("gn", "Monaco"),
            ("gu", "મોન\u{ac7}કો"),
            ("gv", "Monaco"),
            ("ha", "Monaco"),
            ("he", "מונקו"),
            ("hi", "मोन\u{948}को"),
            ("hr", "Monako"),
            ("ht", "Monako"),
            ("hu", "Monaco"),
            ("hy", "Մոնակո"),
            ("ia", "Monaco"),
            ("id", "Monako"),
            ("io", "Monako"),
            ("is", "Mónakó"),
            ("it", "Monaco"),
            ("iu", "Monaco"),
            ("ja", "モナコ"),
            ("ka", "მონაკო"),
            ("ki", "Monaco"),
            ("kk", "Монако"),
            ("kl", "Monaco"),
            ("km", "ម\u{17c9}\u{17bc}ណាក\u{17bc}"),
            ("kn", "ಮೊನಾಕೋ"),
            ("ko", "모나코"),
            ("ku", "Monako"),
            ("kv", "Монако"),
            ("kw", "Monako"),
            ("ky", "Монако"),
            ("lo", "ປະເທດໂມນາໂກ"),
            ("lt", "Monakas"),
            ("lv", "Monako"),
            ("mi", "Manako"),
            ("mk", "Монако"),
            ("ml", "മൊണ\u{d3e}ക\u{d4d}കോ"),
            ("mn", "Монако"),
            ("mr", "मोन\u{945}को"),
            ("ms", "Monaco"),
            ("mt", "Monako"),
            (
                "my",
                "မ\u{102d}\u{102f}နာက\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Monako"),
            ("nb", "Monaco"),
            ("ne", "मोनाको"),
            ("nl", "Monaco"),
            ("nn", "Monaco"),
            ("nv", "Monaco"),
            ("oc", "Mónegue"),
            ("or", "ମୋନ\u{b3e}କୋ"),
            ("pa", "ਮ\u{a4b}ਨਕ\u{a4b}"),
            ("pi", "मोनाको"),
            ("pl", "Monako"),
            ("ps", "Monaco"),
            ("pt", "Mónaco"),
            ("pt_BR", "Mônaco"),
            ("ro", "Monaco"),
            ("ru", "Монако"),
            ("rw", "Monako"),
            ("sc", "Mònaco"),
            ("sd", "Monaco"),
            ("si", "මොන\u{dcf}කෝ"),
            ("sk", "Monako"),
            ("sl", "Monako"),
            ("so", "Moonako"),
            ("sq", "Monako"),
            ("sr", "Монако"),
            ("sv", "Monaco"),
            ("sw", "Monaco"),
            ("ta", "மொன\u{bbe}க\u{bcd}கோ"),
            ("te", "మ\u{c4b}న\u{c3e}క\u{c4b}"),
            ("tg", "Монако"),
            ("th", "โมนาโก"),
            ("ti", "Monaco"),
            ("tk", "Monako"),
            ("tl", "Monaco"),
            ("tr", "Monako"),
            ("tt", "Манако"),
            ("ug", "موناكو"),
            ("uk", "Монако"),
            ("ur", "موناکو"),
            ("uz", "Monako"),
            ("ve", "Monaco"),
            ("vi", "Mo-na-cô"),
            ("wa", "Monaco"),
            ("wo", "Monaako"),
            ("xh", "Monaco"),
            ("yo", "Mónakò"),
            ("zh_CN", "摩纳哥"),
            ("zh_HK", "摩納哥"),
            ("zh_TW", "摩納哥"),
            ("zu", "Monaco"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}