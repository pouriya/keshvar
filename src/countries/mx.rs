// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The United Mexican States

#[cfg(all(feature = "mx", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::MX;
    pub const ALPHA3: Alpha3 = Alpha3::MEX;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 52;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::MXN;
    pub const GEC: Option<GEC> = Some(GEC::MX);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::MEX);
    pub const ISO_SHORT_NAME: &str = "Mexico";
    pub const ISO_LONG_NAME: &str = "The United Mexican States";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "01";
    pub const NATIONALITY: Option<&str> = Some("Mexican");
    pub const NUMBER: &str = "484";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAmerica);
    pub const UN_LOCODE: &str = "MX";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Mexico", "Mexiko", "Mexique", "México", "メキシコ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Mexico"),
        ("af", "Meksiko"),
        ("ak", "Mexico"),
        ("am", "ሣጤሲጥ"),
        ("an", "Mexico"),
        ("ar", "المكسيك"),
        ("as", "মেক\u{9cd}সিকো"),
        ("ay", "Mexico"),
        ("az", "Meksika"),
        ("ba", "Mexico"),
        ("be", "Мексіка"),
        ("bg", "Мексико"),
        ("bi", "Mexico"),
        ("bn", "মেক\u{9cd}সিকো"),
        ("bn_IN", "মেক\u{9cd}সিকো"),
        ("br", "Mec'hiko"),
        ("bs", "Meksiko"),
        ("ca", "Mèxic"),
        ("ce", "Мексика"),
        ("ch", "Mexico"),
        ("cs", "Mexiko"),
        ("cv", "Мексика"),
        ("cy", "Mecsico"),
        ("da", "Mexico"),
        ("de", "Mexiko"),
        ("dv", "މ\u{7ac}ކ\u{7b0}ސ\u{7a8}ކ\u{7af}"),
        ("dz", "མ\u{f7a}ག་ས\u{f72}་ཀ\u{f7c}།"),
        ("ee", "Mexico"),
        ("el", "Μεξικό"),
        ("en", "Mexico"),
        ("eo", "Meksiko"),
        ("es", "México"),
        ("et", "Mehhiko"),
        ("eu", "Mexiko"),
        ("fa", "مکزیک"),
        ("ff", "Mexico"),
        ("fi", "Meksiko"),
        ("fo", "Meksiko"),
        ("fr", "Mexique"),
        ("fy", "Meksiko"),
        ("ga", "Meicsiceo"),
        ("gl", "México"),
        ("gn", "Mexico"),
        ("gu", "મ\u{ac7}ક\u{acd}સિકો"),
        ("gv", "Meksico"),
        ("ha", "Mexico"),
        ("he", "מקסיקו"),
        ("hi", "म\u{947}क\u{94d}सिको"),
        ("hr", "Meksiko"),
        ("ht", "Meksik"),
        ("hu", "Mexikó"),
        ("hy", "Մեքսիկա"),
        ("ia", "Mexico"),
        ("id", "Meksiko"),
        ("io", "Mexikia"),
        ("is", "Mexíkó"),
        ("it", "Messico"),
        ("iu", "ᒦᒃᓰᖂ"),
        ("ja", "メキシコ"),
        ("ka", "მექსიკა"),
        ("ki", "Mexico"),
        ("kk", "Мексика"),
        ("kl", "Mexico"),
        ("km", "ម\u{17c9}\u{17b7}ចស\u{17ca}\u{17b7}ក"),
        ("kn", "ಹೊಂಡುರಾಸ\u{ccd}"),
        ("ko", "멕시코"),
        ("ku", "Mexîko"),
        ("kv", "Мексика"),
        ("kw", "Meksiko"),
        ("ky", "Мексика"),
        ("lo", "ປະເທດເມ\u{eb1}ກຊ\u{eb4}ກ"),
        ("lt", "Meksika"),
        ("lv", "Meksika"),
        ("mi", "Mehiko"),
        ("mk", "Мексико"),
        ("ml", "മെക\u{d4d}സികോ"),
        ("mn", "Мексик"),
        ("mr", "म\u{947}क\u{94d}सिको"),
        ("ms", "Meksiko"),
        ("mt", "Messiku"),
        (
            "my",
            "မက\u{1039}ကဆ\u{102e}က\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Meketiko"),
        ("nb", "Mexico"),
        ("ne", "म\u{947}क\u{94d}सिको"),
        ("nl", "Mexico"),
        ("nn", "Mexico"),
        ("nv", "Naakaii Bikéyah"),
        ("oc", "Mexic"),
        ("or", "ମେକ\u{b4d}ସ\u{b3f}କୋ"),
        ("pa", "ਮ\u{a48}ਕਸੀਕ\u{a4b}"),
        ("pi", "म\u{947}क\u{94d}सिको"),
        ("pl", "Meksyk"),
        ("ps", "Mexico"),
        ("pt", "México"),
        ("pt_BR", "México"),
        ("ro", "Mexic"),
        ("ru", "Мексика"),
        ("rw", "Megizike"),
        ("sc", "Mèssicu"),
        ("sd", "Mexico"),
        ("si", "මෙක\u{dca}ස\u{dd2}කෝව"),
        ("sk", "Mexiko"),
        ("sl", "Mehika"),
        ("so", "Meksiko"),
        ("sq", "Meksikë"),
        ("sr", "Мексико"),
        ("sv", "Mexiko"),
        ("sw", "Mexico"),
        ("ta", "மெக\u{bcd}ஸிகோ"),
        ("te", "మ\u{c47}క\u{c4d}స\u{c3f}క\u{c4b}"),
        ("tg", "Мексика"),
        ("th", "เม\u{e47}กซ\u{e34}โก"),
        ("ti", "ሜክሲኮ"),
        ("tk", "Meksikanyň"),
        ("tl", "Mehiko"),
        ("tr", "Meksika"),
        ("tt", "Мексико"),
        ("ug", "مېكسىكا"),
        ("uk", "Мексика"),
        ("ur", "میکسیکو"),
        ("uz", "Meksika"),
        ("ve", "Mexico"),
        ("vi", "Mê-hi-cô"),
        ("wa", "Mecsike"),
        ("wo", "Meksik"),
        ("xh", "Mexico"),
        ("yo", "Mẹ\u{301}ksíkò"),
        ("zh_CN", "墨西哥"),
        ("zh_HK", "墨西哥"),
        ("zh_TW", "墨西哥"),
        ("zu", "IMekisiko"),
    ];
    #[cfg(all(feature = "mx", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 23.634501;
        pub const LONGITUDE: f64 = -102.552784;
        pub const MAX_LATITUDE: f64 = 32.7186534;
        pub const MAX_LONGITUDE: f64 = -86.5887;
        pub const MIN_LATITUDE: f64 = 14.3895;
        pub const MIN_LONGITUDE: f64 = -118.6523001;
        pub const NORTHEAST_LATITUDE: f64 = 32.7186534;
        pub const NORTHEAST_LONGITUDE: f64 = -86.5887;
        pub const SOUTHWEST_LATITUDE: f64 = 14.3895;
        pub const SOUTHWEST_LONGITUDE: f64 = -118.6523001;
    }
}
#[cfg(all(feature = "mx", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 23.634501,
            longitude: -102.552784,
            max_latitude: 32.7186534,
            max_longitude: -86.5887,
            min_latitude: 14.3895,
            min_longitude: -118.6523001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 32.7186534,
                    longitude: -86.5887,
                },
                southwest: CountryGeoBound {
                    latitude: 14.3895,
                    longitude: -118.6523001,
                },
            },
        }
    }
}

