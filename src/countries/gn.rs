// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Guinea

#[cfg(all(feature = "gn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GN;
    pub const ALPHA3: Alpha3 = Alpha3::GIN;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 224;
    pub const CURRENCY_CODE: &str = "GNF";
    pub const GEC: Option<GEC> = Some(GEC::GV);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("GUI");
    pub const ISO_SHORT_NAME: &str = "Guinea";
    pub const ISO_LONG_NAME: &str = "The Republic of Guinea";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ff", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ff", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Guinean");
    pub const NUMBER: &str = "324";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAfrica);
    pub const UN_LOCODE: &str = "GN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Guinea", "Guinée", "ギニア", "Guinee"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Guinea"),
        ("af", "Guinee"),
        ("ak", "Guinea"),
        ("am", "ጊኔ"),
        ("an", "Guinea"),
        ("ar", "غينيا"),
        ("as", "গিনি"),
        ("ay", "Guinea"),
        ("az", "Qvineya"),
        ("ba", "Guinea"),
        ("be", "Гвінея"),
        ("bg", "Гвинея"),
        ("bi", "Guinea"),
        ("bn", "গিনি"),
        ("bn_IN", "গিনি"),
        ("br", "Ginea"),
        ("bs", "Gvineja"),
        ("ca", "Guinea"),
        ("ce", "Гвине"),
        ("ch", "Guinea"),
        ("cs", "Guinea"),
        ("cv", "Гвине"),
        ("cy", "Guinea"),
        ("da", "Guinea"),
        ("de", "Guinea"),
        ("dv", "ގ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "ག\u{f72}་ན\u{f72}།"),
        ("ee", "Guinea"),
        ("el", "Γουινέα"),
        ("en", "Guinea"),
        ("eo", "Gvineo"),
        ("es", "Guinea"),
        ("et", "Guinea"),
        ("eu", "Ginea"),
        ("fa", "گینه"),
        ("ff", "Gine"),
        ("fi", "Guinea"),
        ("fo", "Guinea"),
        ("fr", "Guinée"),
        ("fy", "Guinee"),
        ("ga", "An Ghuine"),
        ("gl", "Guinea"),
        ("gn", "Guinea"),
        ("gu", "ગિએના"),
        ("gv", "Yn Ghuinea"),
        ("ha", "Gine"),
        ("he", "גינאה"),
        ("hi", "गिनी"),
        ("hr", "Gvineja"),
        ("ht", "Gine"),
        ("hu", "Guinea"),
        ("hy", "Գվինեա"),
        ("ia", "Guinea"),
        ("id", "Guinea"),
        ("io", "Guinea"),
        ("is", "Gínea"),
        ("it", "Guinea"),
        ("iu", "Guinea"),
        ("ja", "ギニア"),
        ("ka", "გვინეა"),
        ("ki", "Guinea"),
        ("kk", "Гвинея"),
        ("kl", "Guinea"),
        ("km", "ហ\u{17d2}គ\u{17b8}ណេ"),
        ("kn", "ಗ\u{cbf}ನೀ"),
        ("ko", "기니"),
        ("ku", "Gîne"),
        ("kv", "Guinea"),
        ("kw", "Gyni"),
        ("ky", "Гвинея"),
        ("lo", "Guinea"),
        ("lt", "Gvinėja"),
        ("lv", "Gvineja"),
        ("mi", "Guinea"),
        ("mk", "Гвинеја"),
        ("ml", "ഗിനിയ"),
        ("mn", "Гвиней"),
        ("mr", "गिनी"),
        ("ms", "Guinea"),
        ("mt", "Gineja"),
        (
            "my",
            "ဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Gini"),
        ("nb", "Guinea"),
        ("ne", "जिनिया"),
        ("nl", "Guinee"),
        ("nn", "Guinea"),
        ("nv", "Guinea"),
        ("oc", "Guinèa"),
        ("or", "ଗ\u{b3f}ନୀ"),
        ("pa", "ਗ\u{a42}ਈਨੀਆ"),
        ("pi", "गिनी"),
        ("pl", "Gwinea"),
        ("ps", "ګیانا"),
        ("pt", "Guiné"),
        ("pt_BR", "Guiné"),
        ("ro", "Guinea"),
        ("ru", "Гвинея"),
        ("rw", "Gineya"),
        ("sc", "Guinea"),
        ("sd", "Guinea"),
        ("si", "ග\u{dd2}න\u{dd2}ය\u{dcf}ව"),
        ("sk", "Guinea"),
        ("sl", "Gvineja"),
        ("so", "Gini"),
        ("sq", "Guine"),
        ("sr", "Гвинеја"),
        ("sv", "Guinea"),
        ("sw", "Guinea"),
        ("ta", "கினி"),
        ("te", "గ\u{c3f}న\u{c40}"),
        ("tg", "Гвинея"),
        ("th", "ก\u{e34}น\u{e35}"),
        ("ti", "ጊኒ"),
        ("tk", "Gwineýa"),
        ("tl", "Guinea"),
        ("tr", "Gine"),
        ("tt", "Gуинеа"),
        ("ug", "گىۋىنېيە"),
        ("uk", "Гвінея"),
        ("ur", "جمہوریہ گنی"),
        ("uz", "Gvineya"),
        ("ve", "Guinea"),
        ("vi", "Ghi-nê"),
        ("wa", "Guinêye"),
        ("wo", "Ginne"),
        ("xh", "Guinea"),
        ("yo", "Guinea"),
        ("zh_CN", "几内亚"),
        ("zh_HK", "畿內亞"),
        ("zh_TW", "幾內亞"),
        ("zu", "IGini"),
    ];
    #[cfg(all(feature = "gn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 9.945587;
        pub const LONGITUDE: f64 = -9.696645;
        pub const MAX_LATITUDE: f64 = 12.6748616;
        pub const MAX_LONGITUDE: f64 = -7.637853;
        pub const MIN_LATITUDE: f64 = 7.190909099999999;
        pub const MIN_LONGITUDE: f64 = -15.282;
        pub const NORTHEAST_LATITUDE: f64 = 12.6748616;
        pub const NORTHEAST_LONGITUDE: f64 = -7.637853;
        pub const SOUTHWEST_LATITUDE: f64 = 7.190909099999999;
        pub const SOUTHWEST_LONGITUDE: f64 = -15.282;
    }
}
#[cfg(all(feature = "gn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 9.945587,
            longitude: -9.696645,
            max_latitude: 12.6748616,
            max_longitude: -7.637853,
            min_latitude: 7.190909099999999,
            min_longitude: -15.282,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 12.6748616,
                    longitude: -7.637853,
                },
                southwest: CountryGeoBound {
                    latitude: 7.190909099999999,
                    longitude: -15.282,
                },
            },
        }
    }
}

