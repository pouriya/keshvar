// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Latvia

#[cfg(all(feature = "lv", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::LV;
    pub const ALPHA3: Alpha3 = Alpha3::LVA;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 371;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::LG);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::LAT);
    pub const ISO_SHORT_NAME: &str = "Latvia";
    pub const ISO_LONG_NAME: &str = "The Republic of Latvia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["lv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["lv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Latvian");
    pub const NUMBER: &str = "428";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("LV-\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "LV";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Latvia",
        "Lettland",
        "Lettonie",
        "Letonia",
        "ラトビア",
        "Letland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Latvia"),
        ("af", "Letland"),
        ("ak", "Latvia"),
        ("am", "ሒትቱ።"),
        ("an", "Latvia"),
        ("ar", "لاتفيا"),
        ("as", "ল\u{9be}টভিয়\u{9be}"),
        ("ay", "Latvia"),
        ("az", "Latviya"),
        ("ba", "Latvia"),
        ("be", "Латвія"),
        ("bg", "Латвия"),
        ("bi", "Latvia"),
        ("bn", "ল\u{9be}টভিয়\u{9be}"),
        ("bn_IN", "ল\u{9be}টভিয়\u{9be}"),
        ("br", "Latvia"),
        ("bs", "Latvija"),
        ("ca", "Letònia"),
        ("ce", "Латви"),
        ("ch", "Latvia"),
        ("cs", "Lotyšsko"),
        ("cv", "Латви"),
        ("cy", "Latfia"),
        ("da", "Letland"),
        ("de", "Lettland"),
        ("dv", "ލ\u{7ac}ޓ\u{7aa}ވ\u{7a8}އ\u{7a7}"),
        ("dz", "ལ\u{f7a}ཊ\u{f72}་བ\u{f72}་ཡ།"),
        ("ee", "Latvia"),
        ("el", "Λετονία"),
        ("en", "Latvia"),
        ("eo", "Latvio"),
        ("es", "Letonia"),
        ("et", "Läti"),
        ("eu", "Letonia"),
        ("fa", "لتونی"),
        ("ff", "Latvia"),
        ("fi", "Latvia"),
        ("fo", "Lettland"),
        ("fr", "Lettonie"),
        ("fy", "Letlân"),
        ("ga", "An Laitvia"),
        ("gl", "Letonia"),
        ("gn", "Latvia"),
        ("gu", "લ\u{ac7}ટવિયા"),
        ("gv", "Yn Latvey"),
        ("ha", "Laitfiya"),
        ("he", "לטביה"),
        ("hi", "लातविया"),
        ("hr", "Latvija"),
        ("ht", "Letoni"),
        ("hu", "Lettország"),
        ("hy", "Լատվիա"),
        ("ia", "Lettonia"),
        ("id", "Latvia"),
        ("io", "Latvia"),
        ("is", "Lettland"),
        ("it", "Lettonia"),
        ("iu", "Latvia"),
        ("ja", "ラトビア"),
        ("ka", "ლატვია"),
        ("ki", "Latvia"),
        ("kk", "Латвия"),
        ("kl", "Latvia"),
        ("km", "ឡាតវ\u{17b8}យ\u{17c9}ា"),
        ("kn", "ಲಾತ\u{ccd}ವ\u{cbf}ಯಾ"),
        ("ko", "라트비아"),
        ("ku", "Latviya"),
        ("kv", "Латвия"),
        ("kw", "Latvi"),
        ("ky", "Латвия"),
        ("lo", "ປະເທດແລດໂຕນ\u{eb5}"),
        ("lt", "Latvija"),
        ("lv", "Latvija"),
        ("mi", "Rāwhia"),
        ("mk", "Летонија"),
        ("ml", "ല\u{d3e}ത\u{d4d}വിയ"),
        ("mn", "Латви"),
        ("mr", "लाटव\u{94d}हिया"),
        ("ms", "Latvia"),
        ("mt", "Latvja"),
        (
            "my",
            "လတ\u{103a}ဗ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ratebiya"),
        ("nb", "Latvia"),
        ("ne", "लात\u{94d}भिया"),
        ("nl", "Letland"),
        ("nn", "Latvia"),
        ("nv", "Létbiiya"),
        ("oc", "Letònia"),
        ("or", "ଲ\u{b3e}ଟଭ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਲਾਟੀਵਾਆ"),
        ("pi", "लाट\u{94d}विया"),
        ("pl", "Łotwa"),
        ("ps", "Latvia"),
        ("pt", "Letónia"),
        ("pt_BR", "Letônia"),
        ("ro", "Letonia"),
        ("ru", "Латвия"),
        ("rw", "Lativiya"),
        ("sc", "Letònia"),
        ("sd", "Latvia"),
        ("si", "ලැත\u{dca}ව\u{dd2}ය\u{dcf}ව"),
        ("sk", "Lotyšsko"),
        ("sl", "Latvija"),
        ("so", "Laatfiya"),
        ("sq", "Letoni"),
        ("sr", "Летонија"),
        ("sv", "Lettland"),
        ("sw", "Latvia"),
        ("ta", "லட\u{bcd}விய\u{bbe}"),
        ("te", "ల\u{c3e}టవ\u{c4d}హ\u{c3f}య\u{c3e}"),
        ("tg", "Латвия"),
        ("th", "ล\u{e31}ตเว\u{e35}ย"),
        ("ti", "ላትቪያ"),
        ("tk", "Litwiýa"),
        ("tl", "Latvia"),
        ("tr", "Letonya"),
        ("tt", "Латвиа"),
        ("ug", "لاتۋىيە"),
        ("uk", "Латвія"),
        ("ur", "لٹویا"),
        ("uz", "Latviya"),
        ("ve", "Latvia"),
        ("vi", "Lát-vi-a"),
        ("wa", "Letoneye"),
        ("wo", "Letóoni"),
        ("xh", "Latvia"),
        ("yo", "Látfíà"),
        ("zh_CN", "拉脱维亚"),
        ("zh_HK", "拉脫維亞"),
        ("zh_TW", "拉脫維亞"),
        ("zu", "ILatviya"),
    ];
    #[cfg(all(feature = "lv", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 56.879635;
        pub const LONGITUDE: f64 = 24.603189;
        pub const MAX_LATITUDE: f64 = 58.0855688;
        pub const MAX_LONGITUDE: f64 = 28.2414029;
        pub const MIN_LATITUDE: f64 = 55.6747769;
        pub const MIN_LONGITUDE: f64 = 20.8465999;
        pub const NORTHEAST_LATITUDE: f64 = 58.0855688;
        pub const NORTHEAST_LONGITUDE: f64 = 28.2414029;
        pub const SOUTHWEST_LATITUDE: f64 = 55.6747769;
        pub const SOUTHWEST_LONGITUDE: f64 = 20.8465999;
    }
}
#[cfg(all(feature = "lv", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 56.879635,
            longitude: 24.603189,
            max_latitude: 58.0855688,
            max_longitude: 28.2414029,
            min_latitude: 55.6747769,
            min_longitude: 20.8465999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 58.0855688,
                    longitude: 28.2414029,
                },
                southwest: CountryGeoBound {
                    latitude: 55.6747769,
                    longitude: 20.8465999,
                },
            },
        }
    }
}

