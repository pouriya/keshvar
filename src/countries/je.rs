// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Bailiwick of Jersey

#[cfg(all(feature = "je", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::JE;
    pub const ALPHA3: Alpha3 = Alpha3::JEY;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 44;
    pub const CURRENCY_CODE: &str = "GBP";
    pub const GEC: Option<GEC> = Some(GEC::JE);
    pub const INTERNATIONAL_PREFIX: &str = "";
    pub const IOC: Option<&str> = None;
    pub const ISO_SHORT_NAME: &str = "Jersey";
    pub const ISO_LONG_NAME: &str = "The Bailiwick of Jersey";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_PREFIX: &str = "";
    pub const NATIONALITY: Option<&str> = Some("Channel Islander");
    pub const NUMBER: &str = "832";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("JE\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "JE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Jersey", "ジャージー"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Jersey"),
        ("af", "Jersey"),
        ("ak", "Jersey"),
        ("am", "Jersey"),
        ("an", "Jersey"),
        ("ar", "جيرسي"),
        ("as", "জ\u{9be}ৰ\u{9cd}চী"),
        ("ay", "Jersey"),
        ("az", "Jersey"),
        ("ba", "Jersey"),
        ("be", "Джэрсі"),
        ("bg", "Джърси"),
        ("bi", "Jersey"),
        ("bn", "জ\u{9be}র\u{9cd}সি"),
        ("bn_IN", "জ\u{9be}র\u{9cd}সি"),
        ("br", "Jerzenez"),
        ("bs", "Džerzi"),
        ("ca", "Jersey"),
        ("ce", "Jersey"),
        ("ch", "Jersey"),
        ("cs", "Jersey"),
        ("cv", "Jersey"),
        ("cy", "Jersey"),
        ("da", "Jersey"),
        ("de", "Jersey"),
        ("dv", "ޖ\u{7a7}ސ\u{7ad}"),
        ("dz", "ཇར་ས\u{f72}།"),
        ("ee", "Jersey"),
        ("el", "Τζέρσεϊ"),
        ("en", "Jersey"),
        ("eo", "Ĵerzejo"),
        ("es", "Jersey"),
        ("et", "Jersey"),
        ("eu", "Jersey"),
        ("fa", "جرسی"),
        ("ff", "Jersey"),
        ("fi", "Jersey"),
        ("fo", "Jersey"),
        ("fr", "Jersey"),
        ("fy", "Jersey"),
        ("ga", "Geirsí"),
        ("gl", "Jersey"),
        ("gn", "Jersey"),
        ("gu", "જર\u{acd}સી"),
        ("gv", "Jersee"),
        ("ha", "Jersey"),
        ("he", "ג'רזי"),
        ("hi", "जर\u{94d}सी"),
        ("hr", "Jersey"),
        ("ht", "Jersey"),
        ("hu", "Jersey"),
        ("hy", "Ջերսի"),
        ("ia", "Jersey"),
        ("id", "Jersey"),
        ("io", "Jersey"),
        ("is", "Jersey"),
        ("it", "Jersey"),
        ("iu", "Jersey"),
        ("ja", "ジャージー"),
        ("ka", "ჯერსი"),
        ("ki", "Jersey"),
        ("kk", "Джерси"),
        ("kl", "Jersey"),
        ("km", "Jersey"),
        ("kn", "ಜರ\u{ccd}ಸ\u{cbf}"),
        ("ko", "저지 섬"),
        ("ku", "Jersey"),
        ("kv", "Jersey"),
        ("kw", "Jersi"),
        ("ky", "Джерси"),
        ("lo", "Jersey"),
        ("lt", "Džersis"),
        ("lv", "Džersija"),
        ("mi", "Tōrehe"),
        ("mk", "Џерси"),
        ("ml", "ജെര\u{d4d}\u{200d}സി"),
        ("mn", "Jersey"),
        ("mr", "जर\u{94d}सी"),
        ("ms", "Jersey"),
        ("mt", "Jersey"),
        ("my", "Jersey"),
        ("na", "Jersey"),
        ("nb", "Jersey"),
        ("ne", "जर\u{94d}सी"),
        ("nl", "Jersey"),
        ("nn", "Jersey"),
        ("nv", "Jersey"),
        ("oc", "Jersei"),
        ("or", "ଜର\u{b4d}ସୀ"),
        ("pa", "ਜਰਸੀ"),
        ("pi", "Jersey"),
        ("pl", "Jersey"),
        ("ps", "Jersey"),
        ("pt", "Jersey"),
        ("pt_BR", "Jersey"),
        ("ro", "Jersey"),
        ("ru", "Джерси"),
        ("rw", "Jersey"),
        ("sc", "Jersey"),
        ("sd", "Jersey"),
        ("si", "ජර\u{dca}ස\u{dd2}"),
        ("sk", "Jersey"),
        ("sl", "Jersey"),
        ("so", "Jersey"),
        ("sq", "Xhersi"),
        ("sr", "Џерси"),
        ("sv", "Jersey"),
        ("sw", "Jersey"),
        ("ta", "ஜெர\u{bcd}சி"),
        ("te", "జర\u{c4d}స\u{c40}"),
        ("tg", "Ҷерси"),
        ("th", "เจอร\u{e4c}ซ\u{e35}ย\u{e4c}"),
        ("ti", "Jersey"),
        ("tk", "Jersey"),
        ("tl", "Jersey"),
        ("tr", "Jersey"),
        ("tt", "Jersey"),
        ("ug", "جېرسېي"),
        ("uk", "Джерсі"),
        ("ur", "جرزی"),
        ("uz", "Jersey"),
        ("ve", "Jersey"),
        ("vi", "Giơ-xị"),
        ("wa", "Djersey"),
        ("wo", "Jersey"),
        ("xh", "Jersey"),
        ("yo", "Jersey"),
        ("zh_CN", "泽西岛"),
        ("zh_HK", "澤西"),
        ("zh_TW", "澤西島"),
        ("zu", "Jersey"),
    ];
    #[cfg(all(feature = "je", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 49.214439;
        pub const LONGITUDE: f64 = -2.13125;
        pub const MAX_LATITUDE: f64 = 49.26650009999999;
        pub const MAX_LONGITUDE: f64 = -2.0013001;
        pub const MIN_LATITUDE: f64 = 49.1582;
        pub const MIN_LONGITUDE: f64 = -2.2602001;
        pub const NORTHEAST_LATITUDE: f64 = 49.26650009999999;
        pub const NORTHEAST_LONGITUDE: f64 = -2.0013001;
        pub const SOUTHWEST_LATITUDE: f64 = 49.1582;
        pub const SOUTHWEST_LONGITUDE: f64 = -2.2602001;
    }
}
#[cfg(all(feature = "je", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 49.214439,
            longitude: -2.13125,
            max_latitude: 49.26650009999999,
            max_longitude: -2.0013001,
            min_latitude: 49.1582,
            min_longitude: -2.2602001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 49.26650009999999,
                    longitude: -2.0013001,
                },
                southwest: CountryGeoBound {
                    latitude: 49.1582,
                    longitude: -2.2602001,
                },
            },
        }
    }
}

