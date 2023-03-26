// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Denmark

#[cfg(all(feature = "dk", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::DK;
    pub const ALPHA3: Alpha3 = Alpha3::DNK;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 45;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::DKK;
    pub const GEC: Option<GEC> = Some(GEC::DA);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::DEN);
    pub const ISO_SHORT_NAME: &str = "Denmark";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Denmark";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["da"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["da"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Danish");
    pub const NUMBER: &str = "208";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "DK";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Denmark",
        "Dänemark",
        "Danemark",
        "Dinamarca",
        "デンマーク",
        "Denemarken",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Denmark"),
        ("af", "Denemarke"),
        ("ak", "Denmark"),
        ("am", "፤ንሢሴጤ"),
        ("an", "Denmark"),
        ("ar", "الد\u{651}نمارك"),
        ("as", "ডেনম\u{9be}ৰ\u{9cd}ক"),
        ("ay", "Denmark"),
        ("az", "Danimarka"),
        ("ba", "Denmark"),
        ("be", "Данія"),
        ("bg", "Дания"),
        ("bi", "Denmark"),
        ("bn", "ডেনম\u{9be}র\u{9cd}ক"),
        ("bn_IN", "ডেনম\u{9be}র\u{9cd}ক"),
        ("br", "Danmark"),
        ("bs", "Denmark"),
        ("ca", "Dinamarca"),
        ("ce", "Дани"),
        ("ch", "Denmark"),
        ("cs", "Dánsko"),
        ("cv", "Дани"),
        ("cy", "Denmarc"),
        ("da", "Danmark"),
        ("de", "Dänemark"),
        ("dv", "ޑ\u{7ac}ނ\u{7b0}މ\u{7a7}ކ\u{7aa}"),
        ("dz", "ཌ\u{f7a}ན་མ\u{f71}རཀ།"),
        ("ee", "Denmark"),
        ("el", "Δανία"),
        ("en", "Denmark"),
        ("eo", "Danio"),
        ("es", "Dinamarca"),
        ("et", "Taani"),
        ("eu", "Danimarka"),
        ("fa", "دانمارک"),
        ("ff", "Danemark"),
        ("fi", "Tanska"),
        ("fo", "Danmørk"),
        ("fr", "Danemark"),
        ("fy", "Denemark"),
        ("ga", "An Danmhairg"),
        ("gl", "Dinamarca"),
        ("gn", "Denmark"),
        ("gu", "ડ\u{ac7}નમાર\u{acd}ક"),
        ("gv", "Yn Danvarg"),
        ("ha", "Denmark"),
        ("he", "דנמרק"),
        ("hi", "ड\u{947}नमार\u{94d}क"),
        ("hr", "Danska"),
        ("ht", "Danmak"),
        ("hu", "Dánia"),
        ("hy", "Դանիա"),
        ("ia", "Danmark"),
        ("id", "Denmark"),
        ("io", "Dania"),
        ("is", "Danmörk"),
        ("it", "Danimarca"),
        ("iu", "Denmark"),
        ("ja", "デンマーク"),
        ("ka", "დანია"),
        ("ki", "Denmark"),
        ("kk", "Дания"),
        ("kl", "Denmark"),
        ("km", "ដាណ\u{17ba}ម\u{17c9}ាក"),
        ("kn", "ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd}"),
        ("ko", "덴마크"),
        ("ku", "Danîmarka"),
        ("kv", "Дания"),
        ("kw", "Danmark"),
        ("ky", "Дания"),
        ("lo", "ປະເທດດານມາກ"),
        ("lt", "Danija"),
        ("lv", "Dānija"),
        ("mi", "Tenemāka"),
        ("mk", "Данска"),
        ("ml", "ഡെന\u{d4d}മ\u{d3e}ര\u{d4d}\u{200d}ക\u{d4d}ക\u{d4d}"),
        ("mn", "Дани"),
        ("mr", "ड\u{947}न\u{94d}मार\u{94d}क"),
        ("ms", "Denmark"),
        ("mt", "Danimarka"),
        (
            "my",
            "ဒ\u{102d}န\u{103a}းမတ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Denemark"),
        ("nb", "Danmark"),
        ("ne", "ड\u{947}नमार\u{94d}क"),
        ("nl", "Denemarken"),
        ("nn", "Danmark"),
        ("nv", "Denmark"),
        ("oc", "Danemarc"),
        ("or", "ଡେନମ\u{b3e}ର\u{b4d}କ"),
        ("pa", "ਡ\u{a48}ਨਮਾਰਕ"),
        ("pi", "ड\u{947}नमार\u{94d}क"),
        ("pl", "Dania"),
        ("ps", "ډنمارک"),
        ("pt", "Dinamarca"),
        ("pt_BR", "Dinamarca"),
        ("ro", "Danemarca"),
        ("ru", "Дания"),
        ("rw", "Danimarike"),
        ("sc", "Danimarca"),
        ("sd", "Denmark"),
        ("si", "ඩෙන\u{dca}ම\u{dcf}කය"),
        ("sk", "Dánsko"),
        ("sl", "Danska"),
        ("so", "Danmaark"),
        ("sq", "Danimarkë"),
        ("sr", "Данска"),
        ("sv", "Danmark"),
        ("sw", "Denmark"),
        ("ta", "டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd}"),
        ("te", "డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d}"),
        ("tg", "Дания"),
        ("th", "เดนมาร\u{e4c}ก"),
        ("ti", "ዴንማርክ"),
        ("tk", "Daniýa"),
        ("tl", "Denmark"),
        ("tr", "Danimarka"),
        ("tt", "Даниа"),
        ("ug", "دانىيە"),
        ("uk", "Данія"),
        ("ur", "ڈنمارک"),
        ("uz", "Daniya"),
        ("ve", "Denmark"),
        ("vi", "Đan Mạch"),
        ("wa", "Daenmåtche"),
        ("wo", "Danmaark"),
        ("xh", "Denmark"),
        ("yo", "Dẹ\u{301}nmárkì"),
        ("zh_CN", "丹麦"),
        ("zh_HK", "丹麥"),
        ("zh_TW", "丹麥"),
        ("zu", "IDenimaki"),
    ];
    #[cfg(all(feature = "dk", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 56.26392;
        pub const LONGITUDE: f64 = 9.501785;
        pub const MAX_LATITUDE: f64 = 58.02846;
        pub const MAX_LONGITUDE: f64 = 15.2298;
        pub const MIN_LATITUDE: f64 = 54.4317001;
        pub const MIN_LONGITUDE: f64 = 7.855200099999999;
        pub const NORTHEAST_LATITUDE: f64 = 58.02846;
        pub const NORTHEAST_LONGITUDE: f64 = 15.2298;
        pub const SOUTHWEST_LATITUDE: f64 = 54.4317001;
        pub const SOUTHWEST_LONGITUDE: f64 = 7.855200099999999;
    }
}
#[cfg(all(feature = "dk", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 56.26392,
            longitude: 9.501785,
            max_latitude: 58.02846,
            max_longitude: 15.2298,
            min_latitude: 54.4317001,
            min_longitude: 7.855200099999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 58.02846,
                    longitude: 15.2298,
                },
                southwest: CountryGeoBound {
                    latitude: 54.4317001,
                    longitude: 7.855200099999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "dk", feature = "subdivisions"))]
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
                    "81",
                    Subdivision{
                        name: "81",
                        country_alpha2: Alpha2::DK,
                        code: "81",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.8307416), longitude: Some(9.4930528), max_latitude: Some(57.7518131), min_latitude: Some(56.550334), max_longitude: Some(11.200088), min_longitude: Some(8.2120049)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نورييلاند"), ("be", "Вобласць Паўночная Ютландыя"), ("bg", "Северна Ютландия"), ("bn", "নর\u{9cd}থ ডেনম\u{9be}র\u{9cd}ক অঞ\u{9cd}চল"), ("bs", "Sjeverna Danska"), ("ca", "Regió de Nordjylland"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄓𑄬𑄚\u{11134}𑄟𑄢\u{11134}𑄇\u{11134}"), ("ceb", "North Denmark Region"), ("cs", "Nordjylland"), ("da", "Region Nordjylland"), ("de", "Region Nordjylland"), ("el", "Περιφέρεια Βόρειας Δανίας"), ("en", "Northern Denmark"), ("es", "Jutlandia Septentrional"), ("et", "Põhja-Jüütimaa piirkond"), ("eu", "Ipar Jutlandia"), ("fa", "نوردیولند"), ("fi", "Pohjois-Jyllanti"), ("fr", "Jutland-du-Nord"), ("gu", "ઉત\u{acd}તર ડ\u{ac7}નમાર\u{acd}ક પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तरी ड\u{947}नमार\u{94d}क क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Sjeverni Jutland"), ("hu", "Nordjylland régió"), ("hy", "Հյուսիսային Դանիա տարածաշրջան"), ("id", "Region Nordjylland"), ("it", "Jutland settentrionale"), ("ja", "北ユラン地域"), ("ka", "ჩრდილოეთ იუტლანდიის რეგიონი"), ("kk", "Солтүстік Ютландия (облыс)"), ("kn", "ಉತ\u{ccd}ತರ ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "북윌란 지역"), ("lt", "Šiaurės Jutlandijos regionas"), ("lv", "Ziemeļjitlandes reģions"), ("mk", "Северна Данска"), ("mr", "उत\u{94d}तर ड\u{947}न\u{94d}मार\u{94d}क प\u{94d}रद\u{947}श"), ("ms", "Region Nordjylland"), ("nb", "Region Nordjylland"), ("nl", "Noord-Jutland"), ("no", "Region Nordjylland"), ("pl", "Jutlandia Północna"), ("pt", "Jutlândia do Norte"), ("ro", "Regiunea Nordjylland"), ("ru", "Северная Ютландия"), ("si", "උත\u{dd4}ර\u{dd4} ඩෙන\u{dca}ම\u{dcf}ර\u{dca}ක\u{dca} කල\u{dcf}පය"), ("sk", "Severné Jutsko (administratívny región)"), ("sr", "Северна Данска"), ("sr_Latn", "Severna Danska"), ("sv", "Region Nordjylland"), ("ta", "வடக\u{bcd}கு டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd} பகுதி"), ("te", "ఉత\u{c4d}తర డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตการปกครองเดนมาร\u{e4c}กเหน\u{e37}อ"), ("tr", "Kuzey Danimarka Bölgesi"), ("uk", "Північна Ютландія"), ("ur", "شمالی ڈنمارک علاقہ"), ("vi", "Bắc Jutland"), ("zh", "北日德兰大区")]),
                        unofficial_name_list: ["North Denmark", "North Denmark Region", "North Jutland"].to_vec(),
                    }
                ),
                (
                    "82",
                    Subdivision{
                        name: "82",
                        country_alpha2: Alpha2::DK,
                        code: "82",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.302139), longitude: Some(9.3027769), max_latitude: Some(56.846539), min_latitude: Some(55.668859), max_longitude: Some(11.6613), min_longitude: Some(8.0976872)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ميديولند"), ("be", "Цэнтральная Ютландыя"), ("bg", "Централна Ютландия"), ("bn", "কেন\u{9cd}দ\u{9cd}রিয\u{9bc} ডেনম\u{9be}র\u{9cd}ক অঞ\u{9cd}চল"), ("bs", "Centralna Danska"), ("ca", "Regió de Midtjylland"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄓𑄬𑄚\u{11134}𑄟𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Region Midtjylland"), ("cs", "Midtjylland"), ("da", "Region Midtjylland"), ("de", "Region Midtjylland"), ("el", "Περιφέρεια Κεντρικής Δανίας"), ("en", "Central Denmark"), ("es", "Jutlandia Central"), ("et", "Kesk-Jüütimaa"), ("eu", "Erdialdeko Jutlandia"), ("fa", "میدیولند"), ("fi", "Keski-Jyllanti"), ("fr", "Jutland-Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ ડ\u{ac7}નમાર\u{acd}ક પ\u{acd}રા\u{a82}ત"), ("hi", "मध\u{94d}य ड\u{947}नमार\u{94d}क प\u{94d}रद\u{947}श"), ("hr", "Središnji Jutland"), ("hu", "Midtjylland régió"), ("hy", "Կենտրոնական Յուտլանդիա տարածաշրջան"), ("id", "Region Midtjylland"), ("it", "Jutland centrale"), ("ja", "中央ユラン地域"), ("ka", "ცენტრალური იუტლანდიის რეგიონი"), ("kk", "Орталық Ютландия (облыс)"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "중앙윌란 지역"), ("lt", "Vidurio Jutlandijos regionas"), ("lv", "Vidusjitlandes reģions"), ("mk", "Средна Данска"), ("mr", "स\u{947}\u{902}ट\u{94d}रल ड\u{947}नमार\u{94d}क प\u{94d}रद\u{947}श"), ("ms", "Region Midtjylland"), ("nb", "Region Midtjylland"), ("nl", "Midden-Jutland"), ("no", "Region Midtjylland"), ("pl", "Jutlandia Środkowa"), ("pt", "Jutlândia Central"), ("ro", "Regiunea Midtjylland"), ("ru", "Центральная Ютландия"), ("si", "මද\u{dca}\u{200d}යම ඩෙන\u{dca}ම\u{dcf}ර\u{dca}ක\u{dca} කල\u{dcf}පය"), ("sk", "Region Midtjylland"), ("sr", "Средишња Данска"), ("sr_Latn", "Središnja Danska"), ("sv", "Region Mittjylland"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd} பகுதி"), ("te", "స\u{c46}ంట\u{c4d}ర\u{c4d}రల\u{c4d} డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตเซนทร\u{e31}ลเดนหมาก"), ("tr", "Merkezi Danimarka Bölgesi"), ("uk", "Центральна Ютландія"), ("ur", "وسطی ڈنمارک علاقہ"), ("vi", "Trung Jutland"), ("zh", "中日德兰大区")]),
                        unofficial_name_list: ["Central Denmark", "Central Jutland", "Central Jutland Region", "Mid Jutland"].to_vec(),
                    }
                ),
                (
                    "83",
                    Subdivision{
                        name: "83",
                        country_alpha2: Alpha2::DK,
                        code: "83",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.26392), longitude: Some(9.501785), max_latitude: Some(57.8794382), min_latitude: Some(54.5591211), max_longitude: Some(15.1972813), min_longitude: Some(8.072240899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سيد دنمارك"), ("az", "Cənubi Danimarka"), ("be", "Паўднёвая Данія"), ("bg", "Южна Дания"), ("bn", "অঞ\u{9cd}চলঅফ স\u{9be}উদ\u{9be}র\u{9cd}ন ডেরম\u{9be}র\u{9cd}ক"), ("bs", "Južna Danska"), ("ca", "Regió de Syddanmark"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄓𑄬𑄚\u{11134}𑄟𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Region Syddanmark"), ("cs", "Syddanmark"), ("da", "Region Syddanmark"), ("de", "Region Syddanmark"), ("el", "Περιφέρεια Νότιας Δανίας"), ("en", "Southern Denmark"), ("es", "Dinamarca Meridional"), ("et", "Lõuna-Taani piirkond"), ("eu", "Hegoaldeko Danimarka"), ("fa", "سیددانمارک"), ("fi", "Etelä-Tanska"), ("fr", "Danemark-du-Sud"), ("gu", "દક\u{acd}ષિણી ડ\u{ac7}નમાર\u{acd}ક પ\u{acd}રદ\u{ac7}શ"), ("hi", "दक\u{94d}षिणी ड\u{947}नमार\u{94d}क क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Južna Danska"), ("hu", "Syddanmark régió"), ("hy", "Հարավային Դանիա տարածաշրջան"), ("id", "Region Syddanmark"), ("it", "Danimarca meridionale"), ("ja", "南デンマーク地域"), ("ka", "სამხრეთ დანიის რეგიონი"), ("kk", "Оңтүстік Дания"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd}ನ ಪ\u{ccd}ರದೇಶ"), ("ko", "남덴마크 지역"), ("lt", "Pietų Danijos regionas"), ("lv", "Dienviddānijas reģions"), ("mk", "Јужна Данска"), ("mr", "दक\u{94d}षिणी ड\u{947}न\u{94d}मार\u{94d}कचा प\u{94d}रद\u{947}श"), ("ms", "Region Syddanmark"), ("nb", "Region Syddanmark"), ("nl", "Zuid-Denemarken"), ("no", "Region Syddanmark"), ("pl", "Dania Południowa"), ("pt", "Dinamarca do Sul"), ("ro", "Regiunea Syddanmark"), ("ru", "Южная Дания"), ("si", "දක\u{dd4}ණ\u{dd4} ඩෙන\u{dca}ම\u{dcf}ර\u{dca}ක\u{dca} කල\u{dcf}පය"), ("sk", "Južné Dánsko"), ("sr", "Јужна Данска"), ("sr_Latn", "Južna Danska"), ("sv", "Region Syddanmark"), ("ta", "டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd} தெற\u{bcd}கு பகுதி"), ("te", "దక\u{c4d}ష\u{c3f}ణ డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ร\u{e35}เจน ออฟ เซาท\u{e4c}เท\u{e34}ร\u{e4c}น"), ("tr", "Güney Danimarka Bölgesi"), ("uk", "Південна Данія"), ("ur", "جنوبی ڈنمارک علاقہ"), ("vi", "Nam Đan Mạch"), ("zh", "南丹麦大区")]),
                        unofficial_name_list: ["South Denmark"].to_vec(),
                    }
                ),
                (
                    "84",
                    Subdivision{
                        name: "84",
                        country_alpha2: Alpha2::DK,
                        code: "84",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.6751812), longitude: Some(12.5493261), max_latitude: Some(56.200283), min_latitude: Some(54.98718119999999), max_longitude: Some(15.157218), min_longitude: Some(11.694833)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم هوفدستادن"), ("be", "Сталічная вобласць"), ("bg", "Столична област"), ("bn", "কেপিট\u{9be}ল অঞ\u{9cd}চলঅফ ডেরম\u{9be}র\u{9cd}ক"), ("bs", "Regija glavnog grada"), ("ca", "Regió de Hovedstaden"), ("ccp", "𑄢𑄌\u{11134}𑄙𑄚\u{11129} 𑄢𑄬𑄎\u{11133}𑄠\u{11127}"), ("ceb", "Region Hovedstaden"), ("cs", "Hovedstaden"), ("da", "Region Hovedstaden"), ("de", "Region Hovedstaden"), ("el", "Περιφέρεια Πρωτεύουσας της Δανίας"), ("en", "Capital Region"), ("es", "Región Capital"), ("et", "Pealinna piirkond"), ("eu", "Hovedstaden"), ("fa", "هوودستادن"), ("fi", "Pääkaupunkialue"), ("fr", "Hovedstaden"), ("gu", "ડ\u{ac7}નમાર\u{acd}કનો ક\u{ac7}પિટલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "ड\u{947}नमार\u{94d}क राजधानी क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Hovedstaden"), ("hu", "Hovedstaden régió"), ("hy", "Մայրաքաղաքային տարածաշրջան"), ("id", "Region Hovedstaden"), ("is", "Höfuðborgarsvæði Danmerkur"), ("it", "Hovedstaden"), ("ja", "デンマーク首都地域"), ("ka", "დედაქალაქის რეგიონი"), ("kk", "Астаналық облыс"), ("kn", "ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd}ನ ರಾಜಧಾನ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "덴마크 수도 지역"), ("lt", "Sostinės regionas"), ("lv", "Galvaspilsētas reģions"), ("mk", "Голем Копенхаген"), ("mr", "ड\u{947}न\u{94d}मार\u{94d}कचा राजधानी प\u{94d}रद\u{947}श"), ("ms", "Region Hovedstaden"), ("nb", "Region Hovedstaden"), ("nl", "Hoofdstad"), ("no", "Region Hovedstaden"), ("pl", "Region Stołeczny"), ("pt", "Região da Capital"), ("ro", "Regiunea Hovedstaden"), ("ru", "Ховедстаден"), ("si", "ඩෙන\u{dca}ම\u{dcf}ර\u{dca}ක\u{dca} අග නගරය"), ("sk", "Hovedstaden"), ("sr", "Велики Копенхаген"), ("sr_Latn", "Veliki Kopenhagen"), ("sv", "Region Hovedstaden"), ("ta", "கேப\u{bcd}பிடல\u{bcd} பகுதி டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd}"), ("te", "డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d} ర\u{c3e}జధ\u{c3e}న\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ราชอาณาจ\u{e31}กรเดนมาร\u{e4c}ก"), ("tr", "Danimarka Capital Bölgesi"), ("uk", "Столичний регіон"), ("ur", "دارالحکومت علاقہ ڈنمارک"), ("vi", "Vùng thủ đô Đan Mạch"), ("zh", "首都大区")]),
                        unofficial_name_list: ["Capital Region", "Region Hovedstaden"].to_vec(),
                    }
                ),
                (
                    "85",
                    Subdivision{
                        name: "85",
                        country_alpha2: Alpha2::DK,
                        code: "85",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.4632518), longitude: Some(11.7214979), max_latitude: Some(56.129846), min_latitude: Some(54.962158), max_longitude: Some(12.6244919), min_longitude: Some(10.8682958)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم شيلندا"), ("be", "Зеландыя"), ("bg", "Зеландия"), ("bs", "Zealand (regija)"), ("ca", "Regió de Sjælland"), ("ccp", "𑄎\u{11129}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Region Sjælland"), ("cs", "Sjælland"), ("da", "Region Sjælland"), ("de", "Region Sjælland"), ("el", "Σγιέλαν"), ("en", "Zealand"), ("es", "Región de Selandia"), ("et", "Sjællandi piirkond"), ("eu", "Seelandia eskualdea"), ("fa", "استان شیلند"), ("fi", "Sjælland"), ("fr", "Sjælland"), ("he", "קטגוריה:שלן"), ("hr", "Zeland"), ("hu", "Sjælland régió"), ("hy", "Զելանդիա տարածաշրջան"), ("id", "Region Sjælland"), ("it", "Selandia"), ("ja", "シェラン地域"), ("ka", "ზელანდიის რეგიონი"), ("kk", "Зеландия"), ("ko", "셸란 지역"), ("lt", "Zelandijos regionas"), ("mk", "Сјеланд"), ("ms", "Region Sjælland"), ("nb", "Region Sjælland"), ("nl", "Seeland"), ("no", "Region Sjælland"), ("pl", "Zelandia"), ("pt", "Zelândia"), ("ro", "Regiunea Sjælland"), ("ru", "Зеландия"), ("sk", "Sjælland (región)"), ("sr", "Сјеланд (покрајина)"), ("sr_Latn", "Sjeland (pokrajina)"), ("sv", "Region Själland"), ("tr", "Zealand Bölgesi"), ("uk", "Зеландія"), ("vi", "Zealand"), ("yue", "舍蘭大區"), ("yue_Hans", "舍兰大区"), ("zh", "西兰大区")]),
                        unofficial_name_list: ["Zeeland"].to_vec(),
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
#[cfg(feature = "dk")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::DK,
        alpha3: Alpha3::DNK,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 45,
        currency_code: CurrencyCode::DKK,
        gec: Some(GEC::DA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::DEN),
        iso_long_name: "The Kingdom of Denmark",
        iso_short_name: "Denmark",
        official_language_list: ["da"].to_vec(),
        spoken_language_list: ["da"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Danish"),
        number: "208",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "DK",
        unofficial_name_list: [
            "Denmark",
            "Dänemark",
            "Danemark",
            "Dinamarca",
            "デンマーク",
            "Denemarken",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Denmark"),
            ("af", "Denemarke"),
            ("ak", "Denmark"),
            ("am", "፤ንሢሴጤ"),
            ("an", "Denmark"),
            ("ar", "الد\u{651}نمارك"),
            ("as", "ডেনম\u{9be}ৰ\u{9cd}ক"),
            ("ay", "Denmark"),
            ("az", "Danimarka"),
            ("ba", "Denmark"),
            ("be", "Данія"),
            ("bg", "Дания"),
            ("bi", "Denmark"),
            ("bn", "ডেনম\u{9be}র\u{9cd}ক"),
            ("bn_IN", "ডেনম\u{9be}র\u{9cd}ক"),
            ("br", "Danmark"),
            ("bs", "Denmark"),
            ("ca", "Dinamarca"),
            ("ce", "Дани"),
            ("ch", "Denmark"),
            ("cs", "Dánsko"),
            ("cv", "Дани"),
            ("cy", "Denmarc"),
            ("da", "Danmark"),
            ("de", "Dänemark"),
            ("dv", "ޑ\u{7ac}ނ\u{7b0}މ\u{7a7}ކ\u{7aa}"),
            ("dz", "ཌ\u{f7a}ན་མ\u{f71}རཀ།"),
            ("ee", "Denmark"),
            ("el", "Δανία"),
            ("en", "Denmark"),
            ("eo", "Danio"),
            ("es", "Dinamarca"),
            ("et", "Taani"),
            ("eu", "Danimarka"),
            ("fa", "دانمارک"),
            ("ff", "Danemark"),
            ("fi", "Tanska"),
            ("fo", "Danmørk"),
            ("fr", "Danemark"),
            ("fy", "Denemark"),
            ("ga", "An Danmhairg"),
            ("gl", "Dinamarca"),
            ("gn", "Denmark"),
            ("gu", "ડ\u{ac7}નમાર\u{acd}ક"),
            ("gv", "Yn Danvarg"),
            ("ha", "Denmark"),
            ("he", "דנמרק"),
            ("hi", "ड\u{947}नमार\u{94d}क"),
            ("hr", "Danska"),
            ("ht", "Danmak"),
            ("hu", "Dánia"),
            ("hy", "Դանիա"),
            ("ia", "Danmark"),
            ("id", "Denmark"),
            ("io", "Dania"),
            ("is", "Danmörk"),
            ("it", "Danimarca"),
            ("iu", "Denmark"),
            ("ja", "デンマーク"),
            ("ka", "დანია"),
            ("ki", "Denmark"),
            ("kk", "Дания"),
            ("kl", "Denmark"),
            ("km", "ដាណ\u{17ba}ម\u{17c9}ាក"),
            ("kn", "ಡ\u{cc6}ನ\u{ccd}ಮಾರ\u{ccd}ಕ\u{ccd}"),
            ("ko", "덴마크"),
            ("ku", "Danîmarka"),
            ("kv", "Дания"),
            ("kw", "Danmark"),
            ("ky", "Дания"),
            ("lo", "ປະເທດດານມາກ"),
            ("lt", "Danija"),
            ("lv", "Dānija"),
            ("mi", "Tenemāka"),
            ("mk", "Данска"),
            ("ml", "ഡെന\u{d4d}മ\u{d3e}ര\u{d4d}\u{200d}ക\u{d4d}ക\u{d4d}"),
            ("mn", "Дани"),
            ("mr", "ड\u{947}न\u{94d}मार\u{94d}क"),
            ("ms", "Denmark"),
            ("mt", "Danimarka"),
            (
                "my",
                "ဒ\u{102d}န\u{103a}းမတ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Denemark"),
            ("nb", "Danmark"),
            ("ne", "ड\u{947}नमार\u{94d}क"),
            ("nl", "Denemarken"),
            ("nn", "Danmark"),
            ("nv", "Denmark"),
            ("oc", "Danemarc"),
            ("or", "ଡେନମ\u{b3e}ର\u{b4d}କ"),
            ("pa", "ਡ\u{a48}ਨਮਾਰਕ"),
            ("pi", "ड\u{947}नमार\u{94d}क"),
            ("pl", "Dania"),
            ("ps", "ډنمارک"),
            ("pt", "Dinamarca"),
            ("pt_BR", "Dinamarca"),
            ("ro", "Danemarca"),
            ("ru", "Дания"),
            ("rw", "Danimarike"),
            ("sc", "Danimarca"),
            ("sd", "Denmark"),
            ("si", "ඩෙන\u{dca}ම\u{dcf}කය"),
            ("sk", "Dánsko"),
            ("sl", "Danska"),
            ("so", "Danmaark"),
            ("sq", "Danimarkë"),
            ("sr", "Данска"),
            ("sv", "Danmark"),
            ("sw", "Denmark"),
            ("ta", "டென\u{bcd}ம\u{bbe}ர\u{bcd}க\u{bcd}"),
            ("te", "డ\u{c46}న\u{c4d}మ\u{c3e}ర\u{c4d}క\u{c4d}"),
            ("tg", "Дания"),
            ("th", "เดนมาร\u{e4c}ก"),
            ("ti", "ዴንማርክ"),
            ("tk", "Daniýa"),
            ("tl", "Denmark"),
            ("tr", "Danimarka"),
            ("tt", "Даниа"),
            ("ug", "دانىيە"),
            ("uk", "Данія"),
            ("ur", "ڈنمارک"),
            ("uz", "Daniya"),
            ("ve", "Denmark"),
            ("vi", "Đan Mạch"),
            ("wa", "Daenmåtche"),
            ("wo", "Danmaark"),
            ("xh", "Denmark"),
            ("yo", "Dẹ\u{301}nmárkì"),
            ("zh_CN", "丹麦"),
            ("zh_HK", "丹麥"),
            ("zh_TW", "丹麥"),
            ("zu", "IDenimaki"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