#[cfg(all(feature = "lv", feature = "subdivisions"))]
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
                    "002",
                    Subdivision{
                        name: "002",
                        country_alpha2: Alpha2::LV,
                        code: "002",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية إيزكروكل"), ("az", "Ayzqrauql diyarı"), ("be", "Айзкраўкльскі край"), ("bn", "আইজক\u{9cd}রকল পৌরসভ\u{9be}"), ("ca", "Municipi d’Aizkraukle"), ("ccp", "𑄃𑄃\u{11128}𑄌\u{11134}𑄇\u{11133}𑄢\u{1112d}𑄅\u{1112a}𑄇\u{11134}𑄣𑄬"), ("da", "Aizkraukle municipality"), ("de", "Bezirk Aizkraukle"), ("el", "Άιζκραουκλε"), ("en", "Aizkraukle"), ("es", "Municipio de Aizkraukle"), ("et", "Aizkraukle piirkond"), ("eu", "Aizkraukleko udalerria"), ("fa", "شهر ایزکراکل"), ("fi", "Aizkrauklen kunta"), ("fr", "Aizkraukles novads"), ("gu", "ઐઝક\u{acd}ર\u{ac1}કલ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "एज\u{93c}क\u{94d}रोक\u{94d}ल\u{947} नगरपालिका"), ("hy", "Այզկրաուկլեի շրջան"), ("id", "Kotamadya Aizkraukle"), ("it", "Comune di Aizkraukle"), ("ja", "アイズクラウクレ"), ("ka", "აიზკრაუკლეს მხარე"), ("kn", "ಐಜ\u{ccd}ಕ\u{ccd}ರಾಕುಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "아이스크라우클레 시"), ("lt", "Aizkrauklės savivaldybė"), ("lv", "Aizkraukles novads"), ("mk", "Ајзкраукле"), ("mr", "एईस\u{94d}क\u{94d}राइकल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Aizkraukle municipality"), ("nb", "Aizkraukle Kommune"), ("nl", "Aizkraukles novads"), ("no", "Aizkraukle Kommune"), ("pl", "Gmina Aizkraukle"), ("pt", "Município de Aizkraukle"), ("ru", "Айзкраукльский край"), ("si", "අය\u{dd2}ස\u{dca}ක\u{dca}රව\u{dd4}ක\u{dca}ලේ නගර සභ\u{dcf}ව"), ("sv", "Aizkraukle kommun"), ("ta", "ஐஸ\u{bcd}க\u{bcd}ரூக\u{bcd}களே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఐజుక\u{c4d}ర\u{c3e}క\u{c4d}ల\u{c47} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอ\u{e34}ซเคราเค\u{e34}ลส\u{e4c}"), ("tr", "Aizkraukle Belediyesi"), ("uk", "Айзкраукльський край"), ("ur", "آئزکراوکلے بلدیہ"), ("vi", "Đô thị tự trị Aizkraukle"), ("zh", "艾茲克勞克萊自治市")]),
                        unofficial_name_list: ["Aizkraukle"].to_vec(),
                    }
                ),
                (
                    "007",
                    Subdivision{
                        name: "007",
                        country_alpha2: Alpha2::LV,
                        code: "007",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ألوكسنه"), ("be", "Алуксненскі край"), ("bn", "আল\u{9c1}ক\u{9cd}সনে পৌরসভ\u{9be}"), ("ca", "Municipi d’Alūksne"), ("ccp", "𑄃𑄣\u{1112a}𑄇\u{11134}𑄌\u{11134}"), ("da", "Alūksne municipality"), ("de", "Bezirk Alūksne"), ("el", "Αλούκσνε"), ("en", "Alūksne"), ("es", "Municipalidad de Alūksne"), ("et", "Alūksne piirkond"), ("eu", "Alūksne udalerria"), ("fa", "شهر الاکسن"), ("fi", "Alūksnen kunta"), ("fr", "Alūksne"), ("gu", "અલ\u{ac1}ક\u{acd}સન\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "अल\u{94d}य\u{942}कन\u{947} नगरपालिका"), ("hy", "Ալուքսենեի շրջան"), ("id", "Kotamadya Alūksne"), ("it", "Alūksne"), ("ja", "アルークスネ"), ("ka", "ალუქსნეს მხარე"), ("kn", "ಅಲುಕ\u{ccd}ಸ\u{ccd}ನ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "알룩스네 시"), ("lt", "Alūksnės savivaldybė"), ("lv", "Alūksnes novads"), ("mk", "Алуксне"), ("mr", "अल\u{941}क\u{94d}सन\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Aluksne municipality"), ("nb", "Aluksne kommune"), ("nl", "Alūksnes novads"), ("no", "Aluksne kommune"), ("pl", "Gmina Alūksnē"), ("pt", "Município de Aluksne"), ("ru", "Алуксненский край"), ("si", "අල\u{dd4}ක\u{dca}ස\u{dca}නේ නගර සභ\u{dcf}ව"), ("sv", "Aluksne kommun"), ("ta", "லுக\u{bcd}ஸ\u{bcd}நெ நகர\u{bbe}ட\u{bcd}சி"), ("te", "అలూక\u{c4d}స\u{c4d}\u{200c}న\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "อล\u{e38}กน\u{e35} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Aluksne Belediyesi"), ("uk", "Алуксненський край"), ("ur", "آلوکسنے بلدیہ"), ("vi", "Đô thị tự trị Aluksne"), ("zh", "阿盧克斯內自治市")]),
                        unofficial_name_list: ["Alūksne"].to_vec(),
                    }
                ),
                (
                    "011",
                    Subdivision{
                        name: "011",
                        country_alpha2: Alpha2::LV,
                        code: "011",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية آداجي"), ("be", "Адажскі край"), ("bn", "আদ\u{9be}জি পৌরসভ\u{9be}"), ("ca", "Municipi d’Ādaži"), ("ccp", "𑄃𑄓𑄎\u{11128}"), ("ceb", "Ādažu Novads"), ("da", "Ādaži municipality"), ("de", "Bezirk Ādaži"), ("el", "Αντάζι"), ("en", "Ādaži"), ("es", "Municipalidad de Ādaži"), ("et", "Ādaži piirkond"), ("eu", "Ādaži udalerria"), ("fa", "شهرداری آداژی"), ("fi", "Ādažin kunta"), ("fr", "Ādaži"), ("gu", "આડ\u{ac7}ઝી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "अदाज\u{93c}ी नगरपालिका"), ("hy", "Ադաժի շրջան"), ("id", "Kotamadya Ādaži"), ("it", "Comune di Ādaži"), ("ja", "アーダジ"), ("ka", "ადაჟის მხარე"), ("kn", "ಆಜಾಜ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "아다지 시"), ("lt", "Adažų savivaldybė"), ("lv", "Ādažu novads"), ("mk", "Адажи"), ("mr", "आड\u{947}झी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Adazi municipality"), ("nb", "Adazi kommune"), ("nl", "Ādažu novads"), ("no", "Adazi kommune"), ("pl", "Gmina Ādaži"), ("pt", "Município de Adazi"), ("ru", "Адажский край"), ("si", "අඩස\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Ādažu novads"), ("ta", "அடசி நகர\u{bbe}ட\u{bcd}சி"), ("te", "అడ\u{c3e}జ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอดาซ\u{e35}"), ("tr", "Adazi Belediyesi"), ("uk", "Адажський край"), ("ur", "آداژی بلدیہ"), ("vi", "Đô thị tự trị Adazi"), ("zh", "阿達日自治市")]),
                        unofficial_name_list: ["Ādaži"].to_vec(),
                    }
                ),
                (
                    "015",
                    Subdivision{
                        name: "015",
                        country_alpha2: Alpha2::LV,
                        code: "015",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بالفي"), ("be", "Балвскі край"), ("bn", "ব\u{9be}ল\u{9cd}ভি পৌরসভ\u{9be}"), ("ca", "Municipi de Balvi"), ("ccp", "𑄝\u{11127}𑄣\u{11134}𑄞\u{11128}"), ("da", "Balvi municipality"), ("de", "Bezirk Balvi"), ("el", "Μπαλβί"), ("en", "Balvi"), ("es", "Municipalidad de Balvi"), ("et", "Balvi piirkond"), ("eu", "Balvi udalerria"), ("fa", "شهرداری بالوی"), ("fi", "Balvin kunta"), ("fr", "Balvi"), ("gu", "બાલ\u{acd}વી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बलवी नगरपालिका"), ("hy", "Բալվիի շրջան"), ("id", "Kotamadya Balvi"), ("it", "Balvi"), ("ja", "バルヴィ"), ("ka", "ბალვის მხარე"), ("kn", "ಬಾಲ\u{ccd}ವ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "발비 시"), ("lt", "Balvų savivaldybė"), ("lv", "Balvu novads"), ("mk", "Балви"), ("mr", "बलवी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Balvi"), ("nb", "Balvi kommune"), ("nl", "Balvu novads"), ("no", "Balvi kommune"), ("pl", "Gmina Balvi"), ("pt", "Município de Balvi"), ("ru", "Балвский край"), ("si", "බල\u{dca}ව\u{dd3} නගර සභ\u{dcf}ව"), ("sv", "Balvi kommun"), ("ta", "ப\u{bbe}ல\u{bcd}வி நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c3e}ల\u{c4d}వ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลบาลว\u{e35}"), ("tr", "Balvi Belediyesi"), ("uk", "Балвський край"), ("ur", "بالوی بلدیہ"), ("vi", "Đô thị tự trị Balvi"), ("zh", "巴爾維自治市")]),
                        unofficial_name_list: ["Balvi"].to_vec(),
                    }
                ),
                (
                    "016",
                    Subdivision{
                        name: "016",
                        country_alpha2: Alpha2::LV,
                        code: "016",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوسكا"), ("be", "Баўскі край"), ("bn", "ব\u{9be}উস\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Bauska"), ("ccp", "𑄝𑄅\u{1112a}𑄌\u{11134}𑄇"), ("da", "Bauska municipality"), ("de", "Bezirk Bauska"), ("el", "Μπάουσκα"), ("en", "Bauska"), ("es", "Municipalidad de Bauska"), ("et", "Bauska piirkond"), ("eu", "Bauska udalerria"), ("fa", "شهرداری باوسکا"), ("fi", "Bauskan kunta"), ("fr", "Bauska"), ("gu", "બાઉસ\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बाउस\u{94d}का नगर पालिका"), ("hy", "Բաուսկայի շրջան"), ("id", "Kotamadya Bauska"), ("it", "Comune di Bauska"), ("ja", "バウスカ"), ("ka", "ბაუსკის მხარე"), ("kn", "ಬಾಸ\u{ccd}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "바우스카 시"), ("lt", "Bauskės savivaldybė"), ("lv", "Bauskas novads"), ("mk", "Бауска"), ("mr", "बाऊसका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bauska municipality"), ("nb", "Bauska kommune"), ("nl", "Bauskas novads"), ("no", "Bauska kommune"), ("pl", "Gmina Bauska"), ("pt", "Município de Bauska"), ("ru", "Бауский край"), ("si", "බෞස\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Bauska kommun"), ("ta", "பஸ\u{bcd}க\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c3e}స\u{c4d}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เบาสกา"), ("tr", "Bauska Belediyesi"), ("uk", "Бауський край"), ("ur", "باوسکا میونسپلٹی"), ("vi", "Đô thị tự trị Bauska"), ("zh", "包斯卡自治市")]),
                        unofficial_name_list: ["Bauska"].to_vec(),
                    }
                ),
                (
                    "022",
                    Subdivision{
                        name: "022",
                        country_alpha2: Alpha2::LV,
                        code: "022",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تسيسس"), ("be", "Цэсіскі край"), ("bn", "চেসিস পৌরসভ\u{9be}"), ("ca", "Municipi de Cēsis"), ("ccp", "𑄥𑄬𑄌\u{11128}𑄌\u{11134}"), ("ceb", "Cēsu Novads"), ("da", "Cēsis municipality"), ("de", "Bezirk Cēsis"), ("el", "Κέσις"), ("en", "Cēsis"), ("es", "Municipalidad de Cēsis"), ("et", "Cēsise piirkond"), ("eu", "Cēsis udalerria"), ("fa", "شهرداری سسیس"), ("fi", "Cēsisin kunta"), ("fr", "Cēsis"), ("gu", "ક\u{ac7}સીસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{947}सिस नगरपालिका"), ("id", "Kotamadya Cēsis"), ("it", "Comune di Cēsis"), ("ja", "ツェースィス"), ("ka", "ცესისის მხარე"), ("kn", "ಕ\u{cc6}ಸ\u{cbf}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "체시스 시"), ("lt", "Cėsių savivaldybė"), ("lv", "Cēsu novads"), ("mk", "Цесис"), ("mr", "क\u{945}सिस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cesis municipality"), ("nb", "Cesis Kommune"), ("nl", "Cēsu novads"), ("no", "Cesis Kommune"), ("pl", "Gmina Kieś"), ("pt", "Município de Cesis"), ("ru", "Цесисский край"), ("si", "සේස\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Cēsu novads"), ("ta", "சேசிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}స\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซซ\u{e34}ส"), ("tr", "Cesis Bölgesi"), ("uk", "Цесіський край"), ("ur", "چیسس بلدیہ"), ("vi", "Đô thị tự trị Cesis"), ("zh", "采西斯自治市")]),
                        unofficial_name_list: ["Cēsis"].to_vec(),
                    }
                ),
                (
                    "026",
                    Subdivision{
                        name: "026",
                        country_alpha2: Alpha2::LV,
                        code: "026",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبيل"), ("be", "Добэльскі край"), ("bn", "ডোবেল পৌরসভ\u{9be}"), ("ca", "Municipi de Dobele"), ("ccp", "𑄓\u{11127}𑄝\u{11128}𑄣\u{11128}"), ("da", "Dobele municipality"), ("de", "Bezirk Dobele"), ("el", "Ντόμπελε"), ("en", "Dobele"), ("es", "Municipalidad de Dobele"), ("et", "Dobele piirkond"), ("eu", "Dobele udalerria"), ("fa", "شهرداری دوبل"), ("fi", "Dobelen kunta"), ("fr", "Dobele"), ("gu", "ડોબ\u{ac7}લ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "दोब\u{947}ल\u{947} नगरपालिका"), ("hy", "Դոբելեի շրջան"), ("id", "Kotamadya Dobele"), ("it", "Comune di Dobele"), ("ja", "ドベレ"), ("ka", "დობელეს მხარე"), ("kn", "ಡೊಬ\u{cbf}ಲ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "도벨레 시"), ("lt", "Duobelės savivaldybė"), ("lv", "Dobeles novads"), ("mk", "Добеле"), ("mr", "डोब\u{947}ल\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dobele municipality"), ("nb", "Dobele Kommune"), ("nl", "Dobele"), ("no", "Dobele Kommune"), ("pl", "Gmina Dobele"), ("pt", "Município de Dobele"), ("ru", "Добельский край"), ("si", "ඩොබෙලේ නගර සභ\u{dcf}ව"), ("sv", "Dobele (kommun)"), ("ta", "டொபெலே நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}బ\u{c46}ల\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโดเบเล\u{e48}"), ("tr", "Dobele Belediyesi"), ("uk", "Добельський край"), ("ur", "دوبیلے بلدیہ"), ("vi", "Đô thị tự trị Dobele"), ("zh", "多貝萊自治市")]),
                        unofficial_name_list: ["Dobele"].to_vec(),
                    }
                ),
                (
                    "033",
                    Subdivision{
                        name: "033",
                        country_alpha2: Alpha2::LV,
                        code: "033",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غولبين"), ("be", "Гулбенскі край"), ("bn", "গ\u{9c1}ল\u{9cd}বেন পৌরসভ\u{9be}"), ("ca", "Municipi de Gulbene"), ("ccp", "𑄉\u{1112a}𑄣\u{11134}𑄝𑄬𑄚𑄬"), ("da", "Gulbene municipality"), ("de", "Bezirk Gulbene"), ("el", "Γκουλμπένε"), ("en", "Gulbene"), ("es", "Municipalidad de Gulbene"), ("et", "Gulbene piirkond"), ("eu", "Gulbene udalerria"), ("fa", "شهرداری گولبن"), ("fi", "Gulbene"), ("fr", "Gulbene"), ("gu", "ગ\u{ac1}લબ\u{ac7}ન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ग\u{941}लब\u{947}न\u{947} नगरपालिका"), ("hy", "Գուլբենեի շրջան"), ("id", "Kotamadya Gulbene"), ("it", "Comune di Gulbene"), ("ja", "グルベネ"), ("ka", "გულბენეს მხარე"), ("kn", "ಗುಲ\u{ccd}ಬೀನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "굴베네 시"), ("lt", "Gulbenės savivaldybė"), ("lv", "Gulbenes novads"), ("mk", "Гулбене"), ("mr", "ग\u{941}लाब\u{947}न म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gulbene municipality"), ("nb", "Gulbene Kommune"), ("nl", "Gulbenes novads"), ("no", "Gulbene Kommune"), ("pl", "Gmina Gulbene"), ("pt", "Município de Gulbene"), ("ru", "Гулбенский край"), ("si", "ග\u{dd4}ල\u{dca}බෙනේ නගර සභ\u{dcf}ව"), ("sv", "Gulbene (kommun)"), ("ta", "கோல\u{bcd}பேனே நகர\u{bbe}ட\u{bcd}சி"), ("te", "గుల\u{c4d}బ\u{c40}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องก\u{e38}ลเบเน"), ("tr", "Gulbene Belgesi"), ("uk", "Гулбенський край"), ("ur", "گولبینے بلدیہ"), ("vi", "Đô thị tự trị Gulbene"), ("zh", "古爾貝內自治市")]),
                        unofficial_name_list: ["Gulbene"].to_vec(),
                    }
                ),
                (
                    "041",
                    Subdivision{
                        name: "041",
                        country_alpha2: Alpha2::LV,
                        code: "041",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جيلغافا"), ("be", "Елгаўскі край"), ("bn", "জেল\u{9cd}গ\u{9be}ভ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Jelgava"), ("ccp", "𑄎𑄬𑄣\u{11134}𑄉𑄞 𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄥\u{11128}𑄛𑄣\u{11128}𑄑\u{11128}"), ("da", "Jelgava municipality"), ("de", "Bezirk Jelgava"), ("el", "Γιέλγκαβα"), ("en", "Jelgava Municipality"), ("es", "Municipalidad de Jelgava"), ("et", "Jelgava piirkond"), ("eu", "Jelgava udalerria"), ("fa", "شهرداری یلگاوا"), ("fi", "Jelgavan kunta"), ("fr", "Jelgava"), ("gu", "જ\u{ac7}લગાવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{947}ल\u{94d}गावा नगरपालिका"), ("hy", "Ելգավայի շրջան"), ("id", "Kotamadya Jelgava"), ("it", "Jelgava"), ("ja", "ヤルガワ"), ("ka", "ელგავის მხარე"), ("kn", "ಜ\u{cc6}ಲ\u{ccd}ಗಾವ ಪುರಸಭ\u{cc6}"), ("ko", "옐가바 시"), ("lt", "Jelgavos savivaldybė"), ("lv", "Jelgavas novads"), ("mk", "Јелгава"), ("mr", "ज\u{947}ल\u{94d}गा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jelgava municipality"), ("nb", "Jelgava kommune"), ("nl", "Jelgavas novads"), ("no", "Jelgava kommune"), ("pl", "Gmina Jełgawa"), ("pt", "Município de Jelgava"), ("ru", "Елгавский край"), ("si", "ජෙල\u{dca}ගව\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Jelgava kommun"), ("ta", "ஜெல\u{bcd}கவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జల\u{c4d}గ\u{c3e}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลนครเยลกาวา"), ("tr", "Jelgava Belediyesi"), ("uk", "Єлгавський край"), ("ur", "جیلجاوا میونسپلٹی"), ("vi", "Đô thị tự trị Jelgava"), ("zh", "葉爾加瓦自治市")]),
                        unofficial_name_list: ["Jelgava"].to_vec(),
                    }
                ),
                (
                    "042",
                    Subdivision{
                        name: "042",
                        country_alpha2: Alpha2::LV,
                        code: "042",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جوكابيلز"), ("be", "Екабпілскі край"), ("bn", "জেক\u{9be}বপিলস পৌরসভ\u{9be}"), ("ca", "Municipi de Jēkabpils"), ("ccp", "𑄎𑄬𑄇𑄛\u{11134}𑄛\u{11128}𑄣\u{11134} 𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄥\u{11128}𑄛𑄣\u{11128}𑄑\u{11128}"), ("ceb", "Jēkabpils Municipality"), ("da", "Jekabpils municipality"), ("de", "Jēkabpils novads"), ("el", "Γιέκαμπιλς"), ("en", "Jēkabpils Municipality"), ("es", "Municipalidad de Jēkabpils"), ("et", "Jēkabpilsi piirkond"), ("eu", "Jēkabpils udalerria"), ("fa", "شهرداری یکابپلیس"), ("fi", "Jēkabpilsin kunta"), ("fr", "Jēkabpils"), ("gu", "જ\u{ac7}ક\u{ac7}બપિલ\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{947}कबपिल\u{94d}स नगरपालिका"), ("hy", "Եկաբպիսլի շրջան"), ("id", "Kotamadya Jēkabpils"), ("it", "Jēkabpils"), ("ja", "ヤーカブピルス"), ("ka", "იეკაბპილსის მხარე"), ("kn", "ಜ\u{cc6}ಕಾಬ\u{ccd}ಪ\u{cbf}ಲ\u{ccd}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "예캅필스 시"), ("lt", "Jėkabpilio savivaldybė"), ("lv", "Jēkabpils novads"), ("mk", "Јекабпилс"), ("mr", "ज\u{947}कबपिल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jekabpils municipality"), ("nb", "Jekabpils kommune"), ("nl", "Jēkabpils novads"), ("no", "Jekabpils kommune"), ("pl", "Gmina Jēkabpils"), ("pt", "Município de Jekabpils"), ("ru", "Екабпилсский край"), ("si", "ජේකබ\u{dca}ප\u{dd2}ල\u{dca}ස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Jēkabpils novads"), ("ta", "ஜெகபபிள\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}క\u{c3e}బ\u{c4d}ప\u{c3f}ల\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเจค\u{e31}บพ\u{e34}ล"), ("tr", "Jekabpils Belediyesi"), ("uk", "Єкабпілський край"), ("ur", "جیکابپیلس میونسپلٹی"), ("vi", "Đô thị tự trị Jekabpils"), ("zh", "葉卡布皮爾斯自治市")]),
                        unofficial_name_list: ["Jēkabpils"].to_vec(),
                    }
                ),
                (
                    "047",
                    Subdivision{
                        name: "047",
                        country_alpha2: Alpha2::LV,
                        code: "047",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كراسلافا"), ("be", "Краслаўскі край"), ("bn", "ক\u{9cd}র\u{9be}স\u{9cd}ল\u{9be}ভ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Krāslava"), ("ccp", "𑄇\u{11133}𑄢𑄌\u{11134}𑄣𑄞"), ("da", "Krāslava municipality"), ("de", "Bezirk Krāslava"), ("el", "Κράσλαβα"), ("en", "Krāslava"), ("es", "Municipalidad de Krāslava"), ("et", "Krāslava piirkond"), ("eu", "Krāslava udalerria"), ("fa", "شهرداری کراسلاوا"), ("fi", "Krāslavan kunta"), ("fr", "Krāslava"), ("gu", "ક\u{acd}રાસ\u{acd}લાવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रस\u{94d}लावा नगरपालिका"), ("id", "Kotamadya Krāslava"), ("it", "Comune di Krāslava"), ("ja", "クラスラヴァ"), ("ka", "კრასლავის მხარე"), ("kn", "ಕ\u{ccd}ರಾಸ\u{ccd}ಲಾವಾ ಪುರಸಭ\u{cc6}"), ("ko", "크라슬라바 시"), ("lt", "Kraslavos savivaldybė"), ("lv", "Krāslavas novads"), ("mk", "Краслава"), ("mr", "क\u{94d}रासलवा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Kraslave"), ("nb", "Kraslava kommune"), ("nl", "Krāslavas novads"), ("no", "Kraslava kommune"), ("pl", "Gmina Krasław"), ("pt", "Município de Kraslava"), ("ru", "Краславский край"), ("si", "ක\u{dca}ර\u{dcf}සල\u{dca}ව\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Kraslava kommun"), ("ta", "கிர\u{bbe}ஸ\u{bcd}லவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c3e}స\u{c4d}ల\u{c3e}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลคราสลาว\u{e48}า"), ("tr", "Kraslava Belediyesi"), ("uk", "Краславський край"), ("ur", "کراسلاوا میونسپلٹی"), ("vi", "Đô thị tự trị Kraslava"), ("zh", "克拉斯拉瓦自治市")]),
                        unofficial_name_list: ["Krāslava"].to_vec(),
                    }
                ),
                (
                    "050",
                    Subdivision{
                        name: "050",
                        country_alpha2: Alpha2::LV,
                        code: "050",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كولديغا"), ("be", "Кулдыгскі край"), ("bn", "ক\u{9c1}লদিগ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Kuldīga"), ("ccp", "𑄇\u{1112a}𑄣\u{11134}𑄓\u{11128}𑄉"), ("da", "Kuldīga municipality"), ("de", "Bezirk Kuldīga"), ("el", "Κουλντίγκα"), ("en", "Kuldīga"), ("es", "Municipalidad de Kuldīga"), ("et", "Kuldīga piirkond"), ("eu", "Kuldīga udalerria"), ("fa", "شهرداری کولدیگا"), ("fi", "Kuldīgan kunta"), ("fr", "Kuldīga"), ("gu", "ક\u{ac1}લડીગા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{941}लदीगा नगरपालिका"), ("id", "Kotamadya Kuldīga"), ("it", "Comune di Kuldīga"), ("ja", "クルディーガ"), ("ka", "კულდიგის მხარე"), ("kn", "ಕುಲ\u{ccd}ದ\u{cbf}ಗಾ ಪುರಸಭ\u{cc6}"), ("ko", "쿨디가 시"), ("lt", "Kuldygos savivaldybė"), ("lv", "Kuldīgas novads"), ("mk", "Кулдига"), ("mr", "क\u{941}लदीगा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kuldiga municipality"), ("nb", "Kuldiga kommune"), ("nl", "Kuldīgas novads"), ("no", "Kuldiga kommune"), ("pl", "Gmina Kuldīga"), ("pt", "Kuldiga"), ("ru", "Кулдигский край"), ("si", "ක\u{dd4}ල\u{dca}ඩ\u{dd2}ග\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Kuldiga kommun"), ("ta", "குலடிக\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "కుల\u{c4d}డ\u{c3f}గ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ค\u{e38}ลด\u{e35}กา"), ("tr", "Kuldiga Belediyesi"), ("uk", "Кулдізький край"), ("ur", "کولدیجا میونسپلٹی"), ("vi", "Đô thị tự trị Kuldiga"), ("zh", "庫爾迪加自治市")]),
                        unofficial_name_list: ["Kuldīga"].to_vec(),
                    }
                ),
                (
                    "052",
                    Subdivision{
                        name: "052",
                        country_alpha2: Alpha2::LV,
                        code: "052",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كيكافا"), ("be", "Кекаўскі край"), ("bn", "কেক\u{9be}ভ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Ķekava"), ("ccp", "𑄇𑄬𑄇𑄞"), ("da", "Ķekava municipality"), ("de", "Bezirk Ķekava"), ("el", "Κέκαβα"), ("en", "Ķekava"), ("es", "Municipalidad de Ķekava"), ("et", "Ķekava piirkond"), ("eu", "Ķekava udalerria"), ("fa", "شهرداری ککاوا"), ("fi", "Ķekavan kunta"), ("fr", "Ķekava"), ("gu", "ક\u{ac7}કાવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{947}कावा नगरपालिका"), ("hy", "Կեկավայի շրջան"), ("id", "Kotamadya Ķekava"), ("it", "Ķekava"), ("ja", "キェカワ"), ("ka", "კეკავის მხარე"), ("kn", "ಐಕ\u{cc6}ಟ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "체카바 시"), ("lt", "Kekavos savivaldybė"), ("lv", "Ķekavas novads"), ("mk", "Ќекава"), ("mr", "ऐक\u{94d}वा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kekava municipality"), ("nb", "Kekava kommune"), ("nl", "Ķekavas novads"), ("no", "Kekava kommune"), ("pl", "Gmina Ķekava"), ("pt", "Município de Kekava"), ("ru", "Кекавский край"), ("si", "කෙකව\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Kekava kommun"), ("ta", "கேயெகோவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c46}క\u{c3e}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเคกาวา"), ("tr", "Kekava Belediyesi"), ("uk", "Кекавський край"), ("ur", "کیکاوا میونسپلٹی"), ("vi", "Đô thị tự trị Kekava"), ("zh", "科卡瓦斯自治市")]),
                        unofficial_name_list: ["Ķekava"].to_vec(),
                    }
                ),
                (
                    "054",
                    Subdivision{
                        name: "054",
                        country_alpha2: Alpha2::LV,
                        code: "054",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ليمباجي"), ("be", "Лімбажскі край"), ("bn", "লিম\u{9cd}ব\u{9be}জি পৌরসভ\u{9be}"), ("ca", "Municipi de Limbaži"), ("ccp", "𑄣𑄟\u{11134}𑄝𑄎\u{11128}"), ("da", "Limbaži municipality"), ("de", "Limbaži"), ("el", "Λιμπάζι"), ("en", "Limbaži"), ("es", "Municipalidad de Limbaži"), ("et", "Limbaži piirkond"), ("eu", "Limbaži udalerria"), ("fa", "شهرداری لیمباژی"), ("fi", "Limbažin kunta"), ("fr", "Limbaži"), ("gu", "લિમ\u{acd}બ\u{ac7}ઝિ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लीम\u{94d}बाजी नगरपालिका"), ("hy", "Լիմբաժիի շրջան"), ("id", "Kotamadya Limbaži"), ("it", "Limbaži"), ("ja", "リンバジ"), ("ka", "ლიმბაჟის მხარე"), ("kn", "ಲ\u{cbf}ಂಬಾಝ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "림바지 시"), ("lt", "Limbažių savivaldybė"), ("lv", "Limbažu novads"), ("mk", "Лимбажи"), ("mr", "लिम\u{94d}बो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Limbazi municipality"), ("nb", "Limbazi kommune"), ("nl", "Limbažu novads"), ("no", "Limbazi kommune"), ("pl", "Gmina Limbaži"), ("pt", "Município de Limbazi"), ("ru", "Лимбажский край"), ("si", "ල\u{dd2}ම\u{dca}බස\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Limbazi kommun"), ("ta", "லிம\u{bcd}ப\u{bbe}ஜி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c3f}ంబ\u{c3e}జ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ดาม\u{e31}นและด\u{e35}อ\u{e39}"), ("tr", "Limbazi Belediyesi"), ("uk", "Лімбазький край"), ("ur", "لیمبازی میونسپلٹی"), ("vi", "Đô thị tự trị Limbazi"), ("zh", "林巴濟自治市")]),
                        unofficial_name_list: ["Limbaži"].to_vec(),
                    }
                ),
                (
                    "056",
                    Subdivision{
                        name: "056",
                        country_alpha2: Alpha2::LV,
                        code: "056",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ليفاني"), ("be", "Ліванскі край"), ("bn", "লিভ\u{9be}নি পৌরসভ\u{9be}"), ("ca", "Municipi de Līvāni"), ("ccp", "𑄣\u{11128}𑄞𑄚\u{11128}"), ("ceb", "Līvānu Novads"), ("da", "Līvāni"), ("de", "Bezirk Līvāni"), ("el", "Λιβάνι"), ("en", "Līvāni"), ("es", "Municipalidad de Līvāni"), ("et", "Līvāni piirkond"), ("eu", "Līvāni udalerria"), ("fa", "شهرداری لیوانی"), ("fi", "Līvānin kunta"), ("fr", "Līvāni"), ("gu", "લીવાની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लिवानी नगरपालिका"), ("hy", "Լիվանիի շրջան"), ("id", "Kotamadya Līvāni"), ("it", "Līvāni"), ("ja", "リーヴァーニ"), ("ka", "ლივანის მხარე"), ("kn", "ಲ\u{cbf}ವಾನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "리바니 시"), ("lt", "Lyvanų savivaldybė"), ("lv", "Līvānu novads"), ("mk", "Ливани"), ("mr", "लीवनि म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Livani municipality"), ("nb", "Livani kommune"), ("nl", "Līvānu novads"), ("no", "Livani kommune"), ("pl", "Gmina Līvāni"), ("pt", "Livani"), ("ru", "Ливанский край"), ("si", "ල\u{dd3}ව\u{dcf}න\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Līvānu novads"), ("ta", "லிவனி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c3f}వ\u{c3e}న\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลล\u{e34}วาน\u{e35}\u{e48}"), ("tr", "Livani Belediyesi"), ("uk", "Ліванський край"), ("ur", "لیوانی میونسپلٹی"), ("vi", "Đô thị tự trị Livani"), ("zh", "利瓦尼自治市")]),
                        unofficial_name_list: ["Līvāni"].to_vec(),
                    }
                ),
                (
                    "058",
                    Subdivision{
                        name: "058",
                        country_alpha2: Alpha2::LV,
                        code: "058",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لودزا"), ("be", "Лудзенскі край"), ("bn", "ল\u{9c1}দজ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Ludza"), ("ccp", "𑄣\u{1112a}𑄖\u{11134}𑄎"), ("ceb", "Lielais Ludzas Ezers"), ("da", "Ludza municipality"), ("de", "Bezirk Ludza"), ("el", "Λούντζα"), ("en", "Ludza"), ("es", "Municipalidad de Ludza"), ("et", "Ludza piirkond"), ("eu", "Ludza udalerria"), ("fa", "شهرداری لودزا"), ("fi", "Ludzan kunta"), ("fr", "Ludza"), ("gu", "લ\u{ac1}દઝા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ल\u{941}ड\u{94d}ज\u{93c}ा नगरपालिका"), ("id", "Kotamadya Ludza"), ("it", "Ludza"), ("ja", "ルヅァ"), ("ka", "ლუძის მხარე"), ("kn", "ಲುಡ\u{ccd}ಜಾ ಪುರಸಭ\u{cc6}"), ("ko", "루자 시"), ("lt", "Ludzos savivaldybė"), ("lv", "Ludzas novads"), ("mk", "Луѕа"), ("mr", "ल\u{942}डझ म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ludza municipality"), ("nb", "Ludza kommune"), ("nl", "Ludzas novads"), ("no", "Ludza kommune"), ("pl", "Gmina Lucyn"), ("pt", "Ludza"), ("ru", "Лудзенский край"), ("si", "ල\u{dd4}ඩ\u{dca}ස\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Ludza kommun"), ("ta", "லுடச\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "లుడ\u{c4d}జ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลล\u{e31}ดซา"), ("tr", "Ludza Belediyesi"), ("uk", "Лудзенський край"), ("ur", "لودزا میونسپلٹی"), ("vi", "Đô thị tự trị Ludza"), ("zh", "盧扎自治市")]),
                        unofficial_name_list: ["Ludza"].to_vec(),
                    }
                ),
                (
                    "059",
                    Subdivision{
                        name: "059",
                        country_alpha2: Alpha2::LV,
                        code: "059",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.8498701), longitude: Some(18.2723841), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مادونا"), ("be", "Маданскі край"), ("bn", "ম\u{9cd}য\u{9be}ডোন\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Madona"), ("ccp", "𑄟\u{11127}𑄓\u{1112e}𑄚"), ("ceb", "Madona Municipality"), ("da", "Madona municipality"), ("de", "Bezirk Madona"), ("el", "Μαντόνα"), ("en", "Madona"), ("es", "Municipalidad de Madona"), ("et", "Madona piirkond"), ("eu", "Madona udalerria"), ("fa", "شهرداری مادونا"), ("fi", "Madona"), ("fr", "Madona"), ("gu", "મડોના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{948}डोना नगरपालिका"), ("id", "Kotamadya Madona"), ("it", "Madona"), ("ja", "マドゥアナ"), ("ka", "მადონას მხარე"), ("kn", "ಮಡೋನಾ ಪುರಸಭ\u{cc6}"), ("ko", "마도나 시"), ("lt", "Maduonos savivaldybė"), ("lv", "Madonas novads"), ("mk", "Мадона"), ("mr", "म\u{945}डोना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Madona municipality"), ("nb", "Madona kommune"), ("nl", "Madonas novads"), ("no", "Madona kommune"), ("pl", "Gmina Madona"), ("pt", "Município de Madona"), ("ru", "Мадонский край"), ("si", "මැඩෝන\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Madonas novads"), ("ta", "மடோன\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మడ\u{c4b}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โดโบรวา โพลโฮว กราเดค"), ("tr", "Madona Belediyesi"), ("uk", "Мадонський край"), ("ur", "مادونا میونسپلٹی"), ("vi", "Đô thị tự trị Madona"), ("zh", "馬多納自治市")]),
                        unofficial_name_list: ["Madona"].to_vec(),
                    }
                ),
                (
                    "062",
                    Subdivision{
                        name: "062",
                        country_alpha2: Alpha2::LV,
                        code: "062",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماروب"), ("be", "Марупскі край"), ("bn", "ম\u{9be}র\u{9c1}প পৌরসভ\u{9be}"), ("ca", "Municipi de Mārupe"), ("ccp", "𑄟𑄢\u{1112a}𑄛\u{11134}"), ("ceb", "Mārupes Novads"), ("da", "Mārupe municipality"), ("de", "Bezirk Mārupe"), ("el", "Μάρουπε"), ("en", "Mārupe"), ("es", "Municipalidad de Mārupe"), ("et", "Mārupe piirkond"), ("eu", "Mārupe udalerria"), ("fa", "شهرداری ماروپه"), ("fi", "Mārupen kunta"), ("fr", "Mārupes novads"), ("gu", "મર\u{ac2}પ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मर\u{942}प नगरपालिका"), ("hy", "Մարուպեի շրջան"), ("id", "Kotamadya Mārupe"), ("it", "Mārupe"), ("ja", "マールペ"), ("ka", "მარუპეს მხარე"), ("kn", "ಮಾರಪ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "마루페 시"), ("lt", "Marupės savivaldybė"), ("lv", "Mārupes novads"), ("mk", "Марупе"), ("mr", "माप\u{942} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Marupe municipality"), ("nb", "Marupe kommune"), ("nl", "Mārupes novads"), ("no", "Marupe kommune"), ("pl", "Gmina Mārupe"), ("pt", "Município de Marupe"), ("ru", "Марупский край"), ("si", "ම\u{dcf}ර\u{dd4}පේ නගර සභ\u{dcf}ව"), ("sl", "Mārupe"), ("sv", "Mārupes novads"), ("ta", "மறுப\u{bcd}பே நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}రుప\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องมาร\u{e39}เป\u{e49}"), ("tr", "Mareupe Belediyesi"), ("uk", "Марупський край"), ("ur", "مروپی میونسپلٹی"), ("vi", "Đô thị tự trị Marupe"), ("zh", "馬魯皮斯自治市")]),
                        unofficial_name_list: ["Mārupe"].to_vec(),
                    }
                ),
                (
                    "067",
                    Subdivision{
                        name: "067",
                        country_alpha2: Alpha2::LV,
                        code: "067",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أوغري"), ("be", "Огрскі край"), ("bn", "ওগ\u{9cd}রে পৌরসভ\u{9be}"), ("ca", "Municipi d’Ogre"), ("ccp", "𑄃\u{11127}𑄉\u{11133}𑄢𑄬"), ("ceb", "Ogres novads"), ("da", "Ogre municipality"), ("de", "Bezirk Ogre"), ("el", "Όγκρε"), ("en", "Ogre"), ("es", "Municipalidad de Ogre"), ("et", "Ogre piirkond"), ("eu", "Ogre udalerria"), ("fa", "شهرداری اوگره"), ("fi", "Ogren kunta"), ("fr", "Ogre"), ("gu", "ઓગ\u{acd}ર\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ओग\u{94d}र\u{947} नगरपालिका"), ("hy", "Օգրեի շրջան"), ("id", "Kotamadya Ogre"), ("it", "Comune di Ogre"), ("ja", "ウアグレ"), ("ka", "ოგრეს მხარე"), ("kn", "ಓಗ\u{ccd}ರ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "오그레 시"), ("lt", "Uogrės savivaldybė"), ("lv", "Ogres novads"), ("mk", "Огре"), ("mr", "ओगरी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ogre municipality"), ("nb", "Ogre Kommune"), ("nl", "Ogres novads"), ("no", "Ogre Kommune"), ("pl", "Gmina Ogre"), ("pt", "Município de Ogre"), ("ru", "Огрский край"), ("si", "ඔග\u{dca}\u{200d}රේ නගර සභ\u{dcf}ව"), ("sv", "Ogres novads"), ("ta", "ஓக\u{bcd}கிறே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఓగ\u{c4d}ర\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโอเกร"), ("tr", "Ogre Belediyesi"), ("uk", "Огрський край"), ("ur", "وجری میونسپلٹی"), ("vi", "Đô thị tự trị Ogre"), ("zh", "奧格雷自治市")]),
                        unofficial_name_list: ["Ogre"].to_vec(),
                    }
                ),
                (
                    "068",
                    Subdivision{
                        name: "068",
                        country_alpha2: Alpha2::LV,
                        code: "068",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أولاين"), ("be", "Олайнскі край"), ("bn", "ওল\u{9be}ইন পৌরসভ\u{9be}"), ("ca", "Municipi d’Olaine"), ("ccp", "𑄃\u{1112e}𑄣\u{1112d}𑄚\u{11134}"), ("da", "Olaine municipality"), ("de", "Bezirk Olaine"), ("el", "Ολαίνε"), ("en", "Olaine"), ("es", "Municipalidad de Olaine"), ("et", "Olaine piirkond"), ("eu", "Olaine udalerria"), ("fa", "شهرداری اولاین"), ("fi", "Olainen kunta"), ("fr", "Olaine"), ("gu", "ઓલાઇન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ओल\u{947}न नगरपालिका"), ("hy", "Օլաինեի շրջան"), ("id", "Kotamadya Olaine"), ("it", "Comune di Olaine"), ("ja", "ウアライネ"), ("ka", "ოლაინეს მხარე"), ("kn", "ಒಲೇನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "올라이네 시"), ("lt", "Uolainės savivaldybė"), ("lv", "Olaines novads"), ("mk", "Олаине"), ("mr", "ओलाईन\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Olaine municipality"), ("nb", "Olaine kommune"), ("nl", "Olaines novads"), ("no", "Olaine kommune"), ("pl", "Gmina Olaine"), ("pt", "Município de Olaine"), ("ru", "Олайнский край"), ("si", "ඔලය\u{dca}නේ නගර සභ\u{dcf}ව"), ("sv", "Olaine kommun"), ("ta", "ஓலைனே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఓల\u{c48}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดนายาลา"), ("tr", "Olaine Belediyesi"), ("uk", "Олайнський край"), ("ur", "ولاینی میونسپلٹی"), ("vi", "Đô thị tự trị Olaine"), ("zh", "奧萊內自治市")]),
                        unofficial_name_list: ["Olaine"].to_vec(),
                    }
                ),
                (
                    "073",
                    Subdivision{
                        name: "073",
                        country_alpha2: Alpha2::LV,
                        code: "073",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريلي"), ("be", "Прэйльскі край"), ("bn", "প\u{9cd}রেইলি পৌরসভ\u{9be}"), ("ca", "Municipi de Preiļi"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄣\u{11128}"), ("ceb", "Preiļi Municipality"), ("da", "Preiļi"), ("de", "Bezirk Preiļi"), ("el", "Κοινότητα Πρέιλι"), ("en", "Preiļi"), ("es", "Municipalidad de Preiļi"), ("et", "Preiļi piirkond"), ("eu", "Preiļi udalerria"), ("fa", "شهرداری پریلی"), ("fi", "Preiļin kunta"), ("fr", "Preiļi"), ("gu", "પ\u{acd}ર\u{ac7}લી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}रीली नगरपालिका"), ("id", "Kotamadya Preiļi"), ("it", "Preiļi"), ("ja", "プレイリ"), ("ka", "პრეილის მხარე"), ("kn", "ಪ\u{ccd}ರೈಲ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "프레일리 시"), ("lt", "Preilių savivaldybė"), ("lv", "Preiļu novads"), ("mk", "Преиљи"), ("mr", "प\u{94d}र\u{947}इ म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Preiļi"), ("nb", "Preili kommune"), ("nl", "Preiļu novads"), ("no", "Preili kommune"), ("pl", "Gmina Preiļi"), ("pt", "Município de Preili"), ("ru", "Прейльский край"), ("si", "ප\u{dca}\u{200d}රේය\u{dd2}ල\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Preiļi Municipality"), ("ta", "பிரெய\u{bcd}லி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c40}ల\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เขตเทศบาล พรายล\u{e34}"), ("tr", "Preili Belediyesi"), ("uk", "Прейльський край"), ("ur", "میونسپلٹی"), ("vi", "Đô Thị Tự Trị Preili"), ("zh", "普雷利自治市")]),
                        unofficial_name_list: ["Preiļi"].to_vec(),
                    }
                ),
                (
                    "077",
                    Subdivision{
                        name: "077",
                        country_alpha2: Alpha2::LV,
                        code: "077",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ريزكنه"), ("be", "Рэзэкненскі край"), ("bn", "রেজেকনি পৌরসভ\u{9be}"), ("ca", "Municipi de Rēzekne"), ("ccp", "𑄢𑄬𑄎𑄬𑄇\u{11134}𑄚\u{11128} 𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄥\u{11128}𑄛𑄣\u{11128}𑄑\u{11128}"), ("ceb", "Rēzeknes Novads"), ("da", "Rēzekne municipality"), ("de", "Bezirk Rēzekne"), ("el", "Ρέζεκνε"), ("en", "Rēzekne Municipality"), ("es", "Municipalidad de Rēzekne"), ("et", "Rēzekne piirkond"), ("eu", "Rēzekne udalerria"), ("fa", "شهرداری رزکنه"), ("fi", "Rēzeknen kunta"), ("fr", "Rēzekne"), ("gu", "રીજ\u{ac7}ક\u{acd}ન\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "र\u{947}ज\u{93c}\u{947}क\u{94d}न\u{947} नगरपालिका"), ("id", "Kotamadya Rēzekne"), ("it", "Rēzekne"), ("ja", "レーゼクネ"), ("ka", "რეზეკნეს მხარე"), ("kn", "ರೇಜ\u{cc6}ನ\u{ccd}ನ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "레제크네 시"), ("lt", "Rėzeknės savivaldybė"), ("lv", "Rēzeknes novads"), ("mk", "Резекне"), ("mr", "र\u{947}ज\u{947}न\u{94d}न\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Rezekne municipality"), ("nb", "Rezekne kommune"), ("nl", "Rēzeknes novads"), ("no", "Rezekne kommune"), ("pl", "Gmina Rzeżyca"), ("pt", "Município de Rezekne"), ("ru", "Резекненский край"), ("si", "රේසේන\u{dca}ක\u{dca}නේ නගර සභ\u{dcf}ව"), ("sv", "Rēzeknes Novads"), ("ta", "ரெஸிக\u{bcd}னே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3f}జ\u{c46}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเรเซคเน"), ("tr", "Rezekne Belediyesi"), ("uk", "Резекненський край"), ("ur", "ریزیکنی میونسپلٹی"), ("vi", "Đô thị tự trị Rezekne"), ("zh", "雷澤克內自治市")]),
                        unofficial_name_list: ["Rēzekne"].to_vec(),
                    }
                ),
                (
                    "080",
                    Subdivision{
                        name: "080",
                        country_alpha2: Alpha2::LV,
                        code: "080",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية روباجي"), ("be", "Ропажскі край"), ("bn", "রোপ\u{9be}জ\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Ropaži"), ("ccp", "𑄢\u{1112e}𑄛𑄎\u{11128}"), ("ceb", "Ropažu Novads"), ("da", "Ropaži municipality"), ("de", "Bezirk Ropaži"), ("el", "Ροπάζι"), ("en", "Ropaži"), ("es", "Municipalidad de Ropaži"), ("et", "Ropaži piirkond"), ("eu", "Ropaži udalerria"), ("fa", "شهرداری روپاژی"), ("fi", "Ropažin kunta"), ("fr", "Ropaži"), ("gu", "રોપાજી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रोपाजी नगरपालिका"), ("id", "Kotamadya Ropaži"), ("it", "Ropaži"), ("ja", "ルアパジ"), ("ka", "როპაჟის მხარე"), ("kn", "ರೊಪಾಝ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "로파지 시"), ("lt", "Ruopažų savivaldybė"), ("lv", "Ropažu novads"), ("mk", "Ропажи"), ("mr", "रोपाजी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ropazi municipality"), ("nb", "Ropazi kommune"), ("nl", "Ropažu novads"), ("no", "Ropazi kommune"), ("pl", "Gmina Ropaži"), ("pt", "Município de Ropazi"), ("ru", "Ропажский край"), ("si", "රෝපස\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Ropažu novads"), ("ta", "ரோபஸ\u{bcd}ய\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c4b}ప\u{c3e}జ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโรปาซ\u{e35}"), ("tr", "Ropazi Belediyesi"), ("uk", "Ропажський край"), ("ur", "روپازی میونسپلٹی"), ("vi", "Đô thị tự trị Ropazi"), ("zh", "羅帕日自治市")]),
                        unofficial_name_list: ["Ropaži"].to_vec(),
                    }
                ),
                (
                    "087",
                    Subdivision{
                        name: "087",
                        country_alpha2: Alpha2::LV,
                        code: "087",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سالاسبيلز"), ("be", "Саласпілскі край"), ("bn", "স\u{9be}ল\u{9be}স\u{9cd}পিলস পৌরসভ\u{9be}"), ("ca", "Municipi de Salaspils"), ("ccp", "𑄥𑄣𑄌\u{11134}𑄛\u{11128}𑄣\u{11134}𑄥\u{11134}"), ("ceb", "Salaspils Novads"), ("da", "Salaspils municipality"), ("de", "Bezirk Salaspils"), ("el", "Σάλασπιλς"), ("en", "Salaspils"), ("es", "Municipalidad de Salaspils"), ("et", "Salaspilsi piirkond"), ("eu", "Salaspils udalerria"), ("fa", "شهرداری سالاسپیلس"), ("fi", "Salaspilsin kunta"), ("fr", "Salaspils"), ("gu", "સાલાસ\u{acd}પિલ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सलासपिल\u{94d}स नगर पालिका"), ("hy", "Սալասպիլսի շրջան"), ("id", "Kotamadya Salaspils"), ("it", "Comune di Salaspils"), ("ja", "サラスピルス"), ("ka", "სალასპილსის მხარე"), ("kn", "ಸಲಾಸ\u{ccd}ಪೈಲ\u{ccd}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "살라스필스 시"), ("lt", "Salaspilio savivaldybė"), ("lv", "Salaspils novads"), ("mk", "Саласпилс"), ("mr", "सालास\u{94d}पिलस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Salaspils municipality"), ("nb", "Salaspils Kommune"), ("nl", "Salaspils novads"), ("no", "Salaspils Kommune"), ("pl", "Gmina Salaspils"), ("pt", "Município de Salaspils"), ("ru", "Саласпилсский край"), ("si", "සලප\u{dd2}ල\u{dca}ස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Salaspils Novads"), ("ta", "சலசபிள\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c3e}ల\u{c3e}స\u{c4d}\u{200c}ప\u{c3f}ల\u{c4d}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องซาลาสป\u{e34}ลส\u{e4c}"), ("tr", "Salaspils Belediyesi"), ("uk", "Саласпілський край"), ("ur", "سالاسپیلس میونسپلٹی"), ("vi", "Đô thị tự trị Salaspils"), ("zh", "薩拉斯皮爾斯自治市")]),
                        unofficial_name_list: ["Salaspils"].to_vec(),
                    }
                ),
                (
                    "088",
                    Subdivision{
                        name: "088",
                        country_alpha2: Alpha2::LV,
                        code: "088",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.6653019), longitude: Some(22.492169), max_latitude: Some(56.6855229), min_latitude: Some(56.6456732), max_longitude: Some(22.5252327), min_longitude: Some(22.4596109)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سالدوس"), ("be", "Салдускі край"), ("bn", "স\u{9cd}য\u{9be}লদ\u{9c1}স পৌরসভ\u{9be}"), ("ca", "Municipi de Saldus"), ("ccp", "𑄥𑄣\u{11134}𑄓𑄌\u{11134}"), ("ceb", "Saldus Municipality"), ("da", "Saldus Municipality"), ("de", "Bezirk Saldus"), ("el", "Σάλντους"), ("en", "Saldus"), ("es", "Municipalidad de Saldus"), ("et", "Salduse piirkond"), ("eu", "Saldus udalerria"), ("fa", "شهرداری سالدوس"), ("fi", "Saldusin kunta"), ("fr", "Saldus"), ("gu", "સાલડસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सल\u{94d}द\u{941}स नगरपालिका"), ("hy", "Սալդուսի շրջան"), ("id", "Kotamadya Saldus"), ("it", "Saldus"), ("ja", "サルドゥス"), ("ka", "სალდუსის მხარე"), ("kn", "ಸಲ\u{ccd}ಡಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "살두스 시"), ("lt", "Saldaus savivaldybė"), ("lv", "Saldus novads"), ("mk", "Салдус"), ("mr", "सालडस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Saldus Municipality"), ("nb", "Saldus kommune"), ("nl", "Saldus novads"), ("no", "Saldus kommune"), ("pl", "Gmina Saldus"), ("pt", "Município de Saldus"), ("ru", "Салдусский край"), ("si", "සල\u{dca}දස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Saldus novads"), ("ta", "ச\u{bbe}ல\u{bcd}டஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c3e}ల\u{c4d}డస\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องซาลด\u{e31}ส"), ("tr", "Saldus Belediyesi"), ("uk", "Салдуський край"), ("ur", "سالدوس میونسپلٹی"), ("vi", "Đô thị tự trị Saldus"), ("zh", "薩爾杜斯自治市")]),
                        unofficial_name_list: ["Saldus"].to_vec(),
                    }
                ),
                (
                    "089",
                    Subdivision{
                        name: "089",
                        country_alpha2: Alpha2::LV,
                        code: "089",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سولكراستي"), ("be", "Саўлкрасцкі край"), ("bn", "সল\u{9cd}ক\u{9cd}র\u{9be}স\u{9cd}তি পৌরসভ\u{9be}"), ("ca", "Municipi de Saulkrasti"), ("ccp", "𑄥\u{1112f}𑄇\u{11134}𑄇\u{11133}𑄢𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Saulkrastu Novads"), ("da", "Saulkrasti municipality"), ("de", "Bezirk Saulkrasti"), ("el", "Σολκράστι"), ("en", "Saulkrasti"), ("es", "Municipalidad de Saulkrasti"), ("et", "Saulkrasti piirkond"), ("eu", "Saulkrasti udalerria"), ("fa", "شهرداری ساولکراستی"), ("fi", "Saulkrastin kunta"), ("fr", "Saulkrasti"), ("gu", "સૌલ\u{acd}ક\u{acd}રસ\u{acd}ટી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सौलक\u{94d}रास\u{94d}ती नगरपालिका"), ("id", "Kotamadya Saulkrasti"), ("it", "Saulkrasti"), ("ja", "サウルクラスチ"), ("ka", "საულკრასტის მხარე"), ("kn", "ಸ\u{ccc}ಲ\u{ccd}ಕ\u{ccd}ರಾಸ\u{ccd}ಟ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "사울크라스티 시"), ("lt", "Saulkrastų savivaldybė"), ("lv", "Saulkrastu novads"), ("mk", "Саулкрасти"), ("mr", "शौलक\u{94d}रस\u{94d}ती म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Saulkrasti municipality"), ("nb", "Saulkrasi kommune"), ("nl", "Saulkrastu novads"), ("no", "Saulkrasi kommune"), ("pl", "Gmina Saulkrasti"), ("pt", "Município de Saulkrasti"), ("ru", "Саулкрастский край"), ("si", "සෞල\u{dca}ක\u{dca}රස\u{dca}ට\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Saulkrastu novads"), ("ta", "ஸுலக\u{bcd}ரஸ\u{bcd}ட\u{bcd}டி நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4c}ల\u{c4d}క\u{c4d}ర\u{c3e}స\u{c4d}ట\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องเซาคราสต\u{e34}"), ("tr", "Saulkrasti Belediyesi"), ("uk", "Саулкрастський край"), ("ur", "ساولکراستی میونسپلٹی"), ("vi", "Đô thị tự trị Saulkrasti"), ("zh", "薩烏爾克拉斯蒂自治市")]),
                        unofficial_name_list: ["Saulkrasti"].to_vec(),
                    }
                ),
                (
                    "091",
                    Subdivision{
                        name: "091",
                        country_alpha2: Alpha2::LV,
                        code: "091",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيغولدا"), ("be", "Сігулдскі край"), ("bn", "সিগ\u{9c1}ল\u{9cd}ড\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Sigulda"), ("ccp", "𑄥\u{11128}𑄉\u{1112a}𑄣\u{11134}𑄓"), ("ceb", "Siguldas Novads"), ("da", "Sigulda municipality"), ("de", "Bezirk Sigulda"), ("el", "Σιγκούλντα"), ("en", "Sigulda"), ("es", "Municipalidad de Sigulda"), ("et", "Sigulda piirkond"), ("eu", "Sigulda udalerria"), ("fa", "شهرداری سیگولدا"), ("fi", "Siguldan kunta"), ("fr", "Sigulda"), ("gu", "સિગ\u{ac1}લ\u{acd}ડા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सिग\u{941}लडा नगरपालिका"), ("id", "Kotamadya Sigulda"), ("it", "Sigulda"), ("ja", "スィグルダ"), ("ka", "სიგულდის მხარე"), ("kn", "ಸ\u{cbf}ಗುಲ\u{ccd}ಡಾ ಪುರಸಭ\u{cc6}"), ("ko", "시굴다 시"), ("lt", "Siguldos savivaldybė"), ("lv", "Siguldas novads"), ("mk", "Сигулда"), ("mr", "सिघ\u{941}ल\u{94d}ड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sigulda municipality"), ("nb", "Sigulda Kommune"), ("nl", "Siguldas novads"), ("no", "Sigulda Kommune"), ("pl", "Gmina Sigulda"), ("pt", "Município de Sigulda"), ("ru", "Сигулдский край"), ("si", "ස\u{dd2}ග\u{dd4}ල\u{dca}ඩ\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Siguldas novads"), ("ta", "சிகுலட\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c3f}గుల\u{c4d}ద\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ซ\u{e34}ก\u{e38}ลดา"), ("tr", "Sigulda Belediyesi"), ("uk", "Сігулдський край"), ("ur", "سیجولدا میونسپلٹی"), ("vi", "Đô thị tự trị Sigulda"), ("zh", "錫古爾達自治市")]),
                        unofficial_name_list: ["Sigulda"].to_vec(),
                    }
                ),
                (
                    "094",
                    Subdivision{
                        name: "094",
                        country_alpha2: Alpha2::LV,
                        code: "094",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سميلتين"), ("be", "Смілтэнскі край"), ("bn", "স\u{9cd}মিল\u{9cd}টেন পৌরসভ\u{9be}"), ("ca", "Municipi de Smiltene"), ("ccp", "𑄌\u{11133}𑄟\u{1112d}𑄣\u{11134}𑄑𑄬𑄚𑄬"), ("ceb", "Smiltenes Novads"), ("da", "Smiltene municipality"), ("de", "Bezirk Smiltene"), ("el", "Σμιλτένε"), ("en", "Smiltene"), ("es", "Municipalidad de Smiltene"), ("et", "Smiltene piirkond"), ("eu", "Smiltene udalerria"), ("fa", "شهرداری اسمیلتن"), ("fi", "Smiltenen kunta"), ("fr", "Smiltene"), ("gu", "સ\u{acd}મિલટ\u{ac7}ન\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}मिल\u{94d}टीन नगरपालिका"), ("hy", "Սմիլտենեի շրջան"), ("id", "Kotamadya Smiltene"), ("it", "Smiltene"), ("ja", "スミルテネ"), ("ka", "სმილტენეს მხარე"), ("kn", "ಸ\u{ccd}ಮ\u{cbf}ಲ\u{cc6}ನ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "스밀테네 시"), ("lt", "Smiltenės savivaldybė"), ("lv", "Smiltenes novads"), ("mk", "Смилтене"), ("mr", "स\u{94d}मिल\u{94d}डीन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Smiltene municipality"), ("nb", "Smiltene kommune"), ("nl", "Smiltenes novads"), ("no", "Smiltene kommune"), ("pl", "Gmina Smiltene"), ("pt", "Município de Smiltene"), ("ru", "Смилтенский край"), ("si", "ස\u{dca}ම\u{dd2}ල\u{dca}ටෙනේ නගර සභ\u{dcf}ව"), ("sv", "Smiltenes novads"), ("ta", "ஸ\u{bcd}மைல\u{bcd}ட\u{bcd}டேனே நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}మ\u{c3f}ల\u{c4d}ట\u{c40}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องสม\u{e34}ลเตเน"), ("tr", "Smiltene Belediyesi"), ("uk", "Смілтенський край"), ("ur", "سمیلتینی میونسپلٹی"), ("vi", "Đô thị tự trị Smiltene"), ("zh", "斯米爾泰內自治市")]),
                        unofficial_name_list: ["Smiltene"].to_vec(),
                    }
                ),
                (
                    "097",
                    Subdivision{
                        name: "097",
                        country_alpha2: Alpha2::LV,
                        code: "097",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.2480409), longitude: Some(22.5873859), max_latitude: Some(57.265104), min_latitude: Some(57.2275979), max_longitude: Some(22.6215941), min_longitude: Some(22.5537861)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تالشي"), ("be", "Талсінскі край"), ("bn", "ত\u{9be}লসি পৌরসভ\u{9be}"), ("ca", "Municipi de Talsi"), ("ccp", "𑄑𑄣\u{11134}𑄥\u{11128}"), ("ceb", "Talsi Municipality"), ("da", "Talsi municipality"), ("de", "Bezirk Talsi"), ("el", "Τάλσι"), ("en", "Talsi"), ("es", "Municipalidad de Talsi"), ("et", "Talsi piirkond"), ("eu", "Talsi udalerria"), ("fa", "شهرداری تالسی"), ("fi", "Talsin kunta"), ("fr", "Municipalité de Talsi"), ("gu", "તાલ\u{acd}સી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "टाल\u{94d}सी नगरपालिका"), ("hy", "Տալսիի շրջան"), ("id", "Kotamadya Talsi"), ("it", "Talsi"), ("ja", "タルスィ"), ("ka", "ტალსის მხარე"), ("kn", "ಟಾಲ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "탈시 시"), ("lt", "Talsų savivaldybė"), ("lv", "Talsu novads"), ("mk", "Талси"), ("mr", "तल\u{94d}सी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Talsi municipality"), ("nb", "Talsi kommune"), ("nl", "Talsu novads"), ("no", "Talsi kommune"), ("pl", "Gmina Talsi"), ("pt", "Município de Talsi"), ("ru", "Талсинский край"), ("si", "ටල\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Talsu novads"), ("ta", "டல\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c3e}ల\u{c4d}స\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องทาสซ\u{e34}"), ("tr", "Talsi Belediyesi"), ("uk", "Талсінський край"), ("ur", "تالسی میونسپلٹی"), ("vi", "Đô thị tự trị Talsi"), ("zh", "塔爾西自治市")]),
                        unofficial_name_list: ["Talsi"].to_vec(),
                    }
                ),
                (
                    "099",
                    Subdivision{
                        name: "099",
                        country_alpha2: Alpha2::LV,
                        code: "099",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.96966399999999), longitude: Some(23.15319), max_latitude: Some(56.9880529), min_latitude: Some(56.9496259), max_longitude: Some(23.205056), min_longitude: Some(23.1016533)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية توكومز"), ("be", "Тукумскі край"), ("bn", "ট\u{9c1}ক\u{9c1}মস পৌরসভ\u{9be}"), ("ca", "Municipi de Tukums"), ("ccp", "𑄑\u{1112a}𑄇\u{1112a}𑄟\u{11134}𑄌\u{11134}"), ("cs", "Tukums"), ("da", "Tukums Municipality"), ("de", "Bezirk Tukums"), ("el", "Τουκούμς"), ("en", "Tukums"), ("es", "Municipalidad de Tukums"), ("et", "Tukumsi piirkond"), ("eu", "Tukums udalerria"), ("fa", "شهرداری توکومس"), ("fi", "Tukumsin kunta"), ("fr", "Tukums"), ("gu", "ટ\u{ac1}ક\u{ac1}મ\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("he", "טוקומס"), ("hi", "त\u{941}क\u{941}म\u{94d}स नगरपालिका"), ("hu", "Tukums község"), ("hy", "Տուկումայի շրջան"), ("id", "Kotamadya Tukums"), ("it", "Tukums"), ("ja", "トゥクムス"), ("ka", "ტუკუმსის მხარე"), ("kn", "ತುಕಮ\u{ccd}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "투쿰스 시"), ("lt", "Tukumo savivaldybė"), ("lv", "Tukuma novads"), ("mk", "Тукумс"), ("mr", "त\u{941}म म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Tukums Municipality"), ("nb", "Tukums kommune"), ("nl", "Tukuma novads"), ("no", "Tukums kommune"), ("pl", "Gmina Tukums"), ("pt", "Tukums"), ("ru", "Тукумсский край"), ("si", "ට\u{dd4}ක\u{dd4}ම\u{dca}ස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Tukums kommun"), ("ta", "டுக\u{bcd}கும\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "టుకుమ\u{c4d}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องต\u{e39}ก\u{e38}มส\u{e4c}"), ("tr", "Tukums Belediyesi"), ("uk", "Тукумський край"), ("ur", "توکومس میونسپلٹی"), ("vi", "Đô thị tự trị Tukums"), ("zh", "圖庫姆斯自治市")]),
                        unofficial_name_list: ["Tukums"].to_vec(),
                    }
                ),
                (
                    "101",
                    Subdivision{
                        name: "101",
                        country_alpha2: Alpha2::LV,
                        code: "101",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فالكا"), ("be", "Валцкі край"), ("bn", "ভল\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipi de Valka"), ("ccp", "𑄞𑄣\u{11134}𑄇"), ("ceb", "Valka Municipality"), ("da", "Valka municipality"), ("de", "Bezirk Valka"), ("el", "Βάλκα"), ("en", "Valka"), ("es", "Municipalidad de Valka"), ("et", "Valka piirkond"), ("eu", "Valka udalerria"), ("fa", "شهرداری والکا"), ("fi", "Valkan kunta"), ("fr", "Valka"), ("gu", "વાલ\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "वाल\u{94d}का नगरपालिका"), ("hy", "Վալկայի շրջան"), ("id", "Kotamadya Valka"), ("it", "Comune di Valka"), ("ja", "ヴァルカ"), ("ka", "ვალკის მხარე"), ("kn", "ವಲ\u{ccd}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "발카 시"), ("lt", "Valkos savivaldybė"), ("lv", "Valkas novads"), ("mk", "Валка"), ("mr", "वल\u{94d}का म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Valka municipality"), ("nb", "Valka kommune"), ("nl", "Valkas novads"), ("no", "Valka kommune"), ("pl", "Gmina Valka"), ("pt", "Município de Valka"), ("ru", "Валкский край"), ("si", "වලක\u{dcf} නගර සභ\u{dcf}ව"), ("sv", "Valka Municipality"), ("ta", "வெல\u{bcd}க நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3e}ల\u{c4d}క మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องว\u{e31}ลก\u{e49}า"), ("tr", "Valka Belediyesi"), ("uk", "Валкський край"), ("ur", "والکا میونسپلٹی"), ("vi", "Đô thị tự trị Valka"), ("zh", "瓦爾加自治市")]),
                        unofficial_name_list: ["Valka"].to_vec(),
                    }
                ),
                (
                    "102",
                    Subdivision{
                        name: "102",
                        country_alpha2: Alpha2::LV,
                        code: "102",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فاراكاياني"), ("be", "Вараклянскі край"), ("bn", "ভ\u{9be}র\u{9be}ক\u{9cd}ল\u{9be}নি পৌরসভ\u{9be}"), ("ca", "Municipi de Varakļāni"), ("ccp", "𑄞𑄢𑄇\u{11134}𑄣𑄚\u{11128}"), ("ceb", "Varakļānu Novads"), ("da", "Varakļāni municipality"), ("de", "Bezirk Varakļāni"), ("el", "Βαρακλάνι"), ("en", "Varakļāni"), ("es", "Municipalidad de Varakļāni"), ("et", "Varakļāni piirkond"), ("eu", "Varakļāni udalerria"), ("fa", "واراکلانی مونیکیپلیتی"), ("fi", "Varakļānin kunta"), ("fr", "Varakļāni"), ("gu", "વરાકલાની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "वरक\u{94d}लानी नगरपालिका"), ("hy", "Վարակլյանիի շրջան"), ("id", "Kotamadya Varakļāni"), ("it", "Varakļāni"), ("ja", "ヴァラクリャーニ"), ("ka", "ვარაკლიანის მხარე"), ("kn", "ವರಾಕ\u{ccd}ಲಾನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "바라클랴니 시"), ("lt", "Varaklianų savivaldybė"), ("lv", "Varakļānu novads"), ("mk", "Варакљани"), ("mr", "वरकणी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Varaklani municipality"), ("nb", "Varaklani kommune"), ("nl", "Varakļānu novads"), ("no", "Varaklani kommune"), ("pl", "Gmina Varakļāni"), ("pt", "Município de Varaklani"), ("ru", "Вараклянский край"), ("si", "වරක\u{dca}ල\u{dcf}න\u{dd2} නගර සභ\u{dcf}ව"), ("sv", "Varakļānu novads"), ("ta", "வரக\u{bcd}கனி நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3e}రక\u{c4d}ల\u{c3e}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องวาร\u{e31}คลาน\u{e35}"), ("tr", "Varakjani Belediyesi"), ("uk", "Вараклянський край"), ("ur", "واراکلانی میونسپلٹی"), ("vi", "Đô thị tự trị Varaklani"), ("zh", "瓦拉克利亞尼自治市")]),
                        unofficial_name_list: ["Varakļāni"].to_vec(),
                    }
                ),
                (
                    "106",
                    Subdivision{
                        name: "106",
                        country_alpha2: Alpha2::LV,
                        code: "106",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.3937216), longitude: Some(21.5647066), max_latitude: Some(57.47072199999999), min_latitude: Some(57.34809389999999), max_longitude: Some(21.6736649), min_longitude: Some(21.5157259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فينتسبيلس"), ("be", "Вэнтспілскі край"), ("bn", "ভেন\u{9cd}টস\u{9cd}পিল পৌরসভ\u{9be}"), ("ca", "Municipi de Ventspils"), ("ccp", "𑄞𑄬𑄚\u{11134}𑄑\u{11134}𑄛\u{11128}𑄣\u{11134}𑄥\u{11134} 𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄥\u{11128}𑄛𑄣\u{11128}𑄑\u{11128}"), ("ceb", "Ventspils Municipality"), ("da", "Ventspils municipality"), ("de", "Bezirk Ventspils"), ("el", "Βέντσπιλς"), ("en", "Ventspils Municipality"), ("es", "Municipalidad de Ventspils"), ("et", "Ventspilsi piirkond"), ("eu", "Ventspils udalerria"), ("fa", "ونتسپیلس مونیکیپلیتی"), ("fi", "Ventspilsin kunta"), ("fr", "Ventspils"), ("gu", "વ\u{ac7}ન\u{acd}ટ\u{acd}સપિલ\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{947}\u{902}टस\u{94d}पिल\u{94d}स नगरपालिका"), ("hy", "Վենտսպիլսի շրջան"), ("id", "Kotamadya Ventspils"), ("it", "Ventspils"), ("ja", "ヴェンツピルス"), ("ka", "ვენტსპილსის მხარე"), ("kn", "ವ\u{cc6}ಂಟ\u{ccd}ಸ\u{ccd}ಪ\u{cbf}ಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "벤츠필스 시"), ("lt", "Ventspilio savivaldybė"), ("lv", "Ventspils novads"), ("mk", "Вентспилс"), ("mr", "व\u{947}न\u{94d}टस\u{94d}पिलस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ventspils municipality"), ("nb", "Ventspils kommune"), ("nl", "Ventspils novads"), ("no", "Ventspils kommune"), ("pl", "Gmina Windawa"), ("pt", "Município de Ventspils"), ("ru", "Вентспилсский край"), ("si", "වෙන\u{dca}ට\u{dca}ස\u{dca}ප\u{dd2}ල\u{dca}ස\u{dca} නගර සභ\u{dcf}ව"), ("sv", "Ventspils novads"), ("ta", "வென\u{bcd}டஸிபிள\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c46}ంట\u{c4d}స\u{c4d}\u{200c}ప\u{c3f}ల\u{c4d}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เวนต\u{e4c}สป\u{e34}ลส\u{e4c}"), ("tr", "Ventspils Belediyesi"), ("uk", "Вентспілський край"), ("ur", "وینتسپیلس میونسپلٹی"), ("vi", "Đô thị tự trị Ventspils"), ("zh", "文茨皮爾斯自治市")]),
                        unofficial_name_list: ["Ventspils"].to_vec(),
                    }
                ),
                (
                    "111",
                    Subdivision{
                        name: "111",
                        country_alpha2: Alpha2::LV,
                        code: "111",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Augšdaugavas novads"), ("lv", "Augšdaugavas novads")]),
                        unofficial_name_list: ["Augšdaugava"].to_vec(),
                    }
                ),
                (
                    "112",
                    Subdivision{
                        name: "112",
                        country_alpha2: Alpha2::LV,
                        code: "112",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Dienvidkurzemes Novads"), ("lv", "Dienvidkurzemes Novads")]),
                        unofficial_name_list: ["Dienvidkurzeme"].to_vec(),
                    }
                ),
                (
                    "113",
                    Subdivision{
                        name: "113",
                        country_alpha2: Alpha2::LV,
                        code: "113",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Valmieras Novads"), ("lv", "Valmieras Novads")]),
                        unofficial_name_list: ["Valmiera"].to_vec(),
                    }
                ),
                (
                    "DGV",
                    Subdivision{
                        name: "DGV",
                        country_alpha2: Alpha2::LV,
                        code: "DGV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.874736), longitude: Some(26.536179), max_latitude: Some(55.955541), min_latitude: Some(55.836144), max_longitude: Some(26.6361371), min_longitude: Some(26.438294)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Daugavpils"), ("ar", "داوغافبيلس"), ("az", "Dauqavpils"), ("be", "Даўгаўпілс"), ("bg", "Даугавпилс"), ("bn", "দ গ ভ\u{9c1}লস"), ("ca", "Daugavpils"), ("ccp", "𑄓\u{1112f}𑄉𑄛\u{11134}𑄛\u{11128}𑄣\u{11134}𑄥\u{11134}"), ("ceb", "Daugavpils"), ("cs", "Daugavpils"), ("cy", "Daugavpils"), ("da", "Daugavpils"), ("de", "Daugavpils"), ("el", "Νταουγκάβπιλς"), ("en", "Daugavpils"), ("es", "Daugavpils"), ("et", "Daugavpils"), ("eu", "Daugavpils"), ("fa", "دوگاپیلس"), ("fi", "Daugavpils"), ("fr", "Daugavpils"), ("gl", "Daugavpils"), ("gu", "ડૌગ\u{ac7}વપિલ\u{acd}સ"), ("he", "דאוגבפילס"), ("hi", "डौगाव\u{94d}पिल\u{94d}स"), ("hr", "Daugavpils"), ("hu", "Daugavpils"), ("hy", "Դաուգավպիլս"), ("id", "Daugavpils"), ("it", "Daugavpils"), ("ja", "ダウガフピルス²"), ("ka", "დაუგავპილსი"), ("kk", "Даугавпилс"), ("kn", "ದಾವ\u{ccd}ಗವ\u{ccd}ಪ\u{cbf}ಲ\u{ccd}ಸ\u{ccd}"), ("ko", "다우가프필스"), ("lt", "Daugpilis"), ("lv", "Daugavpils"), ("mk", "Даугавпилс"), ("mr", "दौगौपिल\u{94d}स"), ("ms", "Daugavpils"), ("nb", "Daugavpils"), ("nl", "Daugavpils"), ("no", "Daugavpils"), ("pl", "Dyneburg"), ("pt", "Daugavpils"), ("ro", "Daugavpils"), ("ru", "Даугавпилс"), ("si", "ඩොගව\u{dca}ප\u{dd2}ල\u{dca}ස\u{dca}"), ("sk", "Daugavpils"), ("sl", "Daugavpils"), ("sq", "Daugavpils"), ("sr", "Даугавпилс"), ("sr_Latn", "Daugavpils"), ("sv", "Daugavpils"), ("ta", "டக\u{bbe}வபிள\u{bcd}ஸ\u{bcd}"), ("te", "డ\u{c4c}గ\u{c3e}వ\u{c4d}\u{200c}ప\u{c4d}ల\u{c3f}స\u{c4d}"), ("th", "เดาก\u{e31}ฟป\u{e34}ลส\u{e4c}"), ("tr", "Daugavpils"), ("uk", "Даугавпілс"), ("ur", "داوگاوپلس"), ("uz", "Daugavpils"), ("vi", "Daugavpils"), ("zh", "陶格夫匹尔斯")]),
                        unofficial_name_list: ["Daugavpils"].to_vec(),
                    }
                ),
                (
                    "JEL",
                    Subdivision{
                        name: "JEL",
                        country_alpha2: Alpha2::LV,
                        code: "JEL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.6511091), longitude: Some(23.7213541), max_latitude: Some(56.6916349), min_latitude: Some(56.596751), max_longitude: Some(23.8002769), min_longitude: Some(23.623268)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jelgava"), ("ar", "جيلغافا"), ("az", "Yelqava"), ("be", "Елгава"), ("bg", "Йелгава"), ("bn", "জেল\u{9cd}গ\u{9be}ভ\u{9be}"), ("ca", "Jelgava"), ("ccp", "𑄎𑄬𑄣\u{11134}𑄉𑄞"), ("ceb", "Jelgava (munisipyo)"), ("cs", "Jelgava"), ("cy", "Jelgava"), ("da", "Jelgava"), ("de", "Jelgava"), ("el", "Γιέλγκαβα²"), ("en", "Jelgava"), ("es", "Jelgava"), ("et", "Jelgava"), ("eu", "Jelgava"), ("fa", "یلگاوا"), ("fi", "Jelgava"), ("fr", "Jelgava²"), ("gl", "Jelgava"), ("gu", "જ\u{ac7}લગાવા"), ("he", "ילגבה"), ("hi", "ज\u{947}ल\u{94d}गावा"), ("hr", "Jelgava"), ("hu", "Jelgava"), ("hy", "Ելգավա"), ("id", "Jelgava"), ("it", "Jelgava²"), ("ja", "イェルガヴァ"), ("ka", "ელგავა"), ("kk", "Елгава"), ("kn", "ಜ\u{cc6}ಲ\u{ccd}ಗವ"), ("ko", "옐가바"), ("lt", "Jelgava"), ("lv", "Jelgava"), ("mk", "Јелгава²"), ("mr", "ज\u{947}ऌगवा"), ("ms", "Jelgava"), ("nb", "Jelgava"), ("nl", "Jelgava"), ("no", "Jelgava"), ("pl", "Jełgawa"), ("pt", "Jelgava"), ("ro", "Jelgava"), ("ru", "Елгава"), ("si", "ජෙල\u{dca}ග\u{dcf}ව\u{dcf}"), ("sk", "Jelgava"), ("sl", "Jelgava"), ("sr", "Јелгава"), ("sr_Latn", "Jelgava"), ("sv", "Jelgava"), ("ta", "ஜெல\u{bcd}கவ\u{bbe}"), ("te", "జ\u{c46}ల\u{c4d}గ\u{c3e}వ\u{c3e}"), ("th", "เยลกาวา"), ("tr", "Jelgava"), ("uk", "Єлгава"), ("ur", "یالگاوا"), ("uz", "Yelgava"), ("vi", "Jelgava"), ("zh", "叶尔加瓦")]),
                        unofficial_name_list: ["Jelgava"].to_vec(),
                    }
                ),
                (
                    "JUR",
                    Subdivision{
                        name: "JUR",
                        country_alpha2: Alpha2::LV,
                        code: "JUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.947079), longitude: Some(23.6168486), max_latitude: Some(57.007044), min_latitude: Some(56.923886), max_longitude: Some(23.9693479), min_longitude: Some(23.47297)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jūrmala"), ("ar", "جورمالا"), ("az", "Yurmala"), ("be", "Юрмала"), ("bg", "Юрмала"), ("bn", "জ\u{9c1}ম\u{9be}ল\u{9be}"), ("ca", "Jūrmala"), ("ccp", "𑄎𑄢\u{11134}𑄟𑄣"), ("cs", "Jūrmala"), ("cy", "Jūrmala"), ("da", "Jūrmala"), ("de", "Jūrmala"), ("el", "Γιούρμαλα"), ("en", "Jūrmala"), ("es", "Jūrmala"), ("et", "Jūrmala"), ("eu", "Jūrmala"), ("fa", "یورمالا"), ("fi", "Jūrmala"), ("fr", "Jurmala"), ("gl", "Jūrmala"), ("gu", "જ\u{ac1}ર\u{acd}માલા"), ("he", "יורמלה"), ("hi", "ज\u{941}रमाला"), ("hr", "Jūrmala"), ("hu", "Jūrmala"), ("hy", "Յուրմալա"), ("id", "Jūrmala"), ("it", "Jūrmala"), ("ja", "ユールマラ"), ("ka", "იურმალა"), ("kk", "Юрмала"), ("kn", "ಜುರ\u{ccd}ಮಾಲಾ"), ("ko", "유르말라"), ("lt", "Jūrmala"), ("lv", "Jūrmala"), ("mk", "Јурмала"), ("mr", "ज\u{941}र\u{94d}मला"), ("ms", "Jurmala"), ("nb", "Jūrmala"), ("nl", "Jūrmala"), ("no", "Jūrmala"), ("pl", "Jurmała"), ("pt", "Jūrmala"), ("ro", "Jūrmala"), ("ru", "Юрмала"), ("si", "ජ\u{dd4}ර\u{dca}මල\u{dcf}"), ("sk", "Jūrmala"), ("sl", "Jūrmala"), ("sr", "Јурмала"), ("sr_Latn", "Jurmala"), ("sv", "Jūrmala"), ("ta", "ஜுர\u{bcd}ம\u{bbe}ல\u{bbe}"), ("te", "జుర\u{c4d}మ\u{c3e}ల\u{c3e}"), ("th", "ย\u{e31}วมาลา"), ("tr", "Jurmala"), ("uk", "Юрмала"), ("ur", "یورمالا"), ("uz", "Yurmala"), ("vi", "Jūrmala"), ("zh", "尤爾馬拉")]),
                        unofficial_name_list: ["Jurmala"].to_vec(),
                    }
                ),
                (
                    "LPX",
                    Subdivision{
                        name: "LPX",
                        country_alpha2: Alpha2::LV,
                        code: "LPX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.5046678), longitude: Some(21.010806), max_latitude: Some(56.60919999999999), min_latitude: Some(56.464691), max_longitude: Some(21.1057334), min_longitude: Some(20.971237)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Liepaja"), ("ar", "ليباجا"), ("be", "Ліепая"), ("bg", "Лиепая"), ("bn", "লিপ\u{9be}জ\u{9be}"), ("ca", "Liepāja"), ("ccp", "𑄣\u{1112d}𑄛𑄎"), ("ceb", "Liepāja"), ("cs", "Liepāja"), ("cy", "Liepāja"), ("da", "Liepāja"), ("de", "Liepāja"), ("el", "Λιεπάγια"), ("en", "Liepāja"), ("es", "Liepāja"), ("et", "Liepāja"), ("eu", "Liepāja"), ("fa", "لیپایا"), ("fi", "Liepāja"), ("fr", "Liepāja"), ("gl", "Liepāja"), ("gu", "લીપ\u{ac7}જા"), ("he", "לייפאיה"), ("hi", "लिएपाजा"), ("hr", "Liepāja"), ("hu", "Liepāja"), ("hy", "Լիեպայա"), ("id", "Liepāja"), ("it", "Liepāja"), ("ja", "リエパーヤ"), ("ka", "ლიეპაია"), ("kk", "Лиепая"), ("kn", "ಲ\u{cbf}ಏಪಜಾ"), ("ko", "리예파야"), ("ky", "Лиепая"), ("lt", "Liepoja"), ("lv", "Liepāja"), ("mr", "लीपाया"), ("ms", "Liepāja"), ("nb", "Liepāja"), ("nl", "Liepāja"), ("no", "Liepāja"), ("pa", "ਲਿਪਾਯਾ"), ("pl", "Lipawa"), ("pt", "Liepāja"), ("ro", "Liepāja"), ("ru", "Лиепая"), ("si", "ල\u{dd3}පජ\u{dcf}"), ("sk", "Liepāja"), ("sr", "Лијепаја"), ("sr_Latn", "Lijepaja"), ("sv", "Liepāja"), ("ta", "லிப\u{bbe}ஜ\u{bbe}"), ("te", "ల\u{c40}ప\u{c3e}జ\u{c3e}"), ("th", "ไลพาจาส\u{e4c}"), ("tr", "Liepāja"), ("uk", "Лієпая"), ("ur", "لیاپائیا"), ("uz", "Liyepaya"), ("vi", "Liepāja"), ("zh", "利耶帕亚")]),
                        unofficial_name_list: ["Liepaja"].to_vec(),
                    }
                ),
                (
                    "REZ",
                    Subdivision{
                        name: "REZ",
                        country_alpha2: Alpha2::LV,
                        code: "REZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.5099223), longitude: Some(27.3331357), max_latitude: Some(56.53846009999999), min_latitude: Some(56.4775499), max_longitude: Some(27.379451), min_longitude: Some(27.3038497)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rēzekne"), ("ar", "ريزكني"), ("az", "Rezekne"), ("be", "Горад Рэзэкнэ"), ("bg", "Резекне"), ("bn", "রেজেকনে"), ("ca", "Rēzekne"), ("ccp", "𑄢𑄬𑄎𑄬𑄇\u{11134}𑄚\u{11128}"), ("ceb", "Rēzekne (kapital sa munisipyo)"), ("cs", "Rēzekne"), ("cy", "Rēzekne"), ("da", "Rēzekne"), ("de", "Rēzekne"), ("el", "Ρέζεκνε²"), ("en", "Rēzekne"), ("es", "Rēzekne"), ("et", "Rēzekne"), ("eu", "Rēzekne"), ("fa", "رزکنه"), ("fi", "Rēzekne"), ("fr", "Rēzekne²"), ("gl", "Rēzekne"), ("gu", "રીજ\u{ac7}ક\u{acd}ન\u{ac7}"), ("he", "רזקנה"), ("hi", "रीजक\u{947}न"), ("hu", "Rēzekne"), ("hy", "Ռեզեկնե"), ("id", "Rēzekne"), ("it", "Rēzekne²"), ("ja", "レーゼクネ²"), ("ka", "რეზეკნე"), ("kk", "Резекне"), ("kn", "ರ\u{cc6}ಝ\u{cc6}ಕ\u{ccd}ನ\u{cc6}"), ("ko", "레제크네"), ("lt", "Rėzeknė"), ("lv", "Rēzekne"), ("mk", "Резекне²"), ("mr", "रीज\u{947}क\u{94d}न\u{947}"), ("ms", "Rezekne"), ("nb", "Rēzekne"), ("nl", "Rēzekne"), ("no", "Rēzekne"), ("pa", "ਰੀਜਿਕਨ"), ("pl", "Rzeżyca"), ("pt", "Rezekne"), ("ro", "Rēzekne"), ("ru", "Резекне"), ("si", "රෙසේක\u{dca}නේ"), ("sk", "Rēzekne"), ("sl", "Režica"), ("sq", "Rēzekne"), ("sr", "Резекне"), ("sr_Latn", "Rezekne"), ("sv", "Rēzekne"), ("ta", "ரெசெக\u{bcd}னி"), ("te", "ర\u{c46}జ\u{c46}క\u{c4d}న\u{c47}"), ("th", "เรเซคเน"), ("tr", "Rezekne"), ("uk", "Резекне"), ("ur", "ریزکنے"), ("vi", "Rezekne"), ("zh", "雷澤克內")]),
                        unofficial_name_list: ["Rezekne"].to_vec(),
                    }
                ),
                (
                    "RIX",
                    Subdivision{
                        name: "RIX",
                        country_alpha2: Alpha2::LV,
                        code: "RIX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.9496487), longitude: Some(24.1051864), max_latitude: Some(57.085921), min_latitude: Some(56.8570279), max_longitude: Some(24.3246659), min_longitude: Some(23.9336591)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Riga"), ("am", "ሪጋ"), ("ar", "ريغا"), ("az", "Riqa"), ("be", "Рыга"), ("bg", "Рига"), ("bn", "রিগ\u{9be}"), ("bs", "Riga"), ("ca", "Riga"), ("ccp", "𑄢\u{11128}𑄉"), ("ceb", "Riga"), ("cs", "Riga"), ("cy", "Riga"), ("da", "Riga"), ("de", "Riga"), ("el", "Ρίγα"), ("en", "Riga"), ("es", "Riga"), ("et", "Riia"), ("eu", "Riga"), ("fa", "ریگا"), ("fi", "Riika"), ("fr", "Riga"), ("ga", "Ríge"), ("gl", "Riga"), ("gu", "રીગા"), ("ha", "Riga"), ("ha_NE", "Riga"), ("he", "ריגה"), ("hi", "रीगा"), ("hr", "Riga"), ("hu", "Riga"), ("hy", "Ռիգա"), ("id", "Riga"), ("is", "Ríga"), ("it", "Riga"), ("ja", "リガ"), ("jv", "Riga"), ("ka", "რიგა"), ("kk", "Рига"), ("kn", "ರ\u{cbf}ಗಾ"), ("ko", "리가"), ("ky", "Рига"), ("lt", "Ryga"), ("lv", "Rīga"), ("mk", "Рига"), ("ml", "റിഗ"), ("mn", "Рига"), ("mr", "रिगा"), ("ms", "Riga"), ("my", "ရ\u{102e}ဂါမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Riga"), ("nl", "Riga"), ("no", "Riga"), ("pa", "ਰੀਗਾ"), ("pl", "Ryga"), ("pt", "Riga"), ("ro", "Riga"), ("ru", "Рига"), ("si", "ර\u{dd2}ග\u{dcf}"), ("sk", "Riga"), ("sl", "Riga"), ("sq", "Riga"), ("sr", "Рига"), ("sr_Latn", "Riga"), ("sv", "Riga"), ("sw", "Riga"), ("ta", "ர\u{bc0}க\u{bbe}"), ("te", "ర\u{c40}గ\u{c3e}"), ("th", "ร\u{e35}กา"), ("tk", "Riga"), ("tr", "Riga"), ("uk", "Рига"), ("ur", "ریگا"), ("uz", "Riga"), ("vi", "Riga"), ("yo", "Riga"), ("yo_BJ", "Riga"), ("yue", "里加"), ("yue_Hans", "里加"), ("zh", "里加"), ("zu", "IRiga")]),
                        unofficial_name_list: ["Riga"].to_vec(),
                    }
                ),
                (
                    "VEN",
                    Subdivision{
                        name: "VEN",
                        country_alpha2: Alpha2::LV,
                        code: "VEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.3937216), longitude: Some(21.5647066), max_latitude: Some(57.47072199999999), min_latitude: Some(57.34809389999999), max_longitude: Some(21.6736649), min_longitude: Some(21.5157259)}),
                        comments: None,
                        subdivision_type: SubdivisionType::StateCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ventspils"), ("ar", "فينتسبيلس"), ("be", "Горад Вэнтспілс"), ("bg", "Вентспилс"), ("bn", "ভেন\u{9cd}টস\u{9cd}পিলস"), ("ca", "Ventspils"), ("ccp", "𑄞𑄚\u{11134}𑄑\u{11134}𑄛\u{11128}𑄣\u{11134}𑄥\u{11134}"), ("ceb", "Ventspils"), ("cs", "Ventspils"), ("cy", "Ventspils"), ("da", "Ventspils"), ("de", "Ventspils"), ("el", "Βέντσπιλς²"), ("en", "Ventspils"), ("es", "Ventspils"), ("et", "Ventspils"), ("eu", "Ventspils"), ("fa", "ونتسپیلس"), ("fi", "Ventspils"), ("fr", "Ventspils²"), ("gl", "Ventspils"), ("gu", "વ\u{ac7}ન\u{acd}ટસપિલ\u{acd}સ"), ("he", "ונטספילס"), ("hi", "व\u{947}\u{902}टसपिल\u{94d}स"), ("hr", "Ventspils"), ("hu", "Ventspils"), ("hy", "Վենտսպիլս"), ("id", "Ventspils"), ("it", "Ventspils²"), ("ja", "ヴェンツピルス²"), ("ka", "ვენტსპილსი"), ("kk", "Вентспилс"), ("kn", "ವ\u{cc6}ಂಟ\u{ccd}ಸ\u{ccd}ಪ\u{cbf}ಲ\u{ccd}ಸ\u{ccd},"), ("ko", "벤츠필스"), ("ky", "Вентспилс"), ("lt", "Ventspilis"), ("lv", "Ventspils"), ("mk", "Вентспилс²"), ("mr", "व\u{947}न\u{94d}टस\u{94d}पिल\u{94d}स"), ("ms", "Ventspils"), ("nb", "Ventspils"), ("nl", "Ventspils"), ("no", "Ventspils"), ("pa", "ਵ\u{a47}\u{a02}ਟਸਪਿਲਸ"), ("pl", "Windawa"), ("pt", "Ventspils"), ("ro", "Ventspils"), ("ru", "Вентспилс"), ("si", "වෙන\u{dca}ට\u{dca}ස\u{dca}ප\u{dd2}ල\u{dca}ස\u{dca}"), ("sk", "Ventspils"), ("sr", "Вентспилс"), ("sr_Latn", "Ventspils"), ("sv", "Ventspils"), ("ta", "வென\u{bcd}ட\u{bcd}ஸ\u{bcd}ப\u{bcd}பிள\u{bcd}ஸ\u{bcd}"), ("te", "వ\u{c46}ంట\u{c4d}స\u{c4d}\u{200c}ప\u{c3f}ల\u{c4d}స\u{c4d}"), ("th", "เวนต\u{e4c}สป\u{e34}ลส\u{e4c}²"), ("tr", "Ventspils"), ("uk", "Вентспілс"), ("ur", "وینتپلس"), ("uz", "Ventspils"), ("vi", "Ventspils"), ("zh", "文茨皮尔斯")]),
                        unofficial_name_list: ["Ventspils"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "lv")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::LV,
        alpha3: Alpha3::LVA,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 371,
        currency_code: "EUR",
        gec: Some(GEC::LG),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::LAT),
        iso_long_name: "The Republic of Latvia",
        iso_short_name: "Latvia",
        official_language_list: ["lv"].to_vec(),
        spoken_language_list: ["lv"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "8",
        nationality: Some("Latvian"),
        number: "428",
        postal_code: true,
        postal_code_format: Some("LV-\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "LV",
        unofficial_name_list: [
            "Latvia",
            "Lettland",
            "Lettonie",
            "Letonia",
            "ラトビア",
            "Letland",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Latvia"),
            ("af", "Letland"),
            ("ak", "Latvia"),
            ("am", "ሒትቱ።"),
            ("an", "Latvia"),
            ("ar", "لاتفيا"),
            ("as", "ল\u{9be}টভিয়\u{9be}"),
            ("ay", "Latvia"),
            ("az", "Latviya"),
            ("ba", "Latvia"),
            ("be", "Латвія"),
            ("bg", "Латвия"),
            ("bi", "Latvia"),
            ("bn", "ল\u{9be}টভিয়\u{9be}"),
            ("bn_IN", "ল\u{9be}টভিয়\u{9be}"),
            ("br", "Latvia"),
            ("bs", "Latvija"),
            ("ca", "Letònia"),
            ("ce", "Латви"),
            ("ch", "Latvia"),
            ("cs", "Lotyšsko"),
            ("cv", "Латви"),
            ("cy", "Latfia"),
            ("da", "Letland"),
            ("de", "Lettland"),
            ("dv", "ލ\u{7ac}ޓ\u{7aa}ވ\u{7a8}އ\u{7a7}"),
            ("dz", "ལ\u{f7a}ཊ\u{f72}་བ\u{f72}་ཡ།"),
            ("ee", "Latvia"),
            ("el", "Λετονία"),
            ("en", "Latvia"),
            ("eo", "Latvio"),
            ("es", "Letonia"),
            ("et", "Läti"),
            ("eu", "Letonia"),
            ("fa", "لتونی"),
            ("ff", "Latvia"),
            ("fi", "Latvia"),
            ("fo", "Lettland"),
            ("fr", "Lettonie"),
            ("fy", "Letlân"),
            ("ga", "An Laitvia"),
            ("gl", "Letonia"),
            ("gn", "Latvia"),
            ("gu", "લ\u{ac7}ટવિયા"),
            ("gv", "Yn Latvey"),
            ("ha", "Laitfiya"),
            ("he", "לטביה"),
            ("hi", "लातविया"),
            ("hr", "Latvija"),
            ("ht", "Letoni"),
            ("hu", "Lettország"),
            ("hy", "Լատվիա"),
            ("ia", "Lettonia"),
            ("id", "Latvia"),
            ("io", "Latvia"),
            ("is", "Lettland"),
            ("it", "Lettonia"),
            ("iu", "Latvia"),
            ("ja", "ラトビア"),
            ("ka", "ლატვია"),
            ("ki", "Latvia"),
            ("kk", "Латвия"),
            ("kl", "Latvia"),
            ("km", "ឡាតវ\u{17b8}យ\u{17c9}ា"),
            ("kn", "ಲಾತ\u{ccd}ವ\u{cbf}ಯಾ"),
            ("ko", "라트비아"),
            ("ku", "Latviya"),
            ("kv", "Латвия"),
            ("kw", "Latvi"),
            ("ky", "Латвия"),
            ("lo", "ປະເທດແລດໂຕນ\u{eb5}"),
            ("lt", "Latvija"),
            ("lv", "Latvija"),
            ("mi", "Rāwhia"),
            ("mk", "Летонија"),
            ("ml", "ല\u{d3e}ത\u{d4d}വിയ"),
            ("mn", "Латви"),
            ("mr", "लाटव\u{94d}हिया"),
            ("ms", "Latvia"),
            ("mt", "Latvja"),
            (
                "my",
                "လတ\u{103a}ဗ\u{102e}ယာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ratebiya"),
            ("nb", "Latvia"),
            ("ne", "लात\u{94d}भिया"),
            ("nl", "Letland"),
            ("nn", "Latvia"),
            ("nv", "Létbiiya"),
            ("oc", "Letònia"),
            ("or", "ଲ\u{b3e}ଟଭ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਲਾਟੀਵਾਆ"),
            ("pi", "लाट\u{94d}विया"),
            ("pl", "Łotwa"),
            ("ps", "Latvia"),
            ("pt", "Letónia"),
            ("pt_BR", "Letônia"),
            ("ro", "Letonia"),
            ("ru", "Латвия"),
            ("rw", "Lativiya"),
            ("sc", "Letònia"),
            ("sd", "Latvia"),
            ("si", "ලැත\u{dca}ව\u{dd2}ය\u{dcf}ව"),
            ("sk", "Lotyšsko"),
            ("sl", "Latvija"),
            ("so", "Laatfiya"),
            ("sq", "Letoni"),
            ("sr", "Летонија"),
            ("sv", "Lettland"),
            ("sw", "Latvia"),
            ("ta", "லட\u{bcd}விய\u{bbe}"),
            ("te", "ల\u{c3e}టవ\u{c4d}హ\u{c3f}య\u{c3e}"),
            ("tg", "Латвия"),
            ("th", "ล\u{e31}ตเว\u{e35}ย"),
            ("ti", "ላትቪያ"),
            ("tk", "Litwiýa"),
            ("tl", "Latvia"),
            ("tr", "Letonya"),
            ("tt", "Латвиа"),
            ("ug", "لاتۋىيە"),
            ("uk", "Латвія"),
            ("ur", "لٹویا"),
            ("uz", "Latviya"),
            ("ve", "Latvia"),
            ("vi", "Lát-vi-a"),
            ("wa", "Letoneye"),
            ("wo", "Letóoni"),
            ("xh", "Latvia"),
            ("yo", "Látfíà"),
            ("zh_CN", "拉脱维亚"),
            ("zh_HK", "拉脫維亞"),
            ("zh_TW", "拉脫維亞"),
            ("zu", "ILatviya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
