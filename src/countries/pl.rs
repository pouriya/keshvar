// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Poland

#[cfg(all(feature = "pl", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::PL;
    pub const ALPHA3: Alpha3 = Alpha3::POL;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 48;
    pub const CURRENCY_CODE: &str = "PLN";
    pub const GEC: Option<GEC> = Some(GEC::PL);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::POL);
    pub const ISO_SHORT_NAME: &str = "Poland";
    pub const ISO_LONG_NAME: &str = "The Republic of Poland";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pl"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pl"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Polish");
    pub const NUMBER: &str = "616";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{2}-\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "PL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Poland", "Polen", "Pologne", "Polonia", "ポーランド"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Poland"),
        ("af", "Pole"),
        ("ak", "Poland"),
        ("am", "ፖሒን፥"),
        ("an", "Poland"),
        ("ar", "بولندا"),
        ("as", "পলেণ\u{9cd}ড"),
        ("ay", "Poland"),
        ("az", "Polşa"),
        ("ba", "Poland"),
        ("be", "Польшча"),
        ("bg", "Полша"),
        ("bi", "Poland"),
        ("bn", "পোল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("bn_IN", "পোল\u{9cd}য\u{9be}ন\u{9cd}ড"),
        ("br", "Polonia"),
        ("bs", "Poljska"),
        ("ca", "Polònia"),
        ("ce", "Польша"),
        ("ch", "Polaki"),
        ("cs", "Polsko"),
        ("cv", "Польша"),
        ("cy", "Gwlad Pwyl"),
        ("da", "Polen"),
        ("de", "Polen"),
        ("dv", "ޕ\u{7ae}ލ\u{7ac}ނ\u{7b0}ޑ\u{7aa}"),
        ("dz", "པ\u{f7c}་ལ\u{f7a}ནཌ\u{f72}།"),
        ("ee", "Poland"),
        ("el", "Πολωνία"),
        ("en", "Poland"),
        ("eo", "Pollando"),
        ("es", "Polonia"),
        ("et", "Poola"),
        ("eu", "Polonia"),
        ("fa", "لهستان"),
        ("ff", "Poloonya"),
        ("fi", "Puola"),
        ("fo", "Pólland"),
        ("fr", "Pologne"),
        ("fy", "Poalen"),
        ("ga", "An Pholainn"),
        ("gl", "Polonia"),
        ("gn", "Poland"),
        ("gu", "પોલ\u{ac7}ન\u{acd}ડ"),
        ("gv", "Yn Pholynn"),
        ("ha", "Poland"),
        ("he", "פולין"),
        ("hi", "पोल\u{948}\u{902}ड"),
        ("hr", "Poljska"),
        ("ht", "Polòy"),
        ("hu", "Lengyelország"),
        ("hy", "Լեհաստան"),
        ("ia", "Polonia"),
        ("id", "Polandia"),
        ("io", "Polonia"),
        ("is", "Pólland"),
        ("it", "Polonia"),
        ("iu", "ᐳᓚᓐᑦ"),
        ("ja", "ポーランド"),
        ("ka", "პოლონეთი"),
        ("ki", "Poland"),
        ("kk", "Польша"),
        ("kl", "Poland"),
        ("km", "ប\u{17c9}\u{17bc}ឡ\u{17bc}ញ"),
        ("kn", "ಪೋಲಂಡ\u{ccd}"),
        ("ko", "폴란드"),
        ("ku", "Polonya"),
        ("kv", "Польша"),
        ("kw", "Poloni"),
        ("ky", "Польша"),
        ("lo", "ປະເທດໂປໂລຍ"),
        ("lt", "Lenkija"),
        ("lv", "Polija"),
        ("mi", "Pōrana"),
        ("mk", "Полска"),
        ("ml", "പോളണ\u{d4d}ട\u{d4d}"),
        ("mn", "Польш"),
        ("mr", "पोल\u{902}ड"),
        ("ms", "Poland"),
        ("mt", "Polonja"),
        (
            "my",
            "ပ\u{102d}\u{102f}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Poran"),
        ("nb", "Polen"),
        ("ne", "पोल\u{94d}याण\u{94d}ड"),
        ("nl", "Polen"),
        ("nn", "Polen"),
        ("nv", "Póolish Dineʼé Bikéyah"),
        ("oc", "Polonha"),
        ("or", "ପୋଲ\u{b3e}ଣ\u{b4d}ଡ"),
        ("pa", "ਪ\u{a4b}ਲ\u{a48}\u{a02}ਡ"),
        ("pi", "पोल\u{948}\u{902}ड"),
        ("pl", "Polska"),
        ("ps", "پولنډ"),
        ("pt", "Polónia"),
        ("pt_BR", "Polônia"),
        ("ro", "Polonia"),
        ("ru", "Польша"),
        ("rw", "Polonye"),
        ("sc", "Polònia"),
        ("sd", "پولينڊ"),
        ("si", "පෝලන\u{dca}තය"),
        ("sk", "Poľsko"),
        ("sl", "Poljska"),
        ("so", "Booland"),
        ("sq", "Poloni"),
        ("sr", "Пољска"),
        ("sv", "Polen"),
        ("sw", "Poland"),
        ("ta", "போலந\u{bcd}து"),
        ("te", "ప\u{c4b}ల\u{c3e}ండ\u{c4d}"),
        ("tg", "Лаҳистон"),
        ("th", "โปแลนด\u{e4c}"),
        ("ti", "ፖላንድ"),
        ("tk", "Polşa"),
        ("tl", "Poland"),
        ("tr", "Polonya"),
        ("tt", "Полониа, Полша"),
        ("ug", "پولشا"),
        ("uk", "Польща"),
        ("ur", "بولندا"),
        ("uz", "Polsha"),
        ("ve", "Poland"),
        ("vi", "Ba Lan"),
        ("wa", "Pologne"),
        ("wo", "Poloñ"),
        ("xh", "Poland"),
        ("yo", "Pólàndì"),
        ("zh_CN", "波兰"),
        ("zh_HK", "波蘭"),
        ("zh_TW", "波蘭"),
        ("zu", "IPolandi"),
    ];
    #[cfg(all(feature = "pl", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 51.919438;
        pub const LONGITUDE: f64 = 19.145136;
        pub const MAX_LATITUDE: f64 = 54.9054761;
        pub const MAX_LONGITUDE: f64 = 24.1458931;
        pub const MIN_LATITUDE: f64 = 49.002025;
        pub const MIN_LONGITUDE: f64 = 14.1228641;
        pub const NORTHEAST_LATITUDE: f64 = 54.9054761;
        pub const NORTHEAST_LONGITUDE: f64 = 24.1458931;
        pub const SOUTHWEST_LATITUDE: f64 = 49.002025;
        pub const SOUTHWEST_LONGITUDE: f64 = 14.1228641;
    }
}
#[cfg(all(feature = "pl", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 51.919438,
            longitude: 19.145136,
            max_latitude: 54.9054761,
            max_longitude: 24.1458931,
            min_latitude: 49.002025,
            min_longitude: 14.1228641,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 54.9054761,
                    longitude: 24.1458931,
                },
                southwest: CountryGeoBound {
                    latitude: 49.002025,
                    longitude: 14.1228641,
                },
            },
        }
    }
}

