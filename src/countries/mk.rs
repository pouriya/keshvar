// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of North Macedonia

#[cfg(all(feature = "mk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::MK;
    pub const ALPHA3: Alpha3 = Alpha3::MKD;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 389;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::MKD;
    pub const GEC: Option<GEC> = Some(GEC::MK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::MKD);
    pub const ISO_SHORT_NAME: &str = "North Macedonia";
    pub const ISO_LONG_NAME: &str = "The Republic of North Macedonia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["mk"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["mk"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Macedonian");
    pub const NUMBER: &str = "807";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "MK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Macedonia",
        "Mazedonien",
        "Macédoine",
        "F.Y.R.O.M (Macedonia)",
        "マケドニア旧ユーゴスラビア共和国",
        "Macedonië [FYROM]",
        "Macedonia (The Former Yugoslav Republic of)",
        "North Macedonia",
        "Macedonia (FYROM)",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "North Macedonia"),
        ("af", "North Macedonia"),
        ("ak", "North Macedonia"),
        ("am", "ሰሜን መቄዶኒያ"),
        ("an", "North Macedonia"),
        ("ar", "مقدونيا الشمالية"),
        ("as", "North Macedonia"),
        ("ay", "North Macedonia"),
        ("az", "North Macedonia"),
        ("ba", "North Macedonia"),
        ("be", "Паўночная Македонія"),
        ("bg", "North Macedonia"),
        ("bi", "North Macedonia"),
        ("bn", "উত\u{9cd}তর ম\u{9cd}য\u{9be}সেডোনিয়\u{9be}"),
        ("bn_IN", "উত\u{9cd}তর ম\u{9cd}য\u{9be}সেডোনিয়\u{9be}"),
        ("br", "North Macedonia"),
        ("bs", "North Macedonia"),
        ("ca", "North Macedonia"),
        ("ce", "North Macedonia"),
        ("ch", "North Macedonia"),
        ("cs", "Severní Makedonie"),
        ("cv", "North Macedonia"),
        ("cy", "Gogledd Macedonia"),
        ("da", "Nordmakedonien"),
        ("de", "Nordmazedonien"),
        ("dv", "North Macedonia"),
        ("dz", "North Macedonia"),
        ("ee", "North Macedonia"),
        ("el", "Βόρεια Μακεδονία"),
        ("en", "North Macedonia"),
        ("eo", "Nord-Makedonio"),
        ("es", "Macedonia del Norte"),
        ("et", "Põhja-Makedoonia"),
        ("eu", "Ipar Mazedonia"),
        ("fa", "مقدونیه شمالی"),
        ("ff", "North Macedonia"),
        ("fi", "North Macedonia"),
        ("fo", "North Macedonia"),
        ("fr", "Macédoine du Nord"),
        ("fy", "North Macedonia"),
        ("ga", "North Macedonia"),
        ("gl", "North Macedonia"),
        ("gn", "North Macedonia"),
        ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"),
        ("gv", "North Macedonia"),
        ("ha", "North Macedonia"),
        ("he", "צפון קלדוניה"),
        ("hi", "उत\u{94d}तर म\u{948}स\u{947}डोनिया"),
        ("hr", "Sjeverna Makedonija"),
        ("ht", "North Macedonia"),
        ("hu", "Észak-Macedónia"),
        ("hy", "North Macedonia"),
        ("ia", "Macedonia del Nord"),
        ("id", "Makedonia Utara"),
        ("io", "North Macedonia"),
        ("is", "Norður-Makedónía"),
        ("it", "Macedonia del Nord"),
        ("iu", "North Macedonia"),
        ("ja", "North Macedonia"),
        ("ka", "North Macedonia"),
        ("ki", "North Macedonia"),
        ("kk", "North Macedonia"),
        ("kl", "North Macedonia"),
        ("km", "North Macedonia"),
        ("kn", "North Macedonia"),
        ("ko", "북마케도니아"),
        ("ku", "Kaledonyaya Nû"),
        ("kv", "North Macedonia"),
        ("kw", "North Macedonia"),
        ("ky", "North Macedonia"),
        ("lo", "North Macedonia"),
        ("lt", "North Macedonia"),
        ("lv", "North Macedonia"),
        ("mi", "North Macedonia"),
        ("mk", "Северна Македонија"),
        ("ml", "North Macedonia"),
        ("mn", "North Macedonia"),
        ("mr", "उत\u{94d}तर म\u{945}स\u{947}डोनिया"),
        ("ms", "North Macedonia"),
        ("mt", "North Macedonia"),
        ("my", "North Macedonia"),
        ("na", "North Macedonia"),
        ("nb", "Nord-Makedonia"),
        ("ne", "North Macedonia"),
        ("nl", "Noord-Macedonië"),
        ("nn", "North Macedonia"),
        ("nv", "North Macedonia"),
        ("oc", "Macedònia del Nòrd"),
        (
            "or",
            "ନର\u{b4d}ଥ ମ\u{b4d}ୟ\u{b3e}ସ\u{b3f}ଡୋନ\u{b3f}ୟ\u{b3e}",
        ),
        ("pa", "ਨਾਰਥ ਕਾਲੀਡ\u{a4b}ਨੀਆ"),
        ("pi", "North Macedonia"),
        ("pl", "Macedonia Północna"),
        ("ps", "North Macedonia"),
        ("pt", "Macedónia do Norte"),
        ("pt_BR", "Macedônia do Norte"),
        ("ro", "North Macedonia"),
        ("ru", "Северная Македония"),
        ("rw", "North Macedonia"),
        ("sc", "Matzedònia de su Nord"),
        ("sd", "North Macedonia"),
        ("si", "North Macedonia"),
        ("sk", "North Macedonia"),
        ("sl", "North Macedonia"),
        ("so", "North Macedonia"),
        ("sq", "Maqedonia e Veriut"),
        ("sr", "Северна Македонија"),
        ("sv", "Nordmakedonien"),
        ("sw", "North Macedonia"),
        ("ta", "North Macedonia"),
        ("te", "North Macedonia"),
        ("tg", "Македонияи Шимолӣ"),
        ("th", "North Macedonia"),
        ("ti", "North Macedonia"),
        ("tk", "North Macedonia"),
        ("tl", "North Macedonia"),
        ("tr", "Kuzey Makedonya"),
        ("tt", "North Macedonia"),
        ("ug", "شىمالىي ماكېدونىيە"),
        ("uk", "Північна Македонія"),
        ("ur", "North Macedonia"),
        ("uz", "North Macedonia"),
        ("ve", "North Macedonia"),
        ("vi", "North Macedonia"),
        ("wa", "North Macedonia"),
        ("wo", "North Macedonia"),
        ("xh", "North Macedonia"),
        ("yo", "North Macedonia"),
        ("zh_CN", "北马其顿"),
        ("zh_HK", "北馬其頓"),
        ("zh_TW", "北馬其頓"),
        ("zu", "North Macedonia"),
    ];
    #[cfg(all(feature = "mk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 41.608635;
        pub const LONGITUDE: f64 = 21.745275;
        pub const MAX_LATITUDE: f64 = 42.373646;
        pub const MAX_LONGITUDE: f64 = 23.0340441;
        pub const MIN_LATITUDE: f64 = 40.8537826;
        pub const MIN_LONGITUDE: f64 = 20.452423;
        pub const NORTHEAST_LATITUDE: f64 = 42.373646;
        pub const NORTHEAST_LONGITUDE: f64 = 23.0340441;
        pub const SOUTHWEST_LATITUDE: f64 = 40.8537826;
        pub const SOUTHWEST_LONGITUDE: f64 = 20.452423;
    }
}
#[cfg(all(feature = "mk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 41.608635,
            longitude: 21.745275,
            max_latitude: 42.373646,
            max_longitude: 23.0340441,
            min_latitude: 40.8537826,
            min_longitude: 20.452423,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 42.373646,
                    longitude: 23.0340441,
                },
                southwest: CountryGeoBound {
                    latitude: 40.8537826,
                    longitude: 20.452423,
                },
            },
        }
    }
}

