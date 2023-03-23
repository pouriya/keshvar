// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Slovenia

#[cfg(all(feature = "si", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SI;
    pub const ALPHA3: Alpha3 = Alpha3::SVN;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 386;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::SI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("SLO");
    pub const ISO_SHORT_NAME: &str = "Slovenia";
    pub const ISO_LONG_NAME: &str = "The Republic of Slovenia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["sl"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["sl"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Slovene");
    pub const NUMBER: &str = "705";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "SI";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Slovenia",
        "Slowenien",
        "Slovénie",
        "Eslovenia",
        "スロベニア",
        "Slovenië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Slovenia"),
        ("af", "Slowenië"),
        ("ak", "Slovenia"),
        ("am", "ስሕታኒ።"),
        ("an", "Slovenia"),
        ("ar", "سلوفينيا"),
        ("as", "স\u{9cd}লোভেনিয়\u{9be}"),
        ("ay", "Slovenia"),
        ("az", "Sloveniya"),
        ("ba", "Slovenia"),
        ("be", "Славенія"),
        ("bg", "Словения"),
        ("bi", "Slovenia"),
        ("bn", "স\u{9cd}লোভেনিয়\u{9be}"),
        ("bn_IN", "স\u{9cd}লোভেনিয়\u{9be}"),
        ("br", "Slovenia"),
        ("bs", "Slovenija"),
        ("ca", "Eslovènia"),
        ("ce", "Словени"),
        ("ch", "Slovenia"),
        ("cs", "Slovinsko"),
        ("cv", "Словени"),
        ("cy", "Slofenia"),
        ("da", "Slovenien"),
        ("de", "Slowenien"),
        ("dv", "ސ\u{7aa}ލ\u{7ae}ވ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "ས\u{f7c}ལ\u{f7c}་བ\u{f72}་ན\u{f72}་ཡ།"),
        ("ee", "Slovenia"),
        ("el", "Σλοβενία"),
        ("en", "Slovenia"),
        ("eo", "Slovenio"),
        ("es", "Eslovenia"),
        ("et", "Sloveenia"),
        ("eu", "Eslovenia"),
        ("fa", "اسلوونی"),
        ("ff", "Suloweniya"),
        ("fi", "Slovenia"),
        ("fo", "Slovenia"),
        ("fr", "Slovénie"),
        ("fy", "Sloveenje"),
        ("ga", "An tSlóivéin"),
        ("gl", "Eslovenia"),
        ("gn", "Slovenia"),
        ("gu", "સ\u{acd}લોવ\u{ac7}નિયા"),
        ("gv", "Yn Clovean"),
        ("ha", "Sloveniya"),
        ("he", "סלובניה"),
        ("hi", "स\u{94d}लोव\u{947}निया"),
        ("hr", "Slovenija"),
        ("ht", "Sloveni"),
        ("hu", "Szlovénia"),
        ("hy", "Սլովենիա"),
        ("ia", "Slovenia"),
        ("id", "Slovenia"),
        ("io", "Slovenia"),
        ("is", "Slóvenía"),
        ("it", "Slovenia"),
        ("iu", "Slovenia"),
        ("ja", "スロベニア"),
        ("ka", "სლოვენია"),
        ("ki", "Slovenia"),
        ("kk", "Словения"),
        ("kl", "Slovenia"),
        ("km", "ស\u{17d2}ល\u{17bc}វ\u{17c9}ាន\u{17b8}"),
        ("kn", "ಸ\u{ccd}ಲೋವೇನ\u{cbf}ಯಾ"),
        ("ko", "슬로베니아"),
        ("ku", "Slovenya"),
        ("kv", "Словения"),
        ("kw", "Sloveni"),
        ("ky", "Словения"),
        ("lo", "ປະເທດສະໂລເວນ\u{eb5}"),
        ("lt", "Slovėnija"),
        ("lv", "Slovēnija"),
        ("mi", "Horowinia"),
        ("mk", "Словенија"),
        ("ml", "സ\u{d4d}ലോവേനിയ"),
        ("mn", "Словени"),
        ("mr", "स\u{94d}लोव\u{94d}ह\u{947}निया"),
        ("ms", "Slovenia"),
        ("mt", "Slovenja"),
        (
            "my",
            "ဆလ\u{102d}\u{102f}ဗေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Tsirobeniya"),
        ("nb", "Slovenia"),
        ("ne", "स\u{94d}लोभ\u{947}निया"),
        ("nl", "Slovenië"),
        ("nn", "Slovenia"),
        ("nv", "Słobíín Bikéyah"),
        ("oc", "Eslovènia"),
        ("or", "ସ\u{b4d}ଲୋଭ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਸਲ\u{a4b}ਵੀਨੀਆ"),
        ("pi", "स\u{94d}लोवीनिया"),
        ("pl", "Słowenia"),
        ("ps", "سلووانیا"),
        ("pt", "Eslovénia"),
        ("pt_BR", "Eslovênia"),
        ("ro", "Slovenia"),
        ("ru", "Словения"),
        ("rw", "Siloveniya"),
        ("sc", "Islovènia"),
        ("sd", "Slovenia"),
        ("si", "ස\u{dca}ලෝව\u{dd3}න\u{dd2}ය\u{dcf}ව"),
        ("sk", "Slovinsko"),
        ("sl", "Slovenija"),
        ("so", "Slovenia"),
        ("sq", "Slloveni"),
        ("sr", "Словенија"),
        ("sv", "Slovenien"),
        ("sw", "Slovenia"),
        ("ta", "ஸ\u{bcd}லோவேனிய\u{bbe}"),
        ("te", "స\u{c4d}ల\u{c4b}వ\u{c47}న\u{c3f}య\u{c3e}"),
        ("tg", "Словения"),
        ("th", "สโลว\u{e35}เน\u{e35}ย"),
        ("ti", "ስሎቬኒያ"),
        ("tk", "Sloweniýa"),
        ("tl", "Slovenia"),
        ("tr", "Slovenya"),
        ("tt", "Словениа"),
        ("ug", "سىلوۋېنىيە"),
        ("uk", "Словенія"),
        ("ur", "سلووینیا"),
        ("uz", "Sloveniya"),
        ("ve", "Slovenia"),
        ("vi", "Xlô-ven"),
        ("wa", "Esloveneye"),
        ("wo", "Esloweeni"),
        ("xh", "Slovenia"),
        ("yo", "Sloféníà"),
        ("zh_CN", "斯洛文尼亚"),
        ("zh_HK", "斯洛文尼亞"),
        ("zh_TW", "斯洛維尼亞"),
        ("zu", "ISloveniya"),
    ];
    #[cfg(all(feature = "si", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 46.151241;
        pub const LONGITUDE: f64 = 14.995463;
        pub const MAX_LATITUDE: f64 = 46.876659;
        pub const MAX_LONGITUDE: f64 = 16.6107038;
        pub const MIN_LATITUDE: f64 = 45.4218356;
        pub const MIN_LONGITUDE: f64 = 13.3753355;
        pub const NORTHEAST_LATITUDE: f64 = 46.876659;
        pub const NORTHEAST_LONGITUDE: f64 = 16.6107038;
        pub const SOUTHWEST_LATITUDE: f64 = 45.4218356;
        pub const SOUTHWEST_LONGITUDE: f64 = 13.3753355;
    }
}
#[cfg(all(feature = "si", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 46.151241,
            longitude: 14.995463,
            max_latitude: 46.876659,
            max_longitude: 16.6107038,
            min_latitude: 45.4218356,
            min_longitude: 13.3753355,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 46.876659,
                    longitude: 16.6107038,
                },
                southwest: CountryGeoBound {
                    latitude: 45.4218356,
                    longitude: 13.3753355,
                },
            },
        }
    }
}

#[cfg(all(feature = "si", feature = "subdivisions"))]
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
                    "001",
                    Subdivision{
                        name: "001",
                        country_alpha2: Alpha2::SI,
                        code: "001",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8869025), longitude: Some(13.909914), max_latitude: Some(45.9068796), min_latitude: Some(45.8652066), max_longitude: Some(13.9250956), min_longitude: Some(13.8808621)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية آيدوشتشينا"), ("bn", "আজদোভসিন\u{9be} পৌরসভ\u{9be}"), ("bs", "Ajdovščina"), ("ca", "Ajdovščina"), ("ccp", "𑄃𑄌\u{11134}𑄓\u{1112e}𑄛\u{11134}𑄥\u{11128}𑄚"), ("ceb", "Občina Ajdovščina (munisipyo sa Eslobenya)"), ("cs", "Občina Ajdovščina"), ("da", "Ajdovščina Municipality"), ("de", "Ajdovščina"), ("el", "Αζντοβστσίνα"), ("en", "Ajdovščina"), ("es", "Municipio de Ajdovščina"), ("fi", "Ajdovščinan kunta"), ("fr", "Ajdovščina"), ("gu", "અજડોવસીના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "एडोव\u{94d}सिना नगरपालिका"), ("hr", "Općina Ajdovščina"), ("hu", "Ajdovščina"), ("hy", "Այդովշչինա"), ("id", "Ajdovščina"), ("it", "Aidussina"), ("ja", "アイドフシュチナ"), ("kn", "ಅಜ\u{ccd}ಡೊವ\u{ccd}ಸ\u{ccd}ಕ\u{cbf}ನಾ ಪುರಸಭ\u{cc6}"), ("ko", "아이도브슈치나"), ("lt", "Aidovščina"), ("lv", "Ajdovščinas pašvaldība"), ("mk", "Ајдовшчина"), ("mr", "अजोडोस\u{94d}चिना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ajdovscina Municipality"), ("nb", "Ajdovscina kommune"), ("nl", "Ajdovščina"), ("no", "Ajdovscina kommune"), ("pl", "Gmina Ajdovščina"), ("pt", "Ajdovščina"), ("ro", "Ajdovščina"), ("ru", "Айдовшчина"), ("si", "අජ\u{dca}ඩොව\u{dca}ස\u{dca}ක\u{dd2}න\u{dcf} නගර සභ\u{dcf}ව"), ("sk", "Ajdovščina"), ("sl", "Občina Ajdovščina"), ("sr", "Општина Ајдовшчина"), ("sr_Latn", "Opština Ajdovščina"), ("sv", "Ajdovscina"), ("ta", "அஜிடோவ\u{bcd}சின\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "అజ\u{c4d}డ\u{c4b}వస\u{c4d}క\u{c3f}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องไอย\u{e4c}โดชช\u{e34}นา"), ("tr", "Ajdovščina"), ("uk", "Айдовщина"), ("ur", "بلدیہ آیدووشچینا"), ("vi", "Ajdovščina"), ("zh", "阿伊多夫什契纳")]),
                        unofficial_name_list: ["Ajdovšcina"].to_vec(),
                    }
                ),
                (
                    "002",
                    Subdivision{
                        name: "002",
                        country_alpha2: Alpha2::SI,
                        code: "002",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.60791529999999), longitude: Some(16.2365127), max_latitude: Some(46.6263849), min_latitude: Some(46.5791012), max_longitude: Some(16.2670594), min_longitude: Some(16.2038046)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيلتينتسي"), ("bn", "বেল\u{9cd}টিঙ\u{9cd}কি পৌরসভ\u{9be}"), ("ccp", "𑄝𑄬𑄣\u{11134}𑄑\u{11128}𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Beltinci (munisipyo)"), ("cs", "Občina Beltinci"), ("da", "Beltinci Municipality"), ("de", "Gemeinde Beltinci"), ("el", "Μπελτίνκι"), ("en", "Beltinci"), ("es", "Municipio de Beltinci"), ("fi", "Beltincu"), ("fr", "Municipalité de Beltinci"), ("gu", "બ\u{ac7}લ\u{acd}ટિન\u{acd}ચિ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{947}ल\u{94d}टि\u{902}की नगर पालिका"), ("hr", "Općina Beltinci"), ("hu", "Belatinc község"), ("id", "Kotamadya Beltinci"), ("it", "Comune di Beltinci"), ("ja", "ベルティンツィ"), ("kn", "ಬ\u{cc6}ಲ\u{ccd}ಟನ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "벨틴치 지방 자치제"), ("lt", "Beltincio savivaldybė"), ("lv", "Beltincu pašvaldība"), ("mr", "ब\u{947}लटी\u{902}ची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Beltinci Municipality"), ("nb", "Beltinci kommune"), ("nl", "Beltinci"), ("no", "Beltinci kommune"), ("pl", "Gmina Beltinci"), ("pt", "Município de Beltinci"), ("ro", "Comuna Beltinci"), ("ru", "Бельтинци"), ("si", "බෙල\u{dca}ට\u{dd2}න\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Beltinci"), ("sr", "Општина Белтинци"), ("sr_Latn", "Opština Beltinci"), ("sv", "Beltinci kommun"), ("ta", "பெல\u{bcd}டின\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c46}ల\u{c4d}ట\u{c3f}ంచ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเบลต\u{e34}นซ\u{e35}"), ("tr", "Beltinci Belediyesi"), ("uk", "Белтинці"), ("ur", "بلدیہ بیلتینتسی"), ("vi", "Đô thị tự trị Beltinci"), ("zh", "貝爾廷齊鎮")]),
                        unofficial_name_list: ["Beltinci"].to_vec(),
                    }
                ),
                (
                    "003",
                    Subdivision{
                        name: "003",
                        country_alpha2: Alpha2::SI,
                        code: "003",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3683266), longitude: Some(14.1145798), max_latitude: Some(46.3861596), min_latitude: Some(46.3446751), max_longitude: Some(14.1353007), min_longitude: Some(14.0565934)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بليد"), ("bn", "ব\u{9cd}লেড পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11133}𑄣𑄬𑄖\u{11134}"), ("ceb", "Občina Bled (munisipyo sa Eslobenya)"), ("cs", "Občina Bled"), ("da", "Bled Municipality"), ("de", "Gemeinde Bled"), ("el", "Μπλέντ"), ("en", "Bled"), ("es", "Municipalidad del Bled"), ("fi", "Bledin kunta"), ("fr", "Bled"), ("gu", "બ\u{acd}લ\u{ac7}ડ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}ल\u{947}ड नगरपालिका"), ("hr", "Općina Bled"), ("id", "Kotamadya Bled"), ("it", "Bled"), ("ja", "ブレッド"), ("kn", "ಬ\u{ccd}ಲೇಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "블레드 지방 자치제"), ("lt", "Bledo savivaldybė"), ("lv", "Bledas pašvaldība"), ("mk", "Општина Блед"), ("mr", "ब\u{94d}ल\u{947}ड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bled Municipality"), ("nb", "Bled kommune"), ("nl", "Bled"), ("no", "Bled kommune"), ("pl", "Gmina Bled"), ("pt", "Município de Bled"), ("ro", "Comuna Bled"), ("ru", "Блед"), ("si", "බ\u{dca}ලේඩ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Bled"), ("sr", "Општина Блед"), ("sr_Latn", "Opština Bled"), ("sv", "Bled kommun"), ("ta", "ப\u{bcd}ளேட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ల\u{c46}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเบด"), ("tr", "Bled Belediyesi"), ("uk", "Блед (община)"), ("ur", "بلدیہ بلیت"), ("vi", "Đô thị tự trị Bled"), ("zh", "布萊德鎮")]),
                        unofficial_name_list: ["Bled"].to_vec(),
                    }
                ),
                (
                    "004",
                    Subdivision{
                        name: "004",
                        country_alpha2: Alpha2::SI,
                        code: "004",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.30056520000001), longitude: Some(13.9427195), max_latitude: Some(46.378352), min_latitude: Some(46.22574300000001), max_longitude: Some(14.1815722), min_longitude: Some(13.7243757)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوهيني"), ("bn", "বোহিঞ\u{9cd}জ পৌরসভ\u{9be}"), ("bs", "Bohinj"), ("ca", "Bohinj"), ("ccp", "𑄝\u{1112e}𑄦\u{11128}𑄚\u{11134}𑄎"), ("ceb", "Bohinj"), ("cs", "Občina Bohinj"), ("da", "Bohinj Municipality"), ("de", "Gemeinde Bohinj"), ("el", "Μποχίντζ"), ("en", "Bohinj"), ("es", "Municipalidad del Cayo"), ("eu", "Bohinj"), ("fi", "Bohinjn kunta"), ("fr", "Municipalité de Bohinj"), ("gu", "બોહિ\u{a82}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोहि\u{902}ज नगरपालिका"), ("hr", "Općina Bohinj"), ("hu", "Bohinj község"), ("id", "Kotamadya Bohinj"), ("it", "Bohinj"), ("ja", "ボーヒニ"), ("kn", "ಬೊಹ\u{cbf}ಂಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "보힌"), ("lt", "Bochinio savivaldybė"), ("lv", "Bohinjas pašvaldība"), ("mr", "बोहिनज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bohinj Municipality"), ("nb", "Bohinj Kommune"), ("nl", "Bohinj"), ("no", "Bohinj Kommune"), ("pl", "Gmina Bohinj"), ("pt", "Bohinj"), ("ro", "Comuna Bohinj"), ("ru", "Бохинь"), ("si", "බොහ\u{dd2}න\u{dca}ජ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Bohinj"), ("sr", "Општина Бохињ"), ("sr_Latn", "Opština Bohinj"), ("sv", "Bohinj"), ("ta", "போஹிஞ\u{bcd}ச\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4b}హ\u{c3f}ంజ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โบฮ\u{e34}นจ\u{e4c}"), ("tr", "Bohinj Belediyesi"), ("uk", "Бохінь"), ("ur", "بلدیہ بوخن"), ("vi", "Khu tự trị Bohinj"), ("zh", "博希尼")]),
                        unofficial_name_list: ["Bohinj"].to_vec(),
                    }
                ),
                (
                    "005",
                    Subdivision{
                        name: "005",
                        country_alpha2: Alpha2::SI,
                        code: "005",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9193034), longitude: Some(14.3640682), max_latitude: Some(45.9358871), min_latitude: Some(45.9012234), max_longitude: Some(14.3847739), min_longitude: Some(14.345481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوروفنيكا"), ("bn", "বোরোভনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11127}𑄢\u{1112e}𑄛\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Borovnica"), ("cs", "Občina Borovnica"), ("da", "Borovnica Municipality"), ("de", "Gemeinde Borovnica"), ("el", "Μποροβνίκα"), ("en", "Borovnica"), ("es", "Municipalidad del Borovnica"), ("fi", "Borovnican kunta"), ("fr", "Municipalité de Borovnica"), ("gu", "બોરોવ\u{acd}નીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोरोव\u{94d}निका नगर पालिका"), ("hr", "Općina Borovnica"), ("id", "Kotamadya Borovnica"), ("it", "Borovnica"), ("ja", "ボロヴニツァ"), ("kn", "ಬೊರೊವ\u{ccd}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "보로브니차 지방 자치제"), ("lt", "Borovnicos savivaldybė"), ("lv", "Borovnicas pašvaldība"), ("mr", "बोरोविका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Borovnica Municipality"), ("nb", "Borovnica kommune"), ("nl", "Borovnica"), ("no", "Borovnica kommune"), ("pl", "Gmina Borovnica"), ("pt", "Município de Borovnica"), ("ro", "Comuna Borovnica"), ("ru", "Боровница"), ("si", "බොරොව\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Borovnica"), ("sr", "Општина Боровница"), ("sr_Latn", "Opština Borovnica"), ("sv", "Borovnica kommun"), ("ta", "போரோவனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4b}ర\u{c4b}వ\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องโบรอฟน\u{e34}เซ"), ("tr", "Borovnica Belediyesi"), ("uk", "Боровниця (община)"), ("ur", "بلدیہ بوروونیتسا"), ("vi", "Đô thị tự trị Borovnica"), ("zh", "博羅夫尼察鎮")]),
                        unofficial_name_list: ["Borovnica"].to_vec(),
                    }
                ),
                (
                    "006",
                    Subdivision{
                        name: "006",
                        country_alpha2: Alpha2::SI,
                        code: "006",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3376387), longitude: Some(13.551683), max_latitude: Some(46.3704078), min_latitude: Some(46.3150629), max_longitude: Some(13.5923068), min_longitude: Some(13.490824)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوفيتش"), ("bn", "বোভেক পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11127}𑄞𑄬𑄇\u{11134}"), ("ceb", "Občina Bovec"), ("cs", "Občina Bovec"), ("da", "Bovec Municipality"), ("de", "Gemeinde Bovec"), ("el", "Μποβέκ"), ("en", "Bovec"), ("es", "Municipalidad del Bovec"), ("fi", "Bovecin kunta"), ("fr", "Municipalité de Bovec"), ("gu", "બોવ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बोव\u{947}क नगर पालिका"), ("hr", "Općina Bovec"), ("id", "Kotamadya Bovec"), ("it", "Comune di Bovec"), ("ja", "ボヴェツ"), ("kn", "ಬೊವ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "보베츠 지방 자치제"), ("lt", "Boveco savivaldybė"), ("lv", "Bovecas pašvaldība"), ("mr", "बोव\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bovec Municipality"), ("nb", "Bovec kommune"), ("nl", "Bovec"), ("no", "Bovec kommune"), ("pl", "Gmina Bovec"), ("pt", "Bovec"), ("ro", "Comuna Bovec"), ("ru", "Бовец"), ("si", "බොවෙක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Bovec"), ("sr", "Општина Бовец"), ("sr_Latn", "Opština Bovec"), ("sv", "Bovec kommun"), ("ta", "போவேக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4b}వ\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องก\u{e31}ลเล\u{e48}"), ("tr", "Bovec Belediyesi"), ("uk", "Бовец"), ("ur", "بلدیہ بوویتس"), ("vi", "Đô thị tự trị Bovec"), ("zh", "博韋茨鎮")]),
                        unofficial_name_list: ["Bovec"].to_vec(),
                    }
                ),
                (
                    "007",
                    Subdivision{
                        name: "007",
                        country_alpha2: Alpha2::SI,
                        code: "007",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4716586), longitude: Some(15.1439794), max_latitude: Some(46.4788807), min_latitude: Some(46.4569051), max_longitude: Some(15.1765445), min_longitude: Some(15.1337258)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بردا"), ("bn", "ব\u{9cd}রড\u{9be} পৌরসভ\u{9be}"), ("bs", "Brda"), ("ca", "Brda"), ("ccp", "𑄝\u{11133}𑄢\u{11127}𑄓"), ("ceb", "Brda"), ("cs", "Občina Brda"), ("da", "Brda Municipality"), ("de", "Brda"), ("el", "Μπρντά"), ("en", "Brda"), ("es", "Brda"), ("fi", "Brdan kunta"), ("fr", "Brda"), ("gu", "બ\u{acd}રડા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बर\u{94d}डा नगरपालिका"), ("hr", "Općina Brda"), ("hu", "Brda község"), ("id", "Kotamadya Brda"), ("it", "Collio"), ("ja", "ブルダ"), ("kn", "ಬ\u{ccd}ರಾಡಾ ಪುರಸಭ\u{cc6}"), ("ko", "브르다"), ("lt", "Brdos savivaldybė"), ("lv", "Brdas pašvaldība"), ("mr", "ब\u{94d}रडा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Brda Municipality"), ("nb", "Brda kommune"), ("nl", "Brda"), ("no", "Brda kommune"), ("pl", "Gmina Brda"), ("pt", "Brda"), ("ro", "Brda"), ("ru", "Брда"), ("si", "බ\u{dca}\u{200d}ර\u{dca}ඩ\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Brda"), ("sr", "Општина Брда"), ("sr_Latn", "Opština Brda"), ("sv", "Brda"), ("ta", "பிரட\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ర\u{c3f}డ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอาร\u{e4c}ดา"), ("tr", "Brda Belediyesi"), ("uk", "Брда"), ("ur", "بلدیہ بردا"), ("vi", "Đô thị tự trị Brda"), ("zh", "布爾達")]),
                        unofficial_name_list: ["Brda"].to_vec(),
                    }
                ),
                (
                    "008",
                    Subdivision{
                        name: "008",
                        country_alpha2: Alpha2::SI,
                        code: "008",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.872713), longitude: Some(15.2412372), max_latitude: Some(45.88924420000001), min_latitude: Some(45.8690201), max_longitude: Some(15.2499418), min_longitude: Some(15.2181643)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريزوفيتسا"), ("bn", "ব\u{9cd}রেজোভিক\u{9be}"), ("bs", "Brezovica"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄎\u{1112e}𑄞\u{11128}𑄇"), ("ceb", "Brezovica"), ("cs", "Občina Brezovica"), ("da", "Brezovica Municipality"), ("de", "Brezovica"), ("el", "Μπρέζοβιτσα"), ("en", "Brezovica"), ("es", "Municipalidad del Brezovica"), ("fi", "Brezovican kunta"), ("fr", "Brezovica"), ("gu", "બ\u{acd}ર\u{ac7}ઝોવિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}र\u{947}ज\u{93c}ोविका नगर पालिका"), ("hr", "Općina Brezovica"), ("hu", "Brezovica"), ("id", "Kotamadya Brezovica"), ("it", "Brezovica"), ("ja", "ブレゾヴィツァ"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಝೋವ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "브레조비차"), ("lt", "Brezovicos savivaldybė"), ("lv", "Brezovicas pašvaldība"), ("mr", "ब\u{94d}र\u{947}झोवीच\u{902} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Brezovica Municipality"), ("nb", "Brezovica Kommune"), ("nl", "Brezovica"), ("no", "Brezovica Kommune"), ("pl", "Gmina Brezovica"), ("pt", "Brezovica"), ("ro", "Brezovica"), ("ru", "Брезовица"), ("si", "බ\u{dca}\u{200d}රේසොව\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Brezovica"), ("sr", "Општина Брезовица"), ("sr_Latn", "Opština Brezovica"), ("sv", "Brezovica"), ("ta", "ப\u{bcd}ரெஸ\u{bcd}வ\u{bcd}விக\u{bcd}க நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ర\u{c46}జ\u{c3e}వ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเบรโซว\u{e34}ก\u{e49}า"), ("tr", "Brezovica"), ("uk", "Брезовиця (община)"), ("ur", "بلدیہ بریزوویتسا"), ("vi", "Brezovica"), ("zh", "布雷佐维察")]),
                        unofficial_name_list: ["Brezovica"].to_vec(),
                    }
                ),
                (
                    "009",
                    Subdivision{
                        name: "009",
                        country_alpha2: Alpha2::SI,
                        code: "009",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9088102), longitude: Some(15.5964652), max_latitude: Some(45.9408957), min_latitude: Some(45.8930075), max_longitude: Some(15.6281685), min_longitude: Some(15.5797608)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريجيتسه"), ("bn", "ব\u{9cd}রেজিচ পৌরসভ\u{9be}"), ("ca", "Brežice"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄎\u{1112d}𑄌\u{11134}"), ("ceb", "Občina Brežice"), ("cs", "Občina Brežice"), ("da", "Brežice Municipality"), ("de", "Gemeinde Brežice"), ("el", "Μπρεζίκε"), ("en", "Brežice"), ("es", "Brežice"), ("fi", "Brežicen kunta"), ("fr", "Brežice"), ("gu", "બ\u{acd}ર\u{ac7}ઝિસ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}र\u{947}ज\u{93c}िस नगर पालिका"), ("hr", "Općina Brežice"), ("hu", "Brežice"), ("id", "Kotamadya Brežice"), ("it", "Brežice"), ("ja", "ブレージツェ"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಝ\u{cbf}ಸ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "브레지체 지방 자치제"), ("lt", "Brežicės savivaldybė"), ("lv", "Brežices pašvaldība"), ("mr", "ब\u{94d}र\u{947}झिस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Brezice Municipality"), ("nb", "Brezice kommune"), ("nl", "Brežice"), ("no", "Brezice kommune"), ("pl", "Gmina Brežice"), ("pt", "Município de Brezice"), ("ro", "Comuna Brežice"), ("ru", "Брежице"), ("si", "බ\u{dca}රෙස\u{dd2}කේ නගර සභ\u{dcf}ව"), ("sl", "Občina Brežice"), ("sr", "Општина Брежице"), ("sr_Latn", "Opština Brežice"), ("sv", "Brezice kommun"), ("ta", "பிரெஜிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ర\u{c3f}జ\u{c48}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องบร\u{e34}ซ\u{e34}เซ\u{e48}"), ("tr", "Brezice Belediyesi"), ("uk", "Брежице"), ("ur", "بلدیہ بریژیتسے"), ("vi", "Đô thị tự trị Brezice"), ("zh", "布雷日采鎮")]),
                        unofficial_name_list: ["Brežice"].to_vec(),
                    }
                ),
                (
                    "010",
                    Subdivision{
                        name: "010",
                        country_alpha2: Alpha2::SI,
                        code: "010",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6511382), longitude: Some(16.0834627), max_latitude: Some(46.663156), min_latitude: Some(46.6282126), max_longitude: Some(16.0958059), min_longitude: Some(16.0726483)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄑\u{11128}𑄥\u{11128}𑄚"), ("ceb", "Občina Tišina"), ("cs", "Občina Tišina"), ("da", "Tišina"), ("de", "Tišina"), ("en", "Tišina"), ("fa", "تیشینا"), ("fr", "Tišina"), ("hr", "Tišina (Tišina, Slovenija)"), ("hu", "Csendlak"), ("it", "Tišina"), ("ja", "ティシナ"), ("ko", "티시나"), ("nb", "Tišina"), ("nl", "Tišina"), ("no", "Tišina"), ("pl", "Gmina Tišina"), ("pt", "Tišina"), ("ro", "Tišina"), ("ru", "Тишина"), ("sl", "Tišina, Tišina"), ("sr", "Општина Тишина"), ("sr_Latn", "Opština Tišina"), ("sv", "Tišina"), ("uk", "Тишина"), ("vi", "Tišina"), ("zh", "蒂希納")]),
                        unofficial_name_list: ["Tišina"].to_vec(),
                    }
                ),
                (
                    "011",
                    Subdivision{
                        name: "011",
                        country_alpha2: Alpha2::SI,
                        code: "011",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.23974949999999), longitude: Some(15.2677063), max_latitude: Some(46.2649277), min_latitude: Some(46.20673679999999), max_longitude: Some(15.2977829), min_longitude: Some(15.2114242)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مدينة تسيليه"), ("be", "Цэле"), ("bn", "শহর পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄣\u{11134}𑄎𑄬"), ("ceb", "Celje"), ("cs", "Městská občina Celje"), ("da", "City Municipality of Celje"), ("de", "Stadtgemeinde Celje"), ("el", "Δημοτική Κοινότητα του Κέλτζε"), ("en", "Celje"), ("es", "Ciudad del Celje"), ("fi", "Celjen kaupunginosa"), ("fr", "Celje (Municipalité urbaine)"), ("gu", "સ\u{ac7}લ\u{acd}જ\u{ac7}ની શહ\u{ac7}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}लिय\u{947} शहर नगरपालिका"), ("hr", "Gradska općina Celje"), ("hu", "Celje városi község"), ("id", "Kotamadya Celje"), ("it", "Celje"), ("ja", "ツェリエ"), ("kn", "ಸ\u{cbf}ಟ\u{cbf} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf} ಆಫ\u{ccd} ಸ\u{cc6}ಲ\u{ccd}ಜ\u{cc6}"), ("ko", "첼레의 도시 지방 자치제"), ("lt", "Celjės miesto savivaldybė"), ("lv", "Celjes pilsētas pašvaldība"), ("mr", "स\u{947}ल\u{94d}ज\u{947} ची शहर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "City Municipality of Celje"), ("nb", "Celje Hovedstad"), ("nl", "Celje"), ("no", "Celje Hovedstad"), ("pl", "Gmina miejska Celje"), ("pt", "Cidade Municipal de Celje"), ("ro", "Comuna urbană Celje"), ("ru", "Целе"), ("si", "සෙල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Mestna občina Celje"), ("sr", "Општина Цеље"), ("sr_Latn", "Opština Celje"), ("sv", "Ceje Huvudtsadskommun"), ("ta", "நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி ஒப\u{bcd}பி செஜே"), ("te", "స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40} ఆఫ\u{c4d} స\u{c46}ల\u{c4d}జ\u{c47}"), ("th", "แคว\u{e49}นโอฮ\u{e31}งเวนา"), ("tr", "Şehir Meclisi"), ("uk", "Целє"), ("ur", "شہر بلدیہ تسیلیے"), ("vi", "Đô thị tự trị của Celje"), ("zh", "采列市")]),
                        unofficial_name_list: ["Celje"].to_vec(),
                    }
                ),
                (
                    "012",
                    Subdivision{
                        name: "012",
                        country_alpha2: Alpha2::SI,
                        code: "012",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2531292), longitude: Some(14.4868829), max_latitude: Some(46.2625842), min_latitude: Some(46.2452394), max_longitude: Some(14.4934701), min_longitude: Some(14.4646636)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تسيركليه نا غورينيسكيم"), ("bn", "চ\u{9be}র\u{9cd}লিযে ন\u{9be} গোরেন পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄢\u{11134}𑄇\u{11134}𑄣\u{11134}𑄎𑄬 𑄚 𑄉\u{11127}𑄢𑄬𑄚\u{11134}𑄥\u{11134}𑄇𑄬𑄟\u{11134}"), ("ceb", "Cerklje na Gorenjskem"), ("cs", "Občina Cerklje na Gorenjskem"), ("da", "Cerklje na Gorenjskem Municipality"), ("de", "Cerklje na Gorenjskem"), ("el", "Κέρκλτζε να Γκορέντζσκεμ"), ("en", "Cerklje na Gorenjskem"), ("fi", "Cerklje na Gorenjskem"), ("fr", "Cerklje na Gorenjskem"), ("gu", "સર\u{acd}કલજ\u{ac7} ના ગોર\u{ac7}ન\u{acd}જસ\u{acd}ક\u{ac7}મ , મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सर\u{94d}कलिए ना गोर\u{947}न\u{94d}सक\u{947}म नगरपालिका"), ("hr", "Općina Cerklje na Gorenjskem"), ("hu", "Cerklje na Gorenjskem"), ("id", "Kotamadya Cerklje na Gorenjskem"), ("it", "Cerklje na Gorenjskem"), ("ja", "ツェルクニェ・ナ・ゴレニスケム"), ("kn", "ಸ\u{cbf}ರ\u{ccd}ಕ\u{ccd}ಲ\u{cc6}ಜ\u{cc6} ನಾ ಗೊರ\u{cc6}ಂಜ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "체르클레나고렌스켐"), ("lt", "Cerkle na Gorenskjemo savivaldybė"), ("lv", "Cerklje na Gorenjskemas pašvaldība"), ("mr", "सर\u{94d}कलज\u{947} ना गोर\u{947}नस\u{94d}क\u{945}म म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cerklje na Gorenjskem Municipality"), ("nb", "Cerklje na Gorenjskem"), ("nl", "Cerklje na Gorenjskem"), ("no", "Cerklje na Gorenjskem"), ("pl", "Gmina Cerklje na Gorenjskem"), ("pt", "Cerklje na Gorenjskem"), ("ro", "Cerklje na Gorenjskem"), ("ru", "Церкле-на-Гореньскем"), ("si", "සර\u{dca}ක\u{dca}ල\u{dca}ජේ න\u{dcf} ගොරෙන\u{dca}ජ\u{dca}ස\u{dca}කෙම\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Cerklje na Gorenjskem"), ("sr", "Општина Церкље на Горењскем"), ("sr_Latn", "Opština Cerklje na Gorenjskem"), ("sv", "Cerklje na Gorenjskem"), ("ta", "செர\u{bcd}க\u{bcd}லஜே ந\u{bbe} கோரேஞ\u{bcd}ச\u{bcd}கஎம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ర\u{c4d}కుజ\u{c47} న\u{c3e} గ\u{c4b}ర\u{c46}ంజస\u{c4d}క\u{c46}మ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เค\u{e34}ร\u{e4c}กเจนาโกเรนจ\u{e4c}สเคม"), ("tr", "Cerklje na Gorenjskem Belediyesi"), ("uk", "Церклє на Гореньскем"), ("ur", "کیرکلجی نا جورینجسکیم میونسپلٹی"), ("vi", "Cerklje na Gorenjskem"), ("zh", "戈雷尼斯卡地区采尔克列")]),
                        unofficial_name_list: ["Cerklje na Gorenjskem"].to_vec(),
                    }
                ),
                (
                    "013",
                    Subdivision{
                        name: "013",
                        country_alpha2: Alpha2::SI,
                        code: "013",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7955099), longitude: Some(14.3621843), max_latitude: Some(45.8239503), min_latitude: Some(45.7732728), max_longitude: Some(14.4148252), min_longitude: Some(14.3233742)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تسركنيتسا"), ("bn", "সের\u{9cd}কনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄢\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Cerknica (munisipyo)"), ("cs", "Občina Cerknica"), ("da", "Cerknica"), ("de", "Gemeinde Cerknica"), ("el", "Κερκνίκα"), ("en", "Cerknica"), ("es", "Municipalidad del Cerknica"), ("fi", "Cerknican kunta"), ("fr", "Cerknica"), ("gu", "સ\u{ac7}ર\u{acd}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "च\u{947}र\u{94d}कनिका नगरपालिका"), ("hr", "Općina Cerknica"), ("id", "Kotamadya Cerknica"), ("it", "Circonio"), ("ja", "ツェルクニツァ"), ("kn", "ಸ\u{cc6}ರ\u{ccd}ಕ\u{ccd}ನ\u{cbf}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "체르크니차 지방 자치제"), ("lt", "Cerknicos savivaldybė"), ("lv", "Cerknicas pašvaldība"), ("mr", "स\u{947}र\u{947}क\u{94d}नाका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cerknica Municipality"), ("nb", "Cerknica kommune"), ("nl", "Cerknica"), ("no", "Cerknica kommune"), ("pl", "Gmina Cerknica"), ("pt", "Município de Cerknica"), ("ro", "Comuna Cerknica"), ("ru", "Церкница"), ("si", "සර\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Cerknica"), ("sr", "Општина Церкница"), ("sr_Latn", "Opština Cerknica"), ("sv", "Cerknica kommun"), ("ta", "சேர\u{bcd}க\u{bcd}கினிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ర\u{c4d}క\u{c4d}\u{200c}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเจอร\u{e4c}คน\u{e34}กา"), ("tr", "Cerknica Belediyesi"), ("uk", "Церкниця (община)"), ("ur", "بلدیہ تسیرکنیتسا"), ("vi", "Đô thị tự trị Cerknica"), ("zh", "采爾克尼察鎮")]),
                        unofficial_name_list: ["Cerknica"].to_vec(),
                    }
                ),
                (
                    "014",
                    Subdivision{
                        name: "014",
                        country_alpha2: Alpha2::SI,
                        code: "014",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.128954), longitude: Some(13.9891931), max_latitude: Some(46.1402911), min_latitude: Some(46.1034782), max_longitude: Some(14.0049282), min_longitude: Some(13.9494446)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تسيركنو"), ("bn", "চেরকনো পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄢\u{11134}𑄚\u{1112e}"), ("ceb", "Cerkno"), ("cs", "Občina Cerkno"), ("da", "Cerkno Municipality"), ("de", "Gemeinde Cerkno"), ("el", "Κέρκνον"), ("en", "Cerkno"), ("es", "Municipalidad del Cerkno"), ("fi", "Cerknon kunta"), ("fr", "Commune de Cerkno"), ("gu", "સ\u{ac7}ર\u{acd}ક\u{acd}નો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}र\u{94d}क\u{94d}नो नगरपालिका"), ("hr", "Općina Cerkno"), ("id", "Kotamadya Cerkno"), ("it", "Circhina"), ("ja", "ツェルクノ"), ("kn", "ಸ\u{cc6}ರ\u{ccd}ಕ\u{ccd}ನೋ ಪುರಸಭ\u{cc6}"), ("ko", "체르크노 지방 자치제"), ("lt", "Cerkno savivaldybė"), ("lv", "Cerkno pašvaldība"), ("mr", "स\u{947}रक\u{94d}नी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbadaran Cerkno"), ("nb", "Cerko kommune"), ("nl", "Cerkno"), ("no", "Cerko kommune"), ("pl", "Gmina Cerkno"), ("pt", "Município de Cerkno"), ("ro", "Comuna Cerkno"), ("ru", "Церкно"), ("si", "සර\u{dca}ක\u{dca}නෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Cerkno"), ("sr", "Општина Церкно"), ("sr_Latn", "Opština Cerkno"), ("sv", "Cerkno kommun"), ("ta", "சேர\u{bcd}க\u{bcd}கனோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ర\u{c4d}క\u{c4d}న\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเจอร\u{e4c}คโน"), ("tr", "Cerkno Belediyesi"), ("uk", "Община Церкно"), ("ur", "بلدیہ تسیرکنو"), ("vi", "Đô thị tự trị Cerkno"), ("zh", "采爾克諾鎮")]),
                        unofficial_name_list: ["Cerkno"].to_vec(),
                    }
                ),
                (
                    "015",
                    Subdivision{
                        name: "015",
                        country_alpha2: Alpha2::SI,
                        code: "015",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5720029), longitude: Some(16.2877346), max_latitude: Some(46.5852017), min_latitude: Some(46.5272102), max_longitude: Some(16.3453289), min_longitude: Some(16.2666812)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تشرينشوفتسي"), ("bn", "ক\u{9cd}রেন\u{9cd}সোভকি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢𑄬𑄚\u{11134}𑄥\u{1112e}𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Občina Črenšovci"), ("cs", "Občina Črenšovci"), ("da", "Črenšovci"), ("de", "Črenšovci"), ("el", "Κρενσόβκι"), ("en", "Črenšovci"), ("es", "Municipalidad del Črenšovci"), ("fi", "Črenšovcin kunta"), ("fr", "Črenšovci"), ("gu", "ક\u{acd}ર\u{ac7}નસોવ\u{acd}કી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}र\u{947}\u{902}सोव\u{94d}की नगरपालिका"), ("hr", "Općina Črenšovci"), ("hu", "Cserföld"), ("id", "Kotamadya Črenšovci"), ("it", "Črenšovci"), ("ja", "チュレンショフツィ"), ("kn", "ಕ\u{ccd}ರ\u{cc6}ನೊವೊಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "치렌쇼브치"), ("lt", "Črenšovcio savivaldybė"), ("lv", "Črenšovcu pašvaldība"), ("mr", "क\u{94d}र\u{947}नसॉव\u{94d}हसी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Crensovci Municipality"), ("nb", "Crensovci kommune"), ("nl", "Črenšovci"), ("no", "Crensovci kommune"), ("pl", "Gmina Črenšovci"), ("pt", "Črenšovci"), ("ro", "Črenšovci"), ("ru", "Креншовцы"), ("si", "ක\u{dca}රෙන\u{dca}සොව\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Črenšovci"), ("sr", "Општина Чреншовци"), ("sr_Latn", "Opština Črenšovci"), ("sv", "Crensovci kommun"), ("ta", "கிரென\u{bcd}சோவசி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c46}న\u{c4b}వ\u{c3f}స\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเซรนโซว\u{e34}"), ("tr", "Crensovci Belediyesi"), ("uk", "Чреншовці"), ("ur", "بلدیہ چرینشووتسی"), ("vi", "Črenšovci"), ("zh", "奇倫紹夫齊")]),
                        unofficial_name_list: ["Crenšovci"].to_vec(),
                    }
                ),
                (
                    "016",
                    Subdivision{
                        name: "016",
                        country_alpha2: Alpha2::SI,
                        code: "016",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4703983), longitude: Some(14.8498787), max_latitude: Some(46.4825972), min_latitude: Some(46.4495989), max_longitude: Some(14.8836197), min_longitude: Some(14.8165776)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تشرنا نا كوروشكيم"), ("bn", "ক\u{9be}র\u{9cd}ন\u{9be} ন\u{9be} করো\u{981}কে পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢𑄚 𑄚 𑄇\u{11127}𑄢\u{1112e}𑄢\u{11134}𑄇𑄬𑄟\u{11134}"), ("ceb", "Občina Črna na Koroškem"), ("cs", "Občina Črna na Koroškem"), ("da", "Črna na Koroškem Municipality"), ("de", "Črna na Koroškem"), ("el", "Κρνα να Κορόσκεμ"), ("en", "Črna na Koroškem"), ("fi", "Črna na Koroškemin kunta"), ("fr", "Črna na Koroškem"), ("gu", "કર\u{acd}ના ના કોરોસ\u{acd}ક\u{ac7}મ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रना ना कोरोस\u{94d}क\u{947}म नगरपालिका"), ("hr", "Općina Črna na Koroškem"), ("hu", "Črna na Koroškem"), ("id", "Kotamadya Črna na Koroškem"), ("it", "Črna na Koroškem"), ("ja", "チュルナ・ナ・コロシュケム"), ("kn", "ಸ\u{cbf}ರ\u{ccd}ನಾ ನಾ ಕೊರೊಸ\u{ccd}ಕ\u{cc6}ಮ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "치르나나코로슈켐"), ("lt", "Črna na Koroškemo savivaldybė"), ("lv", "Črna na Koroškemas pašvaldība"), ("mr", "कर\u{94d}ण ना कॉरोस\u{94d}क\u{947}म म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Crna na Koroskem"), ("nb", "Črna na Koroškem"), ("nl", "Črna na Koroškem"), ("no", "Črna na Koroškem"), ("pl", "Gmina Črna na Koroškem"), ("pt", "Črna na Koroškem"), ("ro", "Črna na Koroškem"), ("ru", "Чрна-на-Корошкем"), ("si", "ක\u{dca}ර\u{dca}ණ\u{dcf} න\u{dcf}"), ("sl", "Občina Črna na Koroškem"), ("sr", "Општина Чрна на Корошкем"), ("sr_Latn", "Opština Črna na Koroškem"), ("sv", "Črna na Koroškem"), ("ta", "கிரண ந\u{bbe} கோரோஸ\u{bcd}கிம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "కర\u{c4d}న\u{c3e} ల\u{c3e} క\u{c4b}రకమ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "นา นา โครอสเคม ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Crna Na Koroske Belediyesi"), ("uk", "Чрна-на-Корошкем"), ("ur", "بلدیہ چرنا نا کوروشکیم"), ("vi", "Đô thị tự trị Crna na Koroskem"), ("zh", "科罗什卡地区奇尔纳")]),
                        unofficial_name_list: ["Crna na Koroškem"].to_vec(),
                    }
                ),
                (
                    "017",
                    Subdivision{
                        name: "017",
                        country_alpha2: Alpha2::SI,
                        code: "017",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.57148369999999), longitude: Some(15.1930773), max_latitude: Some(45.59965), min_latitude: Some(45.5460565), max_longitude: Some(15.2236525), min_longitude: Some(15.165933)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كموميليه"), ("bn", "ক\u{9cd}রোনোমেলি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11133}𑄢𑄚\u{11127}𑄟𑄬𑄣\u{11134}𑄎𑄬"), ("ceb", "Občina Črnomelj"), ("cs", "Občina Črnomelj"), ("da", "Črnomelj Municipality"), ("de", "Gemeinde Črnomelj"), ("el", "Κρνομέλτζ"), ("en", "Črnomelj"), ("es", "Črnomelj"), ("fi", "Črnomeljin kunta"), ("fr", "Municipalité de Črnomelj"), ("gu", "ક\u{acd}રનોમ\u{ac7}લ\u{acd}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}र\u{94d}नोमलज\u{947} नगर पालिका"), ("hr", "Općina Črnomelj"), ("hu", "Črnomelj"), ("id", "Kotamadya Črnomelj"), ("it", "Črnomelj"), ("ja", "チュルノメリ"), ("kn", "ಕ\u{ccd}ರುನೋಮ\u{cc6}ಲ\u{ccd}ಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "치르노멜 지방 자치제"), ("lt", "Črnomelio savivaldybė"), ("lv", "Črnomeljas pašvaldība"), ("mr", "सिमोम\u{947}लज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Crnomelj Municipality"), ("nb", "Cronmelj kommune"), ("nl", "Črnomelj"), ("no", "Cronmelj kommune"), ("pl", "Gmina Črnomelj"), ("pt", "Cronmelj"), ("ro", "Comuna Črnomelj"), ("ru", "Чрномель"), ("si", "ක\u{dca}\u{200d}රනොමෙල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Črnomelj"), ("sr", "Општина Чрномељ"), ("sr_Latn", "Opština Črnomelj"), ("sv", "Cronmelj kommun"), ("ta", "கிரணோமேல\u{bcd}ஜ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c4b}న\u{c4b}మ\u{c46}ల\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องชรโนเมล"), ("tr", "Cmomelj Belediyesi"), ("uk", "Чрномель"), ("ur", "بلدیہ چرنومیل"), ("vi", "Đô thị tự trị Črnomelj"), ("zh", "奇爾諾梅利鎮")]),
                        unofficial_name_list: ["Crnomelj"].to_vec(),
                    }
                ),
                (
                    "018",
                    Subdivision{
                        name: "018",
                        country_alpha2: Alpha2::SI,
                        code: "018",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4922368), longitude: Some(15.8777956), max_latitude: Some(46.4972951), min_latitude: Some(46.4853092), max_longitude: Some(15.8860775), min_longitude: Some(15.8745583)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ديسترنيك"), ("bn", "ডেস\u{9cd}টমিক পৌরসভ\u{9be}"), ("ccp", "𑄓𑄬𑄌\u{11134}𑄑\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Destrnik"), ("cs", "Občina Destrnik"), ("da", "Destrnik Municipality"), ("de", "Destrnik"), ("el", "Ντέστρνικ"), ("en", "Destrnik"), ("es", "Municipalidad del Destrnik"), ("fi", "Destrnikin kunta"), ("fr", "Destrnik"), ("gu", "ડ\u{ac7}સ\u{acd}ટ\u{acd}રનિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{947}स\u{94d}ट\u{94d}रनिक नगरपालिका"), ("hr", "Općina Destrnik"), ("id", "Kotamadya Destrnik"), ("it", "Destrnik"), ("ja", "デストルニク"), ("kn", "ಡ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}ರನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "데스트르니크"), ("lt", "Destrniko savivaldybė"), ("lv", "Destrnikas municipalitāte"), ("mr", "द\u{947}स\u{94d}तार\u{94d}निक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Destrnik Municipality"), ("nb", "Destrnik kommune"), ("nl", "Destrnik"), ("no", "Destrnik kommune"), ("pl", "Gmina Destrnik"), ("pt", "Destrnik"), ("ro", "Destrnik"), ("ru", "Дестрник"), ("si", "ඩෙස\u{dca}ට\u{dca}ම\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Destrnik"), ("sr", "Општина Дестрник"), ("sr_Latn", "Opština Destrnik"), ("sv", "Destrnik"), ("ta", "டெஸ\u{bcd}டர\u{bcd}னிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c46}స\u{c4d}ట\u{c46}మ\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเดสท\u{e4c}ม\u{e34}ค"), ("tr", "Destrnik Belediyesi"), ("uk", "Дестрник"), ("ur", "دیسترنیک میونسپلٹی"), ("vi", "Đô thị tự trị Destrnik"), ("zh", "代斯特尔尼克")]),
                        unofficial_name_list: ["Destrnik"].to_vec(),
                    }
                ),
                (
                    "019",
                    Subdivision{
                        name: "019",
                        country_alpha2: Alpha2::SI,
                        code: "019",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.68291319999999), longitude: Some(13.9697218), max_latitude: Some(45.7069151), min_latitude: Some(45.6697414), max_longitude: Some(14.0009139), min_longitude: Some(13.9333018)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ديفاتشا"), ("bn", "দিভিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄓\u{11128}𑄞𑄇"), ("ceb", "Občina Divača"), ("cs", "Občina Divača"), ("da", "Divaca"), ("de", "Gemeinde Divača"), ("el", "Ντιβάτσα"), ("en", "Divača"), ("es", "Divača"), ("fi", "Divačan kunta"), ("fr", "Municipalité de Divača"), ("gu", "દિવાકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "दिवाका नगरपालिका"), ("hr", "Općina Divača"), ("id", "Kotamadya Divača"), ("it", "Divaccia"), ("ja", "ディヴァーチャ"), ("kn", "ದ\u{cbf}ವಾಸಾ ಪುರಸಭ\u{cc6}"), ("ko", "디바차 지방 자치제"), ("lt", "Divačos savivaldybė"), ("lv", "Divačas pašvaldība"), ("mr", "दिवाका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Divaca Municipality"), ("nb", "Divaca Kommune"), ("nl", "Divača"), ("no", "Divaca Kommune"), ("pl", "Gmina Divača"), ("pt", "Divaca"), ("ro", "Comuna Divača"), ("ru", "Дивача"), ("si", "ද\u{dd2}වක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Divača"), ("sr", "Општина Дивача"), ("sr_Latn", "Opština Divača"), ("sv", "Divaca (kommun)"), ("ta", "டிவ\u{bbe}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c3f}వ\u{c3e}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องด\u{e34}วาก\u{e49}า"), ("tr", "Divaca Belediyesi"), ("uk", "Дівача"), ("ur", "دیواکا میونسپلٹی"), ("vi", "Đô thị tự trị Divaca"), ("zh", "迪瓦查")]),
                        unofficial_name_list: ["Divaca"].to_vec(),
                    }
                ),
                (
                    "020",
                    Subdivision{
                        name: "020",
                        country_alpha2: Alpha2::SI,
                        code: "020",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8524951), longitude: Some(14.7083109), max_latitude: Some(45.8934452), min_latitude: Some(45.7265798), max_longitude: Some(14.8421099), min_longitude: Some(14.6339332)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبريبوليه"), ("bn", "দোব\u{9cd}রেপলজে পৌরসভ\u{9be}"), ("bs", "Dobrepolje"), ("ccp", "𑄓\u{11127}𑄝\u{11133}𑄢𑄬𑄛\u{1112e}𑄣\u{11134}𑄎𑄬"), ("ceb", "Dobrepolje"), ("cs", "Občina Dobrepolje"), ("da", "Dobrepolje Municipality"), ("de", "Dobrepolje"), ("el", "Ντομπρέπολτζε"), ("en", "Dobrepolje"), ("es", "Minicipalidad Dobrepolje"), ("fi", "Dobrepoljen kunta"), ("fr", "Dobrepolje"), ("gu", "ડોબ\u{acd}પ\u{acd}રોલ\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोबरपोलिए नगरपालिका"), ("hr", "Općina Dobrepolje"), ("id", "Kotamadya Dobrepolje"), ("it", "Dobrepolje"), ("ja", "ドブレポリェ"), ("kn", "ಡೊಬ\u{ccd}ರ\u{cc6}ಪೊಲ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "도브레폴레"), ("lt", "Dobrepolės savivaldybė"), ("lv", "Dobrepoļes pašvaldība"), ("mr", "डोब\u{94d}रोपोलिए म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dobrepolje Municipality"), ("nb", "Dobrepolje kommune"), ("nl", "Dobrepolje"), ("no", "Dobrepolje kommune"), ("pl", "Gmina Dobrepolje"), ("pt", "Dobrepolje"), ("ro", "Dobrepolje"), ("ru", "Добреполье"), ("si", "දොබ\u{dca}රේපොල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Dobrepolje"), ("sr", "Општина Добрепоље"), ("sr_Latn", "Opština Dobrepolje"), ("sv", "Dobrepolje (kommun)"), ("ta", "டூப\u{bcd}ரேபொல\u{bcd}ஜெ நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}బ\u{c4d}రప\u{c4b}ల\u{c4d}జ\u{c47} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโดบร\u{e35}พอลเจ"), ("tr", "Dobrepolje Belediyesi"), ("uk", "Добреполє"), ("ur", "دوبریپولجی میونسپلٹی"), ("vi", "Dobrepolje"), ("zh", "多布雷波列鎮")]),
                        unofficial_name_list: ["Dobrepolje"].to_vec(),
                    }
                ),
                (
                    "021",
                    Subdivision{
                        name: "021",
                        country_alpha2: Alpha2::SI,
                        code: "021",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0678183), longitude: Some(14.2439619), max_latitude: Some(46.107928), min_latitude: Some(46.01133180000001), max_longitude: Some(14.4427003), min_longitude: Some(14.1883156)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبروفا بولهوف غرادك"), ("bn", "দব\u{9cd}রোভ\u{9be}-পলহভ গ\u{9cd}র\u{9be}দেচ পৌরসভ\u{9be}"), ("bs", "Dobrova-Polhov Gradec"), ("ccp", "𑄓\u{11127}𑄝\u{11133}𑄢\u{1112e}𑄞-𑄛\u{11127}𑄣\u{11134}𑄦\u{1112e}𑄛\u{11134} 𑄉\u{11133}𑄢𑄓𑄬𑄇\u{11134}"), ("ceb", "Dobrova-Polhov Gradec"), ("cs", "Občina Dobrova-Polhov Gradec"), ("da", "Dobrova–Polhov Gradec Municipality"), ("de", "Dobrova-Polhov Gradec"), ("el", "Ντόμπροβα-Πολχόβ Γκράντεκ"), ("en", "Dobrova–Polhov Gradec"), ("es", "Dobrova-Polhov Gradec"), ("fi", "Dobrova–Polhov Gradecin kunta"), ("fr", "Dobrova-Polhov Gradec"), ("gu", "ડોબરોવા-પોલહોવ ગ\u{acd}ર\u{ac7}ડ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोब\u{94d}रोवा-पोल\u{94d}होव ग\u{94d}राद\u{947}क नगरपालिका"), ("hr", "Općina Dobrova-Polhov Gradec"), ("id", "Kotamadya Dobrova–Polhov Gradec"), ("it", "Dobrova-Polhov Gradec"), ("kn", "ಡೊಬ\u{ccd}ರೋವಾ-ಪೋಲ\u{ccd}ಹೋವ\u{ccd} ಗ\u{ccd}ರಾಡ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "도브로바폴호브그라데츠"), ("lt", "Dobrova-Polchov Gradeco savivaldybė"), ("lv", "Dobrovas–Polhovas Gradecas pašvaldība"), ("mr", "डोबोरावा-पोलोहोव\u{94d}ह ग\u{94d}राद\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dobrova–Polhov Gradec Municipality"), ("nb", "Dobrova-Polhow Gradec Kommune"), ("nl", "Dobrova-Polhov Gradec"), ("no", "Dobrova-Polhow Gradec Kommune"), ("pl", "Gmina Dobrova-Polhov Gradec"), ("pt", "Dobrova-Polhov Gradec"), ("ro", "Dobrova-Polhov Gradec"), ("ru", "Доброва-Полхов Градец"), ("si", "ඩොබ\u{dca}රෝව\u{dcf}-පොල\u{dca}හොව\u{dca} ග\u{dca}රඩෙක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Dobrova - Polhov Gradec"), ("sr", "Општина Доброва - Полхов Градец"), ("sr_Latn", "Opština Dobrova - Polhov Gradec"), ("ta", "டோபிரோவ\u{bbe} –பொலஹோவ\u{bcd} கிர\u{bbe}டெக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}బ\u{c4d}ర\u{c4b}వ\u{c3e}-ప\u{c4b}ల\u{c4d}హ\u{c4b}వ\u{c4d} గ\u{c4d}ర\u{c3e}డ\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "แอด ดาค\u{e4c}ฮ\u{e34}ลล\u{e34}ยาร\u{e4c} โกเวอโนเรท"), ("tr", "Dobrova-Polhov"), ("uk", "Доброва-Полхов Градець"), ("vi", "Dobrova-Polhov Gradec"), ("zh", "多布罗瓦-霍尔尤尔-波尔霍夫格拉代茨")]),
                        unofficial_name_list: ["Dobrova-Polhov Gradec"].to_vec(),
                    }
                ),
                (
                    "022",
                    Subdivision{
                        name: "022",
                        country_alpha2: Alpha2::SI,
                        code: "022",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0884386), longitude: Some(14.6424792), max_latitude: Some(46.0914789), min_latitude: Some(46.0849755), max_longitude: Some(14.650608), min_longitude: Some(14.6384198)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دول بري ليوبلياني"), ("bn", "ডল প\u{9cd}রি ল\u{9c1}ব\u{9cd}ল\u{9be}জনি পৌরসভ\u{9be}"), ("ccp", "𑄓\u{1112e}𑄣\u{11134} 𑄛\u{11133}𑄢\u{1112d} 𑄎\u{1112a}𑄚\u{11134}𑄎𑄚\u{11128}"), ("ceb", "Dol pri Ljubljani"), ("cs", "Občina Dol pri Ljubljani"), ("da", "Dol pri Ljubljani"), ("de", "Dol pri Ljubljani"), ("el", "Ντολ πρι Λουμπλτζάνι"), ("en", "Dol pri Ljubljani"), ("es", "Dol pri Ljubljani"), ("fi", "Dol pri Ljubljanin kunta"), ("fr", "Dol pri Ljubljani"), ("gu", "ડોલ પ\u{acd}રી , લ\u{ac1}જબલજાની મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोल प\u{94d}री ल\u{941}ब\u{94d}लियानी नगरपालिका"), ("hr", "Općina Dol pri Ljubljani"), ("id", "Kotamadya Dol pri Ljubljani"), ("it", "Dol pri Ljubljani"), ("ja", "ドル・プリ・リュブリャニ"), ("kn", "ಡಾಲ\u{ccd} ಪ\u{ccd}ರ\u{cbf}ಯ ಲುಜುಬ\u{ccd}ಲಾಜನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "돌프리류블랴니"), ("lt", "Dol pri Liublianos savivaldybė"), ("lv", "Dol Pri Ļjubļaņes pašvaldība"), ("mr", "डोल प\u{94d}री लज\u{94d}ब\u{94d}लजानी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dol pri Ljubljani Municipality"), ("nb", "Dol pri Ljubljani kommune"), ("nl", "Dol pri Ljubljani"), ("no", "Dol pri Ljubljani kommune"), ("pl", "Gmina Dol pri Ljubljani"), ("pt", "Dol pri Ljubljani"), ("ro", "Dol pri Ljubljani"), ("ru", "Доль-при-Любляне"), ("si", "ඩොල\u{dca} ප\u{dca}\u{200d}ර\u{dd2} එල\u{dca}ජ\u{dd4}බ\u{dca}ල\u{dca}ජ\u{dcf}න\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Dol pri Ljubljani"), ("sr", "Општина Дол при Љубљани"), ("sr_Latn", "Opština Dol pri Ljubljani"), ("sv", "Dol pri Ljubljani"), ("ta", "டோல\u{bcd} பிரி லெஜுபில\u{bcd}ஜனி நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}ల\u{c4d} ప\u{c4d}ర\u{c3f} లూజ\u{c3e}బ\u{c3f}ల\u{c4d}జ\u{c3e}న\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโดล พร\u{e34} ล\u{e38}จบ\u{e4c}จาน\u{e34}"), ("tr", "Dol pri Ljubljani Belediyesi"), ("uk", "Дол-при-Любляні"), ("ur", "ڈول پری لجوبلجانی میونسپلٹی"), ("vi", "Dol pri Ljubljani"), ("zh", "卢布尔雅那附近多尔")]),
                        unofficial_name_list: ["Dol pri Ljubljani"].to_vec(),
                    }
                ),
                (
                    "023",
                    Subdivision{
                        name: "023",
                        country_alpha2: Alpha2::SI,
                        code: "023",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1419417), longitude: Some(14.5944171), max_latitude: Some(46.1514012), min_latitude: Some(46.112468), max_longitude: Some(14.6112884), min_longitude: Some(14.5766171)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Дамжале"), ("ccp", "𑄓\u{11127}𑄟\u{11134}𑄎𑄣𑄬"), ("ceb", "Občina Domžale"), ("cs", "Občina Domžale"), ("en", "Domžale"), ("es", "Municipio de Domžale"), ("hr", "Općina Domžale"), ("nl", "Domžale"), ("pl", "Gmina Domžale"), ("ro", "Comuna Domžale"), ("ru", "Домжале"), ("sl", "Občina Domžale"), ("sr", "Општина Домжале"), ("sr_Latn", "Opština Domžale"), ("uk", "Домжале (община)"), ("zh", "多姆扎萊鎮")]),
                        unofficial_name_list: ["Domžale"].to_vec(),
                    }
                ),
                (
                    "024",
                    Subdivision{
                        name: "024",
                        country_alpha2: Alpha2::SI,
                        code: "024",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4356499), longitude: Some(15.952954), max_latitude: Some(46.453094), min_latitude: Some(46.4194043), max_longitude: Some(15.9683311), min_longitude: Some(15.9352057)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دورنافا"), ("bg", "Дорнава"), ("bn", "ডরন\u{9be}ভ\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄓\u{11127}𑄢\u{11134}𑄚𑄞"), ("ceb", "Dornava"), ("cs", "Občina Dornava"), ("da", "Dornava"), ("de", "Dornava"), ("el", "Ντορνάβα"), ("en", "Dornava"), ("es", "Municipalidad del Dornava"), ("fi", "Dornavan kunta"), ("fr", "Dornava"), ("gu", "ડોર\u{acd}નવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोरनावा नगरपालिका"), ("hr", "Općina Dornava"), ("id", "Kotamadya Dornava"), ("it", "Dornava"), ("ja", "ドルナヴァ"), ("kn", "ಡೊರ\u{ccd}ನವ ಪುರಸಭ\u{cc6}"), ("ko", "도르나바"), ("lt", "Dornavos savivaldybė"), ("lv", "Dornavas pašvaldība"), ("mr", "डोर\u{94d}नवा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Dornava"), ("nb", "Dornava kommune"), ("nl", "Dornava"), ("no", "Dornava kommune"), ("pl", "Gmina Dornava"), ("pt", "Dornava"), ("ro", "Dornava"), ("ru", "Дорнава"), ("si", "ඩොර\u{dca}න\u{dcf}ව\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Dornava"), ("sr", "Општина Дорнава"), ("sr_Latn", "Opština Dornava"), ("sv", "Dornava"), ("ta", "டோர\u{bcd}னவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}ర\u{c4d}న\u{c3e}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบอลดอร\u{e4c}นาวา"), ("tr", "Dornava Beledyesi"), ("uk", "Дорнава"), ("ur", "دورناوا میونسپلٹی"), ("vi", "Đô thị tự trị Dornava"), ("zh", "多爾納瓦")]),
                        unofficial_name_list: ["Dornava"].to_vec(),
                    }
                ),
                (
                    "025",
                    Subdivision{
                        name: "025",
                        country_alpha2: Alpha2::SI,
                        code: "025",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.589219), longitude: Some(15.0246021), max_latitude: Some(46.5990855), min_latitude: Some(46.5792596), max_longitude: Some(15.0373244), min_longitude: Some(15.0069603)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية درافوغراد"), ("bn", "ড\u{9cd}র\u{9be}ভোগ\u{9cd}র\u{9be}ড পৌরসভ\u{9be}"), ("ccp", "𑄓\u{11133}𑄢𑄞\u{1112e}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Dravograd"), ("cs", "Občina Dravograd"), ("da", "Dravograd Municipality"), ("de", "Gemeinde Dravograd"), ("el", "Ντραβογκράντ"), ("en", "Dravograd"), ("es", "Dravograd"), ("fi", "Dravogradin kunta"), ("fr", "Dravograd"), ("gu", "દ\u{acd}રાવોગ\u{acd}રાદ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{94d}र\u{947}वोग\u{94d}राद नगरपालिका"), ("hr", "Općina Dravograd"), ("id", "Kotamadya Dravograd"), ("it", "Dravograd"), ("ja", "ドラボグラード"), ("kn", "ದ\u{ccd}ರಾವೋಗ\u{ccd}ರಾಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "드라보그라드 지방 자치제"), ("lt", "Dravogrado savivaldybė"), ("lv", "Dravogradas pašvaldība"), ("mr", "द\u{94d}रोग\u{94d}र\u{945}ड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dravograd Municipality"), ("nb", "Dravograd kommune"), ("nl", "Dravograd"), ("no", "Dravograd kommune"), ("pl", "Gmina Dravograd"), ("pt", "Dravograd"), ("ro", "Comuna Dravograd"), ("ru", "Дравоград"), ("si", "ඩ\u{dca}රවෝග\u{dca}\u{200d}රඩ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Dravograd"), ("sr", "Општина Дравоград"), ("sr_Latn", "Opština Dravograd"), ("sv", "Dravograd kommun"), ("ta", "ட\u{bcd}ரவோக\u{bcd}ர\u{bbe}ட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4d}ర\u{c3e}వ\u{c4b}గ\u{c4d}ర\u{c3e}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ดาวอการ\u{e4c}ด ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Dravograd Belediyesi"), ("uk", "Дравоград"), ("ur", "دراووجراد میونسپلٹی"), ("vi", "Đô thị tự trị Dravograd"), ("zh", "德拉沃格勒鎮")]),
                        unofficial_name_list: ["Dravograd"].to_vec(),
                    }
                ),
                (
                    "026",
                    Subdivision{
                        name: "026",
                        country_alpha2: Alpha2::SI,
                        code: "026",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5185203), longitude: Some(15.7791896), max_latitude: Some(46.5500747), min_latitude: Some(46.468525), max_longitude: Some(15.8177803), min_longitude: Some(15.7124262)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبليك"), ("bg", "Дуплек"), ("bn", "ড\u{9c1}প\u{9cd}লেক পৌরসভ\u{9be}"), ("bs", "Duplek"), ("ccp", "𑄓\u{1112a}𑄛\u{11133}𑄣𑄬𑄇\u{11134}"), ("ceb", "Duplek"), ("cs", "Občina Duplek"), ("da", "Duplek Municipality"), ("de", "Duplek"), ("el", "Ντούπλεκ"), ("en", "Duplek"), ("es", "Duplek"), ("fi", "Duplekin kunta"), ("fr", "Duplek"), ("gu", "ડ\u{ac1}પ\u{acd}લ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ड\u{941}प\u{94d}ल\u{947}क नगरपालिका"), ("hr", "Općina Duplek"), ("id", "Kotamadya Duplek"), ("it", "Duplek"), ("ja", "ドゥプレク"), ("kn", "ಡ\u{ccd}ಯುಪ\u{ccd}ಲ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "두플레크"), ("lt", "Dupleko savivaldybė"), ("lv", "Duplekas pašvaldība"), ("mr", "ड\u{941}प\u{94d}ल\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Duplek Municipality"), ("nb", "Duplek Kommune"), ("nl", "Duplek"), ("no", "Duplek Kommune"), ("pl", "Gmina Duplek"), ("pt", "Duplek"), ("ro", "Duplek"), ("ru", "Дуплек"), ("si", "ඩ\u{dd4}ප\u{dca}ලේක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Duplek"), ("sr", "Општина Дуплек"), ("sr_Latn", "Opština Duplek"), ("sv", "Duplek"), ("ta", "டுப\u{bcd}லெக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4d}యూప\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ด\u{e39}เพล\u{e47}กซ\u{e4c}"), ("tr", "Duplek Belediyesi"), ("uk", "Дуплек"), ("ur", "دوپلیک میونسپلٹی"), ("vi", "Duplek"), ("zh", "杜普莱克")]),
                        unofficial_name_list: ["Duplek"].to_vec(),
                    }
                ),
                (
                    "027",
                    Subdivision{
                        name: "027",
                        country_alpha2: Alpha2::SI,
                        code: "027",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1230068), longitude: Some(14.1151746), max_latitude: Some(46.1809494), min_latitude: Some(46.03694549999999), max_longitude: Some(14.2279654), min_longitude: Some(14.0178293)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غورينيا فاس بوليان"), ("bn", "গোরেঞ\u{9cd}জ\u{9be} ভ\u{9be}স-পোলিয\u{9bc}\u{9be}ন পৌরসভ\u{9be}"), ("bs", "Gorenja vas-Poljane"), ("ccp", "𑄉\u{11127}𑄢𑄬𑄚\u{11134}𑄎 𑄞𑄌\u{11134}-𑄛\u{1112e}𑄣\u{11134}𑄎𑄚𑄬"), ("ceb", "Gorenja Vas-Poljane"), ("cs", "Občina Gorenja vas-Poljane"), ("da", "Gorenja Vas–Poljane Municipality"), ("de", "Gorenja vas-Poljane"), ("el", "Γκορένζα Βας-Πολτζάνε"), ("en", "Gorenja Vas–Poljane"), ("es", "Gorenja vas-Poljane"), ("fi", "Gorenja Vas–Poljanen kunta"), ("fr", "Gorenja vas-Poljane"), ("gu", "ગોર\u{ac7}ન\u{acd}જા વાસ-પોલજ\u{ac7}ન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "गोर\u{947}\u{902}जा वास-पोलिआन\u{947} नगरपालिका"), ("hr", "Općina Gorenja vas - Poljane"), ("id", "Kotamadya Gorenja Vas–Poljane"), ("it", "Gorenja vas-Poljane"), ("kn", "ಗೊರ\u{cc6}ಂಜಾ ವಾಸ\u{ccd}-ಪೋಲ\u{ccd}ಜೇನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "고레냐바스폴랴네"), ("lt", "Gorenja vas Poljanės savivaldybė"), ("lv", "Gorenjas Vas–Poljanes pašvaldība"), ("mr", "गोर\u{947}न\u{94d}जा वास-पोळजण\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gorenja Vas–Poljane Municipality"), ("nb", "Gorenja Vas-Poljane kommune"), ("nl", "Gorenja vas-Poljane"), ("no", "Gorenja Vas-Poljane kommune"), ("pl", "Gmina Gorenja vas-Poljane"), ("pt", "Gorenja vas-Poljane"), ("ro", "Gorenja vas-Poljane"), ("ru", "Горенья-вас-Поляне"), ("si", "ගොර\u{dca}න\u{dca}ජ\u{dcf} ව\u{dcf}ස\u{dca}-පොල\u{dca}ජනේ නගර සභ\u{dcf}ව"), ("sl", "Občina Gorenja vas-Poljane"), ("sr", "Општина Горења Вас - Пољане"), ("sr_Latn", "Opština Gorenja Vas - Poljane"), ("sv", "Gorenja Vas-Poljane kommun"), ("ta", "கோரிஞ\u{bcd}ச\u{bbe} வ\u{bbe}ஸ\u{bcd} –பொலஜனே நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4b}ర\u{c46}ంజ\u{c3e} వ\u{c3e}స\u{c4d}-ప\u{c4b}ల\u{c4d}జ\u{c3e}న\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโมเรนจา วาส โพลเจน"), ("tr", "Gorenja Vas-Poljane Belediyesi"), ("uk", "Гореня вас-Поляне"), ("vi", "Đô thị tự trị Gorenja Vas–Poljane"), ("zh", "戈雷尼亚村-波利亚内")]),
                        unofficial_name_list: ["Gorenja vas-Poljane"].to_vec(),
                    }
                ),
                (
                    "028",
                    Subdivision{
                        name: "028",
                        country_alpha2: Alpha2::SI,
                        code: "028",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4120271), longitude: Some(16.0133089), max_latitude: Some(46.4225317), min_latitude: Some(46.4012183), max_longitude: Some(16.0357271), min_longitude: Some(15.9939354)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غوريشنيكا"), ("bn", "গোরিস\u{9cd}নিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄉\u{1112e}𑄢\u{11128}𑄌\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Občina Gorišnica"), ("cs", "Občina Gorišnica"), ("da", "Gorišnica Municipality"), ("de", "Gorišnica"), ("el", "Γκορισνίκα"), ("en", "Gorišnica"), ("es", "Municipalidad del Gorišnica"), ("fi", "Gorišnican kunta"), ("fr", "Gorišnica"), ("gu", "ગોરીસ\u{acd}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "गोरिस\u{94d}निका नगरपालिका"), ("hr", "Općina Gorišnica"), ("id", "Kotamadya Gorišnica"), ("it", "Gorišnica"), ("ja", "ゴリシュニツァ"), ("kn", "ಗೋರ\u{cbf}ಸ\u{ccd}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "고리슈니차"), ("lt", "Gorišnicos savivaldybė"), ("lv", "Gorišnicas pašvaldība"), ("mr", "गोरीशिका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gorisnica Municipality"), ("nb", "Gorisnica kommune"), ("nl", "Gorišnica"), ("no", "Gorisnica kommune"), ("pl", "Gmina Gorišnica"), ("pt", "Gorišnica"), ("ro", "Gorišnica"), ("ru", "Горишница"), ("si", "ගොර\u{dd2}ස\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Gorišnica"), ("sr", "Општина Горишница"), ("sr_Latn", "Opština Gorišnica"), ("sv", "Gorisnica kommun"), ("ta", "கோரிசனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4b}ర\u{c4d}స\u{c3f}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "กอร\u{e34}สน\u{e34}กา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Gorisnica Belediyesi"), ("uk", "Горишниця"), ("ur", "جوریسنیکا میونسپلٹی"), ("vi", "Đô thị tự trị Gorisnica"), ("zh", "戈里什尼察")]),
                        unofficial_name_list: ["Gorišnica"].to_vec(),
                    }
                ),
                (
                    "029",
                    Subdivision{
                        name: "029",
                        country_alpha2: Alpha2::SI,
                        code: "029",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6767099), longitude: Some(15.9910846), max_latitude: Some(46.6846593), min_latitude: Some(46.6586933), max_longitude: Some(16.0169898), min_longitude: Some(15.9758497)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄉\u{1112e}𑄢\u{11134}𑄎 𑄢𑄖\u{11134}𑄉\u{1112e}𑄚"), ("ceb", "Gornja Radgona"), ("cs", "Občina Gornja Radgona"), ("en", "Gornja Radgona"), ("hr", "Općina Gornja Radgona"), ("it", "Gornja Radgona"), ("nl", "Gornja Radgona"), ("pl", "Gmina Gornja Radgona"), ("ro", "Comuna Gornja Radgona"), ("sl", "Občina Gornja Radgona"), ("sr", "Општина Горња Радгона"), ("sr_Latn", "Opština Gornja Radgona"), ("uk", "Горня Радгона"), ("zh", "上拉德戈納鎮")]),
                        unofficial_name_list: ["Gornja Radgona"].to_vec(),
                    }
                ),
                (
                    "030",
                    Subdivision{
                        name: "030",
                        country_alpha2: Alpha2::SI,
                        code: "030",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2961712), longitude: Some(14.8062347), max_latitude: Some(46.3124327), min_latitude: Some(46.2533725), max_longitude: Some(14.834534), min_longitude: Some(14.7926643)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غورني غراد"), ("bn", "গর\u{9cd}নজি গ\u{9cd}র\u{9be}দ পৌরসভ\u{9be}"), ("ccp", "𑄉\u{1112e}𑄢\u{11134}𑄎\u{11128} 𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Gornji Grad"), ("cs", "Občina Gornji Grad"), ("da", "Gornji Grad Municipality"), ("de", "Gornji Grad"), ("el", "Γκόρντζι Γκράντ"), ("en", "Gornji Grad"), ("es", "Municipalidad del Gornji Grad"), ("fi", "Gornji Gradin kunta"), ("fr", "Gornji Grad"), ("gu", "ગોર\u{acd}ન\u{acd}જી ગ\u{acd}રાદ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "गोर\u{94d}न\u{94d}जी ग\u{94d}र\u{948}ड नगर पालिका"), ("hr", "Općina Gornji Grad"), ("id", "Kotamadya Gornji Grad"), ("it", "Gornji Grad"), ("ja", "ゴルニ・グラード"), ("kn", "ಗೊರ\u{ccd}ನ\u{ccd}ಜ\u{cbf} ಗ\u{ccd}ರಾಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "고르니그라드"), ("lt", "Gorni Grado savivaldybė"), ("lv", "Gornji Gradas pašvaldība"), ("mr", "गोर\u{94d}न\u{94d}जी ग\u{94d}राद म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Gornji Grad"), ("nb", "Gornij Grand kommune"), ("nl", "Gornji Grad"), ("no", "Gornij Grand kommune"), ("pl", "Gmina Gornji Grad"), ("pt", "Gornji Grad"), ("ro", "Gornji Grad"), ("ru", "Горний Град"), ("si", "ගොන\u{dca}ජ\u{dd2} ග\u{dca}රඩ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Gornji Grad"), ("sr", "Општина Горњи Град"), ("sr_Latn", "Opština Gornji Grad"), ("sv", "Gornij Grand kommun"), ("ta", "கோரஞ\u{bcd}சி க\u{bcd}ர\u{bbe}ட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4b}ర\u{c4d}స\u{c3f}ంజ\u{c3f} గ\u{c4d}ర\u{c3e}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "กอนจ\u{e34} กราด ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Gornji Grad Belediyesi"), ("uk", "Горній Град (община)"), ("ur", "جورنجی گراڈ میونسپلٹی"), ("vi", "Đô thị tự trị Gornji Grad"), ("zh", "戈爾尼格勒")]),
                        unofficial_name_list: ["Gornji Grad"].to_vec(),
                    }
                ),
                (
                    "031",
                    Subdivision{
                        name: "031",
                        country_alpha2: Alpha2::SI,
                        code: "031",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8047535), longitude: Some(16.2180722), max_latitude: Some(46.8182835), min_latitude: Some(46.7921829), max_longitude: Some(16.2325301), min_longitude: Some(16.1923273)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غورني بيتروفتسي"), ("bn", "গরনি পেত\u{9cd}রভচি পৌরসভ\u{9be}"), ("ccp", "𑄉\u{1112e}𑄢\u{11134}𑄎\u{11128} 𑄛𑄬𑄑\u{11133}𑄢\u{1112e}𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Gornji Petrovci"), ("cs", "Občina Gornji Petrovci"), ("da", "Gornji Petrovci Municipality"), ("de", "Gemeinde Gornji Petrovci"), ("el", "Γκόρντζι Πετρόβσκι"), ("en", "Gornji Petrovci"), ("es", "Municipalidad del Gornji Petrovci"), ("fi", "Gornji Petrovcin kunta"), ("fr", "Municipalité de Gornji Petrovci"), ("gu", "ગોર\u{acd}નજી પ\u{ac7}ટ\u{acd}રોવસિ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "गोर\u{94d}नजी प\u{947}त\u{94d}रोव\u{94d}की नगरपालिका"), ("hr", "Općina Gornji Petrovci"), ("id", "Kotamadya Gornji Petrovci"), ("it", "Gornji Petrovci"), ("ja", "ゴルニ・ペトロフツィ"), ("kn", "ಗೊರ\u{ccd}ನ\u{ccd}ಜ\u{cbf} ಪ\u{cc6}ಟ\u{ccd}ರೋವ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "고르니페트로브치 지방 자치제"), ("lt", "Gornio Petrovco savivaldybė"), ("lv", "Gornji Petrovcu pašvaldība"), ("mr", "गोर\u{94d}नजी प\u{947}ट\u{94d}रोव\u{94d}हिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Gornji Petrovci Municipality"), ("nb", "Gornji kommune"), ("nl", "Gornji Petrovci"), ("no", "Gornji kommune"), ("pl", "Gmina Gornji Petrovci"), ("pt", "Município de Gornji Petrovci"), ("ro", "Comuna Gornji Petrovci"), ("ru", "Горни-Петровцы"), ("si", "ගොර\u{dca}න\u{dca}ජේ පෙට\u{dca}\u{200d}රොවෙස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Gornji Petrovci"), ("sr", "Општина Горњи Петровци"), ("sr_Latn", "Opština Gornji Petrovci"), ("sv", "Gornji kommun"), ("ta", "கோரஞ\u{bcd}இ பெட\u{bcd}ரோவ\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4b}రంజ\u{c40} ప\u{c46}ట\u{c4d}ర\u{c4b}వస\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องกอร\u{e4c}นจ\u{e34} พ\u{e35}โทรซ\u{e34}"), ("tr", "Gornji Petrovci Belediyesi"), ("uk", "Горні Петровці"), ("ur", "جورنجی پیترووکی میونسپلٹی"), ("vi", "Đô thị tự trị Gornji Petrovci"), ("zh", "上彼得羅夫齊鎮")]),
                        unofficial_name_list: ["Gornji Petrovci"].to_vec(),
                    }
                ),
                (
                    "032",
                    Subdivision{
                        name: "032",
                        country_alpha2: Alpha2::SI,
                        code: "032",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9557645), longitude: Some(14.658899), max_latitude: Some(45.9878805), min_latitude: Some(45.9430472), max_longitude: Some(14.6758637), min_longitude: Some(14.6435568)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غروسوبلي"), ("be", "Грасупле"), ("bn", "গ\u{9cd}রোস\u{9c1}পজে পৌরসভ\u{9be}"), ("ccp", "𑄉\u{11133}𑄢\u{1112e}𑄥𑄛\u{11134}𑄎𑄬"), ("ceb", "Grosuplje"), ("cs", "Občina Grosuplje"), ("da", "Grosuplje Municipality"), ("de", "Gemeinde Grosuplje"), ("el", "Γκροσουπλτζέ"), ("en", "Grosuplje"), ("es", "Municipalidad del Grosuplje"), ("fi", "Grosupljen kunta"), ("fr", "Municipalité de Grosuplje"), ("gu", "ગ\u{acd}રોસપ\u{acd}લજ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ग\u{94d}रोस\u{941}प\u{94d}लिए नगर पालिका"), ("hr", "Općina Grosuplje"), ("id", "Kotamadya Grosuplje"), ("it", "Grosuplje"), ("ja", "グロースプリェ"), ("kn", "ಗ\u{ccd}ರೊಸುಪ\u{ccd}ಲ\u{cc6}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "그로수플예 지방 자치제"), ("lt", "Grosuplės savivaldybė"), ("lv", "Grosupljes pašvaldība"), ("mr", "ग\u{94d}रॉसप\u{94d}लज\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Grosuplje"), ("nb", "Grosuplje kommune"), ("nl", "Grosuplje"), ("no", "Grosuplje kommune"), ("pl", "Gmina Grosuplje"), ("pt", "Município de Grosuplje"), ("ro", "Comuna Grosuplje"), ("ru", "Гросупле"), ("si", "ග\u{dca}රෝස\u{dca}ප\u{dca}ල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Grosuplje"), ("sr", "Гросупље"), ("sr_Latn", "Grosuplje"), ("sv", "Grosuplje kommun"), ("ta", "கிரோசுப\u{bcd}லஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4d}ర\u{c4b}సుపుల\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "กรอสซ\u{e31}พจ\u{e35} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Grospulje Belediyesi"), ("uk", "Гросуплє (община)"), ("ur", "جروسوپلجی میونسپلٹی"), ("vi", "Đô thị tự trị Grosuplje"), ("zh", "格羅蘇普列鎮")]),
                        unofficial_name_list: ["Grosuplje"].to_vec(),
                    }
                ),
                (
                    "033",
                    Subdivision{
                        name: "033",
                        country_alpha2: Alpha2::SI,
                        code: "033",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8235919), longitude: Some(16.2881649), max_latitude: Some(46.8480024), min_latitude: Some(46.8010138), max_longitude: Some(16.3113042), min_longitude: Some(16.2538269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تشالوفتسي"), ("bn", "স\u{9cd}য\u{9be}লোভচি"), ("bs", "Šalovci"), ("ccp", "𑄥𑄣\u{1112e}𑄌\u{11134}𑄥\u{11128}"), ("ceb", "Občina Šalovci"), ("cs", "Občina Šalovci"), ("da", "Šalovci"), ("de", "Šalovci"), ("el", "Σαλόβκι"), ("en", "Šalovci"), ("es", "Šalovci"), ("fi", "Šalovci"), ("fr", "Šalovci"), ("gu", "સાલોવ\u{acd}કિ"), ("hi", "सलोव\u{94d}की"), ("hr", "Šalovci, općina"), ("hu", "Sal"), ("id", "Šalovci"), ("it", "Šalovci"), ("ja", "シャロフツィ"), ("kn", "ಸಾಲೋವ\u{cbf}"), ("ko", "샬로브치"), ("lt", "Šalovcis"), ("lv", "Šalovci"), ("mr", "स\u{94d}लोवसि"), ("ms", "Salovci"), ("nb", "Salovci"), ("nl", "Šalovci"), ("no", "Salovci"), ("pl", "Gmina Šalovci"), ("pt", "Šalovci"), ("ro", "Comuna Šalovci"), ("ru", "Шаловци"), ("si", "සලොව\u{dca}ස\u{dd2}"), ("sl", "Občina Šalovci"), ("sr", "Општина Шаловци"), ("sr_Latn", "Opština Šalovci"), ("sv", "Šalovci"), ("ta", "ச\u{bcd}லோவசி"), ("te", "స\u{c3e}ల\u{c4b}వ\u{c4d}స\u{c3f}"), ("th", "ซาลอวค\u{e34}"), ("tr", "Salovci"), ("uk", "Шаловці"), ("ur", "سالووکی"), ("vi", "Šalovci"), ("zh", "沙洛夫齊")]),
                        unofficial_name_list: ["Šalovci"].to_vec(),
                    }
                ),
                (
                    "034",
                    Subdivision{
                        name: "034",
                        country_alpha2: Alpha2::SI,
                        code: "034",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1417288), longitude: Some(15.0844894), max_latitude: Some(46.1557132), min_latitude: Some(46.1138134), max_longitude: Some(15.1043942), min_longitude: Some(15.0682763)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هراستنيك"), ("bn", "হ\u{9cd}র\u{9be}সৎনিক পৌরসভ\u{9be}"), ("ccp", "𑄦\u{11133}𑄢𑄌\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Hrastnik"), ("cs", "Občina Hrastnik"), ("da", "Hrastnik"), ("de", "Gemeinde Hrastnik"), ("el", "Χράστνικ"), ("en", "Hrastnik"), ("es", "Municipalidad del Hrastnik"), ("fi", "Hrastnikin kunta"), ("fr", "Municipalité de Hrastnik"), ("gu", "હ\u{acd}રાસ\u{acd}તનિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ह\u{94d}रास\u{94d}तनिक नगर पालिका"), ("hr", "Općina Hrastnik"), ("id", "Kotamadya Hrastnik"), ("it", "Hrastnik"), ("ja", "フラストニク"), ("kn", "ಹ\u{ccd}ರಾಸ\u{ccd}ಟ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "흐라스트니크 지방 자치제"), ("lt", "Hrastniko savivaldybė"), ("lv", "Hrastnikas pašvaldība"), ("mr", "हरास\u{94d}तनीक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Hrastnik Municipality"), ("nb", "Hrastnik Kommune"), ("nl", "Hrastnik"), ("no", "Hrastnik Kommune"), ("pl", "Gmina Hrastnik"), ("pt", "Município de Hrastnik"), ("ro", "Comuna Hrastnik"), ("ru", "Храстник"), ("si", "හ\u{dca}රස\u{dca}ට\u{dca}න\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Hrastnik"), ("sr", "Општина Храстник"), ("sr_Latn", "Opština Hrastnik"), ("sv", "Hrastnik (kommun)"), ("ta", "ஹரசட\u{bcd}னிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "హ\u{c4d}ర\u{c3e}స\u{c4d}ట\u{c4d}న\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องฮราสท\u{e4c}น\u{e34}ค"), ("tr", "Hrastnik Belediyesi"), ("uk", "Храстник (община)"), ("ur", "حراستنیک میونسپلٹی"), ("vi", "Đô thị tự trị Hrastnik"), ("zh", "赫拉斯特尼克鎮")]),
                        unofficial_name_list: ["Hrastnik"].to_vec(),
                    }
                ),
                (
                    "035",
                    Subdivision{
                        name: "035",
                        country_alpha2: Alpha2::SI,
                        code: "035",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5945539), longitude: Some(14.0056771), max_latitude: Some(45.6444312), min_latitude: Some(45.48086869999999), max_longitude: Some(14.1316015), min_longitude: Some(13.8687912)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هربيليه - كوزينا"), ("bn", "হ\u{9cd}র\u{9be}র\u{9be}পেলি-কোজিন\u{9be} পৌরসভ\u{9be}"), ("bs", "Hrpelje-Kozina"), ("ccp", "𑄦\u{11133}𑄢𑄛𑄬𑄣\u{11134}𑄎𑄬-𑄇\u{1112e}𑄎\u{11128}𑄚"), ("ceb", "Hrpelje-Kozina"), ("cs", "Občina Hrpelje-Kozina"), ("da", "Hrpelje-Kozina"), ("de", "Hrpelje-Kozina"), ("el", "Χρπέλτζε-Κοζίνα"), ("en", "Hrpelje–Kozina"), ("es", "Municipalidad del Hrpelje-Kozina"), ("fi", "Hrpelje–Kozinan kunta"), ("fr", "Hrpelje-Kozina"), ("gu", "હર\u{acd}પ\u{ac7}લજ\u{ac7}-કોઝિના , મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ह\u{94d}रप\u{947}लिए-कोज\u{93c}िना नगरपालिका"), ("hr", "Općina Hrpelje - Kozina"), ("id", "Kotamadya Hrpelje–Kozina"), ("it", "Erpelle-Cosina"), ("kn", "ಹರ\u{ccd}ಪ\u{cc6}ಲ\u{ccd}ಜ\u{cc6}-ಕೋಝ\u{cbf}ನಾ ಪುರಸಭ\u{cc6}"), ("ko", "흐르펠레코지나"), ("lt", "Hrpelje-Kozinos savivaldybė"), ("lv", "Hrpelje–Kozinas pašvaldība"), ("mr", "हरप\u{947}लज\u{947}-कोझीन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Hrpelje–Kozina Municipality"), ("nb", "Hrpelje Kozina Kommune"), ("nl", "Hrpelje-Kozina"), ("no", "Hrpelje Kozina Kommune"), ("pl", "Gmina Hrpelje-Kozina"), ("pt", "Hrpelje-Kozina"), ("ro", "Hrpelje-Kozina"), ("ru", "Хрпелье-Козина"), ("si", "පෙල\u{dd2}යේ-කෝස\u{dd3}න\u{dcf} නගරසභ\u{dcf}ව"), ("sl", "Občina Hrpelje - Kozina"), ("sr", "Општина Хрпеље - Козина"), ("sr_Latn", "Opština Hrpelje - Kozina"), ("sv", "Hrpelje-Kozina"), ("ta", "ஹர\u{bcd}பெல\u{bcd}ஜெ –கூசின நகர\u{bbe}ட\u{bcd}சி"), ("te", "హ\u{c46}ర\u{c4d}ప\u{c46}ల\u{c4d}జ\u{c46}-క\u{c4b}జ\u{c3f}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องฮาลเพเจ โคซ\u{e34}นา"), ("tr", "Hrpelje-Kozina"), ("uk", "Хрпелє-Козіна"), ("vi", "Hrpelje-Kozina"), ("zh", "赫尔佩列-科济纳")]),
                        unofficial_name_list: ["Hrpelje-Kozina"].to_vec(),
                    }
                ),
                (
                    "036",
                    Subdivision{
                        name: "036",
                        country_alpha2: Alpha2::SI,
                        code: "036",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.00294539999999), longitude: Some(14.0278459), max_latitude: Some(46.0273159), min_latitude: Some(45.9806064), max_longitude: Some(14.057244), min_longitude: Some(13.9910771)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيدرييا"), ("ccp", "𑄃\u{11128}𑄓\u{11133}𑄢\u{11128}𑄎"), ("ceb", "Idrija"), ("cs", "Občina Idrija"), ("en", "Idrija"), ("hr", "Općina Idrija"), ("it", "Idria"), ("nl", "Idrija"), ("pl", "Gmina Idrija"), ("ro", "Comuna Idrija"), ("sl", "Občina Idrija"), ("sr", "Општина Идрија"), ("sr_Latn", "Opština Idrija"), ("zh", "伊德里亞鎮")]),
                        unofficial_name_list: ["Idrija"].to_vec(),
                    }
                ),
                (
                    "037",
                    Subdivision{
                        name: "037",
                        country_alpha2: Alpha2::SI,
                        code: "037",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9588213), longitude: Some(14.5282718), max_latitude: Some(45.99051650000001), min_latitude: Some(45.9171235), max_longitude: Some(14.5590687), min_longitude: Some(14.5129422)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية إيغ"), ("bn", "ইগ পৌরসভ\u{9be}"), ("ccp", "𑄃\u{11128}𑄇\u{11134}"), ("ceb", "Ig"), ("cs", "Občina Ig"), ("da", "Ig Municipality"), ("de", "Ig"), ("el", "Ιγκ"), ("en", "Ig"), ("es", "Municipalidad del Ig"), ("fi", "Ign kunta"), ("fr", "Ig"), ("gu", "આઈજી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "आइग नगरपालिका"), ("hr", "Općina Ig"), ("hu", "Ig"), ("id", "Kotamadya Ig"), ("it", "Ig"), ("ja", "イグ"), ("kn", "ಇಗ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "이그"), ("lt", "Igas"), ("lv", "Igas pašvaldība"), ("mr", "आयजी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Municipality of Ig"), ("nb", "Ig Kommune"), ("nl", "Ig"), ("no", "Ig Kommune"), ("pl", "Gmina Ig"), ("pt", "Ig"), ("ro", "Ig"), ("ru", "Иг"), ("si", "ඉග\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Ig"), ("sr", "Иг (град)"), ("sr_Latn", "Ig (grad)"), ("sv", "Ig (kommun)"), ("ta", "இக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఇగ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องอ\u{e34}ก"), ("tr", "Ig Belediyes"), ("uk", "Іг"), ("ur", "یج میونسپلٹی"), ("vi", "Đô thị tự trị Ig"), ("zh", "伊格鎮")]),
                        unofficial_name_list: ["Ig"].to_vec(),
                    }
                ),
                (
                    "038",
                    Subdivision{
                        name: "038",
                        country_alpha2: Alpha2::SI,
                        code: "038",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.570099), longitude: Some(14.2418616), max_latitude: Some(45.5976891), min_latitude: Some(45.5553652), max_longitude: Some(14.3981604), min_longitude: Some(14.2257751)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيليرسكا بيتريسا"), ("ccp", "𑄃\u{11128}𑄣\u{11128}𑄢\u{11134}𑄇 𑄝\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄇"), ("ceb", "Ilirska Bistrica"), ("cs", "Občina Ilirska Bistrica"), ("de", "Ilirska Bistrica"), ("en", "Ilirska Bistrica"), ("es", "Municipio de Ilirska Bistrica"), ("hr", "Općina Ilirska Bistrica"), ("id", "Kota praja Ilirska Bistrica"), ("nl", "Ilirska Bistrica"), ("ro", "Ilirska Bistrica"), ("sl", "Občina Ilirska Bistrica"), ("sr", "Општина Илирска Бистрица"), ("sr_Latn", "Opština Ilirska Bistrica"), ("uk", "Ілірська Бистриця (община)"), ("zh", "伊利爾斯卡比斯特里察鎮")]),
                        unofficial_name_list: ["Ilirska Bistrica"].to_vec(),
                    }
                ),
                (
                    "039",
                    Subdivision{
                        name: "039",
                        country_alpha2: Alpha2::SI,
                        code: "039",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9395841), longitude: Some(14.8047626), max_latitude: Some(45.9464556), min_latitude: Some(45.9342078), max_longitude: Some(14.8189485), min_longitude: Some(14.7943209)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية إيفانتسا غوريكا"), ("bn", "ইভ\u{9be}ঙ\u{9cd}ক\u{9be} গোরিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄃\u{11128}𑄞𑄚\u{11134}𑄇𑄚 𑄉\u{1112e}𑄢\u{11128}𑄇"), ("ceb", "Občina Ivančna Gorica"), ("cs", "Občina Ivančna Gorica"), ("da", "Ivančna Gorica Municipality"), ("de", "Ivančna Gorica"), ("el", "Ιβάνκνα Γκορίκα"), ("en", "Ivančna Gorica"), ("es", "Municipalidad del Ivančna Gorica"), ("fi", "Ivančna Gorican kunta"), ("fr", "Ivančna Gorica"), ("gu", "ઈવાનકના ગોરિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "इवानकना गोरिका नगर पालिका"), ("hr", "Općina Ivančna Gorica"), ("id", "Kotamadya Ivančna Gorica"), ("it", "Ivančna Gorica"), ("ja", "イヴァンチュナ・ゴリツァ"), ("kn", "ಇವಾನ\u{ccd}ಕ\u{ccd}ನಾ ಗೋರ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "이반치나고리차"), ("lt", "Ivančna Gorica"), ("lv", "Ivančna Gorica pašvaldība"), ("mr", "इवानसना गोरीच म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ivancna Gorica Municipality"), ("nb", "Ivancna Gorica Kommune"), ("nl", "Ivančna Gorica"), ("no", "Ivancna Gorica Kommune"), ("pl", "Gmina Ivančna Gorica"), ("pt", "Ivančna Gorica"), ("ro", "Ivančna Gorica"), ("ru", "Иванчна Горица"), ("si", "ඉවන\u{dca}ක\u{dca}න\u{dcf} ගොර\u{dca}ස\u{dd2}ය\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Ivančna Gorica"), ("sr", "Општина Иванчна Горица"), ("sr_Latn", "Opština Ivančna Gorica"), ("sv", "Ivancna Gorica Kommun"), ("ta", "இவ\u{bcd}நகின கோரிக\u{bcd}க நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఐవ\u{c3e}న\u{c3e} గ\u{c4b}ర\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องไอแวนซ\u{e4c}นา โกร\u{e34}คา"), ("tr", "Ivančna Gorica Belediyesi"), ("uk", "Іванчна Гориця (община)"), ("ur", "یوانکنا جوریکا میونسپلٹی"), ("vi", "Ivančna Gorica"), ("zh", "伊万奇纳戈里察")]),
                        unofficial_name_list: ["Ivancna Gorica"].to_vec(),
                    }
                ),
                (
                    "040",
                    Subdivision{
                        name: "040",
                        country_alpha2: Alpha2::SI,
                        code: "040",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5374048), longitude: Some(13.6600802), max_latitude: Some(45.5481856), min_latitude: Some(45.5142565), max_longitude: Some(13.6940814), min_longitude: Some(13.6463316)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيزولا"), ("bn", "ইজ\u{9c1}ল\u{9be}"), ("bs", "Izola"), ("ca", "Izola"), ("ccp", "𑄃\u{11128}𑄎\u{1112e}𑄣\u{11134}"), ("ceb", "Izola"), ("cs", "Občina Izola"), ("da", "Izola"), ("de", "Izola"), ("el", "Ίζολα"), ("en", "Izola"), ("es", "Izola"), ("eu", "Izola"), ("fa", "ایزولا"), ("fi", "Izola"), ("fr", "Izola"), ("ga", "Izola"), ("gu", "ઇઝોલા"), ("he", "איזולה"), ("hi", "इज\u{93c}ोला"), ("hr", "Općina Izola"), ("hu", "Izola"), ("id", "Izola"), ("it", "Isola"), ("ja", "イゾラ"), ("ka", "იზოლა"), ("kn", "ಇಝೋಲಾ"), ("ko", "이졸라"), ("lt", "Izola"), ("lv", "Izola"), ("mk", "Изола"), ("mr", "इज\u{93c}ोला"), ("ms", "Izola"), ("nb", "Izola"), ("nl", "Izola"), ("no", "Izola"), ("pl", "Gmina Izola"), ("pt", "Izola"), ("ro", "Comuna Izola"), ("ru", "Изола"), ("si", "ඉසොල\u{dcf}"), ("sk", "Izola"), ("sl", "Občina Izola"), ("sr", "Општина Изола"), ("sr_Latn", "Opština Izola"), ("sv", "Izola"), ("ta", "இஸ\u{bcd}யோல\u{bbe}"), ("te", "ఇజ\u{c4b}ల\u{c3e}"), ("th", "เม\u{e37}องม\u{e34}ซาม\u{e34}ส ออคซ\u{e34}เดนต\u{e31}ล"), ("tr", "Izola"), ("uk", "Ізола"), ("ur", "یزولا"), ("vi", "Izola"), ("zh", "伊佐拉")]),
                        unofficial_name_list: ["Izola/Isola"].to_vec(),
                    }
                ),
                (
                    "041",
                    Subdivision{
                        name: "041",
                        country_alpha2: Alpha2::SI,
                        code: "041",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4367047), longitude: Some(14.0526057), max_latitude: Some(46.4501715), min_latitude: Some(46.4210423), max_longitude: Some(14.0812469), min_longitude: Some(14.0084605)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية يسينيتسه"), ("bg", "Есенице"), ("bn", "জেসেনিচ পৌরসভ\u{9be}"), ("bs", "Jesenice"), ("ccp", "𑄎𑄬𑄥𑄬𑄚\u{1112d}𑄌\u{11134}"), ("ceb", "Jesenice"), ("cs", "Občina Jesenice"), ("da", "Jesenice"), ("de", "Jesenice"), ("el", "Τζεσενίκε"), ("en", "Jesenice"), ("es", "Jesenice"), ("fi", "Jesenicen kunta"), ("fr", "Jesenice"), ("gu", "જ\u{ac7}સ\u{ac7}નિસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{947}स\u{947}नीक नगरपालिका"), ("hr", "Jesenice"), ("hu", "Jesenice"), ("id", "Jesenice"), ("it", "Jesenice"), ("ja", "イェセニツェ"), ("kn", "ಜ\u{cc6}ಸ\u{ccd}ಸ\u{cc6}ನ\u{cbf}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "예세니체"), ("lt", "Jesenicės"), ("lv", "Jesenices pašvaldība"), ("mk", "Јесенице"), ("mr", "ज\u{945}स\u{947}नसा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jesenice Municipality"), ("nb", "Jesenice"), ("nl", "Jesenice"), ("no", "Jesenice"), ("pl", "Gmina Jesenice"), ("pt", "Jesenice"), ("ro", "Jesenice"), ("ru", "Есенице"), ("si", "ජෙසෙනය\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Jesenice"), ("sl", "Občina Jesenice"), ("sr", "Јесенице"), ("sr_Latn", "Jesenice"), ("sv", "Jesenice"), ("ta", "ஜெசெனிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జస\u{c46}న\u{c48}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องซ\u{e39}วา เรกา"), ("tr", "Jesenice Belediyesi"), ("uk", "Єсеніце"), ("ur", "بلدیہ یسینیتسے"), ("vi", "Jesenice, Slovenia"), ("zh", "耶塞尼采")]),
                        unofficial_name_list: ["Jesenice"].to_vec(),
                    }
                ),
                (
                    "042",
                    Subdivision{
                        name: "042",
                        country_alpha2: Alpha2::SI,
                        code: "042",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4850899), longitude: Some(15.9714564), max_latitude: Some(46.5012097), min_latitude: Some(46.4791653), max_longitude: Some(15.9925779), min_longitude: Some(15.9625412)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية يورشينتسي"), ("bn", "জ\u{9c1}রসিঞ\u{9cd}চি পৌরসভ\u{9be}"), ("ccp", "𑄎𑄢\u{11134}𑄥\u{11128}𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Občina Juršinci"), ("cs", "Občina Juršinci"), ("da", "Juršinci Municipality"), ("de", "Juršinci"), ("el", "Τζουρσίνκι"), ("en", "Juršinci"), ("es", "Juršinci"), ("fi", "Juršincin kunta"), ("fr", "Juršinci"), ("gu", "જ\u{ac1}ર\u{acd}સિ\u{a82}કી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{941}र\u{94d}सि\u{902}की नगरपालिका"), ("hr", "Općina Juršinci"), ("id", "Kotamadya Juršinci"), ("it", "Juršinci"), ("ja", "ユルシンツィ"), ("kn", "ಜುರ\u{ccd}ಸ\u{cbf}ನ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "유르신치"), ("lt", "Juršincio savivaldybė"), ("lv", "Juršincu pašvaldība"), ("mr", "ज\u{941}र\u{94d}निस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jursinci Municipality"), ("nb", "Jursinci kommune"), ("nl", "Juršinci"), ("no", "Jursinci kommune"), ("pl", "Gmina Juršinci"), ("pt", "Juršinci"), ("ro", "Juršinci"), ("ru", "Юршинци"), ("si", "ජර\u{dca}ස\u{dd2}න\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Juršinci"), ("sr", "Општина Јуршинци"), ("sr_Latn", "Opština Juršinci"), ("sv", "Juršinci"), ("ta", "ஜூரஸின\u{bcd}ஸி நகர\u{bbe}ட\u{bcd}சி"), ("te", "జుర\u{c4d}స\u{c3f}ంచ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ย\u{e39}ร\u{e4c}ซ\u{e34}ซ\u{e35}"), ("tr", "Jursinca Belediyesi"), ("uk", "Юршинці"), ("ur", "جورسینکی میونسپلٹی"), ("vi", "Juršinci"), ("zh", "尤尔欣齐")]),
                        unofficial_name_list: ["Juršinci"].to_vec(),
                    }
                ),
                (
                    "043",
                    Subdivision{
                        name: "043",
                        country_alpha2: Alpha2::SI,
                        code: "043",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2221964), longitude: Some(14.6072968), max_latitude: Some(46.245163), min_latitude: Some(46.1980487), max_longitude: Some(14.6428399), min_longitude: Some(14.5902802)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كامنيك"), ("bn", "ক\u{9be}ম\u{9cd}নিক পৌরসভ\u{9be}"), ("ccp", "𑄇𑄟\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Kamnik"), ("cs", "Občina Kamnik"), ("da", "Kamnik Municipality"), ("de", "Gemeinde Kamnik"), ("el", "Καμνίκ"), ("en", "Kamnik"), ("es", "Municipalidad Kamnik"), ("fi", "Kamnikin kunta"), ("fr", "Kamnik"), ("gu", "કમ\u{acd}નીક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कम\u{94d}निक नगरपालिका"), ("hr", "Općina Kamnik"), ("id", "Kotamadya Kamnik"), ("it", "Kamnik"), ("ja", "カムニーク"), ("kn", "ಕಮ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "캄니크 지방 자치제"), ("lt", "Kamniko savivaldybė"), ("lv", "Kamnikas pašvaldība"), ("mr", "कारणिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kamnik Municipality"), ("nb", "Kamnik kommune"), ("nl", "Kamnik"), ("no", "Kamnik kommune"), ("pl", "Gmina Kamnik"), ("pt", "Município de Kamnik"), ("ro", "Comuna Kamnik"), ("ru", "Камник"), ("si", "කම\u{dca}න\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Kamnik"), ("sr", "Општина Камник"), ("sr_Latn", "Opština Kamnik"), ("sv", "Kamnik kommun"), ("ta", "க\u{bbe}மினிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3e}మ\u{c4d}న\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องคามน\u{e34}ค"), ("tr", "Kamnik Belediyesi"), ("uk", "Камник (община)"), ("ur", "کامنیک میونسپلٹی"), ("vi", "Đô thị tự trị Kamnik"), ("zh", "卡姆尼克鎮")]),
                        unofficial_name_list: ["Kamnik"].to_vec(),
                    }
                ),
                (
                    "044",
                    Subdivision{
                        name: "044",
                        country_alpha2: Alpha2::SI,
                        code: "044",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0853803), longitude: Some(13.634308), max_latitude: Some(46.096127), min_latitude: Some(46.0728802), max_longitude: Some(13.6614195), min_longitude: Some(13.6281754)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانال أوب سوتسي"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}স\u{9be}\u{9be}ল অব সোচি"), ("bs", "Kanal"), ("ccp", "𑄇𑄚𑄣\u{11134}"), ("ceb", "Kanal"), ("cs", "Občina Kanal ob Soči"), ("da", "Kanal ob Soči"), ("de", "Kanal ob Soči"), ("el", "Κανάλ ομπ Σοτσί"), ("en", "Kanal"), ("es", "Kanal ob Soči"), ("fi", "Kanal ob Soči"), ("fr", "Kanal"), ("gu", "કનાલ ઓબ સોસી"), ("hi", "कनाल ऑब सोची"), ("hr", "Općina Kanal ob Soči"), ("id", "Kanal ob Soči"), ("it", "Canale d’Isonzo"), ("ja", "カナル"), ("kn", "ಕ\u{cc6}ನಲ\u{ccd} ಓಬ\u{ccd} ಸೊಸ\u{cbf}"), ("ko", "카날오브소치"), ("lt", "Kanal ob Sočis"), ("lv", "Kanala ob Soči"), ("mr", "क\u{945}नाल ओब सोस"), ("ms", "Kanal ob Soči"), ("nb", "Kanal ob Soci"), ("nl", "Kanal ob Soči"), ("no", "Kanal ob Soci"), ("pl", "Gmina Kanal ob Soči"), ("pt", "Kanal ob Soči"), ("ro", "Comuna Kanal ob Soči"), ("ru", "Канал-об-Сочи"), ("si", "කැනල\u{dca} ඔබ\u{dca} සොස\u{dd2}"), ("sl", "Občina Kanal ob Soči"), ("sr", "Општина Канал об Сочи"), ("sr_Latn", "Opština Kanal ob Soči"), ("sv", "Kanal ob Soci"), ("ta", "கனல\u{bcd} ஒப\u{bcd}பி ஸோச\u{bc0}"), ("te", "కన\u{c3e}ల\u{c4d} ఓబ\u{c4d} స\u{c4b}స\u{c3f}"), ("th", "คานาล ออป โซซ\u{e34}"), ("tr", "Kanal ob Soci"), ("uk", "Канал"), ("ur", "کنال اوب سوکی"), ("vi", "Kanal ob Soči"), ("zh", "卡那尔")]),
                        unofficial_name_list: ["Kanal"].to_vec(),
                    }
                ),
                (
                    "045",
                    Subdivision{
                        name: "045",
                        country_alpha2: Alpha2::SI,
                        code: "045",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4050135), longitude: Some(15.7947234), max_latitude: Some(46.4248816), min_latitude: Some(46.3808671), max_longitude: Some(15.8052251), min_longitude: Some(15.7747304)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كيدريتشيفو"), ("bn", "কিদ\u{9cd}রিচেভো পৌরসভ\u{9be}"), ("ccp", "𑄇\u{11128}𑄓\u{11133}𑄢\u{1112d}𑄌\u{11134}𑄞\u{1112e}"), ("ceb", "Občina Kidričevo"), ("cs", "Občina Kidričevo"), ("da", "Municipality of Kidričevo"), ("de", "Kidričevo"), ("el", "Κιντριτσέβο"), ("en", "Kidričevo"), ("es", "Municipalidad del Kidričevo"), ("fi", "Kidričevon kunta"), ("fr", "Kidričevo"), ("gu", "કિડ\u{acd}રિસ\u{ac7}વો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "किड\u{94d}रिच\u{947}वो नगर पालिका"), ("hr", "Općina Kidričevo"), ("id", "Kotamadya Kidričevo"), ("it", "Kidričevo"), ("ja", "キドリチェヴォ"), ("kn", "ಕ\u{cbf}ಡ\u{ccd}ರ\u{cbf}ಸ\u{cc6}ವೊ ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "키드리체보"), ("lt", "Kidričevas"), ("lv", "Kidričevo pašvaldība"), ("mr", "किड\u{94d}रिसिएवो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kidricevo Municipality"), ("nb", "Kidricevo Kommune"), ("nl", "Kidričevo"), ("no", "Kidricevo Kommune"), ("pl", "Gmina Kidričevo"), ("pt", "Kidričevo"), ("ro", "Kidričevo"), ("ru", "Кидричево"), ("si", "ක\u{dd2}ඩ\u{dca}ර\u{dd2}කෙවෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Kidričevo"), ("sr", "Општина Кидричево"), ("sr_Latn", "Opština Kidričevo"), ("sv", "Kidricevo kommun"), ("ta", "கிடரிசேவோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c40}డ\u{c4d}ర\u{c48}స\u{c3f}వ\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e34}ดรอเซโว"), ("tr", "Kidricevo Belediyesi"), ("uk", "Кидричево"), ("ur", "کیدریکیوو میونسپلٹی"), ("vi", "Đô thị tự trị Kidricevo"), ("zh", "基德里切沃")]),
                        unofficial_name_list: ["Kidricevo"].to_vec(),
                    }
                ),
                (
                    "046",
                    Subdivision{
                        name: "046",
                        country_alpha2: Alpha2::SI,
                        code: "046",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2476549), longitude: Some(13.5791749), max_latitude: Some(46.26068249999999), min_latitude: Some(46.2271445), max_longitude: Some(13.5921482), min_longitude: Some(13.5482611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوباريد"), ("bn", "কোব\u{9be}রিদ পৌরসভ\u{9be}"), ("ca", "Kobarid"), ("ccp", "𑄇\u{1112e}𑄝𑄢\u{11128}𑄖\u{11134}"), ("ceb", "Občina Kobarid"), ("cs", "Občina Kobarid"), ("da", "Kobarid Municipality"), ("de", "Kobarid"), ("el", "Κομπαρίντ"), ("en", "Kobarid"), ("es", "Kobarid"), ("fi", "Kobaridin kunta"), ("fr", "Kobarid"), ("gu", "કોબારીદ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोबारीद नगरपालिका"), ("hr", "Općina Kobarid"), ("hu", "Kobarid"), ("hy", "Կոբարիդ"), ("id", "Kotamadya Kobarid"), ("it", "Caporetto"), ("ja", "コバリード"), ("kn", "ಕೋಬರ\u{cbf}ದ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "코바리드 지방 자치제"), ("lt", "Kobarido savivaldybė"), ("lv", "Kobaridas pašvaldība"), ("mr", "कोबरद म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kobarid Municipality"), ("nb", "Kobarid"), ("nl", "Kobarid"), ("no", "Kobarid"), ("pl", "Gmina Kobarid"), ("pt", "Kobarid"), ("ro", "Kobarid"), ("ru", "Кобарид"), ("si", "කොබර\u{dd2}ඩ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Kobarid"), ("sr", "Кобарид"), ("sr_Latn", "Kobarid"), ("sv", "Kobarid"), ("ta", "கோபரிட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}బ\u{c3e}ర\u{c3f}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โคบาร\u{e34}ด"), ("tr", "Kobarid Belediyesi"), ("uk", "Кобарід"), ("ur", "کوبارید میونسپلٹی"), ("vi", "Kobarid"), ("zh", "科巴里德")]),
                        unofficial_name_list: ["Kobarid"].to_vec(),
                    }
                ),
                (
                    "047",
                    Subdivision{
                        name: "047",
                        country_alpha2: Alpha2::SI,
                        code: "047",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.68518), longitude: Some(16.3936719), max_latitude: Some(46.7042549), min_latitude: Some(46.658474), max_longitude: Some(16.4286219), min_longitude: Some(16.3528199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوبيليه"), ("bn", "কোবিলি পৌরসভ\u{9be}"), ("ccp", "𑄇\u{1112e}𑄝\u{11128}𑄣\u{11134}𑄎𑄬"), ("ceb", "Kobilje"), ("cs", "Občina Kobilje"), ("da", "Kobilje Municipality"), ("de", "Kobilje"), ("el", "Κομπίλτζε"), ("en", "Kobilje"), ("es", "Kobilje"), ("et", "Kobilje vald"), ("fi", "Kobiljen kunta"), ("fr", "Kobilje"), ("gu", "કોબિલ\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोबीलिए नगरपालिका"), ("hr", "Općina Kobilje"), ("hu", "Kebeleszentmárton"), ("id", "Kotamadya Kobilje"), ("it", "Kobilje"), ("ja", "コビリェ"), ("kn", "ಕೋಬ\u{cbf}ಲ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "코빌레"), ("lt", "Kobilės savivaldybė"), ("lv", "Kobiljes pašvaldība"), ("mr", "कोबिल\u{94d}ह\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kobilje Municipality"), ("nb", "Kobilje"), ("nl", "Kobilje"), ("no", "Kobilje"), ("pl", "Gmina Kobilje"), ("pt", "Kobilje"), ("ro", "Kobilje"), ("ru", "Кобилье"), ("si", "කොබ\u{dd2}ල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Kobilje"), ("sr", "Општина Кобиље"), ("sr_Latn", "Opština Kobilje"), ("sv", "Kobilje"), ("ta", "கொப\u{bcd}பிலேஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}బ\u{c3f}ల\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโกบ\u{e34}ลเจ"), ("tr", "Kobilje Belediyesi"), ("uk", "Кобилє"), ("ur", "کوبیلجی میونسپلٹی"), ("vi", "Kobilje"), ("zh", "科比列")]),
                        unofficial_name_list: ["Kobilje"].to_vec(),
                    }
                ),
                (
                    "048",
                    Subdivision{
                        name: "048",
                        country_alpha2: Alpha2::SI,
                        code: "048",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6409009), longitude: Some(14.8633128), max_latitude: Some(45.6612108), min_latitude: Some(45.6119211), max_longitude: Some(14.8861107), min_longitude: Some(14.8144785)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوتشيفيه"), ("be", "Качэўе"), ("bg", "Кочеве"), ("bn", "কোচেভি পৌরসভ\u{9be}"), ("ca", "Kočevje"), ("ccp", "𑄇\u{1112e}𑄥𑄬𑄛\u{11134}𑄎𑄬"), ("ceb", "Občina Kočevje"), ("cs", "Občina Kočevje"), ("da", "Kočevje"), ("de", "Kočevje"), ("el", "Κοτσέβιε"), ("en", "Kočevje"), ("es", "Kočevje"), ("eu", "Kočevje"), ("fi", "Kočevjen kunta"), ("fr", "Kočevje"), ("gu", "કોસ\u{ac7}વ\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोस\u{947}विय\u{947} नगर पालिका"), ("hr", "Općina Kočevje"), ("hu", "Kočevje"), ("id", "Kočevje"), ("it", "Kočevje"), ("ja", "コチェーヴィエ"), ("kn", "ಕೊಸ\u{cc6}ವ\u{cc6}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "코체베"), ("lt", "Kočevė"), ("lv", "Kočevjes pašvaldība"), ("mr", "कोसव\u{94d}हीज\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kocevje Municipality"), ("nb", "Kocevje kommune"), ("nl", "Kočevje"), ("no", "Kocevje kommune"), ("pl", "Gmina Kočevje"), ("pt", "Kočevje"), ("ro", "Kočevje"), ("ru", "Кочевье"), ("si", "කොසේව\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sk", "Kočevje"), ("sl", "Občina Kočevje"), ("sr", "Кочевје"), ("sr_Latn", "Kočevje"), ("sv", "Kočevje"), ("ta", "கோஸ\u{bcd}த\u{bcd}வஜ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}స\u{c46}వజ\u{c47} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โคเซวเจ"), ("tr", "Kocevje Belediyesi"), ("uk", "Кочевʼє"), ("ur", "کوکیوجی میونسپلٹی"), ("vi", "Kočevje"), ("zh", "科切維")]),
                        unofficial_name_list: ["Kocevje"].to_vec(),
                    }
                ),
                (
                    "049",
                    Subdivision{
                        name: "049",
                        country_alpha2: Alpha2::SI,
                        code: "049",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.81752350000001), longitude: Some(13.7482711), max_latitude: Some(45.8425255), min_latitude: Some(45.8028622), max_longitude: Some(13.7716317), min_longitude: Some(13.7290246)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كومين"), ("bn", "কোমেন পৌরসভ\u{9be}"), ("bs", "Komen"), ("ca", "Komen"), ("ccp", "𑄇\u{11127}𑄟𑄬𑄚\u{11134}"), ("ceb", "Komen"), ("cs", "Občina Komen"), ("da", "Komen Municipality"), ("de", "Komen"), ("el", "Κόμεν"), ("en", "Komen"), ("es", "Komen"), ("fi", "Komenin kunta"), ("fr", "Komen"), ("gu", "કોમ\u{ac7}ન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोम\u{947}\u{902} नगर पालिका"), ("hr", "Općina Komen"), ("id", "Kotamadya Komen"), ("it", "Komen"), ("ja", "コメン"), ("kn", "ಕೊಮ\u{cc6}ನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "코멘 지방 자치제"), ("lt", "Komeno savivaldybė"), ("lv", "Komenas pašvaldība"), ("mr", "कोम\u{947}न म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Komen Municipality"), ("nb", "Komen Kommune"), ("nl", "Komen"), ("no", "Komen Kommune"), ("pl", "Komen"), ("pt", "Komen"), ("ro", "Comuna Komen"), ("ru", "Комен"), ("si", "කොමන\u{dca} නගරසභ\u{dcf}ව"), ("sl", "Občina Komen"), ("sr", "Комен"), ("sr_Latn", "Komen"), ("sv", "Komen (kommun)"), ("ta", "கொமென\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}మ\u{c4b}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโกเมน"), ("tr", "Komen Belediyesi"), ("uk", "Комен"), ("ur", "کومین میونسپلٹی"), ("vi", "Đô thị tự trị Komen"), ("zh", "科門鎮")]),
                        unofficial_name_list: ["Komen"].to_vec(),
                    }
                ),
                (
                    "050",
                    Subdivision{
                        name: "050",
                        country_alpha2: Alpha2::SI,
                        code: "050",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.54805899999999), longitude: Some(13.7301877), max_latitude: Some(45.55769069999999), min_latitude: Some(45.5234682), max_longitude: Some(13.7607302), min_longitude: Some(13.6895796)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كوبر"), ("ccp", "𑄇\u{11127}𑄛𑄢\u{11134}"), ("ceb", "Koper"), ("cs", "Městská občina Koper"), ("en", "Koper"), ("hr", "Gradska općina Kopar"), ("hu", "Koper városi község"), ("pl", "Gmina miejska Koper"), ("ro", "Comuna urbană Koper"), ("sl", "Mestna občina Koper"), ("sr", "Општина Копар"), ("sr_Latn", "Opština Kopar"), ("uk", "Копер"), ("zh", "科佩爾市")]),
                        unofficial_name_list: ["Koper/Capodistria"].to_vec(),
                    }
                ),
                (
                    "051",
                    Subdivision{
                        name: "051",
                        country_alpha2: Alpha2::SI,
                        code: "051",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0733211), longitude: Some(15.5596719), max_latitude: Some(46.0887294), min_latitude: Some(46.0549994), max_longitude: Some(15.5803351), min_longitude: Some(15.5249249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوزيه"), ("bn", "কোজে পৌরসভ\u{9be}"), ("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄎𑄬"), ("ceb", "Kozje"), ("cs", "Občina Kozje"), ("da", "Kozje"), ("de", "Kozje"), ("el", "Κόζτζε"), ("en", "Kozje"), ("es", "Kozje"), ("fi", "Kozjen kunta"), ("fr", "Kozje"), ("gu", "કોઝ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोज\u{93c}िए नगरपालिका"), ("hr", "Općina Kozje"), ("id", "Kotamadya Kozje"), ("it", "Kozje"), ("ja", "コジエ"), ("kn", "ಕೋಜ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "코제"), ("lt", "Kozjės savivaldybė"), ("lv", "Kozjes pašvaldība"), ("mr", "कॉझज\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kozje Municipality"), ("nb", "Kozje Kommune"), ("nl", "Kozje"), ("no", "Kozje Kommune"), ("pl", "Gmina Kozje"), ("pt", "Kozje"), ("ro", "Kozje"), ("ru", "Козье"), ("si", "කොස\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Kozje"), ("sr", "Општина Козје"), ("sr_Latn", "Opština Kozje"), ("sv", "Kozje (kommun)"), ("ta", "கோஸ\u{bcd}ஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3e}జ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคอซเจ"), ("tr", "Kozje Belediyesi"), ("uk", "Козє"), ("ur", "کوزجی میونسپلٹی"), ("vi", "Kozje"), ("zh", "科茲耶")]),
                        unofficial_name_list: ["Kozje"].to_vec(),
                    }
                ),
                (
                    "052",
                    Subdivision{
                        name: "052",
                        country_alpha2: Alpha2::SI,
                        code: "052",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2428344), longitude: Some(14.3555417), max_latitude: Some(46.2704589), min_latitude: Some(46.198773), max_longitude: Some(14.3992928), min_longitude: Some(14.3060005)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كراني ستي"), ("be", "Горад Крань"), ("bg", "Кран"), ("bn", "ক\u{9cd}র\u{9be}ঞ\u{9cd}জ শহর পৌরসভ\u{9be}"), ("bs", "Kranj"), ("ca", "Kranj"), ("ccp", "𑄇\u{11133}𑄢𑄚\u{11134}𑄎\u{11128}"), ("ceb", "Kranj"), ("cs", "Městská občina Kranj"), ("cy", "Kranj"), ("da", "Kranj City Municipality"), ("de", "Gemeinde Kranj"), ("el", "Κραντζ"), ("en", "Kranj"), ("es", "Municipalidad Ciudad de Kranj"), ("et", "Kranj"), ("eu", "Kranj"), ("fa", "کرانی"), ("fi", "Kranj Cityn kunta"), ("fr", "Kranj"), ("gu", "ક\u{acd}ર\u{a82}જ શહ\u{ac7}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("he", "קראן"), ("hi", "क\u{94d}र\u{902}ज शहर नगरपालिका"), ("hr", "Gradska općina Kranj"), ("hu", "Kranj"), ("id", "Kotamadya Kranj City"), ("it", "Kranj"), ("ja", "クラーニ"), ("ka", "კრანი"), ("kn", "ಕ\u{ccd}ರಾಂಜ\u{ccd} ನಗರ ಪುರಸಭ\u{cc6}"), ("ko", "크란 도시 지방 자치제"), ("lt", "Kranis"), ("lv", "Kraņas pilsētas pašvaldība"), ("mk", "Крањ"), ("mr", "कर\u{902}ज सिटी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kranj City Municipality"), ("nb", "Kranj City kommune"), ("nl", "Kranj"), ("no", "Kranj City kommune"), ("pl", "Gmina miejska Kranj"), ("pt", "Cidade Municipal de Kranj"), ("ro", "Comuna urbană Kranj"), ("ru", "Крань"), ("si", "ක\u{dca}\u{200d}රන\u{dca}ජ\u{dca} ස\u{dd2}ට\u{dd2} නගර සභ\u{dcf}ව"), ("sk", "Kranj"), ("sl", "Mestna občina Kranj"), ("sr", "Општина Крањ"), ("sr_Latn", "Opština Kranj"), ("sv", "Kranj City kommun"), ("ta", "க\u{bcd}ர\u{bbe}ஞ\u{bcd} நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}రంజ\u{c4d} స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ไอดร\u{e34}จา"), ("tr", "Kranj Belediyesi"), ("uk", "Крань"), ("ur", "کرانج شہر میونسپلٹی"), ("vi", "Đô thị tự trị Kranj"), ("zh", "克拉尼")]),
                        unofficial_name_list: ["Kranj"].to_vec(),
                    }
                ),
                (
                    "053",
                    Subdivision{
                        name: "053",
                        country_alpha2: Alpha2::SI,
                        code: "053",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4845293), longitude: Some(13.7857145), max_latitude: Some(46.5110281), min_latitude: Some(46.4098216), max_longitude: Some(13.8151357), min_longitude: Some(13.7264278)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كرانجسكا غورا"), ("bg", "Кранска гора"), ("bn", "ক\u{9cd}র\u{9be}ঞ\u{9cd}জস\u{9cd}ক\u{9be} গোর\u{9be} পৌরসভ\u{9be}"), ("ca", "Kranjska Gora"), ("ccp", "𑄇\u{11133}𑄢𑄚\u{11134}𑄎\u{11128}𑄇 𑄉\u{1112e}𑄢"), ("ceb", "Kranjska Gora"), ("cs", "Občina Kranjska Gora"), ("da", "Kranjska Gora"), ("de", "Kranjska Gora"), ("el", "Κράντζσκα Γκόρα"), ("en", "Kranjska Gora"), ("es", "Kranjska Gora"), ("fi", "Kranjska Gora"), ("fr", "Kranjska Gora"), ("gu", "ક\u{acd}રા\u{a82}જ\u{acd}સ\u{acd}કા ગોરા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}र\u{902}ज\u{94d}सका गोरा नगरपालिका"), ("hr", "Kranjska Gora"), ("hu", "Kranjska Gora"), ("id", "Kotamadya Kranjska Gora"), ("it", "Kranjska Gora"), ("ja", "クランスカ・ゴーラ"), ("kn", "ಕ\u{ccd}ರಾಂಜಸ\u{ccd}ಕಾ ಗೋರಾ ಪುರಸಭ\u{cc6}"), ("ko", "크란스카고라"), ("lt", "Kranska Gora"), ("lv", "Kranjskas Goras pašvaldība"), ("mr", "क\u{94d}रा\u{902}जका गोरा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kranjska Gora Municipality"), ("nb", "Kranjska Gora"), ("nl", "Kranjska Gora"), ("no", "Kranjska Gora"), ("pl", "Gmina Kranjska Gora"), ("pt", "Kranjska Gora"), ("ro", "Kranjska Gora"), ("ru", "Краньска-Гора"), ("si", "ක\u{dca}\u{200d}රන\u{dca}ජ\u{dca}ස\u{dca}ක\u{dcf} ගෝර\u{dcf} නගර සභ\u{dcf}ව"), ("sk", "Kranjska Gora"), ("sl", "Občina Kranjska Gora"), ("sr", "Општина Крањска Гора"), ("sr_Latn", "Opština Kranjska Gora"), ("sv", "Kranjska Gora"), ("ta", "க\u{bcd}ரஞ\u{bcd}சஸ\u{bcd}க\u{bcd}க\u{bbe} கோர\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4d}ర\u{c3e}ంజ\u{c4d}\u{200c}స\u{c4d}క\u{c3e} గ\u{c4b}ర\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลครานสกากอรา"), ("tr", "Kanjska Gora Belediyesi"), ("uk", "Краньска Гора"), ("ur", "کرانجسکا گورا میونسپلٹی"), ("vi", "Đô thị tự trị Kranjska Gora"), ("zh", "克拉尼斯卡戈拉")]),
                        unofficial_name_list: ["Kranjska Gora"].to_vec(),
                    }
                ),
                (
                    "054",
                    Subdivision{
                        name: "054",
                        country_alpha2: Alpha2::SI,
                        code: "054",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9592256), longitude: Some(15.4950212), max_latitude: Some(45.98534679999999), min_latitude: Some(45.9360264), max_longitude: Some(15.5130312), min_longitude: Some(15.4666032)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كرشكو"), ("bn", "পৌরসভ\u{9be} অব ক\u{9cd}রস\u{9cd}কো"), ("ccp", "𑄇\u{11133}𑄢𑄇\u{1112e}"), ("ceb", "Občina Krško"), ("cs", "Občina Krško"), ("da", "Municipality of Krško"), ("de", "Gemeinde Krško"), ("el", "Κρσκο"), ("en", "Krško"), ("es", "Municipalidad del Krško"), ("fi", "Krškon kunta"), ("fr", "Krško"), ("gu", "ક\u{acd}ર\u{ac7}સ\u{acd}કો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रॉस\u{94d}को नगर पालिका"), ("hr", "Općina Krško"), ("id", "Kotamadya Krško"), ("it", "Krško"), ("ja", "クルシュコ"), ("kn", "ಕ\u{ccd}ರುಕೊ ಪುರಸಭ\u{cc6}"), ("ko", "크르슈코 지방 자치제"), ("lt", "Krško savivaldybė"), ("lv", "Krško pašvaldība"), ("mr", "क\u{94d}रॉस\u{94d}को म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Municipality of Krsko"), ("nb", "Krsko kommune"), ("nl", "Krško"), ("no", "Krsko kommune"), ("pl", "Gmina Krško"), ("pt", "Município de Krsko"), ("ro", "Comuna Krško"), ("ru", "Кршко"), ("si", "ක\u{dca}ර\u{dca}ස\u{dca}කෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Krško"), ("sr", "Општина Кршко"), ("sr_Latn", "Opština Krško"), ("sv", "Krsko kommun"), ("ta", "கர\u{bcd}ஸக\u{bcd}கோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3f}య\u{c3e}ర\u{c46}స\u{c4d}క\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49} ออฟ ค\u{e31}สโค"), ("tr", "Krsko Belediyesi"), ("uk", "Кршко"), ("ur", "میونسپلٹی کرسکو"), ("vi", "Đô thị tự trị của Krsko"), ("zh", "克爾什科鎮")]),
                        unofficial_name_list: ["Krško"].to_vec(),
                    }
                ),
                (
                    "055",
                    Subdivision{
                        name: "055",
                        country_alpha2: Alpha2::SI,
                        code: "055",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6418793), longitude: Some(15.6036288), max_latitude: Some(46.6903706), min_latitude: Some(46.59235), max_longitude: Some(15.6624836), min_longitude: Some(15.5424122)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كونغوتا"), ("bn", "ক\u{9c1}নগট\u{9be} পৌরসভ\u{9be}"), ("bs", "Kungota"), ("ccp", "𑄇\u{1112a}𑄚\u{11134}𑄉\u{1112e}𑄑"), ("ceb", "Kungota"), ("cs", "Občina Kungota"), ("da", "Municipality of Kungota"), ("de", "Kungota"), ("el", "Κουνγκότα"), ("en", "Kungota"), ("es", "Kungota"), ("fi", "Kungotan kunta"), ("fr", "Kungota"), ("gu", "ક\u{ac1}\u{a82}ગોતા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{941}\u{902}गोता नगर पालिका"), ("hr", "Općina Kungota"), ("id", "Kotamadya Kungota"), ("it", "Kungota"), ("ja", "クンゴタ"), ("kn", "ಕುಂಗೋಟಾ ಪುರಸಭ\u{cc6}"), ("ko", "쿤고타"), ("lt", "Kungotos savivaldybė"), ("lv", "Kungotas pašvaldība"), ("mr", "क\u{941}\u{902}गटोटा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kungota Municipality"), ("nb", "Kungoto Kommune"), ("nl", "Kungota"), ("no", "Kungoto Kommune"), ("pl", "Gmina Kungota"), ("pt", "Kungota"), ("ro", "Kungota"), ("ru", "Кунгота"), ("si", "ක\u{dd4}න\u{dca}ගොට\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Kungota"), ("sr", "Општина Кунгота"), ("sr_Latn", "Opština Kungota"), ("sv", "Kungota"), ("ta", "குங\u{bcd}கொட நகர\u{bbe}ட\u{bcd}சி"), ("te", "కుంగ\u{c4b}ట\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องค\u{e31}นโกต\u{e49}า"), ("tr", "Kungato Belediyesi"), ("uk", "Кунгота"), ("ur", "کونجوتا میونسپلٹی"), ("vi", "Kungota"), ("zh", "昆戈塔")]),
                        unofficial_name_list: ["Kungota"].to_vec(),
                    }
                ),
                (
                    "056",
                    Subdivision{
                        name: "056",
                        country_alpha2: Alpha2::SI,
                        code: "056",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8351038), longitude: Some(16.08071), max_latitude: Some(46.8532224), min_latitude: Some(46.8209718), max_longitude: Some(16.0919423), min_longitude: Some(16.0643504)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوزما"), ("bn", "ক\u{9c1}জম\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄇\u{1112a}𑄌\u{11134}𑄟"), ("ceb", "Kuzma"), ("cs", "Občina Kuzma"), ("da", "Kuzma"), ("de", "Kuzma"), ("el", "Κούζμα"), ("en", "Kuzma"), ("es", "Municipalidad del Kuzma"), ("fi", "Kuzman kunta"), ("fr", "Kuzma"), ("gu", "ક\u{ac1}ઝ\u{acd}મા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{941}ज\u{93c}\u{94d}मा नगरपालिका"), ("hr", "Općina Kuzma"), ("hu", "Kuzma"), ("id", "Kotamadya Kuzma"), ("it", "Kuzma"), ("ja", "クズマ"), ("kn", "ಕುಜ\u{ccd}ಮಾ ಪುರಸಭ\u{cc6}"), ("ko", "쿠즈마"), ("lt", "Kuzmos savivaldybė"), ("lv", "Kuzmas pašvaldība"), ("mr", "क\u{941}ज\u{94d}मा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kuzma Municipality"), ("nb", "Kuzma kommune"), ("nl", "Kuzma"), ("no", "Kuzma kommune"), ("pl", "Gmina Kuzma"), ("pt", "Kuzma"), ("ro", "Kuzma"), ("ru", "Кузма"), ("si", "ක\u{dd4}ස\u{dca}ම\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Kuzma"), ("sr", "Општина Кузма"), ("sr_Latn", "Opština Kuzma"), ("sv", "Kumza (kommun)"), ("ta", "குஸ\u{bcd}ம\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "కుజ\u{c4d}మ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลค\u{e39}ซมา"), ("tr", "Kuzma Belediyesi"), ("uk", "Кузма"), ("ur", "کوزما میونسپلٹی"), ("vi", "Đô thị tự trị Kuzma"), ("zh", "库兹马")]),
                        unofficial_name_list: ["Kuzma"].to_vec(),
                    }
                ),
                (
                    "057",
                    Subdivision{
                        name: "057",
                        country_alpha2: Alpha2::SI,
                        code: "057",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1542793), longitude: Some(15.2359978), max_latitude: Some(46.17712179999999), min_latitude: Some(46.1451113), max_longitude: Some(15.2548406), min_longitude: Some(15.2226445)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لاشكو"), ("be", "Лашка"), ("bn", "ল\u{9be}স\u{9cd}কো পৌরসভ\u{9be}"), ("ccp", "𑄣𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Občina Laško"), ("cs", "Laško"), ("da", "Laško Municipality"), ("de", "Laško"), ("el", "Λάσκο"), ("en", "Laško"), ("es", "Laško"), ("fi", "Laškon kunta"), ("fr", "Laško"), ("gu", "લાસ\u{acd}કો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("he", "לאשקו"), ("hi", "लास\u{94d}को नगर पालिका"), ("hr", "Općina Laško"), ("hu", "Laško"), ("id", "Kotamadya Laško"), ("it", "Laško"), ("ja", "ラーシュコ"), ("kn", "ಲಾಸ\u{ccd}ಕೊ ಪುರಸಭ\u{cc6}"), ("ko", "라슈코"), ("lt", "Laško savivaldybė"), ("lv", "Laško pašvaldība"), ("mr", "लास\u{94d}को म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Lasko Municipality"), ("nb", "Lasko kommune"), ("nl", "Laško"), ("no", "Lasko kommune"), ("pl", "Gmina Laško"), ("pt", "Laško"), ("ro", "Laško"), ("ru", "Лашко"), ("si", "ලස\u{dca}කෝ නගර සභ\u{dcf}ව"), ("sk", "Laško"), ("sl", "Občina Laško"), ("sr", "Општина Лашко"), ("sr_Latn", "Opština Laško"), ("sv", "Lasko kommun"), ("ta", "லஸ\u{bcd}க\u{bcd}கோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c3e}స\u{c4d}క\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องลาสโก\u{e49}"), ("tr", "Lasko Belediyesi"), ("uk", "Лашко"), ("ur", "لاسکو میونسپلٹی"), ("vi", "Đô thị tự trị Lasko"), ("zh", "拉什科")]),
                        unofficial_name_list: ["Laško"].to_vec(),
                    }
                ),
                (
                    "058",
                    Subdivision{
                        name: "058",
                        country_alpha2: Alpha2::SI,
                        code: "058",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5954161), longitude: Some(15.8637359), max_latitude: Some(46.7012008), min_latitude: Some(46.494476), max_longitude: Some(15.9811767), min_longitude: Some(15.7503131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لينارت"), ("bn", "লিন\u{9be}র\u{9cd}ট পৌরসভ\u{9be}"), ("bs", "Lenart"), ("ccp", "𑄣𑄬𑄚𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Lenart"), ("cs", "Občina Lenart"), ("da", "Lenart Municipality"), ("de", "Gemeinde Lenart"), ("el", "Λενάρτ"), ("en", "Lenart"), ("es", "Municipalidad del Lenart"), ("fi", "Lenartin kunta"), ("fr", "Municipalité de Lenart"), ("gu", "લ\u{ac7}નાર\u{acd}ટ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ल\u{947}नार\u{94d}ट नगरपालिका"), ("hr", "Općina Lenart"), ("id", "Kotamadya Lenart"), ("it", "Comune di Lenart"), ("ja", "レナルト"), ("kn", "ಲ\u{cc6}ನಾರ\u{ccd}ಟ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "레나르트 지방 자치제"), ("lt", "Lenarto savivaldybė"), ("lv", "Lenartas pašvaldība"), ("mr", "ल\u{947}नर\u{94d}ट म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Lenart Municipality"), ("nb", "Lenart Kommune"), ("nl", "Lenart"), ("no", "Lenart Kommune"), ("pl", "Gmina Lenart"), ("pt", "Lenart"), ("ro", "Comuna Lenart"), ("ru", "Ленарт"), ("si", "ලෙන\u{dcf}ර\u{dca}ට\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Lenart"), ("sr", "Општина Ленарт"), ("sr_Latn", "Opština Lenart"), ("sv", "Lenart kommun"), ("ta", "லேன\u{bcd}ட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c46}న\u{c3e}ర\u{c4d}ట\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเลนาร\u{e4c}ท"), ("tr", "Lenart Belediyesi"), ("uk", "Ленарт"), ("ur", "لینارت میونسپلٹی"), ("vi", "Đô thị tự trị Lenart"), ("zh", "来纳尔特")]),
                        unofficial_name_list: ["Lenart"].to_vec(),
                    }
                ),
                (
                    "059",
                    Subdivision{
                        name: "059",
                        country_alpha2: Alpha2::SI,
                        code: "059",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5644783), longitude: Some(16.453063), max_latitude: Some(46.5751146), min_latitude: Some(46.5476736), max_longitude: Some(16.4729848), min_longitude: Some(16.4138543)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ليندافا"), ("bn", "লেন\u{9cd}ড\u{9be}ভ\u{9be}"), ("bs", "Lendava"), ("ca", "Lendava"), ("ccp", "𑄣𑄬𑄚\u{11134}𑄓𑄞"), ("ceb", "Lendava"), ("cs", "Občina Lendava"), ("da", "Lendava"), ("de", "Lendava"), ("el", "Λεντάβα"), ("en", "Lendava"), ("es", "Lendava"), ("et", "Lendava"), ("eu", "Lendava"), ("fa", "لنداوا"), ("fi", "Lendava"), ("fr", "Lendava"), ("ga", "Lendava"), ("gu", "લ\u{ac7}ન\u{acd}ડાવા"), ("he", "לנדאווה"), ("hi", "ल\u{947}\u{902}डवा"), ("hr", "Općina Lendava"), ("hu", "Lendva"), ("id", "Lendava"), ("it", "Lendava"), ("ja", "レンダヴァ"), ("kn", "ಲ\u{cc6}ಂಡವ"), ("ko", "렌다바"), ("lt", "Lendava"), ("lv", "Lendava"), ("mr", "ल\u{901}डवा"), ("ms", "Lendava"), ("nb", "Lendava"), ("nl", "Lendava"), ("no", "Lendava"), ("pl", "Gmina Lendava"), ("pt", "Lendava"), ("ro", "Comuna Lendava"), ("ru", "Лендава"), ("si", "ලෙන\u{dca}ඩව\u{dcf}"), ("sk", "Lendava"), ("sl", "Občina Lendava"), ("sr", "Лендава"), ("sr_Latn", "Lendava"), ("sv", "Lendava"), ("ta", "லேண\u{bcd}டவ\u{bbe}"), ("te", "ల\u{c46}ండ\u{c3e}వ\u{c3e}"), ("th", "เม\u{e37}องเลนดาวา"), ("tr", "Lendava"), ("uk", "Лендава"), ("ur", "لینداوا"), ("vi", "Lendava"), ("zh", "倫達瓦")]),
                        unofficial_name_list: ["Lendava/Lendva"].to_vec(),
                    }
                ),
                (
                    "060",
                    Subdivision{
                        name: "060",
                        country_alpha2: Alpha2::SI,
                        code: "060",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.05923050000001), longitude: Some(14.8266015), max_latitude: Some(46.073628), min_latitude: Some(46.0477392), max_longitude: Some(14.8490818), min_longitude: Some(14.8034047)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ليتيجا"), ("bn", "লিতিজ\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄣\u{11128}𑄑\u{11128}𑄎"), ("ceb", "Litija"), ("cs", "Občina Litija"), ("da", "Litija"), ("de", "Litija"), ("el", "Λίτιτζα"), ("en", "Litija"), ("es", "Litija"), ("fa", "لیتییا"), ("fi", "Litijan kunta"), ("fr", "Litija"), ("gu", "લીતીજા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लितिजा महानगरपालिका"), ("hr", "Općina Litija"), ("id", "Kotamadya Litija"), ("it", "Litija"), ("ja", "リティヤ"), ("kn", "ಲ\u{cbf}ಟ\u{cbf}ಜಾ ಪುರಸಭ\u{cc6}"), ("ko", "리티야"), ("lt", "Litija"), ("lv", "Litijas pašvaldība"), ("mr", "लिटिजा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Litija"), ("nb", "Litija kommune"), ("nl", "Litija"), ("no", "Litija kommune"), ("pl", "Gmina Litija"), ("pt", "Litija"), ("ro", "Litija"), ("ru", "Лития"), ("si", "ල\u{dd2}ට\u{dd2}ජ\u{dcf} නගර සභ\u{dcf}ව"), ("sk", "Litija"), ("sl", "Litija"), ("sr", "Општина Литија"), ("sr_Latn", "Opština Litija"), ("sv", "Litjia kommun"), ("ta", "லிடிஜ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c3f}ట\u{c3f}జ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลล\u{e34}ท\u{e34}แจ"), ("tr", "Litija Belediyesi"), ("uk", "Літія"), ("ur", "لیتیجا میونسپلٹی"), ("vi", "Litija"), ("zh", "利蒂亞")]),
                        unofficial_name_list: ["Litija"].to_vec(),
                    }
                ),
                (
                    "061",
                    Subdivision{
                        name: "061",
                        country_alpha2: Alpha2::SI,
                        code: "061",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0569465), longitude: Some(14.5057515), max_latitude: Some(46.1422017), min_latitude: Some(45.990033), max_longitude: Some(14.6446963), min_longitude: Some(14.4195019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Любляна"), ("ca", "Municipalitat de Ljubljana"), ("ccp", "𑄎\u{1112a}𑄛\u{11134}𑄎𑄚"), ("ceb", "Mestna Občina Ljubljana"), ("cs", "Městská občina Lublaň"), ("de", "Stadtgemeinde Ljubljana"), ("el", "Δήμος Πόλης της Λιουμπλιάνα"), ("en", "Ljubljana"), ("es", "Municipalidad de la Ciudad de Liubliana"), ("eu", "Ljubljanako udalerria"), ("fr", "municipalité de Ljubljana"), ("hr", "Gradska općina Ljubljana"), ("hu", "Ljubljana városi község"), ("it", "Comune di Lubiana"), ("nl", "Ljubljana"), ("pl", "Gmina miejska Lublana"), ("ro", "Comuna urbană Ljubljana"), ("ru", "Любляна"), ("sl", "Mestna občina Ljubljana"), ("sr", "Општина Љубљана"), ("sr_Latn", "Opština Ljubljana"), ("uk", "Любляна"), ("zh", "卢布尔雅那市")]),
                        unofficial_name_list: ["Ljubljana"].to_vec(),
                    }
                ),
                (
                    "062",
                    Subdivision{
                        name: "062",
                        country_alpha2: Alpha2::SI,
                        code: "062",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.31358350000001), longitude: Some(14.2458583), max_latitude: Some(46.3223771), min_latitude: Some(46.2969027), max_longitude: Some(14.2670191), min_longitude: Some(14.2379746)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ليوبنو"), ("bn", "ল\u{9c1}বনো পৌরসভ\u{9be}"), ("bs", "Ljubno"), ("ccp", "𑄎\u{1112a}𑄛\u{11134}𑄚\u{1112e}"), ("ceb", "Ljubno"), ("cs", "Občina Ljubno"), ("da", "Ljubno Municipality"), ("de", "Ljubno"), ("el", "Λτζούμπνο"), ("en", "Ljubno"), ("es", "Ljubno"), ("fi", "Ljubnon kunta"), ("fr", "Ljubno"), ("gu", "લ\u{acd}ય\u{ac1}બનો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ल\u{941}ब\u{94d}नो नगरपालिका"), ("hr", "Općina Ljubno"), ("id", "Kotamadya Ljubno"), ("it", "Ljubno"), ("ja", "リュブノ"), ("kn", "ಲುಬ\u{ccd}ಯುನೊ ಪುರಸಭ\u{cc6}"), ("ko", "류브노"), ("lt", "Liubno savivaldybė"), ("lv", "Ljubno pašvaldība"), ("mr", "ल\u{94d}य\u{941}ब\u{94d}नो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ljubno Municipality"), ("nb", "Ljubno kommune"), ("nl", "Ljubno"), ("no", "Ljubno kommune"), ("pl", "Gmina Ljubno"), ("pt", "Ljubno"), ("ro", "Ljubno"), ("ru", "Любно"), ("si", "ල\u{dca}ජ\u{dd4}බ\u{dca}නෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Ljubno"), ("sr", "Општина Љубно"), ("sr_Latn", "Opština Ljubno"), ("sv", "Ljubno kommun"), ("ta", "லெஜுபினோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "లుజబ\u{c4d}న\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องล\u{e39}บโน"), ("tr", "Ljubno Belediyesi"), ("uk", "Любно"), ("ur", "لجوبنو میونسپلٹی"), ("vi", "Ljubno"), ("zh", "柳布诺")]),
                        unofficial_name_list: ["Ljubno"].to_vec(),
                    }
                ),
                (
                    "063",
                    Subdivision{
                        name: "063",
                        country_alpha2: Alpha2::SI,
                        code: "063",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5193583), longitude: Some(16.197814), max_latitude: Some(46.5448579), min_latitude: Some(46.5052073), max_longitude: Some(16.2174697), min_longitude: Some(16.167963)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎\u{1112a}𑄑\u{1112e}𑄟𑄢\u{11134}"), ("ceb", "Ljutomer"), ("cs", "Občina Ljutomer"), ("de", "Ljutomer"), ("en", "Ljutomer"), ("es", "Ljutomer"), ("fr", "Ljutomer"), ("hr", "Općina Ljutomer"), ("hy", "Լյուտոմեր"), ("it", "Ljutomer"), ("ja", "リュトメル"), ("ko", "류토메르"), ("nl", "Ljutomer"), ("pl", "Gmina Ljutomer"), ("pt", "Ljutomer"), ("ro", "Ljutomer"), ("sk", "Ljutomer"), ("sl", "Občina Ljutomer"), ("sr", "Љутомер"), ("sr_Latn", "Ljutomer"), ("sv", "Ljutomer"), ("uk", "Лютомер"), ("vi", "Ljutomer"), ("zh", "柳托梅爾")]),
                        unofficial_name_list: ["Ljutomer"].to_vec(),
                    }
                ),
                (
                    "064",
                    Subdivision{
                        name: "064",
                        country_alpha2: Alpha2::SI,
                        code: "064",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.91708190000001), longitude: Some(14.2285582), max_latitude: Some(45.9531231), min_latitude: Some(45.88443780000001), max_longitude: Some(14.2955564), min_longitude: Some(14.1717294)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوغاتيتس"), ("bn", "লোজেটেক পৌরসভ\u{9be}"), ("ccp", "𑄣\u{11127}𑄉\u{1112e}𑄑𑄬𑄇\u{11134}"), ("ceb", "Logatec"), ("cs", "Občina Logatec"), ("da", "Logatec"), ("de", "Logatec"), ("el", "Λόγκατεκ"), ("en", "Logatec"), ("es", "Logatec"), ("eu", "Logatec"), ("fi", "Logatecin kunta"), ("fr", "Logatec"), ("gu", "લોગ\u{ac7}ટ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लोगाट\u{947}क नगरपालिका"), ("hr", "Općina Logatec"), ("id", "Logatec"), ("it", "Longatico"), ("ja", "ロガテツ"), ("kn", "ಲೋಗಟ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "로가테츠"), ("lt", "Logatecas"), ("lv", "Logatecas pašvaldība"), ("mr", "लॉगट\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Logatec Municipality"), ("nb", "Logatec"), ("nl", "Logatec"), ("no", "Logatec"), ("pl", "Logatec"), ("pt", "Logatec"), ("ro", "Logatec"), ("ru", "Логатец"), ("si", "ලොගටෙක\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Logatec"), ("sl", "Občina Logatec"), ("sr", "Логатец"), ("sr_Latn", "Logatec"), ("sv", "Logatec"), ("ta", "லொக\u{bbe}டேகி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c4b}గ\u{c3e}ట\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโลกาเทค"), ("tr", "Logatec Belediyesi"), ("uk", "Логатець (община)"), ("ur", "لوجاتیک میونسپلٹی"), ("vi", "Logatec"), ("zh", "洛加泰茨")]),
                        unofficial_name_list: ["Logatec"].to_vec(),
                    }
                ),
                (
                    "065",
                    Subdivision{
                        name: "065",
                        country_alpha2: Alpha2::SI,
                        code: "065",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6477908), longitude: Some(14.4973147), max_latitude: Some(45.7490038), min_latitude: Some(45.5796949), max_longitude: Some(14.5701394), min_longitude: Some(14.3794264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوتسكا دولينا"), ("bn", "লস\u{9cd}ক\u{9be} ডোলিন\u{9be} পৌরসভ\u{9be}"), ("bs", "Loška dolina"), ("ccp", "𑄣\u{1112e}𑄌\u{11134}𑄇 𑄓\u{1112e}𑄣\u{11128}𑄚"), ("ceb", "Občina Loška Dolina"), ("cs", "Občina Loška dolina"), ("da", "Loška Dolina Municipality"), ("de", "Loška Dolina"), ("el", "Λόσκα Ντολίνα"), ("en", "Loška Dolina"), ("es", "Loška Dolina"), ("fi", "Loška Dolinan kunta"), ("fr", "Loška Dolina"), ("gu", "લોસ\u{acd}કા ડોલીના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लोस\u{94d}का डोलिना नगर पालिका"), ("hr", "Općina Loška dolina"), ("id", "Kotamadya Loška Dolina"), ("it", "Loška Dolina"), ("ja", "ロシュカ・ドリナ"), ("kn", "ಲೊಸ\u{ccd}ಕಾ ಡೊಲ\u{cbf}ನಾ ಪುರಸಭ\u{cc6}"), ("ko", "로슈카돌리나"), ("lt", "Loškos Dolinos savivaldybė"), ("lv", "Loška Dolinas pašvaldība"), ("mr", "लोस\u{94d}ता डोलिना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Loska Dolina Municipality"), ("nb", "Loska Dolina Kommune"), ("nl", "Loška Dolina"), ("no", "Loska Dolina Kommune"), ("pl", "Gmina Loška dolina"), ("pt", "Loška Dolina"), ("ro", "Loška Dolina"), ("ru", "Лошка-Долина"), ("si", "ලොස\u{dca}ක\u{dcf} ඩොල\u{dd2}න\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Loška dolina"), ("sr", "Општина Лошка Долина"), ("sr_Latn", "Opština Loška Dolina"), ("sv", "Loska Dolina (kommun)"), ("ta", "லஸ\u{bcd}க\u{bbe} தோலின\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c4b}స\u{c4d}క\u{c3e} డ\u{c4b}ల\u{c3f}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดอาร\u{e4c}ต\u{e35}โบน\u{e35}ต"), ("tr", "Loska Dolina Belediyesi"), ("uk", "Лошка Долина"), ("ur", "لوسکا دولینا میونسپلٹی"), ("vi", "Loška Dolina"), ("zh", "罗斯卡得利亚")]),
                        unofficial_name_list: ["Loška dolina"].to_vec(),
                    }
                ),
                (
                    "066",
                    Subdivision{
                        name: "066",
                        country_alpha2: Alpha2::SI,
                        code: "066",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.664967), longitude: Some(14.6622265), max_latitude: Some(45.7436929), min_latitude: Some(45.5813591), max_longitude: Some(14.7350609), min_longitude: Some(14.5375037)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوتسكي بوتوك"), ("bn", "লস\u{9cd}কি পোটোক পৌরসভ\u{9be}"), ("bs", "Loški Potok"), ("ccp", "𑄣\u{1112e}𑄌\u{11134}𑄇\u{11128} 𑄛\u{1112e}𑄑\u{1112e}𑄇\u{11134}"), ("ceb", "Občina Loški Potok"), ("cs", "Občina Loški Potok"), ("da", "Loški Potok Municipality"), ("de", "Loški Potok"), ("el", "Λόσκι Πότοκ"), ("en", "Loški Potok"), ("es", "Loški Potok"), ("fi", "Loški Potokin kunta"), ("fr", "Loški Potok"), ("gu", "લોસ\u{acd}કી પોટૉક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "लोस\u{94d}की पोटोक नगर पालिका"), ("hr", "Općina Loški Potok"), ("id", "Kotamadya Loški Potok"), ("it", "Loški Potok"), ("ja", "ロシュキ・ポトク"), ("kn", "ಲೋಸ\u{ccd}ಕ\u{cbf} ಪೋಟೋಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "로슈키포토크"), ("lt", "Loški Potoko savivaldybė"), ("lv", "Loški Potokas pašvaldība"), ("mr", "लोस\u{94d}की पोटोक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Loski Potok Municipality"), ("nb", "Loski Potok Kommune"), ("nl", "Loški Potok"), ("no", "Loski Potok Kommune"), ("pl", "Gmina Loški Potok"), ("pt", "Loški Potok"), ("ro", "Loški Potok"), ("ru", "Лошки-Поток"), ("si", "ලෝස\u{dca}ක\u{dd2} පොටොක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Loški potok"), ("sr", "Општина Лошки Поток"), ("sr_Latn", "Opština Loški Potok"), ("sv", "Loski Potok (kommun)"), ("ta", "ல\u{bcd}வ\u{bcd}ஸ\u{bcd}கி போடோக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ల\u{c4b}స\u{c4d}క\u{c40} ప\u{c4b}ట\u{c4b}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ลอสก\u{e34} โพทอค"), ("tr", "Loški Potok"), ("uk", "Лошкий Поток"), ("ur", "لوسکی پوتوک میونسپلٹی"), ("vi", "Loški Potok"), ("zh", "洛什基波托克")]),
                        unofficial_name_list: ["Loški Potok"].to_vec(),
                    }
                ),
                (
                    "067",
                    Subdivision{
                        name: "067",
                        country_alpha2: Alpha2::SI,
                        code: "067",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.35487209999999), longitude: Some(14.7458015), max_latitude: Some(46.3590009), min_latitude: Some(46.35276469999999), max_longitude: Some(14.7482263), min_longitude: Some(14.7346965)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوتشه"), ("bn", "ল\u{9c1}সে পৌরসভ\u{9be}"), ("ccp", "𑄣𑄌\u{11134}"), ("ceb", "Občina Luče"), ("cs", "Občina Luče"), ("da", "Luče"), ("de", "Luče"), ("el", "Λούτσε"), ("en", "Luče"), ("es", "Municipalidad del Luce"), ("fi", "Lučen kunta"), ("fr", "Luče"), ("gu", "લ\u{ac1}ક\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ल\u{94d}य\u{942}स नगरपालिका"), ("hr", "Općina Luče"), ("id", "Kotamadya Luče"), ("it", "Luče"), ("ja", "ルチェ"), ("kn", "ಲುಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "루체"), ("lt", "Lučės savivaldybė"), ("lv", "Lučes pašvaldība"), ("mr", "ल\u{941}क\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Luce Municipality"), ("nb", "Luca kommune"), ("nl", "Luče"), ("no", "Luca kommune"), ("pl", "Gmina Luče"), ("pt", "Luče"), ("ro", "Luče"), ("ru", "Люче"), ("si", "ල\u{dd4}සේ නගර සභ\u{dcf}ව"), ("sl", "Občina Luče"), ("sr", "Општина Луче"), ("sr_Latn", "Opština Luče"), ("sv", "Luce"), ("ta", "லூசே நகர\u{bbe}ட\u{bcd}சி"), ("te", "లూస\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบ\u{e34}ลล\u{e39}ซ\u{e35}"), ("tr", "Luce Belediyesi"), ("uk", "Луче"), ("ur", "لوکی میونسپلٹی"), ("vi", "Đô thị tự trị Luce"), ("zh", "卢切")]),
                        unofficial_name_list: ["Luce"].to_vec(),
                    }
                ),
                (
                    "068",
                    Subdivision{
                        name: "068",
                        country_alpha2: Alpha2::SI,
                        code: "068",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1696293), longitude: Some(14.6907259), max_latitude: Some(46.1794056), min_latitude: Some(46.1629481), max_longitude: Some(14.7040913), min_longitude: Some(14.6788945)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية لوكوفيتسا"), ("bn", "ল\u{9c1}কোভিক\u{9be} পৌরসভ\u{9be}"), ("bs", "Lukovica"), ("ccp", "𑄣\u{1112a}𑄇\u{1112e}𑄞\u{11128}𑄇"), ("ceb", "Lukovica"), ("cs", "Občina Lukovica"), ("da", "Lukovica"), ("de", "Lukovica"), ("el", "Λουκοβίκα"), ("en", "Lukovica"), ("es", "Lukovica"), ("fi", "Lukovican kunta"), ("fr", "Lukovica"), ("gu", "લ\u{ac1}કોવિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ल\u{941}कोविका नगर पालिका"), ("hr", "Općina Lukovica"), ("id", "Kotamadya Lukovica"), ("it", "Lukovica"), ("ja", "ルコヴィツァ"), ("kn", "ಲುಕೋವ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "루코비차"), ("lt", "Lukovicos savivaldybė"), ("lv", "Lukovicas pašvaldība"), ("mr", "ल\u{941}कोविका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Lukovica Municipality"), ("nb", "Lukovica Kommune"), ("nl", "Lukovica"), ("no", "Lukovica Kommune"), ("pl", "Gmina Lukovica"), ("pt", "Lukovica"), ("ro", "Lukovica"), ("ru", "община Луковица"), ("si", "ල\u{dd4}කොව\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Lukovica"), ("sr", "Општина Луковица"), ("sr_Latn", "Opština Lukovica"), ("sv", "Lukovica"), ("ta", "லுகோவிக\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "లుక\u{c4b}వ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องล\u{e39}โคว\u{e34}ค\u{e48}า"), ("tr", "Lukovica Belediyesi"), ("uk", "Луковиця (община)"), ("ur", "لوکوویکا میونسپلٹی"), ("vi", "Đô thị tự trị Lukovica"), ("zh", "卢科维察")]),
                        unofficial_name_list: ["Lukovica"].to_vec(),
                    }
                ),
                (
                    "069",
                    Subdivision{
                        name: "069",
                        country_alpha2: Alpha2::SI,
                        code: "069",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3503019), longitude: Some(15.7340595), max_latitude: Some(46.3584699), min_latitude: Some(46.3405229), max_longitude: Some(15.7457981), min_longitude: Some(15.7208665)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماجتشيرك"), ("bn", "ম\u{9be}জস\u{9cd}প\u{9be}র\u{9cd}ক পৌরসভ\u{9be}"), ("ccp", "𑄟𑄌\u{11134}𑄛𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Občina Majšperk (munisipyo sa Eslobenya)"), ("cs", "Občina Majšperk"), ("da", "Majšperk Municipality"), ("de", "Majšperk"), ("el", "Μάτζσπερκ"), ("en", "Majšperk"), ("es", "Municipalidad del Majšperk"), ("fi", "Majšperkin kunta"), ("fr", "Majšperk"), ("gu", "માજ\u{acd}સપર\u{acd}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "माजस\u{94d}पर\u{94d}क नगर पालिका"), ("hr", "Općina Majšperk"), ("id", "Kotamadya Majšperk"), ("it", "Majšperk"), ("ja", "マイシュペルク"), ("kn", "ಮಜ\u{ccd}ಸ\u{cc6}ಪರ\u{ccd}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "마이슈페르크"), ("lt", "Majšperko savivaldybė"), ("lv", "Majšperkas pašvaldība"), ("mr", "मझोप\u{945}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Majsperk Municipality"), ("nb", "Majsperk kommune"), ("nl", "Majšperk"), ("no", "Majsperk kommune"), ("pl", "Gmina Majšperk"), ("pt", "Majšperk"), ("ro", "Majšperk"), ("ru", "Майшперк"), ("si", "මජ\u{dca}ස\u{dca}පර\u{dca}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Majšperk"), ("sr", "Општина Мајшперк"), ("sr_Latn", "Opština Majšperk"), ("sv", "Majšperk"), ("ta", "மஜ\u{bcd}ஜிஸ\u{bcd}பெர\u{bcd}க\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}జ\u{c4d}\u{200c}స\u{c4d}ప\u{c46}ర\u{c4d}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องมาจ\u{e4c}ซเพ\u{e34}ค"), ("tr", "Majsprk Belediyesi"), ("uk", "Майшперк"), ("ur", "ماجسپیرک میونسپلٹی"), ("vi", "Đô thị tự trị Majsperk"), ("zh", "馬伊什佩克")]),
                        unofficial_name_list: ["Majšperk"].to_vec(),
                    }
                ),
                (
                    "070",
                    Subdivision{
                        name: "070",
                        country_alpha2: Alpha2::SI,
                        code: "070",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5546503), longitude: Some(15.6458812), max_latitude: Some(46.59536490000001), min_latitude: Some(46.5117149), max_longitude: Some(15.700245), min_longitude: Some(15.5888547)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماريبور ستي"), ("be", "Марыбар"), ("bg", "Марибор"), ("bn", "ম\u{9be}রিবর শহর পৌরসভ\u{9be}"), ("ccp", "𑄟𑄢\u{11128}𑄝\u{1112e}𑄢\u{11134}"), ("ceb", "Maribor"), ("cs", "Městská občina Maribor"), ("da", "Maribor City Municipality"), ("de", "Maribor"), ("el", "Μάριμπορ"), ("en", "Maribor"), ("es", "Ciudad Maribor"), ("fi", "Mariborin kunta"), ("fr", "Maribor"), ("gu", "મ\u{ac7}રિબોર શહ\u{ac7}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मारीबोर शहर नगरपालिका"), ("hr", "Gradska općina Maribor"), ("hu", "Maribor városi község"), ("id", "Kotamadya Maribor City"), ("it", "Comune di Maribor City"), ("ja", "マリボル"), ("kn", "ಮರ\u{cbf}ಬೋರ\u{ccd} ಸ\u{cbf}ಟ\u{cbf} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "마리보르 도시 지방 자치제"), ("lt", "Mariboro miesto savivaldybė"), ("lv", "Mariboras pilsētas pašvaldība"), ("mr", "म\u{947}रिबोर सिटी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Maribor City Municipality"), ("nb", "Maribor City kommune"), ("nl", "Maribor"), ("no", "Maribor City kommune"), ("pl", "Gmina miejska Maribor"), ("pt", "Cidade Municipal de Maribor"), ("ro", "Comuna urbană Maribor"), ("ru", "Марибор"), ("si", "ම\u{dcf}ර\u{dd2}බෝර\u{dca} ස\u{dd2}ට\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Mestna občina Maribor"), ("sr", "Општина Марибор"), ("sr_Latn", "Opština Maribor"), ("sv", "Maribor City kommun"), ("ta", "மரிபோர\u{bcd} நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}ర\u{c3f}బ\u{c4b}ర\u{c4d} స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โดฮ\u{e38}ค โกเวอโนเรท"), ("tr", "Maribot Belediyesi"), ("uk", "Марибор"), ("ur", "مریبور شہر میونسپلٹی"), ("vi", "Đô thị tự trị Maribor"), ("zh", "馬里博爾市")]),
                        unofficial_name_list: ["Maribor"].to_vec(),
                    }
                ),
                (
                    "071",
                    Subdivision{
                        name: "071",
                        country_alpha2: Alpha2::SI,
                        code: "071",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1413926), longitude: Some(14.411347), max_latitude: Some(46.1553266), min_latitude: Some(46.12268359999999), max_longitude: Some(14.4318997), min_longitude: Some(14.3930648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مدفودي"), ("bn", "মেদভোদ পৌরসভ\u{9be}"), ("ccp", "𑄟𑄬𑄖\u{11134}𑄞\u{1112e}𑄖\u{11134}"), ("ceb", "Medvode"), ("cs", "Občina Medvode"), ("da", "Medvode Municipality"), ("de", "Medvode"), ("el", "Μεντβόντε"), ("en", "Medvode"), ("es", "Medvode"), ("fa", "مدوده"), ("fi", "Medvoden kunta"), ("fr", "Medvode"), ("gu", "મ\u{ac7}ડવોડ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{947}डवोड\u{947} नगर पालिका"), ("hr", "Općina Medvode"), ("hu", "Medvode"), ("id", "Kotamadya Medvode"), ("it", "Medvode"), ("ja", "メドヴォデ"), ("kn", "ಮ\u{cc6}ಡ\u{ccd}ವೋಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "메드보데"), ("lt", "Medvodės"), ("lv", "Medvodes pašvaldība"), ("mr", "म\u{947}दव\u{947}द म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Medvode Municipality"), ("nb", "Medvode Kommune"), ("nl", "Medvode"), ("no", "Medvode Kommune"), ("pl", "Gmina Medvode"), ("pt", "Medvode"), ("ro", "Medvode"), ("ru", "Медводе"), ("si", "මෙඩ\u{dca}වොඩේ නගර සභ\u{dcf}ව"), ("sk", "Medvode"), ("sl", "Občina Medvode"), ("sr", "Општина Медводе"), ("sr_Latn", "Opština Medvode"), ("sv", "Medvode (kommun)"), ("ta", "மேடிவோடே நகர\u{bbe}ட\u{bcd}சி"), ("te", "మడ\u{c4d}వ\u{c4b}డ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เมดโวเด"), ("tr", "Medvode"), ("uk", "Медводе"), ("ur", "میدوودی میونسپلٹی"), ("vi", "Medvode"), ("zh", "梅德沃代")]),
                        unofficial_name_list: ["Medvode"].to_vec(),
                    }
                ),
                (
                    "072",
                    Subdivision{
                        name: "072",
                        country_alpha2: Alpha2::SI,
                        code: "072",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.16591220000001), longitude: Some(14.5719694), max_latitude: Some(46.1957794), min_latitude: Some(46.1406997), max_longitude: Some(14.5928141), min_longitude: Some(14.5137529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مينغش"), ("bn", "মেঞ\u{9cd}জেস পৌরসভ\u{9be}"), ("ccp", "𑄟𑄬𑄚\u{11134}𑄉𑄬𑄌\u{11134}"), ("ceb", "Občina Mengeš"), ("cs", "Občina Mengeš"), ("da", "Mengeš Municipality"), ("de", "Mengeš"), ("el", "Μενγκές"), ("en", "Mengeš"), ("es", "Mengeš"), ("fi", "Mengešn kunta"), ("fr", "Mengeš"), ("gu", "મ\u{ac7}ન\u{acd}જ\u{ac7}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{947}\u{902}ज\u{947}स नगर पालिका"), ("hr", "Općina Mengeš"), ("id", "Kotamadya Mengeš"), ("it", "Mengeš"), ("ja", "メンゲシュ"), ("kn", "ಮ\u{cc6}ನ\u{ccd}ಜ\u{cc6}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "멘게시"), ("lt", "Mengešo savivaldybė"), ("lv", "Mengešas pašvaldība"), ("mr", "म\u{947}न\u{94d}ज\u{947}श म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Menges Municipality"), ("nb", "Menges Kommune"), ("nl", "Mengeš"), ("no", "Menges Kommune"), ("pl", "Gmina Mengeš"), ("pt", "Mengeš"), ("ro", "Mengeš"), ("ru", "Менгеш"), ("si", "මෙන\u{dca}ගේස\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Mengeš"), ("sl", "Občina Mengeš"), ("sr", "Општина Менгеш"), ("sr_Latn", "Opština Mengeš"), ("sv", "Menges (kommun)"), ("ta", "மென\u{bcd}கேஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c46}ంజ\u{c46}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เมนเกส"), ("tr", "Menges Belediyesi"), ("uk", "Менгеш"), ("ur", "مینجیس میونسپلٹی"), ("vi", "Đô thị tự trị Menges"), ("zh", "門蓋什")]),
                        unofficial_name_list: ["Mengeš"].to_vec(),
                    }
                ),
                (
                    "073",
                    Subdivision{
                        name: "073",
                        country_alpha2: Alpha2::SI,
                        code: "073",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6473994), longitude: Some(15.3176752), max_latitude: Some(45.6678447), min_latitude: Some(45.6331496), max_longitude: Some(15.3524822), min_longitude: Some(15.2820696)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "متليكا"), ("bn", "মেটলিক\u{9be}"), ("bs", "Metlika"), ("ca", "Metlika"), ("ccp", "𑄟𑄬𑄖\u{11134}𑄣\u{11128}𑄇"), ("ceb", "Metlika"), ("cs", "Občina Metlika"), ("da", "Metlika"), ("de", "Metlika"), ("el", "Μετλίκα"), ("en", "Metlika"), ("es", "Metlika"), ("fa", "متلیکا"), ("fi", "Metlika"), ("fr", "Metlika"), ("gu", "મ\u{ac7}ટ\u{acd}લીકા"), ("he", "מטליקה"), ("hi", "म\u{947}ट\u{94d}लिका"), ("hr", "Općina Metlika"), ("id", "Metlika"), ("it", "Metlika"), ("ja", "メトリカ"), ("kn", "ಮ\u{cc6}ಟ\u{ccd}ಲ\u{cbf}ಕಾ"), ("ko", "메틀리카"), ("lt", "Metlika"), ("lv", "Metlika"), ("mr", "म\u{947}ट\u{94d}लिका"), ("ms", "Metlika"), ("nb", "Metlika"), ("nl", "Metlika"), ("no", "Metlika"), ("pl", "Gmina Metlika"), ("pt", "Metlika"), ("ro", "Comuna Metlika"), ("ru", "Метлика"), ("si", "මෙට\u{dca}ල\u{dd2}ක\u{dcf}"), ("sk", "Metlika"), ("sl", "Občina Metlika"), ("sr", "Општина Метлика"), ("sr_Latn", "Opština Metlika"), ("sv", "Metlika"), ("ta", "மென\u{bcd}டலிக\u{bbe}"), ("te", "మ\u{c46}ట\u{c3f}ల\u{c4d}క\u{c3e}"), ("th", "เมตล\u{e34}กา"), ("tr", "Metlika"), ("uk", "Метлика"), ("ur", "میتلیکا"), ("vi", "Metlika"), ("zh", "梅特利卡")]),
                        unofficial_name_list: ["Metlika"].to_vec(),
                    }
                ),
                (
                    "074",
                    Subdivision{
                        name: "074",
                        country_alpha2: Alpha2::SI,
                        code: "074",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5214145), longitude: Some(14.8523952), max_latitude: Some(46.5276309), min_latitude: Some(46.5095402), max_longitude: Some(14.867506), min_longitude: Some(14.8422086)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄬𑄎\u{11128}𑄇"), ("ceb", "Občina Mežica"), ("cs", "Občina Mežica"), ("de", "Mežica"), ("en", "Mežica"), ("fr", "Mežica"), ("it", "Mežica"), ("nl", "Mežica"), ("ro", "Comuna Mežica"), ("sl", "Občina Mežica"), ("sr", "Општина Межица"), ("sr_Latn", "Opština Mežica"), ("uk", "Межиця")]),
                        unofficial_name_list: ["Mežica"].to_vec(),
                    }
                ),
                (
                    "075",
                    Subdivision{
                        name: "075",
                        country_alpha2: Alpha2::SI,
                        code: "075",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.84360290000001), longitude: Some(13.6276647), max_latitude: Some(45.9038824), min_latitude: Some(45.8158667), max_longitude: Some(13.71941), min_longitude: Some(13.5744186)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ميرين-كوسفانيفيتسا"), ("bn", "মিরেনকস\u{9cd}তণজেভ পৌরসভ\u{9be}"), ("bs", "Miren-Kostanjevica"), ("ca", "Miren-Kostanjevica"), ("ccp", "𑄟\u{11128}𑄢𑄬𑄚\u{11134}-𑄇\u{1112e}𑄌\u{11134}𑄑𑄚\u{11134}𑄎𑄬𑄞\u{11128}𑄇"), ("ceb", "Miren-Kostanjevica"), ("cs", "Občina Miren-Kostanjevica"), ("da", "Miren–Kostanjevica Municipality"), ("de", "Miren-Kostanjevica"), ("el", "Μιρέν-Κοσταντζεβίκα"), ("en", "Miren–Kostanjevica"), ("es", "Miren-Kostanjevica"), ("fi", "Miren–Kostanjevican kunta"), ("fr", "Miren-Kostanjevica"), ("gu", "મિર\u{ac7}ન-કોસ\u{acd}ટાન\u{acd}જ\u{ac7}વિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मिर\u{947}न-कोस\u{94d}टानय\u{947}विका नगर पालिका"), ("hr", "Općina Miren - Kostanjevica"), ("id", "Kotamadya Miren–Kostanjevica"), ("it", "Merna-Castagnevizza"), ("kn", "ಮ\u{cbf}ರ\u{cc6}ನ\u{ccd}-ಕೊಸ\u{ccd}ತಾನ\u{ccd}ಜ\u{cc6}ವ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "미렌코스타네비차"), ("lt", "Miren-Kostanevicos savivaldybė"), ("lv", "Miren–Kostanjevicas pašvaldība"), ("mr", "मोर\u{947}न-कोस\u{94d}टानज\u{947}व\u{94d}हिका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Miren–Kostanjevica Municipality"), ("nb", "Miren-kostanjevica kommune"), ("nl", "Miren-Kostanjevica"), ("no", "Miren-kostanjevica kommune"), ("pl", "Gmina Miren-Kostanjevica"), ("pt", "Miren-Kostanjevica"), ("ro", "Miren-Kostanjevica"), ("ru", "Мирен-Костаньевица"), ("si", "ම\u{dd2}රෙන\u{dca}-කොස\u{dca}ටන\u{dca}ජේව\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Miren - Kostanjevica"), ("sr", "Општина Мирен - Костањевица"), ("sr_Latn", "Opština Miren - Kostanjevica"), ("sv", "Miren-kostanjevica kommun"), ("ta", "மின\u{bcd}றேன\u{bcd} –கோஸ\u{bcd}ட\u{bbe}ஞ\u{bcd}சேவிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3f}ర\u{c46}న\u{c4d}-క\u{c4b}స\u{c4d}ట\u{c3e}ంజ\u{c46}వ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ไมเรน คอสตานเจว\u{e35}คา"), ("tr", "Miren-Kostanjevica"), ("uk", "Мірен-Костанєвіца"), ("vi", "Miren-Kostanjevica"), ("zh", "米伦-科斯塔涅维察")]),
                        unofficial_name_list: ["Miren-Kostanjevica"].to_vec(),
                    }
                ),
                (
                    "076",
                    Subdivision{
                        name: "076",
                        country_alpha2: Alpha2::SI,
                        code: "076",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4439734), longitude: Some(15.1985947), max_latitude: Some(46.4963761), min_latitude: Some(46.4278601), max_longitude: Some(15.3282435), min_longitude: Some(15.1504946)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميسلينيا"), ("bn", "মিসলিঞ\u{9cd}জ\u{9be}"), ("bs", "Mislinja"), ("ccp", "𑄟\u{11128}𑄣\u{11134}𑄣\u{11128}𑄚\u{11134}𑄎"), ("ceb", "Mislinja"), ("cs", "Občina Mislinja"), ("da", "Mislinja"), ("de", "Mislinja"), ("el", "Μισλίντζα"), ("en", "Mislinja"), ("es", "Mislinja"), ("fi", "Mislinja"), ("fr", "Mislinja"), ("gu", "મિસ\u{acd}લીન\u{acd}જા"), ("hi", "मिस\u{94d}लिन\u{94d}या"), ("hr", "Općina Mislinja"), ("id", "Mislinja"), ("it", "Mislinja"), ("ja", "ミスリニャ"), ("kn", "ಮ\u{cbf}ಸ\u{ccd}ಲ\u{cbf}ನ\u{ccd}ಜ"), ("ko", "미슬리냐"), ("lt", "Mislinja"), ("lv", "Mislinja"), ("mr", "मिहिलिनजा"), ("ms", "Mislinja"), ("nb", "Mislinja"), ("nl", "Mislinja"), ("no", "Mislinja"), ("pl", "Gmina Mislinja"), ("pt", "Mislinja"), ("ro", "Mislinja"), ("ru", "Мислиня"), ("si", "ම\u{dd2}ස\u{dca}ල\u{dd2}න\u{dca}ජ\u{dcf}"), ("sl", "Mislinja"), ("sr", "Мислиња"), ("sr_Latn", "Mislinja"), ("sv", "Mislinja"), ("ta", "மிஸ\u{bcd}லிஞ\u{bcd}ச\u{bbe}"), ("te", "మ\u{c3f}స\u{c4d}ల\u{c3f}ంజ\u{c3e}"), ("th", "ม\u{e34}สล\u{e34}นจา"), ("tr", "Mislinja"), ("uk", "Мислиня"), ("ur", "مسلینجا"), ("vi", "Mislinja"), ("zh", "米斯利尼亚")]),
                        unofficial_name_list: ["Mislinja"].to_vec(),
                    }
                ),
                (
                    "077",
                    Subdivision{
                        name: "077",
                        country_alpha2: Alpha2::SI,
                        code: "077",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1357218), longitude: Some(14.7444352), max_latitude: Some(46.1442446), min_latitude: Some(46.1250171), max_longitude: Some(14.7557551), min_longitude: Some(14.7283915)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مورافسكه، مورافسكه"), ("bn", "মোর\u{9be}ভি পৌরসভ\u{9be}"), ("ccp", "𑄟𑄢𑄛\u{11134}"), ("ceb", "Občina Moravče"), ("cs", "Občina Moravče"), ("da", "Moravče"), ("de", "Moravče"), ("el", "Μοράβτσε"), ("en", "Moravče"), ("es", "Municipalidad del Moravče"), ("fi", "Moravčen kunta"), ("fr", "Moravče"), ("gu", "મોરાવ\u{acd}ક\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मोर\u{947}व\u{94d}स\u{947} नगरपालिका"), ("hr", "Općina Moravče"), ("id", "Kotamadya Moravče"), ("it", "Moravče"), ("ja", "モラフチェ"), ("kn", "ಮೊರಾವ\u{cc6}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "모라브체"), ("lt", "Moravčės savivaldybė"), ("lv", "Moravčes pašvaldība"), ("mr", "मोरावस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Moravce Municipality"), ("nb", "Moravce kommune"), ("nl", "Moravče"), ("no", "Moravce kommune"), ("pl", "Gmina Moravče"), ("pt", "Moravče"), ("ro", "Moravče"), ("ru", "Моравче"), ("si", "මොර\u{dcf}ව\u{dca}සේ නගර සභ\u{dcf}ව"), ("sl", "Občina Moravče"), ("sr", "Општина Моравче"), ("sr_Latn", "Opština Moravče"), ("sv", "Moravce kommun"), ("ta", "மொர\u{bbe}வ\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c4b}ర\u{c3e}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโมราฟเซ"), ("tr", "Moravce Belediyesi"), ("uk", "Моравче"), ("ur", "موراویس میونسپلٹی"), ("vi", "Moravče"), ("zh", "摩拉夫切")]),
                        unofficial_name_list: ["Moravce"].to_vec(),
                    }
                ),
                (
                    "078",
                    Subdivision{
                        name: "078",
                        country_alpha2: Alpha2::SI,
                        code: "078",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6856932), longitude: Some(16.2224582), max_latitude: Some(46.7265446), min_latitude: Some(46.6780051), max_longitude: Some(16.2623714), min_longitude: Some(16.2059497)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مورافسك توبليس"), ("bn", "মোর\u{9be}ভস\u{9cd}কে টপ\u{9cd}লিস পৌরসভ\u{9be}"), ("ccp", "𑄟𑄢𑄛\u{11134}𑄥\u{11134}𑄇𑄬 𑄑\u{1112a}𑄛\u{11134}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Moravske Toplice"), ("cs", "Občina Moravske Toplice"), ("da", "Moravske Toplice Municipality"), ("de", "Gemeinde Moravske Toplice"), ("el", "Μοράβσκε Τοπλίτσε"), ("en", "Moravske Toplice"), ("es", "Municipalidad del Moravske Toplice"), ("fi", "Moravske Toplicen kunta"), ("fr", "Moravske Toplice"), ("gu", "મોર\u{ac7}વસ\u{acd}ક ટોપ\u{acd}લાઈસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मोराव\u{94d}सक\u{947} टोप\u{94d}लिस नगरपालिका"), ("hr", "Općina Moravske Toplice"), ("hu", "Alsómarác község"), ("id", "Kotamadya Moravske Toplice"), ("it", "Moravske Toplice"), ("ja", "モラフスケ・トプリツェ"), ("kn", "ಮೊರಾವ\u{ccd}ಸ\u{ccd}ಕ\u{cc6} ಟಾಪ\u{ccd}ಲ\u{cbf}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "모라브스케 토플리체 지방 자치제"), ("lt", "Moravskės Toplisės savivaldybė"), ("lv", "Moravskes Toplices province"), ("mr", "मोराव\u{94d}स\u{94d} टोप\u{94d}लाइस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Moravske Toplice"), ("nb", "MOravske Toplice kommune"), ("nl", "Moravske Toplice"), ("no", "MOravske Toplice kommune"), ("pl", "Gmina Moravske Toplice"), ("pt", "Município de Moravske Toplice"), ("ro", "Comuna Moravske Toplice"), ("ru", "Моравске Топлице"), ("si", "මොරවස\u{dca}කේ ටොප\u{dca}ල\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Moravske Toplice"), ("sr", "Општина Моравске Топлице"), ("sr_Latn", "Opština Moravske Toplice"), ("sv", "Moravske Toplice"), ("ta", "மொர\u{bbe}வ\u{bcd}ஸ\u{bcd}கி டோபிலிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c4b}ర\u{c3e}వస\u{c4d}క\u{c3f} ట\u{c4b}ప\u{c4d}ల\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโมราฟสเกโทพล\u{e34}ซ"), ("tr", "Moravske Toplice Belediyesi"), ("uk", "Моравське Топлице"), ("ur", "موراوسکی توپلیکی میونسپلٹی"), ("vi", "Đô thị tự trị Moravske Toplice"), ("zh", "摩拉瓦-托普利采")]),
                        unofficial_name_list: ["Moravske Toplice"].to_vec(),
                    }
                ),
                (
                    "079",
                    Subdivision{
                        name: "079",
                        country_alpha2: Alpha2::SI,
                        code: "079",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3392772), longitude: Some(14.9599807), max_latitude: Some(46.3516621), min_latitude: Some(46.32539180000001), max_longitude: Some(14.9753351), min_longitude: Some(14.9433924)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية موزيريه"), ("bn", "মোজির\u{9cd}জে পৌরসভ\u{9be}"), ("ccp", "𑄟\u{11127}𑄎\u{11128}𑄢\u{11134}𑄎𑄬"), ("ceb", "Mozirje"), ("cs", "Občina Mozirje"), ("da", "Mozirje Municipality"), ("de", "Mozirje"), ("el", "Μοζίρτζε"), ("en", "Mozirje"), ("es", "Mozirje"), ("fi", "Mozirjen kunta"), ("fr", "Mozirje"), ("gu", "મોઝિર\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मोज\u{93c}िरय\u{947} नगर पालिका"), ("hr", "Općina Mozirje"), ("id", "Kotamadya Mozirje"), ("it", "Mozirje"), ("ja", "モジリェ"), ("kn", "ಮೊಜ\u{cbf}ರ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "모지레"), ("lt", "Mozirjės savivaldybė"), ("lv", "Mozirjes pašvaldība"), ("mr", "मोझिरज\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Mozirje Municipality"), ("nb", "Mozirje kommune"), ("nl", "Mozirje"), ("no", "Mozirje kommune"), ("pl", "Gmina Mozirje"), ("pt", "Mozirje"), ("ro", "Mozirje"), ("ru", "Мозирье"), ("si", "මොස\u{dd2}ර\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Mozirje"), ("sr", "Општина Мозирје"), ("sr_Latn", "Opština Mozirje"), ("sv", "Mozirje"), ("ta", "மொஜிர\u{bcd}ஜெ நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c4b}జ\u{c3f}ర\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโมเซอร\u{e4c}ช\u{e35}"), ("tr", "Mozirce Belediyesi"), ("uk", "Мозирє (община)"), ("ur", "موزیرجی میونسپلٹی"), ("vi", "Mozirje"), ("zh", "莫濟列")]),
                        unofficial_name_list: ["Mozirje"].to_vec(),
                    }
                ),
                (
                    "080",
                    Subdivision{
                        name: "080",
                        country_alpha2: Alpha2::SI,
                        code: "080",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6581381), longitude: Some(16.1610293), max_latitude: Some(46.6840267), min_latitude: Some(46.6226685), max_longitude: Some(16.2032004), min_longitude: Some(16.1393097)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مدينة مورسكا سوبوتا"), ("bn", "ম\u{9c1}রস\u{9cd}ক\u{9be} সোবোত\u{9be} শহর পৌরসভ\u{9be}"), ("ccp", "𑄟𑄢\u{11134}𑄌\u{11134}𑄇 𑄥\u{1112e}𑄝\u{1112e}𑄑"), ("ceb", "Murska Sobota"), ("cs", "Městská občina Murska Sobota"), ("da", "Murska Sobota City Municipality"), ("de", "Gemeinde Murska Sobota"), ("el", "Μούρσκα Σομπότα"), ("en", "Murska Sobota"), ("es", "Ciudad Municipalidad Murska Sobota"), ("fi", "Murska Sobota City - kunta"), ("fr", "Murska Sobota"), ("gu", "મ\u{ac1}ર\u{acd}સ\u{acd}કા સોબોટા શહ\u{ac7}ર , મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("he", "מורסקה סובוטה"), ("hi", "म\u{941}र\u{94d}सका सोबोटा शहर नगर पालिका"), ("hr", "Gradska općina Murska Sobota"), ("hu", "Muraszombat község"), ("id", "Kotamadya Murska Sobota City"), ("it", "Murska Sobota"), ("ja", "ムルスカ・ソボタ"), ("kn", "ಮುರ\u{ccd}ಕಾ ಸೋಬೊಟಾ ನಗರ ಪುರಸಭ\u{cc6}"), ("ko", "무르스카소보타 도시 지방 자치제"), ("lt", "Murska Sobotos savivaldybė"), ("lv", "Murska Sobotas pilsēta"), ("mr", "म\u{941}र\u{94d}स\u{94d}का सोबोटा शहर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Murska Sobota City Municipality"), ("nb", "Murska Sobota City kommune"), ("nl", "Murska Sobota"), ("no", "Murska Sobota City kommune"), ("pl", "Gmina miejska Murska Sobota"), ("pt", "Cidade Municipal de Murska Sobota"), ("ro", "Comuna urbană Murska Sobota"), ("ru", "Мурска-Собота"), ("si", "ම\u{dd4}ර\u{dca}ස\u{dca}ක\u{dcf} සොබෝට\u{dcf} ස\u{dd2}ට\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Mestna občina Murska Sobota"), ("sr", "Општина Мурска Собота"), ("sr_Latn", "Opština Murska Sobota"), ("sv", "Murska Sobota City kommun"), ("ta", "முர\u{bcd}ஸக\u{bbe} சோபோட\u{bbe} நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ముర\u{c4d}స\u{c4d}క\u{c3e} స\u{c4b}బ\u{c4b}ట\u{c3e} స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเม\u{e34}คสกา โซโบตา"), ("tr", "Murska Sobota Belediyesi"), ("uk", "Мурська Собота"), ("ur", "مورسکا سوبوتا شہر میونسپلٹی"), ("vi", "Đô thị tự trị Murska Sobota"), ("zh", "穆爾斯卡索博塔市")]),
                        unofficial_name_list: ["Murska Sobota"].to_vec(),
                    }
                ),
                (
                    "081",
                    Subdivision{
                        name: "081",
                        country_alpha2: Alpha2::SI,
                        code: "081",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6097366), longitude: Some(15.1629994), max_latitude: Some(46.6192274), min_latitude: Some(46.5915292), max_longitude: Some(15.1772468), min_longitude: Some(15.1346814)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية موتا"), ("bn", "ম\u{9c1}ত\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄟\u{1112a}𑄑"), ("ceb", "Muta"), ("cs", "Občina Muta"), ("da", "Muta Municipality"), ("de", "Muta"), ("el", "Μούτα"), ("en", "Muta"), ("es", "Muta"), ("fi", "Mutan kunta"), ("fr", "Muta"), ("gu", "મ\u{ac1}ટા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "म\u{942}टा नगर पालिका"), ("hr", "Općina Muta"), ("id", "Kotamadya Muta"), ("it", "Muta"), ("ja", "ムタ"), ("kn", "ಮುಟಾ ಪುರಸಭ\u{cc6}"), ("ko", "무타"), ("lt", "Mutos savivaldybė"), ("lv", "Mutas pašvaldība"), ("mr", "म\u{941}टा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Muta Municipality"), ("nb", "Muta kommune"), ("nl", "Muta"), ("no", "Muta kommune"), ("pl", "Gmina Muta"), ("pt", "Muta"), ("ro", "Muta"), ("ru", "Мута"), ("si", "ම\u{dd4}ට\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Muta"), ("sr", "Општина Мута"), ("sr_Latn", "Opština Muta"), ("sv", "Muta"), ("ta", "மூட\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మూట\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ม\u{e39}ทา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Muta Belediyesi"), ("uk", "Мута"), ("ur", "متا میونسپلٹی"), ("vi", "Đô thị tự trị Muta"), ("zh", "穆塔")]),
                        unofficial_name_list: ["Muta"].to_vec(),
                    }
                ),
                (
                    "082",
                    Subdivision{
                        name: "082",
                        country_alpha2: Alpha2::SI,
                        code: "082",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2718663), longitude: Some(14.3156932), max_latitude: Some(46.2859105), min_latitude: Some(46.2609036), max_longitude: Some(14.3292088), min_longitude: Some(14.2762726)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ناكلو"), ("bn", "ন\u{9be}ক\u{9cd}লো পৌরসভ\u{9be}"), ("ccp", "𑄚𑄇\u{11134}𑄣\u{1112e}"), ("ceb", "Naklo"), ("cs", "Občina Naklo"), ("da", "Naklo Municipality"), ("de", "Gemeinde Naklo"), ("el", "Νάκλο"), ("en", "Naklo"), ("es", "Municipalidad del Naklo"), ("fi", "Naklon kunta"), ("fr", "Naklo"), ("gu", "નાક\u{acd}લો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "नाक\u{94d}लो नगर पालिका"), ("hr", "Općina Naklo"), ("id", "Kotamadya Naklo"), ("it", "Naklo"), ("ja", "ナクロ"), ("kn", "ನಕ\u{ccd}ಲೊ ಪುರಸಭ\u{cc6}"), ("ko", "나클로 지방 자치제"), ("lt", "Naklo savivaldybė"), ("lv", "Naklo pašvaldība"), ("mr", "नाकलो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Naklo Municipality"), ("nb", "Naklo kommune"), ("nl", "Naklo"), ("no", "Naklo kommune"), ("pl", "Gmina Naklo"), ("pt", "Município de Naklo"), ("ro", "Comuna Naklo"), ("ru", "Накло"), ("si", "නක\u{dca}ලෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Naklo"), ("sr", "Општина Накло"), ("sr_Latn", "Opština Naklo"), ("sv", "Naklo kommun"), ("ta", "ந\u{bbe}க\u{bcd}கிலோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "న\u{c3e}క\u{c4d}ల\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องนาโคล"), ("tr", "Naklo Belediyesi"), ("uk", "Накло (община)"), ("ur", "ناکلو میونسپلٹی"), ("vi", "Đô thị tự trị Naklo"), ("zh", "納克洛鎮")]),
                        unofficial_name_list: ["Naklo"].to_vec(),
                    }
                ),
                (
                    "083",
                    Subdivision{
                        name: "083",
                        country_alpha2: Alpha2::SI,
                        code: "083",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3179494), longitude: Some(14.9470385), max_latitude: Some(46.3264519), min_latitude: Some(46.3150209), max_longitude: Some(14.9579734), min_longitude: Some(14.9376884)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية نازارج"), ("bn", "ন\u{9be}জ\u{9be}রজে পৌরসভ\u{9be}"), ("ccp", "𑄚𑄎𑄢\u{11134}𑄎𑄬"), ("ceb", "Nazarje"), ("cs", "Občina Nazarje"), ("da", "Nazarje"), ("de", "Nazarje"), ("el", "Κοινότητα Ναζάρτζε"), ("en", "Nazarje"), ("es", "Municipalidad del Nazarje"), ("fi", "Nazarjen kunta"), ("fr", "Nazarje"), ("gu", "નજર\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "नज\u{93c}ारिए नगरपालिका"), ("hr", "Općina Nazarje"), ("id", "Kotamadya Nazarje"), ("it", "Nazarje"), ("ja", "ナザリェ"), ("kn", "ನಜರ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "나자레"), ("lt", "Nazarjės savivaldybė"), ("lv", "Nazarjes pašvaldība"), ("mr", "नजार\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Nazarje"), ("nb", "Nazarje kommune"), ("nl", "Nazarje"), ("no", "Nazarje kommune"), ("pl", "Gmina Nazarje"), ("pt", "Nazarje"), ("ro", "Nazarje"), ("ru", "Назарье"), ("si", "නසර\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Nazarje"), ("sr", "Општина Назарје"), ("sr_Latn", "Opština Nazarje"), ("sv", "Nazarje"), ("ta", "நச\u{bbe}ர\u{bcd}ஜ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "నజ\u{c3e}ర\u{c4d}య\u{c47} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลนาซาร\u{e4c}เจ"), ("tr", "Nazarje Belediyesi"), ("uk", "Назарє (община)"), ("ur", "نازارجی میونسپلٹی"), ("vi", "Đô thị tự trị Nazarje"), ("zh", "納扎列")]),
                        unofficial_name_list: ["Nazarje"].to_vec(),
                    }
                ),
                (
                    "084",
                    Subdivision{
                        name: "084",
                        country_alpha2: Alpha2::SI,
                        code: "084",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9549755), longitude: Some(13.6493044), max_latitude: Some(45.9668771), min_latitude: Some(45.9457426), max_longitude: Some(13.6625232), min_longitude: Some(13.6335026)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نوفا جوريتسا"), ("be", "Горад Нова-Горыца"), ("bg", "Нова Горица"), ("bs", "Nova Gorica"), ("ca", "Nova Gorica"), ("ccp", "𑄚\u{1112e}𑄞 𑄉\u{1112e}𑄢\u{11128}𑄇"), ("ceb", "Nova Gorica"), ("cs", "Městská občina Nova Gorica"), ("da", "Nova Gorica"), ("de", "Nova Gorica"), ("el", "Νόβα Γκόριτσα"), ("en", "Nova Gorica"), ("es", "Nova Gorica"), ("eu", "Nova Gorica"), ("fa", "نوا گریتسا"), ("fi", "Nova Gorica"), ("fr", "Nova Gorica"), ("he", "נובה גוריצה"), ("hr", "Gradska općina Nova Gorica"), ("hu", "Nova Gorica"), ("id", "Nova Gorica"), ("it", "Nova Gorica"), ("ja", "ノヴァ・ゴリツァ"), ("ka", "ნოვა-გორიცა"), ("ko", "노바고리차"), ("lt", "Nova Gorica"), ("mk", "Нова Горица"), ("nb", "Nova Gorica"), ("nl", "Nova Gorica"), ("no", "Nova Gorica"), ("pl", "Gmina miejska Nova Gorica"), ("pt", "Nova Gorica"), ("ro", "Comuna urbană Nova Gorica"), ("ru", "Нова-Горица"), ("sk", "Nova Gorica"), ("sl", "Mestna občina Nova Gorica"), ("sr", "Општина Нова Горица"), ("sr_Latn", "Opština Nova Gorica"), ("sv", "Nova Gorica"), ("ta", "நொவ\u{bbe} கோரிக\u{bbe}"), ("th", "นอวากอร\u{e35}ตซา"), ("tr", "Nova Gorica"), ("uk", "Нова Гориця"), ("ur", "نووا گوریتسا"), ("vi", "Nova Gorica"), ("zh", "新戈里察")]),
                        unofficial_name_list: ["Nova Gorica"].to_vec(),
                    }
                ),
                (
                    "085",
                    Subdivision{
                        name: "085",
                        country_alpha2: Alpha2::SI,
                        code: "085",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.80108240000001), longitude: Some(15.1710089), max_latitude: Some(45.8389841), min_latitude: Some(45.7681929), max_longitude: Some(15.2119462), min_longitude: Some(15.127425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مدينة نوفو ميستو"), ("be", "Нова-Места"), ("bn", "নোভো মেস\u{9cd}টো পৌরসভ\u{9be} শহর"), ("ccp", "𑄚\u{1112e}𑄞\u{1112e} 𑄟𑄬𑄌\u{11134}𑄑\u{1112e}"), ("ceb", "Mestna Občina Novo mesto"), ("cs", "Městská občina Novo mesto"), ("da", "Novo Mesto"), ("de", "Stadtgemeinde von Novo Mesto"), ("el", "Κοινότητα Νόβο Μέστο"), ("en", "Novo Mesto"), ("fi", "Novo Meston kaupunki"), ("fr", "Novo Mesto"), ("gu", "નોવો મ\u{ac7}સ\u{acd}ટોની શહ\u{ac7}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "नोवो म\u{947}स\u{94d}टो शहर नगरपालिका"), ("hr", "Gradska općina Novo Mesto"), ("hu", "Novo mesto városi község"), ("id", "Kotamadya Novo Mesto"), ("it", "Novo Mesto"), ("ja", "ノヴォ・メスト"), ("kn", "ನೊವೊ ಮ\u{cc6}ಸ\u{ccd}ಟೊ ನಗರದ ಪುರಸಭ\u{cc6}"), ("ko", "노보메스토 지방 자치 도시"), ("lt", "Novo Mesto miesto savivaldybė"), ("lv", "Novo Mesto pilsētas pašvaldība"), ("mr", "नोवो म\u{947}स\u{94d}टो शहराची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Bandar Novo Mesto"), ("nb", "City kommune av Novo"), ("nl", "Novo Mesto"), ("no", "City kommune av Novo"), ("pl", "Novo Mesto"), ("pt", "Cidade Municipal do Novo"), ("ro", "Comuna urbană Novo mesto"), ("ru", "Ново-Место"), ("si", "නොවො මෙස\u{dca}ටෝ මහ\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Mestna občina Novo mesto"), ("sr", "Општина Ново Место"), ("sr_Latn", "Opština Novo Mesto"), ("sv", "Stadskommun av Novo Mesto"), ("ta", "நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி நோவோ மெஸ\u{bcd}ட\u{bcd}டோ"), ("te", "న\u{c4b}వ\u{c4b} మ\u{c46}స\u{c4d}ట\u{c4b} స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโนโวเมสโต"), ("tr", "Belediye"), ("uk", "Ново Место"), ("ur", "شہر میونسپلٹی نووو میستو"), ("vi", "Đô thị tự trị Novo Mesto"), ("zh", "新梅斯托市")]),
                        unofficial_name_list: ["Novo mesto"].to_vec(),
                    }
                ),
                (
                    "086",
                    Subdivision{
                        name: "086",
                        country_alpha2: Alpha2::SI,
                        code: "086",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5847279), longitude: Some(16.2762351), max_latitude: Some(46.6016558), min_latitude: Some(46.56877859999999), max_longitude: Some(16.2985609), min_longitude: Some(16.2506427)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أودرانسي"), ("bn", "ওদ\u{9cd}র\u{9be}ঞ\u{9cd}চি"), ("bs", "Odranci"), ("ccp", "𑄃\u{11127}𑄖\u{11134}𑄢𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Odranci"), ("cs", "Občina Odranci"), ("da", "Odranci"), ("de", "Odranci"), ("el", "Οντράντσι"), ("en", "Odranci"), ("es", "Odranci"), ("fi", "Odranci"), ("fr", "Odranci"), ("gu", "ઓડ\u{acd}રાન\u{acd}સી"), ("hi", "ओद\u{94d}रा\u{902}ची"), ("hr", "Odranci"), ("hu", "Adorjánfalva"), ("id", "Odranci"), ("it", "Odranci"), ("ja", "オドランツィ"), ("kn", "ಒಡ\u{ccd}ರನ\u{ccd}ಸ\u{cbf}"), ("ko", "오드란치"), ("lt", "Odrancis"), ("lv", "Odranci"), ("mr", "ओद\u{94d}र\u{947}\u{902}सी"), ("ms", "Odranci"), ("nb", "Odranci"), ("nl", "Odranci"), ("no", "Odranci"), ("pl", "Gmina Odranci"), ("pt", "Odranci"), ("ro", "Odranci"), ("ru", "Одранци"), ("si", "ඔඩ\u{dca}රන\u{dca}ස\u{dd2}"), ("sl", "Odranci"), ("sr", "Одранци"), ("sr_Latn", "Odranci"), ("sv", "Odranci"), ("ta", "ஓட\u{bcd}ர\u{bbe}ன\u{bcd}ஸி"), ("te", "ఓడ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c3f}"), ("th", "ออดรานซ\u{e34}"), ("tr", "Odranci"), ("uk", "Одранці"), ("ur", "ودرانکی"), ("vi", "Odranci"), ("zh", "奧德蘭齊")]),
                        unofficial_name_list: ["Odranci"].to_vec(),
                    }
                ),
                (
                    "087",
                    Subdivision{
                        name: "087",
                        country_alpha2: Alpha2::SI,
                        code: "087",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4086282), longitude: Some(16.1508154), max_latitude: Some(46.4156206), min_latitude: Some(46.3940475), max_longitude: Some(16.1776242), min_longitude: Some(16.128301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أورموش"), ("bg", "Ормож"), ("bn", "অরম\u{9c1}জ"), ("ca", "Ormož"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄟\u{1112e}𑄌\u{11134}"), ("ceb", "Občina Ormož"), ("cs", "Občina Ormož"), ("da", "Ormož"), ("de", "Ormož"), ("el", "Ορμόζ"), ("en", "Ormož"), ("es", "Ormož"), ("fa", "ارمژ"), ("fi", "Ormož"), ("fr", "Ormož"), ("gl", "Ormož"), ("gu", "ઓરમોઝ"), ("hi", "ओरमोज"), ("hr", "Ormož"), ("hu", "Ormosd"), ("id", "Ormož"), ("it", "Ormož"), ("ja", "オルモジュ"), ("kn", "ಆರ\u{ccd}ಮೊಝ\u{ccd}"), ("ko", "오르모주"), ("lt", "Ormožas"), ("lv", "Ormoža"), ("mr", "ओरमोज"), ("ms", "Ormoz"), ("nb", "Ormoz"), ("nl", "Ormož"), ("no", "Ormoz"), ("pl", "Ormož"), ("pt", "Ormoz"), ("ro", "Ormož"), ("ru", "Ормож"), ("si", "ඔර\u{dca}මෝස\u{dca}"), ("sk", "Ormož"), ("sl", "Ormož"), ("sr", "Ормож"), ("sr_Latn", "Ormož"), ("sv", "Ormož"), ("ta", "ஓரமே"), ("te", "ఓర\u{c4d}మ\u{c4b}జ\u{c4d}"), ("th", "เม\u{e37}องออร\u{e4c}มอส"), ("tr", "Ormoz"), ("uk", "Ормож"), ("ur", "ورموز"), ("vi", "Ormoz"), ("zh", "奧爾莫日")]),
                        unofficial_name_list: ["Ormož"].to_vec(),
                    }
                ),
                (
                    "088",
                    Subdivision{
                        name: "088",
                        country_alpha2: Alpha2::SI,
                        code: "088",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5291637), longitude: Some(14.6984205), max_latitude: Some(45.5413028), min_latitude: Some(45.5253378), max_longitude: Some(14.7123838), min_longitude: Some(14.6939524)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃\u{1112e}𑄥\u{11128}𑄣\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Osilnica"), ("cs", "Občina Osilnica"), ("de", "Osilnica"), ("en", "Osilnica"), ("fr", "Osilnica"), ("hr", "Općina Osilnica"), ("it", "Osilnica"), ("ja", "オシルニツァ"), ("ko", "오실니차"), ("nl", "Osilnica"), ("pl", "Gmina Osilnica"), ("pt", "Osilnica"), ("ro", "Osilnica"), ("sl", "Občina Osilnica"), ("sr", "Општина Осилница"), ("sr_Latn", "Opština Osilnica"), ("sv", "Osilnica"), ("uk", "Осилниця"), ("zh", "奧西爾尼察")]),
                        unofficial_name_list: ["Osilnica"].to_vec(),
                    }
                ),
                (
                    "089",
                    Subdivision{
                        name: "089",
                        country_alpha2: Alpha2::SI,
                        code: "089",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6373877), longitude: Some(15.5543464), max_latitude: Some(46.64874270000001), min_latitude: Some(46.6278036), max_longitude: Some(15.5811768), min_longitude: Some(15.5424122)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيسنيسكا"), ("bn", "পেস\u{9cd}নিক\u{9be} পৌরসভ\u{9be}"), ("bs", "Pesnica"), ("ccp", "𑄛𑄬𑄌\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Pesnica"), ("cs", "Občina Pesnica"), ("da", "Pesnica Municipality"), ("de", "Pesnica"), ("el", "Πεσνίκα"), ("en", "Pesnica"), ("es", "Pesnica"), ("fi", "Pesnican kunta"), ("fr", "Pesnica"), ("gu", "પ\u{ac7}સ\u{acd}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{947}सनिका नगर पालिका"), ("hr", "Općina Pesnica"), ("id", "Kotamadya Pesnica"), ("it", "Pesnica"), ("ja", "ペスニツァ"), ("kn", "ಪ\u{cc6}ಸ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "페스니차"), ("lt", "Pesnicos savivaldybė"), ("lv", "Pesnicas pašvaldība"), ("mr", "प\u{947}सनीका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Pesnica Municipality"), ("nb", "Pesnica kommune"), ("nl", "Pesnica"), ("no", "Pesnica kommune"), ("pl", "Gmina Pesnica"), ("pt", "Pesnica"), ("ro", "Pesnica"), ("ru", "Песница"), ("si", "පෙස\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Pesnica"), ("sr", "Општина Песница"), ("sr_Latn", "Opština Pesnica"), ("sv", "Pesnica kommun"), ("ta", "பேசனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c46}స\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เปสน\u{e34}ก\u{e49}า"), ("tr", "Pesnica"), ("uk", "Песниця"), ("ur", "پیسنیکا میونسپلٹی"), ("vi", "Pesnica"), ("zh", "佩斯尼察")]),
                        unofficial_name_list: ["Pesnica"].to_vec(),
                    }
                ),
                (
                    "090",
                    Subdivision{
                        name: "090",
                        country_alpha2: Alpha2::SI,
                        code: "090",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.528319), longitude: Some(13.5682895), max_latitude: Some(45.5306107), min_latitude: Some(45.5174117), max_longitude: Some(13.5752889), min_longitude: Some(13.5628854)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Piran"), ("ar", "بيران"), ("be", "Піран"), ("bg", "Пиран"), ("bn", "পির\u{9be}ন"), ("bs", "Piran"), ("ca", "Piran"), ("ccp", "𑄛\u{11128}𑄢𑄚\u{11134}"), ("ceb", "Piran"), ("cs", "Občina Piran"), ("da", "Piran"), ("de", "Piran"), ("el", "Πιράν"), ("en", "Piran"), ("es", "Piran"), ("et", "Piran"), ("eu", "Piran"), ("fa", "پیران"), ("fi", "Piran"), ("fr", "Piran"), ("ga", "Piran"), ("gl", "Piran"), ("gu", "પિરાન"), ("he", "פיראן"), ("hi", "पिरान"), ("hr", "Općina Piran"), ("hu", "Piran"), ("hy", "Պիրան"), ("id", "Piran, Slovenia"), ("is", "Píran"), ("it", "Pirano"), ("ja", "ピラン"), ("ka", "პირანი"), ("kn", "ಪ\u{cbf}ರಾನ\u{ccd}"), ("ko", "피란"), ("lt", "Piranas"), ("lv", "Pirana"), ("mk", "Пиран"), ("mr", "पिरान"), ("ms", "Piran"), ("nb", "Piran"), ("nl", "Piran"), ("no", "Piran"), ("pl", "Gmina Piran"), ("pt", "Piran"), ("ro", "Comuna Piran"), ("ru", "Пиран"), ("si", "ප\u{dd2}රන\u{dca}"), ("sk", "Piran"), ("sl", "Občina Piran"), ("sr", "Општина Пиран"), ("sr_Latn", "Opština Piran"), ("sv", "Piran"), ("ta", "பிர\u{bbe}ன\u{bcd}"), ("te", "ప\u{c3f}ర\u{c3e}న\u{c4d}"), ("th", "ไพราน"), ("tr", "Piran"), ("uk", "Піран"), ("ur", "پیران"), ("vi", "Piran"), ("zh", "皮兰")]),
                        unofficial_name_list: ["Piran/Pirano"].to_vec(),
                    }
                ),
                (
                    "091",
                    Subdivision{
                        name: "091",
                        country_alpha2: Alpha2::SI,
                        code: "091",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6825301), longitude: Some(14.1960582), max_latitude: Some(45.6911654), min_latitude: Some(45.6622463), max_longitude: Some(14.2213455), min_longitude: Some(14.1734865)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيفكا"), ("bn", "পিভক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11128}𑄛\u{11134}𑄇"), ("ceb", "Pivka"), ("cs", "Občina Pivka"), ("da", "Pivka Municipality"), ("de", "Gemeinde Pivka"), ("el", "Πίβκα"), ("en", "Pivka"), ("es", "Municipalidad del Pivka"), ("fi", "Pivkan kunta"), ("fr", "Pivka"), ("gu", "પિવ\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पिवका नगरपालिका"), ("hr", "Općina Pivka"), ("id", "Kotamadya Pivka"), ("it", "San Pietro del Carso"), ("ja", "ピフカ"), ("kn", "ಪ\u{cbf}ವಾಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "피브카"), ("lt", "Pivkos savivaldybė"), ("lv", "Pivkas pašvaldība"), ("mr", "प\u{94d}रिवक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Pivka Municipality"), ("nb", "Pivka kommune"), ("nl", "Pivka"), ("no", "Pivka kommune"), ("pl", "Gmina Pivka"), ("pt", "Pivka"), ("ro", "Pivka"), ("ru", "Пивка"), ("si", "ප\u{dd2}ව\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Pivka"), ("sr", "Пивка"), ("sr_Latn", "Pivka"), ("sv", "Pivka"), ("ta", "ப\u{bc0}வக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c3f}వ\u{c4d}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ป\u{e34}วกา ม\u{e39}น\u{e34}ซ\u{e34}พาล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Pivka Belediyesi"), ("uk", "Півка"), ("ur", "پیوکا میونسپلٹی"), ("vi", "Đô thị tự trị Pivka"), ("zh", "皮夫卡")]),
                        unofficial_name_list: ["Pivka"].to_vec(),
                    }
                ),
                (
                    "092",
                    Subdivision{
                        name: "092",
                        country_alpha2: Alpha2::SI,
                        code: "092",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1559752), longitude: Some(15.5975783), max_latitude: Some(46.1729841), min_latitude: Some(46.14496279999999), max_longitude: Some(15.6142634), min_longitude: Some(15.5724693)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بودتسترتيك"), ("bn", "পোদেরত\u{9cd}রেটেক পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11127}𑄖\u{11134}𑄥𑄬𑄑\u{11133}𑄢\u{11127}𑄑𑄬𑄇\u{11134}"), ("ceb", "Občina Podčetrtek"), ("cs", "Občina Podčetrtek"), ("da", "Podčetrtek Municipality"), ("de", "Podčetrtek"), ("el", "Πόντκετρτεκ"), ("en", "Podčetrtek"), ("es", "Podčetrtek"), ("fi", "Podčetrtekin kunta"), ("fr", "Podčetrtek"), ("gu", "પોડ\u{acd}ક\u{ac7}ત\u{acd}રટ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पॉडस\u{947}ट\u{94d}र\u{947}ट\u{947}क नगर पालिका"), ("hr", "Općina Podčetrtek"), ("id", "Kotamadya Podčetrtek"), ("it", "Podčetrtek"), ("ja", "ポッチェトルテク"), ("kn", "ಪೊಡ\u{ccd}ಕ\u{cc6}ಟ\u{ccd}ರ\u{cc6}ಟ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "포드체트르테크"), ("lt", "Podčetrteko savivaldybė"), ("lv", "Podčetrtekas pašvaldība"), ("mr", "पॉडसत\u{94d}र\u{947}ट\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Podcetrtek Municipality"), ("nb", "Podcetrtek Kommune"), ("nl", "Podčetrtek"), ("no", "Podcetrtek Kommune"), ("pl", "Gmina Podčetrtek"), ("pt", "Podčetrtek"), ("ro", "Podčetrtek"), ("ru", "Подчертрек"), ("si", "පොඩ\u{dca}සේට\u{dca}\u{200d}ර\u{dca}ටෙක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Podčetrtek"), ("sr", "Општина Подчетртек"), ("sr_Latn", "Opština Podčetrtek"), ("sv", "Podčetrtek"), ("ta", "பொட\u{bcd}ஸ\u{bcd}ட\u{bcd}ர\u{bcd}ட\u{bcd}டெக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c3e}డ\u{c4d}\u{200c}స\u{c46}ట\u{c4d}రట\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องพอดเซตระเทค"), ("tr", "Podčetrtek Belediyesi"), ("uk", "Подчетртек"), ("ur", "پودکیترتیک میونسپلٹی"), ("vi", "Podčetrtek"), ("zh", "波德切特泰克")]),
                        unofficial_name_list: ["Podcetrtek"].to_vec(),
                    }
                ),
                (
                    "093",
                    Subdivision{
                        name: "093",
                        country_alpha2: Alpha2::SI,
                        code: "093",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5903517), longitude: Some(15.3310343), max_latitude: Some(46.5930529), min_latitude: Some(46.562105), max_longitude: Some(15.3532612), min_longitude: Some(15.3199505)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بودفيلكا"), ("bn", "পোডভেল\u{9cd}ক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11127}𑄖\u{11134}𑄞𑄬𑄣\u{11134}𑄇"), ("ceb", "Podvelka (munisipyo)"), ("cs", "Občina Podvelka"), ("da", "Podvelka Municipality"), ("de", "Podvelka"), ("el", "Ποντβέλκα"), ("en", "Podvelka"), ("es", "Podvelka"), ("fi", "Podvelkan kunta"), ("fr", "Podvelka"), ("gu", "પોડવ\u{ac7}લ\u{acd}કા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पोडव\u{947}लका नगरपालिका"), ("hr", "Općina Podvelka"), ("id", "Kotamadya Podvelka"), ("it", "Podvelka"), ("ja", "ポドヴェルカ"), ("kn", "ಪೋಡ\u{ccd}ವ\u{cc6}ಲ\u{ccd}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "포드벨카"), ("lt", "Podvelkos savivaldybė"), ("lv", "Podvelkas pašvaldība"), ("mr", "पॉडव\u{947}लक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Podvelka Municipality"), ("nb", "Podvelka Kommune"), ("nl", "Podvelka"), ("no", "Podvelka Kommune"), ("pl", "Gmina Podvelka"), ("pt", "Podvelka"), ("ro", "Podvelka"), ("ru", "Подвелка"), ("si", "පොඩ\u{dca}වෙල\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Podvelka"), ("sr", "Општина Подвелка"), ("sr_Latn", "Opština Podvelka"), ("sv", "Podvelka Kommun"), ("ta", "போடவேல\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4b}డ\u{c4d}వ\u{c46}ల\u{c4d}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องปอดเวลก\u{e49}า"), ("tr", "Podvelka Belediyesi"), ("uk", "Подвелка"), ("ur", "پودویلکا میونسپلٹی"), ("vi", "Podvelka"), ("zh", "波德韦尔卡")]),
                        unofficial_name_list: ["Podvelka"].to_vec(),
                    }
                ),
                (
                    "094",
                    Subdivision{
                        name: "094",
                        country_alpha2: Alpha2::SI,
                        code: "094",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7749704), longitude: Some(14.2133245), max_latitude: Some(45.8089095), min_latitude: Some(45.7388591), max_longitude: Some(14.326014), min_longitude: Some(14.1801071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بوستوينا"), ("bn", "পস\u{9cd}তোন\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11127}𑄌\u{11134}𑄑\u{1112e}𑄌\u{11134}𑄚"), ("ceb", "Postojna"), ("cs", "Občina Postojna"), ("da", "Postojna Municipality"), ("de", "Gemeinde Postojna"), ("el", "Ποστοτζνά"), ("en", "Postojna"), ("es", "Municipalidad del Postojina"), ("fi", "Postojnan kunta"), ("fr", "Municipalité de Postojna"), ("gu", "પોસ\u{acd}ટોજના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पोस\u{94d}टोयना नगर पालिका"), ("hr", "Općina Postojna"), ("id", "Kotamadya Postojna"), ("it", "Postumia"), ("ja", "ポストイナ"), ("kn", "ಪೋಟೋಜನಾ ಪುರಸಭ\u{cc6}"), ("ko", "포스토이나 지방 자치제"), ("lt", "Postoinos savivaldybė"), ("lv", "Postojnas pašvaldība"), ("mr", "पोस\u{94d}टोजन\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Postojna Municipality"), ("nb", "Postojna kommune"), ("nl", "Postojna"), ("no", "Postojna kommune"), ("pl", "Gmina Postojna"), ("pt", "Município de Postojna"), ("ro", "Comuna Postojna"), ("ru", "Постойна"), ("si", "පොස\u{dca}ටොජ\u{dca}න\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Postojna"), ("sr", "Општина Постојна"), ("sr_Latn", "Opština Postojna"), ("sv", "Postojna kommun"), ("ta", "பொஸ\u{bcd}டொஜ\u{bcd}ன\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4b}స\u{c4d}ట\u{c4b}జ\u{c4d}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องโพสตอยนา"), ("tr", "Postojna Belediyesi"), ("uk", "Постойна"), ("ur", "پوستوجنا میونسپلٹی"), ("vi", "Đô thị tự trị Postojna"), ("zh", "波斯托伊納鎮")]),
                        unofficial_name_list: ["Postojna"].to_vec(),
                    }
                ),
                (
                    "095",
                    Subdivision{
                        name: "095",
                        country_alpha2: Alpha2::SI,
                        code: "095",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3017155), longitude: Some(14.4216753), max_latitude: Some(46.3094882), min_latitude: Some(46.2947993), max_longitude: Some(14.4268728), min_longitude: Some(14.4063767)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريدفور"), ("bn", "প\u{9cd}রেডভোর পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄓\u{11133}𑄦𑄞\u{1112e}𑄢\u{11134}"), ("ceb", "Preddvor"), ("cs", "Občina Preddvor"), ("da", "Preddvor Municipality"), ("de", "Preddvor"), ("el", "Πρέντβορ"), ("en", "Preddvor"), ("es", "Preddvor"), ("fi", "Preddvorin kunta"), ("fr", "Preddvor"), ("gu", "પ\u{acd}ર\u{ac7}ડોવર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}र\u{947}ड\u{94d}वोर नगर पालिका"), ("hr", "Općina Preddvor"), ("id", "Kotamadya Preddvor"), ("it", "Preddvor"), ("ja", "プレッドヴォル"), ("kn", "ಪ\u{ccd}ರ\u{cc6}ಡ\u{ccd}ಡೋರ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "프레드보르"), ("lt", "Predoro savivaldybė"), ("lv", "Preddvoras pašvaldība"), ("mr", "प\u{94d}र\u{947}दडोओर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Preddvor Municipality"), ("nb", "Preddvor kommune"), ("nl", "Preddvor"), ("no", "Preddvor kommune"), ("pl", "Gmina Preddvor"), ("pt", "Preddvor"), ("ro", "Preddvor"), ("ru", "Преддвор"), ("si", "ප\u{dca}\u{200d}රෙඩ\u{dca}වොර\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Preddvor"), ("sr", "Општина Преддвор"), ("sr_Latn", "Opština Preddvor"), ("sv", "Preddvor kommun"), ("ta", "ப\u{bcd}ரெட\u{bcd}ட\u{bcd}வொர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c46}డ\u{c4d}వ\u{c3e}ర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเพรดดวอรร\u{e4c}"), ("tr", "Preddvor Municipality"), ("uk", "Преддвор (община)"), ("ur", "پریدوور میونسپلٹی"), ("vi", "Đô thị tự trị Preddvor"), ("zh", "普雷德沃尔")]),
                        unofficial_name_list: ["Preddvor"].to_vec(),
                    }
                ),
                (
                    "096",
                    Subdivision{
                        name: "096",
                        country_alpha2: Alpha2::SI,
                        code: "096",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4199535), longitude: Some(15.8696884), max_latitude: Some(46.4587718), min_latitude: Some(46.3856254), max_longitude: Some(15.9180402), min_longitude: Some(15.8237933)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيتوج"), ("be", "Горад Птуй"), ("bg", "Птуй"), ("bn", "প\u{9c1}জ"), ("bs", "Ptuj"), ("ca", "Ptuj"), ("ccp", "𑄑\u{1112a}𑄌\u{11134}"), ("ceb", "Ptuj"), ("cs", "Městská občina Ptuj"), ("cy", "Ptuj"), ("da", "Ptuj"), ("de", "Ptuj"), ("el", "Πτούι"), ("en", "Ptuj"), ("es", "Ptuj"), ("et", "Ptuj"), ("eu", "Ptuj"), ("fa", "پتوی"), ("fi", "Ptuj"), ("fr", "Ptuj"), ("gl", "Ptuj"), ("gu", "પટ\u{ac1}જ"), ("he", "פטוי"), ("hi", "पत\u{941}ज"), ("hr", "Gradska općina Ptuj"), ("hu", "Ptuj"), ("id", "Ptuj"), ("it", "Ptuj"), ("ja", "プトゥイ"), ("ka", "პტუი"), ("kn", "ಪ\u{ccd}ಯ\u{cc2}ಜ\u{ccd}"), ("ko", "프투이"), ("lt", "Ptujus"), ("lv", "Ptuja"), ("mk", "Птуј"), ("mr", "पट\u{941}ज"), ("ms", "Ptuj"), ("nb", "Ptuj"), ("nl", "Ptuj"), ("no", "Ptuj"), ("pl", "Gmina miejska Ptuj"), ("pt", "Ptuj"), ("ro", "Comuna urbană Ptuj"), ("ru", "Птуй"), ("si", "ප\u{dca}\u{200d}ට\u{dd2}ය\u{dd4}ජ\u{dca}"), ("sk", "Ptuj"), ("sl", "Mestna občina Ptuj"), ("sr", "Општина Птуј"), ("sr_Latn", "Opština Ptuj"), ("sv", "Ptuj"), ("ta", "ப\u{bcd}டுஜ\u{bcd}"), ("te", "పుతుజ\u{c46}"), ("th", "พท\u{e39}ย"), ("tr", "Ptuj"), ("uk", "Птуй"), ("ur", "پتوج"), ("vi", "Ptuj"), ("zh", "普图伊")]),
                        unofficial_name_list: ["Ptuj"].to_vec(),
                    }
                ),
                (
                    "097",
                    Subdivision{
                        name: "097",
                        country_alpha2: Alpha2::SI,
                        code: "097",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.7046395), longitude: Some(16.1572755), max_latitude: Some(46.7195726), min_latitude: Some(46.684222), max_longitude: Some(16.1766865), min_longitude: Some(16.1457567)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{1112a}𑄇\u{1112e}𑄚𑄥\u{11128}"), ("ceb", "Puconci"), ("cs", "Občina Puconci"), ("en", "Puconci"), ("hr", "Općina Puconci"), ("hu", "Battyánd község"), ("pl", "Gmina Puconci"), ("ro", "Comuna Puconci"), ("sl", "Občina Puconci"), ("sr", "Општина Пуцонци"), ("sr_Latn", "Opština Puconci"), ("uk", "Пуцонці")]),
                        unofficial_name_list: ["Puconci"].to_vec(),
                    }
                ),
                (
                    "098",
                    Subdivision{
                        name: "098",
                        country_alpha2: Alpha2::SI,
                        code: "098",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4552612), longitude: Some(15.6389275), max_latitude: Some(46.4661091), min_latitude: Some(46.4427643), max_longitude: Some(15.6586513), min_longitude: Some(15.6198414)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية راتسي فرام"), ("bn", "রেস-ফ\u{9cd}র\u{9be}ম পৌরসভ\u{9be}"), ("bs", "Rače-Fram"), ("ccp", "𑄢\u{11133}𑄠𑄌\u{11134}-𑄜\u{11133}𑄢𑄟\u{11134}"), ("ceb", "Občina Rače-Fram"), ("cs", "Občina Rače-Fram"), ("da", "Rače–Fram Municipality"), ("de", "Rače-Fram"), ("el", "Ρέις-Φραμ"), ("en", "Rače–Fram"), ("es", "Rače-Fram"), ("fi", "Rače–Framin kunta"), ("fr", "Rače-Fram"), ("gu", "રાક\u{ac7}-ફ\u{acd}રામ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "राक\u{947}-फ\u{94d}र\u{948}म नगरपालिका"), ("hr", "Općina Rače - Fram"), ("id", "Kotamadya Rače–Fram"), ("it", "Rače-Fram"), ("kn", "ರಾಸ\u{cc6}-ಫ\u{ccd}ರಮ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "라체프람"), ("lt", "Rače-Framo savivaldybė"), ("lv", "Rače–Framas pašvaldība"), ("mr", "राक\u{947}-फ\u{94d}रम म\u{941}नी"), ("ms", "Municipality of Race-Fram"), ("nb", "Race-Farm Kommune"), ("nl", "Rače-Fram"), ("no", "Race-Farm Kommune"), ("pl", "Gmina Rače-Fram"), ("pt", "Rače-Fram"), ("ro", "Rače-Fram"), ("ru", "Раче-Фрам"), ("si", "රේස\u{dca}-ෆ\u{dca}රම\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Rače-Fram"), ("sr", "Општина Раче - Фрам"), ("sr_Latn", "Opština Rače - Fram"), ("sv", "Race-Farm Kommun"), ("ta", "ரேஸ\u{bcd} –பிரேம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c47}స\u{c4d}-ఫ\u{c4d}ర\u{c3e}మ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ราเซ\u{e48}-ฟร\u{e31}ม"), ("tr", "Race-Fram Belediyesi"), ("uk", "Раче-Фрам"), ("vi", "Rače - Fram"), ("zh", "拉切-弗拉姆")]),
                        unofficial_name_list: ["Race-Fram"].to_vec(),
                    }
                ),
                (
                    "099",
                    Subdivision{
                        name: "099",
                        country_alpha2: Alpha2::SI,
                        code: "099",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0655001), longitude: Some(15.1821191), max_latitude: Some(46.08046119999999), min_latitude: Some(46.0587423), max_longitude: Some(15.1952936), min_longitude: Some(15.1550379)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "رادتشه"), ("bn", "র\u{200d}\u{9cd}য\u{9be}ডেচে"), ("ca", "Radeče"), ("ccp", "𑄢𑄓𑄬𑄇\u{11134}"), ("ceb", "Občina Radeče"), ("cs", "Občina Radeče"), ("da", "Radeče"), ("de", "Radeče"), ("el", "Ραντετσέ"), ("en", "Radeče"), ("es", "Radeče"), ("fa", "رادچه"), ("fi", "Radeče"), ("fr", "Radeče"), ("gu", "રડ\u{ac7}સ\u{ac7}"), ("hi", "राड\u{947}क\u{947}"), ("hr", "Općina Radeče"), ("id", "Radeče"), ("it", "Radeče"), ("ja", "ラデチェ"), ("kn", "ರಾಡ\u{cc6}ಸ\u{cc6}"), ("ko", "라데체"), ("mr", "र\u{945}डिस"), ("ms", "Radece"), ("nb", "Radece"), ("nl", "Radeče"), ("no", "Radece"), ("pl", "Gmina Radeče"), ("pt", "Radeče"), ("ro", "Comuna Radeče"), ("ru", "Радече"), ("si", "රඩෙස\u{dca}"), ("sk", "Radeče"), ("sl", "Občina Radeče"), ("sr", "Општина Радече"), ("sr_Latn", "Opština Radeče"), ("sv", "Radece"), ("ta", "ர\u{bbe}டேஸ\u{bcd}"), ("te", "ర\u{c3e}డ\u{c46}స\u{c4d}"), ("th", "ราเดเซ\u{e48}"), ("tr", "Radece"), ("uk", "Радече (община)"), ("ur", "رادیکی"), ("vi", "Radeče"), ("zh", "拉代切")]),
                        unofficial_name_list: ["Radece"].to_vec(),
                    }
                ),
                (
                    "100",
                    Subdivision{
                        name: "100",
                        country_alpha2: Alpha2::SI,
                        code: "100",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6439196), longitude: Some(16.0402676), max_latitude: Some(46.6494227), min_latitude: Some(46.6255604), max_longitude: Some(16.0574762), min_longitude: Some(16.0276807)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄢𑄓𑄬𑄚\u{11134}𑄥\u{11128}"), ("ceb", "Radenci"), ("cs", "Občina Radenci"), ("de", "Radenci"), ("en", "Radenci"), ("fr", "Radenci"), ("hr", "Općina Radenci"), ("hu", "Radenci"), ("it", "Radenci"), ("ja", "ラデンツィ"), ("ko", "라덴치"), ("nl", "Radenci"), ("pl", "Gmina Radenci"), ("pt", "Radenci"), ("ro", "Radenci"), ("sl", "Občina Radenci"), ("sr", "Општина Раденци"), ("sr_Latn", "Opština Radenci"), ("uk", "Раденці"), ("zh", "拉登齐")]),
                        unofficial_name_list: ["Radenci"].to_vec(),
                    }
                ),
                (
                    "101",
                    Subdivision{
                        name: "101",
                        country_alpha2: Alpha2::SI,
                        code: "101",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.614358), longitude: Some(15.2255472), max_latitude: Some(46.6191192), min_latitude: Some(46.6070295), max_longitude: Some(15.2356657), min_longitude: Some(15.2033572)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية رادلي أوب درافي"), ("bn", "র\u{9be}দলেজে অব দ\u{9cd}র\u{9be}ভি পৌরসভ\u{9be}"), ("ccp", "𑄢𑄓\u{11127}𑄣\u{11134}𑄎𑄬 𑄃\u{11127}𑄛\u{11134} 𑄓\u{11133}𑄢𑄞\u{11128}"), ("ceb", "Radlje ob Dravi"), ("cs", "Občina Radlje ob Dravi"), ("da", "Radlje ob Dravi Municipality"), ("de", "Radlje ob Dravi"), ("el", "Ράντλτζε ομπ Ντραβί"), ("en", "Radlje ob Dravi"), ("es", "Municipalidad de Radjle del Dravi"), ("fi", "Radlje ob Dravin kunta"), ("fr", "Radlje ob Dravi"), ("gu", "રાડ\u{acd}લજ\u{ac7} ઓબ દ\u{acd}રાવી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "राडिय\u{947} ऑब ड\u{94d}रावी नगर पालिका"), ("hr", "Općina Radlje ob Dravi"), ("id", "Kotamadya Radlje ob Dravi"), ("it", "Radlje ob Dravi"), ("ja", "ラドリェ・オブ・ドラヴィ"), ("kn", "ರಾಡ\u{ccd}ಲ\u{ccd}ಜ\u{cc6} ಒಬ\u{ccd} ಡ\u{ccd}ರಾವ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "라들레오브드라비"), ("lt", "Radlio ob Dravio savivaldybė"), ("lv", "Radljes ob Dravu pašvaldība"), ("mr", "राडल\u{947}ज\u{947} ओ. बाब\u{94d}र\u{947}द\u{947}वी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Radlje ob Dravi Municipality"), ("nb", "Radlje ob Dravi kommune"), ("nl", "Radlje ob Dravi"), ("no", "Radlje ob Dravi kommune"), ("pl", "Gmina Radlje ob Dravi"), ("pt", "Radlje ob Dravi"), ("ro", "Radlje ob Dravi"), ("ru", "Радле-об-Драви"), ("si", "රඩ\u{dca}ල\u{dca}ජෙ ඔබ\u{dca} ඩ\u{dca}රව\u{dd3} නගර සභ\u{dcf}ව"), ("sl", "Občina Radlje ob Dravi"), ("sr", "Општина Радље об Драви"), ("sr_Latn", "Opština Radlje ob Dravi"), ("sv", "Radlje ob Dravi"), ("ta", "ர\u{bbe}ட\u{bcd}ல\u{bcd}ஜெ ஒப\u{bcd} ட\u{bcd}ர\u{bbe}வி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3e}డ\u{c3f}ల\u{c4d}జ\u{c46} ఓబ\u{c4d} డ\u{c4d}ర\u{c3e}వ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ร\u{e31}ทจ\u{e34} อฟ ดราว\u{e34} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Radje ob Dravi Belediyesi"), ("uk", "Радлє-об-Драві"), ("ur", "رادلجی اوب دراوی میونسپلٹی"), ("vi", "Radlje ob Dravi"), ("zh", "德拉瓦河畔拉德列")]),
                        unofficial_name_list: ["Radlje ob Dravi"].to_vec(),
                    }
                ),
                (
                    "102",
                    Subdivision{
                        name: "102",
                        country_alpha2: Alpha2::SI,
                        code: "102",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3437361), longitude: Some(14.1740042), max_latitude: Some(46.3553924), min_latitude: Some(46.3284814), max_longitude: Some(14.1977016), min_longitude: Some(14.1554444)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية رادوفلييكا"), ("bn", "র\u{200d}\u{9cd}য\u{9be}দোভ\u{9cd}লজিক\u{9be} পৌরসভ\u{9be}"), ("ca", "Municipalitat de Radovljica"), ("ccp", "𑄢\u{11127}𑄓\u{1112e}𑄛\u{11134}𑄣\u{11134}𑄎\u{11128}𑄇"), ("ceb", "Radovljica"), ("cs", "Občina Radovljica"), ("da", "Radovljica Municipality"), ("de", "Radovljica"), ("el", "Ραντοβλτζίκα"), ("en", "Radovljica"), ("es", "Radovljica"), ("eu", "Radovljica"), ("fi", "Radovljican kunta"), ("fr", "Radovljica"), ("gu", "ર\u{ac7}ડોવ\u{acd}લજીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "राडोवलिका नगरपालिका"), ("hr", "Općina Radovljica"), ("id", "Kotamadya Radovljica"), ("it", "Radovljica"), ("ja", "ラドヴリツァ"), ("kn", "ರಾಡೊವ\u{ccd}ಲ\u{ccd}ಜ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "라도블리차"), ("lt", "Radovlicos savivaldybė"), ("lv", "Radovļjicas pašvaldība"), ("mr", "र\u{945}डोवलीजीक\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Radovljica Municipality"), ("nb", "Radovljica"), ("nl", "Radovljica"), ("no", "Radovljica"), ("pl", "Radovljica Municipality"), ("pt", "Radovljica"), ("ro", "Radovljica"), ("ru", "Радовлица"), ("si", "රඩොල\u{dca}ව\u{dca}ජ\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sk", "Radovljica"), ("sl", "Občina Radovljica"), ("sr", "Општина Радовљица"), ("sr_Latn", "Opština Radovljica"), ("sv", "Radovljica"), ("ta", "ர\u{bbe}டோவில\u{bcd}ஜிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3e}డ\u{c4b}వ\u{c4d}ల\u{c3f}జ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องราดอฟล\u{e4c}จ\u{e34}กา"), ("tr", "Radovljica Belediyesi"), ("uk", "Радовлиця"), ("ur", "رادوولجیکا میونسپلٹی"), ("vi", "Đô thị tự trị Radovljica"), ("zh", "拉多夫利察")]),
                        unofficial_name_list: ["Radovljica"].to_vec(),
                    }
                ),
                (
                    "103",
                    Subdivision{
                        name: "103",
                        country_alpha2: Alpha2::SI,
                        code: "103",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5448323), longitude: Some(14.9660445), max_latitude: Some(46.5489776), min_latitude: Some(46.5341932), max_longitude: Some(14.9775721), min_longitude: Some(14.9472544)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "رافنه ناد كروشكيم"), ("bn", "র\u{9be}ভনে ন\u{9be} করোস\u{9cd}কে"), ("bs", "Ravne na Koroškem"), ("ca", "Ravne na Koroškem"), ("ccp", "𑄢𑄛\u{11134}𑄚𑄬 𑄚 𑄇\u{1112e}𑄢\u{1112e}𑄌\u{11134}𑄇𑄬𑄟\u{11134}"), ("ceb", "Občina Ravne na Koroškem"), ("cs", "Občina Ravne na Koroškem"), ("da", "Ravne na Koroškem"), ("de", "Ravne na Koroškem"), ("el", "Ραβνέ να Κορόσκεμ"), ("en", "Ravne na Koroškem"), ("es", "Ravne na Koroškem"), ("fa", "راونه ناد کرشکم"), ("fi", "Ravnena Koroškem"), ("fr", "Ravne na Koroškem"), ("gu", "રાવન\u{ac7} ના કોરોસ\u{acd}ક\u{ac7}મ"), ("hi", "रवन\u{947} ना कोरोसक\u{947}म"), ("hr", "Općina Ravne na Koroškem"), ("hu", "Ravne na Koroškem"), ("id", "Ravne na Koroškem"), ("it", "Ravne na Koroškem"), ("ja", "ラヴネ・ナ・コロシュケム"), ("kn", "ರಾವ\u{ccd}ನ\u{cc6} ನಾ ಕೊರೊಸ\u{ccd}ಕ\u{cc6}ಮ\u{ccd}"), ("ko", "라브네나코로슈켐"), ("lt", "Ravnė na Koroškemas"), ("lv", "Ravne uz Koroškema"), ("mr", "रावन\u{947} ना कोरोस\u{94d}कम"), ("ms", "Ravne na Koroskem"), ("nb", "Ravne na Koroskem"), ("nl", "Ravne na Koroškem"), ("no", "Ravne na Koroskem"), ("pl", "Gmina Ravne na Koroškem"), ("pt", "Ravne na Koroškem"), ("ro", "Comuna Ravne na Koroškem"), ("ru", "Равне-на-Корошкем"), ("si", "රව\u{dca}නේ න\u{dcf} කොරෝස\u{dca}කෙම\u{dca}"), ("sk", "Ravne na Koroškem"), ("sl", "Občina Ravne na Koroškem"), ("sr", "Општина Равне на Корошкем"), ("sr_Latn", "Opština Ravne na Koroškem"), ("sv", "Ravne na Koroskem"), ("ta", "ரவனே ந\u{bbe} கோரோஸ\u{bcd}கிம\u{bcd}"), ("te", "ర\u{c3e}వ\u{c4d}న\u{c47} న\u{c3e} క\u{c4b}ర\u{c4b}స\u{c4d}క\u{c46}మ\u{c4d}"), ("th", "ราวเนนาคอโรชเคม"), ("tr", "Ravne na Koroškem"), ("uk", "Равне-на-Корошкем"), ("ur", "راونی نا کوروسکیم"), ("vi", "Ravne na Koroškem"), ("zh", "拉夫內納科羅什凱姆")]),
                        unofficial_name_list: ["Ravne na Koroškem"].to_vec(),
                    }
                ),
                (
                    "104",
                    Subdivision{
                        name: "104",
                        country_alpha2: Alpha2::SI,
                        code: "104",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7400303), longitude: Some(14.7265782), max_latitude: Some(45.7731152), min_latitude: Some(45.7043966), max_longitude: Some(14.7646492), min_longitude: Some(14.6829771)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ريبنيتسا"), ("bn", "রিবনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄢\u{11128}𑄛\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Ribnica"), ("cs", "Občina Ribnica"), ("da", "Ribnica"), ("de", "Ribnica"), ("el", "Ριμπνίκα"), ("en", "Ribnica"), ("es", "Ribnica"), ("fi", "Ribnican kunta"), ("fr", "Ribnica"), ("gu", "રિબ\u{acd}નીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रीबनिका नगर पालिका"), ("hr", "Općina Ribnica"), ("id", "Kotamadya Ribnica"), ("it", "Ribnica"), ("ja", "リブニツァ"), ("kk", "Рибница"), ("kn", "ರ\u{cbf}ಬ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "리브니차"), ("lt", "Ribnicos savivaldybė"), ("lv", "Ribnicas pašvaldība"), ("mr", "रिबिना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ribnica Municipality"), ("nb", "Ribnica Kommune"), ("nl", "Ribnica"), ("no", "Ribnica Kommune"), ("pl", "Gmina Ribnica"), ("pt", "Ribnica"), ("ro", "Ribnica"), ("ru", "Рибница"), ("si", "ර\u{dd2}බ\u{dca}න\u{dd2}ක\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Ribnica"), ("sl", "Občina Ribnica"), ("sr", "Општина Рибница"), ("sr_Latn", "Opština Ribnica"), ("sv", "Ribnica"), ("ta", "ரிபனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3f}బ\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องร\u{e34}บน\u{e34}กา"), ("tr", "Ribnica Belediyesi"), ("uk", "Рибниця"), ("ur", "ریبنیکا میونسپلٹی"), ("vi", "Đô thị tự trị Ribnica"), ("zh", "里布尼察")]),
                        unofficial_name_list: ["Ribnica"].to_vec(),
                    }
                ),
                (
                    "105",
                    Subdivision{
                        name: "105",
                        country_alpha2: Alpha2::SI,
                        code: "105",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8055785), longitude: Some(16.0345237), max_latitude: Some(46.8129722), min_latitude: Some(46.79668179999999), max_longitude: Some(16.0586442), min_longitude: Some(16.0035833)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية روغاشوفتسي"), ("bn", "রোগ\u{9be}সভচি পৌরসভ\u{9be}"), ("ccp", "𑄢\u{1112e}𑄉𑄥\u{1112e}𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Občina Rogašovci"), ("cs", "Občina Rogašovci"), ("da", "Rogasovci"), ("de", "Rogašovci"), ("el", "Ρογκασόβκι"), ("en", "Rogašovci"), ("es", "Rogašovci"), ("fi", "Rogašovcin kunta"), ("fr", "Rogašovci"), ("gu", "રોગાસોવ\u{acd}કિ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रोगास\u{94d}योव\u{94d}की नगर पालिका"), ("hr", "Općina Rogašovci"), ("hu", "Szarvaslak"), ("id", "Kotamadya Rogašovci"), ("it", "Rogašovci"), ("ja", "ロガショフツィ"), ("kn", "ರೊಗ\u{ccd}ಸೊಸ\u{ccd}ಕೊ ಪುರಸಭ\u{cc6}"), ("ko", "로가쇼브치"), ("lt", "Ragašovcio savivaldybė"), ("lv", "Rogašovcu pašvaldība"), ("mr", "रोगासोवसि म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Rogasovci Municipality"), ("nb", "Rogasovci Kommune"), ("nl", "Rogašovci"), ("no", "Rogasovci Kommune"), ("pl", "Gmina Rogašovci"), ("pt", "Rogašovci"), ("ro", "Rogašovci"), ("ru", "Рогашовци"), ("si", "රෝග\u{dcf}සොව\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Rogašovci"), ("sr", "Општина Рогашовци"), ("sr_Latn", "Opština Rogašovci"), ("sv", "Rogasovci (kommun)"), ("ta", "ரோக\u{bcd}க\u{bbe}ஸோவிக\u{bcd}கி நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c4b}గస\u{c4b}వ\u{c4d}స\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโรกาซอฟช\u{e35}\u{e48}"), ("tr", "Rogašovci Belediyesi"), ("uk", "Рогашовці"), ("ur", "روجاسووکی میونسپلٹی"), ("vi", "Rogašovci"), ("zh", "羅加舍夫齊")]),
                        unofficial_name_list: ["Rogašovci"].to_vec(),
                    }
                ),
                (
                    "106",
                    Subdivision{
                        name: "106",
                        country_alpha2: Alpha2::SI,
                        code: "106",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2331941), longitude: Some(15.6378802), max_latitude: Some(46.2490379), min_latitude: Some(46.215743), max_longitude: Some(15.6545818), min_longitude: Some(15.6180735)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "روغاشكا سلاتينا"), ("az", "Roqaşka-Slatina"), ("bg", "Рогашка Слатина"), ("bn", "রোগ\u{9be}স\u{9cd}ক\u{9be} স\u{9cd}ল\u{9be}টিন\u{9be}"), ("bs", "Rogaška Slatina"), ("ca", "Rogaška Slatina"), ("ccp", "𑄢\u{1112e}𑄉𑄌\u{11134}𑄇 𑄥\u{11133}𑄣𑄑\u{11128}𑄚"), ("ceb", "Rogaška Slatina (kapital sa munisipyo sa Eslobenya)"), ("cs", "Občina Rogaška Slatina"), ("da", "Rogaška Slatina"), ("de", "Rogaška Slatina"), ("el", "Ρογκάσκα Σλατίνα"), ("en", "Rogaška Slatina"), ("es", "Rogaška Slatina"), ("fa", "رگاشکا سلاتینا"), ("fi", "Rogaška Slatina"), ("fr", "Rogaska Slatina"), ("gu", "રૉગાસ\u{acd}કા સ\u{acd}લ\u{ac7}ટીના"), ("hi", "रोगास\u{94d}का स\u{94d}लातिना"), ("hr", "Rogaška Slatina"), ("id", "Rogaška Slatina"), ("it", "Rogaška Slatina"), ("ja", "ロガーシュカ・スラティナ"), ("kn", "ರೊಗಾಸ\u{ccd}ಕಾ ಸ\u{ccd}ಲಾಟ\u{cbf}ನಾ"), ("ko", "로가슈카슬라티나"), ("lt", "Rogaška Slatina"), ("lv", "Rogaška Slatina"), ("mr", "रोगास\u{94d}क सल\u{945}टिन"), ("ms", "Rogaska Slatina"), ("nb", "Rogaska Slatina"), ("nl", "Rogaška Slatina"), ("no", "Rogaska Slatina"), ("pl", "Rogaška Slatina"), ("pt", "Rogaska Slatina"), ("ro", "Rogaška Slatina"), ("ru", "Рогашка-Слатина"), ("si", "රගොඅස\u{dca}ක\u{dcf} ස\u{dca}ලට\u{dd2}න\u{dcf}"), ("sk", "Rogaška Slatina"), ("sl", "Rogaška Slatina"), ("sr", "Рогашка Слатина"), ("sr_Latn", "Rogaška Slatina"), ("sv", "Rogaska Slatina"), ("ta", "ரோகஸ\u{bcd}க\u{bcd}க\u{bbe} ஸ\u{bcd}ல\u{bbe}ட\u{bcd}டின\u{bbe}"), ("te", "ర\u{c4b}గ\u{c3e}స\u{c4d}క\u{c3e} స\u{c4d}ల\u{c3e}ట\u{c3f}న\u{c3e}"), ("th", "โรกาสกาสลาต\u{e34}นา"), ("tr", "Rogaska Slatina"), ("uk", "Рогашка Слатина"), ("ur", "روجاسکا سلاتینا"), ("vi", "Rogaška Slatina"), ("zh", "羅加什卡斯拉蒂納")]),
                        unofficial_name_list: ["Rogaška Slatina"].to_vec(),
                    }
                ),
                (
                    "107",
                    Subdivision{
                        name: "107",
                        country_alpha2: Alpha2::SI,
                        code: "107",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2258891), longitude: Some(15.7000313), max_latitude: Some(46.240435), min_latitude: Some(46.2198708), max_longitude: Some(15.7396848), min_longitude: Some(15.6835002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية روجاتيك"), ("bn", "রোগ\u{9be}টেক পৌরসভ\u{9be}"), ("ccp", "𑄢\u{1112e}𑄉𑄑𑄬𑄇\u{11134}"), ("ceb", "Rogatec"), ("cs", "Občina Rogatec"), ("da", "Rogatec Municipality"), ("de", "Rogatec"), ("el", "Ρόγκατεκ"), ("en", "Rogatec"), ("es", "Municipalidad del Rogatec"), ("fi", "Rogatecin kunta"), ("fr", "Rogatec"), ("gu", "રોગ\u{ac7}ટ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रोगाट\u{947}क नगर पालिका"), ("hr", "Općina Rogatec"), ("id", "Kotamadya Rogatec"), ("it", "Rogatec"), ("ja", "ロガテツ²"), ("kn", "ರೋಗಾಟ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "로가테츠²"), ("lt", "Rogateko savivaldybė"), ("lv", "Rogatecas pašvaldība"), ("mr", "रोगट\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Rogatec Municipality"), ("nb", "Rogatec kommune"), ("nl", "Rogatec"), ("no", "Rogatec kommune"), ("pl", "Gmina Rogatec"), ("pt", "Rogatec"), ("ro", "Rogatec"), ("ru", "Рогатец"), ("si", "රෝගටෙක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Rogatec"), ("sr", "Општина Рогатец"), ("sr_Latn", "Opština Rogatec"), ("sv", "Rogatec kommun"), ("ta", "ரோகடெக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c4b}గ\u{c3e}ట\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดโรกาเทค"), ("tr", "Rogatec Belediyesi"), ("uk", "Рогатець (община)"), ("ur", "روجاتیک میونسپلٹی"), ("vi", "Đô thị tự trị Rogatec"), ("zh", "羅加泰茨")]),
                        unofficial_name_list: ["Rogatec"].to_vec(),
                    }
                ),
                (
                    "108",
                    Subdivision{
                        name: "108",
                        country_alpha2: Alpha2::SI,
                        code: "108",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5399962), longitude: Some(15.5073987), max_latitude: Some(46.5540659), min_latitude: Some(46.5305317), max_longitude: Some(15.5300521), min_longitude: Some(15.4850029)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية روشه"), ("bn", "র\u{9c1}সে পৌরসভ\u{9be}"), ("ccp", "𑄢𑄅\u{1112a}𑄌\u{11134}"), ("ceb", "Občina Ruše"), ("cs", "Občina Ruše"), ("da", "Ruše Municipality"), ("de", "Gemeinde Ruše"), ("el", "Ρούσε"), ("en", "Ruše"), ("es", "Ruše"), ("fi", "Rušen kunta"), ("fr", "Municipalité de Ruše"), ("gu", "ર\u{ac1}સ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "र\u{941}स\u{947} नगरपालिका"), ("hr", "Općina Ruše"), ("id", "Kotamadya Ruše"), ("it", "Comune di Ruše"), ("ja", "ルシェ"), ("kn", "ರ\u{cc2}ಸೇ ಪುರಸಭ\u{cc6}"), ("ko", "루셰 지방 자치제"), ("lt", "Rušės savivaldybė"), ("lv", "Rušes pašvaldība"), ("mr", "र\u{941}स\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ruse Municipality"), ("nb", "Ruse Kommune"), ("nl", "Ruše"), ("no", "Ruse Kommune"), ("pl", "Gmina Ruše"), ("pt", "Município de Ruse"), ("ro", "Comuna Ruše"), ("ru", "Руше"), ("si", "ර\u{dd4}සේ නගර සභ\u{dcf}ව"), ("sl", "Občina Ruše"), ("sr", "Општина Руше"), ("sr_Latn", "Opština Ruše"), ("sv", "Ruse (kommun)"), ("ta", "ருஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "రూస\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องร\u{e39}เซ\u{e48}"), ("tr", "Ruse Belediyesi"), ("uk", "Руше"), ("ur", "روز میونسپلٹی"), ("vi", "Đô thị tự trị Ruse"), ("zh", "魯舍鎮")]),
                        unofficial_name_list: ["Ruše"].to_vec(),
                    }
                ),
                (
                    "109",
                    Subdivision{
                        name: "109",
                        country_alpha2: Alpha2::SI,
                        code: "109",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6520534), longitude: Some(15.1820701), max_latitude: Some(45.668519), min_latitude: Some(45.62551999999999), max_longitude: Some(15.2062962), min_longitude: Some(15.1472437)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيميتش"), ("bn", "সেমিক পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄟\u{11128}𑄇\u{11134}"), ("ceb", "Občina Semič"), ("cs", "Občina Semič"), ("da", "Semič Municipality"), ("de", "Semič"), ("el", "Σεμίτς"), ("en", "Semič"), ("es", "Municipalidad Semič"), ("fi", "Semičn kunta"), ("fr", "Municipalité de Semič"), ("gu", "સ\u{ac7}મિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}मिक नगरपालिका"), ("hr", "Općina Semič"), ("id", "Kotamadya Semič"), ("it", "Semič"), ("ja", "セミチ"), ("kn", "ಸ\u{cc6}ಮ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "세미치"), ("lt", "Semičiaus savivaldybė"), ("lv", "Semičas pašvaldība"), ("mr", "स\u{947}मिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Semic Municipality"), ("nb", "Semic kommune"), ("nl", "Semič"), ("no", "Semic kommune"), ("pl", "Gmina Semič"), ("pt", "Semič"), ("ro", "Semič"), ("ru", "Семич"), ("si", "සෙම\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Semič"), ("sr", "Општина Семич"), ("sr_Latn", "Opština Semič"), ("sv", "Semic kommun"), ("ta", "செமிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}మ\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซม\u{e34}ค"), ("tr", "Semic Belediyesi"), ("uk", "Семич"), ("ur", "سیمیک میونسپلٹی"), ("vi", "Đô thị tự trị Semic"), ("zh", "賽米奇")]),
                        unofficial_name_list: ["Semic"].to_vec(),
                    }
                ),
                (
                    "110",
                    Subdivision{
                        name: "110",
                        country_alpha2: Alpha2::SI,
                        code: "110",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0129508), longitude: Some(15.2979868), max_latitude: Some(46.0263166), min_latitude: Some(46.00335159999999), max_longitude: Some(15.3319916), min_longitude: Some(15.2884441)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيفتيتسا"), ("bn", "সেভনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄛\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Sevnica"), ("cs", "Občina Sevnica"), ("da", "Sevnica Municipality"), ("de", "Gemeinde Sevnica"), ("el", "Σεβνίκα"), ("en", "Sevnica"), ("es", "Municipalidad del Sevnica"), ("fi", "Sevnican kunta"), ("fr", "Municipalité de Sevnica"), ("gu", "સ\u{ac7}વ\u{acd}નીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}व\u{94d}निका नगर पालिका"), ("hr", "Općina Sevnica"), ("id", "Kotamadya Sevnica"), ("it", "Comune di Sevnica"), ("ja", "セヴニツァ"), ("kn", "ಸ\u{cc6}ವ\u{cbf}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "세브니차 지방 자치제"), ("lt", "Sevnicos savivaldybė"), ("lv", "Sevnicas pašvaldība"), ("mr", "स\u{947}वनीच म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sevnica Municipality"), ("nb", "Sevnica kommune"), ("nl", "Sevnica"), ("no", "Sevnica kommune"), ("pl", "Gmina Sevnica"), ("pt", "Município de Sevnica"), ("ro", "Comuna Sevnica"), ("ru", "Севница"), ("si", "සෙව\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Sevnica"), ("sr", "Општина Севница"), ("sr_Latn", "Opština Sevnica"), ("sv", "Sevnica kommun"), ("ta", "செவனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}వ\u{c4d}\u{200c}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โบวล\u{e34}\u{e48}ง พ\u{e49}อยท\u{e4c}, แอนก\u{e34}วก\u{e34}ลลา"), ("tr", "Sevnica Belediyesi"), ("uk", "Севниця (община)"), ("ur", "سیونیکا میونسپلٹی"), ("vi", "Đô thị tự trị Sevnica"), ("zh", "塞夫尼察鎮")]),
                        unofficial_name_list: ["Sevnica"].to_vec(),
                    }
                ),
                (
                    "111",
                    Subdivision{
                        name: "111",
                        country_alpha2: Alpha2::SI,
                        code: "111",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7093519), longitude: Some(13.8738185), max_latitude: Some(45.7290231), min_latitude: Some(45.68053279999999), max_longitude: Some(13.8946603), min_longitude: Some(13.8292526)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيجانا"), ("bn", "সেজ\u{9be}ন\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄎𑄚"), ("ceb", "Občina Sežana"), ("cs", "Občina Sežana"), ("da", "Sežana Municipality"), ("de", "Gemeinde Sežana"), ("el", "Σεζάνα"), ("en", "Sežana"), ("es", "Municipalidad del Sežana"), ("fi", "Sežanan kunta"), ("fr", "Municipalité de Sežana"), ("gu", "સ\u{ac7}ઝાના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}जाना नगर पालिका"), ("hr", "Općina Sežana"), ("id", "Kotamadya Sežana"), ("it", "Comune di Sežana"), ("ja", "セジャーナ"), ("kn", "ಸೀಜಾನಾ ಪುರಸಭ\u{cc6}"), ("ko", "세자나 지방 자치제"), ("lt", "Sežanos savivaldybė"), ("lv", "Sežanas pašvaldība"), ("mr", "स\u{947}झाना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sezana Municipality"), ("nb", "Sezana Kommune"), ("nl", "Sežana"), ("no", "Sezana Kommune"), ("pl", "Gmina Sežana"), ("pt", "Município de Sezana"), ("ro", "Comuna Sežana"), ("ru", "Сежана"), ("si", "සෙසන\u{dcf} නගරසභ\u{dcf}ව"), ("sl", "Občina Sežana"), ("sr", "Општина Сежана"), ("sr_Latn", "Opština Sežana"), ("sv", "Sezana (kommun)"), ("ta", "செச\u{bbe}ன நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}జ\u{c3e}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("tr", "Sezana Belediyesi"), ("uk", "Сежана"), ("ur", "سیزانا میونسپلٹی"), ("vi", "Đô thị tự trị Sezana"), ("zh", "塞扎納鎮")]),
                        unofficial_name_list: ["Sežana"].to_vec(),
                    }
                ),
                (
                    "112",
                    Subdivision{
                        name: "112",
                        country_alpha2: Alpha2::SI,
                        code: "112",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5075851), longitude: Some(15.0768162), max_latitude: Some(46.5183186), min_latitude: Some(46.4874225), max_longitude: Some(15.1006887), min_longitude: Some(15.056726)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية مدينة سلوفني غراديك"), ("bn", "স\u{9cd}লোভেঞ\u{9cd}জ গ\u{9cd}র\u{9be}দেক শহর পৌরসভ\u{9be}"), ("ccp", "𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134} 𑄉\u{11133}𑄢𑄓𑄬𑄇\u{11134}"), ("ceb", "Slovenj Gradec"), ("cs", "Městská občina Slovenj Gradec"), ("da", "Slovenj Gradec City Municipality"), ("de", "Gemeinde Slovenj Gradec"), ("el", "Σλοβέντς Γκράντεκ"), ("en", "Slovenj Gradec"), ("fi", "Slovenj Gradec City - kunta"), ("fr", "Slovenj Gradec"), ("gu", "સ\u{acd}લોવ\u{ac7}જ ગ\u{acd}ર\u{ac7}ડ\u{ac7}ક, શહ\u{ac7}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}लोव\u{947}\u{902}ज ग\u{94d}र\u{947}ड\u{947}क शहर नगरपालिका"), ("hr", "Gradska općina Slovenj Gradec"), ("id", "Kotamadya Slovenj Gradec City"), ("it", "Comune di Slovenj Gradec City"), ("ja", "スロヴェニ・グラデツ"), ("kn", "ಸ\u{ccd}ಲೊವ\u{cc6}ನ\u{ccd}ಜ\u{ccd} ಗ\u{ccd}ರ\u{ccd}ಯಾಡ\u{cc6}ಕ\u{ccd} ನಗರ ಪುರಸಭ\u{cc6}"), ("ko", "슬로벤그라데츠 도시 지방 자치제"), ("lt", "Slovėn Gradeco miesto savivaldybė"), ("lv", "Slovenjas Gradecas pilsētas pašvaldība"), ("mr", "स\u{94d}लोव\u{947}न\u{94d}ज ग\u{94d}र\u{945}ड\u{947}च शहर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Slovenj Gradec City Municipality"), ("nb", "Slovenj Gradec City kommune"), ("nl", "Slovenj Gradec (gemeente)"), ("no", "Slovenj Gradec City kommune"), ("pl", "Gmina miejska Slovenj Gradec"), ("pt", "Cidade Slovenj Gradec"), ("ro", "Comuna urbană Slovenj Gradec"), ("ru", "Словень-Градец"), ("si", "ස\u{dca}ලෝවෙන\u{dca}ජ\u{dca} ග\u{dca}රඩෙක\u{dca} ස\u{dd2}ට\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Mestna občina Slovenj Gradec"), ("sr", "Општина Словењ Градец"), ("sr_Latn", "Opština Slovenj Gradec"), ("sv", "Slovenj Gradec City kommun"), ("ta", "ஸ\u{bcd}லோவேஞ\u{bcd} க\u{bcd}ர\u{bbe}டெக\u{bcd} நகரம\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ల\u{c4b}వ\u{c46}ంజ\u{c4d} గ\u{c4d}ర\u{c3e}డ\u{c46}క\u{c4d} స\u{c3f}ట\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสโลเวนจ\u{e4c} กราเดค"), ("tr", "Slovenj Gradec Belediyesi"), ("uk", "Словень Градець"), ("ur", "سلووینج جرادیک شہر میونسپلٹی"), ("vi", "Đô thị tự trị Slovenj Gradec"), ("zh", "斯洛文尼格拉代茨市")]),
                        unofficial_name_list: ["Slovenj Gradec"].to_vec(),
                    }
                ),
                (
                    "113",
                    Subdivision{
                        name: "113",
                        country_alpha2: Alpha2::SI,
                        code: "113",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3919813), longitude: Some(15.5727868), max_latitude: Some(46.4034978), min_latitude: Some(46.3749754), max_longitude: Some(15.5960519), min_longitude: Some(15.5426134)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سلوفينيسكا بيستريسا"), ("bn", "স\u{9cd}লোভেনস\u{9cd}ক\u{9be} বিস\u{9cd}ত\u{9cd}রিক\u{9be}"), ("bs", "Slovenska Bistrica"), ("ca", "Slovenska Bistrica"), ("ccp", "𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134}𑄇 𑄝\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄇"), ("ceb", "Slovenska Bistrica"), ("cs", "Občina Slovenska Bistrica"), ("da", "Slovenska Bistrica"), ("de", "Slovenska Bistrica"), ("el", "Σλοβένσκα Μπίστρικα"), ("en", "Slovenska Bistrica"), ("es", "Slovenska Bistrica"), ("eu", "Slovenska Bistrica"), ("fa", "سلونسکا بیستریتسا"), ("fi", "Slovenska Bistrica"), ("fr", "Slovenska Bistrica"), ("gu", "સ\u{acd}લોવ\u{ac7}ન\u{acd}સ\u{acd}કા બિસ\u{acd}ટ\u{acd}રિકા"), ("hi", "स\u{94d}लोव\u{947}न\u{94d}स\u{94d}का बिस\u{94d}ट\u{94d}रिका"), ("hr", "Općina Slovenska Bistrica"), ("hu", "Vendbeszterce"), ("id", "Slovenska Bistrica"), ("it", "Slovenska Bistrica"), ("ja", "スロヴェンスカ・ビストリツァ"), ("kn", "ಸ\u{ccd}ಲೋವ\u{cc6}ನ\u{ccd}ಸ\u{ccd}ಕಾ ಬ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ"), ("ko", "슬로벤스카비스트리차"), ("lt", "Slovenska Bistrica"), ("lv", "Slovenska Bistrica"), ("mk", "Словенска Бистрица"), ("mr", "स\u{94d}लोव\u{94d}ह\u{947}न\u{94d}स\u{94d} बिस\u{94d}ट\u{94d}रिका"), ("ms", "Slovenska Bistrica"), ("nb", "Slovenska Bistrica"), ("nl", "Slovenska Bistrica"), ("no", "Slovenska Bistrica"), ("pl", "Gmina Slovenska Bistrica"), ("pt", "Slovenska Bistrica"), ("ro", "Comuna Slovenska Bistrica"), ("ru", "Словенска-Бистрица"), ("si", "ස\u{dca}ලෝවෙන\u{dca}ස\u{dca}ක\u{dcf} බ\u{dd2}ස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ක\u{dcf}"), ("sk", "Slovenska Bistrica"), ("sl", "Občina Slovenska Bistrica"), ("sr", "Општина Словенска Бистрица"), ("sr_Latn", "Opština Slovenska Bistrica"), ("sv", "Slovenska Bistrica"), ("ta", "ஸ\u{bcd}லோவின\u{bcd}ஸ\u{bcd}க\u{bbe} பிஸ\u{bcd}டரிக\u{bbe}"), ("te", "స\u{c4d}ల\u{c4b}వ\u{c46}న\u{c4d}స\u{c3f}క\u{c3e} బ\u{c3f}స\u{c4d}ట\u{c4d}ర\u{c3f}క\u{c3e}"), ("th", "สโลเวนสกาบ\u{e34}สทร\u{e34}กา"), ("tr", "Slovenska Bistrica"), ("uk", "Словенська Бистриця"), ("ur", "سلووینسکا بیستریکا"), ("vi", "Slovenska Bistrica"), ("zh", "斯洛文尼亞比斯特里察")]),
                        unofficial_name_list: ["Slovenska Bistrica"].to_vec(),
                    }
                ),
                (
                    "114",
                    Subdivision{
                        name: "114",
                        country_alpha2: Alpha2::SI,
                        code: "114",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3369191), longitude: Some(15.4214708), max_latitude: Some(46.3451701), min_latitude: Some(46.3302031), max_longitude: Some(15.4486172), min_longitude: Some(15.4101433)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سلوفنيسكه كونجيتسه"), ("bg", "Словенске Конице"), ("bn", "স\u{9cd}লোভেনস\u{9cd}কে কোঞ\u{9cd}জিচে"), ("bs", "Slovenske Konjice"), ("ca", "Slovenske Konjice"), ("ccp", "𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134}𑄇𑄬 𑄇\u{1112e}𑄚\u{11134}𑄎\u{11128}𑄌\u{11134}"), ("ceb", "Slovenske Konjice"), ("cs", "Občina Slovenske Konjice"), ("da", "Slovenske Konjice"), ("de", "Slovenske Konjice"), ("el", "Σλοβένσκε Κοντζίκε"), ("en", "Slovenske Konjice"), ("es", "Slovenske Konjice"), ("fa", "سلونسکه کنییتسه"), ("fi", "Slovenske Konjice"), ("fr", "Slovenske Konjice"), ("gu", "સ\u{acd}લોવ\u{ac7}\u{a82}સ\u{acd}ક કોન\u{acd}જિસ"), ("hi", "स\u{94d}लोव\u{947}न\u{94d}स\u{94d}क कौन\u{94d}यिस"), ("hr", "Slovenske Konjice"), ("hu", "Slovenske Konjice"), ("id", "Slovenske Konjice"), ("it", "Slovenske Konjice"), ("ja", "スロヴェンスケ・コニツェ"), ("kn", "ಸ\u{ccd}ಲೋವ\u{cc6}ನ\u{ccd}ಸ\u{ccd}ಕ\u{cc6} ಕೊಂಜೈಸ\u{ccd}"), ("lt", "Slovenske Kondžicė"), ("lv", "Slovenske Konjice"), ("mr", "स\u{94d}लोव\u{94d}ह\u{947}न\u{94d}स\u{94d}क कोनजिस"), ("ms", "Slovenske Konjice"), ("nb", "Slovenske Konjice"), ("nl", "Slovenske Konjice"), ("no", "Slovenske Konjice"), ("pl", "Gmina Slovenske Konjice"), ("pt", "Slovenske Konjice"), ("ro", "Comuna Slovenske Konjice"), ("ru", "Словенске-Конице"), ("si", "ස\u{dca}ලෝවෙන\u{dca}ස\u{dca}කේ කොන\u{dca}ජ\u{dd2}ස\u{dca}"), ("sk", "Slovenske Konjice"), ("sl", "Občina Slovenske Konjice"), ("sr", "Словенске Коњице"), ("sr_Latn", "Slovenske Konjice"), ("sv", "Slovenske Konjice"), ("ta", "ஸ\u{bcd}லோவின\u{bcd}ஸ\u{bcd}கி கொஞ\u{bcd}சிஸ\u{bcd}"), ("te", "స\u{c4d}ల\u{c4b}వ\u{c46}న\u{c4d}స\u{c4d}\u{200c}క\u{c46} క\u{c4b}ంజ\u{c3f}స\u{c4d}"), ("th", "สโลเวนสเกคอนจ\u{e35}ซ"), ("tr", "Slovenske Konjice"), ("uk", "Словенське Коніце (община)"), ("ur", "سلووینسکی کونجیکی"), ("vi", "Slovenske Konjice"), ("zh", "斯洛文尼亞科尼采")]),
                        unofficial_name_list: ["Slovenske Konjice"].to_vec(),
                    }
                ),
                (
                    "115",
                    Subdivision{
                        name: "115",
                        country_alpha2: Alpha2::SI,
                        code: "115",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4674331), longitude: Some(15.7640547), max_latitude: Some(46.4813213), min_latitude: Some(46.4546657), max_longitude: Some(15.7884309), min_longitude: Some(15.7470676)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ستارتش"), ("bn", "স\u{9cd}ট\u{9be}স পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄑𑄢\u{11134}"), ("ceb", "Občina Starše"), ("cs", "Občina Starše"), ("da", "Starše Municipality"), ("de", "Starše"), ("el", "Στάρσε"), ("en", "Starše"), ("es", "Starše"), ("fi", "Staršen kunta"), ("fr", "Starše"), ("gu", "સ\u{acd}ટાર\u{acd}સ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}टार\u{94d}स नगरपालिका"), ("hr", "Općina Starše"), ("id", "Kotamadya Starše"), ("it", "Starše"), ("ja", "スタルチェ"), ("kn", "ಸ\u{ccd}ಟಾರ\u{ccd}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "스타르셰"), ("lt", "Staršės savivaldybė"), ("lv", "Staršes pašvaldība"), ("mr", "स\u{94d}टारस\u{94d} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Starse Municipality"), ("nb", "Starse Kommune"), ("nl", "Starše"), ("no", "Starse Kommune"), ("pl", "Gmina Starše"), ("pt", "Starše"), ("ro", "Starše"), ("ru", "Старше"), ("si", "ස\u{dca}ටර\u{dca}සේ නගර සභ\u{dcf}ව"), ("sl", "Občina Starše"), ("sr", "Општина Старше"), ("sr_Latn", "Opština Starše"), ("sv", "Starse Kommun"), ("ta", "ஸ\u{bcd}ட\u{bbe}ர\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ట\u{c3e}ర\u{c4d}స\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสตาร\u{e4c}เซ"), ("tr", "Starse Belediyesi"), ("uk", "Старше"), ("ur", "اسٹارسی میونسپلٹی"), ("vi", "Starše"), ("zh", "斯塔爾舍")]),
                        unofficial_name_list: ["Starše"].to_vec(),
                    }
                ),
                (
                    "116",
                    Subdivision{
                        name: "116",
                        country_alpha2: Alpha2::SI,
                        code: "116",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8016061), longitude: Some(16.0368654), max_latitude: Some(46.8055527), min_latitude: Some(46.7860315), max_longitude: Some(16.0584371), min_longitude: Some(16.0083605)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سفيتي يوري في سلوفينسكيخ"), ("bn", "ভেতি জ\u{9c1}রিজ অব স\u{9cd}ক\u{9cd}য\u{9be}বনিকি পৌরসভ\u{9be}"), ("ccp", "𑄞𑄬𑄑\u{11128} 𑄎\u{1112a}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Občina Sveti Jurij ob Ščavnici"), ("cs", "Občina Sveti Jurij ob Ščavnici"), ("da", "Sveti Jurij ob Ščavnici Municipality"), ("de", "Sveti Jurij"), ("el", "Σβέτι Τζούριτζ ομπ Σκαβνίτσι"), ("en", "Sveti Jurij"), ("fi", "Sveti Jurij ob Ščavnicin kunta"), ("fr", "Sveti Jurij"), ("gu", "સ\u{acd}વ\u{ac7}તી જ\u{ac1}રીજ ઓબ સ\u{acd}કાવનિકી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}व\u{947}ती ज\u{942}रीय ऑब स\u{94d}कावनीकी नगरपालिका"), ("hr", "Općina Sveti Jurij ob Ščavnici"), ("id", "Kotamadya Sveti Jurij ob Ščavnici"), ("it", "Sveti Jurij ob Ščavnici"), ("ja", "スヴェティ・ユリイ・オプ・シュチャヴニツィ"), ("kn", "ಸ\u{ccd}ವ\u{cc6}ಟ\u{cbf} ಜುರ\u{cbf}ಜ\u{ccd} ಒಬ\u{ccd} ಸ\u{ccd}ಸ\u{ccd}ವಾವ\u{cbf}ನ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "스베티유리오브슈차브니치"), ("lt", "Šventasis Jurijus Ščavnicoje"), ("lv", "Sveti Jurija ob Ščavnicu pašvaldība"), ("mr", "स\u{94d}व\u{947}ति ज\u{94d}य\u{942}रिज स\u{94d}कावि\u{902}ची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sveti Jurij ob Scavnici Municipality"), ("nb", "Sveti Jurij Kommune"), ("nl", "Sveti Jurij"), ("no", "Sveti Jurij Kommune"), ("pl", "Gmina Sveti Jurij ob Ščavnici"), ("pt", "Sveti Jurij ob Ščavnici"), ("ro", "Sveti Jurij"), ("ru", "Святи-Юрий-об-Шчавници"), ("si", "ස\u{dca}වෙට\u{dd2} ජ\u{dd4}ර\u{dd2}ජ\u{dca} ඔබ\u{dca} ස\u{dca}කව\u{dca}න\u{dd2}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Sveti Jurij ob Ščavnici"), ("sr", "Општина Свети Јуриј об Шчавници"), ("sr_Latn", "Opština Sveti Jurij ob Ščavnici"), ("sv", "Sveti Jurij"), ("ta", "ஸ\u{bcd}வெடி ஜுரைஜ\u{bcd} ஒப\u{bcd} ஸ\u{bcd}கேவனிசி நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}వ\u{c46}ట\u{c3f} జూర\u{c3f}జ\u{c4d} ఓబ\u{c4d} స\u{c4d}క\u{c3e}వ\u{c3f}న\u{c4d}స\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสเตว\u{e34} เจออ\u{e34}จ ออบ สกาฟน\u{e34}ซ\u{e35}"), ("tr", "Sveti Jurij ob Ščavnici Belediyesi"), ("uk", "Светий Юрій-об-Щавниці"), ("ur", "سویتی جوریج اوب سکاونیکی میونسپلٹی"), ("vi", "Sveti Jurij ob Ščavnici"), ("zh", "什察夫尼西河畔斯韋提尤爾利")]),
                        unofficial_name_list: ["Sveti Jurij"].to_vec(),
                    }
                ),
                (
                    "117",
                    Subdivision{
                        name: "117",
                        country_alpha2: Alpha2::SI,
                        code: "117",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2433699), longitude: Some(14.4192223), max_latitude: Some(46.25425420000001), min_latitude: Some(46.2210118), max_longitude: Some(14.4447634), min_longitude: Some(14.396405)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شينتشور"), ("bn", "সেঙ\u{9cd}ক\u{9c1}র পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄇\u{1112a}𑄢\u{11134}"), ("ceb", "Občina Šenčur"), ("cs", "Občina Šenčur"), ("da", "Šenčur"), ("de", "Šenčur"), ("el", "Σένκουρ"), ("en", "Šenčur"), ("es", "Municipalidad del Šenčur"), ("fi", "Šenčurin kunta"), ("fr", "Šenčur"), ("gu", "સ\u{ac7}ન\u{acd}ક\u{acd}ય\u{ac1}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}\u{902}स\u{941}र नगरपालिका"), ("hr", "Općina Šenčur"), ("id", "Kotamadya Šenčur"), ("it", "Šenčur"), ("ja", "シェンチュル"), ("kn", "ಸ\u{cc6}ಂಕ\u{cc2}ರ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "셴추르"), ("lt", "Šenčuro savivaldybė"), ("lv", "Šenčuras pašvaldība"), ("mr", "सि\u{902}चन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sencur Municipality"), ("nb", "Sencur kommune"), ("nl", "Šenčur"), ("no", "Sencur kommune"), ("pl", "Gmina Šenčur"), ("pt", "Šenčur"), ("ro", "Šenčur"), ("ru", "Шенчур"), ("si", "සෙන\u{dca}කර\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Šenčur"), ("sr", "Општина Шенчур"), ("sr_Latn", "Opština Šenčur"), ("sv", "Sencur kommun"), ("ta", "செனகர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంకుర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเซนเคอร\u{e4c}"), ("tr", "Sencur Belediyesi"), ("uk", "Шенчур (община)"), ("ur", "سینکور میونسپلٹی"), ("vi", "Šenčur"), ("zh", "申丘爾")]),
                        unofficial_name_list: ["Šencur"].to_vec(),
                    }
                ),
                (
                    "118",
                    Subdivision{
                        name: "118",
                        country_alpha2: Alpha2::SI,
                        code: "118",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6862839), longitude: Some(15.7103566), max_latitude: Some(46.7074672), min_latitude: Some(46.6317092), max_longitude: Some(15.8059742), min_longitude: Some(15.6207647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شينتيلي"), ("bn", "সেন\u{9cd}টিলি পৌরসভ\u{9be}"), ("bs", "Šentilj"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11128}𑄣\u{11134}"), ("ceb", "Občina Šentilj"), ("cs", "Občina Šentilj"), ("da", "Šentilj Municipality"), ("de", "Šentilj"), ("el", "Σεντίλτζ"), ("en", "Šentilj"), ("es", "Šentilj"), ("fi", "Šentiljin kunta"), ("fr", "Šentilj"), ("gu", "સ\u{ac7}ન\u{acd}ટીલ\u{acd}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}\u{902}तिल\u{94d}ज नगर पालिका"), ("hr", "Općina Šentilj"), ("id", "Kotamadya Šentilj"), ("it", "Šentilj"), ("ja", "シェンティリ"), ("kn", "ಸ\u{cc6}ಂಟ\u{cbf}ಲ\u{ccd}ಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "셴틸"), ("lt", "Šentilio savivaldybė"), ("lv", "Šentiljas pašvaldība"), ("mr", "स\u{947}\u{902}टील\u{94d}ज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sentilj Municipality"), ("nb", "Sentilj Kommune"), ("nl", "Šentilj"), ("no", "Sentilj Kommune"), ("pl", "Gmina Šentilj"), ("pt", "Šentilj"), ("ro", "Šentilj"), ("ru", "Шентиль"), ("si", "සෙන\u{dca}ට\u{dd2}ල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Šentilj"), ("sr", "Општина Шентиљ"), ("sr_Latn", "Opština Šentilj"), ("sv", "Šentilj"), ("ta", "சென\u{bcd}டில\u{bcd}ஜ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంట\u{c3f}ల\u{c4d}జ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซนท\u{e34}ลจ\u{e4c}"), ("tr", "Šentilj Belediyesi"), ("uk", "Шентиль"), ("ur", "سینتیلج میونسپلٹی"), ("vi", "Šentilj"), ("zh", "申蒂利")]),
                        unofficial_name_list: ["Šentilj"].to_vec(),
                    }
                ),
                (
                    "119",
                    Subdivision{
                        name: "119",
                        country_alpha2: Alpha2::SI,
                        code: "119",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.843413), longitude: Some(15.3378312), max_latitude: Some(45.8503768), min_latitude: Some(45.8343709), max_longitude: Some(15.3516799), min_longitude: Some(15.3294207)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شينتيرني"), ("bn", "সেন\u{9cd}টজ\u{9be}রনেজ পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄎𑄢\u{11134}𑄚𑄬𑄌\u{11134}"), ("ceb", "Občina Šentjernej"), ("cs", "Občina Šentjernej"), ("da", "Šentjernej Municipality"), ("de", "Šentjernej"), ("el", "Κοινότητα Σέντζεμετζ"), ("en", "Šentjernej"), ("es", "Municipalidad del Šentjernej"), ("fi", "Šentjernejin kunta"), ("fr", "Šentjernej"), ("gu", "સ\u{ac7}ન\u{acd}ટજ\u{ac7}ર\u{acd}ન\u{ac7}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}\u{902}ज\u{947}र\u{94d}न\u{947}ज"), ("hr", "Općina Šentjernej"), ("id", "Kotamadya Šentjernej"), ("it", "Šentjernej"), ("ja", "シェントイェルニェイ"), ("kn", "ಸ\u{cc6}ನ\u{ccd}ಟ\u{ccd}ಜ\u{cc6}ರ\u{ccd}ನ\u{cc6}ಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "셴체르네이"), ("lt", "Šenternėjaus savivaldybė"), ("lv", "Šentjernejas pašvaldība"), ("mr", "स\u{947}\u{902}टज\u{947}र\u{94d}न\u{947}ज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Sentjernej"), ("nb", "Sentjernej kommune"), ("nl", "Šentjernej"), ("no", "Sentjernej kommune"), ("pl", "Gmina Šentjernej"), ("pt", "Šentjernej"), ("ro", "Šentjernej"), ("ru", "Шентджернедж"), ("si", "සෙන\u{dca}ට\u{dca}ජර\u{dca}නේජ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Šentjernej"), ("sr", "Општина Шентјернеј"), ("sr_Latn", "Opština Šentjernej"), ("sv", "Šentjernej"), ("ta", "செண\u{bcd}ட\u{bcd}ஜெரனேஜ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంట\u{c4d}జ\u{c46}ర\u{c4d}న\u{c46}జ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเซนต\u{e4c}เท\u{e35}ยร\u{e4c}เนจ"), ("tr", "Sentjernej Belediyesi"), ("uk", "Шентєрней"), ("ur", "سینتجیرنیج میونسپلٹی"), ("vi", "Šentjernej"), ("zh", "申特耶爾內伊")]),
                        unofficial_name_list: ["Šentjernej"].to_vec(),
                    }
                ),
                (
                    "120",
                    Subdivision{
                        name: "120",
                        country_alpha2: Alpha2::SI,
                        code: "120",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1425286), longitude: Some(15.4473443), max_latitude: Some(46.3118567), min_latitude: Some(46.0655551), max_longitude: Some(15.5358711), min_longitude: Some(15.3301529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شينتيور"), ("bn", "সেন\u{9cd}ট\u{9c1}র পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134}𑄎\u{1112a}𑄢\u{11134}"), ("ceb", "Občina Šentjur"), ("cs", "Občina Šentjur"), ("da", "Šentjur"), ("de", "Gemeinde Šentjur"), ("el", "Σέντζουρ"), ("en", "Šentjur"), ("es", "Šentjur"), ("fi", "Šentjurin kunta"), ("fr", "Municipalité de Šentjur"), ("gu", "સ\u{ac7}ન\u{acd}ટજ\u{ac2}ર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}\u{902}त\u{94d}य\u{941}र नगर पालिका"), ("hr", "Općina Šentjur"), ("id", "Kotamadya Šentjur"), ("it", "Comune di Šentjur"), ("ja", "シェントユル"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ಜರ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "센튜르 지방 자치제"), ("lt", "Šentjuro savivaldybė"), ("lv", "Šentjuras pašvaldība"), ("mr", "स\u{947}ण\u{94d}टज\u{942}र म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sentjur Municipality"), ("nb", "Sentjur Kommune"), ("nl", "Šentjur pri Celju"), ("no", "Sentjur Kommune"), ("pl", "Gmina Šentjur"), ("pt", "Sentjur"), ("ro", "Comuna Šentjur"), ("ru", "Шентьюр"), ("si", "සන\u{dca}ච\u{dd4}වර\u{dca}"), ("sl", "Občina Šentjur"), ("sr", "Општина Шентјур"), ("sr_Latn", "Opština Šentjur"), ("sv", "Sentjur kommun"), ("ta", "சென\u{bcd}டஜூர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంట\u{c4d}జూర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเซนเจอร\u{e4c}"), ("tr", "Sentjur Belediyesi"), ("uk", "Шентюр (община)"), ("ur", "سینتجور میونسپلٹی"), ("vi", "Đô thị tự trị Sentjur"), ("zh", "申特尤爾鎮")]),
                        unofficial_name_list: ["Šentjur pri Celju"].to_vec(),
                    }
                ),
                (
                    "121",
                    Subdivision{
                        name: "121",
                        country_alpha2: Alpha2::SI,
                        code: "121",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9072257), longitude: Some(15.292258), max_latitude: Some(45.9128124), min_latitude: Some(45.8978091), max_longitude: Some(15.2965868), min_longitude: Some(15.2679185)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شكوتسيان"), ("bn", "স\u{9cd}কোকজ\u{9be}ন পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄇\u{11127}𑄇\u{11134}𑄎𑄚\u{11134}"), ("ceb", "Občina Škocjan"), ("cs", "Občina Škocjan"), ("da", "Škocjan Municipality"), ("de", "Škocjan"), ("el", "Σκοκτζάν"), ("en", "Škocjan"), ("es", "Škocjan"), ("fi", "Škocjanin kunta"), ("fr", "Škocjan"), ("gu", "સ\u{acd}કોક\u{acd}જ\u{ac7}ન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}कोक\u{94d}य\u{947}न नगर पालिका"), ("hr", "Općina Škocjan"), ("id", "Kotamadya Škocjan"), ("it", "Škocjan"), ("ja", "シュコツィアン"), ("kn", "ಸ\u{ccd}ಕೋಜನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "슈코찬"), ("lt", "Škocjano savivaldybė"), ("lv", "Škocjanas pašvaldība"), ("mr", "स\u{94d}कोकोन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Skocjan Municipality"), ("nb", "Skocjan kommune"), ("nl", "Škocjan"), ("no", "Skocjan kommune"), ("pl", "Gmina Škocjan"), ("pt", "Škocjan"), ("ro", "Škocjan"), ("ru", "Шкоцьян"), ("si", "ස\u{dca}කොක\u{dca}ජ\u{dcf}න\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Škocjan"), ("sr", "Општина Шкоцјан"), ("sr_Latn", "Opština Škocjan"), ("sv", "Skocjan kommun"), ("ta", "ஸ\u{bcd}கேஅஜன\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}క\u{c4b}స\u{c4d}జ\u{c3e}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "สคอคแจน"), ("tr", "Skocjan Belediyesi"), ("uk", "Шкоцян"), ("ur", "سکوکجان میونسپلٹی"), ("vi", "Škocjan"), ("zh", "斯科茨揚")]),
                        unofficial_name_list: ["Škocjan"].to_vec(),
                    }
                ),
                (
                    "122",
                    Subdivision{
                        name: "122",
                        country_alpha2: Alpha2::SI,
                        code: "122",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1671294), longitude: Some(14.3058337), max_latitude: Some(46.1796641), min_latitude: Some(46.1543495), max_longitude: Some(14.3454528), min_longitude: Some(14.2863019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شكوفيا لوكا"), ("bn", "স\u{9cd}কোফিয\u{9bc}\u{9be} লোক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄇\u{11127}𑄛\u{11134}𑄎 𑄣\u{1112e}𑄇"), ("ceb", "Občina Škofja Loka"), ("cs", "Občina Škofja Loka"), ("da", "Škofja Loka Municipality"), ("de", "Škofja Loka"), ("el", "Σκόφτζα Λόκα"), ("en", "Škofja Loka"), ("es", "Škofja Loka"), ("fi", "Škofja Lokan kunta"), ("fr", "Škofja Loka"), ("gu", "સ\u{acd}કોફજા લોકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}कोफिया लोका नगरपालिका"), ("hr", "Općina Škofja Loka"), ("id", "Kotamadya Škofja Loka"), ("it", "Škofja Loka"), ("ja", "シュコーフィア・ロカ"), ("kn", "ಸ\u{ccd}ಕೋಜಾ ಲೋಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "슈코퍄로카 지방 자치제"), ("lt", "Škofja Lokos savivaldybė"), ("lv", "Škofjas Lokas pašvaldība"), ("mr", "स\u{94d}कोफजा लोका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Skofja Loka Municipality"), ("nb", "Skofja Loka kommune"), ("nl", "Škofja Loka"), ("no", "Skofja Loka kommune"), ("pl", "Gmina Škofja Loka"), ("pt", "Skofja Loka"), ("ro", "Comuna Škofja Loka"), ("ru", "Шкофья-Лока"), ("si", "ස\u{dca}කොෆ\u{dca}ජ\u{dcf} ලෝක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Škofja Loka"), ("sr", "Општина Шкофја Лока"), ("sr_Latn", "Opština Škofja Loka"), ("sv", "Skofja Loka kommun"), ("ta", "ஸ\u{bcd}கொபிஜெ லோக\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}క\u{c4b}ఫ\u{c4d}జ\u{c3e} ల\u{c4b}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "สคอฟยา โลคา"), ("tr", "Škofja Loka"), ("uk", "Шкофя Лока"), ("ur", "سکوفجا لوکا میونسپلٹی"), ("vi", "Đô thị tự trị Skofja Loka"), ("zh", "斯科菲亞洛卡")]),
                        unofficial_name_list: ["Škofja Loka"].to_vec(),
                    }
                ),
                (
                    "123",
                    Subdivision{
                        name: "123",
                        country_alpha2: Alpha2::SI,
                        code: "123",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.98596510000001), longitude: Some(14.5721903), max_latitude: Some(45.9968959), min_latitude: Some(45.9709285), max_longitude: Some(14.5893571), min_longitude: Some(14.5509336)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شكوفليتسا"), ("bn", "স\u{9cd}কোফ\u{9cd}লিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄇\u{11127}𑄛\u{11134}𑄎\u{11128}𑄇"), ("ceb", "Občina Škofljica"), ("cs", "Občina Škofljica"), ("da", "Škofljica Municipality"), ("de", "Škofljica"), ("el", "Σκοφλτζίκα"), ("en", "Škofljica"), ("es", "Škofljica"), ("fi", "Škofljican kunta"), ("fr", "Škofljica"), ("gu", "સ\u{acd}કોફ\u{acd}લજિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}कोफ\u{93c}लिका नगर पालिका"), ("hr", "Općina Škofljica"), ("id", "Kotamadya Škofljica"), ("it", "Škofljica"), ("ja", "シュコフリツァ"), ("kn", "ಸ\u{ccd}ಕೋಫ\u{ccd}ಲ\u{cbf}ಕಜಾ ಪುರಸಭ\u{cc6}"), ("ko", "슈코플리차"), ("lt", "Škoflicos savivaldybė"), ("lv", "Škofljicas pašvaldība"), ("mr", "स\u{94d}कोफिजिका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Skofljica Municipality"), ("nb", "Skofljica kommune"), ("nl", "Škofljica"), ("no", "Skofljica kommune"), ("pl", "Gmina Škofljica"), ("pt", "Škofljica"), ("ro", "Škofljica"), ("ru", "Шкофлица"), ("si", "ස\u{dca}කොෆ\u{dca}ල\u{dca}ජ\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Škofljica"), ("sr", "Општина Шкофљица"), ("sr_Latn", "Opština Škofljica"), ("sv", "Skofljica kommun"), ("ta", "ஸ\u{bcd}கொபிள\u{bcd}ஜிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}క\u{c4b}ఫ\u{c4d}ల\u{c3f}జ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสคอร\u{e4c}ฟจ\u{e34}คา"), ("tr", "Skoflika Belediyesi"), ("uk", "Шкофліца (община)"), ("ur", "سکوفلجیکا میونسپلٹی"), ("vi", "Škofljica"), ("zh", "什科夫利察")]),
                        unofficial_name_list: ["Škofljica"].to_vec(),
                    }
                ),
                (
                    "124",
                    Subdivision{
                        name: "124",
                        country_alpha2: Alpha2::SI,
                        code: "124",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2287025), longitude: Some(15.5190353), max_latitude: Some(46.2359701), min_latitude: Some(46.2236306), max_longitude: Some(15.5324245), min_longitude: Some(15.5001446)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شماريه بري يلشاخ"), ("bn", "ম\u{9be}রজে প\u{9cd}রি জলশ\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄟𑄢\u{11134}𑄎𑄬 𑄛\u{11133}𑄢\u{1112d} 𑄎𑄬𑄣\u{11134}𑄥𑄦\u{11134}"), ("ceb", "Občina Šmarje pri Jelšah"), ("cs", "Občina Šmarje pri Jelšah"), ("da", "Šmarje pri Jelšah Municipality"), ("de", "Šmarje pri Jelšah"), ("el", "Σμάρτζε πρι Τζελσάχ"), ("en", "Šmarje pri Jelšah"), ("es", "Municipalidad del Šmarje pri Jelšah"), ("fi", "Šmarje pri Jelšahin kunta"), ("fr", "Šmarje pri Jelšah"), ("gu", "સ\u{acd}મરજ\u{ac7} પ\u{acd}રી જ\u{ac7}લ\u{acd}સહ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}मारिय\u{947} प\u{94d}री ज\u{947}ल\u{94d}साह नगरपालिका"), ("hr", "Općina Šmarje pri Jelšah"), ("id", "Kotamadya Šmarje pri Jelšah"), ("it", "Šmarje pri Jelšah"), ("ja", "シュマリェ・プリ・イェルシャフ"), ("kn", "ಸ\u{ccd}ಮಾರ\u{ccd}ಜ\u{cc6} ಪ\u{ccd}ರೈ ಜ\u{cc6}ಲ\u{ccd}ಸಾ ಪುರಸಭ\u{cc6}"), ("ko", "슈마레프리옐샤흐"), ("lt", "Šmare pri Jelša"), ("lv", "Šmarjes pri Jelšahas pašvaldība"), ("mk", "Општина Шмарје кај Јелше"), ("mr", "समरज\u{947}\u{902} प\u{94d}री ज\u{947}लसा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Smarje pri Jelsah Municipality"), ("nb", "Smarje pri Jelsah kommune"), ("nl", "Šmarje pri Jelšah"), ("no", "Smarje pri Jelsah kommune"), ("pl", "Gmina Šmarje pri Jelšah"), ("pt", "Šmarje pri Jelšah"), ("ro", "Šmarje pri Jelšah"), ("ru", "Шмарье-при-Елшах"), ("si", "ස\u{dca}මර\u{dca}ජේ ප\u{dca}\u{200d}ර\u{dd2} ජෙල\u{dca}ස\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Šmarje pri Jelšah"), ("sr", "Општина Шмарје при Јелшах"), ("sr_Latn", "Opština Šmarje pri Jelšah"), ("sv", "Smarje pri Jelsah kommun"), ("ta", "சம\u{bbe}ர\u{bcd}ஜே பிரி ஜல\u{bcd}ஸ\u{bbe}ஹ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}మ\u{c3e}ర\u{c4d}జ\u{c3f} ప\u{c4d}ర\u{c3f} జ\u{c46}ల\u{c4d}స\u{c3e}హ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสมาเร\u{e35}ย ปร\u{e35} เยลซาห\u{e4c}"), ("tr", "Smarje Pri Jelsah"), ("uk", "Шмарє-при-Єлшах (община)"), ("ur", "سمارجی پری جیلساح میونسپلٹی"), ("vi", "Šmarje pri Jelšah"), ("zh", "耶爾沙赫附近什馬列")]),
                        unofficial_name_list: ["Šmarje pri Jelšah"].to_vec(),
                    }
                ),
                (
                    "125",
                    Subdivision{
                        name: "125",
                        country_alpha2: Alpha2::SI,
                        code: "125",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.32903719999999), longitude: Some(15.0333937), max_latitude: Some(46.3375545), min_latitude: Some(46.3243351), max_longitude: Some(15.0429404), min_longitude: Some(15.0239169)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شمارتنو أوب باكي"), ("bn", "স\u{9cd}ম\u{9be}র\u{9cd}টনো পৌরসভ\u{9be}"), ("ccp", "𑄌\u{11133}𑄟𑄢\u{11134}𑄑\u{11134}𑄚\u{1112e} 𑄃\u{11127}𑄛\u{11134} 𑄛𑄇\u{11128}"), ("ceb", "Občina Šmartno ob Paki"), ("cs", "Občina Šmartno ob Paki"), ("da", "Šmartno ob Paki Municipality"), ("de", "Šmartno ob Paki"), ("el", "Σμάρτνο ομπ Πάκι"), ("en", "Šmartno ob Paki"), ("es", "Municipalidad Šmartno ob Paki"), ("fi", "Šmartno ob Pakin kunta"), ("fr", "Šmartno ob Paki"), ("gu", "સ\u{acd}માર\u{acd}ટનો ઓબ પાકી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}मार\u{94d}त\u{94d}नो ओब पाकी नगर पालिका"), ("hr", "Općina Šmartno ob Paki"), ("id", "Kotamadya Šmartno ob Paki"), ("it", "Šmartno ob Paki"), ("ja", "シュマルトノ・オプ・パキ"), ("kn", "ಸ\u{ccd}ರ\u{ccd}ಮಾರ\u{ccd}ಟ\u{ccd}ನೋ ಒಬ ಪಾಕ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "슈마르트노오브파키"), ("lt", "Šmartnas prie Pakos"), ("lv", "Šmartno ob Paku pašvaldība"), ("mr", "स\u{94d}मार\u{94d}टनो ओब पाकी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Smartno ob Paki Municipality"), ("nb", "Smartno ob Paki Kommune"), ("nl", "Šmartno ob Paki"), ("no", "Smartno ob Paki Kommune"), ("pl", "Gmina Šmartno ob Paki"), ("pt", "Šmartno ob Paki"), ("ro", "Šmartno ob Paki"), ("ru", "Шмартно-об-Паки"), ("si", "ස\u{dca}ම\u{dcf}ර\u{dca}ට\u{dca}නෝ ඔබ\u{dca} පක\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Šmartno ob Paki"), ("sr", "Општина Шмартно об Паки"), ("sr_Latn", "Opština Šmartno ob Paki"), ("sv", "Smartno ob Paki Kommun"), ("ta", "ஸ\u{bcd}ம\u{bbe}ர\u{bcd}ட\u{bcd}னரோ ஒப\u{bcd} ப\u{bbe}க\u{bcd}கி நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}మ\u{c3e}ర\u{c4d}ట\u{c4d}న\u{c4b} ఓబ\u{c4d} ప\u{c3e}క\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสมาร\u{e4c}ทโน ออบ พาค\u{e34}"), ("tr", "Smartno ob Paki Belediyesi"), ("uk", "Шмартно-об-Пакі (община)"), ("ur", "سمارتنو اوب پکی میونسپلٹی"), ("vi", "Šmartno ob Paki"), ("zh", "帕卡河畔什馬爾特諾")]),
                        unofficial_name_list: ["Šmartno ob Paki"].to_vec(),
                    }
                ),
                (
                    "126",
                    Subdivision{
                        name: "126",
                        country_alpha2: Alpha2::SI,
                        code: "126",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3782836), longitude: Some(15.0461378), max_latitude: Some(46.390128), min_latitude: Some(46.3678584), max_longitude: Some(15.0592616), min_longitude: Some(15.0245747)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شوشتاني"), ("bn", "সস\u{9cd}ত\u{9be}ঞ\u{9cd}জ পৌরসভ\u{9be}"), ("ccp", "𑄥\u{1112e}𑄌\u{11134}𑄑𑄚\u{11134}𑄎\u{11128}"), ("ceb", "Občina Šoštanj"), ("cs", "Občina Šoštanj"), ("da", "Šoštanj"), ("de", "Šoštanj"), ("el", "Σοστάντζ"), ("en", "Šoštanj"), ("es", "Šoštanj"), ("fi", "Šoštanjin kunta"), ("fr", "Šoštanj"), ("gu", "સોસ\u{acd}તા\u{a82}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सोस\u{94d}ता\u{902}ज नगरपालिका"), ("hr", "Općina Šoštanj"), ("id", "Kotamadya Šoštanj"), ("it", "Šoštanj"), ("ja", "ショーシュタニ"), ("kn", "ಸಸ\u{ccd}ತಾನ\u{ccd}ಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "쇼슈탄"), ("lt", "Šoštanio savivaldybė"), ("lv", "Šoštanjas pašvaldība"), ("mr", "सॉस\u{94d}त\u{94d}ज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sostanj Municipality"), ("nb", "Sostanj kommune"), ("nl", "Šoštanj"), ("no", "Sostanj kommune"), ("pl", "Gmina Šoštanj"), ("pt", "Šoštanj"), ("ro", "Šoštanj"), ("ru", "Шоштань"), ("si", "සොස\u{dca}ට\u{dcf}න\u{dca}ජ\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Šoštanj"), ("sl", "Občina Šoštanj"), ("sr", "Општина Шоштањ"), ("sr_Latn", "Opština Šoštanj"), ("sv", "Sostanj kommun"), ("ta", "சோஸ\u{bcd}டஞ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4b}స\u{c4d}ట\u{c3e}ంజ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลโซซานจ\u{e4c}"), ("tr", "Sostanj Belediyesi"), ("uk", "Шоштань"), ("ur", "سوستانج میونسپلٹی"), ("vi", "Šoštanj"), ("zh", "紹什塔尼")]),
                        unofficial_name_list: ["Šoštanj"].to_vec(),
                    }
                ),
                (
                    "127",
                    Subdivision{
                        name: "127",
                        country_alpha2: Alpha2::SI,
                        code: "127",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2205619), longitude: Some(15.3154297), max_latitude: Some(46.2332033), min_latitude: Some(46.2155242), max_longitude: Some(15.332858), min_longitude: Some(15.3019398)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شتوره"), ("bn", "স\u{9cd}টোর পৌরসভ\u{9be}"), ("ca", "Štore"), ("ccp", "𑄌\u{11133}𑄑\u{11127}𑄢\u{11134}"), ("ceb", "Občina Štore"), ("cs", "Občina Štore"), ("da", "Štore Municipality"), ("de", "Štore"), ("el", "Στόρε"), ("en", "Štore"), ("es", "Štore"), ("fi", "Štoren kunta"), ("fr", "Štore"), ("gu", "સ\u{acd}ટોર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}टोर नगरपालिका"), ("hr", "Općina Štore"), ("id", "Kotamadya Štore"), ("it", "Štore"), ("ja", "シュトレ (スロベニア)"), ("kn", "ಸ\u{ccd}ಟೋರ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "슈토레"), ("lt", "Štorės savivaldybė"), ("lv", "Štores pašvaldība"), ("mr", "स\u{94d}टोअर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Store Municipality"), ("nb", "Store kommune"), ("nl", "Štore"), ("no", "Store kommune"), ("pl", "Gmina Štore"), ("pt", "Štore"), ("ro", "Štore"), ("ru", "Шторе"), ("si", "ස\u{dca}ටෝර\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Štore"), ("sr", "Општина Шторе"), ("sr_Latn", "Opština Štore"), ("sv", "Store kommun"), ("ta", "ஸ\u{bcd}டோர\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}ట\u{c4b}ర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องสตอร\u{e4c}"), ("tr", "Store Belediyesi"), ("uk", "Шторе (община)"), ("ur", "اسٹور میونسپلٹی"), ("vi", "Štore"), ("zh", "什托雷")]),
                        unofficial_name_list: ["Štore"].to_vec(),
                    }
                ),
                (
                    "128",
                    Subdivision{
                        name: "128",
                        country_alpha2: Alpha2::SI,
                        code: "128",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1857188), longitude: Some(13.7319838), max_latitude: Some(46.19265799999999), min_latitude: Some(46.1730367), max_longitude: Some(13.7411589), min_longitude: Some(13.7133538)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تولمين"), ("bn", "টোল\u{9cd}মিন পৌরসভ\u{9be}"), ("ca", "Tolmin"), ("ccp", "𑄑\u{11127}𑄣\u{11134}𑄟\u{11128}𑄚\u{11134}"), ("ceb", "Občina Tolmin"), ("cs", "Občina Tolmin"), ("da", "Tolmin Municipality"), ("de", "Tolmin"), ("el", "Τόλμιν"), ("en", "Tolmin"), ("es", "Tolmin"), ("fi", "Tolminin kunta"), ("fr", "Tolmin"), ("gu", "ટોલ\u{acd}મીન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "टोल\u{94d}मिन नगरपालिका"), ("hr", "Općina Tolmin"), ("hu", "Tolmin"), ("id", "Kotamadya Tolmin"), ("it", "Tolmino"), ("ja", "トールミン"), ("kn", "ಟಾಲ\u{ccd}ಮ\u{cbf}ನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "톨민"), ("lt", "Tolminas"), ("lv", "Tolminas pašvaldība"), ("mr", "टॉलमीन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Tolmin Municipality"), ("nb", "Tolmin kommune"), ("nl", "Tolmin"), ("no", "Tolmin kommune"), ("pl", "Gmina Tolmin"), ("pt", "Tolmin"), ("ro", "Tolmin"), ("ru", "Толмин"), ("si", "ටෝල\u{dca}ම\u{dd2}න\u{dca} නගරසභ\u{dcf}ව"), ("sk", "Tolmin"), ("sl", "Občina Tolmin"), ("sr", "Толмин"), ("sr_Latn", "Tolmin"), ("sv", "Tolmin"), ("ta", "தோல\u{bcd}ம\u{bc0}ன\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c4b}ల\u{c4d}మ\u{c3f}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องโทลม\u{e34}น"), ("tr", "Tolmin Belediyesi"), ("uk", "Толмин"), ("ur", "تولمین میونسپلٹی"), ("vi", "Đô thị tự trị Tolmin"), ("zh", "托爾明")]),
                        unofficial_name_list: ["Tolmin"].to_vec(),
                    }
                ),
                (
                    "129",
                    Subdivision{
                        name: "129",
                        country_alpha2: Alpha2::SI,
                        code: "129",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1503563), longitude: Some(15.0453138), max_latitude: Some(46.1716084), min_latitude: Some(46.1139747), max_longitude: Some(15.0648983), min_longitude: Some(15.0132381)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Trbovlje"), ("ccp", "𑄝\u{1112e}𑄛\u{11134}𑄎𑄬"), ("ceb", "Trbovlje"), ("cs", "Občina Trbovlje"), ("da", "Trbovlje"), ("de", "Trbovlje"), ("en", "Trbovlje"), ("es", "Trbovlje"), ("fr", "Trbovlje"), ("he", "טרבוולז׳"), ("hr", "Općina Trbovlje"), ("hu", "Trbovlje"), ("id", "Trbovlje"), ("it", "Trbovlje"), ("ja", "トルボヴリェ"), ("ka", "ტრბოვლიე"), ("ko", "트르보블레"), ("lt", "Trbovlės"), ("mk", "Трбовље"), ("mn", "Трбовле"), ("nb", "Trbovlje"), ("nl", "Trbovlje"), ("no", "Trbovlje"), ("pl", "Gmina Trbovlje"), ("pt", "Trbovlje"), ("ro", "Trbovlje"), ("ru", "Трбовле"), ("sk", "Trbovlje"), ("sl", "Občina Trbovlje"), ("sr", "Општина Трбовље"), ("sr_Latn", "Opština Trbovlje"), ("sv", "Trbovlje"), ("uk", "Трбовлє"), ("zh", "特尔伯夫列")]),
                        unofficial_name_list: ["Trbovlje"].to_vec(),
                    }
                ),
                (
                    "130",
                    Subdivision{
                        name: "130",
                        country_alpha2: Alpha2::SI,
                        code: "130",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9081708), longitude: Some(15.0125985), max_latitude: Some(45.9228233), min_latitude: Some(45.8970172), max_longitude: Some(15.0344809), min_longitude: Some(14.9921395)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تريبنيه"), ("bn", "ত\u{9cd}রেবনিজ পৌরসভ\u{9be}"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄛\u{11134}𑄎𑄬"), ("ceb", "Trebnje"), ("cs", "Občina Trebnje"), ("da", "Trebnje Municipality"), ("de", "Trebnje"), ("el", "Τρέμπντζε"), ("en", "Trebnje"), ("es", "Municipalidad del Trebnje"), ("fi", "Trebnjen kunta"), ("fr", "Municipalité de Trebnje"), ("gu", "ટ\u{acd}ર\u{ac7}બન\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ट\u{94d}र\u{947}ब\u{947}\u{902}ज नगरपालिका"), ("hr", "Općina Trebnje"), ("id", "Kotamadya Trebnje"), ("it", "Comune di Trebnje"), ("ja", "トレビニェ"), ("kn", "ಟ\u{ccd}ರ\u{cc6}ಬ\u{ccd}ನ\u{cc6}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "트레브네 지방 자치제"), ("lt", "Trebnės savivaldybė"), ("lv", "Trebnjes pašvaldība"), ("mr", "ट\u{94d}र\u{947}\u{902}ब\u{902}ज\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Trebnje Municipality"), ("nb", "Trebnje kommune"), ("nl", "Trebnje"), ("no", "Trebnje kommune"), ("pl", "Gmina Trebnje"), ("pt", "Trebnje kommune"), ("ro", "Comuna Trebnje"), ("ru", "Требне"), ("si", "ට\u{dca}\u{200d}රේබ\u{dca}න\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Trebnje"), ("sr", "Општина Требње"), ("sr_Latn", "Opština Trebnje"), ("sv", "Trebnje kommun"), ("ta", "ட\u{bcd}ரெபிஞ\u{bcd}சே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c4d}ర\u{c46}బంజ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเตรบเจ"), ("tr", "Trenje Belediyesi"), ("uk", "Требнє"), ("ur", "تریبنجی میونسپلٹی"), ("vi", "Đô thị tự trị Trebnje"), ("zh", "特雷布涅鎮")]),
                        unofficial_name_list: ["Trebnje"].to_vec(),
                    }
                ),
                (
                    "131",
                    Subdivision{
                        name: "131",
                        country_alpha2: Alpha2::SI,
                        code: "131",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.36215170000001), longitude: Some(14.3083372), max_latitude: Some(46.382076), min_latitude: Some(46.3507707), max_longitude: Some(14.3356295), min_longitude: Some(14.2912574)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تريجيك"), ("bn", "ট\u{9cd}রজিক পৌরসভ\u{9be}"), ("ccp", "𑄎\u{11128}𑄇\u{11134}"), ("ceb", "Občina Tržič"), ("cs", "Občina Tržič"), ("da", "Tržič"), ("de", "Tržič"), ("el", "Τρζικ"), ("en", "Tržič"), ("es", "Tržič"), ("fa", "ترژیچ"), ("fi", "Tržičin kunta"), ("fr", "Tržič"), ("gu", "ટ\u{acd}રીઝિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "तर\u{94d}ज\u{93c}िक महानगरपालिका"), ("hr", "Općina Tržič"), ("id", "Kotamadya Tržič"), ("it", "Tržič"), ("ja", "トルジッチ"), ("kn", "ಟ\u{ccd}ರಾಝ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "트르지치"), ("lt", "Trižčės savivaldybė"), ("lv", "Tržičas pašvaldība"), ("mr", "ट\u{94d}र\u{901}झिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Perbandaran Trzic"), ("nb", "Trzic kommune"), ("nl", "Tržič"), ("no", "Trzic kommune"), ("pl", "Gmina Tržič"), ("pt", "Tržič"), ("ro", "Tržič"), ("ru", "Тржич"), ("si", "ට\u{dca}ර\u{dca}ස\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Tržič"), ("sl", "Občina Tržič"), ("sr", "Општина Тржич"), ("sr_Latn", "Opština Tržič"), ("sv", "Trzic kommun"), ("ta", "டர\u{bcd}ஜிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c4d}ర\u{c3f}జ\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลตระซ\u{e34}ซ"), ("tr", "Trizic Belediyesi"), ("uk", "Тржич"), ("ur", "ترزیک میونسپلٹی"), ("vi", "Tržič"), ("zh", "特爾日奇")]),
                        unofficial_name_list: ["Tržic"].to_vec(),
                    }
                ),
                (
                    "132",
                    Subdivision{
                        name: "132",
                        country_alpha2: Alpha2::SI,
                        code: "132",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.625456), longitude: Some(16.3144961), max_latitude: Some(46.6432698), min_latitude: Some(46.6098349), max_longitude: Some(16.3358325), min_longitude: Some(16.2645197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄑𑄢\u{11134}𑄚\u{1112d}𑄌\u{11134}"), ("ceb", "Občina Turnišče"), ("cs", "Občina Turnišče"), ("de", "Turnišče"), ("en", "Turnišče"), ("fr", "Turnišče"), ("hr", "Općina Turnišče"), ("hu", "Bántornya község"), ("it", "Turnišče"), ("ja", "トゥルニシチェ"), ("ko", "투르니슈체"), ("nl", "Turnišče"), ("pl", "Gmina Turnišče"), ("pt", "Turnišče"), ("ro", "Turnišče"), ("sl", "Občina Turnišče"), ("sr", "Општина Турнишче"), ("sr_Latn", "Opština Turnišče"), ("uk", "Турнище"), ("vi", "Turnišče"), ("zh", "圖爾尼什切")]),
                        unofficial_name_list: ["Turnišce"].to_vec(),
                    }
                ),
                (
                    "133",
                    Subdivision{
                        name: "133",
                        country_alpha2: Alpha2::SI,
                        code: "133",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3622743), longitude: Some(15.1106582), max_latitude: Some(46.3809067), min_latitude: Some(46.346456), max_longitude: Some(15.1443209), min_longitude: Some(15.0634323)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄞𑄬𑄣𑄬𑄚\u{11134}𑄎𑄬"), ("ceb", "Velenje"), ("cs", "Městská občina Velenje"), ("en", "Velenje"), ("hr", "Gradska općina Velenje"), ("it", "Velenje"), ("nl", "Velenje"), ("pl", "Gmina miejska Velenje"), ("ro", "Comuna urbană Velenje"), ("ru", "Веленье"), ("sl", "Mestna občina Velenje"), ("sr", "Општина Велење"), ("sr_Latn", "Opština Velenje"), ("uk", "Веленє"), ("zh", "韋萊涅市")]),
                        unofficial_name_list: ["Velenje"].to_vec(),
                    }
                ),
                (
                    "134",
                    Subdivision{
                        name: "134",
                        country_alpha2: Alpha2::SI,
                        code: "134",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8336591), longitude: Some(14.6362363), max_latitude: Some(45.850463), min_latitude: Some(45.8210074), max_longitude: Some(14.6703333), min_longitude: Some(14.6151067)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فيليك لاستش"), ("bn", "ভেলিক\u{9be} ল\u{9be}সে পৌরসভ\u{9be}"), ("ccp", "𑄞𑄬𑄣\u{1112d}𑄇\u{11134} 𑄣𑄌\u{11134}"), ("ceb", "Občina Velike Lašče"), ("cs", "Občina Velike Lašče"), ("da", "Velike Lašče Municipality"), ("de", "Gemeinde Velike Lašče"), ("el", "Βελίκε Λάσκε"), ("en", "Velike Lašče"), ("es", "Velike Lašče"), ("fi", "Velike Laščen kunta"), ("fr", "Velike Lašče"), ("gu", "વ\u{ac7}લિક લાસ\u{acd}ક\u{ac7} , મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{947}लिक\u{947} लास\u{947} नगरपालिका"), ("id", "Kotamadya Velike Lašče"), ("it", "Velike Lašče"), ("ja", "ヴェリケ・ラシチェ"), ("kn", "ವೇಲ\u{cbf}ಕ\u{cc6} ಲಾಸ\u{ccd}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "벨리케라슈체 지방 자치제"), ("lt", "Velike Laščės savivaldybė"), ("lv", "Velike Laščes pašvaldība"), ("mr", "व\u{947}लिक\u{947} लासच\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Velike Lasce Municipality"), ("nb", "Velike Lasce kommune"), ("nl", "Velike Lašče"), ("no", "Velike Lasce kommune"), ("pl", "Gmina Velike Lašče"), ("pt", "Velike Lasce Kommune"), ("ro", "Comuna Velike Lašče"), ("ru", "Велике-Лашче"), ("si", "වෙල\u{dd2}කේ ලස\u{dca}කේ නගර සභ\u{dcf}ව"), ("sl", "Občina Velike Lašče"), ("sr", "Општина Велике Лашче"), ("sr_Latn", "Opština Velike Lašče"), ("sv", "Velike Lasce kommun"), ("ta", "விலைக\u{bcd} ல\u{bbe}சஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c46}ల\u{c3f}క\u{c46} ల\u{c3e}స\u{c4d}క\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเวล\u{e34}เก\u{e49} ลาสเซ\u{e48}"), ("tr", "Velike Lasce Belediyesi"), ("uk", "Велике Лаще (община)"), ("ur", "ویلیکی لاسکی میونسپلٹی"), ("vi", "Đô thị tự trị Velike Lasce"), ("zh", "韋利克拉什切")]),
                        unofficial_name_list: ["Velike Lašce"].to_vec(),
                    }
                ),
                (
                    "135",
                    Subdivision{
                        name: "135",
                        country_alpha2: Alpha2::SI,
                        code: "135",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8490426), longitude: Some(14.6947037), max_latitude: Some(45.856063), min_latitude: Some(45.8407962), max_longitude: Some(14.7065854), min_longitude: Some(14.6668756)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Videm"), ("ccp", "𑄞\u{1112d}𑄓𑄬𑄟\u{11134}"), ("ceb", "Videm"), ("cs", "Občina Videm"), ("de", "Videm"), ("en", "Videm"), ("fr", "Videm"), ("hr", "Općina Videm"), ("it", "Videm"), ("ja", "ヴィデム"), ("ko", "비뎀"), ("nl", "Videm"), ("pl", "Gmina Videm"), ("pt", "Videm"), ("ro", "Videm"), ("sl", "Občina Videm"), ("sr", "Општина Видем"), ("sr_Latn", "Opština Videm"), ("uk", "Видем"), ("vi", "Videm"), ("zh", "維代姆鎮")]),
                        unofficial_name_list: ["Videm"].to_vec(),
                    }
                ),
                (
                    "136",
                    Subdivision{
                        name: "136",
                        country_alpha2: Alpha2::SI,
                        code: "136",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8455744), longitude: Some(13.9625431), max_latitude: Some(45.8606021), min_latitude: Some(45.8185959), max_longitude: Some(13.9871222), min_longitude: Some(13.9441306)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فيبافا"), ("bn", "ভিপ\u{9be}ভ\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄞\u{11128}𑄛𑄞"), ("ceb", "Vipava"), ("cs", "Občina Vipava"), ("da", "Vipava Municipality"), ("de", "Gemeinde Vipava"), ("el", "Βιπάβα"), ("en", "Vipava"), ("es", "Municipalidad del Vipava"), ("fi", "Vipavan kunta"), ("fr", "Municipalité de Vipava"), ("gu", "વિપાવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "विपावा नगर पालिका"), ("hr", "Općina Vipava"), ("id", "Kotamadya Vipava"), ("it", "Comune di Vipava"), ("ja", "ヴィパーヴァ"), ("kn", "ವ\u{cbf}ಪವ ಪುರಸಭ\u{cc6}"), ("ko", "비파바 지방 자치제"), ("lt", "Vipavos savivaldybė"), ("lv", "Vipavas pašvaldība"), ("mr", "विपपो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vipava Municipality"), ("nb", "Vipava kommune"), ("nl", "Vipava"), ("no", "Vipava kommune"), ("pl", "Gmina Vipava"), ("pt", "Município de Vipava"), ("ro", "Comuna Vipava"), ("ru", "Випава"), ("si", "ව\u{dd2}පව\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Vipava"), ("sr", "Општина Випава"), ("sr_Latn", "Opština Vipava"), ("sv", "Vipava kommun"), ("ta", "விபவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3f}ప\u{c3e}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องว\u{e35}ปาวา"), ("tr", "Vipava Belediyesi"), ("uk", "Віпава"), ("ur", "ویپاوا میونسپلٹی"), ("vi", "Đô thị tự trị Vipava"), ("zh", "維帕瓦鎮")]),
                        unofficial_name_list: ["Vipava"].to_vec(),
                    }
                ),
                (
                    "137",
                    Subdivision{
                        name: "137",
                        country_alpha2: Alpha2::SI,
                        code: "137",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3815074), longitude: Some(15.2950333), max_latitude: Some(46.3899328), min_latitude: Some(46.369059), max_longitude: Some(15.3071957), min_longitude: Some(15.2761003)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فيتانيه"), ("bn", "ভিট\u{9be}ন"), ("bs", "Vitanje"), ("ccp", "𑄞\u{11128}𑄖𑄚\u{11134}𑄎𑄬"), ("ceb", "Vitanje"), ("cs", "Občina Vitanje"), ("cy", "Vitanje"), ("da", "Vitanje"), ("de", "Vitanje"), ("el", "Βιτάντζε"), ("en", "Vitanje"), ("es", "Vitanje"), ("fi", "Vitanje"), ("fr", "Vitanje"), ("gu", "વિટ\u{ac7}ન\u{acd}જ\u{ac7}"), ("hi", "वितान\u{94d}य\u{947}"), ("hr", "Općina Vitanje"), ("id", "Vitanje"), ("it", "Vitanje"), ("ja", "ヴィタニェ"), ("kn", "ವ\u{cbf}ಟಾನ\u{ccd}ಜ\u{cc6}"), ("ko", "비타네"), ("lt", "Vitanė"), ("lv", "Vitanje"), ("mr", "विट\u{901}ज\u{947}"), ("ms", "Vitanje"), ("nb", "Vitanje"), ("nl", "Vitanje"), ("no", "Vitanje"), ("pl", "Gmina Vitanje"), ("pt", "Vitanje"), ("ro", "Comuna Vitanje"), ("ru", "Витанье"), ("si", "ව\u{dd2}ටන\u{dca}ජේ"), ("sl", "Občina Vitanje"), ("sr", "Витање"), ("sr_Latn", "Vitanje"), ("sv", "Vitanje"), ("ta", "விதஞ\u{bcd}செ"), ("te", "వ\u{c3f}ట\u{c3e}ంజ\u{c46}"), ("th", "ว\u{e34}ทานเจ"), ("tr", "Vitanje"), ("uk", "Витанє (община)"), ("ur", "ویتانجی"), ("vi", "Vitanje"), ("zh", "維塔涅")]),
                        unofficial_name_list: ["Vitanje"].to_vec(),
                    }
                ),
                (
                    "138",
                    Subdivision{
                        name: "138",
                        country_alpha2: Alpha2::SI,
                        code: "138",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1896643), longitude: Some(14.493854), max_latitude: Some(46.2058137), min_latitude: Some(46.1795832), max_longitude: Some(14.5083227), min_longitude: Some(14.459533)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فودايس"), ("bg", "Водице"), ("bn", "ভোডিস"), ("bs", "Vodice"), ("ccp", "𑄞\u{1112e}𑄓\u{11128}𑄌\u{11134}"), ("ceb", "Vodice"), ("cs", "Občina Vodice"), ("da", "Vodice"), ("de", "Vodice"), ("el", "Βόντικε"), ("en", "Vodice"), ("es", "Vodice"), ("fi", "Vodice"), ("fr", "Vodice"), ("gu", "વોડિસ"), ("hi", "वोदिस"), ("hr", "Općina Vodice"), ("id", "Vodice"), ("it", "Vodice"), ("ja", "ヴォディツェ"), ("kn", "ವೊಡ\u{cbf}ಸ\u{ccd}"), ("ko", "보디체"), ("lt", "Vodicė"), ("lv", "Vodice"), ("mr", "वोडिस"), ("ms", "Vodice"), ("nb", "Vodice"), ("nl", "Vodice"), ("no", "Vodice"), ("pl", "Gmina Vodice"), ("pt", "Vodice (Eslovênia)"), ("ro", "Vodice"), ("ru", "Водице"), ("si", "වොඩ\u{dd2}ස\u{dca}"), ("sl", "Vodice"), ("sr", "Водице (Словенија)"), ("sr_Latn", "Vodice (Slovenija)"), ("sv", "Vodice"), ("ta", "வோடிஸ\u{bcd}"), ("te", "వ\u{c4b}డ\u{c3f}స\u{c4d}"), ("th", "วอร\u{e4c}ไดซ\u{e4c}"), ("tr", "Vodice"), ("uk", "Водице"), ("ur", "وودیکی"), ("vi", "Vodice"), ("zh", "沃迪采")]),
                        unofficial_name_list: ["Vodice"].to_vec(),
                    }
                ),
                (
                    "139",
                    Subdivision{
                        name: "139",
                        country_alpha2: Alpha2::SI,
                        code: "139",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2920581), longitude: Some(15.302058), max_latitude: Some(46.3003366), min_latitude: Some(46.2726675), max_longitude: Some(15.3221094), min_longitude: Some(15.284065)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فوينيك"), ("bn", "ভোজনিক পৌরসভ\u{9be}"), ("ccp", "𑄞\u{1112e}𑄌\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Vojnik"), ("cs", "Občina Vojnik"), ("da", "Vojnik Municipality"), ("de", "Gemeinde Vojnik"), ("el", "Βόζνικ"), ("en", "Vojnik"), ("es", "Municipalidad del Vojnik"), ("fi", "Vojnik"), ("fr", "Municipalité deVojnik"), ("gu", "વોજ\u{acd}નિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "वोयनिक नगरपालिका"), ("hr", "Općina Vojnik"), ("id", "Kotamadya Vojnik"), ("it", "Vojnik"), ("ja", "ヴォイニク"), ("kn", "ವೊನ\u{cbf}ಕ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "보이니크 지방 자치제"), ("lt", "Voiniko savivaldybė"), ("lv", "Vojnikas pašvaldība"), ("mr", "वोजनिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vojnik Municipality"), ("nb", "Vojnik kommune"), ("nl", "Vojnik"), ("no", "Vojnik kommune"), ("pl", "Gmina Vojnik"), ("pt", "Vojnik kommune"), ("ro", "Comuna Vojnik"), ("ru", "Войник"), ("si", "වොජ\u{dca}න\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Vojnik"), ("sr", "Општина Војник"), ("sr_Latn", "Opština Vojnik"), ("sv", "Vojnik kommun"), ("ta", "வோஜனிக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c4b}జ\u{c4d}న\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องวอยน\u{e34}ค"), ("tr", "Vojnik Belediyesi"), ("uk", "Войник (община)"), ("ur", "ووجنیک میونسپلٹی"), ("vi", "Đô thị tự trị Vojnik"), ("zh", "沃伊尼克")]),
                        unofficial_name_list: ["Vojnik"].to_vec(),
                    }
                ),
                (
                    "140",
                    Subdivision{
                        name: "140",
                        country_alpha2: Alpha2::SI,
                        code: "140",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.966583), longitude: Some(14.2973873), max_latitude: Some(45.9783576), min_latitude: Some(45.9045486), max_longitude: Some(14.3161392), min_longitude: Some(14.2396743)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فرنيكا"), ("be", "Врхніка"), ("bn", "ভ\u{9cd}রহ\u{9cd}নিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄚\u{11128}𑄇"), ("ceb", "Vrhnika"), ("cs", "Občina Vrhnika"), ("da", "Vrhnika Municipality"), ("de", "Gemeinde Vrhnika"), ("el", "Βρχνίκα"), ("en", "Vrhnika"), ("es", "Municipalidad del Vrhnika"), ("fi", "Vrhnikan kunta"), ("fr", "Municipalité de Vrhnika"), ("gu", "વ\u{acd}ર\u{acd}હ\u{acd}નીકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{94d}रहनिका नगर पालिका"), ("hr", "Općina Vrhnika"), ("id", "Kotamadya Vrhnika"), ("it", "Nauporto"), ("ja", "ブルフニカ"), ("kn", "ವ\u{ccd}ರನ\u{cbf}ಕ ಪುರಸಭ\u{cc6}"), ("ko", "브르흐니카 지방 자치제"), ("lt", "Vrchnikos savivaldybė"), ("lv", "Vrhnikas pašvaldība"), ("mk", "Врхника"), ("mr", "वर\u{94d}थनिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vrhnika Municipality"), ("nb", "Vrhnika kommune"), ("nl", "Vrhnika"), ("no", "Vrhnika kommune"), ("pl", "Gmina Vrhnika"), ("pt", "Município de Vrhnika"), ("ro", "Comuna Vrhnika"), ("ru", "Врхника"), ("si", "ව\u{dca}ර\u{dca}හ\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Vrhnika"), ("sr", "Општина Врхника"), ("sr_Latn", "Opština Vrhnika"), ("sv", "Vrhnika kommun"), ("ta", "வர\u{bcd}ஹனிக\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c3f}ర\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เวอห\u{e4c}น\u{e34}กา"), ("tr", "Vrhnika Belediyesi"), ("uk", "Врхника"), ("ur", "ورحنیکا میونسپلٹی"), ("vi", "Vrhnika Đô thị tự trị"), ("zh", "弗爾赫尼卡鎮")]),
                        unofficial_name_list: ["Vrhnika"].to_vec(),
                    }
                ),
                (
                    "141",
                    Subdivision{
                        name: "141",
                        country_alpha2: Alpha2::SI,
                        code: "141",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5980836), longitude: Some(15.1657237), max_latitude: Some(46.6124685), min_latitude: Some(46.5769963), max_longitude: Some(15.1829375), min_longitude: Some(15.1412488)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فوزينيكا"), ("bn", "ভ\u{9c1}জেনিক\u{9be} পৌরসভ\u{9be}"), ("ca", "Vuzenica"), ("ccp", "𑄞\u{1112e}𑄎𑄬𑄚\u{11128}𑄇"), ("ceb", "Vuzenica (munisipyo)"), ("cs", "Občina Vuzenica"), ("da", "Vuzenica Municipality"), ("de", "Vuzenica"), ("el", "Βουζενίκα"), ("en", "Vuzenica"), ("es", "Vuzenica"), ("fi", "Vuzenican kunta"), ("fr", "Vuzenica"), ("gu", "વ\u{ac1}ઝ\u{ac7}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{941}ज\u{93c}\u{947}निका नगर पालिका"), ("hr", "Općina Vuzenica"), ("id", "Kotamadya Vuzenica"), ("it", "Vuzenica"), ("ja", "ヴゼニツァ"), ("kn", "ವ\u{ccd}ಯುಜ\u{cbf}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "부제니차"), ("lt", "Vuzenica"), ("lv", "Vuzenicas pašvaldība"), ("mr", "ऊझ\u{947}नचिया म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Vuzenica Municipality"), ("nb", "Vuzenica Kommune"), ("nl", "Vuzenica"), ("no", "Vuzenica Kommune"), ("pl", "Gmina Vuzenica"), ("pt", "Vuzenica"), ("ro", "Vuzenica"), ("ru", "Вузеница"), ("si", "ව\u{dd4}සෙන\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Vuzenica"), ("sr", "Општина Вузеница"), ("sr_Latn", "Opština Vuzenica"), ("sv", "Vuzenica Kommun"), ("ta", "வுஸினிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వుజ\u{c46}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องว\u{e39}เซน\u{e34}ก\u{e49}า"), ("tr", "Vuzenica Belediyesi"), ("uk", "Вузениця"), ("ur", "ووزینیکا میونسپلٹی"), ("vi", "Vuzenica"), ("zh", "弗贊尼卡")]),
                        unofficial_name_list: ["Vuzenica"].to_vec(),
                    }
                ),
                (
                    "142",
                    Subdivision{
                        name: "142",
                        country_alpha2: Alpha2::SI,
                        code: "142",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1345186), longitude: Some(14.9945975), max_latitude: Some(46.1428459), min_latitude: Some(46.1116695), max_longitude: Some(15.0149749), min_longitude: Some(14.9775918)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎𑄉\u{1112e}𑄢\u{11134}𑄎𑄬 𑄃\u{11127}𑄛\u{11134} 𑄥𑄞\u{11128}"), ("ceb", "Zagorje ob Savi"), ("cs", "Občina Zagorje ob Savi"), ("en", "Zagorje ob Savi"), ("hr", "Općina Zagorje ob Savi"), ("hu", "Zagorje ob Savi község"), ("pl", "Gmina Zagorje ob Savi"), ("ro", "Comuna Zagorje ob Savi"), ("sl", "Občina Zagorje ob Savi"), ("sr", "Општина Загорје об Сави"), ("sr_Latn", "Opština Zagorje ob Savi"), ("uk", "Загорє-об-Саві (община)"), ("zh", "薩瓦河畔扎戈列鎮")]),
                        unofficial_name_list: ["Zagorje ob Savi"].to_vec(),
                    }
                ),
                (
                    "143",
                    Subdivision{
                        name: "143",
                        country_alpha2: Alpha2::SI,
                        code: "143",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3856393), longitude: Some(16.0470768), max_latitude: Some(46.392795), min_latitude: Some(46.38135219999999), max_longitude: Some(16.0559568), min_longitude: Some(16.0254542)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية زافرتش"), ("bn", "জ\u{9be}ভ\u{9cd}রচ পৌরসভ\u{9be}"), ("ccp", "𑄎𑄛\u{11134}𑄢\u{11134}"), ("ceb", "Občina Zavrč"), ("cs", "Občina Zavrč"), ("da", "Zavrč Municipality"), ("de", "Zavrč"), ("el", "Ζαβρκ"), ("en", "Zavrč"), ("es", "Zavrč"), ("fi", "Zavrčn kunta"), ("fr", "Zavrč"), ("gu", "ઝ\u{ac7}વ\u{acd}રક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}ावर\u{94d}क नगरपालिका"), ("hr", "Općina Zavrč"), ("id", "Kotamadya Zavrč"), ("it", "Zavrč"), ("ja", "ザヴルチ"), ("kn", "ಜವಾರ\u{ccd}ಕ\u{ccd} ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "자브르치"), ("lt", "Zavrčo savivaldybė"), ("lv", "Zavrčas pašvaldība"), ("mr", "झ\u{945}व\u{94d}हर\u{94d}स म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zavrc Municipality"), ("nb", "Zavrc kommune"), ("nl", "Zavrč"), ("no", "Zavrc kommune"), ("pl", "Gmina Zavrč"), ("pt", "Zavrč"), ("ro", "Zavrč"), ("ru", "Заврч"), ("si", "සව\u{dca}ර\u{dca}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Zavrč"), ("sr", "Општина Заврч"), ("sr_Latn", "Opština Zavrč"), ("sv", "Zavrc kommun"), ("ta", "ச\u{bbe}வர\u{bcd}க\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c3e}ర\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องซาวค\u{e4c}"), ("tr", "Zavrc Belediyesi"), ("uk", "Заврч"), ("ur", "زاورک میونسپلٹی"), ("vi", "Zavrč"), ("zh", "薩弗爾奇")]),
                        unofficial_name_list: ["Zavrc"].to_vec(),
                    }
                ),
                (
                    "144",
                    Subdivision{
                        name: "144",
                        country_alpha2: Alpha2::SI,
                        code: "144",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3696591), longitude: Some(15.3918526), max_latitude: Some(46.3891589), min_latitude: Some(46.3623322), max_longitude: Some(15.4118749), min_longitude: Some(15.3599061)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زريس"), ("bn", "জেরিয\u{9bc}ে পৌরসভ\u{9be}"), ("ccp", "𑄎\u{11133}𑄢𑄬𑄌\u{11134}"), ("ceb", "Občina Zreče"), ("cs", "Občina Zreče"), ("da", "Zreče"), ("de", "Zreče"), ("el", "Ζρέτσε"), ("en", "Zreče"), ("es", "Municipalidad del Zreče"), ("fi", "Zrečen kunta"), ("fr", "Zreče"), ("gu", "ઝર\u{ac7}ક\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}डर\u{947}स नगरपालिका"), ("hr", "Općina Zreče"), ("id", "Kotamadya Zreče"), ("it", "Zreče"), ("ja", "ズレチェ"), ("kn", "ಝ\u{ccd}ರ\u{cc6}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "즈레체"), ("lt", "Zrečės savivaldybė"), ("lv", "Zrečes pašvaldība"), ("mr", "झर\u{947}स\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zrece Municipality"), ("nb", "Zrece kommune"), ("nl", "Zreče"), ("no", "Zrece kommune"), ("pl", "Gmina Zreče"), ("pt", "Zreče"), ("ro", "Zreče"), ("ru", "Жрече"), ("si", "ස\u{dca}රෙසේ නගර සභ\u{dcf}ව"), ("sl", "Občina Zreče"), ("sr", "Општина Зрече"), ("sr_Latn", "Opština Zreče"), ("sv", "Zrece kommun"), ("ta", "ஸ\u{bcd}ர\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c3f}యర\u{c40}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลซเรเซ"), ("tr", "Zrece Belediyesi"), ("uk", "Зрече (община)"), ("ur", "میونسپلٹی"), ("vi", "Zreče"), ("zh", "茲雷切")]),
                        unofficial_name_list: ["Zrece"].to_vec(),
                    }
                ),
                (
                    "146",
                    Subdivision{
                        name: "146",
                        country_alpha2: Alpha2::SI,
                        code: "146",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.22544), longitude: Some(14.1692478), max_latitude: Some(46.2399012), min_latitude: Some(46.19893709999999), max_longitude: Some(14.1769495), min_longitude: Some(14.0979763)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جيليزنيكي"), ("bn", "জেলেজনিকি পৌরসভ\u{9be}"), ("ccp", "𑄎𑄬𑄣𑄬𑄌\u{11134}𑄚\u{11128}𑄇\u{11128}"), ("ceb", "Občina Železniki"), ("cs", "Občina Železniki"), ("da", "Železniki Municipality"), ("de", "Železniki"), ("el", "Ζελεζνίκι"), ("en", "Železniki"), ("es", "Železniki"), ("fa", "ژلزنیکی"), ("fi", "Železnikin kunta"), ("fr", "Železniki"), ("gu", "ઝ\u{ac7}લ\u{ac7}ઝ\u{acd}નિકી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}ल\u{947}ज\u{93c}निकी नगर पालिका"), ("hr", "Općina Železniki"), ("id", "Kotamadya Železniki"), ("it", "Železniki"), ("ja", "ジェレズニキ"), ("ko", "젤레즈니키"), ("lt", "Železnikai"), ("lv", "Železniku pašvaldība"), ("mr", "झ\u{947}ल\u{947}झनिकी म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Municipality of Zelezniki"), ("nb", "Zelezniki Kommune"), ("nl", "Železniki"), ("no", "Zelezniki Kommune"), ("pl", "Gmina Železniki"), ("pt", "Železniki"), ("ro", "Železniki"), ("ru", "Железники"), ("si", "සෙලෙස\u{dca}න\u{dd2}ක\u{dd2} නගර සභ\u{dcf}ව"), ("sk", "Železniki"), ("sl", "Občina Železniki"), ("sr", "Општина Железники"), ("sr_Latn", "Opština Železniki"), ("sv", "Zelezniki (kommun)"), ("ta", "ஸியேஸ\u{bcd}னிக\u{bcd}கி நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}ల\u{c46}జ\u{c4d}న\u{c3f}క\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เซเลซน\u{e34}ก\u{e35}"), ("tr", "Zelezniki Belediyesi"), ("uk", "Железнікі"), ("ur", "زیلیزنیکی میونسپلٹی"), ("vi", "Železniki"), ("zh", "熱萊茲尼基")]),
                        unofficial_name_list: ["Železniki"].to_vec(),
                    }
                ),
                (
                    "147",
                    Subdivision{
                        name: "147",
                        country_alpha2: Alpha2::SI,
                        code: "147",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0513094), longitude: Some(14.111887), max_latitude: Some(46.0671155), min_latitude: Some(46.0305451), max_longitude: Some(14.1330883), min_longitude: Some(14.0769395)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جيري"), ("bg", "Жири"), ("ccp", "𑄎\u{11128}𑄢\u{11128}"), ("ceb", "Občina Žiri"), ("cs", "Občina Žiri"), ("de", "Žiri"), ("en", "Žiri"), ("fa", "ژیری"), ("fr", "Žiri"), ("hr", "Općina Žiri"), ("it", "Žiri"), ("ja", "ジリ"), ("ko", "지리 (도시)"), ("nl", "Žiri"), ("pl", "Gmina Žiri"), ("pt", "Žiri"), ("ro", "Žiri"), ("ru", "Жири"), ("sk", "Žiri"), ("sl", "Občina Žiri"), ("sr", "Општина Жири"), ("sr_Latn", "Opština Žiri"), ("th", "เทศบาลช\u{e35}ร\u{e35}"), ("uk", "Жири"), ("vi", "Žiri"), ("zh", "日里")]),
                        unofficial_name_list: ["Žiri"].to_vec(),
                    }
                ),
                (
                    "148",
                    Subdivision{
                        name: "148",
                        country_alpha2: Alpha2::SI,
                        code: "148",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6075732), longitude: Some(15.8896942), max_latitude: Some(46.62573510000001), min_latitude: Some(46.5930314), max_longitude: Some(15.9011585), min_longitude: Some(15.8639755)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Benedikt"), ("ar", "بلدية بينيديكت"), ("bn", "বেনেডিক\u{9cd}ট পৌরসভ\u{9be}"), ("bs", "Benedikt"), ("ca", "Benedikt"), ("ccp", "𑄝𑄬𑄚𑄬𑄓\u{11128}𑄇\u{11134}𑄑\u{11134}"), ("ceb", "Benedikt (munisipyo)"), ("cs", "Občina Benedikt"), ("cy", "Benedikt"), ("da", "Benedikt"), ("de", "Benedikt"), ("el", "Μπενεντίκτ"), ("en", "Benedikt"), ("es", "Benedikt"), ("et", "Benedikt"), ("eu", "Benedikt"), ("fi", "Benedikt"), ("fr", "Benedikt"), ("ga", "Benedikt"), ("gl", "Benedikt"), ("gu", "બ\u{ac7}ન\u{ac7}ડિક\u{acd}ટ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{947}न\u{947}डिक\u{94d}ट नगर पालिका"), ("hr", "Općina Benedikt"), ("hu", "Benedikt"), ("id", "Kotamadya Benedikt"), ("is", "Benedikt"), ("it", "Benedikt"), ("ja", "ベネディクト"), ("kn", "ಬ\u{cc6}ನ\u{cc6}ಡ\u{cbf}ಕ\u{ccd}ಟ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "베네딕트 지방 자치제"), ("lt", "Benedikt"), ("lv", "Benediktas pašvaldība"), ("mr", "ब\u{947}निडिक\u{94d}ट म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Benedikt Municipality"), ("nb", "Benedikt"), ("nl", "Benedikt"), ("no", "Benedikt"), ("pl", "Benedikt"), ("pt", "Benedikt"), ("ro", "Comuna Benedikt"), ("ru", "Бенедикт"), ("si", "බෙනේඩ\u{dd2}ක\u{dca}ට\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Benedikt"), ("sl", "Občina Benedikt"), ("sq", "Benedikt"), ("sr", "Општина Бенедикт"), ("sr_Latn", "Opština Benedikt"), ("sv", "Benedikt"), ("ta", "பெனடிக\u{bcd}ட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c46}న\u{c46}\u{c46}డ\u{c3f}క\u{c4d}ఠ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องรานโควเซ\u{e48}"), ("tr", "Benedikt"), ("uk", "Бенедикт"), ("ur", "بلدیہ بینیدیکت"), ("vi", "Đô thị tự trị Benedikt"), ("zh", "贝内迪克特")]),
                        unofficial_name_list: ["Benedikt"].to_vec(),
                    }
                ),
                (
                    "149",
                    Subdivision{
                        name: "149",
                        country_alpha2: Alpha2::SI,
                        code: "149",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0565579), longitude: Some(15.6625947), max_latitude: Some(46.0672027), min_latitude: Some(46.0455998), max_longitude: Some(15.684388), min_longitude: Some(15.6559366)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بيستريكا أوب سوتلي"), ("bn", "বিস\u{9cd}ত\u{9cd}রিক\u{9be} অব সোতলি পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{1112d}𑄇 𑄃\u{11127}𑄛\u{11134} 𑄥\u{1112e}𑄖\u{11134}𑄣\u{11128}"), ("ceb", "Bistrica ob Sotli"), ("cs", "Občina Bistrica ob Sotli"), ("da", "Bistrica ob Sotli Municipality"), ("de", "Gemeinde Bistrica ob Sotli"), ("el", "Μπιστρίκα ομπ Σότλι"), ("en", "Bistrica ob Sotli"), ("es", "Municipio de Bistrica ob Sotli"), ("fi", "Bistrica ob Sotlin kunta"), ("fr", "Municipalité de Bistrica ob Sotli"), ("gu", "બ\u{acd}રીસ\u{acd}ટિકા ઓબ સોતલી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "बिस\u{94d}ट\u{94d}रिका ओब सोटली नगर पालिका"), ("hr", "Općina Bistrica ob Sotli"), ("id", "Kotamadya Bistrica ob Sotli"), ("it", "Bistrica ob Sotli"), ("ja", "ビストリツァ・オプ・ソトリ"), ("kn", "ಬ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ ಆಬ\u{ccd} ಸೊಟ\u{ccd}ಲ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "비스트리차오브소틀리 지방 자치제"), ("lt", "Bistricos ir Sotlio savivaldybė"), ("lv", "Bistricas ob Sotli pašvaldība"), ("mr", "बिस\u{94d}ट\u{94d}रिका ओब सोटली म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bistrica ob Sotli Municipality"), ("nb", "Bistrica ob Sotli kommune"), ("nl", "Bistrica ob Sotli"), ("no", "Bistrica ob Sotli kommune"), ("pl", "Gmina Bistrica ob Sotli"), ("pt", "Município de Bistrica de Sotli"), ("ro", "Comuna Bistrica ob Sotli"), ("ru", "Бистрица-об-Сотли"), ("si", "බ\u{dd2}ස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ක\u{dcf} ඔබ\u{dca} සොට\u{dca}ල\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Bistrica ob Sotli"), ("sr", "Општина Бистрица об Сотли"), ("sr_Latn", "Opština Bistrica ob Sotli"), ("sv", "Bistrica ob Sotli kommun"), ("ta", "பிஸ\u{bcd}டரிக\u{bbe} ஒப\u{bcd} சொட\u{bcd}டளி நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c3f}స\u{c4d}ట\u{c4d}ర\u{c3f}క\u{c3e} ఓబ\u{c4d} స\u{c4b}ట\u{c4d}ల\u{c40} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "\u{e34}เม\u{e37}องบ\u{e34}สทร\u{e34}ซา ออป ซอทล\u{e34}"), ("tr", "Bistrica ob Sotli Belediyesi"), ("uk", "Бистриця-об-Сотлі (община)"), ("ur", "بلدیہ بیستریتسا اوپ سوتلی"), ("vi", "Đô thị tự trị Bistrica ob Sotli"), ("zh", "索特拉河畔比斯特里察")]),
                        unofficial_name_list: ["Bistrica ob Sotli"].to_vec(),
                    }
                ),
                (
                    "150",
                    Subdivision{
                        name: "150",
                        country_alpha2: Alpha2::SI,
                        code: "150",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.791555), longitude: Some(14.5067013), max_latitude: Some(45.8439663), min_latitude: Some(45.7301958), max_longitude: Some(14.5787575), min_longitude: Some(14.4371763)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بلوك"), ("bn", "ব\u{9cd}লোক পৌরসভ\u{9be}"), ("bs", "Bloke"), ("ccp", "𑄇\u{11133}𑄣\u{1112e}𑄇\u{11134}"), ("ceb", "Bloke (munisipyo sa Eslobenya)"), ("cs", "Občina Bloke"), ("da", "Bloke Municipality"), ("de", "Bloke"), ("el", "Μπλόουκ"), ("en", "Bloke"), ("es", "Municipalidad del Bloke"), ("fi", "Bloken kunta"), ("fr", "Bloke"), ("gu", "બ\u{acd}લોક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}लोक नगर पालिका"), ("hr", "Općina Bloke"), ("hu", "Bloke"), ("id", "Kotamadya Bloke"), ("it", "Bloke"), ("ja", "ブロケ"), ("kn", "ಬ\u{ccd}ಲಾಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "블로케"), ("lt", "Blokės savivaldybė"), ("lv", "Blokes pašvaldība"), ("mr", "ब\u{94d}लोक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Bloke Municipality"), ("nb", "Bloke kommune"), ("nl", "Bloke"), ("no", "Bloke kommune"), ("pl", "Gmina Bloke"), ("pt", "Bloke"), ("ro", "Bloke"), ("ru", "Блоке (Словения)"), ("si", "බ\u{dca}ලොකේ නගර සභ\u{dcf}ව"), ("sl", "Občina Bloke"), ("sr", "Општина Блоке"), ("sr_Latn", "Opština Bloke"), ("sv", "Bloke"), ("ta", "ப\u{bcd}ளோக\u{bcd}கே நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ల\u{c4b}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโบรค"), ("tr", "Bloke Belediyesi"), ("uk", "Блоке"), ("ur", "بلدیہ بلوکے"), ("vi", "Đô thị tự trị Bloke"), ("zh", "布洛凯")]),
                        unofficial_name_list: ["Bloke"].to_vec(),
                    }
                ),
                (
                    "151",
                    Subdivision{
                        name: "151",
                        country_alpha2: Alpha2::SI,
                        code: "151",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2880914), longitude: Some(15.0397466), max_latitude: Some(46.2958925), min_latitude: Some(46.2829951), max_longitude: Some(15.0487472), min_longitude: Some(15.0227904)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية براسلوفتشه"), ("bn", "ব\u{9cd}র\u{9be}স\u{9cd}লোভিচ পৌরসভ\u{9be}"), ("ccp", "𑄝\u{11133}𑄢𑄥\u{11133}𑄣\u{1112e}𑄛\u{11134}"), ("ceb", "Občina Braslovče"), ("cs", "Občina Braslovče"), ("da", "Braslovče Municipality"), ("de", "Gemeinde Braslovče"), ("el", "Μπράσλοβτσε"), ("en", "Braslovče"), ("es", "Braslovče"), ("fi", "Braslovčen kunta"), ("fr", "Municipalité de Braslovče"), ("gu", "બ\u{acd}રાસ\u{acd}લોવ\u{acd}ક\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ब\u{94d}रास\u{94d}लोविए नगर पालिका"), ("hr", "Braslovče"), ("id", "Kotamadya Braslovče"), ("it", "Braslovče"), ("ja", "ブラスロフチェ"), ("kn", "ಬ\u{ccd}ರಾಸ\u{ccd}ಲೋವ\u{ccd}ಸ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "블라슬로브체 지방 자치제"), ("lt", "Braslovčės savivaldybė"), ("lv", "Braslovčes pašvaldība"), ("mr", "ब\u{94d}रास\u{94d}लोव\u{94d}हक\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Braslovce Municipality"), ("nb", "Braslovce Kommune"), ("nl", "Braslovče"), ("no", "Braslovce Kommune"), ("pl", "Gmina Braslovče"), ("pt", "Município de Braslovce"), ("ro", "Comuna Braslovče"), ("ru", "Брасловче"), ("si", "බ\u{dca}\u{200d}රස\u{dca}ලෝව\u{dca}සේ නගර සභ\u{dcf}ව"), ("sl", "Braslovče"), ("sr", "Општина Брасловче"), ("sr_Latn", "Opština Braslovče"), ("sv", "Braslovce (kommun)"), ("ta", "ப\u{bcd}ரஸ\u{bcd}லோவ\u{bcd}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "బ\u{c4d}ర\u{c3e}స\u{c4d}ల\u{c4b}వ\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องบราสลอฟเซ"), ("tr", "Braslovce Belediyesi"), ("uk", "Брасловче (община)"), ("ur", "بلدیہ براسلووچے"), ("vi", "Đô thị tự trị Braslovce"), ("zh", "布拉斯洛夫采")]),
                        unofficial_name_list: ["Braslovce"].to_vec(),
                    }
                ),
                (
                    "152",
                    Subdivision{
                        name: "152",
                        country_alpha2: Alpha2::SI,
                        code: "152",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.71823699999999), longitude: Some(16.0197222), max_latitude: Some(46.7318479), min_latitude: Some(46.7108072), max_longitude: Some(16.0392574), min_longitude: Some(16.0054689)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كانكوفا"), ("bn", "ক\u{9cd}য\u{9be}ঙ\u{9cd}কভ\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄇𑄚\u{11134}𑄇\u{1112e}𑄞"), ("ceb", "Cankova (munisipyo)"), ("cs", "Občina Cankova"), ("da", "Cankova Municipality"), ("de", "Gemeinde Cankova"), ("el", "Κάνκοβα"), ("en", "Cankova"), ("es", "Municipalidad del Cankova"), ("fi", "Cankovan kunta"), ("fr", "Cankova"), ("gu", "ક\u{ac7}ન\u{acd}કોવા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{948}\u{902}कोवा नगरपालिका"), ("hr", "Općina Cankova"), ("id", "Kotamadya Cankova"), ("it", "Cankova"), ("ja", "ツァンコヴァ"), ("kn", "ಕ\u{ccd}ಯಾಂಕೊವಾ ಪುರಸಭ\u{cc6}"), ("ko", "찬토바 지방 자치제"), ("lt", "Cankovos savivaldybė"), ("lv", "Cankovas pašvaldība"), ("mr", "कॉ\u{902}कोवा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cankova Municipality"), ("nb", "Cankova kommune"), ("nl", "Cankova"), ("no", "Cankova kommune"), ("pl", "Gmina Cankova"), ("pt", "Município de Cankova"), ("ro", "Comuna Cankova"), ("ru", "Канкова"), ("si", "කන\u{dca}කොව\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Cankova"), ("sr", "Општина Цанкова"), ("sr_Latn", "Opština Cankova"), ("sv", "Cankova kommun"), ("ta", "க\u{bbe}ங\u{bcd}கோவ\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c3e}ంక\u{c4b}వ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลแคนโกวา"), ("tr", "Cankova Belediyesi"), ("uk", "Цанкова"), ("ur", "بلدیہ تسانکووا"), ("vi", "Đô thị tự trị Cankova"), ("zh", "灿科瓦")]),
                        unofficial_name_list: ["Cankova"].to_vec(),
                    }
                ),
                (
                    "153",
                    Subdivision{
                        name: "153",
                        country_alpha2: Alpha2::SI,
                        code: "153",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5670711), longitude: Some(15.9429753), max_latitude: Some(46.5695352), min_latitude: Some(46.5602132), max_longitude: Some(15.9554071), min_longitude: Some(15.9381732)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيركفنياك"), ("bn", "চ\u{9be}র\u{9cd}কভেঞ\u{9cd}জ\u{9be}ক পৌরসভ\u{9be}"), ("ccp", "𑄇𑄢\u{11134}𑄞𑄬𑄚\u{11134}𑄎𑄇\u{11134}"), ("ceb", "Cerkvenjak"), ("cs", "Občina Cerkvenjak"), ("da", "Cerkvenjak Municipality"), ("de", "Gemeinde Cerkvenjak"), ("el", "Κέρκβεντζακ"), ("en", "Cerkvenjak"), ("es", "Municipalidad del Cerkvenjak"), ("fi", "Cerkvenjakin kunta"), ("fr", "Municipalité de Cerkvenjak"), ("gu", "સિર\u{acd}કવ\u{ac7}નજક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "चर\u{94d}कव\u{947}न\u{94d}नय\u{947}क नगर पालिका"), ("hr", "Općina Cerkvenjak"), ("id", "Kotamadya Cerkvenjak"), ("it", "Cerkvenjak"), ("ja", "ツェルクヴェニャク"), ("kn", "ಸ\u{cc6}ರ\u{ccd}ಕ\u{cc6}ವ\u{cc6}ಜಾಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "체르크베냐크 지방 자치제"), ("lt", "Cerkveniako savivaldybė"), ("lv", "Cerkvenjakas pašvaldība"), ("mr", "सरल\u{947}व\u{94d}हनजक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Cerkvenjak Municipality"), ("nb", "Cerkvenjak kommune"), ("nl", "Cerkvenjak"), ("no", "Cerkvenjak kommune"), ("pl", "Gmina Cerkvenjak"), ("pt", "Município de Cerkvenjak"), ("ro", "Comuna Cerkvenjak"), ("ru", "Церквеняк"), ("si", "සර\u{dca}ක\u{dca}වෙන\u{dca}ජක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Cerkvenjak"), ("sr", "Церквењак"), ("sr_Latn", "Cerkvenjak"), ("sv", "Cerkvenjak kommun"), ("ta", "சேர\u{bcd}க\u{bcd}கவெஞ\u{bcd}ச\u{bbe}க\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "సర\u{c4d}క\u{c46}వ\u{c46}ంజ\u{c3e}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเจอร\u{e4c}คเวนย\u{e31}ก"), ("tr", "Cerkvenjak Belediyesi"), ("uk", "Церквеняк"), ("ur", "بلدیہ تسیرکوینیاک"), ("vi", "Đô thị tự trị Cerkvenjak"), ("zh", "采尔克韦尼亚克")]),
                        unofficial_name_list: ["Cerkvenjak"].to_vec(),
                    }
                ),
                (
                    "154",
                    Subdivision{
                        name: "154",
                        country_alpha2: Alpha2::SI,
                        code: "154",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.134898), longitude: Some(15.3963025), max_latitude: Some(46.1601974), min_latitude: Some(46.10936359999999), max_longitude: Some(15.4371281), min_longitude: Some(15.3630527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبي"), ("bn", "দোবে পৌরসভ\u{9be}"), ("bs", "Dobje"), ("ccp", "𑄓\u{11127}𑄛\u{11134}𑄎𑄬"), ("ceb", "Dobje (munisipyo sa Eslobenya)"), ("cs", "Občina Dobje"), ("da", "Dobje Municipality"), ("de", "Dobje"), ("el", "Ντόμπτζε"), ("en", "Dobje"), ("es", "Dobje"), ("fi", "Dobjen kunta"), ("fr", "Dobje"), ("gu", "ડોબ\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोबिए नगर पालिका"), ("hr", "Općina Dobje"), ("hu", "Dobje"), ("id", "Kotamadya Dobje"), ("it", "Dobje"), ("ja", "ドビェ"), ("kn", "ಡೋಬ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "도베"), ("lt", "Dobdžės savivaldybė"), ("lv", "Dobjes pašvaldība"), ("mr", "डोब\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dobje Municipality"), ("nb", "Dobje"), ("nl", "Dobje"), ("no", "Dobje"), ("pl", "Gmina Dobje"), ("pt", "Dobje"), ("ro", "Dobje"), ("ru", "Добье"), ("si", "ඩොබ\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Dobje"), ("sr", "Општина Добје"), ("sr_Latn", "Opština Dobje"), ("sv", "Dobje"), ("ta", "டோபிஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}బ\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องด\u{e4a}อบเจ"), ("tr", "Dobje Belediyesi"), ("uk", "Добє (община)"), ("ur", "دوبجے میونسپلٹی"), ("vi", "Dobje"), ("zh", "多别")]),
                        unofficial_name_list: ["Dobje"].to_vec(),
                    }
                ),
                (
                    "155",
                    Subdivision{
                        name: "155",
                        country_alpha2: Alpha2::SI,
                        code: "155",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3356141), longitude: Some(15.2259732), max_latitude: Some(46.3493703), min_latitude: Some(46.3319907), max_longitude: Some(15.2373527), min_longitude: Some(15.2055934)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دوبرنا"), ("bn", "ডর\u{9cd}বন\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄓\u{11127}𑄛\u{11134}𑄢\u{11134}𑄚"), ("ceb", "Dobrna"), ("cs", "Občina Dobrna"), ("da", "Dobrna Municipality"), ("de", "Dobrna"), ("el", "Ντόμπρνα"), ("en", "Dobrna"), ("es", "Občina Dobrna"), ("fi", "Dobrnan kunta"), ("fr", "Dobrna"), ("gu", "ડોબ\u{acd}રના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोबरना नगरपालिका"), ("hr", "Općina Dobrna"), ("id", "Kotamadya Dobrna"), ("it", "Dobrna"), ("ja", "ドブラナ"), ("kn", "ಡೊಬ\u{ccd}ರ\u{ccd}ನಾ ಪುರಸಭ\u{cc6}"), ("ko", "도브르나"), ("lt", "Dobrnos savivaldybė"), ("lv", "Dobrnas pašvaldība"), ("mr", "डोब\u{94d}रा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dobrna Municipality"), ("nb", "Dobrna kommune"), ("nl", "Dobrna"), ("no", "Dobrna kommune"), ("pl", "Gmina Dobrna"), ("pt", "Dobrna"), ("ro", "Dobrna"), ("ru", "Добрна"), ("si", "ඩොබ\u{dca}\u{200d}ර\u{dca}න\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Dobrna"), ("sr", "Општина Добрна"), ("sr_Latn", "Opština Dobrna"), ("sv", "Dobrna kommun"), ("ta", "டொப\u{bcd}ரண\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}బర\u{c4d}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องดอบระนา"), ("tr", "Dobrna Belediyesi"), ("uk", "Добрна"), ("ur", "دوبرنا میونسپلٹی"), ("vi", "Dobrna"), ("zh", "多布爾納鎮")]),
                        unofficial_name_list: ["Dobrna"].to_vec(),
                    }
                ),
                (
                    "156",
                    Subdivision{
                        name: "156",
                        country_alpha2: Alpha2::SI,
                        code: "156",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6516758), longitude: Some(16.3429113), max_latitude: Some(46.6754685), min_latitude: Some(46.6121896), max_longitude: Some(16.3748429), min_longitude: Some(16.3092205)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓\u{11127}𑄛\u{11134}𑄢\u{1112e}𑄛\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Dobrovnik"), ("cs", "Občina Dobrovnik"), ("en", "Dobrovnik"), ("hr", "Općina Dobrovnik"), ("hu", "Dobronak község"), ("it", "Dobrovnik"), ("nb", "Dobronak kommune"), ("nl", "Dobrovnik"), ("no", "Dobronak kommune"), ("pl", "Gmina Dobrovnik"), ("ro", "Comuna Dobrovnik"), ("sl", "Občina Dobrovnik"), ("sr", "Општина Добровник"), ("sr_Latn", "Opština Dobrovnik"), ("uk", "Добровник"), ("zh", "多布罗夫尼克")]),
                        unofficial_name_list: ["Dobrovnik/Dobronak"].to_vec(),
                    }
                ),
                (
                    "157",
                    Subdivision{
                        name: "157",
                        country_alpha2: Alpha2::SI,
                        code: "157",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7558584), longitude: Some(15.0592333), max_latitude: Some(45.7667726), min_latitude: Some(45.7422996), max_longitude: Some(15.0779699), min_longitude: Some(15.0491719)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية دولينيسكي توبليس"), ("bn", "ডলেনজস\u{9cd}ক টোপলিস পৌরসভ\u{9be}"), ("ccp", "𑄓\u{11127}𑄣𑄬𑄚\u{11134}𑄌\u{11134}𑄇𑄬 𑄑\u{11127}𑄛\u{11134}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Dolenjske Toplice"), ("cs", "Občina Dolenjske Toplice"), ("da", "Dolenjske Toplice Municipality"), ("de", "Dolenjske Toplice"), ("el", "Ντολέντζσκε"), ("en", "Dolenjske Toplice"), ("es", "Municipalidad Dolenjske Toplice"), ("fi", "Dolenjske Toplicen kunta"), ("fr", "Dolenjske Toplice"), ("gu", "ડોલ\u{ac7}ન\u{acd}સ\u{acd}ક\u{ac7} ટોપલાઈસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "डोल\u{94d}न\u{94d}जस\u{94d}क\u{947} टॉप\u{94d}लिस नगर पालिका"), ("hr", "Općina Dolenjske Toplice"), ("id", "Kotamadya Dolenjske Toplice"), ("it", "Dolenjske Toplice"), ("ja", "ドレーニスケ・トプリーツェ"), ("kn", "ಡೊಲ\u{cc6}ನ\u{ccd}ಜ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಟೋಪ\u{ccd}ಲ\u{cbf}ಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "돌렌스케토플리체"), ("lt", "Dolenskės Toplicės savivaldybė"), ("lv", "Dolenjskes Toplices pašvaldība"), ("mr", "डॉल\u{94d}न\u{94d}ज\u{947}स\u{94d} टोप\u{94d}लाइस म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Dolenjske Toplice Municipality"), ("nb", "Dolenjske Toplice kommune"), ("nl", "Dolenjske Toplice"), ("no", "Dolenjske Toplice kommune"), ("pl", "Gmina Dolenjske Toplice"), ("pt", "Dolenjske Toplice"), ("ro", "Dolenjske Toplice"), ("ru", "Доленьске-Топлице"), ("si", "ඩොලේන\u{dca}ජ\u{dca}ස\u{dca}කේ ටොප\u{dca}ල\u{dd2}ස\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Dolenjske Toplice"), ("sr", "Општина Долењске Топлице"), ("sr_Latn", "Opština Dolenjske Toplice"), ("sv", "Dolenjske Toplice kommun"), ("ta", "டோலேஞ\u{bcd}ஸ\u{bcd}கி டோபிலிஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "డ\u{c4b}ల\u{c46}ంజ\u{c46}స\u{c4d}క\u{c4d} ట\u{c3e}ప\u{c4d}ల\u{c3f}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โดเลนจ\u{e4c}สเกโทพล\u{e34}ซ"), ("tr", "Dolenjske Toplice Belediyesi"), ("uk", "Доленьське Топлице"), ("ur", "دولینجسکی توپلیکی میونسپلٹی"), ("vi", "Dolenjske Toplice"), ("zh", "多萊尼斯凱托普利采鎮")]),
                        unofficial_name_list: ["Dolenjske Toplice"].to_vec(),
                    }
                ),
                (
                    "158",
                    Subdivision{
                        name: "158",
                        country_alpha2: Alpha2::SI,
                        code: "158",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.801689), longitude: Some(16.0924382), max_latitude: Some(46.8309741), min_latitude: Some(46.7819677), max_longitude: Some(16.1148534), min_longitude: Some(16.070861)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية غراد"), ("bg", "Град"), ("bn", "গ\u{9cd}র\u{9be}ড পৌরসভ\u{9be}"), ("ccp", "𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Grad"), ("cs", "Občina Grad"), ("da", "Grad Municipality"), ("de", "Grad"), ("el", "Γκραντ"), ("en", "Grad"), ("es", "Grad"), ("fi", "Gradin kunta"), ("fr", "Grad"), ("gu", "ગ\u{acd}ર\u{ac7}ડ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ग\u{94d}र\u{948}ड नगरपालिका"), ("hr", "Općina Grad"), ("hu", "Felsőlendva község"), ("id", "Kotamadya Grad"), ("it", "Grad"), ("ja", "グラード"), ("kn", "ಗ\u{ccd}ರ\u{ccd}ಯಾಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "그라드"), ("lt", "Grado savivaldybė"), ("lv", "Gradas pašvaldība"), ("mr", "ग\u{94d}र\u{945}ड म\u{94d}य\u{941}निसिपलिटी"), ("ms", "Grad Municipality"), ("nb", "Grad Kommune"), ("nl", "Grad"), ("no", "Grad Kommune"), ("pl", "Gmina Grad"), ("pt", "Grad"), ("ro", "Grad"), ("ru", "Град"), ("si", "ග\u{dca}\u{200d}රඩ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Grad"), ("sr", "Општина Град"), ("sr_Latn", "Opština Grad"), ("sv", "Grad"), ("ta", "கிரேட\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "గ\u{c4d}ర\u{c3e}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดเกรด"), ("tr", "Grad Belediyesi"), ("uk", "Град"), ("ur", "گراڈ میونسپلٹی"), ("vi", "Đô thị tự trị Grad"), ("zh", "格拉德")]),
                        unofficial_name_list: ["Grad"].to_vec(),
                    }
                ),
                (
                    "159",
                    Subdivision{
                        name: "159",
                        country_alpha2: Alpha2::SI,
                        code: "159",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4185014), longitude: Some(15.8244722), max_latitude: Some(46.4570904), min_latitude: Some(46.3827696), max_longitude: Some(15.8682326), min_longitude: Some(15.7916534)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هايدينا"), ("bn", "হ\u{9be}জদিন\u{9be} পৌরসভ\u{9be}"), ("bs", "Hajdina"), ("ccp", "𑄦𑄌\u{11134}𑄓\u{11128}𑄥"), ("ceb", "Hajdina"), ("cs", "Občina Hajdina"), ("da", "Municipality of Hajdina"), ("de", "Hajdina"), ("el", "Χατζντίνα"), ("en", "Hajdina"), ("es", "Municipalidade del Hajdina"), ("fi", "Hajdinan kunta"), ("fr", "Hajdina"), ("gu", "હઝડીના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "हदीना नगरपालिका"), ("hr", "Općina Hajdina"), ("id", "Kotamadya Hajdina"), ("it", "Hajdina"), ("ja", "ハイディナ"), ("kn", "ಹಜ\u{ccd}ದ\u{cbf}ನಾ ಪುರಸಭ\u{cc6}"), ("ko", "하이디나"), ("lt", "Hajdinos savivaldybė"), ("lv", "Hajdinas pašvaldība"), ("mr", "हजदीना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Hajdina Municipality"), ("nb", "Hajdina Kommune"), ("nl", "Hajdina"), ("no", "Hajdina Kommune"), ("pl", "Gmina Hajdina"), ("pt", "Hajdina"), ("ro", "Hajdina"), ("ru", "Хайдина"), ("si", "හජ\u{dca}ද\u{dd2}න\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Hajdina"), ("sr", "Општина Хајдина"), ("sr_Latn", "Opština Hajdina"), ("sv", "Hajdina"), ("ta", "ஹஜ\u{bcd}ஜிடின\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "హజ\u{c4d}డ\u{c3f}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องฮาจจ\u{e34}นา"), ("tr", "Hajdina Belediyesi"), ("uk", "Хайдина"), ("ur", "حاجدینا میونسپلٹی"), ("vi", "Hajdina"), ("zh", "哈伊迪纳")]),
                        unofficial_name_list: ["Hajdina"].to_vec(),
                    }
                ),
                (
                    "160",
                    Subdivision{
                        name: "160",
                        country_alpha2: Alpha2::SI,
                        code: "160",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.477858), longitude: Some(15.6476005), max_latitude: Some(46.5216497), min_latitude: Some(46.4586495), max_longitude: Some(15.7108982), min_longitude: Some(15.5359515)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هوتشه-سليفنيكا"), ("bn", "হোচে-স\u{9cd}লিভনিক\u{9be} পৌরসভ\u{9be}"), ("bs", "Hoče-Slivnica"), ("ccp", "𑄦\u{1112e}𑄌\u{11134}-𑄥\u{11133}𑄣\u{11128}𑄛\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Občina Hoče-Slivnica"), ("cs", "Občina Hoče-Slivnica"), ("da", "Hoče–Slivnica Municipality"), ("de", "Hoče-Slivnica"), ("el", "Χότσε-Σλιβνίκα"), ("en", "Hoče–Slivnica"), ("es", "Hoče-Slivnica"), ("fi", "Hoče–Slivnican kunta"), ("fr", "Hoče-Slivnica"), ("gu", "હોક\u{ac7}-સ\u{acd}લાઈનિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "होस\u{947}-स\u{94d}लिवनिका नगरपालिका"), ("hr", "Općina Hoče - Slivnica"), ("id", "Kotamadya Hoče–Slivnica"), ("it", "Hoče-Slivnica"), ("kn", "ಹೋಸ\u{cc6}-ಸ\u{ccd}ಲ\u{cbf}ವ\u{ccd}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "호체슬리브니차"), ("lt", "Hoče-Slivnicos savivaldybė"), ("lv", "Hoče–Slivnicas pašvaldība"), ("mr", "होक\u{947}-स\u{94d}लिव\u{94d}निका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Hoce–Slivnica Municipality"), ("nb", "Hoce-Slivnica kommune"), ("nl", "Hoče-Slivnica"), ("no", "Hoce-Slivnica kommune"), ("pl", "Gmina Hoče-Slivnica"), ("pt", "Hoče-Slivnica"), ("ro", "Comuna Hoče-Slivnica"), ("ru", "Хоче-Сливница"), ("si", "හොසේ ස\u{dca}ල\u{dd2}ව\u{dd2}න\u{dca}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Hoče - Slivnica"), ("sr", "Општина Хоче - Сливница"), ("sr_Latn", "Opština Hoče - Slivnica"), ("sv", "Hoče-Slivnica"), ("ta", "ஹோஸ\u{bcd} –ஸ\u{bcd}ல\u{bc0}வ\u{bcd}னிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "హ\u{c4b}స\u{c4d}-స\u{c3f}ల\u{c4d}వ\u{c3f}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โฮเซ-สล\u{e34}ฟน\u{e34}คา"), ("tr", "Hoče - Slivnica Beledyesi"), ("uk", "Хоче-Сливниця"), ("vi", "Hoče - Slivnica"), ("zh", "霍采-斯利夫尼察")]),
                        unofficial_name_list: ["Hoce-Slivnica"].to_vec(),
                    }
                ),
                (
                    "161",
                    Subdivision{
                        name: "161",
                        country_alpha2: Alpha2::SI,
                        code: "161",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8314134), longitude: Some(16.321068), max_latitude: Some(46.8664431), min_latitude: Some(46.8128415), max_longitude: Some(16.3508332), min_longitude: Some(16.2907565)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هودوش"), ("bn", "হোডোস পৌরসভ\u{9be}"), ("ccp", "𑄦\u{1112e}𑄓\u{1112e}𑄌\u{11134}"), ("ceb", "Hodos"), ("cs", "Občina Hodoš"), ("da", "Hodoš"), ("de", "Gemeinde Hodoš"), ("el", "Χόντος"), ("en", "Hodoš"), ("es", "Municipalidad Hodoš"), ("fi", "Hodošnin kunta"), ("fr", "Hodos"), ("gu", "હોડોસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "होडोस नगर पालिका"), ("hr", "Općina Hodoš"), ("hu", "Őrihodos község"), ("id", "Kotamadya Hodoš"), ("it", "Hodoš"), ("ja", "ホドシュ"), ("kn", "ಹೊಡೊಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "호도스 지방 자치제"), ("lt", "Hodošo savivaldybė"), ("lv", "Hodošas pašvaldība"), ("mr", "होडो म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Hodos Municipality"), ("nb", "Hodos kommune"), ("nl", "Hodoš"), ("no", "Hodos kommune"), ("pl", "Gmina Hodoš"), ("pt", "Município de Hodos"), ("ro", "Comuna Hodoš"), ("ru", "Ходош"), ("si", ", හොඩොස\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Hodoš"), ("sr", "Општина Ходош"), ("sr_Latn", "Opština Hodoš"), ("sv", "Hodos kommun"), ("ta", "ஓடோஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "హ\u{c4b}డ\u{c4b}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องโฮดอส"), ("tr", "Hodos Belediyesi"), ("uk", "Ходош"), ("ur", "ہودوس میونسپلٹی"), ("vi", "Đô thị tự trị Hodos"), ("zh", "霍多什鎮")]),
                        unofficial_name_list: ["Hodoš/Hodos"].to_vec(),
                    }
                ),
                (
                    "162",
                    Subdivision{
                        name: "162",
                        country_alpha2: Alpha2::SI,
                        code: "162",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.02253779999999), longitude: Some(14.2986269), max_latitude: Some(46.0321054), min_latitude: Some(46.0030247), max_longitude: Some(14.3166555), min_longitude: Some(14.2826504)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية هريول"), ("bn", "হর\u{9cd}জ\u{9c1}ল পৌরসভ\u{9be}"), ("ccp", "𑄦\u{1112e}𑄢\u{11134}𑄎\u{1112a}𑄣\u{11134}"), ("ceb", "Horjul"), ("cs", "Občina Horjul"), ("da", "Horjul Municipality"), ("de", "Horjul"), ("el", "Χόρτζουλ"), ("en", "Horjul"), ("es", "Horjul"), ("fi", "Horjulin kunta"), ("fr", "Horjul"), ("gu", "હોરજ\u{ac1}લ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "होर\u{94d}ज\u{941}ल नगर पालिका"), ("hr", "Općina Horjul"), ("id", "Kotamadya Horjul"), ("it", "Horjul"), ("ja", "ホリュル"), ("kn", "ಹೊರ\u{ccd}ಜುಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "호률"), ("lt", "Chorjulo savivaldybė"), ("lv", "Horjulas pašvaldība"), ("mr", "होज\u{941}ल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Horjul Municipality"), ("nb", "Horjul Kommune"), ("nl", "Horjul"), ("no", "Horjul Kommune"), ("pl", "Gmina Horjul"), ("pt", "Horjul"), ("ro", "Horjul"), ("ru", "Хорьюл"), ("si", "හොර\u{dca}ජල\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Horjul"), ("sr", "Општина Хорјул"), ("sr_Latn", "Opština Horjul"), ("sv", "Horjul"), ("ta", "ஹொர\u{bcd}ஜுல\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "హ\u{c4b}ర\u{c4d}జుల\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องฮอร\u{e4c}จ\u{e38}ล"), ("tr", "Horjul Belediyesi"), ("uk", "Хорюл"), ("ur", "ہورجول میونسپلٹی"), ("vi", "Đô thị tự trị Horjul"), ("zh", "霍尔尤尔")]),
                        unofficial_name_list: ["Horjul"].to_vec(),
                    }
                ),
                (
                    "163",
                    Subdivision{
                        name: "163",
                        country_alpha2: Alpha2::SI,
                        code: "163",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3914249), longitude: Some(14.4623209), max_latitude: Some(46.4271851), min_latitude: Some(46.34558819999999), max_longitude: Some(14.5652288), min_longitude: Some(14.4013998)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جيزرسكو"), ("bg", "Езерско"), ("bn", "জেজেরস\u{9cd}কো পৌরসভ\u{9be}"), ("bs", "Jezersko"), ("ccp", "𑄎𑄬𑄎𑄢\u{11134}𑄖\u{1112e}"), ("ceb", "Jezersko (munisipyo sa Eslobenya)"), ("cs", "Občina Jezersko"), ("da", "Jezersko Municipality"), ("de", "Jezersko"), ("el", "Τζεζέρσκο"), ("en", "Jezersko"), ("es", "Municipalidad del Jezersko"), ("fi", "Jezerskon kunta"), ("fr", "Jezersko (Slovénie)"), ("gu", "જ\u{ac7}ઝ\u{ac7}ર\u{acd}સ\u{acd}કો મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{947}ज\u{93c}\u{947}र\u{94d}सको नगरपालिका"), ("hr", "Općina Jezersko"), ("id", "Kotamadya Jezersko"), ("it", "Jezersko"), ("ja", "イェゼルスコ"), ("kn", "ಜ\u{cc6}ಜ\u{cc6}ರ\u{ccd}ಸ\u{ccd}ಕೊ ಪುರಸಭ\u{cc6}"), ("ko", "예제르스코"), ("lt", "Ezersko savivaldybė"), ("lv", "Jezersko pašvaldība"), ("mr", "ज\u{947}झ\u{947}रस\u{94d}को म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Jezersko Municipality"), ("nb", "Jezersko kommune"), ("nl", "Jezersko"), ("no", "Jezersko kommune"), ("pl", "Gmina Jezersko"), ("pt", "Jezersko (Eslovênia)"), ("ro", "Jezersko"), ("ru", "Езерско"), ("si", "ජෙසර\u{dca}කෝ නගර සභ\u{dcf}ව"), ("sl", "Občina Jezersko"), ("sr", "Општина Језерско"), ("sr_Latn", "Opština Jezersko"), ("sv", "Jezersko kommun"), ("ta", "ஜெஸிரஸ\u{bcd}க\u{bcd}கோ நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}జ\u{c46}ర\u{c4d}సక\u{c4b} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องเจซเซอร\u{e4c}สโก\u{e49}"), ("tr", "Jezersko Belediyesi"), ("uk", "Єзерско (община)"), ("ur", "جیزیرسکو میونسپلٹی"), ("vi", "Đô thị tự trị Jezersko"), ("zh", "耶泽尔斯科")]),
                        unofficial_name_list: ["Jezersko"].to_vec(),
                    }
                ),
                (
                    "164",
                    Subdivision{
                        name: "164",
                        country_alpha2: Alpha2::SI,
                        code: "164",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2052815), longitude: Some(14.5391653), max_latitude: Some(46.2117197), min_latitude: Some(46.1945788), max_longitude: Some(14.5473957), min_longitude: Some(14.5138101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوميندا"), ("bn", "কোমেন\u{9cd}ড\u{9be} পৌরসভ\u{9be}"), ("ca", "Komenda"), ("ccp", "𑄇\u{1112e}𑄟𑄬𑄚\u{11134}𑄓"), ("ceb", "Komenda"), ("cs", "Občina Komenda"), ("da", "Komenda Municipality"), ("de", "Komenda"), ("el", "Κομέντα"), ("en", "Komenda"), ("es", "Komenda"), ("fi", "Komendan kunta"), ("fr", "Komenda"), ("gu", "કોમ\u{ac7}ન\u{acd}ડા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोम\u{947}न\u{94d}डा नगर पालिका"), ("hr", "Općina Komenda"), ("id", "Kotamadya Komenda"), ("it", "Komenda"), ("ja", "コメンダ"), ("kn", "ಕೊಮ\u{cc6}ಂಡಾ ಪುರಸಭ\u{cc6}"), ("ko", "코멘다"), ("lt", "Komendos savivaldybė"), ("lv", "Komendas pašvaldība"), ("mr", "कोम\u{947}ड\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Komenda Municipality"), ("nb", "Komenda Kommune"), ("nl", "Komenda"), ("no", "Komenda Kommune"), ("pl", "Gmina Komenda"), ("pt", "Komenda"), ("ro", "Komenda"), ("ru", "Коменда"), ("si", "කොමෙන\u{dca}ද\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Komenda"), ("sr", "Општина Коменда"), ("sr_Latn", "Opština Komenda"), ("sv", "Komenda kommun"), ("ta", "கொமென\u{bcd}ட\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}మ\u{c46}ండ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โกเมนดา"), ("tr", "Komeda"), ("uk", "Коменда (община)"), ("ur", "کومیندا میونسپلٹی"), ("vi", "Đô thị tự trị Komenda"), ("zh", "科门达")]),
                        unofficial_name_list: ["Komenda"].to_vec(),
                    }
                ),
                (
                    "165",
                    Subdivision{
                        name: "165",
                        country_alpha2: Alpha2::SI,
                        code: "165",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5083472), longitude: Some(14.9087946), max_latitude: Some(45.5130323), min_latitude: Some(45.5047512), max_longitude: Some(14.9142067), min_longitude: Some(14.900478)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كوستيل"), ("bg", "Костел"), ("bn", "কস\u{9cd}টেল পৌরসভ\u{9be}"), ("bs", "Kostel"), ("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄑𑄬𑄣\u{11134}"), ("ceb", "Kostel"), ("cs", "Občina Kostel"), ("da", "Kostel Municipality"), ("de", "Gemeinde Kostel"), ("el", "Κοστέλ"), ("en", "Kostel"), ("es", "Municipalidad del Kostel"), ("fi", "Kostelin kunta"), ("fr", "Municipalité de Kostel"), ("gu", "કોસ\u{acd}ટ\u{ac7}લ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "कोस\u{94d}टल नगर पालिका"), ("hr", "Općina Kostel"), ("id", "Kotamadya Kostel"), ("it", "Kostel"), ("ja", "コステル"), ("kn", "ಕೋಸ\u{ccd}ಟಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "코스텔 지방 자치제"), ("lt", "Kostelo savivaldybė"), ("lv", "Kostelas pašvaldība"), ("mr", "कोस\u{94d}टल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Kostel Municipality"), ("nb", "Kostel kommune"), ("nl", "Kostel"), ("no", "Kostel kommune"), ("pl", "Gmina Kostel"), ("pt", "Kostel"), ("ro", "Comuna Kostel"), ("ru", "Костел"), ("si", "කොස\u{dca}ටෙල\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Kostel"), ("sr", "Општина Костел"), ("sr_Latn", "Opština Kostel"), ("sv", "Kostel kommun"), ("ta", "கொஸ\u{bcd}டெல\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "క\u{c4b}స\u{c4d}ట\u{c46}ల\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องคอสเทล"), ("tr", "Kostel Belediyesi"), ("uk", "Костел"), ("ur", "کوستیل میونسپلٹی"), ("vi", "Đô thị tự trị Kostel"), ("zh", "科斯泰尔")]),
                        unofficial_name_list: ["Kostel"].to_vec(),
                    }
                ),
                (
                    "166",
                    Subdivision{
                        name: "166",
                        country_alpha2: Alpha2::SI,
                        code: "166",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.7923997), longitude: Some(16.2348194), max_latitude: Some(46.8043105), min_latitude: Some(46.7675639), max_longitude: Some(16.2714041), min_longitude: Some(16.2002046)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية كريجيفتسي"), ("bn", "ক\u{9cd}রিজেভচি পৌরসভ\u{9be}"), ("bs", "Križevci"), ("ccp", "𑄇\u{11133}𑄢\u{1112d}𑄎𑄬𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Občina Križevci"), ("cs", "Občina Križevci"), ("da", "Križevci Municipality"), ("de", "Križevci"), ("el", "Κριζέβκι"), ("en", "Križevci"), ("es", "Municipalidad del Križevci"), ("fi", "Križevcin kunta"), ("fr", "Križevci"), ("gu", "ક\u{acd}રીઝ\u{ac7}વ\u{acd}કી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "क\u{94d}रिज\u{93c}\u{947}व\u{94d}की नगर पालिका"), ("hr", "Općina Križevci"), ("id", "Kotamadya Križevci"), ("it", "Križevci"), ("ja", "クリジェフツィ"), ("kn", "ಕ\u{ccd}ರ\u{cbf}ಜ\u{ccd}ಜ\u{cc6}ಕ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "크리제브치"), ("lt", "Križevcio savivaldybė"), ("lv", "Križevci pašvaldība"), ("mr", "क\u{94d}रीझविची म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Krizevci Municipality"), ("nb", "Krizevci kommune"), ("nl", "Križevci"), ("no", "Krizevci kommune"), ("pl", "Gmina Križevci"), ("pt", "Križevci"), ("ro", "Križevci"), ("ru", "Крижевци"), ("si", "ක\u{dca}\u{200d}ර\u{dd2}සේව\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Križevci"), ("sr", "Општина Крижевци"), ("sr_Latn", "Opština Križevci"), ("sv", "Krizevci kommun"), ("ta", "க\u{bcd}ரிஸிவசி நகர\u{bbe}ட\u{bcd}சி"), ("te", "మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ไกซ\u{e34}วซ\u{e35} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Krizevci Belediyesi"), ("uk", "Крижевці"), ("ur", "کریزیوکی میونسپلٹی"), ("vi", "Đô thị tự trị Krizevci"), ("zh", "克里熱夫齊")]),
                        unofficial_name_list: ["Križevci"].to_vec(),
                    }
                ),
                (
                    "167",
                    Subdivision{
                        name: "167",
                        country_alpha2: Alpha2::SI,
                        code: "167",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.54027749999999), longitude: Some(15.3884817), max_latitude: Some(46.5523297), min_latitude: Some(46.528119), max_longitude: Some(15.412745), min_longitude: Some(15.3720041)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄣\u{1112e}𑄛\u{11134}𑄢𑄬𑄚\u{11134} 𑄚 𑄛\u{1112e}𑄦\u{1112e}𑄢\u{11134}𑄎\u{1112a}"), ("ceb", "Lovrenc na Pohorju"), ("cs", "Občina Lovrenc na Pohorju"), ("de", "Lovrenc na Pohorju"), ("en", "Lovrenc na Pohorju"), ("fr", "Lovrenc na Pohorju"), ("hr", "Općina Lovrenc na Pohorju"), ("it", "Lovrenc na Pohorju"), ("ja", "ロヴェンツェ・ナ・ポホリュ"), ("ko", "로브렌츠나포호류"), ("nl", "Lovrenc na Pohorju"), ("pl", "Gmina Lovrenc na Pohorju"), ("pt", "Lovrenc na Pohorju"), ("ro", "Lovrenc na Pohorju"), ("sl", "Občina Lovrenc na Pohorju"), ("sr", "Општина Ловренц на Похорју"), ("sr_Latn", "Opština Lovrenc na Pohorju"), ("uk", "Ловренц-на-Похорю"), ("vi", "Lovrenc na Pohorju"), ("zh", "波霍列山区洛夫伦茨")]),
                        unofficial_name_list: ["Lovrenc na Pohorju"].to_vec(),
                    }
                ),
                (
                    "168",
                    Subdivision{
                        name: "168",
                        country_alpha2: Alpha2::SI,
                        code: "168",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8428456), longitude: Some(16.2349557), max_latitude: Some(46.8644145), min_latitude: Some(46.8235579), max_longitude: Some(16.2596196), min_longitude: Some(16.2204124)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ماركوفتشي"), ("bn", "ম\u{9be}র\u{9cd}কোভসি পৌরসভ\u{9be}"), ("ccp", "𑄟𑄢\u{11134}𑄇\u{1112e}𑄛\u{11134}𑄥\u{11128}"), ("ceb", "Markovci"), ("cs", "Občina Markovci"), ("da", "Markovci Municipality"), ("de", "Markovci"), ("el", "Μαρκόβτσι"), ("en", "Markovci"), ("es", "Municipalidad del Markovci"), ("fi", "Markovcin kunta"), ("fr", "Markovci"), ("gu", "માર\u{acd}કોવિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मर\u{94d}कोव\u{94d}सी नगरपालिका"), ("hr", "Općina Markovci"), ("id", "Kotamadya Markovci"), ("it", "Markovci"), ("ja", "マルコフツィ"), ("kn", "ಮಾರ\u{ccd}ಕೊವ\u{cbf}ಸ\u{ccd}ಸ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "마르코브치"), ("lt", "Markovcio savivaldybė"), ("lv", "Markovcu pašvaldība"), ("mr", "मार\u{94d}कोव\u{94d}हसि म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Markovci Municipality"), ("nb", "Markovci kommune"), ("nl", "Markovci"), ("no", "Markovci kommune"), ("pl", "Gmina Markovci"), ("pt", "Markovci"), ("ro", "Markovci"), ("ru", "Марковцы"), ("si", "ම\u{dcf}ර\u{dca}කොව\u{dca}ස\u{dd2} නගර සභ\u{dcf}ව"), ("sl", "Občina Markovci"), ("sr", "Општина Марковци"), ("sr_Latn", "Opština Markovci"), ("sv", "Markovci kommun"), ("ta", "ம\u{bbe}ர\u{bcd}க\u{bcd}கோவ\u{bcd}சி நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3e}ర\u{c4d}క\u{c4b}వచ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องมาร\u{e4c}โควค\u{e34}"), ("tr", "Markovici Beledyesi"), ("uk", "Марковці"), ("ur", "مرکووکی میونسپلٹی"), ("vi", "Markovci"), ("zh", "马尔科夫齐")]),
                        unofficial_name_list: ["Markovci"].to_vec(),
                    }
                ),
                (
                    "169",
                    Subdivision{
                        name: "169",
                        country_alpha2: Alpha2::SI,
                        code: "169",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5082628), longitude: Some(15.6952065), max_latitude: Some(46.5171316), min_latitude: Some(46.4875418), max_longitude: Some(15.730069), min_longitude: Some(15.6817881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟\u{11128}𑄇\u{11134}𑄣𑄛\u{11134} 𑄚 𑄓\u{11133}𑄢𑄛\u{11134}𑄇𑄬𑄟\u{11134} 𑄛\u{11127}𑄣\u{11134}𑄎\u{1112a}"), ("ceb", "Občina Miklavž na Dravskem Polju"), ("cs", "Občina Miklavž na Dravskem Polju"), ("de", "Miklavž na Dravskem Polju"), ("en", "Miklavž na Dravskem Polju"), ("fr", "Miklavž na Dravskem polju"), ("hr", "Općina Miklavž na Dravskem polju"), ("it", "Miklavž na Dravskem polju"), ("ja", "ミクラヴジュ・ナ・ドラフスケム・ポリュ"), ("ko", "미클라브주나드라브스켐폴류"), ("nl", "Miklavž na Dravskem polju"), ("pl", "Gmina Miklavž na Dravskem polju"), ("pt", "Miklavž na Dravskem polju"), ("ro", "Miklavž na Dravskem polju"), ("sl", "Občina Miklavž na Dravskem polju"), ("sr", "Општина Миклавж на Дравском пољу"), ("sr_Latn", "Opština Miklavž na Dravskom polju"), ("uk", "Миклавж-на-Дравськем Полю"), ("vi", "Miklavž na Dravskem Polju"), ("zh", "德拉夫什卡波卢地区米克拉夫兹")]),
                        unofficial_name_list: ["Miklavž na Dravskem polju"].to_vec(),
                    }
                ),
                (
                    "170",
                    Subdivision{
                        name: "170",
                        country_alpha2: Alpha2::SI,
                        code: "170",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8594478), longitude: Some(15.0830457), max_latitude: Some(45.8708266), min_latitude: Some(45.8396018), max_longitude: Some(15.1094655), min_longitude: Some(15.0532856)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ميرنا بيتش"), ("bn", "মিরন\u{9be} পেচ পৌরসভ\u{9be}"), ("ccp", "𑄟\u{11128}𑄢\u{11134}𑄚 𑄛𑄬𑄇\u{11134}"), ("ceb", "Občina Mirna Peč"), ("cs", "Občina Mirna Peč"), ("da", "Mirna Peč Municipality"), ("de", "Mirna Peč"), ("el", "Μίρνα Πεκ"), ("en", "Mirna Peč"), ("es", "Mirna Peč"), ("fi", "Mirna Pečin kunta"), ("fr", "Mirna Peč"), ("gu", "મિર\u{acd}ના પ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "मीरना प\u{947}क नगरपालिका"), ("hr", "Općina Mirna Peč"), ("id", "Kotamadya Mirna Peč"), ("it", "Mirna Peč"), ("ja", "ミルナ・ペチ"), ("kn", "ಮ\u{cbf}ರ\u{ccd}ನಾ ಪ\u{cc6}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "미르나페치"), ("lt", "Mirna Pečo savivaldybė"), ("lv", "Mirna Pečas pašvaldība"), ("mr", "मिर\u{94d}ना प\u{947}क म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Mirna Pec Municipality"), ("nb", "Mirna Pec Kommune"), ("nl", "Mirna Peč"), ("no", "Mirna Pec Kommune"), ("pl", "Gmina Mirna Peč"), ("pt", "Mirna Peč"), ("ro", "Mirna Peč"), ("ru", "Мирна Печ"), ("si", "ම\u{dd2}ර\u{dca}න\u{dcf} පෙක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Mirna Peč"), ("sr", "Општина Мирна Печ"), ("sr_Latn", "Opština Mirna Peč"), ("sv", "Mirna Pec Kommun"), ("ta", "மிர\u{bcd}ன\u{bbe} பேக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "మ\u{c3f}ర\u{c4d}న\u{c3e} ప\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเมอนา เป\u{e4a}คร\u{e4c}"), ("tr", "Mirna Pec Belediyesi"), ("uk", "Мирна Печ"), ("ur", "مرنا پیک میونسپلٹی"), ("vi", "Mirna Peč"), ("zh", "米尔纳佩奇")]),
                        unofficial_name_list: ["Mirna Pec"].to_vec(),
                    }
                ),
                (
                    "171",
                    Subdivision{
                        name: "171",
                        country_alpha2: Alpha2::SI,
                        code: "171",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.387163), longitude: Some(15.4458131), max_latitude: Some(46.3960908), min_latitude: Some(46.3707474), max_longitude: Some(15.4692903), min_longitude: Some(15.4334869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أوبلوتنيكا"), ("bn", "ওপ\u{9cd}লোটনিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄃\u{11127}𑄛\u{11134}𑄣\u{1112e}𑄖\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Oplotnica"), ("cs", "Občina Oplotnica"), ("da", "Oplotnica Municipality"), ("de", "Oplotnica"), ("el", "Οπλοτνίκα"), ("en", "Oplotnica"), ("es", "Oplotnica"), ("fi", "Oplotnican kunta"), ("fr", "Oplotnica"), ("gu", "ઑપ\u{acd}લોટ\u{acd}નિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ओप\u{94d}लोटनिका नगरपालिका"), ("hr", "Općina Oplotnica"), ("id", "Kotamadya Oplotnica"), ("it", "Oplotnica"), ("ja", "オプロトニツァ"), ("kn", "ಒಪ\u{ccd}ಲೊಟ\u{ccd}ನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "오플로트니차"), ("lt", "Oplotnicos savivaldybė"), ("lv", "Oplotnicas pašvaldība"), ("mr", "ओप\u{94d}लॉटणीच म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Oplotnica Municipality"), ("nb", "Oplotnica kommune"), ("nl", "Oplotnica"), ("no", "Oplotnica kommune"), ("pl", "Gmina Oplotnica"), ("pt", "Oplotnica"), ("ro", "Oplotnica"), ("ru", "Оплотница"), ("si", "ඔප\u{dca}ලෝට\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Oplotnica"), ("sr", "Општина Оплотница"), ("sr_Latn", "Opština Oplotnica"), ("sv", "Oplotnica"), ("ta", "ஒப\u{bcd}ளோட\u{bcd}னிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఓప\u{c4d}ల\u{c3e}ట\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ออปลอตน\u{e34}ซา"), ("tr", "Oplotnica Belediyesi"), ("uk", "Оплотниця"), ("ur", "وپلوتنیکا میونسپلٹی"), ("vi", "Oplotnica"), ("zh", "奥普洛特尼察")]),
                        unofficial_name_list: ["Oplotnica"].to_vec(),
                    }
                ),
                (
                    "172",
                    Subdivision{
                        name: "172",
                        country_alpha2: Alpha2::SI,
                        code: "172",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3358974), longitude: Some(15.8787245), max_latitude: Some(46.3526277), min_latitude: Some(46.328792), max_longitude: Some(15.9010752), min_longitude: Some(15.8628032)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بودلنيك"), ("bn", "পোদলেহ\u{9cd}নিক পৌরসভ\u{9be}"), ("ccp", "𑄛\u{11127}𑄖\u{11134}𑄣𑄬𑄦\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Podlehnik"), ("cs", "Občina Podlehnik"), ("da", "Podlehnik Municipality"), ("de", "Podlehnik"), ("el", "Ποντλεχνίκ"), ("en", "Podlehnik"), ("es", "Municipalidad del Podlehnik"), ("fi", "Podlehnikin kunta"), ("fr", "Podlehnik"), ("gu", "પોડ\u{acd}લ\u{ac7}હનિક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पोडल\u{947}निक नगर पालिका"), ("hr", "Općina Podlehnik"), ("id", "Kotamadya Podlehnik"), ("it", "Podlehnik"), ("ja", "ポドレフニク"), ("kn", "ಪೋಡ\u{ccd}ಲ\u{cc6}ಹ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "포들레흐니크"), ("lt", "Podlechniko savivaldybė"), ("lv", "Podlehnikas pašvaldība"), ("mr", "पॉडल\u{947}हनिक म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Podlehnik Municipality"), ("nb", "Podlehnik kommune"), ("nl", "Podlehnik"), ("no", "Podlehnik kommune"), ("pl", "Gmina Podlehnik"), ("pt", "Podlehnik"), ("ro", "Podlehnik"), ("ru", "Подлехник"), ("si", "පොඩ\u{dca}ලෙහ\u{dca}න\u{dd2}ක\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Podlehnik"), ("sr", "Општина Подлехник"), ("sr_Latn", "Opština Podlehnik"), ("sv", "Podlehnik"), ("ta", "போடலேஹ\u{bcd}னய\u{bcd}க\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4b}డ\u{c4d}ల\u{c46}హ\u{c4d}న\u{c3f}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องพอดเลฮ\u{e4c}น\u{e34}ค"), ("tr", "Podlehnik Belediyesi"), ("uk", "Подлехник"), ("ur", "پودلیحنیک میونسپلٹی"), ("vi", "Podlehnik"), ("zh", "波德莱赫尼克")]),
                        unofficial_name_list: ["Podlehnik"].to_vec(),
                    }
                ),
                (
                    "173",
                    Subdivision{
                        name: "173",
                        country_alpha2: Alpha2::SI,
                        code: "173",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.280897), longitude: Some(15.0737321), max_latitude: Some(46.3021933), min_latitude: Some(46.2686365), max_longitude: Some(15.0857564), min_longitude: Some(15.0597401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بولزيلا"), ("bn", "পোলজেল\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄛\u{1112e}𑄣\u{11134}𑄎𑄬𑄣"), ("ceb", "Polzela"), ("cs", "Občina Polzela"), ("da", "Polzela Municipality"), ("de", "Polzela"), ("el", "Πολζέλα"), ("en", "Polzela"), ("es", "Municipalidad del Polzela"), ("fi", "Polzelan kunta"), ("fr", "Polzela"), ("gu", "પોલઝ\u{ac7}લા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "पोल\u{94d}ज\u{93c}\u{947}ला नगरपालिका"), ("hr", "Općina Polzela"), ("id", "Kotamadya Polzela"), ("it", "Polzela"), ("ja", "ポルゼラ"), ("kn", "ಪೊಲ\u{ccd}ಜ\u{cc6}ಲಾ ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("ko", "폴젤라"), ("lt", "Polzelos savivaldybė"), ("lv", "Polzelas pašvaldība"), ("mr", "पॉलझ\u{947}ला म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Polzela Municipality"), ("nb", "Polzela kommune"), ("nl", "Polzela"), ("no", "Polzela kommune"), ("pl", "Gmina Polzela"), ("pt", "Polzela"), ("ro", "Polzela"), ("ru", "Ползела"), ("si", "පොල\u{dca}සේල\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Polzela"), ("sr", "Општина Ползела"), ("sr_Latn", "Opština Polzela"), ("sv", "Polzela"), ("ta", "பொலிஸில\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4b}ల\u{c4d}జ\u{c46}ల\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "พอลเซลลา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Polzela Belediyesi"), ("uk", "Ползела (община)"), ("ur", "پولزیلا میونسپلٹی"), ("vi", "Polzela"), ("zh", "波尔泽拉")]),
                        unofficial_name_list: ["Polzela"].to_vec(),
                    }
                ),
                (
                    "174",
                    Subdivision{
                        name: "174",
                        country_alpha2: Alpha2::SI,
                        code: "174",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2365972), longitude: Some(15.091648), max_latitude: Some(46.2393412), min_latitude: Some(46.2244723), max_longitude: Some(15.1033384), min_longitude: Some(15.0761371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية بريبولد"), ("bn", "প\u{9cd}রিবোল\u{9cd}ড পৌরসভ\u{9be}"), ("ca", "Prebold"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄝\u{1112a}𑄣\u{11134}"), ("ceb", "Prebold"), ("cs", "Občina Prebold"), ("da", "Prebold Municipality"), ("de", "Prebold"), ("el", "Πρεμπόλντ"), ("en", "Prebold"), ("es", "Prebold"), ("fi", "Preboldin kunta"), ("fr", "Prebold"), ("gu", "પ\u{acd}રીબોલ\u{acd}ડ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}रीबॉल\u{94d}ड नगर पालिका"), ("hr", "Općina Prebold"), ("id", "Kotamadya Prebold"), ("it", "Prebold"), ("ja", "プレボルド"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ಬೋಲ\u{ccd}ಡ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "프레볼드"), ("lt", "Preboldo savivaldybė"), ("lv", "Preboldas pašvaldība"), ("mr", "प\u{94d}रीबॉल\u{94d}ड म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Prebold Municipality"), ("nb", "Prebold Kommune"), ("nl", "Prebold"), ("no", "Prebold Kommune"), ("pl", "Gmina Prebold"), ("pt", "Prebold"), ("ro", "Prebold"), ("ru", "Преболд"), ("si", "ප\u{dca}\u{200d}ර\u{dd2}බෝල\u{dca}ඩ\u{dca} නගරසභ\u{dcf}ව"), ("sl", "Občina Prebold"), ("sr", "Општина Преболд"), ("sr_Latn", "Opština Prebold"), ("sv", "Prebold Kommun"), ("ta", "ப\u{bcd}ரெபோல\u{bcd}டு நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c3f}బ\u{c4b}ల\u{c4d}డ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องพร\u{e35}โบล"), ("tr", "Prebold Belediyesi"), ("uk", "Преболд"), ("ur", "پریبولد میونسپلٹی"), ("vi", "Đô thị tự trị Prebold"), ("zh", "普雷博尔德")]),
                        unofficial_name_list: ["Prebold"].to_vec(),
                    }
                ),
                (
                    "175",
                    Subdivision{
                        name: "175",
                        country_alpha2: Alpha2::SI,
                        code: "175",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.54687879999999), longitude: Some(14.9197479), max_latitude: Some(46.5543747), min_latitude: Some(46.5355214), max_longitude: Some(14.9388744), min_longitude: Some(14.8907918)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بريفالجه"), ("bn", "প\u{9cd}রিভ\u{9cd}লিজ পৌরসভ\u{9be}"), ("ca", "Prevalje"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄞𑄣\u{11134}𑄎𑄬"), ("ceb", "Prevalje (munisipyo)"), ("cs", "Občina Prevalje"), ("da", "Prevalje Municipality"), ("de", "Prevalje"), ("el", "Πρεβαλτζέ"), ("en", "Prevalje"), ("es", "Prevalje"), ("fa", "پروالیه"), ("fi", "Prevalje"), ("fr", "Prevalje"), ("gu", "પ\u{acd}ર\u{ac7}વલ\u{acd}જ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "प\u{94d}रीव\u{947}लिय\u{947} नगरपालिका"), ("hr", "Općina Prevalje"), ("id", "Kotamadya Prevalje"), ("it", "Prevalje"), ("ja", "プレヴァリェ"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ವಲ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "프레발레"), ("lt", "Prevalės"), ("lv", "Prevaljes pašvaldība"), ("mr", "प\u{94d}रीव\u{947}ल\u{94d}व\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Prevalje Municipality"), ("nb", "Prevalje kommune"), ("nl", "Prevalje"), ("no", "Prevalje kommune"), ("pl", "Gmina Prevalje"), ("pt", "Prevalje"), ("ro", "Prevalje"), ("ru", "Превалье"), ("si", "ප\u{dca}රෙවල\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sk", "Prevalje"), ("sl", "Občina Prevalje"), ("sr", "Општина Преваље"), ("sr_Latn", "Opština Prevalje"), ("sv", "Prevalje kommun"), ("ta", "பிரெவ\u{bcd}ளஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ప\u{c4d}ర\u{c46}వ\u{c3e}ల\u{c4d}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเปรว\u{e31}ลเจ"), ("tr", "Prevalje Belediyesi"), ("uk", "Превалє"), ("ur", "پریوالجی میونسپلٹی"), ("vi", "Prevalje"), ("zh", "普雷瓦列")]),
                        unofficial_name_list: ["Prevalje"].to_vec(),
                    }
                ),
                (
                    "176",
                    Subdivision{
                        name: "176",
                        country_alpha2: Alpha2::SI,
                        code: "176",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5246331), longitude: Some(16.27627), max_latitude: Some(46.538927), min_latitude: Some(46.5141574), max_longitude: Some(16.2829085), min_longitude: Some(16.2642234)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية رازكريجيه"), ("bn", "র\u{9be}জক\u{9cd}রিজে পৌরসভ\u{9be}"), ("ccp", "𑄢𑄌\u{11134}𑄢\u{11128}𑄌\u{11134}𑄎𑄬"), ("ceb", "Občina Razkrižje"), ("cs", "Občina Razkrižje"), ("da", "Razkrižje Municipality"), ("de", "Gemeinde Razkrižje"), ("el", "Ραζκρίζτζε"), ("en", "Razkrižje"), ("es", "Municipalidad del Razkrižje"), ("fi", "Razkrižjen kunta"), ("fr", "Municipalité de Razkrižje"), ("gu", "રઝક\u{acd}રીજ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "राजक\u{94d}रिज\u{93c}िए नगरपालिका"), ("hr", "Općina Razkrižje"), ("hu", "Ráckanizsa község"), ("id", "Kotamadya Razkrižje"), ("it", "Comune di Razkrižje"), ("ja", "ラスクリジイェ"), ("kn", "ರಾಜ\u{ccd}ಕ\u{ccd}ರ\u{cbf}ಜ\u{ccd}ಜ\u{cc6} ಪುರಸಭ\u{cc6}"), ("ko", "라즈크리제 지방 자치제"), ("lt", "Razkrižės savivaldybė"), ("lv", "Razkrižjes pašvaldība"), ("mr", "राझीग\u{94d}रीज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Razkrizje Municipality"), ("nb", "Razkrizje Kommune"), ("nl", "Razkrižje"), ("no", "Razkrizje Kommune"), ("pl", "Gmina Razkrižje"), ("pt", "Município de Razkrizje"), ("ro", "Comuna Razkrižje"), ("ru", "Разкрижье"), ("si", "රස\u{dca}ක\u{dca}ර\u{dd2}ස\u{dca}ජේ නගර සභ\u{dcf}ව"), ("sl", "Občina Razkrižje"), ("sr", "Општина Разкрижје"), ("sr_Latn", "Opština Razkrižje"), ("sv", "Razkrizje Kommun"), ("ta", "ர\u{bbe}ஸ\u{bcd}க\u{bcd}ரிஸ\u{bcd}ஜே நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3e}జ\u{c4d}\u{200c}క\u{c4d}ర\u{c3f}జ\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องราซคร\u{e34}ซเจ"), ("tr", "Razkrizje Belediyesi"), ("uk", "Разкрижє"), ("ur", "رازکریزجی میونسپلٹی"), ("vi", "Đô thị tự trị Razkrizje"), ("zh", "拉茲科日列")]),
                        unofficial_name_list: ["Razkrižje"].to_vec(),
                    }
                ),
                (
                    "177",
                    Subdivision{
                        name: "177",
                        country_alpha2: Alpha2::SI,
                        code: "177",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5356403), longitude: Some(15.2675136), max_latitude: Some(46.54968059999999), min_latitude: Some(46.52034339999999), max_longitude: Some(15.3026023), min_longitude: Some(15.2533515)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ريبنيكا نا بوهوريو"), ("bn", "রিবনিক\u{9be} ন\u{9be} পোহরজ\u{9c1} পৌরসভ\u{9be}"), ("ccp", "𑄢\u{11128}𑄛\u{11134}𑄚\u{11128}𑄇 𑄚 𑄛\u{11127}𑄦\u{1112e}𑄢\u{11134}𑄎\u{1112a}"), ("ceb", "Ribnica na Pohorju"), ("cs", "Občina Ribnica na Pohorju"), ("da", "Ribnica na Pohorju Municipality"), ("de", "Ribnica na Pohorju"), ("el", "Ριμπνίκα να Ποχόρτζου"), ("en", "Ribnica na Pohorju"), ("es", "Municipalidad Ribnica na Pohorju"), ("fi", "Ribnica na Pohorjun kunta"), ("fr", "Ribnica na Pohorju"), ("gu", "રિબ\u{acd}નિકા ના પોહોર\u{acd}જ\u{ac1} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "रिबनिका ना पोहोर\u{941} नगरपालिका"), ("hr", "Općina Ribnica na Pohorju"), ("id", "Kotamadya Ribnica na Pohorju"), ("it", "Ribnica na Pohorju"), ("ja", "リブニツァ・ナ・ポホリュ"), ("kn", "ರ\u{cbf}ಬ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ನಾ ಪೊಹೊರ\u{ccd}ಜು ಪುರಸಭ\u{cc6}"), ("ko", "리브니차나포호류"), ("lt", "Ribnica na Pochordžu"), ("lv", "Ribnicas na Pohorju pašvaldība"), ("mr", "रिबनिका ना पोहर\u{942} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ribnica na Pohorju Municipality"), ("nb", "Ribnica na Pohorju kommune"), ("nl", "Ribnica na Pohorju"), ("no", "Ribnica na Pohorju kommune"), ("pl", "Gmina Ribnica na Pohorju"), ("pt", "Ribnica na Pohorju"), ("ro", "Ribnica na Pohorju"), ("ru", "Рибница-на-Похорью"), ("si", "ර\u{dd2}බ\u{dca}න\u{dd2}ක\u{dcf} න\u{dcf} පොහොර\u{dca}ජ\u{dd4} නගර සභ\u{dcf}ව"), ("sl", "Občina Ribnica na Pohorju"), ("sr", "Општина Рибница на Похорју"), ("sr_Latn", "Opština Ribnica na Pohorju"), ("sv", "Ribnica na Pohorju kommun"), ("ta", "ரிப\u{bcd}பினிக\u{bcd}க\u{bbe} ந\u{bbe} போஹோர\u{bcd}ஜு நகர\u{bbe}ட\u{bcd}சி"), ("te", "ర\u{c3f}బ\u{c4d}న\u{c3f}క\u{c3e} న\u{c3e} ప\u{c4b}హ\u{c4b}ర\u{c4d}జు మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "พอโฮลจ\u{e39} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Ribnica Na Pohorju Belediyesi"), ("uk", "Рибниця-на-Похорю"), ("ur", "ریبنیکا نا پوحورجو میونسپلٹی"), ("vi", "Ribnica na Pohorju"), ("zh", "波霍爾尤爾地區里布尼察")]),
                        unofficial_name_list: ["Ribnica na Pohorju"].to_vec(),
                    }
                ),
                (
                    "178",
                    Subdivision{
                        name: "178",
                        country_alpha2: Alpha2::SI,
                        code: "178",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5513918), longitude: Some(15.492941), max_latitude: Some(46.56129259999999), min_latitude: Some(46.54081970000001), max_longitude: Some(15.510273), min_longitude: Some(15.4809647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سيلنيكا أوب درافي"), ("bn", "সেলনিক\u{9be} অব দ\u{9cd}র\u{9be}ভি পৌরসভ\u{9be}"), ("ccp", "𑄥𑄬𑄣\u{11134}𑄚\u{11128}𑄇 𑄃\u{11127}𑄛\u{11134} 𑄓\u{11133}𑄢𑄞\u{11128}"), ("ceb", "Selnica ob Dravi"), ("cs", "Občina Selnica ob Dravi"), ("da", "Selnica ob Dravi Municipality"), ("de", "Selnica ob Dravi"), ("el", "Σέλνικα ομπ Ντραβί"), ("en", "Selnica ob Dravi"), ("es", "Selnica ob Dravi"), ("fi", "Selnica ob Dravinin kunta"), ("fr", "Selnica ob Dravi"), ("gu", "સ\u{ac7}લ\u{acd}નિકા ઓબ ડ\u{acd}રાવી મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{947}ल\u{94d}निका ओब द\u{94d}रावी नगरपालिका"), ("hr", "Općina Selnica ob Dravi"), ("id", "Kotamadya Selnica ob Dravi"), ("it", "Selnica ob Dravi"), ("ja", "セルニツァ・オブ・ドラヴィ"), ("kn", "ಸ\u{cc6}ಲ\u{ccd}ನ\u{cbf}ಕಾ ಒಬ\u{ccd} ಡ\u{ccd}ರಾವ\u{cbf} ಪುರಸಭ\u{cc6}"), ("ko", "셀니차오브드라비"), ("lt", "Selnica ob Dravis"), ("lv", "Selnica ob Dravi"), ("mr", "स\u{947}लिना ओब ड\u{94d}रॉही म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Selnica ob Dravi Municipality"), ("nb", "Selnica ob Dravi kommune"), ("nl", "Selnica ob Dravi"), ("no", "Selnica ob Dravi kommune"), ("pl", "Gmina Selnica ob Dravi"), ("pt", "Selnica ob Dravi"), ("ro", "Selnica ob Dravi"), ("ru", "Сельница-об-Драви"), ("si", "සෙලන\u{dd2}ක\u{dcf} ඔබ\u{dca} ඩ\u{dca}රව\u{dd3} නගර සභ\u{dcf}ව"), ("sl", "Občina Selnica ob Dravi"), ("sr", "Општина Селница об Драви"), ("sr_Latn", "Opština Selnica ob Dravi"), ("sv", "Selnica ob Dravi kommun"), ("ta", "செலனிக\u{bcd}க\u{bbe} ஒப\u{bcd} ட\u{bcd}ர\u{bbe}வி நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ల\u{c4d}న\u{c3f}క\u{c3e} ఆబ\u{c4d} డ\u{c4d}ర\u{c3e}వ\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เซลน\u{e34}ชา ออบ ดราว\u{e35}"), ("tr", "Selnica ob Dravi"), ("uk", "Селниця-об-Драві"), ("ur", "سیلنیکا اوب دراوی میونسپلٹی"), ("vi", "Selnica ob Dravi"), ("zh", "德拉維河畔賽爾尼察")]),
                        unofficial_name_list: ["Selnica ob Dravi"].to_vec(),
                    }
                ),
                (
                    "179",
                    Subdivision{
                        name: "179",
                        country_alpha2: Alpha2::SI,
                        code: "179",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.762125), longitude: Some(14.6362158), max_latitude: Some(45.7848226), min_latitude: Some(45.7284568), max_longitude: Some(14.6461315), min_longitude: Some(14.6105465)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سودراجيتسا"), ("bn", "সোদ\u{9cd}র\u{9be}জিক\u{9be} পৌরসভ\u{9be}"), ("ccp", "𑄥\u{1112e}𑄓\u{11133}𑄢𑄎\u{11128}𑄇"), ("ceb", "Občina Sodražica"), ("cs", "Občina Sodražica"), ("da", "Sodražica Municipality"), ("de", "Sodražica"), ("el", "Σοντραζίκα"), ("en", "Sodražica"), ("es", "Municipalidad del Sodražica"), ("fi", "Sodražican kunta"), ("fr", "Sodražica"), ("gu", "સોદ\u{acd}રાજિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "सोद\u{94d}र\u{947}ज\u{93c}िका नगरपालिका"), ("hr", "Općina Sodražica"), ("id", "Kotamadya Sodražica"), ("it", "Sodražica"), ("ja", "ソドラジツァ"), ("kn", "ಸೋಡ\u{ccd}ರಾಜ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "소드라지차"), ("lt", "Dodražicos savivaldybė"), ("lv", "Sodražicas pašvaldība"), ("mr", "सोद\u{94d}र\u{947}झिका म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sodrazica Municipality"), ("nb", "Sodrazica kommune"), ("nl", "Sodražica"), ("no", "Sodrazica kommune"), ("pl", "Gmina Sodražica"), ("pt", "Sodražica"), ("ro", "Sodražica"), ("ru", "Содражица"), ("si", "සොද\u{dca}\u{200d}ර\u{dcf}ස\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Sodražica"), ("sr", "Општина Содражица"), ("sr_Latn", "Opština Sodražica"), ("sv", "Sodrazica kommun"), ("ta", "சொட\u{bcd}ர\u{bbe}ஜிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4b}డ\u{c4d}ర\u{c3e}జ\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "โซดาซ\u{e34}กา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}"), ("tr", "Sodrazica Eyaleti"), ("uk", "Содражиця"), ("ur", "سودرازیکا میونسپلٹی"), ("vi", "Đô thị tự trị Sodrazica"), ("zh", "索德拉日卡")]),
                        unofficial_name_list: ["Sodražica"].to_vec(),
                    }
                ),
                (
                    "180",
                    Subdivision{
                        name: "180",
                        country_alpha2: Alpha2::SI,
                        code: "180",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4200935), longitude: Some(14.6917325), max_latitude: Some(46.4311116), min_latitude: Some(46.3982247), max_longitude: Some(14.7285787), min_longitude: Some(14.6357179)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{1112e}𑄣\u{11134}𑄇𑄞"), ("ceb", "Občina Solčava"), ("cs", "Občina Solčava"), ("de", "Solčava"), ("en", "Solčava"), ("fr", "Solčava"), ("hr", "Općina Solčava"), ("it", "Solčava"), ("nl", "Solčava"), ("pl", "Gmina Solčava"), ("pt", "Solčava"), ("ro", "Solčava"), ("sl", "Občina Solčava"), ("sr", "Солчава"), ("sr_Latn", "Solčava"), ("uk", "Солчава (община)")]),
                        unofficial_name_list: ["Solcava"].to_vec(),
                    }
                ),
                (
                    "181",
                    Subdivision{
                        name: "181",
                        country_alpha2: Alpha2::SI,
                        code: "181",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.65), longitude: Some(15.845278), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سفيتا آنا"), ("bn", "সভেত\u{9be} আন\u{9be} পৌরসভ\u{9be}"), ("bs", "Sveta Ana v Slovenskih goricah"), ("ccp", "𑄥\u{11133}𑄞𑄬𑄑 𑄃𑄚"), ("ceb", "Sv. Ana v Slov. Goricah"), ("cs", "Občina Sveta Ana"), ("da", "Sveta Ana Municipality"), ("de", "Sveta Ana"), ("el", "Σβέτα Άνα"), ("en", "Sveta Ana"), ("es", "Sveta Ana"), ("fi", "Sveta Anan kunta"), ("fr", "Sveta Ana"), ("gu", "સ\u{acd}વ\u{ac7}ટ આના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}व\u{947}ता ऐना नगरपालिका"), ("hr", "Općina Sveta Ana"), ("id", "Kotamadya Sveta Ana"), ("it", "Sveta Ana"), ("ja", "スヴェタ・アナ"), ("kn", "ಸ\u{ccd}ವ\u{cc6}ಟಾ ಅನಾ ಪುರಸಭ\u{cc6}"), ("lt", "Šventosios Anos savivaldybė"), ("lv", "Sveta Anas pašvaldība"), ("mr", "स\u{94d}व\u{947}ट आना म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sveta Ana Municipality"), ("nb", "Sveta Ana kommune"), ("nl", "Sveta Ana v Slovenskih goricah"), ("no", "Sveta Ana kommune"), ("pl", "Gmina Sveta Ana"), ("pt", "Sveta Ana"), ("ro", "Sveta Ana v Slovenskih goricah"), ("ru", "Света-Ана"), ("si", "ස\u{dca}වෙට\u{dcf} අන\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Sveta Ana v Slovenskih goricah"), ("sr", "Света Ана в Словенских горицах"), ("sr_Latn", "Sveta Ana v Slovenskih goricah"), ("sv", "Sveta Ana"), ("ta", "ஸ\u{bcd}வேத\u{bbe} ஆன\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}వ\u{c47}ట\u{c3e} అన\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ซ\u{e34}วตา อนา ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Sveta Ana Belediyesi"), ("uk", "Света Ана-в-Словенських Горицах"), ("ur", "سویتا انا میونسپلٹی"), ("vi", "Sveta Ana"), ("zh", "斯洛文尼亞丘陵內斯維塔安那")]),
                        unofficial_name_list: ["Sveta Ana"].to_vec(),
                    }
                ),
                (
                    "182",
                    Subdivision{
                        name: "182",
                        country_alpha2: Alpha2::SI,
                        code: "182",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5189747), longitude: Some(15.9498262), max_latitude: Some(46.5463902), min_latitude: Some(46.490061), max_longitude: Some(16.0019974), min_longitude: Some(15.920033)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية سفيتي أندراش في سلوفينسكيخ غوريتساخ"), ("bn", "ভেতি আন\u{9cd}দ\u{9cd}রেজ ভ স\u{9cd}লোভেনস\u{9cd}কি গোরিক\u{9be}হ পৌরসভ\u{9be}"), ("bs", "Sveti Andraž v Slovenskih goricah"), ("ccp", "𑄥\u{11133}𑄞𑄬𑄑\u{11128} 𑄃𑄚\u{11134}𑄓\u{11133}𑄢𑄌\u{11134} 𑄞\u{11128} 𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134}𑄇\u{11128}𑄦\u{11134} 𑄉\u{1112e}𑄢\u{11128}𑄇𑄦\u{11134}"), ("ceb", "Občina Sveti Andraž v Slovenskih Goricah"), ("cs", "Občina Sveti Andraž v Slovenskih goricah"), ("da", "Sveti Andraž v Slovenskih Goricah Municipality"), ("de", "Sveti Andraž v Slovenskih goricah"), ("el", "Σβέτι Αντράζ"), ("en", "Sveti Andraž v Slovenskih Goricah"), ("fi", "Sveti Andraž v Slovenskih Goricahn kunta"), ("fr", "Sveti Andraž v Slovenskih goricah"), ("gu", "સ\u{acd}વ\u{ac7}ત\u{acd}તી એન\u{acd}ડરાઝ વી સ\u{acd}લોવ\u{ac7}ન\u{acd}સકિહ ગોરીકાહ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{94d}व\u{947}ती ए\u{902}ड\u{94d}राज\u{93c} वी स\u{94d}लोव\u{947}नस\u{94d}कि गोरिकाह नगरपालिका"), ("hr", "Općina Sveti Andraž v Slovenskih goricah"), ("hu", "Sveti Andraž v Slovenskih goricah"), ("id", "Kotamadya Sveti Andraž v Slovenskih Goricah"), ("it", "Sveti Andraž v Slovenskih goricah"), ("ja", "スヴェティ・アンドラジュ・フ・スロヴェンスキフ・ゴリツァフ"), ("kn", "ಸ\u{ccd}ವ\u{cc6}ಟ\u{cbf} ಆಂಡ\u{ccd}ರಜ\u{ccd} ವ\u{cbf} ಸ\u{ccd}ಲೋವ\u{cc6}ನ\u{ccd}ಸ\u{ccd}ಕ\u{cbf} ಗೋರ\u{cbf}ಕಾ ಮುನ\u{ccd}ಸ\u{cbf}ಪಾಲ\u{cbf}ಟ\u{cbf}"), ("lt", "Šventojo Andriaus Slovėnų Kalnuose savivaldybė"), ("lv", "Sveti Andražas v Slovenskih Goricah pašvaldība"), ("mr", "स\u{94d}व\u{947}ति अ\u{901}ड\u{94d}राझ वि स\u{94d}लोव\u{94d}ह\u{947}न\u{94d}स\u{94d}कीह गोरिकह म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sveti Andraz v Slovenskih Goricah Municipality"), ("nb", "Sveti Andraz v Slovenskih Goricah kommune"), ("nl", "Sveti Andraž"), ("no", "Sveti Andraz v Slovenskih Goricah kommune"), ("pl", "Gmina Sveti Andraž v Slovenskih goricah"), ("pt", "Sveti Andraž v Slovenskih goricah"), ("ro", "Sveti Andraž v Slovenskih goricah"), ("ru", "Свети-Андраж"), ("si", "ස\u{dca}වෙට\u{dd2} අන\u{dca}ද\u{dca}\u{200d}ර\u{dcf}ස\u{dca} ව\u{dd2} ස\u{dca}ලෝවෙන\u{dca}ස\u{dca}ක\u{dd2} ගොර\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Sveti Andraž v Slovenskih goricah"), ("sr", "Општина Свети Андраж в Словенских Горицах"), ("sr_Latn", "Opština Sveti Andraž v Slovenskih Goricah"), ("sv", "Sveti Andraž v Slovenskih Goricah"), ("ta", "ஸ\u{bcd}வெடி ஆண\u{bcd}ட\u{bcd}ர\u{bbe}ஸ\u{bcd} வி ஸ\u{bcd}லோவின\u{bcd}ஸ\u{bcd}கிஹ\u{bcd} குறிக\u{bcd}க\u{bbe}ஹ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c4d}వ\u{c46}ట\u{c3f} ఆండ\u{c4d}ర\u{c3e}జ\u{c4d} వ\u{c3f} స\u{c4d}ల\u{c4b}వ\u{c46}ంక\u{c3f}హ\u{c4d} గ\u{c4b}ర\u{c3f}క\u{c3e}హ\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "แคปป\u{e34}ทอล ด\u{e34}สทร\u{e34}ค"), ("tr", "Sveti Andraž v Slovenskih Goricah"), ("uk", "Светий Андраж-в-Словенських Горицях"), ("ur", "سویتی اندراز وی سلووینسکیح جوریکاح میونسپلٹی"), ("vi", "Sveti Andraž v Slovenskih goricah"), ("zh", "斯洛文尼亞丘陵內斯韋提安德拉日")]),
                        unofficial_name_list: ["Sveti Andraž v Slovenskih goricah"].to_vec(),
                    }
                ),
                (
                    "183",
                    Subdivision{
                        name: "183",
                        country_alpha2: Alpha2::SI,
                        code: "183",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.9143357), longitude: Some(13.6427256), max_latitude: Some(45.93944339999999), min_latitude: Some(45.8961874), max_longitude: Some(13.6862827), min_longitude: Some(13.6113664)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية شيمبيتر - فرتويبا"), ("bn", "সেম\u{9cd}পীট\u{9be}র-ভ\u{9cd}রত\u{9cd}রইব\u{9be} পৌরসভ\u{9be}"), ("bs", "Šempeter-Vrtojba"), ("ccp", "𑄥𑄬𑄟\u{11134}𑄛\u{11128}𑄑𑄢\u{11134}-𑄞\u{11133}𑄢𑄑\u{1112e}𑄝"), ("ceb", "Občina Šempeter-Vrtojba"), ("cs", "Občina Šempeter-Vrtojba"), ("da", "Šempeter-Vrtojba"), ("de", "Šempeter-Vrtojba"), ("el", "Σέμπετερ-Βρτότζμπα"), ("en", "Šempeter–Vrtojba"), ("es", "Šempeter-Vrtojba"), ("fi", "Šempeter–Vrtojban kunta"), ("fr", "Šempeter-Vrtojba"), ("gu", "સ\u{ac7}મ\u{acd}પ\u{ac7}ટર-વ\u{acd}ર\u{acd}ટોજ\u{acd}બા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "स\u{948}म\u{94d}पीटर-व\u{94d}रतोईबा नगरपालिका"), ("hr", "Općina Šempeter - Vrtojba"), ("hu", "Šempeter–Vrtojba község"), ("id", "Kotamadya Šempeter–Vrtojba"), ("it", "San Pietro-Vertoiba"), ("ka", "შემპეტერ-ვრტოიბა"), ("kn", "ಸ\u{cc6}ಂಪ\u{cc6}ರ\u{ccd}-ವರ\u{ccd}ಟೋಜ\u{ccd}ಬಾ ಪುರಸಭ\u{cc6}"), ("ko", "솀페테르브르토이바"), ("lt", "Šempeter-Vurtobos savivaldybė"), ("lv", "Šempeter–Vrtojbas pašvaldība"), ("mr", "स\u{947}मपीटर-वर\u{94d}तोजबा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Sempeter–Vrtojba Municipality"), ("nb", "Senoeter Vrtojba kommune"), ("nl", "Šempeter-Vrtojba"), ("no", "Senoeter Vrtojba kommune"), ("pl", "Šempeter pri Gorici"), ("pt", "Šempeter-Vrtojba"), ("ro", "Šempeter-Vrtojba"), ("ru", "Шемпетер-Вртойба"), ("si", "සෙම\u{dca}පෙටෙර\u{dca}-ව\u{dca}ර\u{dca}ටෝජ\u{dca}බ\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Šempeter - Vrtojba"), ("sr", "Општина Шемпетер - Вртојба"), ("sr_Latn", "Opština Šempeter - Vrtojba"), ("sv", "Senoeter Vrtojba kommun"), ("ta", "செம\u{bcd}பேட\u{bcd}டர\u{bcd}–வர\u{bcd}றோஜப\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "స\u{c46}ంప\u{c40}టర\u{c4d}-వర\u{c4d}ట\u{c4b}జ\u{c4d}బ\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เซมป\u{e35}เตอ วาทอปจ\u{e4c}"), ("tr", "Sepmeter-Vrtojba Belediyesi"), ("uk", "Шемпетер-Вртойба"), ("vi", "Šempeter-Vrtojba"), ("zh", "塞姆皮特-普羅特伊巴")]),
                        unofficial_name_list: ["Šempeter-Vrtojba"].to_vec(),
                    }
                ),
                (
                    "184",
                    Subdivision{
                        name: "184",
                        country_alpha2: Alpha2::SI,
                        code: "184",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2346683), longitude: Some(15.0167331), max_latitude: Some(46.2422044), min_latitude: Some(46.2247663), max_longitude: Some(15.0280818), min_longitude: Some(15.0014522)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية تابور"), ("bn", "ত\u{9be}বোর পৌরসভ\u{9be}"), ("bs", "Tabor"), ("ccp", "𑄑𑄝\u{1112e}𑄢\u{11134}"), ("ceb", "Tabor"), ("cs", "Občina Tabor"), ("da", "Tabor Municipality"), ("de", "Tabor"), ("el", "Ταμπόρ"), ("en", "Tabor"), ("es", "Tabor"), ("fi", "Taborin kunta"), ("fr", "Tabor"), ("gu", "ટાબોર મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "टाबोर नगरपालिका"), ("hr", "Općina Tabor"), ("id", "Kotamadya Tabor"), ("it", "Tabor"), ("ja", "ターボル"), ("kn", "ಟ\u{ccd}ಯಾಬರ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "타보르"), ("lt", "Taboro savivaldybė"), ("lv", "Taboras pašvaldība"), ("mr", "टोबर म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Tabor Municipality"), ("nb", "Tabor kommune"), ("nl", "Tabor"), ("no", "Tabor kommune"), ("pl", "Gmina Tabor"), ("pt", "Tabor"), ("ro", "Tabor"), ("ru", "Табор"), ("si", "ටබෝර\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Tabor, Tabor"), ("sr", "Општина Табор"), ("sr_Latn", "Opština Tabor"), ("sv", "Tabor kommun"), ("ta", "டப\u{bbe}ற\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c3e}బ\u{c4b}ర\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ทาบอร\u{e4c} ม\u{e39}น\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Tabor Belediyesi"), ("uk", "Табор (Табор)"), ("ur", "تابور میونسپلٹی"), ("vi", "Đô thị tự trị Tabor"), ("zh", "塔博爾")]),
                        unofficial_name_list: ["Tabor"].to_vec(),
                    }
                ),
                (
                    "185",
                    Subdivision{
                        name: "185",
                        country_alpha2: Alpha2::SI,
                        code: "185",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5201168), longitude: Some(15.8866431), max_latitude: Some(46.5376071), min_latitude: Some(46.5070018), max_longitude: Some(15.9255082), min_longitude: Some(15.8799778)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية ترنوفسكا فاس"), ("bn", "টর\u{9cd}নোভস\u{9cd}ক\u{9be} ভ\u{9be}স পৌরসভ\u{9be}"), ("ccp", "𑄚\u{1112e}𑄛\u{11134}𑄇 𑄞𑄌\u{11134}"), ("ceb", "Občina Trnovska vas"), ("cs", "Občina Trnovska vas"), ("da", "Trnovska Vas Municipality"), ("de", "Trnovska vas"), ("el", "Τρνόβσκα Βας"), ("en", "Trnovska Vas"), ("es", "Trnovska vas"), ("fi", "Trnovska Vasin kunta"), ("fr", "Trnovska vas"), ("gu", "ટ\u{acd}રનોવ\u{acd}સકા વાસ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ट\u{94d}र\u{94d}नोवस\u{94d}का वास नगर पालिका"), ("hr", "Općina Trnovska vas"), ("id", "Kotamadya Trnovska Vas"), ("it", "Trnovska vas"), ("ja", "トルノフスカ・ヴァス"), ("kn", "ಟ\u{ccd}ರುನೋವ\u{ccd}ಸ\u{ccd}ಕಾ ವಾಸ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "트르노브스카바스"), ("lt", "Trnovska Vaso savivaldybė"), ("lv", "Trnovska Vasas pašvaldība"), ("mr", "ट\u{94d}र\u{94d}नोवस\u{94d}का वास म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Trnovska Vas Municipality"), ("nb", "Trnovska Vas kommune"), ("nl", "Trnovska vas"), ("no", "Trnovska Vas kommune"), ("pl", "Gmina Trnovska vas"), ("pt", "Trnovska vas"), ("ro", "Trnovska vas"), ("ru", "Трновска-Вас"), ("si", "ට\u{dca}\u{200d}රොනොව\u{dca}ස\u{dca}ක\u{dcf} ව\u{dcf}ස\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Trnovska vas"), ("sr", "Општина Трновска Вас"), ("sr_Latn", "Opština Trnovska Vas"), ("sv", "Trnovska Vas kommun"), ("ta", "ற\u{bcd}றனொவ\u{bcd}ஸ\u{bcd}க வ\u{bbe}ஸ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ట\u{c4d}ర\u{c4b}న\u{c4b}వస\u{c4d}క\u{c3e} వ\u{c3e}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องทโนฟสกา วาส"), ("tr", "Trnovska Vas Belediyesi"), ("uk", "Трновська Вас"), ("ur", "ترنووسکا واس میونسپلٹی"), ("vi", "Trnovska Vas")]),
                        unofficial_name_list: ["Trnovska vas"].to_vec(),
                    }
                ),
                (
                    "186",
                    Subdivision{
                        name: "186",
                        country_alpha2: Alpha2::SI,
                        code: "186",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1363399), longitude: Some(14.5616878), max_latitude: Some(46.145734), min_latitude: Some(46.1095535), max_longitude: Some(14.5830211), min_longitude: Some(14.520235)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ترزين"), ("bn", "ত\u{9cd}রিন"), ("bs", "Trzin"), ("ccp", "𑄎\u{11128}𑄚\u{11134}"), ("ceb", "Trzin"), ("cs", "Občina Trzin"), ("da", "Trzin"), ("de", "Trzin"), ("el", "Τρζίν"), ("en", "Trzin"), ("es", "Trzin"), ("fi", "Trzin"), ("fr", "Trzin"), ("gu", "ટ\u{acd}રઝિન"), ("hi", "त\u{94d}रज\u{93c}िन"), ("hr", "Općina Trzin"), ("id", "Trzin"), ("it", "Trzin"), ("ja", "トルジン"), ("kn", "ಟ\u{ccd}ರ\u{ccd}ಜ\u{cbf}ನ\u{ccd}"), ("ko", "트르진"), ("lt", "Trzinas"), ("lv", "Trzina"), ("mr", "ट\u{94d}रझिन"), ("ms", "Trzin"), ("nb", "Trzin"), ("nl", "Trzin"), ("no", "Trzin"), ("pl", "Gmina Trzin"), ("pt", "Trzin"), ("ro", "Trzin"), ("ru", "Трзин"), ("si", "ට\u{dca}ර\u{dca}ස\u{dd2}න\u{dca}"), ("sl", "Trzin"), ("sr", "Трзин"), ("sr_Latn", "Trzin"), ("sv", "Trzin"), ("ta", "டிரஜின\u{bcd}"), ("te", "ట\u{c4d}ర\u{c3f}జ\u{c3f}న\u{c4d}"), ("th", "ทรซ\u{e34}น"), ("tr", "Trzin"), ("uk", "Трзин"), ("ur", "طرزیں"), ("vi", "Trzin"), ("zh", "特尔津")]),
                        unofficial_name_list: ["Trzin"].to_vec(),
                    }
                ),
                (
                    "187",
                    Subdivision{
                        name: "187",
                        country_alpha2: Alpha2::SI,
                        code: "187",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5731715), longitude: Some(16.3444126), max_latitude: Some(46.5874722), min_latitude: Some(46.55340229999999), max_longitude: Some(16.3761691), min_longitude: Some(16.3215055)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فيليكا بولانا"), ("bn", "কোমেন\u{9cd}ড\u{9be} পৌরসভ\u{9be}²"), ("ccp", "𑄞𑄬𑄣\u{11128}𑄇 𑄛\u{1112a}𑄣𑄚"), ("ceb", "Velika Polana"), ("cs", "Občina Velika Polana"), ("da", "Velika Polana Municipality"), ("de", "Velika Polana"), ("el", "Βελίκα Πολάνα"), ("en", "Velika Polana"), ("es", "Municipalidad de Vekila Polana"), ("fi", "Velika Polanan kunta"), ("fr", "Velika Polana"), ("gu", "વ\u{ac7}લિકા પોલાના મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{947}लिका पोलाना नगर पालिका"), ("hr", "Općina Velika Polana"), ("hu", "Nagypalina"), ("id", "Kotamadya Velika Polana"), ("it", "Velika Polana"), ("ja", "ヴェリカ・ポラナ"), ("kn", "ವ\u{cc6}ಲ\u{cbf}ಕಾ ಪೋಲಾನಾ ಪುರಸಭ\u{cc6}"), ("ko", "벨리카폴라나"), ("lt", "Velika Polanos savivaldybė"), ("lv", "Velika Polanas pašvaldība"), ("mr", "व\u{947}लिका पोलन म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Velika Polana Municipality"), ("nb", "Velika Polana kommune"), ("nl", "Velika Polana"), ("no", "Velika Polana kommune"), ("pl", "Gmina Velika Polana"), ("pt", "Velika Polana"), ("ro", "Velika Polana"), ("ru", "Велика-Плана"), ("si", "වෙල\u{dd2}ක\u{dcf} පොලන\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Velika Polana"), ("sr", "Општина Велика Полана"), ("sr_Latn", "Opština Velika Polana"), ("sv", "Velika Polana (kommun)"), ("ta", "வெலிக\u{bcd}க போல\u{bbe}ன நகர\u{bbe}ட\u{bcd}சி"), ("te", "వ\u{c46}ల\u{c3f}క\u{c3e} ప\u{c4b}ల\u{c3e}న\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เวล\u{e34}กา โพลานา"), ("tr", "Velika Polana Belediyesi"), ("uk", "Велика Полана"), ("ur", "ویلیکا پولانا میونسپلٹی"), ("vi", "Velika Polana"), ("zh", "韋利卡波拉那")]),
                        unofficial_name_list: ["Velika Polana"].to_vec(),
                    }
                ),
                (
                    "188",
                    Subdivision{
                        name: "188",
                        country_alpha2: Alpha2::SI,
                        code: "188",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5832778), longitude: Some(16.1640173), max_latitude: Some(46.5977089), min_latitude: Some(46.5649081), max_longitude: Some(16.2007696), min_longitude: Some(16.1504664)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية فيرزي"), ("bn", "ভ\u{9be}র\u{9cd}জেস পৌরসভ\u{9be}"), ("ccp", "𑄞𑄢\u{11134}𑄥𑄬𑄌\u{11134}"), ("ceb", "Občina Veržej"), ("cs", "Občina Veržej"), ("da", "Veržej"), ("de", "Veržej"), ("el", "Βέρζετζ"), ("en", "Veržej"), ("es", "Veržej"), ("fi", "Veržejin kunta"), ("fr", "Veržej"), ("gu", "વર\u{acd}ઝ\u{ac7}જ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "व\u{947}र\u{94d}ज\u{947}ज नगर पालिका"), ("hr", "Općina Veržej"), ("id", "Kotamadya Veržej"), ("it", "Veržej"), ("ja", "ヴェルジェイ"), ("kn", "ವ\u{cc6}ರ\u{ccd}ಜ\u{cc6}ಜ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "베르제이"), ("lt", "Veržėjaus savivaldybė"), ("lv", "Veržejas pašvaldība"), ("mr", "व\u{94d}ह\u{947}र\u{94d}झ\u{947}ज म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Verzej Municipality"), ("nb", "Verzej kommune"), ("nl", "Veržej"), ("no", "Verzej kommune"), ("pl", "Gmina Veržej"), ("pt", "Veržej"), ("ro", "Veržej"), ("ru", "Вержей"), ("si", "වර\u{dca}සේජ\u{dca} නගර සභ\u{dcf}ව"), ("sl", "Občina Veržej"), ("sr", "Општина Вержеј"), ("sr_Latn", "Opština Veržej"), ("sv", "Verzej kommun"), ("ta", "வேரஸிஜ\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "వర\u{c4d}జ\u{c46}స\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องเวอร\u{e4c}เจซ"), ("tr", "Verzej Belediyesi"), ("uk", "Вержей"), ("ur", "ویرزیج میونسپلٹی"), ("vi", "Veržej"), ("zh", "韋爾熱")]),
                        unofficial_name_list: ["Veržej"].to_vec(),
                    }
                ),
                (
                    "189",
                    Subdivision{
                        name: "189",
                        country_alpha2: Alpha2::SI,
                        code: "189",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2444682), longitude: Some(14.9511982), max_latitude: Some(46.2541176), min_latitude: Some(46.2306325), max_longitude: Some(14.9643893), min_longitude: Some(14.9328115)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فرانسكو"), ("bn", "ভ\u{9cd}র\u{9be}নস\u{9cd}কো"), ("bs", "Vransko"), ("ccp", "𑄞\u{11133}𑄢𑄚\u{11134}𑄇\u{1112e}"), ("ceb", "Vransko (kapital sa munisipyo)"), ("cs", "Občina Vransko"), ("da", "Vransko"), ("de", "Vransko"), ("el", "Βράνσκο"), ("en", "Vransko"), ("es", "Vransko"), ("fi", "Vransko"), ("fr", "Vransko"), ("gu", "વ\u{acd}રાન\u{acd}સ\u{acd}કો"), ("hi", "व\u{94d}रान\u{94d}सको"), ("hr", "Općina Vransko"), ("id", "Vransko"), ("it", "Vransko"), ("ja", "ヴランスコ"), ("kn", "ವ\u{ccd}ಯಾನ\u{ccd}ಸ\u{ccd}ಕೊ"), ("ko", "브란스코"), ("lt", "Vranskas"), ("lv", "Vransko"), ("mr", "व\u{94d}रान\u{94d}स\u{94d}को"), ("ms", "Vransko"), ("nb", "Vransko"), ("nl", "Vransko"), ("no", "Vransko"), ("pl", "Gmina Vransko"), ("pt", "Vransko"), ("ro", "Comuna Vransko"), ("ru", "Вранско"), ("si", "ව\u{dd2}ර\u{dcf}න\u{dca}ස\u{dca}කෝ"), ("sl", "Občina Vransko"), ("sr", "Општина Вранско"), ("sr_Latn", "Opština Vransko"), ("sv", "Vransko"), ("ta", "வரன\u{bcd}ஸ\u{bcd}க\u{bcd}கோ"), ("te", "వ\u{c4d}ర\u{c3e}న\u{c4d}స\u{c4d}క\u{c4b}"), ("th", "วรานสโก"), ("tr", "Vransko"), ("uk", "Врансько (община)"), ("ur", "ورانسکو"), ("vi", "Vransko"), ("zh", "弗蘭斯科")]),
                        unofficial_name_list: ["Vransko"].to_vec(),
                    }
                ),
                (
                    "190",
                    Subdivision{
                        name: "190",
                        country_alpha2: Alpha2::SI,
                        code: "190",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.251984), longitude: Some(15.1650282), max_latitude: Some(46.2575649), min_latitude: Some(46.2355865), max_longitude: Some(15.1804589), min_longitude: Some(15.1406234)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جاليتس"), ("bg", "Жалец"), ("bn", "জ\u{9be}লেক পৌরসভ\u{9be}"), ("bs", "Žalec"), ("ca", "Žalec"), ("ccp", "𑄎𑄣𑄬𑄇\u{11134}"), ("ceb", "Občina Žalec"), ("cs", "Žalec"), ("da", "Žalec Municipality"), ("de", "Žalec"), ("el", "Ζάλεκ"), ("en", "Žalec"), ("es", "Žalec"), ("fa", "ژالتس"), ("fi", "Žalecin kunta"), ("fr", "Žalec"), ("gu", "ઝાલ\u{ac7}ક મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}ल\u{947}क नगर पालिका"), ("hr", "Općina Žalec"), ("id", "Kotamadya Žalec"), ("it", "Žalec"), ("ja", "ジャレツ"), ("kn", "ಲ\u{ccd}ಲೇಕ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "잘레츠"), ("lt", "Žaleco savivaldybė"), ("lv", "Žalecas pašvaldība"), ("mr", "झाल\u{947}च म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zalec Municipality"), ("nb", "Zalec kommune"), ("nl", "Žalec"), ("no", "Zalec kommune"), ("pl", "Gmina Žalec"), ("pt", "Žalec"), ("ro", "Žalec"), ("ru", "Жалец"), ("si", "සලෙක\u{dca} නගර සභ\u{dcf}ව"), ("sk", "Žalec"), ("sl", "Občina Žalec"), ("sr", "Општина Жалец"), ("sr_Latn", "Opština Žalec"), ("sv", "Zalec kommun"), ("ta", "ச\u{bbe}லெக\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c3e}ల\u{c46}క\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เทศบาลเม\u{e37}องซาเลก"), ("tr", "Zalec Belediyesi"), ("uk", "Жалець (община)"), ("ur", "زالیک میونسپلٹی"), ("vi", "Žalec"), ("zh", "扎萊茨")]),
                        unofficial_name_list: ["Žalec"].to_vec(),
                    }
                ),
                (
                    "191",
                    Subdivision{
                        name: "191",
                        country_alpha2: Alpha2::SI,
                        code: "191",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2784002), longitude: Some(15.8122225), max_latitude: Some(46.290547), min_latitude: Some(46.2592789), max_longitude: Some(15.8683349), min_longitude: Some(15.7860413)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جيتيل"), ("bn", "জেত\u{9be}ল পৌরসভ\u{9be}"), ("ccp", "𑄎𑄑𑄣𑄬"), ("ceb", "Občina Žetale"), ("cs", "Občina Žetale"), ("da", "Žetale Municipality"), ("de", "Žetale"), ("el", "Ζετάλε"), ("en", "Žetale"), ("es", "Minicipalidad del Žetale"), ("fi", "Žetalen kunta"), ("fr", "Žetale"), ("gu", "ઝ\u{ac7}ટલ\u{ac7} મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}\u{947}ताल\u{947} नगरपालिका"), ("hr", "Općina Žetale"), ("id", "Kotamadya Žetale"), ("it", "Žetale"), ("ja", "ジェタレ"), ("kn", "ಝೀಟ\u{cc6}ಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "제탈레"), ("lt", "Žetalės savivaldybė"), ("lv", "Žetales pašvaldība"), ("mr", "झ\u{947}टल\u{947} म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zetale Municipality"), ("nb", "Zetale kommune"), ("nl", "Žetale"), ("no", "Zetale kommune"), ("pl", "Gmina Žetale"), ("pt", "Žetale"), ("ro", "Žetale"), ("ru", "Жетале"), ("si", "සේටලේ නගර සභ\u{dcf}ව"), ("sl", "Občina Žetale"), ("sr", "Општина Жетале"), ("sr_Latn", "Opština Žetale"), ("sv", "Žetale"), ("ta", "ஸிடல\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జ\u{c46}ట\u{c3e}ల\u{c46} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "เม\u{e37}องซ\u{e35}เทล"), ("tr", "Zelate Belediyesi"), ("uk", "Жетале"), ("ur", "زیتالی میونسپلٹی"), ("vi", "Žetale"), ("zh", "熱塔雷")]),
                        unofficial_name_list: ["Žetale"].to_vec(),
                    }
                ),
                (
                    "192",
                    Subdivision{
                        name: "192",
                        country_alpha2: Alpha2::SI,
                        code: "192",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4044239), longitude: Some(14.1375549), max_latitude: Some(46.40837459999999), min_latitude: Some(46.3947787), max_longitude: Some(14.141806), min_longitude: Some(14.1331969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية جيروفنيتسا"), ("bn", "জিরোভনিক\u{9be} পৌরসভ\u{9be}"), ("bs", "Žirovnica"), ("ccp", "𑄎\u{11128}𑄢\u{1112e}𑄛\u{11134}𑄚\u{11128}𑄇"), ("ceb", "Občina Žirovnica (munisipyo sa Eslobenya)"), ("cs", "Občina Žirovnica"), ("da", "Žirovnica Municipality"), ("de", "Žirovnica"), ("el", "Ζιροβνίκα"), ("en", "Žirovnica"), ("es", "Žirovnica"), ("fi", "Žirovnican kunta"), ("fr", "Žirovnica (Slovénie)"), ("gu", "ઝીરોનિકા મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "ज\u{93c}िरोवनिका नगरपालिका"), ("hr", "Općina Žirovnica"), ("id", "Kotamadya Žirovnica"), ("it", "Žirovnica"), ("ja", "ジロヴニツァ"), ("kn", "ಸ\u{cbf}ರೊನ\u{cbf}ಕಾ ಪುರಸಭ\u{cc6}"), ("ko", "지로브니차"), ("lt", "Žirovnicos savivaldybė"), ("lv", "Žirovnicas pašvaldība"), ("mr", "झिरोवनीचा म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Zirovnica Municipality"), ("nb", "Zirovnica kommune"), ("nl", "Žirovnica"), ("no", "Zirovnica kommune"), ("pl", "Gmina Žirovnica"), ("pt", "Žirovnica"), ("ro", "Žirovnica"), ("ru", "Жировница"), ("si", "ස\u{dd2}රෝව\u{dca}න\u{dd2}ක\u{dcf} නගර සභ\u{dcf}ව"), ("sl", "Občina Žirovnica"), ("sr", "Општина Жировница"), ("sr_Latn", "Opština Žirovnica"), ("sv", "Zirovnica kommun"), ("ta", "சிரோவனிக\u{bcd}க\u{bbe} நகர\u{bbe}ட\u{bcd}சி"), ("te", "జర\u{c3f}\u{c4b}వ\u{c4d}న\u{c3f}క\u{c3e} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "ซ\u{e34}รอฟน\u{e34}ชา"), ("tr", "Zirovnica Belediyesi"), ("uk", "Жировніца"), ("ur", "زیروونیکا میونسپلٹی"), ("vi", "Žirovnica"), ("zh", "日羅夫尼察")]),
                        unofficial_name_list: ["Žirovnica"].to_vec(),
                    }
                ),
                (
                    "193",
                    Subdivision{
                        name: "193",
                        country_alpha2: Alpha2::SI,
                        code: "193",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.830669), longitude: Some(14.9298228), max_latitude: Some(45.8477587), min_latitude: Some(45.8019543), max_longitude: Some(14.9495254), min_longitude: Some(14.9077541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جوجيمبيرك"), ("bg", "Жужемберк"), ("bn", "জ\u{9c1}জেমব\u{9be}র\u{9cd}ক"), ("bs", "Žužemberk"), ("ccp", "𑄎\u{1112a}𑄎𑄬𑄟\u{11134}𑄝𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Občina Žužemberk"), ("cs", "Občina Žužemberk"), ("da", "Žužemberk"), ("de", "Žužemberk"), ("el", "Ζουζέμπερκ"), ("en", "Žužemberk"), ("es", "Žužemberk"), ("fi", "Žužemberk"), ("fr", "Žužemberk"), ("gu", "ઝ\u{ac1}ઝ\u{ac7}મબર\u{acd}ક"), ("hi", "ज\u{93c}\u{941}ज\u{93c}\u{947}मबर\u{94d}क"), ("hr", "Općina Žužemberk"), ("hu", "Žužemberk"), ("id", "Žužemberk"), ("it", "Žužemberk"), ("ja", "ジュジェンベルク"), ("kn", "ಜುಝ\u{cc6}ಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "주젬베르크"), ("lt", "Žužemberkas"), ("lv", "Žužemberka"), ("mr", "झ\u{941}झ\u{947}मबर\u{94d}क"), ("ms", "Zuzemberk"), ("nb", "Zuzenberk"), ("nl", "Žužemberk"), ("no", "Zuzenberk"), ("pl", "Gmina Žužemberk"), ("pt", "Žužemberk"), ("ro", "Comuna Žužemberk"), ("ru", "Жужемберк"), ("si", "ස\u{dd4}සේම\u{dca}බර\u{dca}ක\u{dca}"), ("sl", "Občina Žužemberk"), ("sr", "Општина Жужемберк"), ("sr_Latn", "Opština Žužemberk"), ("sv", "Žužemberk"), ("ta", "ஸுஎம\u{bcd}பேர\u{bcd}க\u{bcd}"), ("te", "జుజ\u{c46}ంబర\u{c4d}క\u{c4d}"), ("th", "ซ\u{e39}เซมบาร\u{e4c}ค"), ("tr", "Zuzembark"), ("uk", "Жужемберк"), ("ur", "زوزیمبیرک"), ("vi", "Žužemberk"), ("zh", "祖熱姆布爾克")]),
                        unofficial_name_list: ["Žužemberk"].to_vec(),
                    }
                ),
                (
                    "194",
                    Subdivision{
                        name: "194",
                        country_alpha2: Alpha2::SI,
                        code: "194",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0457437), longitude: Some(14.8410058), max_latitude: Some(46.0538612), min_latitude: Some(46.0301017), max_longitude: Some(14.8583307), min_longitude: Some(14.8248383)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سمارتنو بري ليتييه"), ("bn", "স\u{9cd}ম\u{9be}র\u{9cd}ত প\u{9cd}রি লিতি"), ("bs", "Šmartno pri Litiji"), ("ccp", "𑄥\u{11133}𑄟𑄢\u{11134}𑄑\u{11134}𑄚\u{1112e} 𑄛\u{11133}𑄢\u{1112d} 𑄣\u{11128}𑄑\u{11128}𑄎\u{11128}"), ("ceb", "Občina Šmartno pri Litiji"), ("cs", "Občina Šmartno pri Litiji"), ("da", "Šmartno pri Litiji"), ("de", "Šmartno pri Litiji"), ("el", "Σμάρντο πρι Λιτίτζι"), ("en", "Šmartno pri Litiji"), ("es", "Šmartno pri Litiji"), ("fi", "Šmartno pri Litiji"), ("fr", "Šmartno pri Litiji"), ("gu", "લક\u{acd}ષદ\u{acd}વીપ"), ("hi", "स\u{94d}मार\u{94d}टनो प\u{94d}री लितिजी"), ("hr", "Općina Šmartno pri Litiji"), ("id", "Šmartno pri Litiji"), ("it", "Šmartno pri Litiji"), ("ja", "シュマルトノ・プリ・リティイ"), ("kn", "ಸ\u{ccd}ರ\u{ccd}ಮಾರ\u{ccd}ಟ\u{ccd}ನ ಪ\u{ccd}ರ\u{cbf}ಯ ಲ\u{cbf}ಟ\u{cbf}ಜ\u{cbf}"), ("ko", "슈마르트노프리리티이"), ("lt", "Šmartno pri Litidžis"), ("lv", "Šmartno Pri Litiji"), ("mr", "स\u{94d}मरर\u{94d}टो प\u{94d}र\u{947} लीटीज"), ("ms", "Smartno pri Litiji"), ("nb", "Smartno pri Litjii"), ("nl", "Šmartno pri Litiji"), ("no", "Smartno pri Litjii"), ("pl", "Gmina Šmartno pri Litiji"), ("pt", "Šmartno pri Litiji"), ("ro", "Comuna Šmartno pri Litiji"), ("ru", "Шмартно-при-Литии"), ("si", "ස\u{dca}ම\u{dcf}ර\u{dca}ට\u{dca}නෝ ප\u{dca}\u{200d}ර\u{dd2} ල\u{dd2}ට\u{dd2}ජ\u{dd2}"), ("sl", "Občina Šmartno pri Litiji"), ("sr", "Општина Шмартно при Литији"), ("sr_Latn", "Opština Šmartno pri Litiji"), ("sv", "Smartno pri Litjii"), ("ta", "ஸ\u{bcd}ம\u{bbe}ர\u{bcd}ட\u{bcd}னோ பிரி லிடிஜி"), ("te", "స\u{c4d}మ\u{c3e}ర\u{c4d}ట\u{c4d}న\u{c4b} ప\u{c4d}ర\u{c3f} ల\u{c3f}ట\u{c3f}జ\u{c3f}"), ("th", "สมาร\u{e4c}ทโน พร\u{e34} ล\u{e34}ท\u{e34}จ\u{e34}"), ("tr", "Šmartno pri Litiji"), ("uk", "Шмартно-при-Літії (община)"), ("ur", "سمارتنو پری لیتیجی"), ("vi", "Šmartno pri Litiji"), ("zh", "利蒂伊附近什馬爾特諾")]),
                        unofficial_name_list: ["Šmartno pri Litiji"].to_vec(),
                    }
                ),
                (
                    "195",
                    Subdivision{
                        name: "195",
                        country_alpha2: Alpha2::SI,
                        code: "195",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄛\u{1112d}𑄌\u{11134}"), ("ceb", "Občina Apače"), ("cs", "Občina Apače"), ("de", "Apače"), ("en", "Apače"), ("es", "Municipio de Apače"), ("hr", "Općina Apače"), ("nl", "Apače"), ("pl", "Gmina Apače"), ("ro", "Comuna Apače"), ("ru", "Апаче"), ("sl", "Občina Apače"), ("sr", "Општина Апаче"), ("sr_Latn", "Opština Apače"), ("uk", "Апаче"), ("ur", "بلدیہ آپاچے"), ("zh", "阿帕切鎮")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "196",
                    Subdivision{
                        name: "196",
                        country_alpha2: Alpha2::SI,
                        code: "196",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄢\u{11134}𑄇\u{1112a}𑄣𑄚\u{11134}"), ("ceb", "Cirkulane (munisipyo)"), ("cs", "Občina Cirkulane"), ("de", "Cirkulane"), ("en", "Cirkulane"), ("hr", "Općina Cirkulane"), ("nl", "Cirkulane"), ("pl", "Gmina Cirkulane"), ("ro", "Comuna Cirkulane"), ("sl", "Občina Cirkulane"), ("uk", "Циркулане"), ("ur", "بلدیہ تسیرکولانے"), ("zh", "齊爾庫拉內鎮")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "197",
                    Subdivision{
                        name: "197",
                        country_alpha2: Alpha2::SI,
                        code: "197",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{1112e}𑄌\u{11134}𑄑𑄚\u{11134}𑄎𑄬𑄞\u{11128}𑄇 𑄚 𑄇\u{11128}"), ("ceb", "Kostanjevica na Krki (munisipyo)"), ("cs", "Občina Kostanjevica na Krki"), ("en", "Kostanjevica na Krki"), ("hr", "Općina Kostanjevica na Krki"), ("it", "Kostanjevica na Krki"), ("nl", "Kostanjevica na Krki"), ("pl", "Gmina Kostanjevica na Krki"), ("ro", "Comuna Kostanjevica na Krki"), ("sl", "Občina Kostanjevica na Krki"), ("sr", "Општина Костањевица на Крки"), ("sr_Latn", "Opština Kostanjevica na Krki"), ("uk", "Костанєвиця-на-Кркі (община)"), ("zh", "科爾基地區科斯坦耶維察")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "198",
                    Subdivision{
                        name: "198",
                        country_alpha2: Alpha2::SI,
                        code: "198",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟𑄇\u{1112e}𑄣\u{11128}"), ("ceb", "Makole"), ("cs", "Občina Makole"), ("de", "Makole"), ("en", "Makole"), ("fr", "Makole"), ("hr", "Općina Makole"), ("it", "Makole"), ("ja", "マコレ"), ("ko", "마콜레"), ("nl", "Makole"), ("pl", "Gmina Makole"), ("pt", "Makole"), ("ro", "Comuna Makole"), ("sl", "Občina Makole"), ("uk", "Маколе"), ("vi", "Makole"), ("zh", "馬科萊")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "199",
                    Subdivision{
                        name: "199",
                        country_alpha2: Alpha2::SI,
                        code: "199",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Mokronog-Trebelno"), ("ccp", "𑄟\u{11127}𑄇\u{11134}𑄢\u{1112e}𑄚\u{11127}𑄇\u{11134}-𑄑\u{11133}𑄢𑄬𑄝𑄬𑄣\u{11134}𑄚\u{1112e}"), ("ceb", "Mokronog-Trebelno"), ("cs", "Občina Mokronog-Trebelno"), ("de", "Mokronog-Trebelno"), ("en", "Mokronog–Trebelno"), ("fr", "Mokronog-Trebelno"), ("hr", "Općina Mokronog - Trebelno"), ("it", "Mokronog-Trebelno"), ("ko", "모크로노그트레벨노"), ("nl", "Mokronog-Trebelno"), ("pl", "Gmina Mokronog-Trebelno"), ("pt", "Mokronog-Trebelno"), ("ro", "Comuna Mokronog-Trebelno"), ("ru", "община Мокроног-Требелно"), ("sl", "Občina Mokronog - Trebelno"), ("uk", "Мокроног-Требелно"), ("vi", "Mokronog-Trebelno"), ("zh", "摩克隆諾格-特雷別諾")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "200",
                    Subdivision{
                        name: "200",
                        country_alpha2: Alpha2::SI,
                        code: "200",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{11127}𑄣\u{11134}𑄇𑄬𑄚\u{11134}"), ("ceb", "Občina Poljčane"), ("cs", "Občina Poljčane"), ("de", "Poljčane"), ("en", "Poljčane"), ("fi", "Poljčane"), ("fr", "Poljčane"), ("hr", "Općina Poljčane"), ("it", "Poljčane"), ("ja", "ポリチャネ"), ("ko", "폴차네"), ("nl", "Poljčane"), ("pl", "Gmina Poljčane"), ("pt", "Poljčane"), ("ro", "Comuna Poljčane"), ("sl", "Občina Poljčane"), ("uk", "Польчане"), ("vi", "Poljčane"), ("zh", "波爾伊察涅")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "201",
                    Subdivision{
                        name: "201",
                        country_alpha2: Alpha2::SI,
                        code: "201",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Renče-Vogrsko"), ("ccp", "𑄢𑄬𑄚\u{11134}-𑄞\u{1112e}𑄇\u{11134}𑄢\u{11134}𑄇\u{1112e}"), ("ceb", "Občina Renče-Vogrsko"), ("cs", "Občina Renče-Vogrsko"), ("de", "Renče-Vogrsko"), ("en", "Renče–Vogrsko"), ("fr", "Renče-Vogrsko"), ("hr", "Općina Renče - Vogrsko"), ("hu", "Renče - Vogrsko"), ("it", "Ranziano-Voghersca"), ("ko", "렌체보그르스코"), ("nl", "Renče-Vogrsko"), ("pl", "Gmina Renče-Vogrsko"), ("pt", "Renče-Vogrsko"), ("ro", "Comuna Renče-Vogrsko"), ("sl", "Občina Renče - Vogrsko"), ("uk", "Ренче-Вогрско"), ("vi", "Renče-Vogrsko"), ("zh", "倫切-沃格斯科")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "202",
                    Subdivision{
                        name: "202",
                        country_alpha2: Alpha2::SI,
                        code: "202",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Središče ob Dravi"), ("ccp", "𑄥\u{11133}𑄢𑄬𑄓\u{11128}𑄌\u{11134}𑄥\u{11134} 𑄃\u{11127}𑄛\u{11134} 𑄓\u{11133}𑄢𑄞\u{11128}"), ("ceb", "Občina Središče ob Dravi"), ("cs", "Občina Središče ob Dravi"), ("de", "Središče ob Dravi"), ("en", "Središče ob Dravi"), ("fr", "Središče ob Dravi"), ("hr", "Općina Središče ob Dravi"), ("it", "Središče ob Dravi"), ("ja", "スレディシュチェ・オブ・ドラヴィ"), ("ko", "스레디슈체오브드라비"), ("nl", "Središče ob Dravi"), ("pl", "Gmina Središče ob Dravi"), ("pt", "Središče ob Dravi"), ("ro", "Comuna Središče ob Dravi"), ("sl", "Občina Središče ob Dravi"), ("sr", "Општина Средишче об Драви"), ("sr_Latn", "Opština Središče ob Dravi"), ("uk", "Средище-об-Драві"), ("vi", "Središče ob Dravi"), ("zh", "德拉維河畔斯雷蒂什采")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "203",
                    Subdivision{
                        name: "203",
                        country_alpha2: Alpha2::SI,
                        code: "203",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄌\u{11133}𑄑𑄢𑄎"), ("ceb", "Občina Straža"), ("cs", "Občina Straža"), ("de", "Straža"), ("en", "Straža"), ("fr", "Straža"), ("hr", "Općina Straža"), ("it", "Straža"), ("ja", "ストラジャ"), ("ko", "스트라자"), ("nl", "Straža"), ("pl", "Gmina Straža"), ("pt", "Straža"), ("ro", "Comuna Straža"), ("ru", "Стража"), ("sl", "Občina Straža"), ("uk", "Стража"), ("zh", "史特拉札")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "204",
                    Subdivision{
                        name: "204",
                        country_alpha2: Alpha2::SI,
                        code: "204",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{11133}𑄞𑄬𑄑 𑄑\u{11133}𑄢\u{1112e}𑄎\u{11128}𑄇 𑄞\u{11128} 𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134}𑄇\u{11128}𑄦\u{11134} 𑄉\u{1112e}𑄢\u{11128}𑄇𑄦\u{11134}"), ("ceb", "Sveta Trojica v Slovenskih Goricah"), ("cs", "Občina Sveta Trojica v Slovenskih goricah"), ("de", "Sveta Trojica v Slovenskih goricah"), ("en", "Sveta Trojica v Slovenskih Goricah"), ("fr", "Sveta Trojica v Slovenskih goricah"), ("hr", "Općina Sveta Trojica v Slovenskih goricah"), ("hu", "Sveta Trojica v Slovenskih goricah"), ("it", "Sveta Trojica v Slovenskih goricah"), ("ja", "スヴェタ・トロイツァ・ウ・スロヴェンスキフ・ゴリツァフ"), ("ko", "스베타트로이차브슬로벤스키흐고리차흐"), ("nl", "Sveta Trojica v Slovenskih goricah"), ("pl", "Gmina Sveta Trojica v Slovenskih goricah"), ("pt", "Sveta Trojica v Slovenskih goricah"), ("ro", "Comuna Sveta Trojica v Slovenskih goricah"), ("sl", "Občina Sveta Trojica v Slovenskih goricah"), ("uk", "Света Троїца-в-Словенських Горицях"), ("vi", "Sveta Trojica v Slovenskih goricah"), ("zh", "斯洛文尼亞丘陵內斯維塔特羅伊卡")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "205",
                    Subdivision{
                        name: "205",
                        country_alpha2: Alpha2::SI,
                        code: "205",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥\u{11133}𑄞𑄬𑄑\u{11128} 𑄑\u{1112e}𑄟𑄌\u{11134}"), ("ceb", "Občina Sveti Tomaž"), ("cs", "Občina Sveti Tomaž"), ("de", "Sveti Tomaž"), ("en", "Sveti Tomaž"), ("fr", "Sveti Tomaž"), ("hr", "Općina Sveti Tomaž"), ("it", "Sveti Tomaž"), ("ja", "スヴェティ・トマシュ"), ("nl", "Sveti Tomaž"), ("pl", "Gmina Sveti Tomaž"), ("pt", "Sveti Tomaž"), ("ro", "Comuna Sveti Tomaž"), ("sl", "Občina Sveti Tomaž"), ("uk", "Светий Томаж"), ("vi", "Sveti Tomaž"), ("zh", "斯韋提托馬日")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "206",
                    Subdivision{
                        name: "206",
                        country_alpha2: Alpha2::SI,
                        code: "206",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄌\u{11133}𑄟𑄢\u{11134}𑄎𑄬𑄌\u{11134}𑄇\u{11128} 𑄑\u{11127}𑄛\u{11134}𑄣\u{1112d}𑄌\u{11134}"), ("ceb", "Občina Šmarješke Toplice"), ("cs", "Občina Šmarješke Toplice"), ("en", "Šmarješke Toplice"), ("hr", "Općina Šmarješke Toplice"), ("ro", "Comuna Šmarješke Toplice"), ("sl", "Občina Šmarješke Toplice"), ("uk", "Шмарєшке Топлице"), ("zh", "什馬列什克-托普利采")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "207",
                    Subdivision{
                        name: "207",
                        country_alpha2: Alpha2::SI,
                        code: "207",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Gorje"), ("ccp", "𑄉\u{11127}\u{1112e}𑄢\u{11134}𑄎𑄬"), ("ceb", "Gorje"), ("cs", "Občina Gorje"), ("de", "Gorje"), ("en", "Gorje"), ("fr", "Gorje"), ("hr", "Općina Gorje"), ("it", "Gorje"), ("ja", "ゴリェ"), ("ko", "고레"), ("mk", "Општина Горје"), ("nl", "Gorje"), ("pl", "Gmina Gorje"), ("pt", "Gorje"), ("ro", "Comuna Gorje"), ("sl", "Občina Gorje"), ("uk", "Горє (община)"), ("vi", "Gorje"), ("zh", "戈列鎮")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "208",
                    Subdivision{
                        name: "208",
                        country_alpha2: Alpha2::SI,
                        code: "208",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Log-Dragomer"), ("ccp", "𑄣\u{11127}𑄇\u{11134}-𑄓\u{11133}𑄢𑄉\u{1112e}𑄟𑄢\u{11134}"), ("ceb", "Log-Dragomer"), ("cs", "Občina Log-Dragomer"), ("de", "Log-Dragomer"), ("en", "Log–Dragomer"), ("fr", "Log-Dragomer"), ("he", "לוג דרגומר"), ("hr", "Općina Log - Dragomer"), ("it", "Log-Dragomer"), ("ja", "ログ・ドラゴメル"), ("ko", "로그드라고메르"), ("nl", "Log-Dragomer"), ("pl", "Gmina Log-Dragomer"), ("pt", "Log-Dragomer"), ("ro", "Comuna Log-Dragomer"), ("sl", "Občina Log - Dragomer"), ("sr", "Општина Лог - Драгомер"), ("sr_Latn", "Opština Log - Dragomer"), ("uk", "Лог-Драгомер"), ("vi", "Log-Dragomer"), ("zh", "洛格-德拉戈梅爾鎮")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "209",
                    Subdivision{
                        name: "209",
                        country_alpha2: Alpha2::SI,
                        code: "209",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Rečica ob Savinji"), ("ccp", "𑄢𑄬𑄥𑄬𑄇 𑄃\u{11127}𑄛\u{11134} 𑄥𑄞\u{11128}𑄚\u{11134}𑄎\u{11128}"), ("ceb", "Občina Rečica ob Savinji"), ("cs", "Občina Rečica ob Savinji"), ("de", "Rečica ob Savinji"), ("en", "Rečica ob Savinji"), ("fr", "Rečica ob Savinji"), ("hr", "Općina Rečica ob Savinji"), ("it", "Rečica ob Savinji"), ("ja", "レチナ・オプ・サヴィニ"), ("ko", "레치차오브사비니"), ("nl", "Rečica ob Savinji"), ("pl", "Gmina Rečica ob Savinji"), ("pt", "Rečica ob Savinji"), ("ro", "Comuna Rečica ob Savinji"), ("sl", "Občina Rečica ob Savinji"), ("uk", "Речиця-об-Савиньї"), ("vi", "Rečica ob Savinji"), ("zh", "薩文利畔雷契察")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "210",
                    Subdivision{
                        name: "210",
                        country_alpha2: Alpha2::SI,
                        code: "210",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bs", "Sveti Jurij v Slovenskih goricah"), ("ccp", "𑄌\u{11133}𑄞𑄬𑄑\u{11128} 𑄎\u{1112a}𑄢\u{11128}𑄌\u{11134} 𑄞\u{11128} 𑄥\u{11133}𑄣\u{1112e}𑄞𑄬𑄚\u{11134}𑄇\u{11128}𑄦\u{11134} 𑄉\u{1112e}𑄢\u{11128}𑄇𑄦\u{11134}"), ("ceb", "Sveti Jurij v Slovenskih Goricah"), ("cs", "Občina Sveti Jurij v Slovenskih goricah"), ("de", "Sveti Jurij v Slovenskih goricah"), ("en", "Sveti Jurij v Slovenskih Goricah"), ("fr", "Sveti Jurij v Slovenskih goricah"), ("hr", "Općina Sveti Jurij v Slovenskih goricah"), ("hu", "Sveti Jurij v Slovenskih goricah"), ("it", "Sveti Jurij v Slovenskih goricah"), ("ja", "スヴェティ・ユリイ・ウ・スロヴェンスキフ・ゴリツァフ"), ("ko", "스베티유리브슬로벤스키흐고리차흐"), ("nl", "Sveti Jurij v Slovenskih goricah"), ("pl", "Gmina Sveti Jurij v Slovenskih goricah"), ("pt", "Sveti Jurij v Slovenskih goricah"), ("ro", "Comuna Sveti Jurij v Slovenskih goricah"), ("sl", "Občina Sveti Jurij v Slovenskih goricah"), ("uk", "Светий Юрій-в-Словенських Горицях"), ("vi", "Sveti Jurij v Slovenskih goricah"), ("zh", "斯洛文尼亞丘陵內斯韋提尤爾利")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "211",
                    Subdivision{
                        name: "211",
                        country_alpha2: Alpha2::SI,
                        code: "211",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢\u{1112a}𑄛𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Občina Šentrupert"), ("cs", "Občina Šentrupert"), ("de", "Šentrupert"), ("en", "Šentrupert"), ("fr", "Šentrupert"), ("hr", "Općina Šentrupert"), ("it", "Šentrupert"), ("ja", "シェントルペルト"), ("ko", "셴트루페르트"), ("nl", "Šentrupert"), ("pl", "Gmina Šentrupert"), ("pt", "Šentrupert"), ("ro", "Comuna Šentrupert"), ("sl", "Občina Šentrupert"), ("sr", "Општина Шентруперт"), ("sr_Latn", "Opština Šentrupert"), ("uk", "Шентруперт"), ("vi", "Šentrupert"), ("zh", "申特魯佩爾特")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "212",
                    Subdivision{
                        name: "212",
                        country_alpha2: Alpha2::SI,
                        code: "212",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄟\u{11128}𑄢\u{11134}𑄚"), ("cs", "Občina Mirna"), ("de", "Mirna"), ("en", "Mirna"), ("fr", "Mirna"), ("hr", "Mirna"), ("it", "Mirna"), ("ja", "ミルナ"), ("ko", "미르나"), ("nl", "Mirna"), ("pt", "Mirna"), ("ro", "Mirna"), ("ru", "Мирна"), ("sl", "Mirna"), ("uk", "Мирна"), ("vi", "Mirna")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "213",
                    Subdivision{
                        name: "213",
                        country_alpha2: Alpha2::SI,
                        code: "213",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ankaran"), ("bg", "Анкаран"), ("ca", "Ankaran"), ("ccp", "𑄃𑄚\u{11134}𑄇\u{11127}𑄢𑄚\u{11134}"), ("cs", "Občina Ankaran"), ("da", "Ankaran"), ("de", "Ankaran"), ("en", "Ankaran"), ("es", "Ankaran"), ("et", "Ankaran"), ("eu", "Ankaran"), ("fi", "Ankaran"), ("fr", "Ankaran"), ("gl", "Ankaran"), ("hr", "Ankaran"), ("hu", "Ankaran"), ("id", "Ankaran"), ("it", "Ancarano"), ("ja", "アンカラン"), ("ko", "안카란"), ("lt", "Ankaran"), ("lv", "Ankaran"), ("ms", "Ankaran"), ("nb", "Ankaran"), ("nl", "Ankaran"), ("no", "Ankaran"), ("pl", "Ankaran"), ("pt", "Ankaran"), ("ro", "Comuna Ankaran"), ("sk", "Ankaran"), ("sl", "Občina Ankaran"), ("sv", "Ankaran"), ("sw", "Ankaran"), ("th", "อ\u{e31}งคาราน"), ("tr", "Ankaran"), ("ur", "انکاران"), ("vi", "Ankaran"), ("zh", "安卡蘭"), ("zu", "Ankaran")]),
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
#[cfg(feature = "si")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SI,
        alpha3: Alpha3::SVN,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 386,
        currency_code: "EUR",
        gec: Some(GEC::SI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("SLO"),
        iso_long_name: "The Republic of Slovenia",
        iso_short_name: "Slovenia",
        official_language_list: ["sl"].to_vec(),
        spoken_language_list: ["sl"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Slovene"),
        number: "705",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "SI",
        unofficial_name_list: [
            "Slovenia",
            "Slowenien",
            "Slovénie",
            "Eslovenia",
            "スロベニア",
            "Slovenië",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Slovenia"),
            ("af", "Slowenië"),
            ("ak", "Slovenia"),
            ("am", "ስሕታኒ።"),
            ("an", "Slovenia"),
            ("ar", "سلوفينيا"),
            ("as", "স\u{9cd}লোভেনিয়\u{9be}"),
            ("ay", "Slovenia"),
            ("az", "Sloveniya"),
            ("ba", "Slovenia"),
            ("be", "Славенія"),
            ("bg", "Словения"),
            ("bi", "Slovenia"),
            ("bn", "স\u{9cd}লোভেনিয়\u{9be}"),
            ("bn_IN", "স\u{9cd}লোভেনিয়\u{9be}"),
            ("br", "Slovenia"),
            ("bs", "Slovenija"),
            ("ca", "Eslovènia"),
            ("ce", "Словени"),
            ("ch", "Slovenia"),
            ("cs", "Slovinsko"),
            ("cv", "Словени"),
            ("cy", "Slofenia"),
            ("da", "Slovenien"),
            ("de", "Slowenien"),
            ("dv", "ސ\u{7aa}ލ\u{7ae}ވ\u{7a9}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "ས\u{f7c}ལ\u{f7c}་བ\u{f72}་ན\u{f72}་ཡ།"),
            ("ee", "Slovenia"),
            ("el", "Σλοβενία"),
            ("en", "Slovenia"),
            ("eo", "Slovenio"),
            ("es", "Eslovenia"),
            ("et", "Sloveenia"),
            ("eu", "Eslovenia"),
            ("fa", "اسلوونی"),
            ("ff", "Suloweniya"),
            ("fi", "Slovenia"),
            ("fo", "Slovenia"),
            ("fr", "Slovénie"),
            ("fy", "Sloveenje"),
            ("ga", "An tSlóivéin"),
            ("gl", "Eslovenia"),
            ("gn", "Slovenia"),
            ("gu", "સ\u{acd}લોવ\u{ac7}નિયા"),
            ("gv", "Yn Clovean"),
            ("ha", "Sloveniya"),
            ("he", "סלובניה"),
            ("hi", "स\u{94d}लोव\u{947}निया"),
            ("hr", "Slovenija"),
            ("ht", "Sloveni"),
            ("hu", "Szlovénia"),
            ("hy", "Սլովենիա"),
            ("ia", "Slovenia"),
            ("id", "Slovenia"),
            ("io", "Slovenia"),
            ("is", "Slóvenía"),
            ("it", "Slovenia"),
            ("iu", "Slovenia"),
            ("ja", "スロベニア"),
            ("ka", "სლოვენია"),
            ("ki", "Slovenia"),
            ("kk", "Словения"),
            ("kl", "Slovenia"),
            ("km", "ស\u{17d2}ល\u{17bc}វ\u{17c9}ាន\u{17b8}"),
            ("kn", "ಸ\u{ccd}ಲೋವೇನ\u{cbf}ಯಾ"),
            ("ko", "슬로베니아"),
            ("ku", "Slovenya"),
            ("kv", "Словения"),
            ("kw", "Sloveni"),
            ("ky", "Словения"),
            ("lo", "ປະເທດສະໂລເວນ\u{eb5}"),
            ("lt", "Slovėnija"),
            ("lv", "Slovēnija"),
            ("mi", "Horowinia"),
            ("mk", "Словенија"),
            ("ml", "സ\u{d4d}ലോവേനിയ"),
            ("mn", "Словени"),
            ("mr", "स\u{94d}लोव\u{94d}ह\u{947}निया"),
            ("ms", "Slovenia"),
            ("mt", "Slovenja"),
            (
                "my",
                "ဆလ\u{102d}\u{102f}ဗေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Tsirobeniya"),
            ("nb", "Slovenia"),
            ("ne", "स\u{94d}लोभ\u{947}निया"),
            ("nl", "Slovenië"),
            ("nn", "Slovenia"),
            ("nv", "Słobíín Bikéyah"),
            ("oc", "Eslovènia"),
            ("or", "ସ\u{b4d}ଲୋଭ\u{b3e}ନ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਸਲ\u{a4b}ਵੀਨੀਆ"),
            ("pi", "स\u{94d}लोवीनिया"),
            ("pl", "Słowenia"),
            ("ps", "سلووانیا"),
            ("pt", "Eslovénia"),
            ("pt_BR", "Eslovênia"),
            ("ro", "Slovenia"),
            ("ru", "Словения"),
            ("rw", "Siloveniya"),
            ("sc", "Islovènia"),
            ("sd", "Slovenia"),
            ("si", "ස\u{dca}ලෝව\u{dd3}න\u{dd2}ය\u{dcf}ව"),
            ("sk", "Slovinsko"),
            ("sl", "Slovenija"),
            ("so", "Slovenia"),
            ("sq", "Slloveni"),
            ("sr", "Словенија"),
            ("sv", "Slovenien"),
            ("sw", "Slovenia"),
            ("ta", "ஸ\u{bcd}லோவேனிய\u{bbe}"),
            ("te", "స\u{c4d}ల\u{c4b}వ\u{c47}న\u{c3f}య\u{c3e}"),
            ("tg", "Словения"),
            ("th", "สโลว\u{e35}เน\u{e35}ย"),
            ("ti", "ስሎቬኒያ"),
            ("tk", "Sloweniýa"),
            ("tl", "Slovenia"),
            ("tr", "Slovenya"),
            ("tt", "Словениа"),
            ("ug", "سىلوۋېنىيە"),
            ("uk", "Словенія"),
            ("ur", "سلووینیا"),
            ("uz", "Sloveniya"),
            ("ve", "Slovenia"),
            ("vi", "Xlô-ven"),
            ("wa", "Esloveneye"),
            ("wo", "Esloweeni"),
            ("xh", "Slovenia"),
            ("yo", "Sloféníà"),
            ("zh_CN", "斯洛文尼亚"),
            ("zh_HK", "斯洛文尼亞"),
            ("zh_TW", "斯洛維尼亞"),
            ("zu", "ISloveniya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}