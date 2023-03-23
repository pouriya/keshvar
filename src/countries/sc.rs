// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Seychelles

#[cfg(all(feature = "sc", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::SC;
    pub const ALPHA3: Alpha3 = Alpha3::SYC;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 248;
    pub const CURRENCY_CODE: &str = "SCR";
    pub const GEC: Option<GEC> = Some(GEC::SE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("SEY");
    pub const ISO_SHORT_NAME: &str = "Seychelles";
    pub const ISO_LONG_NAME: &str = "The Republic of Seychelles";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Seychellois");
    pub const NUMBER: &str = "690";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "SC";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Seychelles", "Seychellen", "セーシェル"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Seychelles"),
        ("af", "Seychelle"),
        ("ak", "Seychelles"),
        ("am", "ሲሸልስ"),
        ("an", "Seychelles"),
        ("ar", "الس\u{651}يشل"),
        ("as", "ছেয়চেলছ"),
        ("ay", "Seychelles"),
        ("az", "Seyşel"),
        ("ba", "Seychelles"),
        ("be", "Сейшэльскія Астравы"),
        ("bg", "Сейшели"),
        ("bi", "Seychelles"),
        ("bn", "সেশেলস"),
        ("bn_IN", "সেশেলস"),
        ("br", "Sechelez"),
        ("bs", "Sejšelsko otočje"),
        ("ca", "Seychelles"),
        ("ce", "Сейшелан гlайреш"),
        ("ch", "Seychelles"),
        ("cs", "Seychely"),
        ("cv", "Сейшелан гlайреш"),
        ("cy", "Seychelles"),
        ("da", "Seychellerne"),
        ("de", "Seychellen"),
        ("dv", "ތ\u{7ad}ނ\u{7b0}ގ\u{7a6}ދ\u{7a9}ބ\u{7aa}"),
        ("dz", "ས\u{f72}་ཅ\u{f72}་ལ\u{f7a}ས\u{f72}།"),
        ("ee", "Seychelles"),
        ("el", "Σεϋχέλλες"),
        ("en", "Seychelles"),
        ("eo", "Sejŝeloj"),
        ("es", "Seychelles"),
        ("et", "Seišellid"),
        ("eu", "Seychelleak"),
        ("fa", "سیشل"),
        ("ff", "Seychelles"),
        ("fi", "Seychellit"),
        ("fo", "Seyskelloyggjarnar"),
        ("fr", "Seychelles"),
        ("fy", "Seysjellen"),
        ("ga", "Na Séiséil"),
        ("gl", "Seichelles"),
        ("gn", "Seychelles"),
        ("gu", "સ\u{ac7}શ\u{ac7}લ\u{acd}સ"),
        ("gv", "Ny h-Ellanyn Heshell"),
        ("ha", "Seychelles"),
        ("he", "סיישל"),
        ("hi", "स\u{947}श\u{947}ल\u{94d}स"),
        ("hr", "Sejšeli"),
        ("ht", "Sechèl"),
        ("hu", "Seychelle-szigetek"),
        ("hy", "Սեյշելներ"),
        ("ia", "Seychelles"),
        ("id", "Seychelles"),
        ("io", "Seycheli"),
        ("is", "Seychelles-eyjar"),
        ("it", "Seychelles"),
        ("iu", "Seychelles"),
        ("ja", "セーシェル"),
        ("ka", "სეიშელის კუნძულები"),
        ("ki", "Seychelles"),
        ("kk", "Сейшел аралдары"),
        ("kl", "Seychelles"),
        ("km", "ស\u{17b8}ស\u{17d2}ហែល"),
        ("kn", "ಸೇಷಲ\u{ccd}ಸ\u{ccd}"),
        ("ko", "세이셸"),
        ("ku", "Seyselan"),
        ("kv", "Seychelles"),
        ("kw", "Seychellys"),
        ("ky", "Сейшел аралдары"),
        ("lo", "Seychelles"),
        ("lt", "Seišeliai"),
        ("lv", "Seišelas"),
        ("mi", "Seychelles"),
        ("mk", "Сејшели"),
        ("ml", "സെയ\u{d4d}ഷെല\u{d4d}\u{200d}സ\u{d4d}"),
        ("mn", "Seychelles"),
        ("mr", "स\u{947}श\u{947}ल\u{94d}स"),
        ("ms", "Seychelles"),
        ("mt", "Seychelles"),
        (
            "my",
            "ဆေးရ\u{103e}\u{1032}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Seychelles"),
        ("nb", "Seychellene"),
        ("ne", "सिच\u{947}ल\u{94d}लिस"),
        ("nl", "Seychellen"),
        ("nn", "Seychellane"),
        ("nv", "Seychelles"),
        ("oc", "Seichèlas"),
        ("or", "ସେଶେଲସ\u{b4d}"),
        ("pa", "ਸੀਆਚਿਲਸ"),
        ("pi", "स\u{947}श\u{947}ल"),
        ("pl", "Seszele"),
        ("ps", "سیشل"),
        ("pt", "Seychelles"),
        ("pt_BR", "Seychelles"),
        ("ro", "Seychelles"),
        ("ru", "Сейшелы"),
        ("rw", "Seyishele"),
        ("sc", "Seychelles"),
        ("sd", "Seychelles"),
        ("si", "ස\u{dd3}ශෙල\u{dca}ස\u{dca}"),
        ("sk", "Seychely"),
        ("sl", "Sejšeli"),
        ("so", "Seychelles"),
        ("sq", "Sejshelle"),
        ("sr", "Сејшели"),
        ("sv", "Seychellerna"),
        ("sw", "Seychelles"),
        ("ta", "ச\u{bc0}செல\u{bcd}சு"),
        ("te", "స\u{c47}శ\u{c47}ల\u{c4d}స\u{c4d}"),
        ("tg", "Сейшел"),
        ("th", "เซเชลส\u{e4c}"),
        ("ti", "Seychelles"),
        ("tk", "Seýşel Adalary"),
        ("tl", "Seychelles"),
        ("tr", "Seyşeller"),
        ("tt", "Сейшелләр"),
        ("ug", "سېيشېل"),
        ("uk", "Сейшели"),
        ("ur", "سیچیلیس"),
        ("uz", "Seyshell orollari"),
        ("ve", "Seychelles"),
        ("vi", "Xây-sen"),
        ("wa", "Seycheles"),
        ("wo", "Seychelles"),
        ("xh", "Seychelles"),
        ("yo", "Ṣèíhẹ\u{301}lẹ\u{301}sì"),
        ("zh_CN", "塞舌尔"),
        ("zh_HK", "塞舌爾"),
        ("zh_TW", "塞席爾"),
        ("zu", "IsiSeyisheli"),
    ];
    #[cfg(all(feature = "sc", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -4.679574;
        pub const LONGITUDE: f64 = 55.491977;
        pub const MAX_LATITUDE: f64 = -3.7091721;
        pub const MAX_LONGITUDE: f64 = 56.3928224;
        pub const MIN_LATITUDE: f64 = -10.4716073;
        pub const MIN_LONGITUDE: f64 = 45.9832764;
        pub const NORTHEAST_LATITUDE: f64 = -3.7091721;
        pub const NORTHEAST_LONGITUDE: f64 = 56.3928224;
        pub const SOUTHWEST_LATITUDE: f64 = -10.4716073;
        pub const SOUTHWEST_LONGITUDE: f64 = 45.9832764;
    }
}
#[cfg(all(feature = "sc", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -4.679574,
            longitude: 55.491977,
            max_latitude: -3.7091721,
            max_longitude: 56.3928224,
            min_latitude: -10.4716073,
            min_longitude: 45.9832764,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -3.7091721,
                    longitude: 56.3928224,
                },
                southwest: CountryGeoBound {
                    latitude: -10.4716073,
                    longitude: 45.9832764,
                },
            },
        }
    }
}