#[cfg(all(feature = "gn", feature = "subdivisions"))]
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
                    "B",
                    Subdivision{
                        name: "B",
                        country_alpha2: Alpha2::GN,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم بوكيه"), ("be", "Рэгіён Баке"), ("bg", "Боке"), ("bn", "ব\u{9c1}কে অঞ\u{9cd}চল"), ("ca", "Regió de Boké"), ("ccp", "𑄝\u{1112e}𑄇\u{11134} 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Boke Region"), ("da", "Boké Region"), ("de", "Region Boké"), ("el", "Μποκέ"), ("en", "Boké Region"), ("es", "Boké"), ("eu", "Bokéko eskualdea"), ("fi", "Bokén alue"), ("fr", "Région de Boké"), ("gu", "બોક પ\u{acd}રદ\u{ac7}શ"), ("hi", "बोक\u{947} क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Boké"), ("it", "regione di Boké"), ("ja", "ボケ州"), ("kn", "ಬೊಕ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "보케 주"), ("lt", "Bokės regionas"), ("lv", "Bokes reģions"), ("mr", "बोक\u{947} प\u{94d}रद\u{947}श"), ("ms", "Boke Region"), ("nb", "Boké"), ("nl", "Boké"), ("no", "Boké"), ("pl", "Boké"), ("pt", "Boké"), ("ro", "Regiunea Boké"), ("ru", "Боке"), ("si", "බොකේ කල\u{dcf}පය"), ("sv", "Boke Region"), ("ta", "போக\u{bcd}கே பகுதி"), ("te", "బ\u{c4b}క\u{c46} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e39}ก\u{e34}"), ("tr", "Boké Bölgesi"), ("uk", "Регіон Боке"), ("ur", "بوک ریجن"), ("vi", "Khu vực Boké"), ("zh", "博凱大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::GN,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.683333), longitude: Some(-8.633333), max_latitude: Some(8.7033838), min_latitude: Some(8.6760634), max_longitude: Some(-8.623409200000001), min_longitude: Some(-8.6646938)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بيلا"), ("bg", "Бейла"), ("bn", "বেল\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄝𑄬𑄣"), ("ceb", "Beyla Prefecture"), ("da", "Beyla Prefecture"), ("de", "Beyla (Präfektur)"), ("el", "Μπεϋλά"), ("en", "Beyla"), ("es", "Beyla"), ("fi", "Beylan prefektuuri"), ("fr", "préfecture de Beyla"), ("gu", "બ\u{ac7}યલા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "ब\u{947}यला प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Beyla"), ("it", "Prefettura di Beyla"), ("ja", "ベイラ県"), ("kn", "ಬೇಲಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "베일라 지방 행정 구역"), ("lt", "Beilos prefektūra"), ("lv", "Beilas prefektūra"), ("mr", "ब\u{947}यला प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Beyla Prefecture"), ("nb", "Beyla"), ("nl", "Beyla Prefecture"), ("no", "Beyla"), ("pl", "Prefektura Beyla"), ("pt", "Beyla"), ("ru", "Префектура Бейла"), ("si", "බෙය\u{dd2}ල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Beyla Prefecture"), ("ta", "பெய\u{bcd}ல ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "బ\u{c47}య\u{c4d}\u{200c}ల\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบย\u{e4c}ลา"), ("tr", "Beyla Prefecture"), ("uk", "Префектура Бейла"), ("ur", "بیئلا پریفیکچور"), ("vi", "Quận Beyla"), ("zh", "貝拉省")]),
                        unofficial_name_list: ["Beyla"].to_vec(),
                    }
                ),
                (
                    "BF",
                    Subdivision{
                        name: "BF",
                        country_alpha2: Alpha2::GN,
                        code: "BF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.1808254), longitude: Some(-14.0391615), max_latitude: Some(10.1955781), min_latitude: Some(10.1706568), max_longitude: Some(-14.0259361), min_longitude: Some(-14.050312)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بوفا"), ("bn", "বোফ\u{9cd}য\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄝\u{1112e}𑄜"), ("ceb", "Boffa (prepektura)"), ("da", "Boffa Prefecture"), ("de", "Boffa (Präfektur)"), ("el", "Μποφφά"), ("en", "Boffa"), ("es", "Ayuntamiento del Boffa"), ("fi", "Boffan prefektuuri"), ("fr", "préfecture de Boffa"), ("gu", "બોફ\u{ac7} પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "बोफ\u{93c}ा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Boffa"), ("it", "Prefettura di Boffa"), ("ja", "ボッファ県"), ("kn", "ಬೋಫಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "보파 지방 행정 구역"), ("lt", "Bofos prefektūra"), ("lv", "Bofas prefektūra"), ("mr", "बोफो प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Boffa Prefecture"), ("nb", "Boffa"), ("nl", "Boffa Prefecture"), ("no", "Boffa"), ("pl", "Prefektura Boffa"), ("pt", "Prefeitura de Boffa"), ("ru", "Префектура Боффа"), ("si", "බෝඵ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Boffa (prefektur)"), ("ta", "போப\u{bcd}ப\u{bbe}ய ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "బ\u{c4b}ఫ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e39}ฟฟา"), ("tr", "Boffa Prefecture"), ("uk", "Префектура Бофа"), ("ur", "بوفا پریفیکچور"), ("vi", "Quận Boffa"), ("zh", "博法省")]),
                        unofficial_name_list: ["Boffa"].to_vec(),
                    }
                ),
                (
                    "BK",
                    Subdivision{
                        name: "BK",
                        country_alpha2: Alpha2::GN,
                        code: "BK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.1864672), longitude: Some(-14.1001326), max_latitude: Some(12.67622), min_latitude: Some(9.810391599999999), max_longitude: Some(-12.555448), min_longitude: Some(-15.0782058)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بریفيكجور بوكه"), ("ccp", "𑄝\u{11127}𑄇\u{11134}"), ("ceb", "Boke Prefecture"), ("de", "Boké (Präfektur)"), ("el", "Μποκέ²"), ("en", "Boké"), ("es", "Boké²"), ("fr", "préfecture de Boké"), ("it", "Prefettura di Boké"), ("ja", "ボケ県"), ("nb", "Boké²"), ("nl", "Boké Prefecture"), ("no", "Boké²"), ("pl", "Prefektura Boké"), ("pt", "Boké²"), ("sv", "Boke Prefecture"), ("ur", "بوکے پریفیکچور"), ("zh", "博凱省")]),
                        unofficial_name_list: ["Boké"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::GN,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.509167), longitude: Some(-13.712222), max_latitude: Some(9.770441199999999), min_latitude: Some(9.5007226), max_longitude: Some(-13.4295724), min_longitude: Some(-13.7279041)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Conakry"), ("am", "ኮናክሪ"), ("ar", "كوناكري"), ("az", "Konakri"), ("be", "Горад Конакры"), ("bg", "Конакри"), ("bn", "কোন\u{9be}ক\u{9cd}রি"), ("bs", "Conakry"), ("ca", "Conakry"), ("ccp", "𑄇\u{11127}𑄚𑄇\u{11133}𑄢\u{11128}"), ("ceb", "Conakry Region"), ("cs", "Conakry"), ("cy", "Conakry"), ("da", "Conakry"), ("de", "Conakry"), ("el", "Κονακρί"), ("en", "Conakry"), ("es", "Conakri"), ("et", "Conakry"), ("eu", "Konakry"), ("fa", "کوناکری"), ("fi", "Conakry"), ("fr", "Conakry"), ("ga", "Conacraí"), ("gl", "Conakry"), ("gu", "કોનાક\u{acd}રી"), ("ha", "Conakry"), ("ha_NE", "Conakry"), ("he", "קונאקרי"), ("hi", "कोनाक\u{94d}री"), ("hr", "Conakry"), ("hu", "Conakry"), ("hy", "Կոնակրի"), ("id", "Conakry"), ("is", "Kónakrí"), ("it", "Conakry"), ("ja", "コナクリ"), ("jv", "Conakry"), ("ka", "კონაკრი"), ("kn", "ಕೊನಾಕ\u{ccd}ರ\u{cbf}"), ("ko", "코나크리"), ("ky", "Конакри"), ("lt", "Konakris"), ("lv", "Konakri"), ("mk", "Конакри"), ("ml", "കൊണ\u{d3e}ക\u{d4d}രി"), ("mn", "Конакри"), ("mr", "कोनाक\u{94d}री"), ("ms", "Conakry"), ("nb", "Conakry"), ("nl", "Conakry"), ("no", "Conakry"), ("pa", "ਕ\u{a4b}ਨਾਕਰੀ"), ("pl", "Konakry"), ("pt", "Conacri"), ("ro", "Conakry"), ("ru", "Конакри"), ("si", "කොනක\u{dca}\u{200d}ර\u{dd2}"), ("sk", "Konakry"), ("sl", "Conakry"), ("so", "Konakri"), ("sq", "Konakri"), ("sr", "Конакри"), ("sr_Latn", "Konakri"), ("sv", "Conakry"), ("sw", "Conakry"), ("ta", "கொன\u{bbe}க\u{bcd}ரி"), ("te", "క\u{c4b}న\u{c3e}క\u{c4d}ర\u{c40}"), ("th", "โคนาคร\u{e35}"), ("tk", "Konakri"), ("tr", "Konakri"), ("uk", "Конакрі"), ("ur", "کوناکری"), ("uz", "Konakri"), ("vi", "Conakry"), ("yo", "Conakry"), ("yo_BJ", "Conakry"), ("yue", "康納克立"), ("yue_Hans", "康纳克立"), ("zh", "科納克里")]),
                        unofficial_name_list: ["Conakry"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::GN,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.7), longitude: Some(-13.383333), max_latitude: Some(9.7186919), min_latitude: Some(9.692723899999999), max_longitude: Some(-13.3728929), min_longitude: Some(-13.4085811)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كويا"), ("bn", "কোয\u{9bc}\u{9be}হ প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄇\u{1112e}𑄠𑄦\u{11134}"), ("ceb", "Coyah"), ("da", "Coyah Prefecture"), ("de", "Coyah"), ("el", "Κογιά"), ("en", "Coyah"), ("es", "Coyah"), ("fi", "Coyahin prefektuuri"), ("fr", "préfecture de Coyah"), ("gu", "કોયાહ પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "कोयाह प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Coyah"), ("it", "Prefettura di Coyah"), ("ja", "コヤ県"), ("kn", "ಕೊಯಾಹ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "코야 지방 행정 구역"), ("lt", "Kojos Prefektūra"), ("lv", "Kojahas prefektūra"), ("mr", "कोयाह प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Coyah Prefecture"), ("nb", "Coyah"), ("nl", "Coyah Prefecture"), ("no", "Coyah"), ("pl", "Prefektura Coyah"), ("pt", "Coyah"), ("ru", "Префектура Койя"), ("si", "කොය\u{dcf}හ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Coyah (prefektur)"), ("ta", "கோயத\u{bcd} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "క\u{c4b}య\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "โคย\u{e48}า พร\u{e35}เพคเตอ"), ("tr", "Coyah Prefecture"), ("uk", "Префектура Койя"), ("ur", "کویاہ پریفیکچور"), ("vi", "Quận Coyah"), ("zh", "科亞省")]),
                        unofficial_name_list: ["Coyah"].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::GN,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كينديا"), ("be", "Рэгіён Кіндыя"), ("bg", "Киндия"), ("bn", "কিন\u{9cd}ডিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("ca", "Regió de Kindia"), ("ccp", "𑄇\u{11128}𑄚\u{11134}𑄓\u{11128}𑄠 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Kindia Region"), ("da", "Kindia Region"), ("de", "Region Kindia"), ("el", "Κιντιά"), ("en", "Kindia Region"), ("es", "Kindia"), ("eu", "Kindiako eskualdea"), ("fi", "Kindian alue"), ("fr", "Région de Kindia"), ("gu", "કિ\u{a82}ડિયા પ\u{acd}રદ\u{ac7}શ"), ("hi", "क\u{947}\u{902}डिया क\u{94d}ष\u{947}त\u{94d}र"), ("id", "Wilayah Kindia"), ("it", "regione di Kindia"), ("ja", "キンディア州"), ("kn", "ಕ\u{cbf}ಂಡ\u{cbf}ಯಾ ಪ\u{ccd}ರದೇಶ"), ("ko", "킨디아 주"), ("lt", "Kindijos regionas"), ("lv", "Kindijas reģions"), ("mr", "क\u{947}\u{902}डिया प\u{94d}रद\u{947}श"), ("ms", "Kindia Region"), ("nb", "Kindia"), ("nl", "Kindia"), ("no", "Kindia"), ("pl", "Kindia"), ("pt", "Kindia"), ("ro", "Regiunea Kindia"), ("ru", "Киндия"), ("si", "ක\u{dd2}න\u{dca}ඩ\u{dd2}ය\u{dcf} කල\u{dcf}පය"), ("sv", "Kindia Region"), ("ta", "கிண\u{bcd}டின\u{bbe} பகுதி"), ("te", "క\u{c3f}ండ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ค\u{e34}นเด\u{e35}ย"), ("tr", "Kindia Bölgesi"), ("uk", "Кіндія"), ("ur", "کیندیا علاقہ"), ("vi", "Khu vực Kindia"), ("zh", "金迪亞大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DB",
                    Subdivision{
                        name: "DB",
                        country_alpha2: Alpha2::GN,
                        code: "DB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.75), longitude: Some(-11.116667), max_latitude: Some(10.7523239), min_latitude: Some(10.7100748), max_longitude: Some(-11.0782528), min_longitude: Some(-11.142025)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دابولا"), ("bn", "দ\u{9be}বোল\u{9be}"), ("ca", "prefactura de Dabola"), ("ccp", "𑄓𑄝\u{1112e}𑄣"), ("ceb", "Dabola (prepektura)"), ("da", "Dabola Prefecture"), ("de", "Dabola (Präfektur)"), ("el", "Νταμπολά"), ("en", "Dabola"), ("es", "Ayuntamiento del Dabola"), ("fi", "Dabolan prefektuuri"), ("fr", "préfecture de Dabola"), ("gu", "ડાબોલા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "दबोला प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Dabola"), ("it", "Prefettura di Dabola"), ("ja", "ダボラ県"), ("kn", "ಡಬೋಲಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "다보라 지방 행정 구역"), ("lt", "Dabolos prefektūra"), ("lv", "Dabolas prefektūra"), ("mr", "दबोला प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Dabola Prefecture"), ("nb", "Dabola"), ("nl", "Dabola Prefecture"), ("no", "Dabola"), ("pl", "Prefektura Dabola"), ("pt", "Preeitura de Dabola"), ("ru", "Префектура Дабола"), ("si", "ඩබෝල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Dabola"), ("ta", "தபல\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "డ\u{c3e}బ\u{c4b}ల\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e39}เนโอ"), ("tr", "Dabola Prefecture"), ("uk", "Префектура Дабола"), ("ur", "دابولا پریفیکچور"), ("vi", "Tỉnh Dabola"), ("zh", "達波拉省")]),
                        unofficial_name_list: ["Dabola"].to_vec(),
                    }
                ),
                (
                    "DI",
                    Subdivision{
                        name: "DI",
                        country_alpha2: Alpha2::GN,
                        code: "DI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.3), longitude: Some(-10.716667), max_latitude: Some(11.3052249), min_latitude: Some(11.2771965), max_longitude: Some(-10.6942462), min_longitude: Some(-10.7283639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دينغويراي"), ("bn", "ডিঙ\u{9cd}গ\u{9c1}ইর\u{9be}য\u{9bc} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄓\u{11128}\u{11101}𑄉\u{1112d}\u{1112a}𑄢𑄠𑄬"), ("ceb", "Dinguiraye Prefecture"), ("da", "Dinguiraye Prefecture"), ("de", "Dinguiraye (Präfektur)"), ("el", "Ντινγιράιγ"), ("en", "Dinguiraye"), ("es", "Ayuntamiento del Dinguiraye"), ("fi", "Dinguirayen prefektuuri"), ("fr", "préfecture de Dinguiraye"), ("gu", "ડિ\u{a82}ગ\u{ac1}ઇરાય\u{ac7} પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "डि\u{902}ग\u{941}इर\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Dinguiraye"), ("it", "Prefettura di Dinguiraye"), ("ja", "ダンギレ県"), ("kn", "ಡ\u{cbf}ಂಗ\u{ccd}ಯುರಾಯ\u{cc6} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "딩기라예 지방 행정 구역"), ("lt", "Dingirajės prefektūra"), ("lv", "Dingirajes prefektūra"), ("mr", "दि\u{902}ग\u{941}इराय\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Dinguiraye Prefecture"), ("nb", "Dinguiraye"), ("nl", "Dinguiraye Prefecture"), ("no", "Dinguiraye"), ("pl", "Prefektura Dinguiraye"), ("pt", "Prefeitura de Dinguiraye"), ("ru", "Префектура Дингирайе"), ("si", "ද\u{dd2}න\u{dca}ග\u{dd4}ය\u{dd2}රයේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Dinguiraye Prefecture"), ("ta", "டிங\u{bcd}குகிற\u{bbe}யே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "డ\u{c3f}ంగ\u{c4d}వ\u{c48}ర\u{c47} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ด\u{e34}นก\u{e38}ยเรย\u{e4c} พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Ginguiraye Prefecture"), ("uk", "Префектура Дінгірае"), ("ur", "دنگویرائے پریفیکچور"), ("vi", "Quận Dinguiraye"), ("zh", "丁吉拉伊省")]),
                        unofficial_name_list: ["Dinguiraye"].to_vec(),
                    }
                ),
                (
                    "DL",
                    Subdivision{
                        name: "DL",
                        country_alpha2: Alpha2::GN,
                        code: "DL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.166667), longitude: Some(-9.383333), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دالابا"), ("bn", "ব\u{9be}ল\u{9be}ব\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄓𑄣𑄝"), ("ceb", "Dalaba (prepektura)"), ("da", "Dalaba Prefecture"), ("de", "Dalaba"), ("el", "Νταλαμπά"), ("en", "Dalaba"), ("es", "Ayuntamiento del Dalaba"), ("fi", "Dalaban prefektuuri"), ("fr", "préfecture de Dalaba"), ("gu", "દલાબા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "दलाबा प\u{94d}रान\u{94d}त"), ("id", "Prefektur Dalaba"), ("it", "Prefettura di Dalaba"), ("ja", "ダラバ県"), ("kn", "ದಲಾಬಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "달라바 지방 행정 구역"), ("lt", "Dalabos prefektūra"), ("lv", "Dalabas prefektūra"), ("mr", "डलबा प\u{94d}रान\u{94d}त"), ("ms", "Dalaba Prefecture"), ("nb", "Dalaba"), ("nl", "Dalaba Prefecture"), ("no", "Dalaba"), ("pl", "Prefektura Dalaba"), ("pt", "Dalaba"), ("ru", "Префектура Далаба"), ("si", "ඩලබ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Dalaba (prefektur)"), ("ta", "டலப\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "డ\u{c3e}ల\u{c3e}బ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ดาลาบา พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Dalaba Prefecture"), ("uk", "Префектура Далаба"), ("ur", "دالابا پریفیکچور"), ("vi", "Tỉnh Dalaba"), ("zh", "達拉巴省")]),
                        unofficial_name_list: ["Dalaba"].to_vec(),
                    }
                ),
                (
                    "DU",
                    Subdivision{
                        name: "DU",
                        country_alpha2: Alpha2::GN,
                        code: "DU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.783332999999999), longitude: Some(-13.516667), max_latitude: Some(9.807793799999999), min_latitude: Some(9.769929800000002), max_longitude: Some(-13.472023), min_longitude: Some(-13.5531284)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دوبريكا"), ("bn", "দেব\u{9cd}রেক\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄓\u{1112a}𑄝\u{11133}𑄢𑄬𑄇"), ("ceb", "Préfecture de Dubréka"), ("da", "Dubréka Prefecture"), ("de", "Dubréka (Präfektur)"), ("el", "Ντουμπρεκά"), ("en", "Dubréka"), ("es", "Dubréka"), ("fi", "Dubrékan prefektuuri"), ("fr", "préfecture de Dubréka"), ("gu", "ડ\u{ac1}બ\u{acd}ર\u{ac7}કા પ\u{acd}રિફ\u{ac7}ક\u{acd}ચર"), ("hi", "डब\u{94d}र\u{947}का प\u{94d}रान\u{94d}त"), ("id", "Prefektur Dubréka"), ("it", "Prefettura di Dubréka"), ("ja", "ドュブレカ県"), ("kn", "ಡುಬ\u{ccd}ರ\u{cc6}ಕಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "두브레카 지방 행정 구역"), ("lt", "Dubrekos prefektūra"), ("lv", "Dubrekas prefektūra"), ("mr", "डब\u{94d}र\u{947}का प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Dubreka Prefecture"), ("nb", "Dubréka"), ("nl", "Dubréka Prefecture"), ("no", "Dubréka"), ("pl", "Prefektura Dubréka"), ("pt", "Prefeitura de Dubreka"), ("ru", "Префектура Дубрека"), ("si", "ඩ\u{dd4}බ\u{dca}රෙක\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Préfecture de Dubréka"), ("ta", "டூப\u{bcd}ரேக\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "డుబ\u{c4d}ర\u{c46}క\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ด\u{e39}เบรก\u{e49}า พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Dubréka Prefecture"), ("uk", "Префектура Дубрека"), ("ur", "دوبریکا پریفیکچور"), ("vi", "Quận Dubréka"), ("zh", "杜布雷卡省")]),
                        unofficial_name_list: ["Dubréka"].to_vec(),
                    }
                ),
                (
                    "F",
                    Subdivision{
                        name: "F",
                        country_alpha2: Alpha2::GN,
                        code: "F",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Рэгіён Фарана"), ("bg", "Фарана"), ("ca", "Faranah"), ("ccp", "𑄜𑄢\u{11134}𑄚𑄦\u{11134} 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Faranah Region"), ("de", "Region Faranah"), ("el", "Φαρανά"), ("en", "Faranah Region"), ("es", "Faranah"), ("eu", "Faranahko eskualdea"), ("fi", "Faranahin alue"), ("fr", "Région de Faranah"), ("it", "regione di Faranah"), ("ja", "ファラナ州"), ("ko", "파라나 주"), ("lt", "Faranos regionas"), ("nb", "Faranah"), ("nl", "Faranah"), ("no", "Faranah"), ("pl", "Faranah"), ("pt", "Faranah"), ("ro", "Regiunea Faranah"), ("ru", "Фарана"), ("sv", "Faranah Region (region i Guinea)"), ("ur", "فاراناہ علاقہ"), ("zh", "法拉納大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "FA",
                    Subdivision{
                        name: "FA",
                        country_alpha2: Alpha2::GN,
                        code: "FA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.033333), longitude: Some(-10.733333), max_latitude: Some(10.0750513), min_latitude: Some(10.0104814), max_longitude: Some(-10.7218838), min_longitude: Some(-10.7830811)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄜𑄢𑄚𑄦\u{11134}"), ("ceb", "Faranah Prefecture"), ("de", "Faranah (Präfektur)"), ("el", "Φαρανά²"), ("en", "Faranah"), ("es", "Faranah²"), ("fr", "préfecture de Faranah"), ("it", "Prefettura di Faranah"), ("ja", "ファラナ県"), ("nb", "Faranah²"), ("nl", "Faranah Prefecture"), ("no", "Faranah²"), ("pl", "Prefektura Faranah"), ("sv", "Faranah Prefecture"), ("ur", "فاراناہ پریفیکچور"), ("zh", "法拉納省")]),
                        unofficial_name_list: ["Faranah"].to_vec(),
                    }
                ),
                (
                    "FO",
                    Subdivision{
                        name: "FO",
                        country_alpha2: Alpha2::GN,
                        code: "FO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.433333), longitude: Some(-13.083333), max_latitude: Some(9.444088899999999), min_latitude: Some(9.4211622), max_longitude: Some(-13.0725472), min_longitude: Some(-13.1032728)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄜\u{1112e}𑄢\u{11134}𑄇𑄬𑄢\u{11128}𑄠𑄦\u{11134}"), ("ceb", "Préfecture de Forécariah"), ("de", "Forécariah (Präfektur)"), ("el", "Φορεκαριά"), ("en", "Forécariah"), ("es", "Forécariah"), ("fr", "préfecture de Forécariah"), ("it", "Prefettura di Forécariah"), ("ja", "フォレカリア県"), ("nb", "Forécariah"), ("nl", "Forécariah Prefecture"), ("no", "Forécariah"), ("pl", "Prefektura Forécariah"), ("sv", "Préfecture de Forécariah"), ("ur", "فوریکاریاہ پریفیکچور"), ("zh", "福雷卡里亞省")]),
                        unofficial_name_list: ["Forécaria"].to_vec(),
                    }
                ),
                (
                    "FR",
                    Subdivision{
                        name: "FR",
                        country_alpha2: Alpha2::GN,
                        code: "FR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.3674543), longitude: Some(-13.5841871), max_latitude: Some(10.3925134), min_latitude: Some(10.3439033), max_longitude: Some(-13.5586739), min_longitude: Some(-13.5969328)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فريا"), ("bn", "ফ\u{9cd}রিয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄜\u{11133}𑄢\u{1112d}𑄠"), ("ceb", "Fria"), ("da", "Fria Prefecture"), ("de", "Fria (Präfektur)"), ("el", "Φριά"), ("en", "Fria"), ("es", "Ayuntamiento Veszprém"), ("fi", "Frian prefektuuri"), ("fr", "préfecture de Fria"), ("gu", "ફ\u{acd}રિયા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "फ\u{94d}रिया प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Fria"), ("it", "Prefettura di Fria"), ("ja", "フリア県"), ("kn", "ಪ\u{ccd}ಹ\u{ccd}ರ\u{cbf}ಯಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "프리아 지방 행정 구역"), ("lt", "Frijos prefektūra"), ("lv", "Fria prefektūra"), ("mr", "फ\u{94d}रीआ प\u{94d}रिफ\u{947}क\u{94d}चर"), ("ms", "Fria Prefecture"), ("nb", "Fria"), ("nl", "Fria Prefecture"), ("no", "Fria"), ("pl", "Prefektura Fria"), ("pt", "Fria"), ("ru", "Префектура Фрия"), ("si", "ෆ\u{dca}\u{200d}ර\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Fria"), ("ta", "பிரிய\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ఫ\u{c4d}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "เพร\u{e35}ย พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Fria Prefecture"), ("uk", "Префектура Фріа"), ("ur", "فریا پریفیکچور"), ("vi", "Quận Fria"), ("zh", "弗里亞省")]),
                        unofficial_name_list: ["Fria"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::GN,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.75), longitude: Some(-13.2), max_latitude: Some(11.7684275), min_latitude: Some(11.7405713), max_longitude: Some(-13.1911039), min_longitude: Some(-13.2174969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة غاوال"), ("bn", "গ\u{9be}উল প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄉𑄅\u{1112a}𑄠𑄣\u{11134}"), ("ceb", "Gaoual Prefecture"), ("da", "Gaoual Prefecture"), ("de", "Gaoual"), ("el", "Γκαουάλ"), ("en", "Gaoual"), ("es", "Ayuntamiento Gaoual"), ("fi", "Gaoualin prefektuuri"), ("fr", "préfecture de Gaoual"), ("gu", "ગાઓઉલ પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "गाओल प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Gaoual"), ("it", "Prefettura di Gaoual"), ("ja", "ガワル県"), ("kn", "ಗ\u{ccc}\u{ccc}ಲ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "가우알 지방 행정 구역"), ("lt", "Gauvalo prefektūra"), ("lv", "Gavalas prefektūra"), ("mr", "गाओल प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Gaoual Prefecture"), ("nb", "Gaoual"), ("nl", "Gaoual Prefecture"), ("no", "Gaoual"), ("pl", "Prefektura Gaoual"), ("pt", "Gaoual"), ("ru", "Префектура Гауаль"), ("si", "ගොඋඅල\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Gaoual Prefecture"), ("ta", "க\u{bbe}யல\u{bcd} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "గ\u{c3e}వువల\u{c4d} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "เกาอล พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Gaoual Prefecture"), ("uk", "Префектура Гауаль"), ("ur", "گاؤال پریفیکچور"), ("vi", "Quận Gaoual"), ("zh", "加瓦爾省")]),
                        unofficial_name_list: ["Gaoual"].to_vec(),
                    }
                ),
                (
                    "GU",
                    Subdivision{
                        name: "GU",
                        country_alpha2: Alpha2::GN,
                        code: "GU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.566666999999999), longitude: Some(-10.133333), max_latitude: Some(8.597061199999999), min_latitude: Some(8.5333639), max_longitude: Some(-10.094719), min_longitude: Some(-10.1548433)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة غوكيدو"), ("bg", "Гекеду"), ("bn", "গ\u{9c1}য\u{9bc}েকেদ\u{9c1} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄉\u{1112a}𑄇𑄬𑄓\u{1112f}"), ("ceb", "Préfecture de Guékédou"), ("da", "Guéckédou Prefecture"), ("de", "Guéckédou"), ("el", "Γκουεκεντού"), ("en", "Guéckédou"), ("es", "Guéckédou"), ("fi", "Guéckédoun prefektuuri"), ("fr", "préfecture de Guéckédou"), ("gu", "ગ\u{acd}ય\u{ac1}એક\u{ac7}ડોઉ પ\u{acd}રિફ\u{ac7}ક\u{acd}ચર"), ("hi", "ग\u{94d}व\u{947}क\u{947}ड\u{942} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Guéckédou"), ("it", "Prefettura di Guéckédou"), ("ja", "ゲケドゥ県"), ("kn", "ಗುಕ\u{cc6}ಡ\u{cc6}ವ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "게케두 지방 행정 구역"), ("lt", "Gekedo prefektūra"), ("lv", "Gekedu prefektūra"), ("mr", "ग\u{941}एकदौ प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Gueckedou Prefecture"), ("nb", "Guéckédou"), ("nl", "Guéckédou Prefecture"), ("no", "Guéckédou"), ("pl", "Prefektura Guéckédou"), ("pt", "Prefeitura de Guéckédou"), ("ru", "Префектура Гекеду"), ("si", "ග\u{dd4}වෙකෙඩොව\u{dd6} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Préfecture de Guékédou"), ("ta", "குய\u{bcd}க\u{bcd}கெடோடு ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "గుయ\u{c46}క\u{c4d}\u{200c}డ\u{c4d}యూ"), ("th", "อำเภอก\u{e38}คเคโดว"), ("tr", "Guéckéedou Prefecture"), ("uk", "Гекеду"), ("ur", "گویکیدؤ پریفیکچور"), ("vi", "Quận Guéckédou"), ("zh", "蓋凱杜省")]),
                        unofficial_name_list: ["Guékédou"].to_vec(),
                    }
                ),
                (
                    "K",
                    Subdivision{
                        name: "K",
                        country_alpha2: Alpha2::GN,
                        code: "K",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم كانكان"), ("be", "Рэгіён Канкан"), ("bg", "Канкан"), ("bn", "ক\u{9be}নক\u{9be}ন অঞ\u{9cd}চল"), ("ccp", "𑄇𑄚\u{11134}𑄇𑄚\u{11134} 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Kankan Region"), ("da", "Kankan Region"), ("de", "Region Kankan"), ("el", "Κανκάν"), ("en", "Kankan Region"), ("es", "Kankan"), ("eu", "Kankango eskualdea"), ("fi", "Kankanin alue"), ("fr", "Région de Kankan"), ("gu", "કા\u{a82}કન પ\u{acd}રદ\u{ac7}શ"), ("hi", "का\u{902}कन प\u{94d}रद\u{947}श"), ("id", "Wilayah Kankan"), ("it", "regione di Kankan"), ("ja", "カンカン州"), ("kn", "ಕಂಕಾನ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "캉칸 주"), ("lt", "Kankano regionas"), ("lv", "Kankanas reģions"), ("mr", "काकण प\u{94d}रद\u{947}श"), ("ms", "Kankan Region"), ("nb", "Kankan"), ("nl", "Kankan"), ("no", "Kankan"), ("pl", "Kankan"), ("pt", "Kankan"), ("ro", "Regiunea Kankan"), ("ru", "Канкан"), ("si", "කන\u{dca}කන\u{dca} කල\u{dcf}පය"), ("sv", "Kankan Region"), ("ta", "கங\u{bcd}கன\u{bcd} பகுதி"), ("te", "కంకన\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "กานกาน"), ("tr", "Kankan Bölgesi"), ("uk", "Канкан"), ("ur", "کانکان علاقہ"), ("vi", "Khu vực Kankan"), ("zh", "康康大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "KA",
                        country_alpha2: Alpha2::GN,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.120923), longitude: Some(-9.5450974), max_latitude: Some(12.5026249), min_latitude: Some(8.7616421), max_longitude: Some(-7.947203899999999), min_longitude: Some(-10.90847)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇𑄚\u{11134}𑄇𑄚\u{11134}"), ("ceb", "Kankan Prefecture"), ("de", "Kankan"), ("el", "Κανκάν²"), ("en", "Kankan"), ("es", "Kankan²"), ("fr", "préfecture de Kankan"), ("it", "Prefettura di Kankan"), ("ja", "カンカン県"), ("nb", "Kankan²"), ("nl", "Kankan Prefecture"), ("no", "Kankan²"), ("pl", "Prefektura Kankan"), ("sv", "Kankan Prefecture"), ("ur", "کانکان پریفیکچور"), ("zh", "康康省")]),
                        unofficial_name_list: ["Kankan"].to_vec(),
                    }
                ),
                (
                    "KB",
                    Subdivision{
                        name: "KB",
                        country_alpha2: Alpha2::GN,
                        code: "KB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.583333), longitude: Some(-11.9), max_latitude: Some(11.5942277), min_latitude: Some(11.5748048), max_longitude: Some(-11.8801688), min_longitude: Some(-11.9058323)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كوبيا"), ("bn", "ক\u{9c1}বিয\u{9bc}\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄇\u{1112f}𑄝\u{11128}𑄠"), ("ceb", "Koubia"), ("da", "Koubia Prefecture"), ("de", "Koubia"), ("el", "Κουμπιά"), ("en", "Koubia"), ("es", "Ayuntamiento Koubia"), ("fi", "Koubian prefektuuri"), ("fr", "préfecture de Koubia"), ("gu", "રિકિયવિક"), ("hi", "क\u{942}बिया प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Koubia"), ("it", "Prefettura di Koubia"), ("ja", "クビア県"), ("kn", "ಕ\u{ccd}ಯುಬ\u{cbf}ಯಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "쿠비아 지방 행정 구역"), ("lt", "Kubijos prefektūra"), ("lv", "Kubijas prefektūra"), ("mr", "कौबिया प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Koubia Prefecture"), ("nb", "Koubia"), ("nl", "Koubia Prefecture"), ("no", "Koubia"), ("pl", "Prefektura Koubia"), ("pt", "Koubia"), ("ru", "Префектура Кубия"), ("si", "කොඋබ\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Koubia (prefektur)"), ("ta", "கௌப\u{bc0}ஆ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "క\u{c4c}బ\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ค\u{e31}วเบ\u{e35}ย พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Koubia Prefectura"), ("uk", "Префектура Кубія"), ("ur", "کؤبیا پریفیکچور"), ("vi", "Tỉnh Koubia"), ("zh", "庫比亞省")]),
                        unofficial_name_list: ["Koubia"].to_vec(),
                    }
                ),
                (
                    "KD",
                    Subdivision{
                        name: "KD",
                        country_alpha2: Alpha2::GN,
                        code: "KD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.066667), longitude: Some(-12.85), max_latitude: Some(10.0824454), min_latitude: Some(10.0121295), max_longitude: Some(-12.8158951), min_longitude: Some(-12.8973483)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{11128}𑄚\u{11134}𑄓\u{11128}𑄠"), ("de", "Kindia (Präfektur)"), ("el", "Κιντιά²"), ("en", "Kindia"), ("es", "Kindia²"), ("fr", "préfecture de Kindia"), ("it", "Prefettura di Kindia"), ("ja", "キンディア県"), ("nb", "Kindia²"), ("nl", "Kindia Prefecture"), ("no", "Kindia²"), ("pl", "Prefektura Kindia"), ("pt", "Quindia (prefeitura)"), ("ur", "کیندیا پریفیکچور"), ("zh", "金迪亞省")]),
                        unofficial_name_list: ["Kindia"].to_vec(),
                    }
                ),
                (
                    "KE",
                    Subdivision{
                        name: "KE",
                        country_alpha2: Alpha2::GN,
                        code: "KE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.266667), longitude: Some(-9.016667), max_latitude: Some(9.282653), min_latitude: Some(9.2568165), max_longitude: Some(-8.9937686), min_longitude: Some(-9.023037)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كيرونيه"), ("bn", "কের\u{9c1}য\u{9bc}\u{9be}নে প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄇𑄬𑄢\u{1112f}𑄠𑄚\u{11128}"), ("ceb", "Kerouane Prefecture"), ("da", "Kérouané Prefecture"), ("de", "Kérouané"), ("el", "Κερουανέ"), ("en", "Kérouané"), ("es", "Kérouané"), ("fi", "Kérouanén prefektuuri"), ("fr", "préfecture de Kérouane"), ("gu", "ક\u{ac7}રોઉન પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "क\u{947}रोवा\u{902} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Kérouané"), ("it", "Prefettura di Kérouané"), ("ja", "ケルワヌ県"), ("kn", "ಕ\u{cc6}ರೋನ\u{cc6} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "케로우아네 지방 행정 구역"), ("lt", "Kervano prefektūra"), ("lv", "Keruanes prefektūra"), ("mr", "क\u{947}रोवा\u{902} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Kerouane Prefecture"), ("nb", "Kérouané"), ("nl", "Kérouané Prefecture"), ("no", "Kérouané"), ("pl", "Prefektura Kérouané"), ("pt", "Kérouané"), ("ru", "Префектура Керуане"), ("si", "කෙරෝඅනේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Kerouane Prefecture"), ("ta", "கேரோவனே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "క\u{c46}ర\u{c4b}వ\u{c47}న\u{c4d} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ค\u{e35}ร\u{e31}วเน\u{e48} พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Kérouane Prefecture"), ("uk", "Префектура Керуане"), ("ur", "کیرؤانے پریفیکچور"), ("vi", "Quận Kérouané"), ("zh", "凱魯阿內省")]),
                        unofficial_name_list: ["Kérouané"].to_vec(),
                    }
                ),
                (
                    "KN",
                    Subdivision{
                        name: "KN",
                        country_alpha2: Alpha2::GN,
                        code: "KN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.483333), longitude: Some(-13.3), max_latitude: Some(12.5115815), min_latitude: Some(12.4696819), max_longitude: Some(-13.2766341), min_longitude: Some(-13.3191205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كاوندارا"), ("bn", "ক\u{9c1}ন\u{9cd}ড\u{9be}র\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄇\u{1112f}𑄚\u{11134}𑄓𑄢"), ("ceb", "Koundara Prefecture"), ("da", "Koundara Prefecture"), ("de", "Koundara"), ("el", "Κουνταρά"), ("en", "Koundara"), ("es", "Ayuntamiento Koundara"), ("fi", "Koundaran prefektuuri"), ("fr", "préfecture de Koundara"), ("gu", "ખોવ\u{acd}દ"), ("hi", "कौन\u{94d}डारा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Koundara"), ("it", "Prefettura di Koundara"), ("ja", "クンダラ県"), ("kn", "ಕ\u{ccc}ಂಡಾರಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "쿤다라 지방 행정 구역"), ("lt", "Koundaros prefektūra"), ("lv", "Kundaras prefektūra"), ("mr", "को\u{902}डारा प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Koundara Prefecture"), ("nb", "Koundara"), ("nl", "Koundara Prefecture"), ("no", "Koundara"), ("pl", "Prefektura Koundara"), ("pt", "Koundara"), ("ru", "Префектюр-де-Кундара"), ("si", "කව\u{dd4}න\u{dca}ඩර\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Koundara Prefecture"), ("ta", "கௌண\u{bcd}டர\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}டர\u{bcd}"), ("te", "క\u{c4c}ండ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ควนดารา พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Koundrata Prefecture"), ("uk", "Префектура Кундара"), ("ur", "کؤندارا پریفیکچور"), ("vi", "Tỉnh Koundara"), ("zh", "孔達拉省")]),
                        unofficial_name_list: ["Koundara"].to_vec(),
                    }
                ),
                (
                    "KO",
                    Subdivision{
                        name: "KO",
                        country_alpha2: Alpha2::GN,
                        code: "KO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.65), longitude: Some(-9.883333), max_latitude: Some(10.6671028), min_latitude: Some(10.6314216), max_longitude: Some(-9.866494999999999), min_longitude: Some(-9.9002372)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوروسا"), ("bn", "করৌস\u{9cd}য\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄇\u{1112f}𑄢\u{1112f}𑄥\u{11133}𑄦"), ("ceb", "Kouroussa"), ("da", "Kouroussa Prefecture"), ("de", "Kouroussa"), ("el", "Κουρουσσά"), ("en", "Kouroussa"), ("es", "Kouroussa"), ("fi", "Kouroussan prefektuuri"), ("fr", "préfecture de Kouroussa"), ("gu", "કૌરૌસા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "कौर\u{941}सा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Kouroussa"), ("it", "Prefettura di Kouroussa"), ("ja", "クールーサ県"), ("kn", "ಕ\u{ccc}ರ\u{ccc}ಸ\u{ccd}ಸಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "쿠루사 지방 행정 구역"), ("lt", "Kurusos prefektūra"), ("lv", "Kuruso prefektūra"), ("mr", "क\u{941}रौसा प\u{94d}रिफ\u{947}क\u{94d}च\u{941}अर"), ("ms", "Kouroussa Prefecture"), ("nb", "Kouroussa"), ("nl", "Kouroussa Prefecture"), ("no", "Kouroussa"), ("pl", "Prefektura Kouroussa"), ("pt", "Kouroussa"), ("ru", "Префектура Курусса"), ("si", "ක\u{dd4}ර\u{dd4}ස\u{dca}ස\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Kouroussa"), ("ta", "கௌரியஸ\u{bcd}ஸ\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "క\u{c4a}ర\u{c4c}స\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "เคาเราส\u{e4c}ซา"), ("tr", "Kouroussa Prefecture"), ("uk", "Префектура Куруса"), ("ur", "کؤرؤسا پریفیکچور"), ("vi", "Tỉnh Kouroussa"), ("zh", "庫魯薩省")]),
                        unofficial_name_list: ["Kouroussa"].to_vec(),
                    }
                ),
                (
                    "KS",
                    Subdivision{
                        name: "KS",
                        country_alpha2: Alpha2::GN,
                        code: "KS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.183333), longitude: Some(-10.1), max_latitude: Some(9.234620999999999), min_latitude: Some(9.1458251), max_longitude: Some(-10.0576401), min_longitude: Some(-10.1441575)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كيسيدوغو"), ("bn", "কিস\u{9cd}যিদোগ\u{9c1} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "prefectura de Kissidougou"), ("ccp", "𑄇\u{11128}𑄥\u{11128}𑄓\u{1112f}𑄉\u{1112f}"), ("ceb", "Kissidougou (prepektura)"), ("da", "Kissidougou Prefecture"), ("de", "Kissidougou"), ("el", "Κισσιντουγκού"), ("en", "Kissidougou"), ("es", "Kissidougou"), ("fi", "Kissidougoun prefektuuri"), ("fr", "préfecture de Kissidougou"), ("gu", "કિસિડૌગૌ પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "किसिड\u{942}ग\u{942} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Kissidougou"), ("it", "Prefettura di Kissidougou"), ("ja", "キシドゥグ県"), ("kn", "ಕ\u{cbf}ಸ\u{ccd}ಡ\u{ccc}ಗ\u{ccd}ಗ\u{ccc} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "키시두구 지방 행정 구역"), ("lt", "Kisidugo prefektūra"), ("lv", "Kisidugo prefektūra"), ("mr", "किस\u{94d}सीडॉगौ प\u{94d}रीफ\u{947}क\u{94d}च\u{941}अर"), ("ms", "Kissidougou Prefecture"), ("nb", "Kissidougou"), ("nl", "Kissidougou Prefecture"), ("no", "Kissidougou"), ("pl", "Prefektura Kissidougou"), ("pt", "Kissidougou"), ("ru", "Киссидугу"), ("si", "ක\u{dd2}ස\u{dca}ස\u{dd2}ඩොඋග\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Kissidougou"), ("ta", "கிஸ\u{bcd}ஸிடௌகௌ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "క\u{c3f}స\u{c3f}డ\u{c3e}గ\u{c4b} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ค\u{e34}สส\u{e34}โดวก\u{e39}ว พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Kissidougou Prefecture"), ("uk", "Префектура Кісідугу"), ("ur", "کیسیدؤگؤ پریفیکچور"), ("vi", "Tỉnh Kissidougou"), ("zh", "基西杜古省")]),
                        unofficial_name_list: ["Kissidougou"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::GN,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Рэгіён Лабэ"), ("bg", "Лабе"), ("ccp", "𑄣𑄝𑄬 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Labé Region"), ("de", "Region Labé"), ("el", "Λαμπέ"), ("en", "Labé Region"), ("es", "Labé"), ("eu", "Labéko eskualdea"), ("fi", "Labén alue"), ("fr", "Région de Labé"), ("it", "regione di Labé"), ("ja", "ラベ州"), ("ko", "라베 주"), ("lt", "Labės regionas"), ("nb", "Labé"), ("nl", "Labé"), ("no", "Labé"), ("pl", "Labé"), ("pt", "Labé"), ("ro", "Regiunea Labé"), ("ru", "Лабе"), ("sv", "Labé Region"), ("uk", "Лабе²"), ("ur", "لابی ریجن"), ("zh", "拉貝大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::GN,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.641159), longitude: Some(-11.8891721), max_latitude: Some(12.4428371), min_latitude: Some(11.0049169), max_longitude: Some(-11.109311), min_longitude: Some(-13.027551)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لابي"), ("bn", "ল\u{9be}বি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄣\u{11127}𑄝𑄬"), ("ceb", "Labe Prefecture"), ("da", "Labé Prefecture"), ("de", "Labé"), ("el", "Λαμπέ²"), ("en", "Labé"), ("es", "Labé²"), ("fi", "Labén prefektuuri"), ("fr", "préfecture de Labé"), ("gu", "લાબ\u{ac7} પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "लाब\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Labé"), ("it", "Prefettura di Labé"), ("ja", "ラベ県"), ("kn", "ಲ\u{ccd}ಯಾಬ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "라베 지방 행정 구역"), ("lt", "Labės prefektūra"), ("lv", "Labes prefektūra"), ("mr", "ल\u{945}ब\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Labe Prefecture"), ("nb", "Labé²"), ("nl", "Labé Prefecture"), ("no", "Labé²"), ("pl", "Prefektura Labé"), ("pt", "Labé²"), ("ru", "Префектура Лабе"), ("si", "ලබේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Labe Prefecture"), ("ta", "லபே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ల\u{c3e}బ\u{c46} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ลาเบ พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Labé Prefecture"), ("uk", "Лабе"), ("ur", "لابی پریفیکچر"), ("vi", "Quận Labé"), ("zh", "拉貝省")]),
                        unofficial_name_list: ["Labé"].to_vec(),
                    }
                ),
                (
                    "LE",
                    Subdivision{
                        name: "LE",
                        country_alpha2: Alpha2::GN,
                        code: "LE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.4739967), longitude: Some(-12.6675575), max_latitude: Some(11.831968), min_latitude: Some(11.102719), max_longitude: Some(-12.424321), min_longitude: Some(-13.027551)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ليلوما"), ("bn", "লিল\u{9c1}ম\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄣𑄬𑄣\u{1112f}𑄟"), ("ceb", "Lelouma Prefecture"), ("da", "Lélouma Prefecture"), ("de", "Lélouma"), ("el", "Λελουμά"), ("en", "Lélouma"), ("es", "Ayuntamiento del Lélouma"), ("fi", "Lélouman prefektuuri"), ("fr", "préfecture de Lélouma"), ("gu", "લ\u{ac7}લૌમા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "ल\u{947}लौमा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Lélouma"), ("it", "Prefettura di Lélouma"), ("ja", "レルーマ県"), ("kn", "ಲ\u{cc6}ಲೊಮಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "렐루마 지방 행정 구역"), ("lt", "Lelumos prefektūra"), ("lv", "Lelumas prefektūra"), ("mr", "ल\u{947}म\u{941}मा प\u{94d}रिफ\u{947}क\u{94d}च\u{941}अर"), ("ms", "Lelouma Prefecture"), ("nb", "Lélouma"), ("nl", "Lélouma Prefecture"), ("no", "Lélouma"), ("pl", "Prefektura Lélouma"), ("pt", "Prefeitura de Lélouma"), ("ru", "Префектура Лелума"), ("si", "ලේලොඋම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Lelouma Prefecture"), ("ta", "லெலூம\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ల\u{c46}ల\u{c4b}మ\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "เลล\u{e31}วมา พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Lélouma Prefecure"), ("uk", "Префектура Лелоума"), ("ur", "لیلؤما پریفیکچور"), ("vi", "Tỉnh Lélouma"), ("zh", "萊盧馬省")]),
                        unofficial_name_list: ["Lélouma"].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::GN,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.8), longitude: Some(-8.533332999999999), max_latitude: Some(7.8204807), min_latitude: Some(7.784528399999999), max_longitude: Some(-8.5110372), min_longitude: Some(-8.5487326)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية لولا"), ("bn", "লোল\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄣\u{11127}𑄣"), ("ceb", "Lola (prepektura)"), ("da", "Lola Prefecture"), ("de", "Lola"), ("el", "Λολά"), ("en", "Lola"), ("es", "Lola"), ("fi", "Lolan prefektuuri"), ("fr", "préfecture de Lola"), ("gu", "લોલા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "लोला प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Lola"), ("it", "Prefettura di Lola"), ("ja", "ローラ県"), ("kn", "ಲೋಲಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "롤라 지방 행정 구역"), ("lt", "Lolos prefektūra"), ("lv", "Lolas prefektūra"), ("mr", "लोला प\u{94d}रिफ\u{947}क\u{94d}चर"), ("ms", "Lola Prefecture"), ("nb", "Lola"), ("nl", "Lola Prefecture"), ("no", "Lola"), ("pl", "Prefektura Lola"), ("pt", "Lola"), ("ru", "Префектура Лола"), ("si", "ලොල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Lola (prefektur)"), ("ta", "லோல\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ల\u{c4b}ల\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "โลล\u{e48}า พน\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Lola Prefecture"), ("uk", "Префектура Лола"), ("ur", "لولا پریفیکچور"), ("vi", "Quận Lola"), ("zh", "洛拉省")]),
                        unofficial_name_list: ["Lola"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::GN,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مامو"), ("be", "Рэгіён Маму"), ("bg", "Маму"), ("bn", "ম\u{9be}ম\u{9c1} অঞ\u{9cd}চল"), ("ccp", "𑄟𑄟\u{1112f} 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Mamou Region"), ("da", "Mamou Region"), ("de", "Region Mamou"), ("el", "Μαμού"), ("en", "Mamou Region"), ("es", "Mamou"), ("eu", "Mamouko eskualdea"), ("fi", "Mamoun alue"), ("fr", "Région de Mamou"), ("gu", "મામો પ\u{acd}રદ\u{ac7}શ"), ("hi", "म\u{948}म\u{942} प\u{94d}रा\u{902}त"), ("id", "Wilayah Mamou"), ("it", "regione di Mamou"), ("ja", "マムー州"), ("kn", "ಮಾಮ\u{ccc} ಪ\u{ccd}ರದೇಶ"), ("ko", "마무 주"), ("lt", "Mamu regionas"), ("lv", "Mamū reģions"), ("mr", "माम\u{941} प\u{94d}रद\u{947}श"), ("ms", "Daerah Mamou"), ("nb", "Mamou"), ("nl", "Mamou"), ("no", "Mamou"), ("pl", "Mamou"), ("pt", "Mamou"), ("ro", "Regiunea Mamou"), ("ru", "Маму"), ("si", "මමෞ කල\u{dcf}පය"), ("sv", "Mamou Region"), ("ta", "மமோஉ பகுதி"), ("te", "మ\u{c3e}మ\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตมาม\u{e31}ว"), ("tr", "Mamou Bölgesi"), ("uk", "Ругіон Маму"), ("ur", "مامؤ علاقہ"), ("vi", "Khu vực Mamou"), ("zh", "馬木大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::GN,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.549999999999999), longitude: Some(-9.466667), max_latitude: Some(8.563367999999999), min_latitude: Some(8.516344799999999), max_longitude: Some(-9.4360541), min_longitude: Some(-9.496479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ماسينتا"), ("bn", "ম\u{9cd}য\u{9be}সেন\u{9cd}ট\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄟𑄥𑄬𑄚\u{11134}𑄑"), ("ceb", "Macenta (prepektura)"), ("da", "Macenta Prefecture"), ("de", "Macenta"), ("el", "Μασεντά"), ("en", "Macenta"), ("es", "Macenta"), ("fi", "Macentan prefektuuri"), ("fr", "préfecture de Macenta"), ("gu", "મક\u{ac7}ન\u{acd}ટા પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("hi", "म\u{948}स\u{947}न\u{94d}टा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Macenta"), ("it", "Prefettura di Macenta"), ("ja", "マサンタ県"), ("kn", "ಮ\u{ccd}ಯಾಕ\u{cc6}ಂತಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "마센타 지방 행정 구역"), ("lt", "Macentos prefektūra"), ("lv", "Masentas prefektūra"), ("mr", "म\u{945}न\u{94d}टा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Macenta Prefecture"), ("nb", "Macenta"), ("nl", "Macenta Prefecture"), ("no", "Macenta"), ("pl", "Prefektura Macenta"), ("pt", "Macenta"), ("ru", "Префектура Масента"), ("si", "මසේන\u{dca}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Macenta (prefektur)"), ("ta", "மசெண\u{bcd}ட ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "మ\u{c3e}స\u{c46}ంట\u{c3e} ప\u{c4d}ర\u{c3f}ప\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาเซนตา"), ("tr", "Macenta Prefecture"), ("uk", "Масента"), ("ur", "میکینٹا پریفیکٹوری"), ("vi", "Quận Macenta"), ("zh", "馬桑塔省")]),
                        unofficial_name_list: ["Macenta"].to_vec(),
                    }
                ),
                (
                    "MD",
                    Subdivision{
                        name: "MD",
                        country_alpha2: Alpha2::GN,
                        code: "MD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.6172827), longitude: Some(-8.6985716), max_latitude: Some(10.640026), min_latitude: Some(10.6024858), max_longitude: Some(-8.6830616), min_longitude: Some(-8.7058068)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مانديانا"), ("bn", "ম\u{9be}ন\u{9cd}দিয\u{9bc}\u{9be}ন\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄟\u{11133}𑄠𑄚\u{11134}𑄓\u{11128}𑄠𑄚"), ("ceb", "Mandiana Prefecture"), ("da", "Mandiana Prefecture"), ("de", "Mandiana"), ("el", "Μαντιανά"), ("en", "Mandiana"), ("es", "Ayuntamiento Mandiana"), ("fi", "Mandianan prefektuuri"), ("fr", "préfecture de Mandiana"), ("gu", "મ\u{ac5}ન\u{acd}ડિયાના પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "म\u{948}न\u{94d}दियाना प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Mandiana"), ("it", "Prefettura di Mandiana"), ("ja", "マンディアナ県"), ("kn", "ಮಾಂಡ\u{cbf}ಯಾನ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "만디아나 지방 행정 구역"), ("lt", "Mandijanos prefektūra"), ("lv", "Mandianas prefektūra"), ("mr", "म\u{947}डियाना परफ\u{947}क\u{94d}च\u{941}अर"), ("ms", "Mandiana Prefecture"), ("nb", "Mandiana"), ("nl", "Mandiana Prefecture"), ("no", "Mandiana"), ("pl", "Prefektura Mandiana"), ("pt", "Prefeitura de Mandiana"), ("ru", "Префектура Мандиана"), ("si", "මැන\u{dca}ඩ\u{dd2}ය\u{dcf}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Mandiana Prefecture"), ("ta", "மண\u{bcd}டிய\u{bbe}ன\u{bbe} ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "మ\u{c3e}ండ\u{c3f}య\u{c3e}న\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "แมนเด\u{e35}ยนา พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Mandiana Prefecture"), ("uk", "Префектура Мандіана"), ("ur", "ماندیانا پریفیکچور"), ("vi", "Tỉnh Mandiana"), ("zh", "芒賈納省")]),
                        unofficial_name_list: ["Mandiana"].to_vec(),
                    }
                ),
                (
                    "ML",
                    Subdivision{
                        name: "ML",
                        country_alpha2: Alpha2::GN,
                        code: "ML",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.083333), longitude: Some(-12.3), max_latitude: Some(12.087919), min_latitude: Some(12.0613546), max_longitude: Some(-12.2849035), min_longitude: Some(-12.310524)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية مالي"), ("bn", "ম\u{9be}লি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄟𑄣\u{11128}"), ("ceb", "Mali Prefecture"), ("da", "Mali Prefecture"), ("de", "Mali"), ("el", "Μαλί"), ("en", "Mali"), ("es", "Ayuntamiento del Mali"), ("fi", "Malin prefektuuri"), ("fr", "préfecture de Mali"), ("gu", "માલી પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "माली प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Mali"), ("it", "Prefettura di Mali"), ("ja", "マリ県"), ("kn", "ಮಾಲ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "말리 지방 행정 구역"), ("lt", "Malio prefektūra"), ("lv", "Mali prefektūra"), ("mr", "माली प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Mali Prefecture"), ("nb", "Mali"), ("nl", "Mali Prefecture"), ("no", "Mali"), ("pl", "Prefektura Mali"), ("pt", "Prefeitura de Mali"), ("ru", "Префектура Мали"), ("si", "ම\u{dcf}ල\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Mali Prefecture"), ("ta", "மலி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "మ\u{c3e}ల\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}ఛర\u{c4d}"), ("th", "เม\u{e37}องแมล\u{e34}"), ("tr", "Mali Prefecture"), ("uk", "Префектура Малі"), ("ur", "مالی پریفیکچور"), ("vi", "Tỉnh Mali"), ("zh", "馬里省")]),
                        unofficial_name_list: ["Mali"].to_vec(),
                    }
                ),
                (
                    "MM",
                    Subdivision{
                        name: "MM",
                        country_alpha2: Alpha2::GN,
                        code: "MM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.5736024), longitude: Some(-11.8891721), max_latitude: Some(11.335539), min_latitude: Some(9.871374999999999), max_longitude: Some(-11.3715681), min_longitude: Some(-12.976967)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄟\u{1112f}"), ("ceb", "Mamou Prefecture"), ("de", "Mamou"), ("el", "Μαμού²"), ("en", "Mamou"), ("es", "Mamou²"), ("fr", "préfecture de Mamou"), ("it", "Prefettura di Mamou"), ("ja", "マムー県"), ("nb", "Mamou²"), ("nl", "Mamou Prefecture"), ("no", "Mamou²"), ("pl", "Prefektura Mamou"), ("pt", "Mamou²"), ("ru", "Маму²"), ("sv", "Mamou Prefecture"), ("ur", "مامؤ پریفیکچور"), ("zh", "馬木省")]),
                        unofficial_name_list: ["Mamou"].to_vec(),
                    }
                ),
                (
                    "N",
                    Subdivision{
                        name: "N",
                        country_alpha2: Alpha2::GN,
                        code: "N",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AdministrativeRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم نزيريكوري"), ("be", "Рэгіён Нзерэкарэ"), ("bg", "Нзерекоре"), ("bn", "এনজেরেক\u{9c1}রে অঞ\u{9cd}চল"), ("ccp", "𑄎𑄬𑄢𑄬𑄇\u{1112e}𑄢𑄬 𑄢\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Nzerekore Region"), ("da", "Nzerekore Region"), ("de", "Region Nzérékoré"), ("el", "Νζερεκορέ"), ("en", "Nzérékoré Region"), ("es", "Nzérékoré"), ("eu", "Nzérékoréko eskualdea"), ("fi", "Nzérékoré"), ("fr", "Région de Nzérékoré"), ("gu", "નઝ\u{ac7}ર\u{ac7}કોર\u{ac7} પ\u{acd}રદ\u{ac7}શ"), ("hi", "ज\u{93c}\u{947}रीकोर प\u{94d}रद\u{947}श"), ("id", "Wilayah Nzérékoré"), ("it", "regione di Nzérékoré"), ("ja", "ンゼレコレ州"), ("kn", "ಎನ\u{ccd}ಜ\u{cc6}ರ\u{ccd}ಕ\u{cc6}ರ\u{cc6} ಪ\u{ccd}ರದೇಶ"), ("ko", "은제레코레 주"), ("lt", "Nzerekorės regionas"), ("lv", "Nzerekores reģions"), ("mr", "नझ\u{947}र\u{947}कोर\u{947} प\u{94d}रद\u{947}श"), ("ms", "Nzerekore Region"), ("nb", "Nzérékoré"), ("nl", "Nzérékoré"), ("no", "Nzérékoré"), ("pl", "Nzérékoré"), ("pt", "Nzérékoré"), ("ro", "Regiunea Nzérékoré"), ("ru", "Нзерекоре"), ("si", "එන\u{dca}සේරෙකොරේ කල\u{dcf}පය"), ("sv", "Nzerekore Region"), ("ta", "நஸிரிகோரே பகுதி"), ("te", "ఎన\u{c4d}జ\u{c3f}యర\u{c4d}క\u{c4b}ర\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แถบเอ\u{e47}นเซเรโคเร"), ("tr", "Nzérékoré Bölgesi"), ("uk", "Регіон Нзерекоре"), ("ur", "نزیریکورے علاقہ"), ("vi", "Khu vực Nzérékoré"), ("zh", "恩澤雷科雷大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NZ",
                    Subdivision{
                        name: "NZ",
                        country_alpha2: Alpha2::GN,
                        code: "NZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.75), longitude: Some(-8.816666999999999), max_latitude: Some(7.803350799999999), min_latitude: Some(7.7149041), max_longitude: Some(-8.780307800000001), min_longitude: Some(-8.845367399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎𑄬𑄢𑄬𑄇\u{1112e}𑄢𑄬"), ("ceb", "Nzerekore Prefecture"), ("de", "Nzérékoré"), ("el", "Νζερεκορέ²"), ("en", "Nzérékoré"), ("es", "Nzérékoré²"), ("fr", "préfecture de Nzérékoré"), ("it", "Prefettura di Nzérékoré"), ("ja", "ンゼレコレ県"), ("nb", "Nzérékoré²"), ("nl", "Nzérékoré Prefecture"), ("no", "Nzérékoré²"), ("pl", "Prefektura Nzérékoré"), ("sv", "Nzerekore Prefecture"), ("ur", "نزیریکورے پریفیکچور"), ("zh", "恩澤雷科雷省")]),
                        unofficial_name_list: ["Nzérékoré"].to_vec(),
                    }
                ),
                (
                    "PI",
                    Subdivision{
                        name: "PI",
                        country_alpha2: Alpha2::GN,
                        code: "PI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.059568), longitude: Some(-12.395582), max_latitude: Some(11.0790367), min_latitude: Some(11.0282726), max_longitude: Some(-12.370305), min_longitude: Some(-12.4155379)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بيتا"), ("bn", "পিট\u{9be} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄛\u{11128}𑄑"), ("ceb", "Pita"), ("da", "Pita Prefecture"), ("de", "Pita"), ("el", "Πιτά"), ("en", "Pita"), ("es", "Ayuntamiento Pita"), ("fi", "Pitan prefektuuri"), ("fr", "préfecture de Pita"), ("gu", "પિટા પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "पीटा प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Pita"), ("it", "Prefettura di Pita"), ("ja", "ピタ県"), ("kn", "ಪ\u{cbf}ಟಾ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "피타 지방 행정 구역"), ("lt", "Pitos prefektūra"), ("lv", "Pitas prefektūra"), ("mr", "पिटा प\u{94d}रीफ\u{947}क\u{94d}च\u{941}अर"), ("ms", "Pita Prefecture"), ("nb", "Pita"), ("nl", "Pita Prefecture"), ("no", "Pita"), ("pl", "Pita (prefektura)"), ("pt", "Pita"), ("ru", "Префектура Пита"), ("si", "ප\u{dd2}ට\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Pita (prefektur)"), ("ta", "பிட ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "ప\u{c40}ట\u{c3e} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "พ\u{e34}ต\u{e49}า พร\u{e35}เฟ\u{e47}คเตอร\u{e4c}"), ("tr", "Pita Prefecture"), ("uk", "Префектура Піта"), ("ur", "پیتا پریفیکچور"), ("vi", "Tỉnh Pita"), ("zh", "皮塔省")]),
                        unofficial_name_list: ["Pita"].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::GN,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.416667), longitude: Some(-9.166667), max_latitude: Some(11.440052), min_latitude: Some(11.3927665), max_longitude: Some(-9.1520426), min_longitude: Some(-9.2001034)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيغيري"), ("bn", "সিগ\u{9c1}ইরি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ca", "prefectura de Siguiri"), ("ccp", "𑄥\u{11128}𑄉\u{1112d}\u{1112a}𑄢\u{11128}"), ("ceb", "Siguiri Prefecture"), ("da", "Siguiri Prefecture"), ("de", "Siguiri"), ("el", "Σιγκιρί"), ("en", "Siguiri"), ("es", "Condado de Siguiri"), ("fi", "Siguirin prefektuuri"), ("fr", "préfecture de Siguiri"), ("gu", "સિગ\u{ac1}ઈરી પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "सिग\u{941}इरी प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Siguiri"), ("it", "Prefettura di Siguiri"), ("ja", "シギリ県"), ("kn", "ಸ\u{cbf}ಗುರ\u{cbf} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "시기리 현"), ("lt", "Siguirio prefektūra"), ("lv", "Sigiri prefektūra"), ("mr", "सिग\u{941}ई प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Siguiri Prefecture"), ("nb", "Siguiri"), ("nl", "Siguiri Prefecture"), ("no", "Siguiri"), ("pl", "Prefektura Siguiri"), ("pt", "Siguiri"), ("ru", "Префектура Сигири"), ("si", "ස\u{dd2}න\u{dca}ග\u{dd4}ය\u{dd2}ර\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Siguiri Prefecture"), ("ta", "ச\u{bc0}க\u{bcd}குயிரி ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுர\u{bcd}"), ("te", "స\u{c3f}గ\u{c48}ర\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ซ\u{e34}ก\u{e39}อ\u{e34}ร\u{e35} พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Siguiri Prefecture"), ("uk", "Префектура Сігірі"), ("ur", "سیگویری پریفیکچور"), ("vi", "Quận Siguiri"), ("zh", "錫吉里省")]),
                        unofficial_name_list: ["Siguiri"].to_vec(),
                    }
                ),
                (
                    "TE",
                    Subdivision{
                        name: "TE",
                        country_alpha2: Alpha2::GN,
                        code: "TE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.9), longitude: Some(-13.033333), max_latitude: Some(10.9323432), min_latitude: Some(10.8810167), max_longitude: Some(-13.0202578), min_longitude: Some(-13.0482388)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تيليميلي"), ("bn", "টেলিমেলি প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄑𑄬𑄣\u{11128}𑄟𑄬𑄣𑄬"), ("ceb", "Telimele Prefecture"), ("da", "Télimélé Prefecture"), ("de", "Télimélé"), ("el", "Τελιμελέ"), ("en", "Télimélé"), ("es", "Télimélé"), ("fa", "تلیمله، گینه"), ("fi", "Télimélén prefektuuri"), ("fr", "préfecture de Télimélé"), ("gu", "ટ\u{ac7}લીમ\u{ac7}લ\u{ac7} પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "ट\u{947}ल\u{947}मिल\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Télimélé"), ("it", "Prefettura di Télimélé"), ("ja", "テリメレ県"), ("kn", "ಟ\u{cc6}ಲ\u{cbf}ಲ\u{cbf}ಫ\u{cc6} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "텔리멜리 지방 행정 구역"), ("lt", "Telimelės prefektūra"), ("lv", "Telimeles prefektūra"), ("mr", "ट\u{947}लिम\u{947}ल\u{947} प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Telimele Prefecture"), ("nb", "Télimélé"), ("nl", "Télimélé Prefecture"), ("no", "Télimélé"), ("pl", "Prefektura Télimélé"), ("pt", "Télimélé"), ("ru", "Префектура Телимеле"), ("si", "ටෙල\u{dd2}මෙලේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Telimele Prefecture"), ("ta", "டெலிமெலே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ట\u{c46}ల\u{c3f}మ\u{c3f}ల\u{c3f} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "เทล\u{e34}เมเล"), ("tr", "Télimélé Prefecture"), ("uk", "Префектура Телімеле"), ("ur", "تیلیمیلے پریفیکچور"), ("vi", "Quận Télimélé"), ("zh", "泰利梅萊省")]),
                        unofficial_name_list: ["Télimélé"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::GN,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.4464221), longitude: Some(-11.6641388), max_latitude: Some(11.4590797), min_latitude: Some(11.4313187), max_longitude: Some(-11.6549492), min_longitude: Some(-11.6786385)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة توغيه"), ("bn", "তউগ\u{9c1} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄒𑄋\u{11134}"), ("ceb", "Tougue Prefecture"), ("da", "Tougué Prefecture"), ("de", "Tougué"), ("el", "Τουγκουέ"), ("en", "Tougué"), ("es", "Tougué"), ("fi", "Touguén prefektuuri"), ("fr", "préfecture de Tougué"), ("gu", "ટ\u{ac1}ગ\u{acd}ય\u{ac1}એ પ\u{acd}રીફ\u{ac7}કચર"), ("hi", "त\u{941}ग\u{941}ए प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Tougué"), ("it", "Prefettura di Tougué"), ("ja", "トゥゲ県"), ("kn", "ಟ\u{ccc}ಗೇ ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "투구 지방 행정 구역"), ("lt", "Togės prefektūra"), ("lv", "Tuges prefektūra"), ("mr", "ट\u{941}ग\u{947}ई प\u{94d}रीफ\u{947}क\u{94d}चर"), ("ms", "Tougue Prefecture"), ("nb", "Tougué"), ("nl", "Tougué Prefecture"), ("no", "Tougué"), ("pl", "Prefektura Tougué"), ("pt", "Prefeitura de Tougué"), ("ru", "Префектура Туге"), ("si", "ටෝඌ ග\u{dd4} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Tougue Prefecture"), ("ta", "டௌகுவே ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "ట\u{c4c}గ\u{c4d} ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "ต\u{e39}วก\u{e39}ว พร\u{e35}เฟคเตอร\u{e4c}"), ("tr", "Tougué Prefecture"), ("uk", "Префектура Туге"), ("ur", "تؤگوے پریفیکچور"), ("vi", "Quận Tougué"), ("zh", "圖蓋省")]),
                        unofficial_name_list: ["Tougué"].to_vec(),
                    }
                ),
                (
                    "YO",
                    Subdivision{
                        name: "YO",
                        country_alpha2: Alpha2::GN,
                        code: "YO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.566000000000001), longitude: Some(-9.2533), max_latitude: Some(7.5800725), min_latitude: Some(7.5576959), max_longitude: Some(-9.2434503), min_longitude: Some(-9.2723751)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Prefecture,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة يومو"), ("bn", "ইয\u{9bc}োম\u{9c1} প\u{9cd}রশ\u{9be}সনিক অঞ\u{9cd}চল"), ("ccp", "𑄃\u{11128}𑄠𑄟\u{1112f}"), ("ceb", "Yomou"), ("da", "Yomou Prefecture"), ("de", "Yomou"), ("el", "Γιομού"), ("en", "Yomou"), ("es", "Yomou"), ("fi", "Yomoun prefektuuri"), ("fr", "préfecture de Yomou"), ("gu", "યોમોઉ પ\u{acd}રીફ\u{ac7}ક\u{acd}ચર"), ("hi", "यौमो प\u{94d}रीफ\u{947}क\u{94d}चर"), ("id", "Prefektur Yomou"), ("it", "Prefettura di Yomou"), ("ja", "ヨムー県"), ("kn", "ಯೋಮ\u{ccc} ಪ\u{ccd}ರ\u{cbf}ಫ\u{cc6}ಕ\u{ccd}ಚರ\u{ccd}"), ("ko", "요무 지방 행정 구역"), ("lt", "Jumou prefektūra"), ("lv", "Jomu prefektūra"), ("mr", "योमोऊ प\u{94d}रीफ\u{947}क\u{94d}चअर"), ("ms", "Yomou Prefecture"), ("nb", "Yomou"), ("nl", "Yomou Prefecture"), ("no", "Yomou"), ("pl", "Prefektura Yomou"), ("pt", "Prefeitura de Yomou"), ("ru", "Префектура Йому"), ("si", "යොමෞ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Yomou (prefektur)"), ("ta", "யோமோஉ ப\u{bcd}ர\u{bc0}பெக\u{bcd}ட\u{bcd}டுறே"), ("te", "య\u{c4b}మ\u{c4b}వు ప\u{c4d}ర\u{c3f}ఫ\u{c46}క\u{c4d}చర\u{c4d}"), ("th", "โยวม\u{e38} พร\u{e35}เพคเตอร\u{e4c}"), ("tr", "Yomou Prefecture"), ("uk", "Префектура Йому"), ("ur", "یومو پرفکترے"), ("vi", "Quận Yomou"), ("zh", "約穆省")]),
                        unofficial_name_list: ["Yomou"].to_vec(),
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
#[cfg(feature = "gn")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GN,
        alpha3: Alpha3::GIN,
        address_format: None,
        continent: Continent::Africa,
        country_code: 224,
        currency_code: "GNF",
        gec: Some(GEC::GV),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("GUI"),
        iso_long_name: "The Republic of Guinea",
        iso_short_name: "Guinea",
        official_language_list: ["ff", "fr"].to_vec(),
        spoken_language_list: ["ff", "fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Guinean"),
        number: "324",
        postal_code: true,
        postal_code_format: Some("\\d{3}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAfrica),
        un_locode: "GN",
        unofficial_name_list: ["Guinea", "Guinée", "ギニア", "Guinee"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Guinea"),
            ("af", "Guinee"),
            ("ak", "Guinea"),
            ("am", "ጊኔ"),
            ("an", "Guinea"),
            ("ar", "غينيا"),
            ("as", "গিনি"),
            ("ay", "Guinea"),
            ("az", "Qvineya"),
            ("ba", "Guinea"),
            ("be", "Гвінея"),
            ("bg", "Гвинея"),
            ("bi", "Guinea"),
            ("bn", "গিনি"),
            ("bn_IN", "গিনি"),
            ("br", "Ginea"),
            ("bs", "Gvineja"),
            ("ca", "Guinea"),
            ("ce", "Гвине"),
            ("ch", "Guinea"),
            ("cs", "Guinea"),
            ("cv", "Гвине"),
            ("cy", "Guinea"),
            ("da", "Guinea"),
            ("de", "Guinea"),
            ("dv", "ގ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "ག\u{f72}་ན\u{f72}།"),
            ("ee", "Guinea"),
            ("el", "Γουινέα"),
            ("en", "Guinea"),
            ("eo", "Gvineo"),
            ("es", "Guinea"),
            ("et", "Guinea"),
            ("eu", "Ginea"),
            ("fa", "گینه"),
            ("ff", "Gine"),
            ("fi", "Guinea"),
            ("fo", "Guinea"),
            ("fr", "Guinée"),
            ("fy", "Guinee"),
            ("ga", "An Ghuine"),
            ("gl", "Guinea"),
            ("gn", "Guinea"),
            ("gu", "ગિએના"),
            ("gv", "Yn Ghuinea"),
            ("ha", "Gine"),
            ("he", "גינאה"),
            ("hi", "गिनी"),
            ("hr", "Gvineja"),
            ("ht", "Gine"),
            ("hu", "Guinea"),
            ("hy", "Գվինեա"),
            ("ia", "Guinea"),
            ("id", "Guinea"),
            ("io", "Guinea"),
            ("is", "Gínea"),
            ("it", "Guinea"),
            ("iu", "Guinea"),
            ("ja", "ギニア"),
            ("ka", "გვინეა"),
            ("ki", "Guinea"),
            ("kk", "Гвинея"),
            ("kl", "Guinea"),
            ("km", "ហ\u{17d2}គ\u{17b8}ណេ"),
            ("kn", "ಗ\u{cbf}ನೀ"),
            ("ko", "기니"),
            ("ku", "Gîne"),
            ("kv", "Guinea"),
            ("kw", "Gyni"),
            ("ky", "Гвинея"),
            ("lo", "Guinea"),
            ("lt", "Gvinėja"),
            ("lv", "Gvineja"),
            ("mi", "Guinea"),
            ("mk", "Гвинеја"),
            ("ml", "ഗിനിയ"),
            ("mn", "Гвиней"),
            ("mr", "गिनी"),
            ("ms", "Guinea"),
            ("mt", "Gineja"),
            (
                "my",
                "ဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Gini"),
            ("nb", "Guinea"),
            ("ne", "जिनिया"),
            ("nl", "Guinee"),
            ("nn", "Guinea"),
            ("nv", "Guinea"),
            ("oc", "Guinèa"),
            ("or", "ଗ\u{b3f}ନୀ"),
            ("pa", "ਗ\u{a42}ਈਨੀਆ"),
            ("pi", "गिनी"),
            ("pl", "Gwinea"),
            ("ps", "ګیانا"),
            ("pt", "Guiné"),
            ("pt_BR", "Guiné"),
            ("ro", "Guinea"),
            ("ru", "Гвинея"),
            ("rw", "Gineya"),
            ("sc", "Guinea"),
            ("sd", "Guinea"),
            ("si", "ග\u{dd2}න\u{dd2}ය\u{dcf}ව"),
            ("sk", "Guinea"),
            ("sl", "Gvineja"),
            ("so", "Gini"),
            ("sq", "Guine"),
            ("sr", "Гвинеја"),
            ("sv", "Guinea"),
            ("sw", "Guinea"),
            ("ta", "கினி"),
            ("te", "గ\u{c3f}న\u{c40}"),
            ("tg", "Гвинея"),
            ("th", "ก\u{e34}น\u{e35}"),
            ("ti", "ጊኒ"),
            ("tk", "Gwineýa"),
            ("tl", "Guinea"),
            ("tr", "Gine"),
            ("tt", "Gуинеа"),
            ("ug", "گىۋىنېيە"),
            ("uk", "Гвінея"),
            ("ur", "جمہوریہ گنی"),
            ("uz", "Gvineya"),
            ("ve", "Guinea"),
            ("vi", "Ghi-nê"),
            ("wa", "Guinêye"),
            ("wo", "Ginne"),
            ("xh", "Guinea"),
            ("yo", "Guinea"),
            ("zh_CN", "几内亚"),
            ("zh_HK", "畿內亞"),
            ("zh_TW", "幾內亞"),
            ("zu", "IGini"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}