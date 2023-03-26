// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Albania

#[cfg(all(feature = "al", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AL;
    pub const ALPHA3: Alpha3 = Alpha3::ALB;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 355;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::ALL;
    pub const GEC: Option<GEC> = Some(GEC::AL);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ALB);
    pub const ISO_SHORT_NAME: &str = "Albania";
    pub const ISO_LONG_NAME: &str = "The Republic of Albania";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["sq"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["sq"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Albanian");
    pub const NUMBER: &str = "008";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "AL";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Albania", "Albanien", "Albanie", "アルバニア", "Albanië"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Albania"),
        ("af", "Albanië"),
        ("ak", "Albania"),
        ("am", "ጐሔባኒ።"),
        ("an", "Albania"),
        ("ar", "ألبانيا"),
        ("as", "আলবেনিয়\u{9be}"),
        ("ay", "Albania"),
        ("az", "Albaniya"),
        ("ba", "Albania"),
        ("be", "Албанія"),
        ("bg", "Албания"),
        ("bi", "Albania"),
        ("bn", "আলবেনিয়\u{9be}"),
        ("bn_IN", "আলবেনিয়\u{9be}"),
        ("br", "Albania"),
        ("bs", "Albanija"),
        ("ca", "Albània"),
        ("ce", "Албани"),
        ("ch", "Albania"),
        ("cs", "Albánie"),
        ("cv", "Албани"),
        ("cy", "Albania"),
        ("da", "Albanien"),
        ("de", "Albanien"),
        ("dv", "އ\u{7a6}ލ\u{7b0}ބ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
        ("dz", "ཨ\u{f71}ལ་བ་ན\u{f72}་ཡ།"),
        ("ee", "Albania"),
        ("el", "Αλβανία"),
        ("en", "Albania"),
        ("eo", "Albanio"),
        ("es", "Albania"),
        ("et", "Albaania"),
        ("eu", "Albania"),
        ("fa", "آلبانی"),
        ("ff", "Albaniya"),
        ("fi", "Albania"),
        ("fo", "Albania"),
        ("fr", "Albanie"),
        ("fy", "Albaanje"),
        ("ga", "An Albáin"),
        ("gl", "Albania"),
        ("gn", "Albania"),
        ("gu", "અલ\u{acd}બ\u{ac7}નિયા"),
        ("gv", "Yn Albaan"),
        ("ha", "Albaniya"),
        ("he", "אלבניה"),
        ("hi", "अल\u{94d}बानिया"),
        ("hr", "Albanija"),
        ("ht", "Albani"),
        ("hu", "Albánia"),
        ("hy", "Ալբանիա"),
        ("ia", "Albania"),
        ("id", "Albania"),
        ("io", "Albania"),
        ("is", "Albanía"),
        ("it", "Albania"),
        ("iu", "Albania"),
        ("ja", "アルバニア"),
        ("ka", "ალბანეთი"),
        ("ki", "Arũmbĩnia"),
        ("kk", "Албания"),
        ("kl", "Albania"),
        ("km", "អាល\u{17cb}បាន\u{17b8}"),
        ("kn", "ಅಲ\u{ccd}ಬೇನ\u{cbf}ಯಾ"),
        ("ko", "알바니아"),
        ("ku", "Arnavût"),
        ("kv", "Албания"),
        ("kw", "Albani"),
        ("ky", "Албания"),
        ("lo", "Albania"),
        ("lt", "Albanija"),
        ("lv", "Albānija"),
        ("mi", "Arapeinia"),
        ("mk", "Албанија"),
        ("ml", "അല\u{d4d}\u{200d}ബേനിയ"),
        ("mn", "Албани"),
        ("mr", "अल\u{94d}ब\u{947}निया"),
        ("ms", "Albania"),
        ("mt", "Albanija"),
        (
            "my",
            "အယ\u{103a}လ\u{103a}ဘေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Arbainiya"),
        ("nb", "Albania"),
        ("ne", "अल\u{94d}बानिया"),
        ("nl", "Albanië"),
        ("nn", "Albania"),
        ("nv", "Dziłigaii Bikéyah"),
        ("oc", "Albania"),
        ("or", "ଅଲ\u{b4d}ବ\u{b3e}ନୀୟ\u{b3e}"),
        ("pa", "ਅਲਬੀਨਾ"),
        ("pi", "अल\u{94d}बानिया"),
        ("pl", "Albania"),
        ("ps", "البانیه"),
        ("pt", "Albânia"),
        ("pt_BR", "Albânia"),
        ("ro", "Albania"),
        ("ru", "Албания"),
        ("rw", "Alubaniya"),
        ("sc", "Albania"),
        ("sd", "Albania"),
        ("si", "ඇල\u{dca}බේන\u{dd2}ය\u{dcf}ව"),
        ("sk", "Albánsko"),
        ("sl", "Albanija"),
        ("so", "Albania"),
        ("sq", "Shqipëri"),
        ("sr", "Албанија"),
        ("sv", "Albanien"),
        ("sw", "Albania"),
        ("ta", "அல\u{bcd}பேனிய\u{bbe}"),
        ("te", "అల\u{c4d}బ\u{c47}న\u{c3f}య\u{c3e}"),
        ("tg", "Албания"),
        ("th", "แอลเบเน\u{e35}ย"),
        ("ti", "አልባኒያ"),
        ("tk", "Albaniýa"),
        ("tl", "Albanya"),
        ("tr", "Arnavutluk"),
        ("tt", "Албаниа"),
        ("ug", "ئالبانىيە"),
        ("uk", "Албанія"),
        ("ur", "البانیا"),
        ("uz", "Albaniya"),
        ("ve", "Albania"),
        ("vi", "An-ba-ni"),
        ("wa", "Albaneye"),
        ("wo", "Albaani"),
        ("xh", "Albania"),
        ("yo", "Albáníà"),
        ("zh_CN", "阿尔巴尼亚"),
        ("zh_HK", "阿爾巴尼亞"),
        ("zh_TW", "阿爾巴尼亞"),
        ("zu", "I-Albaniya"),
    ];
    #[cfg(all(feature = "al", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 41.153332;
        pub const LONGITUDE: f64 = 20.168331;
        pub const MAX_LATITUDE: f64 = 42.6611669;
        pub const MAX_LONGITUDE: f64 = 21.0572394;
        pub const MIN_LATITUDE: f64 = 39.6447296;
        pub const MIN_LONGITUDE: f64 = 19.1217;
        pub const NORTHEAST_LATITUDE: f64 = 42.6611669;
        pub const NORTHEAST_LONGITUDE: f64 = 21.0572394;
        pub const SOUTHWEST_LATITUDE: f64 = 39.6447296;
        pub const SOUTHWEST_LONGITUDE: f64 = 19.1217;
    }
}
#[cfg(all(feature = "al", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 41.153332,
            longitude: 20.168331,
            max_latitude: 42.6611669,
            max_longitude: 21.0572394,
            min_latitude: 39.6447296,
            min_longitude: 19.1217,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 42.6611669,
                    longitude: 21.0572394,
                },
                southwest: CountryGeoBound {
                    latitude: 39.6447296,
                    longitude: 19.1217,
                },
            },
        }
    }
}

