// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Bulgaria

#[cfg(all(feature = "bg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::BG;
    pub const ALPHA3: Alpha3 = Alpha3::BGR;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 359;
    pub const CURRENCY_CODE: &str = "BGN";
    pub const GEC: Option<GEC> = Some(GEC::BU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::BUL);
    pub const ISO_SHORT_NAME: &str = "Bulgaria";
    pub const ISO_LONG_NAME: &str = "The Republic of Bulgaria";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["bg"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["bg"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9, 10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Bulgarian");
    pub const NUMBER: &str = "100";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "BG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Bulgaria",
        "Bulgarien",
        "Bulgarie",
        "ブルガリア",
        "Bulgarije",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bulgaria"),
        ("af", "Bulgarye"),
        ("ak", "Bulgaria"),
        ("am", "ቡሔጓሱ።"),
        ("an", "Bulgaria"),
        ("ar", "بلغاريا"),
        ("as", "ব\u{9c1}লগ\u{9be}ৰিয়\u{9be}"),
        ("ay", "Bulgaria"),
        ("az", "Bolqarıstan"),
        ("ba", "Bulgaria"),
        ("be", "Балгарыя"),
        ("bg", "България"),
        ("bi", "Bulgaria"),
        ("bn", "ব\u{9c1}লগ\u{9be}রিয়\u{9be}"),
        ("bn_IN", "ব\u{9c1}লগ\u{9be}রিয়\u{9be}"),
        ("br", "Bulgaria"),
        ("bs", "Bugarska"),
        ("ca", "Bulgària"),
        ("ce", "Болгари"),
        ("ch", "Bulgaria"),
        ("cs", "Bulharsko"),
        ("cv", "Болгари"),
        ("cy", "Bwlgaria"),
        ("da", "Bulgarien"),
        ("de", "Bulgarien"),
        ("dv", "ބ\u{7a6}ލ\u{7b0}ގ\u{7ad}ރ\u{7a8}އ\u{7a7}"),
        ("dz", "བ\u{f71}ལ་ག་ར\u{f72}་ཡ།"),
        ("ee", "Bulgaria"),
        ("el", "Βουλγαρία"),
        ("en", "Bulgaria"),
        ("eo", "Bulgario"),
        ("es", "Bulgaria"),
        ("et", "Bulgaaria"),
        ("eu", "Bulgaria"),
        ("fa", "بلغارستان"),
        ("ff", "Bulgariya"),
        ("fi", "Bulgaria"),
        ("fo", "Bulgaria"),
        ("fr", "Bulgarie"),
        ("fy", "Bulgarije"),
        ("ga", "An Bhulgáir"),
        ("gl", "Bulgaria"),
        ("gn", "Bulgaria"),
        ("gu", "બલ\u{acd}ગ\u{ac7}રિયા"),
        ("gv", "Yn Vulgeyr"),
        ("ha", "Bulgairiya"),
        ("he", "בולגריה"),
        ("hi", "ब\u{941}ल\u{94d}गारिया"),
        ("hr", "Bugarska"),
        ("ht", "Bilgari"),
        ("hu", "Bulgária"),
        ("hy", "Բուլղարիա"),
        ("ia", "Bulgaria"),
        ("id", "Bulgaria"),
        ("io", "Bulgaria"),
        ("is", "Búlgaría"),
        ("it", "Bulgaria"),
        ("iu", "Bulgaria"),
        ("ja", "ブルガリア"),
        ("ka", "ბულგარეთი"),
        ("ki", "Bulgaria"),
        ("kk", "Болгария"),
        ("kl", "Bulgaria"),
        ("km", "ប\u{17ca}\u{17bb}លហ\u{17d2}គារ\u{17b8}"),
        ("kn", "ಬಲ\u{ccd}ಗೇರ\u{cbf}ಯಾ"),
        ("ko", "불가리아"),
        ("ku", "Bulgaristan"),
        ("kv", "Болгария"),
        ("kw", "Bulgari"),
        ("ky", "Болгария"),
        ("lo", "ປະເທດບ\u{eb9}ນກາລ\u{eb5}"),
        ("lt", "Bulgarija"),
        ("lv", "Bulgārija"),
        ("mi", "Purukāria"),
        ("mk", "Бугарија"),
        ("ml", "ബള\u{d4d}\u{200d}ഗേറിയ"),
        ("mn", "Болгар"),
        ("mr", "बल\u{94d}ग\u{947}रिया"),
        ("ms", "Bulgaria"),
        ("mt", "Bulgarija"),
        (
            "my",
            "ဘ\u{1030}လ\u{103a}ဂေးရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Borgeriya"),
        ("nb", "Bulgaria"),
        ("ne", "ब\u{941}लग\u{947}रिया"),
        ("nl", "Bulgarije"),
        ("nn", "Bulgaria"),
        ("nv", "Bálgaa Bikéyah"),
        ("oc", "Bulgaria"),
        ("or", "ବ\u{b41}ଲଗ\u{b3e}ର\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਬ\u{a41}ਲਗਾਰੀਆ"),
        ("pi", "ब\u{941}ल\u{94d}गारिया"),
        ("pl", "Bułgaria"),
        ("ps", "بلغاریه"),
        ("pt", "Bulgária"),
        ("pt_BR", "Bulgária"),
        ("ro", "Bulgaria"),
        ("ru", "Болгария"),
        ("rw", "Buligariya"),
        ("sc", "Bulgaria"),
        ("sd", "بلغاريه"),
        ("si", "බල\u{dca}ගේර\u{dd2}ය\u{dcf}"),
        ("sk", "Bulharsko"),
        ("sl", "Bolgarija"),
        ("so", "Bulgaria"),
        ("sq", "Bullgarië"),
        ("sr", "Бугарска"),
        ("sv", "Bulgarien"),
        ("sw", "Bulgaria"),
        ("ta", "பல\u{bcd}கேரிய\u{bbe}"),
        ("te", "బల\u{c4d}గ\u{c47}ర\u{c3f}య\u{c3e}"),
        ("tg", "Булғория"),
        ("th", "บ\u{e31}ลแกเร\u{e35}ย"),
        ("ti", "ቡልጋሪያ"),
        ("tk", "Bolgariýa"),
        ("tl", "Bulgaria"),
        ("tr", "Bulgaristan"),
        ("tt", "Булgариа"),
        ("ug", "بۇلغارىيە"),
        ("uk", "Болгарія"),
        ("ur", "بلغاریہ"),
        ("uz", "Bolgariya"),
        ("ve", "Bulgaria"),
        ("vi", "Bua-ga-ri"),
        ("wa", "Bulgåreye"),
        ("wo", "Bulgaari"),
        ("xh", "Bulgaria"),
        ("yo", "Bùlgáríà"),
        ("zh_CN", "保加利亚"),
        ("zh_HK", "保加利亞"),
        ("zh_TW", "保加利亞"),
        ("zu", "IBulgariya"),
    ];
    #[cfg(all(feature = "bg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 42.733883;
        pub const LONGITUDE: f64 = 25.48583;
        pub const MAX_LATITUDE: f64 = 44.2153059;
        pub const MAX_LONGITUDE: f64 = 28.7292001;
        pub const MIN_LATITUDE: f64 = 41.2354469;
        pub const MIN_LONGITUDE: f64 = 22.3573446;
        pub const NORTHEAST_LATITUDE: f64 = 44.2153059;
        pub const NORTHEAST_LONGITUDE: f64 = 28.7292001;
        pub const SOUTHWEST_LATITUDE: f64 = 41.2354469;
        pub const SOUTHWEST_LONGITUDE: f64 = 22.3573446;
    }
}
#[cfg(all(feature = "bg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 42.733883,
            longitude: 25.48583,
            max_latitude: 44.2153059,
            max_longitude: 28.7292001,
            min_latitude: 41.2354469,
            min_longitude: 22.3573446,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 44.2153059,
                    longitude: 28.7292001,
                },
                southwest: CountryGeoBound {
                    latitude: 41.2354469,
                    longitude: 22.3573446,
                },
            },
        }
    }
}

