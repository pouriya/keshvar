// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Federal Democratic Republic of Nepal

#[cfg(all(feature = "np", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::NP;
    pub const ALPHA3: Alpha3 = Alpha3::NPL;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 977;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::NPR;
    pub const GEC: Option<GEC> = Some(GEC::NP);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::NEP);
    pub const ISO_SHORT_NAME: &str = "Nepal";
    pub const ISO_LONG_NAME: &str = "The Federal Democratic Republic of Nepal";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ne"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["bho", "mai", "ne", "new", "urd"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Nepalese");
    pub const NUMBER: &str = "524";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "NP";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Nepal",
        "Népal",
        "the Federal Democratic Republic of Nepal",
        "ネパール",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Nepal"),
        ("af", "Nepal"),
        ("ak", "Nepal"),
        ("am", "ኔፓሔ"),
        ("an", "Nepal"),
        ("ar", "نيبال"),
        ("as", "নেপ\u{9be}ল"),
        ("ay", "Nepal"),
        ("az", "Nepal"),
        ("ba", "Nepal"),
        ("be", "Непал"),
        ("bg", "Непал"),
        ("bi", "Nepal"),
        ("bn", "নেপ\u{9be}ল"),
        ("bn_IN", "নেপ\u{9be}ল"),
        ("br", "Nepal"),
        ("bs", "Nepal"),
        ("ca", "Nepal"),
        ("ce", "Непал"),
        ("ch", "Nepal"),
        ("cs", "Nepál"),
        ("cv", "Непал"),
        ("cy", "Nepal"),
        ("da", "Nepal"),
        ("de", "Nepal"),
        ("dv", "ނ\u{7ad}ޕ\u{7a7}ލ\u{7b0}"),
        ("dz", "ན\u{f7a}་པ\u{f71}ལ།"),
        ("ee", "Nepal"),
        ("el", "Νεπάλ"),
        ("en", "Nepal"),
        ("eo", "Nepalo"),
        ("es", "Nepal"),
        ("et", "Nepal"),
        ("eu", "Nepal"),
        ("fa", "نپال"),
        ("ff", "Nepal"),
        ("fi", "Nepal"),
        ("fo", "Nepal"),
        ("fr", "Népal"),
        ("fy", "Nepal"),
        ("ga", "Neipeal"),
        ("gl", "Nepal"),
        ("gn", "Nepal"),
        ("gu", "ન\u{ac7}પાળ"),
        ("gv", "Nepaal"),
        ("ha", "Nepal"),
        ("he", "נפאל"),
        ("hi", "न\u{947}पाल"),
        ("hr", "Nepal"),
        ("ht", "Nepal"),
        ("hu", "Nepál"),
        ("hy", "Նեպալ"),
        ("ia", "Nepal"),
        ("id", "Nepal"),
        ("io", "Nepal"),
        ("is", "Nepal"),
        ("it", "Nepal"),
        ("iu", "Nepal"),
        ("ja", "ネパール"),
        ("ka", "ნეპალი"),
        ("ki", "Nepal"),
        ("kk", "Непал"),
        ("kl", "Nepal"),
        ("km", "នេប\u{17c9}ាល\u{17cb}"),
        ("kn", "ನೇಪಾಳ"),
        ("ko", "네팔"),
        ("ku", "Nepal"),
        ("kv", "Непал"),
        ("kw", "Nepal"),
        ("ky", "Непал"),
        ("lo", "Nepal"),
        ("lt", "Nepalas"),
        ("lv", "Nepāla"),
        ("mi", "Nepōra"),
        ("mk", "Непал"),
        ("ml", "നേപ\u{d3e}ള\u{d4d}\u{200d}"),
        ("mn", "Нипал"),
        ("mr", "न\u{947}पाळ"),
        ("ms", "Nepal"),
        ("mt", "Nepal"),
        ("my", "န\u{102e}ပေါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Nepar"),
        ("nb", "Nepal"),
        ("ne", "न\u{947}पाल"),
        ("nl", "Nepal"),
        ("nn", "Nepal"),
        ("nv", "Dziłghą\u{301}ąʼdi Naakaii Dootłʼizhí Bikéyah"),
        ("oc", "Nepal"),
        ("or", "ନେପ\u{b3e}ଳ"),
        ("pa", "ਨ\u{a47}ਪਾਲ"),
        ("pi", "न\u{947}पाल"),
        ("pl", "Nepal"),
        ("ps", "نیپال"),
        ("pt", "Nepal"),
        ("pt_BR", "Nepal"),
        ("ro", "Nepal"),
        ("ru", "Непал"),
        ("rw", "Nepali"),
        ("sc", "Nepal"),
        ("sd", "نيپال"),
        ("si", "නේප\u{dcf}ලය"),
        ("sk", "Nepál"),
        ("sl", "Nepal"),
        ("so", "Nepal"),
        ("sq", "Nepal"),
        ("sr", "Непал"),
        ("sv", "Nepal"),
        ("sw", "Nepal"),
        ("ta", "நேப\u{bbe}ளம\u{bcd}"),
        ("te", "న\u{c47}ప\u{c3e}ల\u{c4d}"),
        ("tg", "Непал"),
        ("th", "เนปาล"),
        ("ti", "ኔፓል"),
        ("tk", "Nepal"),
        ("tl", "Nepal"),
        ("tr", "Nepal"),
        ("tt", "Непал"),
        ("ug", "نېپال"),
        ("uk", "Непал"),
        ("ur", "نیپال"),
        ("uz", "Nepal"),
        ("ve", "Nepal"),
        ("vi", "Nê-pan"),
        ("wa", "Nepal"),
        ("wo", "Nepaal"),
        ("xh", "Nepal"),
        ("yo", "Nepal"),
        ("zh_CN", "尼泊尔"),
        ("zh_HK", "尼泊爾"),
        ("zh_TW", "尼泊爾"),
        ("zu", "Nepal"),
    ];
    #[cfg(all(feature = "np", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 28.394857;
        pub const LONGITUDE: f64 = 84.12400799999999;
        pub const MAX_LATITUDE: f64 = 30.4473898;
        pub const MAX_LONGITUDE: f64 = 88.20182969999999;
        pub const MIN_LATITUDE: f64 = 26.3473741;
        pub const MIN_LONGITUDE: f64 = 80.05846980000001;
        pub const NORTHEAST_LATITUDE: f64 = 30.4473898;
        pub const NORTHEAST_LONGITUDE: f64 = 88.20182969999999;
        pub const SOUTHWEST_LATITUDE: f64 = 26.3473741;
        pub const SOUTHWEST_LONGITUDE: f64 = 80.05846980000001;
    }
}
#[cfg(all(feature = "np", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 28.394857,
            longitude: 84.12400799999999,
            max_latitude: 30.4473898,
            max_longitude: 88.20182969999999,
            min_latitude: 26.3473741,
            min_longitude: 80.05846980000001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 30.4473898,
                    longitude: 88.20182969999999,
                },
                southwest: CountryGeoBound {
                    latitude: 26.3473741,
                    longitude: 80.05846980000001,
                },
            },
        }
    }
}