#[cfg(all(feature = "mx", feature = "subdivisions"))]
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
                    "AGU",
                    Subdivision{
                        name: "AGU",
                        country_alpha2: Alpha2::MX,
                        code: "AGU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.8852562), longitude: Some(-102.2915677), max_latitude: Some(21.9580787), min_latitude: Some(21.8200957), max_longitude: Some(-102.2284165), min_longitude: Some(-102.3555606)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية اغواسكالينتس"), ("be", "Штат Агуаскальентэс"), ("bg", "Агуаскалиентес"), ("bn", "আগ\u{9c1}য\u{9bc}\u{9be}সক\u{9be}লিয\u{9bc}েন\u{9cd}তেস"), ("bs", "Aguascalientes"), ("ca", "Estat d’Aguascalientes"), ("ccp", "𑄃𑄉\u{1112a}𑄠𑄌\u{11134}𑄇𑄣\u{11128}𑄠𑄬𑄚\u{11134}𑄑\u{11128}𑄌\u{11134}"), ("ceb", "Estado de Aguascalientes"), ("cs", "Aguascalientes"), ("cy", "Aguascalientes"), ("da", "Aguascalientes"), ("de", "Aguascalientes"), ("el", "Αγουασκαλιέντες"), ("en", "Aguascalientes"), ("es", "Aguascalientes"), ("et", "Aguascalientese osariik"), ("eu", "Aguascalientes"), ("fa", "آگوئاسکالینتس"), ("fi", "Aguascalientes"), ("fr", "Aguascalientes"), ("ga", "Aguascalientes"), ("gl", "Estado de Aguascalientes"), ("gu", "આગવાસ\u{acd}કલ\u{ac7}\u{a82}ટ\u{ac7}સ"), ("he", "אגואסקליינטס"), ("hi", "अग\u{941}आसक\u{948}लिए\u{902}ट\u{947}स"), ("hr", "Aguascalientes"), ("hu", "Aguascalientes"), ("hy", "Ագուասկալիենտես"), ("id", "Aguascalientes"), ("it", "Aguascalientes"), ("ja", "アグアスカリエンテス州"), ("ka", "აგუასკალიენტესის შტატი"), ("kn", "ಅಗುಸ\u{ccd}ಕಲ\u{cc6}ಂಟ\u{cbf}ಸ\u{ccd}"), ("ko", "아과스칼리엔테스 주"), ("lt", "Aguaskaljentesas"), ("lv", "Agvaskaljentesa"), ("mk", "Агваскалиентес"), ("mr", "अग\u{94d}वासकाल\u{94d}य\u{947}\u{902}त\u{947}स"), ("ms", "Aguascalientes"), ("nb", "Aguascalientes"), ("nl", "Aguascalientes"), ("no", "Aguascalientes"), ("pa", "ਆਗਵਾਸਕਾਲੀਐ\u{a02}ਤ\u{a47}ਸ"), ("pl", "Aguascalientes"), ("pt", "Aguascalientes"), ("ro", "Aguascalientes"), ("ru", "Агуаскальентес"), ("si", "අග\u{dd4}ආස\u{dca}කල\u{dd2}එන\u{dca}ටෙස\u{dca}"), ("sk", "Aguascalientes"), ("sr", "Агваскалијентес"), ("sr_Latn", "Agvaskalijentes"), ("sv", "Aguascalientes"), ("sw", "Aguascalientes"), ("ta", "அகுற\u{bcd}ஸ\u{bcd}கேலின\u{bcd}ட\u{bcd}ஸ\u{bcd}"), ("te", "అగువ\u{c3e}స\u{c4d}క\u{c3e}ల\u{c3f}య\u{c46}ంట\u{c46}స\u{c4d}"), ("th", "ร\u{e31}ฐอากว\u{e31}สกาเล\u{e35}ยนเตส"), ("tr", "Aguascalientes"), ("uk", "Аґуаскальєнтес"), ("ur", "آگوسکالینٹس"), ("uz", "Aguaskalentes"), ("vi", "Aguascalientes"), ("yue", "阿瓜斯卡連特斯州"), ("yue_Hans", "阿瓜斯卡连特斯州"), ("zh", "阿瓜斯卡連特斯州")]),
                        unofficial_name_list: ["Aguascalientes"].to_vec(),
                    }
                ),
                (
                    "BCN",
                    Subdivision{
                        name: "BCN",
                        country_alpha2: Alpha2::MX,
                        code: "BCN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.8406338), longitude: Some(-115.2837585), max_latitude: Some(32.7186534), min_latitude: Some(27.9999998), max_longitude: Some(-112.7555545), min_longitude: Some(-118.36443)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Baja California"), ("ar", "ولاية باها كاليفورنيا"), ("az", "Aşağı Kaliforniya"), ("be", "Штат Ніжняя Каліфорнія"), ("bg", "Долна Калифорния"), ("bn", "ব\u{9be}হ\u{9be} ক\u{9be}লিফোর\u{9cd}নিয\u{9bc}\u{9be}"), ("bs", "Donja Kalifornija"), ("ca", "Baixa Califòrnia"), ("ccp", "𑄝𑄎 𑄇\u{11133}𑄠𑄣\u{11128}𑄜\u{1112e}𑄢\u{11134}𑄚\u{11128}𑄠"), ("ceb", "Estado de Baja California"), ("cs", "Baja California"), ("cy", "Baja California"), ("da", "Baja California"), ("de", "Baja California"), ("el", "Μπάχα Καλιφόρνια"), ("en", "Baja California"), ("es", "Baja California"), ("et", "Baja California"), ("eu", "Kalifornia Beherea"), ("fa", "باخا کالیفرنیا"), ("fi", "Baja California"), ("fr", "Basse-Californie"), ("ga", "Baja California"), ("gl", "Baixa California - Baja California"), ("gu", "બાજા ક\u{ac7}લિફોર\u{acd}નિયા"), ("he", "באחה קליפורניה"), ("hi", "बाखा क\u{948}लिफ\u{93c}ोर\u{94d}निया"), ("hr", "Baja California"), ("hu", "Alsó-Kalifornia"), ("hy", "Ստորին Կալիֆոռնիա"), ("id", "Baja California"), ("it", "Bassa California"), ("ja", "バハ・カリフォルニア州"), ("ka", "ქვემო კალიფორნია"), ("kk", "Төменгі Калифорния"), ("kn", "ಬಾಜಾ ಕ\u{ccd}ಯಾಲ\u{cbf}ಫೋರ\u{ccd}ನ\u{cbf}ಯಾ"), ("ko", "바하칼리포르니아 주"), ("lt", "Žemutinė Kalifornija"), ("lv", "Lejaskalifornija"), ("mk", "Долна Калифорнија"), ("ml", "ബ\u{d3e}ഹ\u{d3e} ക\u{d3e}ലിഫോർണിയ"), ("mr", "बाहा कालिफोर\u{94d}निया"), ("ms", "Baja California"), ("nb", "Baja California"), ("nl", "Baja California"), ("no", "Baja California"), ("pa", "ਬਾਖ\u{a3c}ਾ ਕਾਲੀਫ\u{a3c}\u{a4b}ਰਨੀਆ"), ("pl", "Kalifornia Dolna"), ("pt", "Baja California"), ("ro", "Baja California"), ("ru", "Нижняя Калифорния"), ("si", "බජ\u{dcf} කල\u{dd2}ෆෝර\u{dca}ණ\u{dd2}ය\u{dcf}ව"), ("sk", "Baja California"), ("sq", "Baja California"), ("sr", "Доња Калифорнија"), ("sr_Latn", "Donja Kalifornija"), ("sv", "Baja California"), ("sw", "Baja California"), ("ta", "ப\u{bbe}க\u{bbe} கலிபோர\u{bcd}னிய\u{bbe}"), ("te", "బ\u{c3e}జ\u{c3e} క\u{c3e}ల\u{c3f}ఫ\u{c4b}ర\u{c4d}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐบาฮากาล\u{e34}ฟอร\u{e4c}เน\u{e35}ย"), ("tr", "Aşağı Kaliforniya"), ("uk", "Баха-Каліфорнія"), ("ur", "باجا کیلیفورنیا"), ("uz", "Quyi shimoliy Kaliforniya"), ("vi", "Baja California"), ("yue", "下加利福尼亞州"), ("yue_Hans", "下加利福尼亚州"), ("zh", "下加利福尼亞州")]),
                        unofficial_name_list: ["Baja California"].to_vec(),
                    }
                ),
                (
                    "BCS",
                    Subdivision{
                        name: "BCS",
                        country_alpha2: Alpha2::MX,
                        code: "BCS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.0444446), longitude: Some(-111.6660725), max_latitude: Some(28.0000017), min_latitude: Some(22.8722626), max_longitude: Some(-109.4134348), min_longitude: Some(-115.2137241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Baja California Sur"), ("ar", "ولاية باخا كاليفورنيا سور"), ("be", "Штат Паўднёвая Ніжняя Каліфорнія"), ("bg", "Южна Долна Калифорния"), ("bn", "ব\u{9be}হ\u{9be} ক\u{9be}লিফোর\u{9cd}নিয\u{9bc}\u{9be} স\u{9c1}র"), ("ca", "Baixa Califòrnia Sud"), ("ccp", "𑄝𑄎 𑄇\u{11133}𑄠𑄣\u{11128}𑄜\u{1112e}𑄢\u{11134}𑄚\u{11128}𑄠 𑄥𑄢\u{11134}"), ("ceb", "Estado de Baja California Sur"), ("cs", "Baja California Sur"), ("cy", "Baja California Sur"), ("da", "Baja California Sur"), ("de", "Baja California Sur"), ("el", "Μπάχα Καλιφόρνια Σουρ"), ("en", "Baja California Sur"), ("es", "Baja California Sur"), ("et", "Baja California Sur"), ("eu", "Hego Kalifornia Beherea"), ("fa", "باحا کالیفرنیا سور"), ("fi", "Baja California Sur"), ("fr", "Basse-Californie du Sud"), ("ga", "Baja California Sur"), ("gl", "Baixa California Sur - Baja California Sur"), ("gu", "બાજા ક\u{ac7}લિફોર\u{acd}નિયા સ\u{ac1}ર"), ("he", "באחה קליפורניה הדרומית"), ("hi", "बाखा क\u{948}लिफोर\u{94d}निया स\u{941}र"), ("hr", "Baja California Sur"), ("hu", "Déli-Alsó-Kalifornia"), ("hy", "Հարավային Ստորին Կալիֆոռնիա"), ("id", "Baja California Sur"), ("it", "Bassa California del Sud"), ("ja", "バハ・カリフォルニア・スル州"), ("ka", "ქვემო კალიფორნია²"), ("kk", "Төменгі оңтүстік Калифорния"), ("kn", "ಬಾಜಾ ಕ\u{ccd}ಯಾಲ\u{cbf}ಫೋರ\u{ccd}ನ\u{cbf}ಯಾ ಸುರ\u{ccd}"), ("ko", "바하칼리포르니아수르 주"), ("lt", "Žemutinė Pietų Kalifornija"), ("lv", "Dienvidu Lejaskalifornija"), ("mk", "Јужна Долна Калифорнија"), ("mr", "बाहा कालिफोर\u{94d}निया स\u{941}र"), ("ms", "Baja California Sur"), ("nb", "Baja California Sur"), ("nl", "Baja California Sur"), ("no", "Baja California Sur"), ("pl", "Kalifornia Dolna Południowa"), ("pt", "Baja California Sur"), ("ro", "Baja California Sur"), ("ru", "Южная Нижняя Калифорния"), ("si", "බජ\u{dcf} කැල\u{dd2}ෆෝන\u{dd2}ය\u{dcf} සර\u{dca}"), ("sk", "Baja California Sur"), ("sr", "Јужна Доња Калифорнија"), ("sr_Latn", "Južna Donja Kalifornija"), ("sv", "Baja California Sur"), ("sw", "Baja California Sur"), ("ta", "தெற\u{bcd}கு ப\u{bbe}க\u{bbe} கலிபோர\u{bcd}னிய\u{bbe}"), ("te", "బ\u{c3e}జ\u{c3e} క\u{c3e}ల\u{c3f}ఫ\u{c4b}ర\u{c4d}న\u{c3f}య\u{c3e} సుర\u{c4d}"), ("th", "ร\u{e31}ฐบาฮากาล\u{e34}ฟอร\u{e4c}เน\u{e35}ยซ\u{e39}ร\u{e4c}"), ("tr", "Güney Aşağı Kaliforniya"), ("uk", "Баха-Каліфорнія-Сюр"), ("ur", "باجا کیلیفورنیا سر"), ("uz", "Quyi janubiy Kaliforniya"), ("vi", "Baja California Sur"), ("yue", "南下加利福尼亞州"), ("yue_Hans", "南下加利福尼亚州"), ("zh", "南下加利福尼亞州")]),
                        unofficial_name_list: ["Baja California Sur"].to_vec(),
                    }
                ),
                (
                    "CAM",
                    Subdivision{
                        name: "CAM",
                        country_alpha2: Alpha2::MX,
                        code: "CAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.8301251), longitude: Some(-90.5349087), max_latitude: Some(19.8763407), min_latitude: Some(19.7848441), max_longitude: Some(-90.4777941), min_longitude: Some(-90.6223751)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Campeche"), ("ar", "ولاية كامبيتشي"), ("be", "Штат Кампечэ"), ("bg", "Кампече"), ("bn", "ক\u{9be}ম\u{9cd}পেচে"), ("ca", "Estat de Campeche"), ("ccp", "𑄇\u{11133}𑄠𑄟\u{11134}𑄛𑄬𑄌𑄬"), ("cs", "Campeche"), ("cy", "Campeche"), ("da", "Campeche"), ("de", "Campeche"), ("el", "Καμπέτσε"), ("en", "Campeche"), ("es", "Campeche"), ("et", "Campeche osariik"), ("eu", "Campeche"), ("fa", "کامپچه"), ("fi", "Campeche"), ("fr", "Campeche"), ("ga", "Campeche"), ("gl", "Estado de Campeche"), ("gu", "ક\u{ac7}મ\u{acd}પ\u{ac7}ક"), ("he", "קמפצ׳ה"), ("hi", "क\u{948}म\u{94d}प\u{947}च\u{947}"), ("hr", "Campeche"), ("hu", "Campeche"), ("hy", "Կամպեչե"), ("id", "Campeche"), ("it", "Campeche"), ("ja", "カンペチェ州"), ("ka", "კამპეჩეს შტატი"), ("kn", "ಕ\u{ccd}ಯಾಂಪೇಚ\u{cc6}"), ("ko", "캄페체 주"), ("lt", "Kampečė"), ("lv", "Kampeče"), ("mk", "Кампече"), ("mr", "का\u{902}प\u{947}च\u{947}"), ("ms", "Campeche"), ("nb", "Campeche"), ("nl", "Campeche"), ("no", "Campeche"), ("pl", "Campeche"), ("pt", "Campeche"), ("ro", "Campeche"), ("ru", "Кампече"), ("si", "කැම\u{dca}පේචේ"), ("sk", "Campeche"), ("sr", "Држава Кампече"), ("sr_Latn", "Država Kampeče"), ("sv", "Campeche"), ("sw", "Campeche"), ("ta", "க\u{bbe}ம\u{bcd}பெச\u{bcd}சே"), ("te", "క\u{c3e}ంప\u{c40}చ\u{c46}"), ("th", "ร\u{e31}ฐก\u{e31}มเปเช"), ("tr", "Campeche"), ("uk", "Кампече"), ("ur", "کامپیچی"), ("uz", "Kampeche"), ("vi", "Campeche"), ("yue", "坎佩切州"), ("yue_Hans", "坎佩切州"), ("zh", "坎佩切州")]),
                        unofficial_name_list: ["Campeche"].to_vec(),
                    }
                ),
                (
                    "CHH",
                    Subdivision{
                        name: "CHH",
                        country_alpha2: Alpha2::MX,
                        code: "CHH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.6329957), longitude: Some(-106.0691004), max_latitude: Some(28.7729082), min_latitude: Some(28.5586774), max_longitude: Some(-105.9612896), min_longitude: Some(-106.1671059)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chihuahua"), ("ar", "ولاية شيواوا"), ("az", "Çiuaua"), ("be", "Штат Чыўаўа"), ("bg", "Чиуауа"), ("bn", "চিওয\u{9bc}\u{9be}ওয\u{9bc}\u{9be}"), ("bs", "Chihuahua"), ("ca", "Estat de Chihuahua"), ("ccp", "𑄌\u{11128}𑄦\u{11131}𑄦\u{11131}"), ("ceb", "Chihuahua"), ("cs", "Chihuahua"), ("cy", "Chihuahua"), ("da", "Chihuahua"), ("de", "Chihuahua"), ("el", "Τσιουάουα"), ("en", "Chihuahua"), ("es", "Chihuahua"), ("et", "Chihuahua osariik"), ("eu", "Chihuahua"), ("fa", "ایالت چیواوا"), ("fi", "Chihuahua"), ("fr", "Chihuahua"), ("ga", "Chihuahua"), ("gl", "Estado de Chihuahua"), ("gu", "ચિહ\u{ac1}આહ\u{ac1}આ"), ("he", "צ׳יוואווה"), ("hi", "चिह\u{941}आह\u{941}आ"), ("hr", "Chihuahua"), ("hu", "Chihuahua"), ("hy", "Չիուաուա"), ("id", "Chihuahua"), ("it", "Chihuahua"), ("ja", "チワワ州"), ("ka", "ჩიუაუა"), ("kn", "ಚ\u{cbf}ಹುವಾಹುವಾ"), ("ko", "치와와 주"), ("lt", "Čihuahua"), ("lv", "Čivava"), ("mk", "Чивава"), ("mr", "चिवावा"), ("ms", "Chihuahua"), ("nb", "Chihuahua"), ("ne", "चिह\u{941}वाह\u{941}वा"), ("nl", "Chihuahua"), ("no", "Chihuahua"), ("pa", "ਚੀਵਾਵਾ"), ("pl", "Chihuahua"), ("pt", "Chihuahua"), ("ro", "Chihuahua"), ("ru", "Чиуауа"), ("si", "ච\u{dd2}හ\u{dd4}ආහ\u{dd4}ආ"), ("sk", "Chihuahua"), ("sr", "чивава"), ("sr_Latn", "čivava"), ("sv", "Chihuahua"), ("sw", "Chihuahua"), ("ta", "சிஹுஹ\u{bbe}ஹுவ\u{bbe}"), ("te", "చ\u{c3f}హువ\u{c3e}హువ\u{c3e}"), ("th", "ร\u{e31}ฐช\u{e35}วาวา"), ("tr", "Chihuahua"), ("uk", "Чіуауа"), ("ur", "چہواہوا"), ("uz", "Chiuaua"), ("vi", "Chihuahua"), ("yue", "芝華華州"), ("yue_Hans", "芝华华州"), ("zh", "奇瓦瓦州")]),
                        unofficial_name_list: ["Chihuahua"].to_vec(),
                    }
                ),
                (
                    "CHP",
                    Subdivision{
                        name: "CHP",
                        country_alpha2: Alpha2::MX,
                        code: "CHP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.7569318), longitude: Some(-93.12923529999999), max_latitude: Some(17.9852877), min_latitude: Some(14.5320984), max_longitude: Some(-90.3702138), min_longitude: Some(-94.13915589999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chiapas"), ("ar", "تشياباس"), ("bg", "Чиапас"), ("bn", "চিয\u{9bc}\u{9be}প\u{9be}স"), ("bs", "Chiapas"), ("ca", "Chiapas"), ("ccp", "𑄌\u{11128}𑄠𑄛𑄌\u{11134}"), ("ceb", "Estado de Chiapas"), ("cs", "Chiapas"), ("cy", "Chiapas"), ("da", "Chiapas"), ("de", "Chiapas"), ("el", "Τσιάπας"), ("en", "Chiapas"), ("es", "Chiapas"), ("et", "Chiapas"), ("eu", "Chiapas"), ("fa", "چیاپاس"), ("fi", "Chiapas"), ("fr", "Chiapas"), ("ga", "Chiapas"), ("gl", "Chiapas"), ("gu", "ચીઆપાસ"), ("he", "צ׳יאפס"), ("hi", "चियापास"), ("hr", "Chiapas"), ("hu", "Chiapas"), ("hy", "Չիապաս"), ("id", "Chiapas"), ("it", "Chiapas"), ("ja", "チアパス州"), ("ka", "ჩიაპასი"), ("kn", "ಚ\u{cbf}ಯಾಪಾಸ\u{ccd}"), ("ko", "치아파스 주"), ("lt", "Čiapasas"), ("lv", "Čjapasa"), ("mr", "च\u{94d}यापास"), ("ms", "Chiapas"), ("nb", "Chiapas"), ("nl", "Chiapas"), ("no", "Chiapas"), ("pl", "Chiapas"), ("pt", "Chiapas"), ("ro", "Chiapas"), ("ru", "Чьяпас"), ("si", "ච\u{dd2}යප\u{dcf}ස\u{dca}"), ("sk", "Chiapas"), ("sr", "Чијапас"), ("sr_Latn", "Čijapas"), ("sv", "Chiapas"), ("sw", "Chiapas"), ("ta", "சிய\u{bbe}ப\u{bbe}ஸ\u{bcd}"), ("te", "చ\u{c3f}య\u{c3e}పస\u{c4d}"), ("th", "ร\u{e31}ฐเช\u{e35}ยป\u{e31}ส"), ("tr", "Chiapas"), ("uk", "Чіапас"), ("ur", "چیاپاس"), ("uz", "Chyapas"), ("vi", "Chiapas"), ("yue", "恰帕斯州"), ("yue_Hans", "恰帕斯州"), ("zh", "恰帕斯州")]),
                        unofficial_name_list: ["Chiapas"].to_vec(),
                    }
                ),
                (
                    "CMX",
                    Subdivision{
                        name: "CMX",
                        country_alpha2: Alpha2::MX,
                        code: "CMX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.2464696), longitude: Some(-99.1013498), max_latitude: Some(19.5927571), min_latitude: Some(19.0482366), max_longitude: Some(-98.94030269999999), min_longitude: Some(-99.36492419999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Capital,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Meksikostad"), ("am", "ሜክሲኮ ከተማ"), ("ar", "مدينة مكسيكو"), ("az", "Mexiko"), ("be", "Мехіка"), ("bg", "Мексико"), ("bn", "মেক\u{9cd}সিকো সিটি"), ("bs", "Ciudad de México"), ("ca", "Ciutat de Mèxic"), ("ccp", "𑄌\u{1112d}𑄅\u{1112a}𑄓𑄖\u{11134} 𑄓𑄬 𑄟𑄬𑄇\u{11134}𑄥\u{11128}𑄇\u{1112e}"), ("ceb", "Dakbayan sa Mehiko"), ("cs", "Ciudad de México"), ("cy", "Dinas Mexico"), ("da", "Mexico City"), ("de", "Mexiko-Stadt"), ("el", "Πόλη του Μεξικού"), ("en", "Ciudad de Mexico"), ("es", "Ciudad de México"), ("et", "México"), ("eu", "Mexiko Hiria"), ("fa", "مکزیکو سیتی"), ("fi", "México"), ("fr", "Mexico"), ("ga", "Cathair Mheicsiceo"), ("gl", "Cidade de México"), ("gu", "મ\u{ac7}ક\u{acd}સિકો સિટી"), ("ha", "Mexico"), ("ha_NE", "Mexico"), ("he", "מקסיקו סיטי"), ("hi", "म\u{947}क\u{94d}सिको नगर"), ("hr", "Ciudad de México"), ("hu", "Mexikóváros"), ("hy", "Մեխիկո"), ("id", "Ciudad de México"), ("is", "Mexíkóborg"), ("it", "Città del Messico"), ("ja", "メキシコシティ"), ("jv", "Mexico City"), ("ka", "მეხიკო"), ("kk", "Мехико"), ("kn", "ಮ\u{cc6}ಕ\u{ccd}ಸ\u{cbf}ಕೋ ನಗರ"), ("ko", "멕시코시티"), ("ky", "Мехико"), ("lt", "Meksikas"), ("lv", "Mehiko"), ("mk", "Мексико"), ("ml", "മെക\u{d4d}സിക\u{d4d}കോ സിറ\u{d4d}റി"), ("mn", "Мехико"), ("mr", "म\u{947}क\u{94d}सिको सिटी"), ("ms", "Bandar Raya Mexico"), ("my", "မက\u{1039}ကဆ\u{102e}က\u{102d}\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Mexico by"), ("nl", "Mexico-Stad"), ("no", "Mexico by"), ("or", "ମେକ\u{b4d}ସ\u{b3f}କୋ ସହର"), ("pa", "ਮ\u{a48}ਕਸੀਕ\u{a4b} ਸ\u{a3c}ਹਿਰ"), ("pl", "Meksyk"), ("pt", "Cidade do México"), ("ro", "Ciudad de México"), ("ru", "Мехико"), ("si", "මෙක\u{dca}ස\u{dd2}කෝ නගරය"), ("sk", "Mexiko"), ("sl", "Ciudad de México"), ("sq", "Meksiko"), ("sr", "Мексико Сити"), ("sr_Latn", "Meksiko Siti"), ("sv", "Mexico City"), ("sw", "Mexico"), ("ta", "மெக\u{bcd}சிகோ நகரம\u{bcd}"), ("te", "మ\u{c46}క\u{c4d}స\u{c3f}క\u{c4b} స\u{c3f}ట\u{c40}"), ("th", "เม\u{e47}กซ\u{e34}โกซ\u{e34}ต\u{e35}"), ("tk", "Mehiko"), ("tr", "Meksika"), ("uk", "Мехіко"), ("ur", "میکسیکو شہر"), ("uz", "Mexiko shahri"), ("vi", "Thành phố México"), ("yo", "Ìlú Mẹ\u{301}ksíkò"), ("yo_BJ", "Ìlú Mɛ\u{301}ksíkò"), ("yue", "墨西哥城"), ("yue_Hans", "墨西哥城"), ("zh", "墨西哥城")]),
                        unofficial_name_list: ["Ciudad de México"].to_vec(),
                    }
                ),
                (
                    "COA",
                    Subdivision{
                        name: "COA",
                        country_alpha2: Alpha2::MX,
                        code: "COA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.058676), longitude: Some(-101.7068294), max_latitude: Some(29.8800241), min_latitude: Some(24.542684), max_longitude: Some(-99.8431197), min_longitude: Some(-103.9600019)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Coahuila"), ("ar", "ولاية كواهويلا"), ("be", "Штат Кааўіла"), ("bg", "Коауила де Сарагоса"), ("bn", "কোয\u{9bc}\u{9be}উইল\u{9be}"), ("ca", "Coahuila"), ("ccp", "𑄇\u{1112e}𑄦\u{1112d}\u{1112a}𑄣"), ("ceb", "Estado de Coahuila de Zaragoza"), ("cs", "Coahuila"), ("cy", "Coahuila"), ("da", "Coahuila"), ("de", "Coahuila"), ("el", "Κοαουίλα"), ("en", "Coahuila"), ("es", "Coahuila de Zaragoza"), ("et", "Coahuila"), ("eu", "Coahuila de Zaragoza"), ("fa", "کواویلا"), ("fi", "Coahuila"), ("fr", "Coahuila"), ("ga", "Coahuila"), ("gl", "Coahuila de Zaragoza"), ("gu", "કોઆહ\u{ac1}ઈલા"), ("he", "קואווילה"), ("hi", "कोह\u{941}इला"), ("hr", "Coahuila"), ("hu", "Coahuila"), ("hy", "Կոաուիլա"), ("id", "Coahuila"), ("it", "Coahuila"), ("ja", "コアウイラ州"), ("ka", "კოაუილა"), ("kn", "ಕೋಹುಲಾಲಾ"), ("ko", "코아우일라 주"), ("lt", "Koahuila"), ("lv", "Koavila de Saragosa"), ("mr", "कोआविला"), ("ms", "Coahuila"), ("nb", "Coahuila"), ("nl", "Coahuila de Zaragoza"), ("no", "Coahuila"), ("pl", "Coahuila"), ("pt", "Coahuila de Zaragoza"), ("ro", "Coahuila"), ("ru", "Коауила"), ("si", "ක\u{dcf}ඔහ\u{dd4}ය\u{dd2}ල\u{dcf}"), ("sk", "Coahuila"), ("sr", "Коавила"), ("sr_Latn", "Koavila"), ("sv", "Coahuila"), ("sw", "Coahuila"), ("ta", "கோகுய\u{bcd}ல\u{bbe}"), ("te", "క\u{c4b}హ\u{c3f}ల\u{c3e}"), ("th", "ร\u{e31}ฐโกอาว\u{e35}ลา"), ("tr", "Coahuila"), ("uk", "Коауїла"), ("ur", "کواہويلا"), ("uz", "Koauila"), ("vi", "Coahuila"), ("yue", "科阿韋拉州"), ("yue_Hans", "科阿韦拉州"), ("zh", "科阿韋拉州")]),
                        unofficial_name_list: ["Coahuila de Zaragoza"].to_vec(),
                    }
                ),
                (
                    "COL",
                    Subdivision{
                        name: "COL",
                        country_alpha2: Alpha2::MX,
                        code: "COL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.2452342), longitude: Some(-103.7240868), max_latitude: Some(19.2943022), min_latitude: Some(19.1987996), max_longitude: Some(-103.6734973), min_longitude: Some(-103.7838196)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Colima"), ("ar", "ولاية كوليما"), ("be", "Штат Каліма"), ("bg", "Колима"), ("bn", "কোলিম\u{9be}"), ("ca", "Estat de Colima"), ("ccp", "𑄇\u{1112e}𑄣\u{11128}𑄟"), ("ceb", "Estado de Colima"), ("cs", "Colima"), ("cy", "Colima"), ("da", "Colima"), ("de", "Colima"), ("el", "Κολίμα"), ("en", "Colima"), ("es", "Colima"), ("et", "Colima osariik"), ("eu", "Colima"), ("fa", "کولیما"), ("fi", "Colima"), ("fr", "Colima"), ("ga", "Colima"), ("gl", "Estado de Colima"), ("gu", "કોલિમા"), ("he", "קולימה"), ("hi", "कोलीमा"), ("hr", "Colima"), ("hu", "Colima"), ("hy", "Կոլիմա"), ("id", "Colima"), ("it", "Colima"), ("ja", "コリマ州"), ("ka", "კოლიმა"), ("kn", "ಕೊಲ\u{cbf}ಮಾ"), ("ko", "콜리마 주"), ("lt", "Kolima"), ("lv", "Kolima"), ("mr", "कोलिमा"), ("ms", "Colima"), ("nb", "Colima"), ("nl", "Colima"), ("no", "Colima"), ("pl", "Colima"), ("pt", "Colima"), ("ro", "Colima"), ("ru", "Колима"), ("si", "කොල\u{dd2}ම\u{dcf}"), ("sk", "Colima"), ("sr", "Држава Колима"), ("sr_Latn", "Država Kolima"), ("sv", "Colima"), ("sw", "Colima"), ("ta", "கல\u{bc0}ம\u{bbe}"), ("te", "క\u{c4b}ల\u{c3f}మ\u{c3e}"), ("th", "ร\u{e31}ฐโกล\u{e35}มา"), ("tr", "Colima"), ("uk", "Коліма"), ("ur", "کولیما"), ("uz", "Kolima"), ("vi", "Colima"), ("yue", "科利馬州"), ("yue_Hans", "科利马州"), ("zh", "科利馬州")]),
                        unofficial_name_list: ["Colima"].to_vec(),
                    }
                ),
                (
                    "DUR",
                    Subdivision{
                        name: "DUR",
                        country_alpha2: Alpha2::MX,
                        code: "DUR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.0277202), longitude: Some(-104.6531759), max_latitude: Some(24.0945669), min_latitude: Some(23.9633543), max_longitude: Some(-104.5754014), min_longitude: Some(-104.7151846)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Durango"), ("ar", "ولاية دورانجو"), ("be", "Штат Дуранга"), ("bg", "Дуранго"), ("bn", "দ\u{9c1}র\u{9be}ঙ\u{9cd}গো"), ("ca", "Estat de Durango"), ("ccp", "𑄓\u{1112a}𑄢𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Estado de Durango"), ("cs", "Durango"), ("cy", "Durango"), ("da", "Durango"), ("de", "Durango"), ("el", "Ντουράνγκο"), ("en", "Durango"), ("es", "Durango"), ("et", "Durango osariik"), ("eu", "Durango"), ("fa", "دورانگو"), ("fi", "Durango"), ("fr", "Durango"), ("ga", "Durango"), ("gl", "Estado de Durango"), ("gu", "દ\u{ac1}રાન\u{acd}ગો"), ("he", "דורנגו"), ("hi", "द\u{941}र\u{902}गो"), ("hr", "Durango"), ("hu", "Durango"), ("hy", "Դուրանգո"), ("id", "Durango"), ("it", "Durango"), ("ja", "ドゥランゴ州"), ("ka", "დურანგოს შტატი"), ("kn", "ದುರಾಂಗೊ"), ("ko", "두랑고 주"), ("lt", "Durangas"), ("lv", "Durango"), ("mk", "Дуранго"), ("mr", "द\u{941}रा\u{902}गो"), ("ms", "Durango"), ("nb", "Durango"), ("nl", "Durango"), ("no", "Durango"), ("pl", "Durango"), ("pt", "Durango"), ("ro", "Durango"), ("ru", "Дуранго"), ("si", "ඩ\u{dd4}රන\u{dca}ගෝ"), ("sk", "Durango"), ("sr", "Држава Дуранго"), ("sr_Latn", "Država Durango"), ("sv", "Durango"), ("sw", "Durango"), ("ta", "டுரங\u{bcd}கோ"), ("te", "డుర\u{c3e}ంగ\u{c4b}"), ("th", "ร\u{e31}ฐด\u{e39}ร\u{e31}งโก"), ("tr", "Durango"), ("uk", "Дуранго"), ("ur", "دورانگو"), ("uz", "Durango (shtat)"), ("vi", "Durango"), ("yue", "杜蘭戈州"), ("yue_Hans", "杜兰戈州"), ("zh", "杜蘭戈州")]),
                        unofficial_name_list: ["Durango"].to_vec(),
                    }
                ),
                (
                    "GRO",
                    Subdivision{
                        name: "GRO",
                        country_alpha2: Alpha2::MX,
                        code: "GRO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.4391926), longitude: Some(-99.54509739999999), max_latitude: Some(18.8878466), min_latitude: Some(16.3159525), max_longitude: Some(-98.00727630000002), min_longitude: Some(-102.1834566)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية غيريرو"), ("be", "Штат Герэра"), ("bg", "Гереро"), ("bn", "গেরেরো"), ("ca", "Guerrero"), ("ccp", "𑄉\u{1112a}𑄠𑄬𑄢𑄬𑄢\u{1112e}"), ("ceb", "Estado de Guerrero"), ("cs", "Guerrero"), ("cy", "Guerrero"), ("da", "Guerrero"), ("de", "Guerrero"), ("el", "Γκερρέρο"), ("en", "Guerrero"), ("es", "Estado de Guerrero"), ("et", "Guerrero"), ("eu", "Guerrero"), ("fa", "گوئررو"), ("fi", "Guerrero"), ("fr", "Guerrero"), ("ga", "Guerrero"), ("gl", "Estado de Guerrero"), ("gu", "એન\u{acd}ગ\u{acd}લ\u{ac1}ઇશ"), ("he", "גררו"), ("hi", "ग\u{94d}व\u{947}र\u{947}रो"), ("hr", "Guerrero"), ("hu", "Guerrero"), ("hy", "Գեռերո"), ("id", "Guerrero"), ("it", "Guerrero"), ("ja", "ゲレーロ州"), ("ka", "გერეროს შტატი"), ("kn", "ಎನ\u{ccd}ಗ\u{ccd}ಲುಯ\u{cbf}ಷ\u{ccd}"), ("ko", "게레로 주"), ("lt", "Gereras"), ("lv", "Gerrero"), ("mk", "Гереро"), ("mr", "ग\u{947}र\u{947}रो"), ("ms", "Guerrero"), ("nb", "Guerrero"), ("nl", "Guerrero"), ("no", "Guerrero"), ("pl", "Guerrero"), ("pt", "Guerrero"), ("ro", "Guerrero"), ("ru", "Герреро"), ("si", "එන\u{dca}ග\u{dca}ල\u{dd4}ය\u{dd2}ෂ\u{dca}"), ("sk", "Guerrero"), ("sr", "Гереро"), ("sr_Latn", "Gerero"), ("sv", "Guerrero"), ("sw", "Guerrero"), ("ta", "எங\u{bcd}கிலிஷ\u{bcd}"), ("te", "గుయ\u{c46}ర\u{c47}ర\u{c4b}"), ("th", "ร\u{e31}ฐเกร\u{e4c}เรโร"), ("tr", "Guerrero"), ("uk", "Ґерреро"), ("ur", "گیریرو"), ("uz", "Gerrero"), ("vi", "Guerrero"), ("yue", "格雷羅州"), ("yue_Hans", "格雷罗州"), ("zh", "格雷羅州")]),
                        unofficial_name_list: ["Guerrero"].to_vec(),
                    }
                ),
                (
                    "GUA",
                    Subdivision{
                        name: "GUA",
                        country_alpha2: Alpha2::MX,
                        code: "GUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.0190145), longitude: Some(-101.2573586), max_latitude: Some(21.0516362), min_latitude: Some(20.9986079), max_longitude: Some(-101.226862), min_longitude: Some(-101.2801804)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Guanajuato"), ("ar", "ولاية غواناخواتو"), ("be", "Штат Гуанахуата"), ("bg", "Гуанахуато"), ("bn", "গ\u{9c1}য\u{9bc}\u{9be}ন\u{9be}হ\u{9c1}য\u{9bc}\u{9be}তো"), ("ca", "Estat de Guanajuato"), ("ccp", "𑄉\u{1112d}\u{1112a}𑄚𑄎\u{1112a}𑄠𑄑\u{1112e}"), ("ceb", "Estado de Guanajuato"), ("cs", "Guanajuato"), ("cy", "Guanajuato"), ("da", "Guanajuato"), ("de", "Guanajuato"), ("el", "Γκουαναχουάτο"), ("en", "Guanajuato"), ("es", "Guanajuato"), ("et", "Guanajuato osariik"), ("eu", "Guanajuato"), ("fa", "گوآناخوآتو"), ("fi", "Guanajuato"), ("fr", "Guanajuato"), ("ga", "Guanajuato"), ("gl", "Estado de Guanajuato"), ("gu", "ગ\u{ac1}આનાજ\u{ac1}આતો"), ("he", "גואנחואטו"), ("hi", "ग\u{941}आनाउआटो"), ("hr", "Guanajuato"), ("hu", "Guanajuato"), ("hy", "Գուանախուատո"), ("id", "Guanajuato"), ("it", "Guanajuato"), ("ja", "グアナフアト州"), ("ka", "გუანახუატო"), ("kn", "ಗುವಾನಾಜುವಾಟೊ"), ("ko", "과나후아토 주"), ("lt", "Guanachuatas"), ("lv", "Gvanahvato"), ("mk", "Гванахуато"), ("mr", "ग\u{94d}वानाह\u{94d}वातो"), ("ms", "Guanajuato"), ("nb", "Guanajuato"), ("nl", "Guanajuato"), ("no", "Guanajuato"), ("pl", "Guanajuato"), ("pt", "Guanajuato"), ("ro", "Guanajuato"), ("ru", "Гуанахуато"), ("si", "ග\u{dcf}න\u{dcf}ජ\u{dd4}ආටෝ"), ("sk", "Guanajuato"), ("sr", "Држава Гванахуато"), ("sr_Latn", "Država Gvanahuato"), ("sv", "Guanajuato"), ("sw", "Guanajuato"), ("ta", "குணஜூஅக\u{bcd}டோ"), ("te", "గ\u{c4d}వ\u{c3e}న\u{c3e}జువ\u{c3e}ట\u{c4b}"), ("th", "ร\u{e31}ฐกวานาค\u{e31}วโต"), ("tr", "Guanajuato"), ("uk", "Ґуанахуато"), ("ur", "گوانجواتو"), ("uz", "Guanaxuato"), ("vi", "Guanajuato"), ("yue", "瓜納華托州"), ("yue_Hans", "瓜纳华托州"), ("zh", "瓜納華托州")]),
                        unofficial_name_list: ["Guanajuato"].to_vec(),
                    }
                ),
                (
                    "HID",
                    Subdivision{
                        name: "HID",
                        country_alpha2: Alpha2::MX,
                        code: "HID",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.0910963), longitude: Some(-98.7623874), max_latitude: Some(21.3985208), min_latitude: Some(19.5977581), max_longitude: Some(-97.9849289), min_longitude: Some(-99.85954129999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية هيدالغو"), ("be", "Штат Ідальга"), ("bg", "Идалго"), ("bn", "ইদ\u{9be}ল\u{9cd}গো"), ("bs", "Hidalgo"), ("ca", "Hidalgo"), ("ccp", "𑄦\u{11128}𑄓𑄣\u{11134}𑄉\u{1112e}"), ("ceb", "Estado de Hidalgo"), ("cs", "Hidalgo"), ("cy", "Hidalgo"), ("da", "Hidalgo"), ("de", "Hidalgo"), ("el", "Ιδάλγο"), ("en", "Hidalgo"), ("es", "Estado de Hidalgo"), ("et", "Hidalgo osariik"), ("eu", "Hidalgo"), ("fa", "ایالت ایدالگو"), ("fi", "Hidalgo"), ("fr", "Hidalgo"), ("ga", "Hidalgo"), ("gl", "Estado de Hidalgo"), ("gu", "હિદાલ\u{acd}ગો"), ("he", "אידלגו"), ("hi", "हिदाल\u{94d}गो"), ("hr", "Hidalgo"), ("hu", "Hidalgo"), ("hy", "Իդալգո"), ("id", "Hidalgo"), ("it", "Hidalgo"), ("ja", "イダルゴ州"), ("ka", "იდალგოს შტატი"), ("kn", "ಹ\u{cbf}ಡಾಲ\u{ccd}ಗೊ"), ("ko", "이달고 주"), ("lt", "Idalgas"), ("lv", "Idalgo"), ("mk", "Идалго"), ("mr", "इदाल\u{94d}गो"), ("ms", "Hidalgo"), ("nb", "Hidalgo"), ("nl", "Hidalgo"), ("no", "Hidalgo"), ("pl", "Hidalgo"), ("pt", "Hidalgo"), ("ro", "Hidalgo"), ("ru", "Идальго"), ("si", "හ\u{dd2}දල\u{dca}ගෝ"), ("sk", "Hidalgo"), ("sr", "Држава Идалго"), ("sr_Latn", "Država Idalgo"), ("sv", "Hidalgo"), ("sw", "Hidalgo"), ("ta", "ஹிட\u{bbe}ல\u{bcd}கோ"), ("te", "హ\u{c3f}డ\u{c3e}ల\u{c4d}గ\u{c4b}"), ("th", "ร\u{e31}ฐอ\u{e35}ด\u{e31}ลโก"), ("tr", "Hidalgo"), ("uk", "Ідальго"), ("ur", "ہیدالگو (ریاست)"), ("uz", "Idalgo"), ("vi", "Hidalgo"), ("yue", "伊達爾戈州"), ("yue_Hans", "伊达尔戈州"), ("zh", "伊達爾戈州")]),
                        unofficial_name_list: ["Hidalgo"].to_vec(),
                    }
                ),
                (
                    "JAL",
                    Subdivision{
                        name: "JAL",
                        country_alpha2: Alpha2::MX,
                        code: "JAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.6595382), longitude: Some(-103.3494376), max_latitude: Some(22.7502459), min_latitude: Some(18.9258718), max_longitude: Some(-101.5105417), min_longitude: Some(-105.6947969)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Jalisco"), ("ar", "ولاية خاليسكو"), ("be", "штат Халіска"), ("bg", "Халиско"), ("bn", "হ\u{9be}লিস\u{9cd}কো"), ("ca", "Jalisco"), ("ccp", "𑄎𑄣\u{11128}𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Estado de Jalisco"), ("cs", "Jalisco"), ("cy", "Jalisco"), ("da", "Jalisco"), ("de", "Jalisco"), ("el", "Χαλίσκο"), ("en", "Jalisco"), ("es", "Jalisco"), ("et", "Jalisco"), ("eu", "Jalisco"), ("fa", "خالیسکو"), ("fi", "Jalisco"), ("fr", "Jalisco"), ("ga", "Jalisco"), ("gl", "Jalisco"), ("gu", "જ\u{ac7}લિસ\u{acd}કો"), ("he", "חליסקו"), ("hi", "जलिस\u{94d}को"), ("hr", "Jalisco"), ("hu", "Jalisco"), ("hy", "Խալիսկո"), ("id", "Jalisco"), ("it", "Jalisco"), ("ja", "ハリスコ州"), ("ka", "ხალისკო"), ("kn", "ಜಲ\u{cbf}ಸ\u{ccd}ಕೊ"), ("ko", "할리스코 주"), ("lt", "Chaliskas"), ("lv", "Halisko"), ("mk", "Халиско"), ("mr", "हालिस\u{94d}को"), ("ms", "Jalisco"), ("nb", "Jalisco"), ("nl", "Jalisco"), ("no", "Jalisco"), ("pl", "Jalisco"), ("pt", "Jalisco"), ("ro", "Jalisco"), ("ru", "Халиско"), ("si", "ජල\u{dd2}ස\u{dca}කෝ"), ("sk", "Jalisco"), ("sl", "Jalisco"), ("sr", "Халиско"), ("sr_Latn", "Halisko"), ("sv", "Jalisco"), ("sw", "Jalisco"), ("ta", "ஜ\u{bbe}லிஸ\u{bcd}கோ"), ("te", "జ\u{c3e}ల\u{c3f}స\u{c4d}క\u{c4b}"), ("th", "ร\u{e31}ฐฮาล\u{e34}สโก"), ("tr", "Jalisco"), ("uk", "Халіско"), ("ur", "جلیسکو"), ("uz", "Xalisko"), ("vi", "Jalisco"), ("yue", "哈利斯戈州"), ("yue_Hans", "哈利斯戈州"), ("zh", "哈利斯科州")]),
                        unofficial_name_list: ["Jalisco"].to_vec(),
                    }
                ),
                (
                    "MEX",
                    Subdivision{
                        name: "MEX",
                        country_alpha2: Alpha2::MX,
                        code: "MEX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.4326077), longitude: Some(-99.133208), max_latitude: Some(19.5919189), min_latitude: Some(19.0482787), max_longitude: Some(-98.9401855), min_longitude: Some(-99.3641835)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مكسيكو"), ("be", "Штат Мехіка"), ("bg", "Мексико²"), ("bn", "মেক\u{9cd}সিকো র\u{9be}জ\u{9cd}য"), ("ca", "Estat de Mèxic"), ("ccp", "𑄟𑄬𑄇\u{11134}𑄥\u{11128}𑄇\u{1112e} 𑄌\u{11133}𑄑𑄬𑄖\u{11134}"), ("ceb", "Estado de México (estado)"), ("cs", "México"), ("cy", "Talaith Mexico"), ("da", "Mexico"), ("de", "México"), ("el", "Πολιτεία του Μεξικού"), ("en", "Mexico State"), ("es", "Estado de México"), ("et", "México osariik"), ("eu", "Mexikoko estatua"), ("fa", "ایالت\u{200c} مکزیک"), ("fi", "Estado de México"), ("fr", "État de Mexico"), ("ga", "Meicsiceo"), ("gl", "Estado de México"), ("gu", "મ\u{ac7}ક\u{acd}સિકો"), ("he", "מקסיקו"), ("hi", "म\u{947}क\u{94d}सिको"), ("hr", "ples"), ("hu", "México"), ("hy", "Մեխիկո²"), ("id", "Meksiko"), ("it", "Messico"), ("ja", "メヒコ州"), ("ka", "მეხიკოს შტატი"), ("kn", "ಮ\u{cc6}ಕ\u{ccd}ಸ\u{cbf}ಕೊ"), ("ko", "멕시코 주"), ("lt", "Meksikas²"), ("lv", "Mehiko²"), ("mr", "म\u{947}हिको"), ("ms", "Mexico"), ("nb", "México"), ("nl", "Mexico"), ("no", "México"), ("pl", "Meksyk²"), ("pt", "México"), ("ro", "México"), ("ru", "Мехико²"), ("si", "මෙක\u{dca}ස\u{dd2}කෝ"), ("sk", "Mexiko²"), ("sr", "Мексико"), ("sr_Latn", "Meksiko"), ("sv", "Mexiko"), ("sw", "Mexico²"), ("ta", "மெக\u{bcd}சிக\u{bcd}கோ ம\u{bbe}நிலம\u{bcd}"), ("te", "మ\u{c46}క\u{c4d}స\u{c3f}క\u{c4b}"), ("th", "ร\u{e31}ฐเมฮ\u{e35}โก"), ("tr", "Meksika²"), ("uk", "Мехіко²"), ("ur", "ریاست میکسیکو"), ("uz", "Mexiko"), ("vi", "México"), ("yue", "墨西哥州"), ("yue_Hans", "墨西哥州"), ("zh", "墨西哥州")]),
                        unofficial_name_list: ["México"].to_vec(),
                    }
                ),
                (
                    "MIC",
                    Subdivision{
                        name: "MIC",
                        country_alpha2: Alpha2::MX,
                        code: "MIC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.5665192), longitude: Some(-101.7068294), max_latitude: Some(20.3942215), min_latitude: Some(17.9149077), max_longitude: Some(-100.0630329), min_longitude: Some(-103.7381271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية ميتشواكان"), ("be", "штат Мічаакан"), ("bg", "Мичоакан"), ("bn", "মিচোয\u{9bc}\u{9be}ক\u{9be}ন"), ("ca", "Michoacán"), ("ccp", "𑄟\u{11128}𑄇\u{1112e}𑄥𑄚\u{11134}"), ("ceb", "Estado de Michoacán de Ocampo"), ("cs", "Michoacán"), ("cy", "Michoacán"), ("da", "Michoacán"), ("de", "Michoacán"), ("el", "Μιτσοακάν"), ("en", "Michoacán"), ("es", "Michoacán"), ("et", "Michoacán"), ("eu", "Michoacán"), ("fa", "میچوآکان"), ("fi", "Michoacán"), ("fr", "Michoacán"), ("ga", "Michoacán"), ("gl", "Michoacán"), ("gu", "મિચોકાન"), ("he", "מיצ׳ואקאן"), ("hi", "मिचौआक\u{948}न"), ("hr", "Michoacán"), ("hu", "Michoacán"), ("hy", "Միչոական"), ("id", "Michoacán"), ("it", "Michoacán"), ("ja", "ミチョアカン州"), ("ka", "მიჩოაკანის შტატი"), ("kn", "ಮೈಕೋವಕಾನ\u{ccd}"), ("ko", "미초아칸 주"), ("lt", "Mičoakanas"), ("lv", "Mičoakana de Okampo"), ("mr", "मिचोआकान"), ("ms", "Michoacán"), ("nb", "Michoacán"), ("nl", "Michoacán de Ocampo"), ("no", "Michoacán"), ("pl", "Michoacán"), ("pt", "Michoacán"), ("ro", "Michoacán"), ("ru", "Мичоакан"), ("si", "ම\u{dd2}චොආකෑන\u{dca}"), ("sk", "Michoacán"), ("sr", "Мичоакан"), ("sr_Latn", "Mičoakan"), ("sv", "Michoacán de Ocampo"), ("sw", "Michoacán"), ("ta", "மிசோ கண\u{bcd}"), ("te", "మ\u{c3f}చ\u{c4b}క\u{c3e}న\u{c4d}"), ("th", "ร\u{e31}ฐม\u{e34}โชอาก\u{e31}ง"), ("tr", "Michoacán"), ("uk", "Мічоакан"), ("ur", "میشواکان"), ("uz", "Michoakan"), ("vi", "Michoacán"), ("yue", "米卻肯州"), ("yue_Hans", "米却肯州"), ("zh", "米却肯州")]),
                        unofficial_name_list: ["Michoacán de Ocampo"].to_vec(),
                    }
                ),
                (
                    "MOR",
                    Subdivision{
                        name: "MOR",
                        country_alpha2: Alpha2::MX,
                        code: "MOR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.6813049), longitude: Some(-99.1013498), max_latitude: Some(19.1317018), min_latitude: Some(18.332373), max_longitude: Some(-98.63294660000001), min_longitude: Some(-99.4944141)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Morelos"), ("ar", "ولاية موريلوس"), ("be", "Штат Марэлас"), ("bg", "Морелос"), ("bn", "মোরেলোস"), ("ca", "Morelos"), ("ccp", "𑄟\u{1112e}𑄢\u{11134}𑄣𑄬𑄌\u{11134}"), ("ceb", "Estado de Morelos"), ("cs", "Morelos"), ("cy", "Morelos"), ("da", "Morelos"), ("de", "Morelos"), ("el", "Μορέλος"), ("en", "Morelos"), ("es", "Morelos"), ("et", "Morelos"), ("eu", "Morelos"), ("fa", "ایالت مورلوس"), ("fi", "Morelos"), ("fr", "Morelos"), ("ga", "Morelos"), ("gl", "Morelos"), ("he", "מורלוס"), ("hr", "Morelos"), ("hu", "Morelos"), ("hy", "Մորելոս"), ("id", "Morelos"), ("it", "Morelos"), ("ja", "モレロス州"), ("ka", "მორელოსი"), ("ko", "모렐로스 주"), ("lt", "Morelosas"), ("lv", "Morelosa"), ("mr", "मोर\u{947}लोस"), ("ms", "Morelos"), ("nb", "Morelos"), ("nl", "Morelos"), ("no", "Morelos"), ("pa", "ਮ\u{a4b}ਰ\u{a47}ਲ\u{a4b}ਸ"), ("pl", "Morelos"), ("pt", "Morelos"), ("ro", "Morelos"), ("ru", "Морелос"), ("sk", "Morelos"), ("sl", "Morelos"), ("sr", "Морелос"), ("sr_Latn", "Morelos"), ("sv", "Morelos"), ("sw", "Morelos"), ("th", "ร\u{e31}ฐโมเรโลส"), ("tr", "Morelos"), ("uk", "Морелос"), ("ur", "موریلوس"), ("uz", "Morelos"), ("vi", "Morelos"), ("yue", "莫雷洛斯州"), ("yue_Hans", "莫雷洛斯州"), ("zh", "莫雷洛斯州")]),
                        unofficial_name_list: ["Morelos"].to_vec(),
                    }
                ),
                (
                    "NAY",
                    Subdivision{
                        name: "NAY",
                        country_alpha2: Alpha2::MX,
                        code: "NAY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.7513844), longitude: Some(-104.8454619), max_latitude: Some(23.0845034), min_latitude: Some(20.6032209), max_longitude: Some(-103.7208954), min_longitude: Some(-106.6877266)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nayarit"), ("ar", "ولاية ناياريت"), ("be", "Штат Наярыт"), ("bg", "Наярит"), ("bn", "ন\u{9be}য\u{9bc}\u{9be}রিত"), ("ca", "Nayarit"), ("ccp", "𑄚𑄬𑄠𑄢\u{11128}𑄖\u{11134}"), ("ceb", "Estado de Nayarit"), ("cs", "Nayarit"), ("cy", "Nayarit"), ("da", "Nayarit"), ("de", "Nayarit"), ("el", "Ναγιαρίτ"), ("en", "Nayarit"), ("es", "Nayarit"), ("et", "Nayarit"), ("eu", "Nayarit"), ("fa", "نایاریت"), ("fi", "Nayarit"), ("fr", "Nayarit"), ("ga", "Nayarit"), ("gl", "Nayarit"), ("gu", "નાયારિત"), ("he", "נאיאריט"), ("hi", "नयारित"), ("hr", "Nayarit"), ("hu", "Nayarit"), ("hy", "Նայարիտ"), ("id", "Nayarit"), ("it", "Nayarit"), ("ja", "ナヤリット州"), ("ka", "ნაიარიტის შტატი"), ("kn", "ನಾಯರ\u{cbf}ತ\u{ccd}"), ("ko", "나야리트 주"), ("lt", "Najaritas"), ("lv", "Najarita"), ("mr", "नायारित"), ("ms", "Nayarit"), ("nb", "Nayarit"), ("nl", "Nayarit"), ("no", "Nayarit"), ("pa", "ਨਾਈਆਰੀਤ"), ("pl", "Nayarit"), ("pt", "Nayarit"), ("ro", "Nayarit"), ("ru", "Наярит"), ("si", "නයර\u{dd2}ට\u{dca}"), ("sk", "Nayarit"), ("sr", "Најарит"), ("sr_Latn", "Najarit"), ("sv", "Nayarit"), ("sw", "Nayarit"), ("ta", "நயரிட\u{bcd}"), ("te", "నయ\u{c3e}ర\u{c3f}ట\u{c4d}"), ("th", "ร\u{e31}ฐนายาร\u{e34}ต"), ("tr", "Nayarit"), ("uk", "Наяріт"), ("ur", "نایاریت"), ("uz", "Nayarit"), ("vi", "Nayarit"), ("yue", "納亞里特州"), ("yue_Hans", "纳亚里特州"), ("zh", "納亞里特州")]),
                        unofficial_name_list: ["Nayarit"].to_vec(),
                    }
                ),
                (
                    "NLE",
                    Subdivision{
                        name: "NLE",
                        country_alpha2: Alpha2::MX,
                        code: "NLE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.592172), longitude: Some(-99.99619469999999), max_latitude: Some(27.7991372), min_latitude: Some(23.162683), max_longitude: Some(-98.4215759), min_longitude: Some(-101.2067627)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية نويفو ليون"), ("be", "Штат Нуэва-Леон"), ("bg", "Нуево Леон"), ("bn", "ন\u{9c1}য\u{9bc}েভো লেওন"), ("ca", "Nuevo León"), ("ccp", "𑄚\u{11131}𑄠𑄬𑄞\u{1112e} 𑄣\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Estado de Nuevo León"), ("cs", "Nuevo León"), ("cy", "Nuevo León"), ("da", "Nuevo León"), ("de", "Nuevo León"), ("el", "Νουέβο Λεόν"), ("en", "Nuevo León"), ("es", "Nuevo León"), ("et", "Nuevo León"), ("eu", "Nuevo Leon"), ("fa", "نوئوو لئون"), ("fi", "Nuevo León"), ("fr", "Nuevo León"), ("ga", "Nuevo León"), ("gl", "Nuevo León"), ("gu", "ન\u{ac1}એવો લિઓન"), ("he", "נואבו לאון"), ("hi", "न\u{941}एवो लिओन"), ("hr", "Nuevo León"), ("hu", "Új-León"), ("hy", "Նուևո Լեոն"), ("id", "León Baru"), ("it", "Nuevo León"), ("ja", "ヌエボ・レオン州"), ("ka", "ნუევო-ლეონი"), ("kn", "ನ\u{ccd}ಯ\u{cc2}ವೊ ಲ\u{cbf}ಯೋನ\u{ccd}"), ("ko", "누에보레온 주"), ("lt", "Nuevo Leonas"), ("lv", "Nuevoleona"), ("mr", "न\u{941}एव\u{94d}हो ल\u{947}ओन"), ("ms", "Nuevo León"), ("nb", "Nuevo León"), ("nl", "Nuevo León"), ("no", "Nuevo León"), ("pl", "Nuevo León"), ("pt", "Nuevo León"), ("ro", "Nuevo León"), ("ru", "Нуэво-Леон"), ("si", "න\u{dd4}එවෝ ල\u{dd2}යොන\u{dca}"), ("sk", "Nuevo León"), ("sr", "Нови Леон"), ("sr_Latn", "Novi Leon"), ("sv", "Nuevo León"), ("sw", "Nuevo León"), ("ta", "நுஏவோ லியோன\u{bcd}"), ("te", "న\u{c4d}యూవ\u{c4b} ల\u{c3f}య\u{c4b}న\u{c4d}"), ("th", "ร\u{e31}ฐนวยโวเลออง"), ("tr", "Nuevo León"), ("uk", "Нуево-Леон"), ("ur", "نیوو لیون"), ("uz", "Nuevoleon"), ("vi", "Nuevo León"), ("yue", "新萊昂州"), ("yue_Hans", "新莱昂州"), ("zh", "新萊昂州")]),
                        unofficial_name_list: ["Nuevo León"].to_vec(),
                    }
                ),
                (
                    "OAX",
                    Subdivision{
                        name: "OAX",
                        country_alpha2: Alpha2::MX,
                        code: "OAX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.0594169), longitude: Some(-96.7216219), max_latitude: Some(17.1332787), min_latitude: Some(17.0222818), max_longitude: Some(-96.6762766), min_longitude: Some(-96.78067659999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oaxaca"), ("am", "ወሓካ"), ("ar", "ولاية واهاكا"), ("az", "Oaxaka"), ("be", "Штат Аахака"), ("bg", "Оахака"), ("bn", "ওআজ\u{9be}চ\u{9be}"), ("ca", "Estat d’Oaxaca"), ("ccp", "𑄃\u{1112e}𑄠𑄇\u{11134}𑄥𑄇"), ("ceb", "Estado de Oaxaca"), ("cs", "Oaxaca"), ("cy", "Oaxaca"), ("da", "Oaxaca"), ("de", "Oaxaca"), ("el", "Οαχάκα"), ("en", "Oaxaca"), ("es", "Oaxaca"), ("et", "Oaxaca osariik"), ("eu", "Oaxaca"), ("fa", "اوآخاکا"), ("fi", "Oaxaca"), ("fr", "Oaxaca"), ("ga", "Oaxaca"), ("gl", "Oaxaca"), ("gu", "ઓક\u{acd}સાકા"), ("he", "אואחאקה"), ("hi", "वाहाका"), ("hr", "Oaxaca"), ("hu", "Oaxaca"), ("hy", "Օահակա"), ("id", "Oaxaca"), ("it", "Oaxaca"), ("ja", "オアハカ州"), ("ka", "ოახაკა"), ("kn", "ಓಕ\u{ccd}ಸಾಕ"), ("ko", "오악사카 주"), ("lt", "Oachaka"), ("lv", "Oahaka"), ("ml", "വഹ\u{d3e}ക\u{d4d}ക"), ("mn", "Оахака"), ("mr", "वाशाका"), ("ms", "Oaxaca"), ("my", "အ\u{102d}\u{102f}အေဂျေစ\u{102e}အ"), ("nb", "Oaxaca"), ("nl", "Oaxaca"), ("no", "Oaxaca"), ("pa", "ਵਾਹਾਕਾ"), ("pl", "Oaxaca"), ("pt", "Oaxaca"), ("ro", "Oaxaca"), ("ru", "Оахака"), ("si", "ඔඇක\u{dca}සක\u{dcf}"), ("sk", "Oaxaca"), ("sl", "Oaxaca"), ("sq", "Oaxaca"), ("sr", "Оахака"), ("sr_Latn", "Oahaka"), ("sv", "Oaxaca"), ("sw", "Oaxaca"), ("ta", "வஃக\u{bbe}க\u{bcd}க\u{bbe}"), ("te", "ఓవక\u{c4d}స\u{c3e}క\u{c3e}"), ("th", "ร\u{e31}ฐวาฮากา"), ("tr", "Oaxaca"), ("uk", "Оахака"), ("ur", "اوکساکا"), ("uz", "Oaksaka"), ("vi", "Oaxaca"), ("yue", "瓦哈卡"), ("yue_Hans", "瓦哈卡"), ("zh", "瓦哈卡州")]),
                        unofficial_name_list: ["Oaxaca"].to_vec(),
                    }
                ),
                (
                    "PUE",
                    Subdivision{
                        name: "PUE",
                        country_alpha2: Alpha2::MX,
                        code: "PUE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.0412967), longitude: Some(-98.20619959999999), max_latitude: Some(19.1381977), min_latitude: Some(18.9443809), max_longitude: Some(-98.1035011), min_longitude: Some(-98.2824308)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية بويبلا"), ("be", "Штат Пуэбла"), ("bg", "Пуебла"), ("bn", "প\u{9c1}য\u{9bc}েব\u{9cd}ল\u{9be}"), ("ca", "Estat de Puebla"), ("ccp", "𑄛\u{1112a}𑄠𑄬𑄛\u{11134}𑄣"), ("ceb", "Estado de Puebla"), ("cs", "Puebla"), ("cy", "Puebla"), ("da", "Puebla"), ("de", "Puebla"), ("el", "Πουέμπλα"), ("en", "Puebla"), ("es", "Puebla"), ("et", "Puebla osariik"), ("eu", "Puebla"), ("fa", "پوئبلا"), ("fi", "Puebla"), ("fr", "Puebla"), ("ga", "Puebla"), ("gl", "Estado de Puebla"), ("gu", "પ\u{ac1}એબ\u{acd}લા"), ("he", "פואבלה"), ("hi", "प\u{94d}य\u{942}एबला"), ("hr", "Puebla"), ("hu", "Puebla"), ("hy", "Պուեբլա"), ("id", "Puebla"), ("it", "Puebla"), ("ja", "プエブラ州"), ("ka", "პუებლა"), ("kn", "ಪ\u{ccd}ಯುಬ\u{ccd}ಲಾ"), ("ko", "푸에블라 주"), ("lt", "Puebla"), ("lv", "Puebla"), ("mr", "प\u{947}ब\u{94d}ला"), ("ms", "Puebla"), ("nb", "Puebla"), ("nl", "Puebla"), ("no", "Puebla"), ("pa", "ਪ\u{a41}ਐਬਲਾ"), ("pl", "Puebla"), ("pt", "Puebla"), ("ro", "Puebla"), ("ru", "Пуэбла"), ("si", "ප\u{dd4}එබ\u{dca}ල\u{dcf}"), ("sk", "Puebla"), ("sl", "Puebla"), ("sr", "Држава Пуебла"), ("sr_Latn", "Država Puebla"), ("sv", "Puebla"), ("sw", "Puebla"), ("ta", "புவெப\u{bcd}ல\u{bbe}"), ("te", "పుయ\u{c46}బ\u{c4d}ల\u{c3e}"), ("th", "ร\u{e31}ฐปวยบลา"), ("tr", "Puebla"), ("uk", "Пуебла"), ("ur", "پوئبلا"), ("uz", "Puebla"), ("vi", "Puebla"), ("yue", "普埃布拉州"), ("yue_Hans", "普埃布拉州"), ("zh", "普埃布拉州")]),
                        unofficial_name_list: ["Puebla"].to_vec(),
                    }
                ),
                (
                    "QUE",
                    Subdivision{
                        name: "QUE",
                        country_alpha2: Alpha2::MX,
                        code: "QUE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.5887932), longitude: Some(-100.3898881), max_latitude: Some(20.6854032), min_latitude: Some(20.5434421), max_longitude: Some(-100.3285436), min_longitude: Some(-100.482931)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Querétaro"), ("ar", "ولاية كويريتارو"), ("be", "Штат Керэтара"), ("bg", "Керетаро"), ("bn", "কেরেত\u{9be}রো"), ("ca", "Querétaro"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄢𑄬𑄑𑄢\u{1112e}"), ("ceb", "Estado de Querétaro"), ("cs", "Querétaro"), ("cy", "Querétaro"), ("da", "Querétaro"), ("de", "Querétaro"), ("el", "Κερέταρο"), ("en", "Querétaro"), ("es", "Querétaro"), ("et", "Querétaro"), ("eu", "Querétaro"), ("fa", "کرتارو"), ("fi", "Querétaro"), ("fr", "Querétaro"), ("ga", "Querétaro"), ("gl", "Querétaro"), ("gu", "ક\u{acd}વ\u{ac7}ર\u{ac7}ટ\u{ac7}રો"), ("he", "קרטרו"), ("hi", "क\u{94d}व\u{947}र\u{947}तारो"), ("hr", "Querétaro"), ("hu", "Querétaro"), ("hy", "Կերետարո"), ("id", "Querétaro"), ("it", "Querétaro"), ("ja", "ケレタロ州"), ("ka", "კერეტაროს შტატი"), ("kn", "ಕ\u{ccd}ವ\u{cc6}ರ\u{cc6}ಟೊ"), ("ko", "케레타로 주"), ("lt", "Keretaras"), ("lv", "Keretaro"), ("mr", "क\u{947}र\u{947}तारो"), ("ms", "Querétaro"), ("nb", "Querétaro"), ("nl", "Querétaro de Arteaga"), ("no", "Querétaro"), ("pl", "Querétaro"), ("pt", "Querétaro"), ("ro", "Querétaro"), ("ru", "Керетаро"), ("si", "ක\u{dca}වෙරෙටරෝ"), ("sk", "Querétaro"), ("sr", "Керетаро"), ("sr_Latn", "Keretaro"), ("sv", "Querétaro Arteaga"), ("sw", "Querétaro"), ("ta", "யூரேட\u{bcd}டரோ"), ("te", "క\u{c4d}వ\u{c46}ర\u{c46}ట\u{c3e}ర\u{c4b}"), ("th", "ร\u{e31}ฐเกเรตาโร"), ("tr", "Querétaro"), ("uk", "Керетаро"), ("ur", "کوارتارو"), ("uz", "Keretaro (Meksika)"), ("vi", "Querétaro"), ("yue", "克雷塔羅州"), ("yue_Hans", "克雷塔罗州"), ("zh", "克雷塔羅州")]),
                        unofficial_name_list: ["Querétaro"].to_vec(),
                    }
                ),
                (
                    "ROO",
                    Subdivision{
                        name: "ROO",
                        country_alpha2: Alpha2::MX,
                        code: "ROO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.1817393), longitude: Some(-88.4791376), max_latitude: Some(21.587621), min_latitude: Some(17.8939856), max_longitude: Some(-86.7105607), min_longitude: Some(-89.29656179999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية كينتانا رو"), ("be", "Штат Кінтана-Роа"), ("bg", "Кинтана Ро"), ("bn", "কিন\u{9cd}ত\u{9be}ন\u{9be} রো"), ("ca", "Quintana Roo"), ("ccp", "𑄇\u{1112d}\u{1112a}𑄃\u{11128}𑄚\u{11134}𑄑𑄚 𑄢\u{1112b}"), ("ceb", "Estado de Quintana Roo"), ("cs", "Quintana Roo"), ("cy", "Quintana Roo"), ("da", "Quintana Roo"), ("de", "Quintana Roo"), ("el", "Κιντάνα Ρόο"), ("en", "Quintana Roo"), ("es", "Quintana Roo"), ("et", "Quintana Roo"), ("eu", "Quintana Roo"), ("fa", "کینتانا رو"), ("fi", "Quintana Roo"), ("fr", "Quintana Roo"), ("ga", "Quintana Roo"), ("gl", "Quintana Roo"), ("gu", "ક\u{acd}વિન\u{acd}ટાના ર\u{ac1}"), ("he", "קינטנה רו"), ("hi", "क\u{94d}वि\u{902}टाना र\u{942}"), ("hr", "Quintana Roo"), ("hu", "Quintana Roo"), ("hy", "Կինտանա Ռոո"), ("id", "Quintana Roo"), ("it", "Quintana Roo"), ("ja", "キンタナ・ロー州"), ("ka", "კინტანა-როოს შტატი"), ("kn", "ಕ\u{ccd}ವ\u{cbf}ಂಟಾನಾ ರ\u{cc2}"), ("ko", "킨타나로오 주"), ("lt", "Kintana Roo"), ("lv", "Kintana Roo"), ("mk", "Кинтана Ро"), ("mr", "कि\u{902}ताना रो"), ("ms", "Quintana Roo"), ("nb", "Quintana Roo"), ("nl", "Quintana Roo"), ("no", "Quintana Roo"), ("pl", "Quintana Roo"), ("pt", "Quintana Roo"), ("ro", "Quintana Roo"), ("ru", "Кинтана-Роо"), ("si", "ක\u{dca}ව\u{dd2}න\u{dca}ටන\u{dcf} ර\u{dd6}"), ("sk", "Quintana Roo"), ("sr", "Кинтана Ро"), ("sr_Latn", "Kintana Ro"), ("sv", "Quintana Roo"), ("sw", "Quintana Roo"), ("ta", "குயின\u{bcd}டன\u{bbe} ரூ"), ("te", "క\u{c4d}వ\u{c3f}ంట\u{c3e}న\u{c3e} రూ"), ("th", "ร\u{e31}ฐก\u{e34}นตานาโร"), ("tr", "Quintana Roo"), ("uk", "Кінтана-Роо"), ("ur", "کوینتانا رو"), ("uz", "Kintana-roo"), ("vi", "Quintana Roo"), ("yue", "金塔納羅奧州"), ("yue_Hans", "金塔纳罗奥州"), ("zh", "金塔納羅奧州")]),
                        unofficial_name_list: ["Quintana Roo"].to_vec(),
                    }
                ),
                (
                    "SIN",
                    Subdivision{
                        name: "SIN",
                        country_alpha2: Alpha2::MX,
                        code: "SIN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.825701), longitude: Some(-108.214302), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sinaloa"), ("ar", "ولاية سينالوا"), ("be", "Штат Сіналоа"), ("bg", "Синалоа"), ("bn", "সিন\u{9be}লোয\u{9bc}\u{9be}"), ("ca", "Sinaloa"), ("ccp", "𑄥\u{11128}𑄚𑄣\u{11131}"), ("ceb", "Estado de Sinaloa"), ("cs", "Sinaloa"), ("cy", "Sinaloa"), ("da", "Sinaloa"), ("de", "Sinaloa"), ("el", "Σιναλόα"), ("en", "Sinaloa"), ("es", "Sinaloa"), ("et", "Sinaloa"), ("eu", "Sinaloa"), ("fa", "سینالوآ"), ("fi", "Sinaloa"), ("fr", "Sinaloa"), ("ga", "Sinaloa"), ("gl", "Sinaloa"), ("gu", "સિનાલોઆ"), ("he", "סינלואה"), ("hi", "सिनालोआ"), ("hr", "Sinaloa"), ("hu", "Sinaloa"), ("hy", "Սինալոա"), ("id", "Sinaloa"), ("is", "Sinaloa"), ("it", "Sinaloa"), ("ja", "シナロア州"), ("jv", "Sinaloa"), ("ka", "სინალოა"), ("kn", "ಸ\u{cbf}ನಾಲೋವಾ"), ("ko", "시날로아 주"), ("lt", "Sinaloa"), ("lv", "Sinaloa"), ("mr", "सिनालोआ"), ("ms", "Sinaloa"), ("nb", "Sinaloa"), ("ne", "सिनालोआ"), ("nl", "Sinaloa"), ("no", "Sinaloa"), ("pl", "Sinaloa"), ("pt", "Sinaloa"), ("ro", "Sinaloa"), ("ru", "Синалоа"), ("si", "ස\u{dd2}නලොආ"), ("sk", "Sinaloa"), ("sr", "Синалоа"), ("sr_Latn", "Sinaloa"), ("sv", "Sinaloa"), ("sw", "Sinaloa"), ("ta", "சினலோங\u{bcd}"), ("te", "స\u{c3f}న\u{c3e}ల\u{c4b}వ\u{c3e}"), ("th", "ร\u{e31}ฐซ\u{e35}นาโลอา"), ("tr", "Sinaloa"), ("uk", "Сіналоа"), ("ur", "سینالوا"), ("uz", "Sinaloa"), ("vi", "Sinaloa"), ("yo", "Sinaloa"), ("yo_BJ", "Sinaloa"), ("yue", "錫那羅亞州"), ("yue_Hans", "锡那罗亚州"), ("zh", "錫那羅亞州")]),
                        unofficial_name_list: ["Sinaloa"].to_vec(),
                    }
                ),
                (
                    "SLP",
                    Subdivision{
                        name: "SLP",
                        country_alpha2: Alpha2::MX,
                        code: "SLP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.1564699), longitude: Some(-100.9855409), max_latitude: Some(22.2092432), min_latitude: Some(22.0620884), max_longitude: Some(-100.8747596), min_longitude: Some(-101.0467011)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية سان لويس بوتوسي"), ("be", "Штат Сан-Луіс-Патасі"), ("bg", "Сан Луис Потоси"), ("bn", "স\u{9be}ন ল\u{9c1}ইস পোতোসি"), ("ca", "Estat de San Luis Potosí"), ("ccp", "𑄥𑄚\u{11134} 𑄣\u{1112a}𑄃\u{11128}𑄌\u{11134} 𑄛\u{1112e}𑄑\u{1112e}𑄥\u{11128}"), ("ceb", "Estado de San Luis Potosí"), ("cs", "San Luis Potosí"), ("cy", "San Luis Potosí"), ("da", "San Luis Potosí"), ("de", "San Luis Potosí"), ("el", "Σαν Λουίς Ποτοσί"), ("en", "San Luis Potosí"), ("es", "San Luis Potosí"), ("et", "San Luis Potosí osariik"), ("eu", "San Luis Potosí"), ("fa", "سن لوئیس پوتوسی"), ("fi", "San Luis Potosí"), ("fr", "San Luis Potosí"), ("ga", "San Luis Potosí"), ("gl", "Estado de San Luis Potosí"), ("gu", "સાન લ\u{ac1}ઈસ પોટોસી"), ("he", "סן לואיס פוטוסי"), ("hi", "स\u{948}न ल\u{941}इ पोटोसी"), ("hr", "San Luis Potosí"), ("hu", "San Luis Potosí"), ("hy", "Սան Լուիս"), ("id", "San Luis Potosí"), ("it", "San Luis Potosí"), ("ja", "サン・ルイス・ポトシ州"), ("ka", "სან-ლუის-პოტოსის შტატი"), ("kn", "ಸ\u{ccd}ಯಾನ\u{ccd} ಲ\u{cc2}ಯ\u{cbf}ಸ\u{ccd} ಪೊಟೊಸ\u{cbf}"), ("ko", "산루이스포토시 주"), ("lt", "San Luis Potosi"), ("lv", "Sanluisa Potosi"), ("mr", "सान ल\u{941}इस पोतोसी"), ("ms", "San Luis Potosí"), ("nb", "San Luis Potosí"), ("nl", "San Luis Potosí"), ("no", "San Luis Potosí"), ("pl", "San Luis Potosí"), ("pt", "San Luis Potosí"), ("ro", "San Luis Potosí"), ("ru", "Сан-Луис-Потоси"), ("si", "සැන\u{dca} ල\u{dd4}ව\u{dd2}ස\u{dca} පොටෝස\u{dd2}"), ("sk", "San Luis Potosí"), ("sr", "Држава Сан Луис Потоси"), ("sr_Latn", "Država San Luis Potosi"), ("sv", "San Luis Potosí"), ("sw", "San Luis Potosí"), ("ta", "ச\u{bbe}ன\u{bcd} லுயிஸ\u{bcd} போடோசி"), ("te", "స\u{c3e}న\u{c4d} లూయ\u{c40} ప\u{c4b}ట\u{c4b}స\u{c3f}"), ("th", "ร\u{e31}ฐซ\u{e31}นล\u{e38}ยส\u{e4c}โปโตซ\u{e35}"), ("tr", "San Luis Potosí"), ("uk", "Сан-Луїс-Потосі"), ("ur", "سان لوئیس پوتوسی"), ("uz", "San-Luis-Potosi"), ("vi", "San Luis Potosí"), ("yue", "聖路易斯波托西州"), ("yue_Hans", "圣路易斯波托西州"), ("zh", "聖路易斯波托西州")]),
                        unofficial_name_list: ["San Luis Potosí"].to_vec(),
                    }
                ),
                (
                    "SON",
                    Subdivision{
                        name: "SON",
                        country_alpha2: Alpha2::MX,
                        code: "SON",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.2972247), longitude: Some(-110.3308814), max_latitude: Some(32.4937985), min_latitude: Some(26.2969879), max_longitude: Some(-108.4242708), min_longitude: Some(-115.0530223)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sonora"), ("ar", "ولاية سونورا"), ("be", "Штат Санора"), ("bg", "Сонора"), ("bn", "সোনোর\u{9be}"), ("bs", "Sonora"), ("ca", "Sonora"), ("ccp", "𑄥\u{1112e}𑄚\u{1112e}𑄢"), ("ceb", "Estado de Sonora"), ("cs", "Sonora"), ("cy", "Sonora"), ("da", "Sonora"), ("de", "Sonora"), ("el", "Σονόρα"), ("en", "Sonora"), ("es", "Sonora"), ("et", "Sonora"), ("eu", "Sonora"), ("fa", "ایالت سونورا"), ("fi", "Sonora"), ("fr", "Sonora"), ("ga", "Sonora"), ("gl", "Sonora"), ("gu", "સોનોરા"), ("he", "סונורה"), ("hi", "सोनोरा"), ("hr", "Sonora"), ("hu", "Sonora"), ("hy", "Սոնորա"), ("id", "Sonora"), ("it", "Sonora"), ("ja", "ソノラ州"), ("ka", "სონორა"), ("kn", "ಸೊನೊರಾ"), ("ko", "소노라 주"), ("lt", "Sonora"), ("lv", "Sonora"), ("mk", "Сонора"), ("mr", "सोनोरा"), ("ms", "Sonora"), ("nb", "Sonora"), ("nl", "Sonora"), ("no", "Sonora"), ("pa", "ਸ\u{a4b}ਨ\u{a4b}ਰਾ"), ("pl", "Sonora"), ("pt", "Sonora"), ("ro", "Sonora"), ("ru", "Сонора"), ("si", "සැනොර\u{dcf}"), ("sk", "Sonora"), ("sr", "Сонора"), ("sr_Latn", "Sonora"), ("sv", "Sonora"), ("sw", "Sonora"), ("ta", "சொனோர\u{bbe}"), ("te", "స\u{c4b}న\u{c4b}ర\u{c3e}"), ("th", "ร\u{e31}ฐโซโนรา"), ("tr", "Sonora"), ("uk", "Сонора"), ("ur", "سونورا"), ("uz", "Sonora"), ("vi", "Sonora"), ("yue", "索諾拉州"), ("yue_Hans", "索诺拉州"), ("zh", "索諾拉州")]),
                        unofficial_name_list: ["Sonora"].to_vec(),
                    }
                ),
                (
                    "TAB",
                    Subdivision{
                        name: "TAB",
                        country_alpha2: Alpha2::MX,
                        code: "TAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.8409173), longitude: Some(-92.6189273), max_latitude: Some(18.6502994), min_latitude: Some(17.2508934), max_longitude: Some(-90.98745919999999), min_longitude: Some(-94.12979179999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tabasco"), ("ar", "تاباسكو"), ("be", "Штат Табаска"), ("bg", "Табаско"), ("bn", "ত\u{9be}ব\u{9be}সকো"), ("ca", "Tabasco"), ("ccp", "𑄑𑄝𑄌\u{11134}𑄇\u{1112e}"), ("ceb", "Estado de Tabasco"), ("cs", "Tabasco"), ("cy", "Tabasco"), ("da", "Tabasco"), ("de", "Tabasco"), ("el", "Ταμπάσκο"), ("en", "Tabasco"), ("es", "Tabasco"), ("et", "Tabasco"), ("eu", "Tabasco"), ("fa", "ایالت تاباسکو"), ("fi", "Tabasco"), ("fr", "Tabasco"), ("ga", "Tabasco"), ("gl", "Tabasco"), ("gu", "તાબાસ\u{acd}કો"), ("he", "טבסקו"), ("hi", "टब\u{948}स\u{94d}को"), ("hr", "Tabasco"), ("hu", "Tabasco"), ("hy", "Տաբասկո"), ("id", "Tabasco"), ("it", "Tabasco"), ("ja", "タバスコ州"), ("ka", "ტაბასკოს შტატი"), ("kn", "ತಬಾಸ\u{ccd}ಕೊ"), ("ko", "타바스코 주"), ("lt", "Tabaskas"), ("lv", "Tabasko"), ("ml", "ടബ\u{d3e}സ\u{d4d}കോ"), ("mr", "ताबास\u{94d}को"), ("ms", "Tabasco"), ("nb", "Tabasco"), ("nl", "Tabasco"), ("no", "Tabasco"), ("pl", "Tabasco"), ("pt", "Tabasco"), ("ro", "Tabasco"), ("ru", "Табаско"), ("si", "ටබස\u{dca}කෝ"), ("sk", "Tabasco"), ("sr", "Табаско"), ("sr_Latn", "Tabasko"), ("sv", "Tabasco"), ("sw", "Tabasco"), ("ta", "டப\u{bbe}ஸ\u{bcd}கோ"), ("te", "టబ\u{c3e}స\u{c4d}క\u{c4b}"), ("th", "ร\u{e31}ฐตาบ\u{e31}สโก"), ("tr", "Tabasco"), ("uk", "Табаско"), ("ur", "تاباسکو"), ("uz", "Tabasko"), ("vi", "Tabasco"), ("yue", "塔巴斯科州"), ("yue_Hans", "塔巴斯科州"), ("zh", "塔巴斯科州")]),
                        unofficial_name_list: ["Tabasco"].to_vec(),
                    }
                ),
                (
                    "TAM",
                    Subdivision{
                        name: "TAM",
                        country_alpha2: Alpha2::MX,
                        code: "TAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.26694), longitude: Some(-98.8362755), max_latitude: Some(27.6791263), min_latitude: Some(22.2069658), max_longitude: Some(-97.14426089999999), min_longitude: Some(-100.1449502)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولاية تاماوليباس"), ("be", "Штат Тамауліпас"), ("bg", "Тамаулипас"), ("bn", "ত\u{9be}ম\u{9be}উলিপ\u{9be}স"), ("ca", "Tamaulipas"), ("ccp", "𑄑𑄟𑄅\u{1112a}𑄣\u{11128}𑄛𑄌\u{11134}"), ("ceb", "Estado de Tamaulipas"), ("cs", "Tamaulipas"), ("cy", "Tamaulipas"), ("da", "Tamaulipas"), ("de", "Tamaulipas"), ("el", "Ταμαουλίπας"), ("en", "Tamaulipas"), ("es", "Tamaulipas"), ("et", "Tamaulipas"), ("eu", "Tamaulipas"), ("fa", "تامائولیپاس"), ("fi", "Tamaulipas"), ("fr", "Tamaulipas"), ("ga", "Tamaulipas"), ("gl", "Tamaulipas"), ("gu", "તામાઉલિપાસ"), ("he", "טמאוליפס"), ("hi", "तमौलिपास"), ("hr", "Tamaulipas"), ("hu", "Tamaulipas"), ("hy", "Տամաուլիպաս"), ("id", "Tamaulipas"), ("it", "Tamaulipas"), ("ja", "タマウリパス州"), ("ka", "ტამაულიპასის შტატი"), ("kn", "ತಮ\u{ccc}ಲ\u{cbf}ಪಾಸ\u{ccd}"), ("ko", "타마울리파스 주"), ("lt", "Tamaulipasas"), ("lv", "Tamaulipasa"), ("mr", "तामौलिपास"), ("ms", "Tamaulipas"), ("nb", "Tamaulipas"), ("nl", "Tamaulipas"), ("no", "Tamaulipas"), ("pl", "Tamaulipas"), ("pt", "Tamaulipas"), ("ro", "Tamaulipas"), ("ru", "Тамаулипас"), ("si", "ටමෞල\u{dd2}ප\u{dcf}ස\u{dca}"), ("sk", "Tamaulipas"), ("sl", "Tamaulipas"), ("sr", "Тамаулипас"), ("sr_Latn", "Tamaulipas"), ("sv", "Tamaulipas"), ("sw", "Tamaulipas"), ("ta", "தமௌலிப\u{bbe}ஸ\u{bcd}"), ("te", "ట\u{c3e}మ\u{c3e}ల\u{c3f}ప\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐตาเมาล\u{e35}ป\u{e31}ส"), ("tr", "Tamaulipas"), ("uk", "Тамауліпас"), ("ur", "تاماولیپاس"), ("uz", "Tamaulipas"), ("vi", "Tamaulipas"), ("yue", "塔毛利帕斯州"), ("yue_Hans", "塔毛利帕斯州"), ("zh", "塔毛利帕斯州")]),
                        unofficial_name_list: ["Tamaulipas"].to_vec(),
                    }
                ),
                (
                    "TLA",
                    Subdivision{
                        name: "TLA",
                        country_alpha2: Alpha2::MX,
                        code: "TLA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.3181521), longitude: Some(-98.2375146), max_latitude: Some(19.3264598), min_latitude: Some(19.3056666), max_longitude: Some(-98.2316719), min_longitude: Some(-98.24796239999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tlaxcala"), ("ar", "ولاية تلاكسكالا"), ("be", "Штат Тласкала"), ("bg", "Тласкала"), ("bn", "ত\u{9cd}ল\u{9be}ক\u{9cd}সক\u{9be}ল\u{9be}"), ("ca", "Estat de Tlaxcala"), ("ccp", "𑄣𑄇\u{11134}𑄇𑄣"), ("ceb", "Estado de Tlaxcala"), ("cs", "Tlaxcala"), ("cy", "Tlaxcala"), ("da", "Tlaxcala"), ("de", "Tlaxcala"), ("el", "Τλαξκάλα"), ("en", "Tlaxcala"), ("es", "Tlaxcala"), ("et", "Tlaxcala osariik"), ("eu", "Tlaxcala"), ("fa", "تلاسکالا"), ("fi", "Tlaxcala"), ("fr", "Tlaxcala"), ("ga", "Tlaxcala"), ("gl", "Estado de Tlaxcala"), ("gu", "ત\u{acd}લાક\u{acd}સ\u{acd}કાલા"), ("he", "טלקסקלה"), ("hi", "ल\u{948}क\u{94d}सकाला"), ("hr", "Tlaxcala"), ("hu", "Tlaxcala"), ("hy", "Տլասկալա"), ("id", "Tlaxcala"), ("it", "Tlaxcala"), ("ja", "トラスカラ州"), ("ka", "ტლასკალა"), ("kn", "ಟ\u{cbf}ಲಾಕ\u{ccd}ಸ\u{ccd}ಕಾಲಾ"), ("ko", "틀락스칼라 주"), ("lt", "Tlaskala"), ("lv", "Tlaskala"), ("mr", "त\u{94d}लास\u{94d}काला"), ("ms", "Tlaxcala"), ("nb", "Tlaxcala"), ("nl", "Tlaxcala"), ("no", "Tlaxcala"), ("pl", "Tlaxcala"), ("pt", "Tlaxcala"), ("ro", "Tlaxcala"), ("ru", "Тласкала"), ("si", "ට\u{dca}ලක\u{dca}ස\u{dca}කල\u{dcf}"), ("sk", "Tlaxcala"), ("sr", "Држава Тласкала"), ("sr_Latn", "Država Tlaskala"), ("sv", "Tlaxcala"), ("sw", "Tlaxcala"), ("ta", "ட\u{bcd}லஸ\u{bcd}க\u{bcd}க\u{bbe}ல"), ("te", "ట\u{c4d}ల\u{c3e}క\u{c4d}స\u{c3f}క\u{c3e}ల\u{c3e}"), ("th", "ร\u{e31}ฐตล\u{e31}ซกาลา"), ("tr", "Tlaxcala"), ("uk", "Тласкала"), ("ur", "تلاکسکالا"), ("uz", "Tlaskala"), ("vi", "Tlaxcala"), ("yue", "特拉斯卡拉州"), ("yue_Hans", "特拉斯卡拉州"), ("zh", "特拉斯卡拉州")]),
                        unofficial_name_list: ["Tlaxcala"].to_vec(),
                    }
                ),
                (
                    "VER",
                    Subdivision{
                        name: "VER",
                        country_alpha2: Alpha2::MX,
                        code: "VER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.173773), longitude: Some(-96.1342241), max_latitude: Some(19.2266685), min_latitude: Some(19.1309432), max_longitude: Some(-96.1181965), min_longitude: Some(-96.206622)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Veracruz"), ("ar", "ولاية فيراكروز"), ("be", "Штат Веракрус"), ("bg", "Веракрус"), ("bn", "ভের\u{9be}ক\u{9cd}র\u{9c1}জ"), ("ca", "Veracruz"), ("ccp", "𑄞𑄢𑄝\u{11133}𑄢\u{1112a}𑄌\u{11134}"), ("ceb", "Estado de Veracruz-Llave"), ("cs", "Veracruz"), ("cy", "Veracruz"), ("da", "Veracruz"), ("de", "Veracruz"), ("el", "Βερακρούζ"), ("en", "Veracruz"), ("es", "Veracruz"), ("et", "Veracruzi osariik"), ("eu", "Veracruz"), ("fa", "وراکروس"), ("fi", "Veracruz"), ("fr", "Veracruz"), ("ga", "Veracruz"), ("gl", "Estado de Veracruz"), ("gu", "વ\u{ac7}રાક\u{acd}ર\u{ac1}ઝ"), ("he", "וראקרוס"), ("hi", "व\u{947}राक\u{94d}र\u{941}ज\u{93c}"), ("hr", "Veracruz"), ("hu", "Veracruz"), ("hy", "Վերակրուս"), ("id", "Veracruz"), ("it", "Veracruz"), ("ja", "ベラクルス州"), ("ka", "ვერაკრუსის შტატი"), ("kn", "ವ\u{cc6}ರಾಕ\u{ccd}ರಜ\u{ccd}"), ("ko", "베라크루스 주"), ("lt", "Verakrusas"), ("lv", "Verakrusa de Ignasio de la Ljave"), ("mk", "Веракруз"), ("mr", "ब\u{947}राक\u{94d}र\u{941}थ"), ("ms", "Veracruz"), ("my", "ဗေရကရ\u{102f}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Veracruz"), ("nl", "Veracruz de Ignacio de la Llave"), ("no", "Veracruz"), ("pa", "ਬ\u{a47}ਰਾਕਰ\u{a42}ਸ"), ("pl", "Veracruz"), ("pt", "Veracruz"), ("ro", "Statul Veracruz"), ("ru", "Веракрус"), ("si", "වෙරකෘස\u{dca}"), ("sk", "Veracruz"), ("sr", "Држава Веракруз"), ("sr_Latn", "Država Verakruz"), ("sv", "Veracruz"), ("sw", "Veracruz"), ("ta", "வேர\u{bbe}கிருஸ\u{bcd}"), ("te", "వ\u{c46}ర\u{c3e}క\u{c4d}రజ\u{c4d}"), ("th", "ร\u{e31}ฐเบรากร\u{e38}ซ"), ("tr", "Veracruz"), ("uk", "Веракрус"), ("ur", "ویراکروز"), ("uz", "Verakrus"), ("vi", "Veracruz"), ("yue", "韋拉克魯斯州"), ("yue_Hans", "韦拉克鲁斯州"), ("zh", "韋拉克魯斯州")]),
                        unofficial_name_list: ["Veracruz de Ignacio de la Llave"].to_vec(),
                    }
                ),
                (
                    "YUC",
                    Subdivision{
                        name: "YUC",
                        country_alpha2: Alpha2::MX,
                        code: "YUC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.7098786), longitude: Some(-89.0943377), max_latitude: Some(21.6242092), min_latitude: Some(19.551174), max_longitude: Some(-87.5331451), min_longitude: Some(-90.40699579999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Yucatán"), ("ar", "ولاية يوكاتان"), ("be", "Штат Юкатан"), ("bg", "Юкатан"), ("bn", "ইয\u{9bc}\u{9c1}ক\u{9be}ত\u{9be}ন"), ("ca", "Yucatán"), ("ccp", "𑄃\u{11128}𑄅\u{1112a}𑄇𑄑𑄚\u{11134}"), ("ceb", "Estado de Yucatán"), ("cs", "Yucatán"), ("cy", "Yucatán"), ("da", "Yucatán"), ("de", "Yucatán"), ("el", "Γιουκατάν"), ("en", "Yucatán"), ("es", "Yucatán"), ("et", "Yucatán"), ("eu", "Yucatán"), ("fa", "یوکاتان"), ("fi", "Yucatán"), ("fr", "Yucatán"), ("ga", "Yucatán"), ("gl", "Estado de Iucatán"), ("gu", "ય\u{ac1}કાટન"), ("he", "יוקטן"), ("hi", "य\u{941}काटन"), ("hr", "Yucatán"), ("hu", "Yucatán"), ("hy", "Յուկատան նահանգ"), ("id", "Yucatán"), ("it", "Yucatán"), ("ja", "ユカタン州"), ("ka", "იუკატანის შტატი"), ("kn", "ಯುಕಾಟಾನ\u{ccd}"), ("ko", "유카탄 주"), ("lt", "Jukatanas"), ("lv", "Jukatana"), ("mk", "Јукатан"), ("mr", "य\u{941}कातान"), ("ms", "Yucatán"), ("nb", "Yucatán"), ("nl", "Yucatán"), ("no", "Yucatán"), ("pa", "ਯ\u{a41}ਕਾਤਾਨ"), ("pl", "Jukatan"), ("pt", "Iucatã"), ("ro", "Yucatán"), ("ru", "Юкатан"), ("si", "ය\u{dd4}කට\u{dcf}න\u{dca}"), ("sk", "Yucatán"), ("sr", "Јукатан"), ("sr_Latn", "Jukatan"), ("sv", "Yucatán"), ("sw", "Yucatán"), ("ta", "யுகேடன\u{bcd}"), ("te", "యూక\u{c3e}టన\u{c4d}"), ("th", "ร\u{e31}ฐย\u{e39}กาต\u{e31}ง"), ("tr", "Yucatán"), ("uk", "Юкатан"), ("ur", "یوکتان"), ("uz", "Yukatan"), ("vi", "Yucatán"), ("yue", "尤卡坦州"), ("yue_Hans", "尤卡坦州"), ("zh", "尤卡坦州")]),
                        unofficial_name_list: ["Yucatán"].to_vec(),
                    }
                ),
                (
                    "ZAC",
                    Subdivision{
                        name: "ZAC",
                        country_alpha2: Alpha2::MX,
                        code: "ZAC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.7709249), longitude: Some(-102.5832539), max_latitude: Some(22.7901168), min_latitude: Some(22.7451586), max_longitude: Some(-102.5549838), min_longitude: Some(-102.6196943)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zacatecas"), ("ar", "ولاية زاكاتيكاس"), ("be", "Штат Сакатэкас"), ("bg", "Сакатекас"), ("bn", "য\u{9be}ক\u{9be}তেক\u{9be}স"), ("ca", "Estat de Zacatecas"), ("ccp", "𑄎𑄇𑄖\u{11134}𑄇𑄌\u{11134}"), ("ceb", "Estado de Zacatecas"), ("cs", "Zacatecas"), ("cy", "Zacatecas"), ("da", "Zacatecas"), ("de", "Zacatecas"), ("el", "Ζακατέκας"), ("en", "Zacatecas"), ("es", "Zacatecas"), ("et", "Zacatecase osariik"), ("eu", "Zacatecas"), ("fa", "ساکاتکاس"), ("fi", "Zacatecas"), ("fr", "Zacatecas"), ("ga", "Zacatecas"), ("gl", "Estado de Zacatecas"), ("gu", "ઝ\u{ac7}કાટ\u{ac7}કાસ"), ("he", "סקטקס"), ("hi", "ज\u{93c}काट\u{947}कास"), ("hr", "Zacatecas"), ("hu", "Zacatecas"), ("hy", "Սակատեկաս"), ("id", "Zacatecas"), ("it", "Zacatecas"), ("ja", "サカテカス州"), ("ka", "საკატეკასის შტატი"), ("kn", "ಝಕಟ\u{cc6}ಕಾಸ\u{ccd}"), ("ko", "사카테카스 주"), ("lt", "Sakatekas"), ("lv", "Sakatekasa"), ("mk", "Закатекас"), ("mr", "साकात\u{947}कास"), ("ms", "Zacatecas"), ("nb", "Zacatecas"), ("nl", "Zacatecas"), ("no", "Zacatecas"), ("pa", "ਸਾਕਾਤ\u{a47}ਕਾਸ"), ("pl", "Zacatecas"), ("pt", "Zacatecas"), ("ro", "Zacatecas"), ("ru", "Сакатекас"), ("si", "සකටෙක\u{dcf}ස\u{dca}"), ("sk", "Zacatecas"), ("sr", "Држава Закатекас"), ("sr_Latn", "Država Zakatekas"), ("sv", "Zacatecas"), ("sw", "Zacatecas"), ("ta", "ச\u{bbe}க\u{bcd}க\u{bbe}டேஸ\u{bbe}ஸ\u{bcd}"), ("te", "జక\u{c3e}ట\u{c46}క\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐซากาเตก\u{e31}ส"), ("tr", "Zacatecas"), ("uk", "Сакатекас"), ("ur", "زاکاٹیکاس"), ("uz", "Sakatekas"), ("vi", "Zacatecas"), ("yue", "薩卡特卡斯州"), ("yue_Hans", "萨卡特卡斯州"), ("zh", "薩卡特卡斯州")]),
                        unofficial_name_list: ["Zacatecas"].to_vec(),
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
#[cfg(feature = "mx")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MX,
        alpha3: Alpha3::MEX,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::NorthAmerica,
        country_code: 52,
        currency_code: CurrencyCode::MXN,
        gec: Some(GEC::MX),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::MEX),
        iso_long_name: "The United Mexican States",
        iso_short_name: "Mexico",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9, 10].to_vec(),
        national_prefix: "01",
        nationality: Some("Mexican"),
        number: "484",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAmerica),
        un_locode: "MX",
        unofficial_name_list: ["Mexico", "Mexiko", "Mexique", "México", "メキシコ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Mexico"),
            ("af", "Meksiko"),
            ("ak", "Mexico"),
            ("am", "ሣጤሲጥ"),
            ("an", "Mexico"),
            ("ar", "المكسيك"),
            ("as", "মেক\u{9cd}সিকো"),
            ("ay", "Mexico"),
            ("az", "Meksika"),
            ("ba", "Mexico"),
            ("be", "Мексіка"),
            ("bg", "Мексико"),
            ("bi", "Mexico"),
            ("bn", "মেক\u{9cd}সিকো"),
            ("bn_IN", "মেক\u{9cd}সিকো"),
            ("br", "Mec'hiko"),
            ("bs", "Meksiko"),
            ("ca", "Mèxic"),
            ("ce", "Мексика"),
            ("ch", "Mexico"),
            ("cs", "Mexiko"),
            ("cv", "Мексика"),
            ("cy", "Mecsico"),
            ("da", "Mexico"),
            ("de", "Mexiko"),
            ("dv", "މ\u{7ac}ކ\u{7b0}ސ\u{7a8}ކ\u{7af}"),
            ("dz", "མ\u{f7a}ག་ས\u{f72}་ཀ\u{f7c}།"),
            ("ee", "Mexico"),
            ("el", "Μεξικό"),
            ("en", "Mexico"),
            ("eo", "Meksiko"),
            ("es", "México"),
            ("et", "Mehhiko"),
            ("eu", "Mexiko"),
            ("fa", "مکزیک"),
            ("ff", "Mexico"),
            ("fi", "Meksiko"),
            ("fo", "Meksiko"),
            ("fr", "Mexique"),
            ("fy", "Meksiko"),
            ("ga", "Meicsiceo"),
            ("gl", "México"),
            ("gn", "Mexico"),
            ("gu", "મ\u{ac7}ક\u{acd}સિકો"),
            ("gv", "Meksico"),
            ("ha", "Mexico"),
            ("he", "מקסיקו"),
            ("hi", "म\u{947}क\u{94d}सिको"),
            ("hr", "Meksiko"),
            ("ht", "Meksik"),
            ("hu", "Mexikó"),
            ("hy", "Մեքսիկա"),
            ("ia", "Mexico"),
            ("id", "Meksiko"),
            ("io", "Mexikia"),
            ("is", "Mexíkó"),
            ("it", "Messico"),
            ("iu", "ᒦᒃᓰᖂ"),
            ("ja", "メキシコ"),
            ("ka", "მექსიკა"),
            ("ki", "Mexico"),
            ("kk", "Мексика"),
            ("kl", "Mexico"),
            ("km", "ម\u{17c9}\u{17b7}ចស\u{17ca}\u{17b7}ក"),
            ("kn", "ಹೊಂಡುರಾಸ\u{ccd}"),
            ("ko", "멕시코"),
            ("ku", "Mexîko"),
            ("kv", "Мексика"),
            ("kw", "Meksiko"),
            ("ky", "Мексика"),
            ("lo", "ປະເທດເມ\u{eb1}ກຊ\u{eb4}ກ"),
            ("lt", "Meksika"),
            ("lv", "Meksika"),
            ("mi", "Mehiko"),
            ("mk", "Мексико"),
            ("ml", "മെക\u{d4d}സികോ"),
            ("mn", "Мексик"),
            ("mr", "म\u{947}क\u{94d}सिको"),
            ("ms", "Meksiko"),
            ("mt", "Messiku"),
            (
                "my",
                "မက\u{1039}ကဆ\u{102e}က\u{102d}\u{102f}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Meketiko"),
            ("nb", "Mexico"),
            ("ne", "म\u{947}क\u{94d}सिको"),
            ("nl", "Mexico"),
            ("nn", "Mexico"),
            ("nv", "Naakaii Bikéyah"),
            ("oc", "Mexic"),
            ("or", "ମେକ\u{b4d}ସ\u{b3f}କୋ"),
            ("pa", "ਮ\u{a48}ਕਸੀਕ\u{a4b}"),
            ("pi", "म\u{947}क\u{94d}सिको"),
            ("pl", "Meksyk"),
            ("ps", "Mexico"),
            ("pt", "México"),
            ("pt_BR", "México"),
            ("ro", "Mexic"),
            ("ru", "Мексика"),
            ("rw", "Megizike"),
            ("sc", "Mèssicu"),
            ("sd", "Mexico"),
            ("si", "මෙක\u{dca}ස\u{dd2}කෝව"),
            ("sk", "Mexiko"),
            ("sl", "Mehika"),
            ("so", "Meksiko"),
            ("sq", "Meksikë"),
            ("sr", "Мексико"),
            ("sv", "Mexiko"),
            ("sw", "Mexico"),
            ("ta", "மெக\u{bcd}ஸிகோ"),
            ("te", "మ\u{c47}క\u{c4d}స\u{c3f}క\u{c4b}"),
            ("tg", "Мексика"),
            ("th", "เม\u{e47}กซ\u{e34}โก"),
            ("ti", "ሜክሲኮ"),
            ("tk", "Meksikanyň"),
            ("tl", "Mehiko"),
            ("tr", "Meksika"),
            ("tt", "Мексико"),
            ("ug", "مېكسىكا"),
            ("uk", "Мексика"),
            ("ur", "میکسیکو"),
            ("uz", "Meksika"),
            ("ve", "Mexico"),
            ("vi", "Mê-hi-cô"),
            ("wa", "Mecsike"),
            ("wo", "Meksik"),
            ("xh", "Mexico"),
            ("yo", "Mẹ\u{301}ksíkò"),
            ("zh_CN", "墨西哥"),
            ("zh_HK", "墨西哥"),
            ("zh_TW", "墨西哥"),
            ("zu", "IMekisiko"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
