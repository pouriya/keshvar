// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Liberia

#[cfg(all(feature = "lr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::LR;
    pub const ALPHA3: Alpha3 = Alpha3::LBR;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 231;
    pub const CURRENCY_CODE: &str = "LRD";
    pub const GEC: Option<GEC> = Some(GEC::LI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("LBR");
    pub const ISO_SHORT_NAME: &str = "Liberia";
    pub const ISO_LONG_NAME: &str = "The Republic of Liberia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6, 7, 8];
    pub const NATIONAL_PREFIX: &str = "22";
    pub const NATIONALITY: Option<&str> = Some("Liberian");
    pub const NUMBER: &str = "430";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAfrica);
    pub const UN_LOCODE: &str = "LR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Liberia", "リベリア"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Liberia"),
        ("af", "Liberië"),
        ("ak", "Liberia"),
        ("am", "ላይቤሪያ"),
        ("an", "Liberia"),
        ("ar", "ليبيريا"),
        ("as", "ল\u{9be}ইবেৰিয়\u{9be}"),
        ("ay", "Liberia"),
        ("az", "Liberiya"),
        ("ba", "Liberia"),
        ("be", "Ліберыя"),
        ("bg", "Либерия"),
        ("bi", "Liberia"),
        ("bn", "ল\u{9be}ইবেরিয়\u{9be}"),
        ("bn_IN", "ল\u{9be}ইবেরিয়\u{9be}"),
        ("br", "Liberia"),
        ("bs", "Liberija"),
        ("ca", "Libèria"),
        ("ce", "Либери"),
        ("ch", "Liberia"),
        ("cs", "Libérie"),
        ("cv", "Либери"),
        ("cy", "Liberia"),
        ("da", "Liberia"),
        ("de", "Liberia"),
        ("dv", "ލ\u{7a6}އ\u{7a8}ބ\u{7a9}ރ\u{7a8}އ\u{7a7}"),
        ("dz", "ལ\u{f72}་བ\u{f72}་ར\u{f72}་ཡ།"),
        ("ee", "Liberia"),
        ("el", "Λιβερία"),
        ("en", "Liberia"),
        ("eo", "Liberio"),
        ("es", "Liberia"),
        ("et", "Libeeria"),
        ("eu", "Liberia"),
        ("fa", "لیبریا"),
        ("ff", "Labiriyaa"),
        ("fi", "Liberia"),
        ("fo", "Liberia"),
        ("fr", "Libéria"),
        ("fy", "Libearia"),
        ("ga", "An Libéir"),
        ("gl", "Liberia"),
        ("gn", "Liberia"),
        ("gu", "લાઇબ\u{ac7}રિયા"),
        ("gv", "Yn Laibeer"),
        ("ha", "Liberia"),
        ("he", "ליבריה"),
        ("hi", "लाइब\u{947}रिया"),
        ("hr", "Liberija"),
        ("ht", "Liberya"),
        ("hu", "Libéria"),
        ("hy", "Լիբերիա"),
        ("ia", "Liberia"),
        ("id", "Liberia"),
        ("io", "Liberia"),
        ("is", "Líbería"),
        ("it", "Liberia"),
        ("iu", "Liberia"),
        ("ja", "リベリア"),
        ("ka", "ლიბერია"),
        ("ki", "Liberia"),
        ("kk", "Либерия"),
        ("kl", "Liberia"),
        ("km", "ល\u{17b8}បេរ\u{17b8}យ\u{17c9}ា"),
        ("kn", "ಲೈಬೀರ\u{cbf}ಯಾ"),
        ("ko", "라이베리아"),
        ("ku", "Lîberya"),
        ("kv", "Liberia"),
        ("kw", "Liberi"),
        ("ky", "Либерия"),
        ("lo", "Liberia"),
        ("lt", "Liberija"),
        ("lv", "Libērija"),
        ("mi", "Liberia"),
        ("mk", "Либерија"),
        ("ml", "ലൈബീരിയ"),
        ("mn", "Liberia"),
        ("mr", "लायब\u{947}रिया"),
        ("ms", "Liberia"),
        ("mt", "Liberja"),
        (
            "my",
            "လ\u{102d}\u{102f}က\u{103a}ဘေးရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Liberia"),
        ("nb", "Liberia"),
        ("ne", "लिब\u{947}रीया"),
        ("nl", "Liberia"),
        ("nn", "Liberia"),
        ("nv", "Liberia"),
        ("oc", "Libèria"),
        ("or", "ଲ\u{b3e}ଇବେର\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਲੀਬਾਰੀਆ"),
        ("pi", "लायबीरिया"),
        ("pl", "Liberia"),
        ("ps", "لایبریا"),
        ("pt", "Libéria"),
        ("pt_BR", "Libéria"),
        ("ro", "Liberia"),
        ("ru", "Либерия"),
        ("rw", "Liberiya"),
        ("sc", "Libèria"),
        ("sd", "Liberia"),
        ("si", "ල\u{dd2}ය\u{dd2}බ\u{dd3}ර\u{dd2}ය\u{dcf}ව"),
        ("sk", "Libéria"),
        ("sl", "Liberija"),
        ("so", "Laybeeriya"),
        ("sq", "Liberi"),
        ("sr", "Либерија"),
        ("sv", "Liberia"),
        ("sw", "Liberia"),
        ("ta", "லைப\u{bc0}ரிய\u{bbe}"),
        ("te", "ల\u{c48}బ\u{c40}ర\u{c3f}య\u{c3e}"),
        ("tg", "Либерия"),
        ("th", "ไลบ\u{e35}เร\u{e35}ย"),
        ("ti", "Liberia"),
        ("tk", "Liberiýa"),
        ("tl", "Liberia"),
        ("tr", "Liberya"),
        ("tt", "Либериа"),
        ("ug", "لىبېرىيە"),
        ("uk", "Ліберія"),
        ("ur", "لائبیریا"),
        ("uz", "Liberiya"),
        ("ve", "Liberia"),
        ("vi", "Li-bê-ri-a"),
        ("wa", "Liberia"),
        ("wo", "Libeeria"),
        ("xh", "Liberia"),
        ("yo", "Làìbéríà"),
        ("zh_CN", "利比里亚"),
        ("zh_HK", "利比里亞"),
        ("zh_TW", "賴比瑞亞"),
        ("zu", "ILiberia"),
    ];
    #[cfg(all(feature = "lr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 6.428055;
        pub const LONGITUDE: f64 = -9.429499000000002;
        pub const MAX_LATITUDE: f64 = 8.551986;
        pub const MAX_LONGITUDE: f64 = -7.3692549;
        pub const MIN_LATITUDE: f64 = 4.269699999999999;
        pub const MIN_LONGITUDE: f64 = -11.5355999;
        pub const NORTHEAST_LATITUDE: f64 = 8.551986;
        pub const NORTHEAST_LONGITUDE: f64 = -7.3692549;
        pub const SOUTHWEST_LATITUDE: f64 = 4.269699999999999;
        pub const SOUTHWEST_LONGITUDE: f64 = -11.5355999;
    }
}
#[cfg(all(feature = "lr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 6.428055,
            longitude: -9.429499000000002,
            max_latitude: 8.551986,
            max_longitude: -7.3692549,
            min_latitude: 4.269699999999999,
            min_longitude: -11.5355999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 8.551986,
                    longitude: -7.3692549,
                },
                southwest: CountryGeoBound {
                    latitude: 4.269699999999999,
                    longitude: -11.5355999,
                },
            },
        }
    }
}

