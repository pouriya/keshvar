// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Swiss Confederation

#[cfg(all(feature = "ch", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::CH;
    pub const ALPHA3: Alpha3 = Alpha3::CHE;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 41;
    pub const CURRENCY_CODE: &str = "CHF";
    pub const GEC: Option<GEC> = Some(GEC::SZ);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::SUI);
    pub const ISO_SHORT_NAME: &str = "Switzerland";
    pub const ISO_LONG_NAME: &str = "The Swiss Confederation";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["de", "fr", "it"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["de", "fr", "it"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Swiss");
    pub const NUMBER: &str = "756";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "CH";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Switzerland",
        "Schweiz",
        "Suisse",
        "Suiza",
        "スイス",
        "Zwitserland",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Switzerland"),
        ("af", "Switserland"),
        ("ak", "Switzerland"),
        ("am", "ስፁፈሴሒን፥"),
        ("an", "Switzerland"),
        ("ar", "سويسرا"),
        ("as", "ছ\u{9c1}ইজ\u{9be}ৰলেণ\u{9cd}ড"),
        ("ay", "Switzerland"),
        ("az", "İsveçrə"),
        ("ba", "Switzerland"),
        ("be", "Швейцарыя"),
        ("bg", "Швейцария"),
        ("bi", "Switzerland"),
        ("bn", "স\u{9c1}ইৎজ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "স\u{9c1}ইৎজ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Suis"),
        ("bs", "Švicarska"),
        ("ca", "Suïssa"),
        ("ce", "Швейцари"),
        ("ch", "Switzerland"),
        ("cs", "Švýcarsko"),
        ("cv", "Швейцари"),
        ("cy", "Y Swistir"),
        ("da", "Schweiz"),
        ("de", "Schweiz"),
        ("dv", "ސ\u{7aa}ވ\u{7a8}ޒ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
        ("dz", "ས་ཝ་ཛ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"),
        ("ee", "Switzerland"),
        ("el", "Ελβετία"),
        ("en", "Switzerland"),
        ("eo", "Svislando"),
        ("es", "Suiza"),
        ("et", "Šveits"),
        ("eu", "Suitza"),
        ("fa", "سوئیس"),
        ("ff", "Suwis"),
        ("fi", "Sveitsi"),
        ("fo", "Sveis"),
        ("fr", "Suisse"),
        ("fy", "Switserlân"),
        ("ga", "An Eilvéis"),
        ("gl", "Suíza"),
        ("gn", "Switzerland"),
        ("gu", "સ\u{acd}વિત\u{acd}ઝરલ\u{ac7}ન\u{acd}ડ"),
        ("gv", "Yn Elveeish"),
        ("ha", "Switzerland"),
        ("he", "שווייץ"),
        ("hi", "स\u{94d}विट\u{94d}ज\u{93c}रल\u{948}ण\u{94d}ड"),
        ("hr", "Švicarska"),
        ("ht", "Swis"),
        ("hu", "Svájc"),
        ("hy", "Շվեյցարիա"),
        ("ia", "Suissa"),
        ("id", "Swiss"),
        ("io", "Suisia"),
        ("is", "Sviss"),
        ("it", "Svizzera"),
        ("iu", "Switzerland"),
        ("ja", "スイス"),
        ("ka", "შვეიცარია"),
        ("ki", "Switzerland"),
        ("kk", "Швейцария"),
        ("kl", "Switzerland"),
        ("km", "ស\u{17d2}វ\u{17ca}\u{17b8}ស"),
        ("kn", "ಸ\u{ccd}ವ\u{cbf}ಜರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"),
        ("ko", "스위스"),
        ("ku", "Swîsre"),
        ("kv", "Швейцария"),
        ("kw", "Swistir"),
        ("ky", "Швейцария"),
        ("lo", "ປະເທດສະວ\u{eb4}ດ"),
        ("lt", "Šveicarija"),
        ("lv", "Šveice"),
        ("mi", "Huiterangi"),
        ("mk", "Швајцарија"),
        (
            "ml",
            "സ\u{d4d}വിറ\u{d4d}റ\u{d4d}സര\u{d4d}\u{200d}ല\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d}",
        ),
        ("mn", "Швецарь"),
        ("mr", "स\u{94d}वित\u{94d}झर\u{94d}ल\u{902}ड"),
        ("ms", "Switzerland"),
        ("mt", "Svizzera"),
        (
            "my",
            "ဆ\u{103d}စ\u{103a}ဇာလန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Witsierand"),
        ("nb", "Sveits"),
        ("ne", "स\u{94d}विजरल\u{94d}याण\u{94d}ड"),
        ("nl", "Zwitserland"),
        ("nn", "Sveits"),
        ("nv", "Swis Bikéyah"),
        ("oc", "Soïssa"),
        ("or", "ସ\u{b4d}ବ\u{b3f}ଟ\u{b4d}ଜରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਸਵਿਟਰਜ਼ਰਲ\u{a48}\u{a02}ਡ"),
        ("pi", "स\u{94d}विटजरल\u{948}\u{902}ड"),
        ("pl", "Szwajcaria"),
        ("ps", "سویس"),
        ("pt", "Suíça"),
        ("pt_BR", "Suíça"),
        ("ro", "Elveția"),
        ("ru", "Швейцария"),
        ("rw", "Ubusuwisi"),
        ("sc", "Isvìtzera"),
        ("sd", "Switzerland"),
        ("si", "ස\u{dca}ව\u{dd2}ට\u{dca}සර\u{dca}ලන\u{dca}තය"),
        ("sk", "Švajčiarsko"),
        ("sl", "Švica"),
        ("so", "Swiiserlaand"),
        ("sq", "Zvicër"),
        ("sr", "Швајцарска"),
        ("sv", "Schweiz"),
        ("sw", "Uswisi"),
        ("ta", "சுவிட\u{bcd}சர\u{bcd}ல\u{bbe}ந\u{bcd}து"),
        ("te", "స\u{c4d}వ\u{c3f}డ\u{c4d}జర\u{c4d}ల\u{c3e}ండ\u{c4d}"),
        ("tg", "Швейтсария"),
        ("th", "สว\u{e34}ตเซอร\u{e4c}แลนด\u{e4c}"),
        ("ti", "ስዊዘርላንድ"),
        ("tk", "Şweýsariýa"),
        ("tl", "Switzerland"),
        ("tr", "İsviçre"),
        ("tt", "İсвичрә"),
        ("ug", "شىۋېيىتسارىيە"),
        ("uk", "Швейцарія"),
        ("ur", "سویٹزرلینڈ"),
        ("uz", "Shveysariya"),
        ("ve", "Switzerland"),
        ("vi", "Thụy Sĩ"),
        ("wa", "Swisse"),
        ("wo", "Suwis"),
        ("xh", "Switzerland"),
        ("yo", "Swítsàlandì"),
        ("zh_CN", "瑞士"),
        ("zh_HK", "瑞士"),
        ("zh_TW", "瑞士"),
        ("zu", "I-Switzerland"),
    ];
    #[cfg(all(feature = "ch", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 46.818188;
        pub const LONGITUDE: f64 = 8.227511999999999;
        pub const MAX_LATITUDE: f64 = 47.8084546;
        pub const MAX_LONGITUDE: f64 = 10.4923401;
        pub const MIN_LATITUDE: f64 = 45.81792;
        pub const MIN_LONGITUDE: f64 = 5.95608;
        pub const NORTHEAST_LATITUDE: f64 = 47.8084546;
        pub const NORTHEAST_LONGITUDE: f64 = 10.4923401;
        pub const SOUTHWEST_LATITUDE: f64 = 45.81792;
        pub const SOUTHWEST_LONGITUDE: f64 = 5.95608;
    }
}
#[cfg(all(feature = "ch", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 46.818188,
            longitude: 8.227511999999999,
            max_latitude: 47.8084546,
            max_longitude: 10.4923401,
            min_latitude: 45.81792,
            min_longitude: 5.95608,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 47.8084546,
                    longitude: 10.4923401,
                },
                southwest: CountryGeoBound {
                    latitude: 45.81792,
                    longitude: 5.95608,
                },
            },
        }
    }
}

