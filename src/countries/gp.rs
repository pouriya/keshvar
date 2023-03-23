// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Guadeloupe

#[cfg(all(feature = "gp", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GP;
    pub const ALPHA3: Alpha3 = Alpha3::GLP;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 590;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::GP);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = None;
    pub const ISO_SHORT_NAME: &str = "Guadeloupe";
    pub const ISO_LONG_NAME: &str = "Guadeloupe";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Guadeloupian");
    pub const NUMBER: &str = "312";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("9[78][01]\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "GP";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Guadeloupe", "Guadalupe", "グアドループ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Guadeloupe"),
        ("af", "Guadeloupe"),
        ("ak", "Guadeloupe"),
        ("am", "Guadeloupe"),
        ("an", "Guadeloupe"),
        ("ar", "جوادالوب\u{651}ي"),
        ("as", "গ\u{9c1}ৱ\u{9be}ডেলোপ"),
        ("ay", "Guadeloupe"),
        ("az", "Quadelup"),
        ("ba", "Guadeloupe"),
        ("be", "Гвадэлупа"),
        ("bg", "Гваделупа"),
        ("bi", "Guadeloupe"),
        ("bn", "গ\u{9c1}য়\u{9be}ডেল\u{9cd}য\u{9c1}প"),
        ("bn_IN", "গ\u{9c1}য়\u{9be}ডেল\u{9cd}য\u{9c1}প"),
        ("br", "Gwadaloup"),
        ("bs", "Gvadalupe"),
        ("ca", "Guadalupe"),
        ("ce", "Гваделупа"),
        ("ch", "Guadeloupe"),
        ("cs", "Guadeloupe"),
        ("cv", "Гваделупа"),
        ("cy", "Guadeloupe"),
        ("da", "Guadeloupe"),
        ("de", "Guadeloupe"),
        ("dv", "ގ\u{7af}ޑ\u{7a8}ލ\u{7ab}ޕ\u{7aa}"),
        ("dz", "ག\u{f74}་འ་ད\u{f72}་ལ\u{f7c}པ།"),
        ("ee", "Guadeloupe"),
        ("el", "Γουαδελούπη"),
        ("en", "Guadeloupe"),
        ("eo", "Gvadelupo"),
        ("es", "Guadalupe"),
        ("et", "Guadeloupe"),
        ("eu", "Guadalupe"),
        ("fa", "گوادلوپ"),
        ("ff", "Guadeloupe"),
        ("fi", "Guadeloupe"),
        ("fo", "Guadeloupe"),
        ("fr", "Guadeloupe"),
        ("fy", "Guadeloupe"),
        ("ga", "Guadalúip"),
        ("gl", "Guadalupe"),
        ("gn", "Guadeloupe"),
        ("gu", "ગ\u{acd}ડ\u{ac7}લોપ\u{ac7}"),
        ("gv", "Guadeloupe"),
        ("ha", "Guadeloupe"),
        ("he", "גוואדלופ"),
        ("hi", "ग\u{941}आद\u{947}ल\u{942}प"),
        ("hr", "Gvadalupa"),
        ("ht", "Gwadloup"),
        ("hu", "Guadeloupe"),
        ("hy", "Գվադելուպա"),
        ("ia", "Guadeloupe"),
        ("id", "Guadeloupe"),
        ("io", "Guadelupa"),
        ("is", "Gvadalúp"),
        ("it", "Guadalupa"),
        ("iu", "Guadeloupe"),
        ("ja", "グアドループ"),
        ("ka", "გვადალუპე"),
        ("ki", "Guadeloupe"),
        ("kk", "Гваделупа"),
        ("kl", "Guadeloupe"),
        ("km", "ហ\u{17d2}គ\u{17bc}អាដ\u{17ba}ល\u{17bc}ប\u{17c9}េ"),
        ("kn", "Guadeloupe"),
        ("ko", "과들루프"),
        ("ku", "Guadelûp"),
        ("kv", "Guadeloupe"),
        ("kw", "Gwadeloup"),
        ("ky", "Гваделупа"),
        ("lo", "Guadeloupe"),
        ("lt", "Gvadelupa"),
        ("lv", "Gvadelupa"),
        ("mi", "Guadeloupe"),
        ("mk", "Гуадалопе"),
        ("ml", "ഗ\u{d4d}വ\u{d3e}ഡെലോപ\u{d4d}"),
        ("mn", "Guadeloupe"),
        ("mr", "ग\u{94d}वाड\u{947}लोप"),
        ("ms", "Guadeloupe"),
        ("mt", "Gwadelupe"),
        ("my", "ဂ\u{103d}ါဒလ\u{103a}ကျ\u{103d}န\u{103a}း"),
        ("na", "Guadeloupe"),
        ("nb", "Guadeloupe"),
        ("ne", "ग\u{94d}वाद\u{947}लोप"),
        ("nl", "Guadeloupe"),
        ("nn", "Guadeloupe"),
        ("nv", "Guadeloupe"),
        ("oc", "Guadalope"),
        ("or", "ଗ\u{b41}ଆଡେଲୋପ"),
        ("pa", "ਗ\u{a41}ਆਡੀਲ\u{a42}ਪੀ"),
        ("pi", "Guadeloupe"),
        ("pl", "Gwadelupa"),
        ("ps", "Guadeloupe"),
        ("pt", "Guadalupe"),
        ("pt_BR", "Guadalupe"),
        ("ro", "Guadelupa"),
        ("ru", "Гваделупа"),
        ("rw", "Gwadelupe"),
        ("sc", "Guadalupa"),
        ("sd", "Guadeloupe"),
        ("si", "ග\u{dd4}ව\u{dcf}ඩ\u{dd2}ලෝප\u{dca}"),
        ("sk", "Guadeloupe"),
        ("sl", "Gvadelup"),
        ("so", "Guadeloupe"),
        ("sq", "Guadalupë"),
        ("sr", "Гвадалупе"),
        ("sv", "Guadeloupe"),
        ("sw", "Guadeloupe"),
        ("ta", "க\u{bbe}உடிலொபி"),
        ("te", "గ\u{c4d}వ\u{c3e}డ\u{c47}ల\u{c4b}ప\u{c4d}"),
        ("tg", "Гваделупа"),
        ("th", "กวาเดอล\u{e39}ป"),
        ("ti", "Guadeloupe"),
        ("tk", "Gwadelupa"),
        ("tl", "Guadeloupe"),
        ("tr", "Guadeloupe"),
        ("tt", "Gуаделупа"),
        ("ug", "گىۋادېلۇپ"),
        ("uk", "Гваделупа"),
        ("ur", "گواڈیلوپ"),
        ("uz", "Gvadelupa"),
        ("ve", "Guadeloupe"),
        ("vi", "Gu-a-đe-lup"),
        ("wa", "Gwadeloupe"),
        ("wo", "Guwaadaluup"),
        ("xh", "Guadeloupe"),
        ("yo", "Guadeloupe"),
        ("zh_CN", "瓜德罗普"),
        ("zh_HK", "瓜德魯普島"),
        ("zh_TW", "瓜地洛普"),
        ("zu", "Guadeloupe"),
    ];
    #[cfg(all(feature = "gp", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 16.265;
        pub const LONGITUDE: f64 = -61.55099999999999;
        pub const MAX_LATITUDE: f64 = 16.5572273;
        pub const MAX_LONGITUDE: f64 = -60.9473;
        pub const MIN_LATITUDE: f64 = 15.742032;
        pub const MIN_LONGITUDE: f64 = -61.8468;
        pub const NORTHEAST_LATITUDE: f64 = 16.5572273;
        pub const NORTHEAST_LONGITUDE: f64 = -60.9473;
        pub const SOUTHWEST_LATITUDE: f64 = 15.742032;
        pub const SOUTHWEST_LONGITUDE: f64 = -61.8468;
    }
}
#[cfg(all(feature = "gp", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 16.265,
            longitude: -61.55099999999999,
            max_latitude: 16.5572273,
            max_longitude: -60.9473,
            min_latitude: 15.742032,
            min_longitude: -61.8468,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 16.5572273,
                    longitude: -60.9473,
                },
                southwest: CountryGeoBound {
                    latitude: 15.742032,
                    longitude: -61.8468,
                },
            },
        }
    }
}