#[cfg(all(feature = "al", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "Berat",
                        country_alpha2: Alpha2::AL,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.7086377), longitude: Some(19.9437314), max_latitude: Some(40.7306085), min_latitude: Some(40.6923532), max_longitude: Some(19.9808693), min_longitude: Some(19.9287128)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيرات"), ("be", "Вобласць Берат"), ("bg", "Берат"), ("bn", "বের\u{9be}ট ক\u{9be}উন\u{9cd}টি"), ("bs", "Okrug Berat"), ("ca", "comtat de Berat"), ("ccp", "𑄝𑄬𑄢\u{11127}𑄖\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Beratit"), ("cs", "Berat (kraj)"), ("da", "Berat County"), ("de", "Qark Berat"), ("el", "Νομός Μπεράτ"), ("en", "Berat County"), ("es", "condado de Berat"), ("et", "Berati maakond"), ("eu", "Berat konderria"), ("fa", "شهرستان برات"), ("fi", "Beratin maakunta"), ("fr", "préfecture de Berat"), ("ga", "Contae Berat"), ("gl", "condado de Berat"), ("gu", "બ\u{ac7}રાટ કાઉન\u{acd}ટી"), ("hi", "ब\u{947}रत प\u{94d}रा\u{902}त"), ("hr", "Beratski okrug"), ("hu", "Berat megye"), ("hy", "Բերատի գավառ"), ("id", "Berat County"), ("it", "prefettura di Berat"), ("ja", "ベラト州"), ("ka", "ბერატის ოლქი"), ("kn", "ಬ\u{cc6}ರಾಟ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베라트 주"), ("lt", "Beračio apskritis"), ("lv", "Berati apgabals"), ("mk", "Белград"), ("mn", "Берат хошуу"), ("mr", "ब\u{947}राट काउ\u{902}टी"), ("ms", "Berat County"), ("nb", "Berat"), ("nl", "Berat"), ("no", "Berat"), ("pa", "ਬ\u{a47}ਰਾਤ ਕਾਉ\u{a02}ਟੀ"), ("pl", "obwód Berat"), ("pt", "Berat"), ("ro", "Regiunea Berat"), ("ru", "Берат"), ("si", "බෙරට\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Berat"), ("sq", "Qarku i Beratit"), ("sv", "Beratprefekturen"), ("ta", "பிரெட\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c46}ర\u{c3e}ట\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เบราท"), ("tr", "Berat ili"), ("uk", "Берат"), ("ur", "صوبہ بیرات"), ("vi", "Berat"), ("zh", "培拉特州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "Durrës",
                        country_alpha2: Alpha2::AL,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.316667), longitude: Some(19.45), max_latitude: Some(41.3809305), min_latitude: Some(41.2817411), max_longitude: Some(19.5148087), min_longitude: Some(19.418378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوريس"), ("be", "Дурэс"), ("bg", "Дуръс"), ("bn", "দ\u{9c1}রস ক\u{9be}উন\u{9cd}টি"), ("bs", "Okrug Drač"), ("ca", "Durrës"), ("ccp", "𑄓\u{11128}𑄅\u{1112a}𑄢𑄬𑄌\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Durrësit"), ("cs", "Durrës (kraj)"), ("da", "Durrës County"), ("de", "Qark Durrës"), ("el", "Νομός Δυρραχίου"), ("en", "Durrës County"), ("es", "condado de Durrës"), ("et", "Durrësi maakond"), ("eu", "Durrës konderria"), ("fa", "شهرستان دورس"), ("fi", "Durrësin maakunta"), ("fr", "préfecture de Durrës"), ("ga", "Contae Durrës"), ("gl", "condado de Durrës"), ("gu", "દ\u{ac1}ર\u{ac7}સ કાઉન\u{acd}ટી"), ("hi", "ड\u{941}र\u{94d}र\u{947}स प\u{94d}रा\u{902}त"), ("hr", "Drački okrug"), ("hu", "Durrës megye"), ("hy", "Դուրրես"), ("id", "Durrës County"), ("it", "prefettura di Durazzo"), ("ja", "ドゥラス州"), ("kn", "ಡರ\u{ccd}ರ\u{cc6}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "두러스 주"), ("lt", "Duresio apskritis"), ("lv", "Duresi apgabals"), ("mk", "Драч"), ("mn", "Дуррес хошуу"), ("mr", "द\u{941}रस\u{947} काउ\u{902}टी"), ("ms", "Durres County"), ("nb", "Durrës"), ("nl", "Durrës"), ("no", "Durrës"), ("pa", "ਡ\u{a42}ਰ\u{a47}ਸ ਕਾਉ\u{a02}ਟੀ"), ("pl", "obwód Durrës"), ("pt", "Durrës"), ("ro", "Regiunea Durrës"), ("ru", "Дуррес"), ("si", "ඩරස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Drač"), ("sq", "Qarku i Durrësit"), ("sv", "Durrës prefektur"), ("ta", "டூர\u{bcd}ஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "డుర\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "โลวเรนซ\u{e4c} นา โพฮอร\u{e4c}จ\u{e39}"), ("tr", "Dıraç ili"), ("uk", "Дуррес"), ("ur", "صوبہ دراج"), ("vi", "Durrës"), ("zh", "杜勒斯州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "Elbasan",
                        country_alpha2: Alpha2::AL,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.11023), longitude: Some(20.0866554), max_latitude: Some(41.1308961), min_latitude: Some(41.0875028), max_longitude: Some(20.1270677), min_longitude: Some(20.0506784)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إلباسان"), ("be", "вобласць Эльбасан"), ("bg", "Елбасан"), ("bs", "Okrug Elbasan"), ("ca", "comtat d’Elbasan"), ("ccp", "𑄃𑄬𑄣\u{11134}𑄝𑄥𑄚\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Elbasanit"), ("cs", "Elbasan (kraj)"), ("de", "Qark Elbasan"), ("el", "Νομός Ελβασάν"), ("en", "Elbasan County"), ("es", "condado de Elbasan"), ("et", "Elbasani maakond"), ("eu", "Elbasan konderria"), ("fa", "شهرستان الباسان"), ("fi", "Elbasanin maakunta"), ("fr", "préfecture d’Elbasan"), ("ga", "Contae Elbasan"), ("gl", "condado de Elbasan"), ("hi", "एलबसन प\u{94d}रा\u{902}त"), ("hr", "Elbasanski okrug"), ("hu", "Elbasan megye"), ("hy", "Էլբասանի գավառ"), ("it", "prefettura di Elbasan"), ("ja", "エルバサン州"), ("ko", "엘바산 주"), ("lt", "Elbasanio apskritis"), ("mk", "Елбасан"), ("mn", "Эльбасан хошуу"), ("nb", "Elbasan"), ("nl", "Elbasan"), ("no", "Elbasan"), ("pl", "obwód Elbasan"), ("pt", "Elbasan"), ("ro", "Regiunea Elbasan"), ("ru", "Эльбасан"), ("sk", "Elbasan"), ("sq", "Qarku i Elbasanit"), ("sv", "Elbasan prefektur"), ("tr", "Elbasan ili"), ("uk", "Ельбасан"), ("ur", "صوبہ الباسان"), ("vi", "Elbasan"), ("zh", "艾巴申州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "Fier",
                        country_alpha2: Alpha2::AL,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9191392), longitude: Some(19.6639309), max_latitude: Some(41.06486599999999), min_latitude: Some(40.4181381), max_longitude: Some(19.92129), min_longitude: Some(19.321621)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيير"), ("bg", "Фиер"), ("bs", "Okrug Fier"), ("ca", "Comtat de Fier"), ("ccp", "𑄜\u{1112d}𑄠𑄬𑄢\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Fierit"), ("cs", "Fier (kraj)"), ("de", "Qark Fier"), ("el", "Νομός Φιέρ"), ("en", "Fier County"), ("es", "Condado de Fier"), ("et", "Fieri maakond"), ("eu", "Fier konderria"), ("fa", "شهرستان فیر"), ("fi", "Fierin maakunta"), ("fr", "préfecture de Fier"), ("ga", "Contae Fier"), ("gl", "Condado de Fier"), ("hi", "फिएर प\u{94d}रा\u{902}त"), ("hr", "Fierski okrug"), ("hu", "Fier megye"), ("hy", "Ֆիերիի գավառ"), ("it", "prefettura di Fier"), ("ja", "フィエル州"), ("ko", "피에르 주"), ("lt", "Fierio apskritis"), ("mk", "Фиер"), ("mn", "Фиер хошуу"), ("nb", "Fier"), ("nl", "Fier"), ("no", "Fier"), ("pl", "Obwód Fier"), ("pt", "Fier"), ("ro", "Regiunea Fier"), ("ru", "Фиери"), ("sk", "Fier"), ("sq", "Qarku i Fierit"), ("sv", "Fier prefektur"), ("tr", "Fier ili"), ("uk", "Фієр"), ("ur", "صوبہ فیر"), ("vi", "Fier"), ("zh", "非夏爾州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "Gjirokastër",
                        country_alpha2: Alpha2::AL,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.0672874), longitude: Some(20.1045229), max_latitude: Some(40.1064699), min_latitude: Some(40.0330693), max_longitude: Some(20.1654482), min_longitude: Some(20.0553703)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غيروكاستر"), ("be", "Гіракастра"), ("bg", "Аргирокастро"), ("bn", "গিজ\u{9be}রক\u{9be}স\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("bs", "Okrug Gjirokastër"), ("ca", "Comtat de Gjirokastër"), ("ccp", "𑄎\u{11128}𑄢\u{11127}𑄇𑄌\u{11134}𑄑𑄢\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Gjirokastrës"), ("cs", "Gjirokastër"), ("da", "Gjirokastër County"), ("de", "Qark Gjirokastra"), ("el", "Νομός Αργυροκάστρου"), ("en", "Gjirokastër County"), ("es", "Condado de Gjirokastër"), ("et", "Gjirokastëri maakond"), ("eu", "Gjirokastër konderria"), ("fa", "شهرستان گیروکاستر"), ("fi", "Gjirokastërin maakunta"), ("fr", "préfecture de Gjirokastër"), ("ga", "Contae Gjirokastër"), ("gl", "Condado de Gjirokastër"), ("gu", "ગ\u{acd}જીરોકાસ\u{acd}ટર કાઉન\u{acd}ટી"), ("he", "ג׳ירוקסטרה"), ("hi", "घिरोकसत\u{947}र प\u{94d}रा\u{902}त"), ("hr", "Gjirokastërski okrug"), ("hu", "Gjirokastra megye"), ("hy", "Գիրոկաստրայի գավառ"), ("id", "Gjirokastër County"), ("it", "prefettura di Argirocastro"), ("ja", "ジロカストラ州"), ("ka", "გიროკასტრის ოლქი"), ("kn", "ಜ\u{cbf}ಜ\u{cbf}ರೋಸ\u{ccd}ಟೇರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "지로카스터르 주"), ("lt", "Girokasterio apskritis"), ("lv", "Ģirokastras apgabals"), ("mk", "Ѓирокастро"), ("mr", "जिजिरोकास\u{94d}ट\u{94d}र काउ\u{902}टी"), ("ms", "Gjirokaster County"), ("nb", "Gjirokastër"), ("nl", "Gjirokastër"), ("no", "Gjirokastër"), ("pl", "Obwód Gjirokastra"), ("pt", "Gjirokastër"), ("ro", "Regiunea Gjirokastër"), ("ru", "Гирокастра"), ("si", "ජ\u{dd2}යරෝක\u{dcf}ස\u{dca}ටර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Gjirokastër"), ("sl", "Pokrajina Gjirokastër"), ("sq", "Qarku i Gjirokastrës"), ("sv", "Gjirokastër prefektur"), ("ta", "கிஜிரோகஸ\u{bcd}ட\u{bcd}டர\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c3f}జ\u{c3f}ర\u{c4b}క\u{c3e}స\u{c4d}టర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องก\u{e35}โรคาสเตอร\u{e4c}"), ("tr", "Ergiri ili"), ("uk", "Ґʼїрокастер"), ("ur", "صوبہ ارجیر"), ("vi", "Gjirokastër"), ("zh", "吉羅卡斯特州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "Korçë",
                        country_alpha2: Alpha2::AL,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6140785), longitude: Some(20.7778071), max_latitude: Some(40.63663890000001), min_latitude: Some(40.5985741), max_longitude: Some(20.7976342), min_longitude: Some(20.762229)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كورتشي"), ("be", "вобласць Корча"), ("bg", "Корча"), ("bn", "কোরচে ক\u{9be}উন\u{9cd}টি"), ("bs", "Okrug Korča"), ("ca", "Comtat de Korçë"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄌\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Korçës"), ("cs", "Korçë (kraj)"), ("da", "Korçë County"), ("de", "Qark Korça"), ("el", "Νομός Κορυτσάς"), ("en", "Korçë County"), ("es", "Condado de Korçë"), ("et", "Korçë maakond"), ("eu", "Korçë konderria"), ("fa", "شهرستان کورچه"), ("fi", "Korçën maakunta"), ("fr", "préfecture de Korçë"), ("ga", "Contae Korçë"), ("gl", "Condado de Korçë"), ("gu", "કોરસ\u{ac7} કાઉન\u{acd}ટી"), ("hi", "कोर\u{94d}स\u{947} प\u{94d}रा\u{902}त"), ("hr", "Korčanski okrug"), ("hu", "Korça megye"), ("hy", "Կորչայի գավառ"), ("id", "Korçë County"), ("it", "prefettura di Coriza"), ("ja", "コルチャ州"), ("kn", "ಕೊರ\u{ccd}ಸ\u{cbf}ಯ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "코르처 주"), ("lt", "Korčės apskritis"), ("lv", "Korčas apgabals"), ("mk", "Горица"), ("mr", "कोरॉए काउ\u{902}टी"), ("ms", "Korce County"), ("nb", "Korçë"), ("ne", "कोर\u{94d}स\u{947} काउन\u{94d}टी"), ("nl", "Korçë"), ("no", "Korçë"), ("pl", "Obwód Korcza"), ("pt", "Korçë"), ("ro", "Regiunea Korçë"), ("ru", "Корча"), ("si", "කොර\u{dca}සේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Korçë"), ("sq", "Qarku i Korçës"), ("sv", "Korçë prefektur"), ("ta", "கூர\u{bcd}க\u{bcd}ஏ கவுண\u{bcd}டி"), ("te", "క\u{c4b}ర\u{c4d}స\u{c46} క\u{c4c}ంట\u{c40}"), ("th", "คอซ\u{e35}\u{e48}"), ("tr", "Görice ili"), ("uk", "Корча"), ("ur", "صوبہ کورچہ"), ("vi", "Korçë"), ("zh", "科赤州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "Kukës",
                        country_alpha2: Alpha2::AL,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.076667), longitude: Some(20.421667), max_latitude: Some(42.1007892), min_latitude: Some(42.0344725), max_longitude: Some(20.4430247), min_longitude: Some(20.3917409)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوكس"), ("be", "Кукес"), ("bg", "Кукъс"), ("bs", "Okrug Kukës"), ("ca", "Comtat de Kukës"), ("ccp", "𑄇\u{1112a}𑄇𑄬𑄌\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Kukësit"), ("cs", "Kukës (kraj)"), ("de", "Qark Kukës"), ("el", "Νομός Κουκς"), ("en", "Kukës County"), ("es", "Condado de Kukës"), ("et", "Kukësi maakond"), ("eu", "Kukës konderria"), ("fa", "شهرستان کوکس"), ("fi", "Kukësin maakunta"), ("fr", "préfecture de Kukës"), ("ga", "Contae Kukës"), ("gl", "Condado de Kukës"), ("hi", "क\u{941}क\u{947}स प\u{94d}रा\u{902}त"), ("hr", "Kukëski okrug"), ("hu", "Kukës megye"), ("hy", "Կուկասի գավառ"), ("it", "prefettura di Kukës"), ("ja", "クケス州"), ("ko", "쿠커스 주"), ("lt", "Kukesio apskritis"), ("mk", "Кукуш"), ("nb", "Kukës"), ("nl", "Kukës"), ("no", "Kukës"), ("pl", "Obwód Kukës"), ("pt", "Kukës"), ("ro", "Regiunea Kukës"), ("ru", "Кукес"), ("sk", "Kukës"), ("sq", "Qarku i Kukësit"), ("sv", "Kukës prefektur"), ("tr", "Kukës ili"), ("uk", "Кукес"), ("ur", "صوبہ کوکس"), ("vi", "Kukës"), ("zh", "庫克斯州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "Lezhë",
                        country_alpha2: Alpha2::AL,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.78607299999999), longitude: Some(19.6460758), max_latitude: Some(41.8086206), min_latitude: Some(41.7682711), max_longitude: Some(19.6730233), min_longitude: Some(19.6120548)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لجه"), ("be", "Лежа"), ("bg", "Лежа"), ("bs", "Okrug Lješ"), ("ca", "Comtat de Lezhë"), ("ccp", "𑄣\u{11127}𑄏𑄬 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Lezhës"), ("cs", "Lezhë (kraj)"), ("de", "Qark Lezha"), ("el", "Νομός Λεζ"), ("en", "Lezhë County"), ("es", "Condado de Lezhë"), ("et", "Lezhë maakond"), ("eu", "Lezhë konderria"), ("fa", "شهرستان لژه"), ("fi", "Lezhën maakunta"), ("fr", "préfecture de Lezhë"), ("ga", "Contae Lezhë"), ("gl", "Condado de Lezhë"), ("hi", "ल\u{947}ज\u{93c}ह प\u{94d}रा\u{902}त"), ("hr", "Lješki okrug"), ("hu", "Lezha megye"), ("hy", "Լեժա"), ("it", "prefettura di Alessio"), ("ja", "レジャ州"), ("ko", "레저 주"), ("lt", "Ležės apskritis"), ("mk", "Леска"), ("mn", "Лежа хошуу"), ("nb", "Lezhë"), ("nl", "Lezhë"), ("no", "Lezhë"), ("pl", "Obwód Lezha"), ("pt", "Lezhë"), ("ro", "Regiunea Lezhë"), ("ru", "Лежа"), ("sk", "Lezhë"), ("sq", "Qarku i Lezhës"), ("sv", "Lezhë prefektur"), ("tr", "Leç ili"), ("uk", "Лежа"), ("ur", "صوبہ لژہ"), ("vi", "Lezhë"), ("zh", "列澤州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "Dibër",
                        country_alpha2: Alpha2::AL,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5888163), longitude: Some(20.2355647), max_latitude: Some(41.899441), min_latitude: Some(41.26974), max_longitude: Some(20.583229), min_longitude: Some(19.778668)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ديبر"), ("bg", "Дебър"), ("bs", "Okrug Dibr"), ("ca", "comtat de Dibër"), ("ccp", "𑄓\u{1112d}𑄝𑄢\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Dibrës"), ("cs", "Dibrë (kraj)"), ("de", "Qark Dibra"), ("el", "Νομός Ντίμπρα"), ("en", "Dibër County"), ("es", "condado de Dibër"), ("et", "Dibëri maakond"), ("eu", "Dibër konderria"), ("fa", "شهرستان دیبر"), ("fi", "Dibërin maakunta"), ("fr", "préfecture de Dibër"), ("ga", "Contae Dibër"), ("gl", "condado de Dibër"), ("hi", "डिब\u{945}र प\u{94d}रा\u{902}त"), ("hr", "Dibrski okrug"), ("hu", "Dibra megye"), ("hy", "Դիբարի գավառ"), ("it", "prefettura di Dibër"), ("ja", "ディブラ州"), ("ka", "დიბრის ოლქი"), ("ko", "디버르 주"), ("lt", "Diberio apskritis"), ("lv", "Dibras apgabals"), ("mk", "Дебар"), ("nb", "Dibër"), ("nl", "Dibër"), ("no", "Dibër"), ("pa", "ਡਿਬਰ ਕਾਉ\u{a02}ਟੀ"), ("pl", "obwód Dibra"), ("pt", "Dibër"), ("ro", "Regiunea Dibër"), ("ru", "Дибра"), ("sk", "Dibër"), ("sq", "Qarku i Dibrës"), ("sv", "Dibër prefektur"), ("tr", "Debre ili"), ("uk", "Дібер"), ("ur", "صوبہ دیبر"), ("vi", "Dibër"), ("zh", "第巴爾州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "Shkodër",
                        country_alpha2: Alpha2::AL,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0692985), longitude: Some(19.5032559), max_latitude: Some(42.0965346), min_latitude: Some(42.0324324), max_longitude: Some(19.537983), min_longitude: Some(19.3796252)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شكودر"), ("be", "Шкодэр"), ("bg", "Шкодра"), ("bs", "Okrug Skadar"), ("ca", "Comtat de Shkodër"), ("ccp", "𑄇\u{1112e}𑄓𑄢\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Shkodrës"), ("cs", "Shkodër (kraj)"), ("da", "Shkodër"), ("de", "Qark Shkodra"), ("el", "Νομός Σκόδρας"), ("en", "Shkodër County"), ("es", "Condado de Shkodër"), ("et", "Shkodëri maakond"), ("eu", "Shkodër konderria"), ("fa", "شهرستان شکودر"), ("fi", "Shkodërin maakunta"), ("fr", "préfecture de Shkodër"), ("ga", "Contae Shkodër"), ("gl", "Condado de Shkodër"), ("hi", "श\u{94d}कोड\u{947}र प\u{94d}रा\u{902}त"), ("hr", "Skadarski okrug"), ("hu", "Shkodra megye"), ("hy", "Շկոդեր"), ("it", "prefettura di Scutari"), ("ja", "シュコドラ州"), ("ko", "슈코더르 주"), ("lt", "Škoderio apskritis"), ("mk", "Скадар"), ("mn", "Шкодер хошуу"), ("nb", "Shkodër"), ("nl", "Shkodër"), ("no", "Shkodër"), ("pl", "Obwód Szkodra"), ("pt", "Shkodër"), ("ro", "Regiunea Shkodër"), ("ru", "Шкодер"), ("sk", "Shkodër"), ("sq", "Qarku i Shkodrës"), ("sr", "Округ Скадар"), ("sr_Latn", "Okrug Skadar"), ("sv", "Shkodër prefektur"), ("tr", "İşkodra ili"), ("uk", "Шкодер"), ("ur", "صوبہ شکودر"), ("vi", "Shkodër"), ("zh", "士科德州")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "Tiranë",
                        country_alpha2: Alpha2::AL,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.3275459), longitude: Some(19.8186982), max_latitude: Some(41.36684109999999), min_latitude: Some(41.29512330000001), max_longitude: Some(19.8820782), min_longitude: Some(19.7535682)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيرانا"), ("az", "Tirana vilayəti"), ("be", "вобласць Тырана"), ("bg", "Тирана"), ("bs", "Okrug Tirana"), ("ca", "Comtat de Tirana"), ("ccp", "𑄑\u{11128}𑄢\u{11127}𑄚 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Tiranës"), ("cs", "Tiranë (kraj)"), ("de", "Qark Tirana"), ("el", "Νομός Τιράνων"), ("en", "Tirana County"), ("es", "Condado de Tirana"), ("et", "Tirana maakond"), ("eu", "Tirana konderria"), ("fa", "شهرستان تیرانا"), ("fi", "Tiranan maakunta"), ("fr", "préfecture de Tirana"), ("ga", "Contae Tiranë"), ("gl", "Condado de Tirana"), ("hi", "तिराना प\u{94d}रा\u{902}त"), ("hr", "Tiranski okrug"), ("hu", "Tirana megye"), ("hy", "Տիրանայի գավառ"), ("it", "prefettura di Tirana"), ("ja", "ティラナ州"), ("ko", "티라나 주"), ("lt", "Tiranos apskritis"), ("lv", "Tirānas ķarka"), ("mk", "Тирана"), ("mn", "Тиран хошуу"), ("nb", "Tiranë"), ("nl", "Tirana"), ("no", "Tiranë"), ("pl", "Obwód Tirana"), ("pt", "Tirana"), ("ro", "Regiunea Tirana"), ("ru", "Тирана"), ("sk", "Tirana"), ("sq", "Qarku i Tiranës"), ("sv", "Tiranë prefektur"), ("th", "มณฑลต\u{e34}รานา"), ("tr", "Tiran ili"), ("uk", "Тирана"), ("ur", "صوبہ تیرانا"), ("vi", "Tirana"), ("zh", "地拉那州")]),
                        unofficial_name_list: ["Tirana", "Tirana", "Tiranë"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "Vlorë",
                        country_alpha2: Alpha2::AL,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.465), longitude: Some(19.485), max_latitude: Some(40.4912342), min_latitude: Some(40.4103918), max_longitude: Some(19.5100021), min_longitude: Some(19.4530106)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فلوره"), ("be", "Вобласць Влёра"), ("bg", "Вльора"), ("bn", "ভ\u{9cd}ল ভ\u{9cd}লোর ক\u{9be}উন\u{9cd}টি"), ("bs", "Okrug Vlora"), ("ca", "Comtat de Vlorë"), ("ccp", "𑄞\u{11133}𑄣\u{1112e}𑄢𑄬 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Qarku i Vlorës"), ("cs", "Vlora"), ("da", "Vlorë County"), ("de", "Qark Vlora"), ("el", "Νομός Αυλώνα"), ("en", "Vlorë County"), ("es", "Condado de Vlorë"), ("et", "Vlorë maakond"), ("eu", "Vlorë konderria"), ("fa", "شهرستان ولوره"), ("fi", "Vlorën maakunta"), ("fr", "préfecture de Vlorë"), ("ga", "Contae Vlorë"), ("gl", "Condado de Vlorë"), ("gu", "વલોર\u{ac7} કાઉન\u{acd}ટી"), ("hi", "व\u{94d}लोर\u{947} प\u{94d}रा\u{902}त"), ("hr", "Valonski okrug"), ("hu", "Vlora megye"), ("hy", "Վլյորա"), ("id", "Vlorë County"), ("it", "prefettura di Valona"), ("ja", "ヴロラ州"), ("ka", "ვლორის ოლქი"), ("kn", "ವ\u{ccd}ಲೋರ\u{cc6} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "블로러 주"), ("lt", "Vliorės apskritis"), ("lv", "Vļoras apgabals"), ("mk", "Валона"), ("mr", "वोरोरिया"), ("ms", "Vlore County"), ("nb", "Vlorë"), ("nl", "Vlorë"), ("no", "Vlorë"), ("pl", "Obwód Wlora"), ("pt", "Vlorë"), ("ro", "Regiunea Vlorë"), ("ru", "Влёра"), ("si", "ව\u{dd2}ලොරේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vlorë"), ("sq", "Qarku i Vlorës"), ("sv", "Vlorë prefektur"), ("ta", "வல\u{bcd}லோர\u{bcd} கவுண\u{bcd}டி"), ("te", "ల\u{c4b}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลวโลเร"), ("tr", "Avlonya ili"), ("uk", "Вльора"), ("ur", "صوبہ ولورہ"), ("vi", "Vlorë"), ("zh", "夫羅勒州")]),
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
#[cfg(feature = "al")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AL,
        alpha3: Alpha3::ALB,
        address_format: None,
        continent: Continent::Europe,
        country_code: 355,
        currency_code: CurrencyCode::ALL,
        gec: Some(GEC::AL),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ALB),
        iso_long_name: "The Republic of Albania",
        iso_short_name: "Albania",
        official_language_list: ["sq"].to_vec(),
        spoken_language_list: ["sq"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Albanian"),
        number: "008",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "AL",
        unofficial_name_list: ["Albania", "Albanien", "Albanie", "アルバニア", "Albanië"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Albania"),
            ("af", "Albanië"),
            ("ak", "Albania"),
            ("am", "ጐሔባኒ።"),
            ("an", "Albania"),
            ("ar", "ألبانيا"),
            ("as", "আলবেনিয়\u{9be}"),
            ("ay", "Albania"),
            ("az", "Albaniya"),
            ("ba", "Albania"),
            ("be", "Албанія"),
            ("bg", "Албания"),
            ("bi", "Albania"),
            ("bn", "আলবেনিয়\u{9be}"),
            ("bn_IN", "আলবেনিয়\u{9be}"),
            ("br", "Albania"),
            ("bs", "Albanija"),
            ("ca", "Albània"),
            ("ce", "Албани"),
            ("ch", "Albania"),
            ("cs", "Albánie"),
            ("cv", "Албани"),
            ("cy", "Albania"),
            ("da", "Albanien"),
            ("de", "Albanien"),
            ("dv", "އ\u{7a6}ލ\u{7b0}ބ\u{7ad}ނ\u{7a8}އ\u{7a7}"),
            ("dz", "ཨ\u{f71}ལ་བ་ན\u{f72}་ཡ།"),
            ("ee", "Albania"),
            ("el", "Αλβανία"),
            ("en", "Albania"),
            ("eo", "Albanio"),
            ("es", "Albania"),
            ("et", "Albaania"),
            ("eu", "Albania"),
            ("fa", "آلبانی"),
            ("ff", "Albaniya"),
            ("fi", "Albania"),
            ("fo", "Albania"),
            ("fr", "Albanie"),
            ("fy", "Albaanje"),
            ("ga", "An Albáin"),
            ("gl", "Albania"),
            ("gn", "Albania"),
            ("gu", "અલ\u{acd}બ\u{ac7}નિયા"),
            ("gv", "Yn Albaan"),
            ("ha", "Albaniya"),
            ("he", "אלבניה"),
            ("hi", "अल\u{94d}बानिया"),
            ("hr", "Albanija"),
            ("ht", "Albani"),
            ("hu", "Albánia"),
            ("hy", "Ալբանիա"),
            ("ia", "Albania"),
            ("id", "Albania"),
            ("io", "Albania"),
            ("is", "Albanía"),
            ("it", "Albania"),
            ("iu", "Albania"),
            ("ja", "アルバニア"),
            ("ka", "ალბანეთი"),
            ("ki", "Arũmbĩnia"),
            ("kk", "Албания"),
            ("kl", "Albania"),
            ("km", "អាល\u{17cb}បាន\u{17b8}"),
            ("kn", "ಅಲ\u{ccd}ಬೇನ\u{cbf}ಯಾ"),
            ("ko", "알바니아"),
            ("ku", "Arnavût"),
            ("kv", "Албания"),
            ("kw", "Albani"),
            ("ky", "Албания"),
            ("lo", "Albania"),
            ("lt", "Albanija"),
            ("lv", "Albānija"),
            ("mi", "Arapeinia"),
            ("mk", "Албанија"),
            ("ml", "അല\u{d4d}\u{200d}ബേനിയ"),
            ("mn", "Албани"),
            ("mr", "अल\u{94d}ब\u{947}निया"),
            ("ms", "Albania"),
            ("mt", "Albanija"),
            (
                "my",
                "အယ\u{103a}လ\u{103a}ဘေးန\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Arbainiya"),
            ("nb", "Albania"),
            ("ne", "अल\u{94d}बानिया"),
            ("nl", "Albanië"),
            ("nn", "Albania"),
            ("nv", "Dziłigaii Bikéyah"),
            ("oc", "Albania"),
            ("or", "ଅଲ\u{b4d}ବ\u{b3e}ନୀୟ\u{b3e}"),
            ("pa", "ਅਲਬੀਨਾ"),
            ("pi", "अल\u{94d}बानिया"),
            ("pl", "Albania"),
            ("ps", "البانیه"),
            ("pt", "Albânia"),
            ("pt_BR", "Albânia"),
            ("ro", "Albania"),
            ("ru", "Албания"),
            ("rw", "Alubaniya"),
            ("sc", "Albania"),
            ("sd", "Albania"),
            ("si", "ඇල\u{dca}බේන\u{dd2}ය\u{dcf}ව"),
            ("sk", "Albánsko"),
            ("sl", "Albanija"),
            ("so", "Albania"),
            ("sq", "Shqipëri"),
            ("sr", "Албанија"),
            ("sv", "Albanien"),
            ("sw", "Albania"),
            ("ta", "அல\u{bcd}பேனிய\u{bbe}"),
            ("te", "అల\u{c4d}బ\u{c47}న\u{c3f}య\u{c3e}"),
            ("tg", "Албания"),
            ("th", "แอลเบเน\u{e35}ย"),
            ("ti", "አልባኒያ"),
            ("tk", "Albaniýa"),
            ("tl", "Albanya"),
            ("tr", "Arnavutluk"),
            ("tt", "Албаниа"),
            ("ug", "ئالبانىيە"),
            ("uk", "Албанія"),
            ("ur", "البانیا"),
            ("uz", "Albaniya"),
            ("ve", "Albania"),
            ("vi", "An-ba-ni"),
            ("wa", "Albaneye"),
            ("wo", "Albaani"),
            ("xh", "Albania"),
            ("yo", "Albáníà"),
            ("zh_CN", "阿尔巴尼亚"),
            ("zh_HK", "阿爾巴尼亞"),
            ("zh_TW", "阿爾巴尼亞"),
            ("zu", "I-Albaniya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
