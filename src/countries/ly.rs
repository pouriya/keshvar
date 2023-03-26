// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The State of Libya

#[cfg(all(feature = "ly", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::LY;
    pub const ALPHA3: Alpha3 = Alpha3::LBY;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 218;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::LYD;
    pub const GEC: Option<GEC> = Some(GEC::LY);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::LBA);
    pub const ISO_SHORT_NAME: &str = "Libya";
    pub const ISO_LONG_NAME: &str = "The State of Libya";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Libyan");
    pub const NUMBER: &str = "434";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAfrica);
    pub const UN_LOCODE: &str = "LY";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Libya",
        "ليبيا",
        "Libyen",
        "Libye",
        "Libia",
        "リビア",
        "Libië",
        "Libyan Arab Jamahiriya",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Libya"),
        ("af", "Libië"),
        ("ak", "Libya"),
        ("am", "ሊቢያ"),
        ("an", "Libya"),
        ("ar", "ليبيا"),
        ("as", "লিবিয়\u{9be}"),
        ("ay", "Libya"),
        ("az", "Libya"),
        ("ba", "Libya"),
        ("be", "Лівія"),
        ("bg", "Либия"),
        ("bi", "Libya"),
        ("bn", "লিবিয়\u{9be}"),
        ("bn_IN", "লিবিয়\u{9be}"),
        ("br", "Libia"),
        ("bs", "Libija"),
        ("ca", "Líbia"),
        ("ce", "Ливи"),
        ("ch", "Libya"),
        ("cs", "Libye"),
        ("cv", "Ливи"),
        ("cy", "Libya"),
        ("da", "Libyen"),
        ("de", "Libyen"),
        ("dv", "ލ\u{7a9}ބ\u{7a8}ޔ\u{7a7}"),
        ("dz", "ལ\u{f72}བ་ཡ་"),
        ("ee", "Libya"),
        ("el", "Λιβύη"),
        ("en", "Libya"),
        ("eo", "Libio"),
        ("es", "Libia"),
        ("et", "Liibüa"),
        ("eu", "Libia"),
        ("fa", "لیبی"),
        ("ff", "Libya"),
        ("fi", "Libya"),
        ("fo", "Libya"),
        ("fr", "Libye"),
        ("fy", "Lybje"),
        ("ga", "An Libia"),
        ("gl", "Libia"),
        ("gn", "Libya"),
        ("gu", "લિબયા"),
        ("gv", "Yn Leeb"),
        ("ha", "Libya"),
        ("he", "לוב"),
        ("hi", "लीबिया"),
        ("hr", "Libija"),
        ("ht", "Libi"),
        ("hu", "Líbia"),
        ("hy", "Լիբիա"),
        ("ia", "Libya"),
        ("id", "Libya"),
        ("io", "Libia"),
        ("is", "Líbýa"),
        ("it", "Libia"),
        ("iu", "Libya"),
        ("ja", "リビア"),
        ("ka", "Libya"),
        ("ki", "Libya"),
        ("kk", "Ливия"),
        ("kl", "Libya"),
        ("km", "ល\u{17b8}បេរ\u{17b8}យ\u{17c9}ា"),
        ("kn", "Libya"),
        ("ko", "리비아"),
        ("ku", "Lîbya"),
        ("kv", "Libya"),
        ("kw", "Libi"),
        ("ky", "Ливия"),
        ("lo", "Libya"),
        ("lt", "Libija"),
        ("lv", "Lībija"),
        ("mi", "Libya"),
        ("mk", "Либија"),
        ("ml", "Libya"),
        ("mn", "Ливи"),
        ("mr", "लिबया"),
        ("ms", "Libya"),
        ("mt", "Libja"),
        ("my", "လစ\u{103a}ဗျားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Libya"),
        ("nb", "Libya"),
        ("ne", "लिब\u{947}या"),
        ("nl", "Libië"),
        ("nn", "Libya"),
        ("nv", "Libya"),
        ("oc", "Libia"),
        ("or", "ଲୀବ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਲੀਬੀਆ"),
        ("pi", "लिबिया"),
        ("pl", "Libia"),
        ("ps", "Libya"),
        ("pt", "Líbia"),
        ("pt_BR", "Líbia"),
        ("ro", "Libia"),
        ("ru", "Ливия"),
        ("rw", "Libiya"),
        ("sc", "Lìbia"),
        ("sd", "لبيا"),
        ("si", "ල\u{dd2}බ\u{dd2}ය\u{dcf}ව"),
        ("sk", "Líbya"),
        ("sl", "Libija"),
        ("so", "Libiya"),
        ("sq", "Libi"),
        ("sr", "Либија"),
        ("sv", "Libyen"),
        ("sw", "Libya"),
        ("ta", "Libya"),
        ("te", "ల\u{c3f}బయ\u{c3e}"),
        ("tg", "Либия"),
        ("th", "ล\u{e34}เบ\u{e35}ย"),
        ("ti", "ሊብያ"),
        ("tk", "Liwiýa"),
        ("tl", "Libya"),
        ("tr", "Libya"),
        ("tt", "Libya"),
        ("ug", "لىۋىيە"),
        ("uk", "Лівія"),
        ("ur", "لیبیا"),
        ("uz", "Liviya"),
        ("ve", "Libya"),
        ("vi", "Li-bi"),
        ("wa", "Libeye"),
        ("wo", "Libi"),
        ("xh", "Libya"),
        ("yo", "Líbyà"),
        ("zh_CN", "利比亚"),
        ("zh_HK", "利比亞"),
        ("zh_TW", "利比亞"),
        ("zu", "ILibiya"),
    ];
    #[cfg(all(feature = "ly", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 26.3351;
        pub const LONGITUDE: f64 = 17.228331;
        pub const MAX_LATITUDE: f64 = 33.2203;
        pub const MAX_LONGITUDE: f64 = 25.2686;
        pub const MIN_LATITUDE: f64 = 19.5;
        pub const MIN_LONGITUDE: f64 = 9.391466;
        pub const NORTHEAST_LATITUDE: f64 = 33.2203;
        pub const NORTHEAST_LONGITUDE: f64 = 25.2686;
        pub const SOUTHWEST_LATITUDE: f64 = 19.5;
        pub const SOUTHWEST_LONGITUDE: f64 = 9.391466;
    }
}
#[cfg(all(feature = "ly", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 26.3351,
            longitude: 17.228331,
            max_latitude: 33.2203,
            max_longitude: 25.2686,
            min_latitude: 19.5,
            min_longitude: 9.391466,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 33.2203,
                    longitude: 25.2686,
                },
                southwest: CountryGeoBound {
                    latitude: 19.5,
                    longitude: 9.391466,
                },
            },
        }
    }
}

