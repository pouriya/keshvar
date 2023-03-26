// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Bouvet Island

#[cfg(all(feature = "bv", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BV;
    pub const ALPHA3: Alpha3 = Alpha3::BVT;
    pub const CONTINENT: Continent = Continent::Antarctica;
    pub const COUNTRY_CODE: usize = 47;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::NOK;
    pub const GEC: Option<GEC> = Some(GEC::BV);
    pub const INTERNATIONAL_PREFIX: &str = "";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Bouvet Island";
    pub const ISO_LONG_NAME: &str = "Bouvet Island";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &[];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &[];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_PREFIX: &str = "";
    pub const NATIONALITY: Option<&str> = None;
    pub const NUMBER: &str = "074";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = None;
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = None;
    pub const UN_LOCODE: &str = "BV";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Bouvet Island", "Bouvetinsel", "ブーベ島", "Bouveteiland"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bouvet Island"),
        ("af", "Bouvet-eiland"),
        ("ak", "Bouvet Island"),
        ("am", "Bouvet Island"),
        ("an", "Isla Bouvet"),
        ("ar", "جزيرة بوفي"),
        ("as", "বোভেট দ\u{9cd}বীপ"),
        ("ay", "Bouvet Island"),
        ("az", "Buvet Adası"),
        ("ba", "Bouvet Island"),
        ("be", "Востраў Бувэ"),
        ("bg", "Остров Буве"),
        ("bi", "Bouvet Island"),
        ("bn", "ব\u{9c1}ভে দ\u{9cd}বীপ"),
        ("bn_IN", "ব\u{9c1}ভে দ\u{9cd}বীপ"),
        ("br", "Enez Bouvet"),
        ("bs", "Ostrvo Bouvet"),
        ("ca", "Illa Bouvet"),
        ("ce", "Бувен гӀайре"),
        ("ch", "Bouvet Island"),
        ("cs", "Bouvetův ostrov"),
        ("cv", "Бувен гӀайре"),
        ("cy", "Ynys Bouvet"),
        ("da", "Bouvet-øen"),
        ("de", "Bouvet-Insel"),
        ("dv", "Bouvet Island"),
        (
            "dz",
            "བའ\u{f74}་བ\u{f7a}ཊ\u{f72}་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།",
        ),
        ("ee", "Bouvet Island"),
        ("el", "Νήσος Μπουβέ"),
        ("en", "Bouvet Island"),
        ("eo", "Buvetinsulo"),
        ("es", "Isla Bouvet"),
        ("et", "Bouvet' saar"),
        ("eu", "Bouvet uhartea"),
        ("fa", "جزیره\u{654} بووت"),
        ("ff", "Bouvet Island"),
        ("fi", "Bouvet'nsaari"),
        ("fo", "Bouvet Island"),
        ("fr", "île Bouvet"),
        ("fy", "Bouveteilân"),
        ("ga", "Oileán Bouvet"),
        ("gl", "Illa Bouvet"),
        ("gn", "Bouvet Island"),
        ("gu", "બોઉવ\u{ac7}ટ ટાપ\u{ac1}ઓ"),
        ("gv", "Bouvet Island"),
        ("ha", "Bouvet Island"),
        ("he", "בובה"),
        ("hi", "बोउव\u{947}ट आइल\u{948}\u{902}ड"),
        ("hr", "Otok Bouvet"),
        ("ht", "Bouvet Island"),
        ("hu", "Bouvet-sziget"),
        ("hy", "Բուվետ Կղզի"),
        ("ia", "Insula Bouvet"),
        ("id", "Pulau Bouvet"),
        ("io", "Bouvet Insulo"),
        ("is", "Bouveteyja"),
        ("it", "Isola Bouvet"),
        ("iu", "Bouvet Island"),
        ("ja", "ブーベ島"),
        ("ka", "ბუვეტის კუნძული"),
        ("ki", "Bouvet Island"),
        ("kk", "Буве аралдары"),
        ("kl", "Bouvet Island"),
        ("km", "កោះ\u{200b}ប\u{17ca}\u{17bc}វ\u{17c9}ែ"),
        ("kn", "ಬವೇಟ\u{ccd} ದ\u{ccd}ವೀಪ"),
        ("ko", "부베 섬"),
        ("ku", "Girava Boyuvet"),
        ("kv", "Bouvet Island"),
        ("kw", "Bouvet Island"),
        ("ky", "Буве аралы"),
        ("lo", "Bouvet Island"),
        ("lt", "Buvė sala"),
        ("lv", "Buvē Sala"),
        ("mi", "Bouvet Island"),
        ("mk", "Бувет острови"),
        ("ml", "ബൌവെ ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}"),
        ("mn", "Буве арал"),
        ("mr", "बोव\u{94d}ह\u{947} आयल\u{945}\u{902}ड"),
        ("ms", "Kepulauan Bouvet"),
        ("mt", "Gżira ta' Bouvet"),
        ("my", "Bouvet Island"),
        ("na", "Bouvet Island"),
        ("nb", "Bouvetøya"),
        ("ne", "बोउव\u{947}ट टाप\u{941}"),
        ("nl", "Bouveteiland"),
        ("nn", "Bouvetøya"),
        ("nv", "Bouvet Island"),
        ("oc", "Illa Bouvet"),
        ("or", "ବ\u{b41}ଭେଟ\u{b4d} ଦ\u{b4d}ବୀପ"),
        ("pa", "ਬ\u{a42}ਟਵਟ ਟਾਪ\u{a42}"),
        ("pi", "Bouvet Island"),
        ("pl", "Wyspa Bouveta"),
        ("ps", "Bouvet Island"),
        ("pt", "Ilha Bouvet"),
        ("pt_BR", "Ilha Bouvet"),
        ("ro", "Insula Bouvet"),
        ("ru", "Остров Буве"),
        ("rw", "Ikirwa cya Bouve"),
        ("sc", "Ìsula Bouvet"),
        ("sd", "Bouvet Island"),
        ("si", "බෝවෙට\u{dca} ද\u{dd6}පත"),
        ("sk", "Bouvetov ostrov"),
        ("sl", "Bouvetov otok"),
        ("so", "Bouvet Island"),
        ("sq", "Ishulli Buve"),
        ("sr", "Острво Бов"),
        ("sv", "Bouvetön"),
        ("sw", "Kisiwa cha Bouvet"),
        ("ta", "போவெட\u{bcd} த\u{bc0}வு"),
        ("te", "బ\u{c4b}వ\u{c4d}హ\u{c47} ఐల\u{c3e}ండ\u{c4d}"),
        ("tg", "Ҷазираи Бувит"),
        ("th", "เกาะบ\u{e39}เวต\u{e4c}"),
        ("ti", "Bouvet Island"),
        ("tk", "Bouvet Island"),
        ("tl", "Islang Bouvet"),
        ("tr", "Bouvet Adası"),
        ("tt", "Боувет Утравы"),
        ("ug", "بۇۋېت ئارىلى"),
        ("uk", "Острів Буве"),
        ("ur", "جزیرہ بووہ"),
        ("uz", "Bouvet Island"),
        ("ve", "Bouvet Island"),
        ("vi", "Quần đảo Bu-vê"),
        ("wa", "Iye Bouvet"),
        ("wo", "Dunu Buwet"),
        ("xh", "Bouvet Island"),
        ("yo", "Erékùṣù Bouvet"),
        ("zh_CN", "布维群岛"),
        ("zh_HK", "波維特島"),
        ("zh_TW", "布威島"),
        ("zu", "Bouvet Island"),
    ];
    #[cfg(all(feature = "bv", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -54.4207915;
        pub const LONGITUDE: f64 = 3.3464497;
        pub const MAX_LATITUDE: f64 = -54.3869298;
        pub const MAX_LONGITUDE: f64 = 3.4332785;
        pub const MIN_LATITUDE: f64 = -54.4541004;
        pub const MIN_LONGITUDE: f64 = 3.2858826;
        pub const NORTHEAST_LATITUDE: f64 = -54.3869298;
        pub const NORTHEAST_LONGITUDE: f64 = 3.4332785;
        pub const SOUTHWEST_LATITUDE: f64 = -54.4541004;
        pub const SOUTHWEST_LONGITUDE: f64 = 3.2858826;
    }
}
#[cfg(all(feature = "bv", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -54.4207915,
            longitude: 3.3464497,
            max_latitude: -54.3869298,
            max_longitude: 3.4332785,
            min_latitude: -54.4541004,
            min_longitude: 3.2858826,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -54.3869298,
                    longitude: 3.4332785,
                },
                southwest: CountryGeoBound {
                    latitude: -54.4541004,
                    longitude: 3.2858826,
                },
            },
        }
    }
}

