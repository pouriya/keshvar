// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Ukraine

#[cfg(all(feature = "ua", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}}\n{{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::UA;
    pub const ALPHA3: Alpha3 = Alpha3::UKR;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 380;
    pub const CURRENCY_CODE: &str = "UAH";
    pub const GEC: Option<GEC> = Some(GEC::UP);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::UKR);
    pub const ISO_SHORT_NAME: &str = "Ukraine";
    pub const ISO_LONG_NAME: &str = "Ukraine";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["uk"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["uk"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Ukrainian");
    pub const NUMBER: &str = "804";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "UA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Ukraine",
        "Ucrania",
        "ウクライナ",
        "Oekraïne",
        "Украина",
        "Україна",
        "Украіна",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Ukraine"),
        ("af", "Oekraïne"),
        ("ak", "Ukraine"),
        ("am", "ዩክሬን"),
        ("an", "Ukraine"),
        ("ar", "أوكرانيا"),
        ("as", "ইউক\u{9cd}ৰেইন"),
        ("ay", "Ukraine"),
        ("az", "Ukrayna"),
        ("ba", "Ukraine"),
        ("be", "Украіна"),
        ("bg", "Украйна"),
        ("bi", "Ukraine"),
        ("bn", "ইউক\u{9cd}রেন"),
        ("bn_IN", "ইউক\u{9cd}রেন"),
        ("br", "Ukraina"),
        ("bs", "Ukrajina"),
        ("ca", "Ucraïna"),
        ("ce", "Украина"),
        ("ch", "Ukraine"),
        ("cs", "Ukrajina"),
        ("cv", "Украина"),
        ("cy", "Wcrain"),
        ("da", "Ukraine"),
        ("de", "Ukraine"),
        ("dv", "ޔ\u{7aa}ކ\u{7b0}ރ\u{7ac}އ\u{7a8}ނ\u{7b0}"),
        ("dz", "ཡ\u{f74}་ཀ\u{f7a}རན།"),
        ("ee", "Ukraine"),
        ("el", "Ουκρανία"),
        ("en", "Ukraine"),
        ("eo", "Ukrainio"),
        ("es", "Ucrania"),
        ("et", "Ukraina"),
        ("eu", "Ukrania"),
        ("fa", "اکراین"),
        ("ff", "Ukrayiina"),
        ("fi", "Ukraina"),
        ("fo", "Ukreina"),
        ("fr", "Ukraine"),
        ("fy", "Oekraïne"),
        ("ga", "An Úcráin"),
        ("gl", "Ucraína"),
        ("gn", "Ukraine"),
        ("gu", "ય\u{ac1}ક\u{acd}ર\u{ac7}ન"),
        ("gv", "Yn Ookraan"),
        ("ha", "Ukraniya"),
        ("he", "אוקראינה"),
        ("hi", "य\u{941}क\u{94d}र\u{947}न"),
        ("hr", "Ukrajina"),
        ("ht", "Ikrèn"),
        ("hu", "Ukrajna"),
        ("hy", "Ուկրաինա"),
        ("ia", "Ukraina"),
        ("id", "Ukraina"),
        ("io", "Ukrainia"),
        ("is", "Úkraína"),
        ("it", "Ucraina"),
        ("iu", "ᑯᑯᓯ ᓄᓇ"),
        ("ja", "ウクライナ"),
        ("ka", "უკრაინა"),
        ("ki", "Ukraine"),
        ("kk", "Украина"),
        ("kl", "Ukraine"),
        ("km", "អ\u{17ca}\u{17bb}យក\u{17d2}រែន"),
        ("kn", "ಯುಕ\u{ccd}ರೇನ\u{ccd}"),
        ("ko", "우크라이나"),
        ("ku", "Ukrayna"),
        ("kv", "Украина"),
        ("kw", "Ukrayn"),
        ("ky", "Украина"),
        ("lo", "ປະເທດອ\u{eb9}ກະແລນ"),
        ("lt", "Ukraina"),
        ("lv", "Ukraina"),
        ("mi", "Ūkareinga"),
        ("mk", "Украина"),
        ("ml", "ഉക\u{d4d}രൈന\u{d4d}\u{200d}"),
        ("mn", "Украйн"),
        ("mr", "य\u{941}क\u{94d}र\u{947}न"),
        ("ms", "Ukraine"),
        ("mt", "Ukranja"),
        (
            "my",
            "ယ\u{1030}ကရ\u{102d}န\u{103a}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ukraine"),
        ("nb", "Ukraina"),
        ("ne", "य\u{941}क\u{94d}र\u{947}न"),
        ("nl", "Oekraïne"),
        ("nn", "Ukraina"),
        ("nv", "Yóókwein"),
        ("oc", "Ucraïna"),
        ("or", "ୟ\u{b41}କ\u{b4d}ରେନ"),
        ("pa", "ਯ\u{a42}ਕਰ\u{a47}ਨ"),
        ("pi", "Ukraine"),
        ("pl", "Ukraina"),
        ("ps", "اوکراین"),
        ("pt", "Ucrânia"),
        ("pt_BR", "Ucrânia"),
        ("ro", "Ucraina"),
        ("ru", "Украина"),
        ("rw", "Ikerene"),
        ("sc", "Ucraina"),
        ("sd", "يوڪرين"),
        ("si", "ය\u{dd4}ක\u{dca}රේනය"),
        ("sk", "Ukrajina"),
        ("sl", "Ukrajina"),
        ("so", "Ukraine"),
        ("sq", "Ukrainë"),
        ("sr", "Украјина"),
        ("sv", "Ukraina"),
        ("sw", "Ukraine"),
        ("ta", "உக\u{bcd}ரெயின\u{bcd}"),
        ("te", "యుక\u{c4d}ర\u{c48}న\u{c4d}"),
        ("tg", "Украина"),
        ("th", "ย\u{e39}เครน"),
        ("ti", "ዩክረይን"),
        ("tk", "Ukraina"),
        ("tl", "Ukraine"),
        ("tr", "Ukrayna"),
        ("tt", "Украин"),
        ("ug", "ئۇكرائىنا"),
        ("uk", "Україна"),
        ("ur", "یوکرین"),
        ("uz", "Ukraina"),
        ("ve", "Ukraine"),
        ("vi", "U-cờ-rai-na"),
        ("wa", "Oucrinne"),
        ("wo", "Ukreen"),
        ("xh", "Ukraine"),
        ("yo", "Ukréìn"),
        ("zh_CN", "乌克兰"),
        ("zh_HK", "烏克蘭"),
        ("zh_TW", "烏克蘭"),
        ("zu", "I-Yukreyini"),
    ];
    #[cfg(all(feature = "ua", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 48.379433;
        pub const LONGITUDE: f64 = 31.1655799;
        pub const MAX_LATITUDE: f64 = 52.3793713;
        pub const MAX_LONGITUDE: f64 = 40.2204802;
        pub const MIN_LATITUDE: f64 = 44.2924;
        pub const MIN_LONGITUDE: f64 = 22.137159;
        pub const NORTHEAST_LATITUDE: f64 = 52.3793713;
        pub const NORTHEAST_LONGITUDE: f64 = 40.2204802;
        pub const SOUTHWEST_LATITUDE: f64 = 44.2924;
        pub const SOUTHWEST_LONGITUDE: f64 = 22.137159;
    }
}
#[cfg(all(feature = "ua", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 48.379433,
            longitude: 31.1655799,
            max_latitude: 52.3793713,
            max_longitude: 40.2204802,
            min_latitude: 44.2924,
            min_longitude: 22.137159,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 52.3793713,
                    longitude: 40.2204802,
                },
                southwest: CountryGeoBound {
                    latitude: 44.2924,
                    longitude: 22.137159,
                },
            },
        }
    }
}

