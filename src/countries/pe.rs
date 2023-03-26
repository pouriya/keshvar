// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Perú

#[cfg(all(feature = "pe", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PE;
    pub const ALPHA3: Alpha3 = Alpha3::PER;
    pub const CONTINENT: Continent = Continent::SouthAmerica;
    pub const COUNTRY_CODE: usize = 51;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::PEN;
    pub const GEC: Option<GEC> = Some(GEC::PE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::PER);
    pub const ISO_SHORT_NAME: &str = "Peru";
    pub const ISO_LONG_NAME: &str = "The Republic of Perú";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Peruvian");
    pub const NUMBER: &str = "604";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(?:LIMA \\d{1,2}|CALLAO 0?\\d)|[0-2]\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthAmerica);
    pub const UN_LOCODE: &str = "PE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Peru", "Pérou", "Perú", "ペルー"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Peru"),
        ("af", "Peru"),
        ("ak", "Peru"),
        ("am", "ፔሩ"),
        ("an", "Peru"),
        ("ar", "البيرو"),
        ("as", "পেৰ\u{9c1}"),
        ("ay", "Peru"),
        ("az", "Peru"),
        ("ba", "Peru"),
        ("be", "Перу"),
        ("bg", "Перу"),
        ("bi", "Peru"),
        ("bn", "পের\u{9c1}"),
        ("bn_IN", "পের\u{9c1}"),
        ("br", "Perou"),
        ("bs", "Peru"),
        ("ca", "Perú"),
        ("ce", "Перу"),
        ("ch", "Perú"),
        ("cs", "Peru"),
        ("cv", "Перу"),
        ("cy", "Periw"),
        ("da", "Peru"),
        ("de", "Peru"),
        ("dv", "ޕ\u{7ac}ރ\u{7ab}"),
        ("dz", "པ\u{f7a}་ར\u{f74}།"),
        ("ee", "Peru"),
        ("el", "Περού"),
        ("en", "Peru"),
        ("eo", "Peruo"),
        ("es", "Perú"),
        ("et", "Peruu"),
        ("eu", "Peru"),
        ("fa", "پرو"),
        ("ff", "Peru"),
        ("fi", "Peru"),
        ("fo", "Perú"),
        ("fr", "Pérou"),
        ("fy", "Perû"),
        ("ga", "Peiriú"),
        ("gl", "Perú"),
        ("gn", "Peru"),
        ("gu", "પ\u{ac7}ર\u{ac1}"),
        ("gv", "Yn Pheroo"),
        ("ha", "Peru"),
        ("he", "פרו"),
        ("hi", "प\u{947}र\u{942}"),
        ("hr", "Peru"),
        ("ht", "Pewou"),
        ("hu", "Peru"),
        ("hy", "Պերու"),
        ("ia", "Peru"),
        ("id", "Peru"),
        ("io", "Peru"),
        ("is", "Perú"),
        ("it", "Perù"),
        ("iu", "ᐱᕉ"),
        ("ja", "ペルー"),
        ("ka", "პერუ"),
        ("ki", "Peru"),
        ("kk", "Перу"),
        ("kl", "Peru"),
        ("km", "ប\u{17c9}េរ\u{17c9}\u{17bc}"),
        ("kn", "ಪ\u{cc6}ರು"),
        ("ko", "페루"),
        ("ku", "Perû"),
        ("kv", "Перу"),
        ("kw", "Perou"),
        ("ky", "Перу"),
        ("lo", "ປະເທດເປຣ\u{eb9}"),
        ("lt", "Peru"),
        ("lv", "Peru"),
        ("mi", "Perū"),
        ("mk", "Перу"),
        ("ml", "പെറ\u{d41}"),
        ("mn", "Перу"),
        ("mr", "प\u{947}र\u{942}"),
        ("ms", "Peru"),
        ("mt", "Peru"),
        (
            "my",
            "ပ\u{102e}ရ\u{1030}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Peru"),
        ("nb", "Peru"),
        ("ne", "प\u{947}र\u{941}"),
        ("nl", "Peru"),
        ("nn", "Peru"),
        ("nv", "Dibénééz Bikéyah"),
        ("oc", "Peró"),
        ("or", "ପେର\u{b41}"),
        ("pa", "ਪ\u{a47}ਰ\u{a42}"),
        ("pi", "प\u{947}र\u{941}"),
        ("pl", "Peru"),
        ("ps", "پيرو"),
        ("pt", "Peru"),
        ("pt_BR", "Peru"),
        ("ro", "Peru"),
        ("ru", "Перу"),
        ("rw", "Peru"),
        ("sc", "Perù"),
        ("sd", "پيرو"),
        ("si", "පේර\u{dd4}"),
        ("sk", "Peru"),
        ("sl", "Peru"),
        ("so", "Peru"),
        ("sq", "Peru"),
        ("sr", "Перу"),
        ("sv", "Peru"),
        ("sw", "Peru"),
        ("ta", "பெரு"),
        ("te", "ప\u{c46}రూ"),
        ("tg", "Перу"),
        ("th", "เปร\u{e39}"),
        ("ti", "ፔሩ"),
        ("tk", "Peru"),
        ("tl", "Peru"),
        ("tr", "Peru"),
        ("tt", "Перу"),
        ("ug", "پېرۇ"),
        ("uk", "Перу"),
        ("ur", "پیرو"),
        ("uz", "Peru"),
        ("ve", "Peru"),
        ("vi", "Pê-ru"),
        ("wa", "Perou"),
        ("wo", "Peru"),
        ("xh", "Peru"),
        ("yo", "Perú"),
        ("zh_CN", "秘鲁"),
        ("zh_HK", "秘魯"),
        ("zh_TW", "祕魯"),
        ("zu", "I-Peru"),
    ];
    #[cfg(all(feature = "pe", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -9.189967;
        pub const LONGITUDE: f64 = -75.015152;
        pub const MAX_LATITUDE: f64 = -0.0387769;
        pub const MAX_LONGITUDE: f64 = -68.65232879999999;
        pub const MIN_LATITUDE: f64 = -18.4483;
        pub const MIN_LONGITUDE: f64 = -81.3867001;
        pub const NORTHEAST_LATITUDE: f64 = -0.0387769;
        pub const NORTHEAST_LONGITUDE: f64 = -68.65232879999999;
        pub const SOUTHWEST_LATITUDE: f64 = -18.4483;
        pub const SOUTHWEST_LONGITUDE: f64 = -81.3867001;
    }
}
#[cfg(all(feature = "pe", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -9.189967,
            longitude: -75.015152,
            max_latitude: -0.0387769,
            max_longitude: -68.65232879999999,
            min_latitude: -18.4483,
            min_longitude: -81.3867001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -0.0387769,
                    longitude: -68.65232879999999,
                },
                southwest: CountryGeoBound {
                    latitude: -18.4483,
                    longitude: -81.3867001,
                },
            },
        }
    }
}