#[cfg(all(feature = "bg", feature = "subdivisions"))]
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
                    "01",
                    Subdivision{
                        name: "01",
                        country_alpha2: Alpha2::BG,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0208569), longitude: Some(23.0943385), max_latitude: Some(42.0296711), min_latitude: Some(41.9978951), max_longitude: Some(23.1195894), min_longitude: Some(23.0698939)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بلاغووغراد"), ("az", "Blaqoyevqrad vilayəti"), ("be", "Благаеўградская вобласць"), ("bg", "Област Благоевград"), ("bn", "ব\u{9cd}ল\u{9be}গোভগ\u{9cd}র\u{9be}ড প\u{9cd}রদেশ"), ("bs", "Oblast Blagoevgrad"), ("ca", "Província de Blagoevgrad"), ("ccp", "𑄝\u{11133}𑄣\u{11127}𑄉\u{1112e}𑄠𑄬𑄛\u{11134}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Blagoevgrad"), ("cs", "Blagojevgradská oblast"), ("da", "Blagoevgrad (provins)"), ("de", "Oblast Blagoewgrad"), ("el", "Επαρχία Μπλαγκόεβγκραντ"), ("en", "Blagoevgrad"), ("es", "Blagoevgrad"), ("et", "Blagoevgradi piirkond"), ("eu", "Blagoevgrad probintzia"), ("fa", "استان بلاگووگراد"), ("fi", "Blagoevgradin alue"), ("fr", "Blagoevgrad"), ("ga", "Cúige Blagoevgrad"), ("gu", "બ\u{acd}લાગોવગ\u{acd}ર\u{ac5}ડ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז בלגואבגרד"), ("hi", "ब\u{94d}ल\u{947}गोवग\u{94d}र\u{947}ड प\u{94d}रा\u{902}त"), ("hr", "Oblast Blagoevgrad"), ("hu", "Blagoevgrad megye"), ("hy", "Բլագոևգրադի մարզ"), ("id", "Provinsi Blagoevgrad"), ("it", "Blagoevgrad"), ("ja", "ブラゴエヴグラト州"), ("ka", "ბლაგოევგრადის ოლქი"), ("kn", "ಬ\u{ccd}ಲಾಗೋವ\u{ccd}ಗ\u{ccd}ರಾಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "블라고에브그라드 주"), ("lt", "Blagojevgrado sritis"), ("lv", "Blagojevgrandas apgabals"), ("mk", "Благоевградска област"), ("mn", "Благоевград аймаг"), ("mr", "ब\u{94d}लोगोव\u{94d}हग\u{94d}र\u{945}ड प\u{94d}रा\u{902}त"), ("ms", "Wilayah Blagoevgrad"), ("nb", "Blagoevgrad oblast"), ("nl", "Blagoëvgrad"), ("no", "Blagoevgrad oblast"), ("pl", "Obwód Błagojewgrad"), ("pt", "Blagoevgrad (província)"), ("ro", "Regiunea Blagoevgrad"), ("ru", "Благоевградская область"), ("si", "බ\u{dca}ලගෝඉව\u{dca}ග\u{dca}රඩ\u{dca} පළ\u{dcf}ත"), ("sk", "Blagoevgrad (oblasť)"), ("sl", "Blagoevgrad (okraj)"), ("sq", "Provinca Blagojevgrad"), ("sr", "Благоевградска област"), ("sr_Latn", "Blagoevgradska oblast"), ("sv", "Blagoevgrad (region)"), ("ta", "பலகோயிவ\u{bcd}க\u{bcd}ர\u{bbe}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c4d}ల\u{c3e}గ\u{c4b}వ\u{c4d}\u{200c}గ\u{c4d}ర\u{c3e}డ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บลาโกเอฟกราด"), ("tr", "Yukarı Cuma ili"), ("uk", "Благоєвградська область"), ("ur", "بلاگووگراد صوبہ"), ("vi", "Blagoevgrad (tỉnh)"), ("zh", "布拉格耶夫格勒州")]),
                        unofficial_name_list: ["Blagoevgrad"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::BG,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.50479259999999), longitude: Some(27.4626361), max_latitude: Some(42.6139216), min_latitude: Some(42.4391223), max_longitude: Some(27.5458556), min_longitude: Some(27.3580762)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بورغاس"), ("be", "Бургаская вобласць"), ("bg", "Област Бургас"), ("bn", "ব\u{9c1}রগ\u{9be}স প\u{9cd}রদেশ"), ("bs", "Oblast Burgas"), ("ca", "Província de Burgas"), ("ccp", "𑄝\u{11128}𑄅\u{1112a}𑄉\u{11133}𑄢𑄌\u{11134}"), ("ceb", "Burgas"), ("cs", "Burgaská oblast"), ("da", "Burgas"), ("de", "Oblast Burgas"), ("el", "Επαρχία Μπουργκάς"), ("en", "Burgas"), ("es", "Burgas"), ("et", "Burgasi piirkond"), ("eu", "Burgas probintzia"), ("fa", "استان بورگاس"), ("fi", "Burgasin alue"), ("fr", "Bourgas"), ("ga", "Cúige Burgas"), ("gu", "બર\u{acd}ગસ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז בורגס"), ("hi", "ब\u{941}र\u{94d}गास प\u{94d}रा\u{902}त"), ("hr", "Oblast Burgas"), ("hu", "Burgasz megye"), ("hy", "Բուրգասի մարզ"), ("id", "Provinsi Burgas"), ("it", "Burgas"), ("ja", "ブルガス州"), ("ka", "ბურგასის ოლქი"), ("kn", "ಬರ\u{ccd}ಗಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부르가스 주"), ("lt", "Burgaso sritis"), ("lv", "Burgasas apgabals"), ("mk", "Бургаска област"), ("mn", "Бургас аймаг"), ("mr", "बरग\u{945}स प\u{94d}रा\u{902}त"), ("ms", "Burgas Province"), ("nb", "Burgas oblast"), ("nl", "Boergas"), ("no", "Burgas oblast"), ("pl", "Obwód Burgas"), ("pt", "Burgas"), ("ro", "Regiunea Burgas"), ("ru", "Бургасская область"), ("si", "බර\u{dca}ග\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sk", "Burgas"), ("sl", "Burgas"), ("sq", "Provinca Burgas"), ("sr", "Бургаска област"), ("sr_Latn", "Burgaska oblast"), ("sv", "Burgas"), ("ta", "பிரக\u{bbe}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుర\u{c4d}గ\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เขตปกครองพ\u{e34}เศษเฟเดอร\u{e31}ลแคพ\u{e34}ทอลเทร\u{e4c}ร\u{e34}ทอร\u{e35}"), ("tr", "Burgaz ili"), ("uk", "Бургаська область"), ("ur", "بورگاس صوبہ"), ("uz", "Burgas"), ("vi", "Burgas"), ("zh", "布爾加斯州")]),
                        unofficial_name_list: ["Burgas"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::BG,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2140504), longitude: Some(27.9147333), max_latitude: Some(43.3094528), min_latitude: Some(43.1002294), max_longitude: Some(28.0559078), min_longitude: Some(27.8299093)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فارنا"), ("be", "Варненская вобласць"), ("bg", "Област Варна"), ("bs", "Oblast Varna"), ("ca", "Província de Varna"), ("ccp", "𑄞𑄢\u{11134}𑄚"), ("ceb", "Varna"), ("cs", "Varenská oblast"), ("da", "Varna"), ("de", "Oblast Warna"), ("el", "Επαρχία Βάρνας"), ("en", "Varna"), ("es", "Varna"), ("et", "Varna piirkond"), ("eu", "Varna probintzia"), ("fa", "استان وارنا"), ("fi", "Varnan alue"), ("fr", "Varna"), ("ga", "Cúige Varna"), ("he", "מחוז וארנה"), ("hr", "Oblast Varna"), ("hu", "Várna megye"), ("hy", "Վարնայի մարզ"), ("id", "Provinsi Varna"), ("it", "Varna"), ("ja", "ヴァルナ州"), ("ka", "ვარნის ოლქი"), ("ko", "바르나 주"), ("lt", "Varnos sritis"), ("lv", "Varnas apgabals"), ("mk", "Варненска област"), ("mn", "Варна аймаг"), ("nb", "Varna oblast"), ("nl", "Varna"), ("no", "Varna oblast"), ("pl", "Obwód Warna"), ("pt", "Varna"), ("ro", "Regiunea Varna"), ("ru", "Варненская область"), ("sk", "Varna"), ("sl", "Varna"), ("sq", "Provinca Varna"), ("sr", "Варненска област"), ("sr_Latn", "Varnenska oblast"), ("sv", "Varna"), ("tr", "Varna ili"), ("uk", "Варненська область"), ("ur", "وارنا صوبہ"), ("vi", "Varna"), ("zh", "瓦爾納州")]),
                        unofficial_name_list: ["Varna"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::BG,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0756739), longitude: Some(25.6171514), max_latitude: Some(43.1103064), min_latitude: Some(43.0578391), max_longitude: Some(25.6943964), min_longitude: Some(25.5718485)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة وليكو تارنوو"), ("be", "Вялікатырноўская вобласць"), ("bg", "Област Велико Търново"), ("bn", "ভেলিকো ট\u{9be}র\u{9cd}নোভো"), ("bs", "Oblast Veliko Trnovo"), ("ca", "Província de Veliko Tarnovo"), ("ccp", "𑄞𑄬𑄣\u{11128}𑄇\u{1112e} 𑄑𑄢\u{11134}𑄚\u{11127}𑄞\u{1112e}"), ("ceb", "Oblast Veliko Tŭrnovo"), ("cs", "Velikotarnovská oblast"), ("da", "Veliko Tarnovo (provins)"), ("de", "Oblast Weliko Tarnowo"), ("el", "Επαρχία Βέλικο Τάρνοβο"), ("en", "Veliko Tarnovo"), ("es", "Veliko Tarnovo"), ("et", "Veliko Tărnovo piirkond"), ("eu", "Veliko Tarnovo probintzia"), ("fa", "استان ولیکو ترنوو"), ("fi", "Veliko Tărnovon alue"), ("fr", "Veliko Tarnovo"), ("ga", "Cúige Veliko Tarnovo"), ("gu", "વ\u{ac7}લિકો ટર\u{acd}નોવો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז וליקו טרנובו"), ("hi", "व\u{947}लिको टारनोवो प\u{94d}रा\u{902}त"), ("hr", "Oblast Veliko Trnovo"), ("hu", "Veliko Tarnovo megye"), ("hy", "Վելիկո Տիրնովոյի մարզ"), ("id", "Provinsi Veliko Tarnovo"), ("it", "Veliko Tărnovo"), ("ja", "ヴェリコ・タルノヴォ州"), ("ka", "ველიკო-ტირნოვოს ოლქი"), ("kn", "ವ\u{cc6}ಲ\u{cbf}ಕೊ ಟರ\u{ccd}ನೋವೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "벨리코터르노보 주"), ("lt", "Veliko Tirnovo sritis"), ("lv", "Veliko Tarnovas apgabals"), ("mk", "Великотрновска област"), ("mn", "Велико-Тырново аймаг"), ("mr", "व\u{947}लिको टारनोवो प\u{94d}रा\u{902}त"), ("ms", "Veliko Tarnovo Province"), ("nb", "Veliko Tarnovo oblast"), ("nl", "Veliko Tarnovo"), ("no", "Veliko Tarnovo oblast"), ("pl", "Obwód Wielkie Tyrnowo"), ("pt", "Veliko Tarnovo (província)"), ("ro", "Regiunea Veliko Tărnovo"), ("ru", "Великотырновская область"), ("si", "වෙල\u{dd2}කෝ ටර\u{dca}නොවෝ පළ\u{dcf}ත"), ("sk", "Veliko Tărnovo (oblasť)"), ("sl", "Veliko Trnovo (okraj)"), ("sq", "Provinca Veliko Tërnovo"), ("sr", "Трновска област"), ("sr_Latn", "Trnovska oblast"), ("sv", "Veliko Tarnovo (region)"), ("ta", "வெளிக\u{bcd}கோ தரனோவோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}ల\u{c3f}క\u{c4b} ట\u{c3e}ర\u{c4d}న\u{c3e}వ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวล\u{e35}คอเทอร\u{e4c}โนโว"), ("tr", "Veliko Tırnovo ili"), ("uk", "Великотирновська область"), ("ur", "ویلیکو تارنوو صوبہ"), ("vi", "Veliko Tarnovo (tỉnh)"), ("zh", "大特爾諾沃州")]),
                        unofficial_name_list: ["Veliko Tarnovo"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::BG,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.996159), longitude: Some(22.8679302), max_latitude: Some(44.0154499), min_latitude: Some(43.9437989), max_longitude: Some(22.9156826), min_longitude: Some(22.8390656)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيدين"), ("az", "Vidin vilayəti"), ("be", "Відынская вобласць"), ("bg", "Област Видин"), ("bn", "ভিডিন প\u{9cd}রদেশ"), ("bs", "Oblast Vidin"), ("ca", "Província de Vidin"), ("ccp", "𑄞\u{1112d}𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Oblast Vidin"), ("cs", "Vidinská oblast"), ("da", "Vidin (provins)"), ("de", "Oblast Widin"), ("el", "Επαρχία Βιδινίου"), ("en", "Vidin"), ("es", "Vidin"), ("et", "Vidini piirkond"), ("eu", "Vidin probintzia"), ("fa", "استان ویدین"), ("fi", "Vidinin alue"), ("fr", "Vidin"), ("ga", "Cúige Vidin"), ("gu", "વિડીન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז וידין"), ("hi", "विदीन प\u{94d}रा\u{902}त"), ("hr", "Oblast Vidin"), ("hu", "Vidin megye"), ("hy", "Վիդինի մարզ"), ("id", "Provinsi Vidin"), ("it", "Vidin"), ("ja", "ヴィディン州"), ("ka", "ვიდინის ოლქი"), ("kn", "ವ\u{cbf}ಡ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비딘 주"), ("lt", "Vidino sritis"), ("lv", "Vidinas apgabals"), ("mk", "Видинска област"), ("mn", "Видин аймаг"), ("mr", "विडी\u{902} प\u{94d}रा\u{902}त"), ("ms", "Vidin Province"), ("nb", "Vidin oblast"), ("nl", "Vidin"), ("no", "Vidin oblast"), ("pl", "Obwód Widin"), ("pt", "Vidin (província)"), ("ro", "Regiunea Vidin"), ("ru", "Видинская область"), ("si", "ව\u{dd2}ඩ\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sk", "Vidin (oblasť)"), ("sl", "Vidin (okraj)"), ("sq", "Provinca Vidin"), ("sr", "Видинска област"), ("sr_Latn", "Vidinska oblast"), ("sv", "Vidin (region)"), ("ta", "விடின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3f}డ\u{c4d}ల\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดว\u{e34}ด\u{e34}น"), ("tr", "Vidin ili"), ("uk", "Видинська область"), ("ur", "ویدین صوبہ"), ("vi", "Vidin (tỉnh)"), ("zh", "維丁州")]),
                        unofficial_name_list: ["Vidin"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::BG,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2102375), longitude: Some(23.5528803), max_latitude: Some(43.2297109), min_latitude: Some(43.1850586), max_longitude: Some(23.5920197), min_longitude: Some(23.5066083)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فراتسا"), ("az", "Vratsa vilayəti"), ("be", "Урачанская вобласць"), ("bg", "Област Враца"), ("bn", "ভ\u{9cd}র\u{9be}স\u{9be} প\u{9cd}রদেশ"), ("bs", "Oblast Vraca"), ("ca", "Província de Vratsa"), ("ccp", "𑄞\u{11133}𑄢𑄖\u{11134}𑄥"), ("ceb", "Oblast Vratsa"), ("cs", "Vracká oblast"), ("da", "Vratsa (provins)"), ("de", "Oblast Wraza"), ("el", "Βράτσα"), ("en", "Vratsa"), ("es", "Vratsa"), ("et", "Vraca piirkond"), ("eu", "Vratsa probintzia"), ("fa", "استان وراتسا"), ("fi", "Vratsan alue"), ("fr", "Vratsa"), ("ga", "Cúige Vratsa"), ("gu", "વ\u{acd}રતસા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז וראצה"), ("hi", "व\u{94d}रतसा प\u{94d}रा\u{902}त"), ("hr", "Oblast Vraca"), ("hu", "Vraca megye"), ("hy", "Վրաչայի մարզ"), ("id", "Provinsi Vratsa"), ("it", "Vraca"), ("ja", "ヴラツァ州"), ("ka", "ვრაცის ოლქი"), ("kn", "ವ\u{ccd}ರಾಟ\u{ccd}ಸಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "브라차 주"), ("ky", "Догдурлук аймак"), ("lt", "Vracos sritis"), ("lv", "Vracas apgabals"), ("mk", "Врачанска област"), ("mn", "Враца аймаг"), ("mr", "व\u{94d}रतसा प\u{94d}रा\u{902}त"), ("ms", "Vratsa Province"), ("nb", "Vratsa oblast"), ("nl", "Vratsa"), ("no", "Vratsa oblast"), ("pl", "Obwód Wraca"), ("pt", "Vratsa (província)"), ("ro", "Regiunea Vrața"), ("ru", "Врачанская область"), ("si", "ව\u{dca}රස\u{dca}ට\u{dcf} කල\u{dcf}පය"), ("sk", "Vraca (oblasť)"), ("sl", "Vraca (okraj)"), ("sq", "Provinca Vraca"), ("sr", "Врачанска област"), ("sr_Latn", "Vračanska oblast"), ("sv", "Vratsa (region)"), ("ta", "வரட\u{bcd}ச\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c4d}ర\u{c3e}ట\u{c4d}స\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวราคา"), ("tr", "Vratsa ili"), ("uk", "Врачанська область"), ("ur", "وراتسا صوبہ"), ("vi", "Vratsa (tỉnh)"), ("zh", "弗拉察州")]),
                        unofficial_name_list: ["Vratsa"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::BG,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.8742212), longitude: Some(25.3186837), max_latitude: Some(42.9150995), min_latitude: Some(42.78276880000001), max_longitude: Some(25.3884394), min_longitude: Some(25.262113)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غابرووو"), ("be", "Габроўская вобласць"), ("bg", "Област Габрово"), ("bn", "গ\u{9cd}য\u{9be}ব\u{9cd}রোভো প\u{9cd}রদেশ"), ("bs", "Oblast Gabrovo"), ("ca", "Província de Gabrovo"), ("ccp", "𑄉\u{11133}𑄠𑄝\u{11133}𑄢𑄞\u{1112e}"), ("ceb", "Gabrovo"), ("cs", "Gabrovská oblast"), ("da", "Gabrovo (provins)"), ("de", "Oblast Gabrowo"), ("el", "Γκάμπροβο"), ("en", "Gabrovo"), ("es", "Gabrovo"), ("et", "Gabrovo piirkond"), ("eu", "Gabrovo"), ("fa", "استان گابرووو"), ("fi", "Gabrovon alue"), ("fr", "Gabrovo"), ("ga", "Cúige Gabrovo"), ("gu", "ગ\u{ac7}બ\u{acd}રોવો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז גברובו"), ("hi", "ग\u{948}ब\u{94d}रोवो प\u{94d}रा\u{902}त"), ("hr", "Oblast Gabrovo"), ("hu", "Gabrovo megye"), ("hy", "Գաբրովոյի մարզ"), ("id", "Provinsi Gabrovo"), ("it", "Gabrovo"), ("ja", "ガブロヴォ州"), ("ka", "გაბროვოს ოლქი"), ("kn", "ಗ\u{ccd}ಯಾಬ\u{ccd}ರೊವೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "가브로보 주"), ("lt", "Gabrovo sritis"), ("lv", "Gabrovas apgabals"), ("mk", "Габровска област"), ("mn", "Габрово аймаг"), ("mr", "ग\u{945}बरो प\u{94d}रा\u{902}त"), ("ms", "Gabrovo Province"), ("nb", "Gabrovo oblast"), ("nl", "Gabrovo"), ("no", "Gabrovo oblast"), ("pl", "Obwód Gabrowo"), ("pt", "Gabrovo (província)"), ("ro", "Regiunea Gabrovo"), ("ru", "Габровская область"), ("si", "ගරබෝවෝ පළ\u{dcf}ත"), ("sk", "Gabrovo (oblasť)"), ("sl", "Gabrovo (okraj)"), ("sq", "Provinca Gabrovo"), ("sr", "Габровска област"), ("sr_Latn", "Gabrovska oblast"), ("sv", "Gabrovo (region)"), ("ta", "கப\u{bcd}ரோவோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3e}బ\u{c4d}ర\u{c4b}వ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาโบรโว"), ("tr", "Gabrova ili"), ("uk", "Габровська область"), ("ur", "گابروو صوبہ"), ("vi", "Gabrovo (tỉnh)"), ("zh", "加布羅沃州")]),
                        unofficial_name_list: ["Gabrovo"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::BG,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.57259), longitude: Some(27.8272606), max_latitude: Some(43.608704), min_latitude: Some(43.54184679999999), max_longitude: Some(27.8607443), min_longitude: Some(27.763096)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة دوبريتش"), ("be", "Дабрыцкая вобласць"), ("bg", "Област Добрич"), ("bn", "ডোবরিক প\u{9cd}রদেশ"), ("bs", "Oblast Dobrič"), ("ca", "Província de Dobritx"), ("ccp", "𑄓\u{11127}𑄝\u{11133}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Oblast Dobrich"), ("cs", "Dobričská oblast"), ("da", "Dobritj (provins)"), ("de", "Oblast Dobritsch"), ("el", "Ντόμπριτς"), ("en", "Dobrich"), ("es", "Dobrich"), ("et", "Dobriči piirkond"), ("eu", "Dobritx probintzia"), ("fa", "استان دوبریچ"), ("fi", "Dobritšin alue"), ("fr", "Dobritch"), ("ga", "Cúige Dobrich"), ("gu", "ડોબ\u{acd}રિચ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז דובריץ׳"), ("hi", "डोब\u{94d}रिच प\u{94d}रा\u{902}त"), ("hr", "Oblast Dobrič"), ("hu", "Dobrics megye"), ("hy", "Դոբրիչի մարզ"), ("id", "Provinsi Dobrich"), ("it", "Dobrič"), ("ja", "ドブリチ州"), ("ka", "დობრიჩის ოლქი"), ("kn", "ಡೊಬ\u{ccd}ರ\u{cbf}ಚ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "도브리치 주"), ("lt", "Dobričo sritis"), ("lv", "Dobričas apgabals"), ("mk", "Добричка област"), ("mn", "Добрич аймаг"), ("mr", "डोब\u{94d}रिच प\u{94d}रा\u{902}त"), ("ms", "Wilayah Dobrich"), ("nb", "Dobritsj oblast"), ("nl", "Dobritsj"), ("no", "Dobritsj oblast"), ("pl", "Obwód Dobricz"), ("pt", "Dobrich (província)"), ("ro", "Regiunea Dobrici"), ("ru", "Добричская область"), ("si", "ඩොබ\u{dca}\u{200d}ර\u{dd2}ච\u{dca} පළ\u{dcf}ත"), ("sk", "Dobrič (oblasť)"), ("sl", "Dobrič (okraj)"), ("sq", "Provinca Dobriç"), ("sr", "Добричка област"), ("sr_Latn", "Dobrička oblast"), ("sv", "Dobritj (region)"), ("ta", "டோபிரிச\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "డ\u{c3e}బ\u{c4d}ర\u{c3f}చ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโดบร\u{e34}ช"), ("tr", "Dobriç ili"), ("uk", "Добрицька область"), ("ur", "دوبریچ صوبہ"), ("vi", "Dobrich (tỉnh)"), ("zh", "多布里奇州")]),
                        unofficial_name_list: ["Dobrich"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::BG,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6338439), longitude: Some(25.3777119), max_latitude: Some(41.6597227), min_latitude: Some(41.5978213), max_longitude: Some(25.4243002), min_longitude: Some(25.3410976)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كرجالي"), ("be", "Кірджалійская вобласць"), ("bg", "Област Кърджали"), ("bn", "ক\u{9be}র\u{9cd}ডঝ\u{9be}লি প\u{9cd}রদেশ"), ("bs", "Oblast Krdžali"), ("ca", "Província de Kardzhali"), ("ccp", "𑄇𑄢\u{11133}𑄓\u{11134}𑄎𑄣\u{11128}"), ("ceb", "Kŭrdzhali"), ("cs", "Kardžalijská oblast"), ("cy", "Rhanbarth Kardzhali"), ("da", "Kardzjali (provins)"), ("de", "Oblast Kardschali"), ("el", "Κάρτζαλι"), ("en", "Kardzhali"), ("es", "Kardzhali"), ("et", "Kǎrdžali piirkond"), ("eu", "Kardzhali probintzia"), ("fa", "استان کرجالی"), ("fi", "Kărdžalin alue"), ("fr", "Kardjali"), ("ga", "Cúige Kardzhali"), ("gu", "કાર\u{acd}દઝાલી પ\u{acd}રા\u{a82}ત"), ("he", "מחוז קרדז׳אלי"), ("hi", "कर\u{94d}दझली प\u{94d}रा\u{902}त"), ("hr", "Oblast Krdžali"), ("hu", "Kardzsali megye"), ("hy", "Կարջալիի մարզ"), ("id", "Provinsi Kardzhali"), ("it", "Kărdžali"), ("ja", "クルジャリ州"), ("ka", "კირჯალის ოლქი"), ("kn", "ಕರ\u{ccd}ಡ\u{ccd}ಝಾಲ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "커르잘리 주"), ("lt", "Kirdžalų sritis"), ("lv", "Kardžali apgabals"), ("mk", "Крџалиска област"), ("mn", "Кыржали аймаг"), ("mr", "कार\u{94d}द\u{94d}झली प\u{94d}रा\u{902}त"), ("ms", "Kardzhali Province"), ("nb", "Kardzhali provins"), ("nl", "Kardzjali (oblast)"), ("no", "Kardzhali provins"), ("pl", "Obwód Kyrdżali"), ("pt", "Kardzhali (província)"), ("ro", "Regiunea Kărdjali"), ("ru", "Кырджалийская область"), ("si", "කර\u{dca}ද\u{dca}ශ\u{dcf}ල\u{dd2} පළ\u{dcf}ත"), ("sk", "Kărdžali (oblasť)"), ("sl", "Krdžali (okraj)"), ("sq", "Provinca Kërxhali"), ("sr", "Крџалијска област"), ("sr_Latn", "Krdžalijska oblast"), ("sv", "Kardzjali (region)"), ("ta", "க\u{bbe}ர\u{bcd}டச\u{bbe}லி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ర\u{c4d}డ\u{200c}జ\u{c3e}ల\u{c48} ప\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดการ\u{e4c}ดซาล\u{e35}"), ("tr", "Kırcaali ili"), ("uk", "Кирджалійська область"), ("ur", "کاردژالی صوبہ"), ("vi", "Kardzhali (tỉnh)"), ("zh", "克爾賈利州")]),
                        unofficial_name_list: ["Kardzhali"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::BG,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.2868817), longitude: Some(22.6939308), max_latitude: Some(42.3022897), min_latitude: Some(42.2711228), max_longitude: Some(22.7273332), min_longitude: Some(22.6594581)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كيوستنديل"), ("be", "Кюсцендыльская вобласць"), ("bg", "Област Кюстендил"), ("bn", "ক\u{9c1}স\u{9cd}টেন\u{9cd}ডিল প\u{9cd}রদেশ"), ("bs", "Oblast Ćustendil"), ("ca", "Província de Kiustendil"), ("ccp", "𑄇\u{1112d}𑄠𑄬𑄌\u{11134}𑄑𑄬𑄚\u{11134}𑄓\u{11128}𑄣\u{11134}"), ("ceb", "Oblast Kyustendil"), ("cs", "Kjustendilská oblast"), ("da", "Kjustendil (provins)"), ("de", "Oblast Kjustendil"), ("el", "Επαρχία Κιουστεντίλ"), ("en", "Kyustendil"), ("es", "Kyustendil"), ("et", "Kjustendili piirkond"), ("eu", "Kiustendil probintzia"), ("fa", "استان کیوستندیل"), ("fi", "Kjustendilin alue"), ("fr", "Kyoustendil"), ("ga", "Cúige Kyustendil"), ("gu", "ક\u{acd}ય\u{ac1}ન\u{acd}સ\u{acd}ટ\u{ac7}ન\u{acd}ડીલ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז קיוסטנדיל"), ("hi", "क\u{94d}य\u{942}सट\u{947}\u{902}डिल प\u{94d}रा\u{902}त"), ("hr", "Oblast Ćustendil"), ("hu", "Kjusztendil megye"), ("hy", "Կյուստենդիլի մարզ"), ("id", "Provinsi Kyustendil"), ("it", "Kjustendil"), ("ja", "キュステンディル州"), ("ka", "კიუსტენდილის ოლქი"), ("kn", "ಕ\u{ccd}ಯ\u{cc2}ಸ\u{ccd}ಟ\u{cc6}ಂಡ\u{cbf}ಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "큐스텐딜 주"), ("lt", "Kiustendilo sritis"), ("lv", "Kujstendilas province"), ("mk", "Ќустендилска област"), ("mr", "क\u{94d}य\u{942}स\u{94d}ट\u{947}\u{902}डील प\u{94d}रा\u{902}त"), ("ms", "Kyustendil Province"), ("nb", "Kyustendil provins"), ("nl", "Kjoestendil"), ("no", "Kyustendil provins"), ("pl", "Obwód Kiustendił"), ("pt", "Kyustendil (província)"), ("ro", "Regiunea Kiustendil"), ("ru", "Кюстендилская область"), ("si", "ක\u{dca}ය\u{dd4}ස\u{dca}ටේන\u{dca}ඩ\u{dd2}ල\u{dca} පළ\u{dcf}ත"), ("sk", "Kjustendil (oblasť)"), ("sl", "Kjustendil (okraj)"), ("sq", "Provinca Kjustendill"), ("sr", "Ћустендилска област"), ("sr_Latn", "Ćustendilska oblast"), ("sv", "Kjustendil (region)"), ("ta", "க\u{bcd}யுஸ\u{bcd}டேண\u{bcd}டில\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}యూస\u{c4d}ట\u{c46}ండ\u{c3f}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เม\u{e37}องค\u{e38}ยสเตนด\u{e34}ล"), ("tr", "Köstendil ili"), ("uk", "Кюстендильська область"), ("ur", "کیوستندیل صوبہ"), ("vi", "Kyustendil (tỉnh)"), ("zh", "丘斯滕迪爾州")]),
                        unofficial_name_list: ["Kjustendil"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::BG,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1369534), longitude: Some(24.7141906), max_latitude: Some(43.1819797), min_latitude: Some(43.1069685), max_longitude: Some(24.7467215), min_longitude: Some(24.6753677)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لووتش"), ("be", "Ловечская вобласць"), ("bg", "Област Ловеч"), ("bn", "লোভেচ প\u{9cd}রদেশ"), ("bs", "Oblast Loveč"), ("ca", "Província de Lovetx"), ("ccp", "𑄣\u{1112e}𑄞𑄬𑄌\u{11134}"), ("ceb", "Lovech"), ("cs", "Lovečská oblast"), ("da", "Lovetj"), ("de", "Oblast Lowetsch"), ("el", "Λόβετς"), ("en", "Lovech"), ("es", "Lovech"), ("et", "Loveči piirkond"), ("eu", "Lovetx probintzia"), ("fa", "استان لووچ"), ("fi", "Lovetšin alue"), ("fr", "Lovetch"), ("ga", "Cúige Lovech"), ("gu", "લોવ\u{ac7}ચ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז לובץ׳"), ("hi", "लोव\u{947}क प\u{94d}रा\u{902}त"), ("hr", "Oblast Loveč"), ("hu", "Lovecs megye"), ("hy", "Լովեչի մարզ"), ("id", "Provinsi Lovetch"), ("it", "Loveč"), ("ja", "ロヴェチ州"), ("ka", "ლოვეჩის ოლქი"), ("kn", "ಲವ\u{ccd}ಚ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "로베치 주"), ("lt", "Lovečo sritis"), ("lv", "Lovečas apgabals"), ("mk", "Ловечка област"), ("mn", "Ловеч аймаг"), ("mr", "लवच प\u{94d}रा\u{902}त"), ("ms", "Lovech Province"), ("nb", "Lovetsj oblast"), ("nl", "Lovetsj"), ("no", "Lovetsj oblast"), ("pl", "Obwód Łowecz"), ("pt", "Lovech"), ("ro", "Regiunea Loveci"), ("ru", "Ловечская область"), ("si", "ලොවෙච\u{dca} පළ\u{dcf}ත"), ("sk", "Loveč"), ("sl", "Loveč"), ("sq", "Provinca Loveç"), ("sr", "Ловечка област"), ("sr_Latn", "Lovečka oblast"), ("sv", "Lovetj"), ("ta", "லோவ\u{bcd}ச\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c4b}వ\u{c46}చ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโลเวช"), ("tr", "Lofça ili"), ("uk", "Ловецька область"), ("ur", "لوویچ صوبہ"), ("vi", "Lovech"), ("zh", "洛維奇州")]),
                        unofficial_name_list: ["Lovech"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::BG,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.4085161), longitude: Some(23.2257292), max_latitude: Some(43.45198269999999), min_latitude: Some(43.3764219), max_longitude: Some(23.2474243), min_longitude: Some(23.2073812)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مونتانا"), ("be", "Мантанская вобласць"), ("bg", "Област Монтана"), ("bs", "Oblast Montana"), ("ca", "Província de Montana"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑𑄚"), ("ceb", "Oblast Montana"), ("cs", "Montanská oblast"), ("da", "Montana (provins)"), ("de", "Oblast Montana"), ("el", "Επαρχία Μοντάνα"), ("en", "Montana"), ("es", "Provincia de Montana"), ("et", "Montana piirkond"), ("eu", "Montana probintzia"), ("fa", "استان مونتانا"), ("fi", "Montanan alue"), ("fr", "Montana"), ("ga", "Cúige Montana"), ("he", "מחוז מונטנה"), ("hr", "Oblast Montana"), ("hu", "Montana megye"), ("hy", "Մոնտանայի մարզ"), ("id", "Provinsi Montana"), ("it", "Montana"), ("ja", "モンタナ州 (ブルガリア)"), ("ka", "მონტანის ოლქი"), ("ko", "몬타나 주 (불가리아)"), ("lt", "Montanos sritis"), ("mk", "Монтанска област"), ("mn", "Монтана аймаг"), ("nb", "Montana oblast"), ("nl", "Montana"), ("no", "Montana oblast"), ("pl", "Obwód Montana"), ("pt", "Montana (província)"), ("ro", "Regiunea Montana"), ("ru", "Монтанская область"), ("sk", "Montana (oblasť)"), ("sl", "Montana (okraj)"), ("sq", "Provinca Montana"), ("sr", "Монтанска област"), ("sr_Latn", "Montanska oblast"), ("sv", "Montana (region)"), ("tr", "Montana ili"), ("uk", "Монтанська область"), ("ur", "مونٹانا صوبہ"), ("vi", "Montana (tỉnh)"), ("zh", "蒙塔納州")]),
                        unofficial_name_list: ["Montana"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::BG,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.1927654), longitude: Some(24.3335662), max_latitude: Some(42.2142634), min_latitude: Some(42.1632603), max_longitude: Some(24.3626333), min_longitude: Some(24.2980485)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بازارجيك"), ("be", "Пазарджыкская вобласць"), ("bg", "Област Пазарджик"), ("bn", "প\u{9be}জ\u{9be}রদঝিক প\u{9cd}রদেশ"), ("bs", "Oblast Pazardžik"), ("ca", "Província de Pazardzhik"), ("ccp", "𑄛\u{11127}𑄎𑄢\u{11133}𑄓\u{11134}𑄏\u{11128}𑄇\u{11134}"), ("ceb", "Pazardzhik"), ("cs", "Pazardžická oblast"), ("da", "Pazardsjik (provins)"), ("de", "Oblast Pasardschik"), ("el", "Πάζαρτζικ"), ("en", "Pazardzhik"), ("es", "Pazardzhik"), ("et", "Pazardžiki piirkond"), ("eu", "Pazardzhik probintzia"), ("fa", "استان پازارجیک"), ("fi", "Pazardžikin alue"), ("fr", "Pazardjik"), ("ga", "Cúige Pazardzhik"), ("gu", "પ\u{ac7}ઝાર\u{acd}ડઝિક પ\u{acd}રા\u{a82}ત"), ("he", "מחוז פאזארדז׳יק"), ("hi", "पज\u{93c}ार\u{94d}दज\u{94d}हिक प\u{94d}रा\u{902}त"), ("hr", "Oblast Pazardžik"), ("hu", "Pazardzsik megye"), ("hy", "Պազարջիկի մարզ"), ("id", "Provinsi Pazardzhik"), ("it", "Pazardžik"), ("ja", "パザルジク州"), ("ka", "პაზარჯიკის ოლქი"), ("kn", "ಪಝಾರ\u{ccd}ಝ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파자르지크 주"), ("lt", "Pazardžiko sritis"), ("lv", "Pazardžikas apgabals"), ("mk", "Пазарџичка област"), ("mn", "Пазаржик аймаг"), ("mr", "प\u{945}झार\u{94d}डझिक प\u{94d}रा\u{902}त"), ("ms", "Pazardzhik Province"), ("nb", "Pazardzhik provins"), ("nl", "Pazardzjik"), ("no", "Pazardzhik provins"), ("pl", "Obwód Pazardżik"), ("pt", "Pazardzhik"), ("ro", "Regiunea Pazargik"), ("ru", "Пазарджикская область"), ("si", "පසර\u{dca}ඩ\u{dca}ස\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sk", "Pazardžik (oblasť)"), ("sl", "Pazardžik (okraj)"), ("sq", "Provinca Pazarxhik"), ("sr", "Пазарџичка област"), ("sr_Latn", "Pazardžička oblast"), ("sv", "Pazardzjik (oblast)"), ("ta", "பச\u{bbe}ர\u{bcd}டஜிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పజ\u{c3e}ర\u{c4d}డ\u{c3f}జ\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพาซาร\u{e4c}ช\u{e34}ค"), ("tr", "Pazarcık ili"), ("uk", "Пазарджицька область"), ("ur", "پازارجیک صوبہ"), ("vi", "Pazardzhik (tỉnh)"), ("zh", "帕扎爾吉克州")]),
                        unofficial_name_list: ["Pazardzhik"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::BG,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6051862), longitude: Some(23.0378368), max_latitude: Some(42.6243972), min_latitude: Some(42.5681881), max_longitude: Some(23.1273676), min_longitude: Some(22.9755276)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة برنيك"), ("be", "Пернікская вобласць"), ("bg", "Област Перник"), ("bn", "পসর\u{9cd}নিক প\u{9cd}রদেশ"), ("bs", "Oblast Pernik"), ("ca", "Província de Pernik"), ("ccp", "𑄛𑄢\u{11134}𑄚\u{11128}𑄇\u{11134}"), ("ceb", "Pernik"), ("cs", "Pernická oblast"), ("da", "Pernik (provins)"), ("de", "Oblast Pernik"), ("el", "Πέρνικ"), ("en", "Pernik"), ("es", "Pernik"), ("et", "Perniki piirkond"), ("eu", "Pernik probintzia"), ("fa", "استان پرنیک"), ("fi", "Pernikin alue"), ("fr", "Pernik"), ("ga", "Cúige Pernik"), ("gu", "પર\u{acd}નિક પ\u{acd}રા\u{a82}ત"), ("he", "מחוז פרניק"), ("hi", "पर\u{94d}निक प\u{94d}रा\u{902}त"), ("hr", "Oblast Pernik"), ("hu", "Pernik megye"), ("hy", "Պերնիկի մարզ"), ("id", "Provinsi Pernik"), ("it", "Pernik"), ("ja", "ペルニク州"), ("ka", "პერნიკის ოლქი"), ("kn", "ಪ\u{cc6}ರ\u{ccd}ನ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "페르니크 주"), ("lt", "Perniko sritis"), ("lv", "Pernikas apgabals"), ("mk", "Перничка област"), ("mn", "Перник аймаг"), ("mr", "पर\u{94d}निक प\u{94d}रा\u{902}त"), ("ms", "Pernik Province"), ("nb", "Pernik oblast"), ("nl", "Pernik"), ("no", "Pernik oblast"), ("pl", "Obwód Pernik"), ("pt", "Pernik (província)"), ("ro", "Regiunea Pernik"), ("ru", "Перникская область"), ("si", "පර\u{dca}න\u{dd2}ක\u{dca} පල\u{dcf}ත"), ("sk", "Pernik (oblasť)"), ("sl", "Pernik (okraj)"), ("sq", "Provinca Pernik"), ("sr", "Перничка област"), ("sr_Latn", "Pernička oblast"), ("sv", "Pernik (region)"), ("ta", "பெர\u{bcd}னிக\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c46}ర\u{c4d}న\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเพร\u{e4c}น\u{e34}ก"), ("tr", "Pernik ili"), ("uk", "Перницька область"), ("ur", "پیرنک صوبہ"), ("vi", "Pernik (tỉnh)"), ("zh", "佩爾尼克州")]),
                        unofficial_name_list: ["Pernik"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::BG,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.4170423), longitude: Some(24.6066847), max_latitude: Some(43.4504952), min_latitude: Some(43.3918598), max_longitude: Some(24.6600647), min_longitude: Some(24.5690017)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلفن"), ("be", "Плевенская вобласць"), ("bg", "Област Плевен"), ("bn", "প\u{9cd}লেভেন প\u{9cd}রদেশ"), ("bs", "Oblast Pleven"), ("ca", "Província de Pleven"), ("ccp", "𑄛\u{11133}𑄣𑄬𑄞𑄬𑄚\u{11134}"), ("ceb", "Obshtina Pleven"), ("cs", "Plevenská oblast"), ("da", "Pleven (provins)"), ("de", "Oblast Plewen"), ("el", "Πλέβεν"), ("en", "Pleven"), ("es", "Pleven"), ("et", "Pleveni piirkond"), ("eu", "Pleven probintzia"), ("fa", "استان پلون"), ("fi", "Plevenin alue"), ("fr", "Pleven"), ("ga", "Cúige Pleven"), ("gu", "પ\u{acd}લ\u{ac7}વ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז פלבן"), ("hi", "प\u{94d}ल\u{947}व\u{947}न प\u{94d}रा\u{902}त"), ("hr", "Oblast Pleven"), ("hu", "Pleven megye"), ("hy", "Պլևենի մարզ"), ("id", "Provinsi Pleven"), ("it", "Pleven"), ("ja", "プレヴェン州"), ("ka", "პლევენის ოლქი"), ("kn", "ಪ\u{ccd}ಲ\u{cc6}ವ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "플레벤 주"), ("lt", "Pleveno sritis"), ("lv", "Plevenas apgabals"), ("mk", "Плевенска област"), ("mn", "Плевен аймаг"), ("mr", "प\u{94d}ल\u{947}व\u{94d}ह\u{947}\u{902} प\u{94d}रा\u{902}त"), ("ms", "Pleven Province"), ("nb", "Pleven oblast"), ("nl", "Pleven"), ("no", "Pleven oblast"), ("pl", "Obwód Plewen"), ("pt", "Pleven (província)"), ("ro", "Regiunea Plevna"), ("ru", "Плевенская область"), ("si", "ප\u{dca}ලෙවෙන\u{dca} පළ\u{dcf}ත"), ("sk", "Pleven (oblasť)"), ("sl", "Pleven (okraj)"), ("sq", "Provinca Pleven"), ("sr", "Плевенска област"), ("sr_Latn", "Plevenska oblast"), ("sv", "Pleven (region)"), ("ta", "ப\u{bcd}லெவேன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ల\u{c46}వ\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพล\u{e35}เวน"), ("tr", "Plevne ili"), ("uk", "Плевенська область"), ("ur", "پلیوین صوبہ"), ("vi", "Pleven (tỉnh)"), ("zh", "普列文州")]),
                        unofficial_name_list: ["Pleven"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::BG,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.1354079), longitude: Some(24.7452904), max_latitude: Some(42.1982332), min_latitude: Some(42.0900086), max_longitude: Some(24.8240283), min_longitude: Some(24.6577202)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلووديو"), ("be", "Плоўдзіўская вобласць"), ("bg", "Област Пловдив"), ("bn", "প\u{9cd}লোভদিভ প\u{9cd}রদেশ"), ("bs", "Oblast Plovdiv"), ("ca", "Província de Plovdiv"), ("ccp", "𑄛\u{11133}𑄣\u{1112e}𑄛\u{11134}𑄓\u{11128}𑄛\u{11134}"), ("ceb", "Plovdiv"), ("cs", "Plovdivská oblast"), ("da", "Plovdiv"), ("de", "Oblast Plowdiw"), ("el", "Επαρχία Φιλιππούπολης"), ("en", "Plovdiv"), ("es", "Plovdiv"), ("et", "Plovdivi piirkond"), ("eu", "Plovdiv probintzia"), ("fa", "استان پلوودیو"), ("fi", "Plovdivin alue"), ("fr", "Plovdiv"), ("ga", "Cúige Plovdiv"), ("gu", "પ\u{acd}લોવડિવ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז פלובדיב"), ("hi", "प\u{94d}लोवदीव प\u{94d}रा\u{902}त"), ("hr", "Oblast Plovdiv"), ("hu", "Plovdiv megye"), ("hy", "Պլովդիվի մարզ"), ("id", "Provinsi Plovdiv"), ("it", "Plovdiv"), ("ja", "プロヴディフ州"), ("ka", "პლოვდივის ოლქი"), ("kn", "ಪ\u{ccd}ಲೋವ\u{ccd}ಡ\u{cbf}ವ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "플로브디프 주"), ("lt", "Plovdivo sritis"), ("lv", "Plovdivas apgabals"), ("mk", "Пловдивска област"), ("mn", "Пловдив аймаг"), ("mr", "प\u{94d}लोवदीव\u{94d}ह प\u{94d}रा\u{902}त"), ("ms", "Plovdiv Province"), ("nb", "Plovdiv oblast"), ("nl", "Plovdiv"), ("no", "Plovdiv oblast"), ("pl", "Obwód Płowdiw"), ("pt", "Plovdiv"), ("ro", "Regiunea Plovdiv"), ("ru", "Пловдивская область"), ("si", "ප\u{dca}ලොඩ\u{dd2}ව\u{dca} පළ\u{dcf}ත"), ("sk", "Plovdiv"), ("sl", "Plovdiv"), ("sq", "Provinca Plovdiv"), ("sr", "Пловдивска област"), ("sr_Latn", "Plovdivska oblast"), ("sv", "Plovdiv"), ("ta", "ப\u{bcd}ளோவ\u{bcd}டிவ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c4b}వ\u{c4d}డ\u{c3f}వ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปลอฟด\u{e34}ฟ"), ("tr", "Filibe ili"), ("uk", "Пловдивська область"), ("ur", "پلوودیف صوبہ"), ("vi", "Plovdiv"), ("zh", "普羅夫迪夫州")]),
                        unofficial_name_list: ["Plovdiv"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::BG,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.53367189999999), longitude: Some(26.5411164), max_latitude: Some(43.5504861), min_latitude: Some(43.5089142), max_longitude: Some(26.5636871), min_longitude: Some(26.4945973)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة رازغراد"), ("be", "Разградская вобласць"), ("bg", "Област Разград"), ("bn", "র\u{9be}জগ\u{9cd}র\u{9be}দ প\u{9cd}রদেশ"), ("bs", "Oblast Razgrad"), ("ca", "Província de Razgrad"), ("ccp", "𑄢𑄌\u{11134}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Oblast Razgrad"), ("cs", "Razgradská oblast"), ("da", "Razgrad (provins)"), ("de", "Oblast Rasgrad"), ("el", "Ράζγκραντ"), ("en", "Razgrad"), ("es", "Razgrad"), ("et", "Razgradi piirkond"), ("eu", "Razgrad probintzia"), ("fa", "استان رازگراد"), ("fi", "Razgradin alue"), ("fr", "Razgrad"), ("ga", "Cúige Razgrad"), ("gu", "રઝગ\u{acd}ર\u{ac7}ડ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז ראזגרד"), ("hi", "र\u{948}ज\u{93c}ग\u{94d}र\u{947}ड प\u{94d}रा\u{902}त"), ("hr", "Oblast Razgrad"), ("hu", "Razgrad megye"), ("hy", "Ռազգրադի մարզ"), ("id", "Provinsi Razgrad"), ("it", "Razgrad"), ("ja", "ラズグラト州"), ("ka", "რაზგრადის ოლქი"), ("kn", "ರಜ\u{ccd}ಗ\u{ccd}ರಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라즈그라드 주"), ("lt", "Razgrado sritis"), ("lv", "Razgradas apgabals"), ("mk", "Разградска област"), ("mn", "Разград аймаг"), ("mr", "र\u{947}जग\u{94d}र\u{945}ड प\u{94d}रा\u{902}त"), ("ms", "Razgrad Province"), ("nb", "Razgrad oblast"), ("nl", "Razgrad"), ("no", "Razgrad oblast"), ("pl", "Obwód Razgrad"), ("pt", "Razgrad (província)"), ("ro", "Regiunea Razgrad"), ("ru", "Разградская область"), ("si", "රස\u{dca}ග\u{dca}රඩ\u{dca} පළ\u{dcf}ත"), ("sk", "Razgrad (oblasť)"), ("sl", "Razgrad (okraj)"), ("sq", "Provinca Razgrad"), ("sr", "Разградска област"), ("sr_Latn", "Razgradska oblast"), ("sv", "Razgrad (region)"), ("ta", "ர\u{bbe}ஸ\u{bcd}க\u{bcd}ர\u{bbe}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3e}జ\u{c4d}\u{200c}గ\u{c4d}ర\u{c3e}డ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e31}ซกราด"), ("tr", "Razgrad ili"), ("uk", "Разградська область"), ("ur", "رازگراڈ صوبہ"), ("vi", "Razgrad (tỉnh)"), ("zh", "拉兹格勒州")]),
                        unofficial_name_list: ["Razgrad"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::BG,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.83557130000001), longitude: Some(25.9656554), max_latitude: Some(43.8890011), min_latitude: Some(43.7542331), max_longitude: Some(26.0490987), min_longitude: Some(25.8704853)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة روسه"), ("be", "Русенская вобласць"), ("bg", "Област Русе"), ("bn", "র\u{9c2}জ প\u{9cd}রদেশ"), ("bs", "Oblast Ruse"), ("ca", "Província de Ruse"), ("ccp", "𑄢𑄌\u{11134}"), ("ceb", "Oblast Ruse"), ("cs", "Rusenská oblast"), ("da", "Ruse (provins)"), ("de", "Oblast Russe"), ("el", "Ρούσε"), ("en", "Ruse"), ("es", "Ruse"), ("et", "Ruse piirkond"), ("eu", "Ruse probintzia"), ("fa", "استان روسه"), ("fi", "Rusen alue"), ("fr", "Roussé"), ("ga", "Cúige Ruse"), ("gu", "ર\u{ac1}ર\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "מחוז רוסה"), ("hi", "र\u{942}ज प\u{94d}रा\u{902}त"), ("hr", "Oblast Ruse"), ("hu", "Rusze megye"), ("hy", "Ռուսեի մարզ"), ("id", "Provinsi Ruse"), ("it", "Ruse"), ("ja", "ルセ州"), ("ka", "რუსეს ოლქი"), ("kn", "ರ\u{cc2}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루세 주"), ("lt", "Rusės sritis"), ("lv", "Ruses apgabals"), ("mk", "Русенска област"), ("mn", "Русе аймаг"), ("mr", "र\u{941}स प\u{94d}रा\u{902}त"), ("ms", "Ruse Province"), ("nb", "Ruse oblast"), ("nl", "Roese"), ("no", "Ruse oblast"), ("pl", "Obwód Ruse"), ("pt", "Ruse (província)"), ("ro", "Regiunea Ruse"), ("ru", "Русенская область"), ("si", "රසේ පළ\u{dcf}ත"), ("sk", "Ruse (oblasť)"), ("sl", "Ruse (okraj)"), ("sq", "Provinca Rusje"), ("sr", "Русенска област"), ("sr_Latn", "Rusenska oblast"), ("sv", "Ruse (region)"), ("ta", "ருஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "రూజ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ซ\u{e34}นเดอร\u{e4c}"), ("tr", "Rusçuk ili"), ("uk", "Русенська область"), ("ur", "روسے صوبہ"), ("vi", "Ruse (tỉnh)"), ("zh", "魯塞州")]),
                        unofficial_name_list: ["Ruse"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::BG,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.1147271), longitude: Some(27.2671901), max_latitude: Some(44.1248362), min_latitude: Some(44.097587), max_longitude: Some(27.2840753), min_longitude: Some(27.2254378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيليسترا"), ("be", "Сілістрэнская вобласць"), ("bg", "Област Силистра"), ("bn", "সিলিস\u{9cd}ত\u{9cd}র\u{9be} প\u{9cd}রদেশ"), ("bs", "Oblast Silistra"), ("ca", "Província de Silistra"), ("ccp", "𑄥\u{11128}𑄣\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢"), ("ceb", "Oblast Silistra"), ("cs", "Silisterská oblast"), ("da", "Silistra (provins)"), ("de", "Oblast Silistra"), ("el", "Σιλίστρα"), ("en", "Silistra"), ("es", "Silistra"), ("et", "Silistra piirkond"), ("eu", "Silistra probintzia"), ("fa", "استان سیلیسترا"), ("fi", "Silistran alue"), ("fr", "Silistra"), ("ga", "Cúige Silistra"), ("gu", "સિલિસ\u{acd}ટ\u{acd}રા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סיליסטרה"), ("hi", "सिलिस\u{94d}ट\u{94d}रा प\u{94d}रा\u{902}त"), ("hr", "Oblast Silistra"), ("hu", "Szilisztra megye"), ("hy", "Սիլիստրայի մարզ"), ("id", "Provinsi Silistra"), ("it", "Silistra"), ("ja", "シリストラ州"), ("ka", "სილისტრის ოლქი"), ("kn", "ಸ\u{cbf}ಲ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "실리스트라 주"), ("lt", "Silistros sritis"), ("lv", "Silistras apgabals"), ("mk", "Силистренска област"), ("mn", "Силистра аймаг"), ("mr", "सिलिस\u{94d}ट\u{94d}रा प\u{94d}रा\u{902}त"), ("ms", "Silistra Province"), ("nb", "Silistra oblast"), ("nl", "Silistra"), ("no", "Silistra oblast"), ("pl", "Obwód Silistra"), ("pt", "Silistra (província)"), ("ro", "Regiunea Silistra"), ("ru", "Силистренская область"), ("si", "ස\u{dd2}ල\u{dca}ස\u{dca}ට\u{dca}\u{200d}ර\u{dcf} පළ\u{dcf}ත"), ("sk", "Silistra (oblasť)"), ("sl", "Silistra (okraj)"), ("sq", "Provinca Silistra"), ("sr", "Силистранска област"), ("sr_Latn", "Silistranska oblast"), ("sv", "Silistra (region)"), ("ta", "கிளிஸ\u{bcd}டர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}ల\u{c3f}స\u{c4d}ట\u{c4d}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e35}ล\u{e34}สตรา"), ("tr", "Silistre ili"), ("uk", "Силістринська область"), ("ur", "سیلیسترا صوبہ"), ("vi", "Silistra (tỉnh)"), ("zh", "錫利斯特拉州")]),
                        unofficial_name_list: ["Silistra"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::BG,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6816536), longitude: Some(26.3228685), max_latitude: Some(42.711123), min_latitude: Some(42.6196377), max_longitude: Some(26.3902393), min_longitude: Some(26.2598866)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة اسليون"), ("be", "Слівенская вобласць"), ("bg", "Област Сливен"), ("bn", "স\u{9cd}লিভেন প\u{9cd}রদেশ"), ("bs", "Oblast Sliven"), ("ca", "Província de Sliven"), ("ccp", "𑄥\u{11133}𑄣\u{11128}𑄞𑄬𑄚\u{11134}"), ("ceb", "Oblast Sliven"), ("cs", "Slivenská oblast"), ("da", "Sliven"), ("de", "Oblast Sliwen"), ("el", "Σλίβεν"), ("en", "Sliven"), ("es", "Sliven"), ("et", "Sliveni piirkond"), ("eu", "Sliven probintzia"), ("fa", "استان اسلیون"), ("fi", "Slivenin alue"), ("fr", "Sliven"), ("ga", "Cúige Sliven"), ("gu", "સ\u{acd}લિવ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סליבן"), ("hi", "स\u{94d}लिव\u{947}न प\u{94d}रा\u{902}त"), ("hr", "Oblast Sliven"), ("hu", "Szliven megye"), ("hy", "Սլիվենի մարզ"), ("id", "Provinsi Sliven"), ("it", "Sliven"), ("ja", "スリヴェン州"), ("ka", "სლივენის ოლქი"), ("kn", "ಸ\u{ccd}ಲ\u{cbf}ವ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "슬리벤 주"), ("lt", "Sliveno sritis"), ("lv", "Slivenas apgabals"), ("mk", "Сливенска област"), ("mn", "Сливен аймаг"), ("mr", "स\u{94d}लिव\u{947}न प\u{94d}रा\u{902}त"), ("ms", "Sliven Province"), ("nb", "Sliven oblast"), ("nl", "Sliven"), ("no", "Sliven oblast"), ("pl", "Obwód Sliwen"), ("pt", "Sliven"), ("ro", "Regiunea Sliven"), ("ru", "Сливенская область"), ("si", "ස\u{dca}ල\u{dd2}වෙන\u{dca} පළ\u{dcf}ත"), ("sk", "Sliven"), ("sl", "Sliven"), ("sq", "Provinca Sliven"), ("sr", "Сливенска област"), ("sr_Latn", "Slivenska oblast"), ("sv", "Sliven"), ("ta", "ஸ\u{bcd}லிவேன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}ల\u{c4d}వ\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสล\u{e34}เวน"), ("tr", "Sliven ili"), ("uk", "Слівенська область"), ("ur", "سیلوین صوبہ"), ("vi", "Sliven"), ("zh", "斯利文州")]),
                        unofficial_name_list: ["Sliven"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::BG,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5774233), longitude: Some(24.7011138), max_latitude: Some(41.5938716), min_latitude: Some(41.5643), max_longitude: Some(24.8138621), min_longitude: Some(24.668448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سموليان"), ("be", "Смалянская вобласць"), ("bg", "Област Смолян"), ("bn", "স\u{9cd}মলিয\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("bs", "Oblast Smoljan"), ("ca", "Província de Smolian"), ("ccp", "𑄥\u{11133}𑄟\u{11127}𑄣\u{11128}𑄠𑄚\u{11134}"), ("ceb", "Oblast Smolyan"), ("cs", "Smoljanská oblast"), ("da", "Smoljan (provins)"), ("de", "Oblast Smoljan"), ("el", "Σμόλιαν"), ("en", "Smolyan"), ("es", "Provincia de Smolyan"), ("et", "Smoljani piirkond"), ("eu", "Smolian probintzia"), ("fa", "استان اسمولیان"), ("fi", "Smoljanin alue"), ("fr", "Smolyan (oblast)"), ("ga", "Cúige Smolyan"), ("gu", "સ\u{acd}મોલ\u{acd}યન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סמוליאן"), ("hi", "समोलिया\u{901} प\u{94d}रा\u{902}त"), ("hr", "Oblast Smoljan"), ("hu", "Szmoljan megye"), ("hy", "Սմոլյանի մարզ"), ("id", "Provinsi Smolyan"), ("it", "regione di Smoljan"), ("ja", "スモリャン州"), ("ka", "სმოლიანის ოლქი"), ("kn", "ಸ\u{ccd}ಮೋಲ\u{cbf}ಯನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "스몰랸 주"), ("lt", "Smoliano sritis"), ("lv", "Smoljanas province"), ("mk", "Смолјанска област"), ("mn", "Смолян аймаг"), ("mr", "स\u{94d}मोल\u{94d}या\u{902} प\u{94d}रा\u{902}त"), ("ms", "Smolyan Province"), ("nb", "Smoljan oblast"), ("nl", "Smoljan"), ("no", "Smoljan oblast"), ("pl", "Obwód Smolan"), ("pt", "Smolyan (província)"), ("ro", "Regiunea Smolean"), ("ru", "Смолянская область"), ("si", "එස\u{dca}මොල\u{dca}ය\u{dcf}න\u{dca} පළ\u{dcf}ත"), ("sk", "Smoľan (oblasť)"), ("sl", "Smoljan (okraj)"), ("sq", "Provinca Smoljan"), ("sr", "Смољанска област"), ("sr_Latn", "Smoljanska oblast"), ("sv", "Smoljan (region)"), ("ta", "ஸ\u{bcd}மோல\u{bcd}யன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4d}మ\u{c4b}ల\u{c4d}య\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสโมเล\u{e35}ยน"), ("tr", "Smolyan ili"), ("uk", "Смолянська область"), ("ur", "سمولیان صوبہ"), ("vi", "Smolyan (tỉnh)"), ("zh", "斯莫梁州")]),
                        unofficial_name_list: ["Smolyan"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::BG,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6977082), longitude: Some(23.3218675), max_latitude: Some(42.7877752), min_latitude: Some(42.4900111), max_longitude: Some(23.4569049), min_longitude: Some(23.1909885)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مدينة صوفيا"), ("be", "Гарадская вобласць Сафія"), ("bg", "Област София"), ("bs", "Oblast Sofija-grad"), ("ca", "Província de Sofia-Ciutat"), ("ccp", "𑄥\u{11127}𑄜\u{11128}𑄠"), ("ceb", "Sofiya-Grad"), ("cs", "Oblast Sofie"), ("de", "Sofia-Stadt"), ("en", "Sofia"), ("es", "Sofía-Ciudad"), ("et", "Sofija-grad"), ("eu", "Sofia-hiria probintzia"), ("fa", "استان صوفیه سیتی"), ("fr", "Sofia-ville"), ("ga", "Cúige Cathair Sofia"), ("hr", "Oblast Sofija"), ("hu", "Szófia főváros"), ("it", "Sofia-grad"), ("ja", "ソフィア市州"), ("ko", "소피아시주"), ("mk", "Софија-град"), ("pl", "Obwód miejski Sofia"), ("ro", "Regiunea Sofia-capitala"), ("ru", "София"), ("sk", "Sofia²"), ("sq", "Sofia kryeqytet"), ("sr", "Област Софија-град"), ("sr_Latn", "Oblast Sofija-grad"), ("uk", "Міська область Софія"), ("zh", "索菲亞市州")]),
                        unofficial_name_list: ["Sofia-Grad"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::BG,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6977082), longitude: Some(23.3218675), max_latitude: Some(42.7877752), min_latitude: Some(42.4900111), max_longitude: Some(23.4569049), min_longitude: Some(23.1909885)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sofia"), ("am", "ሶፊያ"), ("ar", "مقاطعة صوفيا"), ("az", "Sofiya"), ("be", "Сафійская вобласць"), ("bg", "Софийска област"), ("bn", "সোফিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("bs", "Oblast Sofija"), ("ca", "Província de Sofia"), ("ccp", "𑄥\u{11127}𑄜\u{11128}𑄠 𑄎𑄬𑄣"), ("ceb", "Sofiya"), ("cs", "Sofijská oblast"), ("cy", "Sofia"), ("da", "Sofia"), ("de", "Oblast Sofia"), ("el", "Σόφια"), ("en", "Sofia District"), ("es", "Sofía"), ("et", "Sofia piirkond"), ("eu", "Sofia probintzia"), ("fa", "استان صوفیه"), ("fi", "Sofian alue"), ("fr", "Sofia"), ("ga", "Cúige Sofia"), ("gl", "Sofía, Bulgaria"), ("gu", "સોફિયા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סופיה"), ("hi", "सोफिया प\u{94d}रा\u{902}त"), ("hr", "Sofijska oblast"), ("hu", "Szófia"), ("hy", "Սոֆիայի մարզ"), ("id", "Oblast Sofia"), ("is", "Sófía"), ("it", "Sofia"), ("ja", "ソフィア州"), ("jv", "Sofia"), ("ka", "სოფიის ოლქი"), ("kk", "София"), ("kn", "ಸೋಫ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "소피아 주"), ("ky", "София"), ("lt", "Sofijos sritis"), ("lv", "Sofijas apgabals"), ("mk", "Софиска област"), ("ml", "സോഫിയ"), ("mn", "Софи аймаг"), ("mr", "सोफिया प\u{94d}रा\u{902}त"), ("ms", "Sofia Province"), ("nb", "Sofia provins"), ("ne", "सोफिया"), ("nl", "Sofia"), ("no", "Sofia provins"), ("pa", "ਸ\u{a4b}ਫ\u{a3c}ੀਆ"), ("pl", "Obwód sofijski"), ("pt", "Sófia"), ("ro", "Regiunea Sofia"), ("ru", "Софийская область"), ("si", "සොෆ\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sk", "Sofia"), ("sl", "Sofija"), ("sq", "Provinca Sofia"), ("sr", "Софијска област"), ("sr_Latn", "Sofijska oblast"), ("sv", "Sofijska oblast"), ("sw", "Sofia"), ("ta", "சோபிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4b}ప\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโซเฟ\u{e35}ย"), ("tk", "Sofiýa"), ("tr", "Sofya ili"), ("uk", "Софійська область"), ("ur", "صوفیہ صوبہ"), ("uz", "Sofiya"), ("vi", "Tỉnh Sofia"), ("yo", "Sofia"), ("yo_BJ", "Sofia"), ("yue", "索菲亞"), ("yue_Hans", "索菲亚"), ("zh", "索菲亞州")]),
                        unofficial_name_list: ["Sofia"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::BG,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4257769), longitude: Some(25.6344644), max_latitude: Some(42.44570880000001), min_latitude: Some(42.3928015), max_longitude: Some(25.6686784), min_longitude: Some(25.5810451)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة استارا زاغورا"), ("be", "Старазагорская вобласць"), ("bg", "Област Стара Загора"), ("bn", "স\u{9cd}ট\u{9be}র\u{9be} জ\u{9be}গোর\u{9be} প\u{9cd}রদেশ"), ("bs", "Oblast Stara Zagora"), ("ca", "Província de Stara Zagora"), ("ccp", "𑄥\u{11133}𑄑𑄢\u{11134} 𑄎𑄉\u{1112e}𑄢"), ("ceb", "Oblast Stara Zagora"), ("cs", "Starozagorská oblast"), ("da", "Stara Zagora (provins)"), ("de", "Oblast Stara Sagora"), ("el", "Στάρα Ζαγόρα"), ("en", "Stara Zagora"), ("es", "Stara Zagora"), ("et", "Stara Zagora piirkond"), ("eu", "Stara Zagora probintzia"), ("fa", "استان استارا زاگورا"), ("fi", "Stara Zagoran alue"), ("fr", "Stara Zagora"), ("ga", "Cúige Stara Zagora"), ("gu", "સ\u{acd}ટારા ઝાગોરા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סטארה זאגורה"), ("hi", "स\u{94d}टारा ज\u{93c}गोरा प\u{94d}रा\u{902}त"), ("hr", "Oblast Stara Zagora"), ("hu", "Sztara Zagora megye"), ("hy", "Ստարա Զագորայի մարզ"), ("id", "Provinsi Stara Zagora"), ("it", "Stara Zagora"), ("ja", "スタラ・ザゴラ州"), ("ka", "სტარა-ზაგორის ოლქი"), ("kn", "ಸ\u{ccd}ಟ\u{ccd}ರಾ ಝಗೋರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "스타라자고라 주"), ("lt", "Stara Zagoros sritis"), ("lv", "Stara Zagoras apgabals"), ("mk", "Старозагорска област"), ("mn", "Стара-Загора аймаг"), ("mr", "स\u{94d}टारा झगोरा प\u{94d}रा\u{902}त"), ("ms", "Stara Zagora Province"), ("nb", "Stara Zagora oblast"), ("nl", "Stara Zagora"), ("no", "Stara Zagora oblast"), ("pl", "Obwód Stara Zagora"), ("pt", "Stara Zagora (província)"), ("ro", "Regiunea Stara Zagora"), ("ru", "Старозагорская область"), ("si", "එස\u{dca}ට\u{dcf}ර\u{dcf} සගොර\u{dcf} පළ\u{dcf}ත"), ("sk", "Stará Zagora (oblasť)"), ("sq", "Provinca Stara Zagora"), ("sr", "Старозагорска област"), ("sr_Latn", "Starozagorska oblast"), ("sv", "Stara Zagora (region)"), ("ta", "ஸ\u{bcd}ட\u{bbe}ர\u{bbe} ச\u{bbe}குற\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4d}మ\u{c4b}ట\u{c3e}ర\u{c3e} జ\u{c4b}గ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสตารา ซาโกรา"), ("tr", "Eski Zağra ili"), ("uk", "Старозагорська область"), ("ur", "ستارا زاگورا صوبہ"), ("vi", "Stara Zagora (tỉnh)"), ("zh", "舊扎戈拉州")]),
                        unofficial_name_list: ["Stara Zagora"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::BG,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2493556), longitude: Some(26.5727357), max_latitude: Some(43.2856429), min_latitude: Some(43.2275267), max_longitude: Some(26.6072562), min_longitude: Some(26.5195091)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تارغوفيشته"), ("be", "Тырговішцкая вобласць"), ("bg", "Област Търговище"), ("bn", "ট\u{9be}র\u{9cd}গভিস\u{9cd}তে প\u{9cd}রদেশ"), ("bs", "Oblast Trgovište"), ("ca", "Província de Targovixte"), ("ccp", "𑄑𑄢\u{11134}𑄉\u{1112e}𑄞\u{11128}𑄌\u{11134}𑄑𑄬"), ("ceb", "Oblast Tŭrgovishte"), ("cs", "Targovišťská oblast"), ("da", "Targovisjte (provins)"), ("de", "Oblast Targowischte"), ("el", "Ταργκόβιστε"), ("en", "Targovishte"), ("es", "Tărgovište"), ("et", "Tǎrgovište piirkond"), ("eu", "Targovixte probintzia"), ("fa", "استان ترگوویشته"), ("fi", "Tărgovišten alue"), ("fr", "Targovichté"), ("ga", "Cúige Targovishte"), ("gu", "ટાર\u{acd}ગોવિસ\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז טרגובישטה"), ("hi", "ट\u{948}गोविस\u{94d}टी प\u{94d}रा\u{902}त"), ("hr", "Oblast Trgovište"), ("hu", "Targoviste megye"), ("hy", "Տարգովիշտեի մարզ"), ("id", "Provinsi Targovishte"), ("it", "Tărgovište"), ("ja", "トゥルゴヴィシテ州"), ("ka", "ტარგოვიშტეს ოლქი"), ("kn", "ಟರ\u{ccd}ಗೋವ\u{cbf}ಶ\u{ccd}ಟ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "터르고비슈테 주"), ("lt", "Tergovištės sritis"), ("lv", "Targovištes apgabals"), ("mk", "Трговишка област"), ("mn", "Тырговиште аймаг"), ("mr", "ट\u{945}गोव\u{94d}हिस\u{94d}त प\u{94d}रा\u{902}त"), ("ms", "Targovishte Province"), ("nb", "Targovishte provins"), ("nl", "Targovisjte"), ("no", "Targovishte provins"), ("pl", "Obwód Tyrgowiszte"), ("pt", "Targovishte (província)"), ("ro", "Regiunea Tărgoviște"), ("ru", "Тырговиштская область"), ("si", "ටර\u{dca}ගොව\u{dd2}ශ\u{dca}ටේ පළ\u{dcf}ත"), ("sk", "Tărgovište (oblasť)"), ("sl", "Trgovište (okraj)"), ("sq", "Provinca Tërgovishçe"), ("sr", "Трговишка област"), ("sr_Latn", "Trgoviška oblast"), ("sv", "Targovisjte (region)"), ("ta", "டர\u{bcd}கோவிஸ\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టర\u{c4d}గ\u{c4b}వ\u{c3f}ష\u{c4d}ట\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตาร\u{e4c}โกว\u{e34}ชเต"), ("tr", "Eski Cuma ili"), ("uk", "Тирговиштська область"), ("ur", "ترگوویشتے صوبہ"), ("vi", "Targovishte (tỉnh)"), ("zh", "特爾戈維什特州")]),
                        unofficial_name_list: ["Targovishte"].to_vec(),
                    }
                ),
                (
                    "26",
                    Subdivision{
                        name: "26",
                        country_alpha2: Alpha2::BG,
                        code: "26",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9344366), longitude: Some(25.5554462), max_latitude: Some(41.9541525), min_latitude: Some(41.9114011), max_longitude: Some(25.6030698), min_longitude: Some(25.4942471)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خاسكوو"), ("be", "Хаскаўская вобласць"), ("bg", "Област Хасково"), ("bn", "হ\u{9be}স\u{9cd}কোভো প\u{9cd}রদেশ"), ("bs", "Oblast Haskovo"), ("ca", "Província de Khàskovo"), ("ccp", "𑄦𑄌\u{11134}𑄇\u{1112e}𑄞\u{1112e}"), ("ceb", "Oblast Khaskovo"), ("cs", "Chaskovská oblast"), ("da", "Haskovo"), ("de", "Oblast Chaskowo"), ("el", "Χάσκοβο"), ("en", "Haskovo"), ("es", "Haskovo"), ("et", "Haskovo piirkond"), ("eu", "Haskovo probintzia"), ("fa", "استان خاسکوو"), ("fi", "Haskovon alue"), ("fr", "Khaskovo"), ("ga", "Cúige Haskovo"), ("gu", "હસ\u{acd}કોવો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז חאסקובו"), ("hi", "हस\u{94d}कोवो प\u{94d}रा\u{902}त"), ("hr", "Oblast Haskovo"), ("hu", "Haszkovo megye"), ("hy", "Հասկովոյի մարզ"), ("id", "Provinsi Haskovo"), ("it", "Haskovo"), ("ja", "ハスコヴォ州"), ("ka", "ჰასკოვოს ოლქი"), ("kn", "ಹಸ\u{ccd}ಕೊವೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "하스코보 주"), ("lt", "Chaskovo sritis"), ("lv", "Haskovas apgabals"), ("mk", "Хасковска област"), ("mn", "Хасково аймаг"), ("mr", "हस\u{94d}स\u{94d}कोवो प\u{94d}रा\u{902}त"), ("ms", "Haskovo Province"), ("nb", "Haskovo oblast"), ("nl", "Chaskovo"), ("no", "Haskovo oblast"), ("pl", "Obwód Chaskowo"), ("pt", "Haskovo"), ("ro", "Regiunea Haskovo"), ("ru", "Хасковская область"), ("si", "හස\u{dca}කොවෝ පළ\u{dcf}ත"), ("sk", "Chaskovo"), ("sl", "Haskovo"), ("sq", "Provinca Haskovo"), ("sr", "Хасковска област"), ("sr_Latn", "Haskovska oblast"), ("sv", "Chaskovo"), ("ta", "ஹகோவோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హస\u{c4d}క\u{c4b}వ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮาสโคโว"), ("tr", "Hasköy ili"), ("uk", "Хасковська область"), ("ur", "خاسکوو صوبہ"), ("vi", "Haskovo"), ("zh", "哈斯科沃州")]),
                        unofficial_name_list: ["Haskovo"].to_vec(),
                    }
                ),
                (
                    "27",
                    Subdivision{
                        name: "27",
                        country_alpha2: Alpha2::BG,
                        code: "27",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2712398), longitude: Some(26.9361286), max_latitude: Some(43.3144123), min_latitude: Some(43.2250909), max_longitude: Some(27.0506347), min_longitude: Some(26.8972271)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شومن"), ("be", "Шуменская вобласць"), ("bg", "Област Шумен"), ("bn", "শ\u{9c1}মেন প\u{9cd}রদেশ"), ("bs", "Oblast Šumen"), ("ca", "Província de Xumen"), ("ccp", "𑄥\u{1112a}𑄟𑄬𑄚\u{11134}"), ("ceb", "Oblast Shumen"), ("cs", "Šumenská oblast"), ("da", "Sjumen"), ("de", "Oblast Schumen"), ("el", "Σούμεν"), ("en", "Shumen"), ("es", "Shumen"), ("et", "Šumeni piirkond"), ("eu", "Xumen probintzia"), ("fa", "استان شومن"), ("fi", "Šumenin alue"), ("fr", "Choumen"), ("ga", "Cúige Shumen"), ("gu", "શ\u{ac1}મ\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז שומן"), ("hi", "श\u{941}म\u{947}न प\u{94d}रा\u{902}त"), ("hr", "Oblast Šumen"), ("hu", "Sumen megye"), ("hy", "Շումենի մարզ"), ("id", "Provinsi Shumen"), ("it", "Šumen"), ("ja", "シュメン州"), ("ka", "შუმენის ოლქი"), ("kn", "ಷುಮ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "슈멘 주"), ("lt", "Šumeno sritis"), ("lv", "Šumenas apgabals"), ("mk", "Шуменска област"), ("mn", "Шумен аймаг"), ("mr", "श\u{941}म\u{947}न प\u{94d}रा\u{902}त"), ("ms", "Shumen Province"), ("nb", "Sjumen oblast"), ("nl", "Sjoemen"), ("no", "Sjumen oblast"), ("pl", "Obwód Szumen"), ("pt", "Shumen"), ("ro", "Regiunea Șumen"), ("ru", "Шуменская область"), ("si", "ශ\u{dd4}මෙන\u{dca} පළ\u{dcf}ත"), ("sk", "Šumen"), ("sl", "Šumen"), ("sq", "Provinca Shumen"), ("sr", "Шуменска област"), ("sr_Latn", "Šumenska oblast"), ("sv", "Sjumen"), ("ta", "ஷுமேன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "షూమ\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e39}เมน"), ("tr", "Şumnu ili"), ("uk", "Шуменська область"), ("ur", "شومن صوبہ"), ("vi", "Shumen"), ("zh", "舒門州")]),
                        unofficial_name_list: ["Šumen"].to_vec(),
                    }
                ),
                (
                    "28",
                    Subdivision{
                        name: "28",
                        country_alpha2: Alpha2::BG,
                        code: "28",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.48419990000001), longitude: Some(26.5035023), max_latitude: Some(42.5067417), min_latitude: Some(42.44753739999999), max_longitude: Some(26.5412634), min_longitude: Some(26.4646359)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة يامبول"), ("az", "Yambol vilayəti"), ("be", "Ямбальская вобласць"), ("bg", "Област Ямбол"), ("bn", "ইয\u{9bc}\u{9be}ম\u{9cd}বোল প\u{9cd}রদেশ"), ("bs", "Oblast Jambol"), ("ca", "Província de Iambol"), ("ccp", "𑄠𑄟\u{11134}𑄝\u{11127}𑄣\u{11134}"), ("ceb", "Obshtina Yambol"), ("cs", "Jambolská oblast"), ("da", "Jambol"), ("de", "Oblast Jambol"), ("el", "Γιάμπολ"), ("en", "Yambol"), ("es", "Yambol"), ("et", "Jamboli piirkond"), ("eu", "Jambol probintzia"), ("fa", "استان یامبول"), ("fi", "Jambolin alue"), ("fr", "Yambol"), ("ga", "Cúige Yambol"), ("gu", "યમ\u{acd}બોલ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז ימבול"), ("hi", "याम\u{94d}बोल प\u{94d}रा\u{902}त"), ("hr", "Oblast Jambol"), ("hu", "Jambol megye"), ("hy", "Յամբոլի մարզ"), ("id", "Provinsi Yambol"), ("it", "Jambol"), ("ja", "ヤンボル州"), ("ka", "იამბოლის ოლქი"), ("kn", "ಯಂಬೋಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "얌볼 주"), ("lt", "Jambolo sritis"), ("lv", "Jambolas apgabals"), ("mk", "Јамболска област"), ("mn", "Ямбол аймаг"), ("mr", "एम\u{94d}बॉल प\u{94d}रा\u{902}त"), ("ms", "Yambol Province"), ("nb", "Jambol oblast"), ("nl", "Jambol"), ("no", "Jambol oblast"), ("pl", "Obwód Jamboł"), ("pt", "Yambol"), ("ro", "Regiunea Iambol"), ("ru", "Ямболская область"), ("si", "යම\u{dca}බෝල\u{dca} පළ\u{dcf}ත"), ("sk", "Jambol"), ("sl", "Jambol"), ("sq", "Provinca Jambol"), ("sr", "Јамболска област"), ("sr_Latn", "Jambolska oblast"), ("sv", "Jambol"), ("ta", "யம\u{bcd}போல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "య\u{c3e}ంబ\u{c4b}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดย\u{e31}มโบล"), ("tr", "Yambol ili"), ("uk", "Ямбольська область"), ("ur", "یامبول صوبہ"), ("vi", "Yambol"), ("zh", "揚博爾州")]),
                        unofficial_name_list: ["Yambol"].to_vec(),
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
#[cfg(feature = "bg")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BG,
        alpha3: Alpha3::BGR,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 359,
        currency_code: "BGN",
        gec: Some(GEC::BU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::BUL),
        iso_long_name: "The Republic of Bulgaria",
        iso_short_name: "Bulgaria",
        official_language_list: ["bg"].to_vec(),
        spoken_language_list: ["bg"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9, 10].to_vec(),
        national_prefix: "0",
        nationality: Some("Bulgarian"),
        number: "100",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "BG",
        unofficial_name_list: [
            "Bulgaria",
            "Bulgarien",
            "Bulgarie",
            "ブルガリア",
            "Bulgarije",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Bulgaria"),
            ("af", "Bulgarye"),
            ("ak", "Bulgaria"),
            ("am", "ቡሔጓሱ።"),
            ("an", "Bulgaria"),
            ("ar", "بلغاريا"),
            ("as", "ব\u{9c1}লগ\u{9be}ৰিয়\u{9be}"),
            ("ay", "Bulgaria"),
            ("az", "Bolqarıstan"),
            ("ba", "Bulgaria"),
            ("be", "Балгарыя"),
            ("bg", "България"),
            ("bi", "Bulgaria"),
            ("bn", "ব\u{9c1}লগ\u{9be}রিয়\u{9be}"),
            ("bn_IN", "ব\u{9c1}লগ\u{9be}রিয়\u{9be}"),
            ("br", "Bulgaria"),
            ("bs", "Bugarska"),
            ("ca", "Bulgària"),
            ("ce", "Болгари"),
            ("ch", "Bulgaria"),
            ("cs", "Bulharsko"),
            ("cv", "Болгари"),
            ("cy", "Bwlgaria"),
            ("da", "Bulgarien"),
            ("de", "Bulgarien"),
            ("dv", "ބ\u{7a6}ލ\u{7b0}ގ\u{7ad}ރ\u{7a8}އ\u{7a7}"),
            ("dz", "བ\u{f71}ལ་ག་ར\u{f72}་ཡ།"),
            ("ee", "Bulgaria"),
            ("el", "Βουλγαρία"),
            ("en", "Bulgaria"),
            ("eo", "Bulgario"),
            ("es", "Bulgaria"),
            ("et", "Bulgaaria"),
            ("eu", "Bulgaria"),
            ("fa", "بلغارستان"),
            ("ff", "Bulgariya"),
            ("fi", "Bulgaria"),
            ("fo", "Bulgaria"),
            ("fr", "Bulgarie"),
            ("fy", "Bulgarije"),
            ("ga", "An Bhulgáir"),
            ("gl", "Bulgaria"),
            ("gn", "Bulgaria"),
            ("gu", "બલ\u{acd}ગ\u{ac7}રિયા"),
            ("gv", "Yn Vulgeyr"),
            ("ha", "Bulgairiya"),
            ("he", "בולגריה"),
            ("hi", "ब\u{941}ल\u{94d}गारिया"),
            ("hr", "Bugarska"),
            ("ht", "Bilgari"),
            ("hu", "Bulgária"),
            ("hy", "Բուլղարիա"),
            ("ia", "Bulgaria"),
            ("id", "Bulgaria"),
            ("io", "Bulgaria"),
            ("is", "Búlgaría"),
            ("it", "Bulgaria"),
            ("iu", "Bulgaria"),
            ("ja", "ブルガリア"),
            ("ka", "ბულგარეთი"),
            ("ki", "Bulgaria"),
            ("kk", "Болгария"),
            ("kl", "Bulgaria"),
            ("km", "ប\u{17ca}\u{17bb}លហ\u{17d2}គារ\u{17b8}"),
            ("kn", "ಬಲ\u{ccd}ಗೇರ\u{cbf}ಯಾ"),
            ("ko", "불가리아"),
            ("ku", "Bulgaristan"),
            ("kv", "Болгария"),
            ("kw", "Bulgari"),
            ("ky", "Болгария"),
            ("lo", "ປະເທດບ\u{eb9}ນກາລ\u{eb5}"),
            ("lt", "Bulgarija"),
            ("lv", "Bulgārija"),
            ("mi", "Purukāria"),
            ("mk", "Бугарија"),
            ("ml", "ബള\u{d4d}\u{200d}ഗേറിയ"),
            ("mn", "Болгар"),
            ("mr", "बल\u{94d}ग\u{947}रिया"),
            ("ms", "Bulgaria"),
            ("mt", "Bulgarija"),
            (
                "my",
                "ဘ\u{1030}လ\u{103a}ဂေးရ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Borgeriya"),
            ("nb", "Bulgaria"),
            ("ne", "ब\u{941}लग\u{947}रिया"),
            ("nl", "Bulgarije"),
            ("nn", "Bulgaria"),
            ("nv", "Bálgaa Bikéyah"),
            ("oc", "Bulgaria"),
            ("or", "ବ\u{b41}ଲଗ\u{b3e}ର\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਬ\u{a41}ਲਗਾਰੀਆ"),
            ("pi", "ब\u{941}ल\u{94d}गारिया"),
            ("pl", "Bułgaria"),
            ("ps", "بلغاریه"),
            ("pt", "Bulgária"),
            ("pt_BR", "Bulgária"),
            ("ro", "Bulgaria"),
            ("ru", "Болгария"),
            ("rw", "Buligariya"),
            ("sc", "Bulgaria"),
            ("sd", "بلغاريه"),
            ("si", "බල\u{dca}ගේර\u{dd2}ය\u{dcf}"),
            ("sk", "Bulharsko"),
            ("sl", "Bolgarija"),
            ("so", "Bulgaria"),
            ("sq", "Bullgarië"),
            ("sr", "Бугарска"),
            ("sv", "Bulgarien"),
            ("sw", "Bulgaria"),
            ("ta", "பல\u{bcd}கேரிய\u{bbe}"),
            ("te", "బల\u{c4d}గ\u{c47}ర\u{c3f}య\u{c3e}"),
            ("tg", "Булғория"),
            ("th", "บ\u{e31}ลแกเร\u{e35}ย"),
            ("ti", "ቡልጋሪያ"),
            ("tk", "Bolgariýa"),
            ("tl", "Bulgaria"),
            ("tr", "Bulgaristan"),
            ("tt", "Булgариа"),
            ("ug", "بۇلغارىيە"),
            ("uk", "Болгарія"),
            ("ur", "بلغاریہ"),
            ("uz", "Bolgariya"),
            ("ve", "Bulgaria"),
            ("vi", "Bua-ga-ri"),
            ("wa", "Bulgåreye"),
            ("wo", "Bulgaari"),
            ("xh", "Bulgaria"),
            ("yo", "Bùlgáríà"),
            ("zh_CN", "保加利亚"),
            ("zh_HK", "保加利亞"),
            ("zh_TW", "保加利亞"),
            ("zu", "IBulgariya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