#[cfg(all(feature = "pl", feature = "subdivisions"))]
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
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::PL,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.13398609999999), longitude: Some(16.8841961), max_latitude: Some(51.8047592), min_latitude: Some(50.09634), max_longitude: Some(17.798917), min_longitude: Some(14.816831)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيليزيا السفلى"), ("be", "Ніжнесілезскае ваяводства"), ("bg", "Долносилезко войводство"), ("bn", "লোয\u{9bc}\u{9be}র সিলেসিয\u{9bc}\u{9be}ন ভৈভেডেশিপ"), ("ca", "Voivodat de Baixa Silèsia"), ("ccp", "𑄣\u{1112e}𑄠𑄢\u{11134} 𑄥\u{1112d}𑄣𑄬𑄥\u{11128}𑄠𑄚\u{11134}"), ("cs", "Dolnoslezské vojvodství"), ("da", "Województwo dolnośląskie"), ("de", "Woiwodschaft Niederschlesien"), ("el", "Κάτω Σιλεσία"), ("en", "Lower Silesia"), ("es", "Baja Silesia"), ("et", "Alam-Sileesia vojevoodkond"), ("eu", "Silesia Behereko voivoderria"), ("fa", "استان سلیزیا سفلی"), ("fi", "Ala-Sleesian voivodikunta"), ("fr", "Voïvodie de Basse-Silésie"), ("gl", "Baixa Silesia"), ("gu", "લોઅર સિલ\u{ac7}સિઅન વોઈવોડ\u{ac7}શીપ"), ("he", "שלזיה תחתית"), ("hi", "निचला सिल\u{947}सियन वोइवोडीशिप"), ("hr", "Donjošlesko vojvodstvo"), ("hu", "Alsó-sziléziai vajdaság"), ("id", "Provinsi Dolnośląskie"), ("is", "Neðri-Slesía"), ("it", "voivodato della Bassa Slesia"), ("ja", "ドルヌィ・シロンスク県"), ("ka", "ქვემო სილეზიის სავოევოდო"), ("kn", "ಲೋವರ\u{ccd} ಸೈಲ\u{ccd}ಸ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "돌니실롱스크 주"), ("lt", "Žemutinės Silezijos vaivadija"), ("lv", "Lejassilēzijas vojevodiste"), ("mr", "डॉल\u{94d}नोश\u{94d}लो\u{902}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Dolnoslaskie"), ("nb", "Nederschlesiske voivodskap"), ("nl", "Neder-Silezië"), ("no", "Nederschlesiske voivodskap"), ("pl", "województwo dolnośląskie"), ("pt", "Voivodia da Baixa Silésia"), ("ro", "Silezia Inferioară"), ("ru", "Нижнесилезское воеводство"), ("si", "පහල ස\u{dd2}ලෙස\u{dd2}යන\u{dca} වෞඉවොදෙශ\u{dd2}ප\u{dca}"), ("sk", "Dolnosliezske vojvodstvo"), ("sr", "Војводство Доње Шлеско"), ("sr_Latn", "Vojvodstvo Donje Šlesko"), ("sv", "Nedre Schlesiens vojvodskap"), ("ta", "லோவெர\u{bcd} சிலேசியன\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ల\u{c4b}యర\u{c4d} స\u{c3f}ల\u{c47}స\u{c3f}యన\u{c4d} వ\u{c3e}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดดอลน\u{e37}อชล\u{e47}อนสก\u{e4c}"), ("tr", "Aşağı Silezya Voyvodalığı"), ("uk", "Нижньосілезьке воєводство"), ("ur", "زیریں سیلیزیا صوبہ"), ("vi", "Dolnośląskie"), ("zh", "下西里西亚省")]),
                        unofficial_name_list: ["Dolnośląskie", "dolnośląskie"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::PL,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.1648363), longitude: Some(18.4834224), max_latitude: Some(53.7809987), min_latitude: Some(52.33079), max_longitude: Some(19.7618466), min_longitude: Some(17.2472674)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Kujawië-Pommere"), ("ar", "محافظة كويافيا-بوميرانيا"), ("be", "Куяўска-Паморскае ваяводства"), ("bg", "Куявско-Поморско войводство"), ("bn", "ক\u{9c1}য\u{9bc}েভিয\u{9bc}\u{9be}ন-পোম\u{9be}র\u{9be}নিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Cuiàvia i Pomerània"), ("ccp", "𑄇\u{1112a}𑄠𑄞\u{11128}𑄠𑄬𑄚\u{11134}-𑄛\u{11127}𑄟𑄬𑄢𑄚\u{11128}𑄠"), ("cs", "Kujavsko-pomořské vojvodství"), ("da", "Województwo kujawsko-pomorskie"), ("de", "Woiwodschaft Kujawien-Pommern"), ("el", "Κουγιαβιανό-Πομεράνιο Βοϊβοδάτο"), ("en", "Kuyavia-Pomerania"), ("es", "Cuyavia y Pomerania"), ("et", "Kujawy-Pomorze vojevoodkond"), ("eu", "Kujavia-Pomeraniako voivoderria"), ("fa", "استان کویاوی-پومرانیا"), ("fi", "Kujavia-Pommerin voivodikunta"), ("fr", "Voïvodie de Couïavie-Poméranie"), ("gl", "Cuiavia-Pomerania"), ("gu", "ક\u{ac1}આવિયન-, પોમ\u{ac7}ર\u{ac7}નિયન , વોઈવોદ\u{ac7}શીપ"), ("he", "קויאוויה-פומרניה"), ("hi", "क\u{941}आवियन-पोमर\u{947}नियन वोईवोडशिप"), ("hr", "Kujavsko-pomeransko vojvodstvo"), ("hu", "Kujávia-Pomerániai vajdaság"), ("id", "Provinsi Kujawy-Pomorze"), ("is", "Kujavíska-Pommern"), ("it", "voivodato della Cuiavia-Pomerania"), ("ka", "კუიავო-პომერანიის სავოევოდო"), ("kn", "ಕ\u{ccd}ಯುವ\u{cbf}ಯನ\u{ccd}-ಪೊಮ\u{cc6}ರೇನ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "쿠야비포모제 주"), ("lt", "Kujavijos Pamario vaivadija"), ("lv", "Kujāvijas-Pomožes vojevodiste"), ("mr", "क\u{941}यास\u{94d}को-पोमोर\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kujawsko-Pomorskie"), ("nb", "Kujaviskpommerske voivodskap"), ("nl", "Koejavië-Pommeren"), ("no", "Kujaviskpommerske voivodskap"), ("pl", "województwo kujawsko-pomorskie"), ("pt", "Voivodia da Cujávia-Pomerânia"), ("ro", "Voievodatul Cuiavia și Pomerania"), ("ru", "Куявско-Поморское воеводство"), ("si", "ක\u{dd4}යව\u{dd2}යන\u{dca}-පොමරෙන\u{dd2}යන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Kujavsko-pomoranské vojvodstvo"), ("sl", "Kujavsko-Pomorjansko vojvodstvo"), ("sr", "Војводство Кујавско-Поморје"), ("sr_Latn", "Vojvodstvo Kujavsko-Pomorje"), ("sv", "Kujavien-Pommerns vojvodskap"), ("ta", "குய\u{bcd}ய\u{bbe}வின\u{bcd}-போமெரனின\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "కుయ\u{c3e}వ\u{c3f}యన\u{c4d}-ప\u{c4a}మర\u{c47}న\u{c3f}యన\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e39}ยาว\u{e37}อ-ปอมอแช"), ("tr", "Kuyavya-Pomeranya Voyvodalığı"), ("uk", "Куявсько-Поморське воєводство"), ("ur", "کویاوی-پومرانیا صوبہ"), ("vi", "Kujawsko-Pomorskie"), ("zh", "庫亞維-波美拉尼亞省")]),
                        unofficial_name_list: ["Kujawsko-pomorskie", "kujawsko-pomorskie"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::PL,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.2493519), longitude: Some(23.1011392), max_latitude: Some(52.2879201), min_latitude: Some(50.251603), max_longitude: Some(24.1458931), min_longitude: Some(21.6171249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Lublin"), ("ar", "محافظة لوبلين"), ("be", "Люблінскае ваяводства"), ("bg", "Люблинско войводство"), ("bn", "ল\u{9c1}ব\u{9cd}লিন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Lublin"), ("ccp", "𑄣\u{1112a}𑄛\u{11134}𑄣\u{11128}𑄚\u{11134}"), ("cs", "Lublinské vojvodství"), ("da", "Województwo lubelskie"), ("de", "Woiwodschaft Lublin"), ("el", "Λούμπλιν"), ("en", "Lublin"), ("es", "Lublin"), ("et", "Lublini vojevoodkond"), ("eu", "Lublingo voivoderria"), ("fa", "استان لوبلین"), ("fi", "Lublinin voivodikunta"), ("fr", "Voïvodie de Lublin"), ("gl", "Voivodato de Lublin"), ("gu", "લ\u{ac1}બ\u{acd}લિન વિઓવોદ\u{ac7}શીપ"), ("he", "לובלסקי"), ("hi", "ल\u{941}ब\u{94d}लिन"), ("hr", "Lublinsko vojvodstvo"), ("hu", "Lublini vajdaság"), ("hy", "Լյուբլինի վոևոդություն"), ("id", "Provinsi Lublin"), ("is", "Lublin"), ("it", "voivodato di Lublino"), ("ja", "ルブリン県"), ("ka", "ლუბლინის სავოევოდო"), ("kn", "ಲುಬ\u{ccd}ಲ\u{cbf}ನ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "루블린 주"), ("lt", "Liublino vaivadija"), ("lv", "Ļubļinas vojevodiste"), ("mr", "ल\u{941}ब\u{947}ल\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lublin"), ("nb", "Lublin voivodskap"), ("nl", "Lublin"), ("no", "Lublin voivodskap"), ("pl", "województwo lubelskie"), ("pt", "Voivodia de Lublin"), ("ro", "Voievodatul Lublin"), ("ru", "Люблинское воеводство"), ("si", "ලබ\u{dca}ල\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sk", "Lubelské vojvodstvo"), ("sr", "Војводство Лублин"), ("sr_Latn", "Vojvodstvo Lublin"), ("sv", "Lublins vojvodskap"), ("ta", "லுபிளின\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "లుబ\u{c4d}ల\u{c3f}న\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}บล\u{e34}น"), ("tr", "Lublin Voyvodalığı"), ("uk", "Люблінське воєводство"), ("ur", "لوبلین صوبہ"), ("vi", "Lubelskie"), ("zh", "卢布林省")]),
                        unofficial_name_list: ["Lubelskie", "lubelskie"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::PL,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.2274612), longitude: Some(15.2559103), max_latitude: Some(53.1239582), min_latitude: Some(51.363138), max_longitude: Some(16.4163811), min_longitude: Some(14.534127)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لوبوسكي"), ("be", "Любушскае ваяводства"), ("bg", "Любушко войводство"), ("bn", "ল\u{9c1}ব\u{9c1}জ ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Lubusz"), ("ccp", "𑄣𑄝\u{1112a}𑄌\u{11134}"), ("cs", "Lubušské vojvodství"), ("da", "Województwo lubuskie"), ("de", "Woiwodschaft Lebus"), ("el", "Λουμπούζ Βοϊβοδάτο"), ("en", "Lubusz"), ("es", "Voivodato de Lubusz"), ("et", "Lubuszi vojevoodkond"), ("eu", "Lubuszeko voivoderria"), ("fa", "استان لوبوسکی"), ("fi", "Lubuszin voivodikunta"), ("fr", "Voïvodie de Lubusz"), ("gl", "Voivodato de Lubusz"), ("gu", "લ\u{ac1}બ\u{ac1}ઝ વોઈવોડ\u{ac7}શિપ"), ("he", "לובוסקי"), ("hi", "ल\u{941}ब\u{941}स वोइवोडीशिप"), ("hr", "Lubusko vojvodstvo"), ("hu", "Lubusi vajdaság"), ("id", "Provinsi Lubusz"), ("is", "Lubusz"), ("it", "voivodato di Lubusz"), ("ja", "ルブシュ県"), ("ka", "ლუბუშის სავოევოდო"), ("kn", "ಲುಬಸ\u{ccd}ಜ\u{ccd} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "루부시 주"), ("lt", "Liubušo vaivadija"), ("lv", "Lubušas vojevodiste"), ("mr", "ल\u{941}ब\u{941}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lubusz"), ("nb", "Lubusz voivodskap"), ("nl", "Woiwodschap Lubusz"), ("no", "Lubusz voivodskap"), ("pl", "województwo lubuskie"), ("pt", "Voivodia da Lubúsquia"), ("ro", "Voievodatul Lubusz"), ("ru", "Любушское воеводство"), ("si", "ල\u{dd4}බ\u{dd4}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Lubuské vojvodstvo"), ("sl", "Lubuško vojvodstvo"), ("sr", "Војводство Лубуш"), ("sr_Latn", "Vojvodstvo Lubuš"), ("sv", "Lubusz vojvodskap"), ("ta", "லூபஸ\u{bcd}ஸ\u{bcd} ஓய\u{bcd}வொடேஷிப\u{bcd}"), ("te", "లూబస\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}บ\u{e38}ช"), ("tr", "Lubusz Voyvodalığı"), ("uk", "Любуське воєводство"), ("ur", "لوبوش صوبہ"), ("vi", "Lubuskie"), ("zh", "盧布斯卡省")]),
                        unofficial_name_list: ["Lubuskie", "lubuskie"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::PL,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4634771), longitude: Some(19.1726974), max_latitude: Some(52.3940561), min_latitude: Some(50.8430329), max_longitude: Some(20.6591903), min_longitude: Some(18.0750521)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وودج"), ("be", "Лодзьскае ваяводства"), ("bg", "Лодзко войводство"), ("bn", "লোজ ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Łódź"), ("ccp", "𑄣\u{11127}𑄖\u{11134}"), ("cs", "Lodžské vojvodství"), ("da", "Województwo łódzkie"), ("de", "Woiwodschaft Łódź"), ("el", "Λοτζ"), ("en", "Łódź"), ("es", "Voivodato de Łódź"), ("et", "Łódźi vojevoodkond"), ("eu", "Lodzeko voivoderria"), ("fa", "استان ووتسکی"), ("fi", "Łódźin voivodikunta"), ("fr", "Voïvodie de Łódź"), ("gu", "લોડ\u{acd}ઝ વોઈવોદ\u{ac7}શીપ"), ("he", "לודז׳"), ("hi", "लौड\u{94d}ज\u{93c} वोईवोडीशिप"), ("hr", "Vojvodstvo Lodz"), ("hu", "Łódźi vajdaság"), ("id", "Provinsi Łódź"), ("is", "Łódź"), ("it", "voivodato di Łódź"), ("ja", "ウッチ県"), ("ka", "ლოძის სავოევოდო"), ("kn", "ಲೋಡ\u{ccd}ಜ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "우치 주"), ("lt", "Lodzės vaivadija"), ("lv", "Lodzas vojevodiste"), ("mr", "व\u{942}त\u{94d}श\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Lodzkie"), ("nb", "Łódź voivodskap"), ("nl", "Woiwodschap Łódź"), ("no", "Łódź voivodskap"), ("pl", "województwo łódzkie"), ("pt", "Voivodia de Łódź"), ("ro", "Voievodatul Łódź"), ("ru", "Лодзинское воеводство"), ("si", "ලෝඩ\u{dca}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Lodžské vojvodstvo"), ("sr", "Војводство Лођ"), ("sr_Latn", "Vojvodstvo Lođ"), ("sv", "Łódź vojvodskap"), ("ta", "லொட\u{bcd}ஸ\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ల\u{c4b}డ\u{c4d}జ\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e39}ช"), ("tr", "Łódź Voyvodalığı"), ("uk", "Лодзинське воєводство"), ("ur", "ووچ صوبہ"), ("vi", "Łódzkie"), ("zh", "罗兹省")]),
                        unofficial_name_list: ["Łódzkie", "łódzkie"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::PL,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.72253060000001), longitude: Some(20.2503359), max_latitude: Some(50.5200442), min_latitude: Some(49.1785791), max_longitude: Some(21.4213826), min_longitude: Some(19.083192)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Klein-Pole"), ("ar", "محافظة بولندا الصغرى"), ("be", "Малапольскае ваяводства"), ("bg", "Малополско войводство"), ("bn", "লেস\u{9be}র পোল\u{9cd}য\u{9be}ন\u{9cd}ড ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Petita Polònia"), ("ccp", "𑄣𑄬𑄥𑄢\u{11134} 𑄛\u{1112e}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("cs", "Malopolské vojvodství"), ("da", "Województwo małopolskie"), ("de", "Woiwodschaft Kleinpolen"), ("el", "Μαλοπόλσκιε"), ("en", "Lesser Poland"), ("es", "Voivodato de Pequeña Polonia"), ("et", "Väike-Poola vojevoodkond"), ("eu", "Polonia Txikiko voivoderria"), ("fa", "استان لهستان کوچک\u{200c}تر"), ("fi", "Vähä-Puolan voivodikunta"), ("fr", "Voïvodie de Petite-Pologne"), ("gl", "Voivodato da Pequena Polonia"), ("gu", "લ\u{ac7}સર પોલ\u{ac7}ન\u{acd}ડ વોઈવાવોદ\u{ac7}શીપ"), ("he", "מלופולסקה"), ("hi", "लघ\u{941}तर पोल\u{948}\u{902}ड वोइवोडीशिप"), ("hr", "Malopoljsko vojvodstvo"), ("hu", "Kis-lengyelországi vajdaság"), ("id", "Provinsi Polandia Kecil"), ("is", "Litla-Pólland"), ("it", "voivodato della Piccola Polonia"), ("ja", "マウォポルスカ県"), ("ka", "მალოპოლსკის სავოევოდო"), ("kn", "ಕಡ\u{cbf}ಮ\u{cc6} ಪೋಲ\u{cc6}ಂಡ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "마워폴스카 주"), ("lt", "Mažosios Lenkijos vaivadija"), ("lv", "Mazpolijas vojevodiste"), ("mr", "मावोपोल\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Malopolskie"), ("nb", "Lillepolske voivodskap"), ("nl", "Woiwodschap Klein-Polen"), ("no", "Lillepolske voivodskap"), ("pl", "województwo małopolskie"), ("pt", "Voivodia da Pequena Polónia"), ("ro", "Polonia Mică"), ("ru", "Малопольское воеводство"), ("si", "ලෙසර\u{dca} පෝලන\u{dca}ත පළ\u{dcf}ත"), ("sk", "Malopoľské vojvodstvo"), ("sr", "Војводство Малопољско"), ("sr_Latn", "Vojvodstvo Malopoljsko"), ("sv", "Lillpolens vojvodskap"), ("ta", "லெஸ\u{bcd}ஸர\u{bcd} போலந\u{bcd}து ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ల\u{c46}స\u{c4d}సర\u{c4d} ప\u{c4b}లండ\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาวอปอลสกา"), ("tr", "Küçük Polonya Voyvodalığı"), ("uk", "Малопольське воєводство"), ("ur", "اصغر پولینڈ صوبہ"), ("vi", "Małopolskie"), ("zh", "小波兰省")]),
                        unofficial_name_list: ["Małopolskie", "małopolskie"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::PL,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.8927182), longitude: Some(21.0021679), max_latitude: Some(53.4818919), min_latitude: Some(51.0132082), max_longitude: Some(23.1283212), min_longitude: Some(19.2592569)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Masowië"), ("ar", "محافظة مازوفيا"), ("be", "Мазавецкае ваяводства"), ("bg", "Мазовско войводство"), ("bn", "মেসোভিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Masòvia"), ("ccp", "𑄟𑄎\u{1112e}𑄞\u{11128}𑄠"), ("cs", "Mazovské vojvodství"), ("da", "Województwo mazowieckie"), ("de", "Woiwodschaft Masowien"), ("el", "Μασοβία"), ("en", "Mazovia"), ("es", "Voivodato de Mazovia"), ("et", "Masoovia vojevoodkond"), ("eu", "Mazoviako voivoderria"), ("fa", "ماسوویان وویوودشیپ"), ("fi", "Masovian voivodikunta"), ("fr", "Voïvodie de Mazovie"), ("gu", "મ\u{ac7}સોવિઆન વોઈવોદ\u{ac7}શીપ"), ("he", "מזוביה"), ("hi", "मासोवियन वोईवोडीशिप"), ("hr", "Mazovjecko vojvodstvo"), ("hu", "Mazóviai vajdaság"), ("hy", "Մազովեցի վոյեվոդություն"), ("id", "Provinsi Mazowsze"), ("is", "Masóvía"), ("it", "voivodato della Masovia"), ("ja", "マゾフシェ県"), ("ka", "მაზოვიეცის სავოევოდო"), ("kn", "ಮಾವೋವ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "마조프셰 주"), ("lt", "Mazovijos vaivadija"), ("lv", "Mazovijas vojevodiste"), ("mr", "माझोव\u{94d}य\u{947}त\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Mazowieckie"), ("nb", "Masoviske voivodskap"), ("nl", "Woiwodschap Mazovië"), ("no", "Masoviske voivodskap"), ("pl", "województwo mazowieckie"), ("pt", "Voivodia da Mazóvia"), ("ro", "Mazovia"), ("ru", "Мазовецкое воеводство"), ("si", "මැසොව\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Mazovské vojvodstvo"), ("sr", "Војводство Мазовско"), ("sr_Latn", "Vojvodstvo Mazovsko"), ("sv", "Masoviens vojvodskap"), ("ta", "மசோவின\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "మ\u{c3e}స\u{c4b}వ\u{c3f}యన\u{c4d} వ\u{c3e}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาซอฟแช"), ("tr", "Mazovya Voyvodalığı"), ("uk", "Мазовецьке воєводство"), ("ur", "صوبہ ماسووی"), ("vi", "Mazowieckie"), ("zh", "馬佐夫舍省")]),
                        unofficial_name_list: ["Mazowieckie", "mazowieckie"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::PL,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.8003761), longitude: Some(17.937989), max_latitude: Some(51.1945111), min_latitude: Some(49.9725265), max_longitude: Some(18.6957862), min_longitude: Some(16.9087264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أوبولي"), ("be", "Апольскае ваяводства"), ("bg", "Ополско войводство"), ("bn", "ওপোল ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat d’Opole"), ("ccp", "𑄃\u{1112e}𑄛\u{1112e}𑄣\u{11134}"), ("cs", "Opolské vojvodství"), ("da", "Województwo opolskie"), ("de", "Woiwodschaft Opole"), ("el", "Βοϊβοδάτο Οπόλε"), ("en", "Opole"), ("es", "Opole"), ("et", "Opole vojevoodkond"), ("eu", "Opoleko voivoderria"), ("fa", "استان اوپوله"), ("fi", "Opolen voivodikunta"), ("fr", "Voïvodie d’Opole"), ("gl", "Opole"), ("gu", "ઑપોલ વોઈવોડ\u{ac7}શીપ"), ("he", "אופולסקי"), ("hi", "ऑपोल"), ("hr", "Opolsko vojvodstvo"), ("hu", "Opolei vajdaság"), ("id", "Provinsi Opole"), ("is", "Opole"), ("it", "voivodato di Opole"), ("ja", "オポーレ県"), ("ka", "ოპოლეს სავოევოდო"), ("kn", "ಒಪೋಲ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "오폴레 주"), ("lt", "Opolės vaivadija"), ("lv", "Opoles vojevodiste"), ("mr", "ओपोल\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Opole"), ("nb", "Opole voivodskap"), ("nl", "Opole"), ("no", "Opole voivodskap"), ("pl", "województwo opolskie"), ("pt", "Voivodia de Opole"), ("ro", "Voievodatul Opole"), ("ru", "Опольское воеводство"), ("si", "ඔපොලේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Opolské vojvodstvo"), ("sr", "Војводство Опоље"), ("sr_Latn", "Vojvodstvo Opolje"), ("sv", "Opole vojvodskap"), ("ta", "ஓபோலே ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ఓప\u{c4b}ల\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดออปอแล"), ("tr", "Opole Voyvodalığı"), ("uk", "Опольське воєводство"), ("ur", "اوپولے صوبہ"), ("vi", "Opolskie"), ("zh", "奧波萊省")]),
                        unofficial_name_list: ["Opolskie", "opolskie"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::PL,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.0574749), longitude: Some(22.0895691), max_latitude: Some(50.8181161), min_latitude: Some(49.0020252), max_longitude: Some(23.5476409), min_longitude: Some(21.1423457)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Subkarpate"), ("ar", "محافظة بودكارباتسكي"), ("be", "Падкарпацкае ваяводства"), ("bg", "Подкарпатско войводство"), ("bn", "পোডক\u{9be}রপ\u{9cd}য\u{9be}কি ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Subcarpàcia"), ("ccp", "𑄥𑄛\u{11134}𑄇𑄢\u{11134}𑄛𑄗\u{11128}𑄠"), ("cs", "Podkarpatské vojvodství"), ("da", "Województwo podkarpackie"), ("de", "Woiwodschaft Karpatenvorland"), ("el", "Ποντκαρπάκιε"), ("en", "Subcarpathia"), ("es", "Voivodato de Subcarpacia"), ("et", "Podkarpacie vojevoodkond"), ("eu", "Behe Karpatoetako voivoderria"), ("fa", "استان پودکارپاکیه"), ("fi", "Ala-Karpatian voivodikunta"), ("fr", "Voïvodie des Basses-Carpates"), ("gu", "પોડકારપ\u{ac7}કી વોઈવોડ\u{ac7}શીપ"), ("he", "פודקרפאטי"), ("hi", "पॉडकारप\u{948}की वोइवोडीशिप"), ("hr", "Potkarpatsko vojvodstvo"), ("hu", "Kárpátaljai vajdaság"), ("id", "Provinsi Podkarpacie"), ("is", "Neðri-Karpatía"), ("it", "voivodato della Precarpazia"), ("ja", "ポトカルパチェ県"), ("ka", "კარპატების სავოევოდო"), ("kn", "ಪೋಡ\u{ccd}ಕಾರ\u{ccd}ಪಕ\u{ccd}ಕ\u{cbf} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "포드카르파츠키에 주"), ("lt", "Pakarpatės vaivadija"), ("lv", "Piekarpatu vojevodiste"), ("mr", "पोट\u{94d}कर\u{94d}पाट\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Podkarpacie"), ("nb", "Subkarpatiske voivodskap"), ("nl", "Woiwodschap Subkarpaten"), ("no", "Subkarpatiske voivodskap"), ("pl", "województwo podkarpackie"), ("pt", "Voivodia da Subcarpácia"), ("ro", "Voievodatul Subcarpatia"), ("ru", "Подкарпатское воеводство"), ("si", "පොඩ\u{dca}කර\u{dca}පැක\u{dd2} පළ\u{dcf}ත"), ("sk", "Podkarpatské vojvodstvo"), ("sl", "Podkarpatsko vojvodstvo"), ("sr", "Војводство поткарпатско"), ("sr_Latn", "Vojvodstvo potkarpatsko"), ("sv", "Nedre Karpaternas vojvodskap"), ("ta", "போத\u{bcd}கர\u{bcd}ப\u{bcd}ப\u{bbe}க\u{bcd}கியே ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ప\u{c4b}డ\u{c4d}క\u{c3e}ర\u{c4d}ప\u{c3e}క\u{c40} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปอตการ\u{e4c}ปาแช"), ("tr", "Alt Karpatya Voyvodalığı"), ("uk", "Підкарпатське воєводство"), ("ur", "پودکرپاسکیہ صوبہ"), ("vi", "Podkarpackie"), ("zh", "喀尔巴阡山省")]),
                        unofficial_name_list: ["Podkarpackie", "podkarpackie"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::PL,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.0697159), longitude: Some(22.9674639), max_latitude: Some(54.409889), min_latitude: Some(52.2800528), max_longitude: Some(23.9462697), min_longitude: Some(21.5942443)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Podlachië"), ("ar", "محافظة بودلاسكي"), ("be", "Падляскае ваяводства"), ("bg", "Подляско войводство"), ("bn", "পোদল\u{9be}স\u{9cd}কি ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Podlàquia"), ("ccp", "𑄛𑄖\u{11134}𑄣𑄌\u{11134}𑄇\u{11128}"), ("cs", "Podleské vojvodství"), ("da", "Województwo podlaskie"), ("de", "Woiwodschaft Podlachien"), ("el", "Ποντλάσκιε"), ("en", "Podlachia"), ("es", "Voivodato de Podlaquia"), ("et", "Podlaasia vojevoodkond"), ("eu", "Podlasiako voivoderria"), ("fa", "استان پودلاسکی"), ("fi", "Podlasian voivodikunta"), ("fr", "Voïvodie de Podlachie"), ("gu", "પોડ\u{acd}લાસ\u{acd}કી વોઇવોદ\u{ac7}શીપ"), ("he", "פודלסיה"), ("hi", "पॉडलास\u{94d}की वोईवोडीशिप"), ("hr", "Podlasko vojvodstvo"), ("hu", "Podlasiei vajdaság"), ("id", "Provinsi Podlasie"), ("is", "Podlasía"), ("it", "voivodato della Podlachia"), ("ja", "ポドラシェ県"), ("ka", "პოდლასის სავოევოდო"), ("kn", "ಪೊಡ\u{ccd}ಲಾಸ\u{ccd}ಕ\u{cbf} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "포들라스키에 주"), ("lt", "Palenkės vaivadija"), ("lv", "Podlases vojevodiste"), ("mr", "पोडाल\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Podlasie"), ("nb", "Podlasie voivodskap"), ("nl", "Woiwodschap Podlachië"), ("no", "Podlasie voivodskap"), ("pl", "województwo podlaskie"), ("pt", "Voivodia da Podláquia"), ("ro", "Voievodatul Podlasia"), ("ru", "Подляское воеводство"), ("si", "පොඩ\u{dca}ලස\u{dca}ක\u{dd2} පළ\u{dcf}ත"), ("sk", "Podleské vojvodstvo"), ("sl", "Podlaško vojvodstvo"), ("sr", "Војводство Подласко"), ("sr_Latn", "Vojvodstvo Podlasko"), ("sv", "Podlasiens vojvodskap"), ("ta", "போட\u{bcd}லஸ\u{bcd}கியே ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ప\u{c4b}డ\u{c4d}ల\u{c3e}స\u{c4d}క\u{c40} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปอดลาแช"), ("tr", "Podlakya Voyvodalığı"), ("uk", "Підляське воєводство"), ("ur", "پودلاسکیہ صوبہ"), ("vi", "Podlaskie"), ("zh", "波德拉謝省")]),
                        unofficial_name_list: ["Podlaskie", "podlaskie"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::PL,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(54.2944252), longitude: Some(18.1531164), max_latitude: Some(54.83572969999999), min_latitude: Some(53.49067179999999), max_longitude: Some(19.6493699), min_longitude: Some(16.699129)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Pommere"), ("ar", "محافظة بوميرانيا"), ("be", "Паморскае ваяводства"), ("bg", "Поморско войводство"), ("bn", "পোমের\u{9be}নিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Pomerània"), ("ccp", "𑄜𑄬𑄓𑄢𑄬𑄣\u{11134} 𑄇\u{11133}𑄠𑄛\u{11128}𑄑𑄣\u{11134}𑄑𑄬𑄢\u{11128}𑄑\u{11127}𑄢\u{11128}"), ("cs", "Pomořské vojvodství"), ("da", "Województwo pomorskie"), ("de", "Woiwodschaft Pommern"), ("el", "Πομερανία"), ("en", "Pomerania"), ("es", "Voivodato de Pomerania"), ("et", "Pomorze vojevoodkond"), ("eu", "Pomeraniako voivoderria"), ("fa", "استان پومرانی"), ("fi", "Pommerin voivodikunta"), ("fr", "Voïvodie de Poméranie"), ("gu", "પોમ\u{ac7}ર\u{ac7}નિયન વોઈવોડ\u{ac7}શીપ"), ("he", "פומורסקיה"), ("hi", "पोमर\u{947}नियन वोइवोडीशिप"), ("hr", "Pomeransko vojvodstvo"), ("hu", "Pomerániai vajdaság"), ("id", "Provinsi Pomerania"), ("is", "Pommern (hérað)"), ("it", "voivodato della Pomerania"), ("ja", "ポモージェ県"), ("ka", "პომერანიის სავოევოდო"), ("kn", "ಪೋಮರೇನ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "포모제 주"), ("lt", "Pamario vaivadija"), ("lv", "Pomožes vojevodiste"), ("mr", "पोमोर\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pomerania"), ("nb", "Pommerske voivodskap"), ("nl", "Woiwodschap Pommeren"), ("no", "Pommerske voivodskap"), ("pl", "województwo pomorskie"), ("pt", "Voivodia da Pomerânia"), ("ro", "Voievodatul Pomerania"), ("ru", "Поморское воеводство"), ("si", "පොමරෙන\u{dd2}යන\u{dca} පළ\u{dcf}ත"), ("sk", "Pomoranské vojvodstvo"), ("sr", "Војводство Поморје"), ("sr_Latn", "Vojvodstvo Pomorje"), ("sv", "Pommerns vojvodskap"), ("ta", "போமெரனின\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "ప\u{c4b}మర\u{c47}న\u{c3f}యన\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปอมอแช"), ("tr", "Pomeranya Voyvodalığı"), ("uk", "Поморське воєводство"), ("ur", "پومرانیا صوبہ"), ("vi", "Pomorskie"), ("zh", "濱海省")]),
                        unofficial_name_list: ["Pomorskie", "pomorskie"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::PL,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.5716595), longitude: Some(19.3219768), max_latitude: Some(51.0993559), min_latitude: Some(49.393975), max_longitude: Some(19.9739915), min_longitude: Some(18.03475)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيليزيا"), ("be", "Сілезскае ваяводства"), ("bg", "Силезко войводство"), ("bn", "সিলেসিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Silèsia"), ("ccp", "𑄥\u{1112d}𑄣𑄬𑄥\u{11128}𑄠"), ("cs", "Slezské vojvodství"), ("da", "Województwo śląskie"), ("de", "Woiwodschaft Schlesien"), ("el", "Σιλεσία"), ("en", "Silesia"), ("es", "Voivodato de Silesia"), ("et", "Sileesia vojevoodkond"), ("eu", "Silesiako voivoderria"), ("fa", "استان سیلسیان"), ("fi", "Sleesian voivodikunta"), ("fr", "Voïvodie de Silésie"), ("gl", "Voivodato de Silesia"), ("gu", "સિલ\u{ac7}સિઅન વોઈવોડ\u{ac7}શિપ"), ("he", "שלזיה"), ("hi", "सिल\u{947}सियन वोईवोडीशिप"), ("hr", "Šlesko vojvodstvo"), ("hu", "Sziléziai vajdaság"), ("id", "Provinsi Silesia"), ("is", "Slesía"), ("it", "voivodato della Slesia"), ("ja", "シロンスク県"), ("ka", "სილეზიის სავოევოდო"), ("kn", "ಸೈಲ\u{ccd}ಸ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "실롱스크 주"), ("lt", "Silezijos vaivadija"), ("lv", "Silēzijas vojevodiste"), ("mr", "श\u{94d}लो\u{902}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Silesia"), ("nb", "Schlesiske voivodskap"), ("nl", "Woiwodschap Silezië"), ("no", "Schlesiske voivodskap"), ("pl", "województwo śląskie"), ("pt", "Voivodia da Silésia"), ("ro", "Voievodatul Silezia"), ("ru", "Силезское воеводство"), ("si", "සෙලෙස\u{dd2}යන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sliezske vojvodstvo"), ("sr", "Војводство Шлеско"), ("sr_Latn", "Vojvodstvo Šlesko"), ("sv", "Schlesiens vojvodskap"), ("ta", "சிலேசிய\u{bbe}ன\u{bcd} ஓய\u{bcd}வொதேஷிப\u{bcd}"), ("te", "స\u{c3f}ల\u{c47}స\u{c3f}యన\u{c4d} వ\u{c4b}యవ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชล\u{e47}อนสก\u{e4c}"), ("tr", "Silezya Voyvodalığı"), ("uk", "Сілезьке воєводство"), ("ur", "سیلیزیا صوبہ"), ("vi", "Śląskie"), ("zh", "西里西亚省")]),
                        unofficial_name_list: ["Śląskie", "śląskie"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::PL,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.6261041), longitude: Some(20.9406279), max_latitude: Some(51.3421558), min_latitude: Some(50.1855379), max_longitude: Some(21.8703061), min_longitude: Some(19.7044002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة شفينتوكشيسكي"), ("be", "Свентакшыскае ваяводства"), ("bg", "Швентокшиско войводство"), ("bn", "স\u{9c1}ইটোকিজিস\u{9cd}কি ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Santa Creu"), ("ccp", "𑄥\u{1112a}𑄃\u{11128}𑄑\u{1112e}𑄇\u{11133}𑄢\u{1112d}𑄃\u{11128}𑄌\u{11134}𑄇\u{11128}"), ("cs", "Svatokřížské vojvodství"), ("da", "Województwo świętokrzyskie"), ("de", "Woiwodschaft Heiligkreuz"), ("el", "βοϊβοδάτο Σβιετοκρίσκιε"), ("en", "Holy Cross"), ("es", "Voivodato de Santa Cruz"), ("et", "Święty Krzyżi vojevoodkond"), ("eu", "Gurutze Santuko voivoderria"), ("fa", "استان اشوی\u{200c}داشکسیه"), ("fi", "Świętokrzyskin voivodikunta"), ("fr", "Voïvodie de Sainte-Croix"), ("gu", "સ\u{acd}વ\u{ac7}તોક\u{acd}ર\u{acd}ઝીસ\u{acd}કી"), ("he", "שוויינטוקז׳יסקיה"), ("hi", "स\u{94d}व\u{947}तोक\u{94d}रझिस\u{94d}की वोइवोडीशिप"), ("hr", "Svetokriško vojvodstvo"), ("hu", "Szentkereszt vajdaság"), ("id", "Provinsi Święty Krzyż"), ("is", "Święty Krzyż"), ("it", "voivodato della Santacroce"), ("ja", "シフィェンティクシシュ県"), ("ka", "სვიეტოკშის სავოევოდო"), ("kn", "ಶ\u{ccd}ವೇಟೋಕ\u{ccd}ರೋಜ\u{ccd}ಕ\u{cbf} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "시비엥토크시스키에 주"), ("lt", "Švento Kryžiaus vaivadija"), ("lv", "Sventokšiskas vojevodiste"), ("mr", "श\u{94d}व\u{947}\u{902}तोकशिस\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Święty Krzyż"), ("nb", "Helligkorsvoivodskapet"), ("nl", "Święty Krzyż"), ("no", "Helligkorsvoivodskapet"), ("pl", "województwo świętokrzyskie"), ("pt", "Voivodia de Santa Cruz"), ("ro", "Voievodatul Sfintei Cruci"), ("ru", "Свентокшиское воеводство"), ("si", "ස\u{dca}වය\u{dd2}ටොක\u{dca}ර\u{dca}සය\u{dd2}ක\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Svätokrížske vojvodstvo"), ("sr", "Војводство Светокришко"), ("sr_Latn", "Vojvodstvo Svetokriško"), ("sv", "Święty Krzyż vojvodskap"), ("ta", "ஸ\u{bcd}விட\u{bcd}ஓக\u{bcd}கறயசகியே ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "స\u{c4d}వయ\u{c3f}ట\u{c4b}క\u{c4b}జ\u{c3f}స\u{c4d}క\u{c40} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชฟ\u{e35}แยนต\u{e37}อ-กช\u{e36}ช"), ("tr", "Świętokrzyskie Voyvodalığı"), ("uk", "Свентокшиське воєводство"), ("ur", "شوئینتوشوسکیہ صوبہ"), ("vi", "Świętokrzyskie"), ("zh", "聖十字省")]),
                        unofficial_name_list: ["Świętokrzyskie", "świętokrzyskie"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::PL,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.8671117), longitude: Some(20.702786), max_latitude: Some(54.4533097), min_latitude: Some(53.1399971), max_longitude: Some(22.8058724), min_longitude: Some(19.128516)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فارميا-مازوريا"), ("be", "Вармінска-Мазурскае ваяводства"), ("bg", "Варминско-Мазурско войводство"), ("bn", "ওয\u{9bc}\u{9be}রমিয\u{9bc}\u{9be}-ম\u{9be}স\u{9c1}রিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Warmia i Mazury"), ("ccp", "𑄤𑄢\u{11134}𑄟\u{11128}𑄠𑄚\u{11134}-𑄟𑄥\u{1112a}𑄢\u{11128}𑄠"), ("cs", "Varmijsko-mazurské vojvodství"), ("da", "Województwo warmińsko-mazurskie"), ("de", "Woiwodschaft Ermland-Masuren"), ("el", "βοϊβοδάτο Βαρμίας-Μαζουρίας"), ("en", "Warmia-Masuria"), ("es", "Voivodato de Varmia y Masuria"), ("et", "Warmia-Masuuria vojevoodkond"), ("eu", "Varmia-Mazuriako voivoderria"), ("fa", "استان وارمی-ماسوری"), ("fi", "Warmia-Masurian voivodikunta"), ("fr", "Voïvodie de Varmie-Mazurie"), ("gl", "Varmia e Masuria"), ("gu", "વાર\u{acd}મિયન-મસ\u{ac2}રિયન વોઈવોડ\u{ac7}શિપ"), ("he", "ורמיה-מזוריה"), ("hi", "वार\u{94d}मियन-मस\u{941}रियन वाइवोडीशिप"), ("hr", "Varminsko-mazursko vojvodstvo"), ("hu", "Varmia-Mazúriai vajdaság"), ("id", "Provinsi Warmia-Mazury"), ("is", "Ermland-Masúría"), ("it", "voivodato della Varmia-Masuria"), ("ka", "ვარმინო-მაზურის სავოევოდო"), ("kn", "ವಾರ\u{ccd}ಮ\u{cbf}ಯನ\u{ccd}-ಮಸ\u{ccd}ಯರ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "바르미아마주리 주"), ("lt", "Varmijos Mozūrų vaivadija"), ("lv", "Varmijas-Mazūrijas vojevodiste"), ("mr", "वार\u{94d}मिन\u{94d}स\u{94d}को-माझ\u{941}र\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Warminsko-Mazurskie"), ("nb", "Ermlandskmasuriske voivodskap"), ("nl", "Ermland-Mazurië"), ("no", "Ermlandskmasuriske voivodskap"), ("pl", "województwo warmińsko-mazurskie"), ("pt", "Voivodia da Vármia-Masúria"), ("ro", "Voievodatul Varmia și Mazuria"), ("ru", "Варминско-Мазурское воеводство"), ("si", "ව\u{dcf}ර\u{dca}ම\u{dd2}යන\u{dca}මස\u{dd4}ර\u{dd2}යන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Varmsko-mazurské vojvodstvo"), ("sr", "Варминско-мазурско војводство"), ("sr_Latn", "Varminsko-mazursko vojvodstvo"), ("sv", "Ermland-Masuriens vojvodskap"), ("ta", "வ\u{bbe}ர\u{bcd}மிங\u{bcd} -மைசூரின\u{bcd} ஓய\u{bcd}வொடேஷிப\u{bcd}"), ("te", "వ\u{c3e}ర\u{c4d}మ\u{c3f}మ\u{c3e}యన\u{c4d}-మ\u{c3e}సూర\u{c3f}యన\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวาร\u{e4c}เม\u{e35}ย-มาซ\u{e39}ร\u{e37}อ"), ("tr", "Varmiya-Mazurya Voyvodalığı"), ("uk", "Вармінсько-Мазурське воєводство"), ("ur", "وارمیا-ماسوریا صوبہ"), ("vi", "Warmińsko-Mazurskie"), ("zh", "瓦爾米亞-馬祖里省")]),
                        unofficial_name_list: ["Warmińsko-mazurskie", "warmińsko-mazurskie"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::PL,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.279986), longitude: Some(17.3522939), max_latitude: Some(53.6559175), min_latitude: Some(51.1037882), max_longitude: Some(19.103349), min_longitude: Some(15.7789647)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Groot-Pole"), ("ar", "بولندا الكبرى"), ("be", "Велікапольскае ваяводства"), ("bg", "Великополско войводство"), ("bn", "গ\u{9cd}রেট\u{9be}র পোল\u{9cd}য\u{9be}ন\u{9cd}ড ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Gran Polònia"), ("ccp", "𑄉\u{11133}𑄢𑄬𑄑𑄢\u{11134} 𑄛\u{1112e}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("cs", "Velkopolské vojvodství"), ("da", "Województwo wielkopolskie"), ("de", "Woiwodschaft Großpolen"), ("de_CH", "Woiwodschaft Grosspolen"), ("el", "Ευρύτερη Πολωνία"), ("en", "Greater Poland"), ("es", "Voivodato de Gran Polonia"), ("et", "Suur-Poola vojevoodkond"), ("eu", "Polonia Handiko voivoderria"), ("fa", "استان لهستان بزرگ\u{200c}تر"), ("fi", "Ison-Puolan voivodikunta"), ("fr", "Voïvodie de Grande-Pologne"), ("gl", "Voivodato de Gran Polonia"), ("gu", "ગ\u{acd}ર\u{ac7}ટર પોલ\u{ac7}ન\u{acd}ડ વીઓવોડ\u{ac7}શિપ"), ("he", "פולין גדול"), ("hi", "ब\u{943}हत\u{94d}तर पोल\u{948}\u{902}ड वोईवोडीशिप"), ("hr", "Velikopoljsko vojvodstvo"), ("hu", "Nagy-Lengyelországi vajdaság"), ("id", "Provinsi Polandia Besar"), ("is", "Stóra-Pólland"), ("it", "voivodato della Grande Polonia"), ("ja", "ヴィエルコポルスカ県"), ("ka", "ველიკოპოლსკის სავოევოდო"), ("kn", "ಗ\u{ccd}ರೇಟರ\u{ccd} ಪೋಲ\u{cc6}ಂಡ\u{ccd} ವಾಯ\u{cbf}ವೊಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "비엘코폴스카 주"), ("lt", "Didžiosios Lenkijos vaivadija"), ("lv", "Lielpolijas vojevodiste"), ("mn", "Их Польш"), ("mr", "व\u{94d}यील\u{94d}कोपाल\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Wielkopolskie"), ("nb", "Storpolske voivodskap"), ("nl", "Woiwodschap Groot-Polen"), ("no", "Storpolske voivodskap"), ("pl", "województwo wielkopolskie"), ("pt", "Voivodia da Grande Polônia"), ("ro", "Voievodatul Polonia Mare"), ("ru", "Великопольское воеводство"), ("si", "මහ\u{dcf} පෝලන\u{dca}ත පළ\u{dcf}ත"), ("sk", "Veľkopoľské vojvodstvo"), ("sr", "Војводство Великопољско"), ("sr_Latn", "Vojvodstvo Velikopoljsko"), ("sv", "Storpolens vojvodskap"), ("ta", "கிரேட\u{bcd}டர\u{bcd} பொலன\u{bcd}ட\u{bcd} ஓய\u{bcd}வொதேஷிப\u{bcd}"), ("te", "గ\u{c4d}ర\u{c4b}టర\u{c4d} ప\u{c4b}లండ\u{c4d} వ\u{c3e}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e35}แยลกอปอลสกา"), ("tr", "Büyük Polonya Voyvodalığı"), ("uk", "Великопольське воєводство"), ("ur", "اکبر پولینڈ صوبہ"), ("vi", "Wielkopolskie"), ("zh", "大波兰省")]),
                        unofficial_name_list: ["Wielkopolskie", "wielkopolskie"].to_vec(),
                    }
                ),
                (
                    "32",
                    Subdivision{
                        name: "32",
                        country_alpha2: Alpha2::PL,
                        code: "32",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.46578909999999), longitude: Some(15.1822581), max_latitude: Some(54.5690916), min_latitude: Some(52.62466939999999), max_longitude: Some(16.9822089), min_longitude: Some(14.1223531)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Voivodeship,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Woiwodskap Wes-Pommere"), ("ar", "محافظة غرب بوميرانيا"), ("be", "Заходнепаморскае ваяводства"), ("bg", "Западнопоморско войводство"), ("bn", "পশ\u{9cd}চিম পেম\u{9be}র\u{9be}নিয\u{9bc}\u{9be}ন ভয\u{9bc}ভোডেশিপ"), ("ca", "Voivodat de Pomerània Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄛\u{11127}𑄟𑄬𑄢𑄚\u{11128}𑄠"), ("cs", "Západopomořanské vojvodství"), ("da", "Województwo zachodniopomorskie"), ("de", "Woiwodschaft Westpommern"), ("el", "Δυτική Πομερανία"), ("en", "West Pomerania"), ("es", "Voivodato de Pomerania Occidental"), ("et", "Lääne-Pomorze vojevoodkond"), ("eu", "Mendebaldeko Pomeraniako voivoderria"), ("fa", "استان پومرانی غربی"), ("fi", "Länsi-Pommerin voivodikunta"), ("fr", "Voïvodie de Poméranie occidentale"), ("gl", "Voivodato de Pomerania Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ પોમ\u{ac7}ર\u{ac7}નિયન વીઓવોડ\u{ac7}શીપ"), ("he", "פומרניה המערבית"), ("hi", "पश\u{94d}चिम पोम\u{947}रर\u{947}नियन वोइवोडीशिप"), ("hr", "Zapadnopomeransko vojvodstvo"), ("hu", "Nyugat-Pomerániai vajdaság"), ("hy", "Արևմտյան Պոմորիեի վոեվոդություն"), ("id", "Provinsi Pomerania Barat"), ("is", "Vestur-Pommern"), ("it", "voivodato della Pomerania Occidentale"), ("ja", "西ポモージェ県"), ("ka", "დასავლეთ პომერანიის სავოევოდო"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಪೊಮ\u{cc6}ರನ\u{cbf}ಯನ\u{ccd} ವಾಯ\u{cbf}ವೋಡ\u{cc6}ಶ\u{cbf}ಪ\u{ccd}"), ("ko", "서포모제 주"), ("lt", "Vakarų Pamario vaivadija"), ("lv", "Rietumpomožes vojevodiste"), ("mn", "Өрнөд Померан"), ("mr", "झाखोज\u{94d}ञोपोमोर\u{94d}स\u{94d}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Barat Pomerania"), ("nb", "Vestpommerske voivodskap"), ("nl", "Woiwodschap West-Pommeren"), ("no", "Vestpommerske voivodskap"), ("pl", "województwo zachodniopomorskie"), ("pt", "Voivodia da Pomerânia Ocidental"), ("ro", "Pomerania Occidentală"), ("ru", "Западно-Поморское воеводство"), ("si", "බටහ\u{dd2}ර පොමරෙන\u{dd2}යන\u{dca}"), ("sk", "Západopomoranské vojvodstvo"), ("sl", "Zahodnopomorjansko vojvodstvo"), ("sr", "Војводство Западно Поморје"), ("sr_Latn", "Vojvodstvo Zapadno Pomorje"), ("sv", "Västpommerns vojvodskap"), ("ta", "மேற\u{bcd}கு போமெரனின\u{bcd} ஓய\u{bcd}வோடேஷிப\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ ప\u{c4a}మ\u{c46}ర\u{c47}న\u{c3f}యన\u{c4d} వ\u{c4b}య\u{c3f}వ\u{c4b}డ\u{c46}ష\u{c3f}ప\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปอมอแชซาคอดแญ"), ("tr", "Batı Pomeranya Voyvodalığı"), ("uk", "Західнопоморське воєводство"), ("ur", "مغربی پومرانیا صوبہ"), ("vi", "Zachodniopomorskie"), ("zh", "西波美拉尼亚省")]),
                        unofficial_name_list: ["Zachodniopomorskie", "zachodniopomorskie"].to_vec(),
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
#[cfg(feature = "pl")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PL,
        alpha3: Alpha3::POL,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 48,
        currency_code: "PLN",
        gec: Some(GEC::PL),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::POL),
        iso_long_name: "The Republic of Poland",
        iso_short_name: "Poland",
        official_language_list: ["pl"].to_vec(),
        spoken_language_list: ["pl"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Polish"),
        number: "616",
        postal_code: true,
        postal_code_format: Some("\\d{2}-\\d{3}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "PL",
        unofficial_name_list: ["Poland", "Polen", "Pologne", "Polonia", "ポーランド"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Poland"),
            ("af", "Pole"),
            ("ak", "Poland"),
            ("am", "ፖሒን፥"),
            ("an", "Poland"),
            ("ar", "بولندا"),
            ("as", "পলেণ\u{9cd}ড"),
            ("ay", "Poland"),
            ("az", "Polşa"),
            ("ba", "Poland"),
            ("be", "Польшча"),
            ("bg", "Полша"),
            ("bi", "Poland"),
            ("bn", "পোল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("bn_IN", "পোল\u{9cd}য\u{9be}ন\u{9cd}ড"),
            ("br", "Polonia"),
            ("bs", "Poljska"),
            ("ca", "Polònia"),
            ("ce", "Польша"),
            ("ch", "Polaki"),
            ("cs", "Polsko"),
            ("cv", "Польша"),
            ("cy", "Gwlad Pwyl"),
            ("da", "Polen"),
            ("de", "Polen"),
            ("dv", "ޕ\u{7ae}ލ\u{7ac}ނ\u{7b0}ޑ\u{7aa}"),
            ("dz", "པ\u{f7c}་ལ\u{f7a}ནཌ\u{f72}།"),
            ("ee", "Poland"),
            ("el", "Πολωνία"),
            ("en", "Poland"),
            ("eo", "Pollando"),
            ("es", "Polonia"),
            ("et", "Poola"),
            ("eu", "Polonia"),
            ("fa", "لهستان"),
            ("ff", "Poloonya"),
            ("fi", "Puola"),
            ("fo", "Pólland"),
            ("fr", "Pologne"),
            ("fy", "Poalen"),
            ("ga", "An Pholainn"),
            ("gl", "Polonia"),
            ("gn", "Poland"),
            ("gu", "પોલ\u{ac7}ન\u{acd}ડ"),
            ("gv", "Yn Pholynn"),
            ("ha", "Poland"),
            ("he", "פולין"),
            ("hi", "पोल\u{948}\u{902}ड"),
            ("hr", "Poljska"),
            ("ht", "Polòy"),
            ("hu", "Lengyelország"),
            ("hy", "Լեհաստան"),
            ("ia", "Polonia"),
            ("id", "Polandia"),
            ("io", "Polonia"),
            ("is", "Pólland"),
            ("it", "Polonia"),
            ("iu", "ᐳᓚᓐᑦ"),
            ("ja", "ポーランド"),
            ("ka", "პოლონეთი"),
            ("ki", "Poland"),
            ("kk", "Польша"),
            ("kl", "Poland"),
            ("km", "ប\u{17c9}\u{17bc}ឡ\u{17bc}ញ"),
            ("kn", "ಪೋಲಂಡ\u{ccd}"),
            ("ko", "폴란드"),
            ("ku", "Polonya"),
            ("kv", "Польша"),
            ("kw", "Poloni"),
            ("ky", "Польша"),
            ("lo", "ປະເທດໂປໂລຍ"),
            ("lt", "Lenkija"),
            ("lv", "Polija"),
            ("mi", "Pōrana"),
            ("mk", "Полска"),
            ("ml", "പോളണ\u{d4d}ട\u{d4d}"),
            ("mn", "Польш"),
            ("mr", "पोल\u{902}ड"),
            ("ms", "Poland"),
            ("mt", "Polonja"),
            (
                "my",
                "ပ\u{102d}\u{102f}လန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Poran"),
            ("nb", "Polen"),
            ("ne", "पोल\u{94d}याण\u{94d}ड"),
            ("nl", "Polen"),
            ("nn", "Polen"),
            ("nv", "Póolish Dineʼé Bikéyah"),
            ("oc", "Polonha"),
            ("or", "ପୋଲ\u{b3e}ଣ\u{b4d}ଡ"),
            ("pa", "ਪ\u{a4b}ਲ\u{a48}\u{a02}ਡ"),
            ("pi", "पोल\u{948}\u{902}ड"),
            ("pl", "Polska"),
            ("ps", "پولنډ"),
            ("pt", "Polónia"),
            ("pt_BR", "Polônia"),
            ("ro", "Polonia"),
            ("ru", "Польша"),
            ("rw", "Polonye"),
            ("sc", "Polònia"),
            ("sd", "پولينڊ"),
            ("si", "පෝලන\u{dca}තය"),
            ("sk", "Poľsko"),
            ("sl", "Poljska"),
            ("so", "Booland"),
            ("sq", "Poloni"),
            ("sr", "Пољска"),
            ("sv", "Polen"),
            ("sw", "Poland"),
            ("ta", "போலந\u{bcd}து"),
            ("te", "ప\u{c4b}ల\u{c3e}ండ\u{c4d}"),
            ("tg", "Лаҳистон"),
            ("th", "โปแลนด\u{e4c}"),
            ("ti", "ፖላንድ"),
            ("tk", "Polşa"),
            ("tl", "Poland"),
            ("tr", "Polonya"),
            ("tt", "Полониа, Полша"),
            ("ug", "پولشا"),
            ("uk", "Польща"),
            ("ur", "بولندا"),
            ("uz", "Polsha"),
            ("ve", "Poland"),
            ("vi", "Ba Lan"),
            ("wa", "Pologne"),
            ("wo", "Poloñ"),
            ("xh", "Poland"),
            ("yo", "Pólàndì"),
            ("zh_CN", "波兰"),
            ("zh_HK", "波蘭"),
            ("zh_TW", "波蘭"),
            ("zu", "IPolandi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
