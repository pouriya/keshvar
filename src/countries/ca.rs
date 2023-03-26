// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Canada

#[cfg(all(feature = "ca", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::CA;
    pub const ALPHA3: Alpha3 = Alpha3::CAN;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: &str = "CAD";
    pub const GEC: Option<GEC> = Some(GEC::CA);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::CAN);
    pub const ISO_SHORT_NAME: &str = "Canada";
    pub const ISO_LONG_NAME: &str = "Canada";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Canadian");
    pub const NUMBER: &str = "124";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> =
        Some("[ABCEGHJKLMNPRSTVXY]\\d[ABCEGHJ-NPRSTV-Z] ?\\d[ABCEGHJ-NPRSTV-Z]\\d");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAmerica);
    pub const UN_LOCODE: &str = "CA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Canada", "Kanada", "Canadá", "カナダ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Canada"),
        ("af", "Kanada"),
        ("ak", "Canada"),
        ("am", "Canada"),
        ("an", "Canadá"),
        ("ar", "كندا"),
        ("as", "ক\u{9be}ন\u{9be}ড\u{9be}"),
        ("ay", "Canada"),
        ("az", "Kanada"),
        ("ba", "Canada"),
        ("be", "Канада"),
        ("bg", "Канада"),
        ("bi", "Canada"),
        ("bn", "ক\u{9be}ন\u{9be}ড\u{9be}"),
        ("bn_IN", "ক\u{9be}ন\u{9be}ড\u{9be}"),
        ("br", "Kanada"),
        ("bs", "Kanada"),
        ("ca", "Canadà"),
        ("ce", "Канада"),
        ("ch", "Canada"),
        ("cs", "Kanada"),
        ("cv", "Канада"),
        ("cy", "Canada"),
        ("da", "Canada"),
        ("de", "Kanada"),
        ("dv", "ކ\u{7ac}ނ\u{7ac}ޑ\u{7a7}"),
        ("dz", "ཀ\u{f7a}་ན་ཌ།"),
        ("ee", "Canada"),
        ("el", "Καναδάς"),
        ("en", "Canada"),
        ("eo", "Kanado"),
        ("es", "Canadá"),
        ("et", "Kanada"),
        ("eu", "Kanada"),
        ("fa", "کانادا"),
        ("ff", "Canada"),
        ("fi", "Kanada"),
        ("fo", "Kanada"),
        ("fr", "Canada"),
        ("fy", "Kanada"),
        ("ga", "Ceanada"),
        ("gl", "Canadá"),
        ("gn", "Canada"),
        ("gu", "ક\u{ac7}ન\u{ac7}ડા"),
        ("gv", "Yn Chanadey"),
        ("ha", "Kanada"),
        ("he", "קנדה"),
        ("hi", "कनाडा"),
        ("hr", "Kanada"),
        ("ht", "Kanada"),
        ("hu", "Kanada"),
        ("hy", "Կանադա"),
        ("ia", "Canada"),
        ("id", "Kanada"),
        ("io", "Kanada"),
        ("is", "Kanada"),
        ("it", "Canada"),
        ("iu", "ᑲᓇᑕ"),
        ("ja", "カナダ"),
        ("ka", "კანადა"),
        ("ki", "Canada"),
        ("kk", "Канада"),
        ("kl", "Canada"),
        ("km", "កាណាដា"),
        ("kn", "ಕ\u{cc6}ನಡ"),
        ("ko", "캐나다"),
        ("ku", "Kanada"),
        ("kv", "Канада"),
        ("kw", "Kanada"),
        ("ky", "Канада"),
        ("lo", "ປະເທດການາດາ"),
        ("lt", "Kanada"),
        ("lv", "Kanāda"),
        ("mi", "Kānata"),
        ("mk", "Канада"),
        ("ml", "ക\u{d3e}നഡ"),
        ("mn", "Канад"),
        ("mr", "क\u{945}नडा"),
        ("ms", "Kanada"),
        ("mt", "Kanada"),
        ("my", "ကနေဒါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Kanada"),
        ("nb", "Canada"),
        ("ne", "क\u{94d}यानाडा"),
        ("nl", "Canada"),
        ("nn", "Canada"),
        ("nv", "Deeteel Bikéyah"),
        ("oc", "Canadà"),
        ("or", "କ\u{b3e}ନ\u{b3e}ଡ\u{b3e}"),
        ("pa", "ਕ\u{a48}ਨ\u{a47}ਡਾ"),
        ("pi", "क\u{947}नडा"),
        ("pl", "Kanada"),
        ("ps", "کاناډا"),
        ("pt", "Canadá"),
        ("pt_BR", "Canadá"),
        ("ro", "Canada"),
        ("ru", "Канада"),
        ("rw", "Kanada"),
        ("sc", "Cànada"),
        ("sd", "ڪئناڊا"),
        ("si", "කැනඩ\u{dcf}ව"),
        ("sk", "Kanada"),
        ("sl", "Kanada"),
        ("so", "Kanada"),
        ("sq", "Kanada"),
        ("sr", "Канада"),
        ("sv", "Kanada"),
        ("sw", "Kanada"),
        ("ta", "கனட\u{bbe}"),
        ("te", "క\u{c46}నడ\u{c3e}"),
        ("tg", "Канада"),
        ("th", "แคนาดา"),
        ("ti", "ካናዳ"),
        ("tk", "Kanada"),
        ("tl", "Kanada"),
        ("tr", "Kanada"),
        ("tt", "Канада"),
        ("ug", "كانادا"),
        ("uk", "Канада"),
        ("ur", "کینیڈا"),
        ("uz", "Kanada"),
        ("ve", "Canada"),
        ("vi", "Ca-na-đa"),
        ("wa", "Canada"),
        ("wo", "Kanadaa"),
        ("xh", "Canada"),
        ("yo", "Kánádà"),
        ("zh_CN", "加拿大"),
        ("zh_HK", "加拿大"),
        ("zh_TW", "加拿大"),
        ("zu", "I Khanada"),
    ];
    #[cfg(all(feature = "ca", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 56.130366;
        pub const LONGITUDE: f64 = -106.346771;
        pub const MAX_LATITUDE: f64 = 83.6381;
        pub const MAX_LONGITUDE: f64 = -50.9766;
        pub const MIN_LATITUDE: f64 = 41.6765559;
        pub const MIN_LONGITUDE: f64 = -141.00187;
        pub const NORTHEAST_LATITUDE: f64 = 83.6381;
        pub const NORTHEAST_LONGITUDE: f64 = -50.9766;
        pub const SOUTHWEST_LATITUDE: f64 = 41.6765559;
        pub const SOUTHWEST_LONGITUDE: f64 = -141.00187;
    }
}
#[cfg(all(feature = "ca", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 56.130366,
            longitude: -106.346771,
            max_latitude: 83.6381,
            max_longitude: -50.9766,
            min_latitude: 41.6765559,
            min_longitude: -141.00187,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 83.6381,
                    longitude: -50.9766,
                },
                southwest: CountryGeoBound {
                    latitude: 41.6765559,
                    longitude: -141.00187,
                },
            },
        }
    }
}

