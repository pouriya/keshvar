// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Cabo Verde

#[cfg(all(feature = "cv", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::CV;
    pub const ALPHA3: Alpha3 = Alpha3::CPV;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 238;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::CVE;
    pub const GEC: Option<GEC> = Some(GEC::CV);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CPV);
    pub const ISO_SHORT_NAME: &str = "Cabo Verde";
    pub const ISO_LONG_NAME: &str = "The Republic of Cabo Verde";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Cape Verdian");
    pub const NUMBER: &str = "132";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAfrica);
    pub const UN_LOCODE: &str = "CV";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Cape Verde",
        "Kap Verde",
        "Cap Vert",
        "Cabo Verde",
        "カーボベルデ",
        "Kaapverdië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Cabo Verde"),
        ("af", "Cabo Verde"),
        ("ak", "Cabo Verde"),
        ("am", "Cabo Verde"),
        ("an", "Cabo Verde"),
        ("ar", "الرأس الأخضر"),
        ("as", "Cabo Verde"),
        ("ay", "Cabo Verde"),
        ("az", "Cabo Verde"),
        ("ba", "Cabo Verde"),
        ("be", "Каба-Вердэ"),
        ("bg", "Cabo Verde"),
        ("bi", "Cabo Verde"),
        ("bn", "কেবো ভ\u{9be}র\u{9cd}ডে"),
        ("bn_IN", "কেবো ভ\u{9be}র\u{9cd}ডি"),
        ("br", "Cabo Verde"),
        ("bs", "Cabo Verde"),
        ("ca", "Cabo Verde"),
        ("ce", "Cabo Verde"),
        ("ch", "Cabo Verde"),
        ("cs", "Kapverdské ostrovy"),
        ("cv", "Cabo Verde"),
        ("cy", "Penrhyn Verde"),
        ("da", "Kap Verde"),
        ("de", "Cabo Verde"),
        ("dv", "Cabo Verde"),
        ("dz", "Cabo Verde"),
        ("ee", "Cabo Verde"),
        ("el", "Πράσινο Ακρωτήριο"),
        ("en", "Cabo Verde"),
        ("eo", "Kabo-Verdo"),
        ("es", "Cabo Verde"),
        ("et", "Roheneemesaared (Cabo Verde)"),
        ("eu", "Cabo Verde"),
        ("fa", "کیپ\u{200c}ورد"),
        ("ff", "Cabo Verde"),
        ("fi", "Cabo Verde"),
        ("fo", "Cabo Verde"),
        ("fr", "Cap-Vert"),
        ("fy", "Cabo Verde"),
        ("ga", "Cabo Verde"),
        ("gl", "Cabo Verde"),
        ("gn", "Cabo Verde"),
        ("gu", "ક\u{ac7}પ વર\u{acd}દ\u{ac7}"),
        ("gv", "Cabo Verde"),
        ("ha", "Cabo Verde"),
        ("he", "קאבו ורדה"),
        ("hi", "काबो वर\u{94d}ड\u{947}"),
        ("hr", "Zelenortski otoci"),
        ("ht", "Cabo Verde"),
        ("hu", "Zöld-foki-szigetek"),
        ("hy", "Cabo Verde"),
        ("ia", "Capo Verde"),
        ("id", "Cabo Verde"),
        ("io", "Cabo Verde"),
        ("is", "Grænhöfðaeyjar"),
        ("it", "Capo Verde"),
        ("iu", "Cabo Verde"),
        ("ja", "カーボヴェルデ"),
        ("ka", "Cabo Verde"),
        ("ki", "Cabo Verde"),
        ("kk", "Cabo Verde"),
        ("kl", "Cabo Verde"),
        ("km", "កាបវែរ"),
        ("kn", "Cabo Verde"),
        ("ko", "카보베르데"),
        ("ku", "Kap Verde"),
        ("kv", "Cabo Verde"),
        ("kw", "Cabo Verde"),
        ("ky", "Кабо-Верде"),
        ("lo", "Cabo Verde"),
        ("lt", "Cabo Verde"),
        ("lv", "Cabo Verde"),
        ("mi", "Cabo Verde"),
        ("mk", "Cabo Verde"),
        ("ml", "Cabo Verde"),
        ("mn", "Cabo Verde"),
        ("mr", "काबो व\u{94d}हर\u{94d}द\u{947}"),
        ("ms", "Cabo Verde"),
        ("mt", "Cabo Verde"),
        ("my", "Cabo Verde"),
        ("na", "Cabo Verde"),
        ("nb", "Kapp Verde"),
        ("ne", "Cabo Verde"),
        ("nl", "Kaapverdië"),
        ("nn", "Cabo Verde"),
        ("nv", "Cabo Verde"),
        ("oc", "Cap Verd"),
        ("or", "କ\u{b3e}ବୋ ଭଡ\u{b4d}ର\u{b3f}"),
        ("pa", "ਕਾਬ\u{a4b} ਵਾਰਡੀ"),
        ("pi", "Cabo Verde"),
        ("pl", "Republika Zielonego Przylądka"),
        ("ps", "Cabo Verde"),
        ("pt", "Cabo Verde"),
        ("pt_BR", "Cabo Verde"),
        ("ro", "Capul Verde"),
        ("ru", "Кабо-Верде"),
        ("rw", "Cabo Verde"),
        ("sc", "Cabu Birde"),
        ("sd", "Cabo Verde"),
        ("si", "Cabo Verde"),
        ("sk", "Kapverdy"),
        ("sl", "Cabo Verde"),
        ("so", "Cabo Verde"),
        ("sq", "Kepi i Gjelbërt"),
        ("sr", "Зеленортска острва"),
        ("sv", "Kap Verde"),
        ("sw", "Cabo Verde"),
        ("ta", "Cabo Verde"),
        ("te", "Cabo Verde"),
        ("tg", "Кабо-Верде"),
        ("th", "กาบ\u{e39}เวร\u{e4c}ด\u{e35}"),
        ("ti", "Cabo Verde"),
        ("tk", "Cabo Verde"),
        ("tl", "Cabo Verde"),
        ("tr", "Yeşil Burun Adaları"),
        ("tt", "Cabo Verde"),
        ("ug", "يېشىل تۇمشۇق"),
        ("uk", "Кабо-Верде"),
        ("ur", "Cabo Verde"),
        ("uz", "Cabo Verde"),
        ("ve", "Cabo Verde"),
        ("vi", "Cabo Verde"),
        ("wa", "Cabo Verde"),
        ("wo", "Cabo Verde"),
        ("xh", "Cabo Verde"),
        ("yo", "Cabo Verde"),
        ("zh_CN", "佛得角"),
        ("zh_HK", "佛得角"),
        ("zh_TW", "維德角"),
        ("zu", "Cabo Verde"),
    ];
    #[cfg(all(feature = "cv", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 16.5388;
        pub const LONGITUDE: f64 = -23.0418;
        pub const MAX_LATITUDE: f64 = 17.3191764;
        pub const MAX_LONGITUDE: f64 = -22.5933839;
        pub const MIN_LATITUDE: f64 = 14.7270733;
        pub const MIN_LONGITUDE: f64 = -25.383911;
        pub const NORTHEAST_LATITUDE: f64 = 17.3191764;
        pub const NORTHEAST_LONGITUDE: f64 = -22.5933839;
        pub const SOUTHWEST_LATITUDE: f64 = 14.7270733;
        pub const SOUTHWEST_LONGITUDE: f64 = -25.383911;
    }
}
#[cfg(all(feature = "cv", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 16.5388,
            longitude: -23.0418,
            max_latitude: 17.3191764,
            max_longitude: -22.5933839,
            min_latitude: 14.7270733,
            min_longitude: -25.383911,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 17.3191764,
                    longitude: -22.5933839,
                },
                southwest: CountryGeoBound {
                    latitude: 14.7270733,
                    longitude: -25.383911,
                },
            },
        }
    }
}

