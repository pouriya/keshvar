// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Iraq

#[cfg(all(feature = "iq", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::IQ;
    pub const ALPHA3: Alpha3 = Alpha3::IRQ;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 964;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::IQD;
    pub const GEC: Option<GEC> = Some(GEC::IZ);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::IRQ);
    pub const ISO_SHORT_NAME: &str = "Iraq";
    pub const ISO_LONG_NAME: &str = "The Republic of Iraq";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Iraqi");
    pub const NUMBER: &str = "368";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "IQ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Iraq", "العراق", "Irak", "イラク"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Iraq"),
        ("af", "Irak"),
        ("ak", "Iraq"),
        ("am", "ጒሲቅ"),
        ("an", "Iraq"),
        ("ar", "العراق"),
        ("as", "ইৰ\u{9be}ক"),
        ("ay", "Iraq"),
        ("az", "İraq"),
        ("ba", "Iraq"),
        ("be", "Ірак"),
        ("bg", "Ирак"),
        ("bi", "Iraq"),
        ("bn", "ইর\u{9be}ক"),
        ("bn_IN", "ইর\u{9be}ক"),
        ("br", "Irak"),
        ("bs", "Irak"),
        ("ca", "Iraq"),
        ("ce", "Ӏиракъ"),
        ("ch", "Iraq"),
        ("cs", "Irák"),
        ("cv", "Ӏиракъ"),
        ("cy", "Irac"),
        ("da", "Irak"),
        ("de", "Irak"),
        ("dv", "ޢ\u{7a8}ރ\u{7a7}ޤ\u{7aa}"),
        ("dz", "ཨ\u{f72}་རཀ།"),
        ("ee", "Iraq"),
        ("el", "Ιράκ"),
        ("en", "Iraq"),
        ("eo", "Irako"),
        ("es", "Irak"),
        ("et", "Iraak"),
        ("eu", "Irak"),
        ("fa", "عراق"),
        ("ff", "Iraq"),
        ("fi", "Irak"),
        ("fo", "Irak"),
        ("fr", "Irak"),
        ("fy", "Irak"),
        ("ga", "An Iaráic"),
        ("gl", "Iraq"),
        ("gn", "Iraq"),
        ("gu", "ઇરાક"),
        ("gv", "Yn Earack"),
        ("ha", "Irak"),
        ("he", "עיראק"),
        ("hi", "इराक\u{93c}"),
        ("hr", "Irak"),
        ("ht", "Irak"),
        ("hu", "Irak"),
        ("hy", "Իրաք"),
        ("ia", "Irak"),
        ("id", "Irak"),
        ("io", "Irak"),
        ("is", "Írak"),
        ("it", "Iraq"),
        ("iu", "ᐃᕉᒃ"),
        ("ja", "イラク"),
        ("ka", "ერაყი"),
        ("ki", "Iraq"),
        ("kk", "Ирак"),
        ("kl", "Iraq"),
        ("km", "អ\u{17ca}\u{17b8}រ\u{17c9}ាក\u{17cb}"),
        ("kn", "ಇರಾಕ\u{ccd}"),
        ("ko", "이라크"),
        ("ku", "Iraq"),
        ("kv", "Ирак"),
        ("kw", "Irak"),
        ("ky", "Ирак"),
        ("lo", "Iraq"),
        ("lt", "Irakas"),
        ("lv", "Irāka"),
        ("mi", "Īrāki"),
        ("mk", "Ирак"),
        ("ml", "ഇറ\u{d3e}ഖ\u{d4d}"),
        ("mn", "Ирак"),
        ("mr", "इराक"),
        ("ms", "Iraq"),
        ("mt", "Iraq"),
        (
            "my",
            "အ\u{102e}ရတ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Irak"),
        ("nb", "Irak"),
        ("ne", "इराक"),
        ("nl", "Irak"),
        ("nn", "Irak"),
        ("nv", "Iraq"),
        ("oc", "Iraq"),
        ("or", "ଇର\u{b3e}କ"),
        ("pa", "ਇਰਾਕ"),
        ("pi", "ईराक"),
        ("pl", "Irak"),
        ("ps", "عراق"),
        ("pt", "Iraque"),
        ("pt_BR", "Iraque"),
        ("ro", "Irak"),
        ("ru", "Ирак"),
        ("rw", "Irake"),
        ("sc", "Iraq"),
        ("sd", "عراق"),
        ("si", "ඉර\u{dcf}කය"),
        ("sk", "Irak"),
        ("sl", "Irak"),
        ("so", "Ciraaq"),
        ("sq", "Irak"),
        ("sr", "Ирак"),
        ("sv", "Irak"),
        ("sw", "Iraq"),
        ("ta", "ஈர\u{bbe}க\u{bcd}"),
        ("te", "ఇర\u{c3e}క\u{c4d}"),
        ("tg", "Ироқ"),
        ("th", "อ\u{e34}ร\u{e31}ก"),
        ("ti", "ዒራቅ"),
        ("tk", "Irak"),
        ("tl", "Iraq"),
        ("tr", "Irak"),
        ("tt", "Гырак"),
        ("ug", "ئىراق"),
        ("uk", "Ірак"),
        ("ur", "عراق"),
        ("uz", "Iroq"),
        ("ve", "Iraq"),
        ("vi", "I-rắc"),
        ("wa", "Irak"),
        ("wo", "Iraak"),
        ("xh", "Iraq"),
        ("yo", "Irak"),
        ("zh_CN", "伊拉克"),
        ("zh_HK", "伊拉克"),
        ("zh_TW", "伊拉克"),
        ("zu", "I-Iraki"),
    ];
    #[cfg(all(feature = "iq", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 33.223191;
        pub const LONGITUDE: f64 = 43.679291;
        pub const MAX_LATITUDE: f64 = 37.380645;
        pub const MAX_LONGITUDE: f64 = 48.6350999;
        pub const MIN_LATITUDE: f64 = 29.0612079;
        pub const MIN_LONGITUDE: f64 = 38.7936741;
        pub const NORTHEAST_LATITUDE: f64 = 37.380645;
        pub const NORTHEAST_LONGITUDE: f64 = 48.6350999;
        pub const SOUTHWEST_LATITUDE: f64 = 29.0612079;
        pub const SOUTHWEST_LONGITUDE: f64 = 38.7936741;
    }
}
#[cfg(all(feature = "iq", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 33.223191,
            longitude: 43.679291,
            max_latitude: 37.380645,
            max_longitude: 48.6350999,
            min_latitude: 29.0612079,
            min_longitude: 38.7936741,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 37.380645,
                    longitude: 48.6350999,
                },
                southwest: CountryGeoBound {
                    latitude: 29.0612079,
                    longitude: 38.7936741,
                },
            },
        }
    }
}

