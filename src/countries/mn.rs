// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Mongolia

#[cfg(all(feature = "mn", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MN;
    pub const ALPHA3: Alpha3 = Alpha3::MNG;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 976;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::MNT;
    pub const GEC: Option<GEC> = Some(GEC::MG);
    pub const INTERNATIONAL_PREFIX: &str = "001";
    pub const IOC: Option<IOC> = Some(IOC::MGL);
    pub const ISO_SHORT_NAME: &str = "Mongolia";
    pub const ISO_LONG_NAME: &str = "Mongolia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["mn"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["mn"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Mongolian");
    pub const NUMBER: &str = "496";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "MN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Mongolia", "Mongolei", "Mongolie", "モンゴル", "Mongolië"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Mongolia"),
        ("af", "Mongolië"),
        ("ak", "Mongolia"),
        ("am", "ሥንጕሑ።"),
        ("an", "Mongolia"),
        ("ar", "منغوليا"),
        ("as", "মঙ\u{9cd}গোলিয়\u{9be}"),
        ("ay", "Mongolia"),
        ("az", "Monqolustan"),
        ("ba", "Mongolia"),
        ("be", "Манголія"),
        ("bg", "Монголия"),
        ("bi", "Mongolia"),
        ("bn", "মঙ\u{9cd}গোলিয়\u{9be}"),
        ("bn_IN", "মঙ\u{9cd}গোলিয়\u{9be}"),
        ("br", "Mongolia"),
        ("bs", "Mongolija"),
        ("ca", "Mongòlia"),
        ("ce", "Монголи"),
        ("ch", "Mongolia"),
        ("cs", "Mongolsko"),
        ("cv", "Монголи"),
        ("cy", "Mongolia"),
        ("da", "Mongoliet"),
        ("de", "Mongolei"),
        ("dv", "މ\u{7ae}ނ\u{7b0}ގ\u{7af}ލ\u{7a8}އ\u{7a7}"),
        ("dz", "མ\u{f7c}ང་ག\u{f7c}་ལ\u{f72}་ཡ།"),
        ("ee", "Mongolia"),
        ("el", "Μογγολία"),
        ("en", "Mongolia"),
        ("eo", "Mongolio"),
        ("es", "Mongolia"),
        ("et", "Mongoolia"),
        ("eu", "Mongolia"),
        ("fa", "مغولستان"),
        ("ff", "Mongolia"),
        ("fi", "Mongolia"),
        ("fo", "Mongolia"),
        ("fr", "Mongolie"),
        ("fy", "Mongoalje"),
        ("ga", "An Mhongóil"),
        ("gl", "Mongolia"),
        ("gn", "Mongolia"),
        ("gu", "મો\u{a82}ગોલિયા"),
        ("gv", "Yn Vongoil"),
        ("ha", "Mangolia"),
        ("he", "מונגוליה"),
        ("hi", "म\u{902}गोलिया"),
        ("hr", "Mongolija"),
        ("ht", "Mongoli"),
        ("hu", "Mongólia"),
        ("hy", "Մոնղոլիա"),
        ("ia", "Mongolia"),
        ("id", "Mongolia"),
        ("io", "Mongolia"),
        ("is", "Mongólía"),
        ("it", "Mongolia"),
        ("iu", "Mongolia"),
        ("ja", "モンゴル国"),
        ("ka", "მონღოლეთი"),
        ("ki", "Mongolia"),
        ("kk", "Монғолия"),
        ("kl", "Mongolia"),
        ("km", "ម\u{17c9}\u{17bb}ងហ\u{17d2}គោល\u{17b8}"),
        ("kn", "ಮಂಗೋಲ\u{cbf}ಯಾ"),
        ("ko", "몽골"),
        ("ku", "Mongolya"),
        ("kv", "Монголия"),
        ("kw", "Mongoli"),
        ("ky", "Моңголстан"),
        ("lo", "Mongolia"),
        ("lt", "Mongolija"),
        ("lv", "Mongolija"),
        ("mi", "Mongōria"),
        ("mk", "Монголија"),
        ("ml", "മംഗോളിയ"),
        ("mn", "Монгол"),
        ("mr", "म\u{902}गोलिया"),
        ("ms", "Mongolia"),
        ("mt", "Mongolja"),
        (
            "my",
            "မ\u{103d}န\u{103a}ဂ\u{102d}\u{102f}းလ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Mongoriya"),
        ("nb", "Mongolia"),
        ("ne", "मङ\u{94d}गोलिया"),
        ("nl", "Mongolië"),
        ("nn", "Mongolia"),
        ("nv", "Chʼah Diʼilii Bikéyah"),
        ("oc", "Mongolia"),
        ("or", "ମଙ\u{b4d}ଗୋଲ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਮ\u{a70}ਗ\u{a4b}ਲੀਆ"),
        ("pi", "म\u{902}गोलिया"),
        ("pl", "Mongolia"),
        ("ps", "مغولستان"),
        ("pt", "Mongólia"),
        ("pt_BR", "Mongólia"),
        ("ro", "Mongolia"),
        ("ru", "Монголия"),
        ("rw", "Mongoliya"),
        ("sc", "Mongòlia"),
        ("sd", "Mongolia"),
        ("si", "මොංගෝල\u{dd2}ය\u{dcf}ව"),
        ("sk", "Mongolsko"),
        ("sl", "Mongolija"),
        ("so", "Mongolia"),
        ("sq", "Mongoli"),
        ("sr", "Монголија"),
        ("sv", "Mongoliet"),
        ("sw", "Mongolia"),
        ("ta", "மங\u{bcd}கோலிய\u{bbe}"),
        ("te", "మంగ\u{c4b}ల\u{c3f}య\u{c3e}"),
        ("tg", "Муғулистон"),
        ("th", "มองโกเล\u{e35}ย"),
        ("ti", "ሞንጎልያ"),
        ("tk", "Mongoliýa"),
        ("tl", "Mongolia"),
        ("tr", "Moğolistan"),
        ("tt", "Моголстан"),
        ("ug", "موڭغۇلىيە"),
        ("uk", "Монголія"),
        ("ur", "منگولیا"),
        ("uz", "Moʻgʻuliston"),
        ("ve", "Mongolia"),
        ("vi", "Mông Cổ"),
        ("wa", "Mongoleye"),
        ("wo", "Moŋgooli"),
        ("xh", "Mongolia"),
        ("yo", "Mòngólíà"),
        ("zh_CN", "蒙古"),
        ("zh_HK", "蒙古"),
        ("zh_TW", "蒙古"),
        ("zu", "Mongolia"),
    ];
    #[cfg(all(feature = "mn", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 46.862496;
        pub const LONGITUDE: f64 = 103.846656;
        pub const MAX_LATITUDE: f64 = 52.148355;
        pub const MAX_LONGITUDE: f64 = 119.9315098;
        pub const MIN_LATITUDE: f64 = 41.581833;
        pub const MIN_LONGITUDE: f64 = 87.7344789;
        pub const NORTHEAST_LATITUDE: f64 = 52.148355;
        pub const NORTHEAST_LONGITUDE: f64 = 119.9315098;
        pub const SOUTHWEST_LATITUDE: f64 = 41.581833;
        pub const SOUTHWEST_LONGITUDE: f64 = 87.7344789;
    }
}
#[cfg(all(feature = "mn", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 46.862496,
            longitude: 103.846656,
            max_latitude: 52.148355,
            max_longitude: 119.9315098,
            min_latitude: 41.581833,
            min_longitude: 87.7344789,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 52.148355,
                    longitude: 119.9315098,
                },
                southwest: CountryGeoBound {
                    latitude: 41.581833,
                    longitude: 87.7344789,
                },
            },
        }
    }
}

