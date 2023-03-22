// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Korea

#[cfg(all(feature = "kr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}}\n{{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::KR;
    pub const ALPHA3: Alpha3 = Alpha3::KOR;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 82;
    pub const CURRENCY_CODE: &str = "KRW";
    pub const GEC: Option<GEC> = Some(GEC::KS);
    pub const INTERNATIONAL_PREFIX: &str = "001";
    pub const IOC: Option<&str> = Some("KOR");
    pub const ISO_SHORT_NAME: &str = "Korea (Republic of)";
    pub const ISO_LONG_NAME: &str = "The Republic of Korea";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ko"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ko"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("South Korean");
    pub const NUMBER: &str = "410";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "KR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "South Korea",
        "Korea (South)",
        "Südkorea",
        "Corée du Sud",
        "Corea del Sur",
        "大韓民国",
        "Zuid-Korea",
        "Korea (Republic of)",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "South Korea"),
        ("af", "Suid-Korea"),
        ("ak", "South Korea"),
        ("am", "ደቡብ ኮሪያ"),
        ("an", "South Korea"),
        ("ar", "كوريا، جمهوري\u{651}ة كوريا"),
        ("as", "কোৰিয়\u{9be} প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"),
        ("ay", "South Korea"),
        ("az", "Koreya, Respublika"),
        ("ba", "South Korea"),
        ("be", "Карэя, Рэспубліка"),
        ("bg", "Корея, Република"),
        ("bi", "South Korea"),
        ("bn", "কোরিয়\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("bn_IN", "কোরিয়\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
        ("br", "South Korea"),
        ("bs", "Koreja, Republika"),
        ("ca", "Corea del Sud"),
        ("ce", "South Korea"),
        ("ch", "South Korea"),
        ("cs", "Korea, republika"),
        ("cv", "South Korea"),
        ("cy", "Corëa, Gweriniaeth"),
        ("da", "Korea, Republikken"),
        ("de", "Südkorea"),
        ("dv", "South Korea"),
        (
            "dz",
            "ཀ\u{f7c}་ར\u{f72}་ཡ། ར\u{f72}་པཔ་ལ\u{f72}ཀ་ ཨ\u{f7c}ཕ་",
        ),
        ("ee", "South Korea"),
        ("el", "Κορέα, Δημοκρατία της"),
        ("en", "South Korea"),
        ("eo", "Sud-Koreio"),
        ("es", "Corea, República de"),
        ("et", "Lõuna-Korea"),
        ("eu", "Korea, Errepublika"),
        ("fa", "جمهوری کره"),
        ("ff", "South Korea"),
        ("fi", "Korean tasavalta"),
        ("fo", "South Korea"),
        ("fr", "Corée du Sud"),
        ("fy", "South Korea"),
        ("ga", "Poblacht na Cóiré"),
        ("gl", "Corea, República de"),
        ("gn", "South Korea"),
        ("gu", "દક\u{acd}ષિણ આફ\u{acd}રિકા"),
        ("gv", "South Korea"),
        ("ha", "South Korea"),
        ("he", "קוריאה הדרומית"),
        ("hi", "दक\u{94d}षिण कोरिया"),
        ("hr", "Južna Korea"),
        ("ht", "South Korea"),
        ("hu", "Koreai Köztársaság"),
        ("hy", "Կորեա Հանարպետություն"),
        ("ia", "Corea, Republica de"),
        ("id", "Korea Selatan"),
        ("io", "South Korea"),
        ("is", "Suður-Kórea"),
        ("it", "Corea del sud"),
        ("iu", "South Korea"),
        ("ja", "大韓民国 (韓国)"),
        ("ka", "კორეა, რესპუბლიკა"),
        ("ki", "South Korea"),
        ("kk", "Корея Республикасы"),
        ("kl", "South Korea"),
        (
            "km",
            "ក\u{17bc}រ\u{17c9}េ សាធារណរដ\u{17d2}ឋ\u{200b}នៃរ\u{17c9}េ",
        ),
        ("kn", "ಕೋರ\u{cbf}ಯಾ ಗಣರಾಜ\u{ccd}ಯ"),
        ("ko", "대한민국"),
        ("ku", "Kore, Komara"),
        ("kv", "South Korea"),
        ("kw", "South Korea"),
        ("ky", "Корея Республикасы"),
        ("lo", "South Korea"),
        ("lt", "Korėjos Respublika"),
        ("lv", "Dienvidkoreja"),
        ("mi", "South Korea"),
        ("mk", "Кореја, Република"),
        ("ml", "കൊറിയ, റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"),
        ("mn", "Солонгос ард улс"),
        ("mr", "कोरिया, रिपब\u{94d}लिक ऑफ"),
        ("ms", "South Korea"),
        ("mt", "South Korea"),
        ("my", "South Korea"),
        ("na", "South Korea"),
        ("nb", "Sør-Korea"),
        ("ne", "कोरियाको गणराज\u{94d}य"),
        ("nl", "Zuid-Korea"),
        ("nn", "Sør-Korea"),
        ("nv", "South Korea"),
        ("oc", "Corèa del Sud"),
        ("or", "କୋର\u{b3f}ଆ, ଗଣତନ\u{b4d}ତ\u{b4d}ର"),
        ("pa", "ਕ\u{a4b}ਰੀਆ ਗਣਰਾਜ"),
        ("pi", "South Korea"),
        ("pl", "Korea Południowa"),
        ("ps", "South Korea"),
        ("pt", "Coreia do Sul"),
        ("pt_BR", "Coreia do Sul"),
        ("ro", "Republica Coreea"),
        ("ru", "Южная Корея"),
        ("rw", "Koreya, Repubulika ya"),
        ("sc", "Corea de su Sud"),
        ("sd", "South Korea"),
        ("si", "කොර\u{dd2}ය\u{dcf} ජනරජය"),
        ("sk", "Kórejská republika"),
        ("sl", "Južna Koreja"),
        ("so", "South Korea"),
        ("sq", "Korea, Republika e"),
        ("sr", "Кореја, Република"),
        ("sv", "Sydkorea"),
        ("sw", "South Korea"),
        ("ta", "கொரிய குடியரசு"),
        (
            "te",
            "క\u{c4b}ర\u{c3f}య\u{c3e}, ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}",
        ),
        ("tg", "Ҷумҳурии Корея"),
        ("th", "เกาหล\u{e35}ใต\u{e49}"),
        ("ti", "South Korea"),
        ("tk", "Koreýa Respublikasy"),
        ("tl", "Korea, Republika ng"),
        ("tr", "Güney Kore"),
        ("tt", "Кореа Җөмһүриәте"),
        ("ug", "كورېيە"),
        ("uk", "Південна Корея"),
        ("ur", "South Korea"),
        ("uz", "South Korea"),
        ("ve", "South Korea"),
        ("vi", "Cộng hoà Nam Hàn"),
        ("wa", "Corêye (nonnrece)"),
        ("wo", "Koore, Republik bu"),
        ("xh", "South Korea"),
        ("yo", "South Korea"),
        ("zh_CN", "韩国"),
        ("zh_HK", "大韓民國"),
        ("zh_TW", "大韓民國"),
        ("zu", "South Korea"),
    ];
    #[cfg(all(feature = "kr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 35.907757;
        pub const LONGITUDE: f64 = 127.766922;
        pub const MAX_LATITUDE: f64 = 38.63400000000001;
        pub const MAX_LONGITUDE: f64 = 131.1603;
        pub const MIN_LATITUDE: f64 = 33.0041;
        pub const MIN_LONGITUDE: f64 = 124.5863;
        pub const NORTHEAST_LATITUDE: f64 = 38.63400000000001;
        pub const NORTHEAST_LONGITUDE: f64 = 131.1603;
        pub const SOUTHWEST_LATITUDE: f64 = 33.0041;
        pub const SOUTHWEST_LONGITUDE: f64 = 124.5863;
    }
}
#[cfg(all(feature = "kr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 35.907757,
            longitude: 127.766922,
            max_latitude: 38.63400000000001,
            max_longitude: 131.1603,
            min_latitude: 33.0041,
            min_longitude: 124.5863,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 38.63400000000001,
                    longitude: 131.1603,
                },
                southwest: CountryGeoBound {
                    latitude: 33.0041,
                    longitude: 124.5863,
                },
            },
        }
    }
}

