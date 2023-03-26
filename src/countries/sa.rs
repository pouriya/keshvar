// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Saudi Arabia

#[cfg(all(feature = "sa", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SA;
    pub const ALPHA3: Alpha3 = Alpha3::SAU;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 966;
    pub const CURRENCY_CODE: &str = "SAR";
    pub const GEC: Option<GEC> = Some(GEC::SA);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::KSA);
    pub const ISO_SHORT_NAME: &str = "Saudi Arabia";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Saudi Arabia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Saudi Arabian");
    pub const NUMBER: &str = "682";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "SA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Saudi Arabia",
        "Kingdom of Saudi Arabia",
        "السعودية",
        "Saudi-Arabien",
        "Arabie Saoudite",
        "Arabia Saudí",
        "サウジアラビア",
        "Saoedi-Arabië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Saudi Arabia"),
        ("af", "Saoedi-Arabië"),
        ("ak", "Saudi Arabia"),
        ("am", "ሳፄ፥ጐረቢ።"),
        ("an", "Saudi Arabia"),
        ("ar", "السعودية"),
        ("as", "ছৌডি আৰব"),
        ("ay", "Saudi Arabia"),
        ("az", "Səudiyyə Ərəbistan"),
        ("ba", "Saudi Arabia"),
        ("be", "Саудаўская Аравія"),
        ("bg", "Саудитска Арабия"),
        ("bi", "Saudi Arabia"),
        ("bn", "সৌদি আরব"),
        ("bn_IN", "সৌদি আরব"),
        ("br", "Arabia Saoudat"),
        ("bs", "Saudijska Arabija"),
        ("ca", "Aràbia Saudita"),
        ("ce", "СаӀудийн Ӏаьрбийчоь"),
        ("ch", "Saudi Arabia"),
        ("cs", "Saúdská Arábie"),
        ("cv", "СаӀудийн Ӏаьрбийчоь"),
        ("cy", "Saudi Arabia"),
        ("da", "Saudi-Arabien"),
        ("de", "Saudi-Arabien"),
        (
            "dv",
            "ސ\u{7a6}އ\u{7ab}ދ\u{7a9} އ\u{7a6}ރ\u{7a6}ބ\u{7a8}އ\u{7b0}ޔ\u{7a7}",
        ),
        ("dz", "སའ\u{f74}་ད\u{f72}་ ཨ་ར་བ\u{f72}་ཡ།"),
        ("ee", "Saudi Arabia"),
        ("el", "Σαουδική Αραβία"),
        ("en", "Saudi Arabia"),
        ("eo", "Saŭda Arabio"),
        ("es", "Arabia Saudí"),
        ("et", "Saudi Araabia"),
        ("eu", "Saudi Arabia"),
        ("fa", "عربستان سعودی"),
        ("ff", "Saudi Arabia"),
        ("fi", "Saudi-Arabia"),
        ("fo", "Saudi-Arábia"),
        ("fr", "Arabie saoudite"),
        ("fy", "Saûdy-Araabje"),
        ("ga", "An Araib Shádach"),
        ("gl", "Arabia Saudí"),
        ("gn", "Saudi Arabia"),
        ("gu", "સાઉદી અર\u{ac7}બિયા"),
        ("gv", "Yn Araab Saudi"),
        ("ha", "Saudiyya"),
        ("he", "ערב הסעודית"),
        ("hi", "सउदी अरब"),
        ("hr", "Saudijska Arabija"),
        ("ht", "Arabi Sawoudit"),
        ("hu", "Szaúd-Arábia"),
        ("hy", "Սաուդիան Արաբիա"),
        ("ia", "Arabia Saudita"),
        ("id", "Arab Saudi"),
        ("io", "Saudi-Arabia"),
        ("is", "Sádí-Arabía"),
        ("it", "Arabia Saudita"),
        ("iu", "Saudi Arabia"),
        ("ja", "サウジアラビア"),
        ("ka", "საუდის არაბეთი"),
        ("ki", "Saudi Arabia"),
        ("kk", "Сауд Арабиясы"),
        ("kl", "Saudi Arabia"),
        (
            "km",
            "អារ\u{17c9}ាប\u{17ca}\u{17b8}សាអ\u{17ca}\u{17bc}ឌ\u{17b8}ត",
        ),
        ("kn", "ಸ\u{ccc}ದ\u{cbf} ಅರ\u{cc6}ಬ\u{cbf}ಯ"),
        ("ko", "사우디아라비아"),
        ("ku", "Si'ûdî Erebistan"),
        ("kv", "Саудса Аравия"),
        ("kw", "Arabi Saoudek"),
        ("ky", "Сауд Арабия Падышалыгы"),
        ("lo", "Saudi Arabia"),
        ("lt", "Saudo Arabija"),
        ("lv", "Saūda Arābija"),
        ("mi", "Hauri Arāpia"),
        ("mk", "Саудиска Арабија"),
        ("ml", "സൌദി അറേബ\u{d4d}യ"),
        ("mn", "Саудын араб"),
        ("mr", "सौदी अर\u{947}बिया"),
        ("ms", "Arab Saudi"),
        ("mt", "Għarabja Sawdita"),
        (
            "my",
            "ဆော\u{103a}ဒ\u{102e}အာရေဗျန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Taudiarabiya"),
        ("nb", "Saudi-Arabia"),
        ("ne", "साउदी अरब"),
        ("nl", "Saoedi-Arabië"),
        ("nn", "Saudi-Arabia"),
        ("nv", "Ásáí Dineʼé Bikéyah Ntsaaígíí"),
        ("oc", "Arabia saudita"),
        ("or", "ସ\u{b3e}ଉଦୀ ଆରବ"),
        ("pa", "ਸਾਊਦੀ ਅਰਬ"),
        ("pi", "सऊदी अरब"),
        ("pl", "Arabia Saudyjska"),
        ("ps", "سعودی عربستان"),
        ("pt", "Arábia Saudita"),
        ("pt_BR", "Arábia Saudita"),
        ("ro", "Arabia Saudită"),
        ("ru", "Саудовская Аравия"),
        ("rw", "Arabiya Sawudite"),
        ("sc", "Aràbia Saudita"),
        ("sd", "سعودي عرب"),
        ("si", "සව\u{dd4}ද\u{dd2} අර\u{dcf}බ\u{dd2}ය"),
        ("sk", "Saudská Arábia"),
        ("sl", "Saudova Arabija"),
        ("so", "Sacuudi Carabiya"),
        ("sq", "Arabia Saudite"),
        ("sr", "Саудијска Арабија"),
        ("sv", "Saudiarabien"),
        ("sw", "Saudi Arabia"),
        ("ta", "சவூதி அரேபிய\u{bbe}"),
        ("te", "స\u{c4c}ద\u{c40} అర\u{c47}బ\u{c3f}య\u{c3e}"),
        ("tg", "Арабистони Саудӣ"),
        ("th", "ซาอ\u{e38}ด\u{e35}อาระเบ\u{e35}ย"),
        ("ti", "ሰዑዲ ዓረብ"),
        ("tk", "Saud Arawiýa"),
        ("tl", "Saudi Arabia"),
        ("tr", "Suudi Arabistan"),
        ("tt", "Сөгүд Гәрәбстан"),
        ("ug", "سەئۇدى ئەرەبىستان"),
        ("uk", "Саудівська Аравія"),
        ("ur", "سعودی عرب"),
        ("uz", "Saudiya Arabistoni"),
        ("ve", "Saudi Arabia"),
        ("vi", "A-rập Xau-đi"),
        ("wa", "Arabeye Sawoudite"),
        ("wo", "Araabi Sawdit"),
        ("xh", "Saudi Arabia"),
        ("yo", "Sáúdí Arábíà"),
        ("zh_CN", "沙特阿拉伯"),
        ("zh_HK", "沙地阿拉伯"),
        ("zh_TW", "沙烏地阿拉伯"),
        ("zu", "Saudi Arabia"),
    ];
    #[cfg(all(feature = "sa", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 23.885942;
        pub const LONGITUDE: f64 = 45.079162;
        pub const MAX_LATITUDE: f64 = 32.154284;
        pub const MAX_LONGITUDE: f64 = 55.6666999;
        pub const MIN_LATITUDE: f64 = 16.0036;
        pub const MIN_LONGITUDE: f64 = 34.4815001;
        pub const NORTHEAST_LATITUDE: f64 = 32.154284;
        pub const NORTHEAST_LONGITUDE: f64 = 55.6666999;
        pub const SOUTHWEST_LATITUDE: f64 = 16.0036;
        pub const SOUTHWEST_LONGITUDE: f64 = 34.4815001;
    }
}
#[cfg(all(feature = "sa", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 23.885942,
            longitude: 45.079162,
            max_latitude: 32.154284,
            max_longitude: 55.6666999,
            min_latitude: 16.0036,
            min_longitude: 34.4815001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 32.154284,
                    longitude: 55.6666999,
                },
                southwest: CountryGeoBound {
                    latitude: 16.0036,
                    longitude: 34.4815001,
                },
            },
        }
    }
}