#[cfg(all(feature = "lr", feature = "subdivisions"))]
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
                    "BG",
                    Subdivision{
                        name: "BG",
                        country_alpha2: Alpha2::LR,
                        code: "BG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.829501899999999), longitude: Some(-9.367308399999999), max_latitude: Some(7.430014999999999), min_latitude: Some(6.397581), max_longitude: Some(-9.094152), min_longitude: Some(-10.4769088)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بونغ"), ("be", "Бонг"), ("bg", "Бонг"), ("bn", "বং ক\u{9be}উন\u{9cd}টি"), ("ca", "Bong"), ("ccp", "𑄝\u{11127}\u{11101}"), ("ceb", "Bong County"), ("da", "Bong County"), ("de", "Bong County"), ("el", "Μπονγκ"), ("en", "Bong"), ("es", "Bong"), ("eu", "Bong"), ("fa", "شهرستان بونگ"), ("fi", "Bong"), ("fr", "Comté de Bong"), ("gu", "બો\u{a82}ગ કાઉન\u{acd}ટી"), ("hi", "बो\u{902}ग काउ\u{902}टी"), ("hy", "Բոնգ երկրամաս"), ("id", "Bong County"), ("it", "contea di Bong"), ("ja", "ボン郡"), ("ka", "ბონგის საგრაფო"), ("kn", "ಬೊಂಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "봉 주"), ("lt", "Bongo apskritis"), ("lv", "Bongas grāfiste"), ("mr", "बो\u{902}ग काउ\u{902}टी"), ("ms", "Bong County"), ("nb", "Bong"), ("nl", "Bong"), ("no", "Bong"), ("pl", "Hrabstwo Bong"), ("pt", "Bong"), ("ru", "Бонг"), ("si", "බොන\u{dca}ග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Bong County"), ("ta", "போங\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c3e}ంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบอง"), ("tr", "Bong County"), ("uk", "Бонг"), ("ur", "بونگ کاؤنٹی"), ("vi", "Hạt Bong"), ("zh", "邦县")]),
                        unofficial_name_list: ["Bong"].to_vec(),
                    }
                ),
                (
                    "BM",
                    Subdivision{
                        name: "BM",
                        country_alpha2: Alpha2::LR,
                        code: "BM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.7562926), longitude: Some(-10.8451467), max_latitude: Some(6.970269), min_latitude: Some(6.479747), max_longitude: Some(-10.4870661), min_longitude: Some(-11.0814981)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بومي"), ("be", "Бомі"), ("bg", "Боми"), ("bn", "বমি ক\u{9be}উন\u{9cd}টি"), ("ca", "Bomi"), ("ccp", "𑄝\u{1112e}𑄟\u{11128}"), ("ceb", "Bomi County"), ("da", "Bomi"), ("de", "Bomi County"), ("el", "Μπόμι"), ("en", "Bomi"), ("es", "Bomi"), ("et", "Bomi maakond"), ("eu", "Bomi"), ("fa", "شهرستان بومی"), ("fi", "Bomi"), ("fr", "Comté de Bomi"), ("gl", "Condado de Bomi"), ("gu", "બોમી કાઉન\u{acd}ટી"), ("hi", "बोमी काउ\u{902}टी"), ("hy", "Բոմի երկրամաս"), ("id", "Bomi County"), ("it", "contea di Bomi"), ("ja", "ボミ郡"), ("ka", "ბომის საგრაფო"), ("kn", "ಬೊಮ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "보미 주"), ("lt", "Bomi apskritis"), ("lv", "Bomi grāfiste"), ("mr", "बोमी काउ\u{902}टी"), ("ms", "Bomi County"), ("nb", "Bomi"), ("nl", "Bomi"), ("no", "Bomi"), ("pl", "Hrabstwo Bomi"), ("pt", "Bomi"), ("ru", "Боми"), ("si", "බෝම\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Bomi County"), ("ta", "போமி கவுண\u{bcd}டி"), ("te", "బ\u{c4b}మ\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโบม\u{e35}"), ("tr", "Bomi County"), ("uk", "Бомі"), ("ur", "بومی کاؤنٹی"), ("vi", "Hạt Bomi"), ("zh", "伯米县")]),
                        unofficial_name_list: ["Bomi"].to_vec(),
                    }
                ),
                (
                    "CM",
                    Subdivision{
                        name: "CM",
                        country_alpha2: Alpha2::LR,
                        code: "CM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.0467758), longitude: Some(-11.0711758), max_latitude: Some(7.527846999999999), min_latitude: Some(6.577183), max_longitude: Some(-10.5165701), min_longitude: Some(-11.486138)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غراند كيب ماونت"), ("be", "Гранд-Кейп-Маўнт"), ("bg", "Гранд Кейп Маунт"), ("bn", "গ\u{9cd}র\u{9cd}য\u{9be}ন\u{9cd}ড কেপ ম\u{9be}উন\u{9cd}ট ক\u{9be}উন\u{9cd}টি"), ("ca", "Grand Cape Mount"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄇\u{11133}𑄠𑄛\u{11134} 𑄟𑄅\u{1112a}𑄚\u{11133}𑄑\u{11134}"), ("ceb", "Grand Cape Mount County"), ("da", "Grand Cape Mount County"), ("de", "Grand Cape Mount County"), ("el", "Γκραντ Κέιπ Μάουντ"), ("en", "Grand Cape Mount"), ("es", "Grand Cape Mount"), ("eu", "Grand Cape Mount"), ("fa", "شهرستان گرند کیپ مانت"), ("fi", "Grand Cape Mountin kunta"), ("fr", "Comté de Grand Cape Mount"), ("gu", "ગ\u{acd}રાન\u{acd}ડ ક\u{ac7}પ માઉન\u{acd}ટ કાઉન\u{acd}ટી"), ("hi", "ग\u{94d}र\u{948}\u{902}ड क\u{947}प माउ\u{902}ट काउ\u{902}टी"), ("hy", "Գրանդ Քեյփ-Մաունթ երկրամաս"), ("id", "Grand Cape Mount County"), ("it", "contea di Grand Cape Mount"), ("ja", "グランドケープマウント郡"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd} ಕೇಪ\u{ccd} ಮ\u{ccc}ಂಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "그랜드케이프마운트 주"), ("lt", "Grand Keip Maunto apskritis"), ("lv", "Grandkeipmauntas grāfiste"), ("mr", "ग\u{94d}र\u{901}ड क\u{947}प माउ\u{902}ट काउ\u{902}टी"), ("ms", "Grand Cape Mount County"), ("nb", "Grand Cape Mount"), ("nl", "Grand Cape Mount"), ("no", "Grand Cape Mount"), ("pl", "Hrabstwo Grand Cape Mount"), ("pt", "Grand Cape Mount"), ("ru", "Гранд-Кейп-Маунт"), ("si", "ග\u{dca}\u{200d}රෑන\u{dca}ඩ\u{dca} කේප\u{dca} මව\u{dd4}න\u{dca}ට\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Grand Cape Mount County"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} க\u{bbe}ப\u{bcd}பே மவுண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c4d}ర\u{c3e}ండ\u{c4d} క\u{c47}ప\u{c4d} మ\u{c4c}ంట\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแกรนด\u{e4c}เคปเมานต\u{e4c}"), ("tr", "Grand Cape Mount County"), ("uk", "Гранд-Кейп-Маунт"), ("ur", "گرینڈ کیپ ماؤنٹ کاؤنٹی"), ("vi", "Hạt Grand Cape Mount"), ("zh", "大角山县")]),
                        unofficial_name_list: ["Grand Cape Mount"].to_vec(),
                    }
                ),
                (
                    "GB",
                    Subdivision{
                        name: "GB",
                        country_alpha2: Alpha2::LR,
                        code: "GB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.2308452), longitude: Some(-9.8124935), max_latitude: Some(6.7131769), min_latitude: Some(5.5731877), max_longitude: Some(-9.0244939), min_longitude: Some(-10.3763649)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غراند باسا"), ("be", "Гранд-Баса"), ("bg", "Гранд Баса"), ("bn", "গ\u{9cd}র\u{9be}ন\u{9cd}ড ব\u{9be}সস\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Grand Bassa"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄝𑄥"), ("ceb", "Grand Bassa County"), ("da", "Grand Bassa County"), ("de", "Grand Bassa County"), ("el", "Γκραντ Μπάσσα"), ("en", "Grand Bassa"), ("es", "Grand Bassa"), ("eu", "Grand Bassa"), ("fa", "شهرستان گرند باسا"), ("fi", "Grand Bassan kunta"), ("fr", "Comté de Grand Bassa"), ("gu", "ગ\u{acd}રાન\u{acd}ડ બાસા કાઉન\u{acd}ટી"), ("hi", "ग\u{94d}र\u{948}\u{902}ड बासा काउ\u{902}टी"), ("hy", "Գրանդ Բասսա երկրամաս"), ("id", "Grand Bassa County"), ("it", "contea di Grand Bassa"), ("ja", "グランドバッサ郡"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd} ಬಾಸ\u{ccd}ಸ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "그랜드바사 주"), ("lt", "Grand Basos apskritis"), ("lv", "Grandbasas grāfiste"), ("mr", "ग\u{94d}र\u{901}ड बासा काउ\u{902}टी"), ("ms", "Grand Bassa County"), ("nb", "Grand Bassa"), ("nl", "Grand Bassa"), ("no", "Grand Bassa"), ("pl", "Grand Bassa"), ("pt", "Grand Bassa"), ("ru", "Гранд-Баса"), ("si", "ග\u{dca}\u{200d}රෑන\u{dca}ඩ\u{dca} බස\u{dca}ස\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Grand Bassa County"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} பஸ\u{bcd}ஸ\u{bbe} கவுண\u{bcd}டி"), ("te", "గ\u{c4d}ర\u{c3e}ంండ\u{c4d} బ\u{c3e}స\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแกรนด\u{e4c}แบสซา"), ("tr", "Grand Bassa County"), ("uk", "Гранд-Баса"), ("ur", "گرینڈ باسا کاؤنٹی"), ("vi", "Hạt Grand Bassa"), ("zh", "大巴萨县")]),
                        unofficial_name_list: ["Grand Bassa"].to_vec(),
                    }
                ),
                (
                    "GG",
                    Subdivision{
                        name: "GG",
                        country_alpha2: Alpha2::LR,
                        code: "GG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.9222078), longitude: Some(-8.2212979), max_latitude: Some(6.505594899999999), min_latitude: Some(5.514285), max_longitude: Some(-7.371477899999999), min_longitude: Some(-9.0887029)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غراند غيده"), ("be", "Гранд-Гедэ"), ("bg", "Гранд Гедех"), ("bn", "গ\u{9cd}র\u{9be}ন\u{9cd}ড গ\u{9be}দেহ ক\u{9be}উন\u{9cd}টি"), ("ca", "Grand Gedeh"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄉𑄓𑄬𑄦\u{11134}"), ("ceb", "Grand Gedeh County"), ("da", "Grand Gedeh County"), ("de", "Grand Gedeh County"), ("el", "Γκραντ Γκεντέ"), ("en", "Grand Gedeh"), ("es", "Grand Gedeh"), ("eu", "Grand Gedeh"), ("fa", "شهرستان گرند گدهی"), ("fi", "Grand Gedehin kunta"), ("fr", "Comté de Grand Gedeh"), ("gu", "ગ\u{acd}રાન\u{acd}ડ ગ\u{ac7}ડ\u{ac7}હ કાઉન\u{acd}ટી"), ("hi", "ग\u{94d}र\u{948}\u{902}ड ग\u{947}द\u{947}ह काउ\u{902}टी"), ("hy", "Գրանդ Գեդեհ երկրամաս"), ("id", "Grand Gedeh County"), ("it", "contea di Grand Gedeh"), ("ja", "グランドゲデ郡"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd} ಗ\u{cc6}ಡ\u{cc6}ಹ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "그랜드 게데 주"), ("lt", "Grand Gedeho apskritis"), ("lv", "Grandgedes grāfiste"), ("mr", "ग\u{94d}र\u{901}ड गीदह काउ\u{902}टी"), ("ms", "DaerahGrand Gedeh"), ("nb", "Grand Gedeh"), ("nl", "Grand Gedeh"), ("no", "Grand Gedeh"), ("pl", "Grand Gedeh"), ("pt", "Grand Gedeh"), ("ru", "Гранд-Геде"), ("si", "ග\u{dca}ර\u{dcf}න\u{dca}ඩ\u{dca} ගෙඩේහ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Grand Gedeh County"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} கெதேஹ\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c4d}ర\u{c3e}ండ\u{c4d} గ\u{c46}డ\u{c47} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแกรนด\u{e4c}เกเดห\u{e4c}"), ("tr", "Grand Gedeh County"), ("uk", "Гранд-Геде"), ("ur", "گرینڈ گیدیہ کاؤنٹی"), ("vi", "Hạt Grand Gedeh"), ("zh", "大各德县")]),
                        unofficial_name_list: ["Grand Gedah"].to_vec(),
                    }
                ),
                (
                    "GK",
                    Subdivision{
                        name: "GK",
                        country_alpha2: Alpha2::LR,
                        code: "GK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.7613862), longitude: Some(-8.2212979), max_latitude: Some(5.233399899999999), min_latitude: Some(4.434164), max_longitude: Some(-7.759545999999999), min_longitude: Some(-8.6776371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غراند كرو"), ("be", "Гранд-Кру"), ("bg", "Гранд Кру"), ("bn", "গ\u{9cd}র\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9cd}র\u{9c1} ক\u{9be}উন\u{9cd}টি"), ("ca", "Grand Kru"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄇\u{11133}𑄢\u{1112a}"), ("ceb", "Grand Kru County"), ("da", "Grand Kru County"), ("de", "Grand Kru County"), ("el", "Γκραντ Κρου"), ("en", "Grand Kru"), ("es", "Grand Kru"), ("eu", "Grand Kru"), ("fa", "شهرستان گرند کرو"), ("fi", "Grand Krun kunta"), ("fr", "Comté de Grand Kru"), ("gu", "ગ\u{acd}રાન\u{acd}ડ ક\u{acd}ર\u{ac1} કાઉન\u{acd}ટી"), ("hi", "ग\u{94d}र\u{948}\u{902}ड क\u{94d}र\u{942} काउ\u{902}टी"), ("hy", "Գրանդ Կրու երկրամաս"), ("id", "Grand Kru County"), ("it", "contea di Grand Kru"), ("ja", "グランドクル郡"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccd}ರು ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "그랜드크루 주"), ("lt", "Grand Kru apskritis"), ("lv", "Grandkru grāfiste"), ("mr", "ग\u{94d}र\u{901}ड क\u{94d}र\u{942} काउ\u{902}टी"), ("ms", "Grand Kru County"), ("nb", "Grand Kru"), ("nl", "Grand Kru"), ("no", "Grand Kru"), ("pl", "Hrabstwo Grand Kru"), ("pt", "Grand Kru"), ("ru", "Гранд-Кру"), ("si", "ග\u{dca}\u{200d}රෑන\u{dca}ඩ\u{dca} කෘ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Grand Kru County"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} க\u{bcd}ரூ கவுண\u{bcd}டி"), ("te", "గ\u{c4d}ర\u{c3e}ండ\u{c4d} క\u{c4d}రూ క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแกรนด\u{e4c}คร\u{e39}"), ("tr", "Grand Kru County"), ("uk", "Гранд-Кру"), ("ur", "گرینڈ کرو کاؤنٹی"), ("vi", "Hạt Grand Kru"), ("zh", "大克鲁县")]),
                        unofficial_name_list: ["Grand Kru"].to_vec(),
                    }
                ),
                (
                    "GP",
                    Subdivision{
                        name: "GP",
                        country_alpha2: Alpha2::LR,
                        code: "GP",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غباربولو"), ("be", "Гбарпалу"), ("bg", "Гбарполу"), ("bn", "গ\u{9cd}যব\u{9be}রপল\u{9c1} ক\u{9be}উন\u{9cd}টি"), ("ca", "Gbarpolu"), ("ccp", "𑄝𑄢\u{11134}𑄛\u{1112e}𑄣"), ("ceb", "Gbarpolu County"), ("da", "Gbarpolu County"), ("de", "Gbarpolu County"), ("el", "Γκπαρπόλου"), ("en", "Gbarpolu"), ("es", "Gbarpolu"), ("eu", "Gbarpolu"), ("fa", "شهرستان گبارپولو"), ("fi", "Gbarpolu"), ("fr", "Comté de Gbarpolu"), ("gu", "ગબારપોલ\u{ac1} કાઉન\u{acd}ટી"), ("hi", "बारपोल\u{942} काउ\u{902}टी"), ("hy", "Գբարպոլու երկրամաս"), ("id", "Gbarpolu County"), ("it", "contea di Gbarpolu"), ("ja", "バルポル郡"), ("kn", "ಗರ\u{ccd}ಬೊಲು ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "바르폴루 주"), ("lt", "Gbarpolu apskritis"), ("lv", "Gbarpolu grāfiste"), ("mr", "गबाररपोल\u{941} काउ\u{902}टी"), ("ms", "Gbarpolu County"), ("nb", "Gbarpolu"), ("nl", "Gbarpolu"), ("no", "Gbarpolu"), ("pl", "Hrabstwo Gbarpolu"), ("pt", "Gbarpolu"), ("ru", "Гбарполу"), ("si", "ජ\u{dd2}බ\u{dcf}පොල\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Gbarpolu County"), ("ta", "க\u{bcd}ப\u{bbe}ர\u{bcd}போலு கவுண\u{bcd}டி"), ("te", "గబ\u{c3e}ర\u{c4d}ప\u{c4b}లు క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบาร\u{e4c}โพล\u{e39}"), ("tr", "Gbarpolu County"), ("uk", "Гбарполу"), ("ur", "گبارپولو کاؤنٹی"), ("vi", "Hạt Gbarpolu"), ("zh", "巴波卢县")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::LR,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.191118399999999), longitude: Some(-9.7232673), max_latitude: Some(8.551791099999999), min_latitude: Some(7.178834999999999), max_longitude: Some(-9.343918), min_longitude: Some(-10.606035)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوفا"), ("be", "Лофа"), ("bg", "Лофа"), ("bn", "লোফ\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Lofa"), ("ccp", "𑄣\u{1112e}𑄜"), ("ceb", "Lofa County"), ("da", "Lofa County"), ("de", "Lofa County"), ("el", "Λόφα"), ("en", "Lofa"), ("es", "Lofa"), ("eu", "Lofa"), ("fa", "شهرستان لوفا"), ("fi", "Lofa"), ("fr", "Comté de Lofa"), ("gu", "લોફા કાઉન\u{acd}ટી"), ("hi", "लोफ\u{93c}ा काउ\u{902}टी"), ("hy", "Լոֆա երկրամաս"), ("id", "Lofa County"), ("it", "contea di Lofa"), ("ja", "ロファ郡"), ("kn", "ಲೋಫಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "로파 주"), ("lt", "Lofos apskritis"), ("lv", "Lofas grāfiste"), ("mr", "लोफा काउ\u{902}टी"), ("ms", "Lofa County"), ("nb", "Lofa"), ("nl", "Lofa"), ("no", "Lofa"), ("pl", "Hrabstwo Lofa"), ("pt", "Lofa"), ("ru", "Лофа"), ("si", "ලොෆ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Lofa County"), ("ta", "லோப\u{bbe} கவுண\u{bcd}டி"), ("te", "ల\u{c4b}ఫ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโลฟา"), ("tr", "Lofa County"), ("uk", "Лофа"), ("ur", "لوفا کاؤنٹی"), ("vi", "Hạt Lofa"), ("zh", "洛法县")]),
                        unofficial_name_list: ["Lofa"].to_vec(),
                    }
                ),
                (
                    "MG",
                    Subdivision{
                        name: "MG",
                        country_alpha2: Alpha2::LR,
                        code: "MG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.5151875), longitude: Some(-10.3048897), max_latitude: Some(6.8118709), min_latitude: Some(6.138984300000001), max_longitude: Some(-9.722209), min_longitude: Some(-10.647893)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مارجيبي"), ("be", "Маргібі"), ("bg", "Маргиби"), ("bn", "ম\u{9be}রগিবি ক\u{9be}উন\u{9cd}টি"), ("ca", "Margibi"), ("ccp", "𑄟𑄢\u{11134}𑄎\u{11128}𑄝\u{11128}"), ("ceb", "Margibi County"), ("da", "Margibi County"), ("de", "Margibi County"), ("el", "Μάργκιμπι"), ("en", "Margibi"), ("es", "Margibi"), ("eu", "Margibi"), ("fa", "شهرستان مارجیبای"), ("fi", "Margibin kunta"), ("fr", "Comté de Margibi"), ("gu", "માર\u{acd}ગિબી કાઉન\u{acd}ટી"), ("hi", "मारजीबी काउ\u{902}टी"), ("hy", "Մարգիբի երկրամաս"), ("id", "Margibi County"), ("it", "contea di Margibi"), ("ja", "マージビ郡"), ("kn", "ಮಾರ\u{ccd}ಗ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "마르지비 주"), ("lt", "Margibio apskritis"), ("lv", "Margibi grāfiste"), ("mr", "मार\u{94d}गबि काउ\u{902}टी"), ("ms", "Margibi County"), ("nb", "Margibi"), ("nl", "Margibi"), ("no", "Margibi"), ("pl", "Hrabstwo Margibi"), ("pt", "Margibi"), ("ru", "Маргиби"), ("si", "මර\u{dca}ග\u{dd2}බ\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Margibi County"), ("ta", "ம\u{bbe}ர\u{bcd}ஜிபி கவுண\u{bcd}டி"), ("te", "మ\u{c3e}ర\u{c4d}జ\u{c3f}బ\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลมาร\u{e4c}ก\u{e35}บ\u{e35}"), ("tr", "Margibi County"), ("uk", "Маргібі"), ("ur", "مارگیبی کاؤنٹی"), ("vi", "Hạt Margibi"), ("zh", "马及比县")]),
                        unofficial_name_list: ["Margibi"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::LR,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.5525815), longitude: Some(-10.5296115), max_latitude: Some(6.8513641), min_latitude: Some(6.220479999999999), max_longitude: Some(-10.3692761), min_longitude: Some(-10.9532799)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مونتسرادو"), ("be", "Мантсерада"), ("bg", "Монтсерадо"), ("bn", "মন\u{9cd}টসেরর\u{9be}ডো ক\u{9be}উন\u{9cd}টি"), ("ca", "Montserrado"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄌\u{11133}𑄑𑄢\u{11134}𑄢𑄓\u{1112e}"), ("ceb", "Montserrado County"), ("da", "Montserrado County"), ("de", "Montserrado County"), ("el", "Μοντσεράντο"), ("en", "Montserrado"), ("es", "Condado de Montserrado"), ("eu", "Montserrado"), ("fa", "شهرستان مانتسرادو"), ("fi", "Montserrado"), ("fr", "Comté de Montserrado"), ("gu", "મોન\u{acd}ટરાડો કાઉન\u{acd}ટી"), ("hi", "मो\u{902}टस\u{947}र\u{948}डो काउ\u{902}टी"), ("hy", "Մոնտսերադո երկրամաս"), ("id", "Montserrado County"), ("it", "contea di Montserrado"), ("ja", "モンセラード郡"), ("kn", "ಮೋಂಟ\u{ccd}ಸ\u{cc6}ರಾಡೋ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "몽세라도 주"), ("lt", "Montserado apskritis"), ("lv", "Montserado grāfiste"), ("mr", "मॉन\u{94d}स\u{94d}ट\u{947}राडो काउ\u{902}टी"), ("ms", "Montserrado County"), ("nb", "Montserrado"), ("nl", "Montserrado"), ("no", "Montserrado"), ("pl", "Hrabstwo Montserrado"), ("pt", "Montserrado"), ("ru", "Монтсеррадо"), ("si", "මොන\u{dca}ට\u{dca}සෙර\u{dcf}ඩෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Montserrado County"), ("ta", "மொன\u{bcd}டசேர\u{bcd}றடோ கவுண\u{bcd}டி"), ("te", "మ\u{c3e}ంట\u{c4d}స\u{c46}ర\u{c3e}డ\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลมอนต\u{e4c}เซอร\u{e4c}ราโด"), ("tr", "Montserrado County"), ("uk", "Монтсеррадо"), ("ur", "مونٹسیراڈا کاؤنٹی"), ("vi", "Montserrado"), ("zh", "蒙特塞拉多县")]),
                        unofficial_name_list: ["Montserrado"].to_vec(),
                    }
                ),
                (
                    "MY",
                    Subdivision{
                        name: "MY",
                        country_alpha2: Alpha2::LR,
                        code: "MY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(4.7258878), longitude: Some(-7.741670300000001), max_latitude: Some(5.130764999999999), min_latitude: Some(4.3541994), max_longitude: Some(-7.5213661), min_longitude: Some(-8.0795251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماريلاند"), ("be", "Акруга Мэрыленд"), ("bg", "Мериленд"), ("bn", "ম\u{9cd}য\u{9be}রিল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("ca", "Maryland"), ("ccp", "𑄟𑄬𑄢\u{11128}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Maryland County"), ("da", "Maryland"), ("de", "Maryland County"), ("el", "Μέριλαντ"), ("en", "Maryland"), ("es", "Condado de Maryland"), ("eu", "Maryland"), ("fa", "شهرستان مریلند"), ("fi", "Maryland"), ("fr", "Comté de Maryland"), ("gu", "મ\u{ac7}રીલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("hi", "म\u{947}रील\u{948}\u{902}ड काउ\u{902}टी"), ("hy", "Մերիլենդ երկրամաս"), ("id", "Maryland County"), ("it", "contea di Maryland"), ("ja", "メリーランド郡"), ("kn", "ಮೇರ\u{cbf}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "메릴랜드 주"), ("lt", "Merilando apskritis"), ("lv", "Merilendas grāfiste"), ("mr", "म\u{947}रील\u{901}ड काउ\u{902}टी"), ("ms", "Maryland County"), ("nb", "Maryland"), ("nl", "Maryland"), ("no", "Maryland"), ("pl", "Hrabstwo Maryland"), ("pt", "Maryland"), ("ru", "Мэриленд"), ("si", "මේර\u{dd2}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Maryland County"), ("ta", "மரில\u{bbe}ண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "మ\u{c47}ర\u{c40}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแมร\u{e34}แลนด\u{e4c}"), ("tr", "Maryland County"), ("uk", "Меріленд"), ("ur", "میری لینڈ کاؤنٹی، لائبیریا"), ("vi", "Hạt Maryland"), ("zh", "馬里蘭縣")]),
                        unofficial_name_list: ["Maryland"].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::LR,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.8427612), longitude: Some(-8.6600586), max_latitude: Some(7.696163100000001), min_latitude: Some(5.823793999999999), max_longitude: Some(-8.27361), min_longitude: Some(-9.2169951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نيمبا"), ("be", "Німба"), ("bg", "Нимба"), ("bn", "নিম\u{9cd}ব\u{9be}"), ("ca", "Nimba"), ("ccp", "𑄚𑄟\u{11134}𑄝"), ("ceb", "Nimba County"), ("da", "Nimba"), ("de", "Nimba County"), ("el", "Νίμπα"), ("en", "Nimba"), ("es", "Nimba"), ("eu", "Nimba"), ("fa", "شهرستان نیمبا"), ("fi", "Nimba"), ("fr", "Comté de Nimba"), ("gu", "નિમ\u{acd}બા"), ("hi", "निम\u{94d}बा"), ("hy", "Նիմբա երկրամաս"), ("id", "Nimba"), ("it", "contea di Nimba"), ("ja", "ニンバ郡"), ("kn", "ನ\u{cbf}ಂಬಾ"), ("ko", "님바 주"), ("lt", "Nimbos apskritis"), ("lv", "Nimba"), ("mr", "नि\u{902}बा"), ("ms", "Nimba"), ("nb", "Nimba"), ("nl", "Nimba"), ("no", "Nimba"), ("pl", "Hrabstwo Nimba"), ("pt", "Nimba"), ("ru", "Нимба"), ("si", "න\u{dd2}ම\u{dca}බ\u{dcf}"), ("sv", "Nimba County"), ("ta", "நிம\u{bcd}ப\u{bbe}"), ("te", "న\u{c3f}ంబ\u{c3e}"), ("th", "เทศมณฑลน\u{e34}มบา"), ("tr", "Nimba"), ("uk", "Німба"), ("ur", "نمبا کاؤنٹی"), ("vi", "Nimba"), ("zh", "寧巴縣")]),
                        unofficial_name_list: ["Nimba"].to_vec(),
                    }
                ),
                (
                    "RG",
                    Subdivision{
                        name: "RG",
                        country_alpha2: Alpha2::LR,
                        code: "RG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريفر جي"), ("be", "Рывер-Гі"), ("bg", "Ривър Джий"), ("bn", "রিভ\u{9be}র গি ক\u{9be}উন\u{9cd}টি"), ("ca", "River Gee"), ("ccp", "𑄢\u{11128}𑄞𑄢\u{11134} 𑄉\u{11128}"), ("ceb", "River Gee County"), ("da", "River Gee County"), ("de", "River Gee County"), ("el", "Ρίβερ Γκι"), ("en", "River Gee"), ("es", "River Gee"), ("eu", "River Gee"), ("fa", "شهرستان ریور جی"), ("fi", "River Geen kunta"), ("fr", "Comté de River Gee"), ("gu", "નદી ગી કાઉન\u{acd}ટી"), ("hi", "रिवर जी काउ\u{902}टी"), ("hy", "Ռիվր Գի երկրամաս"), ("id", "River Gee County"), ("it", "contea di River Gee"), ("ja", "リバージー郡"), ("kn", "ರ\u{cbf}ವರ\u{ccd} ಗೀ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "리버지 주"), ("lt", "River Gi apskritis"), ("lv", "Rivergī grāfiste"), ("mr", "रिव\u{94d}हर जी काउ\u{902}टी"), ("ms", "River Gee County"), ("nb", "River Gee"), ("nl", "River Gee"), ("no", "River Gee"), ("pl", "Hrabstwo River Gee"), ("pt", "River Gee"), ("ru", "Ривер-Ги"), ("si", "ර\u{dd2}වර\u{dca} ග\u{dd3} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "River Gee County"), ("ta", "ரிவேர\u{bcd} க\u{bc0} கவுண\u{bcd}டி"), ("te", "ర\u{c3f}వర\u{c4d} జ\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลร\u{e34}เวอร\u{e4c}ก\u{e35}"), ("tr", "River Gee County"), ("uk", "Рівер-Гі"), ("ur", "دریائے جی کاؤنٹی"), ("vi", "Hạt River Gee"), ("zh", "吉河县")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::LR,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.902532799999999), longitude: Some(-9.456155), max_latitude: Some(6.378757), min_latitude: Some(5.269763), max_longitude: Some(-9.058736), min_longitude: Some(-9.733108999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريفرسيس"), ("be", "Акруга Рывер-Сес"), ("bg", "Ривър Сес"), ("bn", "রিভ\u{9be}রকেস ক\u{9be}উন\u{9cd}টি"), ("ccp", "𑄢\u{11128}𑄞𑄢\u{11134}𑄥𑄬𑄌\u{11134}"), ("ceb", "River Cess County"), ("da", "Rivercess County"), ("de", "River Cess County"), ("el", "Ρίβερκες"), ("en", "Rivercess"), ("es", "Condado de Rivercess"), ("eu", "River Cess"), ("fa", "شهرستان ریورسس"), ("fi", "Rivercessin kunta"), ("fr", "Comté de River Cess"), ("gu", "રિવરસ\u{ac7}સ કાઉન\u{acd}ટી"), ("hi", "रिवरस\u{947}स काउ\u{902}टी"), ("hy", "Ռիվրցես երկրամաս"), ("id", "Rivercess County"), ("it", "contea di River Cess"), ("ja", "リバーセス郡"), ("kn", "ರ\u{cbf}ವರ\u{ccd}ಸಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "리버세스 주"), ("lt", "River Seso apskritis"), ("lv", "Riversesas grāfiste"), ("mr", "रिवर\u{947}ट काउ\u{902}टी"), ("ms", "Rivercess County"), ("nb", "Rivercess"), ("nl", "River Cess"), ("no", "Rivercess"), ("pl", "Hrabstwo River Cess"), ("pt", "River Cess"), ("ru", "Ривер-Сесс"), ("si", "ර\u{dd2}වර\u{dca}සෙස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "River Cess County"), ("ta", "ரிவேர\u{bcd}ஸ\u{bcd}ஸ\u{bcd}ஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "ర\u{c3f}వర\u{c4d}స\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลร\u{e34}เวอร\u{e4c}เซสส\u{e4c}"), ("tr", "Rivercess County"), ("uk", "Рівер-Сесс (графство)"), ("ur", "ریویرکیس کاؤنٹی"), ("vi", "Hạt Rivercess"), ("zh", "里弗塞斯縣")]),
                        unofficial_name_list: ["Rivercess"].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::LR,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.49871), longitude: Some(-8.6600586), max_latitude: Some(5.844906), min_latitude: Some(4.804354), max_longitude: Some(-8.2341789), min_longitude: Some(-9.4184888)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سينو"), ("be", "Сіноэ"), ("bg", "Синое"), ("bn", "সিনো ক\u{9be}উন\u{9cd}টি"), ("ca", "Sinoe"), ("ccp", "𑄥\u{11128}𑄚\u{1112e}𑄠\u{11128}"), ("ceb", "Sinoe County"), ("da", "Sinoe County"), ("de", "Sinoe County"), ("el", "Σινόε"), ("en", "Sinoe"), ("es", "Sinoe"), ("eu", "Sinoe"), ("fa", "شهرستان ساینو"), ("fi", "Sinoe"), ("fr", "Comté de Sinoe"), ("gu", "સીનોએ કાઉન\u{acd}ટી"), ("hi", "सीनो काउ\u{902}टी"), ("hy", "Սինոե երկրամաս"), ("id", "Sinoe County"), ("it", "contea di Sinoe"), ("ja", "シノエ郡"), ("kn", "ಸ\u{cbf}ನೊ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "시노에 주"), ("lt", "Sinoės apskritis"), ("lv", "Sinoe grāfiste"), ("mr", "स\u{941}नी काउ\u{902}टी"), ("ms", "Sinoe County"), ("nb", "Sinoe"), ("nl", "Sinoe"), ("no", "Sinoe"), ("pl", "Hrabstwo Sinoe"), ("pt", "Sinoe"), ("ru", "Синоэ"), ("si", "ස\u{dd2}නොය\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Sinoe County"), ("ta", "சினோ கவுண\u{bcd}டி"), ("te", "స\u{c3f}న\u{c4b}య\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลไซโน"), ("tr", "Sinoe County"), ("uk", "Сіное"), ("ur", "سینؤ کاؤنٹی"), ("vi", "Hạt Sinoe"), ("zh", "錫諾縣")]),
                        unofficial_name_list: ["Sinoe"].to_vec(),
                    }
                ),
                (
                    "X1~",
                    Subdivision{
                        name: "X1~",
                        country_alpha2: Alpha2::LR,
                        code: "X1~",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.495263700000001), longitude: Some(-10.0807298), max_latitude: Some(7.998692), min_latitude: Some(6.829072), max_longitude: Some(-9.636509), min_longitude: Some(-10.9533111)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Gbarpolu")]),
                        unofficial_name_list: ["Gbarpolu"].to_vec(),
                    }
                ),
                (
                    "X2~",
                    Subdivision{
                        name: "X2~",
                        country_alpha2: Alpha2::LR,
                        code: "X2~",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(5.2604894), longitude: Some(-7.872159999999999), max_latitude: Some(5.6097129), min_latitude: Some(4.8041949), max_longitude: Some(-7.365113), min_longitude: Some(-8.328244999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "River Gee")]),
                        unofficial_name_list: ["River Gee"].to_vec(),
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
#[cfg(feature = "lr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::LR,
        alpha3: Alpha3::LBR,
        address_format: None,
        continent: Continent::Africa,
        country_code: 231,
        currency_code: "LRD",
        gec: Some(GEC::LI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("LBR"),
        iso_long_name: "The Republic of Liberia",
        iso_short_name: "Liberia",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [6, 7, 8].to_vec(),
        national_prefix: "22",
        nationality: Some("Liberian"),
        number: "430",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAfrica),
        un_locode: "LR",
        unofficial_name_list: ["Liberia", "リベリア"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Liberia"),
            ("af", "Liberië"),
            ("ak", "Liberia"),
            ("am", "ላይቤሪያ"),
            ("an", "Liberia"),
            ("ar", "ليبيريا"),
            ("as", "ল\u{9be}ইবেৰিয়\u{9be}"),
            ("ay", "Liberia"),
            ("az", "Liberiya"),
            ("ba", "Liberia"),
            ("be", "Ліберыя"),
            ("bg", "Либерия"),
            ("bi", "Liberia"),
            ("bn", "ল\u{9be}ইবেরিয়\u{9be}"),
            ("bn_IN", "ল\u{9be}ইবেরিয়\u{9be}"),
            ("br", "Liberia"),
            ("bs", "Liberija"),
            ("ca", "Libèria"),
            ("ce", "Либери"),
            ("ch", "Liberia"),
            ("cs", "Libérie"),
            ("cv", "Либери"),
            ("cy", "Liberia"),
            ("da", "Liberia"),
            ("de", "Liberia"),
            ("dv", "ލ\u{7a6}އ\u{7a8}ބ\u{7a9}ރ\u{7a8}އ\u{7a7}"),
            ("dz", "ལ\u{f72}་བ\u{f72}་ར\u{f72}་ཡ།"),
            ("ee", "Liberia"),
            ("el", "Λιβερία"),
            ("en", "Liberia"),
            ("eo", "Liberio"),
            ("es", "Liberia"),
            ("et", "Libeeria"),
            ("eu", "Liberia"),
            ("fa", "لیبریا"),
            ("ff", "Labiriyaa"),
            ("fi", "Liberia"),
            ("fo", "Liberia"),
            ("fr", "Libéria"),
            ("fy", "Libearia"),
            ("ga", "An Libéir"),
            ("gl", "Liberia"),
            ("gn", "Liberia"),
            ("gu", "લાઇબ\u{ac7}રિયા"),
            ("gv", "Yn Laibeer"),
            ("ha", "Liberia"),
            ("he", "ליבריה"),
            ("hi", "लाइब\u{947}रिया"),
            ("hr", "Liberija"),
            ("ht", "Liberya"),
            ("hu", "Libéria"),
            ("hy", "Լիբերիա"),
            ("ia", "Liberia"),
            ("id", "Liberia"),
            ("io", "Liberia"),
            ("is", "Líbería"),
            ("it", "Liberia"),
            ("iu", "Liberia"),
            ("ja", "リベリア"),
            ("ka", "ლიბერია"),
            ("ki", "Liberia"),
            ("kk", "Либерия"),
            ("kl", "Liberia"),
            ("km", "ល\u{17b8}បេរ\u{17b8}យ\u{17c9}ា"),
            ("kn", "ಲೈಬೀರ\u{cbf}ಯಾ"),
            ("ko", "라이베리아"),
            ("ku", "Lîberya"),
            ("kv", "Liberia"),
            ("kw", "Liberi"),
            ("ky", "Либерия"),
            ("lo", "Liberia"),
            ("lt", "Liberija"),
            ("lv", "Libērija"),
            ("mi", "Liberia"),
            ("mk", "Либерија"),
            ("ml", "ലൈബീരിയ"),
            ("mn", "Liberia"),
            ("mr", "लायब\u{947}रिया"),
            ("ms", "Liberia"),
            ("mt", "Liberja"),
            (
                "my",
                "လ\u{102d}\u{102f}က\u{103a}ဘေးရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Liberia"),
            ("nb", "Liberia"),
            ("ne", "लिब\u{947}रीया"),
            ("nl", "Liberia"),
            ("nn", "Liberia"),
            ("nv", "Liberia"),
            ("oc", "Libèria"),
            ("or", "ଲ\u{b3e}ଇବେର\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਲੀਬਾਰੀਆ"),
            ("pi", "लायबीरिया"),
            ("pl", "Liberia"),
            ("ps", "لایبریا"),
            ("pt", "Libéria"),
            ("pt_BR", "Libéria"),
            ("ro", "Liberia"),
            ("ru", "Либерия"),
            ("rw", "Liberiya"),
            ("sc", "Libèria"),
            ("sd", "Liberia"),
            ("si", "ල\u{dd2}ය\u{dd2}බ\u{dd3}ර\u{dd2}ය\u{dcf}ව"),
            ("sk", "Libéria"),
            ("sl", "Liberija"),
            ("so", "Laybeeriya"),
            ("sq", "Liberi"),
            ("sr", "Либерија"),
            ("sv", "Liberia"),
            ("sw", "Liberia"),
            ("ta", "லைப\u{bc0}ரிய\u{bbe}"),
            ("te", "ల\u{c48}బ\u{c40}ర\u{c3f}య\u{c3e}"),
            ("tg", "Либерия"),
            ("th", "ไลบ\u{e35}เร\u{e35}ย"),
            ("ti", "Liberia"),
            ("tk", "Liberiýa"),
            ("tl", "Liberia"),
            ("tr", "Liberya"),
            ("tt", "Либериа"),
            ("ug", "لىبېرىيە"),
            ("uk", "Ліберія"),
            ("ur", "لائبیریا"),
            ("uz", "Liberiya"),
            ("ve", "Liberia"),
            ("vi", "Li-bê-ri-a"),
            ("wa", "Liberia"),
            ("wo", "Libeeria"),
            ("xh", "Liberia"),
            ("yo", "Làìbéríà"),
            ("zh_CN", "利比里亚"),
            ("zh_HK", "利比里亞"),
            ("zh_TW", "賴比瑞亞"),
            ("zu", "ILiberia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