#[cfg(all(feature = "bv", feature = "subdivisions"))]
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
#[cfg(feature = "bv")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BV,
        alpha3: Alpha3::BVT,
        address_format: None,
        continent: Continent::Antarctica,
        country_code: 47,
        currency_code: CurrencyCode::NOK,
        gec: Some(GEC::BV),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "",
        ioc: None,
        iso_long_name: "Bouvet Island",
        iso_short_name: "Bouvet Island",
        official_language_list: [].to_vec(),
        spoken_language_list: [].to_vec(),
        national_destination_code_length_list: [].to_vec(),
        national_number_length_list: [].to_vec(),
        national_prefix: "",
        nationality: None,
        number: "074",
        postal_code: false,
        postal_code_format: None,
        region: None,
        start_of_week: WeekDay::Monday,
        subregion: None,
        un_locode: "BV",
        unofficial_name_list: ["Bouvet Island", "Bouvetinsel", "ブーベ島", "Bouveteiland"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Bouvet Island"),
            ("af", "Bouvet-eiland"),
            ("ak", "Bouvet Island"),
            ("am", "Bouvet Island"),
            ("an", "Isla Bouvet"),
            ("ar", "جزيرة بوفي"),
            ("as", "বোভেট দ\u{9cd}বীপ"),
            ("ay", "Bouvet Island"),
            ("az", "Buvet Adası"),
            ("ba", "Bouvet Island"),
            ("be", "Востраў Бувэ"),
            ("bg", "Остров Буве"),
            ("bi", "Bouvet Island"),
            ("bn", "ব\u{9c1}ভে দ\u{9cd}বীপ"),
            ("bn_IN", "ব\u{9c1}ভে দ\u{9cd}বীপ"),
            ("br", "Enez Bouvet"),
            ("bs", "Ostrvo Bouvet"),
            ("ca", "Illa Bouvet"),
            ("ce", "Бувен гӀайре"),
            ("ch", "Bouvet Island"),
            ("cs", "Bouvetův ostrov"),
            ("cv", "Бувен гӀайре"),
            ("cy", "Ynys Bouvet"),
            ("da", "Bouvet-øen"),
            ("de", "Bouvet-Insel"),
            ("dv", "Bouvet Island"),
            (
                "dz",
                "བའ\u{f74}་བ\u{f7a}ཊ\u{f72}་ ཨའ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།",
            ),
            ("ee", "Bouvet Island"),
            ("el", "Νήσος Μπουβέ"),
            ("en", "Bouvet Island"),
            ("eo", "Buvetinsulo"),
            ("es", "Isla Bouvet"),
            ("et", "Bouvet' saar"),
            ("eu", "Bouvet uhartea"),
            ("fa", "جزیره\u{654} بووت"),
            ("ff", "Bouvet Island"),
            ("fi", "Bouvet'nsaari"),
            ("fo", "Bouvet Island"),
            ("fr", "île Bouvet"),
            ("fy", "Bouveteilân"),
            ("ga", "Oileán Bouvet"),
            ("gl", "Illa Bouvet"),
            ("gn", "Bouvet Island"),
            ("gu", "બોઉવ\u{ac7}ટ ટાપ\u{ac1}ઓ"),
            ("gv", "Bouvet Island"),
            ("ha", "Bouvet Island"),
            ("he", "בובה"),
            ("hi", "बोउव\u{947}ट आइल\u{948}\u{902}ड"),
            ("hr", "Otok Bouvet"),
            ("ht", "Bouvet Island"),
            ("hu", "Bouvet-sziget"),
            ("hy", "Բուվետ Կղզի"),
            ("ia", "Insula Bouvet"),
            ("id", "Pulau Bouvet"),
            ("io", "Bouvet Insulo"),
            ("is", "Bouveteyja"),
            ("it", "Isola Bouvet"),
            ("iu", "Bouvet Island"),
            ("ja", "ブーベ島"),
            ("ka", "ბუვეტის კუნძული"),
            ("ki", "Bouvet Island"),
            ("kk", "Буве аралдары"),
            ("kl", "Bouvet Island"),
            ("km", "កោះ\u{200b}ប\u{17ca}\u{17bc}វ\u{17c9}ែ"),
            ("kn", "ಬವೇಟ\u{ccd} ದ\u{ccd}ವೀಪ"),
            ("ko", "부베 섬"),
            ("ku", "Girava Boyuvet"),
            ("kv", "Bouvet Island"),
            ("kw", "Bouvet Island"),
            ("ky", "Буве аралы"),
            ("lo", "Bouvet Island"),
            ("lt", "Buvė sala"),
            ("lv", "Buvē Sala"),
            ("mi", "Bouvet Island"),
            ("mk", "Бувет острови"),
            ("ml", "ബൌവെ ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}"),
            ("mn", "Буве арал"),
            ("mr", "बोव\u{94d}ह\u{947} आयल\u{945}\u{902}ड"),
            ("ms", "Kepulauan Bouvet"),
            ("mt", "Gżira ta' Bouvet"),
            ("my", "Bouvet Island"),
            ("na", "Bouvet Island"),
            ("nb", "Bouvetøya"),
            ("ne", "बोउव\u{947}ट टाप\u{941}"),
            ("nl", "Bouveteiland"),
            ("nn", "Bouvetøya"),
            ("nv", "Bouvet Island"),
            ("oc", "Illa Bouvet"),
            ("or", "ବ\u{b41}ଭେଟ\u{b4d} ଦ\u{b4d}ବୀପ"),
            ("pa", "ਬ\u{a42}ਟਵਟ ਟਾਪ\u{a42}"),
            ("pi", "Bouvet Island"),
            ("pl", "Wyspa Bouveta"),
            ("ps", "Bouvet Island"),
            ("pt", "Ilha Bouvet"),
            ("pt_BR", "Ilha Bouvet"),
            ("ro", "Insula Bouvet"),
            ("ru", "Остров Буве"),
            ("rw", "Ikirwa cya Bouve"),
            ("sc", "Ìsula Bouvet"),
            ("sd", "Bouvet Island"),
            ("si", "බෝවෙට\u{dca} ද\u{dd6}පත"),
            ("sk", "Bouvetov ostrov"),
            ("sl", "Bouvetov otok"),
            ("so", "Bouvet Island"),
            ("sq", "Ishulli Buve"),
            ("sr", "Острво Бов"),
            ("sv", "Bouvetön"),
            ("sw", "Kisiwa cha Bouvet"),
            ("ta", "போவெட\u{bcd} த\u{bc0}வு"),
            ("te", "బ\u{c4b}వ\u{c4d}హ\u{c47} ఐల\u{c3e}ండ\u{c4d}"),
            ("tg", "Ҷазираи Бувит"),
            ("th", "เกาะบ\u{e39}เวต\u{e4c}"),
            ("ti", "Bouvet Island"),
            ("tk", "Bouvet Island"),
            ("tl", "Islang Bouvet"),
            ("tr", "Bouvet Adası"),
            ("tt", "Боувет Утравы"),
            ("ug", "بۇۋېت ئارىلى"),
            ("uk", "Острів Буве"),
            ("ur", "جزیرہ بووہ"),
            ("uz", "Bouvet Island"),
            ("ve", "Bouvet Island"),
            ("vi", "Quần đảo Bu-vê"),
            ("wa", "Iye Bouvet"),
            ("wo", "Dunu Buwet"),
            ("xh", "Bouvet Island"),
            ("yo", "Erékùṣù Bouvet"),
            ("zh_CN", "布维群岛"),
            ("zh_HK", "波維特島"),
            ("zh_TW", "布威島"),
            ("zu", "Bouvet Island"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