#[cfg(all(feature = "pe", feature = "subdivisions"))]
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
                    "AMA",
                    Subdivision{
                        name: "AMA",
                        country_alpha2: Alpha2::PE,
                        code: "AMA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.115146), longitude: Some(-78.11082789999999), max_latitude: Some(-2.990077), min_latitude: Some(-6.976671), max_longitude: Some(-77.1344191), min_longitude: Some(-78.712204)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أمازوناس"), ("be", "Рэгіён Амасонас"), ("bg", "Амасонас"), ("bn", "আম\u{9be}জ\u{9c1}ন\u{9be}স অঞ\u{9cd}চল"), ("ca", "Regió de l’Amazones"), ("ccp", "𑄃𑄟𑄎\u{1112e}𑄚𑄌\u{11134}"), ("ceb", "Amazonas"), ("cs", "Amazonas"), ("da", "Amazonas"), ("de", "Amazonas"), ("el", "Αμαζόνες"), ("en", "Amazonas"), ("es", "Departamento de Amazonas"), ("et", "Amazonase piirkond"), ("fa", "ناحیه امازوناس"), ("fi", "Amazonasin alue"), ("fr", "Amazonas"), ("gl", "Amazonas"), ("gu", "એમ\u{ac7}ઝોનસ પ\u{acd}રદ\u{ac7}શ"), ("hi", "एम\u{947}ज\u{93c}ोनस क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Amazonas megye"), ("id", "Region Amazonas"), ("it", "regione di Amazonas"), ("ja", "アマソナス県"), ("ka", "ამასონასი"), ("kn", "ಅಮ\u{cc6}ಜೋನಾಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아마소나스 주"), ("lt", "Amazonės departamentas"), ("lv", "Amazonasas departaments"), ("mk", "Амазон"), ("mr", "अम\u{945}झॉनस प\u{94d}रद\u{947}श"), ("ms", "Wilayah Amazonas"), ("nb", "Amazonas"), ("nl", "Amazonas"), ("no", "Amazonas"), ("pl", "Region Amazonas"), ("pt", "Amazonas"), ("ro", "Amazonas"), ("ru", "Амасонас"), ("si", "ඇමසෝන\u{dcf}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Регион Амазонас"), ("sr_Latn", "Region Amazonas"), ("sv", "Amazonas"), ("ta", "அமேச\u{bbe}ன\u{bbe}ஸ\u{bcd} பகுதி"), ("te", "అమ\u{c46}జ\u{c4b}న\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอามาโซน\u{e31}ส"), ("tr", "Amazonas Bölgesi"), ("uk", "Амазонас"), ("ur", "امازوناس علاقہ"), ("vi", "Khu vực Amazonas"), ("zh", "亚马孙大区")]),
                        unofficial_name_list: ["Amazonas"].to_vec(),
                    }
                ),
                (
                    "ANC",
                    Subdivision{
                        name: "ANC",
                        country_alpha2: Alpha2::PE,
                        code: "ANC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.3250497), longitude: Some(-77.5619419), max_latitude: Some(-8.013484), min_latitude: Some(-10.7835641), max_longitude: Some(-76.7310711), min_longitude: Some(-78.6806381)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة أنكاش"), ("be", "Рэгіён Анкаш"), ("bg", "Анкаш"), ("bn", "আনক\u{9be}শ অঞ\u{9cd}চল"), ("bs", "Ancash"), ("ca", "Regió d’Ancash"), ("ccp", "𑄃𑄚\u{11134}𑄇𑄌\u{11134}"), ("ceb", "Ancash"), ("cs", "Ancash"), ("da", "Ancash"), ("de", "Ancash"), ("el", "Ανκάς"), ("en", "Ancash"), ("es", "Departamento de Áncash"), ("fa", "ناحیه انکاش"), ("fi", "Ancash"), ("fr", "Région d’Ancash"), ("gl", "Ancash"), ("gu", "એન\u{acd}ક\u{ac7}શ પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז אנקש"), ("hi", "एन\u{94d}क\u{948}श क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Ancash megye"), ("hy", "Անկաշ"), ("id", "Region Ancash"), ("it", "regione di Ancash"), ("ja", "アンカシュ県"), ("ka", "ანკაში"), ("kn", "ಅಂಕಾಶ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "앙카시 주"), ("lt", "Ankašo departamentas"), ("lv", "Ankašas departaments"), ("mk", "Анкаш"), ("mr", "अनक\u{945}श प\u{94d}रद\u{947}श"), ("ms", "Wilayah Ancash"), ("nb", "Ancash"), ("nl", "Ancash"), ("no", "Ancash"), ("pl", "Region Ancash"), ("pt", "Ancash"), ("ro", "Ancash"), ("ru", "Анкаш"), ("si", "ආන\u{dca}ක\u{dcf}ශ\u{dca} පළ\u{dcf}ත"), ("sv", "Ancash"), ("ta", "அஙகேஷ\u{bcd} பகுதி"), ("te", "అంక\u{c3e}శ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอ\u{e31}งก\u{e31}ช"), ("tr", "Ancash Bölgesi"), ("uk", "Анкаш"), ("ur", "انکاش علاقہ"), ("vi", "Vùng Ancash"), ("zh", "安卡什大区")]),
                        unofficial_name_list: ["Anqas"].to_vec(),
                    }
                ),
                (
                    "APU",
                    Subdivision{
                        name: "APU",
                        country_alpha2: Alpha2::PE,
                        code: "APU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.0504533), longitude: Some(-73.087749), max_latitude: Some(-13.203756), min_latitude: Some(-14.83203), max_longitude: Some(-72.05669379999999), min_longitude: Some(-73.849785)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أبوريماك"), ("be", "Рэгіён Апурымак"), ("bg", "Апуримак"), ("bn", "আপ\u{9c1}রিম\u{9be}ক অঞ\u{9cd}চল"), ("bs", "Apurímac"), ("ca", "Regió d’Apurímac"), ("ccp", "𑄃𑄛\u{1112a}𑄢\u{11128}𑄟𑄇\u{11134}"), ("ceb", "Apurimac"), ("cs", "Apurímac"), ("da", "Apurímac"), ("de", "Apurimac"), ("el", "Απουρίμακ"), ("en", "Apurímac"), ("es", "Departamento de Apurímac"), ("fa", "منطقه اپوریماک"), ("fi", "Apurímacin alue"), ("fr", "Région d’Apurímac"), ("ga", "Apurímac"), ("gl", "Departamento de Apurímac"), ("gu", "અપ\u{ac1}રિમક પ\u{acd}રા\u{a82}ત"), ("hi", "अप\u{941}रिमक क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Apurímac megye"), ("id", "Region Apurímac"), ("it", "regione di Apurímac"), ("ja", "アプリマク県"), ("ka", "აპურიმაკი"), ("kn", "ಅಪುರ\u{cbf}ಮಾಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "아푸리막 주"), ("lt", "Apurimako departamentas"), ("lv", "Apurimakas departaments"), ("mk", "Апуримак"), ("mr", "अप\u{941}रिमक प\u{94d}रद\u{947}श"), ("ms", "Wilayah Apurímac"), ("nb", "Apurímac"), ("nl", "Apurímac"), ("no", "Apurímac"), ("pl", "Region Apurímac"), ("pt", "Apurímac"), ("ro", "Apurímac"), ("ru", "Апуримак"), ("si", "අප\u{dd4}ර\u{dd2}මක\u{dca} කල\u{dcf}පය"), ("sv", "Apurímac"), ("ta", "அப\u{bcd}புரிமைக\u{bcd} பகுதி"), ("te", "అపూర\u{c3f}మ\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอาป\u{e39}ร\u{e35}ม\u{e31}ก"), ("tr", "Apurímac Bölgesi"), ("uk", "Апурімак"), ("ur", "اپوریماک علاقہ"), ("vi", "Apurímac"), ("zh", "阿普里马克大区")]),
                        unofficial_name_list: ["Apurímac"].to_vec(),
                    }
                ),
                (
                    "ARE",
                    Subdivision{
                        name: "ARE",
                        country_alpha2: Alpha2::PE,
                        code: "ARE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.4090474), longitude: Some(-71.53745099999999), max_latitude: Some(-16.3752714), min_latitude: Some(-16.4328333), max_longitude: Some(-71.51334229999999), min_longitude: Some(-71.5646806)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أريكيبا"), ("be", "Рэгіён Арэкіпа"), ("bg", "Арекипа"), ("bn", "আরেক\u{9c1}প\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Arequipa"), ("ccp", "𑄃𑄬𑄢𑄇\u{1112d}\u{1112a}𑄛"), ("ceb", "Arequipa"), ("cs", "Arequipa"), ("da", "Arequipa"), ("de", "Arequipa"), ("el", "Επαρχία Αρεκουίπα"), ("en", "Arequipa"), ("es", "Arequipa"), ("et", "Arequipa piirkond"), ("fa", "منطقه ارکیپا"), ("fi", "Arequipan alue"), ("fr", "Région d’Arequipa"), ("gl", "Departamento de Arequipa"), ("gu", "અર\u{ac7}ક\u{acd}વીપા પ\u{acd}રદ\u{ac7}શ"), ("hi", "एर\u{947}किपा प\u{94d}रद\u{947}श"), ("hu", "Arequipa megye"), ("id", "Region Arequipa"), ("it", "regione di Arequipa"), ("ja", "アレキパ県"), ("ka", "არეკიპა"), ("kn", "ಅರ\u{cc6}ಕ\u{ccd}ವ\u{cbf}ಪಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "아레키파 주"), ("lt", "Arekipos departamentas"), ("lv", "Arekipa reģions"), ("mk", "Арекипа"), ("ml", "അരെക\u{d4d}വിപ\u{d4d}പ"), ("mr", "अर\u{947}क\u{94d}विपा प\u{94d}रद\u{947}श"), ("ms", "Wilayah Arequipa"), ("nb", "Arequipa"), ("nl", "Arequipa"), ("no", "Arequipa"), ("pl", "Region Arequipa"), ("pt", "Arequipa"), ("ro", "Arequipa"), ("ru", "Арекипа"), ("si", "අරෙක\u{dd4}ය\u{dd2}ප\u{dcf} කල\u{dcf}පය"), ("sv", "Arequipa"), ("ta", "அரோகுய\u{bcd}ப\u{bcd}ப\u{bbe} பகுதி"), ("te", "అర\u{c46}క\u{c4d}వ\u{c3f}ప\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอาเรก\u{e35}ปา"), ("tr", "Arequipa bölgesi"), ("uk", "Арекіпа"), ("ur", "اریکیپا علاقہ"), ("vi", "Khu vực Arequipa"), ("zh", "阿雷基帕大区")]),
                        unofficial_name_list: ["Areqepa"].to_vec(),
                    }
                ),
                (
                    "AYA",
                    Subdivision{
                        name: "AYA",
                        country_alpha2: Alpha2::PE,
                        code: "AYA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.1638737), longitude: Some(-74.22356409999999), max_latitude: Some(-13.1192574), min_latitude: Some(-13.197473), max_longitude: Some(-74.18004379999999), min_longitude: Some(-74.2445644)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أياكوتشو"), ("be", "Рэгіён Аякуча"), ("bg", "Аякучо"), ("bn", "আয\u{9bc}\u{9be}ক\u{9c1}কো অঞ\u{9cd}চল"), ("ca", "Regió d’Ayacucho"), ("ccp", "𑄃\u{11128}𑄠𑄇\u{1112a}𑄌\u{1112e}"), ("ceb", "Ayacucho"), ("cs", "Ayacucho"), ("da", "Ayacucho"), ("de", "Ayacucho"), ("el", "Αγιακούτσο"), ("en", "Ayacucho"), ("es", "Departamento de Ayacucho"), ("eu", "Ayacucho"), ("fa", "منطقه آیاکوچو"), ("fi", "Ayacuchon alue"), ("fr", "Région d’Ayacucho"), ("gl", "Departamento de Ayacucho"), ("gu", "આયાક\u{ac1}ચો પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז אייקוצ׳ו"), ("hi", "अयाक\u{941}चो प\u{94d}रद\u{947}श"), ("hu", "Ayacucho megye"), ("id", "Region Ayacucho"), ("it", "regione di Ayacucho"), ("ja", "アヤクーチョ県"), ("ka", "აიაკუჩო"), ("kn", "ಅಯಕುಚೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "아야쿠초 주"), ("lt", "Ajakučo departamentas"), ("lv", "Ajakučo departaments"), ("mk", "Ајакучо"), ("mr", "आय\u{945}क\u{941}चो प\u{94d}रद\u{947}श"), ("ms", "Wilayah Ayacucho"), ("nb", "Ayacucho"), ("nl", "Ayacucho"), ("no", "Ayacucho"), ("pl", "Region Ayacucho"), ("pt", "Ayacucho"), ("ro", "Ayacucho"), ("ru", "Аякучо"), ("si", "අයක\u{dd4}චෝ කල\u{dcf}පය"), ("sv", "Ayacucho"), ("ta", "அயசுக\u{bcd}கோ பகுதி"), ("te", "అయ\u{c3e}కుచ\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอยาค\u{e31}ตโซ"), ("tr", "Ayacuchı Bölgesi"), ("uk", "Аякучо"), ("ur", "ایاکوچو علاقہ"), ("vi", "Vùng Ayacucho"), ("zh", "阿亚库乔大区")]),
                        unofficial_name_list: ["Ayakuchu"].to_vec(),
                    }
                ),
                (
                    "CAJ",
                    Subdivision{
                        name: "CAJ",
                        country_alpha2: Alpha2::PE,
                        code: "CAJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.4549673), longitude: Some(-78.8382644), max_latitude: Some(-4.6272731), min_latitude: Some(-7.757741900000001), max_longitude: Some(-77.7339101), min_longitude: Some(-79.4470061)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاخاماركا"), ("be", "Рэгіён Кахамарка"), ("bg", "Кахамарка"), ("bn", "ক\u{9be}জ\u{9be}ম\u{9be}র\u{9cd}ক\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Cajamarca"), ("ccp", "𑄇𑄎𑄟𑄢\u{11134}𑄇"), ("ceb", "Cajamarca"), ("cs", "Cajamarca"), ("da", "Cajamarca"), ("de", "Cajamarca"), ("el", "Κατζαμάρκα"), ("en", "Cajamarca"), ("es", "Departamento de Cajamarca"), ("fa", "منطقه کاخامارکا"), ("fi", "Cajamarcan alue"), ("fr", "Région de Cajamarca"), ("gu", "કાજમાર\u{acd}કા પ\u{acd}રદ\u{ac7}શ"), ("hi", "कजमार\u{94d}का क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Cajamarca megye"), ("id", "Region Cajamarca"), ("it", "regione di Cajamarca"), ("ja", "カハマルカ県"), ("ka", "კახამარკა"), ("kn", "ಕ\u{ccd}ಯಾಜಮಾರ\u{ccd}ಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "카하마르카 주"), ("lt", "Kachamarkos departamentas"), ("lv", "Kahamarkas departaments"), ("mk", "Кахамарка"), ("mr", "कजमार\u{94d}का प\u{94d}रद\u{947}श"), ("ms", "Wilayah Cajamarca"), ("nb", "Cajamarca"), ("nl", "Cajamarca"), ("no", "Cajamarca"), ("pl", "Region Cajamarca"), ("pt", "Cajamarca"), ("ro", "Cajamarca"), ("ru", "Кахамарка"), ("si", "ක\u{dcf}ජමර\u{dca}ක\u{dcf} කල\u{dcf}පය"), ("sv", "Cajamarca"), ("ta", "கஜம\u{bbe}ர\u{bcd}க\u{bcd}க பகுதி"), ("te", "క\u{c3e}జమ\u{c3e}ర\u{c4d}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาฮามาร\u{e4c}กา"), ("tr", "Cajamarca"), ("uk", "Кахамарка"), ("ur", "کاخامارکا علاقہ"), ("vi", "Cajamarca"), ("zh", "卡哈马卡大区")]),
                        unofficial_name_list: ["Cajamarca"].to_vec(),
                    }
                ),
                (
                    "CAL",
                    Subdivision{
                        name: "CAL",
                        country_alpha2: Alpha2::PE,
                        code: "CAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.0508491), longitude: Some(-77.1259843), max_latitude: Some(-11.9369042), min_latitude: Some(-12.0703989), max_longitude: Some(-77.0768166), min_longitude: Some(-77.158476)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالاو"), ("ca", "Regió de Callao"), ("ccp", "𑄃𑄬𑄣\u{11134} 𑄇𑄣𑄃\u{1112e}"), ("cs", "Callao"), ("da", "Callao"), ("en", "El Callao"), ("es", "Gobierno Regional del Callao"), ("fi", "Callaon alue"), ("fr", "Région de Callao"), ("hu", "Callao tartomány"), ("id", "Region Callao"), ("it", "regione di Callao"), ("ja", "カヤオ特別区"), ("ka", "კალიაო"), ("ms", "Wilayah Callao"), ("nb", "Callao"), ("nl", "Callao"), ("no", "Callao"), ("pt", "Callao"), ("ru", "Кальяо"), ("sv", "Callao"), ("uk", "Кальяо"), ("zh", "卡亚俄大区")]),
                        unofficial_name_list: ["El Callao"].to_vec(),
                    }
                ),
                (
                    "CUS",
                    Subdivision{
                        name: "CUS",
                        country_alpha2: Alpha2::PE,
                        code: "CUS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.53195), longitude: Some(-71.96746259999999), max_latitude: Some(-13.4973908), min_latitude: Some(-13.5626478), max_longitude: Some(-71.8533325), min_longitude: Some(-72.02516560000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كوسكو"), ("be", "Рэгіён Куска"), ("bg", "Куско"), ("bn", "ক\u{9c1}সক\u{9c1} অঞ\u{9cd}চল"), ("ca", "Departament de Cusco"), ("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Cusco"), ("cs", "Cuzco"), ("da", "Cusco"), ("de", "Cusco"), ("el", "Κούσκο"), ("en", "Cusco"), ("es", "Departamento de Cuzco"), ("eu", "Cuscoko eskualdea"), ("fa", "منطقه کوزکو"), ("fi", "Cuscon alue"), ("fr", "Région de Cuzco"), ("gu", "ક\u{ac1}સ\u{acd}કો પ\u{acd}રદ\u{ac7}શ"), ("hi", "क\u{941}स\u{94d}को प\u{94d}रद\u{947}श"), ("hu", "Cusco megye"), ("id", "Region Cusco"), ("it", "regione di Cusco"), ("ja", "クスコ県"), ("ka", "კუსკო"), ("kn", "ಕುಸ\u{ccd}ಕೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "쿠스코 주"), ("lt", "Kusko departamentas"), ("lv", "Kusko departaments"), ("mk", "Куско"), ("mr", "क\u{941}स\u{94d}को प\u{94d}रद\u{947}श"), ("ms", "Wilayah Cusco"), ("nb", "Cusco"), ("nl", "Cuzco"), ("no", "Cusco"), ("pl", "Region Cuzco"), ("pt", "Cusco"), ("ro", "Cusco"), ("ru", "Куско"), ("si", "ක\u{dd4}ස\u{dca}කෝ කල\u{dcf}පය"), ("sv", "Cusco"), ("ta", "கஸ\u{bcd}கோ பகுதி"), ("te", "కుస\u{c4d}క\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นก\u{e38}สโก"), ("tr", "Cusco Bölgesi"), ("uk", "Куско"), ("ur", "کوزکو علاقہ"), ("vi", "Khu vực Cusco"), ("zh", "库斯科大区")]),
                        unofficial_name_list: ["Cuzco", "Qosqo"].to_vec(),
                    }
                ),
                (
                    "HUC",
                    Subdivision{
                        name: "HUC",
                        country_alpha2: Alpha2::PE,
                        code: "HUC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.9298454), longitude: Some(-76.24326529999999), max_latitude: Some(-9.8383029), min_latitude: Some(-9.9442711), max_longitude: Some(-76.18090509999999), min_longitude: Some(-76.4056207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم هانوكو"), ("be", "Рэгіён Гуанука"), ("bg", "Хуануко"), ("bn", "হোয\u{9bc}\u{9be}ঙ\u{9cd}ক\u{9c1} অঞ\u{9cd}চল"), ("ca", "Departament de Huánuco"), ("ccp", "𑄦\u{1112a}𑄠𑄚\u{1112a}𑄇\u{1112e}"), ("ceb", "Región de Huánuco"), ("cs", "Huánuco"), ("da", "Huánuco"), ("de", "Huánuco"), ("el", "Χουανούκο"), ("en", "Huánuco"), ("es", "Departamento de Huánuco"), ("fa", "منطقه اوانوکو"), ("fi", "Huánucon alue"), ("fr", "Région de Huánuco"), ("gu", "હ\u{ac1}આન\u{ac1}કો પ\u{acd}રદ\u{ac7}શ"), ("hi", "ह\u{941}आन\u{941}को क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Huánuco megye"), ("id", "Region Huánuco"), ("it", "regione di Huánuco"), ("ja", "ワヌコ県"), ("ka", "უანუკო"), ("kn", "ಹುನಾನ\u{ccd}ಕೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "우아누코 주"), ("lt", "Huanuko departamentas"), ("lv", "Vanuko departaments"), ("mk", "Уануко"), ("mr", "ह\u{941}आन\u{941}को प\u{94d}रद\u{947}श"), ("ms", "Wilayah Huánuco"), ("nb", "Huánuco"), ("nl", "Huánuco"), ("no", "Huánuco"), ("pl", "Region Huánuco"), ("pt", "Huánuco"), ("ru", "Уануко"), ("si", "හ\u{dd4}වන\u{dd4}කෝ කල\u{dcf}පය"), ("sv", "Huánuco"), ("ta", "ஹஅணுக\u{bbe} பகுதி"), ("te", "హువ\u{c3e}నుక\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "อ\u{e31}วน\u{e39}โก"), ("tr", "Huanuco Region"), ("uk", "Уануко"), ("ur", "وانوکو علاقہ"), ("vi", "Khu vực Huánuco"), ("zh", "瓦努科大区")]),
                        unofficial_name_list: ["Huánuco"].to_vec(),
                    }
                ),
                (
                    "HUV",
                    Subdivision{
                        name: "HUV",
                        country_alpha2: Alpha2::PE,
                        code: "HUV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.786389), longitude: Some(-74.975556), max_latitude: Some(-12.7743664), min_latitude: Some(-12.7912327), max_longitude: Some(-74.9511767), min_longitude: Some(-74.9997568)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم هوانكافليكا"), ("be", "Рэгіён Гуанкавеліка"), ("bg", "Хуанкавелика (регион)"), ("bn", "হ\u{9c1}য\u{9bc}\u{9be}ঙ\u{9cd}ক\u{9be}ভেলিক\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Huancavelica"), ("ccp", "𑄦\u{1112a}𑄠𑄚\u{11134}𑄇𑄞𑄬𑄣\u{11128}𑄇"), ("ceb", "Huancavelica"), ("cs", "Huancavelica"), ("da", "Huancavelica"), ("de", "Huancavelica"), ("el", "Χουανκαβελίκα"), ("en", "Huancavelica"), ("es", "Departamento de Huancavelica"), ("fa", "منطقه اوانکاولیکا"), ("fi", "Huancavelican alue"), ("fr", "Région de Huancavelica"), ("gu", "હ\u{ac1}આન\u{acd}કાવ\u{ac7}લિકા પ\u{acd}રદ\u{ac7}શ"), ("hi", "ह\u{941}आ\u{902}काव\u{947}लिका प\u{94d}रद\u{947}श"), ("hu", "Huancavelica megye"), ("hy", "Ուանկավելիկա"), ("id", "Region Huancavelica"), ("it", "regione di Huancavelica"), ("ja", "ワンカベリカ県"), ("ka", "უანკაველიკა"), ("kn", "ಹುವಾನ\u{ccd}ವ\u{cc6}ವ\u{cc6}ಲ\u{cbf}ಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "우앙카벨리카 주"), ("lt", "Huankavelikos departamentas"), ("lv", "Vankavelikas departaments"), ("mk", "Уанкавелика"), ("mr", "ह\u{94d}वान\u{94d}सव\u{947}लिका प\u{94d}रद\u{947}श"), ("ms", "Wilayah Huancavelica"), ("nb", "Huancavelica"), ("nl", "Huancavelica"), ("no", "Huancavelica"), ("pl", "Region Huancavelica"), ("pt", "Huancavelica"), ("ru", "Уанкавелика"), ("si", "හ\u{dd4}අන\u{dca}ක\u{dcf}වෙල\u{dd2}ක\u{dcf} කල\u{dcf}පය"), ("sl", "Huancavelica"), ("sv", "Huancavelica"), ("ta", "ஹஅஞ\u{bcd}சவேலிக\u{bcd}க\u{bbe} பகுதி"), ("te", "హువ\u{c3e}న\u{c4d}క\u{c3e}వ\u{c46}ల\u{c3f}క\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นอวงกาเบล\u{e35}กา"), ("tr", "Huancavelica Bölgesi"), ("uk", "Уанкавеліка"), ("ur", "وانکابیلیکا علاقہ"), ("vi", "Khu vực Huancavelica"), ("zh", "萬卡韋利卡大區")]),
                        unofficial_name_list: ["Huancavelica"].to_vec(),
                    }
                ),
                (
                    "ICA",
                    Subdivision{
                        name: "ICA",
                        country_alpha2: Alpha2::PE,
                        code: "ICA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.9379378), longitude: Some(-75.8007093), max_latitude: Some(-12.9657768), min_latitude: Some(-15.4353454), max_longitude: Some(-74.65894279999999), min_longitude: Some(-76.4667187)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إيكا"), ("be", "Рэгіён Іка"), ("bg", "Ика"), ("bn", "ইক\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió d’Ica"), ("ccp", "𑄃\u{1112d}𑄇"), ("ceb", "Ica"), ("cs", "Ica"), ("da", "Ica"), ("de", "Ica"), ("el", "Ίκα"), ("en", "Ica"), ("es", "Departamento de Ica"), ("fa", "منطقه ایکا"), ("fi", "Ican alue"), ("fr", "Région d’Ica"), ("gu", "આઈકા પ\u{acd}રદ\u{ac7}શ"), ("hi", "इका प\u{94d}रद\u{947}श"), ("hu", "Ica megye"), ("id", "Region Ica"), ("it", "regione di Ica"), ("ja", "イカ県"), ("ka", "იკა"), ("kn", "ಇಕಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "이카 주"), ("lt", "Ikos departamentas"), ("lv", "Ikas departaments"), ("mk", "Ика"), ("mr", "आयसीए प\u{94d}रद\u{947}श"), ("ms", "Wilayah Ica"), ("nb", "Ica"), ("nl", "Ica"), ("no", "Ica"), ("pl", "Region Ica"), ("pt", "Ica"), ("ru", "Ика"), ("si", "ඉක\u{dcf} කල\u{dcf}පය"), ("sv", "Ica"), ("ta", "ஈஸ\u{bbe} பகுதி"), ("te", "ఇక\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตอ\u{e34}กา"), ("tr", "Ica Bölgesi"), ("uk", "Іка"), ("ur", "اکا علاقہ"), ("vi", "Khu vực Ica"), ("zh", "伊卡大区")]),
                        unofficial_name_list: ["Ica"].to_vec(),
                    }
                ),
                (
                    "JUN",
                    Subdivision{
                        name: "JUN",
                        country_alpha2: Alpha2::PE,
                        code: "JUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.335798), longitude: Some(-75.34121789999999), max_latitude: Some(-10.6655961), min_latitude: Some(-12.6817741), max_longitude: Some(-73.3522989), min_longitude: Some(-76.513082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم جونين"), ("be", "Рэгіён Хунін"), ("bg", "Хунин"), ("bn", "জ\u{9c1}নিন অঞ\u{9cd}চল"), ("ca", "Regió de Junín"), ("ccp", "𑄎\u{1112a}𑄚\u{11128}𑄚\u{11134}"), ("ceb", "Junín"), ("cs", "Junín"), ("da", "Junín"), ("de", "Junín"), ("el", "Τζουνίν"), ("en", "Junín"), ("es", "Departamento de Junín"), ("fa", "منطقه خونین"), ("fi", "Junínin alue"), ("fr", "Région de Junín"), ("gu", "જ\u{ac1}નિન પ\u{acd}રદ\u{ac7}શ"), ("hi", "ज\u{942}निन क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Junín megye"), ("id", "Region Junín"), ("it", "regione di Junín"), ("ja", "フニン県"), ("ka", "ხუნინი"), ("kn", "ಜುನ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "후닌 주"), ("lt", "Chunino departamentas"), ("lv", "Huninas departaments"), ("mk", "Хунин"), ("mr", "ज\u{941}निन प\u{94d}रद\u{947}श"), ("ms", "Wilayah Junín"), ("nb", "Junín"), ("nl", "Junín"), ("no", "Junín"), ("pl", "Region Junín"), ("pt", "Junín"), ("ro", "Junín"), ("ru", "Хунин"), ("si", "ජ\u{dd4}න\u{dd2}න\u{dca} කල\u{dcf}පය"), ("sv", "Junín"), ("ta", "ஜூனின\u{bcd} பகுதி"), ("te", "జూన\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e39}น\u{e34}น"), ("tr", "Junin Region"), ("uk", "Хунін"), ("ur", "خونین علاقہ"), ("vi", "Khu vực Junín"), ("zh", "胡宁大区")]),
                        unofficial_name_list: ["Junín"].to_vec(),
                    }
                ),
                (
                    "LAL",
                    Subdivision{
                        name: "LAL",
                        country_alpha2: Alpha2::PE,
                        code: "LAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.143593300000001), longitude: Some(-78.4751945), max_latitude: Some(-6.95362), min_latitude: Some(-8.9774429), max_longitude: Some(-76.89797279999999), min_longitude: Some(-79.68024)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم لا ليبرتاد"), ("be", "Рэгіён Ла-Лібертад"), ("bg", "Ла Либертад"), ("bn", "ল\u{9be} লিব\u{9be}র\u{9cd}টেড অঞ\u{9cd}চল"), ("ca", "Regió de La Libertad"), ("ccp", "𑄣 𑄣\u{11128}𑄝𑄢\u{11134}𑄑𑄖\u{11134}"), ("ceb", "La Libertad"), ("cs", "La Libertad"), ("da", "La Libertad"), ("de", "La Libertad"), ("el", "Λα Λιμπερτάντ"), ("en", "La Libertad"), ("es", "Departamento de La Libertad"), ("fa", "منطقه لا لیبرتاد"), ("fi", "La Libertad"), ("fr", "Région de La Libertad"), ("gu", "લા લિબર\u{acd}ટાડ પ\u{acd}રદ\u{ac7}શ"), ("hi", "ला लिबर\u{94d}टाड प\u{94d}रद\u{947}श"), ("hu", "La Libertad megye"), ("id", "Region La Libertad"), ("it", "regione di La Libertad"), ("ja", "ラ・リベルタ県"), ("ka", "ლა-ლიბერტადი"), ("kn", "ಲಾ ಲ\u{cbf}ಬರ\u{ccd}ಟಾಡ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "라리베르타드 주"), ("lt", "La Libertado departamentas"), ("lv", "Lalibertadas departaments"), ("mk", "Ла Либертад"), ("mr", "ला लिबर\u{94d}टद प\u{94d}रद\u{947}श"), ("ms", "Wilayah La Libertad"), ("nb", "La Libertad"), ("nl", "La Libertad"), ("no", "La Libertad"), ("pl", "Region La Libertad"), ("pt", "Liberdade"), ("ru", "Ла-Либертад"), ("si", "ල\u{dcf} ල\u{dd2}බරෙටෙඩ\u{dca} කල\u{dcf}පය"), ("sv", "La Libertad"), ("ta", "ல\u{bbe} லிபேர\u{bcd}ட\u{bcd}டட\u{bcd} பகுதி"), ("te", "ల\u{c3e} ల\u{c3f}బర\u{c4d}ట\u{c3e}డ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตลา ล\u{e34}แบร\u{e4c}ต\u{e31}ด"), ("tr", "La Libertad Bölgesi"), ("uk", "Ла-Лібертад"), ("ur", "لا لیورتاد علاقہ"), ("vi", "Khu vực La Libertad"), ("zh", "拉利伯塔德大区")]),
                        unofficial_name_list: ["La Libertad"].to_vec(),
                    }
                ),
                (
                    "LAM",
                    Subdivision{
                        name: "LAM",
                        country_alpha2: Alpha2::PE,
                        code: "LAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.4776528), longitude: Some(-79.9192702), max_latitude: Some(-5.5400261), min_latitude: Some(-7.178573999999999), max_longitude: Some(-79.131943), min_longitude: Some(-80.6023481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم لمباييكه"), ("be", "Рэгіён Ламбаеке"), ("bg", "Ламбайеке"), ("bn", "ল\u{9be}ম\u{9cd}ব\u{9be}য\u{9bc}েক অঞ\u{9cd}চল"), ("ca", "Regió de Lambayeque"), ("ccp", "𑄣𑄟\u{11134}𑄝𑄠𑄬𑄇\u{11134}"), ("ceb", "Lambayeque"), ("cs", "Lambayeque"), ("da", "Lambayeque"), ("de", "Lambayeque"), ("el", "Λαμπαγιέκ"), ("en", "Lambayeque"), ("es", "Departamento de Lambayeque"), ("fa", "منطقه لامبایکه"), ("fi", "Lambayequen alue"), ("fr", "Région de Lambayeque"), ("gu", "લા\u{a82}બાય\u{ac7}ક પ\u{acd}રદ\u{ac7}શ"), ("hi", "ल\u{948}म\u{94d}ब\u{947}क प\u{94d}रद\u{947}श"), ("hu", "Lambayeque megye"), ("id", "Region Lambayeque"), ("it", "regione di Lambayeque"), ("ja", "ランバイエケ県"), ("ka", "ლამბაიეკე"), ("kn", "ಲ\u{ccd}ಯಾಂಬಯ\u{cc6}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "람바예케 주"), ("lt", "Lambajekės departamentas"), ("lv", "Lambajekes departaments"), ("mk", "Ламбајеке"), ("mr", "ला\u{902}बीय\u{941}क प\u{94d}रद\u{947}श"), ("ms", "Wilayah Lambayeque"), ("nb", "Lambayeque"), ("nl", "Lambayeque"), ("no", "Lambayeque"), ("pl", "Region Lambayeque"), ("pt", "Lambayeque"), ("ru", "Ламбаеке"), ("si", "ලැම\u{dca}බේ එක\u{dca}වේ කල\u{dcf}පය"), ("sv", "Lambayeque"), ("ta", "லம\u{bcd}ப\u{bbe}ஏகியு ர\u{bc0}ஜியன\u{bcd}"), ("te", "ల\u{c3e}ంబ\u{c47}ఖ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นล\u{e31}มบาเยเก"), ("tr", "Lambayaque Bölgesi"), ("uk", "Ламбаєке"), ("ur", "لامبایےکے علاقہ"), ("vi", "Khu vực Lambayeque"), ("zh", "兰巴耶克大区")]),
                        unofficial_name_list: ["Lambayeque"].to_vec(),
                    }
                ),
                (
                    "LIM",
                    Subdivision{
                        name: "LIM",
                        country_alpha2: Alpha2::PE,
                        code: "LIM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.046374), longitude: Some(-77.0427934), max_latitude: Some(-12.0308632), min_latitude: Some(-12.0798252), max_longitude: Some(-77.0020311), min_longitude: Some(-77.0883395)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ليما"), ("be", "Рэгіён Ліма"), ("bg", "Лима"), ("bn", "লিম\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Lima"), ("ccp", "𑄣\u{11128}𑄟 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("cs", "Lima"), ("da", "Lima"), ("de", "Lima"), ("el", "Λίμα"), ("en", "Lima Region"), ("es", "Lima"), ("fa", "منطقه لیما"), ("fi", "Liman alue"), ("fr", "Lima"), ("gu", "લિમા પ\u{acd}રદ\u{ac7}શ"), ("hi", "लीमा क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Lima megye"), ("id", "Region Lima"), ("it", "Lima"), ("ja", "リマ県"), ("ka", "ლიმა"), ("kn", "ಲ\u{cbf}ಮಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "리마 주"), ("lt", "Limos departamentas"), ("lv", "Limas departaments"), ("mk", "Лима"), ("mr", "लिमा प\u{94d}रद\u{947}श"), ("ms", "Wilayah Lima"), ("nb", "Lima"), ("nl", "Lima"), ("no", "Lima"), ("pl", "Region Lima"), ("pt", "Lima"), ("ru", "Лима"), ("si", "ල\u{dd2}ම\u{dcf} පළ\u{dcf}ත"), ("sv", "Lima"), ("ta", "ல\u{bc0}ம\u{bbe} பகுதி"), ("te", "ల\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ล\u{e34}มา"), ("tr", "Lima Bölgesi"), ("uk", "Ліма"), ("ur", "لیما علاقہ"), ("vi", "Khu vực Lima"), ("zh", "利馬大區")]),
                        unofficial_name_list: ["Lima"].to_vec(),
                    }
                ),
                (
                    "LMA",
                    Subdivision{
                        name: "LMA",
                        country_alpha2: Alpha2::PE,
                        code: "LMA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.046374), longitude: Some(-77.0427934), max_latitude: Some(-12.0308632), min_latitude: Some(-12.0798252), max_longitude: Some(-77.0020311), min_longitude: Some(-77.0883395)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ليما"), ("be", "правінцыя Ліма"), ("bn", "লিম\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Lima"), ("ccp", "𑄣\u{11128}𑄟"), ("ceb", "Provincia de Lima"), ("da", "Lima²"), ("de", "Provinz Lima"), ("el", "Επαρχία Λίμας"), ("en", "Lima"), ("es", "Provincia de Lima"), ("fi", "Liman metropolialue"), ("fr", "province de Lima"), ("gl", "Provincia de Lima"), ("gu", "લિમા પ\u{acd}રા\u{a82}ત"), ("hi", "लीमा प\u{94d}रा\u{902}त"), ("id", "Provinsi Lima"), ("it", "provincia di Lima"), ("ja", "リマ郡"), ("ka", "ლიმა²"), ("kn", "ಲ\u{cbf}ಮಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리마 군"), ("lt", "Limos provincija"), ("lv", "Limas province"), ("mr", "लिमा प\u{94d}रा\u{902}त"), ("ms", "Lima Province"), ("nb", "Lima provins"), ("nl", "Lima²"), ("no", "Lima provins"), ("pl", "Prowincja Lima"), ("pt", "Lima²"), ("ro", "Provincia Lima"), ("ru", "Лима²"), ("si", "ල\u{dd2}ම\u{dcf} පළ\u{dcf}ත²"), ("sv", "Lima²"), ("ta", "லிம\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3f}మ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e34}มา"), ("tr", "Lima ili"), ("uk", "Ліма²"), ("ur", "لیما صوبہ"), ("vi", "Tỉnh Lima"), ("zh", "利馬省")]),
                        unofficial_name_list: ["Lima Metropolitana"].to_vec(),
                    }
                ),
                (
                    "LOR",
                    Subdivision{
                        name: "LOR",
                        country_alpha2: Alpha2::PE,
                        code: "LOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.232472899999999), longitude: Some(-74.21793260000001), max_latitude: Some(-0.012977), min_latitude: Some(-8.697728999999999), max_longitude: Some(-69.94945500000001), min_longitude: Some(-77.80600299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم لوريتو"), ("be", "Рэгіён Ларэта"), ("bg", "Лорето"), ("bn", "লরেতো অঞ\u{9cd}চল"), ("ca", "Departament de Loreto"), ("ccp", "𑄣\u{1112e}𑄢𑄬𑄑\u{1112e}"), ("ceb", "Loreto"), ("cs", "Loreto"), ("da", "Loreto"), ("de", "Loreto"), ("el", "Λορέτο"), ("en", "Loreto"), ("es", "Departamento de Loreto"), ("et", "Loreto piirkond"), ("eu", "Loreto departamendua"), ("fa", "منطقه لورتو"), ("fi", "Loreton alue"), ("fr", "région de Loreto"), ("gu", "લોર\u{ac7}ટો પ\u{acd}રદ\u{ac7}શ"), ("hi", "लोर\u{947}टो क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Loreto megye"), ("id", "Region Loreto"), ("it", "regione di Loreto"), ("ja", "ロレート県"), ("ka", "ლორეტო"), ("kn", "ಲೊರ\u{cc6}ಟೋ ಪ\u{ccd}ರದೇಶ"), ("ko", "로레토 주"), ("lt", "Loreto departamentas"), ("lv", "Loreto departaments"), ("mk", "Лорето"), ("mr", "लोर\u{947}र\u{947} प\u{94d}रद\u{947}श"), ("ms", "Wilayah Loreto"), ("nb", "Loreto"), ("nl", "Loreto"), ("no", "Loreto"), ("pl", "Region Loreto"), ("pt", "Loreto"), ("ro", "Loreto"), ("ru", "Лорето"), ("si", "ලොරෙටෝ කල\u{dcf}පය"), ("sv", "Loreto"), ("ta", "லொரேட\u{bcd}டோ பகுதி"), ("te", "ల\u{c4b}ర\u{c46}ట\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นโลเรโต"), ("tr", "Loreto Bölgesi"), ("uk", "Лорето"), ("ur", "لوریتو علاقہ"), ("vi", "Khu vực Loreto"), ("zh", "洛雷托大区")]),
                        unofficial_name_list: ["Loreto"].to_vec(),
                    }
                ),
                (
                    "MDD",
                    Subdivision{
                        name: "MDD",
                        country_alpha2: Alpha2::PE,
                        code: "MDD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.5986), longitude: Some(-70.09058399999999), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم مادر دي ديوس"), ("bg", "Мадре де Диос"), ("bn", "ম\u{9be}দ\u{9cd}রি দি ড\u{9be}ইওস অঞ\u{9cd}চল"), ("ca", "Departament de Madre de Dios"), ("ccp", "𑄟𑄓\u{11133}𑄢𑄬 𑄓𑄬 𑄓\u{11128}𑄠\u{1112e}𑄌\u{11134}"), ("ceb", "Madre de Dios"), ("cs", "Madre de Dios"), ("da", "Madre de Dios"), ("de", "Madre de Dios"), ("el", "Μάντρε ντε Ρίος"), ("en", "Madre de Dios"), ("es", "Departamento de Madre de Dios"), ("fa", "منطقه مادره ده دیوس"), ("fi", "Madre de Diosin alue"), ("fr", "Région de Madre de Dios"), ("gu", "માદ\u{acd}ર\u{ac7} દ\u{ac7} ડીઓસ પ\u{acd}રદ\u{ac7}શ"), ("he", "מדרה דה דיוס"), ("hi", "माद\u{94d}र\u{947} डी डियोस क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Madre de Dios megye"), ("id", "Region Madre de Dios"), ("it", "regione di Madre de Dios"), ("ja", "マードレ・デ・ディオス県"), ("ka", "მადრე-დე-დიოსი"), ("kn", "ಮ\u{ccd}ಯಾಡ\u{ccd}ರ\u{cc6} ಡ\u{cbf} ಡ\u{cbf}ಯೊಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "마드레데디오스 주"), ("lt", "Madre de Dioso departamentas"), ("lv", "Madre De Dios reģions"), ("mk", "Мадре де Диос"), ("mr", "माद\u{947} ड\u{947} डीस प\u{94d}रद\u{947}श"), ("ms", "Wilayah Madre de Dios"), ("nb", "Madre de Dios"), ("nl", "Madre de Dios"), ("no", "Madre de Dios"), ("pl", "Region Madre de Dios"), ("pt", "Madre de Deus (região)"), ("ru", "Мадре-де-Дьос"), ("si", "මද\u{dca}රේ ඩ\u{dd2} ඩ\u{dd2}යෝස\u{dca} කල\u{dcf}පය"), ("sv", "Madre de Dios"), ("ta", "மன\u{bcd}றே டே டியோஸ\u{bcd} பகுதி"), ("te", "మ\u{c3e}డ\u{c4d}ర\u{c47} ద డ\u{c3f}య\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตมาเดร เด ด\u{e34}โอส"), ("tr", "Madre de Dior Bölgesi"), ("uk", "Мадре-де-Дьйос"), ("ur", "مادرے دے دیوس علاقہ"), ("vi", "Khu vực Madre de Dios"), ("zh", "马德雷德迪奥斯大区")]),
                        unofficial_name_list: ["Madre de Dios"].to_vec(),
                    }
                ),
                (
                    "MOQ",
                    Subdivision{
                        name: "MOQ",
                        country_alpha2: Alpha2::PE,
                        code: "MOQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-17.1927361), longitude: Some(-70.93281379999999), max_latitude: Some(-17.1641233), min_latitude: Some(-17.2175438), max_longitude: Some(-70.8882093), min_longitude: Some(-70.9717655)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم موكيغوا"), ("be", "Рэгіён Макегуа"), ("bg", "Мокегуа"), ("bn", "ম\u{9c1}কেগোয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Moquegua"), ("ccp", "𑄟\u{11127}𑄇\u{1112a}𑄠𑄬𑄉\u{1112a}𑄠"), ("ceb", "Departamento de Moquegua"), ("cs", "Moquegua"), ("da", "Moquegua"), ("de", "Moquegua"), ("el", "Μοκέγκουα"), ("en", "Moquegua"), ("es", "Departamento de Moquegua"), ("fa", "منطقه موکگوا"), ("fi", "Moqueguan alue"), ("fr", "Région de Moquegua"), ("gu", "મોક\u{ac7}ગ\u{ac1}આ પ\u{acd}રદ\u{ac7}શ"), ("hi", "मोक\u{947}ग\u{941}आ क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Moquegua megye"), ("id", "Region Moquegua"), ("it", "regione di Moquegua"), ("ja", "モケグア県"), ("ka", "მოკეგუა"), ("kn", "ಮೊಕುಗುವಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "모케과 주"), ("lt", "Mokegvos departamentas"), ("lv", "Mokegvas departaments"), ("mk", "Мокегва"), ("mr", "मोक\u{947}ग\u{941}आ प\u{94d}रद\u{947}श"), ("ms", "Wilayah Moquegua"), ("nb", "Moquegua"), ("nl", "Moquegua"), ("no", "Moquegua"), ("pl", "Region Moquegua"), ("pt", "Moquegua"), ("ro", "Moquegua"), ("ru", "Мокегуа"), ("si", "මොක\u{dca}වෙග\u{dd4}ව\u{dcf} කල\u{dcf}පය"), ("sv", "Moquegua"), ("ta", "மொயூகுஞ\u{bcd} பகுதி"), ("te", "మ\u{c3e}క\u{c4d}వ\u{c46}గ\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นโมเกกวา"), ("tr", "Moquegua Bölgesi"), ("uk", "Мокеґуа"), ("ur", "موکیگوا علاقہ"), ("vi", "Khu vực Moquegua"), ("zh", "莫克瓜大区")]),
                        unofficial_name_list: ["Moquegua"].to_vec(),
                    }
                ),
                (
                    "PAS",
                    Subdivision{
                        name: "PAS",
                        country_alpha2: Alpha2::PE,
                        code: "PAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.4475753), longitude: Some(-75.1545381), max_latitude: Some(-9.431508), min_latitude: Some(-11.134321), max_longitude: Some(-74.1266931), min_longitude: Some(-76.704674)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم باسكو"), ("be", "Рэгіён Паска"), ("bg", "Паско"), ("bn", "প\u{9be}স\u{9cd}ক\u{9c1} অঞ\u{9cd}চল"), ("ca", "Regió de Pasco"), ("ccp", "𑄛𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Pasco"), ("cs", "Pasco"), ("da", "Pasco"), ("de", "Pasco"), ("el", "Πάσκο"), ("en", "Pasco"), ("es", "Departamento de Pasco"), ("fa", "منطقه پاسکو"), ("fi", "Pascon alue"), ("fr", "Région de Pasco"), ("gu", "પાસ\u{acd}કો પ\u{acd}રદ\u{ac7}શ"), ("hi", "पास\u{94d}को प\u{94d}रद\u{947}श"), ("hu", "Pasco megye"), ("id", "Region Pasco"), ("it", "regione di Pasco"), ("ja", "パスコ県"), ("ka", "პასკო"), ("kn", "ಪಾಸ\u{ccd}ಕೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "파스코 주"), ("lt", "Pasko departamentas"), ("lv", "Pasko departaments"), ("mk", "Паско"), ("mr", "पास\u{94d}को प\u{94d}रद\u{947}श"), ("ms", "Negeri Pasco"), ("nb", "Pasco"), ("nl", "Pasco"), ("no", "Pasco"), ("pl", "Region Pasco"), ("pt", "Pasco"), ("ro", "Pasco"), ("ru", "Паско"), ("si", "පැස\u{dca}කෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Pasco"), ("ta", "ப\u{bbe}ஸ\u{bcd}கோ பகுதி"), ("te", "ప\u{c3e}స\u{c4d}క\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดปาสโก"), ("tr", "Pasco Bölgesi"), ("uk", "Паско"), ("ur", "پاسکو علاقہ"), ("vi", "Khu vực Pasco"), ("zh", "帕斯科大区")]),
                        unofficial_name_list: ["Pasco"].to_vec(),
                    }
                ),
                (
                    "PIU",
                    Subdivision{
                        name: "PIU",
                        country_alpha2: Alpha2::PE,
                        code: "PIU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.2), longitude: Some(-80.633333), max_latitude: Some(-5.1395019), min_latitude: Some(-5.2466696), max_longitude: Some(-80.6152725), min_longitude: Some(-80.71834009999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيورا"), ("bg", "Пиура"), ("bn", "পিউর\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Piura"), ("ccp", "𑄛\u{1112d}𑄅\u{1112a}𑄢"), ("ceb", "Piura"), ("cs", "Piura"), ("da", "Piura"), ("de", "Piura"), ("el", "Πιούρα"), ("en", "Piura"), ("es", "Departamento de Piura"), ("fa", "پیورا"), ("fi", "Piuran alue"), ("fr", "Région de Piura"), ("gu", "પિય\u{ac1}રા પ\u{acd}રદ\u{ac7}શ"), ("hi", "पिउरा प\u{94d}रद\u{947}श"), ("hu", "Piura megye"), ("id", "Region Piura"), ("it", "regione di Piura"), ("ja", "ピウラ県"), ("ka", "პიურა"), ("kn", "ಪ\u{cbf}ಯುರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "피우라 주"), ("lt", "Piuros departamentas"), ("lv", "Pjuras departaments"), ("mk", "Пиура"), ("mr", "पिउरा प\u{94d}रद\u{947}श"), ("ms", "Wilayah Piura"), ("nb", "Piura"), ("nl", "Piura"), ("no", "Piura"), ("pl", "Region Piura"), ("pt", "Piura"), ("ro", "Piura"), ("ru", "Пьюра"), ("si", "ප\u{dd2}ය\u{dd4}ර\u{dcf} කල\u{dcf}පය"), ("sv", "Piura"), ("ta", "பியூர\u{bbe} பகுதி"), ("te", "ప\u{c3f}యూర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นป\u{e34}วรา"), ("tr", "Piura Bölgesi"), ("uk", "Піура"), ("ur", "پیورا علاقہ"), ("vi", "Khu vực Piura"), ("zh", "皮乌拉地区")]),
                        unofficial_name_list: ["Piura"].to_vec(),
                    }
                ),
                (
                    "PUN",
                    Subdivision{
                        name: "PUN",
                        country_alpha2: Alpha2::PE,
                        code: "PUN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.234875), longitude: Some(-70.050314), max_latitude: Some(-13.0444181), min_latitude: Some(-17.294243), max_longitude: Some(-68.82681529999999), min_longitude: Some(-71.1451721)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بونو"), ("be", "Рэгіён Пуна"), ("bg", "Пуно"), ("ca", "Regió de Puno"), ("ccp", "𑄛\u{1112a}𑄚\u{1112e}"), ("ceb", "Puno"), ("cs", "Puno"), ("da", "Puno"), ("de", "Puno"), ("en", "Puno"), ("es", "Departamento de Puno"), ("fa", "منطقه پونو"), ("fi", "Punon alue"), ("fr", "Région de Puno"), ("he", "פונו"), ("hu", "Puno megye"), ("id", "Region Puno"), ("it", "regione di Puno"), ("ja", "プーノ県"), ("ka", "პუნო"), ("ko", "푸노 주"), ("lt", "Puno departamentas"), ("mk", "Пуно"), ("ms", "Wilayah Puno"), ("nb", "Puno"), ("nl", "Puno"), ("no", "Puno"), ("pl", "Region Puno"), ("pt", "Puno"), ("ro", "Puno"), ("ru", "Пуно"), ("sv", "Puno"), ("th", "แคว\u{e49}นป\u{e39}โน"), ("tr", "Puno Bölgesi"), ("uk", "Пуно"), ("ur", "پونو علاقہ"), ("vi", "Puno"), ("zh", "普诺大区")]),
                        unofficial_name_list: ["Puno"].to_vec(),
                    }
                ),
                (
                    "SAM",
                    Subdivision{
                        name: "SAM",
                        country_alpha2: Alpha2::PE,
                        code: "SAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.244488100000001), longitude: Some(-76.8259652), max_latitude: Some(-5.394636999999999), min_latitude: Some(-8.796522), max_longitude: Some(-75.502786), min_longitude: Some(-77.780281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سان مارتين"), ("be", "Рэгіён Сан-Марцін"), ("bg", "Сан Мартин"), ("bn", "স\u{9be}ন ম\u{9be}র\u{9cd}টিন অঞ\u{9cd}চল"), ("ca", "Departament de San Martín"), ("ccp", "𑄥𑄚\u{11134} 𑄟𑄢\u{11134}𑄑\u{11128}𑄚\u{11134}"), ("ceb", "Región de San Martín"), ("cs", "San Martín"), ("da", "San Martín"), ("de", "Region San Martín"), ("el", "Σαν Μάρτιν"), ("en", "San Martín"), ("es", "Departamento de San Martín"), ("fa", "منطقه سن مارتین"), ("fi", "San Martínin alue"), ("fr", "Région de San Martín"), ("gu", "સ\u{ac7}ન માર\u{acd}ટિન પ\u{acd}રદ\u{ac7}શ"), ("he", "סן מרטין"), ("hi", "स\u{948}न मार\u{94d}टिन क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "San Martín megye"), ("hy", "Սան Մարտին տարածաշրջան"), ("id", "Region San Martín"), ("it", "regione di San Martín"), ("ja", "サン・マルティン県"), ("ka", "სან-მარტინი"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಮಾರ\u{ccd}ಟ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "산마르틴 주"), ("lt", "San Martino departamentas"), ("lv", "Sanmartinas departaments"), ("mk", "Сан Мартин"), ("mr", "स\u{945}न मार\u{94d}टिन प\u{94d}रद\u{947}श"), ("ms", "Wilayah San Martín"), ("nb", "San Martín"), ("nl", "San Martín"), ("no", "San Martín"), ("pl", "Region San Martín"), ("pt", "San Martín"), ("ru", "Сан-Мартин"), ("si", "සැන\u{dca} ම\u{dcf}ර\u{dca}ට\u{dd2}න\u{dca} කල\u{dcf}පය"), ("sv", "San Martín"), ("ta", "ச\u{bbe}ன\u{bcd} ம\u{bbe}ர\u{bcd}ட\u{bcd}டின\u{bcd} பகுதி"), ("te", "స\u{c3e}న\u{c4d} మ\u{c3e}ర\u{c4d}ట\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นซานมาร\u{e4c}ต\u{e34}น"), ("tr", "San Martín Bölgesi"), ("uk", "Сан-Мартін"), ("ur", "سان مارتین علاقہ"), ("vi", "San Martín (tỉnh)"), ("zh", "圣马丁大区")]),
                        unofficial_name_list: ["San Martín"].to_vec(),
                    }
                ),
                (
                    "TAC",
                    Subdivision{
                        name: "TAC",
                        country_alpha2: Alpha2::PE,
                        code: "TAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-18.0065679), longitude: Some(-70.2462741), max_latitude: Some(-17.9556994), min_latitude: Some(-18.0900544), max_longitude: Some(-70.1808773), min_longitude: Some(-70.3142166)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اقليم تاكنا"), ("be", "Рэгіён Такна"), ("bg", "Такна"), ("bn", "ত\u{9be}কন\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Tacna"), ("ccp", "𑄑𑄬𑄇\u{11134}𑄚"), ("ceb", "Tacna"), ("cs", "Tacna"), ("da", "Tacna"), ("de", "Tacna"), ("el", "Τάκνα"), ("en", "Tacna"), ("es", "Departamento de Tacna"), ("et", "Tacna piirkond"), ("fa", "منطقه تاکنا"), ("fi", "Tacnan alue"), ("fr", "Région de Tacna"), ("gu", "ટાક\u{acd}ના પ\u{acd}રા\u{a82}ત"), ("hi", "ट\u{948}क\u{94d}ना क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Tacna megye"), ("id", "Region Tacna"), ("it", "regione di Tacna"), ("ja", "タクナ県"), ("ka", "ტაკნა"), ("kn", "ಟಕ\u{ccd}ನಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "타크나 주"), ("lt", "Taknos departamentas"), ("lv", "Taknas departaments"), ("mk", "Такна"), ("mr", "टाक\u{94d}ना प\u{94d}रद\u{947}श"), ("ms", "Wilayah Tacna"), ("nb", "Tacna"), ("nl", "Tacna"), ("no", "Tacna"), ("pl", "Region Tacna"), ("pt", "Tacna"), ("ro", "Tacna"), ("ru", "Такна"), ("si", "ටක\u{dca}න\u{dcf} කල\u{dcf}පය"), ("sv", "Tacna"), ("ta", "ட\u{bbe}கின பகுதி"), ("te", "ట\u{c3e}క\u{c4d}న\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เทคนา"), ("tr", "Tacna Bölge"), ("uk", "Такна"), ("ur", "تاکنا علاقہ"), ("vi", "Khu vựcTacna"), ("zh", "塔克纳大区")]),
                        unofficial_name_list: ["Tacna"].to_vec(),
                    }
                ),
                (
                    "TUM",
                    Subdivision{
                        name: "TUM",
                        country_alpha2: Alpha2::PE,
                        code: "TUM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.9338256), longitude: Some(-80.54384499999999), max_latitude: Some(-3.404815), min_latitude: Some(-4.2320951), max_longitude: Some(-80.15360989999999), min_longitude: Some(-81.0339359)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة تومبيس"), ("be", "Рэгіён Тумбес"), ("bg", "Тумбес"), ("bn", "ত\u{9c1}ম\u{9cd}বেস বিভ\u{9be}গ"), ("ca", "Regió de Tumbes"), ("ccp", "𑄑\u{1112a}𑄟\u{11134}𑄝𑄬𑄌\u{11134}"), ("ceb", "Tumbes"), ("cs", "Tumbes"), ("da", "Tumbes"), ("de", "Tumbes"), ("el", "Τάμπες"), ("en", "Tumbes"), ("es", "Departamento de Tumbes"), ("fa", "منطقه تومبس"), ("fi", "Tumbesin alue"), ("fr", "Région de Tumbes"), ("gu", "ટમ\u{acd}બ\u{acd}સ વિભાગ"), ("hi", "त\u{94d}य\u{942}म\u{94d}ब\u{947}स विभाग"), ("hu", "Tumbes megye"), ("id", "Region Tumbes"), ("it", "regione di Tumbes"), ("ja", "トゥンベス県"), ("ka", "ტუმბესი"), ("kn", "ತುಂಬೇಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "툼베스 주"), ("lt", "Tumbeso departamentas"), ("lv", "Tumbesas departaments"), ("mk", "Тумбес"), ("mr", "ट\u{941}ब\u{947}स विभाग"), ("ms", "Wilayah Tumbes"), ("nb", "Tumbes"), ("nl", "Tumbes"), ("no", "Tumbes"), ("pl", "Region Tumbes"), ("pt", "Tumbes"), ("ru", "Тумбес"), ("si", "ටම\u{dca}බස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Tumbes"), ("ta", "டும\u{bcd}பஸ\u{bcd} துறை"), ("te", "టంబ\u{c46}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "แคว\u{e49}นต\u{e38}มเบส"), ("tr", "Tumbe Departmanı"), ("uk", "Тумбес"), ("ur", "تومبیس علاقہ"), ("vi", "Khu vực hành chính Tumbes"), ("zh", "通贝斯大区")]),
                        unofficial_name_list: ["Tumbes"].to_vec(),
                    }
                ),
                (
                    "UCA",
                    Subdivision{
                        name: "UCA",
                        country_alpha2: Alpha2::PE,
                        code: "UCA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.8251183), longitude: Some(-73.087749), max_latitude: Some(-7.292605099999999), min_latitude: Some(-11.4428331), max_longitude: Some(-70.49636090000001), min_longitude: Some(-75.96401209999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم أوكايالي"), ("be", "Рэгіён Укаялі"), ("bg", "Укаяли"), ("bn", "ওক\u{9be}য\u{9bc}\u{9be}লি অঞ\u{9cd}চল"), ("bs", "Ucayali"), ("ca", "Departament d’Ucayali"), ("ccp", "𑄅\u{1112a}𑄇𑄬𑄠𑄣\u{11128}"), ("ceb", "Ucayali"), ("cs", "Ucayali"), ("da", "Ucayali"), ("de", "Ucayali"), ("el", "Ουκαγιάλι"), ("en", "Ucayali"), ("es", "Departamento de Ucayali"), ("et", "Ucayali piirkond"), ("eu", "Ucayali"), ("fa", "منطقه\u{654} اوکایالی"), ("fi", "Ucayalin alue"), ("fr", "Région d’Ucayali"), ("gu", "ઉકાયલી પ\u{acd}રદ\u{ac7}શ"), ("hi", "उकायली क\u{94d}ष\u{947}त\u{94d}र"), ("hu", "Ucayali megye"), ("id", "Region Ucayali"), ("it", "regione di Ucayali"), ("ja", "ウカヤリ県"), ("ka", "უკაიალი"), ("kn", "ಉಕಯಾಲ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "우카얄리 주"), ("lt", "Ukajalio departamentas"), ("lv", "Ukajali departaments"), ("mk", "Укајали"), ("mr", "उकायली प\u{94d}रद\u{947}श"), ("ms", "Ucayali Region"), ("nb", "Ucayali"), ("nl", "Ucayali"), ("no", "Ucayali"), ("pl", "Region Ukajali"), ("pt", "Ucayali"), ("ru", "Укаяли"), ("si", "උක\u{dcf}යල\u{dd2} කල\u{dcf}පය"), ("sv", "Ucayali"), ("ta", "உக\u{bbe}யலி பகுதி"), ("te", "ఉకయ\u{c3e}ల\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดดเจลฟา"), ("tr", "Ucayali Bölgesi"), ("uk", "Укаялі"), ("ur", "اکیالی علاقہ"), ("vi", "Ucayali"), ("zh", "乌卡亚利大区")]),
                        unofficial_name_list: ["Ucayali"].to_vec(),
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
#[cfg(feature = "pe")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PE,
        alpha3: Alpha3::PER,
        address_format: None,
        continent: Continent::SouthAmerica,
        country_code: 51,
        currency_code: CurrencyCode::PEN,
        gec: Some(GEC::PE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::PER),
        iso_long_name: "The Republic of Perú",
        iso_short_name: "Peru",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Peruvian"),
        number: "604",
        postal_code: true,
        postal_code_format: Some("(?:LIMA \\d{1,2}|CALLAO 0?\\d)|[0-2]\\d{4}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthAmerica),
        un_locode: "PE",
        unofficial_name_list: ["Peru", "Pérou", "Perú", "ペルー"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Peru"),
            ("af", "Peru"),
            ("ak", "Peru"),
            ("am", "ፔሩ"),
            ("an", "Peru"),
            ("ar", "البيرو"),
            ("as", "পেৰ\u{9c1}"),
            ("ay", "Peru"),
            ("az", "Peru"),
            ("ba", "Peru"),
            ("be", "Перу"),
            ("bg", "Перу"),
            ("bi", "Peru"),
            ("bn", "পের\u{9c1}"),
            ("bn_IN", "পের\u{9c1}"),
            ("br", "Perou"),
            ("bs", "Peru"),
            ("ca", "Perú"),
            ("ce", "Перу"),
            ("ch", "Perú"),
            ("cs", "Peru"),
            ("cv", "Перу"),
            ("cy", "Periw"),
            ("da", "Peru"),
            ("de", "Peru"),
            ("dv", "ޕ\u{7ac}ރ\u{7ab}"),
            ("dz", "པ\u{f7a}་ར\u{f74}།"),
            ("ee", "Peru"),
            ("el", "Περού"),
            ("en", "Peru"),
            ("eo", "Peruo"),
            ("es", "Perú"),
            ("et", "Peruu"),
            ("eu", "Peru"),
            ("fa", "پرو"),
            ("ff", "Peru"),
            ("fi", "Peru"),
            ("fo", "Perú"),
            ("fr", "Pérou"),
            ("fy", "Perû"),
            ("ga", "Peiriú"),
            ("gl", "Perú"),
            ("gn", "Peru"),
            ("gu", "પ\u{ac7}ર\u{ac1}"),
            ("gv", "Yn Pheroo"),
            ("ha", "Peru"),
            ("he", "פרו"),
            ("hi", "प\u{947}र\u{942}"),
            ("hr", "Peru"),
            ("ht", "Pewou"),
            ("hu", "Peru"),
            ("hy", "Պերու"),
            ("ia", "Peru"),
            ("id", "Peru"),
            ("io", "Peru"),
            ("is", "Perú"),
            ("it", "Perù"),
            ("iu", "ᐱᕉ"),
            ("ja", "ペルー"),
            ("ka", "პერუ"),
            ("ki", "Peru"),
            ("kk", "Перу"),
            ("kl", "Peru"),
            ("km", "ប\u{17c9}េរ\u{17c9}\u{17bc}"),
            ("kn", "ಪ\u{cc6}ರು"),
            ("ko", "페루"),
            ("ku", "Perû"),
            ("kv", "Перу"),
            ("kw", "Perou"),
            ("ky", "Перу"),
            ("lo", "ປະເທດເປຣ\u{eb9}"),
            ("lt", "Peru"),
            ("lv", "Peru"),
            ("mi", "Perū"),
            ("mk", "Перу"),
            ("ml", "പെറ\u{d41}"),
            ("mn", "Перу"),
            ("mr", "प\u{947}र\u{942}"),
            ("ms", "Peru"),
            ("mt", "Peru"),
            (
                "my",
                "ပ\u{102e}ရ\u{1030}းန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Peru"),
            ("nb", "Peru"),
            ("ne", "प\u{947}र\u{941}"),
            ("nl", "Peru"),
            ("nn", "Peru"),
            ("nv", "Dibénééz Bikéyah"),
            ("oc", "Peró"),
            ("or", "ପେର\u{b41}"),
            ("pa", "ਪ\u{a47}ਰ\u{a42}"),
            ("pi", "प\u{947}र\u{941}"),
            ("pl", "Peru"),
            ("ps", "پيرو"),
            ("pt", "Peru"),
            ("pt_BR", "Peru"),
            ("ro", "Peru"),
            ("ru", "Перу"),
            ("rw", "Peru"),
            ("sc", "Perù"),
            ("sd", "پيرو"),
            ("si", "පේර\u{dd4}"),
            ("sk", "Peru"),
            ("sl", "Peru"),
            ("so", "Peru"),
            ("sq", "Peru"),
            ("sr", "Перу"),
            ("sv", "Peru"),
            ("sw", "Peru"),
            ("ta", "பெரு"),
            ("te", "ప\u{c46}రూ"),
            ("tg", "Перу"),
            ("th", "เปร\u{e39}"),
            ("ti", "ፔሩ"),
            ("tk", "Peru"),
            ("tl", "Peru"),
            ("tr", "Peru"),
            ("tt", "Перу"),
            ("ug", "پېرۇ"),
            ("uk", "Перу"),
            ("ur", "پیرو"),
            ("uz", "Peru"),
            ("ve", "Peru"),
            ("vi", "Pê-ru"),
            ("wa", "Perou"),
            ("wo", "Peru"),
            ("xh", "Peru"),
            ("yo", "Perú"),
            ("zh_CN", "秘鲁"),
            ("zh_HK", "秘魯"),
            ("zh_TW", "祕魯"),
            ("zu", "I-Peru"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
