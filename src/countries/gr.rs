// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Hellenic Republic

#[cfg(all(feature = "gr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::GR;
    pub const ALPHA3: Alpha3 = Alpha3::GRC;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 30;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::GR);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("GRE");
    pub const ISO_SHORT_NAME: &str = "Greece";
    pub const ISO_LONG_NAME: &str = "The Hellenic Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["el"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["el"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Greek");
    pub const NUMBER: &str = "300";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3} ?\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "GR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Greece",
        "Griechenland",
        "Grèce",
        "Grecia",
        "ギリシャ",
        "Griekenland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Greece"),
        ("af", "Griekeland"),
        ("ak", "Greece"),
        ("am", "ጔሱጤ"),
        ("an", "Greece"),
        ("ar", "اليونان"),
        ("as", "গ\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিকীচ"),
        ("ay", "Greece"),
        ("az", "Yunanıstan"),
        ("ba", "Greece"),
        ("be", "Грэцыя"),
        ("bg", "Гърция"),
        ("bi", "Greece"),
        ("bn", "গ\u{9cd}রিস"),
        ("bn_IN", "গ\u{9cd}রিস"),
        ("br", "Gres"),
        ("bs", "Grčka"),
        ("ca", "Grècia"),
        ("ce", "Грекийн"),
        ("ch", "Greece"),
        ("cs", "Řecko"),
        ("cv", "Грекийн"),
        ("cy", "Groeg"),
        ("da", "Grækenland"),
        ("de", "Griechenland"),
        ("dv", "ޔ\u{7ab}ނ\u{7a7}ނ\u{7b0}"),
        ("dz", "ག\u{f72}ར\u{f72}ས\u{f72}་།"),
        ("ee", "Greece"),
        ("el", "Ελλάδα"),
        ("en", "Greece"),
        ("eo", "Grekio"),
        ("es", "Grecia"),
        ("et", "Kreeka"),
        ("eu", "Grezia"),
        ("fa", "یونان"),
        ("ff", "Yunan"),
        ("fi", "Kreikka"),
        ("fo", "Grikkaland"),
        ("fr", "Grèce"),
        ("fy", "Grikelân"),
        ("ga", "An Ghréig"),
        ("gl", "Grecia"),
        ("gn", "Greece"),
        ("gu", "ગ\u{acd}રીસ"),
        ("gv", "Yn Ghreag"),
        ("ha", "Greek"),
        ("he", "יוון"),
        ("hi", "य\u{942}नान"),
        ("hr", "Grčka"),
        ("ht", "Grès"),
        ("hu", "Görögország"),
        ("hy", "Հունաստան"),
        ("ia", "Grecia"),
        ("id", "Yunani"),
        ("io", "Grekia"),
        ("is", "Grikkland"),
        ("it", "Grecia"),
        ("iu", "Greece"),
        ("ja", "ギリシャ"),
        ("ka", "საბერძნეთი"),
        ("ki", "Ngiriki"),
        ("kk", "Грекия"),
        ("kl", "Greece"),
        ("km", "ក\u{17d2}រ\u{17b7}ក"),
        ("kn", "ಗ\u{ccd}ರೀಸ\u{ccd}"),
        ("ko", "그리스"),
        ("ku", "Yewnanistan"),
        ("kv", "Эллада"),
        ("kw", "Pow Grek"),
        ("ky", "Греция"),
        ("lo", "ປະເທດກະແລ\u{eb1}ດ"),
        ("lt", "Graikija"),
        ("lv", "Grieķija"),
        ("mi", "Kirihi"),
        ("mk", "Грција"),
        ("ml", "ഗ\u{d4d}രീസ\u{d4d}"),
        ("mn", "Грек"),
        ("mr", "ग\u{94d}रीस"),
        ("ms", "Yunani"),
        ("mt", "Greċja"),
        ("my", "ဂရ\u{102d}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Grit"),
        ("nb", "Hellas"),
        ("ne", "ग\u{94d}रीस"),
        ("nl", "Griekenland"),
        ("nn", "Hellas"),
        ("nv", "Gwíík Dineʼé Bikéyah"),
        ("oc", "Grècia"),
        ("or", "ଗ\u{b4d}ରୀସ"),
        ("pa", "ਗਰੀਸ"),
        ("pi", "ग\u{94d}रीस (य\u{942}नान)"),
        ("pl", "Grecja"),
        ("ps", "یونان"),
        ("pt", "Grécia"),
        ("pt_BR", "Grécia"),
        ("ro", "Grecia"),
        ("ru", "Греция"),
        ("rw", "Ikigereki"),
        ("sc", "Grètzia"),
        ("sd", "يونان"),
        ("si", "ග\u{dca}\u{200d}ර\u{dd3}ස\u{dd2}ය"),
        ("sk", "Grécko"),
        ("sl", "Grčija"),
        ("so", "Giriigga"),
        ("sq", "Greqi"),
        ("sr", "Грчка"),
        ("sv", "Grekland"),
        ("sw", "Greece"),
        ("ta", "கிர\u{bc0}ஸ\u{bcd}"),
        ("te", "గ\u{c4d}ర\u{c40}స\u{c4d}"),
        ("tg", "Юнон"),
        ("th", "กร\u{e35}ซ"),
        ("ti", "ግሪኽ"),
        ("tk", "Grek"),
        ("tl", "Gresya"),
        ("tr", "Yunanistan"),
        ("tt", "Юнанстан"),
        ("ug", "گىرېتسىيە"),
        ("uk", "Греція"),
        ("ur", "یونان"),
        ("uz", "Yunoniston"),
        ("ve", "Greece"),
        ("vi", "Hy Lạp"),
        ("wa", "Grece"),
        ("wo", "Girees"),
        ("xh", "Greece"),
        ("yo", "Gríìsì"),
        ("zh_CN", "希腊"),
        ("zh_HK", "希臘"),
        ("zh_TW", "希臘"),
        ("zu", "IGreki"),
    ];
    #[cfg(all(feature = "gr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 39.074208;
        pub const LONGITUDE: f64 = 21.824312;
        pub const MAX_LATITUDE: f64 = 41.7488784;
        pub const MAX_LONGITUDE: f64 = 29.6527999;
        pub const MIN_LATITUDE: f64 = 34.5428;
        pub const MIN_LONGITUDE: f64 = 19.3098;
        pub const NORTHEAST_LATITUDE: f64 = 41.7488784;
        pub const NORTHEAST_LONGITUDE: f64 = 29.6527999;
        pub const SOUTHWEST_LATITUDE: f64 = 34.5428;
        pub const SOUTHWEST_LONGITUDE: f64 = 19.3098;
    }
}
#[cfg(all(feature = "gr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 39.074208,
            longitude: 21.824312,
            max_latitude: 41.7488784,
            max_longitude: 29.6527999,
            min_latitude: 34.5428,
            min_longitude: 19.3098,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 41.7488784,
                    longitude: 29.6527999,
                },
                southwest: CountryGeoBound {
                    latitude: 34.5428,
                    longitude: 19.3098,
                },
            },
        }
    }
}