#[cfg(all(feature = "iq", feature = "subdivisions"))]
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
                    "AN",
                    Subdivision{
                        name: "Al Anbar",
                        country_alpha2: Alpha2::IQ,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.5597614), longitude: Some(41.9196471), max_latitude: Some(35.1012211), min_latitude: Some(30.5896601), max_longitude: Some(44.307257), min_longitude: Some(38.7958871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الأنبار"), ("az", "Əl-Anbar"), ("be", "Мухафаза Анбар"), ("bg", "Ал Анбар"), ("bn", "আল আনব\u{9be}র প\u{9cd}রদেশ"), ("ca", "Governació d’Al-Anbar"), ("ccp", "𑄃𑄣\u{11134} 𑄃𑄚\u{11134}𑄝𑄢\u{11134}"), ("cs", "Anbár"), ("cy", "Al Anbar"), ("da", "Al Anbar"), ("de", "al-Anbar"), ("el", "Κυβερνείο Ανμπάρ"), ("en", "Al Anbar"), ("es", "Gobernación de Ambar"), ("et", "Al-Anbār"), ("eu", "Anbar probintzia"), ("fa", "استان انبار"), ("fi", "Al-Anbar"), ("fr", "Al-Anbar"), ("gu", "અલ અનબાર ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-אנבר"), ("hi", "अनबार प\u{94d}रान\u{94d}त"), ("hu", "Anbár kormányzóság"), ("id", "Kegubernuran Al Anbar"), ("it", "Governatorato di al-Anbar"), ("ja", "アンバール県"), ("ka", "ანბარის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಅನಬರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "안바르 주"), ("lt", "Al Ambar gubernija"), ("lv", "Anbāras muhāfaza"), ("ml", "അൻബ\u{d3e}ർ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "अल अनबर गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Al Anbar"), ("nb", "Anbar"), ("nl", "Al-Anbar"), ("no", "Anbar"), ("pl", "Al-Anbar"), ("pt", "Al-Anbar"), ("ro", "Al-Anbar"), ("ru", "Анбар"), ("si", "අල\u{dca} අන\u{dca}බ\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sk", "Al-Anbár"), ("sv", "al-Anbar"), ("ta", "அல\u{bcd} அன\u{bcd}ப\u{bbe}ர\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} అంబర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ลอ\u{e31}นบาร\u{e4c}"), ("tr", "Anbar ili"), ("uk", "Анбар"), ("ur", "محافظہ الانبار"), ("vi", "Tỉnh Al Anbar"), ("zh", "安巴尔省")]),
                        unofficial_name_list: ["al-Anbar"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "Arbil",
                        country_alpha2: Alpha2::IQ,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.191111), longitude: Some(44.009167), max_latitude: Some(36.2661445), min_latitude: Some(36.1284471), max_longitude: Some(44.101181), min_longitude: Some(43.91613)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أربيل"), ("az", "Ərbil"), ("bg", "Арбил"), ("bn", "আর\u{9cd}বিল প\u{9cd}রদেশ"), ("ca", "Governació d’Erbil"), ("ccp", "𑄃\u{11128}𑄢\u{11134}𑄝\u{11128}𑄣\u{11134}"), ("cs", "Arbíl"), ("cy", "Arbil"), ("da", "Arbil"), ("de", "Arbil"), ("el", "Έρμπιλ"), ("en", "Erbil"), ("es", "Gobernación de Erbil"), ("et", "Arbīli kubernerkond"), ("eu", "Arbil probintzia"), ("fa", "استان اربیل"), ("fi", "Erbil"), ("fr", "Arbil"), ("gu", "એર\u{acd}બિલ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז ארביל"), ("hi", "अरबील प\u{94d}रान\u{94d}त"), ("hu", "Erbíl kormányzóság"), ("id", "Kegubernuran Arbil"), ("it", "Governatorato di Erbil"), ("ja", "アルビール県"), ("ka", "ერბილის მუჰაფაზა"), ("kn", "ಎರ\u{ccd}ಬ\u{cbf}ಲ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "아르빌 주"), ("lt", "Erbilo gubernija"), ("lv", "Erbīlas muhāfaza"), ("mk", "Ербил"), ("mr", "अर\u{94d}बिल प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Arbil"), ("nb", "Arbil"), ("nl", "Arbil"), ("no", "Arbil"), ("pl", "Irbil"), ("pt", "Arbil"), ("ro", "Arbil"), ("ru", "Эрбиль"), ("si", "එර\u{dca}බ\u{dd2}ල\u{dca} පළ\u{dcf}ත"), ("sk", "Arbíl"), ("sr", "Гувернорат Ербил"), ("sr_Latn", "Guvernorat Erbil"), ("sv", "Arbil"), ("ta", "எர\u{bcd}பில\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "ఎర\u{c4d}బ\u{c3f}ల\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เออร\u{e4c}บ\u{e34}ล"), ("tr", "Erbil ili"), ("uk", "Ербіль"), ("ur", "محافظہ اربیل"), ("vi", "Tỉnh Erbil"), ("zh", "埃爾比勒省")]),
                        unofficial_name_list: ["Arbil", "Erbil", "Irbil"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "Al Basrah",
                        country_alpha2: Alpha2::IQ,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.5), longitude: Some(47.816667), max_latitude: Some(30.6321515), min_latitude: Some(30.4303216), max_longitude: Some(47.9299164), min_longitude: Some(47.6141454)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة البصرة"), ("az", "Bəsrə"), ("be", "Басра"), ("bg", "Басра"), ("bn", "ব\u{9be}সর\u{9be}হ প\u{9cd}রদেশ"), ("ca", "Governació de Basra"), ("ccp", "𑄝𑄌\u{11134}𑄢"), ("cs", "Basra"), ("cy", "Basra"), ("da", "Al Basrah"), ("de", "Basra"), ("el", "Μπάσρα Γκοβερνοράτε"), ("en", "Basra"), ("es", "Gobernación de Basora"), ("et", "Al-Başrah’ kubernerkond"), ("eu", "Basorako probintzia"), ("fa", "استان بصره"), ("fi", "Basra"), ("fr", "Al-Basra"), ("gu", "બસરા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז בצרה"), ("hi", "बसरा प\u{94d}रान\u{94d}त"), ("hu", "Baszra kormányzóság"), ("id", "Kegubernuran Basra"), ("it", "Governatorato di Bassora"), ("ja", "バスラ県"), ("ka", "ბასრის მუჰაფაზა"), ("kn", "ಬಸ\u{ccd}ರಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "바스라 주"), ("lt", "Basra"), ("lv", "Basra"), ("mk", "Басра"), ("mr", "बसरा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Basrah"), ("nb", "Basra"), ("nl", "Basra"), ("no", "Basra"), ("pl", "Basra"), ("pt", "Baçorá"), ("ro", "Al-Basra"), ("ru", "Басра"), ("si", "බස\u{dca}ර\u{dcf} ගවනොරෙට\u{dca}"), ("sk", "Al-Basra"), ("sv", "Basra"), ("ta", "ப\u{bbe}ஸ\u{bcd}ர\u{bbe} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "బస\u{c4d}ర\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "บาสลา อ\u{e34}ล"), ("tr", "Basra ili"), ("uk", "Басра"), ("ur", "محافظہ بصرہ"), ("vi", "Tỉnh Basra"), ("zh", "巴士拉省")]),
                        unofficial_name_list: ["Basra", "Bassora", "al-Basrah"].to_vec(),
                    }
                ),
                (
                    "BB",
                    Subdivision{
                        name: "Babil",
                        country_alpha2: Alpha2::IQ,
                        code: "BB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.468191), longitude: Some(44.5501935), max_latitude: Some(33.233322), min_latitude: Some(32.0787799), max_longitude: Some(45.214476), min_longitude: Some(43.849441)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بابل"), ("az", "Babil"), ("be", "Бабіль"), ("bg", "Бабил"), ("bn", "ব\u{9be}বিল প\u{9cd}রদেশ"), ("ca", "Governació de Babil"), ("ccp", "𑄝𑄬𑄝\u{11128}𑄣\u{11127}𑄚\u{11134}"), ("cs", "Babylón"), ("cy", "Bābil"), ("da", "Babil"), ("de", "Babil"), ("el", "Μπάμπιλον Γκοβερνοράτε"), ("en", "Babylon"), ("es", "Babilonia"), ("et", "Bābil"), ("eu", "Babilonia probintzia"), ("fa", "استان بابل"), ("fi", "Babil"), ("fr", "Babil"), ("gu", "બ\u{ac7}બીલોન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז באבל"), ("hi", "बाबिल प\u{94d}रान\u{94d}त"), ("hu", "Bábil kormányzóság"), ("id", "Kegubernuran Babil"), ("it", "Governatorato di Babil"), ("ja", "バービル県"), ("ka", "ბაბილის მუჰაფაზა"), ("kn", "ಬ\u{ccd}ಯಾಬ\u{cbf}ಲೋನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "바빌 주"), ("lt", "Babilono gubernija"), ("lv", "Bābilas muhāfaza"), ("mr", "ब\u{945}बिलो गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Babil"), ("nb", "Babil"), ("nl", "Babil"), ("no", "Babil"), ("pl", "Babil"), ("pt", "Babil"), ("ro", "Babil"), ("ru", "Бабиль"), ("si", "බෙබ\u{dd2}ලෝන\u{dca} පළ\u{dcf}ත"), ("sv", "Babil"), ("ta", "ப\u{bbe}பிலோன\u{bcd} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "బ\u{c3e}బ\u{c3f}ల\u{c4b}న\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองบาบ\u{e34}ลอน"), ("tr", "Babil ili"), ("uk", "Бабіль"), ("ur", "محافظہ بابل"), ("vi", "Tỉnh Babylon"), ("zh", "巴比倫省")]),
                        unofficial_name_list: ["Babil", "Babylon"].to_vec(),
                    }
                ),
                (
                    "BG",
                    Subdivision{
                        name: "Baghdad",
                        country_alpha2: Alpha2::IQ,
                        code: "BG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.325), longitude: Some(44.422), max_latitude: Some(33.4350586), min_latitude: Some(33.1883135), max_longitude: Some(44.5558261), min_longitude: Some(44.1559839)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بغداد"), ("az", "Bağdad"), ("be", "Багдад"), ("bg", "Багдад"), ("bn", "ব\u{9be}গদ\u{9be}দ প\u{9cd}রদেশ"), ("ca", "Governació de Bagdad"), ("ccp", "𑄝𑄇\u{11134}𑄘𑄖\u{11134}"), ("cs", "Bagdád"), ("da", "Bagdad"), ("de", "Bagdad"), ("en", "Baghdad"), ("es", "Gobernación de Bagdad"), ("et", "Bagdadi kubernerkond"), ("eu", "Bagdad probintzia"), ("fa", "استان بغداد"), ("fi", "Bagdad"), ("fr", "Bagdad"), ("gl", "Provincia de Bagdad"), ("he", "מחוז בגדאד"), ("hi", "बग\u{93c}दाद प\u{94d}रान\u{94d}त"), ("hu", "Bagdad kormányzóság"), ("id", "Kegubernuran Bagdad"), ("it", "Governatorato di Baghdad"), ("ja", "バグダード県"), ("ka", "ბაღდადის მუჰაფაზა"), ("ko", "바그다드 주"), ("ms", "Pentadbiran Baghdad"), ("nb", "Bagdad"), ("nl", "Bagdad"), ("no", "Bagdad"), ("pa", "ਬਗ\u{a3c}ਦਾਦ ਪ\u{a4d}ਰਾ\u{a02}ਤ"), ("pl", "Bagdad"), ("pt", "Bagdad"), ("ro", "Bagdad"), ("ru", "Багдад"), ("sk", "Bagdad"), ("sv", "Guvernementet Bagdad"), ("tr", "Bağdat ili"), ("uk", "Багдад"), ("ur", "محافظہ بغداد"), ("zh", "巴格達省")]),
                        unofficial_name_list: ["Bagdad", "Bagdad", "Baġdād"].to_vec(),
                    }
                ),
                (
                    "DA",
                    Subdivision{
                        name: "Dahuk",
                        country_alpha2: Alpha2::IQ,
                        code: "DA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.133389), longitude: Some(43.1309888), max_latitude: Some(37.3780401), min_latitude: Some(36.696166), max_longitude: Some(44.110214), min_longitude: Some(42.3622129)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دهوك"), ("az", "Dəhuk"), ("bg", "Дахук"), ("bn", "দহ\u{9c1}ক প\u{9cd}রদেশ"), ("ca", "Governació de Dohuk"), ("ccp", "𑄘\u{1112e}𑄦\u{1112a}𑄇\u{11134}"), ("ceb", "Dihok"), ("cs", "Dahúk"), ("da", "Dahuk"), ("de", "Gouvernement Dahuk"), ("el", "Ντοχούκ"), ("en", "Dohuk"), ("es", "Duhok"), ("et", "Dahūki kubernerkond"), ("eu", "Dohukeko probintzia"), ("fa", "استان دهوک"), ("fi", "Dahuk"), ("fr", "Dahuk"), ("gu", "દોહ\u{ac1}ક ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "דהוכ"), ("hi", "दोह\u{942}क प\u{94d}रान\u{94d}त"), ("hu", "Dahúk kormányzóság"), ("id", "Kegubernuran Dahuk"), ("it", "Governatorato di Dahuk"), ("ja", "ドホーク県"), ("jv", "Kagubernuran Dohuk"), ("ka", "დაჰუკის მუჰაფაზა"), ("kk", "Дахук"), ("kn", "ಡೊಹಕ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다후크 주"), ("lt", "Dobuko gubernija"), ("lv", "Dahūkas muhāfaza"), ("mr", "दोह\u{941}क गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Dahuk"), ("nb", "Dahuk"), ("nl", "Duhok"), ("no", "Dahuk"), ("pl", "Dahuk"), ("pt", "Dahuk"), ("ro", "Dahuk"), ("ru", "Дахук"), ("si", "ඩොහ\u{dd4}ක\u{dca} පළ\u{dcf}ත"), ("sr", "Гувернорат Дахук"), ("sr_Latn", "Guvernorat Dahuk"), ("sv", "Dahuk"), ("ta", "டொஹுக\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "డ\u{c4b}హుక\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เทศบาลเม\u{e37}องเคน"), ("tr", "Duhok ili"), ("uk", "Дахук"), ("ur", "محافظہ دھوک"), ("vi", "Tỉnh Dohuk"), ("zh", "杜胡克省")]),
                        unofficial_name_list: ["Dahuk"].to_vec(),
                    }
                ),
                (
                    "DI",
                    Subdivision{
                        name: "Diyalá",
                        country_alpha2: Alpha2::IQ,
                        code: "DI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.7733487), longitude: Some(45.1494505), max_latitude: Some(35.1153911), min_latitude: Some(33.006982), max_longitude: Some(46.01852), min_longitude: Some(44.292854)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ديالى"), ("az", "Diyalə"), ("bg", "Дияла"), ("bn", "দিয\u{9bc}\u{9be}ল\u{9be} প\u{9cd}রদেশ"), ("ca", "Governació de Diyala"), ("ccp", "𑄘\u{11128}𑄠𑄣"), ("ceb", "Diyālá"), ("cs", "Dijála"), ("da", "Diyala"), ("de", "Governorat Diyala"), ("el", "Ντιγιάλα"), ("en", "Diyala"), ("es", "Gobernación de Diala"), ("et", "Diyālá"), ("eu", "Dijala probintzia"), ("fa", "استان دیاله"), ("fi", "Diyala"), ("fr", "Diyala"), ("gl", "Provincia de Diala"), ("gu", "દિયાલા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "דיאלא"), ("hi", "दियाला प\u{94d}रान\u{94d}त"), ("hu", "Dijála kormányzóság"), ("id", "Kegubernuran Diyala"), ("it", "Governatorato di Diyala"), ("ja", "ディヤーラー県"), ("ka", "დიიალის მუჰაფაზა"), ("kn", "ಡ\u{cbf}ಯಾಲಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "디얄라 주"), ("lt", "Dijalos gubernija"), ("lv", "Dijālas muhāfaza"), ("mr", "दीयाल गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Diyala Governorate"), ("nb", "Diyala"), ("nl", "Diyala"), ("no", "Diyala"), ("pl", "Dijala"), ("pt", "Diyala"), ("ru", "Дияла"), ("si", "ද\u{dd2}යල\u{dcf} පළ\u{dcf}ත"), ("sk", "Dijálá"), ("sv", "Diyala"), ("te", "డ\u{c3f}య\u{c3e}ల\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ด\u{e34}ยาลา"), ("tr", "Diyala ili"), ("uk", "Діяла"), ("ur", "محافظہ دیالی"), ("vi", "Tỉnh Diyala"), ("zh", "迪亞拉省")]),
                        unofficial_name_list: ["Diyala"].to_vec(),
                    }
                ),
                (
                    "DQ",
                    Subdivision{
                        name: "Dhi Qar",
                        country_alpha2: Alpha2::IQ,
                        code: "DQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.1042292), longitude: Some(46.3624686), max_latitude: Some(31.9975451), min_latitude: Some(30.564833), max_longitude: Some(47.1987489), min_longitude: Some(45.643673)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ذي قار"), ("az", "Ziqar"), ("bg", "Ди Кар"), ("bn", "জি ক\u{9be}র প\u{9cd}রদেশ"), ("ccp", "𑄙\u{11128} 𑄇\u{1112e}𑄠𑄢\u{11134}"), ("cs", "Dhíkár"), ("da", "Dhi Qar"), ("de", "Dhi Qar"), ("el", "Ντι Καρ"), ("en", "Dhi Qar"), ("es", "Gobernación de Di Car"), ("et", "Dhī Qār"), ("eu", "Dhi Qarreko probintzia"), ("fa", "استان ذی\u{200c}قار"), ("fi", "Dhi Qar"), ("fr", "Dhi Qar"), ("gu", "ધી કાર ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "ד׳י קאר"), ("hi", "ज\u{93c}ी क\u{93c}ार प\u{94d}रान\u{94d}त"), ("hu", "Dzi Kár kormányzóság"), ("id", "Kegubernuran Dhi Qar"), ("it", "Governatorato di Dhi Qar"), ("ja", "ジーカール県"), ("ka", "დი-ქარის მუჰაფაზა"), ("kn", "ಧ\u{cbf} ಖಾರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "디카르 주"), ("lt", "Di Karo gubernija"), ("lv", "Dīkāras muhāfaza"), ("mr", "धी कार गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Dhi Qar"), ("nb", "Dhi Qar"), ("nl", "Dhi Qar"), ("no", "Dhi Qar"), ("pl", "Zi Kar"), ("pt", "Dhi Qar"), ("ru", "Ди-Кар"), ("si", "ධ\u{dd2}ක\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sr", "Ди Кар"), ("sr_Latn", "Di Kar"), ("sv", "Dhi Qar"), ("ta", "தி கியர\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ఢ\u{c3f} ఖ\u{c3e}ర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองด\u{e34}การ\u{e4c}"), ("tr", "Zi Kar İli"), ("uk", "Ді-Кар"), ("ur", "محافظہ ذی قار"), ("vi", "Tỉnh Dhi Qar"), ("zh", "濟加爾省")]),
                        unofficial_name_list: ["Dhi Qar", "Thi Qar"].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "Karbala'",
                        country_alpha2: Alpha2::IQ,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.616667), longitude: Some(44.033333), max_latitude: Some(32.6572976), min_latitude: Some(32.5575024), max_longitude: Some(44.0673636), min_longitude: Some(43.9558696)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كربلاء"), ("az", "Kərbəla"), ("bg", "Кербала"), ("bn", "ক\u{9be}রব\u{9be}ল\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄇𑄢\u{11134}𑄝𑄣"), ("cs", "Karbalá"), ("da", "Karbala"), ("de", "Karbala"), ("el", "Κάρμπαλα"), ("en", "Karbala"), ("es", "Gobernación de Kerbala"), ("et", "Karbalā’ kubernerkond"), ("eu", "Karbalako probintzia"), ("fa", "استان کربلا"), ("fi", "Karbalan maakunta"), ("fr", "Karbala"), ("gu", "કરબલા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז כרבלא"), ("hi", "करबला प\u{94d}रान\u{94d}त"), ("hu", "Kerbela kormányzóság"), ("id", "Kegubernuran Karbala"), ("it", "Governatorato di Karbala"), ("ja", "カルバラー県"), ("ka", "ქარბალის მუჰაფაზა"), ("kn", "ಕರ\u{ccd}ಬಲಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "카르발라 주"), ("lt", "Kerbelos gubernija"), ("lv", "Kerbelas muhāfaza"), ("mr", "कार\u{94d}बाला गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Karbala"), ("nb", "Karbala"), ("nl", "Karbala"), ("no", "Karbala"), ("pl", "Karbala"), ("pt", "Karbala"), ("ru", "Кербела"), ("si", "කර\u{dca}බල\u{dcf} පළ\u{dcf}ත"), ("sv", "Karbala"), ("ta", "கர\u{bcd}பல\u{bbe} கோவெர\u{bcd}னோரே"), ("te", "క\u{c3e}ర\u{c4d}బ\u{c3e}ల\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "การ\u{e4c}บาลา"), ("tr", "Kerbela ili"), ("uk", "Кербела"), ("ur", "محافظہ کربلا"), ("vi", "Tỉnh Karbala"), ("zh", "卡爾巴拉省")]),
                        unofficial_name_list: ["Karbala", "Kerbala", "Kerbela"].to_vec(),
                    }
                ),
                (
                    "KI",
                    Subdivision{
                        name: "كركوك",
                        country_alpha2: Alpha2::IQ,
                        code: "KI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كركوك"), ("az", "Kərkük"), ("bg", "Киркук"), ("bn", "কির\u{9cd}ক\u{9c1}ক প\u{9cd}রদেশ"), ("ccp", "𑄇\u{11128}𑄢\u{11134}𑄇\u{1112a}𑄇\u{11134}"), ("cs", "Kirkúk"), ("da", "Kirkuk"), ("de", "Kirkuk"), ("el", "Κίρκουκ Γκοβερνοράτε"), ("en", "Kirkuk"), ("es", "Gobernación de Kirkuk"), ("et", "Kirkūki kubernerkond"), ("eu", "At Ta’mim probintzia"), ("fa", "استان کرکوک"), ("fi", "Kirkukin maakunta"), ("fr", "Kirkuk"), ("gu", "કિર\u{acd}ક\u{ac1}ક ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "כרכוכ"), ("hi", "करक\u{942}क प\u{94d}रान\u{94d}त"), ("hu", "Kirkuk kormányzóság"), ("id", "Provinsi Kirkuk"), ("it", "Governatorato di Kirkuk"), ("ja", "キルクーク県"), ("ka", "კირკუკის მუჰაფაზა"), ("kn", "ಕ\u{cbf}ರ\u{ccd}ಕ\u{ccd}ಕುಕ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "키르쿠크 주"), ("lt", "Kirkuko gubernija"), ("lv", "Kirkuko muhāfaza"), ("mr", "किर\u{94d}क\u{941}क गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran At-Ta’mim"), ("nb", "Kirkuk"), ("nl", "Kirkoek"), ("no", "Kirkuk"), ("pl", "Kirkuk"), ("pt", "Kirkuk"), ("ru", "Киркук"), ("si", "කර\u{dca}කක\u{dca} පළ\u{dcf}ත"), ("sv", "Kirkuk"), ("ta", "கிர\u{bcd}குக\u{bcd} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "క\u{c3f}ర\u{c4d}కుక\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เคอค\u{e38}ก กอเวอโนเรท"), ("tr", "Kerkük ili"), ("uk", "Кіркук"), ("ur", "محافظہ کرکوک"), ("vi", "Tỉnh Kirkuk"), ("zh", "基爾庫克省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KR",
                    Subdivision{
                        name: "إقليم كردستان",
                        country_alpha2: Alpha2::IQ,
                        code: "KR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كردستان"), ("en", "Kurdistan"), ("ku", "ھەرێمی کوردستان")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "Maysan",
                        country_alpha2: Alpha2::IQ,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.8734002), longitude: Some(47.1362125), max_latitude: Some(32.84025200000001), min_latitude: Some(31.142853), max_longitude: Some(47.8643842), min_longitude: Some(46.30178309999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميسان"), ("az", "Meysan"), ("bg", "Майсан"), ("bn", "ম\u{9be}য\u{9bc}স\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Maysan"), ("ccp", "𑄟𑄬𑄥𑄚\u{11134}"), ("cs", "Majsán"), ("da", "Maysan"), ("de", "Maisan"), ("el", "Μέισαν"), ("en", "Maysan"), ("es", "Mesena"), ("et", "Maysān"), ("eu", "Maysango probintzia"), ("fa", "استان میسان"), ("fi", "Maysan"), ("fr", "Maysan"), ("gl", "Provincia de Maisan"), ("gu", "મ\u{ac7}સન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מיסאן"), ("hi", "मयसान प\u{94d}रान\u{94d}त"), ("hu", "Mejszán kormányzóság"), ("id", "Kegubernuran Maysan"), ("it", "Governatorato di Maysan"), ("ja", "マイサーン県"), ("ka", "მაისანის მუჰაფაზა"), ("kn", "ಮೇಸಾನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "마이산 주"), ("lt", "Maisano gubernija"), ("lv", "Meisānas muhāfaza"), ("mr", "म\u{947}सन प\u{94d}रशासकीय"), ("ms", "Pentadbiran Maysan"), ("nb", "Maysan"), ("nl", "Maysan"), ("no", "Maysan"), ("pl", "Majsan"), ("pt", "Maysan"), ("ru", "Майсан"), ("si", "මේසන\u{dca} පළ\u{dcf}ත"), ("sk", "Majsán"), ("sv", "Maysan"), ("ta", "மேசன\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "మ\u{c47}స\u{c3e}న\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองเมย\u{e4c}แซน"), ("tr", "Maysan ili"), ("uk", "Майсан"), ("ur", "محافظہ میسان"), ("vi", "Tỉnh Maysan"), ("zh", "米桑省")]),
                        unofficial_name_list: ["Maysan"].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "Al Muthanná",
                        country_alpha2: Alpha2::IQ,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.9133171), longitude: Some(45.29938620000001), max_latitude: Some(31.699816), min_latitude: Some(29.078844), max_longitude: Some(46.648625), min_longitude: Some(43.807403)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة المثنى"), ("az", "Əl-Müsənnə"), ("bg", "Ал Мутана"), ("bn", "আল ম\u{9c1}স\u{9be}ন\u{9cd}ন\u{9be} প\u{9cd}রদেশ"), ("ca", "Muthanna"), ("ccp", "𑄃𑄢\u{11134} 𑄟\u{1112a}𑄗𑄚\u{11134}"), ("cs", "Mutanná"), ("da", "Al Muthanna"), ("de", "al-Muthanna"), ("el", "Αλ Μουθάννα"), ("en", "Al Muthanna"), ("es", "Mutana"), ("et", "Al-Muthanná"), ("eu", "Al-Muthanna probintzia"), ("fa", "استان مثنی"), ("fi", "Al-Muthanna"), ("fr", "Al-Muthanna"), ("gu", "અલ મ\u{ac1}થાના ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-מות׳נא"), ("hi", "म\u{941}सन\u{94d}ना प\u{94d}रान\u{94d}त"), ("hu", "Muszanna kormányzóság"), ("id", "Kegubernuran Al Muthanna"), ("it", "Governatorato di al-Muthanna"), ("ja", "ムサンナー県"), ("ka", "მუთანის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಮುಥಣ\u{ccd}ಣ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "무탄나 주"), ("lt", "Al Mutanos gubernija"), ("lv", "Mutanna muhāfaza"), ("mr", "अल म\u{941}ता\u{902}बाना गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Al Muthanna"), ("nb", "Muthanna"), ("nl", "Al-Muthanna"), ("no", "Muthanna"), ("pl", "Al-Musanna"), ("pt", "Al-Muthanna"), ("ro", "Al-Muthanna"), ("ru", "Мутанна"), ("si", "අල\u{dca} ම\u{dd4}තන\u{dca}න\u{dcf} පළ\u{dcf}ත"), ("sv", "Al-Muthanna"), ("ta", "அல\u{bcd} முத\u{bcd}தண\u{bcd}ண\u{bbe} கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "అల\u{c4d} ముత\u{c3e}న\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลมาธ\u{e31}นนา"), ("tr", "Mutanna ili"), ("uk", "Мутанна"), ("ur", "محافظہ مثنی"), ("vi", "Tỉnh Al Muthanna"), ("zh", "穆薩納省")]),
                        unofficial_name_list: ["al-Muthanna"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "An Najaf",
                        country_alpha2: Alpha2::IQ,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.0), longitude: Some(44.33), max_latitude: Some(32.0761164), min_latitude: Some(31.9676002), max_longitude: Some(44.3853665), min_longitude: Some(44.2736149)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "النجف"), ("az", "Nəcəf"), ("bg", "Наджаф"), ("bn", "ন\u{9be}জ\u{9be}ফ প\u{9cd}রদেশ"), ("ca", "Governació de Najaf"), ("ccp", "𑄚𑄎𑄛\u{11134}"), ("ceb", "An Najaf"), ("cs", "Nadžaf"), ("da", "Najaf"), ("de", "Gouvernement Nadschaf"), ("el", "Νατζάφ"), ("en", "Najaf"), ("es", "Gobernación de Nayaf"), ("et", "An-Najafi kubernerkond"), ("eu", "Najafeko probintzia"), ("fa", "استان نجف"), ("fi", "Najaf (maakunta)"), ("fr", "An-Najaf"), ("gu", "નજફ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "א-נג׳ף"), ("hi", "नजफ\u{93c} प\u{94d}रान\u{94d}त"), ("hu", "Nedzsef kormányzóság"), ("id", "Kegubernuran Najaf"), ("it", "Governatorato di al-Najaf"), ("ja", "ナジャフ県"), ("ka", "ნაჯაფის მუჰაფაზა"), ("kn", "ನಜಾಫ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "나자프 주"), ("lt", "Nadžafo gubernija"), ("lv", "Nedžefas muhāfaza"), ("mr", "नजफ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Najaf"), ("nb", "Najaf"), ("nl", "An-Najaf"), ("no", "Najaf"), ("pl", "An-Nadżaf"), ("pt", "An-Najaf"), ("ro", "Al-Najaf"), ("ru", "Наджаф"), ("si", "නජ\u{dcf}ෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Наџаф"), ("sr_Latn", "Nadžaf"), ("sv", "Najaf"), ("ta", "நஜ\u{bbe}ப\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "న\u{c3e}జఫ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}นนาจาฟ"), ("tr", "Necef ili"), ("uk", "Наджаф"), ("ur", "محافظہ نجف"), ("vi", "Tỉnh Najaf"), ("zh", "納傑夫省")]),
                        unofficial_name_list: ["Najaf", "Nedjef", "Nejef"].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "Ninawá",
                        country_alpha2: Alpha2::IQ,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.229574), longitude: Some(42.2362435), max_latitude: Some(37.06718100000001), min_latitude: Some(34.8811661), max_longitude: Some(44.309753), min_longitude: Some(41.218105)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نينوى"), ("az", "Ninəvə"), ("be", "Найнава"), ("bg", "Нинава"), ("bn", "নিন\u{9be}ওয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Governació de Nínive"), ("ccp", "𑄚\u{1112d}𑄚𑄬𑄛\u{11134}𑄦\u{11134}"), ("cs", "Ninive"), ("da", "Ninawa"), ("de", "Ninawa"), ("el", "Ντουχόκ"), ("en", "Nineveh"), ("es", "Gobernación de Nínive"), ("et", "Nīnawá"), ("eu", "Ninawa"), ("fa", "استان نینوا"), ("fi", "Ninawa"), ("fr", "Ninawa"), ("gu", "ડ\u{ac2}હોક ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז נינוה"), ("hi", "नीनवा प\u{94d}रान\u{94d}त"), ("hu", "Ninive kormányzóság"), ("hy", "Նինվեի մարզ"), ("id", "Kegubernuran Ninawa"), ("it", "Governatorato di Ninawa"), ("ja", "ニーナワー県"), ("ka", "ნაინავის მუჰაფაზა"), ("kn", "ಡುಹೊಕ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "니나와 주"), ("lt", "Dahuko gubernija"), ("lv", "Dahūkas muhāfaza²"), ("mk", "Нинива"), ("mr", "द\u{941}होक गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Ninawa"), ("nb", "Ninawa"), ("nl", "Ninawa"), ("no", "Ninawa"), ("pl", "Niniwa"), ("pt", "Ninawa"), ("ru", "Найнава"), ("si", "ඩ\u{dd4}හොක\u{dca} පළ\u{dcf}ත"), ("sr", "Нинива"), ("sr_Latn", "Niniva"), ("sv", "Ninawa"), ("ta", "துஹ\u{bbe}க\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "డుహ\u{c4b}క\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "น\u{e34}นาวะ"), ("tr", "Nineve ili"), ("uk", "Найнава"), ("ur", "محافظہ نینوی"), ("vi", "Tỉnh Duhok"), ("zh", "尼尼微省")]),
                        unofficial_name_list: ["Nineveh", "Niniveh"].to_vec(),
                    }
                ),
                (
                    "QA",
                    Subdivision{
                        name: "Al Qadisiyah",
                        country_alpha2: Alpha2::IQ,
                        code: "QA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.043691), longitude: Some(45.1494505), max_latitude: Some(32.420094), min_latitude: Some(31.25445389999999), max_longitude: Some(45.82649199999999), min_longitude: Some(44.376614)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة القادسية"), ("az", "Əl-Qədisiyə"), ("bg", "Ал-Кадисия"), ("bn", "আল-ক\u{9be}দিসিয\u{9bc}\u{9cd}য\u{9be}হ প\u{9cd}রদেশ"), ("ca", "Governació d’Al-Qadisiyyah"), ("ccp", "𑄃𑄣\u{11134}-𑄇\u{1112e}𑄠𑄓\u{11128}𑄥\u{11128}𑄠𑄦\u{11134}"), ("cs", "Kádisíja"), ("da", "Al Qadisiyah"), ("de", "al-Qadisiyya"), ("el", "Αλ Καντισιγιά Γκοβερνοράτε"), ("en", "Al-Qādisiyyah"), ("es", "Cadisia"), ("et", "Al-Qādisīyah"), ("eu", "Al-Qadisiya probintzia"), ("fa", "استان قادسیه"), ("fi", "Qadisiyya"), ("fr", "Al-Qadisiyya"), ("gu", "અલ-કાદસીયાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-קאדסיה"), ("hu", "Kádiszijja kormányzóság"), ("id", "Kegubernuran Al-Qādisiyyah"), ("it", "Governatorato di al-Qadisiyya"), ("ja", "カーディーシーヤ県"), ("ka", "ქადისიის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd}-ಕದ\u{cbf}ಶ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "카디시야 주"), ("lt", "Al Kadisijos gubernija"), ("lv", "Kādisījas muhāfaza"), ("mr", "अल-कादानियाय गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Al-Qādisiyyah"), ("nb", "Al-Qadisiyya"), ("nl", "Al-Qadisiyah"), ("no", "Al-Qadisiyya"), ("pl", "Al-Kadisijja"), ("pt", "Al-Qadisiyyah"), ("ro", "Al-Qadisiyya"), ("ru", "Кадисия"), ("si", "අල\u{dca} කද\u{dd2}ස\u{dca}ස\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sv", "al-Qadisiyya"), ("ta", "அல\u{bcd} -கிடிஸிய\u{bcd}யஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d}-ఖ\u{c3e}డ\u{c3f}స\u{c3f}య\u{c4d}య\u{c3e}హ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ล คาด\u{e34}สอ\u{e34}ยยา โกเวอโนเรท"), ("tr", "Kadisiye ili"), ("uk", "Кадісія"), ("ur", "محافظہ قادسیہ"), ("vi", "Tỉnh Al-Qadisiyyah"), ("zh", "卡迪西亞省")]),
                        unofficial_name_list: ["al-Qadisiyah"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "Salah ad Din",
                        country_alpha2: Alpha2::IQ,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.5337527), longitude: Some(43.483738), max_latitude: Some(35.675014), min_latitude: Some(33.414859), max_longitude: Some(44.955274), min_longitude: Some(42.450488)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "صلاح الدين"), ("az", "Səlahəddin"), ("be", "правінцыя Салах-эд-Дзін"), ("bg", "Салах ад Дин"), ("bn", "স\u{9be}ল\u{9be}হউদ\u{9cd}দিন প\u{9cd}রদেশ"), ("ca", "Governació de Salah ad-Din"), ("ccp", "𑄥𑄣𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Muhafazat Salah ad Din"), ("cs", "Saladdín"), ("da", "Salah ad Din"), ("de", "Salah ad-Din"), ("el", "Κυβερνείο Σαλαντίν"), ("en", "Saladin"), ("es", "Gobernación de Saladino"), ("et", "Şalāḩ ad-Dīni kubernerkond"), ("eu", "Saladin probintzia"), ("fa", "استان صلاح\u{200c}الدین"), ("fi", "Salah al-Din"), ("fr", "Salah ad-Din"), ("gu", "સલાડિન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז סלאח א-דין"), ("hi", "सलाह\u{941}द\u{94d}दीन प\u{94d}रान\u{94d}त"), ("hu", "Szaláh ed-Dín kormányzóság"), ("id", "Kegubernuran Salah ad Din"), ("it", "Governatorato di Salah al-Din"), ("ja", "サラーフッディーン県"), ("ka", "სალაჰ-ედ-დინის მუჰაფაზა"), ("kn", "ಸಲಾದ\u{cbf}ನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "살라딘 주"), ("lt", "Saladino gubernija"), ("lv", "Salāh ed Dīnas muhāfaza"), ("ml", "സല\u{d3e} അ ദിൻ"), ("mr", "सालादिन गोव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Salah ad Din"), ("nb", "Salah ad Din"), ("nl", "Salah ad Din"), ("no", "Salah ad Din"), ("pl", "Salah Ad-Din"), ("pt", "Salah-ad-Din"), ("ro", "Salah ad Din"), ("ru", "Салах-эд-Дин"), ("si", "සලඩ\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sk", "Saláh ad-Dín"), ("sr", "Саладин"), ("sr_Latn", "Saladin"), ("sv", "Saladin"), ("ta", "சல\u{bbe}டின\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "సల\u{c3e}ద\u{c3f}న\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองซาลาด\u{e34}น"), ("tr", "Selahaddin ili"), ("uk", "Салах-ед-Дін"), ("ur", "محافظہ صلاح الدین"), ("vi", "Tỉnh Saladin"), ("zh", "萨拉赫丁省")]),
                        unofficial_name_list: ["Salah-ad-Din"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "As Sulaymaniyah",
                        country_alpha2: Alpha2::IQ,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.55), longitude: Some(45.433333), max_latitude: Some(35.6029511), min_latitude: Some(35.5232852), max_longitude: Some(45.476532), min_longitude: Some(45.278778)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "السليمانية"), ("az", "Süleymaniyə"), ("be", "Мухафаза Сулейманія"), ("bg", "Сулеймания"), ("bn", "আস স\u{9c1}ল\u{9be}য\u{9bc}ম\u{9be}নিয\u{9bc}\u{9be}হ প\u{9cd}রদেশ"), ("ca", "Governació de Sulaymaniyya"), ("ccp", "𑄥\u{1112a}𑄣\u{1112d}𑄟𑄚\u{11128}𑄠𑄦\u{11134}"), ("cs", "Sulajmáníja"), ("da", "As Sulaymaniyah"), ("de", "as-Sulaimaniyya"), ("el", "Σουλαϊμανιγιά"), ("en", "Sulaymaniyah"), ("es", "Gobernación de Solimania"), ("et", "As-Sulaymānīyah’ kubernerkond"), ("eu", "Sulaimaniya probintzia"), ("fa", "استان سلیمانیه"), ("fi", "Sulaymaniyya"), ("fr", "As-Sulaymaniya"), ("gu", "સ\u{ac1}લ\u{ac7}માનીયાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מחוז א-סולימאניה"), ("hi", "स\u{941}लयमानियाह प\u{94d}रान\u{94d}त"), ("hu", "Szulejmánijja kormányzóság"), ("id", "Kegubernuran As Sulaymaniyah"), ("it", "Governatorato di al-Sulaymaniyya"), ("ja", "スレイマニヤ県"), ("ka", "სულეიმანიის მუჰაფაზა"), ("kn", "ಸುಲೈಮಾನ\u{cbf}ಯಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "술라이마니야 주"), ("lt", "Suleimanijos gubernija"), ("lv", "Suleimānījas muhāfaza"), ("mr", "स\u{941}लायमानियाह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran As Sulaymaniyah"), ("nb", "Suleimania"), ("nl", "Sulaymaniyah"), ("no", "Suleimania"), ("pl", "As-Sulajmanijja"), ("pt", "As-Sulaymaniyah"), ("ro", "Sulaymaniyya"), ("ru", "Сулеймания"), ("si", "ස\u{dd4}ලේමන\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sr", "Гувернорат Сулејманија"), ("sr_Latn", "Guvernorat Sulejmanija"), ("sv", "Sulaymaniyya"), ("ta", "சுலேம\u{bbe}ணிய கோவெர\u{bcd}னோரே"), ("te", "సుల\u{c3e}య\u{c4d}మ\u{c3e}న\u{c3f}య\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ส\u{e38}เลมาน\u{e34}ยาห\u{e4c}"), ("tr", "Süleymaniye ili"), ("uk", "Сулейманія"), ("ur", "محافظہ سلیمانیہ"), ("vi", "Tỉnh Sulaymaniyah"), ("zh", "蘇萊曼尼亞省")]),
                        unofficial_name_list: ["Sulaymaniya", "Sulaymānīyah", "as-Sulaymaniyah"].to_vec(),
                    }
                ),
                (
                    "WA",
                    Subdivision{
                        name: "Wasit",
                        country_alpha2: Alpha2::IQ,
                        code: "WA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.6024094), longitude: Some(45.7520985), max_latitude: Some(33.47649), min_latitude: Some(31.9118621), max_longitude: Some(46.606579), min_longitude: Some(44.5098)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "واسط"), ("az", "Vasit"), ("bg", "Уасит"), ("bn", "ওয\u{9bc}\u{9be}সিত প\u{9cd}রদেশ"), ("ca", "Governació de Wasit"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄥\u{11128}𑄖\u{11134}"), ("ceb", "Muhafazat Wasit"), ("cs", "Wásit"), ("da", "Wasit"), ("de", "al-Wasit"), ("el", "Βάσιτ"), ("en", "Wasit"), ("es", "Wasit"), ("et", "Wāsiţ"), ("eu", "Wasit probintzia"), ("fa", "استان واسط"), ("fi", "Wasit"), ("fr", "Wasit"), ("gu", "વશિત ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "ואסט"), ("hi", "वासित प\u{94d}रान\u{94d}त"), ("hu", "Vászit kormányzóság"), ("id", "Kegubernuran Wasit"), ("it", "Governatorato di Wasit"), ("ja", "ワーシト県"), ("ka", "ვასიტის მუჰაფაზა"), ("kn", "ವಾಸ\u{cbf}ತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "와시트 주"), ("lt", "Vasito gubernija"), ("lv", "Vāsitas muhāfaza"), ("mr", "वासिट गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Pentadbiran Wasit"), ("nb", "Wasit"), ("nl", "Wasit"), ("no", "Wasit"), ("pl", "Wasit"), ("pt", "Wasit"), ("ro", "Wasit"), ("ru", "Васит"), ("si", "වෙය\u{dd2}ස\u{dca}ට\u{dca} පර\u{dd2}ප\u{dcf}ලන ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Wásit"), ("sl", "Wásit"), ("sv", "Wasit"), ("ta", "வ\u{bbe}சிட\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "వ\u{c3e}స\u{c3f}ట\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตวาส\u{e34}ท"), ("tr", "Vasit ili"), ("uk", "Васит"), ("ur", "محافظہ واسط"), ("vi", "Tỉnh Wasit"), ("zh", "瓦西特省")]),
                        unofficial_name_list: ["Wasit"].to_vec(),
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
#[cfg(feature = "iq")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::IQ,
        alpha3: Alpha3::IRQ,
        address_format: None,
        continent: Continent::Asia,
        country_code: 964,
        currency_code: CurrencyCode::IQD,
        gec: Some(GEC::IZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::IRQ),
        iso_long_name: "The Republic of Iraq",
        iso_short_name: "Iraq",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9, 10].to_vec(),
        national_prefix: "None",
        nationality: Some("Iraqi"),
        number: "368",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "IQ",
        unofficial_name_list: ["Iraq", "العراق", "Irak", "イラク"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Iraq"),
            ("af", "Irak"),
            ("ak", "Iraq"),
            ("am", "ጒሲቅ"),
            ("an", "Iraq"),
            ("ar", "العراق"),
            ("as", "ইৰ\u{9be}ক"),
            ("ay", "Iraq"),
            ("az", "İraq"),
            ("ba", "Iraq"),
            ("be", "Ірак"),
            ("bg", "Ирак"),
            ("bi", "Iraq"),
            ("bn", "ইর\u{9be}ক"),
            ("bn_IN", "ইর\u{9be}ক"),
            ("br", "Irak"),
            ("bs", "Irak"),
            ("ca", "Iraq"),
            ("ce", "Ӏиракъ"),
            ("ch", "Iraq"),
            ("cs", "Irák"),
            ("cv", "Ӏиракъ"),
            ("cy", "Irac"),
            ("da", "Irak"),
            ("de", "Irak"),
            ("dv", "ޢ\u{7a8}ރ\u{7a7}ޤ\u{7aa}"),
            ("dz", "ཨ\u{f72}་རཀ།"),
            ("ee", "Iraq"),
            ("el", "Ιράκ"),
            ("en", "Iraq"),
            ("eo", "Irako"),
            ("es", "Irak"),
            ("et", "Iraak"),
            ("eu", "Irak"),
            ("fa", "عراق"),
            ("ff", "Iraq"),
            ("fi", "Irak"),
            ("fo", "Irak"),
            ("fr", "Irak"),
            ("fy", "Irak"),
            ("ga", "An Iaráic"),
            ("gl", "Iraq"),
            ("gn", "Iraq"),
            ("gu", "ઇરાક"),
            ("gv", "Yn Earack"),
            ("ha", "Irak"),
            ("he", "עיראק"),
            ("hi", "इराक\u{93c}"),
            ("hr", "Irak"),
            ("ht", "Irak"),
            ("hu", "Irak"),
            ("hy", "Իրաք"),
            ("ia", "Irak"),
            ("id", "Irak"),
            ("io", "Irak"),
            ("is", "Írak"),
            ("it", "Iraq"),
            ("iu", "ᐃᕉᒃ"),
            ("ja", "イラク"),
            ("ka", "ერაყი"),
            ("ki", "Iraq"),
            ("kk", "Ирак"),
            ("kl", "Iraq"),
            ("km", "អ\u{17ca}\u{17b8}រ\u{17c9}ាក\u{17cb}"),
            ("kn", "ಇರಾಕ\u{ccd}"),
            ("ko", "이라크"),
            ("ku", "Iraq"),
            ("kv", "Ирак"),
            ("kw", "Irak"),
            ("ky", "Ирак"),
            ("lo", "Iraq"),
            ("lt", "Irakas"),
            ("lv", "Irāka"),
            ("mi", "Īrāki"),
            ("mk", "Ирак"),
            ("ml", "ഇറ\u{d3e}ഖ\u{d4d}"),
            ("mn", "Ирак"),
            ("mr", "इराक"),
            ("ms", "Iraq"),
            ("mt", "Iraq"),
            (
                "my",
                "အ\u{102e}ရတ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Irak"),
            ("nb", "Irak"),
            ("ne", "इराक"),
            ("nl", "Irak"),
            ("nn", "Irak"),
            ("nv", "Iraq"),
            ("oc", "Iraq"),
            ("or", "ଇର\u{b3e}କ"),
            ("pa", "ਇਰਾਕ"),
            ("pi", "ईराक"),
            ("pl", "Irak"),
            ("ps", "عراق"),
            ("pt", "Iraque"),
            ("pt_BR", "Iraque"),
            ("ro", "Irak"),
            ("ru", "Ирак"),
            ("rw", "Irake"),
            ("sc", "Iraq"),
            ("sd", "عراق"),
            ("si", "ඉර\u{dcf}කය"),
            ("sk", "Irak"),
            ("sl", "Irak"),
            ("so", "Ciraaq"),
            ("sq", "Irak"),
            ("sr", "Ирак"),
            ("sv", "Irak"),
            ("sw", "Iraq"),
            ("ta", "ஈர\u{bbe}க\u{bcd}"),
            ("te", "ఇర\u{c3e}క\u{c4d}"),
            ("tg", "Ироқ"),
            ("th", "อ\u{e34}ร\u{e31}ก"),
            ("ti", "ዒራቅ"),
            ("tk", "Irak"),
            ("tl", "Iraq"),
            ("tr", "Irak"),
            ("tt", "Гырак"),
            ("ug", "ئىراق"),
            ("uk", "Ірак"),
            ("ur", "عراق"),
            ("uz", "Iroq"),
            ("ve", "Iraq"),
            ("vi", "I-rắc"),
            ("wa", "Irak"),
            ("wo", "Iraak"),
            ("xh", "Iraq"),
            ("yo", "Irak"),
            ("zh_CN", "伊拉克"),
            ("zh_HK", "伊拉克"),
            ("zh_TW", "伊拉克"),
            ("zu", "I-Iraki"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
