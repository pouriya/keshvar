// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Japan

#[cfg(all(feature = "jp", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("〒{{postalcode}}\n{{region_short}}{{city}}{{street}}\n{{recipient}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::JP;
    pub const ALPHA3: Alpha3 = Alpha3::JPN;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 81;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::JPY;
    pub const GEC: Option<GEC> = Some(GEC::JA);
    pub const INTERNATIONAL_PREFIX: &str = "010";
    pub const IOC: Option<IOC> = Some(IOC::JPN);
    pub const ISO_SHORT_NAME: &str = "Japan";
    pub const ISO_LONG_NAME: &str = "Japan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ja"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ja"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Japanese");
    pub const NUMBER: &str = "392";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}-?\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "JP";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Japan", "Japon", "Japón", "日本"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Japan"),
        ("af", "Japan"),
        ("ak", "Japan"),
        ("am", "ጃፓን"),
        ("an", "Japan"),
        ("ar", "اليابان"),
        ("as", "জ\u{9be}প\u{9be}ন"),
        ("ay", "Japan"),
        ("az", "Yaponiya"),
        ("ba", "Japan"),
        ("be", "Японія"),
        ("bg", "Япония"),
        ("bi", "Japan"),
        ("bn", "জ\u{9be}প\u{9be}ন"),
        ("bn_IN", "জ\u{9be}প\u{9be}ন"),
        ("br", "Japan"),
        ("bs", "Japan"),
        ("ca", "Japó"),
        ("ce", "Япони"),
        ("ch", "Chapan"),
        ("cs", "Japonsko"),
        ("cv", "Япони"),
        ("cy", "Japan"),
        ("da", "Japan"),
        ("de", "Japan"),
        ("dv", "ޖ\u{7a6}ޕ\u{7a7}ނ\u{7aa}"),
        ("dz", "ཇ་པ\u{f71}ན།"),
        ("ee", "Japan"),
        ("el", "Ιαπωνία"),
        ("en", "Japan"),
        ("eo", "Japanio"),
        ("es", "Japón"),
        ("et", "Jaapan"),
        ("eu", "Japonia"),
        ("fa", "ژاپن"),
        ("ff", "Japan"),
        ("fi", "Japani"),
        ("fo", "Japan"),
        ("fr", "Japon"),
        ("fy", "Japan"),
        ("ga", "An tSeapáin"),
        ("gl", "Xapón"),
        ("gn", "Japan"),
        ("gu", "જાપાન"),
        ("gv", "Yn Çhapaan"),
        ("ha", "Japan"),
        ("he", "יפן"),
        ("hi", "जापान"),
        ("hr", "Japan"),
        ("ht", "Japon"),
        ("hu", "Japán"),
        ("hy", "Ճապոնիա"),
        ("ia", "Japon"),
        ("id", "Jepang"),
        ("io", "Japonia"),
        ("is", "Japan"),
        ("it", "Giappone"),
        ("iu", "ᓃᑉᐊᓐ"),
        ("ja", "日本"),
        ("ka", "იაპონია"),
        ("ki", "Japan"),
        ("kk", "Жапония"),
        ("kl", "Japan"),
        ("km", "ជប\u{17c9}\u{17bb}ន"),
        ("kn", "ಜಪಾನ\u{ccd}"),
        ("ko", "일본"),
        ("ku", "Japonya"),
        ("kv", "Япония"),
        ("kw", "Nihon"),
        ("ky", "Жапония"),
        ("lo", "ປະເທດຍ\u{eb5}\u{ec8}ປ\u{eb8}\u{ec8}ນ"),
        ("lt", "Japonija"),
        ("lv", "Japāna"),
        ("mi", "Nipono"),
        ("mk", "Јапонија"),
        ("ml", "ജപ\u{d4d}പ\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Япон"),
        ("mr", "जपान"),
        ("ms", "Jepun"),
        ("mt", "Ġappun"),
        ("my", "ဂျပန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Djapan"),
        ("nb", "Japan"),
        ("ne", "जापान"),
        ("nl", "Japan"),
        ("nn", "Japan"),
        ("nv", "Binaʼadaałtzózí Dinéʼiʼ Bikéyah"),
        ("oc", "Japon"),
        ("or", "ଜ\u{b3e}ପ\u{b3e}ନ"),
        ("pa", "ਜਾਪਾਨ"),
        ("pi", "जापान"),
        ("pl", "Japonia"),
        ("ps", "جاپان"),
        ("pt", "Japão"),
        ("pt_BR", "Japão"),
        ("ro", "Japonia"),
        ("ru", "Япония"),
        ("rw", "Ubuyapani"),
        ("sc", "Giapone"),
        ("sd", "جاپان"),
        ("si", "ජප\u{dcf}නය"),
        ("sk", "Japonsko"),
        ("sl", "Japonska"),
        ("so", "Jabbaan"),
        ("sq", "Japoni"),
        ("sr", "Јапан"),
        ("sv", "Japan"),
        ("sw", "Japan"),
        ("ta", "ஜப\u{bcd}ப\u{bbe}ன\u{bcd}"),
        ("te", "జప\u{c3e}న\u{c4d}"),
        ("tg", "Ҷопон"),
        ("th", "ญ\u{e35}\u{e48}ป\u{e38}\u{e48}น"),
        ("ti", "ጃፓን"),
        ("tk", "Ýaponiýa"),
        ("tl", "Hapon"),
        ("tr", "Japonya"),
        ("tt", "Жапан, Япониа"),
        ("ug", "ياپونىيە"),
        ("uk", "Японія"),
        ("ur", "جاپان"),
        ("uz", "Yaponiya"),
        ("ve", "Japan"),
        ("vi", "Nhật"),
        ("wa", "Djapon"),
        ("wo", "Japoŋ"),
        ("xh", "Japhani"),
        ("yo", "Japan"),
        ("zh_CN", "日本"),
        ("zh_HK", "日本"),
        ("zh_TW", "日本"),
        ("zu", "IJaphani"),
    ];
    #[cfg(all(feature = "jp", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 36.204824;
        pub const LONGITUDE: f64 = 138.252924;
        pub const MAX_LATITUDE: f64 = 45.6412626;
        pub const MAX_LONGITUDE: f64 = 154.0031455;
        pub const MIN_LATITUDE: f64 = 20.3585295;
        pub const MIN_LONGITUDE: f64 = 122.8554688;
        pub const NORTHEAST_LATITUDE: f64 = 45.6412626;
        pub const NORTHEAST_LONGITUDE: f64 = 154.0031455;
        pub const SOUTHWEST_LATITUDE: f64 = 20.3585295;
        pub const SOUTHWEST_LONGITUDE: f64 = 122.8554688;
    }
}
#[cfg(all(feature = "jp", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 36.204824,
            longitude: 138.252924,
            max_latitude: 45.6412626,
            max_longitude: 154.0031455,
            min_latitude: 20.3585295,
            min_longitude: 122.8554688,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 45.6412626,
                    longitude: 154.0031455,
                },
                southwest: CountryGeoBound {
                    latitude: 20.3585295,
                    longitude: 122.8554688,
                },
            },
        }
    }
}