#[cfg(all(feature = "gr", feature = "subdivisions"))]
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
                    "69",
                    Subdivision{
                        name: "69",
                        country_alpha2: Alpha2::GR,
                        code: "69",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.2644928), longitude: Some(24.2152731), max_latitude: Some(40.451581), min_latitude: Some(40.11453119999999), max_longitude: Some(24.3991418), min_longitude: Some(23.993087)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SelfGovernedPart,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Berg Athos"), ("ar", "جبل آثوس"), ("az", "Afon"), ("be", "Гара Афон"), ("bg", "Атон"), ("bs", "Sveta Gora"), ("ca", "Athos"), ("ccp", "𑄟𑄅\u{1112a}𑄚\u{11133}𑄑\u{11134} 𑄃𑄗\u{1112e}𑄌\u{11134}"), ("ceb", "Mount Athos"), ("cs", "Athos"), ("cy", "Mynydd Athos"), ("da", "Athos"), ("de", "Athos"), ("el", "Άγιο Όρος"), ("en", "Mount Athos"), ("es", "Monte Athos"), ("et", "Áthos"), ("eu", "Athos"), ("fa", "کوه آثوس"), ("fi", "Athos"), ("fr", "Aktè"), ("gl", "Monte Athos"), ("he", "הר אתוס"), ("hi", "एथोस पर\u{94d}वत"), ("hr", "Sveta Gora"), ("hu", "Athosz-hegy"), ("hy", "Աֆոն"), ("id", "Gunung Athos"), ("is", "Aþos"), ("it", "Monte Athos"), ("ja", "アトス山"), ("jv", "Mount Athos"), ("ka", "აიონ-ოროსი"), ("kk", "Афон тауы"), ("ko", "아토스 산"), ("lt", "Atonas"), ("lv", "Ajonora"), ("mk", "Света Гора"), ("ml", "മ\u{d57}ണ\u{d4d}ട\u{d4d} ആഥോസ\u{d4d}"), ("nb", "Athos"), ("nl", "Oros Athos"), ("no", "Athos"), ("pl", "Athos"), ("pt", "Monte Atos"), ("ro", "Muntele Athos"), ("ru", "Афон"), ("sk", "Athos"), ("sq", "Mali i Shenjtë"), ("sr", "Света гора"), ("sr_Latn", "Sveta gora"), ("sv", "Athos"), ("th", "เขาแอทอส"), ("tr", "Aynoroz"), ("uk", "Афон"), ("ur", "کوہ آتھوس"), ("vi", "Núi Athos"), ("zh", "阿索斯山")]),
                        unofficial_name_list: ["Aghion Oros", "Agion Oros", "Akte", "Aktí", "Athos", "Mount Athos"].to_vec(),
                    }
                ),
                (
                    "A",
                    Subdivision{
                        name: "A",
                        country_alpha2: Alpha2::GR,
                        code: "A",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقدونيا الشرقية وتراقيا"), ("az", "Şərqi Makedoniya və Trakya"), ("be", "Усходняя Македонія і Фракія"), ("bg", "Източна Македония и Тракия"), ("bn", "প\u{9c2}র\u{9cd}ব ম\u{9cd}য\u{9be}সেডোনিয\u{9bc}\u{9be} এবং থ\u{9cd}রেস"), ("ca", "Macedònia Oriental i Tràcia"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄟𑄥𑄬𑄓\u{1112e}𑄚\u{11128}𑄠 𑄃\u{11133}𑄃 𑄒\u{11133}𑄢𑄌\u{11134}"), ("ceb", "East Macedonia and Thrace"), ("cs", "Východní Makedonie a Thrákie"), ("da", "Østmakedonien og Thrakien"), ("de", "Ostmakedonien und Thrakien"), ("el", "Περιφέρεια Ανατολικής Μακεδονίας και Θράκης"), ("en", "East Macedonia and Thrace"), ("es", "Macedonia Oriental y Tracia"), ("et", "Ida-Makedoonia ja Traakia"), ("eu", "Ekialdeko Mazedonia eta Trazia"), ("fa", "مقدونیه شرقی و تراکیه"), ("fi", "Itä-Makedonia ja Traakia"), ("fr", "Macédoine-Orientale-et-Thrace"), ("gl", "Macedonia Oriental e Tracia"), ("gu", "ઇસ\u{acd}ટ મ\u{ac7}સ\u{ac7}ડોનિયા એન\u{acd}ડ થ\u{acd}ર\u{ac7}સ"), ("he", "מזרח מקדוניה ותראקיה"), ("hi", "प\u{942}र\u{94d}वी म\u{948}स\u{947}डोनिया और थ\u{94d}र\u{947}स"), ("hr", "Periferija Istočna Makedonija i Trakija"), ("hu", "Kelet-Makedónia és Thrákia"), ("hy", "Արևելյան Մակեդոնիա և Թրակիա"), ("id", "Makedonia Timur dan Trasia"), ("it", "Macedonia Orientale e Tracia"), ("ja", "東マケドニア・トラキア"), ("jv", "Makedonia Wétan lan Trasia"), ("ka", "აღმოსავლეთი მაკედონია და თრაკია"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಮ\u{cc6}ಸ\u{cbf}ಡೋನ\u{cbf}ಯಾ ಮತ\u{ccd}ತು ಥ\u{ccd}ರೇಸ\u{ccd}"), ("ko", "동마케도니아 트라키 주"), ("lt", "Rytų Makedonijos ir Trakijos periferija"), ("lv", "Austrumu Maķedonija un Trāķija"), ("mk", "Источна Македонија и Тракија"), ("mr", "ईस\u{94d}ट म\u{945}सिडोनिया अ\u{901}ड थ\u{94d}र\u{947}स"), ("ms", "East Macedonia and Thrace"), ("nb", "Øst-Makedonia og Thrakia"), ("nl", "Oost-Macedonië en Thracië"), ("no", "Øst-Makedonia og Thrakia"), ("pl", "Region Macedonia Wschodnia i Tracja"), ("pt", "Macedônia Oriental e Trácia"), ("ro", "Macedonia de Est și Tracia"), ("ru", "Восточная Македония и Фракия"), ("si", "නැගෙනහ\u{dd2}ර මැස\u{dd2}ඩෝන\u{dd2}ය\u{dcf} සහ ත\u{dca}රේස\u{dca}"), ("sk", "Východná Makedónia a Trácia"), ("sq", "Maqedonia Lindore dhe Thrakia"), ("sr", "Периферија Источна Македонија и Тракија"), ("sr_Latn", "Periferija Istočna Makedonija i Trakija"), ("sv", "Östra Makedonien och Thrakien"), ("ta", "கிழக\u{bcd}கு ம\u{bbe}சிடோனிய\u{bbe} அண\u{bcd}ட\u{bcd} த\u{bcd}ர\u{bbe}ஸ\u{bcd}"), ("te", "తూర\u{c4d}పు మ\u{c46}సడ\u{c4b}న\u{c3f}య\u{c3e} మర\u{c3f}యు త\u{c4d}ర\u{c47}స\u{c4d}"), ("th", "มาซ\u{e34}โดเน\u{e35}ยตะว\u{e31}นออกและเทรซ"), ("tr", "Doğu Makedonya ve Trakya"), ("uk", "Східна Македонія та Фракія"), ("ur", "مشرقی مقدونیہ اور تھریس"), ("vi", "Đông Macedonia và Thrace"), ("zh", "东马其顿-色雷斯")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "B",
                    Subdivision{
                        name: "B",
                        country_alpha2: Alpha2::GR,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقدونيا الوسطى"), ("az", "Orta Makedoniya"), ("be", "Цэнтральная Македонія"), ("bg", "Централна Македония"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল মেসিডোনিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Macedònia Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄟𑄥𑄬𑄓\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Central Macedonia"), ("cs", "Střední Makedonie"), ("da", "Central Macedonia Region"), ("de", "Zentralmakedonien"), ("el", "Κεντρική Μακεδονία"), ("en", "Central Macedonia"), ("es", "Macedonia Central"), ("et", "Kesk-Makedoonia"), ("eu", "Erdialdeko Mazedonia"), ("fa", "مقدونیه مرکزی"), ("fi", "Keski-Makedonia"), ("fr", "Macédoine-Centrale"), ("gl", "Macedonia Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ મ\u{ac7}સ\u{ac7}ડોનિયા પ\u{acd}રદ\u{ac7}શ"), ("he", "מרכז מקדוניה"), ("hi", "क\u{947}\u{902}द\u{94d}रीय म\u{948}स\u{947}डोनिया क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Središnja Makedonija"), ("hu", "Közép-Makedónia"), ("hy", "Կենտրոնական Մակեդոնիա"), ("id", "Makedonia Tengah"), ("it", "Macedonia Centrale"), ("ja", "中央マケドニア"), ("jv", "Makedonia Tengah"), ("ka", "ცენტრალური მაკედონია"), ("kn", "ಮಧ\u{ccd}ಯ ಮಾಸ\u{cc6}ಡೋನ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "중앙마케도니아 주"), ("lt", "Centrinės Makedonijos periferija"), ("lv", "Centrālās Maķedonijas perifērija"), ("mk", "Централна Македонија"), ("mr", "मध\u{94d}यवर\u{94d}ती म\u{945}सिडोनिया प\u{94d}रद\u{947}श"), ("ms", "Macedonia Tengah"), ("nb", "Sentral-Makedonia"), ("nl", "Centraal-Macedonië"), ("no", "Sentral-Makedonia"), ("pl", "Region Macedonia Środkowa"), ("pt", "Macedônia Central"), ("ro", "Macedonia Centrală"), ("ru", "Центральная Македония"), ("si", "මධ\u{dca}\u{200d}යම මැස\u{dd2}ඩෝන\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Stredná Makedónia"), ("sq", "Maqedonia Qendrore"), ("sr", "Периферија Средишња Македонија"), ("sr_Latn", "Periferija Središnja Makedonija"), ("sv", "Mellersta Makedonien"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ம\u{bbe}சிடோனிய\u{bbe} பகுதி"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} మ\u{c3e}స\u{c46}డ\u{c4b}న\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มาเกโดเน\u{e35}ยกลาง"), ("tr", "Orta Makedonya"), ("uk", "Центральна Македонія"), ("ur", "وسطی مقدونیہ"), ("vi", "Trung Macedonia"), ("zh", "中马其顿")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::GR,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقدونيا الغربية"), ("az", "Qərbi Makedoniya"), ("be", "Заходняя Македонія"), ("bg", "Западна Македония"), ("bn", "ওয\u{9bc}েস\u{9cd}ট মেসিডোনিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Macedònia Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄟𑄥𑄬𑄓\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "West Macedonia"), ("cs", "Západní Makedonie"), ("da", "West Macedonia Region"), ("de", "Westmakedonien"), ("el", "Περιφέρεια Δυτικής Μακεδονίας"), ("en", "West Macedonia"), ("es", "Macedonia Occidental"), ("et", "Lääne-Makedoonia"), ("eu", "Mendebaldeko Mazedonia"), ("fa", "مقدونیه غربی"), ("fi", "Länsi-Makedonia"), ("fr", "Macédoine-Occidentale"), ("gl", "Macedonia Occidental"), ("gu", "પશ\u{acd}ચિમ મ\u{ac7}સ\u{ac7}ડોનિયા પ\u{acd}રદ\u{ac7}શ"), ("he", "מערב מקדוניה"), ("hi", "पश\u{94d}चिमी म\u{948}स\u{947}डोनिया क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Zapadna Makedonija"), ("hu", "Nyugat-Makedónia"), ("id", "Makedonia Barat"), ("it", "Macedonia Occidentale"), ("ja", "西マケドニア"), ("jv", "Makedonia Kulon"), ("ka", "დასავლეთი მაკედონია"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಮಾಸ\u{cc6}ಡೋನ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "서마케도니아 주"), ("lt", "Vakarų Makedonijos periferija"), ("lv", "Rietummaķedonijas perifērija"), ("mk", "Западна Македонија"), ("mr", "व\u{947}स\u{94d}ट म\u{945}स\u{947}डोनिया प\u{94d}रद\u{947}श"), ("ms", "West Macedonia Region"), ("nb", "Vest-Makedonia"), ("nl", "West-Macedonië"), ("no", "Vest-Makedonia"), ("pl", "Region Macedonia Zachodnia"), ("pt", "Macedônia Ocidental"), ("ro", "Macedonia de Vest"), ("ru", "Западная Македония"), ("si", "බටහ\u{dd2}ර මැස\u{dd2}ඩෝන\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Západná Makedónia"), ("sq", "Maqedonia Perëndimore"), ("sr", "Периферија Западна Македонија"), ("sr_Latn", "Periferija Zapadna Makedonija"), ("sv", "Västra Makedonien"), ("ta", "மேற\u{bcd}கு ம\u{bbe}சிடோனிய\u{bbe} பகுதி"), ("te", "పశ\u{c4d}చ\u{c3f}మ మ\u{c46}స\u{c3f}డ\u{c4b}న\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เวส มาซ\u{e34}โดเน\u{e35}ย"), ("tr", "Batı Makedonya"), ("uk", "Західна Македонія"), ("ur", "مغربی مقدونیہ"), ("vi", "Tây Macedonia"), ("zh", "西马其顿")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::GR,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إبيروس"), ("be", "Эпір"), ("bg", "Епир"), ("bn", "এপির\u{9be}স অঞ\u{9cd}চল"), ("ca", "Regne de l’Epir"), ("ccp", "𑄃\u{11128}𑄛\u{1112d}𑄢𑄌\u{11134}"), ("cs", "Epirus"), ("cy", "Epiros"), ("da", "Epirus Region"), ("de", "Epirus"), ("el", "Περιφέρεια Ηπείρου"), ("en", "Epirus"), ("es", "Epiro"), ("et", "Ípeiros"), ("eu", "Epiro"), ("fa", "ایپیروس"), ("fi", "Epeiros"), ("fr", "Épire (périphérie)"), ("gl", "Épiro"), ("gu", "એપિરસસ પ\u{acd}રદ\u{ac7}શ"), ("he", "אפירוס"), ("hi", "एपिरस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Epir"), ("hu", "Epirusz"), ("hy", "Էպիրուս"), ("id", "Epirus"), ("it", "Epiro"), ("ja", "イピロス"), ("jv", "Epirus (periphery)"), ("ka", "ეპირი"), ("kn", "ಎಪ\u{cbf}ರಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "이피로스 주"), ("lt", "Epyro periferija"), ("lv", "Epīras perifērija"), ("mk", "Епир"), ("mr", "एपिअरस प\u{94d}रद\u{947}श"), ("ms", "Epirus"), ("nb", "Epirus"), ("nl", "Epirus"), ("no", "Epirus"), ("pl", "Epir"), ("pt", "Epiro"), ("ro", "Epir"), ("ru", "Эпир"), ("si", "එප\u{dd2}රස\u{dca} කල\u{dcf}පය"), ("sk", "Epirus"), ("sl", "Čamerija"), ("sq", "Epiri (periferia)"), ("sr", "Периферија Епир"), ("sr_Latn", "Periferija Epir"), ("sv", "Epirus"), ("ta", "எபிரோஸ\u{bcd} பகுதி"), ("te", "ఎప\u{c3f}రస\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e34}ไพร\u{e31}ส"), ("tr", "Epir"), ("uk", "Епір"), ("ur", "اپیروس (علاقہ)"), ("vi", "Epirus"), ("zh", "伊庇鲁斯")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "E",
                    Subdivision{
                        name: "E",
                        country_alpha2: Alpha2::GR,
                        code: "E",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ثيساليا"), ("az", "Fessaliya"), ("be", "Фесалія"), ("bg", "Тесалия"), ("bn", "থেস\u{9be}লি"), ("ca", "Tessàlia"), ("ccp", "𑄗𑄬𑄥𑄣\u{11128}"), ("cs", "Thesálie"), ("cy", "Thessalia"), ("da", "Thessaly"), ("de", "Thessalien"), ("el", "Θεσσαλία"), ("en", "Thessaly"), ("es", "Tesalia"), ("et", "Tessaalia piirkond"), ("eu", "Tesalia"), ("fa", "تسالی"), ("fi", "Thessalia"), ("fr", "Thessalie"), ("ga", "An Teasáil"), ("gl", "Tesalia"), ("gu", "થ\u{ac7}સાલી"), ("he", "תסליה"), ("hi", "थ\u{947}सली"), ("hr", "Tesalija"), ("hu", "Thesszália"), ("hy", "Թեսալիա"), ("id", "Thessalia"), ("is", "Þessalía"), ("it", "Tessaglia"), ("ja", "テッサリア"), ("jv", "Tesalonika"), ("ka", "თესალია"), ("kn", "ಥ\u{cc6}ಸ\u{ccd}ಸಲ\u{cbf}"), ("ko", "테살리아"), ("lt", "Tesalija"), ("lv", "Tesālija"), ("mk", "Тесалија"), ("mr", "थिसलीस"), ("ms", "Thessaly"), ("nb", "Thessalia"), ("nl", "Thessalië"), ("no", "Thessalia"), ("pl", "Tesalia"), ("pt", "Tessália"), ("ro", "Tesalia"), ("ru", "Фессалия"), ("si", "තෙසලේ"), ("sk", "Tesália"), ("sl", "Tesalija"), ("sq", "Thesalia"), ("sr", "Тесалија"), ("sr_Latn", "Tesalija"), ("sv", "Thessalien"), ("ta", "தெஸ\u{bcd}ஸ\u{bbe}லி"), ("te", "త\u{c46}స\u{c3e}ల\u{c40}"), ("th", "เทสซาล\u{e35}"), ("tr", "Tesalya"), ("uk", "Фессалія"), ("ur", "ثیسالیا"), ("vi", "Thessaly"), ("zh", "色萨利")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "F",
                    Subdivision{
                        name: "F",
                        country_alpha2: Alpha2::GR,
                        code: "F",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الجزر الأيونية"), ("be", "Перыферыя Іанічныя астравы"), ("ca", "Regió de les Illes Jòniques"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄚\u{11128}𑄠\u{11127}𑄚\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄌\u{11134}"), ("ceb", "Ionian Islands"), ("cs", "Jónské ostrovy (kraj)"), ("de", "Ionische Inseln"), ("el", "Περιφέρεια Ιονίων Νήσων"), ("en", "Ionian Islands"), ("es", "Periferia de Islas Jónicas"), ("et", "Joonia saarte piirkond"), ("fa", "استان جزایر ایونی"), ("fi", "Jooniansaaret"), ("fr", "Îles Ioniennes (périphérie)"), ("he", "האיים היוניים"), ("hr", "Periferija Jonski otoci"), ("hy", "Հոնիական կղզիներ"), ("it", "Isole Ionie"), ("ko", "이오니아 제도주"), ("lt", "Jonijos salų periferija"), ("nb", "De joniske øyer"), ("nl", "Ionian Islands Region"), ("no", "De joniske øyer"), ("pl", "Region Wyspy Jońskie"), ("pt", "Ilhas Jónicas"), ("ru", "периферия Ионические острова"), ("sq", "Ishujt e Jonit"), ("sr", "Периферија Јонска острва"), ("sr_Latn", "Periferija Jonska ostrva"), ("tr", "İyonya Adaları"), ("uk", "Іонічні острови"), ("ur", "ایونی جزائر (علاقہ)"), ("zh", "愛奧尼亞群島")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "G",
                        country_alpha2: Alpha2::GR,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غرب اليونان"), ("be", "Заходняя Грэцыя"), ("bg", "Западна Гърция"), ("bn", "ওয\u{9bc}েস\u{9cd}ট গ\u{9cd}রীস অঞ\u{9cd}চল"), ("ca", "Grècia Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄉\u{11133}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "West Greece"), ("cs", "Západní Řecko"), ("da", "West Greece Region"), ("de", "Westgriechenland"), ("el", "Περιφέρεια Δυτικής Ελλάδας"), ("en", "West Greece"), ("es", "Grecia Occidental"), ("et", "Lääne-Kreeka piirkond"), ("fa", "یونان غربی"), ("fi", "Länsi-Kreikka"), ("fr", "Grèce-Occidentale"), ("gl", "Grecia Occidental"), ("gu", "પશ\u{acd}ચિમ ગ\u{acd}રીસ પ\u{acd}રદ\u{ac7}શ"), ("he", "מערב יוון"), ("hi", "पश\u{94d}चिमी ग\u{94d}रीस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Zapadna Grčka"), ("hu", "Nyugat-Görögország"), ("hy", "Արևմտյան Հունաստան"), ("id", "Yunani Barat"), ("it", "Grecia Occidentale"), ("ja", "西ギリシャ"), ("jv", "Yunani Kulon"), ("ka", "დასავლეთი საბერძნეთი"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಗ\u{ccd}ರೀಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "서그리스 주"), ("lt", "Vakarų Graikijos periferija"), ("lv", "Rietumgrieķijas perifērija"), ("mk", "Западна Грција"), ("mr", "व\u{947}स\u{94d}ट ग\u{94d}रीस प\u{94d}रद\u{947}श"), ("ms", "West Greece Region"), ("nb", "Vest-Hellas"), ("nl", "West-Griekenland"), ("no", "Vest-Hellas"), ("pl", "Region Grecja Zachodnia"), ("pt", "Grécia Ocidental"), ("ro", "Grecia de Vest"), ("ru", "Западная Греция"), ("si", "වෙස\u{dca}ට\u{dca} ග\u{dca}\u{200d}ර\u{dd3}ස\u{dca} කල\u{dcf}පය"), ("sk", "Západné Grécko"), ("sr", "Периферија Западна Грчка"), ("sr_Latn", "Periferija Zapadna Grčka"), ("sv", "Västra Grekland"), ("ta", "மேற\u{bcd}கு கிர\u{bc0}ஸ\u{bcd} பகுதி"), ("te", "పశ\u{c4d}చ\u{c3f}మ గ\u{c4d}ర\u{c40}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เวสเท\u{e34}ร\u{e4c}กร\u{e35}ซ"), ("tr", "Batı Yunanistan"), ("uk", "Західна Греція"), ("ur", "مغربی یونان"), ("vi", "Tây Hy Lạp"), ("zh", "西希腊")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "H",
                        country_alpha2: Alpha2::GR,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وسط اليونان"), ("be", "Цэнтральная Грэцыя"), ("bg", "Централна Гърция"), ("ca", "Grècia Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄉\u{11133}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Central Greece"), ("cs", "Střední Řecko"), ("de", "Mittelgriechenland"), ("el", "Περιφέρεια Στερεάς Ελλάδας"), ("en", "Central Greece"), ("es", "Grecia Central"), ("et", "Kesk-Kreeka piirkond"), ("eu", "Erdialdeko Grezia"), ("fa", "یونان مرکزی"), ("fi", "Keski-Kreikka"), ("fr", "Grèce-Centrale"), ("gl", "Grecia Central"), ("he", "מרכז יוון"), ("hi", "मध\u{94d}य ग\u{94d}रीस क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Središnja Grčka"), ("hu", "Közép-Görögország"), ("hy", "Կենտրոնական Հունաստան"), ("id", "Yunani Tengah"), ("it", "Grecia Centrale"), ("ja", "中央ギリシャ"), ("jv", "Yunani Tengah"), ("ko", "중앙그리스 주"), ("lt", "Centrinės Graikijos periferija"), ("lv", "Centrālās Grieķijas perifērija"), ("mk", "Централна Грција"), ("nb", "Sentral-Hellas"), ("nl", "Centraal-Griekenland"), ("no", "Sentral-Hellas"), ("pl", "Region Grecja Środkowa"), ("pt", "Grécia Central"), ("ro", "Grecia Centrală"), ("ru", "Центральная Греция"), ("sq", "Greqia Qendrore"), ("sr", "Периферија Средишња Грчка"), ("sr_Latn", "Periferija Središnja Grčka"), ("sv", "Grekiska fastlandet"), ("tr", "Orta Yunanistan"), ("uk", "Центральна Греція"), ("ur", "وسطی یونان (علاقہ)"), ("vi", "Trung Hy Lạp"), ("zh", "中希臘")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "I",
                    Subdivision{
                        name: "I",
                        country_alpha2: Alpha2::GR,
                        code: "I",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أتيكا"), ("be", "Атыка"), ("bn", "আটিক\u{9be} অঞ\u{9cd}চল"), ("ca", "Àtica"), ("ccp", "𑄃\u{11128}𑄑\u{11133}𑄦\u{11128}𑄇"), ("ceb", "Attica"), ("cs", "Attika (kraj)"), ("da", "Attica Region"), ("de", "Attika"), ("el", "Αττική"), ("en", "Attica"), ("es", "Región Attica"), ("et", "Atika piirkond"), ("eu", "Atika"), ("fa", "آتیک (ناحیه)"), ("fi", "Attican maakunta"), ("fr", "Attique (périphérie)"), ("gl", "Periferia de Ática"), ("gu", "અટીકા પ\u{acd}રદ\u{ac7}શ"), ("hi", "अटिका क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Atika"), ("hy", "Ատիկա"), ("id", "Wilayah Attica"), ("it", "Attica"), ("ja", "アッティキ"), ("ka", "ატიკა"), ("kn", "ಅಟ\u{ccd}ಟ\u{cbf}ಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "아티카 주"), ("lt", "Atikos periferija"), ("lv", "Atikas perifērija"), ("mr", "अटिका प\u{94d}रद\u{947}श"), ("ms", "Attica Region"), ("nb", "Attica region"), ("nl", "Attica"), ("no", "Attica region"), ("pl", "Region Attyka"), ("pt", "Região Attica"), ("ro", "Attica"), ("ru", "Аттика"), ("si", "අට\u{dca}ට\u{dd2}ක\u{dcf} කල\u{dcf}පය"), ("sk", "Atika"), ("sl", "Atika"), ("sq", "Atika"), ("sr", "Периферија Атика"), ("sr_Latn", "Periferija Atika"), ("sv", "Attica (region)"), ("ta", "அட\u{bcd}டிக\u{bbe} பகுதி"), ("te", "అట\u{c4d}ట\u{c3f}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ภ\u{e39}ม\u{e34}ภาคอ\u{e31}ตต\u{e34}กะ"), ("tr", "Attika"), ("uk", "периферія Аттика"), ("ur", "اتیکا (علاقہ)"), ("vi", "Khu vực Attica"), ("zh", "阿提卡 (地區)")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "J",
                    Subdivision{
                        name: "J",
                        country_alpha2: Alpha2::GR,
                        code: "J",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيلوبونيز"), ("be", "Акруга Пелапанес"), ("bg", "Пелопонес"), ("bn", "পেলোপোনেসে অঞ\u{9cd}চল"), ("ca", "Regió del Peloponès"), ("ccp", "𑄛𑄬𑄣\u{1112e}𑄛\u{11127}𑄚\u{11128}𑄌\u{11134}"), ("ceb", "Peloponnese"), ("cs", "Peloponés"), ("da", "Peloponnese Region"), ("de", "Peloponnes"), ("el", "Περιφέρεια Πελοποννήσου"), ("en", "Peloponnese"), ("es", "Periferia de Peloponeso"), ("et", "Peloponnesose piirkond"), ("fa", "استان پلپونز"), ("fi", "Peloponnesos"), ("fr", "Péloponnèse (périphérie)"), ("gl", "Periferia do Peloponeso"), ("gu", "પ\u{ac7}લોપોનિસિ પ\u{acd}રદ\u{ac7}શ"), ("he", "פלופונסוס"), ("hi", "प\u{947}लोपोनीज\u{93c} प\u{94d}रद\u{947}श"), ("hr", "Periferija Peloponez"), ("hu", "Peloponnészosz"), ("hy", "Պելոպոնես"), ("id", "Peloponnesa"), ("it", "Peloponneso"), ("ja", "ペロポネソス"), ("jv", "Peloponnesa"), ("ka", "პელოპონესის პერიფერია"), ("kn", "ಪ\u{cc6}ಲೋಪೋನೀಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "펠로폰네소스 주"), ("lt", "Peloponeso periferija"), ("lv", "Peloponesas perifērija"), ("mk", "Пелопонез"), ("mr", "प\u{947}लोपोनिस प\u{94d}रद\u{947}श"), ("ms", "Peloponnesa"), ("nb", "Peloponnes"), ("nl", "Peloponnesos"), ("no", "Peloponnes"), ("pl", "Region Peloponez"), ("pt", "Região de Peloponnes"), ("ro", "Peloponez"), ("ru", "Пелопоннес"), ("si", "පෙලොපොන\u{dd2}ස\u{dca} කල\u{dcf}පය"), ("sr", "Периферија Пелопонез"), ("sr_Latn", "Periferija Peloponez"), ("sv", "Peloponnesos"), ("ta", "பெலோபொன\u{bcd}னெஸ\u{bcd} பகுதி"), ("te", "ప\u{c46}ల\u{c46}ప\u{c4b}న\u{c46}స\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตเพโลพอนเนส"), ("tr", "Mora"), ("uk", "Пелопоннес"), ("ur", "پیلوپونیز (علاقہ)"), ("vi", "Khu vực Peloponnese"), ("zh", "伯罗奔尼撒")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "K",
                    Subdivision{
                        name: "K",
                        country_alpha2: Alpha2::GR,
                        code: "K",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شمال إيجة"), ("be", "Паўночныя Эгейскія астравы"), ("bg", "Северен Егей"), ("bn", "নর\u{9cd}থ এজিয\u{9bc}\u{9be}ন অঞ\u{9cd}চল"), ("ca", "Egeu Septentrional"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄃𑄃\u{11128}𑄎\u{11129}𑄚\u{11134}"), ("ceb", "North Aegean"), ("cs", "Severní Egeis"), ("da", "North Aegean Region"), ("de", "Nördliche Ägäis"), ("el", "Περιφέρεια Βορείου Αιγαίου"), ("en", "North Aegean"), ("es", "Egeo Septentrional"), ("et", "Põhja-Egeus"), ("fa", "استان اژه شمالی"), ("fi", "Pohjois-Egean saaret"), ("fr", "Égée-Septentrionale"), ("gl", "Exeo Setentrional"), ("gu", "ઉત\u{acd}તર એજીયન પ\u{acd}રદ\u{ac7}શ"), ("he", "צפון הים האגאי"), ("hi", "उत\u{94d}तरी एजियन क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Periferija Sjeverni Egej"), ("hu", "Észak-Égei-szigetek"), ("hy", "Հյուսիսային Եգեյան կղզիներ"), ("id", "Aegea Utara"), ("it", "Egeo Settentrionale"), ("ja", "北エーゲ"), ("jv", "Aegea Lor"), ("ka", "ჩრდილოეთ ეგეოსის კუნძულები"), ("kn", "ಉತ\u{ccd}ತರ ಏಜ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "북에게 주"), ("lt", "Šiaurės Egėjo periferija"), ("lv", "Ziemeļegejas perifērija"), ("mk", "Северен Егеј"), ("mr", "नॉर\u{94d}थ एजियन प\u{94d}रद\u{947}श"), ("ms", "Aegea Utara"), ("nb", "Nordlige egeiske øyer"), ("nl", "Noord-Egeïsche Eilanden"), ("no", "Nordlige egeiske øyer"), ("pl", "Region Wyspy Egejskie Północne"), ("pt", "Egeu Setentrional"), ("ro", "Egeea de Nord"), ("ru", "Северные Эгейские острова"), ("si", "උත\u{dd4}ර\u{dd4} එය\u{dd2}ජ\u{dd2}යන\u{dca} කල\u{dcf}පය"), ("sr", "Периферија Северни Егеј"), ("sr_Latn", "Periferija Severni Egej"), ("sv", "Nordegeiska öarna"), ("ta", "வடக\u{bcd}கு ஏகன\u{bcd} பகுதி"), ("te", "ఉత\u{c4d}తర ఏజ\u{c3f}యన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e35}เจ\u{e35}ยนเหน\u{e37}อ"), ("tr", "Kuzey Ege"), ("uk", "Північні Егейські острови"), ("ur", "شمالی ایجیئن"), ("vi", "Bắc Aegea"), ("zh", "北愛琴")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::GR,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب إيجة"), ("be", "Паўднёвыя Эгейскія астравы"), ("bg", "Южен Егей"), ("bn", "দক\u{9cd}ষিণ এজিয\u{9bc}েন"), ("ca", "Egeu Meridional"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄃𑄃\u{11128}𑄎\u{11129}𑄚\u{11134}"), ("ceb", "South Aegean"), ("cs", "Jižní Egeis"), ("da", "South Aegean"), ("de", "Südliche Ägäis"), ("el", "Περιφέρεια Νοτίου Αιγαίου"), ("en", "South Aegean"), ("es", "Egeo Meridional"), ("et", "Lõuna-Egeus"), ("fa", "استان اژه جنوبی"), ("fi", "Etelä-Egean saaret"), ("fr", "Égée-Méridionale"), ("gl", "Exeo Meridional"), ("gu", "સાઉથ એજીયન"), ("he", "דרום הים האגאי"), ("hi", "दक\u{94d}षिण एजियन"), ("hr", "Periferija Južni Egej"), ("hu", "Dél-Égei-szigetek"), ("hy", "Հարավային Եգեյան կղզիներ"), ("id", "Aegea Selatan"), ("it", "Egeo Meridionale"), ("ja", "南エーゲ"), ("jv", "Aegea Kidul"), ("ka", "სამხრეთ ეგეოსის კუნძულები"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಏಜ\u{cbf}ಯನ\u{ccd}"), ("ko", "남에게 주"), ("lt", "Pietų Egėjo periferija"), ("lv", "Dienvidegeja"), ("mk", "Јужен Егеј"), ("mr", "दक\u{94d}षिण एजियन"), ("ms", "Aegea Selatan"), ("nb", "Sørlige egeiske øyer"), ("nl", "Zuid-Egeïsche Eilanden"), ("no", "Sørlige egeiske øyer"), ("pl", "Region Wyspy Egejskie Południowe"), ("pt", "Egeu Meridional"), ("ro", "Egeea de Sud"), ("ru", "Южные Эгейские острова"), ("si", "දක\u{dd4}ණ\u{dd4} අය\u{dd2}ජ\u{dd2}යන\u{dca}"), ("sr", "Периферија Јужни Егеј"), ("sr_Latn", "Periferija Južni Egej"), ("sv", "Sydegeiska öarna"), ("ta", "தெற\u{bcd}கு ஏஜென\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ఆగ\u{c3f}య\u{c3e}న\u{c4d}"), ("th", "เซาร\u{e4c}ทอ\u{e35}เจ\u{e35}ยน"), ("tr", "Güney Ege"), ("uk", "Південні Егейські острови"), ("ur", "جنوبی ایجیئن"), ("vi", "Nam Aegea"), ("zh", "南愛琴")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::GR,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Акруга Крыт"), ("ca", "Regió de Creta"), ("ccp", "𑄇\u{11133}𑄢𑄬𑄖\u{11134}"), ("cs", "Kréta"), ("de", "Kreta"), ("el", "Περιφέρεια Κρήτης"), ("en", "Crete"), ("es", "Periferia de Creta"), ("et", "Kreeta piirkond"), ("fi", "Kreeta"), ("fr", "Crète (périphérie)"), ("hr", "Periferija Kreta"), ("ko", "크레타주"), ("nl", "Kreta"), ("pl", "Region Kreta"), ("ru", "Крит"), ("sr", "Периферија Крит"), ("sr_Latn", "Periferija Krit")]),
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
#[cfg(feature = "gr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GR,
        alpha3: Alpha3::GRC,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 30,
        currency_code: "EUR",
        gec: Some(GEC::GR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("GRE"),
        iso_long_name: "The Hellenic Republic",
        iso_short_name: "Greece",
        official_language_list: ["el"].to_vec(),
        spoken_language_list: ["el"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "None",
        nationality: Some("Greek"),
        number: "300",
        postal_code: true,
        postal_code_format: Some("\\d{3} ?\\d{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "GR",
        unofficial_name_list: [
            "Greece",
            "Griechenland",
            "Grèce",
            "Grecia",
            "ギリシャ",
            "Griekenland",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Greece"),
            ("af", "Griekeland"),
            ("ak", "Greece"),
            ("am", "ጔሱጤ"),
            ("an", "Greece"),
            ("ar", "اليونان"),
            ("as", "গ\u{9cd}ইচ\u{9cd}\u{200c}ল\u{9be}মিকীচ"),
            ("ay", "Greece"),
            ("az", "Yunanıstan"),
            ("ba", "Greece"),
            ("be", "Грэцыя"),
            ("bg", "Гърция"),
            ("bi", "Greece"),
            ("bn", "গ\u{9cd}রিস"),
            ("bn_IN", "গ\u{9cd}রিস"),
            ("br", "Gres"),
            ("bs", "Grčka"),
            ("ca", "Grècia"),
            ("ce", "Грекийн"),
            ("ch", "Greece"),
            ("cs", "Řecko"),
            ("cv", "Грекийн"),
            ("cy", "Groeg"),
            ("da", "Grækenland"),
            ("de", "Griechenland"),
            ("dv", "ޔ\u{7ab}ނ\u{7a7}ނ\u{7b0}"),
            ("dz", "ག\u{f72}ར\u{f72}ས\u{f72}་།"),
            ("ee", "Greece"),
            ("el", "Ελλάδα"),
            ("en", "Greece"),
            ("eo", "Grekio"),
            ("es", "Grecia"),
            ("et", "Kreeka"),
            ("eu", "Grezia"),
            ("fa", "یونان"),
            ("ff", "Yunan"),
            ("fi", "Kreikka"),
            ("fo", "Grikkaland"),
            ("fr", "Grèce"),
            ("fy", "Grikelân"),
            ("ga", "An Ghréig"),
            ("gl", "Grecia"),
            ("gn", "Greece"),
            ("gu", "ગ\u{acd}રીસ"),
            ("gv", "Yn Ghreag"),
            ("ha", "Greek"),
            ("he", "יוון"),
            ("hi", "य\u{942}नान"),
            ("hr", "Grčka"),
            ("ht", "Grès"),
            ("hu", "Görögország"),
            ("hy", "Հունաստան"),
            ("ia", "Grecia"),
            ("id", "Yunani"),
            ("io", "Grekia"),
            ("is", "Grikkland"),
            ("it", "Grecia"),
            ("iu", "Greece"),
            ("ja", "ギリシャ"),
            ("ka", "საბერძნეთი"),
            ("ki", "Ngiriki"),
            ("kk", "Грекия"),
            ("kl", "Greece"),
            ("km", "ក\u{17d2}រ\u{17b7}ក"),
            ("kn", "ಗ\u{ccd}ರೀಸ\u{ccd}"),
            ("ko", "그리스"),
            ("ku", "Yewnanistan"),
            ("kv", "Эллада"),
            ("kw", "Pow Grek"),
            ("ky", "Греция"),
            ("lo", "ປະເທດກະແລ\u{eb1}ດ"),
            ("lt", "Graikija"),
            ("lv", "Grieķija"),
            ("mi", "Kirihi"),
            ("mk", "Грција"),
            ("ml", "ഗ\u{d4d}രീസ\u{d4d}"),
            ("mn", "Грек"),
            ("mr", "ग\u{94d}रीस"),
            ("ms", "Yunani"),
            ("mt", "Greċja"),
            ("my", "ဂရ\u{102d}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Grit"),
            ("nb", "Hellas"),
            ("ne", "ग\u{94d}रीस"),
            ("nl", "Griekenland"),
            ("nn", "Hellas"),
            ("nv", "Gwíík Dineʼé Bikéyah"),
            ("oc", "Grècia"),
            ("or", "ଗ\u{b4d}ରୀସ"),
            ("pa", "ਗਰੀਸ"),
            ("pi", "ग\u{94d}रीस (य\u{942}नान)"),
            ("pl", "Grecja"),
            ("ps", "یونان"),
            ("pt", "Grécia"),
            ("pt_BR", "Grécia"),
            ("ro", "Grecia"),
            ("ru", "Греция"),
            ("rw", "Ikigereki"),
            ("sc", "Grètzia"),
            ("sd", "يونان"),
            ("si", "ග\u{dca}\u{200d}ර\u{dd3}ස\u{dd2}ය"),
            ("sk", "Grécko"),
            ("sl", "Grčija"),
            ("so", "Giriigga"),
            ("sq", "Greqi"),
            ("sr", "Грчка"),
            ("sv", "Grekland"),
            ("sw", "Greece"),
            ("ta", "கிர\u{bc0}ஸ\u{bcd}"),
            ("te", "గ\u{c4d}ర\u{c40}స\u{c4d}"),
            ("tg", "Юнон"),
            ("th", "กร\u{e35}ซ"),
            ("ti", "ግሪኽ"),
            ("tk", "Grek"),
            ("tl", "Gresya"),
            ("tr", "Yunanistan"),
            ("tt", "Юнанстан"),
            ("ug", "گىرېتسىيە"),
            ("uk", "Греція"),
            ("ur", "یونان"),
            ("uz", "Yunoniston"),
            ("ve", "Greece"),
            ("vi", "Hy Lạp"),
            ("wa", "Grece"),
            ("wo", "Girees"),
            ("xh", "Greece"),
            ("yo", "Gríìsì"),
            ("zh_CN", "希腊"),
            ("zh_HK", "希臘"),
            ("zh_TW", "希臘"),
            ("zu", "IGreki"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}