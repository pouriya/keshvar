// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Commonwealth of Puerto Rico

#[cfg(all(feature = "pr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PR;
    pub const ALPHA3: Alpha3 = Alpha3::PRI;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: &str = "USD";
    pub const GEC: Option<GEC> = Some(GEC::RQ);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::PUR);
    pub const ISO_SHORT_NAME: &str = "Puerto Rico";
    pub const ISO_LONG_NAME: &str = "The Commonwealth of Puerto Rico";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Puerto Rican");
    pub const NUMBER: &str = "630";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(00[679]\\d{2})(?:[ \\-](\\d{4}))?");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "PR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Puerto Rico", "プエルトリコ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Puerto Rico"),
        ("af", "Puerto Rico"),
        ("ak", "Puerto Rico"),
        ("am", "ፖሴታ ሱጥ"),
        ("an", "Puerto Rico"),
        ("ar", "بورتوريكو"),
        ("as", "প\u{9c2}ৱেৰ\u{9cd}টো ৰিকো"),
        ("ay", "Puerto Rico"),
        ("az", "Porto Riko"),
        ("ba", "Puerto Rico"),
        ("be", "Пуэрта-Рыка"),
        ("bg", "Пуерто Рико"),
        ("bi", "Puerto Rico"),
        ("bn", "প\u{9c1}য়ের\u{9cd}তো রিকো"),
        ("bn_IN", "প\u{9c1}য়ের\u{9cd}তো রিকো"),
        ("br", "Puerto Rico"),
        ("bs", "Portoriko"),
        ("ca", "Puerto Rico"),
        ("ce", "Пуэрто-Рико"),
        ("ch", "Puerto Rico"),
        ("cs", "Portoriko"),
        ("cv", "Пуэрто-Рико"),
        ("cy", "Puerto Rico"),
        ("da", "Puerto Rico"),
        ("de", "Puerto Rico"),
        ("dv", "ޕ\u{7aa}އ\u{7ac}ރ\u{7b0}ތ\u{7ae} ރ\u{7a9}ކ\u{7af}"),
        ("dz", "པ\u{f74}ར་ཊ\u{f7c}་ ར\u{f72}་ཀ\u{f7c}།"),
        ("ee", "Puerto Rico"),
        ("el", "Πουέρτο Ρίκο"),
        ("en", "Puerto Rico"),
        ("eo", "Puerto-Riko"),
        ("es", "Puerto Rico"),
        ("et", "Puerto Rico"),
        ("eu", "Puerto Rico"),
        ("fa", "پورتو ریکو"),
        ("ff", "Puerto Rico"),
        ("fi", "Puerto Rico"),
        ("fo", "Puerto Rico"),
        ("fr", "Porto Rico"),
        ("fy", "Puerto Riko"),
        ("ga", "Portó Ríce"),
        ("gl", "Porto Rico"),
        ("gn", "Puerto Rico"),
        ("gu", "પ\u{ac1}ર\u{acd}ટો રીકો"),
        ("gv", "Yn Phurt Verçhagh"),
        ("ha", "Puerto Rico"),
        ("he", "פוארטו ריקו"),
        ("hi", "प\u{94d}य\u{941}र\u{94d}तो रिको"),
        ("hr", "Portoriko"),
        ("ht", "Pòtoriko"),
        ("hu", "Puerto Rico"),
        ("hy", "Պուերտո Ռիկո"),
        ("ia", "Porto Rico"),
        ("id", "Puerto Riko"),
        ("io", "Portuo Riko"),
        ("is", "Puerto Ríko"),
        ("it", "Portorico"),
        ("iu", "Puerto Rico"),
        ("ja", "プエルトリコ"),
        ("ka", "პუერტო-რიკო"),
        ("ki", "Puerto Rico"),
        ("kk", "Пуэрто-Рико"),
        ("kl", "Puerto Rico"),
        ("km", "ព\u{17d0}រត\u{17bc}រ\u{17b8}ក\u{17bc}"),
        ("kn", "ಪೋರ\u{ccd}ಟೋರ\u{cbf}ಕಾ"),
        ("ko", "푸에르토리코"),
        ("ku", "Porto Rîko"),
        ("kv", "Puerto Rico"),
        ("kw", "Puerto Rico"),
        ("ky", "Пуэрто-Рико"),
        ("lo", "Puerto Rico"),
        ("lt", "Puerto Rikas"),
        ("lv", "Puertoriko"),
        ("mi", "Puerto Rico"),
        ("mk", "Пуерто Рико"),
        ("ml", "പ\u{d4d}യ\u{d42}ര\u{d4d}\u{200d}ട\u{d4d}ടോ റികോ"),
        ("mn", "Пуерто рико"),
        ("mr", "प\u{94d}य\u{941}र\u{94d}टो रिको"),
        ("ms", "Puerto Rico"),
        ("mt", "Porto Riku"),
        ("my", "Puerto Rico"),
        ("na", "Puerto Rico"),
        ("nb", "Puerto Rico"),
        ("ne", "प\u{941}रटोरिको"),
        ("nl", "Puerto Rico"),
        ("nn", "Puerto Rico"),
        ("nv", "Puerto Rico"),
        ("oc", "Puerto Rico"),
        ("or", "ପ\u{b4d}ଯ\u{b42}ରେଟୋ ର\u{b3f}କୋ"),
        ("pa", "ਪ\u{a41}ਈਰਟ\u{a4b} ਰੀਸ\u{a4b}"),
        ("pi", "Puerto Rico"),
        ("pl", "Portoryko"),
        ("ps", "Puerto Rico"),
        ("pt", "Porto Rico"),
        ("pt_BR", "Porto Rico"),
        ("ro", "Puerto Rico"),
        ("ru", "Пуэрто-Рико"),
        ("rw", "Puwerito Riko"),
        ("sc", "Puerto Rico"),
        ("sd", "Puerto Rico"),
        ("si", "ප\u{dd4}වටෝ ර\u{dd2}කෝව"),
        ("sk", "Portoriko"),
        ("sl", "Portoriko"),
        ("so", "Puerto Rico"),
        ("sq", "Porto Riko"),
        ("sr", "Порторико"),
        ("sv", "Puerto Rico"),
        ("sw", "Puerto Rico"),
        ("ta", "போர\u{bcd}ட\u{bcd}டோ ர\u{bc0}க\u{bcd}கோ"),
        ("te", "ప\u{c4d}యుర\u{c4d}ట\u{c4b} ర\u{c3f}క\u{c4b}"),
        ("tg", "Пуэрто-Рико"),
        ("th", "เปอร\u{e4c}โตร\u{e34}โก"),
        ("ti", "ፖርታ ሪኮ"),
        ("tk", "Puerto-Riko"),
        ("tl", "Puerto Rico"),
        ("tr", "Porto Riko"),
        ("tt", "Пуерто Рико"),
        ("ug", "پۇئېرتو-رىكو"),
        ("uk", "Пуерто-Рико"),
        ("ur", "پورٹو ریکو"),
        ("uz", "Puerto-Riko"),
        ("ve", "Puerto Rico"),
        ("vi", "Pu-éc-thô Ri-cô"),
        ("wa", "Porto Rico"),
        ("wo", "Poorto Riiko"),
        ("xh", "Puerto Rico"),
        ("yo", "Púẹ\u{301}rtò Ríkò"),
        ("zh_CN", "波多黎各"),
        ("zh_HK", "波多黎各"),
        ("zh_TW", "波多黎各"),
        ("zu", "Puerto Rico"),
    ];
    #[cfg(all(feature = "pr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 18.220833;
        pub const LONGITUDE: f64 = -66.590149;
        pub const MAX_LATITUDE: f64 = 18.5720479;
        pub const MAX_LONGITUDE: f64 = -65.2105715;
        pub const MIN_LATITUDE: f64 = 17.8449191;
        pub const MIN_LONGITUDE: f64 = -67.9611844;
        pub const NORTHEAST_LATITUDE: f64 = 18.5720479;
        pub const NORTHEAST_LONGITUDE: f64 = -65.2105715;
        pub const SOUTHWEST_LATITUDE: f64 = 17.8449191;
        pub const SOUTHWEST_LONGITUDE: f64 = -67.9611844;
    }
}
#[cfg(all(feature = "pr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 18.220833,
            longitude: -66.590149,
            max_latitude: 18.5720479,
            max_longitude: -65.2105715,
            min_latitude: 17.8449191,
            min_longitude: -67.9611844,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 18.5720479,
                    longitude: -65.2105715,
                },
                southwest: CountryGeoBound {
                    latitude: 17.8449191,
                    longitude: -67.9611844,
                },
            },
        }
    }
}