#[cfg(all(feature = "sa", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::SA,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.633333), longitude: Some(46.716667), max_latitude: Some(25.1564724), min_latitude: Some(24.2939113), max_longitude: Some(47.34695430000001), min_longitude: Some(46.2981033)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة الرياض"), ("be", "Правінцыя Рыяд"), ("ca", "Província de Riad"), ("ccp", "𑄢\u{11128}𑄠𑄖\u{11134}"), ("ceb", "Minţaqat ar Riyāḑ"), ("de", "Provinz Riad"), ("el", "Ριάντ"), ("en", "Riyadh"), ("es", "Provincia de Riad"), ("et", "Ar-Riyāḑi provints"), ("fa", "استان ریاض"), ("fi", "Ar Riyad"), ("fr", "Riyad (province)"), ("he", "מחוז ריאד"), ("hi", "रियाद प\u{94d}रान\u{94d}त"), ("hr", "Rijad (pokrajina)"), ("hu", "Rijád tartomány"), ("id", "Provinsi Riyadh"), ("it", "Al-Riyad"), ("ja", "リヤード州"), ("ko", "리야드 주"), ("ml", "റിയ\u{d3e}ദ\u{d4d} പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "रियाध प\u{94d}रा\u{902}त"), ("nb", "Ar Riyad (provins)"), ("nl", "Riyad"), ("no", "Ar Riyad (provins)"), ("pl", "Rijad (prowincja)"), ("pt", "Ar Riyad"), ("ro", "Provincia Riad"), ("ru", "Эр-Рияд"), ("sv", "Ar-Riyad"), ("tr", "Riyad Bölgesi"), ("uk", "Ер-Ріяд"), ("ur", "صوبہ الرياض"), ("vi", "Riyadh"), ("zh", "利雅得省")]),
                        unofficial_name_list: ["Riad", "Riad", "Riyadh", "ar-Riyad", "ar-Riyād\u{328}"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::SA,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.416667), longitude: Some(39.816667), max_latitude: Some(21.5930031), min_latitude: Some(21.2790277), max_longitude: Some(40.0028579), min_longitude: Some(39.6902353)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة مكة المكرمة"), ("be", "Правінцыя Мекка"), ("bn", "মক\u{9cd}ক\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de la Meca"), ("ccp", "𑄟\u{11127}𑄇\u{11133}𑄦𑄦\u{11134}"), ("ceb", "Makkah Province"), ("da", "Makkah Region"), ("de", "Provinz Mekka"), ("el", "Επαρχία Μάκκα"), ("en", "Makkah"), ("es", "Provincia de La Meca"), ("et", "Meka provints"), ("fa", "استان مکه"), ("fi", "Makkah"), ("fr", "La Mecque"), ("gu", "મક\u{acd}કાહ પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז מכה"), ("hi", "मक\u{94d}का प\u{94d}रान\u{94d}त"), ("hu", "Mekka tartomány"), ("id", "Provinsi Makkah"), ("it", "provincia della Mecca"), ("ja", "マッカ州"), ("jv", "Provinsi Mekkah"), ("kn", "ಮಕಾಹ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "메카 주"), ("lt", "Mekos emyratas"), ("lv", "Mekas mintaka"), ("mk", "Мека"), ("ml", "മക\u{d4d}ക പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "मक\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Makkah Region"), ("nb", "Mekka"), ("nl", "Mekka"), ("no", "Mekka"), ("pa", "ਮ\u{a71}ਕਾ ਸ\u{a42}ਬਾ"), ("pl", "Mekka"), ("ps", "مکه معظمه"), ("pt", "Meca"), ("ro", "Provincia Mecca"), ("ru", "Мекка"), ("si", "මක\u{dca}කම ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Мека"), ("sr_Latn", "Meka"), ("sv", "Mekka"), ("ta", "மக\u{bcd}க\u{bbe} பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("te", "మక\u{c4d}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e31}กกะฮ\u{e4c}"), ("tr", "Mekke Bölgesi"), ("uk", "Мекка (провінція)"), ("ur", "صوبہ المکہ"), ("vi", "Khu vực Makkah"), ("zh", "麥加省")]),
                        unofficial_name_list: ["La Meca", "Mecca"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::SA,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.466667), longitude: Some(39.6), max_latitude: Some(24.6591863), min_latitude: Some(24.2829585), max_longitude: Some(39.861145), min_longitude: Some(39.37397)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة المدينة المنورة"), ("az", "Əl-Mədinə"), ("be", "Эль-Медына"), ("bn", "আল মদিন\u{9be} অঞ\u{9cd}চল"), ("ca", "Província de Medina"), ("ccp", "𑄃𑄣\u{11134} 𑄟\u{11127}𑄓\u{11128}𑄚𑄦\u{11134}"), ("ceb", "Al Madīnah al Munawwarah"), ("da", "Al Madinah Region"), ("de", "Provinz Medina"), ("el", "Επαρχία Μαδίνα"), ("en", "Al Madinah"), ("es", "Provincia de Medina"), ("et", "Mediina provints"), ("fa", "استان مدینه"), ("fi", "Al Madinah"), ("fr", "Médine"), ("gu", "અલ મદીના પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז אל-מדינה"), ("hi", "मदीना प\u{94d}रान\u{94d}त"), ("hr", "Medina"), ("hu", "Medina tartomány"), ("id", "Provinsi Madinah"), ("it", "Medina"), ("ja", "マディーナ州"), ("kn", "ಅಲ\u{ccd} ಮಡ\u{cbf}ನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "메디나 주"), ("lt", "Medinos emyratas"), ("lv", "Medīnas mintaka"), ("mk", "Медина"), ("ml", "മദീന പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "अल मदीना प\u{94d}रा\u{902}त"), ("ms", "Al Madinah"), ("nb", "Medina"), ("nl", "Medina"), ("no", "Medina"), ("pl", "Medyna"), ("pt", "Al Madinah"), ("ro", "Provincia Medina"), ("ru", "Медина (провинция)"), ("si", "අල\u{dca} මද\u{dd3}න\u{dcf} කල\u{dcf}පය"), ("sr", "Ел Медина"), ("sr_Latn", "El Medina"), ("sv", "Al-Madinah"), ("ta", "அல\u{bcd}-மத\u{bc0}ன\u{bbe} பிர\u{bbe}ந\u{bcd}தியம\u{bcd}"), ("te", "అల\u{c4d} మ\u{c3e}ద\u{c40}న\u{c3e}హ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอ\u{e31}ลมะด\u{e35}นะฮ\u{e4c}"), ("tr", "Medine Bölgesi"), ("uk", "Ель-Мадіна"), ("ur", "صوبہ المدينہ"), ("vi", "Khu vực Al Madinah"), ("zh", "麦地那省")]),
                        unofficial_name_list: ["Medina", "Médine", "al-Madinah"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::SA,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.2954496), longitude: Some(50.6793759), max_latitude: Some(29.1188431), min_latitude: Some(17.013492), max_longitude: Some(55.6665879), min_longitude: Some(44.9209199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة الشرقية"), ("be", "Эш-Шаркія"), ("bn", "ইস\u{9cd}ট\u{9be}র\u{9cd}ন প\u{9cd}রদেশ"), ("ca", "Província Oriental"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄑𑄢\u{11134}𑄚\u{11134}"), ("ceb", "Al Minţaqah ash Sharqīyah"), ("da", "Eastern Province"), ("de", "Provinz asch-Scharqiyya"), ("el", "Ανατολική επαρχία"), ("en", "Eastern"), ("es", "Provincia Oriental"), ("et", "Ash-Sharqīyah"), ("fa", "استان شرقی (عربستان سعودی)"), ("fi", "Al-Šarqiyya"), ("fr", "Ach-Charqiya"), ("gu", "પ\u{ac2}ર\u{acd}વીય પ\u{acd}રા\u{a82}ત"), ("he", "מחוז א-שרקייה"), ("hi", "प\u{942}र\u{94d}वी प\u{94d}रान\u{94d}त, सउदी अरब"), ("hr", "Istočna pokrajina"), ("hu", "Keleti tartomány"), ("id", "Syarqiyah, Arab Saudi"), ("it", "Al-Sharqiyya"), ("ja", "東部州"), ("ka", "ეშ-შარკია"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "동부 주"), ("lt", "Rytų Provincija"), ("lv", "Austrumu province"), ("ml", "കിഴക\u{d4d}കൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "प\u{942}र\u{94d}व प\u{94d}रा\u{902}त"), ("ms", "Ash Syarqiyah"), ("nb", "Ash Sharqiyah"), ("nl", "Ash Sharqiyah"), ("no", "Ash Sharqiyah"), ("pl", "Prowincja Wschodnia"), ("pt", "Ash Sharqiyah"), ("ro", "Ash Sharqiyah"), ("ru", "Восточная провинция"), ("si", "නැගෙනහ\u{dd2}ර පළ\u{dcf}ත"), ("sr", "Источна провинција"), ("sr_Latn", "Istočna provincija"), ("sv", "Ash Sharqiyah"), ("ta", "கிழக\u{bcd}கு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "สยาร\u{e34}อาห\u{e4c}, ซาอ\u{e38}ด\u{e35}อาระเบ\u{e35}ย"), ("tr", "Doğu Bölgesi"), ("uk", "Еш-Шаркійя"), ("ur", "صوبہ الشرقيہ"), ("vi", "Tỉnh Miền Đông"), ("zh", "东部省")]),
                        unofficial_name_list: ["Eastern", "ash-Sharqiyah"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::SA,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.8274886), longitude: Some(42.8637875), max_latitude: Some(27.3311671), min_latitude: Some(24.6168831), max_longitude: Some(44.8272001), min_longitude: Some(41.40690499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة القصيم"), ("be", "Правінцыя Эль-Касім"), ("bn", "আল ক\u{9be}সিম অঞ\u{9cd}চল"), ("ca", "Al-Kasim"), ("ccp", "𑄃𑄣\u{11134}-𑄇\u{1112a}𑄥\u{11128}𑄟\u{11134}"), ("ceb", "Al-Qassim Province"), ("da", "Al-Qassim Region"), ("de", "Provinz al-Qasim"), ("el", "Επαρχία Κασίμ"), ("en", "Al-Qassim"), ("es", "Casim"), ("et", "Al-Qaşīm"), ("fa", "استان قصیم"), ("fi", "Al Qasim"), ("fr", "Al Qasim"), ("gu", "અલ-કાસીમ પ\u{acd}રદ\u{ac7}શ"), ("he", "אזור אלקסים"), ("hi", "क\u{93c}सीम प\u{94d}रान\u{94d}त"), ("hr", "Kasim (pokrajina)"), ("hu", "Kaszím tartomány"), ("id", "Provinsi Qasim"), ("it", "Al-Qasim"), ("ja", "カスィーム州"), ("kn", "ಅಲ\u{ccd}-ಖಾಸ\u{ccd}ಸ\u{cbf}ಮ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "카심 주"), ("lt", "Kasimo emyratas"), ("lv", "Kasīmas mintaka"), ("ml", "അൽ ഖസീം"), ("mr", "अल-कासिम प\u{94d}रद\u{947}श"), ("ms", "Al Qasim"), ("nb", "Al Qasim (provins)"), ("nl", "Al Qasim"), ("no", "Al Qasim (provins)"), ("pa", "ਅਲ-ਕਸੀਮ ਸ\u{a42}ਬਾ"), ("pl", "Al-Kasim"), ("pt", "Al Qasim"), ("ro", "Provincia Al-Qassim"), ("ru", "Эль-Касим"), ("si", "අල\u{dca}-කස\u{dd3}ම\u{dca} කල\u{dcf}පය"), ("sr", "Ел Касим"), ("sr_Latn", "El Kasim"), ("sv", "Al Qasim"), ("ta", "அல\u{bcd} -கிஆசிம\u{bcd} பகுதி"), ("te", "అల\u{c4d}-ఖ\u{c3e}స\u{c3f}ం ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เม\u{e37}องแอลเจ\u{e35}ยร\u{e4c}"), ("tr", "El Kasım Bölgesi"), ("uk", "Ель-Касим"), ("ur", "صوبہ القصيم"), ("vi", "Khu vực Al-Qassim"), ("zh", "盖西姆省")]),
                        unofficial_name_list: ["Qaseem"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::SA,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.516667), longitude: Some(41.683333), max_latitude: Some(27.6987282), min_latitude: Some(27.3530152), max_longitude: Some(41.8434906), min_longitude: Some(41.5061761)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة حائل"), ("be", "Правінцыя Хаіль"), ("bn", "হ\u{9be}ইল অঞ\u{9cd}চল"), ("ca", "Província de Ha’il"), ("ccp", "𑄦‘𑄃\u{11128}𑄣\u{11134}"), ("ceb", "Minţaqat Ḩā’il"), ("da", "v"), ("de", "Provinz Ha’il"), ("el", "Επαρχία Χαΐλ"), ("en", "Ha’il"), ("es", "Provincia de Haíl"), ("et", "Ḩā’ili provints"), ("fa", "منطقه حائل"), ("fi", "Ha’il"), ("fr", "Haïl"), ("gu", "હાઈલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "हाइल प\u{94d}रान\u{94d}त"), ("hr", "Ha’il"), ("hu", "Háil tartomány"), ("hy", "Հայիլի էմիրություն"), ("id", "Provinsi Ha’il"), ("it", "Ha’il"), ("ja", "ハーイル州"), ("ka", "ჰაილის პროვინცია"), ("kn", "ಹಾಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "하일 주"), ("lt", "Hailo emyratas"), ("lv", "Hāilas mintaka"), ("mk", "Хаил"), ("ml", "ഹ\u{d3e}യിൽ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "हाईल प\u{94d}रद\u{947}श"), ("ms", "Ha’il"), ("nb", "Ha’il"), ("nl", "Hail"), ("no", "Ha’il"), ("pa", "ਹਾਇਲ ਰਿਆਸਤ"), ("pl", "Hail"), ("pt", "Ha’il"), ("ro", "Provincia Ha’il"), ("ru", "Хаиль"), ("si", "හ\u{dcf} ‘ඉල\u{dca} කල\u{dcf}පය"), ("sr", "Хаил"), ("sr_Latn", "Hail"), ("sv", "Ha’il"), ("ta", "ஹ\u{bbe} ‘இல\u{bcd} பகுதி"), ("te", "హ\u{c3e}య\u{c3f}ల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตฮาอ\u{e34}ล"), ("tr", "Hail Bölgesi"), ("uk", "Хаїль"), ("ur", "صوبہ حائل"), ("vi", "Tỉnh Ha’il"), ("zh", "哈伊勒省")]),
                        unofficial_name_list: ["Hail"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::SA,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.383333), longitude: Some(36.583333), max_latitude: Some(28.4718602), min_latitude: Some(28.3255379), max_longitude: Some(36.6991425), min_longitude: Some(36.4326624)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة تبوك"), ("be", "Правінцыя Табук"), ("bn", "ত\u{9be}ব\u{9c1}ক অঞ\u{9cd}চল"), ("ca", "Província de Tabuk"), ("ccp", "𑄑𑄝\u{1112a}𑄇\u{11134}"), ("ceb", "Minţaqat Tabūk"), ("da", "Tabuk Region"), ("de", "Provinz Tabuk"), ("el", "Επαρχία Ταμπούκ"), ("en", "Tabuk"), ("es", "Provincia de Tabuk"), ("et", "Tabūki provints"), ("fa", "استان تبوک"), ("fi", "Tabuk"), ("fr", "Tabuk (province)"), ("gu", "તાબ\u{ac2}ક પ\u{acd}રદ\u{ac7}શ"), ("hi", "तब\u{942}क प\u{94d}रान\u{94d}त"), ("hr", "Tabuk (pokrajina)"), ("hu", "Tabúk tartomány"), ("id", "Provinsi Tabuk"), ("it", "Tabuk"), ("ja", "タブーク州"), ("kn", "ತಾಬುಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "타부크 주"), ("lt", "Tabuko emyratas"), ("lv", "Tebūkas mintaka"), ("ml", "തബ\u{d42}ക\u{d4d}ക\u{d4d} പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "ताब\u{942}क प\u{94d}रद\u{947}श"), ("ms", "Tabuk"), ("nb", "Tabuk (provins)"), ("nl", "Tabuk"), ("no", "Tabuk (provins)"), ("pa", "ਤਬ\u{a42}ਕ ਸ\u{a42}ਬਾ"), ("pl", "Tabuk (prowincja)"), ("pt", "Tabuk (província)"), ("ro", "Provincia Tabuk"), ("ru", "Табук"), ("si", ", ටබ\u{dd4}ක\u{dca} කල\u{dcf}පය"), ("sr", "Табук"), ("sr_Latn", "Tabuk"), ("sv", "Tabuk (provins)"), ("ta", "தபூக\u{bcd} பகுதி"), ("te", "ట\u{c3e}బుక\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตตาบ\u{e38}ก"), ("tr", "Tebük Bölgesi"), ("uk", "Табук"), ("ur", "صوبہ تبوک"), ("vi", "Khu vực Tabuk"), ("zh", "塔布克省")]),
                        unofficial_name_list: ["Tabook"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::SA,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.7248676), longitude: Some(42.2362435), max_latitude: Some(32.158333), min_latitude: Some(27.459833), max_longitude: Some(45.92311), min_longitude: Some(37.8584141)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة الحدود الشمالية"), ("be", "Эль-Худуд эш-Шамалія"), ("ca", "Província de la Frontera Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄥\u{11128}𑄟\u{11128}𑄚𑄬"), ("ceb", "Minţaqat al Ḩudūd ash Shamālīyah"), ("de", "Provinz al-Hudud asch-schamaliyya"), ("el", "Επαρχία Βορείων Συνόρων"), ("en", "Northern Borders"), ("es", "Provincia de la Frontera del Norte"), ("et", "Al-Ḩudūd ash-Shamālīyah"), ("fa", "استان مرزهای شمالی"), ("fi", "Al Hudud ash Shamaliyah"), ("fr", "Al-Hudud ach-Chamaliya"), ("hi", "उत\u{94d}तरी सीमाए\u{901} प\u{94d}रान\u{94d}त"), ("hu", "Északi határvidék tartomány"), ("id", "Hududusy Syamaliyah"), ("it", "Al-Hudud al-Shamaliyya"), ("ja", "北部国境州"), ("ko", "북부 변경 주"), ("lt", "Šiaurinio Krašto emyratas"), ("ml", "വടക\u{d4d}കൻ അതിർത\u{d4d}തി പ\u{d4d}രവിശ\u{d4d}യ"), ("ms", "Al Hudud ash Shamaliyah"), ("nb", "Al Hudud ash Shamaliyah"), ("nl", "Al Hudud ash Shamaliyah"), ("no", "Al Hudud ash Shamaliyah"), ("pl", "Północna Prowincja Graniczna"), ("pt", "Al Hudud ash Shamaliyah"), ("ro", "Al Hudud ash Shamaliyah"), ("ru", "Эль-Худуд-эш-Шамалия"), ("sr", "Северна граница"), ("sr_Latn", "Severna granica"), ("sv", "Al Hudud ash Shamaliyah"), ("tr", "Kuzey Sınır Bölgesi"), ("uk", "Ель-Худуд еш-Шамалійя"), ("ur", "صوبہ الحدود الشماليہ"), ("vi", "Biên giới Phương Bắc"), ("zh", "北部边疆省")]),
                        unofficial_name_list: ["Northern", "al-Hudud ash-Shamaliyah"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::SA,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.889167), longitude: Some(42.561111), max_latitude: Some(16.9898154), min_latitude: Some(16.8181793), max_longitude: Some(42.64789589999999), min_longitude: Some(42.5295783)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة جازان"), ("be", "Правінцыя Джызан"), ("ca", "Jizan"), ("ccp", "𑄎\u{11128}𑄎𑄚\u{11134}"), ("ceb", "Jizan Region"), ("de", "Provinz Dschāzān"), ("el", "Επαρχία Τζιζάν"), ("en", "Jizan"), ("es", "Provincia de Jizán"), ("et", "Jīzāni provints"), ("fa", "استان جازان"), ("fi", "Jizan"), ("fr", "Jizan"), ("hi", "जाज\u{93c}ान प\u{94d}रान\u{94d}त"), ("hu", "Dzsízán tartomány"), ("id", "Provinsi Jizan"), ("it", "Jazan"), ("ja", "ジーザーン州"), ("ko", "지잔 주"), ("lt", "Džizano provincija"), ("ml", "ജിസ\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("ms", "Jizan"), ("nb", "Jizan"), ("nl", "Jizan"), ("no", "Jizan"), ("pl", "Dżizan"), ("pt", "Jizan"), ("ro", "Provincia Jizan"), ("ru", "Джизан"), ("sr", "Џизан"), ("sr_Latn", "Džizan"), ("sv", "Jizan"), ("tr", "Cizan Bölgesi"), ("uk", "Джизан"), ("ur", "صوبہ جازان"), ("vi", "Jizan"), ("zh", "吉赞省")]),
                        unofficial_name_list: ["Jizan"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::SA,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.491667), longitude: Some(44.132222), max_latitude: Some(17.6004129), min_latitude: Some(17.4746589), max_longitude: Some(44.2933307), min_longitude: Some(44.0930793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة نجران"), ("be", "Правінцыя Наджран"), ("bn", "ন\u{9be}জর\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Província de Najran"), ("ccp", "𑄚𑄌\u{11134}𑄢𑄚\u{11134}"), ("ceb", "Minţaqat Najrān"), ("da", "Najran Region"), ("de", "Provinz Nadschran"), ("el", "Περιφέρεια Νατζράν"), ("en", "Najran"), ("es", "Provincia de Najrán"), ("et", "Najrāni provints"), ("fa", "استان نجران"), ("fi", "Najran"), ("fr", "Najran"), ("gu", "નજરાન પ\u{acd}રદ\u{ac7}શ"), ("hi", "नजरान प\u{94d}रान\u{94d}त"), ("hu", "Nadzsrán tartomány"), ("id", "Provinsi Najran"), ("it", "Najran"), ("ja", "ナジュラーン州"), ("kn", "ನಜ\u{ccd}ರಾನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "나즈란 주"), ("lt", "Nadžrano regionas"), ("lv", "Nedžrānas mintaka"), ("ml", "നജ\u{d4d}റ\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "नजर\u{947}न प\u{94d}रद\u{947}श"), ("ms", "Najran"), ("nb", "Najran"), ("nl", "Najran"), ("no", "Najran"), ("pl", "Nadżran"), ("pt", "Najran"), ("ro", "Provincia Najran"), ("ru", "Наджран"), ("si", "නජ\u{dca}රන\u{dca} කල\u{dcf}පය"), ("sr", "Наџран"), ("sr_Latn", "Nadžran"), ("sv", "Najran"), ("ta", "நஜ\u{bcd}ர\u{bbe}ன\u{bcd} பகுதி"), ("te", "నజ\u{c4d}రన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "น\u{e31}จญ\u{e4c}รอน"), ("tr", "Necran Bölgesi"), ("uk", "Наджран"), ("ur", "صوبہ نجران"), ("vi", "Khu vực Najran"), ("zh", "奈季兰省")]),
                        unofficial_name_list: ["Najran"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::SA,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.4556901), longitude: Some(39.2133847), max_latitude: Some(21.4561942), min_latitude: Some(21.4550067), max_longitude: Some(39.2135471), min_longitude: Some(39.21315860000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة الباحة"), ("be", "Правінцыя Эль-Баха"), ("bn", "আল ব\u{9be}হ\u{9be} অঞ\u{9cd}চল"), ("ca", "Província d’Al-Bahah"), ("ccp", "𑄃𑄣\u{11134} 𑄝𑄦\u{11134}"), ("ceb", "Minţaqat al Bāḩah"), ("da", "Al Bahah Region"), ("de", "Provinz al-Baha"), ("el", "Επαρχία Μπαά"), ("en", "Al Bahah"), ("es", "Provincia de Baha"), ("et", "Al-Bāḩah’ provints"), ("fa", "استان باحه"), ("fi", "Al Bahah"), ("fr", "Al Bahah"), ("gu", "અલ બહાહ પ\u{acd}રદ\u{ac7}શ"), ("hi", "बाहाह प\u{94d}रान\u{94d}त"), ("hr", "Bahah (pokrajina)"), ("hu", "Báha tartomány"), ("id", "Provinsi Bahah"), ("it", "Al-Baha"), ("ja", "バーハ州"), ("kn", "ಅಲ\u{ccd} ಬಹಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "바하 주"), ("lt", "Al Bahos emyratas"), ("lv", "Bāhas mintaka"), ("ml", "അൽ ബഹ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "अल-बाहा प\u{94d}रा\u{902}त"), ("ms", "Al Bahah"), ("nb", "al-Bāḥa"), ("nl", "Al-Bahah"), ("no", "al-Bāḥa"), ("pa", "ਅਲਬਾਹਾ ਸ\u{a42}ਬਾ"), ("pl", "Al-Baha (prowincja)"), ("pt", "Al Bahah (distrito)"), ("ro", "Provincia Al Bahah"), ("ru", "Эль-Баха"), ("si", "අල\u{dca} බහ\u{dcf} කල\u{dcf}පය"), ("sr", "Ал Баха (покрајина)"), ("sr_Latn", "Al Baha (pokrajina)"), ("sv", "Al Bahah"), ("ta", "அல\u{bcd} பஹத\u{bcd} பகுதி"), ("te", "అల\u{c4d} బహ\u{c3e}హ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}ลบาฮา"), ("tr", "El Baha Bölgesi"), ("uk", "Ель-Баха (провінція)"), ("ur", "صوبہ الباحہ"), ("vi", "Khu vực Al Bahah"), ("zh", "巴哈省")]),
                        unofficial_name_list: ["Baha"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::SA,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.887356), longitude: Some(39.3206241), max_latitude: Some(31.7389761), min_latitude: Some(28.1313139), max_longitude: Some(41.91381500000001), min_longitude: Some(34.8245874)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة الجوف"), ("be", "Правінцыя Эль-Джаўф"), ("bg", "Ал-Джоуф"), ("bn", "আল য\u{9be}ফ অঞ\u{9cd}চল"), ("ca", "Província d’Al-Jawf"), ("ccp", "𑄃𑄣\u{11134} 𑄎\u{11127}𑄠\u{1112e}𑄛\u{11134}"), ("ceb", "Al Jawf"), ("da", "Al Jawf Region"), ("de", "Provinz al-Dschauf"), ("el", "Επαρχία Τζωφ"), ("en", "Al Jawf"), ("es", "Provincia de Yauf"), ("et", "Al-Jawfi provints"), ("fa", "جوف"), ("fi", "Al Jawf"), ("fr", "Al Jawf (province)"), ("gu", "અલ જૉફ પ\u{acd}રદ\u{ac7}શ"), ("hi", "जौफ\u{93c} प\u{94d}रान\u{94d}त"), ("hr", "Džavf (pokrajina)"), ("hu", "Dzsauf tartomány (Szaúd-Arábia)"), ("id", "Provinsi Jauf"), ("it", "Al-Jawf"), ("ja", "ジャウフ州"), ("kn", "ಅಲ\u{ccd} ಜಾವ\u{ccd}ಫ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "자우프 주 (사우디아라비아)"), ("lt", "Džaufo emyratas"), ("lv", "Džaufas mintaka"), ("ml", "അൽ ജ\u{d57}ഫ\u{d4d} പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "अल जौफ प\u{94d}रद\u{947}श"), ("ms", "Wilayah Al Jawf"), ("nb", "Al Jawf (provins)"), ("nl", "Al Jawf"), ("no", "Al Jawf (provins)"), ("pa", "ਜ\u{a4c}ਫ\u{a3c} ਸ\u{a42}ਬਾ"), ("pl", "Al-Dżauf (prowincja)"), ("pt", "Al Jawf (Arábia Saudita)"), ("ro", "Provincia Al Jawf"), ("ru", "Эль-Джауф"), ("si", "අල\u{dca} ජෝෆ\u{dca} කල\u{dcf}පය"), ("sr", "Ел Џауф"), ("sr_Latn", "El Džauf"), ("sv", "Al Jawf"), ("ta", "அல\u{bcd} ஜ\u{bbe}வ\u{bcd}ப\u{bcd} பகுதி"), ("te", "అల\u{c4d} జ\u{c3e}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}ลเจาฟ\u{e4c}"), ("tr", "Cevf Bölgesi"), ("uk", "Ель-Джауф"), ("ur", "صوبہ الجوف"), ("vi", "Khu vực Al Jawf"), ("zh", "焦夫省 (沙地阿拉伯)")]),
                        unofficial_name_list: ["Al-Jouf"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::SA,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.0969062), longitude: Some(42.8637875), max_latitude: Some(20.970846), min_latitude: Some(17.422384), max_longitude: Some(44.528442), min_longitude: Some(41.38029)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة عسير"), ("be", "Правінцыя Асір"), ("bn", "আসির প\u{9cd}রদেশ"), ("ca", "Regió d’Asir"), ("ccp", "𑄃𑄥\u{11128}𑄢\u{11134}"), ("ceb", "Minţaqat ‘Asīr"), ("cs", "Asír"), ("da", "Asir Region"), ("de", "Provinz Asir"), ("el", "Επαρχία Ασίρ"), ("en", "Asir"), ("es", "Provincia de Asir"), ("fa", "استان عسیر"), ("fi", "Asir"), ("fr", "Asir (province)"), ("gu", "આસિર પ\u{acd}રદ\u{ac7}શ"), ("he", "אזור עסיר"), ("hi", "असीर प\u{94d}रान\u{94d}त"), ("hr", "Asir (pokrajina)"), ("hu", "Aszír tartomány"), ("id", "Provinsi ‘Asir"), ("ja", "アスィール州"), ("ka", "ასირი"), ("kn", "ಆಸ\u{cbf}ರ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아시르 주"), ("lt", "Asiro emyratas"), ("lv", "Asīras mintaka"), ("ml", "അസീർ പ\u{d4d}രവിശ\u{d4d}യ"), ("mr", "असर प\u{94d}रद\u{947}श"), ("ms", "‘Asir"), ("nb", "Asir"), ("ne", "असिर प\u{94d}रान\u{94d}त"), ("nl", "Asir"), ("no", "Asir"), ("pa", "ਅਸੀਰ ਰਿਆਸਤ"), ("pl", "Asir"), ("pt", "‘Asir"), ("ro", "Provincia ‘Asir"), ("ru", "Асир (провинция)"), ("si", "අස\u{dd3}ර\u{dca} කල\u{dcf}පය"), ("sr", "Асир"), ("sr_Latn", "Asir"), ("sv", "‘Asir"), ("ta", "ஆச\u{bc0}ர\u{bcd} பகுதி"), ("te", "య\u{c3e}స\u{c3f}ర\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดอาเซอร\u{e4c}"), ("tr", "Asir Bölgesi"), ("uk", "Асір"), ("ur", "صوبہ عسير"), ("vi", "Khu vực Asir"), ("zh", "阿西尔省")]),
                        unofficial_name_list: ["Aseer"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "sa")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SA,
        alpha3: Alpha3::SAU,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 966,
        currency_code: "SAR",
        gec: Some(GEC::SA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::KSA),
        iso_long_name: "The Kingdom of Saudi Arabia",
        iso_short_name: "Saudi Arabia",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Saudi Arabian"),
        number: "682",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "SA",
        unofficial_name_list: [
            "Saudi Arabia",
            "Kingdom of Saudi Arabia",
            "السعودية",
            "Saudi-Arabien",
            "Arabie Saoudite",
            "Arabia Saudí",
            "サウジアラビア",
            "Saoedi-Arabië",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Saudi Arabia"),
            ("af", "Saoedi-Arabië"),
            ("ak", "Saudi Arabia"),
            ("am", "ሳፄ፥ጐረቢ።"),
            ("an", "Saudi Arabia"),
            ("ar", "السعودية"),
            ("as", "ছৌডি আৰব"),
            ("ay", "Saudi Arabia"),
            ("az", "Səudiyyə Ərəbistan"),
            ("ba", "Saudi Arabia"),
            ("be", "Саудаўская Аравія"),
            ("bg", "Саудитска Арабия"),
            ("bi", "Saudi Arabia"),
            ("bn", "সৌদি আরব"),
            ("bn_IN", "সৌদি আরব"),
            ("br", "Arabia Saoudat"),
            ("bs", "Saudijska Arabija"),
            ("ca", "Aràbia Saudita"),
            ("ce", "СаӀудийн Ӏаьрбийчоь"),
            ("ch", "Saudi Arabia"),
            ("cs", "Saúdská Arábie"),
            ("cv", "СаӀудийн Ӏаьрбийчоь"),
            ("cy", "Saudi Arabia"),
            ("da", "Saudi-Arabien"),
            ("de", "Saudi-Arabien"),
            (
                "dv",
                "ސ\u{7a6}އ\u{7ab}ދ\u{7a9} އ\u{7a6}ރ\u{7a6}ބ\u{7a8}އ\u{7b0}ޔ\u{7a7}",
            ),
            ("dz", "སའ\u{f74}་ད\u{f72}་ ཨ་ར་བ\u{f72}་ཡ།"),
            ("ee", "Saudi Arabia"),
            ("el", "Σαουδική Αραβία"),
            ("en", "Saudi Arabia"),
            ("eo", "Saŭda Arabio"),
            ("es", "Arabia Saudí"),
            ("et", "Saudi Araabia"),
            ("eu", "Saudi Arabia"),
            ("fa", "عربستان سعودی"),
            ("ff", "Saudi Arabia"),
            ("fi", "Saudi-Arabia"),
            ("fo", "Saudi-Arábia"),
            ("fr", "Arabie saoudite"),
            ("fy", "Saûdy-Araabje"),
            ("ga", "An Araib Shádach"),
            ("gl", "Arabia Saudí"),
            ("gn", "Saudi Arabia"),
            ("gu", "સાઉદી અર\u{ac7}બિયા"),
            ("gv", "Yn Araab Saudi"),
            ("ha", "Saudiyya"),
            ("he", "ערב הסעודית"),
            ("hi", "सउदी अरब"),
            ("hr", "Saudijska Arabija"),
            ("ht", "Arabi Sawoudit"),
            ("hu", "Szaúd-Arábia"),
            ("hy", "Սաուդիան Արաբիա"),
            ("ia", "Arabia Saudita"),
            ("id", "Arab Saudi"),
            ("io", "Saudi-Arabia"),
            ("is", "Sádí-Arabía"),
            ("it", "Arabia Saudita"),
            ("iu", "Saudi Arabia"),
            ("ja", "サウジアラビア"),
            ("ka", "საუდის არაბეთი"),
            ("ki", "Saudi Arabia"),
            ("kk", "Сауд Арабиясы"),
            ("kl", "Saudi Arabia"),
            (
                "km",
                "អារ\u{17c9}ាប\u{17ca}\u{17b8}សាអ\u{17ca}\u{17bc}ឌ\u{17b8}ត",
            ),
            ("kn", "ಸ\u{ccc}ದ\u{cbf} ಅರ\u{cc6}ಬ\u{cbf}ಯ"),
            ("ko", "사우디아라비아"),
            ("ku", "Si'ûdî Erebistan"),
            ("kv", "Саудса Аравия"),
            ("kw", "Arabi Saoudek"),
            ("ky", "Сауд Арабия Падышалыгы"),
            ("lo", "Saudi Arabia"),
            ("lt", "Saudo Arabija"),
            ("lv", "Saūda Arābija"),
            ("mi", "Hauri Arāpia"),
            ("mk", "Саудиска Арабија"),
            ("ml", "സൌദി അറേബ\u{d4d}യ"),
            ("mn", "Саудын араб"),
            ("mr", "सौदी अर\u{947}बिया"),
            ("ms", "Arab Saudi"),
            ("mt", "Għarabja Sawdita"),
            (
                "my",
                "ဆော\u{103a}ဒ\u{102e}အာရေဗျန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Taudiarabiya"),
            ("nb", "Saudi-Arabia"),
            ("ne", "साउदी अरब"),
            ("nl", "Saoedi-Arabië"),
            ("nn", "Saudi-Arabia"),
            ("nv", "Ásáí Dineʼé Bikéyah Ntsaaígíí"),
            ("oc", "Arabia saudita"),
            ("or", "ସ\u{b3e}ଉଦୀ ଆରବ"),
            ("pa", "ਸਾਊਦੀ ਅਰਬ"),
            ("pi", "सऊदी अरब"),
            ("pl", "Arabia Saudyjska"),
            ("ps", "سعودی عربستان"),
            ("pt", "Arábia Saudita"),
            ("pt_BR", "Arábia Saudita"),
            ("ro", "Arabia Saudită"),
            ("ru", "Саудовская Аравия"),
            ("rw", "Arabiya Sawudite"),
            ("sc", "Aràbia Saudita"),
            ("sd", "سعودي عرب"),
            ("si", "සව\u{dd4}ද\u{dd2} අර\u{dcf}බ\u{dd2}ය"),
            ("sk", "Saudská Arábia"),
            ("sl", "Saudova Arabija"),
            ("so", "Sacuudi Carabiya"),
            ("sq", "Arabia Saudite"),
            ("sr", "Саудијска Арабија"),
            ("sv", "Saudiarabien"),
            ("sw", "Saudi Arabia"),
            ("ta", "சவூதி அரேபிய\u{bbe}"),
            ("te", "స\u{c4c}ద\u{c40} అర\u{c47}బ\u{c3f}య\u{c3e}"),
            ("tg", "Арабистони Саудӣ"),
            ("th", "ซาอ\u{e38}ด\u{e35}อาระเบ\u{e35}ย"),
            ("ti", "ሰዑዲ ዓረብ"),
            ("tk", "Saud Arawiýa"),
            ("tl", "Saudi Arabia"),
            ("tr", "Suudi Arabistan"),
            ("tt", "Сөгүд Гәрәбстан"),
            ("ug", "سەئۇدى ئەرەبىستان"),
            ("uk", "Саудівська Аравія"),
            ("ur", "سعودی عرب"),
            ("uz", "Saudiya Arabistoni"),
            ("ve", "Saudi Arabia"),
            ("vi", "A-rập Xau-đi"),
            ("wa", "Arabeye Sawoudite"),
            ("wo", "Araabi Sawdit"),
            ("xh", "Saudi Arabia"),
            ("yo", "Sáúdí Arábíà"),
            ("zh_CN", "沙特阿拉伯"),
            ("zh_HK", "沙地阿拉伯"),
            ("zh_TW", "沙烏地阿拉伯"),
            ("zu", "Saudi Arabia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
