// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Panamá

#[cfg(all(feature = "pa", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PA;
    pub const ALPHA3: Alpha3 = Alpha3::PAN;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 507;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::PAB;
    pub const GEC: Option<GEC> = Some(GEC::PM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::PAN);
    pub const ISO_SHORT_NAME: &str = "Panama";
    pub const ISO_LONG_NAME: &str = "The Republic of Panamá";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Panamanian");
    pub const NUMBER: &str = "591";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "PA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Panama", "Panamá", "パナマ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Panama"),
        ("af", "Panama"),
        ("ak", "Panama"),
        ("am", "ፓናማ"),
        ("an", "Panama"),
        ("ar", "بنما"),
        ("as", "প\u{9be}ন\u{9be}ম\u{9be}"),
        ("ay", "Panama"),
        ("az", "Panama"),
        ("ba", "Panama"),
        ("be", "Панама"),
        ("bg", "Панама"),
        ("bi", "Panama"),
        ("bn", "প\u{9be}ন\u{9be}ম\u{9be}"),
        ("bn_IN", "প\u{9be}ন\u{9be}ম\u{9be}"),
        ("br", "Panama"),
        ("bs", "Panama"),
        ("ca", "Panamà"),
        ("ce", "Панама"),
        ("ch", "Panama"),
        ("cs", "Panama"),
        ("cv", "Панама"),
        ("cy", "Panama"),
        ("da", "Panama"),
        ("de", "Panama"),
        ("dv", "ޕ\u{7ac}ނ\u{7a6}މ\u{7a7}"),
        ("dz", "པ་ན་མ།"),
        ("ee", "Panama"),
        ("el", "Παναμάς"),
        ("en", "Panama"),
        ("eo", "Panamo"),
        ("es", "Panamá"),
        ("et", "Panama"),
        ("eu", "Panama"),
        ("fa", "پاناما"),
        ("ff", "Panama"),
        ("fi", "Panama"),
        ("fo", "Panama"),
        ("fr", "Panama"),
        ("fy", "Panama"),
        ("ga", "Panama"),
        ("gl", "Panamá"),
        ("gn", "Panama"),
        ("gu", "પનામા"),
        ("gv", "Yn Phanamaa"),
        ("ha", "Panama"),
        ("he", "פנמה"),
        ("hi", "पनामा"),
        ("hr", "Panama"),
        ("ht", "Panama"),
        ("hu", "Panama"),
        ("hy", "Պանամա"),
        ("ia", "Panama"),
        ("id", "Panama"),
        ("io", "Panama"),
        ("is", "Panama"),
        ("it", "Panama"),
        ("iu", "Panama"),
        ("ja", "パナマ"),
        ("ka", "პანამა"),
        ("ki", "Panama"),
        ("kk", "Панама"),
        ("kl", "Panama"),
        ("km", "ប\u{17c9}ាណាម\u{17c9}ា"),
        ("kn", "ಪನಾಮಾ"),
        ("ko", "파나마"),
        ("ku", "Panama"),
        ("kv", "Панама"),
        ("kw", "Panama"),
        ("ky", "Панама"),
        ("lo", "Panama"),
        ("lt", "Panama"),
        ("lv", "Panama"),
        ("mi", "Panama"),
        ("mk", "Панама"),
        ("ml", "പന\u{d3e}മ"),
        ("mn", "Панама"),
        ("mr", "पनामा"),
        ("ms", "Panama"),
        ("mt", "Panama"),
        ("my", "ပနားမားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Panama"),
        ("nb", "Panama"),
        ("ne", "पानामा"),
        ("nl", "Panama"),
        ("nn", "Panama"),
        ("nv", "Panama"),
        ("oc", "Panamà"),
        ("or", "ପ\u{b3e}ନ\u{b3e}ମ\u{b3e}"),
        ("pa", "ਪ\u{a48}ਨਾਮਾ"),
        ("pi", "पानामा"),
        ("pl", "Panama"),
        ("ps", "پاناما"),
        ("pt", "Panamá"),
        ("pt_BR", "Panamá"),
        ("ro", "Panama"),
        ("ru", "Панама"),
        ("rw", "Panama"),
        ("sc", "Pànama"),
        ("sd", "پاناما"),
        ("si", "පැනම\u{dcf}ව"),
        ("sk", "Panama"),
        ("sl", "Panama"),
        ("so", "Panama"),
        ("sq", "Panama"),
        ("sr", "Панама"),
        ("sv", "Panama"),
        ("sw", "Panama"),
        ("ta", "பன\u{bbe}ம\u{bbe}"),
        ("te", "పన\u{c3e}మ\u{c3e}"),
        ("tg", "Панама"),
        ("th", "ปานามา"),
        ("ti", "ፓናማ"),
        ("tk", "Panama"),
        ("tl", "Panama"),
        ("tr", "Panama"),
        ("tt", "Панама"),
        ("ug", "پاناما"),
        ("uk", "Панама"),
        ("ur", "پاناما"),
        ("uz", "Panama"),
        ("ve", "Panama"),
        ("vi", "Pa-na-ma"),
        ("wa", "Panama"),
        ("wo", "Panamaa"),
        ("xh", "Panama"),
        ("yo", "Panamá"),
        ("zh_CN", "巴拿马"),
        ("zh_HK", "巴拿馬"),
        ("zh_TW", "巴拿馬"),
        ("zu", "Panama"),
    ];
    #[cfg(all(feature = "pa", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 8.537981;
        pub const LONGITUDE: f64 = -80.782127;
        pub const MAX_LATITUDE: f64 = 9.7145001;
        pub const MAX_LONGITUDE: f64 = -77.1584879;
        pub const MIN_LATITUDE: f64 = 7.0409;
        pub const MIN_LONGITUDE: f64 = -83.05224109999999;
        pub const NORTHEAST_LATITUDE: f64 = 9.7145001;
        pub const NORTHEAST_LONGITUDE: f64 = -77.1584879;
        pub const SOUTHWEST_LATITUDE: f64 = 7.0409;
        pub const SOUTHWEST_LONGITUDE: f64 = -83.05224109999999;
    }
}
#[cfg(all(feature = "pa", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 8.537981,
            longitude: -80.782127,
            max_latitude: 9.7145001,
            max_longitude: -77.1584879,
            min_latitude: 7.0409,
            min_longitude: -83.05224109999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 9.7145001,
                    longitude: -77.1584879,
                },
                southwest: CountryGeoBound {
                    latitude: 7.0409,
                    longitude: -83.05224109999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "pa", feature = "subdivisions"))]
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
                    "1",
                    Subdivision{
                        name: "1",
                        country_alpha2: Alpha2::PA,
                        code: "1",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.340556), longitude: Some(-82.240556), max_latitude: Some(9.453332), min_latitude: Some(9.3292808), max_longitude: Some(-82.2274304), min_longitude: Some(-82.33235839999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بوكاس ديل تورو"), ("be", "Правінцыя Бокас-дэль-Тора"), ("bg", "Бокас дел Торо"), ("bn", "বোক\u{9be}স ডেল টোরো প\u{9cd}রদেশ"), ("ca", "Província de Bocas del Toro"), ("ccp", "𑄝\u{11127}𑄇𑄌\u{11134} 𑄓𑄬𑄣\u{11134} 𑄑\u{1112e}𑄢\u{1112e}"), ("ceb", "Provincia de Bocas del Toro"), ("cs", "Bocas del Toro"), ("da", "Bocas del Toro Province"), ("de", "Provinz Bocas del Toro"), ("el", "Μπόκας ντελ Τόρο"), ("en", "Bocas del Toro"), ("es", "Provincia de Bocas del Toro"), ("eu", "Bocas del Toroko probintzia"), ("fa", "استان بوکاس دل تورو"), ("fi", "Bocas del Toro"), ("fr", "Bocas del Toro"), ("gu", "બોકાસ ડ\u{ac7}લ ટોરો પ\u{acd}રા\u{a82}ત"), ("hi", "बोकास ड\u{947}ल टोरो प\u{94d}रा\u{902}त"), ("id", "Provinsi Bocas del Toro"), ("it", "provincia di Bocas del Toro"), ("ja", "ボカス・デル・トーロ県"), ("ka", "ბოკას-დელ-ტოროს პროვინცია"), ("kn", "ಬೋಕಾಸ\u{ccd} ಡ\u{cc6}ಲ\u{ccd} ಟೊರೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "보카스델토로 주"), ("lt", "Bokas del Toro provincija"), ("lv", "Bokasa del Toro"), ("mr", "बोकास ड\u{947}ल टोरो प\u{94d}रा\u{902}त"), ("ms", "Bocas del Toro Province"), ("nb", "Bocas del Toro"), ("nl", "Bocas del Toro"), ("no", "Bocas del Toro"), ("pl", "Bocas del Toro"), ("pt", "Bocas del Toro"), ("ro", "Provincia Bocas del Toro"), ("ru", "Бокас-дель-Торо"), ("si", "බොක\u{dcf}ස\u{dca} ඩෙල\u{dca} ටොරෝ පළ\u{dcf}ත"), ("sv", "Bocas del Toro"), ("ta", "போகஸ\u{bcd} டெல\u{bcd} டோரோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4b}క\u{c3e}స\u{c4d} డ\u{c46}ల\u{c4d} ట\u{c4b}ర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโบคาส เดล โตโร"), ("tr", "Bocas del Toro"), ("uk", "Бокас-дель-Торо"), ("ur", "بوکاس دیل تورو صوبہ"), ("vi", "Bocas del Toro"), ("zh", "博卡斯德爾托羅省")]),
                        unofficial_name_list: ["Bocas del Toro"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::PA,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Панама Оесте"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄛𑄚𑄟"), ("cs", "Západní Panama"), ("de", "Provinz Panamá Oeste"), ("en", "West Panamá"), ("es", "Provincia de Panamá Oeste"), ("eu", "Mendebaldeko Panama"), ("fa", "استان پاناما اوسته"), ("fr", "Panama Ouest"), ("hy", "Արևմտյան Պանամա"), ("ja", "西パナマ県"), ("ko", "서파나마 주"), ("lv", "Rietumpanama"), ("nl", "Panama Oeste"), ("pl", "Panama Zachodnia"), ("ru", "Западная Панама"), ("uk", "Західна Панама"), ("ur", "پاناما غربی صوبہ"), ("zh", "西巴拿馬省")]),
                        unofficial_name_list: ["Panamá Oeste"].to_vec(),
                    }
                ),
                (
                    "2",
                    Subdivision{
                        name: "2",
                        country_alpha2: Alpha2::PA,
                        code: "2",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.400144899999999), longitude: Some(-80.42152460000001), max_latitude: Some(8.4878652), min_latitude: Some(8.3281104), max_longitude: Some(-80.3550767), min_longitude: Some(-80.482192)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوكلتي"), ("bg", "Кокле"), ("bn", "কোক\u{9cd}লে প\u{9cd}রদেশ"), ("ca", "Província de Coclé"), ("ccp", "𑄇\u{1112e}𑄇\u{11134}𑄣𑄬"), ("ceb", "Provincia de Coclé"), ("cs", "Coclé"), ("da", "Coclé Province"), ("de", "Provinz Coclé"), ("el", "Κοκλέ"), ("en", "Coclé"), ("es", "Provincia de Coclé"), ("eu", "Cocle"), ("fa", "استان کوکله"), ("fi", "Coclén provinssi"), ("fr", "Coclé"), ("gu", "કોક\u{acd}લ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "कोस\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Coclé"), ("it", "provincia di Coclé"), ("ja", "コクレ県"), ("ka", "კოკლეს პროვინცია"), ("kn", "ಕೊಕ\u{ccd}ಲ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코클레 주"), ("lt", "Koklės provincija"), ("lv", "Kokle"), ("mr", "कोक\u{94d}ल\u{947} प\u{94d}रा\u{902}त"), ("ms", "Cocle Province"), ("nb", "Cocle provins"), ("nl", "Coclé"), ("no", "Cocle provins"), ("pl", "Coclé"), ("pt", "Coclé"), ("ro", "Provincia Coclé"), ("ru", "Кокле"), ("si", "කොක\u{dca}ලේ පළ\u{dcf}ත"), ("sv", "Coclé"), ("ta", "கோகிலே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}కల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคเค\u{e34}ล"), ("tr", "Coclé Province"), ("uk", "Кокле"), ("ur", "کوکل صوبہ"), ("vi", "Tỉnh Coclé"), ("zh", "科克萊省")]),
                        unofficial_name_list: ["Coclé"].to_vec(),
                    }
                ),
                (
                    "3",
                    Subdivision{
                        name: "3",
                        country_alpha2: Alpha2::PA,
                        code: "3",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.333333), longitude: Some(-79.89999999999999), max_latitude: Some(9.4046939), min_latitude: Some(9.242330299999999), max_longitude: Some(-79.83102629999999), min_longitude: Some(-80.04467009999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كولون"), ("be", "Калон"), ("bg", "Колон"), ("bn", "কোলন প\u{9cd}রদেশ"), ("ca", "Província de Colón"), ("ccp", "𑄇\u{11127}𑄣\u{1112e}𑄚\u{11134}"), ("ceb", "Provincia de Colón"), ("cs", "Colón"), ("da", "Colón Province"), ("de", "Provinz Colón"), ("el", "Κολόν"), ("en", "Colón"), ("es", "Provincia de Colón"), ("eu", "Colongo probintzia"), ("fi", "Colón"), ("fr", "Colón"), ("gu", "કોલોન પ\u{acd}રા\u{a82}ત"), ("he", "קולון"), ("hi", "कोलोन प\u{94d}रा\u{902}त"), ("hy", "Կոլոն"), ("id", "Provinsi Colón"), ("it", "provincia di Colón"), ("ja", "コロン県"), ("ka", "კოლონის პროვინცია"), ("kn", "ಕೊಲೊನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "콜론 주"), ("lt", "Kolono provincija"), ("lv", "Kolona"), ("mr", "कोलोन प\u{94d}रा\u{902}त"), ("ms", "Colon Province"), ("nb", "Colon provins"), ("nl", "Colón"), ("no", "Colon provins"), ("pl", "Colón"), ("pt", "Colón"), ("ro", "Provincia Colón"), ("ru", "Колон"), ("si", "කොලෝන\u{dca} පළ\u{dcf}ත"), ("sv", "Colón"), ("ta", "கோலோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4a}ల\u{c4b}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "โคโลน"), ("tr", "Colon Province"), ("uk", "Колон"), ("ur", "کولون صوبہ"), ("vi", "Tỉnh Colón"), ("zh", "科隆省")]),
                        unofficial_name_list: ["Colón"].to_vec(),
                    }
                ),
                (
                    "4",
                    Subdivision{
                        name: "4",
                        country_alpha2: Alpha2::PA,
                        code: "4",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.3866964), longitude: Some(-82.2800546), max_latitude: Some(8.4415651), min_latitude: Some(8.2904865), max_longitude: Some(-82.2074748), min_longitude: Some(-82.3967314)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيريكي"), ("bg", "Чирики"), ("bn", "চিরিক\u{9c1}ই প\u{9cd}রদেশ"), ("ca", "Província de Chiriquí"), ("ccp", "𑄥\u{11128}𑄢\u{11128}𑄇\u{1112d}\u{1112a}"), ("ceb", "Provincia de Chiriquí"), ("cs", "Chiriquí"), ("da", "Chiriquí Province"), ("de", "Provinz Chiriquí"), ("el", "Τσιρίκι"), ("en", "Chiriquí"), ("es", "Provincia de Chiriquí"), ("eu", "Chiriqui"), ("fa", "استان چیریکی"), ("fi", "Chiriquí"), ("fr", "Chiriquí"), ("gu", "ચિરિકી પ\u{acd}રા\u{a82}ત"), ("hi", "चिरिकी प\u{94d}रा\u{902}त"), ("id", "Provinsi Chiriquí"), ("it", "provincia di Chiriquí"), ("ja", "チリキ県"), ("ka", "ჩირიკის პროვინცია"), ("kn", "ಚ\u{cbf}ರ\u{ccd}ವ\u{cbf}ವ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "치리키 주"), ("lt", "Čiriki provincija"), ("lv", "Čiriki"), ("mr", "चिरिकि प\u{94d}रा\u{902}त"), ("ms", "Chiriqui Province"), ("nb", "Chiriqui Prrvins"), ("nl", "Chiriquí"), ("no", "Chiriqui Prrvins"), ("pl", "Chiriquí"), ("pt", "Chiriquí"), ("ro", "Provincia Chiriquí"), ("ru", "Чирики"), ("si", "ච\u{dd2}ර\u{dd2}ක\u{dd2} පළ\u{dcf}ත"), ("sv", "Chiriquí"), ("ta", "சிறிகுய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3f}ర\u{c3f}క\u{c4d}వ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ด ไคร\u{e34}ไคว\u{e4c}"), ("tr", "Chiriqui Province"), ("uk", "Чирики"), ("ur", "چیرکی صوبہ"), ("vi", "Tỉnh Chiriquí"), ("zh", "奇里基省")]),
                        unofficial_name_list: ["Chiriquí"].to_vec(),
                    }
                ),
                (
                    "5",
                    Subdivision{
                        name: "5",
                        country_alpha2: Alpha2::PA,
                        code: "5",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.868171299999999), longitude: Some(-77.8367282), max_latitude: Some(8.9348909), min_latitude: Some(7.221865600000001), max_longitude: Some(-77.17411), min_longitude: Some(-78.511646)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة داريان"), ("bg", "Дариен"), ("bn", "ডোরিয\u{9bc}েন প\u{9cd}রদেশ"), ("ca", "Província de Darién"), ("ccp", "𑄓𑄢\u{11128}𑄠𑄬𑄚\u{11134}"), ("ceb", "Provincia del Darién"), ("cs", "Darién"), ("da", "Darién Province"), ("de", "Provinz Darién"), ("el", "Νταριέν"), ("en", "Darién"), ("es", "Provincia de Darién"), ("eu", "Darien"), ("fa", "استان دارئین"), ("fi", "Dariénin lääni"), ("fr", "Darién"), ("gu", "ડ\u{ac7}ર\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "ड\u{948}रिएन प\u{94d}रा\u{902}त"), ("id", "Provinsi Darién"), ("it", "provincia di Darién"), ("ja", "ダリエン県"), ("ka", "დარიენის პროვინცია"), ("kn", "ಡೇರ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "다리엔 주"), ("lt", "Darjeno provincija"), ("lv", "Darjena"), ("mr", "ड\u{945}रिएन प\u{94d}रा\u{902}त"), ("ms", "Darien Province"), ("nb", "Darien provins"), ("nl", "Darién"), ("no", "Darien provins"), ("pl", "Darién"), ("pt", "Darién"), ("ro", "Provincia Darién"), ("ru", "Дарьен"), ("si", "ඩර\u{dd2}යන\u{dca} පළ\u{dcf}ත"), ("sr", "Даријен"), ("sr_Latn", "Darijen"), ("sv", "Darién"), ("ta", "டைரியின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c3e}ర\u{c3f}య\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดาเร\u{e35}ยน"), ("tr", "Darien Province"), ("uk", "Дарʼєн"), ("ur", "داریئن صوبہ"), ("vi", "Tỉnh Darién"), ("zh", "達連省")]),
                        unofficial_name_list: ["Darién"].to_vec(),
                    }
                ),
                (
                    "6",
                    Subdivision{
                        name: "6",
                        country_alpha2: Alpha2::PA,
                        code: "6",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.7704282), longitude: Some(-80.7214417), max_latitude: Some(8.131537), min_latitude: Some(7.513487), max_longitude: Some(-80.3965529), min_longitude: Some(-80.96242)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هيريرا"), ("bg", "Ерера"), ("bn", "হেরের\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Herrera"), ("ccp", "𑄦𑄬𑄢𑄢"), ("ceb", "Provincia de Herrera"), ("cs", "Herrera"), ("da", "Herrera Province"), ("de", "Provinz Herrera"), ("el", "Χερέρα"), ("en", "Herrera"), ("es", "Provincia de Herrera"), ("eu", "Herrerako probintzia"), ("fa", "استان اررا"), ("fi", "Herreran lääni"), ("fr", "Herrera"), ("gu", "હ\u{ac7}ર\u{ac7}રા પ\u{acd}રા\u{a82}ત"), ("hi", "ह\u{947}र\u{947}रा प\u{94d}रा\u{902}त"), ("id", "Provinsi Herrera"), ("it", "provincia di Herrera"), ("ja", "エレーラ県"), ("ka", "ერერას პროვინცია"), ("kn", "ಹ\u{cc6}ರ\u{cc6}ರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "에레라 주"), ("lt", "Hereros provincija"), ("lv", "Errera"), ("mr", "ह\u{947}र\u{947}रा प\u{94d}रा\u{902}त"), ("ms", "Herrera Province"), ("nb", "Herrera Kommune"), ("nl", "Herrera"), ("no", "Herrera Kommune"), ("pl", "Herrera"), ("pt", "Herrera"), ("ro", "Provincia Herrera"), ("ru", "Эррера"), ("si", "හෙර\u{dca}රෙර\u{dcf} පළ\u{dcf}ත"), ("sv", "Herrera"), ("ta", "ஹெற\u{bc0}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హ\u{c46}ర\u{c4d}ర\u{c46}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฮอร\u{e4c}เรรา"), ("tr", "Herrera Province"), ("uk", "Еррера"), ("ur", "ہیریرا صوبہ"), ("vi", "Tỉnh Herrera"), ("zh", "埃雷拉省")]),
                        unofficial_name_list: ["Herrera"].to_vec(),
                    }
                ),
                (
                    "7",
                    Subdivision{
                        name: "7",
                        country_alpha2: Alpha2::PA,
                        code: "7",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.5909302), longitude: Some(-80.365865), max_latitude: Some(8.000313), min_latitude: Some(7.2297201), max_longitude: Some(-79.9944531), min_longitude: Some(-80.734346)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لوس سانتوس"), ("bg", "Лос Сантос"), ("bn", "লস স\u{9be}ন\u{9cd}টোস প\u{9cd}রদেশ"), ("ca", "Província de Los Santos"), ("ccp", "𑄣\u{11127}𑄌\u{11134} 𑄥𑄚\u{11134}𑄑\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Los Santos"), ("cs", "Los Santos"), ("da", "Los Santos Province"), ("de", "Provinz Los Santos"), ("el", "Λος Σάντος"), ("en", "Los Santos"), ("es", "Provincia de Los Santos"), ("eu", "Los Santosgo probintzia"), ("fa", "استان لوس سانتوس"), ("fi", "Los Santosin lääni"), ("fr", "Los Santos"), ("gu", "લોસ સાન\u{acd}ટોસ પ\u{acd}રા\u{a82}ત"), ("hi", "लॉस स\u{948}\u{902}टोस प\u{94d}रा\u{902}त"), ("id", "Provinsi Los Santos"), ("it", "provincia di Los Santos"), ("ja", "ロス・サントス県"), ("ka", "ლოს-სანტოსის პროვინცია"), ("kn", "ಲಾಸ\u{ccd} ಸ\u{ccd}ಯಾಂಟೋಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로스산토스 주"), ("lt", "Los Santoso provincija"), ("lv", "Lossantosa"), ("mr", "लॉस सा\u{902}तोस प\u{94d}रा\u{902}त"), ("ms", "Los Santos Province"), ("nb", "Los Santos Provins"), ("nl", "Los Santos"), ("no", "Los Santos Provins"), ("pl", "Los Santos"), ("pt", "Los Santos"), ("ro", "Provincia Los Santos"), ("ru", "Лос-Сантос"), ("si", "ලොස\u{dca} සැන\u{dca}ටෝස\u{dca} පළ\u{dcf}ත"), ("sv", "Los Santos"), ("ta", "ல\u{bbe}ஸ\u{bcd} சந\u{bcd}தோஷ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}స\u{c4d} స\u{c3e}ంట\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลอสซานโตส"), ("tr", "Los Santos Province"), ("uk", "Лос-Сантос"), ("ur", "لاس سانتوس صوبہ"), ("vi", "Tỉnh Los Santos"), ("zh", "洛斯桑托斯省")]),
                        unofficial_name_list: ["Los Santos"].to_vec(),
                    }
                ),
                (
                    "8",
                    Subdivision{
                        name: "8",
                        country_alpha2: Alpha2::PA,
                        code: "8",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.983333), longitude: Some(-79.516667), max_latitude: Some(9.2596966), min_latitude: Some(8.906754), max_longitude: Some(-79.3068941), min_longitude: Some(-79.5978646)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Panama əyaləti"), ("be", "правінцыя Панама"), ("bg", "Панама"), ("ca", "Província de Panamà"), ("ccp", "𑄛𑄚𑄟"), ("ceb", "Provincia de Panamá"), ("cs", "Panama"), ("de", "Provinz Panamá"), ("en", "Panamá"), ("es", "Provincia de Panamá"), ("eu", "Panamako probintzia"), ("fa", "استان پاناما"), ("fr", "Panama"), ("hy", "Պանամա նահանգ"), ("id", "Provinsi Panamá"), ("it", "provincia di Panama"), ("ja", "パナマ県"), ("ka", "პანამის პროვინცია"), ("ko", "파나마 주"), ("lt", "Panamos provincija"), ("lv", "Panama"), ("nb", "Panamá"), ("nl", "Panama"), ("no", "Panamá"), ("pl", "Panama"), ("pt", "Panamá"), ("ro", "Provincia Panamá"), ("ru", "Панама"), ("sv", "Panamá"), ("th", "จ\u{e31}งหว\u{e31}ดปานามา"), ("tr", "Panama ili"), ("uk", "Панама"), ("ur", "پاناما صوبہ"), ("vi", "Panamá"), ("zh", "巴拿馬省")]),
                        unofficial_name_list: ["Panamá"].to_vec(),
                    }
                ),
                (
                    "9",
                    Subdivision{
                        name: "9",
                        country_alpha2: Alpha2::PA,
                        code: "9",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.866667), longitude: Some(-80.89999999999999), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فيراغواس"), ("bg", "Верагуас"), ("bn", "ভের\u{9be}গ\u{9be}স প\u{9cd}রদেশ"), ("ca", "Província de Veraguas"), ("ccp", "𑄞𑄢\u{11134}𑄉𑄅\u{1112a}𑄌\u{11134}"), ("ceb", "Provincia de Veraguas"), ("cs", "Veraguas"), ("da", "Veraguas Province"), ("de", "Provinz Veraguas"), ("el", "Βεραγκούας"), ("en", "Veraguas"), ("es", "Provincia de Veraguas"), ("eu", "Veraguas"), ("fa", "ورگواس پروینک"), ("fi", "Veraguasin lääni"), ("fr", "Veraguas"), ("gu", "વ\u{ac7}રાગ\u{ac1}સ પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{947}रग\u{941}आस प\u{94d}रा\u{902}त"), ("hy", "Վերագուաս"), ("id", "Provinsi Veraguas"), ("it", "provincia di Veraguas"), ("ja", "ベラグアス県"), ("ka", "ვერაგუასის პროვინცია"), ("kn", "ವ\u{cc6}ರ\u{ccd}ಗುವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "베라과스 주"), ("lt", "Veragvaso provincija"), ("lv", "Veragvasa"), ("mr", "वर\u{941}ग\u{941}आस प\u{94d}रा\u{902}त"), ("ms", "Veraguas Province"), ("nb", "Veraguas provins"), ("nl", "Veraguas"), ("no", "Veraguas provins"), ("pl", "Veraguas"), ("pt", "Veraguas"), ("ro", "Provincia Veraguas"), ("ru", "Верагуас"), ("si", "වෙර\u{dcf}ග\u{dd4}ආස\u{dca} පළ\u{dcf}ත"), ("sv", "Veraguas"), ("ta", "வேர\u{bbe}குவஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}ర\u{c3e}గ\u{c4d}వస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวราก\u{e38}อ\u{e31}ส"), ("tr", "Veraguas Province"), ("uk", "Вераґуас"), ("ur", "ویراگواس صوبہ"), ("vi", "Veraguas"), ("zh", "貝拉瓜斯省")]),
                        unofficial_name_list: ["Veraguas"].to_vec(),
                    }
                ),
                (
                    "EM",
                    Subdivision{
                        name: "EM",
                        country_alpha2: Alpha2::PA,
                        code: "EM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::IndigenousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوماركا امبيرا-وونان"), ("bg", "Ембера"), ("bn", "এম\u{9cd}বের\u{9be}-উন\u{9be}ন ক\u{9be}ম\u{9be}রক\u{9cd}য\u{9be}"), ("ca", "Emberá-Wounaan"), ("ccp", "𑄃𑄬𑄟𑄝𑄢\u{11134}"), ("ceb", "Emberá-Wounaan"), ("cs", "Emberá-Wounaan"), ("da", "Emberá-Wounaan Comarca"), ("de", "Emberá-Wounaan"), ("el", "Εμπερά-Γουναάν"), ("en", "Emberá"), ("es", "Emberá-Wounaan"), ("eu", "Emberá-Wounaan"), ("fi", "Emberá-Wounaan Comarca"), ("fr", "Comarca Emberá-Wounaan"), ("gu", "એમ\u{acd}બરા-વોઉનાન કોમાર\u{acd}કા"), ("hi", "एम\u{94d}ब\u{947}रा-व\u{942}नान कोमारका"), ("hy", "Էմբերա Վոունաան"), ("id", "Emberá-Wounaan Comarca"), ("it", "Emberá-Wounaan"), ("ja", "エンベラ自治区"), ("ka", "ემბერა-ვოუნაანი"), ("kn", "ಎಬ\u{cc6}ರಾ-ವ\u{ccc}ನಾನ\u{ccd} ಕೊಮಾರ\u{ccd}ಕಾ"), ("ko", "엠베라워우난 특구"), ("lt", "Embera Vounaanas"), ("lv", "Embera-Vounaana"), ("mr", "एम\u{94d}बर\u{945}-वोउनान कॉमार\u{94d}च"), ("ms", "Embera-Wounaan Comarca"), ("nb", "Embera Wounaan Comarca"), ("nl", "Emberá-Wounaan"), ("no", "Embera Wounaan Comarca"), ("pl", "Emberá-Wounaan"), ("pt", "Emberá"), ("ro", "Emberá-Wounaan"), ("ru", "Эмбера-Воунаан"), ("si", "එම\u{dca}බෙර\u{dcf} -ව\u{dd4}න\u{dcf}න\u{dca} කොමර\u{dca}ක\u{dcf}"), ("sv", "Emberá-Wounaan"), ("ta", "எம\u{bcd}பெற\u{bbe} -ஒயின\u{bbe}ன\u{bcd} க\u{bbe}மரக\u{bcd}க\u{bbe}"), ("te", "ఎంబ\u{c46}ర\u{c3e}-వ\u{c4b}న\u{c3e}న\u{c4d} క\u{c3e}మర\u{c4d}స\u{c3e}"), ("th", "เอมเบอรา-ว\u{e39}นาน โคมาร\u{e4c}คา"), ("tr", "Embera-Wounaa Comarca"), ("uk", "Ембера-Воунаан"), ("ur", "کومارکا امبیرا-ووناو"), ("vi", "Emberá-Wounaan Comarca"), ("zh", "安貝拉自治區")]),
                        unofficial_name_list: ["Emberá-Wounaan"].to_vec(),
                    }
                ),
                (
                    "KY",
                    Subdivision{
                        name: "KY",
                        country_alpha2: Alpha2::PA,
                        code: "KY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.537981), longitude: Some(-80.782127), max_latitude: Some(9.6477792), min_latitude: Some(7.203556400000001), max_longitude: Some(-77.15848799999999), min_longitude: Some(-83.05224109999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::IndigenousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غونا يالا"), ("bg", "Куна Яла"), ("bn", "গ\u{9c1}ন\u{9be} ইয\u{9bc}\u{9be}ল\u{9be}"), ("ca", "Kuna Yala"), ("ccp", "𑄉\u{1112a}𑄚 𑄃\u{11128}𑄠𑄣"), ("ceb", "Guna Yala"), ("cs", "Kuna Yala"), ("da", "Guna Yala"), ("de", "Guna Yala"), ("el", "Γκούνα Γιάλα"), ("en", "Guna Yala"), ("es", "Guna Yala"), ("eu", "Guna Yala"), ("fi", "Guna Yala"), ("fr", "Kuna Yala"), ("gu", "ગ\u{ac1}ના યલા"), ("hi", "ग\u{941}ना याला"), ("hy", "Կունա Յալա"), ("id", "Guna Yala"), ("it", "Kuna Yala"), ("ja", "クナ・ヤラ自治区"), ("ka", "გუნა-იალა"), ("kn", "ಗುನಾ ಯಲಾ"), ("ko", "구나얄라 특구"), ("lt", "Kuna Jala"), ("lv", "Gunajala"), ("mr", "ग\u{941}ना यला"), ("ms", "Guna Yala"), ("nb", "Kuna Yala"), ("nl", "Kuna Yala"), ("no", "Kuna Yala"), ("pl", "Kuna Yala"), ("pt", "Kuna Yala"), ("ro", "Guna Yala"), ("ru", "Куна-Яла"), ("si", "ග\u{dd4}ණ ය\u{dcf}ල"), ("sl", "Kuna Yala"), ("sv", "Guna Yala"), ("ta", "குண\u{bbe} ய\u{bbe}ள\u{bbe}"), ("te", "గున\u{c3e} య\u{c3e}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e39}นายาลา"), ("tr", "Guna Yala"), ("uk", "Гуна-Яла"), ("ur", "گونا یالا"), ("vi", "Guna Yala"), ("zh", "雅拉庫納族自治區")]),
                        unofficial_name_list: ["Guna Yala"].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::PA,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::IndigenousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نجوب-بولجي كوماركا"), ("bg", "Нгьобе-Бугле"), ("bn", "ন\u{9be}গ\u{9c1}বে বিউগল ক\u{9c1}ম\u{9be}ক\u{9be}"), ("ca", "Ngöbe-Buglé"), ("ccp", "𑄉\u{1112e}𑄝𑄬-𑄝𑄉\u{11127}𑄣\u{11134}"), ("ceb", "Ngöbe-Buglé"), ("cs", "Ngöbe-Buglé"), ("da", "Ngöbe-Buglé Comarca"), ("de", "Ngöbe-Buglé"), ("el", "Νγκόμπε-Μπουγκλέ"), ("en", "Ngöbe-Buglé"), ("es", "Ngäbe-Buglé"), ("eu", "Ngäbe-Buglé"), ("fi", "Ngöbe-Buglé Comarca"), ("fr", "Ngöbe-Buglé"), ("gu", "એન\u{acd}ગોબ\u{ac7}-બ\u{ac1}ગલ કોમાર\u{acd}કા"), ("hi", "गोब-ब\u{94d}य\u{942}गल कोमारका"), ("hy", "Նգոբե Բուգլե"), ("id", "Ngöbe-Buglé Comarca"), ("it", "Ngäbe-Buglé"), ("ja", "ノベ・ブグレ自治区"), ("ka", "ნგობე-ბუგლე"), ("kn", "ನೊಬ\u{cc6}-ಬುಗ\u{ccd}ಲ\u{cc6} ಕೊಮಾರ\u{ccd}ಕಾ"), ("ko", "응가베부글레 특구"), ("lt", "Ngobė Buglė"), ("lv", "Ngobe-Bugle"), ("mr", "नोब\u{947}-ब\u{941}गल कोमारका"), ("ms", "Ngobe-Bugle Comarca"), ("nb", "Ngobe-Bugle Comarca"), ("nl", "Ngöbe-Buglé"), ("no", "Ngobe-Bugle Comarca"), ("pl", "Ngöbe-Buglé"), ("pt", "Ngöbe-Buglé"), ("ro", "Ngäbe-Buglé"), ("ru", "Нгобе-Бугле"), ("si", "එන\u{dca}ගෝබේ-බ\u{dd4}ග\u{dca}ලේ කොමර\u{dca}ක\u{dcf}"), ("sv", "Ngöbe-Buglé"), ("ta", "ங\u{bcd}கோபி-புக\u{bcd}லெ கோமரக\u{bcd}க\u{bbe}"), ("te", "ఎన\u{c4d}గ\u{c4b}బ\u{c4d} బూగుల\u{c4d} కమ\u{c3e}ర\u{c4d}స\u{c3e}"), ("th", "นโกบ\u{e35}-บ\u{e39}เก\u{e34}\u{e49}ล โคมาซา"), ("tr", "Ngöbe-Buglé County"), ("uk", "Нґобе-Буґле"), ("ur", "نگابے-بوگلے کومارکا"), ("vi", "Ngobe-Buglé Comarca"), ("zh", "恩戈貝布格勒自治區")]),
                        unofficial_name_list: ["Ngäbe-Buglé"].to_vec(),
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
#[cfg(feature = "pa")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PA,
        alpha3: Alpha3::PAN,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 507,
        currency_code: CurrencyCode::PAB,
        gec: Some(GEC::PM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::PAN),
        iso_long_name: "The Republic of Panamá",
        iso_short_name: "Panama",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Panamanian"),
        number: "591",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "PA",
        unofficial_name_list: ["Panama", "Panamá", "パナマ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Panama"),
            ("af", "Panama"),
            ("ak", "Panama"),
            ("am", "ፓናማ"),
            ("an", "Panama"),
            ("ar", "بنما"),
            ("as", "প\u{9be}ন\u{9be}ম\u{9be}"),
            ("ay", "Panama"),
            ("az", "Panama"),
            ("ba", "Panama"),
            ("be", "Панама"),
            ("bg", "Панама"),
            ("bi", "Panama"),
            ("bn", "প\u{9be}ন\u{9be}ম\u{9be}"),
            ("bn_IN", "প\u{9be}ন\u{9be}ম\u{9be}"),
            ("br", "Panama"),
            ("bs", "Panama"),
            ("ca", "Panamà"),
            ("ce", "Панама"),
            ("ch", "Panama"),
            ("cs", "Panama"),
            ("cv", "Панама"),
            ("cy", "Panama"),
            ("da", "Panama"),
            ("de", "Panama"),
            ("dv", "ޕ\u{7ac}ނ\u{7a6}މ\u{7a7}"),
            ("dz", "པ་ན་མ།"),
            ("ee", "Panama"),
            ("el", "Παναμάς"),
            ("en", "Panama"),
            ("eo", "Panamo"),
            ("es", "Panamá"),
            ("et", "Panama"),
            ("eu", "Panama"),
            ("fa", "پاناما"),
            ("ff", "Panama"),
            ("fi", "Panama"),
            ("fo", "Panama"),
            ("fr", "Panama"),
            ("fy", "Panama"),
            ("ga", "Panama"),
            ("gl", "Panamá"),
            ("gn", "Panama"),
            ("gu", "પનામા"),
            ("gv", "Yn Phanamaa"),
            ("ha", "Panama"),
            ("he", "פנמה"),
            ("hi", "पनामा"),
            ("hr", "Panama"),
            ("ht", "Panama"),
            ("hu", "Panama"),
            ("hy", "Պանամա"),
            ("ia", "Panama"),
            ("id", "Panama"),
            ("io", "Panama"),
            ("is", "Panama"),
            ("it", "Panama"),
            ("iu", "Panama"),
            ("ja", "パナマ"),
            ("ka", "პანამა"),
            ("ki", "Panama"),
            ("kk", "Панама"),
            ("kl", "Panama"),
            ("km", "ប\u{17c9}ាណាម\u{17c9}ា"),
            ("kn", "ಪನಾಮಾ"),
            ("ko", "파나마"),
            ("ku", "Panama"),
            ("kv", "Панама"),
            ("kw", "Panama"),
            ("ky", "Панама"),
            ("lo", "Panama"),
            ("lt", "Panama"),
            ("lv", "Panama"),
            ("mi", "Panama"),
            ("mk", "Панама"),
            ("ml", "പന\u{d3e}മ"),
            ("mn", "Панама"),
            ("mr", "पनामा"),
            ("ms", "Panama"),
            ("mt", "Panama"),
            ("my", "ပနားမားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Panama"),
            ("nb", "Panama"),
            ("ne", "पानामा"),
            ("nl", "Panama"),
            ("nn", "Panama"),
            ("nv", "Panama"),
            ("oc", "Panamà"),
            ("or", "ପ\u{b3e}ନ\u{b3e}ମ\u{b3e}"),
            ("pa", "ਪ\u{a48}ਨਾਮਾ"),
            ("pi", "पानामा"),
            ("pl", "Panama"),
            ("ps", "پاناما"),
            ("pt", "Panamá"),
            ("pt_BR", "Panamá"),
            ("ro", "Panama"),
            ("ru", "Панама"),
            ("rw", "Panama"),
            ("sc", "Pànama"),
            ("sd", "پاناما"),
            ("si", "පැනම\u{dcf}ව"),
            ("sk", "Panama"),
            ("sl", "Panama"),
            ("so", "Panama"),
            ("sq", "Panama"),
            ("sr", "Панама"),
            ("sv", "Panama"),
            ("sw", "Panama"),
            ("ta", "பன\u{bbe}ம\u{bbe}"),
            ("te", "పన\u{c3e}మ\u{c3e}"),
            ("tg", "Панама"),
            ("th", "ปานามา"),
            ("ti", "ፓናማ"),
            ("tk", "Panama"),
            ("tl", "Panama"),
            ("tr", "Panama"),
            ("tt", "Панама"),
            ("ug", "پاناما"),
            ("uk", "Панама"),
            ("ur", "پاناما"),
            ("uz", "Panama"),
            ("ve", "Panama"),
            ("vi", "Pa-na-ma"),
            ("wa", "Panama"),
            ("wo", "Panamaa"),
            ("xh", "Panama"),
            ("yo", "Panamá"),
            ("zh_CN", "巴拿马"),
            ("zh_HK", "巴拿馬"),
            ("zh_TW", "巴拿馬"),
            ("zu", "Panama"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
