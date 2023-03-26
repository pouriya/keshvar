// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Costa Rica

#[cfg(all(feature = "cr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CR;
    pub const ALPHA3: Alpha3 = Alpha3::CRI;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 506;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::CRC;
    pub const GEC: Option<GEC> = Some(GEC::CS);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CRC);
    pub const ISO_SHORT_NAME: &str = "Costa Rica";
    pub const ISO_LONG_NAME: &str = "The Republic of Costa Rica";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Costa Rican");
    pub const NUMBER: &str = "188";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4,5}|\\d{3}-\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "CR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Costa Rica", "コスタリカ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Costa Rica"),
        ("af", "Costa Rica"),
        ("ak", "Costa Rica"),
        ("am", "Costa Rica"),
        ("an", "Costa Rica"),
        ("ar", "كوستاريكا"),
        ("as", "কোস\u{9cd}ট\u{9be} ৰিক\u{9be}"),
        ("ay", "Costa Rica"),
        ("az", "Kosta Rika"),
        ("ba", "Costa Rica"),
        ("be", "Коста-Рыка"),
        ("bg", "Коста Рика"),
        ("bi", "Costa Rica"),
        ("bn", "কোস\u{9cd}ট\u{9be} রিক\u{9be}"),
        ("bn_IN", "কোস\u{9cd}ট\u{9be} রিক\u{9be}"),
        ("br", "Costa Rica"),
        ("bs", "Kostarika"),
        ("ca", "Costa Rica"),
        ("ce", "Коста-Рика"),
        ("ch", "Costa Rica"),
        ("cs", "Kostarika"),
        ("cv", "Коста-Рика"),
        ("cy", "Costa Rica"),
        ("da", "Costa Rica"),
        ("de", "Costa Rica"),
        ("dv", "ކ\u{7ae}ސ\u{7b0}ޓ\u{7a6}ރ\u{7a9}ކ\u{7a7}"),
        ("dz", "ཀ\u{f71}\u{f7c}ས\u{f72}་ཊ་ར\u{f72}་ཀ།"),
        ("ee", "Costa Rica"),
        ("el", "Κόστα Ρίκα"),
        ("en", "Costa Rica"),
        ("eo", "Kostariko"),
        ("es", "Costa Rica"),
        ("et", "Costa Rica"),
        ("eu", "Costa Rica"),
        ("fa", "کاستاریکا"),
        ("ff", "Costa Rica"),
        ("fi", "Costa Rica"),
        ("fo", "Kosta Rika"),
        ("fr", "Costa Rica"),
        ("fy", "Kosta Rika"),
        ("ga", "Cósta Rice"),
        ("gl", "Costa Rica"),
        ("gn", "Costa Rica"),
        ("gu", "કોસ\u{acd}ટા રીકા"),
        ("gv", "Yn Coose Berçhagh"),
        ("ha", "Costa Rica"),
        ("he", "קוסטה ריקה"),
        ("hi", "कोस\u{94d}टा रीका"),
        ("hr", "Kostarika"),
        ("ht", "Kostarika"),
        ("hu", "Costa Rica"),
        ("hy", "Կոստա-Ռիկա"),
        ("ia", "Costa Rica"),
        ("id", "Kosta Rika"),
        ("io", "Kosta Rika"),
        ("is", "Kosta Ríka"),
        ("it", "Costa Rica"),
        ("iu", "Costa Rica"),
        ("ja", "コスタリカ"),
        ("ka", "კოსტა-რიკა"),
        ("ki", "Costa Rica"),
        ("kk", "Коста-Рика"),
        ("kl", "Costa Rica"),
        ("km", "ក\u{17bc}ស\u{17d2}តារ\u{17b8}កា"),
        ("kn", "ಕೋಸ\u{ccd}ಟಾರ\u{cbf}ಕಾ"),
        ("ko", "코스타리카"),
        ("ku", "Kosta Rîka"),
        ("kv", "Коста-Рика"),
        ("kw", "Kosta Rika"),
        ("ky", "Коста-Рика"),
        ("lo", "ປະເທດກ\u{ebb}ດສະຕາລ\u{eb4}ກາ"),
        ("lt", "Kosta Rika"),
        ("lv", "Kostarika"),
        ("mi", "Costa Rica"),
        ("mk", "Костарика"),
        ("ml", "കോസ\u{d4d}റ\u{d4d}റ റിക"),
        ("mn", "Коста рика"),
        ("mr", "कोस\u{94d}टारिका"),
        ("ms", "Costa Rica"),
        ("mt", "Kosta Rika"),
        (
            "my",
            "ကော\u{1037}စတာရ\u{102e}ကာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Kosta Rika"),
        ("nb", "Costa Rica"),
        ("ne", "कोस\u{94d}टारिका"),
        ("nl", "Costa Rica"),
        ("nn", "Costa Rica"),
        ("nv", "Costa Rica"),
        ("oc", "Còsta Rica"),
        ("or", "କୋଷ\u{b4d}ଟ\u{b3e} ର\u{b3f}କ\u{b3e}"),
        ("pa", "ਕਾਸਟ ਰੀਕਾ"),
        ("pi", "कोस\u{94d}टा रीका"),
        ("pl", "Kostaryka"),
        ("ps", "کوسټاریکا"),
        ("pt", "Costa Rica"),
        ("pt_BR", "Costa Rica"),
        ("ro", "Costa Rica"),
        ("ru", "Коста-Рика"),
        ("rw", "Kosita Rika"),
        ("sc", "Costa Rica"),
        ("sd", "Costa Rica"),
        ("si", "කොස\u{dca}ට\u{dcf} ර\u{dd3}ක\u{dcf}"),
        ("sk", "Kostarika"),
        ("sl", "Kostarika"),
        ("so", "Kosta Rika"),
        ("sq", "Kosta Rikë"),
        ("sr", "Костарика"),
        ("sv", "Costa Rica"),
        ("sw", "Costa Rica"),
        ("ta", "கோஸ\u{bcd}ட\u{bbe}ரிக\u{bbe}"),
        ("te", "క\u{c4b}స\u{c4d}ట\u{c3e} ర\u{c3f}క\u{c3e}"),
        ("tg", "Коста-Рика"),
        ("th", "คอสตาร\u{e34}กา"),
        ("ti", "ኮስታ ሪካ"),
        ("tk", "Kosta-Rika"),
        ("tl", "Costa Rica"),
        ("tr", "Kosta Rika"),
        ("tt", "Коста Рика"),
        ("ug", "كوستارىكا"),
        ("uk", "Коста-Рика"),
        ("ur", "کوسٹاریکا"),
        ("uz", "Kosta Rika"),
        ("ve", "Costa Rica"),
        ("vi", "Cốt-x-tha Ri-ca"),
        ("wa", "Costa Rica"),
        ("wo", "Kosta Riika"),
        ("xh", "Costa Rica"),
        ("yo", "Kóstá Rikà"),
        ("zh_CN", "哥斯达黎加"),
        ("zh_HK", "哥斯達黎加"),
        ("zh_TW", "哥斯大黎加"),
        ("zu", "Costa Rica"),
    ];
    #[cfg(all(feature = "cr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 9.748916999999999;
        pub const LONGITUDE: f64 = -83.753428;
        pub const MAX_LATITUDE: f64 = 11.2196806;
        pub const MAX_LONGITUDE: f64 = -82.51830009999999;
        pub const MIN_LATITUDE: f64 = 5.496099999999999;
        pub const MIN_LONGITUDE: f64 = -87.09899999999999;
        pub const NORTHEAST_LATITUDE: f64 = 11.2196806;
        pub const NORTHEAST_LONGITUDE: f64 = -82.51830009999999;
        pub const SOUTHWEST_LATITUDE: f64 = 5.496099999999999;
        pub const SOUTHWEST_LONGITUDE: f64 = -87.09899999999999;
    }
}
#[cfg(all(feature = "cr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 9.748916999999999,
            longitude: -83.753428,
            max_latitude: 11.2196806,
            max_longitude: -82.51830009999999,
            min_latitude: 5.496099999999999,
            min_longitude: -87.09899999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 11.2196806,
                    longitude: -82.51830009999999,
                },
                southwest: CountryGeoBound {
                    latitude: 5.496099999999999,
                    longitude: -87.09899999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "cr", feature = "subdivisions"))]
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
                    "A",
                    Subdivision{
                        name: "Alajuela",
                        country_alpha2: Alpha2::CR,
                        code: "A",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.391583), longitude: Some(-84.4382721), max_latitude: Some(11.0793161), min_latitude: Some(9.8215051), max_longitude: Some(-84.149513), min_longitude: Some(-85.43863999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الأخويلا"), ("bg", "Алахуела"), ("bn", "আল\u{9be}জ\u{9c1}য\u{9bc}েল\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Alajuela"), ("ccp", "𑄃𑄣\u{11134}𑄎\u{1112a}𑄠𑄬𑄣"), ("ceb", "Provincia de Alajuela"), ("cs", "Alajuela"), ("da", "Alajuela"), ("de", "Provinz Alajuela"), ("el", "Επαρχία Αλαχουέλα"), ("en", "Alajuela"), ("es", "Provincia de Alajuela"), ("eu", "Alajuela (probintzia)"), ("fa", "استان الاهوئلا"), ("fi", "Alajuelan maakunta"), ("fr", "Alajuela"), ("gu", "અલાજ\u{ac1}એલા પ\u{acd}રા\u{a82}ત"), ("he", "אלחואלה"), ("hi", "अलाउएला प\u{94d}रा\u{902}त"), ("hr", "Alajuela, provincija"), ("hy", "Ալախուելա"), ("id", "Provinsi Alajuela"), ("it", "provincia di Alajuela"), ("ja", "アラフエラ州"), ("ka", "ალახუელის პროვინცია"), ("kn", "ಅಲಾಜುವ\u{cc6}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알라후엘라 주"), ("lt", "Alachuelos provincija"), ("lv", "Alahvelas province"), ("mr", "अलाज\u{941}एला प\u{94d}रा\u{902}त"), ("ms", "Alajuela Province"), ("nb", "Alajuela"), ("nl", "Alajuela"), ("no", "Alajuela"), ("pl", "Alajuela"), ("pt", "Alajuela"), ("ro", "Provincia Alajuela"), ("ru", "Алахуэла"), ("sd", "الاخويلا"), ("si", "අලජ\u{dd4}එල\u{dcf} පළ\u{dcf}ත"), ("sr", "Алахуела"), ("sr_Latn", "Alahuela"), ("sv", "Alajuela"), ("ta", "ல\u{bbe}ஜுஎல\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అల\u{c3e}జుయ\u{c46}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาลาค\u{e39}เอลา,"), ("tr", "Alajuela Province"), ("uk", "Алахуела"), ("ur", "الاخویلا صوبہ"), ("vi", "Alajuela"), ("zh", "阿拉胡埃拉省")]),
                        unofficial_name_list: ["Alajuela"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "Cartago",
                        country_alpha2: Alpha2::CR,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.7539596), longitude: Some(-83.67739279999999), max_latitude: Some(10.1429971), min_latitude: Some(9.505219100000001), max_longitude: Some(-83.301808), min_longitude: Some(-84.0711091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كارتاغو"), ("bg", "Картаго"), ("ca", "Província de Cartago"), ("ccp", "𑄇𑄢\u{11134}𑄑𑄉\u{1112e}"), ("ceb", "Provincia de Cartago"), ("cs", "Cartago"), ("da", "Cartago"), ("de", "Provinz Cartago"), ("el", "Επαρχία Καρτάγο"), ("en", "Cartago"), ("es", "Provincia de Cartago"), ("eu", "Cartago (probintzia)"), ("fa", "استان کارتاگو"), ("fi", "Cartagon maakunta"), ("fr", "Cartago"), ("he", "קרטגו"), ("hr", "Cartago, provincija"), ("hy", "Կարտագո"), ("id", "Provinsi Cartago"), ("it", "provincia di Cartago"), ("ja", "カルタゴ州"), ("ka", "კარტაგოს პროვინცია"), ("ko", "카르타고 주"), ("lt", "Kartago provincija"), ("nb", "Cartago"), ("nl", "Cartago"), ("no", "Cartago"), ("pl", "Cartago"), ("pt", "Cartago"), ("ro", "Provincia Cartago"), ("ru", "Картаго"), ("sr", "Картаго"), ("sr_Latn", "Kartago"), ("sv", "Cartago"), ("tr", "Cartago ili"), ("uk", "Картаго"), ("ur", "کارتاگو صوبہ"), ("vi", "Cartago"), ("zh", "卡塔戈省")]),
                        unofficial_name_list: ["Cartago"].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "Guanacaste",
                        country_alpha2: Alpha2::CR,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.4957971), longitude: Some(-85.35496499999999), max_latitude: Some(11.2168181), min_latitude: Some(9.723861099999999), max_longitude: Some(-84.760645), min_longitude: Some(-85.9460828)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غواناكاسته"), ("bg", "Гуанакасте"), ("bn", "গ\u{9c1}ন\u{9be}স\u{9cd}য\u{9be}চ প\u{9cd}রদেশ"), ("ca", "Província de Guanacaste"), ("ccp", "𑄉\u{1112a}𑄠𑄚𑄇𑄌\u{11134}𑄑𑄬"), ("ceb", "Provincia de Guanacaste"), ("cs", "Guanacaste"), ("da", "Guanacaste"), ("de", "Provinz Guanacaste"), ("el", "Επαρχία Γκουανακάστε"), ("en", "Guanacaste"), ("es", "Provincia de Guanacaste"), ("fa", "استان گواناکاسته"), ("fi", "Guanacasten maakunta"), ("fr", "Guanacaste"), ("gu", "ગ\u{ac1}આનાકાસ\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("he", "גואנאקאסטה"), ("hi", "ग\u{941}नाकास\u{94d}ट प\u{94d}रा\u{902}त"), ("hr", "Guanacaste, provincija"), ("hy", "Գուանակաստե"), ("id", "Provinsi Guanacaste"), ("it", "provincia di Guanacaste"), ("ja", "グアナカステ州"), ("ka", "გუანაკასტეს პროვინცია"), ("kn", "ಗುವಾನಾಕಸ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "과나카스테 주"), ("lt", "Gvanakastės provincija"), ("lv", "Gvanakastes province"), ("mr", "ग\u{941}नाकास\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Guanacaste Province"), ("nb", "Guanacaste"), ("nl", "Guanacaste"), ("no", "Guanacaste"), ("pl", "Guanacaste"), ("pt", "Guanacaste"), ("ro", "Provincia Guanacaste"), ("ru", "Гуанакасте"), ("si", "ගයනකස\u{dca}ටේ පළ\u{dcf}ත"), ("sr", "Гванакасте"), ("sr_Latn", "Gvanakaste"), ("sv", "Guanacaste"), ("ta", "குணஸ\u{bbe}ஸ\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గువ\u{c3e}న\u{c3e}క\u{c3e}స\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ก\u{e31}วนาคาสเต\u{e49}"), ("tr", "Guanacaste Province"), ("uk", "Ґуанакасте"), ("ur", "گواناکاستے صوبہ"), ("vi", "Guanacaste"), ("zh", "瓜纳卡斯特省")]),
                        unofficial_name_list: ["Guanacaste"].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "Heredia",
                        country_alpha2: Alpha2::CR,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.473523), longitude: Some(-84.01674229999999), max_latitude: Some(10.789644), min_latitude: Some(9.9513191), max_longitude: Some(-83.714804), min_longitude: Some(-84.19046589999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيريذيا"), ("be", "Правінцыя Эрэдзія"), ("bg", "Ередия"), ("ca", "Província d’Heredia"), ("ccp", "𑄦𑄬𑄢𑄬𑄓\u{11128}𑄠"), ("ceb", "Provincia de Heredia"), ("cs", "Heredia"), ("da", "Heredia"), ("de", "Provinz Heredia"), ("el", "Επαρχία Ερέδια"), ("en", "Heredia"), ("es", "Heredia"), ("eu", "Heredia (probintzia)"), ("fa", "استان اردیا"), ("fi", "Heredian maakunta"), ("fr", "Heredia"), ("he", "ארדיה"), ("hr", "Heredia, provincija"), ("hu", "Heredia tartomány"), ("hy", "Էրեդիա"), ("id", "Provinsi Heredia"), ("it", "provincia di Heredia"), ("ja", "エレディア州"), ("ka", "ერედიის პროვინცია"), ("ko", "에레디아 주"), ("lt", "Heredijos provincija"), ("nb", "Heredia"), ("nl", "Heredia"), ("no", "Heredia"), ("pl", "Heredia"), ("pt", "Heredia"), ("ro", "Provincia Heredia"), ("ru", "Эредия"), ("sr", "Ередија"), ("sr_Latn", "Eredija"), ("sv", "Heredia"), ("tr", "Heredia ili"), ("uk", "Ередія"), ("ur", "ایریدیا صوبہ"), ("vi", "Heredia"), ("zh", "埃雷迪亚省")]),
                        unofficial_name_list: ["Heredia"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "Limón",
                        country_alpha2: Alpha2::CR,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.983333), longitude: Some(-83.033333), max_latitude: Some(10.0114533), min_latitude: Some(9.9721473), max_longitude: Some(-83.0199856), min_longitude: Some(-83.06642529999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ليمون"), ("bg", "Лимон"), ("ca", "Província de Limón"), ("ccp", "𑄣\u{11128}𑄟\u{11127}𑄚\u{11134}"), ("ceb", "Provincia de Limón"), ("cs", "Limón"), ("da", "Limón"), ("de", "Provinz Limón"), ("el", "Επαρχία Λιμόν"), ("en", "Limón"), ("es", "Limón"), ("fa", "استان لیمون"), ("fi", "Limónin maakunta"), ("fr", "Limón"), ("he", "לימון"), ("hr", "Limón, provincija"), ("hy", "Լիմոն"), ("id", "Provinsi Limón"), ("it", "provincia di Limón"), ("ja", "リモン州"), ("ka", "ლიმონის პროვინცია"), ("ko", "리몬 주"), ("lt", "Limono provincija"), ("nb", "Limón"), ("nl", "Limón"), ("no", "Limón"), ("pl", "Limón"), ("pt", "Limón"), ("ru", "Лимон"), ("sr", "Лимон"), ("sr_Latn", "Limon"), ("sv", "Limón"), ("tr", "Limón ili"), ("uk", "Лимон"), ("ur", "لیمون صوبہ"), ("vi", "Limón"), ("zh", "利蒙省")]),
                        unofficial_name_list: ["Limón"].to_vec(),
                    }
                ),
                (
                    "P",
                    Subdivision{
                        name: "Puntarenas",
                        country_alpha2: Alpha2::CR,
                        code: "P",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.9778439), longitude: Some(-84.82942109999999), max_latitude: Some(9.983031), min_latitude: Some(9.9740775), max_longitude: Some(-84.7929956), min_longitude: Some(-84.8512676)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بونتاريناس"), ("bg", "Пунтаренас"), ("bn", "প\u{9c1}ন\u{9cd}ট\u{9be}রেন\u{9be}স প\u{9cd}রদেশ"), ("ca", "Província de Puntarenas"), ("ccp", "𑄛\u{1112a}𑄚\u{11134}𑄑𑄢𑄬𑄚𑄌\u{11134}"), ("ceb", "Provincia de Puntarenas"), ("cs", "Puntarenas"), ("da", "Puntarenas"), ("de", "Provinz Puntarenas"), ("el", "Επαρχία Πουνταρένας"), ("en", "Puntarenas"), ("es", "Puntarenas"), ("eu", "Puntarenas (probintzia)"), ("fa", "استان پونتارناس"), ("fi", "Puntarenasin maakunta"), ("fr", "Puntarenas"), ("gu", "પન\u{acd}ટાર\u{ac7}નાસ પ\u{acd}રા\u{a82}ત"), ("he", "פונטרנס"), ("hi", "प\u{902}टार\u{947}नस प\u{94d}रा\u{902}त"), ("hr", "Puntarenas, provincija"), ("hy", "Պունտարենաս"), ("id", "Provinsi Puntarenas"), ("it", "provincia di Puntarenas"), ("ja", "プンタレナス州"), ("ka", "პუნტარენასის პროვინცია"), ("kn", "ಪಂಟರ\u{cc6}ನಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "푼타레나스 주"), ("lt", "Puntarenaso provincija"), ("lv", "Puantarenasas province"), ("mr", "प\u{902}टार\u{947}नस प\u{94d}रा\u{902}त"), ("ms", "Puntarenas Province"), ("nb", "Puntarenas"), ("nl", "Puntarenas"), ("no", "Puntarenas"), ("pl", "Puntarenas"), ("pt", "Puntarenas"), ("ru", "Пунтаренас"), ("si", "ප\u{dd4}න\u{dca}ටරෙන\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Пунтаренас"), ("sr_Latn", "Puntarenas"), ("sv", "Puntarenas"), ("ta", "புன\u{bcd}டரேனஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పుంట\u{c3e}ర\u{c46}న\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e31}นตาเรนาส"), ("tr", "Puntaneras Province"), ("uk", "Пунтаренас"), ("ur", "پونتاریناس صوبہ"), ("vi", "Puntarenas"), ("zh", "蓬塔雷纳斯省")]),
                        unofficial_name_list: ["Puntarenas"].to_vec(),
                    }
                ),
                (
                    "SJ",
                    Subdivision{
                        name: "San José",
                        country_alpha2: Alpha2::CR,
                        code: "SJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.9280694), longitude: Some(-84.0907246), max_latitude: Some(9.9712598), min_latitude: Some(9.899968699999999), max_longitude: Some(-84.047191), min_longitude: Some(-84.1794991)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سان خوسيه"), ("be", "правінцыя Сан-Хасэ"), ("bg", "Сан Хосе"), ("bn", "স\u{9cd}য\u{9be}\u{9be}ন জোসে প\u{9cd}রদেশ"), ("ca", "província de San José"), ("ccp", "𑄥𑄚\u{11134} 𑄎\u{11127}𑄥𑄬"), ("ceb", "Provincia de San José"), ("cs", "San José"), ("da", "San José"), ("de", "Provinz San José"), ("el", "Επαρχία Σαν Χοσέ"), ("en", "San José"), ("es", "Provincia de San José"), ("eu", "San Jose (probintzia)"), ("fa", "استان سن خوزه"), ("fi", "San Josén maakunta"), ("fr", "San José"), ("gu", "સ\u{ac7}ન જોસ પ\u{acd}રા\u{a82}ત"), ("he", "סן חוסה"), ("hi", "स\u{948}न जोस प\u{94d}रा\u{902}त"), ("hr", "San José, provincija"), ("hy", "Սան Խոսե"), ("id", "Provinsi San José"), ("it", "provincia di San José"), ("ja", "サンホセ州"), ("ka", "სან-ხოსეს პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಜೋಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산호세 주"), ("lt", "San Chosė provincija"), ("lv", "Sanhosē province"), ("mr", "स\u{945}न जोस प\u{94d}रा\u{902}त"), ("ms", "San Jose Province"), ("nb", "San José"), ("nl", "San José"), ("no", "San José"), ("pl", "San José"), ("pt", "San José"), ("ru", "Сан-Хосе"), ("si", "සැන\u{dca} ජොසේ පළ\u{dcf}ත"), ("sr", "Сан Хосе"), ("sr_Latn", "San Hose"), ("sv", "San José"), ("ta", "ச\u{bbe}ன\u{bcd} ஜோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}న\u{c4d} జ\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานโฮเซ\u{e48}"), ("tr", "San soze Province"), ("uk", "Сан-Хосе"), ("ur", "سان خوزے صوبہ"), ("vi", "San José"), ("zh", "圣何塞省")]),
                        unofficial_name_list: ["San José"].to_vec(),
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
#[cfg(feature = "cr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CR,
        alpha3: Alpha3::CRI,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 506,
        currency_code: CurrencyCode::CRC,
        gec: Some(GEC::CS),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CRC),
        iso_long_name: "The Republic of Costa Rica",
        iso_short_name: "Costa Rica",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Costa Rican"),
        number: "188",
        postal_code: true,
        postal_code_format: Some("\\d{4,5}|\\d{3}-\\d{4}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "CR",
        unofficial_name_list: ["Costa Rica", "コスタリカ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Costa Rica"),
            ("af", "Costa Rica"),
            ("ak", "Costa Rica"),
            ("am", "Costa Rica"),
            ("an", "Costa Rica"),
            ("ar", "كوستاريكا"),
            ("as", "কোস\u{9cd}ট\u{9be} ৰিক\u{9be}"),
            ("ay", "Costa Rica"),
            ("az", "Kosta Rika"),
            ("ba", "Costa Rica"),
            ("be", "Коста-Рыка"),
            ("bg", "Коста Рика"),
            ("bi", "Costa Rica"),
            ("bn", "কোস\u{9cd}ট\u{9be} রিক\u{9be}"),
            ("bn_IN", "কোস\u{9cd}ট\u{9be} রিক\u{9be}"),
            ("br", "Costa Rica"),
            ("bs", "Kostarika"),
            ("ca", "Costa Rica"),
            ("ce", "Коста-Рика"),
            ("ch", "Costa Rica"),
            ("cs", "Kostarika"),
            ("cv", "Коста-Рика"),
            ("cy", "Costa Rica"),
            ("da", "Costa Rica"),
            ("de", "Costa Rica"),
            ("dv", "ކ\u{7ae}ސ\u{7b0}ޓ\u{7a6}ރ\u{7a9}ކ\u{7a7}"),
            ("dz", "ཀ\u{f71}\u{f7c}ས\u{f72}་ཊ་ར\u{f72}་ཀ།"),
            ("ee", "Costa Rica"),
            ("el", "Κόστα Ρίκα"),
            ("en", "Costa Rica"),
            ("eo", "Kostariko"),
            ("es", "Costa Rica"),
            ("et", "Costa Rica"),
            ("eu", "Costa Rica"),
            ("fa", "کاستاریکا"),
            ("ff", "Costa Rica"),
            ("fi", "Costa Rica"),
            ("fo", "Kosta Rika"),
            ("fr", "Costa Rica"),
            ("fy", "Kosta Rika"),
            ("ga", "Cósta Rice"),
            ("gl", "Costa Rica"),
            ("gn", "Costa Rica"),
            ("gu", "કોસ\u{acd}ટા રીકા"),
            ("gv", "Yn Coose Berçhagh"),
            ("ha", "Costa Rica"),
            ("he", "קוסטה ריקה"),
            ("hi", "कोस\u{94d}टा रीका"),
            ("hr", "Kostarika"),
            ("ht", "Kostarika"),
            ("hu", "Costa Rica"),
            ("hy", "Կոստա-Ռիկա"),
            ("ia", "Costa Rica"),
            ("id", "Kosta Rika"),
            ("io", "Kosta Rika"),
            ("is", "Kosta Ríka"),
            ("it", "Costa Rica"),
            ("iu", "Costa Rica"),
            ("ja", "コスタリカ"),
            ("ka", "კოსტა-რიკა"),
            ("ki", "Costa Rica"),
            ("kk", "Коста-Рика"),
            ("kl", "Costa Rica"),
            ("km", "ក\u{17bc}ស\u{17d2}តារ\u{17b8}កា"),
            ("kn", "ಕೋಸ\u{ccd}ಟಾರ\u{cbf}ಕಾ"),
            ("ko", "코스타리카"),
            ("ku", "Kosta Rîka"),
            ("kv", "Коста-Рика"),
            ("kw", "Kosta Rika"),
            ("ky", "Коста-Рика"),
            ("lo", "ປະເທດກ\u{ebb}ດສະຕາລ\u{eb4}ກາ"),
            ("lt", "Kosta Rika"),
            ("lv", "Kostarika"),
            ("mi", "Costa Rica"),
            ("mk", "Костарика"),
            ("ml", "കോസ\u{d4d}റ\u{d4d}റ റിക"),
            ("mn", "Коста рика"),
            ("mr", "कोस\u{94d}टारिका"),
            ("ms", "Costa Rica"),
            ("mt", "Kosta Rika"),
            (
                "my",
                "ကော\u{1037}စတာရ\u{102e}ကာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Kosta Rika"),
            ("nb", "Costa Rica"),
            ("ne", "कोस\u{94d}टारिका"),
            ("nl", "Costa Rica"),
            ("nn", "Costa Rica"),
            ("nv", "Costa Rica"),
            ("oc", "Còsta Rica"),
            ("or", "କୋଷ\u{b4d}ଟ\u{b3e} ର\u{b3f}କ\u{b3e}"),
            ("pa", "ਕਾਸਟ ਰੀਕਾ"),
            ("pi", "कोस\u{94d}टा रीका"),
            ("pl", "Kostaryka"),
            ("ps", "کوسټاریکا"),
            ("pt", "Costa Rica"),
            ("pt_BR", "Costa Rica"),
            ("ro", "Costa Rica"),
            ("ru", "Коста-Рика"),
            ("rw", "Kosita Rika"),
            ("sc", "Costa Rica"),
            ("sd", "Costa Rica"),
            ("si", "කොස\u{dca}ට\u{dcf} ර\u{dd3}ක\u{dcf}"),
            ("sk", "Kostarika"),
            ("sl", "Kostarika"),
            ("so", "Kosta Rika"),
            ("sq", "Kosta Rikë"),
            ("sr", "Костарика"),
            ("sv", "Costa Rica"),
            ("sw", "Costa Rica"),
            ("ta", "கோஸ\u{bcd}ட\u{bbe}ரிக\u{bbe}"),
            ("te", "క\u{c4b}స\u{c4d}ట\u{c3e} ర\u{c3f}క\u{c3e}"),
            ("tg", "Коста-Рика"),
            ("th", "คอสตาร\u{e34}กา"),
            ("ti", "ኮስታ ሪካ"),
            ("tk", "Kosta-Rika"),
            ("tl", "Costa Rica"),
            ("tr", "Kosta Rika"),
            ("tt", "Коста Рика"),
            ("ug", "كوستارىكا"),
            ("uk", "Коста-Рика"),
            ("ur", "کوسٹاریکا"),
            ("uz", "Kosta Rika"),
            ("ve", "Costa Rica"),
            ("vi", "Cốt-x-tha Ri-ca"),
            ("wa", "Costa Rica"),
            ("wo", "Kosta Riika"),
            ("xh", "Costa Rica"),
            ("yo", "Kóstá Rikà"),
            ("zh_CN", "哥斯达黎加"),
            ("zh_HK", "哥斯達黎加"),
            ("zh_TW", "哥斯大黎加"),
            ("zu", "Costa Rica"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
