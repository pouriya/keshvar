// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Sweden

#[cfg(all(feature = "se", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::SE;
    pub const ALPHA3: Alpha3 = Alpha3::SWE;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 46;
    pub const CURRENCY_CODE: &str = "SEK";
    pub const GEC: Option<GEC> = Some(GEC::SW);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::SWE);
    pub const ISO_SHORT_NAME: &str = "Sweden";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Sweden";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["sv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["sv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Swedish");
    pub const NUMBER: &str = "752";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3} ?\\d{2}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "SE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Sweden",
        "Schweden",
        "Suède",
        "Suecia",
        "スウェーデン",
        "Zweden",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Sweden"),
        ("af", "Swede"),
        ("ak", "Sweden"),
        ("am", "ስፁ፥ን"),
        ("an", "Sweden"),
        ("ar", "الس\u{651}ويد"),
        ("as", "ছ\u{9c1}ইডেন"),
        ("ay", "Sweden"),
        ("az", "İsveç"),
        ("ba", "Sweden"),
        ("be", "Швецыя"),
        ("bg", "Швеция"),
        ("bi", "Sweden"),
        ("bn", "স\u{9c1}ইডেন"),
        ("bn_IN", "স\u{9c1}ইডেন"),
        ("br", "Sveden"),
        ("bs", "Švedska"),
        ("ca", "Suècia"),
        ("ce", "Швеци"),
        ("ch", "Sweden"),
        ("cs", "Švédsko"),
        ("cv", "Швеци"),
        ("cy", "Sweden"),
        ("da", "Sverige"),
        ("de", "Schweden"),
        ("dv", "ސ\u{7aa}ވ\u{7a8}ޑ\u{7a6}ނ\u{7b0}"),
        ("dz", "ས\u{f74}འ\u{f72}་ཌ\u{f7a}ན།"),
        ("ee", "Sweden"),
        ("el", "Σουηδία"),
        ("en", "Sweden"),
        ("eo", "Svedio"),
        ("es", "Suecia"),
        ("et", "Rootsi"),
        ("eu", "Suedia"),
        ("fa", "سوئد"),
        ("ff", "Suwed"),
        ("fi", "Ruotsi"),
        ("fo", "Svøríki"),
        ("fr", "Suède"),
        ("fy", "Sweden"),
        ("ga", "An tSualainn"),
        ("gl", "Suecia"),
        ("gn", "Sweden"),
        ("gu", "સ\u{acd}વિડન"),
        ("gv", "Yn Toolynn"),
        ("ha", "Sweden"),
        ("he", "שוודיה"),
        ("hi", "स\u{94d}वीडन"),
        ("hr", "Švedska"),
        ("ht", "Syèd"),
        ("hu", "Svédország"),
        ("hy", "Շվեդիա"),
        ("ia", "Svedia"),
        ("id", "Swedia"),
        ("io", "Suedia"),
        ("is", "Svíþjóð"),
        ("it", "Svezia"),
        ("iu", "ᔅᕗᕆᑭ"),
        ("ja", "スウェーデン"),
        ("ka", "შვეცია"),
        ("ki", "Sweden"),
        ("kk", "Швеция"),
        ("kl", "Sweden"),
        ("km", "ស\u{17ca}\u{17bb}យអែត"),
        ("kn", "ಸ\u{ccd}ವೀಡನ\u{ccd}"),
        ("ko", "스웨덴"),
        ("ku", "Swêd"),
        ("kv", "Швеция"),
        ("kw", "Swedherwyk"),
        ("ky", "Швеция"),
        ("lo", "ປະເທດຊ\u{eb9}ແອດ"),
        ("lt", "Švedija"),
        ("lv", "Zviedrija"),
        ("mi", "Huitene"),
        ("mk", "Шведска"),
        ("ml", "സ\u{d4d}വീഡന\u{d4d}\u{200d}"),
        ("mn", "Швед"),
        ("mr", "स\u{94d}वीडन"),
        ("ms", "Sweden"),
        ("mt", "Svezja"),
        (
            "my",
            "ဆ\u{103d}\u{102e}ဒင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Widen"),
        ("nb", "Sverige"),
        ("ne", "स\u{94d}विड\u{947}न"),
        ("nl", "Zweden"),
        ("nn", "Sverige"),
        ("nv", "Chʼah Bideeʼí Dineʼé Bikéyah"),
        ("oc", "Suècia"),
        ("or", "ସ\u{b4d}ବୀଡେନ"),
        ("pa", "ਸਵੀਡਨ"),
        ("pi", "स\u{94d}वीडन"),
        ("pl", "Szwecja"),
        ("ps", "سویډن"),
        ("pt", "Suécia"),
        ("pt_BR", "Suécia"),
        ("ro", "Suedia"),
        ("ru", "Швеция"),
        ("rw", "Suwede"),
        ("sc", "Isvètzia"),
        ("sd", "سويڊن"),
        ("si", "ස\u{dca}ව\u{dd3}ඩනය"),
        ("sk", "Švédsko"),
        ("sl", "Švedska"),
        ("so", "Iswidhan"),
        ("sq", "Suedi"),
        ("sr", "Шведска"),
        ("sv", "Sverige"),
        ("sw", "Sweden"),
        ("ta", "சுவ\u{bc0}டன\u{bcd}"),
        ("te", "స\u{c4d}వ\u{c40}డన\u{c4d}"),
        ("tg", "Шветсия"),
        ("th", "สว\u{e35}เดน"),
        ("ti", "ስወደን"),
        ("tk", "Şwesiýa"),
        ("tl", "Sweden"),
        ("tr", "İsveç"),
        ("tt", "İсвәҗ, Шведсиа"),
        ("ug", "شىۋېتسىيە"),
        ("uk", "Швеція"),
        ("ur", "سویڈن"),
        ("uz", "Shvetsiya"),
        ("ve", "Sweden"),
        ("vi", "Thuỵ Điển"),
        ("wa", "Suwede"),
        ("wo", "Suweed"),
        ("xh", "Sweden"),
        ("yo", "Swídìn"),
        ("zh_CN", "瑞典"),
        ("zh_HK", "瑞典"),
        ("zh_TW", "瑞典"),
        ("zu", "ISwidi"),
    ];
    #[cfg(all(feature = "se", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 60.12816100000001;
        pub const LONGITUDE: f64 = 18.643501;
        pub const MAX_LATITUDE: f64 = 69.0599709;
        pub const MAX_LONGITUDE: f64 = 24.1773101;
        pub const MIN_LATITUDE: f64 = 55.0059799;
        pub const MIN_LONGITUDE: f64 = 10.5798;
        pub const NORTHEAST_LATITUDE: f64 = 69.0599709;
        pub const NORTHEAST_LONGITUDE: f64 = 24.1773101;
        pub const SOUTHWEST_LATITUDE: f64 = 55.0059799;
        pub const SOUTHWEST_LONGITUDE: f64 = 10.5798;
    }
}
#[cfg(all(feature = "se", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 60.12816100000001,
            longitude: 18.643501,
            max_latitude: 69.0599709,
            max_longitude: 24.1773101,
            min_latitude: 55.0059799,
            min_longitude: 10.5798,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 69.0599709,
                    longitude: 24.1773101,
                },
                southwest: CountryGeoBound {
                    latitude: 55.0059799,
                    longitude: 10.5798,
                },
            },
        }
    }
}

