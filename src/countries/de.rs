// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Federal Republic of Germany

#[cfg(all(feature = "de", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::DE;
    pub const ALPHA3: Alpha3 = Alpha3::DEU;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 49;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::GM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::GER);
    pub const ISO_SHORT_NAME: &str = "Germany";
    pub const ISO_LONG_NAME: &str = "The Federal Republic of Germany";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["de"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["de"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2, 3, 4, 5];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6, 7, 8, 9, 10, 11];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("German");
    pub const NUMBER: &str = "276";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "DE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Germany",
        "Deutschland",
        "Allemagne",
        "Alemania",
        "ドイツ",
        "Duitsland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Germany"),
        ("af", "Duitsland"),
        ("ak", "Germany"),
        ("am", "ጀሴመን"),
        ("an", "Germany"),
        ("ar", "ألمانيا"),
        (
            "as",
            "জ\u{9be}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9cd}ম\u{9be}নী",
        ),
        ("ay", "Germany"),
        ("az", "Almaniya"),
        ("ba", "Germany"),
        ("be", "Германія"),
        ("bg", "Германия"),
        ("bi", "Germany"),
        ("bn", "জ\u{9be}র\u{9cd}ম\u{9be}নি"),
        ("bn_IN", "জ\u{9be}র\u{9cd}ম\u{9be}নি"),
        ("br", "Alamagn"),
        ("bs", "Njemačka"),
        ("ca", "Alemanya"),
        ("ce", "Германи"),
        ("ch", "Alemaña"),
        ("cs", "Německo"),
        ("cv", "Германи"),
        ("cy", "Yr Almaen"),
        ("da", "Tyskland"),
        ("de", "Deutschland"),
        (
            "dv",
            "ޖ\u{7a6}ރ\u{7aa}މ\u{7a6}ނ\u{7aa}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}",
        ),
        ("dz", "ཇར་མ་ན\u{f72}།"),
        ("ee", "Germany"),
        ("el", "Γερμανία"),
        ("en", "Germany"),
        ("eo", "Germanio"),
        ("es", "Alemania"),
        ("et", "Saksamaa"),
        ("eu", "Alemania"),
        ("fa", "آلمان"),
        ("ff", "Almaanya"),
        ("fi", "Saksa"),
        ("fo", "Týskland"),
        ("fr", "Allemagne"),
        ("fy", "Dútslân"),
        ("ga", "An Ghearmáin"),
        ("gl", "Alemaña"),
        ("gn", "Germany"),
        ("gu", "જર\u{acd}મની"),
        ("gv", "Yn Ghermaan"),
        ("ha", "Jamus"),
        ("he", "גרמניה"),
        ("hi", "जर\u{94d}मनी"),
        ("hr", "Njemačka"),
        ("ht", "Almay"),
        ("hu", "Németország"),
        ("hy", "Գերմանիա"),
        ("ia", "Germania"),
        ("id", "Jerman"),
        ("io", "Germania"),
        ("is", "Þýskaland"),
        ("it", "Germania"),
        ("iu", "ᔮᒪᓂ"),
        ("ja", "ドイツ"),
        ("ka", "გერმანია"),
        ("ki", "Germany"),
        ("kk", "Германия"),
        ("kl", "Germany"),
        ("km", "អាល\u{17d2}ល\u{17ba}ម\u{17c9}ង\u{17cb}"),
        ("kn", "ಜರ\u{ccd}ಮನ\u{cbf}"),
        ("ko", "독일"),
        ("ku", "Almanya"),
        ("kv", "Германия"),
        ("kw", "Almayn"),
        ("ky", "Германия"),
        ("lo", "ປະເທດເຢ\u{eb1}ຽລະມ\u{eb1}ນ"),
        ("lt", "Vokietija"),
        ("lv", "Vācija"),
        ("mi", "Tiamana"),
        ("mk", "Германија"),
        ("ml", "ജര\u{d4d}\u{200d}മനി"),
        ("mn", "Герман"),
        ("mr", "जर\u{94d}मनी"),
        ("ms", "Jerman"),
        ("mt", "Ġermanja"),
        ("my", "ဂျာမန\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Djermani"),
        ("nb", "Tyskland"),
        ("ne", "जर\u{94d}मनी"),
        ("nl", "Duitsland"),
        ("nn", "Tyskland"),
        ("nv", "Béésh Bichʼahii Bikéyah"),
        ("oc", "Alemanha"),
        ("or", "ଜର\u{b4d}ମ\u{b3e}ନୀ"),
        ("pa", "ਜਰਮਨੀ"),
        ("pi", "जर\u{94d}मनी"),
        ("pl", "Niemcy"),
        ("ps", "المان"),
        ("pt", "Alemanha"),
        ("pt_BR", "Alemanha"),
        ("ro", "Germania"),
        ("ru", "Германия"),
        ("rw", "Ubudage"),
        ("sc", "Germània"),
        ("sd", "جرمني"),
        ("si", "ජර\u{dca}මන\u{dd2}ය"),
        ("sk", "Nemecko"),
        ("sl", "Nemčija"),
        ("so", "Jarmal"),
        ("sq", "Gjermani"),
        ("sr", "Немачка"),
        ("sv", "Tyskland"),
        ("sw", "Germany"),
        ("ta", "ஜெர\u{bcd}மனி"),
        ("te", "జర\u{c4d}మన\u{c40}"),
        ("tg", "Олмон"),
        ("th", "เยอรมน\u{e35}"),
        ("ti", "ጀርመን"),
        ("tk", "Germaniýa"),
        ("tl", "Alemanya"),
        ("tr", "Almanya"),
        ("tt", "Алманиа"),
        ("ug", "گېرمانىيە"),
        ("uk", "Німеччина"),
        ("ur", "جرمنی"),
        ("uz", "Olmoniya"),
        ("ve", "Germany"),
        ("vi", "Đức"),
        ("wa", "Almagne"),
        ("wo", "Almaañ"),
        ("xh", "Jamani"),
        ("yo", "Jẹ\u{301}mánì"),
        ("zh_CN", "德国"),
        ("zh_HK", "德國"),
        ("zh_TW", "德國"),
        ("zu", "IJalimani"),
    ];
    #[cfg(all(feature = "de", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 51.165691;
        pub const LONGITUDE: f64 = 10.451526;
        pub const MAX_LATITUDE: f64 = 55.0815;
        pub const MAX_LONGITUDE: f64 = 15.0418962;
        pub const MIN_LATITUDE: f64 = 47.2701115;
        pub const MIN_LONGITUDE: f64 = 5.8663425;
        pub const NORTHEAST_LATITUDE: f64 = 55.0815;
        pub const NORTHEAST_LONGITUDE: f64 = 15.0418962;
        pub const SOUTHWEST_LATITUDE: f64 = 47.2701115;
        pub const SOUTHWEST_LONGITUDE: f64 = 5.8663425;
    }
}
#[cfg(all(feature = "de", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 51.165691,
            longitude: 10.451526,
            max_latitude: 55.0815,
            max_longitude: 15.0418962,
            min_latitude: 47.2701115,
            min_longitude: 5.8663425,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 55.0815,
                    longitude: 15.0418962,
                },
                southwest: CountryGeoBound {
                    latitude: 47.2701115,
                    longitude: 5.8663425,
                },
            },
        }
    }
}