#[cfg(all(feature = "np", feature = "subdivisions"))]
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
                    "1",
                    Subdivision{
                        name: "मध\u{94d}यमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र",
                        country_alpha2: Alpha2::NP,
                        code: "1",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DevelopmentRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة التنموية الوسطى"), ("bg", "Мадхяманчал"), ("bn", "মধ\u{9cd}যম\u{9be}ঞ\u{9cd}চল বিক\u{9be}স ক\u{9cd}ষেত\u{9cd}র, নেপ\u{9be}ল"), ("ca", "Regió Central"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134}"), ("ceb", "Madhyamanchal"), ("de", "Entwicklungsregion Mitte"), ("el", "Κεντρική αναπτυξιακή περιοχή"), ("en", "Central"), ("es", "Región Central"), ("eu", "Erdialdeko eskualdea"), ("fi", "Keskinen kehitysalue"), ("fr", "Région de développement Centre"), ("hi", "मध\u{94d}यमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("hy", "Կենտրոնական շրջան"), ("it", "regione di Sviluppo Centrale"), ("ja", "中部開発区域"), ("ko", "중부 개발 지구"), ("ne", "मध\u{94d}यमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Central Region"), ("pl", "Central Development Region"), ("pt", "Centro"), ("ru", "Центральный регион"), ("ta", "மத\u{bcd}திய வளர\u{bcd}ச\u{bcd}சி பிர\u{bbe}ந\u{bcd}தியம\u{bcd}, நேப\u{bbe}ளம\u{bcd}"), ("uk", "Центральний регіон"), ("ur", "مرکزی ترقیاتی علاقہ، نیپال"), ("vi", "Trung Nepal"), ("zh", "中部發展區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "2",
                    Subdivision{
                        name: "मध\u{94d}य-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र",
                        country_alpha2: Alpha2::NP,
                        code: "2",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DevelopmentRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة التنموية الغربية الوسطى"), ("bn", "মধ\u{9cd}য-পশ\u{9cd}চিম\u{9be}ঞ\u{9cd}চল বিক\u{9be}স ক\u{9cd}ষেত\u{9cd}র, নেপ\u{9be}ল"), ("ccp", "𑄟\u{11127}𑄖\u{11134}𑄙\u{11133}𑄠\u{11127} 𑄛\u{11127}𑄌\u{11134}𑄥\u{11128}𑄟𑄚\u{11134}𑄌\u{11127}𑄣\u{11134}"), ("de", "Entwicklungsregion Mittelwest"), ("el", "Μεσοδυτική αναπτυξιακή περιοχή"), ("en", "Madhya Pashchimanchal"), ("eu", "Erdi-mendebaldeko eskualdea"), ("fi", "Keskilännen kehitysalue"), ("fr", "Région de développement Moyen-Ouest"), ("hi", "मध\u{94d}य-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("it", "regione di Sviluppo del Medio Occidente"), ("ja", "中西部開発区域"), ("ko", "중서부 개발 지구"), ("ne", "मध\u{94d}य-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Mid-Western Region"), ("pl", "Mid-Western Development Region"), ("pt", "Centro-Oeste"), ("ru", "Среднезападный регион"), ("ta", "மத\u{bcd}தியமேற\u{bcd}கு வளர\u{bcd}ச\u{bcd}சி பிர\u{bbe}ந\u{bcd}தியம\u{bcd}, நேப\u{bbe}ளம\u{bcd}"), ("uk", "Середньозахідний регіон"), ("ur", "وسط-مغربی ترقیاتی علاقہ، نیپال"), ("vi", "Trung Tây Nepal"), ("zh", "中西部發展區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "3",
                    Subdivision{
                        name: "पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र",
                        country_alpha2: Alpha2::NP,
                        code: "3",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DevelopmentRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة التنموية الغربية"), ("be", "Заходні рэгіён"), ("bg", "Пашчиманчал"), ("bn", "পশ\u{9cd}চিম\u{9be}ঞ\u{9cd}চল বিক\u{9be}স ক\u{9cd}ষেত\u{9cd}র, নেপ\u{9be}ল"), ("ca", "Regió Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Pashchimanchal"), ("de", "Entwicklungsregion West"), ("el", "Δυτική αναπτυξιακή περιοχή"), ("en", "Western"), ("eu", "Mendebaldeko eskualdea"), ("fi", "Läntinen kehitysalue"), ("fr", "Région de développement Ouest"), ("hi", "पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("hy", "Արևմտյան շրջան"), ("it", "regione di Sviluppo Occidentale"), ("ja", "西部開発区域"), ("ko", "서부 개발 지구"), ("ne", "पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Western Region"), ("pl", "Western Development Region"), ("pt", "Oeste"), ("ru", "Западный регион"), ("ta", "மேற\u{bcd}கு வளர\u{bcd}ச\u{bcd}சி பிர\u{bbe}ந\u{bcd}தியம\u{bcd}, நேப\u{bbe}ளம\u{bcd}"), ("uk", "Західний регіон"), ("ur", "مغربی ترقیاتی علاقہ، نیپال"), ("vi", "Tây Nepal"), ("zh", "西部发展区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "4",
                    Subdivision{
                        name: "प\u{942}र\u{94d}वाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र",
                        country_alpha2: Alpha2::NP,
                        code: "4",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DevelopmentRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة التنموية الشرقية"), ("bn", "প\u{9c2}র\u{9cd}ব\u{9be}ঞ\u{9cd}চল বিক\u{9be}স ক\u{9cd}ষেত\u{9cd}র"), ("ca", "Regió de Desenvolupament Est"), ("ccp", "𑄛\u{1112a}𑄢\u{11134}𑄤𑄚\u{11134}𑄌\u{11127}𑄣\u{11134}"), ("ceb", "Purwanchal"), ("de", "Entwicklungsregion Ost"), ("el", "Ανατολική αναπτυξιακή περιοχή"), ("en", "Purwanchal"), ("es", "región de desarrollo Este"), ("eu", "Ekialdeko eskualdea"), ("fi", "Itäinen kehitysalue"), ("fr", "Région de développement Est"), ("hi", "प\u{942}र\u{94d}वाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("it", "regione di Sviluppo Orientale"), ("ja", "東部開発区域"), ("ko", "동부 개발 지구"), ("ne", "प\u{942}र\u{94d}वाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Eastern Development Region"), ("pl", "Eastern Development Region"), ("pt", "Região Leste"), ("ru", "Восточный регион"), ("ta", "கிழக\u{bcd}கு வளர\u{bcd}ச\u{bcd}சி பிர\u{bbe}ந\u{bcd}தியம\u{bcd}, நேப\u{bbe}ளம\u{bcd}"), ("uk", "Східний регіон"), ("ur", "مشرقی ترقیاتی علاقہ، نیپال"), ("vi", "Đông Nepal"), ("zh", "東部發展區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "5",
                    Subdivision{
                        name: "स\u{941}द\u{942}र-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र",
                        country_alpha2: Alpha2::NP,
                        code: "5",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DevelopmentRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المنطقة التنموية الغربية البعيدة"), ("bn", "স\u{9c1}দ\u{9c2}র পশ\u{9cd}চিম\u{9be}ঞ\u{9cd}চল বিক\u{9be}স ক\u{9cd}ষেত\u{9cd}র, নেপ\u{9be}ল"), ("ccp", "𑄥\u{1112a}𑄘\u{1112a}𑄢\u{11134} 𑄛\u{11127}𑄌\u{11134}𑄥\u{11128}𑄟𑄚\u{11134}𑄌\u{11127}𑄣\u{11134}"), ("de", "Entwicklungsregion Fernwest"), ("el", "Άπω δυτική αναπτυξιακή περιοχή"), ("en", "Sudur Pashchimanchal"), ("eu", "Mendebaldeko Urruneko eskualdea"), ("fi", "Kaukolännen kehitysalue"), ("fr", "Région de développement Extrême-Ouest"), ("hi", "स\u{941}द\u{942}र-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("it", "regione di Sviluppo dell’Estremo Occidente"), ("ja", "極西部開発区域"), ("ko", "극서부 개발 지구"), ("ne", "स\u{941}द\u{942}र-पश\u{94d}चिमाञ\u{94d}चल विकास क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Far-Western Development Region, Nepal"), ("pl", "Far-Western Development Region"), ("pt", "Extremo-Oeste"), ("ru", "Дальнезападный регион"), ("ta", "தூரமேற\u{bcd}கு வளர\u{bcd}ச\u{bcd}சி பிர\u{bbe}ந\u{bcd}தியம\u{bcd}, நேப\u{bbe}ளம\u{bcd}"), ("uk", "Далекозахідний регіон"), ("ur", "بعید-مغربی ترقیاتی علاقہ، نیپال"), ("vi", "Viễn Tây Nepal"), ("zh", "遠西部發展區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "Bagmati",
                        country_alpha2: Alpha2::NP,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.0367577), longitude: Some(85.4375574), max_latitude: Some(28.3420049), min_latitude: Some(27.3182049), max_longitude: Some(86.06501), min_longitude: Some(84.62609499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية باجماتي"), ("bg", "Багмати"), ("bn", "ব\u{9be}গমতী অঞ\u{9cd}চল"), ("ca", "Zona de Bagmati"), ("ccp", "𑄝𑄇\u{11134}𑄟𑄑\u{11128}"), ("ceb", "Bāgmatī Zone"), ("da", "Bagmati Zone"), ("de", "Bagmati"), ("el", "Μπαγκμάτι"), ("en", "Bagmati"), ("es", "Zona de Bagmati"), ("eu", "Bagmati Gunea"), ("fi", "Bagmati"), ("fr", "Bagmati"), ("gu", "બાગમતી પ\u{acd}રા\u{a82}ત"), ("hi", "बागमती प\u{94d}रान\u{94d}त"), ("id", "Zona Bagmati"), ("it", "Bagmati"), ("ja", "バグマティ県"), ("kn", "ಬಾಗ\u{ccd}ಮತ\u{cbf} ವಲಯ"), ("ko", "바그마티 구"), ("lt", "Bagmačio zona"), ("lv", "Bāgmati zona"), ("mr", "बागमती झोन"), ("ms", "Bagmati Zone"), ("nb", "Bagmati Sone"), ("ne", "बागमती अञ\u{94d}चल"), ("nl", "Bagmati"), ("no", "Bagmati Sone"), ("pl", "Bagmati"), ("pt", "Bagmati"), ("ru", "Багмати"), ("si", "බග\u{dca}මට\u{dd2} කල\u{dcf}පය"), ("sk", "Bágmatí"), ("sv", "Bagmati Zone"), ("ta", "ப\u{bbe}க\u{bcd}ம\u{bbe}டி சோன\u{bcd}"), ("te", "బ\u{c3e}గ\u{c4d}మత\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "เขตบากมาต\u{e34}"), ("tr", "Bagmati Zon"), ("uk", "Багматі"), ("ur", "باگمتی زون"), ("vi", "Vùng Bagmati"), ("zh", "巴格馬蒂專區")]),
                        unofficial_name_list: ["Bagmati"].to_vec(),
                    }
                ),
                (
                    "BH",
                    Subdivision{
                        name: "Bheri",
                        country_alpha2: Alpha2::NP,
                        code: "BH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.517456), longitude: Some(81.77870209999999), max_latitude: Some(29.135872), min_latitude: Some(27.85854), max_longitude: Some(82.45864910000002), min_longitude: Some(80.98443600000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية بهري"), ("bg", "Бхери"), ("bn", "ভেরী অঞ\u{9cd}চল"), ("ccp", "𑄞𑄬𑄢\u{11128}"), ("ceb", "Bherī Zone"), ("da", "Bheri Zone"), ("de", "Bheri"), ("el", "Μπέρι"), ("en", "Bheri"), ("es", "Zona de Bheri"), ("eu", "Bheri Gunea"), ("fi", "Bheri"), ("fr", "Bheri"), ("gu", "ભ\u{ac7}રી પ\u{acd}રા\u{a82}ત"), ("hi", "भ\u{947}री अ\u{902}चल"), ("id", "Bheri Zone"), ("it", "Bheri"), ("ja", "ベリ県"), ("kn", "ಭೇರ\u{cbf} ವಲಯ"), ("ko", "베리 구"), ("lt", "Bherio zona"), ("lv", "Berio zona"), ("mr", "ब\u{947}री झोन"), ("ms", "Bheri Zone"), ("nb", "Bheri sone"), ("ne", "भ\u{947}री अञ\u{94d}चल"), ("nl", "Bheri"), ("no", "Bheri sone"), ("pl", "Bheri"), ("pt", "Bheri"), ("ru", "Бхери"), ("si", "භේර\u{dd2} කල\u{dcf}පය"), ("sk", "Bherí"), ("sv", "Bheri"), ("ta", "பஹிரி ஸ\u{bcd}யோனே"), ("te", "భ\u{c47}ర\u{c40} జ\u{c4b}న\u{c4d}"), ("th", "เขตเบร\u{e35}"), ("tr", "Bheri Zone"), ("uk", "Бхері"), ("ur", "بھیری زون"), ("vi", "Vùng Bheri"), ("zh", "佩里專區")]),
                        unofficial_name_list: ["Bheri"].to_vec(),
                    }
                ),
                (
                    "DH",
                    Subdivision{
                        name: "Dhawalagiri",
                        country_alpha2: Alpha2::NP,
                        code: "DH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.611176), longitude: Some(83.5070203), max_latitude: Some(29.3155821), min_latitude: Some(27.9980379), max_longitude: Some(84.19720490000002), min_longitude: Some(82.869958)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية دهوالاجيري"), ("bg", "Дхавалагири"), ("bn", "ধল\u{9be}গিরি অঞ\u{9cd}চল"), ("ccp", "𑄙\u{11127}𑄤𑄣𑄉\u{11128}𑄢\u{11128}"), ("ceb", "Dhawalāgiri Zone"), ("da", "Dhaulagiri Zone"), ("de", "Dhaulagiri"), ("el", "Νταουλαγκίρι"), ("en", "Dhawalagiri"), ("es", "Zona de Dhawalagiri"), ("eu", "Dhawalagiri Gunea"), ("fi", "Dhawalagiri"), ("fr", "Dhawalagiri"), ("gu", "ધવલાગિરી પ\u{acd}રા\u{a82}ત"), ("hi", "धौलागिरी अ\u{902}चल"), ("id", "Dhaulagiri Zone"), ("it", "Dhawalagiri"), ("ja", "ダウラギリ県"), ("kk", "Дхаулагири"), ("kn", "ಧ\u{ccc}ಲಗ\u{cbf}ರ\u{cbf} ವಲಯ"), ("ko", "다울라기리 구"), ("lt", "Dulagirio zona"), ("lv", "Dhaulāgiri zona"), ("mr", "धौलगरी झोन"), ("ms", "Dhaulagiri Zone"), ("nb", "Dhaulagiri sone"), ("ne", "धौलागिरी अञ\u{94d}चल"), ("nl", "Dhawalagiri"), ("no", "Dhaulagiri sone"), ("pl", "Dhawalagiri"), ("pt", "Dhaulagiri (zona)"), ("ru", "Дхаулагири"), ("si", "දව\u{dd4}ලග\u{dd2}ර\u{dd2} කල\u{dcf}පය"), ("sk", "Dhawalágiri"), ("sv", "Dhaulagiri Zone"), ("ta", "தவுலகிரி சோன\u{bcd}"), ("te", "డ\u{c4c}లగ\u{c3f}ర\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "ดาฮ\u{e4c}ลาก\u{e34}ร\u{e34} โซน"), ("tr", "Dhaulagiri Zon"), ("uk", "Дгаулагірі"), ("ur", "دھولاگیری زون"), ("vi", "Vùng Dhaulagiri"), ("zh", "道拉吉里专区")]),
                        unofficial_name_list: ["Dhawalagiri"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "Gandaki",
                        country_alpha2: Alpha2::NP,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.3732037), longitude: Some(84.4382721), max_latitude: Some(28.91346), min_latitude: Some(27.7426719), max_longitude: Some(85.19313799999999), min_longitude: Some(83.4406971)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية غانداكي"), ("bg", "Гандаки"), ("bn", "গণ\u{9cd}ডকী অঞ\u{9cd}চল"), ("ca", "zona de Gandaki"), ("ccp", "𑄉\u{11127}𑄚\u{11134}𑄘\u{11127}𑄇\u{11128}"), ("ceb", "Gandakī Zone"), ("da", "Gandaki Zone"), ("de", "Gandaki"), ("el", "Γκαντάκι"), ("en", "Gandaki"), ("es", "Zona de Gandaki"), ("eu", "Gandaki Gunea"), ("fi", "Gandaki"), ("fr", "Gandaki"), ("gu", "ગ\u{a82}ડકી પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{902}डकी अ\u{902}चल"), ("hy", "Գանդակի"), ("id", "Gandaki Zone"), ("it", "Gandaki"), ("ja", "ガンダキ県"), ("kn", "ಗಂಡಕ\u{cbf} ವಲಯ"), ("ko", "간다키 구"), ("lt", "Gandakio zona"), ("lv", "Gandakī zona"), ("mr", "ग\u{902}डकी झोन"), ("ms", "Gandaki Zone"), ("nb", "Gandaki sone"), ("ne", "गण\u{94d}डकी अञ\u{94d}चल"), ("nl", "Gandaki"), ("no", "Gandaki sone"), ("pl", "Gandaki"), ("pt", "Gandaki"), ("ru", "Гандаки"), ("si", "ගන\u{dca}ඩක\u{dd2} කල\u{dcf}පය"), ("sk", "Gandakí"), ("sv", "Gandaki Zone"), ("ta", "கண\u{bcd}டகி ஸ\u{bcd}ஒன\u{bcd}"), ("te", "గండ\u{c3e}క\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "เม\u{e37}องการ\u{e4c}ตาโก"), ("tr", "Gandaki Zon"), ("uk", "Гандакі"), ("ur", "گنداکی زون"), ("vi", "Vùng Gandaki"), ("zh", "甘达基专区")]),
                        unofficial_name_list: ["Gandaki"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "Janakpur",
                        country_alpha2: Alpha2::NP,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.728611), longitude: Some(85.925), max_latitude: Some(26.8383167), min_latitude: Some(26.6703051), max_longitude: Some(85.9942174), min_longitude: Some(85.8793826)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية جاناكبور"), ("bg", "Джанакпур"), ("bn", "জনকপ\u{9c1}র অঞ\u{9cd}চল"), ("ccp", "𑄎𑄚\u{11127}𑄇\u{11134}𑄛\u{1112a}𑄢\u{11134}"), ("ceb", "Janakpur Zone"), ("da", "Janakpur Zone"), ("de", "Janakpur"), ("el", "Τζανακπούρ"), ("en", "Janakpur"), ("es", "Zona de Janakpur"), ("eu", "Janakpur Gunea"), ("fa", "جاناکپور"), ("fi", "Janakpur"), ("fr", "Janakpur"), ("gu", "જનકપ\u{ac1}ર પ\u{acd}રા\u{a82}ત"), ("hi", "जनकप\u{941}र अ\u{902}चल"), ("id", "Janakpur Zone"), ("it", "Janakapura"), ("ja", "ジャナクプル県"), ("kn", "ಜನಕ\u{ccd}ಪುರ\u{ccd} ವಲಯ"), ("ko", "자낙푸르 구"), ("lt", "Džanakpuro zona"), ("lv", "Džanakpuras zona"), ("mr", "जनकप\u{942}र झोन"), ("ms", "Janakpur Zone"), ("nb", "Janakpur sone"), ("ne", "जनकप\u{941}र अञ\u{94d}चल"), ("nl", "Janakpur"), ("no", "Janakpur sone"), ("pl", "Dźanakpur"), ("pt", "Janakpur"), ("ru", "Джанакпур"), ("si", "ජනක\u{dca}ප\u{dd4}ර\u{dca} කල\u{dcf}පය"), ("sk", "Džanakpur"), ("sv", "Janakpur Zone"), ("ta", "ஜனக\u{bcd}பூர\u{bcd} ஸ\u{bcd}ஒன\u{bcd}"), ("te", "జనక\u{c4d}పూర\u{c4d} జ\u{c4b}న\u{c4d}"), ("th", "จาน\u{e31}กป\u{e39}ร\u{e4c} โซน"), ("tr", "Janakpur Zon"), ("uk", "Джанакпур"), ("ur", "جنکپور زون"), ("vi", "Vùng Janakpur"), ("zh", "賈納克布爾專區")]),
                        unofficial_name_list: ["Janakpur"].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "Karnali",
                        country_alpha2: Alpha2::NP,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.3862555), longitude: Some(82.38857829999999), max_latitude: Some(30.4333931), min_latitude: Some(28.7097791), max_longitude: Some(83.6897499), min_longitude: Some(81.255875)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية كارنالي"), ("bg", "Карнали"), ("bn", "কর\u{9cd}ণ\u{9be}লী অঞ\u{9cd}চল"), ("ccp", "𑄇𑄢\u{11134}𑄚𑄣\u{11128}"), ("ceb", "Karnālī Zone"), ("da", "Karnali Zone"), ("de", "Karnali"), ("el", "Καρνάλι"), ("en", "Karnali"), ("es", "Zona de Karnali"), ("eu", "Karnali Gunea"), ("fi", "Karnali"), ("fr", "Karnali"), ("gu", "કર\u{acd}ણાલી પ\u{acd}રા\u{a82}ત"), ("hi", "कर\u{94d}णाली अ\u{902}चल"), ("id", "Karnali Zone"), ("it", "Karnali"), ("ja", "カルナリ県"), ("kn", "ಕರ\u{ccd}ನಾಲ\u{cbf} ವಲಯ"), ("ko", "카르날리 구"), ("lt", "Karnalio zona"), ("lv", "Karnālī zona"), ("mr", "कर\u{94d}णली झोन"), ("ms", "Zon Karnali"), ("nb", "Karnali sone"), ("ne", "कर\u{94d}णाली अञ\u{94d}चल"), ("nl", "Karnali"), ("no", "Karnali sone"), ("pl", "Karnali"), ("pt", "Karnali"), ("ru", "Карнали"), ("si", "කමල\u{dd2} කල\u{dcf}පය"), ("sk", "Karnálí"), ("sv", "Karnali Zone"), ("ta", "கர\u{bcd}ன\u{bbe}லி ஸ\u{bcd}வ\u{bcd}ன\u{bcd}"), ("te", "కర\u{c4d}న\u{c3e}ల\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "คาร\u{e4c}นาล\u{e35}"), ("tr", "Kamali Zon"), ("uk", "Карналі"), ("ur", "کرنالی زون"), ("vi", "Vùng Karnali"), ("zh", "格爾納利專區")]),
                        unofficial_name_list: ["Karnali"].to_vec(),
                    }
                ),
                (
                    "KO",
                    Subdivision{
                        name: "Kosi [Koshi]",
                        country_alpha2: Alpha2::NP,
                        code: "KO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.5459247), longitude: Some(86.93737159999999), max_latitude: Some(26.8262154), min_latitude: Some(25.396598), max_longitude: Some(87.26650049999999), min_longitude: Some(86.4182005)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية كوسي"), ("bg", "Коси"), ("bn", "কোশী অঞ\u{9cd}চল"), ("ca", "Kosi"), ("ccp", "𑄇\u{1112e}𑄥\u{11128}"), ("ceb", "Kosī Zone"), ("de", "Kosi"), ("en", "Kosi"), ("es", "Zona de Kosi"), ("eu", "Koshi Gunea"), ("fi", "Kosi"), ("fr", "Koshi"), ("gu", "કોશી પ\u{acd}રા\u{a82}ત"), ("hi", "कोशी अ\u{902}चल"), ("it", "Kosi"), ("ja", "コシ県"), ("ko", "코시 구"), ("ne", "कोशी अञ\u{94d}चल"), ("nl", "Kosi"), ("pl", "Kośi"), ("pt", "Kosi"), ("ru", "Коси"), ("sk", "Koší"), ("ta", "கோசி மண\u{bcd}டலம\u{bcd}"), ("uk", "Косі"), ("ur", "کوسی زون"), ("zh", "戈西專區")]),
                        unofficial_name_list: ["Kosi [Koshi]"].to_vec(),
                    }
                ),
                (
                    "LU",
                    Subdivision{
                        name: "Lumbini",
                        country_alpha2: Alpha2::NP,
                        code: "LU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.45), longitude: Some(83.25), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية لومبيني"), ("be", "Лумбіні"), ("bg", "Лумбини"), ("bn", "ল\u{9c1}ম\u{9cd}বিনী অঞ\u{9cd}চল"), ("ccp", "𑄣\u{1112a}𑄟\u{11134}𑄝\u{11128}𑄚\u{11128}"), ("ceb", "Lumbinī Zone"), ("de", "Lumbini"), ("el", "Λουμπίνι"), ("en", "Lumbini"), ("es", "Zona de Lumbini"), ("et", "Luṁbinī ringkond"), ("eu", "Lumbini Gunea"), ("fi", "Lumbini"), ("fr", "Lumbinî"), ("gu", "લ\u{ac1}મ\u{acd}બિની પ\u{acd}રા\u{a82}ત"), ("hy", "Լումբինի"), ("it", "Lumbini"), ("ja", "ルンビニ県"), ("ko", "룸비니 구"), ("mk", "Лумбини"), ("ne", "ल\u{941}म\u{94d}बिनी अञ\u{94d}चल"), ("nl", "Lumbini"), ("pl", "Lumbini"), ("pt", "Lumbini"), ("ru", "Лумбини"), ("sk", "Lumbiní"), ("ta", "லும\u{bcd}பினி மண\u{bcd}டலம\u{bcd}"), ("ur", "لومبینی زون"), ("zh", "藍毗尼專區")]),
                        unofficial_name_list: ["Lumbini"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "Mahakali",
                        country_alpha2: Alpha2::NP,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.39), longitude: Some(80.3), max_latitude: Some(29.4097501), min_latitude: Some(29.3691794), max_longitude: Some(80.33278229999999), min_longitude: Some(80.2704906)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية ماهاكالي"), ("bg", "Махакали"), ("bn", "মহ\u{9be}ক\u{9be}লী অঞ\u{9cd}চল"), ("ccp", "𑄟\u{11127}𑄦𑄇\u{11127}𑄣\u{11128}"), ("ceb", "Mahākālī Zone"), ("da", "Mahakali Zone"), ("de", "Mahakali"), ("el", "Μαχακάλι"), ("en", "Mahakali"), ("es", "Zona de Mahakali"), ("eu", "Mahakali Gunea"), ("fi", "Mahakali"), ("fr", "Mahakali"), ("gu", "મહાકાલી પ\u{acd}રા\u{a82}ત"), ("hi", "महाकाली जोन"), ("id", "Mahakali Zone"), ("it", "Mahakali"), ("ja", "マハカリ県"), ("kn", "ಮಹಾಕಾಳ\u{cbf} ವಲಯ"), ("ko", "마하칼리 구"), ("lt", "Mahakalio zona"), ("lv", "Mahākālī zona"), ("mr", "महाकाली झोन"), ("ms", "Zon Mahakali"), ("nb", "Mahakali Sone"), ("ne", "महाकाली अञ\u{94d}चल"), ("nl", "Mahakali"), ("no", "Mahakali Sone"), ("pl", "Mahakali"), ("pt", "Mahakali"), ("ru", "Махакали"), ("si", "මහ\u{dcf}ක\u{dcf}ල\u{dd2} කල\u{dcf}පය"), ("sk", "Mahákálí"), ("sv", "Mahakali Zone"), ("ta", "மஹ\u{bbe}க\u{bbe}ளி ஸ\u{bcd}வ\u{bcd}ன\u{bcd}"), ("te", "మహ\u{c3e}క\u{c3e}ళ\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "เขตมหากาล\u{e35}"), ("tr", "Mahakali zone"), ("uk", "Махакалі"), ("ur", "مہاکالی زون"), ("vi", "Vùng Mahakali"), ("zh", "馬哈卡利專區")]),
                        unofficial_name_list: ["Mahakali"].to_vec(),
                    }
                ),
                (
                    "ME",
                    Subdivision{
                        name: "Mechi",
                        country_alpha2: Alpha2::NP,
                        code: "ME",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.8760007), longitude: Some(87.9334803), max_latitude: Some(27.9174019), min_latitude: Some(26.361404), max_longitude: Some(88.19932589999999), min_longitude: Some(87.4689351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية ميتشي"), ("bg", "Мечи"), ("bn", "মেচী অঞ\u{9cd}চল"), ("ccp", "𑄟𑄬𑄌\u{11128}"), ("ceb", "Mechī Zone"), ("da", "Mechi Zone"), ("de", "Mechi"), ("el", "Μέτσι"), ("en", "Mechi"), ("es", "Zona de Mechi"), ("eu", "Mechi Gunea"), ("fi", "Mechi"), ("fr", "Mechi"), ("gu", "મ\u{ac7}ચી પ\u{acd}રા\u{a82}ત"), ("hi", "म\u{947}ची अञ\u{94d}चल"), ("id", "Mechi Zone"), ("it", "Mechi"), ("ja", "メチ県"), ("kn", "ಮ\u{cc6}ಚ\u{cbf} ವಲಯ"), ("ko", "메치 구"), ("lt", "Mečio zona"), ("lv", "Mečī zona"), ("mr", "म\u{947}ची झोन"), ("ms", "Mechi Zone"), ("nb", "Mechi"), ("ne", "म\u{947}ची अञ\u{94d}चल"), ("nl", "Mechi"), ("no", "Mechi"), ("pl", "Meći"), ("pt", "Mechi"), ("ru", "Мечи"), ("si", "මෙච\u{dd2} කල\u{dcf}පය"), ("sk", "Mečhí"), ("sv", "Mechi"), ("ta", "மெச\u{bcd}சி ஸ\u{bcd}யோனே"), ("te", "మ\u{c46}చ\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "เมจ\u{e34}"), ("tr", "Mechi Zon"), ("uk", "Мечі"), ("ur", "میچی زون"), ("vi", "Vùng Mechi"), ("zh", "梅吉專區")]),
                        unofficial_name_list: ["Mechi"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "Narayani",
                        country_alpha2: Alpha2::NP,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.5863482), longitude: Some(84.3226488), max_latitude: Some(27.6141404), min_latitude: Some(27.5510084), max_longitude: Some(84.344962), min_longitude: Some(84.1606619)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية ناراياني"), ("bg", "Нараяни"), ("bn", "ন\u{9be}র\u{9be}য\u{9bc}ণী অঞ\u{9cd}চল"), ("ccp", "𑄚𑄢𑄠\u{11127}𑄚\u{11128}"), ("ceb", "Nārāyanī Zone"), ("da", "Narayani Zone"), ("de", "Narayani"), ("el", "Ναραγιάνι"), ("en", "Narayani"), ("es", "Zona de Narayani"), ("eu", "Narayani Gunea"), ("fi", "Narayani"), ("fr", "Narayani"), ("gu", "નારાયણી પ\u{acd}રા\u{a82}ત"), ("hi", "नारायणी जोन"), ("id", "Narayani Zone"), ("it", "Narayani"), ("ja", "ナラヤニ県"), ("kn", "ನಾರಾಯಣ\u{cbf} ವಲಯ"), ("ko", "나라야니 구"), ("lt", "Narajanio zona"), ("lv", "Nārājanī zona"), ("mr", "नारायणी झोन"), ("ms", "Narayani Zone"), ("nb", "Narayani Sone"), ("ne", "नारायणी अञ\u{94d}चल"), ("nl", "Narayani"), ("no", "Narayani Sone"), ("pl", "Narajani"), ("pt", "Narayani (Nepal)"), ("ru", "Нараяни"), ("si", "නරයන\u{dd2} කල\u{dcf}පය"), ("sk", "Nárájaní"), ("sv", "Narayani Zone"), ("ta", "ந\u{bbe}ர\u{bbe}யணி ஸ\u{bcd}யோனே"), ("te", "న\u{c3e}ర\u{c3e}యన\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "นารางาน\u{e34}"), ("tr", "Narayani Zon"), ("uk", "Нараяні"), ("ur", "نارائنی زون"), ("vi", "Vùng Narayani"), ("zh", "納拉亞尼專區")]),
                        unofficial_name_list: ["Narayani"].to_vec(),
                    }
                ),
                (
                    "P1",
                    Subdivision{
                        name: "Province 1",
                        country_alpha2: Alpha2::NP,
                        code: "P1",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Province 1")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P2",
                    Subdivision{
                        name: "Province 2",
                        country_alpha2: Alpha2::NP,
                        code: "P2",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Province 2")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P3",
                    Subdivision{
                        name: "Province 3",
                        country_alpha2: Alpha2::NP,
                        code: "P3",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Province 3")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P4",
                    Subdivision{
                        name: "Gandaki²",
                        country_alpha2: Alpha2::NP,
                        code: "P4",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.394857), longitude: Some(84.12400799999999), max_latitude: Some(30.4473898), min_latitude: Some(26.3473741), max_longitude: Some(88.20182969999999), min_longitude: Some(80.05846980000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Gandaki²")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P5",
                    Subdivision{
                        name: "Province 5",
                        country_alpha2: Alpha2::NP,
                        code: "P5",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Province 5")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P6",
                    Subdivision{
                        name: "Karnali²",
                        country_alpha2: Alpha2::NP,
                        code: "P6",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.394857), longitude: Some(84.12400799999999), max_latitude: Some(30.4473898), min_latitude: Some(26.3473741), max_longitude: Some(88.20182969999999), min_longitude: Some(80.05846980000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Karnali²")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "P7",
                    Subdivision{
                        name: "Province 7",
                        country_alpha2: Alpha2::NP,
                        code: "P7",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Province 7")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "Rapti",
                        country_alpha2: Alpha2::NP,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.274347), longitude: Some(82.38857829999999), max_latitude: Some(28.99212589999999), min_latitude: Some(27.682508), max_longitude: Some(83.1547849), min_longitude: Some(81.7472689)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية رابتي"), ("bg", "Рапти"), ("bn", "র\u{9be}প\u{9cd}তী অঞ\u{9cd}চল"), ("ccp", "𑄢𑄛\u{11134}𑄑\u{11128}"), ("ceb", "Rāptī Zone"), ("da", "Rapti Zone"), ("de", "Rapti"), ("el", "Ράπτι"), ("en", "Rapti"), ("es", "Zona de Rapti"), ("eu", "Rapti Gunea"), ("fi", "Rapti"), ("fr", "Rapti"), ("gu", "રાપ\u{acd}તી પ\u{acd}રા\u{a82}ત"), ("hi", "र\u{948}प\u{94d}टी जोन"), ("id", "Rapti Zone"), ("it", "Rapti"), ("ja", "ラプティ県"), ("kn", "ರಾಪ\u{ccd}ತ\u{cbf} ವಲಯ"), ("ko", "랍티 구"), ("lt", "Rapčio zona"), ("lv", "Rāptī zona"), ("mr", "राती झोन"), ("ms", "Rapti Zone"), ("nb", "Rapti sone"), ("ne", "राप\u{94d}ती अञ\u{94d}चल"), ("nl", "Rapti"), ("no", "Rapti sone"), ("pl", "Rapti"), ("pt", "Rapti"), ("ru", "Рапти"), ("si", "රප\u{dca}ට\u{dd2} කල\u{dcf}පය"), ("sk", "Ráptí"), ("sv", "Rapti Zone"), ("ta", "ர\u{bbe}ப\u{bcd}தி ஸ\u{bcd}யோனே"), ("te", "ర\u{c3e}ప\u{c4d}త\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "แรฟต\u{e34}โซน"), ("tr", "Rapti Zone"), ("uk", "Рапті"), ("ur", "راپتی زون"), ("vi", "Vùng Rapti"), ("zh", "拉布蒂專區")]),
                        unofficial_name_list: ["Rapti"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "Sagarmatha",
                        country_alpha2: Alpha2::NP,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.9878093), longitude: Some(86.9249842), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية ساجارماثا"), ("bg", "Сагарматха"), ("bn", "সগরম\u{9be}থ\u{9be} অঞ\u{9cd}চল"), ("ca", "zona del Sagarmatha"), ("ccp", "𑄥𑄉\u{11127}𑄢\u{11134}𑄟𑄗"), ("ceb", "Sagarmāthā Zone"), ("da", "Sagarmatha Zone"), ("de", "Sagarmatha"), ("el", "Σαγκαρμάθα"), ("en", "Sagarmatha"), ("es", "Zona de Sagarmatha"), ("eu", "Sagarmatha Gunea"), ("fi", "Sagarmatha"), ("fr", "Sagarmatha"), ("gu", "સગરમાથા પ\u{acd}રા\u{a82}ત"), ("hi", "सगरमाथा अ\u{902}चल"), ("id", "Sagarmatha Zone"), ("it", "Sagarmatha"), ("ja", "サガルマタ県"), ("kn", "ಸಾಗರ\u{ccd}ಮಾತಾ ವಲಯ"), ("ko", "사가르마타 구"), ("lt", "Sagarmatos zona"), ("lv", "Sagarmāthas zona"), ("mr", "सागरमाथा झोन"), ("ms", "Sagarmatha Zone"), ("nb", "Sagarmatha Sone"), ("ne", "सगरमाथा अञ\u{94d}चल"), ("nl", "Sagarmatha"), ("no", "Sagarmatha Sone"), ("pl", "Sagarmatha"), ("pt", "Sagarmatha"), ("ru", "Сагарматха"), ("si", "ස\u{dcf}ගර\u{dca}ම\u{dcf}ත\u{dcf} කල\u{dcf}පය"), ("sk", "Sagarmáthá"), ("sv", "Sagarmatha"), ("ta", "சகர\u{bcd}ம\u{bbe}த\u{bbe} ஸ\u{bcd}வ\u{bcd}ன\u{bcd}"), ("te", "స\u{c3e}గర\u{c4d}మ\u{c3e}త జ\u{c4b}న\u{c4d}"), ("th", "เขตสกรมาธา"), ("tr", "Sagarmatha Zon"), ("uk", "Сагарматха"), ("ur", "ساگرمتھا زون"), ("vi", "Vùng Sagarmatha"), ("zh", "萨加玛塔专区")]),
                        unofficial_name_list: ["Sagarmatha"].to_vec(),
                    }
                ),
                (
                    "SE",
                    Subdivision{
                        name: "Seti",
                        country_alpha2: Alpha2::NP,
                        code: "SE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.6905427), longitude: Some(81.3399414), max_latitude: Some(30.0715771), min_latitude: Some(28.394239), max_longitude: Some(81.815972), min_longitude: Some(80.5038579)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Zone,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مديرية سيتي"), ("bg", "Сети"), ("bn", "সেতী অঞ\u{9cd}চল"), ("ccp", "𑄥𑄬𑄑\u{11128}"), ("ceb", "Setī Zone"), ("da", "Seti Zone"), ("de", "Seti"), ("el", "Σέτι"), ("en", "Seti"), ("es", "Zona de Seti"), ("eu", "Seti Gunea"), ("fi", "Seti"), ("fr", "Seti"), ("gu", "સ\u{ac7}તી પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{947}ती अ\u{902}चल"), ("id", "Seti Zone"), ("it", "Seti"), ("ja", "セティ県"), ("kn", "ಸ\u{cc6}ಟ\u{cbf} ವಲಯ"), ("ko", "세티 구"), ("lt", "Sečio zona"), ("lv", "Setī zona"), ("mr", "स\u{947}टी झोन"), ("ms", "Seti Zone"), ("nb", "Seti"), ("ne", "स\u{947}ती अञ\u{94d}चल"), ("nl", "Seti"), ("no", "Seti"), ("pl", "Seti"), ("pt", "Seti"), ("ru", "Сетхи"), ("si", "සෙට\u{dd2} කල\u{dcf}පය"), ("sk", "Setí"), ("sv", "Seti"), ("ta", "செடி ஸ\u{bcd}ஒன\u{bcd}"), ("te", "స\u{c47}ట\u{c3f} జ\u{c4b}న\u{c4d}"), ("th", "กราเซ\u{e35}ย อ อ\u{e34}ออส"), ("tr", "Seti Zon"), ("uk", "Сеті"), ("ur", "سیتی زون"), ("vi", "Vùng Seti"), ("zh", "塞蒂專區")]),
                        unofficial_name_list: ["Seti"].to_vec(),
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
#[cfg(feature = "np")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::NP,
        alpha3: Alpha3::NPL,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}}\n{{region}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 977,
        currency_code: CurrencyCode::NPR,
        gec: Some(GEC::NP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::NEP),
        iso_long_name: "The Federal Democratic Republic of Nepal",
        iso_short_name: "Nepal",
        official_language_list: ["ne"].to_vec(),
        spoken_language_list: ["bho", "mai", "ne", "new", "urd"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8].to_vec(),
        national_prefix: "0",
        nationality: Some("Nepalese"),
        number: "524",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "NP",
        unofficial_name_list: [
            "Nepal",
            "Népal",
            "the Federal Democratic Republic of Nepal",
            "ネパール",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Nepal"),
            ("af", "Nepal"),
            ("ak", "Nepal"),
            ("am", "ኔፓሔ"),
            ("an", "Nepal"),
            ("ar", "نيبال"),
            ("as", "নেপ\u{9be}ল"),
            ("ay", "Nepal"),
            ("az", "Nepal"),
            ("ba", "Nepal"),
            ("be", "Непал"),
            ("bg", "Непал"),
            ("bi", "Nepal"),
            ("bn", "নেপ\u{9be}ল"),
            ("bn_IN", "নেপ\u{9be}ল"),
            ("br", "Nepal"),
            ("bs", "Nepal"),
            ("ca", "Nepal"),
            ("ce", "Непал"),
            ("ch", "Nepal"),
            ("cs", "Nepál"),
            ("cv", "Непал"),
            ("cy", "Nepal"),
            ("da", "Nepal"),
            ("de", "Nepal"),
            ("dv", "ނ\u{7ad}ޕ\u{7a7}ލ\u{7b0}"),
            ("dz", "ན\u{f7a}་པ\u{f71}ལ།"),
            ("ee", "Nepal"),
            ("el", "Νεπάλ"),
            ("en", "Nepal"),
            ("eo", "Nepalo"),
            ("es", "Nepal"),
            ("et", "Nepal"),
            ("eu", "Nepal"),
            ("fa", "نپال"),
            ("ff", "Nepal"),
            ("fi", "Nepal"),
            ("fo", "Nepal"),
            ("fr", "Népal"),
            ("fy", "Nepal"),
            ("ga", "Neipeal"),
            ("gl", "Nepal"),
            ("gn", "Nepal"),
            ("gu", "ન\u{ac7}પાળ"),
            ("gv", "Nepaal"),
            ("ha", "Nepal"),
            ("he", "נפאל"),
            ("hi", "न\u{947}पाल"),
            ("hr", "Nepal"),
            ("ht", "Nepal"),
            ("hu", "Nepál"),
            ("hy", "Նեպալ"),
            ("ia", "Nepal"),
            ("id", "Nepal"),
            ("io", "Nepal"),
            ("is", "Nepal"),
            ("it", "Nepal"),
            ("iu", "Nepal"),
            ("ja", "ネパール"),
            ("ka", "ნეპალი"),
            ("ki", "Nepal"),
            ("kk", "Непал"),
            ("kl", "Nepal"),
            ("km", "នេប\u{17c9}ាល\u{17cb}"),
            ("kn", "ನೇಪಾಳ"),
            ("ko", "네팔"),
            ("ku", "Nepal"),
            ("kv", "Непал"),
            ("kw", "Nepal"),
            ("ky", "Непал"),
            ("lo", "Nepal"),
            ("lt", "Nepalas"),
            ("lv", "Nepāla"),
            ("mi", "Nepōra"),
            ("mk", "Непал"),
            ("ml", "നേപ\u{d3e}ള\u{d4d}\u{200d}"),
            ("mn", "Нипал"),
            ("mr", "न\u{947}पाळ"),
            ("ms", "Nepal"),
            ("mt", "Nepal"),
            ("my", "န\u{102e}ပေါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Nepar"),
            ("nb", "Nepal"),
            ("ne", "न\u{947}पाल"),
            ("nl", "Nepal"),
            ("nn", "Nepal"),
            ("nv", "Dziłghą\u{301}ąʼdi Naakaii Dootłʼizhí Bikéyah"),
            ("oc", "Nepal"),
            ("or", "ନେପ\u{b3e}ଳ"),
            ("pa", "ਨ\u{a47}ਪਾਲ"),
            ("pi", "न\u{947}पाल"),
            ("pl", "Nepal"),
            ("ps", "نیپال"),
            ("pt", "Nepal"),
            ("pt_BR", "Nepal"),
            ("ro", "Nepal"),
            ("ru", "Непал"),
            ("rw", "Nepali"),
            ("sc", "Nepal"),
            ("sd", "نيپال"),
            ("si", "නේප\u{dcf}ලය"),
            ("sk", "Nepál"),
            ("sl", "Nepal"),
            ("so", "Nepal"),
            ("sq", "Nepal"),
            ("sr", "Непал"),
            ("sv", "Nepal"),
            ("sw", "Nepal"),
            ("ta", "நேப\u{bbe}ளம\u{bcd}"),
            ("te", "న\u{c47}ప\u{c3e}ల\u{c4d}"),
            ("tg", "Непал"),
            ("th", "เนปาล"),
            ("ti", "ኔፓል"),
            ("tk", "Nepal"),
            ("tl", "Nepal"),
            ("tr", "Nepal"),
            ("tt", "Непал"),
            ("ug", "نېپال"),
            ("uk", "Непал"),
            ("ur", "نیپال"),
            ("uz", "Nepal"),
            ("ve", "Nepal"),
            ("vi", "Nê-pan"),
            ("wa", "Nepal"),
            ("wo", "Nepaal"),
            ("xh", "Nepal"),
            ("yo", "Nepal"),
            ("zh_CN", "尼泊尔"),
            ("zh_HK", "尼泊爾"),
            ("zh_TW", "尼泊爾"),
            ("zu", "Nepal"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