#[cfg(all(feature = "pr", feature = "subdivisions"))]
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
use crate::{
    Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "pr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PR,
        alpha3: Alpha3::PRI,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: "USD",
        gec: Some(GEC::RQ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::PUR),
        iso_long_name: "The Commonwealth of Puerto Rico",
        iso_short_name: "Puerto Rico",
        official_language_list: ["en", "es"].to_vec(),
        spoken_language_list: ["en", "es"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Puerto Rican"),
        number: "630",
        postal_code: true,
        postal_code_format: Some("(00[679]\\d{2})(?:[ \\-](\\d{4}))?"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "PR",
        unofficial_name_list: ["Puerto Rico", "プエルトリコ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Puerto Rico"),
            ("af", "Puerto Rico"),
            ("ak", "Puerto Rico"),
            ("am", "ፖሴታ ሱጥ"),
            ("an", "Puerto Rico"),
            ("ar", "بورتوريكو"),
            ("as", "প\u{9c2}ৱেৰ\u{9cd}টো ৰিকো"),
            ("ay", "Puerto Rico"),
            ("az", "Porto Riko"),
            ("ba", "Puerto Rico"),
            ("be", "Пуэрта-Рыка"),
            ("bg", "Пуерто Рико"),
            ("bi", "Puerto Rico"),
            ("bn", "প\u{9c1}য়ের\u{9cd}তো রিকো"),
            ("bn_IN", "প\u{9c1}য়ের\u{9cd}তো রিকো"),
            ("br", "Puerto Rico"),
            ("bs", "Portoriko"),
            ("ca", "Puerto Rico"),
            ("ce", "Пуэрто-Рико"),
            ("ch", "Puerto Rico"),
            ("cs", "Portoriko"),
            ("cv", "Пуэрто-Рико"),
            ("cy", "Puerto Rico"),
            ("da", "Puerto Rico"),
            ("de", "Puerto Rico"),
            ("dv", "ޕ\u{7aa}އ\u{7ac}ރ\u{7b0}ތ\u{7ae} ރ\u{7a9}ކ\u{7af}"),
            ("dz", "པ\u{f74}ར་ཊ\u{f7c}་ ར\u{f72}་ཀ\u{f7c}།"),
            ("ee", "Puerto Rico"),
            ("el", "Πουέρτο Ρίκο"),
            ("en", "Puerto Rico"),
            ("eo", "Puerto-Riko"),
            ("es", "Puerto Rico"),
            ("et", "Puerto Rico"),
            ("eu", "Puerto Rico"),
            ("fa", "پورتو ریکو"),
            ("ff", "Puerto Rico"),
            ("fi", "Puerto Rico"),
            ("fo", "Puerto Rico"),
            ("fr", "Porto Rico"),
            ("fy", "Puerto Riko"),
            ("ga", "Portó Ríce"),
            ("gl", "Porto Rico"),
            ("gn", "Puerto Rico"),
            ("gu", "પ\u{ac1}ર\u{acd}ટો રીકો"),
            ("gv", "Yn Phurt Verçhagh"),
            ("ha", "Puerto Rico"),
            ("he", "פוארטו ריקו"),
            ("hi", "प\u{94d}य\u{941}र\u{94d}तो रिको"),
            ("hr", "Portoriko"),
            ("ht", "Pòtoriko"),
            ("hu", "Puerto Rico"),
            ("hy", "Պուերտո Ռիկո"),
            ("ia", "Porto Rico"),
            ("id", "Puerto Riko"),
            ("io", "Portuo Riko"),
            ("is", "Puerto Ríko"),
            ("it", "Portorico"),
            ("iu", "Puerto Rico"),
            ("ja", "プエルトリコ"),
            ("ka", "პუერტო-რიკო"),
            ("ki", "Puerto Rico"),
            ("kk", "Пуэрто-Рико"),
            ("kl", "Puerto Rico"),
            ("km", "ព\u{17d0}រត\u{17bc}រ\u{17b8}ក\u{17bc}"),
            ("kn", "ಪೋರ\u{ccd}ಟೋರ\u{cbf}ಕಾ"),
            ("ko", "푸에르토리코"),
            ("ku", "Porto Rîko"),
            ("kv", "Puerto Rico"),
            ("kw", "Puerto Rico"),
            ("ky", "Пуэрто-Рико"),
            ("lo", "Puerto Rico"),
            ("lt", "Puerto Rikas"),
            ("lv", "Puertoriko"),
            ("mi", "Puerto Rico"),
            ("mk", "Пуерто Рико"),
            ("ml", "പ\u{d4d}യ\u{d42}ര\u{d4d}\u{200d}ട\u{d4d}ടോ റികോ"),
            ("mn", "Пуерто рико"),
            ("mr", "प\u{94d}य\u{941}र\u{94d}टो रिको"),
            ("ms", "Puerto Rico"),
            ("mt", "Porto Riku"),
            ("my", "Puerto Rico"),
            ("na", "Puerto Rico"),
            ("nb", "Puerto Rico"),
            ("ne", "प\u{941}रटोरिको"),
            ("nl", "Puerto Rico"),
            ("nn", "Puerto Rico"),
            ("nv", "Puerto Rico"),
            ("oc", "Puerto Rico"),
            ("or", "ପ\u{b4d}ଯ\u{b42}ରେଟୋ ର\u{b3f}କୋ"),
            ("pa", "ਪ\u{a41}ਈਰਟ\u{a4b} ਰੀਸ\u{a4b}"),
            ("pi", "Puerto Rico"),
            ("pl", "Portoryko"),
            ("ps", "Puerto Rico"),
            ("pt", "Porto Rico"),
            ("pt_BR", "Porto Rico"),
            ("ro", "Puerto Rico"),
            ("ru", "Пуэрто-Рико"),
            ("rw", "Puwerito Riko"),
            ("sc", "Puerto Rico"),
            ("sd", "Puerto Rico"),
            ("si", "ප\u{dd4}වටෝ ර\u{dd2}කෝව"),
            ("sk", "Portoriko"),
            ("sl", "Portoriko"),
            ("so", "Puerto Rico"),
            ("sq", "Porto Riko"),
            ("sr", "Порторико"),
            ("sv", "Puerto Rico"),
            ("sw", "Puerto Rico"),
            ("ta", "போர\u{bcd}ட\u{bcd}டோ ர\u{bc0}க\u{bcd}கோ"),
            ("te", "ప\u{c4d}యుర\u{c4d}ట\u{c4b} ర\u{c3f}క\u{c4b}"),
            ("tg", "Пуэрто-Рико"),
            ("th", "เปอร\u{e4c}โตร\u{e34}โก"),
            ("ti", "ፖርታ ሪኮ"),
            ("tk", "Puerto-Riko"),
            ("tl", "Puerto Rico"),
            ("tr", "Porto Riko"),
            ("tt", "Пуерто Рико"),
            ("ug", "پۇئېرتو-رىكو"),
            ("uk", "Пуерто-Рико"),
            ("ur", "پورٹو ریکو"),
            ("uz", "Puerto-Riko"),
            ("ve", "Puerto Rico"),
            ("vi", "Pu-éc-thô Ri-cô"),
            ("wa", "Porto Rico"),
            ("wo", "Poorto Riiko"),
            ("xh", "Puerto Rico"),
            ("yo", "Púẹ\u{301}rtò Ríkò"),
            ("zh_CN", "波多黎各"),
            ("zh_HK", "波多黎各"),
            ("zh_TW", "波多黎各"),
            ("zu", "Puerto Rico"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
