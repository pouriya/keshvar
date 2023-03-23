// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Ecuador

#[cfg(all(feature = "ec", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::EC;
    pub const ALPHA3: Alpha3 = Alpha3::ECU;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 593;
    pub const CURRENCY_CODE: &str = "USD";
    pub const GEC: Option<GEC> = Some(GEC::EC);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("ECU");
    pub const ISO_SHORT_NAME: &str = "Ecuador";
    pub const ISO_LONG_NAME: &str = "The Republic of Ecuador";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Ecuadorean");
    pub const NUMBER: &str = "218";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "EC";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Ecuador", "Équateur", "エクアドル"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Ecuador"),
        ("af", "Ecuador"),
        ("ak", "Ecuador"),
        ("am", "ጒጣ፦ሴ"),
        ("an", "Ecuador"),
        ("ar", "الإكوادور"),
        ("as", "ইকোৱেড'ৰ"),
        ("ay", "Ecuador"),
        ("az", "Ekvador"),
        ("ba", "Ecuador"),
        ("be", "Эквадор"),
        ("bg", "Еквадор"),
        ("bi", "Ecuador"),
        ("bn", "ইক\u{9c1}য়েডোর"),
        ("bn_IN", "ইক\u{9c1}য়েডোর"),
        ("br", "Ekuador"),
        ("bs", "Ekvador"),
        ("ca", "Equador"),
        ("ce", "Эквадор"),
        ("ch", "Ecuador"),
        ("cs", "Ekvádor"),
        ("cv", "Эквадор"),
        ("cy", "Ecwador"),
        ("da", "Ecuador"),
        ("de", "Ecuador"),
        ("dv", "އ\u{7a8}ކ\u{7aa}އ\u{7ac}ޑ\u{7af}ރ\u{7aa}"),
        ("dz", "ཨ\u{f72}་ཀ\u{f74}ཡ\u{f7a}་ཌ\u{f7c}ར།"),
        ("ee", "Ecuador"),
        ("el", "Ισημερινός"),
        ("en", "Ecuador"),
        ("eo", "Ekvadoro"),
        ("es", "Ecuador"),
        ("et", "Ecuador"),
        ("eu", "Ekuador"),
        ("fa", "اکوادور"),
        ("ff", "Ecuador"),
        ("fi", "Ecuador"),
        ("fo", "Ekvador"),
        ("fr", "Équateur"),
        ("fy", "Ekwador"),
        ("ga", "Eacuadór"),
        ("gl", "Ecuador"),
        ("gn", "Ecuador"),
        ("gu", "ઇક\u{acd}વાડોર"),
        ("gv", "Ecuador"),
        ("ha", "Ecuador"),
        ("he", "אקוודור"),
        ("hi", "ईक\u{94d}वाडोर"),
        ("hr", "Ekvador"),
        ("ht", "Ekwatè"),
        ("hu", "Ecuador"),
        ("hy", "Էկվադոր"),
        ("ia", "Ecuador"),
        ("id", "Ekuador"),
        ("io", "Equador"),
        ("is", "Ekvador"),
        ("it", "Ecuador"),
        ("iu", "Ecuador"),
        ("ja", "エクアドル"),
        ("ka", "ეკვადორი"),
        ("ki", "Ecuador"),
        ("kk", "Эквадор"),
        ("kl", "Ecuador"),
        ("km", "អេក\u{17d2}វាឌ\u{17d0}រ"),
        ("kn", "ಈಕ\u{ccd}ವ\u{cc6}ಡಾರ\u{ccd}"),
        ("ko", "에콰도르"),
        ("ku", "Ekvator"),
        ("kv", "Ecuador"),
        ("kw", "Pow Ekwadorel"),
        ("ky", "Эквадор"),
        ("lo", "ປະເທດເອກ\u{ebb}ວເຕ\u{eb5}"),
        ("lt", "Ekvadoras"),
        ("lv", "Ekvadora"),
        ("mi", "Ekuatoa"),
        ("mk", "Еквадор"),
        ("ml", "ഇക\u{d4d}വഡോര\u{d4d}\u{200d}"),
        ("mn", "Эквадор"),
        ("mr", "इक\u{94d}व\u{947}डॉर"),
        ("ms", "Ecuador"),
        ("mt", "Ecuador"),
        (
            "my",
            "အ\u{102e}က\u{103d}ေဒေါန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ekwador"),
        ("nb", "Ecuador"),
        ("ne", "इक\u{94d}व\u{947}डर"),
        ("nl", "Ecuador"),
        ("nn", "Ecuador"),
        ("nv", "Ecuador"),
        ("oc", "Eqüator"),
        ("or", "ଏକ\u{b4d}ଯ\u{b41}ଡୋର"),
        ("pa", "ਏਕਵ\u{a47}ਡਰ"),
        ("pi", "एक\u{94d}वाडोर"),
        ("pl", "Ekwador"),
        ("ps", "اېکوادور"),
        ("pt", "Equador"),
        ("pt_BR", "Equador"),
        ("ro", "Ecuador"),
        ("ru", "Эквадор"),
        ("rw", "Ekwadoro"),
        ("sc", "Ècuador"),
        ("sd", "Ecuador"),
        ("si", "ඉක\u{dca}වදෝරය"),
        ("sk", "Ekvádor"),
        ("sl", "Ekvador"),
        ("so", "Ikwadoor"),
        ("sq", "Ekuador"),
        ("sr", "Еквадор"),
        ("sv", "Ecuador"),
        ("sw", "Ecuador"),
        ("ta", "ஈக\u{bcd}வட\u{bbe}ர\u{bcd}"),
        ("te", "ఈక\u{c4d}వ\u{c3f}డ\u{c3e}ర\u{c4d}"),
        ("tg", "Эквадор"),
        ("th", "เอกวาดอร\u{e4c}"),
        ("ti", "ኤኳዶር"),
        ("tk", "Ekwador"),
        ("tl", "Ekwador"),
        ("tr", "Ekvador"),
        ("tt", "Екуадор"),
        ("ug", "ئېكۋادور"),
        ("uk", "Еквадор"),
        ("ur", "ایکواڈور"),
        ("uz", "Ekvador"),
        ("ve", "Ecuador"),
        ("vi", "Ê-cu-a-đoa"),
        ("wa", "Ecwåteur"),
        ("wo", "Ekwadoor"),
        ("xh", "Ecuador"),
        ("yo", "Ẹ\u{300}kùàdọ\u{300}r"),
        ("zh_CN", "厄瓜多尔"),
        ("zh_HK", "厄瓜多爾"),
        ("zh_TW", "厄瓜多"),
        ("zu", "Ecuador"),
    ];
    #[cfg(all(feature = "ec", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -1.831239;
        pub const LONGITUDE: f64 = -78.18340599999999;
        pub const MAX_LATITUDE: f64 = 2.2955;
        pub const MAX_LONGITUDE: f64 = -75.1887938;
        pub const MIN_LATITUDE: f64 = -5.0143509;
        pub const MIN_LONGITUDE: f64 = -92.60379999999999;
        pub const NORTHEAST_LATITUDE: f64 = 2.2955;
        pub const NORTHEAST_LONGITUDE: f64 = -75.1887938;
        pub const SOUTHWEST_LATITUDE: f64 = -5.0143509;
        pub const SOUTHWEST_LONGITUDE: f64 = -92.60379999999999;
    }
}
#[cfg(all(feature = "ec", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -1.831239,
            longitude: -78.18340599999999,
            max_latitude: 2.2955,
            max_longitude: -75.1887938,
            min_latitude: -5.0143509,
            min_longitude: -92.60379999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 2.2955,
                    longitude: -75.1887938,
                },
                southwest: CountryGeoBound {
                    latitude: -5.0143509,
                    longitude: -92.60379999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "ec", feature = "subdivisions"))]
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
                    "A",
                    Subdivision{
                        name: "A",
                        country_alpha2: Alpha2::EC,
                        code: "A",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.9633245), longitude: Some(-79.1096901), max_latitude: Some(-2.55303), min_latitude: Some(-3.620066), max_longitude: Some(-78.43984999999999), min_longitude: Some(-79.76039899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أزواي"), ("be", "Правінцыя Асуай"), ("bg", "Асуай"), ("bn", "আজ\u{9c1}য\u{9bc}\u{9be}য\u{9bc} প\u{9cd}রদেশ"), ("ca", "Província d’Azuay"), ("ccp", "𑄃𑄎\u{1112a}𑄠𑄬"), ("ceb", "Provincia del Azuay"), ("da", "Azuay"), ("de", "Provinz Azuay"), ("el", "Ασουάι"), ("en", "Azuay"), ("es", "Provincia de Azuay"), ("fa", "آزوای"), ("fi", "Azuay"), ("fr", "Azuay"), ("gu", "એઝ\u{acd}ય\u{ac1}એ પ\u{acd}રા\u{a82}ત"), ("he", "אסואי"), ("hi", "अज\u{93c}\u{941}य\u{947} प\u{94d}रा\u{902}त"), ("hu", "Azuay tartomány"), ("hy", "Ասուայ"), ("id", "Provinsi Azuay"), ("it", "provincia di Azuay"), ("ja", "アスアイ県"), ("ka", "ასუაის პროვინცია"), ("kn", "ಅಝ\u{cc2} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아수아이 주"), ("lt", "Asuajaus provincija"), ("lv", "Asvajas province"), ("mk", "Азуај"), ("mr", "अझ\u{942}य प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Azuay"), ("nb", "Azuay"), ("nl", "Azuay"), ("no", "Azuay"), ("pl", "Prowincja Azuay"), ("pt", "Azuay"), ("ru", "Асуай"), ("si", "අස\u{dd4}වේ පළ\u{dcf}ත"), ("sv", "Azuay"), ("ta", "அஸுய\u{bbe}ய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అజువ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาซวย"), ("tr", "Azuay Province"), ("uk", "Асуай"), ("ur", "آسوای صوبہ"), ("vi", "Azuay"), ("zh", "阿苏艾省")]),
                        unofficial_name_list: ["Azuay"].to_vec(),
                    }
                ),
                (
                    "B",
                    Subdivision{
                        name: "B",
                        country_alpha2: Alpha2::EC,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.5696244), longitude: Some(-79.1096901), max_latitude: Some(-1.148095), min_latitude: Some(-2.2034861), max_longitude: Some(-78.85047899999999), min_longitude: Some(-79.4053569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوليفار"), ("be", "Правінцыя Балівар"), ("bg", "Боливар"), ("bn", "বলিভ\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Bolívar"), ("ccp", "𑄝\u{11127}𑄣\u{11128}𑄞𑄢\u{11134}"), ("ceb", "Provincia de Bolívar (lalawigan)"), ("da", "Bolívar"), ("de", "Provinz Bolívar"), ("el", "Επαρχία Μπολίβαρ"), ("en", "Bolívar"), ("es", "Provincia de Bolívar"), ("fa", "استان بلیوار"), ("fi", "Bolívar"), ("fr", "Bolívar"), ("gu", "બોલિવર પ\u{acd}રા\u{a82}ત"), ("hi", "बोलीवार प\u{94d}रा\u{902}त"), ("hu", "Bolívar tartomány"), ("hy", "Բոլիվար"), ("id", "Provinsi Bolívar"), ("it", "provincia di Bolívar"), ("ja", "ボリーバル県"), ("ka", "ბოლივარის პროვინცია"), ("kn", "ಬೊಲ\u{cbf}ವಾರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "볼리바르 주"), ("lt", "Bolivaro provincija"), ("lv", "Bolivara province"), ("mr", "बोलिवर प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Bolívar"), ("nb", "Bolívar"), ("nl", "Bolívar"), ("no", "Bolívar"), ("pl", "Prowincja Bolívar"), ("pt", "Bolívar"), ("ru", "Боливар"), ("si", "බොල\u{dd2}වර\u{dca} පළ\u{dcf}ත"), ("sv", "Bolívar"), ("ta", "பொலிவ\u{bbe}ர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4b}ల\u{c3f}వర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "\u{e34}ดเด\u{e34}\u{e49}ล เชลเบลล\u{e35}"), ("tr", "Bolivar Province"), ("uk", "Болівар"), ("ur", "بولیوار صوبہ"), ("vi", "Tỉnh Bolívar"), ("zh", "玻利瓦尔省")]),
                        unofficial_name_list: ["Bolívar"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::EC,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.6625532), longitude: Some(-78.0195387), max_latitude: Some(1.1878679), min_latitude: Some(0.360788), max_longitude: Some(-77.5582881), min_longitude: Some(-78.55054489999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كارتشي"), ("be", "Правінцыя Карчы"), ("bg", "Карчи"), ("bn", "ক\u{9be}রচি প\u{9cd}রদেশ"), ("ca", "Província de Carchi"), ("ccp", "𑄇𑄢\u{11134}𑄌\u{11128}"), ("ceb", "Provincia del Carchi"), ("da", "Carchi"), ("de", "Provinz Carchi"), ("el", "Κάρτσι"), ("en", "Carchi"), ("es", "Provincia de Carchi"), ("fa", "کارچی"), ("fi", "Carchi"), ("fr", "Carchi"), ("gl", "Carchi"), ("gu", "કાર\u{acd}ચી પ\u{acd}રા\u{a82}ત"), ("he", "מחוז קארצ׳י"), ("hi", "कारची प\u{94d}रा\u{902}त"), ("hu", "Carchi tartomány"), ("hy", "Կարչի"), ("id", "Provinsi Carchi"), ("it", "provincia del Carchi"), ("ja", "カルチ県"), ("ka", "კარჩის პროვინცია"), ("kn", "ಕಾರ\u{ccd}ಚ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카르치 주"), ("lt", "Karčio provincija"), ("lv", "Karči province"), ("mr", "कारची प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Carchi"), ("nb", "Carchi"), ("nl", "Carchi"), ("no", "Carchi"), ("pl", "Prowincja Carchi"), ("pt", "Carchi"), ("ru", "Карчи"), ("si", "කර\u{dca}ච\u{dd2} පළ\u{dcf}ත"), ("sv", "Carchi"), ("ta", "சர\u{bcd}ச\u{bcd}சை ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}చ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาร\u{e4c}ช\u{e34}"), ("tr", "Carchi Province"), ("uk", "Карчі"), ("ur", "کارچی صوبہ"), ("vi", "Tỉnh Carchi"), ("zh", "卡尔奇省")]),
                        unofficial_name_list: ["Carchi"].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::EC,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.7112664999999999), longitude: Some(-77.15426839999999), max_latitude: Some(-0.6898046), min_latitude: Some(-0.7265373), max_longitude: Some(-77.13389389999999), min_longitude: Some(-77.181015)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوريينا"), ("be", "Правінцыя Арэльяна"), ("bg", "Ореляна"), ("bn", "ওরেল\u{9cd}ল\u{9be}ন\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Orellana"), ("ccp", "𑄃\u{11127}𑄢𑄬𑄣𑄚"), ("ceb", "Provincia de Francisco de Orellana"), ("da", "Orellana Province"), ("de", "Provinz Orellana"), ("el", "Ορεγιάνα"), ("en", "Orellana"), ("es", "Provincia de Orellana"), ("fa", "استان اوریانا"), ("fi", "Orellana"), ("fr", "Orellana"), ("gu", "ઓર\u{ac7}લના પ\u{acd}રા\u{a82}ત"), ("hi", "ओर\u{947}ल\u{94d}लाना प\u{94d}रा\u{902}त"), ("hu", "Orellana tartomány"), ("hy", "Օրելյանա"), ("id", "Provinsi Orellana"), ("it", "provincia di Orellana"), ("ja", "オレリャーナ県"), ("ka", "ორელიანას პროვინცია"), ("kn", "ಓರ\u{cc6}ಲ\u{ccd}ಲಾನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오레야나 주"), ("lt", "Oreljanos provincija"), ("lv", "Oreljanas province"), ("mr", "ओर\u{947}ल\u{94d}याना प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Orellana"), ("nb", "Orellana"), ("nl", "Orellana"), ("no", "Orellana"), ("pl", "Prowincja Orellana"), ("pt", "Orellana"), ("ru", "Орельяна"), ("si", "ඔරෙල\u{dca}ලන\u{dcf} පළ\u{dcf}ත"), ("sv", "Orellana"), ("ta", "ஒரெல\u{bcd}லன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓర\u{c46}ల\u{c4d}ల\u{c3e}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอเรลลาน\u{e48}า"), ("tr", "Orellana Province"), ("uk", "Орельяна"), ("ur", "اوریانا صوبہ"), ("vi", "Tỉnh Orellana"), ("zh", "奥雷亚纳省")]),
                        unofficial_name_list: ["Orellana"].to_vec(),
                    }
                ),
                (
                    "E",
                    Subdivision{
                        name: "E",
                        country_alpha2: Alpha2::EC,
                        code: "E",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.9681789), longitude: Some(-79.6517202), max_latitude: Some(0.9975799), min_latitude: Some(0.9292910000000001), max_longitude: Some(-79.6095084), min_longitude: Some(-79.672122)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إسمرالداس"), ("be", "Правінцыя Эсмеральдас"), ("bg", "Есмералдас"), ("bn", "এস\u{9cd}ম\u{9be}রেল\u{9cd}ড\u{9be}স"), ("ca", "Província d’Esmeraldas"), ("ccp", "𑄃𑄬𑄌\u{11134}𑄟𑄢𑄣\u{11134}𑄓𑄌\u{11134}"), ("ceb", "Provincia de Esmeraldas"), ("da", "Esmeraldas"), ("de", "Provinz Esmeraldas"), ("el", "Εσμεράλντας"), ("en", "Esmeraldas"), ("es", "Provincia de Esmeraldas"), ("fa", "استان اسمرالداس"), ("fi", "Esmeraldas"), ("fr", "Esmeraldas"), ("gu", "એસ\u{acd}મ\u{ac7}રલ\u{acd}ડસ"), ("he", "מחוז אסמרלדס"), ("hi", "एस\u{94d}मराल\u{94d}डा प\u{94d}रा\u{902}त"), ("hu", "Esmeraldas tartomány"), ("hy", "Էսմերալդաս"), ("id", "Provinsi Esmeraldas"), ("it", "provincia di Esmeraldas"), ("ja", "エスメラルダス県"), ("ka", "ესმერალდასის პროვინცია"), ("kn", "ಎಸ\u{ccd}ಮ\u{cc6}ರಾಲ\u{ccd}ಡಾಸ\u{ccd}"), ("ko", "에스메랄다스 주"), ("lt", "Esmeraldaso provincija"), ("lv", "Esmeraldasas province"), ("mr", "एस\u{94d}म\u{947}रल\u{94d}डस"), ("ms", "Pentadbiran Esmeraldas"), ("nb", "Esmeraldas"), ("nl", "Esmeraldas"), ("no", "Esmeraldas"), ("pl", "Prowincja Esmeraldas"), ("pt", "Esmeraldas"), ("ru", "Эсмеральдас"), ("si", "එස\u{dca}මෙරල\u{dca}ඩ\u{dcf}ස\u{dca}"), ("sv", "Esmeraldas"), ("ta", "எஸ\u{bcd}மெர\u{bbe}ல\u{bcd}டஸ\u{bcd}"), ("te", "ఎస\u{c4d}మర\u{c3e}ల\u{c4d}డ\u{c3e}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอสเมอร\u{e31}ลเดส"), ("tr", "Esmeraldas"), ("uk", "Есмеральдас"), ("ur", "ایزمیراداس صوبہ"), ("vi", "Esmeraldas"), ("zh", "埃斯梅拉达斯省")]),
                        unofficial_name_list: ["Esmeraldas"].to_vec(),
                    }
                ),
                (
                    "F",
                    Subdivision{
                        name: "F",
                        country_alpha2: Alpha2::EC,
                        code: "F",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.555695), longitude: Some(-78.9344814), max_latitude: Some(-2.5385129), min_latitude: Some(-2.5782558), max_longitude: Some(-78.9242364), min_longitude: Some(-78.9535904)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كانار"), ("be", "Правінцыя Каньяр"), ("bg", "Каняр"), ("bn", "ক\u{9be}ন\u{9be}র প\u{9cd}রদেশ’"), ("ca", "Província de Cañar"), ("ccp", "𑄇𑄬𑄚𑄢\u{11134}"), ("ceb", "Provincia del Cañar"), ("da", "Cañar"), ("de", "Provinz Cañar"), ("el", "Κανιάρ"), ("en", "Cañar"), ("es", "Provincia de Cañar"), ("fa", "استان کانیار"), ("fi", "Cañar"), ("fr", "Cañar"), ("gu", "કાનર પ\u{acd}રા\u{a82}ત"), ("hi", "कनार प\u{94d}रा\u{902}त"), ("hu", "Cañar tartomány"), ("id", "Provinsi Cañar"), ("it", "provincia di Cañar"), ("ja", "カニャール県"), ("ka", "კანიარის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾನಾರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카냐르 주"), ("lt", "Kanjaro provincija"), ("lv", "Kanjaras province"), ("mr", "क\u{945}नर प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Cañar"), ("nb", "Cañar"), ("nl", "Cañar"), ("no", "Cañar"), ("pl", "Prowincja Cañar"), ("pt", "Cañar"), ("ru", "Каньяр"), ("si", "කන\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sv", "Cañar"), ("ta", "க\u{bbe}ண\u{bbe}ர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}న\u{c3e}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บ\u{e39}ลามบ\u{e39}ลร"), ("tr", "Canar Province"), ("uk", "Каньяр"), ("ur", "کانیار صوبہ"), ("vi", "Tỉnh Canar"), ("zh", "卡尼亞爾省")]),
                        unofficial_name_list: ["Cañar"].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "G",
                        country_alpha2: Alpha2::EC,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.9574839), longitude: Some(-79.9192702), max_latitude: Some(-0.839573), min_latitude: Some(-3.062438), max_longitude: Some(-79.09728199999999), min_longitude: Some(-80.561928)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غاياس"), ("be", "Правінцыя Гуаяс"), ("bg", "Гуаяс"), ("bn", "জ\u{9be}মবী প\u{9cd}রোগ\u{9be}য\u{9bc}\u{9be}স প\u{9cd}রদেশ"), ("ca", "província del Guayas"), ("ccp", "𑄉\u{1112a}𑄠𑄠𑄬𑄌\u{11134}"), ("ceb", "Provincia del Guayas"), ("da", "Guayas Province"), ("de", "Provinz Guayas"), ("el", "Γουάγιας"), ("en", "Guayas"), ("es", "Provincia de Guayas"), ("fa", "استان گوایاس"), ("fi", "Guayas"), ("fr", "province du Guayas"), ("gl", "Guayas"), ("gu", "ગ\u{ac1}આયસ પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{941}आयास प\u{94d}रा\u{902}त"), ("hu", "Guayas tartomány"), ("id", "Provinsi Guayas"), ("it", "provincia del Guayas"), ("ja", "グアヤス県"), ("ka", "გუაიასის პროვინცია"), ("kn", "ಗುಯಯಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "과야스 주"), ("lt", "Guajaso provincija"), ("lv", "Gvajasas province"), ("mr", "ग\u{941}आस प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Guayas"), ("nb", "Guayas"), ("nl", "Guayas"), ("no", "Guayas"), ("pl", "Prowincja Guayas"), ("pt", "Guayas"), ("ru", "Гуаяс"), ("si", "ගය\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sl", "Provinca Guayas"), ("sv", "Guayas"), ("ta", "குயஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గుయ\u{c3e}యస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องก\u{e35}เช"), ("tr", "Guayas"), ("uk", "Гуаяс"), ("ur", "گوایاس صوبہ"), ("vi", "Tỉnh Guayas"), ("zh", "瓜亚斯省")]),
                        unofficial_name_list: ["Guayas"].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "H",
                        country_alpha2: Alpha2::EC,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.4693018), longitude: Some(-78.8169396), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيمبورازو"), ("be", "Правінцыя Чымбараса"), ("bg", "Чимборасо"), ("bn", "চিম\u{9cd}বোর\u{9be}জো প\u{9cd}রদেশ"), ("ca", "Província de Chimborazo"), ("ccp", "𑄌\u{11128}𑄟\u{11134}𑄝\u{11127}𑄢𑄎\u{1112e}"), ("ceb", "Provincia del Chimborazo"), ("da", "Chimborazo Province"), ("de", "Provinz Chimborazo"), ("el", "Τσιμποράσο"), ("en", "Chimborazo"), ("es", "Provincia de Chimborazo"), ("fa", "استان چیمبورازو"), ("fi", "Chimborazo"), ("fr", "Chimborazo"), ("gl", "Provincia de Chimborazo"), ("gu", "ચિમ\u{acd}બોરાઝો પ\u{acd}રા\u{a82}ત"), ("hi", "चि\u{902}बोराजो प\u{94d}रा\u{902}त"), ("hu", "Chimborazo tartomány"), ("id", "Provinsi Chimborazo"), ("it", "provincia del Chimborazo"), ("ja", "チンボラソ県"), ("ka", "ჩიმბორასოს პროვინცია"), ("kk", "Чимборасо"), ("kn", "ಚ\u{cbf}ಂಬೊರೊಜೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "침보라소 주"), ("lt", "Čimboraso provincija"), ("lv", "Čimbaraso province"), ("mk", "Чимборасо"), ("mr", "चि\u{902}बोराझो प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Chimborazo"), ("nb", "Chimborazo"), ("nl", "Chimborazo"), ("no", "Chimborazo"), ("pl", "Prowincja Chimborazo"), ("pt", "Chimborazo"), ("ru", "Чимборасо"), ("si", "ච\u{dd2}ම\u{dca}බෝර\u{dcf}සෝ පළ\u{dcf}ත"), ("sv", "Chimborazo"), ("ta", "சிம\u{bcd}பொற\u{bcd}சோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3f}ంబ\u{c4b}ర\u{c3e}జ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ช\u{e34}มโบราโซ"), ("tr", "Chimborazo"), ("uk", "Чімборасо"), ("ur", "چیمبوراسو صوبہ"), ("vi", "Chimborazo"), ("zh", "钦博拉索省")]),
                        unofficial_name_list: ["Chimborazo"].to_vec(),
                    }
                ),
                (
                    "I",
                    Subdivision{
                        name: "I",
                        country_alpha2: Alpha2::EC,
                        code: "I",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(0.367159), longitude: Some(-78.3842227), max_latitude: Some(0.8569121), min_latitude: Some(0.125442), max_longitude: Some(-77.828034), min_longitude: Some(-79.230362)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إمبابورا"), ("be", "Правінцыя Імбабура"), ("bg", "Имбабура"), ("bn", "ইম\u{9cd}ব\u{9be}ব\u{9c1}র\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Imbabura"), ("ccp", "𑄃\u{11128}𑄟\u{11134}𑄝𑄝\u{1112a}𑄢"), ("ceb", "Provincia de Imbabura"), ("da", "Imbabura Province"), ("de", "Provinz Imbabura"), ("el", "Ιμπαμπούρα"), ("en", "Imbabura"), ("es", "Provincia de Imbabura"), ("fa", "ایمبابورا"), ("fi", "Imbabura"), ("fr", "Imbabura"), ("gu", "ઇમ\u{acd}બાબ\u{ac1}રા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אימבאבורה"), ("hi", "इम\u{94d}बाब\u{941}रा प\u{94d}रा\u{902}त"), ("hu", "Imbabura tartomány"), ("hy", "Իմբաբուրա"), ("id", "Provinsi Imbabura"), ("it", "provincia dell’Imbabura"), ("ja", "インバブーラ県"), ("ka", "იმბაბურის პროვინცია"), ("kn", "ಇಂಬಾಬಾರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "임바부라 주"), ("lt", "Imbaburos provincija"), ("lv", "Imbaburas province"), ("mr", "इम\u{94d}बाब\u{941}रा प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Imbabura"), ("nb", "Imbabura"), ("nl", "Imbabura"), ("no", "Imbabura"), ("pl", "Prowincja Imbabura"), ("pt", "Imbabura"), ("ru", "Имбабура"), ("si", "ඉම\u{dca}බබ\u{dd4}ර\u{dcf} පළ\u{dcf}ත"), ("sv", "Imbabura"), ("ta", "இம\u{bcd}பபுர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఇంబ\u{c3e}బుర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e34}มบาบ\u{e39}รา"), ("tr", "Imbabura Province"), ("uk", "Імбабура"), ("ur", "امبابورا صوبہ"), ("vi", "Imbabura"), ("zh", "因巴布拉省")]),
                        unofficial_name_list: ["Imbabura"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::EC,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.983333), longitude: Some(-79.2), max_latitude: Some(-3.9635861), min_latitude: Some(-4.0516043), max_longitude: Some(-79.1812133), min_longitude: Some(-79.23554419999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Loja"), ("ar", "مقاطعة لوخا"), ("be", "Правінцыя Лоха"), ("bg", "Лоха"), ("bn", "লোজ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Loja"), ("ccp", "𑄣\u{1112e}𑄎"), ("ceb", "Provincia de Loja"), ("da", "Loja Province"), ("de", "Provinz Loja"), ("el", "Επαρχία Λόχα"), ("en", "Loja"), ("es", "Provincia de Loja"), ("fi", "Loja"), ("fr", "province de Loja"), ("gu", "લોજા પ\u{acd}રા\u{a82}ત"), ("hi", "लोजा प\u{94d}रा\u{902}त"), ("hu", "Loja tartomány"), ("id", "Provinsi Loja"), ("it", "provincia di Loja"), ("ja", "ロハ県"), ("ka", "ლოხის პროვინცია"), ("kn", "ಲೋಜಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로하 주"), ("lt", "Lochos provincija"), ("lv", "Lohas province"), ("mr", "लोजा प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Loja"), ("nb", "Loja"), ("nl", "Loja"), ("no", "Loja"), ("pl", "Prowincja Loja"), ("pt", "Loja"), ("ru", "Лоха"), ("si", "ලොජ\u{dcf} පළ\u{dcf}ත"), ("sv", "Loja"), ("ta", "லோஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c4b}జ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโลคา"), ("tr", "Loja province"), ("uk", "Лоха"), ("ur", "لوخا صوبہ"), ("vi", "Tỉnh Loja"), ("zh", "洛哈省")]),
                        unofficial_name_list: ["Loja"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::EC,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.7416532), longitude: Some(-79.9192702), max_latitude: Some(0.394737), min_latitude: Some(-1.92619), max_longitude: Some(-79.3954009), min_longitude: Some(-81.0774715)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مانابي"), ("be", "Правінцыя Манабі"), ("bg", "Манаби"), ("bn", "ম\u{9be}ন\u{9be}বি প\u{9cd}রদেশ"), ("ca", "Província de Manabí"), ("ccp", "𑄟𑄚𑄝\u{11128}"), ("ceb", "Provincia de Manabí"), ("cs", "Manabí"), ("da", "Manabí Province"), ("de", "Provinz Manabí"), ("el", "Μανάμπι"), ("en", "Manabí"), ("es", "Provincia de Manabí"), ("fa", "استان مانابی"), ("fi", "Manabí"), ("fr", "Manabí"), ("gu", "મનાબી પ\u{acd}રા\u{a82}ત"), ("he", "מחוז מנבי"), ("hi", "मनाबी प\u{94d}रा\u{902}त"), ("hu", "Manabí tartomány"), ("id", "Provinsi Manabí"), ("it", "provincia di Manabí"), ("ja", "マナビ県"), ("ka", "მანაბის პროვინცია"), ("kn", "ಮನಬ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마나비 주"), ("lt", "Manabi provincija"), ("lv", "Manabi province"), ("mr", "माणबी प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Manabí"), ("nb", "Manabí"), ("nl", "Manabí"), ("no", "Manabí"), ("pl", "Prowincja Manabí"), ("pt", "Manabí"), ("ru", "Манаби"), ("si", "මනබ\u{dd2} පළ\u{dcf}ත"), ("sk", "Manabí"), ("sv", "Manabí"), ("ta", "ம\u{bbe}நபி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మన\u{c3e}బ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมานาบ\u{e35}"), ("tr", "Manabi Province"), ("uk", "Манабі"), ("ur", "مانابی صوبہ"), ("vi", "Tỉnh Manabí"), ("zh", "马纳比省")]),
                        unofficial_name_list: ["Manabí"].to_vec(),
                    }
                ),
                (
                    "N",
                    Subdivision{
                        name: "N",
                        country_alpha2: Alpha2::EC,
                        code: "N",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.7371233), longitude: Some(-78.11082789999999), max_latitude: Some(0.036432), min_latitude: Some(-1.273843), max_longitude: Some(-77.049857), min_longitude: Some(-78.410088)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نابو"), ("be", "Правінцыя Напа"), ("bg", "Напо"), ("bn", "ন\u{9cd}য\u{9be}পো প\u{9cd}রদেশ"), ("ca", "Província de Napo"), ("ccp", "𑄚𑄛\u{1112e}"), ("ceb", "Provincia de Napo"), ("da", "Napo Province"), ("de", "Provinz Napo"), ("el", "Νάπο"), ("en", "Napo"), ("es", "Provincia de Napo"), ("fa", "استان ناپو"), ("fi", "Napo"), ("fr", "Napo"), ("gu", "ન\u{ac7}પો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז נפו"), ("hi", "नापो प\u{94d}रा\u{902}त"), ("hu", "Napo tartomány"), ("id", "Provinsi Napo"), ("it", "provincia del Napo"), ("ja", "ナポ県"), ("ka", "ნაპოს პროვინცია"), ("kn", "ನೇಪೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "나포 주"), ("lt", "Napo provincija"), ("lv", "Napo province"), ("mr", "नापो प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Napo"), ("nb", "Napo"), ("nl", "Napo"), ("no", "Napo"), ("pl", "Prowincja Napo"), ("pt", "Napo"), ("ro", "Provincia Napo"), ("ru", "Напо"), ("si", "න\u{dcf}පෝ පළ\u{dcf}ත"), ("sv", "Napo"), ("ta", "ந\u{bbe}ப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c3e}ప\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนาโป"), ("tr", "Napo Province"), ("uk", "Напо"), ("ur", "ناپو صوبہ"), ("vi", "Tỉnh Napo"), ("zh", "纳波省")]),
                        unofficial_name_list: ["Napo"].to_vec(),
                    }
                ),
                (
                    "O",
                    Subdivision{
                        name: "O",
                        country_alpha2: Alpha2::EC,
                        code: "O",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.4454787), longitude: Some(-79.8296743), max_latitude: Some(-3.0484269), min_latitude: Some(-3.90139), max_longitude: Some(-79.358108), min_longitude: Some(-80.30034599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إل أورو"), ("be", "Правінцыя Эль-Ора"), ("bg", "Ел Оро"), ("bn", "এল ওরো প\u{9cd}রদেশ"), ("ca", "Província d’El Oro"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄃\u{11127}𑄢\u{1112e}"), ("ceb", "Provincia de El Oro"), ("da", "El Oro Province"), ("de", "Provinz El Oro"), ("el", "Ελ Όρο"), ("en", "El Oro"), ("es", "Provincia de El Oro"), ("fa", "استان ال اورو"), ("fi", "El Oro"), ("fr", "El Oro"), ("gu", "અલ ઓરો પ\u{acd}રા\u{a82}ત"), ("hi", "एल ओरो प\u{94d}रा\u{902}त"), ("hu", "El Oro tartomány"), ("hy", "Էլ Օրո"), ("id", "Provinsi El Oro"), ("it", "provincia di El Oro"), ("ja", "エル・オロ県"), ("ka", "ელ-ოროს პროვინცია"), ("kn", "ಎಲ\u{ccd} ಓರೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엘오로 주"), ("lt", "El Oro provincija"), ("lv", "Eloro province"), ("mr", "एल ओरो प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran El Oro"), ("nb", "El Oro"), ("nl", "El Oro"), ("no", "El Oro"), ("pl", "Prowincja El Oro"), ("pt", "El Oro"), ("ru", "Эль-Оро"), ("si", "එල\u{dca} ඔරෝ පළ\u{dcf}ත"), ("sv", "El Oro"), ("ta", "எல\u{bcd} ஒரு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎల\u{c4d} ఓర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอลโอโร"), ("tr", "El Oro Province"), ("uk", "Ель-Оро"), ("ur", "ایل اورو صوبہ"), ("vi", "Tỉnh El Oro"), ("zh", "埃爾奧羅省")]),
                        unofficial_name_list: ["El Oro"].to_vec(),
                    }
                ),
                (
                    "P",
                    Subdivision{
                        name: "P",
                        country_alpha2: Alpha2::EC,
                        code: "P",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.1464847), longitude: Some(-78.4751945), max_latitude: Some(0.344864), min_latitude: Some(-0.714463), max_longitude: Some(-77.83552499999999), min_longitude: Some(-79.368635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيتشينتشا"), ("be", "Правінцыя Пічынча"), ("bg", "Пичинча"), ("bn", "পিচিঞ\u{9cd}চ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Pichincha"), ("ccp", "𑄛\u{11128}𑄌\u{11128}𑄚\u{11134}𑄌"), ("ceb", "Provincia de Pichincha"), ("da", "Pichincha Province"), ("de", "Provinz Pichincha"), ("el", "Πιτσίντσα"), ("en", "Pichincha"), ("es", "Provincia de Pichincha"), ("fa", "استان پیچینچا"), ("fi", "Pichincha"), ("fr", "province de Pichincha"), ("gl", "Provincia de Pichincha"), ("gu", "પિચિન\u{acd}ચા પ\u{acd}રા\u{a82}ત"), ("hi", "पिचि\u{902}चा प\u{94d}रा\u{902}त"), ("hu", "Pichincha tartomány"), ("hy", "Պիչինչա"), ("id", "Provinsi Pichincha"), ("it", "provincia del Pichincha"), ("ja", "ピチンチャ県"), ("ka", "პიჩინჩის პროვინცია"), ("kn", "ಪ\u{cbf}ಚ\u{cbf}ಂಚಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "피친차 주"), ("lt", "Pičinčos provincija"), ("lv", "Pičinčas province"), ("mr", "पिचिनचा प\u{94d}रा\u{902}त"), ("ms", "Pichincha Province"), ("nb", "Pichincha"), ("nl", "Pichincha"), ("no", "Pichincha"), ("pl", "Prowincja Pichincha"), ("pt", "Pichincha"), ("ru", "Пичинча"), ("si", "ප\u{dd2}ච\u{dd2}න\u{dca}ච\u{dcf} පළ\u{dcf}ත"), ("sv", "Pichincha"), ("ta", "பிசைஞ\u{bcd}ச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3f}చ\u{c3f}ంచ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e35}ช\u{e34}นชา"), ("tr", "Pichincha ili"), ("uk", "Пічінча"), ("ur", "پیچینچا صوبہ"), ("vi", "Tỉnh Pichincha"), ("zh", "皮欽查省")]),
                        unofficial_name_list: ["Pichincha"].to_vec(),
                    }
                ),
                (
                    "R",
                    Subdivision{
                        name: "R",
                        country_alpha2: Alpha2::EC,
                        code: "R",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.4191719), longitude: Some(-79.4703885), max_latitude: Some(-0.5303), min_latitude: Some(-2.1302071), max_longitude: Some(-79.080353), min_longitude: Some(-79.8767999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوس ريوس"), ("be", "Правінцыя Лос-Рыяс"), ("bg", "Лос Риос"), ("bn", "লস রিওস প\u{9cd}রদেশ"), ("ca", "Província de Los Ríos"), ("ccp", "𑄣\u{11127}𑄌\u{11134} 𑄢\u{11128}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Los Ríos"), ("da", "Los Ríos Province"), ("de", "Provinz Los Ríos"), ("el", "Επαρχία Λος Ρίος"), ("en", "Los Ríos"), ("es", "Los Ríos"), ("fa", "استان لوس ریوس"), ("fi", "Los Ríos"), ("fr", "Los Ríos"), ("gu", "લોસ રિયોસ પ\u{acd}રા\u{a82}ત"), ("hi", "लॉस रिओस प\u{94d}रा\u{902}त"), ("hu", "Los Ríos tartomány"), ("hy", "Լոս Ռիո"), ("id", "Provinsi Los Ríos"), ("it", "provincia di Los Ríos"), ("ja", "ロス・リオス県"), ("ka", "ლოს-რიოსის პროვინცია"), ("kn", "ಲಾಸ\u{ccd} ರೈಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로스리오스 주"), ("lt", "Los Rioso provincija"), ("lv", "Losriosas province"), ("mr", "लॉस रिओस प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Los Ríos"), ("nb", "Los Ríos"), ("nl", "Los Ríos"), ("no", "Los Ríos"), ("pl", "Prowincja Los Ríos"), ("pt", "Los Ríos"), ("ru", "Лос-Риос"), ("si", "ලොස\u{dca} රයොස\u{dca} පළ\u{dcf}ත"), ("sv", "Los Ríos"), ("ta", "ல\u{bbe}ஸ\u{bcd} ரியோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}స\u{c4d} ర\u{c3f}య\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาเซ\u{e35}ยนเตป"), ("tr", "Los Rios Province"), ("uk", "Лос-Ріос"), ("ur", "لاس ریوس صوبہ"), ("vi", "Los Ríos"), ("zh", "洛斯里奧斯省")]),
                        unofficial_name_list: ["Los Ríos"].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "S",
                        country_alpha2: Alpha2::EC,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.5628216), longitude: Some(-78.11082789999999), max_latitude: Some(-1.443362), min_latitude: Some(-3.823733), max_longitude: Some(-76.73908999999999), min_longitude: Some(-78.969154)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مورونا سانتياغو"), ("bg", "Морона Сантяго"), ("bn", "মোরোন\u{9be}-স\u{9be}ন\u{9cd}টিয\u{9bc}\u{9be}গো প\u{9cd}রদেশ"), ("ca", "Província de Morona-Santiago"), ("ccp", "𑄟\u{11127}𑄢\u{1112e}𑄚-𑄥𑄚\u{11134}𑄑\u{11128}𑄠𑄉\u{1112e}"), ("ceb", "Provincia de Morona-Santiago"), ("da", "Morona-Santiago Province"), ("de", "Provinz Morona Santiago"), ("el", "Μορόνα-Σαντιάγκο"), ("en", "Morona-Santiago"), ("es", "Provincia de Morona Santiago"), ("fa", "مورونا-سانتیاگو"), ("fi", "Morona-Santiago"), ("fr", "Morona-Santiago"), ("gu", "મોરોના-સાન\u{acd}તિયાગો પ\u{acd}રા\u{a82}ત"), ("hi", "मोरोना-स\u{948}\u{902}टियागो प\u{94d}रा\u{902}त"), ("hu", "Morona Santiago tartomány"), ("hy", "Մորոնա Սանտյագո"), ("id", "Provinsi Morona-Santiago"), ("it", "provincia di Morona-Santiago"), ("ja", "モロナ・サンティアゴ県"), ("ka", "მორონა-სანტიაგოს პროვინცია"), ("kn", "ಮೊರೊನಾ-ಸ\u{ccd}ಯಾಂಟ\u{cbf}ಯಾಗೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "모로나산티아고 주"), ("lt", "Morona Santjago provincija"), ("lv", "Moronas Santjago province"), ("mr", "मोरोना-सा\u{902}तियागो प\u{94d}रा\u{902}त"), ("ms", "Morona-Santiago Province"), ("nb", "Morona Santiago"), ("nl", "Morona-Santiago"), ("no", "Morona Santiago"), ("pl", "Prowincja Morona-Santiago"), ("pt", "Morona-Santiago"), ("ru", "Морона-Сантьяго"), ("si", "මොරෝන\u{dcf}-සන\u{dca}ත\u{dd2}ය\u{dcf}ගෝ පළ\u{dcf}ත"), ("sv", "Morona Santiago"), ("ta", "மோரோன\u{bbe} -ச\u{bbe}ண\u{bcd}டிய\u{bbe}கோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c4b}ర\u{c4b}న\u{c3e}-స\u{c3e}ంట\u{c3f}య\u{c3e}గ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "โมโรนา-ซานต\u{e34}เอโก"), ("tr", "Morona-Santiago"), ("uk", "Морона-Сантьяго"), ("ur", "مورونا-سانتیاگو صوبہ"), ("vi", "Tỉnh Morona-Santiago"), ("zh", "莫罗纳-圣地亚哥省")]),
                        unofficial_name_list: ["Morona-Santiago"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::EC,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.2205563), longitude: Some(-79.29021329999999), max_latitude: Some(0.123351), min_latitude: Some(-0.6861889999999999), max_longitude: Some(-78.73494099999999), min_longitude: Some(-79.623871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتو دومينغو دا لوس تساتشيلاس"), ("be", "Правінцыя Санта-Дамінга-дэ-лос-Тсачылас"), ("bg", "Санто Доминго де лос Цачилас"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}তো ডোমিঙ\u{9cd}গ দে লস স\u{9be}চিল\u{9be}স প\u{9cd}রদেশ"), ("ccp", "𑄥𑄚\u{11134}𑄑\u{1112e} 𑄓\u{1112e}𑄟\u{11128}𑄚\u{11134}𑄉\u{1112e} 𑄓𑄬 𑄣\u{11127}𑄌\u{11134} 𑄥𑄌\u{11128}𑄣𑄌\u{11134}"), ("ceb", "Provincia de Santo Domingo de los Tsáchilas"), ("da", "Santo Domingo de los Tsáchilas Province"), ("de", "Provinz Santo Domingo de los Tsáchilas"), ("el", "Σάντο Δομίγκο δε λος Τσατσίλας"), ("en", "Santo Domingo de los Tsáchilas"), ("es", "Provincia de Santo Domingo de los Tsáchilas"), ("fi", "Santo Domingo de los Tsáchilas"), ("fr", "Santo Domingo de los Tsáchilas"), ("gu", "સાન\u{acd}ટો ડોમિ\u{a82}ગો દ\u{ac7} લોસ સાચિલાસ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}\u{902}टो डोमि\u{902}गो डी लोस स\u{948}किलास प\u{94d}रा\u{902}त"), ("hy", "Սանտա Դոմինգո դե լոս Տսաչիլաս"), ("id", "Provinsi Santo Domingo de los Tsáchilas"), ("it", "provincia di Santo Domingo de los Tsáchilas"), ("ja", "サント・ドミンゴ・デ・ロス・ツァチラス県"), ("ka", "სანტო-დომინგო-დე-ლოს-ტსაჩილასის პროვინცია"), ("kn", "ಸ\u{ccd}ಯಾಂಟೋ ಡೊಮ\u{cbf}ಂಗೊ ಡ\u{cbf} ಲಾಸ\u{ccd} ತ\u{ccd}ಸಾಚ\u{cbf}ಲಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산토도밍고데로스차칠라스 주"), ("lt", "Santo Domingo de los Cačilaso provincija"), ("lv", "Santodomingo de los Cačilasas province"), ("mr", "स\u{901}टो डोमि\u{902}गो डी लॉस त\u{94d}साचिलास प\u{94d}रा\u{902}त"), ("ms", "Daerah Santo Domingo de los Tsáchilas"), ("nb", "Santo Domingo de las Tsachilas provins"), ("nl", "Santo Domingo de los Tsáchilas"), ("no", "Santo Domingo de las Tsachilas provins"), ("pl", "Prowincja Santo Domingo de los Tsáchilas"), ("pt", "Santo Domingo de los Tsáchilas"), ("ru", "Санто-Доминго-де-лос-Тсачилас"), ("si", "සැන\u{dca}ටෝ ඩොම\u{dd2}න\u{dca}ගෝ ඩ\u{dd2} ලොස\u{dca} ට\u{dca}ස\u{dcf}ච\u{dd2}ල\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Santo Domingo de los Tsáchilas"), ("ta", "சந\u{bcd}தோ டொமிங\u{bcd}கோ டி ல\u{bbe}ஸ\u{bcd} டச\u{bcd}சிலஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c4b} డ\u{c3e}మ\u{c3f}ంగ\u{c4b} ద ల\u{c3e}స\u{c4d} స\u{c3e}చ\u{c3f}ల\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานโตโดม\u{e34}งโกเดลาค\u{e31}ลซาดา"), ("tr", "Santo Domingo de los Tsachilas Province"), ("uk", "Санто-Домінго-де-лос-Тсачилас"), ("ur", "سانتو دومنگو دے لاس تسآچیلاس صوبہ"), ("vi", "Tỉnh Santo Domingo de los Tsáchilas"), ("zh", "聖多明各-德洛斯查奇拉斯省")]),
                        unofficial_name_list: ["Santo Domingo de los Tsachilas"].to_vec(),
                    }
                ),
                (
                    "SE",
                    Subdivision{
                        name: "SE",
                        country_alpha2: Alpha2::EC,
                        code: "SE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.1934804), longitude: Some(-80.54384499999999), max_latitude: Some(-1.647962), min_latitude: Some(-2.508378), max_longitude: Some(-80.2266889), min_longitude: Some(-81.0067736)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سانتا إيلينا"), ("be", "Правінцыя Санта-Элена"), ("bg", "Света Елена"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} এলেন\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄃𑄬𑄣𑄬𑄚"), ("ceb", "Provincia de Santa Elena"), ("da", "Santa Elena Province"), ("de", "Santa Elena"), ("el", "Επαρχία Σάντα Έλενα"), ("en", "Santa Elena"), ("es", "Provincia de Santa Elena"), ("fa", "استان سانتا النا"), ("fi", "Santa Elena"), ("fr", "Santa Elena"), ("gu", "સા\u{a82}તા એલ\u{ac7}ના પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}\u{902}टा एल\u{947}ना प\u{94d}रा\u{902}त"), ("id", "Provinsi Santa Elena"), ("it", "provincia di Santa Elena"), ("ja", "サンタ・エレーナ県"), ("ka", "სანტა-ელენას პროვინცია"), ("kn", "ಸಾಂಟಾ ಎಲ\u{cc6}ನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산타엘레나 주"), ("lt", "Santa Elenos provincija"), ("lv", "Santaelenas province"), ("mk", "Санта Елена"), ("mr", "सा\u{902}ता एल\u{947}ना प\u{94d}रा\u{902}त"), ("ms", "Santa Elena Province"), ("nb", "Santa Elena provins"), ("nl", "Santa Elena"), ("no", "Santa Elena provins"), ("pl", "Prowincja Santa Elena"), ("pt", "Santa Elena (província)"), ("ru", "Санта-Элена"), ("si", "සැන\u{dca}ට\u{dcf} එලේන\u{dcf} පළ\u{dcf}ත"), ("sv", "Santa Elena"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} எலென\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3e} ఎల\u{c40}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานตา เอเลนา"), ("tr", "Santa Elena Province"), ("uk", "Санта-Елена"), ("ur", "سانتا الینا صوبہ"), ("vi", "Tỉnh Santa Elena"), ("zh", "聖埃倫娜省")]),
                        unofficial_name_list: ["Santa Elena"].to_vec(),
                    }
                ),
                (
                    "T",
                    Subdivision{
                        name: "T",
                        country_alpha2: Alpha2::EC,
                        code: "T",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.2635284), longitude: Some(-78.5660852), max_latitude: Some(-0.9803009999999999), min_latitude: Some(-1.507969), max_longitude: Some(-78.144211), min_longitude: Some(-78.93395199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تونغوراهوا"), ("be", "Правінцыя Тунгурауа"), ("bg", "Тунгурауа"), ("bn", "ত\u{9be}ঙ\u{9cd}গ\u{9c1}র\u{9be}হ\u{9c1}য\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Tungurahua"), ("ccp", "𑄑\u{1112a}𑄋\u{11134}𑄉\u{1112a}𑄢𑄦\u{1112a}𑄠"), ("ceb", "Provincia del Tungurahua"), ("da", "Tungurahua Province"), ("de", "Provinz Tungurahua"), ("el", "Τουνγκουράουα"), ("en", "Tungurahua"), ("es", "Provincia de Tungurahua"), ("fa", "استان تونگوراهوا"), ("fi", "Tungurahua"), ("fr", "Tungurahua"), ("gu", "ત\u{ac1}\u{a82}ગ\u{ac1}રાહ\u{ac1}આ પ\u{acd}રા\u{a82}ત"), ("hi", "त\u{941}\u{902}ग\u{941}राह\u{941}आ प\u{94d}रा\u{902}त"), ("hy", "Տունգուրահուա"), ("id", "Provinsi Tungurahua"), ("it", "provincia del Tungurahua"), ("ja", "トゥングラワ県"), ("ka", "ტუნგურაუის პროვინცია"), ("kn", "ತುಂಗ\u{cc2}ರಾಹು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "퉁구라우아 주"), ("lt", "Tungurahvos provincija"), ("lv", "Tunguravas province"), ("mr", "त\u{941}\u{902}ग\u{941}राह\u{941}आ प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Tungurahua"), ("nb", "Tungurahua"), ("nl", "Tungurahua"), ("no", "Tungurahua"), ("pl", "Prowincja Tungurahua"), ("pt", "Tungurahua"), ("ru", "Тунгурауа"), ("si", "ට\u{dd4}න\u{dca}ග\u{dd4}රහ\u{dd4}ආ පළ\u{dcf}ත"), ("sv", "Tungurahua"), ("ta", "தங\u{bcd}குறகுஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తుంగుర\u{c3e}హువ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ต\u{e31}นก\u{e39}ราฮ\u{e31}ว"), ("tr", "Tungurahua Province"), ("uk", "Тунґурауа"), ("ur", "تونگوراہوا صوبہ"), ("vi", "Tungurahua"), ("zh", "通古拉瓦省")]),
                        unofficial_name_list: ["Tungurahua"].to_vec(),
                    }
                ),
                (
                    "U",
                    Subdivision{
                        name: "U",
                        country_alpha2: Alpha2::EC,
                        code: "U",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.1452276), longitude: Some(-76.2710833), max_latitude: Some(0.651662), min_latitude: Some(-0.648883), max_longitude: Some(-75.2348859), min_longitude: Some(-77.9738229)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سوكومبيوس"), ("be", "Правінцыя Сукумбіас"), ("bg", "Сукумбиос"), ("bn", "স\u{9c1}ক\u{9c1}ম\u{9cd}বিওস প\u{9cd}রদেশ"), ("ca", "Província de Sucumbíos"), ("ccp", "𑄥𑄇\u{1112a}𑄟\u{11134}𑄝𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Sucumbíos"), ("da", "Sucumbíos Province"), ("de", "Provinz Sucumbíos"), ("el", "Σουκούμπιος"), ("en", "Sucumbíos"), ("es", "Provincia de Sucumbíos"), ("fa", "ساکومبیوس"), ("fi", "Sucumbíos"), ("fr", "Sucumbíos"), ("gu", "સ\u{ac1}ક\u{ac1}મબિઓસ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סוקומביוס"), ("hi", "स\u{941}क\u{941}म\u{94d}बियोस प\u{94d}रा\u{902}त"), ("id", "Provinsi Sucumbíos"), ("it", "provincia di Sucumbíos"), ("ja", "スクンビオス県"), ("ka", "სუკუმბიოსის პროვინცია"), ("kn", "ಸುಕುಂಬ\u{cbf}ಯಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "수쿰비오스 주"), ("lt", "Sukumbijoso provincija"), ("lv", "Sukumbiosas province"), ("mr", "स\u{941}कम\u{94d}बाइओस प\u{94d}रा\u{902}त"), ("ms", "Wilayah Sucumbíos"), ("nb", "Sucumbíos"), ("nl", "Sucumbíos"), ("no", "Sucumbíos"), ("pl", "Prowincja Sucumbíos"), ("pt", "Sucumbíos"), ("ru", "Сукумбиос"), ("si", "ස\u{dd4}ක\u{dd4}ම\u{dca}බ\u{dd2}යෝස\u{dca} පළ\u{dcf}ත"), ("sv", "Sucumbíos"), ("ta", "சுசும\u{bcd}பியஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సుకుంబ\u{c3f}య\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e39}ค\u{e31}มบ\u{e34}ออส"), ("tr", "Sucumbios Province"), ("uk", "Сукумбіос"), ("ur", "سوکومبیوس صوبہ"), ("vi", "Tỉnh Sucumbíos"), ("zh", "苏昆比奥斯省")]),
                        unofficial_name_list: ["Sucumbíos"].to_vec(),
                    }
                ),
                (
                    "W",
                    Subdivision{
                        name: "W",
                        country_alpha2: Alpha2::EC,
                        code: "W",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.8292784), longitude: Some(-90.9820668), max_latitude: Some(1.6815752), min_latitude: Some(-1.4115146), max_longitude: Some(-89.24079549999999), min_longitude: Some(-92.0084893)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غالاباغوس"), ("be", "Правінцыя Галапагос"), ("bn", "গ\u{9be}ল\u{9be}প\u{9be}গোস প\u{9cd}রদেশ"), ("ca", "Província de Galápagos"), ("ccp", "𑄉\u{11133}𑄠𑄣𑄛\u{11134}𑄛𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Galápagos"), ("da", "Galápagos Province"), ("de", "Provinz Galápagos"), ("el", "Γκαλαπάγκος"), ("en", "Galápagos"), ("es", "Provincia de Galápagos"), ("fa", "استان گالاپاگوس"), ("fi", "Galápagosin lääni"), ("fr", "province de Galápagos"), ("gu", "ગાલાપાગોસ પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{948}लापागोस प\u{94d}रा\u{902}त"), ("id", "Provinsi Galápagos"), ("it", "provincia delle Galápagos"), ("ja", "ガラパゴス県"), ("ka", "გალაპაგოსის პროვინცია"), ("kn", "ಗ\u{ccd}ಯಾಲಪಗೋಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "갈라파고스 주"), ("lt", "Galapagų provincija"), ("lv", "Galapagu province"), ("mr", "ग\u{945}लापागोस प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Galápagos"), ("nb", "Galapagos provins"), ("nl", "Galápagos"), ("no", "Galapagos provins"), ("pl", "Prowincja Galápagos"), ("pt", "Galápagos"), ("ro", "Provincia Galápagos"), ("ru", "Галапагос"), ("si", "ගලපග\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Galápagos"), ("ta", "கலப\u{bcd}பகோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3e}ల\u{c3e}ప\u{c3e}గ\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาลาปาโกส"), ("tr", "Galapagos Province"), ("uk", "Галапагос"), ("ur", "گالاپاگوس صوبہ"), ("vi", "Tỉnh Galápagos"), ("zh", "加拉帕戈斯省")]),
                        unofficial_name_list: ["Galápagos"].to_vec(),
                    }
                ),
                (
                    "X",
                    Subdivision{
                        name: "X",
                        country_alpha2: Alpha2::EC,
                        code: "X",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-0.8139666999999999), longitude: Some(-78.9288242), max_latitude: Some(-0.324243), min_latitude: Some(-1.2108109), max_longitude: Some(-78.370667), min_longitude: Some(-79.329689)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوتوباكسي"), ("be", "Правінцыя Катапаксі"), ("bg", "Котопакси"), ("bn", "কটোপ\u{9be}ক\u{9cd}সি প\u{9cd}রদেশ"), ("ca", "Província de Cotopaxi"), ("ccp", "𑄇\u{11127}𑄑\u{1112e}𑄛𑄇\u{11134}𑄥\u{11128}"), ("ceb", "Provincia de Cotopaxi"), ("da", "Cotopaxi Province"), ("de", "Provinz Cotopaxi"), ("el", "Επαρχία Κοτοπάξι"), ("en", "Cotopaxi"), ("es", "Provincia de Cotopaxi"), ("fa", "استان کوتوپاکسی"), ("fi", "Cotopaxi"), ("fr", "Cotopaxi"), ("gu", "કોટોપ\u{ac7}ક\u{acd}સિ પ\u{acd}રા\u{a82}ત"), ("hi", "कोटोप\u{948}क\u{94d}सी प\u{94d}रा\u{902}त"), ("hu", "Cotopaxi tartomány"), ("hy", "Կոտոպախի"), ("id", "Provinsi Cotopaxi"), ("it", "provincia del Cotopaxi"), ("ja", "コトパクシ県"), ("ka", "კოტოპახის პროვინცია"), ("kn", "ಕೊಟೊಪಾಕ\u{ccd}ಸ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코토팍시 주"), ("lt", "Kotopaksio provincija"), ("lv", "Kotopaksi province"), ("mr", "कोत\u{94d}प\u{945}क\u{94d}सिको प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Cotopaxi"), ("nb", "Cotopaxi"), ("nl", "Cotopaxi"), ("no", "Cotopaxi"), ("pl", "Prowincja Cotopaxi"), ("pt", "Cotopaxi"), ("ru", "Котопахи"), ("si", "කොටොපක\u{dca}ස\u{dd2} පළ\u{dcf}ත"), ("sv", "Cotopaxi"), ("ta", "கோடோப\u{bbe}க\u{bcd}சி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}ట\u{c4b}ప\u{c3e}క\u{c4d}స\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโคโตพอกซ\u{e35}\u{e48}"), ("tr", "Cotopaxi Province"), ("uk", "Котопахі"), ("ur", "کوتوپازی صوبہ"), ("vi", "Cotopaxi"), ("zh", "科托帕希省")]),
                        unofficial_name_list: ["Cotopaxi"].to_vec(),
                    }
                ),
                (
                    "Y",
                    Subdivision{
                        name: "Y",
                        country_alpha2: Alpha2::EC,
                        code: "Y",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.6332034), longitude: Some(-77.010385), max_latitude: Some(-1.083427), min_latitude: Some(-2.6300901), max_longitude: Some(-75.576691), min_longitude: Some(-78.30367509999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باستازا"), ("be", "Правінцыя Пастаса"), ("bg", "Пастаса"), ("bn", "প\u{9be}স\u{9cd}ত\u{9be}জ\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Pastaza"), ("ccp", "𑄛𑄌\u{11134}𑄑𑄎"), ("ceb", "Provincia del Pastaza"), ("da", "Pastaza Province"), ("de", "Provinz Pastaza"), ("el", "Παστάσα"), ("en", "Pastaza"), ("es", "Provincia de Pastaza"), ("fa", "استان پاستاسا"), ("fi", "Pastaza"), ("fr", "Pastaza"), ("gu", "પાસ\u{acd}ટાઝા પ\u{acd}રા\u{a82}ત"), ("hi", "पासताज\u{93c}ा प\u{94d}रा\u{902}त"), ("hy", "Պաստասա"), ("id", "Provinsi Pastaza"), ("it", "provincia del Pastaza"), ("ja", "パスタサ県"), ("ka", "პასტასის პროვინცია"), ("kn", "ಪಾಸ\u{ccd}ಜಾಜಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파스타사 주"), ("lt", "Pastasos provincija"), ("lv", "Pastasas province"), ("mr", "प\u{947}टाटा प\u{94d}रा\u{902}त"), ("ms", "Pentadbiran Pastaza"), ("nb", "Pastaza"), ("nl", "Pastaza"), ("no", "Pastaza"), ("pl", "Prowincja Pastaza"), ("pt", "Pastaza"), ("ro", "Provincia Pastaza"), ("ru", "Пастаса"), ("si", "පස\u{dca}ටස\u{dcf} කල\u{dcf}පය"), ("sv", "Pastaza"), ("ta", "பஸ\u{bcd}டஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}స\u{c4d}ట\u{c3e}జ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดป\u{e31}สตาซา"), ("tr", "Pastaza Province"), ("uk", "Пастаса"), ("ur", "پاستاسا صوبہ"), ("vi", "Tỉnh Pastaza"), ("zh", "帕斯塔薩省")]),
                        unofficial_name_list: ["Pastaza"].to_vec(),
                    }
                ),
                (
                    "Z",
                    Subdivision{
                        name: "Z",
                        country_alpha2: Alpha2::EC,
                        code: "Z",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.1535434), longitude: Some(-78.8382644), max_latitude: Some(-3.3357891), min_latitude: Some(-4.998823), max_longitude: Some(-78.3993), min_longitude: Some(-79.427795)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زامورا تشينتشيبه"), ("be", "Правінцыя Самора-Чынчыпэ"), ("bg", "Самора Чинчипе"), ("bn", "জ\u{9be}মোর\u{9be}-চিঞ\u{9cd}চিপে প\u{9cd}রদেশ"), ("ca", "Província de Zamora-Chinchipe"), ("ccp", "𑄎𑄟\u{1112e}𑄢-𑄌\u{11128}𑄚\u{11134}𑄌\u{11128}𑄛𑄬"), ("ceb", "Provincia de Zamora-Chinchipe"), ("da", "Zamora-Chinchipe Province"), ("de", "Provinz Zamora Chinchipe"), ("el", "Επαρχία Σαμόρα Τσιντσίπε"), ("en", "Zamora-Chinchipe"), ("es", "Provincia de Zamora Chinchipe"), ("fa", "زامورا-چینچیپ"), ("fi", "Zamora-Chinchipe"), ("fr", "Zamora-Chinchipe"), ("gu", "ઝામોરા-ચિ\u{a82}નચિપી પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{93c}मोरा-चि\u{902}चिप\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Zamora-Chinchipe"), ("it", "provincia di Zamora-Chinchipe"), ("ja", "サモラ・チンチペ県"), ("ka", "სამორა-ჩინჩიპეს პროვინცია"), ("kn", "ಝಮೊರಾ-ಚ\u{cbf}ಂಚೈಪ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사모라친치페 주"), ("lt", "Samoros-Činčipės provincija"), ("lv", "Samoras Činčipes province"), ("mr", "ज\u{93c}मोरा-चि\u{902}चिपी प\u{94d}रा\u{902}त"), ("ms", "Zamora-Chinchipe Province"), ("nb", "Zamora Chinchipe"), ("nl", "Zamora-Chinchipe"), ("no", "Zamora Chinchipe"), ("pl", "Prowincja Zamora-Chinchipe"), ("pt", "Zamora-Chinchipe"), ("ru", "Самора-Чинчипе"), ("si", "සමෝර\u{dcf}-ච\u{dd2}න\u{dca}ච\u{dd2}පේ පළ\u{dcf}ත"), ("sv", "Zamora Chinchipe"), ("ta", "சமோர\u{bbe} -சிஞ\u{bcd}சிப\u{bcd}பெ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జమ\u{c4b}ర\u{c3e}-చ\u{c3f}ంచ\u{c3f}ప\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ซาโมรา-ช\u{e34}นช\u{e35}ป\u{e35}\u{e49}"), ("tr", "Zamora-Chinchip Province"), ("uk", "Самора-Чинчипе"), ("ur", "سامورا-چینچیپے صوبہ"), ("vi", "Tỉnh Zamora-Chinchipe"), ("zh", "萨莫拉-钦奇佩省")]),
                        unofficial_name_list: ["Zamora-Chinchipe"].to_vec(),
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
#[cfg(feature = "ec")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::EC,
        alpha3: Alpha3::ECU,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 593,
        currency_code: "USD",
        gec: Some(GEC::EC),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("ECU"),
        iso_long_name: "The Republic of Ecuador",
        iso_short_name: "Ecuador",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Ecuadorean"),
        number: "218",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "EC",
        unofficial_name_list: ["Ecuador", "Équateur", "エクアドル"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Ecuador"),
            ("af", "Ecuador"),
            ("ak", "Ecuador"),
            ("am", "ጒጣ፦ሴ"),
            ("an", "Ecuador"),
            ("ar", "الإكوادور"),
            ("as", "ইকোৱেড'ৰ"),
            ("ay", "Ecuador"),
            ("az", "Ekvador"),
            ("ba", "Ecuador"),
            ("be", "Эквадор"),
            ("bg", "Еквадор"),
            ("bi", "Ecuador"),
            ("bn", "ইক\u{9c1}য়েডোর"),
            ("bn_IN", "ইক\u{9c1}য়েডোর"),
            ("br", "Ekuador"),
            ("bs", "Ekvador"),
            ("ca", "Equador"),
            ("ce", "Эквадор"),
            ("ch", "Ecuador"),
            ("cs", "Ekvádor"),
            ("cv", "Эквадор"),
            ("cy", "Ecwador"),
            ("da", "Ecuador"),
            ("de", "Ecuador"),
            ("dv", "އ\u{7a8}ކ\u{7aa}އ\u{7ac}ޑ\u{7af}ރ\u{7aa}"),
            ("dz", "ཨ\u{f72}་ཀ\u{f74}ཡ\u{f7a}་ཌ\u{f7c}ར།"),
            ("ee", "Ecuador"),
            ("el", "Ισημερινός"),
            ("en", "Ecuador"),
            ("eo", "Ekvadoro"),
            ("es", "Ecuador"),
            ("et", "Ecuador"),
            ("eu", "Ekuador"),
            ("fa", "اکوادور"),
            ("ff", "Ecuador"),
            ("fi", "Ecuador"),
            ("fo", "Ekvador"),
            ("fr", "Équateur"),
            ("fy", "Ekwador"),
            ("ga", "Eacuadór"),
            ("gl", "Ecuador"),
            ("gn", "Ecuador"),
            ("gu", "ઇક\u{acd}વાડોર"),
            ("gv", "Ecuador"),
            ("ha", "Ecuador"),
            ("he", "אקוודור"),
            ("hi", "ईक\u{94d}वाडोर"),
            ("hr", "Ekvador"),
            ("ht", "Ekwatè"),
            ("hu", "Ecuador"),
            ("hy", "Էկվադոր"),
            ("ia", "Ecuador"),
            ("id", "Ekuador"),
            ("io", "Equador"),
            ("is", "Ekvador"),
            ("it", "Ecuador"),
            ("iu", "Ecuador"),
            ("ja", "エクアドル"),
            ("ka", "ეკვადორი"),
            ("ki", "Ecuador"),
            ("kk", "Эквадор"),
            ("kl", "Ecuador"),
            ("km", "អេក\u{17d2}វាឌ\u{17d0}រ"),
            ("kn", "ಈಕ\u{ccd}ವ\u{cc6}ಡಾರ\u{ccd}"),
            ("ko", "에콰도르"),
            ("ku", "Ekvator"),
            ("kv", "Ecuador"),
            ("kw", "Pow Ekwadorel"),
            ("ky", "Эквадор"),
            ("lo", "ປະເທດເອກ\u{ebb}ວເຕ\u{eb5}"),
            ("lt", "Ekvadoras"),
            ("lv", "Ekvadora"),
            ("mi", "Ekuatoa"),
            ("mk", "Еквадор"),
            ("ml", "ഇക\u{d4d}വഡോര\u{d4d}\u{200d}"),
            ("mn", "Эквадор"),
            ("mr", "इक\u{94d}व\u{947}डॉर"),
            ("ms", "Ecuador"),
            ("mt", "Ecuador"),
            (
                "my",
                "အ\u{102e}က\u{103d}ေဒေါန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ekwador"),
            ("nb", "Ecuador"),
            ("ne", "इक\u{94d}व\u{947}डर"),
            ("nl", "Ecuador"),
            ("nn", "Ecuador"),
            ("nv", "Ecuador"),
            ("oc", "Eqüator"),
            ("or", "ଏକ\u{b4d}ଯ\u{b41}ଡୋର"),
            ("pa", "ਏਕਵ\u{a47}ਡਰ"),
            ("pi", "एक\u{94d}वाडोर"),
            ("pl", "Ekwador"),
            ("ps", "اېکوادور"),
            ("pt", "Equador"),
            ("pt_BR", "Equador"),
            ("ro", "Ecuador"),
            ("ru", "Эквадор"),
            ("rw", "Ekwadoro"),
            ("sc", "Ècuador"),
            ("sd", "Ecuador"),
            ("si", "ඉක\u{dca}වදෝරය"),
            ("sk", "Ekvádor"),
            ("sl", "Ekvador"),
            ("so", "Ikwadoor"),
            ("sq", "Ekuador"),
            ("sr", "Еквадор"),
            ("sv", "Ecuador"),
            ("sw", "Ecuador"),
            ("ta", "ஈக\u{bcd}வட\u{bbe}ர\u{bcd}"),
            ("te", "ఈక\u{c4d}వ\u{c3f}డ\u{c3e}ర\u{c4d}"),
            ("tg", "Эквадор"),
            ("th", "เอกวาดอร\u{e4c}"),
            ("ti", "ኤኳዶር"),
            ("tk", "Ekwador"),
            ("tl", "Ekwador"),
            ("tr", "Ekvador"),
            ("tt", "Екуадор"),
            ("ug", "ئېكۋادور"),
            ("uk", "Еквадор"),
            ("ur", "ایکواڈور"),
            ("uz", "Ekvador"),
            ("ve", "Ecuador"),
            ("vi", "Ê-cu-a-đoa"),
            ("wa", "Ecwåteur"),
            ("wo", "Ekwadoor"),
            ("xh", "Ecuador"),
            ("yo", "Ẹ\u{300}kùàdọ\u{300}r"),
            ("zh_CN", "厄瓜多尔"),
            ("zh_HK", "厄瓜多爾"),
            ("zh_TW", "厄瓜多"),
            ("zu", "Ecuador"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}