#[cfg(all(feature = "mn", feature = "subdivisions"))]
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
                    "035",
                    Subdivision{
                        name: "Orhon",
                        country_alpha2: Alpha2::MN,
                        code: "035",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.004705), longitude: Some(104.3016527), max_latitude: Some(49.1322389), min_latitude: Some(48.9046019), max_longitude: Some(104.577072), min_longitude: Some(104.004139)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أورخون"), ("az", "Orxon vilayəti"), ("bg", "Орхон"), ("bn", "ওরকন প\u{9cd}রদেশ"), ("ca", "Província d’Orkhon"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄈\u{1112e}𑄚\u{11134}"), ("ceb", "Orhon Aymag"), ("cs", "Orchonský ajmag"), ("da", "Orchon-Aimag"), ("de", "Orchon-Aimag"), ("el", "Ορκόν"), ("en", "Orkhon"), ("es", "Orhon"), ("et", "Orhoni aimakk"), ("fa", "استان آرخان"), ("fi", "Orhon"), ("fr", "Orkhon"), ("ga", "Orkhon"), ("gu", "ઓરખોન પ\u{acd}રા\u{a82}ત"), ("he", "אורחון"), ("hi", "ओरख\u{93c}ोन प\u{94d}रान\u{94d}त"), ("hu", "Orhon (tartomány)"), ("id", "Provinsi Orkhon"), ("it", "provincia dell’Orhon"), ("ja", "オルホン県"), ("ka", "ორხონის აიმაკი"), ("kn", "ಆರ\u{ccd}ಖೋನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오르홍 주"), ("lt", "Orchono aimakas"), ("lv", "Orhonas aimaks"), ("mk", "Орхон"), ("mn", "Орхон"), ("mr", "ओरखोन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Orkhon"), ("nb", "Orkhon"), ("nl", "Orhon"), ("no", "Orkhon"), ("pl", "Ajmak orchoński"), ("pt", "Orhon"), ("ro", "Orchon-Aimag"), ("ru", "Орхон"), ("si", "ඔර\u{dca}ක\u{dca}හොන\u{dca} පළ\u{dcf}ත"), ("sv", "Orchon"), ("ta", "ஓர\u{bcd}க\u{bcd}ஹன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆర\u{c4d}ఖ\u{c4b}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดออร\u{e4c}คอน"), ("tr", "Orhun"), ("uk", "Орхон"), ("ur", "ارخون صوبہ"), ("vi", "Orkhon"), ("zh", "鄂尔浑省")]),
                        unofficial_name_list: ["Orhon"].to_vec(),
                    }
                ),
                (
                    "037",
                    Subdivision{
                        name: "Darhan uul",
                        country_alpha2: Alpha2::MN,
                        code: "037",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.448929), longitude: Some(106.2325245), max_latitude: Some(49.82596590000001), min_latitude: Some(49.084558), max_longitude: Some(106.806087), min_longitude: Some(105.8157009)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة درخان-وول"), ("az", "Darxan-Uul vilayəti"), ("bg", "Дархан Ул"), ("bn", "দ\u{9be}রখ\u{9be}ন-উল প\u{9cd}রদেশ"), ("ca", "Província de Darkhan-Uul"), ("ccp", "𑄓𑄢\u{11134}𑄈𑄚\u{11134}-𑄅\u{1112b}𑄣\u{11134}"), ("ceb", "Darhan-Uul Aymag"), ("cs", "Darchanúlský ajmag"), ("da", "Darkhan-Uul"), ("de", "Darchan-Uul-Aimag"), ("el", "Νταρκχάν-Ουούλ"), ("en", "Darkhan-Uul"), ("es", "Darhan-Uul"), ("et", "Darhani aimakk"), ("fa", "استان دارخن-یول"), ("fi", "Darhan-Uul"), ("fr", "Darhan-Uul"), ("ga", "Darkhan-Uul"), ("gu", "ડાર\u{acd}ખન-ઉલ પ\u{acd}રા\u{a82}ત"), ("he", "דרחאן-אול"), ("hi", "दरख\u{93c}ान-ऊल प\u{94d}रा\u{902}त"), ("hu", "Darhan-Úl ajmag"), ("id", "Provinsi Darkhan-Uul"), ("it", "provincia di Darhan-Uul"), ("ja", "ダルハン・オール県"), ("ka", "დარხან-უული"), ("kn", "ಡರ\u{ccd}ಖನ\u{ccd}-ಉಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "다르항올 주"), ("lt", "Darchan Ulo aimakas"), ("lv", "Darhanūlas aimaks"), ("mk", "Дархан-Ул"), ("mn", "Дархан-Уул"), ("mr", "दारखान-उ\u{942}ल प\u{94d}रा\u{902}त"), ("ms", "Darkhan-Uul Province"), ("nb", "Darkhan-Uul"), ("nl", "Darhan-Uul"), ("no", "Darkhan-Uul"), ("pl", "Ajmak darchański"), ("pt", "Darhan-Uul"), ("ro", "Darchan-Uul-Aimag"), ("ru", "Дархан-Уул"), ("si", "ඩර\u{dca}ක\u{dca}හන\u{dca}-ඌල\u{dca} පළ\u{dcf}ත"), ("sr", "Дархан-Ул"), ("sr_Latn", "Darhan-Ul"), ("sv", "Darchan-Uul"), ("ta", "ட\u{bbe}ர\u{bcd}க\u{bcd}கின\u{bcd}-யூல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డర\u{c4d}ఖన\u{c4d}-ఉల\u{c4d}-ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดาร\u{e4c}ฮาน-อ\u{e39}ล"), ("tr", "Darhan-Ul"), ("uk", "Дархан-Уул"), ("ur", "درخان-اول صوبہ"), ("vi", "Darkhan-Uul"), ("zh", "达尔汗乌勒省")]),
                        unofficial_name_list: ["Darhan uul"].to_vec(),
                    }
                ),
                (
                    "039",
                    Subdivision{
                        name: "Hentiy",
                        country_alpha2: Alpha2::MN,
                        code: "039",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6081209), longitude: Some(109.9372856), max_latitude: Some(49.4067421), min_latitude: Some(46.062024), max_longitude: Some(112.6971711), min_longitude: Some(108.350123)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة خنتي"), ("bg", "Хентий"), ("bn", "খনিটি প\u{9cd}রদেশ"), ("ca", "Província de Khentii"), ("ccp", "𑄈𑄬𑄚\u{11134}𑄑\u{11133}𑄦\u{11128}"), ("ceb", "Hentiy Aymag"), ("da", "Khentij"), ("de", "Chentii-Aimag"), ("el", "Χέντιι"), ("en", "Khentii"), ("es", "Hentiy"), ("et", "Hentij aimakk"), ("fa", "استان خنتی"), ("fi", "Hentii"), ("fr", "Khentii"), ("gu", "ખ\u{ac7}\u{a82}તી પ\u{acd}રા\u{a82}ત"), ("he", "חנטי"), ("hi", "ख\u{93c}\u{947}न\u{94d}ती प\u{94d}रा\u{902}त"), ("hu", "Hentij tartomány"), ("id", "Provinsi Khentii"), ("it", "provincia del Hėntij"), ("ja", "ヘンティー県"), ("ka", "ხენტიი"), ("kn", "ಕ\u{cc6}ಂಥ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "헹티 주"), ("lt", "Chentėjaus aimakas"), ("lv", "Henteja aimaks"), ("mk", "Хентиј"), ("mn", "Хэнтий"), ("mr", "ख\u{947}तीई प\u{94d}रा\u{902}त"), ("ms", "Khentii Province"), ("nb", "Khentij"), ("nl", "Henti"), ("no", "Khentij"), ("pl", "Ajmak chentejski"), ("pt", "Khentiy"), ("ro", "Chentii-Aimag"), ("ru", "Хэнтий"), ("si", "ඛේන\u{dca}ට\u{dd2} පළ\u{dcf}ත"), ("sv", "Chentij"), ("ta", "க\u{bcd}ஹெண\u{bcd}டிய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c47}ంట\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเฮ\u{e47}นต\u{e35}"), ("tr", "Hentii ili"), ("uk", "Хентій"), ("ur", "خنتی صوبہ"), ("vi", "Khentii"), ("zh", "肯特省")]),
                        unofficial_name_list: ["Hentii", "Khentii"].to_vec(),
                    }
                ),
                (
                    "041",
                    Subdivision{
                        name: "Hövsgöl",
                        country_alpha2: Alpha2::MN,
                        code: "041",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.2204484), longitude: Some(100.3213768), max_latitude: Some(52.1542471), min_latitude: Some(48.233659), max_longitude: Some(102.78852), min_longitude: Some(96.879998)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خوفسغول"), ("bg", "Хьовсгьол"), ("bn", "খবসগোল প\u{9cd}রদেশ"), ("ca", "Província de Khövsgöl"), ("ccp", "𑄈\u{1112e}𑄛\u{11134}𑄉\u{1112e}𑄣\u{11134}"), ("ceb", "Hövsgöl Aymag"), ("da", "Khövsgöl"), ("de", "Chöwsgöl-Aimag"), ("el", "Κχοβσγκόλ"), ("en", "Khövsgöl"), ("es", "Hövsgöl"), ("et", "Hövsgöli aimakk"), ("fa", "استان خووسگول"), ("fi", "Hövsgöl"), ("fr", "Khövsgöl"), ("ga", "Khövsgöl"), ("gu", "ખોવસ\u{acd}ગોલ પ\u{acd}રા\u{a82}ત"), ("he", "חובסגול"), ("hi", "ख\u{93c}ोव\u{94d}स\u{94d}गोल प\u{94d}रा\u{902}त"), ("hu", "Hövszgöl ajmag"), ("id", "Provinsi Khövsgöl"), ("it", "provincia del Hôvsgôl"), ("ja", "フブスグル県"), ("ka", "ხუვსგელი"), ("kk", "Хөвсгөл аймағы"), ("kn", "ಖೊವಾಸ\u{ccd}ಗೋಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "흐브스글 주"), ("lt", "Chubsugulo aimakas"), ("lv", "Huvsgula aimaks"), ("mk", "Ховсгол"), ("mn", "Хөвсгөл"), ("mr", "खोव\u{94d}स\u{94d}ग\u{94d}लल प\u{94d}रा\u{902}त"), ("ms", "Khovsgol Province"), ("nb", "Khövsgöl"), ("nl", "Hövsgöl"), ("no", "Khövsgöl"), ("pl", "Ajmak chubsugulski"), ("pt", "Khövsgöl"), ("ro", "Chöwsgöl-Aimag"), ("ru", "Хувсгел"), ("si", "කොව\u{dca}ස\u{dca}ගෝල\u{dca} පළ\u{dcf}ත"), ("sv", "Chövsgöl"), ("ta", "க\u{bcd}ஹோவஸ\u{bcd}கொல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఖ\u{c4b}వస\u{c4d}\u{200c}గ\u{c4b}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกฟโกล"), ("tr", "Hövsgöl"), ("uk", "Хувсгел"), ("ur", "خووسگول صوبہ"), ("vi", "Tỉnh Khovsgol"), ("zh", "库苏古尔省")]),
                        unofficial_name_list: ["Hovsgol", "Khuvsgul"].to_vec(),
                    }
                ),
                (
                    "043",
                    Subdivision{
                        name: "Hovd",
                        country_alpha2: Alpha2::MN,
                        code: "043",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.004167), longitude: Some(91.65), max_latitude: Some(48.0193817), min_latitude: Some(47.9380819), max_longitude: Some(91.67026520000002), min_longitude: Some(91.6108704)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Xovd aymakı"), ("bg", "Ховд"), ("ca", "Província de Khovd"), ("ccp", "𑄇\u{1112e}𑄛\u{11134}𑄓\u{11134}"), ("ceb", "Hovd"), ("cy", "Talaith Khovd"), ("da", "Khovd"), ("de", "Chowd-Aimag"), ("en", "Khovd"), ("es", "Hovd"), ("et", "Hovdi aimakk"), ("fa", "استان خاود"), ("fi", "Hovd"), ("fr", "Khovd"), ("he", "חובד"), ("hi", "ख\u{93c}ोव\u{94d}द प\u{94d}रा\u{902}त"), ("hu", "Hovd tartomány"), ("id", "Provinsi Khovd"), ("it", "provincia di Hovd"), ("ja", "ホブド県"), ("ka", "ხოვდის აიმაკი"), ("kk", "Ховда аймағы"), ("ko", "허브드 주"), ("lt", "Kobdo aimakas"), ("mk", "Ховд"), ("mn", "Ховд"), ("nb", "Khovd"), ("nl", "Ajmag Hovd"), ("no", "Khovd"), ("pl", "Ajmak kobdoski"), ("pt", "Khovd"), ("ro", "Chowd-Aimag"), ("ru", "Ховд"), ("sv", "Chovd"), ("tr", "Hovd"), ("uk", "Ховд"), ("ur", "خوود صوبہ"), ("vi", "Khovd"), ("zh", "科布多省")]),
                        unofficial_name_list: ["Khovd"].to_vec(),
                    }
                ),
                (
                    "046",
                    Subdivision{
                        name: "Uvs",
                        country_alpha2: Alpha2::MN,
                        code: "046",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.64497069999999), longitude: Some(93.27365759999999), max_latitude: Some(50.89464599999999), min_latitude: Some(48.4370969), max_longitude: Some(95.7276089), min_longitude: Some(90.000101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوفس"), ("az", "Uvs"), ("be", "Убсунурскі аймак"), ("bg", "Увс"), ("bn", "উভস প\u{9cd}রদেশ"), ("ca", "Uvs"), ("ccp", "𑄅\u{1112a}𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Uvs Aymag"), ("da", "Uvs"), ("de", "Uws-Aimag"), ("el", "Ουβς"), ("en", "Uvs"), ("es", "Uvs"), ("et", "Uvsi aimakk"), ("fa", "استان اووس"), ("fi", "Uvs"), ("fr", "Uvs"), ("gu", "ય\u{ac1}વ\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("he", "אובס"), ("hi", "उव\u{94d}स प\u{94d}रा\u{902}त"), ("hu", "Uvsz tartomány"), ("id", "Provinsi Uvs"), ("it", "provincia dell’Uvs"), ("ja", "オブス県"), ("ka", "უვსის აიმაკი"), ("kk", "Увс аймағы"), ("kn", "ಯುವಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오브스 주"), ("lt", "Ubso aimakas"), ("lv", "Uvsas aimaks"), ("mk", "Увс"), ("mn", "Увс"), ("mr", "य\u{942}व\u{94d}हस प\u{94d}रा\u{902}त"), ("ms", "Uvs Province"), ("nb", "Uvs"), ("nl", "Uvs"), ("no", "Uvs"), ("pl", "Ajmak uwski"), ("pt", "Uvs"), ("ro", "Uws-Aimag"), ("ru", "Увс"), ("si", "උව\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sr", "Увс"), ("sr_Latn", "Uvs"), ("sv", "Uvs"), ("ta", "உவ\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "యువ\u{c3f}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e38}ฟส\u{e4c}"), ("tr", "Uvs"), ("uk", "Увс"), ("ur", "یوس صوبہ"), ("vi", "Uvs"), ("zh", "乌布苏省")]),
                        unofficial_name_list: ["Uvs"].to_vec(),
                    }
                ),
                (
                    "047",
                    Subdivision{
                        name: "Töv",
                        country_alpha2: Alpha2::MN,
                        code: "047",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2124056), longitude: Some(106.41541), max_latitude: Some(49.1231531), min_latitude: Some(46.38429199999999), max_longitude: Some(109.044458), min_longitude: Some(104.070697)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة توف"), ("az", "Töv vilayəti"), ("be", "Цэнтральны аймак"), ("bg", "Тьов"), ("bn", "তভ প\u{9cd}রদেশ"), ("ca", "Província de Töv"), ("ccp", "𑄑\u{1112e}𑄛\u{11134}"), ("ceb", "Töv Aymag"), ("da", "Töv"), ("de", "Töw-Aimag"), ("el", "Τοβ"), ("en", "Töv"), ("es", "Töv"), ("et", "Keskaimakk"), ("fa", "استان تو"), ("fi", "Tuv"), ("fr", "Töv"), ("gu", "ટોવ પ\u{acd}રા\u{a82}ત"), ("he", "טוב"), ("hi", "तोव प\u{94d}रा\u{902}त"), ("hu", "Központi tartomány"), ("hy", "Տուվե"), ("id", "Provinsi Töv"), ("it", "provincia del Tôv"), ("ja", "トゥブ県"), ("ka", "ტუვე"), ("kn", "ಟೊವ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "투브 주"), ("lt", "Tiobo aimakas"), ("lv", "Tuves aimaks"), ("mk", "Тов"), ("mn", "Төв"), ("mr", "टॉव प\u{94d}रा\u{902}त"), ("ms", "Tov Province"), ("nb", "Töv"), ("nl", "Töv"), ("no", "Töv"), ("pl", "Ajmak centralny"), ("pt", "Töv"), ("ro", "Töw-Aimag"), ("ru", "Туве"), ("si", "ටෝව\u{dca} පළ\u{dcf}ත"), ("sv", "Töv"), ("ta", "டோவ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c4b}వ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโตว"), ("tr", "Tuv"), ("uk", "Туве"), ("ur", "توو صوبہ"), ("vi", "Töv"), ("zh", "中央省")]),
                        unofficial_name_list: ["Tov"].to_vec(),
                    }
                ),
                (
                    "049",
                    Subdivision{
                        name: "Selenge",
                        country_alpha2: Alpha2::MN,
                        code: "049",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.0059273), longitude: Some(106.4434108), max_latitude: Some(50.4804), min_latitude: Some(48.49716799999999), max_longitude: Some(108.5555881), min_longitude: Some(104.345383)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيلنج"), ("bg", "Селенге"), ("bn", "সেলেঙ\u{9cd}গে প\u{9cd}রদেশ"), ("ca", "Província de Selenge"), ("ccp", "𑄥𑄬𑄣𑄬𑄚\u{11134}"), ("ceb", "Selenge Aymag"), ("cs", "Selengský ajmag"), ("da", "Selenge"), ("de", "Selenge-Aimag"), ("el", "Σέλενγκε"), ("en", "Selenge"), ("es", "Selenge"), ("et", "Selenge aimakk"), ("fa", "استان سلنگ"), ("fi", "Selenge"), ("fr", "Selenge"), ("gu", "સ\u{ac7}લ\u{ac7}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("he", "סלנגה"), ("hi", "स\u{947}ल\u{947}\u{902}ग\u{947} प\u{94d}रा\u{902}त"), ("hu", "Szelenge ajmag"), ("id", "Provinsi Selenge"), ("it", "provincia del Sėlėngė"), ("ja", "セレンゲ県"), ("ka", "სელენგე"), ("kn", "ಸ\u{cc6}ಲ\u{cc6}ಂಗ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "셀렝게 주"), ("lt", "Selengės aimakas"), ("lv", "Selengas aimaks"), ("mk", "Селенге"), ("mn", "Сэлэнгэ"), ("mr", "स\u{947}ल\u{947}झि\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Selenge Province"), ("nb", "Selenge"), ("nl", "Selenge"), ("no", "Selenge"), ("pl", "Ajmak selengijski"), ("pt", "Selenge"), ("ro", "Selenge-Aimag"), ("ru", "Сэлэнгэ"), ("si", "සෙලෙන\u{dca}ගේ පළ\u{dcf}ත"), ("sv", "Selenga"), ("ta", "செலெங\u{bcd}கே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ల\u{c46}ంజ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเซเลงเก"), ("tr", "Selenge"), ("uk", "Селенге"), ("ur", "سیلینگی صوبہ"), ("vi", "Selenge"), ("zh", "色楞格省")]),
                        unofficial_name_list: ["Selenge"].to_vec(),
                    }
                ),
                (
                    "051",
                    Subdivision{
                        name: "Sühbaatar",
                        country_alpha2: Alpha2::MN,
                        code: "051",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.236389), longitude: Some(106.206389), max_latitude: Some(50.265753), min_latitude: Some(50.1804158), max_longitude: Some(106.2765886), min_longitude: Some(106.1433793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سوخباتار"), ("bg", "Сухе Батор"), ("bn", "স\u{9c1}খবরত প\u{9cd}রদেশ"), ("ca", "Província de Sükhbaatar"), ("ccp", "𑄥\u{1112a}𑄇\u{11134}𑄝𑄑𑄢\u{11134}"), ("ceb", "Sühbaatar Aymag"), ("da", "Sükhbaatar"), ("de", "Süchbaatar-Aimag"), ("el", "Σουκχμπαατάρ"), ("en", "Sükhbaatar"), ("es", "Sühbaatar"), ("et", "Sühbaatari aimakk"), ("fa", "استان سوخباتر"), ("fi", "Sühbaatar"), ("fr", "Sühbaatar"), ("gu", "સ\u{ac1}ખબાતર પ\u{acd}રા\u{a82}ત"), ("he", "סוחבאאטאר"), ("hi", "स\u{942}खबातर (शहर)"), ("hu", "Szühebátor ajmag"), ("id", "Provinsi Sükhbaatar"), ("it", "provincia di Sùhbaatar"), ("ja", "スフバートル県"), ("ka", "სუხე-ბატორის აიმაკი"), ("kn", "ಸುಖಾಬಾಟರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "수흐바타르 주"), ("lt", "Suche Batoro aimakas"), ("lv", "Suhebatoras aimaks"), ("mk", "Сухбатар"), ("mn", "Сүхбаатар"), ("mr", "श\u{941}बब\u{94d}बर प\u{94d}रा\u{902}त"), ("ms", "Sukhbaatar Province"), ("nb", "Sükhbaatar"), ("nl", "Sühbaatar"), ("no", "Sükhbaatar"), ("pl", "Ajmak suchebatorski"), ("pt", "Sükhbaatar"), ("ro", "Süchbaatar-Aimag"), ("ru", "Сухэ-Батор"), ("si", "ස\u{dd4}ක\u{dca}බ\u{dcf}තර\u{dca} පළ\u{dcf}ත"), ("sv", "Süchbaatar"), ("ta", "சுக\u{bcd}ஹப\u{bbe}தர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సుఖ\u{c4d}\u{200c}బ\u{c3e}తర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e37}อบาตาร\u{e4c}"), ("tr", "Sühbatur"), ("uk", "Сухе-Батор"), ("ur", "سخباتار صوبہ"), ("vi", "Sükhbaatar"), ("zh", "苏赫巴托尔省")]),
                        unofficial_name_list: ["Sukhbaatar"].to_vec(),
                    }
                ),
                (
                    "053",
                    Subdivision{
                        name: "Ömnögovi",
                        country_alpha2: Alpha2::MN,
                        code: "053",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.500024), longitude: Some(104.2861116), max_latitude: Some(45.188183), min_latitude: Some(41.567638), max_longitude: Some(108.033485), min_longitude: Some(99.389793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أومنوغوفي"), ("bg", "Йомньо Гови"), ("bn", "ওম\u{9cd}নোগোভি প\u{9cd}রদেশ"), ("ca", "Província d’Ömnögovi"), ("ccp", "𑄃\u{11127}𑄟\u{11134}𑄚\u{1112e}𑄉\u{1112e}𑄞\u{11128}"), ("ceb", "Ömnögovi Province"), ("cs", "Jihogobijský ajmag"), ("da", "Ömnögov"), ("de", "Ömnö-Gobi-Aimag"), ("el", "Ομνογκόβι"), ("en", "Ömnögovi"), ("es", "Ömnögovi"), ("et", "Lõuna-Gobi aimakk"), ("fa", "استان اومنوگاوی"), ("fi", "Ömnögovi"), ("fr", "Ömnögovi"), ("gu", "ઓમ\u{acd}નોગોવિ પ\u{acd}રા\u{a82}ત"), ("he", "אומנוגובי"), ("hi", "ओम\u{94d}नोगोवी प\u{94d}रा\u{902}त"), ("hu", "Ömnögovi ajmag"), ("id", "Provinsi Ömnögovi"), ("it", "provincia dell’Ômnôgov’"), ("ja", "ウムヌゴビ県"), ("ka", "უმნეგოვი"), ("kn", "ಒಮ\u{ccd}ನೊಗೊವ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "으므느고비 주"), ("lt", "Pietų Gobio aimakas"), ("lv", "Dienvidgobi aimaks"), ("mk", "Омногови"), ("mn", "Өмнөговь"), ("mr", "ओमनोगोवी प\u{94d}रा\u{902}त"), ("ms", "Omnogovi Province"), ("nb", "Ömnögov"), ("nl", "Ömnögovĭ"), ("no", "Ömnögov"), ("pl", "Ajmak południowogobijski"), ("pt", "Ömnögovĭ"), ("ro", "Ömnö-Gobi-Aimag"), ("ru", "Умнеговь"), ("si", "ඕම\u{dca}නොගොව\u{dd3} පළ\u{dcf}ත"), ("sv", "Ömnögobi"), ("ta", "ஒம\u{bcd}நகோவி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓమ\u{c4d}న\u{c4b}గ\u{c4b}వ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดออมโนโกว\u{e34}"), ("tr", "Umnugobi"), ("uk", "Умнеговь"), ("ur", "اومنوگووی صوبہ"), ("vi", "Ömnögovi"), ("zh", "南戈壁省")]),
                        unofficial_name_list: ["Omnogobi"].to_vec(),
                    }
                ),
                (
                    "055",
                    Subdivision{
                        name: "Övörhangay",
                        country_alpha2: Alpha2::MN,
                        code: "055",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7624392), longitude: Some(103.0917032), max_latitude: Some(47.414307), min_latitude: Some(44.056532), max_longitude: Some(104.655877), min_longitude: Some(101.1333329)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوفوخاناجي"), ("az", "Övörxanqay vilayəti"), ("be", "Увэр-Хангайскі аймак"), ("bg", "Йовьорхангай"), ("bn", "ওভোরখ\u{9be}ঙ\u{9cd}গ\u{9be}ই প\u{9cd}রদেশ"), ("ca", "Província d’Övörkhangai"), ("ccp", "𑄃\u{1112e}𑄞\u{11127}𑄢\u{11134}𑄈𑄚\u{11134}𑄉\u{1112d}"), ("ceb", "Övörhangay Aymag"), ("da", "Övörkhangaj"), ("de", "Öwörchangai-Aimag"), ("el", "Οβορκχανγκάι"), ("en", "Övörkhangai"), ("es", "Övörhangay"), ("et", "Övörhangaj aimakk"), ("fa", "استان اوورخنگای"), ("fi", "Övörhangai"), ("fr", "Övörkhangai aïmag"), ("ga", "Övörkhangai"), ("gu", "ઓવોખ\u{a82}ગાઈ પ\u{acd}રા\u{a82}ત"), ("he", "אובורחאנגאי"), ("hi", "ओवोरख\u{93c}ानगई प\u{94d}रा\u{902}त"), ("hu", "Övörhangaj ajmag"), ("id", "Provinsi Övörkhangai"), ("it", "provincia del Ôvôrhangaj"), ("ja", "ウブルハンガイ県"), ("ka", "უვერხანგაი"), ("kn", "ಓವೊರ\u{ccd}ಖಂಗೈ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "으브르항가이 주"), ("lt", "Pietinio Changajaus aimakas"), ("lv", "Uvurhangaja aimaks"), ("mk", "Оворхангај"), ("mn", "Өвөрхангай"), ("mr", "ओवोरखा\u{902}गाई प\u{94d}रा\u{902}त"), ("ms", "Ovorkhangai Province"), ("nb", "Övörkhangaj"), ("nl", "Övörhangaj"), ("no", "Övörkhangaj"), ("pl", "Ajmak południowochangajski"), ("pt", "Övörkhangay"), ("ro", "Öwörchangai-Aimag"), ("ru", "Уверхангай"), ("si", "ඔවෝර\u{dca}ඛන\u{dca}ග\u{dcf}ය\u{dd2} පළ\u{dcf}ත"), ("sv", "Övörchangaj"), ("ta", "ஒவொர\u{bcd}க\u{bcd}ஹங\u{bcd}கை ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓవర\u{c4d}క\u{c3e}ంగ\u{c48} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโอเวอค\u{e31}นใจ"), ("tr", "Övörhangay"), ("uk", "Уверхангай"), ("ur", "اوورخانگائی صوبہ"), ("vi", "Övörkhangai"), ("zh", "前杭爱省")]),
                        unofficial_name_list: ["Ovorhangai", "Uvurkhangai"].to_vec(),
                    }
                ),
                (
                    "057",
                    Subdivision{
                        name: "Dzavhan",
                        country_alpha2: Alpha2::MN,
                        code: "057",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.2388147), longitude: Some(96.07030189999999), max_latitude: Some(50.041237), min_latitude: Some(46.583037), max_longitude: Some(99.2185871), min_longitude: Some(93.17390499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة زافخان"), ("bg", "Завхан"), ("bn", "জ\u{9be}বখ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de Zavkhan"), ("ccp", "𑄎𑄛\u{11134}𑄈𑄚\u{11134}"), ("ceb", "Dzavhan Aymag"), ("da", "Zavkhan"), ("de", "Dsawchan-Aimag"), ("el", "Ζαβκχάν"), ("en", "Zavkhan"), ("es", "Zavhan (aymag)"), ("et", "Dzavhani aimakk"), ("fa", "استان زوخان"), ("fi", "Zavhan"), ("fr", "Zavkhan"), ("gu", "ઝવખાન પ\u{acd}રા\u{a82}ત"), ("he", "זבחאן"), ("hi", "ज\u{93c}वख\u{93c}ान प\u{94d}रा\u{902}त"), ("hu", "Dzavhan tartomány"), ("id", "Provinsi Zavkhan"), ("it", "provincia del Zavhan"), ("ja", "ザブハン県"), ("ka", "ზავხანი"), ("kk", "Завхан аймағы"), ("kn", "ಜವಕನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "자브항 주"), ("lt", "Dzabchano aimakas"), ("lv", "Dzavhanas aimaks"), ("mk", "Завхан (покраина)"), ("mn", "Завхан"), ("mr", "झवखान प\u{94d}रा\u{902}त"), ("ms", "Zavkhan Province"), ("nb", "Zavkhan"), ("nl", "Zavhan"), ("no", "Zavkhan"), ("pl", "Ajmak dzawchański"), ("pt", "Zavkhan"), ("ro", "Zawchan-Aimag"), ("ru", "Завхан"), ("si", "සව\u{dca}ඛ\u{dcf}න\u{dca} [පළ\u{dcf}ත"), ("sv", "Dzavchan"), ("ta", "சவ\u{bcd}க\u{bbe}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c3e}వఖ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาวค\u{e4c}"), ("tr", "Zavhan"), ("uk", "Завхан"), ("ur", "زاوخان صوبہ"), ("vi", "Zavkhan (tỉnh)"), ("zh", "扎布汗省")]),
                        unofficial_name_list: ["Zavkhan"].to_vec(),
                    }
                ),
                (
                    "059",
                    Subdivision{
                        name: "Dundgovi",
                        country_alpha2: Alpha2::MN,
                        code: "059",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5822786), longitude: Some(106.7644209), max_latitude: Some(46.779331), min_latitude: Some(44.179281), max_longitude: Some(108.658953), min_longitude: Some(103.618634)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دوندغوفي"), ("bg", "Дунд Гови"), ("bn", "দ\u{9c1}ন\u{9cd}ডগোভি"), ("ca", "Província de Dundgovi"), ("ccp", "𑄓\u{1112a}𑄚\u{11133}𑄓\u{11134}𑄉\u{1112e}𑄞\u{11128}"), ("ceb", "Dundgovĭ Aymag"), ("da", "Dundgov"), ("de", "Dund-Gobi-Aimag"), ("el", "Ντουνγκόβι"), ("en", "Dundgovi"), ("es", "Dundgovi"), ("et", "Kesk-Gobi aimakk"), ("fa", "استان دوندگاوی"), ("fi", "Dundgovi"), ("fr", "Dundgovi"), ("ga", "Dundgovi"), ("gu", "ડ\u{ac1}ન\u{acd}ડગોવિ પ\u{acd}રા\u{a82}ત"), ("he", "דונדגובי"), ("hi", "द\u{941}न\u{94d}दगोवी प\u{94d}रा\u{902}त"), ("hu", "Közép-Góbi tartomány"), ("id", "Provinsi Dundgovi"), ("it", "provincia del Dundgov’"), ("ja", "ドンドゴビ県"), ("ka", "დუნდგოვი"), ("kn", "ಡುಂಡ\u{ccd}ಗೊವ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "동드고비 주"), ("lt", "Vidurinio Gobio aimakas"), ("lv", "Vidusgobi aimaks"), ("mk", "Дундгови"), ("mn", "Дундговь"), ("mr", "द\u{941}\u{902}डगोई प\u{94d}रा\u{902}त"), ("ms", "Dundgovi Province"), ("nb", "Dundgov"), ("nl", "Dundgovĭ"), ("no", "Dundgov"), ("pl", "Ajmak środkowogobijski"), ("pt", "Dundgovĭ"), ("ro", "Dund-Gobi-Aimag"), ("ru", "Дундговь"), ("si", "ඩන\u{dca}ඩ\u{dca}ගොව\u{dd2} පළ\u{dcf}ත"), ("sr", "Дундгови"), ("sr_Latn", "Dundgovi"), ("sv", "Dundgobi"), ("ta", "டுண\u{bcd}ட\u{bcd}கோவி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దుండ\u{c4d}గ\u{c4b}వ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดด\u{e38}นด\u{e4c}โกว\u{e34}"), ("tr", "Dundgobi"), ("uk", "Дундговь"), ("ur", "دوندگووی صوبہ"), ("vi", "Dundgovi"), ("zh", "中戈壁省")]),
                        unofficial_name_list: ["Dundgobi"].to_vec(),
                    }
                ),
                (
                    "061",
                    Subdivision{
                        name: "Dornod",
                        country_alpha2: Alpha2::MN,
                        code: "061",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.4658154), longitude: Some(115.3927119), max_latitude: Some(50.28290879999999), min_latitude: Some(46.23549999999999), max_longitude: Some(119.924301), min_longitude: Some(111.9632341)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دورنود"), ("bg", "Дорнод"), ("bn", "ডর\u{9cd}নড প\u{9cd}রদেশ"), ("ca", "Província de Dornod"), ("ccp", "𑄓\u{11127}𑄢\u{11134}𑄚\u{1112e}𑄖\u{11134}"), ("ceb", "Dornod Aymag"), ("cs", "Východní ajmag"), ("da", "Dornod"), ("de", "Dornod-Aimag"), ("el", "Ντορνόντ"), ("en", "Dornod"), ("es", "Dornod"), ("et", "Idaaimakk"), ("fa", "استان دارناد"), ("fi", "Dornod"), ("fr", "Dornod"), ("ga", "Dornod"), ("gu", "ડોર\u{acd}નોડ પ\u{acd}રા\u{a82}ત"), ("he", "דורנוד"), ("hi", "दोरनोद प\u{94d}रा\u{902}त"), ("hu", "Keleti tartomány (Mongólia)"), ("id", "Provinsi Dornod"), ("it", "provincia del Dornod"), ("ja", "ドルノド県"), ("ka", "დორნოდი"), ("kk", "Дорнод"), ("kn", "ಡೊರ\u{ccd}ನಾಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "더르너드 주"), ("lt", "Dornodo aimakas"), ("lv", "Dornodo province"), ("mk", "Дорнод"), ("mn", "Дорнод"), ("mr", "डोर\u{94d}नोड प\u{94d}रा\u{902}त"), ("ms", "Dornod Province"), ("nb", "Dornod"), ("nl", "Dornod"), ("no", "Dornod"), ("pl", "Ajmak wschodni"), ("pt", "Dornod"), ("ro", "Dornod-Aimag"), ("ru", "Дорнод"), ("si", "දොර\u{dca}නොඩ\u{dca} පළ\u{dcf}ත"), ("sr", "Дорнод"), ("sr_Latn", "Dornod"), ("sv", "Dornod"), ("ta", "டோர\u{bcd}னோட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c4b}ర\u{c4d}న\u{c4b}డ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดอร\u{e4c}นอด"), ("tr", "Dornod"), ("uk", "Дорнод"), ("ur", "دورنود صوبہ"), ("vi", "Dornod"), ("zh", "东方省")]),
                        unofficial_name_list: ["Dornod"].to_vec(),
                    }
                ),
                (
                    "063",
                    Subdivision{
                        name: "Dornogovi",
                        country_alpha2: Alpha2::MN,
                        code: "063",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9653889), longitude: Some(109.1773459), max_latitude: Some(46.62841299999999), min_latitude: Some(42.387329), max_longitude: Some(112.015564), min_longitude: Some(107.636619)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دورنوغوفي"), ("bg", "Дорно Гови"), ("bn", "ডর\u{9cd}নোগোবি প\u{9cd}রদেশ"), ("ca", "Dornogovi"), ("ccp", "𑄓\u{11127}𑄢𑄚\u{1112e}𑄉\u{1112e}𑄞\u{11128}"), ("ceb", "Dornogovĭ Aymag"), ("cs", "Východogobijský ajmag"), ("cy", "Talaith Dornogovi"), ("da", "Dornogov"), ("de", "Dorno-Gobi-Aimag"), ("el", "Ντορνογκόβι"), ("en", "Dornogovi"), ("es", "Dornogovi"), ("et", "Ida-Gobi aimakk"), ("fa", "استان دارناگاوی"), ("fi", "Dornogovi"), ("fr", "Dornogovi"), ("gu", "ડોર\u{acd}નોગોવી પ\u{acd}રા\u{a82}ત"), ("he", "דורנוגובי"), ("hi", "दोरनोगोवी प\u{94d}रा\u{902}त"), ("hu", "Kelet-Góbi tartomány"), ("id", "Provinsi Dornogovi"), ("it", "provincia del Dornogov’"), ("ja", "ドルノゴビ県"), ("ka", "დორნოგოვი"), ("kn", "ಡೊರ\u{ccd}ನೊಗೊವ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "더르너고비 주"), ("lt", "Rytų Gobio aimakas"), ("lv", "Austrumgobi aimaks"), ("mk", "Дорногови"), ("mn", "Дорноговь"), ("mr", "डॉर\u{94d}नोगोवी प\u{94d}रा\u{902}त"), ("ms", "Wilayah Dornogovi"), ("nb", "Dornogov"), ("nl", "Dornogovĭ"), ("no", "Dornogov"), ("pl", "Ajmak wschodniogobijski"), ("pt", "Dornogovĭ"), ("ro", "Dorno-Gobi-Aimag"), ("ru", "Дорноговь"), ("si", "ඩොර\u{dca}නෝගොව\u{dd2} පළ\u{dcf}ත"), ("sr", "Дорногови"), ("sr_Latn", "Dornogovi"), ("sv", "Dornogobi"), ("ta", "டொரினோகோவி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c4b}ర\u{c4d}న\u{c4b}గ\u{c4b}వ\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดอร\u{e4c}โนโกว\u{e35}"), ("tr", "Dornogobi"), ("uk", "Дорноговь"), ("ur", "دورنوگووی صوبہ"), ("vi", "Dornogovi"), ("zh", "东戈壁省")]),
                        unofficial_name_list: ["Dornogobi"].to_vec(),
                    }
                ),
                (
                    "064",
                    Subdivision{
                        name: "Govi-Sümber",
                        country_alpha2: Alpha2::MN,
                        code: "064",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4762754), longitude: Some(108.5570627), max_latitude: Some(46.98451), min_latitude: Some(45.906124), max_longitude: Some(109.0039898), min_longitude: Some(107.9454489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جوفيسومبر"), ("bg", "Гови Сумбер"), ("bn", "গোভিস\u{9c1}ম\u{9cd}ব\u{9be}র প\u{9cd}রদেশ"), ("ca", "Província de Govisümber"), ("ccp", "𑄉\u{1112e}𑄞\u{11128}𑄥\u{1112a}𑄟\u{11134}𑄝𑄢\u{11134}"), ("ceb", "Govĭ-Sumber"), ("da", "Gov-Sümber"), ("de", "Gobi-Sümber-Aimag"), ("el", "Γκοβισούμπερ"), ("en", "Govisümber"), ("es", "Govisümber"), ("et", "Gov-Sümberi aimakk"), ("fa", "استان گاوی\u{200c}سومبر"), ("fi", "Govisumber"), ("fr", "Govisümber"), ("ga", "Govisümber"), ("gu", "ગોવિસ\u{ac1}મ\u{acd}બર પ\u{acd}રા\u{a82}ત"), ("he", "גוביסומבר"), ("hi", "गोवीस\u{941}म\u{94d}ब\u{947}र प\u{94d}रा\u{902}त"), ("hu", "Góbi-Szümber ajmag"), ("id", "Provinsi Govisümber"), ("it", "provincia del Gov’-Sùmbėr"), ("ja", "ゴビスンベル県"), ("ka", "გობი-სუმბერი"), ("kn", "ಗೋವ\u{cbf}ಶಂಬರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "고비숨베르 주"), ("lt", "Gobisumbero aimakas"), ("lv", "Govisumberas aimaks"), ("mk", "Говисумбер"), ("mn", "Говьсүмбэр"), ("mr", "गोविस\u{94d}क\u{941}\u{902}ब\u{947}र प\u{94d}रा\u{902}त"), ("ms", "Govisumber Province"), ("nb", "Gov-Sümber"), ("nl", "Govĭsümber"), ("no", "Gov-Sümber"), ("pl", "Ajmak gobijsko-sumberski"), ("pt", "Govĭsümber"), ("ro", "Gobi-Sümber-Aimag"), ("ru", "Говь-Сумбэр"), ("si", "ගොව\u{dd2}සම\u{dca}බර\u{dca} පළ\u{dcf}ත"), ("sv", "Gobi-Sümber"), ("ta", "கொவிசும\u{bcd}பேர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4b}వ\u{c3f}సుంబర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกว\u{e34}ซ\u{e31}มเบอร\u{e4c}"), ("tr", "Gobisümber"), ("uk", "Говь-Сумбер"), ("ur", "گوویسمبر صوبہ"), ("vi", "Govisümber"), ("zh", "戈壁苏木贝尔省")]),
                        unofficial_name_list: ["Gobisumber"].to_vec(),
                    }
                ),
                (
                    "065",
                    Subdivision{
                        name: "Govi-Altay",
                        country_alpha2: Alpha2::MN,
                        code: "065",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.4511227), longitude: Some(95.8505766), max_latitude: Some(47.7823939), min_latitude: Some(42.70520399999999), max_longitude: Some(98.466612), min_longitude: Some(93.063152)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غوفي-ألتاي"), ("az", "Qovi-Altay vilayəti"), ("bg", "Гови Алтай"), ("bn", "গোভি-আল\u{9cd}ট\u{9be}ই প\u{9cd}রদেশ"), ("ca", "Província de Govi-Altai"), ("ccp", "𑄉\u{1112e}𑄞\u{11128}-𑄃𑄣\u{11134}𑄑\u{1112d}"), ("ceb", "Govĭ-Altay Aymag"), ("cy", "Talaith Govi-Altai"), ("da", "Gov-Altaj"), ("de", "Gobi-Altai-Aimag"), ("el", "Γκόβι-Αλτάι"), ("en", "Govi-Altai"), ("es", "Govi-Altay"), ("et", "Gobi Altai aimakk"), ("fa", "استان گاوی-آلتای"), ("fi", "Govi-Altai"), ("fr", "Govi-Altai"), ("gu", "ગોવિ-અલ\u{acd}તાઇ પ\u{acd}રા\u{a82}ત"), ("he", "גובי-אלטאי"), ("hi", "गोवी-अल\u{94d}ताई प\u{94d}रा\u{902}त"), ("hu", "Góbi-Altaj tartomány"), ("id", "Provinsi Govi-Altai"), ("it", "provincia del Gov’-Altaj"), ("ja", "ゴビ・アルタイ県"), ("ka", "გობი-ალთაი"), ("kk", "Говь Алтай аймағы"), ("kn", "ಗೋವ\u{cbf}-ಆಲ\u{ccd}ಟಾಯ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "고비알타이 주"), ("lt", "Gobio Altajaus aimakas"), ("lv", "Gobi Altaja aimaks"), ("mk", "Гови-Алтај"), ("mn", "Говь-Алтай"), ("mr", "गोवि-अल\u{94d}ताई प\u{94d}रा\u{902}त"), ("ms", "Govi-Altai Province"), ("nb", "Gov-Altaj"), ("nl", "Govĭ-Altaj"), ("no", "Gov-Altaj"), ("pl", "Ajmak gobijsko-ałtajski"), ("pt", "Govi-Altay"), ("ro", "Gobi-Altai-Aimag"), ("ru", "Говь-Алтай"), ("si", "ගොව\u{dd2}-අල\u{dca}ටය\u{dd2} පළ\u{dcf}ත"), ("sr", "Гови-Алтај"), ("sr_Latn", "Govi-Altaj"), ("sv", "Gobi-Altaj"), ("ta", "கோவி -அல\u{bcd}டை ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4b}వ\u{c3f}-అల\u{c4d}ట\u{c3e}య\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโกว\u{e34}-อ\u{e31}ลไต"), ("tr", "Gobi-Altay"), ("uk", "Говь-Алтай"), ("ur", "گووی-التائی صوبہ"), ("vi", "Govi-Altai"), ("zh", "戈壁阿尔泰省")]),
                        unofficial_name_list: ["Gobi-Altai", "Gobi-Altay"].to_vec(),
                    }
                ),
                (
                    "067",
                    Subdivision{
                        name: "Bulgan",
                        country_alpha2: Alpha2::MN,
                        code: "067",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.811944), longitude: Some(103.533611), max_latitude: Some(48.8664087), min_latitude: Some(48.7746887), max_longitude: Some(103.587513), min_longitude: Some(103.4531021)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بولغان"), ("bg", "Булган"), ("bn", "ব\u{9c1}ল\u{9cd}গ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "Província de Bulgan"), ("ccp", "𑄝\u{1112a}𑄣\u{11134}𑄉𑄚\u{11134}"), ("ceb", "Bulgan"), ("da", "Bulgan"), ("de", "Bulgan-Aimag"), ("el", "Μπούλγκαν"), ("en", "Bulgan"), ("es", "Bulgan"), ("et", "Bulgani aimakk"), ("fa", "استان بولگن"), ("fi", "Bulgan"), ("fr", "Bulgan"), ("ga", "Bulgan"), ("gu", "બ\u{ac1}લ\u{acd}ગન પ\u{acd}રા\u{a82}ત"), ("he", "בולגאן"), ("hi", "ब\u{941}ल\u{94d}गन प\u{94d}रा\u{902}त"), ("hu", "Bulgan tartomány"), ("id", "Provinsi Bulgan"), ("it", "provincia di Bulgan"), ("ja", "ボルガン県"), ("ka", "ბულგანის აიმაკი"), ("kn", "ಬುಲ\u{ccd}ಗನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "볼간 주"), ("lt", "Bulgano aimakas"), ("lv", "Bulganas aimaks"), ("mk", "Булган"), ("mn", "Булган"), ("mr", "ब\u{941}ल\u{94d}गान प\u{94d}रा\u{902}त"), ("ms", "Bulgan Province"), ("nb", "Bulgan"), ("nl", "Ajmag Bulgan"), ("no", "Bulgan"), ("pl", "Ajmak bulgański"), ("pt", "Bulgan"), ("ro", "Bulgan-Aimag"), ("ru", "Булган"), ("si", "බ\u{dd4}ල\u{dca}ග\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sr", "Булган"), ("sr_Latn", "Bulgan"), ("sv", "Bulgan"), ("ta", "புல\u{bcd}கண\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుల\u{c4d}గ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}ลแกน"), ("tr", "Bulgan"), ("uk", "Булган"), ("ur", "بلگان صوبہ"), ("vi", "Bulgan"), ("zh", "布尔干省")]),
                        unofficial_name_list: ["Bulgan"].to_vec(),
                    }
                ),
                (
                    "069",
                    Subdivision{
                        name: "Bayanhongor",
                        country_alpha2: Alpha2::MN,
                        code: "069",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.191667), longitude: Some(100.717778), max_latitude: Some(46.2142884), min_latitude: Some(46.1453508), max_longitude: Some(100.7368184), min_longitude: Some(100.6864358)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بايانخونغور"), ("az", "Bayanxonqor vilayəti"), ("be", "Баян-Хангорскі аймак"), ("bg", "Баянхонгор"), ("bn", "ব\u{9be}য\u{9bc}\u{9be}নকোংওর প\u{9cd}রদেশ"), ("ca", "Província de Baiankhongor"), ("ccp", "𑄝𑄬𑄠𑄚\u{11134}𑄈\u{11127}\u{11101}𑄉\u{11127}𑄢\u{11134}"), ("ceb", "Bayanhongor Aymag"), ("da", "Bajankhongor"), ("de", "Bajanchongor-Aimag"), ("el", "Μπαγιανκχονγκόρ"), ("en", "Bayankhongor"), ("es", "Bayanhongor"), ("et", "Bajanhongori aimakk"), ("fa", "استان بایان خونگور"), ("fi", "Bajanhongor"), ("fr", "Bayanhongor"), ("gu", "બાય\u{a82}ખો\u{a82}ગોર પ\u{acd}રા\u{a82}ત"), ("he", "באיינחונגור"), ("hi", "बयानख\u{93c}ो\u{902}गोर प\u{94d}रा\u{902}त"), ("hu", "Bajanhongor tartomány"), ("id", "Provinsi Bayankhongor"), ("it", "provincia di Bajanhongor"), ("ja", "バヤンホンゴル県"), ("ka", "ბაიანხონგორის აიმაკი"), ("kn", "ಬೇಯಾನ\u{ccd}ಖೊಂಗೋರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바잉헝거르 주"), ("lt", "Bajanchongoro aimakas"), ("lv", "Bajanhongoras aimaks"), ("mk", "Бајанхонгор"), ("mn", "Баянхонгор"), ("mr", "बायानखो\u{902}गोर प\u{94d}रा\u{902}त"), ("ms", "Bayankhongor Province"), ("nb", "Bajankhongor"), ("nl", "Bajanhongor"), ("no", "Bajankhongor"), ("pl", "Ajmak bajanchongorski"), ("pt", "Bayankhongor"), ("ro", "Bajanchongor-Aimag"), ("ru", "Баянхонгор"), ("si", "බයන\u{dca}කොන\u{dca}ඝෝර\u{dca} පළ\u{dcf}ත"), ("sr", "Бајанхонгор"), ("sr_Latn", "Bajanhongor"), ("sv", "Bajanchongor"), ("ta", "ப\u{bbe}ய\u{bcd}நக\u{bcd}ஹோங\u{bcd}கோர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బయ\u{c3e}న\u{c4d}క\u{c4b}ంగర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาย\u{e31}นคนกอร\u{e4c}"), ("tr", "Bayanhongor"), ("uk", "Баянхонгор"), ("ur", "بیانخونگور صوبہ"), ("vi", "Bayankhongor"), ("zh", "巴彦洪戈尔省")]),
                        unofficial_name_list: ["Bayanhongor", "Bayankhongor"].to_vec(),
                    }
                ),
                (
                    "071",
                    Subdivision{
                        name: "Bayan-Ölgiy",
                        country_alpha2: Alpha2::MN,
                        code: "071",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.3983254), longitude: Some(89.66259149999999), max_latitude: Some(50.009587), min_latitude: Some(46.544199), max_longitude: Some(91.9268699), min_longitude: Some(87.74966409999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بايان-أولجي"), ("az", "Bayan-Ölgiy"), ("bg", "Баян Йолгий"), ("bn", "ব\u{9be}য\u{9bc}\u{9be}ন-ওলগি প\u{9cd}রদেশ"), ("ca", "Província de Bayan-Ölgii"), ("ccp", "𑄝𑄬𑄠𑄚\u{11134}-𑄃𑄣\u{11134}𑄎\u{11133}𑄦\u{11128}"), ("ceb", "Bayan-Ölgiy Aymag"), ("da", "Bajan-Ölgij"), ("de", "Bajan-Ölgii-Aimag"), ("el", "Μπαγιάν Ολγκίι"), ("en", "Bayan-Ölgii"), ("es", "Bayan-Ölgiy"), ("et", "Bajan-Ölgij aimakk"), ("fa", "استان بایان-اولگی"), ("fi", "Bajan-Ölgii"), ("fr", "Bayan-Ölgii"), ("ga", "Bayan-Ölgii"), ("gu", "બાયન-ઓલ\u{acd}ગી પ\u{acd}રા\u{a82}ત"), ("he", "באיין-אולגי"), ("hi", "बयान-ओलगी प\u{94d}रा\u{902}त"), ("hu", "Bajan-Ölgij tartomány"), ("id", "Provinsi Bayan-Ölgii"), ("it", "provincia del Bajan-Ôlgij"), ("ja", "バヤン・ウルギー県"), ("ka", "ბაიან-ულგიი"), ("kk", "Баян-Өлгей аймағы"), ("kn", "ಬೇಯಾನ\u{ccd}-ಒಲ\u{ccd}ಗ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바잉울기 주"), ("lt", "Bajan Ulegėjaus aimakas"), ("lv", "Bajanulgī aimaks"), ("mk", "Бајан-Олги"), ("mn", "Баян-Өлгий"), ("mr", "बायान-ऑल\u{94d}जी प\u{94d}रा\u{902}त"), ("ms", "Bayan-Olgii Province"), ("nb", "Bajan-Ölgij"), ("nl", "Bajan-Ölgi"), ("no", "Bajan-Ölgij"), ("pl", "Ajmak bajanolgijski"), ("pt", "Bayan-Ölgiy"), ("ro", "Bajan-Ölgii-Aimag"), ("ru", "Баян-Улгий"), ("si", "බය\u{dcf}න\u{dca}-ඔල\u{dca}ග\u{dd2} පළ\u{dcf}ත"), ("sr", "Бајан-Улгиј"), ("sr_Latn", "Bajan-Ulgij"), ("sv", "Bajan-Ölgij"), ("ta", "ப\u{bbe}ய\u{bbe}ன\u{bcd} -ஒல\u{bcd}கிய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బయ\u{c3e}న\u{c4d}-ఓల\u{c4d}గ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาย\u{e31}น-เอ\u{e34}ลก\u{e35}"), ("tr", "Bayan Ölgii ili"), ("uk", "Баян-Улгий"), ("ur", "بیان-اولگی صوبہ"), ("vi", "Bayan-Ölgii"), ("zh", "巴彦乌列盖省")]),
                        unofficial_name_list: ["Bayan-Olgii", "Bayan-Ulgii"].to_vec(),
                    }
                ),
                (
                    "073",
                    Subdivision{
                        name: "Arhangay",
                        country_alpha2: Alpha2::MN,
                        code: "073",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.8971101), longitude: Some(100.7240165), max_latitude: Some(49.1983439), min_latitude: Some(46.826294), max_longitude: Some(103.673478), min_longitude: Some(98.166168)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أرخانغاي"), ("az", "Arxanqay"), ("be", "Ара-Хангайскі аймак"), ("bg", "Архангай"), ("bn", "আর\u{9cd}খঙ\u{9cd}গ\u{9be}ই প\u{9cd}রদেশ"), ("ca", "Arkhangai"), ("ccp", "𑄃𑄢\u{11134}𑄈𑄚\u{11134}𑄉\u{1112d}"), ("ceb", "Arhangay Aymag"), ("cs", "Severochangajský ajmag"), ("da", "Arkhangaj"), ("de", "Archangai-Aimag"), ("el", "Αρκανγκάι"), ("en", "Arkhangai"), ("es", "Arhangay"), ("et", "Arhangaj aimakk"), ("fa", "استان آرخانگای"), ("fi", "Arkhangai"), ("fr", "Arkhangai"), ("ga", "Arkhangai"), ("gu", "અર\u{acd}ખ\u{a82}ગાઈ પ\u{acd}રા\u{a82}ત"), ("he", "ארחאנגאי"), ("hi", "अरख\u{93c}ानगई प\u{94d}रा\u{902}त"), ("hu", "Észak-Hangáj tartomány"), ("id", "Provinsi Arkhangai"), ("it", "provincia dell’Arhangaj"), ("ja", "アルハンガイ県"), ("ka", "არახანგაის აიმაკი"), ("kn", "ಅರ\u{ccd}ಖಂಗೈ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아르항가이 주"), ("lt", "Archangajaus aimakas"), ("lv", "Arhangaja aimaks"), ("mk", "Архангај"), ("mn", "Архангай"), ("mr", "अर\u{94d}खा\u{902}गई प\u{94d}रा\u{902}त"), ("ms", "Arkhangai Province"), ("nb", "Arkhangaj"), ("nl", "Arhangaj"), ("no", "Arkhangaj"), ("pl", "Ajmak północnochangajski"), ("pt", "Arkhangay"), ("ro", "Archangai-Aimag"), ("ru", "Архангай"), ("si", "අර\u{dca}ක\u{dca}ගන\u{dca}ගය\u{dd2} පළ\u{dcf}ත"), ("sr", "Архангај"), ("sr_Latn", "Arhangaj"), ("sv", "Archangaj"), ("ta", "ஆர\u{bcd}க\u{bcd}கன\u{bcd}க\u{bbe}ய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అర\u{c4d}క\u{c3e}ంగ\u{c48} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ฮ\u{e31}งไก"), ("tr", "Arhangay"), ("uk", "Архангай"), ("ur", "ارخانگائی صوبہ"), ("vi", "Arkhangai"), ("zh", "后杭爱省")]),
                        unofficial_name_list: ["Arhangai", "Arhangay", "Arkhangai"].to_vec(),
                    }
                ),
                (
                    "1",
                    Subdivision{
                        name: "Ulaanbaatar",
                        country_alpha2: Alpha2::MN,
                        code: "1",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.91999999999999), longitude: Some(106.92), max_latitude: Some(47.9591229), min_latitude: Some(47.8241773), max_longitude: Some(107.1043), min_longitude: Some(106.6994427)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ulaanbaatar"), ("am", "ኡላዓን ባዓታር"), ("ar", "أولان باتور"), ("az", "Ulan-Bator"), ("be", "Улан-Батар"), ("bg", "Улан Батор"), ("bn", "উল\u{9be}নব\u{9be}টর"), ("bs", "Ulan Bator"), ("ca", "Ulan Bator"), ("ccp", "𑄅\u{1112a}𑄣𑄚\u{11134}𑄝𑄑𑄢\u{11134}"), ("ceb", "Ulaanbaatar Hot"), ("cs", "Ulánbátar"), ("cy", "Ulan Bator"), ("da", "Ulan Bator"), ("de", "Ulaanbaatar"), ("el", "Ουλάν Μπατόρ"), ("en", "Ulaanbaatar"), ("es", "Ulán Bator"), ("et", "Ulaanbaatar"), ("eu", "Ulan Bator"), ("fa", "اولان\u{200c}باتور"), ("fi", "Ulan Bator"), ("fr", "Oulan-Bator"), ("ga", "Ulánbátar"), ("gl", "Ulán Bátor"), ("gu", "ઉલાનબાતર"), ("he", "אולן בטור"), ("hi", "उलान बतोर"), ("hr", "Ulan Bator"), ("hu", "Ulánbátor"), ("hy", "Ուլան Բատոր"), ("id", "Ulan Bator"), ("is", "Úlan Bator"), ("it", "Ulan Bator"), ("ja", "ウランバートル"), ("jv", "Ulaanbaatar"), ("ka", "ულან-ბატორი"), ("kk", "Ұланбатыр"), ("km", "អ\u{17ca}\u{17bc}លេនបាធរ"), ("kn", "ಉಲಾನ\u{ccd} ಬತೊರ\u{ccd}"), ("ko", "울란바토르"), ("ky", "Улан-Баатыр"), ("lt", "Ulan Batoras"), ("lv", "Ulanbatora"), ("mk", "Улан Батор"), ("ml", "ഉല\u{d3e}ൻബ\u{d3e}ത\u{d3e}ർ"), ("mn", "Улаанбаатар"), ("mr", "उलानबातर"), ("ms", "Ulan Bator"), ("my", "ဦလန\u{103a}ဘာတာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Ulan Bator"), ("ne", "उलानबटोर"), ("nl", "Ulaanbaatar"), ("no", "Ulan Bator"), ("pa", "ਉਲਾਨ ਬਤ\u{a4b}ਰ"), ("pl", "Ułan Bator"), ("pt", "Ulan Bator"), ("ro", "Ulaanbaatar"), ("ru", "Улан-Батор"), ("si", "ඌෆ\u{dcf}"), ("sk", "Ulanbátar"), ("sl", "Ulan Bator"), ("so", "Ulan Bator"), ("sq", "Ulan Bator"), ("sr", "Улан Батор"), ("sr_Latn", "Ulan Bator"), ("sv", "Ulan Bator"), ("sw", "Ulaanbaatar"), ("ta", "ஊல\u{bbe}ன\u{bcd} ப\u{bbe}டோர\u{bcd}"), ("te", "ఊల\u{c3e}న\u{c4d} బట\u{c4b}ర\u{c4d}"), ("th", "อ\u{e39}ลานบาตาร\u{e4c}"), ("tk", "Ulan-Bator"), ("tr", "Ulan Batur"), ("uk", "Улан-Батор"), ("ur", "اولان\u{200c} باتور"), ("uz", "Ulan-Bator"), ("vi", "Ulaanbaatar"), ("yo", "Ulan Bator"), ("yo_BJ", "Ulan Bator"), ("yue", "烏蘭巴托"), ("yue_Hans", "乌兰巴托"), ("zh", "乌兰巴托")]),
                        unofficial_name_list: ["Ulaanbaatar"].to_vec(),
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
#[cfg(feature = "mn")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::MN,
        alpha3: Alpha3::MNG,
        address_format: None,
        continent: Continent::Asia,
        country_code: 976,
        currency_code: CurrencyCode::MNT,
        gec: Some(GEC::MG),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "001",
        ioc: Some(IOC::MGL),
        iso_long_name: "Mongolia",
        iso_short_name: "Mongolia",
        official_language_list: ["mn"].to_vec(),
        spoken_language_list: ["mn"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Mongolian"),
        number: "496",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "MN",
        unofficial_name_list: ["Mongolia", "Mongolei", "Mongolie", "モンゴル", "Mongolië"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Mongolia"), ("af", "Mongolië"), ("ak", "Mongolia"), ("am", "ሥንጕሑ።"), ("an", "Mongolia"), ("ar", "منغوليا"), ("as", "মঙ\u{9cd}গোলিয়\u{9be}"), ("ay", "Mongolia"), ("az", "Monqolustan"), ("ba", "Mongolia"), ("be", "Манголія"), ("bg", "Монголия"), ("bi", "Mongolia"), ("bn", "মঙ\u{9cd}গোলিয়\u{9be}"), ("bn_IN", "মঙ\u{9cd}গোলিয়\u{9be}"), ("br", "Mongolia"), ("bs", "Mongolija"), ("ca", "Mongòlia"), ("ce", "Монголи"), ("ch", "Mongolia"), ("cs", "Mongolsko"), ("cv", "Монголи"), ("cy", "Mongolia"), ("da", "Mongoliet"), ("de", "Mongolei"), ("dv", "މ\u{7ae}ނ\u{7b0}ގ\u{7af}ލ\u{7a8}އ\u{7a7}"), ("dz", "མ\u{f7c}ང་ག\u{f7c}་ལ\u{f72}་ཡ།"), ("ee", "Mongolia"), ("el", "Μογγολία"), ("en", "Mongolia"), ("eo", "Mongolio"), ("es", "Mongolia"), ("et", "Mongoolia"), ("eu", "Mongolia"), ("fa", "مغولستان"), ("ff", "Mongolia"), ("fi", "Mongolia"), ("fo", "Mongolia"), ("fr", "Mongolie"), ("fy", "Mongoalje"), ("ga", "An Mhongóil"), ("gl", "Mongolia"), ("gn", "Mongolia"), ("gu", "મો\u{a82}ગોલિયા"), ("gv", "Yn Vongoil"), ("ha", "Mangolia"), ("he", "מונגוליה"), ("hi", "म\u{902}गोलिया"), ("hr", "Mongolija"), ("ht", "Mongoli"), ("hu", "Mongólia"), ("hy", "Մոնղոլիա"), ("ia", "Mongolia"), ("id", "Mongolia"), ("io", "Mongolia"), ("is", "Mongólía"), ("it", "Mongolia"), ("iu", "Mongolia"), ("ja", "モンゴル国"), ("ka", "მონღოლეთი"), ("ki", "Mongolia"), ("kk", "Монғолия"), ("kl", "Mongolia"), ("km", "ម\u{17c9}\u{17bb}ងហ\u{17d2}គោល\u{17b8}"), ("kn", "ಮಂಗೋಲ\u{cbf}ಯಾ"), ("ko", "몽골"), ("ku", "Mongolya"), ("kv", "Монголия"), ("kw", "Mongoli"), ("ky", "Моңголстан"), ("lo", "Mongolia"), ("lt", "Mongolija"), ("lv", "Mongolija"), ("mi", "Mongōria"), ("mk", "Монголија"), ("ml", "മംഗോളിയ"), ("mn", "Монгол"), ("mr", "म\u{902}गोलिया"), ("ms", "Mongolia"), ("mt", "Mongolja"), ("my", "မ\u{103d}န\u{103a}ဂ\u{102d}\u{102f}းလ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Mongoriya"), ("nb", "Mongolia"), ("ne", "मङ\u{94d}गोलिया"), ("nl", "Mongolië"), ("nn", "Mongolia"), ("nv", "Chʼah Diʼilii Bikéyah"), ("oc", "Mongolia"), ("or", "ମଙ\u{b4d}ଗୋଲ\u{b3f}ୟ\u{b3e}"), ("pa", "ਮ\u{a70}ਗ\u{a4b}ਲੀਆ"), ("pi", "म\u{902}गोलिया"), ("pl", "Mongolia"), ("ps", "مغولستان"), ("pt", "Mongólia"), ("pt_BR", "Mongólia"), ("ro", "Mongolia"), ("ru", "Монголия"), ("rw", "Mongoliya"), ("sc", "Mongòlia"), ("sd", "Mongolia"), ("si", "මොංගෝල\u{dd2}ය\u{dcf}ව"), ("sk", "Mongolsko"), ("sl", "Mongolija"), ("so", "Mongolia"), ("sq", "Mongoli"), ("sr", "Монголија"), ("sv", "Mongoliet"), ("sw", "Mongolia"), ("ta", "மங\u{bcd}கோலிய\u{bbe}"), ("te", "మంగ\u{c4b}ల\u{c3f}య\u{c3e}"), ("tg", "Муғулистон"), ("th", "มองโกเล\u{e35}ย"), ("ti", "ሞንጎልያ"), ("tk", "Mongoliýa"), ("tl", "Mongolia"), ("tr", "Moğolistan"), ("tt", "Моголстан"), ("ug", "موڭغۇلىيە"), ("uk", "Монголія"), ("ur", "منگولیا"), ("uz", "Moʻgʻuliston"), ("ve", "Mongolia"), ("vi", "Mông Cổ"), ("wa", "Mongoleye"), ("wo", "Moŋgooli"), ("xh", "Mongolia"), ("yo", "Mòngólíà"), ("zh_CN", "蒙古"), ("zh_HK", "蒙古"), ("zh_TW", "蒙古"), ("zu", "Mongolia")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