#[cfg(all(feature = "ua", feature = "subdivisions"))]
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
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::UA,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.233083), longitude: Some(28.468217), max_latitude: Some(49.889537), min_latitude: Some(48.064971), max_longitude: Some(30.022071), min_longitude: Some(27.37479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Winnitsja-oblast"), ("ar", "فينيتسا أوبلاست"), ("az", "Vinnitsya vilayəti"), ("be", "Вінніцкая вобласць"), ("bg", "Виницка област"), ("bn", "ভিনিৎসিয\u{9bc}\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Vinička oblast"), ("ca", "Província de Vínnitsia"), ("ccp", "𑄞\u{11128}𑄚\u{11133}𑄦\u{11128}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Vinnyts’ka Oblast’"), ("cs", "Vinnycká oblast"), ("da", "Vinnitsja oblast"), ("de", "Oblast Winnyzja"), ("el", "Βιννύτσια Ομπλάστ"), ("en", "Vinnychchyna"), ("es", "Óblast de Vinnytsia"), ("et", "Vinnõtsja oblast"), ("eu", "Vinnytsiako oblasta"), ("fa", "استان وینیتسیا"), ("fi", "Vinnytsjan alue"), ("fr", "Oblast de Vinnytsia"), ("gl", "Oblast de Vinnitsia"), ("gu", "વિનિટ\u{acd}સિયા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ויניצה"), ("hi", "विनित\u{94d}सिया ओब\u{94d}लास\u{94d}ट"), ("hr", "Vinička oblast"), ("hu", "Vinnicjai terület"), ("hy", "Վիննիցայի մարզ"), ("id", "Oblast Vinnytsia"), ("it", "Oblast’ di Vinnycja"), ("ja", "ヴィーンヌィツャ州"), ("ka", "ვინიცის ოლქი"), ("kn", "ವ\u{cbf}ನ\u{ccd}ನ\u{cbf}ತ\u{ccd}ಸ\u{cbf}ಯಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "빈니차 주"), ("ky", "Винница облусу"), ("lt", "Vinycios sritis"), ("lv", "Vinnicas apgabals"), ("mk", "Виницка област"), ("mn", "Винниц муж"), ("mr", "व\u{94d}हिनित\u{94d}सिया ओब\u{94d}लास\u{94d}त"), ("ms", "Vinnytsia Oblast"), ("nb", "Vinnytsia oblast"), ("nl", "Oblast Vinnytsja"), ("no", "Vinnytsia oblast"), ("pl", "Obwód winnicki"), ("pt", "Oblast de Vinnitsa"), ("ro", "Regiunea Vinița"), ("ru", "Винницкая область"), ("si", "ව\u{dd2}න\u{dd2}ට\u{dca}ස\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vinnycká oblasť"), ("sl", "Viniška oblast"), ("sr", "Виничка област"), ("sr_Latn", "Vinička oblast"), ("sv", "Vinnytsia oblast"), ("ta", "வின\u{bcd}னிட\u{bcd}ஸ\u{bcd}ய\u{bbe} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "వ\u{c3f}న\u{c4d}న\u{c3f}ట\u{c4d}స\u{c3f}య\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานม\u{e35}เกล"), ("tr", "Vinnitsa Oblastı"), ("uk", "Вінницька область"), ("ur", "وینیتسیا اوبلاست"), ("uz", "Vinnitsa viloyati"), ("vi", "Vinnytsia"), ("zh", "文尼察州")]),
                        unofficial_name_list: ["Vinnica", "Vinnitsa", "Vinnytska", "Vinnytsya"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::UA,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.74723299999999), longitude: Some(25.325383), max_latitude: Some(51.969238), min_latitude: Some(50.2871931), max_longitude: Some(26.1062831), min_longitude: Some(23.603933)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wolhinië-oblast"), ("ar", "فولين أوبلاست"), ("az", "Volın vilayəti"), ("be", "Валынская вобласць"), ("bg", "Волинска област"), ("bn", "ভোলিন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Volinjska oblast"), ("ca", "Província de Volínia"), ("ccp", "𑄞\u{1112e}𑄣\u{11128}𑄠𑄚\u{11134}"), ("ceb", "Volyns’ka Oblast’"), ("cs", "Volyňská oblast"), ("da", "Volyn oblast"), ("de", "Oblast Wolhynien"), ("el", "Βολίν Ομπλάστ"), ("en", "Volyn"), ("es", "Óblast de Volyn"), ("et", "Volõõnia oblast"), ("eu", "Voliniako oblasta"), ("fa", "استان ولین"), ("fi", "Volynian alue"), ("fr", "Oblast de Volhynie"), ("gu", "વોલિન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ווהלין"), ("hi", "वोलिन ओब\u{94d}लास\u{94d}ट"), ("hr", "Volinjska oblast"), ("hu", "Volinyi terület"), ("hy", "Վոլինի մարզ"), ("id", "Oblast Volyn"), ("it", "Oblast’ di Volinia"), ("ja", "ヴォルィーニ州"), ("ka", "ვოლინის ოლქი"), ("kn", "ವೋಲ\u{cbf}ನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "볼린 주"), ("ky", "Волынь областы"), ("lt", "Voluinės sritis"), ("lv", "Volīnijas apgabals"), ("mk", "Волинска област"), ("mn", "Волынь муж"), ("mr", "व\u{94d}होलिन ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Volyn"), ("nb", "Volyn oblast"), ("nl", "Oblast Wolynië"), ("no", "Volyn oblast"), ("pl", "Obwód wołyński"), ("pt", "Oblast de Volínia"), ("ro", "Regiunea Volîn"), ("ru", "Волынская область"), ("si", "වොල\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Volynská oblasť"), ("sr", "Волињска област"), ("sr_Latn", "Volinjska oblast"), ("sv", "Volyn oblast"), ("ta", "வோலின\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "వ\u{c4b}ల\u{c3f}న\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "โวลน\u{e4c} โอแบลส"), ("tr", "Volın Oblastı"), ("uk", "Волинська область"), ("ur", "ولین اوبلاست"), ("uz", "Volin viloyati"), ("vi", "Volyn"), ("zh", "沃倫州")]),
                        unofficial_name_list: ["Volyn", "Volynska"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::UA,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.574041), longitude: Some(39.307815), max_latitude: Some(50.088428), min_latitude: Some(47.825031), max_longitude: Some(40.2275119), min_longitude: Some(37.83751609999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loehansk-oblast"), ("ar", "لوهانسك أوبلاست"), ("az", "Luqansk vilayəti"), ("be", "Луганская вобласць"), ("bg", "Луганска област"), ("bn", "ল\u{9c1}হ\u{9be}নস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("ca", "Província de Luhansk"), ("ccp", "𑄣\u{1112a}𑄦𑄚\u{11134}𑄌\u{11133}𑄌\u{11128}𑄚"), ("ceb", "Luhans’ka Oblast’"), ("cs", "Luhanská oblast"), ("da", "Lugansk oblast"), ("de", "Oblast Luhansk"), ("el", "Λουχάνσκ Ομπλάστ"), ("en", "Luhanshchyna"), ("es", "Óblast de Lugansk"), ("et", "Luganski oblast"), ("eu", "Luhanskeko oblasta"), ("fa", "استان لوهانسک"), ("fi", "Luhanskin alue"), ("fr", "Oblast de Louhansk"), ("ga", "Cúige Luhansk"), ("gu", "લ\u{ac1}હાન\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז לוהנסק"), ("hi", "ल\u{941}हान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Luhanska oblast"), ("hu", "Luhanszki terület"), ("hy", "Լուգանսկի մարզ"), ("id", "Oblast Luhansk"), ("it", "Oblast’ di Luhans’k"), ("ja", "ルハーンシク州"), ("ka", "ლუგანსკის ოლქი"), ("kn", "ಲುಹನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "루한스크 주"), ("ky", "Луганск областы"), ("lt", "Luhansko sritis"), ("lv", "Luhanskas apgabals"), ("mk", "Луганска област"), ("mn", "Луганск муж"), ("mr", "ल\u{941}हान\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Luhansk Oblast"), ("nb", "Luhansk oblast"), ("nl", "Oblast Loehansk"), ("no", "Luhansk oblast"), ("pl", "Obwód ługański"), ("pt", "Oblast de Lugansk"), ("ro", "Regiunea Luhansk"), ("ru", "Луганская область"), ("si", "ල\u{dd4}හන\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Luhanská oblasť"), ("sl", "Luhanska Oblast"), ("sr", "Луганска област"), ("sr_Latn", "Luganska oblast"), ("sv", "Luhansk oblast"), ("ta", "லுஹ\u{bbe}ன\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "లూహ\u{c3e}ంక\u{c4d}స\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ล\u{e39}ฮ\u{e31}นซก อบลาส"), ("tr", "Luhansk Oblastı"), ("uk", "Луганська область"), ("ur", "لوہانسک اوبلاست"), ("uz", "Lugansk viloyati"), ("vi", "Luhansk"), ("zh", "盧甘斯克州")]),
                        unofficial_name_list: ["Lugansk", "Luhanska", "Luhansʿk", "Voroshilovgrad", "Vorošilovgrad"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::UA,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.464717), longitude: Some(35.046183), max_latitude: Some(49.193473), min_latitude: Some(47.481922), max_longitude: Some(36.9364431), min_longitude: Some(32.959522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Dnipropetrofsk-oblast"), ("ar", "دنيبروبتروفسك أوبلاست"), ("az", "Dnepropetrovsk vilayəti"), ("be", "Днепрапятроўская вобласць"), ("bg", "Днепропетровска област"), ("bn", "নিপ\u{9cd}রপেত\u{9cd}রভস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Dnjipropetrovska oblast"), ("ca", "Província de Dnipropetrovsk"), ("ccp", "𑄓\u{11128}𑄛\u{11133}𑄢\u{11127}𑄛𑄬𑄑\u{11133}𑄢\u{1112e}𑄛\u{11134}𑄌\u{11133}𑄌\u{11128}𑄚"), ("ceb", "Dnipropetrovska Oblast’"), ("cs", "Dněpropetrovská oblast"), ("da", "Dnipropetrovsk oblast"), ("de", "Oblast Dnipropetrowsk"), ("el", "Ντνιπροπετρόβσκ"), ("en", "Dnipropetrovshchyna"), ("es", "Óblast de Dnipropetrovsk"), ("et", "Dnipropetrovski oblast"), ("eu", "Dnipropetrovskeko oblasta"), ("fa", "استان دنیپروپتروفسک"), ("fi", "Dnepropetrovskin alue"), ("fr", "Oblast de Dnipropetrovsk"), ("gl", "Rexión de Dnipropetrovsk"), ("gu", "ડીનીપ\u{acd}રોપ\u{ac7}ટ\u{acd}રોવસ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "דנפרופטרובסק"), ("hi", "नीप\u{94d}रोप\u{947}ट\u{94d}रोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Dnjipropetrovska oblast"), ("hu", "Dnyipropetrovszki terület"), ("hy", "Դնեպրոպետրովսկի մարզ"), ("id", "Oblast Dnipropetrovsk"), ("it", "Oblast’ di Dnipropetrovs’k"), ("ja", "ドニプロペトロウシク州"), ("ka", "დნეპროპეტროვსკის ოლქი"), ("kn", "ದ\u{ccd}ನ\u{cbf}ಪ\u{ccd}ರೋಪ\u{cc6}ತ\u{ccd}ರೋವ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "드니프로페트로우스크 주"), ("lt", "Dniepropetrovsko sritis"), ("lv", "Dņepropetrovskas apgabals"), ("mk", "Днепропетровска област"), ("mn", "Днепропетровск муж"), ("mr", "द\u{94d}न\u{947}प\u{94d}रोप\u{947}त\u{94d}रोव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Dnipropetrovsk Oblast"), ("nb", "Dnipropetrovsk oblast"), ("nl", "Oblast Dnjepropetrovsk"), ("no", "Dnipropetrovsk oblast"), ("pl", "Obwód dniepropetrowski"), ("pt", "Oblast de Dnipropetrovsk"), ("ro", "Regiunea Dnipropetrovsk"), ("ru", "Днепропетровская область"), ("si", "ඩ\u{dca}න\u{dd2}ප\u{dca}රෝපෙට\u{dca}\u{200d}රෝව\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Dnepropetrovská oblasť"), ("sr", "Дњепропетровска област"), ("sr_Latn", "Dnjepropetrovska oblast"), ("sv", "Dnipropetrovsk oblast"), ("ta", "டணிப\u{bcd}ரோபெட\u{bcd}ரோவ\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "డ\u{c3f}న\u{c46}ప\u{c4d}ర\u{c4a}ప\u{c46}ట\u{c4d}ర\u{c4b}వస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ดเนโปรเปตรอฟสก\u{e4c} โอบลาส"), ("tr", "Dnipropetrovsk Oblastı"), ("uk", "Дніпропетровська область"), ("ur", "دنیپروپیترووسک اوبلاست"), ("uz", "Dnepropetrovsk viloyati"), ("vi", "Dnipropetrovsk"), ("zh", "第聂伯罗彼得罗夫斯克州")]),
                        unofficial_name_list: ["Dnepropetrovsk", "Dnipropetrovsk", "Dnipropetrovska", "Dnjepropetrovsk"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::UA,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.015883), longitude: Some(37.80285), max_latitude: Some(49.236797), min_latitude: Some(46.867733), max_longitude: Some(39.09210179999999), min_longitude: Some(36.541492)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Donetsk-oblast"), ("ar", "دونيتسك أوبلاست"), ("az", "Donetsk Oblastı"), ("be", "Данецкая вобласць"), ("bg", "Донецка област"), ("bn", "ডোনেস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("ca", "Província de Donetsk"), ("ccp", "𑄓\u{1112e}𑄚𑄬𑄌\u{11133}𑄌\u{11128}𑄚"), ("ceb", "Donets’ka Oblast’"), ("cs", "Doněcká oblast"), ("da", "Donetsk Oblast"), ("de", "Oblast Donezk"), ("el", "Ντόνετσκ Ομπλάστ"), ("en", "Donechchyna"), ("es", "Óblast de Donetsk"), ("et", "Donetski oblast"), ("eu", "Donetskeko oblasta"), ("fa", "استان دونتسک"), ("fi", "Donetskin alue"), ("fr", "Oblast de Donetsk"), ("gu", "ડોન\u{ac7}ટ\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז דונצק"), ("hi", "डोन\u{947}ट\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Donečka oblast"), ("hu", "Donecki terület"), ("hy", "Դոնեցկի մարզ"), ("id", "Oblast Donetsk"), ("it", "Oblast’ di Donec’k"), ("ja", "ドネツィク州"), ("ka", "დონეცკის ოლქი"), ("kn", "ಡೊನ\u{cc6}ಟ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "도네츠크 주"), ("lt", "Donecko sritis"), ("lv", "Doņeckas apgabals"), ("mk", "Донечка област"), ("mn", "Донецк муж"), ("mr", "दोन\u{947}त\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Donetsk Oblast"), ("nb", "Donetsk oblast"), ("nl", "Oblast Donetsk"), ("no", "Donetsk oblast"), ("pl", "Obwód doniecki"), ("pt", "Oblast de Donetsk"), ("ro", "Regiunea Donețk"), ("ru", "Донецкая область"), ("si", "ඩොනේට\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Donecká oblasť"), ("sr", "Доњечка област"), ("sr_Latn", "Donječka oblast"), ("sv", "Donetsk oblast"), ("ta", "டநிட\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "డ\u{c3e}న\u{c46}స\u{c4d}క\u{c4d}"), ("th", "โดเนตสค\u{e4c}"), ("tr", "Donetsk Oblastı"), ("uk", "Донецька область"), ("ur", "دونیتسک اوبلاست"), ("uz", "Donetsk viloyati"), ("vi", "Donetsk (tỉnh)"), ("yue", "頓涅茨克州"), ("yue_Hans", "顿涅茨克州"), ("zh", "顿涅茨克州")]),
                        unofficial_name_list: ["Doneck", "Donetska", "Donetsʿka"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::UA,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.25465), longitude: Some(28.658667), max_latitude: Some(51.6818959), min_latitude: Some(49.58293099999999), max_longitude: Some(29.7354618), min_longitude: Some(27.1897231)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sjitomir-oblast"), ("ar", "زيتومير أوبلاست"), ("az", "Jitomir vilayəti"), ("be", "Жытомірская вобласць"), ("bg", "Житомирска област"), ("bn", "জ\u{9cd}যটোম\u{9be}র ওপল\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Žitomirska oblast"), ("ca", "Província de Jitòmir"), ("ccp", "𑄎\u{1112d}𑄑\u{1112e}𑄟\u{11128}𑄠𑄢\u{11134}𑄌\u{11133}𑄌\u{11128}𑄚"), ("ceb", "Zhytomyrs’ka Oblast’"), ("cs", "Žytomyrská oblast"), ("cy", "Zhytomyr Oblast"), ("da", "Zjitomir oblast"), ("de", "Oblast Schytomyr"), ("el", "Ζιτομίρ Ομπλάστ"), ("en", "Zhytomyrshchyna"), ("es", "Óblast de Zhytomyr"), ("et", "Žõtomõri oblast"), ("eu", "Zhytomyrko oblasta"), ("fa", "استان ژیتومیر"), ("fi", "Žytomyrin alue"), ("fr", "Oblast de Jytomyr"), ("gu", "ઝિટોમીર ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז ז׳יטומיר"), ("hi", "ज\u{93c}ायटोमीर ओब\u{94d}लास\u{94d}ट"), ("hr", "Žitomirska oblast"), ("hu", "Zsitomiri terület"), ("hy", "Ժիտոմիրի մարզ"), ("id", "Oblast Zhytomyr"), ("is", "Zjytómýrfylki"), ("it", "Oblast’ di Žytomyr"), ("ja", "ジトームィル州"), ("ka", "ჟიტომირის ოლქი"), ("kn", "ಜೈಥೊಮ\u{cbf}ರ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "지토미르 주"), ("lt", "Žytomyro sritis"), ("lv", "Žitomiras apgabals"), ("mk", "Житомирска област"), ("mn", "Житомир муж"), ("mr", "झितोमिर ओब\u{94d}लास\u{94d}त"), ("ms", "Zhytomyr Oblast"), ("nb", "Zjytomyr oblast"), ("nl", "Oblast Zjytomyr"), ("no", "Zjytomyr oblast"), ("pl", "Obwód żytomierski"), ("pt", "Oblast de Jitomir"), ("ro", "Regiunea Jîtomîr"), ("ru", "Житомирская область"), ("si", "ස\u{dd2}ටෝම\u{dd3}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Žytomyrská oblasť"), ("sr", "Житомирска област"), ("sr_Latn", "Žitomirska oblast"), ("sv", "Zjytomyr oblast"), ("ta", "ழயடோமிர\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "జ\u{c48}ట\u{c4b}మ\u{c3f}ర\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ไซทอม เบลส"), ("tr", "Jitomir Oblastı"), ("uk", "Житомирська область"), ("ur", "ژیتومیر اوبلاست"), ("uz", "Jitomir viloyati"), ("vi", "Zhytomyr"), ("zh", "日托米爾州")]),
                        unofficial_name_list: ["Zhitomir", "Zhytomyrska"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::UA,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.6208), longitude: Some(22.287883), max_latitude: Some(49.09755699999999), min_latitude: Some(47.896507), max_longitude: Some(24.627378), min_longitude: Some(22.135906)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Transkarpatië-oblast"), ("ar", "زاكارباتيا أوبلاست"), ("az", "Zakarpattya vilayəti"), ("be", "Закарпацкая вобласць"), ("bg", "Закарпатска област"), ("bn", "জ\u{9be}ক\u{9be}রপ\u{9be}ত\u{9cd}তিয\u{9bc}\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Zakarpatska oblast"), ("ca", "Transcarpàcia"), ("ccp", "𑄎𑄇𑄢\u{11134}𑄛𑄑\u{11133}𑄦\u{11128}𑄠"), ("ceb", "Zakarpattia Oblast"), ("cs", "Zakarpatská oblast"), ("da", "Zakarpatska oblast"), ("de", "Oblast Transkarpatien"), ("el", "Ζακαρπάτια"), ("en", "Zakarpattia"), ("es", "Óblast de Zakarpatia"), ("et", "Taga-Karpaatia"), ("eu", "Transkarpatiako oblasta"), ("fa", "استان زاکارپیتا"), ("fi", "Taka-Karpatian alue"), ("fr", "Oblast de Transcarpatie"), ("gu", "ઝકારપ\u{ac7}ટ\u{acd}ટીયા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "זקרפטיה"), ("hi", "ज\u{93c}कारपटिया ओब\u{94d}लास\u{94d}ट"), ("hr", "Zakarpatska oblast"), ("hu", "Kárpátalja"), ("hy", "Անդրկարպատյան մարզ"), ("id", "Oblast Zakarpattia"), ("it", "Oblast’ di Transcarpazia"), ("ja", "ザカルパッチャ州"), ("ka", "ტრანსკარპატიის ოლქი"), ("kn", "ಜಕಾರ\u{ccd}ಪಟ\u{ccd}ಯಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "자카르파탸 주"), ("ky", "Закарпатье"), ("lt", "Užkarpatės sritis"), ("lv", "Aizkarpatu apgabals"), ("mk", "Закарпатска област"), ("mn", "Закарпат муж"), ("mr", "झाकारपत\u{94d}तिया ओब\u{94d}लास\u{94d}त"), ("ms", "Zakarpattia Oblast"), ("nb", "Zakarpattja oblast"), ("nl", "Oblast Transkarpatië"), ("no", "Zakarpattja oblast"), ("pl", "Obwód zakarpacki"), ("pt", "Oblast da Transcarpátia"), ("ro", "Regiunea Transcarpatia"), ("ru", "Закарпатская область"), ("si", "සකර\u{dca}පට\u{dca}ට\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Zakarpatská oblasť"), ("sr", "Закарпатска област"), ("sr_Latn", "Zakarpatska oblast"), ("sv", "Zakarpattia oblast"), ("ta", "சக\u{bcd}கர\u{bcd}பட\u{bcd}டிய ஓப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "జక\u{c3e}ర\u{c4d}ప\u{c3e}ట\u{c3f}య\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลซาคาร\u{e4c}แพทเท\u{e35}ย"), ("tr", "Zakarpatya Oblastı"), ("uk", "Закарпатська область"), ("ur", "زاکارپتیا اوبلاست"), ("uz", "Zakarpatye viloyati"), ("vi", "Vùng Zakarpattia"), ("zh", "外喀爾巴阡州")]),
                        unofficial_name_list: ["Transcarpathia", "Zakarpatska", "Zakarpatsʿka Oblast"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::UA,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.8388), longitude: Some(35.139567), max_latitude: Some(48.14402), min_latitude: Some(46.2642371), max_longitude: Some(37.2450479), min_longitude: Some(34.245512)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saporisja-oblast"), ("ar", "زابوروجييه أوبلاست"), ("az", "Zaporojya vilayəti"), ("be", "Запарожская вобласць"), ("bg", "Запорожка област"), ("bn", "জ\u{9cd}য\u{9be}পরিঝিঝিয\u{9bc}\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Zaporiška oblast"), ("ca", "Província de Zaporíjjia"), ("ccp", "𑄎𑄛\u{1112e}𑄢\u{11128}𑄎\u{11133}𑄎\u{11128}𑄠"), ("ceb", "Zaporiz’ka Oblast’"), ("cs", "Záporožská oblast"), ("da", "Zaporizjzja oblast"), ("de", "Oblast Saporischschja"), ("el", "Ζαποριζγία Ομπλάστ"), ("en", "Zaporizhzhya"), ("es", "Óblast de Zaporiyia"), ("et", "Zaporižžja oblast"), ("eu", "Zaporizhiako oblasta"), ("fa", "استان زاپروژیا"), ("fi", "Zaporižžjan alue"), ("fr", "Oblast de Zaporijia"), ("ga", "Cúige Zaporizhia"), ("gu", "ઝ\u{ac7}પોરિઝઝયા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "זפורוז׳יה"), ("hi", "ज\u{93c}\u{948}पोरिझ\u{94d}झया ओब\u{94d}लास\u{94d}ट"), ("hr", "Zaporiška oblast"), ("hu", "Zaporizzsjai terület"), ("hy", "Զապորոժիեի մարզ"), ("id", "Oblast Zaporizhia"), ("is", "Zapórizjyskfylki"), ("it", "Oblast’ di Zaporižžja"), ("ja", "ザポリージャ州"), ("ka", "ზაპოროჟიეს ოლქი"), ("kn", "ಜಪೊರ\u{cbf}ಝ\u{ccd}ಝ\u{ccd}ಲಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "자포리자 주"), ("lt", "Zaporožės sritis"), ("lv", "Zaporožjes apgabals"), ("mk", "Запорошка област"), ("mn", "Запорожье муж"), ("mr", "झापोरिझिया ओब\u{94d}लास\u{94d}त"), ("ms", "Zaporizhzhya Oblast"), ("nb", "Zaporizjzja oblast"), ("nl", "Oblast Zaporizja"), ("no", "Zaporizjzja oblast"), ("pl", "Obwód zaporoski"), ("pt", "Oblast de Zaporíjia"), ("ro", "Regiunea Zaporijjea"), ("ru", "Запорожская область"), ("si", "සැපොර\u{dd2}ස\u{dca}ශ\u{dca}\u{200d}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Záporožská oblasť"), ("sr", "Запорошка област"), ("sr_Latn", "Zaporoška oblast"), ("sv", "Zaporizjzja oblast"), ("ta", "சபோரிலலய\u{bbe} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "జ\u{c3e}ప\u{c4a}ర\u{c40}జ\u{c3f}హ\u{c3f}య\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ร\u{e31}ฐซานควน"), ("tr", "Zaporijya Oblastı"), ("uk", "Запорізька область"), ("ur", "زابروژیا اوبلاست"), ("uz", "Zaporijya viloyati"), ("vi", "Zaporizhia"), ("zh", "扎波羅熱州")]),
                        unofficial_name_list: ["Zaporizhzhya", "Zaporizka", "Zaporozhye", "Zaporožje"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::UA,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.922633), longitude: Some(24.711117), max_latitude: Some(49.559585), min_latitude: Some(47.724266), max_longitude: Some(25.6529901), min_longitude: Some(23.545547)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Iwano-Frankifsk-oblast"), ("ar", "إيفانو فرانكيفسك أوبلاست"), ("az", "İvano-Frankivsk vilayəti"), ("be", "Івана-Франкоўская вобласць"), ("bg", "Ивано-Франковска област"), ("bn", "ইভ\u{9be}নো-ফ\u{9cd}র\u{9be}ঙ\u{9cd}কিভস\u{9cd}ক ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Ivano-Frankivska oblast"), ("ca", "Província d’Ivano-Frankivsk"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄇𑄢\u{11134}𑄛𑄑\u{11133}𑄦\u{11128}𑄠"), ("ceb", "Ivano-Frankivs’ka Oblast’"), ("cs", "Ivanofrankivská oblast"), ("da", "Ivano-Frankivsk oblast"), ("de", "Oblast Iwano-Frankiwsk"), ("el", "Ιβάνο-Φρανκίβσκ Ομπλάστ"), ("en", "Prykarpattia"), ("es", "Óblast de Ivano-Frankivsk"), ("et", "Ivano-Frankivski oblast"), ("eu", "Ivano-Frankivskeko oblasta"), ("fa", "استان ایوانو فرانکیسوک"), ("fi", "Ivano-Frankivskin alue"), ("fr", "Oblast d’Ivano-Frankivsk"), ("gu", "ઇવાનો-ફ\u{acd}ર\u{ac7}ન\u{acd}કીસ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז איוונו-פרנקיבסק"), ("hi", "इव\u{948}नो-फ\u{94d}र\u{948}\u{902}किव\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}ट"), ("hr", "Ivano-Frankivska oblast"), ("hu", "Ivano-frankivszki terület"), ("hy", "Իվանո-Ֆրանկովսկի մարզ"), ("id", "Oblast Ivano-Frankivsk"), ("it", "Oblast’ di Ivano-Frankivs’k"), ("ka", "ივანო-ფრანკოვსკის ოლქი"), ("kn", "ಇವಾನೋ-ಫ\u{ccd}ರಾಂಕ\u{cbf}ವ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "이바노프란키우스크 주"), ("lt", "Ivano Frankivsko sritis"), ("lv", "Ivanofrankivskas apgabals"), ("mk", "Ивано-Франковска област"), ("mn", "Ивано-Франковск муж"), ("mr", "इव\u{94d}हानो-फ\u{94d}रा\u{902}किव\u{94d}ह\u{94d}स\u{94d}क ओब\u{94d}लास\u{94d}त"), ("ms", "Ivano-Frankivsk Oblast"), ("nb", "Ivano-Frankivsk oblast"), ("nl", "Oblast Ivano-Frankivsk"), ("no", "Ivano-Frankivsk oblast"), ("pl", "Obwód iwanofrankiwski"), ("pt", "Oblast de Ivano-Frankivsk"), ("ro", "Regiunea Ivano-Frankivsk"), ("ru", "Ивано-Франковская область"), ("si", "ඉව\u{dcf}නෝ ෆ\u{dca}රන\u{dca}ක\u{dd2}ව\u{dca}ස\u{dca}ක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ivanofrankivská oblasť"), ("sr", "Ивано-Франкивска област"), ("sr_Latn", "Ivano-Frankivska oblast"), ("sv", "Ivano-Frankivsk oblast"), ("ta", "இவனோ -பிர\u{bbe}ன\u{bcd}கிவ\u{bcd}ஸ\u{bcd}க\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ఇవ\u{c3e}న\u{c4b}-ఫ\u{c4d}ర\u{c3e}ంక\u{c3f}వస\u{c4d}క\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "อ\u{e35}วาโนฟร\u{e31}งค\u{e35}ฟสค\u{e4c}"), ("tr", "İvano-Frankivsk Oblastı"), ("uk", "Івано-Франківська область"), ("ur", "ایوانو-فرانکیوسک اوبلاست"), ("uz", "Ivano-frankivsk viloyati"), ("vi", "Ivano-Frankivsk"), ("zh", "伊万诺-弗兰科夫斯克州")]),
                        unofficial_name_list: ["Ivano-Frankivsk", "Ivano-Frankivska", "Ivano-Frankovsk"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::UA,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.4501), longitude: Some(30.5234), max_latitude: Some(50.590798), min_latitude: Some(50.213273), max_longitude: Some(30.825941), min_longitude: Some(30.2394401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kiëf"), ("am", "ኪየቭ"), ("ar", "كييف"), ("az", "Kiyev"), ("be", "Кіеў"), ("bg", "Киев"), ("bn", "কিয\u{9bc}েভ"), ("bs", "Kijev"), ("ca", "Kíev"), ("ccp", "𑄇\u{11128}𑄠𑄬𑄛\u{11134}"), ("ceb", "Kiev"), ("cs", "Kyjev"), ("cy", "Kiev"), ("da", "Kijev"), ("de", "Kiew"), ("el", "Κίεβο"), ("en", "Kyiv"), ("es", "Kiev"), ("et", "Kiiev"), ("eu", "Kiev"), ("fa", "کی\u{200c}یف"), ("fi", "Kiova"), ("fr", "Kiev"), ("ga", "Cív"), ("gl", "Kiev"), ("gu", "ક\u{acd}યીવ"), ("he", "קייב"), ("hi", "कीव"), ("hr", "Kijev"), ("hu", "Kijev"), ("hy", "Կիև"), ("id", "Kiev"), ("is", "Kíev"), ("it", "Kiev"), ("ja", "キエフ"), ("jv", "Kiev"), ("ka", "კიევი"), ("kk", "Киев"), ("kn", "ಕೀವ\u{ccd}"), ("ko", "키예프"), ("ky", "Киев"), ("lt", "Kijevas"), ("lv", "Kijeva"), ("mk", "Киев"), ("ml", "കീവ\u{d4d}"), ("mn", "Киев"), ("mr", "क\u{94d}यीव"), ("ms", "Kiev"), ("my", "က\u{102e}းယက\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Kiev"), ("nl", "Kiev"), ("no", "Kiev"), ("pa", "ਕੀਵ"), ("pl", "Kijów"), ("ps", "کيېف"), ("pt", "Kiev"), ("ro", "Kiev"), ("ru", "Киев"), ("si", "ක\u{dd2}ව\u{dca}"), ("sk", "Kyjev"), ("sl", "Kijev"), ("sq", "Kievi"), ("sr", "Кијев"), ("sr_Latn", "Kijev"), ("sv", "Kiev"), ("sw", "Kiev"), ("ta", "க\u{bc0}வ\u{bcd}"), ("te", "క\u{c4d}య\u{c3f}వ\u{c4d}"), ("th", "เค\u{e35}ยฟ"), ("tk", "Kyýiw"), ("tr", "Kiev"), ("uk", "Київ"), ("ur", "کیف"), ("uz", "Kiyev"), ("vi", "Kiev"), ("yo", "Kiev"), ("yo_BJ", "Kiev"), ("yue", "基輔"), ("yue_Hans", "基辅"), ("zh", "基輔"), ("zu", "IKiyevi")]),
                        unofficial_name_list: ["Kiev", "Kyiv", "Kyyiv"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::UA,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.0529506), longitude: Some(30.7667133), max_latitude: Some(51.554014), min_latitude: Some(49.1791191), max_longitude: Some(32.160736), min_longitude: Some(29.2664181)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kiëf-oblast"), ("ar", "كييف أوبلاست"), ("az", "Kiyev vilayəti"), ("be", "Кіеўская вобласць"), ("bg", "Киевска област"), ("bn", "কিভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kijevska oblast"), ("ca", "Província de Kíev"), ("ccp", "𑄇\u{11128}𑄠𑄅\u{1112a}𑄛\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Kyiv Oblast"), ("cs", "Kyjevská oblast"), ("da", "Kijev oblast"), ("de", "Oblast Kyjiw"), ("el", "Κιβ Ομπλάστ"), ("en", "Kyivshchyna"), ("es", "Óblast de Kiev"), ("et", "Kiievi oblast"), ("eu", "Kieveko oblasta"), ("fa", "استان کیف"), ("fi", "Kiovan alue"), ("fr", "Oblast de Kyiv"), ("gu", "ક\u{acd}યીવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קייב"), ("hi", "कीव ओब\u{94d}लास\u{94d}ट"), ("hr", "Kijevska oblast"), ("hu", "Kijevi terület"), ("hy", "Կիևի մարզ"), ("id", "Oblast Kiev"), ("it", "Oblast’ di Kiev"), ("ja", "キエフ州"), ("ka", "კიევის ოლქი"), ("kn", "ಕೀವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "키예프 주"), ("lt", "Kijevo sritis"), ("lv", "Kijevas apgabals"), ("mk", "Киевска област"), ("mn", "Киев муж"), ("mr", "क\u{94d}यीव ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Kiev"), ("nb", "Kiev oblast"), ("nl", "Oblast Kiev"), ("no", "Kiev oblast"), ("pl", "Obwód kijowski"), ("pt", "Oblast de Kiev"), ("ro", "Regiunea Kiev"), ("ru", "Киевская область"), ("si", "ක\u{dd2}ව\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Kyjevská oblasť"), ("sr", "Кијевска област"), ("sr_Latn", "Kijevska oblast"), ("sv", "Kiev oblast"), ("ta", "கிளிவ\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ఖ\u{c4d}య\u{c3f}వ\u{c4d}"), ("th", "เม\u{e37}องหลวงเค\u{e35}ยฟ"), ("tr", "Kiev Oblastı"), ("uk", "Київська область"), ("ur", "کیف اوبلاست"), ("uz", "Kiyev viloyati"), ("vi", "Kiev²"), ("zh", "基辅州")]),
                        unofficial_name_list: ["Kyyivsʿka Oblast", "Kyyivsʿka Oblastʿ"].to_vec(),
                    }
                ),
                (
                    "35",
                    Subdivision{
                        name: "35",
                        country_alpha2: Alpha2::UA,
                        code: "35",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.50793300000001), longitude: Some(32.262317), max_latitude: Some(49.1651489), min_latitude: Some(47.749134), max_longitude: Some(33.8891529), min_longitude: Some(29.749174)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kirowohrad-oblast"), ("ar", "كيروفوهراد أوبلاست"), ("az", "Kirovoqrad vilayəti"), ("be", "Кіраваградская вобласць"), ("bg", "Кировоградска област"), ("bn", "কিরোভোর\u{9be}দ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Kirovogradska oblast"), ("ca", "Província de Kirovohrad"), ("ccp", "𑄇\u{11128}𑄢\u{1112e}𑄞\u{1112e}𑄦\u{11134}𑄢𑄖\u{11134}𑄌\u{11128}𑄚"), ("ceb", "Kirovohrads’ka Oblast’"), ("cs", "Kirovohradská oblast"), ("da", "Kirovograd oblast"), ("de", "Oblast Kirowohrad"), ("el", "Κιροβοχράντ"), ("en", "Kirovohradschyna"), ("es", "Óblast de Kirovogrado"), ("et", "Kirovogradi oblast"), ("eu", "Kirovohradeko oblasta"), ("fa", "استان کیرووهراد"), ("fi", "Kirovohradin alue"), ("fr", "Oblast de Kirovohrad"), ("gu", "કિરોવોહ\u{acd}રાદ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז קירובוגרד"), ("hi", "किरोवोरॉड ओब\u{94d}लास\u{94d}ट"), ("hr", "Kirovogradska oblast"), ("hu", "Kirovohradi terület"), ("hy", "Կիրովոգրադի մարզ"), ("id", "Oblast Kirovohrad"), ("it", "Oblast’ di Kirovohrad"), ("ja", "キロヴォフラード州"), ("ka", "კიროვოგრადის ოლქი"), ("kn", "ಕ\u{cbf}ರೊವೊಹ\u{ccd}ರಾಡ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "키로보흐라드 주"), ("lt", "Kirovohrado sritis"), ("lv", "Kirovogradas apgabals"), ("mk", "Кировоградска област"), ("mn", "Кировоград муж"), ("mr", "किरोव\u{94d}होराद ओब\u{94d}लास\u{94d}त"), ("ms", "Kirovohrad Oblast"), ("nb", "Kirovohrad oblast"), ("nl", "Oblast Kirovohrad"), ("no", "Kirovohrad oblast"), ("pl", "Obwód kirowohradzki"), ("pt", "Oblast de Kirovogrado"), ("ro", "Regiunea Kirovohrad"), ("ru", "Кировоградская область"), ("si", "ක\u{dd2}රෝවෝහ\u{dca}ර\u{dcf}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kirovohradská oblasť"), ("sr", "Кировоградска област"), ("sr_Latn", "Kirovogradska oblast"), ("sv", "Kirovohrad oblast"), ("ta", "கிரோவோஹ\u{bcd}ர\u{bbe}ட\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "క\u{c3f}ర\u{c4b}వ\u{c4b}హ\u{c4d}ర\u{c3e}డ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e35}โรโวฮราด"), ("tr", "Kirovograd Oblastı"), ("uk", "Кіровоградська область"), ("ur", "کیرووہراد اوبلاست"), ("uz", "Kirovograd viloyati"), ("vi", "Kirovohrad"), ("zh", "基洛夫格勒州")]),
                        unofficial_name_list: ["Kirovograd", "Kirovohrad", "Kirovohradska"].to_vec(),
                    }
                ),
                (
                    "40",
                    Subdivision{
                        name: "40",
                        country_alpha2: Alpha2::UA,
                        code: "40",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.61665), longitude: Some(33.525367), max_latitude: Some(44.841316), min_latitude: Some(44.387115), max_longitude: Some(33.897497), min_longitude: Some(33.3785472)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sewastopol"), ("ar", "سيفاستوبول"), ("az", "Sevastopol"), ("be", "Севастопаль"), ("bg", "Севастопол"), ("bn", "সেভ\u{9be}স\u{9cd}তোপোল"), ("bs", "Sevastopolj"), ("ca", "Sebastòpol"), ("ccp", "𑄥𑄬𑄞𑄌\u{11134}𑄑\u{1112e}𑄛\u{1112e}𑄣\u{11134}"), ("ceb", "Sevastopol"), ("cs", "Sevastopol"), ("cy", "Sevastopol"), ("da", "Sevastopol"), ("de", "Sewastopol"), ("el", "Σεβαστούπολη"), ("en", "Sevastopol"), ("es", "Sebastopol"), ("et", "Sevastopol"), ("eu", "Sebastopol"), ("fa", "سواستوپول"), ("fi", "Sevastopol"), ("fr", "Sébastopol"), ("gl", "Sebastopol"), ("gu", "સ\u{ac7}વાસ\u{acd}તોપોલ"), ("he", "סבסטופול"), ("hi", "स\u{947}वस\u{94d}तोपोल"), ("hr", "Sevastopolj"), ("hu", "Szevasztopol"), ("hy", "Սևաստոպոլ"), ("id", "Sevastopol"), ("is", "Sevastópol"), ("it", "Sebastopoli"), ("ja", "セヴァストポリ"), ("ka", "სევასტოპოლი"), ("kn", "ಸ\u{cc6}ವಸ\u{ccd}ಟಾಪೋಲ\u{ccd}"), ("ko", "세바스토폴"), ("ky", "Севастополь"), ("lt", "Sevastopolis"), ("lv", "Sevastopole"), ("mk", "Севастопол"), ("mn", "Севастополь"), ("mr", "स\u{947}व\u{94d}हास\u{94d}तोपोल"), ("ms", "Sevastopol"), ("my", "ဆ\u{102e}ပတ\u{103a}စတ\u{102d}\u{102f}ပ\u{102d}\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Sevastopol"), ("nl", "Sebastopol"), ("no", "Sevastopol"), ("pa", "ਸ\u{a47}ਵਾਸਤ\u{a4b}ਪ\u{a4b}ਲ"), ("pl", "Sewastopol"), ("pt", "Sebastopol"), ("ro", "Sevastopol"), ("ru", "Севастополь"), ("si", "සේවස\u{dca}ටෝපොල\u{dca}"), ("sk", "Sevastopoľ"), ("sl", "Sevastopol"), ("sr", "Севастопољ"), ("sr_Latn", "Sevastopolj"), ("sv", "Sevastopol"), ("sw", "Sevastopol"), ("ta", "செவஸ\u{bcd}டோபோல\u{bcd}"), ("te", "స\u{c46}వ\u{c3e}స\u{c4d}ట\u{c4b}ప\u{c4b}ల\u{c4d}"), ("th", "เซว\u{e31}สโตปอล"), ("tk", "Sewastopol"), ("tr", "Sivastopol"), ("uk", "Севастополь"), ("ur", "سواستوپول"), ("uz", "Sevastopol"), ("vi", "Sevastopol"), ("yue", "塞凡堡"), ("yue_Hans", "塞凡堡"), ("zh", "塞瓦斯托波爾")]),
                        unofficial_name_list: ["Sebastopol", "Sevastopol"].to_vec(),
                    }
                ),
                (
                    "43",
                    Subdivision{
                        name: "43",
                        country_alpha2: Alpha2::UA,
                        code: "43",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.952117), longitude: Some(34.102417), max_latitude: Some(46.2291611), min_latitude: Some(44.3864399), max_longitude: Some(36.6467392), min_longitude: Some(32.4792759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Republic,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Outonome Republiek van Krim"), ("ar", "جمهورية القرم المستقلة ذاتيا\u{64b}"), ("az", "Krım Muxtar Respublikası"), ("be", "Аўтаномная Рэспубліка Крым"), ("bg", "Автономна република Крим"), ("bn", "ক\u{9cd}রিমিয\u{9bc}\u{9be}"), ("bs", "Autonomna Republika Krim"), ("ca", "República Autònoma de Crimea"), ("ccp", "𑄇\u{11133}𑄢\u{11128}𑄟𑄠"), ("ceb", "Autonomous Republic of Crimea"), ("cs", "Autonomní republika Krym"), ("cy", "Gweriniaeth Hunanlywodraethol y Crimea"), ("da", "Autonome Republik Krim"), ("de", "Autonome Republik Krim"), ("el", "Αυτόνομη Δημοκρατία της Κριμαίας"), ("en", "Crimea"), ("es", "República Autónoma de Crimea"), ("et", "Krimmi Autonoomne Vabariik"), ("eu", "Krimeako Errepublika Autonomoa"), ("fa", "جمهوری خودمختار کریمه"), ("fi", "Krimin autonominen tasavalta"), ("fr", "République autonome de Crimée"), ("ga", "Poblacht Fhéinrialaitheach na Crimé"), ("gl", "República Autónoma de Crimea"), ("he", "הרפובליקה האוטונומית של קרים"), ("hr", "Autonomna Republika Krim"), ("hu", "Krími Autonóm Köztársaság"), ("hy", "Ղրիմի Ինքնավար Հանրապետություն"), ("id", "Republik Otonom Krimea"), ("it", "Repubblica Autonoma di Crimea"), ("ja", "クリミア自治共和国"), ("jv", "Republik Otonom Krimea"), ("ka", "ყირიმის ავტონომიური რესპუბლიკა"), ("kk", "Қырым Автономиялық Республикасы"), ("ko", "크림 자치 공화국"), ("ky", "Крым Республикасы"), ("lt", "Krymo autonominė respublika"), ("lv", "Krimas Autonomā Republika"), ("mk", "Автономна Република Крим"), ("ml", "ഓട\u{d4d}ടോണോമസ\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d} ക\u{d4d}രിമിയ"), ("mn", "Бүгд Найрамдах Өөртөө Засах Крым Улс"), ("ms", "Republik Autonomi Krimea"), ("nb", "Den autonome republikken Krim"), ("nl", "Autonome Republiek van de Krim"), ("no", "Den autonome republikken Krim"), ("pl", "Republika Autonomiczna Krymu"), ("pt", "República Autónoma da Crimeia"), ("ro", "Republica Autonomă Crimeea"), ("ru", "Автономная Республика Крым"), ("sk", "Krymská autonómna republika"), ("sl", "Avtonomna republika Krim"), ("sq", "Republika Autonome e Krimesë"), ("sr", "Аутономна Република Крим"), ("sr_Latn", "Autonomna Republika Krim"), ("sv", "Autonoma republiken Krim"), ("ta", "கிரிமிய\u{bbe} தன\u{bcd}ன\u{bbe}ட\u{bcd}சிக\u{bcd} குடியரசு"), ("te", "క\u{c4d}ర\u{c3f}మ\u{c3f}య\u{c3e}"), ("th", "สาธารณร\u{e31}ฐปกครองตนเองไครเม\u{e35}ย"), ("tk", "Awtonom Respublikasy Krym"), ("tr", "Kırım Özerk Cumhuriyeti"), ("uk", "Автономна Республіка Крим"), ("ur", "خود مختار جمہوریہ کریمیا"), ("uz", "Qrim Muxtor Respublikasi"), ("vi", "Cộng hòa Tự trị Krym"), ("yue", "克里米亞自治共和國"), ("yue_Hans", "克里米亚自治共和国"), ("zh", "克里米亚自治共和国")]),
                        unofficial_name_list: ["Crimea", "Krim"].to_vec(),
                    }
                ),
                (
                    "46",
                    Subdivision{
                        name: "46",
                        country_alpha2: Alpha2::UA,
                        code: "46",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.839683), longitude: Some(24.029717), max_latitude: Some(50.6488831), min_latitude: Some(48.7189779), max_longitude: Some(25.426912), min_longitude: Some(22.6406759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lwif-oblast"), ("ar", "لفيف أوبلاست"), ("az", "Lvov vilayəti"), ("be", "Львоўская вобласць"), ("bg", "Лвовска област"), ("bn", "লিভিভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Lavovska oblast"), ("ca", "Província de Lviv"), ("ccp", "𑄞\u{1112d}𑄛\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "L’vivs’ka Oblast’"), ("cs", "Lvovská oblast"), ("da", "Lviv oblast"), ("de", "Oblast Lwiw"), ("el", "Λβίβ Ομπλάστ"), ("en", "Lvivshchyna"), ("es", "Óblast de Leópolis"), ("et", "Lvivi oblast"), ("eu", "Lviveko oblasta"), ("fa", "استان لووف"), ("fi", "Lvivin alue"), ("fr", "Oblast de Lviv"), ("gu", "લ\u{acd}વિવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז לבוב"), ("hi", "लवीव ओब\u{94d}लास\u{94d}ट"), ("hr", "Ljvivska oblast"), ("hu", "Lvivi terület"), ("hy", "Լվովի մարզ"), ("id", "Oblast Lviv"), ("it", "Oblast’ di Leopoli"), ("ja", "リヴィウ州"), ("ka", "ლვოვის ოლქი"), ("kn", "ಎಲ\u{ccd}ವ\u{cbf}ವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "리비우 주"), ("lt", "Lvovo sritis"), ("lv", "Ļvovas apgabals"), ("mk", "Лавовска област"), ("mn", "Львов муж"), ("mr", "लिव\u{94d}हिव ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Lviv"), ("nb", "Lviv oblast"), ("nl", "Oblast Lviv"), ("no", "Lviv oblast"), ("pl", "Obwód lwowski"), ("pt", "Oblast de Lviv"), ("ro", "Regiunea Liov"), ("ru", "Львовская область"), ("si", "එල\u{dca}ව\u{dd2}ව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ľvovská oblasť"), ("sr", "Лавовска област"), ("sr_Latn", "Lavovska oblast"), ("sv", "Lviv oblast"), ("ta", "லிவிவ\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ల\u{c46}వ\u{c3f}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "ล\u{e34}ว อ\u{e4a}ปลาส"), ("tr", "Lviv Oblastı"), ("uk", "Львівська область"), ("ur", "لویئو اوبلاست"), ("uz", "Lviv viloyati"), ("vi", "Lviv"), ("zh", "利沃夫州")]),
                        unofficial_name_list: ["Lvivska", "Lvov", "Lʿviv"].to_vec(),
                    }
                ),
                (
                    "48",
                    Subdivision{
                        name: "48",
                        country_alpha2: Alpha2::UA,
                        code: "48",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.975033), longitude: Some(31.994583), max_latitude: Some(48.2316741), min_latitude: Some(46.36890890000001), max_longitude: Some(33.183647), min_longitude: Some(30.2075529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mikolajif-oblast"), ("ar", "ميكولايف أوبلاست"), ("az", "Nikolayev vilayəti"), ("be", "Мікалаеўская вобласць"), ("bg", "Николаевска област"), ("bn", "ম\u{9be}ইকোল\u{9be}ইভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Mikolajivska oblast"), ("ca", "Província de Mikolàiv"), ("ccp", "𑄟\u{1112d}𑄇\u{1112e}𑄣𑄬𑄠\u{11128}𑄛\u{11134}𑄌\u{11128}𑄚"), ("ceb", "Mykolayivs’ka Oblast’"), ("cs", "Nikolajevská oblast"), ("da", "Mikolaiv oblast"), ("de", "Oblast Mykolajiw"), ("el", "Μγικολάιβ Ομπλάστ"), ("en", "Mykolayivschyna"), ("es", "Óblast de Mykolaiv"), ("et", "Mõkolajivi oblast"), ("eu", "Mykolaiveko oblasta"), ("fa", "استان میکولائیف"), ("fi", "Mykolajivin alue"), ("fr", "Oblast de Mykolaïv"), ("gu", "માયકોલાઈવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז מיקולאייב"), ("hi", "मायकोल\u{947}व ओब\u{94d}लास\u{94d}ट"), ("hr", "Mikolajivska oblast"), ("hu", "Mikolajivi terület"), ("hy", "Նիկոլաևի մարզ"), ("id", "Oblast Mykolaiv"), ("it", "Oblast’ di Mykolaïv"), ("ja", "ムィコラーイウ州"), ("ka", "ნიკოლაევის ოლქი"), ("kn", "ಮೈಕೊಲೈವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "미콜라이우 주"), ("ky", "Николаев областы"), ("lt", "Mykolajivo sritis"), ("lv", "Nikolajevas apgabals"), ("mk", "Николаевска област"), ("mn", "Николаев муж"), ("mr", "मिकोलाइव\u{94d}ह ओब\u{94d}लास\u{94d}त"), ("ms", "Mykolaiv Oblast"), ("nb", "Mykolajiv oblast"), ("nl", "Oblast Mykolajiv"), ("no", "Mykolajiv oblast"), ("pl", "Obwód mikołajowski"), ("pt", "Oblast de Mikolaiv"), ("ro", "Regiunea Mîkolaiiv"), ("ru", "Николаевская область"), ("si", "ම\u{dd2}කොලය\u{dd2}ව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mykolajivská oblasť"), ("sr", "Миколајивска област"), ("sr_Latn", "Mikolajivska oblast"), ("sv", "Mykolajiv oblast"), ("ta", "மிக\u{bcd}கோளைவ\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "మ\u{c48}క\u{c4b}ల\u{c3e}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e34}โคลาเยฟ"), ("tr", "Mıkolayiv Oblastı"), ("uk", "Миколаївська область"), ("ur", "میکولائیو اوبلاست"), ("uz", "Mikolayiv viloyati"), ("vi", "Mykolaiv"), ("zh", "尼古拉耶夫州")]),
                        unofficial_name_list: ["Mykolayivsk", "Mykolayivska", "Nikolajev", "Nikolayev"].to_vec(),
                    }
                ),
                (
                    "51",
                    Subdivision{
                        name: "51",
                        country_alpha2: Alpha2::UA,
                        code: "51",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.484583), longitude: Some(30.7326), max_latitude: Some(48.233306), min_latitude: Some(45.204082), max_longitude: Some(31.305305), min_longitude: Some(28.211238)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Odessa-oblast"), ("ar", "أوديسا أوبلاست"), ("az", "Odessa vilayəti"), ("be", "Адэская вобласць"), ("bg", "Одеска област"), ("bn", "ওডেস\u{9cd}য\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Odeska oblast"), ("ca", "Província d’Odessa"), ("ccp", "𑄃\u{1112e}𑄓𑄬𑄌\u{11134}𑄌\u{11128}𑄚"), ("ceb", "Odes’ka Oblast’"), ("cs", "Oděská oblast"), ("da", "Odessa oblast"), ("de", "Oblast Odessa"), ("el", "Οντέσσα Ομπλάστ"), ("en", "Odeshchyna"), ("es", "Óblast de Odesa"), ("et", "Odessa oblast"), ("eu", "Odesako oblasta"), ("fa", "استان اودسا"), ("fi", "Odessan alue"), ("fr", "Oblast d’Odessa"), ("gl", "Óblast de Odesa"), ("gu", "ઓડ\u{ac7}સ\u{acd}સા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז אודסה"), ("hi", "ओड\u{947}सा ओब\u{94d}लास\u{94d}ट"), ("hr", "Odeška oblast"), ("hu", "Odesszai terület"), ("hy", "Օդեսայի մարզ"), ("id", "Oblast Odessa"), ("it", "Oblast’ di Odessa"), ("ja", "オデッサ州"), ("ka", "ოდესის ოლქი"), ("kn", "ಒಡ\u{cc6}ಸ\u{ccd}ಸಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "오데사 주"), ("lt", "Odesos sritis"), ("lv", "Odesas apgabals"), ("mk", "Одеска област"), ("mn", "Одесс муж"), ("mr", "ओद\u{947}सा ओब\u{94d}लास\u{94d}त"), ("ms", "Odessa Oblast"), ("nb", "Odessa oblast"), ("nl", "Oblast Odessa"), ("no", "Odessa oblast"), ("pl", "Obwód odeski"), ("pt", "Oblast de Odessa"), ("ro", "Regiunea Odesa"), ("ru", "Одесская область"), ("si", "ඔඩෙස\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Odeská oblasť"), ("sl", "Odeška pokrajina"), ("sr", "Одешка област"), ("sr_Latn", "Odeška oblast"), ("sv", "Odessa oblast"), ("ta", "ஒடிச\u{bbe} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ఒడ\u{c46}స\u{c4d}స\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "โอเดสซา"), ("tr", "Odessa Oblastı"), ("uk", "Одеська область"), ("ur", "اودیسا اوبلاست"), ("uz", "Odessa viloyati"), ("vi", "Odessa"), ("zh", "敖德萨州")]),
                        unofficial_name_list: ["Odesa", "Odeska", "Odessa"].to_vec(),
                    }
                ),
                (
                    "53",
                    Subdivision{
                        name: "53",
                        country_alpha2: Alpha2::UA,
                        code: "53",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.58826699999999), longitude: Some(34.551417), max_latitude: Some(50.5535039), min_latitude: Some(48.742984), max_longitude: Some(35.4904511), min_longitude: Some(32.08674999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Poltawa-oblast"), ("ar", "بولتافا أوبلاست"), ("az", "Poltava vilayəti"), ("be", "Палтаўская вобласць"), ("bg", "Полтавска област"), ("bn", "পোল\u{9cd}ট\u{9be}ভ\u{9be} ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Poltavska oblast"), ("ca", "Província de Poltava"), ("ccp", "𑄛\u{1112e}𑄣\u{11134}𑄑𑄛\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Poltavs’ka Oblast’"), ("cs", "Poltavská oblast"), ("da", "Poltava oblast"), ("de", "Oblast Poltawa"), ("el", "Πολτάβα Ομπλάστ"), ("en", "Poltavshchyna"), ("es", "Óblast de Poltava"), ("et", "Poltava oblast"), ("eu", "Poltavako oblasta"), ("fa", "استان پولتاوا"), ("fi", "Pultavan alue"), ("fr", "Oblast de Poltava"), ("gl", "Oblast de Poltava"), ("gu", "પોલ\u{acd}ટાવા ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "פולטבה"), ("hi", "पोल\u{94d}टावा ओब\u{94d}लास\u{94d}ट"), ("hr", "Poltavska oblast"), ("hu", "Poltavai terület"), ("hy", "Պոլտավայի մարզ"), ("id", "Oblast Poltava"), ("it", "Oblast’ di Poltava"), ("ja", "ポルタヴァ州"), ("ka", "პოლტავის ოლქი"), ("kk", "Полтава облысы"), ("kn", "ಪೊಲ\u{ccd}ಟಾವಾ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "폴타바 주"), ("lt", "Poltavos sritis"), ("lv", "Poltavas apgabals"), ("mk", "Полтавска област"), ("mn", "Полтав муж"), ("mr", "पोल\u{94d}ताव\u{94d}हा ओब\u{94d}लास\u{94d}त"), ("ms", "Poltava Oblast"), ("nb", "Poltava oblast"), ("nl", "Oblast Poltava"), ("no", "Poltava oblast"), ("pl", "Obwód połtawski"), ("pt", "oblast de Poltava"), ("ro", "Regiunea Poltava"), ("ru", "Полтавская область"), ("si", "පොල\u{dca}ටව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Poltavská oblasť"), ("sr", "Полтавска област"), ("sr_Latn", "Poltavska oblast"), ("sv", "Poltava oblast"), ("ta", "போல\u{bcd}ட\u{bcd}ட\u{bbe}வ\u{bbe} ஓப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ప\u{c4b}ల\u{c4d}ట\u{c3e}వ\u{c3e} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลโพล\u{e4c}ทาวา"), ("tr", "Poltava Oblastı"), ("uk", "Полтавська область"), ("ur", "پولتاوا اوبلاست"), ("uz", "Poltava viloyati"), ("vi", "Poltava"), ("zh", "波尔塔瓦州")]),
                        unofficial_name_list: ["Poltava", "Poltavska"].to_vec(),
                    }
                ),
                (
                    "56",
                    Subdivision{
                        name: "56",
                        country_alpha2: Alpha2::UA,
                        code: "56",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.6199), longitude: Some(26.251617), max_latitude: Some(51.9498591), min_latitude: Some(50.0043561), max_longitude: Some(27.729513), min_longitude: Some(25.0837821)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Riwne-oblast"), ("ar", "ريفنا أوبلاست"), ("az", "Rivne vilayəti"), ("be", "Ровенская вобласць"), ("bg", "Ровненска област"), ("bn", "রিভনে ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Rivanjska oblast"), ("ca", "Província de Rivne"), ("ccp", "𑄢\u{11128}𑄛\u{11134}𑄚𑄬𑄚\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Rivnens’ka Oblast’"), ("cs", "Rovenská oblast"), ("da", "Rivne oblast"), ("de", "Oblast Riwne"), ("el", "Ρίβνε Όμπλαστ"), ("en", "Rivnenshchyna"), ("es", "Óblast de Rivne"), ("et", "Rivne oblast"), ("eu", "Rivneko oblasta"), ("fa", "استان ریونه"), ("fi", "Rivnen alue"), ("fr", "Oblast de Rivne"), ("gu", "રિવ\u{acd}ન\u{ac7} ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "רובנו"), ("hi", "रिव\u{94d}न\u{947} ओब\u{94d}लास\u{94d}ट"), ("hr", "Rivnenska oblast"), ("hy", "Ռովնոյի մարզ"), ("id", "Oblast Rivne"), ("it", "Oblast’ di Rivne"), ("ja", "リウネ州"), ("ka", "როვნოს ოლქი"), ("kk", "Ровно облысы"), ("kn", "ರ\u{cbf}ವ\u{ccd}ನ\u{cc6} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "리우네 주"), ("lt", "Rivnės sritis"), ("lv", "Rivnes apgabals"), ("mk", "Ровненска област"), ("mn", "Ровно муж"), ("mr", "रिव\u{94d}ह\u{94d}न\u{947} ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Rivne"), ("nb", "Rivne oblast"), ("nl", "Oblast Rivne"), ("no", "Rivne oblast"), ("pl", "Obwód rówieński"), ("pt", "Oblast de Rivne"), ("ro", "Regiunea Rivne"), ("ru", "Ровненская область"), ("si", "ර\u{dd2}ව\u{dca}නේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Rivnenská oblasť"), ("sr", "Ривањска област"), ("sr_Latn", "Rivanjska oblast"), ("sv", "Rivne oblast"), ("ta", "ரிவனே ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ర\u{c3f}వ\u{c4d}న\u{c46} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e34}ฟเน"), ("tr", "Rivne Oblastı"), ("uk", "Рівненська область"), ("ur", "ریونہ اوبلاست"), ("uz", "Rovno viloyati"), ("vi", "Rivne"), ("zh", "羅夫諾州")]),
                        unofficial_name_list: ["Rivne", "Rivnenska", "Rovno", "Równe"].to_vec(),
                    }
                ),
                (
                    "59",
                    Subdivision{
                        name: "59",
                        country_alpha2: Alpha2::UA,
                        code: "59",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.9077), longitude: Some(34.7981), max_latitude: Some(52.367214), min_latitude: Some(50.109886), max_longitude: Some(35.6927141), min_longitude: Some(32.943534)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Soemi-oblast"), ("ar", "سومي أوبلاست"), ("az", "Sumı vilayəti"), ("be", "Сумская вобласць"), ("bg", "Сумска област"), ("bn", "স\u{9c1}ম\u{9cd}যি ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Sumska oblast"), ("ca", "Província de Sumy"), ("ccp", "𑄥𑄟\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Sums’ka Oblast’"), ("cs", "Sumská oblast"), ("da", "Sumi oblast"), ("de", "Oblast Sumy"), ("el", "Σούμσκαγια"), ("en", "Sumshchyna"), ("es", "Óblast de Sumy"), ("et", "Sumõ oblast"), ("eu", "Sumyko oblasta"), ("fa", "استان سومی"), ("fi", "Sumyn alue"), ("fr", "Oblast de Soumy"), ("ga", "Cúige Shumy"), ("gu", "સ\u{ac1}મી ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז סומי"), ("hi", "स\u{941}मी ओब\u{94d}लास\u{94d}ट"), ("hr", "Sumska oblast"), ("hu", "Szumi terület"), ("hy", "Սումիի մարզ"), ("id", "Oblast Sumy"), ("it", "Oblast’ di Sumy"), ("ja", "スームィ州"), ("ka", "სუმის ოლქი"), ("kn", "ಸುಮ\u{cbf} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "수미 주"), ("lt", "Sumų sritis"), ("lv", "Sumu apgabals"), ("mk", "Сумска област"), ("mn", "Сумы муж"), ("mr", "स\u{941}मी ओब\u{94d}लास\u{94d}त"), ("ms", "Sumy Oblast"), ("nb", "Sumy oblast"), ("nl", "Oblast Soemy"), ("no", "Sumy oblast"), ("pl", "Obwód sumski"), ("pt", "Oblast de Sumi"), ("ro", "Regiunea Sumî"), ("ru", "Сумская область"), ("si", "ස\u{dcf}ම\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sumská oblasť"), ("sl", "Sumi Oblast"), ("sr", "Сумска област"), ("sr_Latn", "Sumska oblast"), ("sv", "Sumy oblast"), ("ta", "சுமி ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "సుమ\u{c40} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "เขตปกครองซ\u{e39}ม\u{e35}"), ("tr", "Sumi Oblastı"), ("uk", "Сумська область"), ("ur", "سومی اوبلاست"), ("uz", "Sumi viloyati"), ("vi", "Sumi"), ("zh", "蘇梅州")]),
                        unofficial_name_list: ["Sumska", "Sumy"].to_vec(),
                    }
                ),
                (
                    "61",
                    Subdivision{
                        name: "61",
                        country_alpha2: Alpha2::UA,
                        code: "61",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.553517), longitude: Some(25.594767), max_latitude: Some(50.267215), min_latitude: Some(48.507534), max_longitude: Some(26.4434051), min_longitude: Some(24.718486)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ternopil-oblast"), ("ar", "تيرنوبل أوبلاست"), ("az", "Ternopil vilayəti"), ("be", "Цярнопальская вобласць"), ("bg", "Тернополска област"), ("bn", "টের\u{9cd}নোপিল ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Ternopiljska oblast"), ("ca", "Província de Ternópill"), ("ccp", "𑄑𑄬𑄢\u{11134}𑄚\u{1112e}𑄛\u{11128}𑄣\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Ternopil’s’ka Oblast’"), ("cs", "Ternopilská oblast"), ("da", "Ternopil oblast"), ("de", "Oblast Ternopil"), ("el", "Τερνοπίλ Ομπλάστ"), ("en", "Ternopilshchyna"), ("es", "Óblast de Ternópil"), ("et", "Ternopili oblast"), ("eu", "Ternopilgo oblasta"), ("fa", "استان ترنوپیل"), ("fi", "Ternopilin alue"), ("fr", "Oblast de Ternopil"), ("gl", "Rexión de Ternopil"), ("gu", "ટ\u{ac7}ર\u{acd}નોપિલ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז טרנופול"), ("hi", "ट\u{947}मोपिल ओब\u{94d}लास\u{94d}ट"), ("hr", "Ternopiljska oblast"), ("hu", "Ternopili terület"), ("hy", "Տերնոպոլի մարզ"), ("id", "Oblast Ternopil"), ("it", "Oblast’ di Ternopil’"), ("ja", "テルノーピリ州"), ("ka", "ტერნოპოლის ოლქი"), ("kn", "ತರ\u{ccd}ನೋಫ\u{cbf}ಲ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "테르노필 주"), ("lt", "Ternopilio sritis"), ("lv", "Ternopiļas apgabals"), ("mk", "Тернополска област"), ("mn", "Тернополь муж"), ("mr", "त\u{947}र\u{94d}नोपिल ओब\u{94d}लास\u{94d}त"), ("ms", "Ternopil Oblast"), ("nb", "Ternopil oblast"), ("nl", "Oblast Ternopil"), ("no", "Ternopil oblast"), ("pl", "Obwód tarnopolski"), ("pt", "Oblast de Ternopil"), ("ro", "Regiunea Ternopil"), ("ru", "Тернопольская область"), ("si", "ටර\u{dca}නොප\u{dd2}ල\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ternopiľská oblasť"), ("sr", "Тернопољска област"), ("sr_Latn", "Ternopoljska oblast"), ("sv", "Ternopil oblast"), ("ta", "டெர\u{bcd}னோபிள\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ట\u{c46}ర\u{c4d}న\u{c4b}ప\u{c4b}ల\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทอโนพ\u{e34}ล โอเบลส"), ("tr", "Ternopil Oblastı"), ("uk", "Тернопільська область"), ("ur", "تیرنوپیل اوبلاست"), ("uz", "Ternopol viloyati"), ("vi", "Ternopil"), ("zh", "捷爾諾波爾州")]),
                        unofficial_name_list: ["Ternopil", "Ternopilska", "Ternopol"].to_vec(),
                    }
                ),
                (
                    "63",
                    Subdivision{
                        name: "63",
                        country_alpha2: Alpha2::UA,
                        code: "63",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.9935), longitude: Some(36.230383), max_latitude: Some(50.459388), min_latitude: Some(48.531375), max_longitude: Some(38.09530609999999), min_longitude: Some(34.8563339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Charkif-oblast"), ("ar", "خاركيف أوبلاست"), ("az", "Xarkov vilayəti"), ("be", "Харкаўская вобласць"), ("bg", "Харковска област"), ("bn", "ক\u{9be}রকিভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Harkivska oblast"), ("ca", "Província de Khàrkiv"), ("ccp", "𑄈𑄢\u{11134}𑄇\u{11128}𑄛\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Kharkivs’ka Oblast’"), ("cs", "Charkovská oblast"), ("da", "Kharkiv oblast"), ("de", "Oblast Charkiw"), ("el", "Καρκίβ Ομπλάστ"), ("en", "Kharkivshchyna"), ("es", "Óblast de Járkov"), ("et", "Harkivi oblast"), ("eu", "Kharkiveko oblasta"), ("fa", "استان خارکف"), ("fi", "Harkovan alue"), ("fr", "Oblast de Kharkiv"), ("gu", "ખાર\u{acd}કીવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז חרקוב"), ("hi", "खार\u{94d}किव ओब\u{94d}लास\u{94d}ट"), ("hr", "Harkivska oblast"), ("hu", "Harkivi terület"), ("hy", "Խարկովի մարզ"), ("id", "Oblast Kharkiv"), ("it", "Oblast’ di Charkiv"), ("ja", "ハルキウ州"), ("ka", "ხარკოვის ოლქი"), ("kn", "ಖಾರ\u{ccd}ಕ\u{cbf}ವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "하르키우 주"), ("lt", "Charkovo sritis"), ("lv", "Harkovas apgabals"), ("mk", "Харковска област"), ("mn", "Харьков муж"), ("mr", "खार\u{94d}कीव\u{94d}ह ओब\u{94d}लास\u{94d}त"), ("ms", "Kharkiv Oblast"), ("nb", "Kharkiv oblast"), ("nl", "Oblast Charkov"), ("no", "Kharkiv oblast"), ("pl", "Obwód charkowski"), ("pt", "Oblast de Kharkiv"), ("ro", "Regiunea Harkov"), ("ru", "Харьковская область"), ("si", "කර\u{dca}ක\u{dd2}ව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Charkovská oblasť"), ("sr", "Харковска област"), ("sr_Latn", "Harkovska oblast"), ("sv", "Charkiv oblast"), ("ta", "க\u{bbe}ர\u{bcd}க\u{bcd}கிவ\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "హ\u{c3e}ర\u{c4d}క\u{c3f}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "คาร\u{e4c}ค\u{e34}ฟ"), ("tr", "Harkiv Oblastı"), ("uk", "Харківська область"), ("ur", "خارکیو اوبلاست"), ("uz", "Xarkiv viloyati"), ("vi", "Kharkiv"), ("zh", "哈爾科夫州")]),
                        unofficial_name_list: ["Harkov", "Kharkiv", "Kharkivska", "Kharkov"].to_vec(),
                    }
                ),
                (
                    "65",
                    Subdivision{
                        name: "65",
                        country_alpha2: Alpha2::UA,
                        code: "65",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.635417), longitude: Some(32.616867), max_latitude: Some(47.600048), min_latitude: Some(45.7607072), max_longitude: Some(35.281549), min_longitude: Some(31.5124959)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cherson-oblast"), ("ar", "خيرسون أوبلاست"), ("az", "Xerson vilayəti"), ("be", "Херсонская вобласць"), ("bg", "Херсонска област"), ("bn", "খেরসন ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Hersonska oblast"), ("ca", "Província de Kherson"), ("ccp", "𑄈𑄬𑄢\u{11134}𑄥\u{1112e}𑄚\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Khersons’ka Oblast’"), ("cs", "Chersonská oblast"), ("da", "Kherson oblast"), ("de", "Oblast Cherson"), ("el", "Κχέρσον Ομπλάστ"), ("en", "Khersonshchyna"), ("es", "Óblast de Jersón"), ("et", "Hersoni oblast"), ("eu", "Khersongo oblasta"), ("fa", "استان خرسون"), ("fi", "Hersonin alue"), ("fr", "Oblast de Kherson"), ("ga", "Cúige Kherson"), ("gu", "ખ\u{ac7}ર\u{acd}સન ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז חרסון"), ("hi", "ख\u{947}र\u{94d}सौन ओब\u{94d}लास\u{94d}ट"), ("hr", "Hersonska oblast"), ("hu", "Herszoni terület"), ("hy", "Խերսոնի մարզ"), ("id", "Oblast Kherson"), ("it", "Oblast’ di Cherson"), ("ja", "ヘルソン州"), ("ka", "ხერსონის ოლქი"), ("kn", "ಖ\u{cc6}ರ\u{ccd}ಸೋನ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "헤르손 주"), ("lt", "Chersono sritis"), ("lv", "Hersonas apgabals"), ("mk", "Херсонска област"), ("mn", "Херсон муж"), ("mr", "ख\u{947}र\u{94d}सन ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Kherson"), ("nb", "Kherson oblast"), ("nl", "Oblast Cherson"), ("no", "Kherson oblast"), ("pl", "Obwód chersoński"), ("pt", "Oblast de Kherson"), ("ro", "Regiunea Herson"), ("ru", "Херсонская область"), ("si", "කෙර\u{dca}සොන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Chersonská oblasť"), ("sl", "Hersonska oblast"), ("sr", "Херсонска област"), ("sr_Latn", "Hersonska oblast"), ("sv", "Cherson oblast"), ("ta", "க\u{bcd}ஹெர\u{bcd}சோன\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ఖ\u{c46}ర\u{c4d}సన\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลเคอร\u{e4c}ซ\u{e31}น"), ("tr", "Herson Oblastı"), ("uk", "Херсонська область"), ("ur", "خیرسون اوبلاست"), ("uz", "Xerson viloyati"), ("vi", "Kherson"), ("zh", "赫尔松州")]),
                        unofficial_name_list: ["Herson", "Kherson", "Khersonska"].to_vec(),
                    }
                ),
                (
                    "68",
                    Subdivision{
                        name: "68",
                        country_alpha2: Alpha2::UA,
                        code: "68",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.422983), longitude: Some(26.987133), max_latitude: Some(50.594764), min_latitude: Some(48.4516599), max_longitude: Some(27.8984479), min_longitude: Some(26.1329989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chmelnitski-oblast"), ("ar", "خملنيتسكي أوبلاست"), ("az", "Xmelnitski vilayəti"), ("be", "Хмяльніцкая вобласць"), ("bg", "Хмелницка област"), ("bn", "খেমেলনিতস\u{9cd}কি ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Hmeljnička oblast"), ("ca", "Província de Khmelnitski"), ("ccp", "𑄈\u{11127}𑄟𑄬𑄣\u{11134}𑄥\u{11128}𑄠𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Khmel’nyts’ka Oblast’"), ("cs", "Chmelnycká oblast"), ("da", "Khmelnitskij oblast"), ("de", "Oblast Chmelnyzkyj"), ("el", "Κχμελνίτσκι Ομπλάστ"), ("en", "Khmelnychchyna"), ("es", "Óblast de Jmelnitski"), ("et", "Hmelnõtskõi oblast"), ("eu", "Khmelnytskyko oblasta"), ("fa", "استان خملنیتسکی"), ("fi", "Hmelnitskyin alue"), ("fr", "oblast de Khmelnitski"), ("gu", "ખ\u{acd}મ\u{ac7}લિન\u{acd}ત\u{acd}સ\u{acd}ક ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז חמלניצקי"), ("hi", "ख\u{94d}म\u{947}लनिट\u{94d}सकी ओब\u{94d}लास\u{94d}ट"), ("hr", "Hmeljnička oblast"), ("hu", "Hmelnickiji terület"), ("hy", "Խմելնիցկիի մարզ"), ("id", "Oblast Khmelnytskyi"), ("it", "Oblast’ di Chmel’nyc’kyj"), ("ja", "フメリヌィーツィクィイ州"), ("ka", "ხმელნიცკის ოლქი"), ("kk", "Хмельницкий облысы"), ("kn", "ಖ\u{ccd}ಮ\u{cc6}ಲ\u{ccd}ನ\u{cbf}ಟ\u{ccd}ಸ\u{ccd}ಕ\u{cbf} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "흐멜니츠키 주"), ("lt", "Chmelnyckio sritis"), ("lv", "Hmeļnickas apgabals"), ("mk", "Хмелницка област"), ("mn", "Хмельницкий муж"), ("mr", "ख\u{94d}म\u{947}ल\u{94d}नित\u{94d}स\u{94d}की ओब\u{94d}लास\u{94d}त"), ("ms", "Khmelnytsky Oblast"), ("nb", "Khmelnytskyj oblast"), ("nl", "Oblast Chmelnytsky"), ("no", "Khmelnytskyj oblast"), ("pl", "Obwód chmielnicki"), ("pt", "Oblast de Khmelnitski"), ("ro", "Regiunea Hmelnîțkîi"), ("ru", "Хмельницкая область"), ("si", "ක\u{dca}මෙල\u{dca}න\u{dd2}ට\u{dca}ස\u{dca}ක\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Chmeľnycká oblasť"), ("sr", "Хмељничка област"), ("sr_Latn", "Hmeljnička oblast"), ("sv", "Chmelnytskyj oblast"), ("ta", "க\u{bcd}ஹமெளனிட\u{bcd}ஸ\u{bcd}க\u{bcd} ஓப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "ఖ\u{c4d}మ\u{c46}ల\u{c4d}న\u{c3f}ట\u{c4d}స\u{c4d}క\u{c40} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลฮเมลน\u{e34}ซก\u{e35}"), ("tr", "Hmelnitski Oblastı"), ("uk", "Хмельницька область"), ("ur", "خمیلنیتسکی اوبلاست"), ("uz", "Xmelnitskiy viloyati"), ("vi", "Khmelnytskyi"), ("zh", "赫梅利尼茨基州")]),
                        unofficial_name_list: ["Hmelnickij", "Khmelnitsky", "Khmelnytska", "Khmelnytskyy"].to_vec(),
                    }
                ),
                (
                    "71",
                    Subdivision{
                        name: "71",
                        country_alpha2: Alpha2::UA,
                        code: "71",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.444433), longitude: Some(32.059767), max_latitude: Some(50.228656), min_latitude: Some(48.4519151), max_longitude: Some(32.8737981), min_longitude: Some(29.6063949)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjerkasi-oblast"), ("ar", "محافظة تشيركاسي"), ("az", "Çerkası vilayəti"), ("be", "Чаркаская вобласць"), ("bg", "Черкаска област"), ("bn", "চেরক\u{9be}সি ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Čerkaška oblast"), ("ca", "Província de Txerkassy"), ("ccp", "𑄌𑄬𑄢\u{11134}𑄇𑄌\u{11134}𑄌\u{11128}𑄚"), ("ceb", "Cherkas’ka Oblast’"), ("cs", "Čerkaská oblast"), ("da", "Tjerkasi oblast"), ("de", "Oblast Tscherkassy"), ("el", "Τσερκάσι Ομπλάστ"), ("en", "Cherkashchyna"), ("es", "Óblast de Cherkasy"), ("et", "Tšerkassõ oblast"), ("eu", "Txerkasyko oblasta"), ("fa", "استان چرکاسی"), ("fi", "Tšerkasyn alue"), ("fr", "Oblast de Tcherkassy"), ("ga", "Cúige Cherkasy"), ("gl", "Óblast de Cherkasy"), ("gu", "ચ\u{ac7}ર\u{acd}કાસી ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז צ׳רקאסי"), ("hi", "च\u{947}रकासी ओब\u{94d}लास\u{94d}ट"), ("hr", "Čerkaška oblast"), ("hu", "Cserkaszi terület"), ("hy", "Չերկասիի մարզ"), ("id", "Oblast Cherkasy"), ("it", "Oblast’ di Čerkasy"), ("ja", "チェルカースィ州"), ("ka", "ჩერკასის ოლქი"), ("kk", "Черкассы облысы"), ("kn", "ಚ\u{cc6}ರ\u{ccd}ಕಾಸೀ ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "체르카시 주"), ("lt", "Čerkasų sritis"), ("lv", "Čerkasu apgabals"), ("mk", "Черкаска област"), ("mn", "Черкас муж"), ("mr", "च\u{947}र\u{94d}कासी ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Cherkasy"), ("nb", "Tsjerkasy oblast"), ("nl", "Oblast Tsjerkasy"), ("no", "Tsjerkasy oblast"), ("pl", "Obwód czerkaski"), ("pt", "Oblast de Tcherkássi"), ("ro", "Regiunea Cerkasî"), ("ru", "Черкасская область"), ("si", "චර\u{dca}කස\u{dd2} පළ\u{dcf}ත"), ("sk", "Čerkaská oblasť"), ("sl", "Čerkaška oblast"), ("sr", "Черкашка област"), ("sr_Latn", "Čerkaška oblast"), ("sv", "Tjerkasy oblast"), ("ta", "சேர\u{bcd}க\u{bcd}கஸி ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "చ\u{c46}ర\u{c4d}క\u{c47}స\u{c40} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "แคว\u{e49}นปกครองเช\u{e35}ยร\u{e4c}คาซ\u{e35}"), ("tr", "Çerkası Oblastı"), ("uk", "Черкаська область"), ("ur", "چیرکاسی اوبلاست"), ("uz", "Cherkasi viloyati"), ("vi", "Cherkasy"), ("zh", "切爾卡瑟州")]),
                        unofficial_name_list: ["Cherkask", "Cherkassy"].to_vec(),
                    }
                ),
                (
                    "74",
                    Subdivision{
                        name: "74",
                        country_alpha2: Alpha2::UA,
                        code: "74",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4982), longitude: Some(31.28935), max_latitude: Some(52.379379), min_latitude: Some(50.345535), max_longitude: Some(33.5009611), min_longitude: Some(30.49716799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjernihif-oblast"), ("ar", "تشرنيهيف أوبلاست"), ("az", "Çerniqov vilayəti"), ("be", "Чарнігаўская вобласць"), ("bg", "Черниговска област"), ("bn", "চের\u{9cd}নিহিভ ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Černigivska oblast"), ("ca", "Província de Txerníhiv"), ("ccp", "𑄌𑄬𑄢\u{11134}𑄚\u{11128}𑄦\u{11128}𑄛\u{11134}𑄌\u{11133}𑄦\u{11128}𑄚"), ("ceb", "Chernihivs’ka Oblast’"), ("cs", "Černihivská oblast"), ("da", "Tjernihiv oblast"), ("de", "Oblast Tschernihiw"), ("el", "Τσερνιχίβ Ομπλάστ"), ("en", "Chernihivshchyna"), ("es", "Óblast de Chernihiv"), ("et", "Tšernigivi oblast"), ("eu", "Txernihiveko oblasta"), ("fa", "استان چرنیهیو"), ("fi", "Tšernihivin alue"), ("fr", "Oblast de Tchernihiv"), ("gu", "ચ\u{ac7}ર\u{acd}નીહીવ ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז צ׳רניהיב"), ("hi", "च\u{947}रनिहाइव ओब\u{94d}लास\u{94d}ट"), ("hr", "Černigivska oblast"), ("hu", "Csernyihivi terület"), ("hy", "Չեռնիգովի մարզ"), ("id", "Oblast Chernihiv"), ("it", "Oblast’ di Černihiv"), ("ja", "チェルニーヒウ州"), ("ka", "ჩერნიგოვის ოლქი"), ("kn", "ಚ\u{cc6}ರ\u{ccd}ನ\u{cbf}ಹ\u{cbf}ವ\u{ccd} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "체르니히우 주"), ("lt", "Černigovo sritis"), ("lv", "Čerņigovas apgabals"), ("mk", "Черниговска област"), ("mn", "Чернигов муж"), ("mr", "च\u{947}र\u{94d}निहिव\u{94d}ह ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Chernihiv"), ("nb", "Tsjernihiv oblast"), ("nl", "Oblast Tsjernihiv"), ("no", "Tsjernihiv oblast"), ("pl", "Obwód czernihowski"), ("pt", "Oblast de Chernigov"), ("ro", "Regiunea Cernihiv"), ("ru", "Черниговская область"), ("si", "චෙන\u{dd2}හ\u{dd2}ව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Černihivská oblasť"), ("sr", "Чернигивска област"), ("sr_Latn", "Černigivska oblast"), ("sv", "Tjernihiv oblast"), ("ta", "செர\u{bcd}னிஹிவ\u{bcd} ஒப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "చ\u{c46}ర\u{c4d}న\u{c3f}వ\u{c4d} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเช\u{e35}ยร\u{e4c}น\u{e35}ฮ\u{e34}ฟ"), ("tr", "Çernigov Oblastı"), ("uk", "Чернігівська область"), ("ur", "چیرنیہیو اوبلاست"), ("uz", "Chernigiv viloyati"), ("vi", "Chernihiv"), ("zh", "切爾尼戈夫州")]),
                        unofficial_name_list: ["Chernigov", "Chernihiv", "Černigov"].to_vec(),
                    }
                ),
                (
                    "77",
                    Subdivision{
                        name: "77",
                        country_alpha2: Alpha2::UA,
                        code: "77",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.291683), longitude: Some(25.935217), max_latitude: Some(48.6759141), min_latitude: Some(47.72516599999999), max_longitude: Some(27.533325), min_longitude: Some(24.908172)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tsjerniftsi-oblast"), ("ar", "تشيرنيفتسي أوبلاست"), ("az", "Çernivtsı vilayəti"), ("be", "Чарнавіцкая вобласць"), ("bg", "Черновицка област"), ("bn", "চের\u{9cd}নিভিৎসি ওব\u{9cd}ল\u{9be}স\u{9cd}ট"), ("bs", "Černivačka oblast"), ("ca", "Província de Txernivtsí"), ("ccp", "𑄌𑄬𑄢\u{11134}𑄚\u{11128}𑄛\u{11134}𑄑\u{11134}𑄥\u{11128} 𑄃\u{11127}𑄛\u{11134}𑄣𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Chernivets’ka Oblast’"), ("cs", "Černovická oblast"), ("da", "Tjernivtsi oblast"), ("de", "Oblast Tscherniwzi"), ("el", "Τσερνίβτσι Ομπλάστ"), ("en", "Chernivtsi Oblast"), ("es", "Óblast de Chernivtsi"), ("et", "Tšernivtsi oblast"), ("eu", "Txernivtsiko oblasta"), ("fa", "استان چرنیوتسی"), ("fi", "Tšernivtsin alue"), ("fr", "Oblast de Tchernivtsi"), ("gu", "ચ\u{ac7}ર\u{acd}નિવત\u{acd}સકી ઓબ\u{acd}લાસ\u{acd}ટ"), ("he", "מחוז צ׳רנוביץ"), ("hi", "च\u{947}मिव\u{94d}त\u{94d}सी ओब\u{94d}लास\u{94d}ट"), ("hr", "Černivecka oblast"), ("hu", "Csernyivci terület"), ("hy", "Չեռնովցիի մարզ"), ("id", "Oblast Chernivtsi"), ("it", "Oblast’ di Černivci"), ("ja", "チェルニウツィー州"), ("ka", "ჩერნოვცის ოლქი"), ("kk", "Черновцы облысы"), ("kn", "ಚ\u{cc6}ರ\u{ccd}ನ\u{cbf}ವಟ\u{ccd}ಸ\u{cbf} ಒಬ\u{ccd}ಲಾಸ\u{ccd}ಟ\u{ccd}"), ("ko", "체르니우치 주"), ("lt", "Černivcių sritis"), ("lv", "Černivcu apgabals"), ("mk", "Черновечка област"), ("mn", "Черновцы муж"), ("mr", "च\u{947}र\u{94d}निव\u{94d}हत\u{94d}सी ओब\u{94d}लास\u{94d}त"), ("ms", "Oblast Chernivtsi"), ("nb", "Tsjernivtsi oblast"), ("nl", "Oblast Tsjernivtsi"), ("no", "Tsjernivtsi oblast"), ("pl", "Obwód czerniowiecki"), ("pt", "Oblast de Chernivtsi"), ("ro", "Regiunea Cernăuți"), ("ru", "Черновицкая область"), ("si", "චෙන\u{dd2}ව\u{dca}ට\u{dca}ස\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Černovická oblasť"), ("sr", "Чернивачка област"), ("sr_Latn", "Černivačka oblast"), ("sv", "Tjernivtsi oblast"), ("ta", "செர\u{bcd}னிவடசி ஓப\u{bcd}ள\u{bbe}ஸ\u{bcd}ட\u{bcd}"), ("te", "చ\u{c46}ర\u{c4d}న\u{c3f}వట\u{c4d}స\u{c40} ఓబ\u{c4d}ల\u{c3e}స\u{c4d}ట\u{c4d}"), ("th", "มณฑลเชอร\u{e4c}เนฟฟ\u{e35}"), ("tr", "Çernovtsi Oblastı"), ("uk", "Чернівецька область"), ("ur", "چیرنیوتسی اوبلاست"), ("uz", "Chernivsi viloyati"), ("vi", "Chernivtsi"), ("zh", "切爾諾夫策州")]),
                        unofficial_name_list: ["Cernăuţi", "Chernivtsi", "Chernivtsy", "Chernovtsy", "Czernowitz", "Tschernowitz", "Černovcy"].to_vec(),
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
#[cfg(feature = "ua")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::UA,
        alpha3: Alpha3::UKR,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}}\n{{postalcode}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 380,
        currency_code: "UAH",
        gec: Some(GEC::UP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::UKR),
        iso_long_name: "Ukraine",
        iso_short_name: "Ukraine",
        official_language_list: ["uk"].to_vec(),
        spoken_language_list: ["uk"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "8",
        nationality: Some("Ukrainian"),
        number: "804",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "UA",
        unofficial_name_list: [
            "Ukraine",
            "Ucrania",
            "ウクライナ",
            "Oekraïne",
            "Украина",
            "Україна",
            "Украіна",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Ukraine"),
            ("af", "Oekraïne"),
            ("ak", "Ukraine"),
            ("am", "ዩክሬን"),
            ("an", "Ukraine"),
            ("ar", "أوكرانيا"),
            ("as", "ইউক\u{9cd}ৰেইন"),
            ("ay", "Ukraine"),
            ("az", "Ukrayna"),
            ("ba", "Ukraine"),
            ("be", "Украіна"),
            ("bg", "Украйна"),
            ("bi", "Ukraine"),
            ("bn", "ইউক\u{9cd}রেন"),
            ("bn_IN", "ইউক\u{9cd}রেন"),
            ("br", "Ukraina"),
            ("bs", "Ukrajina"),
            ("ca", "Ucraïna"),
            ("ce", "Украина"),
            ("ch", "Ukraine"),
            ("cs", "Ukrajina"),
            ("cv", "Украина"),
            ("cy", "Wcrain"),
            ("da", "Ukraine"),
            ("de", "Ukraine"),
            ("dv", "ޔ\u{7aa}ކ\u{7b0}ރ\u{7ac}އ\u{7a8}ނ\u{7b0}"),
            ("dz", "ཡ\u{f74}་ཀ\u{f7a}རན།"),
            ("ee", "Ukraine"),
            ("el", "Ουκρανία"),
            ("en", "Ukraine"),
            ("eo", "Ukrainio"),
            ("es", "Ucrania"),
            ("et", "Ukraina"),
            ("eu", "Ukrania"),
            ("fa", "اکراین"),
            ("ff", "Ukrayiina"),
            ("fi", "Ukraina"),
            ("fo", "Ukreina"),
            ("fr", "Ukraine"),
            ("fy", "Oekraïne"),
            ("ga", "An Úcráin"),
            ("gl", "Ucraína"),
            ("gn", "Ukraine"),
            ("gu", "ય\u{ac1}ક\u{acd}ર\u{ac7}ન"),
            ("gv", "Yn Ookraan"),
            ("ha", "Ukraniya"),
            ("he", "אוקראינה"),
            ("hi", "य\u{941}क\u{94d}र\u{947}न"),
            ("hr", "Ukrajina"),
            ("ht", "Ikrèn"),
            ("hu", "Ukrajna"),
            ("hy", "Ուկրաինա"),
            ("ia", "Ukraina"),
            ("id", "Ukraina"),
            ("io", "Ukrainia"),
            ("is", "Úkraína"),
            ("it", "Ucraina"),
            ("iu", "ᑯᑯᓯ ᓄᓇ"),
            ("ja", "ウクライナ"),
            ("ka", "უკრაინა"),
            ("ki", "Ukraine"),
            ("kk", "Украина"),
            ("kl", "Ukraine"),
            ("km", "អ\u{17ca}\u{17bb}យក\u{17d2}រែន"),
            ("kn", "ಯುಕ\u{ccd}ರೇನ\u{ccd}"),
            ("ko", "우크라이나"),
            ("ku", "Ukrayna"),
            ("kv", "Украина"),
            ("kw", "Ukrayn"),
            ("ky", "Украина"),
            ("lo", "ປະເທດອ\u{eb9}ກະແລນ"),
            ("lt", "Ukraina"),
            ("lv", "Ukraina"),
            ("mi", "Ūkareinga"),
            ("mk", "Украина"),
            ("ml", "ഉക\u{d4d}രൈന\u{d4d}\u{200d}"),
            ("mn", "Украйн"),
            ("mr", "य\u{941}क\u{94d}र\u{947}न"),
            ("ms", "Ukraine"),
            ("mt", "Ukranja"),
            (
                "my",
                "ယ\u{1030}ကရ\u{102d}န\u{103a}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ukraine"),
            ("nb", "Ukraina"),
            ("ne", "य\u{941}क\u{94d}र\u{947}न"),
            ("nl", "Oekraïne"),
            ("nn", "Ukraina"),
            ("nv", "Yóókwein"),
            ("oc", "Ucraïna"),
            ("or", "ୟ\u{b41}କ\u{b4d}ରେନ"),
            ("pa", "ਯ\u{a42}ਕਰ\u{a47}ਨ"),
            ("pi", "Ukraine"),
            ("pl", "Ukraina"),
            ("ps", "اوکراین"),
            ("pt", "Ucrânia"),
            ("pt_BR", "Ucrânia"),
            ("ro", "Ucraina"),
            ("ru", "Украина"),
            ("rw", "Ikerene"),
            ("sc", "Ucraina"),
            ("sd", "يوڪرين"),
            ("si", "ය\u{dd4}ක\u{dca}රේනය"),
            ("sk", "Ukrajina"),
            ("sl", "Ukrajina"),
            ("so", "Ukraine"),
            ("sq", "Ukrainë"),
            ("sr", "Украјина"),
            ("sv", "Ukraina"),
            ("sw", "Ukraine"),
            ("ta", "உக\u{bcd}ரெயின\u{bcd}"),
            ("te", "యుక\u{c4d}ర\u{c48}న\u{c4d}"),
            ("tg", "Украина"),
            ("th", "ย\u{e39}เครน"),
            ("ti", "ዩክረይን"),
            ("tk", "Ukraina"),
            ("tl", "Ukraine"),
            ("tr", "Ukrayna"),
            ("tt", "Украин"),
            ("ug", "ئۇكرائىنا"),
            ("uk", "Україна"),
            ("ur", "یوکرین"),
            ("uz", "Ukraina"),
            ("ve", "Ukraine"),
            ("vi", "U-cờ-rai-na"),
            ("wa", "Oucrinne"),
            ("wo", "Ukreen"),
            ("xh", "Ukraine"),
            ("yo", "Ukréìn"),
            ("zh_CN", "乌克兰"),
            ("zh_HK", "烏克蘭"),
            ("zh_TW", "烏克蘭"),
            ("zu", "I-Yukreyini"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