#[cfg(all(feature = "ch", feature = "subdivisions"))]
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
                    "AG",
                    Subdivision{
                        name: "AG",
                        country_alpha2: Alpha2::CH,
                        code: "AG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3876664), longitude: Some(8.2554295), max_latitude: Some(47.6209201), min_latitude: Some(47.13755), max_longitude: Some(8.455169999999999), min_longitude: Some(7.713470099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kanton Aargau"), ("ar", "كانتون أرجاو"), ("be", "Кантон Ааргау"), ("bg", "Ааргау"), ("bn", "আরগ\u{9be}ও"), ("bs", "Aargau"), ("ca", "Argòvia"), ("ccp", "𑄃𑄢\u{11134}𑄉𑄅\u{1112a}"), ("ceb", "Kanton Aargau"), ("cs", "Aargau"), ("cy", "Aargau"), ("da", "Aargau"), ("de", "Aargau"), ("el", "Άαργκαου"), ("en", "Aargau"), ("es", "Argovia"), ("et", "Aargau kanton"), ("eu", "Argovia"), ("fa", "ایالت آرگاو"), ("fi", "Aargau"), ("fr", "canton d’Argovie"), ("gl", "Argovia"), ("gu", "એરાગાઉ"), ("he", "ארגאו"), ("hi", "आरगाउ क\u{948}न\u{94d}टन"), ("hr", "Aargau"), ("hu", "Aargau kanton"), ("hy", "Արգաու"), ("id", "Kanton Aargau"), ("is", "Aargau"), ("it", "Canton Argovia"), ("ja", "アールガウ州"), ("jv", "Kanton Aargau"), ("ka", "აარგაუს კანტონი"), ("kk", "Аргау"), ("kn", "ಅರ\u{ccd}ಗ\u{ccc}"), ("ko", "아르가우 주"), ("lt", "Argau"), ("lv", "Ārgavas kantons"), ("mk", "Аргау"), ("mr", "आर\u{94d}गाउ"), ("ms", "Aargau"), ("nb", "Aargau"), ("nl", "Aargau"), ("no", "Aargau"), ("pl", "Argowia"), ("pt", "Argóvia"), ("ro", "Cantonul Argovia"), ("ru", "Аргау"), ("si", "ආර\u{dca}ගෞ"), ("sk", "Aargau"), ("sl", "Aargau"), ("sq", "Kantoni Aargau"), ("sr", "Кантон Аргау"), ("sr_Latn", "Kanton Argau"), ("sv", "Aargau"), ("sw", "Aargau"), ("ta", "ஆர\u{bcd}க\u{bbe}வு"), ("te", "ఆర\u{c3e}గ\u{c3e}వ\u{c4d}"), ("th", "ร\u{e31}ฐอาร\u{e4c}เกา"), ("tr", "Aargau"), ("uk", "Ааргау"), ("ur", "آرگاؤ"), ("vi", "Aargau"), ("yue", "阿爾膠州"), ("yue_Hans", "阿尔胶州"), ("zh", "阿爾高州")]),
                        unofficial_name_list: ["Argovie"].to_vec(),
                    }
                ),
                (
                    "AI",
                    Subdivision{
                        name: "AI",
                        country_alpha2: Alpha2::CH,
                        code: "AI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3161925), longitude: Some(9.4316573), max_latitude: Some(47.43874), min_latitude: Some(47.2339999), max_longitude: Some(9.617479999999999), min_longitude: Some(9.309809999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون أبينزيل إينرهودن"), ("be", "Кантон Апенцэль — Інерродэн"), ("bg", "Апенцел Инерроден"), ("bn", "অ\u{9cd}য\u{9be}প\u{9cd}যেনজেল ইন\u{9be}রহোদেন"), ("bs", "Appenzell Innerrhoden"), ("ca", "Appenzell Inner-Rhoden"), ("ccp", "𑄃𑄬𑄛𑄬𑄚\u{11134}𑄎𑄬𑄣\u{11134} 𑄃\u{11128}𑄚𑄬𑄢\u{11134}𑄦\u{1112e}𑄓𑄬𑄚\u{11134}"), ("ceb", "Kanton Appenzell Innerrhoden"), ("cs", "Appenzell Innerrhoden"), ("cy", "Appenzell Innerrhoden"), ("da", "Appenzell Innerrhoden"), ("de", "Appenzell Innerrhoden"), ("el", "Απενζέλ Ινερχόντεν"), ("en", "Appenzell Innerrhoden"), ("es", "Cantón de Appenzell Rodas Interiores"), ("et", "Appenzell Innerrhoden"), ("eu", "Appenzell Innerrhoden"), ("fa", "ایالت آپنتزل اینرهودن"), ("fi", "Appenzell Innerrhoden"), ("fr", "canton d’Appenzell Rhodes-Intérieures"), ("gl", "Appenzell Interior"), ("gu", "એપ\u{ac7}નઝ\u{ac7}લ ઈનરહોડન"), ("he", "אפנצל אינר-רודן"), ("hi", "आपनत\u{94d}स\u{947}ल इन\u{94d}नररोडन क\u{948}न\u{94d}टन"), ("hr", "Appenzell Innerrhoden"), ("hu", "Appenzell Innerrhoden kanton"), ("id", "Kanton Appenzell Innerrhoden"), ("it", "Canton Appenzello Interno"), ("ja", "アッペンツェル・インナーローデン準州"), ("jv", "Kanton Appenzell Innerrhoden"), ("ka", "აპენცელ-ინერჰოდენის კანტონი"), ("kk", "Аппенцелль-Иннерроден"), ("kn", "ಅಪ\u{ccd}ಪ\u{cc6}ನ\u{ccd}ಜ\u{cc6}ಲ\u{ccd} ಇನ\u{ccd}ನರ\u{ccd}ಹೊಡ\u{cc6}ನ\u{ccd}"), ("ko", "아펜첼이너로덴 주"), ("lt", "Apencelis-Inerodenas"), ("lv", "Apencelles-Innerrodenes kantons"), ("mk", "Апенцел Инероден"), ("mr", "आप\u{947}\u{902}झ\u{947}ल इनरर\u{94d} होडन"), ("ms", "Appenzell Innerrhoden"), ("nb", "Appenzell Innerrhoden"), ("nl", "Appenzell Innerrhoden"), ("no", "Appenzell Innerrhoden"), ("pl", "Appenzell Innerrhoden"), ("pt", "Appenzell Interior"), ("ro", "Cantonul Appenzell Intern"), ("ru", "Аппенцелль-Иннерроден"), ("si", "අප\u{dca}පෙන\u{dca}සෙල\u{dca} ඉනේර\u{dca}හෝඩ\u{dca}ඩෙන\u{dca}"), ("sk", "Appenzell Innerrhoden"), ("sq", "Kantoni Appenzell Innerrhoden"), ("sr", "Кантон Апенцел Инероден"), ("sr_Latn", "Kanton Apencel Ineroden"), ("sv", "Appenzell Innerrhoden"), ("sw", "Appenzell Innerrhoden"), ("ta", "அப\u{bcd}பென\u{bcd}ஸ\u{bcd}ல\u{bcd} இன\u{bcd}னேற\u{bcd}ரஹோடேன\u{bcd}"), ("te", "అప\u{c46}ంజ\u{c46}ల\u{c4d} ఇన\u{c4d}నర\u{c4d}హ\u{c4b}డ\u{c46}న\u{c4d}"), ("th", "แอฟเพนเซลล\u{e4c} อ\u{e34}นเนอร\u{e4c}โรเดน"), ("tr", "Appenzell Innerrhoden"), ("uk", "Аппенцелль — Іннерроден"), ("ur", "اپینسیل انیررودن"), ("vi", "Appenzell Innerrhoden"), ("yue", "內阿彭策州"), ("yue_Hans", "内阿彭策州"), ("zh", "內阿彭策爾州")]),
                        unofficial_name_list: ["Appenzell Innerrhoden (de)"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "AR",
                        country_alpha2: Alpha2::CH,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.366481), longitude: Some(9.3000916), max_latitude: Some(47.4690301), min_latitude: Some(47.24695), max_longitude: Some(9.63088), min_longitude: Some(9.1910399)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون أبينزيل أوسيرهودن"), ("be", "Кантон Апенцэль — Інерродэн²"), ("bg", "Апенцел Аусерроден"), ("bn", "অ\u{9cd}য\u{9be}পেনজেল অস\u{9be}রহোডেন"), ("bs", "Appenzell Ausserrhoden"), ("ca", "Appenzell Ausser-Rhoden"), ("ccp", "𑄃𑄬𑄛𑄬𑄚\u{11134}𑄎𑄬𑄣\u{11134} 𑄃\u{11127}𑄅\u{1112a}𑄥𑄬𑄢\u{11134}𑄦\u{1112e}𑄓𑄬𑄚\u{11134}"), ("ceb", "Kanton Appenzell Ausserrhoden"), ("cs", "Appenzell Ausserrhoden"), ("cy", "Appenzell Ausserrhoden"), ("da", "Appenzell Ausserrhoden"), ("de", "Appenzell Außerrhoden"), ("de_CH", "Appenzell Ausserrhoden"), ("el", "Απενζέλ Οσερχόντεν"), ("en", "Appenzell Ausserrhoden"), ("es", "Cantón de Appenzell Rodas Exteriores"), ("et", "Appenzell Ausserrhoden"), ("eu", "Appenzell Ausserrhoden"), ("fa", "ایالت آپنتزل آوسرهودن"), ("fi", "Appenzell Ausserrhoden"), ("fr", "canton d’Appenzell Rhodes-Extérieures"), ("gl", "Appenzell Exterior"), ("gu", "એપ\u{ac7}નઝ\u{ac7}લ ઔસરહોડ\u{ac7}ન"), ("he", "אפנצל אוסר-רודן"), ("hi", "आपनत\u{94d}स\u{947}ल आउसारोडन क\u{948}न\u{94d}टन"), ("hr", "Appenzell Ausserrhoden"), ("hu", "Appenzell Ausserrhoden kanton"), ("id", "Kanton Appenzell Ausserrhoden"), ("it", "Canton Appenzello Esterno"), ("ja", "アッペンツェル・アウサーローデン準州"), ("jv", "Kanton Appenzell Ausserrhoden"), ("ka", "აპენცელ-აუსერჰოდენის კანტონი"), ("kk", "Аппенцелль-Ауссерроден"), ("kn", "ಅಪ\u{ccd}ಪ\u{cc6}ನ\u{ccd}ಜ\u{cc6}ಲ\u{ccd} ಆಸ\u{cc6}ರ\u{ccd}ಹೊಡ\u{cc6}ನ\u{ccd}"), ("ko", "아펜첼아우서로덴 주"), ("lt", "Apencelis-Auserodenas"), ("lv", "Apencelles-Auserrodenes kantons"), ("mk", "Апенцел Аусероден"), ("mr", "आप\u{947}\u{902}झ\u{947}ल आउसरर\u{94d} होडन"), ("ms", "Appenzell Ausserrhoden"), ("nb", "Appenzell Ausserrhoden"), ("nl", "Appenzell Ausserrhoden"), ("no", "Appenzell Ausserrhoden"), ("pl", "Appenzell Ausserrhoden"), ("pt", "Appenzell Exterior"), ("ro", "Cantonul Appenzell Extern"), ("ru", "Аппенцелль-Ауссерроден"), ("si", "ඇපන\u{dca}සෙල\u{dca} ඔස\u{dca}සර\u{dca}හෝඩෙන\u{dca}"), ("sk", "Appenzell Ausserrhoden"), ("sq", "Kantoni Appenzell Ausserrhoden"), ("sr", "Кантон Апенцел Аусероден"), ("sr_Latn", "Kanton Apencel Auseroden"), ("sv", "Appenzell Ausserrhoden"), ("sw", "Appenzell Ausserrhoden"), ("ta", "அப\u{bcd}பென\u{bcd}ஸ\u{bcd}ல\u{bcd} ஒஸ\u{bcd}செற\u{bcd}ரஹோடேன\u{bcd}"), ("te", "అప\u{c46}ంజ\u{c3f}ల\u{c4d} అస\u{c46}ర\u{c4d}హ\u{c4b}డ\u{c46}న\u{c4d}"), ("th", "อ\u{e31}พเพ\u{e34}นท\u{e4c}เซลล\u{e4c}เอาส\u{e4c}เซอร\u{e4c}โรเด\u{e34}น"), ("tr", "Appenzell Ausserrhoden"), ("uk", "Аппенцелль — Ауссерроден"), ("ur", "اپینسیل اوسیررودن"), ("vi", "Appenzell Ausserrhoden"), ("yue", "外阿彭策州"), ("yue_Hans", "外阿彭策州"), ("zh", "外阿彭策爾州")]),
                        unofficial_name_list: ["Appenzell-Ausser Rhoden"].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::CH,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.9479222), longitude: Some(7.444608499999999), max_latitude: Some(46.99019), min_latitude: Some(46.9191499), max_longitude: Some(7.495510099999999), min_longitude: Some(7.294230000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون برن"), ("be", "Кантон Берн"), ("bg", "Берн"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অব ব\u{9be}র\u{9cd}ন"), ("bs", "Bern"), ("ca", "Cantó de Berna"), ("ccp", "𑄝𑄢\u{11133}𑄚\u{11134}"), ("ceb", "Canton de Berne"), ("cs", "Bern"), ("cy", "Bern"), ("da", "Kanton Bern"), ("de", "Bern"), ("el", "Καντόνι της Βέρνης"), ("en", "Bern"), ("es", "Berna"), ("et", "Berni kanton"), ("eu", "Berna kantonamendua"), ("fa", "ایالت برن"), ("fi", "Bern"), ("fr", "canton de Berne"), ("gl", "Cantón de Berna"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ બર\u{acd}ન"), ("he", "ברן"), ("hi", "बर\u{94d}न क\u{948}न\u{94d}टन"), ("hu", "Bern kanton"), ("id", "Kanton Bern"), ("is", "Bern"), ("it", "Canton Berna"), ("ja", "ベルン州"), ("jv", "Kanton Bern"), ("ka", "ბერნის კანტონი"), ("kk", "Берн"), ("kn", "ಬರ\u{ccd}ನ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "베른 주"), ("lt", "Berno kantonas"), ("lv", "Bernes kantons"), ("mk", "Берн"), ("mr", "बर\u{94d}न"), ("ms", "Canton of Bern"), ("nb", "Bern"), ("nl", "Bern"), ("no", "Bern"), ("pl", "Berno"), ("pt", "Berna"), ("ro", "Berna"), ("ru", "Берн"), ("si", "බෙම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Bern"), ("sl", "Kanton Bern"), ("sr", "Кантон Берн"), ("sr_Latn", "Kanton Bern"), ("sv", "Bern"), ("sw", "Jimbo la Bern"), ("ta", "க\u{bbe}ன\u{bcd}டோன\u{bcd} ஆப\u{bcd} பெர\u{bcd}ன\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} బ\u{c46}ర\u{c4d}న\u{c4d}"), ("th", "ร\u{e31}ฐแบร\u{e4c}น"), ("tr", "Bern"), ("uk", "Берн"), ("ur", "کینٹن برن"), ("vi", "Bern"), ("yue", "伯恩州"), ("yue_Hans", "伯恩州"), ("zh", "伯恩州")]),
                        unofficial_name_list: ["Berne"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::CH,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.44181220000001), longitude: Some(7.7644002), max_latitude: Some(47.56441), min_latitude: Some(47.33792), max_longitude: Some(7.9618001), min_longitude: Some(7.32527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون ريف بازل"), ("be", "Кантон Базель-Ланд"), ("bg", "Базел Ландшафт"), ("bn", "ব\u{9cd}য\u{9be}সেল-ল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Basel-provincija"), ("ca", "Basilea-Camp"), ("ccp", "𑄝𑄥𑄬𑄣\u{11134}-𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄌\u{11133}𑄠𑄜\u{11133}𑄑\u{11134}"), ("ceb", "Kanton Basel-Landschaft"), ("cs", "Basilej-venkov"), ("cy", "Basel Wledig"), ("da", "Basel-Landschaft"), ("de", "Basel-Landschaft"), ("el", "Καντόνι της Μπάζελ-Λάντσαφτ"), ("en", "Basel-Landschaft"), ("es", "Cantón de Basilea-Campiña"), ("et", "Basel-Landschaft"), ("eu", "Basilea Herrialdea"), ("fa", "ایالت بازل-لاندشافت"), ("fi", "Basel-Landschaft"), ("fr", "canton de Bâle-Campagne"), ("gl", "Basilea-Campo"), ("gu", "બ\u{ac7}સલ-લ\u{ac7}ન\u{acd}ડ"), ("he", "בזל-לנדשאפט"), ("hi", "बासल-लान\u{94d}डशाफ\u{93c}\u{94d}ट क\u{948}न\u{94d}टन"), ("hr", "Basel-Landschaft"), ("hu", "Basel-Landschaft kanton"), ("hy", "Բազել-Լանդ"), ("id", "Kanton Basel-Negeri"), ("it", "Canton Basilea Campagna"), ("jv", "Basel-Landschaft"), ("ka", "ბაზელ-ლანდის კანტონი"), ("kk", "Базель-Ланд"), ("kn", "ಬಸ\u{cc6}ಲ\u{ccd}-ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "바젤란트 주"), ("lt", "Bazelio sritis"), ("lv", "Bāzeles lauku kantons"), ("mk", "Базел-краина"), ("mr", "बासल-ला\u{902}डशाफ\u{94d}ट"), ("ms", "Basel-Landschaft"), ("nb", "Basel-Landschaft"), ("nl", "Basel-Landschaft"), ("no", "Basel-Landschaft"), ("pl", "Bazylea-Okręg"), ("pt", "Basileia-Campo"), ("ro", "Cantonul Basel-Provincie"), ("ru", "Базель-Ланд"), ("si", "බසෙල\u{dca}-ලෑන\u{dca}ඩ\u{dca}"), ("sk", "Bazilej-vidiek"), ("sr", "Кантон Базел-провинција"), ("sr_Latn", "Kanton Bazel-provincija"), ("sv", "Basel-Landschaft"), ("sw", "Jimbo la Basel"), ("ta", "ப\u{bbe}ஸல\u{bcd} -லேண\u{bcd}ட\u{bcd}"), ("te", "బ\u{c47}స\u{c46}ల-ల\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐบาเซ\u{e34}ล-ล\u{e31}นท\u{e4c}ช\u{e31}ฟท\u{e4c}"), ("tr", "Basel-Landschaft"), ("uk", "Базель-Ланд"), ("ur", "بازل-لاندشافت"), ("vi", "Basel-Landschaft"), ("yue", "巴塞爾鄉村州"), ("yue_Hans", "巴塞尔乡村州"), ("zh", "巴塞爾鄉村州")]),
                        unofficial_name_list: ["Bâle-Campagne"].to_vec(),
                    }
                ),
                (
                    "BS",
                    Subdivision{
                        name: "BS",
                        country_alpha2: Alpha2::CH,
                        code: "BS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.5674422), longitude: Some(7.597550699999999), max_latitude: Some(47.5899201), min_latitude: Some(47.51931), max_longitude: Some(7.634099999999999), min_longitude: Some(7.554819900000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون مدينة بازل"), ("be", "Кантон Базель-Штат"), ("bg", "Базел Щат"), ("bn", "ব\u{9be}\u{9cd}য\u{9be}সেল-স\u{9cd}ট\u{9be}ড"), ("bs", "Basel-grad"), ("ca", "Basilea-Ciutat"), ("ccp", "𑄝𑄥𑄬𑄣\u{11134}-𑄌\u{11133}𑄑𑄖\u{11134}𑄑\u{11134}"), ("ceb", "Kanton Basel-Stadt"), ("cs", "Basilej-město"), ("cy", "Basel Ddinesig"), ("da", "Basel-Stadt"), ("de", "Basel-Stadt"), ("el", "Καντόνι της Μπάζελ-Στατ"), ("en", "Basel-Stadt"), ("es", "Cantón de Basilea-Ciudad"), ("et", "Basel-Stadt"), ("eu", "Basilea Hiria"), ("fa", "ایالت بازل اشتاد"), ("fi", "Basel-Stadt"), ("fr", "canton de Bâle-Ville"), ("gl", "Basilea-Cidade"), ("gu", "બ\u{ac7}સલ-સ\u{acd}ટ\u{ac7}ટ"), ("he", "בזל-שטאדט"), ("hi", "बासल-श\u{94d}तात क\u{948}न\u{94d}टन"), ("hu", "Basel-Stadt kanton"), ("id", "Kanton Basel-Kota"), ("it", "Canton Basilea Città"), ("jv", "Basel-Stadt"), ("ka", "ბაზელ-შტადტის კანტონი"), ("kk", "Базель-Штадт"), ("kn", "ಬಸ\u{cc6}ಲ\u{ccd}-ಸ\u{ccd}ಟಾಡ\u{ccd}ಟ\u{ccd}"), ("ko", "바젤슈타트 주"), ("lt", "Baselstadas"), ("lv", "Bāzeles pilsētas kantons"), ("mk", "Базел-град"), ("mr", "बासल-श\u{94d}टाट"), ("ms", "Bandar Basel"), ("nb", "Basel-Stadt"), ("nl", "Bazel-Stad"), ("no", "Basel-Stadt"), ("pl", "Bazylea-Miasto"), ("pt", "Basileia-Cidade"), ("ro", "Cantonul Basel-Oraș"), ("ru", "Базель-Штадт"), ("si", "බැසෙල\u{dca}-ස\u{dca}ටඩ\u{dca}ට\u{dca}"), ("sk", "Bazilej-mesto"), ("sq", "Kantoni Basel-Stadt"), ("sr", "Кантон Базел-град"), ("sr_Latn", "Kanton Bazel-grad"), ("sv", "Basel-Stadt"), ("sw", "Mji wa Basel"), ("ta", "ப\u{bbe}ஸல\u{bcd} -ஸ\u{bcd}ட\u{bbe}ட\u{bcd}ட\u{bcd}"), ("te", "బ\u{c3e}స\u{c46}ల\u{c4d}-స\u{c4d}ట\u{c3e}ట\u{c4d}"), ("th", "ก\u{e34}\u{e48}งร\u{e31}ฐบาเซ\u{e34}ล-ชต\u{e31}ดท\u{e4c}"), ("tr", "Basel-Stadt"), ("uk", "Базель-Штадт"), ("ur", "بازل-شتادت"), ("vi", "Basel-Stadt"), ("yue", "巴塞爾城市州"), ("yue_Hans", "巴塞尔城市州"), ("zh", "巴塞爾城市州")]),
                        unofficial_name_list: ["Basel", "Basel-Stadt", "Basilea", "Basle", "Bâle-Ville"].to_vec(),
                    }
                ),
                (
                    "FR",
                    Subdivision{
                        name: "FR",
                        country_alpha2: Alpha2::CH,
                        code: "FR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8016663), longitude: Some(7.145568300000001), max_latitude: Some(46.82144), min_latitude: Some(46.78489), max_longitude: Some(7.1838299), min_longitude: Some(7.1357)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون فريبورغ"), ("be", "Кантон Фрыбур"), ("bg", "Фрибур"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অফ ফিরব\u{9be}র\u{9cd}গ"), ("bs", "Fribourg"), ("ca", "Cantó de Friburg"), ("ccp", "𑄜\u{11133}𑄢\u{11128}𑄝\u{11127}𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Canton de Fribourg"), ("cs", "Fribourg"), ("cy", "Fribourg"), ("da", "Kanton Fribourg"), ("de", "Freiburg"), ("el", "Κάντον οφ Φρίμπουργκ"), ("en", "Fribourg"), ("es", "Friburgo"), ("et", "Fribourgi kanton"), ("eu", "Friburgo kantonamendua"), ("fa", "ایالت فریبورگ"), ("fi", "Fribourg"), ("fr", "canton de Fribourg"), ("gl", "Cantón de Friburgo"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ ફ\u{acd}રિબર\u{acd}ગ"), ("he", "פריבור"), ("hi", "फ\u{93c}\u{94d}राइब\u{942}र\u{94d}ग क\u{948}न\u{94d}टन"), ("hr", "Fribourg"), ("hu", "Fribourg kanton"), ("id", "Kanton Fribourg"), ("is", "Fribourg"), ("it", "Canton Friburgo"), ("ja", "フリブール州"), ("jv", "Kanton Fribourg"), ("ka", "ფრიბურის კანტონი"), ("kk", "Фрибур"), ("kn", "ಫ\u{ccd}ರ\u{cbf}ಬೋರ\u{ccd}ಗ\u{ccd}ನ ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "프리부르 주"), ("lt", "Fribūro kantonas"), ("lv", "Fribūras kantons"), ("mk", "Фрибур"), ("mr", "फ\u{94d}रिबोर\u{94d}ग"), ("ms", "Fribourg"), ("nb", "Fribourg"), ("nl", "Fribourg"), ("no", "Fribourg"), ("pl", "Fryburg"), ("pt", "Friburgo"), ("ro", "Cantonul Fribourg"), ("ru", "Фрибур"), ("si", "ෆ\u{dca}ර\u{dca}ය\u{dd2}බර\u{dca}ග\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Fribourg"), ("sq", "Kantoni Fribourg"), ("sr", "Кантон Фрибур"), ("sr_Latn", "Kanton Fribur"), ("sv", "Fribourg"), ("sw", "Jimbo la Fribourg"), ("ta", "கேண\u{bcd}டோன\u{bcd} ஆப\u{bcd} பிரிபெர\u{bcd}க\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} ఫ\u{c4d}ర\u{c48}బ\u{c4b}ర\u{c4d}గ\u{c4d}"), ("th", "ร\u{e31}ฐฟร\u{e35}บ\u{e39}ร\u{e4c}"), ("tr", "Fribourg"), ("uk", "Фрібур"), ("ur", "کینٹن فریبور"), ("vi", "Fribourg"), ("yue", "傅賴堡州"), ("yue_Hans", "傅赖堡州"), ("zh", "弗里堡州")]),
                        unofficial_name_list: ["Freiburg"].to_vec(),
                    }
                ),
                (
                    "GE",
                    Subdivision{
                        name: "GE",
                        country_alpha2: Alpha2::CH,
                        code: "GE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1983922), longitude: Some(6.142296099999999), max_latitude: Some(46.232399), min_latitude: Some(46.17766), max_longitude: Some(6.177856999999999), min_longitude: Some(6.1103201)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون جنيف"), ("be", "Кантон Жэнева"), ("bg", "Женева"), ("bs", "Ženeva"), ("ca", "Cantó de Ginebra"), ("ccp", "𑄎𑄬𑄚𑄬𑄞"), ("ceb", "Genève (kanton)"), ("cs", "Ženeva"), ("cy", "Genefa"), ("da", "Kanton Genève"), ("de", "Genf"), ("en", "Geneva"), ("es", "Cantón de Ginebra"), ("et", "Genfi kanton"), ("eu", "Geneva kantonamendua"), ("fa", "ایالت ژنو"), ("fi", "Geneve"), ("fr", "canton de Genève"), ("gl", "Cantón de Xenebra"), ("he", "ז׳נבה"), ("hi", "जनीवा क\u{948}न\u{94d}टन"), ("hr", "Kanton Ženeva"), ("hu", "Genf kanton"), ("hy", "Ժնև"), ("id", "Kanton Jenewa"), ("it", "Canton Ginevra"), ("ja", "ジュネーヴ州"), ("jv", "Kanton Jenéwa"), ("ka", "ჟენევის კანტონი"), ("kk", "Женева"), ("ko", "제네바 주"), ("lt", "Ženevos kantonas"), ("lv", "Ženēvas kantons"), ("mk", "Женева"), ("mr", "जिनिव\u{94d}हा"), ("ms", "Kanton Geneva"), ("nb", "Genève"), ("nl", "Genève"), ("no", "Genève"), ("pl", "Genewa"), ("pt", "Genebra (cantão)"), ("ro", "Geneva"), ("ru", "Женева"), ("sk", "Ženeva"), ("sl", "Kanton Ženeva"), ("sq", "Kantoni Gjenevë"), ("sr", "Кантон Женева"), ("sr_Latn", "Kanton Ženeva"), ("sv", "Genève"), ("sw", "Jimbo la Geneva"), ("th", "ร\u{e31}ฐเจน\u{e35}วา"), ("tr", "Cenevre"), ("uk", "Женева"), ("ur", "کینٹن جنیوا"), ("vi", "Genève"), ("yue", "日內瓦州"), ("yue_Hans", "日内瓦州"), ("zh", "日內瓦州")]),
                        unofficial_name_list: ["Genf", "Genève", "Ginebra", "Ginevra"].to_vec(),
                    }
                ),
                (
                    "GL",
                    Subdivision{
                        name: "GL",
                        country_alpha2: Alpha2::CH,
                        code: "GL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.0404265), longitude: Some(9.0672085), max_latitude: Some(47.1739299), min_latitude: Some(46.7963601), max_longitude: Some(9.25249), min_longitude: Some(8.871229999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kanton Glarus"), ("ar", "كانتون غلاروس"), ("be", "Кантон Гларус"), ("bg", "Гларус"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অব গ\u{9cd}ল\u{9be}র\u{9c1}স"), ("bs", "Glarus"), ("ca", "Cantó de Glarus"), ("ccp", "𑄉\u{11133}𑄣𑄢𑄌\u{11134}"), ("ceb", "Kanton Glarus"), ("cs", "Glarus"), ("cy", "Glarus"), ("da", "Kanton Glarus"), ("de", "Glarus"), ("el", "Κάντον οφ Γκλάρους"), ("en", "Glarus"), ("es", "Cantón de Glaris"), ("et", "Glaruse kanton"), ("eu", "Glaris kantonamendua"), ("fa", "ایالت گلاروس"), ("fi", "Glarus"), ("fr", "canton de Glaris"), ("gl", "Cantón de Glarus"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ ગ\u{acd}લ\u{ac7}ર\u{ac1}સ"), ("he", "גלרוס"), ("hi", "ग\u{94d}लार\u{941}स क\u{948}न\u{94d}टन"), ("hr", "Glarus"), ("hu", "Glarus kanton"), ("id", "Kanton Glarus"), ("is", "Glarus"), ("it", "Canton Glarona"), ("ja", "グラールス州"), ("jv", "Canton Glarus"), ("ka", "გლარუსის კანტონი"), ("kk", "Гларус"), ("kn", "ಗ\u{ccd}ಲಾರಸ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "글라루스 주"), ("lt", "Glaruso kantonas"), ("lv", "Glarusas kantons"), ("mk", "Гларус"), ("mr", "ग\u{94d}लार\u{941}स"), ("ms", "Canton of Glarus"), ("nb", "Glarus"), ("nl", "Glarus"), ("no", "Glarus"), ("pl", "Glarus"), ("pt", "Glarus"), ("ro", "Cantonul Glarus"), ("ru", "Гларус"), ("si", "ග\u{dca}ලරස\u{dca} කල\u{dcf}පය"), ("sk", "Glarus"), ("sq", "Kantoni Glarus"), ("sr", "Кантон Гларус"), ("sr_Latn", "Kanton Glarus"), ("sv", "Glarus"), ("sw", "Jimbo la Glarus"), ("ta", "கண\u{bcd}டோம\u{bcd} ஆப\u{bcd} கிளறுஸ\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} గ\u{c4d}ల\u{c3e}రస\u{c4d}"), ("th", "ร\u{e31}ฐกลาร\u{e38}ส"), ("tr", "Glarus"), ("uk", "Гларус"), ("ur", "کینٹن گلاروس"), ("vi", "Glarus"), ("yue", "格拉魯斯州"), ("yue_Hans", "格拉鲁斯州"), ("zh", "格拉魯斯州")]),
                        unofficial_name_list: ["Glaris"].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::CH,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.65698709999999), longitude: Some(9.578025700000001), max_latitude: Some(47.06496), min_latitude: Some(46.1690499), max_longitude: Some(10.4923401), min_longitude: Some(8.6509399)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون غراوبوندن"), ("be", "Кантон Граўбюндэн"), ("bg", "Граубюнден"), ("bn", "গ\u{9cd}র\u{9be}উব\u{9be}ন\u{9cd}ডেন"), ("bs", "Graubünden"), ("ca", "Grisons"), ("ccp", "𑄉\u{11133}𑄢\u{1112a}𑄝𑄚\u{11134}𑄓𑄬𑄚\u{11134}"), ("ceb", "Kanton Graubünden"), ("cs", "Graubünden"), ("cy", "Canton y Grisons"), ("da", "Graubünden"), ("de", "Graubünden"), ("el", "Γκραουμπούντεν"), ("en", "Graubünden"), ("es", "Grisones"), ("et", "Graubündeni kanton"), ("eu", "Grisonia"), ("fa", "ایالت گراوبوندن"), ("fi", "Graubünden"), ("fr", "canton des Grisons"), ("gl", "Grisóns"), ("gu", "ગ\u{acd}ર\u{ac7}ઉબ\u{ac1}ન\u{acd}ડન"), ("he", "גראובינדן"), ("hi", "ग\u{94d}राउब\u{941}न\u{94d}डन क\u{948}न\u{94d}टन"), ("hr", "Graubünden"), ("hu", "Graubünden kanton"), ("hy", "Գրաուբյուդեն"), ("id", "Kanton Graubünden"), ("is", "Graubünden"), ("it", "Cantone dei Grigioni"), ("ja", "グラウビュンデン州"), ("jv", "Graubünden"), ("ka", "გრაუბიუნდენის კანტონი"), ("kk", "Граубюнден"), ("kn", "ಗ\u{ccd}ರ\u{ccc}ಬುಂಡ\u{cc6}ನ\u{ccd}"), ("ko", "그라우뷘덴 주"), ("lt", "Graubiundenas"), ("lv", "Graubindenes kantons"), ("mk", "Граубинден"), ("mr", "ग\u{94d}राउब\u{94d}य\u{941}\u{902}डन"), ("ms", "Graubünden"), ("nb", "Graubünden"), ("nl", "Graubünden"), ("no", "Graubünden"), ("pl", "Gryzonia"), ("pt", "Grisões"), ("ro", "Cantonul Grisunilor"), ("ru", "Граубюнден"), ("si", "ග\u{dca}රෞබ\u{dd4}න\u{dca}ඩෙන\u{dca}"), ("sk", "Graubünden"), ("sl", "Graubünden"), ("sq", "Kantoni Graubünden"), ("sr", "Кантон Граубинден"), ("sr_Latn", "Kanton Graubinden"), ("sv", "Graubünden"), ("sw", "Graubünden"), ("ta", "குறூபுண\u{bcd}டேன\u{bcd}"), ("te", "గ\u{c4d}ర\u{c4c}బుండ\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐเกราบ\u{e36}นเด\u{e34}น"), ("tr", "Graubünden"), ("uk", "Граубюнден"), ("ur", "گراوبوندن"), ("vi", "Graubünden"), ("yue", "格留邊登州"), ("yue_Hans", "格留边登州"), ("zh", "格勞賓登州")]),
                        unofficial_name_list: ["Grigioni", "Grisons"].to_vec(),
                    }
                ),
                (
                    "JU",
                    Subdivision{
                        name: "JU",
                        country_alpha2: Alpha2::CH,
                        code: "JU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3444474), longitude: Some(7.143060800000001), max_latitude: Some(47.50452), min_latitude: Some(47.15047999999999), max_longitude: Some(7.55821), min_longitude: Some(6.84042)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون جورا"), ("be", "Кантон Юра"), ("bg", "Юра"), ("bn", "জ\u{9c1}র\u{9be}"), ("bs", "Kanton Jura"), ("ca", "Cantó del Jura"), ("ccp", "𑄎\u{1112a}𑄢"), ("ceb", "Kanton Jura"), ("cs", "Jura"), ("cy", "Canton Jura"), ("da", "Kanton Jura"), ("de", "Jura"), ("el", "Τζούρα"), ("en", "Jura"), ("es", "Jura"), ("et", "Jura kanton"), ("eu", "Jura kantonamendua"), ("fa", "ایالت ژورا"), ("fi", "Jura"), ("fr", "canton du Jura"), ("gl", "Cantón de Xura"), ("gu", "જ\u{ac1}રા"), ("he", "ז׳ורה"), ("hi", "ज\u{942}रा क\u{948}न\u{94d}टन"), ("hr", "Jura"), ("hu", "Jura kanton"), ("id", "Kanton Jura"), ("is", "Júra"), ("it", "Canton Giura"), ("ja", "ジュラ州"), ("jv", "Kanton Jura"), ("ka", "იურის კანტონი"), ("kk", "Кантон Юра"), ("kn", "ಜ\u{cc2}ರಾ"), ("ko", "쥐라 주"), ("lt", "Juros kantonas"), ("lv", "Juras kantons"), ("mk", "Кантон Јура"), ("mr", "य\u{941}रा"), ("ms", "Wilayah Jura"), ("nb", "Jura"), ("nl", "Jura"), ("no", "Jura"), ("pl", "Jura"), ("pt", "Jura"), ("ro", "Cantonul Jura"), ("ru", "Юра"), ("si", "ජ\u{dd4}ර\u{dcf}"), ("sk", "Jura"), ("sr", "Кантон Јура"), ("sr_Latn", "Kanton Jura"), ("sv", "Jura"), ("sw", "Jimbo la Jura"), ("ta", "ஜூர\u{bbe}"), ("te", "జూర\u{c3e}"), ("th", "ร\u{e31}ฐช\u{e39}รา"), ("tr", "Jura"), ("uk", "Юра"), ("ur", "کینٹن جورا"), ("vi", "Jura"), ("yue", "茹拉州"), ("yue_Hans", "茹拉州"), ("zh", "汝拉州")]),
                        unofficial_name_list: ["Jura (fr)"].to_vec(),
                    }
                ),
                (
                    "LU",
                    Subdivision{
                        name: "LU",
                        country_alpha2: Alpha2::CH,
                        code: "LU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.0500376), longitude: Some(8.3089295), max_latitude: Some(47.08349), min_latitude: Some(47.02601989999999), max_longitude: Some(8.358139999999999), min_longitude: Some(8.206470099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون لوسيرن"), ("be", "Кантон Люцэрн"), ("bg", "Люцерн"), ("bn", "ল\u{9c1}সেন ক\u{9cd}য\u{9be}ন\u{9cd}টন"), ("bs", "Lucern"), ("ca", "Cantó de Lucerna"), ("ccp", "𑄣\u{1112a}𑄥𑄬𑄢\u{11134}𑄚𑄬"), ("ceb", "Kanton Luzern"), ("cs", "Lucern"), ("cy", "Lucerne"), ("da", "Kanton Luzern"), ("de", "Luzern"), ("el", "Λουζέμ"), ("en", "Lucerne"), ("es", "Lucerna"), ("et", "Luzerni kanton"), ("eu", "Luzerna kantonamendua"), ("fa", "ایالت لوسرن"), ("fi", "Luzern"), ("fr", "canton de Lucerne"), ("gl", "Cantón de Lucerna"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ લ\u{acd}ય\u{ac1}સ\u{ac7}ર\u{acd}ન"), ("he", "לוצרן"), ("hi", "ल\u{942}सर\u{94d}न क\u{948}न\u{94d}टन"), ("hr", "Luzern"), ("hu", "Luzern kanton"), ("id", "Kanton Luzern"), ("it", "Canton Lucerna"), ("ja", "ルツェルン州"), ("ka", "ლუცერნის კანტონი"), ("kk", "Люцерн"), ("kn", "ಲ\u{ccd}ಯ\u{cc2}ಸರ\u{ccd}ನ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "루체른 주"), ("lt", "Liucernos kantonas"), ("lv", "Lucernas kantons"), ("mk", "Луцерн"), ("mr", "ल\u{941}त\u{94d}सर\u{94d}न"), ("ms", "Lucerne"), ("nb", "Luzern"), ("nl", "Luzern"), ("no", "Luzern"), ("pl", "Lucerna"), ("pt", "Lucerna"), ("ro", "Cantonul Lucerna"), ("ru", "Люцерн"), ("si", "ල\u{dd4}සර\u{dca}නේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Luzern"), ("sl", "Kanton Lucern"), ("sq", "Kantoni Luzern"), ("sr", "Кантон Луцерн"), ("sr_Latn", "Kanton Lucern"), ("sv", "Luzern"), ("sw", "Jimbo la Lucerne"), ("ta", "கேண\u{bcd}டோன\u{bcd} ஆப\u{bcd} லூசெர\u{bcd}ன\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} లూసర\u{c4d}న\u{c4d}"), ("th", "ล\u{e39}เซ\u{e34}ร\u{e4c}น"), ("tr", "Luzern"), ("uk", "Люцерн"), ("ur", "کینٹن لوتسیرن"), ("vi", "Luzern"), ("yue", "琉森州"), ("yue_Hans", "琉森州"), ("zh", "卢塞恩州")]),
                        unofficial_name_list: ["Lucerna", "Lucerne"].to_vec(),
                    }
                ),
                (
                    "NE",
                    Subdivision{
                        name: "NE",
                        country_alpha2: Alpha2::CH,
                        code: "NE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.99297929999999), longitude: Some(6.931932499999999), max_latitude: Some(47.06389), min_latitude: Some(46.97817810000001), max_longitude: Some(6.99177), min_longitude: Some(6.893409999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون نيوشاتل"), ("be", "Кантон Нёўшатэль"), ("bg", "Нюшател"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অফ ন\u{9c1}চ\u{9be}চেল"), ("bs", "Neuchâtel"), ("ca", "Cantó de Neuchâtel"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄌𑄑𑄬𑄣\u{11134}"), ("ceb", "Neuchâtel (kanton)"), ("cs", "Neuchâtel"), ("cy", "Neuchâtel"), ("da", "Kanton Neuchâtel"), ("de", "Neuenburg"), ("el", "Καντόνι του Νεσατέλ"), ("en", "Neuchâtel"), ("es", "Neuchâtel"), ("et", "Neuchâteli kanton"), ("eu", "Neuchâtel kantonamendua"), ("fa", "ایالت نوشاتل"), ("fi", "Neuchâtel"), ("fr", "canton de Neuchâtel"), ("gl", "Cantón de Neuchâtel"), ("gu", "નૌચ\u{ac8}ટ\u{ac7}લ ક\u{ac7}ન\u{acd}ટન"), ("he", "נשאטל"), ("hi", "नोशात\u{947}ल क\u{948}न\u{94d}टन"), ("hr", "Neuchâtel"), ("hu", "Neuchâtel kanton"), ("id", "Kanton Neuchâtel"), ("is", "Neuchatel"), ("it", "Canton Neuchâtel"), ("ja", "ヌーシャテル州"), ("jv", "Canton Neuchâtel"), ("ka", "ნევშატელის კანტონი"), ("kk", "Нёвшатель"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ಚಟ\u{cc6}ಲ\u{ccd}ನ ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "뇌샤텔 주"), ("lt", "Nešatelio kantonas"), ("lv", "Neišateles kantons"), ("mk", "Нешател"), ("mr", "न\u{942}शात\u{947}ल"), ("ms", "Neuchâtel"), ("nb", "Neuchâtel"), ("nl", "Neuchâtel"), ("no", "Neuchâtel"), ("pl", "Neuchâtel"), ("pt", "Neuchâtel"), ("ro", "Cantonul Neuchâtel"), ("ru", "Невшатель"), ("si", "නෙය\u{dd4}ච\u{dcf}ටෙල\u{dca} පළ\u{dcf}ත"), ("sk", "Neuchâtel"), ("sl", "Kanton Neuchâtel"), ("sq", "Kantoni Neuchâtel"), ("sr", "Кантон Нешател"), ("sr_Latn", "Kanton Nešatel"), ("sv", "Neuchâtel"), ("sw", "Jimbo la Neuchâtel"), ("ta", "க\u{bbe}ண\u{bcd}டோன\u{bcd} ஆப\u{bcd} னேஉச\u{bcd}சிட\u{bcd}டல\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} న\u{c4d}యూచ\u{c3e}ట\u{c46}ల\u{c4d}"), ("th", "ร\u{e31}ฐเนอชาแตล"), ("tr", "Neuchâtel"), ("uk", "Невшатель"), ("ur", "کینٹن نوشاتل"), ("vi", "Bang Neuchâtel"), ("yue", "新城堡州"), ("yue_Hans", "新城堡州"), ("zh", "納沙泰爾州")]),
                        unofficial_name_list: ["Neuenburg"].to_vec(),
                    }
                ),
                (
                    "NW",
                    Subdivision{
                        name: "NW",
                        country_alpha2: Alpha2::CH,
                        code: "NW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.9267016), longitude: Some(8.3849982), max_latitude: Some(47.0036169), min_latitude: Some(46.77149), max_longitude: Some(8.57489), min_longitude: Some(8.2182101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون نيدفالدن"), ("be", "Кантон Нідвальдэн"), ("bg", "Нидвалден"), ("bn", "নিডওয\u{9bc}ডেন"), ("bs", "Nidwalden"), ("ca", "Nidwalden"), ("ccp", "𑄚\u{11128}𑄖\u{11134}𑄤𑄣\u{11134}𑄓𑄬𑄚\u{11134}"), ("ceb", "Nidwalden"), ("cs", "Nidwalden"), ("cy", "Nidwalden"), ("da", "Nidwalden"), ("de", "Nidwalden"), ("el", "Νιντγουάλντεν"), ("en", "Nidwalden"), ("es", "Nidwalden"), ("et", "Nidwalden"), ("eu", "Nidwalden"), ("fa", "ایالت نیدوالدن"), ("fi", "Nidwalden"), ("fr", "canton de Nidwald"), ("gl", "Nidwalden"), ("gu", "નિદ\u{acd}વાલડ\u{ac7}ન"), ("he", "נידוולדן"), ("hi", "नीडवाल\u{94d}डन क\u{948}न\u{94d}टन"), ("hr", "Nidwalden"), ("hu", "Nidwalden kanton"), ("hy", "Նիդվալդեն"), ("id", "Kanton Nidwalden"), ("it", "Canton Nidvaldo"), ("ja", "ニトヴァルデン準州"), ("ka", "ნიდვალდენის კანტონი"), ("kk", "Нидвальден"), ("kn", "ನ\u{cbf}ಡ\u{ccd}ವಾಲ\u{ccd}ಡ\u{cc6}ನ\u{ccd}"), ("ko", "니트발덴 주"), ("lt", "Nidvaldenas"), ("lv", "Nidvaldenes kantons"), ("mk", "Нидвалден"), ("mr", "निडवाल\u{94d}डन"), ("ms", "Nidwalden"), ("nb", "Nidwalden"), ("nl", "Nidwalden"), ("no", "Nidwalden"), ("pl", "Nidwalden"), ("pt", "Nidwald"), ("ro", "Cantonul Nidwald"), ("ru", "Нидвальден"), ("si", "න\u{dd2}ඩ\u{dca}වල\u{dca}දෙන\u{dca}"), ("sk", "Nidwalden"), ("sr", "Кантон Нидвалден"), ("sr_Latn", "Kanton Nidvalden"), ("sv", "Nidwalden"), ("sw", "Nidwalden"), ("ta", "நிட\u{bcd}வல\u{bcd}டென\u{bcd}"), ("te", "న\u{c3f}డ\u{c4d}వ\u{c3e}ల\u{c4d}జ\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐน\u{e34}ดว\u{e31}ลเด\u{e34}น"), ("tr", "Nidwalden"), ("uk", "Нідвальден"), ("ur", "نیدوالدن"), ("vi", "Nidwalden"), ("yue", "下華登州"), ("yue_Hans", "下华登州"), ("zh", "下瓦爾登州")]),
                        unofficial_name_list: ["Nidwald"].to_vec(),
                    }
                ),
                (
                    "OW",
                    Subdivision{
                        name: "OW",
                        country_alpha2: Alpha2::CH,
                        code: "OW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.877858), longitude: Some(8.251249), max_latitude: Some(46.98021), min_latitude: Some(46.75307), max_longitude: Some(8.506639999999999), min_longitude: Some(8.042349999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون أوبفالدن"), ("be", "Кантон Обвальдэн"), ("bg", "Обвалден"), ("bn", "ওবেল\u{9cd}ডেন"), ("bs", "Obwalden"), ("ca", "Obwalden"), ("ccp", "𑄃\u{11127}𑄛\u{11134}𑄤𑄣\u{11134}𑄓𑄬𑄚\u{11134}"), ("ceb", "Kanton Obwalden"), ("cs", "Obwalden"), ("cy", "Obwalden"), ("da", "Obwalden"), ("de", "Obwalden"), ("el", "Ομπγουάλντεν"), ("en", "Obwalden"), ("es", "Obwalden"), ("et", "Obwalden"), ("eu", "Obwalden"), ("fa", "ایالت ابوالدن"), ("fi", "Obwalden"), ("fr", "canton d’Obwald"), ("gl", "Obwalden"), ("gu", "ઓબ\u{acd}વાલ\u{acd}ડન"), ("he", "אובוולדן"), ("hi", "ओबवाल\u{94d}डन क\u{948}न\u{94d}टन"), ("hr", "Obwalden"), ("hu", "Obwalden kanton"), ("id", "Kanton Obwalden"), ("it", "Canton Obvaldo"), ("ja", "オプヴァルデン準州"), ("ka", "ობვალდენის კანტონი"), ("kk", "Обвальден"), ("kn", "ಓಬ\u{ccd}ವಾಲ\u{ccd}ಡ\u{cc6}ನ\u{ccd}"), ("ko", "옵발덴 주"), ("lt", "Obvaldenas"), ("lv", "Obvaldenes kantons"), ("mk", "Обвалден"), ("mr", "ओबवाल\u{94d}डन"), ("ms", "Obwalden"), ("nb", "Obwalden"), ("ne", "ओब\u{94d}वाल\u{94d}ड\u{947}न"), ("nl", "Obwalden"), ("no", "Obwalden"), ("pl", "Obwalden"), ("pt", "Obwald"), ("ro", "Cantonul Obwald"), ("ru", "Обвальден"), ("si", "ඔබ\u{dca}වල\u{dca}ඩන\u{dca}"), ("sk", "Obwalden"), ("sq", "Kantoni Obwalden"), ("sr", "Кантон Обвалден"), ("sr_Latn", "Kanton Obvalden"), ("sv", "Obwalden"), ("sw", "Obwalden"), ("ta", "உபவ\u{bbe}ல\u{bcd}டேன\u{bcd}"), ("te", "ఓబ\u{c4d}వ\u{c3e}ల\u{c4d}డ\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐออบว\u{e31}ลเด\u{e34}น"), ("tr", "Obwalden"), ("uk", "Обвальден"), ("ur", "اوبوالدن"), ("vi", "Obwalden"), ("yue", "上華登州"), ("yue_Hans", "上华登州"), ("zh", "上瓦爾登州")]),
                        unofficial_name_list: ["Obwald"].to_vec(),
                    }
                ),
                (
                    "SG",
                    Subdivision{
                        name: "SG",
                        country_alpha2: Alpha2::CH,
                        code: "SG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.4179284), longitude: Some(9.3643968), max_latitude: Some(47.4530299), min_latitude: Some(47.3950999), max_longitude: Some(9.4353001), min_longitude: Some(9.29144)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kanton St. Gallen"), ("ar", "كانتون سانت غالن"), ("be", "Кантон Санкт-Гален"), ("bg", "Санкт Гален"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন সেন\u{9cd}ট গ\u{9cd}য\u{9be}লেন"), ("bs", "St. Gallen"), ("ca", "Cantó de Sankt Gallen"), ("ccp", "𑄥𑄬𑄚\u{11133}𑄑\u{11134} 𑄉𑄣𑄬𑄚\u{11134}"), ("ceb", "Kanton St. Gallen"), ("cs", "Sankt Gallen"), ("cy", "St. Gallen"), ("da", "Kanton Sankt Gallen"), ("de", "St. Gallen"), ("el", "Κάντον οφ Σαίντ Γκαλέν"), ("en", "St. Gallen"), ("es", "Cantón de San Galo"), ("et", "Sankt Galleni kanton"), ("eu", "Sankt Gallen kantonamendua"), ("fa", "کانتون سانکت گالن"), ("fi", "Sankt Gallen"), ("fr", "canton de Saint-Gall"), ("gl", "Cantón de San Galo"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ સ\u{ac7}ન\u{acd}ટ ગ\u{ac7}લન"), ("he", "סנט גאלן"), ("hi", "सा\u{902}क\u{94d}त गालन क\u{948}न\u{94d}टन"), ("hr", "St. Gallen"), ("hu", "Sankt Gallen kanton"), ("id", "Kanton St. Gallen"), ("is", "St. Gallen"), ("it", "Canton San Gallo"), ("ja", "ザンクト・ガレン州"), ("jv", "Kanton St. Gallen"), ("ka", "სანქტ-გალენის კანტონი"), ("kk", "Санкт-Галлен"), ("kn", "ಸೇಂಟ\u{ccd} ಗ\u{ccd}ಯಾಲ\u{cc6}ನ\u{ccd}ನ ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "장크트갈렌 주"), ("lt", "Sankt Galeno kantonas"), ("lv", "Sanktgallenes kantons"), ("mk", "Санкт Гален"), ("mr", "सा\u{902}क\u{94d}ट गाल\u{947}न"), ("ms", "St. Gallen"), ("nb", "Sankt Gallen"), ("nl", "Sankt Gallen"), ("no", "Sankt Gallen"), ("pl", "St. Gallen"), ("pt", "São Galo"), ("ro", "Cantonul St. Gallen"), ("ru", "Санкт-Галлен"), ("si", "ශ\u{dcf}න\u{dca}ත ගල\u{dca}ලෙන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sankt Gallen"), ("sq", "Kantoni St. Gallen"), ("sr", "Кантон Санкт Гален"), ("sr_Latn", "Kanton Sankt Galen"), ("sv", "Sankt Gallen"), ("sw", "Jimbo la St. Gallen"), ("ta", "கேண\u{bcd}டோன\u{bcd} ஆப\u{bcd} செயின\u{bcd}ட\u{bcd}. கல\u{bcd}லென\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} స\u{c46}య\u{c3f}ంట\u{c4d} గ\u{c3e}ల\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐซ\u{e31}งคท\u{e4c}ก\u{e31}ลเล\u{e34}น"), ("tr", "St. Gallen"), ("uk", "Санкт-Галлен"), ("ur", "کینٹن سانکت گالن"), ("vi", "Bang St. Gallen"), ("yue", "聖加倫州"), ("yue_Hans", "圣加伦州"), ("zh", "聖加侖州")]),
                        unofficial_name_list: ["Saint Galle", "Saint-Gall", "Sankt Gallen"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::CH,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.7077664), longitude: Some(8.641442399999999), max_latitude: Some(47.74462), min_latitude: Some(47.68589), max_longitude: Some(8.7058599), min_longitude: Some(8.585830099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون شافهوزن"), ("be", "Кантон Шафхаўзен"), ("bg", "Шафхаузен"), ("bn", "স\u{9cd}ক\u{9cd}য\u{9be}ফহোসেন ক\u{9cd}য\u{9be}ন\u{9cd}টন"), ("bs", "Schaffhausen"), ("ca", "Cantó de Schaffhausen"), ("ccp", "𑄌\u{11133}𑄇𑄛\u{11134}𑄦𑄅\u{1112a}𑄥𑄬𑄚\u{11134}"), ("ceb", "Kanton Schaffhausen"), ("cs", "Schaffhausen"), ("cy", "Schaffhausen"), ("da", "Kanton Schaffhausen"), ("de", "Schaffhausen"), ("el", "Σαφχάουζεν (καντόνι)"), ("en", "Schaffhausen"), ("es", "Schaffhausen"), ("et", "Schaffhauseni kanton"), ("eu", "Schaffhausen kantonamendua"), ("fa", "ایالت شاف هاوزن"), ("fi", "Schaffhausen"), ("fr", "canton de Schaffhouse"), ("gl", "Cantón de Schaffhausen"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ સ\u{acd}કહાફહૌસન"), ("he", "שפהאוזן"), ("hi", "शाफ\u{93c}हाउसन क\u{948}न\u{94d}टन"), ("hr", "Schaffhausen"), ("hu", "Schaffhausen kanton"), ("id", "Kanton Schaffhausen"), ("it", "Canton Sciaffusa"), ("ja", "シャフハウゼン州"), ("ka", "შაფჰაუზენის კანტონი"), ("kk", "Шаффхаузен"), ("kn", "ಸ\u{ccd}ಕಾಫ\u{ccd}ಹ\u{ccc}ಸ\u{cc6}ನ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "샤프하우젠 주"), ("lt", "Šafhauzeno kantonas"), ("lv", "Šafhauzenes kantons"), ("mk", "Шафхаузен"), ("mr", "शाफहाउजन"), ("ms", "Canton of Schaffhausen"), ("nb", "Schaffhausen"), ("nl", "Schaffhausen"), ("no", "Schaffhausen"), ("pl", "Szafuza"), ("pt", "Schaffhausen"), ("ro", "Cantonul Schaffhausen"), ("ru", "Шаффхаузен"), ("si", "ස\u{dca}කෆ\u{dca}හව\u{dd4}සෙන\u{dca} පළ\u{dcf}ත"), ("sk", "Schaffhausen"), ("sr", "Кантон Шафхаузен"), ("sr_Latn", "Kanton Šafhauzen"), ("sv", "Schaffhausen"), ("sw", "Jimbo la Schaffhausen"), ("ta", "க\u{bbe}ண\u{bcd}டோன\u{bcd} ஆப\u{bcd} ஸ\u{bcd}சஆஹூசேன\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆప\u{c4d} ష\u{c3e}ఫ\u{c4d}\u{200c}హ\u{c3e}స\u{c46}న\u{c4d}"), ("th", "ชาฟฟ\u{e4c}เฮาเซ\u{e34}น"), ("tr", "Schaffhausen"), ("uk", "Шаффгаузен"), ("ur", "کینٹن شافہاوزن"), ("vi", "Schaffhausen"), ("yue", "沙夫候臣州"), ("yue_Hans", "沙夫候臣州"), ("zh", "沙夫豪森州")]),
                        unofficial_name_list: ["Schaffhouse"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::CH,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2086574), longitude: Some(7.5379549), max_latitude: Some(47.22025), min_latitude: Some(47.19591), max_longitude: Some(7.552230000000001), min_longitude: Some(7.50923)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون سولوتورن"), ("be", "Кантон Залатурн"), ("bg", "Золотурн"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অব সোলোথ\u{9be}ম"), ("bs", "Solothurn"), ("ca", "Cantó de Solothurn"), ("ccp", "𑄥\u{11127}𑄣\u{11127}𑄗𑄢\u{11134}𑄚\u{11134}"), ("ceb", "Kanton Solothurn"), ("cs", "Solothurn"), ("cy", "Solothurn"), ("da", "Kanton Solothurn"), ("de", "Solothurn"), ("el", "Κάντον οφ Σολόθουμ"), ("en", "Solothurn"), ("es", "Soleura"), ("et", "Solothurni kanton"), ("eu", "Solothurn kantonamendua"), ("fa", "ایالت سولوتهورن"), ("fi", "Solothurn"), ("fr", "canton de Soleure"), ("gl", "Cantón de Soleura"), ("gu", "ક\u{ac7}ન\u{acd}ટોન ઓફ સોલોથર\u{acd}ન"), ("he", "זולותורן"), ("hi", "सोलोथ\u{942}र\u{94d}न क\u{948}न\u{94d}टन"), ("hr", "Solothurn"), ("hu", "Solothurn kanton"), ("id", "Kanton Solothurn"), ("is", "Solothurn"), ("it", "Canton Soletta"), ("ja", "ゾロトゥルン州"), ("ka", "ზოლოთურნის კანტონი"), ("kk", "Золотурн"), ("kn", "ಕ\u{ccd}ಯಾಂಟನ\u{ccd} ಆಫ\u{ccd} ಸೋಲೋಥರ\u{ccd}ನ\u{ccd}"), ("ko", "졸로투른 주"), ("lt", "Zoloturno kantonas"), ("lv", "Zoloturnas kantons"), ("mk", "Золотурн"), ("mr", "सोलोथ\u{941}र\u{94d}न"), ("ms", "Solothurn"), ("nb", "Solothurn"), ("nl", "Solothurn"), ("no", "Solothurn"), ("pl", "Solura"), ("pt", "Soleura"), ("ro", "Cantonul Solothurn"), ("ru", "Золотурн"), ("si", "සොලොතර\u{dca}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Solothurn"), ("sq", "Kantoni Solothurn"), ("sr", "Кантон Золотурн"), ("sr_Latn", "Kanton Zoloturn"), ("sv", "Solothurn"), ("sw", "Jimbo la Solothurn"), ("ta", "க\u{bbe}ன\u{bcd}டோன\u{bcd} ஆப\u{bcd} சொலத\u{bcd}தூர\u{bcd}ன\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} స\u{c4b}ల\u{c4b}థమ\u{c4d}"), ("th", "ร\u{e31}ฐโซโลทวร\u{e4c}น"), ("tr", "Solothurn"), ("uk", "Золотурн"), ("ur", "کینٹن زولوتورن"), ("vi", "Solothurn"), ("yue", "索洛同州"), ("yue_Hans", "索洛同州"), ("zh", "索洛圖恩州")]),
                        unofficial_name_list: ["Soleure"].to_vec(),
                    }
                ),
                (
                    "SZ",
                    Subdivision{
                        name: "SZ",
                        country_alpha2: Alpha2::CH,
                        code: "SZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.0198346), longitude: Some(8.647397699999999), max_latitude: Some(47.06635), min_latitude: Some(46.98516), max_longitude: Some(8.7787901), min_longitude: Some(8.60339)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون شفيتس"), ("az", "Şvis"), ("be", "Кантон Швіц"), ("bg", "Швиц"), ("bn", "শ\u{9be}ভেজের ক\u{9cd}য\u{9be}ন\u{9cd}টন"), ("bs", "Schwyz"), ("ca", "Cantó de Schwyz"), ("ccp", "𑄌\u{11133}𑄇𑄬𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Kanton Schwyz"), ("cs", "Schwyz"), ("cy", "Schwyz"), ("da", "Kanton Schwyz"), ("de", "Schwyz"), ("el", "Στσγουίζ"), ("en", "Schwyz"), ("es", "Schwyz"), ("et", "Schwyzi kanton"), ("eu", "Schwyz kantonamendua"), ("fa", "ایالت شویتز"), ("fi", "Schwyz"), ("fr", "canton de Schwytz"), ("gl", "Cantón de Schwyz"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ શ\u{acd}વાઝ"), ("he", "שוויץ"), ("hi", "श\u{94d}वीट\u{94d}ज\u{93c} क\u{948}न\u{94d}टन"), ("hr", "Schwyz"), ("hu", "Schwyz kanton"), ("hy", "Շվից"), ("id", "Kanton Schwyz"), ("is", "Schwyz"), ("it", "Canton Svitto"), ("ja", "シュヴィーツ州"), ("jv", "Kanton Schwyz"), ("ka", "შვიცის კანტონი"), ("kk", "Швиц"), ("kn", "ಶ\u{ccd}ವ\u{cbf}ಜ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "슈비츠 주"), ("lt", "Švico kantonas"), ("lv", "Švīces kantons"), ("mk", "Швиц"), ("mr", "श\u{94d}वित\u{94d}स"), ("ms", "Kanton Schwyz"), ("nb", "Schwyz"), ("nl", "Schwyz"), ("no", "Schwyz"), ("pl", "Schwyz"), ("pt", "Schwyz"), ("ro", "Cantonul Schwyz"), ("ru", "Швиц"), ("si", "ස\u{dca}ක\u{dca}ව\u{dd2}ස\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Schwyz"), ("sq", "Schwyz"), ("sr", "Кантон Швиц"), ("sr_Latn", "Kanton Švic"), ("sv", "Schwyz"), ("sw", "Jimbo la Schwyz"), ("ta", "க\u{bbe}ண\u{bcd}டோன\u{bcd} ஆப\u{bcd} ஸ\u{bcd}ச\u{bcd}வ\u{bcd}ய\u{bcd}ஸ\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} ష\u{c4d}వ\u{c3f}జ\u{c4d}"), ("th", "ร\u{e31}ฐชว\u{e35}ซ"), ("tr", "Schwyz"), ("uk", "Швіц"), ("ur", "کینٹن شویتس"), ("vi", "Bang Schwyz"), ("yue", "舒懷茨州"), ("yue_Hans", "舒怀茨州"), ("zh", "施維茨州")]),
                        unofficial_name_list: ["Schwyz (de)"].to_vec(),
                    }
                ),
                (
                    "TG",
                    Subdivision{
                        name: "TG",
                        country_alpha2: Alpha2::CH,
                        code: "TG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.60378559999999), longitude: Some(9.0557371), max_latitude: Some(47.6954101), min_latitude: Some(47.3757001), max_longitude: Some(9.4764901), min_longitude: Some(8.66793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون تورغاو"), ("be", "Кантон Тургау"), ("bg", "Тургау"), ("bn", "থোগ\u{9be}ও"), ("bs", "Thurgau"), ("ca", "Turgòvia"), ("ccp", "𑄗𑄢\u{11134}𑄉𑄅\u{1112a}"), ("ceb", "Kanton Thurgau"), ("cs", "Thurgau"), ("cy", "Thurgau"), ("da", "Thurgau"), ("de", "Thurgau"), ("el", "Θέργκο"), ("en", "Thurgau"), ("es", "Turgovia"), ("et", "Thurgau kanton"), ("eu", "Turgovia"), ("fa", "ایالت تورگاو"), ("fi", "Thurgau"), ("fr", "canton de Thurgovie"), ("gl", "Turgovia"), ("gu", "થરગાઉ"), ("he", "תורגאו"), ("hi", "ठ\u{942}रगाउ क\u{948}न\u{94d}टन"), ("hr", "Thurgau"), ("hu", "Thurgau kanton"), ("id", "Kanton Thurgau"), ("is", "Thurgau"), ("it", "Canton Turgovia"), ("ja", "トゥールガウ州"), ("ka", "თურგაუს კანტონი"), ("kk", "Тургау"), ("kn", "ತುರ\u{ccd}ಗ\u{ccc}"), ("ko", "투르가우 주"), ("lt", "Turgau"), ("lv", "Turgavas kantons"), ("mk", "Тургау"), ("mr", "थ\u{941}र\u{94d}गाउ"), ("ms", "St. Thurgau"), ("nb", "Thurgau"), ("nl", "Thurgau"), ("no", "Thurgau"), ("pl", "Turgowia"), ("pt", "Turgóvia"), ("ro", "Cantonul Turgovia"), ("ru", "Тургау"), ("si", "ත\u{dd4}ර\u{dca}ගෞ"), ("sk", "Thurgau"), ("sl", "Thurgau"), ("sq", "Thurgau"), ("sr", "Кантон Тургау"), ("sr_Latn", "Kanton Turgau"), ("sv", "Thurgau"), ("sw", "Thurgau"), ("ta", "துர\u{bcd}க\u{bbe}வு"), ("te", "తుర\u{c4d}గ\u{c3e}వ\u{c4d}"), ("th", "ร\u{e31}ฐท\u{e31}วร\u{e4c}เกา"), ("tr", "Thurgau"), ("uk", "Тургау"), ("ur", "تھورگاو"), ("vi", "Thurgau"), ("yue", "圖爾膠州"), ("yue_Hans", "图尔胶州"), ("zh", "圖爾高州")]),
                        unofficial_name_list: ["Thurgovie"].to_vec(),
                    }
                ),
                (
                    "TI",
                    Subdivision{
                        name: "TI",
                        country_alpha2: Alpha2::CH,
                        code: "TI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.331734), longitude: Some(8.800452900000002), max_latitude: Some(46.63241), min_latitude: Some(45.81792), max_longitude: Some(9.159730000000001), min_longitude: Some(8.38218)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون تيسينو"), ("az", "Ticino"), ("be", "Кантон Тычына"), ("bg", "Тичино"), ("bn", "তিকিনো"), ("bs", "Ticino"), ("ca", "cantó de Ticino"), ("ccp", "𑄖\u{1112d}𑄌\u{11128}𑄚\u{1112e}"), ("ceb", "Ticino (kanton)"), ("cs", "Ticino"), ("cy", "Ticino"), ("da", "Ticino"), ("de", "Tessin"), ("el", "Τιτσίνο"), ("en", "Ticino"), ("es", "Tesino"), ("et", "Ticino kanton"), ("eu", "Ticino"), ("fa", "ایالت تیچینو"), ("fi", "Ticino"), ("fr", "canton du Tessin"), ("gl", "Tesino"), ("gu", "ટિસીનો"), ("he", "טיצ׳ינו"), ("hi", "तिचीनो क\u{948}न\u{94d}टन"), ("hr", "Ticino"), ("hu", "Ticino kanton"), ("hy", "Տիչինո"), ("id", "Kanton Ticino"), ("is", "Ticino"), ("it", "canton Ticino"), ("ja", "ティチーノ州"), ("jv", "Kanton Ticino"), ("ka", "ტიჩინოს კანტონი"), ("kk", "Тичино"), ("kn", "ಟ\u{cbf}ಸ\u{cbf}ನೊ"), ("ko", "티치노 주"), ("lt", "Tičinas"), ("lv", "Tičīno kantons"), ("mk", "Тичино"), ("ml", "ടിച\u{d4d}ചീനോ"), ("mr", "तिचिनो"), ("ms", "Ticino"), ("nb", "Ticino"), ("nl", "Ticino"), ("no", "Ticino"), ("pl", "Ticino"), ("pt", "Ticino"), ("ro", "Cantonul Ticino"), ("ru", "Тичино"), ("si", "ට\u{dd2}ස\u{dd2}නෝ"), ("sk", "Ticino"), ("sq", "Kantoni Ticino"), ("sr", "Кантон Тичино"), ("sr_Latn", "Kanton Tičino"), ("sv", "Ticino"), ("sw", "Ticino"), ("ta", "டிசினோ"), ("te", "ట\u{c3f}స\u{c3f}న\u{c4b}"), ("th", "ร\u{e31}ฐต\u{e35}ช\u{e35}โน"), ("tr", "Ticino"), ("uk", "Тічино"), ("ur", "تیچینو"), ("vi", "Ticino"), ("yue", "鐵千諾州"), ("yue_Hans", "铁千诺州"), ("zh", "提契諾州")]),
                        unofficial_name_list: ["Tessin", "Tessin"].to_vec(),
                    }
                ),
                (
                    "UR",
                    Subdivision{
                        name: "UR",
                        country_alpha2: Alpha2::CH,
                        code: "UR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.7738629), longitude: Some(8.602515300000002), max_latitude: Some(46.9880412), min_latitude: Some(46.52757), max_longitude: Some(8.95788), min_longitude: Some(8.397459999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون أوري"), ("be", "Кантон Уры"), ("bg", "Ури"), ("bn", "উরি"), ("bs", "Uri"), ("ca", "Uri"), ("ccp", "𑄅\u{1112a}𑄢\u{11128}"), ("ceb", "Kanton Uri"), ("cs", "Uri"), ("cy", "Uri"), ("da", "Kanton Uri"), ("de", "Uri"), ("el", "Ούρι"), ("en", "Uri"), ("es", "Uri"), ("et", "Uri kanton"), ("eu", "Uri kantonamendua"), ("fa", "ایالت اوری"), ("fi", "Uri"), ("fr", "canton d’Uri"), ("gl", "Cantón de Uri"), ("gu", "ઉરી"), ("he", "אורי"), ("hi", "ऊरी क\u{948}न\u{94d}टन"), ("hr", "Uri"), ("hu", "Uri kanton"), ("id", "Kanton Uri"), ("is", "Uri"), ("it", "Canton Uri"), ("ja", "ウーリ州"), ("ka", "ურის კანტონი"), ("kk", "Ури"), ("kn", "ಯುರ\u{cbf}"), ("ko", "우리 주"), ("lt", "Uris"), ("lv", "Ūrī kantons"), ("mk", "Ури"), ("mr", "उरी"), ("ms", "Uri"), ("nb", "Uri"), ("nl", "Uri"), ("no", "Uri"), ("pl", "Uri"), ("pt", "Uri"), ("ro", "Cantonul Uri"), ("ru", "Ури"), ("si", "ය\u{dd6}ර\u{dd2}"), ("sk", "Uri"), ("sl", "Uri"), ("sq", "Kantoni Uri"), ("sr", "Кантон Ури"), ("sr_Latn", "Kanton Uri"), ("sv", "Uri"), ("sw", "Jimbo la Uri"), ("ta", "உரி"), ("te", "ఉర\u{c3f}"), ("th", "ย\u{e39}ร\u{e34}"), ("tr", "Uri"), ("uk", "Урі"), ("ur", "کینٹن اوری"), ("vi", "Bang Uri"), ("yue", "烏里州"), ("yue_Hans", "乌里州"), ("zh", "烏里州")]),
                        unofficial_name_list: ["Uri (de)"].to_vec(),
                    }
                ),
                (
                    "VD",
                    Subdivision{
                        name: "VD",
                        country_alpha2: Alpha2::CH,
                        code: "VD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5613135), longitude: Some(6.536765), max_latitude: Some(46.98170229999999), min_latitude: Some(46.1870301), max_longitude: Some(7.2492599), min_longitude: Some(6.06401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون فود"), ("be", "Во"), ("bg", "Во"), ("bn", "ভৌড ক\u{9cd}য\u{9be}ন\u{9cd}টন"), ("bs", "Vaud"), ("ca", "Vaud"), ("ccp", "𑄞𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "Canton de Vaud"), ("cs", "Vaud"), ("cy", "Vaud"), ("da", "Vaud"), ("de", "Waadt"), ("el", "Καντόνι του Βω"), ("en", "Vaud"), ("es", "Vaud"), ("et", "Vaud’ kanton"), ("eu", "Vaud kantonamendua"), ("fa", "ایالت وو"), ("fi", "Vaud"), ("fr", "canton de Vaud"), ("gl", "Vaud"), ("gu", "ક\u{ac7}ન\u{acd}ટોન ઓફ વૌડ"), ("he", "וו"), ("hi", "वो क\u{948}न\u{94d}टन"), ("hr", "Vaud"), ("hu", "Vaud kanton"), ("id", "Kanton Vaud"), ("is", "Vaud"), ("it", "Canton Vaud"), ("ja", "ヴォー州"), ("jv", "Kanton Vaud"), ("ka", "ვოს კანტონი"), ("kk", "Во"), ("kn", "ವಾಡ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "보 주"), ("lt", "Vo"), ("lv", "Vo kantons"), ("mk", "Во"), ("mr", "व\u{94d}हो"), ("ms", "Vaud"), ("nb", "Vaud"), ("nl", "Vaud"), ("no", "Vaud"), ("pl", "Vaud"), ("pt", "Vaud"), ("ro", "Cantonul Vaud"), ("ru", "Во"), ("si", "වව\u{dd4}ඩ\u{dca} පළ\u{dcf}ත"), ("sk", "Vaud"), ("sq", "Kantoni Vaud"), ("sr", "Кантон Во"), ("sr_Latn", "Kanton Vo"), ("sv", "Vaud"), ("sw", "Vaud"), ("ta", "க\u{bbe}ண\u{bcd}டோன\u{bcd} ஆப\u{bcd} வ\u{bbe}உத\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} వ\u{c3e}డ\u{c4d}"), ("th", "ร\u{e31}ฐโว"), ("tr", "Vaud"), ("uk", "Во"), ("ur", "وو"), ("vi", "Vaud"), ("yue", "禾州"), ("yue_Hans", "禾州"), ("zh", "沃州")]),
                        unofficial_name_list: ["Waadt"].to_vec(),
                    }
                ),
                (
                    "VS",
                    Subdivision{
                        name: "VS",
                        country_alpha2: Alpha2::CH,
                        code: "VS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1904614), longitude: Some(7.5449226), max_latitude: Some(46.6539699), min_latitude: Some(45.85827), max_longitude: Some(8.4785401), min_longitude: Some(6.77046)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون فاليز"), ("be", "Кантон Вале"), ("bg", "Вале"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অব ভ\u{9cd}য\u{9be}ল\u{9be}ইস"), ("bs", "Valais"), ("ca", "Valais"), ("ccp", "𑄞𑄬𑄣\u{11128}𑄠𑄬𑄌\u{11134}"), ("ceb", "Canton du Valais"), ("cs", "Valais"), ("cy", "Valais"), ("da", "Valais"), ("de", "Wallis"), ("el", "Καντόνι του Βαλαί"), ("en", "Valais"), ("es", "cantón del Valais"), ("et", "Valais’ kanton"), ("eu", "Valais kantonamendua"), ("fa", "ایالت وله"), ("fi", "Valais"), ("fr", "canton du Valais"), ("gl", "Valais"), ("gu", "વાલાઇસ ક\u{ac7}ન\u{acd}ટન"), ("he", "ואלה"), ("hi", "वाल\u{947} क\u{948}न\u{94d}टन"), ("hr", "Valais"), ("hu", "Wallis kanton"), ("id", "Kanton Valais"), ("is", "Valais"), ("it", "Canton Vallese"), ("ja", "ヴァレー州"), ("jv", "Valais"), ("ka", "ვალეს კანტონი"), ("kk", "Вале"), ("kn", "ವಲಾಯ\u{cbf}ಸ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "발레 주"), ("lt", "Valė"), ("lv", "Valē kantons"), ("mk", "Вале"), ("mr", "व\u{94d}हाल\u{947}"), ("ms", "Valais"), ("nb", "Wallis"), ("nl", "Wallis"), ("no", "Wallis"), ("pl", "Valais"), ("pt", "Valais"), ("ro", "Cantonul Valais"), ("ru", "Вале"), ("si", "වලය\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sk", "Valais"), ("sl", "Valais"), ("sq", "Kantoni Valais"), ("sr", "Кантон Вале"), ("sr_Latn", "Kanton Vale"), ("sv", "Valais"), ("sw", "Valais"), ("ta", "வலெய\u{bcd}ஸ\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} వ\u{c3e}ల\u{c46}య\u{c3f}స\u{c4d}"), ("th", "ร\u{e31}ฐวาเล"), ("tr", "Valais"), ("uk", "Вале"), ("ur", "والے"), ("vi", "Valais"), ("yue", "華麗州"), ("yue_Hans", "华丽州"), ("zh", "瓦莱州")]),
                        unofficial_name_list: ["Vallese", "Wallis"].to_vec(),
                    }
                ),
                (
                    "ZG",
                    Subdivision{
                        name: "ZG",
                        country_alpha2: Alpha2::CH,
                        code: "ZG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.1745887), longitude: Some(8.513854), max_latitude: Some(47.18973), min_latitude: Some(47.1144752), max_longitude: Some(8.55844), min_longitude: Some(8.4754)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون تسوغ"), ("az", "Çuq"), ("be", "Кантон Цуг"), ("bg", "Цуг"), ("bn", "কেন\u{9cd}টনঅফ জগ"), ("bs", "Zug"), ("ca", "Cantó de Zug"), ("ccp", "𑄎𑄇\u{11134}"), ("ceb", "Kanton Zug"), ("cs", "Zug"), ("cy", "Zug"), ("da", "Kanton Zug"), ("de", "Zug"), ("el", "Κάντον οφ Ζάγκ"), ("en", "Zug"), ("es", "Zug"), ("et", "Zugi kanton"), ("eu", "Zug kantonamendua"), ("fa", "ایالت زوگ"), ("fi", "Zug"), ("fr", "canton de Zoug"), ("gl", "Cantón de Zug"), ("gu", "ઝ\u{ac1}ગ ક\u{ac7}ન\u{acd}ટન"), ("he", "צוג"), ("hi", "ज\u{93c}\u{942}ग क\u{948}न\u{94d}टन"), ("hr", "Zug"), ("hu", "Zug kanton"), ("id", "Kanton Zug"), ("it", "Canton Zugo"), ("ja", "ツーク州"), ("jv", "Kanton Zug"), ("ka", "ცუგის კანტონი"), ("kk", "Цуг"), ("kn", "ಝಗ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "추크 주"), ("lt", "Cugo kantonas"), ("lv", "Cūgas kantons"), ("mk", "Цуг"), ("mr", "त\u{94d}स\u{941}ग"), ("ms", "Canton of Zug"), ("nb", "Zug"), ("nl", "Zug"), ("no", "Zug"), ("pl", "Zug"), ("pt", "Zug"), ("ro", "Cantonul Zug"), ("ru", "Цуг"), ("si", "කැන\u{dca}ටන\u{dca} ඔෆ\u{dca} සග\u{dca}"), ("sk", "Zug"), ("sq", "Kantoni Zug"), ("sr", "Кантон Цуг"), ("sr_Latn", "Kanton Cug"), ("sv", "Zug"), ("sw", "Jimbo la Zug"), ("ta", "கண\u{bcd}டோம\u{bcd} ஆப\u{bcd} ஸுக\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} జుగ\u{c4d}"), ("th", "ร\u{e31}ฐซ\u{e39}ค"), ("tr", "Zug"), ("uk", "Цуг"), ("ur", "کینٹن تسوگ"), ("vi", "Bang Zug"), ("yue", "祖格州"), ("yue_Hans", "祖格州"), ("zh", "楚格州")]),
                        unofficial_name_list: ["Zoug"].to_vec(),
                    }
                ),
                (
                    "ZH",
                    Subdivision{
                        name: "ZH",
                        country_alpha2: Alpha2::CH,
                        code: "ZH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3686498), longitude: Some(8.539182499999999), max_latitude: Some(47.43468), min_latitude: Some(47.32023), max_longitude: Some(8.6253701), min_longitude: Some(8.448059899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Canton,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كانتون زيورخ"), ("be", "кантон Цюрых"), ("bg", "Цюрих"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}টন অফ জ\u{9c1}রিখ"), ("bs", "Zürich"), ("ca", "Cantó de Zuric"), ("ccp", "𑄎\u{1112a}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Kanton Zürich"), ("cs", "Curych"), ("cy", "Zürich"), ("da", "Kanton Zürich"), ("de", "Zürich"), ("el", "Κάντον οφ Ζυρίχη"), ("en", "Zürich"), ("es", "Zúrich"), ("et", "Zürichi kanton"), ("eu", "Zürich kantonamendua"), ("fa", "ایالت زوریخ"), ("fi", "Zürich"), ("fr", "canton de Zurich"), ("gl", "Cantón de Zúric"), ("gu", "ક\u{ac7}ન\u{acd}ટન ઓફ ઝ\u{acd}ય\u{ac1}રિખ"), ("he", "ציריך"), ("hi", "ज\u{93c}\u{94d}य\u{942}रिख\u{93c} क\u{948}न\u{94d}टन"), ("hr", "Zürich"), ("hu", "Zürich kanton"), ("hy", "Ցյուրիխ"), ("id", "Kanton Zürich"), ("is", "Zürich"), ("it", "Canton Zurigo"), ("ja", "チューリヒ州"), ("jv", "Kanton Zurich"), ("ka", "ციურიხის კანტონი"), ("kk", "Цюрих"), ("kn", "ಜುರ\u{cbf}ಚ\u{ccd} ಕ\u{ccd}ಯಾಂಟನ\u{ccd}"), ("ko", "취리히 주"), ("lt", "Ciuricho kantonas"), ("lv", "Cīrihes kantons"), ("mk", "Цирих"), ("mr", "झ\u{94d}य\u{941}रिक"), ("ms", "Canton of Zurich"), ("nb", "Zürich"), ("nl", "Zürich"), ("no", "Zürich"), ("pl", "Zurych"), ("pt", "Zurique"), ("ro", "Cantonul Zürich"), ("ru", "Цюрих"), ("si", "ස\u{dd6}ර\u{dd2}ච\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Zürich"), ("sl", "Kanton Zürich"), ("sq", "Kantoni Cyrih"), ("sr", "Кантон Цирих"), ("sr_Latn", "Kanton Cirih"), ("sv", "Zürich"), ("sw", "Jimbo la Zürich"), ("ta", "க\u{bbe}ன\u{bcd}டோன\u{bcd} ஆப\u{bcd} சூரிச\u{bcd}"), ("te", "క\u{c3e}ంటన\u{c4d} ఆఫ\u{c4d} జ\u{c4d}యూర\u{c3f}క\u{c4d}"), ("th", "ร\u{e31}ฐซ\u{e37}อร\u{e34}ช"), ("tr", "Zürih"), ("uk", "Цюрих"), ("ur", "کینٹن زیورخ"), ("vi", "Zürich"), ("yue", "蘇黎世州"), ("yue_Hans", "苏黎世州"), ("zh", "苏黎世州")]),
                        unofficial_name_list: ["Zurich", "Zurigo", "Zürich"].to_vec(),
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
#[cfg(feature = "ch")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CH,
        alpha3: Alpha3::CHE,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 41,
        currency_code: "CHF",
        gec: Some(GEC::SZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::SUI),
        iso_long_name: "The Swiss Confederation",
        iso_short_name: "Switzerland",
        official_language_list: ["de", "fr", "it"].to_vec(),
        spoken_language_list: ["de", "fr", "it"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Swiss"),
        number: "756",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "CH",
        unofficial_name_list: [
            "Switzerland",
            "Schweiz",
            "Suisse",
            "Suiza",
            "スイス",
            "Zwitserland",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Switzerland"),
            ("af", "Switserland"),
            ("ak", "Switzerland"),
            ("am", "ስፁፈሴሒን፥"),
            ("an", "Switzerland"),
            ("ar", "سويسرا"),
            ("as", "ছ\u{9c1}ইজ\u{9be}ৰলেণ\u{9cd}ড"),
            ("ay", "Switzerland"),
            ("az", "İsveçrə"),
            ("ba", "Switzerland"),
            ("be", "Швейцарыя"),
            ("bg", "Швейцария"),
            ("bi", "Switzerland"),
            ("bn", "স\u{9c1}ইৎজ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("bn_IN", "স\u{9c1}ইৎজ\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("br", "Suis"),
            ("bs", "Švicarska"),
            ("ca", "Suïssa"),
            ("ce", "Швейцари"),
            ("ch", "Switzerland"),
            ("cs", "Švýcarsko"),
            ("cv", "Швейцари"),
            ("cy", "Y Swistir"),
            ("da", "Schweiz"),
            ("de", "Schweiz"),
            ("dv", "ސ\u{7aa}ވ\u{7a8}ޒ\u{7a6}ލ\u{7ad}ނ\u{7b0}ޑ\u{7aa}"),
            ("dz", "ས་ཝ་ཛ\u{f72}་ལ\u{f7a}ནཌ\u{f72}།"),
            ("ee", "Switzerland"),
            ("el", "Ελβετία"),
            ("en", "Switzerland"),
            ("eo", "Svislando"),
            ("es", "Suiza"),
            ("et", "Šveits"),
            ("eu", "Suitza"),
            ("fa", "سوئیس"),
            ("ff", "Suwis"),
            ("fi", "Sveitsi"),
            ("fo", "Sveis"),
            ("fr", "Suisse"),
            ("fy", "Switserlân"),
            ("ga", "An Eilvéis"),
            ("gl", "Suíza"),
            ("gn", "Switzerland"),
            ("gu", "સ\u{acd}વિત\u{acd}ઝરલ\u{ac7}ન\u{acd}ડ"),
            ("gv", "Yn Elveeish"),
            ("ha", "Switzerland"),
            ("he", "שווייץ"),
            ("hi", "स\u{94d}विट\u{94d}ज\u{93c}रल\u{948}ण\u{94d}ड"),
            ("hr", "Švicarska"),
            ("ht", "Swis"),
            ("hu", "Svájc"),
            ("hy", "Շվեյցարիա"),
            ("ia", "Suissa"),
            ("id", "Swiss"),
            ("io", "Suisia"),
            ("is", "Sviss"),
            ("it", "Svizzera"),
            ("iu", "Switzerland"),
            ("ja", "スイス"),
            ("ka", "შვეიცარია"),
            ("ki", "Switzerland"),
            ("kk", "Швейцария"),
            ("kl", "Switzerland"),
            ("km", "ស\u{17d2}វ\u{17ca}\u{17b8}ស"),
            ("kn", "ಸ\u{ccd}ವ\u{cbf}ಜರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"),
            ("ko", "스위스"),
            ("ku", "Swîsre"),
            ("kv", "Швейцария"),
            ("kw", "Swistir"),
            ("ky", "Швейцария"),
            ("lo", "ປະເທດສະວ\u{eb4}ດ"),
            ("lt", "Šveicarija"),
            ("lv", "Šveice"),
            ("mi", "Huiterangi"),
            ("mk", "Швајцарија"),
            (
                "ml",
                "സ\u{d4d}വിറ\u{d4d}റ\u{d4d}സര\u{d4d}\u{200d}ല\u{d3e}ന\u{d4d}\u{200d}ഡ\u{d4d}",
            ),
            ("mn", "Швецарь"),
            ("mr", "स\u{94d}वित\u{94d}झर\u{94d}ल\u{902}ड"),
            ("ms", "Switzerland"),
            ("mt", "Svizzera"),
            (
                "my",
                "ဆ\u{103d}စ\u{103a}ဇာလန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Witsierand"),
            ("nb", "Sveits"),
            ("ne", "स\u{94d}विजरल\u{94d}याण\u{94d}ड"),
            ("nl", "Zwitserland"),
            ("nn", "Sveits"),
            ("nv", "Swis Bikéyah"),
            ("oc", "Soïssa"),
            ("or", "ସ\u{b4d}ବ\u{b3f}ଟ\u{b4d}ଜରଲ\u{b4d}ଯ\u{b3e}ଣ\u{b4d}ଡ"),
            ("pa", "ਸਵਿਟਰਜ਼ਰਲ\u{a48}\u{a02}ਡ"),
            ("pi", "स\u{94d}विटजरल\u{948}\u{902}ड"),
            ("pl", "Szwajcaria"),
            ("ps", "سویس"),
            ("pt", "Suíça"),
            ("pt_BR", "Suíça"),
            ("ro", "Elveția"),
            ("ru", "Швейцария"),
            ("rw", "Ubusuwisi"),
            ("sc", "Isvìtzera"),
            ("sd", "Switzerland"),
            ("si", "ස\u{dca}ව\u{dd2}ට\u{dca}සර\u{dca}ලන\u{dca}තය"),
            ("sk", "Švajčiarsko"),
            ("sl", "Švica"),
            ("so", "Swiiserlaand"),
            ("sq", "Zvicër"),
            ("sr", "Швајцарска"),
            ("sv", "Schweiz"),
            ("sw", "Uswisi"),
            ("ta", "சுவிட\u{bcd}சர\u{bcd}ல\u{bbe}ந\u{bcd}து"),
            ("te", "స\u{c4d}వ\u{c3f}డ\u{c4d}జర\u{c4d}ల\u{c3e}ండ\u{c4d}"),
            ("tg", "Швейтсария"),
            ("th", "สว\u{e34}ตเซอร\u{e4c}แลนด\u{e4c}"),
            ("ti", "ስዊዘርላንድ"),
            ("tk", "Şweýsariýa"),
            ("tl", "Switzerland"),
            ("tr", "İsviçre"),
            ("tt", "İсвичрә"),
            ("ug", "شىۋېيىتسارىيە"),
            ("uk", "Швейцарія"),
            ("ur", "سویٹزرلینڈ"),
            ("uz", "Shveysariya"),
            ("ve", "Switzerland"),
            ("vi", "Thụy Sĩ"),
            ("wa", "Swisse"),
            ("wo", "Suwis"),
            ("xh", "Switzerland"),
            ("yo", "Swítsàlandì"),
            ("zh_CN", "瑞士"),
            ("zh_HK", "瑞士"),
            ("zh_TW", "瑞士"),
            ("zu", "I-Switzerland"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
