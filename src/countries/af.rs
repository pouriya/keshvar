// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Islamic Republic of Afghanistan

#[cfg(all(feature = "af", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AF;
    pub const ALPHA3: Alpha3 = Alpha3::AFG;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 93;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::AFN;
    pub const GEC: Option<GEC> = Some(GEC::AF);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::AFG);
    pub const ISO_SHORT_NAME: &str = "Afghanistan";
    pub const ISO_LONG_NAME: &str = "The Islamic Republic of Afghanistan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ps", "tk", "uz"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ps", "tk", "uz"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Afghan");
    pub const NUMBER: &str = "004";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "AF";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Afghanistan", "Afganistán", "アフガニスタン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Afghanistan"),
        ("af", "Afghanistan"),
        ("ak", "Afghanistan"),
        ("am", "አፍጋኒስታን"),
        ("an", "Afganistán"),
        ("ar", "أفغانستان"),
        ("as", "আফগ\u{9be}নিস\u{9cd}থ\u{9be}ন"),
        ("ay", "Afghanistan"),
        ("az", "Əfqanıstan"),
        ("ba", "Afghanistan"),
        ("be", "Афганістан"),
        ("bg", "Афганистан"),
        ("bi", "Afghanistan"),
        ("bn", "আফগ\u{9be}নিস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "আফগ\u{9be}নিস\u{9cd}ত\u{9be}ন"),
        ("br", "Afghanistan"),
        ("bs", "Afganistan"),
        ("ca", "Afganistan"),
        ("ce", "АфгIанистан"),
        ("ch", "Afghanistan"),
        ("cs", "Afghánistán"),
        ("cv", "Афганистан"),
        ("cy", "Afghanistan"),
        ("da", "Afghanistan"),
        ("de", "Afghanistan"),
        (
            "dv",
            "އ\u{7a6}ފ\u{7b0}ޣ\u{7a7}ނ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}",
        ),
        ("dz", "ཨཕ་ག་ན\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Afghanistan"),
        ("el", "Αφγανιστάν"),
        ("en", "Afghanistan"),
        ("eo", "Afganio"),
        ("es", "Afganistán"),
        ("et", "Afganistan"),
        ("eu", "Afganistan"),
        ("fa", "افغانستان"),
        ("ff", "Afghanistan"),
        ("fi", "Afghanistan"),
        ("fo", "Afganistan"),
        ("fr", "Afghanistan"),
        ("fy", "Afganistan"),
        ("ga", "An Afganastáin"),
        ("gl", "Afganistán"),
        ("gn", "Afghanistan"),
        ("gu", "અફઘાનિસ\u{acd}તાન"),
        ("gv", "Yn Afghanistaan"),
        ("ha", "Afghanistan"),
        ("he", "אפגניסטן"),
        ("hi", "अफ\u{93c}\u{94d}गानिस\u{94d}तान"),
        ("hr", "Afganistan"),
        ("ht", "Afganistan"),
        ("hu", "Afganisztán"),
        ("hy", "Աֆղանստան"),
        ("ia", "Afghanistan"),
        ("id", "Afganistan"),
        ("io", "Afganistan"),
        ("is", "Afganistan"),
        ("it", "Afghanistan"),
        ("iu", "Afghanistan"),
        ("ja", "アフガニスタン"),
        ("ka", "ავღანეთი"),
        ("ki", "Afghanistan"),
        ("kk", "Ауғанстан"),
        ("kl", "Afghanistan"),
        ("km", "អាហ\u{17d2}គាន\u{17b8}ស\u{17d2}ថាន"),
        ("kn", "ಅಫ\u{ccd}ಗಾನ\u{cbf}ಸ\u{ccd}ಥಾನ\u{ccd}"),
        ("ko", "아프가니스탄"),
        ("ku", "Efxanistan"),
        ("kv", "Афганистан"),
        ("kw", "Afghanistan"),
        ("ky", "Ооганстан"),
        ("lo", "Afghanistan"),
        ("lt", "Afganistanas"),
        ("lv", "Afganistāna"),
        ("mi", "Āwhekenetāna"),
        ("mk", "Авганистан"),
        ("ml", "അഫ\u{d4d}ഗ\u{d3e}നിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
        ("mn", "Афганстан"),
        ("mr", "अफगाणिस\u{94d}तान"),
        ("ms", "Afghanistan"),
        ("mt", "Afganistan"),
        (
            "my",
            "အာဖဂန\u{103a}နစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Apeganitan"),
        ("nb", "Afghanistan"),
        ("ne", "अफगानिस\u{94d}तान"),
        ("nl", "Afghanistan"),
        ("nn", "Afghanistan"),
        ("nv", "Afghanistan"),
        ("oc", "Afganistan"),
        ("or", "ଆଫଗ\u{b3e}ନ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਅਫਗਾਨਿਸਤਾਨ"),
        ("pi", "अफगानस\u{94d}थान"),
        ("pl", "Afganistan"),
        ("ps", "افغانستان"),
        ("pt", "Afeganistão"),
        ("pt_BR", "Afeganistão"),
        ("ro", "Afganistan"),
        ("ru", "Афганистан"),
        ("rw", "Afuganisita"),
        ("sc", "Afghànistan"),
        ("sd", "افغانستان"),
        ("si", "ඇෆ\u{dca}ගන\u{dd2}ස\u{dca}ත\u{dcf}නය"),
        ("sk", "Afganistan"),
        ("sl", "Afganistan"),
        ("so", "Afgaanistaan"),
        ("sq", "Afganistan"),
        ("sr", "Авганистан"),
        ("sv", "Afghanistan"),
        ("sw", "Afghanistani"),
        ("ta", "ஆப\u{bcd}க\u{bbe}னிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
        ("te", "ఆఫ\u{c4d}ఘన\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Афғонистон"),
        ("th", "อ\u{e31}ฟกาน\u{e34}สถาน"),
        ("ti", "ኣፍጋኒስታን"),
        ("tk", "Owganystan"),
        ("tl", "Apganistan"),
        ("tr", "Afganistan"),
        ("tt", "Әфгәнстан"),
        ("ug", "ئافغانىستان"),
        ("uk", "Афганістан"),
        ("ur", "افغانستان"),
        ("uz", "Afgʻoniston"),
        ("ve", "Afghanistan"),
        ("vi", "A Phú Hãn"),
        ("wa", "Afganistan"),
        ("wo", "Afganistaan"),
        ("xh", "Afghanistan"),
        ("yo", "Afghanístàn"),
        ("zh_CN", "阿富汗"),
        ("zh_HK", "阿富汗"),
        ("zh_TW", "阿富汗"),
        ("zu", "I-Afganistani"),
    ];
    #[cfg(all(feature = "af", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 33.93911;
        pub const LONGITUDE: f64 = 67.709953;
        pub const MAX_LATITUDE: f64 = 38.49087670000001;
        pub const MAX_LONGITUDE: f64 = 74.8898619;
        pub const MIN_LATITUDE: f64 = 29.3772;
        pub const MIN_LONGITUDE: f64 = 60.5170005;
        pub const NORTHEAST_LATITUDE: f64 = 38.49087670000001;
        pub const NORTHEAST_LONGITUDE: f64 = 74.8898619;
        pub const SOUTHWEST_LATITUDE: f64 = 29.3772;
        pub const SOUTHWEST_LONGITUDE: f64 = 60.5170005;
    }
}
#[cfg(all(feature = "af", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 33.93911,
            longitude: 67.709953,
            max_latitude: 38.49087670000001,
            max_longitude: 74.8898619,
            min_latitude: 29.3772,
            min_longitude: 60.5170005,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 38.49087670000001,
                    longitude: 74.8898619,
                },
                southwest: CountryGeoBound {
                    latitude: 29.3772,
                    longitude: 60.5170005,
                },
            },
        }
    }
}

