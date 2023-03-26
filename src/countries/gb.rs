// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The United Kingdom of Great Britain and Northern Ireland

#[cfg(all(feature = "gb", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::GB;
    pub const ALPHA3: Alpha3 = Alpha3::GBR;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 44;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::GBP;
    pub const GEC: Option<GEC> = Some(GEC::UK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::GBR);
    pub const ISO_SHORT_NAME: &str = "United Kingdom of Great Britain and Northern Ireland";
    pub const ISO_LONG_NAME: &str = "The United Kingdom of Great Britain and Northern Ireland";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10, 11];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("British");
    pub const NUMBER: &str = "826";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("GIR ?0AA|(?:(?:AB|AL|B|BA|BB|BD|BF|BH|BL|BN|BR|BS|BT|BX|CA|CB|CF|CH|CM|CO|CR|CT|CV|CW|DA|DD|DE|DG|DH|DL|DN|DT|DY|E|EC|EH|EN|EX|FK|FY|G|GL|GY|GU|HA|HD|HG|HP|HR|HS|HU|HX|IG|IM|IP|IV|JE|KA|KT|KW|KY|L|LA|LD|LE|LL|LN|LS|LU|M|ME|MK|ML|N|NE|NG|NN|NP|NR|NW|OL|OX|PA|PE|PH|PL|PO|PR|RG|RH|RM|S|SA|SE|SG|SK|SL|SM|SN|SO|SP|SR|SS|ST|SW|SY|TA|TD|TF|TN|TQ|TR|TS|TW|UB|W|WA|WC|WD|WF|WN|WR|WS|WV|YO|ZE)(?:\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}))|BFPO ?\\d{1,4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "GB";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "United Kingdom",
        "The United Kingdom",
        "England",
        "Großbritannien",
        "Vereinigtes Königreich",
        "Royaume-Uni",
        "Reino Unido",
        "イギリス",
        "Verenigd Koninkrijk",
        "Great Britain (UK)",
        "UK",
        "Великобритания",
        "Velká Británie",
        "İngiltere",
        "Великобританія",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "United Kingdom"),
        ("af", "Verenidge Koninkryk"),
        ("ak", "United Kingdom"),
        ("am", "ጕንጔሑፔ"),
        ("an", "United Kingdom"),
        ("ar", "المملكة المتحدة"),
        ("as", "য\u{9c1}ক\u{9cd}তৰ\u{9be}জ\u{9cd}য"),
        ("ay", "United Kingdom"),
        ("az", "United Kingdom"),
        ("ba", "United Kingdom"),
        ("be", "Злучанае Каралеўства"),
        ("bg", "Обединено кралство"),
        ("bi", "United Kingdom"),
        ("bn", "য\u{9c1}ক\u{9cd}তর\u{9be}জ\u{9cd}য"),
        ("bn_IN", "য\u{9c1}ক\u{9cd}তর\u{9be}জ\u{9cd}য"),
        ("br", "Rouantelezh-Unanet"),
        ("bs", "Velika Britanija"),
        ("ca", "Regne Unit"),
        ("ce", "Йоккха Британи"),
        ("ch", "United Kingdom"),
        ("cs", "Spojené království"),
        ("cv", "Йоккха Британи"),
        ("cy", "Y Deyrnas Unedig"),
        ("da", "Storbritannien"),
        ("de", "Vereinigtes Königreich"),
        (
            "dv",
            "ޔ\u{7aa}ނ\u{7a6}އ\u{7a8}ޓ\u{7ac}ޑ\u{7b0} ކ\u{7a8}ނ\u{7b0}ގ\u{7b0}ޑ\u{7a6}މ\u{7b0}",
        ),
        ("dz", "ཡ\u{f74}་ནའ\u{f72}་ཊ\u{f7a}ཊ་ ཀ\u{f72}ང་ཌ\u{f7c}མ།"),
        ("ee", "United Kingdom"),
        ("el", "Ηνωμένο Βασίλειο"),
        ("en", "United Kingdom"),
        ("eo", "Britio"),
        ("es", "Reino Unido"),
        ("et", "Suurbritannia"),
        ("eu", "Erresuma Batua"),
        ("fa", "انگلستان"),
        ("ff", "Biritaani-Mawndi"),
        ("fi", "Yhdistynyt kuningaskunta"),
        ("fo", "United Kingdom"),
        ("fr", "Royaume-Uni"),
        ("fy", "Grut-Brittanje"),
        ("ga", "An Ríocht Aontaithe"),
        ("gl", "Reino Unido"),
        ("gn", "United Kingdom"),
        ("gu", "ય\u{ac1}નાઇટ\u{ac7}ડ કિ\u{a82}ગડમ"),
        ("gv", "Reeriaght Unnaneysit"),
        ("ha", "Birtaniya"),
        ("he", "הממלכה המאוחדת"),
        ("hi", "य\u{942}नाइट\u{947}ड कि\u{902}गडम"),
        ("hr", "Ujedinjeno Kraljevstvo"),
        ("ht", "Wayòm Ini"),
        ("hu", "Egyesült Királyság"),
        ("hy", "Մեծ Բրիտանիա"),
        ("ia", "Regno Unite"),
        ("id", "Britania Raya"),
        ("io", "Unionita Rejio"),
        ("is", "Bretland"),
        ("it", "Regno Unito"),
        ("iu", "ᑐᓗᐃᑦ ᓄᓈᑦ"),
        ("ja", "英国"),
        ("ka", "გაერთიანებული სამეფო"),
        ("ki", "United Kingdom"),
        ("kk", "Ұлыбритания"),
        ("kl", "United Kingdom"),
        ("km", "ចក\u{17d2}រភព\u{200b}អង\u{17cb}គ\u{17d2}លេស"),
        ("kn", "ಯುನೈಟ\u{cc6}ಡ\u{ccd} ಕ\u{cbf}ಂಗ\u{ccd}ಡಮ\u{ccd}"),
        ("ko", "영국"),
        ("ku", "Şahitiya Yekbuyî"),
        ("kv", "Ыджыд Британия"),
        ("kw", "Ruwvaneth Unys"),
        ("ky", "Улуу Британия"),
        ("lo", "ສະຫະລາຊະອານາຈ\u{eb1}ກ"),
        ("lt", "Jungtinė Karalystė"),
        ("lv", "Lielbritānija"),
        ("mi", "Kīngitanga Kotahi"),
        ("mk", "Велика Британија"),
        ("ml", "യ\u{d41}ണൈറ\u{d4d}റഡ\u{d4d} കിങ\u{d4d}ഡം"),
        ("mn", "Нэгдсэн Вант Улс"),
        ("mr", "य\u{941}नायट\u{947}ड कि\u{902}गडम"),
        ("ms", "United Kingdom"),
        ("mt", "Ingilterra"),
        (
            "my",
            "ယ\u{1030}န\u{102d}\u{102f}က\u{103a}တက\u{103a} ကင\u{103a}းဒမ\u{103a}း",
        ),
        ("na", "Ingerand"),
        ("nb", "Storbritannia"),
        ("ne", "स\u{902}य\u{941}क\u{94d}त अधिराज\u{94d}य"),
        ("nl", "Verenigd Koninkrijk"),
        ("nn", "Storbritannia"),
        ("nv", "Tó Táʼ Dinéʼiʼ Bikéyah"),
        ("oc", "Reialme Unit"),
        ("or", "ଯ\u{b41}କ\u{b4d}ତ ର\u{b3e}ଜ\u{b4d}ଯ"),
        ("pa", "ਬਰਤਾਨੀਆ"),
        ("pi", "United Kingdom"),
        ("pl", "Wielka Brytania"),
        ("ps", "برتانیه"),
        ("pt", "Reino Unido"),
        ("pt_BR", "Reino Unido"),
        ("ro", "Regatul Unit"),
        ("ru", "Соединённое Королевство"),
        ("rw", "Ubwongereza (UK)"),
        ("sc", "Rennu Unidu"),
        ("sd", "United Kingdom"),
        ("si", "එක\u{dca}සත\u{dca} ර\u{dcf}ජධ\u{dcf}න\u{dd2}ය"),
        ("sk", "Spojené kráľovstvo"),
        ("sl", "Združeno kraljestvo"),
        ("so", "Midowga boqortooyada Britan"),
        ("sq", "Mbretëria e Bashkuar"),
        ("sr", "Уједињено Краљевство"),
        ("sv", "Storbritannien"),
        ("sw", "United Kingdom"),
        ("ta", "ஐக\u{bcd}கிய இர\u{bbe}ஜ\u{bcd}ஜியம\u{bcd}"),
        (
            "te",
            "యున\u{c48}ట\u{c46}డ\u{c4d} క\u{c3f}ంగ\u{c4d}\u{200c}డమ\u{c4d}",
        ),
        ("tg", "Шоҳигарии Муттаҳида"),
        ("th", "สหราชอาณาจ\u{e31}กร"),
        ("ti", "ብሪጣንያ"),
        ("tk", "Birleşen Patyşalyk"),
        ("tl", "Nagkaisang Kaharian"),
        ("tr", "Birleşik Krallık"),
        ("tt", "Берләшкән Падшаһлык"),
        ("ug", "ئەنگلىيە"),
        ("uk", "Велика Британія"),
        ("ur", "برطانیہ"),
        ("uz", "Birlashgan Qirollik"),
        ("ve", "United Kingdom"),
        ("vi", "Vương Quốc Anh Thống Nhất"),
        ("wa", "Rweyåme Uni"),
        ("wo", "Ruwaayom bu Bennoo"),
        ("xh", "United Kingdom"),
        ("yo", "Ilẹ\u{300}ọba Aṣọ\u{300}kan"),
        ("zh_CN", "英国"),
        ("zh_HK", "英國"),
        ("zh_TW", "英國"),
        ("zu", "Umbuso Ohlangeneyo"),
    ];
    #[cfg(all(feature = "gb", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 55.378051;
        pub const LONGITUDE: f64 = -3.435973;
        pub const MAX_LATITUDE: f64 = 60.91569999999999;
        pub const MAX_LONGITUDE: f64 = 1.68153079591;
        pub const MIN_LATITUDE: f64 = 49.959999905;
        pub const MIN_LONGITUDE: f64 = -7.57216793459;
        pub const NORTHEAST_LATITUDE: f64 = 60.91569999999999;
        pub const NORTHEAST_LONGITUDE: f64 = 1.68153079591;
        pub const SOUTHWEST_LATITUDE: f64 = 49.959999905;
        pub const SOUTHWEST_LONGITUDE: f64 = -7.57216793459;
    }
}
#[cfg(all(feature = "gb", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 55.378051,
            longitude: -3.435973,
            max_latitude: 60.91569999999999,
            max_longitude: 1.68153079591,
            min_latitude: 49.959999905,
            min_longitude: -7.57216793459,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 60.91569999999999,
                    longitude: 1.68153079591,
                },
                southwest: CountryGeoBound {
                    latitude: 49.959999905,
                    longitude: -7.57216793459,
                },
            },
        }
    }
}

#[cfg(all(feature = "gb", feature = "subdivisions"))]
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
                    "ABC",
                    Subdivision{
                        name: "Armagh City, Banbridge and Craigavon",
                        country_alpha2: Alpha2::GB,
                        code: "ABC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "আর\u{9cd}ম\u{9be}"), ("ccp", "𑄃𑄢\u{11134}𑄟𑄇\u{11134}, 𑄝𑄚\u{11134}𑄝\u{11133}𑄢\u{11128}𑄌\u{11134} 𑄃\u{11133}𑄃 𑄇\u{11133}𑄢\u{1112d}𑄉𑄞\u{11127}𑄚\u{11134}"), ("de", "Armagh, Banbridge and Craigavon"), ("en", "Armagh, Banbridge and Craigavon"), ("es", "Armagh"), ("fa", "آرما، بنبریج و کریگ\u{200c}آوون"), ("fr", "Armagh, Banbridge and Craigavon"), ("gu", "અર\u{acd}માઘ"), ("it", "Distretto di Armagh, Banbridge e Craigavon"), ("ja", "アーマー・シティ・バンブリッジ・アンド・クレイガヴォン"), ("kn", "ಅರ\u{ccd}ಮಗ\u{ccd}ಹ\u{ccd}"), ("ko", "아마 구 밴브리지와 크레이가본"), ("lt", "Benbridžas"), ("nl", "Armagh, Banbridge and Craigavon"), ("pt", "Armagh"), ("ru", "Арма, Бенбридж и Крейгавон"), ("ta", "அர\u{bcd}ம\u{bbe}க\u{bcd}"), ("te", "ఆర\u{c4d}మ\u{c3e}ఘ\u{c4d}"), ("ur", "آرماہ، بینبریج اور کرائگیون")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ABD",
                    Subdivision{
                        name: "Aberdeenshire",
                        country_alpha2: Alpha2::GB,
                        code: "ABD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.16214290000001), longitude: Some(-2.7194167), max_latitude: Some(57.7012045), min_latitude: Some(56.7471851), max_longitude: Some(-1.7644527), min_longitude: Some(-3.801648199999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبردينشاير"), ("az", "Aberdinşir"), ("be", "Абердзіншыр"), ("bg", "Абърдийншър"), ("bn", "স\u{9cd}কটল\u{9cd}য\u{9be}ন\u{9cd}ড-"), ("ca", "Aberdeenshire"), ("ccp", "𑄃𑄝𑄬𑄢\u{11134}𑄓\u{11128}𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Aberdeenshire"), ("cs", "Aberdeenshire"), ("cy", "Swydd Aberdeen"), ("da", "Aberdeenshire"), ("de", "Aberdeenshire"), ("en", "Aberdeenshire"), ("es", "Aberdeenshire"), ("et", "Aberdeenshire"), ("eu", "Aberdeenshire"), ("fa", "ابردین\u{200c}شیر"), ("fi", "Aberdeenshire"), ("fr", "Aberdeenshire"), ("ga", "Contae Obar Deathain"), ("gu", "એબરડિનશાયર"), ("hi", "एबर\u{94d}डीनशायर"), ("hu", "Aberdeenshire"), ("hy", "Աբերդինշիր"), ("it", "Aberdeenshire"), ("ja", "アバディーンシャー"), ("kn", "ಆಬರ\u{ccd}ಡೀನ\u{ccd}ಸ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "애버딘셔"), ("lt", "Aberdynšyras"), ("nb", "Aberdeenshire"), ("nl", "Aberdeenshire"), ("no", "Aberdeenshire"), ("pl", "Aberdeenshire"), ("pt", "Aberdeenshire"), ("ro", "Aberdeenshire"), ("ru", "Абердиншир"), ("sv", "Aberdeenshire"), ("ta", "அபெர\u{bcd}ட\u{bc0}ன\u{bcd}ஷிர\u{bcd}"), ("te", "అబ\u{c46}ర\u{c4d}డ\u{c40}న\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Абердиншир"), ("ur", "ابیردینشائر"), ("yo", "Aberdeenshire"), ("yo_BJ", "Aberdeenshire"), ("zh", "阿伯丁郡")]),
                        unofficial_name_list: ["Siorrachd Obar Dheathain"].to_vec(),
                    }
                ),
                (
                    "ABE",
                    Subdivision{
                        name: "Aberdeen City",
                        country_alpha2: Alpha2::GB,
                        code: "ABE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.149717), longitude: Some(-2.094278), max_latitude: Some(57.19565069999999), min_latitude: Some(57.1041518), max_longitude: Some(-2.0461811), min_longitude: Some(-2.2058926)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Aberdeen"), ("ar", "أبردين"), ("az", "Aberdin"), ("be", "Абердзін"), ("bg", "Абърдийн"), ("bn", "অ\u{9cd}য\u{9be}বরদিন"), ("bs", "Aberdeen"), ("ca", "Aberdeen"), ("ccp", "𑄃𑄝𑄬𑄢\u{11134}𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Aberdeen"), ("cs", "Aberdeen"), ("cy", "Aberdeen"), ("da", "Aberdeen"), ("de", "Aberdeen"), ("el", "Αμπερντήν"), ("en", "Aberdeen"), ("es", "Aberdeen"), ("et", "Aberdeen"), ("eu", "Aberdeen"), ("fa", "ابردین"), ("fi", "Aberdeen"), ("fr", "Aberdeen"), ("ga", "Obar Dheathain"), ("gl", "Aberdeen"), ("gu", "એબરડીન"), ("he", "אברדין"), ("hi", "एबरडीन"), ("hr", "Aberdeen"), ("hu", "Aberdeen"), ("hy", "Աբերդին"), ("id", "Aberdeen"), ("is", "Aberdeen"), ("it", "Aberdeen"), ("ja", "アバディーン"), ("ka", "აბერდინი"), ("kk", "Абердин"), ("kn", "ಅಬರ\u{ccd}ದೀನ\u{ccd}"), ("ko", "애버딘"), ("lt", "Aberdynas"), ("lv", "Aberdīna"), ("mk", "Абердин"), ("ml", "അബർഡീൻ"), ("mn", "Абердин"), ("mr", "अ\u{200d}\u{945}बर\u{94d}डीन"), ("ms", "Aberdeen"), ("my", "အဘာဒင\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Aberdeen"), ("nl", "Aberdeen"), ("no", "Aberdeen"), ("pl", "Aberdeen"), ("pt", "Aberdeen"), ("ro", "Aberdeen"), ("ru", "Абердин"), ("si", "ඇබර\u{dca}ඩ\u{dd3}න\u{dca}"), ("sk", "Aberdeen"), ("sl", "Aberdeen"), ("sq", "Aberdeen"), ("sr", "Абердин"), ("sr_Latn", "Aberdin"), ("sv", "Aberdeen"), ("sw", "Aberdeen"), ("ta", "அபெர\u{bcd}டின\u{bcd}"), ("te", "అబ\u{c46}ర\u{c4d}డ\u{c40}న\u{c4d}"), ("th", "แอเบอร\u{e4c}ด\u{e35}น"), ("tr", "Aberdeen"), ("uk", "Абердин"), ("ur", "ابرڈین"), ("uz", "Aberdin"), ("vi", "Aberdeen"), ("yo", "Aberdeen"), ("yo_BJ", "Aberdeen"), ("yue", "鴨巴甸"), ("yue_Hans", "鸭巴甸"), ("zh", "阿伯丁")]),
                        unofficial_name_list: ["Aberdeen City"].to_vec(),
                    }
                ),
                (
                    "AGB",
                    Subdivision{
                        name: "Argyll and Bute",
                        country_alpha2: Alpha2::GB,
                        code: "AGB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.37004630000001), longitude: Some(-5.0318965), max_latitude: Some(56.7048488), min_latitude: Some(55.274488), max_longitude: Some(-4.5598516), min_longitude: Some(-7.112457900000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Arqayl-end-Büt"), ("bg", "Аргил анд Бют"), ("bn", "আর\u{9cd}জেল ও ব\u{9c1}ট"), ("ca", "Consell d’Argyll and Bute"), ("ccp", "𑄃𑄉\u{11133}𑄢\u{11128}𑄣\u{11134} 𑄃\u{11133}𑄃 𑄝\u{1112a}𑄑𑄬"), ("ceb", "Argyll and Bute"), ("cs", "Argyll a Bute"), ("cy", "Argyll a Bute"), ("de", "Argyll and Bute"), ("en", "Argyll and Bute"), ("es", "Argyll and Bute"), ("et", "Argyll and Bute"), ("eu", "Argyll eta Bute"), ("fa", "آرگایل و بوت"), ("fi", "Argyll ja Bute"), ("fr", "Argyll and Bute"), ("ga", "Earra-Ghaidheal agus Bód"), ("gl", "Argyll and Bute"), ("gu", "આર\u{acd}ગીલ અન\u{ac7} બ\u{ac2}ટ"), ("he", "ארגייל וביוט"), ("hr", "Argyll i Bute"), ("id", "Argyll and Bute"), ("it", "Argyll e Bute"), ("ja", "アーガイル・アンド・ビュート"), ("kk", "Аргайл-энд-Бьют"), ("kn", "ಆರ\u{ccd}ಗೈಲ\u{ccd} ಮತ\u{ccd}ತು ಬ\u{ccd}ಯುಟ\u{cc6}"), ("ko", "아가일 뷰트"), ("lt", "Argailas ir Bjutas"), ("nb", "Argyll and Bute"), ("nl", "Argyll and Bute"), ("no", "Argyll and Bute"), ("pl", "Argyll and Bute"), ("pt", "Argyll and Bute"), ("ro", "Argyll and Bute"), ("ru", "Аргайл-энд-Бьют"), ("sv", "Argyll and Bute"), ("ta", "அகில\u{bcd} & பூட\u{bcd}"), ("te", "ఆర\u{c4d}గ\u{c3f}ల\u{c4d} అండ\u{c4d} బుట\u{c4d}"), ("uk", "Аргілл-і-Бʼют"), ("zh", "阿盖尔-比特")]),
                        unofficial_name_list: ["Argyll and Bute"].to_vec(),
                    }
                ),
                (
                    "AGY",
                    Subdivision{
                        name: "Isle of Anglesey [Sir Ynys Môn GB-YNM]",
                        country_alpha2: Alpha2::GB,
                        code: "AGY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.265325), longitude: Some(-4.429140299999999), max_latitude: Some(53.4300953), min_latitude: Some(53.12660229999999), max_longitude: Some(-4.0401791), min_longitude: Some(-4.5885415)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Anglesey"), ("ar", "أنغلسي"), ("be", "Англсі"), ("bg", "Ангълси"), ("ca", "Anglesey"), ("ccp", "𑄃\u{11101}𑄣𑄬𑄥\u{11128}"), ("ceb", "Anglesey"), ("cs", "Anglesey"), ("cy", "Ynys Môn"), ("da", "Anglesey"), ("de", "Anglesey"), ("el", "Άνγκλεσι"), ("en", "Anglesey"), ("es", "Anglesey"), ("et", "Anglesey"), ("eu", "Anglesey"), ("fa", "انگلسی"), ("fi", "Anglesey"), ("fr", "Anglesey"), ("ga", "Ynys Môn"), ("gl", "Anglesey - Ynys Môn"), ("he", "אנגלסי"), ("hi", "ए\u{902}ग\u{94d}ल\u{947}सी"), ("hu", "Anglesey"), ("hy", "Անգլսի"), ("id", "Anglesey"), ("is", "Öngulsey"), ("it", "Anglesey"), ("ja", "アングルシー島"), ("ko", "앵글시 섬"), ("lt", "Anglesis"), ("ms", "Anglesey"), ("nb", "Anglesey"), ("nl", "Anglesey"), ("no", "Anglesey"), ("pl", "Anglesey"), ("pt", "Anglesey"), ("ro", "Anglesey"), ("ru", "Англси"), ("sk", "Anglesey"), ("sl", "Anglesey"), ("sr", "Англси"), ("sr_Latn", "Anglsi"), ("sv", "Anglesey"), ("th", "แองเก\u{e34}ลซ\u{e35}ย\u{e4c}"), ("tr", "Anglesey"), ("uk", "Анґлсі"), ("vi", "Anglesey"), ("yue", "安格爾西島"), ("yue_Hans", "安格尔西岛"), ("zh", "安格尔西岛")]),
                        unofficial_name_list: ["Ynys MÃ´n"].to_vec(),
                    }
                ),
                (
                    "AND",
                    Subdivision{
                        name: "Ards and North Down",
                        country_alpha2: Alpha2::GB,
                        code: "AND",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "আর\u{9cd}ডস"), ("ccp", "𑄃𑄢\u{11134}𑄓\u{11134}𑄥\u{11134} 𑄃\u{11133}𑄃 𑄚\u{11127}𑄢\u{11134}𑄖\u{11134} 𑄓𑄅\u{1112a}𑄚\u{11134}"), ("de", "Ards and North Down"), ("en", "Ards and North Down"), ("es", "Ards and North Down"), ("fa", "آردس و نورث داون"), ("fr", "Ards and North Down"), ("gu", "આર\u{acd}ડ\u{acd}સ અન\u{ac7} ઉત\u{acd}તર ડાઉન"), ("it", "Distretto di Ards e North Down"), ("ja", "アーズ・アンド・ノース・ダウン"), ("kn", "ಆರ\u{ccd}ಡ\u{ccd}ಸ\u{ccd} ಮತ\u{ccd}ತು ನಾರ\u{ccd}ತ\u{ccd} ಡ\u{ccc}ನ\u{ccd}"), ("ko", "아르즈"), ("lt", "Ardsas"), ("nl", "Ards and North Down"), ("pt", "Ards and North Down"), ("ru", "Ардс и Норф-Даун"), ("ta", "ஆர\u{bcd}ட\u{bcd}ஸ\u{bcd} அண\u{bcd}ட\u{bcd} வடக\u{bcd}கு டௌன\u{bcd}"), ("te", "ఆర\u{c4d}డ\u{c4d}స\u{c4d} అండ\u{c4d} న\u{c3e}ర\u{c4d}త\u{c4d} డ\u{c4c}న\u{c4d}"), ("ur", "آرڈز اور نارتھ ڈاؤن")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ANN",
                    Subdivision{
                        name: "Antrim and Newtownabbey",
                        country_alpha2: Alpha2::GB,
                        code: "ANN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄚\u{11134}𑄑\u{11133}𑄢\u{11128}𑄟\u{11134} 𑄃\u{11133}𑄃 𑄚\u{11128}𑄅\u{1112a}𑄑\u{1112d}𑄅\u{1112a}𑄚\u{11134}𑄝𑄬"), ("de", "Antrim and Newtownabbey"), ("en", "Antrim and Newtownabbey"), ("fa", "انتریم و نیوتاون\u{200c}ابی"), ("fr", "Antrim and Newtownabbey"), ("it", "Distretto di Antrim e Newtownabbey"), ("ja", "アントリム・アンド・ニュータウンアベイ"), ("nl", "Antrim and Newtownabbey"), ("ur", "انٹریم اور نیو ٹاؤن ایبی"), ("zh", "安特里姆與紐頓阿比區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ANS",
                    Subdivision{
                        name: "Angus",
                        country_alpha2: Alpha2::GB,
                        code: "ANS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.7969965), longitude: Some(-2.9206818), max_latitude: Some(56.9868164), min_latitude: Some(56.4639635), max_longitude: Some(-2.4265296), min_longitude: Some(-3.4070219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أنغوس"), ("be", "Вобласць Ангус"), ("bg", "Ангъс"), ("bn", "অ\u{9cd}য\u{9be}ঙ\u{9cd}গ\u{9be}স"), ("ca", "Angus"), ("ccp", "𑄃\u{11101}𑄉\u{1112a}𑄌\u{11134}"), ("ceb", "Angus"), ("cs", "Angus"), ("cy", "Angus"), ("da", "Angus"), ("de", "Angus"), ("el", "Άνγκους"), ("en", "Angus"), ("es", "Angus"), ("et", "Angus"), ("eu", "Angus"), ("fa", "آنگوس"), ("fi", "Angus"), ("fr", "Angus"), ("ga", "Aonghas"), ("gl", "Angus"), ("gu", "એ\u{a82}ગસ"), ("he", "אנגוס"), ("hy", "Անգուս"), ("it", "Angus"), ("ja", "アンガス"), ("ka", "ანგუსი"), ("kn", "ಆಂಗಸ\u{ccd}"), ("ko", "앵거스"), ("lt", "Angusas"), ("mk", "Ангус"), ("nb", "Angus"), ("nl", "Angus"), ("no", "Angus"), ("pl", "Angus"), ("pt", "Angus"), ("ro", "Angus"), ("ru", "Ангус"), ("sv", "Angus"), ("ta", "அங\u{bcd}குஸ\u{bcd}"), ("te", "అంగూస\u{c4d}"), ("tr", "Angus"), ("uk", "Ангус"), ("ur", "انجوس"), ("yue", "安格斯"), ("yue_Hans", "安格斯"), ("zh", "安格斯")]),
                        unofficial_name_list: ["Angus"].to_vec(),
                    }
                ),
                (
                    "BAS",
                    Subdivision{
                        name: "Bath and North East Somerset",
                        country_alpha2: Alpha2::GB,
                        code: "BAS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.36362930000001), longitude: Some(-2.4399987), max_latitude: Some(51.4395359), min_latitude: Some(51.2731011), max_longitude: Some(-2.2785436), min_longitude: Some(-2.7059546)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Бат і Паўночна-Усходні Сомерсет"), ("bg", "Бат и Североизточен Съмърсет (община)"), ("bn", "ব\u{9be}থ ও উত\u{9cd}তর প\u{9c2}র\u{9cd}ব সম\u{9be}রসেট"), ("ccp", "𑄝𑄖\u{11134} 𑄃\u{11133}𑄃 𑄅\u{11127}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{1112a}𑄇\u{11134} 𑄥\u{11127}𑄟𑄢\u{11134}𑄥𑄬𑄖\u{11134}"), ("ceb", "Bath and North East Somerset"), ("de", "Bath and North East Somerset"), ("en", "Bath and North East Somerset"), ("es", "Bath and North East Somerset"), ("fa", "باث و سامرست شمال\u{200c}شرقی"), ("fr", "Bath and North East Somerset"), ("gu", "બાથ અન\u{ac7} ઉત\u{acd}તર પ\u{ac2}ર\u{acd}વ સોમરસ\u{ac7}ટ"), ("it", "Bath and North East Somerset"), ("ja", "バース・アンド・ノース・イースト・サマセット"), ("kn", "ಬಾತ\u{ccd} ಮತ\u{ccd}ತು ಈಶಾನ\u{ccd}ಯ ಸೊಮರ\u{ccd}ಸ\u{cc6}ಟ\u{ccd}"), ("ko", "바스 노스이스트서머싯"), ("mk", "Бат и Североисточен Сомерсет"), ("nb", "Bath and North East Somerset"), ("nl", "Bath and North East Somerset"), ("no", "Bath and North East Somerset"), ("pl", "Bath and North East Somerset"), ("pt", "Bath and North East Somerset"), ("ro", "Bath and North East Somerset"), ("ru", "Бат и Северо-Восточный Сомерсет"), ("sk", "Bath and North East Somerset"), ("sv", "Bath and North East Somerset"), ("ta", "ப\u{bbe}த\u{bcd} அண\u{bcd}ட\u{bcd} வட கிழக\u{bcd}கு சொமேர\u{bcd}செட\u{bcd}"), ("te", "బ\u{c3e}త\u{c4d} మర\u{c3f}యు తూర\u{c4d}పు స\u{c4b}మర\u{c4d} స\u{c46}ట\u{c4d}"), ("uk", "Бат і Північно-Східний Сомерсет"), ("ur", "باتھ اور شمال مشرقی سامرسیٹ"), ("zh", "巴斯和東北薩默塞特")]),
                        unofficial_name_list: ["Bath and North East Somerset"].to_vec(),
                    }
                ),
                (
                    "BBD",
                    Subdivision{
                        name: "Blackburn with Darwen",
                        country_alpha2: Alpha2::GB,
                        code: "BBD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.68985960000001), longitude: Some(-2.4678625), max_latitude: Some(53.7818044), min_latitude: Some(53.6165679), max_longitude: Some(-2.3626429), min_longitude: Some(-2.5647704)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "ব\u{9cd}ল\u{9cd}য\u{9be}কব\u{9be}র\u{9cd}ন-ও-ড\u{9be}রওয\u{9bc}েন"), ("ccp", "𑄇\u{11133}𑄣\u{11133}𑄠𑄇\u{11134}𑄝𑄢\u{11134}𑄚\u{11134} 𑄃\u{1112a}𑄃\u{11128}𑄖\u{11134} 𑄓𑄢\u{11134}𑄃\u{1112a}𑄃\u{11128}𑄚\u{11134}"), ("ceb", "Blackburn with Darwen"), ("de", "Blackburn with Darwen"), ("en", "Blackburn with Darwen"), ("es", "Blackburn with Darwen"), ("et", "Blackburn with Darwen"), ("fr", "Blackburn with Darwen"), ("gu", "બ\u{acd}લ\u{ac7}કબર\u{acd}ન વિથ ડાર\u{acd}વ\u{ac7}ન"), ("it", "Blackburn with Darwen"), ("ja", "ブラックバーン・ウィズ・ダーウェン"), ("kn", "ಡಾರ\u{ccd}ವ\u{cc6}ನ\u{ccd}ನೊಂದ\u{cbf}ಗ\u{cc6} ಬ\u{ccd}ಲ\u{ccd}ಯಾಕ\u{ccd}ಬರ\u{ccd}ನ\u{ccd}"), ("ko", "블랙번위드다언"), ("nb", "Blackburn with Darwen"), ("nl", "Blackburn with Darwen"), ("no", "Blackburn with Darwen"), ("pl", "Blackburn with Darwen"), ("pt", "Blackburn with Darwen"), ("ro", "Blackburn with Darwen"), ("ru", "Блэкберн и Дарвен"), ("sk", "Blackburn with Darwen"), ("sv", "Blackburn with Darwen"), ("ta", "ப\u{bcd}ள\u{bbe}க\u{bcd}கபூரின\u{bcd} வித\u{bcd} ட\u{bbe}ரவென\u{bcd}"), ("te", "బ\u{c4d}ల\u{c3e}క\u{c4d} బర\u{c4d}న\u{c4d} వ\u{c3f}త\u{c4d} డ\u{c3e}ర\u{c4d}వ\u{c3f}న\u{c4d}"), ("tr", "Blackburn ile Darwen"), ("uk", "Блекберн і Дарвен"), ("ur", "بلیکبرن مع ڈاروین"), ("zh", "布莱克本-達文")]),
                        unofficial_name_list: ["Blackburn with Darwen"].to_vec(),
                    }
                ),
                (
                    "BCP",
                    Subdivision{
                        name: "Bournemouth, Christchurch and Poole",
                        country_alpha2: Alpha2::GB,
                        code: "BCP",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Bournemouth, Christchurch and Poole")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BDF",
                    Subdivision{
                        name: "Bedfordshire",
                        country_alpha2: Alpha2::GB,
                        code: "BDF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.1359735), longitude: Some(-0.4666565), max_latitude: Some(52.3229384), min_latitude: Some(52.0545323), max_longitude: Some(-0.2407452), min_longitude: Some(-0.6687744)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄝𑄬𑄖\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Bedford (kondado)"), ("da", "Borough of Bedford"), ("de", "Borough of Bedford"), ("en", "Bedford"), ("es", "Bedford"), ("fa", "بدفورد بورو"), ("fr", "Bedford"), ("it", "Bedford"), ("ja", "ベッドフォード (バラ)"), ("ko", "베드퍼드 구"), ("nb", "Bedford (distrikt)"), ("nl", "Bedford"), ("no", "Bedford (distrikt)"), ("pl", "Bedford (borough)"), ("ru", "Бедфорд"), ("sv", "Bedford"), ("uk", "Бедфорд (район)"), ("ur", "بورو بیڈفورڈ"), ("zh", "貝德福德區")]),
                        unofficial_name_list: ["Bedfordshire"].to_vec(),
                    }
                ),
                (
                    "BDG",
                    Subdivision{
                        name: "Barking and Dagenham",
                        country_alpha2: Alpha2::GB,
                        code: "BDG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5464828), longitude: Some(0.1293497), max_latitude: Some(51.59943639999999), min_latitude: Some(51.51138), max_longitude: Some(0.1901898), min_longitude: Some(0.0666489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Barkinq və Dagenhem borosu"), ("be", "бора Баркінг і Дагенем"), ("bn", "ব\u{9be}র\u{9cd}কিং ও ডেগেনহ\u{9cd}য\u{9be}ম"), ("ca", "Barking i Dagenham"), ("ccp", "𑄝𑄢\u{11134}𑄇\u{11128}\u{11101} 𑄃\u{11133}𑄃 𑄓\u{11127}𑄉𑄬𑄚\u{11134}𑄦𑄟\u{11134}"), ("ceb", "Barking and Dagenham"), ("cs", "Barking a Dagenham"), ("cy", "Barking a Dagenham"), ("da", "Barking and Dagenham"), ("de", "London Borough of Barking and Dagenham"), ("en", "Barking and Dagenham"), ("es", "Barking y Dagenham"), ("eu", "Barking eta Dagenham"), ("fa", "منطقه بارکینگ و دگنهام لندن"), ("fi", "Barking and Dagenham"), ("fr", "district londonien de Barking et Dagenham"), ("ga", "Buirg Londan Barking agus Dagenham"), ("gu", "લ\u{a82}ડન બોરો ઓફ બાર\u{acd}કિ\u{a82}ગ અન\u{ac7} ડગ\u{ac7}નહામ"), ("he", "בארקינג ודאגנהאם"), ("hi", "बार\u{94d}कि\u{902}ग ऐ\u{902}ड ड\u{947}गनहम बरो"), ("hu", "Barking and Dagenham kerület"), ("id", "Barking dan Dagenham"), ("is", "Barking og Dagenham"), ("it", "Barking e Dagenham"), ("ja", "バーキング・アンド・ダゲナム・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಬಾರ\u{ccd}ಕ\u{cbf}ಂಗ\u{ccd} ಮತ\u{ccd}ತು ಡೇಗ\u{cc6}ನ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}"), ("ko", "바킹 대거넘 구"), ("nb", "Barking and Dagenham"), ("nl", "Barking en Dagenham"), ("no", "Barking and Dagenham"), ("pl", "London Borough of Barking and Dagenham"), ("pt", "Barking e Dagenham"), ("ro", "Barking and Dagenham"), ("ru", "Баркинг и Дагенхэм"), ("sk", "Barking and Dagenham"), ("sr", "Лондонска општина Баркинг и Дагенам"), ("sr_Latn", "Londonska opština Barking i Dagenam"), ("sv", "London Borough of Barking and Dagenham"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} ப\u{bbe}ர\u{bcd}க\u{bcd}கிங\u{bcd} & டகென\u{bcd}ஹ\u{bbe}ம\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} బ\u{c3e}ర\u{c4d}క\u{c3f}ంగ\u{c4d} అండ\u{c4d} డ\u{c3e}గ\u{c46}ంహం"), ("tr", "Barking ve Dagenham"), ("uk", "Баркінг і Дагенем"), ("ur", "بارکنگ اور ڈیگنہیم بورو"), ("vi", "Khu Barking và Dagenham của Luân Đôn"), ("zh", "巴金-達格納姆區")]),
                        unofficial_name_list: ["Barking and Dagenham"].to_vec(),
                    }
                ),
                (
                    "BEN",
                    Subdivision{
                        name: "Brent",
                        country_alpha2: Alpha2::GB,
                        code: "BEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5672808), longitude: Some(-0.2710568), max_latitude: Some(51.60037029999999), min_latitude: Some(51.527654), max_longitude: Some(-0.1914835), min_longitude: Some(-0.3355844)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Brent borosu"), ("be", "Брэнт"), ("bn", "বেন\u{9cd}ট"), ("ca", "Brent"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Brent (distrito)"), ("cs", "Brent"), ("cy", "Brent"), ("da", "Brent"), ("de", "London Borough of Brent"), ("en", "Brent"), ("es", "Brent"), ("et", "Brenti linnaosa"), ("eu", "Brent"), ("fa", "منطقه برنت لندن"), ("fi", "Brent"), ("fr", "district londonien de Brent"), ("ga", "Buirg Londan Brent"), ("gu", "લ\u{a82}ડન બોરો ઓફ બ\u{acd}ર\u{ac7}ન\u{acd}ટ"), ("he", "ברנט"), ("hi", "ब\u{94d}र\u{947}\u{902}ट बरो"), ("hu", "Brent kerület"), ("is", "Brent"), ("it", "Borgo londinese di Brent"), ("ja", "ブレント・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಬ\u{ccd}ರ\u{cc6}ಂಟ\u{ccd}"), ("ko", "브렌트 구"), ("mk", "Брент"), ("nb", "Brent"), ("nl", "Brent"), ("no", "Brent"), ("pl", "London Borough of Brent"), ("pt", "Brent"), ("ro", "Brent"), ("ru", "Брент"), ("sl", "London Borough of Brent"), ("sr", "Лондонска општина Брент"), ("sr_Latn", "Londonska opština Brent"), ("sv", "London Borough of Brent"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} ப\u{bcd}ரெண\u{bcd}ட\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} బ\u{c4d}ర\u{c46}ంట\u{c4d}"), ("tr", "Brent, Londra"), ("uk", "Брент"), ("ur", "برینٹ بورو"), ("vi", "Khu Brent của Luân Đôn"), ("zh", "布倫特區")]),
                        unofficial_name_list: ["Brent"].to_vec(),
                    }
                ),
                (
                    "BEX",
                    Subdivision{
                        name: "Bexley",
                        country_alpha2: Alpha2::GB,
                        code: "BEX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.439933), longitude: Some(0.154327), max_latitude: Some(51.4493819), min_latitude: Some(51.4233213), max_longitude: Some(0.1774081), min_longitude: Some(0.1139954)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Beksli borosu"), ("be", "Бекслі"), ("ca", "Bexley"), ("ccp", "𑄝𑄬𑄇\u{11133}𑄥\u{11134}𑄣𑄬"), ("ceb", "Bexley (distrito)"), ("cs", "Bexley"), ("cy", "Bexley"), ("da", "Bexley"), ("de", "London Borough of Bexley"), ("en", "Bexley"), ("es", "Municipio de Bexley"), ("et", "Bexley linnaosa"), ("eu", "Bexley"), ("fa", "منطقه بکسلی لندن"), ("fi", "Bexley"), ("fr", "district londonien de Bexley"), ("ga", "London Borough of Bexley"), ("he", "בקסלי"), ("hi", "ब\u{947}क\u{94d}सली बरो"), ("hu", "Bexley kerület"), ("id", "Bexley"), ("is", "Bexley"), ("it", "Bexley"), ("ja", "ベクスリー・ロンドン特別区"), ("ka", "ბექსლი"), ("ko", "벡슬리 구"), ("nb", "Bexley"), ("nl", "Bexley"), ("no", "Bexley"), ("pl", "London Borough of Bexley"), ("pt", "Bexley"), ("ro", "Bexley"), ("ru", "Бексли"), ("sl", "Bexley, London"), ("sr", "Лондонска општина Бексли"), ("sr_Latn", "Londonska opština Beksli"), ("sv", "London Borough of Bexley"), ("tr", "Bexley"), ("uk", "Бекслі"), ("ur", "بیکزلی بورو"), ("vi", "Khu Bexley của Luân Đôn"), ("zh", "貝克斯利區")]),
                        unofficial_name_list: ["Bexley"].to_vec(),
                    }
                ),
                (
                    "BFS",
                    Subdivision{
                        name: "Belfast",
                        country_alpha2: Alpha2::GB,
                        code: "BFS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.59728500000001), longitude: Some(-5.93012), max_latitude: Some(54.65920999999999), min_latitude: Some(54.53055), max_longitude: Some(-5.808004599999999), min_longitude: Some(-6.0452601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "акруга Белфаст"), ("bg", "Белфаст"), ("ca", "Districte de Belfast"), ("ccp", "𑄝𑄬𑄣\u{11134}𑄜𑄌\u{11133}𑄑\u{11134}"), ("en", "Belfast"), ("fr", "district de Belfast"), ("hy", "Բելֆաստ"), ("ja", "ベルファスト区"), ("ko", "벨파스트 구"), ("ru", "Белфаст"), ("uk", "Белфаст"), ("zh", "貝爾法斯特區")]),
                        unofficial_name_list: ["Belfast"].to_vec(),
                    }
                ),
                (
                    "BGE",
                    Subdivision{
                        name: "Bridgend [Pen-y-bont ar Ogwr GB-POG]",
                        country_alpha2: Alpha2::GB,
                        code: "BGE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.504286), longitude: Some(-3.576945), max_latitude: Some(51.5336115), min_latitude: Some(51.4821997), max_longitude: Some(-3.5250057), min_longitude: Some(-3.6716169)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Брыджэнд"), ("bg", "Бридженд"), ("bn", "ব\u{9cd}রিজেন\u{9cd}ড ক\u{9be}উন\u{9cd}টি বরো"), ("ca", "Bridgend"), ("ccp", "𑄝\u{11133}𑄢\u{11128}𑄖\u{11134}𑄎𑄬𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Bridgend county borough"), ("cy", "Bwrdeistref Sirol Pen-y-bont ar Ogwr"), ("de", "Bridgend"), ("en", "Bridgend"), ("es", "Bridgend County Borough"), ("et", "Bridgend"), ("eu", "Bridgend"), ("fa", "بریج\u{200c}اند"), ("fi", "Bridgend"), ("fr", "Bridgend"), ("ga", "Pen-y-bont ar Ogwr"), ("gl", "Pen-y-bont ar Ogwr"), ("gu", "બ\u{acd}રિજ\u{ac7}ન\u{acd}ડ કાઉન\u{acd}ટી બોરો"), ("it", "distretto di contea di Bridgend"), ("ja", "ブリジェンド"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಡ\u{ccd}ಜ\u{cc6}ಂಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "브리젠드 자치시"), ("lt", "Bridžendas"), ("nb", "Bridgend"), ("nl", "Bridgend"), ("no", "Bridgend"), ("pl", "Bridgend"), ("pt", "Bridgend County Borough"), ("ro", "Bridgend"), ("ru", "Бридженд"), ("sk", "Bridgend"), ("sv", "Bridgend"), ("ta", "பிரிட\u{bcd}ஜென\u{bcd}ட\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3f}డ\u{c4d}జ\u{c46}ండ\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("uk", "Брідженд"), ("zh", "布里真德郡級自治市")]),
                        unofficial_name_list: ["Pen-y-bont ar Ogwr"].to_vec(),
                    }
                ),
                (
                    "BGW",
                    Subdivision{
                        name: "Blaenau Gwent",
                        country_alpha2: Alpha2::GB,
                        code: "BGW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.7875779), longitude: Some(-3.2043931), max_latitude: Some(51.8254832), min_latitude: Some(51.6812847), max_longitude: Some(-3.1060076), min_longitude: Some(-3.3100856)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بليناو غونت"), ("bg", "Блайнай Гуент"), ("bn", "ব\u{9cd}লেইনউ গ\u{9cd}যেন\u{9cd}ট ক\u{9be}উন\u{9cd}টি বরো"), ("ca", "Blaenau Gwent"), ("ccp", "𑄝\u{11133}𑄣𑄬𑄚𑄅\u{1112a} 𑄉𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Blaenau Gwent"), ("cy", "Blaenau Gwent"), ("de", "Blaenau Gwent"), ("en", "Blaenau Gwent"), ("es", "Blaenau Gwent"), ("et", "Blaenau Gwent"), ("eu", "Blaenau Gwent"), ("fa", "بلاینای گونت"), ("fi", "Blaenau Gwent"), ("fr", "Blaenau Gwent"), ("ga", "Blaenau Gwent"), ("gu", "બ\u{acd}લ\u{ac7}નો ગ\u{acd}વ\u{ac7}\u{a82}ટ કાઉન\u{acd}ટી બોરો"), ("it", "distretto di contea di Blaenau Gwent"), ("ja", "ブライナイ・グエント"), ("kn", "ಬ\u{ccd}ಲೇನ\u{ccc} ಗ\u{ccd}ವ\u{cc6}ಂಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "블라이나이궨트"), ("lt", "Blainai Gventas"), ("nb", "Blaenau Gwent"), ("nl", "Blaenau Gwent"), ("no", "Blaenau Gwent"), ("pl", "Blaenau Gwent"), ("pt", "Blaenau Gwent"), ("ro", "Blaenau Gwent"), ("ru", "Блайнай-Гвент"), ("sv", "Blaenau Gwent"), ("ta", "பிளேனு குவென\u{bcd}ட\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "బ\u{c4d}ల\u{c46}న\u{c3e}వు గ\u{c4d}వ\u{c46}ంట\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("uk", "Бланау-Гвент"), ("zh", "布莱耐格温特")]),
                        unofficial_name_list: ["Blaenau Gwent"].to_vec(),
                    }
                ),
                (
                    "BIR",
                    Subdivision{
                        name: "Birmingham",
                        country_alpha2: Alpha2::GB,
                        code: "BIR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.48624299999999), longitude: Some(-1.890401), max_latitude: Some(52.5688762), min_latitude: Some(52.385999), max_longitude: Some(-1.7098294), min_longitude: Some(-2.0174336)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Birmingham"), ("am", "በርሚንግሃም"), ("ar", "برمينغهام"), ("az", "Birmingem"), ("be", "Бірмінгем"), ("bg", "Бирмингам"), ("bn", "ব\u{9be}র\u{9cd}মিংহ\u{9be}ম"), ("ca", "Birmingham"), ("ccp", "𑄝𑄢\u{11134}𑄟\u{11128}\u{11101}𑄦𑄟\u{11134}"), ("ceb", "City and Borough of Birmingham"), ("cs", "Birmingham"), ("da", "Birmingham"), ("de", "Birmingham"), ("el", "Μπέρμιγχαμ"), ("en", "Birmingham"), ("es", "Birmingham"), ("et", "Birmingham"), ("eu", "Birmingham"), ("fa", "بیرمنگام"), ("fi", "Birmingham"), ("fr", "Birmingham"), ("gl", "Birmingham"), ("gu", "બર\u{acd}મિ\u{a82}ગહામ"), ("he", "ברמינגהאם"), ("hi", "बर\u{94d}मि\u{902}घम"), ("hr", "Birmingham"), ("hu", "Birmingham"), ("hy", "Բիրմինգհեմ"), ("id", "Birmingham"), ("is", "Birmingham"), ("it", "Birmingham"), ("ja", "バーミンガム"), ("ka", "ბირმინგემი"), ("kn", "ಬರ\u{ccd}ಮ\u{cbf}ಂಗ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}"), ("ko", "버밍엄"), ("lt", "Birmingamas"), ("lv", "Birmingema"), ("ml", "ബിർമിങ\u{d4d}ഹ\u{d3e}ം"), ("mn", "Бирмингем"), ("mr", "बर\u{94d}मि\u{902}गह\u{945}म"), ("ms", "Birmingham"), ("nb", "Birmingham"), ("nl", "Birmingham"), ("no", "Birmingham"), ("pl", "Birmingham"), ("pt", "Birmingham"), ("ro", "Birmingham"), ("ru", "Бирмингем"), ("si", "බර\u{dca}ම\u{dd2}න\u{dca}ග\u{dca}හැම\u{dca}"), ("sk", "Birmingham"), ("sl", "Birmingham"), ("sr", "Бирмингем"), ("sr_Latn", "Birmingem"), ("sv", "Birmingham"), ("sw", "Birmingham"), ("ta", "பர\u{bcd}மிங\u{bcd}க\u{bbe}ம\u{bcd}"), ("te", "బర\u{c4d}మ\u{c3f}ంగ\u{c4d}\u{200c}హ\u{c3e}మ\u{c4d}"), ("th", "เบอร\u{e4c}ม\u{e34}งแฮม"), ("tr", "Birmingham"), ("uk", "Бірмінгем"), ("ur", "برمنگھم"), ("vi", "Birmingham"), ("zh", "伯明翰"), ("zu", "Birmingham")]),
                        unofficial_name_list: ["Birmingham"].to_vec(),
                    }
                ),
                (
                    "BKM",
                    Subdivision{
                        name: "Buckinghamshire",
                        country_alpha2: Alpha2::GB,
                        code: "BKM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8072204), longitude: Some(-0.8127664), max_latitude: Some(52.0815218), min_latitude: Some(51.4854818), max_longitude: Some(-0.4766167999999999), min_longitude: Some(-1.1406962)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Buckinghamshire"), ("ar", "باكينغهامشير"), ("be", "Бакінгемшыр"), ("bg", "Бъкингамшър"), ("bn", "ব\u{9be}কিংহ\u{9be}মশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Buckinghamshire"), ("ccp", "𑄝𑄇\u{11128}\u{11101}𑄦𑄟\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Buckinghamshire"), ("cs", "Buckinghamshire"), ("cy", "Swydd Buckingham"), ("da", "Buckinghamshire"), ("de", "Buckinghamshire"), ("el", "Μπάκιγχαμ"), ("en", "Buckinghamshire"), ("es", "Buckinghamshire"), ("et", "Buckinghamshire"), ("eu", "Buckinghamshire"), ("fa", "باکینگهام\u{200c}شایر"), ("fi", "Buckinghamshire"), ("fr", "Buckinghamshire"), ("ga", "Buckinghamshire"), ("gl", "Buckinghamshire"), ("gu", "બકિ\u{a82}ગહામશાયર"), ("he", "בקינגהאמשייר"), ("hi", "बकि\u{902}घमशायर"), ("hu", "Buckinghamshire"), ("hy", "Բաքինհեմշիր"), ("id", "Buckinghamshire"), ("is", "Buckinghamshire"), ("it", "Buckinghamshire"), ("ja", "バッキンガムシャー"), ("ka", "ბაკინგემშირი"), ("kn", "ಬಕ\u{cbf}ಂಗ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "버킹엄셔 주"), ("lt", "Bakingamšyras"), ("lv", "Bekingemšīra"), ("mk", "Бакингемшир"), ("ml", "ബക\u{d4d}കിങ\u{d4d}ങ\u{d3e}ംഷയർ"), ("mr", "बकि\u{902}गह\u{945}मशायर"), ("nb", "Buckinghamshire"), ("nl", "Buckinghamshire"), ("no", "Buckinghamshire"), ("pl", "Buckinghamshire"), ("pt", "Buckinghamshire"), ("ro", "Buckinghamshire"), ("ru", "Бакингемшир"), ("sk", "Buckinghamshire"), ("sr", "Бакингемшир"), ("sr_Latn", "Bakingemšir"), ("sv", "Buckinghamshire"), ("ta", "புக\u{bcd}கிங\u{bcd}ஹம\u{bcd}ஷிர\u{bcd}"), ("te", "బక\u{c3f}ంగ\u{c4d}హంష\u{c48}ర\u{c4d}"), ("th", "บ\u{e31}กก\u{e34}งแฮมเชอร\u{e4c}"), ("tr", "Buckinghamshire"), ("uk", "Бакінгемшир"), ("ur", "بکنگھمشائر"), ("vi", "Buckinghamshire"), ("yue", "白金漢郡"), ("yue_Hans", "白金汉郡"), ("zh", "白金汉郡")]),
                        unofficial_name_list: ["Buckinghamshire"].to_vec(),
                    }
                ),
                (
                    "BNE",
                    Subdivision{
                        name: "Barnet",
                        country_alpha2: Alpha2::GB,
                        code: "BNE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.6569225), longitude: Some(-0.1949252), max_latitude: Some(51.67997459999999), min_latitude: Some(51.63570430000001), max_longitude: Some(-0.1198855), min_longitude: Some(-0.2562643)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Barnet-distrik"), ("az", "Barnet borosu"), ("be", "Барнет"), ("bg", "Барнет"), ("bn", "ব\u{9be}রনেট"), ("ca", "Barnet"), ("ccp", "𑄝𑄢\u{11134}𑄚𑄬𑄖\u{11134}"), ("ceb", "Barnet (distrito)"), ("cs", "Barnet"), ("cy", "Barnet"), ("da", "Barnet"), ("de", "London Borough of Barnet"), ("el", "Λονδρέζικο Προάστιο του Μπαρνέτ"), ("en", "Barnet"), ("es", "Municipio de Barnet"), ("eu", "Barnet"), ("fa", "منطقه بارنت لندن"), ("fi", "Barnet"), ("fr", "district londonien de Barnet"), ("ga", "Buirg Londan Barnet"), ("gu", "લ\u{a82}ડન બોરો ઓફ બાર\u{acd}ન\u{ac7}ટ"), ("he", "בארנט"), ("hi", "बार\u{94d}न\u{947}ट बरो"), ("hu", "Barnet kerület"), ("hy", "Բարնեթ"), ("id", "Barnet"), ("is", "Barnet"), ("it", "Barnet"), ("ja", "バーネット・ロンドン特別区"), ("kn", "ಬಾರ\u{ccd}ನ\u{cc6}ಟ\u{ccd}ನ ಲಂಡನ\u{ccd} ಬರೋ"), ("ko", "바닛 구"), ("mk", "Барнет"), ("nb", "Barnet"), ("nl", "Barnet"), ("no", "Barnet"), ("pl", "London Borough of Barnet"), ("pt", "Barnet"), ("ro", "Barnet"), ("ru", "Барнет"), ("sl", "London Borough of Barnet"), ("sr", "Лондонска општина Барнет"), ("sr_Latn", "Londonska opština Barnet"), ("sv", "London Borough of Barnet"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ப\u{bbe}ர\u{bcd}னெட\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b} ఆఫ\u{c4d} బ\u{c3e}ర\u{c4d}న\u{c46}ట\u{c4d}"), ("tr", "Barnet"), ("uk", "Барнет"), ("ur", "بارنیٹ بورو"), ("vi", "Khu Barnet của Luân Đôn"), ("yue", "班列特"), ("yue_Hans", "班列特"), ("zh", "巴尼特區")]),
                        unofficial_name_list: ["Barnet"].to_vec(),
                    }
                ),
                (
                    "BNH",
                    Subdivision{
                        name: "Brighton and Hove",
                        country_alpha2: Alpha2::GB,
                        code: "BNH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.83516050000001), longitude: Some(-0.1261028), max_latitude: Some(50.892374), min_latitude: Some(50.7991466), max_longitude: Some(-0.0160306), min_longitude: Some(-0.2450771)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "برايتون أند هوف"), ("be", "Брайтан і Хоўв"), ("bn", "ব\u{9cd}র\u{9be}ইটন ও হোভ"), ("ccp", "𑄝\u{11133}𑄢\u{1112d}𑄑\u{11127}𑄚\u{11134} 𑄃\u{11133}𑄃 𑄦\u{1112e}𑄛\u{11134}"), ("ceb", "Brighton and Hove"), ("cy", "Brighton a Hove"), ("da", "Brighton and Hove"), ("de", "Brighton and Hove"), ("el", "Μπράιτον εντ Χόουβ"), ("en", "Brighton and Hove"), ("es", "Brighton & Hove"), ("eu", "Brighton eta Hove"), ("fa", "برایتون اند هوو"), ("fi", "Brighton & Hove"), ("fr", "Brighton et Hove"), ("ga", "Brighton & Hove"), ("gu", "બ\u{acd}રાઇટન અન\u{ac7} હોવ"), ("hy", "Բրայթոն և Հոուֆ"), ("id", "Brighton & Hove"), ("it", "Brighton & Hove"), ("ja", "ブライトン・アンド・ホヴ"), ("kn", "ಬ\u{ccd}ರೈಟನ\u{ccd} ಮತ\u{ccd}ತು ಹೋವ\u{ccd}"), ("ko", "브라이턴앤드호브"), ("mk", "Брајтон и Хоув"), ("nb", "Brighton and Hove"), ("nl", "Brighton and Hove"), ("no", "Brighton and Hove"), ("pl", "Brighton and Hove"), ("pt", "Brighton e Hove"), ("ro", "Brighton & Hove"), ("ru", "Брайтон и Хоув"), ("sk", "Brighton and Hove"), ("sr", "Брајтон и Хоув"), ("sr_Latn", "Brajton i Houv"), ("sv", "Brighton & Hove"), ("ta", "பிரைட\u{bcd}டன\u{bcd} அண\u{bcd}ட\u{bcd} ஹொவ\u{bcd}"), ("te", "బ\u{c4d}ర\u{c48}టన\u{c4d} అండ\u{c4d} హ\u{c4b}వ\u{c4d}"), ("th", "ไบรต\u{e31}นและโฮฟ"), ("tr", "Brighton & Hove"), ("uk", "Брайтон і Гоув"), ("ur", "برائیٹن اینڈ ہوو"), ("vi", "Brighton và Hove"), ("zh", "布赖顿-霍夫")]),
                        unofficial_name_list: ["Brighton and Hove"].to_vec(),
                    }
                ),
                (
                    "BNS",
                    Subdivision{
                        name: "Barnsley",
                        country_alpha2: Alpha2::GB,
                        code: "BNS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.55263), longitude: Some(-1.479726), max_latitude: Some(53.6013373), min_latitude: Some(53.5271566), max_longitude: Some(-1.3683433), min_longitude: Some(-1.5282373)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Муніцыпальны раён Барнслі"), ("bg", "Барнзли"), ("ccp", "𑄝𑄢\u{11134}𑄚\u{11134}𑄌\u{11133}𑄣𑄬"), ("ceb", "Barnsley (kondado)"), ("de", "Metropolitan Borough of Barnsley"), ("en", "Barnsley"), ("es", "Municipio metropolitano de Barnsley"), ("fa", "کلان\u{200c}شهر مستقل بارنزلی"), ("fr", "district métropolitain de Barnsley"), ("it", "Barnsley"), ("ja", "メトロポリタン・バラ・オブ・バーンズリー"), ("ko", "반즐리 도시 자치구"), ("nb", "Barnsley"), ("nl", "Barnsley"), ("no", "Barnsley"), ("pl", "Metropolitan Borough of Barnsley"), ("ru", "Муниципальный район Барнсли"), ("sv", "Barnsley"), ("uk", "Барнслі"), ("ur", "میٹروپولیٹن بورو بارنسلے"), ("zh", "巴恩斯利都市自治市")]),
                        unofficial_name_list: ["Barnsley"].to_vec(),
                    }
                ),
                (
                    "BOL",
                    Subdivision{
                        name: "Bolton",
                        country_alpha2: Alpha2::GB,
                        code: "BOL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.57686469999999), longitude: Some(-2.4282192), max_latitude: Some(53.632951), min_latitude: Some(53.5466579), max_longitude: Some(-2.3630087), min_longitude: Some(-2.5456287)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "раён Болтан"), ("bg", "Болтън"), ("ccp", "𑄝\u{1112e}𑄣\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Borough of Bolton"), ("de", "Metropolitan Borough of Bolton"), ("en", "Bolton"), ("es", "Bolton"), ("fa", "کلان\u{200c}شهر مستقل بولتون"), ("fr", "district métropolitain de Bolton"), ("he", "בולטון"), ("it", "Metropolitan Borough of Bolton"), ("ja", "ボルトン"), ("ko", "볼턴 도시 자치구"), ("nb", "Bolton"), ("nl", "Bolton"), ("no", "Bolton"), ("pl", "Metropolitan Borough of Bolton"), ("pt", "Bolton"), ("ru", "Болтон"), ("sv", "Bolton"), ("tr", "Bolton Metropoliten Borough"), ("ur", "میٹروپولیٹن بورو بولٹن"), ("zh", "博爾頓都市自治市")]),
                        unofficial_name_list: ["Bolton"].to_vec(),
                    }
                ),
                (
                    "BPL",
                    Subdivision{
                        name: "Blackpool",
                        country_alpha2: Alpha2::GB,
                        code: "BPL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.8175053), longitude: Some(-3.0356748), max_latitude: Some(53.8782396), min_latitude: Some(53.7725781), max_longitude: Some(-2.9858799), min_longitude: Some(-3.0624611)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Blackpool"), ("ar", "بلاكبول"), ("az", "Bləkpul"), ("be", "Горад Блэкпул"), ("bg", "Блакпул"), ("bn", "ব\u{9cd}ল\u{9cd}য\u{9be}কপ\u{9c1}ল"), ("ca", "Blackpool"), ("ccp", "𑄝\u{11133}𑄣\u{11133}𑄠𑄇\u{11134}𑄛\u{1112a}𑄣\u{11134}"), ("ceb", "Blackpool"), ("cs", "Blackpool"), ("cy", "Blackpool"), ("da", "Blackpool"), ("de", "Blackpool"), ("el", "Μπλάκπουλ"), ("en", "Blackpool"), ("es", "Blackpool"), ("et", "Blackpool"), ("eu", "Blackpool"), ("fa", "بلکپول"), ("fi", "Blackpool"), ("fr", "Blackpool"), ("ga", "Blackpool"), ("gu", "બ\u{acd}લ\u{ac7}કપ\u{ac1}લ"), ("he", "בלקפול"), ("hr", "Blackpool"), ("hu", "Blackpool"), ("hy", "Բլեքփուլ"), ("id", "Blackpool"), ("is", "Blackpool"), ("it", "Blackpool"), ("ja", "ブラックプール"), ("ka", "ბლეკპული"), ("kn", "ಬ\u{ccd}ಲ\u{ccd}ಯಾಕ\u{ccd}ಪ\u{cc2}ಲ\u{ccd}"), ("ko", "블랙풀"), ("ky", "Блэкпул шаары"), ("lt", "Blakpulas"), ("lv", "Blekpūla"), ("mk", "Блекпул"), ("nb", "Blackpool"), ("nl", "Blackpool"), ("no", "Blackpool"), ("pl", "Blackpool"), ("pt", "Blackpool"), ("ro", "Blackpool"), ("ru", "Блэкпул"), ("sk", "Blackpool"), ("sr", "Блекпул"), ("sr_Latn", "Blekpul"), ("sv", "Blackpool"), ("sw", "Blackpool"), ("ta", "ப\u{bcd}ள\u{bbe}க\u{bcd}ப\u{bcd}பூள\u{bcd}"), ("te", "బ\u{c4d}ల\u{c3e}క\u{c4d} పూల\u{c4d}"), ("th", "แบล\u{e47}กพ\u{e39}ล"), ("tr", "Blackpool"), ("uk", "Блекпул"), ("ur", "بلیکپول"), ("uz", "Blekpul"), ("vi", "Blackpool"), ("yue", "布力浦"), ("yue_Hans", "布力浦"), ("zh", "黑潭")]),
                        unofficial_name_list: ["Blackpool"].to_vec(),
                    }
                ),
                (
                    "BRC",
                    Subdivision{
                        name: "Bracknell Forest",
                        country_alpha2: Alpha2::GB,
                        code: "BRC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4076953), longitude: Some(-0.729976), max_latitude: Some(51.4687317), min_latitude: Some(51.3319361), max_longitude: Some(-0.6306976), min_longitude: Some(-0.8373663)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Бракнъл Форест (община)"), ("bn", "ব\u{9cd}র\u{9cd}র\u{9cd}য\u{9be}কনেল ফরেস\u{9cd}ট"), ("ccp", "𑄝\u{11133}𑄢𑄇\u{11134}𑄚𑄬𑄣\u{11134} 𑄜\u{11127}𑄢𑄬𑄥\u{11133}𑄑\u{11134}"), ("ceb", "Bracknell Forest"), ("da", "Bracknell Forest"), ("de", "Bracknell Forest"), ("en", "Bracknell Forest"), ("es", "Bracknell Forest"), ("fa", "برکنل فورست"), ("fr", "Bracknell Forest"), ("gu", "બ\u{acd}ર\u{ac7}કન\u{ac7}લ ફોર\u{ac7}સ\u{acd}ટ"), ("it", "Bracknell Forest"), ("ja", "ブラックネル・フォレスト"), ("kn", "ಬ\u{ccd}ರಾಕ\u{ccd}ನ\u{cc6}ಲ\u{ccd} ಫಾರ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}"), ("ko", "브랙널포리스트"), ("nb", "Bracknell Forest"), ("nl", "Bracknell Forest"), ("no", "Bracknell Forest"), ("pl", "Bracknell Forest"), ("pt", "Bracknell Forest"), ("ro", "Bracknell Forest"), ("ru", "Брэкнелл Форест"), ("sv", "Bracknell Forest"), ("ta", "பிரேக\u{bcd}கனெல\u{bcd}ல போரெஸ\u{bcd}ட\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3e}క\u{c4d} న\u{c46}ల\u{c4d} ఫ\u{c3e}ర\u{c46}స\u{c4d}ట\u{c4d}"), ("uk", "Брекнелл Форест"), ("ur", "براکنیل فارسٹ"), ("zh", "布拉克內爾森林")]),
                        unofficial_name_list: ["Bracknell Forest"].to_vec(),
                    }
                ),
                (
                    "BRD",
                    Subdivision{
                        name: "Bradford",
                        country_alpha2: Alpha2::GB,
                        code: "BRD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.795984), longitude: Some(-1.759398), max_latitude: Some(53.84684919999999), min_latitude: Some(53.7471441), max_longitude: Some(-1.6778126), min_longitude: Some(-1.8309082)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "бора Брэдфард"), ("bg", "Брадфорд"), ("ccp", "𑄝\u{11133}𑄢𑄖\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Bradford"), ("de", "City of Bradford"), ("en", "Bradford"), ("es", "Ciudad de Bradford"), ("fa", "سیتی بردفورد"), ("fr", "cité de Bradford"), ("hy", "Բրադֆորդ"), ("it", "City of Bradford"), ("ja", "シティ・オブ・ブラッドフォード"), ("ko", "시티오브브래드퍼드"), ("nb", "City of Bradford"), ("nl", "City of Bradford"), ("no", "City of Bradford"), ("pl", "City of Bradford"), ("ru", "Брадфорд"), ("sv", "Bradford"), ("ta", "பிர\u{bbe}ட\u{bcd}போர\u{bcd}டு"), ("uk", "Брадфорд"), ("ur", "بریڈفورڈ شہر"), ("zh", "布拉德福德市")]),
                        unofficial_name_list: ["Bradford"].to_vec(),
                    }
                ),
                (
                    "BRY",
                    Subdivision{
                        name: "Bromley",
                        country_alpha2: Alpha2::GB,
                        code: "BRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.406025), longitude: Some(0.013156), max_latitude: Some(51.4356667), min_latitude: Some(51.35400569999999), max_longitude: Some(0.0690479), min_longitude: Some(-0.0149361)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bromley"), ("ar", "منطقة بروملي"), ("be", "Бромлі"), ("bs", "Bromley"), ("ca", "Bromley"), ("ccp", "𑄝\u{11133}𑄢\u{1112e}𑄟\u{11134}𑄣\u{11128}"), ("ceb", "Bromley"), ("cs", "Bromley"), ("cy", "Bromley"), ("da", "Bromley"), ("de", "London Borough of Bromley"), ("el", "Μπρόμλεϊ"), ("en", "Bromley"), ("es", "Municipio de Bromley"), ("et", "Bromley linnaosa"), ("eu", "Bromley"), ("fa", "منطقه بروملی لندن"), ("fi", "Bromley"), ("fr", "Bromley"), ("ga", "Bromley"), ("gl", "Bromley"), ("he", "ברומלי"), ("hi", "ब\u{94d}रॉमली बरो"), ("hr", "Bromley"), ("hu", "Bromley kerület"), ("hy", "Բրոմլի"), ("is", "Bromley"), ("it", "Bromley"), ("ja", "ブロムリー・ロンドン特別区"), ("ka", "ბრომლი"), ("ko", "브롬리 구"), ("lt", "Bromley"), ("mk", "Бромли"), ("nb", "Bromley"), ("nl", "Bromley"), ("no", "Bromley"), ("pl", "London Borough of Bromley"), ("pt", "Bromley"), ("ro", "Bromley"), ("ru", "Бромли"), ("sk", "Bromley"), ("sl", "Bromley"), ("sq", "Bromley"), ("sr", "Лондонска општина Бромли"), ("sr_Latn", "Londonska opština Bromli"), ("sv", "London Borough of Bromley"), ("tr", "Bromley"), ("uk", "Бромлі"), ("ur", "بروملی بورو"), ("vi", "Khu Bromley của Luân Đôn"), ("zh", "布羅姆利區")]),
                        unofficial_name_list: ["Bromley"].to_vec(),
                    }
                ),
                (
                    "BST",
                    Subdivision{
                        name: "Bristol, City of",
                        country_alpha2: Alpha2::GB,
                        code: "BST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.454513), longitude: Some(-2.58791), max_latitude: Some(51.5444326), min_latitude: Some(51.39254529999999), max_longitude: Some(-2.4509024), min_longitude: Some(-2.7305164)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bristol"), ("am", "ብርስተል"), ("ar", "برستل"), ("az", "Bristol"), ("be", "Брысталь"), ("bg", "Бристъл"), ("bn", "ব\u{9cd}রিস\u{9cd}টল"), ("bs", "Bristol"), ("ca", "Bristol"), ("ccp", "𑄝\u{11133}𑄢\u{11128}𑄌\u{11134}𑄑\u{11127}𑄣\u{11134}"), ("ceb", "Bristol"), ("cs", "Bristol"), ("cy", "Bryste"), ("da", "Bristol"), ("de", "Bristol"), ("el", "Μπρίστολ"), ("en", "Bristol"), ("es", "Brístol"), ("et", "Bristol"), ("eu", "Bristol"), ("fa", "بریستول"), ("fi", "Bristol"), ("fr", "Bristol"), ("ga", "Briostó"), ("gl", "Bristol"), ("gu", "બ\u{acd}રિસ\u{acd}ટોલ"), ("ha", "Bristol"), ("ha_NE", "Bristol"), ("he", "בריסטול"), ("hi", "ब\u{94d}रिस\u{94d}टल"), ("hr", "Bristol"), ("hu", "Bristol"), ("hy", "Բրիստոլ"), ("id", "Bristol"), ("is", "Bristol"), ("it", "Bristol"), ("ja", "ブリストル"), ("ka", "ბრისტოლი"), ("kk", "Бристоль"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಸ\u{ccd}ಟಲ\u{ccd}\u{200c}"), ("ko", "브리스틀"), ("lt", "Bristolis"), ("lv", "Bristole"), ("mk", "Бристол"), ("ml", "ബ\u{d4d}രിസ\u{d4d}റ\u{d4d}റൽ"), ("mr", "ब\u{94d}रिस\u{94d}टल"), ("ms", "Bristol"), ("nb", "Bristol"), ("nl", "Bristol"), ("no", "Bristol"), ("pa", "ਬਰਿਸਟਲ"), ("pl", "Bristol"), ("pt", "Bristol"), ("ro", "Bristol"), ("ru", "Бристоль"), ("si", "බ\u{dca}\u{200d}ර\u{dd2}ස\u{dca}ටල\u{dca}"), ("sk", "Bristol"), ("sl", "Bristol"), ("sq", "Bristol"), ("sr", "Бристол"), ("sr_Latn", "Bristol"), ("sv", "Bristol"), ("sw", "Bristol"), ("ta", "பிரிஸ\u{bcd}டல\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3f}స\u{c4d}టల\u{c4d}"), ("th", "บร\u{e34}สตอล"), ("tr", "Bristol"), ("uk", "Бристоль"), ("ur", "برسٹل"), ("uz", "Bristol"), ("vi", "Bristol"), ("yo", "Bristol"), ("yo_BJ", "Bristol"), ("yue", "碧仙桃"), ("yue_Hans", "碧仙桃"), ("zh", "布里斯托尔")]),
                        unofficial_name_list: ["City of Bristol"].to_vec(),
                    }
                ),
                (
                    "BUR",
                    Subdivision{
                        name: "Bury",
                        country_alpha2: Alpha2::GB,
                        code: "BUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5933498), longitude: Some(-2.2966054), max_latitude: Some(53.6447333), min_latitude: Some(53.5497338), max_longitude: Some(-2.2425392), min_longitude: Some(-2.3518793)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Беры"), ("bg", "Бери"), ("ccp", "𑄝𑄢\u{11128}"), ("ceb", "Borough of Bury"), ("de", "Metropolitan Borough of Bury"), ("en", "Bury"), ("es", "Municipio metropolitano de Bury"), ("fa", "کلان\u{200c}شهر مستقل بری"), ("fr", "district métropolitain de Bury"), ("it", "Metropolitan Borough of Bury"), ("ja", "メトロポリタン・バラ・オブ・ベリー"), ("ko", "베리 도시 자치구"), ("nb", "Bury"), ("nl", "Bury"), ("no", "Bury"), ("pl", "Metropolitan Borough of Bury"), ("ru", "Бери"), ("sv", "Bury"), ("uk", "Бері"), ("ur", "میٹروپولیٹن بورو بری"), ("zh", "貝里都市自治市")]),
                        unofficial_name_list: ["Bury"].to_vec(),
                    }
                ),
                (
                    "CAM",
                    Subdivision{
                        name: "Cambridgeshire",
                        country_alpha2: Alpha2::GB,
                        code: "CAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.2052973), longitude: Some(0.1218195), max_latitude: Some(52.7399809), min_latitude: Some(52.005779), max_longitude: Some(0.5144548), min_longitude: Some(-0.4999075)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cambridgeshire"), ("ar", "كامبريدجشير"), ("be", "Кембрыджшыр"), ("bg", "Кеймбриджшър"), ("bn", "কেমব\u{9cd}রিজশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Cambridgeshire"), ("ccp", "𑄝\u{11133}𑄠𑄟\u{11134}𑄝\u{11133}𑄢\u{11128}𑄌\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Cambridgeshire"), ("cs", "Cambridgeshire"), ("cy", "Swydd Gaergrawnt"), ("da", "Cambridgeshire"), ("de", "Cambridgeshire"), ("el", "Κέμπριτζσαϊρ"), ("en", "Cambridgeshire"), ("es", "Cambridgeshire"), ("et", "Cambridgeshire"), ("eu", "Cambridgeshire"), ("fa", "کمبریج\u{200c}شایر"), ("fi", "Cambridgeshire"), ("fr", "Cambridgeshire"), ("ga", "Cambridgeshire"), ("gl", "Cambridgeshire"), ("gu", "ક\u{ac7}મ\u{acd}બ\u{acd}રિજશાયર"), ("he", "קיימברידג׳שייר"), ("hi", "क\u{947}मब\u{94d}रिजशायर"), ("hu", "Cambridgeshire"), ("hy", "Քեմբրիջշիր"), ("id", "Cambridgeshire"), ("is", "Cambridgeshire"), ("it", "Cambridgeshire"), ("ja", "ケンブリッジシャー"), ("ka", "კემბრიჯშირი"), ("kn", "ಕೇಂಬ\u{ccd}ರ\u{cbf}ಜ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "케임브리지셔 주"), ("lt", "Kembridžšyras"), ("lv", "Kembridžšīra"), ("mk", "Кембриџшир"), ("mr", "क\u{947}\u{902}ब\u{94d}रिजशायर"), ("nb", "Cambridgeshire"), ("nl", "Cambridgeshire"), ("no", "Cambridgeshire"), ("pl", "Cambridgeshire"), ("pt", "Cambridgeshire"), ("ro", "Cambridgeshire"), ("ru", "Кембриджшир"), ("sk", "Cambridgeshire"), ("sr", "Кембриџшир"), ("sr_Latn", "Kembridžšir"), ("sv", "Cambridgeshire"), ("ta", "கேம\u{bcd}பிறிட\u{bcd}ஜ\u{bcd}ஷிர\u{bcd}"), ("te", "క\u{c47}ంబ\u{c4d}ర\u{c3f}డ\u{c4d}జ\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "เคมบร\u{e34}ดจ\u{e4c}เชอร\u{e4c}"), ("tr", "Cambridgeshire"), ("uk", "Кембриджшир"), ("ur", "کیمبرجشائر"), ("vi", "Cambridgeshire"), ("yue", "劍橋郡"), ("yue_Hans", "剑桥郡"), ("zh", "劍橋郡")]),
                        unofficial_name_list: ["Cambridgeshire"].to_vec(),
                    }
                ),
                (
                    "CAY",
                    Subdivision{
                        name: "Caerphilly [Caerffili GB-CAF]",
                        country_alpha2: Alpha2::GB,
                        code: "CAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.578829), longitude: Some(-3.218134), max_latitude: Some(51.6040631), min_latitude: Some(51.54968359999999), max_longitude: Some(-3.1765608), min_longitude: Some(-3.2555854)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Кайрфілі"), ("bg", "Карфили"), ("ca", "Sir Caerffili"), ("ccp", "𑄇𑄠𑄬𑄢\u{11134}𑄜\u{11128}𑄣\u{11128}"), ("ceb", "Caerphilly County Borough"), ("cs", "Caerphilly County Borough"), ("cy", "Bwrdeistref Sirol Caerffili"), ("de", "Caerphilly"), ("en", "Caerphilly"), ("es", "Caerphilly County Borough"), ("et", "Caerphilly"), ("eu", "Caerphilly"), ("fa", "شهرستان مستقل کرفیلی"), ("fi", "Caerphilly"), ("fr", "Caerphilly"), ("ga", "Caerffili"), ("it", "distretto di contea di Caerphilly"), ("ja", "ケアフィリ"), ("ko", "케어필리 자치시"), ("lt", "Karfilis"), ("nb", "Caerphilly"), ("nl", "Caerphilly"), ("no", "Caerphilly"), ("pl", "Caerphilly"), ("pt", "Caerphilly"), ("ro", "Caerphilly"), ("ru", "Кайрфилли"), ("sv", "Caerphilly"), ("uk", "Карфіллі"), ("zh", "卡菲利自治市")]),
                        unofficial_name_list: ["Caerffili"].to_vec(),
                    }
                ),
                (
                    "CBF",
                    Subdivision{
                        name: "Central Bedfordshire",
                        country_alpha2: Alpha2::GB,
                        code: "CBF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Цэнтральны Бедфардшыр"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল বেডফোর\u{9cd}ডশ\u{9be}য\u{9bc}\u{9be}র"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢𑄣\u{11134} 𑄝𑄬𑄖\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Central Bedfordshire"), ("da", "Central Bedfordshire"), ("de", "Central Bedfordshire"), ("en", "Central Bedfordshire"), ("es", "Central Bedfordshire"), ("et", "Kesk-Bedfordshire"), ("fa", "بدفوردشر مرکزی"), ("fr", "Central Bedfordshire"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ બ\u{ac7}ડફોર\u{acd}ડશાયર"), ("it", "Central Bedfordshire"), ("ja", "ベッドフォードシャー中心部"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರಲ\u{ccd} ಬ\u{cc6}ಡ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "센트럴베드퍼드셔"), ("nl", "Central Bedfordshire"), ("pl", "Central Bedfordshire"), ("pt", "Central Bedfordshire"), ("ru", "Центральный Бедфордшир"), ("sv", "Central Bedfordshire"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} பெட\u{bcd}டிபோர\u{bcd}டஷிர\u{bcd}"), ("te", "మధ\u{c4d}య బ\u{c46}డ\u{c4d} ఫ\u{c4b}ర\u{c4d}డ\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Центральний Бедфордшир"), ("ur", "وسطی بیڈفورڈشائر"), ("yue", "中百福郡"), ("yue_Hans", "中百福郡"), ("zh", "中央貝德福")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CCG",
                    Subdivision{
                        name: "Causeway Coast and Glens",
                        country_alpha2: Alpha2::GB,
                        code: "CCG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "কোসওয\u{9bc}ে কোস\u{9cd}ট ও গ\u{9cd}লেনস"), ("ccp", "𑄇\u{11127}𑄌\u{11134}𑄤𑄠𑄬 𑄇\u{1112e}𑄌\u{11134}𑄑\u{11134} 𑄃\u{11133}𑄃 𑄉\u{11133}𑄣𑄬𑄚\u{11134}𑄥\u{11134}"), ("de", "Causeway Coast and Glens"), ("en", "Causeway Coast and Glens"), ("es", "Causeway Coast and Glens"), ("fa", "کازوی کوست و گلنز"), ("fr", "Causeway Coast and Glens"), ("gu", "કોઝવ\u{ac7} કોસ\u{acd}ટ અન\u{ac7} ગ\u{acd}લ\u{ac7}ન\u{acd}સ"), ("it", "Distretto di Causeway Coast e Glens"), ("ja", "コーズウェー・コースト・アンド・グランス"), ("kn", "ಕಾಸ\u{ccd}ವೇ ಕೋಸ\u{ccd}ಟ\u{ccd} ಮತ\u{ccd}ತು ಗ\u{ccd}ಲ\u{cc6}ನ\u{ccd}ಸ\u{ccd}"), ("ko", "코즈웨이 연안과 글렌스"), ("lt", "Kausevai Kostas"), ("nl", "Causeway Coast and Glens"), ("pt", "Causeway Coast and Glens"), ("ru", "Косвей-Кост и Гленс"), ("ta", "க\u{bbe}ஸ\u{bcd}வே கோஸ\u{bcd}ட\u{bcd} & க\u{bcd}ளென\u{bcd}ஸ\u{bcd}"), ("te", "క\u{c3e}జ\u{c4d}వ\u{c47} క\u{c4b}స\u{c4d}ట\u{c4d} మర\u{c3f}యు గ\u{c4d}ల\u{c46}న\u{c4d}స\u{c4d}"), ("ur", "کازوئے کوسٹ اور گلینز")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CGN",
                    Subdivision{
                        name: "Ceredigion [Sir Ceredigion]",
                        country_alpha2: Alpha2::GB,
                        code: "CGN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.40728799999999), longitude: Some(-4.069603), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Керъдигиън"), ("bn", "কেরেডিজন"), ("ca", "Ceredigion"), ("ccp", "𑄇𑄬𑄢𑄬𑄓\u{11128}𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "County of Ceredigion"), ("cs", "Ceredigion"), ("cy", "Ceredigion"), ("de", "Ceredigion"), ("en", "Ceredigion"), ("es", "Ceredigion"), ("et", "Ceredigion"), ("eu", "Ceredigion"), ("fa", "کردیگیون"), ("fi", "Ceredigion"), ("fr", "Ceredigion"), ("ga", "Ceredigion"), ("gl", "Ceredigion"), ("gu", "ક\u{ac7}ર\u{ac7}ડિગિયોન"), ("he", "קרדיג׳נשייר"), ("id", "Ceredigion"), ("it", "Ceredigion"), ("ja", "ケレディジョン"), ("kn", "ಸ\u{cc6}ರ\u{cc6}ಡ\u{cbf}ಜನ\u{ccd}"), ("ko", "케레디기온"), ("lt", "Keredigionas"), ("nb", "Ceredigion"), ("nl", "Ceredigion"), ("no", "Ceredigion"), ("pl", "Ceredigion"), ("pt", "Cardiganshire"), ("ro", "Ceredigion"), ("ru", "Кередигион"), ("sv", "Ceredigion"), ("ta", "செரெடிகின\u{bcd}"), ("te", "క\u{c46}ర\u{c46}డ\u{c3f}గ\u{c3f}య\u{c3e}న\u{c4d}"), ("uk", "Кередігіон"), ("ur", "کیریدیجیون"), ("vi", "Ceredigion"), ("zh", "锡尔迪金")]),
                        unofficial_name_list: ["Ceredigion [Sir Ceredigion]"].to_vec(),
                    }
                ),
                (
                    "CHE",
                    Subdivision{
                        name: "Cheshire East",
                        country_alpha2: Alpha2::GB,
                        code: "CHE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Усходні Чэшыр"), ("bn", "চেশ\u{9be}য\u{9bc}\u{9be}র ইস\u{9cd}ট"), ("ccp", "𑄇𑄬𑄥𑄠𑄢\u{11134} 𑄛\u{1112a}𑄇\u{11134}"), ("ceb", "Cheshire East"), ("de", "Cheshire East"), ("en", "Cheshire East"), ("es", "Cheshire East"), ("fa", "چشر شرقی"), ("fr", "Cheshire East"), ("gu", "ચ\u{ac7}શાયર પ\u{ac2}ર\u{acd}વ"), ("hy", "Արևելյան Չեշիր"), ("it", "Cheshire East"), ("ja", "チェシャー・イースト"), ("kn", "ಚ\u{cc6}ಷೈರ\u{ccd} ಈಸ\u{ccd}ಟ\u{ccd}"), ("ko", "체셔이스트"), ("nl", "Cheshire East"), ("pl", "Cheshire East"), ("pt", "Cheshire East"), ("ru", "Восточный Чешир"), ("sv", "Cheshire East"), ("ta", "கிழக\u{bcd}கு சேஸ\u{bc0}ர\u{bcd}"), ("te", "చ\u{c46}ష\u{c48}ర\u{c4d} ఈస\u{c4d}ట\u{c4d}"), ("uk", "Східний Чешир"), ("ur", "چیشائر مشرقی"), ("zh", "東柴郡")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CHW",
                    Subdivision{
                        name: "Cheshire West and Chester",
                        country_alpha2: Alpha2::GB,
                        code: "CHW",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Заходні Чэшыр і Чэстэр"), ("bn", "চেশ\u{9be}য\u{9bc}\u{9be}র ওয\u{9bc}েস\u{9cd}ট ও চেস\u{9cd}ট\u{9be}র"), ("ccp", "𑄇𑄬𑄥𑄠𑄢\u{11134} 𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄃\u{11133}𑄃 𑄇𑄬𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Cheshire West and Chester"), ("de", "Cheshire West and Chester"), ("en", "Cheshire West and Chester"), ("es", "Cheshire West and Chester"), ("fa", "چشر غربی و چستر"), ("fr", "Cheshire West and Chester"), ("gl", "Cheshire West and Chester"), ("gu", "ચ\u{ac7}શાયર પશ\u{acd}ચિમ અન\u{ac7} ચ\u{ac7}સ\u{acd}ટર"), ("it", "Cheshire West and Chester"), ("ja", "チェシャーウェスト・アンド・チェスター"), ("kn", "ಚ\u{cc6}ಶೈರ\u{ccd} ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಮತ\u{ccd}ತು ಚ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}"), ("ko", "체셔웨스트 체스터"), ("nl", "Cheshire West and Chester"), ("pl", "Cheshire West and Chester"), ("pt", "Cheshire West and Chester"), ("ru", "Западный Чешир и Честер"), ("sv", "Cheshire West and Chester"), ("ta", "சேஷிர\u{bcd} அண\u{bcd}ட\u{bcd} மேற\u{bcd}கு செஸ\u{bcd}டர\u{bcd}"), ("te", "చ\u{c46}ష\u{c48}ర\u{c4d} వ\u{c46}స\u{c4d}ట\u{c4d} మర\u{c3f}యు చ\u{c46}స\u{c4d}టర\u{c4d}"), ("uk", "Західний Чешир і Честер"), ("zh", "西柴郡與切斯特")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CLD",
                    Subdivision{
                        name: "Calderdale",
                        country_alpha2: Alpha2::GB,
                        code: "CLD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.716157), longitude: Some(-1.85846), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Калдэрдэйл"), ("bg", "Колдърдейл (община)"), ("ccp", "𑄇\u{11133}𑄠𑄣\u{11134}𑄓𑄢\u{11134}𑄓𑄣𑄬"), ("ceb", "Calderdale"), ("de", "Calderdale"), ("en", "Calderdale"), ("es", "Calderdale"), ("eu", "Calderdale"), ("fa", "کالدردیل"), ("fr", "Calderdale"), ("it", "Calderdale"), ("ja", "カルダーデール"), ("ko", "콜더데일"), ("nb", "Calderdale"), ("nl", "Calderdale"), ("no", "Calderdale"), ("pl", "Calderdale"), ("ro", "Calderdale"), ("ru", "Калдердейл"), ("sv", "Calderdale"), ("uk", "Калдердейл"), ("ur", "کلڈرڈیل"), ("zh", "卡爾德達爾")]),
                        unofficial_name_list: ["Calderdale"].to_vec(),
                    }
                ),
                (
                    "CLK",
                    Subdivision{
                        name: "Clackmannanshire",
                        country_alpha2: Alpha2::GB,
                        code: "CLK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.1241394), longitude: Some(-3.758379), max_latitude: Some(56.2172345), min_latitude: Some(56.07242429999999), max_longitude: Some(-3.5714122), min_longitude: Some(-3.8850904)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كلاكمانشاير"), ("bg", "Клакмананшър"), ("bn", "ক\u{9cd}ল\u{9cd}য\u{9be}কম\u{9be}ন\u{9cd}ন\u{9be}নশ\u{9cd}য\u{9be}র"), ("ca", "Clackmannanshire"), ("ccp", "𑄇\u{11133}𑄣\u{11133}𑄠𑄌\u{11134}𑄟\u{11133}𑄠𑄚𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Clackmannanshire"), ("cs", "Clackmannanshire"), ("cy", "Swydd Clackmannan"), ("de", "Clackmannanshire"), ("en", "Clackmannanshire"), ("es", "Clackmannanshire"), ("et", "Clackmannanshire"), ("eu", "Clackmannanshire"), ("fa", "کلاکماننشر"), ("fi", "Clackmannanshire"), ("fr", "Clackmannanshire"), ("ga", "Clach Mhanainn"), ("gu", "ક\u{acd}લ\u{ac7}કમ\u{ac7}નશાયર"), ("it", "Clackmannanshire"), ("ja", "クラックマナンシャー"), ("kn", "ಕ\u{ccd}ಲಾಕ\u{ccd}ಮನ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "클라크매넌셔"), ("lt", "Klakmananšyras"), ("nb", "Clackmannanshire"), ("nl", "Clackmannanshire"), ("no", "Clackmannanshire"), ("pl", "Clackmannanshire"), ("pt", "Clackmannanshire"), ("ro", "Clackmannanshire"), ("ru", "Клакманнаншир"), ("sv", "Clackmannanshire"), ("ta", "கிள\u{bbe}க\u{bcd}க\u{bcd}மனிஷிர\u{bcd}"), ("te", "క\u{c4d}ల\u{c3e}క\u{c4d} మన\u{c4d}నన\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Клакманнаншир"), ("ur", "کلاکماننانشائر"), ("zh", "克拉克曼南郡")]),
                        unofficial_name_list: ["Clackmannanshire"].to_vec(),
                    }
                ),
                (
                    "CMA",
                    Subdivision{
                        name: "Cumbria",
                        country_alpha2: Alpha2::GB,
                        code: "CMA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.5772323), longitude: Some(-2.7974835), max_latitude: Some(55.18898129999999), min_latitude: Some(54.0418929), max_longitude: Some(-2.1590187), min_longitude: Some(-3.6402003)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cumbria"), ("ar", "كامبريا"), ("be", "Камбрыя"), ("bg", "Къмбрия"), ("bn", "ক\u{9be}ম\u{9cd}ব\u{9cd}রিয\u{9bc}\u{9be}"), ("ca", "Cúmbria"), ("ccp", "𑄇𑄟\u{11134}𑄝\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Cumbria"), ("cs", "Cumbria"), ("cy", "Cumbria"), ("da", "Cumbria"), ("de", "Cumbria"), ("el", "Κούμπρια"), ("en", "Cumbria"), ("es", "Cumbria"), ("et", "Cumbria"), ("eu", "Cumbria"), ("fa", "کامبریا"), ("fi", "Cumbria"), ("fr", "Cumbria"), ("ga", "Cumbria"), ("gl", "Cumbria"), ("gu", "કમ\u{acd}બ\u{acd}રિયા"), ("he", "קאמבריה"), ("hi", "कम\u{94d}ब\u{94d}रिया"), ("hr", "Cumbria"), ("hu", "Cumbria"), ("hy", "Կումբրիա"), ("id", "Cumbria"), ("is", "Cumbria"), ("it", "Cumbria"), ("ja", "カンブリア"), ("kn", "ಕುಂಬ\u{ccd}ರ\u{cbf}ಯಾ"), ("ko", "컴브리아 주"), ("lt", "Kambrija"), ("lv", "Kambrija"), ("mk", "Камбрија"), ("mr", "क\u{902}ब\u{94d}रिया"), ("nb", "Cumbria"), ("ne", "कम\u{94d}ब\u{94d}रिआ"), ("nl", "Cumbria"), ("no", "Cumbria"), ("pl", "Kumbria"), ("pt", "Cúmbria"), ("ro", "Cumbria"), ("ru", "Камбрия"), ("sk", "Cumbria"), ("sr", "Камбрија"), ("sr_Latn", "Kambrija"), ("sv", "Cumbria"), ("ta", "கும\u{bcd}பரிய\u{bbe}"), ("te", "కుంబ\u{c4d}ర\u{c3f}య\u{c3e}"), ("th", "ค\u{e31}มเบร\u{e35}ย"), ("tr", "Cumbria"), ("uk", "Камбрія"), ("ur", "کامبریا"), ("vi", "Cumbria"), ("yue", "金巴倫郡"), ("yue_Hans", "金巴伦郡"), ("zh", "坎布里亞郡")]),
                        unofficial_name_list: ["Cumbria"].to_vec(),
                    }
                ),
                (
                    "CMD",
                    Subdivision{
                        name: "Camden",
                        country_alpha2: Alpha2::GB,
                        code: "CMD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.55170589999999), longitude: Some(-0.1588255), max_latitude: Some(51.5729787), min_latitude: Some(51.5126521), max_longitude: Some(-0.1053499), min_longitude: Some(-0.2135012)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Бора Кэмдэн"), ("bn", "ক\u{9cd}য\u{9be}মডেন"), ("ca", "Camden"), ("ccp", "𑄇\u{11133}𑄠𑄟\u{11134}𑄓𑄬𑄚\u{11134}"), ("cs", "Camden"), ("cy", "Camden"), ("da", "Camden"), ("de", "London Borough of Camden"), ("en", "Camden"), ("es", "Camden"), ("et", "Camden"), ("eu", "Camden"), ("fa", "منطقه کمدن لندن"), ("fi", "Camden"), ("fr", "borough londonien de Camden"), ("ga", "Buirg Londan Camden"), ("gu", "લ\u{a82}ડન બોરો ઓફ ક\u{ac7}મડ\u{ac7}ન"), ("he", "קמדן"), ("hi", "क\u{948}मडन बरो"), ("hu", "Camden kerület"), ("hy", "Քամդեն"), ("is", "Camden"), ("it", "Borgo londinese di Camden"), ("ja", "カムデン・ロンドン特別区"), ("ka", "კემდენი"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಕ\u{ccd}ಯಾಮ\u{ccd}ಡ\u{cc6}ನ\u{ccd}"), ("ko", "캠던 구"), ("mk", "Камден"), ("nb", "Camden"), ("nl", "Camden"), ("no", "Camden"), ("pl", "London Borough of Camden"), ("pt", "Camden"), ("ro", "Camden"), ("ru", "Камден"), ("sk", "Camden"), ("sl", "Camden, London"), ("sv", "London Borough of Camden"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} கேம\u{bcd}டன\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} క\u{c3e}మ\u{c4d}ద\u{c47}న\u{c4d}"), ("tr", "Camden, Londra"), ("uk", "Кемден"), ("ur", "کیمڈن بورو"), ("vi", "Khu Camden của Luân Đôn"), ("yue", "劍頓區"), ("yue_Hans", "剑顿区"), ("zh", "卡姆登區")]),
                        unofficial_name_list: ["Camden"].to_vec(),
                    }
                ),
                (
                    "CMN",
                    Subdivision{
                        name: "Carmarthenshire [Sir Gaerfyrddin GB-GFY]",
                        country_alpha2: Alpha2::GB,
                        code: "CMN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8598535), longitude: Some(-4.260853099999999), max_latitude: Some(52.1423962), min_latitude: Some(51.6547722), max_longitude: Some(-3.6471249), min_longitude: Some(-4.723076)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Кармартэншыр"), ("bg", "Кармартъншър"), ("bn", "ক\u{9be}র\u{9cd}ম\u{9be}দেনশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Sir Gaerfyrddin"), ("ccp", "𑄇𑄢\u{11134}𑄟𑄢\u{11134}𑄗𑄬𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Carmarthenshire"), ("cs", "Carmarthenshire"), ("cy", "Sir Gaerfyrddin"), ("de", "Carmarthenshire"), ("en", "Carmarthenshire"), ("es", "Carmarthenshire"), ("et", "Carmarthenshire"), ("eu", "Carmarthenshire"), ("fa", "کارمارتنشر"), ("fi", "Carmarthenshire"), ("fr", "Carmarthenshire"), ("ga", "Sir Gaerfyrddin"), ("gl", "Sir Gaerfyrddin"), ("gu", "કાર\u{acd}માથ\u{ac7}નશાયર"), ("he", "קרמרתנשייר"), ("hi", "कार\u{94d}मार\u{94d}थनशायर"), ("hu", "Carmarthenshire"), ("id", "Carmarthenshire"), ("it", "Carmarthenshire"), ("ja", "カーマーゼンシャー"), ("kn", "ಕಾರ\u{ccd}ಮರ\u{ccd}ಥನ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "카마던셔"), ("lt", "Karmartenšyras"), ("nb", "Carmarthenshire"), ("nl", "Carmarthenshire"), ("no", "Carmarthenshire"), ("pl", "Carmarthenshire"), ("pt", "Carmarthenshire"), ("ro", "Carmarthenshire"), ("ru", "Кармартеншир"), ("sv", "Carmarthenshire"), ("ta", "க\u{bbe}ர\u{bcd}மர\u{bcd}தேன\u{bcd}ஷிர\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}మ\u{c46}ర\u{c4d}థ\u{c40}న\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Кармартеншир"), ("ur", "کارمارتھینشائر"), ("vi", "Carmarthenshire"), ("zh", "卡马森郡")]),
                        unofficial_name_list: ["Sir Gaerfyrddin"].to_vec(),
                    }
                ),
                (
                    "CON",
                    Subdivision{
                        name: "Cornwall",
                        country_alpha2: Alpha2::GB,
                        code: "CON",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.5036299), longitude: Some(-4.6524982), max_latitude: Some(50.93127), min_latitude: Some(49.9554143), max_longitude: Some(-4.1661755), min_longitude: Some(-5.7226218)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cornwall"), ("ar", "كورنوال"), ("be", "Паўвостраў Корнуал"), ("bg", "Корнуол"), ("bn", "কর\u{9cd}নওয\u{9bc}\u{9be}ল"), ("ca", "Cornualla"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄚\u{11134}𑄤𑄣\u{11134}"), ("ceb", "Cornwall"), ("cs", "Cornwall"), ("cy", "Cernyw"), ("da", "Cornwall"), ("de", "Cornwall"), ("el", "Κορνουάλη"), ("en", "Cornwall"), ("es", "Cornualles"), ("et", "Cornwall"), ("eu", "Kornualles"), ("fa", "کورن وال"), ("fi", "Cornwall"), ("fr", "Cornouailles"), ("gl", "Cornualles - Kernow"), ("gu", "કોર\u{acd}નવોલ"), ("he", "קורנוול"), ("hi", "कॉर\u{94d}नवल"), ("hr", "Cornwall"), ("hu", "Cornwall"), ("hy", "Կոռնուոլ"), ("id", "Cornwall"), ("is", "Cornwall"), ("it", "Cornovaglia"), ("ja", "コーンウォール"), ("ka", "კორნუოლი"), ("kn", "ಕಾರ\u{ccd}ನ\u{ccd}ವಾಲ\u{ccd}"), ("ko", "콘월 주"), ("lt", "Kornvalis"), ("lv", "Kornvola"), ("mr", "कॉर\u{94d}नवॉल"), ("ms", "Cornwall"), ("nb", "Cornwall"), ("nl", "Cornwall"), ("no", "Cornwall"), ("pl", "Kornwalia"), ("pt", "Cornualha"), ("ro", "Cornwall"), ("ru", "Корнуолл"), ("sk", "Cornwall"), ("sl", "Cornwall"), ("sr", "Корнвол"), ("sr_Latn", "Kornvol"), ("sv", "Cornwall"), ("ta", "கோர\u{bcd}ன\u{bcd}வ\u{bbe}ல\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}న\u{c4d}వ\u{c3e}ల\u{c4d}"), ("th", "คอร\u{e4c}นวอลล\u{e4c}"), ("tr", "Cornwall"), ("uk", "Корнуол"), ("ur", "کونوال"), ("vi", "Cornwall"), ("zh", "康沃爾郡")]),
                        unofficial_name_list: ["Cornwall and Isles of Scilly"].to_vec(),
                    }
                ),
                (
                    "COV",
                    Subdivision{
                        name: "Coventry",
                        country_alpha2: Alpha2::GB,
                        code: "COV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.406822), longitude: Some(-1.519693), max_latitude: Some(52.4634723), min_latitude: Some(52.3639108), max_longitude: Some(-1.4239508), min_longitude: Some(-1.6058853)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Coventry"), ("ar", "كوفنتري"), ("az", "Koventri"), ("be", "Ковентры"), ("bg", "Ковънтри"), ("bn", "কোভেন\u{9cd}ট\u{9cd}রি"), ("ca", "Coventry"), ("ccp", "𑄇\u{1112e}𑄞𑄬𑄚\u{11134}𑄑\u{11133}𑄢\u{11128}"), ("ceb", "Coventry"), ("cs", "Coventry"), ("da", "Coventry"), ("de", "Coventry"), ("el", "Κόβεντρι"), ("en", "Coventry"), ("es", "Coventry"), ("et", "Coventry"), ("eu", "Coventry"), ("fa", "کاونتری"), ("fi", "Coventry"), ("fr", "Coventry"), ("gl", "Coventry"), ("gu", "કોવ\u{ac7}ન\u{acd}ટ\u{acd}રી"), ("he", "קובנטרי"), ("hi", "कोव\u{947}न\u{94d}ट\u{94d}री"), ("hr", "Coventry"), ("hu", "Coventry"), ("hy", "Կովենտրի"), ("id", "Coventry"), ("is", "Coventry"), ("it", "Coventry"), ("ja", "コヴェントリー"), ("ka", "კოვენტრი"), ("kn", "ಕೊವ\u{cc6}ಂಟ\u{ccd}ರ\u{cbf}"), ("ko", "코번트리"), ("lt", "Koventris"), ("lv", "Koventrija"), ("mr", "कॉव\u{94d}ह\u{947}\u{902}ट\u{94d}री"), ("ms", "Coventry"), ("nb", "Coventry"), ("nl", "Coventry"), ("no", "Coventry"), ("pl", "Coventry"), ("pt", "Coventry"), ("ro", "Coventry"), ("ru", "Ковентри"), ("si", "කොවන\u{dca}ට\u{dca}\u{200d}ර\u{dd2}"), ("sk", "Coventry"), ("sl", "Coventry"), ("sr", "Ковентри"), ("sr_Latn", "Koventri"), ("sv", "Coventry"), ("sw", "Coventry"), ("ta", "கோவென\u{bcd}றி"), ("te", "క\u{c3e}వ\u{c46}ంట\u{c4d}ర\u{c40}"), ("th", "คอเวนทร\u{e35}"), ("tr", "Coventry"), ("uk", "Ковентрі"), ("ur", "کووینٹری"), ("vi", "Coventry"), ("zh", "考文垂")]),
                        unofficial_name_list: ["Coventry"].to_vec(),
                    }
                ),
                (
                    "CRF",
                    Subdivision{
                        name: "Cardiff [Caerdydd GB-CRD]",
                        country_alpha2: Alpha2::GB,
                        code: "CRF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.48158100000001), longitude: Some(-3.17909), max_latitude: Some(51.5609063), min_latitude: Some(51.4457441), max_longitude: Some(-3.1215184), min_longitude: Some(-3.2823817)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cardiff"), ("am", "ካርዲፍ"), ("ar", "كارديف"), ("az", "Kardiff"), ("be", "акруга Кардыф"), ("bg", "Кардиф"), ("bn", "ক\u{9be}র\u{9cd}ডিফ"), ("bs", "Cardiff"), ("ca", "Cardiff"), ("ccp", "𑄇𑄢\u{11134}𑄓\u{11128}𑄛\u{11134}"), ("ceb", "Cardiff"), ("cs", "Cardiff"), ("cy", "Dinas a Sir Caerdydd"), ("da", "Cardiff"), ("de", "Cardiff"), ("el", "Κάρντιφ"), ("en", "Cardiff"), ("es", "Cardiff"), ("et", "Cardiff"), ("eu", "Cardiff"), ("fa", "کاردیف"), ("fi", "Cardiff"), ("fr", "Cardiff"), ("ga", "Caerdydd"), ("gl", "Cardiff"), ("gu", "કાર\u{acd}ડિફ"), ("he", "קארדיף"), ("hi", "कार\u{94d}डिफ\u{93c}"), ("hr", "Cardiff"), ("hu", "Cardiff"), ("hy", "Քարդիֆ"), ("id", "Cardiff"), ("is", "Cardiff"), ("it", "Cardiff"), ("ja", "カーディフ"), ("jv", "Cardiff"), ("ka", "კარდიფი"), ("kk", "Кардифф"), ("kn", "ಕಾರ\u{ccd}ಡ\u{cbf}ಫ\u{ccd}"), ("ko", "카디프"), ("ky", "Кардифф"), ("lt", "Kardifas"), ("lv", "Kārdifa"), ("mk", "Кардиф"), ("ml", "ക\u{d3e}ർഡിഫ\u{d4d}"), ("mn", "Кардифф"), ("mr", "कार\u{94d}डिफ"), ("ms", "Cardiff"), ("my", "ကားဒစ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Cardiff"), ("nl", "City and County of Cardiff"), ("no", "Cardiff"), ("pa", "ਕਾਰਡਿਫ\u{a3c}"), ("pl", "Cardiff"), ("pt", "Cardiff"), ("ro", "Cardiff"), ("ru", "Кардифф"), ("si", "ක\u{dcf}ඩ\u{dd2}ෆ\u{dca}"), ("sk", "Cardiff"), ("sl", "Cardiff"), ("sq", "Kardif"), ("sr", "Кардиф"), ("sr_Latn", "Kardif"), ("sv", "Cardiff"), ("sw", "Cardiff"), ("ta", "க\u{bbe}ர\u{bcd}டிஃப\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}డ\u{c3f}ఫ\u{c4d}"), ("th", "คาร\u{e4c}ด\u{e34}ฟฟ\u{e4c}"), ("tk", "Kardiff"), ("tr", "Cardiff"), ("uk", "Кардіфф"), ("ur", "کارڈف"), ("uz", "Kardiff"), ("vi", "Cardiff"), ("yue", "卡迪夫"), ("yue_Hans", "卡迪夫"), ("zh", "加的夫")]),
                        unofficial_name_list: ["Caerdydd"].to_vec(),
                    }
                ),
                (
                    "CRY",
                    Subdivision{
                        name: "Croydon",
                        country_alpha2: Alpha2::GB,
                        code: "CRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.376165), longitude: Some(-0.098234), max_latitude: Some(51.40585189999999), min_latitude: Some(51.3298043), max_longitude: Some(-0.0106914), min_longitude: Some(-0.1537876)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "бора Кройдан"), ("ca", "Croydon"), ("ccp", "𑄇\u{11133}𑄢\u{11130}𑄓\u{11127}𑄚\u{11134}"), ("ceb", "Croydon"), ("cs", "Croydon"), ("cy", "Croydon"), ("da", "Croydon"), ("de", "London Borough of Croydon"), ("en", "Croydon"), ("es", "Croydon"), ("et", "Croydon"), ("eu", "Croydon"), ("fa", "منطقه کرویدون لندن"), ("fi", "Croydon"), ("fr", "district londonien de Croydon"), ("ga", "Buirg Londan Croydon"), ("gl", "Croydon"), ("he", "קרוידון"), ("hi", "क\u{94d}रॉयडन बरो"), ("hu", "Croydon kerület"), ("hy", "Քրոյդոն"), ("is", "Croydon"), ("it", "Croydon"), ("ja", "クロイドン・ロンドン特別区"), ("ko", "크로이던 구"), ("mr", "क\u{94d}रॉयडन"), ("nb", "Croydon"), ("nl", "Croydon"), ("no", "Croydon"), ("pl", "London Borough of Croydon"), ("pt", "Croydon"), ("ro", "Croydon"), ("ru", "Кройдон"), ("sl", "Croydon, London"), ("sv", "London Borough of Croydon"), ("tr", "Croydon"), ("uk", "Кройдон"), ("ur", "کروئڈن بورو"), ("vi", "Khu Croydon của Luân Đôn"), ("zh", "克羅伊登區")]),
                        unofficial_name_list: ["Croydon"].to_vec(),
                    }
                ),
                (
                    "CWY",
                    Subdivision{
                        name: "Conwy",
                        country_alpha2: Alpha2::GB,
                        code: "CWY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.282872), longitude: Some(-3.82948), max_latitude: Some(53.2933317), min_latitude: Some(53.2549437), max_longitude: Some(-3.8198164), min_longitude: Some(-3.861234699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Конуи"), ("ca", "Conwy"), ("ccp", "𑄇\u{11127}𑄚\u{11134}𑄃\u{1112a}𑄃\u{11128}"), ("ceb", "Conwy"), ("cs", "Conwy"), ("cy", "Bwrdeistref Sirol Conwy"), ("de", "Conwy"), ("en", "Conwy"), ("es", "Conwy County Borough"), ("et", "Conwy"), ("eu", "Conwy"), ("fa", "شهرستان مستقل کانوی"), ("fi", "Conwy"), ("fr", "Conwy"), ("ga", "Conwy"), ("gl", "Conwy County Borough"), ("he", "קונווי"), ("it", "distretto di contea di Conwy"), ("ja", "コンウィ"), ("ko", "콘위 자치시"), ("lt", "Konvis"), ("nb", "Conwy"), ("nl", "Conwy"), ("no", "Conwy"), ("pl", "Conwy"), ("ro", "Conwy"), ("ru", "Конуи"), ("sv", "Conwy"), ("uk", "Конві"), ("zh", "康威自治市")]),
                        unofficial_name_list: ["Aberconwy and Colwyn"].to_vec(),
                    }
                ),
                (
                    "DAL",
                    Subdivision{
                        name: "Darlington",
                        country_alpha2: Alpha2::GB,
                        code: "DAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.52361), longitude: Some(-1.559458), max_latitude: Some(54.55878449999999), min_latitude: Some(54.50360509999999), max_longitude: Some(-1.4971054), min_longitude: Some(-1.6095913)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓𑄢\u{11134}𑄣\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Darlington (kondado)"), ("de", "Borough of Darlington"), ("en", "Darlington"), ("fr", "Darlington"), ("it", "Darlington (borough)"), ("ja", "ダーリントン"), ("ko", "달링턴 구"), ("nb", "Darlington"), ("nl", "Darlington"), ("no", "Darlington"), ("pl", "Darlington (borough)"), ("pt", "Darlington"), ("ru", "Дарлингтон"), ("sv", "Darlington"), ("uk", "Дарлінгтон (район)"), ("zh", "達靈頓區")]),
                        unofficial_name_list: ["Darlington"].to_vec(),
                    }
                ),
                (
                    "DBY",
                    Subdivision{
                        name: "Derbyshire",
                        country_alpha2: Alpha2::GB,
                        code: "DBY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.122322), longitude: Some(-1.5136821), max_latitude: Some(53.5404138), min_latitude: Some(52.6965216), max_longitude: Some(-1.1664888), min_longitude: Some(-2.0340978)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Derbyshire"), ("ar", "داربيشير"), ("be", "Графства Дэрбішыр"), ("bg", "Дарбишър"), ("bn", "ড\u{9be}র\u{9cd}বিশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Derbyshire"), ("ccp", "𑄓𑄢\u{11134}𑄝\u{1112d}𑄥𑄠𑄢\u{11134}"), ("ceb", "Derbyshire"), ("cs", "Derbyshire"), ("cy", "Swydd Derby"), ("da", "Derbyshire"), ("de", "Derbyshire"), ("el", "Ντέρμπισαϊρ"), ("en", "Derbyshire"), ("es", "Derbyshire"), ("et", "Derbyshire"), ("eu", "Derbyshire"), ("fa", "دربی\u{200c}شر"), ("fi", "Derbyshire"), ("fr", "Derbyshire"), ("ga", "Derbyshire"), ("gl", "Derbyshire"), ("gu", "ડર\u{acd}બિશાયર"), ("he", "דרבישייר"), ("hi", "डर\u{94d}बीशायर"), ("hu", "Derbyshire"), ("hy", "Դերբիշիր"), ("id", "Derbyshire"), ("is", "Derbyshire"), ("it", "Derbyshire"), ("ja", "ダービーシャー"), ("kn", "ಡರ\u{ccd}ಬ\u{cbf}ಶೈರ\u{ccd}"), ("ko", "더비셔 주"), ("lt", "Derbišyras"), ("lv", "Dārbišīra"), ("mk", "Дарбишир"), ("mr", "डर\u{94d}बीशायर"), ("nb", "Derbyshire"), ("nl", "Derbyshire"), ("no", "Derbyshire"), ("pl", "Derbyshire"), ("pt", "Derbyshire"), ("ro", "Derbyshire"), ("ru", "Дербишир"), ("sk", "Derbyshire"), ("sl", "Derbyshire"), ("sr", "Дарбишир"), ("sr_Latn", "Darbišir"), ("sv", "Derbyshire"), ("ta", "தேர\u{bcd}ப\u{bcd}யஷிர\u{bcd}"), ("te", "డర\u{c4d}బ\u{c3f}ష\u{c48}ర\u{c4d}"), ("th", "ดาร\u{e4c}บ\u{e34}เชอร\u{e4c}"), ("tr", "Derbyshire"), ("uk", "Дербішир"), ("ur", "ڈربیشائر"), ("vi", "Derbyshire"), ("yue", "打比郡"), ("yue_Hans", "打比郡"), ("zh", "德比郡")]),
                        unofficial_name_list: ["Derbyshire"].to_vec(),
                    }
                ),
                (
                    "DEN",
                    Subdivision{
                        name: "Denbighshire [Sir Ddinbych GB-DDB]",
                        country_alpha2: Alpha2::GB,
                        code: "DEN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "графства Дэнбішыр"), ("bg", "Денбишър"), ("bn", "ডেনবিগশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Sir Ddinbych"), ("ccp", "𑄓𑄚\u{11134}𑄝\u{1112d}𑄥𑄠𑄢\u{11134}"), ("ceb", "Denbighshire"), ("cs", "Denbighshire"), ("cy", "Sir Ddinbych"), ("de", "Denbighshire"), ("en", "Denbighshire"), ("es", "Denbighshire"), ("et", "Denbighshire"), ("eu", "Denbighshire"), ("fa", "دنبیشر"), ("fi", "Denbighshire"), ("fr", "Denbighshire"), ("ga", "Sir Ddinbych"), ("gl", "Sir Ddinbych"), ("gu", "ડ\u{ac7}નબિઘશાયર"), ("he", "דנבישייר"), ("hy", "Դենբիշիր"), ("id", "Denbighshire"), ("it", "Denbighshire"), ("ja", "デンビーシャー"), ("kn", "ಡ\u{cc6}ನ\u{ccd}ಬ\u{cbf}ಘೈರ\u{ccd}"), ("ko", "덴비셔"), ("lt", "Denbigšyras"), ("nb", "Denbighshire"), ("nl", "Denbighshire"), ("no", "Denbighshire"), ("pl", "Denbighshire"), ("pt", "Denbighshire"), ("ro", "Denbighshire"), ("ru", "Денбишир"), ("sl", "Denbighshire"), ("sv", "Denbighshire"), ("ta", "தேம\u{bcd}பியிஷிர\u{bcd}"), ("te", "డ\u{c46}న\u{c4d} బ\u{c3f}గ\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Денбігшир"), ("ur", "دینبیگھشائر"), ("vi", "Denbighshire"), ("zh", "登比郡")]),
                        unofficial_name_list: ["Sir Ddinbych"].to_vec(),
                    }
                ),
                (
                    "DER",
                    Subdivision{
                        name: "Derby",
                        country_alpha2: Alpha2::GB,
                        code: "DER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.9225301), longitude: Some(-1.4746186), max_latitude: Some(52.9681352), min_latitude: Some(52.861037), max_longitude: Some(-1.3830782), min_longitude: Some(-1.556857)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ደርቢ"), ("ar", "ديربي"), ("az", "Derbi"), ("be", "Дэрбі"), ("bg", "Дарби"), ("bn", "ড\u{9be}র\u{9cd}বি"), ("ca", "Derby"), ("ccp", "𑄓𑄢\u{11134}𑄝\u{1112d}"), ("ceb", "City of Derby"), ("cs", "Derby"), ("cy", "Derby"), ("da", "Derby"), ("de", "Derby"), ("el", "Ντέρμπι"), ("en", "Derby"), ("es", "Derby"), ("et", "Derby"), ("eu", "Derby"), ("fa", "داربی، انگلستان"), ("fi", "Derby"), ("fr", "Derby"), ("ga", "Derby"), ("gu", "ડર\u{acd}બી"), ("he", "דרבי"), ("hi", "डर\u{94d}बी"), ("hu", "Derby"), ("hy", "Դերբի"), ("id", "Derby"), ("is", "Derby"), ("it", "Derby"), ("ja", "ダービー"), ("jv", "Derby"), ("kn", "ಡರ\u{ccd}ಬ\u{cbf}"), ("ko", "더비"), ("lt", "Derbis"), ("lv", "Dārbi"), ("mk", "Дарби"), ("mr", "डर\u{94d}बी"), ("ms", "Derby"), ("nb", "Derby"), ("nl", "Derby"), ("no", "Derby"), ("pa", "ਡਰਬੀ"), ("pl", "Derby"), ("pt", "Derby"), ("ro", "Derby"), ("ru", "Дерби"), ("si", "ඩර\u{dca}බ\u{dd2}"), ("sk", "Derby"), ("sl", "Derby"), ("sq", "Derby"), ("sr", "Дарби"), ("sr_Latn", "Darbi"), ("sv", "Derby"), ("sw", "Derby"), ("ta", "டர\u{bcd}பி"), ("te", "డ\u{c46}ర\u{c4d}బ\u{c3f}"), ("th", "ดาร\u{e4c}บ\u{e35}"), ("tr", "Derby"), ("uk", "Дербі"), ("ur", "ڈربی"), ("vi", "Derby"), ("yue", "打比市"), ("yue_Hans", "打比市"), ("zh", "德比")]),
                        unofficial_name_list: ["Derby"].to_vec(),
                    }
                ),
                (
                    "DEV",
                    Subdivision{
                        name: "Devon",
                        country_alpha2: Alpha2::GB,
                        code: "DEV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.77721349999999), longitude: Some(-3.999461), max_latitude: Some(51.2461984), min_latitude: Some(50.2018962), max_longitude: Some(-2.8866406), min_longitude: Some(-4.6806563)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Devon"), ("ar", "ديفون"), ("az", "Devon"), ("be", "Графства Дэваншыр"), ("bg", "Девън"), ("bn", "ডেভন"), ("ca", "Devon"), ("ccp", "𑄓𑄬𑄞\u{11127}𑄚\u{11134}"), ("ceb", "Devon (kondado)"), ("cs", "Devon"), ("cy", "Dyfnaint"), ("da", "Devon"), ("de", "Devon"), ("el", "Ντέβον"), ("en", "Devon"), ("es", "Devon"), ("et", "Devoni krahvkond"), ("eu", "Devon"), ("fa", "دوون"), ("fi", "Devon"), ("fr", "Devon"), ("ga", "Devon"), ("gl", "Devon"), ("gu", "ડ\u{ac7}વોન"), ("he", "דבון"), ("hi", "ड\u{947}वन"), ("hr", "Devon"), ("hu", "Devon"), ("hy", "Դևոն"), ("id", "Devon"), ("is", "Devon"), ("it", "Devon"), ("ja", "デヴォン"), ("kn", "ಡ\u{cc6}ವೊನ\u{ccd}"), ("ko", "데번 주"), ("lt", "Devonas"), ("lv", "Devona"), ("mk", "Девон"), ("ml", "ഡെവൺ"), ("mr", "ड\u{947}व\u{94d}हॉन"), ("nb", "Devon"), ("nl", "Devon"), ("no", "Devon"), ("pl", "Devon"), ("pt", "Devon"), ("ro", "Devon"), ("ru", "Девон"), ("sk", "Devon"), ("sl", "Devon"), ("sr", "Девон"), ("sr_Latn", "Devon"), ("sv", "Devon"), ("ta", "டேவன\u{bcd}"), ("te", "డ\u{c46}వన\u{c4d}"), ("th", "เดวอน"), ("tr", "Devon"), ("uk", "Девон"), ("ur", "ڈیون"), ("vi", "Devon"), ("yue", "德雲 （行政郡）"), ("yue_Hans", "德云 （行政郡）"), ("zh", "德文郡")]),
                        unofficial_name_list: ["Devon"].to_vec(),
                    }
                ),
                (
                    "DGY",
                    Subdivision{
                        name: "Dumfries and Galloway",
                        country_alpha2: Alpha2::GB,
                        code: "DGY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.988285), longitude: Some(-3.8577839), max_latitude: Some(55.46405009999999), min_latitude: Some(54.6332381), max_longitude: Some(-2.8573637), min_longitude: Some(-5.187639)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دومفريز وغالاوي"), ("be", "Дамфрыс-энд-Галаўэй"), ("bg", "Дъмфрийс анд Голоуей"), ("bn", "ড\u{9be}ম\u{9cd}ফ\u{9cd}র\u{9be}ই ও গ\u{9be}লোভেই"), ("ca", "Dumfries i Galloway"), ("ccp", "𑄓\u{1112a}𑄟\u{11134}𑄜\u{11133}𑄢\u{1112d}𑄌\u{11134} 𑄃\u{11133}𑄃 𑄉\u{11133}𑄠𑄣\u{1112e}𑄠𑄬"), ("ceb", "Dumfries and Galloway"), ("cs", "Dumfries a Galloway"), ("cy", "Dumfries a Galloway"), ("de", "Dumfries and Galloway"), ("en", "Dumfries and Galloway"), ("es", "Dumfries and Galloway"), ("et", "Dumfries and Galloway"), ("eu", "Dumfries eta Galloway"), ("fa", "دامفریس و گالووی"), ("fi", "Dumfries ja Galloway"), ("fr", "Dumfries and Galloway"), ("ga", "Comhairle Dhún Phrís is Gall-Ghaidhealaibh"), ("gu", "ડમફ\u{acd}રીઝ અન\u{ac7} ગ\u{ac7}લોવ\u{ac7}"), ("id", "Dumfries and Galloway"), ("it", "Dumfries e Galloway"), ("ja", "ダンフリーズ・アンド・ガロウェイ"), ("kn", "ಡಮ\u{ccd}ಫ\u{ccd}ರೀಸ\u{ccd} ಮತ\u{ccd}ತು ಗ\u{ccd}ಯಾಲೋವ\u{cc6}"), ("ko", "덤프리스 갤러웨이"), ("lt", "Damfrisas ir Galovėjus"), ("nb", "Dumfries and Galloway"), ("nl", "Dumfries and Galloway"), ("no", "Dumfries and Galloway"), ("pl", "Dumfries and Galloway"), ("pt", "Dumfries and Galloway"), ("ro", "Dumfries and Galloway"), ("ru", "Дамфрис-энд-Галловей"), ("sv", "Dumfries and Galloway"), ("ta", "டுமிபிரிஸ\u{bcd} & கல\u{bcd}லோவ\u{bbe}ய\u{bcd}"), ("te", "డంఫ\u{c4d}ర\u{c40}స\u{c4d} మర\u{c3f}యు గ\u{c3e}ల\u{c4d}ల\u{c4b}వ\u{c47}"), ("uk", "Дамфріс і Галловей"), ("zh", "鄧弗里斯-加洛韋")]),
                        unofficial_name_list: ["Dumfries and Galloway"].to_vec(),
                    }
                ),
                (
                    "DNC",
                    Subdivision{
                        name: "Doncaster",
                        country_alpha2: Alpha2::GB,
                        code: "DNC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.52282), longitude: Some(-1.128462), max_latitude: Some(53.5671532), min_latitude: Some(53.4705407), max_longitude: Some(-1.0472957), min_longitude: Some(-1.1927515)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "бора Данкастэр"), ("bg", "Донкастър (община)"), ("ccp", "𑄓\u{11127}𑄚\u{11134}𑄇𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Doncaster (kondado)"), ("de", "Metropolitan Borough of Doncaster"), ("en", "Doncaster"), ("es", "Municipio metropolitano de Doncaster"), ("fa", "کلان\u{200c}شهر مستقل دانکستر"), ("fr", "district métropolitain de Doncaster"), ("it", "Doncaster"), ("ja", "メトロポリタン・バラ・オブ・ドンカスター"), ("ko", "동커스터 도시 자치구"), ("nb", "Doncaster (distrikt)"), ("nl", "Doncaster"), ("no", "Doncaster (distrikt)"), ("pl", "Metropolitan Borough of Doncaster"), ("ru", "Донкастер"), ("sv", "Doncaster"), ("uk", "Донкастер"), ("ur", "میٹروپولیٹن بورو ڈانکاسٹر"), ("zh", "唐卡斯特都會自治市")]),
                        unofficial_name_list: ["Doncaster"].to_vec(),
                    }
                ),
                (
                    "DND",
                    Subdivision{
                        name: "Dundee City",
                        country_alpha2: Alpha2::GB,
                        code: "DND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.462018), longitude: Some(-2.970721), max_latitude: Some(56.50559699999999), min_latitude: Some(56.45092169999999), max_longitude: Some(-2.8356419), min_longitude: Some(-3.0980246)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Dundee, Skotland"), ("ar", "دندي"), ("az", "Dandi"), ("be", "Горад Дандзі"), ("bg", "Дънди"), ("bn", "ড\u{9cd}য\u{9be}ন\u{9cd}ডি"), ("ca", "Dundee"), ("ccp", "𑄓𑄚\u{11134}𑄓\u{11129}"), ("ceb", "Dundee"), ("cs", "Dundee"), ("cy", "Dundee"), ("da", "Dundee"), ("de", "Dundee"), ("el", "Ντάντι"), ("en", "Dundee"), ("es", "Dundee"), ("et", "Dundee"), ("eu", "Dundee"), ("fa", "داندی"), ("fi", "Dundee"), ("fr", "Dundee"), ("ga", "Dún Déagh"), ("gl", "Dundee"), ("gu", "ડ\u{a82}ડી"), ("he", "דנדי"), ("hi", "डन\u{94d}डी नोड"), ("hu", "Dundee"), ("hy", "Դանդի"), ("id", "Dundee"), ("is", "Dundee"), ("it", "Dundee"), ("ja", "ダンディー"), ("ka", "დანდი"), ("kk", "Данди"), ("kn", "ದಂಡೀ"), ("ko", "던디"), ("ky", "Данди"), ("lt", "Dandi"), ("lv", "Dandī"), ("mk", "Данди"), ("mr", "ड\u{902}डी"), ("ms", "Dundee"), ("nb", "Dundee"), ("nl", "Dundee"), ("no", "Dundee"), ("pl", "Dundee"), ("pt", "Dundee"), ("ro", "Dundee"), ("ru", "Данди"), ("si", "ඩන\u{dca}ඩ\u{dd2}"), ("sk", "Dundee"), ("sl", "Dundee"), ("sr", "Данди"), ("sr_Latn", "Dandi"), ("sv", "Dundee"), ("ta", "டண\u{bcd}ட\u{bc0}"), ("te", "డండ\u{c40}"), ("th", "ด\u{e31}นด\u{e35}"), ("tr", "Dundee"), ("uk", "Данді"), ("ur", "ڈنڈی، سکاٹ لینڈ"), ("uz", "Dandi"), ("vi", "Dundee"), ("yue", "登地"), ("yue_Hans", "登地"), ("zh", "邓迪")]),
                        unofficial_name_list: ["Dundee City"].to_vec(),
                    }
                ),
                (
                    "DOR",
                    Subdivision{
                        name: "Dorset",
                        country_alpha2: Alpha2::GB,
                        code: "DOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.7390661), longitude: Some(-2.3382346), max_latitude: Some(51.0809977), min_latitude: Some(50.5130724), max_longitude: Some(-1.6816633), min_longitude: Some(-2.9615945)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Dorset"), ("ar", "دورست"), ("be", "Дорсет"), ("bg", "Дорсет"), ("bn", "ডরসেট"), ("ca", "Dorset"), ("ccp", "𑄓\u{11127}𑄢\u{11134}𑄥𑄬𑄖\u{11134}"), ("ceb", "Dorset (kondado sa Hiniusang Gingharian)"), ("cs", "Dorset"), ("cy", "Dorset"), ("da", "Dorset"), ("de", "Dorset"), ("el", "Ντόρσετ"), ("en", "Dorset"), ("es", "Dorset"), ("et", "Dorset"), ("eu", "Dorset"), ("fa", "دورست (انگلستان)"), ("fi", "Dorset"), ("fr", "Dorset"), ("gl", "Dorset"), ("gu", "ડોરસ\u{ac7}ટ"), ("he", "דורסט"), ("hi", "डॉर\u{94d}स\u{947}ट"), ("hu", "Dorset"), ("hy", "Դորսետ"), ("id", "Dorset"), ("is", "Dorset"), ("it", "Dorset"), ("ja", "ドーセット"), ("ka", "დორსეტი"), ("kn", "ಡಾರ\u{ccd}ಸ\u{cc6}ಟ\u{ccd}"), ("ko", "도싯 주"), ("lt", "Dorsetas"), ("lv", "Dorseta"), ("mk", "Дорсет"), ("ml", "ഡോർസെറ\u{d4d}റ\u{d4d}"), ("mr", "डॉर\u{94d}स\u{947}ट"), ("nb", "Dorset"), ("nl", "Dorset"), ("no", "Dorset"), ("pl", "Dorset"), ("pt", "Dorset"), ("ro", "Dorset"), ("ru", "Дорсет"), ("sk", "Dorset"), ("sl", "Dorset"), ("sr", "Дорсет"), ("sr_Latn", "Dorset"), ("sv", "Dorset"), ("ta", "டோர\u{bcd}செட\u{bcd}"), ("te", "డ\u{c4b}ర\u{c4d} స\u{c46}ట\u{c4d}"), ("th", "ดอร\u{e4c}เซต"), ("tr", "Dorset"), ("uk", "Дорсет"), ("ur", "ڈورسٹ"), ("vi", "Dorset"), ("yue", "多實郡"), ("yue_Hans", "多实郡"), ("zh", "多塞特郡")]),
                        unofficial_name_list: ["Dorset"].to_vec(),
                    }
                ),
                (
                    "DRS",
                    Subdivision{
                        name: "Derry and Strabane",
                        country_alpha2: Alpha2::GB,
                        code: "DRS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓𑄬𑄢\u{11128} 𑄃\u{11133}𑄃 𑄌\u{11133}𑄑\u{11133}𑄢\u{11133}𑄢\u{11133}𑄢\u{11127}𑄝𑄚\u{11128}"), ("de", "Derry and Strabane"), ("en", "Derry and Strabane"), ("es", "Distrito de Derry y Strabane"), ("fa", "دری و استرابن"), ("fr", "Derry and Strabane"), ("it", "Distretto di Derry e Strabane"), ("ja", "デリー・シティ・アンド・ストラバン"), ("nl", "Derry and Strabane"), ("ru", "Дерри-сити и Страбэйн"), ("ur", "ڈیری اور سٹربین"), ("zh", "德里城和斯特拉班")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DUD",
                    Subdivision{
                        name: "Dudley",
                        country_alpha2: Alpha2::GB,
                        code: "DUD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.512255), longitude: Some(-2.081112), max_latitude: Some(52.5581732), min_latitude: Some(52.4735445), max_longitude: Some(-2.0571865), min_longitude: Some(-2.1567844)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄓\u{1112a}𑄖\u{11134}𑄣𑄬"), ("ceb", "Dudley"), ("de", "Metropolitan Borough of Dudley"), ("en", "Dudley"), ("fa", "کلان\u{200c}شهر مستقل دادلی"), ("fr", "district métropolitain de Dudley"), ("it", "Dudley"), ("ja", "メトロポリタン・バラ・オブ・ダドリー"), ("ko", "더들리 도시 자치구"), ("nb", "Dudley"), ("nl", "Dudley"), ("no", "Dudley"), ("pl", "Metropolitan Borough of Dudley"), ("ru", "Дадли"), ("sv", "Dudley"), ("ur", "میٹروپولیٹن بورو ڈڈلی"), ("zh", "達德利區")]),
                        unofficial_name_list: ["Dudley"].to_vec(),
                    }
                ),
                (
                    "DUR",
                    Subdivision{
                        name: "Durham",
                        country_alpha2: Alpha2::GB,
                        code: "DUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.77525), longitude: Some(-1.584852), max_latitude: Some(54.827611), min_latitude: Some(54.7358989), max_longitude: Some(-1.4974256), min_longitude: Some(-1.6070484)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Durham-graafskap"), ("ar", "دورهام"), ("be", "Графства Дарэм"), ("bg", "Дърам"), ("bn", "ক\u{9be}উন\u{9cd}টি ড\u{9be}রহ\u{9be}ম"), ("ca", "Comtat de Durham"), ("ccp", "𑄓\u{1112a}𑄢\u{11134}𑄦𑄟\u{11134}"), ("ceb", "County Durham"), ("cs", "Hrabství Durham"), ("cy", "Swydd Durham"), ("da", "County Durham"), ("de", "Durham"), ("en", "Durham"), ("es", "Durham"), ("et", "Durhami krahvkond"), ("eu", "Durham"), ("fa", "کانتی دورهام"), ("fi", "County Durham"), ("fr", "Durham"), ("gl", "Condado de Durham"), ("gu", "કાઉન\u{acd}ટી ડરહામ"), ("he", "דרהאם"), ("hi", "काउ\u{902}टी डरहम"), ("hu", "Durham"), ("hy", "Դարհեմ"), ("id", "County Durham"), ("is", "Durham-sýsla"), ("it", "Durham"), ("ja", "ダラム"), ("kn", "ಕ\u{ccc}ಂಟ\u{cbf} ಡರ\u{ccd}ಹಾಮ\u{ccd}"), ("ko", "더럼 주"), ("lt", "Daramas"), ("lv", "Daremas grāfiste"), ("mr", "ड\u{94d}य\u{941}र\u{945}म"), ("nb", "Durham"), ("nl", "Durham"), ("no", "Durham"), ("pl", "Durham"), ("pt", "Durham"), ("ro", "Durham"), ("ru", "Дарем"), ("sk", "Durham"), ("sl", "Durham"), ("sr", "Дарам"), ("sr_Latn", "Daram"), ("sv", "Durham"), ("ta", "கவுண\u{bcd}டி டர\u{bcd}ஹ\u{bbe}ம\u{bcd}"), ("te", "క\u{c4c}ంట\u{c40} దుర\u{c4d}హం"), ("th", "เดอร\u{e31}ม"), ("tr", "Durham Kontluğu"), ("uk", "Дарем"), ("ur", "کاؤنٹی ڈرہم"), ("vi", "Durham"), ("zh", "達勒姆郡")]),
                        unofficial_name_list: ["Durham"].to_vec(),
                    }
                ),
                (
                    "EAL",
                    Subdivision{
                        name: "Ealing",
                        country_alpha2: Alpha2::GB,
                        code: "EAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5250257), longitude: Some(-0.3415002), max_latitude: Some(51.5596836), min_latitude: Some(51.4904703), max_longitude: Some(-0.2450868), min_longitude: Some(-0.4196215)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إيلنغ"), ("be", "Бора Ілінг"), ("bg", "Еалинг"), ("ca", "Ealing"), ("ccp", "𑄃\u{11128}𑄠𑄢\u{11134}𑄣\u{11128}\u{11101}"), ("ceb", "Ealing"), ("cs", "Ealing"), ("cy", "Ealing"), ("da", "Ealing"), ("de", "London Borough of Ealing"), ("en", "Ealing"), ("es", "Municipio de Ealing"), ("et", "Ealingi linnaosa"), ("eu", "Ealing"), ("fa", "منطقه ایلینگ لندن"), ("fi", "Ealing"), ("fr", "district londonien d’Ealing"), ("ga", "Buirg Londan Ealing"), ("he", "אילינג"), ("hi", "ईलि\u{902}ग बरो"), ("hu", "Ealing kerület"), ("is", "Ealing"), ("ja", "イーリング・ロンドン特別区"), ("ko", "일링 구"), ("nb", "Ealing"), ("nl", "Ealing"), ("no", "Ealing"), ("pl", "London Borough of Ealing"), ("pt", "Ealing"), ("ro", "Ealing"), ("ru", "Илинг"), ("sl", "Ealing"), ("sr", "Лондонска општина Илинг"), ("sr_Latn", "Londonska opština Iling"), ("sv", "London Borough of Ealing"), ("tr", "Ealing"), ("uk", "Ілінг"), ("ur", "ایلنگ بورو"), ("vi", "Khu Ealing của Luân Đôn"), ("zh", "伊靈區")]),
                        unofficial_name_list: ["Ealing"].to_vec(),
                    }
                ),
                (
                    "EAY",
                    Subdivision{
                        name: "East Ayrshire",
                        country_alpha2: Alpha2::GB,
                        code: "EAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.45923149999999), longitude: Some(-4.3338032), max_latitude: Some(55.7637845), min_latitude: Some(55.13805139999999), max_longitude: Some(-3.9570569), min_longitude: Some(-4.6386119)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Усходні Эйршыр"), ("bg", "Източен Еършър"), ("bn", "ইস\u{9cd}ট আরশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "East Ayrshire"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄃𑄠𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "East Ayrshire"), ("cs", "Východní Ayrshire"), ("cy", "Dwyrain Swydd Ayr"), ("de", "East Ayrshire"), ("el", "Ανατολικό Άυρσαιρ"), ("en", "East Ayrshire"), ("es", "East Ayrshire"), ("et", "East Ayrshire"), ("eu", "Ekialdeko Ayrshire"), ("fa", "ایرشر شرقی"), ("fi", "Itä-Ayrshire"), ("fr", "East Ayrshire"), ("ga", "Comhairle Shiorrachd Áir an Ear"), ("gu", "પ\u{ac2}ર\u{acd}વ આયરશાયર"), ("hr", "Istočni Ayrshire"), ("hu", "East Ayrshire"), ("hy", "Արևելյան Էյրշիր"), ("id", "East Ayrshire"), ("it", "Ayrshire Orientale"), ("ja", "イースト・エアシャー"), ("kk", "Ист-Эршир"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಐರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "이스트에어셔"), ("lt", "Rytų Eršyras"), ("nb", "East Ayrshire"), ("nl", "East Ayrshire"), ("no", "East Ayrshire"), ("pl", "East Ayrshire"), ("pt", "East Ayrshire"), ("ro", "East Ayrshire"), ("ru", "Восточный Эйршир"), ("sv", "East Ayrshire"), ("ta", "கிழக\u{bcd}கு அயர\u{bcd}ஸிர\u{bcd}"), ("te", "తూర\u{c4d}పు అయ\u{c3f}ర\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Східний Ершир"), ("ur", "مشرقی آئرشائر"), ("zh", "东艾尔郡")]),
                        unofficial_name_list: ["East Ayrshire"].to_vec(),
                    }
                ),
                (
                    "EDH",
                    Subdivision{
                        name: "Edinburgh, City of",
                        country_alpha2: Alpha2::GB,
                        code: "EDH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.953252), longitude: Some(-3.188267), max_latitude: Some(55.9917083), min_latitude: Some(55.8904228), max_longitude: Some(-3.0777484), min_longitude: Some(-3.3330187)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Edinburg"), ("am", "ኤዲንቦሮ"), ("ar", "إدنبرة"), ("az", "Edinburq"), ("be", "Эдынбург"), ("bg", "Единбург"), ("bn", "এডিনবর\u{9be}"), ("bs", "Edinburgh"), ("ca", "Edimburg"), ("ccp", "𑄃𑄬𑄓𑄚\u{11134}𑄝\u{1112a}𑄢𑄦\u{11134}"), ("ceb", "Edinburgh (kapital sa apil sa nasod)"), ("cs", "Edinburgh"), ("cy", "Caeredin"), ("da", "Edinburgh"), ("de", "Edinburgh"), ("el", "Εδιμβούργο"), ("en", "Edinburgh"), ("es", "Edimburgo"), ("et", "Edinburgh"), ("eu", "Edinburgh"), ("fa", "ادینبرو"), ("fi", "Edinburgh"), ("fr", "Édimbourg"), ("ga", "Dún Éideann"), ("gl", "Edimburgo"), ("gu", "એડિનબર\u{acd}ગ"), ("he", "אדינבורו"), ("hi", "एडिनबरा"), ("hr", "Edinburgh"), ("hu", "Edinburgh"), ("hy", "Էդինբուրգ"), ("id", "Edinburgh"), ("is", "Edinborg"), ("it", "Edimburgo"), ("ja", "エディンバラ"), ("ka", "ედინბურგი"), ("kk", "Эдинбург"), ("kn", "ಎಡ\u{cbf}ನ\u{ccd}\u{200c}ಬರ\u{ccd}ಗ\u{ccd}\u{200c}\u{200c}"), ("ko", "에든버러"), ("lo", "ເອແດມບວກ"), ("lt", "Edinburgas"), ("lv", "Edinburga"), ("mk", "Единбург"), ("ml", "എഡിൻബറോ"), ("mn", "Эдинбург"), ("mr", "एडिनबरा"), ("ms", "Edinburgh"), ("my", "အက\u{103a}ဒင\u{103a}ဗာရာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Edinburgh"), ("nl", "Edinburgh"), ("no", "Edinburgh"), ("pa", "ਐਡਿਨਬਰਾ"), ("pl", "Edynburg"), ("pt", "Edimburgo"), ("ro", "Edinburgh"), ("ru", "Эдинбург"), ("sd", "ايڊنبرگ"), ("si", "එඩ\u{dd2}න\u{dca}බරෝ"), ("sk", "Edinburgh"), ("sl", "Edinburg"), ("sq", "Edinburg"), ("sr", "Единбург"), ("sr_Latn", "Edinburg"), ("sv", "Edinburgh"), ("sw", "Edinburgh"), ("ta", "எடின\u{bcd}பரோ"), ("te", "ఎడ\u{c3f}న\u{c4d}బర\u{c4d}గ\u{c4d}"), ("th", "เอด\u{e34}นบะระ"), ("tk", "Edinburg"), ("tr", "Edinburgh"), ("uk", "Единбург"), ("ur", "ایڈنبرا"), ("uz", "Edinburg"), ("vi", "Edinburgh"), ("yue", "愛丁堡"), ("yue_Hans", "爱丁堡"), ("zh", "爱丁堡")]),
                        unofficial_name_list: ["City of Edinburgh"].to_vec(),
                    }
                ),
                (
                    "EDU",
                    Subdivision{
                        name: "East Dunbartonshire",
                        country_alpha2: Alpha2::GB,
                        code: "EDU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.9755216), longitude: Some(-4.2105149), max_latitude: Some(56.0302966), min_latitude: Some(55.89608879999999), max_longitude: Some(-4.046871299999999), min_longitude: Some(-4.4020525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شرق دونبارتونشاير"), ("bg", "Източен Дънбартъншър"), ("bn", "ইস\u{9cd}ট ড\u{9be}নব\u{9be}র\u{9cd}টনশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "East Dunbartonshire"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄓\u{1112a}𑄚\u{11134}𑄝𑄢\u{11134}𑄑\u{1112a}𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "East Dunbartonshire"), ("cs", "East Dunbartonshire"), ("cy", "Dwyrain Swydd Dunbarton"), ("da", "East Dunbartonshire"), ("de", "East Dunbartonshire"), ("en", "East Dunbartonshire"), ("es", "East Dunbartonshire"), ("et", "East Dunbartonshire"), ("eu", "Ekialdeko Dunbartonshire"), ("fa", "دونبارتونشر شرقی"), ("fi", "Itä-Dunbartonshire"), ("fr", "East Dunbartonshire"), ("ga", "Siorrachd Dhún Bhreatainn an Ear"), ("gu", "પ\u{ac2}ર\u{acd}વ ડનબર\u{acd}ટનશાયર"), ("id", "East Dunbartonshire"), ("it", "Dunbartonshire Orientale"), ("ja", "イースト・ダンバートンシャー"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಡನ\u{ccd}ಬಾರ\u{ccd}ಟನ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "이스트던바턴셔"), ("lt", "Rytų Danbartonšyras"), ("nb", "East Dunbartonshire"), ("nl", "East Dunbartonshire"), ("no", "East Dunbartonshire"), ("pl", "East Dunbartonshire"), ("pt", "East Dunbartonshire"), ("ro", "East Dunbartonshire"), ("ru", "Восточный Дамбартоншир"), ("sv", "East Dunbartonshire"), ("ta", "கிழக\u{bcd}கு ட\u{bbe}ன\u{bcd}ப\u{bbe}ர\u{bcd}டோன\u{bcd}ஷிர\u{bcd}"), ("te", "తూర\u{c4d}పు డన\u{c4d}బర\u{c4d}టన\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Східний Данбартоншир"), ("ur", "ایسٹ دونبارٹنشائر"), ("zh", "東鄧巴頓郡")]),
                        unofficial_name_list: ["East Dunbartonshire"].to_vec(),
                    }
                ),
                (
                    "ELN",
                    Subdivision{
                        name: "East Lothian",
                        country_alpha2: Alpha2::GB,
                        code: "ELN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.9493383), longitude: Some(-2.7704464), max_latitude: Some(56.079289), min_latitude: Some(55.8173207), max_longitude: Some(-2.3642351), min_longitude: Some(-3.0892803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لوثيان الشرقية"), ("be", "Усходні Лотыян"), ("bg", "Източен Лоудиън"), ("bn", "ইস\u{9cd}ট লোথিয\u{9bc}\u{9be}ন"), ("ca", "East Lothian"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄣\u{1112e}𑄑\u{11128}𑄠𑄚\u{11134}"), ("ceb", "East Lothian"), ("cs", "East Lothian"), ("cy", "Dwyrain Lothian"), ("da", "East Lothian"), ("de", "East Lothian"), ("en", "East Lothian"), ("es", "East Lothian"), ("et", "East Lothian"), ("eu", "Ekialdeko Lothian"), ("fa", "لوتیان شرقی"), ("fi", "Itä-Lothian"), ("fr", "East Lothian"), ("ga", "Lodainn an Ear"), ("gu", "પ\u{ac2}ર\u{acd}વ લોથીયાન"), ("it", "East Lothian"), ("ja", "イースト・ロージアン"), ("kk", "Ист-Лотиан"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಲೋಥ\u{cbf}ಯನ\u{ccd}"), ("ko", "이스트로디언"), ("lt", "Rytų Lotianas"), ("nb", "East Lothian"), ("nl", "East Lothian"), ("no", "East Lothian"), ("pl", "East Lothian"), ("pt", "East Lothian"), ("ro", "East Lothian"), ("ru", "Восточный Лотиан"), ("sv", "East Lothian"), ("ta", "கிழக\u{bcd}கு லோதியன\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ ల\u{c4b}థ\u{c3f}యన\u{c4d}"), ("uk", "Східний Лотіан"), ("ur", "ایسٹ لوتھیان"), ("zh", "东洛锡安")]),
                        unofficial_name_list: ["East Lothian"].to_vec(),
                    }
                ),
                (
                    "ELS",
                    Subdivision{
                        name: "Eilean Siar",
                        country_alpha2: Alpha2::GB,
                        code: "ELS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.76), longitude: Some(-7.02), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Buite-Hebride"), ("ar", "هبرديس الخارجي"), ("az", "Xarici Hebrid adaları"), ("be", "Знешнія Гебрыдскія астравы"), ("bg", "Западни острови"), ("ca", "Hèbrides Exteriors"), ("ccp", "𑄃\u{1112f}𑄑𑄢\u{11134} 𑄦𑄝\u{11133}𑄢\u{1112d}𑄖\u{11134}𑄌\u{11134}"), ("ceb", "Outer Hebrides"), ("cs", "Vnější Hebridy"), ("cy", "Ynysoedd Allanol Heledd"), ("da", "Ydre Hebrider"), ("de", "Äußere Hebriden"), ("de_CH", "Äussere Hebriden"), ("en", "Outer Hebrides"), ("es", "Islas Hébridas Exteriores"), ("et", "Välis-Hebriidid"), ("eu", "Kanpoko Hebridak"), ("fa", "مجمع\u{200c}الجزایر هیبرید دوردست"), ("fi", "Ulko-Hebridit"), ("fr", "Hébrides extérieures"), ("ga", "Na hOileáin Siar"), ("gl", "Hébridas de Fóra"), ("hr", "Vanjski Hebridi"), ("id", "Hebrides Luar"), ("is", "Ytri Suðureyjar"), ("it", "Ebridi Esterne"), ("ja", "アウター・ヘブリディーズ"), ("ko", "아우터헤브리디스"), ("lt", "Išoriniai Hebridai"), ("mk", "Надворешни Хебриди"), ("nb", "Ytre Hebridene"), ("nl", "Buiten-Hebriden"), ("no", "Ytre Hebridene"), ("pl", "Hebrydy Zewnętrzne"), ("pt", "Hébridas Exteriores"), ("ro", "Hebridele exterioare"), ("ru", "Внешние Гебриды"), ("sk", "Vonkajšie Hebridy"), ("sr", "Спољни Хебриди"), ("sr_Latn", "Spoljni Hebridi"), ("sv", "Yttre Hebriderna"), ("tr", "Dış Hebridler"), ("uk", "Західні острови"), ("zh", "埃利安锡尔")]),
                        unofficial_name_list: ["Western Isles"].to_vec(),
                    }
                ),
                (
                    "ENF",
                    Subdivision{
                        name: "Enfield",
                        country_alpha2: Alpha2::GB,
                        code: "ENF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.6522994), longitude: Some(-0.0807119), max_latitude: Some(51.6937545), min_latitude: Some(51.6337413), max_longitude: Some(-0.0067053), min_longitude: Some(-0.153915)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة أنفيلد"), ("be", "Энфілд"), ("bn", "এনফিল\u{9cd}ড"), ("ca", "Enfield"), ("ccp", "𑄃𑄬𑄚\u{11134}𑄜\u{11128}𑄣\u{11133}𑄓\u{11134}"), ("ceb", "Enfield (distrito)"), ("cs", "Enfield"), ("cy", "Enfield"), ("da", "Enfield"), ("de", "London Borough of Enfield"), ("en", "Enfield"), ("es", "Enfield"), ("et", "Enfieldi linnaosa"), ("eu", "Enfield"), ("fa", "منطقه انفیلد لندن"), ("fi", "Enfield"), ("fr", "district londonien d’Enfield"), ("ga", "London Borough of Enfield"), ("gu", "લ\u{a82}ડન બોરો ઓફ એનફિલ\u{acd}ડ"), ("he", "אנפילד"), ("hi", "एनफ\u{93c}ील\u{94d}ड बरो"), ("hu", "Enfield kerület"), ("hy", "Էնֆիլդ"), ("is", "Enfield"), ("it", "Borgo londinese di Enfield"), ("ja", "インフィールド・ロンドン特別区"), ("ka", "ენფილდი"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಎನ\u{ccd}ಫೀಲ\u{ccd}ಡ\u{ccd}"), ("ko", "엔필드 구"), ("nb", "Enfield"), ("nl", "Enfield"), ("no", "Enfield"), ("pl", "London Borough of Enfield"), ("pt", "Enfield"), ("ro", "Enfield"), ("ru", "Энфилд"), ("sl", "London Borough of Enfield"), ("sr", "Лондонска општина Енфилд"), ("sr_Latn", "Londonska opština Enfild"), ("sv", "London Borough of Enfield"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} என\u{bcd}ப\u{bc0}ல\u{bcd}ட\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b} ఆఫ\u{c4d} ఎన\u{c4d}ఫ\u{c40}ల\u{c4d}డ\u{c4d}"), ("tr", "Enfield, Londra"), ("uk", "Енфілд"), ("ur", "اینفیلڈ بورو"), ("vi", "Khu Enfield của Luân Đôn"), ("zh", "恩菲爾德區")]),
                        unofficial_name_list: ["Enfield"].to_vec(),
                    }
                ),
                (
                    "ENG",
                    Subdivision{
                        name: "England",
                        country_alpha2: Alpha2::GB,
                        code: "ENG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Engeland"), ("am", "እንግሊዝ"), ("ar", "إنجلترا"), ("as", "ইংলেণ\u{9cd}ড"), ("az", "İngiltərə"), ("be", "Англія"), ("bg", "Англия"), ("bn", "ইংল\u{9cd}য\u{9be}ণ\u{9cd}ড"), ("ca", "Anglaterra"), ("cs", "Anglie"), ("da", "England"), ("de", "England"), ("el", "Αγγλία"), ("en", "England"), ("es", "Inglaterra"), ("et", "Inglismaa"), ("eu", "Ingalaterra"), ("fa", "انگلستان"), ("fi", "Englanti"), ("fil", "England"), ("fr", "Angleterre"), ("gl", "Inglaterra"), ("gu", "ઈ\u{a82}ગ\u{acd}લ\u{ac7}ન\u{acd}ડ"), ("he", "אנגליה"), ("hi", "इ\u{902}ग\u{94d}ल\u{948}\u{902}ड"), ("hr", "Engleska"), ("hu", "Anglia"), ("hy", "Անգլիա"), ("id", "Inggris"), ("is", "England"), ("it", "Inghilterra"), ("ja", "イングランド"), ("ka", "ინგლისი"), ("km", "អង\u{17cb}គ\u{17d2}លេស"), ("kn", "ಇಂಗ\u{ccd}ಲ\u{cc6}ಂಡ\u{ccd}\u{200c}"), ("ko", "잉글랜드"), ("lo", "ອ\u{eb1}ງກ\u{eb4}ດ"), ("lt", "Anglija"), ("lv", "Anglija"), ("ml", "ഇംഗ\u{d4d}ലണ\u{d4d}ട\u{d4d}"), ("mn", "Англи"), ("mr", "इ\u{902}ग\u{94d}ल\u{902}ड"), ("ms", "England"), ("nb", "England"), ("ne", "इ\u{902}ग\u{94d}ल\u{94d}याण\u{94d}ड"), ("nl", "Engeland"), ("or", "ଇଂଲଣ\u{b4d}ଡ"), ("pl", "Anglia"), ("ps", "انګلېنډ"), ("pt", "Inglaterra"), ("ro", "Anglia"), ("ru", "Англия"), ("sd", "انگلينڊ"), ("si", "එංගලන\u{dca}තය"), ("sk", "Anglicko"), ("sl", "Anglija"), ("sr", "Енглеска"), ("sv", "England"), ("sw", "Uingereza"), ("ta", "இங\u{bcd}கில\u{bbe}ந\u{bcd}து"), ("te", "ఇంగ\u{c4d}లండ\u{c4d}"), ("th", "อ\u{e31}งกฤษ"), ("tk", "Angliýa"), ("tr", "İngiltere"), ("uk", "Англія"), ("ur", "انگلینڈ"), ("vi", "Anh Quốc"), ("zh", "英格兰"), ("zh_Hant", "英格蘭")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ERW",
                    Subdivision{
                        name: "East Renfrewshire",
                        country_alpha2: Alpha2::GB,
                        code: "ERW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.74765), longitude: Some(-4.351420399999999), max_latitude: Some(55.8231525), min_latitude: Some(55.67579569999999), max_longitude: Some(-4.2187585), min_longitude: Some(-4.5509528)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Източен Ренфрушър"), ("ca", "East Renfrewshire"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄢𑄬𑄚\u{11134}𑄜\u{11133}𑄢𑄬𑄥𑄠𑄢\u{11134}"), ("ceb", "East Renfrewshire"), ("cs", "East Renfrewshire"), ("cy", "Dwyrain Swydd Renfrew"), ("da", "East Renfrewshire"), ("de", "East Renfrewshire"), ("en", "East Renfrewshire"), ("es", "East Renfrewshire"), ("et", "East Renfrewshire"), ("eu", "Ekialdeko Renfrewshire"), ("fa", "رنفروشر شرقی"), ("fi", "Itä-Renfrewshire"), ("fr", "East Renfrewshire"), ("ga", "Comhairle Shiorrachd Rinn Friú an Ear"), ("it", "Renfrewshire Orientale"), ("ja", "イースト・レンフルーシャー"), ("ko", "이스트렌프루셔"), ("lt", "Rytų Renfrušyras"), ("nb", "East Renfrewshire"), ("nl", "East Renfrewshire"), ("no", "East Renfrewshire"), ("pl", "East Renfrewshire"), ("pt", "East Renfrewshire"), ("ro", "East Renfrewshire"), ("ru", "Восточный Ренфрушир"), ("sv", "East Renfrewshire"), ("uk", "Східний Ренфрюшир"), ("zh", "东伦弗鲁郡")]),
                        unofficial_name_list: ["East Renfrewshire"].to_vec(),
                    }
                ),
                (
                    "ERY",
                    Subdivision{
                        name: "East Riding of Yorkshire",
                        country_alpha2: Alpha2::GB,
                        code: "ERY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.8215889), longitude: Some(-0.7189977), max_latitude: Some(54.1765127), min_latitude: Some(53.57231489999999), max_longitude: Some(0.1478172), min_longitude: Some(-1.1035673)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oos-Yorkshire"), ("ar", "إيست رايدينج أوف يوركشير"), ("be", "Усходні райдынг Ёркшыра"), ("bg", "Източен Йоркшър"), ("bn", "ইয\u{9bc}র\u{9cd}কশ\u{9be}য\u{9bc}\u{9be}র ইস\u{9cd}ট র\u{9be}ইডিং"), ("ca", "East Riding de Yorkshire"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄢\u{1112d}𑄓\u{11128}\u{11101} 𑄃\u{11127}𑄛\u{11134} 𑄃\u{11128}𑄠\u{1112e}𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "East Riding of Yorkshire"), ("cs", "East Riding of Yorkshire"), ("cy", "Riding Dwyreiniol Efrog"), ("da", "East Riding of Yorkshire"), ("de", "East Riding of Yorkshire"), ("en", "East Riding of Yorkshire"), ("es", "Yorkshire del Este"), ("et", "East Riding of Yorkshire"), ("eu", "Ekialdeko Yorkshire"), ("fa", "ریدینگ شرقی یورکشایر"), ("fr", "Yorkshire de l’Est"), ("ga", "East Riding of Yorkshire"), ("gu", "ઇસ\u{acd}ટ રાઇડિ\u{a82}ગ ઓફ યોર\u{acd}કશાયર"), ("he", "מזרח יורקשייר"), ("hi", "ईस\u{94d}ट राइडि\u{902}ग ऑफ\u{93c} यॉर\u{94d}कशायर"), ("hu", "East Riding of Yorkshire"), ("id", "Yorkshire Timur"), ("is", "Austur-Yorkshire"), ("it", "East Riding of Yorkshire"), ("ja", "イースト・ライディング・オブ・ヨークシャー"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ರೈಡ\u{cbf}ಂಗ\u{ccd} ಆಫ\u{ccd} ಯಾರ\u{ccd}ಕ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "이스트라이딩오브요크셔 주"), ("lt", "Rytų Jorkšyras"), ("lv", "Jorkšīras Īstraidinga"), ("mk", "Источен Рајдинг Јоркшир"), ("mr", "ईस\u{94d}ट रायडि\u{902}ग ऑफ यॉर\u{94d}कशायर"), ("nb", "East Riding of Yorkshire"), ("nl", "East Riding of Yorkshire"), ("no", "East Riding of Yorkshire"), ("pl", "East Riding of Yorkshire"), ("pt", "East Riding of Yorkshire"), ("ro", "East Riding of Yorkshire"), ("ru", "Восточный райдинг Йоркшира"), ("sk", "East Riding of Yorkshire"), ("sr", "Источни Рајдинг Јоркшира"), ("sr_Latn", "Istočni Rajding Jorkšira"), ("sv", "East Riding of Yorkshire"), ("ta", "கிழக\u{bcd}கு யோரக\u{bcd}ஷிர\u{bcd} ரைடிங\u{bcd}"), ("te", "తూర\u{c4d}పు ర\u{c48}డ\u{c3f}ంగ\u{c4d} అఫ\u{c4d} య\u{c3e}ర\u{c4d}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("tr", "East Riding Yorkshire"), ("uk", "Східний Йоркширський Райдінг"), ("vi", "East Riding of Yorkshire"), ("yue", "東約郡"), ("yue_Hans", "东约郡"), ("zh", "东约克郡")]),
                        unofficial_name_list: ["East Riding", "East Riding of Yorkshire", "East Yorkshire"].to_vec(),
                    }
                ),
                (
                    "ESS",
                    Subdivision{
                        name: "Essex",
                        country_alpha2: Alpha2::GB,
                        code: "ESS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.76683689999999), longitude: Some(0.4757762), max_latitude: Some(52.09266239999999), min_latitude: Some(51.4510028), max_longitude: Some(1.2965922), min_longitude: Some(-0.0197695)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Essex"), ("ar", "إسكس"), ("be", "Эсэкс"), ("bg", "Есекс"), ("bn", "এসেক\u{9cd}স"), ("ca", "Essex"), ("ccp", "𑄃\u{11128}𑄥𑄬𑄇\u{11134}"), ("ceb", "Essex (kondado)"), ("cs", "Essex"), ("cy", "Essex"), ("da", "Essex"), ("de", "Essex"), ("el", "Έσσεξ"), ("en", "Essex"), ("es", "Essex"), ("et", "Essex"), ("eu", "Essex"), ("fa", "اسکس"), ("fi", "Essex"), ("fr", "Essex"), ("ga", "Essex"), ("gl", "Essex"), ("gu", "એસ\u{ac7}ક\u{acd}સ"), ("he", "אסקס"), ("hi", "एस\u{947}क\u{94d}स"), ("hu", "Essex"), ("hy", "Էսեքս"), ("id", "Essex"), ("is", "Essex"), ("it", "Essex"), ("ja", "エセックス"), ("kn", "ಎಸ\u{ccd}ಸ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}"), ("ko", "에식스 주"), ("lt", "Eseksas"), ("lv", "Eseksa"), ("mk", "Есекс"), ("mr", "एस\u{947}क\u{94d}स"), ("ms", "Essex"), ("nb", "Essex"), ("nl", "Essex"), ("no", "Essex"), ("pl", "Essex"), ("pt", "Essex"), ("ro", "Essex"), ("ru", "Эссекс"), ("sk", "Essex"), ("sl", "Essex"), ("sr", "Есекс"), ("sr_Latn", "Eseks"), ("sv", "Essex"), ("ta", "எஸ\u{bcd}ஸேஸ\u{bcd}"), ("te", "య\u{c46}స\u{c4d}స\u{c46}క\u{c4d}ష\u{c4d}"), ("th", "เอสเซกซ\u{e4c}"), ("tr", "Essex"), ("uk", "Ессекс"), ("ur", "ایسیکس"), ("vi", "Essex"), ("yue", "雅息士郡"), ("yue_Hans", "雅息士郡"), ("zh", "埃塞克斯郡")]),
                        unofficial_name_list: ["Essex"].to_vec(),
                    }
                ),
                (
                    "ESX",
                    Subdivision{
                        name: "East Sussex",
                        country_alpha2: Alpha2::GB,
                        code: "ESX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.9285982), longitude: Some(0.2764899), max_latitude: Some(51.1474019), min_latitude: Some(50.7343442), max_longitude: Some(0.8678558), min_longitude: Some(-0.1358669)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oos-Sussex"), ("ar", "شرق ساسكس"), ("be", "Усходні Сусекс"), ("bg", "Източен Съсекс"), ("bn", "ইস\u{9cd}ট স\u{9be}সেক\u{9cd}স"), ("ca", "East Sussex"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄥𑄥𑄬𑄇\u{11134}"), ("ceb", "East Sussex"), ("cs", "Východní Sussex"), ("cy", "Dwyrain Sussex"), ("da", "East Sussex"), ("de", "East Sussex"), ("el", "Ηστ Σάσσεξ"), ("en", "East Sussex"), ("es", "Sussex Oriental"), ("et", "Ida-Sussex"), ("eu", "Ekialdeko Sussex"), ("fa", "ساسکس شرقی"), ("fi", "East Sussex"), ("fr", "Sussex de l’Est"), ("gl", "East Sussex"), ("gu", "પ\u{ac2}ર\u{acd}વ સ\u{ac1}સ\u{ac7}ક\u{acd}સ"), ("he", "מזרח סאסקס"), ("hi", "ईस\u{94d}ट सस\u{947}क\u{94d}स"), ("hu", "East Sussex"), ("hy", "Արևելյան Սասիքս"), ("id", "East Sussex"), ("is", "Austur-Sussex"), ("it", "East Sussex"), ("ja", "イースト・サセックス"), ("ka", "აღმოსავლეთი სასექსი"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಸಸ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}"), ("ko", "이스트서식스 주"), ("lt", "Rytų Saseksas"), ("lv", "Īstsaseksa"), ("mk", "Источен Сасекс"), ("mr", "ईस\u{94d}ट सस\u{947}क\u{94d}स"), ("nb", "East Sussex"), ("nl", "East Sussex"), ("no", "East Sussex"), ("pl", "East Sussex"), ("pt", "East Sussex"), ("ro", "East Sussex"), ("ru", "Восточный Суссекс"), ("sk", "East Sussex"), ("sl", "Vzhodni Sussex, Anglija"), ("sr", "Источни Сасекс"), ("sr_Latn", "Istočni Saseks"), ("sv", "East Sussex"), ("ta", "கிழக\u{bcd}கு சுஸெஸ\u{bcd}"), ("te", "తూర\u{c4d}పు సస\u{c46}క\u{c4d}స\u{c4d}"), ("th", "อ\u{e35}สต\u{e4c}ซ\u{e31}สเซกซ\u{e4c}"), ("tr", "East Sussex"), ("uk", "Східний Сассекс"), ("ur", "مشرقی سسیکس"), ("vi", "East Sussex"), ("yue", "東修適士"), ("yue_Hans", "东修适士"), ("zh", "东萨塞克斯郡")]),
                        unofficial_name_list: ["East Sussex"].to_vec(),
                    }
                ),
                (
                    "FAL",
                    Subdivision{
                        name: "Falkirk",
                        country_alpha2: Alpha2::GB,
                        code: "FAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.00187750000001), longitude: Some(-3.7839131), max_latitude: Some(56.0267614), min_latitude: Some(55.9830044), max_longitude: Some(-3.7504361), min_longitude: Some(-3.8476074)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Фолкърк"), ("bn", "ফলক\u{9be}র\u{9cd}ক"), ("ccp", "𑄜\u{11127}𑄣\u{11134}𑄇\u{11128}𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Falkirk (dapit sa konseho)"), ("cs", "Falkirk"), ("cy", "Falkirk"), ("da", "Falkirk"), ("de", "Falkirk"), ("en", "Falkirk"), ("es", "Falkirk"), ("et", "Falkirki regioon"), ("eu", "Falkirk"), ("fa", "فالکرک (شهرستان)"), ("fr", "Falkirk"), ("ga", "Comhairle na h-Eaglaise Brice"), ("gu", "ફ\u{ac7}લકિર\u{acd}ક"), ("it", "Falkirk"), ("ja", "フォルカーク"), ("kn", "ಫಾಲ\u{ccd}ಕ\u{cbf}ರ\u{ccd}ಕ\u{ccd}"), ("ko", "폴커크"), ("lt", "Folkerkas"), ("nb", "Falkirk"), ("nl", "Falkirk"), ("no", "Falkirk"), ("pl", "Falkirk"), ("pt", "Falkirk"), ("ro", "Falkirk"), ("ru", "Фолкерк"), ("sv", "Falkirk"), ("ta", "ப\u{bbe}லகிர\u{bcd}க\u{bcd}"), ("te", "ఫ\u{c3e}ల\u{c4d}క\u{c3f}ర\u{c4d}క\u{c4d}"), ("uk", "Фолкерк"), ("ur", "فالکیرک"), ("zh", "福尔柯克")]),
                        unofficial_name_list: ["Falkirk"].to_vec(),
                    }
                ),
                (
                    "FIF",
                    Subdivision{
                        name: "Fife",
                        country_alpha2: Alpha2::GB,
                        code: "FIF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.2082078), longitude: Some(-3.1495175), max_latitude: Some(56.45331530000001), min_latitude: Some(56.0058839), max_longitude: Some(-2.5443315), min_longitude: Some(-3.7399185)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Файф"), ("bg", "Файф"), ("bn", "ফিফে"), ("ca", "Fife"), ("ccp", "𑄜\u{1112d}𑄛\u{11134}"), ("ceb", "Fife"), ("cs", "Fife"), ("cy", "Fife"), ("da", "Fife"), ("de", "Fife"), ("el", "Φάιφ"), ("en", "Fife"), ("es", "Fife"), ("et", "Fife"), ("eu", "Fife"), ("fa", "فایف"), ("fi", "Fife"), ("fr", "Fife"), ("ga", "Foibhe"), ("gu", "ફાઈફ"), ("he", "פיף"), ("hu", "Fife"), ("hy", "Ֆայֆ"), ("is", "Fife"), ("it", "Fife"), ("ja", "ファイフ"), ("kk", "Файф"), ("kn", "ಫೀಫ\u{ccd}"), ("ko", "파이프"), ("lt", "Faifas"), ("mk", "Фајф"), ("nb", "Fife"), ("nl", "Fife"), ("no", "Fife"), ("pl", "Fife"), ("pt", "Fife"), ("ro", "Fife"), ("ru", "Файф"), ("sl", "Fife"), ("sv", "Fife"), ("ta", "பைவ\u{bcd}"), ("te", "ఫ\u{c48}ప\u{c4d}"), ("tr", "Fife"), ("uk", "Файф"), ("ur", "فائف"), ("yue", "快富"), ("yue_Hans", "快富"), ("zh", "法夫")]),
                        unofficial_name_list: ["Fife"].to_vec(),
                    }
                ),
                (
                    "FLN",
                    Subdivision{
                        name: "Flintshire [Sir y Fflint GB-FFL]",
                        country_alpha2: Alpha2::GB,
                        code: "FLN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.197787), longitude: Some(-3.1599793), max_latitude: Some(53.35624079999999), min_latitude: Some(53.0721259), max_longitude: Some(-2.9202759), min_longitude: Some(-3.4005971)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Флінтшыр"), ("bg", "Флинтшър"), ("bn", "স\u{9cd}য\u{9be}র ওয\u{9bc}\u{9be}ই ফ\u{9cd}লিন\u{9cd}ট"), ("ca", "Sir y Fflint"), ("ccp", "𑄜\u{11133}𑄣\u{11128}𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "County of Flintshire"), ("cy", "Sir y Fflint"), ("de", "Flintshire"), ("en", "Flintshire"), ("es", "Flintshire"), ("et", "Flintshire"), ("eu", "Flintshire"), ("fa", "فلینتشر"), ("fi", "Flintshire"), ("fr", "Flintshire"), ("ga", "Sir y Fflint"), ("gl", "Flintshire"), ("gu", "ફ\u{acd}લિ\u{a82}ટશાયર"), ("he", "פלינטשייר"), ("hu", "Flintshire"), ("id", "Flintshire"), ("it", "Flintshire"), ("ja", "フリントシャー"), ("kn", "ಫ\u{ccd}ಲ\u{cbf}ಂಟ\u{ccd}ಸ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "플린트셔"), ("lt", "Flintšyras"), ("nb", "Flintshire"), ("nl", "Flintshire"), ("no", "Flintshire"), ("pl", "Flintshire"), ("pt", "Flintshire"), ("ro", "Flintshire"), ("ru", "Флинтшир"), ("sv", "Flintshire"), ("ta", "பிளிண\u{bcd}ட\u{bcd}ஷிர\u{bcd}"), ("te", "ఫ\u{c4d}ల\u{c3f}ంట\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Флінтшир"), ("ur", "فلینتشائر"), ("vi", "Flintshire"), ("zh", "弗林特郡")]),
                        unofficial_name_list: ["Sir y Fflint"].to_vec(),
                    }
                ),
                (
                    "FMO",
                    Subdivision{
                        name: "Fermanagh and Omagh",
                        country_alpha2: Alpha2::GB,
                        code: "FMO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄜𑄢\u{11134}𑄟𑄚𑄇\u{11134} 𑄃\u{11133}𑄃 𑄃\u{1112e}𑄟𑄇\u{11134}"), ("de", "Fermanagh and Omagh"), ("en", "Fermanagh and Omagh"), ("fa", "فرمانا و اوما"), ("fr", "Fermanagh and Omagh"), ("ga", "Fhear Manach agus na hÓmaí"), ("it", "Distretto di Fermanagh e Omagh"), ("ja", "ファーマナ・アンド・オマー"), ("nl", "Fermanagh and Omagh"), ("ur", "فیرمانہ اور اوما")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GAT",
                    Subdivision{
                        name: "Gateshead",
                        country_alpha2: Alpha2::GB,
                        code: "GAT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.95268), longitude: Some(-1.603411), max_latitude: Some(54.97053469999999), min_latitude: Some(54.91367229999999), max_longitude: Some(-1.516398), min_longitude: Some(-1.661696)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄉𑄬𑄖\u{11134}𑄥𑄬𑄖\u{11134}"), ("ceb", "Gateshead (kondado)"), ("de", "Metropolitan Borough of Gateshead"), ("en", "Gateshead"), ("es", "Municipio de Gateshead"), ("fa", "کلان\u{200c}شهر مستقل گیتس\u{200c}هد"), ("fr", "district métropolitain de Gateshead"), ("it", "Gateshead"), ("ja", "メトロポリタン・バラ・オブ・ゲーツヘッド"), ("ko", "게이츠헤드 도시 자치구"), ("nb", "Gateshead"), ("nl", "Metropolitan Borough of Gateshead"), ("no", "Gateshead"), ("pl", "Metropolitan Borough of Gateshead"), ("ru", "Гейтсхед"), ("sv", "Gateshead (grevskap)"), ("ur", "میٹروپولیٹن بورو گیٹسہیڈ"), ("zh", "蓋茨黑德都市自治市")]),
                        unofficial_name_list: ["Gateshead"].to_vec(),
                    }
                ),
                (
                    "GLG",
                    Subdivision{
                        name: "Glasgow City",
                        country_alpha2: Alpha2::GB,
                        code: "GLG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.864237), longitude: Some(-4.251806), max_latitude: Some(55.9296413), min_latitude: Some(55.7812791), max_longitude: Some(-4.0717167), min_longitude: Some(-4.3932005)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Glasgow"), ("am", "ግላዝጎ"), ("ar", "غلاسكو"), ("az", "Qlazqo"), ("be", "Глазга"), ("bg", "Глазгоу"), ("bn", "গ\u{9cd}ল\u{9be}সগো"), ("bs", "Glasgow"), ("ca", "Glasgow"), ("ccp", "𑄉\u{11133}𑄣𑄌\u{11134}𑄉\u{1112e}"), ("ceb", "Glasgow (kapital sa dapit sa konseho sa Hiniusang Gingharian)"), ("cs", "Glasgow"), ("cy", "Glasgow"), ("da", "Glasgow"), ("de", "Glasgow"), ("el", "Γλασκώβη"), ("en", "Glasgow"), ("es", "Glasgow"), ("et", "Glasgow"), ("eu", "Glasgow"), ("fa", "گلاسگو"), ("fi", "Glasgow"), ("fr", "Glasgow"), ("ga", "Glaschú"), ("gl", "Glasgow"), ("gu", "ગ\u{acd}લાસગો"), ("ha", "Glasgow"), ("ha_NE", "Glasgow"), ("he", "גלאזגו"), ("hi", "ग\u{94d}लासगो"), ("hr", "Glasgow"), ("hu", "Glasgow"), ("hy", "Գլազգո"), ("id", "Glasgow"), ("is", "Glasgow"), ("it", "Glasgow"), ("ja", "グラスゴー"), ("jv", "Glasgow"), ("ka", "გლაზგო"), ("kk", "Глазго"), ("kn", "ಗ\u{ccd}ಲ\u{ccd}ಯಾಸ\u{ccd}ಗೋ"), ("ko", "글래스고"), ("ky", "Глазго"), ("lt", "Glazgas"), ("lv", "Glāzgova"), ("mk", "Глазгов"), ("ml", "ഗ\u{d4d}ല\u{d3e}സ\u{d4d}ഗോ"), ("mr", "ग\u{94d}लासगो"), ("ms", "Glasgow"), ("my", "ဂလပ\u{103a}စဂ\u{102d}\u{102f}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Glasgow"), ("nl", "Glasgow"), ("no", "Glasgow"), ("pa", "ਗਲਾਸਗ\u{a4b}"), ("pl", "Glasgow"), ("pt", "Glasgow"), ("ro", "Glasgow"), ("ru", "Глазго"), ("si", "ග\u{dca}ල\u{dcf}ස\u{dca}ගෝ"), ("sk", "Glasgow"), ("sl", "Glasgow"), ("sq", "Glasgow"), ("sr", "Глазгов"), ("sr_Latn", "Glazgov"), ("sv", "Glasgow"), ("sw", "Glasgow"), ("ta", "கிள\u{bbe}ஸ\u{bcd}கோ"), ("te", "గ\u{c4d}ల\u{c3e}స\u{c4d}గ\u{c4b}"), ("th", "กลาสโกว\u{e4c}"), ("tr", "Glasgow"), ("uk", "Глазго"), ("ur", "گلاسگو"), ("uz", "Glazgo"), ("vi", "Glasgow"), ("yo", "Glasgow"), ("yo_BJ", "Glasgow"), ("yue", "格拉斯哥"), ("yue_Hans", "格拉斯哥"), ("zh", "格拉斯哥")]),
                        unofficial_name_list: ["Glasgow City"].to_vec(),
                    }
                ),
                (
                    "GLS",
                    Subdivision{
                        name: "Gloucestershire",
                        country_alpha2: Alpha2::GB,
                        code: "GLS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8642112), longitude: Some(-2.2380335), max_latitude: Some(52.1125797), min_latitude: Some(51.5775359), max_longitude: Some(-1.615202), min_longitude: Some(-2.6875371)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gloucestershire"), ("ar", "غلوسترشير"), ("az", "Qlosterşir"), ("be", "Глостэршыр"), ("bg", "Глостършър"), ("bn", "গ\u{9cd}লোচেস\u{9cd}ট\u{9be}রশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Gloucestershire"), ("ccp", "𑄉\u{11133}𑄣\u{1112e}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Gloucestershire"), ("cs", "Gloucestershire"), ("cy", "Sir Gaerloyw"), ("da", "Gloucestershire"), ("de", "Gloucestershire"), ("el", "Γκλούσεστερσαϊρ"), ("en", "Gloucestershire"), ("es", "Gloucestershire"), ("et", "Gloucestershire"), ("eu", "Gloucestershire"), ("fa", "گلاسترشایر"), ("fi", "Gloucestershire"), ("fr", "Gloucestershire"), ("ga", "Gloucestershire"), ("gl", "Gloucestershire"), ("gu", "ગ\u{acd}લોસટરશાયર"), ("he", "גלוסטרשייר"), ("hi", "ग\u{94d}लॉस\u{94d}टरशायर"), ("hu", "Gloucestershire"), ("hy", "Գլուսթերշիր"), ("id", "Gloucestershire"), ("is", "Gloucestershire"), ("it", "Gloucestershire"), ("ja", "グロスタシャー"), ("ka", "გლოსტერშირი"), ("kn", "ಗ\u{ccd}ಲ\u{ccc}ಸ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "글로스터셔 주"), ("lt", "Glosteršyras"), ("lv", "Glosteršīra"), ("mk", "Глостершир"), ("mr", "ग\u{94d}लॉस\u{94d}टरशायर"), ("nb", "Gloucestershire"), ("nl", "Gloucestershire"), ("no", "Gloucestershire"), ("pl", "Gloucestershire"), ("pt", "Gloucestershire"), ("ro", "Gloucestershire"), ("ru", "Глостершир"), ("sk", "Gloucestershire"), ("sl", "Gloucestershire"), ("sr", "Глостершир"), ("sr_Latn", "Glosteršir"), ("sv", "Gloucestershire"), ("ta", "க\u{bcd}ளோஸ\u{bcd}ஸ\u{bcd}டேர\u{bcd}ஷிர\u{bcd}"), ("te", "గ\u{c4d}ల\u{c4b}స\u{c4d}టర\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "กลอสเตอร\u{e4c}เชอร\u{e4c}"), ("tr", "Gloucestershire"), ("uk", "Глостершир"), ("ur", "گلوسٹرشائر"), ("vi", "Gloucestershire"), ("yue", "告羅士打郡"), ("yue_Hans", "告罗士打郡"), ("zh", "格洛斯特郡")]),
                        unofficial_name_list: ["Gloucestershire"].to_vec(),
                    }
                ),
                (
                    "GRE",
                    Subdivision{
                        name: "Greenwich",
                        country_alpha2: Alpha2::GB,
                        code: "GRE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.48257659999999), longitude: Some(-0.0076589), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Qrinviç borosu"), ("be", "Грынвіч"), ("ca", "Greenwich"), ("ccp", "𑄉\u{11133}𑄢\u{11128}𑄚\u{11134}𑄃\u{1112a}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Greenwich (distrito)"), ("cs", "Greenwich"), ("cy", "Greenwich"), ("da", "Greenwich"), ("de", "Royal Borough of Greenwich"), ("en", "Greenwich"), ("es", "Municipio de Greenwich"), ("et", "Greenwichi linnaosa"), ("eu", "Greenwich"), ("fa", "منطقه سلطنتی گرینویچ"), ("fr", "district royal de Greenwich"), ("he", "הרובע המלכותי של גריניץ׳"), ("hu", "London Borough of Greenwich"), ("hy", "Գրինվիչ"), ("is", "Greenwich"), ("it", "Borgo reale di Greenwich"), ("ja", "グリニッジ王室特別区"), ("ko", "그리니치 왕립구"), ("mk", "Гринич"), ("mr", "ग\u{94d}रीनिच"), ("nb", "Greenwich"), ("ne", "ग\u{94d}र\u{947}निच"), ("nl", "London Borough of Greenwich"), ("no", "Greenwich"), ("pl", "Royal Borough of Greenwich"), ("pt", "Greenwich"), ("ru", "Гринвич"), ("sk", "Greenwich"), ("sr", "Лондонска општина Гринич"), ("sr_Latn", "Londonska opština Grinič"), ("sv", "Royal Borough of Greenwich"), ("uk", "Гринвіч"), ("ur", "گرینچ کا شاہی بورو"), ("uz", "Grinvich"), ("vi", "Khu Greenwich của Luân Đôn"), ("yue", "格林尼治區"), ("yue_Hans", "格林尼治区"), ("zh", "格林威治區")]),
                        unofficial_name_list: ["Greenwich"].to_vec(),
                    }
                ),
                (
                    "GWN",
                    Subdivision{
                        name: "Gwynedd",
                        country_alpha2: Alpha2::GB,
                        code: "GWN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.8928932), longitude: Some(-3.9958464), max_latitude: Some(53.248342), min_latitude: Some(52.5346386), max_longitude: Some(-3.436765), min_longitude: Some(-4.8043203)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Гвінед"), ("bg", "Гуинед"), ("bn", "গেনেড"), ("ca", "Gwynedd"), ("ccp", "𑄉\u{1112d}𑄚𑄬𑄖\u{11134}"), ("ceb", "Gwynedd"), ("cs", "Gwynedd"), ("cy", "Gwynedd"), ("da", "Gwynedd"), ("de", "Gwynedd"), ("el", "Γκουίνεθ"), ("en", "Gwynedd"), ("es", "Gwynedd"), ("et", "Gwynedd"), ("eu", "Gwynedd"), ("fa", "گویند"), ("fi", "Gwynedd"), ("fr", "Gwynedd"), ("ga", "Gwynedd"), ("gl", "Gwynedd"), ("gu", "ગ\u{acd}વિન\u{ac7}ડ"), ("he", "גוויניד׳"), ("hr", "Gwynedd"), ("hu", "Gwynedd"), ("id", "Gwynedd"), ("it", "Gwynedd"), ("ja", "グウィネズ"), ("kn", "ಗ\u{ccd}ವ\u{cbf}ನ\u{cc6}ಡ\u{ccd}"), ("ko", "귀네드"), ("lt", "Gvinedas"), ("mk", "Гвинед"), ("nb", "Gwynedd"), ("nl", "Gwynedd"), ("no", "Gwynedd"), ("pl", "Gwynedd"), ("pt", "Gwynedd"), ("ro", "Gwynedd"), ("ru", "Гвинед"), ("sv", "Gwynedd"), ("ta", "கினெட\u{bcd}ட\u{bcd}"), ("te", "గ\u{c48}న\u{c46}డ\u{c4d}"), ("th", "กว\u{e34}นเนดด\u{e4c}"), ("uk", "Гвінет"), ("ur", "جوینید"), ("vi", "Gwynedd"), ("zh", "圭内斯")]),
                        unofficial_name_list: ["Gwynedd"].to_vec(),
                    }
                ),
                (
                    "HAL",
                    Subdivision{
                        name: "Halton",
                        country_alpha2: Alpha2::GB,
                        code: "HAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.34902719999999), longitude: Some(-2.7136913), max_latitude: Some(53.402537), min_latitude: Some(53.30502), max_longitude: Some(-2.5952224), min_longitude: Some(-2.8324574)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Халтан"), ("bn", "হ\u{9be}ল\u{9cd}টন"), ("ccp", "𑄦\u{11127}𑄣\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Borough of Halton"), ("de", "Borough of Halton"), ("en", "Halton"), ("es", "Halton"), ("fa", "هالتون"), ("fr", "Halton"), ("gu", "હોલ\u{acd}ટન"), ("hu", "Borough of Halton"), ("it", "Halton"), ("ja", "ハルトン"), ("kn", "ಹ\u{ccd}ಯಾಲ\u{ccd}ಟನ\u{ccd}"), ("ko", "홀턴 구"), ("lt", "Haltonas"), ("nb", "Halton"), ("nl", "Halton"), ("no", "Halton"), ("pl", "Halton"), ("pt", "Halton"), ("ro", "Halton"), ("ru", "Халтон"), ("sk", "Halton"), ("sv", "Borough of Halton"), ("ta", "ஹ\u{bbe}ல\u{bcd}டன\u{bcd}"), ("te", "హ\u{c3e}ల\u{c4d}టన\u{c4d}"), ("uk", "Халтон"), ("ur", "بورو ہالٹن"), ("zh", "霍爾頓")]),
                        unofficial_name_list: ["Halton"].to_vec(),
                    }
                ),
                (
                    "HAM",
                    Subdivision{
                        name: "Hampshire",
                        country_alpha2: Alpha2::GB,
                        code: "HAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.0895203), longitude: Some(-1.216844), max_latitude: Some(51.3839153), min_latitude: Some(50.7060166), max_longitude: Some(-0.7293873), min_longitude: Some(-1.957277)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hampshire"), ("ar", "هامبشاير"), ("be", "Графства Хэмпшыр"), ("bg", "Хампшър"), ("bn", "হ\u{9cd}য\u{9be}ম\u{9cd}পশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Hampshire"), ("ccp", "𑄦𑄟\u{11133}𑄛\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Hampshire (kondado)"), ("cs", "Hampshire"), ("cy", "Hampshire"), ("da", "Hampshire"), ("de", "Hampshire"), ("el", "Χάμπσαϊρ"), ("en", "Hampshire"), ("es", "Hampshire"), ("et", "Hampshire"), ("eu", "Hampshire"), ("fa", "همپشایر"), ("fi", "Hampshire"), ("fr", "Hampshire"), ("ga", "Hampshire"), ("gl", "Hampshire"), ("gu", "હ\u{ac7}મ\u{acd}પશાયર"), ("he", "המפשייר"), ("hi", "ह\u{948}म\u{94d}पशायर"), ("hr", "Hampshire"), ("hu", "Hampshire"), ("hy", "Հեմփշիր"), ("id", "Hampshire"), ("is", "Hampshire"), ("it", "Hampshire"), ("ja", "ハンプシャー"), ("jv", "Hampshire"), ("kn", "ಹ\u{ccd}ಯಾಂಪ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "햄프셔 주"), ("lt", "Hampšyras"), ("lv", "Hempšīra"), ("mk", "Хемпшир"), ("mr", "ह\u{901}पशायर"), ("nb", "Hampshire"), ("nl", "Hampshire"), ("no", "Hampshire"), ("pl", "Hampshire"), ("pt", "Hampshire"), ("ro", "Hampshire"), ("ru", "Гэмпшир"), ("sk", "Hampshire"), ("sl", "Hampshire, Anglija"), ("sr", "Хемпшир"), ("sr_Latn", "Hempšir"), ("sv", "Hampshire"), ("ta", "ஹ\u{bbe}ம\u{bcd}ப\u{bcd}ஷயர\u{bcd}"), ("te", "హంప\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "แฮมป\u{e4c}เชอร\u{e4c}"), ("tr", "Hampshire"), ("uk", "Гемпшир"), ("ur", "ہیمپشائر"), ("vi", "Hampshire"), ("yue", "衡州郡"), ("yue_Hans", "衡州郡"), ("zh", "漢普郡")]),
                        unofficial_name_list: ["Hampshire"].to_vec(),
                    }
                ),
                (
                    "HAV",
                    Subdivision{
                        name: "Havering",
                        country_alpha2: Alpha2::GB,
                        code: "HAV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.577924), longitude: Some(0.2120829), max_latitude: Some(51.6317344), min_latitude: Some(51.48727770000001), max_longitude: Some(0.3339957), min_longitude: Some(0.1381569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "бора Хейверынг"), ("bg", "Хаверинг"), ("bn", "হ\u{9cd}য\u{9be}ভ\u{9be}রিং"), ("ca", "Havering"), ("ccp", "𑄦𑄞𑄬𑄢\u{11128}\u{11101}"), ("ceb", "Havering"), ("cs", "Havering"), ("cy", "Havering"), ("da", "Havering"), ("de", "London Borough of Havering"), ("en", "Havering"), ("es", "Havering"), ("et", "Haveringi linnaosa"), ("eu", "Havering"), ("fa", "منطقه هیورینگ لندن"), ("fi", "Havering"), ("fr", "district londonien d’Havering"), ("ga", "Buirg Londan Havering"), ("gu", "લ\u{a82}ડન બોરો ઓફ હ\u{ac7}વરિ\u{a82}ગ"), ("he", "הברינג"), ("hi", "ह\u{947}वरि\u{902}ग बरो"), ("hu", "Havering kerület"), ("is", "Havering"), ("it", "Havering"), ("ja", "ヘイヴァリング・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಹಾವ\u{cc6}ರ\u{cbf}ಂಗ\u{ccd}"), ("ko", "헤이버링 구"), ("nb", "Havering"), ("nl", "Havering"), ("no", "Havering"), ("pl", "London Borough of Havering"), ("pt", "Havering"), ("ro", "Havering"), ("ru", "Хаверинг"), ("sr", "Лондонска општина Хејверинг"), ("sr_Latn", "Londonska opština Hejvering"), ("sv", "London Borough of Havering"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஹ\u{bbe}வேரிங\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b} ఆఫ\u{c4d} హ\u{c3e}వర\u{c3f}ంగ\u{c4d}"), ("tr", "Havering"), ("uk", "Гейверінг"), ("ur", "ہیورنگ بورو"), ("vi", "Khu Havering của Luân Đôn"), ("zh", "黑弗靈區")]),
                        unofficial_name_list: ["Havering"].to_vec(),
                    }
                ),
                (
                    "HCK",
                    Subdivision{
                        name: "Hackney",
                        country_alpha2: Alpha2::GB,
                        code: "HCK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1563141), longitude: Some(-1.5750216), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "London Borough of Hackney"), ("ar", "حي هكني في لندن"), ("be", "Хакні"), ("ca", "Hackney"), ("ccp", "𑄦𑄇\u{11134}𑄚𑄬"), ("ceb", "Hackney"), ("cs", "Hackney"), ("cy", "Hackney"), ("da", "Hackney"), ("de", "London Borough of Hackney"), ("el", "Χάκνεϊ"), ("en", "Hackney"), ("es", "Hackney"), ("et", "Hackney"), ("eu", "Hackney"), ("fa", "منطقه هکنی لندن"), ("fi", "Hackney"), ("fr", "district londonien de Hackney"), ("ga", "London Borough of Hackney"), ("he", "האקני"), ("hi", "ह\u{948}कनी बरो"), ("hu", "Hackney, London"), ("hy", "Հակնի"), ("is", "Hackney"), ("it", "Borgo londinese di Hackney"), ("ja", "ハックニー・ロンドン特別区"), ("ka", "ჰაკნი"), ("ko", "해크니 구"), ("mk", "Хакни"), ("nb", "Hackney"), ("nl", "Hackney"), ("no", "Hackney"), ("pl", "London Borough of Hackney"), ("pt", "Hackney"), ("ro", "Hackney"), ("ru", "Хакни"), ("sl", "Hackney, London"), ("sr", "Лондоска општина Хекни"), ("sr_Latn", "Londoska opština Hekni"), ("sv", "London Borough of Hackney"), ("tr", "Hackney"), ("uk", "Гекні"), ("ur", "ہیکنی بورو"), ("vi", "Khu Hackney của Luân Đôn"), ("zh", "哈克尼區")]),
                        unofficial_name_list: ["Hackney"].to_vec(),
                    }
                ),
                (
                    "HEF",
                    Subdivision{
                        name: "Herefordshire, County of",
                        country_alpha2: Alpha2::GB,
                        code: "HEF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.0765164), longitude: Some(-2.6544182), max_latitude: Some(52.3954718), min_latitude: Some(51.8259444), max_longitude: Some(-2.3379667), min_longitude: Some(-3.1419149)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Herefordshire"), ("ar", "هيرفوردشير"), ("be", "Графства Херэфардшыр"), ("bg", "Херефордшър"), ("bn", "হেয\u{9bc}\u{9be}রফোর\u{9cd}ডশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Herefordshire"), ("ccp", "𑄦𑄬𑄠𑄢\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Herefordshire"), ("cs", "Herefordshire"), ("cy", "Swydd Henffordd"), ("da", "Herefordshire"), ("de", "Herefordshire"), ("el", "Χέρεφορντσαϊρ"), ("en", "Herefordshire"), ("es", "Herefordshire"), ("et", "Herefordshire"), ("eu", "Herefordshire"), ("fa", "هرفوردشایر"), ("fi", "Herefordshire"), ("fr", "Herefordshire"), ("ga", "Herefordshire"), ("gu", "હિયરફૉર\u{acd}ડશાયર"), ("he", "הרפורדשייר"), ("hi", "हरफ\u{93c}र\u{94d}डशायर"), ("hu", "Herefordshire"), ("hy", "Հերեֆորդշիր"), ("id", "Herefordshire"), ("is", "Herefordshire"), ("it", "Herefordshire"), ("ja", "ヘレフォードシャー"), ("ka", "ჰერეფორდშირი"), ("kn", "ಹ\u{cbf}ಯರ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "헤리퍼드셔 주"), ("lt", "Herefordšyras"), ("lv", "Herefordšīra"), ("mk", "Херефордшир"), ("mr", "हर\u{94d}फर\u{94d}डशायर"), ("nb", "Herefordshire"), ("nl", "Herefordshire"), ("no", "Herefordshire"), ("pl", "Herefordshire"), ("pt", "Herefordshire"), ("ro", "Herefordshire"), ("ru", "Херефордшир"), ("sk", "Herefordshire"), ("sl", "Herefordshire"), ("sr", "Херефордшир"), ("sr_Latn", "Herefordšir"), ("sv", "Herefordshire"), ("ta", "ஹெரேபோர\u{bcd}ட\u{bcd}ஷயர\u{bcd}"), ("te", "హ\u{c46}ర\u{c46} ఫ\u{c4b}ర\u{c4d}డ\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "เฮร\u{e34}ฟอร\u{e4c}ดเชอร\u{e4c}"), ("tr", "Herefordshire"), ("uk", "Герефордшир"), ("ur", "ہیرفورڈشائر"), ("vi", "Herefordshire"), ("yue", "禧福郡"), ("yue_Hans", "禧福郡"), ("zh", "赫里福德郡")]),
                        unofficial_name_list: ["County of Herefordshire"].to_vec(),
                    }
                ),
                (
                    "HIL",
                    Subdivision{
                        name: "Hillingdon",
                        country_alpha2: Alpha2::GB,
                        code: "HIL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5351832), longitude: Some(-0.4481378), max_latitude: Some(51.6317555), min_latitude: Some(51.4532666), max_longitude: Some(-0.3759291), min_longitude: Some(-0.5103751)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ناحية هيلينغدون في لندن"), ("bg", "Хилингдон"), ("bs", "Londonska općina Hillingdon"), ("ca", "Hillingdon"), ("ccp", "𑄦\u{11128}𑄣\u{11128}\u{11101}𑄓\u{11127}𑄚\u{11134}"), ("ceb", "Hillingdon"), ("cs", "Hillingdon"), ("cy", "Hillingdon"), ("da", "Hillingdon"), ("de", "London Borough of Hillingdon"), ("en", "Hillingdon"), ("es", "Municipio de Hillingdon"), ("et", "Hillingdoni linnaosa"), ("eu", "Hillingdon"), ("fa", "منطقه هیلینگدون لندن"), ("fi", "Hillingdon"), ("fr", "district londonien de Hillingdon"), ("ga", "Buirg Londan Hillingdon"), ("gu", "હીલી\u{a82}ગડન બરો"), ("he", "הילינגדון"), ("hi", "हिलि\u{902}गडन बरो"), ("hu", "Hillingdon kerület"), ("hy", "Հիլինգդոն"), ("id", "Hillingdon"), ("is", "Hillingdon"), ("it", "Hillingdon"), ("ja", "ヒリンドン・ロンドン特別区"), ("ko", "힐링던 구"), ("mk", "Хилингдон"), ("mr", "हिलि\u{902}ग\u{94d}डन"), ("nb", "Hillingdon"), ("nl", "Hillingdon"), ("no", "Hillingdon"), ("pl", "London Borough of Hillingdon"), ("pt", "Hillingdon"), ("ro", "Hillingdon"), ("ru", "Хиллингдон"), ("sk", "London Borough of Hillingdon"), ("sl", "London Borough of Hillingdon"), ("sr", "Лондонска општина Хилингдон"), ("sr_Latn", "Londonska opština Hilingdon"), ("sv", "London Borough of Hillingdon"), ("tr", "Hillingdon"), ("uk", "Гіллінгдон"), ("ur", "ھلنگٹن بورو"), ("vi", "Khu Hillingdon của Luân Đôn"), ("zh", "希靈登區")]),
                        unofficial_name_list: ["Hillingdon"].to_vec(),
                    }
                ),
                (
                    "HLD",
                    Subdivision{
                        name: "Highland",
                        country_alpha2: Alpha2::GB,
                        code: "HLD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.3596139), longitude: Some(-5.0992763), max_latitude: Some(58.6973508), min_latitude: Some(56.4971768), max_longitude: Some(-3.0240042), min_longitude: Some(-6.7896386)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هایلند"), ("be", "Хайленд"), ("bg", "Хайланд"), ("bn", "হ\u{9be}ইল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Consell de Highland"), ("ccp", "𑄦\u{1112d}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Highland (dapit sa konseho)"), ("cs", "Highland"), ("cy", "Cyngor yr Ucheldir"), ("de", "Highland"), ("el", "Χάιλαντ"), ("en", "Highland"), ("es", "Highland"), ("et", "Highland"), ("eu", "Highland"), ("fa", "هایلند (شهرستان)"), ("fi", "Ylämaa"), ("fr", "Highland"), ("ga", "Comhairle na Gáidhealtachd"), ("gu", "હાઇલ\u{ac7}ન\u{acd}ડ"), ("hu", "Highland"), ("it", "Highland"), ("ja", "ハイランド"), ("kn", "ಹೈಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "하일랜드"), ("lt", "Hailandas"), ("mk", "Хајленд"), ("ms", "Tanah Tinggi"), ("nb", "Highland"), ("nl", "Highland"), ("no", "Highland"), ("pl", "Highland"), ("pt", "Highland"), ("ro", "Highland"), ("ru", "Хайленд"), ("sv", "Highland"), ("ta", "ஹயிலன\u{bcd}ட\u{bcd}"), ("te", "హ\u{c48}ల\u{c3e}ండ\u{c4d}"), ("uk", "Гайленд"), ("ur", "ہائی لینڈ"), ("zh", "高地")]),
                        unofficial_name_list: ["Highland"].to_vec(),
                    }
                ),
                (
                    "HMF",
                    Subdivision{
                        name: "Hammersmith and Fulham",
                        country_alpha2: Alpha2::GB,
                        code: "HMF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.49901699999999), longitude: Some(-0.2291497), max_latitude: Some(51.5327524), min_latitude: Some(51.4638977), max_longitude: Some(-0.1776272), min_longitude: Some(-0.2550903)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Хамерсміт і Фулем"), ("ca", "Hammersmith i Fulham"), ("ccp", "𑄦\u{11127}𑄟𑄢\u{11134}𑄌\u{11133}𑄟\u{11128}𑄖\u{11134} 𑄃\u{11133}𑄃 𑄜\u{1112a}𑄣\u{11134}𑄦𑄟\u{11134}"), ("ceb", "Hammersmith and Fulham"), ("cs", "Hammersmith a Fulham"), ("cy", "Hammersmith a Fulham"), ("da", "Hammersmith and Fulham"), ("de", "London Borough of Hammersmith and Fulham"), ("en", "Hammersmith and Fulham"), ("es", "Hammersmith y Fulham"), ("eu", "Hammersmith eta Fulham"), ("fa", "منطقه همرسمیت و فولام لندن"), ("fi", "Hammersmith and Fulham"), ("fr", "borough londonien de Hammersmith et Fulham"), ("ga", "Buirg Londan Hammersmith agus Fulham"), ("gl", "Hammersmith e Fulham"), ("he", "האמרסמית׳ ופולהאם"), ("hi", "ह\u{948}मरस\u{94d}मिथ ऐ\u{902}ड फ\u{93c}\u{941}लहम बरो"), ("hu", "London Borough of Hammersmith and Fulham"), ("hy", "Համերսմիթ և Ֆուլեմ"), ("id", "Hammersmith dan Fulham"), ("is", "Hammersmith og Fulham"), ("it", "Hammersmith e Fulham"), ("ja", "ハマースミス・アンド・フラム・ロンドン特別区"), ("ko", "해머스미스 풀럼 구"), ("mk", "Хамерсмит и Фулам"), ("mr", "ह\u{945}मरस\u{94d}मिथ व फ\u{941}लह\u{945}म"), ("nb", "Hammersmith and Fulham"), ("nl", "Hammersmith en Fulham"), ("no", "Hammersmith and Fulham"), ("pl", "London Borough of Hammersmith and Fulham"), ("pt", "Hammersmith e Fulham"), ("ro", "Hammersmith and Fulham"), ("ru", "Хаммерсмит и Фулем"), ("sl", "Hammersmith and Fulham"), ("sr", "Лондонска општина Хамерсмит и Фулам"), ("sr_Latn", "Londonska opština Hamersmit i Fulam"), ("sv", "London Borough of Hammersmith and Fulham"), ("tr", "Hammersmith ve Fulham"), ("uk", "Гаммерсміт і Фулем"), ("ur", "ہیمرسمت اور فلہم بورو"), ("vi", "Khu Hammersmith và Fulham của Luân Đôn"), ("yue", "咸默史密夫及富咸區"), ("yue_Hans", "咸默史密夫及富咸区"), ("zh", "哈默史密斯-富勒姆區")]),
                        unofficial_name_list: ["Hammersmith and Fulham"].to_vec(),
                    }
                ),
                (
                    "HNS",
                    Subdivision{
                        name: "Hounslow",
                        country_alpha2: Alpha2::GB,
                        code: "HNS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.46092179999999), longitude: Some(-0.373149), max_latitude: Some(51.50031509999999), min_latitude: Some(51.4457371), max_longitude: Some(-0.3458572), min_longitude: Some(-0.4168672)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هونسلو"), ("ca", "Hounslow"), ("ccp", "𑄦\u{11127}𑄅\u{1112a}𑄚\u{11134}𑄥\u{11133}𑄣\u{1112e}"), ("ceb", "Hounslow"), ("cs", "Hounslow"), ("cy", "Hounslow"), ("da", "Hounslow"), ("de", "London Borough of Hounslow"), ("en", "Hounslow"), ("es", "Municipio de Hounslow"), ("eu", "Hounslow"), ("fa", "منطقه هانزلو لندن"), ("fi", "Hounslow"), ("fr", "borough londonien de Hounslow"), ("ga", "Buirg Londan Hounslow"), ("he", "האונסלו"), ("hi", "हाउ\u{902}स\u{94d}लो बरो"), ("hu", "Hounslow kerület"), ("hy", "Հաունսլոու"), ("is", "Hounslow"), ("it", "Hounslow"), ("ja", "ハウンズロー・ロンドン特別区"), ("ko", "하운즐로 구"), ("mr", "हाउन\u{94d}स\u{94d}लो"), ("nb", "Hounslow"), ("nl", "Hounslow"), ("no", "Hounslow"), ("pl", "London Borough of Hounslow"), ("ro", "Hounslow"), ("ru", "Хаунслоу"), ("sr", "Лондонска општина Хаунзлов"), ("sr_Latn", "Londonska opština Haunzlov"), ("sv", "London Borough of Hounslow"), ("tr", "Hounslow"), ("uk", "Гаунслоу"), ("ur", "ہونسلو بورو"), ("vi", "Khu Hounslow của Luân Đôn"), ("zh", "豪士羅區")]),
                        unofficial_name_list: ["Hounslow"].to_vec(),
                    }
                ),
                (
                    "HPL",
                    Subdivision{
                        name: "Hartlepool",
                        country_alpha2: Alpha2::GB,
                        code: "HPL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.691745), longitude: Some(-1.212926), max_latitude: Some(54.72333), min_latitude: Some(54.6216585), max_longitude: Some(-1.1577292), min_longitude: Some(-1.2679252)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Хартлпул"), ("bn", "হ\u{9be}র\u{9cd}টলপ\u{9c1}ল"), ("ccp", "𑄦𑄢\u{11134}𑄑\u{11134}𑄣\u{11128}𑄛\u{1112a}𑄣\u{11134}"), ("ceb", "Hartlepool"), ("da", "Hartlepool"), ("de", "Borough of Hartlepool"), ("en", "Hartlepool"), ("es", "Hartlepool"), ("fr", "Hartlepool"), ("gu", "હાર\u{acd}ટલપ\u{ac2}લ"), ("it", "Hartlepool"), ("ja", "ハートリプール"), ("kn", "ಹಾರ\u{ccd}ಟ\u{ccd}ಲ\u{cc6}ಪಲ\u{ccd}"), ("ko", "하틀풀 구"), ("nl", "Hartlepool"), ("pl", "Hartlepool"), ("pt", "Hartlepool"), ("ru", "Хартлпул"), ("sk", "Borough of Hartlepool"), ("sv", "Hartlepool (grevskap)"), ("ta", "ஹ\u{bbe}ர\u{bcd}ட\u{bcd}லெபோல\u{bcd}"), ("te", "హ\u{c3e}ర\u{c4d}ట\u{c4d}ల\u{c3f}పూల\u{c4d}"), ("uk", "Гартлпул"), ("ur", "بورو ہارٹلپول"), ("zh", "哈特爾浦自治市")]),
                        unofficial_name_list: ["Hartlepool"].to_vec(),
                    }
                ),
                (
                    "HRT",
                    Subdivision{
                        name: "Hertfordshire",
                        country_alpha2: Alpha2::GB,
                        code: "HRT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.80978229999999), longitude: Some(-0.2376744), max_latitude: Some(52.08053640000001), min_latitude: Some(51.59961790000001), max_longitude: Some(0.1955669), min_longitude: Some(-0.7457891)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hertfordshire"), ("ar", "هارتفوردشير"), ("az", "Hartfordşir"), ("be", "Хартфардшыр"), ("bg", "Хартфордшър"), ("bn", "হ\u{9be}র\u{9cd}টফোর\u{9cd}ডশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Hertfordshire"), ("ccp", "𑄦𑄢\u{11134}𑄑\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Hertfordshire"), ("cs", "Hertfordshire"), ("cy", "Swydd Hertford"), ("da", "Hertfordshire"), ("de", "Hertfordshire"), ("el", "Χέρτφορντσαϊρ"), ("en", "Hertfordshire"), ("es", "Hertfordshire"), ("et", "Hertfordshire"), ("eu", "Hertfordshire"), ("fa", "هرتفوردشایر"), ("fi", "Hertfordshire"), ("fr", "Hertfordshire"), ("ga", "Hertfordshire"), ("gl", "Hertfordshire"), ("gu", "હર\u{acd}ટફોર\u{acd}ડશાયર"), ("he", "הרטפורדשייר"), ("hi", "हर\u{94d}टफ\u{93c}र\u{94d}डशायर"), ("hu", "Hertfordshire"), ("hy", "Հարթֆորշիր"), ("id", "Hertfordshire"), ("is", "Hertfordshire"), ("it", "Hertfordshire"), ("ja", "ハートフォードシャー"), ("ka", "ჰერტფორდშირი"), ("kn", "ಹರ\u{ccd}ಟ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "하트퍼드셔 주"), ("lt", "Hartfordšyras"), ("lv", "Hārtfordšīra"), ("mk", "Хартфордшир"), ("mr", "हर\u{94d}टफर\u{94d}डशायर"), ("nb", "Hertfordshire"), ("nl", "Hertfordshire"), ("no", "Hertfordshire"), ("pl", "Hertfordshire"), ("pt", "Hertfordshire"), ("ro", "Hertfordshire"), ("ru", "Хартфордшир"), ("sk", "Hertfordshire"), ("sl", "Hertfordshire"), ("sr", "Хартфордшир"), ("sr_Latn", "Hartfordšir"), ("sv", "Hertfordshire"), ("ta", "ஹெர\u{bcd}ட\u{bcd}போர\u{bcd}டஷிர\u{bcd}"), ("te", "హ\u{c46}ర\u{c4d}ట\u{c4d} ఫర\u{c4d}డ\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "ฮาร\u{e4c}ตฟอร\u{e4c}ดเชอร\u{e4c}"), ("tr", "Hertfordshire"), ("uk", "Гартфордшир"), ("ur", "ہارٹفورڈشائر"), ("vi", "Hertfordshire"), ("yue", "赫福郡"), ("yue_Hans", "赫福郡"), ("zh", "赫特福德郡")]),
                        unofficial_name_list: ["Hertfordshire"].to_vec(),
                    }
                ),
                (
                    "HRW",
                    Subdivision{
                        name: "Harrow",
                        country_alpha2: Alpha2::GB,
                        code: "HRW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.580559), longitude: Some(-0.341995), max_latitude: Some(51.6308081), min_latitude: Some(51.5530613), max_longitude: Some(-0.2797453), min_longitude: Some(-0.3864607)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "حي هرو لندن"), ("be", "Хараў"), ("ca", "Harrow"), ("ccp", "𑄦𑄢\u{1112a}"), ("ceb", "Harrow (distrito sa Hiniusang Gingharian)"), ("cs", "Harrow"), ("cy", "Harrow"), ("da", "Harrow"), ("de", "London Borough of Harrow"), ("en", "Harrow"), ("es", "Municipio de Harrow"), ("eu", "Harrow"), ("fa", "منطقه هارو لندن"), ("fi", "Harrow"), ("fr", "district londonien de Harrow"), ("ga", "Buirg Londan Harrow"), ("he", "הארו"), ("hi", "ह\u{948}रो बरो"), ("hu", "Harrow kerület"), ("hy", "Հարրոու"), ("is", "Harrow"), ("it", "Borgo londinese di Harrow"), ("ja", "ハーロウ・ロンドン特別区"), ("ko", "해로 구"), ("nb", "Harrow"), ("nl", "Harrow"), ("no", "Harrow"), ("pl", "London Borough of Harrow"), ("pt", "Harrow"), ("ro", "Harrow"), ("ru", "Харроу"), ("sl", "Harrow, London"), ("sr", "Лондонска општина Хароу"), ("sr_Latn", "Londonska opština Harou"), ("sv", "London Borough of Harrow"), ("uk", "Герроу"), ("ur", "ہیعرو بورو"), ("vi", "Khu Harrow của Luân Đôn"), ("zh", "哈羅區")]),
                        unofficial_name_list: ["Harrow"].to_vec(),
                    }
                ),
                (
                    "HRY",
                    Subdivision{
                        name: "Haringey",
                        country_alpha2: Alpha2::GB,
                        code: "HRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5906113), longitude: Some(-0.1109709), max_latitude: Some(51.6112147), min_latitude: Some(51.564635), max_longitude: Some(-0.0414478), min_longitude: Some(-0.1712852)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هارينجي"), ("bg", "Харингей"), ("ca", "Haringey"), ("ccp", "𑄦𑄢\u{11128}\u{11101}𑄉\u{11133}𑄠"), ("ceb", "Haringey"), ("cs", "Haringey"), ("cy", "Haringey"), ("da", "Haringey"), ("de", "London Borough of Haringey"), ("el", "Χάριγκεϋ"), ("en", "Haringey"), ("es", "Haringey"), ("eu", "Haringey"), ("fa", "منطقه هرینگی لندن"), ("fi", "Haringey"), ("fr", "district londonien de Haringey"), ("ga", "Buirg Londan Haringey"), ("he", "הארינגיי"), ("hi", "ह\u{948}रि\u{902}ग\u{947} बरो"), ("hu", "Haringey kerület"), ("hy", "Հարինգի"), ("is", "Haringey"), ("it", "Haringey"), ("ja", "ハーリンゲイ・ロンドン特別区"), ("ko", "해링게이 구"), ("mk", "Харинги"), ("nb", "Haringey"), ("nl", "Haringey"), ("no", "Haringey"), ("pl", "London Borough of Haringey"), ("pt", "Haringey"), ("ro", "Haringey"), ("ru", "Харинги"), ("sr", "Лондонска општина Харингеј"), ("sr_Latn", "Londonska opština Haringej"), ("sv", "London Borough of Haringey"), ("tr", "Haringey"), ("uk", "Герінгей"), ("ur", "ہارنگے بورو"), ("vi", "Khu Haringey của Luân Đôn"), ("zh", "哈林蓋區")]),
                        unofficial_name_list: ["Haringey"].to_vec(),
                    }
                ),
                (
                    "IOS",
                    Subdivision{
                        name: "Isles of Scilly",
                        country_alpha2: Alpha2::GB,
                        code: "IOS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.925002), longitude: Some(-6.298672000000001), max_latitude: Some(49.9813558), min_latitude: Some(49.8646678), max_longitude: Some(-6.2441587), min_longitude: Some(-6.4179374)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Scilly-eilande"), ("ar", "جزر سيلي"), ("be", "Астравы Сілі"), ("bg", "Острови Сили"), ("bn", "স\u{9cd}কিলি দ\u{9cd}বীপসম\u{9c2}হ"), ("ca", "Illes Scilly"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄣𑄬𑄌\u{11134} 𑄃\u{11127}𑄛\u{11134} 𑄥\u{11128}𑄣\u{11133}𑄦\u{11128}"), ("ceb", "Isles of Scilly"), ("cs", "Scilly"), ("cy", "Ynysoedd Syllan"), ("da", "Isles of Scilly"), ("de", "Scilly-Inseln"), ("el", "Νησιά Σίλι"), ("en", "Isles of Scilly"), ("es", "Islas Sorlingas"), ("et", "Scilly saared"), ("eu", "Scilly uharteak"), ("fa", "مجمع\u{200c}الجزایر سیلی"), ("fi", "Scillynsaaret"), ("fr", "Sorlingues"), ("ga", "Na Scillí"), ("gu", "આઇલ\u{acd}સ ઓફ સ\u{acd}કીલી"), ("he", "איי סילי"), ("hi", "सिसिली द\u{94d}वीप-सम\u{942}ह"), ("hr", "Scilly"), ("hu", "Scilly-szigetek"), ("hy", "Սիլլի կղզիներ"), ("id", "Kepulauan Scilly"), ("is", "Syllingar"), ("it", "Isole Scilly"), ("ja", "シリー諸島"), ("kn", "ಐಲ\u{ccd}ಸ\u{ccd} ಆಫ\u{ccd} ಸ\u{cbf}ಲ\u{ccd}ಲ\u{cbf}"), ("ko", "실리 제도"), ("lt", "Silio salos"), ("mk", "Сили"), ("nb", "Scillyøyene"), ("nl", "Scilly-eilanden"), ("no", "Scillyøyene"), ("pl", "Scilly"), ("pt", "Ilhas Scilly"), ("ro", "Insulele Scilly"), ("ru", "Силли"), ("sk", "Scilly"), ("sr", "Сили"), ("sr_Latn", "Sili"), ("sv", "Scillyöarna"), ("ta", "சில\u{bcd}லி த\u{bc0}வுகள\u{bcd}"), ("te", "స\u{c3f}ల\u{c46}స\u{c4d} అఫ\u{c4d} స\u{c3f}ల\u{c4d}ల\u{c40}"), ("th", "หม\u{e39}\u{e48}เกาะซ\u{e34}ลล\u{e35}"), ("tr", "Scilly Adaları"), ("uk", "Сіллі"), ("ur", "جزائر سیلی"), ("vi", "quần đảo Scilly"), ("yue", "錫利群島"), ("yue_Hans", "锡利群岛"), ("zh", "锡利群岛")]),
                        unofficial_name_list: ["Isles of Scilly"].to_vec(),
                    }
                ),
                (
                    "IOW",
                    Subdivision{
                        name: "Isle of Wight",
                        country_alpha2: Alpha2::GB,
                        code: "IOW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.69271759999999), longitude: Some(-1.3167103), max_latitude: Some(50.7673584), min_latitude: Some(50.5748342), max_longitude: Some(-1.0697787), min_longitude: Some(-1.5917317)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Eiland Wight"), ("ar", "وايت"), ("az", "Vayt adası"), ("be", "Востраў Уайт"), ("bg", "Уайт"), ("bn", "ইসল-অব-উইট"), ("ca", "Illa de Wight"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄣𑄬 𑄃\u{11127}𑄛\u{11134} 𑄃\u{1112e}𑄠𑄖\u{11134}"), ("ceb", "Isle of Wight"), ("cs", "Wight"), ("cy", "Ynys Wyth"), ("da", "Isle of Wight"), ("de", "Isle of Wight"), ("el", "Νήσος Γουάιτ"), ("en", "Isle of Wight"), ("es", "Isla de Wight"), ("et", "Wight"), ("eu", "Wight"), ("fa", "جزیره وایت"), ("fi", "Wightsaari"), ("fr", "île de Wight"), ("ga", "Inis Iocht"), ("gl", "Illa de Wight"), ("gu", "આઇલ ઓફ વાઈટ"), ("he", "האי וייט"), ("hi", "आइल ऑफ\u{93c} वाइट"), ("hu", "Wight-sziget"), ("hy", "Ուայթ կղզի"), ("id", "Pulau Wight"), ("is", "Wighteyja"), ("it", "Isola di Wight"), ("ja", "ワイト島"), ("ka", "უაიტი"), ("km", "កោះវ\u{17c9}ាយធ\u{17cd}"), ("kn", "ಐಲ\u{ccd} ಆಫ\u{ccd} ವ\u{cbf}ಟ\u{ccd}"), ("ko", "아일오브와이트 주"), ("lt", "Vaito sala"), ("lv", "Vaita"), ("mk", "Вајт"), ("mr", "आईल ऑफ वाइट"), ("ms", "Pulau Wight"), ("nb", "Wight"), ("nl", "Isle of Wight"), ("no", "Wight"), ("pa", "ਆੲੀਲ ਆਫ\u{a3c} ਵਾੲੀਟ"), ("pl", "Wight"), ("pt", "Ilha de Wight"), ("ro", "Insula Wight"), ("ru", "Остров Уайт"), ("sk", "Wight"), ("sl", "Isle of Wight"), ("sr", "Острво Вајт"), ("sr_Latn", "Ostrvo Vajt"), ("sv", "Isle of Wight"), ("sw", "Kisiwa cha Wight"), ("ta", "வைட\u{bcd}டுத\u{bcd} த\u{bc0}வு"), ("te", "ఇస\u{c4d}ల\u{c47} అఫ\u{c4d} వ\u{c46}య\u{c3f}ట\u{c4d}"), ("th", "ไอล\u{e4c}ออฟไวต\u{e4c}"), ("tr", "Wight Adası"), ("uk", "Острів Вайт"), ("ur", "آئل آف ویٹ"), ("vi", "Đảo Wight"), ("yue", "威特島"), ("yue_Hans", "威特岛"), ("zh", "怀特岛郡")]),
                        unofficial_name_list: ["Isle of Wight"].to_vec(),
                    }
                ),
                (
                    "ISL",
                    Subdivision{
                        name: "Islington",
                        country_alpha2: Alpha2::GB,
                        code: "ISL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5465063), longitude: Some(-0.1058058), max_latitude: Some(51.5755083), min_latitude: Some(51.5185408), max_longitude: Some(-0.0763576), min_longitude: Some(-0.1425364)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إزلنغتون"), ("be", "Ізлінгтан"), ("ca", "Islington"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄣\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Islington"), ("cs", "Islington"), ("cy", "Islington"), ("da", "Islington"), ("de", "London Borough of Islington"), ("el", "Ίσλινγκτον, Λονδίνο"), ("en", "Islington"), ("es", "Municipio de Islington"), ("et", "Islingtoni linnaosa"), ("eu", "Islington"), ("fa", "منطقه ایزلینگتون لندن"), ("fi", "Islington"), ("fr", "district londonien d’Islington"), ("ga", "Buirg Londan Islington"), ("he", "איזלינגטון"), ("hi", "इस\u{94d}लि\u{902}गटन बरो"), ("hu", "Islington kerület"), ("hy", "Իսլինգթոն"), ("is", "Islington"), ("it", "Borgo londinese di Islington"), ("ja", "イズリントン・ロンドン特別区"), ("ko", "이즐링턴 구"), ("mk", "Излингтон"), ("nb", "Islington"), ("nl", "London Borough of Islington"), ("no", "Islington"), ("pl", "London Borough of Islington"), ("pt", "Islington"), ("ro", "Islington"), ("ru", "Ислингтон"), ("sr", "Лондонска општина Излингтон"), ("sr_Latn", "Londonska opština Izlington"), ("sv", "London Borough of Islington"), ("uk", "Ізлінгтон"), ("ur", "ازلنگٹن بورو"), ("vi", "Khu Islington của Luân Đôn"), ("zh", "伊斯林頓倫敦自治市")]),
                        unofficial_name_list: ["Islington"].to_vec(),
                    }
                ),
                (
                    "IVC",
                    Subdivision{
                        name: "Inverclyde",
                        country_alpha2: Alpha2::GB,
                        code: "IVC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.9118089), longitude: Some(-4.7359063), max_latitude: Some(55.96378319999999), min_latitude: Some(55.8379753), max_longitude: Some(-4.597316), min_longitude: Some(-4.8987741)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Інверклайд"), ("bg", "Инвърклайд"), ("bn", "ইনভিরক\u{9cd}ল\u{9be}ইড"), ("ca", "Inverclyde"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄞𑄢\u{11134}𑄇\u{11133}𑄣\u{1112d}𑄓\u{11128}"), ("cs", "Inverclyde"), ("cy", "Inverclyde"), ("da", "Inverclyde"), ("de", "Inverclyde"), ("en", "Inverclyde"), ("es", "Inverclyde"), ("et", "Inverclyde"), ("eu", "Inverclyde"), ("fa", "اینورکلاید"), ("fi", "Inverclyde"), ("fr", "Inverclyde"), ("ga", "Comhairle Inbhir Chluaidh"), ("gu", "ઈન\u{acd}વરક\u{acd}લાઇડ"), ("it", "Inverclyde"), ("ja", "インヴァークライド"), ("kn", "ಇನ\u{ccd}ವರ\u{ccd}ಕ\u{ccd}ಲೈಡ\u{ccd}"), ("ko", "인버클라이드"), ("lt", "Inverklaidas"), ("nb", "Inverclyde"), ("nl", "Inverclyde"), ("no", "Inverclyde"), ("pl", "Inverclyde"), ("pt", "Inverclyde"), ("ro", "Inverclyde"), ("ru", "Инверклайд"), ("sq", "Inverclyde"), ("sv", "Inverclyde"), ("ta", "இன\u{bcd}வெர\u{bcd}ச\u{bcd}லிட\u{bcd}"), ("te", "ఇన\u{c4d}వర\u{c4d}క\u{c4d}ల\u{c48}డ\u{c4d}"), ("uk", "Інверклайд"), ("ur", "انویرکلیدی"), ("zh", "因弗克萊德")]),
                        unofficial_name_list: ["Inverclyde"].to_vec(),
                    }
                ),
                (
                    "KEC",
                    Subdivision{
                        name: "Kensington and Chelsea",
                        country_alpha2: Alpha2::GB,
                        code: "KEC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4990805), longitude: Some(-0.1938253), max_latitude: Some(51.53035269999999), min_latitude: Some(51.4772216), max_longitude: Some(-0.1498439), min_longitude: Some(-0.2287276)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كينسينغتون وتشيلسي"), ("be", "Бора Кенсінгтон і Чэлсі"), ("bg", "Кенсингтън и Челси"), ("bn", "কেনসিংটন ও চেলসি"), ("ca", "Kensington i Chelsea"), ("ccp", "𑄇𑄬𑄚\u{11134}𑄥\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134} 𑄃\u{11133}𑄃 𑄌𑄬𑄣\u{11134}𑄌\u{11128}"), ("ceb", "Royal Kensington and Chelsea"), ("cs", "Kensington a Chelsea"), ("cy", "Kensington a Chelsea"), ("da", "Kensington and Chelsea"), ("de", "Royal Borough of Kensington and Chelsea"), ("en", "Kensington and Chelsea"), ("es", "Kensington y Chelsea"), ("eu", "Kensington eta Chelsea"), ("fa", "منطقه سلطنتی کنزینگتون و چلسی"), ("fi", "Kensington and Chelsea"), ("fr", "borough royal de Kensington et Chelsea"), ("ga", "Buirg Ríoga Kensington agus Chelsea"), ("gl", "Kensington e Chelsea"), ("gu", "રોયલ બોરો ઓફ ક\u{ac7}ન\u{acd}સિ\u{a82}ગ\u{acd}ટન અન\u{ac7} ચ\u{ac7}લ\u{acd}સી"), ("he", "הרובע המלכותי קנזינגטון וצ׳לסי"), ("hi", "क\u{947}\u{902}सि\u{902}ग\u{94d}टन ऐ\u{902}ड च\u{947}ल\u{94d}सी बरो"), ("hu", "Royal Borough of Kensington and Chelsea"), ("hy", "Կենսինգտոն և Չելսի"), ("id", "Kensington dan Chelsea"), ("is", "Kensington og Chelsea"), ("it", "Kensington e Chelsea"), ("ja", "ケンジントン・アンド・チェルシー王室特別区"), ("ka", "კენსინგტონი და ჩელსი"), ("kn", "ರಾಯಲ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಕ\u{cc6}ನ\u{ccd}ಸ\u{cbf}ಂಗ\u{ccd}ಟನ\u{ccd} ಮತ\u{ccd}ತು ಚ\u{cc6}ಲ\u{ccd}ಸ\u{cbf}ಯಾ"), ("ko", "켄징턴 첼시 구"), ("mk", "Кензингтон и Челси"), ("mr", "क\u{947}न\u{94d}सि\u{902}ग\u{94d}टन व च\u{947}ल\u{94d}सी"), ("nb", "Kensington and Chelsea"), ("nl", "Kensington en Chelsea"), ("no", "Kensington and Chelsea"), ("pl", "Royal Borough of Kensington and Chelsea"), ("pt", "Kensington e Chelsea"), ("ro", "Kensington and Chelsea"), ("ru", "Кенсингтон и Челси"), ("sr", "Лондонска општина Кенсингтон и Челси"), ("sr_Latn", "Londonska opština Kensington i Čelsi"), ("sv", "Royal Borough of Kensington and Chelsea"), ("ta", "ர\u{bbe}யல\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} கென\u{bcd}சிங\u{bcd}டன\u{bcd} & செல\u{bcd}சிய\u{bbe}"), ("te", "ర\u{c3e}యల\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} క\u{c46}న\u{c4d}స\u{c3f}ంగ\u{c4d}టన\u{c4d} అండ\u{c4d} చ\u{c46}ల\u{c4d}స\u{c3f}"), ("tr", "Kensington ve Chelsea"), ("uk", "Кенсінгтон і Челсі"), ("ur", "کینزنگٹن اور چیلسی بورو"), ("vi", "Khu hoàng gia Kensington và Chelsea"), ("zh", "肯辛頓-切爾西區")]),
                        unofficial_name_list: ["Kensington and Chelsea"].to_vec(),
                    }
                ),
                (
                    "KEN",
                    Subdivision{
                        name: "Kent",
                        country_alpha2: Alpha2::GB,
                        code: "KEN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.2787075), longitude: Some(0.5217254000000001), max_latitude: Some(51.4803103), min_latitude: Some(50.9105289), max_longitude: Some(1.4496433), min_longitude: Some(0.0335197)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kent"), ("ar", "كنت"), ("az", "Kent"), ("be", "Кент"), ("bg", "Кент"), ("bn", "কেন\u{9cd}ট"), ("bs", "Kent"), ("ca", "Kent"), ("ccp", "𑄇𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Kent (kondado)"), ("cs", "Kent"), ("cy", "Caint"), ("da", "Kent"), ("de", "Kent"), ("el", "Κεντ"), ("en", "Kent"), ("es", "Kent"), ("et", "Kent"), ("eu", "Kent"), ("fa", "کنت"), ("fi", "Kent"), ("fr", "Kent"), ("ga", "Kent"), ("gl", "Kent"), ("gu", "ક\u{ac7}ન\u{acd}ટ"), ("he", "קנט"), ("hi", "क\u{947}\u{902}ट"), ("hu", "Kent"), ("hy", "Քենթ"), ("id", "Kent"), ("is", "Kent"), ("it", "Kent"), ("ja", "ケント"), ("ka", "კენტი"), ("kn", "ಕ\u{cc6}ಂಟ\u{ccd}"), ("ko", "켄트 주"), ("lt", "Kentas"), ("lv", "Kenta"), ("mk", "Кент"), ("mr", "क\u{947}\u{902}ट"), ("nb", "Kent"), ("nl", "Kent"), ("no", "Kent"), ("pl", "Kent"), ("pt", "Kent"), ("ro", "Kent"), ("ru", "Кент"), ("sk", "Kent"), ("sl", "Kent"), ("sr", "Кент"), ("sr_Latn", "Kent"), ("sv", "Kent"), ("ta", "கென\u{bcd}ட\u{bcd}"), ("te", "క\u{c46}ంట\u{c4d}"), ("th", "เคนต\u{e4c}"), ("tr", "Kent"), ("uk", "Кент"), ("ur", "کینٹ"), ("uz", "Kent"), ("vi", "Kent"), ("yue", "根德郡 (行政郡)"), ("yue_Hans", "根德郡 (行政郡)"), ("zh", "肯特郡")]),
                        unofficial_name_list: ["Kent"].to_vec(),
                    }
                ),
                (
                    "KHL",
                    Subdivision{
                        name: "Kingston upon Hull, City of",
                        country_alpha2: Alpha2::GB,
                        code: "KHL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.7456709), longitude: Some(-0.3367413), max_latitude: Some(53.8132502), min_latitude: Some(53.71951499999999), max_longitude: Some(-0.2413964), min_longitude: Some(-0.4225751)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كينغستون أبون هال"), ("az", "Kinqston-apon-Hall"), ("be", "Горад Кінгстан-апон-Хал"), ("bg", "Кингстън ъпон Хъл"), ("bn", "কিংস\u{9cd}টন আপন হ\u{9be}ল"), ("ca", "Kingston upon Hull"), ("ccp", "𑄇\u{11128}\u{11101}𑄌\u{11133}𑄑\u{11127}𑄚\u{11134} 𑄃𑄛\u{11127}𑄚\u{11134} 𑄦\u{1112a}𑄣\u{11134}"), ("ceb", "City of Kingston upon Hull"), ("cs", "Kingston upon Hull"), ("cy", "Kingston upon Hull"), ("da", "Kingston upon Hull"), ("de", "Kingston upon Hull"), ("el", "Κίνγκστον απόν Χαλ"), ("en", "Kingston upon Hull"), ("es", "Kingston upon Hull"), ("et", "Kingston upon Hull"), ("eu", "Kingston upon Hull"), ("fa", "کینگستن هال"), ("fi", "Kingston upon Hull"), ("fr", "Kingston-upon-Hull"), ("ga", "Kingston upon Hull"), ("gu", "કિ\u{a82}ગ\u{acd}સટન અપોન હલ"), ("he", "האל"), ("hi", "कि\u{902}ग\u{94d}स\u{94d}टन अपॉन ह\u{941}ल"), ("hr", "Kingston na Hullu"), ("hu", "Kingston upon Hull"), ("hy", "Հալ"), ("id", "Kingston upon Hull"), ("is", "Kingston upon Hull"), ("it", "Kingston upon Hull"), ("ja", "キングストン・アポン・ハル"), ("ka", "ჰალი"), ("kn", "ಕ\u{cbf}ಂಗ\u{ccd}ಸ\u{ccd}ಟನ\u{ccd} ಅಪಾನ ಹಾಲ\u{ccd}"), ("ko", "킹스턴어폰헐"), ("lt", "Kingstonas prie Halo"), ("lv", "Kingstona pie Hallas"), ("mk", "Кингстон на Хал"), ("mr", "कि\u{902}ग\u{94d}सस\u{94d}ट\u{945}न अपॉन हल"), ("ms", "Kingston upon Hull"), ("nb", "Kingston upon Hull"), ("nl", "Kingston upon Hull"), ("no", "Kingston upon Hull"), ("pl", "Kingston upon Hull"), ("pt", "Kingston upon Hull"), ("ro", "Kingston upon Hull"), ("ru", "Кингстон-апон-Халл"), ("si", "ක\u{dd2}ංග\u{dca}ස\u{dca}ටන\u{dca} අපොන\u{dca} හ\u{dd2}ල\u{dca}"), ("sk", "Kingston upon Hull"), ("sl", "Kingston upon Hull"), ("sr", "Кингстон на Халу"), ("sr_Latn", "Kingston na Halu"), ("sv", "Kingston upon Hull"), ("sw", "Kingston upon Hull"), ("ta", "கிங\u{bcd}ஸ\u{bcd}டன\u{bcd} அப\u{bbe}ன\u{bcd} ஹில\u{bcd}"), ("te", "క\u{c3f}ంగ\u{c4d}స\u{c4d}టన\u{c4d} అప\u{c3e}న\u{c4d} హల\u{c4d}"), ("th", "ค\u{e34}งส\u{e4c}ต\u{e31}นอะพอนฮ\u{e31}ลล\u{e4c}"), ("tr", "Kingston upon Hull"), ("uk", "Кінгстон-апон-Галл"), ("ur", "کنگسٹن اپون ہل"), ("vi", "Kingston trên sông Hull"), ("yue", "侯城"), ("yue_Hans", "侯城"), ("zh", "赫爾河畔京士頓")]),
                        unofficial_name_list: ["City of Kingston upon Hull"].to_vec(),
                    }
                ),
                (
                    "KIR",
                    Subdivision{
                        name: "Kirklees",
                        country_alpha2: Alpha2::GB,
                        code: "KIR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6020536), longitude: Some(1.2803263), max_latitude: Some(52.6023597), min_latitude: Some(52.6015222), max_longitude: Some(1.2817808), min_longitude: Some(1.278804)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Кърклийс"), ("ccp", "𑄇\u{11128}𑄢\u{11134}𑄇\u{11134}𑄣\u{11128}𑄌\u{11134}"), ("ceb", "Kirklees"), ("de", "Kirklees"), ("en", "Kirklees"), ("es", "Kirklees"), ("fa", "کیرکلیس"), ("fr", "Kirklees"), ("hu", "Kirklees"), ("hy", "Կիրկլիս"), ("id", "Kirklees"), ("it", "Kirklees"), ("ja", "カークリーズ"), ("ko", "커클리스"), ("nb", "Kirklees"), ("nl", "Kirklees"), ("no", "Kirklees"), ("pl", "Kirklees"), ("ro", "Kirklees"), ("ru", "Кирклис"), ("sv", "Kirklees"), ("ta", "கிர\u{bcd}க\u{bcd}ல\u{bc0}சு"), ("uk", "Кірклісс"), ("ur", "کرکلیز"), ("zh", "柯克利斯")]),
                        unofficial_name_list: ["Kirklees"].to_vec(),
                    }
                ),
                (
                    "KTT",
                    Subdivision{
                        name: "Kingston upon Thames",
                        country_alpha2: Alpha2::GB,
                        code: "KTT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.41233), longitude: Some(-0.300689), max_latitude: Some(51.4372907), min_latitude: Some(51.3977209), max_longitude: Some(-0.2495432), min_longitude: Some(-0.3168545)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Кінгстан-на-Тэмзе"), ("ca", "Kingston upon Thames"), ("ccp", "𑄇\u{11128}\u{11101}𑄌\u{11133}𑄑\u{11127}𑄚\u{11134} 𑄃𑄛\u{11127}𑄚\u{11134} 𑄗𑄬𑄟\u{11134}𑄌\u{11134}"), ("ceb", "Royal Kingston upon Thames"), ("cs", "Kingston"), ("cy", "Kingston upon Thames"), ("da", "Kingston upon Thames"), ("de", "Royal Borough of Kingston upon Thames"), ("en", "Kingston upon Thames"), ("es", "Kingston upon Thames"), ("eu", "Kingston upon Thames"), ("fa", "منطقه سلطنتی کینگستون آپون تیمز"), ("fi", "Kingston upon Thames"), ("fr", "district royal de Kingston-upon-Thames"), ("ga", "Ríoga Buirg an Kingston ar Tamais"), ("he", "הרובע המלכותי קינגסטון שעל התמזה"), ("hi", "कि\u{902}ग\u{94d}स\u{94d}टन अपॉन ट\u{947}म\u{94d}स बरो"), ("hu", "Kingston upon Thames kerület"), ("hy", "Քինգսթոն ափոն Թեմզ"), ("is", "Kingston upon Thames"), ("it", "R.B. of Kingston upon Thames"), ("ja", "キングストン・アポン・テムズ王室特別区"), ("ka", "კინგსტონ-აპონ-თემზი"), ("ko", "킹스턴어폰템스 구"), ("mk", "Кралски реон Кингстон на Темза"), ("nb", "Kingston upon Thames"), ("nl", "Kingston upon Thames"), ("no", "Kingston upon Thames"), ("pl", "Royal Borough of Kingston upon Thames"), ("pt", "Kingston upon Thames"), ("ro", "Kingston upon Thames"), ("ru", "Кингстон-апон-Темс"), ("sr", "Лондонска општина Кингстон на Темзи"), ("sr_Latn", "Londonska opština Kingston na Temzi"), ("sv", "Royal Borough of Kingston upon Thames"), ("tr", "Kingston upon Thames"), ("uk", "Кінгстон-на-Темзі"), ("ur", "کنگسٹن اپون تھیمز بورو"), ("zh", "泰晤士河畔京士頓區")]),
                        unofficial_name_list: ["Kingston upon Thames"].to_vec(),
                    }
                ),
                (
                    "KWL",
                    Subdivision{
                        name: "Knowsley",
                        country_alpha2: Alpha2::GB,
                        code: "KWL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.454594), longitude: Some(-2.852907), max_latitude: Some(53.467191), min_latitude: Some(53.4296819), max_longitude: Some(-2.7951111), min_longitude: Some(-2.8915896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Ноўслі"), ("bn", "নোজলে"), ("ccp", "𑄚\u{1112e}𑄌\u{11134}𑄣\u{11128}"), ("ceb", "Knowsley"), ("da", "Knowsley"), ("de", "Knowsley"), ("en", "Knowsley"), ("es", "Knowsley"), ("fa", "کلان\u{200c}شهر مستقل نوزلی"), ("fr", "district métropolitain de Knowsley"), ("gu", "નોસ\u{acd}લી"), ("it", "Metropolitan Borough of Knowsley"), ("ja", "ノーズリー"), ("kn", "ನೋಸ\u{ccd}ಲೇ"), ("ko", "노즐리 도시 자치구"), ("lt", "Knauslis"), ("nb", "Knowsley"), ("nl", "Knowsley"), ("no", "Knowsley"), ("pl", "Metropolitan Borough of Knowsley"), ("pt", "Knowsley"), ("ro", "Knowsley"), ("ru", "Ноусли"), ("sv", "Knowsley"), ("ta", "கினோவ\u{bcd}ஸ\u{bcd}லே"), ("te", "న\u{c4a}స\u{c4d}ల\u{c47}"), ("uk", "Ноуслі"), ("ur", "میٹروپولیٹن بورو نوزلی"), ("zh", "諾斯利都市自治市")]),
                        unofficial_name_list: ["Knowsley"].to_vec(),
                    }
                ),
                (
                    "LAN",
                    Subdivision{
                        name: "Lancashire",
                        country_alpha2: Alpha2::GB,
                        code: "LAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.9690089), longitude: Some(-2.6276908), max_latitude: Some(54.2395574), min_latitude: Some(53.48276569999999), max_longitude: Some(-2.0450727), min_longitude: Some(-3.057182)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lancashire"), ("ar", "لانكشر"), ("az", "Lankaşir"), ("be", "Ланкашыр"), ("bg", "Ланкашър"), ("bn", "ল\u{9cd}য\u{9be}ঙ\u{9cd}ক\u{9be}শ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Lancashire"), ("ccp", "𑄣\u{11133}𑄠𑄚\u{11134}𑄇𑄬𑄥𑄠𑄢\u{11134}"), ("ceb", "Lancashire"), ("cs", "Lancashire"), ("cy", "Swydd Gaerhirfryn"), ("da", "Lancashire"), ("de", "Lancashire"), ("el", "Λανκασάιρ"), ("en", "Lancashire"), ("es", "Lancashire"), ("et", "Lancashire"), ("eu", "Lancashire"), ("fa", "لانکاشایر"), ("fi", "Lancashire"), ("fr", "Lancashire"), ("ga", "Lancashire"), ("gl", "Lancashire"), ("gu", "લન\u{acd}કાશાયર"), ("he", "לנקשייר"), ("hi", "ल\u{948}\u{902}काशिर"), ("hu", "Lancashire"), ("hy", "Լանկաշիր"), ("id", "Lancashire"), ("is", "Lancashire"), ("it", "Lancashire"), ("ja", "ランカシャー"), ("ka", "ლანკაშირი"), ("kn", "ಲಂಕಾಷೈರ\u{ccd}"), ("ko", "랭커셔 주"), ("lt", "Lankašyras"), ("lv", "Lankašīra"), ("mk", "Ланкашир"), ("mr", "ल\u{901}क\u{947}शायर"), ("nb", "Lancashire"), ("nl", "Lancashire"), ("no", "Lancashire"), ("pl", "Lancashire"), ("pt", "Lancashire"), ("ro", "Lancashire"), ("ru", "Ланкашир"), ("sk", "Lancashire"), ("sl", "Lancashire"), ("sr", "Ланкашир"), ("sr_Latn", "Lankašir"), ("sv", "Lancashire"), ("ta", "லேன\u{bcd}கஷ\u{bc0}ர\u{bcd}"), ("te", "ల\u{c3e}ంక\u{c4d}ష\u{c48}ర\u{c4d}"), ("th", "แลงคาเชอร\u{e4c}"), ("tr", "Lancashire"), ("uk", "Ланкашир"), ("ur", "لنکاشائر"), ("vi", "Lancashire"), ("yue", "蘭開夏郡"), ("yue_Hans", "兰开夏郡"), ("zh", "兰开夏郡")]),
                        unofficial_name_list: ["Lancashire"].to_vec(),
                    }
                ),
                (
                    "LBC",
                    Subdivision{
                        name: "Lisburn and Castlereagh",
                        country_alpha2: Alpha2::GB,
                        code: "LBC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄣\u{11128}𑄌\u{11134}𑄝𑄢\u{11134}𑄚\u{11134} 𑄃\u{11133}𑄃 𑄇\u{11133}𑄠𑄥\u{11127}𑄣\u{11134}𑄢\u{11128}𑄇\u{11134}"), ("de", "Lisburn and Castlereagh"), ("en", "Lisburn and Castlereagh"), ("fa", "لیسبورن و کاسلری"), ("fr", "Lisburn and Castlereagh"), ("it", "Distretto di Lisburn e Castlereagh"), ("ja", "リスバーン・アンド・キャッスルレー"), ("nl", "Lisburn and Castlereagh"), ("ur", "لسبرن اور کیسلرے")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LBH",
                    Subdivision{
                        name: "Lambeth",
                        country_alpha2: Alpha2::GB,
                        code: "LBH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4571477), longitude: Some(-0.1230681), max_latitude: Some(51.5098711), min_latitude: Some(51.410991), max_longitude: Some(-0.07830669999999999), min_longitude: Some(-0.1512314)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لامبيث"), ("be", "бора Ламбет"), ("ca", "Lambeth"), ("ccp", "𑄣𑄟\u{11134}𑄝𑄬𑄖\u{11134}"), ("ceb", "Lambeth (distrito)"), ("cs", "Lambeth"), ("cy", "Lambeth"), ("da", "Lambeth"), ("de", "London Borough of Lambeth"), ("el", "Λάμπεθ, δημοτικό διαμέρισμα του Λονδίνου"), ("en", "Lambeth"), ("es", "Municipio de Lambeth"), ("eu", "Lambeth"), ("fa", "منطقه لمبث لندن"), ("fi", "Lambeth"), ("fr", "district londonien de Lambeth"), ("ga", "Buirg Londan Lambeth"), ("he", "למבת׳"), ("hi", "ल\u{948}म\u{94d}ब\u{947}थ बरो"), ("hu", "London Borough of Lambeth"), ("hy", "Լամբեթ"), ("id", "Lambeth"), ("is", "Lambeth"), ("it", "Borgo londinese di Lambeth"), ("ja", "ランベス・ロンドン特別区"), ("ko", "램버스 구"), ("lv", "Lembeta"), ("nb", "Lambeth"), ("nl", "Lambeth"), ("no", "Lambeth"), ("pl", "London Borough of Lambeth"), ("pt", "Lambeth"), ("ro", "Lambeth"), ("ru", "Ламбет"), ("sl", "London Borough of Lambeth"), ("sr", "Лондонска општина Ламбет"), ("sr_Latn", "Londonska opština Lambet"), ("sv", "London Borough of Lambeth"), ("tr", "Lambeth"), ("uk", "Ламбет"), ("ur", "لیمبیتھ بورو"), ("vi", "Khu Lambeth của Luân Đôn"), ("zh", "蘭貝斯區")]),
                        unofficial_name_list: ["Lambeth"].to_vec(),
                    }
                ),
                (
                    "LCE",
                    Subdivision{
                        name: "Leicester",
                        country_alpha2: Alpha2::GB,
                        code: "LCE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6368778), longitude: Some(-1.1397592), max_latitude: Some(52.6915038), min_latitude: Some(52.5806504), max_longitude: Some(-1.0462128), min_longitude: Some(-1.2159877)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Leicester"), ("ar", "لستر"), ("az", "Lester"), ("be", "Лестэр"), ("bg", "Лестър"), ("bn", "লিচেস\u{9cd}ট\u{9be}র"), ("ca", "Leicester"), ("ccp", "𑄣𑄬\u{1112d}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "City of Leicester"), ("cs", "Leicester"), ("da", "Leicester"), ("de", "Leicester"), ("el", "Λέστερ"), ("en", "Leicester"), ("es", "Leicester"), ("et", "Leicester"), ("eu", "Leicester"), ("fa", "لستر"), ("fi", "Leicester"), ("fr", "Leicester"), ("gl", "Leicester"), ("gu", "લ\u{ac7}સ\u{acd}ટર"), ("he", "לסטר"), ("hi", "लीस\u{947}स\u{94d}टर"), ("hr", "Leicester"), ("hu", "Leicester"), ("hy", "Լեսթեր"), ("id", "Leicester"), ("is", "Leicester"), ("it", "Leicester"), ("ja", "レスター"), ("ka", "ლესტერი"), ("kn", "ಲೀಸ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}"), ("ko", "레스터"), ("lt", "Lesteris"), ("lv", "Lestera"), ("mr", "ल\u{947}स\u{94d}टर"), ("ms", "Leicester"), ("nb", "Leicester"), ("nl", "Leicester"), ("no", "Leicester"), ("pl", "Leicester"), ("pt", "Leicester"), ("ro", "Leicester"), ("ru", "Лестер"), ("si", "ලෙස\u{dca}ටර\u{dca}"), ("sk", "Leicester"), ("sl", "Leicester"), ("sr", "Лестер"), ("sr_Latn", "Lester"), ("sv", "Leicester"), ("sw", "Leicester"), ("ta", "லெஸ\u{bcd}டர\u{bcd}"), ("te", "ల\u{c47}స\u{c4d}టర\u{c4d}"), ("th", "เลสเตอร\u{e4c}"), ("tr", "Leicester"), ("uk", "Лестер"), ("ur", "لیسٹر"), ("vi", "Leicester"), ("zh", "莱斯特")]),
                        unofficial_name_list: ["Leicester"].to_vec(),
                    }
                ),
                (
                    "LDS",
                    Subdivision{
                        name: "Leeds",
                        country_alpha2: Alpha2::GB,
                        code: "LDS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.8007554), longitude: Some(-1.5490774), max_latitude: Some(53.8812021), min_latitude: Some(53.7308029), max_longitude: Some(-1.3973731), min_longitude: Some(-1.6740915)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Lids"), ("be", "Сіці-оф-Лідс"), ("bg", "Лийдс"), ("ccp", "𑄣\u{11129}𑄖\u{11134}𑄌\u{11134}"), ("cs", "Leeds"), ("de", "City of Leeds"), ("en", "Leeds"), ("fa", "سیتی لیدز"), ("fr", "cité de Leeds"), ("hu", "City of Leeds"), ("id", "Kota Leeds"), ("it", "City of Leeds"), ("ja", "シティ・オブ・リーズ"), ("ko", "시티오브리즈"), ("mk", "Град Лидс"), ("nb", "City of Leeds"), ("nl", "City of Leeds"), ("no", "City of Leeds"), ("pl", "City of Leeds"), ("ru", "Сити-оф-Лидс"), ("sv", "Leeds"), ("ta", "ல\u{bc0}ட\u{bcd}சு"), ("tr", "Leeds Şehri"), ("uk", "Сіті-оф-Лідс"), ("ur", "لیڈز شہر"), ("zh", "里茲市")]),
                        unofficial_name_list: ["Leeds"].to_vec(),
                    }
                ),
                (
                    "LEC",
                    Subdivision{
                        name: "Leicestershire",
                        country_alpha2: Alpha2::GB,
                        code: "LEC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.74012279999999), longitude: Some(-1.1405925), max_latitude: Some(52.9776572), min_latitude: Some(52.3921566), max_longitude: Some(-0.6641109), min_longitude: Some(-1.5975473)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Leicestershire"), ("ar", "ليسترشير"), ("be", "Лестэршыр"), ("bg", "Лестършър"), ("bn", "লিচেস\u{9cd}ট\u{9be}রশ\u{9be}য\u{9bc}\u{9be}র"), ("bs", "Leicestershire"), ("ca", "Leicestershire"), ("ccp", "𑄣𑄬\u{1112d}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Leicestershire"), ("cs", "Leicestershire"), ("cy", "Swydd Gaerlŷr"), ("da", "Leicestershire"), ("de", "Leicestershire"), ("el", "Λέστερσαϊρ"), ("en", "Leicestershire"), ("es", "Leicestershire"), ("et", "Leicestershire"), ("eu", "Leicestershire"), ("fa", "لسترشایر"), ("fi", "Leicestershire"), ("fr", "Leicestershire"), ("ga", "Leicestershire"), ("gl", "Leicestershire"), ("gu", "લીક\u{ac7}સ\u{acd}ટરશાયર"), ("he", "לסטרשייר"), ("hi", "ल\u{947}स\u{94d}टरशायर"), ("hr", "Leicestershire"), ("hu", "Leicestershire"), ("hy", "Լեստերշիր"), ("id", "Leicestershire"), ("is", "Leicestershire"), ("it", "Leicestershire"), ("ja", "レスターシャー"), ("jv", "Leicestershire"), ("ka", "ლესტერშირი"), ("kn", "ಲೈಸ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "레스터셔 주"), ("lt", "Lesteršyras"), ("lv", "Lesteršīra"), ("mk", "Лестершир"), ("mr", "ल\u{947}स\u{94d}टरशायर"), ("nb", "Leicestershire"), ("nl", "Leicestershire"), ("no", "Leicestershire"), ("pl", "Leicestershire"), ("pt", "Leicestershire"), ("ro", "Leicestershire"), ("ru", "Лестершир"), ("sk", "Leicestershire"), ("sl", "Leicestershire"), ("sq", "Leicestershire"), ("sr", "Лестершир"), ("sr_Latn", "Lesteršir"), ("sv", "Leicestershire"), ("ta", "லேய\u{bcd}ஸ\u{bcd}ஸ\u{bcd}டேர\u{bcd}ஸயர\u{bcd}"), ("te", "ల\u{c46}స\u{c46}స\u{c4d}టర\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "เลสเตอร\u{e4c}เชอร\u{e4c}"), ("tr", "Leicestershire"), ("uk", "Лестершир"), ("ur", "لیسٹرشائر"), ("vi", "Leicestershire"), ("yue", "里斯特郡"), ("yue_Hans", "里斯特郡"), ("zh", "莱斯特郡")]),
                        unofficial_name_list: ["Leicestershire"].to_vec(),
                    }
                ),
                (
                    "LEW",
                    Subdivision{
                        name: "Lewisham",
                        country_alpha2: Alpha2::GB,
                        code: "LEW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4414579), longitude: Some(-0.0117006), max_latitude: Some(51.4935635), min_latitude: Some(51.41355), max_longitude: Some(0.0390452), min_longitude: Some(-0.0750933)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لويشام"), ("be", "Луішэм"), ("ca", "Lewisham"), ("ccp", "𑄣𑄬𑄃\u{1112a}𑄃\u{11128}𑄥\u{11133}𑄠𑄟\u{11134}"), ("ceb", "Lewisham"), ("cs", "Lewisham"), ("cy", "Lewisham"), ("da", "Lewisham"), ("de", "London Borough of Lewisham"), ("en", "Lewisham"), ("es", "Municipio de Lewisham"), ("eu", "Lewisham"), ("fa", "منطقه لویشام لندن"), ("fi", "Lewisham"), ("fr", "district londonien de Lewisham"), ("ga", "Buirg Londan Lewisham"), ("he", "לואישהאם"), ("hi", "ल\u{942}विशम बरो"), ("hu", "Lewisham kerület"), ("is", "Lewisham"), ("it", "Lewisham"), ("ja", "ルイシャム・ロンドン特別区"), ("ka", "ლუიშემი"), ("ko", "루이셤 구"), ("mk", "Луишам"), ("nb", "Lewisham"), ("nl", "Lewisham (borough)"), ("no", "Lewisham"), ("pl", "London Borough of Lewisham"), ("pt", "Lewisham"), ("ro", "Lewisham"), ("ru", "Луишем"), ("sl", "London Borough of Lewisham"), ("sr", "Лондонска општина Луишам"), ("sr_Latn", "Londonska opština Luišam"), ("sv", "London Borough of Lewisham"), ("tr", "Lewisham"), ("uk", "Луїшем"), ("ur", "لیوشم بورو"), ("vi", "Khu Lewisham của Luân Đôn"), ("zh", "劉易舍姆區")]),
                        unofficial_name_list: ["Lewisham"].to_vec(),
                    }
                ),
                (
                    "LIN",
                    Subdivision{
                        name: "Lincolnshire",
                        country_alpha2: Alpha2::GB,
                        code: "LIN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.2178821), longitude: Some(-0.1999702), max_latitude: Some(53.6163661), min_latitude: Some(52.6402179), max_longitude: Some(0.3562637), min_longitude: Some(-0.8206513999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lincolnshire"), ("ar", "لينكونشير"), ("az", "Linkolnşir"), ("be", "Графства Лінкальншыр"), ("bg", "Линкълншър"), ("bn", "লিংকনশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Lincolnshire"), ("ccp", "𑄣\u{11128}𑄚\u{11134}𑄇\u{1112e}𑄣\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Lincolnshire (kondado)"), ("cs", "Lincolnshire"), ("cy", "Swydd Lincoln"), ("da", "Lincolnshire"), ("de", "Lincolnshire"), ("el", "Λίνκολνσαϊρ"), ("en", "Lincolnshire"), ("es", "Lincolnshire"), ("et", "Lincolnshire"), ("eu", "Lincolnshire"), ("fa", "لینکولن\u{200c}شایر"), ("fi", "Lincolnshire"), ("fr", "Lincolnshire"), ("ga", "Lincolnshire"), ("gl", "Lincolnshire"), ("gu", "લિ\u{a82}કનશાયર"), ("he", "לינקולנשייר"), ("hi", "लि\u{902}कनशायर"), ("hr", "Lincolnshire"), ("hu", "Lincolnshire"), ("hy", "Լինկոլնշիր"), ("id", "Lincolnshire"), ("is", "Lincolnshire"), ("it", "Lincolnshire"), ("ja", "リンカンシャー"), ("ka", "ლინკოლნშირი"), ("kn", "ಲ\u{cbf}ಂಕನ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "링컨셔 주"), ("lt", "Linkolnšyras"), ("lv", "Linkolnšīra"), ("mk", "Линколншир"), ("mr", "लि\u{902}कनशायर"), ("nb", "Lincolnshire"), ("ne", "लिङकनसायर"), ("nl", "Lincolnshire"), ("no", "Lincolnshire"), ("pl", "Lincolnshire"), ("pt", "Lincolnshire"), ("ro", "Lincolnshire"), ("ru", "Линкольншир"), ("sk", "Lincolnshire"), ("sq", "Lincolnshire"), ("sr", "Линколншир"), ("sr_Latn", "Linkolnšir"), ("sv", "Lincolnshire"), ("ta", "லிண\u{bcd}க\u{bbe}ன\u{bcd}ஷிர\u{bcd}"), ("te", "ల\u{c3f}ంకన\u{c4d}ష\u{c48}ర\u{c4d}"), ("th", "ล\u{e34}งคอล\u{e4c}นเชอร\u{e4c}"), ("tr", "Lincolnshire"), ("uk", "Лінкольншир"), ("ur", "لنکنشائر"), ("vi", "Lincolnshire"), ("yue", "林肯郡"), ("yue_Hans", "林肯郡"), ("zh", "林肯郡")]),
                        unofficial_name_list: ["Lincolnshire"].to_vec(),
                    }
                ),
                (
                    "LIV",
                    Subdivision{
                        name: "Liverpool",
                        country_alpha2: Alpha2::GB,
                        code: "LIV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4083714), longitude: Some(-2.9915726), max_latitude: Some(53.503907), min_latitude: Some(53.335905), max_longitude: Some(-2.8129382), min_longitude: Some(-3.0087541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Liverpool"), ("am", "ሊቨርፑል፣ እንግሊዝ"), ("ar", "ليفربول"), ("az", "Liverpul"), ("be", "Ліверпул"), ("bg", "Ливърпул"), ("bn", "লিভ\u{9be}রপ\u{9c1}ল"), ("ca", "Liverpool"), ("ccp", "𑄣\u{11128}𑄞𑄢\u{11134}𑄛\u{1112a}𑄣\u{11134}"), ("ceb", "Liverpool (kondado sa Hiniusang Gingharian, Inglatera, lat 53,42, long -2,92)"), ("cs", "Liverpool"), ("da", "Liverpool"), ("de", "Liverpool"), ("el", "Λίβερπουλ"), ("en", "Liverpool"), ("es", "Liverpool"), ("et", "Liverpool"), ("eu", "Liverpool"), ("fa", "لیورپول"), ("fi", "Liverpool"), ("fr", "Liverpool"), ("gl", "Liverpool"), ("gu", "લીવરપ\u{ac2}લ"), ("he", "ליברפול"), ("hi", "लिवरप\u{942}ल"), ("hr", "Liverpool"), ("hu", "Liverpool"), ("hy", "Լիվերպուլ"), ("id", "Liverpool"), ("is", "Liverpool"), ("it", "Liverpool"), ("ja", "リヴァプール"), ("ka", "ლივერპული"), ("kn", "ಲ\u{cbf}ವರ\u{ccd}\u{200c}ಪ\u{cc2}ಲ\u{ccd}"), ("ko", "리버풀"), ("lt", "Liverpulis"), ("lv", "Liverpūle"), ("ml", "ലിവർപ\u{d42}ൾ"), ("mn", "Ливерпүүл"), ("mr", "लिव\u{94d}हरप\u{942}ल"), ("ms", "Liverpool"), ("nb", "Liverpool"), ("ne", "लिभरप\u{941}ल"), ("nl", "Liverpool"), ("no", "Liverpool"), ("pl", "Liverpool"), ("pt", "Liverpool"), ("ro", "Liverpool"), ("ru", "Ливерпуль"), ("si", "ල\u{dd2}වර\u{dca}ප\u{dd6}ල\u{dca}"), ("sk", "Liverpool"), ("sl", "Liverpool"), ("sr", "Ливерпул"), ("sr_Latn", "Liverpul"), ("sv", "Liverpool"), ("sw", "Liverpool"), ("ta", "லிவர\u{bcd}பூல\u{bcd}"), ("te", "ల\u{c3f}వర\u{c4d}\u{200c}పూల\u{c4d}"), ("th", "ล\u{e34}เวอร\u{e4c}พ\u{e39}ล"), ("tr", "Liverpool"), ("uk", "Ліверпуль"), ("ur", "لیورپول"), ("vi", "Liverpool"), ("zh", "利物浦")]),
                        unofficial_name_list: ["Liverpool"].to_vec(),
                    }
                ),
                (
                    "LND",
                    Subdivision{
                        name: "London, City of",
                        country_alpha2: Alpha2::GB,
                        code: "LND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5073509), longitude: Some(-0.1277583), max_latitude: Some(51.6723432), min_latitude: Some(51.38494009999999), max_longitude: Some(0.148271), min_longitude: Some(-0.3514683)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityCorporation,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "City of London"), ("am", "ለንደን"), ("ar", "مدينة لندن"), ("az", "London"), ("be", "Сіці"), ("bg", "Сити"), ("bn", "সিটি অব লন\u{9cd}ডন"), ("bs", "London"), ("ca", "La City"), ("ccp", "𑄣\u{11127}𑄚\u{11134}𑄓\u{11127}𑄚\u{11134}"), ("ceb", "Dakbayan sa Londres"), ("cs", "City"), ("cy", "Dinas Llundain"), ("da", "City of London"), ("de", "City of London"), ("el", "Σίτι του Λονδίνου"), ("en", "London"), ("es", "City de Londres"), ("et", "City of London"), ("eu", "City of London"), ("fa", "سیتی لندن"), ("fi", "Lontoon City"), ("fr", "cité de Londres"), ("ga", "Cathair Londan"), ("gl", "Cidade de Londres"), ("gu", "લ\u{a82}ડનન\u{ac1}\u{a82} શહ\u{ac7}ર"), ("ha", "Landan"), ("ha_NE", "Landan"), ("he", "הסיטי של לונדון"), ("hi", "सिटी ऑफ\u{93c} ल\u{902}दन"), ("hr", "London"), ("hu", "City of London"), ("hy", "Լոնդոնյան Սիթի"), ("id", "City of London"), ("ig", "London"), ("is", "Lundúnaborg"), ("it", "Città di Londra"), ("ja", "シティ・オブ・ロンドン"), ("jv", "London"), ("ka", "ლონდონის სიტი"), ("kk", "Сити"), ("km", "ទ\u{17b8}ក\u{17d2}រ\u{17bb}ងឡ\u{17bb}ង"), ("kn", "ಸ\u{cbf}ಟ\u{cbf} ಆಫ\u{ccd} ಲಂಡನ\u{ccd}"), ("ko", "시티 오브 런던"), ("ky", "Лондон"), ("lo", "ລອນດອນ"), ("lt", "Londono Sitis"), ("lv", "Londonas Sitija"), ("mk", "Сити"), ("ml", "ലണ\u{d4d}ടൻ"), ("mn", "Лондон"), ("mr", "सिटी ऑफ ल\u{902}डन"), ("ms", "Bandaraya London"), ("my", "လန\u{103a}ဒန\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "City of London"), ("ne", "लण\u{94d}डन"), ("nl", "City of London"), ("no", "City of London"), ("or", "ଲଣ\u{b4d}ଡନ"), ("pa", "ਲ\u{a70}ਡਨ"), ("pl", "City of London"), ("ps", "لندن"), ("pt", "Cidade de Londres"), ("ro", "City of London"), ("ru", "Лондонский Сити"), ("sd", "لنڊن"), ("si", "ලන\u{dca}ඩන\u{dca} නගරය"), ("sk", "City of London"), ("sl", "City of London"), ("so", "London"), ("sq", "Londra"), ("sr", "Сити"), ("sr_Latn", "Siti"), ("sv", "City of London"), ("sw", "London"), ("ta", "லண\u{bcd}டன\u{bcd} நகரம\u{bcd}"), ("te", "లండన\u{c4d} నగరం"), ("th", "นครลอนดอน"), ("tk", "London"), ("tr", "Londra Şehri"), ("uk", "Лондонське Сіті"), ("ur", "لندن شہر"), ("uz", "London"), ("vi", "Thành phố Luân Đôn"), ("yo", "Lọndọnu"), ("yo_BJ", "Lɔndɔnu"), ("yue", "倫敦市"), ("yue_Hans", "伦敦市"), ("zh", "倫敦市"), ("zu", "Idolobha weLondon")]),
                        unofficial_name_list: ["London, City of"].to_vec(),
                    }
                ),
                (
                    "LUT",
                    Subdivision{
                        name: "Luton",
                        country_alpha2: Alpha2::GB,
                        code: "LUT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8786707), longitude: Some(-0.4200255), max_latitude: Some(51.9277388), min_latitude: Some(51.8544733), max_longitude: Some(-0.349923), min_longitude: Some(-0.5059485)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لوتن"), ("az", "Luton"), ("bg", "Лутън"), ("bn", "ল\u{9c1}টন"), ("ca", "Luton"), ("ccp", "𑄣\u{11128}𑄅\u{1112a}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Luton"), ("cs", "Luton"), ("cy", "Luton"), ("da", "Luton"), ("de", "Luton"), ("el", "Λούτον"), ("en", "Luton"), ("es", "Luton"), ("eu", "Luton"), ("fa", "لوتن"), ("fi", "Luton"), ("fr", "Luton"), ("ga", "Luton"), ("gu", "લ\u{acd}ય\u{ac1}ટોન"), ("he", "לוטון"), ("hu", "Luton"), ("hy", "Լութոն"), ("id", "Luton"), ("is", "Luton"), ("it", "Luton"), ("ja", "ルートン"), ("kn", "ಲುಟನ\u{ccd}"), ("ko", "루턴"), ("lt", "Lutonas"), ("mk", "Лутон"), ("ms", "Luton"), ("nb", "Luton"), ("nl", "Luton"), ("no", "Luton"), ("pl", "Luton"), ("pt", "Luton"), ("ro", "Luton"), ("ru", "Лутон"), ("sk", "Luton"), ("sl", "Luton"), ("sr", "Лутон"), ("sr_Latn", "Luton"), ("sv", "Luton"), ("sw", "Luton"), ("ta", "லூடன\u{bcd}"), ("te", "లూటన\u{c4d}"), ("tr", "Luton"), ("uk", "Лутон"), ("ur", "لوٹن"), ("vi", "Luton"), ("yue", "盧頓"), ("yue_Hans", "卢顿"), ("zh", "卢顿")]),
                        unofficial_name_list: ["Luton"].to_vec(),
                    }
                ),
                (
                    "MAN",
                    Subdivision{
                        name: "Manchester",
                        country_alpha2: Alpha2::GB,
                        code: "MAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4807593), longitude: Some(-2.2426305), max_latitude: Some(53.5445879), min_latitude: Some(53.39990299999999), max_longitude: Some(-2.1470875), min_longitude: Some(-2.3000969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Manchester"), ("am", "ማንችስተር"), ("ar", "مانشستر"), ("be", "Манчэстэр"), ("bg", "Манчестър"), ("bn", "ম\u{9cd}য\u{9be}নচেস\u{9cd}ট\u{9be}র"), ("ca", "Manchester"), ("ccp", "𑄟\u{11133}𑄠𑄚\u{11134}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Manchester"), ("cs", "Manchester"), ("da", "Manchester"), ("de", "Manchester"), ("el", "Μάντσεστερ"), ("en", "Manchester"), ("es", "Mánchester"), ("et", "Manchester"), ("eu", "Manchester"), ("fa", "منچستر"), ("fi", "Manchester"), ("fr", "Manchester"), ("gl", "Manchester"), ("gu", "માન\u{acd}ચ\u{ac7}સ\u{acd}ટર"), ("he", "מנצ׳סטר"), ("hi", "म\u{948}न\u{94d}च\u{947}स\u{94d}टर"), ("hr", "Manchester"), ("hu", "Manchester"), ("hy", "Մանչեսթեր"), ("id", "Manchester"), ("is", "Manchester"), ("it", "Manchester"), ("ja", "マンチェスター"), ("ka", "მანჩესტერი"), ("km", "មែនឈេសទ\u{17d0}រ"), ("kn", "ಮ\u{ccd}ಯಾಂಚ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}"), ("ko", "맨체스터"), ("lt", "Mančesteris"), ("lv", "Mančestra"), ("ml", "മ\u{d3e}ഞ\u{d4d}ചസ\u{d4d}റ\u{d4d}റർ"), ("mn", "Манчестер"), ("mr", "म\u{901}च\u{947}स\u{94d}टर"), ("ms", "Manchester"), ("nb", "Manchester"), ("ne", "म\u{94d}यानच\u{947}स\u{94d}टर"), ("nl", "Manchester"), ("no", "Manchester"), ("pl", "Manchester"), ("pt", "Manchester"), ("ro", "Manchester"), ("ru", "Манчестер"), ("si", "මැන\u{dca}චෙස\u{dca}ටර\u{dca}"), ("sk", "Manchester"), ("sl", "Manchester"), ("sr", "Манчестер"), ("sr_Latn", "Mančester"), ("sv", "Manchester"), ("sw", "Manchester"), ("ta", "ம\u{bbe}ன\u{bcd}செஸ\u{bcd}டர\u{bcd}"), ("te", "మ\u{c3e}ంచ\u{c46}స\u{c4d}టర\u{c4d}"), ("th", "แมนเชสเตอร\u{e4c}"), ("tr", "Manchester"), ("uk", "Манчестер"), ("ur", "مانچسٹر"), ("vi", "Manchester"), ("zh", "曼彻斯特")]),
                        unofficial_name_list: ["Manchester"].to_vec(),
                    }
                ),
                (
                    "MDB",
                    Subdivision{
                        name: "Middlesbrough",
                        country_alpha2: Alpha2::GB,
                        code: "MDB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.574227), longitude: Some(-1.234956), max_latitude: Some(54.5915173), min_latitude: Some(54.51755199999999), max_longitude: Some(-1.1666256), min_longitude: Some(-1.2794684)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميدلزبرة"), ("az", "Midlsbro"), ("be", "Горад Мідлсбра"), ("bg", "Мидълзбро"), ("bn", "মিডলসব\u{9cd}রো"), ("bs", "Middlesbrough"), ("ca", "Middlesbrough"), ("ccp", "𑄟\u{11128}𑄓\u{11127}𑄣\u{11134}𑄌\u{11134}𑄝\u{11133}𑄢\u{1112f}"), ("ceb", "Middlesbrough (kondado)"), ("cs", "Middlesbrough"), ("cy", "Middlesbrough"), ("da", "Middlesbrough"), ("de", "Middlesbrough"), ("el", "Μίντλεσμπρο"), ("en", "Middlesbrough"), ("es", "Middlesbrough"), ("et", "Middlesbrough"), ("eu", "Middlesbrough"), ("fa", "میدلزبورو"), ("fi", "Middlesbrough"), ("fr", "Middlesbrough"), ("ga", "Middlesbrough"), ("gu", "મીડલ\u{acd}સબ\u{acd}રો"), ("he", "מידלסברו"), ("hr", "Middlesbrough"), ("hu", "Middlesbrough"), ("hy", "Միդլսբրո"), ("id", "Middlesbrough"), ("is", "Middlesbrough"), ("it", "Middlesbrough"), ("ja", "ミドルズブラ"), ("ka", "მიდლზბრო"), ("kk", "Мидлсбро"), ("kn", "ಮ\u{cbf}ಡಲ\u{ccd}ಸ\u{ccd}ಬರೋ"), ("ko", "미들즈브러"), ("lt", "Midlsbro"), ("lv", "Midlsbro"), ("mr", "मिडल\u{94d}सब\u{94d}रो"), ("nb", "Middlesbrough"), ("nl", "Middlesbrough"), ("no", "Middlesbrough"), ("pl", "Middlesbrough"), ("pt", "Middlesbrough"), ("ro", "Middlesbrough"), ("ru", "Мидлсбро"), ("sk", "Middlesbrough"), ("sl", "Middlesbrough"), ("sr", "Мидлсбро"), ("sr_Latn", "Midlsbro"), ("sv", "Middlesbrough"), ("sw", "Middlesbrough"), ("ta", "மிட\u{bcd}ட\u{bcd}லேஸ\u{bcd}ப\u{bcd}ரவுக\u{bcd}ஹ\u{bcd}"), ("te", "మ\u{c3f}డ\u{c3f}ల\u{c4d}స\u{c4d} బ\u{c4d}రఫ\u{c4d}"), ("th", "ม\u{e34}ดเด\u{e34}ลส\u{e4c}เบรอ"), ("tr", "Middlesbrough"), ("uk", "Мідлсбро"), ("ur", "میڈیلزبرو"), ("vi", "Middlesbrough"), ("yue", "米杜士堡"), ("yue_Hans", "米杜士堡"), ("zh", "米德爾斯伯勒")]),
                        unofficial_name_list: ["Middlesbrough"].to_vec(),
                    }
                ),
                (
                    "MDW",
                    Subdivision{
                        name: "Medway",
                        country_alpha2: Alpha2::GB,
                        code: "MDW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4084923), longitude: Some(0.5878457), max_latitude: Some(51.4874216), min_latitude: Some(51.32789649999999), max_longitude: Some(0.723711), min_longitude: Some(0.3973173)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميدواي"), ("bn", "মেডওয\u{9bc}ে"), ("ca", "Medway"), ("ccp", "𑄟\u{11128}𑄖\u{11134}𑄤𑄬"), ("ceb", "Medway (kondado)"), ("da", "Medway"), ("de", "Borough of Medway"), ("en", "Medway"), ("es", "Medway"), ("fr", "Medway"), ("ga", "Medway"), ("gu", "મ\u{ac7}ડવ\u{ac7}"), ("it", "Medway"), ("ja", "メドウェイ"), ("kn", "ಮ\u{cc6}ಡ\u{ccd}ವೇ"), ("ko", "메드웨이"), ("nb", "Medway"), ("nl", "Medway"), ("no", "Medway"), ("pl", "Medway"), ("pt", "Medway"), ("ro", "Medway"), ("ru", "Медуэй"), ("sv", "Medway"), ("ta", "மெட\u{bcd}வே"), ("te", "మ\u{c3f}డ\u{c4d} వ\u{c47}"), ("tr", "medway"), ("uk", "Медвей (район)"), ("ur", "میڈوے"), ("zh", "梅德韋")]),
                        unofficial_name_list: ["Medway"].to_vec(),
                    }
                ),
                (
                    "MEA",
                    Subdivision{
                        name: "Mid and East Antrim",
                        country_alpha2: Alpha2::GB,
                        code: "MEA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "মিড এবং ইস\u{9cd}ট এন\u{9cd}ট\u{9cd}র\u{9be}ম"), ("ccp", "𑄟\u{11128}𑄖\u{11134} 𑄃\u{11133}𑄃 𑄃\u{11128}𑄌\u{11134}𑄑\u{11134} 𑄃𑄚\u{11134}𑄑\u{11133}𑄢\u{11128}𑄟\u{11134}"), ("de", "Mid and East Antrim"), ("en", "Mid and East Antrim"), ("es", "Mid and East Antrim"), ("fa", "مید و انتریم شرقی"), ("fr", "Mid and East Antrim"), ("gu", "મધ\u{acd}ય અન\u{ac7} પ\u{ac2}ર\u{acd}વ ઍન\u{acd}ટ\u{acd}રિમ"), ("it", "Distretto di Mid e East Antrim"), ("ja", "ミッド・アンド・イースト・アントリム"), ("kn", "ಮ\u{cbf}ಡ\u{ccd} ಮತ\u{ccd}ತು ಈಸ\u{ccd}ಟ\u{ccd} ಆಂಟ\u{ccd}ರ\u{cbf}ಮ\u{ccd}"), ("ko", "중부와 동부 앤트림"), ("lt", "Midas"), ("nl", "Mid and East Antrim"), ("pt", "Mid and East Antrim"), ("ru", "Средний и Восточный Антрим"), ("ta", "நடு கிழக\u{bcd}கு அன\u{bcd}ரிம\u{bcd}"), ("te", "మధ\u{c4d}య మర\u{c3f}యు తూర\u{c4d}పు ఆంట\u{c4d}ర\u{c3f}మ\u{c4d},"), ("ur", "وسطی اور مشرقی انٹریم")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MIK",
                    Subdivision{
                        name: "Milton Keynes",
                        country_alpha2: Alpha2::GB,
                        code: "MIK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.0406224), longitude: Some(-0.7594171), max_latitude: Some(52.0526971), min_latitude: Some(52.0235794), max_longitude: Some(-0.7345839), min_longitude: Some(-0.7807044)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "মিল\u{9cd}টন কিনস"), ("ccp", "𑄟\u{11128}𑄣\u{11134}𑄑\u{11127}𑄚\u{11134} 𑄇\u{11128}𑄚𑄬𑄌\u{11134}"), ("ceb", "Milton Keynes"), ("de", "Borough of Milton Keynes"), ("en", "Milton Keynes"), ("es", "Municipio de Milton Keynes"), ("et", "Milton Keynes"), ("fr", "Milton Keynes (borough)"), ("gu", "મિલ\u{acd}ટન ક\u{ac7}ઇન\u{acd}સ"), ("it", "Milton Keynes"), ("ja", "ミルトンキーンズ"), ("kn", "ಮ\u{cbf}ಲ\u{ccd}ಟನ\u{ccd} ಕೀನ\u{ccd}ಸ\u{ccd}"), ("ko", "밀턴킨즈 구"), ("nb", "Milton Keynes"), ("nl", "Milton Keynes"), ("no", "Milton Keynes"), ("pl", "Milton Keynes (borough)"), ("pt", "Milton Keynes"), ("ru", "Милтон-Кинс (унитарная единица)"), ("sv", "Milton Keynes"), ("ta", "மில\u{bcd}டன\u{bcd} கேயனெஸ\u{bcd}"), ("te", "మ\u{c3f}ల\u{c4d}టన\u{c4d} క\u{c40}న\u{c4d}స\u{c4d}"), ("uk", "Мілтон-Кінз"), ("ur", "بورو ملٹن کینز")]),
                        unofficial_name_list: ["Milton Keynes"].to_vec(),
                    }
                ),
                (
                    "MLN",
                    Subdivision{
                        name: "Midlothian",
                        country_alpha2: Alpha2::GB,
                        code: "MLN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.8292247), longitude: Some(-3.1338428), max_latitude: Some(55.9320234), min_latitude: Some(55.7103492), max_longitude: Some(-2.8462381), min_longitude: Some(-3.3690393)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميدلوثيان"), ("be", "Мідлотыян"), ("bg", "Мидлоудиън"), ("bn", "মিডলোথিয\u{9bc}\u{9be}ন"), ("ca", "Midlothian"), ("ccp", "𑄟\u{11128}𑄖\u{11134}𑄣\u{1112e}𑄗\u{11128}𑄠𑄚\u{11134}"), ("ceb", "Midlothian"), ("cs", "Střední Lothian"), ("cy", "Midlothian"), ("da", "Midlothian"), ("de", "Midlothian"), ("el", "Μιντλόδιαν"), ("en", "Midlothian"), ("es", "Midlothian"), ("et", "Midlothian"), ("eu", "Midlothian"), ("fa", "میدلودین"), ("fi", "Midlothian"), ("fr", "Midlothian"), ("ga", "Labhaidh Láir"), ("gu", "મિડલોથિઆન"), ("it", "Midlothian"), ("ja", "ミッドロージアン"), ("kn", "ಮ\u{cbf}ಡ\u{ccd}ಲೊಥ\u{cbf}ಯನ\u{ccd}"), ("ko", "미들로디언"), ("lt", "Midlotianas"), ("nb", "Midlothian"), ("nl", "Midlothian"), ("no", "Midlothian"), ("pl", "Midlothian"), ("pt", "Midlothian"), ("ro", "Midlothian"), ("ru", "Мидлотиан"), ("sl", "Midlothian"), ("sv", "Midlothian"), ("ta", "மிடலோதிய\u{bbe}ன\u{bcd}"), ("te", "మ\u{c3f}డ\u{c4d}ల\u{c4b}థ\u{c3f}య\u{c3e}న\u{c4d}"), ("uk", "Середній Лотіан"), ("ur", "میدلوتھیان"), ("zh", "中洛锡安")]),
                        unofficial_name_list: ["Midlothian"].to_vec(),
                    }
                ),
                (
                    "MON",
                    Subdivision{
                        name: "Monmouthshire [Sir Fynwy GB-FYN]",
                        country_alpha2: Alpha2::GB,
                        code: "MON",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8157729), longitude: Some(-3.0123791), max_latitude: Some(51.8200766), min_latitude: Some(51.8102124), max_longitude: Some(-3.00958), min_longitude: Some(-3.0161242)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Монмутшыр"), ("bg", "Мънмътшър"), ("bn", "মনমউথশ\u{9be}য\u{9bc}র"), ("ca", "Monmouthshire"), ("ccp", "𑄟𑄚\u{11134}𑄟𑄅\u{1112a}𑄖\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Monmouthshire (munisipyo sa Hiniusang Gingharian)"), ("cs", "Monmouthshire"), ("cy", "Sir Fynwy"), ("de", "Monmouthshire"), ("en", "Monmouthshire"), ("es", "Monmouthshire"), ("et", "Monmouthshire"), ("eu", "Monmouthshire"), ("fa", "مونموت\u{200c}شر"), ("fi", "Monmouthshire"), ("fr", "Monmouthshire"), ("ga", "Sir Fynwy"), ("gl", "Sir Fynwy"), ("gu", "મોનમાઉથશાયર"), ("he", "מונמאות׳שייר"), ("hi", "मॉनमाउथशायर"), ("id", "Monmouthshire"), ("it", "Monmouthshire"), ("ja", "モンマスシャー"), ("kn", "ಮೊನ\u{ccd}ಮ\u{ccc}ತ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "몬머스셔"), ("lt", "Monmutšyras"), ("nb", "Monmouthshire"), ("nl", "Monmouthshire"), ("no", "Monmouthshire"), ("pl", "Monmouthshire"), ("pt", "Monmouthshire"), ("ro", "Monmouthshire"), ("ru", "Монмутшир"), ("sv", "Monmouthshire"), ("ta", "மொன\u{bcd}மொஉத\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", "మ\u{c4b}న\u{c4d}మ\u{c4c}త\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Монмутшир"), ("ur", "مونمووتھشائر"), ("vi", "Monmouthshire"), ("zh", "蒙茅斯郡")]),
                        unofficial_name_list: ["Sir Fynwy"].to_vec(),
                    }
                ),
                (
                    "MRT",
                    Subdivision{
                        name: "Merton",
                        country_alpha2: Alpha2::GB,
                        code: "MRT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.891854), longitude: Some(-4.095316), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Мертан"), ("bn", "মেটন"), ("ca", "Merton"), ("ccp", "𑄟𑄬𑄢\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Merton (distrito)"), ("cs", "Merton"), ("cy", "Merton"), ("da", "Merton"), ("de", "London Borough of Merton"), ("el", "Λονδρέζικος Δήμος του Μέρτον"), ("en", "Merton"), ("es", "Merton"), ("eu", "Merton"), ("fa", "منطقه مرتون لندن"), ("fi", "Merton"), ("fr", "district londonien de Merton"), ("ga", "Buirg Londan Merton"), ("gu", "લ\u{a82}ડન બોરો ઓફ મ\u{ac7}ર\u{acd}ટન"), ("he", "מרטון"), ("hi", "मर\u{94d}टन बरो"), ("hu", "Merton kerület"), ("hy", "Մերտոն"), ("is", "Merton"), ("it", "Merton"), ("ja", "マートン・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಮ\u{cc6}ರ\u{ccd}ಟನ\u{ccd}"), ("ko", "머턴 구"), ("lv", "Mertona"), ("nb", "Merton"), ("nl", "Merton"), ("no", "Merton"), ("pl", "London Borough of Merton"), ("pt", "Merton"), ("ro", "Merton"), ("ru", "Мертон"), ("sr", "Лондонска општина Мертон"), ("sr_Latn", "Londonska opština Merton"), ("sv", "London Borough of Merton"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} மெர\u{bcd}டோன\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} మ\u{c46}ర\u{c4d}టన\u{c4d}"), ("tr", "Merton"), ("uk", "Мертон"), ("ur", "مرٹن بورو"), ("vi", "Khu Merton của Luân Đôn"), ("zh", "默頓區")]),
                        unofficial_name_list: ["Merton"].to_vec(),
                    }
                ),
                (
                    "MRY",
                    Subdivision{
                        name: "Moray",
                        country_alpha2: Alpha2::GB,
                        code: "MRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(57.511548), longitude: Some(-3.2483773), max_latitude: Some(57.7300454), min_latitude: Some(57.06845360000001), max_longitude: Some(-2.6498225), min_longitude: Some(-3.7647451)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Мъри"), ("bn", "মোরে"), ("ca", "Consell de Moray"), ("ccp", "𑄟\u{11127}𑄢𑄬"), ("ceb", "Moray"), ("cs", "Moray"), ("cy", "Moray"), ("de", "Moray"), ("en", "Moray"), ("es", "Moray"), ("et", "Moray"), ("eu", "Moray"), ("fi", "Moray"), ("fr", "Moray"), ("ga", "Comhairle Mhoireibh"), ("gu", "મોર\u{ac7}"), ("he", "מוריי, סקוטלנד"), ("it", "Moray"), ("ja", "マレー"), ("kk", "Мори"), ("kn", "ಮೊರೇ"), ("ko", "머리"), ("lt", "Morėjus"), ("mk", "Мари"), ("nb", "Moray"), ("nl", "Moray"), ("no", "Moray"), ("pl", "Moray"), ("pt", "Moray"), ("ro", "Moray"), ("ru", "Мори"), ("sv", "Moray"), ("ta", "முறை"), ("te", "మ\u{c4b}ర\u{c47}"), ("uk", "Морей"), ("ur", "موڑے"), ("zh", "馬里")]),
                        unofficial_name_list: ["Moray"].to_vec(),
                    }
                ),
                (
                    "MTY",
                    Subdivision{
                        name: "Merthyr Tydfil [Merthyr Tudful GB-MTU]",
                        country_alpha2: Alpha2::GB,
                        code: "MTY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.74872999999999), longitude: Some(-3.381646), max_latitude: Some(51.7696031), min_latitude: Some(51.73217229999999), max_longitude: Some(-3.3561396), min_longitude: Some(-3.400272)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Мърдър Тидфил (град)"), ("bn", "ম\u{9be}র\u{9cd}থ\u{9be}র টিডভিল"), ("ccp", "𑄟𑄬𑄢\u{11134}𑄗𑄠𑄢\u{11134} 𑄑\u{1112d}𑄖\u{11134}𑄜\u{11128}𑄣\u{11134}"), ("ceb", "Merthyr Tydfil County Borough"), ("cy", "Cyngor Bwrdeisdref Merthyr Tudful"), ("de", "Merthyr Tydfil"), ("en", "Merthyr Tydfil"), ("es", "Merthyr Tydfil"), ("eu", "Merthyr Tydfil"), ("fa", "شهرستان مستقل مرثر تیدویل"), ("fr", "Merthyr Tydfil"), ("gu", "મ\u{ac7}ર\u{acd}થીર ટાઇદફિલ કાઉન\u{acd}ટી બોરો"), ("it", "distretto di contea di Merthyr Tydfil"), ("ja", "マーサー・ティドビル"), ("kn", "ಮ\u{cc6}ರ\u{ccd}ಥ\u{cbf}ರ\u{ccd} ಟೈಡ\u{ccd}ಫ\u{cbf}ಲ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "머서티드빌 자치시"), ("pl", "Merthyr Tydfil"), ("pt", "Merthyr Tydfil County Borough"), ("ru", "Мертир-Тидвил"), ("sv", "Merthyr Tydfil County Borough"), ("ta", "மெர\u{bcd}த\u{bcd}திற\u{bcd} டிடபில\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "మ\u{c46}ర\u{c4d}త\u{c3f}ర\u{c4d} ట\u{c48}డ\u{c4d}ఫ\u{c3f}ల\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("uk", "Мертір-Тідвіл")]),
                        unofficial_name_list: ["Merthyr Tudful"].to_vec(),
                    }
                ),
                (
                    "MUL",
                    Subdivision{
                        name: "Mid Ulster",
                        country_alpha2: Alpha2::GB,
                        code: "MUL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "মিড-উলস\u{9cd}ট\u{9be}র"), ("ccp", "𑄟\u{11128}𑄖\u{11134} 𑄃𑄣\u{11134}𑄑𑄢\u{11134}"), ("de", "Mid Ulster"), ("en", "Mid Ulster"), ("es", "Mid Ulster"), ("fa", "بخش مید-اولستر"), ("fr", "Mid-Ulster District"), ("ga", "Lár Uladh"), ("gu", "મિડ-અલ\u{acd}સ\u{acd}ટર"), ("it", "Distretto di Mid-Ulster"), ("ja", "ミッド・アルスター"), ("kn", "ಮ\u{cbf}ಡ\u{ccd}-ಅಲ\u{ccd}ಸ\u{ccd}ಟರ\u{ccd}"), ("ko", "중부 얼스터"), ("nl", "Mid Ulster"), ("pt", "Mid-Ulster"), ("ru", "Мидлсбро²"), ("ta", "மிட\u{bcd} -அல\u{bcd}ஸ\u{bcd}டர\u{bcd}"), ("te", "మ\u{c3f}డ\u{c4d}-ఉల\u{c4d}స\u{c4d}టర\u{c4d}"), ("ur", "وسطی-السٹر ضلع")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NAY",
                    Subdivision{
                        name: "North Ayrshire",
                        country_alpha2: Alpha2::GB,
                        code: "NAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.7113902), longitude: Some(-4.729983400000001), max_latitude: Some(55.89261370000001), min_latitude: Some(55.4242441), max_longitude: Some(-4.4924741), min_longitude: Some(-5.397798799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Паўночны Эйршыр"), ("bg", "Северен Еършър"), ("bn", "নর\u{9cd}থ আয\u{9bc}\u{9be}রশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "North Ayrshire"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄃𑄠𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "North Ayrshire"), ("cs", "Severní Ayrshire"), ("cy", "Gogledd Swydd Ayr"), ("de", "North Ayrshire"), ("en", "North Ayrshire"), ("es", "North Ayrshire"), ("et", "North Ayrshire"), ("eu", "Iparraldeko Ayrshire"), ("fa", "ایرشر شمالی"), ("fi", "Pohjois-Ayrshire"), ("fr", "North Ayrshire"), ("ga", "Comhairle Shiorrachd Áir a Tuath"), ("gu", "ઉત\u{acd}તર આયરશાયર"), ("he", "צפון איירשייר"), ("hr", "Sjeverni Ayrshire"), ("hu", "North Ayrshire"), ("it", "Ayrshire Settentrionale"), ("ja", "ノース・エアシャー"), ("kk", "Норт-Эршир"), ("kn", "ಉತ\u{ccd}ತರ ಐರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "노스에어셔"), ("lt", "Šiaurės Eršyras"), ("nb", "North Ayrshire"), ("nl", "North Ayrshire"), ("no", "North Ayrshire"), ("pl", "North Ayrshire"), ("pt", "North Ayrshire"), ("ro", "North Ayrshire"), ("ru", "Северный Эйршир"), ("sv", "North Ayrshire"), ("ta", "வடக\u{bcd}கு அயர\u{bcd}ஸிர\u{bcd}"), ("te", "ఉత\u{c4d}తర ఆయర\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Північний Ершир"), ("ur", "شمالی آئرشائر"), ("zh", "北艾尔郡")]),
                        unofficial_name_list: ["North Ayrshire"].to_vec(),
                    }
                ),
                (
                    "NBL",
                    Subdivision{
                        name: "Northumberland",
                        country_alpha2: Alpha2::GB,
                        code: "NBL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.2082542), longitude: Some(-2.0784138), max_latitude: Some(55.8110863), min_latitude: Some(54.7823704), max_longitude: Some(-1.4603163), min_longitude: Some(-2.689785)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Northumberland"), ("ar", "نورثمبرلاند"), ("be", "Нартумберленд"), ("bg", "Нортъмбърланд"), ("bn", "নদ\u{9be}মব\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Northumberland"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄗𑄟\u{11134}𑄝𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Northumberland (kondado)"), ("cs", "Northumberland"), ("cy", "Northumberland"), ("da", "Northumberland"), ("de", "Northumberland"), ("el", "Νορθάμπερλαντ"), ("en", "Northumberland"), ("es", "Northumberland"), ("et", "Northumberland"), ("eu", "Northumberland"), ("fa", "نورث\u{200c}آمبرلند"), ("fi", "Northumberland"), ("fr", "Northumberland"), ("ga", "Northumberland"), ("gu", "ઉત\u{acd}તરઅમ\u{acd}બરલ\u{ac7}ન\u{acd}ડ"), ("he", "נורת׳מברלנד"), ("hi", "नॉर\u{94d}थम\u{94d}बरल\u{948}\u{902}ड"), ("hu", "Northumberland"), ("hy", "Նորթամբերլենդ"), ("id", "Northumberland"), ("is", "Norðymbraland"), ("it", "Northumberland"), ("ja", "ノーサンバーランド"), ("kn", "ನಾರ\u{ccd}ಥಂಬರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "노섬벌랜드 주"), ("lt", "Nortamberlandas"), ("lv", "Nortamberlenda"), ("mk", "Нортамберленд"), ("mr", "नॉर\u{94d}थअ\u{902}बरल\u{901}ड"), ("nb", "Northumberland"), ("nl", "Northumberland"), ("no", "Northumberland"), ("pl", "Northumberland"), ("pt", "Northumberland"), ("ro", "Northumberland"), ("ru", "Нортамберленд"), ("sd", "نارٿمبر لينڊ"), ("sk", "Northumberland"), ("sl", "Northumberland"), ("sr", "Нортамберланд"), ("sr_Latn", "Nortamberland"), ("sv", "Northumberland"), ("ta", "நோர\u{bcd}த\u{bcd}தும\u{bcd}பேர\u{bcd}லண\u{bcd}ட\u{bcd}"), ("te", "న\u{c3e}ర\u{c4d}తంబర\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "นอร\u{e4c}ท\u{e31}มเบอร\u{e4c}แลนด\u{e4c}"), ("tr", "Northumberland"), ("uk", "Нортумберленд"), ("ur", "نارتھمبرلینڈ"), ("vi", "Northumberland"), ("yue", "諾森伯倫郡"), ("yue_Hans", "诺森伯伦郡"), ("zh", "諾森伯蘭郡")]),
                        unofficial_name_list: ["Northumberland"].to_vec(),
                    }
                ),
                (
                    "NEL",
                    Subdivision{
                        name: "North East Lincolnshire",
                        country_alpha2: Alpha2::GB,
                        code: "NEL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5418558), longitude: Some(-0.1263736), max_latitude: Some(53.6399645), min_latitude: Some(53.4335754), max_longitude: Some(0.0250924), min_longitude: Some(-0.2921103)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Паўночна-Усходні Лінкальншыр"), ("bn", "উত\u{9cd}তর প\u{9c2}র\u{9cd}ব লিঙ\u{9cd}কনশ\u{9be}য\u{9bc}\u{9be}র"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{1112a}𑄇\u{11134} 𑄣\u{11128}𑄚\u{11134}𑄇\u{1112e}𑄣\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "North East Lincolnshire"), ("de", "North East Lincolnshire"), ("en", "North East Lincolnshire"), ("es", "North East Lincolnshire"), ("fr", "Lincolnshire du Nord-Est"), ("gu", "ઉત\u{acd}તર પ\u{ac2}ર\u{acd}વ લિ\u{a82}કનશાયર"), ("it", "North East Lincolnshire"), ("ja", "ノース・イースト・リンカンシャー"), ("kn", "ನಾರ\u{ccd}ತ\u{ccd} ಈಸ\u{ccd}ಟ\u{ccd} ಲ\u{cbf}ಂಕನ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "노스이스트링컨셔"), ("nb", "North East Lincolnshire"), ("nl", "North East Lincolnshire"), ("no", "North East Lincolnshire"), ("pl", "North East Lincolnshire"), ("pt", "North East Lincolnshire"), ("ro", "North East Lincolnshire"), ("ru", "Северо-Восточный Линкольншир"), ("sv", "North East Lincolnshire"), ("ta", "வட கிழக\u{bcd}கு லிஙக\u{bbe}ன\u{bcd}ஷிர\u{bbe}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} ఈస\u{c4d}ట\u{c4d} ల\u{c3f}ంకన\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Північно-Східний Лінкольншир"), ("ur", "شمالی مشرقی لنکنشائر"), ("zh", "东北林肯郡")]),
                        unofficial_name_list: ["North East Lincolnshire"].to_vec(),
                    }
                ),
                (
                    "NET",
                    Subdivision{
                        name: "Newcastle upon Tyne",
                        country_alpha2: Alpha2::GB,
                        code: "NET",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.978252), longitude: Some(-1.61778), max_latitude: Some(55.0453044), min_latitude: Some(54.9594399), max_longitude: Some(-1.5326051), min_longitude: Some(-1.7810817)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Newcastle upon Tyne"), ("ar", "نيوكاسل أبون تاين"), ("az", "Nyukasl apon Tayn"), ("be", "Ньюкасл-эпон-Тайн"), ("bg", "Нюкасъл ъпон Тайн"), ("bn", "নিউক\u{9cd}য\u{9be}সল আপন ট\u{9be}ইন"), ("ca", "Newcastle upon Tyne"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄇\u{11133}𑄠𑄥\u{11127}𑄣\u{11134} 𑄃𑄛\u{11127}𑄚\u{11134} 𑄑\u{1112d}𑄚\u{11134}"), ("ceb", "Newcastle upon Tyne"), ("cs", "Newcastle upon Tyne"), ("da", "Newcastle upon Tyne"), ("de", "Newcastle upon Tyne"), ("el", "Νιούκασλ"), ("en", "Newcastle upon Tyne"), ("es", "Newcastle upon Tyne"), ("et", "Newcastle upon Tyne"), ("eu", "Newcastle upon Tyne"), ("fa", "نیوکاسل آپون تاین"), ("fi", "Newcastle upon Tyne"), ("fr", "Newcastle upon Tyne"), ("gl", "Newcastle upon Tyne"), ("gu", "ન\u{acd}ય\u{ac2}ક\u{ac7}સલ અપોન ટાઇન"), ("he", "ניוקאסל"), ("hi", "न\u{94d}य\u{942}क\u{948}सल अपॉन टाइन नोड"), ("hr", "Newcastle upon Tyne"), ("hu", "Newcastle upon Tyne"), ("hy", "Նյուքասլ-ափոն-Թայն"), ("id", "Newcastle upon Tyne"), ("is", "Newcastle upon Tyne"), ("it", "Newcastle upon Tyne"), ("ja", "ニューカッスル・アポン・タイン"), ("ka", "ნიუკასლ-აპონ-ტაინი"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ಕ\u{ccd}ಯಾಸಲ\u{ccd} ಅಪಾನ ಟೈನ\u{ccd}"), ("ko", "뉴캐슬어폰타인"), ("lt", "Niukaslas prie Taino"), ("lv", "Ņūkāsla pie Tainas"), ("mn", "Ньюкасл"), ("mr", "न\u{94d}य\u{942}क\u{945}सल अपॉन टाईन"), ("ms", "Newcastle upon Tyne"), ("nb", "Newcastle upon Tyne"), ("nl", "Newcastle upon Tyne"), ("no", "Newcastle upon Tyne"), ("pl", "Newcastle upon Tyne"), ("pt", "Newcastle upon Tyne"), ("ro", "Newcastle-upon-Tyne"), ("ru", "Ньюкасл-апон-Тайн"), ("si", "න\u{dd2}ව\u{dca}ක\u{dcf}සල\u{dca} අපෝන\u{dca} ටය\u{dd2}න\u{dca}"), ("sk", "Newcastle upon Tyne"), ("sl", "Newcastle upon Tyne"), ("sr", "Њукасл на Тајну"), ("sr_Latn", "Njukasl na Tajnu"), ("sv", "Newcastle upon Tyne"), ("sw", "Newcastle upon Tyne"), ("ta", "டைன\u{bcd} ஆற\u{bcd}றங\u{bcd}கரை நியூ க\u{bbe}சில\u{bcd}"), ("te", "న\u{c4d}యూక\u{c3e}జ\u{c3f}ల\u{c4d} అప\u{c3e}న\u{c4d} ట\u{c48}న\u{c4d}"), ("th", "น\u{e34}วคาสเซ\u{e34}ลอะพอนไทน\u{e4c}"), ("tr", "Newcastle upon Tyne"), ("uk", "Ньюкасл-апон-Тайн"), ("ur", "نیوکیسل اپون ٹائین"), ("vi", "Newcastle trên sông Tyne"), ("zh", "泰恩河畔纽卡斯尔")]),
                        unofficial_name_list: ["Newcastle upon Tyne"].to_vec(),
                    }
                ),
                (
                    "NFK",
                    Subdivision{
                        name: "Norfolk",
                        country_alpha2: Alpha2::GB,
                        code: "NFK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6139686), longitude: Some(0.8864021), max_latitude: Some(52.9894423), min_latitude: Some(52.355367), max_longitude: Some(1.7454611), min_longitude: Some(0.1535552)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Norfolk"), ("ar", "نورفولك"), ("az", "Norfolk"), ("be", "Графства Норфалк"), ("bg", "Норфолк"), ("bn", "নরফোক"), ("ca", "Norfolk"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄛\u{1112e}𑄇\u{11134}"), ("ceb", "Norfolk (kondado)"), ("cs", "Norfolk"), ("cy", "Norfolk"), ("da", "Norfolk"), ("de", "Norfolk"), ("el", "Νόρφολκ"), ("en", "Norfolk"), ("es", "Norfolk"), ("et", "Norfolk"), ("eu", "Norfolk"), ("fa", "نورفک"), ("fi", "Norfolk"), ("fr", "Norfolk"), ("ga", "Norfolk"), ("gl", "Condado de Norfolk, Inglaterra"), ("gu", "નોર\u{acd}ફોલ\u{acd}ક"), ("he", "נורפוק"), ("hi", "नॉर\u{94d}फ\u{93c}क"), ("hu", "Norfolk"), ("hy", "Նորֆոլկ"), ("id", "Norfolk"), ("is", "Norfolk"), ("it", "Norfolk"), ("ja", "ノーフォーク"), ("ka", "ნორფოლკი"), ("kn", "ನಾರ\u{ccd}ಫೋಕ\u{ccd}"), ("ko", "노퍽 주"), ("lt", "Norfolkas"), ("lv", "Norfolka"), ("mk", "Норфолк"), ("mr", "नॉरफोक"), ("nb", "Norfolk"), ("nl", "Norfolk"), ("no", "Norfolk"), ("pl", "Norfolk"), ("pt", "Norfolk"), ("ro", "Norfolk"), ("ru", "Норфолк"), ("sk", "Norfolk"), ("sl", "Norfolk"), ("sr", "Норфок"), ("sr_Latn", "Norfok"), ("sv", "Norfolk"), ("sw", "Norfolk"), ("ta", "ந\u{bbe}ரபோல\u{bcd}க\u{bcd}"), ("te", "న\u{c4b}ర\u{c4d}ఫ\u{c4b}క\u{c4d}"), ("th", "นอร\u{e4c}ฟอล\u{e4c}ก"), ("tr", "Norfolk"), ("uk", "Норфолк"), ("ur", "نارفوک"), ("vi", "Norfolk"), ("yue", "諾福郡"), ("yue_Hans", "诺福郡"), ("zh", "诺福克郡")]),
                        unofficial_name_list: ["Norfolk"].to_vec(),
                    }
                ),
                (
                    "NGM",
                    Subdivision{
                        name: "Nottingham",
                        country_alpha2: Alpha2::GB,
                        code: "NGM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.95478319999999), longitude: Some(-1.1581086), max_latitude: Some(53.019045), min_latitude: Some(52.889), max_longitude: Some(-1.0918336), min_longitude: Some(-1.2482899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nottingham"), ("am", "ኖቲንግሃም"), ("ar", "نوتنغهام"), ("az", "Nottingem"), ("be", "Горад Нотынгем"), ("bg", "Нотингам"), ("bn", "নটিংহ\u{9be}ম"), ("bs", "Nottingham"), ("ca", "Nottingham"), ("ccp", "𑄚\u{11127}𑄑\u{11133}𑄦\u{11128}\u{11101}𑄦𑄟\u{11134}"), ("ceb", "City of Nottingham"), ("cs", "Nottingham"), ("cy", "Nottingham"), ("da", "Nottingham"), ("de", "Nottingham"), ("el", "Νότιγχαμ"), ("en", "Nottingham"), ("es", "Nottingham"), ("et", "Nottingham"), ("eu", "Nottingham"), ("fa", "ناتینگهام"), ("fi", "Nottingham"), ("fr", "Nottingham"), ("ga", "Nottingham"), ("gl", "Nottingham"), ("gu", "નોટિ\u{a82}ઘમ"), ("ha", "Nottingham"), ("ha_NE", "Nottingham"), ("he", "נוטינגהאם"), ("hi", "नॉटि\u{902}घम"), ("hr", "Nottingham"), ("hu", "Nottingham"), ("hy", "Նոթինգհեմ"), ("id", "Nottingham"), ("is", "Nottingham"), ("it", "Nottingham"), ("ja", "ノッティンガム"), ("ka", "ნოტინგემი"), ("kn", "ನಾಟ\u{cbf}ಂಗ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}"), ("ko", "노팅엄"), ("lt", "Notingemas"), ("lv", "Notingema"), ("mk", "Нотингем"), ("ml", "നോട\u{d4d}ടിങ\u{d4d}ഹ\u{d3e}ം"), ("mn", "Ноттингем"), ("mr", "नॉटि\u{902}गह\u{945}म"), ("ms", "Nottingham"), ("nb", "Nottingham"), ("nl", "Nottingham"), ("no", "Nottingham"), ("pl", "Nottingham"), ("pt", "Nottingham"), ("ro", "Nottingham"), ("ru", "Ноттингем"), ("si", "නොට\u{dd2}ංහැම\u{dca}"), ("sk", "Nottingham"), ("sl", "Nottingham"), ("sr", "Нотингем"), ("sr_Latn", "Notingem"), ("sv", "Nottingham"), ("sw", "Nottingham"), ("ta", "நோட\u{bcd}டிங\u{bcd}க\u{bbe}ம\u{bcd}"), ("te", "న\u{c3e}ట\u{c3f}ంగ\u{c4d}\u{200c}హ\u{c3e}మ\u{c4d}"), ("th", "นอตท\u{e34}งแฮม"), ("tr", "Nottingham"), ("uk", "Ноттінгем"), ("ur", "ناٹنگھم"), ("uz", "Nottingem"), ("vi", "Nottingham"), ("yue", "諾定咸"), ("yue_Hans", "诺定咸"), ("zh", "诺丁汉")]),
                        unofficial_name_list: ["Nottingham"].to_vec(),
                    }
                ),
                (
                    "NIR",
                    Subdivision{
                        name: "Northern Ireland",
                        country_alpha2: Alpha2::GB,
                        code: "NIR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Ierland"), ("am", "ስሜን አየርላንድ"), ("ar", "أيرلندا الشمالية"), ("az", "Şimali İrlandiya"), ("be", "Паўночная Ірландыя"), ("bg", "Северна Ирландия"), ("bn", "উত\u{9cd}তর আয\u{9bc}\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Sjeverna Irska"), ("ca", "Irlanda del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄃𑄠𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Northern Ireland"), ("cs", "Severní Irsko"), ("cy", "Gogledd Iwerddon"), ("da", "Nordirland"), ("de", "Nordirland"), ("el", "Βόρεια Ιρλανδία"), ("en", "Northern Ireland"), ("es", "Irlanda del Norte"), ("et", "Põhja-Iirimaa"), ("eu", "Ipar Irlanda"), ("fa", "ایرلند شمالی"), ("fi", "Pohjois-Irlanti"), ("fr", "Irlande du Nord"), ("ga", "Tuaisceart Éireann"), ("gl", "Irlanda do Norte"), ("gu", "નોર\u{acd}ધન આયર\u{acd}લ\u{ac7}ન\u{acd}ડ"), ("he", "צפון אירלנד"), ("hi", "उत\u{94d}तरी आयरल\u{948}\u{902}ड"), ("hr", "Sjeverna Irska"), ("hu", "Észak-Írország"), ("hy", "Հյուսիսային Իռլանդիա"), ("id", "Irlandia Utara"), ("is", "Norður-Írland"), ("it", "Irlanda del Nord"), ("ja", "北アイルランド"), ("jv", "Irlandia Lor"), ("ka", "ჩრდილოეთი ირლანდია"), ("kk", "Солтүстік Ирландия"), ("km", "អៀរឡង\u{17cb} ខាងជើង"), ("kn", "ಉತ\u{ccd}ತರ ಐರ\u{ccd}ಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "북아일랜드"), ("ky", "Түндүк Ирландия"), ("lt", "Šiaurės Airija"), ("lv", "Ziemeļīrija"), ("mk", "Северна Ирска"), ("ml", "വടക\u{d4d}കൻ അയർലണ\u{d4d}ട\u{d4d}"), ("mn", "Умард Ирланд"), ("mr", "उत\u{94d}तर आयर\u{94d}ल\u{902}ड"), ("ms", "Ireland Utara"), ("nb", "Nord-Irland"), ("ne", "उत\u{94d}तरी आयरल\u{94d}यान\u{94d}ड"), ("nl", "Noord-Ierland"), ("no", "Nord-Irland"), ("pa", "ਉ\u{a71}ਤਰੀ ਆਇਰਲ\u{a48}\u{a02}ਡ"), ("pl", "Irlandia Północna"), ("pt", "Irlanda do Norte"), ("ro", "Irlanda de Nord"), ("ru", "Северная Ирландия"), ("si", "උත\u{dd4}ර\u{dd4} අයර\u{dca}ලන\u{dca}තය"), ("sk", "Severné Írsko"), ("sl", "Severna Irska"), ("so", "Waqooyiga Ayrland"), ("sq", "Irlanda Veriore"), ("sr", "Северна Ирска"), ("sr_Latn", "Severna Irska"), ("sv", "Nordirland"), ("sw", "Eire ya Kaskazini"), ("ta", "வட அயர\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "ఉత\u{c4d}తర ఐర\u{c4d}లండ\u{c4d}"), ("th", "นอร\u{e4c}เท\u{e34}ร\u{e4c}นไอร\u{e4c}แลนด\u{e4c}"), ("tr", "Kuzey İrlanda"), ("uk", "Північна Ірландія"), ("ur", "شمالی آئرلینڈ"), ("uz", "Shimoliy Irlandiya"), ("vi", "Bắc Ireland"), ("yo", "Írẹ\u{301}lándì Apáàríwá"), ("yo_BJ", "Írɛ\u{301}lándì Apáàríwá"), ("yue", "北愛爾蘭"), ("yue_Hans", "北爱尔兰"), ("zh", "北爱尔兰")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NLK",
                    Subdivision{
                        name: "North Lanarkshire",
                        country_alpha2: Alpha2::GB,
                        code: "NLK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.8289139), longitude: Some(-3.9221958), max_latitude: Some(56.0313682), min_latitude: Some(55.7347607), max_longitude: Some(-3.711135), min_longitude: Some(-4.1945028)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Северен Ланаркшър"), ("bn", "উত\u{9cd}তর ল\u{9be}রন\u{9be}কশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "North Lanarkshire"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄣\u{11127}𑄚𑄢\u{11134}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "North Lanarkshire"), ("cs", "Severní Lanarkshire"), ("cy", "Gogledd Swydd Lanark"), ("da", "North Lanarkshire"), ("de", "North Lanarkshire"), ("en", "North Lanarkshire"), ("es", "North Lanarkshire"), ("et", "North Lanarkshire"), ("eu", "Iparraldeko Lanarkshire"), ("fa", "لانارکشر شمالی"), ("fi", "Pohjois-Lanarkshire"), ("fr", "North Lanarkshire"), ("ga", "Comhairle Shiorrachd Lannraig a Tuath"), ("gu", "ઉત\u{acd}તર લ\u{ac7}નાર\u{acd}કશાયર"), ("hr", "Sjeverni Lanarkshire"), ("id", "North Lanarkshire"), ("is", "Norður-Lanarkshire"), ("it", "Lanarkshire Settentrionale"), ("ja", "ノース・ラナークシャー"), ("kn", "ನಾರ\u{ccd}ತ\u{ccd} ಲ\u{ccd}ಯಾನಾರ\u{ccd}ಕ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "노스래너크셔"), ("lt", "Šiaurės Lanarkšyras"), ("nb", "North Lanarkshire"), ("nl", "North Lanarkshire"), ("no", "North Lanarkshire"), ("pl", "North Lanarkshire"), ("pt", "North Lanarkshire"), ("ro", "North Lanarkshire"), ("ru", "Норт-Ланаркшир"), ("sv", "North Lanarkshire"), ("ta", "வடக\u{bcd}கு ல\u{bbe}ன\u{bbe}ர\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", "ఉత\u{c4d}తర ల\u{c3e}నర\u{c4d}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Північний Ланаркшир"), ("zh", "北拉纳克郡")]),
                        unofficial_name_list: ["North Lanarkshire"].to_vec(),
                    }
                ),
                (
                    "NLN",
                    Subdivision{
                        name: "North Lincolnshire",
                        country_alpha2: Alpha2::GB,
                        code: "NLN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.60555919999999), longitude: Some(-0.5596582), max_latitude: Some(53.71463019999999), min_latitude: Some(53.4550434), max_longitude: Some(-0.2007016), min_longitude: Some(-0.9500055000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "উত\u{9cd}তর লিংকনশ\u{9be}য\u{9bc}\u{9be}র"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄣\u{11128}𑄚\u{11134}𑄇\u{1112e}𑄣\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "North Lincolnshire"), ("cs", "North Lincolnshire"), ("de", "North Lincolnshire"), ("el", "Βόρειο Λινκολνσάιρ"), ("en", "North Lincolnshire"), ("es", "North Lincolnshire"), ("fr", "Lincolnshire du Nord"), ("gu", "ઉત\u{acd}તર લિ\u{a82}કનશાયર"), ("he", "צפון לינקולנשיר"), ("it", "North Lincolnshire"), ("ja", "ノース・リンカンシャー"), ("kn", "ಉತ\u{ccd}ತರ ಲ\u{cbf}ಂಕನ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "노스링컨셔"), ("nb", "North Lincolnshire"), ("nl", "North Lincolnshire"), ("no", "North Lincolnshire"), ("pl", "North Lincolnshire"), ("pt", "North Lincolnshire"), ("ro", "North Lincolnshire"), ("ru", "Северный Линкольншир"), ("sv", "North Lincolnshire"), ("ta", "வடக\u{bcd}கு லிஙக\u{bbe}ன\u{bcd}ஷிர\u{bcd}"), ("te", "ఉత\u{c4d}తర ల\u{c3f}ంకన\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Північний Лінкольншир"), ("ur", "شمالی لنکنشائر"), ("zh", "北林肯郡")]),
                        unofficial_name_list: ["North Lincolnshire"].to_vec(),
                    }
                ),
                (
                    "NMD",
                    Subdivision{
                        name: "Newry, Mourne and Down",
                        country_alpha2: Alpha2::GB,
                        code: "NMD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "নিউরি"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄢\u{11128}, 𑄟\u{1112f}𑄢\u{11134} 𑄃\u{11133}𑄃 𑄓𑄅\u{1112a}𑄚\u{11134}"), ("de", "Newry, Mourne and Down"), ("en", "Newry, Mourne and Down"), ("es", "Newry"), ("fa", "نیوری، مورن و داون"), ("fr", "Newry, Mourne and Down"), ("gu", "ન\u{acd}ય\u{ac2}રી"), ("it", "Distretto di Newry, Mourne e Down"), ("ja", "ニューリー・モーン・アンド・ダウン"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ರ\u{cbf}"), ("ko", "뉴리"), ("lt", "Niūris Mornas"), ("nl", "Newry, Mourne and Down"), ("pt", "Newry"), ("ru", "Ньюри, Мурн и Даун"), ("ta", "நியூரி"), ("te", "న\u{c4d}యూర\u{c3f}"), ("ur", "نیوری، مورن اور ڈاؤن")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NSM",
                    Subdivision{
                        name: "North Somerset",
                        country_alpha2: Alpha2::GB,
                        code: "NSM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4409659), longitude: Some(-2.7426528), max_latitude: Some(51.5026811), min_latitude: Some(51.2906185), max_longitude: Some(-2.5872009), min_longitude: Some(-3.1153294)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Паўночны Сомерсет"), ("bg", "Северен Съмърсет"), ("bn", "নর\u{9cd}থ সম\u{9be}রসেট"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄥\u{11127}𑄟𑄢\u{11134}𑄥𑄬𑄖\u{11134}"), ("ceb", "North Somerset"), ("cy", "Gogledd Gwlad yr Haf"), ("da", "North Somerset"), ("de", "North Somerset"), ("en", "North Somerset"), ("es", "North Somerset"), ("fa", "سامرست شمالی"), ("fr", "Somerset du Nord"), ("gu", "ઉત\u{acd}તર સોમરસ\u{ac7}ટ"), ("hy", "Հյուսիսային Սոմերսեթ"), ("it", "North Somerset"), ("ja", "ノース・サマセット"), ("kn", "ಉತ\u{ccd}ತರ ಸಾಮರ\u{ccd}ಸ\u{cc6}ಟ\u{ccd}"), ("ko", "노스서머싯"), ("nb", "North Somerset"), ("nl", "North Somerset"), ("no", "North Somerset"), ("pl", "North Somerset"), ("pt", "North Somerset"), ("ro", "North Somerset"), ("ru", "Северный Сомерсет"), ("sv", "North Somerset"), ("ta", "வட சொமேர\u{bcd}செட\u{bcd}"), ("te", "ఉత\u{c4d}తర స\u{c4b}మర\u{c4d}స\u{c46}ట\u{c4d}"), ("uk", "Північний Сомерсет"), ("ur", "شمالی سامرسیٹ"), ("zh", "北索美塞特")]),
                        unofficial_name_list: ["North Somerset"].to_vec(),
                    }
                ),
                (
                    "NTH",
                    Subdivision{
                        name: "Northamptonshire",
                        country_alpha2: Alpha2::GB,
                        code: "NTH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.27299439999999), longitude: Some(-0.8755514999999999), max_latitude: Some(52.64360079999999), min_latitude: Some(51.9772815), max_longitude: Some(-0.341608), min_longitude: Some(-1.332346)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Northamptonshire"), ("ar", "نورثهامبتونشاير"), ("be", "Графства Нартгемптаншыр"), ("bg", "Нортхамптъншър"), ("bn", "নর\u{9cd}থ\u{9be}ম\u{9cd}পটনশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Northamptonshire"), ("ccp", "𑄚\u{11127}𑄢\u{11134}𑄗𑄟\u{11134}𑄑\u{11127}𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Northamptonshire"), ("cs", "Northamptonshire"), ("cy", "Swydd Northampton"), ("da", "Northamptonshire"), ("de", "Northamptonshire"), ("el", "Νορθάμπτονσαϊρ"), ("en", "Northamptonshire"), ("es", "Northamptonshire"), ("et", "Northamptonshire"), ("eu", "Northamptonshire"), ("fa", "نورث\u{200c}همپتون\u{200c}شایر"), ("fi", "Northamptonshire"), ("fr", "Northamptonshire"), ("ga", "Northamptonshire"), ("gl", "Northamptonshire"), ("gu", "ઉત\u{acd}તરહ\u{ac7}મ\u{acd}પ\u{acd}ટશાયર"), ("he", "נורת׳האמפטונשייר"), ("hi", "नॉर\u{94d}थह\u{948}म\u{94d}पटनशायर"), ("hu", "Northamptonshire"), ("hy", "Նորթհեմփթոնշիր"), ("id", "Northamptonshire"), ("is", "Northamptonshire"), ("it", "Northamptonshire"), ("ja", "ノーサンプトンシャー"), ("kn", "ನಾರ\u{ccd}ಥಾಂಪ\u{ccd}ಟನ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "노샘프턴셔 주"), ("lt", "Nortamptonšyras"), ("lv", "Northemptonšīra"), ("mk", "Нортхемптоншир"), ("ml", "നോർത\u{d4d}ത\u{d3e}ംപ\u{d4d}റ\u{d4d}റൺഷയർ"), ("mr", "नॉर\u{94d}थअ\u{901}प\u{94d}टनशायर"), ("nb", "Northamptonshire"), ("nl", "Northamptonshire"), ("no", "Northamptonshire"), ("pl", "Northamptonshire"), ("pt", "Northamptonshire"), ("ro", "Northamptonshire"), ("ru", "Нортгемптоншир"), ("sk", "Northamptonshire"), ("sl", "Northamptonshire"), ("sr", "Нортхемптоншир"), ("sr_Latn", "Northemptonšir"), ("sv", "Northamptonshire"), ("ta", "நோர\u{bcd}தம\u{bcd}ப\u{bcd}டன\u{bcd}ஷிர\u{bcd}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c3e}ంప\u{c4d}టన\u{c4d}ష\u{c48}ర\u{c4d}"), ("th", "นอร\u{e4c}แทมป\u{e4c}ต\u{e31}นเชอร\u{e4c}"), ("tr", "Northamptonshire"), ("uk", "Нортгемптоншир"), ("ur", "نارتھیمپٹنشائر"), ("vi", "Northamptonshire"), ("yue", "諾咸頓郡"), ("yue_Hans", "诺咸顿郡"), ("zh", "北安普敦郡")]),
                        unofficial_name_list: ["Northamptonshire"].to_vec(),
                    }
                ),
                (
                    "NTL",
                    Subdivision{
                        name: "Neath Port Talbot [Castell-nedd Port Talbot GB-CTL]",
                        country_alpha2: Alpha2::GB,
                        code: "NTL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.656991), longitude: Some(-3.805476), max_latitude: Some(51.6982626), min_latitude: Some(51.6182591), max_longitude: Some(-3.7612876), min_longitude: Some(-3.8580652)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Нийт Порт Толбът"), ("bn", "নিথ পোর\u{9cd}ট ত\u{9be}লবোট ক\u{9be}উন\u{9cd}টি বোরো"), ("ca", "Castell-nedd Port Talbot"), ("ccp", "𑄚\u{11128}𑄖\u{11134} 𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134} 𑄑𑄣\u{11134}𑄝\u{1112e}𑄖\u{11134}"), ("ceb", "Neath Port Talbot"), ("cs", "Neath Port Talbot"), ("cy", "Castell-nedd Port Talbot"), ("de", "Neath Port Talbot"), ("en", "Neath Port Talbot"), ("es", "Neath-Port Talbot"), ("et", "Neath Port Talbot"), ("eu", "Neath Port Talbot"), ("fa", "نیث بندر تالبوت"), ("fi", "Neath Port Talbot"), ("fr", "Neath Port Talbot"), ("ga", "Castell-nedd Port Talbot"), ("gu", "નીથ પોર\u{acd}ટ ટ\u{ac7}લબોટ કાઉન\u{acd}ટી બોરો"), ("he", "ניית׳ פורט טלבוט"), ("hu", "Neath Port Talbot"), ("it", "distretto di contea di Neath Port Talbot"), ("ja", "ニース・ポート・タルボット"), ("kn", "ನೀತ\u{ccd} ಪೋರ\u{ccd}ಟ\u{ccd} ಟಾಲ\u{ccd}ಬೋಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "니스포트탤벗"), ("lt", "Nit Port Talbotas"), ("mk", "Нит Порт Толбот"), ("nb", "Neath Port Talbot"), ("nl", "Neath Port Talbot"), ("no", "Neath Port Talbot"), ("pl", "Neath Port Talbot"), ("pt", "Neath Port Talbot"), ("ro", "Neath Port Talbot"), ("ru", "Нит — Порт-Толбот"), ("sv", "Neath Port Talbot"), ("ta", "ந\u{bc0}த\u{bcd} போர\u{bcd}ட\u{bcd} ட\u{bbe}ல\u{bcd}போட\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "న\u{c40}త\u{c4d} ప\u{c4b}ర\u{c4d}ట\u{c4d} ట\u{c3e}ల\u{c4d}బ\u{c4b}ట\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("uk", "Ніт-Порт-Толбот"), ("zh", "下塔尔波特港")]),
                        unofficial_name_list: ["Castell-nedd Porth Talbot"].to_vec(),
                    }
                ),
                (
                    "NTT",
                    Subdivision{
                        name: "Nottinghamshire",
                        country_alpha2: Alpha2::GB,
                        code: "NTT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1285044), longitude: Some(-0.9030999), max_latitude: Some(53.502478), min_latitude: Some(52.7894374), max_longitude: Some(-0.6662629999999999), min_longitude: Some(-1.3445929)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nottinghamshire"), ("ar", "نوتنغهامشير"), ("be", "Нотынгемшыр"), ("bg", "Нотингамшър"), ("bn", "নটিংহ\u{9cd}য\u{9be}মশ\u{9be}য\u{9bc}\u{9be}রে"), ("bs", "Nottinghamshire"), ("ca", "Nottinghamshire"), ("ccp", "𑄚\u{11127}𑄑\u{11133}𑄦\u{11128}\u{11101}𑄦𑄟\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Nottinghamshire"), ("cs", "Nottinghamshire"), ("cy", "Swydd Nottingham"), ("da", "Nottinghamshire"), ("de", "Nottinghamshire"), ("el", "Νότιγχαμσιερ"), ("en", "Nottinghamshire"), ("es", "Nottinghamshire"), ("et", "Nottinghamshire"), ("eu", "Nottinghamshire"), ("fa", "ناتینگهام\u{200c}شایر"), ("fi", "Nottinghamshire"), ("fr", "Nottinghamshire"), ("ga", "Nottinghamshire"), ("gu", "નોટિ\u{a82}ગહામશાયર"), ("he", "נוטינגהאמשייר"), ("hi", "नॉटि\u{902}घमशायर"), ("hu", "Nottinghamshire"), ("hy", "Նոթթինգեմշիր"), ("id", "Nottinghamshire"), ("is", "Nottinghamshire"), ("it", "Nottinghamshire"), ("ja", "ノッティンガムシャー"), ("ka", "ნოტინგემშირი"), ("kn", "ನಾಟ\u{cbf}ಂಗ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "노팅엄셔 주"), ("lt", "Notingamšyras"), ("lv", "Notingemšīra"), ("mk", "Нотингемшир"), ("mr", "नॉटि\u{902}गह\u{945}मशायर"), ("nb", "Nottinghamshire"), ("nl", "Nottinghamshire"), ("no", "Nottinghamshire"), ("pa", "ਨ\u{a4c}ਟਿ\u{a70}ਘਮਸ\u{a3c}ਰ"), ("pl", "Nottinghamshire"), ("pt", "Nottinghamshire"), ("ro", "Nottinghamshire"), ("ru", "Ноттингемшир"), ("sk", "Nottinghamshire"), ("sl", "Nottinghamshire"), ("sr", "Нотингамшир"), ("sr_Latn", "Notingamšir"), ("sv", "Nottinghamshire"), ("ta", "நோட\u{bcd}டிங\u{bcd}ஹம\u{bcd}ஷிர\u{bcd}"), ("te", "న\u{c3e}ట\u{c3f}ంగ\u{c4d} హమ\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "นอตท\u{e34}งแฮมเชอร\u{e4c}"), ("tr", "Nottinghamshire"), ("uk", "Ноттінгемшир"), ("ur", "ناٹنگھمشائر"), ("vi", "Nottinghamshire"), ("yue", "諾定咸郡"), ("yue_Hans", "诺定咸郡"), ("zh", "諾丁漢郡")]),
                        unofficial_name_list: ["Nottinghamshire"].to_vec(),
                    }
                ),
                (
                    "NTY",
                    Subdivision{
                        name: "North Tyneside",
                        country_alpha2: Alpha2::GB,
                        code: "NTY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.008), longitude: Some(-1.546), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "নর\u{9cd}থ ট\u{9be}ইনিস\u{9be}ইড"), ("ca", "North Tyneside"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄑\u{1112d}𑄚\u{11134}𑄥\u{1112d}𑄖\u{11134}"), ("ceb", "Borough of North Tyneside"), ("de", "North Tyneside"), ("en", "North Tyneside"), ("es", "North Tyneside"), ("fa", "تاینساید شمالی"), ("fi", "North Tyneside"), ("fr", "North Tyneside"), ("gu", "ઉત\u{acd}તર ટાઇનીસાઇડ"), ("hu", "North Tyneside"), ("it", "North Tyneside"), ("ja", "ノース・タインサイド"), ("kn", "ನಾರ\u{ccd}ತ\u{ccd} ಟೈನ\u{cc6}ಸೈಡ\u{ccd}"), ("ko", "노스타인사이드"), ("lt", "North Tinesidas"), ("nb", "North Tyneside"), ("nl", "North Tyneside"), ("no", "North Tyneside"), ("pl", "North Tyneside"), ("pt", "North Tyneside"), ("ro", "North Tyneside"), ("ru", "Норт-Тайнсайд"), ("sv", "Borough of North Tyneside"), ("ta", "வடக\u{bcd}கு டினேசைடு"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} ట\u{c48}న\u{c4d}స\u{c48}డ\u{c4d}"), ("ur", "شمالی ٹینیسائڈ"), ("vi", "Bắc Tyneside"), ("zh", "北泰因賽德")]),
                        unofficial_name_list: ["North Tyneside"].to_vec(),
                    }
                ),
                (
                    "NWM",
                    Subdivision{
                        name: "Newham",
                        country_alpha2: Alpha2::GB,
                        code: "NWM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5255162), longitude: Some(0.0352163), max_latitude: Some(51.5639889), min_latitude: Some(51.49770669999999), max_longitude: Some(0.09767660000000002), min_longitude: Some(-0.0212642)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نيوهام"), ("bn", "নিউহ\u{9cd}য\u{9be}ম"), ("ca", "Newham"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄦𑄟\u{11134}"), ("ceb", "Newham"), ("cs", "Newham"), ("cy", "Newham"), ("da", "Newham"), ("de", "London Borough of Newham"), ("el", "Νιούχαμ"), ("en", "Newham"), ("es", "Newham"), ("eu", "Newham"), ("fa", "منطقه نیوهام لندن"), ("fi", "Newham"), ("fr", "district londonien de Newham"), ("ga", "Buirg Londan Newham"), ("gu", "લ\u{a82}ડન બોરો ઓફ ન\u{acd}ય\u{ac2}હામ"), ("he", "ניוהאם"), ("hi", "न\u{94d}य\u{942}ह\u{948}म बरो"), ("hu", "Newham kerület"), ("hy", "Նյուհեմ"), ("is", "Newham"), ("it", "Newham"), ("ja", "ニューハム・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ನ\u{ccd}ಯ\u{cc2}ಹಾಮ\u{ccd}"), ("ko", "뉴엄 구"), ("mr", "न\u{94d}य\u{942}ह\u{945}म"), ("nb", "Newham"), ("nl", "Newham"), ("no", "Newham"), ("pl", "London Borough of Newham"), ("pt", "Newham"), ("ro", "Newham"), ("ru", "Ньюэм"), ("sr", "Лондонска општина Њуам"), ("sr_Latn", "Londonska opština Njuam"), ("sv", "London Borough of Newham"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} நியூஹ\u{bbe}ம\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} న\u{c4d}యూహం"), ("tr", "Newham"), ("uk", "Ньюем"), ("ur", "نیوہیم بورو"), ("vi", "Khu Newham của Luân Đôn"), ("yue", "紐咸區"), ("yue_Hans", "纽咸区"), ("zh", "紐漢區")]),
                        unofficial_name_list: ["Newham"].to_vec(),
                    }
                ),
                (
                    "NWP",
                    Subdivision{
                        name: "Newport [Casnewydd GB-CNW]",
                        country_alpha2: Alpha2::GB,
                        code: "NWP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.584151), longitude: Some(-2.997664), max_latitude: Some(51.6213929), min_latitude: Some(51.535335), max_longitude: Some(-2.9204637), min_longitude: Some(-3.0381005)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Ньюпарт"), ("ccp", "𑄚\u{11128}𑄅\u{1112a}𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Newport"), ("cy", "Casnewydd"), ("de", "Newport"), ("en", "Newport"), ("fr", "Newport"), ("nl", "Newport"), ("pl", "Newport"), ("ru", "Ньюпорт"), ("sv", "Newport"), ("uk", "Ньюпорт")]),
                        unofficial_name_list: ["Casnewydd"].to_vec(),
                    }
                ),
                (
                    "NYK",
                    Subdivision{
                        name: "North Yorkshire",
                        country_alpha2: Alpha2::GB,
                        code: "NYK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.25035949999999), longitude: Some(-1.4708553), max_latitude: Some(54.5601224), min_latitude: Some(53.6210943), max_longitude: Some(-0.2124959), min_longitude: Some(-2.5647389)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Yorkshire"), ("ar", "شمال يوركشير"), ("be", "Норт-Ёркшыр"), ("bg", "Северен Йоркшър"), ("bn", "উত\u{9cd}তর ইয\u{9bc}র\u{9cd}কশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "North Yorkshire"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄃\u{11128}𑄠\u{1112e}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "North Yorkshire"), ("cs", "Severní Yorkshire"), ("cy", "Swydd Gogledd Efrog"), ("da", "North Yorkshire"), ("de", "North Yorkshire"), ("en", "North Yorkshire"), ("es", "Yorkshire del Norte"), ("et", "Põhja-Yorkshire"), ("eu", "Iparraldeko Yorkshire"), ("fa", "یورک\u{200c}شایر شمالی"), ("fi", "North Yorkshire"), ("fr", "Yorkshire du Nord"), ("ga", "Yorkshire Thuaidh"), ("gl", "North Yorkshire"), ("gu", "ઉત\u{acd}તર યોર\u{acd}કશાયર"), ("he", "צפון יורקשייר"), ("hi", "नॉर\u{94d}थ यॉर\u{94d}कशायर"), ("hu", "North Yorkshire"), ("hy", "Նորթ Յորքշիր"), ("id", "Yorkshire Utara"), ("is", "Norður-Yorkshire"), ("it", "North Yorkshire"), ("ja", "ノース・ヨークシャー"), ("kn", "ನಾರ\u{ccd}ತ\u{ccd} ಯಾರ\u{ccd}ಕ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "노스요크셔 주"), ("lt", "Šiaurės Jorkšyras"), ("lv", "Nortjorkšīra"), ("mk", "Северен Јоркшир"), ("mr", "नॉर\u{94d}थ यॉर\u{94d}कशायर"), ("nb", "North Yorkshire"), ("nl", "North Yorkshire"), ("no", "North Yorkshire"), ("pl", "North Yorkshire"), ("pt", "North Yorkshire"), ("ro", "North Yorkshire"), ("ru", "Норт-Йоркшир"), ("sk", "North Yorkshire"), ("sr", "Северни Јоркшир"), ("sr_Latn", "Severni Jorkšir"), ("sv", "North Yorkshire"), ("ta", "வடக\u{bcd}கு யோர\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", "ఉత\u{c4d}తర య\u{c3e}ర\u{c4d}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("tr", "North Yorkshire"), ("uk", "Північний Йоркшир"), ("ur", "شمالی یارکشائر"), ("vi", "North Yorkshire"), ("yue", "北約郡"), ("yue_Hans", "北约郡"), ("zh", "北约克郡")]),
                        unofficial_name_list: ["North Yorkshire"].to_vec(),
                    }
                ),
                (
                    "OLD",
                    Subdivision{
                        name: "Oldham",
                        country_alpha2: Alpha2::GB,
                        code: "OLD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5409298), longitude: Some(-2.1113659), max_latitude: Some(53.5733246), min_latitude: Some(53.4989624), max_longitude: Some(-2.0435901), min_longitude: Some(-2.1437648)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Олдэм"), ("bg", "Олдъм"), ("ccp", "𑄃\u{1112e}𑄣\u{11133}𑄓\u{11134}𑄦𑄟\u{11134}"), ("ceb", "Borough of Oldham"), ("de", "Metropolitan Borough of Oldham"), ("en", "Oldham"), ("fa", "کلان\u{200c}شهر مستقل اولدهام"), ("fr", "district métropolitain de Oldham"), ("it", "Metropolitan Borough of Oldham"), ("ja", "メトロポリタン・バラ・オブ・オールダム"), ("ko", "올덤 도시 자치구"), ("nb", "Oldham"), ("nl", "Oldham"), ("no", "Oldham"), ("pl", "Metropolitan Borough of Oldham"), ("ru", "Олдем"), ("sv", "Oldham"), ("tr", "Oldham Metropoliten Borough"), ("uk", "Олдем"), ("ur", "میٹروپولیٹن بورو اولڈھم"), ("zh", "奧爾德姆都市自治市")]),
                        unofficial_name_list: ["Oldham"].to_vec(),
                    }
                ),
                (
                    "ORK",
                    Subdivision{
                        name: "Orkney Islands",
                        country_alpha2: Alpha2::GB,
                        code: "ORK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(59.04291250000001), longitude: Some(-3.1542155), max_latitude: Some(59.39279309999999), min_latitude: Some(58.6730495), max_longitude: Some(-2.3704314), min_longitude: Some(-4.507253599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Orkney"), ("am", "ኦርክኒ"), ("ar", "جزر أوركني"), ("az", "Orkney adaları"), ("be", "Аркнейскія астравы"), ("bg", "Оркни"), ("bn", "ওর\u{9cd}কনে দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"), ("ca", "Illes Òrcades"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄇\u{11134}𑄚𑄬 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄌\u{11134}"), ("ceb", "Orkney Islands (dapit sa konseho)"), ("cs", "Orkneje"), ("cy", "Ynysoedd Erch"), ("da", "Orkneyøerne"), ("de", "Orkney"), ("el", "Ορκάδες"), ("en", "Orkney Islands"), ("es", "Orcadas"), ("et", "Orkney saared"), ("eu", "Orkadak"), ("fa", "اورکنی"), ("fi", "Orkneysaaret"), ("fr", "Orcades"), ("ga", "Inse Orc"), ("gl", "Orcadas"), ("gu", "ઓર\u{acd}કની આઇલ\u{ac7}ન\u{acd}ડ\u{acd}સ"), ("he", "אורקני"), ("hi", "ओर\u{94d}कन\u{947}"), ("hr", "Orkneyski otoci"), ("hu", "Orkney-szigetek"), ("hy", "Օրկնեյան կղզիներ"), ("id", "Orkney"), ("is", "Orkneyjar"), ("it", "Isole Orcadi"), ("ja", "オークニー諸島"), ("kn", "ಆರ\u{ccd}ಕ\u{ccd}ನ\u{cbf} ದ\u{ccd}ವೀಪಗಳು"), ("ko", "오크니 제도"), ("ky", "Оркни аралдары"), ("lt", "Orknio salos"), ("lv", "Orkneju salas"), ("mk", "Оркниски Острови"), ("nb", "Orknøyene"), ("nl", "Orkney-eilanden"), ("no", "Orknøyene"), ("pl", "Orkady"), ("ps", "اورکني"), ("pt", "Órcades"), ("ro", "Orkney"), ("ru", "Оркнейские острова"), ("sk", "Orkneje"), ("sr", "Оркнијска острва"), ("sr_Latn", "Orknijska ostrva"), ("sv", "Orkneyöarna"), ("sw", "Visiwa vya Orkney"), ("ta", "ஒர\u{bcd}க\u{bcd}கனே த\u{bc0}வுகள\u{bcd}"), ("te", "ఓర\u{c4d}క\u{c4d}న\u{c40}క\u{c3f} ఐల\u{c3e}ండ\u{c4d}"), ("th", "ออร\u{e4c}กน\u{e35}ย\u{e4c}"), ("tr", "Orkney Adaları"), ("uk", "Оркнейські острови"), ("vi", "Orkney"), ("yue", "奧克尼"), ("yue_Hans", "奥克尼"), ("zh", "奥克尼群岛")]),
                        unofficial_name_list: ["Orkney Islands"].to_vec(),
                    }
                ),
                (
                    "OXF",
                    Subdivision{
                        name: "Oxfordshire",
                        country_alpha2: Alpha2::GB,
                        code: "OXF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.7612056), longitude: Some(-1.2464674), max_latitude: Some(52.16847139999999), min_latitude: Some(51.4594132), max_longitude: Some(-0.8700833), min_longitude: Some(-1.7195008)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oxfordshire"), ("am", "ኦክስፎርድ"), ("ar", "أكسفوردشير"), ("az", "Oksford"), ("be", "Оксфардшыр"), ("bg", "Оксфордшър"), ("bn", "অক\u{9cd}সফোর\u{9cd}ডশ\u{9be}য\u{9bc}\u{9be}র"), ("bs", "Oxford"), ("ca", "Oxfordshire"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Oxford (kapital sa kondado sa Hiniusang Gingharian)"), ("cs", "Oxfordshire"), ("cy", "Rhydychen"), ("da", "Oxfordshire"), ("de", "Oxfordshire"), ("el", "Όξφορντσαϊρ"), ("en", "Oxfordshire"), ("es", "Oxfordshire"), ("et", "Oxfordshire"), ("eu", "Oxfordshire"), ("fa", "آکسفوردشایر"), ("fi", "Oxfordshire"), ("fr", "Oxfordshire"), ("ga", "Oxford"), ("gl", "Oxfordshire"), ("gu", "ઓક\u{acd}ષફર\u{acd}ડ"), ("he", "אוקספורדשייר"), ("hi", "ऑक\u{94d}सफ\u{93c}र\u{94d}डशायर"), ("hr", "Oxford"), ("hu", "Oxfordshire"), ("hy", "Օքսֆորդշիր"), ("id", "Oxfordshire"), ("is", "Oxfordshire"), ("it", "Oxfordshire"), ("ja", "オックスフォードシャー"), ("jv", "Oxford"), ("ka", "ოქსფორდშირი"), ("kk", "Оксфорд"), ("kn", "ಆಕ\u{ccd}ಸ\u{ccd}\u{200c}ಫರ\u{ccd}ಡ\u{ccd}"), ("ko", "옥스퍼드셔 주"), ("ky", "Оксфорд"), ("lt", "Oksfordšyras"), ("lv", "Oksfordšīra"), ("mk", "Оксфорд"), ("mr", "ऑक\u{94d}सफर\u{94d}डशायर"), ("ms", "Oxford"), ("my", "အောက\u{103a}စဖ\u{102d}\u{102f}\u{1037}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Oxfordshire"), ("ne", "अक\u{94d}सफोर\u{94d}डसायर"), ("nl", "Oxfordshire"), ("no", "Oxfordshire"), ("pa", "ਆਕਸਫ\u{a3c}\u{a4b}ਰਡ"), ("pl", "Oxfordshire"), ("pt", "Oxfordshire"), ("ro", "Oxfordshire"), ("ru", "Оксфордшир"), ("si", "ඔක\u{dca}ස\u{dca}ෆර\u{dca}ඩ\u{dca}"), ("sk", "Oxfordshire"), ("sl", "Oxfordshire"), ("sq", "Oksford"), ("sr", "Оксфордшир"), ("sr_Latn", "Oksfordšir"), ("sv", "Oxfordshire"), ("sw", "Oxford"), ("ta", "ஆக\u{bcd}சுபோர\u{bcd}ட\u{bcd}சையர\u{bcd}"), ("te", "ఆక\u{c4d}స\u{c4d}\u{200c}ఫర\u{c4d}డ\u{c4d}"), ("th", "อ\u{e4a}อกซฟอร\u{e4c}ดเชอร\u{e4c}"), ("tr", "Oxfordshire"), ("uk", "Оксфордшир"), ("ur", "آکسفورڈشائر"), ("uz", "Oksford"), ("vi", "Oxfordshire"), ("yue", "牛津郡"), ("yue_Hans", "牛津郡"), ("zh", "牛津郡")]),
                        unofficial_name_list: ["Oxfordshire"].to_vec(),
                    }
                ),
                (
                    "PEM",
                    Subdivision{
                        name: "Pembrokeshire [Sir Benfro GB-BNF]",
                        country_alpha2: Alpha2::GB,
                        code: "PEM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8757077), longitude: Some(-4.939193299999999), max_latitude: Some(52.1176059), min_latitude: Some(51.59607829999999), max_longitude: Some(-4.485517499999999), min_longitude: Some(-5.670226899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيمبروكشاير"), ("be", "Пембрукшыр"), ("bg", "Пембрукшър"), ("bn", "পেমব\u{9cd}রোকশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Sir Benfro"), ("ccp", "𑄛𑄬𑄟\u{11134}𑄝\u{11133}𑄢\u{1112e}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Pembrokeshire"), ("cs", "Pembrokeshire"), ("cy", "Sir Benfro"), ("de", "Pembrokeshire"), ("en", "Pembrokeshire"), ("es", "Pembrokeshire"), ("et", "Pembrokeshire"), ("eu", "Pembrokeshire"), ("fa", "پمبروکشر"), ("fi", "Pembrokeshire"), ("fr", "Pembrokeshire"), ("ga", "Sir Benfro"), ("gl", "Sir Benfro"), ("gu", "પ\u{ac7}મ\u{acd}બ\u{acd}રોકશાયર"), ("he", "פמברוקשייר"), ("hu", "Pembrokeshire"), ("it", "Pembrokeshire"), ("ja", "ペンブルックシャー"), ("kn", "ಪ\u{cc6}ಂಬ\u{ccd}ರೋಕ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "펨브로크셔"), ("lt", "Pembrukšyras"), ("ms", "Pembrokeshire"), ("nb", "Pembrokeshire"), ("nl", "Pembrokeshire"), ("no", "Pembrokeshire"), ("pl", "Pembrokeshire"), ("pt", "Pembrokeshire"), ("ro", "Pembrokeshire"), ("ru", "Пембрукшир"), ("sk", "Pembrokeshire"), ("sl", "Pembrokeshire"), ("sv", "Pembrokeshire"), ("ta", "பிஎம\u{bcd}ப\u{bcd}ரோகேஷிர\u{bcd}"), ("te", "ప\u{c4d}ర\u{c46}ంబ\u{c4d}ర\u{c4b}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Пембрукшир"), ("ur", "پیمبروکشائر"), ("vi", "Pembrokeshire"), ("zh", "彭布罗克郡")]),
                        unofficial_name_list: ["Sir Benfro"].to_vec(),
                    }
                ),
                (
                    "PKN",
                    Subdivision{
                        name: "Perth and Kinross",
                        country_alpha2: Alpha2::GB,
                        code: "PKN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.5917369), longitude: Some(-3.8557348), max_latitude: Some(56.9486332), min_latitude: Some(56.1324367), max_longitude: Some(-3.044693), min_longitude: Some(-4.7204447)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيرث وكينروس"), ("az", "Pert-end-Kinross"), ("be", "Перт-энд-Кінрос"), ("bg", "Пърт анд Кинрос"), ("bn", "প\u{9be}র\u{9cd}থ ও কিনরোস"), ("ca", "Perth i Kinross"), ("ccp", "𑄛\u{11127}𑄢\u{11134}𑄖\u{11134} 𑄃\u{11133}𑄃 𑄇\u{11128}𑄚\u{11134}𑄢\u{1112e}𑄌\u{11134}"), ("ceb", "Perth and Kinross"), ("cs", "Perth and Kinross"), ("cy", "Perth a Kinross"), ("da", "Perth and Kinross"), ("de", "Perth and Kinross"), ("en", "Perth and Kinross"), ("es", "Perth and Kinross"), ("et", "Perth and Kinross"), ("eu", "Perth eta Kinross"), ("fa", "پرت و کینروس"), ("fi", "Perth ja Kinross"), ("fr", "Perth and Kinross"), ("ga", "Comhairle Pheairt is Cheann Rois"), ("gu", "પર\u{acd}થ અન\u{ac7} કિનરોસ"), ("he", "פרת וקינרוס"), ("hy", "Փերթ և Կինրոս"), ("it", "Perth e Kinross"), ("ja", "パース・アンド・キンロス"), ("kk", "Перт-энд-Кинросс"), ("kn", "ಪರ\u{ccd}ತ\u{ccd} ಮತ\u{ccd}ತು ಕ\u{cbf}ನ\u{ccd}ರಾಸ\u{ccd}"), ("ko", "퍼스 킨로스"), ("lt", "Pertas ir Kinrosas"), ("mk", "Перт и Кинрос"), ("nb", "Perth and Kinross"), ("nl", "Perth and Kinross"), ("no", "Perth and Kinross"), ("pl", "Perth and Kinross"), ("pt", "Perth and Kinross"), ("ro", "Perth and Kinross"), ("ru", "Перт-энд-Кинросс"), ("sv", "Perth and Kinross"), ("ta", "பெர\u{bcd}த\u{bcd} & கிந\u{bcd}த\u{bcd}ரோஸ\u{bcd}"), ("te", "ప\u{c46}ర\u{c4d}త\u{c4d} మర\u{c3f}యు క\u{c3f}న\u{c4d}ర\u{c4b}స\u{c4d}"), ("uk", "Перт-і-Кінросс"), ("zh", "珀斯-金罗斯")]),
                        unofficial_name_list: ["Perth and Kinross"].to_vec(),
                    }
                ),
                (
                    "PLY",
                    Subdivision{
                        name: "Plymouth",
                        country_alpha2: Alpha2::GB,
                        code: "PLY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.3754565), longitude: Some(-4.1426565), max_latitude: Some(50.4441664), min_latitude: Some(50.3333185), max_longitude: Some(-4.0196074), min_longitude: Some(-4.205532499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Plymouth"), ("ar", "بليموث"), ("az", "Plimut"), ("be", "Горад Плімут"), ("bg", "Плимът"), ("bn", "প\u{9cd}ল\u{9be}ইম\u{9be}উথ"), ("ca", "Plymouth"), ("ccp", "𑄛\u{11133}𑄣\u{1112d}𑄟𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "Plymouth (kondado)"), ("cs", "Plymouth"), ("da", "Plymouth"), ("de", "Plymouth"), ("el", "Πλίμουθ"), ("en", "Plymouth"), ("es", "Plymouth"), ("et", "Plymouth"), ("eu", "Plymouth"), ("fa", "پلیموث"), ("fi", "Plymouth"), ("fr", "Plymouth"), ("gl", "Plymouth"), ("gu", "પ\u{acd}લાયમાઉથ"), ("he", "פלימות׳"), ("hi", "प\u{94d}लायमाउथ"), ("hr", "Plymouth"), ("hu", "Plymouth"), ("hy", "Պլիմուտ"), ("id", "Plymouth"), ("is", "Plymouth"), ("it", "Plymouth"), ("ja", "プリマス"), ("kn", "ಪ\u{ccd}ಲೈಮ\u{ccc}ತ\u{ccd}"), ("ko", "플리머스"), ("lt", "Plimutas"), ("lv", "Plimuta"), ("mr", "प\u{94d}लिमथ"), ("ms", "Plymouth"), ("nb", "Plymouth"), ("nl", "Plymouth"), ("no", "Plymouth"), ("pl", "Plymouth"), ("pt", "Plymouth"), ("ro", "Plymouth"), ("ru", "Плимут"), ("si", "ප\u{dca}ලය\u{dd2}මව\u{dd4}ත\u{dca}"), ("sk", "Plymouth"), ("sl", "Plymouth"), ("sr", "Плимут"), ("sr_Latn", "Plimut"), ("sv", "Plymouth"), ("sw", "Plymouth"), ("ta", "ப\u{bcd}ளைமொத\u{bcd}"), ("te", "ప\u{c4d}ల\u{c3f}మ\u{c4c}త\u{c4d}"), ("th", "พล\u{e34}ม\u{e31}ท"), ("tr", "Plymouth"), ("uk", "Плімут"), ("ur", "پلایماؤتھ"), ("vi", "Plymouth"), ("zh", "普利茅斯")]),
                        unofficial_name_list: ["Plymouth"].to_vec(),
                    }
                ),
                (
                    "POR",
                    Subdivision{
                        name: "Portsmouth",
                        country_alpha2: Alpha2::GB,
                        code: "POR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.816667), longitude: Some(-1.083333), max_latitude: Some(50.8593091), min_latitude: Some(50.7501628), max_longitude: Some(-1.0206408), min_longitude: Some(-1.1724027)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Portsmouth"), ("ar", "بورتسموث"), ("az", "Portsmut"), ("be", "Портсмут"), ("bg", "Портсмът"), ("bn", "পোর\u{9cd}টস\u{9cd}\u{200c}ম\u{9be}থ"), ("ca", "Portsmouth"), ("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134}𑄟𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "City of Portsmouth"), ("cs", "Portsmouth"), ("cy", "Portsmouth"), ("da", "Portsmouth"), ("de", "Portsmouth"), ("el", "Πόρτσμουθ"), ("en", "Portsmouth"), ("es", "Portsmouth"), ("et", "Portsmouth"), ("eu", "Portsmouth"), ("fa", "پورتسموث"), ("fi", "Portsmouth"), ("fr", "Portsmouth"), ("ga", "Portsmouth"), ("gl", "Portsmouth"), ("gu", "પોર\u{acd}ટ\u{acd}સમાઉથ"), ("ha", "Portsmouth"), ("ha_NE", "Portsmouth"), ("he", "פורטסמות׳"), ("hi", "पोर\u{94d}ट\u{94d}समाउथ"), ("hr", "Portsmouth"), ("hu", "Portsmouth"), ("hy", "Պորտսմութ"), ("id", "Portsmouth"), ("is", "Portsmouth"), ("it", "Portsmouth"), ("ja", "ポーツマス"), ("jv", "Portsmouth"), ("ka", "პორტსმუთი"), ("kk", "Портсмунт"), ("kn", "ಪೋರ\u{ccd}ಟ\u{ccd}ಸಮ\u{ccc}ಥ\u{ccd}"), ("ko", "포츠머스"), ("lt", "Portsmutas"), ("lv", "Portsmuta"), ("mk", "Портсмут"), ("mr", "पोर\u{94d}टस\u{94d}मथ"), ("ms", "Portsmouth"), ("nb", "Portsmouth"), ("nl", "Portsmouth"), ("no", "Portsmouth"), ("pl", "Portsmouth"), ("pt", "Portsmouth"), ("ro", "Portsmouth"), ("ru", "Портсмут"), ("si", "පොර\u{dca}ට\u{dca}ස\u{dca}මව\u{dd4}ත\u{dca}"), ("sk", "Portsmouth"), ("sl", "Portsmouth"), ("sr", "Портсмут"), ("sr_Latn", "Portsmut"), ("sv", "Portsmouth"), ("sw", "Portsmouth"), ("ta", "போர\u{bcd}ட\u{bcd}ஸ\u{bcd}மவுத\u{bcd}"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4d}స\u{c4d} మ\u{c4c}త\u{c4d}"), ("th", "พอร\u{e4c}ตสม\u{e31}ท"), ("tr", "Portsmouth"), ("uk", "Портсмут"), ("ur", "پورٹسماؤتھ"), ("uz", "Portsmut"), ("vi", "Portsmouth"), ("yue", "樸茨茅夫"), ("yue_Hans", "朴茨茅夫"), ("zh", "朴次茅斯")]),
                        unofficial_name_list: ["Portsmouth"].to_vec(),
                    }
                ),
                (
                    "POW",
                    Subdivision{
                        name: "Powys",
                        country_alpha2: Alpha2::GB,
                        code: "POW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.1430868), longitude: Some(-3.3736821), max_latitude: Some(52.9015606), min_latitude: Some(51.75274539999999), max_longitude: Some(-2.9496345), min_longitude: Some(-3.9293532)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "باويس"), ("be", "Графства Повіс"), ("bg", "Поуис"), ("bn", "পয\u{9bc}েজ"), ("ca", "Powys"), ("ccp", "𑄛\u{11127}𑄃\u{1112e}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Sir Powys"), ("cs", "Powys"), ("cy", "Powys"), ("de", "Powys"), ("el", "Πόουις"), ("en", "Powys"), ("es", "Powys"), ("et", "Powys"), ("eu", "Powys"), ("fa", "پویس"), ("fi", "Powys"), ("fr", "Powys"), ("ga", "Powys"), ("gu", "પોવીસ"), ("he", "פוויס"), ("hu", "Powys"), ("hy", "Պոուիս"), ("it", "Powys"), ("ja", "ポーイス"), ("kn", "ಪೊವ\u{cbf}ಸ\u{ccd}"), ("ko", "포위스 주"), ("lt", "Povis"), ("nb", "Powys"), ("nl", "Powys"), ("no", "Powys"), ("pl", "Powys"), ("pt", "Powys"), ("ro", "Powys"), ("ru", "Поуис"), ("sv", "Powys"), ("ta", "பௌய\u{bcd}ஸ\u{bcd}"), ("te", "ప\u{c3e}వ\u{c47}స\u{c4d}"), ("uk", "Повіс"), ("ur", "پوویس"), ("vi", "Powys"), ("zh", "波伊斯")]),
                        unofficial_name_list: ["Powys"].to_vec(),
                    }
                ),
                (
                    "PTE",
                    Subdivision{
                        name: "Peterborough",
                        country_alpha2: Alpha2::GB,
                        code: "PTE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.56949849999999), longitude: Some(-0.2405299), max_latitude: Some(52.6637708), min_latitude: Some(52.5060949), max_longitude: Some(-0.1032429), min_longitude: Some(-0.4976634)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Peterborough"), ("ar", "بيتربرة"), ("bn", "পিট\u{9be}রব\u{9be}র\u{9cd}গ"), ("ca", "Peterborough"), ("ccp", "𑄛\u{11128}𑄑𑄢\u{11134}"), ("ceb", "City of Peterborough"), ("cs", "Peterborough"), ("da", "Peterborough"), ("de", "City of Peterborough"), ("el", "Πήτερμπορο"), ("en", "Peter"), ("es", "Peterborough"), ("et", "Peterborough"), ("eu", "Peterborough"), ("fa", "پیتربورو"), ("fi", "Peterborough"), ("fr", "Peterborough"), ("gu", "પીટરબરો"), ("he", "פטרבורו"), ("hi", "पीटरबरो"), ("hu", "Peterborough"), ("hy", "Պիտերբորո"), ("id", "Peterborough"), ("is", "Peterborough"), ("it", "Peterborough"), ("ja", "ピーターバラ"), ("kn", "ಪೀಟರ\u{ccd} ಬೊರೊಹ\u{ccd}"), ("ko", "피터버러"), ("lt", "Piterboras"), ("lv", "Pīterboro"), ("mr", "पीटरबॉरो"), ("ms", "Peterborough"), ("nb", "Peterborough"), ("nl", "Peterborough"), ("no", "Peterborough"), ("pl", "City of Peterborough"), ("pt", "Peterborough"), ("ro", "Peterborough"), ("ru", "Питерборо"), ("si", "ප\u{dd3}ටර\u{dca}බොරෝ"), ("sk", "Peterborough"), ("sl", "Peterborough"), ("sr", "Питерборо"), ("sr_Latn", "Piterboro"), ("sv", "Peterborough"), ("sw", "Peterborough"), ("ta", "ப\u{bc0}ட\u{bcd}டர\u{bcd}ப\u{bbe}ரோ"), ("te", "ప\u{c40}టర\u{c4d} బర\u{c4b}"), ("th", "ป\u{e35}เตอร\u{e4c}โบโร"), ("tr", "Peterborough"), ("uk", "Пітерборо"), ("ur", "پیٹربورو"), ("vi", "Peterborough"), ("zh", "彼得伯勒")]),
                        unofficial_name_list: ["Peterborough"].to_vec(),
                    }
                ),
                (
                    "RCC",
                    Subdivision{
                        name: "Redcar and Cleveland",
                        country_alpha2: Alpha2::GB,
                        code: "RCC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.5393369), longitude: Some(-1.0310219), max_latitude: Some(54.6475271), min_latitude: Some(54.48783479999999), max_longitude: Some(-0.7884143), min_longitude: Some(-1.2017289)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "রেডক\u{9be}র ও ক\u{9cd}লিভল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ccp", "𑄢𑄬𑄖\u{11134}𑄇𑄢\u{11134} 𑄃\u{11133}𑄃 𑄇\u{11133}𑄣\u{11128}𑄛\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Redcar and Cleveland"), ("de", "Redcar and Cleveland"), ("en", "Redcar and Cleveland"), ("es", "Redcar and Cleveland"), ("fr", "Redcar et Cleveland"), ("gu", "ર\u{ac7}ડકાર અન\u{ac7} ક\u{acd}લ\u{ac7}વલ\u{ac7}ન\u{acd}ડ"), ("it", "Redcar and Cleveland"), ("ja", "レッドカー・アンド・クリーヴランド"), ("kn", "ರ\u{cc6}ಡ\u{ccd}ಕಾರ\u{ccd} ಮತ\u{ccd}ತು ಕ\u{ccd}ಲೀವ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "레드카 클리블랜드"), ("nb", "Redcar and Cleveland"), ("nl", "Redcar and Cleveland"), ("no", "Redcar and Cleveland"), ("pl", "Redcar and Cleveland"), ("pt", "Redcar and Cleveland"), ("ro", "Redcar and Cleveland"), ("ru", "Редкар и Кливленд"), ("sv", "Redcar and Cleveland"), ("ta", "ரெட\u{bcd}க\u{bcd}க\u{bbe}ர\u{bcd} ஆன\u{bcd} கிள\u{bc0}வ\u{bcd}லன\u{bcd}ட\u{bcd}"), ("te", "ర\u{c46}డ\u{c4d} క\u{c3e}ర\u{c4d} మర\u{c3f}యు క\u{c4d}ల\u{c47}వ\u{c4d} ల\u{c4d}య\u{c3e}ండ\u{c4d}"), ("tr", "Redcar ve Cleveland"), ("uk", "Редкар і Клівленд"), ("ur", "ریڈکار اور کلیولینڈ"), ("zh", "雷德卡-克利夫兰")]),
                        unofficial_name_list: ["Redcar and Cleveland"].to_vec(),
                    }
                ),
                (
                    "RCH",
                    Subdivision{
                        name: "Rochdale",
                        country_alpha2: Alpha2::GB,
                        code: "RCH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.6097136), longitude: Some(-2.1561), max_latitude: Some(53.67075209999999), min_latitude: Some(53.5780313), max_longitude: Some(-2.1117023), min_longitude: Some(-2.2717896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Рочдейл"), ("ccp", "𑄢\u{1112e}𑄌\u{11134}𑄓𑄣𑄬"), ("ceb", "Borough of Rochdale"), ("da", "Metropolitan Borough of Rochdale"), ("de", "Metropolitan Borough of Rochdale"), ("en", "Rochdale"), ("fa", "کلان\u{200c}شهر مستقل روچدیل"), ("fr", "district métropolitain de Rochdale"), ("it", "Metropolitan Borough of Rochdale"), ("ja", "メトロポリタン・バラ・オブ・ロッチデール"), ("ko", "로치데일 도시 자치구"), ("nb", "Rochdale"), ("nl", "Rochdale"), ("no", "Rochdale"), ("pl", "Metropolitan Borough of Rochdale"), ("ru", "Рочдейл"), ("sv", "Borough of Rochdale"), ("uk", "Рочдейл"), ("ur", "میٹروپولیٹن بورو راچڈیل"), ("zh", "羅奇代爾都市自治市")]),
                        unofficial_name_list: ["Rochdale"].to_vec(),
                    }
                ),
                (
                    "RCT",
                    Subdivision{
                        name: "Rhondda, Cynon, Taff [Rhondda, Cynon,Taf]",
                        country_alpha2: Alpha2::GB,
                        code: "RCT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.6035773), longitude: Some(-3.3411765), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "روندا كينون تاف"), ("be", "Ронта-Кінан-Тав"), ("bg", "Ронда Кънън Таф"), ("ca", "Rhondda Cynon Taf"), ("ccp", "𑄛\u{1112e}𑄚\u{11134}𑄓\u{11133}𑄦 𑄥\u{1112d}𑄠\u{11127}𑄚\u{11134} 𑄑𑄛\u{11134}"), ("ceb", "Rhondda Cynon Taf"), ("cs", "Rhondda Cynon Taf"), ("cy", "Rhondda Cynon Taf"), ("de", "Rhondda Cynon Taf"), ("el", "Ρόντα Σίνον Ταφ"), ("en", "Rhondda Cynon Taf"), ("es", "Rhondda Cynon Taff"), ("et", "Rhondda Cynon Taff"), ("eu", "Rhondda Cynon Taf"), ("fa", "روندا کنون تاو"), ("fi", "Rhondda Cynon Taf"), ("fr", "Rhondda Cynon Taf"), ("ga", "Rhondda Cynon Taf"), ("hu", "Rhondda Cynon Taf"), ("it", "distretto di contea di Rhondda Cynon Taf"), ("ja", "ロンザ・カノン・タフ"), ("ko", "론다커논타브"), ("lt", "Ronda-Kinonas-Tafas"), ("nb", "Rhondda Cynon Taf"), ("nl", "Rhondda Cynon Taf"), ("no", "Rhondda Cynon Taf"), ("pl", "Rhondda Cynon Taff"), ("ro", "Rhondda Cynon Taff"), ("ru", "Ронта-Кинон-Тав"), ("sv", "Rhondda Cynon Taf"), ("uk", "Ронта-Кінон-Тав"), ("zh", "朗达卡嫩塔夫")]),
                        unofficial_name_list: ["Rhondda Cynon Taff"].to_vec(),
                    }
                ),
                (
                    "RDB",
                    Subdivision{
                        name: "Redbridge",
                        country_alpha2: Alpha2::GB,
                        code: "RDB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.58861), longitude: Some(0.0823976), max_latitude: Some(51.6290223), min_latitude: Some(51.5435688), max_longitude: Some(0.149485), min_longitude: Some(0.0086784)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Рэдбрыдж"), ("bg", "Редбридге"), ("bn", "রেডব\u{9cd}রিজ"), ("ca", "Redbridge"), ("ccp", "𑄢𑄬𑄖\u{11134}𑄝\u{11133}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Redbridge"), ("cs", "Redbridge"), ("cy", "Redbridge"), ("da", "Redbridge"), ("de", "London Borough of Redbridge"), ("en", "Redbridge"), ("es", "Municipio de Redbridge"), ("eu", "Redbridge"), ("fa", "منطقه ردبریج لندن"), ("fi", "Redbridge"), ("fr", "district londonien de Redbridge"), ("ga", "London Borough of Redbridge"), ("gu", "લ\u{a82}ડન બોરો ઓફ ર\u{ac7}ડબ\u{acd}રિજ"), ("he", "רדברידג׳"), ("hi", "र\u{947}डब\u{94d}रिज बरो"), ("hu", "Redbridge kerület"), ("is", "Redbridge"), ("it", "Redbridge"), ("ja", "レッドブリッジ・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ರ\u{cc6}ಡ\u{ccd}ಬ\u{ccd}ರ\u{cbf}ಡ\u{ccd}ಜ\u{ccd}"), ("ko", "레드브리지 구"), ("nb", "Redbridge"), ("nl", "Redbridge"), ("no", "Redbridge"), ("pl", "London Borough of Redbridge"), ("pt", "Redbridge"), ("ro", "Redbridge"), ("ru", "Редбридж"), ("sr", "Редбриџ"), ("sr_Latn", "Redbridž"), ("sv", "London Borough of Redbridge"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ரெட\u{bcd}ப\u{bcd}பிரிட\u{bcd}ஜ\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b} అఫ\u{c4d} ర\u{c46}డ\u{c4d} బ\u{c4d}ర\u{c3f}డ\u{c4d}జ\u{c4d}"), ("tr", "Redbridge"), ("uk", "Редбридж"), ("ur", "ریڈبرج بورو"), ("vi", "Khu Redbridge của Luân Đôn"), ("yue", "紅橋"), ("yue_Hans", "红桥"), ("zh", "雷德布里奇區")]),
                        unofficial_name_list: ["Redbridge"].to_vec(),
                    }
                ),
                (
                    "RDG",
                    Subdivision{
                        name: "Reading",
                        country_alpha2: Alpha2::GB,
                        code: "RDG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4542645), longitude: Some(-0.9781303), max_latitude: Some(51.4931339), min_latitude: Some(51.4097795), max_longitude: Some(-0.9284944), min_longitude: Some(-1.0636011)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Reading"), ("ar", "ريدنج"), ("az", "Ridinq"), ("be", "Рэдынг"), ("bg", "Рединг"), ("bn", "রিডিং"), ("ca", "Reading"), ("ccp", "𑄢\u{11128}𑄓\u{11128}\u{11101}"), ("ceb", "Reading (kondado)"), ("cs", "Reading"), ("cy", "Reading"), ("da", "Reading"), ("de", "Reading"), ("el", "Ρέντινγκ"), ("en", "Reading"), ("es", "Reading"), ("et", "Reading"), ("eu", "Reading"), ("fa", "ردینگ، انگلستان"), ("fi", "Reading"), ("fr", "Reading"), ("ga", "Reading"), ("gu", "રીડિ\u{a82}ગ"), ("he", "רדינג"), ("hi", "र\u{947}डि\u{902}ग"), ("hu", "Reading"), ("hy", "Ռեդինգ"), ("id", "Reading"), ("it", "Reading"), ("ja", "レディング"), ("kn", "ರೀಡ\u{cbf}ಂಗ\u{ccd}"), ("ko", "레딩"), ("lt", "Redingas"), ("lv", "Redinga"), ("mk", "Рединг"), ("ml", "റീഡിംഗ\u{d4d}"), ("ms", "Reading"), ("nb", "Reading"), ("nl", "Reading"), ("no", "Reading"), ("pl", "Reading"), ("pt", "Reading"), ("ro", "Reading"), ("ru", "Рединг"), ("sk", "Reading"), ("sl", "Reading"), ("sr", "Рединг"), ("sr_Latn", "Reding"), ("sv", "Reading"), ("sw", "Reading"), ("ta", "ர\u{bc0}டிங\u{bcd}"), ("te", "ర\u{c40}డ\u{c3f}ంగ\u{c4d}"), ("th", "เรด\u{e34}ง"), ("tr", "Reading"), ("uk", "Редінг"), ("ur", "ریڈنگ، بارکشائر"), ("vi", "Reading"), ("yue", "雷丁"), ("yue_Hans", "雷丁"), ("zh", "雷丁")]),
                        unofficial_name_list: ["Reading"].to_vec(),
                    }
                ),
                (
                    "RFW",
                    Subdivision{
                        name: "Renfrewshire",
                        country_alpha2: Alpha2::GB,
                        code: "RFW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.8298581), longitude: Some(-4.5428385), max_latitude: Some(55.9362966), min_latitude: Some(55.75953209999999), max_longitude: Some(-4.353106599999999), min_longitude: Some(-4.7840113)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Рэнфрушыр"), ("bg", "Ренфрушър"), ("bn", "রেনফ\u{9cd}রিউশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Renfrewshire"), ("ccp", "𑄢𑄬𑄚\u{11134}𑄜\u{11133}𑄢𑄬𑄅\u{1112a}𑄥𑄠𑄢\u{11134}"), ("ceb", "Renfrewshire"), ("cs", "Renfrewshire"), ("cy", "Swydd Renfrew"), ("da", "Renfrewshire"), ("de", "Renfrewshire"), ("en", "Renfrewshire"), ("es", "Renfrewshire"), ("et", "Renfrewshire"), ("eu", "Renfrewshire"), ("fa", "رنفروشر"), ("fi", "Renfrewshire"), ("fr", "Renfrewshire"), ("ga", "Comhairle Shiorrachd Rinn Friú"), ("gu", "ર\u{ac7}નફ\u{acd}ર\u{acd}ય\u{ac1}શાયર"), ("it", "Renfrewshire"), ("ja", "レンフルーシャー"), ("kn", "ರ\u{cc6}ನ\u{ccd}ಫ\u{ccd}ರ\u{cc2}ಷೈರ\u{ccd}"), ("ko", "렌프루셔"), ("lt", "Renfrušyras"), ("nb", "Renfrewshire"), ("nl", "Renfrewshire"), ("no", "Renfrewshire"), ("pl", "Renfrewshire"), ("pt", "Renfrewshire"), ("ro", "Renfrewshire"), ("ru", "Ренфрушир"), ("sv", "Renfrewshire"), ("ta", "ரேண\u{bcd}பிரயூஷிர\u{bcd}"), ("te", "ర\u{c46}న\u{c4d} ఫ\u{c4d}ర\u{c46}వ\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Ренфрюшир"), ("ur", "رینفریوشائر"), ("zh", "伦弗鲁郡")]),
                        unofficial_name_list: ["Renfrewshire"].to_vec(),
                    }
                ),
                (
                    "RIC",
                    Subdivision{
                        name: "Richmond upon Thames",
                        country_alpha2: Alpha2::GB,
                        code: "RIC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.46131099999999), longitude: Some(-0.303742), max_latitude: Some(51.4871999), min_latitude: Some(51.4213028), max_longitude: Some(-0.2523359), min_longitude: Some(-0.3303392)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريتشموند، لندن"), ("be", "Рычманд-апан-Тэмс"), ("ca", "Richmond upon Thames"), ("ccp", "𑄢\u{11128}𑄌\u{11134}𑄟\u{11127}𑄚\u{11133}𑄓\u{11134} 𑄃𑄛\u{11127}𑄚\u{11134} 𑄗𑄟\u{11134}"), ("ceb", "Richmond upon Thames"), ("cs", "Richmond"), ("cy", "Richmond upon Thames"), ("da", "Richmond upon Thames"), ("de", "London Borough of Richmond upon Thames"), ("el", "Ρίτσμοντ"), ("en", "Richmond upon Thames"), ("es", "Richmond upon Thames"), ("eu", "Richmond upon Thames"), ("fa", "منطقه ریچموند آپون تیمز لندن"), ("fi", "Richmond upon Thames"), ("fr", "Richmond upon Thames"), ("ga", "Buirg Londan Richmond ar Tamais"), ("he", "ריצ׳מונד שעל התמזה"), ("hi", "रिचम\u{902}ड अपॉन ट\u{947}म\u{94d}स बरो"), ("hu", "Richmond upon Thames kerület"), ("hy", "Ռիչմոնդ ափոն Թեմզա"), ("is", "Richmond upon Thames"), ("it", "Borgo londinese di Richmond upon Thames"), ("ja", "リッチモンド・アポン・テムズ・ロンドン特別区"), ("ko", "리치먼드어폰템스 구"), ("mk", "Ричмонд на Темза"), ("mr", "रिचम\u{902}ड अपॉन थ\u{947}म\u{94d}स"), ("nb", "Richmond upon Thames"), ("nl", "Richmond upon Thames"), ("no", "Richmond upon Thames"), ("pl", "London Borough of Richmond upon Thames"), ("pt", "Richmond upon Thames"), ("ro", "Richmond upon Thames"), ("ru", "Ричмонд-апон-Темс"), ("sl", "Richmond upon Thames"), ("sr", "Лондонска општина Ричмонд на Темзи"), ("sr_Latn", "Londonska opština Ričmond na Temzi"), ("sv", "London Borough of Richmond upon Thames"), ("tr", "Richmond upon Thames"), ("uk", "Річмонд-на-Темзі"), ("ur", "رچمنڈ اپون تھیمز بورو"), ("zh", "泰晤士河畔列治文區")]),
                        unofficial_name_list: ["Richmond upon Thames"].to_vec(),
                    }
                ),
                (
                    "ROT",
                    Subdivision{
                        name: "Rotherham",
                        country_alpha2: Alpha2::GB,
                        code: "ROT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4326035), longitude: Some(-1.3635009), max_latitude: Some(53.46448580000001), min_latitude: Some(53.4024814), max_longitude: Some(-1.300166), min_longitude: Some(-1.4343391)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Родъръм"), ("ccp", "𑄢\u{1112e}𑄗𑄢\u{11134}𑄦𑄟\u{11134}"), ("ceb", "Rotherham (kondado)"), ("de", "Metropolitan Borough of Rotherham"), ("en", "Rotherham"), ("es", "Municipio metropolitano de Rotherham"), ("fa", "کلان\u{200c}شهر مستقل راترهام"), ("fr", "district métropolitain de Rotherham"), ("it", "Rotherham"), ("ja", "メトロポリタン・バラ・オブ・ロザラム"), ("ko", "로더럼 도시 자치구"), ("nb", "Rotherham"), ("nl", "Rotherham"), ("no", "Rotherham"), ("pl", "Metropolitan Borough of Rotherham"), ("ru", "Ротерем"), ("sv", "Rotherham"), ("uk", "Ротергем"), ("ur", "میٹروپولیٹن بورو روتھرہیم"), ("zh", "羅瑟勒姆都市自治市")]),
                        unofficial_name_list: ["Rotherham"].to_vec(),
                    }
                ),
                (
                    "RUT",
                    Subdivision{
                        name: "Rutland",
                        country_alpha2: Alpha2::GB,
                        code: "RUT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.6583014), longitude: Some(-0.639643), max_latitude: Some(52.7598017), min_latitude: Some(52.5247864), max_longitude: Some(-0.4283766), min_longitude: Some(-0.8217616)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rutland"), ("ar", "روتلاند"), ("bg", "Рътланд"), ("bn", "র\u{9be}টল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Rutland"), ("ccp", "𑄢\u{1112a}𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "District of Rutland"), ("cs", "Rutland"), ("cy", "Rutland"), ("da", "Rutland"), ("de", "Rutland"), ("en", "Rutland"), ("es", "Rutland"), ("et", "Rutland"), ("eu", "Rutland"), ("fa", "روتلند"), ("fi", "Rutland"), ("fr", "Rutland"), ("ga", "Rutland"), ("gl", "Rutland"), ("gu", "ર\u{ac1}ટલ\u{ac7}ન\u{acd}ડ"), ("he", "ראטלנד"), ("hi", "रटल\u{948}\u{902}ड"), ("hu", "Rutland"), ("id", "Rutland"), ("is", "Rutland"), ("it", "Rutland"), ("ja", "ラトランド"), ("kn", "ರುಟ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "러틀랜드 주"), ("lt", "Rutlandas"), ("lv", "Ratlenda"), ("mk", "Ратленд"), ("mr", "रटल\u{901}ड"), ("nb", "Rutland"), ("nl", "Rutland"), ("no", "Rutland"), ("pl", "Rutland"), ("pt", "Rutland"), ("ro", "Rutland"), ("ru", "Ратленд"), ("sk", "Rutland"), ("sr", "Ратланд"), ("sr_Latn", "Ratland"), ("sv", "Rutland"), ("ta", "ரூட\u{bcd}லண\u{bcd}ட\u{bcd}"), ("te", "రూట\u{c4d}ల\u{c4d}య\u{c3e}ండ\u{c4d}"), ("uk", "Рутленд"), ("ur", "راٹلینڈ"), ("vi", "Rutland"), ("yue", "律倫郡"), ("yue_Hans", "律伦郡"), ("zh", "拉特蘭")]),
                        unofficial_name_list: ["Rutland"].to_vec(),
                    }
                ),
                (
                    "SAW",
                    Subdivision{
                        name: "Sandwell",
                        country_alpha2: Alpha2::GB,
                        code: "SAW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5361674), longitude: Some(-2.010793), max_latitude: Some(52.569058), min_latitude: Some(52.460697), max_longitude: Some(-1.918163), min_longitude: Some(-2.0971142)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساندويل"), ("ccp", "𑄥\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄃\u{1112e}𑄠𑄬𑄣\u{11134}"), ("ceb", "Sandwell"), ("de", "Sandwell"), ("en", "Sandwell"), ("fa", "کلان\u{200c}شهر مستقل ساندول"), ("fr", "Sandwell"), ("it", "Sandwell"), ("ja", "サンドウェル"), ("ko", "샌드웰"), ("lt", "Sandvelas"), ("nb", "Sandwell"), ("nl", "Sandwell"), ("no", "Sandwell"), ("pl", "Sandwell"), ("ro", "Sandwell"), ("ru", "Сандуэлл"), ("sv", "Sandwell"), ("ur", "سینڈویل"), ("yue", "山威"), ("yue_Hans", "山威"), ("zh", "砂井")]),
                        unofficial_name_list: ["Sandwell"].to_vec(),
                    }
                ),
                (
                    "SAY",
                    Subdivision{
                        name: "South Ayrshire",
                        country_alpha2: Alpha2::GB,
                        code: "SAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.2701113), longitude: Some(-4.6524183), max_latitude: Some(55.6018707), min_latitude: Some(54.9976536), max_longitude: Some(-4.3988596), min_longitude: Some(-5.1242332)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Паўднёвы Эйршыр"), ("bg", "Южен Еършър"), ("bn", "স\u{9be}উথ আরশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "South Ayrshire"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄃𑄠𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "South Ayrshire"), ("cs", "Jižní Ayrshire"), ("cy", "De Swydd Ayr"), ("de", "South Ayrshire"), ("en", "South Ayrshire"), ("es", "South Ayrshire"), ("et", "South Ayrshire"), ("eu", "Hegoaldeko Ayrshire"), ("fa", "ایرشر جنوبی"), ("fi", "Etelä-Ayrshire"), ("fr", "South Ayrshire"), ("gu", "દક\u{acd}ષિણ આયરશાયર"), ("hr", "Južni Ayrshire"), ("it", "Ayrshire Meridionale"), ("ja", "サウス・エアシャー"), ("kk", "Саут-Эршир"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಐರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "사우스에어셔"), ("lt", "Pietų Eršyras"), ("nb", "South Ayrshire"), ("nl", "South Ayrshire"), ("no", "South Ayrshire"), ("pl", "South Ayrshire"), ("pt", "South Ayrshire"), ("ro", "South Ayrshire"), ("ru", "Южный Эйршир"), ("sv", "South Ayrshire"), ("ta", "தெற\u{bcd}கு அயர\u{bcd}ஸிர\u{bcd}"), ("te", "దక\u{c4d}షణ అయ\u{c3f}ర\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Південний Ершир"), ("ur", "جنوبی آئرشائر"), ("zh", "南艾尔郡")]),
                        unofficial_name_list: ["South Ayrshire"].to_vec(),
                    }
                ),
                (
                    "SCB",
                    Subdivision{
                        name: "Scottish Borders, The",
                        country_alpha2: Alpha2::GB,
                        code: "SCB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.54856969999999), longitude: Some(-2.7861388), max_latitude: Some(55.9462394), min_latitude: Some(55.1083444), max_longitude: Some(-2.0343537), min_longitude: Some(-3.539803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Скотыш-Бордэрс"), ("bg", "Шотландски граници"), ("bn", "স\u{9cd}কটিশ বর\u{9cd}ড\u{9be}র"), ("ca", "Scottish Borders"), ("ccp", "𑄌\u{11133}𑄇\u{11127}𑄑\u{11128}𑄌\u{11134} 𑄝\u{11127}𑄢\u{11134}𑄓𑄢\u{11134}"), ("ceb", "The Scottish Borders"), ("cs", "Scottish Borders"), ("cy", "Gororau’r Alban"), ("de", "Scottish Borders"), ("en", "Scottish Borders"), ("es", "Scottish Borders"), ("et", "Scottish Borders"), ("eu", "Scottish Borders"), ("fa", "اسکوتیش بوردرز"), ("fi", "Scottish Borders"), ("fr", "Scottish Borders"), ("ga", "Comhairle nan Críochan"), ("gu", "સ\u{acd}કોટિશ બોર\u{acd}ડર\u{acd}સ"), ("he", "דרום-מזרח סקוטלנד"), ("id", "Scottish Borders"), ("it", "Scottish Borders"), ("ja", "スコティッシュ・ボーダーズ"), ("kn", "ಸ\u{ccd}ಕಾಟ\u{cbf}ಷ\u{ccd} ಬಾರ\u{ccd}ಡರ\u{ccd}ಸ\u{ccd}"), ("ko", "스코티시보더스"), ("lt", "Škotų Sienos"), ("nb", "Scottish Borders"), ("nl", "Scottish Borders"), ("no", "Scottish Borders"), ("pl", "Scottish Borders"), ("pt", "Scottish Borders"), ("ro", "Scottish Borders"), ("ru", "Скоттиш-Бордерс"), ("sv", "Scottish Borders"), ("ta", "ஸ\u{bcd}க\u{bbe}ட\u{bcd}டிஷ\u{bcd} ப\u{bbe}ர\u{bcd}டர\u{bcd}ஸ\u{bcd}"), ("te", "స\u{c4d}క\u{c3e}ట\u{c3f}ష\u{c4d} బ\u{c3e}ర\u{c4d}దర\u{c4d}స\u{c4d}"), ("uk", "Шотландські кордони"), ("ur", "سکاٹش بارڈرز"), ("zh", "蘇格蘭邊區")]),
                        unofficial_name_list: ["The Scottish Border"].to_vec(),
                    }
                ),
                (
                    "SCT",
                    Subdivision{
                        name: "Scotland",
                        country_alpha2: Alpha2::GB,
                        code: "SCT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Skotland"), ("am", "ስኮትላንድ"), ("ar", "اسكتلندا"), ("az", "Şotlandiya"), ("be", "Шатландыя"), ("bg", "Шотландия"), ("bn", "স\u{9cd}কটল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Escòcia"), ("cs", "Skotsko"), ("da", "Scotland"), ("de", "Schottland"), ("el", "Σκωτία"), ("en", "Scotland"), ("es", "Escocia"), ("et", "Šotimaa"), ("eu", "Eskozia"), ("fa", "اسکاتلند"), ("fi", "Skotlanti"), ("fil", "Scotland"), ("fr", "Écosse"), ("gl", "Escocia"), ("gu", "સ\u{acd}કોટલ\u{ac7}ન\u{acd}ડ"), ("he", "סקוטלנד"), ("hi", "स\u{94d}कॉटल\u{948}\u{902}ड"), ("hr", "Škotska"), ("hu", "Skócia"), ("hy", "Շոտլանդիա"), ("id", "Skotlandia"), ("is", "Skotland"), ("it", "Scozia"), ("ja", "スコットランド"), ("ka", "შოტლანდია"), ("km", "ស\u{17d2}ក\u{17bb}តឡែន"), ("kn", "ಸ\u{ccd}ಕಾಟ\u{ccd}ಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "스코틀랜드"), ("lo", "ສະກ\u{eb1}ອດແລນ"), ("lt", "Škotija"), ("lv", "Skotija"), ("ml", "സ\u{d4d}\u{200c}കോട\u{d4d}ട\u{d4d}\u{200c}ലന\u{d4d}റ\u{d4d}"), ("mn", "Шотланд"), ("mr", "स\u{94d}कॉटल\u{902}ड"), ("ms", "Scotland"), ("nb", "Skottland"), ("ne", "स\u{94d}कटल\u{94d}याण\u{94d}ड"), ("nl", "Schotland"), ("pl", "Szkocja"), ("pt", "Escócia"), ("ro", "Scoția"), ("ru", "Шотландия"), ("si", "ස\u{dca}කොට\u{dca}ලන\u{dca}තය"), ("sk", "Škótsko"), ("sl", "Škotska"), ("sr", "Шкотска"), ("sv", "Skottland"), ("sw", "Uskoti"), ("ta", "ஸ\u{bcd}க\u{bbe}ட\u{bcd}ல\u{bbe}ந\u{bcd}து"), ("te", "స\u{c4d}క\u{c3e}ట\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "สกอตแลนด\u{e4c}"), ("tr", "İskoçya"), ("uk", "Шотландія"), ("ur", "اسکاٹ لینڈ"), ("vi", "Scotland"), ("zh", "苏格兰"), ("zh_Hant", "蘇格蘭")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SFK",
                    Subdivision{
                        name: "Suffolk",
                        country_alpha2: Alpha2::GB,
                        code: "SFK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.1872472), longitude: Some(0.9707800999999999), max_latitude: Some(52.5495175), min_latitude: Some(51.9321304), max_longitude: Some(1.7630301), min_longitude: Some(0.3399747)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suffolk"), ("ar", "سوفولك"), ("be", "графства Суфалк"), ("bg", "Съфолк"), ("bn", "স\u{9be}ফোক"), ("ca", "Suffolk"), ("ccp", "𑄥𑄜\u{1112e}𑄇\u{11134}"), ("ceb", "Suffolk (kondado)"), ("cs", "Suffolk"), ("cy", "Suffolk"), ("da", "Suffolk"), ("de", "Suffolk"), ("el", "Σάφολκ"), ("en", "Suffolk"), ("es", "Suffolk"), ("et", "Suffolk"), ("eu", "Suffolk"), ("fa", "سافک"), ("fi", "Suffolk"), ("fr", "Suffolk"), ("ga", "Suffolk"), ("gl", "Suffolk"), ("gu", "સફોલ\u{acd}ક"), ("he", "סאפוק"), ("hi", "सफ\u{93c}क"), ("hr", "Suffolk"), ("hu", "Suffolk"), ("id", "Suffolk"), ("is", "Suffolk"), ("it", "Suffolk"), ("ja", "サフォーク"), ("ka", "საფოლკი"), ("kn", "ಸಫೊಲ\u{ccd}ಕ\u{ccd}"), ("ko", "서퍽 주"), ("lt", "Safolkas"), ("lv", "Safolka"), ("mk", "Сафолк"), ("mr", "सफोक"), ("ms", "Suffolk"), ("nb", "Suffolk"), ("nl", "Suffolk"), ("no", "Suffolk"), ("pl", "Suffolk"), ("pt", "Suffolk"), ("ro", "Suffolk"), ("ru", "Саффолк"), ("sk", "Suffolk"), ("sl", "Suffolk"), ("sr", "Сафок"), ("sr_Latn", "Safok"), ("sv", "Suffolk"), ("ta", "சஃபோல\u{bcd}க\u{bcd}"), ("te", "సఫ\u{c4b}ల\u{c4d}క\u{c4d}"), ("th", "ซ\u{e31}ฟฟอล\u{e4c}ก"), ("tr", "Suffolk"), ("uk", "Суффолк"), ("ur", "سافک"), ("vi", "Suffolk"), ("yue", "修福郡"), ("yue_Hans", "修福郡"), ("zh", "薩福克郡")]),
                        unofficial_name_list: ["Suffolk"].to_vec(),
                    }
                ),
                (
                    "SFT",
                    Subdivision{
                        name: "Sefton",
                        country_alpha2: Alpha2::GB,
                        code: "SFT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.503445), longitude: Some(-2.970359), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Сефтан"), ("ccp", "𑄥𑄬𑄛\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Sefton"), ("da", "Metropolitan Borough of Sefton"), ("de", "Metropolitan Borough of Sefton"), ("en", "Sefton"), ("fa", "کلان\u{200c}شهر مستقل سفتون"), ("fr", "Sefton"), ("he", "ספטון"), ("hu", "Metropolitan Borough of Sefton"), ("hy", "Սեֆթոն"), ("it", "Sefton"), ("ja", "メトロポリタン・バラ・オブ・セフトン"), ("ko", "세프턴 도시 자치구"), ("nb", "Sefton"), ("nl", "Sefton"), ("no", "Sefton"), ("pl", "Metropolitan Borough of Sefton"), ("pt", "Borough Metropolitano de Sefton"), ("ro", "Sefton"), ("ru", "Сефтон"), ("sv", "Sefton"), ("uk", "Сефтон"), ("ur", "میٹروپولیٹن بورو سیفٹن"), ("zh", "塞夫頓都會自治市")]),
                        unofficial_name_list: ["Sefton"].to_vec(),
                    }
                ),
                (
                    "SGC",
                    Subdivision{
                        name: "South Gloucestershire",
                        country_alpha2: Alpha2::GB,
                        code: "SGC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.531456), longitude: Some(-2.4547158), max_latitude: Some(51.6772991), min_latitude: Some(51.4159043), max_longitude: Some(-2.2521124), min_longitude: Some(-2.6956995)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Паўднёвы Глостэршыр"), ("bg", "Южен Глостършър"), ("bn", "স\u{9be}উথ গ\u{9cd}লৌচেস\u{9cd}ট\u{9be}রশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Gloucestershire del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄉\u{11133}𑄣\u{1112e}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "South Gloucestershire"), ("de", "South Gloucestershire"), ("en", "South Gloucestershire"), ("es", "South Gloucestershire"), ("fr", "South Gloucestershire"), ("gu", "સાઉથ ગ\u{acd}લોસ\u{ac7}સ\u{acd}ટરશાયર"), ("hu", "South Gloucestershire"), ("hy", "Հարավային Գլուսթերշիր"), ("it", "South Gloucestershire"), ("ja", "サウス・グロスターシャー"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಗ\u{ccd}ಲ\u{ccc}ಸ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "사우스글로스터셔"), ("lt", "Saut Glosteršyras"), ("nb", "South Gloucestershire"), ("nl", "South Gloucestershire"), ("no", "South Gloucestershire"), ("pl", "South Gloucestershire"), ("pt", "South Gloucestershire"), ("ro", "South Gloucestershire"), ("ru", "Южный Глостершир"), ("sv", "South Gloucestershire"), ("ta", "தெற\u{bcd}கு க\u{bcd}ளோஸ\u{bcd}ஸ\u{bcd}டேர\u{bcd}ஷிர\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ గ\u{c4d}ల\u{c4c}స\u{c46}స\u{c4d}టర\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Південний Глостершир"), ("ur", "جنوبی گلوسٹرشائر"), ("zh", "南告羅士打郡")]),
                        unofficial_name_list: ["South Gloucestershire"].to_vec(),
                    }
                ),
                (
                    "SHF",
                    Subdivision{
                        name: "Sheffield",
                        country_alpha2: Alpha2::GB,
                        code: "SHF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.38112899999999), longitude: Some(-1.470085), max_latitude: Some(53.4868828), min_latitude: Some(53.3045505), max_longitude: Some(-1.334953), min_longitude: Some(-1.6639593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sheffield"), ("ar", "شفيلد"), ("as", "শ\u{9cd}বেফিল\u{9cd}ড"), ("az", "Şeffild"), ("be", "Шэфілд"), ("bg", "Шефилд"), ("bn", "শেফিল\u{9cd}ড"), ("ca", "Sheffield"), ("ccp", "𑄥𑄬𑄜\u{11128}𑄣\u{11133}𑄓\u{11134}"), ("ceb", "Sheffield"), ("cs", "Sheffield"), ("da", "Sheffield"), ("de", "Sheffield"), ("el", "Σέφιλντ"), ("en", "Sheffield"), ("es", "Sheffield"), ("et", "Sheffield"), ("eu", "Sheffield"), ("fa", "شفیلد"), ("fi", "Sheffield"), ("fr", "Sheffield"), ("gl", "Sheffield"), ("gu", "શ\u{ac7}ફિલ\u{acd}ડ"), ("he", "שפילד"), ("hi", "श\u{947}फ\u{93c}ील\u{94d}ड"), ("hr", "Sheffield"), ("hu", "Sheffield"), ("hy", "Շեֆիլդ"), ("id", "Sheffield"), ("is", "Sheffield"), ("it", "Sheffield"), ("ja", "シェフィールド"), ("ka", "შეფილდი"), ("kn", "ಷ\u{cc6}ಫೀಲ\u{ccd}ಡ\u{ccd}"), ("ko", "셰필드"), ("lt", "Šefildas"), ("lv", "Šefīlda"), ("mr", "श\u{947}फील\u{94d}ड"), ("ms", "Sheffield"), ("nb", "Sheffield"), ("nl", "Sheffield"), ("no", "Sheffield"), ("pl", "Sheffield"), ("pt", "Sheffield"), ("ro", "Sheffield"), ("ru", "Шеффилд"), ("si", "ෂෙෆ\u{dd3}ල\u{dca}ඩ\u{dca}"), ("sk", "Sheffield"), ("sl", "Sheffield"), ("sr", "Шефилд"), ("sr_Latn", "Šefild"), ("sv", "Sheffield"), ("sw", "Sheffield"), ("ta", "செப\u{bc0}ல\u{bcd}டு"), ("te", "ష\u{c46}ఫ\u{c40}ల\u{c4d}డ\u{c4d}"), ("th", "เชฟฟ\u{e35}ลด\u{e4c}"), ("tr", "Sheffield"), ("uk", "Шеффілд"), ("ur", "شیفیلڈ"), ("vi", "Sheffield"), ("zh", "谢菲尔德")]),
                        unofficial_name_list: ["Sheffield"].to_vec(),
                    }
                ),
                (
                    "SHN",
                    Subdivision{
                        name: "St. Helens",
                        country_alpha2: Alpha2::GB,
                        code: "SHN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.456307), longitude: Some(-2.737095), max_latitude: Some(53.486025), min_latitude: Some(53.4161166), max_longitude: Some(-2.677342), min_longitude: Some(-2.780963)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄬𑄚\u{11134} 𑄦𑄬𑄣𑄬𑄚\u{11134}𑄌\u{11134}"), ("ceb", "St. Helens"), ("da", "St. Helens (Merseyside)"), ("de", "Metropolitan Borough of St Helens"), ("en", "Saint Helens"), ("fa", "کلان\u{200c}شهر مستقل سنت هلنز"), ("fr", "district métropolitain de Saint Helens"), ("it", "Metropolitan Borough of St Helens"), ("ja", "セントヘレンズ (マージーサイド)"), ("ko", "세인트헬렌스 도시 자치구"), ("nb", "St. Helens"), ("nl", "St Helens"), ("no", "St. Helens"), ("pl", "Metropolitan Borough of St Helens"), ("ru", "Сент-Хеленс"), ("sv", "St. Helens"), ("uk", "Сент-Геленс"), ("ur", "میٹروپولیٹن بورو سینٹ ہیلنز"), ("zh", "聖海倫斯都會自治市")]),
                        unofficial_name_list: ["St. Helens"].to_vec(),
                    }
                ),
                (
                    "SHR",
                    Subdivision{
                        name: "Shropshire",
                        country_alpha2: Alpha2::GB,
                        code: "SHR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5864845), longitude: Some(-2.7037501), max_latitude: Some(52.9983945), min_latitude: Some(52.3062678), max_longitude: Some(-2.2328985), min_longitude: Some(-3.2355653)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shropshire"), ("ar", "شروبشاير"), ("be", "графства Шропшыр"), ("bg", "Шропшър"), ("bn", "শ\u{9cd}রপশ\u{9be}র"), ("ca", "Shropshire"), ("ccp", "𑄥\u{11133}𑄢\u{1112e}𑄛\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Shropshire"), ("cs", "Shropshire"), ("cy", "Swydd Amwythig"), ("da", "Shropshire"), ("de", "Shropshire"), ("el", "Σρόπσαϊρ"), ("en", "Shropshire"), ("es", "Shropshire"), ("et", "Shropshire"), ("eu", "Shropshire"), ("fa", "شروپ\u{200c}شایر"), ("fi", "Shropshire"), ("fr", "Shropshire"), ("ga", "Shropshire"), ("gu", "શ\u{acd}રોપશાયર"), ("he", "שרופשייר"), ("hi", "श\u{94d}रॉपशायर"), ("hu", "Shropshire"), ("hy", "Շրոպշիր"), ("id", "Shropshire"), ("is", "Shropshire"), ("it", "Shropshire"), ("ja", "シュロップシャー"), ("kn", "ಶ\u{ccd}ರೊಪ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "슈롭셔 주"), ("lt", "Šropšyras"), ("lv", "Šropšīra"), ("mk", "Шропшир"), ("mr", "श\u{94d}रॉपशायर"), ("nb", "Shropshire"), ("nl", "Shropshire"), ("no", "Shropshire"), ("pl", "Shropshire"), ("pt", "Shropshire"), ("ro", "Shropshire"), ("ru", "Шропшир"), ("sk", "Shropshire"), ("sr", "Шропшир"), ("sr_Latn", "Šropšir"), ("sv", "Shropshire"), ("ta", "ஷ\u{bcd}ரோபஷிர\u{bcd}"), ("te", "ష\u{c4d}ర\u{c3e}ప\u{c4d}ష\u{c48}ర\u{c4d}"), ("tr", "Shropshire"), ("uk", "Шропшир"), ("ur", "شروپشائر"), ("vi", "Shropshire"), ("yue", "史樂郡"), ("yue_Hans", "史乐郡"), ("zh", "什罗普郡")]),
                        unofficial_name_list: ["Shropshire"].to_vec(),
                    }
                ),
                (
                    "SKP",
                    Subdivision{
                        name: "Stockport",
                        country_alpha2: Alpha2::GB,
                        code: "SKP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.41063159999999), longitude: Some(-2.1575332), max_latitude: Some(53.4548791), min_latitude: Some(53.3801502), max_longitude: Some(-2.0825958), min_longitude: Some(-2.2155281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Стокпарт"), ("bg", "Стокпорт"), ("ccp", "𑄌\u{11133}𑄑\u{11127}𑄇\u{11134}𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134}"), ("ceb", "Borough of Stockport"), ("de", "Metropolitan Borough of Stockport"), ("en", "Stockport"), ("fa", "کلان\u{200c}شهر مستقل استکپورت"), ("fr", "district métropolitain de Stockport"), ("it", "Metropolitan Borough of Stockport"), ("ja", "ストックポート・メトロポリタン特別区"), ("ko", "스톡포트 도시 자치구"), ("nb", "Stockport"), ("nl", "Stockport"), ("no", "Stockport"), ("pl", "Metropolitan Borough of Stockport"), ("ru", "Стокпорт"), ("sv", "Stockport"), ("uk", "Стокпорт"), ("ur", "میٹروپولیٹن بورو سٹاکپورٹ"), ("zh", "斯托克波特都市自治市")]),
                        unofficial_name_list: ["Stockport"].to_vec(),
                    }
                ),
                (
                    "SLF",
                    Subdivision{
                        name: "Salford",
                        country_alpha2: Alpha2::GB,
                        code: "SLF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.48752349999999), longitude: Some(-2.2901264), max_latitude: Some(53.51842809999999), min_latitude: Some(53.4648608), max_longitude: Some(-2.2451382), min_longitude: Some(-2.3310829)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سالفورد"), ("az", "Salford"), ("be", "Солфард"), ("ccp", "𑄥𑄣\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "City and Borough of Salford"), ("da", "City of Salford"), ("de", "City of Salford"), ("el", "Πόλη του Σάλφορντ"), ("en", "Salford"), ("es", "Salford"), ("eu", "Salford"), ("fa", "سالفورد"), ("fr", "cité de Salford"), ("he", "סלפורד"), ("it", "City of Salford"), ("ja", "シティ・オブ・サルフォード"), ("ko", "시티오브솔퍼드"), ("nb", "City of Salford"), ("nl", "City of Salford"), ("no", "City of Salford"), ("pl", "City of Salford"), ("ru", "Солфорд"), ("sv", "City and Borough of Salford"), ("tr", "Salford"), ("ur", "سالفورڈ شہر"), ("vi", "Salford"), ("zh", "索爾福德市")]),
                        unofficial_name_list: ["Salford"].to_vec(),
                    }
                ),
                (
                    "SLG",
                    Subdivision{
                        name: "Slough",
                        country_alpha2: Alpha2::GB,
                        code: "SLG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.51053839999999), longitude: Some(-0.5950405999999999), max_latitude: Some(51.5458426), min_latitude: Some(51.4679841), max_longitude: Some(-0.4900443), min_longitude: Some(-0.6601652)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Slough"), ("ar", "سلاو"), ("be", "Слау"), ("bg", "Слау"), ("ccp", "𑄥\u{11133}𑄣\u{1112e}𑄅\u{1112a}"), ("ceb", "Slough"), ("cs", "Slough"), ("da", "Slough"), ("de", "Slough"), ("el", "Σλάου"), ("en", "Slough"), ("es", "Slough"), ("fa", "اسلاو، انگلستان"), ("fi", "Slough"), ("fr", "Slough"), ("he", "סלאו"), ("hu", "Slough"), ("hy", "Սլաու"), ("is", "Slough"), ("it", "Slough"), ("ja", "スラウ"), ("ka", "სლაუ"), ("ko", "슬라우"), ("lt", "Slau"), ("nb", "Slough"), ("nl", "Slough"), ("no", "Slough"), ("pl", "Slough"), ("pt", "Slough"), ("ro", "Slough"), ("ru", "Слау"), ("sk", "Slough"), ("sl", "Slough, Berkshire, Anglija"), ("sr", "Слау"), ("sr_Latn", "Slau"), ("sv", "Slough"), ("uk", "Слау"), ("zh", "斯劳")]),
                        unofficial_name_list: ["Slough"].to_vec(),
                    }
                ),
                (
                    "SLK",
                    Subdivision{
                        name: "South Lanarkshire",
                        country_alpha2: Alpha2::GB,
                        code: "SLK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.5243038), longitude: Some(-3.7035077), max_latitude: Some(55.8444391), min_latitude: Some(55.2907676), max_longitude: Some(-3.396407), min_longitude: Some(-4.2825719)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Саўт-Ланаркшыр"), ("bg", "Южен Ланаркшър"), ("bn", "স\u{9be}উথ ল\u{9cd}য\u{9be}ন\u{9be}র\u{9cd}কশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "South Lanarkshire"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄣\u{11133}𑄠𑄚𑄢\u{11134}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "South Lanarkshire"), ("cs", "Jižní Lanarkshire"), ("cy", "De Swydd Lanark"), ("de", "South Lanarkshire"), ("en", "South Lanarkshire"), ("es", "South Lanarkshire"), ("et", "South Lanarkshire"), ("eu", "Hegoaldeko Lanarkshire"), ("fa", "لانارکشر جنوبی"), ("fi", "Etelä-Lanarkshire"), ("fr", "South Lanarkshire"), ("ga", "Comhairle Shiorrachd Lannraig a Deas"), ("gu", "દક\u{acd}ષિણ લ\u{ac7}નાર\u{acd}કશ\u{ac7}ર"), ("hi", "दक\u{94d}षिण ल\u{948}नार\u{94d}कशायर"), ("hr", "Južni Lanarkshire"), ("hu", "South Lanarkshire"), ("id", "South Lanarkshire"), ("is", "Suður-Lanarkshire"), ("it", "Lanarkshire Meridionale"), ("ja", "サウス・ラナークシャー"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಲನಾರ\u{ccd}ಕ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "사우스래너크셔"), ("lt", "Pietų Lanarkšyras"), ("nb", "South Lanarkshire"), ("nl", "South Lanarkshire"), ("no", "South Lanarkshire"), ("pl", "South Lanarkshire"), ("pt", "South Lanarkshire"), ("ro", "South Lanarkshire"), ("ru", "Саут-Ланаркшир"), ("sv", "South Lanarkshire"), ("ta", "தெற\u{bcd}கு ல\u{bbe}ன\u{bbe}ர\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ల\u{c3e}న\u{c4d}క\u{c3e}ర\u{c4d}ష\u{c48}ర\u{c4d}"), ("uk", "Південний Ланаркшир"), ("zh", "南拉纳克郡")]),
                        unofficial_name_list: ["South Lanarkshire"].to_vec(),
                    }
                ),
                (
                    "SND",
                    Subdivision{
                        name: "Sunderland",
                        country_alpha2: Alpha2::GB,
                        code: "SND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.906869), longitude: Some(-1.383801), max_latitude: Some(54.9441711), min_latitude: Some(54.8471515), max_longitude: Some(-1.3457474), min_longitude: Some(-1.4664999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄥𑄚\u{11134}𑄓𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Sunderland"), ("cy", "Dinas Sunderland"), ("de", "City of Sunderland"), ("en", "Sunderland"), ("fa", "ساندرلند"), ("fr", "cité de Sunderland"), ("hy", "Սանդերլենդ"), ("it", "City of Sunderland"), ("ja", "シティ・オブ・サンダーランド"), ("ko", "시티오브선덜랜드"), ("nb", "City of Sunderland"), ("nl", "City of Sunderland"), ("no", "City of Sunderland"), ("pl", "City of Sunderland"), ("ru", "Сандерленд"), ("sv", "Sunderland"), ("uk", "Сандерленд"), ("ur", "سنڈرلینڈ شہر"), ("zh", "桑德蘭市")]),
                        unofficial_name_list: ["Sunderland"].to_vec(),
                    }
                ),
                (
                    "SOL",
                    Subdivision{
                        name: "Solihull",
                        country_alpha2: Alpha2::GB,
                        code: "SOL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.411811), longitude: Some(-1.77761), max_latitude: Some(52.4583018), min_latitude: Some(52.3918444), max_longitude: Some(-1.7373952), min_longitude: Some(-1.8278389)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "раён Соліхал"), ("ccp", "𑄥\u{1112e}𑄣\u{11128}𑄦𑄣\u{11134}"), ("ceb", "Solihull"), ("de", "Metropolitan Borough of Solihull"), ("en", "Solihull"), ("fa", "کلان\u{200c}شهر مستقل سولیهال"), ("fr", "district métropolitain de Solihull"), ("it", "Solihull"), ("ja", "ソリフル首都区"), ("ko", "솔리헐 도시 자치구"), ("nb", "Solihull (distrikt)"), ("nl", "Solihull"), ("no", "Solihull (distrikt)"), ("pl", "Metropolitan Borough of Solihull"), ("ru", "Солихалл"), ("sv", "Solihull (storstadsdistrikt)"), ("ur", "میٹروپولیٹن بورو سولیہل"), ("zh", "索利赫爾區")]),
                        unofficial_name_list: ["Solihull"].to_vec(),
                    }
                ),
                (
                    "SOM",
                    Subdivision{
                        name: "Somerset",
                        country_alpha2: Alpha2::GB,
                        code: "SOM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.0587013), longitude: Some(-2.9499066), max_latitude: Some(51.3293472), min_latitude: Some(50.8208995), max_longitude: Some(-2.2444341), min_longitude: Some(-3.839803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Somerset"), ("ar", "مقاطعة سومرست"), ("az", "Somerset"), ("be", "Сомерсет"), ("bg", "Съмърсет"), ("bn", "সম\u{9be}রসেট"), ("ca", "Comtat de Somerset"), ("ccp", "𑄥\u{11127}𑄟𑄢\u{11134}𑄥𑄬𑄖\u{11134}"), ("ceb", "Somerset (kondado sa Hiniusang Gingharian)"), ("cs", "Somerset"), ("cy", "Gwlad yr Haf"), ("da", "Somerset"), ("de", "Somerset"), ("el", "Σόμερσετ"), ("en", "Somerset"), ("es", "Somerset"), ("et", "Somerset"), ("eu", "Somerset"), ("fa", "سامرست (شهرستان)"), ("fi", "Somerset"), ("fr", "Somerset"), ("ga", "Somerset"), ("gl", "Somerset"), ("gu", "સોમરસ\u{ac7}ટ"), ("he", "סאמרסט"), ("hi", "समरस\u{947}ट"), ("hu", "Somerset"), ("hy", "Սոմերսեթ"), ("id", "Somerset"), ("is", "Somerset"), ("it", "Somerset"), ("ja", "サマセット"), ("ka", "სომერსეტი"), ("kn", "ಸಾಮರ\u{ccd}ಸ\u{cc6}ಟ\u{ccd}"), ("ko", "서머싯 주"), ("lt", "Somersetas"), ("lv", "Somerseta"), ("mk", "Сомерсет"), ("mr", "सॉमरस\u{947}ट"), ("ms", "Somerset"), ("nb", "Somerset"), ("nl", "Somerset"), ("no", "Somerset"), ("pl", "Somerset"), ("pt", "Somerset"), ("ro", "Somerset"), ("ru", "Сомерсет"), ("sk", "Somerset"), ("sl", "Somerset"), ("sr", "Самерсет"), ("sr_Latn", "Samerset"), ("sv", "Somerset"), ("ta", "சொமேர\u{bcd}செட\u{bcd}"), ("te", "స\u{c4b}మర\u{c4d}స\u{c46}ట\u{c4d}"), ("th", "ซ\u{e31}มเมอร\u{e4c}เซต"), ("tr", "Somerset"), ("uk", "Сомерсет"), ("ur", "سامرسیٹ"), ("vi", "Somerset"), ("yue", "森麻錫郡"), ("yue_Hans", "森麻锡郡"), ("zh", "森麻實郡")]),
                        unofficial_name_list: ["Somerset"].to_vec(),
                    }
                ),
                (
                    "SOS",
                    Subdivision{
                        name: "Southend-on-Sea",
                        country_alpha2: Alpha2::GB,
                        code: "SOS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5459269), longitude: Some(0.7077123), max_latitude: Some(51.5767962), min_latitude: Some(51.5209098), max_longitude: Some(0.8209341), min_longitude: Some(0.6229178)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Southend-on-Sea"), ("az", "Sauthend-on-Si"), ("be", "Горад Саўтэнд-он-Сі"), ("bn", "স\u{9be}উথএন\u{9cd}ড-অন-সী"), ("ca", "Southend-on-Sea"), ("ccp", "𑄥𑄅\u{1112a}𑄘𑄢\u{11134}𑄚\u{11134}-𑄃\u{11127}𑄚\u{11134}-𑄥\u{11128}"), ("ceb", "Southend-on-Sea"), ("cs", "Southend-on-Sea"), ("cy", "Southend-on-Sea"), ("da", "Southend-on-Sea"), ("de", "Southend-on-Sea"), ("en", "Southend-on-Sea"), ("es", "Southend-on-Sea"), ("eu", "Southend-on-Sea"), ("fa", "ساوتند-آن-سی"), ("fi", "Southend-on-Sea"), ("fr", "Southend-on-Sea"), ("ga", "Southend-on-Sea"), ("gu", "સાઉથએન\u{acd}ડ-ઓન-સી"), ("he", "סאות׳אנד-און-סי"), ("hu", "Southend-on-Sea"), ("hy", "Սաութենդ-օն-Սի"), ("id", "Southend-on-Sea"), ("is", "Southend-on-Sea"), ("it", "Southend-on-Sea"), ("ja", "サウスエンド・オン・シー"), ("kn", "ಸ\u{ccc}ಥ\u{cc6}ಂಡ\u{ccd}-ಆನ\u{ccd}-ಸೀ"), ("ko", "사우스엔드온시"), ("lt", "Pajūrio Sautendas"), ("ms", "Southend-on-Sea"), ("nb", "Southend-on-Sea"), ("nl", "Southend-on-Sea"), ("no", "Southend-on-Sea"), ("pl", "Southend-on-Sea"), ("pt", "Southend-on-Sea"), ("ro", "Southend-on-Sea"), ("ru", "Саутенд-он-Си"), ("sr", "Саутенд на Мору"), ("sr_Latn", "Sautend na Moru"), ("sv", "Southend-on-Sea"), ("ta", "சௌதென\u{bcd}ட\u{bcd}-ஆன\u{bcd} -கடல\u{bcd}"), ("te", "స\u{c4c}త\u{c4d} ఎండ\u{c4d}-ఆన\u{c4d}-స\u{c40}"), ("tr", "Southend-on-Sea"), ("uk", "Саутенд-он-Сі"), ("ur", "ساوتھاینڈ-آن-سی"), ("yue", "修安"), ("yue_Hans", "修安"), ("zh", "濱海紹森德")]),
                        unofficial_name_list: ["Southend-on-Sea"].to_vec(),
                    }
                ),
                (
                    "SRY",
                    Subdivision{
                        name: "Surrey",
                        country_alpha2: Alpha2::GB,
                        code: "SRY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.31475930000001), longitude: Some(-0.5599501), max_latitude: Some(51.4715328), min_latitude: Some(51.0714964), max_longitude: Some(0.05821630000000001), min_longitude: Some(-0.8489291)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Surrey"), ("ar", "سري"), ("az", "Surrey"), ("be", "Сурэй"), ("bg", "Съри"), ("bn", "স\u{9be}রে"), ("ca", "Surrey"), ("ccp", "𑄥𑄢𑄬"), ("ceb", "Surrey (kondado)"), ("cs", "Surrey"), ("cy", "Surrey"), ("da", "Surrey"), ("de", "Surrey"), ("el", "Σάρρεϋ"), ("en", "Surrey"), ("es", "Surrey"), ("et", "Surrey"), ("eu", "Surrey"), ("fa", "ساری (انگلستان)"), ("fi", "Surrey"), ("fr", "Surrey"), ("ga", "Surrey"), ("gl", "Surrey"), ("gu", "સર\u{ac7}"), ("he", "סארי"), ("hi", "सरी"), ("hu", "Surrey"), ("hy", "Սյուրեյ"), ("id", "Surrey"), ("is", "Surrey"), ("it", "Surrey"), ("ja", "サリー"), ("ka", "სურეი"), ("kk", "Суррей"), ("kn", "ಸರ\u{ccd}ರ\u{cc6}"), ("ko", "서리 주"), ("lt", "Surėjus"), ("lv", "Sareja"), ("mk", "Сари"), ("mr", "सर\u{947}"), ("ms", "Surrey"), ("nb", "Surrey"), ("nl", "Surrey"), ("no", "Surrey"), ("pl", "Surrey"), ("pt", "Surrey"), ("ro", "Surrey"), ("ru", "Суррей"), ("sk", "Surrey"), ("sl", "Surrey"), ("sr", "Сари"), ("sr_Latn", "Sari"), ("sv", "Surrey"), ("ta", "சர\u{bcd}ரே"), ("te", "సర\u{c4d}ర\u{c47}"), ("th", "เซอร\u{e4c}ร\u{e35}ย\u{e4c}"), ("tr", "Surrey"), ("uk", "Суррей"), ("ur", "سرے"), ("vi", "Surrey"), ("yo", "Surrey"), ("yo_BJ", "Surrey"), ("yue", "舒梨郡"), ("yue_Hans", "舒梨郡"), ("zh", "薩里郡")]),
                        unofficial_name_list: ["Surrey"].to_vec(),
                    }
                ),
                (
                    "STE",
                    Subdivision{
                        name: "Stoke-on-Trent",
                        country_alpha2: Alpha2::GB,
                        code: "STE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.002668), longitude: Some(-2.179404), max_latitude: Some(53.09254689999999), min_latitude: Some(52.95133149999999), max_longitude: Some(-2.0792402), min_longitude: Some(-2.2402466)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ستوك أون ترينت"), ("az", "Stok-on-Trent"), ("be", "Горад Сток-он-Трэнт"), ("bg", "Стоук он Трент"), ("bn", "স\u{9cd}টোক অন ট\u{9cd}রেন\u{9cd}ট"), ("ca", "Stoke-on-Trent"), ("ccp", "𑄌\u{11133}𑄑\u{11127}𑄇\u{11134}-𑄃\u{11127}𑄚\u{11134}-𑄑\u{11133}𑄢𑄬𑄚\u{11134}𑄑\u{11134}"), ("ceb", "City of Stoke-on-Trent"), ("cs", "Stoke-on-Trent"), ("da", "Stoke-on-Trent"), ("de", "Stoke-on-Trent"), ("el", "Στόουκ ον Τρεντ"), ("en", "Stoke-on-Trent"), ("es", "Stoke-on-Trent"), ("et", "Stoke-on-Trent"), ("eu", "Stoke-on-Trent"), ("fa", "استوک-آن-ترنت"), ("fi", "Stoke-on-Trent"), ("fr", "Stoke-on-Trent"), ("gu", "સ\u{acd}ટોક-ઓન-ટ\u{acd}ર\u{ac7}ન\u{acd}ટ"), ("he", "סטוק-און-טרנט"), ("hi", "स\u{94d}टॉक ऑन ट\u{94d}र\u{947}\u{902}ट"), ("hu", "Stoke-on-Trent"), ("hy", "Սթոք-օն-Թրենտ"), ("id", "Stoke-on-Trent"), ("is", "Stoke-on-Trent"), ("it", "Stoke-on-Trent"), ("ja", "ストーク・オン・トレント"), ("ka", "სტოკ-ონ-ტრენტი"), ("kn", "ಸ\u{ccd}ಟೋಕ\u{ccd} ಆನ\u{ccd} ಟ\u{ccd}ರ\u{cc6}ಂಟ\u{ccd}"), ("ko", "스토크온트렌트"), ("lt", "Stokas prie Trento"), ("lv", "Stoka pie Trentas"), ("mr", "स\u{94d}टोक-ऑन-ट\u{94d}र\u{947}\u{902}ट"), ("ms", "Stoke-on-Trent"), ("nb", "Stoke-on-Trent"), ("nl", "Stoke-on-Trent"), ("no", "Stoke-on-Trent"), ("pl", "Stoke-on-Trent"), ("pt", "Stoke-on-Trent"), ("ro", "Stoke-on-Trent"), ("ru", "Сток-он-Трент"), ("si", "ස\u{dca}ටොක\u{dca} - ඕන\u{dca} - ට\u{dca}\u{200d}රෙන\u{dca}ට\u{dca}"), ("sl", "Stoke-on-Trent"), ("sr", "Стоук на Тренту"), ("sr_Latn", "Stouk na Trentu"), ("sv", "Stoke-on-Trent"), ("ta", "ஸ\u{bcd}ட\u{bcd}ரோக\u{bcd} -ஆன\u{bcd}ட\u{bcd}ரெ-ண\u{bcd}ட\u{bcd}"), ("te", "స\u{c4d}ట\u{c4b}క\u{c4d} ఆన\u{c4d} ట\u{c4d}ర\u{c46}ంట\u{c4d}"), ("th", "สโตก-ออน-เทรนต\u{e4c}"), ("tr", "Stoke-on-Trent"), ("uk", "Сток-он-Трент"), ("ur", "سٹوک آن ٹرینٹ"), ("vi", "Stoke-on-Trent"), ("zh", "特倫特河畔斯托克")]),
                        unofficial_name_list: ["Stoke-on-Trent"].to_vec(),
                    }
                ),
                (
                    "STG",
                    Subdivision{
                        name: "Stirling",
                        country_alpha2: Alpha2::GB,
                        code: "STG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(56.1165227), longitude: Some(-3.9369029), max_latitude: Some(56.1455382), min_latitude: Some(56.0918724), max_longitude: Some(-3.9043966), min_longitude: Some(-3.9750002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Стэрлінг"), ("bg", "Стърлинг"), ("ccp", "𑄌\u{11133}𑄑𑄢\u{11134}𑄣\u{11128}\u{11101}"), ("ceb", "Stirling"), ("cs", "Stirling"), ("cy", "Stirling"), ("de", "Stirling"), ("el", "Στέρλινγκ"), ("en", "Stirling"), ("es", "Stirling"), ("et", "Stirlingi regioon"), ("eu", "Stirling"), ("fa", "استیرلینگ (شهرستان)"), ("fi", "Stirling"), ("fr", "Stirling"), ("ga", "Comhairle Shruighlea"), ("he", "סטירלינג (מחוז)"), ("it", "Stirling"), ("ja", "スターリング"), ("ko", "스털링"), ("lt", "Sterlingas"), ("nb", "Stirling"), ("nl", "Stirling"), ("no", "Stirling"), ("pl", "Stirling"), ("pt", "Stirling"), ("ru", "Стерлинг"), ("sv", "Stirling"), ("uk", "Стерлінг"), ("ur", "سٹرلنگ"), ("zh", "史特灵")]),
                        unofficial_name_list: ["Stirling"].to_vec(),
                    }
                ),
                (
                    "STH",
                    Subdivision{
                        name: "Southampton",
                        country_alpha2: Alpha2::GB,
                        code: "STH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.90970040000001), longitude: Some(-1.4043509), max_latitude: Some(50.9561354), min_latitude: Some(50.8773718), max_longitude: Some(-1.3219878), min_longitude: Some(-1.478998)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Southampton"), ("ar", "ساوثهامبتون"), ("az", "Sauthempton"), ("be", "Горад Саўтгемптан"), ("bg", "Саутхамптън"), ("bn", "স\u{9be}উদ\u{9be}ম\u{9cd}পটন"), ("bs", "Southampton"), ("ca", "Southampton"), ("ccp", "𑄥𑄅\u{1112a}𑄗𑄟\u{11134}𑄛\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "City of Southampton"), ("cs", "Southampton"), ("cy", "Southampton"), ("da", "Southampton"), ("de", "Southampton"), ("el", "Σαουθάμπτον"), ("en", "Southampton"), ("es", "Southampton"), ("et", "Southampton"), ("eu", "Southampton"), ("fa", "ساوت\u{200c}همپتون"), ("fi", "Southampton"), ("fr", "Southampton"), ("ga", "Southampton"), ("gl", "Southampton"), ("gu", "સાઉથહ\u{ac7}મ\u{acd}પ\u{acd}ટન"), ("ha", "Southampton"), ("ha_NE", "Southampton"), ("he", "סאות׳המפטון"), ("hi", "साउथह\u{948}\u{902}पटन"), ("hr", "Southampton"), ("hu", "Southampton"), ("hy", "Սաութհեմփթոն"), ("id", "Southampton"), ("is", "Southampton"), ("it", "Southampton"), ("ja", "サウサンプトン"), ("ka", "საუთჰემპტონი"), ("kk", "Саутгемптон"), ("kn", "ಸ\u{ccc}ತಾಂಪ\u{ccd}ಟನ\u{ccd}"), ("ko", "사우샘프턴"), ("lt", "Sautamptonas"), ("lv", "Sauthemptona"), ("mk", "Саутхемптон"), ("mr", "साउथह\u{901}प\u{94d}टन"), ("ms", "Southampton"), ("my", "ဆောက\u{103a}သမ\u{1039}ပတန\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Southampton"), ("nl", "Southampton"), ("no", "Southampton"), ("pa", "ਸਾਊਥਹ\u{a48}\u{a02}ਪਟਨ"), ("pl", "Southampton"), ("pt", "Southampton"), ("ro", "Southampton"), ("ru", "Саутгемптон"), ("si", "සදම\u{dca}ප\u{dca}ටන\u{dca}"), ("sk", "Southampton"), ("sl", "Southampton"), ("sr", "Саутхемптон"), ("sr_Latn", "Sauthempton"), ("sv", "Southampton"), ("sw", "Southampton"), ("ta", "சௌத\u{bbe}ம\u{bcd}ப\u{bcd}டன\u{bcd}"), ("te", "స\u{c4c}త\u{c3e}ంప\u{c4d}టన\u{c4d}"), ("th", "เซาแทมป\u{e4c}ต\u{e31}น"), ("tr", "Southampton"), ("uk", "Саутгемптон"), ("ur", "ساؤتھمپٹن"), ("vi", "Southampton"), ("yue", "修咸頓"), ("yue_Hans", "修咸顿"), ("zh", "南安普敦")]),
                        unofficial_name_list: ["Southampton"].to_vec(),
                    }
                ),
                (
                    "STN",
                    Subdivision{
                        name: "Sutton",
                        country_alpha2: Alpha2::GB,
                        code: "STN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.3614279), longitude: Some(-0.193961), max_latitude: Some(51.3882532), min_latitude: Some(51.3292539), max_longitude: Some(-0.1694085), min_longitude: Some(-0.2407825)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة ساتون"), ("ca", "Sutton"), ("ccp", "𑄥𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Sutton (distrito sa Hiniusang Gingharian, Inglatera, Greater London, lat 51,36, long -0,19)"), ("cs", "Sutton"), ("cy", "Sutton"), ("da", "Sutton"), ("de", "London Borough of Sutton"), ("en", "Sutton"), ("es", "Municipio de Sutton"), ("eu", "Sutton"), ("fa", "منطقه ساتون لندن"), ("fr", "district londonien de Sutton"), ("he", "סאטון"), ("hi", "सटन बरो"), ("hu", "Sutton kerület"), ("is", "Sutton"), ("it", "London Borough of Sutton"), ("ja", "サットン・ロンドン特別区"), ("ko", "서턴 구"), ("nb", "Sutton"), ("nl", "Sutton"), ("no", "Sutton"), ("pl", "London Borough of Sutton"), ("pt", "Sutton"), ("ro", "Sutton"), ("ru", "Саттон"), ("sr", "Лондонска општина Сатон"), ("sr_Latn", "Londonska opština Saton"), ("sv", "London Borough of Sutton"), ("tr", "Sutton"), ("uk", "Саттон"), ("ur", "سٹن بورو"), ("zh", "薩頓區")]),
                        unofficial_name_list: ["Sutton"].to_vec(),
                    }
                ),
                (
                    "STS",
                    Subdivision{
                        name: "Staffordshire",
                        country_alpha2: Alpha2::GB,
                        code: "STS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.7697957), longitude: Some(-2.1045243), max_latitude: Some(53.2262238), min_latitude: Some(52.4232437), max_longitude: Some(-1.585467), min_longitude: Some(-2.4708418)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Staffordshire"), ("ar", "ستافوردشاير"), ("be", "Стафардшыр"), ("bg", "Стафордшър"), ("bn", "স\u{9cd}ট\u{9cd}য\u{9be}ফোর\u{9cd}ডশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Staffordshire"), ("ccp", "𑄌\u{11133}𑄑𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Staffordshire"), ("cs", "Staffordshire"), ("cy", "Swydd Stafford"), ("da", "Staffordshire"), ("de", "Staffordshire"), ("el", "Στάφορντσαϊρ"), ("en", "Staffordshire"), ("es", "Staffordshire"), ("et", "Staffordshire"), ("eu", "Staffordshire"), ("fa", "استافوردشایر"), ("fi", "Staffordshire"), ("fr", "Staffordshire"), ("ga", "Staffordshire"), ("gu", "સ\u{acd}ટ\u{ac7}ફોર\u{acd}ડશાયર"), ("he", "סטפורדשייר"), ("hi", "स\u{94d}ट\u{948}फ\u{93c}र\u{94d}डशायर"), ("hu", "Staffordshire"), ("hy", "Սթեֆորդշիր"), ("id", "Staffordshire"), ("is", "Staffordshire"), ("it", "Staffordshire"), ("ja", "スタッフォードシャー"), ("ka", "სტაფორდშირი"), ("kn", "ಸ\u{ccd}ಟಾಫರ\u{ccd}ಡ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "스태퍼드셔 주"), ("lt", "Stafordšyras"), ("lv", "Stefordšīra"), ("mk", "Стафордшир"), ("mr", "स\u{94d}ट\u{945}फर\u{94d}डशायर"), ("nb", "Staffordshire"), ("nl", "Staffordshire"), ("no", "Staffordshire"), ("pl", "Staffordshire"), ("pt", "Staffordshire"), ("ro", "Staffordshire"), ("ru", "Стаффордшир"), ("sk", "Staffordshire"), ("sl", "Staffordshire"), ("sr", "Стафордшир"), ("sr_Latn", "Stafordšir"), ("sv", "Staffordshire"), ("ta", "ஸ\u{bcd}ட\u{bbe}ப\u{bcd}பிபோர\u{bcd}டஷிர\u{bcd}"), ("te", "స\u{c4d}ట\u{c3e}ఫ\u{c4b}ర\u{c4d}డ\u{c4d} ష\u{c48}ర\u{c4d}"), ("th", "สแตฟฟอร\u{e4c}ดเชอร\u{e4c}"), ("tr", "Staffordshire"), ("uk", "Стаффордшир"), ("ur", "سٹیفورڈشائر"), ("vi", "Staffordshire"), ("yue", "史德福郡"), ("yue_Hans", "史德福郡"), ("zh", "斯塔福德郡")]),
                        unofficial_name_list: ["Staffordshire"].to_vec(),
                    }
                ),
                (
                    "STT",
                    Subdivision{
                        name: "Stockton-on-Tees",
                        country_alpha2: Alpha2::GB,
                        code: "STT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.5704551), longitude: Some(-1.3289821), max_latitude: Some(54.6452745), min_latitude: Some(54.476975), max_longitude: Some(-1.1535482), min_longitude: Some(-1.4363119)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "স\u{9cd}টকটন-অন-টিজ"), ("ccp", "𑄌\u{11133}𑄑\u{11127}𑄇\u{11134}𑄑\u{11127}𑄚\u{11134}-𑄃\u{11127}𑄚\u{11134}-𑄑\u{11128}𑄌\u{11134}"), ("ceb", "Stockton-on-Tees (kondado)"), ("de", "Borough of Stockton-on-Tees"), ("en", "Stockton-on-Tees"), ("es", "Municipio de Stockton-on-Tees"), ("fr", "Stockton-on-Tees"), ("gu", "સ\u{acd}ટોકટન-ઑન-ટીઝ"), ("it", "Stockton-on-Tees (borough)"), ("ja", "ストックトン-オン-ティーズ"), ("kn", "ಸ\u{ccd}ಟಾಕ\u{ccd}ಟನ\u{ccd}-ಆನ\u{ccd}-ಟೀಸ\u{ccd}"), ("ko", "스톡턴온티스 구"), ("lt", "Stoktonas"), ("nb", "Stockton-on-Tees (distrikt)"), ("nl", "Stockton-on-Tees"), ("no", "Stockton-on-Tees (distrikt)"), ("pl", "Stockton-on-Tees (borough)"), ("pt", "Stockton-on-Tees"), ("ru", "Стоктон-он-Тис"), ("sv", "Stockton-on-Tees (grevskap)"), ("ta", "ஸ\u{bcd}ட\u{bbe}க\u{bcd}டன\u{bcd} -ஆன\u{bcd} ட\u{bc0}ஸ\u{bcd}"), ("te", "స\u{c4d}ట\u{c3e}క\u{c4d}టన\u{c4d}-ఆన\u{c4d}-ట\u{c40}స\u{c4d}"), ("tr", "Stockton-on-Tees (borough)"), ("uk", "Стоктон-он-Тіз (район)"), ("ur", "بورو سٹاکٹن-آن-ٹیز"), ("zh", "蒂斯河畔斯托克頓區")]),
                        unofficial_name_list: ["Stockton-on-Tees"].to_vec(),
                    }
                ),
                (
                    "STY",
                    Subdivision{
                        name: "South Tyneside",
                        country_alpha2: Alpha2::GB,
                        code: "STY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.9636693), longitude: Some(-1.4418634), max_latitude: Some(55.01134219999999), min_latitude: Some(54.9284125), max_longitude: Some(-1.35239), min_longitude: Some(-1.5355124)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "South Tyneside"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄑\u{1112d}𑄛\u{11134}𑄥\u{1112d}𑄖\u{11134}"), ("ceb", "South Tyneside"), ("de", "South Tyneside"), ("en", "South Tyneside"), ("fa", "تاینساید جنوبی"), ("fi", "South Tyneside"), ("fr", "South Tyneside"), ("hu", "South Tyneside"), ("it", "South Tyneside"), ("ja", "サウス・タインサイド"), ("ko", "사우스타인사이드"), ("mk", "Јужен Тајнсајд"), ("nb", "South Tyneside"), ("nl", "South Tyneside"), ("no", "South Tyneside"), ("pl", "South Tyneside"), ("ro", "South Tyneside"), ("ru", "Саут-Тайнсайд"), ("sv", "South Tyneside"), ("uk", "Саут-Тайнсайд"), ("ur", "جنوبی ٹینیسائڈ"), ("zh", "南泰因賽德")]),
                        unofficial_name_list: ["South Tyneside"].to_vec(),
                    }
                ),
                (
                    "SWA",
                    Subdivision{
                        name: "Swansea [Abertawe GB-ATA]",
                        country_alpha2: Alpha2::GB,
                        code: "SWA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.62144), longitude: Some(-3.943645999999999), max_latitude: Some(51.6391493), min_latitude: Some(51.6118816), max_longitude: Some(-3.9289927), min_longitude: Some(-3.967001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Swansea"), ("ar", "سوانزي"), ("az", "Suonsi"), ("be", "Суонсі"), ("bg", "Суонзи"), ("bn", "সোয\u{9bc}\u{9be}নসি"), ("bs", "Swansea"), ("ca", "Swansea"), ("ccp", "𑄥\u{11127}𑄠𑄚\u{11134}𑄥\u{11128}"), ("ceb", "City and County of Swansea"), ("cs", "Swansea"), ("cy", "Dinas a Sir Abertawe"), ("da", "Swansea"), ("de", "Swansea"), ("el", "Σουόνσι"), ("en", "Swansea"), ("es", "Swansea"), ("et", "Swansea"), ("eu", "Swansea"), ("fa", "سوانزی"), ("fi", "Swansea"), ("fr", "Swansea"), ("ga", "Abertawe"), ("gl", "Swansea"), ("gu", "સ\u{acd}વાનસી"), ("he", "סוונסי"), ("hi", "स\u{94d}वान\u{94d}ज\u{93c}ी"), ("hr", "Swansea"), ("hu", "Swansea"), ("hy", "Սուոնսի"), ("id", "Swansea"), ("is", "Swansea"), ("it", "Swansea"), ("ja", "スウォンジ"), ("kn", "ಸ\u{ccd}ವಾನ\u{ccd}ಸೀ"), ("ko", "스완지"), ("lt", "Svonsis"), ("lv", "Svonzi"), ("mk", "Свонзи"), ("mr", "स\u{94d}वॉन\u{94d}झी"), ("ms", "Swansea"), ("nb", "Swansea"), ("nl", "City and County of Swansea"), ("no", "Swansea"), ("pl", "Swansea"), ("pt", "Swansea"), ("ro", "Swansea"), ("ru", "Суонси"), ("si", "ස\u{dca}වැන\u{dca}සෝ"), ("sk", "Swansea"), ("sl", "Swansea"), ("sr", "Свонзи"), ("sr_Latn", "Svonzi"), ("sv", "City and County of Swansea"), ("ta", "ஸ\u{bcd}வ\u{bbe}ன\u{bcd}ஸ\u{bc0}"), ("te", "స\u{c4d}వ\u{c3e}న\u{c4d}జ\u{c40}"), ("th", "สวอนซ\u{e35}"), ("tr", "Swansea"), ("uk", "Свонсі"), ("ur", "سوانزی"), ("vi", "Swansea"), ("yue", "史雲斯"), ("yue_Hans", "史云斯"), ("zh", "斯旺西")]),
                        unofficial_name_list: ["Abertawe"].to_vec(),
                    }
                ),
                (
                    "SWD",
                    Subdivision{
                        name: "Swindon",
                        country_alpha2: Alpha2::GB,
                        code: "SWD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.55577390000001), longitude: Some(-1.7797176), max_latitude: Some(51.6091293), min_latitude: Some(51.530334), max_longitude: Some(-1.7194727), min_longitude: Some(-1.8651375)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "স\u{9c1}ইন\u{9cd}ডোন"), ("ccp", "𑄥\u{1112d}\u{1112a}𑄚\u{11134}𑄓\u{11127}𑄚\u{11134}"), ("ceb", "Borough of Swindon"), ("de", "Borough of Swindon"), ("en", "Swindon"), ("es", "Swindon"), ("fa", "سویندون"), ("fr", "Swindon (borough)"), ("gu", "સ\u{acd}વિન\u{acd}ડન"), ("it", "Swindon"), ("ja", "バラ・オブ・スウィンドン"), ("kn", "ಸ\u{ccd}ವ\u{cbf}ಂಡನ\u{ccd}"), ("ko", "스윈던 구"), ("nb", "Swindon"), ("nl", "Swindon"), ("no", "Swindon"), ("pl", "Swindon (borough)"), ("pt", "Swindon"), ("ru", "Суиндон (унитарная единица)"), ("sv", "Borough of Swindon"), ("ta", "ஸ\u{bcd}விண\u{bcd}டோன\u{bcd}"), ("te", "స\u{c4d}వ\u{c40}డన\u{c4d}"), ("uk", "Свіндон-боро"), ("ur", "بورو سوینڈون"), ("yue", "史雲頓區"), ("yue_Hans", "史云顿区"), ("zh", "史雲頓區")]),
                        unofficial_name_list: ["Swindon"].to_vec(),
                    }
                ),
                (
                    "SWK",
                    Subdivision{
                        name: "Southwark",
                        country_alpha2: Alpha2::GB,
                        code: "SWK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.502781), longitude: Some(-0.087738), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Саўтуарк"), ("ca", "Southwark"), ("ccp", "𑄥𑄅\u{1112a}𑄖\u{11134}𑄃\u{1112e}𑄠𑄢\u{11134}𑄇\u{11134}"), ("ceb", "Southwark"), ("cs", "Southwark"), ("cy", "Southwark"), ("da", "Southwark"), ("de", "London Borough of Southwark"), ("en", "Southwark"), ("es", "Municipio de Southwark"), ("eu", "Southwark"), ("fa", "منطقه ساوت\u{200c}وارک لندن"), ("fi", "Southwark"), ("fr", "district londonien de Southwark"), ("ga", "Buirg Londan Southwark"), ("he", "סאת׳ק"), ("hi", "सदक बरो"), ("hu", "London Borough of Southwark"), ("hy", "Սաութուրք"), ("id", "Southwark"), ("is", "Southwark"), ("it", "Borgo londinese di Southwark"), ("ja", "サザーク・ロンドン特別区"), ("ka", "საუთუარკი"), ("ko", "서더크 구"), ("lv", "Sautvorka"), ("mk", "Савак"), ("nb", "Southwark"), ("nl", "Southwark"), ("no", "Southwark"), ("pl", "London Borough of Southwark"), ("ro", "Southwark"), ("ru", "Саутуарк"), ("sl", "London Borough of Southwark"), ("sr", "Лондонска општина Садарк"), ("sr_Latn", "Londonska opština Sadark"), ("sv", "London Borough of Southwark"), ("uk", "Саутерк"), ("ur", "سدرک بورو"), ("vi", "Khu Southwark của Luân Đôn"), ("zh", "南華克區")]),
                        unofficial_name_list: ["Southwark"].to_vec(),
                    }
                ),
                (
                    "TAM",
                    Subdivision{
                        name: "Tameside",
                        country_alpha2: Alpha2::GB,
                        code: "TAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4805828), longitude: Some(-2.0809891), max_latitude: Some(53.5313437), min_latitude: Some(53.4259201), max_longitude: Some(-1.9633883), min_longitude: Some(-2.1696499)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "бора Тэймсайд"), ("ccp", "𑄑𑄬𑄟\u{11134}𑄥\u{1112d}𑄖\u{11134}"), ("ceb", "Borough of Tameside"), ("cy", "Tameside"), ("da", "Metropolitan Borough of Tameside"), ("de", "Tameside"), ("en", "Tameside"), ("fa", "تیمساید"), ("fr", "Tameside"), ("ga", "Tameside"), ("it", "Tameside"), ("ja", "テイムサイド"), ("ko", "테임사이드"), ("nb", "Tameside"), ("nl", "Tameside"), ("no", "Tameside"), ("pl", "Tameside"), ("ro", "Tameside"), ("ru", "Теймсайд"), ("sv", "Borough of Tameside"), ("ur", "ٹیمسائڈ"), ("zh", "坦姆赛德")]),
                        unofficial_name_list: ["Tameside"].to_vec(),
                    }
                ),
                (
                    "TFW",
                    Subdivision{
                        name: "Telford and Wrekin",
                        country_alpha2: Alpha2::GB,
                        code: "TFW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.7409916), longitude: Some(-2.4868586), max_latitude: Some(52.8283724), min_latitude: Some(52.6145437), max_longitude: Some(-2.3122116), min_longitude: Some(-2.6673647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "টেলফোর\u{9cd}ড ও রেকিন"), ("ccp", "𑄑𑄬𑄣\u{11134}𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134} 𑄃\u{11133}𑄃 𑄢𑄬𑄇\u{11128}𑄚\u{11134}"), ("ceb", "Telford and Wrekin"), ("cy", "Telford a Wrekin"), ("de", "Telford and Wrekin"), ("en", "Telford and Wrekin"), ("es", "Telford & Wrekin"), ("fr", "Telford et Wrekin"), ("gu", "ટ\u{ac7}લફોર\u{acd}ડ અન\u{ac7} વ\u{acd}ર\u{ac7}કીન"), ("it", "Telford and Wrekin"), ("ja", "テルフォード・アンド・レキン"), ("kn", "ಟ\u{cc6}ಲ\u{ccd}ಫೋರ\u{ccd}ಡ\u{ccd} ಮತ\u{ccd}ತು ವ\u{ccd}ರ\u{cc6}ಕ\u{cbf}ನ\u{ccd}"), ("ko", "텔퍼드 레킨"), ("nb", "Telford and Wrekin"), ("nl", "Telford and Wrekin"), ("no", "Telford and Wrekin"), ("pl", "Telford and Wrekin"), ("pt", "Telford and Wrekin"), ("ro", "Telford and Wrekin"), ("ru", "Телфорд и Рекин"), ("sv", "Telford and Wrekin"), ("ta", "டெலிபோர\u{bcd}ட\u{bcd} & ரக\u{bcd}கின\u{bcd}"), ("te", "ట\u{c46}ల\u{c4d}ఫ\u{c4b}ర\u{c4d}డ\u{c4d} మర\u{c3f}యు వర\u{c4d}క\u{c3f}న\u{c4d}"), ("uk", "Телфорд і Рекін"), ("ur", "ٹیلفورڈ اور ریکن"), ("zh", "特爾福德和雷金")]),
                        unofficial_name_list: ["Telford and Wrekin"].to_vec(),
                    }
                ),
                (
                    "THR",
                    Subdivision{
                        name: "Thurrock",
                        country_alpha2: Alpha2::GB,
                        code: "THR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4934557), longitude: Some(0.3529197), max_latitude: Some(51.5678194), min_latitude: Some(51.4510028), max_longitude: Some(0.5507865), min_longitude: Some(0.2104604)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ثوروك"), ("bn", "থ\u{9c1}ররক"), ("ccp", "𑄗\u{1112a}𑄢\u{1112e}𑄇\u{11134}"), ("ceb", "Borough of Thurrock"), ("de", "Thurrock"), ("en", "Thurrock"), ("es", "Thurrock"), ("fr", "Thurrock"), ("gu", "થ\u{ac1}રોક"), ("hu", "Thurrock"), ("it", "Thurrock"), ("ja", "サーロック"), ("kn", "ಥುರಾಕ\u{ccd}"), ("ko", "서럭"), ("lt", "Turokas"), ("nb", "Thurrock"), ("nl", "Thurrock"), ("no", "Thurrock"), ("pl", "Thurrock"), ("pt", "Thurrock"), ("ro", "Thurrock"), ("ru", "Таррок"), ("sv", "Thurrock"), ("ta", "துரர\u{bbe}க\u{bcd}"), ("te", "తుర\u{c4d}ర\u{c4b}క\u{c4d},"), ("uk", "Таррек"), ("ur", "تھاراک"), ("yue", "土洛克"), ("yue_Hans", "土洛克"), ("zh", "瑟羅克")]),
                        unofficial_name_list: ["Thurrock"].to_vec(),
                    }
                ),
                (
                    "TOB",
                    Subdivision{
                        name: "Torbay",
                        country_alpha2: Alpha2::GB,
                        code: "TOB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.4619209), longitude: Some(-3.525315), max_latitude: Some(50.502461), min_latitude: Some(50.4510189), max_longitude: Some(-3.4804955), min_longitude: Some(-3.5827901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Торбэй"), ("bg", "Торбей"), ("bn", "তেবে"), ("ccp", "𑄑\u{11127}𑄢\u{11134}𑄝𑄬"), ("ceb", "Borough of Torbay"), ("cs", "Torbay"), ("de", "Torbay"), ("en", "Torbay"), ("es", "Torbay"), ("et", "Torbay"), ("fr", "Torbay"), ("gu", "ટોર\u{acd}બ\u{ac7}"), ("it", "Torbay"), ("ja", "トーベイ"), ("kn", "ಟೊರ\u{ccd}ಬೇ"), ("ko", "토베이"), ("lt", "Torbajus"), ("mk", "Торбеј"), ("nb", "Torbay"), ("nl", "Torbay"), ("no", "Torbay"), ("pl", "Torbay"), ("pt", "Torbay"), ("ro", "Torbay"), ("ru", "Торбей"), ("sk", "Torbay"), ("sv", "Borough of Torbay"), ("ta", "ட\u{bbe}ர\u{bcd}ப\u{bbe}ய\u{bcd}"), ("te", "ట\u{c4a}ర\u{c4d}బ\u{c3e}య\u{c4d}"), ("uk", "Торбей"), ("ur", "ٹوربے"), ("zh", "托貝")]),
                        unofficial_name_list: ["Torbay"].to_vec(),
                    }
                ),
                (
                    "TOF",
                    Subdivision{
                        name: "Torfaen [Tor-faen]",
                        country_alpha2: Alpha2::GB,
                        code: "TOF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.696), longitude: Some(-3.063), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تورفين"), ("bg", "Торвайн"), ("bn", "টরফেন ক\u{9be}উন\u{9cd}টি বরো"), ("ca", "Torfaen"), ("ccp", "𑄑\u{11127}𑄢\u{11134}𑄜𑄬𑄚\u{11134}"), ("ceb", "Torfaen County Borough"), ("cy", "Torfaen"), ("de", "Torfaen"), ("en", "Torfaen"), ("es", "Torfaen"), ("et", "Torfaen"), ("eu", "Torfaen"), ("fa", "تورواین"), ("fi", "Torfaen"), ("fr", "Torfaen"), ("ga", "Tor-faen"), ("gu", "ટોરફાએન કાઉન\u{acd}ટી બોરો"), ("it", "distretto di contea di Torfaen"), ("ja", "トルヴァエン"), ("kn", "ಟೊರ\u{ccd}ಫಾನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "토르바인"), ("lt", "Torfainas"), ("nb", "Torfaen"), ("nl", "Torfaen"), ("no", "Torfaen"), ("pl", "Torfaen"), ("pt", "Torfaen"), ("ro", "Torfaen"), ("ru", "Торвайн"), ("sv", "Torfaen"), ("ta", "டோரப\u{bbe}யின\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "ట\u{c3e}ర\u{c4d}ఫ\u{c3e}న\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("tr", "Torfaen"), ("uk", "Торван"), ("zh", "托法恩")]),
                        unofficial_name_list: ["Tor-faen"].to_vec(),
                    }
                ),
                (
                    "TRF",
                    Subdivision{
                        name: "Trafford",
                        country_alpha2: Alpha2::GB,
                        code: "TRF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4215132), longitude: Some(-2.3517263), max_latitude: Some(53.4803691), min_latitude: Some(53.35740819999999), max_longitude: Some(-2.2530396), min_longitude: Some(-2.4783332)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ترافورد"), ("bn", "ট\u{9cd}র\u{9be}ফোর\u{9cd}ড"), ("ccp", "𑄑\u{11133}𑄢𑄜\u{1112e}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Trafford"), ("cs", "Trafford"), ("da", "Trafford"), ("de", "Trafford"), ("el", "Τράφορντ"), ("en", "Trafford"), ("es", "Trafford"), ("fa", "ترافورد"), ("fr", "Trafford"), ("ga", "Trafford"), ("it", "Trafford"), ("ja", "トラフォード"), ("ko", "트래퍼드"), ("ms", "Trafford"), ("nb", "Trafford"), ("nl", "Trafford"), ("no", "Trafford"), ("pl", "Metropolitan Borough of Trafford"), ("pt", "Trafford"), ("ro", "Trafford"), ("ru", "Траффорд"), ("sl", "Trafford, Manchester"), ("sv", "Trafford (grevskap)"), ("tr", "Trafford"), ("ur", "ٹریفرڈ"), ("vi", "Trafford"), ("zh", "特拉福德")]),
                        unofficial_name_list: ["Trafford"].to_vec(),
                    }
                ),
                (
                    "TWH",
                    Subdivision{
                        name: "Tower Hamlets",
                        country_alpha2: Alpha2::GB,
                        code: "TWH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.52026069999999), longitude: Some(-0.0293396), max_latitude: Some(51.5446859), min_latitude: Some(51.484503), max_longitude: Some(0.0098639), min_longitude: Some(-0.0801899)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "London Borough of Tower Hamlets"), ("ar", "حي تاور هامليتس، لندن"), ("be", "Таўэр-Хэмлетс"), ("bn", "ট\u{9be}ওয\u{9bc}\u{9be}র হ\u{9cd}য\u{9be}মলেট\u{9cd}\u{200c}স"), ("ca", "Tower Hamlets"), ("ccp", "𑄑𑄤𑄢\u{11134} 𑄦𑄟\u{11134}𑄣𑄬𑄖\u{11134}𑄌\u{11134}"), ("ceb", "Tower Hamlets"), ("cs", "Tower Hamlets"), ("cy", "Tower Hamlets"), ("da", "Tower Hamlets"), ("de", "London Borough of Tower Hamlets"), ("en", "Tower Hamlets"), ("es", "Tower Hamlets"), ("et", "Tower Hamlets"), ("eu", "Tower Hamlets"), ("fa", "منطقه تاور هملتس لندن"), ("fi", "Tower Hamlets"), ("fr", "borough londonien de Tower Hamlets"), ("ga", "Buirg Londan Tower Hamlets"), ("gl", "Tower Hamlets"), ("gu", "લ\u{a82}ડન બોરો ઓફ ટાવર હ\u{ac7}મ\u{acd}લ\u{ac7}ટ\u{acd}સ"), ("he", "טאוור האמלטס"), ("hi", "टावर ह\u{948}मलट\u{94d}स बरो"), ("hu", "London Borough of Tower Hamlets"), ("is", "Tower Hamlets"), ("it", "Tower Hamlets"), ("ja", "タワーハムレッツ・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ಟವರ\u{ccd} ಹ\u{ccd}ಯಾಮ\u{ccd}ಲ\u{cc6}ಟ\u{ccd}ಸ\u{ccd}"), ("ko", "타워햄리츠 구"), ("ms", "Tower Hamlets"), ("nb", "Tower Hamlets"), ("nl", "Tower Hamlets"), ("no", "Tower Hamlets"), ("pl", "London Borough of Tower Hamlets"), ("pt", "Tower Hamlets"), ("ro", "Tower Hamlets"), ("ru", "Тауэр-Хэмлетс"), ("sv", "London Borough of Tower Hamlets"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} டவர\u{bcd} ஹ\u{bbe}ம\u{bcd}லெட\u{bcd}ஸ\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} టవర\u{c4d} హమ\u{c4d}ల\u{c46}ట\u{c4d}స\u{c4d}"), ("tr", "Tower Hamlets"), ("uk", "Тауер-Гемлетс"), ("ur", "ٹاور ہیملٹس بورو"), ("zh", "塔村區")]),
                        unofficial_name_list: ["Tower Hamlets"].to_vec(),
                    }
                ),
                (
                    "VGL",
                    Subdivision{
                        name: "Vale of Glamorgan, The [Bro Morgannwg GB-BMG]",
                        country_alpha2: Alpha2::GB,
                        code: "VGL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4443584), longitude: Some(-3.4151166), max_latitude: Some(51.5153438), min_latitude: Some(51.3813424), max_longitude: Some(-3.1637876), min_longitude: Some(-3.6439086)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Вейл ъф Гламорган"), ("bn", "ভেল অব গ\u{9cd}ল\u{9cd}য\u{9be}ম\u{9be}রগ\u{9be}ন"), ("ca", "Bro Morgannwg"), ("ccp", "𑄞𑄣\u{11134} 𑄃\u{11127}𑄛\u{11134} 𑄉\u{11133}𑄣𑄟\u{11127}𑄢\u{11134}𑄉\u{11133}𑄠𑄚\u{11134}"), ("ceb", "Vale of Glamorgan (munisipyo)"), ("cy", "Bro Morgannwg"), ("de", "Vale of Glamorgan"), ("el", "κοιλάδα του Γκλαμόργκαν"), ("en", "Vale of Glamorgan"), ("es", "Vale of Glamorgan"), ("et", "Vale of Glamorgan"), ("eu", "Vale of Glamorgan"), ("fa", "ویل آو گلامورگن"), ("fi", "Vale of Glamorgan"), ("fr", "Vale of Glamorgan"), ("ga", "Bro Morgannwg"), ("gu", "વ\u{ac7}લ ઓફ ગ\u{acd}લ\u{ac7}મોર\u{acd}ગન"), ("it", "distretto di contea di Vale of Glamorgan"), ("ja", "ヴェール・オブ・グラモーガン"), ("kn", "ವೇಲ\u{ccd} ಆಫ\u{ccd} ಗ\u{ccd}ಲಾಮೊರ\u{ccd}ಗನ\u{ccd}"), ("ko", "베일오브글러모건 주"), ("lt", "Glamorgano slėnis"), ("nb", "Vale of Glamorgan"), ("nl", "Vale of Glamorgan"), ("no", "Vale of Glamorgan"), ("pl", "Vale of Glamorgan"), ("pt", "Vale of Glamorgan"), ("ro", "Vale of Glamorgan"), ("ru", "Долина Гламорган"), ("sv", "Vale of Glamorgan"), ("ta", "கிளைமோர\u{bcd}கன\u{bcd} வ\u{bbe}லி"), ("te", "వ\u{c47}ల\u{c4d} ఆఫ\u{c4d} గ\u{c4d}ల\u{c3e}మ\u{c4b}ర\u{c4d}గ\u{c3e}న\u{c4d}"), ("uk", "Долина Гламорган"), ("zh", "格拉摩根谷")]),
                        unofficial_name_list: ["The Vale of Glamorgan"].to_vec(),
                    }
                ),
                (
                    "WAR",
                    Subdivision{
                        name: "Warwickshire",
                        country_alpha2: Alpha2::GB,
                        code: "WAR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.2671353), longitude: Some(-1.4675216), max_latitude: Some(52.6872436), min_latitude: Some(51.95539369999999), max_longitude: Some(-1.1721404), min_longitude: Some(-1.9620066)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Warwickshire"), ("ar", "ووريكشير"), ("az", "Uorvikşir"), ("be", "графства Уорыкшыр"), ("bg", "Уорикшър"), ("bn", "ওয\u{9bc}\u{9be}রউইকশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Warwickshire"), ("ccp", "𑄤𑄢\u{11134}𑄃\u{1112a}𑄃\u{11128}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Warwickshire"), ("cs", "Warwickshire"), ("cy", "Swydd Warwick"), ("da", "Warwickshire"), ("de", "Warwickshire"), ("el", "Γουόργουικσαϊρ"), ("en", "Warwickshire"), ("es", "Warwickshire"), ("et", "Warwickshire"), ("eu", "Warwickshire"), ("fa", "وارویک\u{200c}شایر"), ("fi", "Warwickshire"), ("fr", "Warwickshire"), ("ga", "Warwickshire"), ("gu", "વોરવિકશાયર"), ("he", "ווריקשייר"), ("hi", "वरिकशायर"), ("hu", "Warwickshire"), ("hy", "Ուորիքշիր"), ("id", "Warwickshire"), ("is", "Warwickshire"), ("it", "Warwickshire"), ("ja", "ウォリックシャー"), ("ka", "უორუიკშირი"), ("kn", "ವಾರ\u{ccd}ವ\u{cbf}ಕ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "워릭셔 주"), ("lt", "Vorikšyras"), ("lv", "Vorikšīra"), ("mk", "Ворикшир"), ("mr", "वॉरविकशायर"), ("nb", "Warwickshire"), ("nl", "Warwickshire"), ("no", "Warwickshire"), ("pl", "Warwickshire"), ("pt", "Warwickshire"), ("ro", "Warwickshire"), ("ru", "Уорикшир"), ("sk", "Warwickshire"), ("sl", "Warwickshire"), ("sr", "Ворикшир"), ("sr_Latn", "Vorikšir"), ("sv", "Warwickshire"), ("ta", "வரவிக\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", "వ\u{c3e}ర\u{c4d} వ\u{c3f}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("tr", "Warwickshire"), ("uk", "Ворікшир"), ("ur", "وارکشائر"), ("vi", "Warwickshire"), ("yue", "窩域郡"), ("yue_Hans", "窝域郡"), ("zh", "沃里克郡")]),
                        unofficial_name_list: ["Warwickshire"].to_vec(),
                    }
                ),
                (
                    "WBK",
                    Subdivision{
                        name: "West Berkshire",
                        country_alpha2: Alpha2::GB,
                        code: "WBK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4659863), longitude: Some(-1.2814014), max_latitude: Some(51.5637111), min_latitude: Some(51.3289786), max_longitude: Some(-0.9817454999999999), min_longitude: Some(-1.5880881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Заходні Беркшыр"), ("bg", "Западен Бъркшър"), ("bn", "ওয\u{9bc}েস\u{9cd}ট ব\u{9be}র\u{9cd}কশ\u{9be}য\u{9bc}\u{9be}র"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄝𑄢\u{11134}𑄇\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "West Berkshire"), ("da", "West Berkshire"), ("de", "West Berkshire"), ("el", "Δυτικό Μπέρκσαϊρ"), ("en", "West Berkshire"), ("es", "West Berkshire"), ("fa", "بارکشر غربی"), ("fr", "West Berkshire"), ("gu", "પશ\u{acd}ચિમ બર\u{acd}કશાયર"), ("hy", "Արևմտյան Բերկշիր"), ("it", "West Berkshire"), ("ja", "ウェスト・バークシャー"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಬರ\u{ccd}ಕ\u{ccd}ಷೈರ\u{ccd}"), ("ko", "웨스트버크셔"), ("nb", "West Berkshire"), ("nl", "West Berkshire"), ("no", "West Berkshire"), ("pl", "West Berkshire"), ("pt", "West Berkshire"), ("ro", "West Berkshire"), ("ru", "Западный Беркшир"), ("sl", "West Berkshire"), ("sv", "West Berkshire"), ("ta", "மேற\u{bcd}கு பேர\u{bcd}க\u{bcd}ஷிர\u{bcd}"), ("te", ", పశ\u{c4d}చ\u{c3f}మ బ\u{c46}ర\u{c4d}క\u{c4d} ష\u{c48}ర\u{c4d}"), ("uk", "Західний Беркшир"), ("ur", "مغربی بارکشائر"), ("zh", "西伯克郡")]),
                        unofficial_name_list: ["West Berkshire"].to_vec(),
                    }
                ),
                (
                    "WDU",
                    Subdivision{
                        name: "West Dunbartonshire",
                        country_alpha2: Alpha2::GB,
                        code: "WDU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.9650641), longitude: Some(-4.5063596), max_latitude: Some(56.0731382), min_latitude: Some(55.8891513), max_longitude: Some(-4.3754897), min_longitude: Some(-4.6599081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "دونبارتونشير الغربية"), ("be", "Заходні Дамбартаншыр"), ("bg", "Западен Дънбартъншър"), ("ca", "West Dunbartonshire"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄓𑄚\u{11134}𑄝𑄢\u{11134}𑄑\u{11127}𑄚\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "West Dunbartonshire"), ("cs", "Západní Dunbartonshire"), ("cy", "Gorllewin Swydd Dunbarton"), ("da", "West Dunbartonshire"), ("de", "West Dunbartonshire"), ("el", "Γουέστ Ντανμπάρτοσάιρ"), ("en", "West Dunbartonshire"), ("es", "West Dunbartonshire"), ("et", "West Dunbartonshire"), ("eu", "Mendebaldeko Dunbartonshire"), ("fa", "دونبارتونشر غربی"), ("fi", "Länsi-Dunbartonshire"), ("fr", "West Dunbartonshire"), ("ga", "Comhairle Dhún Breatainn an Iar"), ("he", "דאמברטון מערב"), ("hy", "Արևմտյան Դանբարտոնշիր"), ("it", "Dunbartonshire Occidentale"), ("ja", "ウェスト・ダンバートンシャー"), ("ko", "웨스트던바턴셔"), ("lt", "Vakarų Danbartonšyras"), ("nb", "West Dunbartonshire"), ("nl", "West Dunbartonshire"), ("no", "West Dunbartonshire"), ("pl", "West Dunbartonshire"), ("pt", "West Dunbartonshire"), ("ro", "West Dunbartonshire"), ("ru", "Западный Дамбартоншир"), ("sl", "West Dunbartonshire"), ("sv", "West Dunbartonshire"), ("uk", "Західний Данбартоншир"), ("zh", "西鄧巴頓郡")]),
                        unofficial_name_list: ["West Dunbartonshire"].to_vec(),
                    }
                ),
                (
                    "WFT",
                    Subdivision{
                        name: "Waltham Forest",
                        country_alpha2: Alpha2::GB,
                        code: "WFT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5886383), longitude: Some(-0.0117625), max_latitude: Some(51.64652659999999), min_latitude: Some(51.54992530000001), max_longitude: Some(0.0257123), min_longitude: Some(-0.0622746)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Уолтэм-Форэст"), ("bn", "ওয\u{9bc}\u{9be}লথম ফরেস\u{9cd}ট"), ("ca", "Waltham Forest"), ("ccp", "𑄤𑄣\u{11134}𑄗𑄟\u{11134} 𑄜\u{11127}𑄢𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Waltham Forest"), ("cs", "Waltham Forest"), ("cy", "Waltham Forest"), ("da", "Waltham Forest"), ("de", "London Borough of Waltham Forest"), ("en", "Waltham Forest"), ("es", "Waltham Forest"), ("eu", "Waltham Forest"), ("fa", "منطقه والتهام فورست لندن"), ("fi", "Waltham Forest"), ("fr", "district londonien de Waltham Forest"), ("ga", "Buirg Londan Waltham Forest"), ("gl", "Waltham Forest"), ("gu", "લ\u{a82}ડન બોરો ઓફ વોલ\u{acd}થમ ફોર\u{ac7}સ\u{acd}ટ"), ("he", "וולת׳אם פורסט"), ("hi", "वॉल\u{94d}थम फ\u{93c}ॉरस\u{94d}ट बरो"), ("hu", "Waltham Forest kerület"), ("is", "Waltham Forest"), ("it", "Waltham Forest"), ("ja", "ウォルサム・フォレスト・ロンドン特別区"), ("kn", "ಲಂಡನ\u{ccd} ಬರೋ ಆಫ\u{ccd} ವಾಲ\u{ccd}ತ\u{ccd} ಫಾರ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}"), ("ko", "월섬포리스트 구"), ("nb", "Waltham Forest"), ("nl", "Waltham Forest"), ("no", "Waltham Forest"), ("pl", "London Borough of Waltham Forest"), ("pt", "Waltham Forest"), ("ro", "Waltham Forest"), ("ru", "Уолтем-Форест"), ("sr", "Лондонска општина Волтам Форест"), ("sr_Latn", "Londonska opština Voltam Forest"), ("sv", "London Borough of Waltham Forest"), ("ta", "லண\u{bcd}டன\u{bcd} ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd} ஆப\u{bcd} வ\u{bbe}ள\u{bcd}த\u{bbe}ம\u{bcd} போரஸ\u{bcd}ட\u{bcd}"), ("te", "లండన\u{c4d} బ\u{c4b}ర\u{c4b}గ\u{c4d} అఫ\u{c4d} వ\u{c3e}ల\u{c4d}తం ఫ\u{c3e}ర\u{c46}స\u{c4d}ట\u{c4d}"), ("tr", "Waltham Forest"), ("uk", "Волтем-Форест"), ("ur", "والٹہیم جنگل بورو"), ("zh", "沃爾瑟姆福里斯特區")]),
                        unofficial_name_list: ["Waltham Forest"].to_vec(),
                    }
                ),
                (
                    "WGN",
                    Subdivision{
                        name: "Wigan",
                        country_alpha2: Alpha2::GB,
                        code: "WGN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.5450645), longitude: Some(-2.6325074), max_latitude: Some(53.57304329999999), min_latitude: Some(53.5014838), max_longitude: Some(-2.6003478), min_longitude: Some(-2.70243)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Уигън"), ("ccp", "𑄃\u{1112a}𑄃\u{11128}𑄉\u{11127}𑄚\u{11134}"), ("ceb", "Borough of Wigan"), ("de", "Metropolitan Borough of Wigan"), ("el", "Μητροπολιτικός Δήμος του Γουίγκαν"), ("en", "Wigan"), ("fa", "کلان\u{200c}شهر مستقل ویگان"), ("fr", "district métropolitain de Wigan"), ("it", "Metropolitan Borough of Wigan"), ("ja", "メトロポリタン・バラ・オブ・ウィガン"), ("ko", "위건 도시 자치구"), ("nb", "Wigan"), ("nl", "Wigan"), ("no", "Wigan"), ("pl", "Metropolitan Borough of Wigan"), ("ru", "метропольный боро в Уигане"), ("sv", "Wigan"), ("tr", "Wigan Metropoliten Borough"), ("ur", "میٹروپولیٹن بورو ویگان"), ("zh", "威根都市自治市")]),
                        unofficial_name_list: ["Wigan"].to_vec(),
                    }
                ),
                (
                    "WIL",
                    Subdivision{
                        name: "Wiltshire",
                        country_alpha2: Alpha2::GB,
                        code: "WIL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.2462714), longitude: Some(-1.9922127), max_latitude: Some(51.7031417), min_latitude: Some(50.944992), max_longitude: Some(-1.4857261), min_longitude: Some(-2.3655985)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wiltshire"), ("ar", "ويلتشير"), ("be", "Уілтшыр"), ("bg", "Уилтшър"), ("ca", "Wiltshire"), ("ccp", "𑄃\u{1112a}𑄃\u{11128}𑄣\u{11133}𑄑\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Wiltshire"), ("cs", "Wiltshire"), ("da", "Wiltshire"), ("de", "Wiltshire"), ("el", "Γουίλτσιρ"), ("en", "Wiltshire"), ("es", "Wiltshire"), ("et", "Wiltshire"), ("eu", "Wiltshire"), ("fa", "ویلتشایر"), ("fi", "Wiltshire"), ("fr", "Wiltshire"), ("he", "וילטשייר"), ("hi", "विल\u{94d}टशायर"), ("hu", "Wiltshire"), ("hy", "Ուիլտշիր"), ("id", "Wiltshire"), ("is", "Wiltshire"), ("it", "Wiltshire"), ("ja", "ウィルトシャー"), ("ka", "უილტშირი"), ("ko", "윌트셔 주"), ("lt", "Viltšyras"), ("lv", "Viltšīra"), ("mr", "विल\u{94d}टशायर"), ("nb", "Wiltshire"), ("nl", "Wiltshire"), ("no", "Wiltshire"), ("pl", "Wiltshire"), ("pt", "Wiltshire"), ("ro", "Wiltshire"), ("ru", "Уилтшир"), ("sk", "Wiltshire"), ("sl", "Wiltshire"), ("sr", "Вилтшир"), ("sr_Latn", "Viltšir"), ("sv", "Wiltshire"), ("th", "ว\u{e34}ลต\u{e4c}เชอร\u{e4c}"), ("tr", "Wiltshire"), ("uk", "Вілтшир"), ("ur", "ویلٹشائر"), ("vi", "Wiltshire"), ("zh", "威爾特郡")]),
                        unofficial_name_list: ["Wiltshire"].to_vec(),
                    }
                ),
                (
                    "WKF",
                    Subdivision{
                        name: "Wakefield",
                        country_alpha2: Alpha2::GB,
                        code: "WKF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.683298), longitude: Some(-1.505924), max_latitude: Some(53.7155472), min_latitude: Some(53.6360112), max_longitude: Some(-1.4638399), min_longitude: Some(-1.5566213)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Уэйкфілд"), ("bg", "Уейкфийлд"), ("ccp", "𑄤𑄠𑄬𑄇\u{11134}𑄜\u{11128}𑄣\u{11133}𑄓\u{11134}"), ("ceb", "City and Borough of Wakefield"), ("da", "City of Wakefield"), ("de", "City of Wakefield"), ("en", "Wakefield"), ("es", "Ciudad de Wakefield"), ("fa", "سیتی ویکفیلد"), ("fr", "cité de Wakefield"), ("he", "וייקפילד"), ("hu", "City of Wakefield"), ("it", "City of Wakefield"), ("ja", "シティ・オブ・ウェイクフィールド"), ("ko", "시티오브웨이크필드"), ("nb", "City of Wakefield"), ("nl", "Wakefield"), ("no", "City of Wakefield"), ("pl", "City of Wakefield"), ("pt", "Cidade de Wakefield"), ("ru", "Уэйкфилд"), ("sv", "Wakefield"), ("ta", "வேக\u{bcd}ப\u{bc0}ல\u{bcd}டு"), ("tr", "Wakefield"), ("uk", "Вейкфілд"), ("ur", "ویکفیلڈ شہر"), ("zh", "韋克菲爾德市")]),
                        unofficial_name_list: ["Wakefield"].to_vec(),
                    }
                ),
                (
                    "WLL",
                    Subdivision{
                        name: "Walsall",
                        country_alpha2: Alpha2::GB,
                        code: "WLL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.586214), longitude: Some(-1.982919), max_latitude: Some(52.63832619999999), min_latitude: Some(52.55088259999999), max_longitude: Some(-1.8871438), min_longitude: Some(-2.0401928)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bn", "ওয\u{9bc}\u{9be}লস\u{9be}ল"), ("ccp", "𑄤𑄣\u{11134}𑄥\u{11127}𑄣\u{11134}"), ("ceb", "Walsall"), ("de", "Metropolitan Borough of Walsall"), ("el", "Μητροπολιτικός Δήμος του Γουόλσολ"), ("en", "Walsall"), ("es", "Walsall"), ("fa", "کلان\u{200c}شهر مستقل والسال"), ("fr", "district métropolitain de Walsall"), ("gu", "વોલસોલ"), ("hu", "Walsall kerület"), ("it", "Walsall"), ("ja", "ワルサル"), ("kn", "ವಾಲ\u{ccd}ಸಾಲ\u{ccd}"), ("ko", "월솔 도시 자치구"), ("lt", "Volsolis"), ("nb", "Walsall"), ("nl", "Walsall"), ("no", "Walsall"), ("pl", "Metropolitan Borough of Walsall"), ("pt", "Walsall"), ("ru", "Уолсолл"), ("sv", "Walsall"), ("ta", "வ\u{bbe}ல\u{bcd}ச\u{bbe}ல\u{bcd}"), ("te", "వ\u{c3e}ల\u{c4d}స\u{c3e}ల\u{c4d}"), ("ur", "میٹروپولیٹن بورو والسال"), ("zh", "華素爾區")]),
                        unofficial_name_list: ["Walsall"].to_vec(),
                    }
                ),
                (
                    "WLN",
                    Subdivision{
                        name: "West Lothian",
                        country_alpha2: Alpha2::GB,
                        code: "WLN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(55.9070198), longitude: Some(-3.5517167), max_latitude: Some(56.002197), min_latitude: Some(55.7707171), max_longitude: Some(-3.3866287), min_longitude: Some(-3.8312582)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غرب لوثيان"), ("be", "Заходні Лотыян"), ("bg", "Западен Лоудиън"), ("bn", "ওয\u{9bc}েস\u{9cd}ট লোথিয\u{9bc}\u{9be}ন"), ("ca", "West Lothian"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄣\u{1112e}𑄗\u{11128}𑄠𑄚\u{11134}"), ("ceb", "West Lothian"), ("cs", "Západní Lothian"), ("cy", "Gorllewin Lothian"), ("da", "West Lothian"), ("de", "West Lothian"), ("en", "West Lothian"), ("es", "West Lothian"), ("et", "West Lothian"), ("eu", "Mendebaldeko Lothian"), ("fa", "لوتیان غربی"), ("fi", "Länsi-Lothian"), ("fr", "West Lothian"), ("ga", "Labhaidh Thoir"), ("gu", "પશ\u{acd}ચિમ લોથીયાન"), ("he", "ווסט לות׳יאן"), ("it", "Lothian dell’ovest"), ("ja", "ウェスト・ロージアン"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಲೋಥ\u{cbf}ಯನ\u{ccd}"), ("ko", "웨스트로디언"), ("lt", "Vakarų Lotianas"), ("mk", "Западен Лотијан"), ("nb", "West Lothian"), ("nl", "West Lothian"), ("no", "West Lothian"), ("pl", "West Lothian"), ("pt", "West Lothian"), ("ro", "West Lothian"), ("ru", "Уэст-Лотиан"), ("sv", "West Lothian"), ("ta", "மேற\u{bcd}கு லோதியன\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ ల\u{c4b}థ\u{c3f}యన\u{c4d}²"), ("uk", "Західний Лотіан"), ("ur", "مغربی لوتھین"), ("zh", "西洛锡安")]),
                        unofficial_name_list: ["West Lothian"].to_vec(),
                    }
                ),
                (
                    "WLS",
                    Subdivision{
                        name: "Wales",
                        country_alpha2: Alpha2::GB,
                        code: "WLS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Country,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wales"), ("am", "ዌልስ"), ("ar", "ويلز"), ("az", "Uels"), ("be", "Уэльс"), ("bg", "Уелс"), ("bn", "ওয়েল\u{9cd}স"), ("ca", "Gal·les"), ("cs", "Wales"), ("da", "Wales"), ("de", "Wales"), ("el", "Ουαλία"), ("en", "Wales"), ("es", "Gales"), ("et", "Wales"), ("eu", "Gales"), ("fa", "ویلز"), ("fi", "Wales"), ("fil", "Wales"), ("fr", "Pays de Galles"), ("gl", "Gales"), ("gu", "વોલ\u{acd}સ"), ("he", "ווילס"), ("hi", "व\u{947}ल\u{94d}स"), ("hr", "Wales"), ("hu", "Wales"), ("hy", "Ուելս"), ("id", "Wales"), ("is", "Wales"), ("it", "Galles"), ("ja", "ウェールズ"), ("ka", "უელსი"), ("km", "វ\u{17c9}ាល\u{17cb}ស\u{17cd}"), ("kn", "ವೇಲ\u{ccd}ಸ\u{ccd}\u{200c}"), ("ko", "웨일즈"), ("lo", "ເວລສ\u{ecc}"), ("lt", "Velsas"), ("lv", "Velsa"), ("ml", "വെയിൽസ\u{d4d}"), ("mn", "Уэльс"), ("mr", "व\u{947}ल\u{94d}स"), ("ms", "Wales"), ("nb", "Wales"), ("ne", "व\u{947}ल\u{94d}स"), ("nl", "Wales"), ("pl", "Walia"), ("pt", "País de Gales"), ("ro", "Țara Galilor"), ("ru", "Уэльс"), ("sd", "ويلز"), ("si", "වේල\u{dca}සය"), ("sk", "Wales"), ("sl", "Wales"), ("sr", "Велс"), ("sv", "Wales"), ("sw", "Wales"), ("ta", "வேல\u{bcd}ஸ\u{bcd}"), ("te", "వ\u{c47}ల\u{c4d}స\u{c4d}"), ("th", "เวลส\u{e4c}"), ("tr", "Galler"), ("uk", "Уельс"), ("ur", "ویلز"), ("vi", "Xứ Wales"), ("zh", "威尔士"), ("zh_Hant", "威爾斯")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WLV",
                    Subdivision{
                        name: "Wolverhampton",
                        country_alpha2: Alpha2::GB,
                        code: "WLV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.586973), longitude: Some(-2.12882), max_latitude: Some(52.6476987), min_latitude: Some(52.543167), max_longitude: Some(-2.0505977), min_longitude: Some(-2.2121953)}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wolverhampton"), ("ar", "ولفرهامبتن"), ("az", "Vulverhempton"), ("be", "Горад Вулверхэмптан"), ("bg", "Улвърхамптън"), ("bn", "উভ\u{9be}রহ\u{9cd}য\u{9be}ম\u{9cd}পটন"), ("ca", "Wolverhampton"), ("ccp", "𑄤𑄠\u{1112e}𑄣\u{11134}𑄞𑄢\u{11134}𑄦𑄟\u{11134}𑄛\u{11134}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "City of Wolverhampton"), ("cs", "Wolverhampton"), ("da", "Wolverhampton"), ("de", "Wolverhampton"), ("el", "Γουλβερχάμπτον"), ("en", "Wolverhampton"), ("es", "Wolverhampton"), ("et", "Wolverhampton"), ("eu", "Wolverhampton"), ("fa", "ولورهمپتون"), ("fi", "Wolverhampton"), ("fr", "Wolverhampton"), ("gl", "Wolverhampton"), ("gu", "વોલ\u{acd}વરહ\u{ac7}મ\u{acd}પ\u{acd}ટન"), ("he", "וולברהמפטון"), ("hi", "वॉल\u{94d}वरह\u{948}म\u{94d}प\u{94d}टन"), ("hu", "Wolverhampton"), ("hy", "Վուլվերհեմպտոն"), ("id", "Wolverhampton"), ("is", "Wolverhampton"), ("it", "Wolverhampton"), ("ja", "ウォルヴァーハンプトン"), ("ka", "ვულვერჰემპტონი"), ("kn", "ವೋಲ\u{ccd}ವರ\u{ccd} ಹಾಂಪ\u{ccd}ಟಾನ\u{ccd}"), ("ko", "울버햄프턴"), ("lt", "Vulverhamptonas"), ("lv", "Vulverhemptona"), ("mr", "वोल\u{94d}वरह\u{945}म\u{94d}प\u{94d}टन"), ("ms", "Wolverhampton"), ("nb", "Wolverhampton"), ("nl", "Wolverhampton"), ("no", "Wolverhampton"), ("pl", "Wolverhampton"), ("pt", "Wolverhampton"), ("ro", "Wolverhampton"), ("ru", "Вулвергемптон"), ("si", "ව\u{dd4}ල\u{dca}වර\u{dca}හැම\u{dca}ටන\u{dca}"), ("sl", "Wolverhampton"), ("sr", "Вулверхемптон"), ("sr_Latn", "Vulverhempton"), ("sv", "Wolverhampton"), ("ta", "ஓள\u{bcd}வேர\u{bcd}ஹ\u{bbe}ம\u{bcd}டன\u{bcd}"), ("te", "వ\u{c4b}ల\u{c4d}వ\u{c46}ర\u{c4d}హ\u{c3e}ంప\u{c4d}టన\u{c4d}"), ("th", "ว\u{e38}ลเวอร\u{e4c}แฮมป\u{e4c}ต\u{e31}น"), ("tr", "Wolverhampton"), ("uk", "Вулвергемптон"), ("ur", "وولورہیمپٹن"), ("vi", "Wolverhampton"), ("zh", "伍爾弗漢普頓")]),
                        unofficial_name_list: ["Wolverhampton"].to_vec(),
                    }
                ),
                (
                    "WND",
                    Subdivision{
                        name: "Wandsworth",
                        country_alpha2: Alpha2::GB,
                        code: "WND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4570716), longitude: Some(-0.1817824), max_latitude: Some(51.4858926), min_latitude: Some(51.417752), max_longitude: Some(-0.1263636), min_longitude: Some(-0.259113)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "واندزورث"), ("be", "Уондсуэрт"), ("ca", "Wandsworth"), ("ccp", "𑄤𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{1112e}𑄢\u{11134}𑄖\u{11134}"), ("ceb", "Wandsworth"), ("cs", "Wandsworth"), ("cy", "Wandsworth"), ("da", "Wandsworth"), ("de", "London Borough of Wandsworth"), ("el", "Δημοτικό διαμέρισμα Ουόντσουορθ"), ("en", "Wandsworth"), ("es", "Municipio de Wandsworth"), ("eu", "Wandsworth"), ("fa", "منطقه واندزوورث لندن"), ("fi", "Wandsworth"), ("fr", "district londonien de Wandsworth"), ("ga", "London Borough of Wandsworth"), ("gl", "Wandsworth"), ("he", "וונדסוורת׳"), ("hi", "व\u{902}ड\u{94d}सवर\u{94d}थ बरो"), ("hu", "London Borough of Wandsworth"), ("hy", "Ուոնդսուերտ"), ("id", "Wandsworth"), ("is", "Wandsworth"), ("it", "Wandsworth"), ("ja", "ワンズワース・ロンドン特別区"), ("ko", "완즈워스 구"), ("lv", "Vondsvērta"), ("nb", "Wandsworth"), ("nl", "Wandsworth"), ("no", "Wandsworth"), ("pl", "London Borough of Wandsworth"), ("pt", "Wandsworth"), ("ro", "Wandsworth"), ("ru", "Уондсуэрт"), ("sl", "Wandsworth"), ("sv", "London Borough of Wandsworth"), ("tr", "Wandsworth"), ("uk", "Вандзверт"), ("ur", "وینڈزورتھ بورو"), ("vi", "Khu Wandsworth của Luân Đôn"), ("zh", "旺茲沃思區")]),
                        unofficial_name_list: ["Wandsworth"].to_vec(),
                    }
                ),
                (
                    "WNM",
                    Subdivision{
                        name: "Windsor and Maidenhead",
                        country_alpha2: Alpha2::GB,
                        code: "WNM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4917059), longitude: Some(-0.7321755000000001), max_latitude: Some(51.57782539999999), min_latitude: Some(51.3828725), max_longitude: Some(-0.5227843999999999), min_longitude: Some(-0.8539262)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وندسور و ميدينهيد"), ("be", "Віндзар і Мэйдэнхэд"), ("bg", "Уиндзор и Мейдънхед"), ("bn", "উইন\u{9cd}ডসর ও মেইডেনল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ccp", "𑄃\u{1112a}𑄃\u{11128}𑄚\u{11133}𑄓\u{11134}𑄥\u{1112e}𑄢\u{11134} 𑄃\u{11133}𑄃 𑄟\u{1112d}𑄓𑄬𑄚\u{11134}𑄦𑄬𑄖\u{11134}"), ("ceb", "Royal Borough of Windsor and Maidenhead"), ("da", "Royal Borough of Windsor and Maidenhead"), ("de", "Windsor and Maidenhead"), ("en", "Windsor and Maidenhead"), ("es", "Windsor and Maidenhead"), ("fa", "شهر سلطنتی ویندسور و میدنهد"), ("fr", "Windsor and Maidenhead"), ("gu", "વિન\u{acd}ડસર અન\u{ac7} મ\u{ac8}ડનહ\u{ac7}ડ"), ("hy", "Վինձոր և Մեյդենհեդ"), ("it", "Royal Borough of Windsor and Maidenhead"), ("ja", "ウィンザー・アンド・メイデンヘッド"), ("kn", "ವ\u{cbf}ಂಡ\u{ccd}ಸರ\u{ccd} ಮತ\u{ccd}ತು ಮೇಡನ\u{ccd} ಹ\u{cc6}ಡ\u{ccd}"), ("ko", "윈저 메이든헤드 왕립구"), ("lt", "Vindzoras ir Meidenhedas"), ("nb", "Windsor and Maidenhead"), ("nl", "Windsor and Maidenhead"), ("no", "Windsor and Maidenhead"), ("pl", "Windsor and Maidenhead"), ("pt", "Windsor and Maidenhead"), ("ro", "Windsor and Maidenhead"), ("ru", "Виндзор и Мэйденхэд"), ("sv", "Royal Borough of Windsor and Maidenhead"), ("ta", "வின\u{bcd}ட\u{bcd}சர\u{bcd} & மைடேன\u{bcd}ஹெட\u{bcd}"), ("te", "వ\u{c3f}ండ\u{c4d}సర\u{c4d} మర\u{c3f}యు మ\u{c48}డ\u{c46}న\u{c4d}హ\u{c46}డ\u{c4d}"), ("tr", "Windsor and Maidenhead"), ("uk", "Віндзор і Мейденгед"), ("ur", "شاہی بورو ونڈسر اور میڈنہیڈ"), ("zh", "溫莎-梅登黑德")]),
                        unofficial_name_list: ["Windsor and Maidenhead"].to_vec(),
                    }
                ),
                (
                    "WOK",
                    Subdivision{
                        name: "Wokingham",
                        country_alpha2: Alpha2::GB,
                        code: "WOK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.410457), longitude: Some(-0.833861), max_latitude: Some(51.4298057), min_latitude: Some(51.3866601), max_longitude: Some(-0.7925223), min_longitude: Some(-0.8765881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Уокингам"), ("bn", "ওকিংহ\u{9be}ম"), ("ccp", "𑄃\u{1112e}𑄇\u{11128}\u{11101}𑄦𑄟\u{11134}"), ("ceb", "Wokingham"), ("da", "Borough of Wokingham"), ("de", "Wokingham"), ("en", "Wokingham"), ("es", "Wokingham"), ("fr", "Wokingham"), ("gu", "વોકિ\u{a82}ગહામ"), ("it", "Wokingham"), ("ja", "ウォーキンハム"), ("kn", "ವೋಕ\u{cbf}ಂಗ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd}"), ("ko", "오킹엄 구"), ("mk", "Вокингем"), ("nb", "Wokingham"), ("nl", "Wokingham"), ("no", "Wokingham"), ("pl", "Wokingham"), ("pt", "Wokingham"), ("ro", "Wokingham"), ("ru", "Уокингем"), ("sv", "Wokingham"), ("ta", "ஒக\u{bcd}கிங\u{bcd}ஹம\u{bcd}"), ("te", "వక\u{c3f}ంగ\u{c4d} హమ\u{c4d}"), ("uk", "Вокінгем"), ("ur", "بورو ووکنگہیم"), ("zh", "沃金厄姆區")]),
                        unofficial_name_list: ["Wokingham"].to_vec(),
                    }
                ),
                (
                    "WOR",
                    Subdivision{
                        name: "Worcestershire",
                        country_alpha2: Alpha2::GB,
                        code: "WOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.2545225), longitude: Some(-2.2668382), max_latitude: Some(52.4553026), min_latitude: Some(51.9665565), max_longitude: Some(-1.7574089), min_longitude: Some(-2.6632102)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Worcestershire"), ("ar", "وسترشير"), ("be", "графства Вустэршыр"), ("bg", "Устършър"), ("bn", "ওরচেস\u{9cd}ট\u{9be}রশ\u{9be}য\u{9bc}\u{9be}র"), ("ca", "Worcestershire"), ("ccp", "𑄃\u{1112e}𑄠𑄢\u{11134}𑄥𑄬𑄌\u{11134}𑄑𑄢\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "Worcestershire"), ("cs", "Worcestershire"), ("cy", "Swydd Gaerwrangon"), ("da", "Worcestershire"), ("de", "Worcestershire"), ("en", "Worcestershire"), ("es", "Worcestershire"), ("et", "Worcestershire"), ("eu", "Worcestershire"), ("fa", "ووسترشایر"), ("fi", "Worcestershire"), ("fr", "Worcestershire"), ("ga", "Worcestershire"), ("gu", "વોર\u{acd}ક\u{ac7}સ\u{acd}ટરશાયર"), ("he", "ווסטרשייר"), ("hi", "वॉस\u{94d}टरशायर"), ("hu", "Worcestershire"), ("hy", "Վուստերշիր"), ("id", "Worcestershire"), ("is", "Worcestershire"), ("it", "Worcestershire"), ("ja", "ウスターシャー"), ("kn", "ವರ\u{ccd}ಸ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "우스터셔 주"), ("lt", "Vusteršyras"), ("lv", "Vusteršīra"), ("mk", "Вустершир"), ("mr", "व\u{942}स\u{94d}टरशायर"), ("nb", "Worcestershire"), ("nl", "Worcestershire"), ("no", "Worcestershire"), ("pl", "Worcestershire"), ("pt", "Worcestershire"), ("ro", "Worcestershire"), ("ru", "Вустершир"), ("sk", "Worcestershire"), ("sl", "Worcestershire"), ("sr", "Вустершир"), ("sr_Latn", "Vusteršir"), ("sv", "Worcestershire"), ("ta", "ஒரஸ\u{bcd}ஸ\u{bcd}டேர\u{bcd}ஷிர\u{bcd}"), ("te", "వ\u{c4b}ర\u{c4d}స\u{c46}స\u{c4d}టర\u{c4d}ష\u{c48}ర\u{c4d}"), ("th", "ว\u{e38}ร\u{e4c}สเตอร\u{e4c}เชอร\u{e4c}"), ("tr", "Worcestershire"), ("uk", "Вустершир"), ("ur", "ووسٹرشائر"), ("vi", "Worcestershire"), ("yue", "窩士打郡"), ("yue_Hans", "窝士打郡"), ("zh", "伍斯特郡")]),
                        unofficial_name_list: ["Worcestershire"].to_vec(),
                    }
                ),
                (
                    "WRL",
                    Subdivision{
                        name: "Wirral",
                        country_alpha2: Alpha2::GB,
                        code: "WRL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.3333333), longitude: Some(-3.0833333), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanDistrict,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "раён Уірал"), ("bg", "Уиръл"), ("ccp", "𑄃\u{1112e}𑄠𑄢𑄬𑄣\u{11134}"), ("ceb", "Metropolitan Borough of Wirral"), ("da", "Wirral"), ("de", "Metropolitan Borough of Wirral"), ("en", "Wirral"), ("fa", "کلان\u{200c}شهر مستقل ویرال"), ("fr", "district métropolitain de Wirral"), ("he", "וויירל"), ("it", "Wirral"), ("ja", "ウィラル"), ("ko", "위럴 도시 자치구"), ("lt", "Viralas"), ("nb", "Wirral"), ("nl", "Wirral"), ("no", "Wirral"), ("pl", "Metropolitan Borough of Wirral"), ("ro", "Wirral"), ("ru", "Уиррал"), ("sv", "Wirral"), ("uk", "Віррал"), ("ur", "میٹروپولیٹن برو ویرل"), ("zh", "威勒爾都會自治市")]),
                        unofficial_name_list: ["Wirral"].to_vec(),
                    }
                ),
                (
                    "WRT",
                    Subdivision{
                        name: "Warrington",
                        country_alpha2: Alpha2::GB,
                        code: "WRT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.3900441), longitude: Some(-2.5969501), max_latitude: Some(53.4365627), min_latitude: Some(53.3536486), max_longitude: Some(-2.4493625), min_longitude: Some(-2.6977241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Уорынгтан"), ("bn", "ওয\u{9bc}\u{9be}রিংটন"), ("ca", "Warrington"), ("ccp", "𑄃\u{1112e}𑄠𑄢\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Warrington"), ("cs", "Warrington"), ("de", "Warrington"), ("en", "Warrington"), ("es", "Warrington"), ("et", "Warrington"), ("fa", "وارینگتون"), ("fi", "Warrington"), ("fr", "Warrington"), ("gu", "વોરિ\u{a82}ગટન"), ("he", "ורינגטון"), ("hu", "Warrington"), ("it", "Warrington"), ("ja", "ウォリントン"), ("kn", "ವಾರ\u{cbf}ಂಗ\u{ccd}ಟನ\u{ccd}"), ("ko", "워링턴"), ("lt", "Varingtonas"), ("nb", "Warrington"), ("nl", "Warrington"), ("no", "Warrington"), ("pl", "Warrington"), ("pt", "Warrington"), ("ro", "Warrington"), ("ru", "Уоррингтон"), ("sk", "Warrington"), ("sr", "Ворингтон"), ("sr_Latn", "Vorington"), ("sv", "Warrington"), ("ta", "வ\u{bbe}ரிங\u{bcd}க\u{bcd}டோன\u{bcd}"), ("te", "వర\u{c4d}ర\u{c3f}ంగ\u{c4d}టన\u{c4d}"), ("tr", "Warrington"), ("uk", "Воррінгтон"), ("ur", "وارنگٹن"), ("zh", "沃灵顿")]),
                        unofficial_name_list: ["Warrington"].to_vec(),
                    }
                ),
                (
                    "WRX",
                    Subdivision{
                        name: "Wrexham [Wrecsam GB-WRC]",
                        country_alpha2: Alpha2::GB,
                        code: "WRX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.04304), longitude: Some(-2.992494), max_latitude: Some(53.0730607), min_latitude: Some(53.0291108), max_longitude: Some(-2.9454931), min_longitude: Some(-3.0308429)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Рэксем"), ("bg", "Рексъм"), ("bn", "রেক\u{9cd}সহ\u{9be}ম ক\u{9be}উন\u{9cd}টি বোরো"), ("ca", "Wrexham"), ("ccp", "𑄢𑄬𑄇\u{11134}𑄦𑄟\u{11134}"), ("cy", "Bwrdeistref Sirol Wrecsam"), ("de", "Wrexham County Borough"), ("el", "Κομητειακός Δήμος Ρέξαμ"), ("en", "Wrexham"), ("es", "Wrexham County Borough"), ("et", "Wrexham"), ("eu", "Wrexham"), ("fa", "شهرستان مستقل ورکسام"), ("fi", "Wrexham"), ("fr", "Wrexham"), ("gu", "વ\u{acd}ર\u{ac7}ક\u{acd}શ\u{ac7}મ કાઉન\u{acd}ટી બોરો"), ("he", "רקסהאם"), ("it", "distretto di contea di Wrexham"), ("ja", "レクサム"), ("kn", "ವ\u{ccd}ರ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}ಹ\u{ccd}ಯಾಮ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf} ಬರೋ"), ("ko", "렉섬 자치시"), ("lt", "Reksamas"), ("nb", "Wrexham"), ("nl", "Wrexham"), ("no", "Wrexham"), ("pl", "Wrexham"), ("pt", "Wrexham County Borough"), ("ru", "Рексем"), ("sv", "Wrexham"), ("ta", "ரஸ\u{bcd}ஹ\u{bbe}ம\u{bcd} கவுண\u{bcd}டி ப\u{bbe}ரூக\u{bcd}ஹ\u{bcd}"), ("te", "వ\u{c4d}ర\u{c47}క\u{c4d}స\u{c4d}హ\u{c3e}మ\u{c4d} క\u{c4c}ంట\u{c40} బ\u{c4b}ర\u{c4b}"), ("uk", "Рексем"), ("zh", "雷克瑟姆")]),
                        unofficial_name_list: ["Wrecsam"].to_vec(),
                    }
                ),
                (
                    "WSM",
                    Subdivision{
                        name: "Westminster",
                        country_alpha2: Alpha2::GB,
                        code: "WSM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.5001754), longitude: Some(-0.1332326), max_latitude: Some(51.5397932), min_latitude: Some(51.4838163), max_longitude: Some(-0.1111016), min_longitude: Some(-0.2160886)}),
                        comments: None,
                        subdivision_type: SubdivisionType::LondonBorough,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "City of Westminster"), ("ar", "مدينة وستمنستر"), ("az", "Vestminster borosu"), ("be", "Вэстмінстэр"), ("bg", "Уестминстър"), ("bn", "ওয\u{9bc}েস\u{9cd}টমিনস\u{9cd}ট\u{9be}র শহর"), ("ca", "Ciutat de Westminster"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄌\u{11134}𑄟\u{11128}𑄚\u{11134}𑄌\u{11133}𑄑𑄢\u{11134}"), ("ceb", "City of Westminster"), ("cs", "Westminster"), ("cy", "Dinas Westminster"), ("da", "City of Westminster"), ("de", "City of Westminster"), ("el", "Πόλη του Ουεστμίνστερ"), ("en", "Westminster"), ("es", "Ciudad de Westminster"), ("et", "Westminster"), ("eu", "Westminster Hiria"), ("fa", "شهر وست\u{200c}مینستر"), ("fi", "City of Westminster"), ("fr", "cité de Westminster"), ("ga", "Cathair Westminster"), ("gl", "Cidade de Westminster"), ("gu", "પશ\u{acd}ચિમમિન\u{acd}સ\u{acd}ટરન\u{ac1}\u{a82} શહ\u{ac7}ર"), ("he", "הסיטי של וסטמינסטר"), ("hi", "सिटी ऑफ\u{93c} व\u{947}स\u{94d}टमि\u{902}स\u{94d}टर"), ("hu", "City of Westminster"), ("hy", "Վեստմինստեր"), ("id", "City of Westminster"), ("is", "Westminsterborg"), ("it", "Città di Westminster"), ("ja", "シティ・オブ・ウェストミンスター"), ("kn", "ಸ\u{cbf}ಟ\u{cbf} ಆಫ\u{ccd} ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}ಮ\u{cbf}ನ\u{cbf}ಸ\u{ccd}ಟರ\u{ccd}"), ("ko", "시티오브웨스트민스터"), ("lt", "Vestminsterio miestas"), ("lv", "Vestminstera"), ("mk", "Вестминстер"), ("mr", "सिटी ऑफ व\u{947}स\u{94d}टमिन\u{94d}स\u{94d}टर"), ("ms", "Bandar Westminster"), ("nb", "City of Westminster"), ("nl", "City of Westminster"), ("no", "City of Westminster"), ("pl", "City of Westminster"), ("pt", "Cidade de Westminster"), ("ro", "City of Westminster"), ("ru", "Вестминстер"), ("si", "වෙස\u{dca}ට\u{dca}ම\u{dd2}න\u{dd2}ස\u{dca}ටර\u{dca} නගරය"), ("sk", "City of Westminster"), ("sl", "City of Westminster"), ("sv", "City of Westminster"), ("ta", "வெஸ\u{bcd}ட\u{bcd}மின\u{bcd}ஸ\u{bcd}டர\u{bcd} நகரம\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d}మ\u{c3f}న\u{c4d}స\u{c4d}టర\u{c4d} నగరం"), ("th", "นครเวสต\u{e4c}ม\u{e34}นสเตอร\u{e4c}"), ("tr", "Westminster"), ("uk", "Вестмінстер"), ("ur", "ویسٹمنسٹر شہر"), ("vi", "Thành phố Westminster"), ("yue", "西敏市"), ("yue_Hans", "西敏市"), ("zh", "西敏市")]),
                        unofficial_name_list: ["Westminster"].to_vec(),
                    }
                ),
                (
                    "WSX",
                    Subdivision{
                        name: "West Sussex",
                        country_alpha2: Alpha2::GB,
                        code: "WSX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.9280143), longitude: Some(-0.4617075), max_latitude: Some(51.16730279999999), min_latitude: Some(50.72246759999999), max_longitude: Some(0.0445255), min_longitude: Some(-0.9575972)}),
                        comments: None,
                        subdivision_type: SubdivisionType::TwoTierCounty,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Sussex"), ("ar", "غرب ساسكس"), ("be", "Заходні Сусекс"), ("bg", "Западен Съсекс"), ("bn", "ওয\u{9bc}েস\u{9cd}ট স\u{9be}সেক\u{9cd}স"), ("ca", "West Sussex"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄥𑄥𑄬𑄇\u{11134}"), ("ceb", "West Sussex"), ("cs", "Západní Sussex"), ("cy", "Gorllewin Sussex"), ("da", "West Sussex"), ("de", "West Sussex"), ("el", "Δυτικό Σάσσεξ"), ("en", "West Sussex"), ("es", "Sussex Occidental"), ("et", "Lääne-Sussex"), ("eu", "Mendebaldeko Sussex"), ("fa", "ساسکس غربی"), ("fi", "West Sussex"), ("fr", "Sussex de l’Ouest"), ("ga", "Sussex Thiar"), ("gl", "West Sussex"), ("gu", "પશ\u{acd}ચિમ સ\u{ac1}સ\u{ac7}ક\u{acd}સ"), ("he", "מערב סאסקס"), ("hi", "व\u{947}स\u{94d}ट सस\u{947}क\u{94d}स"), ("hr", "Zapadni Sussex"), ("hu", "West Sussex"), ("hy", "Արևմտյան Սասեքս"), ("id", "West Sussex"), ("is", "Vestur-Sussex"), ("it", "West Sussex"), ("ja", "ウェスト・サセックス"), ("ka", "დასავლეთი სასექსი"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಸಸ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}"), ("ko", "웨스트서식스 주"), ("lt", "Vakarų Saseksas"), ("lv", "Vestsaseksa"), ("mk", "Западен Сасекс"), ("mr", "व\u{947}स\u{94d}ट सस\u{947}क\u{94d}स"), ("nb", "West Sussex"), ("nl", "West Sussex"), ("no", "West Sussex"), ("pl", "West Sussex"), ("pt", "West Sussex"), ("ro", "West Sussex"), ("ru", "Западный Суссекс"), ("sk", "West Sussex"), ("sl", "West Sussex"), ("sr", "Западни Сасекс"), ("sr_Latn", "Zapadni Saseks"), ("sv", "West Sussex"), ("ta", "மேற\u{bcd}கு சுஸெஸ\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d} సస\u{c4d}స\u{c46}క\u{c4d}స\u{c4d}"), ("th", "เวสต\u{e4c}ซ\u{e31}สเซกซ\u{e4c}"), ("tr", "Batı Sussex"), ("uk", "Західний Сассекс"), ("ur", "مغربی سسیکس"), ("vi", "West Sussex"), ("yue", "西修適士"), ("yue_Hans", "西修适士"), ("zh", "西薩塞克斯郡")]),
                        unofficial_name_list: ["West Sussex"].to_vec(),
                    }
                ),
                (
                    "YOR",
                    Subdivision{
                        name: "York",
                        country_alpha2: Alpha2::GB,
                        code: "YOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.95996510000001), longitude: Some(-1.0872979), max_latitude: Some(53.9912585), min_latitude: Some(53.9259345), max_longitude: Some(-1.0139137), min_longitude: Some(-1.1472695)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnitaryAuthority,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "York"), ("ar", "يورك"), ("az", "York"), ("be", "Горад Ёрк"), ("bg", "Йорк"), ("bn", "ইয\u{9bc}র\u{9cd}ক"), ("ca", "York"), ("ccp", "𑄃\u{11128}𑄠\u{11127}𑄢\u{11134}𑄇\u{11134}"), ("ceb", "City of York"), ("cs", "York"), ("da", "York"), ("de", "York"), ("el", "Γιόρκ"), ("en", "York"), ("es", "York"), ("et", "York"), ("eu", "York"), ("fa", "یورک"), ("fi", "York"), ("fr", "York"), ("gl", "York"), ("gu", "યોર\u{acd}ક"), ("he", "יורק"), ("hi", "यॉर\u{94d}क"), ("hr", "York"), ("hu", "York"), ("hy", "Յորք"), ("id", "York"), ("is", "York"), ("it", "York"), ("ja", "ヨーク"), ("ka", "იორკი"), ("kn", "ಯಾರ\u{ccd}ಕ\u{ccd}"), ("ko", "요크"), ("lt", "Jorkas"), ("lv", "Jorka"), ("mr", "यॉर\u{94d}क"), ("ms", "York"), ("nb", "York"), ("nl", "York"), ("no", "York"), ("pl", "York"), ("pt", "Iorque"), ("ro", "York"), ("ru", "Йорк"), ("si", "යෝක\u{dca}"), ("sk", "York"), ("sl", "York"), ("sr", "Јорк"), ("sr_Latn", "Jork"), ("sv", "York"), ("sw", "York"), ("ta", "ய\u{bbe}ர\u{bcd}க\u{bcd}"), ("te", "య\u{c3e}ర\u{c4d}క\u{c4d}"), ("th", "ยอร\u{e4c}ก"), ("tr", "York"), ("uk", "Йорк"), ("ur", "یورک"), ("vi", "York"), ("zh", "約克")]),
                        unofficial_name_list: ["York"].to_vec(),
                    }
                ),
                (
                    "ZET",
                    Subdivision{
                        name: "Shetland Islands",
                        country_alpha2: Alpha2::GB,
                        code: "ZET",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(60.5296507), longitude: Some(-1.2659408), max_latitude: Some(60.86076139999999), min_latitude: Some(59.50682440000001), max_longitude: Some(-0.7245408999999999), min_longitude: Some(-2.1174404)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CouncilArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Shetland"), ("ar", "جزر شتلاند"), ("az", "Şetland adaları"), ("be", "Шэтландскія астравы"), ("bg", "Шетландски острови"), ("bn", "শেটল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Shetland"), ("ccp", "𑄥𑄬𑄖\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Shetland Islands"), ("cs", "Shetlandy"), ("cy", "Shetland"), ("da", "Shetlandsøerne"), ("de", "Shetland"), ("el", "Σέτλαντ"), ("en", "Shetland"), ("es", "Islas Shetland"), ("et", "Shetlandi saared"), ("eu", "Shetlandak"), ("fa", "شتلند"), ("fi", "Shetlandsaaret"), ("fr", "Shetland"), ("ga", "Inse Shealtainn"), ("gl", "Illas Shetland"), ("gu", "શ\u{ac7}ટલ\u{ac7}ન\u{acd}ડ"), ("he", "איי שטלנד"), ("hr", "Shetlandsko otočje"), ("hu", "Shetland"), ("hy", "Շետլանդյան կղզիներ"), ("id", "Shetland"), ("is", "Hjaltlandseyjar"), ("it", "Isole Shetland"), ("ja", "シェトランド諸島"), ("kk", "Шетланд аралдары"), ("kn", "ಶ\u{cc6}ಟ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "셰틀랜드 제도"), ("ky", "Шетленд аралдары"), ("lt", "Šetlando salos"), ("lv", "Šetlendas salas"), ("mk", "Шетландски Острови"), ("nb", "Shetland"), ("nl", "Shetlandeilanden"), ("no", "Shetland"), ("pl", "Szetlandy"), ("ps", "شټلنډ"), ("pt", "Shetland"), ("ro", "Shetland"), ("ru", "Шетландские острова"), ("sk", "Shetlandy"), ("sr", "Шетландска острва"), ("sr_Latn", "Šetlandska ostrva"), ("sv", "Shetlandsöarna"), ("sw", "Visiwa vya Shetland"), ("ta", "ஷெட\u{bcd}லன\u{bcd}ட\u{bcd}"), ("te", "ష\u{c46}ట\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("tr", "Shetland"), ("uk", "Шетландські острови"), ("ur", "شیٹ لینڈ"), ("uz", "Shetlend orollari"), ("vi", "Shetland"), ("yue", "昔德蘭"), ("yue_Hans", "昔德兰"), ("zh", "设德兰群岛")]),
                        unofficial_name_list: ["Shetland Islands"].to_vec(),
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
#[cfg(feature = "gb")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::GB,
        alpha3: Alpha3::GBR,
        address_format: Some("{{recipient}}\n{{street}}\n{{city}}\n{{region}}\n{{postalcode}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 44,
        currency_code: CurrencyCode::GBP,
        gec: Some(GEC::UK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::GBR),
        iso_long_name: "The United Kingdom of Great Britain and Northern Ireland",
        iso_short_name: "United Kingdom of Great Britain and Northern Ireland",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10, 11].to_vec(),
        national_prefix: "0",
        nationality: Some("British"),
        number: "826",
        postal_code: true,
        postal_code_format: Some("GIR ?0AA|(?:(?:AB|AL|B|BA|BB|BD|BF|BH|BL|BN|BR|BS|BT|BX|CA|CB|CF|CH|CM|CO|CR|CT|CV|CW|DA|DD|DE|DG|DH|DL|DN|DT|DY|E|EC|EH|EN|EX|FK|FY|G|GL|GY|GU|HA|HD|HG|HP|HR|HS|HU|HX|IG|IM|IP|IV|JE|KA|KT|KW|KY|L|LA|LD|LE|LL|LN|LS|LU|M|ME|MK|ML|N|NE|NG|NN|NP|NR|NW|OL|OX|PA|PE|PH|PL|PO|PR|RG|RH|RM|S|SA|SE|SG|SK|SL|SM|SN|SO|SP|SR|SS|ST|SW|SY|TA|TD|TF|TN|TQ|TR|TS|TW|UB|W|WA|WC|WD|WF|WN|WR|WS|WV|YO|ZE)(?:\\d[\\dA-Z]? ?\\d[ABD-HJLN-UW-Z]{2}))|BFPO ?\\d{1,4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::NorthernEurope),
        un_locode: "GB",
        unofficial_name_list: ["United Kingdom", "The United Kingdom", "England", "Großbritannien", "Vereinigtes Königreich", "Royaume-Uni", "Reino Unido", "イギリス", "Verenigd Koninkrijk", "Great Britain (UK)", "UK", "Великобритания", "Velká Británie", "İngiltere", "Великобританія"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "United Kingdom"), ("af", "Verenidge Koninkryk"), ("ak", "United Kingdom"), ("am", "ጕንጔሑፔ"), ("an", "United Kingdom"), ("ar", "المملكة المتحدة"), ("as", "য\u{9c1}ক\u{9cd}তৰ\u{9be}জ\u{9cd}য"), ("ay", "United Kingdom"), ("az", "United Kingdom"), ("ba", "United Kingdom"), ("be", "Злучанае Каралеўства"), ("bg", "Обединено кралство"), ("bi", "United Kingdom"), ("bn", "য\u{9c1}ক\u{9cd}তর\u{9be}জ\u{9cd}য"), ("bn_IN", "য\u{9c1}ক\u{9cd}তর\u{9be}জ\u{9cd}য"), ("br", "Rouantelezh-Unanet"), ("bs", "Velika Britanija"), ("ca", "Regne Unit"), ("ce", "Йоккха Британи"), ("ch", "United Kingdom"), ("cs", "Spojené království"), ("cv", "Йоккха Британи"), ("cy", "Y Deyrnas Unedig"), ("da", "Storbritannien"), ("de", "Vereinigtes Königreich"), ("dv", "ޔ\u{7aa}ނ\u{7a6}އ\u{7a8}ޓ\u{7ac}ޑ\u{7b0} ކ\u{7a8}ނ\u{7b0}ގ\u{7b0}ޑ\u{7a6}މ\u{7b0}"), ("dz", "ཡ\u{f74}་ནའ\u{f72}་ཊ\u{f7a}ཊ་ ཀ\u{f72}ང་ཌ\u{f7c}མ།"), ("ee", "United Kingdom"), ("el", "Ηνωμένο Βασίλειο"), ("en", "United Kingdom"), ("eo", "Britio"), ("es", "Reino Unido"), ("et", "Suurbritannia"), ("eu", "Erresuma Batua"), ("fa", "انگلستان"), ("ff", "Biritaani-Mawndi"), ("fi", "Yhdistynyt kuningaskunta"), ("fo", "United Kingdom"), ("fr", "Royaume-Uni"), ("fy", "Grut-Brittanje"), ("ga", "An Ríocht Aontaithe"), ("gl", "Reino Unido"), ("gn", "United Kingdom"), ("gu", "ય\u{ac1}નાઇટ\u{ac7}ડ કિ\u{a82}ગડમ"), ("gv", "Reeriaght Unnaneysit"), ("ha", "Birtaniya"), ("he", "הממלכה המאוחדת"), ("hi", "य\u{942}नाइट\u{947}ड कि\u{902}गडम"), ("hr", "Ujedinjeno Kraljevstvo"), ("ht", "Wayòm Ini"), ("hu", "Egyesült Királyság"), ("hy", "Մեծ Բրիտանիա"), ("ia", "Regno Unite"), ("id", "Britania Raya"), ("io", "Unionita Rejio"), ("is", "Bretland"), ("it", "Regno Unito"), ("iu", "ᑐᓗᐃᑦ ᓄᓈᑦ"), ("ja", "英国"), ("ka", "გაერთიანებული სამეფო"), ("ki", "United Kingdom"), ("kk", "Ұлыбритания"), ("kl", "United Kingdom"), ("km", "ចក\u{17d2}រភព\u{200b}អង\u{17cb}គ\u{17d2}លេស"), ("kn", "ಯುನೈಟ\u{cc6}ಡ\u{ccd} ಕ\u{cbf}ಂಗ\u{ccd}ಡಮ\u{ccd}"), ("ko", "영국"), ("ku", "Şahitiya Yekbuyî"), ("kv", "Ыджыд Британия"), ("kw", "Ruwvaneth Unys"), ("ky", "Улуу Британия"), ("lo", "ສະຫະລາຊະອານາຈ\u{eb1}ກ"), ("lt", "Jungtinė Karalystė"), ("lv", "Lielbritānija"), ("mi", "Kīngitanga Kotahi"), ("mk", "Велика Британија"), ("ml", "യ\u{d41}ണൈറ\u{d4d}റഡ\u{d4d} കിങ\u{d4d}ഡം"), ("mn", "Нэгдсэн Вант Улс"), ("mr", "य\u{941}नायट\u{947}ड कि\u{902}गडम"), ("ms", "United Kingdom"), ("mt", "Ingilterra"), ("my", "ယ\u{1030}န\u{102d}\u{102f}က\u{103a}တက\u{103a} ကင\u{103a}းဒမ\u{103a}း"), ("na", "Ingerand"), ("nb", "Storbritannia"), ("ne", "स\u{902}य\u{941}क\u{94d}त अधिराज\u{94d}य"), ("nl", "Verenigd Koninkrijk"), ("nn", "Storbritannia"), ("nv", "Tó Táʼ Dinéʼiʼ Bikéyah"), ("oc", "Reialme Unit"), ("or", "ଯ\u{b41}କ\u{b4d}ତ ର\u{b3e}ଜ\u{b4d}ଯ"), ("pa", "ਬਰਤਾਨੀਆ"), ("pi", "United Kingdom"), ("pl", "Wielka Brytania"), ("ps", "برتانیه"), ("pt", "Reino Unido"), ("pt_BR", "Reino Unido"), ("ro", "Regatul Unit"), ("ru", "Соединённое Королевство"), ("rw", "Ubwongereza (UK)"), ("sc", "Rennu Unidu"), ("sd", "United Kingdom"), ("si", "එක\u{dca}සත\u{dca} ර\u{dcf}ජධ\u{dcf}න\u{dd2}ය"), ("sk", "Spojené kráľovstvo"), ("sl", "Združeno kraljestvo"), ("so", "Midowga boqortooyada Britan"), ("sq", "Mbretëria e Bashkuar"), ("sr", "Уједињено Краљевство"), ("sv", "Storbritannien"), ("sw", "United Kingdom"), ("ta", "ஐக\u{bcd}கிய இர\u{bbe}ஜ\u{bcd}ஜியம\u{bcd}"), ("te", "యున\u{c48}ట\u{c46}డ\u{c4d} క\u{c3f}ంగ\u{c4d}\u{200c}డమ\u{c4d}"), ("tg", "Шоҳигарии Муттаҳида"), ("th", "สหราชอาณาจ\u{e31}กร"), ("ti", "ብሪጣንያ"), ("tk", "Birleşen Patyşalyk"), ("tl", "Nagkaisang Kaharian"), ("tr", "Birleşik Krallık"), ("tt", "Берләшкән Падшаһлык"), ("ug", "ئەنگلىيە"), ("uk", "Велика Британія"), ("ur", "برطانیہ"), ("uz", "Birlashgan Qirollik"), ("ve", "United Kingdom"), ("vi", "Vương Quốc Anh Thống Nhất"), ("wa", "Rweyåme Uni"), ("wo", "Ruwaayom bu Bennoo"), ("xh", "United Kingdom"), ("yo", "Ilẹ\u{300}ọba Aṣọ\u{300}kan"), ("zh_CN", "英国"), ("zh_HK", "英國"), ("zh_TW", "英國"), ("zu", "Umbuso Ohlangeneyo")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
