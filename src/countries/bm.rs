// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Bermuda

#[cfg(all(feature = "bm", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BM;
    pub const ALPHA3: Alpha3 = Alpha3::BMU;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::BMD;
    pub const GEC: Option<GEC> = Some(GEC::BD);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::BER);
    pub const ISO_SHORT_NAME: &str = "Bermuda";
    pub const ISO_LONG_NAME: &str = "Bermuda";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Bermudian");
    pub const NUMBER: &str = "060";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("[A-Z]{2} ?[A-Z0-9]{2}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAmerica);
    pub const UN_LOCODE: &str = "BM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Bermuda", "Bermudes", "Bermudas", "バミューダ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bermuda"),
        ("af", "Bermunda"),
        ("ak", "Bermuda"),
        ("am", "ቤሴሙ፣"),
        ("an", "Bermudas"),
        ("ar", "برمودا"),
        ("as", "ব\u{9be}ৰ\u{9cd}ম\u{9c1}ড\u{9be}"),
        ("ay", "Bermuda"),
        ("az", "Bermuda"),
        ("ba", "Bermuda"),
        ("be", "Бермудскія астравы"),
        ("bg", "Бермуда"),
        ("bi", "Bermuda"),
        ("bn", "ব\u{9be}র\u{9cd}ম\u{9c1}ড\u{9be}"),
        ("bn_IN", "ব\u{9be}র\u{9cd}ম\u{9c1}ড\u{9be}"),
        ("br", "Bermuda"),
        ("bs", "Bermuda"),
        ("ca", "Bermuda"),
        ("ce", "Бермуда гӀайренаш"),
        ("ch", "Bermuda"),
        ("cs", "Bermudy"),
        ("cv", "Бермуда гӀайренаш"),
        ("cy", "Bermuda"),
        ("da", "Bermuda"),
        ("de", "Bermuda"),
        ("dv", "ބ\u{7a7}މ\u{7a8}އ\u{7aa}ޑ\u{7a7}"),
        ("dz", "བར་མ\u{f74}་ད།"),
        ("ee", "Bermuda"),
        ("el", "Βερμούδες"),
        ("en", "Bermuda"),
        ("eo", "Bermudoj"),
        ("es", "Islas Bermudas"),
        ("et", "Bermuda"),
        ("eu", "Bermuda"),
        ("fa", "برمودا"),
        ("ff", "Bermuda"),
        ("fi", "Bermuda"),
        ("fo", "Bermuda"),
        ("fr", "Bermudes"),
        ("fy", "Bermuda"),
        ("ga", "Na Beirmiúdaí"),
        ("gl", "Illas Bermudas"),
        ("gn", "Bermuda"),
        ("gu", "બર\u{acd}મ\u{acd}ય\u{ac1}ડા"),
        ("gv", "Ny Bermioodee"),
        ("ha", "Bermuda"),
        ("he", "ברמודה"),
        ("hi", "बरम\u{942}डा"),
        ("hr", "Bermudi"),
        ("ht", "Bermid"),
        ("hu", "Bermuda"),
        ("hy", "Բերմուդյան Կղզիներ"),
        ("ia", "Bermuda"),
        ("id", "Bermuda"),
        ("io", "Bermuda"),
        ("is", "Bermúda"),
        ("it", "Bermuda"),
        ("iu", "Bermuda"),
        ("ja", "バーミューダ"),
        ("ka", "ბერმუდა"),
        ("ki", "Bermuda"),
        ("kk", "Бермуд аралдары"),
        ("kl", "Bermuda"),
        ("km", "ប\u{17ca}េរម\u{17bc}ដា"),
        ("kn", "ಬರ\u{ccd}ಮುಡಾ"),
        ("ko", "버뮤다"),
        ("ku", "Bermûda"),
        ("kv", "Bermuda"),
        ("kw", "Bermuda"),
        ("ky", "Бермуд аралдары"),
        ("lo", "Bermuda"),
        ("lt", "Bermuda"),
        ("lv", "Bermunda"),
        ("mi", "Bermuda"),
        ("mk", "Бермуди"),
        ("ml", "ബര\u{d4d}\u{200d}മ\u{d41}ഡ"),
        ("mn", "Bermuda"),
        ("mr", "बर\u{94d}म\u{94d}य\u{941}डा"),
        ("ms", "Bermuda"),
        ("mt", "Bermuda"),
        ("my", "Bermuda"),
        ("na", "Bermuda"),
        ("nb", "Bermuda"),
        ("ne", "बरम\u{941}ढा"),
        ("nl", "Bermuda"),
        ("nn", "Bermuda"),
        ("nv", "Bermuda"),
        ("oc", "Bermudas"),
        ("or", "ବର\u{b4d}ମ\u{b41}ଡ\u{b3e}"),
        ("pa", "ਬਾਰਾਮ\u{a42}ਡਾ"),
        ("pi", "Bermuda"),
        ("pl", "Bermudy"),
        ("ps", "Bermuda"),
        ("pt", "Bermudas"),
        ("pt_BR", "Bermuda"),
        ("ro", "Bermude"),
        ("ru", "Бермуды"),
        ("rw", "Berimuda"),
        ("sc", "Bermuda"),
        ("sd", "برمودا"),
        ("si", "බර\u{dca}ම\u{dd2}ය\u{dd4}ඩ\u{dcf}"),
        ("sk", "Bermudy"),
        ("sl", "Bermuda"),
        ("so", "Bermuda"),
        ("sq", "Bermuda"),
        ("sr", "Бермуда"),
        ("sv", "Bermuda"),
        ("sw", "Bermuda"),
        ("ta", "பெர\u{bcd}முட\u{bbe}"),
        ("te", "బ\u{c46}ర\u{c4d}మ\u{c4d}యుడ\u{c3e}"),
        ("tg", "Бермуда"),
        ("th", "เบอร\u{e4c}ม\u{e34}วดา"),
        ("ti", "ቤርሙዳ"),
        ("tk", "Bermud"),
        ("tl", "Bermuda"),
        ("tr", "Bermuda"),
        ("tt", "Бермуда"),
        ("ug", "بېرمۇدا"),
        ("uk", "Бермудські острови"),
        ("ur", "برمودا"),
        ("uz", "Bermuda orollari"),
        ("ve", "Bermuda"),
        ("vi", "Be-mu-đa"),
        ("wa", "Bermudes"),
        ("wo", "Bermuda"),
        ("xh", "Bermuda"),
        ("yo", "Bẹ\u{300}rmúdà"),
        ("zh_CN", "百慕大"),
        ("zh_HK", "百慕達"),
        ("zh_TW", "百慕達"),
        ("zu", "Bermuda"),
    ];
    #[cfg(all(feature = "bm", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 32.3078;
        pub const LONGITUDE: f64 = -64.7505;
        pub const MAX_LATITUDE: f64 = 32.3961;
        pub const MAX_LONGITUDE: f64 = -64.6413999;
        pub const MIN_LATITUDE: f64 = 32.2424975;
        pub const MIN_LONGITUDE: f64 = -64.89139999999999;
        pub const NORTHEAST_LATITUDE: f64 = 32.3961;
        pub const NORTHEAST_LONGITUDE: f64 = -64.6413999;
        pub const SOUTHWEST_LATITUDE: f64 = 32.2424975;
        pub const SOUTHWEST_LONGITUDE: f64 = -64.89139999999999;
    }
}
#[cfg(all(feature = "bm", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 32.3078,
            longitude: -64.7505,
            max_latitude: 32.3961,
            max_longitude: -64.6413999,
            min_latitude: 32.2424975,
            min_longitude: -64.89139999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 32.3961,
                    longitude: -64.6413999,
                },
                southwest: CountryGeoBound {
                    latitude: 32.2424975,
                    longitude: -64.89139999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "bm", feature = "subdivisions"))]
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
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "bm")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BM,
        alpha3: Alpha3::BMU,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: CurrencyCode::BMD,
        gec: Some(GEC::BD),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::BER),
        iso_long_name: "Bermuda",
        iso_short_name: "Bermuda",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Bermudian"),
        number: "060",
        postal_code: true,
        postal_code_format: Some("[A-Z]{2} ?[A-Z0-9]{2}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernAmerica),
        un_locode: "BM",
        unofficial_name_list: ["Bermuda", "Bermudes", "Bermudas", "バミューダ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Bermuda"),
            ("af", "Bermunda"),
            ("ak", "Bermuda"),
            ("am", "ቤሴሙ፣"),
            ("an", "Bermudas"),
            ("ar", "برمودا"),
            ("as", "ব\u{9be}ৰ\u{9cd}ম\u{9c1}ড\u{9be}"),
            ("ay", "Bermuda"),
            ("az", "Bermuda"),
            ("ba", "Bermuda"),
            ("be", "Бермудскія астравы"),
            ("bg", "Бермуда"),
            ("bi", "Bermuda"),
            ("bn", "ব\u{9be}র\u{9cd}ম\u{9c1}ড\u{9be}"),
            ("bn_IN", "ব\u{9be}র\u{9cd}ম\u{9c1}ড\u{9be}"),
            ("br", "Bermuda"),
            ("bs", "Bermuda"),
            ("ca", "Bermuda"),
            ("ce", "Бермуда гӀайренаш"),
            ("ch", "Bermuda"),
            ("cs", "Bermudy"),
            ("cv", "Бермуда гӀайренаш"),
            ("cy", "Bermuda"),
            ("da", "Bermuda"),
            ("de", "Bermuda"),
            ("dv", "ބ\u{7a7}މ\u{7a8}އ\u{7aa}ޑ\u{7a7}"),
            ("dz", "བར་མ\u{f74}་ད།"),
            ("ee", "Bermuda"),
            ("el", "Βερμούδες"),
            ("en", "Bermuda"),
            ("eo", "Bermudoj"),
            ("es", "Islas Bermudas"),
            ("et", "Bermuda"),
            ("eu", "Bermuda"),
            ("fa", "برمودا"),
            ("ff", "Bermuda"),
            ("fi", "Bermuda"),
            ("fo", "Bermuda"),
            ("fr", "Bermudes"),
            ("fy", "Bermuda"),
            ("ga", "Na Beirmiúdaí"),
            ("gl", "Illas Bermudas"),
            ("gn", "Bermuda"),
            ("gu", "બર\u{acd}મ\u{acd}ય\u{ac1}ડા"),
            ("gv", "Ny Bermioodee"),
            ("ha", "Bermuda"),
            ("he", "ברמודה"),
            ("hi", "बरम\u{942}डा"),
            ("hr", "Bermudi"),
            ("ht", "Bermid"),
            ("hu", "Bermuda"),
            ("hy", "Բերմուդյան Կղզիներ"),
            ("ia", "Bermuda"),
            ("id", "Bermuda"),
            ("io", "Bermuda"),
            ("is", "Bermúda"),
            ("it", "Bermuda"),
            ("iu", "Bermuda"),
            ("ja", "バーミューダ"),
            ("ka", "ბერმუდა"),
            ("ki", "Bermuda"),
            ("kk", "Бермуд аралдары"),
            ("kl", "Bermuda"),
            ("km", "ប\u{17ca}េរម\u{17bc}ដា"),
            ("kn", "ಬರ\u{ccd}ಮುಡಾ"),
            ("ko", "버뮤다"),
            ("ku", "Bermûda"),
            ("kv", "Bermuda"),
            ("kw", "Bermuda"),
            ("ky", "Бермуд аралдары"),
            ("lo", "Bermuda"),
            ("lt", "Bermuda"),
            ("lv", "Bermunda"),
            ("mi", "Bermuda"),
            ("mk", "Бермуди"),
            ("ml", "ബര\u{d4d}\u{200d}മ\u{d41}ഡ"),
            ("mn", "Bermuda"),
            ("mr", "बर\u{94d}म\u{94d}य\u{941}डा"),
            ("ms", "Bermuda"),
            ("mt", "Bermuda"),
            ("my", "Bermuda"),
            ("na", "Bermuda"),
            ("nb", "Bermuda"),
            ("ne", "बरम\u{941}ढा"),
            ("nl", "Bermuda"),
            ("nn", "Bermuda"),
            ("nv", "Bermuda"),
            ("oc", "Bermudas"),
            ("or", "ବର\u{b4d}ମ\u{b41}ଡ\u{b3e}"),
            ("pa", "ਬਾਰਾਮ\u{a42}ਡਾ"),
            ("pi", "Bermuda"),
            ("pl", "Bermudy"),
            ("ps", "Bermuda"),
            ("pt", "Bermudas"),
            ("pt_BR", "Bermuda"),
            ("ro", "Bermude"),
            ("ru", "Бермуды"),
            ("rw", "Berimuda"),
            ("sc", "Bermuda"),
            ("sd", "برمودا"),
            ("si", "බර\u{dca}ම\u{dd2}ය\u{dd4}ඩ\u{dcf}"),
            ("sk", "Bermudy"),
            ("sl", "Bermuda"),
            ("so", "Bermuda"),
            ("sq", "Bermuda"),
            ("sr", "Бермуда"),
            ("sv", "Bermuda"),
            ("sw", "Bermuda"),
            ("ta", "பெர\u{bcd}முட\u{bbe}"),
            ("te", "బ\u{c46}ర\u{c4d}మ\u{c4d}యుడ\u{c3e}"),
            ("tg", "Бермуда"),
            ("th", "เบอร\u{e4c}ม\u{e34}วดา"),
            ("ti", "ቤርሙዳ"),
            ("tk", "Bermud"),
            ("tl", "Bermuda"),
            ("tr", "Bermuda"),
            ("tt", "Бермуда"),
            ("ug", "بېرمۇدا"),
            ("uk", "Бермудські острови"),
            ("ur", "برمودا"),
            ("uz", "Bermuda orollari"),
            ("ve", "Bermuda"),
            ("vi", "Be-mu-đa"),
            ("wa", "Bermudes"),
            ("wo", "Bermuda"),
            ("xh", "Bermuda"),
            ("yo", "Bẹ\u{300}rmúdà"),
            ("zh_CN", "百慕大"),
            ("zh_HK", "百慕達"),
            ("zh_TW", "百慕達"),
            ("zu", "Bermuda"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
