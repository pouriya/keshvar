// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of South Africa

#[cfg(all(feature = "za", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::ZA;
    pub const ALPHA3: Alpha3 = Alpha3::ZAF;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 27;
    pub const CURRENCY_CODE: &str = "ZAR";
    pub const GEC: Option<GEC> = Some(GEC::SF);
    pub const INTERNATIONAL_PREFIX: &str = "09";
    pub const IOC: Option<&str> = Some("RSA");
    pub const ISO_SHORT_NAME: &str = "South Africa";
    pub const ISO_LONG_NAME: &str = "The Republic of South Africa";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] =
        &["af", "en", "nr", "ss", "st", "tn", "ts", "ve", "xh", "zu"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] =
        &["af", "en", "nr", "ss", "st", "tn", "ts", "ve", "xh", "zu"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("South African");
    pub const NUMBER: &str = "710";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAfrica);
    pub const UN_LOCODE: &str = "ZA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "South Africa",
        "Republik Südafrika",
        "Afrique du Sud",
        "República de Sudáfrica",
        "南アフリカ",
        "Zuid-Afrika",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "South Africa"),
        ("af", "Suid-Afrika"),
        ("ak", "South Africa"),
        ("am", "፠ቡብ ጐፔሱጢ"),
        ("an", "South Africa"),
        ("ar", "جنوب إفريقيا"),
        ("as", "দক\u{9cd}ষিণ আফ\u{9cd}ৰিক\u{9be}"),
        ("ay", "South Africa"),
        ("az", "Cənubi Afrika"),
        ("ba", "South Africa"),
        ("be", "Паўднёва-Афрыканская Рэспубліка"),
        ("bg", "Южна Африка"),
        ("bi", "South Africa"),
        ("bn", "দক\u{9cd}ষিণ আফ\u{9cd}রিক\u{9be}"),
        ("bn_IN", "দক\u{9cd}ষিণ আফ\u{9cd}রিক\u{9be}"),
        ("br", "Suafrika"),
        ("bs", "Južna Afrika"),
        ("ca", "Sudàfrica"),
        ("ce", "Къилба-Африкин Республика"),
        ("ch", "South Africa"),
        ("cs", "Jihoafrická republika"),
        ("cv", "Къилба-Африкин Республика"),
        ("cy", "De Affrica"),
        ("da", "Sydafrika"),
        ("de", "Südafrika"),
        (
            "dv",
            "ދ\u{7ac}ކ\u{7aa}ނ\u{7aa} އ\u{7ac}ފ\u{7b0}ރ\u{7a8}ކ\u{7a7}",
        ),
        ("dz", "ལ\u{fb7}\u{f7c}་ཨཕ་ར\u{f72}་ཀ"),
        ("ee", "South Africa"),
        ("el", "Νότια Αφρική"),
        ("en", "South Africa"),
        ("eo", "Sud-Afriko"),
        ("es", "Sudáfrica"),
        ("et", "Lõuna-Aafrika Vabariik"),
        ("eu", "Hegoafrika"),
        ("fa", "آفریقای جنوبی"),
        ("ff", "South Africa"),
        ("fi", "Etelä-Afrikka"),
        ("fo", "Suðurafrika"),
        ("fr", "Afrique du Sud"),
        ("fy", "Súd-Afrika"),
        ("ga", "An Afraic Theas"),
        ("gl", "África do Sur"),
        ("gn", "South Africa"),
        ("gu", "દક\u{acd}ષિણ આફ\u{acd}રિકા"),
        ("gv", "Yn Affrick Yiass"),
        ("ha", "Afirka ta kudu"),
        ("he", "דרום אפריקה"),
        ("hi", "दक\u{94d}षिण अफ\u{93c}\u{94d}रीका"),
        ("hr", "Južnoafrička Republika"),
        ("ht", "Afrik disid"),
        ("hu", "Dél-Afrika"),
        ("hy", "Հարավային Աֆրիկա"),
        ("ia", "Africa del Sud"),
        ("id", "Afrika Selatan"),
        ("io", "Sudafrika"),
        ("is", "Suður-Afríka"),
        ("it", "Sudafrica"),
        ("iu", "South Africa"),
        ("ja", "南アフリカ"),
        ("ka", "სამხრეთ აფრიკა"),
        ("ki", "South Abĩrika"),
        ("kk", "Оңтүстік Африка"),
        ("kl", "South Africa"),
        (
            "km",
            "អាហ\u{17d2}វ\u{17d2}រ\u{17b7}ក\u{200b}ខាងត\u{17d2}ប\u{17bc}ង",
        ),
        ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಆಫ\u{ccd}ರ\u{cbf}ಕಾ"),
        ("ko", "남아프리카 공화국"),
        ("ku", "Efrîka Basur"),
        ("kv", "Лунвыв Африкаса Республика"),
        ("kw", "Afrika Dhyhow"),
        ("ky", "Түштүк-Африка Республикасы"),
        ("lo", "South Africa"),
        ("lt", "Pietų Afrika"),
        ("lv", "Dienvidāfrika"),
        ("mi", "Āwherika-ki-te-tonga"),
        ("mk", "Јужна Африка"),
        ("ml", "സൌത\u{d4d}ത\u{d4d} ആഫ\u{d4d}രിക\u{d4d}ക"),
        ("mn", "Өмнөд африк"),
        ("mr", "साउथ आफ\u{94d}रिका"),
        ("ms", "Afrika Selatan"),
        ("mt", "Afrika t'Isfel"),
        (
            "my",
            "တောင\u{103a}အာဖရ\u{102d}ကန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "South Aprika"),
        ("nb", "Sør-Afrika"),
        ("ne", "दक\u{94d}षिण अफ\u{94d}रिका"),
        ("nl", "Zuid-Afrika"),
        ("nn", "Sør-Afrika"),
        ("nv", "Kéyah Naakaii Łizhinii Bikéyah Shádiʼááhjí Siʼánígíí"),
        ("oc", "Sudafrica"),
        ("or", "ଦକ\u{b4d}ଷ\u{b3f}ଣ ଆଫ\u{b4d}ର\u{b3f}କ\u{b3e}"),
        ("pa", "ਦ\u{a71}ਖਣੀ ਅਫਰੀਕਾ"),
        ("pi", "दक\u{94d}षिण-अफ\u{94d}रीका"),
        ("pl", "Republika Południowej Afryki"),
        ("ps", "سویلي افریقا"),
        ("pt", "África do Sul"),
        ("pt_BR", "África do Sul"),
        ("ro", "Africa de sud"),
        ("ru", "Южная Африка"),
        ("rw", "Afurika yepfo"),
        ("sc", "Sudàfrica"),
        ("sd", "ڏکڻ آفريڪا"),
        ("si", "දක\u{dd4}ණ\u{dd4} අප\u{dca}\u{200d}ර\u{dd2}ක\u{dcf}ව"),
        ("sk", "Južná Afrika"),
        ("sl", "Južna Afrika"),
        ("so", "Koonfur Afrika"),
        ("sq", "Afrikë e Jugut"),
        ("sr", "Јужна Африка"),
        ("sv", "Sydafrika"),
        ("sw", "South Africa"),
        ("ta", "தென\u{bcd} ஆப\u{bcd}ரிக\u{bcd}க\u{bbe}"),
        ("te", "స\u{c4c}త\u{c4d} ఆఫ\u{c4d}ర\u{c3f}క\u{c3e}"),
        ("tg", "Африкаи Ҷанубӣ"),
        ("th", "แอฟร\u{e34}กาใต\u{e49}"),
        ("ti", "ደቡብ ኣፍሪቃ"),
        ("tk", "Günorta Afrika"),
        ("tl", "Timog Aprika"),
        ("tr", "Güney Afrika"),
        ("tt", "Көняк Африка"),
        ("ug", "جەنۇبىي ئافرىقا"),
        ("uk", "Південна Африка"),
        ("ur", "جنوبی افریقا"),
        ("uz", "Janubiy Afrika Respublikasi"),
        ("ve", "Afurika Tshipembe"),
        ("vi", "Nam Phi"),
        ("wa", "Nonne Afrike"),
        ("wo", "Afrik di Sid"),
        ("xh", "Mzantsi Afrika"),
        ("yo", "Gúúsù Áfríkà"),
        ("zh_CN", "南非"),
        ("zh_HK", "南非"),
        ("zh_TW", "南非"),
        ("zu", "IRiphabliki yaseNingizimu Afrika"),
    ];
    #[cfg(all(feature = "za", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -30.559482;
        pub const LONGITUDE: f64 = 22.937506;
        pub const MAX_LATITUDE: f64 = -22.1254239;
        pub const MAX_LONGITUDE: f64 = 38.2216904;
        pub const MIN_LATITUDE: f64 = -47.1313489;
        pub const MIN_LONGITUDE: f64 = 16.2816999;
        pub const NORTHEAST_LATITUDE: f64 = -22.1254239;
        pub const NORTHEAST_LONGITUDE: f64 = 38.2216904;
        pub const SOUTHWEST_LATITUDE: f64 = -47.1313489;
        pub const SOUTHWEST_LONGITUDE: f64 = 16.2816999;
    }
}
#[cfg(all(feature = "za", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -30.559482,
            longitude: 22.937506,
            max_latitude: -22.1254239,
            max_longitude: 38.2216904,
            min_latitude: -47.1313489,
            min_longitude: 16.2816999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -22.1254239,
                    longitude: 38.2216904,
                },
                southwest: CountryGeoBound {
                    latitude: -47.1313489,
                    longitude: 16.2816999,
                },
            },
        }
    }
}