#[cfg(all(feature = "kr", feature = "subdivisions"))]
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
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::KR,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.566535), longitude: Some(126.9779692), max_latitude: Some(37.7017495), min_latitude: Some(37.4259627), max_longitude: Some(127.18359), min_longitude: Some(126.7645827)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Seoel"), ("am", "ሶል"), ("ar", "سول"), ("as", "ছিউল"), ("az", "Seul"), ("be", "Сеул"), ("bg", "Сеул"), ("bn", "সিওল"), ("bs", "Seul"), ("ca", "Seül"), ("ccp", "𑄥\u{11128}𑄅\u{1112a}𑄣\u{11134}"), ("ceb", "Seoul"), ("cs", "Soul"), ("cy", "Seoul"), ("da", "Seoul"), ("de", "Seoul"), ("el", "Σεούλ"), ("en", "Seoul"), ("es", "Seúl"), ("et", "Sŏul"), ("eu", "Seul"), ("fa", "سئول"), ("fi", "Soul"), ("fr", "Séoul"), ("ga", "Seoul"), ("gl", "Seúl"), ("gu", "સિઓલ"), ("ha", "Seoul"), ("ha_NE", "Seoul"), ("he", "סיאול"), ("hi", "सियोल"), ("hr", "Seul"), ("hu", "Szöul"), ("hy", "Սեուլ"), ("id", "Seoul"), ("is", "Seúl"), ("it", "Seul"), ("ja", "ソウル特別市"), ("jv", "Seoul"), ("ka", "სეული"), ("kk", "Сеул"), ("km", "សេអ\u{17ca}\u{17bc}ល"), ("kn", "ಸ\u{ccc}ಲ\u{ccd}"), ("ko", "서울특별시"), ("ky", "Сеул"), ("lt", "Seulas"), ("lv", "Seula"), ("mk", "Сеул"), ("ml", "സോൾ"), ("mn", "Сөүл"), ("mr", "सोल"), ("ms", "Seoul"), ("my", "ဆ\u{102d}\u{102f}းလ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Seoul"), ("ne", "सोल"), ("nl", "Seoel"), ("no", "Seoul"), ("or", "ସ\u{b3f}ଓଲ"), ("pa", "ਸਿਓਲ"), ("pl", "Seul"), ("ps", "سيول"), ("pt", "Seul"), ("ro", "Seul"), ("ru", "Сеул"), ("si", "\u{200d}සෝල\u{dca}"), ("sk", "Soul"), ("sl", "Seul"), ("so", "Seoul"), ("sq", "Seoul"), ("sr", "Сеул"), ("sr_Latn", "Seul"), ("sv", "Seoul"), ("sw", "Seoul"), ("ta", "சியோல\u{bcd}"), ("te", "స\u{c3f}య\u{c4a}ల\u{c4d}"), ("th", "โซล"), ("tk", "Seul"), ("tr", "Seul"), ("uk", "Сеул"), ("ur", "سیول"), ("uz", "Seul"), ("vi", "Seoul"), ("yo", "Seoul"), ("yo_BJ", "Seoul"), ("yue", "首爾"), ("yue_Hans", "首尔"), ("zh", "首爾")]),
                        unofficial_name_list: ["Seoul", "Soul"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::KR,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1795543), longitude: Some(129.0756416), max_latitude: Some(35.3874414), min_latitude: Some(34.9835815), max_longitude: Some(129.3100483), min_longitude: Some(128.787197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Busan"), ("am", "ቡሳን"), ("ar", "بوسان"), ("az", "Busan"), ("be", "Горад Пусан"), ("bg", "Пусан"), ("bn", "ব\u{9c1}স\u{9be}ন"), ("ca", "Busan"), ("ccp", "𑄝\u{1112a}𑄥𑄚\u{11134}"), ("ceb", "Busan (lalawigan)"), ("cs", "Pusan"), ("cy", "Busan"), ("da", "Busan"), ("de", "Busan"), ("el", "Μπούσαν"), ("en", "Busan"), ("es", "Busan"), ("et", "Pusan"), ("eu", "Busan"), ("fa", "بوسان"), ("fi", "Busan"), ("fr", "Busan"), ("ga", "Busan"), ("gl", "Busan"), ("gu", "બ\u{ac1}સાન"), ("ha", "Busan"), ("ha_NE", "Busan"), ("he", "פוסן"), ("hi", "ब\u{941}सान"), ("hr", "Busan"), ("hu", "Puszan"), ("hy", "Բուսան"), ("id", "Busan"), ("is", "Busan"), ("it", "Pusan"), ("ja", "釜山広域市"), ("jv", "Busan"), ("ka", "პუსანი"), ("kk", "Пусан"), ("kn", "ಬುಸಾನ\u{ccd}"), ("ko", "부산광역시"), ("ky", "Пусан"), ("lt", "Busanas"), ("lv", "Pusana"), ("mk", "Пусан"), ("ml", "ബ\u{d41}സ\u{d3e}ൻ"), ("mn", "Пүсан"), ("mr", "ब\u{941}सान"), ("ms", "Busan"), ("my", "ဘ\u{1030}ဆန\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Busan"), ("nl", "Busan"), ("no", "Busan"), ("pa", "ਬ\u{a42}ਸਾਨ"), ("pl", "Pusan"), ("pt", "Busan"), ("ro", "Busan"), ("ru", "Пусан"), ("si", "බ\u{dd4}ස\u{dcf}න\u{dca}"), ("sk", "Pusan"), ("sl", "Pusan"), ("sq", "Busan"), ("sr", "Бусан"), ("sr_Latn", "Busan"), ("sv", "Pusan"), ("sw", "Busan"), ("ta", "புச\u{bbe}ன\u{bcd}"), ("te", "బుస\u{c3e}న\u{c4d}"), ("th", "ป\u{e39}ซาน"), ("tr", "Busan"), ("uk", "Пусан"), ("ur", "بوسان"), ("uz", "Pusan"), ("vi", "Busan"), ("yue", "釜山"), ("yue_Hans", "釜山"), ("zh", "釜山")]),
                        unofficial_name_list: ["Busan"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::KR,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.8714354), longitude: Some(128.601445), max_latitude: Some(36.0172827), min_latitude: Some(35.6071707), max_longitude: Some(128.7632175), min_longitude: Some(128.3497208)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Daegu"), ("ar", "دايغو"), ("be", "Тэгу"), ("bg", "Тегу"), ("bn", "ডেগ\u{9c1}"), ("ca", "Daegu"), ("ccp", "𑄓𑄠𑄬𑄉\u{1112a}"), ("ceb", "Daegu (lalawigan)"), ("cs", "Tegu"), ("da", "Daegu"), ("de", "Daegu"), ("el", "Ντέγκου"), ("en", "Daegu"), ("es", "Daegu"), ("et", "Taegu"), ("eu", "Daegu"), ("fa", "دائجو"), ("fi", "Daegu"), ("fr", "Daegu"), ("gl", "Daegu"), ("gu", "ડ\u{ac7}ગ\u{ac2}"), ("ha", "Daegu"), ("ha_NE", "Daegu"), ("he", "טגו"), ("hi", "दाएग\u{942}"), ("hr", "Daegu"), ("hu", "Tegu"), ("hy", "Դեգու"), ("id", "Daegu"), ("it", "Taegu"), ("ja", "大邱広域市"), ("ka", "ტეგუ"), ("kk", "Тэгу"), ("kn", "ಡೇಗು"), ("ko", "대구광역시"), ("lt", "Tegu"), ("lv", "Tegu"), ("ml", "ദേഗ\u{d41}"), ("mn", "Тэгү"), ("mr", "द\u{948}ग\u{942}"), ("ms", "Daegu"), ("nb", "Daegu"), ("nl", "Daegu"), ("no", "Daegu"), ("pl", "Daegu"), ("pt", "Daegu"), ("ro", "Daegu"), ("ru", "Тэгу"), ("si", "ඩය\u{dd2}ග\u{dd4}"), ("sk", "Tägu"), ("sl", "Daegu"), ("sr", "Тегу"), ("sr_Latn", "Tegu"), ("sv", "Daegu"), ("sw", "Daegu"), ("ta", "டேகு"), ("te", "డ\u{c47}గు"), ("th", "แทก\u{e39}"), ("tr", "Daegu"), ("uk", "Тегу"), ("ur", "ڈائے گو"), ("vi", "Daegu"), ("yue", "大邱"), ("yue_Hans", "大邱"), ("zh", "大邱廣域市")]),
                        unofficial_name_list: ["Daegu"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::KR,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.4562557), longitude: Some(126.7052062), max_latitude: Some(37.982666), min_latitude: Some(37.0063057), max_longitude: Some(126.7936273), min_longitude: Some(124.608139)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Incheon"), ("ar", "إنتشون"), ("be", "Горад Інчхон"), ("bg", "Инчхън"), ("bn", "ইনছন"), ("ca", "Inchon"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄥\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Incheon"), ("cs", "Inčchon"), ("cy", "Incheon"), ("da", "Incheon"), ("de", "Incheon"), ("el", "Ιντσόν"), ("en", "Incheon"), ("es", "Incheon"), ("et", "Inch’ŏn"), ("eu", "Incheon"), ("fa", "اینچئون"), ("fi", "Incheon"), ("fr", "Incheon"), ("gu", "ઇન\u{acd}ચ\u{ac7}ઓન"), ("ha", "Incheon"), ("ha_NE", "Incheon"), ("he", "אינצ׳ון"), ("hi", "इ\u{902}चियोन"), ("hr", "Incheon"), ("hu", "Incshon"), ("hy", "Ինչոն"), ("id", "Incheon"), ("it", "Incheon"), ("ja", "仁川広域市"), ("jv", "Incheon"), ("ka", "ინჩხონი"), ("kk", "Инчхон"), ("kn", "ಇಂಚ\u{cbf}ಯೋನ\u{ccd}"), ("ko", "인천광역시"), ("ky", "Инчхон"), ("lt", "Inčonas"), ("lv", "Inčhona"), ("mn", "Инчон"), ("mr", "इ\u{902}च\u{947}वॉन"), ("ms", "Incheon"), ("my", "အင\u{103a}ချ\u{103d}န\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Incheon"), ("nl", "Incheon"), ("no", "Incheon"), ("pa", "ਇਨਚਨ"), ("pl", "Inczon"), ("pt", "Incheon"), ("ro", "Incheon"), ("ru", "Инчхон"), ("si", "ඉන\u{dca}ච\u{dd2}යෝන\u{dca}"), ("sk", "Inčchon"), ("sr", "Инчон"), ("sr_Latn", "Inčon"), ("sv", "Inchon"), ("sw", "Incheon"), ("ta", "இங\u{bcd}கியோன\u{bcd}"), ("te", "ఇంచ\u{c3f}య\u{c4b}న\u{c4d}"), ("th", "อ\u{e34}นช\u{e47}อน"), ("tr", "İncheon"), ("uk", "Інчхон"), ("ur", "ان چیون"), ("uz", "Inchxon"), ("vi", "Incheon"), ("yue", "仁川"), ("yue_Hans", "仁川"), ("zh", "仁川廣域市")]),
                        unofficial_name_list: ["Incheon", "Inchon"].to_vec(),
                    }
                ),
                (
                    "29",
                    Subdivision{
                        name: "29",
                        country_alpha2: Alpha2::KR,
                        code: "29",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1595454), longitude: Some(126.8526012), max_latitude: Some(35.2589426), min_latitude: Some(35.0508149), max_longitude: Some(127.0229414), min_longitude: Some(126.6449036)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gwangju"), ("ar", "غوانغجو"), ("az", "Kvanju"), ("be", "Горад Кванджу"), ("bg", "Куанджу"), ("bn", "গোয\u{9bc}\u{9be}ংজ\u{9c1}"), ("ca", "Gwangju"), ("ccp", "𑄉\u{11127}\u{11101}𑄎\u{1112a} 𑄥\u{11128}𑄑\u{11128}"), ("ceb", "Gwangju (lalawigan)"), ("cs", "Kwangdžu"), ("cy", "Gwangju"), ("da", "Gwangju"), ("de", "Gwangju"), ("el", "Γκουάνγκτζου"), ("en", "Gwangju City"), ("es", "Gwangju"), ("et", "Kwangju"), ("eu", "Gwangju"), ("fa", "گوانگجو"), ("fi", "Gwangju"), ("fr", "Gwangju"), ("gu", "ગ\u{acd}વા\u{a82}ગજ\u{ac1}"), ("he", "קוואנגג׳ו"), ("hi", "ग\u{94d}वा\u{902}गज\u{942}"), ("hu", "Kvangdzsu"), ("hy", "Գվանջու"), ("id", "Gwangju"), ("it", "Gwangju"), ("ja", "光州広域市"), ("jv", "Gwangju"), ("kk", "Кванджу"), ("kn", "ಗ\u{ccd}ವಾಂಗ\u{ccd}ಜು"), ("ko", "광주광역시"), ("ky", "Кванже"), ("lt", "Kvandžu"), ("lv", "Kvandžu"), ("mk", "Квангџу"), ("mn", "Куанжү"), ("mr", "ग\u{94d}वा\u{902}गज\u{942}"), ("ms", "Gwangju"), ("nb", "Gwangju"), ("nl", "Gwangju"), ("no", "Gwangju"), ("pl", "Gwangju"), ("pt", "Gwangju"), ("ro", "Gwangju"), ("ru", "Кванджу"), ("si", "ග\u{dca}වන\u{dca}ග\u{dca}ජ\u{dd4}"), ("sk", "Kwangdžu"), ("sl", "Gvangdžu"), ("sr", "Квангџу"), ("sr_Latn", "Kvangdžu"), ("sv", "Gwangju"), ("sw", "Gwangju"), ("ta", "குவ\u{bbe}ங\u{bcd}கு"), ("te", "గ\u{c4d}వ\u{c3e}ంగ\u{c4d}జూ"), ("th", "คว\u{e31}งจ\u{e39}"), ("tr", "Gwangju"), ("uk", "Кванджу"), ("ur", "گوانگ جو"), ("vi", "Gwangju"), ("yue", "光州"), ("yue_Hans", "光州"), ("zh", "光州廣域市")]),
                        unofficial_name_list: ["Gwangju"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::KR,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.3504119), longitude: Some(127.3845475), max_latitude: Some(36.4999477), min_latitude: Some(36.1833708), max_longitude: Some(127.5590437), min_longitude: Some(127.2464501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Daejeon"), ("ar", "دايجيون"), ("bg", "Теджън"), ("bn", "ডেজন"), ("ca", "Daejeon"), ("ccp", "𑄓𑄬𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Daejeon (lalawigan)"), ("cs", "Tedžon"), ("da", "Daejeon"), ("de", "Daejeon"), ("el", "Ντέτζον"), ("en", "Daejeon"), ("es", "Daejeon"), ("et", "Taejŏn"), ("eu", "Daejeon"), ("fa", "دائجونگ"), ("fi", "Daejeon"), ("fr", "Daejeon"), ("gu", "ડ\u{ac7}જોન"), ("ha", "Daejeon"), ("ha_NE", "Daejeon"), ("he", "טג׳אן"), ("hi", "डाइज\u{947}न"), ("hr", "Daejeon"), ("hu", "Tedzson"), ("hy", "Դեջոն"), ("id", "Daejeon"), ("it", "Daejeon"), ("ja", "大田広域市"), ("jv", "Daejeon"), ("kk", "Тэджон"), ("kn", "ಡೇಜ\u{cbf}ಯೋನ\u{ccd}"), ("ko", "대전광역시"), ("lt", "Tedžonas"), ("lv", "Tedžona"), ("mk", "Теџон"), ("mn", "Тэжон"), ("mr", "द\u{947}जॉन"), ("ms", "Daejeon"), ("nb", "Daejeon"), ("nl", "Daejeon"), ("no", "Daejeon"), ("pl", "Daejeon"), ("pt", "Daejeon"), ("ro", "Daejeon"), ("ru", "Тэджон"), ("si", "ඩය\u{dd2}ජ\u{dd2}යොන\u{dca}"), ("sk", "Tädžon"), ("sl", "Daedžeon"), ("sr", "Теџон"), ("sr_Latn", "Tedžon"), ("sv", "Daejeon"), ("sw", "Daejeon"), ("ta", "டேய\u{bcd}ஜேயோன\u{bcd}"), ("te", "డ\u{c3e}య\u{c3f}జ\u{c3f}యన\u{c4d}"), ("th", "แทจ\u{e47}อน"), ("tr", "Daejeon"), ("uk", "Теджон"), ("ur", "ڈائے جیون"), ("uz", "Daejeon"), ("vi", "Daejeon"), ("yue", "大田"), ("yue_Hans", "大田"), ("zh", "大田广域市")]),
                        unofficial_name_list: ["Daejeon", "Taejeon", "Taejon"].to_vec(),
                    }
                ),
                (
                    "31",
                    Subdivision{
                        name: "31",
                        country_alpha2: Alpha2::KR,
                        code: "31",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.5383773), longitude: Some(129.3113596), max_latitude: Some(35.7252482), min_latitude: Some(35.3218658), max_longitude: Some(129.4666138), min_longitude: Some(128.9756829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ulsan"), ("ar", "أولسان"), ("az", "Ulsan"), ("bg", "Улсан"), ("bn", "উলস\u{9be}ন"), ("ca", "Ulsan"), ("ccp", "𑄃𑄣\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Ulsan"), ("cs", "Ulsan"), ("da", "Ulsan"), ("de", "Ulsan"), ("el", "Ούλσαν"), ("en", "Ulsan"), ("es", "Ulsan"), ("et", "Ulsan"), ("eu", "Ulsan"), ("fa", "اولسان"), ("fi", "Ulsan"), ("fr", "Ulsan"), ("gu", "ઉલ\u{acd}સન"), ("he", "אולסן"), ("hi", "उल\u{94d}सान"), ("hu", "Ulszan"), ("hy", "Ուլսան"), ("id", "Ulsan"), ("it", "Ulsan"), ("ja", "蔚山広域市"), ("kk", "Ульсан"), ("kn", "ಉಲ\u{ccd}ಸಾನ\u{ccd}"), ("ko", "울산광역시"), ("lt", "Ulsanas"), ("lv", "Ulsana"), ("mn", "Үлсан"), ("mr", "उल\u{94d}सान"), ("ms", "Ulsan"), ("my", "အောလ\u{103a}ဆန\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Ulsan"), ("nl", "Ulsan"), ("no", "Ulsan"), ("pl", "Ulsan"), ("pt", "Ulsan"), ("ro", "Ulsan"), ("ru", "Ульсан"), ("si", "අල\u{dca}ස\u{dcf}න\u{dca}"), ("sr", "Улсан"), ("sr_Latn", "Ulsan"), ("sv", "Ulsan"), ("sw", "Ulsan"), ("ta", "உள\u{bcd}சன\u{bcd}"), ("te", "ఉల\u{c4d}స\u{c3e}న\u{c4d}"), ("th", "อ\u{e38}ลซ\u{e31}น"), ("tr", "Ulsan"), ("uk", "Ульсан"), ("ur", "السان"), ("uz", "Ulsan"), ("vi", "Ulsan"), ("yue", "蔚山"), ("yue_Hans", "蔚山"), ("zh", "蔚山广域市")]),
                        unofficial_name_list: ["Ulsan Gwang'yeogsi [Ulsan-Kwangyokshi]"].to_vec(),
                    }
                ),
                (
                    "41",
                    Subdivision{
                        name: "41",
                        country_alpha2: Alpha2::KR,
                        code: "41",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.41379999999999), longitude: Some(127.5183), max_latitude: Some(38.3026711), min_latitude: Some(36.8939685), max_longitude: Some(127.8582527), min_longitude: Some(126.3763885)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غيونغي"), ("bg", "Кьонги-до"), ("bn", "গিয\u{9bc}েওনগি"), ("ca", "Gyeonggi-do"), ("ccp", "𑄉\u{1112d}𑄠\u{1112e}𑄚\u{11134}𑄉\u{11128}"), ("ceb", "Gyeonggi-do"), ("cs", "Kjonggi"), ("cy", "Talaith Gyeonggi"), ("da", "Gyeonggi Province"), ("de", "Gyeonggi-do"), ("el", "Γκιόνγκι-ντο"), ("en", "Gyeonggi"), ("es", "Gyeonggi"), ("et", "Kyŏnggi provints"), ("eu", "Gyeonggi"), ("fa", "گیونگی-دو"), ("fi", "Gyeonggi"), ("fr", "Gyeonggi"), ("gu", "ગ\u{acd}યો\u{a82}ગી પ\u{acd}રા\u{a82}ત"), ("he", "גיונגי"), ("hi", "गियॉन\u{94d}गी प\u{94d}रा\u{902}त"), ("hu", "Kjonggi"), ("hy", "Գյոնգի-դո"), ("id", "Gyeonggi"), ("it", "Gyeonggi"), ("ja", "京畿道"), ("ka", "კიონგიდო"), ("kk", "Кёңгидо"), ("kn", "ಜ\u{cbf}ಯಾಂಗ\u{ccd}ಗ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "경기도"), ("lt", "Kiongi provincija"), ("lv", "Kjongido"), ("mn", "Кёнги"), ("mr", "ग\u{94d}या\u{901}गी प\u{94d}रा\u{902}त"), ("ms", "Gyeonggi"), ("nb", "Gyeonggi"), ("nl", "Gyeonggi-do"), ("no", "Gyeonggi"), ("pl", "Gyeonggi"), ("pt", "Gyeonggi"), ("ro", "Gyeonggi-do"), ("ru", "Кёнгидо"), ("si", "ජ\u{dd2}යෝන\u{dca}ග\u{dd2} පළ\u{dcf}ත"), ("sk", "Kjonggi"), ("sv", "Gyeonggi"), ("sw", "Gyeonggi-do"), ("ta", "இக\u{bcd}ய\u{bbe}ங\u{bcd}கி ம\u{bbe}நிலம\u{bcd}"), ("te", "జయ\u{c4b}ంగ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคย\u{e47}องก\u{e35}"), ("tr", "Gyeonggi"), ("uk", "Провінція Кьонгі"), ("ur", "گیئونگی صوبہ"), ("vi", "Gyeonggi"), ("yue", "京畿道"), ("yue_Hans", "京畿道"), ("zh", "京畿道")]),
                        unofficial_name_list: ["Gyeonggido [Kyonggi-do]"].to_vec(),
                    }
                ),
                (
                    "42",
                    Subdivision{
                        name: "42",
                        country_alpha2: Alpha2::KR,
                        code: "42",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غانغوون"), ("be", "правінцыя Канвандо"), ("bg", "Кануън-до"), ("bn", "গ\u{9be}ংওয\u{9bc}\u{9be}ন"), ("ca", "Gangwon-do"), ("ccp", "𑄉\u{11133}𑄠\u{11101}𑄤𑄚\u{11134}"), ("ceb", "Gangwon-do"), ("cs", "Kangwŏn"), ("cy", "Talaith Gangwon"), ("da", "Gangwon"), ("de", "Gangwon-do"), ("el", "Γκάνγουον-ντο"), ("en", "Gangwon"), ("es", "Gangwon"), ("et", "Kangwŏni provints"), ("eu", "Gangwon"), ("fa", "گانکون-دو"), ("fi", "Gangwon"), ("fr", "Gangwon"), ("gu", "ગ\u{a82}ગવાન પ\u{acd}રા\u{a82}ત"), ("he", "גאנג וואן"), ("hi", "ग\u{948}\u{902}गवॉन प\u{94d}रा\u{902}त, दक\u{94d}षिण कोरिया"), ("hu", "Kangvon"), ("hy", "Գանգվոն-դո"), ("id", "Gangwon"), ("it", "Gangwon"), ("ja", "江原道 (南)"), ("kk", "Канвондо"), ("kn", "ಗ\u{ccd}ಯಾಂಗ\u{ccd}ವಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "강원도"), ("lt", "Kangvono provincija"), ("lv", "Kanvondo province"), ("mk", "Кангвон"), ("mn", "Өмнө Солонгосын Канвонь аймаг"), ("mr", "ग\u{902}गवान प\u{94d}रा\u{902}त"), ("ms", "Gangwon"), ("nb", "Gangwon"), ("nl", "Gangwon-do"), ("no", "Gangwon"), ("pl", "Gangwon"), ("pt", "Gangwon"), ("ro", "Gangwon-do"), ("ru", "Канвондо"), ("si", "ගන\u{dca}ග\u{dca}වොන\u{dca} පළ\u{dcf}ත"), ("sk", "Kangwon"), ("sv", "Gangwon"), ("sw", "Gangwon-do"), ("ta", "க\u{bbe}ங\u{bcd}வ\u{bcd}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}య\u{c3e}ంగ\u{c4d}వన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e31}งว\u{e47}อน"), ("tr", "Kangvon"), ("uk", "Провінція Канвон"), ("ur", "گانگوان صوبہ"), ("vi", "Gangwon"), ("yue", "江原道"), ("yue_Hans", "江原道"), ("zh", "江原道")]),
                        unofficial_name_list: ["Kangwon"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::KR,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تشنغتشونغ الشمالية"), ("be", "Чхунчхон-Пукто"), ("bg", "Чхунчхън-Пукто"), ("bn", "উত\u{9cd}তর চ\u{9c1}ংচেয\u{9bc}ং"), ("ca", "Chungcheongbuk-do"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄌\u{1112a}\u{11101}𑄌\u{11128}𑄠\u{11127}\u{11101}"), ("ceb", "Chungcheongbuk-do"), ("cs", "Severní Čchungčchong"), ("cy", "Chungcheongbuk-do"), ("da", "Norra Chungcheong"), ("de", "Chungcheongbuk-do"), ("el", "Τσούνγκτσονγκμπουκ-ντο"), ("en", "North Chungcheong"), ("es", "Chungcheong del Norte"), ("et", "Põhja-Ch’ungch’ŏngi provints"), ("eu", "Ipar Chungcheong"), ("fa", "چونگچیونبوک-دو"), ("fi", "Pohjois-Chungcheon"), ("fr", "Chungcheongbuk"), ("gu", "ઉત\u{acd}તર ચ\u{ac1}\u{a82}ગચ\u{ac7}ઓ\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("he", "צפון צ׳ונגצ׳אונג"), ("hi", "उत\u{94d}तर च\u{941}\u{902}गच\u{947}ओग प\u{94d}रा\u{902}त"), ("hu", "Észak-Cshungcshong"), ("hy", "Հյուսիսային Չունգչոնգ"), ("id", "Chungcheong Utara"), ("it", "Nord Chungcheong"), ("ja", "忠清北道"), ("jv", "Chungcheong Utara"), ("kn", "ಉತ\u{ccd}ತರ ಚುಂಗ\u{ccd}ಚ\u{cc6}ಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "충청북도"), ("lt", "Šiaurės Čungčongo provincija"), ("lv", "Čhunčhonpukto province"), ("mn", "Умар Чүнчон"), ("mr", "उत\u{94d}तर च\u{941}\u{902}गचा\u{901}ग प\u{94d}रा\u{902}त"), ("ms", "Chungcheong Utara"), ("nb", "Nord-Chungcheong"), ("nl", "Chungcheongbuk-do"), ("no", "Nord-Chungcheong"), ("pl", "Ch’ungch’ŏng Północny"), ("pt", "Chungcheong do Norte"), ("ru", "Чхунчхон-Пукто"), ("si", "උත\u{dd4}ර\u{dd4} චන\u{dca}ග\u{dca}ච\u{dd2}යොන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sk", "Severný Čchungčchong"), ("sv", "Nordchungcheong"), ("sw", "Chungcheongbuk-do"), ("ta", "வடக\u{bcd}கு சுங\u{bcd}சேயோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర చంగ\u{c4d}\u{200c}చ\u{c3f}య\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e38}งช\u{e47}องเหน\u{e37}อ"), ("tr", "Kuzey Çungçeong"), ("uk", "Північна провінція Чхунчхон"), ("ur", "شمالی چونگچیونگ صوبہ"), ("vi", "Chungcheong Bắc"), ("yue", "忠清北道"), ("yue_Hans", "忠清北道"), ("zh", "忠清北道")]),
                        unofficial_name_list: ["North Chungchong"].to_vec(),
                    }
                ),
                (
                    "44",
                    Subdivision{
                        name: "44",
                        country_alpha2: Alpha2::KR,
                        code: "44",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.5184), longitude: Some(126.8), max_latitude: Some(37.0618896), min_latitude: Some(35.9806574), max_longitude: Some(127.6396353), min_longitude: Some(125.9581375)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشنغتشونغ الجنوبية"), ("bg", "Чхунчхън-Намдо"), ("bn", "দক\u{9cd}ষিণ চ\u{9c1}ংচেয\u{9bc}ং"), ("ca", "Chungcheongnam-do"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄌\u{1112a}\u{11101}𑄌\u{11128}𑄠\u{11127}\u{11101}"), ("ceb", "Chungcheongnam-do"), ("cs", "Jižní Čchungčchong"), ("cy", "Talaith De Chungcheong"), ("da", "South Chungcheong Province"), ("de", "Chungcheongnam-do"), ("el", "Τσούνγκτσονγκναμ-ντο"), ("en", "South Chungcheong"), ("es", "Chungcheong del Sur"), ("et", "Lõuna-Ch’ungch’ŏngi provints"), ("eu", "Hego Chungcheong"), ("fa", "چونگچئونگنام-دو"), ("fi", "Etelä-Chungcheong"), ("fr", "Chungcheong du Sud"), ("gu", "દક\u{acd}ષિણ ચ\u{ac1}\u{a82}ગચ\u{ac7}ઓ\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("he", "דרום צ׳ונגצ׳אונג"), ("hi", "दक\u{94d}षिणी च\u{941}\u{902}गचियो\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Dél-Cshungcshong"), ("hy", "Հարավային Չունգչոնգ"), ("id", "Chungcheong Selatan"), ("it", "Sud Chungcheong"), ("ja", "忠清南道"), ("kk", "Чхуңчхоң-Намдо"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಚುಂಗ\u{ccd}ಚ\u{cc6}ಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "충청남도"), ("lt", "Pietų Čungčongo provincija"), ("lv", "Čhunčhonnamdo province"), ("mn", "Өмнөд Чүнчон"), ("mr", "दक\u{94d}षिण च\u{941}\u{902}गचा\u{901}ग प\u{94d}रा\u{902}त"), ("ms", "Chungcheong Selatan"), ("nb", "Sør-Chungcheong"), ("nl", "Chungcheongnam-do"), ("no", "Sør-Chungcheong"), ("pl", "Chungcheong Południowy"), ("pt", "Chungcheong do Sul"), ("ru", "Чхунчхон-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} චන\u{dca} ච\u{dd2}යෝන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sk", "Južný Čchungčchong"), ("sv", "Sydchungcheong"), ("sw", "Chungcheongnam-do"), ("ta", "தெற\u{bcd}கு சுங\u{bcd}கிச\u{bcd}செவ\u{bcd}ங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ చంగ\u{c4d}\u{200c}చ\u{c3f}య\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e38}งช\u{e47}องใต\u{e49}"), ("tr", "Güney Çungçeong"), ("uk", "Південна провінція Чхунчхон"), ("ur", "جنوبی چونگچیونگ صوبہ"), ("vi", "Chungcheong Nam"), ("yue", "忠清南道"), ("yue_Hans", "忠清南道"), ("zh", "忠清南道")]),
                        unofficial_name_list: ["South Chungchong"].to_vec(),
                    }
                ),
                (
                    "45",
                    Subdivision{
                        name: "45",
                        country_alpha2: Alpha2::KR,
                        code: "45",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جولا الشمالية"), ("be", "Паўночная Чала"), ("bg", "Чъла-Пукто"), ("bn", "উত\u{9cd}তর জেওল\u{9be}"), ("ca", "Jeollabuk-do"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄎\u{11128}𑄠\u{1112e}𑄣\u{11133}𑄦"), ("ceb", "Jeollabuk-do"), ("cs", "Severní Čolla"), ("cy", "Talaith Gogledd Jeolla"), ("da", "North Jeolla Province"), ("de", "Jeollabuk-do"), ("el", "Τζόλαμπουκ-ντο"), ("en", "North Jeolla"), ("es", "Jeolla del Norte"), ("et", "Põhja-Chŏlla provints"), ("eu", "Ipar Jeolla"), ("fa", "جئولابوک-دو"), ("fi", "Pohjois-Jeolla"), ("fr", "Jeolla du Nord"), ("gu", "ઉત\u{acd}તર જ\u{ac7}ઓલા પ\u{acd}રા\u{a82}ત"), ("he", "צפון ג׳אולה"), ("hi", "उत\u{94d}तरी ज\u{947}ओला प\u{94d}रा\u{902}त"), ("hu", "Észak-Csolla"), ("hy", "Հյուսիսային Ջոլա"), ("id", "Jeolla Utara"), ("it", "Nord Jeolla"), ("ja", "全羅北道"), ("jv", "Jeollabuk-do"), ("kk", "Чолла-Пукто"), ("kn", "ಉತ\u{ccd}ತರ ಜ\u{cc6}ಯೋಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "전라북도"), ("lt", "Šiaurės Džolos provincija"), ("lv", "Čollapukto province"), ("mn", "Умард Чолла"), ("mr", "उत\u{94d}तर ज\u{947}ओला प\u{94d}रा\u{902}त"), ("ms", "Jeolla Utara"), ("nb", "Nord-Jeolla"), ("nl", "Jeollabuk-do"), ("no", "Nord-Jeolla"), ("pl", "Jeolla Północna"), ("pt", "Jeolla do Norte"), ("ru", "Чолла-Пукто"), ("si", "උත\u{dd4}ර\u{dd4} ජෙයෝල\u{dca}ල\u{dcf} පළ\u{dcf}ත"), ("sv", "Nordjeolla"), ("sw", "Jeollabuk-do"), ("ta", "வடக\u{bcd}கு ஜெயல\u{bcd}ல ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర జ\u{c3f}య\u{c4b}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e47}อลลาเหน\u{e37}อ"), ("tr", "Kuzey Jeolla Province"), ("uk", "Північна провінція Чолла"), ("ur", "شمالی جئولا صوبہ"), ("vi", "Jeolla Bắc"), ("yue", "全羅北道"), ("yue_Hans", "全罗北道"), ("zh", "全羅北道")]),
                        unofficial_name_list: ["Chollapuk", "North Cholla"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::KR,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جولا الجنوبية"), ("be", "Паўднёвая Чала"), ("bg", "Чъла-Намдо"), ("bn", "দক\u{9cd}ষিণ জেওল\u{9be}"), ("ca", "Jeollanam-do"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄎\u{11128}𑄠\u{1112e}𑄣\u{11133}𑄦"), ("ceb", "Jeollanam-do"), ("cs", "Jižní Čolla"), ("cy", "Talaith De Jeolla"), ("da", "South Jeolla Province"), ("de", "Jeollanam-do"), ("el", "Τζόλαναμ-ντο"), ("en", "South Jeolla"), ("es", "Jeolla del Sur"), ("et", "Lõuna-Chŏlla provints"), ("eu", "Hego Jeolla"), ("fa", "جئولانام-دو"), ("fi", "Etelä-Jeolla"), ("fr", "Jeolla du Sud"), ("gu", "દક\u{acd}ષિણ જ\u{ac7}ઓલ\u{acd}લા પ\u{acd}રા\u{a82}ત"), ("he", "דרום ג׳אולה"), ("hi", "दक\u{94d}षिण ज\u{947}ओला प\u{94d}रा\u{902}त"), ("hu", "Dél-Csolla"), ("hy", "Հարավային Ջոլա"), ("id", "Jeolla Selatan"), ("it", "Sud Jeolla"), ("ja", "全羅南道"), ("kk", "Чолла-Намдо"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಜ\u{cc6}ಯೋಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "전라남도"), ("lt", "Pietų Džolos provincija"), ("lv", "Čollanamdo province"), ("mn", "Өмнөд Чолла"), ("mr", "दक\u{94d}षिण ज\u{947}ओला प\u{94d}रा\u{902}त"), ("ms", "Jeolla Selatan"), ("nb", "Sør-Jeolla"), ("ne", "दक\u{94d}षिण ज\u{947}ओला क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Jeollanam-do"), ("no", "Sør-Jeolla"), ("pl", "Jeolla Południowa"), ("pt", "Jeolla do Sul"), ("ru", "Чолла-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} ජෙඔල\u{dcf} පළ\u{dcf}ත"), ("sv", "Sydjeolla"), ("sw", "Jeollanam-do"), ("ta", "தெற\u{bcd}கு ஜெஒல\u{bcd}ல\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ జ\u{c3f}య\u{c4b}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e47}อลลาใต\u{e49}"), ("tr", "Güney Jeolla"), ("uk", "Південна провінція Чолла"), ("ur", "جنوبی جئولا صوبہ"), ("vi", "Jeolla Nam"), ("yue", "全羅南道"), ("yue_Hans", "全罗南道"), ("zh", "全羅南道")]),
                        unofficial_name_list: ["South Cholla"].to_vec(),
                    }
                ),
                (
                    "47",
                    Subdivision{
                        name: "47",
                        country_alpha2: Alpha2::KR,
                        code: "47",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.4919), longitude: Some(128.8889), max_latitude: Some(37.542778), min_latitude: Some(35.5664734), max_longitude: Some(130.9232178), min_longitude: Some(127.7938878)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غيونغسانغ الشمالية"), ("be", "Кёнсан-Пукта"), ("bg", "Кьонсан-Пукто"), ("bn", "উত\u{9cd}তর গিয\u{9bc}েওংস\u{9be}ং"), ("ca", "Gyeongsangbuk-do"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄎\u{11128}𑄠\u{11127}\u{11101}𑄥\u{11101}"), ("ceb", "Gyeongsangbuk-do"), ("cs", "Severní Kjongsang"), ("cy", "Talaith Gogledd Gyeongsang"), ("da", "North Gyeongsang Province"), ("de", "Gyeongsangbuk-do"), ("el", "Γκιόνγκσανγκμπουκ-ντο"), ("en", "North Gyeongsang"), ("es", "Gyeongsang del Norte"), ("et", "Põhja-Kyŏngsangi provints"), ("eu", "Ipar Gyeongsang"), ("fa", "جئونسانگبوک-دو"), ("fi", "Pohjois-Gyeongsang"), ("fr", "Gyeongsang du Nord"), ("gu", "ઉત\u{acd}તર જ\u{acd}યો\u{a82}ગ\u{acd}સા\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("he", "צפון גיאונגסאנג"), ("hi", "उत\u{94d}तरी ग\u{94d}यो\u{902}गस\u{948}\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Észak-Kjongszang"), ("hy", "Հյուսիսային Գյոնգսանգ"), ("id", "Gyeongsang Utara"), ("it", "Nord Gyeongsang"), ("ja", "慶尚北道"), ("kk", "Кёңсаң-Пукто"), ("kn", "ಉತ\u{ccd}ತರ ಜ\u{cbf}ಯಾಂಗ\u{ccd}ಶಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "경상북도"), ("lt", "Šiaurės Kiongsango provincija"), ("lv", "Kjonsanpukto province"), ("mn", "Умар Кёнсан"), ("mr", "उत\u{94d}तर ग\u{94d}या\u{901}गसा\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Gyeongsang Utara"), ("nb", "Nord-Gyeongsang"), ("nl", "Gyeongsangbuk-do"), ("no", "Nord-Gyeongsang"), ("pl", "Gyeongsang Północny"), ("pt", "Gyeongsang do Norte"), ("ro", "Gyeongsangbuk-do"), ("ru", "Кёнсан-Пукто"), ("si", "උත\u{dd4}ර\u{dd4} ගෙයෝන\u{dca}ග\u{dca}ස\u{dcf}න\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Nordgyeongsang"), ("sw", "Gyeongsangbuk-do"), ("ta", "வடக\u{bcd}கு ஜியொங\u{bcd}சங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర గ\u{c3f}య\u{c4b}ంగ\u{c4d}స\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคย\u{e47}องซ\u{e31}งเหน\u{e37}อ"), ("tr", "Kuzey Gyeongsang"), ("uk", "Північна провінція Кьонсан"), ("ur", "شمالی گیئونگسانگ صوبہ"), ("vi", "Gyeongsang Bắc"), ("yue", "慶尚北道"), ("yue_Hans", "庆尚北道"), ("zh", "庆尚北道")]),
                        unofficial_name_list: ["North Kyongsang"].to_vec(),
                    }
                ),
                (
                    "48",
                    Subdivision{
                        name: "48",
                        country_alpha2: Alpha2::KR,
                        code: "48",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.4606), longitude: Some(128.2132), max_latitude: Some(35.9099572), min_latitude: Some(34.5321655), max_longitude: Some(129.2198762), min_longitude: Some(127.5622163)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غيونغسانغ الجنوبية"), ("bg", "Кьонсан-Намдо"), ("bn", "দক\u{9cd}ষিণ গিয\u{9bc}েওংস\u{9be}ং"), ("ca", "Gyeongsangnam-do"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄎\u{11128}𑄠\u{11127}\u{11101}𑄥\u{11101}"), ("ceb", "Gyeongsangnam-do"), ("cs", "Jižní Kjongsang"), ("cy", "Talaith De Gyeongsang"), ("da", "South Gyeongsang Province"), ("de", "Gyeongsangnam-do"), ("el", "Γκιόνγκσανγκναμ-ντο"), ("en", "South Gyeongsang"), ("es", "Gyeongsang del Sur"), ("et", "Lõuna-Kyŏngsangi provints"), ("eu", "Hego Gyeongsang"), ("fa", "جئونسانگنام-دو"), ("fi", "Etelä-Gyeongsang"), ("fr", "Gyeongsang du Sud"), ("gu", "દક\u{acd}ષિણ જયો\u{a82}ગ\u{acd}સા\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("he", "דרום גיאונגסאנג"), ("hi", "दक\u{94d}षिण ग\u{947}यो\u{902}ग\u{94d}स\u{947}\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Dél-Kjongszang"), ("hy", "Հարավային Գյոնգսանգ"), ("id", "Gyeongsang Selatan"), ("it", "Sud Gyeongsang"), ("ja", "慶尚南道"), ("kk", "Кёнсан-Намдо"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಜ\u{cbf}ಯಾಂಗ\u{ccd}ಗಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "경상남도"), ("lt", "Pietų Kiongsango provincija"), ("lv", "Kjonsannamdo province"), ("mn", "Өмнө Кёнсан"), ("mr", "दक\u{94d}षिण ग\u{94d}या\u{901}गसा\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Gyeongsang Selatan"), ("nb", "Sør-Gyeongsang"), ("nl", "Gyeongsangnam-do"), ("no", "Sør-Gyeongsang"), ("pl", "Gyeongsang Południowy"), ("pt", "Gyeongsang do Sul"), ("ru", "Кёнсан-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} ජ\u{dd2}යෝන\u{dca}ග\u{dca}ස\u{dcf}න\u{dca}ග\u{dca}"), ("sk", "Južný Kjongsang"), ("sv", "Sydgyeongsang"), ("sw", "Gyeongsangnam-do"), ("ta", "தெற\u{bcd}கு குயென\u{bcd}க\u{bcd}சங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ గ\u{c3f}య\u{c3e}ంగ\u{c4d}స\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคย\u{e47}องซ\u{e31}งใต\u{e49}"), ("tr", "Güney Gyeongsang"), ("uk", "Південна провінція Кьонсан"), ("ur", "جنوبی گیئونگسانگ صوبہ"), ("vi", "Gyeongsang Nam"), ("yue", "慶尚南道"), ("yue_Hans", "庆尚南道"), ("zh", "庆尚南道")]),
                        unofficial_name_list: ["Gyeongsangnamdo/ Kyongsang-namdo/ South Kyongsang"].to_vec(),
                    }
                ),
                (
                    "49",
                    Subdivision{
                        name: "49",
                        country_alpha2: Alpha2::KR,
                        code: "49",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.4890113), longitude: Some(126.4983023), max_latitude: Some(34.0062218), min_latitude: Some(33.1061096), max_longitude: Some(126.9742813), min_longitude: Some(126.1637192)}),
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialSelfGoverningProvince,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جيجو"), ("az", "Çejudo adası"), ("be", "Чэджуда"), ("bg", "Чеджу-до"), ("bn", "জেজ\u{9c1} দ\u{9cd}বীপ"), ("ca", "Jeju-do"), ("ccp", "𑄎𑄬𑄎\u{1112a}"), ("ceb", "Jeju-do"), ("cs", "Čedžu"), ("da", "Jeju-do"), ("de", "Jeju-do"), ("el", "Τζετζού"), ("en", "Jeju"), ("es", "Jeju-do"), ("et", "Cheju provints"), ("eu", "Jeju"), ("fa", "ججو-دو"), ("fi", "Jeju"), ("fr", "Jeju-do"), ("gl", "Jeju"), ("gu", "જ\u{ac7}જ\u{ac1}"), ("he", "ג׳ג׳ו"), ("hi", "ज\u{947}ज\u{942}"), ("hr", "Jeju"), ("hu", "Csedzsu-sziget"), ("hy", "Ջեջու նահանգ"), ("id", "Jeju"), ("it", "Jeju-do"), ("ja", "済州特別自治道"), ("jv", "Jejudo"), ("kk", "Чеджудо"), ("kn", "ಜ\u{cc6}ಜು"), ("ko", "제주특별자치도"), ("lt", "Čedžu"), ("lv", "Čendžu"), ("mk", "Чеџу"), ("mr", "ज\u{947}ज\u{942}"), ("ms", "Jeju"), ("nb", "Jeju"), ("ne", "ज\u{947}ज\u{941} प\u{94d}रान\u{94d}त"), ("nl", "Jeju-do"), ("no", "Jeju"), ("pl", "Czedżu"), ("pt", "Jeju"), ("ro", "Insula Jeju"), ("ru", "Чеджудо"), ("si", "ජෙජ\u{dd4} ද\u{dd6}පත"), ("sr", "Чеџу"), ("sr_Latn", "Čedžu"), ("sv", "Jeju"), ("sw", "Jeju-do"), ("ta", "ஜேஜூ"), ("te", "జ\u{c47}జూ"), ("th", "จ\u{e31}งหว\u{e31}ดเชจ\u{e39}"), ("tr", "Jeju"), ("uk", "Провінція Чеджу"), ("ur", "جیجو صوبہ"), ("vi", "Jeju"), ("yue", "濟州特別自治道"), ("yue_Hans", "济州特别自治道"), ("zh", "濟州特別自治道")]),
                        unofficial_name_list: ["Jeju", "Quelpart"].to_vec(),
                    }
                ),
                (
                    "50",
                    Subdivision{
                        name: "50",
                        country_alpha2: Alpha2::KR,
                        code: "50",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialSelfGoverningCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مدينة سيجونغ"), ("bn", "সেজং শহর"), ("ccp", "𑄥𑄬𑄎\u{11127}\u{11101}"), ("ceb", "Sejong-si"), ("cs", "Sedžong"), ("da", "Sejong"), ("de", "Sejong"), ("el", "Σετζόνγκ"), ("en", "Sejong"), ("es", "Sejong"), ("eu", "Sejong"), ("fa", "سجونگ سیتی"), ("fi", "Sejong City"), ("fr", "Sejong"), ("gu", "સિઝો\u{a82}ગ શહ\u{ac7}ર"), ("hi", "स\u{947}जो\u{902}ग सिटी"), ("hu", "Szedzsong (település)"), ("hy", "Սեջոնգ"), ("id", "Kota Sejong"), ("it", "Città di Sejong"), ("ja", "世宗特別自治市"), ("kk", "Сечжон"), ("kn", "ಸ\u{cc6}ಹೋಂಗ\u{ccd} ನಗರ"), ("ko", "세종특별자치시"), ("lt", "Sedžongas"), ("lv", "Sedžonga"), ("mn", "Сэжун хот"), ("mr", "स\u{947}जोन\u{94d}ग शहर"), ("ms", "Sejong"), ("nb", "Sejong by"), ("nl", "Sejong"), ("no", "Sejong by"), ("pl", "Sedżong"), ("pt", "Cidade de Sejong"), ("ru", "Седжон"), ("si", "ස\u{dd2}ජෝන\u{dca}ග\u{dca} නගරය"), ("sv", "Sejong City"), ("ta", "ச\u{bc0}ஜொங\u{bcd} நகரம\u{bcd}"), ("te", "స\u{c46}జ\u{c3e}ంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องเซจง"), ("tr", "Sejong"), ("uk", "Седжон"), ("ur", "سیجونگ شہر"), ("vi", "Sejong"), ("yue", "世宗特別自治市"), ("yue_Hans", "世宗特别自治市"), ("zh", "世宗特別自治市")]),
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
#[cfg(feature = "kr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::KR,
        alpha3: Alpha3::KOR,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}}\n{{postalcode}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 82,
        currency_code: "KRW",
        gec: Some(GEC::KS),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "001",
        ioc: Some("KOR"),
        iso_long_name: "The Republic of Korea",
        iso_short_name: "Korea (Republic of)",
        official_language_list: ["ko"].to_vec(),
        spoken_language_list: ["ko"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("South Korean"),
        number: "410",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "KR",
        unofficial_name_list: [
            "South Korea",
            "Korea (South)",
            "Südkorea",
            "Corée du Sud",
            "Corea del Sur",
            "大韓民国",
            "Zuid-Korea",
            "Korea (Republic of)",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "South Korea"),
            ("af", "Suid-Korea"),
            ("ak", "South Korea"),
            ("am", "ደቡብ ኮሪያ"),
            ("an", "South Korea"),
            ("ar", "كوريا، جمهوري\u{651}ة كوريا"),
            ("as", "কোৰিয়\u{9be} প\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰ"),
            ("ay", "South Korea"),
            ("az", "Koreya, Respublika"),
            ("ba", "South Korea"),
            ("be", "Карэя, Рэспубліка"),
            ("bg", "Корея, Република"),
            ("bi", "South Korea"),
            ("bn", "কোরিয়\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
            ("bn_IN", "কোরিয়\u{9be} প\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}র"),
            ("br", "South Korea"),
            ("bs", "Koreja, Republika"),
            ("ca", "Corea del Sud"),
            ("ce", "South Korea"),
            ("ch", "South Korea"),
            ("cs", "Korea, republika"),
            ("cv", "South Korea"),
            ("cy", "Corëa, Gweriniaeth"),
            ("da", "Korea, Republikken"),
            ("de", "Südkorea"),
            ("dv", "South Korea"),
            (
                "dz",
                "ཀ\u{f7c}་ར\u{f72}་ཡ། ར\u{f72}་པཔ་ལ\u{f72}ཀ་ ཨ\u{f7c}ཕ་",
            ),
            ("ee", "South Korea"),
            ("el", "Κορέα, Δημοκρατία της"),
            ("en", "South Korea"),
            ("eo", "Sud-Koreio"),
            ("es", "Corea, República de"),
            ("et", "Lõuna-Korea"),
            ("eu", "Korea, Errepublika"),
            ("fa", "جمهوری کره"),
            ("ff", "South Korea"),
            ("fi", "Korean tasavalta"),
            ("fo", "South Korea"),
            ("fr", "Corée du Sud"),
            ("fy", "South Korea"),
            ("ga", "Poblacht na Cóiré"),
            ("gl", "Corea, República de"),
            ("gn", "South Korea"),
            ("gu", "દક\u{acd}ષિણ આફ\u{acd}રિકા"),
            ("gv", "South Korea"),
            ("ha", "South Korea"),
            ("he", "קוריאה הדרומית"),
            ("hi", "दक\u{94d}षिण कोरिया"),
            ("hr", "Južna Korea"),
            ("ht", "South Korea"),
            ("hu", "Koreai Köztársaság"),
            ("hy", "Կորեա Հանարպետություն"),
            ("ia", "Corea, Republica de"),
            ("id", "Korea Selatan"),
            ("io", "South Korea"),
            ("is", "Suður-Kórea"),
            ("it", "Corea del sud"),
            ("iu", "South Korea"),
            ("ja", "大韓民国 (韓国)"),
            ("ka", "კორეა, რესპუბლიკა"),
            ("ki", "South Korea"),
            ("kk", "Корея Республикасы"),
            ("kl", "South Korea"),
            (
                "km",
                "ក\u{17bc}រ\u{17c9}េ សាធារណរដ\u{17d2}ឋ\u{200b}នៃរ\u{17c9}េ",
            ),
            ("kn", "ಕೋರ\u{cbf}ಯಾ ಗಣರಾಜ\u{ccd}ಯ"),
            ("ko", "대한민국"),
            ("ku", "Kore, Komara"),
            ("kv", "South Korea"),
            ("kw", "South Korea"),
            ("ky", "Корея Республикасы"),
            ("lo", "South Korea"),
            ("lt", "Korėjos Respublika"),
            ("lv", "Dienvidkoreja"),
            ("mi", "South Korea"),
            ("mk", "Кореја, Република"),
            ("ml", "കൊറിയ, റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"),
            ("mn", "Солонгос ард улс"),
            ("mr", "कोरिया, रिपब\u{94d}लिक ऑफ"),
            ("ms", "South Korea"),
            ("mt", "South Korea"),
            ("my", "South Korea"),
            ("na", "South Korea"),
            ("nb", "Sør-Korea"),
            ("ne", "कोरियाको गणराज\u{94d}य"),
            ("nl", "Zuid-Korea"),
            ("nn", "Sør-Korea"),
            ("nv", "South Korea"),
            ("oc", "Corèa del Sud"),
            ("or", "କୋର\u{b3f}ଆ, ଗଣତନ\u{b4d}ତ\u{b4d}ର"),
            ("pa", "ਕ\u{a4b}ਰੀਆ ਗਣਰਾਜ"),
            ("pi", "South Korea"),
            ("pl", "Korea Południowa"),
            ("ps", "South Korea"),
            ("pt", "Coreia do Sul"),
            ("pt_BR", "Coreia do Sul"),
            ("ro", "Republica Coreea"),
            ("ru", "Южная Корея"),
            ("rw", "Koreya, Repubulika ya"),
            ("sc", "Corea de su Sud"),
            ("sd", "South Korea"),
            ("si", "කොර\u{dd2}ය\u{dcf} ජනරජය"),
            ("sk", "Kórejská republika"),
            ("sl", "Južna Koreja"),
            ("so", "South Korea"),
            ("sq", "Korea, Republika e"),
            ("sr", "Кореја, Република"),
            ("sv", "Sydkorea"),
            ("sw", "South Korea"),
            ("ta", "கொரிய குடியரசு"),
            (
                "te",
                "క\u{c4b}ర\u{c3f}య\u{c3e}, ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}",
            ),
            ("tg", "Ҷумҳурии Корея"),
            ("th", "เกาหล\u{e35}ใต\u{e49}"),
            ("ti", "South Korea"),
            ("tk", "Koreýa Respublikasy"),
            ("tl", "Korea, Republika ng"),
            ("tr", "Güney Kore"),
            ("tt", "Кореа Җөмһүриәте"),
            ("ug", "كورېيە"),
            ("uk", "Південна Корея"),
            ("ur", "South Korea"),
            ("uz", "South Korea"),
            ("ve", "South Korea"),
            ("vi", "Cộng hoà Nam Hàn"),
            ("wa", "Corêye (nonnrece)"),
            ("wo", "Koore, Republik bu"),
            ("xh", "South Korea"),
            ("yo", "South Korea"),
            ("zh_CN", "韩国"),
            ("zh_HK", "大韓民國"),
            ("zh_TW", "大韓民國"),
            ("zu", "South Korea"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