#[cfg(all(feature = "se", feature = "subdivisions"))]
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
                    "AB",
                    Subdivision{
                        name: "AB",
                        country_alpha2: Alpha2::SE,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.60249579999999), longitude: Some(18.1384383), max_latitude: Some(60.2557827), min_latitude: Some(58.7356979), max_longitude: Some(19.3499043), min_longitude: Some(17.2375371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ستوكهولم"), ("be", "лен Стакгольм"), ("bg", "лен Стокхолм"), ("bn", "স\u{9cd}টকহোম ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Stockholm"), ("ca", "comtat d’Estocolm"), ("ccp", "𑄌\u{11133}𑄑\u{11127}𑄇\u{11134}𑄦\u{11127}𑄣\u{11134}𑄟\u{11134}"), ("ceb", "Stockholms län"), ("cs", "Stockholm"), ("da", "Stockholms län"), ("de", "Stockholms län"), ("el", "κομητεία της Στοκχόλμης"), ("en", "Stockholm"), ("es", "provincia de Estocolmo"), ("et", "Stockholmi lään"), ("eu", "Stockholmgo konderria"), ("fa", "استان استکهلم"), ("fi", "Tukholman lääni"), ("fr", "comté de Stockholm"), ("gl", "condado de Estocolmo"), ("gu", "સ\u{acd}ટોકહોમ કાઉન\u{acd}ટી"), ("he", "מחוז סטוקהולם"), ("hi", "स\u{94d}टॉकहोम ल\u{948}न"), ("hr", "Županija Stockholm"), ("hu", "Stockholm megye"), ("hy", "Ստոկհոլմի լեն"), ("id", "Daerah Stockholm"), ("is", "Stokkhólms lán"), ("it", "contea di Stoccolma"), ("ja", "ストックホルム県"), ("ka", "სტოკჰოლმი"), ("kn", "ಸ\u{ccd}ಟಾಕ\u{ccd}ಹೋಮ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "스톡홀름 주"), ("lt", "Stokholmo lėnas"), ("lv", "Stokholmas lēne"), ("mk", "Стокхолм"), ("mr", "स\u{94d}टॉकहोम काउ\u{902}टी"), ("ms", "Daerah Stockholm"), ("nb", "Stockholms län"), ("nl", "Stockholms län"), ("no", "Stockholms län"), ("pl", "Sztokholm"), ("pt", "condado de Estocolmo"), ("ro", "Stockholm län"), ("ru", "Стокгольм"), ("si", "ස\u{dca}ටොක\u{dca}හෝම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Стокхолм"), ("sr_Latn", "Stokholm"), ("sv", "Stockholms län"), ("sw", "Stockholms län"), ("ta", "ஸ\u{bcd}டோக\u{bcd}ஹோல\u{bcd}ம\u{bcd} கவுண\u{bcd}டி"), ("te", "స\u{c4d}ట\u{c3e}క\u{c4d}\u{200c}హ\u{c4b}మ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลสต\u{e47}อกโฮล\u{e4c}ม"), ("tr", "Stokholm ili"), ("uk", "лену Стокгольм"), ("ur", "سٹاکہوم کاؤنٹی"), ("vi", "hạt Stockholm"), ("zh", "斯德哥爾摩省")]),
                        unofficial_name_list: ["Stockholms län"].to_vec(),
                    }
                ),
                (
                    "AC",
                    Subdivision{
                        name: "AC",
                        country_alpha2: Alpha2::SE,
                        code: "AC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(65.3337311), longitude: Some(16.5161695), max_latitude: Some(66.340329), min_latitude: Some(63.4054636), max_longitude: Some(21.6169479), min_longitude: Some(14.2568099)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وستربوتن"), ("be", "лен Вестэрботэн"), ("bg", "Вестерботен"), ("bn", "ভ\u{9cd}য\u{9be}স\u{9cd}ট\u{9be}রবটেন বিভ\u{9be}গ"), ("bs", "Kotar Västerbotten"), ("ca", "comtat de Västerbotten"), ("ccp", "𑄞𑄌\u{11134}𑄑𑄢\u{11134}𑄝\u{1112e}𑄑𑄬𑄚\u{11134}"), ("ceb", "Västerbottens län"), ("cs", "Västerbotten"), ("da", "Västerbottens län"), ("de", "Västerbottens län"), ("el", "Βαστερμπότεν"), ("en", "Västerbotten"), ("es", "provincia de Västerbotten"), ("et", "Västerbotteni lään"), ("eu", "Västerbottengo konderria"), ("fa", "استان وستربوتن"), ("fi", "Västerbottenin lääni"), ("fr", "comté de Västerbotten"), ("gl", "condado de Västerbotten"), ("gu", "વાસ\u{acd}ટરબોટન કાઉન\u{acd}ટી"), ("he", "וסטרבוטן"), ("hi", "व\u{947}स\u{94d}तरबॉत\u{94d}त\u{947}न ल\u{948}न"), ("hr", "Županija Västerbotten"), ("hu", "Västerbotten megye"), ("id", "Daerah Västerbotten"), ("it", "Västerbotten"), ("ja", "ヴェステルボッテン県"), ("kn", "ವಾಸ\u{ccd}ಟ\u{cc6}ರ\u{ccd}ಬಟನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베스테르보텐 주"), ("lt", "Vesterbotenas"), ("lv", "Vesterbotenas lēne"), ("mk", "Вестерботен"), ("mr", "व\u{94d}हॉस\u{94d}टरबॉटल काउ\u{902}टी"), ("ms", "Daerah Västerbotten"), ("nb", "Västerbottens län"), ("nl", "Västerbottens län"), ("no", "Västerbottens län"), ("pl", "Västerbotten"), ("pt", "Västerbotten"), ("ro", "Västerbottens län"), ("ru", "Вестерботтен"), ("si", "ව\u{dcf}ස\u{dca}ටර\u{dca}බොටන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Вестерботен"), ("sr_Latn", "Vesterboten"), ("sv", "Västerbottens län"), ("sw", "Västerbottens län"), ("ta", "வ\u{bbe}ஸ\u{bcd}டெர\u{bcd}போட\u{bcd}டேன\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d}టర\u{c4d}బ\u{c4b}ట\u{c46}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องเวสเตอร\u{e4c}บอทเท\u{e47}น"), ("tr", "Västerbotten ili"), ("uk", "Вестерботтен"), ("ur", "وستربوتن کاؤنٹی"), ("vi", "hạt Västerbotten"), ("zh", "西博滕省")]),
                        unofficial_name_list: ["Västerbottens län"].to_vec(),
                    }
                ),
                (
                    "BD",
                    Subdivision{
                        name: "BD",
                        country_alpha2: Alpha2::SE,
                        code: "BD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(66.8309216), longitude: Some(20.3991966), max_latitude: Some(69.06307199999999), min_latitude: Some(65.06063499999999), max_longitude: Some(24.1624078), min_longitude: Some(15.3723748)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة نوربوتن"), ("be", "лен Норботэн"), ("bg", "Норботен"), ("bn", "নরব\u{9cd}রোটন ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Norrbotten"), ("ca", "comtat de Norrbotten"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄝\u{1112e}𑄑𑄬𑄚\u{11134}"), ("ceb", "Norrbottens län"), ("cs", "Norrbotten"), ("da", "Norrbottens län"), ("de", "Norrbottens län"), ("el", "Νορμπότεν"), ("en", "Norrbotten"), ("es", "provincia de Norrbotten"), ("et", "Norrbotteni lään"), ("eu", "Norrbottengo konderria"), ("fa", "استان نوربوتن"), ("fi", "Norrbottenin lääni"), ("fr", "comté de Norrbotten"), ("gl", "condado de Norrbotten"), ("gu", "નોર\u{acd}બોટન કાઉન\u{acd}ટી"), ("he", "נורבוטן"), ("hi", "नॉरबॉत\u{94d}त\u{947}न ल\u{948}न"), ("hr", "Županija Norrbotten"), ("hu", "Norrbotten megye"), ("hy", "Նորրբոտտեն"), ("id", "Daerah Norrbotten"), ("it", "Norrbotten"), ("ja", "ノールボッテン県"), ("kn", "ನಾರ\u{ccd}ಬರ\u{ccd}ಟ\u{ccc}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "노르보텐 주"), ("lt", "Norbotenas"), ("lv", "Norbotenas lēne"), ("mk", "Норботен"), ("mr", "नॉर\u{94d}बॉट\u{947}न काउ\u{902}टी"), ("ms", "Daerah Norrbotten"), ("nb", "Norrbottens län"), ("nl", "Norrbottens län"), ("no", "Norrbottens län"), ("pl", "Norrbotten"), ("pt", "Norrbotten"), ("ro", "Norrbottens län"), ("ru", "Норрботтен"), ("si", "නොර\u{dca}බෝටෙන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "Norrbottensko okrožje"), ("sr", "Нурботен"), ("sr_Latn", "Nurboten"), ("sv", "Norrbottens län"), ("sw", "Norrbottens län"), ("ta", "நோற\u{bcd}றபோட\u{bcd}டேன\u{bcd} கவுண\u{bcd}டி"), ("te", "న\u{c3e}ర\u{c4d}బ\u{c4b}ట\u{c46}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "นอร\u{e4c}บอทเทน ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Norrbotten ili"), ("uk", "Норрботтен"), ("ur", "نوربوتن کاؤنٹی"), ("vi", "hạt Norrbotten"), ("zh", "北博滕省")]),
                        unofficial_name_list: ["Norrbottens län"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::SE,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(60.00922619999999), longitude: Some(17.2714589), max_latitude: Some(60.7313874), min_latitude: Some(59.4050129), max_longitude: Some(18.7711579), min_longitude: Some(16.678336)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة اوبسالا"), ("be", "лен Упсала"), ("bg", "лен Упсала"), ("bn", "আপস\u{9be}ল\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Uppsala"), ("ca", "comtat d’Uppsala"), ("ccp", "𑄅\u{1112a}𑄛\u{11133}𑄦𑄥𑄣"), ("ceb", "Uppsala län"), ("cs", "Uppsala"), ("cy", "Sir Uppsala"), ("da", "Uppsala län"), ("de", "Uppsala län"), ("el", "Ουπσάλα"), ("en", "Uppsala"), ("es", "provincia de Upsala"), ("et", "Uppsala lään"), ("eu", "Uppsalako konderria"), ("fa", "استان اوپسالا"), ("fi", "Uppsalan lääni"), ("fr", "comté d’Uppsala"), ("gl", "condado de Uppsala"), ("gu", "ઉપ\u{acd}સલા કાઉન\u{acd}ટી"), ("he", "מחוז אופסלה"), ("hi", "उप\u{94d}साला ल\u{948}न"), ("hr", "Županija Uppsala"), ("hu", "Uppsala megye"), ("hy", "Ուփսալա"), ("id", "Daerah Uppsala"), ("is", "Uppsala län"), ("it", "contea di Uppsala"), ("ja", "ウプサラ県"), ("kn", "ಉಪ\u{ccd}ಸಾಲಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "웁살라 주"), ("lt", "Upsalos lėnas"), ("lv", "Upsālas lēne"), ("mk", "Упсала"), ("ml", "അപ\u{d4d}പ\u{d4d}സല ക\u{d57}ണ\u{d4d}ടി"), ("mr", "उप\u{94d}साला काउ\u{902}टी"), ("ms", "Daerah Uppsala"), ("nb", "Uppsala län"), ("nl", "Uppsala län"), ("no", "Uppsala län"), ("pl", "Uppsala län"), ("pt", "condado da Uppsala"), ("ro", "Uppsala län"), ("ru", "Уппсала"), ("si", "උප\u{dca}සල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Упсала"), ("sr_Latn", "Upsala"), ("sv", "Uppsala län"), ("sw", "Uppsala län"), ("ta", "உப\u{bcd}ச\u{bbe}ல\u{bbe} கவுண\u{bcd}டி"), ("te", "ఉప\u{c4d}స\u{c3e}ల\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "อ\u{e38}ปซาลา"), ("tr", "Uppsala ili"), ("uk", "лен Уппсала"), ("ur", "اوپسالا کاؤنٹی"), ("vi", "hạt Uppsala"), ("zh", "乌普萨拉省")]),
                        unofficial_name_list: ["Uppsala län"].to_vec(),
                    }
                ),
                (
                    "D",
                    Subdivision{
                        name: "D",
                        country_alpha2: Alpha2::SE,
                        code: "D",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.03363489999999), longitude: Some(16.7518899), max_latitude: Some(59.5228374), min_latitude: Some(58.56213729999999), max_longitude: Some(17.7453121), min_longitude: Some(15.5932)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سودرمانلاند"), ("be", "лен Сёдэрманланд"), ("bg", "лен Сьодерманланд"), ("bn", "সোড\u{9be}রম\u{9cd}য\u{9be}নল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Södermanland"), ("ca", "comtat de Södermanland"), ("ccp", "𑄥\u{1112e}𑄓𑄢\u{11134}𑄟𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Södermanlands län"), ("cs", "Södermanland"), ("da", "Södermanlands län"), ("de", "Södermanlands län"), ("el", "Σοντερμανλάντ"), ("en", "Södermanland"), ("es", "provincia de Södermanland"), ("et", "Södermanlandi lään"), ("eu", "Södermanlandeko konderria"), ("fa", "استان سودرمانلاند"), ("fi", "Södermanlandin lääni"), ("fr", "comté de Södermanland"), ("gl", "condado de Södermanland"), ("gu", "સોડરમ\u{ac7}નલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "מחוז סדרמנלנד"), ("hi", "सोदरमानलान\u{94d}द ल\u{948}न"), ("hr", "Županija Södermanland"), ("hu", "Södermanland megye"), ("hy", "Սյոդերմանլանդ"), ("id", "Daerah Södermanland"), ("is", "Södermanlands län"), ("it", "contea di Södermanland"), ("ja", "セーデルマンランド県"), ("kn", "ಸೊಡರ\u{ccd}ಮ\u{ccd}ಯಾನ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "쇠데르만란드 주"), ("lt", "Siodermanlandas lėnas"), ("lv", "Sēdermanlandes lēne"), ("mk", "Седерманланд"), ("mr", "सो\u{902}ड\u{947}रमनल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Södermanland"), ("nb", "Södermanlands län"), ("nl", "Södermanlands län"), ("no", "Södermanlands län"), ("pl", "Södermanlands län"), ("pt", "condado da Södermanland"), ("ro", "Södermanlands län"), ("ru", "Сёдерманланд"), ("si", "සොදර\u{dca}මන\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Седерманланд"), ("sr_Latn", "Sedermanland"), ("sv", "Södermanlands län"), ("sw", "Södermanlands län"), ("ta", "சோடெர\u{bcd}மன\u{bcd}லண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "స\u{c4b}డర\u{c4d}మన\u{c4d}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโซเดอร\u{e4c}แมนแลนด\u{e4c}"), ("tr", "Södermanland ili"), ("uk", "лен Седерманланд"), ("ur", "سودرمنلاند کاؤنٹی"), ("vi", "hạt Södermanland"), ("zh", "南曼兰省")]),
                        unofficial_name_list: ["Södermanlands län"].to_vec(),
                    }
                ),
                (
                    "E",
                    Subdivision{
                        name: "E",
                        country_alpha2: Alpha2::SE,
                        code: "E",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.3453635), longitude: Some(15.5197843), max_latitude: Some(59.01996210000001), min_latitude: Some(57.702259), max_longitude: Some(17.0853315), min_longitude: Some(14.5409236)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوسترغوتلاند"), ("be", "лен Эстэргётланд"), ("bg", "лен Йостерйотланд"), ("bn", "ও\u{981}স\u{9cd}ট\u{9be}রগটল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Östergötland"), ("ca", "comtat d’Östergötland"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑𑄢\u{11134}𑄉\u{1112e}𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Östergötlands län"), ("cs", "Östergötland"), ("da", "Östergötlands län"), ("de", "Östergötlands län"), ("el", "κομητεία της Έστεργέτλανδς"), ("en", "Östergötland"), ("es", "provincia de Östergötland"), ("et", "Östergötlandi lään"), ("eu", "Östergötlandeko konderria"), ("fa", "استان اوسترگوتلاند"), ("fi", "Itä-Götanmaan lääni"), ("fr", "comté d’Östergötland"), ("gl", "condado de Östergötland"), ("gu", "ઓસ\u{acd}ટરગોટલ\u{ac7}\u{a82}ડ કાઉન\u{acd}ટી"), ("he", "אסטרייטלנד"), ("hi", "ओस\u{94d}तरयोतलान\u{94d}द ल\u{948}न"), ("hr", "Županija Östergötland"), ("hu", "Östergötland megye"), ("hy", "Էստերգյոտլանդ"), ("id", "Daerah Östergötland"), ("is", "Östergötlands län"), ("it", "contea di Östergötland"), ("ja", "エステルイェータランド県"), ("kn", "ಓಸ\u{ccd}ಟ\u{cc6}ರ\u{ccd}ಗೋಟ\u{ccd}ಲ\u{ccd}ಯಾನ\u{ccd}ಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "외스테르예틀란드 주"), ("lt", "Esterjotlando lėnas"), ("lv", "Esterjētlandes lēne"), ("mk", "Естерјетланд"), ("mr", "ऑस\u{94d}टरगोटल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Östergötland"), ("nb", "Östergötlands län"), ("nl", "Östergötlands län"), ("no", "Östergötlands län"), ("pl", "Östergötlands län"), ("pt", "condado da Östergötland"), ("ro", "Östergötlands län"), ("ru", "Эстергётланд"), ("si", "ඕස\u{dca}ටේරෝගොට\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Естерјетланд"), ("sr_Latn", "Esterjetland"), ("sv", "Östergötlands län"), ("sw", "Östergötlands län"), ("ta", "ஆஸ\u{bcd}டெரிகோட\u{bcd}ல\u{bbe}ண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "ఆస\u{c4d}టర\u{c4d}\u{200c}గ\u{c3e}ట\u{c4d}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ออสเตอร\u{e4c}โกตแลนด\u{e4c}"), ("tr", "Östergötland ili"), ("uk", "лен Естерйотланд"), ("ur", "اوستریوتلاند کاؤنٹی"), ("vi", "hạt Östergötland"), ("zh", "东约特兰省")]),
                        unofficial_name_list: ["Östergötlands län"].to_vec(),
                    }
                ),
                (
                    "F",
                    Subdivision{
                        name: "F",
                        country_alpha2: Alpha2::SE,
                        code: "F",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.3708434), longitude: Some(14.3439173), max_latitude: Some(58.153372), min_latitude: Some(56.885171), max_longitude: Some(15.6562501), min_longitude: Some(13.0688452)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة يونشوبينغ"), ("be", "лен Ёнчэпінг"), ("bg", "лен Йоншьопинг"), ("bn", "জঙ\u{9cd}কোপিং ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Jönköping"), ("ca", "comtat de Jönköping"), ("ccp", "𑄎\u{11127}𑄚\u{11134}𑄇\u{1112e}𑄛\u{11128}\u{11101}"), ("ceb", "Jönköpings län"), ("cs", "Jönköping"), ("da", "Jönköpings län"), ("de", "Jönköpings län"), ("el", "κομητεία της Γιέντσεπινγκ"), ("en", "Jönköping"), ("es", "provincia de Jönköping"), ("et", "Jönköpingi lään"), ("eu", "Jönköpingeko konderria"), ("fa", "استان یونشوپینگ"), ("fi", "Jönköpingin lääni"), ("fr", "comté de Jönköping"), ("gl", "condado de Jönköping"), ("gu", "જોન\u{acd}કોપિ\u{a82}ગ કાઉન\u{acd}ટી"), ("he", "מחוז ינשפינג"), ("hi", "योनशोपि\u{902}ग ल\u{948}न"), ("hr", "Županija Jönköping"), ("hu", "Jönköping megye"), ("id", "Daerah Jönköping"), ("is", "Jönköpings län"), ("it", "contea di Jönköping"), ("ja", "ヨンショーピング県"), ("kn", "ಜೋನ\u{ccd}ಕೊಪ\u{cbf}ಂಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "옌셰핑 주"), ("lt", "Jonšiopingo lėnas"), ("lv", "Jenšēpingas lēne"), ("mk", "Јеншепинг"), ("mr", "जो\u{902}कोपि\u{902}ग काउ\u{902}टी"), ("ms", "Daerah Jönköping"), ("nb", "Jönköpings län"), ("nl", "Jönköpings län"), ("no", "Jönköpings län"), ("pl", "Jönköpings län"), ("pt", "condado da Jönköping"), ("ro", "Jönköpings län"), ("ru", "Йёнчёпинг"), ("si", "ජෝන\u{dca}කොප\u{dd2}න\u{dca}ග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "okrožje Jönköping"), ("sr", "Јенћепинг"), ("sr_Latn", "Jenćeping"), ("sv", "Jönköpings län"), ("sw", "Jönköpings län"), ("ta", "ஜோன\u{bcd}கோபிங\u{bcd} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}ంక\u{c4b}ప\u{c3f}ంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลเย\u{e34}นเชอพ\u{e34}งก\u{e4c}"), ("tr", "Jönköping ili"), ("uk", "лен Йончопінґ"), ("ur", "یونشوپنگ کاؤنٹی"), ("vi", "hạt Jönköping"), ("zh", "延雪平省")]),
                        unofficial_name_list: ["Jönköpings län"].to_vec(),
                    }
                ),
                (
                    "G",
                    Subdivision{
                        name: "G",
                        country_alpha2: Alpha2::SE,
                        code: "G",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.71834029999999), longitude: Some(14.4114674), max_latitude: Some(57.238187), min_latitude: Some(56.35664389999999), max_longitude: Some(15.844141), min_longitude: Some(13.2767319)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كرونوبري"), ("be", "лен Крунуберг"), ("bg", "Крунубери"), ("bn", "ক\u{9cd}য\u{9be}রিনো ক\u{9cd}রোনোব\u{9be}র\u{9cd}গ ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Kronoberg"), ("ca", "comtat de Kronoberg"), ("ccp", "𑄇\u{11133}𑄢\u{1112e}𑄚\u{1112e}𑄝𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Kronoberg"), ("cs", "Kronoberg"), ("da", "Kronobergs län"), ("de", "Kronobergs län"), ("el", "Κρονομπέργκ"), ("en", "Kronoberg"), ("es", "provincia de Kronoberg"), ("et", "Kronobergi lään"), ("eu", "Kronobergeko konderria"), ("fa", "استان کرونوبرگ"), ("fi", "Kronobergin lääni"), ("fr", "comté de Kronoberg"), ("gl", "condado de Kronoberg"), ("gu", "ક\u{acd}રોનોબર\u{acd}ગ કાઉન\u{acd}ટી"), ("hi", "क\u{94d}र\u{942}न\u{941}ब\u{948}र\u{94d}य ल\u{948}न"), ("hr", "Županija Kronoberg"), ("hu", "Kronoberg megye"), ("id", "Daerah Kronoberg"), ("it", "Contea di Kronoberg"), ("ja", "クロノベリ県"), ("kn", "ಕ\u{ccd}ರೊನೊರ\u{ccd}ಗ\u{ccd}ಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "크로노베리 주"), ("lt", "Kronobergo lėnas"), ("lv", "Krūnuberjas lēne"), ("mk", "Крунуберг"), ("ml", "ക\u{d4d}രോണോബെർഗ\u{d4d} ക\u{d57}ണ\u{d4d}ടി"), ("mr", "क\u{94d}रोनोब\u{94d}रग काउ\u{902}टी"), ("ms", "Daerah Kronoberg"), ("nb", "Kronobergs län"), ("nl", "Kronobergs län"), ("no", "Kronobergs län"), ("pl", "Kronoberg"), ("pt", "Kronoberg"), ("ro", "Kronobergs län"), ("ru", "Крунуберг"), ("si", "ක\u{dca}රෝනොබර\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sr", "Крунуберг"), ("sr_Latn", "Krunuberg"), ("sv", "Kronobergs län"), ("sw", "Kronobergs län"), ("ta", "க\u{bcd}ரோநோபேர\u{bcd}க\u{bcd} கவுண\u{bcd}டி"), ("te", "క\u{c4d}ర\u{c4b}న\u{c4b}బర\u{c4d}గ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "โครโนเบ\u{e34}ร\u{e4c}ก ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Kronoberg ili"), ("uk", "Крунуберг"), ("ur", "کرونوبری کاؤنٹی"), ("vi", "hạt Kronoberg"), ("zh", "克鲁努贝里省")]),
                        unofficial_name_list: ["Kronobergs län"].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "H",
                        country_alpha2: Alpha2::SE,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.2350156), longitude: Some(16.1849349), max_latitude: Some(58.141144), min_latitude: Some(56.1941934), max_longitude: Some(17.1506084), min_longitude: Some(15.3360812)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالمار"), ("be", "лен Кальмар"), ("bg", "лен Калмар"), ("bn", "ক\u{9be}ল\u{9cd}ম\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Kalmar"), ("ca", "comtat de Kalmar"), ("ccp", "𑄇𑄣\u{11134}𑄟𑄢\u{11134}"), ("ceb", "Kalmar län"), ("cs", "Kalmar"), ("da", "Kalmar län"), ("de", "Kalmar län"), ("el", "κομητεία της Καλμαρς"), ("en", "Kalmar"), ("es", "provincia de Kalmar"), ("et", "Kalmari lään"), ("eu", "Kalmarko konderria"), ("fa", "استان کالمار"), ("fi", "Kalmarin lääni"), ("fr", "comté de Kalmar"), ("gl", "condado de Kalmar"), ("gu", "કાલમર કાઉન\u{acd}ટી"), ("hi", "कल\u{94d}मार ल\u{948}न"), ("hr", "Županija Kalmar"), ("hu", "Kalmar megye"), ("id", "Daerah Kalmar"), ("is", "Kalmar län"), ("it", "Contea di Kalmar"), ("ja", "カルマル県"), ("kn", "ಕಲ\u{ccd}ಮರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "칼마르 주"), ("lt", "Kalmaro lėnas"), ("lv", "Kalmāras lēne"), ("mk", "Калмар"), ("mr", "कालमर काउ\u{902}टी"), ("ms", "Daerah Kalmar"), ("nb", "Kalmar län"), ("nl", "Kalmar län"), ("no", "Kalmar län"), ("pl", "Kalmar län"), ("pt", "condado da Kalmar"), ("ro", "Kalmar län"), ("ru", "Кальмар"), ("si", "කල\u{dca}ම\u{dcf}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Калмар"), ("sr_Latn", "Kalmar"), ("sv", "Kalmar län"), ("sw", "Kalmar län"), ("ta", "க\u{bbe}ல\u{bcd}மர\u{bcd} கவுண\u{bcd}டி"), ("te", "కల\u{c4d}మ\u{c3e}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "คาลมาร\u{e4c}"), ("tr", "Kalmar ili"), ("uk", "лен Кальмар"), ("ur", "کالمار کاؤنٹی"), ("vi", "hạt Kalmar"), ("zh", "卡尔马省")]),
                        unofficial_name_list: ["Calmar"].to_vec(),
                    }
                ),
                (
                    "I",
                    Subdivision{
                        name: "I",
                        country_alpha2: Alpha2::SE,
                        code: "I",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.46841209999999), longitude: Some(18.4867447), max_latitude: Some(58.3987317), min_latitude: Some(56.9048659), max_longitude: Some(19.3504137), min_longitude: Some(17.9564368)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة جوتلاندز"), ("be", "лен Готланд"), ("bg", "Готланд"), ("bn", "গটল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Gotland"), ("ca", "comtat de Gotland"), ("ccp", "𑄉\u{11127}𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Gotlands län"), ("cs", "Gotland"), ("cy", "Sir Gotland"), ("da", "Gotlands län"), ("de", "Gotlands län"), ("el", "Γκότλαντ"), ("en", "Gotland"), ("es", "provincia de Gotland"), ("et", "Ojamaa lään"), ("eu", "Gotlandeko konderria"), ("fa", "استان گوتلاند"), ("fi", "Gotlannin lääni"), ("fr", "comté de Gotland"), ("gu", "ગોટલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "מחוז גוטלנד"), ("hi", "गॉतलान\u{94d}द ल\u{948}न"), ("hr", "Županija Gotland"), ("hu", "Gotland megye"), ("id", "Daerah Gotland"), ("it", "Gotland"), ("ja", "ゴットランド県"), ("kn", "ಗಾಟ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "고틀란드 주"), ("lt", "Gotlando apygarda"), ("lv", "Gotlandes lēne"), ("mk", "Готланд"), ("mr", "गोटल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Gotland"), ("nb", "Gotlands län"), ("nl", "Gotlands län"), ("no", "Gotlands län"), ("pl", "Gotland"), ("pt", "Gotland"), ("ro", "Gotlands län"), ("ru", "Готланд"), ("si", "ගොට\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Gotland"), ("sr", "Готланд"), ("sr_Latn", "Gotland"), ("sv", "Gotlands län"), ("sw", "Gotlands län"), ("ta", "கோட\u{bcd}ல\u{bbe}ன\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c3e}ట\u{c4d}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลก\u{e4a}อตแลนด\u{e4c}"), ("tr", "Gotland ili"), ("uk", "Ґотланд"), ("ur", "گوتلاند کاؤنٹی"), ("vi", "hạt Gotland"), ("zh", "哥得兰省")]),
                        unofficial_name_list: ["Gotlands län"].to_vec(),
                    }
                ),
                (
                    "K",
                    Subdivision{
                        name: "K",
                        country_alpha2: Alpha2::SE,
                        code: "K",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.2783837), longitude: Some(15.0180058), max_latitude: Some(56.5022141), min_latitude: Some(55.9901428), max_longitude: Some(16.0679243), min_longitude: Some(14.3528851)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بليكينج"), ("be", "лен Блекінгэ"), ("bg", "община Блекинге"), ("bn", "ব\u{9cd}লেকিঞ\u{9cd}জে ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Blekinge"), ("ca", "comtat de Blekinge"), ("ccp", "𑄝\u{11133}𑄣𑄬𑄇\u{11128}𑄚\u{11134}𑄌\u{11134}"), ("ceb", "Blekinge län"), ("da", "Blekinge län"), ("de", "Blekinge län"), ("el", "κομητεία της Μπλεκιγγες"), ("en", "Blekinge"), ("es", "provincia de Blekinge"), ("et", "Blekinge lään"), ("eu", "Blekingeko konderria"), ("fa", "استان بلکینگه"), ("fi", "Blekingen lääni"), ("fr", "comté de Blekinge"), ("gl", "condado de Blekinge"), ("gu", "બ\u{acd}લ\u{ac7}કિ\u{a82}ગ કાઉન\u{acd}ટી"), ("hi", "ब\u{94d}लकि\u{902}ग\u{947} ल\u{948}न"), ("hr", "Županija Blekinge"), ("hu", "Blekinge megye"), ("hy", "Բլեքինգե"), ("id", "Daerah Blekinge"), ("is", "Blekinge län"), ("it", "contea di Blekinge"), ("ja", "ブレーキンゲ県"), ("kn", "ಬ\u{ccd}ಲೇಕ\u{cbf}ಂಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "블레킹에 주"), ("lt", "Blekingės lėnas"), ("lv", "Blēkinges lēne"), ("mk", "Блекинге"), ("mr", "ब\u{94d}ल\u{947}की\u{902}ग काउ\u{902}टी"), ("ms", "Daerah Blekinge"), ("nb", "Blekinge län"), ("nl", "Blekinge län"), ("no", "Blekinge län"), ("pl", "Blekinge län"), ("pt", "Blekinge"), ("ro", "Blekinge län"), ("ru", "Блекинге"), ("si", "බ\u{dca}ලෙක\u{dd2}න\u{dca}ජේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Blekinge"), ("sr", "Блећинге"), ("sr_Latn", "Blećinge"), ("sv", "Blekinge län"), ("sw", "Blekinge län"), ("ta", "ப\u{bcd}ளேக\u{bcd}கிங\u{bcd}க கவுண\u{bcd}டி"), ("te", "బ\u{c4d}ల\u{c46}క\u{c3f}ంగ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "บเลก\u{e34}งเง"), ("tr", "Blekinge ili"), ("uk", "лен Блекінге"), ("ur", "بلیکینے کاؤنٹی"), ("vi", "hạt Blekinge"), ("zh", "布萊金厄省")]),
                        unofficial_name_list: ["Blekinge län"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::SE,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.99025719999999), longitude: Some(13.5957692), max_latitude: Some(56.54260590000001), min_latitude: Some(55.33668249999999), max_longitude: Some(14.5844781), min_longitude: Some(12.4417552)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة اسكونه"), ("az", "Skane"), ("be", "лен Сконэ"), ("bg", "община Сконе"), ("bn", "স\u{9cd}ক\u{9be}ন ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Skåne"), ("ca", "comtat d’Escània"), ("ccp", "𑄌\u{11133}𑄇𑄚\u{11134}"), ("ceb", "Skåne län"), ("cs", "Skåne"), ("da", "Skåne län"), ("de", "Skåne län"), ("el", "κομητεία της Σκωνες"), ("en", "Skåne"), ("es", "provincia de Escania"), ("et", "Skåne lään"), ("eu", "Eskaniako konderria"), ("fa", "استان اسکونه"), ("fi", "Skånen lääni"), ("fr", "comté de Scanie"), ("gl", "condado de Escania"), ("gu", "સ\u{acd}ક\u{ac7}ન કાઉન\u{acd}ટી"), ("he", "מחוז סקונה"), ("hi", "स\u{94d}कॉन\u{947} ल\u{948}न"), ("hr", "Županija Skåne"), ("hu", "Skåne megye"), ("id", "Daerah Skåne"), ("it", "contea della Scania"), ("ja", "スコーネ県"), ("ka", "სკონე"), ("kn", "ಸ\u{ccd}ಕೇನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "스코네 주"), ("lt", "Skonės lėnas"), ("lv", "Skones lēne"), ("mk", "Сконе"), ("mr", "स\u{94d}क\u{947}न काउ\u{902}टी"), ("ms", "Daerah Skåne"), ("nb", "Skåne län"), ("nl", "Skåne län"), ("no", "Skåne län"), ("pl", "Skania"), ("pt", "Escânia"), ("ro", "Skåne län"), ("ru", "лен Сконе"), ("si", "ස\u{dca}ක\u{dcf}නේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sq", "Qarku Skåne"), ("sr", "Сконе"), ("sr_Latn", "Skone"), ("sv", "Skåne län"), ("sw", "Skåne län"), ("ta", "ஸ\u{bcd}கேன\u{bcd} கவுண\u{bcd}டி"), ("te", "స\u{c4d}క\u{c47}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลสโกเนอ"), ("tr", "Skåne ili"), ("uk", "лен Сконе"), ("ur", "سکونہ کاؤنٹی"), ("vi", "hạt Skåne"), ("zh", "斯科讷省")]),
                        unofficial_name_list: ["Scania"].to_vec(),
                    }
                ),
                (
                    "N",
                    Subdivision{
                        name: "N",
                        country_alpha2: Alpha2::SE,
                        code: "N",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.8966805), longitude: Some(12.8033993), max_latitude: Some(57.5731741), min_latitude: Some(56.324419), max_longitude: Some(13.7176809), min_longitude: Some(11.821994)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هاللاند"), ("be", "лен Халанд"), ("bg", "Халанд"), ("bn", "হল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Halland"), ("ca", "comtat de Halland"), ("ccp", "𑄦\u{11127}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Hallands län"), ("cs", "Halland"), ("da", "Hallands län"), ("de", "Hallands län"), ("el", "κομητεία της Ηαλαντς"), ("en", "Halland"), ("es", "provincia de Halland"), ("et", "Hallandi lään"), ("eu", "Hallandeko konderria"), ("fa", "استان هاللاند"), ("fi", "Hallandin lääni"), ("fr", "comté de Halland"), ("gl", "condado de Halland"), ("gu", "હોલ\u{ac7}ન\u{acd}ડ"), ("he", "הלנד"), ("hi", "हल\u{94d}लान\u{94d}द ल\u{948}न"), ("hr", "Županija Halland"), ("hu", "Halland megye"), ("id", "Daerah Halland"), ("is", "Hallands län"), ("it", "contea di Halland"), ("ja", "ハッランド県"), ("kn", "ಹಾಲಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "할란드 주"), ("lt", "Halando lėnas"), ("lv", "Hallandes lēne"), ("mk", "Халанд"), ("mr", "हॉल\u{902}ड काउ\u{902}टी"), ("ms", "Daerah Halland"), ("nb", "Hallands län"), ("nl", "Hallands län"), ("no", "Hallands län"), ("pl", "Hallands län"), ("pt", "condado da Halland"), ("ro", "Hallands län"), ("ru", "лен Халланд"), ("si", "හැලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Халанд"), ("sr_Latn", "Haland"), ("sv", "Hallands län"), ("sw", "Hallands län"), ("ta", "ஹ\u{bbe}லந\u{bcd}து கவுண\u{bcd}டி"), ("te", "హ\u{c3e}లండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ฮอลแลนด\u{e4c}"), ("tr", "Halland ili"), ("uk", "лен Галланд"), ("ur", "ہالاند کاؤنٹی"), ("vi", "hạt Halland"), ("zh", "哈兰省")]),
                        unofficial_name_list: ["Hallands län"].to_vec(),
                    }
                ),
                (
                    "O",
                    Subdivision{
                        name: "O",
                        country_alpha2: Alpha2::SE,
                        code: "O",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(58.2527926), longitude: Some(13.0596425), max_latitude: Some(59.26203409999999), min_latitude: Some(57.14008), max_longitude: Some(14.7148173), min_longitude: Some(10.9631866)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وسترا يوتالاند"), ("be", "лен Вестра-Гёталанд"), ("bg", "лен Вестра Йоталанд"), ("bn", "ভ\u{981}সট\u{9cd}র\u{9be} গো\u{981}ট\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Västra Götaland"), ("ca", "comtat de Västra Götaland"), ("ccp", "𑄞𑄌\u{11134}𑄑\u{11133}𑄢 𑄉\u{11127}𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Västra Götalands län"), ("cs", "Västra Götaland"), ("cy", "Sir Västra Götaland"), ("da", "Västra Götalands län"), ("de", "Västra Götalands län"), ("el", "κομητεία της Βέστρα Γκέταλαντς"), ("en", "Västra Götaland"), ("es", "provincia de Västra Götaland"), ("et", "Västra Götalandi lään"), ("eu", "Västra Götalandeko konderria"), ("fa", "استان وسترا گوتلاند"), ("fi", "Länsi-Götanmaan lääni"), ("fr", "comté de Västra Götaland"), ("gl", "condado de Västra Götaland"), ("gu", "વાસ\u{acd}ટ\u{acd}રા ગોટલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "וסטרה ייטלנד"), ("hi", "व\u{947}स\u{94d}त\u{94d}रा योतालान\u{94d}द ल\u{948}न"), ("hr", "Županija Västra Götaland"), ("hu", "Västra Götaland megye"), ("id", "Daerah Västra Götaland"), ("is", "Västra Götalands län"), ("it", "contea di Västra Götaland"), ("ja", "ヴェストラ・イェータランド県"), ("kn", "ವಾಸ\u{ccd}ಟ\u{ccd}ರಾ ಗೋಲ\u{ccd}ಟಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "베스트라예탈란드 주"), ("lt", "Vestra Jotalando lėnas"), ("lv", "Vesterjētlandes lēne"), ("mk", "Вестра Јеталанд"), ("mr", "व\u{94d}ह\u{945}स\u{94d}टर गोटल\u{902}ड काउ\u{902}टी"), ("ms", "Daerah Västra Götaland"), ("nb", "Västra Götalands län"), ("nl", "Västra Götalands län"), ("no", "Västra Götalands län"), ("pl", "Västra Götalands län"), ("pt", "condado da Västra Götaland"), ("ro", "Västra Götalands län"), ("ru", "Вестра-Гёталанд"), ("si", "වස\u{dca}ට\u{dca}\u{200d}ර\u{dcf} ගෝටලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Västra Götaland"), ("sr", "Вестра Јеталанд"), ("sr_Latn", "Vestra Jetaland"), ("sv", "Västra Götalands län"), ("sw", "Västra Götalands län"), ("ta", "வஸ\u{bcd}ட\u{bcd}ர\u{bbe}ங\u{bcd} கோட\u{bcd}டல\u{bbe}ன\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d}ట\u{c4d}ర\u{c3e} గ\u{c4b}ట\u{c3e}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "าสตรา โกทาแลนด\u{e4c}"), ("tr", "Västra Götaland ili"), ("uk", "лен Вестра Йоталанд"), ("ur", "واسترا یوتالاند کاؤنٹی"), ("vi", "hạt Västra Götaland"), ("zh", "西约塔兰省")]),
                        unofficial_name_list: ["Västra Götalands län"].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "S",
                        country_alpha2: Alpha2::SE,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.7294065), longitude: Some(13.2354024), max_latitude: Some(61.06945409999999), min_latitude: Some(58.83082580000001), max_longitude: Some(14.48975), min_longitude: Some(11.681877)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ورملاند"), ("be", "лен Вермланд"), ("bg", "лен Вермланд"), ("bn", "ভ\u{9be}র\u{9cd}ম\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Värmland"), ("ca", "comtat de Värmland"), ("ccp", "𑄞𑄢\u{11134}𑄟\u{11127}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Värmlands län"), ("da", "Värmlands län"), ("de", "Värmlands län"), ("el", "κομητεία της βερμλανδς"), ("en", "Värmland"), ("es", "provincia de Värmland"), ("et", "Värmlandi lään"), ("eu", "Värmlandeko konderria"), ("fa", "استان ورملاند"), ("fi", "Värmlannin lääni"), ("fr", "comté de Värmland"), ("gl", "condado de Värmland"), ("gu", "વર\u{acd}મલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "ורמלנד"), ("hi", "व\u{948}र\u{94d}मलान\u{94d}द ल\u{948}न"), ("hr", "Županija Värmland"), ("hu", "Värmland megye"), ("id", "Daerah Värmland"), ("is", "Värmlands län"), ("it", "contea di Värmland"), ("ja", "ヴェルムランド県"), ("kn", "ವಾರ\u{ccd}ಮಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베름란드 주"), ("lt", "Vermlando lėnas"), ("lv", "Vermlandes lēne"), ("mk", "Вермланд"), ("mr", "व\u{94d}हरम\u{945}\u{902}ड काउ\u{902}टी"), ("ms", "Daerah Värmland"), ("nb", "Värmlands län"), ("nl", "Värmlands län"), ("no", "Värmlands län"), ("pl", "Värmlands län"), ("pt", "condado da Värmland"), ("ro", "Värmlands län"), ("ru", "Вермланд"), ("si", "ව\u{dcf}ර\u{dca}ම\u{dca}ලන\u{dca}ත ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Värmland"), ("sr", "Вермланд"), ("sr_Latn", "Vermland"), ("sv", "Värmlands län"), ("sw", "Värmlands län"), ("ta", "வர\u{bcd}மலன\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}ర\u{c4d}మ\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลแวร\u{e4c}มล\u{e31}นด\u{e4c}"), ("tr", "Värmland ili"), ("uk", "лен Вермланд"), ("ur", "وارملاند کاؤنٹی"), ("vi", "hạt Värmland"), ("zh", "韦姆兰省")]),
                        unofficial_name_list: ["Värmlands län"].to_vec(),
                    }
                ),
                (
                    "T",
                    Subdivision{
                        name: "T",
                        country_alpha2: Alpha2::SE,
                        code: "T",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.535036), longitude: Some(15.0065731), max_latitude: Some(60.1056601), min_latitude: Some(58.656611), max_longitude: Some(15.7880099), min_longitude: Some(14.243871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة اوربرو"), ("be", "лен Эрэбру"), ("bg", "Йоребру"), ("bn", "ও\u{981}রেব\u{9cd}রো ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Örebro"), ("ca", "comtat d’Örebro"), ("ccp", "𑄃\u{11127}𑄢𑄬𑄝\u{11133}𑄢\u{1112e}"), ("ceb", "Örebro län"), ("cs", "Örebro"), ("cy", "Sir Örebro"), ("da", "Örebro län"), ("de", "Örebro län"), ("el", "Όρεμπρο"), ("en", "Örebro"), ("es", "provincia de Örebro"), ("et", "Örebro lään"), ("eu", "Örebroko konderria"), ("fa", "استان اوربرو"), ("fi", "Örebron lääni"), ("fr", "comté d’Örebro"), ("gl", "condado de Örebro"), ("gu", "ઓર\u{ac7}બ\u{acd}રો કાઉન\u{acd}ટી"), ("he", "מחוז ארברו"), ("hi", "ओर\u{947}ब\u{94d}रो ल\u{948}न"), ("hr", "Županija Örebro"), ("hu", "Örebro megye"), ("hy", "Էրեբրու"), ("id", "Daerah Örebro"), ("it", "Örebro"), ("ja", "エレブルー県"), ("kn", "ಒರ\u{cc6}ಬ\u{ccd}ರೊ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "외레브로 주"), ("lt", "Erebru lėnas"), ("lv", "Erebrū lēne"), ("mk", "Еребру"), ("mr", "ऑर\u{947}ब\u{94d}रो काउ\u{902}टी"), ("ms", "Daerah Örebro"), ("nb", "Örebro län"), ("nl", "Örebro län"), ("no", "Örebro län"), ("pl", "Örebro"), ("pt", "Örebro"), ("ro", "Örebro län"), ("ru", "Эребру"), ("si", "ඔරෙබ\u{dca}රෝ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Örebro"), ("sr", "Еребро"), ("sr_Latn", "Erebro"), ("sv", "Örebro län"), ("sw", "Örebro län"), ("ta", "ஓரெப\u{bcd}ரோ கவுண\u{bcd}டி"), ("te", "ఒర\u{c46}బ\u{c4b} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องโอเรโบร"), ("tr", "Örebro ili"), ("uk", "Еребру"), ("ur", "اوریبرو کاؤنٹی"), ("vi", "hạt Örebro"), ("zh", "厄勒布鲁省")]),
                        unofficial_name_list: ["Örebro län"].to_vec(),
                    }
                ),
                (
                    "U",
                    Subdivision{
                        name: "U",
                        country_alpha2: Alpha2::SE,
                        code: "U",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.6713879), longitude: Some(16.2158954), max_latitude: Some(60.1906571), min_latitude: Some(59.23275400000001), max_longitude: Some(16.9458588), min_longitude: Some(15.41747)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وستمانلاند"), ("be", "лен Вестманланд"), ("bg", "Вестманланд"), ("bn", "ভ\u{9cd}য\u{9be}স\u{9cd}টম\u{9cd}য\u{9be}নল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Västmanland"), ("ca", "comtat de Västmanland"), ("ccp", "𑄞𑄌\u{11134}𑄑\u{11134}𑄟𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Västmanlands län"), ("cs", "Västmanlan"), ("da", "Västmanlands län"), ("de", "Västmanlands län"), ("el", "Βαστμανλάντ"), ("en", "Västmanland"), ("es", "provincia de Västmanland"), ("et", "Västmanlandi lään"), ("eu", "Västmanlandeko konderria"), ("fa", "استان وستمانلاند"), ("fi", "Västmanlandin lääni"), ("fr", "comté de Västmanland"), ("gl", "condado de Västmanland"), ("gu", "વસ\u{acd}ટમ\u{ac7}નલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "מחוז וסטמנלנד"), ("hi", "व\u{947}स\u{94d}तमानलान\u{94d}द ल\u{948}न"), ("hr", "Županija Västmanland"), ("hu", "Västmanland megye"), ("id", "Daerah Västmanland"), ("it", "Västmanland"), ("ja", "ヴェストマンランド県"), ("kn", "ವಾಸ\u{ccd}ಮ\u{ccd}ಯಾನ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베스트만란드 주"), ("lt", "Vestmanlandas"), ("lv", "Vestmanlandes lēne"), ("mk", "Вестманланд"), ("mr", "वस\u{94d}तमानल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Västmanland"), ("nb", "Västmanlands län"), ("nl", "Västmanlands län"), ("no", "Västmanlands län"), ("pl", "Västmanland"), ("pt", "Västmanland"), ("ro", "Västmanlands län"), ("ru", "Вестманланд"), ("si", "ව\u{dcf}ස\u{dca}ට\u{dca}මන\u{dca}ලන\u{dca}ත ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Вестманланд"), ("sr_Latn", "Vestmanland"), ("sv", "Västmanlands län"), ("sw", "Västmanlands län"), ("ta", "வ\u{bbe}ஸ\u{bcd}துமன\u{bcd}லேண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d}ట\u{c4d}మ\u{c3e}న\u{c4d}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลเวสมานล\u{e31}นด\u{e4c}"), ("tr", "Västmanland ili"), ("uk", "Вестманланд"), ("ur", "ویستمانلاند کاؤنٹی"), ("vi", "hạt Västmanland"), ("zh", "西曼兰省")]),
                        unofficial_name_list: ["Västmanlands län"].to_vec(),
                    }
                ),
                (
                    "W",
                    Subdivision{
                        name: "W",
                        country_alpha2: Alpha2::SE,
                        code: "W",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(61.0917012), longitude: Some(14.6663653), max_latitude: Some(62.28024099999999), min_latitude: Some(59.852169), max_longitude: Some(16.739265), min_longitude: Some(12.1331131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دالرناس"), ("be", "лен Даларна"), ("bg", "Даларна"), ("bn", "দ\u{9be}ল\u{9be}র\u{9cd}ন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Dalarna"), ("ca", "comtat de Dalarna"), ("ccp", "𑄓𑄣𑄢\u{11134}𑄚"), ("ceb", "Dalarnas län"), ("cs", "Dalarna"), ("da", "Dalarnas län"), ("de", "Dalarnas län"), ("el", "Νταλάρνα"), ("en", "Dalarna"), ("es", "provincia de Dalarna"), ("et", "Dalarna lään"), ("eu", "Dalarnako konderria"), ("fa", "استان دالارنا"), ("fi", "Taalainmaan lääni"), ("fr", "comté de Dalécarlie"), ("gl", "condado de Dalarna"), ("gu", "દાલર\u{acd}ના કાઉન\u{acd}ટી"), ("he", "מחוז דלרנה"), ("hi", "दालारना ल\u{948}न"), ("hr", "Županija Dalarna"), ("hu", "Dalarna megye"), ("id", "Daerah Dalarna"), ("it", "Dalarna"), ("ja", "ダーラナ県"), ("kn", "ದಲರ\u{ccd}ನಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "달라르나 주"), ("lt", "Dalarna"), ("lv", "Dalarnas lēne"), ("mk", "Даларна"), ("mr", "डालर\u{94d}न काउ\u{902}टी"), ("ms", "Daerah Dalarna"), ("nb", "Dalarnas län"), ("nl", "Dalarnas län"), ("no", "Dalarnas län"), ("pl", "Dalarna"), ("pt", "Dalarna"), ("ro", "Dalarnas län"), ("ru", "Даларна"), ("si", "ඩලම\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Dalarna"), ("sr", "Даларна"), ("sr_Latn", "Dalarna"), ("sv", "Dalarnas län"), ("sw", "Dalarnas län"), ("ta", "ட\u{bbe}லர\u{bcd}ன\u{bbe} கவுண\u{bcd}டி"), ("te", "డ\u{c3e}ల\u{c3e}మ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ดาลาร\u{e4c}นา"), ("tr", "Dalarna ili"), ("uk", "Даларна"), ("ur", "دالارنا کاؤنٹی"), ("vi", "hạt Dalarna"), ("zh", "达拉纳省")]),
                        unofficial_name_list: ["Dalarnas", "Dalecarlia", "Kopparberg"].to_vec(),
                    }
                ),
                (
                    "X",
                    Subdivision{
                        name: "X",
                        country_alpha2: Alpha2::SE,
                        code: "X",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(61.30119930000001), longitude: Some(16.1534215), max_latitude: Some(62.37083), min_latitude: Some(60.201385), max_longitude: Some(17.6414757), min_longitude: Some(14.459403)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة يافلبورغ"), ("be", "лен Еўлебарг"), ("bg", "Йевлебори"), ("bn", "গ\u{9cd}য\u{9be}ভ\u{9cd}লেব\u{9be}র\u{9cd}গ ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Gävleborg"), ("ca", "comtat de Gävleborg"), ("ccp", "𑄉\u{11133}𑄠𑄛\u{11134}𑄣𑄬𑄝\u{1112e}𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Gävleborgs län"), ("cs", "Gävleborg"), ("da", "Gävleborgs län"), ("de", "Gävleborgs län"), ("el", "Γκάβλεμποργκ"), ("en", "Gävleborg"), ("es", "provincia de Gävleborg"), ("et", "Gävleborgi lään"), ("eu", "Gävleborgeko konderria"), ("fa", "استان گولبوری"), ("fi", "Gävleborgin lääni"), ("fr", "comté de Gävleborg"), ("gl", "condado de Gävleborg"), ("gu", "ગ\u{ac7}વ\u{acd}લોબોર\u{acd}ગ કાઉન\u{acd}ટી"), ("he", "יבלבורג"), ("hi", "य\u{948}व\u{94d}ल\u{947}बॉर\u{94d}य ल\u{948}न"), ("hr", "Županija Gävleborg"), ("hu", "Gävleborg megye"), ("id", "Daerah Gävleborg"), ("it", "Gävleborg"), ("ja", "イェヴレボリ県"), ("kn", "ಗವ\u{cc6}ಲ\u{ccd}ಬೋರ\u{ccd}ಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}ಯವರು"), ("ko", "예블레보리 주"), ("lt", "Jevleborgas"), ("lv", "Jēvleborjas lēne"), ("mk", "Јевлеборг"), ("mr", "गॉल\u{94d}ल\u{947}बॉर\u{94d}ग काउ\u{902}टी"), ("ms", "Daerah Gävleborg"), ("nb", "Gävleborgs län"), ("nl", "Gävleborgs län"), ("no", "Gävleborgs län"), ("pl", "Gävleborg"), ("pt", "Gävleborg"), ("ro", "Regiunea Gävleborg"), ("ru", "Евлеборг"), ("si", "ග\u{dcf}ව\u{dca}ලෙබෝර\u{dca}ග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Јевлеборг"), ("sr_Latn", "Jevleborg"), ("sv", "Gävleborgs län"), ("sw", "Gävleborgs län"), ("ta", "க\u{bbe}வ\u{bcd}லேபோர\u{bcd}க\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c3e}వ\u{c4d}ల\u{c47}బ\u{c4b}ర\u{c4d}గ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "แยฟเลบอร\u{e4c}ย"), ("tr", "Gävleborg ili"), ("uk", "Євлеборґ"), ("ur", "یاولیبوری کاؤنٹی"), ("vi", "hạt Gävleborg"), ("zh", "耶夫勒堡省")]),
                        unofficial_name_list: ["Gävleborgs län"].to_vec(),
                    }
                ),
                (
                    "Y",
                    Subdivision{
                        name: "Y",
                        country_alpha2: Alpha2::SE,
                        code: "Y",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(63.4276473), longitude: Some(17.7292444), max_latitude: Some(64.00223489999999), min_latitude: Some(62.13979099999999), max_longitude: Some(19.2867079), min_longitude: Some(14.776302)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وسترنورلاند"), ("be", "лен Вестэрнорланд"), ("bg", "Вестернорланд"), ("bn", "ভ\u{9cd}য\u{9be}স\u{9cd}ট\u{9be}রনরল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Västernorrland"), ("ca", "comtat de Västernorrland"), ("ccp", "𑄞𑄌\u{11134}𑄑𑄢\u{11134}𑄚\u{11127}𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Västernorrlands län"), ("cs", "Västernorrland County"), ("da", "Västernorrlands län"), ("de", "Västernorrlands län"), ("el", "Βαστερνορλάντ"), ("en", "Västernorrland"), ("es", "provincia de Västernorrland"), ("et", "Västernorrlandi lään"), ("eu", "Västernorrlandeko konderria"), ("fa", "استان وسترنورلاند"), ("fi", "Västernorrlandin lääni"), ("fr", "comté de Västernorrland"), ("gl", "condado de Västernorrland"), ("gu", "વ\u{ac7}સ\u{acd}ટર\u{acd}નઓરલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("hi", "व\u{947}स\u{94d}तरनॉरलान\u{94d}द ल\u{948}न"), ("hr", "Županija Västernorrland"), ("hu", "Västernorrland megye"), ("id", "Daerah Västernorrland"), ("it", "Västernorrland"), ("ja", "ヴェステルノールランド県"), ("kn", "ವಾಸ\u{ccd}ಟರ\u{ccd}ನೋರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베스테르노를란드 주"), ("lt", "Vesternorlandas"), ("lv", "Vesternorlandes lēne"), ("mk", "Вестернорланд"), ("mr", "व\u{947}स\u{94d}टरनॉरल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Västernorrland"), ("nb", "Västernorrlands län"), ("nl", "Västernorrlands län"), ("no", "Västernorrlands län"), ("pl", "Västernorrland"), ("pt", "Västernorrland"), ("ro", "Västernorrlands län"), ("ru", "Вестерноррланд"), ("si", "වස\u{dca}ටර\u{dca}නොර\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sr", "Вестернурланд"), ("sr_Latn", "Vesternurland"), ("sv", "Västernorrlands län"), ("sw", "Västernorrlands län"), ("ta", "வ\u{bbe}ஸ\u{bcd}டெர\u{bcd}னோர\u{bcd}ர\u{bcd}லண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d}ట\u{c46}ర\u{c4d}న\u{c4b}ర\u{c4d}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลเวสเตอร\u{e4c}นอร\u{e4c}ล\u{e31}นด\u{e4c}"), ("tr", "Västernorrland ili"), ("uk", "Вестерноррланд"), ("ur", "ویسترنورلاند کاؤنٹی"), ("vi", "hạt Västernorrland"), ("zh", "西诺尔兰省")]),
                        unofficial_name_list: ["Västernorrlands län"].to_vec(),
                    }
                ),
                (
                    "Z",
                    Subdivision{
                        name: "Z",
                        country_alpha2: Alpha2::SE,
                        code: "Z",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(63.1711922), longitude: Some(14.95918), max_latitude: Some(65.073133), min_latitude: Some(61.569695), max_longitude: Some(17.0648909), min_longitude: Some(11.9688662)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة يمتلاند"), ("be", "лен Емтланд"), ("bg", "Лен Йемтланд"), ("bn", "জ\u{9be}মটল\u{9cd}য\u{9be}ন\u{9cd}ড ক\u{9be}উন\u{9cd}টি"), ("bs", "Kotar Jämtland"), ("ca", "comtat de Jämtland"), ("ccp", "𑄎𑄟\u{11134}𑄑\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Jämtlands län"), ("da", "Jämtlands län"), ("de", "Jämtlands län"), ("el", "κομητεία της Ιεμτλανδς"), ("en", "Jämtland"), ("es", "provincia de Jämtland"), ("et", "Jämtlandi lään"), ("eu", "Jämtlandeko konderria"), ("fa", "استان یمتلاند"), ("fi", "Jämtlandin lääni"), ("fr", "comté de Jämtland"), ("gl", "condado de Jämtland"), ("gu", "જ\u{ac7}મ\u{acd}ટલ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી"), ("he", "ימטלנד"), ("hi", "य\u{947}म\u{94d}तलान\u{94d}द ल\u{948}न"), ("hr", "Županija Jämtland"), ("hu", "Jämtland megye"), ("id", "Daerah Jämtland"), ("is", "Jämtlands län"), ("it", "contea di Jämtland"), ("ja", "イェムトランド県"), ("kn", "ಜಾಮ\u{ccd}ಟ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "옘틀란드 주"), ("lt", "Jemtlando lėnas"), ("lv", "Jemtlandes lēne"), ("mk", "Јемтланд"), ("mr", "ज\u{945}टल\u{901}ड काउ\u{902}टी"), ("ms", "Daerah Jämtland"), ("nb", "Jämtlands län"), ("nl", "Jämtlands län"), ("no", "Jämtlands län"), ("pl", "Jämtlands län"), ("pt", "condado da Jämtland"), ("ro", "Jämtlands län"), ("ru", "Емтланд"), ("si", "ජ\u{dcf}ම\u{dca}ට\u{dca}ලන\u{dca}ත පළ\u{dcf}ත"), ("sr", "Јемтланд"), ("sr_Latn", "Jemtland"), ("sv", "Jämtlands län"), ("sw", "Jämtlands län"), ("ta", "ஜமிட\u{bcd}லண\u{bcd}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}మ\u{c4d}ట\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลแยมต\u{e4c}ล\u{e31}นด\u{e4c}"), ("tr", "Jämtland ili"), ("uk", "лен Ємтланд"), ("ur", "جامتلنڈ کاؤنٹی"), ("vi", "hạt Jämtland"), ("zh", "耶姆特兰省")]),
                        unofficial_name_list: ["Jämtlands län"].to_vec(),
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
#[cfg(feature = "se")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SE,
        alpha3: Alpha3::SWE,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 46,
        currency_code: "SEK",
        gec: Some(GEC::SW),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::SWE),
        iso_long_name: "The Kingdom of Sweden",
        iso_short_name: "Sweden",
        official_language_list: ["sv"].to_vec(),
        spoken_language_list: ["sv"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Swedish"),
        number: "752",
        postal_code: true,
        postal_code_format: Some("\\d{3} ?\\d{2}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "SE",
        unofficial_name_list: [
            "Sweden",
            "Schweden",
            "Suède",
            "Suecia",
            "スウェーデン",
            "Zweden",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Sweden"),
            ("af", "Swede"),
            ("ak", "Sweden"),
            ("am", "ስፁ፥ን"),
            ("an", "Sweden"),
            ("ar", "الس\u{651}ويد"),
            ("as", "ছ\u{9c1}ইডেন"),
            ("ay", "Sweden"),
            ("az", "İsveç"),
            ("ba", "Sweden"),
            ("be", "Швецыя"),
            ("bg", "Швеция"),
            ("bi", "Sweden"),
            ("bn", "স\u{9c1}ইডেন"),
            ("bn_IN", "স\u{9c1}ইডেন"),
            ("br", "Sveden"),
            ("bs", "Švedska"),
            ("ca", "Suècia"),
            ("ce", "Швеци"),
            ("ch", "Sweden"),
            ("cs", "Švédsko"),
            ("cv", "Швеци"),
            ("cy", "Sweden"),
            ("da", "Sverige"),
            ("de", "Schweden"),
            ("dv", "ސ\u{7aa}ވ\u{7a8}ޑ\u{7a6}ނ\u{7b0}"),
            ("dz", "ས\u{f74}འ\u{f72}་ཌ\u{f7a}ན།"),
            ("ee", "Sweden"),
            ("el", "Σουηδία"),
            ("en", "Sweden"),
            ("eo", "Svedio"),
            ("es", "Suecia"),
            ("et", "Rootsi"),
            ("eu", "Suedia"),
            ("fa", "سوئد"),
            ("ff", "Suwed"),
            ("fi", "Ruotsi"),
            ("fo", "Svøríki"),
            ("fr", "Suède"),
            ("fy", "Sweden"),
            ("ga", "An tSualainn"),
            ("gl", "Suecia"),
            ("gn", "Sweden"),
            ("gu", "સ\u{acd}વિડન"),
            ("gv", "Yn Toolynn"),
            ("ha", "Sweden"),
            ("he", "שוודיה"),
            ("hi", "स\u{94d}वीडन"),
            ("hr", "Švedska"),
            ("ht", "Syèd"),
            ("hu", "Svédország"),
            ("hy", "Շվեդիա"),
            ("ia", "Svedia"),
            ("id", "Swedia"),
            ("io", "Suedia"),
            ("is", "Svíþjóð"),
            ("it", "Svezia"),
            ("iu", "ᔅᕗᕆᑭ"),
            ("ja", "スウェーデン"),
            ("ka", "შვეცია"),
            ("ki", "Sweden"),
            ("kk", "Швеция"),
            ("kl", "Sweden"),
            ("km", "ស\u{17ca}\u{17bb}យអែត"),
            ("kn", "ಸ\u{ccd}ವೀಡನ\u{ccd}"),
            ("ko", "스웨덴"),
            ("ku", "Swêd"),
            ("kv", "Швеция"),
            ("kw", "Swedherwyk"),
            ("ky", "Швеция"),
            ("lo", "ປະເທດຊ\u{eb9}ແອດ"),
            ("lt", "Švedija"),
            ("lv", "Zviedrija"),
            ("mi", "Huitene"),
            ("mk", "Шведска"),
            ("ml", "സ\u{d4d}വീഡന\u{d4d}\u{200d}"),
            ("mn", "Швед"),
            ("mr", "स\u{94d}वीडन"),
            ("ms", "Sweden"),
            ("mt", "Svezja"),
            (
                "my",
                "ဆ\u{103d}\u{102e}ဒင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Widen"),
            ("nb", "Sverige"),
            ("ne", "स\u{94d}विड\u{947}न"),
            ("nl", "Zweden"),
            ("nn", "Sverige"),
            ("nv", "Chʼah Bideeʼí Dineʼé Bikéyah"),
            ("oc", "Suècia"),
            ("or", "ସ\u{b4d}ବୀଡେନ"),
            ("pa", "ਸਵੀਡਨ"),
            ("pi", "स\u{94d}वीडन"),
            ("pl", "Szwecja"),
            ("ps", "سویډن"),
            ("pt", "Suécia"),
            ("pt_BR", "Suécia"),
            ("ro", "Suedia"),
            ("ru", "Швеция"),
            ("rw", "Suwede"),
            ("sc", "Isvètzia"),
            ("sd", "سويڊن"),
            ("si", "ස\u{dca}ව\u{dd3}ඩනය"),
            ("sk", "Švédsko"),
            ("sl", "Švedska"),
            ("so", "Iswidhan"),
            ("sq", "Suedi"),
            ("sr", "Шведска"),
            ("sv", "Sverige"),
            ("sw", "Sweden"),
            ("ta", "சுவ\u{bc0}டன\u{bcd}"),
            ("te", "స\u{c4d}వ\u{c40}డన\u{c4d}"),
            ("tg", "Шветсия"),
            ("th", "สว\u{e35}เดน"),
            ("ti", "ስወደን"),
            ("tk", "Şwesiýa"),
            ("tl", "Sweden"),
            ("tr", "İsveç"),
            ("tt", "İсвәҗ, Шведсиа"),
            ("ug", "شىۋېتسىيە"),
            ("uk", "Швеція"),
            ("ur", "سویڈن"),
            ("uz", "Shvetsiya"),
            ("ve", "Sweden"),
            ("vi", "Thuỵ Điển"),
            ("wa", "Suwede"),
            ("wo", "Suweed"),
            ("xh", "Sweden"),
            ("yo", "Swídìn"),
            ("zh_CN", "瑞典"),
            ("zh_HK", "瑞典"),
            ("zh_TW", "瑞典"),
            ("zu", "ISwidi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