#[cfg(all(feature = "za", feature = "subdivisions"))]
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
                    "EC",
                    Subdivision{
                        name: "EC",
                        country_alpha2: Alpha2::ZA,
                        code: "EC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-32.2968402), longitude: Some(26.419389), max_latitude: Some(-30.0018012), min_latitude: Some(-34.2136378), max_longitude: Some(30.1947187), min_longitude: Some(22.7357412)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oos-Kaap"), ("ar", "كيب الشرقية"), ("az", "Şərqi Burun vilayəti"), ("be", "Усходне-Капская правінцыя"), ("bg", "Източен Кейп"), ("bn", "প\u{9c2}র\u{9cd}ব কেপ প\u{9cd}রদেশ"), ("ca", "Cap Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄇𑄬𑄛\u{11134}"), ("ceb", "Eastern Cape"), ("cs", "Východní Kapsko"), ("da", "Øst-Kapprovinsen"), ("de", "Ostkap"), ("el", "Ανατολικό Ακρωτήριο"), ("en", "Eastern Cape"), ("es", "Provincia Oriental del Cabo"), ("et", "Ida-Kapimaa"), ("eu", "Lurmutur Ekialdea"), ("fa", "کیپ شرقی"), ("fi", "Eastern Cape"), ("fr", "Cap oriental"), ("gl", "O Cabo Oriental"), ("gu", "ઇસ\u{acd}ટર\u{acd}ન ક\u{ac7}પ"), ("he", "הכף המזרחי"), ("hi", "प\u{942}र\u{94d}वी क\u{947}प प\u{94d}रान\u{94d}त"), ("hr", "Eastern Cape"), ("hu", "Kelet-Fokföld"), ("hy", "Արևելյան Քեյփ²"), ("id", "Eastern Cape"), ("it", "provincia del Capo Orientale"), ("ja", "東ケープ州"), ("ka", "აღმოსავლეთი კაპლანდი"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಕೇಪ\u{ccd}"), ("ko", "이스턴케이프 주"), ("lt", "Rytų Kapas"), ("lv", "Austrumkāpa"), ("mk", "Источен Кејп"), ("ml", "കിഴക\u{d4d}കൻ കേപ\u{d4d}"), ("mr", "ईस\u{94d}टर\u{94d}न क\u{947}प"), ("ms", "Cape Timur"), ("nb", "Eastern Cape"), ("nl", "Oost-Kaap"), ("no", "Eastern Cape"), ("pl", "Prowincja Przylądkowa Wschodnia"), ("pt", "Cabo Oriental"), ("ro", "Provincia Eastern Cape"), ("ru", "Восточно-Капская провинция"), ("si", "නැගෙනහ\u{dd2}ර කේප\u{dca}"), ("so", "Eastern Cape Province"), ("sr", "Источни Кејп"), ("sr_Latn", "Istočni Kejp"), ("sv", "Östra Kapprovinsen"), ("sw", "Rasi ya Mashariki"), ("ta", "கிழக\u{bcd}கு கேப\u{bcd}"), ("te", "తూర\u{c4d}పు క\u{c47}ప\u{c4d}"), ("th", "อ\u{e35}สเท\u{e34}ร\u{e4c}น เคป"), ("tr", "Doğu Kap"), ("uk", "Східна Капська провінція"), ("ur", "مشرقی کیپ"), ("vi", "Đông Cape"), ("yo", "Eastern Cape"), ("yo_BJ", "Eastern Cape"), ("yue", "東開普"), ("yue_Hans", "东开普"), ("zh", "東開普省"), ("zu", "KwaXhosa")]),
                        unofficial_name_list: ["Oos Kaap"].to_vec(),
                    }
                ),
                (
                    "FS",
                    Subdivision{
                        name: "FS",
                        country_alpha2: Alpha2::ZA,
                        code: "FS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-28.4541105), longitude: Some(26.7967849), max_latitude: Some(-26.6687389), min_latitude: Some(-30.69408), max_longitude: Some(29.7851298), min_longitude: Some(24.3466211)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vrystaat"), ("ar", "فري ستيت"), ("az", "Fri Steyt vilayəti"), ("be", "Фры-Стэйт"), ("bg", "Фрайстат"), ("bn", "ফ\u{9cd}রি অঙ\u{9cd}গর\u{9be}জ\u{9cd}য"), ("ca", "Estat Lliure"), ("ccp", "𑄜\u{11133}𑄢\u{11128}"), ("ceb", "Free State"), ("cs", "Svobodný stát"), ("cy", "Free State"), ("da", "Fristatprovinsen"), ("de", "Freistaat"), ("el", "Ελεύθερο Κράτος της Οράγγης"), ("en", "Free State"), ("es", "Provincia del Estado Libre"), ("et", "Free State"), ("eu", "Estatu Askea"), ("fa", "ایالت آزاد (استان)"), ("fi", "Free State"), ("fr", "État-Libre"), ("gl", "Estado Libre"), ("gu", "ફ\u{acd}રી સ\u{acd}ટ\u{ac7}ટ"), ("he", "המדינה החופשית"), ("hi", "फ\u{93c}\u{94d}री स\u{94d}ट\u{947}ट प\u{94d}रान\u{94d}त"), ("hr", "Free State"), ("hu", "Szabadállam"), ("hy", "Ֆրի Սթեյթ"), ("id", "Free State"), ("it", "Free State"), ("ja", "フリーステイト州"), ("ka", "ფრი-სტეიტი"), ("kn", "ಮುಕ\u{ccd}ತ ರಾಜ\u{ccd}ಯ"), ("ko", "프리스테이트 주"), ("lt", "Fristeitas"), ("lv", "Frīsteita"), ("mk", "Фристејт"), ("ml", "ഫ\u{d4d}രീ സ\u{d4d}റ\u{d4d}റേറ\u{d4d}റ\u{d4d}"), ("mr", "फ\u{94d}री स\u{94d}ट\u{947}ट"), ("ms", "Free State"), ("my", "ဩရ\u{102d}န\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Free State"), ("nl", "Vrijstaat"), ("no", "Free State"), ("pl", "Wolne Państwo"), ("pt", "Estado Livre"), ("ro", "Provincia Free State"), ("ru", "Фри-Стейт"), ("si", "න\u{dd2}දහස\u{dca} ර\u{dcf}ජ\u{dca}\u{200d}ය"), ("so", "Free State Province"), ("sr", "Фри Стејт"), ("sr_Latn", "Fri Stejt"), ("sv", "Fristatsprovinsen"), ("sw", "Dola Huru"), ("ta", "விடுதலை இர\u{bbe}ச\u{bcd}சியம\u{bcd}"), ("te", "ఫ\u{c4d}ర\u{c40} స\u{c4d}ట\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐอ\u{e34}สระ"), ("tr", "Özgür Devlet"), ("uk", "Вільна держава"), ("ur", "آزاد ریاست (صوبہ)"), ("vi", "Free State"), ("yo", "Free State"), ("yo_BJ", "Free State"), ("yue", "自由邦"), ("yue_Hans", "自由邦"), ("zh", "自由邦省"), ("zu", "IFleyistata")]),
                        unofficial_name_list: ["Foreistata", "Vrystaat"].to_vec(),
                    }
                ),
                (
                    "GP",
                    Subdivision{
                        name: "GP",
                        country_alpha2: Alpha2::ZA,
                        code: "GP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.2707593), longitude: Some(28.1122679), max_latitude: Some(-25.1096099), min_latitude: Some(-26.9118973), max_longitude: Some(29.0984187), min_longitude: Some(27.1563401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gauteng"), ("ar", "غاوتينج"), ("az", "Qautenq vilayəti"), ("be", "Гаўтэнг"), ("bg", "Гаутенг"), ("bn", "গ\u{9c1}টেং"), ("ca", "Gauteng"), ("ccp", "𑄉𑄅\u{1112a}𑄑𑄬\u{11101}"), ("ceb", "Gauteng"), ("cs", "Gauteng"), ("cy", "Gauteng"), ("da", "Gauteng"), ("de", "Gauteng"), ("el", "Γκαουτένγκ"), ("en", "Gauteng"), ("es", "Gauteng"), ("et", "Gauteng"), ("eu", "Gauteng"), ("fa", "گائوتنگ"), ("fi", "Gauteng"), ("fr", "Gauteng"), ("gu", "ગ\u{acd}વાટ\u{ac7}\u{a82}ગ"), ("he", "חאוטנג"), ("hi", "ख\u{93c}ाउत\u{947}न\u{94d}ग प\u{94d}रान\u{94d}त"), ("hr", "Gauteng"), ("hy", "Գաուտենգ"), ("id", "Gauteng"), ("it", "Gauteng"), ("ja", "ハウテン州"), ("ka", "გაუტენგი"), ("kk", "Гаутенг"), ("kn", "ಗ\u{ccc}ಟ\u{cc6}ಂಗ\u{ccd}"), ("ko", "하우텡 주"), ("lt", "Gautengas"), ("lv", "Hautena"), ("mk", "Гаутенг"), ("ml", "ഗ\u{d57}റ\u{d4d}റെങ\u{d4d}"), ("mr", "ग\u{94d}वाट\u{947}\u{902}ग"), ("ms", "Gauteng"), ("nb", "Gauteng"), ("nl", "Gauteng"), ("no", "Gauteng"), ("pl", "Gauteng"), ("pt", "Gauteng"), ("ro", "Provincia Gauteng"), ("ru", "Гаутенг"), ("si", "ගව\u{dd4}ටෙන\u{dca}ග\u{dca}"), ("sk", "Gauteng"), ("so", "Gauteng Province"), ("sr", "Хаутенг"), ("sr_Latn", "Hauteng"), ("sv", "Gauteng"), ("sw", "Gauteng"), ("ta", "கவுதெங\u{bcd}"), ("te", "గ\u{c4c}ట\u{c46}ంగ\u{c4d}"), ("th", "ก\u{e31}วเตง"), ("tr", "Gauteng"), ("uk", "Ґаутенг"), ("ur", "گاؤتنگ"), ("vi", "Gauteng"), ("yo", "Gauteng"), ("yo_BJ", "Gauteng"), ("yue", "豪登"), ("yue_Hans", "豪登"), ("zh", "豪登省"), ("zu", "IGauteng")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LP",
                    Subdivision{
                        name: "LP",
                        country_alpha2: Alpha2::ZA,
                        code: "LP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-23.4012946), longitude: Some(29.4179324), max_latitude: Some(-22.1250298), min_latitude: Some(-25.4227899), max_longitude: Some(31.8838412), min_longitude: Some(26.4075388)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Limpopo"), ("ar", "ليمبوبو"), ("az", "Limpopo vilayəti"), ("be", "Правінцыя Лімпопа"), ("bg", "Лимпопо"), ("bn", "লিম\u{9cd}পোপো"), ("ca", "Província de Limpopo"), ("ccp", "𑄣\u{11128}𑄟\u{11134}𑄛\u{1112e}𑄛\u{1112e}"), ("ceb", "Limpopo"), ("cs", "Limpopo"), ("da", "Limpopo"), ("de", "Limpopo"), ("el", "Λιμπόπο"), ("en", "Limpopo"), ("es", "Provincia de Limpopo"), ("et", "Limpopo provints"), ("eu", "Limpopo"), ("fa", "لیمپوپو"), ("fi", "Limpopon provinssi"), ("fr", "Limpopo"), ("gl", "Limpopo"), ("gu", "લિમ\u{acd}પોપો"), ("he", "לימפופו"), ("hi", "लिम\u{94d}पोपो प\u{94d}रान\u{94d}त"), ("hr", "Limpopo"), ("hy", "Լիմպոպո"), ("id", "Limpopo"), ("it", "provincia del Limpopo"), ("ja", "リンポポ州"), ("ka", "ლიმპოპო"), ("kn", "ಲ\u{cbf}ಂಪೊಪೋ"), ("ko", "림포포 주"), ("lt", "Limpopas"), ("lv", "Limpopo"), ("mk", "Лимпопо"), ("ml", "ലിംപോപോ"), ("mr", "लिम\u{94d}पोपो"), ("ms", "Limpopo"), ("nb", "Limpopo"), ("nl", "Limpopo"), ("no", "Limpopo"), ("pl", "Limpopo"), ("pt", "Limpopo"), ("ro", "Provincia Limpopo"), ("ru", "Лимпопо"), ("si", "ල\u{dd2}ම\u{dca}පොපෝ"), ("sk", "Limpopo"), ("so", "Limpopo Province"), ("sr", "Лимпопо"), ("sr_Latn", "Limpopo"), ("sv", "Limpopoprovinsen"), ("sw", "Limpopo"), ("ta", "லிம\u{bcd}போபோ"), ("te", "ల\u{c3f}ంప\u{c4b}ప\u{c4b}"), ("th", "ล\u{e34}มโปโป"), ("tr", "Limpopo"), ("uk", "Лімпопо"), ("ur", "لیمپوپو"), ("vi", "Limpopo"), ("yo", "Limpopo"), ("yo_BJ", "Limpopo"), ("yue", "林波波"), ("yue_Hans", "林波波"), ("zh", "林波波省"), ("zu", "Limpopo")]),
                        unofficial_name_list: ["Limpopo"].to_vec(),
                    }
                ),
                (
                    "MP",
                    Subdivision{
                        name: "MP",
                        country_alpha2: Alpha2::ZA,
                        code: "MP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-29.817), longitude: Some(30.637), max_latitude: Some(-29.7785889), min_latitude: Some(-29.853415), max_longitude: Some(30.67445), min_longitude: Some(30.5890079)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mpumalanga"), ("ar", "مبومالانجا"), ("az", "Mpumalanqa vilayəti"), ("be", "Правінцыя Мпумаланга"), ("bg", "Мпумаланга"), ("bn", "এম\u{9cd}পোম\u{9be}ল\u{9be}গ\u{9be}"), ("ca", "Mpumalanga"), ("ccp", "𑄛\u{1112a}𑄟𑄣𑄚\u{11134}𑄉"), ("ceb", "Mpumalanga"), ("cs", "Mpumalanga"), ("da", "Mpumalanga"), ("de", "Mpumalanga"), ("el", "Μπουμαλάνγκα"), ("en", "Mpumalanga"), ("es", "Mpumalanga"), ("et", "Mpumalanga"), ("eu", "Mpumalanga"), ("fa", "امپومالانگا"), ("fi", "Mpumalanga"), ("fr", "Mpumalanga"), ("gl", "Mpumalanga"), ("gu", "એમપ\u{ac1}મલ\u{a82}ગા"), ("he", "מפומלנגה"), ("hi", "अमप\u{942}मला\u{902}गा प\u{94d}रान\u{94d}त"), ("hr", "Mpumalanga"), ("hy", "Մպումալանգա"), ("id", "Mpumalanga"), ("it", "Mpumalanga"), ("ja", "ムプマランガ州"), ("ka", "მპუმალანგა"), ("kn", "ಮಪುಮಾಲಂಗಾ"), ("ko", "음푸말랑가 주"), ("lt", "Mpumalanga"), ("lv", "Mpumalanga"), ("mk", "Мпумаланга"), ("ml", "പ\u{d41}മ\u{d3e}ലൻഗ"), ("mr", "उम\u{94d}प\u{941}माला\u{902}गा"), ("ms", "Mpumalanga"), ("nb", "Mpumalanga"), ("nl", "Mpumalanga"), ("no", "Mpumalanga"), ("pl", "Mpumalanga"), ("pt", "Mpumalanga"), ("ro", "Provincia Mpumalanga"), ("ru", "Мпумаланга"), ("si", "එම\u{dca}ප\u{dd4}මලන\u{dca}ග\u{dcf}"), ("so", "Mpumalanga Province"), ("sr", "Мпумаланга"), ("sr_Latn", "Mpumalanga"), ("sv", "Mpumalanga"), ("sw", "Mpumalanga"), ("ta", "இம\u{bcd}புமல\u{bbe}ங\u{bcd}க\u{bbe}"), ("te", "ఎంపుమల\u{c3e}ంగ\u{c3e}"), ("th", "เขตราชษาฮ\u{e35}"), ("tr", "Mpumalanga"), ("uk", "Мпумаланга"), ("ur", "ماپومالانگا"), ("vi", "Mpumalanga"), ("yo", "Mpumalanga"), ("yo_BJ", "Mpumalanga"), ("yue", "普馬蘭加"), ("yue_Hans", "普马兰加"), ("zh", "普馬蘭加省"), ("zu", "Mpumalanga")]),
                        unofficial_name_list: ["eMpumalanga"].to_vec(),
                    }
                ),
                (
                    "NC",
                    Subdivision{
                        name: "NC",
                        country_alpha2: Alpha2::ZA,
                        code: "NC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-29.0466808), longitude: Some(21.8568586), max_latitude: Some(-24.76586), min_latitude: Some(-32.9449877), max_longitude: Some(25.54933), min_longitude: Some(16.451941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Kaap"), ("ar", "كيب الشمالية"), ("az", "Şimal Burun əyaləti"), ("be", "Паўночна-Капская правінцыя"), ("bg", "Северен Кейп"), ("bn", "উত\u{9cd}তর কেপ প\u{9cd}রদেশ"), ("ca", "Cap Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄇𑄬𑄛\u{11134}"), ("ceb", "Province of Northern Cape"), ("cs", "Severní Kapsko"), ("da", "Nord-Kapprovinsen"), ("de", "Nordkap"), ("el", "Βόρειο Ακρωτήριο"), ("en", "Northern Cape"), ("es", "Provincia Septentrional del Cabo"), ("et", "Põhja-Kapimaa"), ("eu", "Lurmutur Iparraldea"), ("fa", "کیپ شمالی"), ("fi", "Northern Cape"), ("fr", "Cap du Nord"), ("gu", "નોર\u{acd}ધન ક\u{ac7}પ"), ("he", "הכף הצפוני"), ("hi", "उत\u{94d}तरी क\u{947}प प\u{94d}रान\u{94d}त"), ("hr", "Northern Cape"), ("hu", "Észak-Fokföld"), ("hy", "Հյուսիսային Քեյփ"), ("id", "Northern Cape"), ("it", "provincia del Capo Settentrionale"), ("ja", "北ケープ州"), ("ka", "ჩრდილოეთი კაპლანდი"), ("kn", "ಉತ\u{ccd}ತರ ಕೇಪ\u{ccd}"), ("ko", "노던케이프 주"), ("lt", "Šiaurės Kapas"), ("lv", "Ziemeļkāpa"), ("mk", "Северен Кејп"), ("mr", "नॉर\u{94d}दर\u{94d}न क\u{947}प"), ("ms", "Northern Cape"), ("nb", "Northern Cape"), ("nl", "Noord-Kaap"), ("no", "Northern Cape"), ("pl", "Prowincja Przylądkowa Północna"), ("pt", "Cabo Setentrional"), ("ro", "Provincia Northern Cape"), ("ru", "Северо-Капская провинция"), ("si", "උත\u{dd4}ර\u{dd4} කේප\u{dca}"), ("sk", "Severné Kapsko"), ("so", "Northern Cape Province"), ("sr", "Северни Кејп"), ("sr_Latn", "Severni Kejp"), ("sv", "Norra Kapprovinsen"), ("sw", "Rasi ya Kaskazini"), ("ta", "வடக\u{bcd}கு கேப\u{bcd}"), ("te", "ఉత\u{c4d}తర క\u{c47}ప\u{c4d}"), ("th", "นอร\u{e4c}ทเท\u{e34}ร\u{e4c}น แคป"), ("tr", "Kuzey Kap"), ("uk", "Північна Капська провінція"), ("ur", "شمالی کیپ"), ("vi", "Bắc Cape"), ("yo", "Northern Cape"), ("yo_BJ", "Northern Cape"), ("yue", "北開普"), ("yue_Hans", "北开普"), ("zh", "北開普省"), ("zu", "IKipi laseNyakatho")]),
                        unofficial_name_list: ["Northern Cape"].to_vec(),
                    }
                ),
                (
                    "NW",
                    Subdivision{
                        name: "NW",
                        country_alpha2: Alpha2::ZA,
                        code: "NW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.6638599), longitude: Some(25.2837585), max_latitude: Some(-24.6366288), min_latitude: Some(-28.112206), max_longitude: Some(28.2983488), min_longitude: Some(22.6290299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordwes"), ("ar", "الشمالية الغربية"), ("az", "Şimal-Qərb vilayəti"), ("be", "Паўночна-Заходняя правінцыя"), ("bg", "Северозападна провинция"), ("bn", "উত\u{9cd}তর পশ\u{9cd}চিম প\u{9cd}রদেশ"), ("ca", "Província del Nord-oest"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}"), ("ceb", "Province of North West"), ("cs", "Severozápadní provincie"), ("da", "Nordvest"), ("de", "Nordwest"), ("el", "Βορειοδυτική Περιφέρεια, Νότια Αφρική"), ("en", "North West"), ("es", "Provincia del Noroeste"), ("et", "Loodeprovints"), ("eu", "Ipar-mendebaldea"), ("fa", "شمال غربی (استان آفریقای جنوبی)"), ("fi", "North West"), ("fr", "Nord-Ouest"), ("gu", "નોર\u{acd}થ વ\u{ac7}સ\u{acd}ટ"), ("he", "הפרובינציה הצפון-מערבית"), ("hi", "पश\u{94d}चिमोत\u{94d}तर प\u{94d}रान\u{94d}त"), ("hr", "North West"), ("hu", "North West"), ("hy", "Հյուսիսարևմտյան նահանգ"), ("id", "North West, Afrika Selatan"), ("it", "provincia del Nordovest"), ("ja", "北西州"), ("ka", "ჩრდილო-დასავლეთი"), ("kn", "ವಾಯುವ\u{ccd}ಯ ದ. ಆಫ\u{ccd}ರ\u{cbf}ಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "노스웨스트 주"), ("lt", "Šiaurės Vakarai"), ("lv", "Ziemeļrietumi"), ("mk", "Северозапад"), ("mr", "नॉर\u{94d}थ व\u{947}स\u{94d}ट"), ("ms", "North West, Afrika Selatan"), ("nb", "North West"), ("nl", "Noordwest"), ("no", "North West"), ("pl", "Prowincja Północno-Zachodnia"), ("pt", "Noroeste"), ("ro", "Provincia North West"), ("ru", "Северо-Западная провинция"), ("si", "උත\u{dd4}ර\u{dd4} බටහ\u{dd2}ර"), ("so", "North West Province"), ("sr", "Северозападна покрајина"), ("sr_Latn", "Severozapadna pokrajina"), ("sv", "Nordvästprovinsen"), ("sw", "Kaskazini-Magharibi"), ("ta", "வட மேற\u{bcd}கு"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} వ\u{c46}స\u{c4d}ట\u{c4d}"), ("th", "นอร\u{e4c}ธเวสท\u{e4c}"), ("tr", "Kuzeybatı"), ("uk", "Північно-Західна провінція"), ("ur", "شمال مغربی (جنوبی افریقی صوبہ)"), ("vi", "Tây Bắc"), ("yo", "Àríwá-Ìwọòrùn"), ("yo_BJ", "Àríwá-Ìwɔòrùn"), ("yue", "西北省"), ("yue_Hans", "西北省"), ("zh", "西北省"), ("zu", "Nyakatho-Ntshonalanga")]),
                        unofficial_name_list: ["North-West"].to_vec(),
                    }
                ),
                (
                    "WC",
                    Subdivision{
                        name: "WC",
                        country_alpha2: Alpha2::ZA,
                        code: "WC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-33.2277918), longitude: Some(21.8568586), max_latitude: Some(-30.4302599), min_latitude: Some(-34.8330058), max_longitude: Some(24.22241), min_longitude: Some(17.7575637)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Kaap"), ("ar", "كيب الغربية"), ("az", "Qərbi Kaap vilayəti"), ("be", "Заходне-Капская правінцыя"), ("bg", "Западен Кейп"), ("bn", "পশ\u{9cd}চিম কেপ প\u{9cd}রদেশ"), ("ca", "Cap Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄇𑄬𑄛\u{11134}"), ("ceb", "Province of the Western Cape"), ("cs", "Západní Kapsko"), ("da", "Vest-Kapprovinsen"), ("de", "Westkap"), ("el", "Δυτικό Ακρωτήριο"), ("en", "Western Cape"), ("es", "Provincia Occidental del Cabo"), ("et", "Lääne-Kapimaa"), ("eu", "Lurmutur Mendebaldea"), ("fa", "کیپ غربی"), ("fi", "Western Cape"), ("fr", "Cap occidental"), ("gl", "Cabo Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટર\u{acd}ન ક\u{ac7}પ"), ("he", "הכף המערבי"), ("hi", "पश\u{94d}चिमी क\u{947}प प\u{94d}रान\u{94d}त"), ("hr", "Western Cape"), ("hu", "Nyugat-Fokföld"), ("hy", "Արևելյան Քեյփ"), ("id", "Western Cape"), ("it", "provincia del Capo Occidentale"), ("ja", "西ケープ州"), ("ka", "დასავლეთი კაპლანდი"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}ನ\u{ccd} ಕೇಪ\u{ccd}"), ("ko", "웨스턴케이프 주"), ("lt", "Vakarų Kapas"), ("lv", "Rietumkāpa"), ("mk", "Западен Кејп"), ("ml", "പടിഞ\u{d4d}ഞ\u{d3e}റൻ കേപ\u{d4d}"), ("mr", "व\u{947}स\u{94d}टर\u{94d}न क\u{947}प"), ("ms", "Cape Barat"), ("nb", "Western Cape"), ("nl", "West-Kaap"), ("no", "Western Cape"), ("pl", "Prowincja Przylądkowa Zachodnia"), ("pt", "Cabo Ocidental"), ("ro", "Provincia Western Cape"), ("ru", "Западно-Капская провинция"), ("si", "බටහ\u{dd2}ර කේප\u{dca}"), ("sk", "Západné Kapsko"), ("so", "Western Cape Province"), ("sr", "Западни Кејп"), ("sr_Latn", "Zapadni Kejp"), ("sv", "Västra Kapprovinsen"), ("sw", "Rasi ya Magharibi"), ("ta", "மேற\u{bcd}கு கேப\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}టర\u{c4d}న\u{c4d} క\u{c47}ప\u{c4d}"), ("th", "เวสเท\u{e34}ร\u{e4c}น เคป"), ("tr", "Batı Kap"), ("uk", "Західна Капська провінція"), ("ur", "مغربی کیپ"), ("vi", "Tây Cape"), ("yo", "Western Cape"), ("yo_BJ", "Western Cape"), ("yue", "西開普"), ("yue_Hans", "西开普"), ("zh", "西開普省"), ("zu", "IKipi laseNtshonalanga")]),
                        unofficial_name_list: ["Wes Kaap"].to_vec(),
                    }
                ),
                (
                    "ZN",
                    Subdivision{
                        name: "ZN",
                        country_alpha2: Alpha2::ZA,
                        code: "ZN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-28.5305539), longitude: Some(30.8958242), max_latitude: Some(-26.80442), min_latitude: Some(-31.0853648), max_longitude: Some(32.8909911), min_longitude: Some(28.8734801)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "KwaZulu-Natal"), ("ar", "كوازولو ناتال"), ("az", "Kvazulu-Natal vilayəti"), ("be", "Правінцыя Квазулу-Натал"), ("bg", "Квазулу-Натал"), ("bn", "কোয\u{9bc}\u{9be}-জ\u{9c1}ল\u{9c1} ন\u{9be}ট\u{9be}ল প\u{9cd}রদেশ"), ("ca", "KwaZulu-Natal"), ("ccp", "𑄤𑄎\u{1112a}𑄣\u{1112a}-𑄚𑄑𑄣\u{11134}"), ("ceb", "Province of KwaZulu-Natal"), ("cs", "KwaZulu-Natal"), ("cy", "KwaZulu-Natal"), ("da", "KwaZulu-Natal"), ("de", "KwaZulu-Natal"), ("el", "Νατάλ"), ("en", "KwaZulu-Natal"), ("es", "KwaZulu-Natal"), ("et", "KwaZulu-Natal"), ("eu", "KwaZulu-Natal"), ("fa", "کوازولو-ناتال"), ("fi", "KwaZulu-Natal"), ("fr", "KwaZulu-Natal"), ("gl", "KwaZulu-Natal"), ("gu", "ક\u{acd}વાઝ\u{ac1}લ\u{ac1}-નાતાલ"), ("he", "קוואזולו-נטאל"), ("hi", "क\u{94d}वाज\u{93c}\u{942}ल\u{942}-नताल प\u{94d}रान\u{94d}त"), ("hr", "KwaZulu-Natal"), ("hu", "KwaZulu-Natal"), ("hy", "Նատալ"), ("id", "KwaZulu-Natal"), ("it", "KwaZulu-Natal"), ("ja", "クワズール・ナタール州"), ("ka", "კვაზულუ-ნატალი"), ("kn", "ಕ\u{ccd}ವಾಝುಲು-ನಟಾಲ\u{ccd}"), ("ko", "콰줄루나탈 주"), ("lt", "Kvazulu-Natalis"), ("lv", "Kvazulu-Natāla"), ("mk", "Натал"), ("ml", "ക\u{d4d}വ\u{d3e}സ\u{d41}ള\u{d41}-നറ\u{d4d}റ\u{d3e}ൽ"), ("mr", "क\u{94d}वाझ\u{941}ल\u{942}-नाताल"), ("ms", "KwaZulu-Natal"), ("nb", "KwaZulu-Natal"), ("nl", "KwaZoeloe-Natal"), ("no", "KwaZulu-Natal"), ("pl", "KwaZulu-Natal"), ("pt", "KwaZulu-Natal"), ("ro", "Provincia KwaZulu-Natal"), ("ru", "Квазулу-Натал"), ("si", "ක\u{dca}ව\u{dcf}ස\u{dd4}ල\u{dd4}-නට\u{dcf}ල\u{dca}"), ("sk", "Kwa-Zulu Natal"), ("so", "KwaZulu-Natal Province"), ("sr", "Квазулу-Натал"), ("sr_Latn", "Kvazulu-Natal"), ("sv", "KwaZulu-Natal"), ("sw", "KwaZulu-Natal"), ("ta", "குவ\u{bbe}சுலு-நத\u{bbe}ல\u{bcd}"), ("te", "క\u{c4d}వ\u{c3e}జులు-న\u{c47}టల\u{c4d}"), ("th", "ควาซ\u{e39}ล\u{e39} นาทาล"), ("tr", "KwaZulu-Natal"), ("uk", "Квазулу-Наталь"), ("ur", "کوازولو نیٹل"), ("vi", "KwaZulu-Natal"), ("yo", "KwaZulu-Natal"), ("yo_BJ", "KwaZulu-Natal"), ("yue", "夸祖魯-納塔爾"), ("yue_Hans", "夸祖鲁-纳塔尔"), ("zh", "夸祖魯-納塔爾省"), ("zu", "KwaZulu-Natal")]),
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
#[cfg(feature = "za")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::ZA,
        alpha3: Alpha3::ZAF,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{postalcode}}\n{{country}}",
        ),
        continent: Continent::Africa,
        country_code: 27,
        currency_code: "ZAR",
        gec: Some(GEC::SF),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "09",
        ioc: Some("RSA"),
        iso_long_name: "The Republic of South Africa",
        iso_short_name: "South Africa",
        official_language_list: ["af", "en", "nr", "ss", "st", "tn", "ts", "ve", "xh", "zu"]
            .to_vec(),
        spoken_language_list: ["af", "en", "nr", "ss", "st", "tn", "ts", "ve", "xh", "zu"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("South African"),
        number: "710",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernAfrica),
        un_locode: "ZA",
        unofficial_name_list: [
            "South Africa",
            "Republik Südafrika",
            "Afrique du Sud",
            "República de Sudáfrica",
            "南アフリカ",
            "Zuid-Afrika",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "South Africa"),
            ("af", "Suid-Afrika"),
            ("ak", "South Africa"),
            ("am", "፠ቡብ ጐፔሱጢ"),
            ("an", "South Africa"),
            ("ar", "جنوب إفريقيا"),
            ("as", "দক\u{9cd}ষিণ আফ\u{9cd}ৰিক\u{9be}"),
            ("ay", "South Africa"),
            ("az", "Cənubi Afrika"),
            ("ba", "South Africa"),
            ("be", "Паўднёва-Афрыканская Рэспубліка"),
            ("bg", "Южна Африка"),
            ("bi", "South Africa"),
            ("bn", "দক\u{9cd}ষিণ আফ\u{9cd}রিক\u{9be}"),
            ("bn_IN", "দক\u{9cd}ষিণ আফ\u{9cd}রিক\u{9be}"),
            ("br", "Suafrika"),
            ("bs", "Južna Afrika"),
            ("ca", "Sudàfrica"),
            ("ce", "Къилба-Африкин Республика"),
            ("ch", "South Africa"),
            ("cs", "Jihoafrická republika"),
            ("cv", "Къилба-Африкин Республика"),
            ("cy", "De Affrica"),
            ("da", "Sydafrika"),
            ("de", "Südafrika"),
            (
                "dv",
                "ދ\u{7ac}ކ\u{7aa}ނ\u{7aa} އ\u{7ac}ފ\u{7b0}ރ\u{7a8}ކ\u{7a7}",
            ),
            ("dz", "ལ\u{fb7}\u{f7c}་ཨཕ་ར\u{f72}་ཀ"),
            ("ee", "South Africa"),
            ("el", "Νότια Αφρική"),
            ("en", "South Africa"),
            ("eo", "Sud-Afriko"),
            ("es", "Sudáfrica"),
            ("et", "Lõuna-Aafrika Vabariik"),
            ("eu", "Hegoafrika"),
            ("fa", "آفریقای جنوبی"),
            ("ff", "South Africa"),
            ("fi", "Etelä-Afrikka"),
            ("fo", "Suðurafrika"),
            ("fr", "Afrique du Sud"),
            ("fy", "Súd-Afrika"),
            ("ga", "An Afraic Theas"),
            ("gl", "África do Sur"),
            ("gn", "South Africa"),
            ("gu", "દક\u{acd}ષિણ આફ\u{acd}રિકા"),
            ("gv", "Yn Affrick Yiass"),
            ("ha", "Afirka ta kudu"),
            ("he", "דרום אפריקה"),
            ("hi", "दक\u{94d}षिण अफ\u{93c}\u{94d}रीका"),
            ("hr", "Južnoafrička Republika"),
            ("ht", "Afrik disid"),
            ("hu", "Dél-Afrika"),
            ("hy", "Հարավային Աֆրիկա"),
            ("ia", "Africa del Sud"),
            ("id", "Afrika Selatan"),
            ("io", "Sudafrika"),
            ("is", "Suður-Afríka"),
            ("it", "Sudafrica"),
            ("iu", "South Africa"),
            ("ja", "南アフリカ"),
            ("ka", "სამხრეთ აფრიკა"),
            ("ki", "South Abĩrika"),
            ("kk", "Оңтүстік Африка"),
            ("kl", "South Africa"),
            (
                "km",
                "អាហ\u{17d2}វ\u{17d2}រ\u{17b7}ក\u{200b}ខាងត\u{17d2}ប\u{17bc}ង",
            ),
            ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಆಫ\u{ccd}ರ\u{cbf}ಕಾ"),
            ("ko", "남아프리카 공화국"),
            ("ku", "Efrîka Basur"),
            ("kv", "Лунвыв Африкаса Республика"),
            ("kw", "Afrika Dhyhow"),
            ("ky", "Түштүк-Африка Республикасы"),
            ("lo", "South Africa"),
            ("lt", "Pietų Afrika"),
            ("lv", "Dienvidāfrika"),
            ("mi", "Āwherika-ki-te-tonga"),
            ("mk", "Јужна Африка"),
            ("ml", "സൌത\u{d4d}ത\u{d4d} ആഫ\u{d4d}രിക\u{d4d}ക"),
            ("mn", "Өмнөд африк"),
            ("mr", "साउथ आफ\u{94d}रिका"),
            ("ms", "Afrika Selatan"),
            ("mt", "Afrika t'Isfel"),
            (
                "my",
                "တောင\u{103a}အာဖရ\u{102d}ကန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "South Aprika"),
            ("nb", "Sør-Afrika"),
            ("ne", "दक\u{94d}षिण अफ\u{94d}रिका"),
            ("nl", "Zuid-Afrika"),
            ("nn", "Sør-Afrika"),
            ("nv", "Kéyah Naakaii Łizhinii Bikéyah Shádiʼááhjí Siʼánígíí"),
            ("oc", "Sudafrica"),
            ("or", "ଦକ\u{b4d}ଷ\u{b3f}ଣ ଆଫ\u{b4d}ର\u{b3f}କ\u{b3e}"),
            ("pa", "ਦ\u{a71}ਖਣੀ ਅਫਰੀਕਾ"),
            ("pi", "दक\u{94d}षिण-अफ\u{94d}रीका"),
            ("pl", "Republika Południowej Afryki"),
            ("ps", "سویلي افریقا"),
            ("pt", "África do Sul"),
            ("pt_BR", "África do Sul"),
            ("ro", "Africa de sud"),
            ("ru", "Южная Африка"),
            ("rw", "Afurika yepfo"),
            ("sc", "Sudàfrica"),
            ("sd", "ڏکڻ آفريڪا"),
            ("si", "දක\u{dd4}ණ\u{dd4} අප\u{dca}\u{200d}ර\u{dd2}ක\u{dcf}ව"),
            ("sk", "Južná Afrika"),
            ("sl", "Južna Afrika"),
            ("so", "Koonfur Afrika"),
            ("sq", "Afrikë e Jugut"),
            ("sr", "Јужна Африка"),
            ("sv", "Sydafrika"),
            ("sw", "South Africa"),
            ("ta", "தென\u{bcd} ஆப\u{bcd}ரிக\u{bcd}க\u{bbe}"),
            ("te", "స\u{c4c}త\u{c4d} ఆఫ\u{c4d}ర\u{c3f}క\u{c3e}"),
            ("tg", "Африкаи Ҷанубӣ"),
            ("th", "แอฟร\u{e34}กาใต\u{e49}"),
            ("ti", "ደቡብ ኣፍሪቃ"),
            ("tk", "Günorta Afrika"),
            ("tl", "Timog Aprika"),
            ("tr", "Güney Afrika"),
            ("tt", "Көняк Африка"),
            ("ug", "جەنۇبىي ئافرىقا"),
            ("uk", "Південна Африка"),
            ("ur", "جنوبی افریقا"),
            ("uz", "Janubiy Afrika Respublikasi"),
            ("ve", "Afurika Tshipembe"),
            ("vi", "Nam Phi"),
            ("wa", "Nonne Afrike"),
            ("wo", "Afrik di Sid"),
            ("xh", "Mzantsi Afrika"),
            ("yo", "Gúúsù Áfríkà"),
            ("zh_CN", "南非"),
            ("zh_HK", "南非"),
            ("zh_TW", "南非"),
            ("zu", "IRiphabliki yaseNingizimu Afrika"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}