#[cfg(all(feature = "sc", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::SC,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6900443), longitude: Some(55.5150289), max_latitude: Some(-4.685153), min_latitude: Some(-4.698596999999999), max_longitude: Some(55.5228579), min_longitude: Some(55.502337)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آنسي-أوكس-بينس"), ("bn", "আন\u{9cd}স- আউস পিন\u{9cd}স"), ("ca", "Anse aux Pins"), ("ccp", "𑄃𑄚\u{11134}𑄥𑄬 𑄃\u{11127}𑄇\u{11134} 𑄛\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Anse aux Pins"), ("da", "Anse-aux-Pins"), ("de", "Anse aux Pins"), ("el", "Ανσέ-ο-Πινς"), ("en", "Anse aux Pins"), ("es", "Anse aux Pins"), ("fi", "Anse-aux-Pins"), ("fr", "Anse aux Pins"), ("gu", "એનસ-ઑક\u{acd}સ-પિન\u{acd}સ"), ("hi", "ए\u{902}स\u{947}-ऑक\u{94d}स-पिन\u{94d}स"), ("id", "Anse aux Pins"), ("it", "Anse aux Pins"), ("jv", "Anse aux Pins"), ("ka", "ანს-ო-პენი"), ("kn", "ಆನ\u{ccd}ಸ\u{cc6}-ಆಕ\u{ccd}ಸ\u{ccd}-ಪ\u{cbf}ನ\u{ccd}ಸ\u{ccd}"), ("ko", "앙스오팽 구"), ("lt", "Anse-o-pinsas"), ("lv", "Ansopina"), ("mr", "अ\u{902}स-ऑक\u{94d}स-पिन"), ("ms", "Anse-aux-Pins"), ("nb", "Anse aux Pins"), ("nl", "Anse aux Pins"), ("no", "Anse aux Pins"), ("pl", "Anse aux Pins"), ("pt", "Anse-aux-Pins"), ("ro", "Districtul Anse aux Pins"), ("ru", "Анс-о-Пен"), ("si", "අන\u{dca}සේ-ඔක\u{dca}ස\u{dca}-ප\u{dd2}න\u{dca}ස\u{dca}"), ("sv", "Anse aux Pins"), ("ta", "அன\u{bcd}ஸே-ஆஸ\u{bcd}-பின\u{bcd}ஸ\u{bcd}"), ("te", "ఆన\u{c4d}స\u{c46}\u{c4d}స\u{c4d}-ఆక\u{c4d}స\u{c4d}-ప\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "พอร\u{e4c}โต โนโว"), ("tr", "Anse-aux-Pins"), ("uk", "Анс-о-Пен"), ("ur", "انسی-اوز-پینس"), ("vi", "Anse-aux-Pins"), ("zh", "安塞奧潘區")]),
                        unofficial_name_list: ["Anse aux Pins"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::SC,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.7047268), longitude: Some(55.4859363), max_latitude: Some(-4.6840339), min_latitude: Some(-4.7371169), max_longitude: Some(55.50535), min_longitude: Some(55.4748002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أنس بويلاو"), ("bn", "আন\u{9cd}স ব\u{9c1}ইল\u{9c1}"), ("ca", "Anse Boileau"), ("ccp", "𑄃𑄚\u{11134}𑄥𑄬 𑄝\u{11130}𑄣\u{11128}𑄅\u{1112a}"), ("ceb", "Anse Boileau"), ("da", "Anse Boileau"), ("de", "Anse Boileau"), ("el", "Άνσε Μποϊλό"), ("en", "Anse Boileau"), ("es", "Anse Boileau"), ("fi", "Anse Boileau"), ("fr", "Anse Boileau"), ("gu", "એન\u{acd}સ\u{ac7} બોઇલ\u{ac7}ઉ"), ("hi", "ए\u{902}स बॉइलो"), ("id", "Anse Boileau"), ("it", "Anse Boileau"), ("ja", "アンセ・ボイレヤー"), ("jv", "Anse Boileau"), ("ka", "ანს-ბუალო"), ("kn", "ಆನ\u{ccd}ಸ\u{ccd} ಬೋಲ\u{cbf}ಯ\u{ccc}"), ("ko", "앙스부알로 구"), ("lt", "Anse Bualo"), ("lv", "Ansebualo"), ("mr", "एनस\u{947} बोइल\u{94d}य\u{942}"), ("ms", "Anse Boileau"), ("nb", "Anse Boileau"), ("nl", "Anse Boileau"), ("no", "Anse Boileau"), ("pl", "Anse Boileau"), ("pt", "Anse Boileau"), ("ro", "Districtul Anse Boileau"), ("ru", "Анс-Буало"), ("si", "අන\u{dca}සේ බෝය\u{dd2}ල\u{dd2}යව\u{dd6}"), ("sv", "Anse Boileau"), ("ta", "அன\u{bcd}ஸே போயிலே"), ("te", "ఆన\u{c4d}స\u{c4d} బ\u{c3e}య\u{c3f}ల\u{c3e}"), ("th", "แอนซ\u{e35}\u{e48} บอยเล\u{e35}ยว"), ("tr", "Anse Boileau"), ("uk", "Анс-Буало"), ("ur", "انسے بویلیاو"), ("vi", "Anse Boileau"), ("zh", "安塞布瓦洛區")]),
                        unofficial_name_list: ["Anse Boileau"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::SC,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.5962834), longitude: Some(55.4499303), max_latitude: Some(-4.581683), min_latitude: Some(-4.609877), max_longitude: Some(55.462322), min_longitude: Some(55.43989500000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آنسي إيتوال"), ("bn", "আন\u{9cd}সে ইত\u{9c1}য\u{9bc}\u{9be}ইলে"), ("ca", "Anse Etoile"), ("ccp", "𑄃𑄚\u{11134}𑄥𑄬 𑄃\u{11128}𑄑\u{11130}𑄣\u{11134}"), ("ceb", "Anse Etoile"), ("da", "Anse Etoile"), ("de", "Anse Etoile"), ("el", "Άνσε Ετόιλ"), ("en", "Anse Etoile"), ("es", "Anse Etoile"), ("fi", "Anse Etoile"), ("fr", "Anse Etoile"), ("gu", "એનસ\u{ac7} ઇટોઇલ"), ("hi", "एन\u{94d}स\u{947} इतोइल"), ("id", "Anse Etoile"), ("it", "Anse Etoile"), ("ja", "アンセ・エチオエ"), ("jv", "Anse Etoile"), ("ka", "ანს-ეტუალი"), ("kn", "ಅನ\u{ccd}ಸ\u{ccd} ಎಟೈಲ\u{ccd}"), ("ko", "앙스에투알 구"), ("lt", "Anse Etolė"), ("lv", "Ansetolē"), ("mr", "एनस\u{947} इतोइल"), ("ms", "Anse Etoile"), ("nb", "Anse Etoile"), ("nl", "Anse Etoile"), ("no", "Anse Etoile"), ("pl", "Anse Etoile"), ("pt", "Anse Etoile"), ("ro", "Districtul Anse Etoile"), ("ru", "Анс-Этуаль"), ("si", "අන\u{dca}සේ එටෝය\u{dd2}ලේ"), ("sv", "Anse Etoile"), ("ta", "அனஸ\u{bcd} ஏடொய\u{bcd}லெ"), ("te", "ఆన\u{c4d}స\u{c46} ఇట\u{c4b}య\u{c3f}ల\u{c4d}"), ("th", "ช\u{e34}นยานกา"), ("tr", "Anse Etoile"), ("uk", "Анс-Етуаль"), ("ur", "انسے یتویلی"), ("vi", "Anse Etoile"), ("zh", "安塞艾托瓦區")]),
                        unofficial_name_list: ["Anse Étoile"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::SC,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.7189825), longitude: Some(55.4783184), max_latitude: Some(-4.7165001), min_latitude: Some(-4.7208305), max_longitude: Some(55.4798841), min_longitude: Some(55.47635450000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أو كاب"), ("bn", "অ ক\u{9cd}য\u{9be}প"), ("ca", "Au Cap"), ("ccp", "𑄃\u{11127}𑄅\u{1112a} 𑄇\u{11133}𑄠𑄛\u{11134}"), ("ceb", "Au Cap"), ("da", "Au Cap"), ("de", "Au Cap"), ("el", "Ο Καπ"), ("en", "Au Cap"), ("es", "Au Cap"), ("fi", "Au Cap"), ("fr", "Au Cap"), ("gu", "ઑ ક\u{ac7}પ"), ("hi", "ऑ क\u{948}प"), ("id", "Au Cap"), ("it", "Au Cap"), ("ja", "アウ・カップ"), ("ka", "ო-კაპი"), ("kn", "ಔ ಕ\u{ccd}ಯಾಪ\u{ccd}"), ("ko", "오카프 구"), ("lt", "Au Kapas"), ("lv", "Aukapa"), ("mr", "ऑ क\u{945}प"), ("ms", "Au Cap"), ("nb", "Au Cap"), ("nl", "Au Cap"), ("no", "Au Cap"), ("pl", "Anse Louis"), ("pt", "Au cap"), ("ro", "Districtul Au Cap"), ("ru", "О-Кап"), ("si", "ඖ කප\u{dca}"), ("sv", "Au Cap"), ("ta", "ஆ க\u{bbe}ப\u{bcd}"), ("te", "ఔ క\u{c4d}య\u{c3e}ప\u{c4d}"), ("th", "โอแคป"), ("tr", "Au Cap"), ("uk", "О-Кап"), ("ur", "او کیپ"), ("vi", "Au Cap"), ("zh", "奧凱普區")]),
                        unofficial_name_list: ["Anse Louis"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::SC,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.733333), longitude: Some(55.51666700000001), max_latitude: Some(-4.7266898), min_latitude: Some(-4.75697), max_longitude: Some(55.52538509999999), min_longitude: Some(55.4960203)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أنس رويال"), ("bn", "আন\u{9cd}স রয\u{9bc}েল"), ("ca", "Anse Royale"), ("ccp", "𑄃𑄚\u{11134}𑄥𑄬 𑄢\u{1112e}𑄠𑄣\u{11134}"), ("ceb", "Anse Royale"), ("da", "Anse Royale"), ("de", "Anse Royale"), ("el", "Άνσε Ροαγιάλ"), ("en", "Anse Royale"), ("es", "Anse Royale"), ("fi", "Anse Royale"), ("fr", "Anse Royale"), ("gu", "એ\u{a82}સ\u{ac7} રોયાલ"), ("hi", "ए\u{902}स रोयाल"), ("id", "Anse Royale"), ("it", "Anse Royale"), ("ja", "アンセ・ロイヤル"), ("ka", "ანს-რუაიალი"), ("kn", "ಆನ\u{ccd}ಸ\u{cc6} ರಾಯೇಲ\u{ccd}"), ("ko", "앙스루아얄 구"), ("lt", "Anse Rojal"), ("lv", "Ansurualē"), ("mr", "अ\u{902}स रोयाल"), ("ms", "Anse Royale"), ("nb", "Anse Royale"), ("nl", "Anse Royale"), ("no", "Anse Royale"), ("pl", "Anse Royale"), ("pt", "Anse Royale"), ("ro", "Districtul Anse Royale"), ("ru", "Анс-Рояль"), ("si", "අන\u{dca}සේ රෝයලේ"), ("sv", "Anse Royale"), ("ta", "அன\u{bcd}ஸே றோயல\u{bcd}"), ("te", "ఆన\u{c4d}స\u{c4d} ర\u{c3e}య\u{c47}ల\u{c4d}"), ("th", "แอนซ\u{e35}\u{e48} รอย\u{e31}ล"), ("tr", "Anse Royale"), ("uk", "Анс-Рояль"), ("ur", "انسے رویالی"), ("vi", "Anse Royale"), ("zh", "安塞羅亞萊區")]),
                        unofficial_name_list: ["Anse Royale"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::SC,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.7482525), longitude: Some(55.4859363), max_latitude: Some(-4.7334042), min_latitude: Some(-4.766401), max_longitude: Some(55.509342), min_longitude: Some(55.464302)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باي لازاري"), ("bn", "ব\u{9be}ইয\u{9bc}ে ল\u{9be}জ\u{9be}র"), ("ca", "Baie Lazare"), ("ccp", "𑄝\u{1112d} 𑄣𑄎𑄢\u{11134}"), ("ceb", "Baie Lazare"), ("da", "Baie Lazare"), ("de", "Baie Lazare"), ("el", "Μπάιε"), ("en", "Baie Lazare"), ("es", "Baie Lazare"), ("fi", "Baie Lazare"), ("fr", "Baie Lazare"), ("gu", "બ\u{ac7} લઝાર\u{ac7}"), ("hi", "बाए लाज\u{93c}र\u{947}"), ("id", "Baie Lazare"), ("it", "Baie Lazare"), ("ja", "バイエ・ロザーレ"), ("ka", "ბე-ლაზარი"), ("kn", "ಬೈಯ\u{cbf} ಲಜಾರ\u{cc6}"), ("ko", "베라자르 구"), ("lt", "Be Lazarė"), ("lv", "Belazare"), ("mr", "बाईए लाझर\u{947}"), ("ms", "Baie Lazare"), ("nb", "Baie Lazare"), ("nl", "Baie Lazare"), ("no", "Baie Lazare"), ("pl", "Baie Lazare"), ("pt", "Baie Lazare"), ("ro", "Districtul Baie Lazare"), ("ru", "Бэ-Лазар"), ("si", "බෛයෙ ලස\u{dcf}රේ"), ("sv", "Baie Lazare"), ("ta", "பையே லச\u{bbe}ரே"), ("te", "బ\u{c47}య\u{c4d} ల\u{c3e}జ\u{c3e}ర\u{c4d}"), ("th", "เบย\u{e4c} ราแซย\u{e4c}"), ("tr", "Baie Lazare"), ("uk", "Бе-Лазар"), ("ur", "بایی لازاری"), ("vi", "Baie Lazare"), ("zh", "貝拉扎爾區")]),
                        unofficial_name_list: ["Baie Lazare"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::SC,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.3193277), longitude: Some(55.74116919999999), max_latitude: Some(-4.276453099999999), min_latitude: Some(-4.352547899999999), max_longitude: Some(55.7889993), min_longitude: Some(55.68746609999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باي سينت آن"), ("bn", "ব\u{9be}ইয\u{9bc}ে সেন\u{9cd}টে আন\u{9cd}যে"), ("ca", "Baie Sainte Anne"), ("ccp", "𑄝\u{1112d} 𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄃𑄬𑄚\u{11128}"), ("ceb", "Baie Sainte Anne"), ("da", "Baie Sainte Anne"), ("de", "Baie Sainte Anne"), ("el", "Μπάε Σάιντε Άννι"), ("en", "Baie Sainte Anne"), ("es", "Baie Sainte Anne"), ("fi", "Baie Sainte Anne"), ("fr", "Baie Sainte Anne"), ("gu", "બ\u{ac7}ઈ સ\u{ac7}ઇન\u{acd}ટ એન"), ("hi", "बाई स\u{948}न\u{94d}त\u{947} ऐन\u{947}"), ("id", "Baie Sainte Anne"), ("it", "Baie Sainte Anne"), ("ja", "バイエ・セント・アンセ"), ("ka", "ბე-სენტ-ანი"), ("kn", "ಬೈಯ\u{cbf} ಸೇಂಟ\u{ccd} ಅನ\u{ccd}ನ\u{cbf}"), ("ko", "베생트안 구"), ("lt", "Baje Saint Ana"), ("lv", "Besentanne"), ("mr", "ब\u{947}ई स\u{947}\u{902}ट ऍन"), ("ms", "Baie Sainte Anne"), ("nb", "Baie Sainte Anne"), ("nl", "Baie Sainte Anne"), ("no", "Baie Sainte Anne"), ("pl", "Baie Sainte Anne"), ("pt", "Baie Ste. Anne"), ("ro", "Districtul Baie Sainte Anne"), ("ru", "Бе-Сент-Анн"), ("si", "බයේ ශ\u{dcf}න\u{dca}ත ආන\u{dcf}"), ("sv", "Baie Sainte Anne"), ("ta", "பைஈ செயின\u{bcd}ட\u{bcd} அன\u{bcd}னே"), ("te", "బ\u{c46}య\u{c4d} స\u{c46}య\u{c3f}ంట\u{c4d} ఆన\u{c4d}"), ("th", "เบเซนต\u{e4c}แอนน\u{e4c}"), ("tr", "Baie Sainte Anne"), ("uk", "Бе-Сент-Анн"), ("ur", "بایی ساینتی آنے"), ("vi", "Baie Sainte Anne"), ("zh", "貝聖安那區")]),
                        unofficial_name_list: ["Baie Sainte Anne"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::SC,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6210967), longitude: Some(55.4277802), max_latitude: Some(-4.6111603), min_latitude: Some(-4.627564899999999), max_longitude: Some(55.43984409999999), min_longitude: Some(55.4202533)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيو فالون"), ("bn", "বিউ ভ\u{9cd}য\u{9be}লোন"), ("ca", "Beau Vallon"), ("ccp", "𑄝\u{11128}𑄅\u{1112a} 𑄞𑄣\u{1112e}𑄚\u{11134}"), ("ceb", "Beau Vallon"), ("da", "Beau Vallon"), ("de", "Beau Vallon"), ("el", "Μπο Βαλόν"), ("en", "Beau Vallon"), ("es", "Beau Vallon"), ("fi", "Beau Vallon"), ("fr", "Beau Vallon (Seychelles)"), ("gu", "બ\u{acd}ય\u{ac1} વ\u{ac7}લોન"), ("hi", "ब\u{94d}य\u{942} वलोन"), ("id", "Beau Vallon"), ("it", "Beau Vallon"), ("ja", "ベウ・バロン"), ("ka", "ბო-ვალონი"), ("kn", "ಬ\u{ccd}ಯ\u{cc2} ವಾಲ\u{ccd}ಲೊನ\u{ccd}"), ("ko", "보발롱 구"), ("lt", "Bo Valonas"), ("lv", "Bevalonna"), ("mr", "ब\u{947}अउ व\u{947}ल\u{94d}लोण"), ("ms", "Beau Vallon"), ("nb", "Beau Vallon"), ("nl", "Beau Vallon"), ("no", "Beau Vallon"), ("pl", "Beau Vallon"), ("pt", "Beau Vallon"), ("ro", "Districtul Beau Vallon"), ("ru", "Бо-Валлон"), ("si", "බ\u{dd2}ය\u{dd4} වැලෝන\u{dca}"), ("sv", "Beau Vallon"), ("ta", "பியூ வல\u{bcd}லோன\u{bcd}"), ("te", "బ\u{c4d}యూ వ\u{c3e}లన\u{c4d}"), ("th", "บ\u{e34}วว\u{e31}ลลอน"), ("tr", "Beau Vallon"), ("uk", "Бо-Валон"), ("ur", "بیو والون"), ("vi", "Beau Vallon"), ("zh", "博瓦隆區")]),
                        unofficial_name_list: ["Beau Vallon"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::SC,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6440325), longitude: Some(55.4499303), max_latitude: Some(-4.619837), min_latitude: Some(-4.657465), max_longitude: Some(55.46115529999999), min_longitude: Some(55.4380879)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيل اير"), ("bn", "বেল এয\u{9bc}\u{9be}র"), ("ca", "Bel Air (Districte)"), ("ccp", "𑄝𑄬𑄣\u{11134} 𑄃\u{11128}𑄠𑄢\u{11134}"), ("da", "Bel Air"), ("de", "Bel Air"), ("el", "Μπελ Ερ"), ("en", "Bel Air"), ("es", "Bel Air (Seychelles)"), ("fi", "Bel Air"), ("fr", "Bel Air"), ("gu", "બ\u{ac7}લ એર"), ("hi", "ब\u{947}ल एयर"), ("id", "Bel Air, Seychelles"), ("it", "Bel Air"), ("ja", "ベル・エアー"), ("jv", "Bel Air, Syechelles"), ("ka", "ბელ-ერი"), ("kn", "ಬ\u{cc6}ಲ\u{ccd} ಏ\u{ccd}"), ("ko", "벨에르 구"), ("lt", "Bel Eiras"), ("lv", "Belaira"), ("mr", "ब\u{947}ल एयर"), ("ms", "Bel Air, Seychelles"), ("nb", "Bel Air"), ("nl", "Bel Air"), ("no", "Bel Air"), ("pl", "Bel Air (Seszele)"), ("pt", "Bel Air (Seychelles)"), ("ro", "Districtul Bel Air"), ("ru", "Бель-Эйр"), ("si", "බෙල\u{dca} එය\u{dcf}ර\u{dca}"), ("sv", "Bel Air, Seychellerna"), ("ta", "பெல\u{bcd} ஏர\u{bcd}"), ("te", "బ\u{c46}ల\u{c4d} ఎయ\u{c3f}ర\u{c4d}"), ("th", "เบล แอร\u{e4c}"), ("tr", "Bel Air"), ("uk", "Белер"), ("ur", "بل ایئر"), ("vi", "Bel Air"), ("zh", "貝爾艾爾區")]),
                        unofficial_name_list: ["Bel Air"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::SC,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6204838), longitude: Some(55.4167072), max_latitude: Some(-4.6146267), min_latitude: Some(-4.6257684), max_longitude: Some(55.4232574), min_longitude: Some(55.3988171)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيل أومبير"), ("bn", "বেল ওম\u{9cd}ব\u{9cd}রে"), ("ca", "Bel Ombre"), ("ccp", "𑄝𑄬𑄣\u{11134} 𑄃\u{1112e}𑄟\u{11134}𑄝\u{11133}𑄢𑄬"), ("da", "Bel Ombre"), ("de", "Bel Ombre"), ("el", "Μπελ Όμπρε"), ("en", "Bel Ombre"), ("es", "Bel Ombre"), ("fi", "Bel Ombre"), ("fr", "Bel Ombre"), ("gu", "બ\u{ac7}લ ઓમ\u{acd}બ\u{acd}ર\u{ac7}"), ("hi", "ब\u{947}ल ओम\u{94d}बर\u{947}"), ("id", "Bel Ombre"), ("it", "Bel Ombre"), ("ja", "ベル・オンブレ"), ("jv", "Bel Ombre"), ("ka", "ბელ-ომბრი"), ("kn", "ಬ\u{cc6}ಲ\u{ccd} ಓಮ\u{ccd}ಬ\u{ccd}ರ\u{cc6}"), ("ko", "벨옴브르 구"), ("lt", "Bel Ombrė"), ("lv", "Belombre"), ("mr", "ब\u{947}ल ओम\u{94d}ब\u{947}र"), ("ms", "Bel Ombre"), ("nb", "Bel Ombre"), ("nl", "Bel Ombre"), ("no", "Bel Ombre"), ("pl", "Belombre"), ("pt", "Bel Ombre"), ("ro", "Districtul Bel Ombre"), ("ru", "Бель-Омбр"), ("si", "බෙල\u{dca} ඕම\u{dca}බ\u{dca}රේ"), ("sv", "Belombre"), ("ta", "பெல\u{bcd} ஒம\u{bcd}பரே"), ("te", "బ\u{c46}ల\u{c4d} ఓంబ\u{c4d}ర\u{c46}"), ("th", "เบล อมบร\u{e35}"), ("tr", "Bel Ombre"), ("uk", "Бел-Омбре"), ("ur", "بل ومبری"), ("vi", "Bel Ombre"), ("zh", "貝爾翁布雷區")]),
                        unofficial_name_list: ["Bel Ombre"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::SC,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.670554099999999), longitude: Some(55.49701810000001), max_latitude: Some(-4.650223500000001), min_latitude: Some(-4.6916291), max_longitude: Some(55.511308), min_longitude: Some(55.4701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاسكاد"), ("bn", "ক\u{9cd}য\u{9be}স\u{9cd}কেড"), ("ca", "Cascade"), ("ccp", "𑄇𑄌\u{11134}𑄇𑄖\u{11134}"), ("ceb", "Cascade"), ("da", "Cascade"), ("de", "Cascade"), ("el", "Κασκάντε"), ("en", "Cascade"), ("es", "Cascade"), ("fi", "Cascade"), ("fr", "Cascade"), ("gu", "કાસાક\u{ac7}ડ"), ("hi", "कास\u{94d}क\u{947}ड"), ("id", "Cascade, Seychelles"), ("it", "Cascade"), ("ja", "カスカドゥ"), ("ka", "კასკადი"), ("kn", "ಕ\u{ccd}ಯಾಸ\u{ccd}ಕೇಡ\u{ccd}"), ("ko", "카스카드 구"), ("lt", "Kaskada"), ("lv", "Kaskāde"), ("mr", "क\u{945}सक\u{947}ड"), ("ms", "Cascade"), ("nb", "Cascade"), ("nl", "Cascade"), ("no", "Cascade"), ("pl", "Cascade"), ("pt", "Cascade"), ("ro", "Districtul Cascade"), ("ru", "Каскад"), ("si", "කැස\u{dca}කෙඩ\u{dca}"), ("sv", "Cascade, Seychellerna"), ("ta", "க\u{bbe}ஸ\u{bcd}கேடே"), ("te", "క\u{c3e}స\u{c4d}క\u{c47}డ\u{c4d}"), ("th", "คาสเคด เซเชลเลส"), ("tr", "Cascade"), ("uk", "Каскад"), ("ur", "کاسکادی"), ("vi", "Cascade"), ("zh", "卡斯喀得區")]),
                        unofficial_name_list: ["Cascade"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::SC,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.5719729), longitude: Some(55.4416234), max_latitude: Some(-4.5638565), min_latitude: Some(-4.5966793), max_longitude: Some(55.462322), min_longitude: Some(55.4318189)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غلاسيس"), ("be", "Гласіс"), ("bn", "গ\u{9cd}ল\u{9be}চিচ"), ("ca", "Glacis"), ("ccp", "𑄉\u{11133}𑄣𑄌\u{11128}𑄌\u{11134}"), ("ceb", "Glacis"), ("da", "Glacis"), ("de", "Glacis"), ("el", "Γκλέισις"), ("en", "Glacis"), ("es", "Glacis"), ("fi", "Glacis"), ("fr", "Glacis"), ("gu", "ગ\u{acd}લ\u{ac7}સિસ"), ("hi", "ग\u{94d}ल\u{948}सिस"), ("id", "Glacis"), ("it", "Glacis"), ("ja", "グラシ"), ("ka", "გლასი"), ("kn", "ಗ\u{ccd}ಲೇಸ\u{cbf}ಸ\u{ccd}"), ("ko", "글라시 구"), ("lt", "Glasisas"), ("lv", "Glāsisa"), ("mr", "ग\u{94d}ल\u{945}क\u{94d}स"), ("ms", "Glacis"), ("nb", "Glacis"), ("nl", "Glacis"), ("no", "Glacis"), ("pl", "Glacis"), ("pt", "Glacis"), ("ro", "Districtul Glacis"), ("ru", "Гласис"), ("si", "ග\u{dca}ල\u{dcf}ස\u{dd2}ස\u{dca}"), ("sv", "Glacis"), ("ta", "க\u{bcd}ள\u{bbe}ஸிஸ\u{bcd}"), ("te", "గ\u{c4d}ల\u{c47}స\u{c3f}స\u{c4d}"), ("th", "กลาซ\u{e34}ส เซย\u{e4c}เชลเลส"), ("tr", "Glacis"), ("uk", "Гласі, Глясі"), ("ur", "جلاکیس"), ("vi", "Glacis"), ("zh", "格拉西斯區")]),
                        unofficial_name_list: ["Glacis"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::SC,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.677392), longitude: Some(55.463777), max_latitude: Some(-4.657465), min_latitude: Some(-4.7082592), max_longitude: Some(55.486098), min_longitude: Some(55.43838100000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Grand’ Anse"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄃𑄚\u{11134}𑄥𑄬 𑄟𑄦\u{11128}"), ("ceb", "Grand Anse Mahe"), ("de", "Grand Anse (Mahé)"), ("en", "Grand’Anse Mahé"), ("es", "Grand’ Anse"), ("id", "Grand’Anse Mahé"), ("it", "Grand’Anse Mahé"), ("ja", "グランド・アンセ・マーヘ"), ("ka", "გრანდ-ანსი"), ("ko", "그랑당스마에 구"), ("ms", "Grand’Anse Mahé"), ("nl", "Grand’ Anse"), ("pl", "Grand’ Anse"), ("ro", "Districtul Grand’Anse Mahé"), ("sv", "Grand’Anse Mahé"), ("zh", "馬埃大安塞區")]),
                        unofficial_name_list: ["Grand' Anse (Mahé)"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::SC,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.3176219), longitude: Some(55.7078363), max_latitude: Some(-4.292169599999999), min_latitude: Some(-4.3582979), max_longitude: Some(55.761684), min_longitude: Some(55.6446526)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غراند آنز براسلين"), ("bn", "গ\u{9cd}র\u{9be}ন\u{9cd}ড’আন\u{9cd}সে প\u{9cd}র\u{9be}স\u{9cd}লিন"), ("ca", "Grand’ Anse²"), ("ccp", "𑄉\u{11133}𑄢𑄚\u{11133}𑄓\u{11134} 𑄃𑄚\u{11134}𑄥𑄬 𑄛\u{11133}𑄢𑄌\u{11134}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Grand Anse Praslin"), ("da", "Grand’Anse Praslin"), ("de", "Grand Anse"), ("el", "Γκραντ Ανς Πρασλίν"), ("en", "Grand’Anse Praslin"), ("es", "Grand’ Anse (Praslin)"), ("fi", "Grand’Anse Praslin"), ("fr", "Grand’Anse"), ("gu", "ગ\u{acd}રાન\u{acd}ડ’એન\u{acd}સ પ\u{acd}ર\u{ac7}સ\u{acd}લિન"), ("hi", "ग\u{94d}र\u{948}\u{902}ड’अ\u{902}स प\u{94d}र\u{948}लिन"), ("id", "Grand’Anse Praslin"), ("it", "Grand’Anse Praslin"), ("ja", "グランド・アンセ・プラスリン"), ("ka", "გრანდ-ანსი²"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಂಡ\u{ccd}‘ಅನ\u{ccd}ಸ\u{cc6} ಪ\u{ccd}ರಸ\u{ccd}ಲ\u{cbf}ನ\u{ccd}"), ("ko", "그랑당스프라슬랭 구"), ("lt", "Grand Anse Praslinas"), ("lv", "Grandansepraslina"), ("mr", "ग\u{94d}र\u{901}ड’अन\u{94d}स प\u{94d}र\u{945}झिन"), ("ms", "Grand’Anse Praslin"), ("nb", "Grand Anse Praslin"), ("nl", "Grand’ Anse²"), ("no", "Grand Anse Praslin"), ("pl", "Grand’ Anse²"), ("pt", "Grand’Anse Praslin"), ("ro", "Districtul Grand’Anse Praslin"), ("ru", "Гранд-Анс"), ("si", "ග\u{dca}රෑන\u{dca}ඩ\u{dca}‘අන\u{dca}සේ ප\u{dca}\u{200d}රස\u{dca}ල\u{dd2}න\u{dca}"), ("sv", "Grand’Anse Praslin"), ("ta", "கிர\u{bbe}ண\u{bcd}ட\u{bcd} ‘அன\u{bcd}ஸ\u{bcd} பிரெஸ\u{bcd}லின\u{bcd}"), ("te", "గ\u{c4d}ర\u{c3e}ండ\u{c4d}ఆన\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}స\u{c4d}ల\u{c3f}న\u{c4d}"), ("th", "ไอกาอ\u{e34}เลไต"), ("tr", "Grand’Anse Praslin"), ("uk", "Гранд-Анс-Праслін"), ("vi", "Grand’Anse Praslin"), ("zh", "普拉蘭大安塞區")]),
                        unofficial_name_list: ["Grand' Anse (Praslin)"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::SC,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.3590972), longitude: Some(55.8412424), max_latitude: Some(-4.3370025), min_latitude: Some(-4.3831656), max_longitude: Some(55.8564263), min_longitude: Some(55.8242892)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا ديغو"), ("bg", "Диг"), ("ca", "La Digue"), ("ccp", "𑄣 𑄓\u{11128}𑄉\u{1112a}"), ("ceb", "La Digue"), ("cs", "La Digue"), ("de", "La Digue"), ("el", "Λα Ντιγκ"), ("en", "La Digue"), ("es", "La Digue"), ("et", "La Digue’i saar"), ("fr", "La Digue"), ("gl", "La Digue"), ("he", "לה דיג"), ("it", "La Digue"), ("ja", "ラ・ディーグ島"), ("ka", "ლა-დიგი"), ("ko", "라디그 섬"), ("lt", "La Digas"), ("nl", "La Digue"), ("pl", "La Digue"), ("pt", "La Digue"), ("ro", "Districtul La Digue"), ("ru", "Ла-Диг"), ("sk", "La Digue"), ("sv", "La Digue"), ("tr", "La Digue"), ("uk", "Ла-Діг"), ("zh", "拉迪格岛")]),
                        unofficial_name_list: ["La Digue"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::SC,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6149553), longitude: Some(55.4540841), max_latitude: Some(-4.6065289), min_latitude: Some(-4.624853), max_longitude: Some(55.45868609999999), min_longitude: Some(55.4484299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا ريفيير أنغليز"), ("bn", "ল\u{9be} রিভেরে এংল\u{9be}ইস"), ("ca", "La Riviere Anglaise"), ("ccp", "𑄣 𑄢\u{11128}𑄞𑄢\u{11134} 𑄃\u{11101}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "English River"), ("da", "La Rivière Anglaise"), ("de", "La Rivière Anglaise"), ("el", "Λα Ριβιέρε Ανγκλέζε"), ("en", "La Rivière Anglaise"), ("es", "La Riviere Anglaise"), ("fi", "La Rivière Anglaise"), ("fr", "La Rivière Anglaise"), ("gu", "લા રિવિએર\u{ac7} ઍ\u{a82}ગ\u{acd}લ\u{ac7}ઈસ"), ("hi", "ला रिविएर ए\u{902}ग\u{94d}ल\u{947}स"), ("id", "La Rivière Anglaise"), ("it", "La Rivière Anglaise"), ("ja", "ラ・リヴィエール・アングレーズ"), ("ka", "ლა-რივიერ-ანგლეზი"), ("kn", "ಲಾ ರ\u{cbf}ವ\u{cbf}ಯರ\u{ccd} ಆಂಗ\u{ccd}ಲೈಸ\u{ccd}"), ("ko", "라리비에르앙글레즈 구"), ("lt", "La Rivjer Angles"), ("lv", "Inglišrivera"), ("mr", "ला रिविएर अ\u{901}ग\u{94d}लाइज"), ("ms", "La Rivière Anglaise"), ("nb", "English River"), ("nl", "La Riviere Anglaise"), ("no", "English River"), ("pl", "La Riviere Anglaise"), ("pt", "La Riviere Anglaise"), ("ro", "Districtul English River"), ("ru", "Ла-Ривьер-Англез"), ("si", "ල\u{dcf} ර\u{dd2}වෙය\u{dd2}රේ ඇන\u{dca}ග\u{dca}ලය\u{dca}සේ"), ("sv", "English River"), ("ta", "ல\u{bbe} ரிவோரே அங\u{bcd}கல\u{bbe}ய\u{bcd}ஸ\u{bcd}"), ("te", "ల\u{c3e} ర\u{c3f}వ\u{c3f}యర\u{c4d} అంగ\u{c4d}ల\u{c46}య\u{c3f}స\u{c4d}"), ("th", "ลา ร\u{e34}เว\u{e35}บเร\u{e48} แองไลเซ\u{e48}"), ("tr", "La Rivière Anglaise"), ("uk", "Ла-Рівьєр-Англез"), ("ur", "لا ریوییری انجلایسی"), ("vi", "La Rivière Anglaise"), ("zh", "英吉利河")]),
                        unofficial_name_list: ["La Rivière Anglaise"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::SC,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6166667), longitude: Some(55.4457768), max_latitude: Some(-4.609877), min_latitude: Some(-4.622685), max_longitude: Some(55.453148), min_longitude: Some(55.4409089)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مونت بوكستون"), ("bn", "মন\u{9cd}ট ব\u{9be}ক\u{9cd}সটন"), ("ca", "Mont Buxton"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑\u{11134} 𑄝\u{1112a}𑄇\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Mont Buxton"), ("da", "Mont Buxton"), ("de", "Mont Buxton"), ("el", "Μοντ Μπουξτόν"), ("en", "Mont Buxton"), ("es", "Mont Buxton"), ("fi", "Mont Buxton"), ("fr", "Mont Buxton"), ("gu", "મોન\u{acd}ટ બક\u{acd}સટન"), ("hi", "मो\u{902}ट बक\u{94d}सटन"), ("id", "Mont Buxton"), ("it", "Mont Buxton"), ("ja", "モント・ブクソン"), ("ka", "მონ-ბაქსტონი"), ("kn", "ಮಾಂಟ\u{ccd} ಬಕ\u{ccd}ಸ\u{ccd}ಟನ\u{ccd}"), ("ko", "몽뷕스통 구"), ("lt", "Mont Bakstonas"), ("lv", "Montbakstona"), ("mr", "मो\u{902}ट बक\u{94d}स\u{94d}टस\u{94d}टन"), ("ms", "Mont Buxton"), ("nb", "Mont Buxton"), ("nl", "Mont Buxton"), ("no", "Mont Buxton"), ("pl", "Mont Buxton"), ("pt", "Mont Buxton"), ("ro", "Districtul Mont Buxton"), ("ru", "Монт-Бакстон"), ("si", "මොන\u{dca}ට\u{dca} බක\u{dca}ස\u{dca}ටන\u{dca}"), ("sv", "Mont Buxton"), ("ta", "மோண\u{bcd}ட\u{bcd} பூஸ\u{bcd}ட\u{bcd}ட\u{bbe}ன\u{bcd}"), ("te", "మ\u{c3e}ంట\u{c4d} బుక\u{c4d}స\u{c4d}టన\u{c4d}"), ("th", "มอนต\u{e4c} บ\u{e38}คทอล"), ("tr", "Mont Buxton"), ("uk", "Монт-Бакстон"), ("ur", "مونٹ بوزتون"), ("vi", "Mont Buxton"), ("zh", "蒙巴克斯頓區")]),
                        unofficial_name_list: ["Mont Buxton"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::SC,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6356543), longitude: Some(55.4554688), max_latitude: Some(-4.60039), min_latitude: Some(-4.644636), max_longitude: Some(55.50656499999999), min_longitude: Some(55.4475631)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مونت فليوري"), ("bn", "মন\u{9cd}ট ফ\u{9cd}লেঊরি"), ("ca", "Mont Fleuri"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑\u{11134} 𑄜\u{11133}𑄣𑄬𑄅\u{1112a}𑄢\u{11128}"), ("ceb", "Mont Fleuri"), ("da", "Mont Fleuri"), ("de", "Mont Fleuri"), ("el", "Μοντ Φλέρι"), ("en", "Mont Fleuri"), ("es", "Mont Fleuri"), ("fi", "Mont Fleuri"), ("fr", "Mont Fleuri"), ("gu", "મોન\u{acd}ટ ફ\u{acd}લ\u{ac7}ઉરી"), ("hi", "मो\u{902}ट फ\u{94d}ल\u{947}री"), ("id", "Mont Fleuri"), ("it", "Mont Fleuri"), ("ja", "モント・フレーリー"), ("ka", "მონ-ფლორი"), ("kn", "ಮಾಂಟ\u{ccd} ಫ\u{ccd}ಲ\u{cc2}ರ\u{cbf}"), ("ko", "몽플뢰리 구"), ("lt", "Mont Fleris"), ("lv", "Monfleri"), ("mr", "मा\u{901}ट फ\u{941}ल\u{947}री"), ("ms", "Mont Fleuri"), ("nb", "Mont Fleuri"), ("nl", "Mont Fleuri"), ("no", "Mont Fleuri"), ("pl", "Mont Fleuri"), ("pt", "Monte Fleuri"), ("ro", "Districtul Mont Fleuri"), ("ru", "Монт-Флери"), ("si", "මොන\u{dca}ට\u{dca} ෆ\u{dca}ලෙය\u{dd4}ර\u{dd2}"), ("sv", "Mont Fleuri"), ("ta", "மோண\u{bcd}ட\u{bcd} பிளேயரி"), ("te", "మ\u{c3e}ంట\u{c4d} ఫ\u{c4d}ల\u{c4d}యూర\u{c3f}"), ("th", "อนท\u{e4c} เฟ\u{e34}ลอ\u{e39}ร\u{e34}"), ("tr", "Mont Fleuri"), ("uk", "Монт-Флері"), ("ur", "مونٹ فلیوری"), ("vi", "Mont Fleuri"), ("zh", "蒙弗勒利區")]),
                        unofficial_name_list: ["Mont Fleuri"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::SC,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6489918), longitude: Some(55.4610075), max_latitude: Some(-4.6315425), min_latitude: Some(-4.667039), max_longitude: Some(55.4712058), min_longitude: Some(55.4511051)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلايزانس"), ("bn", "প\u{9cd}ল\u{9be}য\u{9bc}সেন\u{9cd}স"), ("ca", "Plaisance"), ("ccp", "𑄛\u{11133}𑄣\u{1112d}𑄥𑄬𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Plaisance (distrito)"), ("da", "Plaisance"), ("de", "Plaisance"), ("el", "Πλεσάνς"), ("en", "Plaisance"), ("es", "Plaisance"), ("fi", "Plaisance"), ("fr", "Plaisance"), ("gu", "પ\u{acd}લ\u{ac7}સ\u{ac7}ન\u{acd}સ"), ("hi", "प\u{94d}ल\u{947}स\u{947}\u{902}स"), ("id", "Plaisance, Seychelles"), ("it", "Plaisance"), ("ja", "プライセンス"), ("ka", "პლეზანსი"), ("kn", "ಪ\u{ccd}ಲೈಸಾನ\u{ccd}ಸ\u{ccd}"), ("ko", "플레장스 구"), ("lt", "Plezansas"), ("lv", "Plezanse"), ("mr", "प\u{94d}ल\u{945}न\u{94d}सिस"), ("ms", "Plaisance, Seychelles"), ("nb", "Plaisance"), ("nl", "Plaisance"), ("no", "Plaisance"), ("pl", "Plaisance (Seszele)"), ("pt", "Plaisance"), ("ro", "Districtul Plaisance"), ("ru", "Плезанс"), ("si", "ප\u{dca}ලය\u{dd2}සන\u{dca}සේ"), ("sv", "Plaisance, Seychellerna"), ("ta", "பிலைஸன\u{bcd}ஸ\u{bcd}"), ("te", "ప\u{c4d}ల\u{c3e}య\u{c3f}స\u{c3e}న\u{c4d}స\u{c4d}"), ("th", "ไพรแซนซ\u{e4c}"), ("tr", "Plasiance"), ("uk", "Плезанс"), ("ur", "پلایسانکی"), ("vi", "Plaisance"), ("zh", "普萊桑斯區")]),
                        unofficial_name_list: ["Plaisance"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::SC,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.680489), longitude: Some(55.51918569999999), max_latitude: Some(-4.6648396), min_latitude: Some(-4.686879999999999), max_longitude: Some(55.5340801), min_longitude: Some(55.50524799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوينت لا رو"), ("bn", "পয\u{9bc}ন\u{9cd}তে ল\u{9be} রো"), ("ca", "Pointe La Rue"), ("ccp", "𑄛\u{11127}𑄠𑄬𑄚\u{11134}𑄑\u{11134} 𑄣 𑄛\u{1112a}"), ("ceb", "Pointe Larue"), ("da", "Pointe La Rue"), ("de", "Pointe Larue"), ("el", "Πόιντε Λα Ρου"), ("en", "Pointe La Rue"), ("es", "Pointe La Rue"), ("fi", "Pointe La Rue"), ("fr", "Pointe La Rue"), ("gu", "પોઇન\u{acd}ટ\u{ac7} લા ર\u{ac1}"), ("hi", "पॉइ\u{902}ट ला र\u{942}"), ("id", "Pointe La Rue"), ("it", "Pointe La Rue"), ("ja", "ポイント・ラ・ルエ"), ("ka", "პუანტ-ლა-რუ"), ("kn", "ಪಾಯ\u{cbf}ಂಟ\u{ccd} ಲಾ ರ\u{cc2}"), ("ko", "푸앵트라루 구"), ("lt", "Puant La Ru"), ("lv", "Puantlaru"), ("mr", "पॉ\u{902}टा ला र\u{942}"), ("ms", "Pointe La Rue"), ("nb", "Pointe La Rue"), ("nl", "Pointe La Rue"), ("no", "Pointe La Rue"), ("pl", "Pointe La Rue"), ("pt", "Point da Rua"), ("ro", "Districtul Pointe La Rue"), ("ru", "Пуант-ля-Рю"), ("si", "පොය\u{dd2}න\u{dca}ටේ ල\u{dcf} ර\u{dd4}ය\u{dd2}"), ("sv", "Pointe La Rue"), ("ta", "ப\u{bbe}யிண\u{bcd}ட\u{bcd} ல\u{bbe} ருயி"), ("te", "ప\u{c3e}య\u{c3f}ంట\u{c4d} ల\u{c3e} ర\u{c4d}యూ"), ("th", "พอยท\u{e4c}ลาล\u{e39}"), ("tr", "Point La Rue"), ("uk", "Пуант-Ларю"), ("ur", "پوائنٹ لا روی"), ("vi", "Pointe La Rue"), ("zh", "普安特拉儒區")]),
                        unofficial_name_list: ["Pointe La Rue"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::SC,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.6488523), longitude: Some(55.41947529999999), max_latitude: Some(-4.6252479), min_latitude: Some(-4.6793682), max_longitude: Some(55.4511051), min_longitude: Some(55.3605731)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بورت غلاود"), ("bn", "পোর\u{9cd}ট গ\u{9cd}লড"), ("ca", "Port Glaud"), ("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134} 𑄉\u{11133}𑄣𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "Port Glaud"), ("da", "Port Glaud"), ("de", "Port Glaud"), ("el", "Πορτ Γκλόντ"), ("en", "Port Glaud"), ("es", "Port Glaud"), ("fi", "Port Glaud"), ("fr", "Port Glaud"), ("gu", "પોર\u{acd}ટ ગ\u{acd}લાઉડ"), ("hi", "पोर\u{94d}ट ग\u{94d}लाउद"), ("id", "Port Glaud"), ("it", "Port Glaud"), ("ja", "ポート・グランド"), ("ka", "პორტ-გლოდი"), ("kn", "ಪೋರ\u{ccd}ಟ\u{ccd} ಗ\u{ccd}ಲಾಡ\u{ccd}"), ("ko", "포르글로드 구"), ("lt", "Port Glo"), ("lv", "Portglo"), ("mr", "पोर\u{94d}ट ग\u{94d}लॉआद"), ("ms", "Pelabuhan Glaud"), ("nb", "Port Glaud"), ("nl", "Port Glaud"), ("no", "Port Glaud"), ("pl", "Port Glaud"), ("pt", "Port Glaud"), ("ro", "Districtul Port Glaud"), ("ru", "Пор Гло"), ("si", "පොර\u{dca}ට\u{dca} ග\u{dca}ලෞඩ\u{dca}"), ("sv", "Port Glaud"), ("ta", "போர\u{bcd}ட\u{bcd} கில\u{bbe}ட\u{bcd}"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4d} గ\u{c4d}ల\u{c3e}డ\u{c4d}"), ("th", "พอร\u{e4c}ตกลาด\u{e4c}"), ("tr", "Port Glaud"), ("uk", "Пор Гло"), ("ur", "پورٹ جلاود"), ("vi", "Port Glaud"), ("zh", "波特格勞德區")]),
                        unofficial_name_list: ["Port Glaud"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::SC,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.625194899999999), longitude: Some(55.4430078), max_latitude: Some(-4.61719), min_latitude: Some(-4.6316139), max_longitude: Some(55.4484299), min_longitude: Some(55.4323299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانت لويس"), ("bn", "সেন\u{9cd}ট ল\u{9c1}ই"), ("ca", "Saint Louis"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄣\u{1112a}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Saint Louis"), ("da", "Saint Louis"), ("de", "Saint Louis"), ("el", "Σεντ Λούις"), ("en", "Saint Louis"), ("es", "Saint Louis"), ("fi", "Saint Louis"), ("fr", "Saint Louis"), ("gu", "સ\u{ac7}ન\u{acd}ટ લ\u{ac2}ઇસ"), ("hi", "स\u{947}\u{902}ट ल\u{941}इ, स\u{947}शल\u{94d}स"), ("id", "Saint Louis, Seychelles"), ("it", "Saint Louis"), ("ja", "セント・ローイス"), ("jv", "Saint Louis, Seychelles"), ("ka", "სენ-ლუი"), ("kn", "ಸೇಂಟ\u{ccd} ಲ\u{cc2}ಯ\u{cbf}ಸ\u{ccd}"), ("ko", "생루이 구"), ("lt", "Saint Luisas"), ("lv", "Sentluisa"), ("ml", "സെന\u{d4d}റ\u{d4d} ല\u{d42}യിസ\u{d4d} ജില\u{d4d}ല"), ("mr", "स\u{947}\u{902}ट ल\u{941}ईस"), ("ms", "Saint Louis, Seychelles"), ("nb", "Saint Louis"), ("nl", "Saint Louis"), ("no", "Saint Louis"), ("pl", "Saint Louis"), ("pt", "São Luís"), ("ru", "Сент-Луис"), ("si", "ශ\u{dcf}න\u{dca}ත ල\u{dd4}ව\u{dd2}ස\u{dca}"), ("sv", "Saint Louis, Seychellerna"), ("ta", "செயின\u{bcd}ட\u{bcd} லூயிஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} లూయ\u{c40}"), ("th", "เซนต\u{e4c} ล\u{e39}อ\u{e34}ส"), ("tr", "Saint Louis"), ("uk", "Парафія Сент-Луїс"), ("ur", "سینٹ لوویس"), ("vi", "Saint Louis"), ("zh", "聖路易區")]),
                        unofficial_name_list: ["Saint Louis"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::SC,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.7843688), longitude: Some(55.5081012), max_latitude: Some(-4.7590715), min_latitude: Some(-4.805029999999999), max_longitude: Some(55.534912), min_longitude: Some(55.486445)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تاكاماكا، سيشيل"), ("bn", "ত\u{9be}ক\u{9be}ম\u{9be}ক\u{9be}"), ("ca", "Takamaka"), ("ccp", "𑄑𑄇𑄟𑄇"), ("ceb", "Takamaka"), ("da", "Takamaka"), ("de", "Takamaka"), ("el", "Τακαμάκα"), ("en", "Takamaka"), ("es", "Takamaka"), ("fi", "Takamaka"), ("fr", "Takamaka"), ("gu", "તાકામાકા"), ("hi", "टाकामाका"), ("id", "Takamaka"), ("it", "Takamaka"), ("ja", "タカマカ"), ("ka", "ტაკამაკა"), ("kn", "ತಕಮಾಕ"), ("ko", "타카마카 구"), ("lt", "Takamaka"), ("lv", "Takamaka"), ("mr", "ताकाम\u{947}क"), ("ms", "Takamaka"), ("nb", "Takamaka"), ("nl", "Takamaka"), ("no", "Takamaka"), ("pl", "Takamaka"), ("pt", "Anse Takamaka"), ("ro", "Districtul Takamaka"), ("ru", "Такамака"), ("si", "ටකමක\u{dcf}"), ("sv", "Takamaka"), ("ta", "டகம\u{bbe}க\u{bbe}"), ("te", "ట\u{c3e}కమ\u{c3e}క\u{c3e}"), ("th", "ทากามากา"), ("tr", "Takamaka"), ("uk", "Такамака"), ("ur", "تاکاماکا"), ("vi", "Takamaka"), ("zh", "塔卡瑪卡區")]),
                        unofficial_name_list: ["Takamaka"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::SC,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لس ماميلس"), ("bn", "লেস ম\u{9be}মেলস"), ("ca", "Les Mamelles"), ("ccp", "𑄣𑄬𑄌\u{11134} 𑄟𑄟𑄬𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Les Mamelles"), ("da", "Les Mamelles"), ("de", "Les Mamelles"), ("el", "Λες Μαμέλες"), ("en", "Les Mamelles"), ("es", "Les Mamelles"), ("fi", "Les Mamelles"), ("fr", "Les Mamelles"), ("gu", "લ\u{ac7}સ મ\u{ac7}મ\u{ac7}લ\u{ac7}સ"), ("hi", "ली म\u{948}म\u{947}ल\u{947}स"), ("id", "Les Mamelles"), ("it", "Les Mamelles"), ("ja", "レス・マーメルズ"), ("ka", "ლე-მამელი"), ("kn", "ಲ\u{cc6}ಸ\u{ccd} ಮಮ\u{cc6}ಲ\u{ccd}ಸ\u{ccd}"), ("ko", "레마멜 구"), ("lt", "Les Mamelesas"), ("lv", "Lesmamelles"), ("mr", "ल\u{947}स म\u{945}म\u{947}ल\u{947}स"), ("ms", "Les Mamelles"), ("nb", "Les Mamelles"), ("nl", "Les Mamelles"), ("no", "Les Mamelles"), ("pl", "Les Mamelles"), ("pt", "Les Mamelles"), ("ro", "Districtul Les Mamelles"), ("ru", "Ле-Мамелль"), ("si", "ලෙස\u{dca} මමෙල\u{dd2}ස\u{dca}"), ("sv", "Les Mamelles"), ("ta", "லெஸ\u{bcd} ம\u{bbe}மெல\u{bcd}ல\u{bcd}ஸ\u{bcd}"), ("te", "ల\u{c47} మ\u{c3e}మ\u{c46}ల\u{c4d}స\u{c4d}"), ("th", "เลส มาเมลเลส"), ("tr", "Les Mamelles"), ("uk", "Ле-Мамель"), ("ur", "لیس مامیلیس"), ("vi", "Les Mamelles"), ("zh", "雷瑪麥爾區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::SC,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "روتش كايمان"), ("bn", "রোচ ক\u{9be}ইম\u{9cd}য\u{9be}ন"), ("ca", "Roche Caiman"), ("ccp", "𑄢\u{1112e}𑄌\u{11134} 𑄇\u{1112d}𑄟\u{11127}𑄚\u{11134}"), ("ceb", "Roche Caiman"), ("da", "Roche Caiman"), ("de", "Roche Caiman"), ("el", "Ρότσε Καϊμάν"), ("en", "Roche Caiman"), ("es", "Roche Caiman"), ("fi", "Roche Caiman"), ("fr", "Roche Caïman"), ("gu", "રોશ ક\u{ac8}મન"), ("hi", "रोश क\u{948}मन"), ("id", "Roche Caiman"), ("it", "Roche Caiman"), ("ja", "ロッチェ・チャイマン"), ("ka", "როშ-კაიმანი"), ("kn", "ರೋಚ\u{cc6} ಕೈಮನ\u{ccd}"), ("ko", "로슈카이망 구"), ("lt", "Rošė Kaimanas"), ("lv", "Roškaimana"), ("mr", "रोश च\u{948}माण"), ("ms", "Roche Caiman"), ("nb", "Roche Caiman"), ("nl", "Roche Caiman"), ("no", "Roche Caiman"), ("pl", "Roche Caiman"), ("pt", "Roche Caiman"), ("ro", "Districtul Roche Caiman"), ("ru", "Рош-Кайман"), ("si", "රෝචේ කය\u{dd2}මන\u{dca}"), ("sv", "Roche Caiman"), ("ta", "ரோசே க\u{bbe}ய\u{bcd}மன\u{bcd}"), ("te", "ర\u{c4b}చ\u{c46} క\u{c48}మన\u{c4d}"), ("th", "โรเซ\u{e48} ไซแมน"), ("tr", "Roche Caiman"), ("uk", "Рош-Кайман"), ("ur", "روچی کایمان"), ("vi", "Roche Caiman"), ("zh", "羅切凱曼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::SC,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Ile Perseverance I")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::SC,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Ile Perseverance II")]),
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
#[cfg(feature = "sc")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SC,
        alpha3: Alpha3::SYC,
        address_format: None,
        continent: Continent::Africa,
        country_code: 248,
        currency_code: "SCR",
        gec: Some(GEC::SE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("SEY"),
        iso_long_name: "The Republic of Seychelles",
        iso_short_name: "Seychelles",
        official_language_list: ["en", "fr"].to_vec(),
        spoken_language_list: ["en", "fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [6].to_vec(),
        national_prefix: "None",
        nationality: Some("Seychellois"),
        number: "690",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "SC",
        unofficial_name_list: ["Seychelles", "Seychellen", "セーシェル"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Seychelles"),
            ("af", "Seychelle"),
            ("ak", "Seychelles"),
            ("am", "ሲሸልስ"),
            ("an", "Seychelles"),
            ("ar", "الس\u{651}يشل"),
            ("as", "ছেয়চেলছ"),
            ("ay", "Seychelles"),
            ("az", "Seyşel"),
            ("ba", "Seychelles"),
            ("be", "Сейшэльскія Астравы"),
            ("bg", "Сейшели"),
            ("bi", "Seychelles"),
            ("bn", "সেশেলস"),
            ("bn_IN", "সেশেলস"),
            ("br", "Sechelez"),
            ("bs", "Sejšelsko otočje"),
            ("ca", "Seychelles"),
            ("ce", "Сейшелан гlайреш"),
            ("ch", "Seychelles"),
            ("cs", "Seychely"),
            ("cv", "Сейшелан гlайреш"),
            ("cy", "Seychelles"),
            ("da", "Seychellerne"),
            ("de", "Seychellen"),
            ("dv", "ތ\u{7ad}ނ\u{7b0}ގ\u{7a6}ދ\u{7a9}ބ\u{7aa}"),
            ("dz", "ས\u{f72}་ཅ\u{f72}་ལ\u{f7a}ས\u{f72}།"),
            ("ee", "Seychelles"),
            ("el", "Σεϋχέλλες"),
            ("en", "Seychelles"),
            ("eo", "Sejŝeloj"),
            ("es", "Seychelles"),
            ("et", "Seišellid"),
            ("eu", "Seychelleak"),
            ("fa", "سیشل"),
            ("ff", "Seychelles"),
            ("fi", "Seychellit"),
            ("fo", "Seyskelloyggjarnar"),
            ("fr", "Seychelles"),
            ("fy", "Seysjellen"),
            ("ga", "Na Séiséil"),
            ("gl", "Seichelles"),
            ("gn", "Seychelles"),
            ("gu", "સ\u{ac7}શ\u{ac7}લ\u{acd}સ"),
            ("gv", "Ny h-Ellanyn Heshell"),
            ("ha", "Seychelles"),
            ("he", "סיישל"),
            ("hi", "स\u{947}श\u{947}ल\u{94d}स"),
            ("hr", "Sejšeli"),
            ("ht", "Sechèl"),
            ("hu", "Seychelle-szigetek"),
            ("hy", "Սեյշելներ"),
            ("ia", "Seychelles"),
            ("id", "Seychelles"),
            ("io", "Seycheli"),
            ("is", "Seychelles-eyjar"),
            ("it", "Seychelles"),
            ("iu", "Seychelles"),
            ("ja", "セーシェル"),
            ("ka", "სეიშელის კუნძულები"),
            ("ki", "Seychelles"),
            ("kk", "Сейшел аралдары"),
            ("kl", "Seychelles"),
            ("km", "ស\u{17b8}ស\u{17d2}ហែល"),
            ("kn", "ಸೇಷಲ\u{ccd}ಸ\u{ccd}"),
            ("ko", "세이셸"),
            ("ku", "Seyselan"),
            ("kv", "Seychelles"),
            ("kw", "Seychellys"),
            ("ky", "Сейшел аралдары"),
            ("lo", "Seychelles"),
            ("lt", "Seišeliai"),
            ("lv", "Seišelas"),
            ("mi", "Seychelles"),
            ("mk", "Сејшели"),
            ("ml", "സെയ\u{d4d}ഷെല\u{d4d}\u{200d}സ\u{d4d}"),
            ("mn", "Seychelles"),
            ("mr", "स\u{947}श\u{947}ल\u{94d}स"),
            ("ms", "Seychelles"),
            ("mt", "Seychelles"),
            (
                "my",
                "ဆေးရ\u{103e}\u{1032}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Seychelles"),
            ("nb", "Seychellene"),
            ("ne", "सिच\u{947}ल\u{94d}लिस"),
            ("nl", "Seychellen"),
            ("nn", "Seychellane"),
            ("nv", "Seychelles"),
            ("oc", "Seichèlas"),
            ("or", "ସେଶେଲସ\u{b4d}"),
            ("pa", "ਸੀਆਚਿਲਸ"),
            ("pi", "स\u{947}श\u{947}ल"),
            ("pl", "Seszele"),
            ("ps", "سیشل"),
            ("pt", "Seychelles"),
            ("pt_BR", "Seychelles"),
            ("ro", "Seychelles"),
            ("ru", "Сейшелы"),
            ("rw", "Seyishele"),
            ("sc", "Seychelles"),
            ("sd", "Seychelles"),
            ("si", "ස\u{dd3}ශෙල\u{dca}ස\u{dca}"),
            ("sk", "Seychely"),
            ("sl", "Sejšeli"),
            ("so", "Seychelles"),
            ("sq", "Sejshelle"),
            ("sr", "Сејшели"),
            ("sv", "Seychellerna"),
            ("sw", "Seychelles"),
            ("ta", "ச\u{bc0}செல\u{bcd}சு"),
            ("te", "స\u{c47}శ\u{c47}ల\u{c4d}స\u{c4d}"),
            ("tg", "Сейшел"),
            ("th", "เซเชลส\u{e4c}"),
            ("ti", "Seychelles"),
            ("tk", "Seýşel Adalary"),
            ("tl", "Seychelles"),
            ("tr", "Seyşeller"),
            ("tt", "Сейшелләр"),
            ("ug", "سېيشېل"),
            ("uk", "Сейшели"),
            ("ur", "سیچیلیس"),
            ("uz", "Seyshell orollari"),
            ("ve", "Seychelles"),
            ("vi", "Xây-sen"),
            ("wa", "Seycheles"),
            ("wo", "Seychelles"),
            ("xh", "Seychelles"),
            ("yo", "Ṣèíhẹ\u{301}lẹ\u{301}sì"),
            ("zh_CN", "塞舌尔"),
            ("zh_HK", "塞舌爾"),
            ("zh_TW", "塞席爾"),
            ("zu", "IsiSeyisheli"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}