#[cfg(all(feature = "gp", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "gp")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GP,
        alpha3: Alpha3::GLP,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 590,
        currency_code: "EUR",
        gec: Some(GEC::GP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: None,
        iso_long_name: "Guadeloupe",
        iso_short_name: "Guadeloupe",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "None",
        nationality: Some("Guadeloupian"),
        number: "312",
        postal_code: true,
        postal_code_format: Some("9[78][01]\\d{2}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "GP",
        unofficial_name_list: ["Guadeloupe", "Guadalupe", "グアドループ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Guadeloupe"),
            ("af", "Guadeloupe"),
            ("ak", "Guadeloupe"),
            ("am", "Guadeloupe"),
            ("an", "Guadeloupe"),
            ("ar", "جوادالوب\u{651}ي"),
            ("as", "গ\u{9c1}ৱ\u{9be}ডেলোপ"),
            ("ay", "Guadeloupe"),
            ("az", "Quadelup"),
            ("ba", "Guadeloupe"),
            ("be", "Гвадэлупа"),
            ("bg", "Гваделупа"),
            ("bi", "Guadeloupe"),
            ("bn", "গ\u{9c1}য়\u{9be}ডেল\u{9cd}য\u{9c1}প"),
            ("bn_IN", "গ\u{9c1}য়\u{9be}ডেল\u{9cd}য\u{9c1}প"),
            ("br", "Gwadaloup"),
            ("bs", "Gvadalupe"),
            ("ca", "Guadalupe"),
            ("ce", "Гваделупа"),
            ("ch", "Guadeloupe"),
            ("cs", "Guadeloupe"),
            ("cv", "Гваделупа"),
            ("cy", "Guadeloupe"),
            ("da", "Guadeloupe"),
            ("de", "Guadeloupe"),
            ("dv", "ގ\u{7af}ޑ\u{7a8}ލ\u{7ab}ޕ\u{7aa}"),
            ("dz", "ག\u{f74}་འ་ད\u{f72}་ལ\u{f7c}པ།"),
            ("ee", "Guadeloupe"),
            ("el", "Γουαδελούπη"),
            ("en", "Guadeloupe"),
            ("eo", "Gvadelupo"),
            ("es", "Guadalupe"),
            ("et", "Guadeloupe"),
            ("eu", "Guadalupe"),
            ("fa", "گوادلوپ"),
            ("ff", "Guadeloupe"),
            ("fi", "Guadeloupe"),
            ("fo", "Guadeloupe"),
            ("fr", "Guadeloupe"),
            ("fy", "Guadeloupe"),
            ("ga", "Guadalúip"),
            ("gl", "Guadalupe"),
            ("gn", "Guadeloupe"),
            ("gu", "ગ\u{acd}ડ\u{ac7}લોપ\u{ac7}"),
            ("gv", "Guadeloupe"),
            ("ha", "Guadeloupe"),
            ("he", "גוואדלופ"),
            ("hi", "ग\u{941}आद\u{947}ल\u{942}प"),
            ("hr", "Gvadalupa"),
            ("ht", "Gwadloup"),
            ("hu", "Guadeloupe"),
            ("hy", "Գվադելուպա"),
            ("ia", "Guadeloupe"),
            ("id", "Guadeloupe"),
            ("io", "Guadelupa"),
            ("is", "Gvadalúp"),
            ("it", "Guadalupa"),
            ("iu", "Guadeloupe"),
            ("ja", "グアドループ"),
            ("ka", "გვადალუპე"),
            ("ki", "Guadeloupe"),
            ("kk", "Гваделупа"),
            ("kl", "Guadeloupe"),
            ("km", "ហ\u{17d2}គ\u{17bc}អាដ\u{17ba}ល\u{17bc}ប\u{17c9}េ"),
            ("kn", "Guadeloupe"),
            ("ko", "과들루프"),
            ("ku", "Guadelûp"),
            ("kv", "Guadeloupe"),
            ("kw", "Gwadeloup"),
            ("ky", "Гваделупа"),
            ("lo", "Guadeloupe"),
            ("lt", "Gvadelupa"),
            ("lv", "Gvadelupa"),
            ("mi", "Guadeloupe"),
            ("mk", "Гуадалопе"),
            ("ml", "ഗ\u{d4d}വ\u{d3e}ഡെലോപ\u{d4d}"),
            ("mn", "Guadeloupe"),
            ("mr", "ग\u{94d}वाड\u{947}लोप"),
            ("ms", "Guadeloupe"),
            ("mt", "Gwadelupe"),
            ("my", "ဂ\u{103d}ါဒလ\u{103a}ကျ\u{103d}န\u{103a}း"),
            ("na", "Guadeloupe"),
            ("nb", "Guadeloupe"),
            ("ne", "ग\u{94d}वाद\u{947}लोप"),
            ("nl", "Guadeloupe"),
            ("nn", "Guadeloupe"),
            ("nv", "Guadeloupe"),
            ("oc", "Guadalope"),
            ("or", "ଗ\u{b41}ଆଡେଲୋପ"),
            ("pa", "ਗ\u{a41}ਆਡੀਲ\u{a42}ਪੀ"),
            ("pi", "Guadeloupe"),
            ("pl", "Gwadelupa"),
            ("ps", "Guadeloupe"),
            ("pt", "Guadalupe"),
            ("pt_BR", "Guadalupe"),
            ("ro", "Guadelupa"),
            ("ru", "Гваделупа"),
            ("rw", "Gwadelupe"),
            ("sc", "Guadalupa"),
            ("sd", "Guadeloupe"),
            ("si", "ග\u{dd4}ව\u{dcf}ඩ\u{dd2}ලෝප\u{dca}"),
            ("sk", "Guadeloupe"),
            ("sl", "Gvadelup"),
            ("so", "Guadeloupe"),
            ("sq", "Guadalupë"),
            ("sr", "Гвадалупе"),
            ("sv", "Guadeloupe"),
            ("sw", "Guadeloupe"),
            ("ta", "க\u{bbe}உடிலொபி"),
            ("te", "గ\u{c4d}వ\u{c3e}డ\u{c47}ల\u{c4b}ప\u{c4d}"),
            ("tg", "Гваделупа"),
            ("th", "กวาเดอล\u{e39}ป"),
            ("ti", "Guadeloupe"),
            ("tk", "Gwadelupa"),
            ("tl", "Guadeloupe"),
            ("tr", "Guadeloupe"),
            ("tt", "Gуаделупа"),
            ("ug", "گىۋادېلۇپ"),
            ("uk", "Гваделупа"),
            ("ur", "گواڈیلوپ"),
            ("uz", "Gvadelupa"),
            ("ve", "Guadeloupe"),
            ("vi", "Gu-a-đe-lup"),
            ("wa", "Gwadeloupe"),
            ("wo", "Guwaadaluup"),
            ("xh", "Guadeloupe"),
            ("yo", "Guadeloupe"),
            ("zh_CN", "瓜德罗普"),
            ("zh_HK", "瓜德魯普島"),
            ("zh_TW", "瓜地洛普"),
            ("zu", "Guadeloupe"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}