#[cfg(all(feature = "cv", feature = "subdivisions"))]
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
                    "B",
                    Subdivision{
                        name: "Ilhas de Barlavento",
                        country_alpha2: Alpha2::CV,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الجزر لافينتو"), ("be", "астравы Барлавенту"), ("ca", "Illes de Barlavento"), ("ccp", "𑄝𑄢\u{11134}𑄣𑄞𑄬𑄚\u{11134}𑄑\u{1112e} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Ilhas do Barlavento"), ("cs", "Barlavento"), ("da", "Barlavento"), ("de", "Ilhas de Barlavento"), ("en", "Barlavento Islands"), ("es", "Barlovento"), ("et", "Tuulepealsed saared"), ("fi", "Barlavento"), ("fr", "Îles de Barlavento"), ("gl", "Illas de Barlovento, Cabo Verde"), ("he", "בארלאוונטו"), ("it", "Ilhas do Barlavento"), ("ja", "バルラヴェント諸島"), ("ko", "바를라벤투 제도"), ("lt", "Priešvėjinės salos"), ("nb", "Barlavento"), ("nl", "Barlavento"), ("no", "Barlavento"), ("pl", "Wyspy Zawietrzne"), ("pt", "Ilhas de Barlavento"), ("sv", "Barlavento"), ("tr", "Barlavento Adaları"), ("ur", "بارلاوینتو جزائر"), ("zh", "向风群岛")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BR",
                    Subdivision{
                        name: "Brava",
                        country_alpha2: Alpha2::CV,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.8661257), longitude: Some(-24.7024179), max_latitude: Some(14.979173), min_latitude: Some(14.8026825), max_longitude: Some(-24.640837), min_longitude: Some(-24.7488499)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "برافا"), ("bn", "ব\u{9cd}র\u{9be}ভ\u{9be}"), ("ca", "Brava"), ("ccp", "𑄝\u{11133}𑄢𑄞"), ("da", "Brava"), ("de", "Brava"), ("el", "Μπράβα"), ("en", "Brava"), ("es", "Brava"), ("fi", "Brava"), ("fr", "Brava"), ("gu", "બ\u{acd}રાવા"), ("hi", "ब\u{94d}रावा"), ("id", "Brava"), ("it", "contea di Brava"), ("ja", "ブラヴァ (カーボベルデ)"), ("kn", "ಬ\u{ccd}ರವ"), ("ko", "브라바 시"), ("lt", "Brava"), ("lv", "Brava"), ("mr", "ब\u{94d}रावा"), ("ms", "Brava"), ("nb", "Brava"), ("nl", "Brava"), ("no", "Brava"), ("pl", "Brava"), ("pt", "Brava"), ("ru", "Брава"), ("si", "බ\u{dca}ර\u{dcf}ව\u{dcf}"), ("sv", "Brava"), ("ta", "பிறவ\u{bbe}"), ("te", "బ\u{c4d}ర\u{c3e}వ\u{c3e}"), ("th", "บราวา"), ("tr", "Brava"), ("uk", "Брава"), ("ur", "براوا"), ("vi", "Brava"), ("zh", "布拉瓦島縣")]),
                        unofficial_name_list: ["Brava"].to_vec(),
                    }
                ),
                (
                    "BV",
                    Subdivision{
                        name: "Boa Vista",
                        country_alpha2: Alpha2::CV,
                        code: "BV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.1372504), longitude: Some(-22.858217), max_latitude: Some(16.2250611), min_latitude: Some(15.9687814), max_longitude: Some(-22.6695356), min_longitude: Some(-22.9649422)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بوا فيستا"), ("bn", "ব\u{9c1}য\u{9bc}ো ভিস\u{9cd}ত\u{9be}"), ("ca", "Boa Vista"), ("ccp", "𑄝\u{1112e}𑄠 𑄞\u{11128}𑄌\u{11134}𑄑"), ("da", "Boa Vista"), ("de", "Boa Vista"), ("el", "Δήμος Μπόα Βίστα"), ("en", "Boa Vista"), ("es", "Boavista"), ("fi", "Boa Vista"), ("fr", "Boa Vista"), ("gu", "બોઆ વિસ\u{acd}ટા"), ("hi", "बोआ विस\u{94d}टा"), ("id", "Boa Vista"), ("it", "contea di Boa Vista"), ("ja", "ボア・ヴィスタ (カーボベルデ)"), ("kn", "ಬೋವಾ ವ\u{cbf}ಸ\u{ccd}ಟಾ"), ("ko", "보아비스타 시"), ("lt", "Boa Vista"), ("lv", "Boavišta"), ("mr", "बोआ व\u{94d}हिस\u{94d}टा"), ("ms", "Boa Vista"), ("nb", "Boa Vista"), ("nl", "Boa Vista"), ("no", "Boa Vista"), ("pl", "Boa Vista"), ("pt", "Boa Vista"), ("ro", "Municipalitatea Boa Vista, Capul Verde"), ("ru", "Боа-Виста"), ("si", "බොආ ව\u{dd2}ස\u{dca}ට\u{dcf}"), ("sv", "Boa Vista"), ("ta", "போஆ விஸ\u{bcd}ட\u{bcd}ட\u{bbe}"), ("te", "బ\u{c4b}వ\u{c3e} వ\u{c3f}స\u{c4d}త\u{c3e}"), ("th", "เบา ว\u{e34}สตา"), ("tr", "Boa Vista"), ("uk", "Боа-Вішта"), ("ur", "بوا وسٹا"), ("vi", "Boa Vista"), ("zh", "博阿維斯塔島縣")]),
                        unofficial_name_list: ["Boa Vista"].to_vec(),
                    }
                ),
                (
                    "CA",
                    Subdivision{
                        name: "Santa Catarina",
                        country_alpha2: Alpha2::CV,
                        code: "CA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.0992532), longitude: Some(-23.6918783), max_latitude: Some(15.199731), min_latitude: Some(14.9808349), max_longitude: Some(-23.6272161), min_longitude: Some(-23.7812941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا كاتارينا"), ("bg", "Санта Катарина"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9cd}য\u{9be}ট\u{9be}রিন\u{9be}"), ("ca", "Santa Catarina"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄠𑄑𑄢\u{11128}𑄚"), ("ceb", "Concelho de Santa Catarina"), ("da", "Santa Catarina"), ("de", "Santa Catarina"), ("el", "Δήμος Σάντα Καταρίνα"), ("en", "Santa Catarina"), ("es", "Santa Catarina"), ("fi", "Santa Catarina"), ("fr", "Santa Catarina"), ("gu", "સા\u{a82}તા ક\u{ac7}ટરિના"), ("he", "סנטה קטרינה"), ("hi", "स\u{948}\u{902}टा क\u{948}टरीना"), ("id", "Santa Catarina"), ("it", "contea di Santa Catarina"), ("ja", "サンタ・カタリナ"), ("kn", "ಸಾಂಟಾ ಕ\u{ccd}ಯಾಟರ\u{cbf}ನಾ"), ("ko", "산타카타리나 시"), ("lt", "Santa Katarinos savivaldybė"), ("lv", "Santakatarina"), ("mr", "सा\u{902}ता क\u{945}टरीना"), ("ms", "Santa Catarina"), ("nb", "Santa Catarina"), ("nl", "Santa Catarina (Kaapverdië)"), ("no", "Santa Catarina"), ("pl", "Santa Catarina"), ("pt", "Santa Catarina"), ("ru", "Санта-Катарина"), ("si", "සැන\u{dca}ට\u{dcf} කැතර\u{dd2}න\u{dcf}"), ("sv", "Concelho de Santa Catarina"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கேட\u{bcd}டறின\u{bbe}"), ("te", "స\u{c3e}ంట\u{c3e} క\u{c47}టర\u{c40}న\u{c3e}"), ("th", "ปาร\u{e4c}มา"), ("tr", "Santa Catarina"), ("uk", "Санта-Катаріна"), ("ur", "سنتا کاتارینا"), ("vi", "Santa Catarina"), ("zh", "聖卡塔琳娜縣")]),
                        unofficial_name_list: ["Santa Catarina"].to_vec(),
                    }
                ),
                (
                    "CF",
                    Subdivision{
                        name: "Santa Catarina do Fogo",
                        country_alpha2: Alpha2::CV,
                        code: "CF",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا كاتارينا دو فوجو"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9cd}য\u{9be}ট\u{9be}রিন\u{9be} দো ফোগো"), ("ca", "Santa Catarina do Fogo"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄠𑄑𑄢\u{11128}𑄚 𑄓\u{1112e} 𑄜\u{11127}𑄉\u{1112e}"), ("ceb", "Concelho de Santa Catarina do Fogo"), ("da", "Santa Catarina do Fogo"), ("de", "Santa Catarina do Fogo"), ("el", "Σάντα Καταρίνα ντο Φόργκο, Κέϊπ Βέρντε"), ("en", "Santa Catarina do Fogo"), ("es", "Santa Catarina do Fogo"), ("fi", "Santa Catarina do Fogo"), ("fr", "Santa Catarina do Fogo"), ("gu", "સા\u{a82}તા ક\u{ac7}ટરિના દો ફૉગો"), ("hi", "स\u{948}\u{902}टा क\u{948}टरीना डो फोगो"), ("id", "Santa Catarina do Fogo"), ("it", "contea di Santa Catarina do Fogo"), ("ja", "サンタ・カタリナ・ド・フォゴ"), ("kn", "ಸಾಂಟಾ ಕ\u{ccd}ಯಾಟರೀನಾ ಫೊಗೊ"), ("ko", "산타카타리나두포구 시"), ("lt", "Santa Katarinos savivaldybė²"), ("lv", "Santakatarina du Fugu"), ("mr", "सा\u{902}ता क\u{945}टरीना ड\u{942} फोगो"), ("ms", "Santa Catarina do Fogo"), ("nb", "Santa Catarina do Fogo"), ("nl", "Santa Catarina do Fogo"), ("no", "Santa Catarina do Fogo"), ("pl", "Santa Catarina do Fogo"), ("pt", "Santa Catarina do Fogo"), ("ru", "Санта-Катарина-до-Фого"), ("si", "සැන\u{dca}ට\u{dcf} කැතර\u{dd2}න\u{dcf}²"), ("sv", "Concelho de Santa Catarina do Fogo"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கேடறின டூ போகோ"), ("te", "స\u{c3e}ంట\u{c3e} క\u{c47}టర\u{c40}న\u{c3e} డ\u{c4b} ఫ\u{c4b}గ\u{c4b}"), ("th", "านต\u{e49}า ซาตาร\u{e34}นาโด โฟโก"), ("tr", "Santa Catarina do Fogo"), ("uk", "Санта-Катаріна-до-Фого"), ("ur", "سنتا کاتارینا دو فوجو"), ("vi", "Santa Catarina do Fogo"), ("zh", "福戈聖卡塔琳娜縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "Santa Cruz",
                        country_alpha2: Alpha2::CV,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.1027292), longitude: Some(-23.5609459), max_latitude: Some(15.1868812), min_latitude: Some(15.041449), max_longitude: Some(-23.4919112), min_longitude: Some(-23.639359)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا كروز"), ("bg", "Санта Круз"), ("bn", "স\u{9be}ন\u{9cd}ত\u{9be} ক\u{9cd}র\u{9c1}জের"), ("ca", "Santa Cruz"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄢\u{1112a}𑄌\u{11134}"), ("ceb", "Santa Cruz"), ("da", "Santa Cruz"), ("de", "Santa Cruz"), ("el", "Δήμος Σάντα Κρουζ"), ("en", "Santa Cruz"), ("es", "Santa Cruz (Cabo Verde)"), ("fi", "Santa Cruz"), ("fr", "Santa Cruz"), ("gu", "સાન\u{acd}તા ક\u{acd}ર\u{ac2}ઝ"), ("hi", "सा\u{902}ता क\u{94d}र\u{941}ज\u{93c}"), ("hu", "Santa Cruz"), ("id", "Santa Cruz"), ("it", "contea di Santa Cruz"), ("ja", "サンタ・クルス (カーボベルデ)"), ("kn", "ಸಾಂತಾ ಕ\u{ccd}ರ\u{cc2}ಜ\u{ccd}"), ("ko", "산타크루스 시"), ("lt", "Santa Kruso savivaldybė"), ("lv", "Santakruza"), ("mr", "सा\u{902}ताक\u{94d}र\u{942}झ"), ("ms", "Santa Cruz"), ("nb", "Santa Cruz"), ("nl", "Santa Cruz (Kaapverdië)"), ("no", "Santa Cruz"), ("pl", "Santa Cruz"), ("pt", "Santa Cruz"), ("ru", "Санта-Круз"), ("si", "සැන\u{dca}ට\u{dcf} කෘස\u{dca}"), ("sv", "Santa Cruz (ort i Kap Verde)"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கிருஸ\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3e} క\u{c4d}రజ\u{c4d}"), ("th", "ซานตาคร\u{e39}ซ"), ("tr", "Santa Cruz"), ("uk", "Санта-Круз"), ("ur", "سنتا کروز"), ("vi", "Santa Cruz"), ("zh", "聖克魯斯縣")]),
                        unofficial_name_list: ["Santa Cruz"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "Calheta de São Miguel",
                        country_alpha2: Alpha2::CV,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.1875), longitude: Some(-23.593), max_latitude: Some(15.1908814), min_latitude: Some(15.1799619), max_longitude: Some(-23.5874318), min_longitude: Some(-23.5979247)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Calheta de São Miguel")]),
                        unofficial_name_list: ["Calheta de São Miguel"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "Maio",
                        country_alpha2: Alpha2::CV,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.2003098), longitude: Some(-23.1679793), max_latitude: Some(15.3377624), min_latitude: Some(15.118355), max_longitude: Some(-23.0840044), min_longitude: Some(-23.2338931)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Maio"), ("ccp", "𑄟\u{1112d}𑄃\u{1112e}"), ("ceb", "Concelho do Maio"), ("de", "Maio"), ("en", "Maio"), ("es", "Maio"), ("fr", "Maio"), ("it", "contea di Maio"), ("ja", "マイオ (カーボベルデ)"), ("ko", "마이우 시"), ("nl", "Maio"), ("pt", "Maio"), ("ur", "مائیو، کیپ ورڈی"), ("zh", "馬約島縣")]),
                        unofficial_name_list: ["Maio"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "Mosteiros",
                        country_alpha2: Alpha2::CV,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.034), longitude: Some(-24.325), max_latitude: Some(15.0359258), min_latitude: Some(15.023137), max_longitude: Some(-24.3187952), min_longitude: Some(-24.3313264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "موستيروس، الرأس الأخضر"), ("bn", "মস\u{9cd}টেইরোস"), ("ca", "Mosteiros"), ("ccp", "𑄟\u{11127}𑄌\u{11134}𑄑𑄬\u{1112d}𑄢\u{1112e}𑄌\u{11134}"), ("ceb", "Concelho dos Mosteiros"), ("da", "Mosteiros"), ("de", "Mosteiros"), ("el", "Μοστέιρος"), ("en", "Mosteiros"), ("es", "Mosteiros"), ("fi", "Mosteiros"), ("fr", "Mosteiros"), ("gu", "મોસ\u{acd}ટ\u{ac7}રોસ"), ("hi", "मोस\u{94d}टायरोस"), ("id", "Mosteiros"), ("it", "contea di Mosteiros"), ("ja", "モシュテイホシュ"), ("kn", "ಮೋಸ\u{ccd}ಟ\u{cbf}ಯೊರೋಸ\u{ccd}"), ("ko", "모스테이루스 시"), ("lt", "Mosteiroso savivaldybė"), ("lv", "Mosteiros"), ("mr", "मोस\u{94d}ट\u{947}इरोस"), ("ms", "Mosteiros"), ("nb", "Mosteiros"), ("nl", "Mosteiros"), ("no", "Mosteiros"), ("pl", "Mosteiros"), ("pt", "Mosteiros"), ("ru", "Моштейруш"), ("si", "මොස\u{dca}ටෙර\u{dd2}යෝස\u{dca}"), ("sv", "Concelho dos Mosteiros"), ("ta", "மோஸ\u{bcd}ட\u{bc0}ரோஸ\u{bcd}"), ("te", "మ\u{c4b}స\u{c4d}ట\u{c46}య\u{c3f}ర\u{c4b}స\u{c4d}"), ("th", "มอสเตอร\u{e4c}รอส"), ("tr", "Mosteiros"), ("uk", "Моштейруш"), ("ur", "موستییروس"), ("vi", "Mosteiros"), ("zh", "莫什泰魯什縣")]),
                        unofficial_name_list: ["Mosteiros"].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "Paúl",
                        country_alpha2: Alpha2::CV,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.1005359), longitude: Some(-24.9872821), max_latitude: Some(17.153166), min_latitude: Some(17.0717081), max_longitude: Some(-24.967329), min_longitude: Some(-25.075462)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بول"), ("bn", "পল"), ("ca", "Paul"), ("ccp", "𑄛\u{11127}𑄣\u{11134}"), ("ceb", "Concelho do Paul"), ("da", "Paúl"), ("de", "Paul"), ("el", "Δήμος Παούλ"), ("en", "Paul"), ("es", "Paul"), ("fi", "Paul"), ("fr", "Paul"), ("gu", "પોલ"), ("hi", "पॉल"), ("id", "Paul"), ("it", "contea di Paul"), ("ja", "パウル (カーボベルデ)"), ("kn", "ಪಾಲ\u{ccd}"), ("ko", "파울 시"), ("lt", "Paulio savivaldybė"), ("lv", "Paula"), ("mr", "पॉल"), ("ms", "Paul"), ("nb", "Paúl"), ("nl", "Paúl"), ("no", "Paúl"), ("pl", "Paúl"), ("pt", "Paul"), ("ro", "Paúl"), ("ru", "Пол"), ("si", "ප\u{dcf}ව\u{dd4}ළ\u{dd4}"), ("sv", "Concelho do Paul"), ("ta", "ப\u{bbe}ல\u{bcd}"), ("te", "ప\u{c3e}ల\u{c4d}"), ("th", "พอล"), ("tr", "Paul"), ("uk", "Поль"), ("ur", "پال"), ("vi", "Paul"), ("zh", "保爾縣")]),
                        unofficial_name_list: ["Paúl"].to_vec(),
                    }
                ),
                (
                    "PN",
                    Subdivision{
                        name: "Porto Novo",
                        country_alpha2: Alpha2::CV,
                        code: "PN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.9911107), longitude: Some(-25.2363408), max_latitude: Some(17.1292379), min_latitude: Some(16.9104722), max_longitude: Some(-24.979912), min_longitude: Some(-25.3556249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بورتو نوفو"), ("bn", "পোর\u{9cd}টো নোভো"), ("ca", "Porto Novo"), ("ccp", "𑄛\u{11127}𑄢\u{11134}𑄑\u{1112e} 𑄚\u{1112e}𑄞\u{1112e}"), ("ceb", "Porto Novo"), ("da", "Porto Novo"), ("de", "Porto Novo"), ("el", "Δήμος Πόρτο Νόβο"), ("en", "Porto Novo"), ("es", "Porto Novo"), ("fi", "Porto Novo"), ("fr", "Porto Novo"), ("gu", "પોર\u{acd}ટો નોવો"), ("hi", "पोर\u{94d}टो नोवो"), ("id", "Porto Novo"), ("it", "contea di Porto Novo"), ("ja", "ポルト・ノボ (カーボベルデ)"), ("kn", "ಪೋರ\u{ccd}ಟೊ ನೋವೋ"), ("ko", "포르투노부 시"), ("lt", "Porto Novas"), ("lv", "Portonovu"), ("mr", "पोर\u{94d}टो नोवो"), ("ms", "Porto Novo"), ("nb", "Porto Novo"), ("nl", "Porto Novo"), ("no", "Porto Novo"), ("pl", "Porto Novo"), ("pt", "Porto Novo"), ("ro", "Porto Novo"), ("ru", "Порто-Ново"), ("si", "පෝර\u{dca}ටෝ-නොවෝ"), ("sv", "Porto Novo"), ("ta", "போர\u{bcd}டோ நோவோ"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4b} న\u{c4b}వ\u{c4b}"), ("th", "ค\u{e35}ช\u{e35}เนา"), ("tr", "Porto Novo"), ("uk", "Порто-Ново"), ("ur", "پورٹو نووو"), ("vi", "Porto Novo"), ("zh", "波多諾伏縣")]),
                        unofficial_name_list: ["Porto Novo"].to_vec(),
                    }
                ),
                (
                    "PR",
                    Subdivision{
                        name: "Praia",
                        country_alpha2: Alpha2::CV,
                        code: "PR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.95), longitude: Some(-23.52), max_latitude: Some(14.9727297), min_latitude: Some(14.9001745), max_longitude: Some(-23.4706162), min_longitude: Some(-23.5428429)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "برايا"), ("be", "Прая"), ("bg", "Прая"), ("bn", "প\u{9be}র\u{9be}ইয\u{9bc}\u{9be}"), ("ca", "Praia"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄠"), ("ceb", "Concelho da Praia"), ("da", "Praia"), ("de", "Praia"), ("el", "Δήμος Πράια"), ("en", "Praia"), ("es", "Praia"), ("fi", "Praia"), ("fr", "Praia"), ("gu", "પ\u{acd}રાઇઆ"), ("hi", "प\u{94d}र\u{947}या"), ("id", "Praia"), ("it", "contea di Praia"), ("ja", "プライア市"), ("kn", "ಪ\u{ccd}ರೈಯ"), ("ko", "프라이아 시"), ("lt", "Praja"), ("lv", "Praja"), ("mr", "प\u{94d}र\u{947}या"), ("ms", "Praia"), ("nb", "Praia"), ("nl", "Praia"), ("no", "Praia"), ("pl", "Praia"), ("pt", "Praia"), ("ro", "Municipalitatea Praia, Capul Verde"), ("ru", "община Прая"), ("si", "ප\u{dca}\u{200d}රය\u{dd2}ය\u{dcf}"), ("sv", "Praia"), ("ta", "ப\u{bcd}ரைய\u{bbe}"), ("te", "ప\u{c4d}ర\u{c3e}య\u{c3e}"), ("th", "ไปรอา"), ("tr", "Praia"), ("uk", "Прая"), ("ur", "پرایا"), ("vi", "Praia"), ("zh", "普拉亞縣")]),
                        unofficial_name_list: ["Praia"].to_vec(),
                    }
                ),
                (
                    "RB",
                    Subdivision{
                        name: "Ribeira Brava",
                        country_alpha2: Alpha2::CV,
                        code: "RB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريبيرا برافا"), ("bn", "রিবেইর\u{9be} ব\u{9cd}র\u{9be}ভ\u{9be}"), ("ca", "Ribeira Brava"), ("ccp", "𑄢\u{11128}𑄝𑄬\u{1112d}𑄢 𑄝\u{11133}𑄢𑄞"), ("ceb", "Concelho da Ribeira Brava"), ("da", "Ribeira Brava"), ("de", "Ribeira Brava"), ("el", "Δήμος Ριμπέιρα Μπράβα"), ("en", "Ribeira Brava"), ("es", "Ribeira Brava"), ("fi", "Ribeira Brava"), ("fr", "Ribeira Brava"), ("gu", "રિબ\u{ac7}રા બ\u{acd}રાવા"), ("hi", "रिब\u{947}रा ब\u{94d}रावा"), ("id", "Ribeira Brava"), ("it", "contea di Ribeira Brava"), ("ja", "リベイラ・ブラヴァ"), ("kn", "ರೈಬ\u{cc6}ರಾ ಬ\u{ccd}ರವಾ"), ("ko", "히베이라브라바 시"), ("lt", "Ribeira Brava"), ("lv", "Ribeirabrava"), ("mr", "रबीरा ब\u{94d}रावा"), ("ms", "Ribeira Brava"), ("nb", "Ribeira Brava"), ("nl", "Ribeira Brava"), ("no", "Ribeira Brava"), ("pl", "Ribeira Brava"), ("pt", "Ribeira Brava"), ("ro", "Ribeira Brava"), ("ru", "Рибейра-Брава"), ("si", "රෙබෙය\u{dd2}ර\u{dcf} බරව\u{dcf}"), ("sv", "Concelho da Ribeira Brava"), ("ta", "ரிபெயர பிறவ\u{bbe}"), ("te", "ర\u{c3f}బ\u{c3f}య\u{c47}ర\u{c3e} బ\u{c4d}ర\u{c3e}వ\u{c3e}"), ("th", "ร\u{e34}เบ\u{e35}ยรา บราว\u{e48}า"), ("tr", "Ribeira Brava"), ("uk", "Рібейра-Брава"), ("ur", "ریبییرا براوا"), ("vi", "Ribeira Brava"), ("zh", "里貝拉布拉瓦縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RG",
                    Subdivision{
                        name: "Ribeira Grande",
                        country_alpha2: Alpha2::CV,
                        code: "RG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.1601387), longitude: Some(-25.1177654), max_latitude: Some(17.199805), min_latitude: Some(17.077749), max_longitude: Some(-25.0201209), min_longitude: Some(-25.2308923)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريبيرا غراندي"), ("bn", "রবির\u{9be} গ\u{9cd}র\u{9cd}য\u{9be}ন\u{9cd}ডে"), ("ca", "Ribeira Grande"), ("ccp", "𑄢\u{11128}𑄝𑄬\u{1112d}𑄢 𑄉\u{11133}𑄢𑄚\u{11134}𑄓𑄬"), ("ceb", "Concelho da Ribeira Grande"), ("da", "Ribeira Grande"), ("de", "Ribeira Grande"), ("el", "Δήμος Ριμπέιρα Γκράντε"), ("en", "Ribeira Grande"), ("es", "Ribeira Grande"), ("fi", "Ribeira Grande"), ("fr", "Ribeira Grande"), ("gu", "રિબ\u{ac7}રા ગ\u{acd}રાન\u{acd}ડ\u{ac7}"), ("hi", "रिब\u{947}रा ग\u{94d}रा\u{902}ड\u{947}"), ("id", "Ribeira Grande"), ("it", "contea di Ribeira Grande"), ("ja", "リベイラ・グランデ"), ("kn", "ರ\u{cbf}ಬೇರ\u{cbf} ಗ\u{ccd}ರಾಂಡ\u{cc6}"), ("ko", "히베이라그란드 시"), ("lt", "Ribeira Grandė"), ("lv", "Ribeiragrandi"), ("mr", "रिब\u{947}रा ग\u{94d}रा\u{902}ड\u{947}"), ("ms", "Ribeira Grande"), ("nb", "Ribeira Grande"), ("nl", "Ribeira Grande"), ("no", "Ribeira Grande"), ("pl", "Ribeira Grande"), ("pt", "Ribeira Grande"), ("ro", "Ribeira Grande"), ("ru", "Рибейра-Гранде"), ("si", "ර\u{dd2}බෙය\u{dd2}ර\u{dcf} ග\u{dca}\u{200d}රන\u{dca}ඩේ"), ("sv", "Ribeira Grande"), ("ta", "ரிபெய\u{bcd}ர\u{bbe} கிர\u{bbe}ண\u{bcd}டே"), ("te", "ర\u{c3f}వ\u{c3f}య\u{c47}ర\u{c3e} గ\u{c4d}ర\u{c3e}ండ\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบเนเวนโต"), ("tr", "Riberie GRande"), ("uk", "Рібейра-Гранде"), ("ur", "ریبییرا جراندی"), ("vi", "Ribeira Grande"), ("zh", "大里貝拉縣")]),
                        unofficial_name_list: ["Santiago"].to_vec(),
                    }
                ),
                (
                    "RS",
                    Subdivision{
                        name: "Ribeira Grande de Santiago",
                        country_alpha2: Alpha2::CV,
                        code: "RS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ريبيرا غراندي دي سانتياغو، الرأس الأخضر"), ("bg", "Рибейра Гранде де Сантяго"), ("bn", "রইবের\u{9be} গ\u{9cd}র\u{9be}ন\u{9cd}ডে দি স\u{9be}ন\u{9cd}তিয\u{9bc}\u{9be}গ\u{9c1}"), ("ca", "Ribeira Grande de Santiago"), ("ccp", "𑄢\u{11128}𑄝𑄬\u{1112d}𑄢 𑄉\u{11133}𑄢𑄚\u{11134}𑄓𑄬 𑄓𑄬 𑄥𑄚\u{11134}𑄑\u{11128}𑄠𑄉\u{1112e}"), ("ceb", "Concelho de Ribeira Grande de Santiago"), ("da", "Ribeira Grande de Santiago"), ("de", "Ribeira Grande de Santiago"), ("el", "Δήμος Ριμπέιρα Γκράντε ντε Σαντιάγο"), ("en", "Ribeira Grande de Santiago"), ("es", "Ribeira Grande de Santiago"), ("fi", "Ribeira Grande de Santiago"), ("fr", "Ribeira Grande de Santiago"), ("gu", "રિબ\u{ac7}રા ગ\u{acd}રાન\u{acd}દ\u{ac7} દ\u{ac7} સ\u{ac7}ન\u{acd}ટિયાગો"), ("hi", "रिब\u{948}रा ग\u{94d}र\u{948}\u{902}ड\u{947} डी स\u{948}\u{902}टियागो"), ("id", "Ribeira Grande de Santiago"), ("it", "contea di Ribeira Grande de Santiago"), ("ja", "リベイラ・グランデ・デ・サンティアゴ (カーボベルデ)"), ("kn", "ರೈಬ\u{cc6}ರಾ ಗ\u{ccd}ರಾಂಡ\u{cc6} ಡ\u{cbf} ಸ\u{ccd}ಯಾಂಟ\u{cbf}ಯಾಗೊ"), ("ko", "히베이라그란드드산티아구 시"), ("lt", "Santjago Ribeira Grandės savivaldybė"), ("lv", "Ribeiragrandi di Santjagu pašvaldība"), ("mr", "रिब\u{947}रा ग\u{94d}रान\u{94d}द\u{947} द\u{947} स\u{945}\u{902}टियागो"), ("ms", "Ribeira Grande de Santiago"), ("nb", "Ribeira Grande de Santiago"), ("nl", "Ribeira Grande de Santiago"), ("no", "Ribeira Grande de Santiago"), ("pl", "Ribeira Grande de Santiago"), ("pt", "Ribeira Grande de Santiago"), ("ru", "Рибейра-Гранде-де-Сантьяго"), ("si", "ර\u{dd2}බෙය\u{dd2}ර\u{dcf} ග\u{dca}\u{200d}රන\u{dca}ඩේ ඩ\u{dd2} සන\u{dca}ත\u{dd2}ය\u{dcf}ගෝ"), ("sv", "Concelho de Ribeira Grande de Santiago"), ("ta", "ரிபெய\u{bcd}ர\u{bbe} கிர\u{bbe}ண\u{bcd}டே டே ச\u{bbe}ண\u{bcd}டிய\u{bbe}கோ"), ("te", "ర\u{c40}బ\u{c47}ర\u{c3e} గ\u{c4d}ర\u{c3e}ండ\u{c46} ద స\u{c3e}ంట\u{c3f}య\u{c3e}గ\u{c4b}"), ("th", "ร\u{e34}เบรา กรานเด ซานต\u{e34}อาโก"), ("tr", "Ribeira Grande de Santiago"), ("uk", "Рібейра-Гранде-де-Сантьяго"), ("ur", "ریبییرا جراندی دے سانتیاگو"), ("vi", "Ribeira Grande de Santiago"), ("zh", "聖地亞哥大里貝拉縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "Ilhas de Sotavento",
                        country_alpha2: Alpha2::CV,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::GeographicalRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جزر سوتافينتو"), ("be", "астравы Сатавенту"), ("ca", "Illes de Sotavento"), ("ccp", "𑄥\u{1112e}𑄑𑄞𑄬𑄚\u{11134}𑄑\u{1112e} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Ilhas do Sotavento"), ("cs", "Sotavento"), ("da", "Sotavento"), ("de", "Ilhas de Sotavento"), ("en", "Sotavento Islands"), ("es", "Sotavento"), ("et", "Tuulealused saared"), ("fa", "جزایر بارلاونتو"), ("fi", "Sotavento"), ("fr", "Îles de Sotavento"), ("gl", "Illas de Sotavento, Cabo Verde"), ("he", "סוטאוונטו"), ("it", "Ilhas do Sotavento"), ("ja", "ソタヴェント諸島"), ("ko", "소타벤투 제도"), ("lt", "Pavėjinės salos"), ("nb", "Sotavento"), ("nl", "Sotavento"), ("no", "Sotavento"), ("pl", "Wyspy Podwietrzne"), ("pt", "Ilhas de Sotavento"), ("sv", "Sotavento"), ("ur", "سوتاوینتو جزائر"), ("zh", "背風群島")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "São Domingos",
                        country_alpha2: Alpha2::CV,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.025), longitude: Some(-23.5625), max_latitude: Some(15.0340585), min_latitude: Some(15.0196346), max_longitude: Some(-23.552177), min_longitude: Some(-23.5748148)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساو دومينغوس"), ("bg", "Сао Домингос"), ("bn", "স\u{9be}\u{981}ও ডোমিঙ\u{9cd}গো"), ("ca", "São Domingos"), ("ccp", "𑄥𑄃\u{1112e} 𑄓\u{1112e}𑄟\u{11128}𑄚\u{11134}𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Concelho de São Domingos"), ("da", "São Domingos"), ("de", "São Domingos"), ("el", "Δήμος Σάο Ντομίνγκος"), ("en", "São Domingos"), ("es", "São Domingos"), ("fi", "São Domingos"), ("fr", "São Domingos"), ("gu", "સાઓ ડોમિ\u{a82}ગોસ"), ("hi", "साओ डोमि\u{902}गोस"), ("id", "São Domingos"), ("it", "contea di São Domingos"), ("ja", "サン・ドミンゴス"), ("kn", "ಸಾವೊ ಡೊಮ\u{cbf}ಂಗೊಸ\u{ccd}"), ("ko", "상도밍구스 시"), ("lt", "San Domingosas"), ("lv", "Sandomigusa"), ("mr", "साओ डोमि\u{902}गोस"), ("ms", "Sao Domingos"), ("nb", "São Domingos"), ("nl", "São Domingos"), ("no", "São Domingos"), ("pl", "São Domingos"), ("pt", "São Domingos"), ("ru", "Сан-Домингос"), ("si", "ස\u{dcf}ඕ ඩොම\u{dd2}න\u{dca}ගොස\u{dca}"), ("sv", "Concelho de São Domingos"), ("ta", "ஸோ டொமிங\u{bcd}கோஸ\u{bcd}"), ("te", "స\u{c3e}వ\u{c4b}\u{c4b} డ\u{c3e}మ\u{c3f}ంగ\u{c4b}స\u{c4d}"), ("th", "เซา โดม\u{e34}นโกส"), ("tr", "Sao Domingos"), ("uk", "Сан-Домінгос"), ("ur", "ساؤ دومینجوس"), ("vi", "Sao Domingos"), ("zh", "聖多明戈斯縣")]),
                        unofficial_name_list: ["São Domingos"].to_vec(),
                    }
                ),
                (
                    "SF",
                    Subdivision{
                        name: "São Filipe",
                        country_alpha2: Alpha2::CV,
                        code: "SF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.895), longitude: Some(-24.498), max_latitude: Some(14.9068766), min_latitude: Some(14.8795941), max_longitude: Some(-24.4744921), min_longitude: Some(-24.5021173)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساو فيليب"), ("bn", "স\u{9be}\u{981}ও ফিলিপ"), ("ca", "São Filipe"), ("ccp", "𑄥𑄃\u{1112e} 𑄜\u{11128}𑄣\u{11128}𑄛\u{11134}"), ("ceb", "Concelho do São Filipe"), ("da", "São Filipe"), ("de", "São Filipe"), ("el", "Σάο Φιλίπε"), ("en", "São Filipe"), ("es", "São Filipe"), ("fi", "São Filipe"), ("fr", "São Filipe"), ("gu", "સાઓ ફિલિપ"), ("hi", "साओ फिलिप"), ("id", "São Filipe"), ("it", "contea di São Filipe"), ("ja", "サン・フィリペ"), ("kn", "ಸಾವೊ ಫ\u{cbf}ಲ\u{cbf}ಪ\u{ccd}"), ("ko", "상필리프 시"), ("lt", "San Filipė"), ("lv", "Sanfilipi"), ("mr", "साओ फिलिप"), ("ms", "Sao Filipe"), ("nb", "São Filipe"), ("nl", "São Filipe"), ("no", "São Filipe"), ("pl", "São Filipe"), ("pt", "São Filipe (concelho de Cabo Verde)"), ("ru", "Сан-Фелипе"), ("si", "ස\u{dcf}ඕ ෆ\u{dd2}ල\u{dd2}පේ"), ("sv", "Concelho do São Filipe"), ("ta", "ஸோ பிலிப\u{bcd}"), ("te", "స\u{c3e}వ\u{c4b} ఫ\u{c3f}ల\u{c3f}ప\u{c46}"), ("th", "เซา ฟ\u{e34}ล\u{e34}เป\u{e49}"), ("tr", "Sao Filipe"), ("uk", "Сан-Філіпе"), ("ur", "ساؤ فیلیپی"), ("vi", "São Filipe"), ("zh", "聖菲利佩縣")]),
                        unofficial_name_list: ["São Filipe"].to_vec(),
                    }
                ),
                (
                    "SL",
                    Subdivision{
                        name: "Sal",
                        country_alpha2: Alpha2::CV,
                        code: "SL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.7266152), longitude: Some(-22.9297109), max_latitude: Some(16.8537251), min_latitude: Some(16.5853415), max_longitude: Some(-22.8727016), min_longitude: Some(-22.9942131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سال"), ("bn", "স\u{9be}ল"), ("ca", "Sal"), ("ccp", "𑄥𑄣\u{11134}"), ("ceb", "Sal Municipality"), ("da", "Sal"), ("de", "Sal"), ("el", "Δήμος Σαλ"), ("en", "Sal"), ("es", "Sal"), ("fi", "Sal"), ("fr", "Sal"), ("gu", "સાલ"), ("hi", "साल"), ("hu", "Sal"), ("id", "Sal"), ("it", "contea di Sal"), ("ja", "サル (カーボベルデ)"), ("kn", "ಸಾಲ\u{ccd}"), ("ko", "살 시"), ("lt", "Salas"), ("lv", "Sala"), ("mr", "साल"), ("ms", "Sal"), ("nb", "Sal"), ("nl", "Sal"), ("no", "Sal"), ("pl", "Sal"), ("pt", "Sal"), ("ru", "Сал"), ("si", "සල\u{dca}"), ("sv", "Sal"), ("ta", "ஸல\u{bcd}"), ("te", "స\u{c3e}ల\u{c4d}"), ("th", "ซาล"), ("tr", "Sal"), ("uk", "Сал"), ("ur", "سال"), ("vi", "Sal"), ("zh", "薩爾縣")]),
                        unofficial_name_list: ["Sal"].to_vec(),
                    }
                ),
                (
                    "SM",
                    Subdivision{
                        name: "São Miguel",
                        country_alpha2: Alpha2::CV,
                        code: "SM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Сао Мигел"), ("ca", "São Miguel"), ("ccp", "𑄥𑄃\u{1112e} 𑄟\u{1112d}𑄉\u{1112a}𑄠𑄬𑄣\u{11134}"), ("ceb", "Concelho de São Miguel"), ("de", "São Miguel"), ("en", "São Miguel"), ("es", "São Miguel (Cabo Verde)"), ("fr", "São Miguel"), ("it", "contea di São Miguel"), ("ja", "サン・ミゲル"), ("ko", "상미겔 시"), ("lt", "San Migelis"), ("nb", "São Miguel"), ("nl", "São Miguel (Kaapverdië)"), ("no", "São Miguel"), ("pl", "São Miguel"), ("pt", "São Miguel"), ("ur", "سآو میگوئل، کیپ ورڈی"), ("zh", "聖米戈爾縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "São Nicolau",
                        country_alpha2: Alpha2::CV,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.6048609), longitude: Some(-24.3103718), max_latitude: Some(16.6813965), min_latitude: Some(16.4791765), max_longitude: Some(-24.0076803), min_longitude: Some(-24.4276023)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "São Nicolau")]),
                        unofficial_name_list: ["São Nicolau"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "São Lourenço dos Órgãos",
                        country_alpha2: Alpha2::CV,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساو لورينسو دوس أورغاوس"), ("bn", "সেন\u{9cd}ট লরেন\u{9cd}স দোস অরগ\u{9cd}য\u{9be}স"), ("ca", "São Lourenço dos Órgãos"), ("ccp", "𑄥𑄃\u{1112e} 𑄣\u{1112e}𑄢𑄬𑄚\u{11134}𑄇\u{1112e} 𑄓\u{1112e}𑄌\u{11134} 𑄃\u{11127}𑄢\u{11134}𑄉𑄃\u{1112e}𑄌\u{11134}"), ("ceb", "São Lourenço dos Órgãos"), ("da", "Sao Lourenco dos Orgaos"), ("de", "São Lourenço dos Órgãos"), ("el", "Δήμος Σάο Λουρένσο ντος Οργκάος"), ("en", "São Lourenço dos Órgãos"), ("es", "São Lourenço dos Órgãos"), ("fi", "São Lourenço dos Órgãos"), ("fr", "São Lourenço dos Órgãos"), ("gu", "સાઓ લોર\u{ac7}ન\u{acd}સો ડોસ ઓગાઓસ"), ("hi", "साओ लोर\u{947}\u{902}को डॉस ऑर\u{947}गोस"), ("id", "São Lourenço dos Órgãos"), ("it", "contea di São Lourenço dos Órgãos"), ("ja", "サン・ローレンソ・ドス・オルガンス"), ("kn", "ಸಾವೊ ಲ\u{ccc}ರ\u{cc6}ನ\u{ccd}ಕೋ ಡಾಸ\u{ccd} ಓರ\u{ccd}ಗಾವೋಸ\u{ccd}"), ("ko", "상로렌수두스오르강스 시"), ("lt", "San Lorenso dos Organjoso savivaldybė"), ("lv", "Sanlourensu du Orgaosa"), ("mr", "साओ ल\u{942}र\u{947}नको डो ओस\u{94d}रागॉओस"), ("ms", "Sao Lourenco dos Orgaos"), ("nb", "São Lourenço dos Órgãos"), ("nl", "São Lourenço dos Órgãos"), ("no", "São Lourenço dos Órgãos"), ("pl", "São Lourenço dos Órgãos"), ("pt", "São Lourenço dos Órgãos"), ("ru", "Сан-Лоренсу-душ-Оргауш"), ("si", "ස\u{dcf}වෝ ලොරෙන\u{dca}සෝ දොස\u{dca} ඔර\u{dca}ගවොස\u{dca}"), ("sv", "São Lourenço dos Órgãos"), ("ta", "ஸோ லூரின\u{bcd}க\u{bcd}கோ ட\u{bbe}ஸ\u{bcd} ஒர\u{bcd}கவ\u{bcd}ஸ\u{bcd}"), ("te", "స\u{c3e}వ\u{c4b} ల\u{c4b}ర\u{c46}ంక\u{c4b} డ\u{c3e}స\u{c4d} ఓర\u{c4d}గ\u{c3e}వ\u{c4b}స\u{c4d}"), ("th", "เซา ร\u{e31}วเรนโค ดอส ออกาโอส"), ("tr", "Sao Lourenco dos Orgaos"), ("uk", "Сан-Лоренсу-душ-Оргауш"), ("ur", "ساؤ لوورینکو ڈوز ورجاوس"), ("vi", "Sao Lourenco dos Orgaos"), ("zh", "聖洛倫索縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "São Salvador do Mundo",
                        country_alpha2: Alpha2::CV,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساو سلفادور دو موندو"), ("bg", "Сао Салвадор до Мундо"), ("bn", "স\u{9be}ও স\u{9be}লভ\u{9be}ডর ড\u{9c1} ম\u{9c1}ন\u{9cd}ডো"), ("ca", "São Salvador do Mundo"), ("ccp", "𑄥𑄃\u{1112e} 𑄥𑄣\u{11134}𑄞𑄘\u{11127}𑄢\u{11134} 𑄓\u{1112e} 𑄟\u{1112a}𑄚\u{11134}𑄓\u{1112e}"), ("ceb", "Concelho de São Salvador do Mundo"), ("da", "São Salvador do Mundo"), ("de", "São Salvador do Mundo"), ("el", "Δήμος Σάο Σαλβαδόρ ντο Μούντο"), ("en", "São Salvador do Mundo"), ("es", "São Salvador do Mundo"), ("fi", "São Salvador do Mundo"), ("fr", "São Salvador do Mundo"), ("gu", "સાઓ સાલ\u{acd}વાડોર દો મ\u{ac1}ન\u{acd}ડો"), ("hi", "साओ साल\u{94d}वाडोर डो म\u{941}\u{902}डो"), ("id", "São Salvador do Mundo"), ("it", "contea di São Salvador do Mundo"), ("ja", "サン・サルバドル・ド・ムンド"), ("kn", "ಸಾವೊ ಸಾಲ\u{ccd}ವಡಾರ\u{ccd} ಡು ಮುಂಡೋ"), ("ko", "상살바도르두문두 시"), ("lt", "San Salvador do Mundas"), ("lv", "Sansalvadora du Mundu"), ("mr", "साओ साल\u{94d}वाडोर ड\u{942} म\u{941}\u{902}डो"), ("ms", "Sao Salvador do Mundo"), ("nb", "São Salvador do Mundo"), ("nl", "São Salvador do Mundo"), ("no", "São Salvador do Mundo"), ("pl", "São Salvador do Mundo"), ("pt", "São Salvador do Mundo"), ("ru", "Сан-Сальвадор-до-Мундо"), ("si", "ස\u{dcf}ඕ සැල\u{dca}වදෝර\u{dca} ඩො ම\u{dd4}න\u{dca}ඩෝ"), ("sv", "Concelho de São Salvador do Mundo"), ("ta", "ஸோ ச\u{bbe}ல\u{bcd}வட\u{bbe}ர\u{bcd} டூ முன\u{bcd}டோ"), ("te", "స\u{c3e}వ\u{c4d}న\u{c3f}\u{c4b} స\u{c3e}ల\u{c4d}వడ\u{c3e}ర\u{c4d} డు ముండ\u{c4b}"), ("th", "เซา ซ\u{e31}ลวาดอร\u{e4c} โดมอนโด"), ("tr", "Sao Salvador do Mundo"), ("uk", "Сан-Сальвадор-до-Мундо"), ("ur", "ساؤ سالوادور دو موندو"), ("vi", "São Salvador do Mundo"), ("zh", "聖薩爾瓦多蒙多縣")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SV",
                    Subdivision{
                        name: "São Vicente",
                        country_alpha2: Alpha2::CV,
                        code: "SV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.8341271), longitude: Some(-24.9279547), max_latitude: Some(16.9222019), min_latitude: Some(16.6069539), max_longitude: Some(-24.5742091), min_longitude: Some(-25.0855633)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ساو فيسنتي"), ("bn", "স\u{9be}\u{981}ও ভিন\u{9cd}সেন\u{9cd}ট"), ("ca", "São Vicente"), ("ccp", "𑄥𑄃\u{1112e} 𑄞\u{1112d}𑄥𑄬𑄚\u{11134}𑄑𑄬"), ("ceb", "Concelho de São Vicente"), ("da", "São Vicente"), ("de", "São Vicente"), ("el", "Δήμος Σάο Βισέντε"), ("en", "São Vicente"), ("es", "São Vicente"), ("fi", "São Vicente"), ("fr", "São Vicente"), ("gu", "સાઓ વિસ\u{ac7}ન\u{acd}ટ\u{ac7}"), ("hi", "साओ विस\u{947}\u{902}ट\u{947}"), ("id", "São Vicente"), ("it", "contea di São Vicente"), ("ja", "サン・ビセンテ"), ("kn", "ಸಾವೊ ವ\u{cbf}ಸ\u{cc6}ಂಟ\u{cc6}"), ("ko", "상비센트 시"), ("lt", "San Visentės savivaldybė"), ("lv", "Sanvinsenti"), ("mr", "साओ विस\u{947}\u{902}ट\u{947}"), ("ms", "Sao Vicente"), ("nb", "São Vicente"), ("nl", "São Vicente"), ("no", "São Vicente"), ("pl", "São Vicente"), ("pt", "São Vicente"), ("ru", "Сан-Висенти"), ("si", "ස\u{dcf}ඕ ව\u{dd2}සෙන\u{dca}ටේ"), ("sv", "São Vicente"), ("ta", "ஸோ விசேன\u{bcd}ட\u{bcd}"), ("te", "స\u{c3e}వ\u{c4b} వ\u{c3f}స\u{c46}ంట\u{c3f}"), ("th", "เขตเซา ว\u{e34}เซนเต"), ("tr", "Sao Vicente"), ("uk", "Сан-Вісенті"), ("ur", "ساؤ ویکینتی"), ("vi", "Sao Vicente"), ("zh", "聖維森特縣")]),
                        unofficial_name_list: ["São Vicente"].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "Tarrafal",
                        country_alpha2: Alpha2::CV,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.278), longitude: Some(-23.752), max_latitude: Some(15.283593), min_latitude: Some(15.2685529), max_longitude: Some(-23.7384294), min_longitude: Some(-23.7590069)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تارفال"), ("bg", "Тарафал"), ("bn", "ত\u{9be}র\u{9be}ফ\u{9be}ল"), ("ca", "Tarrafal"), ("ccp", "𑄑𑄢𑄜\u{11127}𑄣\u{11134}"), ("ceb", "Concelho do Tarrafal"), ("da", "Tarrafal"), ("de", "Tarrafal"), ("el", "Δήμος Ταρραφάλ"), ("en", "Tarrafal"), ("es", "Tarrafal"), ("fi", "Tarrafal"), ("fr", "Tarrafal"), ("gu", "ટ\u{ac7}ર\u{ac7}ફલ"), ("hi", "तार\u{94d}राफल"), ("hy", "Տարրաֆալ"), ("id", "Tarrafal"), ("it", "contea di Tarrafal"), ("ja", "タラファル"), ("kn", "ಟರ\u{ccd}ರಫಾಲ\u{ccd}"), ("ko", "타하팔 시"), ("lt", "Tarafalis"), ("lv", "Tarafala"), ("mr", "तारराफल"), ("ms", "Tarrafal"), ("nb", "Tarrafal"), ("nl", "Tarrafal"), ("no", "Tarrafal"), ("pl", "Tarrafal"), ("pt", "Tarrafal"), ("ru", "Таррафал"), ("si", "ටර\u{dca}රෆල\u{dca}"), ("sv", "Concelho do Tarrafal"), ("ta", "டர\u{bcd}ரப\u{bbe}ல\u{bcd}"), ("te", "ట\u{c3e}ర\u{c3e}\u{c3e}ప\u{c3e}ల\u{c4d}"), ("th", "ทาร\u{e4c}ราฟาล"), ("tr", "Tarrafal"), ("uk", "Таррафал"), ("ur", "تارافال"), ("vi", "Tarrafal"), ("zh", "塔拉法爾縣")]),
                        unofficial_name_list: ["Tarrafal"].to_vec(),
                    }
                ),
                (
                    "TS",
                    Subdivision{
                        name: "Tarrafal de São Nicolau",
                        country_alpha2: Alpha2::CV,
                        code: "TS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تارفال دي ساو نيكولاو"), ("bn", "ত\u{9cd}রর\u{9be}ফ\u{9be}ল দে স\u{9be}ও নিকোল\u{9be}উ"), ("ca", "Tarrafal de São Nicolau"), ("ccp", "𑄑\u{11133}𑄠𑄢𑄜\u{11127}𑄣\u{11134} 𑄓𑄬 𑄥𑄃\u{1112e} 𑄚\u{11128}𑄇\u{1112e}𑄣\u{1112d}𑄅\u{1112a}"), ("ceb", "Concelho do Tarrafal de São Nicolau"), ("da", "Tarrafal de São Nicolau"), ("de", "Tarrafal de São Nicolau"), ("el", "Δήμος Ταρραφάλ ντε Σάο Νικολάου"), ("en", "Tarrafal de São Nicolau"), ("es", "Tarrafal de São Nicolau"), ("fi", "Tarrafal de São Nicolau"), ("fr", "Tarrafal de São Nicolau"), ("gu", "તારાફલ દ\u{ac7} સાઓ નિકોલાઉ"), ("hi", "तराफल ड\u{947} साओ निकोलौ"), ("id", "Tarrafal de São Nicolau"), ("it", "contea di Tarrafal de São Nicolau"), ("ja", "タラファル・デ・サン・ニコラウ"), ("kn", "ಟ\u{ccd}ಯಾರಾಫಾಲ\u{ccd} ಡ\u{cbf} ಸಾವೊ ನ\u{cbf}ಕೋಲ\u{ccc}"), ("ko", "타하팔드상니콜라우 시"), ("lt", "San Nikolau Tarafalis"), ("lv", "Tarafala du Sanikolava"), ("mr", "टर\u{94d}रफल ड\u{947} साओ निकोलाऊ"), ("ms", "Tarrafal de Sao Nicolau"), ("nb", "Tarrafal de São Nicolau"), ("nl", "Tarrafal de São Nicolau"), ("no", "Tarrafal de São Nicolau"), ("pl", "Tarrafal de São Nicolau"), ("pt", "Tarrafal de São Nicolau"), ("ru", "Таррафал-де-Сан-Николау"), ("si", "ටර\u{dcf}ෆල\u{dca} ඩ\u{dd2} ස\u{dcf}ඕ න\u{dd2}කොලෞ"), ("sv", "Tarrafal de São Nicolau"), ("ta", "டர\u{bcd}ரப\u{bbe}ல\u{bcd} டி ஸோ நிகோல\u{bbe}"), ("te", "ట\u{c3e}ర\u{c3e}ఫ\u{c3e}ల\u{c4d} ద స\u{c3e}వ\u{c4b} న\u{c3f}క\u{c4b}ల\u{c3e}వ\u{c4d}"), ("th", "ทาราฟาว เดอ เสา น\u{e34}โคล\u{e31}ว"), ("tr", "Tarrafal de Sao Belediyesi"), ("uk", "Таррафал-де-Сан-Ніколау"), ("ur", "تارافال دے ساؤ نیکولاو"), ("vi", "Tarrafal de Sao Nicolau"), ("zh", "聖尼古勞塔拉法爾縣")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "cv")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::CV,
        alpha3: Alpha3::CPV,
        address_format: None,
        continent: Continent::Africa,
        country_code: 238,
        currency_code: CurrencyCode::CVE,
        gec: Some(GEC::CV),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CPV),
        iso_long_name: "The Republic of Cabo Verde",
        iso_short_name: "Cabo Verde",
        official_language_list: ["pt"].to_vec(),
        spoken_language_list: ["pt"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Cape Verdian"),
        number: "132",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAfrica),
        un_locode: "CV",
        unofficial_name_list: [
            "Cape Verde",
            "Kap Verde",
            "Cap Vert",
            "Cabo Verde",
            "カーボベルデ",
            "Kaapverdië",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Cabo Verde"),
            ("af", "Cabo Verde"),
            ("ak", "Cabo Verde"),
            ("am", "Cabo Verde"),
            ("an", "Cabo Verde"),
            ("ar", "الرأس الأخضر"),
            ("as", "Cabo Verde"),
            ("ay", "Cabo Verde"),
            ("az", "Cabo Verde"),
            ("ba", "Cabo Verde"),
            ("be", "Каба-Вердэ"),
            ("bg", "Cabo Verde"),
            ("bi", "Cabo Verde"),
            ("bn", "কেবো ভ\u{9be}র\u{9cd}ডে"),
            ("bn_IN", "কেবো ভ\u{9be}র\u{9cd}ডি"),
            ("br", "Cabo Verde"),
            ("bs", "Cabo Verde"),
            ("ca", "Cabo Verde"),
            ("ce", "Cabo Verde"),
            ("ch", "Cabo Verde"),
            ("cs", "Kapverdské ostrovy"),
            ("cv", "Cabo Verde"),
            ("cy", "Penrhyn Verde"),
            ("da", "Kap Verde"),
            ("de", "Cabo Verde"),
            ("dv", "Cabo Verde"),
            ("dz", "Cabo Verde"),
            ("ee", "Cabo Verde"),
            ("el", "Πράσινο Ακρωτήριο"),
            ("en", "Cabo Verde"),
            ("eo", "Kabo-Verdo"),
            ("es", "Cabo Verde"),
            ("et", "Roheneemesaared (Cabo Verde)"),
            ("eu", "Cabo Verde"),
            ("fa", "کیپ\u{200c}ورد"),
            ("ff", "Cabo Verde"),
            ("fi", "Cabo Verde"),
            ("fo", "Cabo Verde"),
            ("fr", "Cap-Vert"),
            ("fy", "Cabo Verde"),
            ("ga", "Cabo Verde"),
            ("gl", "Cabo Verde"),
            ("gn", "Cabo Verde"),
            ("gu", "ક\u{ac7}પ વર\u{acd}દ\u{ac7}"),
            ("gv", "Cabo Verde"),
            ("ha", "Cabo Verde"),
            ("he", "קאבו ורדה"),
            ("hi", "काबो वर\u{94d}ड\u{947}"),
            ("hr", "Zelenortski otoci"),
            ("ht", "Cabo Verde"),
            ("hu", "Zöld-foki-szigetek"),
            ("hy", "Cabo Verde"),
            ("ia", "Capo Verde"),
            ("id", "Cabo Verde"),
            ("io", "Cabo Verde"),
            ("is", "Grænhöfðaeyjar"),
            ("it", "Capo Verde"),
            ("iu", "Cabo Verde"),
            ("ja", "カーボヴェルデ"),
            ("ka", "Cabo Verde"),
            ("ki", "Cabo Verde"),
            ("kk", "Cabo Verde"),
            ("kl", "Cabo Verde"),
            ("km", "កាបវែរ"),
            ("kn", "Cabo Verde"),
            ("ko", "카보베르데"),
            ("ku", "Kap Verde"),
            ("kv", "Cabo Verde"),
            ("kw", "Cabo Verde"),
            ("ky", "Кабо-Верде"),
            ("lo", "Cabo Verde"),
            ("lt", "Cabo Verde"),
            ("lv", "Cabo Verde"),
            ("mi", "Cabo Verde"),
            ("mk", "Cabo Verde"),
            ("ml", "Cabo Verde"),
            ("mn", "Cabo Verde"),
            ("mr", "काबो व\u{94d}हर\u{94d}द\u{947}"),
            ("ms", "Cabo Verde"),
            ("mt", "Cabo Verde"),
            ("my", "Cabo Verde"),
            ("na", "Cabo Verde"),
            ("nb", "Kapp Verde"),
            ("ne", "Cabo Verde"),
            ("nl", "Kaapverdië"),
            ("nn", "Cabo Verde"),
            ("nv", "Cabo Verde"),
            ("oc", "Cap Verd"),
            ("or", "କ\u{b3e}ବୋ ଭଡ\u{b4d}ର\u{b3f}"),
            ("pa", "ਕਾਬ\u{a4b} ਵਾਰਡੀ"),
            ("pi", "Cabo Verde"),
            ("pl", "Republika Zielonego Przylądka"),
            ("ps", "Cabo Verde"),
            ("pt", "Cabo Verde"),
            ("pt_BR", "Cabo Verde"),
            ("ro", "Capul Verde"),
            ("ru", "Кабо-Верде"),
            ("rw", "Cabo Verde"),
            ("sc", "Cabu Birde"),
            ("sd", "Cabo Verde"),
            ("si", "Cabo Verde"),
            ("sk", "Kapverdy"),
            ("sl", "Cabo Verde"),
            ("so", "Cabo Verde"),
            ("sq", "Kepi i Gjelbërt"),
            ("sr", "Зеленортска острва"),
            ("sv", "Kap Verde"),
            ("sw", "Cabo Verde"),
            ("ta", "Cabo Verde"),
            ("te", "Cabo Verde"),
            ("tg", "Кабо-Верде"),
            ("th", "กาบ\u{e39}เวร\u{e4c}ด\u{e35}"),
            ("ti", "Cabo Verde"),
            ("tk", "Cabo Verde"),
            ("tl", "Cabo Verde"),
            ("tr", "Yeşil Burun Adaları"),
            ("tt", "Cabo Verde"),
            ("ug", "يېشىل تۇمشۇق"),
            ("uk", "Кабо-Верде"),
            ("ur", "Cabo Verde"),
            ("uz", "Cabo Verde"),
            ("ve", "Cabo Verde"),
            ("vi", "Cabo Verde"),
            ("wa", "Cabo Verde"),
            ("wo", "Cabo Verde"),
            ("xh", "Cabo Verde"),
            ("yo", "Cabo Verde"),
            ("zh_CN", "佛得角"),
            ("zh_HK", "佛得角"),
            ("zh_TW", "維德角"),
            ("zu", "Cabo Verde"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