#[cfg(all(feature = "af", feature = "subdivisions"))]
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
                    "BAL",
                    Subdivision{
                        name: "Balkh",
                        country_alpha2: Alpha2::AF,
                        code: "BAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7550603), longitude: Some(66.8975372), max_latitude: Some(36.7744019), min_latitude: Some(36.739847), max_longitude: Some(66.9103004), min_longitude: Some(66.879015)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بلخ"), ("az", "Bəlx vilayəti"), ("be", "Правінцыя Балх"), ("bg", "Балх"), ("bn", "ব\u{9be}ল\u{9cd}\u{200c}খ প\u{9cd}রদেশ"), ("ca", "Província de Balkh"), ("ccp", "𑄝𑄣\u{11133}𑄈\u{11134}"), ("ceb", "Balkh"), ("cs", "Balch"), ("cy", "Balkh"), ("da", "Balkh"), ("de", "Balch"), ("el", "Μπαλχ"), ("en", "Balkh"), ("es", "Provincia de Balj"), ("et", "Balkhi provints"), ("eu", "Balkh probintzia"), ("fa", "ولایت بلخ"), ("fi", "Balkhin maakunta"), ("fr", "Balkh"), ("gu", "બાલ\u{acd}ખ"), ("hi", "बल\u{94d}ख\u{93c} प\u{94d}रान\u{94d}त"), ("hu", "Balh tartomány"), ("id", "Provinsi Balkh"), ("it", "Balkh"), ("ja", "バルフ州"), ("ka", "ბალხის პროვინცია"), ("kn", "ಬಲ\u{ccd}ಕ\u{ccd}"), ("ko", "발흐 주"), ("lt", "Balcho provincija"), ("lv", "Balhas province"), ("mn", "Балх аймаг"), ("mr", "बाल\u{94d}क"), ("ms", "Wilayah Balkh"), ("nb", "Balkh"), ("nl", "Balch"), ("no", "Balkh"), ("pa", "ਬਲਖ ਸ\u{a42}ਬਾ"), ("pl", "Balch"), ("ps", "بلخ ولايت"), ("pt", "Balkh"), ("ro", "Provincia Balkh"), ("ru", "Балх"), ("si", "බ\u{dcf}ල\u{dca}ක\u{dca}"), ("sk", "Balch"), ("sr", "Провинција Балк"), ("sr_Latn", "Provincija Balk"), ("sv", "Balkh"), ("ta", "ப\u{bbe}ல\u{bcd}க\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ల\u{c4d}క\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}ลข\u{e4c}"), ("tr", "Belh Vilayeti"), ("uk", "Балх"), ("ur", "بلخ"), ("vi", "Balkh"), ("zh", "巴尔赫省")]),
                        unofficial_name_list: ["Balkh"].to_vec(),
                    }
                ),
                (
                    "BAM",
                    Subdivision{
                        name: "Bamian",
                        country_alpha2: Alpha2::AF,
                        code: "BAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.9073296), longitude: Some(67.1894488), max_latitude: Some(35.479285), min_latitude: Some(33.914066), max_longitude: Some(68.2664631), min_longitude: Some(66.2868759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية باميان"), ("az", "Bamiyan"), ("be", "Правінцыя Баміян"), ("bg", "Бамян"), ("bn", "ব\u{9be}মিয\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de Bamiyan"), ("ccp", "𑄝𑄟\u{11128}𑄠𑄚\u{11134}"), ("ceb", "Wilāyat-e Bāmyān"), ("cs", "Bamján"), ("cy", "Bamiyan"), ("da", "Bamiyan"), ("de", "Bamiyan"), ("el", "Μπαμιάν"), ("en", "Bamyan"), ("es", "Provincia de Bamiyan"), ("et", "Bāmīāni provints"), ("eu", "Bamijan probintzia"), ("fa", "ولایت بامیان"), ("fi", "Bamiyanin maakunta"), ("fr", "Bâmiyân"), ("gu", "બામયાન"), ("hi", "बामयान प\u{94d}रान\u{94d}त"), ("hu", "Bámiján tartomány"), ("hy", "Բամիան"), ("id", "Provinsi Bamiyan"), ("it", "provincia di Bamiyan"), ("ja", "バーミヤーン州"), ("ka", "ბამიანის პროვინცია"), ("kn", "ಬಮಯಾನ\u{ccd}"), ("ko", "바미안 주"), ("lt", "Bamijano provincija"), ("lv", "Bāmijānas province"), ("ml", "ബ\u{d3e}മിയ\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mn", "Бамиан аймаг"), ("mr", "बामयान"), ("ms", "Wilayah Bamiyan"), ("nb", "Bamiyan"), ("nl", "Bamyan"), ("no", "Bamiyan"), ("pl", "Bamian"), ("ps", "باميان ولايت"), ("pt", "Bamiyan"), ("ro", "Provincia Bamyan"), ("ru", "Бамиан"), ("sd", "باميان"), ("si", "බම\u{dca}යන\u{dca}"), ("sk", "Bámján"), ("sr", "Провинција Бамијан"), ("sr_Latn", "Provincija Bamijan"), ("sv", "Bamiyan"), ("ta", "ப\u{bbe}மிய\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బమ\u{c4d}యన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาม\u{e35}ยาน"), ("tr", "Bamyan Vilayeti"), ("uk", "Баміан"), ("ur", "بامیان"), ("vi", "Bamyan"), ("zh", "巴米扬省")]),
                        unofficial_name_list: ["Bamian", "Bamiyan", "Bāmīān"].to_vec(),
                    }
                ),
                (
                    "BDG",
                    Subdivision{
                        name: "Badghis",
                        country_alpha2: Alpha2::AF,
                        code: "BDG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.1671339), longitude: Some(63.7695384), max_latitude: Some(36.041935), min_latitude: Some(34.51659600000001), max_longitude: Some(65.07057689999999), min_longitude: Some(62.673593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بادغيس"), ("az", "Badğis"), ("be", "Правінцыя Бадгіс"), ("bg", "Багдис"), ("bn", "বদগিস প\u{9cd}রদেশ"), ("ca", "Badghis"), ("ccp", "𑄝𑄖\u{11134}𑄊\u{11128}𑄌\u{11134}"), ("ceb", "Bādghīs"), ("cs", "Bádghís"), ("cy", "Badghis"), ("da", "Badghis"), ("de", "Badghis"), ("el", "Μπαντγκίς"), ("en", "Badghis"), ("es", "Provincia de Bādgīs"), ("et", "Bādghīsi provints"), ("eu", "Badghis probintzia"), ("fa", "ولایت بادغیس"), ("fi", "Badghisin maakunta"), ("fr", "Bâdghîs"), ("gu", "બ\u{ac7}ડઘીસ"), ("hi", "बादग\u{93c}ीस प\u{94d}रान\u{94d}त"), ("hy", "Բադղիս"), ("id", "Provinsi Badghis"), ("it", "Badghis"), ("ja", "バードギース州"), ("ka", "ბადღისის პროვინცია"), ("kn", "ಬದ\u{ccd}ಗ\u{cbf}ಸ\u{ccd}"), ("ko", "바드기스 주"), ("lt", "Badgiso provincija"), ("lv", "Bādgīsa"), ("ml", "ബ\u{d3e}ദ\u{d4d}ഘീസ\u{d4d} പ\u{d4d}രവിശ\u{d4d}യ"), ("mn", "Бадгис аймаг"), ("mr", "ब\u{945}डघिस"), ("ms", "Badghis"), ("nb", "Badghis"), ("nl", "Badghis"), ("no", "Badghis"), ("pl", "Badghis"), ("ps", "بادغيس ولايت"), ("pt", "Badghis"), ("ro", "Provincia Badghis"), ("ru", "Бадгис"), ("si", "බඩ\u{dca}ග\u{dd2}ස\u{dca}"), ("sk", "Bádgís"), ("sr", "Провинција Бадгис"), ("sr_Latn", "Provincija Badgis"), ("sv", "Badghis"), ("ta", "பட\u{bcd}கிஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}డ\u{c4d}గ\u{c3f}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแบดจ\u{e34}ส"), ("tr", "Badgis Vilayeti"), ("uk", "Бадгіс"), ("ur", "صوبہ بادغیس"), ("vi", "Badghis"), ("zh", "巴德吉斯省")]),
                        unofficial_name_list: ["Badghis", "Badgis", "Bādghīs"].to_vec(),
                    }
                ),
                (
                    "BDS",
                    Subdivision{
                        name: "Badakhshan",
                        country_alpha2: Alpha2::AF,
                        code: "BDS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7347725), longitude: Some(70.81199529999999), max_latitude: Some(38.4906109), min_latitude: Some(35.44512100000001), max_longitude: Some(74.8901951), min_longitude: Some(69.98609689999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بدخشان"), ("az", "Bədəxşan"), ("be", "Правінцыя Бадахшан"), ("bg", "Бадахшан"), ("bn", "ব\u{9be}দ\u{9be}খশন প\u{9cd}রদেশ"), ("ca", "Província de Badakhxan"), ("ccp", "𑄝𑄘𑄇\u{11134}𑄥𑄚\u{11134}"), ("ceb", "Badakhshān"), ("cs", "Badachšán"), ("cy", "Badakhshan"), ("da", "Badakhshan"), ("de", "Badachschan"), ("el", "Μπανταχσάν"), ("en", "Badakhshan"), ("es", "Provincia de Badakhshan"), ("et", "Badakhshāni provints"), ("eu", "Badakhshan probintzia"), ("fa", "ولایت بدخشان"), ("fi", "Badahšan"), ("fr", "Badakhchan"), ("gl", "Badaghxán"), ("gu", "બદાખશન"), ("hi", "बदख\u{93c}\u{94d}शान प\u{94d}रान\u{94d}त"), ("hr", "Provincija Badakhšan"), ("hy", "Բադախշան"), ("id", "Provinsi Badakhshan"), ("it", "Badakhshan"), ("ja", "バダフシャーン州"), ("ka", "ბადახშანის პროვინცია"), ("kn", "ಬಡಾಖ\u{ccd}ಶನ\u{ccd}"), ("ko", "바다흐샨 주"), ("lt", "Badachšanas"), ("lv", "Badahšāna"), ("ml", "ബദഖ\u{d4d}ശ\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mn", "Бадахшан аймаг"), ("mr", "बदखाशन"), ("ms", "Badakhshan"), ("nb", "Badakhshan"), ("nl", "Badachsjan"), ("no", "Badakhshan"), ("pa", "ਸ\u{a42}ਬਾ ਬਦਖ\u{a3c}ਸ\u{a3c}ਾ\u{a02}"), ("pl", "Badachszan"), ("ps", "بدخشان ولايت"), ("pt", "Badakhshan"), ("ro", "Provincia Badakhshan"), ("ru", "Бадахшан"), ("si", "බදක\u{dca}ශ\u{dcf}න\u{dca}"), ("sk", "Badachšán"), ("sr", "Провинција Бадахсан"), ("sr_Latn", "Provincija Badahsan"), ("sv", "Badakhshan"), ("ta", "பட\u{bbe}க\u{bcd}ச\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బడ\u{c3e}క\u{c4d}షన\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาด\u{e31}คชาน"), ("tr", "Badahşan Vilayeti"), ("uk", "Бадахшан"), ("ur", "صوبہ بدخشاں"), ("vi", "Badakhshan"), ("zh", "巴达赫尚省")]),
                        unofficial_name_list: ["Badah\u{331}šan"].to_vec(),
                    }
                ),
                (
                    "BGL",
                    Subdivision{
                        name: "Baghlan",
                        country_alpha2: Alpha2::AF,
                        code: "BGL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.17890260000001), longitude: Some(68.74530639999999), max_latitude: Some(36.1897066), min_latitude: Some(36.162409), max_longitude: Some(68.7745), min_longitude: Some(68.7236881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بغلان"), ("az", "Bəğlan"), ("be", "Правінцыя Баглан"), ("bg", "Баглан"), ("bn", "ব\u{9be}গলন প\u{9cd}রদেশ"), ("ca", "Província de Baghlan"), ("ccp", "𑄝𑄇\u{11134}𑄣𑄚\u{11134}"), ("ceb", "Wilāyat-e Baghlān"), ("cs", "Baghlán"), ("cy", "Baghlan"), ("da", "Baghlan"), ("de", "Baglan"), ("el", "Μπαγκλάν"), ("en", "Baghlan"), ("es", "Provincia de Baghlan"), ("et", "Baghlāni provints"), ("eu", "Baghlan probintzia"), ("fa", "ولایت بغلان"), ("fi", "Baghlanin maakunta"), ("fr", "Baghlân"), ("gu", "બઘલાન"), ("hi", "बग\u{93c}लान प\u{94d}रान\u{94d}त"), ("hu", "Baglán tartomány"), ("id", "Provinsi Baghlan"), ("it", "Baghlan"), ("ja", "バグラーン州"), ("ka", "ბაღლანის პროვინცია"), ("kn", "ಬಾಗ\u{ccd}ಲಾನ\u{ccd}"), ("ko", "바글란 주"), ("lt", "Baglano provincija"), ("lv", "Baglāna"), ("mn", "Баглан аймаг"), ("mr", "बागलाण"), ("ms", "Wilayah Baghlan"), ("nb", "Baghlan"), ("nl", "Baghlan"), ("no", "Baghlan"), ("pl", "Baghlan"), ("ps", "بغلان ولايت"), ("pt", "Baghlan"), ("ro", "Provincia Baghlan"), ("ru", "Баглан"), ("si", "බැග\u{dca}ලන\u{dca}"), ("sr", "Провинција Баглан"), ("sr_Latn", "Provincija Baglan"), ("sv", "Baghlan"), ("ta", "பக\u{bcd}ல\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}గ\u{c4d}ల\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบางก\u{e4c}ลาน"), ("tr", "Baglan Vilayeti"), ("uk", "Баглан"), ("ur", "صوبہ بغلان"), ("vi", "Baghlan"), ("zh", "巴格兰省")]),
                        unofficial_name_list: ["Baghlan", "Baghlān", "Baglan"].to_vec(),
                    }
                ),
                (
                    "DAY",
                    Subdivision{
                        name: "Daykondi",
                        country_alpha2: Alpha2::AF,
                        code: "DAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.669495), longitude: Some(66.0463534), max_latitude: Some(34.36751), min_latitude: Some(32.932084), max_longitude: Some(67.4245309), min_longitude: Some(65.23819689999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية دايكندي"), ("az", "Daykündi"), ("be", "Правінцыя Дайкундзі"), ("bg", "Дайкунди"), ("bn", "দইকোন\u{9cd}দি প\u{9cd}রদেশ"), ("ca", "Província de Daykundi"), ("ccp", "𑄓𑄬𑄇\u{1112a}𑄚\u{11134}𑄘\u{11128}"), ("ceb", "Daykundi Province"), ("cs", "Dájkundí"), ("da", "Daikondi"), ("de", "Daikondi"), ("el", "Ντεϊκούντι"), ("en", "Daykundi"), ("es", "Provincia de Daikondi"), ("et", "Dāykondī provints"), ("eu", "Daikondi probintzia"), ("fa", "ولایت دایکندی"), ("fi", "Daikondin maakunta"), ("fr", "Deykandi"), ("hi", "दायक\u{941}\u{902}दी प\u{94d}रान\u{94d}त"), ("id", "Provinsi Daykundi"), ("it", "Daikondi"), ("ja", "ダーイクンディー州"), ("ka", "დაიკუნდის პროვინცია"), ("ko", "다이쿤디 주"), ("lt", "Daikondžio provincija"), ("lv", "Dājkondī province"), ("mn", "Дайкунди аймаг"), ("ms", "Wilayah Daykundi"), ("nb", "Daikondi"), ("nl", "Daikondi"), ("no", "Daikondi"), ("pl", "Dajkondi"), ("ps", "دايکندي ولايت"), ("pt", "Daikondi"), ("ro", "Provincia Daykundi"), ("ru", "Дайкунди"), ("sk", "Dájkundí"), ("sr", "Провинција Даиконди"), ("sr_Latn", "Provincija Daikondi"), ("sv", "Daikondi"), ("ta", "தேக\u{bcd}கண\u{bcd}டி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "Daykundi Vilayeti"), ("uk", "Дайкунді"), ("ur", "صوبہ دایکندی"), ("vi", "Daykundi"), ("zh", "代孔迪省")]),
                        unofficial_name_list: ["Daikondi"].to_vec(),
                    }
                ),
                (
                    "FRA",
                    Subdivision{
                        name: "Farah",
                        country_alpha2: Alpha2::AF,
                        code: "FRA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.495328), longitude: Some(62.2626627), max_latitude: Some(33.586982), min_latitude: Some(31.3958321), max_longitude: Some(64.750407), min_longitude: Some(60.5861239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية فراه"), ("az", "Fərah"), ("be", "Правінцыя Фарах"), ("bg", "Фарах"), ("bn", "ফ\u{9be}রহ প\u{9cd}রদেশ"), ("ca", "Província de Farah"), ("ccp", "𑄜𑄢𑄦\u{11134}"), ("ceb", "Farāh (lalawigan sa Apganistan)"), ("cs", "Faráh"), ("cy", "Farah"), ("da", "Farah"), ("de", "Farah"), ("el", "Φαράχ"), ("en", "Farah"), ("es", "Provincia de Farāh"), ("et", "Farāhi provints"), ("fa", "ولایت فراه"), ("fi", "Farahin maakunta"), ("fr", "Farâh"), ("gu", "ફરાહ"), ("hi", "फ\u{93c}राह प\u{94d}रान\u{94d}त"), ("id", "Provinsi Farah"), ("it", "provincia di Farah"), ("ja", "ファラー州"), ("ka", "ფარაჰის პროვინცია"), ("kn", "ಫರಾಹ\u{ccd}"), ("ko", "파라 주"), ("lt", "Faraho provincija"), ("lv", "Farāha"), ("mn", "Фарах аймаг"), ("mr", "फराह"), ("ms", "Wilayah Farah"), ("nb", "Farah"), ("nl", "Farah"), ("no", "Farah"), ("pl", "Farah"), ("ps", "فراه ولايت"), ("pt", "Farah"), ("ro", "Provincia Farah"), ("ru", "Фарах"), ("si", "ෆ\u{dcf}ර\u{dcf}"), ("sk", "Faráh"), ("sr", "Провинција Фарах"), ("sr_Latn", "Provincija Farah"), ("sv", "Farah"), ("ta", "ப\u{bbe}ர\u{bbe}"), ("te", "ఫర\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดฟาราห\u{e4c}"), ("tr", "Ferah Vilayeti"), ("uk", "Фарах"), ("ur", "صوبہ فراہ"), ("vi", "Farah"), ("zh", "法拉省")]),
                        unofficial_name_list: ["Fahrah"].to_vec(),
                    }
                ),
                (
                    "FYB",
                    Subdivision{
                        name: "Faryab",
                        country_alpha2: Alpha2::AF,
                        code: "FYB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.0795613), longitude: Some(64.90595499999999), max_latitude: Some(37.2501681), min_latitude: Some(35.1631961), max_longitude: Some(65.822068), min_longitude: Some(63.8947181)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية فاریاب"), ("az", "Fəryab"), ("bg", "Фаряб"), ("bn", "ফ\u{9be}রিয\u{9bc}ব প\u{9cd}রদেশ"), ("ca", "Província de Faryab"), ("ccp", "𑄜𑄢\u{11128}𑄠𑄛\u{11134}"), ("ceb", "Faryab Province"), ("cs", "Fárjáb"), ("cy", "Faryab"), ("da", "Faryab"), ("de", "Faryab"), ("el", "Φαρυάμπ"), ("en", "Faryab"), ("es", "Provincia de Fāryāb"), ("et", "Fāryābi provints"), ("eu", "Fariab probintzia"), ("fa", "ولایت فاریاب"), ("fi", "Faryabin maakunta"), ("fr", "Fâryâb"), ("gu", "ફર\u{acd}યાબ"), ("hi", "फ\u{93c}ारयाब प\u{94d}रान\u{94d}त"), ("id", "Provinsi Faryab"), ("it", "Faryab"), ("ja", "ファーリヤーブ州"), ("ka", "ფარიაბის პროვინცია"), ("kn", "ಫರ\u{cbf}ಯಾಬ\u{ccd}"), ("ko", "파르야브 주"), ("lt", "Farjabo provincija"), ("lv", "Farjaba"), ("mn", "Фарьяб аймаг"), ("mr", "फर\u{94d}याब"), ("ms", "Wilayah Faryab"), ("nb", "Faryab"), ("ne", "फर\u{94d}याब"), ("nl", "Faryab"), ("no", "Faryab"), ("pl", "Farjab"), ("ps", "فارياب ولايت"), ("pt", "Faryab"), ("ro", "Provincia Faryab"), ("ru", "Фарьяб"), ("si", "ෆර\u{dca}යබ\u{dca}"), ("sk", "Fárjáb"), ("sr", "Провинција Фарјаб"), ("sr_Latn", "Provincija Farjab"), ("sv", "Faryab"), ("ta", "ப\u{bbe}ர\u{bcd}ய\u{bbe}ப\u{bcd}"), ("te", "ఫర\u{c4d}య\u{c3e}బ\u{c4d}"), ("th", "ฟาร\u{e4c}ย\u{e31}บ"), ("tr", "Faryab Vilayeti"), ("uk", "Фарʼяб"), ("ur", "صوبہ فاریاب"), ("uz", "Foryob (viloyat)"), ("vi", "Faryab"), ("zh", "法利亚布省")]),
                        unofficial_name_list: ["Fariab"].to_vec(),
                    }
                ),
                (
                    "GHA",
                    Subdivision{
                        name: "Ghazni",
                        country_alpha2: Alpha2::AF,
                        code: "GHA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.4982289), longitude: Some(67.7615983), max_latitude: Some(34.2320759), min_latitude: Some(32.085223), max_longitude: Some(68.834594), min_longitude: Some(66.8315989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غزني"), ("az", "Qəzni"), ("be", "Правінцыя Газні"), ("bg", "Газни"), ("bn", "গজনি প\u{9cd}রদেশ"), ("ca", "Província de Ghazni"), ("ccp", "𑄊𑄌\u{11134}𑄚\u{11128}"), ("ceb", "Wilāyat-e Ghaznī"), ("cs", "Ghazní"), ("cy", "Ghazni"), ("da", "Ghazni"), ("de", "Ghazni"), ("el", "Γκαζνί"), ("en", "Ghazni"), ("es", "Provincia de Gazni"), ("et", "Ghaznī provints"), ("eu", "Ghazni probintzia"), ("fa", "ولایت غزنی"), ("fi", "Ghaznin maakunta"), ("fr", "Ghazni"), ("gu", "ગઝની"), ("hi", "ग\u{93c}ज\u{93c}नी प\u{94d}रान\u{94d}त"), ("hy", "Ղազնի"), ("id", "Provinsi Ghazni"), ("it", "provincia di Ghazni"), ("ja", "ガズニー州"), ("ka", "ღაზნის პროვინცია"), ("kn", "ಘಜ\u{ccd}ನ\u{cbf}"), ("ko", "가즈니 주"), ("lt", "Gaznio provincija"), ("lv", "Gazni province"), ("mn", "Газни аймаг"), ("mr", "गझनी"), ("ms", "Ghazni"), ("nb", "Ghazni"), ("nl", "Ghazni"), ("no", "Ghazni"), ("pa", "ਗਜ\u{a3c}ਨੀ ਸ\u{a42}ਬਾ"), ("pl", "Ghazni"), ("ps", "غزني ولايت"), ("pt", "Ghazni"), ("ro", "Provincia Ghazni"), ("ru", "Газни"), ("si", "ඝස\u{dca}න\u{dd2}"), ("sk", "Gazní"), ("sr", "Провинција Газни"), ("sr_Latn", "Provincija Gazni"), ("sv", "Ghazni"), ("ta", "கஜினி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఘజ\u{c3f}న\u{c3f}"), ("th", "แกซน\u{e34}"), ("tr", "Gazni Vilayeti"), ("uk", "Газні"), ("ur", "صوبہ غزنی"), ("vi", "Ghazni"), ("zh", "加兹尼省")]),
                        unofficial_name_list: ["Gazni", "Ghazni"].to_vec(),
                    }
                ),
                (
                    "GHO",
                    Subdivision{
                        name: "Ghowr",
                        country_alpha2: Alpha2::AF,
                        code: "GHO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0995776), longitude: Some(64.90595499999999), max_latitude: Some(35.276925), min_latitude: Some(33.133217), max_longitude: Some(66.736645), min_longitude: Some(63.19821899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غور"), ("az", "Qövr"), ("be", "Правінцыя Гор"), ("bg", "Гор"), ("bn", "গোর প\u{9cd}রদেশ"), ("ca", "Ghur"), ("ccp", "𑄊\u{1112e}𑄢\u{11134}"), ("ceb", "Wilāyat-e Ghōr"), ("cs", "Ghór"), ("cy", "Ghor"), ("da", "Ghowr"), ("de", "Ghor"), ("el", "Γκοούρ"), ("en", "Ghōr"), ("es", "Provincia de Ġawr"), ("et", "Ghowri provints"), ("eu", "Ghor probintzia"), ("fa", "ولایت غور"), ("fi", "Ghowrin maakunta"), ("fr", "Ghôr"), ("gu", "અલ\u{acd}માટી"), ("hi", "ग\u{93c}ोर प\u{94d}रान\u{94d}त"), ("id", "Provinsi Ghōr"), ("it", "Ghowr"), ("ja", "ゴール州"), ("ka", "ღორის პროვინცია"), ("kn", "ಘೋರ\u{ccd}"), ("ko", "구르 주"), ("lt", "Goro provincija"), ("lv", "Gouras province"), ("mn", "Гор аймаг"), ("mr", "घोर"), ("ms", "Ghor"), ("nb", "Ghowr"), ("nl", "Ghowr"), ("no", "Ghowr"), ("pa", "ਗ\u{a3c}\u{a4b}ਰ ਸ\u{a42}ਬਾ"), ("pl", "Ghor"), ("ps", "غور ولايت"), ("pt", "Ghowr"), ("ro", "Provincia Ghur"), ("ru", "Гур"), ("si", "ඝෝර\u{dca}"), ("sr", "Провинција Гор"), ("sr_Latn", "Provincija Gor"), ("sv", "Ghor"), ("ta", "ஜிஹ\u{bbe}ர\u{bcd}"), ("te", "ఘ\u{c4b}ర\u{c4d}"), ("th", "กอฮ\u{e4c}"), ("tr", "Gur Vilayeti"), ("uk", "Гор"), ("ur", "صوبہ غور"), ("vi", "Ghor"), ("zh", "古爾省")]),
                        unofficial_name_list: ["Ghawr", "Ghor", "Ghowr", "Gōr"].to_vec(),
                    }
                ),
                (
                    "HEL",
                    Subdivision{
                        name: "Helmand",
                        country_alpha2: Alpha2::AF,
                        code: "HEL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.3636474), longitude: Some(63.95861110000001), max_latitude: Some(33.3782999), min_latitude: Some(29.377478), max_longitude: Some(65.38026289999999), min_longitude: Some(62.553695)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية هلمند"), ("az", "Hilmənd"), ("be", "Правінцыя Гільменд"), ("bg", "Хелманд"), ("bn", "হেলম\u{9be}ন\u{9cd}দ প\u{9cd}রদেশ"), ("ca", "Província d’Helmand"), ("ccp", "𑄦𑄬𑄣\u{11134}𑄟𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Helmand"), ("cs", "Hilmand"), ("cy", "Helmand"), ("da", "Helmand"), ("de", "Helmand"), ("el", "Χελμάντ"), ("en", "Helmand"), ("es", "Provincia de Helmand"), ("et", "Helmandi provints"), ("eu", "Helmand probintzia"), ("fa", "ولایت هلمند"), ("fi", "Helmandin maakunta"), ("fr", "Helmand"), ("gu", "હ\u{ac7}લ\u{acd}મ\u{a82}ડ"), ("he", "מחוז הלמן"), ("hi", "ह\u{947}लम\u{902}द प\u{94d}रान\u{94d}त"), ("hu", "Helmand tartomány"), ("hy", "Հիլմենդ"), ("id", "Provinsi Helmand"), ("it", "Helmand"), ("ja", "ヘルマンド州"), ("ka", "ჰელმანდის პროვინცია"), ("kn", "ಹ\u{cc6}ಲ\u{ccd}ಮಾಂಡ\u{ccd}"), ("ko", "헬만드 주"), ("lt", "Helmando provincija"), ("lv", "Helmandas province"), ("mn", "Хильменд аймаг"), ("mr", "ह\u{947}लम\u{902}ड"), ("ms", "Helmand"), ("nb", "Helmand"), ("nl", "Helmand"), ("no", "Helmand"), ("pa", "ਹ\u{a47}ਲਮ\u{a70}ਦ ਸ\u{a42}ਬਾ"), ("pl", "Helmand"), ("ps", "هلمند ولايت"), ("pt", "Helmand"), ("ro", "Provincia Helmand"), ("ru", "Гильменд"), ("si", "හෙල\u{dca}මන\u{dca}ඩ\u{dca}"), ("sr", "Провинција Хелманд"), ("sr_Latn", "Provincija Helmand"), ("sv", "Helmand"), ("ta", "ஹெல\u{bcd}மண\u{bcd}டு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హ\u{c46}ల\u{c4d}మండ\u{c4d}"), ("th", "เฮลซ\u{e4c}แมนด\u{e4c}"), ("tr", "Helmend Vilayeti"), ("uk", "Гільменд"), ("ur", "صوبہ ہلمند"), ("vi", "Helmand"), ("zh", "赫尔曼德省")]),
                        unofficial_name_list: ["Helmand", "Hilmend"].to_vec(),
                    }
                ),
                (
                    "HER",
                    Subdivision{
                        name: "Herat",
                        country_alpha2: Alpha2::AF,
                        code: "HER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.341944), longitude: Some(62.203056), max_latitude: Some(34.3883543), min_latitude: Some(34.3185577), max_longitude: Some(62.2713661), min_longitude: Some(62.150774)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية هرات"), ("az", "Herat vilayəti"), ("be", "Правінцыя Герат"), ("bg", "Херат"), ("bn", "হেরত প\u{9cd}রদেশ"), ("ca", "Província d’Herat"), ("ccp", "𑄦𑄬𑄢𑄖\u{11134}"), ("ceb", "Herāt"), ("cs", "Herát"), ("cy", "Herat"), ("da", "Herat"), ("de", "Herat"), ("el", "Χεράτ"), ("en", "Herat"), ("es", "Provincia de Herāt"), ("et", "Herāti provints"), ("eu", "Herat probintzia"), ("fa", "ولایت هرات"), ("fi", "Heratin maakunta"), ("fr", "Hérât"), ("gu", "હ\u{ac7}રાત"), ("hi", "ह\u{947}रात प\u{94d}रान\u{94d}त"), ("id", "Provinsi Herat"), ("it", "provincia di Herat"), ("ja", "ヘラート州"), ("ka", "ჰერათის პროვინცია"), ("kn", "ಹ\u{cc6}ರಾಟ\u{ccd}"), ("ko", "헤라트 주"), ("lt", "Herato provincija"), ("lv", "Herāta"), ("mn", "Херат аймаг"), ("mr", "ह\u{947}रात"), ("ms", "Herat"), ("nb", "Herat"), ("nl", "Herat"), ("no", "Herat"), ("pl", "Herat"), ("ps", "هرات ولايت"), ("pt", "Herat"), ("ro", "Provincia Herat"), ("ru", "Герат"), ("si", "හෙරට\u{dca}"), ("sr", "Провинција Херат"), ("sr_Latn", "Provincija Herat"), ("sv", "Herat"), ("ta", "ஹேரத\u{bcd}"), ("te", "హ\u{c46}ర\u{c3e}ట\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฮร\u{e31}ท"), ("tr", "Herat Vilayeti"), ("uk", "Герат"), ("ur", "صوبہ ہرات"), ("vi", "Herat"), ("zh", "赫拉特省")]),
                        unofficial_name_list: ["Herat"].to_vec(),
                    }
                ),
                (
                    "JOW",
                    Subdivision{
                        name: "Jowzjan",
                        country_alpha2: Alpha2::AF,
                        code: "JOW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.8969692), longitude: Some(65.6658568), max_latitude: Some(37.544579), min_latitude: Some(35.90039410000001), max_longitude: Some(66.583344), min_longitude: Some(65.1666699)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية جوزجان"), ("az", "Cövzcan"), ("be", "Правінцыя Джаўзджан"), ("bg", "Джаузджан"), ("bn", "জোওয\u{9cd}\u{200c}জ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de Jowzjan"), ("ccp", "𑄎\u{11127}𑄠\u{1112e}𑄌\u{11134}𑄎𑄚\u{11134}"), ("ceb", "Wilāyat-e Jowzjān"), ("cs", "Džúzdžán"), ("da", "Jowzjan"), ("de", "Dschuzdschan"), ("el", "Τζοουζτζάν"), ("en", "Jowzjan"), ("es", "Provincia de Jawzjān"), ("et", "Jowzjāni provints"), ("eu", "Jowzjan probintzia"), ("fa", "ولایت جوزجان"), ("fi", "Jowzjanin maakunta"), ("fr", "Djôzdjân"), ("gu", "જોઝજન"), ("hi", "जोज\u{93c}जान प\u{94d}रान\u{94d}त"), ("id", "Provinsi Jowzjan"), ("it", "Jowzjan"), ("ja", "ジューズジャーン州"), ("ka", "ჯუზჯანის პროვინცია"), ("kn", "ಜೋವ\u{ccd}ಜನ\u{ccd}"), ("ko", "주즈잔 주"), ("lt", "Džouzdžano provincija"), ("lv", "Džouzdžānas province"), ("mn", "Жаузжан аймаг"), ("mr", "जोज\u{94d}जन"), ("ms", "Wilayah Jowzjan"), ("nb", "Jowzjan"), ("nl", "Jowzjan"), ("no", "Jowzjan"), ("pa", "ਜ\u{a4b}ਜਾਨ"), ("pl", "Dżozdżan"), ("ps", "جوزجان ولايت"), ("pt", "Jowzjan"), ("ro", "Provincia Jowzjan"), ("ru", "Джаузджан"), ("si", "ජෝස\u{dca}ජ\u{dcf}න\u{dca}"), ("sl", "provinca Jowzjan, Afganistan"), ("sr", "Провинција Ђузан"), ("sr_Latn", "Provincija Đuzan"), ("sv", "Jowzjan"), ("ta", "ஜோவ\u{bcd}ஸ\u{bcd}ஜன\u{bcd}"), ("te", "జ\u{c4b}వ\u{c4d}\u{200c}జ\u{c3e}న\u{c4d}"), ("th", "ซ\u{e34}นต\u{e4c}มาร\u{e4c}เต\u{e34}น"), ("tr", "Cüzcan Vilayeti"), ("uk", "Джаузджан"), ("ur", "صوبہ جوزجان"), ("uz", "Joʻzjon"), ("vi", "Jowzjan"), ("zh", "朱兹詹省")]),
                        unofficial_name_list: ["Jawzjan", "Jowzjan", "Jowzjān", "Jozjan", "Juzjan"].to_vec(),
                    }
                ),
                (
                    "KAB",
                    Subdivision{
                        name: "Kabul [Kabol]",
                        country_alpha2: Alpha2::AF,
                        code: "KAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.533333), longitude: Some(69.166667), max_latitude: Some(34.7619227), min_latitude: Some(34.3454912), max_longitude: Some(69.4459534), min_longitude: Some(68.9495086)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كابول"), ("az", "Kabil vilayəti"), ("be", "Правінцыя Кабул"), ("bg", "Кабул"), ("bn", "ক\u{9be}ব\u{9c1}ল প\u{9cd}রদেশ"), ("ca", "Província de Kabul"), ("ccp", "𑄇𑄝\u{1112a}𑄣\u{11134}"), ("ceb", "Wilāyat-e Kābul"), ("cs", "Kábul"), ("cy", "Kabul"), ("da", "Kabul"), ("de", "Kabul"), ("el", "Καμπούλ"), ("en", "Kabul"), ("es", "Provincia de Kabul"), ("et", "Kabuli provints"), ("eu", "Kabul probintzia"), ("fa", "ولایت کابل"), ("fi", "Kabulin maakunta"), ("fr", "Kaboul"), ("gu", "કાબ\u{ac1}લ પ\u{acd}રા\u{a82}ત"), ("hi", "काब\u{941}ल प\u{94d}रान\u{94d}त"), ("id", "Provinsi Kabul"), ("it", "provincia di Kabul"), ("ja", "カーブル州"), ("ka", "ქაბულის პროვინცია"), ("kn", "ಕಾಬುಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카불 주"), ("lt", "Kabulo provincija"), ("lv", "Kabulas province"), ("mn", "Кабул аймаг"), ("mr", "काब\u{941}ल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kabul"), ("nb", "Kabul"), ("nl", "Kabul"), ("no", "Kabul"), ("pl", "Kabul"), ("ps", "کابل ولايت"), ("pt", "Cabul"), ("ro", "Provincia Kabul"), ("ru", "Кабул"), ("si", "ක\u{dcf}බ\u{dd4}ල\u{dca} පළ\u{dcf}ත"), ("sr", "Провинција Кабул"), ("sr_Latn", "Provincija Kabul"), ("sv", "Kabul"), ("ta", "க\u{bbe}பூல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}బూల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาบ\u{e39}ล"), ("tk", "Kabul welaýaty"), ("tr", "Kâbil Vilayeti"), ("uk", "Кабул"), ("ur", "صوبہ کابل"), ("vi", "Tỉnh Kabul"), ("zh", "喀布尔省")]),
                        unofficial_name_list: ["Kabol", "Kabul", "Kābol", "Kābul"].to_vec(),
                    }
                ),
                (
                    "KAN",
                    Subdivision{
                        name: "Kandahar",
                        country_alpha2: Alpha2::AF,
                        code: "KAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.6119), longitude: Some(65.6811), max_latitude: Some(31.6725218), min_latitude: Some(31.5973985), max_longitude: Some(65.8307648), min_longitude: Some(65.6695744)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية قندهار"), ("az", "Qəndəhar vilayəti"), ("be", "Правінцыя Кандагар"), ("bg", "Кандахар"), ("bn", "ক\u{9be}ন\u{9cd}দ\u{9be}হ\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Kandahar"), ("ccp", "𑄇𑄚\u{11134}𑄘𑄦𑄢\u{11134}"), ("ceb", "Kandahār (lalawigan)"), ("cs", "Kandahár"), ("cy", "Kandahar"), ("da", "Kandahar"), ("de", "Kandahar"), ("el", "Κανταχάρ"), ("en", "Kandahar"), ("es", "Provincia de Kandahar"), ("et", "Kandahāri provints"), ("eu", "Kandahar probintzia"), ("fa", "ولایت قندهار"), ("fi", "Kandaharin maakunta"), ("fr", "Kandahâr"), ("gu", "ક\u{a82}દાહર"), ("he", "מחוז קנדהאר"), ("hi", "क\u{902}दहार प\u{94d}रान\u{94d}त"), ("hy", "Ղանդահար վիլայեթ"), ("id", "Provinsi Kandahar"), ("it", "provincia di Qandahar"), ("ja", "カンダハール州"), ("ka", "ყანდაარის პროვინცია"), ("kn", "ಕಂಧಹಾರ\u{ccd}"), ("ko", "칸다하르 주"), ("lt", "Kandaharo provincija"), ("lv", "Kandahāra"), ("mn", "Кандахар аймаг"), ("mr", "क\u{902}दाहार"), ("ms", "Wilayah Kandahar"), ("nb", "Kandahar"), ("nl", "Kandahar"), ("no", "Kandahar"), ("pl", "Kandahar"), ("ps", "کندهار ولايت"), ("pt", "Kandahar"), ("ro", "Provincia Kandahar"), ("ru", "Кандагар"), ("si", "කන\u{dca}දහ\u{dcf}ර\u{dca}"), ("sr", "Провинција Кандахар"), ("sr_Latn", "Provincija Kandahar"), ("sv", "Kandahar"), ("ta", "கந\u{bcd}தக\u{bbe}ர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ందహ\u{c3e}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e31}นดะฮาร\u{e4c}"), ("tr", "Kandehar Vilayeti"), ("uk", "Кандагар"), ("ur", "صوبہ قندھار"), ("vi", "Kandahar"), ("zh", "坎大哈省")]),
                        unofficial_name_list: ["Kandahar"].to_vec(),
                    }
                ),
                (
                    "KAP",
                    Subdivision{
                        name: "Kapisa",
                        country_alpha2: Alpha2::AF,
                        code: "KAP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.9810572), longitude: Some(69.6214562), max_latitude: Some(35.1843449), min_latitude: Some(34.636847), max_longitude: Some(69.9292071), min_longitude: Some(69.27812709999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كابيسا"), ("az", "Kapisa"), ("be", "Правінцыя Капіса"), ("bg", "Каписа"), ("bn", "কপিস’ প\u{9cd}রদেশ"), ("ca", "Província de Kapisa"), ("ccp", "𑄇𑄛\u{11128}𑄥"), ("ceb", "Wilāyat-e Kāpīsā"), ("cs", "Kápísá"), ("cy", "Kapisa"), ("da", "Kapisa"), ("de", "Kapisa"), ("el", "Καπίσα"), ("en", "Kapisa"), ("es", "Provincia de Kāpīsā"), ("et", "Kāpīsā provints"), ("eu", "Kapisa probintzia"), ("fa", "ولایت کاپیسا"), ("fi", "Kapisan maakunta"), ("fr", "Kâpîssâ"), ("gu", "કપિસા"), ("hi", "कापीसा प\u{94d}रान\u{94d}त"), ("id", "Provinsi Kapisa"), ("it", "Kapisa"), ("ja", "カーピーサー州"), ("ka", "კაპისას პროვინცია"), ("kn", "ಕಪ\u{cbf}ಸಾ"), ("ko", "카피사 주"), ("lt", "Kapisos provincija"), ("lv", "Kapisa"), ("mn", "Каписа аймаг"), ("mr", "कपिसा"), ("ms", "Wilayah Kapisa"), ("nb", "Kapisa"), ("nl", "Kapisa"), ("no", "Kapisa"), ("pa", "ਕਾਪੀਸਾ"), ("pl", "Kapisa"), ("ps", "کاپيسا ولايت"), ("pt", "Capisa"), ("ro", "Provincia Kapisa"), ("ru", "Каписа"), ("si", "කප\u{dd2}ස\u{dcf}"), ("sr", "Провинција Каписа"), ("sr_Latn", "Provincija Kapisa"), ("sv", "Kapisa"), ("ta", "கப\u{bc0}ஸ\u{bbe}"), ("te", "క\u{c3e}ప\u{c3f}స\u{c3e}"), ("th", "คาป\u{e34}ซา"), ("tr", "Kapisa Vilayeti"), ("uk", "Капіса"), ("ur", "صوبہ کاپیسا"), ("vi", "Kapisa"), ("zh", "卡比萨省")]),
                        unofficial_name_list: ["Kapesa", "Kapisa", "Kapissa"].to_vec(),
                    }
                ),
                (
                    "KDZ",
                    Subdivision{
                        name: "Kondoz [Kunduz]",
                        country_alpha2: Alpha2::AF,
                        code: "KDZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.7285907), longitude: Some(68.8680663), max_latitude: Some(36.77299259999999), min_latitude: Some(36.6518943), max_longitude: Some(68.9773179), min_longitude: Some(68.7845421)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كندوز"), ("az", "Künduz"), ("be", "Правінцыя Кундуз"), ("bg", "Кундуз"), ("bn", "ক\u{9c1}ন\u{9cd}দ\u{9c1}জ প\u{9cd}রদেশ"), ("ca", "Província de Kunduz"), ("ccp", "𑄇\u{1112a}𑄚\u{11134}𑄓\u{1112a}𑄌\u{11134}"), ("ceb", "Kunduz"), ("cs", "Kundúz"), ("cy", "Kunduz"), ("da", "Kunduz"), ("de", "Kunduz"), ("el", "Κουντούζ"), ("en", "Kunduz"), ("es", "Provincia de Qundūz"), ("et", "Kondūzi provints"), ("eu", "Kunduz probintzia"), ("fa", "ولایت کندوز"), ("fi", "Qunduzin maakunta"), ("fr", "Kondôz"), ("gu", "ક\u{ac1}ન\u{acd}ડ\u{ac1}ઝ પ\u{acd}રા\u{a82}ત"), ("he", "קונדוז"), ("hi", "क\u{941}\u{902}द\u{941}ज\u{93c} प\u{94d}रान\u{94d}त"), ("id", "Provinsi Kondoz"), ("it", "provincia di Konduz"), ("ja", "クンドゥーズ州"), ("ka", "ყუნდუზის პროვინცია"), ("kn", "ಕುಂಡುಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿤두즈 주"), ("lt", "Kundūzo provincija"), ("lv", "Kondozas province"), ("mn", "Кундуз аймаг"), ("mr", "क\u{941}\u{902}द\u{941}झ प\u{94d}रा\u{902}त"), ("ms", "Kunduz Province"), ("nb", "Kunduz"), ("ne", "क\u{941}न\u{94d}ड\u{941}ज"), ("nl", "Kunduz"), ("no", "Kunduz"), ("pl", "Kunduz"), ("ps", "کندوز ولايت"), ("pt", "Konduz"), ("ro", "Provincia Kunduz"), ("ru", "Кундуз"), ("si", "ක\u{dd4}න\u{dca}ඩ\u{dd4}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Провинција Кундуз"), ("sr_Latn", "Provincija Kunduz"), ("sv", "Kondoz"), ("ta", "குண\u{bcd}டுஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కుండుజ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e38}นด\u{e38}ซ"), ("tr", "Kunduz Vilayeti"), ("uk", "Кундуз"), ("ur", "صوبہ قندوز"), ("vi", "Tỉnh Kunduz"), ("zh", "昆都士省")]),
                        unofficial_name_list: ["Kondoz", "Kondūz", "Kūnduz", "Qondūz"].to_vec(),
                    }
                ),
                (
                    "KHO",
                    Subdivision{
                        name: "Khowst",
                        country_alpha2: Alpha2::AF,
                        code: "KHO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.3585079), longitude: Some(69.8597406), max_latitude: Some(33.7335318), min_latitude: Some(33.028327), max_longitude: Some(70.3265909), min_longitude: Some(69.3547779)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية خوست"), ("az", "Xust"), ("be", "Правінцыя Хост"), ("bg", "Хост"), ("bn", "খোস\u{9cd}ত প\u{9cd}রদেশ"), ("ca", "Província de Khost"), ("ccp", "𑄈\u{1112e}𑄈\u{11133}𑄑\u{11134}"), ("ceb", "Velāyat-e Khowst"), ("cs", "Chóst"), ("cy", "Khost"), ("da", "Khost"), ("de", "Chost"), ("el", "Χοστ"), ("en", "Khost"), ("es", "Provincia de Jost"), ("et", "Khowsti provints"), ("eu", "Khost probintzia"), ("fa", "ولایت خوست"), ("fi", "Khostin maakunta"), ("fr", "Khôst"), ("gu", "ખોસ\u{acd}ત"), ("hi", "ख\u{93c}ोस\u{94d}त प\u{94d}रान\u{94d}त"), ("id", "Provinsi Khost"), ("it", "provincia di Khowst"), ("ja", "ホースト州"), ("ka", "ხოსტის პროვინცია"), ("kn", "ಖೋಸ\u{ccd}ಟ\u{ccd}"), ("ko", "호스트 주"), ("lt", "Chosto provincija"), ("lv", "Houstas province"), ("mn", "Хост аймаг"), ("mr", "खोस\u{94d}ट"), ("ms", "Khost"), ("nb", "Khost"), ("nl", "Khost"), ("no", "Khost"), ("pa", "ਖ\u{a4b}ਸਤ"), ("pl", "Chost"), ("ps", "خوست ولايت"), ("pt", "Khost"), ("ro", "Provincia Khost"), ("ru", "Хост"), ("si", "කෝස\u{dca}ට\u{dca}"), ("sr", "Провинција Коуст"), ("sr_Latn", "Provincija Koust"), ("sv", "Khost"), ("ta", "கஹோஸ\u{bcd}ட"), ("te", "ఖ\u{c4b}స\u{c4d}ట\u{c4d}"), ("th", "โคลส"), ("tr", "Host Vilayeti"), ("uk", "Хост"), ("ur", "صوبہ خوست"), ("vi", "Khost"), ("zh", "霍斯特省")]),
                        unofficial_name_list: ["H\u{331}awst", "H\u{331}ōst", "Khawst", "Khost", "Matun", "Matūn"].to_vec(),
                    }
                ),
                (
                    "KNR",
                    Subdivision{
                        name: "Konar [Kunar]",
                        country_alpha2: Alpha2::AF,
                        code: "KNR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.0883133), longitude: Some(71.36685039999999), max_latitude: Some(35.0906452), min_latitude: Some(35.0861153), max_longitude: Some(71.3708674), min_longitude: Some(71.3623594)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كنر"), ("az", "Künər"), ("be", "Правінцыя Кунар"), ("bg", "Конар"), ("bn", "ক\u{9c1}ন\u{9be}র"), ("ca", "Província de Kunar"), ("ccp", "𑄇\u{1112a}𑄚𑄢\u{11134}"), ("ceb", "Wilāyat-e Kunaṟ"), ("cs", "Kúnar"), ("da", "Konar"), ("de", "Kunar"), ("el", "Κουνάρ"), ("en", "Kunar"), ("es", "Provincia de Kunar"), ("et", "Konarhā provints"), ("eu", "Kunar probintzia"), ("fa", "ولایت کنر"), ("fi", "Konarin maakunta"), ("fr", "Kounar"), ("gu", "ક\u{ac1}નાર"), ("hi", "क\u{941}नर प\u{94d}रान\u{94d}त"), ("id", "Provinsi Kunar"), ("it", "Konar"), ("ja", "クナル州"), ("ka", "ქუნარის პროვინცია"), ("kn", "ಕುನಾರ\u{ccd}"), ("ko", "쿠나르 주"), ("lt", "Kunaro provincija"), ("lv", "Kunāra"), ("mn", "Кунар аймаг"), ("mr", "क\u{941}नार"), ("ms", "Wilayah Kunar"), ("nb", "Konar"), ("nl", "Kunar"), ("no", "Konar"), ("pa", "ਕ\u{a41}ਨੜ ਸ\u{a42}ਬਾ"), ("pl", "Kunar"), ("ps", "کونړ ولايت"), ("pt", "Kunar"), ("ro", "Provincia Kunar"), ("ru", "Кунар"), ("si", "ක\u{dd4}න\u{dcf}ර\u{dca}"), ("sr", "Провинција Кунар"), ("sr_Latn", "Provincija Kunar"), ("sv", "Konar"), ("ta", "குனர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కున\u{c3e}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกนาร\u{e4c}"), ("tr", "Kunar Vilayeti"), ("uk", "Кунар"), ("ur", "صوبہ کنڑ"), ("vi", "Kunar"), ("zh", "库纳尔省")]),
                        unofficial_name_list: ["Konar", "Konarhā"].to_vec(),
                    }
                ),
                (
                    "LAG",
                    Subdivision{
                        name: "Laghman",
                        country_alpha2: Alpha2::AF,
                        code: "LAG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.6897687), longitude: Some(70.1455805), max_latitude: Some(35.2196171), min_latitude: Some(34.3999048), max_longitude: Some(70.63105999999999), min_longitude: Some(69.8045771)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية لغمان"), ("az", "Ləğman"), ("be", "Правінцыя Лагман"), ("bg", "Лагман"), ("bn", "ল\u{9be}গমন প\u{9cd}রদেশ"), ("ca", "Província de Laghman"), ("ccp", "𑄣𑄇\u{11134}𑄟\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Wilāyat-e Laghmān"), ("cs", "Laghmán"), ("da", "Laghman"), ("de", "Laghman"), ("el", "Λαγκμάν"), ("en", "Laghman"), ("es", "Provincia de Laġmān"), ("et", "Laghmāni provints"), ("eu", "Laghman probintzia"), ("fa", "ولایت لغمان"), ("fi", "Laghmanin maakunta"), ("fr", "Laghmân"), ("gu", "લઘમાન"), ("hi", "लग\u{93c}मान प\u{94d}रान\u{94d}त"), ("id", "Laghman"), ("it", "Laghman"), ("ja", "ラグマーン州"), ("ka", "ლაღმანის პროვინცია"), ("kn", "ಲಗ\u{ccd}ಮನ\u{ccd}"), ("ko", "라그만 주"), ("lt", "Lagmano provincija"), ("lv", "Lagmāna"), ("mn", "Лагман аймаг"), ("mr", "लगमन"), ("ms", "Wilayah Laghman"), ("nb", "Laghman"), ("nl", "Laghman"), ("no", "Laghman"), ("pa", "ਲਗਮਾਨ"), ("pl", "Laghman"), ("ps", "لغمان ولايت"), ("pt", "Laghman"), ("ro", "Provincia Laghman"), ("ru", "Лагман"), ("si", "ලග\u{dca}හ\u{dca}ම\u{dcf}න\u{dca}"), ("sr", "Провинција Лангмар"), ("sr_Latn", "Provincija Langmar"), ("sv", "Laghman"), ("ta", "ல\u{bbe}ஃஹ\u{bcd}ம\u{bbe}ன\u{bcd}"), ("te", "ల\u{c3e}గ\u{c4d}మన\u{c4d}"), ("th", "แลคห\u{e4c}แมน"), ("tr", "Lagman Vilayeti"), ("uk", "Лагман"), ("ur", "لاگھمان صوبہ"), ("vi", "Laghman"), ("zh", "拉格曼省")]),
                        unofficial_name_list: ["Laghman", "Laghmān", "Lagman"].to_vec(),
                    }
                ),
                (
                    "LOG",
                    Subdivision{
                        name: "لوګر ولايت",
                        country_alpha2: Alpha2::AF,
                        code: "LOG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية لوكر"), ("az", "Lövgər vilayəti"), ("be", "Правінцыя Лагар"), ("bg", "Логар"), ("bn", "লগ\u{9be}র"), ("ca", "Província de Lowgar"), ("ccp", "𑄣\u{11127}𑄉𑄢\u{11134}"), ("ceb", "Wilāyat-e Lōgar"), ("cs", "Lógar"), ("da", "Lowgar"), ("de", "Lugar"), ("el", "Λογκάρ"), ("en", "Logar"), ("es", "Provincia de Lawgar"), ("et", "Lowgari provints"), ("eu", "Logar probintzia"), ("fa", "ولایت لوگر"), ("fi", "Logarin maakunta"), ("fr", "Lôgar"), ("gu", "લોગર"), ("he", "לוגר"), ("hi", "लोगर प\u{94d}रान\u{94d}त"), ("id", "Logar"), ("it", "Lowgar"), ("ja", "ローガル州"), ("ka", "ლოგარის პროვინცია"), ("kn", "ಲೋಗರ\u{ccd}"), ("ko", "로가르 주"), ("lt", "Logaro provincija"), ("lv", "Lougara"), ("mn", "Логар аймаг"), ("mr", "लो\u{902}गर"), ("ms", "Wilayah Logar"), ("nb", "Lowgar"), ("nl", "Logar"), ("no", "Lowgar"), ("pl", "Logar"), ("ps", "لوګر ولايت"), ("pt", "Logar"), ("ro", "Provincia Logar"), ("ru", "Логар"), ("si", "ලොග\u{dcf}ර\u{dca}"), ("sr", "Провинција Логар"), ("sr_Latn", "Provincija Logar"), ("sv", "Lowgar"), ("ta", "லோகர\u{bcd}"), ("te", "ల\u{c4b}గర\u{c4d}"), ("th", "แคว\u{e49}นโลเรโต"), ("tr", "Lovgar Vilayeti"), ("uk", "Логар"), ("ur", "صوبہ لوگر"), ("uz", "Luvgar"), ("vi", "Logar"), ("zh", "洛加尔省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NAN",
                    Subdivision{
                        name: "Nangrahar [Nangarhar]",
                        country_alpha2: Alpha2::AF,
                        code: "NAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.1718313), longitude: Some(70.6216794), max_latitude: Some(34.809976), min_latitude: Some(33.9433401), max_longitude: Some(71.17125709999999), min_longitude: Some(69.482636)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ننكرهار"), ("az", "Nəngərhar"), ("be", "Правінцыя Нангархар"), ("bg", "Нангархар"), ("bn", "ন\u{9be}নগ\u{9be}রহর প\u{9cd}রদেশ"), ("ca", "Província de Nangarhar"), ("ccp", "𑄚\u{11127}\u{11101}𑄉\u{11127}𑄢\u{11134}𑄦𑄢\u{11134}"), ("ceb", "Nangarhār"), ("cs", "Nangarhár"), ("cy", "Nangarhar"), ("da", "Nangarhar"), ("de", "Nangarhar"), ("el", "Νανγκαράρ"), ("en", "Nangarhar"), ("es", "Provincia de Nangarhar"), ("et", "Nangarhāri provints"), ("eu", "Nangarhar probintzia"), ("fa", "ولایت ننگرهار"), ("fi", "Nangarharin maakunta"), ("fr", "Nangarhâr"), ("gu", "ન\u{a82}ગારહર"), ("hi", "न\u{902}गरहार प\u{94d}रान\u{94d}त"), ("hu", "Nangarhar"), ("id", "Provinsi Nangarhar"), ("it", "Nangarhar"), ("ja", "ナンガルハール州"), ("ka", "ნანგარჰარის პროვინცია"), ("kn", "ನಂಗಾರ\u{ccd}ಹರ\u{ccd}"), ("ko", "낭가르하르 주"), ("lt", "Nangarharo provincija"), ("lv", "Nangarhara"), ("mn", "Нангархар аймаг"), ("mr", "ना\u{902}जरघर"), ("ms", "Wilayah Nangarhar"), ("nb", "Nangarhar"), ("nl", "Nangarhar"), ("no", "Nangarhar"), ("pa", "ਨ\u{a70}ਗਰਹਾਰ ਸ\u{a42}ਬਾ"), ("pl", "Nangarhar"), ("ps", "د ننګرهار ولايت"), ("pt", "Nangarhar"), ("ro", "Provincia Nangarhar"), ("ru", "Нангархар"), ("si", "නන\u{dca}ගර\u{dca}හ\u{dcf}ර\u{dca}"), ("sr", "Провинција Нангархар"), ("sr_Latn", "Provincija Nangarhar"), ("sv", "Nangarhar"), ("ta", "ந\u{bbe}ங\u{bcd}கர\u{bcd}ஹர\u{bcd}"), ("te", "నంగర\u{c4d}హ\u{c3e}ర\u{c4d}"), ("th", "นานกาฮาร\u{e4c}"), ("tr", "Nangarhar Vilayeti"), ("uk", "Нангархар"), ("ur", "صوبہ ننگرہار"), ("vi", "Nangarhar"), ("zh", "楠格哈尔省")]),
                        unofficial_name_list: ["Nangarhar", "Ningarhar"].to_vec(),
                    }
                ),
                (
                    "NIM",
                    Subdivision{
                        name: "Nimruz",
                        country_alpha2: Alpha2::AF,
                        code: "NIM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.0261488), longitude: Some(62.4504154), max_latitude: Some(32.263132), min_latitude: Some(29.3889659), max_longitude: Some(63.5857501), min_longitude: Some(60.87859700000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نيمروز"), ("az", "Nimruz"), ("be", "Правінцыя Німроз"), ("bg", "Нимруз"), ("bn", "নিমর\u{9c1}জ প\u{9cd}রদেশ"), ("ca", "Província de Nimruz"), ("ccp", "𑄚\u{11128}𑄟\u{11134}𑄢𑄌\u{11134}"), ("ceb", "Wilāyat-e Nīmrōz"), ("cs", "Nímróz"), ("cy", "Nimruz"), ("da", "Nimruz"), ("de", "Nimrus"), ("el", "Νιμρούζ"), ("en", "Nimruz"), ("es", "Provincia de Nimruz"), ("et", "Nīmrūzi provints"), ("eu", "Nimruz probintzia"), ("fa", "ولایت نیمروز"), ("fi", "Nimruzin maakunta"), ("fr", "Nimrôz"), ("gu", "નિમ\u{acd}ર\u{ac1}ઝ"), ("hi", "नीमर\u{942}ज\u{93c} प\u{94d}रान\u{94d}त"), ("hy", "Նիմրուզ"), ("id", "Nimruz"), ("it", "Nimruz"), ("ja", "ニームルーズ州"), ("ka", "ნიმრუზის პროვინცია"), ("kn", "ನ\u{cbf}ಮ\u{ccd}ರುಜ\u{ccd}"), ("ko", "님루즈 주"), ("lt", "Nimrūzo provincija"), ("lv", "Nīmrūzas province"), ("mn", "Нимроз аймаг"), ("mr", "निमर\u{941}झ"), ("ms", "Nimruz"), ("nb", "Nimruz"), ("nl", "Nimruz"), ("no", "Nimruz"), ("pa", "ਨੀਮਰ\u{a4b}ਜ\u{a3c} ਸ\u{a42}ਬਾ"), ("pl", "Nimroz"), ("ps", "نيمروز ولايت"), ("pt", "Nimruz"), ("ro", "Provincia Nimruz"), ("ru", "Нимроз"), ("si", "න\u{dd2}ම\u{dca}ර\u{dd4}ස\u{dca}"), ("sr", "Провинција Нимруз"), ("sr_Latn", "Provincija Nimruz"), ("sv", "Nimruz"), ("ta", "நிம\u{bcd}ருஸ\u{bcd}"), ("te", "న\u{c3f}మ\u{c4d}రుజ\u{c4d}"), ("th", "น\u{e34}มร\u{e31}ซ"), ("tr", "Nimruz Vilayeti"), ("uk", "Німруз"), ("ur", "صوبہ نیمروز"), ("vi", "Nimruz"), ("zh", "尼姆鲁兹省")]),
                        unofficial_name_list: ["Chakhānsur", "Neemroze", "Nimroz", "Nimroze"].to_vec(),
                    }
                ),
                (
                    "NUR",
                    Subdivision{
                        name: "Nurestan",
                        country_alpha2: Alpha2::AF,
                        code: "NUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.3250223), longitude: Some(70.90712359999999), max_latitude: Some(36.0492169), min_latitude: Some(34.9075738), max_longitude: Some(71.614166), min_longitude: Some(69.916771)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نورستان"), ("az", "Nuristan"), ("be", "Правінцыя Нурыстан"), ("bg", "Нуристан"), ("bn", "ন\u{9c1}রেস\u{9cd}তন প\u{9cd}রদেশ"), ("ca", "Província de Nuristan"), ("ccp", "𑄚\u{1112a}𑄢\u{11128}𑄌\u{11134}𑄑𑄚\u{11134}"), ("ceb", "Velāyat-e Nūrestān"), ("cs", "Núristán"), ("da", "Nuristan"), ("de", "Nuristan"), ("el", "Νουρεστάν"), ("en", "Nuristan"), ("es", "Provincia de Nūristān"), ("et", "Nūrestāni provints"), ("eu", "Nuristan probintzia"), ("fa", "ولایت نورستان"), ("fi", "Nurestanin maakunta"), ("fr", "Nourestân"), ("gu", "ન\u{ac1}રિસ\u{acd}તાન"), ("hi", "न\u{942}रिस\u{94d}तान प\u{94d}रान\u{94d}त"), ("id", "Nurestan"), ("is", "Núristan"), ("it", "provincia di Nurestan"), ("ja", "ヌーリスターン州"), ("ka", "ნურისტანის პროვინცია"), ("kn", "ನುರ\u{cbf}ಸ\u{ccd}ತಾನ\u{ccd}"), ("ko", "누리스탄 주"), ("lt", "Nuristano provincija"), ("lv", "Nūrestāna"), ("ml", "ന\u{d42}റിസ\u{d4d}ഥ\u{d3e}ൻ"), ("mn", "Нуристан аймаг"), ("mr", "न\u{941}रिस\u{94d}तान"), ("ms", "Nurestan"), ("nb", "Nuristān"), ("nl", "Nooristan"), ("no", "Nuristān"), ("pa", "ਨ\u{a42}ਰਿਸਤਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Nuristan"), ("ps", "نورستان ولايت"), ("pt", "Nuristão"), ("ro", "Provincia Nuristan"), ("ru", "Нуристан"), ("si", "නර\u{dd2}ස\u{dca}ටන\u{dca}"), ("sq", "Nuristani"), ("sr", "Провинција Нурестан"), ("sr_Latn", "Provincija Nurestan"), ("sv", "Nurestan"), ("ta", "நூரிஸ\u{bcd}த\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "నూర\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e39}ร\u{e34}สถาน"), ("tr", "Nuristan Vilayeti"), ("uk", "Нуристан"), ("ur", "صوبہ نورستان"), ("uz", "Nuriston"), ("vi", "Nuristan"), ("zh", "努尔斯坦省")]),
                        unofficial_name_list: ["Nooristan", "Nouristan", "Nurestan"].to_vec(),
                    }
                ),
                (
                    "PAN",
                    Subdivision{
                        name: "Panjshir",
                        country_alpha2: Alpha2::AF,
                        code: "PAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.3350467), longitude: Some(69.7167783), max_latitude: Some(35.4374561), min_latitude: Some(35.1242394), max_longitude: Some(69.7982025), min_longitude: Some(69.53384400000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بانشير"), ("az", "Pəncşir"), ("be", "Правінцыя Панджшэр"), ("bg", "Панджшир"), ("bn", "প\u{9be}ঞ\u{9cd}জশির প\u{9cd}রদেশ"), ("ca", "Província de Panjshir"), ("ccp", "𑄛𑄐\u{11134}𑄥\u{11128}𑄢\u{11134}"), ("ceb", "Panjshir"), ("cs", "Pandžšír"), ("da", "Panjshir"), ("de", "Pandschschir"), ("el", "Παντζίρ"), ("en", "Panjshir"), ("es", "Provincia de Panjshīr"), ("et", "Panjshēri provints"), ("eu", "Panjshir probintzia"), ("fa", "ولایت پنجشیر"), ("fi", "Panjshirin maakunta"), ("fr", "Pandjchir"), ("hi", "प\u{902}जशीर प\u{94d}रान\u{94d}त"), ("it", "Panjshir"), ("ja", "パンジシール州"), ("ka", "ფანჯშირის პროვინცია"), ("ko", "판지시르 주"), ("lt", "Pandžširo provincija"), ("mn", "Панжшер аймаг"), ("ms", "Wilayah Panjshir"), ("nb", "Panjshir"), ("nl", "Panjshir"), ("no", "Panjshir"), ("pa", "ਪ\u{a70}ਜਸ\u{a3c}ਿਰ"), ("pl", "Pandższir"), ("ps", "پنجشېر ولايت"), ("pt", "Panjshir"), ("ro", "Provincia Panjshir"), ("ru", "Панджшер"), ("sr", "Провинција Пањшир"), ("sr_Latn", "Provincija Panjšir"), ("sv", "Panjshir"), ("ta", "ப\u{bbe}ஞ\u{bcd}ச\u{bcd}சிர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("tr", "Pençşir Vilayeti"), ("uk", "Панджшер"), ("ur", "پنج شیر"), ("zh", "潘傑希爾省")]),
                        unofficial_name_list: ["Panjshir"].to_vec(),
                    }
                ),
                (
                    "PAR",
                    Subdivision{
                        name: "Parwan",
                        country_alpha2: Alpha2::AF,
                        code: "PAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.9630977), longitude: Some(68.81088489999999), max_latitude: Some(35.424221), min_latitude: Some(34.588373), max_longitude: Some(69.616604), min_longitude: Some(68.1907899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بروان"), ("az", "Pərvan"), ("be", "Правінцыя Парван"), ("bg", "Парван"), ("bn", "প\u{9be}রভন প\u{9cd}রদেশ"), ("ca", "Província de Parwan"), ("ccp", "𑄛\u{11127}𑄢\u{11134}𑄃\u{1112e}𑄠𑄚\u{11134}"), ("ceb", "Parwān (lalawigan)"), ("cs", "Parván"), ("da", "Parvan"), ("de", "Parwan"), ("el", "Παρβάν"), ("en", "Parwan"), ("es", "Provincia de Parwān"), ("et", "Parvāni provints"), ("eu", "Parwan probintzia"), ("fa", "ولایت پروان"), ("fi", "Parvanin maakunta"), ("fr", "Parwân"), ("gu", "પારવાન પ\u{acd}રા\u{a82}ત"), ("hi", "परवान प\u{94d}रान\u{94d}त"), ("id", "Provinsi Parwan"), ("it", "Parvan"), ("ja", "パルヴァーン州"), ("ka", "ფარვანის პროვინცია"), ("kn", "ಪರ\u{ccd}ವಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파르반 주"), ("lt", "Parvano provincija"), ("lv", "Parvanas province"), ("mn", "Парван аймаг"), ("mr", "पारवान प\u{94d}रा\u{902}त"), ("ms", "Wilayah Parwan"), ("nb", "Parvan"), ("ne", "पर\u{94d}वान"), ("nl", "Parvan"), ("no", "Parvan"), ("pa", "ਪਰਵਾਨ"), ("pl", "Parwan"), ("ps", "پروان ولايت"), ("pt", "Parwan"), ("ro", "Provincia Parwan"), ("ru", "Парван"), ("si", "පර\u{dca}වන\u{dca} පළ\u{dcf}ත"), ("sr", "Провинција Парван"), ("sr_Latn", "Provincija Parvan"), ("sv", "Parvan"), ("ta", "பர\u{bcd}வ\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}ర\u{c4d}వ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาร\u{e4c}วาน"), ("tr", "Parvan Vilayeti"), ("uk", "Парван"), ("ur", "صوبہ پروان"), ("vi", "Tỉnh Parwan"), ("zh", "帕尔旺省")]),
                        unofficial_name_list: ["Parvan", "Parvān", "Parwan"].to_vec(),
                    }
                ),
                (
                    "PIA",
                    Subdivision{
                        name: "Paktia",
                        country_alpha2: Alpha2::AF,
                        code: "PIA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.706199), longitude: Some(69.3831079), max_latitude: Some(34.0908599), min_latitude: Some(33.1592119), max_longitude: Some(70.0056379), min_longitude: Some(68.772654)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بكتيا"), ("az", "Pəktiya"), ("be", "Правінцыя Пактыя"), ("bg", "Пактия"), ("bn", "প\u{9be}ক\u{9cd}\u{200c}তিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Paktia"), ("ccp", "𑄛𑄇\u{11134}𑄖\u{11128}𑄠"), ("ceb", "Wilāyat-e Paktiyā"), ("cs", "Paktíja"), ("da", "Paktia"), ("de", "Paktia"), ("el", "Πακτιά"), ("en", "Paktia"), ("es", "Provincia de Paktiyā"), ("et", "Paktīā provints"), ("eu", "Paktia probintzia"), ("fa", "ولایت پکتیا"), ("fi", "Paktian maakunta"), ("fr", "Paktiyâ"), ("gu", "પક\u{acd}ટિયા"), ("hi", "पकतिया प\u{94d}रान\u{94d}त"), ("id", "Paktia"), ("it", "Paktia"), ("ja", "パクティヤー州"), ("ka", "ფაქთიის პროვინცია"), ("kn", "ಪಖ\u{cbf}ಯಾ"), ("ko", "팍티야 주"), ("lt", "Paktijos provincija"), ("lv", "Paktija"), ("mn", "Пактия аймаг"), ("mr", "पक\u{94d}तिया"), ("ms", "Paktia"), ("nb", "Paktia"), ("nl", "Paktia"), ("no", "Paktia"), ("pa", "ਪਕਤੀਆ"), ("pl", "Paktija"), ("ps", "پکتيا ولايت"), ("pt", "Paktia"), ("ro", "Provincia Paktia"), ("ru", "Пактия"), ("si", "පක\u{dd2}ට\u{dcf}"), ("sr", "Провинција Пактија"), ("sr_Latn", "Provincija Paktija"), ("sv", "Paktia"), ("ta", "பக\u{bcd}திய\u{bbe}"), ("te", "ప\u{c3e}క\u{c4d}ట\u{c3f}య\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e31}กเต\u{e35}ย"), ("tr", "Paktiya Vilayeti"), ("uk", "Пактія"), ("ur", "پکتیا"), ("vi", "Paktia"), ("zh", "帕克蒂亚省")]),
                        unofficial_name_list: ["Paktia", "Paktiya", "Paktīā"].to_vec(),
                    }
                ),
                (
                    "PKA",
                    Subdivision{
                        name: "Paktika",
                        country_alpha2: Alpha2::AF,
                        code: "PKA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.2645386), longitude: Some(68.52471489999999), max_latitude: Some(33.4239659), min_latitude: Some(31.593519), max_longitude: Some(69.5345761), min_longitude: Some(67.82210479999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بكتيكا"), ("az", "Pəktika"), ("be", "Правінцыя Пактыка"), ("bg", "Пактика"), ("bn", "প\u{9be}ক\u{9cd}তিক\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Paktika"), ("ccp", "𑄛𑄇\u{11134}𑄖\u{11128}𑄇"), ("ceb", "Wilāyat-e Paktīkā"), ("cs", "Paktíka"), ("da", "Paktika"), ("de", "Paktika"), ("el", "Πακτίκα"), ("en", "Paktika"), ("es", "Provincia de Paktīkā"), ("et", "Paktīkā provints"), ("eu", "Paktika probintzia"), ("fa", "ولایت پکتیکا"), ("fi", "Paktikan maakunta"), ("fr", "Paktîkâ"), ("gu", "પક\u{acd}ટીકા"), ("hi", "पकतीका प\u{94d}रान\u{94d}त"), ("id", "Paktika"), ("it", "Paktika"), ("ja", "パクティーカー州"), ("ka", "ფაქთიკის პროვინცია"), ("kn", "ಪಕ\u{ccd}ತ\u{cbf}ಕ"), ("ko", "팍티카 주"), ("lt", "Paktikos provincija"), ("lv", "Paktika"), ("mn", "Пактика аймаг"), ("mr", "पतिकिका"), ("ms", "Paktika"), ("nb", "Paktika"), ("nl", "Paktika"), ("no", "Paktika"), ("pa", "ਪਕਤੀਕਾ"), ("pl", "Paktika"), ("ps", "پکتيکا ولايت"), ("pt", "Paktika"), ("ro", "Provincia Paktika"), ("ru", "Пактика"), ("si", "පක\u{dd2}ට\u{dd2}ක\u{dcf}"), ("sr", "Провинција Пактика"), ("sr_Latn", "Provincija Paktika"), ("sv", "Paktika"), ("ta", "பக\u{bcd}திக\u{bbe}"), ("te", "ప\u{c3e}క\u{c4d}ట\u{c3f}క\u{c3e}"), ("th", "ปาค\u{e34}ทคา"), ("tr", "Paktika Vilayeti"), ("uk", "Пактика"), ("ur", "صوبہ پکتیکا"), ("vi", "Paktika"), ("zh", "帕克蒂卡省")]),
                        unofficial_name_list: ["Paktika"].to_vec(),
                    }
                ),
                (
                    "SAM",
                    Subdivision{
                        name: "Samangan",
                        country_alpha2: Alpha2::AF,
                        code: "SAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.9807296), longitude: Some(67.57085359999999), max_latitude: Some(36.638235), min_latitude: Some(35.34659389999999), max_longitude: Some(68.5661071), min_longitude: Some(66.88432)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سمنكان"), ("az", "Səməngan"), ("be", "Правінцыя Саманган"), ("bg", "Саманган"), ("bn", "স\u{9be}ম\u{9be}ঙ\u{9cd}গন প\u{9cd}রদেশ"), ("ca", "Província de Samangan"), ("ccp", "𑄥𑄟\u{11127}𑄚\u{11134}𑄉\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Samangān"), ("cs", "Samangán"), ("da", "Samangan"), ("de", "Samangan"), ("el", "Σαμανγκάν"), ("en", "Samangan"), ("es", "Provincia de Samangān"), ("et", "Samangāni provints"), ("eu", "Samangan probintzia"), ("fa", "ولایت سمنگان"), ("fi", "Samanganin maakunta"), ("fr", "Samangân"), ("gu", "સમા\u{a82}ગણ"), ("hi", "सम\u{902}गान प\u{94d}रान\u{94d}त"), ("id", "Samangan"), ("it", "Samangan"), ("ja", "サマンガーン州"), ("ka", "სამანგანის პროვინცია"), ("kn", "ಸಮಂಗನ\u{ccd}"), ("ko", "사망간 주"), ("lt", "Samangano provincija"), ("lv", "Samangānas province"), ("mn", "Саманган аймаг"), ("mr", "समा\u{902}गण"), ("ms", "Wilayah Samangan"), ("nb", "Samangan"), ("ne", "समन\u{94d}गन"), ("nl", "Samangan"), ("no", "Samangan"), ("pa", "ਸਮ\u{a70}ਗਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Samangan"), ("ps", "سمنګان ولايت"), ("pt", "Samangan"), ("ro", "Provincia Samangan"), ("ru", "Саманган"), ("si", "සමන\u{dca}ගන\u{dca}"), ("sr", "Провинција Саманган"), ("sr_Latn", "Provincija Samangan"), ("sv", "Samangan"), ("ta", "சமங\u{bcd}கன\u{bcd}"), ("te", "సమంగ\u{c3e}న\u{c4d}"), ("th", "สแมนแกน"), ("tr", "Samangan Vilayeti"), ("uk", "Саманган"), ("ur", "صوبہ سمنگان"), ("vi", "Samangan"), ("zh", "萨曼甘省")]),
                        unofficial_name_list: ["Samangan"].to_vec(),
                    }
                ),
                (
                    "SAR",
                    Subdivision{
                        name: "Sar-e Pol",
                        country_alpha2: Alpha2::AF,
                        code: "SAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.22138899999999), longitude: Some(65.927778), max_latitude: Some(36.23838900000001), min_latitude: Some(36.1827099), max_longitude: Some(65.9589957), min_longitude: Some(65.9110165)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سربل"), ("az", "Səri-Pul"), ("be", "Правінцыя Сары-Пуль"), ("bg", "Сарипол"), ("bn", "স\u{9be}রে পোল প\u{9cd}রদেশ"), ("ca", "Província de Sar-e Pol"), ("ccp", "𑄥𑄢\u{11134}-𑄃\u{11128} 𑄛\u{11127}𑄣\u{11134}"), ("ceb", "Sar-e Pul (lalawigan)"), ("cs", "Sar-e Pol"), ("da", "Sar-e Pol"), ("de", "Sar-i Pul"), ("el", "Σαρ-έ Πολ"), ("en", "Sar-e Pol"), ("es", "Provincia de Sar-e Pul"), ("et", "Sar-e Poli provints"), ("eu", "Sar-e Pol probintzia"), ("fa", "ولایت سرپل"), ("fi", "Sar-e Polin maakunta"), ("fr", "Sar-é Pol"), ("gu", "સર-એ પોલ"), ("hi", "सर-ए-पोल प\u{94d}रान\u{94d}त"), ("id", "Provinsi Sar-e Pol"), ("it", "Provincia di Sar-e Pol"), ("ja", "サーレポル州"), ("ka", "სარიპულის პროვინცია"), ("kn", "ಸರ\u{ccd}-ಇ ಪೋಲ\u{ccd}"), ("ko", "사르이폴 주"), ("lt", "Sari Pulio provincija"), ("lv", "Sarepola"), ("mn", "Сари-Пуль аймаг"), ("mr", "सर-ए पोल"), ("ms", "Wilayah Sar-e Pol"), ("nb", "Sar-e Pol"), ("nl", "Sar-e Pol"), ("no", "Sar-e Pol"), ("pa", "ਸਰ\u{a47} ਪ\u{a4b}ਲ"), ("pl", "Sar-e Pol"), ("ps", "سرپل ولايت"), ("pt", "Sar-e Pol"), ("ro", "Provincia Sar-e Pol"), ("ru", "Сари-Пуль"), ("si", "සරේ පොල\u{dca}"), ("sr", "Провинција Сарипол"), ("sr_Latn", "Provincija Saripol"), ("sv", "Sar-e Pol"), ("ta", "ச\u{bbe}ர\u{bcd} -இ போல\u{bcd}"), ("te", "సర\u{c4d}-ఎ ప\u{c4b}ల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาร\u{e4c}เอโปล"), ("tr", "Sar-i Pol Vilayeti"), ("uk", "Сарі-Пуль"), ("ur", "صوبہ سرپل"), ("vi", "Sar-e Pol"), ("zh", "薩爾普勒省")]),
                        unofficial_name_list: ["Sar-e Pol", "Sar-i Pol", "Sari Pol"].to_vec(),
                    }
                ),
                (
                    "TAK",
                    Subdivision{
                        name: "Takhar",
                        country_alpha2: Alpha2::AF,
                        code: "TAK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.6698013), longitude: Some(69.4784541), max_latitude: Some(37.62126), min_latitude: Some(35.785804), max_longitude: Some(70.4998339), min_longitude: Some(69.1752951)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تخار"), ("az", "Təxar"), ("be", "Правінцыя Тахар"), ("bg", "Тахар"), ("bn", "ত\u{9be}খর প\u{9cd}রদেশ"), ("ca", "Província de Takhar"), ("ccp", "𑄖\u{11127}𑄈\u{11127}𑄢\u{11134}"), ("ceb", "Wilāyat-e Takhār"), ("cs", "Tachár"), ("da", "Takhar"), ("de", "Tachar"), ("el", "Ταχάρ"), ("en", "Takhar"), ("es", "Provincia de Tahār"), ("et", "Takhāri provints"), ("eu", "Takhar probintzia"), ("fa", "ولایت تخار"), ("fi", "Takharin maakunta"), ("fr", "Takhâr"), ("gu", "તખાર"), ("hi", "तख\u{93c}ार प\u{94d}रान\u{94d}त"), ("hy", "Տախար"), ("id", "Takhar"), ("it", "Takhar"), ("ja", "タハール州"), ("ka", "თახარის პროვინცია"), ("kn", "ತಖರ\u{ccd}"), ("ko", "타하르 주"), ("lt", "Tacharo provincija"), ("lv", "Tahāra"), ("mn", "Тахар аймаг"), ("mr", "तखार"), ("ms", "Wilayah Takhar"), ("nb", "Takhar"), ("nl", "Tachar"), ("no", "Takhar"), ("pa", "ਤਖਾਰ ਸ\u{a42}ਬਾ"), ("pl", "Tachar"), ("ps", "تخار ولايت"), ("pt", "Takhar"), ("ro", "Provincia Takhar"), ("ru", "Тахар"), ("si", "ටක\u{dca}හ\u{dcf}ර\u{dca}"), ("sr", "Провинција Такар"), ("sr_Latn", "Provincija Takar"), ("sv", "Takhar"), ("ta", "டக\u{bbe}ர\u{bcd}"), ("te", "టఖర\u{c4d}"), ("th", "ท\u{e31}คฮาร\u{e4c}"), ("tr", "Tahar Vilayeti"), ("uk", "Тахар"), ("ur", "صوبہ تخار"), ("vi", "Takhar"), ("zh", "塔哈尔省")]),
                        unofficial_name_list: ["Tahar", "Takhar", "Takhār"].to_vec(),
                    }
                ),
                (
                    "URU",
                    Subdivision{
                        name: "روزګان ولايت",
                        country_alpha2: Alpha2::AF,
                        code: "URU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية أروزكان"), ("az", "Uruzgan"), ("be", "Правінцыя Урузган"), ("bg", "Орузган"), ("bn", "ওর\u{9c1}জ\u{9cd}\u{200c}গন প\u{9cd}রদেশ"), ("ca", "Província d’Oruzgan"), ("ccp", "𑄅\u{1112a}𑄢\u{11127}𑄌\u{11134}𑄉\u{11127}𑄚\u{11134}"), ("ceb", "Uruzgān"), ("cs", "Orúzgán"), ("da", "Oruzgan"), ("de", "Urusgan"), ("el", "Ορουζγκάν"), ("en", "Urozgan"), ("es", "Provincia de Urūzgān"), ("et", "Uruzgāni provints"), ("eu", "Oruzgan probintzia"), ("fa", "ولایت اروزگان"), ("fi", "Uruzganin maakunta"), ("fr", "Orozgân"), ("gu", "ઉરોઝગન"), ("hi", "ओर\u{942}ज\u{93c}\u{94d}गान प\u{94d}रान\u{94d}त"), ("id", "Provinsi Oruzgan"), ("it", "Oruzgan"), ("ja", "ウルーズガーン州"), ("ka", "ურუზგანის პროვინცია"), ("kn", "ಉರೊಜ\u{ccd}ಗನ\u{ccd}"), ("ko", "우루즈간 주"), ("lt", "Urozgano provincija"), ("lv", "Orūzgānas province"), ("ml", "ഉറോസ\u{d4d}ഗ\u{d3e}ൻ പ\u{d4d}രവിശ\u{d4d}യ"), ("mn", "Урузган аймаг"), ("mr", "उरोजगण"), ("ms", "Urozgan"), ("nb", "Uruzgan"), ("nl", "Uruzgan"), ("no", "Uruzgan"), ("pa", "ਓਰ\u{a41}ਜਗਾਨ ਸ\u{a42}ਬਾ"), ("pl", "Oruzgan"), ("ps", "روزګان ولايت"), ("pt", "Oruzgan"), ("ro", "Provincia Urozgan"), ("ru", "Урузган"), ("si", "උරෝස\u{dca}ගන\u{dca}"), ("sr", "Провинција Орузган"), ("sr_Latn", "Provincija Oruzgan"), ("sv", "Oruzgan"), ("ta", "உரோஸ\u{bcd}க\u{bcd}க\u{bbe}ன\u{bcd}"), ("te", "యూర\u{c4b}జ\u{c4d}గ\u{c3e}న\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e39}รานก\u{e31}น"), ("tr", "Uruzgan Vilayeti"), ("uk", "Урузган"), ("ur", "اروزگان"), ("vi", "Oruzgan"), ("zh", "乌鲁兹甘省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WAR",
                    Subdivision{
                        name: "Wardak [Wardag]",
                        country_alpha2: Alpha2::AF,
                        code: "WAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.3513494), longitude: Some(68.2385339), max_latitude: Some(34.797575), min_latitude: Some(33.683571), max_longitude: Some(68.972619), min_longitude: Some(67.232205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية وردك"), ("az", "Vərdək"), ("be", "Правінцыя Вардак"), ("bg", "Вардак"), ("bn", "ভ\u{9be}র\u{9cd}দ\u{9be}ক প\u{9cd}রদেশ"), ("ca", "Província de Wardak"), ("ccp", "𑄟\u{11133}𑄆𑄓\u{11127}𑄚\u{11134} 𑄤𑄢\u{11134}𑄓\u{11127}𑄇\u{11134}"), ("ceb", "Wardak (lalawigan)"), ("cs", "Vardak"), ("da", "Vardak"), ("de", "Wardak"), ("el", "Βαρντάκ"), ("en", "Maidan Wardak"), ("es", "Provincia de Vardak"), ("et", "Vardaki provints"), ("eu", "Wardak probintzia"), ("fa", "ولایت وردک"), ("fi", "Vardakin maakunta"), ("fr", "Wardak"), ("gu", "મ\u{ac8}દાન વાર\u{acd}ડક"), ("hi", "म\u{948}दान वरदक प\u{94d}रान\u{94d}त"), ("id", "Maidan Wardak"), ("it", "Vardak"), ("ja", "ヴァルダク州"), ("ka", "მაიდან-ვარდაკის პროვინცია"), ("kn", "ಮೈದಾನ\u{ccd} ವಾರ\u{ccd}ಡಾಕ\u{ccd}"), ("ko", "바르다크 주"), ("lt", "Vardako provincija"), ("lv", "Vardaka"), ("mn", "Вардаг аймаг"), ("mr", "म\u{948}दान वार\u{94d}डक"), ("ms", "Wilayah Wardak"), ("nb", "Vardak"), ("nl", "Wardak"), ("no", "Vardak"), ("pa", "ਵਾਰਦਾਕ"), ("pl", "Wardak"), ("ps", "ميدان وردګ ولايت"), ("pt", "Maydan-Wardak"), ("ro", "Provincia Maidan Wardak"), ("ru", "Вардак"), ("si", "ම\u{dcf}ද\u{dd2}යන\u{dca} වර\u{dca}ඩක\u{dca}"), ("sr", "Провинција Вардак"), ("sr_Latn", "Provincija Vardak"), ("sv", "Wardak"), ("ta", "மெய\u{bcd}டன\u{bcd} வ\u{bbe}ர\u{bcd}ட\u{bbe}க\u{bcd}"), ("te", "మ\u{c48}డన\u{c4d} వ\u{c3e}ర\u{c4d}డ\u{c3e}క\u{c4d}"), ("th", "เมเดนวอร\u{e4c}ดาก"), ("tr", "Vardak Vilayeti"), ("uk", "Вардак"), ("ur", "وردک"), ("vi", "Wardak"), ("zh", "瓦尔达克省")]),
                        unofficial_name_list: ["Vardak", "Wardagh", "Wardak"].to_vec(),
                    }
                ),
                (
                    "ZAB",
                    Subdivision{
                        name: "Zabol [Zabul]",
                        country_alpha2: Alpha2::AF,
                        code: "ZAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.1918782), longitude: Some(67.1894488), max_latitude: Some(33.083939), min_latitude: Some(31.505068), max_longitude: Some(68.1166841), min_longitude: Some(66.192899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية زابل"), ("az", "Zabul vilayəti"), ("be", "Правінцыя Забуль"), ("bg", "Забул"), ("bn", "জবোল প\u{9cd}রদেশ"), ("ca", "Província de Zabul"), ("ccp", "𑄎𑄝\u{1112a}𑄣\u{11134}"), ("ceb", "Wilāyat-e Zābul"), ("cs", "Zábul"), ("da", "Zabol (provins)"), ("de", "Zabul"), ("el", "Ζάμπουλ (περιοχή)"), ("en", "Zabul"), ("es", "Provincia de Zābul"), ("et", "Zābuli provints"), ("eu", "Zabul probintzia"), ("fa", "ولایت زابل"), ("fi", "Zabolin maakunta"), ("fr", "Zabol (Afghanistan)"), ("gu", "ઝબ\u{ac1}લ"), ("hi", "ज\u{93c}ाब\u{941}ल प\u{94d}रान\u{94d}त"), ("id", "Zabul"), ("it", "Zabol"), ("ja", "ザーブル州"), ("ka", "ზაბულის პროვინცია"), ("kn", "ಝಬುಲ\u{ccd}"), ("ko", "자불 주"), ("lt", "Zabulio provincija"), ("lv", "Zābola"), ("mn", "Забуль аймаг"), ("mr", "झब\u{941}ल"), ("ms", "Wilayah Zabul"), ("nb", "Zabol"), ("nl", "Zabul"), ("no", "Zabol"), ("pa", "ਜ\u{a3c}ਾਬ\u{a41}ਲ ਸ\u{a42}ਬਾ"), ("pl", "Zabol (prowincja)"), ("ps", "زابل ولايت"), ("pt", "Zabol"), ("ro", "Provincia Zabul"), ("ru", "Забуль"), ("si", "සබ\u{dd4}ල\u{dca}"), ("sr", "Провинција Забул"), ("sr_Latn", "Provincija Zabul"), ("sv", "Zabol (provins)"), ("ta", "சபிள\u{bcd}"), ("te", "జ\u{c3e}బుల\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาบอล"), ("tr", "Zabul Vilayeti"), ("uk", "Забуль"), ("ur", "زابل پراونس"), ("vi", "Zabul (tỉnh)"), ("zh", "扎布尔省")]),
                        unofficial_name_list: ["Zabol", "Zabul", "Zābol"].to_vec(),
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
#[cfg(feature = "af")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AF,
        alpha3: Alpha3::AFG,
        address_format: None,
        continent: Continent::Asia,
        country_code: 93,
        currency_code: CurrencyCode::AFN,
        gec: Some(GEC::AF),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::AFG),
        iso_long_name: "The Islamic Republic of Afghanistan",
        iso_short_name: "Afghanistan",
        official_language_list: ["ps", "tk", "uz"].to_vec(),
        spoken_language_list: ["ps", "tk", "uz"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Afghan"),
        number: "004",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "AF",
        unofficial_name_list: ["Afghanistan", "Afganistán", "アフガニスタン"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Afghanistan"),
            ("af", "Afghanistan"),
            ("ak", "Afghanistan"),
            ("am", "አፍጋኒስታን"),
            ("an", "Afganistán"),
            ("ar", "أفغانستان"),
            ("as", "আফগ\u{9be}নিস\u{9cd}থ\u{9be}ন"),
            ("ay", "Afghanistan"),
            ("az", "Əfqanıstan"),
            ("ba", "Afghanistan"),
            ("be", "Афганістан"),
            ("bg", "Афганистан"),
            ("bi", "Afghanistan"),
            ("bn", "আফগ\u{9be}নিস\u{9cd}ত\u{9be}ন"),
            ("bn_IN", "আফগ\u{9be}নিস\u{9cd}ত\u{9be}ন"),
            ("br", "Afghanistan"),
            ("bs", "Afganistan"),
            ("ca", "Afganistan"),
            ("ce", "АфгIанистан"),
            ("ch", "Afghanistan"),
            ("cs", "Afghánistán"),
            ("cv", "Афганистан"),
            ("cy", "Afghanistan"),
            ("da", "Afghanistan"),
            ("de", "Afghanistan"),
            (
                "dv",
                "އ\u{7a6}ފ\u{7b0}ޣ\u{7a7}ނ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}",
            ),
            ("dz", "ཨཕ་ག་ན\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
            ("ee", "Afghanistan"),
            ("el", "Αφγανιστάν"),
            ("en", "Afghanistan"),
            ("eo", "Afganio"),
            ("es", "Afganistán"),
            ("et", "Afganistan"),
            ("eu", "Afganistan"),
            ("fa", "افغانستان"),
            ("ff", "Afghanistan"),
            ("fi", "Afghanistan"),
            ("fo", "Afganistan"),
            ("fr", "Afghanistan"),
            ("fy", "Afganistan"),
            ("ga", "An Afganastáin"),
            ("gl", "Afganistán"),
            ("gn", "Afghanistan"),
            ("gu", "અફઘાનિસ\u{acd}તાન"),
            ("gv", "Yn Afghanistaan"),
            ("ha", "Afghanistan"),
            ("he", "אפגניסטן"),
            ("hi", "अफ\u{93c}\u{94d}गानिस\u{94d}तान"),
            ("hr", "Afganistan"),
            ("ht", "Afganistan"),
            ("hu", "Afganisztán"),
            ("hy", "Աֆղանստան"),
            ("ia", "Afghanistan"),
            ("id", "Afganistan"),
            ("io", "Afganistan"),
            ("is", "Afganistan"),
            ("it", "Afghanistan"),
            ("iu", "Afghanistan"),
            ("ja", "アフガニスタン"),
            ("ka", "ავღანეთი"),
            ("ki", "Afghanistan"),
            ("kk", "Ауғанстан"),
            ("kl", "Afghanistan"),
            ("km", "អាហ\u{17d2}គាន\u{17b8}ស\u{17d2}ថាន"),
            ("kn", "ಅಫ\u{ccd}ಗಾನ\u{cbf}ಸ\u{ccd}ಥಾನ\u{ccd}"),
            ("ko", "아프가니스탄"),
            ("ku", "Efxanistan"),
            ("kv", "Афганистан"),
            ("kw", "Afghanistan"),
            ("ky", "Ооганстан"),
            ("lo", "Afghanistan"),
            ("lt", "Afganistanas"),
            ("lv", "Afganistāna"),
            ("mi", "Āwhekenetāna"),
            ("mk", "Авганистан"),
            ("ml", "അഫ\u{d4d}ഗ\u{d3e}നിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"),
            ("mn", "Афганстан"),
            ("mr", "अफगाणिस\u{94d}तान"),
            ("ms", "Afghanistan"),
            ("mt", "Afganistan"),
            (
                "my",
                "အာဖဂန\u{103a}နစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Apeganitan"),
            ("nb", "Afghanistan"),
            ("ne", "अफगानिस\u{94d}तान"),
            ("nl", "Afghanistan"),
            ("nn", "Afghanistan"),
            ("nv", "Afghanistan"),
            ("oc", "Afganistan"),
            ("or", "ଆଫଗ\u{b3e}ନ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
            ("pa", "ਅਫਗਾਨਿਸਤਾਨ"),
            ("pi", "अफगानस\u{94d}थान"),
            ("pl", "Afganistan"),
            ("ps", "افغانستان"),
            ("pt", "Afeganistão"),
            ("pt_BR", "Afeganistão"),
            ("ro", "Afganistan"),
            ("ru", "Афганистан"),
            ("rw", "Afuganisita"),
            ("sc", "Afghànistan"),
            ("sd", "افغانستان"),
            ("si", "ඇෆ\u{dca}ගන\u{dd2}ස\u{dca}ත\u{dcf}නය"),
            ("sk", "Afganistan"),
            ("sl", "Afganistan"),
            ("so", "Afgaanistaan"),
            ("sq", "Afganistan"),
            ("sr", "Авганистан"),
            ("sv", "Afghanistan"),
            ("sw", "Afghanistani"),
            ("ta", "ஆப\u{bcd}க\u{bbe}னிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
            ("te", "ఆఫ\u{c4d}ఘన\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
            ("tg", "Афғонистон"),
            ("th", "อ\u{e31}ฟกาน\u{e34}สถาน"),
            ("ti", "ኣፍጋኒስታን"),
            ("tk", "Owganystan"),
            ("tl", "Apganistan"),
            ("tr", "Afganistan"),
            ("tt", "Әфгәнстан"),
            ("ug", "ئافغانىستان"),
            ("uk", "Афганістан"),
            ("ur", "افغانستان"),
            ("uz", "Afgʻoniston"),
            ("ve", "Afghanistan"),
            ("vi", "A Phú Hãn"),
            ("wa", "Afganistan"),
            ("wo", "Afganistaan"),
            ("xh", "Afghanistan"),
            ("yo", "Afghanístàn"),
            ("zh_CN", "阿富汗"),
            ("zh_HK", "阿富汗"),
            ("zh_TW", "阿富汗"),
            ("zu", "I-Afganistani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