#[cfg(all(feature = "ca", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::CA,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.9332706), longitude: Some(-116.5765035), max_latitude: Some(60.00006209999999), min_latitude: Some(48.9966671), max_longitude: Some(-109.9998551), min_longitude: Some(-120.0005219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alberta"), ("am", "አልቤርታ"), ("ar", "ألبرتا"), ("az", "Alberta"), ("be", "Правінцыя Альберта"), ("bg", "Албърта"), ("bn", "এ\u{9cd}য\u{9be}লব\u{9be}র\u{9cd}ট\u{9be}"), ("bs", "Alberta"), ("ca", "Alberta"), ("ccp", "𑄃𑄣\u{11134}𑄝𑄬𑄢\u{11134}𑄑"), ("ceb", "Alberta"), ("cs", "Alberta"), ("cy", "Alberta"), ("da", "Alberta"), ("de", "Alberta"), ("el", "Αλμπέρτα"), ("en", "Alberta"), ("es", "Alberta"), ("et", "Alberta"), ("eu", "Alberta"), ("fa", "آلبرتا"), ("fi", "Alberta"), ("fr", "Alberta"), ("ga", "Alberta"), ("gl", "Alberta"), ("gu", "એલ\u{acd}બર\u{acd}ટા"), ("he", "אלברטה"), ("hi", "अल\u{94d}बर\u{94d}टा"), ("hr", "Alberta"), ("hu", "Alberta"), ("hy", "Ալբերտա"), ("id", "Alberta"), ("is", "Alberta"), ("it", "Alberta"), ("ja", "アルバータ州"), ("jv", "Alberta"), ("ka", "ალბერტა"), ("kk", "Альберта"), ("kn", "ಆಲ\u{ccd}ಬರ\u{ccd}ಟಾ"), ("ko", "앨버타 주"), ("lt", "Alberta"), ("lv", "Alberta"), ("mk", "Алберта"), ("ml", "ആൽബർട\u{d4d}ട"), ("mn", "Альберта"), ("mr", "आल\u{94d}बर\u{94d}टा"), ("ms", "Alberta"), ("my", "အယ\u{103a}လဗားတားပြည\u{103a}နယ\u{103a}"), ("nb", "Alberta"), ("nl", "Alberta"), ("no", "Alberta"), ("pa", "ਐਲਬਰਟਾ"), ("pl", "Alberta"), ("pt", "Alberta"), ("ro", "Alberta"), ("ru", "Альберта"), ("si", "අල\u{dca}බෙර\u{dca}ට\u{dcf}"), ("sk", "Alberta"), ("sl", "Alberta"), ("so", "Alberta"), ("sq", "Alberta"), ("sr", "Алберта"), ("sr_Latn", "Alberta"), ("sv", "Alberta"), ("sw", "Alberta"), ("ta", "ஆல\u{bcd}பர\u{bcd}ட\u{bcd}ட\u{bbe}"), ("te", "అల\u{c4d}బర\u{c4d}ట\u{c3e}"), ("th", "ร\u{e31}ฐแอลเบอร\u{e4c}ตา"), ("tr", "Alberta"), ("uk", "Альберта"), ("ur", "البرٹا"), ("uz", "Alberta"), ("vi", "Alberta"), ("yue", "亞伯達"), ("yue_Hans", "亚伯达"), ("zh", "艾伯塔")]),
                        unofficial_name_list: ["Alberta"].to_vec(),
                    }
                ),
                (
                    "BC",
                    Subdivision{
                        name: "BC",
                        country_alpha2: Alpha2::CA,
                        code: "BC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.7266683), longitude: Some(-127.6476206), max_latitude: Some(60.0001489), min_latitude: Some(48.3089123), max_longitude: Some(-114.0542211), min_longitude: Some(-139.0570702)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Brits-Columbië"), ("am", "ብሪቲሽ ኮለምቢያ"), ("ar", "كولومبيا البريطانية"), ("az", "Britaniya Kolumbiyası"), ("be", "Правінцыя Брытанская Калумбія"), ("bg", "Британска Колумбия"), ("bn", "ব\u{9cd}রিটিশ কল\u{9be}ম\u{9cd}বিয\u{9bc}\u{9be}"), ("bs", "Britanska Kolumbija"), ("ca", "Colúmbia Britànica"), ("ccp", "𑄝\u{11133}𑄢\u{11128}𑄑\u{11128}𑄌\u{11134} 𑄇\u{11127}𑄣\u{11127}𑄟\u{11134}𑄝\u{11128}𑄠"), ("ceb", "British Columbia"), ("cs", "Britská Kolumbie"), ("cy", "British Columbia"), ("da", "Britisk Columbia"), ("de", "British Columbia"), ("el", "Βρετανική Κολομβία"), ("en", "British Columbia"), ("es", "Columbia Británica"), ("et", "Briti Columbia"), ("eu", "Columbia Britainiarra"), ("fa", "بریتیش کلمبیا"), ("fi", "Brittiläinen Kolumbia"), ("fr", "Colombie-Britannique"), ("ga", "An Cholóim Bhriotanach"), ("gl", "Columbia Británica - British Columbia"), ("gu", "બ\u{acd}રિટિશ કોલમ\u{acd}બિયા"), ("he", "קולומביה הבריטית"), ("hi", "ब\u{94d}रिटिश कोलम\u{94d}बिया"), ("hr", "Britanska Kolumbija"), ("hu", "Brit Columbia"), ("hy", "Բրիտանական Կոլումբիա"), ("id", "British Columbia"), ("is", "Breska Kólumbía"), ("it", "Columbia Britannica"), ("ja", "ブリティッシュコロンビア州"), ("jv", "British Columbia"), ("ka", "ბრიტანეთის კოლუმბია"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಟ\u{cbf}ಷ\u{ccd}\u{200c}\u{200c} ಕೊಲಂಬ\u{cbf}ಯಾ"), ("ko", "브리티시컬럼비아 주"), ("lt", "Britų Kolumbija"), ("lv", "Britu Kolumbija"), ("mk", "Британска Колумбија"), ("ml", "ബ\u{d4d}രിട\u{d4d}ടീഷ\u{d4d} കൊളമ\u{d4d}പിയ"), ("mn", "Бритиш Коламбиа"), ("mr", "ब\u{94d}रिटिश कोल\u{902}बिया"), ("ms", "British Columbia"), ("my", "ဗြ\u{102d}တ\u{102d}သျ\u{103e} က\u{102d}\u{102f}လ\u{1036}ဘ\u{102e}ယာ ပြည\u{103a}နယ\u{103a}"), ("nb", "Britisk Columbia"), ("nl", "Brits-Columbia"), ("no", "Britisk Columbia"), ("pa", "ਬਰੀਟੀਸ\u{a3c} ਕ\u{a4b}ਲ\u{a70}ਬਿਆ"), ("pl", "Kolumbia Brytyjska"), ("pt", "Colúmbia Britânica"), ("ro", "Columbia Britanică"), ("ru", "Британская Колумбия"), ("si", "බ\u{dca}\u{200d}ර\u{dd2}ට\u{dd2}ෂ\u{dca} කොලොම\u{dca}බ\u{dd2}ය\u{dcf}"), ("sk", "Britská Kolumbia"), ("sl", "Britanska Kolumbija"), ("sq", "Kolumbia Britanike"), ("sr", "Британска Колумбија"), ("sr_Latn", "Britanska Kolumbija"), ("sv", "British Columbia"), ("sw", "British Kolumbia"), ("ta", "பிரிட\u{bcd}டிசு கொலம\u{bcd}பிய\u{bbe}"), ("te", "బ\u{c4d}ర\u{c3f}ట\u{c3f}ష\u{c4d} క\u{c4a}లంబ\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐบร\u{e34}ต\u{e34}ชโคล\u{e31}มเบ\u{e35}ย"), ("tr", "Britanya Kolumbiyası"), ("uk", "Британська Колумбія"), ("ur", "برٹش کولمبیا"), ("uz", "Britaniya Kolumbiyasi"), ("vi", "British Columbia"), ("yue", "卑詩省"), ("yue_Hans", "卑诗省"), ("zh", "不列颠哥伦比亚")]),
                        unofficial_name_list: ["Colombie-Britannique"].to_vec(),
                    }
                ),
                (
                    "MB",
                    Subdivision{
                        name: "MB",
                        country_alpha2: Alpha2::CA,
                        code: "MB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.7608608), longitude: Some(-98.81387629999999), max_latitude: Some(60.0001031), min_latitude: Some(48.998861), max_longitude: Some(-88.9852265), min_longitude: Some(-102.0000041)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Manitoba"), ("am", "ማኒቶባ"), ("ar", "مانيتوبا"), ("az", "Manitoba"), ("be", "Правінцыя Манітоба"), ("bg", "Манитоба"), ("bn", "ম\u{9cd}য\u{9be}নিটোব\u{9be}"), ("bs", "Manitoba"), ("ca", "Manitoba"), ("ccp", "𑄟𑄚\u{11128}𑄑\u{11127}𑄝"), ("ceb", "Manitoba"), ("cs", "Manitoba"), ("cy", "Manitoba"), ("da", "Manitoba"), ("de", "Manitoba"), ("el", "Μανιτόμπα"), ("en", "Manitoba"), ("es", "Manitoba"), ("et", "Manitoba provints"), ("eu", "Manitoba"), ("fa", "مانیتوبا"), ("fi", "Manitoba"), ("fr", "Manitoba"), ("ga", "Manitoba"), ("gl", "Manitoba"), ("gu", "મ\u{ac7}નિટોબા"), ("he", "מניטובה"), ("hi", "मानिटोबा"), ("hr", "Manitoba"), ("hu", "Manitoba"), ("hy", "Մանիթոբա"), ("id", "Manitoba"), ("is", "Manitoba"), ("it", "Manitoba"), ("ja", "マニトバ州"), ("ka", "მანიტობა"), ("kk", "Манитоба"), ("kn", "ಮ\u{ccd}ಯಾನ\u{cbf}ಟೋಬ"), ("ko", "매니토바 주"), ("lt", "Manitoba"), ("lv", "Manitoba"), ("mk", "Манитоба"), ("ml", "മനിറ\u{d4d}റോബ"), ("mn", "Манитоба"), ("mr", "म\u{945}निटोबा"), ("ms", "Manitoba"), ("my", "မန\u{102e}တ\u{102d}\u{102f}းဗားပြည\u{103a}နယ\u{103a}"), ("nb", "Manitoba"), ("nl", "Manitoba"), ("no", "Manitoba"), ("pa", "ਮਾਨੀਟ\u{a4b}ਬਾ"), ("pl", "Manitoba"), ("pt", "Manitoba"), ("ro", "Manitoba"), ("ru", "Манитоба"), ("si", "මන\u{dd2}ටොබ\u{dcf}"), ("sk", "Manitoba"), ("sl", "Manitoba"), ("sq", "Manitoba"), ("sr", "Манитоба"), ("sr_Latn", "Manitoba"), ("sv", "Manitoba"), ("sw", "Manitoba"), ("ta", "ம\u{bbe}னிட\u{bcd}டோப\u{bbe}"), ("te", "మన\u{c3f}ట\u{c4b}బ\u{c3e}"), ("th", "ร\u{e31}ฐแมน\u{e34}โทบา"), ("tr", "Manitoba"), ("uk", "Манітоба"), ("ur", "مانیٹوبا"), ("uz", "Manitoba (Kanada)"), ("vi", "Manitoba"), ("yue", "緬尼吐巴"), ("yue_Hans", "缅尼吐巴"), ("zh", "曼尼托巴")]),
                        unofficial_name_list: ["Manitoba"].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::CA,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5653163), longitude: Some(-66.46191639999999), max_latitude: Some(48.1735157), min_latitude: Some(44.499574), max_longitude: Some(-63.7706403), min_longitude: Some(-69.05339289999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nieu-Brunswyk"), ("am", "ኒው ብረንዝውክ"), ("ar", "نيو برونزويك"), ("az", "Nyu-Brunsvik"), ("be", "Правінцыя Нью-Брансуік"), ("bg", "Ню Брънзуик"), ("bn", "নিউ ব\u{9cd}র\u{9be}ন\u{9cd}সউইক"), ("bs", "Novi Brunswick"), ("ca", "Nova Brunsvic"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄝\u{11133}𑄢𑄚\u{11134}𑄥\u{1112a}𑄃\u{11128}𑄇\u{11134}"), ("ceb", "New Brunswick (lalawigan)"), ("cs", "Nový Brunšvik"), ("cy", "Brunswick Newydd"), ("da", "New Brunswick"), ("de", "New Brunswick"), ("el", "Νιου Μπράνσγουικ"), ("en", "New Brunswick"), ("es", "Nuevo Brunswick"), ("et", "New Brunswick"), ("eu", "Brunswick Berria"), ("fa", "نیوبرانزویک"), ("fi", "New Brunswick"), ("fr", "Nouveau-Brunswick"), ("ga", "New Brunswick"), ("gl", "Nova Brunswick"), ("gu", "ન\u{acd}ય\u{ac2} બ\u{acd}ર\u{ac1}ન\u{acd}સવિક"), ("he", "ניו ברנזוויק"), ("hi", "न\u{94d}य\u{942} ब\u{94d}र\u{902}सविक"), ("hr", "Novi Brunswick"), ("hu", "Új-Brunswick"), ("hy", "Նյու Բրանսուիկ"), ("id", "New Brunswick"), ("is", "Nýja-Brúnsvík"), ("it", "Nuovo Brunswick"), ("ja", "ニューブランズウィック州"), ("ka", "ნიუ-ბრანსუიკი"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಬ\u{ccd}ರನ\u{ccd}ಸ\u{ccd}ವ\u{cbf}ಕ\u{ccd}"), ("ko", "뉴브런즈윅 주"), ("lt", "Naujasis Bransvikas"), ("lv", "Ņūbransvika"), ("mk", "Њу Бранзвик"), ("mn", "Нью-Брансвик"), ("mr", "न\u{94d}य\u{942} ब\u{94d}र\u{941}न\u{94d}सविक"), ("ms", "New Brunswick"), ("nb", "New Brunswick"), ("nl", "New Brunswick"), ("no", "New Brunswick"), ("pa", "ਨਿਊ ਬਰ\u{a70}ਸਵਿਕ"), ("pl", "Nowy Brunszwik"), ("pt", "Nova Brunswick"), ("ro", "Noul Brunswick"), ("ru", "Нью-Брансуик"), ("si", "නව බ\u{dca}\u{200d}රනස\u{dca}ව\u{dd2}ක\u{dca}"), ("sk", "New Brunswick"), ("sl", "Novi Brunswick"), ("sq", "New Brunswick"), ("sr", "Њу Брансвик"), ("sr_Latn", "Nju Bransvik"), ("sv", "New Brunswick"), ("sw", "New Brunswick"), ("ta", "நியூ பிரன\u{bcd}சுவிக\u{bcd}"), ("te", "న\u{c4d}యూ బ\u{c4d}రున\u{c4d}స\u{c4d}\u{200c}వ\u{c3f}క\u{c4d}"), ("th", "ร\u{e31}ฐน\u{e34}วบร\u{e31}นสว\u{e34}ก"), ("tr", "New Brunswick"), ("uk", "Нью-Брансвік"), ("ur", "نیو برنزویک"), ("uz", "Nyu-bransuyk"), ("vi", "New Brunswick"), ("yue", "紐賓士域省"), ("yue_Hans", "纽宾士域省"), ("zh", "新不倫瑞克")]),
                        unofficial_name_list: ["Nouveau-Brunswick"].to_vec(),
                    }
                ),
                (
                    "NL",
                    Subdivision{
                        name: "NL",
                        country_alpha2: Alpha2::CA,
                        code: "NL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1355091), longitude: Some(-57.6604364), max_latitude: Some(60.37627010000001), min_latitude: Some(46.6114571), max_longitude: Some(-52.6194086), min_longitude: Some(-67.8216849)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Newfoundland en Labrador"), ("am", "ኑፈንላንድና ላብረዶር"), ("ar", "نيوفاوندلاند واللابرادور"), ("az", "Nyufaundlend və Labrador"), ("be", "Правінцыя Ньюфаўндленд і Лабрадор"), ("bg", "Нюфаундленд и Лабрадор"), ("bn", "নিউফ\u{9be}উন\u{9cd}ডল\u{9cd}য\u{9be}ন\u{9cd}ড এবং ল\u{9cd}য\u{9be}ব\u{9cd}র\u{9be}ডর"), ("bs", "Newfoundland i Labrador"), ("ca", "Terranova i Labrador"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄜𑄅\u{1112a}𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134} 𑄃\u{11133}𑄃 𑄣𑄝\u{11133}𑄢𑄓\u{1112e}𑄢\u{11134}"), ("ceb", "Newfoundland and Labrador"), ("cs", "Newfoundland a Labrador"), ("cy", "Newfoundland a Labrador"), ("da", "Newfoundland og Labrador"), ("de", "Neufundland und Labrador"), ("el", "Νέα Γη και Λαμπραντόρ"), ("en", "Newfoundland and Labrador"), ("es", "Terranova y Labrador"), ("et", "Newfoundland ja Labrador"), ("eu", "Ternua eta Labrador"), ("fa", "نیوفاندلند و لابرادور"), ("fi", "Newfoundland ja Labrador"), ("fr", "Terre-Neuve-et-Labrador"), ("ga", "Talamh an Éisc agus Labradar"), ("gl", "Terra Nova e Labrador"), ("gu", "ન\u{acd}ય\u{ac1}ફાઉન\u{acd}ડલ\u{ac7}ન\u{acd}ડ એન\u{acd}ડ લ\u{ac7}બ\u{acd}રાડોર"), ("he", "ניופאונדלנד ולברדור"), ("hi", "न\u{94d}य\u{942}फाउ\u{902}डल\u{948}\u{902}ड और ल\u{948}ब\u{94d}राडोर"), ("hr", "Newfoundland i Labrador"), ("hu", "Új-Fundland és Labrador"), ("hy", "Նյուֆաունդլենդ և Լաբրադոր"), ("id", "Newfoundland dan Labrador"), ("is", "Nýfundnaland og Labrador"), ("it", "Terranova e Labrador"), ("ja", "ニューファンドランド・ラブラドール州"), ("ka", "ნიუფაუნდლენდი და ლაბრადორი"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ಫ\u{ccc}ಂಡ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಮತ\u{ccd}ತು ಲ\u{ccd}ಯಾಬ\u{ccd}ರಡಾರ\u{ccd}"), ("ko", "뉴펀들랜드 래브라도 주"), ("lt", "Niufaundlandas ir Labradoras"), ("lv", "Ņūfaundlenda un Labradora"), ("mk", "Њуфаундленд и Лабрадор"), ("mn", "Ньюфаундленд ба Лабрадор"), ("mr", "न\u{94d}य\u{942} फाउ\u{902}डल\u{902}ड आणि लाब\u{94d}राडोर"), ("ms", "Newfoundland dan Labrador"), ("my", "နယ\u{1030}းဖောင\u{103a}လန\u{103a} န\u{103e}င\u{1037}\u{103a} လာဘရာဒေါ ပြည\u{103a}နယ\u{103a}"), ("nb", "Newfoundland og Labrador"), ("nl", "Newfoundland en Labrador"), ("no", "Newfoundland og Labrador"), ("pa", "ਨਿਊਫ\u{a3c}\u{a70}ਡਲ\u{a48}\u{a02}ਡ ਅਤ\u{a47} ਲਾਬਰਾਡ\u{a4b}ਰ"), ("pl", "Nowa Fundlandia i Labrador"), ("pt", "Terra Nova e Labrador"), ("ro", "Newfoundland și Labrador"), ("ru", "Ньюфаундленд и Лабрадор"), ("si", "න\u{dd2}ව\u{dca} ෆව\u{dd4}න\u{dca}ඩ\u{dca}ලන\u{dca}ඩ\u{dca} සහ ලබ\u{dca}\u{200d}රඩොර\u{dca}"), ("sk", "Newfoundland a Labrador"), ("sl", "Nova Fundlandija in Labrador"), ("sq", "Toka e Re dhe Labradori"), ("sr", "Њуфаундленд и Лабрадор"), ("sr_Latn", "Njufaundlend i Labrador"), ("sv", "Newfoundland och Labrador"), ("sw", "Newfoundland and Labrador"), ("ta", "நியூஃபவுன\u{bcd}ல\u{bbe}ந\u{bcd}து மற\u{bcd}றும\u{bcd} ல\u{bbe}ப\u{bcd}ரடோர\u{bcd}"), ("te", "న\u{c4d}యూఫ\u{c4c}ండ\u{c4d} ల\u{c4d}య\u{c3e}ండ\u{c4d} మర\u{c3f}యు ల\u{c3e}బ\u{c4d}ర\u{c3e}డ\u{c3e}ర\u{c4d}"), ("th", "ร\u{e31}ฐน\u{e34}วฟ\u{e31}นด\u{e4c}แลนด\u{e4c}และแลบราดอร\u{e4c}"), ("tr", "Newfoundland ve Labrador"), ("uk", "Ньюфаундленд і Лабрадор"), ("ur", "نیو فاؤنڈ لینڈ اور لیبراڈور"), ("uz", "Nyufaundlend va Labrador"), ("vi", "Newfoundland và Labrador"), ("yo", "Newfoundland àti Labrador"), ("yo_BJ", "Newfoundland àti Labrador"), ("yue", "紐芬蘭與拉布拉多"), ("yue_Hans", "纽芬兰与拉布拉多"), ("zh", "紐芬蘭與拉布拉多")]),
                        unofficial_name_list: ["Newfoundland", "Terre-Neuve", "Terre-Neuve-et-Labrador"].to_vec(),
                    }
                ),
                (
                    "NS",
                    Subdivision{
                        name: "NS",
                        country_alpha2: Alpha2::CA,
                        code: "NS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.68198659999999), longitude: Some(-63.744311), max_latitude: Some(47.2277438), min_latitude: Some(43.3918805), max_longitude: Some(-59.676914), min_longitude: Some(-66.3948186)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nova Scotia"), ("am", "ኖቫ ስኮሻ"), ("ar", "نوفا سكوشا"), ("az", "Yeni Şotlandiya"), ("be", "Правінцыя Новая Шатландыя"), ("bg", "Нова Скотия"), ("bn", "নভ\u{9be} স\u{9cd}কশিয\u{9bc}\u{9be}"), ("bs", "Nova Scotia"), ("ca", "Nova Escòcia"), ("ccp", "𑄚\u{1112e}𑄞 𑄌\u{11133}𑄇\u{11127}𑄑\u{11128}𑄠"), ("ceb", "Nova Scotia"), ("cs", "Nové Skotsko"), ("cy", "Nova Scotia"), ("da", "Nova Scotia"), ("de", "Nova Scotia"), ("el", "Νέα Σκωτία"), ("en", "Nova Scotia"), ("es", "Nueva Escocia"), ("et", "Nova Scotia"), ("eu", "Eskozia Berria"), ("fa", "نوا اسکوشیا"), ("fi", "Nova Scotia"), ("fr", "Nouvelle-Écosse"), ("ga", "Albain Nua"), ("gl", "Nova Escocia - Nova Scotia"), ("gu", "નોવા સ\u{acd}કોટીઆ"), ("he", "נובה סקוטיה"), ("hi", "नोवा स\u{94d}कॉटिया"), ("hr", "Nova Škotska"), ("hu", "Új-Skócia"), ("hy", "Նոր Շոտլանդիա"), ("id", "Nova Scotia"), ("is", "Nýja-Skotland"), ("it", "Nuova Scozia"), ("ja", "ノバスコシア州"), ("ka", "ახალი შოტლანდია"), ("kn", "ನೋವಾ ಸ\u{ccd}ಕಾಟ\u{cbf}ಯಾ"), ("ko", "노바스코샤 주"), ("lt", "Naujoji Škotija"), ("lv", "Jaunskotija"), ("mk", "Нова Шкотска"), ("ml", "നോവ സ\u{d4d}കോട\u{d4d}ടിയ"), ("mn", "Нова Скоша"), ("mr", "नोव\u{94d}हा स\u{94d}कॉशिया"), ("ms", "Nova Scotia"), ("my", "န\u{102d}\u{102f}ဗာစက\u{102d}\u{102f}းရ\u{103e}ားပြည\u{103a}နယ\u{103a}"), ("nb", "Nova Scotia"), ("nl", "Nova Scotia"), ("no", "Nova Scotia"), ("pa", "ਨ\u{a4b}ਵਾ ਸਕ\u{a4b}ਸ\u{a3c}ਾ"), ("pl", "Nowa Szkocja"), ("pt", "Nova Escócia"), ("ro", "Noua Scoție"), ("ru", "Новая Шотландия"), ("si", "නොව\u{dcf} ස\u{dca}කොට\u{dd2}ය\u{dcf}"), ("sk", "Nové Škótsko"), ("sl", "Nova Škotska"), ("sq", "Skocia e Re"), ("sr", "Нова Шкотска"), ("sr_Latn", "Nova Škotska"), ("sv", "Nova Scotia"), ("sw", "Nova Scotia"), ("ta", "நோவ\u{bbe} ஸ\u{bcd}கோசிய\u{bbe}"), ("te", "న\u{c4b}వ\u{c3e} స\u{c4d}క\u{c4b}ట\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐโนวาสโกเช\u{e35}ย"), ("tr", "Yeni İskoçya"), ("uk", "Нова Шотландія"), ("ur", "نووا سکوشیا"), ("uz", "Yangi Shotlandiya"), ("vi", "Nova Scotia"), ("yue", "諾華斯高沙"), ("yue_Hans", "诺华斯高沙"), ("zh", "新斯科舍")]),
                        unofficial_name_list: ["Nouvelle-Écosse"].to_vec(),
                    }
                ),
                (
                    "NT",
                    Subdivision{
                        name: "NT",
                        country_alpha2: Alpha2::CA,
                        code: "NT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(64.8255441), longitude: Some(-124.8457334), max_latitude: Some(78.7613421), min_latitude: Some(59.99995399999999), max_longitude: Some(-101.999999), min_longitude: Some(-136.4687053)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Territory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noordwestelike gebiede"), ("am", "ስሜን-ምዕራብ ግዛቶች"), ("ar", "الأقاليم الشمالية الغربية"), ("az", "Şimal-Qərb Əraziləri"), ("be", "Паўночна-Заходнія тэрыторыі"), ("bg", "Северозападни територии"), ("bn", "উত\u{9cd}তর পশ\u{9cd}চিম অঞ\u{9cd}চল"), ("bs", "Sjeverozapadne teritorije"), ("ca", "Territoris del Nord-oest"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢𑄬𑄘\u{11128} 𑄢𑄬𑄎\u{11133}𑄠\u{11127}𑄠𑄚\u{11128}"), ("ceb", "Northwest Territories"), ("cs", "Severozápadní teritoria"), ("cy", "Tiriogaethau’r Gogledd-orllewin"), ("da", "Northwest Territories"), ("de", "Nordwest-Territorien"), ("el", "Βορειοδυτικά Εδάφη"), ("en", "Northwest Territories"), ("es", "Territorios del Noroeste"), ("et", "Loodealad"), ("eu", "Ipar-mendebaldeko lurraldeak"), ("fa", "نواحی شمال غرب"), ("fi", "Luoteisterritoriot"), ("fr", "Territoires du Nord-Ouest"), ("ga", "Críocha an Iarthuaiscirt"), ("gl", "Territorios do Noroeste"), ("gu", "નોર\u{acd}થવ\u{ac7}સ\u{acd}ટ ટ\u{ac7}રિટરીઝ"), ("he", "הטריטוריות הצפון-מערביות"), ("hi", "नॉर\u{94d}थव\u{947}स\u{94d}ट ट\u{947}रीटरीज\u{93c}"), ("hr", "Sjeverozapadni teritoriji"), ("hu", "Északnyugati területek"), ("hy", "Հյուսիսարևմտյան տարածքներ"), ("id", "Wilayah Barat Laut"), ("is", "Norðvesturhéruðin"), ("it", "Territori del Nord-Ovest"), ("ja", "ノースウエスト準州"), ("ka", "ჩრდილო-დასავლეთი ტერიტორიები"), ("kn", "ವಾಯುವ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯಗಳು"), ("ko", "노스웨스트 준주"), ("lt", "Šiaurės Vakarai"), ("lv", "Ziemeļrietumu Teritorijas"), ("mk", "Северозападни Територии"), ("mn", "Баруун Хойд Нутаг Дэвсгэр"), ("mr", "नॉर\u{94d}थव\u{947}स\u{94d}ट ट\u{947}रिटोरीज"), ("ms", "Wilayah Barat Laut"), ("nb", "Nordvestterritoriene"), ("nl", "Northwest Territories"), ("no", "Nordvestterritoriene"), ("pa", "ਉ\u{a71}ਤਰ-ਪ\u{a71}ਛਮੀ ਰਾਜਖ\u{a47}ਤਰ"), ("pl", "Terytoria Północno-Zachodnie"), ("pt", "Territórios do Noroeste"), ("ro", "Teritoriile de Nordvest"), ("ru", "Северо-Западные территории"), ("si", "උත\u{dd4}ර\u{dd4} බටහ\u{dd2}ර භ\u{dd4}ම\u{dd2}ය"), ("sk", "Severozápadné teritóriá"), ("sl", "Severozahodni teritoriji"), ("sq", "Territoret Veri-Perëndimore"), ("sr", "Северозападне територије"), ("sr_Latn", "Severozapadne teritorije"), ("sv", "Northwest Territories"), ("sw", "Northwest Territories"), ("ta", "வடமேற\u{bcd}கு நிலப\u{bcd}பகுதிகள\u{bcd}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d}\u{200c}వ\u{c46}స\u{c4d}ట\u{c4d} ట\u{c46}ర\u{c3f}టర\u{c40}స\u{c4d}"), ("th", "นอร\u{e4c}ทเวสต\u{e4c}เทร\u{e4c}ร\u{e34}ทอร\u{e35}ส\u{e4c}"), ("tr", "Kuzeybatı Toprakları"), ("uk", "Північно-Західні території"), ("ur", "شمال مغربی علاقہ جات، کینیڈا"), ("uz", "Shimoli-Gʻarbiy hududlar"), ("vi", "Các Lãnh thổ Tây Bắc"), ("yue", "西北地區"), ("yue_Hans", "西北地区"), ("zh", "西北地区")]),
                        unofficial_name_list: ["Territoires du Nord-Ouest"].to_vec(),
                    }
                ),
                (
                    "NU",
                    Subdivision{
                        name: "NU",
                        country_alpha2: Alpha2::CA,
                        code: "NU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(70.2997711), longitude: Some(-83.1075769), max_latitude: Some(83.0956638), min_latitude: Some(51.6406985), max_longitude: Some(-61.1794384), min_longitude: Some(-121.0492491)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Territory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nunavut"), ("am", "ኑናቩት"), ("ar", "نونافوت"), ("az", "Nunavut"), ("be", "Нунавут"), ("bg", "Нунавут"), ("bn", "ন\u{9c1}ন\u{9be}ভ\u{9c1}ত"), ("bs", "Nunavut"), ("ca", "Nunavut"), ("ccp", "𑄚\u{1112a}𑄚𑄞𑄖\u{11134}"), ("ceb", "Nunavut"), ("cs", "Nunavut"), ("cy", "Nunavut"), ("da", "Nunavut"), ("de", "Nunavut"), ("el", "Νούναβουτ"), ("en", "Nunavut"), ("es", "Nunavut"), ("et", "Nunavut"), ("eu", "Nunavut"), ("fa", "نوناووت"), ("fi", "Nunavut"), ("fr", "Nunavut"), ("ga", "Nunavut"), ("gl", "Nunavut"), ("gu", "ન\u{ac1}નાવત"), ("he", "נונאווט"), ("hi", "न\u{941}नाव\u{941}त"), ("hr", "Nunavut"), ("hu", "Nunavut"), ("hy", "Նունավութ"), ("id", "Nunavut"), ("is", "Nunavut"), ("it", "Nunavut"), ("ja", "ヌナブト準州"), ("ka", "ნუნავუტი"), ("kk", "Нунавут"), ("kn", "ನುನಾವುಟ\u{ccd}"), ("ko", "누나부트 준주"), ("lt", "Nunavutas"), ("lv", "Nunavuta"), ("mk", "Нунавут"), ("ml", "ന\u{d41}ന\u{d3e}വട\u{d4d}"), ("mn", "Нунавут"), ("mr", "न\u{941}नाव\u{94d}ह\u{941}त"), ("ms", "Nunavut"), ("nb", "Nunavut"), ("nl", "Nunavut"), ("no", "Nunavut"), ("pa", "ਨ\u{a42}ਨਾਵ\u{a41}ਤ"), ("pl", "Nunavut"), ("pt", "Nunavut"), ("ro", "Nunavut"), ("ru", "Нунавут"), ("si", "නනව\u{dd4}ට\u{dca}"), ("sk", "Nunavut"), ("sl", "Nunavut"), ("sq", "Nunavut"), ("sr", "Нунавут"), ("sr_Latn", "Nunavut"), ("sv", "Nunavut"), ("sw", "Nunavut"), ("ta", "நுன\u{bbe}வுட\u{bcd}"), ("te", "నున\u{c3e}వుట\u{c4d}"), ("th", "น\u{e39}นาว\u{e38}ต"), ("tr", "Nunavut"), ("uk", "Нунавут"), ("ur", "نناوت"), ("uz", "Nunavut"), ("vi", "Nunavut"), ("yue", "努那烏"), ("yue_Hans", "努那乌"), ("zh", "努納武特")]),
                        unofficial_name_list: ["Nunavut"].to_vec(),
                    }
                ),
                (
                    "ON",
                    Subdivision{
                        name: "ON",
                        country_alpha2: Alpha2::CA,
                        code: "ON",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.253775), longitude: Some(-85.3232139), max_latitude: Some(56.8565279), min_latitude: Some(41.68134879999999), max_longitude: Some(-74.34388229999999), min_longitude: Some(-95.1562271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ontario"), ("am", "ኦንቴሪዮ"), ("ar", "أونتاريو"), ("az", "Ontario"), ("be", "Правінцыя Антарыа"), ("bg", "Онтарио"), ("bn", "অন\u{9cd}ট\u{9be}রিও"), ("bs", "Ontario"), ("ca", "Ontàrio"), ("ccp", "𑄃\u{11127}𑄚\u{11134}𑄑𑄢\u{11128}𑄃\u{1112e}"), ("ceb", "Ontario"), ("cs", "Ontario"), ("cy", "Ontario"), ("da", "Ontario"), ("de", "Ontario"), ("el", "Οντάριο"), ("en", "Ontario"), ("es", "Ontario"), ("et", "Ontario"), ("eu", "Ontario"), ("fa", "انتاریو"), ("fi", "Ontario"), ("fr", "Ontario"), ("ga", "Ontario"), ("gl", "Ontario"), ("gu", "ઑન\u{acd}ટ\u{ac7}રિઓ"), ("he", "אונטריו"), ("hi", "ओण\u{94d}टारियो"), ("hr", "Ontario"), ("hu", "Ontario"), ("hy", "Օնտարիո"), ("id", "Ontario"), ("is", "Ontario"), ("it", "Ontario"), ("ja", "オンタリオ州"), ("ka", "ონტარიო"), ("kn", "ಒಂಟಾರ\u{cbf}ಯೊ"), ("ko", "온타리오 주"), ("lt", "Ontarijas"), ("lv", "Ontārio"), ("mk", "Онтарио"), ("ml", "ഒണ\u{d4d}ട\u{d3e}റിയോ"), ("mn", "Онтарио"), ("mr", "ऑन\u{94d}टारियो"), ("ms", "Ontario"), ("my", "အ\u{103d}န\u{103a}တေးရ\u{102e}းယ\u{102d}\u{102f}းပြည\u{103a}နယ\u{103a}"), ("nb", "Ontario"), ("ne", "ओन\u{94d}टारियो"), ("nl", "Ontario"), ("no", "Ontario"), ("pa", "ਓ\u{a02}ਟਾਰਿਓ"), ("pl", "Ontario"), ("pt", "Ontário"), ("ro", "Ontario"), ("ru", "Онтарио"), ("si", "ඔන\u{dca}ට\u{dcf}ර\u{dd2}යෝ"), ("sk", "Ontário"), ("sl", "Ontario"), ("so", "Ontario"), ("sq", "Ontario"), ("sr", "Онтарио"), ("sr_Latn", "Ontario"), ("sv", "Ontario"), ("sw", "Ontario"), ("ta", "ஒன\u{bcd}ர\u{bbe}றியோ"), ("te", "అంట\u{c3e}ర\u{c3f}య\u{c4b}"), ("th", "ร\u{e31}ฐออนแทร\u{e35}โอ"), ("tr", "Ontario"), ("uk", "Онтаріо"), ("ur", "انٹاریو"), ("uz", "Ontario"), ("vi", "Ontario"), ("yue", "安大略"), ("yue_Hans", "安大略"), ("zh", "安大略")]),
                        unofficial_name_list: ["Ontario"].to_vec(),
                    }
                ),
                (
                    "PE",
                    Subdivision{
                        name: "PE",
                        country_alpha2: Alpha2::CA,
                        code: "PE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.510712), longitude: Some(-63.41681359999999), max_latitude: Some(47.4416628), min_latitude: Some(45.9481845), max_longitude: Some(-61.97075460000001), min_longitude: Some(-64.5661238)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Prins-Edward-Eiland"), ("am", "ፕርንስ ኤድወርድ አይለንድ"), ("ar", "جزيرة الأمير إدوارد"), ("az", "Şahzadə Eduard Adası"), ("be", "Востраў Прынца Эдуарда"), ("bg", "Остров Принц Едуард"), ("bn", "প\u{9cd}রিন\u{9cd}স এডওয\u{9bc}\u{9be}র\u{9cd}ড দ\u{9cd}বীপ"), ("bs", "Ostrvo Princa Edwarda"), ("ca", "Illa del Príncep Eduard"), ("ccp", "𑄛\u{11133}𑄢\u{11128}𑄚\u{11134}𑄥\u{11134} 𑄃𑄬𑄖\u{11134}𑄃\u{1112e}𑄠𑄢\u{11134}𑄓\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Prince Edward Island (lalawigan)"), ("cs", "Ostrov prince Edvarda"), ("cy", "Prince Edward Island"), ("da", "Prince Edward Island"), ("de", "Prince Edward Island"), ("el", "Νήσος του Πρίγκηπα Εδουάρδου"), ("en", "Prince Edward Island"), ("es", "Isla del Príncipe Eduardo"), ("et", "Prints Edwardi saar"), ("eu", "Eduardo Printzearen uhartea"), ("fa", "جزیره پرنس ادوارد"), ("fi", "Prinssi Edwardin saari"), ("fr", "Île-du-Prince-Édouard"), ("ga", "Oileán Phrionsa Éadbhard"), ("gl", "Illa do Príncipe Eduardo"), ("gu", "પ\u{acd}રિન\u{acd}સ એડવર\u{acd}ડ આઇલ\u{ac7}ન\u{acd}ડ"), ("he", "אי הנסיך אדוארד"), ("hi", "प\u{94d}रि\u{902}स एडवर\u{94d}ड द\u{94d}वीप"), ("hr", "Otok Princa Edwarda"), ("hu", "Prince Edward-sziget"), ("hy", "Էդուարդ արքայազնի կղզի"), ("id", "Pulau Pangeran Edward"), ("is", "Eyja Játvarðs prins"), ("it", "Isola del Principe Edoardo"), ("ja", "プリンスエドワードアイランド州"), ("ka", "პრინს-ედუარდის კუნძული"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ನ\u{ccd}ಸ\u{ccd} ಎಡ\u{ccd}ವರ\u{ccd}ಡ\u{ccd} ಐಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "프린스에드워드아일랜드 주"), ("lt", "Princo Edvardo sala"), ("lv", "Prinča Edvarda Sala"), ("mk", "Остров Принц Едвард"), ("mn", "Принс Эдвардын Арал"), ("mr", "प\u{94d}रिन\u{94d}स एडवर\u{94d}ड आयल\u{902}ड"), ("ms", "Pulau Prince Edward"), ("nb", "Prince Edward Island"), ("nl", "Prins Edwardeiland"), ("no", "Prince Edward Island"), ("pa", "ਪ\u{a4d}ਰਿ\u{a70}ਸ ਐਡਵਰਡ ਟਾਪ\u{a42}"), ("pl", "Wyspa Księcia Edwarda"), ("pt", "Ilha do Príncipe Eduardo"), ("ro", "Insula Prințului Edward"), ("ru", "Остров Принца Эдуарда"), ("si", "ප\u{dca}\u{200d}ර\u{dd2}න\u{dca}ස\u{dca} එඩ\u{dca}වඩ\u{dca} ද\u{dd6}පත"), ("sk", "Ostrov princa Eduarda"), ("sl", "Otok princa Edvarda"), ("sq", "Ishulli i Princit Eduard"), ("sr", "Острво Принца Едварда"), ("sr_Latn", "Ostrvo Princa Edvarda"), ("sv", "Prince Edward Island"), ("sw", "Prince Edward Island"), ("ta", "இளவரசர\u{bcd} எட\u{bcd}வர\u{bcd}ட\u{bcd} த\u{bc0}வு"), ("te", "ప\u{c4d}ర\u{c3f}న\u{c4d}స\u{c4d} ఎడ\u{c4d}వర\u{c4d}డ\u{c4d} ద\u{c40}వ\u{c3f}"), ("th", "ร\u{e31}ฐพร\u{e34}นซ\u{e4c}เอ\u{e47}ดเว\u{e34}ร\u{e4c}ดไอแลนด\u{e4c}"), ("tr", "Prens Edward Adası"), ("uk", "Острів Принца Едварда"), ("ur", "پرنس ایڈورڈ آئی لینڈ"), ("uz", "Shahzoda Eduard oroli"), ("vi", "Đảo Hoàng tử Edward"), ("yue", "愛德華太子島"), ("yue_Hans", "爱德华太子岛"), ("zh", "愛德華王子島")]),
                        unofficial_name_list: ["Île-du-Prince-Édouard"].to_vec(),
                    }
                ),
                (
                    "QC",
                    Subdivision{
                        name: "QC",
                        country_alpha2: Alpha2::CA,
                        code: "QC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.9399159), longitude: Some(-73.5491361), max_latitude: Some(62.5830552), min_latitude: Some(44.9913581), max_longitude: Some(-57.1054859), min_longitude: Some(-79.7623371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Quebec"), ("am", "ኬበክ"), ("ar", "كيبك"), ("az", "Kvebek"), ("be", "Правінцыя Квебек"), ("bg", "Квебек"), ("bn", "ক\u{9c1}ইবেক"), ("bs", "Quebec"), ("ca", "Quebec"), ("ccp", "𑄇\u{1112d}\u{1112a}𑄝𑄬𑄇\u{11134}"), ("ceb", "Québec (lalawigan)"), ("cs", "Quebec"), ("cy", "Québec"), ("da", "Québec"), ("de", "Québec"), ("el", "Κεμπέκ"), ("en", "Quebec"), ("es", "Quebec"), ("et", "Québec"), ("eu", "Quebec"), ("fa", "استان کبک"), ("fi", "Quebec"), ("fr", "Québec"), ("ga", "Québec"), ("gl", "Quebec"), ("gu", "ક\u{acd}વિબ\u{ac7}ક"), ("ha", "Kebek"), ("ha_NE", "Kebek"), ("he", "קוויבק"), ("hi", "क\u{94d}य\u{942}ब\u{947}क"), ("hr", "Québec"), ("hu", "Québec"), ("hy", "Քվեբեք"), ("id", "Quebec"), ("is", "Québec"), ("it", "Québec"), ("ja", "ケベック州"), ("jv", "Quebec"), ("ka", "კვებეკი"), ("kk", "Квебек"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ಬ\u{cc6}ಕ\u{ccd}"), ("ko", "퀘벡 주"), ("lo", "ແຂວງເກແບ\u{eb1}ກ"), ("lt", "Kvebekas"), ("lv", "Kvebeka"), ("mk", "Квебек"), ("ml", "ക\u{d4d}യ\u{d42}ബെക\u{d4d}"), ("mn", "Кэбэк"), ("mr", "क\u{94d}व\u{947}ब\u{947}क"), ("ms", "Quebec"), ("my", "က\u{103d}\u{102e}ဗက\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Québec"), ("ne", "क\u{94d}य\u{941}ब\u{947}क"), ("nl", "Québec"), ("no", "Québec"), ("pa", "ਕ\u{a47}ਬ\u{a48}ਕ"), ("pl", "Quebec"), ("pt", "Quebec"), ("ro", "Provincia Québec"), ("ru", "Квебек"), ("si", "ක\u{dd2}ය\u{dd4}බෙක\u{dca}"), ("sk", "Quebec"), ("sl", "Québec"), ("so", "Quebec"), ("sq", "Quebec"), ("sr", "Квебек"), ("sr_Latn", "Kvebek"), ("sv", "Québec"), ("sw", "Quebec"), ("ta", "கியூபெக\u{bcd}"), ("te", "క\u{c4d}యూబ\u{c46}క\u{c4d}"), ("th", "ร\u{e31}ฐคว\u{e34}เบก"), ("tr", "Québec"), ("uk", "Квебек"), ("ur", "کیوبیک"), ("uz", "Kvebek"), ("vi", "Québec"), ("yue", "魁北克"), ("yue_Hans", "魁北克"), ("zh", "魁北克")]),
                        unofficial_name_list: ["Québec"].to_vec(),
                    }
                ),
                (
                    "SK",
                    Subdivision{
                        name: "SK",
                        country_alpha2: Alpha2::CA,
                        code: "SK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.9399159), longitude: Some(-106.4508639), max_latitude: Some(60.000063), min_latitude: Some(48.9988059), max_longitude: Some(-101.362305), min_longitude: Some(-110.0077549)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Saskatchewan"), ("am", "ሰስካቸወን"), ("ar", "ساسكاتشوان"), ("az", "Saskaçevan"), ("be", "Правінцыя Саскачэван"), ("bg", "Саскачеван"), ("bn", "স\u{9be}স\u{9cd}ক\u{9be}চ\u{9c1}য\u{9bc}\u{9be}ন"), ("bs", "Saskatchewan"), ("ca", "Saskatchewan"), ("ccp", "𑄥𑄌\u{11134}𑄇𑄖\u{11134}𑄥𑄬𑄠𑄚\u{11134}"), ("ceb", "Saskatchewan"), ("cs", "Saskatchewan"), ("cy", "Saskatchewan"), ("da", "Saskatchewan"), ("de", "Saskatchewan"), ("el", "Σασκάτσουαν"), ("en", "Saskatchewan"), ("es", "Saskatchewan"), ("et", "Saskatchewani provints"), ("eu", "Saskatchewan"), ("fa", "سسکچوان"), ("fi", "Saskatchewan"), ("fr", "Saskatchewan"), ("ga", "Saskatchewan"), ("gl", "Saskatchewan"), ("gu", "સાસ\u{acd}કાટચ\u{ac7}વન"), ("he", "ססקצ׳ואן"), ("hi", "स\u{948}स\u{94d}क\u{948}च\u{947}व\u{947}न"), ("hr", "Saskatchewan"), ("hu", "Saskatchewan"), ("hy", "Սասկաչևան"), ("id", "Saskatchewan"), ("is", "Saskatchewan"), ("it", "Saskatchewan"), ("ja", "サスカチュワン州"), ("ka", "სასკაჩევანი"), ("kn", "ಸಾಸ\u{ccd}ಕಾಚ\u{cc6}ವನ\u{ccd}"), ("ko", "서스캐처원 주"), ("lt", "Saskačevanas"), ("lv", "Saskačevana"), ("mk", "Саскачеван"), ("mn", "Саскачеван"), ("mr", "सास\u{94d}काच\u{947}वान"), ("ms", "Saskatchewan"), ("my", "ဆက\u{103a}စကက\u{103a}ချ\u{102e}ဝမ\u{103a}နယ\u{103a}"), ("nb", "Saskatchewan"), ("nl", "Saskatchewan"), ("no", "Saskatchewan"), ("pa", "ਸਸਕਾਚਵਾਨ"), ("pl", "Saskatchewan"), ("pt", "Saskatchewan"), ("ro", "Saskatchewan"), ("ru", "Саскачеван"), ("si", "සස\u{dca}කචෙව\u{dcf}න\u{dca}"), ("sk", "Saskatchewan"), ("sl", "Saskatchewan"), ("sq", "Saskatchewan"), ("sr", "Саскачеван"), ("sr_Latn", "Saskačevan"), ("sv", "Saskatchewan"), ("sw", "Saskatchewan"), ("ta", "சஸ\u{bcd}க\u{bbe}ச\u{bcd}சுவ\u{bbe}ன\u{bcd}"), ("te", "స\u{c3e}\u{c4d}స\u{c4d}క\u{c3e}చ\u{c3f}వ\u{c3e}న\u{c4d}"), ("th", "ร\u{e31}ฐซ\u{e31}สแคตเชว\u{e31}น"), ("tr", "Saskatchewan"), ("uk", "Саскачеван"), ("ur", "ساسکچیوان"), ("uz", "Saskachevan"), ("vi", "Saskatchewan"), ("yue", "沙士加芝灣"), ("yue_Hans", "沙士加芝湾"), ("zh", "薩斯喀徹溫")]),
                        unofficial_name_list: ["Saskatchewan"].to_vec(),
                    }
                ),
                (
                    "YT",
                    Subdivision{
                        name: "YT",
                        country_alpha2: Alpha2::CA,
                        code: "YT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(64.2823274), longitude: Some(-135.0), max_latitude: Some(69.646498), min_latitude: Some(59.996889), max_longitude: Some(-123.8009179), min_longitude: Some(-141.00187)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Territory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yukon"), ("am", "ዩካን"), ("ar", "يوكون"), ("az", "Yukon"), ("be", "Тэрыторыя Юкан"), ("bg", "Юкон"), ("bn", "ইউকোন"), ("bs", "Yukon"), ("ca", "Yukon"), ("ccp", "𑄃\u{11128}𑄠𑄇\u{11127}𑄚\u{11134}"), ("ceb", "Yukon (lalawigan)"), ("cs", "Yukon"), ("cy", "Yukon"), ("da", "Yukon"), ("de", "Yukon"), ("el", "Γιούκον"), ("en", "Yukon"), ("es", "Yukón"), ("et", "Yukon"), ("eu", "Yukon"), ("fa", "یوکان"), ("fi", "Yukon"), ("fr", "Yukon"), ("ga", "Yukon"), ("gl", "Yukón"), ("gu", "ય\u{ac1}કોન"), ("he", "יוקון"), ("hi", "य\u{941}कॉन प\u{94d}रा\u{902}त"), ("hr", "Yukon"), ("hu", "Yukon"), ("hy", "Յուկոն"), ("id", "Yukon"), ("is", "Yukon"), ("it", "Yukon"), ("ja", "ユーコン準州"), ("ka", "იუკონი"), ("kn", "ಯುಕಾನ\u{ccd}"), ("ko", "유콘 준주"), ("lt", "Jukonas"), ("lv", "Jukona"), ("mk", "Јукон"), ("mn", "Юкон"), ("mr", "य\u{941}कॉन"), ("ms", "Yukon"), ("my", "ယ\u{1030}းက\u{103d}န\u{103a}းနယ\u{103a}"), ("nb", "Yukon"), ("nl", "Yukon"), ("no", "Yukon"), ("pa", "ਯ\u{a42}ਕ\u{a4b}ਨ"), ("pl", "Jukon"), ("pt", "Yukon"), ("ro", "Yukon"), ("ru", "Юкон"), ("si", "ය\u{dd4}න\u{dca}කොන\u{dca}"), ("sk", "Yukon"), ("sl", "Jukon"), ("sq", "Yukon"), ("sr", "Јукон"), ("sr_Latn", "Jukon"), ("sv", "Yukon"), ("sw", "Yukon"), ("ta", "யூக\u{bcd}க\u{bbe}ன\u{bcd}"), ("te", "యుక\u{c3e}న\u{c4d}"), ("th", "ย\u{e39}คอน"), ("tr", "Yukon"), ("uk", "Юкон"), ("ur", "يوکون"), ("uz", "Yukon"), ("vi", "Yukon"), ("yue", "育空地區"), ("yue_Hans", "育空地区"), ("zh", "育空")]),
                        unofficial_name_list: ["Yukon Territory"].to_vec(),
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
#[cfg(feature = "ca")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CA,
        alpha3: Alpha3::CAN,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: "CAD",
        gec: Some(GEC::CA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::CAN),
        iso_long_name: "Canada",
        iso_short_name: "Canada",
        official_language_list: ["en", "fr"].to_vec(),
        spoken_language_list: ["en", "fr"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Canadian"),
        number: "124",
        postal_code: true,
        postal_code_format: Some(
            "[ABCEGHJKLMNPRSTVXY]\\d[ABCEGHJ-NPRSTV-Z] ?\\d[ABCEGHJ-NPRSTV-Z]\\d",
        ),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::NorthernAmerica),
        un_locode: "CA",
        unofficial_name_list: ["Canada", "Kanada", "Canadá", "カナダ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Canada"),
            ("af", "Kanada"),
            ("ak", "Canada"),
            ("am", "Canada"),
            ("an", "Canadá"),
            ("ar", "كندا"),
            ("as", "ক\u{9be}ন\u{9be}ড\u{9be}"),
            ("ay", "Canada"),
            ("az", "Kanada"),
            ("ba", "Canada"),
            ("be", "Канада"),
            ("bg", "Канада"),
            ("bi", "Canada"),
            ("bn", "ক\u{9be}ন\u{9be}ড\u{9be}"),
            ("bn_IN", "ক\u{9be}ন\u{9be}ড\u{9be}"),
            ("br", "Kanada"),
            ("bs", "Kanada"),
            ("ca", "Canadà"),
            ("ce", "Канада"),
            ("ch", "Canada"),
            ("cs", "Kanada"),
            ("cv", "Канада"),
            ("cy", "Canada"),
            ("da", "Canada"),
            ("de", "Kanada"),
            ("dv", "ކ\u{7ac}ނ\u{7ac}ޑ\u{7a7}"),
            ("dz", "ཀ\u{f7a}་ན་ཌ།"),
            ("ee", "Canada"),
            ("el", "Καναδάς"),
            ("en", "Canada"),
            ("eo", "Kanado"),
            ("es", "Canadá"),
            ("et", "Kanada"),
            ("eu", "Kanada"),
            ("fa", "کانادا"),
            ("ff", "Canada"),
            ("fi", "Kanada"),
            ("fo", "Kanada"),
            ("fr", "Canada"),
            ("fy", "Kanada"),
            ("ga", "Ceanada"),
            ("gl", "Canadá"),
            ("gn", "Canada"),
            ("gu", "ક\u{ac7}ન\u{ac7}ડા"),
            ("gv", "Yn Chanadey"),
            ("ha", "Kanada"),
            ("he", "קנדה"),
            ("hi", "कनाडा"),
            ("hr", "Kanada"),
            ("ht", "Kanada"),
            ("hu", "Kanada"),
            ("hy", "Կանադա"),
            ("ia", "Canada"),
            ("id", "Kanada"),
            ("io", "Kanada"),
            ("is", "Kanada"),
            ("it", "Canada"),
            ("iu", "ᑲᓇᑕ"),
            ("ja", "カナダ"),
            ("ka", "კანადა"),
            ("ki", "Canada"),
            ("kk", "Канада"),
            ("kl", "Canada"),
            ("km", "កាណាដា"),
            ("kn", "ಕ\u{cc6}ನಡ"),
            ("ko", "캐나다"),
            ("ku", "Kanada"),
            ("kv", "Канада"),
            ("kw", "Kanada"),
            ("ky", "Канада"),
            ("lo", "ປະເທດການາດາ"),
            ("lt", "Kanada"),
            ("lv", "Kanāda"),
            ("mi", "Kānata"),
            ("mk", "Канада"),
            ("ml", "ക\u{d3e}നഡ"),
            ("mn", "Канад"),
            ("mr", "क\u{945}नडा"),
            ("ms", "Kanada"),
            ("mt", "Kanada"),
            ("my", "ကနေဒါန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Kanada"),
            ("nb", "Canada"),
            ("ne", "क\u{94d}यानाडा"),
            ("nl", "Canada"),
            ("nn", "Canada"),
            ("nv", "Deeteel Bikéyah"),
            ("oc", "Canadà"),
            ("or", "କ\u{b3e}ନ\u{b3e}ଡ\u{b3e}"),
            ("pa", "ਕ\u{a48}ਨ\u{a47}ਡਾ"),
            ("pi", "क\u{947}नडा"),
            ("pl", "Kanada"),
            ("ps", "کاناډا"),
            ("pt", "Canadá"),
            ("pt_BR", "Canadá"),
            ("ro", "Canada"),
            ("ru", "Канада"),
            ("rw", "Kanada"),
            ("sc", "Cànada"),
            ("sd", "ڪئناڊا"),
            ("si", "කැනඩ\u{dcf}ව"),
            ("sk", "Kanada"),
            ("sl", "Kanada"),
            ("so", "Kanada"),
            ("sq", "Kanada"),
            ("sr", "Канада"),
            ("sv", "Kanada"),
            ("sw", "Kanada"),
            ("ta", "கனட\u{bbe}"),
            ("te", "క\u{c46}నడ\u{c3e}"),
            ("tg", "Канада"),
            ("th", "แคนาดา"),
            ("ti", "ካናዳ"),
            ("tk", "Kanada"),
            ("tl", "Kanada"),
            ("tr", "Kanada"),
            ("tt", "Канада"),
            ("ug", "كانادا"),
            ("uk", "Канада"),
            ("ur", "کینیڈا"),
            ("uz", "Kanada"),
            ("ve", "Canada"),
            ("vi", "Ca-na-đa"),
            ("wa", "Canada"),
            ("wo", "Kanadaa"),
            ("xh", "Canada"),
            ("yo", "Kánádà"),
            ("zh_CN", "加拿大"),
            ("zh_HK", "加拿大"),
            ("zh_TW", "加拿大"),
            ("zu", "I Khanada"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