#[cfg(all(feature = "je", feature = "subdivisions"))]
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
#[cfg(feature = "je")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::JE,
        alpha3: Alpha3::JEY,
        address_format: None,
        continent: Continent::Europe,
        country_code: 44,
        currency_code: "GBP",
        gec: Some(GEC::JE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "",
        ioc: None,
        iso_long_name: "The Bailiwick of Jersey",
        iso_short_name: "Jersey",
        official_language_list: ["en", "fr"].to_vec(),
        spoken_language_list: ["en", "fr"].to_vec(),
        national_destination_code_length_list: [].to_vec(),
        national_number_length_list: [].to_vec(),
        national_prefix: "",
        nationality: Some("Channel Islander"),
        number: "832",
        postal_code: true,
        postal_code_format: Some("JE\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "JE",
        unofficial_name_list: ["Jersey", "ジャージー"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Jersey"),
            ("af", "Jersey"),
            ("ak", "Jersey"),
            ("am", "Jersey"),
            ("an", "Jersey"),
            ("ar", "جيرسي"),
            ("as", "জ\u{9be}ৰ\u{9cd}চী"),
            ("ay", "Jersey"),
            ("az", "Jersey"),
            ("ba", "Jersey"),
            ("be", "Джэрсі"),
            ("bg", "Джърси"),
            ("bi", "Jersey"),
            ("bn", "জ\u{9be}র\u{9cd}সি"),
            ("bn_IN", "জ\u{9be}র\u{9cd}সি"),
            ("br", "Jerzenez"),
            ("bs", "Džerzi"),
            ("ca", "Jersey"),
            ("ce", "Jersey"),
            ("ch", "Jersey"),
            ("cs", "Jersey"),
            ("cv", "Jersey"),
            ("cy", "Jersey"),
            ("da", "Jersey"),
            ("de", "Jersey"),
            ("dv", "ޖ\u{7a7}ސ\u{7ad}"),
            ("dz", "ཇར་ས\u{f72}།"),
            ("ee", "Jersey"),
            ("el", "Τζέρσεϊ"),
            ("en", "Jersey"),
            ("eo", "Ĵerzejo"),
            ("es", "Jersey"),
            ("et", "Jersey"),
            ("eu", "Jersey"),
            ("fa", "جرسی"),
            ("ff", "Jersey"),
            ("fi", "Jersey"),
            ("fo", "Jersey"),
            ("fr", "Jersey"),
            ("fy", "Jersey"),
            ("ga", "Geirsí"),
            ("gl", "Jersey"),
            ("gn", "Jersey"),
            ("gu", "જર\u{acd}સી"),
            ("gv", "Jersee"),
            ("ha", "Jersey"),
            ("he", "ג'רזי"),
            ("hi", "जर\u{94d}सी"),
            ("hr", "Jersey"),
            ("ht", "Jersey"),
            ("hu", "Jersey"),
            ("hy", "Ջերսի"),
            ("ia", "Jersey"),
            ("id", "Jersey"),
            ("io", "Jersey"),
            ("is", "Jersey"),
            ("it", "Jersey"),
            ("iu", "Jersey"),
            ("ja", "ジャージー"),
            ("ka", "ჯერსი"),
            ("ki", "Jersey"),
            ("kk", "Джерси"),
            ("kl", "Jersey"),
            ("km", "Jersey"),
            ("kn", "ಜರ\u{ccd}ಸ\u{cbf}"),
            ("ko", "저지 섬"),
            ("ku", "Jersey"),
            ("kv", "Jersey"),
            ("kw", "Jersi"),
            ("ky", "Джерси"),
            ("lo", "Jersey"),
            ("lt", "Džersis"),
            ("lv", "Džersija"),
            ("mi", "Tōrehe"),
            ("mk", "Џерси"),
            ("ml", "ജെര\u{d4d}\u{200d}സി"),
            ("mn", "Jersey"),
            ("mr", "जर\u{94d}सी"),
            ("ms", "Jersey"),
            ("mt", "Jersey"),
            ("my", "Jersey"),
            ("na", "Jersey"),
            ("nb", "Jersey"),
            ("ne", "जर\u{94d}सी"),
            ("nl", "Jersey"),
            ("nn", "Jersey"),
            ("nv", "Jersey"),
            ("oc", "Jersei"),
            ("or", "ଜର\u{b4d}ସୀ"),
            ("pa", "ਜਰਸੀ"),
            ("pi", "Jersey"),
            ("pl", "Jersey"),
            ("ps", "Jersey"),
            ("pt", "Jersey"),
            ("pt_BR", "Jersey"),
            ("ro", "Jersey"),
            ("ru", "Джерси"),
            ("rw", "Jersey"),
            ("sc", "Jersey"),
            ("sd", "Jersey"),
            ("si", "ජර\u{dca}ස\u{dd2}"),
            ("sk", "Jersey"),
            ("sl", "Jersey"),
            ("so", "Jersey"),
            ("sq", "Xhersi"),
            ("sr", "Џерси"),
            ("sv", "Jersey"),
            ("sw", "Jersey"),
            ("ta", "ஜெர\u{bcd}சி"),
            ("te", "జర\u{c4d}స\u{c40}"),
            ("tg", "Ҷерси"),
            ("th", "เจอร\u{e4c}ซ\u{e35}ย\u{e4c}"),
            ("ti", "Jersey"),
            ("tk", "Jersey"),
            ("tl", "Jersey"),
            ("tr", "Jersey"),
            ("tt", "Jersey"),
            ("ug", "جېرسېي"),
            ("uk", "Джерсі"),
            ("ur", "جرزی"),
            ("uz", "Jersey"),
            ("ve", "Jersey"),
            ("vi", "Giơ-xị"),
            ("wa", "Djersey"),
            ("wo", "Jersey"),
            ("xh", "Jersey"),
            ("yo", "Jersey"),
            ("zh_CN", "泽西岛"),
            ("zh_HK", "澤西"),
            ("zh_TW", "澤西島"),
            ("zu", "Jersey"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
