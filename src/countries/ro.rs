// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Romania

#[cfg(all(feature = "ro", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::RO;
    pub const ALPHA3: Alpha3 = Alpha3::ROU;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 40;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::RON;
    pub const GEC: Option<GEC> = Some(GEC::RO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ROU);
    pub const ISO_SHORT_NAME: &str = "Romania";
    pub const ISO_LONG_NAME: &str = "Romania";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ro"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ro"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Romanian");
    pub const NUMBER: &str = "642";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "RO";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Romania",
        "Rumänien",
        "Roumanie",
        "Rumania",
        "ルーマニア",
        "Roemenië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Romania"),
        ("af", "Roemenië"),
        ("ak", "Romania"),
        ("am", "ስሣኒ።"),
        ("an", "Romania"),
        ("ar", "رومانيا"),
        ("as", "ৰোম\u{9be}নিয়\u{9be}"),
        ("ay", "Romania"),
        ("az", "Ruminıya"),
        ("ba", "Romania"),
        ("be", "Румынія"),
        ("bg", "Румъния"),
        ("bi", "Romania"),
        ("bn", "রোম\u{9be}নিয়\u{9be}"),
        ("bn_IN", "রোম\u{9be}নিয়\u{9be}"),
        ("br", "Roumania"),
        ("bs", "Rumunija"),
        ("ca", "Romania"),
        ("ce", "Румыни"),
        ("ch", "Romania"),
        ("cs", "Rumunsko"),
        ("cv", "Румыни"),
        ("cy", "Rwmania"),
        ("da", "Rumænien"),
        ("de", "Rumänien"),
        ("dv", "ރ\u{7aa}މ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "ར\u{f7c}་མ་ན\u{f72}་ཡ།"),
        ("ee", "Romania"),
        ("el", "Ρουμανία"),
        ("en", "Romania"),
        ("eo", "Rumanio"),
        ("es", "Rumanía"),
        ("et", "Rumeenia"),
        ("eu", "Errumania"),
        ("fa", "رومانی"),
        ("ff", "Romaniya"),
        ("fi", "Romania"),
        ("fo", "Rumenia"),
        ("fr", "Roumanie"),
        ("fy", "Roemeenje"),
        ("ga", "An Rómáin"),
        ("gl", "Romanía"),
        ("gn", "Romania"),
        ("gu", "રોમાનિયા"),
        ("gv", "Yn Romaan"),
        ("ha", "Romainiya"),
        ("he", "רומניה"),
        ("hi", "रोमानिया"),
        ("hr", "Rumunjska"),
        ("ht", "Woumani"),
        ("hu", "Románia"),
        ("hy", "Ռումինիա"),
        ("ia", "Romania"),
        ("id", "Rumania"),
        ("io", "Rumania"),
        ("is", "Rúmenía"),
        ("it", "Romania"),
        ("iu", "Romania"),
        ("ja", "ルーマニア"),
        ("ka", "რუმინეთი"),
        ("ki", "Romania"),
        ("kk", "Румыния"),
        ("kl", "Romania"),
        ("km", "រ\u{17bc}ម\u{17c9}ាន\u{17b8}"),
        ("kn", "ರೊಮೇನ\u{cbf}ಯಾ"),
        ("ko", "루마니아"),
        ("ku", "Romanya"),
        ("kv", "Румыния"),
        ("kw", "Roumani"),
        ("ky", "Румыния"),
        ("lo", "ປະເທດລ\u{eb9}ມານ\u{eb5}"),
        ("lt", "Rumunija"),
        ("lv", "Rumānija"),
        ("mi", "Romeinia"),
        ("mk", "Романија"),
        ("ml", "റൊമ\u{d3e}നിയ"),
        ("mn", "Румын"),
        ("mr", "रोमानिया"),
        ("ms", "Romania"),
        ("mt", "Rumanija"),
        (
            "my",
            "ရ\u{102d}\u{102f}မေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Romania"),
        ("nb", "Romania"),
        ("ne", "रोमानिया"),
        ("nl", "Roemenië"),
        ("nn", "Romania"),
        ("nv", "Wooméiniya"),
        ("oc", "Romania"),
        ("or", "ରୋମ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਰ\u{a4b}ਮਾਨੀਆ"),
        ("pi", "रोमानिया"),
        ("pl", "Rumunia"),
        ("ps", "رومانیا"),
        ("pt", "Roménia"),
        ("pt_BR", "Romênia"),
        ("ro", "România"),
        ("ru", "Румыния"),
        ("rw", "Romaniya"),
        ("sc", "Romania"),
        ("sd", "Romania"),
        ("si", "රොමේන\u{dd2}ය\u{dcf}ව"),
        ("sk", "Rumunsko"),
        ("sl", "Romunija"),
        ("so", "Rumaaniya"),
        ("sq", "Rumani"),
        ("sr", "Румунија"),
        ("sv", "Rumänien"),
        ("sw", "Romania"),
        ("ta", "ரோம\u{bbe}னிய\u{bbe}"),
        ("te", "ర\u{c4b}మ\u{c3e}న\u{c3f}య\u{c3e}"),
        ("tg", "Руминия"),
        ("th", "โรมาเน\u{e35}ย"),
        ("ti", "ሮሜኒያ"),
        ("tk", "Rumyniýa"),
        ("tl", "Romania"),
        ("tr", "Romanya"),
        ("tt", "Романиа"),
        ("ug", "رۇمىنىيە"),
        ("uk", "Румунія"),
        ("ur", "رومانیہ"),
        ("uz", "Ruminiya"),
        ("ve", "Romania"),
        ("vi", "Rô-ma-ni"),
        ("wa", "Roumaneye"),
        ("wo", "Romaani"),
        ("xh", "Romania"),
        ("yo", "Románíà"),
        ("zh_CN", "罗马尼亚"),
        ("zh_HK", "羅馬尼亞"),
        ("zh_TW", "羅馬尼亞"),
        ("zu", "I-Romaniya"),
    ];
    #[cfg(all(feature = "ro", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 45.943161;
        pub const LONGITUDE: f64 = 24.96676;
        pub const MAX_LATITUDE: f64 = 48.26518;
        pub const MAX_LONGITUDE: f64 = 29.77839999999999;
        pub const MIN_LATITUDE: f64 = 43.6186193;
        pub const MIN_LONGITUDE: f64 = 20.2617591;
        pub const NORTHEAST_LATITUDE: f64 = 48.26518;
        pub const NORTHEAST_LONGITUDE: f64 = 29.77839999999999;
        pub const SOUTHWEST_LATITUDE: f64 = 43.6186193;
        pub const SOUTHWEST_LONGITUDE: f64 = 20.2617591;
    }
}
#[cfg(all(feature = "ro", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 45.943161,
            longitude: 24.96676,
            max_latitude: 48.26518,
            max_longitude: 29.77839999999999,
            min_latitude: 43.6186193,
            min_longitude: 20.2617591,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 48.26518,
                    longitude: 29.77839999999999,
                },
                southwest: CountryGeoBound {
                    latitude: 43.6186193,
                    longitude: 20.2617591,
                },
            },
        }
    }
}