#[cfg(all(feature = "mk", feature = "subdivisions"))]
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
                    "101",
                    Subdivision{
                        name: "Veles",
                        country_alpha2: Alpha2::MK,
                        code: "101",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.7138462), longitude: Some(21.7704092), max_latitude: Some(41.737505), min_latitude: Some(41.691838), max_longitude: Some(21.7968535), min_longitude: Some(21.7434913)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Велес"), ("ccp", "𑄞𑄣𑄬𑄌\u{11134}"), ("ceb", "Titov Veles Opština"), ("de", "Opština Veles"), ("en", "Veles"), ("es", "Municipalidad de Veles"), ("fa", "ولس مونیکیپلیتی"), ("fr", "Vélès"), ("hr", "Općina Veles"), ("hy", "Վելեսի համայնք"), ("it", "Veles"), ("ka", "ველესის თემი"), ("ko", "벨레스 시"), ("mk", "Општина Велес"), ("nl", "Veles"), ("ro", "Veles"), ("ru", "Велес"), ("sq", "Komuna e Velesit"), ("sr", "Општина Велес"), ("sr_Latn", "Opština Veles"), ("sv", "Titov Veles Opština"), ("uk", "Велес"), ("ur", "ویلیس بلدیہ"), ("zh", "韋萊斯區")]),
                        unofficial_name_list: ["Veles"].to_vec(),
                    }
                ),
                (
                    "102",
                    Subdivision{
                        name: "Gradsko",
                        country_alpha2: Alpha2::MK,
                        code: "102",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5750625), longitude: Some(21.9494531), max_latitude: Some(41.5846933), min_latitude: Some(41.5631299), max_longitude: Some(21.9661031), min_longitude: Some(21.9327078)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غرادسكو"), ("bg", "Градско"), ("bn", "গ\u{9cd}র\u{9be}ডস\u{9cd}কো পৌরসভ\u{9be}"), ("ccp", "𑄉\u{11133}𑄢𑄖\u{11134}𑄥\u{11134}𑄇\u{1112e}"), ("ceb", "Gradsko"), ("da", "Gradsko Municipality"), ("de", "Opština Gradsko"), ("el", "Δήμος Γκράντσκου"), ("en", "Gradsko"), ("es", "Municipalidad de Gradsko"), ("fi", "Gradskon kunta"), ("fr", "Gradsko"), ("gu", "ગ\u{acd}ર\u{ac7}ડ\u{acd}સ\u{acd}કો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ग\u{94d}र\u{947}ड\u{94d}सको नगरपालिका"), ("hr", "Općina Gradsko"), ("hy", "Գրադսկոյի համայնք"), ("id", "Kotamadya Gradsko"), ("it", "Gradsko"), ("ja", "グラドスコ"), ("ka", "გრადსკოს თემი"), ("kn", "ಗ\u{ccd}ರಾಡ\u{ccd}ಸ\u{ccd}ಕೊ ಪುರಸಭ\u{cc6}"), ("ko", "그라드스코 시"), ("lt", "Gradsko savivaldybė"), ("lv", "Gradsko pašvaldība"), ("mk", "Општина Градско"), ("mr", "ग\u{94d}र\u{945}डस\u{94d}को म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gradsko Municipality"), ("nb", "Gradsko kommune"), ("nl", "Gradsko"), ("no", "Gradsko kommune"), ("pl", "Gmina Gradsko"), ("pt", "Município de Glarus"), ("ru", "Градско"), ("si", "ග\u{dca}\u{200d}රඩ\u{dca}ස\u{dca}කෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Grackës"), ("sr", "Општина Градско"), ("sr_Latn", "Opština Gradsko"), ("sv", "Gradsko"), ("ta", "க\u{bcd}ர\u{bbe}ட\u{bcd}ஸ\u{bcd}கொ நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4d}ర\u{c3e}డ\u{c4d}స\u{c4d}క\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "กราดสโค"), ("tr", "Gadsko Belediyesi"), ("uk", "Градсько"), ("ur", "گرادسکو بلدیہ"), ("vi", "Đô thị tự trị Gradsko"), ("zh", "格拉茲科區")]),
                        unofficial_name_list: ["Gradsko"].to_vec(),
                    }
                ),
                (
                    "103",
                    Subdivision{
                        name: "Demir Kapija",
                        country_alpha2: Alpha2::MK,
                        code: "103",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.411389), longitude: Some(22.242222), max_latitude: Some(41.4142979), min_latitude: Some(41.39890399999999), max_longitude: Some(22.2562623), min_longitude: Some(22.2329164)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Демир Капия"), ("ccp", "𑄓𑄬𑄟\u{11128}𑄢\u{11134} 𑄇𑄛\u{11128}𑄎"), ("ceb", "Demir Kapija"), ("de", "Opština Demir Kapija"), ("en", "Demir Kapija"), ("es", "Municipalidad de Demir Kapija"), ("fr", "Demir Kapija"), ("hr", "Općina Demir Kapija"), ("hy", "Դեմիր Կապիայի համայնք"), ("ja", "デミル・カピヤ"), ("ka", "დემირ-კაპიის თემი"), ("ko", "데미르카피야 시"), ("mk", "Општина Демир Капија"), ("pl", "Gmina Demir Kapija"), ("pt", "Município de Demir Kapija"), ("ru", "Демир-Капия"), ("sq", "Komuna e Demir Kapisë"), ("sr", "Општина Демир Капија"), ("sr_Latn", "Opština Demir Kapija"), ("sv", "Demir Kapija"), ("tr", "Demir Kapı Belediyesi"), ("uk", "Демир-Капія"), ("ur", "دیمیر کاپیا بلدیہ"), ("zh", "德米尔卡皮雅区")]),
                        unofficial_name_list: ["Demir Kapija"].to_vec(),
                    }
                ),
                (
                    "104",
                    Subdivision{
                        name: "Kavadarci",
                        country_alpha2: Alpha2::MK,
                        code: "104",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4329364), longitude: Some(22.0088861), max_latitude: Some(41.4443026), min_latitude: Some(41.4218808), max_longitude: Some(22.026397), min_longitude: Some(21.9955194)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كافادارتسي"), ("bg", "Кавадарци"), ("bn", "ক\u{9be}র\u{9cd}ভ\u{9be}দ\u{9be}রচি পৌরসভ\u{9be}"), ("ca", "Kavadarci"), ("ccp", "𑄇𑄞𑄓𑄢\u{11134}𑄥\u{11128}"), ("ceb", "Kavadarci"), ("da", "Kavadarci Municipality"), ("de", "Opština Kavadarci"), ("el", "Δήμος Καφανταρτσίου"), ("en", "Kavadarci"), ("es", "Municipalidad de Kavadarci"), ("fi", "Kavadarcin kunta"), ("fr", "Kavadarci"), ("gu", "કવાડર\u{acd}સી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कवड\u{93c}ारसी नगर पालिका"), ("hr", "Općina Kavadarci"), ("id", "Kotamadya Kavadarci"), ("it", "Kavadarci"), ("ja", "カヴァダルツィ"), ("ka", "კავადარცის თემი"), ("kn", "ಕವಡಾರ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "카바다르치 시"), ("lt", "Kavadarčio savivaldybė"), ("lv", "Kavadarci pašvaldība"), ("mk", "Општина Кавадарци"), ("mr", "कवडारची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kavadarci Municipality"), ("nb", "Kavadarci kommune"), ("nl", "Kavadarci"), ("no", "Kavadarci kommune"), ("pl", "Gmina Kavadarci"), ("pt", "Município de Kavadarci"), ("ru", "Кавадарци"), ("si", "කවඩර\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kavadarit"), ("sr", "Општина Кавадарци"), ("sr_Latn", "Opština Kavadarci"), ("sv", "Opsjtina Kavadarci"), ("ta", "கவடற\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3e}వ\u{c4b}డ\u{c3e}ర\u{c4d}స\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคาวาดาร\u{e4c}ช\u{e35}"), ("tr", "Kavadarci Belediyesi"), ("uk", "Кавадарці"), ("ur", "کاوادارتسی بلدیہ"), ("vi", "Đô thị tự trị Kavadarci"), ("zh", "卡瓦達爾齊區")]),
                        unofficial_name_list: ["Kavadarci"].to_vec(),
                    }
                ),
                (
                    "105",
                    Subdivision{
                        name: "Lozovo",
                        country_alpha2: Alpha2::MK,
                        code: "105",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.780833), longitude: Some(21.901111), max_latitude: Some(41.7860898), min_latitude: Some(41.7777045), max_longitude: Some(21.9099512), min_longitude: Some(21.8945146)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوزوفو"), ("bg", "Джумайлия"), ("bn", "লোজোভো পৌরসভ\u{9be}"), ("ccp", "𑄣\u{1112e}𑄎\u{1112e}𑄞\u{1112e}"), ("ceb", "Lozovo"), ("da", "Lozovo Municipality"), ("de", "Opština Lozovo"), ("el", "Δήμος Λοζόβου"), ("en", "Lozovo"), ("es", "Municipalidad de Lozovo"), ("fi", "Lozovon kunta"), ("fr", "Lozovo"), ("gu", "લોઝોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लोज\u{93c}ोवो नगर पालिका"), ("hr", "Općina Lozovo"), ("hy", "Լոզովոյի համայնք"), ("id", "Kotamadya Lozovo"), ("it", "Lozovo"), ("ja", "ロゾヴォ"), ("ka", "ლოზოვოს თემი"), ("kn", "ಲೊಝೊವೊ ಪುರಸಭ\u{cc6}"), ("ko", "로조보 시"), ("lt", "Lozovo savivaldybė"), ("lv", "Lozovo pašvaldība"), ("mk", "Општина Лозово"), ("mr", "लोझोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Lozovo Municipality"), ("nb", "Lozovo kommune"), ("nl", "Lozovo"), ("no", "Lozovo kommune"), ("pl", "Gmina Łozowo"), ("pt", "Lozovo"), ("ru", "Лозово"), ("si", "ලොසොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Llozovës"), ("sr", "Општина Лозово"), ("sr_Latn", "Opština Lozovo"), ("sv", "Lozovo"), ("ta", "லொஸ\u{bcd}ஒவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c4b}జ\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โลโซโว"), ("tr", "Lozova Belediyesi"), ("uk", "Лозово"), ("ur", "لوزوو بلدیہ"), ("vi", "Đô thị tự trị Lozovo"), ("zh", "洛佐沃區")]),
                        unofficial_name_list: ["Lozovo"].to_vec(),
                    }
                ),
                (
                    "106",
                    Subdivision{
                        name: "Negotino",
                        country_alpha2: Alpha2::MK,
                        code: "106",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4989985), longitude: Some(22.0953297), max_latitude: Some(41.6542421), min_latitude: Some(41.348664), max_longitude: Some(22.3007181), min_longitude: Some(21.9803289)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية نيغوتينو"), ("bg", "Неготино"), ("bn", "ন\u{9be}গোটিনো পৌরসভ\u{9be}"), ("ca", "Negotino"), ("ccp", "𑄚𑄬𑄉\u{1112e}𑄑\u{11128}𑄚\u{1112e}"), ("ceb", "Negotino"), ("da", "Negotino Municipality"), ("de", "Opština Negotino"), ("el", "Δήμος Νεγκότινου"), ("en", "Negotino"), ("es", "Municipalidad de Negotino"), ("fi", "Negotinon kunta"), ("fr", "Municipalité de Negotino"), ("gu", "ન\u{ac7}ગોટીનો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "न\u{947}गोतिनो नगर पालिका"), ("hr", "Općina Negotino"), ("id", "Kotamadya Negotino"), ("it", "Comune di Negotino"), ("ja", "ネゴティノ"), ("ka", "ნეგოტინოს თემი"), ("kn", "ನ\u{cc6}ಗೊಟ\u{cbf}ನೋ ಪುರಸಭ\u{cc6}"), ("ko", "네고티노 시"), ("lt", "Negotino savivaldybė"), ("lv", "Negotino pašvaldība"), ("mk", "Општина Неготино"), ("mr", "न\u{947}गोतो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Negotino Municipality"), ("nb", "Negotino kommune"), ("nl", "Negotino"), ("no", "Negotino kommune"), ("pl", "Gmina Negotino"), ("pt", "Negotino"), ("ru", "Неготино"), ("si", "නෙගොට\u{dd2}නෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Negotinës"), ("sr", "Општина Неготино"), ("sr_Latn", "Opština Negotino"), ("sv", "Negotino"), ("ta", "னேகொட\u{bcd}டினோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "న\u{c46}గ\u{c4b}ట\u{c3f}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "น\u{e35}โกต\u{e34}โน ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Negotino Belediyesi"), ("uk", "Неготино"), ("ur", "نیگوتینو بلدیہ"), ("vi", "Đô thị tự trị Negotino"), ("zh", "內戈蒂諾區")]),
                        unofficial_name_list: ["Negotino"].to_vec(),
                    }
                ),
                (
                    "107",
                    Subdivision{
                        name: "Rosoman",
                        country_alpha2: Alpha2::MK,
                        code: "107",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4848006), longitude: Some(21.8807064), max_latitude: Some(41.555479), min_latitude: Some(41.4309889), max_longitude: Some(21.984525), min_longitude: Some(21.8100951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Росоман"), ("ccp", "𑄢\u{1112e}𑄥\u{1112e}𑄟𑄚\u{11134}"), ("ceb", "Rosoman"), ("cs", "Opština Rosoman"), ("de", "Opština Rosoman"), ("el", "Δήμος του Ρόσομαν"), ("en", "Rosoman"), ("es", "Municipalidad de Rosoman"), ("fr", "Rosoman"), ("hr", "Općina Rosoman"), ("it", "Rosoman"), ("ja", "ロソマン"), ("ka", "როსომანის თემი"), ("ko", "로소만 시"), ("mk", "Општина Росоман"), ("nl", "Rosoman"), ("sq", "Komuna e Rosomanit"), ("sr", "Општина Росоман"), ("sr_Latn", "Opština Rosoman"), ("sv", "Rosoman"), ("uk", "Росоман"), ("ur", "روسومان بلدیہ"), ("zh", "羅索曼區")]),
                        unofficial_name_list: ["Rosoman"].to_vec(),
                    }
                ),
                (
                    "108",
                    Subdivision{
                        name: "Sveti Nikole",
                        country_alpha2: Alpha2::MK,
                        code: "108",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.866667), longitude: Some(21.95), max_latitude: Some(41.8749934), min_latitude: Some(41.8555619), max_longitude: Some(21.9508554), min_longitude: Some(21.9269728)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سفيتي نيكولا"), ("bg", "Свети Никола"), ("bn", "ভেটি নিকোল পৌরসভ\u{9be}"), ("ccp", "𑄥\u{11133}𑄞𑄬𑄑\u{11128} 𑄚\u{11128}𑄇\u{1112e}𑄣\u{11134}"), ("ceb", "Sveti Nikole"), ("da", "Sveti Nikole Municipality"), ("de", "Opština Sveti Nikole"), ("el", "Δήμος Σβέτι Νίκολα"), ("en", "Sveti Nikole"), ("es", "Municipalidad de Sveti Nikole"), ("fi", "Sveti Nikolen kunta"), ("fr", "Sveti Nikolé"), ("gu", "સ\u{acd}વ\u{ac7}તી નિકોલ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}व\u{947}ती निकोल नगरपालिका"), ("hr", "Općina Sveti Nikole"), ("id", "Kotamadya Sveti Nikole"), ("it", "Sveti Nikole"), ("ja", "スヴェティ・ニコレ"), ("ka", "სვეტი-ნიკოლეს თემი"), ("kn", "ಸ\u{ccd}ವ\u{cc6}ಟ\u{cbf} ನ\u{cbf}ಕೋಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "스베티니콜레 시"), ("lt", "Šventosios Nikolės savivaldybė"), ("lv", "Sveti Nikole pašvaldība"), ("mk", "Општина Свети Николе"), ("mr", "स\u{94d}व\u{947}नटी निकोल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sveti Nikole Municipality"), ("nb", "Sveti Nikole kommune"), ("nl", "Sveti Nikole"), ("no", "Sveti Nikole kommune"), ("pl", "Gmina Sveti Nikole"), ("pt", "Município de Sveki Nikole"), ("ru", "Свети-Николе"), ("si", "ස\u{dca}වෙට\u{dd2} න\u{dd2}කොලේ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Sveti Nikollës"), ("sr", "Општина Свети Никола"), ("sr_Latn", "Opština Sveti Nikola"), ("sv", "Sveti Nikole (kommun)"), ("ta", "ஸ\u{bcd}வெடி நிக\u{bcd}கோல\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}వ\u{c46}\u{c46}ట\u{c3f} న\u{c3f}క\u{c4b}ల\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องบ\u{e31}ตต\u{e34}คาโลอา"), ("tr", "Sveti Nikole Belediyesi"), ("uk", "Светий Николе"), ("ur", "سویتی نیلولے بلدیہ"), ("vi", "Đô thị tự trị Sveti Nikole"), ("zh", "聖尼古萊區")]),
                        unofficial_name_list: ["Sveti Nikole"].to_vec(),
                    }
                ),
                (
                    "109",
                    Subdivision{
                        name: "Čaška",
                        country_alpha2: Alpha2::MK,
                        code: "109",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.647438), longitude: Some(21.6914115), max_latitude: Some(41.6554159), min_latitude: Some(41.6380384), max_longitude: Some(21.697113), min_longitude: Some(21.6835701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تشاشكا"), ("bg", "Чашка"), ("bn", "ক\u{9cd}য\u{9be}স\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ca", "Čaška"), ("ccp", "𑄇𑄌\u{11134}𑄇"), ("ceb", "Opština Čaška"), ("cs", "Opština Čaška"), ("da", "Čaška Municipality"), ("de", "Opština Čaška"), ("el", "Δήμος Τσάσκας"), ("en", "Čaška"), ("es", "Municipalidad de Čaška"), ("fi", "Čaškan kunta"), ("fr", "Tchachka"), ("gu", "કસ\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कास\u{94d}का नगर पालिका"), ("hr", "Općina Čaška"), ("id", "Kotamadya Čaška"), ("it", "Čaška"), ("ja", "チャシュカ"), ("ka", "ჩაშკის თემი"), ("kn", "ಸ\u{cbf}ಸ\u{ccd}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "차슈카 시"), ("lt", "Čaškos savivaldybė"), ("lv", "Čaškas pašvaldība"), ("mk", "Општина Чашка"), ("mr", "क\u{945}स\u{94d}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Caska Municipality"), ("nb", "Caska Kommune"), ("nl", "Čaška"), ("no", "Caska Kommune"), ("pl", "Gmina Czaszka"), ("pt", "Município de Caska"), ("ru", "Чашка"), ("si", "කස\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Çashkës"), ("sr", "Општина Чашка"), ("sr_Latn", "Opština Čaška"), ("sv", "Casjka"), ("ta", "கஸ\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3e}స\u{c4d}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โอไลน\u{e35}"), ("tr", "Caska Belediyesi"), ("uk", "Чашка"), ("ur", "چاشکا بلدیہ"), ("vi", "Đô thị tự trị Caska"), ("zh", "查什卡区")]),
                        unofficial_name_list: ["Čaška"].to_vec(),
                    }
                ),
                (
                    "201",
                    Subdivision{
                        name: "Berovo",
                        country_alpha2: Alpha2::MK,
                        code: "201",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.71), longitude: Some(22.85), max_latitude: Some(41.7220225), min_latitude: Some(41.6987916), max_longitude: Some(22.8621305), min_longitude: Some(22.8312957)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيروفو"), ("bg", "Берово (община)"), ("bn", "বেরোভো পৌরসভ\u{9be}"), ("ccp", "𑄝𑄬𑄢\u{1112e}𑄞\u{1112e}"), ("ceb", "Berovo"), ("da", "Berovo Municipality"), ("de", "Opština Berovo"), ("el", "Δήμος Μπερόβου"), ("en", "Berovo"), ("es", "Municipalidad de Berovo"), ("fi", "Berovon kunta"), ("fr", "Municipalité de Berovo"), ("gu", "બ\u{ac7}રોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{947}रोवो नगरपालिका"), ("hr", "Općina Berovo"), ("hu", "Berovo"), ("id", "Kotamadya Berovo"), ("it", "Berovo"), ("ja", "ベロヴォ (マケドニア)"), ("ka", "ბეროვოს თემი"), ("kn", "ಬ\u{cc6}ರೊವೊ ಪುರಸಭ\u{cc6}"), ("ko", "베로보 시"), ("lt", "Berovo savivaldybė"), ("lv", "Berovo pašvaldība"), ("mk", "Општина Берово"), ("mr", "ब\u{947}रोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Berovo Municipality"), ("nb", "Berovo kommune"), ("nl", "Berevo"), ("no", "Berovo kommune"), ("pl", "Gmina Berowo"), ("pt", "Município de Berovo"), ("ru", "Берово (община)"), ("si", "බෙරෝවො නගර සභ\u{dcf}ව"), ("sq", "Komuna e Berovës"), ("sr", "Општина Берово"), ("sr_Latn", "Opština Berovo"), ("sv", "Berovo"), ("ta", "பிர\u{bbe}வோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c46}ర\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องมาร\u{e35}บอร\u{e4c}"), ("tr", "Berovo Belediyesi"), ("uk", "Берово"), ("ur", "بیروو بلدیہ"), ("vi", "Đô thị tự trị Berovo"), ("zh", "贝罗沃区")]),
                        unofficial_name_list: ["Berovo"].to_vec(),
                    }
                ),
                (
                    "202",
                    Subdivision{
                        name: "Vinica",
                        country_alpha2: Alpha2::MK,
                        code: "202",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.233083), longitude: Some(28.468217), max_latitude: Some(49.27902), min_latitude: Some(49.190448), max_longitude: Some(28.5710879), min_longitude: Some(28.3681799)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فينيتسا"), ("bg", "Виница"), ("bn", "ভিনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄞\u{11128}𑄚\u{11128}𑄇"), ("ceb", "Vinica"), ("da", "Vinica Municipality"), ("de", "Opština Vinica"), ("el", "Δήμος Βίνιτσας"), ("en", "Vinica"), ("es", "Municipalidad de Vinica"), ("fi", "Vinican kunta"), ("fr", "Municipalité de Vinitsa"), ("gu", "વિનીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "विनिका नगर पालिका"), ("hr", "Općina Vinica"), ("id", "Kotamadya Vinica"), ("it", "Comune di Vinica"), ("ja", "ヴィニツァ"), ("ka", "ვინიცის თემი"), ("kn", "ವ\u{cbf}ನ\u{cbf}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "비니차 시"), ("lt", "Vinicos savivaldybė, Makedonija"), ("lv", "Vinicas pašvaldība"), ("mk", "Општина Виница"), ("mr", "व\u{94d}हिनीचा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vinica Municipality"), ("nb", "Vinica Kommune"), ("nl", "Vinica"), ("no", "Vinica Kommune"), ("pl", "Gmina Winica"), ("pt", "Município de Vinica"), ("ru", "Виница"), ("si", "ව\u{dd2}න\u{dd2}ස\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Vinicës"), ("sr", "Општина Виница"), ("sr_Latn", "Opština Vinica"), ("sv", "Vinica (kommun)"), ("ta", "வினிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3f}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องว\u{e34}น\u{e34}ก\u{e49}า"), ("tr", "Vinica Belediyesi"), ("uk", "Виниця"), ("ur", "وینیتسا بلدیہ، مقدونیہ"), ("vi", "Đô thị tự trị Vinica"), ("zh", "维尼察区")]),
                        unofficial_name_list: ["Vinica"].to_vec(),
                    }
                ),
                (
                    "203",
                    Subdivision{
                        name: "Delčevo",
                        country_alpha2: Alpha2::MK,
                        code: "203",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9764), longitude: Some(22.7674), max_latitude: Some(41.9899275), min_latitude: Some(41.9538413), max_longitude: Some(22.7877689), min_longitude: Some(22.7602601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ديلتشيفو"), ("bg", "Царево село"), ("bn", "ডেলচেভো পৌরসভ\u{9be}"), ("ca", "Delčevo"), ("ccp", "𑄓𑄬𑄣\u{11134}𑄥\u{11128}𑄚\u{1112e}"), ("ceb", "Opština Delčevo"), ("da", "Delčevo Municipality"), ("de", "Opština Delčevo"), ("el", "Δήμος Ντελτσέβου"), ("en", "Delčevo"), ("es", "Municipalidad de Delčevo"), ("fi", "Delčevon kunta"), ("fr", "Municipalité de Deltchevo"), ("gu", "ડ\u{ac7}લક\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{947}ल\u{94d}क\u{947}वो नगर पालिका"), ("hr", "Općina Delčevo"), ("id", "Kotamadya Delčevo"), ("it", "Delčevo"), ("ja", "デルチェヴォ"), ("ka", "დელჩევოს თემი"), ("kn", "ಡ\u{cc6}ಲ\u{ccd}ಸ\u{cc6}ವೊ ಪುರಸಭ\u{cc6}"), ("ko", "델체보 시"), ("lt", "Delčevo savivaldybė"), ("lv", "Delčevo pašvaldība"), ("mk", "Општина Делчево"), ("mr", "ड\u{947}लस\u{947}वो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Delcevo Municipality"), ("nb", "Delcevo Kommune"), ("nl", "Delčevo"), ("no", "Delcevo Kommune"), ("pl", "Gmina Dełczewo"), ("pt", "Município de Delcevo"), ("ru", "Делчево"), ("si", "ඩෙල\u{dca}කෙවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Dellçevës"), ("sr", "Општина Делчево"), ("sr_Latn", "Opština Delčevo"), ("sv", "Delčevo"), ("ta", "டெல\u{bcd}சவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c46}ల\u{c4d}స\u{c47}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเดลเซโว"), ("tr", "Delcevo Belediyesi"), ("uk", "Делчево"), ("ur", "بلدیہ دیلچیوو"), ("vi", "Đô thị tự trị Delcevo"), ("zh", "德尔塞沃区")]),
                        unofficial_name_list: ["Delčevo"].to_vec(),
                    }
                ),
                (
                    "204",
                    Subdivision{
                        name: "Zrnovci",
                        country_alpha2: Alpha2::MK,
                        code: "204",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8228221), longitude: Some(22.4172256), max_latitude: Some(41.8693271), min_latitude: Some(41.788068), max_longitude: Some(22.4490741), min_longitude: Some(22.354366)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية زرنوفتشي"), ("bg", "Зърновци"), ("bn", "জ\u{9be}র\u{9cd}নোভচি পৌরসভ\u{9be}"), ("ccp", "𑄚\u{1112e}𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Zrnovci"), ("da", "Zrnovci Municipality"), ("de", "Opština Zrnovci"), ("el", "Δήμος Ζιρνοφτσίου"), ("en", "Zrnovci"), ("es", "Municipalidad de Zrnovci"), ("fi", "Zrnovcin kunta"), ("fr", "Zrnovtsi"), ("gu", "ઝ\u{acd}રનૉવકી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}रनोव\u{94d}की नगर पालिका"), ("hr", "Općina Zrnovci"), ("id", "Kotamadya Zrnovci"), ("it", "Zrnovci"), ("ja", "ズルノヴツィ"), ("ka", "ზრნოვცის თემი"), ("kn", "ಜರ\u{ccd}ನೋವ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "즈르노브치 시"), ("lt", "Zrnovcio savivaldybė"), ("lv", "Zrnovci pašvaldība"), ("mk", "Општина Зрновци"), ("mr", "झ\u{94d}रो\u{94d}नोव\u{94d}ही म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zrnovci Municipality"), ("nb", "Zrnovci kommune"), ("nl", "Zrnovci"), ("no", "Zrnovci kommune"), ("pl", "Gmina Zrnovci"), ("pt", "Município de Zrnovci"), ("ru", "Зрновци"), ("si", "ස\u{dca}රෝනොව\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Zërnocit"), ("sr", "Општина Зрновци"), ("sr_Latn", "Opština Zrnovci"), ("sv", "Zrnovci (kommun)"), ("ta", "ஸ\u{bcd}ரனோவசி நகர\u{bbe}ட\u{bcd}சி"), ("te", "జర\u{c4d}న\u{c4b}వ\u{c4d}చ\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ซาโนวซ\u{e34}"), ("tr", "Zmovci Belediyesi"), ("uk", "Зрновці (община)"), ("ur", "زرنووتسی بلدیہ"), ("vi", "Đô thị tự trị Zrnovci"), ("zh", "日尔诺维奇区")]),
                        unofficial_name_list: ["Zrnovci"].to_vec(),
                    }
                ),
                (
                    "205",
                    Subdivision{
                        name: "Karbinci",
                        country_alpha2: Alpha2::MK,
                        code: "205",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8180159), longitude: Some(22.2324758), max_latitude: Some(41.822406), min_latitude: Some(41.8096282), max_longitude: Some(22.2458124), min_longitude: Some(22.2270155)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاربينتسي"), ("bg", "Карбинци"), ("bn", "ক\u{9be}র\u{9be}বিনকি"), ("ccp", "𑄇𑄢\u{11134}𑄝\u{11128}𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Karbinci"), ("cs", "Opština Karbinci"), ("da", "Karbinci"), ("de", "Opština Karbinci"), ("el", "Δήμος Καρμπιντσίου"), ("en", "Karbinci"), ("es", "Municipalidad de Karbinci"), ("fi", "Karbinci"), ("fr", "Karbintsi"), ("gu", "કર\u{acd}બી\u{a82}ન\u{acd}સી"), ("hi", "कारबि\u{902}ची"), ("hr", "Općina Karbinci"), ("id", "Karbinci"), ("it", "Karbinci"), ("ja", "カルビンツィ"), ("ka", "კარბინცის თემი"), ("kn", "ಕಾರ\u{ccd}ಬ\u{cbf}ನ\u{ccd}ಸ\u{cbf}"), ("ko", "카르빈치 시"), ("lt", "Karbinčio savivaldybė"), ("lv", "Karbinci"), ("mk", "Општина Карбинци"), ("mr", "कार\u{94d}बीन\u{94d}ची"), ("ms", "Karbinci"), ("nb", "Karbinci"), ("nl", "Karbinci"), ("no", "Karbinci"), ("pl", "Karbinci"), ("pt", "Karbinci"), ("ru", "Карбинци"), ("si", "කර\u{dca}බ\u{dd2}න\u{dca}ස\u{dd2}"), ("sq", "Komuna e Karbincës"), ("sr", "Општина Карбинци"), ("sr_Latn", "Opština Karbinci"), ("sv", "Karbinci"), ("ta", "கர\u{bcd}பின\u{bcd}சி"), ("te", "క\u{c3e}ర\u{c4d}బ\u{c3f}న\u{c4d}చ\u{c3f}"), ("th", "จ\u{e31}งหว\u{e31}ดฮามาดาน"), ("tr", "Karbinci"), ("uk", "Карбинці"), ("ur", "کاربنتسی بلدیہ"), ("vi", "Karbinci"), ("zh", "卡尔宾奇区")]),
                        unofficial_name_list: ["Karbinci"].to_vec(),
                    }
                ),
                (
                    "206",
                    Subdivision{
                        name: "Kočani",
                        country_alpha2: Alpha2::MK,
                        code: "206",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.91680480000001), longitude: Some(22.4082849), max_latitude: Some(41.93393890000001), min_latitude: Some(41.9041297), max_longitude: Some(22.4394573), min_longitude: Some(22.3931301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوتشاني"), ("bg", "Кочани"), ("bn", "কৈয\u{9bc}\u{9be}নি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{1112e}𑄥𑄚\u{11128}"), ("ceb", "Opština Kočani"), ("da", "Kocani"), ("de", "Opština Kočani"), ("el", "Δήμος Κότσανης"), ("en", "Kočani"), ("es", "Municipalidad de Kočani"), ("fi", "Kočanin kunta"), ("fr", "municipalité de Kotchani"), ("gu", "કોકની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोसानी नगरपालिका"), ("hr", "Općina Kočani"), ("id", "Kotamadya Kočani"), ("it", "Comune di Kočani"), ("ja", "クアツェーニ"), ("ka", "კოჩანის თემი"), ("kn", "ಕೊಕ\u{ccd}ಕಾನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "코차니 시"), ("lt", "Kočanio savivaldybė"), ("lv", "Kočanu pašvaldība"), ("mk", "Општина Кочани"), ("mr", "कोकनी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kocani Municipality"), ("nb", "Kocani kommune"), ("nl", "Kotsjani"), ("no", "Kocani kommune"), ("pl", "Gmina Koczani"), ("pt", "Município de Kocani"), ("ru", "Кочани"), ("si", "කොක\u{dcf}න\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Koçanit"), ("sr", "Општина Кочани"), ("sr_Latn", "Opština Kočani"), ("sv", "Kočani"), ("ta", "கொக\u{bcd}க\u{bbe}ணி நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}క\u{c3e}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโคซาน\u{e34}"), ("tr", "Kocani Belediyesi"), ("uk", "Кочани (община)"), ("ur", "بلدیہ کوچانی"), ("vi", "Đô thị tự trị Kocani"), ("zh", "科查尼區")]),
                        unofficial_name_list: ["Kočani"].to_vec(),
                    }
                ),
                (
                    "207",
                    Subdivision{
                        name: "Makedonska Kamenica",
                        country_alpha2: Alpha2::MK,
                        code: "207",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.019722), longitude: Some(22.591667), max_latitude: Some(42.0301294), min_latitude: Some(42.0103066), max_longitude: Some(22.5955296), min_longitude: Some(22.5755525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماكيدونسكا كامينيكا"), ("bg", "Каменица"), ("bn", "মেক\u{9be}ডনস\u{9cd}ক\u{9be} ক\u{9be}মেনিয\u{9bc}\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄟𑄇𑄬𑄓\u{1112e}𑄚\u{11134}𑄇 𑄇𑄟𑄬𑄚\u{11128}𑄇"), ("ceb", "Makedonska Kamenica"), ("da", "Makedonska Kamenica Municipality"), ("de", "Opština Makedonska Kamenica"), ("el", "Δήμος Ματσεντόνσκα Καμένιτσα"), ("en", "Makedonska Kamenica"), ("es", "Municipalidad de Makedonska Kamenica"), ("fi", "Makedonska Kamenican kunta"), ("fr", "Makedonska Kamenitsa"), ("gu", "મ\u{ac7}ક\u{ac7}ડોનસ\u{acd}કા ક\u{ac7}મ\u{ac7}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{948}क\u{947}डो\u{902}स\u{94d}का कम\u{948}निका नगरपालिका"), ("hr", "Općina Makedonska Kamenica"), ("id", "Kotamadya Makedonska Kamenica"), ("it", "Makedonska Kamenica"), ("ja", "マケドンスカ・カメニツァ"), ("ka", "მაკედონსკა-კამენიცის თემი"), ("kn", "ಮ\u{ccd}ಯಾಕ\u{cc6}ಡೊನ\u{ccd}ಸ\u{ccd}ಕಾ ಕಾಮ\u{cc6}ನ\u{cbf}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "마케돈스카카메니차 시"), ("lt", "Makedonijos Kamenicos provincija"), ("lv", "Maķedonijas Kamenicas province"), ("mk", "Општина Македонска Каменица"), ("mr", "म\u{945}क\u{94d}डो\u{902}स\u{94d}का काम\u{947}\u{902}इका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Makedonska Kamenica"), ("nb", "Makedonska Kamencia kommune"), ("nl", "Makedonska Kamenica"), ("no", "Makedonska Kamencia kommune"), ("pl", "Gmina Makedonska Kamenica"), ("pt", "Makedonska Kamencia kommune"), ("ru", "Македонска-Каменица"), ("si", "මකෙඩොන\u{dca}ස\u{dca}ක\u{dcf} කමෙන\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kamenicës"), ("sr", "Општина Македонска Каменица"), ("sr_Latn", "Opština Makedonska Kamenica"), ("sv", "Makedonska Kamenica"), ("ta", "மேட\u{bbe}ன\u{bcd}ஸ\u{bcd}க\u{bbe} கமெனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}క\u{c46}డ\u{c4b}న\u{c4d}స\u{c4d}క\u{c3e} క\u{c3e}మ\u{c46}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลมากาดอนสกา คาเมน\u{e34}กา"), ("tr", "Makedonska Kamenica Belediyesi"), ("uk", "Македонська-Камениця"), ("ur", "ماکیدونسکا کامینیتسا بلدیہ"), ("vi", "Đô thị tự trị Makedonska Kamenica"), ("zh", "馬其頓卡梅尼察區")]),
                        unofficial_name_list: ["Makedonska Kamenica"].to_vec(),
                    }
                ),
                (
                    "208",
                    Subdivision{
                        name: "Pehčevo",
                        country_alpha2: Alpha2::MK,
                        code: "208",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.76), longitude: Some(22.88), max_latitude: Some(41.7662424), min_latitude: Some(41.7562428), max_longitude: Some(22.8996384), min_longitude: Some(22.8748494)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيتسيفو"), ("bg", "Пехчево"), ("bn", "পেচেভো পৌরসভ\u{9be}"), ("ccp", "𑄛𑄬𑄦\u{11134}𑄥𑄬𑄞\u{1112e}"), ("ceb", "Opština Pehčevo"), ("da", "Pehčevo Municipality"), ("de", "Opština Pehčevo"), ("el", "Δήμος Πεχτσέβου"), ("en", "Pehčevo"), ("es", "Municipalidad de Pehčevo"), ("fi", "Pehčevon kunta"), ("fr", "Pehtchevo"), ("gu", "પ\u{ac7}શ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पायसीवो नगरपालिका"), ("hr", "Općina Pehčevo"), ("id", "Kotamadya Pehčevo"), ("it", "Pehčevo"), ("ja", "ペフチェヴォ"), ("ka", "პეხჩევოს თემი"), ("kn", "ಪ\u{cc6}ಹ\u{cc6}ಸ\u{cc6}ವೊ ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "페흐체보 시"), ("lt", "Pechčevo savivaldybė"), ("lv", "Pehčevo pašvaldība"), ("mk", "Општина Пехчево"), ("mr", "प\u{947}य\u{947}वो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Pehcevo Municipality"), ("nb", "Pehcevo kommune"), ("nl", "Pehčevo"), ("no", "Pehcevo kommune"), ("pl", "Gmina Pehčevo"), ("pt", "Município de Pehcevo"), ("ru", "Пехчево"), ("si", "පෙසෙවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Peçevës"), ("sr", "Општина Пехчево"), ("sr_Latn", "Opština Pehčevo"), ("sv", "Pehčevo (kommun)"), ("ta", "பெஹஸ\u{bcd}வோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c46}హ\u{c4d}స\u{c47}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเปคเซโว"), ("tr", "Pehcevo Belediyesi"), ("uk", "Пехчево (община)"), ("ur", "بلدیہ پیخچوو"), ("vi", "Đô thị tự trị Pehcevo"), ("zh", "佩赫塞沃區")]),
                        unofficial_name_list: ["Pehčevo"].to_vec(),
                    }
                ),
                (
                    "209",
                    Subdivision{
                        name: "Probištip",
                        country_alpha2: Alpha2::MK,
                        code: "209",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9948176), longitude: Some(22.1877315), max_latitude: Some(42.0117575), min_latitude: Some(41.9841539), max_longitude: Some(22.1926832), min_longitude: Some(22.1594237)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بروتشتيب"), ("bg", "Пробищип"), ("bn", "প\u{9cd}রবিস\u{9cd}টিপ পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11133}𑄢\u{1112e}𑄝\u{11128}𑄌\u{11134}𑄑\u{11128}𑄛\u{11134}"), ("ceb", "Opština Probištip"), ("da", "Probištip Municipality"), ("de", "Opština Probištip"), ("el", "Δήμος Πρόμπιστιπ"), ("en", "Probištip"), ("es", "Municipalidad de Probištip"), ("fi", "Probištipin kunta"), ("fr", "Probichtip"), ("gu", "પ\u{acd}રોબિસ\u{acd}ટિપ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}रोबिस\u{94d}टिप नगर पालिका"), ("hr", "Općina Probištip"), ("id", "Kotamadya Probištip"), ("it", "Probištip"), ("ja", "プロビシュティプ"), ("ka", "პრობიშტიპის თემი"), ("kn", "ಪ\u{ccd}ರೋಬ\u{cbf}ಸ\u{ccd}ಟ\u{cbf}ಪ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "프로비슈티프 시"), ("lt", "Probištipo savivaldybė"), ("lv", "Probištipas pašvaldība"), ("mk", "Општина Пробиштип"), ("mr", "प\u{94d}रोबिस\u{94d}टिप म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Probistip Municipality"), ("nb", "Probistip kommune"), ("nl", "Probištip"), ("no", "Probistip kommune"), ("pl", "Gmina Probištip"), ("pt", "Município de Probistip"), ("ru", "Пробиштип"), ("si", "ප\u{dca}රෝබ\u{dd2}ස\u{dca}ට\u{dd2}ප\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Probishtipit"), ("sr", "Општина Пробиштип"), ("sr_Latn", "Opština Probištip"), ("sv", "Probistip kommun"), ("ta", "ப\u{bcd}ரோபிஸ\u{bcd}டிப\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c4b}బ\u{c3f}స\u{c4d}ట\u{c3f}ప\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โปรบ\u{e34}สท\u{e34}ป"), ("tr", "Probistip Belediyesi"), ("uk", "Пробиштип"), ("ur", "بلدیہ پوربیشتیپ"), ("vi", "Probistip Đô thị tự trị"), ("zh", "普羅比什蒂普區")]),
                        unofficial_name_list: ["Probištip"].to_vec(),
                    }
                ),
                (
                    "210",
                    Subdivision{
                        name: "Češinovo-Obleševo",
                        country_alpha2: Alpha2::MK,
                        code: "210",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8639316), longitude: Some(22.262246), max_latitude: Some(41.948915), min_latitude: Some(41.788461), max_longitude: Some(22.39093), min_longitude: Some(22.187428)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تشيشينوفو- أبليشيفو"), ("bg", "Чешиново-Облешево"), ("bn", "চেচিনোভো-অব\u{9cd}লেচেভো পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄌\u{11128}𑄚\u{1112e}𑄞\u{1112e}-𑄃\u{11127}𑄛\u{11134}𑄣𑄬𑄥\u{1112e}𑄞\u{1112e}"), ("ceb", "Češinovo-Obleševo"), ("da", "Češinovo-Obleševo Municipality"), ("de", "Opština Češinovo-Obleševo"), ("el", "Δήμος Ομπλεσόβου"), ("en", "Češinovo-Obleševo"), ("es", "Municipalidad de Češinovo-Obleševo"), ("fi", "Češinovo-Obleševon kunta"), ("fr", "Tchéchinovo-Obléchévo"), ("hi", "क\u{947}सिनोवो-ओब\u{94d}ल\u{947}स\u{947}वो नगरपालिका"), ("hr", "Općina Češinovo-Obleševo"), ("hu", "Csesinovo-Oblesevo"), ("id", "Kotamadya Češinovo-Obleševo"), ("it", "Češinovo-Obleševo"), ("ka", "ჩეშინოვო-ობლეშევოს თემი"), ("kn", "ಚ\u{cc6}ಸ\u{cbf}ನೋವೊ-ಓಲ\u{cbf}ಸ\u{cc6}ವೊ ಪುರಸಭ\u{cc6}"), ("ko", "체시노보오블레셰보 시"), ("lt", "Češinovo-Oblešovo savivaldybė"), ("lv", "Češinovo-Obleševo pašvaldība"), ("mk", "Општина Чешиново-Облешево"), ("ms", "Cesinovo-Oblesevo Municipality"), ("nb", "Cesinoco-Oblesevo Kommune"), ("nl", "Češinovo-Obleševo"), ("no", "Cesinoco-Oblesevo Kommune"), ("pl", "Gmina Czeszinowo-Obleszewo"), ("pt", "Cesinoco-Oblesevo"), ("ru", "Чешиново-Облешево"), ("si", "සෙන\u{dca}ස\u{dd2}නොවෝ-ඔබ\u{dca}ලෙසේ වෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Çeshinovo-Obleshevës"), ("sr", "Општина Чешиново-Облешево"), ("sr_Latn", "Opština Češinovo-Obleševo"), ("sv", "Česjinovo-Oblesjevo"), ("ta", "சிசினோவோ -ஒப\u{bcd}ளேஸ\u{bcd}வோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}స\u{c3f}\u{c3f}న\u{c4b}వ\u{c4b}-ఓబ\u{c4d}ల\u{c46}స\u{c47}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เชสส\u{e34}โนโว โอเบสช\u{e34}โว"), ("tr", "Cesinovo-Oblesevo Belediyesi"), ("uk", "Чешиново-Облешево (община)"), ("ur", "بلدیہ چیشینوو-وبلیشوو"), ("vi", "Đô thị tự trị Cesinovo-Oblesevo"), ("zh", "切希诺沃-奥布莱舍沃区")]),
                        unofficial_name_list: ["Češinovo-Obleševo"].to_vec(),
                    }
                ),
                (
                    "211",
                    Subdivision{
                        name: "Štip",
                        country_alpha2: Alpha2::MK,
                        code: "211",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.737503), longitude: Some(22.193558), max_latitude: Some(41.7626373), min_latitude: Some(41.72723939999999), max_longitude: Some(22.2150422), min_longitude: Some(22.167642)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شتيب"), ("bg", "Щип"), ("bn", "স\u{9cd}টিপ পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄑\u{11128}𑄛\u{11134}"), ("ceb", "Opština Štip"), ("da", "Štip Municipality"), ("de", "Opština Štip"), ("el", "Δήμος Στιπ"), ("en", "Štip"), ("es", "Municipalidad de Štip"), ("fi", "Štipn kunta"), ("fr", "Municipalité de Chtip"), ("gu", "સ\u{acd}ટિપ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}तिप नगरपालिका"), ("hr", "Općina Štip"), ("id", "Kotamadya Štip"), ("it", "Štip"), ("ja", "シュティプ"), ("ka", "შტიპის თემი"), ("kn", "ಸ\u{ccd}ಟ\u{cbf}ಪ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "슈티프 시"), ("lt", "Štipo savivaldybė"), ("lv", "Štipas pašvaldība"), ("mk", "Општина Штип"), ("mr", "स\u{94d}टिप म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Stip Municipality"), ("nb", "Strip kommune"), ("nl", "Štip"), ("no", "Strip kommune"), ("pl", "Gmina Štip"), ("pt", "Município de Stip"), ("ru", "Штип (община)"), ("si", "ස\u{dca}ට\u{dd2}ප\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Shtipit"), ("sr", "Општина Штип"), ("sr_Latn", "Opština Štip"), ("sv", "Štip"), ("ta", "ஸ\u{bcd}டிப\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ట\u{c3f}ప\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสต\u{e34}ป"), ("tr", "Stip Belediyesi"), ("uk", "Штип"), ("ur", "بلدیہ شتیپ"), ("vi", "Đô thị tự trị Stip"), ("zh", "什提普區")]),
                        unofficial_name_list: ["Štip"].to_vec(),
                    }
                ),
                (
                    "301",
                    Subdivision{
                        name: "Vevčani",
                        country_alpha2: Alpha2::MK,
                        code: "301",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2403), longitude: Some(20.5931), max_latitude: Some(41.25057229999999), min_latitude: Some(41.2352363), max_longitude: Some(20.6043006), min_longitude: Some(20.5857504)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄞𑄬𑄛\u{11134}𑄇𑄚\u{11128}"), ("ceb", "Opština Vevčani"), ("de", "Opština Vevčani"), ("en", "Vevčani"), ("es", "Municipalidad de Vevčani"), ("fr", "Vevtchani"), ("hr", "Općina Vevčani"), ("it", "Vevčani"), ("ja", "ヴェヴチャニ"), ("ka", "ვევჩანის თემი"), ("ko", "베브차니 시"), ("mk", "Општина Вевчани"), ("nl", "Vevčani"), ("pt", "Município de Vevčani"), ("sq", "Komuna e Veçanit"), ("sr", "Општина Вевчани"), ("sr_Latn", "Opština Vevčani"), ("sv", "Vevčani"), ("uk", "Вевчани (община)"), ("ur", "ویوچانی بلدیہ"), ("zh", "維夫查尼區")]),
                        unofficial_name_list: ["Vevčani"].to_vec(),
                    }
                ),
                (
                    "303",
                    Subdivision{
                        name: "Debar",
                        country_alpha2: Alpha2::MK,
                        code: "303",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5266768), longitude: Some(20.5239064), max_latitude: Some(41.54387060000001), min_latitude: Some(41.4924262), max_longitude: Some(20.5436398), min_longitude: Some(20.4922057)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ديبار"), ("bg", "Дебър"), ("bn", "দেব\u{9be}র পৌরসভ\u{9be}"), ("ccp", "𑄓𑄬𑄝\u{11127}𑄢\u{11134}"), ("ceb", "Debar"), ("da", "Debar Municipality"), ("de", "Opština Debar"), ("el", "Δήμος Δίβρης"), ("en", "Debar"), ("es", "Municipalidad de Debar"), ("fi", "Debarin kunta"), ("fr", "Municipalité de Debar"), ("gu", "ડ\u{ac7}બર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{947}बार नगर पालिका"), ("hr", "Općina Debar"), ("id", "Kotamadya Debar"), ("it", "Debar"), ("ja", "デバル"), ("ka", "დებარის თემი"), ("kn", "ಡ\u{cc6}ಬಾರ ಪುರಸಭ\u{cc6}"), ("ko", "데바르 시"), ("lt", "Debaro savivaldybė"), ("lv", "Debaras pašvaldība"), ("mk", "Општина Дебар"), ("mr", "ड\u{947}बर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Debar Municipality"), ("nb", "Debar kommune"), ("nl", "Debar"), ("no", "Debar kommune"), ("pl", "Gmina Debar"), ("pt", "Município de Debar"), ("ru", "Дебар"), ("si", "ඩෙබ\u{dcf}ර\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Dibrës"), ("sr", "Општина Дебар"), ("sr_Latn", "Opština Debar"), ("sv", "Debar"), ("ta", "டெப\u{bbe}ர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c46}బ\u{c3e}ర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เดบาร\u{e4c}"), ("tr", "Debre Belediyesi"), ("uk", "Дебар"), ("ur", "دیبار بلدیہ"), ("vi", "Đô thị tự trị Debar"), ("zh", "德巴尔区")]),
                        unofficial_name_list: ["Debar"].to_vec(),
                    }
                ),
                (
                    "304",
                    Subdivision{
                        name: "Debarca",
                        country_alpha2: Alpha2::MK,
                        code: "304",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0010109), longitude: Some(21.4201574), max_latitude: Some(42.0021113), min_latitude: Some(41.9992847), max_longitude: Some(21.4216447), min_longitude: Some(21.4180968)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ديباركا"), ("bg", "Дебърца"), ("bn", "ডিব\u{9be}র\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄓𑄬𑄝\u{11127}𑄢\u{11134}𑄇"), ("ceb", "Debarca"), ("da", "Debarca Municipality"), ("de", "Opština Debarca"), ("el", "Δήμος Ντεμπάρτσας"), ("en", "Debarca"), ("es", "Municipalidad de Debarca"), ("fi", "Debarcan kunta"), ("fr", "Debartsa"), ("gu", "ડ\u{ac7}બાર\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{947}बर\u{94d}का नगर पालिका"), ("hr", "Općina Debarca"), ("hu", "Debarca"), ("id", "Kotamadya Debarca"), ("it", "Debarca"), ("ja", "デバルツァ"), ("ka", "დებარცის თემი"), ("kn", "ಡ\u{cc6}ಬಾರ\u{ccd}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "데바르차 시"), ("lt", "Debarcos savivaldybė"), ("lv", "Debarcas pašvaldība"), ("mk", "Општина Дебарца"), ("mr", "ड\u{947}ब\u{947}र\u{94d}का म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Debarca Municipality"), ("nb", "Debarca kommune"), ("nl", "Debarca"), ("no", "Debarca kommune"), ("pl", "Gmina Debarca"), ("pt", "Município de Debarca"), ("ro", "Comuna Debarca"), ("ru", "Дебарца"), ("si", "ඩෙබර\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Debarcës"), ("sr", "Општина Дебарца"), ("sr_Latn", "Opština Debarca"), ("sv", "Debarca"), ("ta", "டெபற\u{bcd}க\u{bcd}க நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c46}బ\u{c3e}ర\u{c4d}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเดอบาก\u{e49}า"), ("tr", "Debarca Belediyesi"), ("uk", "Дебарця"), ("ur", "دیبرسا بلدیہ"), ("vi", "Đô thị tự trị Debarca"), ("zh", "德巴尔查区")]),
                        unofficial_name_list: ["Debarca"].to_vec(),
                    }
                ),
                (
                    "307",
                    Subdivision{
                        name: "Kičevo",
                        country_alpha2: Alpha2::MK,
                        code: "307",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5158412), longitude: Some(20.9586619), max_latitude: Some(41.5350852), min_latitude: Some(41.4940576), max_longitude: Some(20.973544), min_longitude: Some(20.9361862)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كيتشيفو"), ("be", "абшчына Кічава"), ("bg", "Кичево"), ("bn", "কিয\u{9bc}েভো পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11128}𑄥𑄬𑄞\u{1112e}"), ("ceb", "Opština Kičevo"), ("da", "Kičevo Municipality"), ("de", "Opština Kičevo"), ("el", "Δήμος Κιτσέβου"), ("en", "Kičevo"), ("es", "Municipalidad de Kičevo"), ("fi", "Kičevon kunta"), ("fr", "Kitchevo"), ("gu", "કિસ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "किस\u{947}वो नगरपालिका"), ("hr", "Općina Kičevo"), ("id", "Kotamadya Kičevo"), ("it", "Kičevo"), ("ja", "キチェボ"), ("ka", "კიჩევოს თემი"), ("kn", "ಕ\u{cbf}ಸ\u{cc6}ವೊ ಪುರಸಭ\u{cc6}"), ("ko", "키체보 시"), ("lt", "Kičevo savivaldybė"), ("lv", "Kičevo pasvaldība"), ("mk", "Општина Кичево"), ("mr", "किसिवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kicevo Municipality"), ("nb", "Kicevo kommune"), ("nl", "Kičevo"), ("no", "Kicevo kommune"), ("pl", "Gmina Kičevo"), ("pt", "Município de Kicevo"), ("ru", "Кичево"), ("si", "ක\u{dd2}සේවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kërçovës"), ("sr", "Општина Кичево"), ("sr_Latn", "Opština Kičevo"), ("sv", "Kicevo"), ("ta", "கிஸ\u{bcd}வோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3f}స\u{c46}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องค\u{e34}ซ\u{e35}โว"), ("tr", "Kırçova Belediyesi"), ("uk", "Кичево"), ("ur", "کیچیوو بلدیہ"), ("vi", "Đô thị tự trị Kicevo"), ("zh", "基切沃區")]),
                        unofficial_name_list: ["Kičevo"].to_vec(),
                    }
                ),
                (
                    "308",
                    Subdivision{
                        name: "Makedonski Brod",
                        country_alpha2: Alpha2::MK,
                        code: "308",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5133088), longitude: Some(21.2174329), max_latitude: Some(41.5210334), min_latitude: Some(41.5046204), max_longitude: Some(21.2262081), min_longitude: Some(21.2085485)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماكيدونسكي برود"), ("bg", "Брод (община)"), ("bn", "মেক\u{9be}ডনস\u{9cd}কি ব\u{9cd}রড পৌরসভ\u{9be}"), ("ccp", "𑄟𑄬𑄇𑄓\u{1112e}𑄚\u{11134}𑄇 𑄝\u{11133}𑄢\u{1112e}𑄖\u{11134}"), ("ceb", "Makedonski Brod"), ("da", "Makedonski Brod Municipality"), ("de", "Opština Makedonski Brod"), ("el", "Δήμος Μακεντόσκι Μπροντ"), ("en", "Makedonski Brod"), ("es", "Municipalidad de Makedonski Brod"), ("fi", "Makedonski Brodin kunta"), ("fr", "Makedonski Brod"), ("gu", "મ\u{ac7}ક\u{ac7}ડોન\u{acd}સ\u{acd}કી બ\u{acd}રોડ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{948}क\u{947}डो\u{902}सकी ब\u{94d}रोड नगरपालिका"), ("hr", "Općina Makedonski Brod"), ("id", "Kotamadya Makedonski Brod"), ("it", "Makedonski Brod"), ("ja", "マケドンスキ・ブロド"), ("ka", "მაკედონსკი-ბროდის თემი"), ("kn", "ಮ\u{ccd}ಯಾಕ\u{ccd}ಡೊನ\u{ccd}ಸ\u{ccd}ಕ\u{cbf} ಬ\u{ccd}ರಾಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "마케돈스키브로드 시"), ("lt", "Makedonijos Brodo savivaldybė"), ("lv", "Makedonskij-Brodas pašvaldība"), ("mk", "Општина Македонски Брод"), ("mr", "म\u{945}क\u{94d}डो\u{902}स\u{94d}की ब\u{94d}रॉड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Makedonski Brod"), ("nb", "Makedoniske Brod kommune"), ("nl", "Makedonski Brod"), ("no", "Makedoniske Brod kommune"), ("pl", "Dystrykt Makedonski Brod"), ("pt", "Makedonski Brod"), ("ru", "Македонски-Брод"), ("si", "මකෙදොන\u{dca}ස\u{dca}ක\u{dd2} බ\u{dca}\u{200d}රොඩ\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Brodit"), ("sr", "Општина Македонски Брод"), ("sr_Latn", "Opština Makedonski Brod"), ("sv", "Makedonski Brod"), ("ta", "மகெந\u{bcd}தோன\u{bcd}ஸ\u{bcd}கி ப\u{bcd}ரோட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}క\u{c46}డ\u{c4b}న\u{c4d}స\u{c4d}క\u{c40} బ\u{c4d}ర\u{c4b}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลมเกดอนสก\u{e34}บรอด"), ("tr", "Makedonski Brod Belediyesi"), ("uk", "Македонський Брод (община)"), ("ur", "ماکیدونسکی برود بلدیہ"), ("vi", "Đô thị tự trị Makedonski Brod"), ("zh", "馬其頓布羅德區")]),
                        unofficial_name_list: ["Makedonski Brod"].to_vec(),
                    }
                ),
                (
                    "310",
                    Subdivision{
                        name: "Ohrid",
                        country_alpha2: Alpha2::MK,
                        code: "310",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1230977), longitude: Some(20.8016481), max_latitude: Some(41.148656), min_latitude: Some(41.0793365), max_longitude: Some(20.8305932), min_longitude: Some(20.7540128)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أوهريد"), ("be", "Охрыд"), ("bg", "Охрид"), ("bn", "ওহরিড পৌরসভ\u{9be}"), ("ca", "Municipi d’Ohrid"), ("ccp", "𑄃\u{1112e}𑄦\u{11134}𑄢\u{11128}𑄖\u{11134}"), ("ceb", "Ohrid"), ("da", "Ohrid Municipality"), ("de", "Opština Ohrid"), ("el", "Δήμος Οχρίδας"), ("en", "Ohrid"), ("es", "Municipalidad de Ohrid"), ("fi", "Ohridin kunta"), ("fr", "Municipalité de Ohrid"), ("gu", "ઓહ\u{acd}રીદ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ओहरिड नगरपालिका"), ("hr", "Općina Ohrid"), ("id", "Kotamadya Ohrid"), ("it", "Comune di Ohrid"), ("ja", "オフリド"), ("ka", "ოჰრიდის თემი"), ("kn", "ಓಹ\u{cbf}ರ\u{ccd}ದ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "오흐리드 시"), ("lt", "Ochrido savivaldybė"), ("lv", "Ohridas pašvaldība"), ("mk", "Општина Охрид"), ("mr", "ओहरीड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ohrid Municipality"), ("nb", "Ohrid Kommune"), ("nl", "Ohrid"), ("no", "Ohrid Kommune"), ("pl", "Gmina Ochryda"), ("pt", "Município de Ohrid"), ("ru", "Охрид"), ("si", "ඔහ\u{dca}ර\u{dd2}ඩ\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Ohrit"), ("sr", "Општина Охрид"), ("sr_Latn", "Opština Ohrid"), ("sv", "Ohrid Opština"), ("ta", "ஓரிட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఓహ\u{c4d}ర\u{c3f}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โอร\u{e34}ด"), ("tr", "Ohri Belediyesi"), ("uk", "Охрид (община)"), ("ur", "اوخرید بلدیہ"), ("vi", "Đô thị tự trị Ohrid"), ("zh", "奧赫里德區")]),
                        unofficial_name_list: ["Ohrid"].to_vec(),
                    }
                ),
                (
                    "311",
                    Subdivision{
                        name: "Plasnica",
                        country_alpha2: Alpha2::MK,
                        code: "311",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.45463489999999), longitude: Some(21.1056539), max_latitude: Some(41.48252400000001), min_latitude: Some(41.406865), max_longitude: Some(21.1960519), min_longitude: Some(21.038705)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بلاسنيكا"), ("bg", "Пласница"), ("bn", "প\u{9cd}ল\u{9be}স\u{9cd}নিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11133}𑄣𑄌\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Plasnica"), ("da", "Plasnica Municipality"), ("de", "Opština Plasnica"), ("el", "Δήμος Πλάσνιτσας"), ("en", "Plasnica"), ("es", "Municipalidad de Plasnica"), ("fi", "Plasnican kunta"), ("fr", "Plasnitsa"), ("gu", "પ\u{acd}લાસ\u{acd}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}ल\u{947}सनिका नगरपालिका"), ("hr", "Općina Plasnica"), ("id", "Kotamadya Plasnica"), ("it", "Plasnica"), ("ja", "プラスニツァ"), ("ka", "პლასნიცის თემი"), ("kn", "ಪ\u{ccd}ಲಾಸ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "플라스니차 시"), ("lt", "Plasnicos savivaldybė"), ("lv", "Plasnicas pašvaldība"), ("mk", "Општина Пласница"), ("mr", "प\u{94d}ल\u{947}सनीच म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Plasnica Municipality"), ("nb", "Plasnica Kommune"), ("nl", "Plasnica"), ("no", "Plasnica Kommune"), ("pl", "Gmina Plasnica"), ("pt", "Município de Plasnica"), ("ru", "Пласница"), ("si", "ප\u{dca}ලස\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Plasnicës"), ("sr", "Општина Пласница"), ("sr_Latn", "Opština Plasnica"), ("sv", "Plasnica"), ("ta", "பல\u{bcd}ஸனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ల\u{c3e}స\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องพลาสน\u{e34}คา"), ("tr", "Plasniça Belediyesi"), ("uk", "Пласниця"), ("ur", "پلاسنیتسا بلدیہ"), ("vi", "Đô thị tự trị Plasnica"), ("zh", "普拉斯尼察區")]),
                        unofficial_name_list: ["Plasnica"].to_vec(),
                    }
                ),
                (
                    "312",
                    Subdivision{
                        name: "Struga",
                        country_alpha2: Alpha2::MK,
                        code: "312",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1778353), longitude: Some(20.6783258), max_latitude: Some(41.1973695), min_latitude: Some(41.1677447), max_longitude: Some(20.7272307), min_longitude: Some(20.6343199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ستروغا"), ("bg", "Струга"), ("bn", "স\u{9cd}ট\u{9c1}গ\u{9be} পৌরসভ\u{9be}]"), ("ccp", "𑄑\u{11133}𑄢\u{1112a}𑄉"), ("ceb", "Struga"), ("da", "Struga Municipality"), ("de", "Opština Struga"), ("el", "Δήμος Στρούγγας"), ("en", "Struga"), ("es", "Municipalidad de Struga"), ("fi", "Strugan kunta"), ("fr", "Struga"), ("gu", "સ\u{acd}ટ\u{acd}રગા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}ट\u{94d}र\u{941}गा नगर पालिका"), ("hr", "Općina Struga"), ("hu", "Sztruga község"), ("id", "Kotamadya Struga"), ("it", "Struga"), ("ja", "ストルガ基礎自治体"), ("ka", "სტრუგის თემი"), ("kn", "ಸ\u{ccd}ಟ\u{ccd}ರುಗ ಪುರಸಭ\u{cc6}"), ("ko", "스트루가 시"), ("lt", "Strugos savivaldybė"), ("lv", "Strugas pašvaldība"), ("mk", "Општина Струга"), ("mr", "स\u{94d}त\u{94d}र\u{942}ग म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Struga Municipality"), ("nb", "Struga kommune"), ("nl", "Struga"), ("no", "Struga kommune"), ("pl", "Gmina Struga"), ("pt", "Município de Struga"), ("ru", "Струга"), ("si", "ස\u{dca}ට\u{dca}ර\u{dd4}ග\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Strugës"), ("sr", "Општина Струга"), ("sr_Latn", "Opština Struga"), ("sv", "Struga (kommun)"), ("ta", "ஸ\u{bcd}டருக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ట\u{c4d}రూగ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสตร\u{e39}กา"), ("tr", "Struga Belediyesi"), ("uk", "Струга"), ("ur", "ستوگا بلدیہ"), ("vi", "Đô thị tự trị Struga"), ("zh", "斯特魯加區")]),
                        unofficial_name_list: ["Struga"].to_vec(),
                    }
                ),
                (
                    "313",
                    Subdivision{
                        name: "Centar Župa",
                        country_alpha2: Alpha2::MK,
                        code: "313",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.47852779999999), longitude: Some(20.5602793), max_latitude: Some(41.48230540000001), min_latitude: Some(41.47322459999999), max_longitude: Some(20.5659829), min_longitude: Some(20.5503664)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سنتار جوبا"), ("bg", "Вапа"), ("bn", "সেন\u{9cd}ট\u{9be}র জ\u{9c1}প\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑𑄢\u{11134} 𑄎\u{1112a}𑄛"), ("ceb", "Opština Centar Župa"), ("da", "Centar Župa Municipality"), ("de", "Opština Centar Župa"), ("el", "Δήμος Τσένταρ Ζούπα"), ("en", "Centar Župa"), ("es", "Municipalidad de Centar Župa"), ("fi", "Centar Župan kunta"), ("fr", "Tsentar Joupa"), ("gu", "સ\u{ac7}ન\u{acd}ટર ઝ\u{ac1}પા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{947}\u{902}द\u{94d}रीय ज\u{942}पा नगर पालिका"), ("hr", "Općina Centar Župa"), ("id", "Kotamadya Centar Župa"), ("it", "Centar Župa"), ("ja", "ツェンタル・ジュパ"), ("ka", "ცენტარ-ჟუპის თემი"), ("kn", "ಸ\u{cc6}ಂಟರ\u{ccd} ಝುಪಾ ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "첸타르주파 시"), ("lt", "Centrinės Župos savivaldybė"), ("lv", "Centaras Župas pašvaldība"), ("mk", "Општина Центар Жупа"), ("mr", "स\u{947}\u{902}टर झ\u{942}पा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Centar Zupa Municipality"), ("nb", "Centar Zupa kommune"), ("nl", "Centar Župa"), ("no", "Centar Zupa kommune"), ("pl", "Gmina Centar Żupa"), ("pt", "Município Central de Zupa"), ("ru", "Центар Жупа"), ("si", "මද\u{dca}\u{200d}යම ස\u{dd4}ප\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Qendrës Zhupa"), ("sr", "Општина Центар Жупа"), ("sr_Latn", "Opština Centar Župa"), ("sv", "Centar Župa"), ("ta", "சென\u{bcd}டர\u{bcd} ஸுப\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంట\u{c3e}ర\u{c4d} జూప\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เซนทา ซ\u{e39}พ\u{e35} ม\u{e39}น\u{e34}ซ\u{e34}พาล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Merkez Jupa Belediyesi"), ("uk", "Центр-Жупа"), ("ur", "سینتر ژوپا بلدیہ"), ("vi", "Đô thị tự trị Centar Zupa"), ("zh", "辛塔尔祖帕区")]),
                        unofficial_name_list: ["Centar Župa"].to_vec(),
                    }
                ),
                (
                    "401",
                    Subdivision{
                        name: "Bogdanci",
                        country_alpha2: Alpha2::MK,
                        code: "401",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.203138), longitude: Some(22.5754421), max_latitude: Some(41.2095342), min_latitude: Some(41.1964168), max_longitude: Some(22.5874615), min_longitude: Some(22.5654031)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوجدانسى"), ("bg", "Богданци"), ("bn", "বোগদ\u{9be}ঞ\u{9cd}চি পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11127}𑄇\u{11134}𑄓𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Bogdanci (munisipyo)"), ("da", "Bogdanci Municipality"), ("de", "Opština Bogdanci"), ("el", "Δήμος Μπογδάντσας"), ("en", "Bogdanci"), ("es", "Municipalidad de Bogdanci"), ("fi", "Bogdancin kunta"), ("fr", "Municipalité de Bogdantsi"), ("gu", "બોગડા\u{a82}સી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोगडान\u{94d}ची नगर पालिका"), ("hr", "Općina Bogdanci"), ("hy", "Բոգդանցիի համայնք"), ("id", "Kotamadya Bogdanci"), ("it", "Bogdanci"), ("ja", "ボグダンツィ"), ("ka", "ბოგდანცის თემი"), ("kn", "ಬೊಗ\u{ccd}ದಾನ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "보그단치 시"), ("lt", "Bogdancio savivaldybė"), ("lv", "Bogdancu pašvaldība"), ("mk", "Општина Богданци"), ("mr", "बोगडा\u{902}ची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bogdanci Municipality"), ("nb", "Bogdanci Kommune"), ("nl", "Bogdanci"), ("no", "Bogdanci Kommune"), ("pl", "Gmina Bogdanci"), ("pt", "Município de Bogdanci"), ("ru", "Богданци"), ("si", "බෝග\u{dca}ඩන\u{dca}ච\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Bogdancës"), ("sr", "Општина Богданци"), ("sr_Latn", "Opština Bogdanci"), ("sv", "Bogdanci (kommun)"), ("ta", "போக\u{bcd}த\u{bbe}ன\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c3e}గ\u{c4d}డ\u{c3e}న\u{c4d}స\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "บอกแดนซ\u{e34}"), ("tr", "Bogdanci Municipality"), ("uk", "Богданці"), ("ur", "بوگدانتسی بلدیہ"), ("vi", "Đô thị tự trị Bogdanci"), ("zh", "博格丹奇区")]),
                        unofficial_name_list: ["Bogdanci"].to_vec(),
                    }
                ),
                (
                    "402",
                    Subdivision{
                        name: "Bosilovo",
                        country_alpha2: Alpha2::MK,
                        code: "402",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4406), longitude: Some(22.7278), max_latitude: Some(41.4443203), min_latitude: Some(41.4356014), max_longitude: Some(22.743305), min_longitude: Some(22.7187556)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوسيلوفو"), ("bg", "Босилово"), ("bn", "বসিলভো পৌরসভ\u{9be}"), ("ccp", "𑄝\u{1112e}𑄥\u{11128}𑄣\u{1112e}𑄞\u{1112e}"), ("ceb", "Bosilovo"), ("cs", "Opština Bosilovo"), ("da", "Bosilovo"), ("de", "Opština Bosilovo"), ("el", "Δήμος Μποσιλόβου"), ("en", "Bosilovo"), ("es", "Municipalidad de Bosilovo"), ("fi", "Bosilovon kunta"), ("fr", "Bosilovo"), ("gu", "બોસિલોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोसिलोवो महानगरपालिका"), ("hr", "Općina Bosilovo"), ("id", "Kotamadya Bosilovo"), ("it", "Bosilovo"), ("ja", "ボシロヴォ"), ("ka", "ბოსილოვოს თემი"), ("kn", "ಬೋಸ\u{cbf}ಲೋವೊ ಪುರಸಭ\u{cc6}"), ("ko", "보실로보 시"), ("lt", "Bosiolovo savivaldybė"), ("lv", "Bosilovo pašvaldība"), ("mk", "Општина Босилово"), ("mr", "बॉसिलोवॊ म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Bosilovo"), ("nb", "Bosiovo kommune"), ("nl", "Bosilovo"), ("no", "Bosiovo kommune"), ("pl", "Gmina Bosiłowo"), ("pt", "Município de Bosilovo"), ("ru", "Босилово"), ("si", "බොස\u{dd2}ලොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Bosilovës"), ("sr", "Општина Босиљово"), ("sr_Latn", "Opština Bosiljovo"), ("sv", "Bosilovo"), ("ta", "ப\u{bbe}சிலோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4b}స\u{c3f}ల\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลบ\u{e35}โซโลโว"), ("tr", "Bosilovo Belediyesi"), ("uk", "Босилово"), ("ur", "بوسیلوو بلدیہ"), ("vi", "Đô thị tự trị Bosilovo"), ("zh", "博西洛沃区")]),
                        unofficial_name_list: ["Bosilovo"].to_vec(),
                    }
                ),
                (
                    "403",
                    Subdivision{
                        name: "Valandovo",
                        country_alpha2: Alpha2::MK,
                        code: "403",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.316944), longitude: Some(22.561111), max_latitude: Some(41.3221053), min_latitude: Some(41.3123712), max_longitude: Some(22.5714327), min_longitude: Some(22.5510478)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Валандово"), ("ccp", "𑄞𑄣𑄚\u{11134}𑄓\u{1112e}𑄞\u{1112e}"), ("ceb", "Valandovo"), ("de", "Opština Valandovo"), ("el", "Δήμος Βαλάντοβο"), ("en", "Valandovo"), ("es", "Municipalidad de Valandovo"), ("fr", "Valandovo"), ("hr", "Općina Valandovo"), ("it", "Valandovo"), ("ja", "ヴァランドヴォ"), ("ka", "ვალანდოვოს თემი"), ("ko", "발란도보 시"), ("mk", "Општина Валандово"), ("nl", "Valandovo"), ("ru", "Валандово"), ("sq", "Komuna e Vallandovës"), ("sr", "Општина Валандово"), ("sr_Latn", "Opština Valandovo"), ("tr", "Valandova Belediyesi"), ("uk", "Валандово"), ("ur", "والاندوو بلدیہ"), ("zh", "瓦蘭多沃區")]),
                        unofficial_name_list: ["Valandovo"].to_vec(),
                    }
                ),
                (
                    "404",
                    Subdivision{
                        name: "Vasilevo",
                        country_alpha2: Alpha2::MK,
                        code: "404",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5201985), longitude: Some(22.6437026), max_latitude: Some(41.631592), min_latitude: Some(41.456077), max_longitude: Some(22.731699), min_longitude: Some(22.499725)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فاسيليفو"), ("bg", "Василево"), ("bn", "ভ\u{9be}সিলেভো পৌরসভ\u{9be}"), ("ccp", "𑄞𑄥\u{11128}𑄣𑄬𑄞\u{1112e}"), ("ceb", "Vasilevo"), ("cs", "Opština Vasilevo"), ("da", "Vasilevo Municipality"), ("de", "Opština Vasilevo"), ("el", "Δήμος Βασιλέβου"), ("en", "Vasilevo"), ("es", "Municipalidad de Vasilevo"), ("fa", "وسیلوو مونیکیپلیتی"), ("fi", "Vasilevon kunta"), ("fr", "Vasilevo"), ("gu", "વાસિલ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "वसिल\u{947}वो नगरपालिका"), ("hr", "Općina Vasilevo"), ("hu", "Vaszilevói járás"), ("id", "Kotamadya Vasilevo"), ("it", "Vasilevo"), ("ja", "ヴァシレヴォ"), ("ka", "ვასილევოს თემი"), ("kn", "ವಾಸ\u{cbf}ಲ\u{cbf}ವೊ ಪುರಸಭ\u{cc6}"), ("ko", "바실레보 시"), ("lt", "Vasilevo savivaldybė"), ("lv", "Vasilevo pašvaldība"), ("mk", "Општина Василево"), ("mr", "वासिलीवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vasilevo Municipality"), ("nb", "Vasilevo kommune"), ("nl", "Vasilevo"), ("no", "Vasilevo kommune"), ("pl", "Gmina Vasilevo"), ("pt", "Vasilevo"), ("ru", "Василево"), ("si", "වස\u{dd2}ලේවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Vasilevës"), ("sr", "Општина Васиљево"), ("sr_Latn", "Opština Vasiljevo"), ("sv", "Vasilevo"), ("ta", "வசிலேவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3e}స\u{c3f}ల\u{c46}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "วาซ\u{e34}เลโว"), ("tr", "Vasilev Municipality"), ("uk", "Василево"), ("ur", "واسیلیوو بلدیہ"), ("vi", "Đô thị tự trị Vasilevo"), ("zh", "瓦西列沃區")]),
                        unofficial_name_list: ["Vasilevo"].to_vec(),
                    }
                ),
                (
                    "405",
                    Subdivision{
                        name: "Gevgelija",
                        country_alpha2: Alpha2::MK,
                        code: "405",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.140278), longitude: Some(22.502778), max_latitude: Some(41.1520004), min_latitude: Some(41.1323508), max_longitude: Some(22.5183677), min_longitude: Some(22.4868571)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Гевгели"), ("ccp", "𑄉𑄬𑄛\u{11134}𑄉𑄬𑄣\u{11128}𑄎"), ("ceb", "Gevgelija"), ("de", "Opština Gevgelija"), ("el", "Δήμος Γευγελής"), ("en", "Gevgelija"), ("es", "Municipalidad de Gevgelija"), ("fr", "Gevgelija"), ("hr", "Općina Gevgelija"), ("hy", "Գևգելիայի համայնք"), ("it", "Gevgelija"), ("ka", "გევგელიის თემი"), ("ko", "게브겔리야 시"), ("mk", "Општина Гевгелија"), ("nl", "Gevgelija"), ("pt", "Município de Gevgelija"), ("ro", "Comuna Gevgelija"), ("ru", "Гевгелия"), ("sq", "Komuna e Gjevgjelisë"), ("sr", "Општина Ђевђелија"), ("sr_Latn", "Opština Đevđelija"), ("tr", "Gevgeli Belediyesi"), ("uk", "Гевгелія"), ("ur", "گیوگیلیا بلدیہ"), ("zh", "蓋夫蓋利亞區")]),
                        unofficial_name_list: ["Gevgelija"].to_vec(),
                    }
                ),
                (
                    "406",
                    Subdivision{
                        name: "Dojran",
                        country_alpha2: Alpha2::MK,
                        code: "406",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2436672), longitude: Some(22.6913764), max_latitude: Some(41.2946641), min_latitude: Some(41.1488839), max_longitude: Some(22.7681351), min_longitude: Some(22.5653611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Дойран"), ("ccp", "𑄓\u{11127}𑄌\u{11134}𑄢𑄚\u{11134}"), ("ceb", "Opština Dojran"), ("cs", "Dojran"), ("de", "Opština Dojran"), ("en", "Dojran"), ("es", "Municipalidad de Dojran"), ("fr", "Dojran"), ("hr", "Općina Dojran"), ("hy", "Դոյրան"), ("it", "Dojran"), ("ja", "ドイラン"), ("ka", "დოირანის თემი"), ("ko", "도이란 시"), ("mk", "Општина Дојран"), ("nl", "Dojran"), ("pt", "Município de Dojran"), ("ru", "Дойран"), ("sl", "občina Dojran"), ("sq", "Komuna e Dojranit"), ("sr", "Општина Дојран"), ("sr_Latn", "Opština Dojran"), ("sv", "Opština Dojran"), ("tr", "Doyran Belediyesi"), ("uk", "Дойран"), ("ur", "دویران بلدیہ"), ("zh", "多伊兰区")]),
                        unofficial_name_list: ["Dojran"].to_vec(),
                    }
                ),
                (
                    "407",
                    Subdivision{
                        name: "Konče",
                        country_alpha2: Alpha2::MK,
                        code: "407",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4965779), longitude: Some(22.3829526), max_latitude: Some(41.4997406), min_latitude: Some(41.4918386), max_longitude: Some(22.3924967), min_longitude: Some(22.3740407)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كونتشه"), ("bg", "Конче"), ("bn", "কঞ\u{9cd}চি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11127}𑄚\u{11134}"), ("ceb", "Opština Konče"), ("da", "Konče Municipality"), ("de", "Opština Konče"), ("el", "Δήμος Κόντσε"), ("en", "Konče"), ("es", "Municipalidad de Konče"), ("fi", "Končen kunta"), ("fr", "Kontché"), ("gu", "કોન\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "को\u{902}च\u{947} नगर पालिका"), ("hr", "Općina Konče"), ("id", "Kotamadya Konče"), ("it", "Konče"), ("ja", "コンチェ"), ("ka", "კონჩეს თემი"), ("kn", "ಕೊನ\u{ccd}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "콘체 시"), ("lt", "Končės savivaldybė"), ("lv", "Končes pašvaldība"), ("mk", "Општина Конче"), ("mr", "कोन\u{94d}स म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Konce Municipality"), ("nb", "Konce Kommune"), ("nl", "Konče"), ("no", "Konce Kommune"), ("pl", "Gmina Konče"), ("pt", "Município de Konce"), ("ru", "Конче"), ("si", "කොන\u{dca}සේ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Konçës"), ("sr", "Општина Конче"), ("sr_Latn", "Opština Konče"), ("sv", "Konce"), ("ta", "கொன\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}న\u{c4d}స\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคอนเซ\u{e48}"), ("tr", "Konçe Belediyesi"), ("uk", "Конче"), ("ur", "کونچے بلدیہ"), ("vi", "Đô thị tự trị Konce"), ("zh", "孔切區")]),
                        unofficial_name_list: ["Konče"].to_vec(),
                    }
                ),
                (
                    "408",
                    Subdivision{
                        name: "Novo Selo",
                        country_alpha2: Alpha2::MK,
                        code: "408",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.413889), longitude: Some(22.880833), max_latitude: Some(41.4201658), min_latitude: Some(41.4070199), max_longitude: Some(22.8911822), min_longitude: Some(22.8698898)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية نوفو سيلو"), ("bg", "Ново село"), ("bn", "নোভো সেলো পৌরসভ\u{9be}"), ("ccp", "𑄚\u{1112e}𑄞\u{1112e} 𑄥𑄬𑄣\u{1112e}"), ("ceb", "Novo Selo"), ("cs", "Opština Novo Selo"), ("da", "Novo Selo Municipality"), ("de", "Opština Novo Selo"), ("el", "Δήμος Νόβο Σέλο"), ("en", "Novo Selo"), ("es", "Municipalidad de Novo Selo"), ("fi", "Novo Selon kunta"), ("fr", "Novo Selo"), ("gu", "નોવો સ\u{ac7}લો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "नोवो स\u{947}लो नगर पालिका"), ("hr", "Općina Novo Selo"), ("id", "Kotamadya Novo Selo"), ("it", "Novo Selo"), ("ja", "ノヴォ・セロ"), ("ka", "ნოვო-სელოს თემი"), ("kn", "ನೊವೊ ಸೇಲೋ ಪುರಸಭ\u{cc6}"), ("ko", "노보셀로 시"), ("lt", "Novo Selo savivaldybė"), ("lv", "Novo Selo pašvaldība"), ("mk", "Општина Ново Село"), ("mr", "नोवो स\u{947}लो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Novo Selo Municipality"), ("nb", "Novo Selo kommune"), ("nl", "Novo Selo"), ("no", "Novo Selo kommune"), ("pl", "Gmina Novo Selo"), ("pt", "Município de Novo Selo"), ("ru", "Ново-Село"), ("si", "නොවෝ සෙලෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Novosellës"), ("sr", "Општина Ново Село"), ("sr_Latn", "Opština Novo Selo"), ("sv", "Novo Selo"), ("ta", "நோவோ சேலோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "న\u{c3e}వ\u{c4b} స\u{c46}ల\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โนโว เซโล ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}โต"), ("tr", "Novo Selo Belediyesi"), ("uk", "Ново-Село"), ("ur", "نوو سیلو بلدیہ"), ("vi", "Novo Selo"), ("zh", "諾沃塞洛區")]),
                        unofficial_name_list: ["Novo Selo"].to_vec(),
                    }
                ),
                (
                    "409",
                    Subdivision{
                        name: "Radoviš",
                        country_alpha2: Alpha2::MK,
                        code: "409",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.636111), longitude: Some(22.466667), max_latitude: Some(41.64887359999999), min_latitude: Some(41.62242029999999), max_longitude: Some(22.4798942), min_longitude: Some(22.4439095)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية رادوفيتش"), ("bg", "Радовиш"), ("bn", "র\u{9be}ডোভিস পৌরসভ\u{9be}"), ("ccp", "𑄢𑄓\u{1112e}𑄞\u{11128}𑄌\u{11134}"), ("ceb", "Opština Radoviš"), ("da", "Radoviš Municipality"), ("de", "Opština Radoviš"), ("el", "Δήμος Ράντοβις"), ("en", "Radoviš"), ("es", "Municipalidad de Radoviš"), ("fi", "Radovišin kunta"), ("fr", "Radovich"), ("gu", "રાડોવીસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रादोविस नगरपालिका"), ("hr", "Općina Radoviš"), ("id", "Kotamadya Radoviš"), ("it", "Comune di Radoviš"), ("ja", "ラドヴィシュ"), ("ka", "რადოვიშის თემი"), ("kn", "ರಾಡೋವ\u{cbf}ಸ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "라도비시 시"), ("lt", "Radovišo savivaldybė"), ("lv", "Radovišas pašvaldība"), ("mk", "Општина Радовиш"), ("mr", "र\u{945}डोवीस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Radovis Municipality"), ("nb", "Radovis kommune"), ("nl", "Radoviš"), ("no", "Radovis kommune"), ("pl", "Gmina Radoviš"), ("pt", "Município de Radovis"), ("ru", "Радовиш"), ("si", "රඩොව\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Radovishtit"), ("sr", "Општина Радовиш"), ("sr_Latn", "Opština Radoviš"), ("sv", "Radovisj"), ("ta", "ர\u{bbe}டோவிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3e}డ\u{c4b}వ\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องราโดว\u{e34}ส"), ("tr", "Radoviş Belediyesi"), ("uk", "Радовиш"), ("ur", "رادوویش بلدیہ"), ("vi", "Đô thị tự trị Radovis"), ("zh", "拉多維什區")]),
                        unofficial_name_list: ["Radoviš"].to_vec(),
                    }
                ),
                (
                    "410",
                    Subdivision{
                        name: "Strumica",
                        country_alpha2: Alpha2::MK,
                        code: "410",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4378004), longitude: Some(22.6427428), max_latitude: Some(41.45609169999999), min_latitude: Some(41.4220055), max_longitude: Some(22.6640654), min_longitude: Some(22.6253986)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ستروميتسا"), ("bg", "Струмица"), ("bn", "স\u{9cd}ট\u{9cd}র\u{9c1}মিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄑\u{11133}𑄢\u{1112a}𑄟\u{11128}𑄇"), ("ceb", "Strumica"), ("da", "Strumica Municipality"), ("de", "Opština Strumica"), ("el", "Δήμος Στρούμιτσας"), ("en", "Strumica"), ("es", "Municipalidad de Strumica"), ("fi", "Strumican kunta"), ("fr", "Municipalité de Stroumitsa"), ("gu", "સ\u{acd}ટ\u{acd}ર\u{ac1}મિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}ट\u{94d}र\u{942}मिका नगरपालिका"), ("hr", "Općina Strumica"), ("id", "Kotamadya Strumica"), ("it", "Strumica"), ("ja", "ストルミツァ"), ("ka", "სტრუმიცის თემი"), ("kn", "ಸ\u{ccd}ಟ\u{ccd}ರುಮ\u{cbf}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "스트루미차 시"), ("lt", "Strumicos savivaldybė"), ("lv", "Strumicas pašvaldība"), ("mk", "Општина Струмица"), ("mr", "स\u{94d}ट\u{941}मिका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Strumica Municipality"), ("nb", "Stumica kommune"), ("nl", "Strumica"), ("no", "Stumica kommune"), ("pl", "Gmina Strumica"), ("pt", "Município de Strumica"), ("ru", "Струмица"), ("si", "සට\u{dca}ර\u{dd4}ම\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Strumica"), ("sq", "Komuna e Strumicës"), ("sr", "Општина Струмица"), ("sr_Latn", "Opština Strumica"), ("sv", "Strumica (kommun)"), ("ta", "சற\u{bcd}றுமிக\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ట\u{c4d}రూమ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "สต\u{e39}ไมกา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Ustrumca Belediyesi"), ("uk", "Струмиця"), ("ur", "سترومیتسا بلدیہ"), ("vi", "Đô thị tự trị Strumica"), ("zh", "斯特魯米察區")]),
                        unofficial_name_list: ["Strumica"].to_vec(),
                    }
                ),
                (
                    "501",
                    Subdivision{
                        name: "Bitola",
                        country_alpha2: Alpha2::MK,
                        code: "501",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.031944), longitude: Some(21.334722), max_latitude: Some(41.048879), min_latitude: Some(41.0027271), max_longitude: Some(21.3622927), min_longitude: Some(21.2948782)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيتولا"), ("bg", "Битоля"), ("bn", "বিটোল\u{9be} পৌরসভ\u{9be}"), ("ca", "municipi de Bitola"), ("ccp", "𑄝\u{11128}𑄑\u{1112e}𑄣"), ("da", "Bitola Municipality"), ("de", "Opština Bitola"), ("el", "Δήμος Μοναστηρίου"), ("en", "Bitola"), ("es", "Municipalidad de Bitola"), ("fi", "Bitolan kunta"), ("fr", "Bitola"), ("gu", "બિટોલા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बिटोला नगर पालिका"), ("hr", "Općina Bitolj"), ("id", "Kotamadya Bitola"), ("it", "Bitola"), ("ja", "ビトラ"), ("ka", "ბიტოლის თემი"), ("kn", "ಬ\u{cbf}ಟೋಲಾ ಪುರಸಭ\u{cc6}"), ("ko", "비톨라 시"), ("lt", "Bitolos savivaldybė"), ("lv", "Bitolas pašvaldība"), ("mk", "Општина Битола"), ("mr", "बिटोला म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bitola Municipality"), ("nb", "Bitcola kommune"), ("nl", "Bitola"), ("no", "Bitcola kommune"), ("pl", "Gmina Bitola"), ("pt", "Município de Bitola"), ("ru", "Битола"), ("si", "බ\u{dd2}ටෝල\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Manastirit"), ("sr", "Општина Битољ"), ("sr_Latn", "Opština Bitolj"), ("sv", "Bitcola kommun"), ("ta", "பைடோல\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c3f}ట\u{c4b}ల\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลบ\u{e34}โตลา"), ("tr", "Manastır Belediyesi"), ("uk", "Бітола"), ("ur", "بیتولا بلدیہ"), ("vi", "Đô thị tự trị Bitola"), ("zh", "比托拉区")]),
                        unofficial_name_list: ["Bitola"].to_vec(),
                    }
                ),
                (
                    "502",
                    Subdivision{
                        name: "Demir Hisar",
                        country_alpha2: Alpha2::MK,
                        code: "502",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.22136039999999), longitude: Some(21.2025287), max_latitude: Some(41.2294553), min_latitude: Some(41.212795), max_longitude: Some(21.2090261), min_longitude: Some(21.1942202)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Демир Хисар"), ("ccp", "𑄓𑄬𑄟\u{11128}𑄢\u{11134} 𑄦\u{11128}𑄥𑄢\u{11134}"), ("ceb", "Demir Hisar"), ("de", "Opština Demir Hisar"), ("el", "Δήμος του Ντεμίρ Χισάρ"), ("en", "Demir Hisar"), ("es", "Municipalidad de Demir Hisar"), ("fa", "دمیر هسار"), ("fr", "Demir Hisar"), ("hr", "Općina Demir Hisar"), ("hu", "Demir Hiszar"), ("it", "Demir Hisar"), ("ja", "デミル・ヒサル"), ("ka", "დემირ-ჰისარის თემი"), ("ko", "데미르히사르 시"), ("mk", "Општина Демир Хисар"), ("nl", "Demir Hisar"), ("pl", "Gmina Demir Hisar"), ("pt", "Município de Demir Hisar"), ("ru", "Демир-Хисар"), ("sq", "Komuna e Demir Hisarit"), ("sr", "Општина Демир Хисар"), ("sr_Latn", "Opština Demir Hisar"), ("sv", "Demir Hisar (kommun)"), ("tr", "Demir Hisar Belediyesi"), ("uk", "Демир-Хисар"), ("ur", "دیمیر حسار بلدیہ"), ("zh", "德米尔希萨尔区")]),
                        unofficial_name_list: ["Murgaševo"].to_vec(),
                    }
                ),
                (
                    "503",
                    Subdivision{
                        name: "Dolneni",
                        country_alpha2: Alpha2::MK,
                        code: "503",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.425278), longitude: Some(21.454444), max_latitude: Some(41.4306496), min_latitude: Some(41.4207005), max_longitude: Some(21.459623), min_longitude: Some(21.4454475)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دولنيني"), ("bg", "Долнени"), ("bn", "ডলনেনি পৌরসভ\u{9be}"), ("ca", "Dolneni"), ("ccp", "𑄓\u{1112e}𑄣\u{11134}𑄚𑄬𑄚\u{11128}"), ("ceb", "Dolneni"), ("da", "Dolneni"), ("de", "Opština Dolneni"), ("el", "Δήμος Ντολμένης"), ("en", "Dolneni"), ("es", "Municipalidad de Dolneni"), ("fi", "Dolnenin kunta"), ("fr", "Dolneni"), ("gu", "ડોલન\u{ac7}ની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोलन\u{947}नी नगरपालिका"), ("hr", "Općina Dolneni"), ("id", "Kotamadya Dolneni"), ("it", "Dolneni"), ("ja", "ドルネニ"), ("ka", "დოლნენის თემი"), ("kn", "ಡೊಲ\u{ccd}ನ\u{cc6}ನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "돌네니 시"), ("lt", "Dolnenio savivaldybė"), ("lv", "Dolnenu pašvaldība"), ("mk", "Општина Долнени"), ("mr", "डॉलन\u{947}नि म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Dolneni"), ("nb", "Dolneni Kommune"), ("nl", "Dolneni"), ("no", "Dolneni Kommune"), ("pl", "Powiat Dolneni"), ("pt", "Município de Dolneni"), ("ru", "Долнени"), ("si", "දොල\u{dca}නෙන\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Dollnenit"), ("sr", "Општина Долнени"), ("sr_Latn", "Opština Dolneni"), ("sv", "Dolneni"), ("ta", "ட\u{bbe}னெனி நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}ల\u{c4d}న\u{c47}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบอลดอลเนน\u{e35}\u{e48}"), ("tr", "Dolneni Belediyesi"), ("uk", "Долнени"), ("ur", "دولنینی بلدیہ"), ("vi", "Đô thị tự trị Dolneni"), ("zh", "多尔涅尼区")]),
                        unofficial_name_list: ["Dolneni"].to_vec(),
                    }
                ),
                (
                    "504",
                    Subdivision{
                        name: "Krivogaštani",
                        country_alpha2: Alpha2::MK,
                        code: "504",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.30823059999999), longitude: Some(21.3679689), max_latitude: Some(41.3876829), min_latitude: Some(41.245415), max_longitude: Some(21.428434), min_longitude: Some(21.3069439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كريفوغاشتاني"), ("bg", "Кривогащани"), ("bn", "ক\u{9cd}রিভোগ\u{9be}স\u{9cd}ট\u{9be}নি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢\u{11128}𑄞\u{1112e}𑄉𑄌\u{11134}𑄑𑄚\u{11128}"), ("ceb", "Opština Krivogaštani"), ("da", "Krivogaštani Municipality"), ("de", "Opština Krivogaštani"), ("el", "Δήμος Κριβοκάστανης"), ("en", "Krivogaštani"), ("es", "Municipalidad de Krivogaštani"), ("fi", "Krivogaštanin kunta"), ("fr", "Krivogachtani"), ("gu", "ક\u{acd}રીવોગાસ\u{acd}તાની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रिवोगस\u{94d}तानी नगर पालिका"), ("hr", "Općina Krivogaštani"), ("id", "Kotamadya Krivogaštani"), ("it", "Krivogaštani"), ("ja", "クリヴォガシュタニ"), ("ka", "კრივოგაშტანის თემი"), ("kn", "ಕ\u{ccd}ರ\u{cbf}ಯೋವಾಸ\u{ccd}ಟಾನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "크리보가슈타니 시"), ("lt", "Krivogaštanio savivaldybė"), ("lv", "Krivogaštanu pašvaldība"), ("mk", "Општина Кривогаштани"), ("mr", "क\u{94d}रिओगस\u{94d}टानी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Krivogastani Municipality"), ("nb", "Krivogasjtani kommune"), ("nl", "Krivogaštani"), ("no", "Krivogasjtani kommune"), ("pl", "Gmina Kriwogasztani"), ("pt", "Município de Krivogachtani"), ("ru", "Кривогаштани"), ("si", "ක\u{dca}\u{200d}ර\u{dd2}වෝගස\u{dca}ත\u{dcf}න\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Krivogashtanit"), ("sr", "Општина Кривогаштани"), ("sr_Latn", "Opština Krivogaštani"), ("sv", "Krivogasjtani"), ("ta", "க\u{bcd}ரிவோகஸ\u{bcd}த\u{bbe}னி நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c3f}వ\u{c4b}గ\u{c3e}స\u{c4d}త\u{c3e}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคร\u{e34}โวกาสตาน\u{e35}"), ("tr", "Krivogastani Belediyesi"), ("uk", "Кривогаштани"), ("ur", "بلدیہ کریووگاشتانی"), ("vi", "Đô thị tự trị Krivogastani"), ("zh", "克里沃加什塔尼区")]),
                        unofficial_name_list: ["Krivogaštani"].to_vec(),
                    }
                ),
                (
                    "505",
                    Subdivision{
                        name: "Kruševo",
                        country_alpha2: Alpha2::MK,
                        code: "505",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.3801216), longitude: Some(21.2435597), max_latitude: Some(41.3801311), min_latitude: Some(41.3796977), max_longitude: Some(21.2442461), min_longitude: Some(21.2435457)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كروشيفو"), ("bg", "Крушево"), ("bn", "ক\u{9cd}র\u{9c1}সেভো পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢\u{1112a}𑄥𑄬𑄞\u{1112e}"), ("ceb", "Opština Kruševo"), ("da", "Kruševo Municipality"), ("de", "Opština Kruševo"), ("el", "Δήμος Κρουσόβου"), ("en", "Kruševo"), ("es", "Municipalidad de Kruševo"), ("fi", "Kruševon kunta"), ("fr", "Krouchevo"), ("gu", "ક\u{acd}ર\u{ac1}સ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}र\u{941}स\u{947}वो नगरपालिका"), ("hr", "Općina Kruševo"), ("hu", "Krusevói járás"), ("id", "Kotamadya Kruševo"), ("it", "Kruševo"), ("ja", "クルシェヴォ"), ("ka", "კრუშევოს თემი"), ("kn", "ಕ\u{ccd}ರುಸೇವೊ ಪುರಸಭ\u{cc6}"), ("ko", "크루셰보 시"), ("lt", "Kruševo savivaldybė"), ("lv", "Kruševo pašvaldība"), ("mk", "Општина Крушево"), ("mr", "किर\u{942}सोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Krusevo Municipality"), ("nb", "Kursevo Kommune"), ("nl", "Kruševo"), ("no", "Kursevo Kommune"), ("pl", "Gmina Kruševo"), ("pt", "Município de Krusevo"), ("ro", "Crușova"), ("ru", "Крушево"), ("si", "කෘසේවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Krushevës"), ("sr", "Општина Крушево"), ("sr_Latn", "Opština Kruševo"), ("sv", "Krusjevo"), ("ta", "க\u{bcd}ருஸேவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}రుస\u{c47}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ค\u{e39}เซวอ ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Krusevo Belediyesi"), ("uk", "Крушево"), ("ur", "بلدیہ کروشیوو"), ("vi", "Đô thị tự trị Krusevo"), ("zh", "克鲁舍沃区")]),
                        unofficial_name_list: ["Kruševo"].to_vec(),
                    }
                ),
                (
                    "506",
                    Subdivision{
                        name: "Mogila",
                        country_alpha2: Alpha2::MK,
                        code: "506",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.108056), longitude: Some(21.378333), max_latitude: Some(41.1192271), min_latitude: Some(41.1013039), max_longitude: Some(21.3925414), min_longitude: Some(21.3631968)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Могила"), ("ccp", "𑄟\u{1112e}𑄉\u{11128}𑄣"), ("ceb", "Mogila"), ("de", "Opština Mogila"), ("en", "Mogila"), ("es", "Municipalidad de Mogila"), ("fr", "Mogila"), ("hr", "Općina Mogila"), ("hu", "Mogila"), ("it", "Mogila"), ("ja", "モギラ"), ("ka", "მოგილის თემი"), ("ko", "모길라 시"), ("mk", "Општина Могила"), ("nl", "Mogila"), ("ru", "Могила"), ("sq", "Komuna e Mogillës"), ("sr", "Општина Могила"), ("sr_Latn", "Opština Mogila"), ("sv", "Mogila"), ("uk", "Могила (община)"), ("ur", "موگیلا بلدیہ"), ("zh", "莫吉拉區")]),
                        unofficial_name_list: ["Mogila"].to_vec(),
                    }
                ),
                (
                    "507",
                    Subdivision{
                        name: "Novaci",
                        country_alpha2: Alpha2::MK,
                        code: "507",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.18), longitude: Some(23.67), max_latitude: Some(45.2153314), min_latitude: Some(45.1469974), max_longitude: Some(23.6997415), min_longitude: Some(23.6471701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Новаци"), ("ccp", "𑄚\u{1112e}𑄞𑄥\u{11128}"), ("ceb", "Novaci (munisipyo)"), ("de", "Opština Novaci"), ("en", "Novaci"), ("es", "Municipalidad de Novaci"), ("fr", "Novatsi"), ("hr", "Općina Novaci"), ("it", "Novaci"), ("ja", "ノヴァツィ"), ("ka", "ნოვაცის თემი"), ("ko", "노바치 시"), ("mk", "Општина Новаци"), ("nl", "Novaci"), ("sq", "Komuna e Novacit"), ("sr", "Општина Новаци"), ("sr_Latn", "Opština Novaci"), ("sv", "Novaci"), ("uk", "Новаці"), ("ur", "نواتسی بلدیہ"), ("zh", "諾瓦濟區")]),
                        unofficial_name_list: ["Novaci"].to_vec(),
                    }
                ),
                (
                    "508",
                    Subdivision{
                        name: "Prilep",
                        country_alpha2: Alpha2::MK,
                        code: "508",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.344444), longitude: Some(21.552778), max_latitude: Some(41.36455429999999), min_latitude: Some(41.3232656), max_longitude: Some(21.5810538), min_longitude: Some(21.5153933)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريليب"), ("bg", "Прилеп (община)"), ("bn", "প\u{9cd}রিলেপ পৌরসভ\u{9be}"), ("ca", "Prilep"), ("ccp", "𑄛\u{11133}𑄢\u{11128}𑄣𑄬𑄛\u{11134}"), ("ceb", "Prilep (munisipyo)"), ("da", "Prilep Municipality"), ("de", "Opština Prilep"), ("el", "Πρίλεπ"), ("en", "Prilep"), ("es", "Municipalidad de Prilep"), ("fi", "Prilepin kunta"), ("fr", "Municipalité de Prilep"), ("gu", "પ\u{acd}રિલ\u{ac7}પ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}रिल\u{947}प नगर पालिका"), ("hr", "Općina Prilep"), ("hy", "Պրիլեպ մունիցիպիալիտետ"), ("id", "Kotamadya Prilep"), ("it", "Comune di Prilep"), ("ja", "プリレプ"), ("ka", "პრილეპის თემი"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ಲ\u{cc6}ಪ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "프릴레프 시"), ("lt", "Prilepo savivaldybė"), ("lv", "Prilepas pašvaldība"), ("mk", "Општина Прилеп"), ("mr", "प\u{94d}र\u{947}ल\u{945}प म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Prilep Municipality"), ("nb", "Prilep kommune"), ("nl", "Prilep"), ("no", "Prilep kommune"), ("pl", "Gmina Prilep"), ("pt", "Município de Prilep"), ("ru", "Прилеп"), ("si", "ප\u{dca}\u{200d}ර\u{dd2}ලේප\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Prilepit"), ("sr", "Општина Прилеп"), ("sr_Latn", "Opština Prilep"), ("sv", "Prilep"), ("ta", "ப\u{bcd}ரிலேப\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c3f}ల\u{c46}ప\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องปร\u{e35}เลป"), ("tr", "Pirlepe Belediyesi"), ("uk", "Прилеп (община)"), ("ur", "پریلپ بلدیہ"), ("vi", "Đô thị tự trị Prilep"), ("zh", "普里萊普區")]),
                        unofficial_name_list: ["Prilep"].to_vec(),
                    }
                ),
                (
                    "509",
                    Subdivision{
                        name: "Resen",
                        country_alpha2: Alpha2::MK,
                        code: "509",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.0889), longitude: Some(21.0122), max_latitude: Some(41.0996959), min_latitude: Some(41.0795937), max_longitude: Some(21.0292053), min_longitude: Some(20.9988428)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ريسن"), ("bg", "Ресен"), ("bn", "র\u{9be}সেন পৌরসভ\u{9be}"), ("ccp", "𑄢𑄬𑄥𑄬𑄚\u{11134}"), ("ceb", "Resen (munisipyo)"), ("da", "Resen Municipality"), ("de", "Opština Resen"), ("el", "Δήμος Ρέσνας"), ("en", "Resen"), ("es", "Municipalidad de Resen"), ("fa", "رسن"), ("fi", "Resen"), ("fr", "Resen"), ("gu", "ર\u{ac7}સન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "र\u{947}स\u{947}न नगरपालिका"), ("hr", "Općina Resen"), ("id", "Kotamadya Resen"), ("it", "Resen"), ("ja", "レセン"), ("ka", "რესენის თემი"), ("kn", "ರ\u{cc6}ಸ\u{cc6}ನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "레센 시"), ("lt", "Reseno savivaldybė"), ("lv", "Reseņas pašvaldība"), ("mk", "Општина Ресен"), ("mr", "रिस\u{947}\u{902} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Resen"), ("nb", "Resen kommune"), ("nl", "Resen"), ("no", "Resen kommune"), ("pl", "Gmina Resen"), ("pt", "Município de Resen"), ("ru", "Ресен"), ("si", "රෙසෙන\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Resnjës"), ("sr", "Општина Ресан"), ("sr_Latn", "Opština Resan"), ("sv", "Resen kommun"), ("ta", "ரேசன\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c46}స\u{c46}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเรเซน"), ("tr", "Resne Belediyesi"), ("uk", "Ресен"), ("ur", "ریسن بلدیہ"), ("vi", "Đô thị tự trị Resen"), ("zh", "里森区")]),
                        unofficial_name_list: ["Resen"].to_vec(),
                    }
                ),
                (
                    "601",
                    Subdivision{
                        name: "Bogovinje",
                        country_alpha2: Alpha2::MK,
                        code: "601",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9236371), longitude: Some(20.9163887), max_latitude: Some(41.9338751), min_latitude: Some(41.9166969), max_longitude: Some(20.9272813), min_longitude: Some(20.9023664)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوغوفينج"), ("bg", "Боговине"), ("bn", "বগোভিঞ\u{9cd}জে পৌরসভ\u{9be}"), ("ccp", "𑄝\u{1112e}𑄉\u{1112e}𑄞\u{11128}𑄚\u{11134}𑄎𑄬"), ("ceb", "Bogovinje"), ("da", "Bogovinje Municipality"), ("de", "Opština Bogovinje"), ("el", "Δήμος Μπογοβίντσε"), ("en", "Bogovinje"), ("es", "Municipalidad de Bogovinje"), ("fi", "Bogovinjen kunta"), ("fr", "Bogovinyé"), ("gu", "બોગોવિન\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोगोविन\u{947} नगरपालिका"), ("hr", "Općina Bogovinje"), ("id", "Kotamadya Bogovinje"), ("it", "Bogovinje"), ("ja", "ボゴヴィニェ"), ("ka", "ბოგოვინიეს თემი"), ("kn", "ಬೊಗೊವ\u{cbf}ನ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "보고비네 시"), ("lt", "Bogovinjės savivaldybė"), ("lv", "Bogovinjes pašvaldība"), ("mk", "Општина Боговиње"), ("mr", "बोगोवि\u{902}क\u{947}\u{902} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bogovinje Municipality"), ("nb", "Bogovinje kommune"), ("nl", "Bogovinje"), ("no", "Bogovinje kommune"), ("pl", "Gmina Bogowińe"), ("pt", "Município de Bogovinje"), ("ru", "Боговинье"), ("si", "බෝගොව\u{dd2}න\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Bogovinës"), ("sr", "Општина Боговиње"), ("sr_Latn", "Opština Bogovinje"), ("sv", "Bogovinje"), ("ta", "போகோவ\u{bcd}ய\u{bcd}ஞ\u{bcd}சே நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4b}గ\u{c4b}వ\u{c3f}ంజ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโบโกว\u{e34}นจ\u{e35}"), ("tr", "Bogonjinje Belediyesi"), ("uk", "Боговинє (община)"), ("ur", "بوگووینیہ بلدیہ"), ("vi", "Đô thị tự trị Bogovinje"), ("zh", "博戈维涅区")]),
                        unofficial_name_list: ["Bogovinje"].to_vec(),
                    }
                ),
                (
                    "602",
                    Subdivision{
                        name: "Brvenica",
                        country_alpha2: Alpha2::MK,
                        code: "602",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9672), longitude: Some(20.9808), max_latitude: Some(41.97606649999999), min_latitude: Some(41.9597614), max_longitude: Some(20.9886396), min_longitude: Some(20.9733618)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية برفينيتسا"), ("bg", "Бървеница"), ("bn", "ব\u{9cd}রভেনিক\u{9be} পৌরসভ\u{9be}"), ("ca", "Brvenica"), ("ccp", "𑄝\u{11133}𑄢𑄞𑄬𑄚\u{11128}𑄇"), ("ceb", "Brvenica"), ("da", "Brvenica Municipality"), ("de", "Opština Brvenica"), ("el", "Δήμος Μπρεβένιτσας"), ("en", "Brvenica"), ("es", "Municipalidad de Brvenica"), ("fi", "Brvenican kunta"), ("fr", "Brvenitsa"), ("gu", "બ\u{acd}રવ\u{ac7}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}रव\u{947}निका नगरपालिका"), ("hr", "Općina Brvenica"), ("id", "Kotamadya Brvenica"), ("it", "Brvenica"), ("ja", "ブルヴェニツァ"), ("ka", "ბრვენიცის თემი"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ವ\u{ccd}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "브르베니차 시"), ("lt", "Brvenicos savivaldybė"), ("lv", "Brvenicas pašvaldība"), ("mk", "Општина Брвеница"), ("mr", "ब\u{94d}रव\u{94d}ह\u{947}नी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Brvenica Municipality"), ("nb", "Brvenica Kommune"), ("nl", "Brvenica"), ("no", "Brvenica Kommune"), ("pl", "Gmina Brwenica"), ("pt", "Município de Brvenica"), ("ru", "Брвеница"), ("si", "බ\u{dca}\u{200d}රවෙන\u{dd2}ක\u{dcf} නගරසභ\u{dcf}ව"), ("sq", "Komuna e Bërvenicës"), ("sr", "Општина Брвеница"), ("sr_Latn", "Opština Brvenica"), ("sv", "Brvenica"), ("ta", "பிரவேனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ర\u{c46}వ\u{c46}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเบรน\u{e34}คา"), ("tr", "Brevnica Belediyesi"), ("uk", "Брвениця"), ("ur", "بروینیسا بلدیہ"), ("vi", "Đô thị tự trị Brvenica"), ("zh", "布尔韦尼察区")]),
                        unofficial_name_list: ["Brvenica"].to_vec(),
                    }
                ),
                (
                    "603",
                    Subdivision{
                        name: "Vrapčište",
                        country_alpha2: Alpha2::MK,
                        code: "603",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.83841779999999), longitude: Some(20.8858003), max_latitude: Some(41.8453661), min_latitude: Some(41.8293341), max_longitude: Some(20.890842), min_longitude: Some(20.8794765)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Врабчище"), ("ccp", "𑄞\u{11133}𑄢𑄛\u{11134}𑄥\u{11128}𑄌\u{11134}𑄑𑄬"), ("ceb", "Opština Vrapčište"), ("de", "Opština Vrapčište"), ("en", "Vrapčište"), ("es", "Municipalidad de Vrapčište"), ("fr", "Vraptchichté"), ("hr", "Općina Vrapčište"), ("it", "Vrapčište"), ("ja", "ヴラプチシュテ"), ("ka", "ვრაპჩიშტეს თემი"), ("ko", "브랍치슈테 시"), ("mk", "Општина Врапчиште"), ("nl", "Vrapčište"), ("ru", "Врапчиште"), ("sq", "Komuna e Vrapçishtit"), ("sr", "Општина Врапчиште"), ("sr_Latn", "Opština Vrapčište"), ("sv", "Vrapcisjte"), ("tr", "Vrapçişte Belediyesi"), ("uk", "Врапчиште"), ("ur", "بلدیہ وراپچیشتے"), ("zh", "弗拉普契什泰區")]),
                        unofficial_name_list: ["Vrapčište"].to_vec(),
                    }
                ),
                (
                    "604",
                    Subdivision{
                        name: "Gostivar",
                        country_alpha2: Alpha2::MK,
                        code: "604",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8), longitude: Some(20.916667), max_latitude: Some(41.81792859999999), min_latitude: Some(41.78423300000001), max_longitude: Some(20.9324742), min_longitude: Some(20.8848596)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غوستيفار"), ("bg", "Гостивар"), ("bn", "গোস\u{9cd}টিভ\u{9be}র পৌরসভ\u{9be}"), ("ccp", "𑄉\u{1112e}𑄌\u{11134}𑄑\u{11128}𑄞𑄢\u{11134}"), ("ceb", "Gostivar (munisipyo)"), ("da", "Gostivar Municipality"), ("de", "Opština Gostivar"), ("el", "Δήμος Γκόστιβαρ"), ("en", "Gostivar"), ("es", "Municipalidad de Gostivar"), ("fi", "Gostivarin kunta"), ("fr", "Municipalité de Gostivar"), ("gu", "ગોસ\u{acd}તિવર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "गोस\u{94d}टिवार नगर पालिका"), ("hr", "Općina Gostivar"), ("hu", "Gosztivari járás"), ("id", "Kotamadya Gostivar"), ("it", "Gostivar"), ("ja", "ゴスティヴァル"), ("ka", "გოსტივარის თემი"), ("kn", "ಗೋಸ\u{ccd}ಟ\u{cbf}ವಾ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "고스티바르 시"), ("lt", "Gostivaro savivaldybė"), ("lv", "Gostivaras pašvaldība"), ("mk", "Општина Гостивар"), ("mr", "गोस\u{94d}टीवर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gostivar Municipality"), ("nb", "Gostivar kommune"), ("nl", "Gostivar Municipality"), ("no", "Gostivar kommune"), ("pl", "Gmina Gostivar"), ("pt", "Município de Gostivar"), ("ro", "Gostivar"), ("ru", "Гостивар"), ("si", "ගොස\u{dca}ට\u{dd2}ව\u{dcf}ර\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Gostivarit"), ("sr", "Општина Гостивар"), ("sr_Latn", "Opština Gostivar"), ("sv", "Opsjtina Gostivar"), ("ta", "கோஸ\u{bcd}டிவ\u{bbe}ர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4b}స\u{c4d}ట\u{c3f}వ\u{c3e}ర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "กอสท\u{e34}วาร\u{e4c}"), ("tr", "Gostivar Belediyesi"), ("uk", "Гостивар (община)"), ("ur", "گوستیوار بلدیہ"), ("vi", "Đô thị tự trị Gostivar"), ("zh", "戈斯蒂瓦区")]),
                        unofficial_name_list: ["Gostivar"].to_vec(),
                    }
                ),
                (
                    "605",
                    Subdivision{
                        name: "Želino",
                        country_alpha2: Alpha2::MK,
                        code: "605",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.98034190000001), longitude: Some(21.0609438), max_latitude: Some(41.9876251), min_latitude: Some(41.9738654), max_longitude: Some(21.0698751), min_longitude: Some(21.0548639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تزيلينو"), ("bg", "Желино"), ("bn", "জেলিনো পৌরসভ\u{9be}"), ("ccp", "𑄎𑄬𑄣\u{11128}𑄚\u{1112e}"), ("ceb", "Opština Želino"), ("da", "Želino Municipality"), ("de", "Opština Želino"), ("el", "Δήμος Ζέλινου"), ("en", "Želino"), ("es", "Municipalidad de Želino"), ("fi", "Želinon kunta"), ("fr", "Jelino"), ("gu", "ઝ\u{ac7}લિનો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}लिनो नगरपालिका"), ("hr", "Općina Želino"), ("id", "Kotamadya Želino"), ("it", "Želino"), ("ja", "ジェリノ"), ("ka", "ჟელინოს თემი"), ("kn", "ಝ\u{cc6}ಲ\u{cbf}ನೋ ಪುರಸಭ\u{cc6}"), ("ko", "젤리노 시"), ("lt", "Želino savivaldybė"), ("lv", "Želino pašvaldība"), ("mk", "Општина Желино"), ("mr", "झ\u{947}लिनो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zelino Municipality"), ("nb", "Zelino kommune"), ("nl", "Želino"), ("no", "Zelino kommune"), ("pl", "Gmina Želino"), ("pt", "Município de Zelino"), ("ru", "Желино"), ("si", "සෙල\u{dd2}නෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Zhelinës"), ("sr", "Општина Желино"), ("sr_Latn", "Opština Želino"), ("sv", "Želino"), ("ta", "ஸிலினோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}ల\u{c3f}న\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซล\u{e34}โน"), ("tr", "Zelino Belediyesi"), ("uk", "Желино"), ("ur", "بلدیہ ژیلینو"), ("vi", "Đô thị tự trị Zelino"), ("zh", "熱利諾區")]),
                        unofficial_name_list: ["Želino"].to_vec(),
                    }
                ),
                (
                    "606",
                    Subdivision{
                        name: "Jegunovce",
                        country_alpha2: Alpha2::MK,
                        code: "606",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.07407200000001), longitude: Some(21.1220478), max_latitude: Some(42.0776731), min_latitude: Some(42.0686995), max_longitude: Some(21.128905), min_longitude: Some(21.1171566)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جيغونوفس"), ("bg", "Йегуновце"), ("bn", "জেগ\u{9c1}নোভচে পৌরসভ\u{9be}"), ("ccp", "𑄎𑄬𑄉\u{1112a}𑄚\u{1112e}𑄛\u{11134}"), ("ceb", "Jegunovce"), ("da", "Jegunovce Municipality"), ("de", "Opština Jegunovce"), ("el", "Δήμος Τζεγκούνοφτσε"), ("en", "Jegunovce"), ("es", "Municipalidad de Jegunovce"), ("fi", "Jegunovcen kunta"), ("fr", "Yégounovtsé"), ("gu", "જ\u{ac7}ગ\u{acd}ય\u{ac1}નોવ\u{ac7}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{947}ग\u{941}नोव\u{94d}स नगरपालिका"), ("hr", "Općina Jegunovce"), ("id", "Kotamadya Jegunovce"), ("it", "Jegunovce"), ("ja", "イェグノヴツェ"), ("ka", "იეგუნოვცეს თემი"), ("kn", "ಜ\u{cc6}ಗುನೋವ\u{cc6}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "예구노브체 시"), ("lt", "Jegunovcės savivaldybė"), ("lv", "Jegunovces municipalitāte"), ("mk", "Општина Јегуновце"), ("mr", "ज\u{947}ग\u{941}नोव\u{94d}हस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jegunovce Municipality"), ("nb", "Jegunovce kommune"), ("nl", "Jegunovce"), ("no", "Jegunovce kommune"), ("pl", "Gmina Jegunovce"), ("pt", "Município de Jegunovce"), ("ru", "Егуновце"), ("si", "ජෙගොනොව\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Jegunocit"), ("sr", "Општина Јегуновце"), ("sr_Latn", "Opština Jegunovce"), ("sv", "Jegunovce (kommun)"), ("ta", "ஜெகுனோவ\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}గన\u{c4b}వ\u{c4d}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเจก\u{e39}โนวเซ\u{e48}"), ("tr", "Jegunovce Belediyesi"), ("uk", "Єгуновце (община)"), ("ur", "یگونووتسہ بلدیہ"), ("vi", "Đô thị tự trị Jegunovce"), ("zh", "耶古诺夫策区")]),
                        unofficial_name_list: ["Jegunovce"].to_vec(),
                    }
                ),
                (
                    "607",
                    Subdivision{
                        name: "Mavrovo i Rostuša",
                        country_alpha2: Alpha2::MK,
                        code: "607",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.65), longitude: Some(20.733333), max_latitude: Some(41.6620718), min_latitude: Some(41.6477431), max_longitude: Some(20.7384775), min_longitude: Some(20.7295946)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مارفوفو و روستيشا"), ("bg", "Маврово и Ростуша"), ("bn", "ম\u{9be}ভ\u{9cd}রোভো অ\u{9cd}য\u{9be}ন\u{9cd}ড রোত\u{9c1}\u{981}স\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄟𑄛\u{11134}𑄢\u{1112e}𑄞\u{1112e} 𑄃\u{11133}𑄃 𑄢\u{1112e}𑄌\u{11134}𑄑\u{11128}𑄅\u{1112a}𑄥"), ("ceb", "Opština Mavrovo i Rostuša"), ("da", "Mavrovo and Rostuša Municipality"), ("de", "Opština Mavrovo und Rostuša"), ("el", "Δήμος Μαυρόβου και Ροστούσας"), ("en", "Mavrovo and Rostuša"), ("es", "Municipalidad de Mavrovo y Rostuša"), ("fi", "Mavrovo and Rostušan"), ("fr", "Mavrovo et Rostoucha"), ("gu", "માવરોવો એન\u{acd}ડ રોસ\u{acd}ત\u{ac1}સા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मावरोवो और रोस\u{94d}त\u{941}सा नगर पालिका"), ("hr", "Općina Mavrovo i Rostuša"), ("hu", "Mavrovo i Rosztusa"), ("id", "Kotamadya Mavrovo dan Rostuša"), ("it", "Mavrovo e Rostuša"), ("ja", "マヴロヴォ・ロストゥシャ"), ("ka", "მავროვო და როსტუშის თემი"), ("kn", "ಮಾವ\u{ccd}ರೋವೊ ಮತ\u{ccd}ತು ರೊಸ\u{ccd}ಟುಸಾ ಪುರಸಭ\u{cc6}"), ("ko", "마브로보 로스투샤 시"), ("lt", "Marvrovo ir Rostušos savivaldybė"), ("lv", "Mavrovo un Rostušas pašvaldība"), ("mk", "Општина Маврово и Ростуша"), ("mr", "मवरोवो अ\u{901}ड रोस\u{94d}त\u{941}सा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Mavrovo and Rostusa Municipality"), ("nb", "Mavrovo og Rostusa kommune"), ("nl", "Mavrovo en Rostuša"), ("no", "Mavrovo og Rostusa kommune"), ("pl", "Gmina Mawrowo-Rostusza"), ("pt", "Município de Mavroso e Rostusa"), ("ru", "Маврово и Ростуша"), ("si", "මව\u{dca}රෝවෝ සහ රෝස\u{dca}ට\u{dd4}ස\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Mavrovës dhe Radostushës"), ("sr", "Општина Маврово и Ростуша"), ("sr_Latn", "Opština Mavrovo i Rostuša"), ("sv", "Mavrovo i Rostusja"), ("ta", "மத\u{bcd}வரோவோ அண\u{bcd}ட\u{bcd} ரோஸ\u{bcd}டுச\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}వ\u{c4d}ర\u{c4b}వ\u{c4b} మర\u{c3f}యు ర\u{c4b}స\u{c4d}టూస\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องมาวโรโว แอน รอสต\u{e39}ซ\u{e48}า"), ("tr", "Mavrova ve Rostuşa Belediyesi"), ("uk", "Маврово і Ростуша"), ("ur", "بلدیہ ماوروو اور روستوشا"), ("vi", "Đô thị tự trị Mavrovo và Rostusa"), ("zh", "馬夫羅沃和羅斯圖沙區")]),
                        unofficial_name_list: ["Mavrovo i Rostuša"].to_vec(),
                    }
                ),
                (
                    "608",
                    Subdivision{
                        name: "Tearce",
                        country_alpha2: Alpha2::MK,
                        code: "608",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0777511), longitude: Some(21.0534923), max_latitude: Some(42.0861885), min_latitude: Some(42.0697506), max_longitude: Some(21.0603228), min_longitude: Some(21.0445913)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تيارس"), ("bg", "Теарце"), ("bn", "টিয\u{9bc}\u{9be}রচ পৌরসভ\u{9be}"), ("ccp", "𑄑\u{11128}𑄠𑄢\u{11134}𑄌\u{11134}"), ("ceb", "Tearce"), ("da", "Tearce Municipality"), ("de", "Opština Tearce"), ("el", "Δήμος Τέαρτσε"), ("en", "Tearce"), ("es", "Municipalidad de Tearce"), ("fi", "Tearcen kunta"), ("fr", "Téartsé"), ("gu", "ટીઅર\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "टियरस\u{947} नगर पालिका"), ("hr", "Općina Tearce"), ("id", "Kotamadya Tearce"), ("it", "Tearce"), ("ja", "テアルツェ"), ("ka", "ტეარცეს თემი"), ("kn", "ಟ\u{cbf}ಯರ\u{ccd}ಸ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "테아르체 시"), ("lt", "Tercės savivaldybė"), ("lv", "Tearces pašvaldība"), ("mk", "Општина Теарце"), ("mr", "टीअरस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Tearce Municipality"), ("nb", "Tearce Kommune"), ("nl", "Tearce"), ("no", "Tearce Kommune"), ("pl", "Gmina Tearce"), ("pt", "Município de Tearce"), ("ru", "Теарце"), ("si", "ට\u{dd2}යර\u{dca}ස\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Tearcës"), ("sr", "Општина Теарце"), ("sr_Latn", "Opština Tearce"), ("sv", "Tearce (kommun)"), ("ta", "டெர\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c3f}యర\u{c4d}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเท\u{e35}ยเซ\u{e48}"), ("tr", "Tearçe Belediyesi"), ("uk", "Теарце"), ("ur", "تیارتسے بلدیہ"), ("vi", "Đô thị tự trị Tearce"), ("zh", "特阿爾塞區")]),
                        unofficial_name_list: ["Tearce"].to_vec(),
                    }
                ),
                (
                    "609",
                    Subdivision{
                        name: "Tetovo",
                        country_alpha2: Alpha2::MK,
                        code: "609",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0), longitude: Some(20.966667), max_latitude: Some(42.0259693), min_latitude: Some(41.9892736), max_longitude: Some(20.9893371), min_longitude: Some(20.9485246)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تيتوفو"), ("bg", "Тетово (община)"), ("bn", "তেতোভো পৌরসভ\u{9be}"), ("ca", "Municipi de Tetovo"), ("ccp", "𑄑𑄬𑄑\u{1112e}𑄞\u{1112e}"), ("ceb", "Tetovo"), ("da", "Tetovo Municipality"), ("de", "Opština Tetovo"), ("el", "Δήμος Τετόβου"), ("en", "Tetovo"), ("es", "Municipalidad de Tetovo"), ("fi", "Tetovon kunta"), ("fr", "Municipalité de Tetovo"), ("gu", "ટિટોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ट\u{947}टवो नगरपालिका"), ("hr", "Općina Tetovo"), ("id", "Kotamadya Tetovo"), ("it", "Municipio di Tetovo"), ("ja", "テトヴォ"), ("ka", "ტეტოვოს თემი"), ("kn", "ಟ\u{cc6}ಟವೊ ಪುರಸಭ\u{cc6}"), ("ko", "테토보 시"), ("lt", "Tetovo savivaldybė"), ("lv", "Tetovo pašvaldība"), ("mk", "Општина Тетово"), ("mr", "ट\u{947}टवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Tetovo Municipality"), ("nb", "Tetovo kommune"), ("nl", "Tetovo"), ("no", "Tetovo kommune"), ("pl", "Gmina Tetowo"), ("pt", "Município Tetovo"), ("ru", "Тетово"), ("si", "ටෙටෝවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Tetovës"), ("sr", "Општина Тетово"), ("sr_Latn", "Opština Tetovo"), ("sv", "Tetovo"), ("ta", "டேட\u{bcd}ட\u{bbe}வோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c46}ట\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เตโทโว ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Kalkandelen Belediyesi"), ("uk", "Тетово (община)"), ("ur", "تیتوو بلدیہ"), ("vi", "Đô thị tự trị Tetovo"), ("zh", "特托沃區")]),
                        unofficial_name_list: ["Tetovo"].to_vec(),
                    }
                ),
                (
                    "701",
                    Subdivision{
                        name: "Kratovo",
                        country_alpha2: Alpha2::MK,
                        code: "701",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0800379), longitude: Some(22.1802799), max_latitude: Some(42.0853964), min_latitude: Some(42.0729499), max_longitude: Some(22.1919322), min_longitude: Some(22.1586944)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كراتوفو"), ("bg", "Кратово"), ("bn", "ক\u{9cd}র\u{9be}টোভো পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢𑄑\u{1112e}𑄞\u{1112e}"), ("ceb", "Kratovo Opština"), ("cs", "Opština Kratovo"), ("da", "Kratovo Municipality"), ("de", "Opština Kratovo"), ("el", "Δήμος Κρατόβου"), ("en", "Kratovo"), ("es", "Kratovo"), ("fi", "Kratovon kunta"), ("fr", "Kratovo"), ("gu", "ક\u{acd}ર\u{ac7}ટોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रतोवो नगरपालिका"), ("hr", "Općina Kratovo"), ("id", "Kotamadya Kratovo"), ("it", "Kratovo"), ("ja", "クラトヴォ"), ("ka", "კრატოვოს თემი"), ("kn", "ಕ\u{ccd}ರಾಟೊವೊ ಪುರಸಭ\u{cc6}"), ("ko", "크라토보 시"), ("lt", "Kratovo savivaldybė"), ("lv", "Kratovo pašvaldība"), ("mk", "Општина Кратово"), ("mr", "क\u{94d}रटोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kratovo Municipality"), ("nb", "Kratovo kommune"), ("nl", "Kratovo"), ("no", "Kratovo kommune"), ("pl", "Gmina Kratowo"), ("pt", "Kratovo"), ("ru", "Кратово"), ("si", "ක\u{dca}රටෝවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kratovës"), ("sr", "Општина Кратово"), ("sr_Latn", "Opština Kratovo"), ("sv", "Kratovo Opština"), ("ta", "க\u{bcd}ர\u{bbe}டோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c3e}ట\u{c3e}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องคราโตโว"), ("tr", "Kratova Belediyesi"), ("uk", "Кратово"), ("ur", "کارتوو بلدیہ"), ("vi", "Đô thị tự trị Kratovo"), ("zh", "克拉托沃區")]),
                        unofficial_name_list: ["Kratovo"].to_vec(),
                    }
                ),
                (
                    "702",
                    Subdivision{
                        name: "Kriva Palanka",
                        country_alpha2: Alpha2::MK,
                        code: "702",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.2058454), longitude: Some(22.3307965), max_latitude: Some(42.2233612), min_latitude: Some(42.1902456), max_longitude: Some(22.3575425), min_longitude: Some(22.3109686)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كريفا بالانكا"), ("bg", "Крива паланка"), ("bn", "ক\u{9cd}রিভ\u{9be} প\u{9be}ল\u{9be}ঙ\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢\u{11128}𑄞 𑄛𑄣𑄚\u{11134}𑄇"), ("ceb", "Kriva Palanka"), ("cs", "Opština Kriva Palanka"), ("da", "Kriva Palanka Municipality"), ("de", "Opština Kriva Palanka"), ("el", "Δήμος Κρίβα Παλάνκα"), ("en", "Kriva Palanka"), ("es", "Municipalidad de Kriva Palanka"), ("fi", "Kriva Palankan kunta"), ("fr", "Kriva Palanka"), ("gu", "ક\u{acd}રિવા પલા\u{a82}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रिवा पला\u{902}का नगरपालिका"), ("hr", "Općina Kriva Palanka"), ("id", "Kotamadya Kriva Palanka"), ("it", "Comune di Kriva Palanka"), ("ja", "クリヴァ・パランカ"), ("ka", "კრივა-პალანკის თემი"), ("kn", "ಕ\u{ccd}ರ\u{cbf}ವಾ ಪಲಂಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "크리바팔란카 시"), ("lt", "Kriva Palankos savivaldybė"), ("lv", "Kriva Palankas pašvaldība"), ("mk", "Општина Крива Паланка"), ("mr", "क\u{94d}रिवि पालनक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kriva Palanka Municipality"), ("nb", "Kriva Palanka kommune"), ("nl", "Kriva Palanka"), ("no", "Kriva Palanka kommune"), ("pl", "Gmina Kriwa Pałanka"), ("pt", "Município de Kriva Palanka"), ("ru", "Крива-Паланка"), ("si", "ක\u{dca}\u{200d}ර\u{dd2}ව\u{dcf} පලන\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kriva Pallankës"), ("sr", "Општина Крива Паланка"), ("sr_Latn", "Opština Kriva Palanka"), ("sv", "Kriva Palanka"), ("ta", "க\u{bcd}ர\u{bc0}வ\u{bbe} பலன\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c3f}వ\u{c3e} ప\u{c3e}లంక\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคร\u{e34}วา ปาลานค\u{e48}า"), ("tr", "Eğri Palanka Belediyesi"), ("uk", "Крива Паланка"), ("ur", "کریوا پالانکا بلدیہ"), ("vi", "Đô thị tự trị Kriva Palanka"), ("zh", "克里瓦帕蘭卡區")]),
                        unofficial_name_list: ["Kriva Palanka"].to_vec(),
                    }
                ),
                (
                    "703",
                    Subdivision{
                        name: "Kumanovo",
                        country_alpha2: Alpha2::MK,
                        code: "703",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.135833), longitude: Some(21.718056), max_latitude: Some(42.170783), min_latitude: Some(42.09290439999999), max_longitude: Some(21.7575538), min_longitude: Some(21.6693091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كومانوفو"), ("be", "абшчына Куманава"), ("bg", "Куманово"), ("bn", "ক\u{9c1}ম\u{9be}নোভো পৌরসভ\u{9be}"), ("ca", "municipi de Kumanovo"), ("ccp", "𑄇\u{1112a}𑄟\u{11134}𑄚\u{1112e}𑄞\u{1112e}"), ("ceb", "Kumanovo"), ("cs", "Opština Kumanovo"), ("da", "Kumanovo Municipality"), ("de", "Opština Kumanovo"), ("el", "Δήμος Κουμανόβου"), ("en", "Kumanovo"), ("es", "Municipalidad de Kumanovo"), ("fi", "Kumanovon kunta"), ("fr", "Municipalité de Koumanovo"), ("gu", "ક\u{ac1}મનોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{941}मानोवो नगरपालिका"), ("hr", "Općina Kumanovo"), ("id", "Kotamadya Kumanovo"), ("it", "Comune di Kumanovo"), ("ja", "クマノヴォ"), ("ka", "კუმანოვოს თემი"), ("kn", "ಕುಮಾನೊವೊ ಪುರಸಭ\u{cc6}"), ("ko", "쿠마노보 시"), ("lt", "Kumanovo savivaldybė"), ("lv", "Kumanovo pašvaldība"), ("mk", "Општина Куманово"), ("mr", "क\u{941}मनोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kumanovo Municipality"), ("nb", "Kumanovo kommune"), ("nl", "Kumanovo"), ("no", "Kumanovo kommune"), ("pl", "Gmina Kumanowo"), ("pt", "Município de Kumanovo"), ("ru", "Куманово"), ("si", "ක\u{dd4}මනොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Kumanovës"), ("sr", "Општина Куманово"), ("sr_Latn", "Opština Kumanovo"), ("sv", "Kumanovo (kommun)"), ("ta", "குமனோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "కుమన\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องค\u{e39}มาโนโว"), ("tr", "Kumanova Belediyesi"), ("uk", "Куманово"), ("ur", "کومانوو بلدیہ"), ("vi", "Đô thị tự trị Kumanovo"), ("zh", "库马诺沃区")]),
                        unofficial_name_list: ["Kumanovo"].to_vec(),
                    }
                ),
                (
                    "704",
                    Subdivision{
                        name: "Lipkovo",
                        country_alpha2: Alpha2::MK,
                        code: "704",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.1561352), longitude: Some(21.5870744), max_latitude: Some(42.1622105), min_latitude: Some(42.1479736), max_longitude: Some(21.5949798), min_longitude: Some(21.5788866)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ليبكوفو"), ("bg", "Липково"), ("bn", "লিপকোভ পৌরসভ\u{9be}"), ("ccp", "𑄣\u{11128}𑄛\u{11134}𑄇\u{1112e}𑄞\u{1112e}"), ("ceb", "Opština Lipkovo"), ("cs", "Opština Lipkovo"), ("da", "Lipkovo Municipality"), ("de", "Opština Lipkovo"), ("el", "Δήμος Λιπκόβου"), ("en", "Lipkovo"), ("es", "Municipalidad de Lipkovo"), ("fi", "Lipkovon kunta"), ("fr", "Lipkovo"), ("gu", "લિપકાવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लिपकोवो नगर पालिका"), ("hr", "Općina Lipkovo"), ("id", "Kotamadya Lipkovo"), ("it", "Lipkovo"), ("ja", "リプコヴォ"), ("ka", "ლიპკოვოს თემი"), ("kn", "ಲ\u{cbf}ಪ\u{ccd}ಕೋವೊ ಪುರಸಭ\u{cc6}"), ("ko", "립코보 시"), ("lt", "Lipkovo savivaldybė"), ("lv", "Lipkovo pašvaldība"), ("mk", "Општина Липково"), ("mr", "लिपकोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Lipkovo Municipality"), ("nb", "Lipkovo kommune"), ("nl", "Lipkovo"), ("no", "Lipkovo kommune"), ("pl", "Gmina Lipkovo"), ("pt", "Município de Lipkovo"), ("ru", "Община Липково"), ("si", "ල\u{dd2}ප\u{dca}කොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Likovës"), ("sr", "Општина Липково"), ("sr_Latn", "Opština Lipkovo"), ("sv", "Opsjtina Lipkovo"), ("ta", "லிப\u{bcd}கோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c3f}ప\u{c4d}క\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องล\u{e34}ปโคโว"), ("tr", "Lipkovo Belediyesi"), ("uk", "Липково"), ("ur", "لیپکوو بلدیہ"), ("vi", "Đô thị tự trị Lipkovo"), ("zh", "里普科沃區")]),
                        unofficial_name_list: ["Lipkovo"].to_vec(),
                    }
                ),
                (
                    "705",
                    Subdivision{
                        name: "Rankovce",
                        country_alpha2: Alpha2::MK,
                        code: "705",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.1695387), longitude: Some(22.116195), max_latitude: Some(42.17991079999999), min_latitude: Some(42.1639124), max_longitude: Some(22.1322583), min_longitude: Some(22.1079684)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية رانكوفيتسه"), ("bg", "Ранковце"), ("bn", "র\u{200d}\u{9cd}য\u{9be}ঙ\u{9cd}কোভচে পৌরসভ\u{9be}"), ("ccp", "𑄢𑄚\u{11134}𑄞\u{1112e}𑄛\u{11134}"), ("ceb", "Opština Rankovce"), ("cs", "Opština Rankovce"), ("da", "Rankovce Municipality"), ("de", "Opština Rankovce"), ("el", "Δήμος Ρανκόβιτσε"), ("en", "Rankovce"), ("es", "Municipalidad de Rankovce"), ("fi", "Rankovcen kunta"), ("fr", "Rankovtsé"), ("gu", "ર\u{ac7}ન\u{acd}કોવ\u{acd}સ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "र\u{948}नकोव\u{94d}स नगरपालिका"), ("hr", "Općina Rankovce"), ("hu", "Rankovce község"), ("id", "Kotamadya Rankovce"), ("it", "Rankovce"), ("ja", "ランコヴツェ"), ("ka", "რანკოვცეს თემი"), ("kn", "ರಾಂಕೋವ\u{ccd}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "란코브체 시"), ("lt", "Rankovcės savivaldybė"), ("lv", "Rankovces pašvaldība"), ("mk", "Општина Ранковце"), ("mr", "र\u{945}न\u{94d}कोव\u{94d}हस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Rankovce Municipality"), ("nb", "Rankovce kommune"), ("nl", "Rankovce"), ("no", "Rankovce kommune"), ("pl", "Gmina Rankovce"), ("pt", "Município de Rankovce"), ("ru", "Ранковце"), ("si", "රන\u{dca}කොව\u{dca}සේ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Rankocit"), ("sr", "Општина Ранковце"), ("sr_Latn", "Opština Rankovce"), ("sv", "Opsjtina Rankovce"), ("ta", "ரங\u{bcd}கோவ\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3e}ంక\u{c4b}వస\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดอาล\u{e34}ก\u{e31}นเต"), ("tr", "Rankovce Belediyesi"), ("uk", "Ранковце"), ("ur", "رانکووتسے بلدیہ"), ("vi", "Đô thị tự trị Rankovce"), ("zh", "蘭科夫采區")]),
                        unofficial_name_list: ["Rankovce"].to_vec(),
                    }
                ),
                (
                    "706",
                    Subdivision{
                        name: "Staro Nagoričane",
                        country_alpha2: Alpha2::MK,
                        code: "706",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.2), longitude: Some(21.833333), max_latitude: Some(42.2047782), min_latitude: Some(42.1955399), max_longitude: Some(21.8368701), min_longitude: Some(21.8216873)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Старо Нагоричане"), ("ccp", "𑄌\u{11133}𑄑𑄢\u{1112e} 𑄚𑄉\u{1112e}𑄢\u{11128}𑄇𑄬𑄚\u{11134}"), ("ceb", "Opština Staro Nagoricane"), ("cs", "Opština Staro Nagoričane"), ("de", "Opština Staro Nagoričane"), ("en", "Staro Nagoričane"), ("es", "Municipalidad de Staro Nagoričane"), ("fr", "Staro Nagoritchané"), ("it", "Staro Nagoričane"), ("ja", "スタロ・ナゴリチャネ"), ("ka", "სტარო-ნაგორიჩანეს თემი"), ("ko", "스타로나고리차네 시"), ("mk", "Општина Старо Нагоричане"), ("nl", "Staro Nagoričane"), ("ro", "Staro Nagoričane"), ("sq", "Komuna e Nagoriçit të Vjetër"), ("sr", "Општина Старо Нагоричано"), ("sr_Latn", "Opština Staro Nagoričano"), ("sv", "Staro Nagoricane"), ("uk", "Старо-Нагоричане (община)"), ("ur", "بلدیہ ستارو ناگوریچانے"), ("zh", "斯塔羅納戈里查內區")]),
                        unofficial_name_list: ["Staro Nagoričane"].to_vec(),
                    }
                ),
                (
                    "801",
                    Subdivision{
                        name: "Aerodrom †",
                        country_alpha2: Alpha2::MK,
                        code: "801",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.94643629999999), longitude: Some(21.4931713), max_latitude: Some(41.9776741), min_latitude: Some(41.918271), max_longitude: Some(21.554503), min_longitude: Some(21.437095)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄠𑄬𑄢\u{1112e}𑄓\u{11133}𑄢\u{1112e}𑄟\u{11134}"), ("en", "Aerodrom †")]),
                        unofficial_name_list: ["Aerodrom †"].to_vec(),
                    }
                ),
                (
                    "802",
                    Subdivision{
                        name: "Aračinovo",
                        country_alpha2: Alpha2::MK,
                        code: "802",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0247381), longitude: Some(21.5766407), max_latitude: Some(42.07069389999999), min_latitude: Some(41.9844389), max_longitude: Some(21.633435), min_longitude: Some(21.53742)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أراتسينوفو"), ("bg", "Арачиново"), ("bn", "আর\u{9be}কিনোভো পৌরসভ\u{9be}"), ("ca", "Aračinovo"), ("ccp", "𑄃𑄢𑄥𑄬𑄚\u{1112e}𑄞\u{1112e}"), ("ceb", "Opština Aračinovo"), ("da", "Aračinovo Municipality"), ("de", "Opština Aračinovo"), ("el", "Δήμος Αρατσίνοβο"), ("en", "Aračinovo"), ("es", "Municipalidad de Aračinovo"), ("fi", "Aračinovon kunta"), ("fr", "Aratchinovo"), ("gu", "અરાકિનોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "अराकीनोवो नगरपालिका"), ("hr", "Općina Aračinovo"), ("hu", "Aracsinovo"), ("id", "Kotamadya Aračinovo"), ("it", "Aračinovo"), ("ja", "アラチノヴォ"), ("ka", "არაჩინოვოს თემი"), ("kn", "ಅರಾಸ\u{cbf}ನೊವೊ ಪುರಸಭ\u{cc6}"), ("ko", "아라치노보 시"), ("lt", "Aračinovo savivaldybė"), ("lv", "Aračinovo pašvaldība"), ("mk", "Општина Арачиново"), ("mr", "अर\u{945}सिनोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Aracinovo Municipality"), ("nb", "Aracinovo Kommune"), ("nl", "Aračinovo"), ("no", "Aracinovo Kommune"), ("pl", "Gmina Araczinowo"), ("pt", "Município de Aracinovo"), ("ru", "Арачиново"), ("si", "අරස\u{dd2}නොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Haraçinës"), ("sr", "Општина Арачиново"), ("sr_Latn", "Opština Aračinovo"), ("sv", "Aračinovo (kommun i Makedonien)"), ("ta", "அரசினோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "అర\u{c3e}స\u{c3f}న\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอราซ\u{e34}โนโว"), ("tr", "Aracinovo Belediyesi"), ("uk", "Арачиново (община)"), ("ur", "بلدیہ آراچینوو"), ("vi", "Đô thị tự trị Aracinovo"), ("zh", "阿拉津诺沃区")]),
                        unofficial_name_list: ["Aracinovo"].to_vec(),
                    }
                ),
                (
                    "803",
                    Subdivision{
                        name: "Butel †",
                        country_alpha2: Alpha2::MK,
                        code: "803",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0895068), longitude: Some(21.463361), max_latitude: Some(42.1423902), min_latitude: Some(41.9966801), max_longitude: Some(21.511645), min_longitude: Some(21.376825)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄝\u{1112a}𑄑𑄬𑄣\u{11134}"), ("en", "Butel †")]),
                        unofficial_name_list: ["Butel †"].to_vec(),
                    }
                ),
                (
                    "804",
                    Subdivision{
                        name: "Gazi Baba †",
                        country_alpha2: Alpha2::MK,
                        code: "804",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0162961), longitude: Some(21.4991334), max_latitude: Some(42.1012791), min_latitude: Some(41.910186), max_longitude: Some(21.582676), min_longitude: Some(21.44421)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄉𑄎\u{11128} 𑄝𑄝"), ("en", "Gazi Baba †")]),
                        unofficial_name_list: ["Gazi Baba †"].to_vec(),
                    }
                ),
                (
                    "805",
                    Subdivision{
                        name: "Gjorče Petrov †",
                        country_alpha2: Alpha2::MK,
                        code: "805",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0606374), longitude: Some(21.3202736), max_latitude: Some(42.1126457), min_latitude: Some(41.9834329), max_longitude: Some(21.360891), min_longitude: Some(21.242506)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎\u{1112e}𑄢\u{11134} 𑄛𑄬𑄑\u{11133}𑄢\u{1112e}𑄛\u{11134}"), ("en", "Gjorče Petrov †")]),
                        unofficial_name_list: ["Gjorče Petrov †"].to_vec(),
                    }
                ),
                (
                    "806",
                    Subdivision{
                        name: "Zelenikovo",
                        country_alpha2: Alpha2::MK,
                        code: "806",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8733812), longitude: Some(21.602725), max_latitude: Some(41.8769431), min_latitude: Some(41.8673996), max_longitude: Some(21.6099306), min_longitude: Some(21.5935826)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية زيلينيكوفو"), ("bg", "Община Зелениково"), ("bn", "জেলেনিকোভো পৌরসভ\u{9be}"), ("ccp", "𑄎𑄬𑄣𑄬𑄚\u{11128}𑄇\u{1112e}𑄞\u{1112e}"), ("ceb", "Zelenikovo"), ("da", "Zelenikovo Municipality"), ("de", "Opština Zelenikovo"), ("el", "Δήμος Ζελενικόβου"), ("en", "Zelenikovo"), ("es", "Municipalidad de Zelenikovo"), ("fi", "Zelenikovon kunta"), ("fr", "Zelenikovo"), ("gu", "ઝ\u{ac7}લ\u{ac7}નિકોવો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}ल\u{947}निकोवो नगरपालिका"), ("hr", "Općina Zelenikovo"), ("hy", "Զելենիկովո"), ("id", "Kotamadya Zelenikovo"), ("it", "Zelenikovo"), ("ja", "ゼレニコヴォ"), ("ka", "ზელენიკოვოს თემი"), ("kn", "ಝ\u{cc6}ಲ\u{cc6}ನ\u{cbf}ಕೋವೊ ಪುರಸಭ\u{cc6}"), ("ko", "젤레니코보 시"), ("lt", "Zelenikovo savivaldybė"), ("lv", "Zelenikovo pašvaldība"), ("mk", "Општина Зелениково"), ("mr", "झ\u{947}ल\u{947}णीकोवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zelenikovo Municipality"), ("nb", "Zelenikovo kommune"), ("nl", "Zelenikovo"), ("no", "Zelenikovo kommune"), ("pl", "Gmina Zelenikovo"), ("pt", "Município de Zelenikovo"), ("ru", "Зелениково"), ("si", "සෙලෙන\u{dd2}කොවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Zelenikovës"), ("sr", "Општина Зелениково"), ("sr_Latn", "Opština Zelenikovo"), ("sv", "Zelenikovo (kommun)"), ("ta", "ஸிலேனிகோவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}ల\u{c46}న\u{c3f}క\u{c4b}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซเลน\u{e34}โตโว"), ("tr", "Zelenikovo Belediyesi"), ("uk", "Зелениково"), ("ur", "زیلینیکوو بلدیہ"), ("vi", "Đô thị tự trị Zelenikovo"), ("zh", "澤倫尼科沃區")]),
                        unofficial_name_list: ["Zelenikovo"].to_vec(),
                    }
                ),
                (
                    "807",
                    Subdivision{
                        name: "Ilinden",
                        country_alpha2: Alpha2::MK,
                        code: "807",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.98999999999999), longitude: Some(21.58), max_latitude: Some(42.0042037), min_latitude: Some(41.9878302), max_longitude: Some(21.5856242), min_longitude: Some(21.5397289)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية إليندن"), ("bg", "Белимбегово"), ("bn", "ইলিন\u{9cd}ডেন পৌরসভ\u{9be}"), ("ca", "Ilinden"), ("ccp", "𑄃\u{11128}𑄣\u{11128}𑄚\u{11134}𑄓𑄬𑄚\u{11134}"), ("ceb", "Ilinden"), ("da", "Ilinden Municipality"), ("de", "Opština Ilinden"), ("el", "Δήμος Ίλιντεν"), ("en", "Ilinden"), ("es", "Municipalidad de Ilinden"), ("fi", "Ilindenin kunta"), ("fr", "Ilinden"), ("gu", "ઇલિન\u{acd}ડ\u{ac7}ન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "इलिन\u{94d}ड\u{947}न नगर पालिका"), ("hr", "Općina Ilinden"), ("id", "Kotamadya Ilinden"), ("it", "Ilinden"), ("ja", "イリンデン"), ("ka", "ილინდენის თემი"), ("kn", "ಇಲ\u{cbf}ನಂಡ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "일린덴 시"), ("lt", "Ilindeno savivaldybė"), ("lv", "Ilindenas pašvaldība"), ("mk", "Општина Илинден"), ("mr", "इलिन\u{94d}ड\u{947}न म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ilinden Municipality"), ("nb", "Ilinden Kommune"), ("nl", "Ilinden"), ("no", "Ilinden Kommune"), ("pl", "Gmina Ilinden"), ("pt", "Município de Ilinden"), ("ru", "Илинден"), ("si", "ල\u{dd2}න\u{dca}ඩෙන\u{dca} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Belimbegut"), ("sr", "Општина Илинден"), ("sr_Latn", "Opština Ilinden"), ("sv", "Ilinden (kommun)"), ("ta", "லிண\u{bcd}டேன\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఇల\u{c3f}ండ\u{c46}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอ\u{e35}ล\u{e34}นเดน"), ("tr", "IIinden Belediyesi"), ("uk", "Ілинден (община)"), ("ur", "ایلندن بلدیہ"), ("vi", "Đô thị tự trị Ilinden"), ("zh", "伊林登區")]),
                        unofficial_name_list: ["Ilinden"].to_vec(),
                    }
                ),
                (
                    "808",
                    Subdivision{
                        name: "Karpoš †",
                        country_alpha2: Alpha2::MK,
                        code: "808",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.156389), longitude: Some(21.741111), max_latitude: Some(42.1677295), min_latitude: Some(42.1460644), max_longitude: Some(21.7596245), min_longitude: Some(21.727438)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇𑄢\u{11134}𑄛\u{1112e}𑄌\u{11134}"), ("en", "Karpoš †")]),
                        unofficial_name_list: ["Karpoš †"].to_vec(),
                    }
                ),
                (
                    "809",
                    Subdivision{
                        name: "Kisela Voda †",
                        country_alpha2: Alpha2::MK,
                        code: "809",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.92748), longitude: Some(21.4931713), max_latitude: Some(41.9678721), min_latitude: Some(41.8891651), max_longitude: Some(21.5344229), min_longitude: Some(21.425928)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{11128}𑄥𑄬𑄣 𑄣\u{1112e}𑄓"), ("en", "Kisela Voda †")]),
                        unofficial_name_list: ["Kisela Voda †"].to_vec(),
                    }
                ),
                (
                    "810",
                    Subdivision{
                        name: "Petrovec",
                        country_alpha2: Alpha2::MK,
                        code: "810",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.933333), longitude: Some(21.616667), max_latitude: Some(41.946245), min_latitude: Some(41.9278888), max_longitude: Some(21.6286469), min_longitude: Some(21.5950228)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Петровец"), ("ccp", "𑄛𑄬𑄑\u{11133}𑄢\u{1112e}𑄞𑄬𑄇\u{11134}"), ("ceb", "Petrovec"), ("de", "Opština Petrovec"), ("el", "Δήμος του Πέτροβετς"), ("en", "Petrovec"), ("es", "Municipalidad de Petrovec"), ("fr", "Petrovets"), ("hr", "Općina Petrovec"), ("it", "Petrovec"), ("ja", "ペトロヴェツ"), ("ka", "პეტროვეცის თემი"), ("ko", "페트로베츠 시"), ("mk", "Општина Петровец"), ("nl", "Petrovec"), ("sq", "Komuna e Petrovecit"), ("sr", "Општина Петровец"), ("sr_Latn", "Opština Petrovec"), ("sv", "Petrovec"), ("uk", "Петровець"), ("ur", "پیتروویتس بلدیہ"), ("zh", "彼得羅韋茨區")]),
                        unofficial_name_list: ["Petrovec"].to_vec(),
                    }
                ),
                (
                    "811",
                    Subdivision{
                        name: "Saraj †",
                        country_alpha2: Alpha2::MK,
                        code: "811",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.477778), longitude: Some(22.7025), max_latitude: Some(41.4809557), min_latitude: Some(41.4716933), max_longitude: Some(22.7108663), min_longitude: Some(22.6946086)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Saraj †")]),
                        unofficial_name_list: ["Saraj †"].to_vec(),
                    }
                ),
                (
                    "812",
                    Subdivision{
                        name: "Sopište",
                        country_alpha2: Alpha2::MK,
                        code: "812",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9500453), longitude: Some(21.4201362), max_latitude: Some(41.9634592), min_latitude: Some(41.9434839), max_longitude: Some(21.4390469), min_longitude: Some(21.4102507)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سوبيتشه"), ("bg", "Сопище"), ("bn", "সোপিসতে\u{981} পৌরসভ\u{9be}"), ("ccp", "𑄥\u{1112e}𑄛\u{11128}𑄌\u{11134}𑄑𑄬"), ("ceb", "Opština Sopište"), ("da", "Sopište Municipality"), ("de", "Opština Sopište"), ("el", "Δήμος Σόπιστας"), ("en", "Sopište"), ("es", "Municipalidad de Sopište"), ("fi", "Sopišten kunta"), ("fr", "Sopichté"), ("gu", "સોપિસ\u{acd}ત\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सौपिस\u{94d}त\u{947} नगरपालिका"), ("hr", "Općina Sopište"), ("id", "Kotamadya Sopište"), ("it", "Sopište"), ("ja", "ソピシュテ"), ("ka", "სოპიშტეს თემი"), ("kn", "ಸೋಪ\u{cbf}ಸ\u{ccd}ಟ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "소피슈테 시"), ("lt", "Sopištės savivaldybė"), ("lv", "Sopištes pašvaldība"), ("mk", "Општина Сопиште"), ("mr", "सोपिएस\u{94d}त म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sopiste Municipality"), ("nb", "Sopiste kommune"), ("nl", "Sopište"), ("no", "Sopiste kommune"), ("pl", "Gmina Sopište"), ("pt", "Município de Sopiste"), ("ru", "Сопиште"), ("si", "සොප\u{dd2}ස\u{dca}ටේ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Sopishtit"), ("sr", "Општина Сопиште"), ("sr_Latn", "Opština Sopište"), ("sv", "Sopisjte (kommun)"), ("ta", "சோபிஸ\u{bcd}ட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4b}ప\u{c3f}స\u{c4d}ట\u{c47} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโซพ\u{e34}ซเต"), ("tr", "Sopiste Belediyesi"), ("uk", "Сопиште"), ("ur", "بلدیہ سوپیشتے"), ("vi", "Đô thị tự trị Sopiste"), ("zh", "蘇皮什特區")]),
                        unofficial_name_list: ["Sopište"].to_vec(),
                    }
                ),
                (
                    "813",
                    Subdivision{
                        name: "Studeničani",
                        country_alpha2: Alpha2::MK,
                        code: "813",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9225639), longitude: Some(21.5363965), max_latitude: Some(41.9318797), min_latitude: Some(41.9109565), max_longitude: Some(21.5444898), min_longitude: Some(21.5259503)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ستودنيتشاني"), ("bg", "Студеничани"), ("bn", "স\u{9cd}ট\u{9c1}ডেনিক\u{9be}নি পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄑\u{1112a}\u{1112a}𑄓𑄬𑄚\u{11128}𑄇𑄚\u{11128}"), ("ceb", "Opština Studeničani"), ("da", "Studeničani Municipality"), ("de", "Opština Studeničani"), ("el", "Δήμος Στουντενίτσανης"), ("en", "Studeničani"), ("es", "Municipalidad de Studeničani"), ("fi", "Studeničanin kunta"), ("fr", "Stoudenitchani"), ("gu", "સ\u{acd}ટ\u{ac1}ડ\u{ac7}નીકાની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}ट\u{941}द\u{947}नीकानी नगर पालिका"), ("hr", "Općina Studeničani"), ("id", "Kotamadya Studeničani"), ("it", "Studeničani"), ("ja", "ストゥデニチャニ"), ("ka", "სტუდენიჩანის თემი"), ("kn", "ಸ\u{ccd}ಟಡೀನ\u{cbf}ಕಾನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "스투데니차니 시"), ("lt", "Studeničanio savivaldybė"), ("lv", "Studeničanu pašvaldība"), ("mk", "Општина Студеничани"), ("mr", "स\u{94d}ट\u{942}ड\u{947}\u{902}नीकानी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Studenicani Municipality"), ("nb", "Studenicani Kommune"), ("nl", "Studeničani"), ("no", "Studenicani Kommune"), ("pl", "Gmina Studeničani"), ("pt", "Município de Studenicani"), ("ru", "Студеничани"), ("si", "සට\u{dd4}දෙන\u{dd2}කණ\u{dd2} නගර සභ\u{dcf}ව"), ("sq", "Komuna e Studeniçanit"), ("sr", "Општина Студеничани"), ("sr_Latn", "Opština Studeničani"), ("sv", "Studenicani"), ("ta", "ஸ\u{bcd}டுடெனிசனி நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}టూడ\u{c46}న\u{c3f}కన\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสต\u{e34}วเดน\u{e34}คาน\u{e34}"), ("tr", "Studenicani Belediyesi"), ("uk", "Студеничани"), ("ur", "بلدیہ ستودینیچانی"), ("vi", "Đô thị tự trị Studenicani"), ("zh", "斯圖登尼查尼區")]),
                        unofficial_name_list: ["Studeničani"].to_vec(),
                    }
                ),
                (
                    "814",
                    Subdivision{
                        name: "Centar †",
                        country_alpha2: Alpha2::MK,
                        code: "814",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9953628), longitude: Some(21.4246078), max_latitude: Some(42.0143879), min_latitude: Some(41.9756039), max_longitude: Some(21.4509344), min_longitude: Some(21.4058088)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134}𑄑𑄢\u{11134}"), ("en", "Centar †")]),
                        unofficial_name_list: ["Centar †"].to_vec(),
                    }
                ),
                (
                    "815",
                    Subdivision{
                        name: "Čair †",
                        country_alpha2: Alpha2::MK,
                        code: "815",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9930355), longitude: Some(21.4365318), max_latitude: Some(41.998022), min_latitude: Some(41.979931), max_longitude: Some(21.4460089), min_longitude: Some(21.4271271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{11128}𑄠𑄢\u{11134}"), ("en", "Čair †")]),
                        unofficial_name_list: ["Čair †"].to_vec(),
                    }
                ),
                (
                    "816",
                    Subdivision{
                        name: "Čučer-Sandevo",
                        country_alpha2: Alpha2::MK,
                        code: "816",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0974773), longitude: Some(21.3877179), max_latitude: Some(42.1044702), min_latitude: Some(42.0934038), max_longitude: Some(21.3955476), min_longitude: Some(21.3779903)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تشوتشر سانديفو"), ("bg", "Чучер"), ("bn", "ক\u{9c1}চ\u{9be}র-স\u{9be}ন\u{9cd}দেভো পৌরসভ\u{9be}"), ("ccp", "𑄥\u{1112a}𑄥𑄬𑄢\u{11134}-𑄥𑄚\u{11134}𑄓𑄬𑄞\u{1112e}"), ("ceb", "Opština Čučer-Sandevo"), ("da", "Čučer-Sandevo Municipality"), ("de", "Opština Čučer-Sandevo"), ("el", "Δήμος Τσούτσερ Σάντσεβο"), ("en", "Čučer-Sandevo"), ("es", "Municipalidad de Čučer-Sandevo"), ("fi", "Čučer-Sandevon kunta"), ("fr", "Tchoutcher Sandevo"), ("gu", "ક\u{ac1}સ\u{ac7}ર-સ\u{ac7}નડ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{941}स\u{947}र-स\u{947}\u{902}द\u{947}वो नगरपालिका"), ("hr", "Općina Čučer-Sandevo"), ("id", "Kotamadya Čučer-Sandevo"), ("it", "Čučer-Sandevo"), ("ja", "チュチェル・サンデヴォ"), ("ka", "ჩუჩერ-სანდევოს თემი"), ("kn", "ಕ\u{ccd}ಯುಕರ\u{ccd}-ಸ\u{ccd}ಯಾಂಡ\u{cc6}ವೊ ಪುರಸಭ\u{cc6}"), ("ko", "추체르산데보 시"), ("lt", "Čučer-Sandevo savivaldybė"), ("lv", "Čučer-Sandevo pašvaldība"), ("mk", "Општина Чучер-Сандево"), ("mr", "च\u{941}च\u{947}र-स\u{945}नड\u{947}वो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cucer-Sandevo Municipality"), ("nb", "Cucer-Sandevo kommune"), ("nl", "Čučer-Sandevo"), ("no", "Cucer-Sandevo kommune"), ("pl", "Gmina Czuczer-Sandewo"), ("pt", "Município Cucer-Sandevo"), ("ro", "Čučer-Sandevo"), ("ru", "Чучер-Сандево"), ("si", "ක\u{dd4}සර\u{dca}-සැන\u{dca}ඩෙවෝ නගර සභ\u{dcf}ව"), ("sq", "Komuna e Çuçer-Sandevës"), ("sr", "Општина Чучер-Сандево"), ("sr_Latn", "Opština Čučer-Sandevo"), ("sv", "Čučer-Sandevo (kommun i Makedonien)"), ("ta", "கஸ\u{bcd}ர\u{bcd}-சண\u{bcd}டேவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}యూసర\u{c4d}-స\u{c3e}ండ\u{c47}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ซ\u{e39}เซอ-ซานเดโว ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Cucer-Sandevo Belediyesi"), ("uk", "Чучер-Сандево (община)"), ("ur", "بلدیہ چوچر-ساندوو"), ("vi", "Đô thị tự trị Cucer-Sandevo"), ("zh", "楚塞尔桑德沃区")]),
                        unofficial_name_list: ["Čučer-Sandevo"].to_vec(),
                    }
                ),
                (
                    "817",
                    Subdivision{
                        name: "Šuto Orizari †",
                        country_alpha2: Alpha2::MK,
                        code: "817",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0290416), longitude: Some(21.4097027), max_latitude: Some(42.0519889), min_latitude: Some(42.009311), max_longitude: Some(21.4261011), min_longitude: Some(21.388165)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{1112a}𑄑\u{1112e} 𑄃\u{11127}𑄢\u{11128}𑄎𑄢\u{11128}"), ("en", "Šuto Orizari †")]),
                        unofficial_name_list: ["Šuto Orizari †"].to_vec(),
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
#[cfg(feature = "mk")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MK,
        alpha3: Alpha3::MKD,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}} {{postalcode}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 389,
        currency_code: CurrencyCode::MKD,
        gec: Some(GEC::MK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::MKD),
        iso_long_name: "The Republic of North Macedonia",
        iso_short_name: "North Macedonia",
        official_language_list: ["mk"].to_vec(),
        spoken_language_list: ["mk"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "0",
        nationality: Some("Macedonian"),
        number: "807",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "MK",
        unofficial_name_list: [
            "Macedonia",
            "Mazedonien",
            "Macédoine",
            "F.Y.R.O.M (Macedonia)",
            "マケドニア旧ユーゴスラビア共和国",
            "Macedonië [FYROM]",
            "Macedonia (The Former Yugoslav Republic of)",
            "North Macedonia",
            "Macedonia (FYROM)",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "North Macedonia"),
            ("af", "North Macedonia"),
            ("ak", "North Macedonia"),
            ("am", "ሰሜን መቄዶኒያ"),
            ("an", "North Macedonia"),
            ("ar", "مقدونيا الشمالية"),
            ("as", "North Macedonia"),
            ("ay", "North Macedonia"),
            ("az", "North Macedonia"),
            ("ba", "North Macedonia"),
            ("be", "Паўночная Македонія"),
            ("bg", "North Macedonia"),
            ("bi", "North Macedonia"),
            ("bn", "উত\u{9cd}তর ম\u{9cd}য\u{9be}সেডোনিয়\u{9be}"),
            ("bn_IN", "উত\u{9cd}তর ম\u{9cd}য\u{9be}সেডোনিয়\u{9be}"),
            ("br", "North Macedonia"),
            ("bs", "North Macedonia"),
            ("ca", "North Macedonia"),
            ("ce", "North Macedonia"),
            ("ch", "North Macedonia"),
            ("cs", "Severní Makedonie"),
            ("cv", "North Macedonia"),
            ("cy", "Gogledd Macedonia"),
            ("da", "Nordmakedonien"),
            ("de", "Nordmazedonien"),
            ("dv", "North Macedonia"),
            ("dz", "North Macedonia"),
            ("ee", "North Macedonia"),
            ("el", "Βόρεια Μακεδονία"),
            ("en", "North Macedonia"),
            ("eo", "Nord-Makedonio"),
            ("es", "Macedonia del Norte"),
            ("et", "Põhja-Makedoonia"),
            ("eu", "Ipar Mazedonia"),
            ("fa", "مقدونیه شمالی"),
            ("ff", "North Macedonia"),
            ("fi", "North Macedonia"),
            ("fo", "North Macedonia"),
            ("fr", "Macédoine du Nord"),
            ("fy", "North Macedonia"),
            ("ga", "North Macedonia"),
            ("gl", "North Macedonia"),
            ("gn", "North Macedonia"),
            ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"),
            ("gv", "North Macedonia"),
            ("ha", "North Macedonia"),
            ("he", "צפון קלדוניה"),
            ("hi", "उत\u{94d}तर म\u{948}स\u{947}डोनिया"),
            ("hr", "Sjeverna Makedonija"),
            ("ht", "North Macedonia"),
            ("hu", "Észak-Macedónia"),
            ("hy", "North Macedonia"),
            ("ia", "Macedonia del Nord"),
            ("id", "Makedonia Utara"),
            ("io", "North Macedonia"),
            ("is", "Norður-Makedónía"),
            ("it", "Macedonia del Nord"),
            ("iu", "North Macedonia"),
            ("ja", "North Macedonia"),
            ("ka", "North Macedonia"),
            ("ki", "North Macedonia"),
            ("kk", "North Macedonia"),
            ("kl", "North Macedonia"),
            ("km", "North Macedonia"),
            ("kn", "North Macedonia"),
            ("ko", "북마케도니아"),
            ("ku", "Kaledonyaya Nû"),
            ("kv", "North Macedonia"),
            ("kw", "North Macedonia"),
            ("ky", "North Macedonia"),
            ("lo", "North Macedonia"),
            ("lt", "North Macedonia"),
            ("lv", "North Macedonia"),
            ("mi", "North Macedonia"),
            ("mk", "Северна Македонија"),
            ("ml", "North Macedonia"),
            ("mn", "North Macedonia"),
            ("mr", "उत\u{94d}तर म\u{945}स\u{947}डोनिया"),
            ("ms", "North Macedonia"),
            ("mt", "North Macedonia"),
            ("my", "North Macedonia"),
            ("na", "North Macedonia"),
            ("nb", "Nord-Makedonia"),
            ("ne", "North Macedonia"),
            ("nl", "Noord-Macedonië"),
            ("nn", "North Macedonia"),
            ("nv", "North Macedonia"),
            ("oc", "Macedònia del Nòrd"),
            (
                "or",
                "ନର\u{b4d}ଥ ମ\u{b4d}ୟ\u{b3e}ସ\u{b3f}ଡୋନ\u{b3f}ୟ\u{b3e}",
            ),
            ("pa", "ਨਾਰਥ ਕਾਲੀਡ\u{a4b}ਨੀਆ"),
            ("pi", "North Macedonia"),
            ("pl", "Macedonia Północna"),
            ("ps", "North Macedonia"),
            ("pt", "Macedónia do Norte"),
            ("pt_BR", "Macedônia do Norte"),
            ("ro", "North Macedonia"),
            ("ru", "Северная Македония"),
            ("rw", "North Macedonia"),
            ("sc", "Matzedònia de su Nord"),
            ("sd", "North Macedonia"),
            ("si", "North Macedonia"),
            ("sk", "North Macedonia"),
            ("sl", "North Macedonia"),
            ("so", "North Macedonia"),
            ("sq", "Maqedonia e Veriut"),
            ("sr", "Северна Македонија"),
            ("sv", "Nordmakedonien"),
            ("sw", "North Macedonia"),
            ("ta", "North Macedonia"),
            ("te", "North Macedonia"),
            ("tg", "Македонияи Шимолӣ"),
            ("th", "North Macedonia"),
            ("ti", "North Macedonia"),
            ("tk", "North Macedonia"),
            ("tl", "North Macedonia"),
            ("tr", "Kuzey Makedonya"),
            ("tt", "North Macedonia"),
            ("ug", "شىمالىي ماكېدونىيە"),
            ("uk", "Північна Македонія"),
            ("ur", "North Macedonia"),
            ("uz", "North Macedonia"),
            ("ve", "North Macedonia"),
            ("vi", "North Macedonia"),
            ("wa", "North Macedonia"),
            ("wo", "North Macedonia"),
            ("xh", "North Macedonia"),
            ("yo", "North Macedonia"),
            ("zh_CN", "北马其顿"),
            ("zh_HK", "北馬其頓"),
            ("zh_TW", "北馬其頓"),
            ("zu", "North Macedonia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
