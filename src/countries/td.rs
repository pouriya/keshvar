// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Chad

#[cfg(all(feature = "td", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::TD;
    pub const ALPHA3: Alpha3 = Alpha3::TCD;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 235;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::XAF;
    pub const GEC: Option<GEC> = Some(GEC::CD);
    pub const INTERNATIONAL_PREFIX: &str = "15";
    pub const IOC: Option<IOC> = Some(IOC::CHA);
    pub const ISO_SHORT_NAME: &str = "Chad";
    pub const ISO_LONG_NAME: &str = "The Republic of Chad";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Chadian");
    pub const NUMBER: &str = "148";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::MiddleAfrica);
    pub const UN_LOCODE: &str = "TD";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Chad", "تشاد", "Tschad", "Tchad", "チャド", "Tsjaad"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Chad"),
        ("af", "Tsjad"),
        ("ak", "Chad"),
        ("am", "ኂ፥"),
        ("an", "Chad"),
        ("ar", "تشاد"),
        ("as", "চ\u{9be}ড"),
        ("ay", "Chad"),
        ("az", "Çad"),
        ("ba", "Chad"),
        ("be", "Чад"),
        ("bg", "Чад"),
        ("bi", "Chad"),
        ("bn", "চ\u{9be}ড"),
        ("bn_IN", "চ\u{9be}ড"),
        ("br", "Tchad"),
        ("bs", "Čad"),
        ("ca", "Txad"),
        ("ce", "Чад"),
        ("ch", "Chad"),
        ("cs", "Čad"),
        ("cv", "Чад"),
        ("cy", "Chad"),
        ("da", "Tchad"),
        ("de", "Tschad"),
        ("dv", "ޝ\u{7a7}ދ\u{7aa}"),
        ("dz", "ཆཌ\u{f72}།"),
        ("ee", "Chad"),
        ("el", "Τσαντ"),
        ("en", "Chad"),
        ("eo", "Ĉado"),
        ("es", "Chad"),
        ("et", "Tšaad"),
        ("eu", "Txad"),
        ("fa", "چاد"),
        ("ff", "Chad"),
        ("fi", "Tšad"),
        ("fo", "Kjad"),
        ("fr", "Tchad"),
        ("fy", "Tsjaad"),
        ("ga", "Sead"),
        ("gl", "Chad"),
        ("gn", "Chad"),
        ("gu", "ચાડ"),
        ("gv", "Shad"),
        ("ha", "Cadi"),
        ("he", "צ׳אד"),
        ("hi", "चाड"),
        ("hr", "Čad"),
        ("ht", "Tchad"),
        ("hu", "Csád"),
        ("hy", "Չադ"),
        ("ia", "Tchad"),
        ("id", "Chad"),
        ("io", "Chad"),
        ("is", "Tsjad"),
        ("it", "Ciad"),
        ("iu", "Chad"),
        ("ja", "チャド"),
        ("ka", "ჩადი"),
        ("ki", "Chad"),
        ("kk", "Чад"),
        ("kl", "Chad"),
        ("km", "ឆាដ"),
        ("kn", "ಛಾಡ\u{ccd}"),
        ("ko", "차드"),
        ("ku", "Çad"),
        ("kv", "Chad"),
        ("kw", "Chad"),
        ("ky", "Чад"),
        ("lo", "Chad"),
        ("lt", "Čadas"),
        ("lv", "Čada"),
        ("mi", "Chad"),
        ("mk", "Чад"),
        ("ml", "ഛ\u{d3e}ഡ\u{d4d}"),
        ("mn", "Чад"),
        ("mr", "चाद"),
        ("ms", "Cad"),
        ("mt", "Ċad"),
        (
            "my",
            "ချဒ\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tsiad"),
        ("nb", "Tsjad"),
        ("ne", "चाद"),
        ("nl", "Tsjaad"),
        ("nn", "Tsjad"),
        ("nv", "Chad"),
        ("oc", "Chad"),
        ("or", "ଚ\u{b3e}ଡ\u{b4d}"),
        ("pa", "ਚਾਦ"),
        ("pi", "चाड"),
        ("pl", "Czad"),
        ("ps", "چاډ"),
        ("pt", "Chade"),
        ("pt_BR", "Chade"),
        ("ro", "Ciad"),
        ("ru", "Чад"),
        ("rw", "Cade"),
        ("sc", "Chad"),
        ("sd", "چاڊ"),
        ("si", "cqDw"),
        ("sk", "Čad"),
        ("sl", "Čad"),
        ("so", "Jaad"),
        ("sq", "Çad"),
        ("sr", "Чад"),
        ("sv", "Tchad"),
        ("sw", "Chad"),
        ("ta", "ச\u{bbe}ட\u{bcd}"),
        ("te", "చ\u{c3e}ద\u{c4d}"),
        ("tg", "Чад"),
        ("th", "ชาด"),
        ("ti", "ቻድ"),
        ("tk", "Çad"),
        ("tl", "Tsad"),
        ("tr", "Çad"),
        ("tt", "Чад"),
        ("ug", "چاد"),
        ("uk", "Чад"),
        ("ur", "چاڈ"),
        ("uz", "Chad"),
        ("ve", "Chad"),
        ("vi", "Chê-đ"),
        ("wa", "Tchad"),
        ("wo", "Caad"),
        ("xh", "Chad"),
        ("yo", "Tsad"),
        ("zh_CN", "乍得"),
        ("zh_HK", "乍得"),
        ("zh_TW", "查德"),
        ("zu", "ITshedi"),
    ];
    #[cfg(all(feature = "td", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 15.454166;
        pub const LONGITUDE: f64 = 18.732207;
        pub const MAX_LATITUDE: f64 = 23.449228;
        pub const MAX_LONGITUDE: f64 = 24.0000011;
        pub const MIN_LATITUDE: f64 = 7.442975;
        pub const MIN_LONGITUDE: f64 = 13.4699999;
        pub const NORTHEAST_LATITUDE: f64 = 23.449228;
        pub const NORTHEAST_LONGITUDE: f64 = 24.0000011;
        pub const SOUTHWEST_LATITUDE: f64 = 7.442975;
        pub const SOUTHWEST_LONGITUDE: f64 = 13.4699999;
    }
}
#[cfg(all(feature = "td", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 15.454166,
            longitude: 18.732207,
            max_latitude: 23.449228,
            max_longitude: 24.0000011,
            min_latitude: 7.442975,
            min_longitude: 13.4699999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 23.449228,
                    longitude: 24.0000011,
                },
                southwest: CountryGeoBound {
                    latitude: 7.442975,
                    longitude: 13.4699999,
                },
            },
        }
    }
}