#[cfg(all(feature = "ro", feature = "subdivisions"))]
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
                    "AB",
                    Subdivision{
                        name: "Alba",
                        country_alpha2: Alpha2::RO,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1558924), longitude: Some(23.5556121), max_latitude: Some(46.569062), min_latitude: Some(45.467659), max_longitude: Some(24.2515318), min_longitude: Some(22.6724509)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ألبا"), ("be", "жудзец Алба"), ("bg", "Алба"), ("bn", "এলব\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Alba (okrug)"), ("ca", "Província d’Alba"), ("ccp", "𑄃𑄣\u{11134}𑄝"), ("ceb", "Județul Alba"), ("cs", "Alba"), ("da", "Alba"), ("de", "Kreis Alba"), ("el", "Άλμπα"), ("en", "Alba"), ("es", "Alba"), ("et", "Alba maakond"), ("eu", "Alba konderria"), ("fa", "شهرستان آلبا"), ("fi", "Alba"), ("fr", "județ d’Alba"), ("gl", "Condado de Alba"), ("gu", "આલ\u{acd}બા કાઉન\u{acd}ટી"), ("he", "אלבה"), ("hi", "अल\u{94d}बा काउ\u{902}टी"), ("hr", "Alba"), ("hu", "Fehér megye"), ("id", "Provinsi Alba"), ("it", "distretto di Alba"), ("ja", "アルバ県"), ("ka", "ალბის ჟუდეცი"), ("kn", "ಆಲ\u{ccd}ಬಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "알바 주"), ("lt", "Albos apskritis"), ("lv", "Albas žudecs"), ("mk", "Алба"), ("mn", "Алба аймаг"), ("mr", "अल\u{94d}बा काउ\u{902}टी"), ("ms", "Daerah Alba"), ("nb", "Alba"), ("nl", "District Alba"), ("no", "Alba"), ("pl", "Okręg Alba"), ("pt", "Alba"), ("ro", "Alba"), ("ru", "Алба"), ("si", "ඇල\u{dca}බ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Alba"), ("sq", "Qarku Alba"), ("sr", "Алба"), ("sr_Latn", "Alba"), ("sv", "Alba"), ("ta", "அல\u{bcd}ப\u{bbe} கவுண\u{bcd}டி"), ("te", "ఆల\u{c4d}బ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "อ\u{e31}ลบา"), ("tr", "Alba ili"), ("uk", "Алба"), ("ur", "البا کاؤنٹی"), ("vi", "Alba"), ("zh", "阿爾巴縣")]),
                        unofficial_name_list: ["Alba"].to_vec(),
                    }
                ),
                (
                    "AG",
                    Subdivision{
                        name: "Arges",
                        country_alpha2: Alpha2::RO,
                        code: "AG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.0722527), longitude: Some(24.8142726), max_latitude: Some(45.610782), min_latitude: Some(44.366297), max_longitude: Some(25.325839), min_longitude: Some(24.427089)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أرغيس"), ("be", "Арджэш"), ("bg", "Арджеш"), ("bn", "আর\u{9cd}জেস ক\u{9be}উন\u{9cd}টি"), ("ca", "Província d’Argeș"), ("ccp", "𑄃𑄢\u{11134}𑄎𑄬𑄌\u{11134}"), ("ceb", "Județul Argeș"), ("cs", "Argeș"), ("da", "Argeș"), ("de", "Kreis Argeș"), ("el", "Άρτζες"), ("en", "Argeș"), ("es", "Argeș"), ("et", "Argeși maakond"), ("fa", "شهرستان ارجش"), ("fi", "Argeș"), ("fr", "județ d’Argeș"), ("gl", "Condado de Argeş"), ("gu", "અર\u{acd}જ\u{ac7}સ કાઉન\u{acd}ટી"), ("he", "מחוז ארג׳ש"), ("hi", "अर\u{94d}ज\u{947}स काउ\u{902}टी"), ("hr", "Argeş"), ("hu", "Argeș megye"), ("id", "Provinsi Argeş"), ("it", "distretto di Argeș"), ("ja", "アルジェシュ県"), ("ka", "არჯეშის ჟუდეცი"), ("kn", "ಅರ\u{ccd}ಗ\u{cc6}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "아르제슈 주"), ("lt", "Ardžešo apskritis"), ("lv", "Ardžešas žudecs"), ("mk", "Арџеш"), ("mn", "Аржеш аймаг"), ("mr", "अर\u{94d}ज\u{947}स काउ\u{902}टी"), ("ms", "Daerah Argeş"), ("nb", "Argeș"), ("nl", "District Argeș"), ("no", "Argeș"), ("pl", "Okręg Ardżesz"), ("pt", "Argeş"), ("ro", "Argeș"), ("ru", "Арджеш"), ("si", "අර\u{dca}ගස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Argeş"), ("sq", "Qarku Argeș"), ("sr", "Арђеш"), ("sr_Latn", "Arđeš"), ("sv", "Argeș"), ("ta", "அர\u{bcd}கெஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "ఆర\u{c4d}జ\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "อาร\u{e4c}เจส"), ("tr", "Argeș ili"), ("uk", "Арджеш"), ("ur", "آرجش کاؤنٹی"), ("vi", "Argeş"), ("zh", "阿爾傑什縣")]),
                        unofficial_name_list: ["Arges"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "Arad",
                        country_alpha2: Alpha2::RO,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.166667), longitude: Some(21.316667), max_latitude: Some(46.2217118), min_latitude: Some(46.1286903), max_longitude: Some(21.3823449), min_longitude: Some(21.230371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أراد"), ("be", "жудзец Арад"), ("bg", "Арад"), ("bn", "আর\u{9be}দ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província d’Arad"), ("ccp", "𑄃𑄢𑄖\u{11134}"), ("ceb", "Arad (lalawigan)"), ("cs", "Arad"), ("da", "Arad"), ("de", "Kreis Arad"), ("el", "Αράντ"), ("en", "Arad"), ("es", "Arad"), ("et", "Aradi maakond"), ("eu", "Arad konderria"), ("fa", "شهرستان آراد"), ("fi", "Arad"), ("fr", "județ d’Arad"), ("gl", "Condado de Arad"), ("gu", "અરાદ કાઉન\u{acd}ટી"), ("he", "מחוז ארד"), ("hi", "अराद काउ\u{902}टी"), ("hr", "Arad"), ("hu", "Arad megye"), ("id", "Provinsi Arad"), ("it", "distretto di Arad"), ("ja", "アラド県"), ("ka", "არადის ჟუდეცი"), ("kn", "ಅರಾದ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "아라드 주"), ("lt", "Arado apskritis"), ("lv", "Aradas žudecs"), ("mk", "Арад"), ("mn", "Арад аймаг"), ("mr", "आर\u{94d}द काउ\u{902}टी"), ("ms", "Daerah Arad"), ("nb", "Arad"), ("nl", "District Arad"), ("no", "Arad"), ("pl", "Okręg Arad"), ("pt", "Arad"), ("ro", "Arad"), ("ru", "Арад"), ("si", "අරඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Arad"), ("sq", "Qarku Arad"), ("sr", "Арад"), ("sr_Latn", "Arad"), ("sv", "Arad"), ("ta", "அரத\u{bcd} கவுண\u{bcd}டி"), ("te", "అరద\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลอาร\u{e31}ด"), ("tr", "Arad ili"), ("uk", "Арад"), ("ur", "آراد کاؤنٹی"), ("vi", "Arad"), ("zh", "阿拉德縣")]),
                        unofficial_name_list: ["Arad"].to_vec(),
                    }
                ),
                (
                    "B",
                    Subdivision{
                        name: "Bucuresti",
                        country_alpha2: Alpha2::RO,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.4267674), longitude: Some(26.1025384), max_latitude: Some(44.541407), min_latitude: Some(44.3342445), max_longitude: Some(26.225575), min_longitude: Some(25.9637001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Boekarest"), ("am", "ቡካረስት"), ("ar", "بوخارست"), ("az", "Buxarest"), ("be", "Бухарэст"), ("bg", "Букурещ"), ("bn", "ব\u{9c1}খ\u{9be}রেস\u{9cd}ট"), ("bs", "Bukurešt"), ("ca", "Bucarest"), ("ccp", "𑄝\u{1112a}𑄇𑄢𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "București (lalawigan sa Rumanya)"), ("cs", "Bukurešť"), ("cy", "Bwcarést"), ("da", "Bukarest"), ("de", "Bukarest"), ("el", "Βουκουρέστι"), ("en", "Bucharest"), ("es", "Bucarest"), ("et", "Bukarest"), ("eu", "Bukarest"), ("fa", "بخارست"), ("fi", "Bukarest"), ("fr", "Bucarest"), ("ga", "Búcairist"), ("gl", "Bucarest"), ("gu", "બ\u{ac1}કાર\u{ac7}સ\u{acd}ટ"), ("he", "בוקרשט"), ("hi", "ब\u{941}खार\u{947}स\u{94d}ट"), ("hr", "Bukurešt"), ("hu", "Bukarest"), ("hy", "Բուխարեստ"), ("id", "Bukares"), ("is", "Búkarest"), ("it", "Bucarest"), ("ja", "ブカレスト"), ("jv", "Bukarès"), ("ka", "ბუქარესტი"), ("kk", "Бухарест"), ("kn", "ಬ\u{ccd}ಯ\u{cc2}ಖರ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}"), ("ko", "부쿠레슈티"), ("ky", "Бухарест"), ("lt", "Bukareštas"), ("lv", "Bukareste"), ("mk", "Букурешт"), ("ml", "ബ\u{d41}ക\u{d4d}ക\u{d3e}റെസ\u{d4d}റ\u{d4d}റ\u{d4d}"), ("mn", "Бухарест"), ("mr", "ब\u{941}खार\u{947}स\u{94d}ट"), ("ms", "Bucharest"), ("my", "ဗ\u{1030}းခရက\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "București"), ("ne", "ब\u{941}खार\u{947}स\u{94d}ट"), ("nl", "Boekarest"), ("no", "București"), ("pa", "ਬ\u{a41}ਖ\u{a3c}ਾਰ\u{a48}ਸਟ"), ("pl", "Bukareszt"), ("ps", "بخارست"), ("pt", "Bucareste"), ("ro", "București"), ("ru", "Бухарест"), ("si", "බ\u{dd4}ක\u{dcf}රෙස\u{dca}ට\u{dca}"), ("sk", "Bukurešť"), ("sl", "Bukarešta"), ("so", "Buqarest"), ("sq", "Bukureshti"), ("sr", "Букурешт"), ("sr_Latn", "Bukurešt"), ("sv", "Bukarest"), ("sw", "Bukarest"), ("ta", "புக\u{bcd}கரெஸ\u{bcd}ட\u{bcd}"), ("te", "బుఖ\u{c3e}ర\u{c46}స\u{c4d}ట\u{c4d}"), ("th", "บ\u{e39}คาเรสต\u{e4c}"), ("tk", "Buharest"), ("tr", "Bükreş"), ("uk", "Бухарест"), ("ur", "بخارسٹ"), ("uz", "Buharest"), ("vi", "Bucharest"), ("yo", "Bucharest"), ("yo_BJ", "Bucharest"), ("yue", "布加勒斯特"), ("yue_Hans", "布加勒斯特"), ("zh", "布加勒斯特")]),
                        unofficial_name_list: ["Bucarest", "Bucarest", "Bucuresti", "Bucureşti", "Bukarest"].to_vec(),
                    }
                ),
                (
                    "BC",
                    Subdivision{
                        name: "Bacau",
                        country_alpha2: Alpha2::RO,
                        code: "BC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.583333), longitude: Some(26.916667), max_latitude: Some(46.6207201), min_latitude: Some(46.5073649), max_longitude: Some(26.9547951), min_longitude: Some(26.8629241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم باكاو"), ("be", "Бакэў"), ("bg", "Бакъу"), ("ca", "Província de Bacău"), ("ccp", "𑄝𑄇𑄬𑄅\u{1112a}"), ("ceb", "Județul Bacău"), ("cs", "Bacău"), ("da", "Bacău"), ("de", "Kreis Bacău"), ("en", "Bacău"), ("es", "Bacău"), ("et", "Bacău maakond"), ("fa", "شهرستان باکائو"), ("fi", "Bacău"), ("fr", "județ de Bacău"), ("gl", "Condado de Bacău"), ("he", "מחוז בוזאו"), ("hr", "Bacău"), ("hu", "Bákó megye"), ("id", "Provinsi Bacău"), ("it", "distretto di Bacău"), ("ja", "バカウ県"), ("ka", "ბაკეუს ჟუდეცი"), ("ko", "바커우 주"), ("lt", "Bakeu apskritis"), ("mk", "Бакау"), ("mn", "Бакау аймаг"), ("ms", "Daerah Bacău"), ("nb", "Bacău"), ("nl", "District Bacău"), ("no", "Bacău"), ("pl", "Okręg Bacău"), ("pt", "Bacău"), ("ro", "Bacău"), ("ru", "Бакэу"), ("sk", "Bacău"), ("sl", "okrožje Bacău"), ("sq", "Qarku Bacău"), ("sr", "Бакау"), ("sr_Latn", "Bakau"), ("sv", "Bacău"), ("tr", "Bacău ili"), ("uk", "Бакеу"), ("ur", "باکئو کاؤنٹی"), ("vi", "Bacău"), ("zh", "巴克烏縣")]),
                        unofficial_name_list: ["Bacau"].to_vec(),
                    }
                ),
                (
                    "BH",
                    Subdivision{
                        name: "Bihor",
                        country_alpha2: Alpha2::RO,
                        code: "BH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.01575159999999), longitude: Some(22.172266), max_latitude: Some(47.607922), min_latitude: Some(46.38481789999999), max_longitude: Some(22.816566), min_longitude: Some(21.427627)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيهور"), ("bg", "Бихор"), ("bn", "বিহোর ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Bihor"), ("ccp", "𑄝\u{11128}𑄦\u{1112e}𑄢\u{11134}"), ("ceb", "Bihor (lalawigan)"), ("cs", "Bihor"), ("da", "Bihor"), ("de", "Kreis Bihor"), ("el", "Μπιχόρ"), ("en", "Bihor"), ("es", "Bihor"), ("et", "Bihori maakond"), ("eu", "Bihor konderria"), ("fa", "شهرستان بیهور"), ("fi", "Bihor"), ("fr", "județ de Bihor"), ("gl", "Condado de Bihor"), ("gu", "બિહોર કાઉન\u{acd}ટી"), ("he", "מחוז ביחור"), ("hi", "बिहोर काउ\u{902}टी"), ("hr", "Bihor"), ("hu", "Bihar megye"), ("id", "Provinsi Bihor"), ("it", "distretto di Bihor"), ("ja", "ビホル県"), ("ka", "ბიჰორის ჟუდეცი"), ("kn", "ಬ\u{cbf}ಹೋರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "비호르 주"), ("lt", "Bihoro apskritis"), ("lv", "Bihoras žudecs"), ("mk", "Бихор"), ("mn", "Бихор аймаг"), ("mr", "बिहोर काउ\u{902}टी"), ("ms", "Wilayah Bihor"), ("nb", "Bihor"), ("nl", "District Bihor"), ("no", "Bihor"), ("pl", "Okręg Bihor"), ("pt", "Bihor"), ("ro", "Bihor"), ("ru", "Бихор"), ("si", "බ\u{dd2}හෝර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Bihor"), ("sq", "Qarku Bihor"), ("sr", "Бихор"), ("sr_Latn", "Bihor"), ("sv", "Bihor"), ("ta", "பிஹ\u{bbe}ர\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c3f}హ\u{c4b}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบ\u{e35}ฮอร\u{e4c}"), ("tr", "Bihor ili"), ("uk", "Біхор"), ("ur", "بیہور کاؤنٹی"), ("vi", "Bihor"), ("zh", "比霍爾縣")]),
                        unofficial_name_list: ["Bihor"].to_vec(),
                    }
                ),
                (
                    "BN",
                    Subdivision{
                        name: "Bistrita-Nasaud",
                        country_alpha2: Alpha2::RO,
                        code: "BN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2486107), longitude: Some(24.5322814), max_latitude: Some(47.6071391), min_latitude: Some(46.7517591), max_longitude: Some(25.0916189), min_longitude: Some(23.9229529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيستريتا-ناسود"), ("bg", "Бистрица-Нъсъуд"), ("bn", "বিস\u{9cd}ত\u{9cd}রিৎস\u{9be}-ন\u{9be}স\u{9be}উদ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Bistrița-Năsăud"), ("ccp", "𑄝\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄑-𑄚𑄥𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "Județul Bistrița-Năsăud"), ("cs", "Bistrița-Năsăud"), ("da", "Bistrița-Năsăud"), ("de", "Kreis Bistrița-Năsăud"), ("el", "Μπιστρίτα-Νασόντ"), ("en", "Bistriţa-Năsăud"), ("es", "Bistrița-Năsăud"), ("et", "Bistrița-Năsăudi maakond"), ("eu", "Bistriţa-Năsăud konderria"), ("fa", "شهرستان بیستریتسا-نسئود"), ("fi", "Bistrița-Năsăud"), ("fr", "Județ de Bistrița-Năsăud"), ("gl", "Condado de Bistriţa-Năsăud"), ("gu", "બિસ\u{acd}ત\u{acd}રિટા-નસૌદ કાઉન\u{acd}ટી"), ("he", "ביסטריצה-נסאוד"), ("hi", "बिस\u{94d}ट\u{94d}रिटा-नसाऊद काउ\u{902}टी"), ("hr", "Bistriţa-Năsăud"), ("hu", "Beszterce-Naszód megye"), ("id", "Provinsi Bistriţa-Năsăud"), ("it", "distretto di Bistrița-Năsăud"), ("ka", "ბისტრიცა-ნესეუდის ჟუდეცი"), ("kn", "ಬ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಟಾ-ನಾಸ\u{ccc}ಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "비스트리차너서우드 주"), ("lt", "Bistricos-Neseudo apskritis"), ("lv", "Bistricas-Neseudas žudecs"), ("mk", "Бистрица-Насауд"), ("mn", "Бистрица-Насауд аймаг"), ("mr", "बिस\u{94d}त\u{94d}रिया-नसाउड काउ\u{902}टी"), ("ms", "Wilayah Bistrița-Năsăud"), ("nb", "Bistrița-Năsăud"), ("nl", "Bistrița-Năsăud"), ("no", "Bistrița-Năsăud"), ("pl", "Okręg Bistriţa-Năsăud"), ("pt", "Bistrița-Năsăud"), ("ro", "Bistrița-Năsăud"), ("ru", "Бистрица-Нэсэуд"), ("si", "බ\u{dd2}ස\u{dca}ට\u{dca}\u{200d}ර\u{dca}ට\u{dcf}-න\u{dcf}ස\u{dcf}උඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Bistriţa-Năsăud"), ("sq", "Qarku Bistrița-Năsăud"), ("sr", "Бистрица-Насауд"), ("sr_Latn", "Bistrica-Nasaud"), ("sv", "Bistrița-Năsăud"), ("ta", "பிஸ\u{bcd}டரிட\u{bbe} -ந\u{bbe}ச\u{bbe}வுட\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c3f}స\u{c4d}ట\u{c4d}ర\u{c3f}ట\u{c3e} -న\u{c3e}స\u{c4c}ద\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ร\u{e31}ฐกะฉ\u{e34}\u{e48}น"), ("tr", "Bistrița-Năsăud ili"), ("uk", "Бистриця-Несеуд"), ("ur", "بیستریتسا-ناسائود کاؤنٹی"), ("vi", "Bistriţa-Năsăud"), ("zh", "比斯特里察-訥瑟烏德縣")]),
                        unofficial_name_list: ["Bistrita-Nasaud"].to_vec(),
                    }
                ),
                (
                    "BR",
                    Subdivision{
                        name: "Braila",
                        country_alpha2: Alpha2::RO,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.269194), longitude: Some(27.957472), max_latitude: Some(45.3137327), min_latitude: Some(45.1678028), max_longitude: Some(28.0004168), min_longitude: Some(27.8928493)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برايلا"), ("be", "Жудзец Брэіла"), ("bg", "Браила"), ("bn", "ব\u{9cd}র\u{9be}ইল\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Brăila"), ("ccp", "𑄝\u{11133}𑄢\u{1112d}𑄣\u{11134}"), ("ceb", "Județul Brăila"), ("cs", "Brăila"), ("da", "Brăila"), ("de", "Kreis Brăila"), ("el", "Βραΐλα"), ("en", "Brăila"), ("es", "Brăila"), ("et", "Brăila maakond"), ("fa", "شهرستان برئیلا"), ("fi", "Brăila"), ("fr", "Județ de Brăila"), ("gl", "Condado de Brăila"), ("gu", "બ\u{acd}ર\u{ac7}લા કાઉન\u{acd}ટી"), ("hi", "ब\u{94d}रायला काउ\u{902}टी"), ("hr", "Brăila"), ("hu", "Brăila megye"), ("id", "Provinsi Brăila"), ("it", "distretto di Brăila"), ("ja", "ブライラ県"), ("ka", "ბრეილის ჟუდეცი"), ("kn", "ಬ\u{ccd}ರೈಲ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "브러일라 주"), ("lt", "Breilos apskritis"), ("lv", "Breilas žudecs"), ("mk", "Браила"), ("mn", "Браила аймаг"), ("mr", "ब\u{94d}राझील काउ\u{902}टी"), ("ms", "Wilayah Brăila"), ("nb", "Brăila"), ("nl", "District Brăila"), ("no", "Brăila"), ("pl", "Okręg Braiła"), ("pt", "Brăila"), ("ro", "Brăila"), ("ru", "Брэила"), ("si", "බ\u{dca}\u{200d}රය\u{dd2}ල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Brăila"), ("sq", "Qarku Brăila"), ("sr", "Браила"), ("sr_Latn", "Braila"), ("sv", "Brăila"), ("ta", "பிரைள\u{bbe} கவுண\u{bcd}டி"), ("te", "బ\u{c4d}ర\u{c46}య\u{c3f}ల\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบราอ\u{e34}ลา"), ("tr", "Brăila ili"), ("uk", "Бреїла"), ("ur", "برئیلا کاؤنٹی"), ("vi", "Hạt Brăila"), ("zh", "布勒伊拉縣")]),
                        unofficial_name_list: ["Braila"].to_vec(),
                    }
                ),
                (
                    "BT",
                    Subdivision{
                        name: "Botosani",
                        country_alpha2: Alpha2::RO,
                        code: "BT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.748611), longitude: Some(26.669444), max_latitude: Some(47.7705485), min_latitude: Some(47.7045958), max_longitude: Some(26.709298), min_longitude: Some(26.609519)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بوتوشان"), ("be", "Баташані"), ("bg", "Ботошани"), ("bn", "বোতোস\u{9be}নি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Botoșani"), ("ccp", "𑄝\u{1112e}𑄑\u{1112e}𑄥𑄚\u{11128}"), ("ceb", "Județul Botoșani"), ("cs", "Botoșani"), ("da", "Botoșani"), ("de", "Kreis Botoșani"), ("el", "Μποτοσάνι"), ("en", "Botoşani"), ("es", "Botoșani"), ("et", "Botoșani maakond"), ("fa", "شهرستان بوتوشانی"), ("fi", "Botoșani"), ("fr", "județ de Botoșani"), ("gl", "Condado de Botoşani"), ("gu", "બોટોસાની કાઉન\u{acd}ટી"), ("he", "בוטושני"), ("hi", "बोतोसानी काउ\u{902}टी"), ("hr", "Botoşani"), ("hu", "Botoșani megye"), ("hy", "Բոտոշանի"), ("id", "Provinsi Botoşani"), ("it", "distretto di Botoșani"), ("ja", "ボトシャニ県"), ("ka", "ბოტოშანის ჟუდეცი"), ("kn", "ಬೊಟೊಸನ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "보토샤니 주"), ("lt", "Botošanio apskritis"), ("lv", "Botošanu žudecs"), ("mk", "Ботошан"), ("mn", "Ботошани аймаг"), ("mr", "बोटोसानी काउ\u{902}टी"), ("ms", "Wilayah Botoșani"), ("nb", "Botoșani"), ("nl", "District Botoșani"), ("no", "Botoșani"), ("pl", "Okręg Botoszany"), ("pt", "Botoşani"), ("ro", "Botoșani"), ("ru", "Ботошани"), ("si", "බෝටෝස\u{dcf}න\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Botoşani"), ("sl", "Botoşani"), ("sq", "Qarku Botoșani"), ("sr", "Ботошани"), ("sr_Latn", "Botošani"), ("sv", "Botoșani"), ("ta", "போட\u{bcd}டோ அணி கவுண\u{bcd}டி"), ("te", "బ\u{c4b}ట\u{c4b}స\u{c3e}న\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโบโตซาน\u{e35}"), ("tr", "Botoșani ili"), ("uk", "Ботошань"), ("ur", "بوتوشانی کاؤنٹی"), ("vi", "Botoşani"), ("zh", "博托沙尼縣")]),
                        unofficial_name_list: ["Botosani"].to_vec(),
                    }
                ),
                (
                    "BV",
                    Subdivision{
                        name: "Brasov",
                        country_alpha2: Alpha2::RO,
                        code: "BV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.666667), longitude: Some(25.616667), max_latitude: Some(45.72210200000001), min_latitude: Some(45.5828542), max_longitude: Some(25.6784767), min_longitude: Some(25.514449)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة براسوف"), ("bg", "Брашов"), ("bn", "ব\u{9cd}র\u{9be}সোভ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Brașov"), ("ccp", "𑄝\u{11133}𑄢𑄥\u{1112e}𑄛\u{11134}"), ("ceb", "Județul Brașov"), ("cs", "Brašov"), ("da", "Brașov"), ("de", "Kreis Brașov"), ("el", "Επαρχία Μπράσοβ"), ("en", "Braşov"), ("es", "Brașov"), ("et", "Brașovi maakond"), ("fa", "شهرستان براشوو"), ("fi", "Brașov"), ("fr", "Județ de Brașov"), ("gl", "Condado de Braşov"), ("gu", "બ\u{acd}રાસોવ કાઉન\u{acd}ટી"), ("hi", "ब\u{94d}रसोव काउ\u{902}टी"), ("hr", "Brașov"), ("hu", "Brassó megye"), ("id", "Provinsi Braşov"), ("it", "distretto di Brașov"), ("ja", "ブラショフ県"), ("ka", "ბრაშოვის ჟუდეცი"), ("kn", "ಬ\u{ccd}ರಾಸೊವ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "브라쇼브 주"), ("lt", "Brašovo apskritis"), ("lv", "Brasovas apgabals"), ("mk", "Брашов"), ("mn", "Брашов аймаг"), ("mr", "ब\u{94d}र\u{945}सोव\u{94d}ह काउ\u{902}टी"), ("ms", "Wilayah Brașov"), ("nb", "Brașov"), ("nl", "District Brașov"), ("no", "Brașov"), ("pl", "Okręg Braszów"), ("pt", "Brașov"), ("ro", "Brașov"), ("ru", "Брашов"), ("si", "බ\u{dca}\u{200d}රසෝව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Braşov"), ("sq", "Qarku Brașov"), ("sr", "Брашов"), ("sr_Latn", "Brašov"), ("sv", "Brașov"), ("ta", "ப\u{bcd}ரஸ\u{bcd}ஒவ\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c4d}ర\u{c3e}స\u{c4b}వ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบราซอฟ"), ("tr", "Brașov ili"), ("uk", "Брашов"), ("ur", "براشوو کاؤنٹی"), ("vi", "Brașov"), ("zh", "布拉索夫縣")]),
                        unofficial_name_list: ["Brasov"].to_vec(),
                    }
                ),
                (
                    "BZ",
                    Subdivision{
                        name: "Buzau",
                        country_alpha2: Alpha2::RO,
                        code: "BZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.166667), longitude: Some(26.816667), max_latitude: Some(45.1814771), min_latitude: Some(45.1162977), max_longitude: Some(26.8720435), min_longitude: Some(26.7563438)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بوزاو"), ("bg", "Бузъу"), ("bn", "ব\u{9c1}জ\u{9be}উ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Buzău"), ("ccp", "𑄝\u{1112a}𑄎𑄅\u{1112a}"), ("ceb", "Județul Buzău"), ("cs", "Buzău"), ("da", "Buzău"), ("de", "Kreis Buzău"), ("el", "Μπουζάου"), ("en", "Buzău"), ("es", "Buzău"), ("et", "Buzău maakond"), ("eu", "Buzau"), ("fa", "شهرستان بوزائو"), ("fi", "Buzău"), ("fr", "Județ de Buzău"), ("gl", "Condado de Buzău"), ("gu", "બ\u{ac1}ઝ\u{ac1} કાઉન\u{acd}ટી"), ("hi", "ब\u{941}ज\u{93c}\u{942} काउ\u{902}टी"), ("hr", "Buzău"), ("hu", "Buzău megye"), ("id", "Provinsi Buzău"), ("it", "distretto di Buzău"), ("ja", "ブザウ県"), ("ka", "ბუზეუს ჟუდეცი"), ("kn", "ಬುಜ\u{ccc} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "부저우 주"), ("lt", "Buzeu apskritis"), ("lv", "Buzeu žudecs"), ("mk", "Бузау"), ("mn", "Бузау аймаг"), ("mr", "ब\u{941}झ\u{942} काउ\u{902}टी"), ("ms", "Wilayah Buzău"), ("nb", "Buzău"), ("nl", "District Buzău"), ("no", "Buzău"), ("pl", "Okręg Buzău"), ("pt", "Buzău"), ("ro", "Buzău"), ("ru", "Бузэу"), ("si", "බ\u{dd4}සෞ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Buzău"), ("sr", "Бузау"), ("sr_Latn", "Buzau"), ("sv", "Buzău"), ("ta", "புச\u{bbe}உ கவுண\u{bcd}டி"), ("te", "బుజ\u{c3e}వ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "บ\u{e39}เซาเคาน\u{e4c}ต\u{e35}"), ("tr", "Buzău ili"), ("uk", "Бузеу"), ("ur", "بوزاو کاؤنٹی"), ("vi", "Hạt Buzau"), ("zh", "布澤烏縣")]),
                        unofficial_name_list: ["Buzau"].to_vec(),
                    }
                ),
                (
                    "CJ",
                    Subdivision{
                        name: "Cluj",
                        country_alpha2: Alpha2::RO,
                        code: "CJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.766667), longitude: Some(23.583333), max_latitude: Some(46.8612951), min_latitude: Some(46.7054762), max_longitude: Some(23.7104809), min_longitude: Some(23.522544)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كلوج"), ("be", "Клуж"), ("bg", "Клуж"), ("bn", "ক\u{9cd}ল\u{9c1}জ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Cluj"), ("ccp", "𑄇\u{11133}𑄣\u{1112a}𑄌\u{11134}"), ("ceb", "Județul Cluj"), ("cs", "Kluž"), ("da", "Cluj"), ("de", "Kreis Cluj"), ("el", "Κλούτζ"), ("en", "Cluj"), ("es", "Cluj"), ("et", "Cluji maakond"), ("eu", "Cluj konderria"), ("fa", "شهرستان کلوژ"), ("fi", "Cluj"), ("fr", "Județ de Cluj"), ("gl", "Condado de Cluj"), ("gu", "ક\u{acd}લ\u{ac1}જ કાઉન\u{acd}ટી"), ("he", "מחוז קלוז׳"), ("hi", "क\u{94d}ल\u{941}ए काउ\u{902}टी"), ("hr", "Cluj"), ("hu", "Kolozs megye"), ("id", "Provinsi Cluj"), ("it", "distretto di Cluj"), ("ja", "クルジュ県"), ("ka", "კლუჟის ჟუდეცი"), ("kn", "ಕ\u{ccd}ಲ\u{cc2}ಜ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "클루지 주"), ("lt", "Klužo apskritis"), ("lv", "Klužas žudecs"), ("mk", "Клуж"), ("mn", "Клуж аймаг"), ("mr", "क\u{94d}ल\u{941}ज काउ\u{902}टी"), ("ms", "Wilayah Cluj"), ("nb", "Cluj"), ("nl", "District Cluj"), ("no", "Cluj"), ("pl", "Okręg Kluż"), ("pt", "Cluj"), ("ro", "Cluj"), ("ru", "Клуж"), ("si", "ක\u{dca}ලජ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Cluj"), ("sr", "Клуж"), ("sr_Latn", "Kluž"), ("sv", "Cluj"), ("ta", "கிலுஜ\u{bcd} கவுண\u{bcd}டி"), ("te", "క\u{c4d}లజ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "คล\u{e39}ช"), ("tr", "Cluj"), ("uk", "Клуж"), ("ur", "کلوژ کاؤنٹی"), ("vi", "Hạt Cluj"), ("zh", "克魯日縣")]),
                        unofficial_name_list: ["Cluj"].to_vec(),
                    }
                ),
                (
                    "CL",
                    Subdivision{
                        name: "Calarasi",
                        country_alpha2: Alpha2::RO,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.2), longitude: Some(27.333333), max_latitude: Some(44.23818929999999), min_latitude: Some(44.1794956), max_longitude: Some(27.3725225), min_longitude: Some(27.277003)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالاراسي"), ("be", "жудзец Кэлэрашы"), ("bg", "Кълъраш"), ("bn", "ক\u{9be}ল\u{9be}র\u{9be}সি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Călăraşi"), ("ccp", "𑄇\u{11133}𑄠𑄣𑄢𑄥\u{11128}"), ("ceb", "Județul Călărași"), ("cs", "Călărași"), ("da", "Călărași"), ("de", "Kreis Călărași"), ("el", "Καλαράσι"), ("en", "Călărași"), ("es", "Călărași"), ("et", "Călărași maakond"), ("fa", "شهرستان کالارشی"), ("fi", "Călărași"), ("fr", "Județ de Călărași"), ("gl", "Condado de Călăraşi"), ("gu", "કાલારાસી કાઉન\u{acd}ટી"), ("hi", "कालारसी काउ\u{902}टी"), ("hr", "Călăraşi"), ("hu", "Călărași megye"), ("id", "Provinsi Călăraşi"), ("it", "distretto di Călărași"), ("ja", "カララシ県"), ("ka", "კელერაშის ჟუდეცი"), ("kn", "ಕ\u{ccd}ಯಾಲಾರಾಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "컬러라시 주"), ("lt", "Kelerašio apskritis"), ("lv", "Kelerašu žudecs"), ("mk", "Калараш"), ("mn", "Калараш аймаг"), ("mr", "क\u{945}ल\u{945}सी काउ\u{902}टी"), ("ms", "Wilayah Călărași"), ("nb", "Călărași"), ("nl", "District Călărași"), ("no", "Călărași"), ("pl", "Okręg Călăraşi"), ("pt", "Călăraşi"), ("ro", "Călărași"), ("ru", "Кэлэраши"), ("si", "ක\u{dcf}ල\u{dcf}රස\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Călăraşi"), ("sr", "Калараши"), ("sr_Latn", "Kalaraši"), ("sv", "Călărași"), ("ta", "கலர\u{bbe}சி கவுண\u{bcd}டி"), ("te", "క\u{c3e}ల\u{c3e}రస\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "คาลาราส\u{e34} ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Călărași ili"), ("uk", "Келераш"), ("ur", "کالاراشی کاؤنٹی"), ("vi", "Hạt Calarasi"), ("zh", "克勒拉希縣")]),
                        unofficial_name_list: ["Calarasi"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "Caras-Severin",
                        country_alpha2: Alpha2::RO,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1139646), longitude: Some(22.0740993), max_latitude: Some(45.67003099999999), min_latitude: Some(44.589462), max_longitude: Some(22.7100121), min_longitude: Some(21.3522489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاراس-سيفيرين"), ("be", "жудзец Караш-Северын"), ("bg", "Караш-Северин"), ("bn", "ক\u{9be}র\u{9be}স-সেভ\u{9be}রিন ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Caraş-Severin"), ("ccp", "𑄇𑄢𑄌\u{11134}-𑄥𑄬𑄞𑄬𑄢\u{11128}𑄚\u{11134}"), ("ceb", "Județul Caraș-Severin"), ("cs", "Caraș-Severin"), ("da", "Caraș-Severin"), ("de", "Kreis Caraș-Severin"), ("el", "Καράς-Σεβερίν"), ("en", "Caraș-Severin"), ("es", "Caraș-Severin"), ("et", "Caraș-Severini maakond"), ("fa", "شهرستان کاراش-سورین"), ("fi", "Caraș-Severin"), ("fr", "Județ de Caraș-Severin"), ("gl", "Condado de Caraş-Severin"), ("gu", "કરસ-સ\u{ac7}વ\u{ac7}રીન કાઉન\u{acd}ટી"), ("hi", "करास-स\u{947}व\u{947}रिन काउ\u{902}टी"), ("hr", "Karaš-severinska županija"), ("hu", "Krassó-Szörény megye"), ("id", "Provinsi Caraş-Severin"), ("it", "distretto di Caraș-Severin"), ("ka", "კარაშ-სევერინის ჟუდეცი"), ("kn", "ಕಾರಾಸ\u{ccd}-ಸ\u{cc6}ವ\u{cc6}ರ\u{cbf}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "카라슈세베린 주"), ("lt", "Karašo-Severino apskritis"), ("lv", "Karašas-Severinas žudecs"), ("mk", "Караш-Северин"), ("mn", "Караш-Северин аймаг"), ("mr", "कार\u{94d}सा-स\u{947}व\u{94d}हरिन काउ\u{902}टी"), ("ms", "Wilayah Caraș-Severin"), ("nb", "Caraș-Severin"), ("nl", "Caraș-Severin"), ("no", "Caraș-Severin"), ("pl", "Okręg Caraş-Severin"), ("pt", "Caraş-Severin"), ("ro", "Caraș-Severin"), ("ru", "Караш-Северин"), ("si", "කර\u{dcf}ස\u{dca} -සේවෙර\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Caraş-Severin"), ("sr", "Караш-Северин"), ("sr_Latn", "Karaš-Severin"), ("sv", "Caraș-Severin"), ("ta", "க\u{bbe}ர\u{bbe}-சேவேரின\u{bcd} கவுண\u{bcd}டி"), ("te", "క\u{c3e}రస\u{c4d}-స\u{c46}వర\u{c3f}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "คาราส เซเวร\u{e34}น ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Caraș-Severin ili"), ("uk", "Караш-Северін"), ("ur", "کاراش-سیویرین کاؤنٹی"), ("vi", "Hạt Caras-Severin"), ("zh", "卡拉什-塞維林縣")]),
                        unofficial_name_list: ["Caras-Severin"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "Constanta",
                        country_alpha2: Alpha2::RO,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.173333), longitude: Some(28.638333), max_latitude: Some(44.2782744), min_latitude: Some(44.0842815), max_longitude: Some(28.7076575), min_longitude: Some(28.5510205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كونستانتسا"), ("be", "Канстанца"), ("bg", "Кюстенджа"), ("bn", "কনস\u{9cd}ট\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Constanţa"), ("ccp", "𑄇\u{11127}𑄚\u{11134}𑄑𑄚\u{11134}𑄑"), ("ceb", "Constanța"), ("cs", "Constanţa"), ("da", "Constanța"), ("de", "Kreis Constanța"), ("el", "Κονστάντα"), ("en", "Constanța"), ("es", "Constanța"), ("et", "Constanța maakond"), ("fa", "شهرستان کونستانسا"), ("fi", "Constanța"), ("fr", "Județ de Constanța"), ("gl", "Condado de Constanţa"), ("gu", "કોન\u{acd}સ\u{acd}ટાન\u{acd}ટા કાઉન\u{acd}ટી"), ("hi", "कॉस\u{94d}ट\u{948}\u{902}टा काउ\u{902}टी"), ("hr", "Constanţa"), ("hu", "Constanța megye"), ("id", "Provinsi Constanţa"), ("it", "distretto di Costanza"), ("ja", "コンスタンツァ県"), ("ka", "კონსტანცის ჟუდეცი"), ("kn", "ಕಾನ\u{ccd}ಸ\u{ccd}ಟಾನ\u{ccd}ಟಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "콘스탄차 주"), ("lt", "Konstancos apskritis"), ("lv", "Konstancas žudecs"), ("mk", "Констанца"), ("mn", "Констанца аймаг"), ("mr", "कॉन\u{94d}स\u{94d}टन\u{94d}टा काउ\u{902}टी"), ("ms", "Wilayah Constanța"), ("nb", "Constanța"), ("nl", "District Constanța"), ("no", "Constanța"), ("pl", "Okręg Konstanca"), ("pt", "Constanţa"), ("ro", "Constanța"), ("ru", "Констанца"), ("si", "කොන\u{dca}ස\u{dca}ටන\u{dca}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Constanţa"), ("sr", "Констанца"), ("sr_Latn", "Konstanca"), ("sv", "Constanța"), ("ta", "க\u{bbe}ன\u{bcd}ஸ\u{bcd}ட\u{bbe}ன\u{bcd}டை கவுண\u{bcd}டி"), ("te", "క\u{c3e}న\u{c4d}\u{200c}స\u{c4d}ట\u{c3e}ంట\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "คอสต\u{e31}นซา"), ("tr", "Köstence ili"), ("uk", "Констанца"), ("ur", "کونستانتسا کاؤنٹی"), ("vi", "Constanta"), ("zh", "康斯坦察縣")]),
                        unofficial_name_list: ["Constanta", "Konstanza"].to_vec(),
                    }
                ),
                (
                    "CV",
                    Subdivision{
                        name: "Covasna",
                        country_alpha2: Alpha2::RO,
                        code: "CV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.849167), longitude: Some(26.185278), max_latitude: Some(45.8621916), min_latitude: Some(45.83421149999999), max_longitude: Some(26.2171125), min_longitude: Some(26.1405088)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوفاسنا"), ("bg", "Ковасна"), ("bn", "কোভ\u{9be}স\u{9cd}ন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Covasna"), ("ccp", "𑄇\u{1112e}𑄞𑄌\u{11134}𑄚"), ("ceb", "Covasna"), ("cs", "Covasna"), ("da", "Covasna"), ("de", "Kreis Covasna"), ("el", "Κοβάσνα"), ("en", "Covasna"), ("es", "Covasna"), ("et", "Covasna maakond"), ("fa", "شهرستان کوواسنا"), ("fi", "Covasna"), ("fr", "județ de Covasna"), ("gl", "Condado de Covasna"), ("gu", "કોવાસ\u{acd}ના કાઉન\u{acd}ટી"), ("hi", "कोवासना काउ\u{902}टी"), ("hr", "Covasna"), ("hu", "Kovászna megye"), ("id", "Provinsi Covasna"), ("it", "distretto di Covasna"), ("ja", "コヴァスナ県"), ("ka", "კოვასნის ჟუდეცი"), ("kn", "ಕೊವಾಸ\u{ccd}ನಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "코바스나 주"), ("lt", "Kovasnos apskritis"), ("lv", "Kovasnas žudecs"), ("mn", "Ковасна аймаг"), ("mr", "कोव\u{94d}हाना काउ\u{902}टी"), ("ms", "Wilayah Covasna"), ("nb", "Covasna"), ("nl", "District Covasna"), ("no", "Covasna"), ("pl", "Okręg Covasna"), ("pt", "Covasna"), ("ro", "Covasna"), ("ru", "Ковасна"), ("si", "කොවස\u{dca}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Covasna"), ("sr", "Ковасна"), ("sr_Latn", "Kovasna"), ("sv", "Covasna"), ("ta", "கோவன கவுண\u{bcd}டி"), ("te", "క\u{c4b}వస\u{c4d}న\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "โควาสนา"), ("tr", "Covasna ili"), ("uk", "Ковасна"), ("ur", "کوواسنا کاؤنٹی"), ("vi", "Hạt Covasna"), ("zh", "科瓦斯納縣")]),
                        unofficial_name_list: ["Covasna"].to_vec(),
                    }
                ),
                (
                    "DB",
                    Subdivision{
                        name: "Dâmbovita",
                        country_alpha2: Alpha2::RO,
                        code: "DB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.9289893), longitude: Some(25.425385), max_latitude: Some(45.440805), min_latitude: Some(44.403276), max_longitude: Some(25.992037), min_longitude: Some(25.124594)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دامبوفيتا"), ("be", "жудзец Дымбавіца"), ("bg", "Дъмбовица"), ("bn", "ড\u{9be}ম\u{9cd}বোভিট\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Dâmbovița"), ("ccp", "𑄓𑄟\u{11134}𑄝\u{1112e}𑄞\u{11128}𑄑"), ("ceb", "Județul Dâmbovița"), ("cs", "Dâmbovița"), ("da", "Dâmbovița"), ("de", "Kreis Dâmbovița"), ("el", "Νταμποβίτα"), ("en", "Dâmbovița"), ("es", "Dâmbovița"), ("et", "Dâmbovița maakond"), ("fa", "شهرستان دمبوویتسا"), ("fi", "Dâmbovița"), ("fr", "Județ de Dâmbovița"), ("gl", "Condado de Dâmboviţa"), ("gu", "દામ\u{acd}બોવિટા કાઉન\u{acd}ટી"), ("he", "מחוז דמבוביצה"), ("hi", "दम\u{94d}बोविता काउ\u{902}टी"), ("hr", "Dâmboviţa"), ("hu", "Dâmbovița megye"), ("id", "Provinsi Dâmboviţa"), ("it", "distretto di Dâmbovița"), ("ja", "ドゥンボヴィツァ県"), ("ka", "დიმბოვიცის ჟუდეცი"), ("kn", "ಡಂಬೋವ\u{cbf}ಟಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "듬보비차 주"), ("lt", "Dimbovicos apskritis"), ("lv", "Dimbovicas žudecs"), ("mn", "Дымбовица аймаг"), ("mr", "दा\u{902}बोवित\u{94d}ता काउ\u{902}टी"), ("ms", "Wilayah Dâmbovița"), ("nb", "Dâmbovița"), ("nl", "District Dâmbovița"), ("no", "Dâmbovița"), ("pl", "Okręg Dymbowica"), ("pt", "Dâmboviţa"), ("ro", "Dâmbovița"), ("ru", "Дымбовица"), ("si", "ඩ\u{dcf}ම\u{dca}බෝව\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Dâmboviţa"), ("sr", "Дамбовица"), ("sr_Latn", "Dambovica"), ("sv", "Dâmbovița"), ("ta", "ட\u{bbe}ம\u{bcd}போவிட\u{bcd}ட கவுண\u{bcd}டி"), ("te", "డ\u{c3e}ంబ\u{c4b}వ\u{c3f}ట\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ดามโบว\u{e34}ต\u{e49}า ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Dâmbovița ili"), ("uk", "Димбовіца"), ("ur", "دیمبوویتسا کاؤنٹی"), ("vi", "Hạt Dambovita"), ("zh", "登博維察縣")]),
                        unofficial_name_list: ["Dambovita", "Dimbovita", "Dîmboviţa"].to_vec(),
                    }
                ),
                (
                    "DJ",
                    Subdivision{
                        name: "Dolj",
                        country_alpha2: Alpha2::RO,
                        code: "DJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.1623022), longitude: Some(23.6325054), max_latitude: Some(44.728214), min_latitude: Some(43.699738), max_longitude: Some(24.2646289), min_longitude: Some(22.8383393)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دولج"), ("be", "жудзец Долж"), ("bg", "Долж"), ("bn", "ডলি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Dolj"), ("ccp", "𑄘\u{1112e}𑄣\u{11134}"), ("ceb", "Dolj"), ("cs", "Dolj"), ("da", "Dolj"), ("de", "Kreis Dolj"), ("el", "Ντόλτζ"), ("en", "Dolj"), ("es", "Dolj"), ("et", "Dolji maakond"), ("fa", "شهرستان دولژ"), ("fi", "Dolj"), ("fr", "Județ de Dolj"), ("gl", "Condado de Dolj"), ("gu", "ડોલ\u{acd}જ કાઉન\u{acd}ટી"), ("he", "דולז׳"), ("hi", "डॉल\u{94d}ज काउ\u{902}टी"), ("hr", "Dolj"), ("hu", "Dolj megye"), ("id", "Provinsi Dolj"), ("it", "distretto di Dolj"), ("ja", "ドルジュ県"), ("ka", "დოლჟის ჟუდეცი"), ("kn", "ಡಾಲ\u{ccd}ಜ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "돌지 주"), ("lt", "Dolžo apskritis"), ("lv", "Dolžas žudecs"), ("mn", "Долж аймаг"), ("mr", "डोलज काउ\u{902}टी"), ("ms", "Wilayah Dolj"), ("nb", "Dolj"), ("nl", "District Dolj"), ("no", "Dolj"), ("pl", "Okręg Dolj"), ("pt", "Dolj"), ("ro", "Dolj"), ("ru", "Долж"), ("si", "ඩොල\u{dca}ජ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Dolj"), ("sr", "Долж"), ("sr_Latn", "Dolž"), ("sv", "Dolj"), ("ta", "ட\u{bbe}ஜ\u{bcd} கவுண\u{bcd}டி"), ("te", "డ\u{c4b}ల\u{c4d}జ\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "โดลจ\u{e4c}"), ("tr", "Dolj ili"), ("uk", "Долж"), ("ur", "دولژ کاؤنٹی"), ("vi", "Hạt Dolj"), ("zh", "多爾日縣")]),
                        unofficial_name_list: ["Dolj"].to_vec(),
                    }
                ),
                (
                    "GJ",
                    Subdivision{
                        name: "Gorj",
                        country_alpha2: Alpha2::RO,
                        code: "GJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.94855949999999), longitude: Some(23.2427079), max_latitude: Some(45.3546451), min_latitude: Some(44.568317), max_longitude: Some(23.8601481), min_longitude: Some(22.5596509)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غورج"), ("be", "Горж"), ("bg", "Горж"), ("bn", "গর\u{9cd}জ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Gorj"), ("ccp", "𑄉\u{1112e}𑄢\u{11134}"), ("ceb", "Gorj"), ("cs", "Gorj"), ("da", "Gorj"), ("de", "Kreis Gorj"), ("el", "Γκορτζ"), ("en", "Gorj"), ("es", "Gorj"), ("et", "Gorji maakond"), ("fa", "شهرستان گورژ"), ("fi", "Gorj"), ("fr", "Județ de Gorj"), ("gl", "Condado de Gorj"), ("gu", "ગોર\u{acd}જ કાઉન\u{acd}ટી"), ("he", "גורז׳"), ("hi", "गोर\u{94d}ज काउ\u{902}टी"), ("hr", "Gorj"), ("hu", "Gorj megye"), ("id", "Provinsi Gorj"), ("it", "distretto di Gorj"), ("ja", "ゴルジュ県"), ("ka", "გორჟის ჟუდეცი"), ("kn", "ಗೊರ\u{ccd}ಜ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "고르지 주"), ("lt", "Goržo apskritis"), ("lv", "Goržas žudecs"), ("mn", "Горж аймаг"), ("mr", "गोरज काउ\u{902}टी"), ("ms", "Wilayah Gorj"), ("nb", "Gorj"), ("nl", "District Gorj"), ("no", "Gorj"), ("pl", "Okręg Gorj"), ("pt", "Gorj"), ("ro", "Gorj"), ("ru", "Горж"), ("si", "ගොර\u{dca}ජ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Gorj"), ("sr", "Горж"), ("sr_Latn", "Gorž"), ("sv", "Gorj"), ("ta", "கோர\u{bcd}ஜ\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c4b}ర\u{c4d}జ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "หาดจอล\u{e4c}จ"), ("tr", "Gorj ili"), ("uk", "Горж"), ("ur", "گورژ کاؤنٹی"), ("vi", "Hạt Gorj"), ("zh", "戈爾日縣")]),
                        unofficial_name_list: ["Gorj"].to_vec(),
                    }
                ),
                (
                    "GL",
                    Subdivision{
                        name: "Galati",
                        country_alpha2: Alpha2::RO,
                        code: "GL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.423333), longitude: Some(28.0425), max_latitude: Some(45.484793), min_latitude: Some(45.3905241), max_longitude: Some(28.1434536), min_longitude: Some(27.9515362)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم غالاتس"), ("be", "Галац"), ("bg", "Галац"), ("bn", "গ\u{9be}লোতি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Galaţi"), ("ccp", "𑄉𑄣𑄑\u{11128}"), ("ceb", "Județul Galați"), ("cs", "Galați"), ("da", "Galaţi"), ("de", "Kreis Galați"), ("el", "Γκαλάτι"), ("en", "Galați"), ("es", "Galați"), ("et", "Galați maakond"), ("fa", "شهرستان گالاتسی"), ("fi", "Galați"), ("fr", "Județ de Galați"), ("gl", "Condado de Galaţi"), ("gu", "ગલાટી કાઉન\u{acd}ટી"), ("he", "מחוז גאלאץ"), ("hi", "गलाती काउ\u{902}टी"), ("hr", "Galaţi"), ("hu", "Galați megye"), ("id", "Provinsi Galaţi"), ("it", "distretto di Galați"), ("ja", "ガラツィ県"), ("ka", "გალაცის ჟუდეცი"), ("kn", "ಗ\u{ccd}ಯಾಲಾಟ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "갈라치 주"), ("lt", "Galacio apskritis"), ("lv", "Galacas žudecs"), ("mn", "Галац аймаг"), ("mr", "गालिटी काउ\u{902}टी"), ("ms", "Wilayah Galaţi"), ("nb", "Galați"), ("nl", "District Galați"), ("no", "Galați"), ("pl", "Okręg Gałacz"), ("pt", "Galaţi"), ("ro", "Galați"), ("ru", "Галац"), ("si", "ගලට\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Galaţi"), ("sr", "Галац"), ("sr_Latn", "Galac"), ("sv", "Galați"), ("ta", "க\u{bbe}ல\u{bbe}டி கவுண\u{bcd}டி"), ("te", "గల\u{c3e}ట\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลกาลาต\u{e34}"), ("tr", "Galați ili"), ("uk", "Галац"), ("ur", "گالاتسی کاؤنٹی"), ("vi", "Hạt Galati"), ("zh", "加拉茨縣")]),
                        unofficial_name_list: ["Galati", "Galatz"].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "Giurgiu",
                        country_alpha2: Alpha2::RO,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.900833), longitude: Some(25.973889), max_latitude: Some(43.9360169), min_latitude: Some(43.8592715), max_longitude: Some(26.0299587), min_longitude: Some(25.8950758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جورجيو"), ("bg", "Гюргево"), ("bn", "গিউরগিউ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Giurgiu"), ("ccp", "𑄎\u{11128}𑄠𑄢\u{11134}𑄎\u{11128}𑄅\u{1112a}"), ("ceb", "Giurgiu"), ("cs", "Giurgiu"), ("da", "Giurgiu"), ("de", "Kreis Giurgiu"), ("el", "Γκιούρτζιου"), ("en", "Giurgiu"), ("es", "Giurgiu"), ("et", "Giurgiu maakond"), ("fa", "شهرستان جورجی"), ("fi", "Giurgiu"), ("fr", "Județ de Giurgiu"), ("gl", "Condado de Giurgiu"), ("gu", "ગિર\u{acd}ગિય\u{ac1} કાઉન\u{acd}ટી"), ("hi", "जिर\u{94d}गिउ काउ\u{902}टी"), ("hr", "Giurgiu"), ("hu", "Giurgiu megye"), ("id", "Provinsi Giurgiu"), ("it", "distretto di Giurgiu"), ("ja", "ジュルジュ県"), ("ka", "ჯურჯუს ჟუდეცი"), ("kn", "ಗ\u{cbf}ರ\u{ccd}ಗುಯ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "지우르지우 주"), ("lt", "Džurdžu apskritis"), ("lv", "Džurdžu žudecs"), ("mn", "Журжу аймаг"), ("mr", "गिर\u{94d}गिउ काउ\u{902}टी"), ("ms", "Wilayah Giurgiu"), ("nb", "Giurgiu"), ("nl", "District Giurgiu"), ("no", "Giurgiu"), ("pl", "Okręg Giurgiu"), ("pt", "Giurgiu"), ("ro", "Giurgiu"), ("ru", "Джурджу"), ("si", "ග\u{dd2}ය\u{dd4}ර\u{dca}ග\u{dd2}ය\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Giurgiu"), ("sr", "Ђурђу"), ("sr_Latn", "Đurđu"), ("sv", "Giurgiu"), ("ta", "கியூர\u{bcd}கியூ கவுண\u{bcd}டி"), ("te", "జ\u{c3f}యర\u{c4d}జ\u{c3f}యు క\u{c4c}ంట\u{c40}"), ("th", "จ\u{e39}ร\u{e4c}จ\u{e39}"), ("tr", "Yergöğü ili"), ("uk", "Джурджу"), ("ur", "جیورجو کاؤنٹی"), ("vi", "Hạt Giurgiu"), ("zh", "久爾久縣")]),
                        unofficial_name_list: ["Giurgiu"].to_vec(),
                    }
                ),
                (
                    "HD",
                    Subdivision{
                        name: "Hunedoara",
                        country_alpha2: Alpha2::RO,
                        code: "HD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.76972199999999), longitude: Some(22.920278), max_latitude: Some(45.7960152), min_latitude: Some(45.7303145), max_longitude: Some(22.9378867), min_longitude: Some(22.8775477)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هوندوارا"), ("be", "жудзец Хунедаара"), ("bg", "Хунедоара"), ("bn", "হ\u{9c1}নেদোয\u{9bc}\u{9be}র\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Hunedoara"), ("ccp", "𑄦\u{1112a}𑄚𑄬𑄓\u{1112e}𑄠𑄢"), ("ceb", "Hunedoara (lalawigan)"), ("cs", "Hunedoara"), ("da", "Hunedoara"), ("de", "Hunedoara"), ("el", "Χουνεντοάρα"), ("en", "Hunedoara"), ("es", "Hunedoara"), ("et", "Hunedoara maakond"), ("fa", "شهرستان هوندوارا"), ("fi", "Hunedoara"), ("fr", "județ de Hunedoara"), ("gl", "Condado de Hunedoara"), ("gu", "હ\u{ac1}ન\u{ac7}ડોઆરા કાઉન\u{acd}ટી"), ("hi", "ह\u{942}न\u{947}दोआरा काउ\u{902}टी"), ("hr", "Hunedoara"), ("hu", "Hunyad megye"), ("id", "Provinsi Hunedoara"), ("it", "distretto di Hunedoara"), ("ja", "フネドアラ県"), ("ka", "ჰუნედოარის ჟუდეცი"), ("kn", "ಹುನ\u{cc6}ಡೊರಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "후네도아라 주"), ("lt", "Hunedoaros apskritis"), ("lv", "Hunedoaras žudecs"), ("mn", "Хунедоара аймаг"), ("mr", "ह\u{941}नदओरा काउ\u{902}टी"), ("ms", "Wilayah Hunedoara"), ("nb", "Hunedoara"), ("nl", "District Hunedoara"), ("no", "Hunedoara"), ("pl", "Okręg Hunedoara"), ("pt", "Hunedoara"), ("ro", "Hunedoara"), ("ru", "Хунедоара"), ("si", "හ\u{dd4}නේඩොආර\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Hunedoara"), ("sq", "Qarku Hunedoara"), ("sr", "Хунедоара"), ("sr_Latn", "Hunedoara"), ("sv", "Hunedoara"), ("ta", "ஹுநேடோர\u{bbe} கவுண\u{bcd}டி"), ("te", "హున\u{c46}ండ\u{c4b}వర\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลฮ\u{e39}เนโดอารา"), ("tr", "Hunedoara ili"), ("uk", "Хунедоара"), ("ur", "ہونےدوارا کاؤنٹی"), ("vi", "Hạt Hunedoara"), ("zh", "胡內多阿拉縣")]),
                        unofficial_name_list: ["Hunedoara"].to_vec(),
                    }
                ),
                (
                    "HR",
                    Subdivision{
                        name: "Harghita",
                        country_alpha2: Alpha2::RO,
                        code: "HR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4928507), longitude: Some(25.6456696), max_latitude: Some(47.182226), min_latitude: Some(46.118927), max_longitude: Some(26.2926781), min_longitude: Some(24.838758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هارغيتا"), ("bg", "Харгита"), ("bn", "হ\u{9be}রঘিত\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Harghita"), ("ccp", "𑄦𑄢\u{11134}𑄊\u{11128}𑄑"), ("ceb", "Harghita"), ("cs", "Harghita"), ("da", "Harghita"), ("de", "Kreis Harghita"), ("el", "Χαργκίτα"), ("en", "Harghita"), ("es", "Harghita"), ("et", "Harghita maakond"), ("fa", "شهرستان هارگیتا"), ("fi", "Harghita"), ("fr", "Județ de Harghita"), ("gl", "Condado de Harghita"), ("gu", "હાર\u{acd}ઘિટા કાઉન\u{acd}ટી"), ("hi", "हरघिता काउ\u{902}टी"), ("hr", "Harghita"), ("hu", "Hargita megye"), ("id", "Provinsi Harghita"), ("it", "distretto di Harghita"), ("ja", "ハルギタ県"), ("ka", "ჰარგიტის ჟუდეცი"), ("kn", "ಹರ\u{ccd}ಘ\u{cbf}ತ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "하르기타 주"), ("lt", "Hargitos apskritis"), ("lv", "Hargitas žudecs"), ("mn", "Харгита аймаг"), ("mr", "हरघिता काउ\u{902}टी"), ("ms", "Wilayah Harghita"), ("nb", "Harghita"), ("nl", "Harghita"), ("no", "Harghita"), ("pl", "Okręg Harghita"), ("pt", "Harghita"), ("ro", "Harghita"), ("ru", "Харгита"), ("si", "හර\u{dca}ඝ\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Harghita"), ("sr", "Харгита"), ("sr_Latn", "Hargita"), ("sv", "Harghita"), ("ta", "ஹ\u{bbe}ஜிஹிட கவுண\u{bcd}டி"), ("te", "హ\u{c3e}ర\u{c4d}గ\u{c3f}ట\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลฮาร\u{e4c}ก\u{e34}ต\u{e49}า"), ("tr", "Harghita ili"), ("uk", "Харгіта"), ("ur", "ہارگیتا کاؤنٹی"), ("vi", "Hạt Harghita"), ("zh", "哈爾吉塔縣")]),
                        unofficial_name_list: ["Harghita"].to_vec(),
                    }
                ),
                (
                    "IF",
                    Subdivision{
                        name: "Ilfov",
                        country_alpha2: Alpha2::RO,
                        code: "IF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.535548), longitude: Some(26.2324886), max_latitude: Some(44.767888), min_latitude: Some(44.21543), max_longitude: Some(26.454579), min_longitude: Some(25.823766)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إلفوف"), ("bg", "Илфов"), ("bn", "ইলফোভ"), ("ca", "Província d’Ilfov"), ("ccp", "𑄃\u{11128}𑄣\u{11134}𑄜\u{1112e}𑄛\u{11134}"), ("ceb", "Ilfov"), ("cs", "Ilfov"), ("da", "Ilfov"), ("de", "Kreis Ilfov"), ("el", "Ίλφοβ"), ("en", "Ilfov"), ("es", "Ilfov"), ("et", "Ilfovi maakond"), ("fa", "شهرستان ایلفوو"), ("fi", "Ilfov"), ("fr", "Județ d’Ilfov"), ("gl", "Condado de Ilfov"), ("gu", "ઇલ\u{acd}ફૉવ કાઉન\u{acd}ટી"), ("hi", "ल\u{947}फ\u{93c}ॉव काउ\u{902}टी"), ("hr", "Ilfov"), ("hu", "Ilfov megye"), ("id", "Provinsi Ilfov"), ("it", "distretto di Ilfov"), ("ja", "イルフォヴ県"), ("ka", "ილფოვის ჟუდეცი"), ("kn", "ಇಲ\u{ccd}ಫೊವ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "일포브 주"), ("lt", "Ilfovo apskritis"), ("lv", "Ilfovas žudecs"), ("mn", "Илфов аймаг"), ("mr", "इल\u{94d}फॉव काउ\u{902}टी"), ("ms", "Wilayah Ilfov"), ("nb", "Ilfov"), ("nl", "District Ilfov"), ("no", "Ilfov"), ("pl", "Okręg Ilfov"), ("pt", "Ilfov"), ("ro", "Ilfov"), ("ru", "Илфов"), ("si", "ඉල\u{dca}ෆොව\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ilfov"), ("sr", "Илфов"), ("sr_Latn", "Ilfov"), ("sv", "Ilfov"), ("ta", "இலபோவ\u{bcd} கவுண\u{bcd}டி"), ("te", "ఇల\u{c4d}ఫ\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "อ\u{e34}ลฟอพ ค\u{e31}นทร\u{e35}"), ("tr", "Ilfov ili"), ("uk", "Ілфов"), ("ur", "الیفوف کاؤنٹی"), ("vi", "Hạt Ilfov"), ("zh", "伊爾福夫縣")]),
                        unofficial_name_list: ["Sectorul Agricol Ilfov"].to_vec(),
                    }
                ),
                (
                    "IL",
                    Subdivision{
                        name: "Ialomita",
                        country_alpha2: Alpha2::RO,
                        code: "IL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.603133), longitude: Some(27.3789914), max_latitude: Some(44.86406700000001), min_latitude: Some(44.330837), max_longitude: Some(28.1100249), min_longitude: Some(26.300869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إيالوميتا"), ("bg", "Яломица"), ("bn", "ইয\u{9bc}\u{9be}লোমিত\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Ialomiţa"), ("ccp", "𑄃\u{1112d}𑄃𑄣\u{1112e}𑄟\u{11128}𑄑"), ("ceb", "Județul Ialomița"), ("cs", "Ialomița"), ("da", "Ialomița"), ("de", "Kreis Ialomița"), ("el", "Ιαλομίτα"), ("en", "Ialomița"), ("es", "Ialomița"), ("et", "Ialomița maakond"), ("fa", "شهرستان ایالومیتسا"), ("fi", "Ialomița"), ("fr", "Județ de Ialomița"), ("gl", "Condado de Ialomiţa"), ("gu", "ઇલોમીટા કાઉન\u{acd}ટી"), ("hi", "इआलोमिटा काउ\u{902}टी"), ("hr", "Ialomiţa"), ("hu", "Ialomița megye"), ("id", "Provinsi Ialomiţa"), ("it", "distretto di Ialomița"), ("ja", "ヤロミツァ県"), ("ka", "იალომიცის ჟუდეცი"), ("kn", "ಐಲೊಮ\u{cbf}ಟಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "이알로미차 주"), ("lt", "Jalomicos apskritis"), ("lv", "Jalomicas žudecs"), ("mn", "Яломица аймаг"), ("mr", "इलॉमीता काउ\u{902}टी"), ("ms", "Wilayah Ialomița"), ("nb", "Ialomiţa"), ("nl", "District Ialomița"), ("no", "Ialomiţa"), ("pl", "Okręg Jałomica"), ("pt", "Ialomiţa"), ("ro", "Ialomița"), ("ru", "Яломица"), ("si", "ලලොම\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ialomiţa"), ("sr", "Јаломица"), ("sr_Latn", "Jalomica"), ("sv", "Ialomița"), ("ta", "இயலொமிட\u{bbe} கவுண\u{bcd}டி"), ("te", "ల\u{c3e}ల\u{c4b}మ\u{c3f}త\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ลาโลม\u{e35}ตา ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Ialomița ili"), ("uk", "Яломіца"), ("ur", "یالومیتسا کاؤنٹی"), ("vi", "Hạt Ialomita"), ("zh", "雅洛米察縣")]),
                        unofficial_name_list: ["Ialomita"].to_vec(),
                    }
                ),
                (
                    "IS",
                    Subdivision{
                        name: "Iasi",
                        country_alpha2: Alpha2::RO,
                        code: "IS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.156944), longitude: Some(27.590278), max_latitude: Some(47.2274375), min_latitude: Some(47.08483709999999), max_longitude: Some(27.6969839), min_longitude: Some(27.4769569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم ياش"), ("be", "Ясы"), ("bg", "Яш"), ("bn", "ইয\u{9bc}\u{9be}সি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Iași"), ("ccp", "𑄃\u{1112d}𑄃𑄥\u{11128}"), ("ceb", "Județul Iași"), ("cs", "Iași"), ("da", "Iași"), ("de", "Kreis Iași"), ("el", "Ιασί"), ("en", "Iași"), ("es", "Iași"), ("et", "Iași maakond"), ("fa", "شهرستان ایاشی"), ("fi", "Iași"), ("fr", "Județ de Iași"), ("gl", "Condado de Iași"), ("gu", "ઇઆસી કાઉન\u{acd}ટી"), ("he", "מחוז יאשי"), ("hi", "इयासी काउ\u{902}टी"), ("hr", "Iaşi"), ("hu", "Iași megye"), ("id", "Provinsi Iaşi"), ("it", "distretto di Iași"), ("ja", "ヤシ県"), ("ka", "იასის ჟუდეცი"), ("kn", "ಇಯಾಸ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "이아시 주"), ("lt", "Jasų apskritis"), ("lv", "Jasu žudecs"), ("mk", "Јаш"), ("mn", "Яаш аймаг"), ("mr", "इसासी काउ\u{902}टी"), ("ms", "Wilayah Iași"), ("nb", "Iași"), ("nl", "District Iași"), ("no", "Iași"), ("pl", "Okręg Jassy"), ("pt", "Iaşi"), ("ro", "Iași"), ("ru", "Яссы"), ("si", "ඉය\u{dcf}ස\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Iaşi"), ("sr", "Јаши"), ("sr_Latn", "Jaši"), ("sv", "Iași"), ("ta", "லிச\u{bcd}சி கவுண\u{bcd}டி"), ("te", "అయ\u{c3e}స\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องยาช"), ("tr", "Yaş ili"), ("uk", "Ясси"), ("ur", "یاشی کاؤنٹی"), ("vi", "Hạt Iasi"), ("zh", "雅西縣")]),
                        unofficial_name_list: ["Iasi", "Jasch", "Jassy", "Yassy"].to_vec(),
                    }
                ),
                (
                    "MH",
                    Subdivision{
                        name: "Mehedinti",
                        country_alpha2: Alpha2::RO,
                        code: "MH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.5515053), longitude: Some(22.9044157), max_latitude: Some(45.108286), min_latitude: Some(44.0983461), max_longitude: Some(23.458584), min_longitude: Some(22.0000145)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ميهيدينتي"), ("be", "жудзец Мехедынцы"), ("bg", "Мехединци"), ("bn", "মেহেদিন\u{9cd}তি ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Mehedinți"), ("ccp", "𑄟𑄬𑄦𑄬𑄓\u{11128}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Județul Mehedinți"), ("cs", "Mehedinți"), ("da", "Mehedinţi"), ("de", "Kreis Mehedinți"), ("el", "Μεχεντίντι"), ("en", "Mehedinți"), ("es", "Mehedinți"), ("et", "Mehedinți maakond"), ("fa", "شهرستان مهدینتسی"), ("fi", "Mehedinți"), ("fr", "Județ de Mehedinți"), ("gl", "Condado de Mehedinţi"), ("gu", "મ\u{ac7}હ\u{ac7}ડિન\u{acd}ટી કાઉન\u{acd}ટી"), ("hi", "मह\u{947}दि\u{902}ती काउ\u{902}टी"), ("hr", "Mehedinţi"), ("hu", "Mehedinți megye"), ("id", "Provinsi Mehedinţi"), ("it", "distretto di Mehedinți"), ("ja", "メヘディンチ県"), ("ka", "მეჰედინცის ჟუდეცი"), ("kn", "ಮ\u{cc6}ಹ\u{cc6}ಥ\u{cbf}ಂಟ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "메헤딘치 주"), ("lt", "Mehedincio apskritis"), ("lv", "Mehedincu žudecs"), ("mn", "Мехединци аймаг"), ("mr", "म\u{947}ह\u{947}दिनटी काउ\u{902}टी"), ("ms", "Wilayah Mehedinți"), ("nb", "Mehedinți"), ("nl", "District Mehedinți"), ("no", "Mehedinți"), ("pl", "Okręg Mehedinţi"), ("pt", "Mehedinţi"), ("ro", "Mehedinți"), ("ru", "Мехединци"), ("si", "මෙහ\u{dca}ඩ\u{dd2}න\u{dca}ට\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mehedinţi"), ("sr", "Мехединци"), ("sr_Latn", "Mehedinci"), ("sv", "Mehedinți"), ("ta", "மெஹெடின\u{bcd}டி கவுண\u{bcd}டி"), ("te", "మ\u{c46}హ\u{c46}డ\u{c3f}న\u{c3f}ట\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เมเฮด\u{e34}นต\u{e34}"), ("tr", "Mehedinți ili"), ("uk", "Мехедінць"), ("ur", "میہیدینتسی کاؤنٹی"), ("vi", "Hạt Mehedinti"), ("zh", "梅赫丁茨縣")]),
                        unofficial_name_list: ["Mehedinti"].to_vec(),
                    }
                ),
                (
                    "MM",
                    Subdivision{
                        name: "Maramures",
                        country_alpha2: Alpha2::RO,
                        code: "MM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6737598), longitude: Some(23.7456285), max_latitude: Some(48.020276), min_latitude: Some(47.324372), max_longitude: Some(25.0557001), min_longitude: Some(22.965186)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "жудзец Марамурэш"), ("bg", "Марамуреш"), ("ca", "Província de Maramureș"), ("ccp", "𑄟𑄢𑄟\u{1112a}𑄢𑄬𑄌\u{11134}"), ("cs", "Maramureș"), ("da", "Maramureș"), ("de", "Kreis Maramureș"), ("en", "Maramureş"), ("es", "Maramureș"), ("et", "Maramureși maakond"), ("fa", "شهرستان مارامورش"), ("fi", "Maramureș"), ("fr", "Județ de Maramureș"), ("gl", "Condado de Maramureş"), ("he", "מחוז מרמורש"), ("hi", "मारम\u{941}र\u{947}स काउ\u{902}टी"), ("hr", "Maramureş"), ("hu", "Máramaros megye"), ("id", "Provinsi Maramureş"), ("it", "distretto di Maramureș"), ("ja", "マラムレシュ県"), ("ka", "მარამურეშის ჟუდეცი"), ("ko", "마라무레슈 주"), ("lt", "Maramurešo apskritis"), ("mn", "Марамуреш аймаг"), ("ms", "Wilayah Maramureș"), ("nb", "Maramureș"), ("nl", "District Maramureș"), ("no", "Maramureș"), ("pl", "Okręg Marmarosz"), ("pt", "Maramureș"), ("ro", "Maramureș"), ("ru", "Марамуреш"), ("sk", "Maramureş"), ("sr", "Марамуреш"), ("sr_Latn", "Maramureš"), ("sv", "Maramureș"), ("tr", "Maramureș ili"), ("uk", "Марамуреш"), ("ur", "ماراموریش کاؤنٹی"), ("zh", "馬拉穆列什縣")]),
                        unofficial_name_list: ["Maramures"].to_vec(),
                    }
                ),
                (
                    "MS",
                    Subdivision{
                        name: "Mures",
                        country_alpha2: Alpha2::RO,
                        code: "MS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5569904), longitude: Some(24.6723215), max_latitude: Some(47.143799), min_latitude: Some(46.075015), max_longitude: Some(25.3139161), min_longitude: Some(23.9577359)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة موريس"), ("be", "Мюрэш"), ("bg", "Муреш"), ("bn", "ম\u{9c1}র ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Mureș"), ("ccp", "𑄟\u{1112a}𑄢𑄬𑄌\u{11134}"), ("ceb", "Județul Mureș"), ("cs", "Mureș"), ("da", "Mureș"), ("de", "Kreis Mureș"), ("el", "Μουρές"), ("en", "Mureş"), ("es", "Mureș"), ("et", "Mureși maakond"), ("fa", "شهرستان مورش"), ("fi", "Mureș"), ("fr", "Județ de Mureș"), ("gl", "Condado de Mureş"), ("gu", "મ\u{ac1}ર\u{ac7}સ કાઉન\u{acd}ટી"), ("hi", "म\u{94d}य\u{942}र\u{947}स काउ\u{902}टी"), ("hr", "Mureș"), ("hu", "Maros megye"), ("id", "Provinsi Mureş"), ("it", "distretto di Mureș"), ("ja", "ムレシュ県"), ("ka", "მურეშის ჟუდეცი"), ("kn", "ಮ\u{ccc}ರ\u{cc6}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "무레슈 주"), ("lt", "Murešo apskritis"), ("lv", "Murešas žudecs"), ("mn", "Муреш аймаг"), ("mr", "मॉर\u{947}स काउ\u{902}टी"), ("ms", "Wilayah Mureș"), ("nb", "Mureș"), ("nl", "District Mureș"), ("no", "Mureș"), ("pl", "Okręg Marusza"), ("pt", "Mureș"), ("ro", "Mureș"), ("ru", "Муреш"), ("si", "ම\u{dd2}ය\u{dd4}රෙස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Mureş"), ("sr", "Муреш"), ("sr_Latn", "Mureš"), ("sv", "Mureș"), ("te", "మూర\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ม\u{e39}เรช"), ("tr", "Mureș ili"), ("uk", "Муреш"), ("ur", "موریش کاؤنٹی"), ("vi", "Hạt Mures"), ("zh", "穆列什縣")]),
                        unofficial_name_list: ["Mures"].to_vec(),
                    }
                ),
                (
                    "NT",
                    Subdivision{
                        name: "Neamt",
                        country_alpha2: Alpha2::RO,
                        code: "NT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.9758685), longitude: Some(26.3818764), max_latitude: Some(47.337545), min_latitude: Some(46.640951), max_longitude: Some(27.246063), min_longitude: Some(25.6630821)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نيامتس"), ("bg", "Нямц"), ("bn", "নিমট ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Neamț"), ("ccp", "𑄚\u{11128}𑄟\u{11134}𑄑\u{11134}"), ("ceb", "Județul Neamț"), ("cs", "Neamț"), ("da", "Neamț"), ("de", "Kreis Neamț"), ("el", "Κομητεία Νεάμτς"), ("en", "Neamţ"), ("es", "Neamț"), ("et", "Neamți maakond"), ("fa", "شهرستان نامس"), ("fi", "Neamț"), ("fr", "Județ de Neamț"), ("gl", "Condado de Neamţ"), ("gu", "ન\u{ac7}માટ કાઉન\u{acd}ટી"), ("he", "ניאמץ"), ("hi", "नीम\u{94d}ट काउ\u{902}टी"), ("hr", "Neamț"), ("hu", "Neamț megye"), ("id", "Provinsi Neamţ"), ("it", "distretto di Neamț"), ("ja", "ネアムツ県"), ("ka", "ნიამცის ჟუდეცი"), ("kn", "ನ\u{cc6}ಮ\u{ccd}ಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "네암츠 주"), ("lt", "Niamco apskritis"), ("lv", "Njamcas žudecs"), ("mn", "Нямц аймаг"), ("mr", "न\u{947}माट काउ\u{902}टी"), ("ms", "Wilayah Neamț"), ("nb", "Neamț"), ("nl", "District Neamț"), ("no", "Neamț"), ("pl", "Okręg Neamţ"), ("pt", "Neamţ"), ("ro", "Neamț"), ("ru", "Нямц"), ("si", "න\u{dd2}ම\u{dca}ට\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Neamț"), ("sr", "Њамц"), ("sr_Latn", "Njamc"), ("sv", "Neamț"), ("ta", "ந\u{bc0}ம\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "న\u{c40}మ\u{c4d}ట\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องแนมท\u{e4c}"), ("tr", "Neamț ili"), ("uk", "Нямц"), ("ur", "نامتس کاؤنٹی"), ("vi", "Hạt Neamt"), ("zh", "尼亞姆茨縣")]),
                        unofficial_name_list: ["Neamt"].to_vec(),
                    }
                ),
                (
                    "OT",
                    Subdivision{
                        name: "Olt",
                        country_alpha2: Alpha2::RO,
                        code: "OT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.200797), longitude: Some(24.5022981), max_latitude: Some(44.896967), min_latitude: Some(43.6820435), max_longitude: Some(24.8545519), min_longitude: Some(23.881877)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أولت"), ("be", "Олт"), ("bg", "Олт"), ("bn", "ওল\u{9cd}ট ক\u{9be}উন\u{9cd}টি"), ("ca", "Província d’Olt"), ("ccp", "𑄃\u{11127}𑄣\u{11134}𑄑\u{11134}"), ("ceb", "Olt (lalawigan)"), ("cs", "Olt"), ("da", "Olt"), ("de", "Kreis Olt"), ("el", "Όλτ"), ("en", "Olt"), ("es", "Olt"), ("et", "Olti maakond"), ("fa", "شهرستان اولت"), ("fi", "Olt"), ("fr", "Județ d’Olt"), ("gl", "Condado de Olt"), ("gu", "ઓલ\u{acd}ટ કાઉન\u{acd}ટી"), ("hi", "ओल\u{94d}ट काउ\u{902}टी"), ("hr", "Olt"), ("hu", "Olt megye"), ("id", "Provinsi Olt"), ("it", "distretto di Olt"), ("ja", "オルト県"), ("ka", "ოლტის ჟუდეცი"), ("kn", "ಓಲ\u{ccd}ಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "올트 주"), ("lt", "Olto apskritis"), ("lv", "Oltas žudecs"), ("mn", "Олт аймаг"), ("mr", "ओल\u{94d}ट काउ\u{902}टी"), ("ms", "Wilayah Olt"), ("nb", "Olt"), ("nl", "District Olt"), ("no", "Olt"), ("pl", "Okręg Aluta"), ("pt", "Olt"), ("ro", "Olt"), ("ru", "Олт"), ("si", "ඕල\u{dca}ට\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Olt"), ("sr", "Олт"), ("sr_Latn", "Olt"), ("sv", "Olt"), ("ta", "ஓல\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "ఓల\u{c4d}ట\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "โอลด\u{e4c} ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Olt ili"), ("uk", "Олт"), ("ur", "اولت کاؤنٹی"), ("vi", "Hạt Olt"), ("zh", "奧爾特縣")]),
                        unofficial_name_list: ["Olt"].to_vec(),
                    }
                ),
                (
                    "PH",
                    Subdivision{
                        name: "Prahova",
                        country_alpha2: Alpha2::RO,
                        code: "PH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.08919059999999), longitude: Some(26.0829312), max_latitude: Some(45.5147729), min_latitude: Some(44.692479), max_longitude: Some(26.6054759), min_longitude: Some(25.4514369)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يراهوفا"), ("be", "Прахова"), ("bg", "Прахова"), ("bn", "প\u{9cd}র\u{9be}হোভ\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Prahova"), ("ccp", "𑄛\u{11133}𑄢\u{11127}𑄦\u{1112e}𑄞"), ("cs", "Prahova"), ("da", "Prahova"), ("de", "Kreis Prahova"), ("el", "Πράχοβα"), ("en", "Prahova"), ("es", "Prahova"), ("et", "Prahova maakond"), ("fa", "شهرستان پراهووا"), ("fi", "Prahova"), ("fr", "Județ de Prahova"), ("gl", "Condado de Prahova"), ("gu", "પ\u{acd}રહોવા કાઉન\u{acd}ટી"), ("hi", "प\u{94d}रहोवा काउ\u{902}टी"), ("hr", "Prahova"), ("hu", "Prahova megye"), ("id", "Provinsi Prahova"), ("it", "distretto di Prahova"), ("ja", "プラホヴァ県"), ("ka", "პრაჰოვის ჟუდეცი"), ("kn", "ಪ\u{ccd}ರಹೋವಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "프라호바 주"), ("lt", "Prahovos apskritis"), ("lv", "Prahovas žudecs"), ("mn", "Прахова аймаг"), ("mr", "प\u{94d}र\u{947}हो काउ\u{902}टी"), ("ms", "Wilayah Prahova"), ("nb", "Prahova"), ("nl", "District Prahova"), ("no", "Prahova"), ("pl", "Okręg Prahova"), ("pt", "Prahova"), ("ro", "Prahova"), ("ru", "Прахова"), ("si", "ප\u{dca}\u{200d}රහොව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Prahova"), ("sr", "Прахова"), ("sr_Latn", "Prahova"), ("sv", "Prahova"), ("ta", "ப\u{bcd}ரஹோவ\u{bbe} கவுண\u{bcd}டி"), ("te", "ప\u{c4d}రహ\u{c4b}వ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดแซเนมาร\u{e4c}น"), ("tr", "Prahova ili"), ("uk", "Прахова"), ("ur", "پراہوا کاؤنٹی"), ("vi", "Hạt Prahova"), ("zh", "普拉霍瓦縣")]),
                        unofficial_name_list: ["Prahova"].to_vec(),
                    }
                ),
                (
                    "SB",
                    Subdivision{
                        name: "Sibiu",
                        country_alpha2: Alpha2::RO,
                        code: "SB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.792784), longitude: Some(24.152069), max_latitude: Some(45.8415964), min_latitude: Some(45.72439780000001), max_longitude: Some(24.2165279), min_longitude: Some(24.0631485)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيبيو"), ("be", "жудзец Сібіу"), ("bg", "Сибиу"), ("bn", "সিবিউ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Sibiu"), ("ccp", "𑄥\u{11128}𑄝\u{11128}𑄅\u{1112a}"), ("ceb", "Județul Sibiu"), ("cs", "Sibiu"), ("da", "Sibiu"), ("de", "Kreis Sibiu"), ("el", "Σίμπιου"), ("en", "Sibiu"), ("es", "Sibiu"), ("et", "Sibiu maakond"), ("eu", "Sibiu"), ("fa", "شهرستان سیبیو"), ("fi", "Sibiu"), ("fr", "Județ de Sibiu"), ("gl", "Condado de Sibiu"), ("gu", "સિબિય\u{ac1} કાઉન\u{acd}ટી"), ("he", "מחוז סיביו"), ("hi", "सिबिउ काउ\u{902}टी"), ("hr", "Sibiu"), ("hu", "Szeben megye"), ("id", "Provinsi Sibiu"), ("it", "distretto di Sibiu"), ("ja", "シビウ県"), ("ka", "სიბიუს ჟუდეცი"), ("kn", "ಸ\u{cbf}ಬ\u{cbf}ಯು ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "시비우 주"), ("lt", "Sibiu apskritis"), ("lv", "Sibiu žudecs"), ("mk", "Сибиу"), ("mn", "Сибиу аймаг"), ("mr", "सिबिय\u{941} काउ\u{902}टी"), ("ms", "Wilayah Sibiu"), ("nb", "Sibiu"), ("nl", "District Sibiu"), ("no", "Sibiu"), ("pl", "Okręg Sybin"), ("pt", "Sibiu"), ("ro", "Sibiu"), ("ru", "Сибиу"), ("si", "ස\u{dd2}බ\u{dd2}ය\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sibiu"), ("sr", "Сибињ"), ("sr_Latn", "Sibinj"), ("sv", "Sibiu"), ("ta", "சிபியு கவுண\u{bcd}டி"), ("te", "స\u{c3f}బ\u{c3f}యు క\u{c4c}ంట\u{c40}"), ("th", "ซ\u{e35}บ\u{e34}ว"), ("tr", "Sibiu ili"), ("uk", "Сібіу"), ("ur", "سیبیو کاؤنٹی"), ("vi", "Hạt Sibiu"), ("zh", "錫比烏縣")]),
                        unofficial_name_list: ["Sibiu"].to_vec(),
                    }
                ),
                (
                    "SJ",
                    Subdivision{
                        name: "Salaj",
                        country_alpha2: Alpha2::RO,
                        code: "SJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2090813), longitude: Some(23.2121901), max_latitude: Some(47.460111), min_latitude: Some(46.854099), max_longitude: Some(23.8409952), min_longitude: Some(22.493326)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سالاج"), ("be", "жудзец Сэлаж"), ("bg", "Сълаж"), ("bn", "স\u{9be}ল\u{9be}জ ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Sălaj"), ("ccp", "𑄥𑄣𑄌\u{11134}"), ("ceb", "Județul Sălaj"), ("cs", "Sălaj"), ("da", "Sălaj"), ("de", "Kreis Sălaj"), ("el", "Σάλατζ"), ("en", "Sălaj"), ("es", "Sălaj"), ("et", "Sălaji maakond"), ("fa", "شهرستان سلاژ"), ("fi", "Sălaj"), ("fr", "Județ de Sălaj"), ("gl", "Condado de Sălaj"), ("gu", "સાલજ કાઉન\u{acd}ટી"), ("he", "סלאז׳"), ("hi", "सलाय काउ\u{902}टी"), ("hr", "Sălaj"), ("hu", "Szilágy megye"), ("id", "Provinsi Sălaj"), ("it", "distretto di Sălaj"), ("ja", "サラージュ県"), ("ka", "სელაჟის ჟუდეცი"), ("kn", "ಸಾಲಾಜ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "설라지 주"), ("lt", "Selažo apskritis"), ("lv", "Selažas žudecs"), ("mn", "Салаж аймаг"), ("mr", "सलज काउ\u{902}टी"), ("ms", "Wilayah Sălaj"), ("nb", "Sălaj"), ("nl", "Sălaj"), ("no", "Sălaj"), ("pl", "Okręg Sălaj"), ("pt", "Sălaj"), ("ro", "Sălaj"), ("ru", "Сэлаж"), ("si", "ස\u{dcf}ල\u{dcf}ජ\u{dca} පළ\u{dcf}ත"), ("sk", "Sălaj"), ("sr", "Салаж"), ("sr_Latn", "Salaž"), ("sv", "Sălaj"), ("ta", "சலஜ\u{bcd} கவுண\u{bcd}டி"), ("te", "సల\u{c3e}జ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ซาลาจ"), ("tr", "Sălaj ili"), ("uk", "Селаж"), ("ur", "سالاژ کاؤنٹی"), ("vi", "Hạt Salaj"), ("zh", "瑟拉日縣")]),
                        unofficial_name_list: ["Salaj"].to_vec(),
                    }
                ),
                (
                    "SM",
                    Subdivision{
                        name: "Satu Mare",
                        country_alpha2: Alpha2::RO,
                        code: "SM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.79), longitude: Some(22.89), max_latitude: Some(47.8945642), min_latitude: Some(47.71166729999999), max_longitude: Some(22.9771327), min_longitude: Some(22.7901506)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساتو ماري"), ("be", "жудзец Сату-Марэ"), ("bg", "Сату Маре"), ("bn", "স\u{9be}ত\u{9c1} ম\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Satu Mare"), ("ccp", "𑄥𑄑\u{1112a} 𑄟𑄬𑄠𑄢\u{11134}"), ("ceb", "Satu Mare"), ("cs", "Satu Mare"), ("da", "Satu Mare"), ("de", "Kreis Satu Mare"), ("el", "επαρχία Σάτου Μάρε"), ("en", "Satu Mare"), ("es", "Satu Mare"), ("et", "Satu Mare maakond"), ("fa", "شهرستان ساتو ماره"), ("fi", "Satu Mare"), ("fr", "județ de Satu Mare"), ("gl", "Condado de Satu Mare"), ("gu", "સત\u{ac1} મ\u{ac7}ર કાઉન\u{acd}ટી"), ("he", "מחוז סאטו מארה"), ("hi", "सात\u{942} म\u{947}अर काउ\u{902}टी"), ("hr", "Satu Mare"), ("hu", "Szatmár megye"), ("hy", "Սատու Մարե"), ("id", "Provinsi Satu Mare"), ("it", "distretto di Satu Mare"), ("ja", "サトゥ・マーレ県"), ("ka", "სატუ-მარეს ჟუდეცი"), ("kn", "ಸತು ಮೇರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "사투마레 주"), ("lt", "Satu Marės apskritis"), ("lv", "Satumares žudecs"), ("mn", "Сату-Маре аймаг"), ("mr", "सात\u{942} म\u{947}अर काउ\u{902}टी"), ("ms", "Wilayah Satu Mare"), ("nb", "Satu Mare"), ("nl", "District Satu Mare"), ("no", "Satu Mare"), ("pl", "Okręg Satu Mare"), ("pt", "Satu Mare"), ("ro", "Satu Mare"), ("ru", "Сату-Маре"), ("si", "සට\u{dd4}මරේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Satu Mare"), ("sr", "Сату Маре"), ("sr_Latn", "Satu Mare"), ("sv", "Satu Mare"), ("ta", "ச\u{bbe}ட\u{bcd}டு ம\u{bbe}றே கவுண\u{bcd}டி"), ("te", "స\u{c3e}టు మ\u{c47}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลซาต\u{e39}มาเร"), ("tr", "Satu Mare ili"), ("uk", "Сату-Маре"), ("ur", "ساتو مارے کاؤنٹی"), ("vi", "Hạt Satu Mare"), ("zh", "薩圖馬雷縣")]),
                        unofficial_name_list: ["Satu Mare"].to_vec(),
                    }
                ),
                (
                    "SV",
                    Subdivision{
                        name: "Suceava",
                        country_alpha2: Alpha2::RO,
                        code: "SV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.65138899999999), longitude: Some(26.255556), max_latitude: Some(47.7191674), min_latitude: Some(47.6085356), max_longitude: Some(26.3217187), min_longitude: Some(26.2014914)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم سوتشيافا"), ("be", "жудзец Сучава"), ("bg", "Сучава"), ("bn", "স\u{9c1}চিভ\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Suceava"), ("ccp", "𑄥\u{1112a}𑄌\u{11128}𑄞"), ("ceb", "Suceava (lalawigan)"), ("cs", "Suceava"), ("da", "Suceava"), ("de", "Kreis Suceava"), ("el", "Σουκεάβα"), ("en", "Suceava"), ("es", "Suceava"), ("et", "Suceava maakond"), ("eu", "Suceava"), ("fa", "شهرستان سوچاوا"), ("fi", "Suceava"), ("fr", "Județ de Suceava"), ("gl", "Condado de Suceava"), ("gu", "સ\u{ac1}સ\u{ac7}વા કાઉન\u{acd}ટી"), ("he", "מחוז סוצ׳אבה"), ("hi", "स\u{941}स\u{947}वा काउ\u{902}टी"), ("hr", "Suceava"), ("hu", "Suceava megye"), ("id", "Provinsi Suceava"), ("it", "distretto di Suceava"), ("ja", "スチャヴァ県"), ("ka", "სუჩავის ჟუდეცი"), ("kn", "ಸ\u{cc2}ಸವ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "수체아바 주"), ("lt", "Sučavos apskritis"), ("lv", "Sučavas žudecs"), ("mn", "Сучава аймаг"), ("mr", "स\u{941}स\u{947}वा काउ\u{902}टी"), ("ms", "Wilayah Suceava"), ("nb", "Suceava"), ("nl", "District Suceava"), ("no", "Suceava"), ("pl", "Okręg Suczawa"), ("pt", "Suceava"), ("ro", "Suceava"), ("ru", "Сучава"), ("si", "ස\u{dd4}සේආව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Suceava"), ("sr", "Сучава"), ("sr_Latn", "Sučava"), ("sv", "Suceava"), ("ta", "சுச\u{bbe}வ\u{bbe} கவுண\u{bcd}டி"), ("te", "సుక\u{c3f}య\u{c3e}వ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ซ\u{e39}ซาวา"), ("tr", "Suceava ili"), ("uk", "Сучава"), ("ur", "سوچاوا کاؤنٹی"), ("vi", "Hạt Suceava"), ("zh", "蘇恰瓦縣")]),
                        unofficial_name_list: ["Suceava"].to_vec(),
                    }
                ),
                (
                    "TL",
                    Subdivision{
                        name: "Tulcea",
                        country_alpha2: Alpha2::RO,
                        code: "TL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1716165), longitude: Some(28.7914439), max_latitude: Some(45.2010679), min_latitude: Some(45.1574314), max_longitude: Some(28.8334142), min_longitude: Some(28.7479162)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تولسيا"), ("be", "жудзец Тулча"), ("bg", "Тулча"), ("bn", "ত\u{9c1}লসিয\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Tulcea"), ("ccp", "𑄑\u{1112a}𑄣\u{11134}𑄥\u{11128}"), ("ceb", "Tulcea"), ("cs", "Tulcea"), ("da", "Tulcea"), ("de", "Kreis Tulcea"), ("el", "Τουλκέα"), ("en", "Tulcea"), ("es", "Tulcea"), ("et", "Tulcea maakond"), ("fa", "شهرستان تولچا"), ("fi", "Tulcea"), ("fr", "Județ de Tulcea"), ("gl", "Condado de Tulcea"), ("gu", "ટ\u{ac1}લ\u{acd}સિઆ કાઉન\u{acd}ટી"), ("he", "טולצ׳ה"), ("hi", "टलसी काउ\u{902}टी"), ("hr", "Tulcea"), ("hu", "Tulcea megye"), ("id", "Provinsi Tulcea"), ("it", "distretto di Tulcea"), ("ja", "トゥルチャ県"), ("ka", "ტულჩის ჟუდეცი"), ("kn", "ತುಲ\u{ccd}ಸ\u{cbf}ಯ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "툴체아 주"), ("lt", "Tulčos apskritis"), ("lv", "Tulčas žudecs"), ("mn", "Тулча аймаг"), ("mr", "ट\u{941}लसीआ काउ\u{902}टी"), ("ms", "Wilayah Tulcea"), ("nb", "Tulcea"), ("nl", "District Tulcea"), ("no", "Tulcea"), ("pl", "Okręg Tulcza"), ("pt", "Tulcea"), ("ro", "Tulcea"), ("ru", "Тулча"), ("si", "ට\u{dd4}ලේස\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tulcea"), ("sr", "Тулча"), ("sr_Latn", "Tulča"), ("sv", "Tulcea"), ("ta", "துளசிய\u{bbe} கவுண\u{bcd}டி"), ("te", "టుల\u{c4d}స\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องต\u{e38}ลเซ"), ("tr", "Tulcea ili"), ("uk", "Тульча"), ("ur", "تولچا کاؤنٹی"), ("vi", "Hạt Tulcea"), ("zh", "圖爾恰縣")]),
                        unofficial_name_list: ["Tulcea"].to_vec(),
                    }
                ),
                (
                    "TM",
                    Subdivision{
                        name: "Timis",
                        country_alpha2: Alpha2::RO,
                        code: "TM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8138902), longitude: Some(21.3331055), max_latitude: Some(46.18994), min_latitude: Some(45.1918419), max_longitude: Some(22.5461279), min_longitude: Some(20.2617593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيميش"), ("be", "Тыміш"), ("bg", "Тимиш"), ("bn", "টিমিস ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Timiş"), ("ccp", "𑄑\u{1112d}𑄟\u{11128}𑄌\u{11134}"), ("ceb", "Județul Timiș"), ("cs", "Timiș"), ("da", "Timiș"), ("de", "Kreis Timiș"), ("el", "Τίμις"), ("en", "Timiș"), ("es", "Timiș"), ("et", "Timiși maakond"), ("fa", "شهرستان تیمیش"), ("fi", "Timiș"), ("fr", "județ de Timiș"), ("gl", "Condado de Timiş"), ("gu", "ટિમીશ કાઉન\u{acd}ટી"), ("he", "טימיש (מחוז)"), ("hi", "टिमिस काउ\u{902}टी"), ("hr", "Timiş"), ("hu", "Temes megye"), ("id", "Provinsi Timiş"), ("it", "distretto di Timiș"), ("ja", "ティミシュ県"), ("ka", "ტიმიშის ჟუდეცი"), ("kn", "ಟ\u{cbf}ಮ\u{cbf}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "티미슈 주"), ("lt", "Timišo apskritis"), ("lv", "Timišas žudecs"), ("mk", "Тимиш"), ("mn", "Тимиш аймаг"), ("mr", "तिमिस काउ\u{902}टी"), ("ms", "Wilayah Timiș"), ("nb", "Timiș"), ("nl", "District Timiș"), ("no", "Timiș"), ("pl", "Okręg Temesz"), ("pt", "Timiş"), ("ro", "Timiș"), ("ru", "Тимиш"), ("si", "ට\u{dd2}ම\u{dd2}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Timiş"), ("sq", "Qarku Timiș"), ("sr", "Тимиш"), ("sr_Latn", "Timiš"), ("sv", "Timiș"), ("ta", "டிமிஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "ట\u{c3f}మ\u{c3f}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ท\u{e34}ม\u{e34}"), ("tr", "Timiș ili"), ("uk", "Тіміш"), ("ur", "تیمیش کاؤنٹی"), ("vi", "Hạt Timis"), ("zh", "蒂米什縣")]),
                        unofficial_name_list: ["Timis"].to_vec(),
                    }
                ),
                (
                    "TR",
                    Subdivision{
                        name: "Teleorman",
                        country_alpha2: Alpha2::RO,
                        code: "TR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0160491), longitude: Some(25.2986628), max_latitude: Some(44.518081), min_latitude: Some(43.6186193), max_longitude: Some(25.713873), min_longitude: Some(24.585499)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيلورمان"), ("be", "жудзец Тэлеарман"), ("bg", "Телеорман"), ("bn", "টেলেওরম\u{9cd}য\u{9be}ন ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Teleorman"), ("ccp", "𑄑𑄬𑄣\u{11128}𑄠\u{1112e}𑄢\u{11134}𑄟𑄚\u{11134}"), ("ceb", "Teleorman"), ("cs", "Teleorman"), ("da", "Teleorman"), ("de", "Kreis Teleorman"), ("el", "Τελεορμάν"), ("en", "Teleorman"), ("es", "Teleorman"), ("et", "Teleormani maakond"), ("fa", "شهرستان تلئورمان"), ("fi", "Teleorman"), ("fr", "Județ de Teleorman"), ("gl", "Condado de Teleorman"), ("gu", "ટ\u{ac7}લિઓર\u{acd}મન કાઉન\u{acd}ટી"), ("he", "טלאורמן (מחוז)"), ("hi", "ट\u{947}लीओर\u{94d}नान काउ\u{902}टी"), ("hr", "Teleorman"), ("hu", "Teleorman megye"), ("id", "Provinsi Teleorman"), ("it", "distretto di Teleorman"), ("ja", "テレオルマン県"), ("ka", "ტელეორმანის ჟუდეცი"), ("kn", "ಟ\u{cc6}ಲ\u{cbf}ಮೋರ\u{ccd}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "텔레오르만 주"), ("lt", "Teleormano apskritis"), ("lv", "Teleormanas žudecs"), ("mn", "Телеорман аймаг"), ("mr", "ट\u{947}लरमन काउ\u{902}टी"), ("ms", "Wilayah Teleorman"), ("nb", "Teleorman"), ("nl", "District Teleorman"), ("no", "Teleorman"), ("pl", "Okręg Teleorman"), ("pt", "Teleorman"), ("ro", "Teleorman"), ("ru", "Телеорман"), ("si", "ටෙලෙයොර\u{dca}මන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Teleorman"), ("sr", "Телеорман"), ("sr_Latn", "Teleorman"), ("sv", "Teleorman"), ("ta", "டெலெஓர\u{bcd}மன\u{bcd} கவுண\u{bcd}டி"), ("te", "ట\u{c46}ల\u{c3f}య\u{c4b}ర\u{c4d}మన\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เตเลออร\u{e4c}ม\u{e31}นเคาน\u{e4c}ต\u{e35}"), ("tr", "Teleorman ili"), ("uk", "Телеорман"), ("ur", "تیلیاورمان کاؤنٹی"), ("vi", "Hạt Teleorman"), ("zh", "特列奧爾曼縣")]),
                        unofficial_name_list: ["Teleorman"].to_vec(),
                    }
                ),
                (
                    "VL",
                    Subdivision{
                        name: "Vâlcea",
                        country_alpha2: Alpha2::RO,
                        code: "VL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.07980509999999), longitude: Some(24.0835282), max_latitude: Some(45.587699), min_latitude: Some(44.49141210000001), max_longitude: Some(24.5211), min_longitude: Some(23.5770419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فالسيا"), ("be", "жудзец Вылча"), ("bg", "Вълча"), ("bn", "ভ\u{9cd}য\u{9be}লসিয\u{9bc}\u{9be}"), ("ca", "Província de Vâlcea"), ("ccp", "𑄞\u{11127}𑄣\u{11134}𑄥\u{11128}"), ("ceb", "Județul Vâlcea"), ("cs", "Vâlcea"), ("da", "Vâlcea"), ("de", "Kreis Vâlcea"), ("el", "Βάλτσεα"), ("en", "Vâlcea"), ("es", "Vâlcea"), ("et", "Vâlcea maakond"), ("fa", "شهرستان ولوچا"), ("fi", "Vâlcea"), ("fr", "Județ de Vâlcea"), ("gl", "Condado de Vâlcea"), ("gu", "વલ\u{acd}સીઆ કાઉન\u{acd}ટી"), ("he", "מחוז ולצ׳ה"), ("hi", "व\u{948}ल\u{94d}सिया काउ\u{902}टी"), ("hr", "Vâlcea"), ("hu", "Vâlcea megye"), ("id", "Provinsi Vâlcea"), ("it", "distretto di Vâlcea"), ("ja", "ヴルチャ県"), ("ka", "ვილჩის ჟუდეცი"), ("kn", "ವಲ\u{ccd}ಸ\u{cbf}ಯಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "블체아 주"), ("lt", "Vilčos apskritis"), ("lv", "Vilčas žudecs"), ("mn", "Вылча аймаг"), ("mr", "व\u{94d}लासीआ काउ\u{902}टी"), ("ms", "Wilayah Vâlcea"), ("nb", "Vâlcea"), ("nl", "District Vâlcea"), ("no", "Vâlcea"), ("pl", "Okręg Vâlcea"), ("pt", "Vâlcea"), ("ro", "Vâlcea"), ("ru", "Вылча"), ("si", "වල\u{dca}ස\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vâlcea"), ("sr", "Валча"), ("sr_Latn", "Valča"), ("sv", "Vâlcea"), ("ta", "வ\u{bbe}ல\u{bcd}ச\u{bbe} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}ల\u{c4d}స\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ว\u{e31}ลเซ\u{e35}ย ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Vâlcea ili"), ("uk", "Вилчя"), ("ur", "ویلچا کاؤنٹی"), ("vi", "Hạt Vâlcea"), ("zh", "沃爾恰縣")]),
                        unofficial_name_list: ["Vîlcea"].to_vec(),
                    }
                ),
                (
                    "VN",
                    Subdivision{
                        name: "Vrancea",
                        country_alpha2: Alpha2::RO,
                        code: "VN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.81348759999999), longitude: Some(27.0657531), max_latitude: Some(46.204405), min_latitude: Some(45.366076), max_longitude: Some(27.558525), min_longitude: Some(26.3742352)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فرنتشيا"), ("be", "Вранча"), ("bg", "Вранча"), ("bn", "ভ\u{9cd}র\u{9be}ন\u{9cd}সিয\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Vrancea"), ("ccp", "𑄞\u{11133}𑄢𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Vrancea"), ("cs", "Vrancea"), ("da", "Vrancea"), ("de", "Kreis Vrancea"), ("el", "Βράντσεα"), ("en", "Vrancea"), ("es", "Vrancea"), ("et", "Vrancea maakond"), ("eu", "Vrancea"), ("fa", "شهرستان ورانچا"), ("fi", "Vrancea"), ("fr", "Județ de Vrancea"), ("gl", "Vrancea"), ("gu", "વ\u{acd}રાન\u{acd}સિયા કાઉન\u{acd}ટી"), ("hi", "व\u{94d}रान\u{94d}सी काउ\u{902}टी"), ("hr", "Vrancea"), ("hu", "Vrancea megye"), ("id", "Provinsi Vrancea"), ("it", "distretto di Vrancea"), ("ja", "ヴランチャ県"), ("ka", "ვრანჩის ჟუდეცი"), ("kn", "ವ\u{ccd}ರಾನ\u{ccd}ಸ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "브란체아 주"), ("lt", "Vrančos apskritis"), ("lv", "Vrančas žudecs"), ("mn", "Вранча аймаг"), ("mr", "व\u{94d}रान\u{94d}सा काउ\u{902}टी"), ("ms", "Wilayah Vrancea"), ("nb", "Vrancea"), ("nl", "District Vrancea"), ("no", "Vrancea"), ("pl", "Okręg Vrancea"), ("pt", "Vrancea"), ("ro", "Vrancea"), ("ru", "Вранча"), ("si", "ව\u{dca}රන\u{dca}ස\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vrancea"), ("sr", "Вранча"), ("sr_Latn", "Vranča"), ("sv", "Vrancea"), ("ta", "விரன\u{bcd}ஸ\u{bbe} கவுண\u{bcd}டி"), ("te", "వ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "วาเลนเซ อ\u{e34}ล"), ("tr", "Vrancea ili"), ("uk", "Вранчя"), ("ur", "ورانچا کاؤنٹی"), ("vi", "Vrancea"), ("zh", "弗朗恰縣")]),
                        unofficial_name_list: ["Vrancea"].to_vec(),
                    }
                ),
                (
                    "VS",
                    Subdivision{
                        name: "Vaslui",
                        country_alpha2: Alpha2::RO,
                        code: "VS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.638333), longitude: Some(27.729167), max_latitude: Some(46.6689939), min_latitude: Some(46.6157827), max_longitude: Some(27.7493192), min_longitude: Some(27.707777)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Department,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم فاسلوي"), ("be", "Васлуй"), ("bg", "Васлуй"), ("bn", "ভ\u{9be}স\u{9cd}ল\u{9c1}য\u{9bc}ী ক\u{9be}উন\u{9cd}টি"), ("ca", "Província de Vaslui"), ("ccp", "𑄞𑄌\u{11134}𑄣\u{1112d}\u{1112a}"), ("ceb", "Vaslui"), ("cs", "Vaslui"), ("da", "Vaslui"), ("de", "Kreis Vaslui"), ("el", "Βασλούι"), ("en", "Vaslui"), ("es", "Vaslui"), ("et", "Vaslui maakond"), ("fa", "شهرستان وزلو"), ("fi", "Vaslui"), ("fr", "județ de Vaslui"), ("gl", "Condado de Vaslui"), ("gu", "વાસ\u{acd}લ\u{ac1}ઈ કાઉન\u{acd}ટી"), ("he", "מחוז וסלוי"), ("hi", "वास\u{94d}ल\u{942}य काउ\u{902}टी"), ("hr", "Vaslui"), ("hu", "Vaslui megye"), ("id", "Provinsi Vaslui"), ("it", "distretto di Vaslui"), ("ja", "ヴァスルイ県"), ("ka", "ვასლუის ჟუდეცი"), ("kn", "ವಾಸ\u{ccd}ಲುಯ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "바슬루이 주"), ("lt", "Vaslujaus apskritis"), ("lv", "Vaslujas žudecs"), ("mk", "Васлуј"), ("mn", "Васлуй аймаг"), ("mr", "वास\u{94d}ल\u{942}य काउ\u{902}टी"), ("ms", "Wilayah Vaslui"), ("nb", "Vaslui"), ("nl", "District Vaslui"), ("no", "Vaslui"), ("pl", "Okręg Vaslui"), ("pt", "Vaslui"), ("ro", "Vaslui"), ("ru", "Васлуй"), ("si", "වස\u{dca}ල\u{dd4}ය\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vaslui"), ("sr", "Васлуј"), ("sr_Latn", "Vasluj"), ("sv", "Vaslui"), ("ta", "வ\u{bbe}சலுய\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d}లూయ\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องหลวงวาสล\u{e38}ย"), ("tr", "Vaslui ili"), ("uk", "Васлуй"), ("ur", "واسلوی کاؤنٹی"), ("vi", "Hạt Vaslui"), ("zh", "瓦斯盧伊縣")]),
                        unofficial_name_list: ["Vaslui"].to_vec(),
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
#[cfg(feature = "ro")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::RO,
        alpha3: Alpha3::ROU,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 40,
        currency_code: CurrencyCode::RON,
        gec: Some(GEC::RO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ROU),
        iso_long_name: "Romania",
        iso_short_name: "Romania",
        official_language_list: ["ro"].to_vec(),
        spoken_language_list: ["ro"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Romanian"),
        number: "642",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "RO",
        unofficial_name_list: [
            "Romania",
            "Rumänien",
            "Roumanie",
            "Rumania",
            "ルーマニア",
            "Roemenië",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Romania"),
            ("af", "Roemenië"),
            ("ak", "Romania"),
            ("am", "ስሣኒ።"),
            ("an", "Romania"),
            ("ar", "رومانيا"),
            ("as", "ৰোম\u{9be}নিয়\u{9be}"),
            ("ay", "Romania"),
            ("az", "Ruminıya"),
            ("ba", "Romania"),
            ("be", "Румынія"),
            ("bg", "Румъния"),
            ("bi", "Romania"),
            ("bn", "রোম\u{9be}নিয়\u{9be}"),
            ("bn_IN", "রোম\u{9be}নিয়\u{9be}"),
            ("br", "Roumania"),
            ("bs", "Rumunija"),
            ("ca", "Romania"),
            ("ce", "Румыни"),
            ("ch", "Romania"),
            ("cs", "Rumunsko"),
            ("cv", "Румыни"),
            ("cy", "Rwmania"),
            ("da", "Rumænien"),
            ("de", "Rumänien"),
            ("dv", "ރ\u{7aa}މ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "ར\u{f7c}་མ་ན\u{f72}་ཡ།"),
            ("ee", "Romania"),
            ("el", "Ρουμανία"),
            ("en", "Romania"),
            ("eo", "Rumanio"),
            ("es", "Rumanía"),
            ("et", "Rumeenia"),
            ("eu", "Errumania"),
            ("fa", "رومانی"),
            ("ff", "Romaniya"),
            ("fi", "Romania"),
            ("fo", "Rumenia"),
            ("fr", "Roumanie"),
            ("fy", "Roemeenje"),
            ("ga", "An Rómáin"),
            ("gl", "Romanía"),
            ("gn", "Romania"),
            ("gu", "રોમાનિયા"),
            ("gv", "Yn Romaan"),
            ("ha", "Romainiya"),
            ("he", "רומניה"),
            ("hi", "रोमानिया"),
            ("hr", "Rumunjska"),
            ("ht", "Woumani"),
            ("hu", "Románia"),
            ("hy", "Ռումինիա"),
            ("ia", "Romania"),
            ("id", "Rumania"),
            ("io", "Rumania"),
            ("is", "Rúmenía"),
            ("it", "Romania"),
            ("iu", "Romania"),
            ("ja", "ルーマニア"),
            ("ka", "რუმინეთი"),
            ("ki", "Romania"),
            ("kk", "Румыния"),
            ("kl", "Romania"),
            ("km", "រ\u{17bc}ម\u{17c9}ាន\u{17b8}"),
            ("kn", "ರೊಮೇನ\u{cbf}ಯಾ"),
            ("ko", "루마니아"),
            ("ku", "Romanya"),
            ("kv", "Румыния"),
            ("kw", "Roumani"),
            ("ky", "Румыния"),
            ("lo", "ປະເທດລ\u{eb9}ມານ\u{eb5}"),
            ("lt", "Rumunija"),
            ("lv", "Rumānija"),
            ("mi", "Romeinia"),
            ("mk", "Романија"),
            ("ml", "റൊമ\u{d3e}നിയ"),
            ("mn", "Румын"),
            ("mr", "रोमानिया"),
            ("ms", "Romania"),
            ("mt", "Rumanija"),
            (
                "my",
                "ရ\u{102d}\u{102f}မေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Romania"),
            ("nb", "Romania"),
            ("ne", "रोमानिया"),
            ("nl", "Roemenië"),
            ("nn", "Romania"),
            ("nv", "Wooméiniya"),
            ("oc", "Romania"),
            ("or", "ରୋମ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਰ\u{a4b}ਮਾਨੀਆ"),
            ("pi", "रोमानिया"),
            ("pl", "Rumunia"),
            ("ps", "رومانیا"),
            ("pt", "Roménia"),
            ("pt_BR", "Romênia"),
            ("ro", "România"),
            ("ru", "Румыния"),
            ("rw", "Romaniya"),
            ("sc", "Romania"),
            ("sd", "Romania"),
            ("si", "රොමේන\u{dd2}ය\u{dcf}ව"),
            ("sk", "Rumunsko"),
            ("sl", "Romunija"),
            ("so", "Rumaaniya"),
            ("sq", "Rumani"),
            ("sr", "Румунија"),
            ("sv", "Rumänien"),
            ("sw", "Romania"),
            ("ta", "ரோம\u{bbe}னிய\u{bbe}"),
            ("te", "ర\u{c4b}మ\u{c3e}న\u{c3f}య\u{c3e}"),
            ("tg", "Руминия"),
            ("th", "โรมาเน\u{e35}ย"),
            ("ti", "ሮሜኒያ"),
            ("tk", "Rumyniýa"),
            ("tl", "Romania"),
            ("tr", "Romanya"),
            ("tt", "Романиа"),
            ("ug", "رۇمىنىيە"),
            ("uk", "Румунія"),
            ("ur", "رومانیہ"),
            ("uz", "Ruminiya"),
            ("ve", "Romania"),
            ("vi", "Rô-ma-ni"),
            ("wa", "Roumaneye"),
            ("wo", "Romaani"),
            ("xh", "Romania"),
            ("yo", "Románíà"),
            ("zh_CN", "罗马尼亚"),
            ("zh_HK", "羅馬尼亞"),
            ("zh_TW", "羅馬尼亞"),
            ("zu", "I-Romaniya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