#[cfg(all(feature = "jp", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::JP,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0), longitude: Some(143.0), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hokkaido"), ("ar", "هوكايدو"), ("az", "Hokkaydo (prefektura)"), ("be", "Губернатарства Хакайда"), ("bg", "Хокайдо"), ("bn", "হোক\u{9cd}ক\u{9be}ইদো"), ("ca", "Prefectura d’Hokkaidō"), ("ccp", "𑄦\u{11127}𑄇\u{11134}𑄇\u{1112d}𑄓\u{1112e}"), ("ceb", "Hokkaidō (prepektura sa Hapon)"), ("cs", "Hokkaidó"), ("da", "Hokkaido"), ("de", "Präfektur Hokkaido"), ("el", "Χοκκάιντο"), ("en", "Hokkaidō"), ("es", "Hokkaidō"), ("et", "Hokkaidō prefektuur"), ("eu", "Hokkaido"), ("fa", "هوکایدو"), ("fi", "Hokkaidō"), ("fr", "préfecture de Hokkaidō"), ("gl", "Hokkaido"), ("gu", "હોક\u{ac8}ડો"), ("he", "הוקאידו"), ("hi", "होक\u{94d}काइदो"), ("hr", "Hokkaido, prefektura"), ("hu", "Hokkaidó prefektúra"), ("hy", "Հոկայդո"), ("id", "Prefektur Hokkaido"), ("is", "Hokkaidō"), ("it", "Prefettura di Hokkaidō"), ("ja", "北海道"), ("jv", "Hokkaido"), ("ka", "ჰოკაიდო"), ("km", "ត\u{17c6}បន\u{17cb}ហ\u{17bb}កកៃដ\u{17bc}"), ("kn", "ಹೊಕೈಡೋ"), ("ko", "홋카이도"), ("lt", "Hokaidas"), ("lv", "Hokaido"), ("ml", "ഹൊക\u{d4d}കൈഡൊ"), ("mn", "Хоккайдо"), ("mr", "होक\u{94d}काइदो"), ("ms", "Hokkaidō"), ("nb", "Hokkaido"), ("nl", "Prefectuur Hokkaidō"), ("no", "Hokkaido"), ("pl", "Hokkaido"), ("pt", "Hokkaido"), ("ro", "Prefectura Hokkaidō"), ("ru", "Хоккайдо"), ("si", "හොක\u{dca}කය\u{dd2}ඩෝ"), ("sk", "Hokkaido"), ("sl", "Hokaido"), ("sr", "Хокаидо"), ("sr_Latn", "Hokaido"), ("sv", "Hokkaido prefektur"), ("sw", "Mkoa wa Hokkaidō"), ("ta", "ஹொக\u{bcd}கைடோ"), ("te", "హ\u{c4a}క\u{c4d}క\u{c3e}య\u{c3f}డ\u{c4b}"), ("th", "เกาะฮอกไกโด"), ("tr", "Hokkaidō"), ("uk", "Префектура Хоккайдо"), ("ur", "ہوکائیدو"), ("vi", "Hokkaidō"), ("yue", "北海道"), ("yue_Hans", "北海道"), ("zh", "北海道")]),
                        unofficial_name_list: ["Hokkaido", "Hokkaidô"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::JP,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.822072), longitude: Some(140.7473647), max_latitude: Some(40.9702795), min_latitude: Some(40.6059389), max_longitude: Some(140.9808429), min_longitude: Some(140.5202819)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aomori Prefektuur"), ("ar", "آوموري"), ("az", "Aomori"), ("be", "прэфекрута Аомары"), ("bg", "Аомори"), ("bn", "আওমোরি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Aomori"), ("ccp", "𑄃𑄟\u{1112e}𑄢\u{11128}"), ("ceb", "Aomori-ken"), ("cs", "Prefektura Aomori"), ("cy", "Aomori"), ("da", "Aomori-præfekturet"), ("de", "Präfektur Aomori"), ("el", "Αομόρι"), ("en", "Aomori"), ("es", "Prefectura de Aomori"), ("et", "Aomori prefektuur"), ("eu", "Aomori"), ("fa", "استان آئوموری"), ("fi", "Aomorin prefektuuri"), ("fr", "préfecture d’Aomori"), ("ga", "Maoracht Aomori"), ("gl", "Prefectura de Aomori"), ("gu", "ઓમોરી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אאומורי"), ("hi", "आओमोरी प\u{94d}रीफ\u{93c}\u{947}क\u{94d}चर"), ("hr", "Aomori"), ("hu", "Aomori prefektúra"), ("hy", "Աոմորի"), ("id", "Prefektur Aomori"), ("it", "prefettura di Aomori"), ("ja", "青森県"), ("jv", "Prefektur Aomori"), ("ka", "აომორის პრეფექტურა"), ("kn", "ಅಮೊರ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "아오모리 현"), ("lt", "Aomorio prefektūra"), ("lv", "Aomori prefektūra"), ("mk", "Аомори"), ("mn", "Аомори"), ("mr", "ओमोरी"), ("ms", "Wilayah Aomori"), ("my", "အအ\u{102d}\u{102f}မ\u{102d}\u{102f}ရ\u{102d}ခရ\u{102d}\u{102f}င\u{103a}"), ("nb", "Aomori"), ("nl", "Aomori"), ("no", "Aomori"), ("pl", "Prefektura Aomori"), ("pt", "Aomori"), ("ro", "Prefectura Aomori"), ("ru", "Аомори"), ("si", "අඔමොර\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Aomori"), ("sl", "prefektura Aomori"), ("sr", "Аомори"), ("sr_Latn", "Aomori"), ("sv", "Aomori prefektur"), ("sw", "Mkoa wa Aomori"), ("ta", "அமர\u{bc0} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఔమ\u{c4b}ర\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c3f}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอะโอะโมะร\u{e34}"), ("tr", "Aomori"), ("uk", "Префектура Аоморі"), ("ur", "اوموری پریفیکچر"), ("vi", "Aomori"), ("yue", "青森縣"), ("yue_Hans", "青森县"), ("zh", "青森縣")]),
                        unofficial_name_list: ["Aomori"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::JP,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7036194), longitude: Some(141.1526839), max_latitude: Some(40.4504511), min_latitude: Some(38.74752), max_longitude: Some(142.0725079), min_longitude: Some(140.6530098)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Iwate Prefektuur"), ("ar", "إيواته"), ("az", "İvate"), ("be", "Прэфектура Іватэ"), ("bg", "Ивате"), ("bn", "ইওয\u{9bc}\u{9be}তে প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Iwate"), ("ccp", "𑄃\u{1112d}𑄃\u{1112e}𑄠𑄖\u{11134}"), ("ceb", "Iwate-ken"), ("cs", "Prefektura Iwate"), ("cy", "Iwate"), ("da", "Iwate-præfekturet"), ("de", "Präfektur Iwate"), ("el", "Ιβάτε"), ("en", "Iwate"), ("es", "Prefectura de Iwate"), ("et", "Iwate prefektuur"), ("eu", "Iwate"), ("fa", "استان ایواته"), ("fi", "Iwaten prefektuuri"), ("fr", "préfecture d’Iwate"), ("ga", "Maoracht Iwate"), ("gl", "Prefectura de Iwate"), ("gu", "ઇવ\u{ac7}ટ પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "איווטה"), ("hi", "इवात\u{947} प\u{94d}रीफ\u{93c}\u{947}क\u{94d}चर"), ("hr", "Iwate"), ("hu", "Ivate prefektúra"), ("hy", "Իվատե"), ("id", "Prefektur Iwate"), ("it", "prefettura di Iwate"), ("ja", "岩手県"), ("jv", "Prefektur Iwate"), ("ka", "ივატეს პრეფექტურა"), ("km", "ខេត\u{17d2}តអ\u{17ca}\u{17b8}វ\u{17c9}ាត\u{17b7}"), ("kn", "ಐವೇಟ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "이와테 현"), ("lt", "Ivatės prefektūra"), ("lv", "Ivates prefektūra"), ("mk", "Ивате"), ("mn", "Иватэ"), ("mr", "इवात\u{947}"), ("ms", "Wilayah Iwate"), ("nb", "Iwate"), ("nl", "Iwate"), ("no", "Iwate"), ("pl", "Prefektura Iwate"), ("pt", "Iwate"), ("ro", "Prefectura Iwate"), ("ru", "Иватэ"), ("si", "අය\u{dd2}වටේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Iwate"), ("sr", "Ивате"), ("sr_Latn", "Ivate"), ("sv", "Iwate"), ("sw", "Mkoa wa Iwate"), ("ta", "இவ\u{bbe}ட\u{bcd}டே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఇవ\u{c3e}ట\u{c46} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}วะเตะ"), ("tr", "Iwate"), ("uk", "Префектура Івате"), ("ur", "ایواتے پریفیکچر"), ("vi", "Iwate"), ("yue", "岩手縣"), ("yue_Hans", "岩手县"), ("zh", "岩手县")]),
                        unofficial_name_list: ["Iwate"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::JP,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.2688373), longitude: Some(140.8721), max_latitude: Some(39.0028588), min_latitude: Some(37.7738621), max_longitude: Some(141.6754538), min_longitude: Some(140.2746668)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Miyagi Prefektuur"), ("ar", "مياغي"), ("az", "Miyaqi"), ("be", "прэфектура Міягі"), ("bg", "Мияги"), ("bn", "মিয\u{9bc}\u{9be}গি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Miyagi"), ("ccp", "𑄟\u{11128}𑄠𑄉\u{11128}"), ("ceb", "Miyagi-ken"), ("cs", "Prefektura Mijagi"), ("cy", "Miyagi"), ("da", "Miyagi-præfekturet"), ("de", "Präfektur Miyagi"), ("el", "Μιγιάγκι"), ("en", "Miyagi"), ("es", "Miyagi"), ("et", "Miyagi prefektuur"), ("eu", "Miyagi prefektura"), ("fa", "استان میاگی"), ("fi", "Miyagin prefektuuri"), ("fr", "préfecture de Miyagi"), ("ga", "Maoracht Miyagi"), ("gl", "Prefectura de Miyagi"), ("gu", "મિયાગી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "מיאגי"), ("hi", "मियागी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Miyagi"), ("hu", "Mijagi prefektúra"), ("hy", "Միյագի"), ("id", "Prefektur Miyagi"), ("it", "prefettura di Miyagi"), ("ja", "宮城県"), ("ka", "მიაგის პრეფექტურა"), ("kn", "ಮ\u{cbf}ಯಾಗ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "미야기 현"), ("lt", "Mijagio prefektūra"), ("lv", "Mijagi prefektūra"), ("mk", "Мијаги"), ("mn", "Мияги"), ("mr", "मियागी"), ("ms", "Wilayah Miyagi"), ("nb", "Miyagi"), ("nl", "Miyagi"), ("no", "Miyagi"), ("pl", "Prefektura Miyagi"), ("pt", "Miyagi"), ("ro", "Prefectura Miyagi"), ("ru", "Мияги"), ("si", "ම\u{dd2}ය\u{dcf}ග\u{dd3} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mijagi"), ("sl", "prefektura Miyagi"), ("sr", "Мијаги"), ("sr_Latn", "Mijagi"), ("sv", "Miyagi prefektur"), ("sw", "Mkoa wa Miyagi"), ("ta", "மிய\u{bbe}கி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "మ\u{c3f}య\u{c3e}గ\u{c40} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}ยะง\u{e34}"), ("tr", "Miyagi"), ("uk", "Префектура Міяґі"), ("ur", "میاگی پریفیکچر"), ("vi", "Miyagi"), ("yue", "宮城縣"), ("yue_Hans", "宫城县"), ("zh", "宮城縣")]),
                        unofficial_name_list: ["Miyagi"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::JP,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.7200079), longitude: Some(140.1025642), max_latitude: Some(39.86527460000001), min_latitude: Some(39.4487449), max_longitude: Some(140.5154197), min_longitude: Some(140.0205635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Akita Prefektuur"), ("ar", "أكيتا"), ("az", "Akita"), ("be", "Акіта"), ("bg", "Акита"), ("bn", "আকিত\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "prefectura d’Akita"), ("ccp", "𑄃𑄇\u{11128}𑄑"), ("ceb", "Akita-ken"), ("cs", "Prefektura Akita"), ("cy", "Akita"), ("da", "Akita-præfekturet"), ("de", "Präfektur Akita"), ("el", "Επαρχία Ακίτα"), ("en", "Akita"), ("es", "Akita"), ("et", "Akita prefektuur"), ("eu", "Akita"), ("fa", "استان آکیتا"), ("fi", "Akitan prefektuuri"), ("fr", "préfecture d’Akita"), ("ga", "Maoracht Akita"), ("gl", "Prefectura de Akita"), ("gu", "અકિટા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אקיטה"), ("hi", "अकिता प\u{94d}रीफ\u{93c}\u{947}क\u{94d}चर"), ("hr", "Akita"), ("hu", "Akita prefektúra"), ("hy", "Ակիտա"), ("id", "Prefektur Akita"), ("it", "prefettura di Akita"), ("ja", "秋田県"), ("jv", "Prefektur Akita"), ("ka", "აკიტის პრეფექტურა"), ("km", "ខេត\u{17d2}តអាគ\u{17b8}តា"), ("kn", "ಅಕ\u{cbf}ಟಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "아키타 현"), ("lt", "Akitos prefektūra"), ("lv", "Akitas prefektūra"), ("mk", "Акита"), ("mn", "Акита"), ("mr", "अकिता"), ("ms", "Wilayah Akita"), ("nb", "Akita"), ("nl", "Akita"), ("no", "Akita"), ("pa", "ਅਕਿਤਾ ਪ\u{a4d}ਰੀਫ\u{a47}ਕਚਰ"), ("pl", "Prefektura Akita"), ("pt", "Akita"), ("ro", "Prefectura Akita"), ("ru", "Акита"), ("si", "අක\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Akita"), ("sl", "prefektura Akita"), ("sr", "Акита"), ("sr_Latn", "Akita"), ("sv", "Akita prefektur"), ("sw", "Mkoa wa Akita"), ("ta", "ஆகிட\u{bcd}ட\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "అక\u{c3f}త\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอะก\u{e34}ตะ"), ("tr", "Akita"), ("uk", "Префектура Акіта"), ("ur", "اکیتا پریفیکچر"), ("vi", "Akita"), ("yue", "秋田縣"), ("yue_Hans", "秋田县"), ("zh", "秋田县")]),
                        unofficial_name_list: ["Akita"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::JP,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.2554388), longitude: Some(140.3396017), max_latitude: Some(38.3521169), min_latitude: Some(38.1435715), max_longitude: Some(140.5307099), min_longitude: Some(140.1793339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yamagata Prefektuur"), ("ar", "ياماغاتا"), ("az", "Yamaqata"), ("be", "Прэфектура Ямагата"), ("bg", "Ямагата"), ("bn", "য\u{9bc}\u{9be}ম\u{9be}গ\u{9be}ত\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Yamagata"), ("ccp", "𑄃\u{11128}𑄠𑄟𑄉𑄑"), ("ceb", "Yamagata-ken"), ("cs", "Prefektura Jamagata"), ("cy", "Yamagata"), ("da", "Yamagata-præfekturet"), ("de", "Präfektur Yamagata"), ("el", "Γιαμαγκάτα"), ("en", "Yamagata"), ("es", "Prefectura de Yamagata"), ("et", "Yamagata prefektuur"), ("eu", "Yamagata"), ("fa", "استان یاماگاتا"), ("fi", "Yamagatan prefektuuri"), ("fr", "préfecture de Yamagata"), ("ga", "Maoracht Yamagata"), ("gl", "Prefectura de Yamagata"), ("gu", "યમાગાતા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "יאמאגטה"), ("hi", "यामागाटा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Yamagata"), ("hu", "Jamagata prefektúra"), ("id", "Prefektur Yamagata"), ("it", "prefettura di Yamagata"), ("ja", "山形県"), ("ka", "იამაგატის პრეფექტურა"), ("km", "ខេត\u{17d2}តយ\u{17c9}ាម\u{17c9}ាកាតា"), ("kn", "ಯಮಗಾಟಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "야마가타 현"), ("lt", "Jamagatos prefektūra"), ("lv", "Jamagatas prefektūra"), ("mk", "Јамагата"), ("mn", "Ямагата-кэн"), ("mr", "यामागाता"), ("ms", "Wilayah Yamagata"), ("nb", "Yamagata"), ("nl", "Yamagata"), ("no", "Yamagata"), ("pl", "Prefektura Yamagata"), ("pt", "Yamagata"), ("ro", "Prefectura Yamagata"), ("ru", "Ямагата"), ("si", "යමග\u{dcf}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jamagata"), ("sr", "Јамагата"), ("sr_Latn", "Jamagata"), ("sv", "Yamagata prefektur"), ("sw", "Mkoa wa Yamagata"), ("ta", "யம\u{bbe}ட\u{bcd}ட\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "యమగ\u{c3e}త\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดยะมะงะตะ"), ("tr", "Yamagata"), ("uk", "Префектура Ямаґата"), ("ur", "یاماگاتا پرفکترے"), ("vi", "Yamagata"), ("yue", "山形縣"), ("yue_Hans", "山形县"), ("zh", "山形县")]),
                        unofficial_name_list: ["Yamagata"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::JP,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.7608337), longitude: Some(140.4747282), max_latitude: Some(37.9766402), min_latitude: Some(37.6243521), max_longitude: Some(140.570933), min_longitude: Some(140.2299783)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Fukushima Prefektuur"), ("ar", "فوكوشيما"), ("az", "Fukusima"), ("be", "прэфектура Фукусіма"), ("bg", "Фукушима"), ("bn", "ফ\u{9c1}ক\u{9c1}শিম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Fukushima"), ("ccp", "𑄜\u{1112a}𑄇\u{1112a}𑄥\u{11128}𑄟"), ("ceb", "Fukushima-ken"), ("cs", "Prefektura Fukušima"), ("cy", "Fukushima"), ("da", "Fukushima-præfekturet"), ("de", "Präfektur Fukushima"), ("el", "Φουκουσίμα"), ("en", "Fukushima"), ("es", "Prefectura de Fukushima"), ("et", "Fukushima prefektuur"), ("eu", "Fukushima"), ("fa", "استان فوکوشیما"), ("fi", "Fukushiman prefektuuri"), ("fr", "préfecture de Fukushima"), ("ga", "Maoracht Fukushima"), ("gl", "Prefectura de Fukushima"), ("gu", "ફ\u{ac2}ક\u{ac1}શીમા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "פוקושימה"), ("hi", "फ\u{93c}\u{942}क\u{942}शिमा प\u{94d}रीफ\u{93c}\u{947}क\u{94d}चर"), ("hr", "Fukushima"), ("hu", "Fukusima prefektúra"), ("hy", "Ֆուկուսիմա"), ("id", "Prefektur Fukushima"), ("it", "prefettura di Fukushima"), ("ja", "福島県"), ("jv", "Prefektur Fukushima"), ("ka", "ფუკუსიმის პრეფექტურა"), ("kn", "ಫುಕುಶ\u{cbf}ಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "후쿠시마 현"), ("lt", "Fukušimos prefektūra"), ("lv", "Fukušima"), ("mk", "Фукушима"), ("mn", "Фүкүшима"), ("mr", "फ\u{941}क\u{941}शिमा"), ("ms", "Wilayah Fukushima"), ("nb", "Fukushima"), ("nl", "Fukushima"), ("no", "Fukushima"), ("pl", "Prefektura Fukushima"), ("pt", "Fukushima"), ("ro", "Prefectura Fukushima"), ("ru", "Фукусима"), ("si", "ෆ\u{dd4}ක\u{dd4}ෂ\u{dd2}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Fukušima"), ("sl", "prefektura Fukušima"), ("sr", "Фукушима"), ("sr_Latn", "Fukušima"), ("sv", "Fukushima prefektur"), ("sw", "Mkoa wa Fukushima"), ("ta", "பிக\u{bcd}குஷிம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ఫుకుష\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟ\u{e38}ก\u{e38}ช\u{e34}มะ"), ("tr", "Fukuşima Prefektörlüğü"), ("uk", "Префектура Фукушіма"), ("ur", "فوکوشیما پریفیکچر"), ("vi", "Fukushima"), ("yue", "福島縣"), ("yue_Hans", "福岛县"), ("zh", "福岛县")]),
                        unofficial_name_list: ["Fukushima", "Hukusima"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::JP,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.3418112), longitude: Some(140.4467935), max_latitude: Some(36.9439966), min_latitude: Some(35.7401721), max_longitude: Some(140.8495266), min_longitude: Some(139.6874576)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ibaraki Prefektuur"), ("ar", "إيباراكي"), ("az", "İbaraki"), ("be", "прэфектура Ібаракі"), ("bg", "Ибараки"), ("bn", "ইব\u{9be}র\u{9be}কি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Ibaraki"), ("ccp", "𑄃\u{1112d}𑄝𑄢𑄇\u{11128}"), ("ceb", "Ibaraki-ken"), ("cs", "Prefektura Ibaraki"), ("cy", "Ibaraki"), ("da", "Ibaraki-præfekturet"), ("de", "Präfektur Ibaraki"), ("el", "Επαρχία Ιμπαράκι"), ("en", "Ibaraki"), ("es", "Prefectura de Ibaraki"), ("et", "Ibaraki prefektuur"), ("eu", "Ibaraki"), ("fa", "استان ایباراکی"), ("fi", "Ibarakin prefektuuri"), ("fr", "préfecture d’Ibaraki"), ("ga", "Maoracht Ibaraki"), ("gl", "Prefectura de Ibaraki"), ("gu", "ઇબારાકી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "איבראקי"), ("hi", "आईबराकी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Ibaraki"), ("hu", "Ibaraki prefektúra"), ("id", "Prefektur Ibaraki"), ("it", "prefettura di Ibaraki"), ("ja", "茨城県"), ("jv", "Prefektur Ibaraki"), ("ka", "იბარაკის პრეფექტურა"), ("km", "ខេត\u{17d2}តអ\u{17ca}\u{17b8}បារ\u{17c9}ាគ\u{17b7}"), ("kn", "ಐಬರಾಕ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "이바라키 현"), ("lt", "Ibarakio prefektūra"), ("lv", "Ibaraki prefektūra"), ("mk", "Ибараки"), ("mn", "Ибараки"), ("mr", "इबाराकी"), ("ms", "Wilayah Ibaraki"), ("nb", "Ibaraki"), ("nl", "Ibaraki"), ("no", "Ibaraki"), ("pl", "Prefektura Ibaraki"), ("pt", "Ibaraki"), ("ro", "Prefectura Ibaraki"), ("ru", "Ибараки"), ("si", "ඉබරක\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ibaraki"), ("sl", "prefektura Ibaraki"), ("sr", "Ибараки"), ("sr_Latn", "Ibaraki"), ("sv", "Ibaraki prefektur"), ("sw", "Mkoa wa Ibaraki"), ("ta", "இபர\u{bbe}க\u{bcd}கி பிரேபிக\u{bcd}டூர\u{bcd}"), ("te", "ఇబర\u{c3e}క\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}บะระก\u{e34}"), ("tr", "Ibaraki"), ("uk", "Префектура Ібаракі"), ("ur", "ایباراکی پریفیکچر"), ("vi", "Ibaraki"), ("yue", "茨城縣"), ("yue_Hans", "茨城县"), ("zh", "茨城縣")]),
                        unofficial_name_list: ["Ibaraki"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::JP,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.565725), longitude: Some(139.8835651), max_latitude: Some(37.15508320000001), min_latitude: Some(36.1979495), max_longitude: Some(140.2924768), min_longitude: Some(139.3265571)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tochigi Prefektuur"), ("ar", "توتشيغي"), ("az", "Totiqi"), ("be", "прэфектура Татыгі"), ("bg", "Точиги"), ("bn", "তোচিগি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Tochigi"), ("ccp", "𑄑\u{1112e}𑄌\u{11128}𑄉\u{11128}"), ("ceb", "Tochigi-ken"), ("cs", "Prefektura Točigi"), ("cy", "Tochigi"), ("da", "Tochigi-præfekturet"), ("de", "Präfektur Tochigi"), ("el", "Τοτσίγκι"), ("en", "Tochigi"), ("es", "Prefectura de Tochigi"), ("et", "Tochigi prefektuur"), ("eu", "Tochigi"), ("fa", "استان توچیگی"), ("fi", "Tochigin prefektuuri"), ("fr", "préfecture de Tochigi"), ("ga", "Maoracht Tochigi"), ("gl", "Prefectura de Tochigi"), ("gu", "ટોચીગી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "טוצ׳יגי"), ("hi", "तोशीगी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Tochigi"), ("hu", "Tocsigi prefektúra"), ("id", "Prefektur Tochigi"), ("it", "prefettura di Tochigi"), ("ja", "栃木県"), ("kn", "ತೋಚ\u{cbf}ಗ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "도치기 현"), ("lt", "Točigio prefektūra"), ("lv", "Točigi prefektūra"), ("mk", "Точиги"), ("mr", "तोचिगी"), ("ms", "Wilayah Tochigi"), ("nb", "Tochigi"), ("nl", "Tochigi"), ("no", "Tochigi"), ("pl", "Prefektura Tochigi"), ("pt", "Tochigi"), ("ro", "Prefectura Tochigi"), ("ru", "Тотиги"), ("si", "ටොච\u{dd2}ග\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Točigi"), ("sl", "prefektura Tochigi"), ("sr", "Точиги"), ("sr_Latn", "Točigi"), ("sv", "Tochigi prefektur"), ("sw", "Mkoa wa Tochigi"), ("ta", "டோச\u{bcd}சிகி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ట\u{c4b}చ\u{c3f}గ\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโทะช\u{e34}ง\u{e34}"), ("tr", "Tochigi"), ("uk", "Префектура Точіґі"), ("ur", "توچیگی پریفیکچر"), ("vi", "Tochigi"), ("yue", "栃木縣"), ("yue_Hans", "栃木县"), ("zh", "栃木縣")]),
                        unofficial_name_list: ["Tochigi", "Totigi"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::JP,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.3906675), longitude: Some(139.0604061), max_latitude: Some(37.0586062), min_latitude: Some(35.9848313), max_longitude: Some(139.6670785), min_longitude: Some(138.3968196)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gunma Prefektuur"), ("ar", "غونما"), ("az", "Qumma"), ("be", "прэфектура Гуму"), ("bg", "Гунма"), ("bn", "গ\u{9c1}ন\u{9cd}\u{200c}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Gunma"), ("ccp", "𑄉\u{1112a}𑄚\u{11134}𑄟"), ("ceb", "Gunma-ken"), ("cs", "Prefektura Gunma"), ("cy", "Gunma"), ("da", "Gunma-præfekturet"), ("de", "Präfektur Gunma"), ("el", "Γκούνμα"), ("en", "Gunma"), ("es", "Prefectura de Gunma"), ("et", "Gumma prefektuur"), ("eu", "Gunma"), ("fa", "استان گونما"), ("fi", "Gunman prefektuuri"), ("fr", "préfecture de Gunma"), ("ga", "Maoracht Gunma"), ("gl", "Prefectura de Gunma"), ("gu", "ગ\u{ac1}\u{a82}મા પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "גונמה"), ("hi", "ग\u{941}नमा प\u{94d}रान\u{94d}त"), ("hr", "Gunma"), ("hu", "Gunma prefektúra"), ("hy", "Գումմա"), ("id", "Prefektur Gunma"), ("it", "prefettura di Gunma"), ("ja", "群馬県"), ("jv", "Prefektur Gunma"), ("km", "ខេត\u{17d2}តហ\u{17d2}គ\u{17ba}នម\u{17c9}ា"), ("kn", "ಗುನ\u{ccd}ಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "군마 현"), ("lt", "Gunmos prefektūra"), ("lv", "Gummas prefektūra"), ("mk", "Гунма"), ("mr", "ग\u{941}न\u{94d}मा"), ("ms", "Wilayah Gunma"), ("nb", "Gunma"), ("nl", "Gunma"), ("no", "Gunma"), ("pl", "Prefektura Gunma"), ("pt", "Gunma"), ("ro", "Prefectura Gunma"), ("ru", "Гумма"), ("si", "ග\u{dd4}න\u{dca}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Gunma"), ("sl", "prefektura Gunma"), ("sr", "Гунма"), ("sr_Latn", "Gunma"), ("sv", "Gunma prefektur"), ("sw", "Mkoa wa Gunma"), ("ta", "குணம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "గన\u{c4d}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e38}มมะ"), ("tr", "Gunma"), ("uk", "Префектура Ґумма"), ("ur", "گونما پریفیکچر"), ("vi", "Gunma"), ("yue", "群馬縣"), ("yue_Hans", "群马县"), ("zh", "群馬縣")]),
                        unofficial_name_list: ["Gunma"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::JP,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.8617292), longitude: Some(139.6454822), max_latitude: Some(36.0025176), min_latitude: Some(35.8286783), max_longitude: Some(139.7574035), min_longitude: Some(139.5401082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saitama Prefektuur"), ("ar", "سايتاما"), ("az", "Saytama"), ("be", "Прэфектура Сайтама"), ("bg", "Сайтама"), ("bn", "স\u{9be}ইত\u{9be}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Saitama"), ("ccp", "𑄥\u{1112d}𑄑\u{11127}𑄟"), ("ceb", "Saitama-ken"), ("cs", "Prefektura Saitama"), ("cy", "Saitama"), ("da", "Saitama-præfekturet"), ("de", "Präfektur Saitama"), ("el", "Σαϊτάμα"), ("en", "Saitama"), ("es", "Prefectura de Saitama"), ("et", "Saitama prefektuur"), ("eu", "Saitama"), ("fa", "استان سایتاما"), ("fi", "Saitaman prefektuuri"), ("fr", "préfecture de Saitama"), ("ga", "Maoracht Saitama"), ("gl", "Prefectura de Saitama"), ("gu", "સ\u{ac8}તામા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "סאיטאמה"), ("hi", "स\u{948}तामा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Saitama"), ("hu", "Szaitama prefektúra"), ("hy", "Սայտամա"), ("id", "Prefektur Saitama"), ("it", "prefettura di Saitama"), ("ja", "埼玉県"), ("km", "ខេត\u{17d2}តសៃតាម\u{17c9}ា"), ("kn", "ಸೈತಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "사이타마 현"), ("lt", "Saitamos prefektūra"), ("lv", "Saitamas prefektūra"), ("mk", "Сајтама"), ("mn", "Сайтама аймаг"), ("mr", "स\u{948}तामा"), ("ms", "Wilayah Saitama"), ("nb", "Saitama"), ("nl", "Saitama"), ("no", "Saitama"), ("pl", "Prefektura Saitama"), ("pt", "Saitama"), ("ro", "Prefectura Saitama"), ("ru", "Сайтама"), ("si", "සය\u{dd2}ට\u{dcf}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Saitama"), ("sl", "prefektura Saitama"), ("sr", "Саитама"), ("sr_Latn", "Saitama"), ("sv", "Saitama prefektur"), ("sw", "Mkoa wa Saitama"), ("ta", "சைடம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "స\u{c3e}య\u{c3f}ట\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไซตะมะ"), ("tr", "Saitama ili"), ("uk", "Префектура Сайтама"), ("ur", "سایتاما"), ("uz", "Saitama"), ("vi", "Saitama"), ("yue", "埼玉縣"), ("yue_Hans", "埼玉县"), ("zh", "埼玉縣")]),
                        unofficial_name_list: ["Saitama"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::JP,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.6072668), longitude: Some(140.1062907), max_latitude: Some(35.7148402), min_latitude: Some(35.4936015), max_longitude: Some(140.3008405), min_longitude: Some(140.0182802)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chiba Prefektuur"), ("ar", "تشيبا"), ("az", "Tiba"), ("be", "Прэфектура Тыба"), ("bg", "Чиба"), ("bn", "চিব\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Chiba"), ("ccp", "𑄌\u{11128}𑄝"), ("ceb", "Chiba-ken"), ("cs", "Prefektura Čiba"), ("cy", "Chiba"), ("da", "Chiba-præfekturet"), ("de", "Präfektur Chiba"), ("el", "Τσίμπα"), ("en", "Chiba"), ("es", "Chiba"), ("et", "Chiba prefektuur"), ("eu", "Chiba"), ("fa", "استان چیبا"), ("fi", "Chiban prefektuuri"), ("fr", "préfecture de Chiba"), ("ga", "Maoracht Chiba"), ("gl", "Prefectura de Chiba"), ("gu", "ચિબા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "צ׳יבה"), ("hi", "शिबा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Chiba"), ("hu", "Csiba prefektúra"), ("id", "Prefektur Chiba"), ("it", "prefettura di Chiba"), ("ja", "千葉県"), ("jv", "Prefektur Chiba"), ("ka", "ტიბის პრეფექტურა"), ("km", "ខេត\u{17d2}តឈ\u{17b8}បា"), ("kn", "ಚ\u{cbf}ಬಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "지바 현"), ("lt", "Čibos prefektūra"), ("lv", "Čibas prefektūra"), ("mk", "Чиба"), ("mn", "Чиба-кэн"), ("mr", "चिबा"), ("ms", "Wilayah Chiba"), ("nb", "Chiba"), ("nl", "Chiba"), ("no", "Chiba"), ("pl", "Prefektura Chiba"), ("pt", "Chiba"), ("ro", "Prefectura Chiba"), ("ru", "Тиба"), ("si", "ච\u{dd2}බ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Čiba"), ("sl", "Prefektura Čiba"), ("sr", "Чиба"), ("sr_Latn", "Čiba"), ("sv", "Chiba prefektur"), ("sw", "Mkoa wa Chiba"), ("ta", "ச\u{bc0}ப\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "చ\u{c3f}బ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}บะ"), ("tr", "Chiba ili"), ("uk", "Префектура Тіба"), ("ur", "چیبا پریفیکچر"), ("vi", "Chiba"), ("yue", "千葉縣"), ("yue_Hans", "千叶县"), ("zh", "千葉縣")]),
                        unofficial_name_list: ["Chiba", "Tiba"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::JP,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7090259), longitude: Some(139.7319925), max_latitude: Some(35.8175168), min_latitude: Some(35.5208632), max_longitude: Some(139.9198562), min_longitude: Some(139.5629047)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tokio"), ("am", "ቶክዮ"), ("ar", "طوكيو"), ("as", "টকিঅ’"), ("az", "Tokio"), ("be", "Токіа"), ("bg", "Токио"), ("bn", "টোকিও"), ("bs", "Tokio"), ("ca", "Tòquio"), ("ccp", "𑄑\u{1112e}𑄇\u{11128}𑄃\u{1112e}"), ("ceb", "Tokyo"), ("cs", "prefektura Tokio"), ("cy", "Tokyo"), ("da", "Tokyo"), ("de", "Präfektur Tokio"), ("el", "Τόκιο"), ("en", "Tokyo"), ("es", "Tokio"), ("et", "Tōkyō prefektuur"), ("eu", "Tokio"), ("fa", "توکیو"), ("fi", "Tokio"), ("fr", "Tokyo"), ("ga", "Tóiceo"), ("gl", "Toquio"), ("gu", "ટોક\u{acd}યો"), ("ha", "Tokyo"), ("ha_NE", "Tokyo"), ("he", "טוקיו"), ("hi", "टोक\u{94d}यो"), ("hr", "Tokio"), ("hu", "Tokió"), ("hy", "Տոկիո"), ("id", "Tokyo"), ("is", "Tókýó"), ("it", "Tokyo"), ("ja", "東京都"), ("jv", "Tokyo"), ("ka", "ტოკიო"), ("kk", "Токио"), ("km", "ត\u{17bc}ក\u{17d2}យ\u{17bc}"), ("kn", "ಟೋಕ\u{ccd}ಯೊ"), ("ko", "도쿄 도"), ("ky", "Токио"), ("lo", "ໂຕກຽວ"), ("lt", "Tokijas"), ("lv", "Tokija"), ("mk", "Токио"), ("ml", "ടോക\u{d4d}കിയോ"), ("mn", "Токио"), ("mr", "तोक\u{94d}यो"), ("ms", "Tokyo"), ("my", "တ\u{102d}\u{102f}ကျ\u{102d}\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Tokyo"), ("ne", "टोकियो"), ("nl", "Tokio"), ("no", "Tokyo"), ("or", "ଟୋକ\u{b3f}ଓ"), ("pa", "ਟ\u{a4b}ਕੀਓ"), ("pl", "Tokio"), ("ps", "توکيو"), ("pt", "Tóquio"), ("ro", "Tokio"), ("ru", "Токио"), ("si", "ටෝක\u{dd2}යෝ"), ("sk", "Tokio"), ("sl", "Tokio"), ("so", "Tokyo"), ("sq", "Tokjo"), ("sr", "Токио"), ("sr_Latn", "Tokio"), ("sv", "Tokyo prefektur"), ("sw", "Tokyo"), ("ta", "தோக\u{bcd}கியோ"), ("te", "ట\u{c4b}క\u{c4d}య\u{c4b}"), ("th", "โตเก\u{e35}ยว"), ("tk", "Tokio"), ("tr", "Tokyo"), ("uk", "префектура Токіо"), ("ur", "توکیو"), ("uz", "Tokio"), ("vi", "Tokyo"), ("yo", "Tokyo"), ("yo_BJ", "Tokyo"), ("yue", "東京都"), ("yue_Hans", "东京都"), ("zh", "東京都"), ("zu", "ITokyo")]),
                        unofficial_name_list: ["Tokio", "Tokyo"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::JP,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.4769011), longitude: Some(139.6294307), max_latitude: Some(35.5063336), min_latitude: Some(35.4635704), max_longitude: Some(139.6691916), min_longitude: Some(139.5677915)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kanagawa Prefektuur"), ("ar", "كاناغاوا"), ("az", "Kanaqava"), ("be", "прэфектура Канагава"), ("bg", "Канагава"), ("bn", "ক\u{9be}ন\u{9be}গ\u{9be}ওয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kanagawa"), ("ccp", "𑄇𑄚\u{11134}𑄉𑄃\u{1112e}𑄠"), ("ceb", "Kanagawa-ken"), ("cs", "Prefektura Kanagawa"), ("cy", "Kanagawa"), ("da", "Kanagawa-præfekturet"), ("de", "Präfektur Kanagawa"), ("el", "Καναγκάβα"), ("en", "Kanagawa"), ("es", "Prefectura de Kanagawa"), ("et", "Kanagawa prefektuur"), ("eu", "Kanagawa"), ("fa", "استان کاناگاوا"), ("fi", "Kanagawan prefektuuri"), ("fr", "préfecture de Kanagawa"), ("ga", "Maoracht Kanagawa"), ("gl", "Prefectura de Kanagawa"), ("gu", "કનાગાવા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "קנאגווה"), ("hi", "कानागावा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Kanagawa"), ("hu", "Kanagava prefektúra"), ("hy", "Կանագավա"), ("id", "Prefektur Kanagawa"), ("it", "prefettura di Kanagawa"), ("ja", "神奈川県"), ("ka", "კანაგავის პრეფექტურა"), ("km", "ខេត\u{17d2}តខាន\u{17cb}ណាហ\u{17d2}គាវ\u{17c9}ា"), ("kn", "ಕನಗಾವಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "가나가와 현"), ("lt", "Kanagavos prefektūra"), ("lv", "Kanagavas prefektūra"), ("mk", "Канагава"), ("mn", "Канагава"), ("mr", "कनागावा"), ("ms", "Wilayah Kanagawa"), ("nb", "Kanagawa"), ("nl", "Kanagawa"), ("no", "Kanagawa"), ("pl", "Prefektura Kanagawa"), ("pt", "Kanagawa"), ("ro", "Prefectura Kanagawa"), ("ru", "Канагава"), ("si", "කනග\u{dcf}ව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kanagawa"), ("sl", "prefektura Kanagawa"), ("sq", "Kanagawa"), ("sr", "Канагава"), ("sr_Latn", "Kanagava"), ("sv", "Kanagawa prefektur"), ("sw", "Mkoa wa Kanagawa"), ("ta", "கனகவ\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "కనగ\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคะนะงะวะ"), ("tr", "Kanagawa ili"), ("uk", "Префектура Канаґава"), ("ur", "کاناگاوا پریفیکچر"), ("vi", "Kanagawa"), ("yue", "神奈川縣"), ("yue_Hans", "神奈川县"), ("zh", "神奈川縣")]),
                        unofficial_name_list: ["Kanagawa"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::JP,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9161924), longitude: Some(139.0364126), max_latitude: Some(38.0196372), min_latitude: Some(37.678484), max_longitude: Some(139.2668815), min_longitude: Some(138.7842984)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Niigata Prefektuur"), ("ar", "نييغاتا"), ("az", "Niiqata"), ("be", "Прэфектура Ніігата"), ("bg", "Ниигата"), ("bn", "নিইগ\u{9be}ত\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Niigata"), ("ccp", "𑄚\u{11129}𑄉𑄖"), ("ceb", "Niigata-ken"), ("cs", "Prefektura Niigata"), ("cy", "Niigata"), ("da", "Niigata-præfekturet"), ("de", "Präfektur Niigata"), ("el", "Νιιγκάτα"), ("en", "Niigata"), ("es", "Prefectura de Niigata"), ("et", "Niigata prefektuur"), ("eu", "Niigata"), ("fa", "استان نیگاتا"), ("fi", "Niigatan prefektuuri"), ("fr", "préfecture de Niigata"), ("ga", "Maoracht Niigata"), ("gl", "Prefectura de Niigata"), ("gu", "નીગાટા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "ניאיגטה"), ("hi", "निगाता प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Niigata"), ("hu", "Niigata prefektúra"), ("hy", "Նիիգատա"), ("id", "Prefektur Niigata"), ("it", "prefettura di Niigata"), ("ja", "新潟県"), ("jv", "Prefektur Niigata"), ("km", "ខេត\u{17d2}តន\u{17b8}ហ\u{17d2}គាតាក\u{17cb}"), ("kn", "ನ\u{cbf}ಗಾಟಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "니가타 현"), ("lt", "Nijigatos prefektūra"), ("lv", "Niigatas prefektūra"), ("mk", "Ниигата"), ("mr", "निगाता"), ("ms", "Wilayah Niigata"), ("nb", "Niigata"), ("nl", "Niigata"), ("no", "Niigata"), ("pl", "Prefektura Niigata"), ("pt", "Niigata"), ("ro", "Prefectura Niigata"), ("ru", "Ниигата"), ("si", "න\u{dd2}ගට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Niigata"), ("sr", "Ниигата"), ("sr_Latn", "Niigata"), ("sv", "Niigata prefektur"), ("sw", "Mkoa wa Niigata"), ("ta", "ந\u{bc0}கட\u{bcd}ட ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "న\u{c40}గ\u{c3e}ట\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e35}งะตะ"), ("tr", "Niigata"), ("uk", "Префектура Ніїґата"), ("ur", "نیگاتا پریفیکچر"), ("vi", "Niigata"), ("yue", "新潟縣"), ("yue_Hans", "新潟县"), ("zh", "新潟县")]),
                        unofficial_name_list: ["Niigata"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::JP,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.6959518), longitude: Some(137.2136768), max_latitude: Some(36.7667013), min_latitude: Some(36.3698396), max_longitude: Some(137.7055334), min_longitude: Some(137.0281635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Toyama Prefektuur"), ("ar", "توياما"), ("az", "Toyama"), ("be", "прэфектура Таяма"), ("bg", "Тояма"), ("bn", "তোয\u{9bc}\u{9cd}য\u{9be}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Toyama"), ("ccp", "𑄑\u{1112e}𑄠𑄟"), ("ceb", "Toyama-ken"), ("cs", "Prefektura Tojama"), ("cy", "Toyama"), ("da", "Toyama-præfekturet"), ("de", "Präfektur Toyama"), ("el", "Τογιάμα"), ("en", "Toyama"), ("es", "Prefectura de Toyama"), ("et", "Toyama prefektuur"), ("eu", "Toyama"), ("fa", "استان تویاما"), ("fi", "Toyaman prefektuuri"), ("fr", "préfecture de Toyama"), ("ga", "Maoracht Toyama"), ("gl", "Prefectura de Toyama"), ("gu", "ટોયમા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "טויאמה"), ("hi", "तोयामा प\u{94d}रभाग"), ("hr", "Toyama"), ("hu", "Tojama prefektúra"), ("hy", "Տոյամա"), ("id", "Prefektur Toyama"), ("it", "prefettura di Toyama"), ("ja", "富山県"), ("km", "ខេត\u{17d2}តត\u{17bb}យ\u{17c9}ាម\u{17c9}ា"), ("kn", "ಟೊಯಾಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "도야마 현"), ("lt", "Tojamos prefektūra"), ("lv", "Tojamas prefektūra"), ("mk", "Тојама"), ("mr", "तोयामा"), ("ms", "Wilayah Toyama"), ("nb", "Toyama"), ("nl", "Toyama"), ("no", "Toyama"), ("pl", "Prefektura Toyama"), ("pt", "Toyama"), ("ro", "Prefectura Toyama"), ("ru", "Тояма"), ("si", "ටෝය\u{dcf}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tojama"), ("sl", "prefektura Toyama"), ("sr", "Тојама"), ("sr_Latn", "Tojama"), ("sv", "Toyama prefektur"), ("sw", "Mkoa wa Toyama"), ("ta", "டோயம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ట\u{c4b}య\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโทะยะมะ"), ("tr", "Toyama"), ("uk", "Префектура Тояма"), ("ur", "تویاما پریفیکچر"), ("vi", "Toyama"), ("yue", "富山縣"), ("yue_Hans", "富山县"), ("zh", "富山縣")]),
                        unofficial_name_list: ["Toyama"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::JP,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.5946816), longitude: Some(136.6255726), max_latitude: Some(37.8553273), min_latitude: Some(36.0672759), max_longitude: Some(137.3608272), min_longitude: Some(136.2429979)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ishikawa Prefektuur"), ("ar", "إيشيكاوا"), ("az", "İşikava"), ("be", "прэфектура Ісікава"), ("bg", "Ишикава"), ("bn", "ইশিক\u{9be}ওয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Ishikawa"), ("ccp", "𑄃\u{11128}𑄥\u{11128}𑄇𑄤"), ("ceb", "Ishikawa-ken"), ("cs", "Prefektura Išikawa"), ("cy", "Ishikawa"), ("da", "Ishikawa-præfekturet"), ("de", "Präfektur Ishikawa"), ("el", "Ισικάβα"), ("en", "Ishikawa"), ("es", "Prefectura de Ishikawa"), ("et", "Ishikawa prefektuur"), ("eu", "Ishikawa"), ("fa", "استان ایشیکاوا"), ("fi", "Ishikawan prefektuuri"), ("fr", "préfecture d’Ishikawa"), ("ga", "Maoracht Ishikawa"), ("gl", "Prefectura de Ishikawa"), ("gu", "ઇશિકાવા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אישיקווה"), ("hi", "इशिकावा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Ishikawa"), ("hu", "Isikava prefektúra"), ("id", "Prefektur Ishikawa"), ("it", "prefettura di Ishikawa"), ("ja", "石川県"), ("ka", "ისიკავის პრეფექტურა"), ("kn", "ಇಶ\u{cbf}ಕಾವಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "이시카와 현"), ("lt", "Išikavos prefektūra"), ("lv", "Isikavas prefektūra"), ("mk", "Ишикава"), ("mr", "इशिकावा"), ("ms", "Wilayah Ishikawa"), ("nb", "Ishikawa"), ("nl", "Ishikawa"), ("no", "Ishikawa"), ("pl", "Prefektura Ishikawa"), ("pt", "Ishikawa"), ("ro", "Prefectura Ishikawa"), ("ru", "Исикава"), ("si", "ඉෂ\u{dd2}ක\u{dcf}ව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Išikawa"), ("sl", "prefektura Išikava"), ("sr", "Ишикава"), ("sr_Latn", "Išikava"), ("sv", "Ishikawa prefektur"), ("sw", "Mkoa wa Ishikawa"), ("ta", "இஷிக\u{bbe}வ\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ఇష\u{c3f}క\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e34}ช\u{e34}กะวะ"), ("tr", "Ishikawa"), ("uk", "Префектура Ісікава"), ("ur", "اشیکاوا پریفیکچر"), ("vi", "Ishikawa"), ("yue", "石川縣"), ("yue_Hans", "石川县"), ("zh", "石川縣")]),
                        unofficial_name_list: ["Ishikawa", "Isikawa"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::JP,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.0640669), longitude: Some(136.2194938), max_latitude: Some(36.1729691), min_latitude: Some(35.9205083), max_longitude: Some(136.4702452), min_longitude: Some(135.9650378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Fukui Prefektuur"), ("ar", "فوكوي"), ("az", "Fukui"), ("bg", "Фукуи"), ("bn", "ফ\u{9c1}ক\u{9c1}ই প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Fukui"), ("ccp", "𑄜\u{1112a}𑄇\u{1112d}"), ("ceb", "Fukui-ken"), ("cs", "Prefektura Fukui"), ("cy", "Fukui"), ("da", "Fukui-præfekturet"), ("de", "Präfektur Fukui"), ("el", "Φουκούι"), ("en", "Fukui"), ("es", "Prefectura de Fukui"), ("et", "Fukui prefektuur"), ("eu", "Fukui"), ("fa", "استان فوکوئی"), ("fi", "Fukuin prefektuuri"), ("fr", "préfecture de Fukui"), ("ga", "Maoracht Fukui"), ("gl", "Prefectura de Fukui"), ("gu", "ફ\u{ac1}ક\u{ac1}ઇ પ\u{acd}રીફ\u{ac7}કચર"), ("he", "פוקואי"), ("hi", "फ\u{941}क\u{941}ई प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Fukui"), ("hu", "Fukui prefektúra"), ("id", "Prefektur Fukui"), ("it", "prefettura di Fukui"), ("ja", "福井県"), ("jv", "Prefektur Fukui"), ("kn", "ಫುಕುಯ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "후쿠이 현"), ("lt", "Fukujaus prefektūra"), ("lv", "Fukuji prefektūra"), ("mk", "Фукуи"), ("mr", "फ\u{941}क\u{941}ई"), ("ms", "Wilayah Fukui"), ("nb", "Fukui"), ("nl", "Fukui"), ("no", "Fukui"), ("pl", "Prefektura Fukui"), ("pt", "Fukui"), ("ro", "Prefectura Fukui"), ("ru", "Фукуи"), ("si", "ෆ\u{dd4}ක\u{dd4}ය\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Fukui"), ("sr", "Фукуј"), ("sr_Latn", "Fukuj"), ("sv", "Fukui prefektur"), ("sw", "Mkoa wa Fukui"), ("ta", "பிக\u{bcd}குய\u{bcd} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఫుకూయ\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟ\u{e38}ก\u{e38}อ\u{e34}"), ("tr", "Fukui"), ("uk", "Префектура Фукуй"), ("ur", "فوکوئی پریفیکچر"), ("vi", "Fukui"), ("yue", "福井縣"), ("yue_Hans", "福井县"), ("zh", "福井縣")]),
                        unofficial_name_list: ["Fukui", "Hukui"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::JP,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.6641575), longitude: Some(138.5684486), max_latitude: Some(35.971911), min_latitude: Some(35.1683022), max_longitude: Some(139.13455), min_longitude: Some(138.1801079)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yamanashi Prefektuur"), ("ar", "ياماناشي"), ("az", "Yamanaşi"), ("be", "прэфектура Яманасі"), ("bg", "Яманаши"), ("bn", "য\u{9bc}\u{9be}ম\u{9be}ন\u{9be}শি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Yamanashi"), ("ccp", "𑄃\u{11128}𑄠𑄟𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Yamanashi-ken"), ("cs", "Prefektura Jamanaši"), ("cy", "Yamanashi"), ("da", "Yamanashi-præfekturet"), ("de", "Präfektur Yamanashi"), ("el", "Γιαμανάσι"), ("en", "Yamanashi"), ("es", "Prefectura de Yamanashi"), ("et", "Yamanashi prefektuur"), ("eu", "Yamanashi"), ("fa", "استان یاماناشی"), ("fi", "Yamanashin prefektuuri"), ("fr", "préfecture de Yamanashi"), ("ga", "Maoracht Yamanashi"), ("gl", "Prefectura de Yamanashi"), ("gu", "યમાનાશી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "יאמאנאשי"), ("hi", "यामानाशी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Yamanashi"), ("hu", "Jamanasi prefektúra"), ("id", "Prefektur Yamanashi"), ("it", "prefettura di Yamanashi"), ("ja", "山梨県"), ("ka", "იამანასის პრეფექტურა"), ("km", "ខេត\u{17d2}តយ\u{17c9}ាម\u{17c9}ាណាស\u{17ca}\u{17b8}"), ("kn", "ಯಮನಶ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "야마나시 현"), ("lt", "Jamanašio prefektūra"), ("lv", "Jamanasi prefektūra"), ("mk", "Јаманаши"), ("mr", "यामानाशी"), ("ms", "Wilayah Yamanashi"), ("nb", "Yamanashi"), ("nl", "Yamanashi"), ("no", "Yamanashi"), ("pl", "Prefektura Yamanashi"), ("pt", "Yamanashi"), ("ro", "Prefectura Yamanashi"), ("ru", "Яманаси"), ("si", "යමන\u{dcf}ශ\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jamanaši"), ("sl", "prefektura Yamanaši"), ("sr", "Јаманаши"), ("sr_Latn", "Jamanaši"), ("sv", "Yamanashi prefektur"), ("sw", "Mkoa wa Yamanashi"), ("ta", "யம\u{bbe}னஷி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "యమన\u{c3e}ష\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดยะมะนะช\u{e34}"), ("tr", "Yamanashi"), ("uk", "Префектура Яманаші"), ("ur", "یماناشی پرفکترے"), ("vi", "Yamanashi"), ("yue", "山梨縣"), ("yue_Hans", "山梨县"), ("zh", "山梨县")]),
                        unofficial_name_list: ["Yamanashi", "Yamanasi"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::JP,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.6485496), longitude: Some(138.1942432), max_latitude: Some(36.835842), min_latitude: Some(36.4604537), max_longitude: Some(138.3190719), min_longitude: Some(137.9100075)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nagano Prefektuur"), ("ar", "ناغانو"), ("az", "Naqano"), ("be", "прэфектура Нагана"), ("bg", "Нагано"), ("bn", "ন\u{9be}গ\u{9be}নো প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Nagano"), ("ccp", "𑄚𑄉𑄚\u{1112e}"), ("ceb", "Nagano-ken"), ("cs", "Prefektura Nagano"), ("cy", "Nagano"), ("da", "Nagano-præfekturet"), ("de", "Präfektur Nagano"), ("el", "Νομός Ναγκάνο"), ("en", "Nagano"), ("es", "Prefectura de Nagano"), ("et", "Nagano prefektuur"), ("eu", "Nagano"), ("fa", "استان ناگانو"), ("fi", "Naganon prefektuuri"), ("fr", "préfecture de Nagano"), ("ga", "Maoracht Nagano"), ("gl", "Prefectura de Nagano"), ("gu", "નાગાનો પ\u{acd}રીફ\u{ac7}કચર"), ("he", "נאגנו"), ("hi", "नागानो प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Nagano"), ("hu", "Nagano prefektúra"), ("id", "Prefektur Nagano"), ("it", "prefettura di Nagano"), ("ja", "長野県"), ("ka", "ნაგანო"), ("km", "ខេត\u{17d2}តណាហ\u{17d2}គាណ\u{17bb}"), ("kn", "ನ\u{ccd}ಯಾಗೊನೋ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "나가노 현"), ("lt", "Nagano prefektūra"), ("lv", "Nagano prefektūra"), ("mk", "Нагано"), ("mr", "नागानो"), ("ms", "Wilayah Nagano"), ("nb", "Nagano"), ("nl", "Nagano"), ("no", "Nagano"), ("pl", "Prefektura Nagano"), ("pt", "Nagano"), ("ro", "Prefectura Nagano"), ("ru", "Нагано"), ("si", "නගනෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Nagano"), ("sl", "Prefektura Nagano"), ("sr", "Нагано"), ("sr_Latn", "Nagano"), ("sv", "Nagano prefektur"), ("sw", "Mkoa wa Nagano"), ("ta", "மகனோ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "న\u{c3e}గ\u{c3e}న\u{c4b} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนะงะโนะ"), ("tr", "Nagano"), ("uk", "Префектура Наґано"), ("ur", "ناگانو پریفیکچر"), ("vi", "Nagano"), ("yue", "長野縣"), ("yue_Hans", "长野县"), ("zh", "长野县")]),
                        unofficial_name_list: ["Nagano"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::JP,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.4232984), longitude: Some(136.7606537), max_latitude: Some(35.543131), min_latitude: Some(35.3513892), max_longitude: Some(136.8861855), min_longitude: Some(136.6783857)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gifu Prefektuur"), ("ar", "غيفو"), ("az", "Qifu"), ("be", "Прэфектура Гіфу"), ("bg", "Гифу"), ("bn", "গিফ\u{9c1} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Gifu"), ("ccp", "𑄎\u{11128}𑄜\u{1112a}"), ("ceb", "Gifu-ken"), ("cs", "Prefektura Gifu"), ("cy", "Gifu"), ("da", "Gifu-præfekturet"), ("de", "Präfektur Gifu"), ("el", "Γκίφου"), ("en", "Gifu"), ("es", "Prefectura de Gifu"), ("et", "Gifu prefektuur"), ("eu", "Gifu"), ("fa", "استان گیفو"), ("fi", "Gifun prefektuuri"), ("fr", "préfecture de Gifu"), ("ga", "Maoracht Gifu"), ("gl", "Prefectura de Gifu"), ("gu", "ગીફ\u{ac1} પ\u{acd}રીફ\u{ac7}કચર"), ("he", "גיפו"), ("hi", "जिफ\u{942} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Gifu"), ("hu", "Gifu prefektúra"), ("id", "Prefektur Gifu"), ("it", "prefettura di Gifu"), ("ja", "岐阜県"), ("jv", "Prefektur Gifu"), ("kn", "ಜ\u{cbf}ಫು ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "기후 현"), ("lt", "Gifu prefektūra"), ("lv", "Gifu prefektūra"), ("mk", "Гифу"), ("mr", "गिफ\u{942}"), ("ms", "Wilayah Gifu"), ("nb", "Gifu"), ("nl", "Gifu"), ("no", "Gifu"), ("pl", "Prefektura Gifu"), ("pt", "Gifu"), ("ro", "Prefectura Gifu"), ("ru", "Гифу"), ("si", "ග\u{dd3}ෆ\u{dd4} පළ\u{dcf}ත"), ("sk", "Gifu"), ("sr", "Гифу"), ("sr_Latn", "Gifu"), ("sv", "Gifu prefektur"), ("sw", "Mkoa wa Gifu"), ("ta", "கிபியூ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "గ\u{c3f}ఫు ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e34}ฟ\u{e38}"), ("tr", "Gifu"), ("uk", "Префектура Ґіфу"), ("ur", "گیفو پریفیکچر"), ("vi", "Gifu"), ("yue", "岐阜縣"), ("yue_Hans", "岐阜县"), ("zh", "岐阜县")]),
                        unofficial_name_list: ["Gifu", "Gihu"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::JP,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.975562), longitude: Some(138.3827596), max_latitude: Some(35.6459882), min_latitude: Some(34.8980251), max_longitude: Some(138.635826), min_longitude: Some(138.0829207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shizuoka Prefektuur"), ("ar", "شيزوكا"), ("az", "Sidzuoka"), ("be", "прэфектура Сідзуока"), ("bg", "Шидзуока"), ("bn", "শিয\u{9c1}ওক\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Shizuoka"), ("ccp", "𑄥\u{11128}𑄎\u{1112a}𑄤𑄇"), ("ceb", "Shizuoka-ken"), ("cs", "Prefektura Šizuoka"), ("cy", "Shizuoka"), ("da", "Shizuoka-præfekturet"), ("de", "Präfektur Shizuoka"), ("el", "Σιζουόκα"), ("en", "Shizuoka"), ("es", "Prefectura de Shizuoka"), ("et", "Shizuoka prefektuur"), ("eu", "Shizuoka"), ("fa", "استان شیزوئوکا"), ("fi", "Shizuokan prefektuuri"), ("fr", "préfecture de Shizuoka"), ("ga", "Maoracht Shizuoka"), ("gl", "Prefectura de Shizuoka"), ("gu", "શિઝ\u{ac1}ઓકા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "שיזואוקה"), ("hi", "शिज\u{93c}\u{941}का प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Shizuoka"), ("hu", "Sizuoka prefektúra"), ("id", "Prefektur Shizuoka"), ("is", "Shizuoka-umdæmi"), ("it", "prefettura di Shizuoka"), ("ja", "静岡県"), ("jv", "Prefektur Shizuoka"), ("kn", "ಶ\u{cbf}ಝುವೊಕಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "시즈오카 현"), ("lt", "Šidzuokos prefektūra"), ("lv", "Sidzuokas prefektūra"), ("mk", "Шизуока"), ("mr", "शिझ\u{941}ओका"), ("ms", "Wilayah Shizuoka"), ("nb", "Shizuoka"), ("nl", "Shizuoka"), ("no", "Shizuoka"), ("pl", "Prefektura Shizuoka"), ("pt", "Shizuoka"), ("ro", "Prefectura Shizuoka"), ("ru", "Сидзуока"), ("si", "ශ\u{dd2}සොක\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Šizuoka"), ("sl", "Prefektura Šizuoka"), ("sr", "Шизуока"), ("sr_Latn", "Šizuoka"), ("sv", "Shizuoka prefektur"), ("sw", "Mkoa wa Shizuoka"), ("ta", "ஷிஸுஒக\u{bcd}க\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ష\u{c3f}జువ\u{c4b}క\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}ซ\u{e38}โอะกะ"), ("tr", "Shizuoka"), ("uk", "Префектура Шідзуока"), ("ur", "شیزوکا پریفیکچر"), ("vi", "Shizuoka"), ("yue", "靜岡縣"), ("yue_Hans", "静冈县"), ("zh", "靜岡縣")]),
                        unofficial_name_list: ["Shizuoka", "Sizuoka"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::JP,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1801883), longitude: Some(136.9065647), max_latitude: Some(35.424822), min_latitude: Some(34.5780876), max_longitude: Some(137.8378675), min_longitude: Some(136.6710227)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aichi Prefektuur"), ("ar", "آيتشي"), ("az", "Ayti"), ("be", "прэфектура Айты"), ("bg", "Айчи"), ("bn", "আইচি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Aichi"), ("ccp", "𑄃\u{1112d}𑄥\u{11128}"), ("ceb", "Aichi-ken"), ("cs", "Prefektura Aiči"), ("cy", "Aichi"), ("da", "Aichi-præfekturet"), ("de", "Präfektur Aichi"), ("el", "Άιτσι"), ("en", "Aichi"), ("es", "Prefectura de Aichi"), ("et", "Aichi prefektuur"), ("eu", "Aichi"), ("fa", "استان آیچی"), ("fi", "Aichin prefektuuri"), ("fr", "préfecture d’Aichi"), ("ga", "Maoracht Aichi"), ("gl", "Prefectura de Aichi"), ("gu", "એચી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אאיצ׳י"), ("hi", "आइची प\u{94d}रीफ\u{93c}\u{947}क\u{94d}चर"), ("hr", "Aichi"), ("hu", "Aicsi prefektúra"), ("hy", "Այչի"), ("id", "Prefektur Aichi"), ("it", "prefettura di Aichi"), ("ja", "愛知県"), ("jv", "Prefektur Aichi"), ("ka", "აიტის პრეფექტურა"), ("km", "ខេត\u{17d2}តអៃឈ\u{17b7}"), ("kn", "ಐಚ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "아이치 현"), ("lt", "Aičio prefektūra"), ("lv", "Aiči"), ("mk", "Ајчи"), ("mn", "Айчи"), ("mr", "ऐची"), ("ms", "Wilayah Aichi"), ("nb", "Aichi"), ("nl", "Aichi"), ("no", "Aichi"), ("pa", "ਆਇਚੀ ਪ\u{a4d}ਰੀਫ\u{a3c}\u{a48}ਕਚਰ"), ("pl", "Prefektura Aichi"), ("pt", "Aichi"), ("ro", "Prefectura Aichi"), ("ru", "Айти"), ("si", "අය\u{dd2}ච\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Aiči"), ("sl", "prefektura Aiči"), ("sr", "Аичи"), ("sr_Latn", "Aiči"), ("sv", "Aichi prefektur"), ("sw", "Mkoa wa Aichi"), ("ta", "ஆட\u{bcd}சி பிரேபிக\u{bcd}ச\u{bcd}சர\u{bcd}"), ("te", "అయ\u{c3f}చ\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไอช\u{e34}"), ("tr", "Aichi"), ("uk", "Префектура Айчі"), ("ur", "ایچی پریفیکچر"), ("vi", "Aichi"), ("yue", "愛知縣"), ("yue_Hans", "爱知县"), ("zh", "愛知縣")]),
                        unofficial_name_list: ["Aichi", "Aiti"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::JP,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.7302829), longitude: Some(136.5085883), max_latitude: Some(35.2576841), min_latitude: Some(33.7226406), max_longitude: Some(136.9877503), min_longitude: Some(135.8528648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mie Prefektuur"), ("ar", "ميه"), ("az", "Mie"), ("be", "Прэфектура Міэ"), ("bg", "Мие"), ("bn", "মিয\u{9bc}ে প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Mie"), ("ccp", "𑄟\u{1112d}𑄠𑄬"), ("ceb", "Mie-ken"), ("cs", "Prefektura Mie"), ("cy", "Mie"), ("da", "Mie-præfekturet"), ("de", "Präfektur Mie"), ("el", "Μίε"), ("en", "Mie"), ("es", "Prefectura de Mie"), ("et", "Mie prefektuur"), ("eu", "Mie"), ("fa", "استان میه"), ("fi", "Mien prefektuuri"), ("fr", "préfecture de Mie"), ("ga", "Maoracht Mie"), ("gl", "Prefectura de Mie"), ("he", "מיאה"), ("hi", "मी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Mie"), ("hu", "Mie prefektúra"), ("hy", "Միե"), ("id", "Prefektur Mie"), ("it", "prefettura di Mie"), ("ja", "三重県"), ("km", "ខេត\u{17d2}តម\u{17b8}អ\u{17b7}"), ("kn", "ಮೀ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "미에 현"), ("lt", "Mijės prefektūra"), ("lv", "Mie prefektūra"), ("mk", "Мие"), ("mn", "Миэ"), ("mr", "मिई"), ("ms", "Wilayah Mie"), ("nb", "Mie"), ("nl", "Mie"), ("no", "Mie"), ("pl", "Prefektura Mie"), ("pt", "Mie"), ("ro", "Prefectura Mie"), ("ru", "Миэ"), ("si", "මය\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mie"), ("sl", "prefektura Mie"), ("sr", "Мије"), ("sr_Latn", "Mije"), ("sv", "Mie prefektur"), ("sw", "Mkoa wa Mie"), ("ta", "ம\u{bc0} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "మ\u{c3f}య\u{c46} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}เอะ"), ("tr", "Mie ili"), ("uk", "Префектура Міє"), ("ur", "میہ پریفیکچر"), ("vi", "Mie"), ("yue", "三重縣"), ("yue_Hans", "三重县"), ("zh", "三重县")]),
                        unofficial_name_list: ["Mie"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::JP,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.0045306), longitude: Some(135.8685899), max_latitude: Some(35.7035792), min_latitude: Some(34.7906231), max_longitude: Some(136.4549543), min_longitude: Some(135.7637715)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shiga Prefektuur"), ("ar", "شيغا"), ("az", "Şiqa"), ("be", "прэфектура Сіга"), ("bg", "Шига"), ("bn", "শিগ\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Shiga"), ("ccp", "𑄥\u{11128}𑄉"), ("ceb", "Shiga-ken"), ("cs", "Prefektura Šiga"), ("cy", "Shiga"), ("da", "Shiga-præfekturet"), ("de", "Präfektur Shiga"), ("el", "Σίγκα"), ("en", "Shiga"), ("es", "Prefectura de Shiga"), ("et", "Shiga prefektuur"), ("eu", "Shiga"), ("fa", "استان شیگا"), ("fi", "Shigan prefektuuri"), ("fr", "préfecture de Shiga"), ("ga", "Maoracht Shiga"), ("gl", "Prefectura de Shiga"), ("gu", "શિગા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "שיגה"), ("hi", "शिगा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Shiga"), ("hu", "Siga prefektúra"), ("id", "Prefektur Shiga"), ("it", "prefettura di Shiga"), ("ja", "滋賀県"), ("jv", "Prefektur Shiga"), ("kn", "ಶ\u{cbf}ಗಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "시가 현"), ("lt", "Šigos prefektūra"), ("lv", "Sigas prefektūra"), ("mk", "Шига"), ("mn", "Шига"), ("mr", "शिगा"), ("ms", "Wilayah Shiga"), ("nb", "Shiga"), ("nl", "Shiga"), ("no", "Shiga"), ("pl", "Prefektura Shiga"), ("pt", "Shiga"), ("ro", "Prefectura Shiga"), ("ru", "Сига"), ("si", "ශ\u{dd2}ග\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Šiga"), ("sl", "prefektura Šiga"), ("sr", "Шига"), ("sr_Latn", "Šiga"), ("sv", "Shiga prefektur"), ("sw", "Mkoa wa Shiga"), ("ta", "ஷிக\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ష\u{c3f}గ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}งะ"), ("tr", "Shiga"), ("uk", "Префектура Сіґа"), ("ur", "شیگا پریفیکچر"), ("vi", "Shiga"), ("yue", "滋賀縣"), ("yue_Hans", "滋贺县"), ("zh", "滋贺县")]),
                        unofficial_name_list: ["Shiga", "Siga"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::JP,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.0116363), longitude: Some(135.7680294), max_latitude: Some(35.321192), min_latitude: Some(34.8748597), max_longitude: Some(135.8787786), min_longitude: Some(135.5589842)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kioto Prefektuur"), ("ar", "كيوتو"), ("az", "Kyoto"), ("be", "прэфектура Кіёта"), ("bg", "Киото"), ("bn", "কিয\u{9bc}োতো প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kyoto"), ("ccp", "𑄇\u{1112d}𑄠\u{11128}𑄑\u{1112e}"), ("ceb", "Kyōto-fu"), ("cs", "Prefektura Kjóto"), ("cy", "Kyoto"), ("da", "Kyoto-præfekturet"), ("de", "Präfektur Kyōto"), ("el", "Νομός Κιότο"), ("en", "Kyōto"), ("es", "Prefectura de Kioto"), ("et", "Kyōto prefektuur"), ("eu", "Kyoto"), ("fa", "استان کیوتو"), ("fi", "Kioton prefektuuri"), ("fr", "préfecture de Kyoto"), ("ga", "Maoracht Kyoto"), ("gl", "Prefectura de Kyōto"), ("gu", "ક\u{acd}યોટો પ\u{acd}રિફ\u{ac7}ક\u{acd}ચર"), ("he", "קיוטו"), ("hi", "क\u{94d}योटो प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Kyōto"), ("hu", "Kiotó prefektúra"), ("id", "Prefektur Kyoto"), ("it", "prefettura di Kyoto"), ("ja", "京都府"), ("km", "ខេត\u{17d2}តក\u{17d2}យ\u{17bc}ត\u{17bb}"), ("kn", "ಕ\u{ccd}ಯೋಟೋ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "교토 부"), ("lt", "Kioto prefektūra"), ("lv", "Kioto prefektūra"), ("mk", "Кјото"), ("mr", "क\u{94d}योतो"), ("ms", "Wilayah Kyoto"), ("nb", "Kyoto"), ("nl", "Kioto"), ("no", "Kyoto"), ("pl", "Prefektura Kioto"), ("pt", "Quioto"), ("ro", "Prefectura Kyoto"), ("ru", "Киото"), ("si", "ක\u{dca}යෝටෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kjóto"), ("sl", "prefektura Kyōto"), ("sr", "Кјото"), ("sr_Latn", "Kjoto"), ("sv", "Kyoto prefektur"), ("sw", "Mkoa wa Kyoto"), ("ta", "கியோத\u{bcd}தோ நகரிய ம\u{bbe}நிலம\u{bcd}"), ("te", "క\u{c4d}య\u{c4b}ట\u{c4b} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเค\u{e35}ยวโตะ"), ("tr", "Kyoto"), ("uk", "Префектура Кіото"), ("ur", "کیوٹو پریفیکچر"), ("vi", "Kyōto"), ("yue", "京都府"), ("yue_Hans", "京都府"), ("zh", "京都府")]),
                        unofficial_name_list: ["Kyoto"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::JP,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6937378), longitude: Some(135.5021651), max_latitude: Some(34.7687542), min_latitude: Some(34.5861473), max_longitude: Some(135.599171), min_longitude: Some(135.3728875)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Osaka Prefektuur"), ("ar", "أوساكا"), ("az", "Osaka"), ("be", "прэфектура Осака"), ("bg", "Осака"), ("bn", "ওস\u{9be}ক\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Osaka"), ("ccp", "𑄃\u{1112e}𑄥𑄇"), ("ceb", "Ōsaka-fu"), ("cs", "Prefektura Ósaka"), ("cy", "Osaka"), ("da", "Osaka-præfekturet"), ("de", "Präfektur Osaka"), ("el", "Οσάκα"), ("en", "Ōsaka"), ("es", "Prefectura de Osaka"), ("et", "Ōsaka prefektuur"), ("eu", "Osakako prefetura"), ("fa", "استان اوساکا"), ("fi", "Osakan prefektuuri"), ("fr", "préfecture d’Osaka"), ("ga", "Maoracht Osaka"), ("gl", "Prefectura de Ōsaka"), ("gu", "ઓસાકા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אוסקה"), ("hi", "ओसाका प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Ōsaka"), ("hu", "Oszaka prefektúra"), ("hy", "Օսակայի պրեֆեկտուրա"), ("id", "Prefektur Osaka"), ("it", "prefettura di Osaka"), ("ja", "大阪府"), ("jv", "Prefektur Osaka"), ("km", "អាណាខេត\u{17d2}តអ\u{17bc}សាកា"), ("kn", "ಒಸಾಕಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "오사카 부"), ("lt", "Osakos prefektūra"), ("lv", "Osakas prefektūra"), ("mk", "Осака"), ("mr", "ओसाका"), ("ms", "Wilayah Osaka"), ("nb", "Osaka"), ("nl", "Osaka"), ("no", "Osaka"), ("pl", "Prefektura Osaka"), ("pt", "Osaka"), ("ro", "Prefectura Osaka"), ("ru", "Осака"), ("si", "ඔස\u{dcf}ක\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Osaka"), ("sl", "Prefektura Osaka"), ("sr", "Осака"), ("sr_Latn", "Osaka"), ("sv", "Osaka prefektur"), ("sw", "Mkoa wa Osaka"), ("ta", "ஒச\u{bbe}க\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఓస\u{c3e}క\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอซะกะ"), ("tr", "Osaka ili"), ("uk", "Префектура Осака"), ("ur", "اوساکا پریفیکچر"), ("vi", "Ōsaka"), ("yue", "大阪府"), ("yue_Hans", "大坂府"), ("zh", "大阪府")]),
                        unofficial_name_list: ["Osaka"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::JP,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6912688), longitude: Some(135.1830706), max_latitude: Some(35.674731), min_latitude: Some(34.1556703), max_longitude: Some(135.4685529), min_longitude: Some(134.2527119)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hyogo Prefektuur"), ("ar", "هيوغو"), ("az", "Hyöqo"), ("be", "Прэфектура Хіёга"), ("bg", "Хього"), ("bn", "হিয\u{9bc}োগো প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Hyōgo"), ("ccp", "𑄦\u{1112d}𑄃\u{1112e}𑄉\u{1112e}"), ("ceb", "Hyōgo-ken"), ("cs", "Prefektura Hjógo"), ("cy", "Hyōgo"), ("da", "Hyougo-præfekturet"), ("de", "Hyōgo"), ("el", "Χιγκόρο"), ("en", "Hyōgo"), ("es", "Prefectura de Hyōgo"), ("et", "Hyōgo prefektuur"), ("eu", "Hyōgo"), ("fa", "استان هیوگو"), ("fi", "Hyōgon prefektuuri"), ("fr", "préfecture de Hyōgo"), ("ga", "Maoracht Hyōgo"), ("gl", "Prefectura de Hyōgo"), ("gu", "હ\u{acd}યોગો પ\u{acd}રીફ\u{ac7}કચર"), ("he", "היוגו"), ("hi", "ह\u{94d}योगो प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Hyōgo"), ("hu", "Hjógo prefektúra"), ("id", "Prefektur Hyogo"), ("is", "Hyogo-hérað"), ("it", "prefettura di Hyōgo"), ("ja", "兵庫県"), ("jv", "Prefektur Hyogo"), ("kn", "ಹೈಗೊ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "효고 현"), ("lt", "Hiogo prefektūra"), ("lv", "Hjogo prefektūra"), ("mk", "Хјого"), ("mr", "ह\u{94d}योगो"), ("ms", "Wilayah Hyōgo"), ("nb", "Hyōgo"), ("nl", "Hyogo"), ("no", "Hyōgo"), ("pl", "Prefektura Hyōgo"), ("pt", "Hyōgo"), ("ro", "Prefectura Hyōgo"), ("ru", "Хёго"), ("si", "හයෝගෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Hjógo"), ("sl", "prefektura Hyōgo"), ("sr", "Хјого"), ("sr_Latn", "Hjogo"), ("sv", "Hyogo prefektur"), ("sw", "Mkoa wa Hyogo"), ("ta", "ஹயோகோ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "హయ\u{c4b}గ\u{c4b} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฮ\u{e35}ยวโงะ"), ("tr", "Hyōgo ili"), ("uk", "Префектура Хьоґо"), ("ur", "ہیوگو پریفیکچر"), ("vi", "Hyōgo"), ("yue", "兵庫縣"), ("yue_Hans", "兵库县"), ("zh", "兵库县")]),
                        unofficial_name_list: ["Hyogo"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::JP,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6850869), longitude: Some(135.8050002), max_latitude: Some(34.7577714), min_latitude: Some(34.558156), max_longitude: Some(136.0710847), min_longitude: Some(135.713102)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nara Prefektuur"), ("ar", "نارا"), ("az", "Nara"), ("be", "Нара"), ("bg", "Нара"), ("bn", "ন\u{9be}র\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Nara"), ("ccp", "𑄚𑄢"), ("ceb", "Nara-ken"), ("cs", "Prefektura Nara"), ("cy", "Nara"), ("da", "Nara-præfekturet"), ("de", "Präfektur Nara"), ("el", "Νάρα"), ("en", "Nara"), ("es", "Prefectura de Nara"), ("et", "Nara prefektuur"), ("eu", "Nara"), ("fa", "استان نارا"), ("fi", "Naran prefektuuri"), ("fr", "préfecture de Nara"), ("ga", "Maoracht Nara"), ("gl", "Prefectura de Nara"), ("gu", "નારા પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "נארה"), ("hi", "नारा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Nara"), ("hu", "Nara prefektúra"), ("id", "Prefektur Nara"), ("it", "prefettura di Nara"), ("ja", "奈良県"), ("jv", "Prefektur Nara"), ("ka", "ნარის პრეფექტურა"), ("km", "អាណាខេត\u{17d2}តណារ\u{17c9}ា"), ("kn", "ನಾರಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "나라 현"), ("lt", "Naros prefektūra"), ("lv", "Naras prefektūra"), ("mk", "Нара"), ("mr", "नारा"), ("ms", "Wilayah Nara"), ("nb", "Nara"), ("nl", "Nara"), ("no", "Nara"), ("pl", "Prefektura Nara"), ("pt", "Nara"), ("ro", "Prefectura Nara"), ("ru", "Нара"), ("si", "න\u{dcf}ර\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Nara"), ("sl", "prefektura Nara"), ("sr", "Нара"), ("sr_Latn", "Nara"), ("sv", "Nara prefektur"), ("sw", "Mkoa wa Nara"), ("ta", "ந\u{bbe}ர\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "న\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนะระ"), ("tr", "Nara"), ("uk", "Префектура Нара"), ("ur", "نارا پریفیکچر"), ("vi", "Nara"), ("yue", "奈良縣"), ("yue_Hans", "奈良县"), ("zh", "奈良县")]),
                        unofficial_name_list: ["Nara"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::JP,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.2305113), longitude: Some(135.1708083), max_latitude: Some(34.3157299), min_latitude: Some(34.1504333), max_longitude: Some(135.3148304), min_longitude: Some(134.9990825)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wakayama Prefektuur"), ("ar", "واكاياما"), ("az", "Vakayama"), ("be", "прэфектура Вакаяма"), ("bg", "Вакаяма"), ("bn", "ওয\u{9bc}\u{9be}ক\u{9be}য\u{9bc}\u{9be}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Wakayama"), ("ccp", "𑄤𑄇𑄬𑄠𑄟"), ("ceb", "Wakayama-ken"), ("cs", "Prefektura Wakajama"), ("cy", "Wakayama"), ("da", "Wakayama-præfekturet"), ("de", "Präfektur Wakayama"), ("el", "Γουακαγιάμα"), ("en", "Wakayama"), ("es", "Prefectura de Wakayama"), ("et", "Wakayama prefektuur"), ("eu", "Wakayama"), ("fa", "استان واکایاما"), ("fi", "Wakayaman prefektuuri"), ("fr", "préfecture de Wakayama"), ("ga", "Maoracht Wakayama"), ("gl", "Prefectura de Wakayama"), ("gu", "વાકાયામા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "וקאיאמה"), ("hi", "वकायामा प\u{94d}रान\u{94d}त"), ("hr", "Wakayama"), ("hu", "Vakajama prefektúra"), ("hy", "Վակայամա"), ("id", "Prefektur Wakayama"), ("it", "prefettura di Wakayama"), ("ja", "和歌山県"), ("jv", "Prefektur Wakayama"), ("kn", "ವಕಾಯಾಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "와카야마 현"), ("lt", "Vakajamos prefektūra"), ("lv", "Vakajamas prefektūra"), ("mk", "Вакајама"), ("mn", "Вакаяма"), ("mr", "वाकायामा"), ("ms", "Wilayah Wakayama"), ("nb", "Wakayama"), ("nl", "Wakayama"), ("no", "Wakayama"), ("pl", "Prefektura Wakayama"), ("pt", "Wakayama"), ("ro", "Prefectura Wakayama"), ("ru", "Вакаяма"), ("si", "වකය\u{dcf}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Wakajama"), ("sr", "Вакајама"), ("sr_Latn", "Vakajama"), ("sv", "Wakayama prefektur"), ("sw", "Mkoa wa Wakayama"), ("ta", "வ\u{bbe}க\u{bcd}கயம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "వక\u{c3e}య\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวะกะยะมะ"), ("tk", "Vakaýama"), ("tr", "Wakayama"), ("uk", "Префектура Вакаяма"), ("ur", "واکایاما پریفیکچر"), ("vi", "Wakayama"), ("yue", "和歌山縣"), ("yue_Hans", "和歌山县"), ("zh", "和歌山县")]),
                        unofficial_name_list: ["Wakayama"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::JP,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.5011326), longitude: Some(134.2350914), max_latitude: Some(35.5728697), min_latitude: Some(35.2716921), max_longitude: Some(134.4408042), min_longitude: Some(133.9459135)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tottori Prefektuur"), ("ar", "توتوري"), ("az", "Tottori"), ("be", "прэфектура Таторы"), ("bg", "Тотори"), ("bn", "তোত\u{9cd}তোরি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Tottori"), ("ccp", "𑄑\u{1112e}𑄖\u{11134}𑄖\u{1112e}𑄢\u{11128}"), ("ceb", "Tottori-ken"), ("cs", "Prefektura Tottori"), ("cy", "Tottori"), ("da", "Tottori-præfekturet"), ("de", "Präfektur Tottori"), ("el", "Τόττορι"), ("en", "Tottori"), ("es", "Prefectura de Tottori"), ("et", "Tottori prefektuur"), ("eu", "Tottori"), ("fa", "استان توتوری"), ("fi", "Tottorin prefektuuri"), ("fr", "préfecture de Tottori"), ("ga", "Maoracht Tottori"), ("gl", "Prefectura de Tottori"), ("gu", "ટોટ\u{acd}ટોરી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "טוטורי"), ("hi", "तोतोरी प\u{94d}रान\u{94d}त"), ("hr", "Tottori"), ("hu", "Tottori prefektúra"), ("id", "Prefektur Tottori"), ("it", "prefettura di Tottori"), ("ja", "鳥取県"), ("jv", "Prefektur Tottori"), ("kn", "ಟೊಟ\u{ccd}ಟೊರ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "돗토리 현"), ("lt", "Totorio prefektūra"), ("lv", "Totori prefektūra"), ("mk", "Тотори"), ("mr", "तोतोरी"), ("ms", "Wilayah Tottori"), ("nb", "Tottori"), ("nl", "Tottori"), ("no", "Tottori"), ("pl", "Prefektura Tottori"), ("pt", "Tottori"), ("ro", "Prefectura Tottori"), ("ru", "Тоттори"), ("si", "ටෝට\u{dca}ටෝර\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tottori"), ("sr", "Тотори"), ("sr_Latn", "Totori"), ("sv", "Tottori prefektur"), ("sw", "Mkoa wa Tottori"), ("ta", "டோட\u{bcd}டோரி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ట\u{c3e}ట\u{c4b}ర\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดทตโตะร\u{e34}"), ("tr", "Tottori"), ("uk", "Префектура Тотторі"), ("ur", "توتوری پریفیکچر"), ("vi", "Tottori"), ("yue", "鳥取縣"), ("yue_Hans", "鸟取县"), ("zh", "鳥取縣")]),
                        unofficial_name_list: ["Tottori"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::JP,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.4722952), longitude: Some(133.0504997), max_latitude: Some(36.3562202), min_latitude: Some(34.302454), max_longitude: Some(133.386651), min_longitude: Some(131.6679648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shimane Prefektuur"), ("ar", "شيمانه"), ("az", "Simane"), ("be", "прэфектура Сіманэ"), ("bg", "Шимане"), ("bn", "শিম\u{9be}নে প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Shimane"), ("ccp", "𑄥\u{11128}𑄟𑄚𑄬"), ("ceb", "Shimane-ken"), ("cs", "Prefektura Šimane"), ("cy", "Shimane"), ("da", "Shimane-præfekturet"), ("de", "Präfektur Shimane"), ("el", "Σιμάνε"), ("en", "Shimane"), ("es", "Prefectura de Shimane"), ("et", "Shimane prefektuur"), ("eu", "Shimane"), ("fa", "استان شیمانه"), ("fi", "Shimanen prefektuuri"), ("fr", "préfecture de Shimane"), ("ga", "Maoracht Shimane"), ("gl", "Prefectura de Shimane"), ("gu", "શિમ\u{ac5}ન પ\u{acd}રીફ\u{ac7}કચર"), ("he", "שימאנה"), ("hi", "शिमान\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Shimane"), ("hu", "Simane prefektúra"), ("id", "Prefektur Shimane"), ("it", "prefettura di Shimane"), ("ja", "島根県"), ("km", "ខេត\u{17d2}តស\u{17ca}\u{17b8}ម\u{17c9}ាណ\u{17b7}"), ("kn", "ಶ\u{cbf}ಮಾನ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "시마네 현"), ("lt", "Šimanės prefektūra"), ("lv", "Simanes prefektūra"), ("mk", "Шимане"), ("mr", "शिमान\u{947}"), ("ms", "Wilayah Shimane"), ("nb", "Shimane"), ("nl", "Shimane"), ("no", "Shimane"), ("pl", "Prefektura Shimane"), ("pt", "Shimane"), ("ro", "Prefectura Shimane"), ("ru", "Симане"), ("si", "ශ\u{dd2}ම\u{dcf}නේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Šimane"), ("sl", "prefektura Šimane"), ("sr", "Шимане"), ("sr_Latn", "Šimane"), ("sv", "Shimane prefektur"), ("sw", "Mkoa wa Shimane"), ("ta", "ஷிம\u{bcd}மனே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ష\u{c3f}మ\u{c47}న\u{c4d} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}มะเนะ"), ("tr", "Shimane"), ("uk", "Префектура Шімане"), ("ur", "شیمانے پریفیکچر"), ("vi", "Shimane"), ("yue", "島根縣"), ("yue_Hans", "岛根县"), ("zh", "岛根县")]),
                        unofficial_name_list: ["Shimane", "Simane"].to_vec(),
                    }
                ),
                (
                    "33",
                    Subdivision{
                        name: "33",
                        country_alpha2: Alpha2::JP,
                        code: "33",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6551456), longitude: Some(133.9195019), max_latitude: Some(34.9489127), min_latitude: Some(34.5183404), max_longitude: Some(134.1230012), min_longitude: Some(133.739373)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Okayama Prefektuur"), ("ar", "أوكاياما"), ("az", "Okayama"), ("be", "прэфектура Акаяма"), ("bg", "Окаяма"), ("bn", "ওক\u{9be}য\u{9bc}\u{9be}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Okayama"), ("ccp", "𑄃\u{1112e}𑄇𑄬𑄠𑄟"), ("ceb", "Okayama-ken"), ("cs", "Prefektura Okajama"), ("cy", "Okayama"), ("da", "Okayama-præfekturet"), ("de", "Präfektur Okayama"), ("el", "Οκαγιάμα"), ("en", "Okayama"), ("es", "Prefectura de Okayama"), ("et", "Okayama prefektuur"), ("eu", "Okayama"), ("fa", "استان اوکایاما"), ("fi", "Okayaman prefektuuri"), ("fr", "préfecture d’Okayama"), ("ga", "Maoracht Okayama"), ("gl", "Prefectura de Okayama"), ("gu", "ઓકાયામા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אוקיאמה"), ("hi", "ओकायामा प\u{94d}रान\u{94d}त"), ("hr", "Okayama"), ("hu", "Okajama prefektúra"), ("id", "Prefektur Okayama"), ("it", "prefettura di Okayama"), ("ja", "岡山県"), ("jv", "Prefektur Okayama"), ("km", "ខេត\u{17d2}តអ\u{17bc}កាយ\u{17c9}ាម\u{17c9}ា"), ("kn", "ಒಕಯಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "오카야마 현"), ("lt", "Okajamos prefektūra"), ("lv", "Okajamas prefektūra"), ("mk", "Окајама"), ("mr", "ओकायामा"), ("ms", "Wilayah Okayama"), ("nb", "Okayama"), ("nl", "Okayama"), ("no", "Okayama"), ("pl", "Prefektura Okayama"), ("pt", "Okayama"), ("ro", "Prefectura Okayama"), ("ru", "Окаяма"), ("si", "ඔකය\u{dcf}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Okajama"), ("sr", "Окајама"), ("sr_Latn", "Okajama"), ("sv", "Okayama prefektur"), ("sw", "Mkoa wa Okayama"), ("ta", "ஆக\u{bbe}யம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఓకయ\u{c3e}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอะกะยะมะ"), ("tr", "Okayama"), ("uk", "Префектура Окаяма"), ("ur", "اوکایاما پریفیکچر"), ("vi", "Okayama"), ("yue", "岡山縣"), ("yue_Hans", "冈山县"), ("zh", "岡山縣")]),
                        unofficial_name_list: ["Okayama"].to_vec(),
                    }
                ),
                (
                    "34",
                    Subdivision{
                        name: "34",
                        country_alpha2: Alpha2::JP,
                        code: "34",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3852029), longitude: Some(132.4552927), max_latitude: Some(34.6156543), min_latitude: Some(34.2966826), max_longitude: Some(132.6960795), min_longitude: Some(132.1788743)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hiroshima Prefektuur"), ("ar", "هيروشيما"), ("az", "Xirosima"), ("be", "Хірасіма"), ("bg", "Хирошима"), ("bn", "হিরোশিম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Hiroshima"), ("ccp", "𑄦\u{11128}𑄢\u{1112e}𑄥\u{11128}𑄟"), ("ceb", "Hiroshima-ken"), ("cs", "Prefektura Hirošima"), ("cy", "Hiroshima"), ("da", "Hiroshima-præfekturet"), ("de", "Präfektur Hiroshima"), ("el", "Χιροσίμα (νομός)"), ("en", "Hiroshima"), ("es", "Prefectura de Hiroshima"), ("et", "Hiroshima prefektuur"), ("eu", "Hiroshima"), ("fa", "استان هیروشیما"), ("fi", "Hiroshiman prefektuuri"), ("fr", "préfecture de Hiroshima"), ("ga", "Maoracht Hiroshima"), ("gl", "Prefectura de Hiroshima"), ("gu", "હિરોશિમા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "הירושימה"), ("hi", "हिरोशिमा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Hiroshima, prefektura"), ("hu", "Hirosima prefektúra"), ("id", "Prefektur Hiroshima"), ("it", "prefettura di Hiroshima"), ("ja", "広島県"), ("ka", "ჰიროსიმის პრეფექტურა"), ("km", "ខេត\u{17d2}តហ\u{17ca}\u{17b8}រ\u{17c9}\u{17bc}ស\u{17ca}\u{17b8}ម\u{17c9}ា"), ("kn", "ಹ\u{cbf}ರೋಷ\u{cbf}ಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "히로시마 현"), ("lt", "Hirosimos prefektūra"), ("lv", "Hirosimas prefektūra"), ("mk", "Хирошима"), ("mr", "हिरोशिमा"), ("ms", "Wilayah Hiroshima"), ("nb", "Hiroshima"), ("nl", "Hiroshima"), ("no", "Hiroshima"), ("pl", "Prefektura Hiroszima"), ("pt", "Província de Hiroshima"), ("ro", "Prefectura Hiroshima"), ("ru", "Хиросима"), ("si", "හ\u{dd2}රෝෂ\u{dd2}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Hirošima"), ("sl", "prefektura Hirošima"), ("sr", "Хирошима"), ("sr_Latn", "Hirošima"), ("sv", "Hiroshima prefektur"), ("sw", "Mkoa wa Hiroshima"), ("ta", "ஹிரோஷிம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "హ\u{c3f}ర\u{c4b}ష\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e34}โระช\u{e34}มะ"), ("tr", "Hiroşima"), ("uk", "Префектура Хіросіма"), ("ur", "ہیروشیما پریفیکچر"), ("vi", "Hiroshima"), ("yue", "廣島縣"), ("yue_Hans", "广岛县"), ("zh", "广岛县")]),
                        unofficial_name_list: ["Hiroshima", "Hirosima"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::JP,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.178496), longitude: Some(131.4737269), max_latitude: Some(34.5045323), min_latitude: Some(33.9677054), max_longitude: Some(131.7956151), min_longitude: Some(131.2919663)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yamaguchi Prefektuur"), ("ar", "ياماغوتشي"), ("az", "Yamaquti"), ("be", "Прэфектура Ямагуты"), ("bg", "Ямагучи"), ("bn", "য\u{9bc}\u{9be}ম\u{9be}গ\u{9c1}চি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Yamaguchi"), ("ccp", "𑄃\u{11128}𑄠𑄟𑄉\u{1112a}𑄥\u{11128}"), ("ceb", "Yamaguchi-ken"), ("cs", "Prefektura Jamaguči"), ("cy", "Yamaguchi"), ("da", "Yamaguchi-præfekturet"), ("de", "Präfektur Yamaguchi"), ("el", "Γιαμαγκούτσι"), ("en", "Yamaguchi"), ("es", "Prefectura de Yamaguchi"), ("et", "Yamaguchi prefektuur"), ("eu", "Yamaguchi"), ("fa", "استان یاماگوچی"), ("fi", "Yamaguchin prefektuuri"), ("fr", "préfecture de Yamaguchi"), ("ga", "Maoracht Yamaguchi"), ("gl", "Prefectura de Yamaguchi"), ("gu", "યામાગ\u{ac1}ચી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "יאמגוצ׳י"), ("hi", "यामाग\u{941}ची प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Yamaguchi"), ("hu", "Jamagucsi prefektúra"), ("hy", "Յամագուտի"), ("id", "Prefektur Yamaguchi"), ("it", "prefettura di Yamaguchi"), ("ja", "山口県"), ("ka", "იამაგუტის პრეფექტურა"), ("km", "ខេត\u{17d2}តយ\u{17c9}ាម\u{17c9}ាហ\u{17d2}គ\u{17b9}ឈ\u{17b7}"), ("kn", "ಯಮಗುಚ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "야마구치 현"), ("lt", "Jamagučio prefektūra"), ("lv", "Jamaguči prefektūra"), ("mk", "Јамагучи"), ("mr", "यामाग\u{941}ची"), ("ms", "Wilayah Yamaguchi"), ("nb", "Yamaguchi"), ("nl", "Yamaguchi"), ("no", "Yamaguchi"), ("pl", "Prefektura Yamaguchi"), ("pt", "Yamaguchi"), ("ro", "Prefectura Yamaguchi"), ("ru", "Ямагути"), ("si", "යමග\u{dd4}ච\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jamaguči"), ("sl", "prefektura Jamaguči"), ("sr", "Јамагучи"), ("sr_Latn", "Jamaguči"), ("sv", "Yamaguchi prefektur"), ("sw", "Mkoa wa Yamaguchi"), ("ta", "ய\u{bbe}ம\u{bbe}குசி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "యమగూచ\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดยะมะง\u{e38}ช\u{e34}"), ("tr", "Yamaguchi Prefecture"), ("uk", "Префектура Ямаґучі"), ("ur", "یاماگوچی پرفکترے"), ("vi", "Yamaguchi"), ("yue", "山口縣"), ("yue_Hans", "山口县"), ("zh", "山口县")]),
                        unofficial_name_list: ["Yamaguchi", "Yamaguti"].to_vec(),
                    }
                ),
                (
                    "36",
                    Subdivision{
                        name: "36",
                        country_alpha2: Alpha2::JP,
                        code: "36",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0702703), longitude: Some(134.5548438), max_latitude: Some(34.1303314), min_latitude: Some(33.9552976), max_longitude: Some(134.6069776), min_longitude: Some(134.4221647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tokushima Prefektuur"), ("ar", "توكوشيما"), ("az", "Tokusima"), ("be", "прэфектура Такусіма"), ("bg", "Токушима"), ("bn", "তোক\u{9c1}শিম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Tokushima"), ("ccp", "𑄑\u{1112e}𑄇\u{1112a}𑄥\u{11128}𑄟"), ("ceb", "Tokushima-ken"), ("cs", "Prefektura Tokušima"), ("cy", "Tokushima"), ("da", "Tokushima-præfekturet"), ("de", "Präfektur Tokushima"), ("el", "Τοκουσίμα"), ("en", "Tokushima"), ("es", "Prefectura de Tokushima"), ("et", "Tokushima prefektuur"), ("eu", "Tokushima"), ("fa", "استان توکوشیما"), ("fi", "Tokushiman prefektuuri"), ("fr", "préfecture de Tokushima"), ("ga", "Maoracht Tokushima"), ("gl", "Prefectura de Tokushima"), ("gu", "ટોક\u{ac1}શિમા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "טוקושימה"), ("hi", "तोक\u{941}शीमा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Tokushima"), ("hu", "Tokusima prefektúra"), ("id", "Prefektur Tokushima"), ("it", "prefettura di Tokushima"), ("ja", "徳島県"), ("jv", "Prefektur Tokushima"), ("kn", "ಟೋಕುಶ\u{cbf}ಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "도쿠시마 현"), ("lt", "Tokušimos prefektūra"), ("lv", "Tokusimas prefektūra"), ("mk", "Токушима"), ("mr", "तोक\u{941}शिमा"), ("ms", "Wilayah Tokushima"), ("nb", "Tokushima"), ("nl", "Tokushima"), ("no", "Tokushima"), ("pl", "Prefektura Tokushima"), ("pt", "Tokushima"), ("ro", "Prefectura Tokushima"), ("ru", "Токусима"), ("si", "ටෝක\u{dd4}ශ\u{dd2}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tokušima (prefektúra)"), ("sr", "Токушима"), ("sr_Latn", "Tokušima"), ("sv", "Tokushima prefektur"), ("sw", "Mkoa wa Tokushima"), ("ta", "டொகுஷிம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ట\u{c4a}కూష\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโทะก\u{e38}ช\u{e34}มะ"), ("tr", "Tokushima"), ("uk", "Префектура Токушіма"), ("ur", "توکوشیما پریفیکچر"), ("vi", "Tokushima"), ("yue", "德島縣"), ("yue_Hans", "德岛县"), ("zh", "德岛县")]),
                        unofficial_name_list: ["Tokushima", "Tokusima"].to_vec(),
                    }
                ),
                (
                    "37",
                    Subdivision{
                        name: "37",
                        country_alpha2: Alpha2::JP,
                        code: "37",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3401491), longitude: Some(134.0434436), max_latitude: Some(34.5648484), min_latitude: Some(34.0123825), max_longitude: Some(134.4407083), min_longitude: Some(133.4466123)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kagawa Prefektuur"), ("ar", "كاغاوا"), ("az", "Kaqava"), ("be", "прэфектура Кагава"), ("bg", "Кагава"), ("bn", "ক\u{9be}গ\u{9be}ওয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kagawa"), ("ccp", "𑄇\u{11127}𑄉𑄤"), ("ceb", "Kagawa-ken"), ("cs", "Prefektura Kagawa"), ("cy", "Kagawa"), ("da", "Kagawa-præfekturet"), ("de", "Präfektur Kagawa"), ("el", "Καγκάβα"), ("en", "Kagawa"), ("es", "Prefectura de Kagawa"), ("et", "Kagawa prefektuur"), ("eu", "Kagawa"), ("fa", "استان کاگاوا"), ("fi", "Kagawan prefektuuri"), ("fr", "préfecture de Kagawa"), ("ga", "Maoracht Kagawa"), ("gl", "Prefectura de Kagawa"), ("gu", "કાગાવા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "קאגווה"), ("hi", "कगावा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Kagawa"), ("hu", "Kagava prefektúra"), ("id", "Prefektur Kagawa"), ("it", "prefettura di Kagawa"), ("ja", "香川県"), ("ka", "კაგავის პრეფექტურა"), ("kn", "ಕಾಗಾವಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "가가와 현"), ("lt", "Kagavos prefektūra"), ("lv", "Kagavas prefektūra"), ("mk", "Кагава"), ("mr", "कागावा"), ("ms", "Wilayah Kagawa"), ("nb", "Kagawa"), ("nl", "Kagawa"), ("no", "Kagawa"), ("pl", "Prefektura Kagawa"), ("pt", "Kagawa"), ("ro", "Prefectura Kagawa"), ("ru", "Кагава"), ("si", "කගව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kagawa"), ("sr", "Кагава"), ("sr_Latn", "Kagava"), ("sv", "Kagawa prefektur"), ("sw", "Mkoa wa Kagawa"), ("ta", "ககவ\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "కగ\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคะงะวะ"), ("tr", "Kagawa"), ("uk", "Префектура Каґава"), ("ur", "کاگاوا پریفیکچر"), ("vi", "Kagawa"), ("yue", "香川縣"), ("yue_Hans", "香川县"), ("zh", "香川县")]),
                        unofficial_name_list: ["Kagawa"].to_vec(),
                    }
                ),
                (
                    "38",
                    Subdivision{
                        name: "38",
                        country_alpha2: Alpha2::JP,
                        code: "38",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.8416238), longitude: Some(132.7656808), max_latitude: Some(34.3017113), min_latitude: Some(32.8977566), max_longitude: Some(133.6928992), min_longitude: Some(132.0126449)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ehime Prefektuur"), ("ar", "إهيمه"), ("az", "Ehime"), ("be", "прэфектура Эхімэ"), ("bg", "Ехиме"), ("bn", "এহিমে প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Ehime"), ("ccp", "𑄃\u{11128}𑄦\u{1112d}𑄟\u{11134}"), ("ceb", "Ehime-ken"), ("cs", "Prefektura Ehime"), ("cy", "Ehime"), ("da", "Ehime-præfekturet"), ("de", "Präfektur Ehime"), ("el", "Εχιμέ"), ("en", "Ehime"), ("es", "Prefectura de Ehime"), ("et", "Ehime prefektuur"), ("eu", "Ehime"), ("fa", "استان اهیمه"), ("fi", "Ehimen prefektuuri"), ("fr", "préfecture d’Ehime"), ("ga", "Maoracht Ehime"), ("gl", "Prefectura de Ehime"), ("gu", "એહિમ\u{ac7} પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "אהימה"), ("hi", "एहिम प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Ehime"), ("hu", "Ehime prefektúra"), ("id", "Prefektur Ehime"), ("it", "prefettura di Ehime"), ("ja", "愛媛県"), ("jv", "Prefektur Ehime"), ("ka", "ეჰიმეს პრეფექტურა"), ("km", "ខេត\u{17d2}តអ\u{17b7}ហ\u{17ca}\u{17b8}ម\u{17b7}"), ("kn", "ಎಹ\u{cbf}ಮ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "에히메 현"), ("lt", "Ehimės prefektūra"), ("lv", "Ehimes prefektūra"), ("mk", "Ехиме"), ("mr", "एहिम\u{947}"), ("ms", "Wilayah Ehime"), ("nb", "Ehime"), ("nl", "Ehime"), ("no", "Ehime"), ("pl", "Prefektura Ehime"), ("pt", "Ehime"), ("ro", "Prefectura Ehime"), ("ru", "Эхиме"), ("si", "එහ\u{dd2}මේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ehime"), ("sr", "Ехиме"), ("sr_Latn", "Ehime"), ("sv", "Ehime prefektur"), ("sw", "Mkoa wa Ehime"), ("ta", "தேஹிமே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఇహ\u{c48}మ\u{c4d} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอะฮ\u{e34}เมะ"), ("tr", "Ehime"), ("uk", "Префектура Ехіме"), ("ur", "اہیمے پریفیکچر"), ("vi", "Ehime"), ("yue", "愛媛縣"), ("yue_Hans", "爱媛县"), ("zh", "爱媛县")]),
                        unofficial_name_list: ["Ehime"].to_vec(),
                    }
                ),
                (
                    "39",
                    Subdivision{
                        name: "39",
                        country_alpha2: Alpha2::JP,
                        code: "39",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5588035), longitude: Some(133.5311675), max_latitude: Some(33.6813748), min_latitude: Some(33.4595414), max_longitude: Some(133.6254958), min_longitude: Some(133.3942252)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kochi Prefektuur"), ("ar", "كوتشي"), ("az", "Koçi"), ("be", "Прэфектура Коты"), ("bg", "Кочи"), ("bn", "কোওচি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kōchi"), ("ccp", "𑄇\u{1112e}𑄌\u{11128}"), ("ceb", "Kōchi-ken"), ("cs", "Prefektura Kóči"), ("cy", "Kōchi"), ("da", "Kouchi-præfekturet"), ("de", "Präfektur Kōchi"), ("el", "Κότσι"), ("en", "Kōchi"), ("es", "Prefectura de Kōchi"), ("et", "Kōchi prefektuur"), ("eu", "Kōchi"), ("fa", "استان کوچی"), ("fi", "Kōchin prefektuuri"), ("fr", "préfecture de Kōchi"), ("ga", "Maoracht Kōchi"), ("gl", "Prefectura de Kōchi"), ("gu", "કોચી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "קוצ׳י"), ("hi", "कोच\u{94d}ची प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Kōchi"), ("hu", "Kócsi prefektúra"), ("id", "Prefektur Kochi"), ("it", "prefettura di Kōchi"), ("ja", "高知県"), ("km", "ខេត\u{17d2}តខ\u{17bc}ឈ\u{17b7}"), ("kn", "ಕೊಚ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "고치 현"), ("lt", "Kočio prefektūra"), ("lv", "Koči prefektūra"), ("mk", "Кочи"), ("mn", "Кочи-кэн"), ("mr", "कोची"), ("ms", "Wilayah Kōchi"), ("nb", "Kōchi"), ("nl", "Kochi"), ("no", "Kōchi"), ("pl", "Prefektura Kōchi"), ("pt", "Kochi"), ("ro", "Prefectura Kōchi"), ("ru", "Коти"), ("si", "කොච\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kóči"), ("sr", "Кочи"), ("sr_Latn", "Koči"), ("sv", "Kochi prefektur"), ("sw", "Mkoa wa Kochi"), ("ta", "கொச\u{bcd}சி ப\u{bcd}ரெபெக\u{bcd}டர\u{bcd}"), ("te", "క\u{c4b}చ\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคช\u{e34}"), ("tr", "Kōchi"), ("uk", "Префектура Кочі"), ("ur", "کوچی پریفیکچر"), ("vi", "Kōchi"), ("yue", "高知縣"), ("yue_Hans", "高知县"), ("zh", "高知县")]),
                        unofficial_name_list: ["Kochi"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::JP,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.5903547), longitude: Some(130.4017155), max_latitude: Some(33.8741146), min_latitude: Some(33.4251209), max_longitude: Some(130.4952854), min_longitude: Some(130.0327988)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Fukuoka Prefektuur"), ("ar", "فوكوكا"), ("az", "Fukuoka"), ("be", "прэфектура Фукуока"), ("bg", "Фукуока"), ("bn", "ফ\u{9c1}ক\u{9c1}ওক\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Fukuoka"), ("ccp", "𑄜\u{1112a}𑄇\u{1112a}𑄃\u{1112e}𑄇"), ("ceb", "Fukuoka-ken"), ("cs", "Prefektura Fukuoka"), ("cy", "Fukuoka"), ("da", "Fukuoka-præfekturet"), ("de", "Präfektur Fukuoka"), ("el", "Φουκουόκα"), ("en", "Fukuoka"), ("es", "Prefectura de Fukuoka"), ("et", "Fukuoka prefektuur"), ("eu", "Fukuoka"), ("fa", "استان فوکوئوکا"), ("fi", "Fukuokan prefektuuri"), ("fr", "préfecture de Fukuoka"), ("ga", "Maoracht Fukuoka"), ("gl", "Prefectura de Fukuoka"), ("gu", "ફ\u{ac1}ક\u{ac1}ઓકા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "פוקואוקה"), ("hi", "फ\u{941}क\u{941}ओका प\u{94d}रान\u{94d}त"), ("hr", "Fukuoka"), ("hu", "Fukuoka prefektúra"), ("hy", "Ֆուկուոկա"), ("id", "Prefektur Fukuoka"), ("it", "prefettura di Fukuoka"), ("ja", "福岡県"), ("jv", "Prefektur Fukuoka"), ("kn", "ಫುಕುಕಾಕಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "후쿠오카 현"), ("lt", "Fukuokos prefektūra"), ("lv", "Fukuokas prefektūra"), ("mk", "Фукуока"), ("mr", "फ\u{941}क\u{941}ओका"), ("ms", "Wilayah Fukuoka"), ("nb", "Fukuoka"), ("nl", "Fukuoka"), ("no", "Fukuoka"), ("pl", "Prefektura Fukuoka"), ("pt", "Fukuoka"), ("ro", "Prefectura Fukuoka"), ("ru", "Фукуока"), ("si", "ෆ\u{dd4}කෝක\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Fukuoka"), ("sl", "Prefektura Fukuoka"), ("sr", "Фукуока"), ("sr_Latn", "Fukuoka"), ("sv", "Fukuoka prefektur"), ("sw", "Mkoa wa Fukuoka"), ("ta", "பிக\u{bcd}குஒக\u{bcd}க\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ఫుకువ\u{c4b}క\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟ\u{e38}ก\u{e38}โอะกะ"), ("tr", "Fukuoka"), ("uk", "Префектура Фукуока"), ("ur", "فوکوکا پریفیکچر"), ("vi", "Fukuoka"), ("yue", "福岡縣"), ("yue_Hans", "福冈县"), ("zh", "福冈县")]),
                        unofficial_name_list: ["Fukuoka", "Hukuoka"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::JP,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.2494416), longitude: Some(130.2997942), max_latitude: Some(33.6188957), min_latitude: Some(32.9505422), max_longitude: Some(130.5422784), min_longitude: Some(129.7395896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saga Prefektuur"), ("ar", "ساغا"), ("az", "Saqa"), ("be", "прэфектура Сага"), ("bg", "Сага"), ("bn", "স\u{9be}গ\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Saga"), ("ccp", "𑄥𑄉"), ("ceb", "Saga-ken"), ("cs", "Prefektura Saga"), ("cy", "Saga"), ("da", "Saga Prefecture"), ("de", "Präfektur Saga"), ("el", "Σάγκα"), ("en", "Saga"), ("es", "Prefectura de Saga"), ("et", "Saga prefektuur"), ("eu", "Saga"), ("fa", "استان ساگا"), ("fi", "Sagan prefektuuri"), ("fr", "préfecture de Saga"), ("ga", "Maoracht Saga"), ("gl", "Prefectura de Saga"), ("gu", "સાગા પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "סאגה"), ("hi", "सागा प\u{94d}रान\u{94d}त"), ("hr", "Saga"), ("hu", "Szaga prefektúra"), ("id", "Prefektur Saga"), ("it", "Prefettura di Saga"), ("ja", "佐賀県"), ("kn", "ಸಾಗಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "사가 현"), ("lt", "Sagos prefektūra"), ("lv", "Sagas prefektūra"), ("mk", "Сага"), ("mr", "सागा"), ("ms", "Wilayah Saga"), ("nb", "Saga"), ("nl", "Saga"), ("no", "Saga"), ("pl", "Prefektura Saga"), ("pt", "Saga"), ("ro", "Prefectura Saga"), ("ru", "Сага"), ("si", "සග\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Saga"), ("sl", "prefektura Saga, Japonska"), ("sr", "Сага"), ("sr_Latn", "Saga"), ("sv", "Saga prefektur"), ("sw", "Mkoa wa Saga"), ("ta", "சக\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "స\u{c3e}గ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซะงะ"), ("tr", "Saga"), ("uk", "Префектура Саґа"), ("ur", "سگا پریفیکچر"), ("vi", "Saga"), ("yue", "佐賀縣"), ("yue_Hans", "佐贺县"), ("zh", "佐贺县")]),
                        unofficial_name_list: ["Saga"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::JP,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.7502856), longitude: Some(129.877667), max_latitude: Some(32.9686468), min_latitude: Some(32.5491748), max_longitude: Some(129.993817), min_longitude: Some(129.5528394)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nagasaki Prefektuur"), ("ar", "ناغاساكي"), ("az", "Naqasaki"), ("be", "Прэфектура Нагасакі"), ("bg", "Нагасаки"), ("bn", "ন\u{9be}গ\u{9be}স\u{9be}কি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Nagasaki"), ("ccp", "𑄚𑄉𑄥𑄇\u{11128}"), ("ceb", "Nagasaki (prepektura sa Hapon)"), ("cs", "Prefektura Nagasaki"), ("cy", "Nagasaki"), ("da", "Nagasaki-præfekturet"), ("de", "Präfektur Nagasaki"), ("el", "Νομαρχία Ναγκασάκι"), ("en", "Nagasaki"), ("es", "Prefectura de Nagasaki"), ("et", "Nagasaki prefektuur"), ("eu", "Nagasaki"), ("fa", "استان ناگاساکی"), ("fi", "Nagasakin prefektuuri"), ("fr", "préfecture de Nagasaki"), ("ga", "Maoracht Nagasaki"), ("gl", "Prefectura de Nagasaki"), ("gu", "નાગાસાકી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "נגסאקי"), ("hi", "नागासाकी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Nagasaki"), ("hu", "Nagaszaki prefektúra"), ("hy", "Նագասակի"), ("id", "Prefektur Nagasaki"), ("it", "prefettura di Nagasaki"), ("ja", "長崎県"), ("km", "ខេត\u{17d2}តណាហ\u{17d2}គាសាគ\u{17b8}"), ("kn", "ನಾಗಸಾಕ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "나가사키 현"), ("lt", "Nagasakio prefektūra"), ("lv", "Nagasaki prefektūra"), ("mk", "Нагасаки"), ("mr", "नागासाकी"), ("ms", "Wilayah Nagasaki"), ("nb", "Nagasaki"), ("nl", "Nagasaki"), ("no", "Nagasaki"), ("pl", "Prefektura Nagasaki"), ("pt", "Nagasaki"), ("ro", "Prefectura Nagasaki"), ("ru", "Нагасаки"), ("si", "න\u{dcf}ගස\u{dcf}ක\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Nagasaki"), ("sl", "prefektura Nagasaki"), ("sr", "Нагасаки"), ("sr_Latn", "Nagasaki"), ("sv", "Nagasaki prefektur"), ("sw", "Mkoa wa Nagasaki"), ("ta", "ந\u{bbe}கச\u{bbe}கி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "న\u{c3e}గ\u{c3e}స\u{c3e}క\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนะงะซะก\u{e34}"), ("tr", "Nagasaki"), ("uk", "Префектура Наґасакі"), ("ur", "ناگاساکی پریفیکچر"), ("vi", "Nagasaki"), ("yue", "長崎縣"), ("yue_Hans", "长崎县"), ("zh", "长崎县")]),
                        unofficial_name_list: ["Nagasaki"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::JP,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.8031004), longitude: Some(130.7078911), max_latitude: Some(32.9799778), min_latitude: Some(32.6602942), max_longitude: Some(130.828973), min_longitude: Some(130.57147)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kumamoto Prefektuur"), ("ar", "كوماموتو"), ("az", "Kumamoto"), ("be", "Прэфектура Кумамота"), ("bg", "Кумамото"), ("bn", "ক\u{9c1}ম\u{9be}মোতো প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kumamoto"), ("ccp", "𑄇\u{1112a}𑄟𑄟\u{1112e}𑄑\u{1112e}"), ("ceb", "Kumamoto-ken"), ("cs", "Prefektura Kumamoto"), ("cy", "Kumamoto"), ("da", "Kumamoto-præfekturet"), ("de", "Präfektur Kumamoto"), ("el", "Κουμαμότο"), ("en", "Kumamoto"), ("es", "Prefectura de Kumamoto"), ("et", "Kumamoto prefektuur"), ("eu", "Kumamoto"), ("fa", "استان کوماموتو"), ("fi", "Kumamoton prefektuuri"), ("fr", "préfecture de Kumamoto"), ("ga", "Maoracht Kumamoto"), ("gl", "Prefectura de Kumamoto"), ("gu", "ક\u{ac1}મામોટો પ\u{acd}રીફ\u{ac7}કચર"), ("he", "קוממוטו"), ("hi", "क\u{941}मामोटो प\u{94d}रान\u{94d}त"), ("hr", "Kumamoto"), ("hu", "Kumamoto prefektúra"), ("id", "Prefektur Kumamoto"), ("it", "prefettura di Kumamoto"), ("ja", "熊本県"), ("km", "ខេត\u{17d2}តឃ\u{17b9}ម\u{17c9}ាម\u{17c9}\u{17bb}ត\u{17bb}"), ("kn", "ಕುಮಾಮೊಟೊ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "구마모토 현"), ("lt", "Kumamoto prefektūra"), ("lv", "Kumamoto prefektūra"), ("mk", "Кумамото"), ("mr", "क\u{941}मामोतो"), ("ms", "Wilayah Kumamoto"), ("nb", "Kumamoto"), ("nl", "Kumamoto"), ("no", "Kumamoto"), ("pl", "Prefektura Kumamoto"), ("pt", "Kumamoto"), ("ro", "Prefectura Kumamoto"), ("ru", "Кумамото"), ("si", "ක\u{dd4}මමොටෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kumamoto"), ("sl", "prefektura Kumamoto"), ("sr", "Кумамото"), ("sr_Latn", "Kumamoto"), ("sv", "Kumamoto prefektur"), ("sw", "Mkoa wa Kumamoto"), ("ta", "குமமோடோ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "కుమ\u{c3e}మ\u{c4b}ట\u{c4b} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e38}มะโมะโตะ"), ("tr", "Kumamoto"), ("uk", "Префектура Кумамото"), ("ur", "کوماموتو پریفیکچر"), ("vi", "Kumamoto"), ("yue", "熊本縣"), ("yue_Hans", "熊本县"), ("zh", "熊本縣")]),
                        unofficial_name_list: ["Kumamoto"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::JP,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.2395578), longitude: Some(131.609272), max_latitude: Some(33.2805132), min_latitude: Some(33.0697618), max_longitude: Some(131.9568311), min_longitude: Some(131.418656)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oita Prefektuur"), ("ar", "أويتا"), ("az", "Oita"), ("be", "прэфектура Оіта"), ("bg", "Оита"), ("bn", "ওওইত\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Ōita"), ("ccp", "𑄃\u{11130}𑄑"), ("ceb", "Ōita Prefecture"), ("cs", "Prefektura Óita"), ("cy", "Ōita"), ("da", "Ooita-præfekturet"), ("de", "Präfektur Ōita"), ("el", "Οοϊτά"), ("en", "Ōita"), ("es", "Prefectura de Ōita"), ("et", "Ōita prefektuur"), ("eu", "Ōita"), ("fa", "استان اوئیتا"), ("fi", "Ōitan prefektuuri"), ("fr", "préfecture d’Ōita"), ("ga", "Maoracht Ōita"), ("gl", "Prefectura de Ōita"), ("gu", "ઓઈટા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "אויטה"), ("hi", "ओइटा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Ōita"), ("hu", "Óita prefektúra"), ("id", "Prefektur Oita"), ("it", "prefettura di Ōita"), ("ja", "大分県"), ("km", "ខេត\u{17d2}តអយតាក\u{17cb}"), ("kn", "ಒಐಟಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "오이타 현"), ("lt", "Oitos prefektūra"), ("lv", "Oitas prefektūra"), ("mk", "Оита"), ("mr", "ओइता"), ("ms", "Wilayah Ōita"), ("nb", "Ōita"), ("nl", "Oita"), ("no", "Ōita"), ("pl", "Prefektura Ōita"), ("pt", "Oita"), ("ro", "Prefectura Ōita"), ("ru", "Оита"), ("si", "ඕය\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Óita"), ("sl", "prefektura Ōita"), ("sr", "Оита"), ("sr_Latn", "Oita"), ("sv", "Oita prefektur"), ("sw", "Mkoa wa Ōita"), ("ta", "ஓட\u{bcd}ட ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ఓయ\u{c3f}ట\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโออ\u{e34}ตะ"), ("tr", "Ōita"), ("uk", "Префектура Ойта"), ("ur", "اوئیتا پریفیکچر"), ("vi", "Ōita"), ("yue", "大分縣"), ("yue_Hans", "大分县"), ("zh", "大分县")]),
                        unofficial_name_list: ["Oita"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::JP,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.9076736), longitude: Some(131.4202411), max_latitude: Some(32.0659325), min_latitude: Some(31.7210322), max_longitude: Some(131.5057758), min_longitude: Some(131.1889657)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Miyazaki Prefektuur"), ("ar", "ميازاكي"), ("az", "Miyazaki"), ("be", "прэфектура Міядзакі"), ("bg", "Миядзаки"), ("bn", "মিয\u{9bc}\u{9be}য\u{9be}কি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Miyazaki"), ("ccp", "𑄟\u{11128}𑄠𑄎𑄇\u{11128}"), ("ceb", "Miyazaki-ken"), ("cs", "Prefektura Mijazaki"), ("cy", "Miyazaki"), ("da", "Miyazaki-præfekturet"), ("de", "Präfektur Miyazaki"), ("el", "Μιγιαζάκι"), ("en", "Miyazaki"), ("es", "Prefectura de Miyazaki"), ("et", "Miyazaki prefektuur"), ("eu", "Miyazaki"), ("fa", "استان میازاکی"), ("fi", "Miyazakin prefektuuri"), ("fr", "préfecture de Miyazaki"), ("ga", "Maoracht Miyazaki"), ("gl", "Prefectura de Miyazaki"), ("gu", "મિયાઝાકી પ\u{acd}રીફ\u{ac7}કચર"), ("he", "מיאזקי"), ("hi", "मियाज\u{93c}ाकि प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Miyazaki"), ("hu", "Mijazaki prefektúra"), ("hy", "Միյաձակի"), ("id", "Prefektur Miyazaki"), ("it", "prefettura di Miyazaki"), ("ja", "宮崎県"), ("km", "ខេត\u{17d2}តម\u{17b8}យ\u{17c9}ាសាគ\u{17b8}"), ("kn", "ಮ\u{cbf}ಯಾಜಾಕ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "미야자키 현"), ("lt", "Mijadzakio prefektūra"), ("lv", "Mijadzaki prefektūra"), ("mk", "Мијазаки"), ("mr", "मियाझाकी"), ("ms", "Wilayah Miyazaki"), ("nb", "Miyazaki"), ("nl", "Miyazaki"), ("no", "Miyazaki"), ("pl", "Prefektura Miyazaki"), ("pt", "Miyazaki"), ("ro", "Prefectura Miyazaki"), ("ru", "Миядзаки"), ("si", "ම\u{dd2}යස\u{dcf}ක\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mijazaki"), ("sr", "Мијазаки"), ("sr_Latn", "Mijazaki"), ("sv", "Miyazaki prefektur"), ("sw", "Mkoa wa Miyazaki"), ("ta", "மிய\u{bbe}ச\u{bbe}க\u{bcd}கி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3f}య\u{c3e}జ\u{c3e}క\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}ยะซะก\u{e34}"), ("tr", "Miyazaki"), ("uk", "Префектура Міядзакі"), ("ur", "میازاکی پریفیکچر"), ("vi", "Miyazaki"), ("yue", "宮崎縣"), ("yue_Hans", "宫崎县"), ("zh", "宮崎縣")]),
                        unofficial_name_list: ["Miyazaki"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::JP,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.5965535), longitude: Some(130.5571158), max_latitude: Some(31.7527317), min_latitude: Some(31.2933182), max_longitude: Some(130.7248899), min_longitude: Some(130.3867022)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kagoshima Prefektuur"), ("ar", "كاغوشيما"), ("az", "Kaqoşima"), ("be", "прэфектура Кагосіма"), ("bg", "Кагошима"), ("bn", "ক\u{9be}গোশিম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura de Kagoshima"), ("ccp", "𑄇𑄉\u{1112e}𑄥\u{11128}𑄟"), ("ceb", "Kagoshima-ken"), ("cs", "Prefektura Kagošima"), ("cy", "Kagoshima"), ("da", "Kagoshima-præfekturet"), ("de", "Präfektur Kagoshima"), ("el", "Περιφέρεια Καγκοσίμα"), ("en", "Kagoshima"), ("es", "Kagoshima"), ("et", "Kagoshima prefektuur"), ("eu", "Kagoshima"), ("fa", "استان کاگوشیما"), ("fi", "Kagoshiman prefektuuri"), ("fr", "préfecture de Kagoshima"), ("ga", "Maoracht Kagoshima"), ("gl", "Prefectura de Kagoshima"), ("gu", "કાગોશીમા પ\u{acd}રીફ\u{ac7}કચર"), ("he", "קגושימה"), ("hi", "कागोशिमा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Kagoshima"), ("hu", "Kagosima prefektúra"), ("hy", "Կոգոշիմա"), ("id", "Prefektur Kagoshima"), ("it", "prefettura di Kagoshima"), ("ja", "鹿児島県"), ("jv", "Prefektur Kagoshima"), ("km", "ខេត\u{17d2}តខាហ\u{17d2}គ\u{17bb}ស\u{17ca}\u{17b8}ម\u{17c9}ា"), ("kn", "ಕಾಗೊಶ\u{cbf}ಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "가고시마 현"), ("lt", "Kagošimos prefektūra"), ("lv", "Kagosimas prefektūra"), ("mk", "Кагошима"), ("mr", "कागोशिमा"), ("ms", "Wilayah Kagoshima"), ("nb", "Kagoshima"), ("nl", "Kagoshima"), ("no", "Kagoshima"), ("pl", "Prefektura Kagoshima"), ("pt", "Kagoshima"), ("ro", "Prefectura Kagoshima"), ("ru", "Кагосима"), ("si", "කගොශ\u{dd2}ම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kagošima"), ("sl", "Prefektura Kagošima"), ("sr", "Кагошима"), ("sr_Latn", "Kagošima"), ("sv", "Kagoshima prefektur"), ("sw", "Mkoa wa Kagoshima"), ("ta", "ககோஷிம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "క\u{c3e}గ\u{c4b}ష\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคะโงะช\u{e34}มะ"), ("tr", "Kagoşima ili"), ("uk", "Префектура Каґошіма"), ("ur", "کاگوشیما پریفیکچر"), ("vi", "Kagoshima"), ("yue", "鹿兒島縣"), ("yue_Hans", "鹿儿岛县"), ("zh", "鹿儿岛县")]),
                        unofficial_name_list: ["Kagoshima", "Kagosima"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::JP,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.2124013), longitude: Some(127.6809317), max_latitude: Some(27.8854933), min_latitude: Some(24.0458042), max_longitude: Some(131.3320097), min_longitude: Some(122.933796)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Okinawa Prefektuur"), ("ar", "أوكيناوا"), ("az", "Okinava"), ("be", "Прэфектура Акінава"), ("bg", "Окинава"), ("bn", "ওকিন\u{9be}ওয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "Prefectura d’Okinawa"), ("ccp", "𑄃\u{1112e}𑄇\u{11128}𑄚𑄤"), ("ceb", "Okinawa-ken"), ("cs", "Prefektura Okinawa"), ("cy", "Okinawa"), ("da", "Okinawa-præfekturet"), ("de", "Okinawa"), ("el", "Οκινάβα"), ("en", "Okinawa"), ("es", "Prefectura de Okinawa"), ("et", "Okinawa prefektuur"), ("eu", "Okinawa"), ("fa", "استان اوکیناوا"), ("fi", "Okinawan prefektuuri"), ("fr", "préfecture d’Okinawa"), ("ga", "Maoracht Okinawa"), ("gu", "ઓકિનાવા પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("he", "אוקינאווה"), ("hi", "ओकीनावा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("hr", "Okinawa, prefektura"), ("hu", "Okinava prefektúra"), ("hy", "Օկինավա"), ("id", "Prefektur Okinawa"), ("it", "prefettura di Okinawa"), ("ja", "沖縄県"), ("jv", "Prefektur Okinawa"), ("km", "ខេត\u{17d2}តអ\u{17bc}គ\u{17b8}ណាវ\u{17c9}ា"), ("kn", "ಒಕ\u{cbf}ನಾವಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "오키나와 현"), ("lt", "Okinavos prefektūra"), ("lv", "Okinavas prefektūra"), ("mk", "Окинава"), ("mr", "ओकिनावा"), ("ms", "Wilayah Okinawa"), ("nb", "Okinawa"), ("nl", "Okinawa"), ("no", "Okinawa"), ("pl", "Prefektura Okinawa"), ("pt", "Okinawa"), ("ro", "Prefectura Okinawa"), ("ru", "Окинава"), ("si", "ඔක\u{dd2}න\u{dcf}ව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Okinawa"), ("sl", "prefektura Okinava"), ("sr", "Окинава"), ("sr_Latn", "Okinava"), ("sv", "Okinawa prefektur"), ("sw", "Mkoa wa Okinawa"), ("ta", "ஓக\u{bcd}கின\u{bbe}வ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓక\u{c3f}న\u{c3e}వ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอะก\u{e34}นะวะ"), ("tr", "Okinawa"), ("uk", "Префектура Окінава"), ("ur", "اوکیناوا پریفیکچر"), ("vi", "Okinawa"), ("yue", "沖繩縣"), ("yue_Hans", "冲绳县"), ("zh", "沖繩縣")]),
                        unofficial_name_list: ["Okinawa"].to_vec(),
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
#[cfg(feature = "jp")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::JP,
        alpha3: Alpha3::JPN,
        address_format: Some(
            "〒{{postalcode}}\n{{region_short}}{{city}}{{street}}\n{{recipient}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 81,
        currency_code: CurrencyCode::JPY,
        gec: Some(GEC::JA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "010",
        ioc: Some(IOC::JPN),
        iso_long_name: "Japan",
        iso_short_name: "Japan",
        official_language_list: ["ja"].to_vec(),
        spoken_language_list: ["ja"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Japanese"),
        number: "392",
        postal_code: true,
        postal_code_format: Some("\\d{3}-?\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "JP",
        unofficial_name_list: ["Japan", "Japon", "Japón", "日本"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Japan"),
            ("af", "Japan"),
            ("ak", "Japan"),
            ("am", "ጃፓን"),
            ("an", "Japan"),
            ("ar", "اليابان"),
            ("as", "জ\u{9be}প\u{9be}ন"),
            ("ay", "Japan"),
            ("az", "Yaponiya"),
            ("ba", "Japan"),
            ("be", "Японія"),
            ("bg", "Япония"),
            ("bi", "Japan"),
            ("bn", "জ\u{9be}প\u{9be}ন"),
            ("bn_IN", "জ\u{9be}প\u{9be}ন"),
            ("br", "Japan"),
            ("bs", "Japan"),
            ("ca", "Japó"),
            ("ce", "Япони"),
            ("ch", "Chapan"),
            ("cs", "Japonsko"),
            ("cv", "Япони"),
            ("cy", "Japan"),
            ("da", "Japan"),
            ("de", "Japan"),
            ("dv", "ޖ\u{7a6}ޕ\u{7a7}ނ\u{7aa}"),
            ("dz", "ཇ་པ\u{f71}ན།"),
            ("ee", "Japan"),
            ("el", "Ιαπωνία"),
            ("en", "Japan"),
            ("eo", "Japanio"),
            ("es", "Japón"),
            ("et", "Jaapan"),
            ("eu", "Japonia"),
            ("fa", "ژاپن"),
            ("ff", "Japan"),
            ("fi", "Japani"),
            ("fo", "Japan"),
            ("fr", "Japon"),
            ("fy", "Japan"),
            ("ga", "An tSeapáin"),
            ("gl", "Xapón"),
            ("gn", "Japan"),
            ("gu", "જાપાન"),
            ("gv", "Yn Çhapaan"),
            ("ha", "Japan"),
            ("he", "יפן"),
            ("hi", "जापान"),
            ("hr", "Japan"),
            ("ht", "Japon"),
            ("hu", "Japán"),
            ("hy", "Ճապոնիա"),
            ("ia", "Japon"),
            ("id", "Jepang"),
            ("io", "Japonia"),
            ("is", "Japan"),
            ("it", "Giappone"),
            ("iu", "ᓃᑉᐊᓐ"),
            ("ja", "日本"),
            ("ka", "იაპონია"),
            ("ki", "Japan"),
            ("kk", "Жапония"),
            ("kl", "Japan"),
            ("km", "ជប\u{17c9}\u{17bb}ន"),
            ("kn", "ಜಪಾನ\u{ccd}"),
            ("ko", "일본"),
            ("ku", "Japonya"),
            ("kv", "Япония"),
            ("kw", "Nihon"),
            ("ky", "Жапония"),
            ("lo", "ປະເທດຍ\u{eb5}\u{ec8}ປ\u{eb8}\u{ec8}ນ"),
            ("lt", "Japonija"),
            ("lv", "Japāna"),
            ("mi", "Nipono"),
            ("mk", "Јапонија"),
            ("ml", "ജപ\u{d4d}പ\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Япон"),
            ("mr", "जपान"),
            ("ms", "Jepun"),
            ("mt", "Ġappun"),
            ("my", "ဂျပန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Djapan"),
            ("nb", "Japan"),
            ("ne", "जापान"),
            ("nl", "Japan"),
            ("nn", "Japan"),
            ("nv", "Binaʼadaałtzózí Dinéʼiʼ Bikéyah"),
            ("oc", "Japon"),
            ("or", "ଜ\u{b3e}ପ\u{b3e}ନ"),
            ("pa", "ਜਾਪਾਨ"),
            ("pi", "जापान"),
            ("pl", "Japonia"),
            ("ps", "جاپان"),
            ("pt", "Japão"),
            ("pt_BR", "Japão"),
            ("ro", "Japonia"),
            ("ru", "Япония"),
            ("rw", "Ubuyapani"),
            ("sc", "Giapone"),
            ("sd", "جاپان"),
            ("si", "ජප\u{dcf}නය"),
            ("sk", "Japonsko"),
            ("sl", "Japonska"),
            ("so", "Jabbaan"),
            ("sq", "Japoni"),
            ("sr", "Јапан"),
            ("sv", "Japan"),
            ("sw", "Japan"),
            ("ta", "ஜப\u{bcd}ப\u{bbe}ன\u{bcd}"),
            ("te", "జప\u{c3e}న\u{c4d}"),
            ("tg", "Ҷопон"),
            ("th", "ญ\u{e35}\u{e48}ป\u{e38}\u{e48}น"),
            ("ti", "ጃፓን"),
            ("tk", "Ýaponiýa"),
            ("tl", "Hapon"),
            ("tr", "Japonya"),
            ("tt", "Жапан, Япониа"),
            ("ug", "ياپونىيە"),
            ("uk", "Японія"),
            ("ur", "جاپان"),
            ("uz", "Yaponiya"),
            ("ve", "Japan"),
            ("vi", "Nhật"),
            ("wa", "Djapon"),
            ("wo", "Japoŋ"),
            ("xh", "Japhani"),
            ("yo", "Japan"),
            ("zh_CN", "日本"),
            ("zh_HK", "日本"),
            ("zh_TW", "日本"),
            ("zu", "IJaphani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