#[cfg(all(feature = "ly", feature = "subdivisions"))]
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
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::LY,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.116667), longitude: Some(20.066667), max_latitude: Some(32.2094962), min_latitude: Some(31.9760544), max_longitude: Some(20.2884299), min_longitude: Some(19.9999527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Benghazi"), ("ar", "بنغازي"), ("az", "Benqazi"), ("be", "Горад Бенгазі"), ("bg", "Бенгази"), ("bn", "বেনগ\u{9be}জি"), ("bs", "Bengazi"), ("ca", "Bengasi"), ("ccp", "𑄝𑄬𑄚\u{11134}𑄊𑄎\u{11128}"), ("ceb", "Sha‘bīyat Banghāzī"), ("cs", "Benghází"), ("cy", "Benghazi"), ("da", "Benghazi"), ("de", "Bengasi"), ("el", "Βεγγάζη"), ("en", "Benghazi"), ("es", "Bengasi"), ("et", "Banghāzī"), ("eu", "Bengazi"), ("fa", "بنغازی"), ("fi", "Bengasi"), ("fr", "Benghazi"), ("ga", "Benghazi"), ("gu", "બ\u{ac7}\u{a82}ઘાઝી"), ("he", "בנגאזי"), ("hi", "ब\u{947}\u{902}घाज\u{93c}ी"), ("hr", "Bengazi"), ("hu", "Bengázi"), ("hy", "Բենգազի"), ("id", "Benghazi"), ("is", "Benghazi"), ("it", "Bengasi"), ("ja", "ベンガジ"), ("ka", "ბენღაზი"), ("kk", "Бенгази"), ("kn", "ಬ\u{cc6}ಂಗಾಝ\u{cbf}"), ("ko", "벵가지"), ("lt", "Bengazis"), ("lv", "Bengāzī"), ("mr", "ब\u{947}नगाझी"), ("ms", "Benghazi"), ("nb", "Benghazi"), ("nl", "Benghazi"), ("no", "Benghazi"), ("pa", "ਬਨਗ\u{a3c}ਾਜ\u{a3c}ੀ"), ("pl", "Bengazi"), ("pt", "Bengasi"), ("ro", "Benghazi"), ("ru", "Бенгази"), ("si", "බෙන\u{dca}ග\u{dcf}ස\u{dd2}"), ("sk", "Bengázi"), ("sl", "Bengazi"), ("sq", "Bengazi"), ("sr", "Бенгази"), ("sr_Latn", "Bengazi"), ("sv", "Benghazi"), ("ta", "பங\u{bcd}க\u{bbe}சி"), ("te", "బ\u{c46}ంఘ\u{c3e}జ\u{c3f}"), ("th", "เบงกาซ\u{e35}"), ("tr", "Bingazi"), ("uk", "Бенгазі"), ("ur", "بنغازی"), ("uz", "Bingʻozi"), ("vi", "Benghazi"), ("yue", "班加西"), ("yue_Hans", "班加西"), ("zh", "班加西")]),
                        unofficial_name_list: ["Banghazi"].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::LY,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.7579854), longitude: Some(23.7632828), max_latitude: Some(32.2137814), min_latitude: Some(27.971698), max_longitude: Some(25.1495356), min_longitude: Some(23.0024879)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية البطنان"), ("be", "Эль-Бутнан"), ("bg", "Ал Бутнан"), ("bn", "ব\u{9c1}টন\u{9be}ন জেল\u{9be}"), ("ca", "Al Butnan"), ("ccp", "𑄝\u{1112a}𑄖\u{11134}𑄚𑄚\u{11134}"), ("ceb", "Sha‘bīyat al Buţnān"), ("da", "Al Butnan"), ("de", "Munizip al-Butnan"), ("el", "Μπουτνάν"), ("en", "Butnan"), ("es", "Al Butnan"), ("eu", "Al Butnan"), ("fa", "استان بطنان"), ("fi", "Butnan kaupunginosa"), ("fr", "Al Boutnan"), ("gu", "બ\u{ac1}ટ\u{acd}નન જિલ\u{acd}લો"), ("hi", "बटनान प\u{94d}रा\u{902}त"), ("hu", "Butnán tartomány"), ("hy", "Էլ-Բուտնան"), ("id", "Al Butnan"), ("it", "Municipalità di Butnan"), ("ja", "ブトナーン県"), ("ka", "ელ-ბუტნანი"), ("kn", "ಬಟ\u{ccd}ನಾನ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "알부트난 주"), ("lt", "Butnano savivaldybė"), ("lv", "Butnānas šebīja"), ("mr", "ब\u{941}ट\u{947}नन जिल\u{94d}हा"), ("ms", "Al Butnan"), ("nb", "Al Butnan"), ("nl", "Al Butnan"), ("no", "Al Butnan"), ("pl", "Al-Butnan"), ("pt", "Al Butnan"), ("ro", "Districtul Al Butnan"), ("ru", "Эль-Бутнан"), ("si", "බ\u{dd4}ට\u{dca}න\u{dcf}න\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Al Butnan"), ("ta", "பட\u{bcd}ன\u{bbe}ன\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బుట\u{c4d}నన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบ\u{e31}ตน\u{e31}น"), ("tr", "El Butnan ili"), ("uk", "Ель-Бутнан"), ("ur", "بطنان ضلع"), ("vi", "Quận Butnan"), ("zh", "布特南省")]),
                        unofficial_name_list: ["Al Butnan"].to_vec(),
                    }
                ),
                (
                    "DR",
                    Subdivision{
                        name: "DR",
                        country_alpha2: Alpha2::LY,
                        code: "DR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.766667), longitude: Some(22.633333), max_latitude: Some(32.7750619), min_latitude: Some(32.7425359), max_longitude: Some(22.6741272), min_longitude: Some(22.6118888)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية درنة"), ("bg", "Дарна"), ("ca", "Derna"), ("ccp", "𑄓𑄢\u{11134}𑄚"), ("ceb", "Darnah"), ("da", "Darnah"), ("de", "Munizip Darna"), ("en", "Derna"), ("es", "Derna"), ("eu", "Derna"), ("fa", "استان درنه"), ("fr", "Darnah"), ("hu", "Darna tartomány"), ("hy", "Դերնա"), ("it", "Municipalità di Derna"), ("ja", "デルナ県"), ("ka", "დარნის რაიონი"), ("ko", "다르나 주"), ("lt", "Darnos savivaldybė"), ("nb", "Darnah"), ("nl", "Derna"), ("no", "Darnah"), ("pl", "Darna"), ("pt", "Darnah"), ("ro", "Districtul Darnah"), ("ru", "Дерна"), ("sv", "Darnah"), ("uk", "Дерна"), ("ur", "درنہ ضلع"), ("zh", "德爾納省")]),
                        unofficial_name_list: ["Darnah"].to_vec(),
                    }
                ),
                (
                    "GT",
                    Subdivision{
                        name: "GT",
                        country_alpha2: Alpha2::LY,
                        code: "GT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.9640371), longitude: Some(10.1759285), max_latitude: Some(24.9905903), min_latitude: Some(24.9340191), max_longitude: Some(10.2097174), min_longitude: Some(10.148612)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية غات"), ("bn", "ঘ\u{9be}ট জেল\u{9be}"), ("ccp", "𑄊𑄖\u{11134}"), ("ceb", "Sha‘bīyat Ghāt"), ("da", "Ghat"), ("de", "Munizip Ghat"), ("el", "Γκατ"), ("en", "Ghat"), ("es", "Ghat"), ("eu", "Ghat"), ("fa", "استان غات"), ("fi", "Ghatin kaupunginosa"), ("fr", "Ghat"), ("gu", "ઘાટ જિલ\u{acd}લો"), ("hi", "घाट जिला"), ("hu", "Gát tartomány"), ("id", "Distrik Ghat"), ("it", "Municipalità di Ghat"), ("ja", "ガート県"), ("ka", "ღათის რაიონი"), ("kn", "ಘಾಟ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "가트 주"), ("lt", "Gato savivaldybė"), ("lv", "Gātas šebīja"), ("mr", "घाट जिल\u{94d}हा"), ("ms", "Ghat District"), ("nb", "Ghat"), ("nl", "Ghat"), ("no", "Ghat"), ("pl", "Ghat"), ("pt", "Ghat"), ("ro", "Districtul Ghat"), ("ru", "Гат"), ("si", "ඝට\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Ghat"), ("ta", "க\u{bbe}ட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఘ\u{c3e}ట\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "วาราซด\u{e34}น"), ("tr", "Ghat District"), ("uk", "Гат"), ("ur", "غات ضلع"), ("vi", "Quận Ghat"), ("zh", "加特省")]),
                        unofficial_name_list: ["Ghat"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::LY,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.4032332), longitude: Some(21.6660725), max_latitude: Some(32.9385909), min_latitude: Some(31.2525281), max_longitude: Some(22.0534001), min_longitude: Some(21.3707509)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الجبل الأخضر"), ("bn", "জ\u{9be}ব\u{9be}ল আল আকদ\u{9be}র"), ("ca", "Al Jabal al Akhdar"), ("ccp", "𑄎𑄝𑄣\u{11134} 𑄃𑄣\u{11134} 𑄃𑄇\u{11134}𑄓𑄢\u{11134}"), ("ceb", "Sha‘bīyat al Jabal al Akhḑar"), ("da", "Al Jabal al Akhdar"), ("de", "Munizip al-Dschabal al-Achdar"), ("el", "Τζαμπάλ αλ Ακντάρ"), ("en", "Jabal al Akhdar"), ("es", "Al Jabal al Akhdar"), ("eu", "Al Jabal al Akhdar"), ("fi", "Al Jabal al Akhdar"), ("fr", "Al Jabal al Akhdar"), ("gu", "જબલ અલ અખદર"), ("hi", "जबल अल अख\u{94d}दर"), ("hu", "Dzsabal el-Ahdar tartomány"), ("id", "Al Jabal al Akhdar"), ("it", "Municipalità di Gebel el-Achdar"), ("ja", "ジャバル・アル・アフダル県"), ("ka", "ელ-ჯებალ-ელ-ახდარი"), ("kn", "ಜಬಲ\u{ccd} ಅಲ\u{ccd} ಅಖ\u{ccd}ದಾರ\u{ccd}"), ("ko", "알자발알아크다르 주"), ("lt", "Džabal al Achdaro savivaldybė"), ("lv", "Džabala al Ahdara"), ("mr", "जबल अल अख\u{93c}\u{94d}दर\u{94d}र"), ("ms", "Jabal al Akhdar"), ("nb", "Al Jabal al Akhdar"), ("nl", "Al Jabal al Akhdar"), ("no", "Al Jabal al Akhdar"), ("pl", "Al-Dżabal al-Achdar"), ("pt", "Al Jabal al Akhdar"), ("ro", "Districtul Al Jabal al Akhdar"), ("ru", "Эль-Джебал-Эль-Ахдар"), ("si", "ජබ\u{dcf}ල\u{dca} අල\u{dca} අක\u{dca}ද\u{dcf}ර\u{dca}"), ("sv", "Al Jabal al Akhdar"), ("ta", "ஜபல\u{bcd} அல\u{bcd} அக\u{bcd}ஹடர\u{bcd}"), ("te", "జబల\u{c4d} అల\u{c4d} అఖదర\u{c4d}"), ("th", "จาเบล อ\u{e31}ล อ\u{e31}คห\u{e4c}ดา"), ("tr", "Jabak Al Akhdar"), ("uk", "Ель-Джебал Ель-Ахдар"), ("ur", "جبل الاخضر"), ("vi", "Jabal al Akhdar"), ("zh", "綠山省")]),
                        unofficial_name_list: ["Al Jabal al Akhḑar"].to_vec(),
                    }
                ),
                (
                    "JG",
                    Subdivision{
                        name: "JG",
                        country_alpha2: Alpha2::LY,
                        code: "JG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الجبل الغربي"), ("be", "Муніцыпалітэт Эль-Джабал-эль-Гарбі"), ("ca", "Al Jabal al Gharbi"), ("ccp", "𑄎𑄝𑄣\u{11134} 𑄃𑄣\u{11134} 𑄊𑄢\u{11134}𑄝\u{11128}"), ("ceb", "Sha‘bīyat al Jabal al Gharbī"), ("de", "Munizip al-Dschabal al-Gharbi"), ("en", "Jabal al Gharbi"), ("es", "Al Jabal al Gharbi"), ("fa", "استان جبل غربی"), ("fr", "Al Djabal al Gharbi"), ("hu", "Dzsabal el-Garbi tartomány"), ("id", "Distrik Al Jabal al Gharbi"), ("it", "Municipalità di Gebel Garbi"), ("ja", "ジャバル・アル・ガルビ県"), ("ka", "ელ-ჯაბალ-ელ-ღარბი"), ("ko", "알자발알가르비 주"), ("nl", "Al Jabal al Gharbi"), ("pl", "Al-Dżabal al-Gharbi"), ("pt", "Jabal Algarbi"), ("ru", "Эль-Джабал-эль-Гарби"), ("sv", "Al Jabal al Gharbi"), ("uk", "Ель-Джабал-ель-Ґарбі"), ("ur", "جبل الغربی ضلع"), ("zh", "西山省")]),
                        unofficial_name_list: ["Al Jabal al Gharbī"].to_vec(),
                    }
                ),
                (
                    "JI",
                    Subdivision{
                        name: "JI",
                        country_alpha2: Alpha2::LY,
                        code: "JI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.4525904), longitude: Some(12.9435536), max_latitude: Some(32.756192), min_latitude: Some(32.236122), max_longitude: Some(13.1125019), min_longitude: Some(12.678772)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الجفارة"), ("be", "Муніцыпалітэт Эль-Джыфара"), ("bn", "জ\u{9be}ফ\u{9be}র\u{9be} পৌরসভ\u{9be}"), ("ca", "Al Jfara"), ("ccp", "𑄎𑄜\u{11127}𑄢\u{11134}"), ("ceb", "Sha‘bīyat al Jafārah"), ("da", "Al Jfara"), ("de", "Munizip al-Dschifara"), ("el", "Τζαφάρα"), ("en", "Jafara"), ("es", "Al Jfara"), ("eu", "Al Jfara"), ("fi", "Jafaran kunta"), ("fr", "Al Djfara"), ("gu", "જાફરા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "जाफरा नगरपालिका"), ("hu", "Dzsifára tartomány"), ("id", "Kotamadya Jafara"), ("it", "Municipalità di Gefara"), ("ja", "ジファーラ県"), ("ka", "ელ-ჯიფარა"), ("kn", "ಜಫರಾ ಪುರಸಭ\u{cc6}"), ("ko", "알즈파라 주"), ("lt", "Džfaros savivaldybė"), ("lv", "Džifāra"), ("mr", "जाफरा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Jafara"), ("nb", "Al Jfara"), ("nl", "Al Jfara"), ("no", "Al Jfara"), ("pl", "Al-Dżifara"), ("pt", "Al Jifarah"), ("ro", "Districtul Al ‘Aziziyah"), ("ru", "Эль-Джифара"), ("si", "ජෆර\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Al Jfara"), ("ta", "ஜப\u{bbe}ர\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జఫ\u{c3e}ర\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศาบาลจาฟารา"), ("tr", "El Cfara ili"), ("uk", "Ель-Джифара"), ("ur", "جفارہ"), ("vi", "Đô thị tự trị Jafara"), ("zh", "吉法拉省")]),
                        unofficial_name_list: ["Al Jifarah"].to_vec(),
                    }
                ),
                (
                    "JU",
                    Subdivision{
                        name: "JU",
                        country_alpha2: Alpha2::LY,
                        code: "JU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.9835135), longitude: Some(16.912251), max_latitude: Some(29.8889149), min_latitude: Some(26.03828), max_longitude: Some(18.9787859), min_longitude: Some(14.263115)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الجفرة"), ("be", "Муніцыпалітэт Эль-Джуфра"), ("bn", "জ\u{9c1}ফ\u{9cd}র\u{9be}"), ("ca", "Al Jufrah"), ("ccp", "𑄎\u{1112a}𑄜\u{11133}𑄢"), ("ceb", "Al Jufrah (distrito)"), ("cs", "Al-Džufra"), ("da", "Al Jufrah"), ("de", "Munizip al-Dschufra"), ("el", "Τζούφρα"), ("en", "Jufra"), ("es", "Al Jufrah"), ("eu", "Al Jufrah"), ("fa", "استان جفره"), ("fi", "Jufra"), ("fr", "Al Djoufrah"), ("gu", "જ\u{ac1}ફ\u{acd}રા"), ("hi", "ज\u{941}फरा जिला"), ("hu", "Dzsufra tartomány"), ("id", "Jufra"), ("it", "Municipalità di Giofra"), ("ja", "ジュフラ県"), ("ka", "ელ-ჯუფრა"), ("kn", "ಜುಫ\u{ccd}ರಾ"), ("ko", "알주프라 주"), ("lt", "Džufros savivaldybė"), ("lv", "Džufras šebīja"), ("mr", "ज\u{941}फारा"), ("ms", "Jufra"), ("nb", "Al Jufrah"), ("nl", "Al Jufrah"), ("no", "Al Jufrah"), ("pl", "Al-Dżufra"), ("pt", "Al Jufrah"), ("ro", "Districtul Al Jufrah"), ("ru", "Эль-Джуфра"), ("si", "ජ\u{dd4}ෆ\u{dca}ර\u{dcf}"), ("sv", "Al Jufrah"), ("ta", "ஜூப\u{bcd}பிர"), ("te", "జుఫ\u{c4d}ర\u{c3e}"), ("th", "จ\u{e39}ฟรา"), ("tr", "El Cufra ili"), ("uk", "Ель-Джуфра"), ("ur", "جفرہ ضلع"), ("vi", "Jufra"), ("zh", "朱夫拉省")]),
                        unofficial_name_list: ["Al Jufrah", "Jofra"].to_vec(),
                    }
                ),
                (
                    "KF",
                    Subdivision{
                        name: "KF",
                        country_alpha2: Alpha2::LY,
                        code: "KF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.3112389), longitude: Some(21.8568586), max_latitude: Some(27.0215295), min_latitude: Some(19.5080431), max_longitude: Some(25.0), min_longitude: Some(18.9363769)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الكفرة"), ("be", "Муніцыпалітэт Эль-Куфра"), ("bn", "ক\u{9c1}ফ\u{9cd}র\u{9be} জেল\u{9be}"), ("ca", "Al Kufrah"), ("ccp", "𑄇\u{1112a}𑄜\u{11133}𑄢"), ("ceb", "Al Kufrah"), ("cs", "Al-Kufra"), ("da", "Al Kufrah"), ("de", "Munizip al-Kufra"), ("el", "Κούφρα"), ("en", "Kufra"), ("es", "Al Kufrah"), ("eu", "Al Kufrah"), ("fa", "استان کفره"), ("fi", "Kufran kaupunginosa"), ("fr", "Al-Koufrah"), ("gu", "ક\u{ac1}ફ\u{acd}રા જિલ\u{acd}લો"), ("hi", "क\u{941}फरा जिला"), ("hu", "Kufra tartomány"), ("id", "Al Kufrah"), ("it", "Municipalità di Cufra"), ("ja", "クフラ県"), ("ka", "ელ-კუფრა"), ("kn", "ಕುಫ\u{ccd}ರಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "알쿠프라 주"), ("lt", "Kufros savivaldybė"), ("lv", "Kufras šebīja"), ("mr", "क\u{941}फरा जिल\u{94d}हा"), ("ms", "Al Kufrah"), ("nb", "Al Kufrah"), ("nl", "Al Kufrah"), ("no", "Al Kufrah"), ("pl", "Al-Kufra"), ("pt", "Al Kufrah"), ("ro", "Districtul Al Kufrah"), ("ru", "Эль-Куфра"), ("si", "ක\u{dd4}ෆ\u{dca}ර\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Al Kufrah"), ("ta", "குப\u{bcd}பிர ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "కుఫ\u{c4d}ర\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e39}ฟรา"), ("tr", "El Kufra ili"), ("uk", "Ель-Куфра"), ("ur", "کفرہ ضلع"), ("vi", "Quận Kufra"), ("zh", "庫夫拉省")]),
                        unofficial_name_list: ["Al Kufrah", "Cufra", "Kofra", "Kufra"].to_vec(),
                    }
                ),
                (
                    "MB",
                    Subdivision{
                        name: "MB",
                        country_alpha2: Alpha2::LY,
                        code: "MB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.4599677), longitude: Some(14.1001326), max_latitude: Some(32.783028), min_latitude: Some(31.961752), max_longitude: Some(14.6691879), min_longitude: Some(13.1125019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية المرقب"), ("be", "Муніцыпалітэт Эль-Маргаб"), ("bn", "ম\u{9c1}রক\u{9be}ব পৌরসভ\u{9be}"), ("ca", "Al Murgub"), ("ccp", "𑄟\u{1112a}𑄢\u{11134}𑄇\u{1112a}𑄛\u{11134}"), ("ceb", "Al Marqab"), ("da", "Al Murgub"), ("de", "Munizip al-Murgub"), ("el", "Κοινότητα Μουρκούμπ"), ("en", "Murqub"), ("es", "Al Murgub"), ("eu", "Al Murgub"), ("fa", "استان مرقب"), ("fi", "Murqubin kunta"), ("fr", "Al Mourqoub"), ("gu", "મ\u{ac1}ર\u{acd}કબ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मरकब नगरपालिका"), ("hu", "Markab tartomány"), ("id", "Kotamadya Murqub"), ("it", "Municipalità di Margheb"), ("ja", "ムルクブ県"), ("ka", "ელ-მარღაბი"), ("kn", "ಮುರ\u{ccd}ಕ\u{ccd}ಬ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "알무르굽 주"), ("lt", "Murgubo savivaldybė"), ("lv", "Murgūbas šebīja"), ("mr", "म\u{941}क\u{94d}वब म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Murqub"), ("nb", "Al Murgub"), ("nl", "Al Murgub"), ("no", "Al Murgub"), ("pl", "Al-Marakib"), ("pt", "Al Marqab"), ("ro", "Districtul Al Murgub"), ("ru", "Эль-Маргаб"), ("si", "ම\u{dd4}ර\u{dca}ක\u{dd4}බ\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Al Murgub"), ("ta", "முற\u{bcd}குப\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ముర\u{c4d}ఖుబ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเมอร\u{e4c}กอบ"), ("tr", "Murqup Belediyesi"), ("uk", "Ель-Марґаб"), ("ur", "مرقب ضلع"), ("vi", "Đô thị tự trị Murqub"), ("zh", "迈尔盖卜省")]),
                        unofficial_name_list: ["Al Marqab", "Al Murqub", "al-Morqib"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "MI",
                        country_alpha2: Alpha2::LY,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.377533), longitude: Some(15.092017), max_latitude: Some(32.4306869), min_latitude: Some(32.1993655), max_longitude: Some(15.2752876), min_longitude: Some(14.9191761)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية مصراتة"), ("be", "Муніцыпалітэт Місурата"), ("bg", "Мисрата"), ("bn", "মেস\u{9be}র\u{9be}ত\u{9be} জেল\u{9be}"), ("ca", "Misurata"), ("ccp", "𑄟\u{11128}𑄥\u{11133}𑄢\u{11127}𑄖"), ("ceb", "Sha‘bīyat Mişrātah"), ("da", "Misrata District"), ("de", "Munizip Misrata"), ("el", "Μισράτα"), ("en", "Misrata"), ("es", "Misurata"), ("fa", "استان مصراته"), ("fi", "Misratan kaupunginosa"), ("fr", "Misratah"), ("gu", "મિસ\u{acd}રાતા જિલ\u{acd}લો"), ("hi", "मिस\u{94d}राटा जिला"), ("hu", "Miszráta tartomány"), ("id", "Distrik Misrata"), ("it", "distretto di Misurata"), ("ja", "ミスラタ県"), ("ka", "მისრათის რაიონი"), ("kn", "ಮ\u{cbf}ಸ\u{ccd}ರಾಟಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "미스라타 주"), ("lt", "Misratos savivaldybė"), ("lv", "Misrātas šebīja"), ("mr", "मिस\u{94d}ट\u{94d}र\u{947}ट जिल\u{94d}हा"), ("ms", "Daerah Misrata"), ("nb", "Misrata distrikt"), ("nl", "Misratah"), ("no", "Misrata distrikt"), ("pl", "Misrata"), ("pt", "Misurata"), ("ro", "Districtul Misratah"), ("ru", "Мисурата"), ("si", "ම\u{dd2}සරට\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Misratah"), ("ta", "மிஸ\u{bcd}ரட\u{bcd}ட\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3f}స\u{c4d}ర\u{c3e}ట\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตม\u{e34}สราทาห\u{e4c}"), ("tr", "Misrata District"), ("uk", "Місурата"), ("ur", "مصراتہ ضلع"), ("vi", "Quận Misrata"), ("zh", "米蘇拉塔省")]),
                        unofficial_name_list: ["Misratah", "Misurata", "Mişrātah"].to_vec(),
                    }
                ),
                (
                    "MJ",
                    Subdivision{
                        name: "MJ",
                        country_alpha2: Alpha2::LY,
                        code: "MJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.486667), longitude: Some(20.833889), max_latitude: Some(32.5197498), min_latitude: Some(32.4698104), max_longitude: Some(20.8616169), min_longitude: Some(20.7941151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية المرج"), ("bn", "ম\u{9be}র\u{9cd}জ জেল\u{9be}"), ("ca", "Al Marj"), ("ccp", "𑄟𑄢\u{11134}𑄎\u{11134}"), ("ceb", "Al Marj"), ("da", "Al Marj"), ("de", "Munizip al-Mardsch"), ("el", "Μάρτζ"), ("en", "Marj"), ("es", "Al Marj"), ("eu", "Al Marj"), ("fa", "استان مرج"), ("fi", "Marjin kaupunginosa"), ("fr", "Al Marj"), ("gu", "માર\u{acd}જ જિલ\u{acd}લો"), ("hi", "मारज जिला"), ("hu", "Mardzs tartomány"), ("id", "Distrik Marj"), ("it", "Municipalità di Barca"), ("ja", "マルジュ県"), ("ka", "ელ-მარჯის რაიონი"), ("kn", "ಮರ\u{ccd}ಜ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "알마르즈 주"), ("lt", "Mardžo savivaldybė"), ("lv", "Mardžas šebīja"), ("mr", "मर\u{94d}ज जिल\u{94d}हा"), ("ms", "Marj District"), ("nb", "Al Marj"), ("nl", "Al Marj"), ("no", "Al Marj"), ("pl", "Al-Mardż"), ("pt", "Al Marj"), ("ro", "Districtul Al Marj"), ("ru", "Эль-Мардж"), ("si", "ම\u{dcf}ර\u{dca}ජ\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Al Marj"), ("ta", "மெர\u{bcd}ஜ\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3e}ర\u{c4d}జ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตมาร\u{e4c}จ"), ("tr", "Marj District"), ("uk", "Ель-Мардж"), ("ur", "مرج ضلع"), ("vi", "Quận Marj"), ("zh", "邁爾季省")]),
                        unofficial_name_list: ["Al Marj", "Marj", "The Meadows"].to_vec(),
                    }
                ),
                (
                    "MQ",
                    Subdivision{
                        name: "MQ",
                        country_alpha2: Alpha2::LY,
                        code: "MQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.9182262), longitude: Some(13.9260001), max_latitude: Some(25.9427651), min_latitude: Some(25.8892733), max_longitude: Some(13.9626293), min_longitude: Some(13.8951154)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية مرزق"), ("be", "Муніцыпалітэт Марзук"), ("bn", "ম\u{9be}রজ\u{9c1}ক জেল\u{9be}"), ("ccp", "𑄟\u{1112a}𑄢\u{11134}𑄎\u{1112a}𑄇\u{11134}"), ("ceb", "Murzuq"), ("cs", "Murzuq"), ("da", "Marzuk"), ("de", "Munizip Murzuq"), ("el", "Μαρζούκ"), ("en", "Murzuq"), ("es", "Distrito de Murzuk"), ("eu", "Murzuq"), ("fa", "استان مرزق"), ("fi", "Murzuq District"), ("fr", "Mourzouq"), ("gu", "મ\u{ac1}ર\u{acd}ઝ\u{ac1}ક જિલ\u{acd}લો"), ("hi", "म\u{941}रज\u{93c}\u{942}क\u{93c} डिस\u{94d}ट\u{94d}रिक\u{94d}ट"), ("hu", "Murzuk tartomány"), ("hy", "Մարզուկ"), ("id", "Distrik Murzuq"), ("it", "Municipalità di Murzuch"), ("ja", "ムルズク県"), ("ka", "მარზუკის რაიონი"), ("kn", "ಮರ\u{ccd}ಜುಕ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "무르주크 주"), ("lt", "Murzuko savivaldybė"), ("lv", "Murzūkas šebīja"), ("mr", "म\u{941}र\u{94d}झ\u{941}क जिल\u{94d}हा"), ("ms", "Murzuq District"), ("nb", "Murzuq distrikt"), ("nl", "Murzuq"), ("no", "Murzuq distrikt"), ("pl", "Marzuk"), ("pt", "Murzuq"), ("ro", "Districtul Murzuq"), ("ru", "Марзук"), ("si", "මර\u{dca}සක\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Murzuq"), ("ta", "முற\u{bcd}ஸுக\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ముర\u{c4d}జుఖ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเมอซ\u{e38}ก"), ("tr", "Murzuq District"), ("uk", "Марзук"), ("ur", "مرزق ضلع"), ("vi", "Quận Murzuq"), ("zh", "邁爾祖格省")]),
                        unofficial_name_list: ["Murzuq"].to_vec(),
                    }
                ),
                (
                    "NL",
                    Subdivision{
                        name: "NL",
                        country_alpha2: Alpha2::LY,
                        code: "NL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.868333), longitude: Some(10.9825), max_latitude: Some(31.8966715), min_latitude: Some(31.8367114), max_longitude: Some(10.9945031), min_longitude: Some(10.9610395)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية نالوت"), ("be", "Муніцыпалітэт Налут"), ("ca", "Nalut"), ("ccp", "𑄚𑄣\u{1112a}𑄖\u{11134}"), ("ceb", "Sha‘bīyat Nālūt"), ("da", "Nalut"), ("de", "Nalut"), ("en", "Nalut"), ("es", "Nalut"), ("eu", "Nalut"), ("fa", "استان نالوت"), ("fr", "Nalout"), ("hu", "Nálút tartomány"), ("it", "Municipalità di Nalut"), ("ja", "ナールート県"), ("ka", "ნალუთის რაიონი"), ("ko", "날루트 주"), ("lt", "Naluto savivaldybė"), ("nb", "Nalut"), ("nl", "Nalut"), ("no", "Nalut"), ("pl", "Nalut"), ("pt", "Nalut"), ("ro", "Districtul Nalut"), ("ru", "Налут"), ("sv", "Nalut"), ("uk", "Налут"), ("ur", "نالوت ضلع"), ("zh", "納盧特省")]),
                        unofficial_name_list: ["Nālūt"].to_vec(),
                    }
                ),
                (
                    "NQ",
                    Subdivision{
                        name: "NQ",
                        country_alpha2: Alpha2::LY,
                        code: "NQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.6914909), longitude: Some(11.8891721), max_latitude: Some(33.1688603), min_latitude: Some(32.201754), max_longitude: Some(12.4340378), min_longitude: Some(11.3924129)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية النقاط الخمس"), ("bg", "Ан Нукат ал Хамс"), ("bn", "নোক\u{9be}ত আল খ\u{9be}মস"), ("ca", "An Nuqat al Khams"), ("ccp", "𑄚\u{1112a}𑄇𑄖\u{11134} 𑄃𑄣\u{11134} 𑄈𑄟\u{11134}𑄌\u{11134}"), ("ceb", "Sha‘bīyat an Nuqāţ al Khams"), ("da", "An Nuqat al Khams"), ("de", "Munizip an-Nuqat al-Chams"), ("el", "Νουγκάτ Αλ Κχαμς"), ("en", "Nuqat al Khams"), ("es", "An Nuqat al Khams"), ("et", "An-Nuqāt al-Khams"), ("eu", "An Nuqat al Khams"), ("fa", "استان نقاط الخمس"), ("fi", "Nuqat al Khams"), ("fr", "An Nouqat al Khams"), ("gu", "ન\u{ac1}કત અલ ખામ\u{acd}સ"), ("hi", "न\u{941}कत अल खाम\u{94d}स"), ("hu", "Nukát el-Hamsz tartomány"), ("id", "An Nuqat al Khams"), ("it", "Municipalità di Nuqat al Khams"), ("ja", "ヌカート・アル・ハムス県"), ("ka", "ენ-ნუგატ-ელ-ხუმსი"), ("kn", "ನುಖತ\u{ccd} ಅಲ\u{ccd} ಖಮ\u{ccd}ಸ\u{ccd}"), ("ko", "안누카트알캄스 주"), ("lt", "Nukat al Chamso savivaldybė"), ("lv", "Nukāta al Hamsa"), ("mr", "न\u{941}कात अल खाम\u{94d}स"), ("ms", "Nuqat al Khams"), ("nb", "An Nuqat al Khams"), ("nl", "An Nuqat al Khams"), ("no", "An Nuqat al Khams"), ("pl", "An-Nukat al-Chams"), ("pt", "An Nuqat al Khams"), ("ro", "Districtul An Nuqat al Khams"), ("ru", "Эн-Нугат-эль-Хумс"), ("si", "න\u{dd4}කට\u{dca} අල\u{dca} කම\u{dca}ස\u{dca}"), ("sv", "An Nuqat al Khams"), ("ta", "நுஃஆட\u{bcd} அல\u{bcd} கம\u{bcd}ஸ"), ("te", "న\u{c4c}ఖత\u{c4d} అల\u{c4d} ఖ\u{c3e}మ\u{c4d}స\u{c4d}"), ("th", "น\u{e39}ก\u{e31}ท อ\u{e31}ล คาม"), ("tr", "Nuqat Al Khams"), ("uk", "Ен-Нуґат ель-Хумс"), ("ur", "نقاط الخمس"), ("vi", "Nuqat al Khams"), ("zh", "努加特海姆斯省")]),
                        unofficial_name_list: ["An Nuqaţ al Khams", "Nuqāṭ al Ḫams"].to_vec(),
                    }
                ),
                (
                    "SB",
                    Subdivision{
                        name: "SB",
                        country_alpha2: Alpha2::LY,
                        code: "SB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.038889), longitude: Some(14.426389), max_latitude: Some(27.0898485), min_latitude: Some(26.9692017), max_longitude: Some(14.5361138), min_longitude: Some(14.3821335)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية سبها"), ("bg", "Сабха"), ("bn", "স\u{9be}ব\u{9be} জেল\u{9be}"), ("ccp", "𑄥𑄞"), ("ceb", "Sha‘bīyat Sabhā"), ("da", "Sabha District"), ("de", "Munizip Sabha"), ("el", "Σάμπχα"), ("en", "Sabha"), ("es", "Sabha"), ("eu", "Sabha"), ("fa", "استان سبها"), ("fi", "Sabhan kaupunginosa"), ("fr", "Sebha"), ("gu", "સભા જિલ\u{acd}લો"), ("hi", "सभा जिला"), ("hu", "Szabha tartomány"), ("id", "Distrik Sabha"), ("it", "Municipalità di Sebha"), ("ja", "サブハー県"), ("ka", "საბჰის რაიონი"), ("kn", "ಸಭಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "세바 주"), ("lt", "Sabhos savivaldybė"), ("lv", "Sebhas šebīja"), ("mr", "सभा जिल\u{94d}हा"), ("ms", "Sabha District"), ("nb", "Sabha"), ("nl", "Sabha"), ("no", "Sabha"), ("pl", "Sabha"), ("pt", "Sabha"), ("ro", "Districtul Sabha"), ("ru", "Сабха"), ("si", "සභ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Sabha"), ("ta", "சப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}భ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ซาบา"), ("tr", "Sabha District"), ("uk", "Сабха"), ("ur", "صبہہ ضلع"), ("vi", "Quận Sabha"), ("zh", "塞卜哈省")]),
                        unofficial_name_list: ["Sabhā"].to_vec(),
                    }
                ),
                (
                    "SR",
                    Subdivision{
                        name: "SR",
                        country_alpha2: Alpha2::LY,
                        code: "SR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.205314), longitude: Some(16.588936), max_latitude: Some(31.2135519), min_latitude: Some(31.156979), max_longitude: Some(16.6286411), min_longitude: Some(16.5155527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية سرت"), ("be", "Муніцыпалітэт Сурт"), ("ca", "Sirte"), ("ccp", "𑄥\u{11128}𑄢\u{11134}𑄑𑄬"), ("ceb", "Surt (distrito sa Libya)"), ("cs", "Surt"), ("da", "Surt"), ("de", "Munizip Surt"), ("en", "Sirte"), ("es", "Sirte"), ("fa", "استان سرت"), ("fr", "Syrte"), ("hu", "Szurt tartomány"), ("id", "Surt"), ("it", "Municipalità di Sirte"), ("ja", "スルト県"), ("ka", "სურთის რაიონი"), ("ko", "시르테 주"), ("lt", "Surto savivaldybė"), ("nb", "Surt"), ("nl", "Sirte"), ("no", "Surt"), ("pl", "Syrta"), ("pt", "Surt"), ("ro", "Districtul Surt"), ("ru", "Сирт"), ("sv", "Surt"), ("uk", "Сирт"), ("ur", "سرت ضلع"), ("zh", "蘇爾特省")]),
                        unofficial_name_list: ["Sirt", "Surt"].to_vec(),
                    }
                ),
                (
                    "TB",
                    Subdivision{
                        name: "TB",
                        country_alpha2: Alpha2::LY,
                        code: "TB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.8829369), longitude: Some(13.1883359), max_latitude: Some(32.9019297), min_latitude: Some(32.8639441), max_longitude: Some(13.2234836), min_longitude: Some(13.1531882)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية طرابلس"), ("be", "Муніцыпалітэт Тарабулус"), ("ca", "Trípoli"), ("ccp", "𑄑\u{11133}𑄢\u{11128}𑄛\u{11127}𑄣\u{11128}"), ("de", "Munizip Tripolis"), ("en", "Tripoli"), ("es", "Trípoli"), ("fr", "Tripoli"), ("hu", "Tripoli tartomány"), ("hy", "Տարաբուլուս"), ("it", "Municipalità di Tripoli"), ("ja", "トリポリ県"), ("ka", "ტრიპოლის რაიონი"), ("ko", "타라불루스 주"), ("lt", "Tripolio savivaldybė"), ("nl", "Tripoli"), ("pl", "Trypolis"), ("pt", "Trípoli"), ("ro", "Districtul Tarabulus"), ("ru", "Тарабулус"), ("sv", "Tarabulus"), ("tr", "Trablus ili"), ("uk", "Триполі"), ("ur", "طرابلس ضلع، لیبیا"), ("zh", "的黎波里省")]),
                        unofficial_name_list: ["Tripoli", "Tripoli", "Ţarābulus"].to_vec(),
                    }
                ),
                (
                    "WA",
                    Subdivision{
                        name: "WA",
                        country_alpha2: Alpha2::LY,
                        code: "WA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.3147011), longitude: Some(14.5032835), max_latitude: Some(32.3185818), min_latitude: Some(32.3120172), max_longitude: Some(14.5151013), min_longitude: Some(14.4937963)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الواحات"), ("be", "Муніцыпалітэт Эль-Вахат"), ("ca", "Al Wahat"), ("ccp", "𑄃𑄣\u{11134} 𑄤𑄦𑄖\u{11134}"), ("ceb", "Sha‘bīyat al Wāḩāt"), ("cs", "Al Wáhát"), ("da", "Al Wahat"), ("de", "Munizip al-Wahat"), ("en", "Al Wahat"), ("es", "Al Wahat"), ("eu", "Al Wahat"), ("fa", "استان واحات"), ("fr", "Al Wahat"), ("hu", "Váhát tartomány"), ("it", "Municipalità di el-Uahat"), ("ja", "アル・ワーハート県"), ("ka", "ელ-ვახატი"), ("ko", "알와하트 주"), ("lt", "Vahato savivaldybė"), ("nb", "Al Wahat"), ("nl", "Al Wahat"), ("no", "Al Wahat"), ("pl", "Al-Wahat"), ("pt", "Al Wahat"), ("ro", "Districtul Al Wahat"), ("ru", "Эль-Вахат"), ("sv", "Al Wahat"), ("uk", "Ель-Вахат"), ("ur", "الواحات ضلع"), ("zh", "绿洲省")]),
                        unofficial_name_list: ["Al Wahad", "Al Wahah", "Al Wahat", "The Oases"].to_vec(),
                    }
                ),
                (
                    "WD",
                    Subdivision{
                        name: "WD",
                        country_alpha2: Alpha2::LY,
                        code: "WD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.5240753), longitude: Some(31.2912463), max_latitude: Some(29.5447935), min_latitude: Some(29.5026438), max_longitude: Some(31.305119), min_longitude: Some(31.2627233)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية وادي الحياة"), ("bn", "ওয\u{9bc}\u{9be}দি আল হ\u{9be}য\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Wadi Al Hayaa"), ("ccp", "𑄤𑄓\u{11128} 𑄃𑄣\u{11134} 𑄦𑄠"), ("ceb", "Sha‘bīyat Wādī al Ḩayāt"), ("da", "Wadi Al Hayaa"), ("de", "Munizip Wadi al-Haya"), ("el", "Γουάντι αλ Χάγιαα"), ("en", "Wadi al Hayaa"), ("es", "Wadi Al Hayaa"), ("eu", "Wadi Al Hayaa"), ("fa", "استان وادی الحیاة"), ("fi", "Wadi al Hayaan kaupunginosa"), ("fr", "Wadi al Hayaat"), ("gu", "વાડી અલ હયા જિલ\u{acd}લો"), ("hi", "वादी अल हया जिला"), ("hu", "Vádi el-Haját tartomány"), ("id", "Distrik Wadi al Hayaa"), ("it", "Municipalità di Uadi el-Agial"), ("ja", "ワジ・アル・ハヤー県"), ("ka", "ვადი-ელ-ჰაიატი"), ("kn", "ವಾಡ\u{cbf} ಅಲ\u{ccd} ಹಯಾಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "와디알하야 주"), ("lt", "Vadi al Hajos savivaldybė"), ("lv", "Vādī el Hajātas šebīja"), ("mr", "वाडी अल हाया जिल\u{94d}हा"), ("ms", "Wadi al Hayaa District"), ("nb", "Wadi Al Hayaa"), ("nl", "Wadi Al Hayaa"), ("no", "Wadi Al Hayaa"), ("pl", "Wadi al-Hajat"), ("pt", "Wadi Al Hayat"), ("ro", "Districtul Wadi Al Hayaa"), ("ru", "Вади-эль-Хаят"), ("si", "වඩ\u{dd2} අල\u{dca} හය\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Wadi Al Hayaa"), ("ta", "வ\u{bbe}டி அல\u{bcd} ஹ\u{bbe}ய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "వ\u{c3e}డ\u{c3f}\u{c3f} అల\u{c4d} హయ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องวาด\u{e35} อ\u{e31}ล ฮายา"), ("tr", "Wadi Al Hayaa District"), ("uk", "Ваді-ель-Хаят"), ("ur", "وادی الحیاہ ضلع"), ("vi", "Quận Wadi al Hayaa"), ("zh", "瓦迪哈耶特省")]),
                        unofficial_name_list: ["Wādī al Ḩayāt"].to_vec(),
                    }
                ),
                (
                    "WS",
                    Subdivision{
                        name: "WS",
                        country_alpha2: Alpha2::LY,
                        code: "WS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية وادي الشاطئ"), ("bn", "ওয\u{9bc}\u{9be}দি আল স\u{9be}থী জেল\u{9be}"), ("ca", "Wadi Al Shatii"), ("ccp", "𑄤𑄓\u{11128} 𑄃𑄣\u{11134} 𑄥𑄑\u{11133}𑄦\u{11128}"), ("ceb", "Sha‘bīyat Wādī ash Shāţi’"), ("da", "Wadi Al Shatii"), ("de", "Munizip Wadi asch-Schati’"), ("el", "Γουάντι αλ Σατί"), ("en", "Wadi al Shatii"), ("es", "Wadi Al Shatii"), ("eu", "Wadi Al Shatii"), ("fi", "Wadi al Shatiin kaupunginosa"), ("fr", "Wadi ach Chatii"), ("gu", "વાડી અલ શતી જિલ\u{acd}લો"), ("hi", "वादी अल शातिल जिला"), ("hu", "Vádi es-Sáti tartomány"), ("hy", "Վադի ալ-Շատի"), ("id", "Distrik Wadi al Shatii"), ("it", "Municipalità di Uadi esc-Sciati"), ("ja", "ワジ・アル・シャーティー県"), ("ka", "ვადი-ეშ-შატი"), ("kn", "ವಾಡ\u{cbf} ಅಲ\u{ccd} ಶತ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "와디알샤티 주"), ("lt", "Vadi al Šati savivaldybė"), ("lv", "Vādī el Šati šebīja"), ("mr", "वाडी अल शाती जिल\u{94d}हा"), ("ms", "Wadi al Shatii District"), ("nb", "Wadi al Shatii distrikt"), ("nl", "Wadi Al Shatii"), ("no", "Wadi al Shatii distrikt"), ("pl", "Wadi asz-Szati"), ("pt", "Ash Shatii"), ("ro", "Districtul Wadi Al Shatii"), ("ru", "Вади-эш-Шати"), ("si", "වඩ\u{dd2} අල\u{dca} ශට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Wadi Al Shatii"), ("ta", "வ\u{bbe}டி அல\u{bcd} ச\u{bbe}தி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "వ\u{c3e}డ\u{c3f} ఎల\u{c4d} ష\u{c3e}ట\u{c40} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตวาด\u{e34} อ\u{e31}ล ชาท\u{e34}"), ("tr", "Vadi El Şati ili"), ("uk", "Ваді-еш-Шаті"), ("ur", "وادی الشاطی ضلع"), ("vi", "Tỉnh Wadi al Shatii"), ("zh", "沙提省")]),
                        unofficial_name_list: ["Wādī ash Shāţiʾ"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::LY,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.5394906), longitude: Some(12.5298028), max_latitude: Some(32.828812), min_latitude: Some(32.2049039), max_longitude: Some(12.9854179), min_longitude: Some(11.934911)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Popularate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شعبية الزاوية"), ("bn", "য\u{9be}ইয\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Zauiya"), ("ccp", "𑄎𑄃\u{11128}𑄠"), ("ceb", "Sha‘bīyat az Zāwiyah"), ("da", "Zawiya District"), ("de", "Munizip az-Zawiya"), ("el", "Ζαγία"), ("en", "Zawiya"), ("es", "Zauiya"), ("eu", "Zawiya"), ("fa", "استان زاویه"), ("fi", "Zawiyan kaupunginosa"), ("fr", "Az Zaouiyah"), ("gu", "ઝાવિયા જિલ\u{acd}લો"), ("hi", "ज\u{93c}ाविया जिला"), ("hu", "Závija tartomány"), ("id", "Distrik Zawiya"), ("it", "Municipalità di Zauia"), ("ja", "ザーウィヤ県"), ("ka", "ეზ-ზავიის რაიონი"), ("kn", "ಝಾವ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "앗자위야 주"), ("lt", "Zavijos savivaldybė"), ("lv", "Zāvijas šebīja"), ("mr", "झाविया जिल\u{94d}हा"), ("ms", "Zawiya District"), ("nb", "Az Zawiyah"), ("nl", "Az Zawiyah"), ("no", "Az Zawiyah"), ("pl", "Az-Zawija"), ("pt", "Az Zawiyah"), ("ro", "Districtul Az Zawiyah"), ("ru", "Эз-Завия"), ("si", "සව\u{dd2}ය\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Az Zawiyah"), ("ta", "ச\u{bbe}விய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "జ\u{c3e}వ\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ซาว\u{e34}ยา"), ("tr", "Zawaiya District"), ("uk", "Ез-Завія"), ("ur", "زاویہ ضلع"), ("vi", "Quận Zawiya"), ("zh", "扎維耶省")]),
                        unofficial_name_list: ["Az Zawiyah"].to_vec(),
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
#[cfg(feature = "ly")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::LY,
        alpha3: Alpha3::LBY,
        address_format: None,
        continent: Continent::Africa,
        country_code: 218,
        currency_code: CurrencyCode::LYD,
        gec: Some(GEC::LY),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::LBA),
        iso_long_name: "The State of Libya",
        iso_short_name: "Libya",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Libyan"),
        number: "434",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::NorthernAfrica),
        un_locode: "LY",
        unofficial_name_list: [
            "Libya",
            "ليبيا",
            "Libyen",
            "Libye",
            "Libia",
            "リビア",
            "Libië",
            "Libyan Arab Jamahiriya",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Libya"),
            ("af", "Libië"),
            ("ak", "Libya"),
            ("am", "ሊቢያ"),
            ("an", "Libya"),
            ("ar", "ليبيا"),
            ("as", "লিবিয়\u{9be}"),
            ("ay", "Libya"),
            ("az", "Libya"),
            ("ba", "Libya"),
            ("be", "Лівія"),
            ("bg", "Либия"),
            ("bi", "Libya"),
            ("bn", "লিবিয়\u{9be}"),
            ("bn_IN", "লিবিয়\u{9be}"),
            ("br", "Libia"),
            ("bs", "Libija"),
            ("ca", "Líbia"),
            ("ce", "Ливи"),
            ("ch", "Libya"),
            ("cs", "Libye"),
            ("cv", "Ливи"),
            ("cy", "Libya"),
            ("da", "Libyen"),
            ("de", "Libyen"),
            ("dv", "ލ\u{7a9}ބ\u{7a8}ޔ\u{7a7}"),
            ("dz", "ལ\u{f72}བ་ཡ་"),
            ("ee", "Libya"),
            ("el", "Λιβύη"),
            ("en", "Libya"),
            ("eo", "Libio"),
            ("es", "Libia"),
            ("et", "Liibüa"),
            ("eu", "Libia"),
            ("fa", "لیبی"),
            ("ff", "Libya"),
            ("fi", "Libya"),
            ("fo", "Libya"),
            ("fr", "Libye"),
            ("fy", "Lybje"),
            ("ga", "An Libia"),
            ("gl", "Libia"),
            ("gn", "Libya"),
            ("gu", "લિબયા"),
            ("gv", "Yn Leeb"),
            ("ha", "Libya"),
            ("he", "לוב"),
            ("hi", "लीबिया"),
            ("hr", "Libija"),
            ("ht", "Libi"),
            ("hu", "Líbia"),
            ("hy", "Լիբիա"),
            ("ia", "Libya"),
            ("id", "Libya"),
            ("io", "Libia"),
            ("is", "Líbýa"),
            ("it", "Libia"),
            ("iu", "Libya"),
            ("ja", "リビア"),
            ("ka", "Libya"),
            ("ki", "Libya"),
            ("kk", "Ливия"),
            ("kl", "Libya"),
            ("km", "ល\u{17b8}បេរ\u{17b8}យ\u{17c9}ា"),
            ("kn", "Libya"),
            ("ko", "리비아"),
            ("ku", "Lîbya"),
            ("kv", "Libya"),
            ("kw", "Libi"),
            ("ky", "Ливия"),
            ("lo", "Libya"),
            ("lt", "Libija"),
            ("lv", "Lībija"),
            ("mi", "Libya"),
            ("mk", "Либија"),
            ("ml", "Libya"),
            ("mn", "Ливи"),
            ("mr", "लिबया"),
            ("ms", "Libya"),
            ("mt", "Libja"),
            ("my", "လစ\u{103a}ဗျားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Libya"),
            ("nb", "Libya"),
            ("ne", "लिब\u{947}या"),
            ("nl", "Libië"),
            ("nn", "Libya"),
            ("nv", "Libya"),
            ("oc", "Libia"),
            ("or", "ଲୀବ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਲੀਬੀਆ"),
            ("pi", "लिबिया"),
            ("pl", "Libia"),
            ("ps", "Libya"),
            ("pt", "Líbia"),
            ("pt_BR", "Líbia"),
            ("ro", "Libia"),
            ("ru", "Ливия"),
            ("rw", "Libiya"),
            ("sc", "Lìbia"),
            ("sd", "لبيا"),
            ("si", "ල\u{dd2}බ\u{dd2}ය\u{dcf}ව"),
            ("sk", "Líbya"),
            ("sl", "Libija"),
            ("so", "Libiya"),
            ("sq", "Libi"),
            ("sr", "Либија"),
            ("sv", "Libyen"),
            ("sw", "Libya"),
            ("ta", "Libya"),
            ("te", "ల\u{c3f}బయ\u{c3e}"),
            ("tg", "Либия"),
            ("th", "ล\u{e34}เบ\u{e35}ย"),
            ("ti", "ሊብያ"),
            ("tk", "Liwiýa"),
            ("tl", "Libya"),
            ("tr", "Libya"),
            ("tt", "Libya"),
            ("ug", "لىۋىيە"),
            ("uk", "Лівія"),
            ("ur", "لیبیا"),
            ("uz", "Liviya"),
            ("ve", "Libya"),
            ("vi", "Li-bi"),
            ("wa", "Libeye"),
            ("wo", "Libi"),
            ("xh", "Libya"),
            ("yo", "Líbyà"),
            ("zh_CN", "利比亚"),
            ("zh_HK", "利比亞"),
            ("zh_TW", "利比亞"),
            ("zu", "ILibiya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