#[cfg(all(feature = "td", feature = "subdivisions"))]
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
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::TD,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.9371775), longitude: Some(18.4276047), max_latitude: Some(16.076306), min_latitude: Some(12.1039549), max_longitude: Some(20.3562929), min_longitude: Some(16.9447559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة البطحة"), ("bg", "Бата"), ("bn", "ব\u{9be}থ\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄝𑄗"), ("ceb", "Batha Region"), ("cs", "Batha"), ("da", "Batha"), ("de", "Region Batha"), ("el", "Μπάθα"), ("en", "Batha"), ("es", "Región de Batha"), ("et", "Batha"), ("eu", "Batha"), ("fi", "Batha"), ("fr", "Batha"), ("gu", "બાથા પ\u{acd}રદ\u{ac7}શ"), ("hi", "बाथा प\u{94d}रद\u{947}श"), ("id", "Region Batha"), ("it", "regione di Batha"), ("ja", "バタ州"), ("ka", "ბათის რეგიონი"), ("kn", "ಬಾತ ಪ\u{ccd}ರದೇಶ"), ("ko", "바타 주"), ("lt", "Batos regionas"), ("lv", "Batas reģions"), ("mr", "बाठा प\u{94d}रद\u{947}श"), ("ms", "Batha Region"), ("nb", "Batha"), ("nl", "Batha"), ("no", "Batha"), ("pl", "Region Batha"), ("pt", "Batha"), ("ro", "Regiunea Batha"), ("ru", "Батха"), ("si", "බ\u{dcf}ත\u{dcf} කල\u{dcf}පය"), ("sv", "Batha (region)"), ("ta", "ப\u{bbe}த\u{bcd}த\u{bbe} பகுதி"), ("te", "బ\u{c3e}త\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "บาธา"), ("tr", "Batha Bölgesi"), ("uk", "Батха"), ("ur", "باثا علاقہ"), ("vi", "Khu vực Batha"), ("zh", "巴塔区")]),
                        unofficial_name_list: ["Batha"].to_vec(),
                    }
                ),
                (
                    "BG",
                    Subdivision{
                        name: "BG",
                        country_alpha2: Alpha2::TD,
                        code: "BG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بحر الغزال"), ("bg", "Бар ел Газел"), ("bn", "ব\u{9be}র এল গ\u{9be}জেল অঞ\u{9cd}চল"), ("ccp", "𑄝𑄦\u{11134}𑄢\u{11134} 𑄃𑄬𑄣\u{11134} 𑄉𑄎𑄬𑄣\u{11134}"), ("ceb", "Barh el Gazel"), ("cs", "Barh El Gazel"), ("da", "Barh El Gazel"), ("de", "Region Barh El Gazel"), ("el", "Μπαχρ ελ Γκαζέλ"), ("en", "Bahr el Gazel"), ("es", "Región de Barh El Gazel"), ("eu", "Barh El Gazel"), ("fi", "Barh el Gazel"), ("fr", "Barh El Gazel"), ("gu", "બાહર અલ ગ\u{ac7}ઝલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "बहल एल ग\u{947}ज\u{93c}\u{947}ल क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Bahr el Gazel"), ("it", "regione di Barh El Gazel"), ("ka", "ბაჰრ-ელ-გაზელის რეგიონი"), ("kn", "ಬಹರ\u{ccd} ಎಲ\u{ccd} ಗಸ\u{cc6}ಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "바르엘가젤 주"), ("lt", "Bahr El Gazelio regionas"), ("lv", "Bahrelgazalas reģions"), ("mr", "बहल एल गाझ\u{947}ल प\u{94d}रद\u{947}श"), ("ms", "Bahr el Gazel Region"), ("nb", "Bahr el Gazel region"), ("nl", "Bahr el Gazel Region"), ("no", "Bahr el Gazel region"), ("pl", "Region Bahr El Gazel"), ("pt", "Barh El Gazel"), ("ro", "Regiunea Barh el Ghazel"), ("ru", "Бахр-эль-Газаль"), ("si", "බහර\u{dca} එල\u{dca} ගසෙල\u{dca} කල\u{dcf}පය"), ("sv", "Bahr el Gazel (region)"), ("ta", "ப\u{bbe}ர\u{bcd} எல\u{bcd} கஸில\u{bcd} பகுதி"), ("te", "బహర\u{c4d} ఎల\u{c4d} గ\u{c3e}జ\u{c46}ల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "บาฮ\u{e4c} เอล กาเซล"), ("tr", "Bahr el Gazel Bölgesi"), ("uk", "Бахр-ель-Газаль"), ("ur", "بحر الغزال علاقہ"), ("vi", "Khu vực Bahr el Gazel"), ("zh", "加扎勒河區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BO",
                    Subdivision{
                        name: "BO",
                        country_alpha2: Alpha2::TD,
                        code: "BO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة بوركو"), ("bn", "ব\u{9c1}রক\u{9c1} অঞ\u{9cd}চল"), ("ca", "Borku"), ("ccp", "𑄝\u{1112e}𑄢\u{11134}𑄇\u{1112f}"), ("ceb", "Borkou Region"), ("cs", "Borkou"), ("da", "Borkou"), ("de", "Region Borkou"), ("el", "Μπόρκου"), ("en", "Borkou"), ("es", "Región de Borkou"), ("eu", "Borkou"), ("fi", "Borkou"), ("fr", "Borkou"), ("gu", "બૉર\u{acd}કાઉ પ\u{acd}રદ\u{ac7}શ"), ("hi", "बोर\u{94d}क\u{942} प\u{94d}रद\u{947}श"), ("hy", "Բորկու"), ("id", "Wilayah Borkou"), ("it", "regione di Borkou"), ("ja", "ボルク州"), ("ka", "ბორკუს რეგიონი"), ("kn", "ಬೊರ\u{ccd}ಕೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "보르쿠 주"), ("lt", "Borku regionas"), ("lv", "Borku reģions"), ("mr", "बोरोक\u{942} प\u{94d}रा\u{902}त"), ("ms", "Borkou Region"), ("nb", "Borkou Region"), ("nl", "Borkou Region"), ("no", "Borkou Region"), ("pl", "Region Borku"), ("pt", "Região Borkou"), ("ro", "Regiunea Borkou"), ("ru", "Борку"), ("si", "බොර\u{dca}කොව\u{dd6} කල\u{dcf}පය"), ("sv", "Borkou (region)"), ("ta", "போர\u{bcd}கோவு பகுதி"), ("te", "బ\u{c4b}ర\u{c4d}క\u{c4b}వ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แถบบอค\u{e39}ย\u{e4c}"), ("tr", "Borkou Bölgesi"), ("uk", "Борку"), ("ur", "بورکؤ علاقہ"), ("vi", "Khu vực Borkou"), ("zh", "博爾庫區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CB",
                    Subdivision{
                        name: "CB",
                        country_alpha2: Alpha2::TD,
                        code: "CB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.2564207), longitude: Some(16.1580937), max_latitude: Some(12.379939), min_latitude: Some(9.802192), max_longitude: Some(17.506004), min_longitude: Some(14.8912919)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة كانم"), ("bg", "Шари-Багирми"), ("bn", "চ\u{9be}রি-ব\u{9be}গ\u{9c1}রমি অঞ\u{9cd}চল"), ("ca", "Regió de Chari-Baguirmi"), ("ccp", "𑄌𑄢\u{11128}-Chari-Baguirmi"), ("ceb", "Chari-Baguirmi Region"), ("cs", "Chari-Baguirmi"), ("da", "Chari-Baguirmi"), ("de", "Region Chari-Baguirmi"), ("el", "Τσάρι-Μπαγκουίρμι"), ("en", "Chari-Baguirmi"), ("es", "Región de Chari-Baguirmi"), ("et", "Chari-Baguirmi"), ("eu", "Chari-Baguirmi"), ("fi", "Chari-Baguirmi"), ("fr", "Chari-Baguirmi"), ("gu", "ચારી-બાગ\u{ac1}રમી પ\u{acd}રદ\u{ac7}શ"), ("he", "צ׳ארי-בגוירמי"), ("hi", "चारी-बग\u{941}रनी क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Chari-Baguirmi"), ("it", "regione di Chari-Baguirmi"), ("ka", "შარი-ბაგირმის რეგიონი"), ("kn", "ಚರ\u{cbf}-ಬಗ\u{ccd}ಯುರ\u{cbf}ಮ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "샤리바기르미 주"), ("lt", "Šario–Bagirmio regionas"), ("lv", "Šari-Bagirmi reģions"), ("mr", "चारी-बाग\u{941}इरी प\u{94d}रद\u{947}श"), ("ms", "Chari-Baguirmi Region"), ("nb", "Chari-Baguirmi"), ("nl", "Chari-Baguirmi"), ("no", "Chari-Baguirmi"), ("pl", "Region Szari-Bagirmi"), ("pt", "Região de Chari-Baguirmi"), ("ro", "Regiunea Chari-Baguirmi"), ("ru", "Шари-Багирми"), ("si", "චර\u{dd2}-බග\u{dd4}ඉර\u{dca}ම\u{dd2} කල\u{dcf}පය"), ("sv", "Chari-Baguirmi (region)"), ("ta", "ச\u{bbe}றி -பகுருமி பகுதி"), ("te", "చ\u{c3e}ర\u{c3f}-బ\u{c3e}గ\u{c4d}వ\u{c3f}ర\u{c4d}మ\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ชาร\u{e34} บาก\u{e38}ยอ\u{e4c}ม\u{e34}"), ("tr", "Chari-Baguirmi Bölgesi"), ("uk", "Регіон Шарі-Багірмі"), ("ur", "شاری-باگیرمی علاقہ"), ("vi", "Khu vực Chari-Baguirmi"), ("zh", "沙里-巴吉尔米区")]),
                        unofficial_name_list: ["Chari-Baguirmi"].to_vec(),
                    }
                ),
                (
                    "EE",
                    Subdivision{
                        name: "EE",
                        country_alpha2: Alpha2::TD,
                        code: "EE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إنيدي الشرقية"), ("ccp", "𑄃\u{11128}𑄚𑄬𑄓\u{11128}-𑄃\u{11128}𑄌\u{11134}𑄑\u{11134}"), ("en", "Ennedi-Est"), ("es", "Ennedi Est"), ("fr", "Ennedi Est"), ("it", "Regione di Ennedi Est"), ("ja", "東エネディ州"), ("ka", "აღმოსავლეთი ენედის რეგიონი"), ("ru", "Восточный Эннеди"), ("ur", "انیدی-مشرقی علاقہ")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "EO",
                    Subdivision{
                        name: "EO",
                        country_alpha2: Alpha2::TD,
                        code: "EO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إنيدي الغربية"), ("ccp", "𑄃\u{11128}𑄚𑄬𑄓\u{11128}-𑄃\u{1112e}𑄠𑄬𑄌\u{11134}𑄑\u{11134}"), ("en", "Ennedi-Ouest"), ("es", "Ennedi Ouest"), ("fr", "Ennedi Ouest"), ("it", "Regione di Ennedi Ovest"), ("ja", "西エネディ州"), ("ka", "დასავლეთი ენედის რეგიონი"), ("ru", "Западный Эннеди"), ("ur", "انیدی-مغربی علاقہ"), ("zh", "西恩內迪區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::TD,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.1219015), longitude: Some(18.4276047), max_latitude: Some(12.990278), min_latitude: Some(9.8482861), max_longitude: Some(20.192167), min_longitude: Some(17.2903469)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة قيرا"), ("bg", "Гера"), ("bn", "গোয\u{9bc}ের\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄉\u{1112a}𑄠𑄬𑄢"), ("ceb", "Guera Region"), ("cs", "Guéra"), ("da", "Guéra"), ("de", "Region Guéra"), ("el", "Γκουέρα"), ("en", "Guéra"), ("es", "Región de Guéra"), ("et", "Guéra"), ("eu", "Guéra"), ("fi", "Guéra"), ("fr", "Guéra"), ("gu", "ગ\u{ac1}એરા પ\u{acd}રદ\u{ac7}શ"), ("hi", "ग\u{941}एरा प\u{94d}रद\u{947}श"), ("id", "Wilayah Guéra"), ("it", "regione di Guéra"), ("ja", "ゲラ州"), ("ka", "გერის რეგიონი"), ("kn", "ಗುರ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "게라 주"), ("lt", "Geros regionas"), ("lv", "Geras reģions"), ("mr", "गिर\u{947} प\u{94d}रद\u{947}श"), ("ms", "Negeri Guéra"), ("nb", "Guéra"), ("nl", "Guéra"), ("no", "Guéra"), ("pl", "Region Guéra"), ("pt", "Região de Guera"), ("ro", "Regiunea Guéra"), ("ru", "Гера"), ("si", "ග\u{dd4}එර\u{dcf} පළ\u{dcf}ත"), ("sv", "Guéra (region)"), ("ta", "குயிர\u{bbe} பகுதி"), ("te", "గువ\u{c46}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตก\u{e38}เอรา"), ("tr", "Guéra Bölgesi"), ("uk", "Регіон Мбарара"), ("ur", "گویرا علاقہ"), ("vi", "Khu vực Guera"), ("zh", "蓋拉區")]),
                        unofficial_name_list: ["Guéra"].to_vec(),
                    }
                ),
                (
                    "HL",
                    Subdivision{
                        name: "HL",
                        country_alpha2: Alpha2::TD,
                        code: "HL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.4577273), longitude: Some(16.7234639), max_latitude: Some(13.3501059), min_latitude: Some(11.4313499), max_longitude: Some(17.6553859), min_longitude: Some(14.550512)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة حجر لميس"), ("bg", "Хаджер-Ламис"), ("bn", "হ\u{9be}জের ল\u{9be}মিস অঞ\u{9cd}চল"), ("ccp", "𑄦𑄖\u{11134}𑄎𑄬𑄢\u{11134}-𑄣𑄟\u{11128}𑄌\u{11134}"), ("ceb", "Hadjer-Lamis"), ("da", "Hadjer-Lamis"), ("de", "Region Hadjer-Lamis"), ("el", "Χάτζερ-Λάμις"), ("en", "Hadjer-Lamis"), ("es", "Región de Hadjer-Lamis"), ("et", "Hadjer-Lamis"), ("eu", "Hadjer-Lamis"), ("fi", "Hadjer-Lamis"), ("fr", "Hadjer-Lamis"), ("gl", "Hadjer-Lamis"), ("gu", "હ\u{ac5}જર-લામિસ પ\u{acd}રદ\u{ac7}શ"), ("hi", "हद\u{94d}जर-लामिस क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Hadjer-Lamis"), ("it", "regione di Hadjer-Lamis"), ("ka", "ჰაჯერ-ლამისის რეგიონი"), ("kn", "ಹಡ\u{ccd}ಜ\u{cc6}ರ\u{ccd}-ಲಾಮ\u{cbf}ಸ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "하제르라미 주"), ("lt", "Hadžero–Lamiso regionas"), ("lv", "Hadžēras-Lami reģions"), ("mr", "हदजर-लामीस प\u{94d}रा\u{902}त"), ("ms", "Hadjer-Lamis Region"), ("nb", "Hadjer-Lamis"), ("nl", "Hadjer-Lamis"), ("no", "Hadjer-Lamis"), ("pl", "Region Hadjer-Lamis"), ("pt", "Hadjer-Lamis"), ("ro", "Regiunea Hadjer-Lamis"), ("ru", "Хаджер-Ламис"), ("si", "හජෙර\u{dca}-ලම\u{dd2}ස\u{dca} කල\u{dcf}පය"), ("sv", "Hadjer-Lamis (region)"), ("ta", "ஹட\u{bcd}ஜர\u{bcd} -ல\u{bbe}மிஸ\u{bcd} பகுதி"), ("te", "హద\u{c4d}జర\u{c4d}-ల\u{c3e}మ\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เฮดเจอ ลาม\u{e34}ส"), ("tr", "Hadjer-Lamis Bölgesi"), ("uk", "Регіон Гаджер-Ламіс"), ("ur", "حجر-لامیس علاقہ"), ("vi", "Khu vực Hadjer-Lamis"), ("zh", "哈傑爾-拉密區")]),
                        unofficial_name_list: ["Hadjer Lamis"].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "KA",
                        country_alpha2: Alpha2::TD,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.8781262), longitude: Some(15.4068079), max_latitude: Some(16.8978711), min_latitude: Some(13.117372), max_longitude: Some(16.7248479), min_longitude: Some(13.633162)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة كانم²"), ("bg", "Канем"), ("bn", "ক\u{9be}নেম অঞ\u{9cd}চল"), ("ca", "Regió de Kanem"), ("ccp", "𑄇𑄚𑄬𑄟\u{11134}"), ("ceb", "Kanem Region"), ("da", "Kanem"), ("de", "Region Kanem"), ("el", "Κανέμ"), ("en", "Kanem"), ("es", "Región de Kanem"), ("et", "Kanem"), ("eu", "Kanem"), ("fi", "Kanem"), ("fr", "Kanem"), ("gu", "કાન\u{ac7}મ પ\u{acd}રદ\u{ac7}શ"), ("hi", "कान\u{947}म प\u{94d}रद\u{947}श"), ("id", "Region Kanem"), ("it", "regione di Kanem"), ("ja", "カネム州"), ("ka", "კანემის რეგიონი"), ("kn", "ಕಾನ\u{cc6}ಮ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "카넴 주"), ("lt", "Kanemo regionas"), ("lv", "Kanemas reģions"), ("mr", "कान\u{947}म प\u{94d}रद\u{947}श"), ("ms", "Kanem Region"), ("nb", "Kanem"), ("nl", "Kanem"), ("no", "Kanem"), ("pl", "Region Kanem"), ("pt", "Kanem"), ("ro", "Regiunea Kanem"), ("ru", "Канем"), ("si", "කනෙම\u{dca} පළ\u{dcf}ත"), ("sv", "Kanem"), ("ta", "க\u{bbe}ணேம\u{bcd} பகுதி"), ("te", "క\u{c3e}న\u{c46}మ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "คาเนม"), ("tr", "Kanem Bölgesi"), ("uk", "Регіон Канем"), ("ur", "کانیم علاقہ"), ("vi", "Khu vực Kanem"), ("zh", "加奈姆區")]),
                        unofficial_name_list: ["Kanem"].to_vec(),
                    }
                ),
                (
                    "LC",
                    Subdivision{
                        name: "LC",
                        country_alpha2: Alpha2::TD,
                        code: "LC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.6915377), longitude: Some(14.1001326), max_latitude: Some(14.5104809), min_latitude: Some(12.9456311), max_longitude: Some(15.5492499), min_longitude: Some(13.4734749)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة البحيرة"), ("bg", "Лак"), ("bn", "ল\u{9be}ক অঞ\u{9cd}চল"), ("ccp", "𑄣𑄬𑄇\u{11134}"), ("ceb", "Lac Region"), ("da", "Lac"), ("de", "Region Lac"), ("el", "Λακ"), ("en", "Lac"), ("es", "Región de Lac"), ("et", "Lac"), ("eu", "Lac"), ("fi", "Lac"), ("fr", "Lac"), ("gu", "લાખ પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{948}क क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Lac"), ("it", "regione del Lago"), ("ja", "ラク州"), ("ka", "ლაკის რეგიონი"), ("kn", "ಲ\u{ccd}ಯಾಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "라크 주"), ("lt", "Ežero regionas"), ("lv", "Lakas reģions"), ("mr", "ल\u{945}क प\u{94d}रद\u{947}श"), ("ms", "Lac Region"), ("nb", "Lac"), ("nl", "Lac"), ("no", "Lac"), ("pl", "Region Lac"), ("pt", "Região Lac"), ("ro", "Regiunea Lac"), ("ru", "Лак"), ("si", "ලැක\u{dca} කල\u{dcf}පය"), ("sv", "Lac (region)"), ("ta", "லேக\u{bcd} பகுதி"), ("te", "ల\u{c3e}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตแรค"), ("tr", "Lac Bölgesi"), ("uk", "Регіон Лак"), ("ur", "لاک ریجن"), ("vi", "Khu vực Lac"), ("zh", "湖區")]),
                        unofficial_name_list: ["Lac"].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::TD,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.759675999999999), longitude: Some(15.876004), max_latitude: Some(9.208981999999999), min_latitude: Some(8.244755), max_longitude: Some(16.5759211), min_longitude: Some(15.2746969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة لوقون الغربي"), ("bg", "Западен Логон (регион)"), ("bn", "ল\u{9c1}\u{9c1}গোনে অক\u{9cd}সিডেন\u{9cd}ট\u{9be}ল অঞ\u{9cd}চল"), ("ccp", "𑄣\u{11127}𑄉\u{1112e}𑄚\u{11134} 𑄃\u{11127}𑄇\u{11134}𑄥\u{11128}𑄓𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("ceb", "Logone Occidental Region"), ("da", "Logone Occidental"), ("de", "Region Logone Occidental"), ("el", "Λογκόνε Οξιντένταλ"), ("en", "Logone Occidental"), ("es", "Región de Logone Occidental"), ("et", "Lääne-Logone"), ("eu", "Mendebaldeko Logone"), ("fi", "Logone Occidental"), ("fr", "Logone Occidental"), ("gu", "લોગોન ઓક\u{acd}સિડ\u{ac7}ન\u{acd}ટલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "लोगोन ऑक\u{94d}सीडो\u{902}टल प\u{94d}रद\u{947}श"), ("id", "Region Logone Occidental"), ("it", "regione del Logone Occidentale"), ("ja", "ロゴン・オクシデンタル州"), ("ka", "დასავლეთი ლოგონის რეგიონი"), ("kn", "ಲೊಗೊನ\u{ccd} ಆಕೇಶನಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "로곤옥시당탈 주"), ("lt", "Vakarų Logonės regionas"), ("lv", "Rietumlogones reģions"), ("mr", "लॉगऑन ओक\u{945}सिड\u{947}\u{902}टल प\u{94d}रद\u{947}श"), ("ms", "Logone Occidental Region"), ("nb", "Logone Occidental"), ("nl", "Logone Occidental"), ("no", "Logone Occidental"), ("pl", "Region Logon Zachodni"), ("pt", "Região Ocidental de Logone"), ("ro", "Regiunea Logone Occidental"), ("ru", "Западный Логон"), ("si", "ලොගොනේ ඔක\u{dca}ස\u{dd2}ඩෙන\u{dca}ටල\u{dca} කල\u{dcf}පය"), ("sv", "Logone Occidental"), ("te", "ల\u{c4b}గ\u{c4b}న\u{c4d} ఆక\u{c4d}స\u{c3f}డ\u{c46}ంటల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โลโกเน ออคซ\u{e34}เดนทอล"), ("tr", "Logone Occidental Bölgesi"), ("uk", "Західний Логон"), ("ur", "لوگونے مغربی علاقہ"), ("vi", "Khu vực Logone Occidental"), ("zh", "西洛貢區")]),
                        unofficial_name_list: ["Logone-Occidental"].to_vec(),
                    }
                ),
                (
                    "LR",
                    Subdivision{
                        name: "LR",
                        country_alpha2: Alpha2::TD,
                        code: "LR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.3149949), longitude: Some(16.3463791), max_latitude: Some(9.115038), min_latitude: Some(7.4410681), max_longitude: Some(17.2695311), min_longitude: Some(15.230739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة لوقون الشرقي"), ("bg", "Източен Логон"), ("bn", "ল\u{9c1}গোনে ওরিয\u{9bc}েন\u{9cd}ট\u{9be}ল অঞ\u{9cd}চল"), ("ccp", "𑄣\u{11127}𑄉\u{1112e}𑄚\u{11134} 𑄃\u{11127}𑄢\u{11128}𑄠𑄬𑄚\u{11134}𑄑𑄣\u{11134}"), ("da", "Logone Oriental"), ("de", "Region Logone Oriental"), ("el", "Λογκόνε Οριένταλ"), ("en", "Logone Oriental"), ("es", "Región de Logone Oriental"), ("et", "Ida-Logone"), ("eu", "Ekialdeko Logone"), ("fi", "Logone Oriental"), ("fr", "Logone Oriental"), ("gu", "લોગોન ઓરિએન\u{acd}ટલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "लॉगोन ओरिए\u{902}टल क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Logone Oriental"), ("it", "regione del Logone Orientale"), ("ja", "ロゴン・オリエンタル州"), ("ka", "აღმოსავლეთი ლოგონის რეგიონი"), ("kn", "ಲೊಗೊನ\u{ccd} ಓರ\u{cbf}ಯಂಟಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "로곤오리앙탈 주"), ("lt", "Rytų Logonės regionas"), ("lv", "Austrumlogones reģions"), ("mr", "लॉगोन ओरिएन\u{94d}टल प\u{94d}रद\u{947}श"), ("ms", "Logone Oriental Region"), ("nb", "Logone Oriental"), ("nl", "Logone Oriental"), ("no", "Logone Oriental"), ("pl", "Region Logone Oriental"), ("pt", "Região de Logone Oriental"), ("ro", "Regiunea Logone Oriental"), ("ru", "Восточный Логон"), ("si", "ලොගොනේ ඔර\u{dd2}යන\u{dca}ටල\u{dca} කල\u{dcf}පය"), ("sv", "Logone Oriental"), ("ta", "லோகோனே ஒரிஎண\u{bcd}டல\u{bcd} பகுதி"), ("te", "ల\u{c4b}గ\u{c4b}న\u{c4d} ఓర\u{c3f}యంటల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "โลโกน โอเร\u{e35}ยลทอล"), ("tr", "Logone Oriental Bölgesi"), ("uk", "Східний Логон (префектура)"), ("ur", "لوگونے مشرقی علاقہ"), ("vi", "Khu vực Logone Oriental"), ("zh", "東洛貢區")]),
                        unofficial_name_list: ["Logone-Oriental"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::TD,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.603091), longitude: Some(17.4795173), max_latitude: Some(9.598262), min_latitude: Some(7.785258), max_longitude: Some(18.164068), min_longitude: Some(17.0344641)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة ماندول"), ("bg", "Мандул"), ("bn", "ম\u{9be}ন\u{9cd}ডোল অঞ\u{9cd}চল"), ("ccp", "𑄟𑄚\u{11134}𑄓\u{1112f}𑄣\u{11134}"), ("ceb", "Mandoul"), ("da", "Mandoul"), ("de", "Region Mandoul"), ("el", "Μαντούλ"), ("en", "Mandoul"), ("es", "Región de Mandoul"), ("et", "Mandoul"), ("eu", "Mandoul"), ("fi", "Mandoul"), ("fr", "Mandoul"), ("gu", "મ\u{ac7}\u{a82}ડોલ પ\u{acd}રદ\u{ac7}શ"), ("hi", "म\u{902}डोल क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Mandoul"), ("it", "regione di Mandoul"), ("ja", "マンドゥル州"), ("ka", "მანდულის რეგიონი"), ("kn", "ಮ\u{ccd}ಯಾಂಡ\u{ccd}ಯುಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "망둘 주"), ("lt", "Mandulo regionas"), ("lv", "Mandūlas reģions"), ("mr", "म\u{945}न\u{94d}डॉल प\u{94d}रद\u{947}श"), ("ms", "Mandoul Region"), ("nb", "Mandoul"), ("nl", "Mandoul"), ("no", "Mandoul"), ("pl", "Region Mandoul"), ("pt", "Mandoul"), ("ro", "Regiunea Mandoul"), ("ru", "Мандуль"), ("si", "මන\u{dca}ඩොල\u{dca} කල\u{dcf}පය"), ("sv", "Mandoul"), ("ta", "மண\u{bcd}டோயுள\u{bcd} பகுதி"), ("te", "మ\u{c3e}ండ\u{c4b}ల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แมนด\u{e39}ลว\u{e4c}"), ("tr", "Mandoul Bölgesi"), ("uk", "Регіон Мандуль"), ("ur", "ماندؤل علاقہ"), ("vi", "Khu vực Mandoul"), ("zh", "芒杜爾區")]),
                        unofficial_name_list: ["Mandoul"].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::TD,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.0639998), longitude: Some(18.4276047), max_latitude: Some(10.5414589), min_latitude: Some(8.0147168), max_longitude: Some(19.927143), min_longitude: Some(17.3586501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة شاري الأوسط"), ("bg", "Моаян-Шари"), ("bn", "মোয\u{9bc}েন চ\u{9be}রি অঞ\u{9cd}চল"), ("ccp", "𑄟\u{11127}𑄠𑄬𑄚\u{11134}-𑄌𑄢\u{11128}"), ("ceb", "Moyen-Chari Region"), ("da", "Moyen-Chari"), ("de", "Region Moyen-Chari"), ("el", "Μογιέν-Τσαρί"), ("en", "Moyen-Chari"), ("es", "Región de Moyen-Chari"), ("et", "Kesk-Chari"), ("eu", "Moyen-Chari"), ("fi", "Moyen-Chari"), ("fr", "Moyen-Chari"), ("gu", "મોય\u{ac7}ન-ચારી પ\u{acd}રદ\u{ac7}શ"), ("hi", "मोय\u{947}न-चारी क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Moyen-Chari"), ("it", "regione di Moyen-Chari"), ("ja", "モワイヤン・シャリ州"), ("ka", "შუა შარის რეგიონი"), ("kn", "ಮೊಯ\u{cc6}ನ\u{ccd}-ಚರ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "무아얭샤리 주"), ("lt", "Vidurio Šario regionas"), ("lv", "Vidusšari reģions"), ("mr", "मोय\u{947}न-चारी प\u{94d}रद\u{947}श"), ("ms", "Moyen-Chari Region"), ("nb", "Moyen-Chari"), ("nl", "Moyen-Chari"), ("no", "Moyen-Chari"), ("pl", "Region Szari Środkowe"), ("pt", "Moyen-Chari"), ("ro", "Regiunea Moyen-Chari"), ("ru", "Среднее Шари"), ("si", "මොයෙන\u{dca}-චර\u{dd2} කල\u{dcf}පය"), ("sv", "Moyen-Chari (region)"), ("ta", "மோயின\u{bcd}-ச\u{bbe}றி பகுதி"), ("te", "మ\u{c4b}య\u{c46}న\u{c4d}-చ\u{c3e}ర\u{c40} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เม\u{e37}องบ\u{e39}ลาค\u{e31}น"), ("tr", "Moyen-Chari Bölgesi"), ("uk", "Середнє Шарі"), ("ur", "موین-شاری علاقہ"), ("vi", "Khu vực Moyen-Chari"), ("zh", "中沙里區")]),
                        unofficial_name_list: ["Moyen-Chari"].to_vec(),
                    }
                ),
                (
                    "ME",
                    Subdivision{
                        name: "ME",
                        country_alpha2: Alpha2::TD,
                        code: "ME",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.4046039), longitude: Some(14.8454619), max_latitude: Some(10.0102089), min_latitude: Some(8.5243641), max_longitude: Some(15.44772), min_longitude: Some(14.0194639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة مايو كيبي الشرقي"), ("bg", "Източен Майо-Кеби"), ("bn", "ম\u{9be}য\u{9bc}ো কেবি ইস\u{9cd}ট অঞ\u{9cd}চল"), ("ccp", "𑄟\u{11127}𑄠-𑄇𑄬𑄝\u{11133}𑄦\u{11128} 𑄃\u{11128}𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Mayo-Kebbi East Region"), ("da", "Mayo-Kebbi Est"), ("de", "Region Mayo-Kebbi Est"), ("el", "Μάγιο-Κέμπι Εστ"), ("en", "Mayo-Kebbi Est"), ("es", "Región de Mayo-Kebbi Este"), ("et", "Ida-Mayo-Kébbi"), ("eu", "Ekialdeko Mayo-Kebbi"), ("fi", "Mayo-Kebbi Est"), ("fr", "Mayo-Kebbi Est"), ("gu", "મ\u{ac7}યો-ક\u{ac7}બી પ\u{ac2}ર\u{acd}વ પ\u{acd}રદ\u{ac7}શ"), ("hi", "म\u{947}यो-क\u{947}बी एस\u{94d}ट प\u{94d}रद\u{947}श"), ("id", "Region Mayo-Kebbi Est"), ("it", "regione di Mayo-Kebbi Est"), ("ja", "東マヨ・ケッビ州"), ("ka", "აღმოსავლეთი მაიო-კების რეგიონი"), ("kn", "ಮಾಯೋ-ಕ\u{cc6}ಬ\u{ccd}ಬ\u{cbf} ಎಸ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "마요케비에스트 주"), ("lt", "Rytų Majo Kebio regionas"), ("lv", "Austrumu Majo Kebi reģions"), ("mr", "म\u{947}यो-क\u{947}ब\u{94d}बी इस\u{94d}ट प\u{94d}रद\u{947}श"), ("ms", "Mayo-Kebbi Est Region"), ("nb", "Mayo-Kebbi Est"), ("nl", "Mayo-Kebbi Est"), ("no", "Mayo-Kebbi Est"), ("pl", "Region Mayo-Kebbi Est"), ("pt", "Mayo-Kebbi Est"), ("ro", "Regiunea Mayo-Kebbi Est"), ("ru", "Восточное Майо-Кеби"), ("si", "ම\u{dcf}යෝ-කෙබ\u{dca}බ\u{dd2} එස\u{dca}ට\u{dca} කල\u{dcf}පය"), ("sv", "Mayo-Kebbi Est"), ("ta", "ம\u{bbe}யோ -கெப\u{bcd}பி கிழக\u{bcd}கு பகுதி"), ("te", "మ\u{c3e}య\u{c4b}-క\u{c46}బ\u{c4d}బ\u{c40} ఈస\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มาโย-เคปบ\u{e35} เอส"), ("tr", "Mayo-Kebbi Est Bölgesi"), ("uk", "Регіон Східний Майо-Кебі"), ("ur", "مایو-کیبی مشرقی علاقہ"), ("vi", "Khu vực Mayo-Kebbi Est"), ("zh", "東凱比河區")]),
                        unofficial_name_list: ["Mayo-Kébbi-Est"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::TD,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.4113014), longitude: Some(15.5943388), max_latitude: Some(11.181067), min_latitude: Some(9.188032), max_longitude: Some(16.2150691), min_longitude: Some(14.949498)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة مايو كيبي الغربي"), ("bg", "Западен Майо-Кеби"), ("bn", "ম\u{9be}য\u{9bc}ো কেব\u{9cd}বি ওয\u{9bc}েস\u{9cd}ট অঞ\u{9cd}চল"), ("ccp", "𑄟\u{11127}𑄠-𑄇𑄬𑄝\u{11133}𑄦\u{11128} 𑄃\u{1112e}𑄠𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Mayo-Kebbi West Region"), ("da", "Mayo-Kebbi Ouest"), ("de", "Region Mayo-Kebbi Ouest"), ("el", "Μάγιο-Κέμπι Ουέστ"), ("en", "Mayo-Kebbi Ouest"), ("es", "Región de Mayo-Kebbi Oeste"), ("et", "Lääne-Mayo-Kébbi"), ("eu", "Mendebaldeko Mayo-Kebbi"), ("fi", "Mayo-Kebbi Ouest"), ("fr", "Mayo-Kebbi Ouest"), ("gu", "મ\u{ac7}યો-ક\u{ac7}બી ઔએસ\u{acd}ટ પ\u{acd}રદ\u{ac7}શ"), ("hi", "म\u{947}यो-क\u{947}बी आउस\u{94d}ट क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Mayo-Kebbi Ouest"), ("it", "regione di Mayo-Kebbi Ovest"), ("ja", "西マヨ・ケッビ州"), ("ka", "დასავლეთი მაიო-კების რეგიონი"), ("kn", "ಮೇಯೊ-ಕ\u{cc6}ಬ\u{ccd}ಬ\u{cbf} ಔಯ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "마요케비우에스트 주"), ("lt", "Vakarų Majo Kebio regionas"), ("lv", "Rietumu Majo Kebi reģions"), ("mr", "म\u{947}यो-क\u{947}बबी आउस\u{94d}ट प\u{94d}रद\u{947}श"), ("ms", "Mayo-Kebbi Ouest Region"), ("nb", "Mayo-Kebbi Ouest"), ("nl", "Mayo-Kebbi Ouest"), ("no", "Mayo-Kebbi Ouest"), ("pl", "Region Mayo-Kebbi Ouest"), ("pt", "Mayo-Kebbi Ouest"), ("ro", "Regiunea Mayo-Kebbi Ouest"), ("ru", "Западное Майо-Кеби"), ("si", "මයෝ-කෙබ\u{dca}බ\u{dd2} ඔඑස\u{dca}ට\u{dca} කල\u{dcf}පය"), ("sv", "Mayo-Kebbi Ouest"), ("ta", "ம\u{bbe}யோ -கேப\u{bcd}பி ஆயுஸ\u{bcd}ட\u{bcd} பகுதி"), ("te", "మ\u{c3e}య\u{c4b}-క\u{c46}బ\u{c4d}బ\u{c3f} ఓయ\u{c46}స\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "มาโย เคบบ\u{e34} เอส"), ("tr", "Mayo-Kebbi Ouest Bölgesi"), ("uk", "Регіон Західний Майо-Кебі"), ("ur", "مایو-کیبی مغربی علاقہ"), ("vi", "Khu vực Mayo-Kebbi Ouest"), ("zh", "西凱比河區")]),
                        unofficial_name_list: ["Mayo-Kébbi-Ouest"].to_vec(),
                    }
                ),
                (
                    "ND",
                    Subdivision{
                        name: "ND",
                        country_alpha2: Alpha2::TD,
                        code: "ND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.113056), longitude: Some(15.049167), max_latitude: Some(12.1801019), min_latitude: Some(12.0603346), max_longitude: Some(15.1484305), min_longitude: Some(14.9664874)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "N’Djamena"), ("am", "ንጃመና"), ("ar", "انجمينا"), ("az", "Ncamena"), ("be", "Горад Нджамена"), ("bg", "Нджамена"), ("bn", "ন’জ\u{9be}মিন\u{9be}"), ("bs", "N’Djamena"), ("ca", "N’Djamena"), ("ccp", "𑄚‘ 𑄎𑄟𑄬𑄚"), ("ceb", "N’Djamena"), ("cs", "N’Djamena"), ("cy", "N’Djamena"), ("da", "N’Djamena"), ("de", "N’Djamena"), ("el", "Ντζαμένα"), ("en", "N’Djamena"), ("es", "Yamena"), ("et", "N’Djamena"), ("eu", "N’Djamena"), ("fa", "انجامنا"), ("fi", "N’Djamena"), ("fr", "N’Djamena"), ("ga", "N’Djamena"), ("gl", "Xamena"), ("gu", "એન\u{acd}ડજામ\u{ac7}ના"), ("ha", "Ndjamena"), ("ha_NE", "Ndjamena"), ("he", "נג׳מנה"), ("hi", "एन ज\u{947}मीना"), ("hr", "N’Djamena"), ("hu", "N’Djamena"), ("hy", "Նջամենա"), ("id", "N’Djamena"), ("is", "N’Djamena"), ("it", "N’Djamena"), ("ja", "ンジャメナ"), ("jv", "N’Djamena"), ("ka", "ნჯამენა"), ("kk", "Нджамена"), ("kn", "ಎ\u{200d}ನ\u{ccd}\u{200c}ಝಮೀನಾ"), ("ko", "은자메나"), ("ky", "Нжамена"), ("lt", "Ndžamena"), ("lv", "Ndžamena"), ("mk", "Нџамена"), ("ml", "ൻഡ\u{d4d}ജ\u{d3e}മെന"), ("mn", "Нжамена"), ("mr", "इ\u{902}जामिना"), ("ms", "N’Djamena"), ("nb", "N’Djamena"), ("nl", "Ndjamena"), ("no", "N’Djamena"), ("pa", "ਨਿਜਾਮੀਨਾ"), ("pl", "Ndżamena"), ("pt", "N’Djamena"), ("ro", "N’Djamena"), ("ru", "Нджамена"), ("si", "න\u{dd2}ව\u{dca}මෙන\u{dcf}"), ("sk", "N’Djamena"), ("sl", "N’Djamena"), ("so", "Nijamiina"), ("sq", "Nxhamena"), ("sr", "Нџамена"), ("sr_Latn", "Ndžamena"), ("sv", "N’Djamena"), ("sw", "N’Djamena"), ("ta", "நிஜ\u{bbe}ம\u{bc0}ன\u{bbe}"), ("te", "ఎన\u{c4d} జమ\u{c40}న\u{c3e}"), ("th", "อ\u{e36}นจาเมนา"), ("tk", "Njamena"), ("tr", "N’Djamena"), ("uk", "Нджамена"), ("ur", "اینجامینا"), ("uz", "Njamena"), ("vi", "N’Djamena"), ("yo", "N’Djamena"), ("yo_BJ", "N’Djamena"), ("yue", "恩賈梅納"), ("yue_Hans", "恩贾梅纳"), ("zh", "恩賈梅納")]),
                        unofficial_name_list: ["Ndjamena"].to_vec(),
                    }
                ),
                (
                    "OD",
                    Subdivision{
                        name: "OD",
                        country_alpha2: Alpha2::TD,
                        code: "OD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.748476), longitude: Some(20.7122465), max_latitude: Some(14.3643509), min_latitude: Some(12.636083), max_longitude: Some(22.284214), min_longitude: Some(20.0267009)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة وداي"), ("bg", "Уадаи"), ("bn", "উয\u{9bc}\u{9be}ড\u{9be}য\u{9bc} অঞ\u{9cd}চল"), ("ca", "Regió d’Ouaddaï"), ("ccp", "𑄃\u{1112f}𑄠𑄓\u{11133}𑄦\u{1112d}"), ("ceb", "Ouaddai Region"), ("da", "Ouaddaï"), ("de", "Region Wadai"), ("el", "Ουαντάι"), ("en", "Ouaddaï"), ("es", "Región de Ouaddaï"), ("et", "Ouaddaï"), ("eu", "Ouaddaï"), ("fi", "Ouaddaï"), ("fr", "Ouaddaï"), ("gu", "ઔડાઇ પ\u{acd}રદ\u{ac7}શ"), ("hi", "उडाई क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Ouaddaï"), ("it", "regione di Ouaddaï"), ("ja", "ワダイ州"), ("ka", "უადაის რეგიონი"), ("kn", "ಒವಾಡ\u{ccd}ಡೈ ಪ\u{ccd}ರದೇಶ"), ("ko", "와다이 주"), ("lt", "Vadajaus regionas"), ("lv", "Vadaji reģions"), ("mr", "उडडाई प\u{94d}रद\u{947}श"), ("ms", "Ouaddai Region"), ("nb", "Ouaddaï"), ("nl", "Ouaddaï"), ("no", "Ouaddaï"), ("pl", "Region Wadaj"), ("pt", "Ouaddaï"), ("ro", "Regiunea Ouaddaï"), ("ru", "Ваддай"), ("si", "ඔඋදද\u{dcf}ය\u{dd2} කල\u{dcf}පය"), ("sv", "Ouaddaï (region)"), ("ta", "ஓயடை பகுதி"), ("te", "ఓవడ\u{c3e}య\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ค\u{e31}วได"), ("tr", "Ouaddaï Bölgesi"), ("uk", "Ваддай"), ("ur", "اوادای علاقہ"), ("vi", "Khu vực Ouaddai"), ("zh", "瓦達伊區")]),
                        unofficial_name_list: ["Ouaddaï"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::TD,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.9691601), longitude: Some(20.7122465), max_latitude: Some(12.088373), min_latitude: Some(9.060796), max_longitude: Some(22.362142), min_longitude: Some(18.8384811)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة سلامات"), ("bg", "Саламат"), ("bn", "স\u{9be}ল\u{9be}ম\u{9be}ত অঞ\u{9cd}চল"), ("ccp", "𑄥𑄣𑄟\u{11127}𑄖\u{11134}"), ("ceb", "Salamat Region"), ("cs", "Salamat"), ("da", "Salamat"), ("de", "Region Salamat"), ("el", "Σαλαμάτ"), ("en", "Salamat"), ("es", "Región de Salamat"), ("et", "Salamat"), ("eu", "Salamat"), ("fi", "Salamat"), ("fr", "Salamat"), ("gu", "સલમાટ પ\u{acd}રદ\u{ac7}શ"), ("hi", "सलामत प\u{94d}रद\u{947}श"), ("id", "Region Salamat"), ("it", "regione del Salamat"), ("ja", "サラマト州"), ("ka", "სალამატის რეგიონი"), ("kn", "ಸಲಾಮಾಟ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "살라마트 주"), ("lt", "Salamato regionas"), ("lv", "Salamatas reģions"), ("mr", "सलमाट प\u{94d}रद\u{947}श"), ("ms", "Salamat Region"), ("nb", "Salamat"), ("nl", "Salamat"), ("no", "Salamat"), ("pl", "Region Salamat"), ("pt", "Região de Salamat"), ("ro", "Regiunea Salamat"), ("ru", "Саламат"), ("si", "සලමට\u{dca} පළ\u{dcf}ත"), ("sv", "Salamat (region)"), ("ta", "சல\u{bbe}மைட\u{bcd} பகுதி"), ("te", "సల\u{c3e}మత\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซาลาม\u{e31}ท"), ("tr", "Salamat Bölgesi"), ("uk", "Регіон Саламат"), ("ur", "سلامات علاقہ"), ("vi", "Khu vực Salamat"), ("zh", "萨拉马特区")]),
                        unofficial_name_list: ["Salamat"].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::TD,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة سيلا"), ("bg", "Сила"), ("bn", "সিল\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄥\u{11128}𑄣"), ("da", "Sila"), ("de", "Region Sila"), ("el", "Σίλα"), ("en", "Sila"), ("es", "Región de Sila"), ("eu", "Sila"), ("fi", "Sila"), ("fr", "Sila"), ("gu", "સિલા પ\u{acd}રદ\u{ac7}શ"), ("hi", "सिला क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Sila"), ("it", "regione del Sila"), ("ja", "シラ州"), ("ka", "სილის რეგიონი"), ("kn", "ಸ\u{cbf}ಲಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "실라 주"), ("lt", "Silos regionas"), ("lv", "Silas reģions"), ("mr", "शिला प\u{94d}रद\u{947}श"), ("ms", "Sila Region"), ("nb", "Sila region"), ("nl", "Sila Region"), ("no", "Sila region"), ("pl", "Region Sila"), ("pt", "Região Sila"), ("ro", "Regiunea Sila"), ("ru", "Сила"), ("si", "ස\u{dd2}ල\u{dcf} කල\u{dcf}පය"), ("sv", "Sila (region)"), ("ta", "சில\u{bbe} பகுதி"), ("te", "స\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ซ\u{e34}ลา"), ("tr", "Sila Bölgesi"), ("uk", "Регіон Сіла"), ("ur", "سیلا علاقہ"), ("vi", "Khu vực Sila"), ("zh", "西拉區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::TD,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.6625729), longitude: Some(16.7234639), max_latitude: Some(10.222793), min_latitude: Some(9.006821), max_longitude: Some(17.552968), min_longitude: Some(15.438418)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة تانجلي"), ("bg", "Танджиле"), ("bn", "ত\u{9be}ঞ\u{9cd}জিলে অঞ\u{9cd}চল"), ("ccp", "𑄑𑄚\u{11134}𑄎\u{1112d}𑄣\u{11134}"), ("da", "Tandjilé"), ("de", "Region Tandjilé"), ("el", "Ταντζιλέ"), ("en", "Tandjilé"), ("es", "Región de Tandjilé"), ("et", "Tandjilé"), ("eu", "Tandjilé"), ("fi", "Tandjilé"), ("fr", "Tandjilé"), ("gu", "ટ\u{a82}ડજિલ\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("hi", "त\u{902}दील\u{947} क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Tandjilé"), ("it", "regione di Tandjilé"), ("ja", "タンジレ州"), ("ka", "ტანჯილეს რეგიონი"), ("kn", "ಟಾಂಡ\u{ccd}ಜ\u{cbf}ಲ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "탕질레 주"), ("lt", "Tandžilės regionas"), ("lv", "Tandžiles reģions"), ("mr", "ता\u{902}डजील प\u{94d}रद\u{947}श"), ("ms", "Tandjile Region"), ("nb", "Tandjilé"), ("nl", "Tandjilé"), ("no", "Tandjilé"), ("pl", "Region Tandjilé"), ("pt", "Tandjilé"), ("ro", "Regiunea Tandjilé"), ("ru", "Танджиле"), ("si", "ටන\u{dca}ඩ\u{dca}ජ\u{dd2}ලේ කල\u{dcf}පය"), ("sv", "Tandjilé (region)"), ("ta", "டண\u{bcd}ட\u{bcd}ஜிலே பகுதி"), ("te", "ట\u{c3e}ండ\u{c4d}జ\u{c3f}ల\u{c47} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แทนจ\u{e34}ลเล\u{e48}"), ("tr", "Tandjilé Bölgesi"), ("uk", "Регіон Танджиле"), ("ur", "تانجیلے علاقہ"), ("vi", "Khu vực Tandjilé"), ("zh", "坦吉萊區")]),
                        unofficial_name_list: ["Tandjilé"].to_vec(),
                    }
                ),
                (
                    "TI",
                    Subdivision{
                        name: "TI",
                        country_alpha2: Alpha2::TD,
                        code: "TI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة تبستي"), ("bg", "Тибести"), ("bn", "তিবেস\u{9cd}টি অঞ\u{9cd}চল"), ("ccp", "𑄑\u{11128}𑄝𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Tibesti Region"), ("da", "Tibesti"), ("de", "Region Tibesti"), ("el", "Τιμπέστι"), ("en", "Tibesti"), ("es", "Región de Tibesti"), ("eu", "Tibesti"), ("fi", "Tibesti"), ("fr", "Tibesti"), ("gu", "તિબ\u{ac7}સ\u{acd}ટી પ\u{acd}રદ\u{ac7}શ"), ("hi", "तिब\u{947}स\u{94d}टी क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Tibesti"), ("it", "regione di Tibesti"), ("ja", "ティベスティ州"), ("ka", "ტიბესტის რეგიონი"), ("kn", "ಟ\u{cbf}ಬ\u{cc6}ಸ\u{ccd}ತ\u{cbf} ಪ\u{ccd}ರದೇಶ"), ("ko", "티베스티 주"), ("lt", "Tibesčio regionas"), ("lv", "Tībestī reģions"), ("mr", "तिब\u{947}स\u{94d}टी प\u{94d}रद\u{947}श"), ("ms", "Tibesti Region"), ("nb", "Tibesti Region"), ("nl", "Tibesti Region"), ("no", "Tibesti Region"), ("pl", "Region Tibesti"), ("pt", "Região de Tibesti"), ("ro", "Regiunea Tibesti"), ("ru", "Тибести"), ("si", "ඉට\u{dca}බෙස\u{dca}ට\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Tibesti (region)"), ("ta", "டிபேஸ\u{bcd}டி பகுதி"), ("te", "ట\u{c3f}బ\u{c46}స\u{c4d}ట\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตท\u{e34}เบสต\u{e35}"), ("tr", "Tibesti Bölgesi"), ("uk", "Регіон Тібесті"), ("ur", "تیبستی علاقہ"), ("vi", "Khu vựcTibesti"), ("zh", "提貝斯提區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WF",
                    Subdivision{
                        name: "WF",
                        country_alpha2: Alpha2::TD,
                        code: "WF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.0892416), longitude: Some(21.4752851), max_latitude: Some(15.9035701), min_latitude: Some(13.85626), max_longitude: Some(23.0038449), min_longitude: Some(20.035872)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة وادي فيرا"), ("bg", "Уади Фира"), ("bn", "ওয\u{9bc}\u{9be}ডি ফির\u{9be} অঞ\u{9cd}চল"), ("ccp", "𑄤𑄓\u{11128} 𑄜\u{11128}𑄢"), ("da", "Wadi Fira"), ("de", "Region Wadi Fira"), ("el", "Γουάντι Φίρα"), ("en", "Wadi Fira"), ("es", "Región de Wadi Fira"), ("et", "Wadi Fira"), ("eu", "Wadi Fira"), ("fi", "Wadi Fira"), ("fr", "Wadi Fira"), ("gu", "વાડી ફિરા પ\u{acd}રદ\u{ac7}શ"), ("hi", "वादी फिरा क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Region Wadi Fira"), ("it", "regione di Wadi Fira"), ("ja", "ワジ・フィラ州"), ("ka", "უადი-ფირის რეგიონი"), ("kn", "ವಾಡ\u{cbf} ಫ\u{cbf}ರಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "와디피라 주"), ("lt", "Vadi Firos regionas"), ("lv", "Vādī Firas reģions"), ("mr", "वाडी फिरा प\u{94d}रद\u{947}श"), ("ms", "Wadi Fira Region"), ("nb", "Wadi Fira"), ("nl", "Wadi Fira"), ("no", "Wadi Fira"), ("pl", "Region Wadi Fira"), ("pt", "Wadi Fira"), ("ro", "Regiunea Wadi Fira"), ("ru", "Вади-Фера"), ("si", "වඩ\u{dd2} ෆ\u{dd2}ර\u{dcf} කල\u{dcf}පය"), ("sv", "Wadi Fira (region)"), ("ta", "வ\u{bbe}டி பிற\u{bbe} பகுதி"), ("te", "వ\u{c3e}డ\u{c3f} ఫ\u{c3f}ర\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "วาด\u{e35}ฟ\u{e35}รา"), ("tr", "Wadi Fira Bölgesi"), ("uk", "Ваді-Фіра"), ("ur", "وادی فیرا علاقہ"), ("vi", "Khu vực Wadi Fira"), ("zh", "瓦迪菲拉區")]),
                        unofficial_name_list: ["Wadi Fira"].to_vec(),
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
#[cfg(feature = "td")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::TD,
        alpha3: Alpha3::TCD,
        address_format: None,
        continent: Continent::Africa,
        country_code: 235,
        currency_code: CurrencyCode::XAF,
        gec: Some(GEC::CD),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "15",
        ioc: Some(IOC::CHA),
        iso_long_name: "The Republic of Chad",
        iso_short_name: "Chad",
        official_language_list: ["ar", "fr"].to_vec(),
        spoken_language_list: ["ar", "fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Chadian"),
        number: "148",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::MiddleAfrica),
        un_locode: "TD",
        unofficial_name_list: ["Chad", "تشاد", "Tschad", "Tchad", "チャド", "Tsjaad"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Chad"),
            ("af", "Tsjad"),
            ("ak", "Chad"),
            ("am", "ኂ፥"),
            ("an", "Chad"),
            ("ar", "تشاد"),
            ("as", "চ\u{9be}ড"),
            ("ay", "Chad"),
            ("az", "Çad"),
            ("ba", "Chad"),
            ("be", "Чад"),
            ("bg", "Чад"),
            ("bi", "Chad"),
            ("bn", "চ\u{9be}ড"),
            ("bn_IN", "চ\u{9be}ড"),
            ("br", "Tchad"),
            ("bs", "Čad"),
            ("ca", "Txad"),
            ("ce", "Чад"),
            ("ch", "Chad"),
            ("cs", "Čad"),
            ("cv", "Чад"),
            ("cy", "Chad"),
            ("da", "Tchad"),
            ("de", "Tschad"),
            ("dv", "ޝ\u{7a7}ދ\u{7aa}"),
            ("dz", "ཆཌ\u{f72}།"),
            ("ee", "Chad"),
            ("el", "Τσαντ"),
            ("en", "Chad"),
            ("eo", "Ĉado"),
            ("es", "Chad"),
            ("et", "Tšaad"),
            ("eu", "Txad"),
            ("fa", "چاد"),
            ("ff", "Chad"),
            ("fi", "Tšad"),
            ("fo", "Kjad"),
            ("fr", "Tchad"),
            ("fy", "Tsjaad"),
            ("ga", "Sead"),
            ("gl", "Chad"),
            ("gn", "Chad"),
            ("gu", "ચાડ"),
            ("gv", "Shad"),
            ("ha", "Cadi"),
            ("he", "צ׳אד"),
            ("hi", "चाड"),
            ("hr", "Čad"),
            ("ht", "Tchad"),
            ("hu", "Csád"),
            ("hy", "Չադ"),
            ("ia", "Tchad"),
            ("id", "Chad"),
            ("io", "Chad"),
            ("is", "Tsjad"),
            ("it", "Ciad"),
            ("iu", "Chad"),
            ("ja", "チャド"),
            ("ka", "ჩადი"),
            ("ki", "Chad"),
            ("kk", "Чад"),
            ("kl", "Chad"),
            ("km", "ឆាដ"),
            ("kn", "ಛಾಡ\u{ccd}"),
            ("ko", "차드"),
            ("ku", "Çad"),
            ("kv", "Chad"),
            ("kw", "Chad"),
            ("ky", "Чад"),
            ("lo", "Chad"),
            ("lt", "Čadas"),
            ("lv", "Čada"),
            ("mi", "Chad"),
            ("mk", "Чад"),
            ("ml", "ഛ\u{d3e}ഡ\u{d4d}"),
            ("mn", "Чад"),
            ("mr", "चाद"),
            ("ms", "Cad"),
            ("mt", "Ċad"),
            (
                "my",
                "ချဒ\u{103a}သမ\u{1039}မတန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tsiad"),
            ("nb", "Tsjad"),
            ("ne", "चाद"),
            ("nl", "Tsjaad"),
            ("nn", "Tsjad"),
            ("nv", "Chad"),
            ("oc", "Chad"),
            ("or", "ଚ\u{b3e}ଡ\u{b4d}"),
            ("pa", "ਚਾਦ"),
            ("pi", "चाड"),
            ("pl", "Czad"),
            ("ps", "چاډ"),
            ("pt", "Chade"),
            ("pt_BR", "Chade"),
            ("ro", "Ciad"),
            ("ru", "Чад"),
            ("rw", "Cade"),
            ("sc", "Chad"),
            ("sd", "چاڊ"),
            ("si", "cqDw"),
            ("sk", "Čad"),
            ("sl", "Čad"),
            ("so", "Jaad"),
            ("sq", "Çad"),
            ("sr", "Чад"),
            ("sv", "Tchad"),
            ("sw", "Chad"),
            ("ta", "ச\u{bbe}ட\u{bcd}"),
            ("te", "చ\u{c3e}ద\u{c4d}"),
            ("tg", "Чад"),
            ("th", "ชาด"),
            ("ti", "ቻድ"),
            ("tk", "Çad"),
            ("tl", "Tsad"),
            ("tr", "Çad"),
            ("tt", "Чад"),
            ("ug", "چاد"),
            ("uk", "Чад"),
            ("ur", "چاڈ"),
            ("uz", "Chad"),
            ("ve", "Chad"),
            ("vi", "Chê-đ"),
            ("wa", "Tchad"),
            ("wo", "Caad"),
            ("xh", "Chad"),
            ("yo", "Tsad"),
            ("zh_CN", "乍得"),
            ("zh_HK", "乍得"),
            ("zh_TW", "查德"),
            ("zu", "ITshedi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
