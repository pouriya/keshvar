// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Tajikistan

#[cfg(all(feature = "tj", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::TJ;
    pub const ALPHA3: Alpha3 = Alpha3::TJK;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 992;
    pub const CURRENCY_CODE: &str = "TJS";
    pub const GEC: Option<GEC> = Some(GEC::TI);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<&str> = Some("TJK");
    pub const ISO_SHORT_NAME: &str = "Tajikistan";
    pub const ISO_LONG_NAME: &str = "The Republic of Tajikistan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ru", "tg"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ru", "tg"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Tadzhik");
    pub const NUMBER: &str = "762";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAsia);
    pub const UN_LOCODE: &str = "TJ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Tajikistan",
        "Tadschikistan",
        "Tayikistán",
        "タジキスタン",
        "Tadzjikistan",
        "Tajikstan",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Tajikistan"),
        ("af", "Tadjikistan"),
        ("ak", "Tajikistan"),
        ("am", "ታጃጡስታን"),
        ("an", "Tajikistan"),
        ("ar", "طاجيكستان"),
        ("as", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
        ("ay", "Tajikistan"),
        ("az", "Tacikistan"),
        ("ba", "Tajikistan"),
        ("be", "Таджыкістан"),
        ("bg", "Таджикистан"),
        ("bi", "Tajikistan"),
        ("bn", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
        ("br", "Tadjikistan"),
        ("bs", "Tadžikistan"),
        ("ca", "Tadjikistan"),
        ("ce", "Таджики"),
        ("ch", "Tajikistan"),
        ("cs", "Tádžikistán"),
        ("cv", "Таджики"),
        ("cy", "Tajikistan"),
        ("da", "Tadsjikistan"),
        ("de", "Tadschikistan"),
        ("dv", "ތ\u{7a6}ޖ\u{7a8}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
        ("dz", "ཏ་ཇ\u{f72}་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Tajikistan"),
        ("el", "Τατζικιστάν"),
        ("en", "Tajikistan"),
        ("eo", "Taĝikio"),
        ("es", "Tayikistán"),
        ("et", "Tadžikistan"),
        ("eu", "Tajikistan"),
        ("fa", "تاجیکستان"),
        ("ff", "Tajikistan"),
        ("fi", "Tadžikistan"),
        ("fo", "Tadsjikistan"),
        ("fr", "Tadjikistan"),
        ("fy", "Tadzjikistan"),
        ("ga", "An Táidsíceastáin"),
        ("gl", "Taxiquistán"),
        ("gn", "Tajikistan"),
        ("gu", "તાજીકિસ\u{acd}તાન"),
        ("gv", "Yn Tajikistaan"),
        ("ha", "Tajikistan"),
        ("he", "טג׳יקיסטן"),
        ("hi", "ताजिकिस\u{94d}तान"),
        ("hr", "Tadžikistan"),
        ("ht", "Tadjikistan"),
        ("hu", "Tádzsikisztán"),
        ("hy", "Տաճիկստան"),
        ("ia", "Tajikistan"),
        ("id", "Tajikistan"),
        ("io", "Tajikistan"),
        ("is", "Tadsíkistan"),
        ("it", "Tagikistan"),
        ("iu", "Tajikistan"),
        ("ja", "タジキスタン"),
        ("ka", "ტაჯიკეთი"),
        ("ki", "Tajikistan"),
        ("kk", "Тәжікстан"),
        ("kl", "Tajikistan"),
        (
            "km",
            "តាហ\u{17d2}ស\u{17ca}\u{17b8}គ\u{17b8}ស\u{17d2}តង\u{17cb}",
        ),
        ("kn", "ತಾಜ\u{cbf}ಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
        ("ko", "타지키스탄"),
        ("ku", "Tacîkistan"),
        ("kv", "Таджикистан"),
        ("kw", "Pow Tajik"),
        ("ky", "Тажикстан"),
        ("lo", "Tajikistan"),
        ("lt", "Tadžikistanas"),
        ("lv", "Tadžikistāna"),
        ("mi", "Tajikistan"),
        ("mk", "Таџикистан"),
        ("ml", "ത\u{d3e}ജികിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Тажикстан"),
        ("mr", "ताजिकिस\u{94d}तान"),
        ("ms", "Tadjikistan"),
        ("mt", "Tajikistan"),
        (
            "my",
            "တာဂျစ\u{103a}ကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tadjikitan"),
        ("nb", "Tadsjikistan"),
        ("ne", "ताजिकिस\u{94d}तान"),
        ("nl", "Tadzjikistan"),
        ("nn", "Tadsjikistan"),
        ("nv", "Tʼajiʼ Bikéyah"),
        ("oc", "Tatgiquistan"),
        ("or", "ତ\u{b3e}ଜୀକ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਤਜ਼ਾਕਸਤਾਨ"),
        ("pi", "ताजिकिस\u{94d}थान"),
        ("pl", "Tadżykistan"),
        ("ps", "تاجکستان"),
        ("pt", "Tajiquistão"),
        ("pt_BR", "Tadjiquistão"),
        ("ro", "Tajikistan"),
        ("ru", "Таджикистан"),
        ("rw", "Tajikisitani"),
        ("sc", "Tagìkistan"),
        ("sd", "تاجڪستان"),
        ("si", "ටජ\u{dd2}ක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
        ("sk", "Tadžikistan"),
        ("sl", "Tadžikistan"),
        ("so", "Tajikistan"),
        ("sq", "Taxhikistan"),
        ("sr", "Таџикистан"),
        ("sv", "Tadzjikistan"),
        ("sw", "Tajikistan"),
        ("ta", "தசிக\u{bcd}கிசுத\u{bbe}ன\u{bcd}"),
        ("te", "తజ\u{c3f}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Тоҷикистон"),
        ("th", "ทาจ\u{e34}ก\u{e34}สถาน"),
        ("ti", "ታጃኪስታን"),
        ("tk", "Täjigistan"),
        ("tl", "Tajikistan"),
        ("tr", "Tacikistan"),
        ("tt", "Тажыкстан"),
        ("ug", "تاجىكىستان"),
        ("uk", "Таджикистан"),
        ("ur", "تاجکستان"),
        ("uz", "Tojikiston"),
        ("ve", "Tajikistan"),
        ("vi", "Tha-gi-ki-xthanh"),
        ("wa", "Tadjikistan"),
        ("wo", "Taajikistaan"),
        ("xh", "Tajikistan"),
        ("yo", "Tajikistan"),
        ("zh_CN", "塔吉克斯坦"),
        ("zh_HK", "塔吉克斯坦"),
        ("zh_TW", "塔吉克"),
        ("zu", "Tajikistan"),
    ];
    #[cfg(all(feature = "tj", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 38.861034;
        pub const LONGITUDE: f64 = 71.276093;
        pub const MAX_LATITUDE: f64 = 41.044367;
        pub const MAX_LONGITUDE: f64 = 75.1539564;
        pub const MIN_LATITUDE: f64 = 36.6719898;
        pub const MIN_LONGITUDE: f64 = 67.34201209999999;
        pub const NORTHEAST_LATITUDE: f64 = 41.044367;
        pub const NORTHEAST_LONGITUDE: f64 = 75.1539564;
        pub const SOUTHWEST_LATITUDE: f64 = 36.6719898;
        pub const SOUTHWEST_LONGITUDE: f64 = 67.34201209999999;
    }
}
#[cfg(all(feature = "tj", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 38.861034,
            longitude: 71.276093,
            max_latitude: 41.044367,
            max_longitude: 75.1539564,
            min_latitude: 36.6719898,
            min_longitude: 67.34201209999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 41.044367,
                    longitude: 75.1539564,
                },
                southwest: CountryGeoBound {
                    latitude: 36.6719898,
                    longitude: 67.34201209999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "tj", feature = "subdivisions"))]
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
                    "DU",
                    Subdivision{
                        name: "DU",
                        country_alpha2: Alpha2::TJ,
                        code: "DU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Doesjanbe"), ("am", "ዱሻንቤ"), ("ar", "دوشانبي"), ("az", "Düşənbə"), ("be", "Горад Душанбэ"), ("bg", "Душанбе"), ("bn", "দ\u{9c1}শ\u{9be}ন\u{9cd}\u{200c}বে"), ("bs", "Dušanbe"), ("ca", "Duixanbe"), ("ccp", "𑄓\u{1112a}𑄥𑄚\u{11134}𑄝𑄬"), ("ceb", "Dushanbe"), ("cs", "Dušanbe"), ("cy", "Dushanbe"), ("da", "Dusjanbe"), ("de", "Duschanbe"), ("el", "Ντουσαμπέ"), ("en", "Dushanbe"), ("es", "Dusambé"), ("et", "Dušanbe"), ("eu", "Dushanbe"), ("fa", "دوشنبه"), ("fi", "Dušanbe"), ("fr", "Douchanbé"), ("ga", "Dushanbe"), ("gl", "Dushanbe"), ("gu", "દ\u{ac1}શ\u{a82}બ\u{ac7}"), ("he", "דושנבה"), ("hi", "द\u{941}शान\u{94d}ब\u{947}"), ("hr", "Dušanbe"), ("hu", "Dusanbe"), ("hy", "Դուշանբե"), ("id", "Dushanbe"), ("is", "Dúsjanbe"), ("it", "Dušanbe"), ("ja", "ドゥシャンベ"), ("jv", "Dushanbe"), ("ka", "დუშანბე"), ("kk", "Душанбе"), ("kn", "ದುಶಾಂಬ\u{cc6}"), ("ko", "두샨베"), ("ky", "Душанбе"), ("lt", "Dušanbė"), ("lv", "Dušanbe"), ("mk", "Душанбе"), ("ml", "ദ\u{d41}ഷ\u{d3e}ൻബെ"), ("mn", "Душанбе"), ("mr", "द\u{941}शा\u{902}ब\u{947}"), ("ms", "Dushanbe"), ("nb", "Dusjanbe"), ("ne", "द\u{941}शान\u{94d}व\u{947}"), ("nl", "Doesjanbe"), ("no", "Dusjanbe"), ("pa", "ਦ\u{a41}ਸ\u{a3c}\u{a70}ਬ\u{a47}"), ("pl", "Duszanbe"), ("ps", "دوشنبه"), ("pt", "Duchambe"), ("ro", "Dușanbe"), ("ru", "Душанбе"), ("si", "ඩ\u{dd4}ෂන\u{dca}බේ"), ("sk", "Dušanbe"), ("sl", "Dušanbe"), ("sq", "Dushanbe"), ("sr", "Душанбе"), ("sr_Latn", "Dušanbe"), ("sv", "Dusjanbe"), ("sw", "Dushanbe"), ("ta", "துச\u{bbe}ன\u{bcd}பே"), ("te", "దుశ\u{c3e}న\u{c4d}బ\u{c46}"), ("th", "ด\u{e39}ชานเบ"), ("tk", "Duşanbe"), ("tr", "Duşanbe"), ("uk", "Душанбе"), ("ur", "دوشنبہ"), ("uz", "Dushanbe"), ("vi", "Dushanbe"), ("yo", "Dushanbe"), ("yo_BJ", "Dushanbe"), ("yue", "杜尚別"), ("yue_Hans", "杜尚别"), ("zh", "杜尚别")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GB",
                    Subdivision{
                        name: "GB",
                        country_alpha2: Alpha2::TJ,
                        code: "GB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.412732), longitude: Some(73.087749), max_latitude: Some(39.46881500000001), min_latitude: Some(36.671535), max_longitude: Some(75.1514831), min_longitude: Some(70.216537)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جورنو"), ("az", "Dağlıq Bədəxşan Muxtar Vilayəti"), ("be", "Горна-Бадахшанская аўтаномная вобласць"), ("bg", "Горнобадахшанска автономна област"), ("bn", "গর\u{9cd}নো-ব\u{9be}দ\u{9be}খোস স\u{9cd}ব\u{9be}য\u{9bc}ত\u{9cd}তশ\u{9be}সিত প\u{9cd}রদেশ"), ("ca", "Gorno-Badakhxan"), ("ccp", "𑄉\u{1112e}𑄢\u{11134}𑄚\u{1112e}-𑄝𑄘\u{11128}𑄇\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Viloyati Mukhtori Kŭhistoni Badakhshon"), ("cs", "Horský Badachšán"), ("da", "Gorno-Badakhshan Autonomous Province"), ("de", "Berg-Badachschan"), ("el", "Αυτόνομη επαρχία Γκόρνο-Μπανταχσάν"), ("en", "Gorno-Badakhshan"), ("es", "Provincia de Alto Badajshán"), ("et", "Mägi-Badahšan"), ("fa", "ولایت مختار کوهستان بدخشان"), ("fi", "Vuoristo-Badahšanin autonominen alue"), ("fr", "Haut-Badakhchan"), ("gl", "Provincia Autónoma do Badaghxán Montañoso"), ("gu", "ગોર\u{acd}નો-બડખશન ઔટોનોમસ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז גורנו-בדחשאן"), ("hi", "क\u{942}हिस\u{94d}तोनी-बदख\u{93c}\u{94d}शान स\u{94d}वशासित प\u{94d}रान\u{94d}त"), ("hy", "Լեռնա-Բադախշանյան ինքնավար մարզ"), ("id", "Provinsi Otonom Gorno-Badakhshan"), ("it", "provincia Autonoma di Gorno-Badachšan"), ("ja", "ゴルノ・バダフシャン自治州"), ("ka", "მთიანი ბადახშანის ავტონომიური ოლქი"), ("kn", "ಗೊರ\u{ccd}ನೊ-ಬದ\u{ccd}ಖ\u{ccd}ಶಾನ\u{ccd} ಸ\u{ccd}ವಾಯತ\u{ccd}ತ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "고르노바다흐샨 자치주"), ("ky", "Тоолуу-Бадахшан автономиялуу облусу"), ("lt", "Kalnų Badachšano autonominė provincija"), ("lv", "Kalnu Badahšāna"), ("mr", "गोर\u{94d}नो-बदखशन स\u{94d}वायत\u{94d}त प\u{94d}रा\u{902}त"), ("ms", "Gorno-Badakhshan Autonomous Province"), ("nb", "Kuhistoni Badakhshon autonome provins"), ("nl", "Gorno-Badachsjan"), ("no", "Kuhistoni Badakhshon autonome provins"), ("pl", "Górski Badachszan"), ("pt", "Gorno-Badakhshan"), ("ru", "Горно-Бадахшанская автономная область"), ("si", "ගොර\u{dca}නෝ බඩක\u{dca}ශ\u{dcf}න\u{dca} ස\u{dca}ව\u{dcf}ධ\u{dd3}න පළ\u{dcf}ත"), ("sr", "Горно-Бадахшан"), ("sr_Latn", "Gorno-Badahšan"), ("sv", "Gorno-Badachsjan"), ("ta", "கோர\u{bcd}னோ -படக\u{bcd}ஹ\u{bcd}ஷன\u{bcd} ஆடோனோமோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3e}ర\u{c4d}న\u{c4b}-బడ\u{c3e}క\u{c4d}షన\u{c4d} అట\u{c3e}నమస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกอร\u{e4c}โน บาดาค\u{e4c}ชาน ออโตโนม\u{e31}ส"), ("tr", "Dağlık Badahşan"), ("uk", "Гірський Бадахшан"), ("ur", "گورنو بدخشاں خود مختار صوبہ"), ("uz", "Togʻli-Badaxshon muxtor viloyati"), ("vi", "Gorno-Badakhshan"), ("zh", "戈爾諾—巴達赫尚自治州")]),
                        unofficial_name_list: ["Gorno-Badahşan"].to_vec(),
                    }
                ),
                (
                    "KT",
                    Subdivision{
                        name: "KT",
                        country_alpha2: Alpha2::TJ,
                        code: "KT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9113562), longitude: Some(69.097023), max_latitude: Some(38.783459), min_latitude: Some(36.9281721), max_longitude: Some(70.34964699999999), min_longitude: Some(67.78614600000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة خاتلون"), ("be", "Хатлонская вобласць"), ("bg", "Хатлонска област"), ("bn", "খ\u{9be}তলোন প\u{9cd}রদেশ"), ("ccp", "𑄈𑄖\u{11134}𑄣\u{11127}𑄚\u{11134}"), ("ceb", "Viloyati Khatlon"), ("cs", "Chatlonský vilájet"), ("da", "Khatlon Province"), ("de", "Chatlon"), ("el", "Κάτλον"), ("en", "Khatlon"), ("es", "Provincia de Khatlon"), ("et", "Hatloni vilajett"), ("fa", "ولایت ختلان"), ("fi", "Hatlon"), ("fr", "Khatlon"), ("gu", "ખતલોન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז חאטלון"), ("hi", "ख\u{93c}तलोन प\u{94d}रान\u{94d}त"), ("hu", "Hatlon"), ("hy", "Խալտոնի մարզ"), ("id", "Khatlon"), ("it", "Chatlon"), ("ja", "ハトロン州"), ("ka", "ხატლონის ოლქი"), ("kn", "ಖಟ\u{ccd}ಲಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "하틀론 주"), ("lt", "Chatlono provincija"), ("lv", "Hatlona"), ("mr", "खतलॉन प\u{94d}रा\u{902}त"), ("ms", "Khatlon Province"), ("nb", "Khatlon"), ("nl", "Khatlon"), ("no", "Khatlon"), ("pl", "Wilajet chatloński"), ("pt", "Khatlon"), ("ru", "Хатлонская область"), ("si", "කැට\u{dca}ලෝන\u{dca} පළ\u{dcf}ත"), ("sv", "Chatlon"), ("ta", "கடலோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c3e}ట\u{c4d}ల\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เขตปกครองค\u{e31}ทลอน"), ("tr", "Hatlon"), ("uk", "Хатлонський вілоят"), ("ur", "صوبہ ختلان"), ("uz", "Xatlon"), ("vi", "Khatlon"), ("zh", "哈特隆州")]),
                        unofficial_name_list: ["Hatlon"].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "RA",
                        country_alpha2: Alpha2::TJ,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DistrictsUnderRepublicAdministration,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعات التبعية الجمهوري"), ("be", "раёны рэспубліканскага падпарадкавання Таджыкістана"), ("bg", "Райони на централно подчинение в Таджикистан"), ("bn", "জেল\u{9be} অফ রিপ\u{9be}বলিক\u{9be}ন স\u{9be}অর\u{9cd}ডিনেশন"), ("ccp", "𑄚\u{1112e}𑄦\u{11128}𑄠\u{1112e}𑄦\u{11130} 𑄑\u{1112e}𑄝𑄬\u{1112d} 𑄎\u{1112a}𑄟\u{11134}𑄦\u{1112a}𑄢\u{11128}"), ("ceb", "Region of Republican Subordination"), ("cs", "Centrálně spravovaná oblast"), ("da", "Karotegin"), ("de", "Nohijahoi tobei dschumhurij"), ("el", "Επαρχίες της Δημοκρατικής Υποταγής"), ("en", "Nohiyahoi Tobei Jumhurí"), ("es", "Región bajo subordinación republicana"), ("et", "Tadžikistani vabariikliku alluvusega rajoonid"), ("fa", "ناحیه\u{200c}های تابع جمهوری"), ("fi", "Hallinnon alaiset piirit"), ("fr", "Nohiyahoi tobei Jumhurii"), ("gu", "રિપબ\u{acd}લિકન , સબોર\u{acd}ડિન\u{ac7}શન , જિલ\u{acd}લો"), ("he", "נפות בכפיפות רפובליקנית"), ("hi", "गणत\u{902}त\u{94d}र-अधीन ज\u{93c}िल\u{947}"), ("id", "Region Republik Subordinasi"), ("it", "distretti di Subordinazione Repubblicana"), ("ja", "共和国直轄地"), ("ka", "ტაჯიკეთის რესპუბლიკური დაქვემდებარების რაიონები"), ("kn", "ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕನ\u{ccd} ಅಧೀನದ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}ಗಳು"), ("ko", "공화국 직할구"), ("lt", "Tiesioginio pavaldumo regionas"), ("lv", "Tadžikistānas republikas pakļautības rajoni"), ("mr", "रिपब\u{94d}लिकन ताल\u{941}क\u{94d}या\u{902}च\u{947} जिल\u{94d}ह\u{947}"), ("ms", "Districts of Republican Subordination"), ("nb", "Rebublican Subordination distrikt"), ("nl", "Regio ondergeordend aan de republiek"), ("no", "Rebublican Subordination distrikt"), ("pl", "Rejony Administrowane Centralnie"), ("pt", "Karotegin"), ("ru", "районы республиканского подчинения Таджикистана"), ("si", "යටත\u{dca}ව\u{dd2}ජ\u{dd2}ත ජනරජ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}ක"), ("sv", "Karotegin"), ("ta", "ரிபப\u{bcd}லிக\u{bcd}கன\u{bcd} சுபோர\u{bcd}டினேஷன\u{bcd} ம\u{bbe}வட\u{bcd}டங\u{bcd}கள\u{bcd}"), ("te", "ర\u{c3f}పబ\u{c4d}ల\u{c3f}కన\u{c4d} సబ\u{c3e}ర\u{c4d}డ\u{c3f}న\u{c47}షన\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}లు"), ("th", "เขค ร\u{e35}พล\u{e31}บบล\u{e34}ค เซอบอด\u{e34}เนช\u{e31}\u{e48}น"), ("tr", "Karategin"), ("uk", "райони республіканського підпорядкування"), ("ur", "جمہوریہ ماتحتی اضلاع"), ("uz", "Tojikistonning respublikaga boʻysunuvchi tumanlari"), ("vi", "Các Quận của Republican Subordination"), ("zh", "國家直轄區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::TJ,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5155326), longitude: Some(69.097023), max_latitude: Some(41.0422438), min_latitude: Some(38.926113), max_longitude: Some(70.984337), min_longitude: Some(67.3871309)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "صغد"), ("az", "Sogd vilayəti"), ("be", "Сагдыйская вобласць"), ("bg", "Согдийска област"), ("bn", "স\u{9c1}গ\u{9cd}ধ প\u{9cd}রদেশ"), ("ca", "Província de Sughd"), ("ccp", "𑄥\u{1112a}𑄇\u{11134}𑄦\u{11134}"), ("ceb", "Viloyati Sughd"), ("cs", "Sogdijský vilájet"), ("da", "Sughd Province"), ("de", "Sughd"), ("el", "Σούγκχντ"), ("en", "Sughd"), ("es", "Provincia de Sughd"), ("et", "Sugdi vilajett"), ("fa", "ولایت سغد"), ("fi", "Sugd"), ("fr", "Sughd"), ("gl", "Provincia de Sogdiana"), ("gu", "સ\u{ac1}ઘડ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סוגד"), ("hi", "स\u{941}ग\u{93c}\u{94d}द प\u{94d}रान\u{94d}त"), ("hy", "Սոգդիյան մարզ"), ("id", "Provinsi Sughd"), ("it", "Suǧd"), ("ja", "ソグド州"), ("ka", "სოღდის ოლქი"), ("kn", "ಸುಗ\u{ccd}ದ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "수그드 주"), ("lt", "Sugdo provincija"), ("lv", "Sugda"), ("mr", "स\u{941}घड प\u{94d}रा\u{902}त"), ("ms", "Sughd Province"), ("nb", "Sughd"), ("nl", "Sughd"), ("no", "Sughd"), ("pl", "Wilajet sogdyjski"), ("pt", "Sughd"), ("ro", "Sughd"), ("ru", "Согдийская область"), ("si", "ස\u{dd4}ග\u{dca}ඩ\u{dca} පළ\u{dcf}ත"), ("sv", "Sughd"), ("ta", "சுகட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సుగ\u{c4d}ధ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เขตซ\u{e31}กฮ\u{e31}ญ\u{e4c}"), ("tk", "Sogd welayaty"), ("tr", "Sogd"), ("uk", "Согдійська область"), ("ur", "صوبہ سغد"), ("uz", "Sugʻd"), ("vi", "Sughd"), ("zh", "索格特州")]),
                        unofficial_name_list: ["Chudjand", "Hucand", "Hudžand", "Hücand", "Khodzent", "Khodzhent", "Khudzhand", "Leninabad"].to_vec(),
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
#[cfg(feature = "tj")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::TJ,
        alpha3: Alpha3::TJK,
        address_format: None,
        continent: Continent::Asia,
        country_code: 992,
        currency_code: "TJS",
        gec: Some(GEC::TI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some("TJK"),
        iso_long_name: "The Republic of Tajikistan",
        iso_short_name: "Tajikistan",
        official_language_list: ["ru", "tg"].to_vec(),
        spoken_language_list: ["ru", "tg"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "8",
        nationality: Some("Tadzhik"),
        number: "762",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAsia),
        un_locode: "TJ",
        unofficial_name_list: [
            "Tajikistan",
            "Tadschikistan",
            "Tayikistán",
            "タジキスタン",
            "Tadzjikistan",
            "Tajikstan",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Tajikistan"),
            ("af", "Tadjikistan"),
            ("ak", "Tajikistan"),
            ("am", "ታጃጡስታን"),
            ("an", "Tajikistan"),
            ("ar", "طاجيكستان"),
            ("as", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
            ("ay", "Tajikistan"),
            ("az", "Tacikistan"),
            ("ba", "Tajikistan"),
            ("be", "Таджыкістан"),
            ("bg", "Таджикистан"),
            ("bi", "Tajikistan"),
            ("bn", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
            ("bn_IN", "ত\u{9be}জিকিস\u{9cd}ত\u{9be}ন"),
            ("br", "Tadjikistan"),
            ("bs", "Tadžikistan"),
            ("ca", "Tadjikistan"),
            ("ce", "Таджики"),
            ("ch", "Tajikistan"),
            ("cs", "Tádžikistán"),
            ("cv", "Таджики"),
            ("cy", "Tajikistan"),
            ("da", "Tadsjikistan"),
            ("de", "Tadschikistan"),
            ("dv", "ތ\u{7a6}ޖ\u{7a8}ކ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"),
            ("dz", "ཏ་ཇ\u{f72}་ཀ\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
            ("ee", "Tajikistan"),
            ("el", "Τατζικιστάν"),
            ("en", "Tajikistan"),
            ("eo", "Taĝikio"),
            ("es", "Tayikistán"),
            ("et", "Tadžikistan"),
            ("eu", "Tajikistan"),
            ("fa", "تاجیکستان"),
            ("ff", "Tajikistan"),
            ("fi", "Tadžikistan"),
            ("fo", "Tadsjikistan"),
            ("fr", "Tadjikistan"),
            ("fy", "Tadzjikistan"),
            ("ga", "An Táidsíceastáin"),
            ("gl", "Taxiquistán"),
            ("gn", "Tajikistan"),
            ("gu", "તાજીકિસ\u{acd}તાન"),
            ("gv", "Yn Tajikistaan"),
            ("ha", "Tajikistan"),
            ("he", "טג׳יקיסטן"),
            ("hi", "ताजिकिस\u{94d}तान"),
            ("hr", "Tadžikistan"),
            ("ht", "Tadjikistan"),
            ("hu", "Tádzsikisztán"),
            ("hy", "Տաճիկստան"),
            ("ia", "Tajikistan"),
            ("id", "Tajikistan"),
            ("io", "Tajikistan"),
            ("is", "Tadsíkistan"),
            ("it", "Tagikistan"),
            ("iu", "Tajikistan"),
            ("ja", "タジキスタン"),
            ("ka", "ტაჯიკეთი"),
            ("ki", "Tajikistan"),
            ("kk", "Тәжікстан"),
            ("kl", "Tajikistan"),
            (
                "km",
                "តាហ\u{17d2}ស\u{17ca}\u{17b8}គ\u{17b8}ស\u{17d2}តង\u{17cb}",
            ),
            ("kn", "ತಾಜ\u{cbf}ಕ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"),
            ("ko", "타지키스탄"),
            ("ku", "Tacîkistan"),
            ("kv", "Таджикистан"),
            ("kw", "Pow Tajik"),
            ("ky", "Тажикстан"),
            ("lo", "Tajikistan"),
            ("lt", "Tadžikistanas"),
            ("lv", "Tadžikistāna"),
            ("mi", "Tajikistan"),
            ("mk", "Таџикистан"),
            ("ml", "ത\u{d3e}ജികിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Тажикстан"),
            ("mr", "ताजिकिस\u{94d}तान"),
            ("ms", "Tadjikistan"),
            ("mt", "Tajikistan"),
            (
                "my",
                "တာဂျစ\u{103a}ကစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tadjikitan"),
            ("nb", "Tadsjikistan"),
            ("ne", "ताजिकिस\u{94d}तान"),
            ("nl", "Tadzjikistan"),
            ("nn", "Tadsjikistan"),
            ("nv", "Tʼajiʼ Bikéyah"),
            ("oc", "Tatgiquistan"),
            ("or", "ତ\u{b3e}ଜୀକ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
            ("pa", "ਤਜ਼ਾਕਸਤਾਨ"),
            ("pi", "ताजिकिस\u{94d}थान"),
            ("pl", "Tadżykistan"),
            ("ps", "تاجکستان"),
            ("pt", "Tajiquistão"),
            ("pt_BR", "Tadjiquistão"),
            ("ro", "Tajikistan"),
            ("ru", "Таджикистан"),
            ("rw", "Tajikisitani"),
            ("sc", "Tagìkistan"),
            ("sd", "تاجڪستان"),
            ("si", "ටජ\u{dd2}ක\u{dd2}ස\u{dca}ත\u{dcf}නය"),
            ("sk", "Tadžikistan"),
            ("sl", "Tadžikistan"),
            ("so", "Tajikistan"),
            ("sq", "Taxhikistan"),
            ("sr", "Таџикистан"),
            ("sv", "Tadzjikistan"),
            ("sw", "Tajikistan"),
            ("ta", "தசிக\u{bcd}கிசுத\u{bbe}ன\u{bcd}"),
            ("te", "తజ\u{c3f}క\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
            ("tg", "Тоҷикистон"),
            ("th", "ทาจ\u{e34}ก\u{e34}สถาน"),
            ("ti", "ታጃኪስታን"),
            ("tk", "Täjigistan"),
            ("tl", "Tajikistan"),
            ("tr", "Tacikistan"),
            ("tt", "Тажыкстан"),
            ("ug", "تاجىكىستان"),
            ("uk", "Таджикистан"),
            ("ur", "تاجکستان"),
            ("uz", "Tojikiston"),
            ("ve", "Tajikistan"),
            ("vi", "Tha-gi-ki-xthanh"),
            ("wa", "Tadjikistan"),
            ("wo", "Taajikistaan"),
            ("xh", "Tajikistan"),
            ("yo", "Tajikistan"),
            ("zh_CN", "塔吉克斯坦"),
            ("zh_HK", "塔吉克斯坦"),
            ("zh_TW", "塔吉克"),
            ("zu", "Tajikistan"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}