#[cfg(all(feature = "de", feature = "subdivisions"))]
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
                    "BB",
                    Subdivision{
                        name: "BB",
                        country_alpha2: Alpha2::DE,
                        code: "BB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.4125287), longitude: Some(12.5316444), max_latitude: Some(52.5417764), min_latitude: Some(52.3115782), max_longitude: Some(12.7259978), min_longitude: Some(12.3610803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Brandenburg"), ("am", "ብራንደንቡርግ"), ("ar", "براندنبورغ"), ("az", "Brandenburq"), ("be", "Брандэнбург"), ("bg", "Бранденбург"), ("bn", "ব\u{9cd}র\u{9cd}য\u{9be}ন\u{9cd}ডেনব\u{9c1}র\u{9cd}গ"), ("bs", "Brandenburg"), ("ca", "Brandenburg"), ("ccp", "𑄝\u{11133}𑄢𑄚\u{11134}𑄓𑄬𑄚\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Brandenburg"), ("cs", "Braniborsko"), ("cy", "Brandenburg"), ("da", "Brandenburg"), ("de", "Brandenburg"), ("el", "Βραδεμβούργο"), ("en", "Brandenburg"), ("es", "Brandeburgo"), ("et", "Brandenburg"), ("eu", "Brandenburgo"), ("fa", "براندنبورگ"), ("fi", "Brandenburg"), ("fr", "Brandebourg"), ("ga", "Brandenburg"), ("gl", "Brandeburgo - Brandenburg"), ("gu", "બ\u{acd}રાન\u{acd}ડ\u{ac7}નબર\u{acd}ગ"), ("he", "ברנדנבורג"), ("hi", "ब\u{94d}र\u{948}\u{902}ड\u{947}नबर\u{94d}ग"), ("hr", "Brandenburg"), ("hu", "Brandenburg"), ("hy", "Բրանդենբուրգ"), ("id", "Brandenburg"), ("is", "Brandenborg"), ("it", "Brandeburgo"), ("ja", "ブランデンブルク州"), ("jv", "Brandenburg"), ("ka", "ბრანდენბურგი"), ("kk", "Бранденбург"), ("kn", "ಬ\u{ccd}ರ\u{ccd}ಯಾಂಡನ\u{ccd}ಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "브란덴부르크 주"), ("lt", "Brandenburgas"), ("lv", "Brandenburga"), ("mk", "Бранденбург"), ("mn", "Бранденбург"), ("mr", "ब\u{94d}रा\u{902}ड\u{947}नब\u{941}र\u{94d}ग"), ("ms", "Brandenburg"), ("nb", "Brandenburg"), ("ne", "ब\u{94d}र\u{948}\u{902}ड\u{947}नबर\u{94d}ग"), ("nl", "Brandenburg"), ("no", "Brandenburg"), ("pa", "ਬ\u{a4d}ਰਾ\u{a02}ਡਨਬ\u{a41}ਰਗ"), ("pl", "Brandenburgia"), ("ps", "براندنبورګ"), ("pt", "Brandemburgo"), ("ro", "Brandenburg"), ("ru", "Бранденбург"), ("sd", "برانڊن برگ"), ("si", "බ\u{dca}\u{200d}රන\u{dca}ඩෙන\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Brandenbursko"), ("sl", "Brandenburg"), ("sq", "Brandenburg"), ("sr", "Бранденбург"), ("sr_Latn", "Brandenburg"), ("sv", "Brandenburg"), ("sw", "Brandenburg"), ("ta", "பிர\u{bbe}ன\u{bcd}டென\u{bcd}பர\u{bcd}க\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3e}ండ\u{c46}న\u{c4d}బర\u{c4d}గ\u{c4d}"), ("th", "ร\u{e31}ฐบร\u{e31}นเด\u{e34}นบวร\u{e4c}ค"), ("tr", "Brandenburg"), ("uk", "Бранденбург"), ("ur", "برندنبرگ"), ("uz", "Brandenburg"), ("vi", "Brandenburg"), ("yo", "Brandenburg"), ("yo_BJ", "Brandenburg"), ("yue", "勃蘭登堡"), ("yue_Hans", "勃兰登堡"), ("zh", "勃兰登堡"), ("zu", "Brandenburg")]),
                        unofficial_name_list: ["Brandenbourg", "Brandenburgo"].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::DE,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.52000659999999), longitude: Some(13.404954), max_latitude: Some(52.6754542), min_latitude: Some(52.33962959999999), max_longitude: Some(13.7611176), min_longitude: Some(13.0891553)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Berlyn"), ("am", "በርሊን"), ("ar", "برلين"), ("az", "Berlin"), ("be", "Берлін"), ("bg", "Берлин"), ("bn", "ব\u{9be}র\u{9cd}লিন"), ("bs", "Berlin"), ("ca", "Berlín"), ("ccp", "𑄝𑄢\u{11134}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Berlin"), ("cs", "Berlín"), ("cy", "Berlin"), ("da", "Berlin"), ("de", "Berlin"), ("el", "Βερολίνο"), ("en", "Berlin"), ("es", "Berlín"), ("et", "Berliin"), ("eu", "Berlin"), ("fa", "برلین"), ("fi", "Berliini"), ("fr", "Berlin"), ("ga", "Beirlín"), ("gl", "Berlín"), ("gu", "બર\u{acd}લિન"), ("ha", "Berlin"), ("ha_NE", "Berlin"), ("he", "ברלין"), ("hi", "बर\u{94d}लिन"), ("hr", "Berlin"), ("hu", "Berlin"), ("hy", "Բեռլին"), ("id", "Berlin"), ("is", "Berlín"), ("it", "Berlino"), ("ja", "ベルリン"), ("jv", "Berlin"), ("ka", "ბერლინი"), ("kk", "Берлин"), ("kn", "ಬರ\u{ccd}ಲ\u{cbf}ನ\u{ccd}"), ("ko", "베를린"), ("ky", "Берлин"), ("lt", "Berlynas"), ("lv", "Berlīne"), ("mk", "Берлин"), ("ml", "ബെർലിൻ"), ("mn", "Берлин"), ("mr", "बर\u{94d}लिन"), ("ms", "Berlin"), ("my", "ဘာလင\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Berlin"), ("ne", "बर\u{94d}लिन"), ("nl", "Berlijn"), ("no", "Berlin"), ("or", "ବର\u{b4d}ଲ\u{b3f}ନ"), ("pa", "ਬਰਲਿਨ"), ("pl", "Berlin"), ("ps", "برلين"), ("pt", "Berlim"), ("ro", "Berlin"), ("ru", "Берлин"), ("si", "බර\u{dca}ල\u{dd2}නය"), ("sk", "Berlín"), ("sl", "Berlin"), ("so", "Baarliin"), ("sq", "Berlin"), ("sr", "Берлин"), ("sr_Latn", "Berlin"), ("sv", "Berlin"), ("sw", "Berlin"), ("ta", "பெர\u{bcd}லின\u{bcd}"), ("te", "బ\u{c46}ర\u{c4d}ల\u{c3f}న\u{c4d}"), ("th", "เบอร\u{e4c}ล\u{e34}น"), ("tk", "Berlin"), ("tr", "Berlin"), ("uk", "Берлін"), ("ur", "برلن"), ("uz", "Berlin"), ("vi", "Berlin"), ("yo", "Berlin"), ("yo_BJ", "Berlin"), ("yue", "柏林"), ("yue_Hans", "柏林"), ("zh", "柏林"), ("zu", "IBerlini")]),
                        unofficial_name_list: ["Berlín"].to_vec(),
                    }
                ),
                (
                    "BW",
                    Subdivision{
                        name: "BW",
                        country_alpha2: Alpha2::DE,
                        code: "BW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.6616037), longitude: Some(9.3501336), max_latitude: Some(49.7913278), min_latitude: Some(47.5323664), max_longitude: Some(10.495573), min_longitude: Some(7.511756799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Baden-Württemberg"), ("am", "ባደን-ቩርተምቡርግ"), ("ar", "بادن-فورتمبيرغ"), ("az", "Baden-Vürtemberq"), ("be", "Бадэн-Вюртэмберг"), ("bg", "Баден-Вюртемберг"), ("bn", "ব\u{9be}ডেন-ভ\u{9c1}র\u{9cd}টেমব\u{9be}র\u{9cd}গ"), ("bs", "Baden-Württemberg"), ("ca", "Baden-Württemberg"), ("ccp", "𑄝𑄓𑄬𑄚\u{11134}-𑄤𑄢\u{11134}𑄑𑄓𑄟\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Baden-Württemberg Region"), ("cs", "Bádensko-Württembersko"), ("cy", "Baden-Württemberg"), ("da", "Baden-Württemberg"), ("de", "Baden-Württemberg"), ("el", "Βάδη-Βυρτεμβέργη"), ("en", "Baden-Württemberg"), ("es", "Baden-Wurtemberg"), ("et", "Baden-Württemberg"), ("eu", "Baden-Württemberg"), ("fa", "بادن-وورتم\u{200c}برگ"), ("fi", "Baden-Württemberg"), ("fr", "Bade-Wurtemberg"), ("ga", "Baden-Württemberg"), ("gl", "Baden-Württemberg"), ("gu", "બ\u{ac7}ડ\u{ac7}ન-વ\u{acd}ય\u{ac1}ર\u{acd}ટ\u{ac7}મબર\u{acd}ગ"), ("he", "באדן-וירטמברג"), ("hi", "ब\u{947}डन-व\u{941}र\u{94d}टमबर\u{94d}ग"), ("hr", "Baden-Württemberg"), ("hu", "Baden-Württemberg"), ("hy", "Բադեն-Վյուրթեմբերգ"), ("id", "Baden-Württemberg"), ("is", "Baden-Württemberg"), ("it", "Baden-Württemberg"), ("jv", "Baden-Württemberg"), ("ka", "ბადენ-ვიურტემბერგი"), ("kk", "Баден-Вюртемберг"), ("kn", "ಬಾಡ\u{cc6}ನ\u{ccd}-ವುರ\u{ccd}ಟ\u{cc6}ಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "바덴뷔르템베르크 주"), ("lt", "Badenas-Viurtembergas"), ("lv", "Bādene-Virtemberga"), ("mk", "Баден-Виртемберг"), ("mn", "Баден-Вюртемберг"), ("mr", "बाड\u{947}न-व\u{94d}य\u{941}र\u{94d}ट\u{947}\u{902}बर\u{94d}ग"), ("ms", "Baden-Württemberg"), ("nb", "Baden-Württemberg"), ("nl", "Baden-Württemberg"), ("no", "Baden-Württemberg"), ("pa", "ਬਾਡਨ-ਵਰਟਮਬਰਕ"), ("pl", "Badenia-Wirtembergia"), ("ps", "باډن ورټمبرګ"), ("pt", "Baden-Württemberg"), ("ro", "Baden-Württemberg"), ("ru", "Баден-Вюртемберг"), ("sd", "بيڊن ورٽمبرگ"), ("si", "බ\u{dcf}ඩෙන\u{dca}-ව\u{dd4}ට\u{dca}ටම\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Bádensko-Württembersko"), ("sl", "Baden-Württemberg"), ("sq", "Baden-Vyrtemberg"), ("sr", "Баден-Виртемберг"), ("sr_Latn", "Baden-Virtemberg"), ("sv", "Baden-Württemberg"), ("sw", "Baden-Württemberg"), ("ta", "பேடன\u{bcd}-வர\u{bcd}ட\u{bcd}டென\u{bcd}பர\u{bcd}க\u{bcd}"), ("te", "బ\u{c3e}డ\u{c46}న\u{c4d}-వుట\u{c46}ంబర\u{c4d}గ\u{c4d}"), ("th", "ร\u{e31}ฐบาเด\u{e34}น-เว\u{e37}อร\u{e4c}ทเท\u{e34}มแบร\u{e4c}ค"), ("tk", "Baden-Württemberg"), ("tr", "Baden-Württemberg"), ("uk", "Баден-Вюртемберг"), ("ur", "بادن-وورتمبرگ"), ("uz", "Baden-Vyurtemberg"), ("vi", "Baden-Württemberg"), ("yo", "Baden-Württemberg"), ("yo_BJ", "Baden-Württemberg"), ("yue", "巴登－烏騰堡"), ("yue_Hans", "巴登－乌腾堡"), ("zh", "巴登－符腾堡"), ("zu", "Baden-Württemberg")]),
                        unofficial_name_list: ["Baden-Württemberg"].to_vec(),
                    }
                ),
                (
                    "BY",
                    Subdivision{
                        name: "BY",
                        country_alpha2: Alpha2::DE,
                        code: "BY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.7904472), longitude: Some(11.4978895), max_latitude: Some(50.5647142), min_latitude: Some(47.2701115), max_longitude: Some(13.8396371), min_longitude: Some(8.9763497)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Beiere"), ("am", "ባቫሪያ"), ("ar", "بافاريا"), ("az", "Bavariya"), ("be", "Баварыя"), ("bg", "Бавария"), ("bn", "ব\u{9be}ভ\u{9be}রিয\u{9bc}\u{9be}"), ("bs", "Bavarska"), ("ca", "Baviera"), ("ccp", "𑄝\u{11127}𑄞𑄢\u{11128}𑄠"), ("ceb", "Bavaria"), ("cs", "Bavorsko"), ("cy", "Bafaria"), ("da", "Bayern"), ("de", "Bayern"), ("el", "Βαυαρία"), ("en", "Bavaria"), ("es", "Baviera"), ("et", "Baieri"), ("eu", "Bavaria"), ("fa", "بایرن"), ("fi", "Baijeri"), ("fr", "Bavière"), ("ga", "An Bhaváir"), ("gl", "Baviera"), ("gu", "બાવ\u{ac7}રિયા"), ("he", "בוואריה"), ("hi", "बायर\u{94d}न"), ("hr", "Bavarska"), ("hu", "Bajorország"), ("hy", "Բավարիա"), ("id", "Bayern"), ("is", "Bæjaraland"), ("it", "Baviera"), ("ja", "バイエルン自由州"), ("jv", "Bayern"), ("ka", "ბავარია"), ("kk", "Бавария"), ("kn", "ಬವೇರ\u{cbf}ಯಾ"), ("ko", "바이에른 주"), ("ky", "Бавария"), ("lt", "Bavarija"), ("lv", "Bavārija"), ("mk", "Баварија"), ("ml", "ബവേറിയ"), ("mn", "Бавар"), ("mr", "बायर\u{94d}न"), ("ms", "Bavaria"), ("nb", "Bayern"), ("ne", "बाभारिया"), ("nl", "Beieren"), ("no", "Bayern"), ("pa", "ਬਾਈਆਨ"), ("pl", "Bawaria"), ("pt", "Baviera"), ("ro", "Bavaria"), ("ru", "Бавария"), ("sd", "بويريا"), ("si", "බවය\u{dd2}ර\u{dcf}"), ("sk", "Bavorsko"), ("sl", "Bavarska"), ("sq", "Bavaria"), ("sr", "Баварска"), ("sr_Latn", "Bavarska"), ("sv", "Bayern"), ("sw", "Bavaria"), ("ta", "பவேரிய\u{bbe}"), ("te", "బవ\u{c47}ర\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐบาวาเร\u{e35}ย"), ("tk", "Bawariýa"), ("tr", "Bavyera"), ("uk", "Баварія"), ("ur", "بواریا"), ("uz", "Bavariya"), ("vi", "Bayern"), ("yo", "Bavaria"), ("yo_BJ", "Bavaria"), ("yue", "拜仁"), ("yue_Hans", "拜仁"), ("zh", "巴伐利亚")]),
                        unofficial_name_list: ["Bavaria", "Bavière", "Bayern"].to_vec(),
                    }
                ),
                (
                    "HB",
                    Subdivision{
                        name: "HB",
                        country_alpha2: Alpha2::DE,
                        code: "HB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.07929619999999), longitude: Some(8.8016937), max_latitude: Some(53.2289105), min_latitude: Some(53.0116038), max_longitude: Some(8.9908131), min_longitude: Some(8.4817585)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vrye Hansestad Bremen"), ("am", "ብሬመን ክፍላገር"), ("ar", "ولاية بريمن"), ("az", "Bremen əyaləti"), ("be", "Вольны ганзейскі горад Брэмен"), ("bg", "Бремен"), ("bn", "ব\u{9cd}রেমেনের ফ\u{9cd}রি হেনসিয\u{9bc}\u{9be}টিক শহর"), ("bs", "Bremen"), ("ca", "Estat de Bremen"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄟𑄬𑄚\u{11134}"), ("ceb", "Bremen"), ("cs", "Svobodné hanzovní město Brémy"), ("cy", "Bremen"), ("da", "Bremen"), ("de", "Bremen"), ("el", "Βρέμη"), ("en", "Bremen"), ("es", "Bremen"), ("et", "Bremeni liidumaa"), ("eu", "Bremen"), ("fa", "برمن"), ("fi", "Bremen"), ("fr", "Brême"), ("ga", "Bremen"), ("gl", "Estado de Bremen - Land Bremen"), ("gu", "બ\u{acd}ર\u{ac7}નન\u{ac1}\u{a82} ફ\u{acd}રી હ\u{ac7}ન\u{acd}સિયાટિક શહ\u{ac7}ર"), ("he", "ברמן"), ("hi", "ब\u{94d}र\u{947}म\u{947}न"), ("hr", "Bremen"), ("hu", "Bréma"), ("id", "Bremen"), ("it", "Brema"), ("ja", "ブレーメン州"), ("jv", "Bremen"), ("ka", "ბრემენი"), ("kk", "Бремен"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಮ\u{cc6}ನ\u{ccd} ಮುಕ\u{ccd}ತ ಹ\u{ccd}ಯಾನ\u{ccd}ಸ\u{cbf}ಯಾಟ\u{cbf}ಕ\u{ccd} ನಗರ"), ("ko", "브레멘 주"), ("lt", "Brėmenas"), ("lv", "Brēmene"), ("mk", "Бремен"), ("mn", "Бремен муж"), ("mr", "ब\u{94d}र\u{947}मन (राज\u{94d}य)"), ("ms", "Bremen"), ("nb", "Bremen"), ("ne", "ब\u{94d}र\u{947}म\u{947}न (राज\u{94d}य)"), ("nl", "Bremen"), ("no", "Bremen"), ("pa", "ਬ\u{a4d}ਰ\u{a48}ਮਨ"), ("pl", "Brema"), ("ps", "برېمن"), ("pt", "Bremen"), ("ro", "Brema"), ("ru", "Бремен"), ("si", "ෆ\u{dca}\u{200d}ර\u{dd3} හන\u{dca}ස\u{dd2}යට\u{dd2}ක\u{dca} බ\u{dca}\u{200d}ර\u{dd2}මෙන\u{dca} නගරය"), ("sk", "Brémy"), ("sl", "Bremen"), ("sq", "Bremen"), ("sr", "Бремен"), ("sr_Latn", "Bremen"), ("sv", "Bremen"), ("ta", "பிரெமென\u{bcd}"), ("te", "ఫ\u{c4d}ర\u{c40} హ\u{c3e}న\u{c4d}స\u{c40}ట\u{c3f}క\u{c4d} స\u{c3f}ట\u{c40} ఆఫ\u{c4d} బ\u{c4d}ర\u{c46}మ\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐเบรเม\u{e34}น"), ("tr", "Bremen"), ("uk", "Бремен"), ("ur", "برمن (صوبہ)"), ("uz", "Bremen"), ("vi", "Bremen"), ("yo", "Bremen"), ("yo_BJ", "Bremen"), ("yue", "不來梅漢莎自由市"), ("yue_Hans", "不来梅汉莎自由市"), ("zh", "不來梅州")]),
                        unofficial_name_list: ["Brème"].to_vec(),
                    }
                ),
                (
                    "HE",
                    Subdivision{
                        name: "HE",
                        country_alpha2: Alpha2::DE,
                        code: "HE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.6520515), longitude: Some(9.162437599999999), max_latitude: Some(51.6575571), min_latitude: Some(49.3952611), max_longitude: Some(10.2363207), min_longitude: Some(7.7724675)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hesse"), ("am", "ሄሠ"), ("ar", "هسن"), ("az", "Hessen"), ("be", "Гесэн"), ("bg", "Хесен"), ("bn", "হেসে"), ("bs", "Hessen"), ("ca", "Hessen"), ("ccp", "𑄦𑄬𑄥𑄬"), ("ceb", "Hessen"), ("cs", "Hesensko"), ("cy", "Hessen"), ("da", "Hessen"), ("de", "Hessen"), ("el", "Έσση"), ("en", "Hesse"), ("es", "Hasse"), ("et", "Hessen"), ("eu", "Hessen"), ("fa", "هسن"), ("fi", "Hessen"), ("fr", "Hesse"), ("ga", "Hessen"), ("gl", "Hessen"), ("gu", "હ\u{ac7}સ\u{ac7}"), ("he", "הסן"), ("hi", "ह\u{947}स\u{947}"), ("hr", "Hessen"), ("hu", "Hessen"), ("hy", "Հեսսեն"), ("id", "Hessen"), ("is", "Hessen"), ("it", "Assia"), ("ja", "ヘッセン州"), ("jv", "Hessen"), ("ka", "ჰესენი"), ("kk", "Гессен"), ("kn", "ಹ\u{cc6}ಸ\u{ccd}ಸ\u{cc6}"), ("ko", "헤센 주"), ("ky", "Гессен"), ("lt", "Hesenas"), ("lv", "Hesene"), ("mk", "Хесен"), ("mn", "Хессен"), ("mr", "ह\u{947}स\u{947}न"), ("ms", "Hesse"), ("nb", "Hessen"), ("ne", "ह\u{947}स\u{947}"), ("nl", "Hessen"), ("no", "Hessen"), ("pa", "ਹ\u{a48}\u{a71}ਸਨ"), ("pl", "Hesja"), ("pt", "Hesse"), ("ro", "Hessa"), ("ru", "Гессен"), ("sd", "ھيسن"), ("si", "හෙසේ"), ("sk", "Hesensko"), ("sl", "Hessen"), ("sq", "Hesia"), ("sr", "Хесен"), ("sr_Latn", "Hesen"), ("sv", "Hessen"), ("sw", "Hesse"), ("ta", "கெஸ\u{bcd}சன\u{bcd}"), ("te", "హ\u{c46}స\u{c4d}"), ("th", "ร\u{e31}ฐเฮ\u{e47}สเซ\u{e34}น"), ("tk", "Hessen"), ("tr", "Hessen"), ("uk", "Гессен"), ("ur", "ہیسے"), ("uz", "Hessen"), ("vi", "Hessen"), ("yo", "Hesse"), ("yo_BJ", "Hesse"), ("yue", "黑森"), ("yue_Hans", "黑森"), ("zh", "黑森")]),
                        unofficial_name_list: ["Hesse", "Hessen"].to_vec(),
                    }
                ),
                (
                    "HH",
                    Subdivision{
                        name: "HH",
                        country_alpha2: Alpha2::DE,
                        code: "HH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5510846), longitude: Some(9.9936818), max_latitude: Some(53.7394338), min_latitude: Some(53.3962857), max_longitude: Some(10.32528), min_longitude: Some(9.7301316)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hamburg"), ("am", "ሃምቡርግ"), ("ar", "هامبورغ"), ("az", "Hamburq"), ("be", "Гамбург"), ("bg", "Хамбург"), ("bn", "হ\u{9be}মব\u{9c1}র\u{9cd}গ"), ("bs", "Hamburg"), ("ca", "Hamburg"), ("ccp", "𑄦𑄟\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Free and Hanseatic City of Hamburg"), ("cs", "Hamburk"), ("cy", "Hamburg"), ("da", "Hamborg"), ("de", "Hamburg"), ("el", "Αμβούργο"), ("en", "Hamburg"), ("es", "Hamburgo"), ("et", "Hamburg"), ("eu", "Hanburgo"), ("fa", "هامبورگ"), ("fi", "Hampuri"), ("fr", "Hambourg"), ("ga", "Hamburg"), ("gl", "Hamburgo"), ("gu", "હ\u{ac7}મ\u{acd}બર\u{acd}ગ"), ("ha", "Hamburg"), ("ha_NE", "Hamburg"), ("he", "המבורג"), ("hi", "ह\u{948}म\u{94d}बर\u{94d}ग"), ("hr", "Hamburg"), ("hu", "Hamburg"), ("hy", "Համբուրգ"), ("id", "Hamburg"), ("is", "Hamborg"), ("it", "Amburgo"), ("ja", "ハンブルク"), ("jv", "Hamburg"), ("ka", "ჰამბურგი"), ("kk", "Гамбург"), ("kn", "ಹ\u{ccd}ಯಾಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "함부르크"), ("ky", "Гамбург"), ("lt", "Hamburgas"), ("lv", "Hamburga"), ("mk", "Хамбург"), ("ml", "ഹ\u{d3e}ംബർഗ\u{d4d}"), ("mn", "Хамбург"), ("mr", "हा\u{902}ब\u{941}र\u{94d}ग"), ("ms", "Hamburg"), ("my", "ဟမ\u{103a}းဗတ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Hamburg"), ("ne", "ह\u{94d}याम\u{94d}बर\u{94d}ग"), ("nl", "Hamburg"), ("no", "Hamburg"), ("pa", "ਹਾਮਬ\u{a41}ਰਗ"), ("pl", "Hamburg"), ("pt", "Hamburgo"), ("ro", "Hamburg"), ("ru", "Гамбург"), ("sd", "ھئمبرگ(شھر)"), ("si", "හැම\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Hamburg"), ("sl", "Hamburg"), ("sq", "Hamburgu"), ("sr", "Хамбург"), ("sr_Latn", "Hamburg"), ("sv", "Hamburg"), ("sw", "Hamburg"), ("ta", "ஹ\u{bbe}ம\u{bcd}பெர\u{bcd}க\u{bcd}"), ("te", "హ\u{c4d}య\u{c3e}ంబర\u{c4d}గ\u{c4d}"), ("th", "ฮ\u{e31}มบวร\u{e4c}ค"), ("tk", "Gamburg"), ("tr", "Hamburg"), ("uk", "Гамбург"), ("ur", "ہامبرگ"), ("uz", "Gamburg"), ("vi", "Hamburg"), ("yo", "Hamburg"), ("yo_BJ", "Hamburg"), ("yue", "漢堡"), ("yue_Hans", "汉堡"), ("zh", "汉堡"), ("zu", "Hamburg")]),
                        unofficial_name_list: ["Amburgo", "Hambourg", "Hamburgo"].to_vec(),
                    }
                ),
                (
                    "MV",
                    Subdivision{
                        name: "MV",
                        country_alpha2: Alpha2::DE,
                        code: "MV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.6126505), longitude: Some(12.4295953), max_latitude: Some(54.68469), min_latitude: Some(53.1103197), max_longitude: Some(14.412257), min_longitude: Some(10.5936138)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mecklenburg-Voorpommere"), ("am", "መክለንቡርክ-ፎርፖመርን"), ("ar", "مكلنبورغ فوربومرن"), ("az", "Meklenburq-Ön Pomeraniya"), ("be", "Мекленбург-Пярэдняя Памеранія"), ("bg", "Мекленбург-Предна Померания"), ("bn", "মেকলেনব\u{9c1}র\u{9cd}গ-ভোপোম\u{9be}ন"), ("bs", "Mecklenburg-Zapadno Pomorje"), ("ca", "Mecklenburg - Pomerània Occidental"), ("ccp", "𑄟𑄬𑄇\u{11134}𑄣𑄬𑄚\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}-𑄞\u{1112e}𑄢\u{11134}𑄛\u{11127}𑄟𑄬𑄚\u{11134}"), ("ceb", "Mecklenburg-Western Pomerania"), ("cs", "Meklenbursko-Přední Pomořansko"), ("cy", "Mecklenburg-Vorpommern"), ("da", "Mecklenburg-Vorpommern"), ("de", "Mecklenburg-Vorpommern"), ("el", "Μεκλεμβούργο-Δυτική Πομερανία"), ("en", "Mecklenburg-Vorpommern"), ("es", "Mecklemburgo-Pomerania Occidental"), ("et", "Mecklenburg-Vorpommern"), ("eu", "Mecklenburg-Aurrepomerania"), ("fa", "مکلنبورگ-فورپومرن"), ("fi", "Mecklenburg-Etu-Pommeri"), ("fr", "Mecklembourg-Poméranie-Occidentale"), ("ga", "Mecklenburg-Vorpommern"), ("gl", "Mecklemburgo-Antepomerania - Mecklenburg-Vorpommern"), ("gu", "મ\u{ac7}ક\u{acd}લ\u{ac7}નબર\u{acd}ગ-વોર\u{acd}પોમ\u{acd}મ\u{ac7}ર\u{acd}ન"), ("he", "מקלנבורג-מערב פומרניה"), ("hi", "म\u{947}क\u{94d}ल\u{947}नबर\u{94d}ग-वोर\u{94d}पोम\u{947}र\u{94d}न"), ("hr", "Mecklenburg-Zapadno Pomorje"), ("hu", "Mecklenburg–Elő-Pomeránia"), ("hy", "Մեկլենբուրգ-Առաջավոր Պոմերանիա"), ("id", "Mecklenburg-Vorpommern"), ("is", "Mecklenborg-Vorpommern"), ("it", "Meclemburgo-Pomerania Anteriore"), ("jv", "Mecklenburg-Vorpommern"), ("ka", "მეკლენბურგ-წინა პომერანია"), ("kk", "Мекленбург-Алдыңғы Померания"), ("kn", "ಮ\u{cc6}ಕ\u{ccd}ಲ\u{cc6}ನ\u{ccd}ಬರ\u{ccd}ಗ\u{ccd}-ವೋರ\u{ccd}ಪೊಮ\u{cc6}ರ\u{ccd}ನ\u{ccd}"), ("ko", "메클렌부르크포어포메른 주"), ("lt", "Meklenburgas-Pomeranija"), ("lv", "Mēklenburga-Priekšpomerānija"), ("mk", "Мекленбург-Западна Померанија"), ("mn", "Мекленбург – Өрнө Померан"), ("mr", "म\u{947}क\u{94d}ल\u{947}नब\u{941}र\u{94d}ग-फोरपोम\u{947}र\u{94d}न"), ("ms", "Mecklenburg-Vorpommern"), ("nb", "Mecklenburg-Vorpommern"), ("ne", "म\u{947}क\u{94d}ल\u{947}नबर\u{94d}ग-वोर\u{94d}पोम\u{947}र\u{94d}न"), ("nl", "Mecklenburg-Voor-Pommeren"), ("no", "Mecklenburg-Vorpommern"), ("pl", "Meklemburgia-Pomorze Przednie"), ("pt", "Mecklemburgo-Pomerânia Ocidental"), ("ro", "Mecklenburg-Pomerania Inferioară"), ("ru", "Мекленбург-Передняя Померания"), ("sd", "ميڪلن برگ فرو پامرن"), ("si", "මෙකෙලන\u{dca}බර\u{dca}ග\u{dca} -වොර\u{dca}පොම\u{dca}මෙම\u{dca}"), ("sk", "Meklenbursko-Predpomoransko"), ("sl", "Mecklenburg-Predpomorjanska"), ("sq", "Mecklenburg-Vorpommern"), ("sr", "Мекленбург-Западна Померанија"), ("sr_Latn", "Meklenburg-Zapadna Pomeranija"), ("sv", "Mecklenburg-Vorpommern"), ("sw", "Mecklenburg-Pomerini Magharibi"), ("ta", "மெக\u{bcd}லென\u{bcd}பர\u{bcd}க\u{bcd}-வொர\u{bcd}ப\u{bcd}பொம\u{bcd}மெர\u{bcd}ன\u{bcd}"), ("te", "మ\u{c46}క\u{c4d}ల\u{c46}న\u{c4d}బర\u{c4d}గ\u{c4d}-వ\u{c4b}ర\u{c4d}ప\u{c46}మ\u{c46}మ\u{c4d}"), ("th", "ร\u{e31}ฐเมคเล\u{e34}นบวร\u{e4c}ค-ฟอร\u{e4c}พ\u{e47}อมเม\u{e34}ร\u{e4c}น"), ("tr", "Mecklenburg-Vorpommern"), ("uk", "Мекленбург — Передня Померанія"), ("ur", "مکلنبرگ-ورپورمرن"), ("uz", "Meklenburg-Old Pomeraniya"), ("vi", "Mecklenburg-Vorpommern"), ("yo", "Mecklenburg-Vorpommern"), ("yo_BJ", "Mecklenburg-Vorpommern"), ("yue", "美倫堡－前波瑪恩"), ("yue_Hans", "美伦堡－前波玛恩"), ("zh", "梅克伦堡－前波美拉尼亚")]),
                        unofficial_name_list: ["Mecklenburg-Vorpommern"].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::DE,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6367036), longitude: Some(9.8450765), max_latitude: Some(53.89221329999999), min_latitude: Some(51.29506740000001), max_longitude: Some(11.5982055), min_longitude: Some(6.6539671)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nedersakse"), ("am", "ኒደርዛክስን"), ("ar", "سكسونيا السفلى"), ("az", "Aşağı Saksoniya"), ("be", "Ніжняя Саксонія"), ("bg", "Долна Саксония"), ("bn", "নিড\u{9be}রজ\u{9be}খসেন"), ("bs", "Donja Saksonija"), ("ca", "Baixa Saxònia"), ("ccp", "𑄣\u{11131}𑄢\u{11134} 𑄥𑄬𑄇\u{11134}𑄥\u{1112e}𑄚\u{11128}"), ("ceb", "Lower Saxony"), ("cs", "Dolní Sasko"), ("cy", "Niedersachsen"), ("da", "Niedersachsen"), ("de", "Niedersachsen"), ("el", "Κάτω Σαξωνία"), ("en", "Lower Saxony"), ("es", "Baja Sajonia"), ("et", "Alam-Saksi"), ("eu", "Saxonia Beherea"), ("fa", "نیدرزاکسن"), ("fi", "Ala-Saksi"), ("fr", "Basse-Saxe"), ("ga", "An tSacsain Íochtarach"), ("gl", "Baixa Saxonia - Niedersachsen"), ("gu", "લોવર સ\u{ac7}ક\u{acd}સોની"), ("he", "סקסוניה התחתונה"), ("hi", "निचला स\u{948}क\u{94d}सोनी"), ("hr", "Donja Saska"), ("hu", "Alsó-Szászország"), ("hy", "Ստորին Սաքսոնիա"), ("id", "Niedersachsen"), ("is", "Neðra-Saxland"), ("it", "Bassa Sassonia"), ("ja", "ニーダーザクセン州"), ("jv", "Niedersachsen"), ("ka", "ქვემო საქსონია"), ("kk", "Төменгі Саксония"), ("kn", "ಲೋಯರ\u{ccd} ಸ\u{ccd}ಯಾಕ\u{ccd}ಸೋನ\u{cbf}"), ("ko", "니더작센 주"), ("lt", "Žemutinė Saksonija"), ("lv", "Lejassaksija"), ("mk", "Долна Саксонија"), ("mn", "Доор Саксон"), ("mr", "नीडर जाक\u{94d}सन"), ("ms", "Niedersachsen"), ("nb", "Niedersachsen"), ("nl", "Nedersaksen"), ("no", "Niedersachsen"), ("pl", "Dolna Saksonia"), ("pt", "Baixa Saxônia"), ("ro", "Saxonia Inferioară"), ("ru", "Нижняя Саксония"), ("sd", "لوئر سئڪسني"), ("si", "පහල සැක\u{dca}සොන\u{dd2}"), ("sk", "Dolné Sasko"), ("sl", "Spodnja Saška"), ("so", "Niedersachsen"), ("sq", "Saksonia e Ulët"), ("sr", "Доња Саксонија"), ("sr_Latn", "Donja Saksonija"), ("sv", "Niedersachsen"), ("sw", "Saksonia Chini"), ("ta", "லோயர\u{bcd} ச\u{bbe}க\u{bcd}ஸ\u{bcd}சனி"), ("te", "ల\u{c4b}వర\u{c4d} స\u{c3e}క\u{c4d}స\u{c4b}న\u{c40}"), ("th", "ร\u{e31}ฐโลว\u{e4c}เออร\u{e4c}แซกโซน\u{e35}"), ("tr", "Aşağı Saksonya"), ("uk", "Нижня Саксонія"), ("ur", "نیدرزاکسن"), ("uz", "Niedersachsen"), ("vi", "Niedersachsen"), ("yo", "Lower Saxony"), ("yo_BJ", "Lower Saxony"), ("yue", "下薩克遜"), ("yue_Hans", "下萨克逊"), ("zh", "下萨克森")]),
                        unofficial_name_list: ["Niedersachsen"].to_vec(),
                    }
                ),
                (
                    "NW",
                    Subdivision{
                        name: "NW",
                        country_alpha2: Alpha2::DE,
                        code: "NW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.43323669999999), longitude: Some(7.661593799999999), max_latitude: Some(52.53146959999999), min_latitude: Some(50.322701), max_longitude: Some(9.461634900000002), min_longitude: Some(5.8663425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordryn-Wesfale"), ("am", "ኖርድራይን-ቬስትፋለን"), ("ar", "شمال الراين-وستفاليا"), ("az", "Şimali Reyn-Vestfaliya"), ("be", "Паўночны Рэйн-Вестфалія"), ("bg", "Северен Рейн-Вестфалия"), ("bn", "নর\u{9cd}থ রিনে-ভেস\u{9cd}টফ\u{9be}লিয\u{9bc}\u{9be}"), ("bs", "Sjeverna Rajna-Vestfalija"), ("ca", "Rin del Nord - Westfàlia"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄢\u{1112d}𑄚\u{11134}-𑄤𑄬𑄌\u{11134}𑄜𑄣\u{11128}𑄠"), ("ceb", "Nordrhein-Westfalen"), ("cs", "Severní Porýní-Vestfálsko"), ("cy", "Nordrhein-Westfalen"), ("da", "Nordrhein-Westfalen"), ("de", "Nordrhein-Westfalen"), ("el", "Βόρεια Ρηνανία-Βεστφαλία"), ("en", "North Rhine-Westphalia"), ("es", "Renania del Norte-Westfalia"), ("et", "Nordrhein-Westfalen"), ("eu", "Ipar Renania-Westfalia"), ("fa", "نوردراین-وستفالن"), ("fi", "Nordrhein-Westfalen"), ("fr", "Rhénanie-du-Nord-Westphalie"), ("ga", "Tuaisceart na Réine agus an Viostfáil"), ("gl", "Renania do Norte-Westfalia - Nordrhein-Westfalen"), ("gu", "નોર\u{acd}થ રાઇન-વ\u{ac7}સ\u{acd}ટફાલિયા"), ("he", "נורדריין-וסטפאליה"), ("hi", "उत\u{94d}तरी राइन"), ("hr", "Sjeverna Rajna-Vestfalija"), ("hu", "Észak-Rajna-Vesztfália"), ("hy", "Հյուսիսային Հռենոս-Վեստֆալիա"), ("id", "Nordrhein-Westfalen"), ("is", "Norðurrín-Vestfalía"), ("it", "Renania Settentrionale-Vestfalia"), ("jv", "Nordrhein-Westfalen"), ("ka", "ჩრდილოეთი რაინ-ვესტფალია"), ("kk", "Солтүстік Рейн-Вестфалия"), ("kn", "ಉತ\u{ccd}ತರ ರೈನ\u{ccd}-ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}ಫಾಲ\u{cbf}ಯಾ"), ("ko", "노르트라인베스트팔렌 주"), ("lt", "Šiaurės Reinas-Vestfalija"), ("lv", "Ziemeļreina-Vestfālene"), ("mk", "Северна Рајна-Вестфалија"), ("mn", "Умар Райн-Вестфален"), ("mr", "नोर\u{94d}डऱ\u{94d}हाइन-व\u{947}स\u{94d}टफालन"), ("ms", "Nordrhein-Westfalen"), ("nb", "Nordrhein-Westfalen"), ("ne", "उत\u{94d}तरी राइन"), ("nl", "Noordrijn-Westfalen"), ("no", "Nordrhein-Westfalen"), ("pa", "ਉ\u{a71}ਤਰੀ ਰਾਈਨ-ਪ\u{a71}ਛਮੀ ਫ\u{a3c}ਾਲਨ"), ("pl", "Nadrenia Północna-Westfalia"), ("pt", "Renânia do Norte-Vestfália"), ("ro", "Renania de Nord-Westfalia"), ("ru", "Северный Рейн-Вестфалия"), ("sd", "نارٿ رائن وستڦيليا"), ("si", "උත\u{dd4}ර\u{dd4} රය\u{dd2}නේ- වෙස\u{dca}ට\u{dca}ෆ\u{dcf}ල\u{dd2}ය\u{dcf}"), ("sk", "Severné Porýnie-Vestfálsko"), ("sl", "Severno Porenje-Vestfalija"), ("sq", "Nordrhein-Westfalen"), ("sr", "Северна Рајна-Вестфалија"), ("sr_Latn", "Severna Rajna-Vestfalija"), ("sv", "Nordrhein-Westfalen"), ("sw", "Rhine Kaskazini-Westfalia"), ("ta", "வட ரைன\u{bcd}-வெஸ\u{bcd}ட\u{bcd}ஃப\u{bbe}லிய\u{bbe}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d}\u{200c} ర\u{c48}న\u{c4d}-వ\u{c46}స\u{c4d}ట\u{c4d}\u{200c}ఫ\u{c3e}ల\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐนอร\u{e4c}ทไรน\u{e4c}-เว\u{e47}สท\u{e4c}ฟาเล\u{e34}น"), ("tr", "Kuzey Ren-Vestfalya"), ("uk", "Північний Рейн-Вестфалія"), ("ur", "نوردرائن-ویسٹ فالیا"), ("uz", "Shimoliy Reyn-Vestfaliya"), ("vi", "Nordrhein-Westfalen"), ("yo", "Nordrhein-Westfalen"), ("yo_BJ", "Nordrhein-Westfalen"), ("yue", "北萊茵－西法倫"), ("yue_Hans", "北莱茵－西法伦"), ("zh", "北莱茵－威斯特法伦")]),
                        unofficial_name_list: ["Nordrhein-Westfalen", "Rhénanie-Westphalie"].to_vec(),
                    }
                ),
                (
                    "RP",
                    Subdivision{
                        name: "RP",
                        country_alpha2: Alpha2::DE,
                        code: "RP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.118346), longitude: Some(7.3089527), max_latitude: Some(50.9423053), min_latitude: Some(48.9664186), max_longitude: Some(8.5083135), min_longitude: Some(6.1122659)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rynland-Palts"), ("am", "ራይንላንት-ፕፋልጽ"), ("ar", "راينلند بالاتينات"), ("az", "Reynland-Pfalts"), ("be", "Рэйнланд-Пфальц"), ("bg", "Рейнланд-Пфалц"), ("bn", "রিনেল\u{9cd}য\u{9be}ন\u{9cd}ড-প\u{9cd}য\u{9be}ল\u{9be}টিন\u{9be}ট\u{9be}"), ("bs", "Porajnje-Falačka"), ("ca", "Renània-Palatinat"), ("ccp", "𑄢\u{1112d}𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}-𑄛𑄣𑄑\u{11128}𑄚𑄬𑄖\u{11134}"), ("ceb", "Rheinland-Pfalz"), ("cs", "Porýní-Falc"), ("cy", "Rheinland-Pfalz"), ("da", "Rheinland-Pfalz"), ("de", "Rheinland-Pfalz"), ("el", "Ρηνανία-Παλατινάτο"), ("en", "Rhineland-Palatinate"), ("es", "Renania-Palatinado"), ("et", "Rheinland-Pfalz"), ("eu", "Renania-Palatinatua"), ("fa", "راینلاند-فالتس"), ("fi", "Rheinland-Pfalz"), ("fr", "Rhénanie-Palatinat"), ("ga", "Tír na Réine agus an Phalaitíneacht"), ("gl", "Renania-Palatinado - Rheinland-Pfalz"), ("gu", "રાઈનલ\u{ac7}ન\u{acd}ડ-પ\u{ac7}લ\u{ac7}ટિન\u{ac7}ટ"), ("he", "ריינלנד-פפאלץ"), ("hi", "राइनल\u{948}न\u{94d}ड"), ("hr", "Falačko Porajnje"), ("hu", "Rajna-vidék–Pfalz"), ("hy", "Ռայնլանդ Պֆալց"), ("id", "Rheinland-Pfalz"), ("is", "Rínarland-Pfalz"), ("it", "Renania-Palatinato"), ("jv", "Rheinland-Pfalz"), ("ka", "რაინლანდ-პფალცი"), ("kk", "Рейнланд-Пфальц"), ("kn", "ರೈನ\u{ccd} ಲ\u{ccd}ಯಾಂಡ\u{ccd}-ಪಲಟ\u{cbf}ನೇಟ\u{ccd}"), ("ko", "라인란트팔츠 주"), ("lt", "Reino kraštas-Pfalcas"), ("lv", "Reinzeme-Pfalca"), ("mk", "Рајнска област-Пфалц"), ("mn", "Райнланд-Пфальц"), ("mr", "र\u{94d}\u{200d}हाइनला\u{902}ड-फाल\u{94d}त\u{94d}स"), ("ms", "Rheinland-Pfalz"), ("nb", "Rheinland-Pfalz"), ("ne", "राइनल\u{94d}यान\u{94d}ड"), ("nl", "Rijnland-Palts"), ("no", "Rheinland-Pfalz"), ("pl", "Nadrenia-Palatynat"), ("pt", "Renânia-Palatinado"), ("ro", "Renania-Palatinat"), ("ru", "Рейнланд-Пфальц"), ("sd", "رائن لينڊ پئليٽينيٽ"), ("si", "රය\u{dd2}න\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dd2}ෆආල\u{dca}ස\u{dca}"), ("sk", "Porýnie-Falcko"), ("sl", "Porenje-Pfalška"), ("sq", "Rheinland-Pfalz"), ("sr", "Рајна-Палатинат"), ("sr_Latn", "Rajna-Palatinat"), ("sv", "Rheinland-Pfalz"), ("sw", "Rhine-Palatino"), ("ta", "ரைன\u{bcd}ல\u{bbe}ந\u{bcd}து-பலென\u{bcd}டினேட\u{bcd}"), ("te", "ర\u{c48}న\u{c4d}ల\u{c3e}ండ\u{c4d}-ప\u{c3e}ల\u{c3e}ట\u{c3f}న\u{c47}ట\u{c4d}"), ("th", "ร\u{e31}ฐไรน\u{e4c}ล\u{e31}นท\u{e4c}-ฟ\u{e31}ลทซ\u{e4c}"), ("tr", "Renanya-Palatina"), ("uk", "Рейнланд-Пфальц"), ("ur", "رائن لینڈ-پلاتینیت"), ("vi", "Rheinland-Pfalz"), ("yo", "Rhineland-Palatinate"), ("yo_BJ", "Rhineland-Palatinate"), ("yue", "萊茵－普法茨"), ("yue_Hans", "莱茵－普法茨"), ("zh", "莱茵兰－普法尔茨")]),
                        unofficial_name_list: ["Rheinland-Pfalz", "Rhineland-Palatinate", "Rhénanie-Palatinat"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::DE,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.21936720000001), longitude: Some(9.696116700000001), max_latitude: Some(55.05812359999999), min_latitude: Some(53.3608263), max_longitude: Some(11.3129207), min_longitude: Some(7.864961300000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sleeswyk-Holstein"), ("am", "ሽለስቭክ-ሖልሽታይን"), ("ar", "شليسفيغ هولشتاين"), ("az", "Şlezviq-Holşteyn"), ("be", "Шлезвіг-Гольштэйн"), ("bg", "Шлезвиг-Холщайн"), ("bn", "শ\u{9cd}লেসভিগ-হোলস\u{9cd}ট\u{9be}ইন"), ("bs", "Schleswig-Holstein"), ("ca", "Slesvig-Holstein"), ("ccp", "𑄌\u{11133}𑄇𑄬𑄌\u{11134}𑄃\u{1112a}𑄃\u{11128}𑄇\u{11134}-𑄦\u{1112e}𑄣\u{11134}𑄌\u{11133}𑄑𑄬𑄃\u{11128}𑄚\u{11134}"), ("ceb", "Schleswig-Holstein"), ("cs", "Šlesvicko-Holštýnsko"), ("cy", "Schleswig-Holstein"), ("da", "Slesvig-Holsten"), ("de", "Schleswig-Holstein"), ("el", "Σλέσβιχ-Χόλσταϊν"), ("en", "Schleswig-Holstein"), ("es", "Schleswig-Holstein"), ("et", "Schleswig-Holstein"), ("eu", "Schleswig-Holstein"), ("fa", "اشلسویگ-هولشتاین"), ("fi", "Schleswig-Holstein"), ("fr", "Schleswig-Holstein"), ("ga", "Schleswig-Holstein"), ("gl", "Schleswig-Holstein"), ("gu", "સ\u{acd}કલ\u{ac7}સ\u{acd}વિગ-હોલસ\u{acd}ટ\u{ac7}ઇન"), ("he", "שלזוויג-הולשטיין"), ("hi", "श\u{94d}ल\u{947}सविग-होल\u{94d}सटीन"), ("hr", "Schleswig-Holstein"), ("hu", "Schleswig-Holstein"), ("hy", "Շլեզվիգ-Հոլշտայն"), ("id", "Schleswig-Holstein"), ("is", "Slésvík-Holtsetaland"), ("it", "Schleswig-Holstein"), ("jv", "Schleswig-Holstein"), ("ka", "შლეზვიგ-ჰოლშტაინი"), ("kk", "Шлезвиг-Гольштейн"), ("kn", "ಶ\u{ccd}ಲ\u{cc6}ಸ\u{ccd}ವ\u{cbf}ಗ\u{ccd}-ಹೋಲ\u{ccd}ಸ\u{ccd}ಟೈನ\u{ccd}"), ("ko", "슐레스비히홀슈타인 주"), ("lt", "Šlėzvigas-Holšteinas"), ("lv", "Šlēsviga-Holšteina"), ("mk", "Шлезвиг-Холштајн"), ("mn", "Шлесвиг-Хольштайн"), ("mr", "श\u{94d}ल\u{947}स\u{94d}विग-होल\u{94d}श\u{94d}टाइन"), ("ms", "Schleswig-Holstein"), ("nb", "Schleswig-Holstein"), ("ne", "श\u{94d}ल\u{947}सविग-होल\u{94d}सटीन"), ("nl", "Sleeswijk-Holstein"), ("no", "Schleswig-Holstein"), ("pl", "Szlezwik-Holsztyn"), ("pt", "Schleswig-Holstein"), ("ro", "Schleswig-Holstein"), ("ru", "Шлезвиг-Гольштейн"), ("sd", "شليس وگ ھوليسٽن"), ("si", "ශ\u{dca}ලෙස\u{dca}ව\u{dd2}ග\u{dca}-හොල\u{dca}ස\u{dca}ටය\u{dd2}න\u{dca}"), ("sk", "Šlezvicko-Holštajnsko"), ("sl", "Schleswig-Holstein"), ("sq", "Schleswig-Holstein"), ("sr", "Шлезвиг-Холштајн"), ("sr_Latn", "Šlezvig-Holštajn"), ("sv", "Schleswig-Holstein"), ("sw", "Schleswig-Holstein"), ("ta", "சிலெஸ\u{bcd}விக\u{bcd}-ஹொல\u{bcd}ஸ\u{bcd}டெயின\u{bcd}"), ("te", "ష\u{c4d}ల\u{c46}స\u{c4d}వ\u{c3f}గ\u{c4d}-హ\u{c4b}ల\u{c4d}\u{200c}స\u{c4d}ట\u{c40}న\u{c4d}"), ("th", "ร\u{e31}ฐชเลสว\u{e34}ช-ฮ\u{e47}อลชไตน\u{e4c}"), ("tr", "Schleswig-Holstein"), ("uk", "Шлезвіг-Гольштейн"), ("ur", "شلسویگ-ہولشتائن"), ("uz", "Schleswig-Holstein"), ("vi", "Schleswig-Holstein"), ("yo", "Schleswig-Holstein"), ("yo_BJ", "Schleswig-Holstein"), ("yue", "舒勒斯維－荷斯泰因"), ("yue_Hans", "舒勒斯维－荷斯泰因"), ("zh", "石勒苏益格-荷尔斯泰因")]),
                        unofficial_name_list: ["Schleswig-Holstein"].to_vec(),
                    }
                ),
                (
                    "SL",
                    Subdivision{
                        name: "SL",
                        country_alpha2: Alpha2::DE,
                        code: "SL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.3964234), longitude: Some(7.0229607), max_latitude: Some(49.6394088), min_latitude: Some(49.1119452), max_longitude: Some(7.404583799999999), min_longitude: Some(6.357608399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saarland"), ("am", "ዛዓርላንት"), ("ar", "سارلاند"), ("az", "Saarland"), ("be", "Саар"), ("bg", "Саарланд"), ("bn", "জ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Saarland"), ("ca", "Saarland"), ("ccp", "𑄥𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Saarland"), ("cs", "Sársko"), ("cy", "Saarland"), ("da", "Saarland"), ("de", "Saarland"), ("el", "Σάαρλαντ"), ("en", "Saarland"), ("es", "Sarre"), ("et", "Saarimaa"), ("eu", "Sarre"), ("fa", "زارلند"), ("fi", "Saarland"), ("fr", "Sarre"), ("ga", "An tSárlainn"), ("gl", "Sarre - Saarland"), ("gu", "સારલ\u{ac7}ન\u{acd}ડ"), ("he", "חבל הסאר"), ("hi", "सारल\u{948}\u{902}ड"), ("hr", "Saarska"), ("hu", "Saar-vidék"), ("hy", "Սաարլանդ"), ("id", "Saarland"), ("is", "Saarland"), ("it", "Saarland"), ("ja", "ザールラント州"), ("jv", "Saarland"), ("ka", "ზაარლანდი"), ("kk", "Саар"), ("kn", "ಸಾರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "자를란트 주"), ("ky", "Саар"), ("lt", "Saro kraštas"), ("lv", "Zāra"), ("mk", "Сар"), ("mn", "Заарланд"), ("mr", "जारला\u{902}ड"), ("ms", "Saarland"), ("nb", "Saarland"), ("ne", "सारल\u{94d}यान\u{94d}ड"), ("nl", "Saarland"), ("no", "Saarland"), ("pa", "ਜ\u{a3c}ਾਰਲਾ\u{a02}ਡ"), ("pl", "Saara"), ("ps", "زارلاند"), ("pt", "Sarre"), ("ro", "Saarland"), ("ru", "Саар"), ("sd", "سارلنڊ"), ("si", "ස\u{dcf}ර\u{dca}ලන\u{dca}ඩ\u{dca}"), ("sk", "Sársko"), ("sl", "Posarje"), ("sq", "Saarland"), ("sr", "Сарланд"), ("sr_Latn", "Sarland"), ("sv", "Saarland"), ("sw", "Saar"), ("ta", "ச\u{bbe}ர\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "స\u{c3e}ర\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐซาร\u{e4c}ล\u{e31}นท\u{e4c}"), ("tr", "Saarland"), ("uk", "Саарланд"), ("ur", "سارلنڈ"), ("uz", "Saarland"), ("vi", "Saarland"), ("yo", "Saarland"), ("yo_BJ", "Saarland"), ("yue", "沙亞"), ("yue_Hans", "沙亚"), ("zh", "萨尔")]),
                        unofficial_name_list: ["Saarland"].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::DE,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.1045407), longitude: Some(13.2017384), max_latitude: Some(51.6847089), min_latitude: Some(50.1713633), max_longitude: Some(15.0418962), min_longitude: Some(11.8714358)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sakse"), ("am", "ዛክስን"), ("ar", "ساكسونيا"), ("az", "Saksoniya"), ("be", "Саксонія"), ("bg", "Саксония"), ("bn", "জ\u{9cd}য\u{9be}ক\u{9cd}সোনি"), ("bs", "Saksonija"), ("ca", "Saxònia"), ("ccp", "𑄥𑄬𑄇\u{11134}𑄥\u{1112e}𑄚\u{11128}"), ("ceb", "Saxony"), ("cs", "Sasko"), ("cy", "Sachsen"), ("da", "Sachsen"), ("de", "Sachsen"), ("el", "Σαξωνία"), ("en", "Saxony"), ("es", "Sajonia"), ("et", "Saksimaa"), ("eu", "Saxonia"), ("fa", "زاکسن"), ("fi", "Saksi"), ("fr", "Saxe"), ("ga", "An tSacsain"), ("gl", "Saxonia - Sachsen"), ("gu", "સ\u{ac7}ક\u{acd}સોની"), ("he", "סקסוניה"), ("hi", "स\u{948}क\u{94d}सोनी"), ("hr", "Saska"), ("hu", "Szászország"), ("hy", "Սաքսոնիա"), ("id", "Sachsen"), ("is", "Saxland"), ("it", "Sassonia"), ("ja", "ザクセン州"), ("jv", "Sachsen"), ("ka", "საქსონია"), ("kk", "Саксония еркін мемлекеті"), ("kn", "ಸ\u{ccd}ಯಾಕ\u{ccd}ಸೋನ\u{cbf}"), ("ko", "작센 주"), ("lt", "Saksonija"), ("lv", "Saksija"), ("mk", "Саксонија"), ("mn", "Саксон"), ("mr", "जाक\u{94d}सन"), ("ms", "Sachsen"), ("nb", "Sachsen"), ("ne", "स\u{94d}याक\u{94d}सोनी"), ("nl", "Saksen"), ("no", "Sachsen"), ("pa", "ਜ\u{a3c}ਾਕਸਨ"), ("pl", "Saksonia"), ("pt", "Saxónia"), ("ro", "Saxonia"), ("ru", "Саксония"), ("sd", "سئڪسني"), ("si", "සැක\u{dca}සොන\u{dd2}"), ("sk", "Sasko"), ("sl", "Saška"), ("sq", "Saksonia"), ("sr", "Саксонија"), ("sr_Latn", "Saksonija"), ("sv", "Sachsen"), ("sw", "Saksonia"), ("ta", "ச\u{bbe}க\u{bcd}ஸ\u{bcd}சனி"), ("te", "స\u{c3e}క\u{c4d}స\u{c4b}న\u{c40}"), ("th", "ร\u{e31}ฐซ\u{e31}คเซ\u{e34}น"), ("tr", "Saksonya"), ("uk", "Вільна держава Саксонія"), ("ur", "زاکسن"), ("uz", "Saksoniya"), ("vi", "Sachsen"), ("yo", "Saxony"), ("yo_BJ", "Saxony"), ("yue", "薩克遜"), ("yue_Hans", "萨克逊"), ("zh", "萨克森自由州")]),
                        unofficial_name_list: ["Sachsen"].to_vec(),
                    }
                ),
                (
                    "ST",
                    Subdivision{
                        name: "ST",
                        country_alpha2: Alpha2::DE,
                        code: "ST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.9502649), longitude: Some(11.6922735), max_latitude: Some(53.0404507), min_latitude: Some(50.9378782), max_longitude: Some(13.1869875), min_longitude: Some(10.5608391)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sakse-Anhalt"), ("am", "ዛክስን-አንሃልት"), ("ar", "ساكسونيا أنهالت"), ("az", "Saksoniya-Anhalt"), ("be", "Саксонія-Ангальт"), ("bg", "Саксония-Анхалт"), ("bn", "জ\u{9cd}য\u{9be}ক\u{9cd}সোনি-আনহ\u{9be}ল\u{9cd}ট"), ("bs", "Saksonija-Anhalt"), ("ca", "Saxònia-Anhalt"), ("ccp", "𑄥𑄬𑄇\u{11134}𑄥\u{1112e}𑄚\u{11128}-\u{11131}𑄃𑄚\u{11134}𑄦𑄣\u{11133}𑄑\u{11134}"), ("ceb", "Saxony-Anhalt"), ("cs", "Sasko-Anhaltsko"), ("cy", "Sachsen-Anhalt"), ("da", "Sachsen-Anhalt"), ("de", "Sachsen-Anhalt"), ("el", "Σαξωνία-Άνχαλτ"), ("en", "Saxony-Anhalt"), ("es", "Sajonia-Anhalt"), ("et", "Saksi-Anhalt"), ("eu", "Saxonia-Anhalt"), ("fa", "زاکسن-آنهالت"), ("fi", "Saksi-Anhalt"), ("fr", "Saxe-Anhalt"), ("ga", "An tSacsain-Anhalt"), ("gl", "Saxonia-Anhalt - Sachsen-Anhalt"), ("gu", "સ\u{ac7}ક\u{acd}સોની- એન\u{acd}હાલ\u{acd}ટ"), ("he", "סקסוניה-אנהלט"), ("hi", "स\u{948}क\u{94d}सोनी-एन\u{94d}हाल\u{94d}ट"), ("hr", "Saska-Anhalt"), ("hu", "Szász-Anhalt"), ("hy", "Սաքսոնիա-Անհալթ"), ("id", "Sachsen-Anhalt"), ("is", "Saxland-Anhalt"), ("it", "Sassonia-Anhalt"), ("jv", "Sachsen-Anhalt"), ("ka", "საქსონია-ანჰალტი"), ("kk", "Саксония-Анхальт"), ("kn", "ಸ\u{ccd}ಯಾಕ\u{ccd}ಸೋನ\u{cbf}-ಅನ\u{ccd}ಹಾಲ\u{ccd}ಟ\u{ccd}"), ("ko", "작센안할트 주"), ("lt", "Saksonija-Anhaltas"), ("lv", "Saksija-Anhalte"), ("mk", "Саксонија-Анхалт"), ("mn", "Саксон-Анхальт"), ("mr", "जाक\u{94d}सन-आनहाल\u{94d}ट"), ("ms", "Sachsen-Anhalt"), ("nb", "Sachsen-Anhalt"), ("nl", "Saksen-Anhalt"), ("no", "Sachsen-Anhalt"), ("pl", "Saksonia-Anhalt"), ("pt", "Saxônia-Anhalt"), ("ro", "Saxonia-Anhalt"), ("ru", "Саксония-Анхальт"), ("sd", "سئڪسني اينھالٽ"), ("si", "සැක\u{dca}සොන\u{dd2}-අන\u{dca}හල\u{dca}ට\u{dca}"), ("sk", "Sasko-Anhaltsko"), ("sl", "Saška-Anhalt"), ("sq", "Saksonia Anhalt"), ("sr", "Саксонија-Анхалт"), ("sr_Latn", "Saksonija-Anhalt"), ("sv", "Sachsen-Anhalt"), ("sw", "Saksonia-Anhalt"), ("ta", "ச\u{bbe}க\u{bcd}ஸ\u{bcd}சனி-அன\u{bcd}ஹ\u{bbe}ல\u{bcd}ட\u{bcd}"), ("te", "స\u{c3e}క\u{c4d}స\u{c4b}న\u{c40}-అన\u{c4d}హ\u{c3e}ల\u{c4d}ట\u{c4d}"), ("th", "ร\u{e31}ฐแซกโซน\u{e35}-อ\u{e31}นฮ\u{e31}ลต\u{e4c}"), ("tr", "Saksonya-Anhalt"), ("uk", "Саксонія-Ангальт"), ("ur", "زاکسن-آنہالت"), ("uz", "Saksoniya-Anhalt"), ("vi", "Sachsen-Anhalt"), ("yo", "Saxony-Anhalt"), ("yo_BJ", "Saxony-Anhalt"), ("yue", "薩克遜－雁候"), ("yue_Hans", "萨克逊－雁候"), ("zh", "萨克森－安哈尔特")]),
                        unofficial_name_list: ["Sachsen-Anhalt"].to_vec(),
                    }
                ),
                (
                    "TH",
                    Subdivision{
                        name: "TH",
                        country_alpha2: Alpha2::DE,
                        code: "TH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.0109892), longitude: Some(10.845346), max_latitude: Some(51.6489359), min_latitude: Some(50.20434669999999), max_longitude: Some(12.6539327), min_longitude: Some(9.876984000000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Thüringen"), ("am", "ቲውሪንገን"), ("ar", "تورينغن"), ("az", "Türingiya"), ("be", "Цюрынгія"), ("bg", "Тюрингия"), ("bn", "থ\u{9c1}রিনগিয\u{9bc}\u{9be}"), ("bs", "Tiringija"), ("ca", "Turíngia"), ("ccp", "𑄗\u{1112a}𑄢\u{11128}\u{11101}𑄉\u{11128}𑄠"), ("ceb", "Thuringia"), ("cs", "Durynsko"), ("cy", "Thüringen"), ("da", "Thüringen"), ("de", "Thüringen"), ("el", "Θουριγγία"), ("en", "Thuringia"), ("es", "Turingia"), ("et", "Tüüringi"), ("eu", "Turingia"), ("fa", "تورینگن"), ("fi", "Thüringen"), ("fr", "Thuringe"), ("ga", "Thüringen"), ("gl", "Turinxia - Thüringen"), ("gu", "થ\u{ac1}રિન\u{acd}જિયા"), ("he", "תורינגיה"), ("hi", "ठ\u{941}र\u{941}\u{902}गिया"), ("hr", "Tiringija"), ("hu", "Türingia"), ("hy", "Թուրինգիա"), ("id", "Thüringen"), ("is", "Þýringaland"), ("it", "Turingia"), ("ja", "テューリンゲン州"), ("jv", "Thüringen"), ("ka", "თიურინგია"), ("kk", "Тюрингия"), ("kn", "ತುರ\u{cbf}ಂಗ\u{cbf}ಯ"), ("ko", "튀링겐 주"), ("ky", "Тюрингия"), ("lt", "Tiuringija"), ("lv", "Tīringene"), ("mk", "Тирингија"), ("mn", "Тюринг"), ("mr", "थ\u{94d}य\u{941}रि\u{902}ग\u{947}न"), ("ms", "Thüringen"), ("nb", "Thüringen"), ("ne", "ठ\u{941}र\u{941}\u{902}गिया"), ("nl", "Thüringen"), ("no", "Thüringen"), ("pl", "Turyngia"), ("pt", "Turíngia"), ("ro", "Turingia"), ("ru", "Тюрингия"), ("sd", "ٿرينجيا"), ("si", "ත\u{dd4}ර\u{dd2}න\u{dca}ජ\u{dd2}ය\u{dcf}"), ("sk", "Durínsko"), ("sl", "Turingija"), ("sq", "Turingia"), ("sr", "Тирингија"), ("sr_Latn", "Tiringija"), ("sv", "Thüringen"), ("sw", "Thuringia"), ("ta", "துரின\u{bcd}ஜிய\u{bbe}"), ("te", "తుర\u{c3f}ంగ\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐท\u{e37}อร\u{e34}งเง\u{e34}น"), ("tr", "Türingiya"), ("uk", "Тюрингія"), ("ur", "تورینگن"), ("uz", "Turingiya"), ("vi", "Thüringen"), ("yo", "Thuringia"), ("yo_BJ", "Thuringia"), ("yue", "脫靈根"), ("yue_Hans", "脱灵根"), ("zh", "图林根")]),
                        unofficial_name_list: ["Thüringen"].to_vec(),
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
#[cfg(feature = "de")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::DE,
        alpha3: Alpha3::DEU,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 49,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::GM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::GER),
        iso_long_name: "The Federal Republic of Germany",
        iso_short_name: "Germany",
        official_language_list: ["de"].to_vec(),
        spoken_language_list: ["de"].to_vec(),
        national_destination_code_length_list: [2, 3, 4, 5].to_vec(),
        national_number_length_list: [6, 7, 8, 9, 10, 11].to_vec(),
        national_prefix: "0",
        nationality: Some("German"),
        number: "276",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "DE",
        unofficial_name_list: [
            "Germany",
            "Deutschland",
            "Allemagne",
            "Alemania",
            "ドイツ",
            "Duitsland",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Germany"),
            ("af", "Duitsland"),
            ("ak", "Germany"),
            ("am", "ጀሴመን"),
            ("an", "Germany"),
            ("ar", "ألمانيا"),
            (
                "as",
                "জ\u{9be}ইচ\u{9cd}\u{200c}ল\u{9be}মিক\u{9cd}ম\u{9be}নী",
            ),
            ("ay", "Germany"),
            ("az", "Almaniya"),
            ("ba", "Germany"),
            ("be", "Германія"),
            ("bg", "Германия"),
            ("bi", "Germany"),
            ("bn", "জ\u{9be}র\u{9cd}ম\u{9be}নি"),
            ("bn_IN", "জ\u{9be}র\u{9cd}ম\u{9be}নি"),
            ("br", "Alamagn"),
            ("bs", "Njemačka"),
            ("ca", "Alemanya"),
            ("ce", "Германи"),
            ("ch", "Alemaña"),
            ("cs", "Německo"),
            ("cv", "Германи"),
            ("cy", "Yr Almaen"),
            ("da", "Tyskland"),
            ("de", "Deutschland"),
            (
                "dv",
                "ޖ\u{7a6}ރ\u{7aa}މ\u{7a6}ނ\u{7aa}ވ\u{7a8}ލ\u{7a7}ތ\u{7b0}",
            ),
            ("dz", "ཇར་མ་ན\u{f72}།"),
            ("ee", "Germany"),
            ("el", "Γερμανία"),
            ("en", "Germany"),
            ("eo", "Germanio"),
            ("es", "Alemania"),
            ("et", "Saksamaa"),
            ("eu", "Alemania"),
            ("fa", "آلمان"),
            ("ff", "Almaanya"),
            ("fi", "Saksa"),
            ("fo", "Týskland"),
            ("fr", "Allemagne"),
            ("fy", "Dútslân"),
            ("ga", "An Ghearmáin"),
            ("gl", "Alemaña"),
            ("gn", "Germany"),
            ("gu", "જર\u{acd}મની"),
            ("gv", "Yn Ghermaan"),
            ("ha", "Jamus"),
            ("he", "גרמניה"),
            ("hi", "जर\u{94d}मनी"),
            ("hr", "Njemačka"),
            ("ht", "Almay"),
            ("hu", "Németország"),
            ("hy", "Գերմանիա"),
            ("ia", "Germania"),
            ("id", "Jerman"),
            ("io", "Germania"),
            ("is", "Þýskaland"),
            ("it", "Germania"),
            ("iu", "ᔮᒪᓂ"),
            ("ja", "ドイツ"),
            ("ka", "გერმანია"),
            ("ki", "Germany"),
            ("kk", "Германия"),
            ("kl", "Germany"),
            ("km", "អាល\u{17d2}ល\u{17ba}ម\u{17c9}ង\u{17cb}"),
            ("kn", "ಜರ\u{ccd}ಮನ\u{cbf}"),
            ("ko", "독일"),
            ("ku", "Almanya"),
            ("kv", "Германия"),
            ("kw", "Almayn"),
            ("ky", "Германия"),
            ("lo", "ປະເທດເຢ\u{eb1}ຽລະມ\u{eb1}ນ"),
            ("lt", "Vokietija"),
            ("lv", "Vācija"),
            ("mi", "Tiamana"),
            ("mk", "Германија"),
            ("ml", "ജര\u{d4d}\u{200d}മനി"),
            ("mn", "Герман"),
            ("mr", "जर\u{94d}मनी"),
            ("ms", "Jerman"),
            ("mt", "Ġermanja"),
            ("my", "ဂျာမန\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Djermani"),
            ("nb", "Tyskland"),
            ("ne", "जर\u{94d}मनी"),
            ("nl", "Duitsland"),
            ("nn", "Tyskland"),
            ("nv", "Béésh Bichʼahii Bikéyah"),
            ("oc", "Alemanha"),
            ("or", "ଜର\u{b4d}ମ\u{b3e}ନୀ"),
            ("pa", "ਜਰਮਨੀ"),
            ("pi", "जर\u{94d}मनी"),
            ("pl", "Niemcy"),
            ("ps", "المان"),
            ("pt", "Alemanha"),
            ("pt_BR", "Alemanha"),
            ("ro", "Germania"),
            ("ru", "Германия"),
            ("rw", "Ubudage"),
            ("sc", "Germània"),
            ("sd", "جرمني"),
            ("si", "ජර\u{dca}මන\u{dd2}ය"),
            ("sk", "Nemecko"),
            ("sl", "Nemčija"),
            ("so", "Jarmal"),
            ("sq", "Gjermani"),
            ("sr", "Немачка"),
            ("sv", "Tyskland"),
            ("sw", "Germany"),
            ("ta", "ஜெர\u{bcd}மனி"),
            ("te", "జర\u{c4d}మన\u{c40}"),
            ("tg", "Олмон"),
            ("th", "เยอรมน\u{e35}"),
            ("ti", "ጀርመን"),
            ("tk", "Germaniýa"),
            ("tl", "Alemanya"),
            ("tr", "Almanya"),
            ("tt", "Алманиа"),
            ("ug", "گېرمانىيە"),
            ("uk", "Німеччина"),
            ("ur", "جرمنی"),
            ("uz", "Olmoniya"),
            ("ve", "Germany"),
            ("vi", "Đức"),
            ("wa", "Almagne"),
            ("wo", "Almaañ"),
            ("xh", "Jamani"),
            ("yo", "Jẹ\u{301}mánì"),
            ("zh_CN", "德国"),
            ("zh_HK", "德國"),
            ("zh_TW", "德國"),
            ("zu", "IJalimani"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
