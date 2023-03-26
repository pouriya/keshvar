// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Angola

#[cfg(all(feature = "ao", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AO;
    pub const ALPHA3: Alpha3 = Alpha3::AGO;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 244;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::AOA;
    pub const GEC: Option<GEC> = Some(GEC::AO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ANG);
    pub const ISO_SHORT_NAME: &str = "Angola";
    pub const ISO_LONG_NAME: &str = "The Republic of Angola";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Angolan");
    pub const NUMBER: &str = "024";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::MiddleAfrica);
    pub const UN_LOCODE: &str = "AO";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Angola", "アンゴラ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Angola"),
        ("af", "Angola"),
        ("ak", "Angola"),
        ("am", "አንጎላ"),
        ("an", "Angola"),
        ("ar", "أنغولا"),
        ("as", "এংগোল\u{9be}"),
        ("ay", "Angola"),
        ("az", "Anqola"),
        ("ba", "Angola"),
        ("be", "Ангола"),
        ("bg", "Ангола"),
        ("bi", "Angola"),
        ("bn", "অ\u{9cd}য\u{9be}ঙ\u{9cd}গোল\u{9be}"),
        ("bn_IN", "অ\u{9cd}য\u{9be}ঙ\u{9cd}গোল\u{9be}"),
        ("br", "Angola"),
        ("bs", "Angola"),
        ("ca", "Angola"),
        ("ce", "Ангола"),
        ("ch", "Angola"),
        ("cs", "Angola"),
        ("cv", "Ангола"),
        ("cy", "Angola"),
        ("da", "Angola"),
        ("de", "Angola"),
        ("dv", "އ\u{7ac}ނ\u{7b0}ގ\u{7af}ލ\u{7a7}"),
        ("dz", "ཨ\u{f7a}ང་ག\u{f7c}་ལ།"),
        ("ee", "Angola"),
        ("el", "Ανγκόλα"),
        ("en", "Angola"),
        ("eo", "Angolo"),
        ("es", "Angola"),
        ("et", "Angola"),
        ("eu", "Angola"),
        ("fa", "آنگولا"),
        ("ff", "Anngolaa"),
        ("fi", "Angola"),
        ("fo", "Angola"),
        ("fr", "Angola"),
        ("fy", "Angoala"),
        ("ga", "Angóla"),
        ("gl", "Angola"),
        ("gn", "Angola"),
        ("gu", "અ\u{a82}ગોલા"),
        ("gv", "Angoley"),
        ("ha", "Angola"),
        ("he", "אנגולה"),
        ("hi", "अ\u{902}गोला"),
        ("hr", "Angola"),
        ("ht", "Angola"),
        ("hu", "Angola"),
        ("hy", "Անգոլա"),
        ("ia", "Angola"),
        ("id", "Angola"),
        ("io", "Angola"),
        ("is", "Angóla"),
        ("it", "Angola"),
        ("iu", "Angola"),
        ("ja", "アンゴラ"),
        ("ka", "ანგოლა"),
        ("ki", "Angola"),
        ("kk", "Ангола"),
        ("kl", "Angola"),
        ("km", "អង\u{17cb}ហ\u{17d2}គោឡា"),
        ("kn", "ಅಂಗೋಲಾ"),
        ("ko", "앙골라"),
        ("ku", "Angola"),
        ("kv", "Angola"),
        ("kw", "Angola"),
        ("ky", "Ангола"),
        ("lo", "ປະເທດອ\u{eb1}ງໂກລາ"),
        ("lt", "Angola"),
        ("lv", "Angola"),
        ("mi", "Anakora"),
        ("mk", "Ангола"),
        ("ml", "അംഗോള"),
        ("mn", "Ангол"),
        ("mr", "अ\u{902}गोला"),
        ("ms", "Angola"),
        ("mt", "Angola"),
        (
            "my",
            "အင\u{103a}ဂ\u{102d}\u{102f}လာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Angora"),
        ("nb", "Angola"),
        ("ne", "एनगोला"),
        ("nl", "Angola"),
        ("nn", "Angola"),
        ("nv", "Angola"),
        ("oc", "Angòla"),
        ("or", "ଆଙ\u{b4d}ଗୋଲ\u{b3e}"),
        ("pa", "ਅ\u{a70}ਗ\u{a4b}ਲਾ"),
        ("pi", "अ\u{902}गोला"),
        ("pl", "Angola"),
        ("ps", "انګولا"),
        ("pt", "Angola"),
        ("pt_BR", "Angola"),
        ("ro", "Angola"),
        ("ru", "Ангола"),
        ("rw", "Angola"),
        ("sc", "Angola"),
        ("sd", "Angola"),
        ("si", "ඇන\u{dca}ගෝල\u{dcf}ව"),
        ("sk", "Angola"),
        ("sl", "Angola"),
        ("so", "Angoola"),
        ("sq", "Angolë"),
        ("sr", "Ангола"),
        ("sv", "Angola"),
        ("sw", "Angola"),
        ("ta", "அங\u{bcd}கோல\u{bbe}"),
        ("te", "అంగ\u{c4b}ల\u{c3e}"),
        ("tg", "Ангола"),
        ("th", "แองโกลา"),
        ("ti", "ኣንጎላ"),
        ("tk", "Angola"),
        ("tl", "Anggola"),
        ("tr", "Angola"),
        ("tt", "Анgола"),
        ("ug", "ئانگولا"),
        ("uk", "Ангола"),
        ("ur", "انگولا"),
        ("uz", "Angola"),
        ("ve", "Angola"),
        ("vi", "Ăng-gô-la"),
        ("wa", "Angola"),
        ("wo", "Angoolaa"),
        ("xh", "Angola"),
        ("yo", "Àngólà"),
        ("zh_CN", "安哥拉"),
        ("zh_HK", "安哥拉"),
        ("zh_TW", "安哥拉"),
        ("zu", "I-Angola"),
    ];
    #[cfg(all(feature = "ao", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -11.202692;
        pub const LONGITUDE: f64 = 17.873887;
        pub const MAX_LATITUDE: f64 = -4.388063300000001;
        pub const MAX_LONGITUDE: f64 = 24.0878855;
        pub const MIN_LATITUDE: f64 = -18.0391039;
        pub const MIN_LONGITUDE: f64 = 11.4696999;
        pub const NORTHEAST_LATITUDE: f64 = -4.388063300000001;
        pub const NORTHEAST_LONGITUDE: f64 = 24.0878855;
        pub const SOUTHWEST_LATITUDE: f64 = -18.0391039;
        pub const SOUTHWEST_LONGITUDE: f64 = 11.4696999;
    }
}
#[cfg(all(feature = "ao", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -11.202692,
            longitude: 17.873887,
            max_latitude: -4.388063300000001,
            max_longitude: 24.0878855,
            min_latitude: -18.0391039,
            min_longitude: 11.4696999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -4.388063300000001,
                    longitude: 24.0878855,
                },
                southwest: CountryGeoBound {
                    latitude: -18.0391039,
                    longitude: 11.4696999,
                },
            },
        }
    }
}

#[cfg(all(feature = "ao", feature = "subdivisions"))]
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
                    "BGO",
                    Subdivision{
                        name: "Bengo",
                        country_alpha2: Alpha2::AO,
                        code: "BGO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.104225699999999), longitude: Some(13.7289167), max_latitude: Some(-7.629126100000001), min_latitude: Some(-10.473318), max_longitude: Some(14.7415391), min_longitude: Some(13.1099769)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bengo-provinsie"), ("am", "ቤንጎ"), ("ar", "مقاطعة بنغو"), ("az", "Benqo"), ("be", "Правінцыя Бенга"), ("bg", "Бенго"), ("bn", "বেনগো প\u{9cd}রদেশ"), ("ca", "Bengo"), ("ccp", "𑄝𑄬𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Bengo Province"), ("cs", "Bengo"), ("da", "Bengo"), ("de", "Provinz Bengo"), ("el", "Μπένγκο"), ("en", "Bengo"), ("es", "Bengo"), ("et", "Bengo provints"), ("fa", "استان بنگو"), ("fi", "Bengo"), ("fr", "Bengo"), ("gl", "Bengo"), ("gu", "બ\u{ac7}ન\u{acd}ગો પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}\u{902}गो प\u{94d}रा\u{902}त"), ("id", "Bengo"), ("it", "provincia del Bengo"), ("ja", "ベンゴ州"), ("ka", "ბენგოს პროვინცია"), ("kn", "ಬ\u{cc6}ಂಗೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "벵구 주"), ("lt", "Bengo provincija"), ("lv", "Bengo province"), ("mr", "ब\u{947}नगो प\u{94d}रा\u{902}त"), ("ms", "Bengo Province"), ("nb", "Bengo"), ("nl", "Bengo"), ("no", "Bengo"), ("pl", "Prowincja Bengo"), ("pt", "Bengo"), ("ro", "Provincia Bengo"), ("ru", "Бенго"), ("si", "බෙන\u{dca}ගෝ පළ\u{dcf}ත"), ("so", "Bengo Province"), ("sr", "Бенго"), ("sr_Latn", "Bengo"), ("sv", "Bengo"), ("sw", "Bengo"), ("ta", "பெங\u{bcd}கோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}ంగ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเบนโก"), ("tr", "Bengo Bölgesi"), ("uk", "Бенго"), ("ur", "بنگو صوبہ"), ("vi", "Bengo"), ("yue", "本哥省"), ("yue_Hans", "本哥省"), ("zh", "本哥省")]),
                        unofficial_name_list: ["Bengo"].to_vec(),
                    }
                ),
                (
                    "BGU",
                    Subdivision{
                        name: "Benguela",
                        country_alpha2: Alpha2::AO,
                        code: "BGU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.5905158), longitude: Some(13.416501), max_latitude: Some(-12.5622063), min_latitude: Some(-12.6193441), max_longitude: Some(13.4346042), min_longitude: Some(13.3585294)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Benguela-provinsie"), ("am", "ቤንጉዌላ"), ("ar", "مقاطعة بنغيلا"), ("be", "Правінцыя Бенгела"), ("bg", "Бенгела"), ("bn", "বেঙ\u{9cd}গ\u{9c1}এল\u{9be} প\u{9cd}রদেশ"), ("ca", "Benguela"), ("ccp", "𑄝𑄬𑄋\u{11134}𑄉\u{1112a}𑄠𑄬𑄣"), ("ceb", "Benguela"), ("cs", "Benguela"), ("da", "Benguela"), ("de", "Provinz Benguela"), ("el", "Μπενγκουέλα"), ("en", "Benguela"), ("es", "Benguela"), ("fa", "استان بنگوئلا"), ("fi", "Benguelan maakunta"), ("fr", "Benguela"), ("gl", "Provincia de Benguela"), ("gu", "બ\u{ac7}ન\u{acd}ગ\u{acd}ય\u{ac1}લા પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{947}\u{902}ग\u{941}ला प\u{94d}रा\u{902}त"), ("id", "Provinsi Benguela"), ("it", "provincia di Benguela"), ("ja", "ベンゲラ州"), ("ka", "ბენგელის პროვინცია"), ("kn", "ಬ\u{cc6}ಂಗ\u{ccd}ಯುಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "벵겔라 주"), ("lt", "Bengelos provincija"), ("lv", "Bengelas province"), ("mr", "ब\u{947}\u{902}ग\u{941}एला प\u{94d}रा\u{902}त"), ("ms", "Benguela Province"), ("nb", "Benguela"), ("nl", "Benguela"), ("no", "Benguela"), ("pl", "Prowincja Benguela"), ("pt", "Benguela"), ("ro", "Provincia Benguela"), ("ru", "Бенгела"), ("si", "බෙන\u{dca}ග\u{dcf}ල\u{dcf} පළ\u{dcf}ත"), ("so", "Benguela Province"), ("sr", "Бенгела"), ("sr_Latn", "Bengela"), ("sv", "Benguela"), ("sw", "Benguela"), ("ta", "பெங\u{bcd}கேல\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c46}ంగ\u{c4d}యువ\u{c46}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เบงก\u{e35}ลา"), ("tr", "Benguela Bölgesi"), ("uk", "Бенгела"), ("ur", "بینگوئلا صوبہ"), ("vi", "Benguela"), ("yue", "班圭拉省"), ("yue_Hans", "班圭拉省"), ("zh", "本吉拉省")]),
                        unofficial_name_list: ["Benguela"].to_vec(),
                    }
                ),
                (
                    "BIE",
                    Subdivision{
                        name: "Bié",
                        country_alpha2: Alpha2::AO,
                        code: "BIE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.5727907), longitude: Some(17.668887), max_latitude: Some(-10.572202), min_latitude: Some(-14.305984), max_longitude: Some(19.2239829), min_longitude: Some(15.7335889)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bié-provinsie"), ("ar", "مقاطعة بيي"), ("be", "Правінцыя Біе"), ("bg", "Бие"), ("bn", "বিই প\u{9cd}রদেশ"), ("ca", "Bié"), ("ccp", "𑄝\u{11128}𑄠𑄬"), ("ceb", "Província do Bié"), ("cs", "Bié"), ("da", "Bié"), ("de", "Provinz Bié"), ("el", "Μπιέ"), ("en", "Bié"), ("es", "Bié"), ("fa", "استان بیه"), ("fi", "Bié"), ("fr", "Bié"), ("gl", "Bié"), ("gu", "બાય પ\u{acd}રા\u{a82}ત"), ("hi", "बिए प\u{94d}रा\u{902}त"), ("id", "Provinsi Bié"), ("it", "provincia di Bié"), ("ja", "ビエ州"), ("ka", "ბიეს პროვინცია"), ("kn", "ಬ\u{cbf}ಯ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "비에 주"), ("lt", "Bijės provincija"), ("lv", "Biē province"), ("mr", "बाय प\u{94d}रा\u{902}त"), ("ms", "Bie Province"), ("nb", "Bié"), ("nl", "Bié"), ("no", "Bié"), ("pl", "Prowincja Bié"), ("pt", "Bié"), ("ro", "Provincia Bié"), ("ru", "Бие"), ("si", "බ\u{dd2}යේ පළ\u{dcf}ත"), ("so", "Bié Province"), ("sr", "Бије"), ("sr_Latn", "Bije"), ("sv", "Bié"), ("sw", "Bié"), ("ta", "பிஇ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c48} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e35}\u{e49}"), ("tr", "Bié Bölgesi"), ("uk", "Біє"), ("ur", "بئے صوبہ"), ("vi", "Bié"), ("yue", "比耶省"), ("yue_Hans", "比耶省"), ("zh", "比耶省")]),
                        unofficial_name_list: ["Bié"].to_vec(),
                    }
                ),
                (
                    "CAB",
                    Subdivision{
                        name: "Cabinda",
                        country_alpha2: Alpha2::AO,
                        code: "CAB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.56), longitude: Some(12.19), max_latitude: Some(-5.5409834), min_latitude: Some(-5.6216234), max_longitude: Some(12.2682953), min_longitude: Some(12.1627235)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cabinda"), ("ar", "مقاطعة كابيندا"), ("az", "Kabinda"), ("be", "Правінцыя Кабінда"), ("bg", "Кабинда"), ("bn", "ক\u{9be}বিনড\u{9be} প\u{9cd}রদেশ"), ("ca", "Cabinda"), ("ccp", "𑄇𑄝\u{11128}𑄚\u{11134}𑄘"), ("ceb", "Cabinda"), ("cs", "Cabinda"), ("cy", "Cabinda"), ("da", "Cabinda"), ("de", "Provinz Cabinda"), ("el", "Καμπίντα"), ("en", "Cabinda"), ("es", "Cabinda"), ("eu", "Kabinda"), ("fa", "استان کابیندا"), ("fi", "Cabinda"), ("fr", "Cabinda"), ("gl", "Cabinda"), ("gu", "કાબિન\u{acd}ડા પ\u{acd}રા\u{a82}ત"), ("he", "קבינדה"), ("hi", "कबिन\u{94d}ड प\u{94d}रा\u{902}त"), ("hr", "Cabinda"), ("hu", "Cabinda"), ("id", "Provinsi Cabinda"), ("it", "provincia di Cabinda"), ("ja", "カビンダ"), ("ka", "კაბინდის პროვინცია"), ("kn", "ಕ\u{ccd}ಯಾಬ\u{cbf}ಂಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카빈다 주"), ("lt", "Kabinda"), ("lv", "Kabindas province"), ("mr", "का\u{902}बीडा प\u{94d}रा\u{902}त"), ("ms", "Cabinda Province"), ("nb", "Cabinda"), ("nl", "Cabinda"), ("no", "Cabinda"), ("pl", "Prowincja Kabinda"), ("pt", "Cabinda"), ("ro", "Provincia Cabinda"), ("ru", "Кабинда"), ("si", "ක\u{dcf}බ\u{dd2}න\u{dca}ඩ\u{dcf} පළ\u{dcf}ත"), ("sk", "Cabinda"), ("so", "Cabinda Province"), ("sr", "Кабинда"), ("sr_Latn", "Kabinda"), ("sv", "Kabinda"), ("sw", "Kabinda"), ("ta", "க\u{bbe}பின\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}బ\u{c3f}ండ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐโนวาสโกเช\u{e35}ย"), ("tr", "Cabinda Bölgesi"), ("uk", "Кабінда"), ("ur", "کابیندا صوبہ"), ("vi", "Cabinda"), ("yue", "喀丙達省"), ("yue_Hans", "喀丙达省"), ("zh", "喀丙達省")]),
                        unofficial_name_list: ["Kabinda"].to_vec(),
                    }
                ),
                (
                    "CCU",
                    Subdivision{
                        name: "Cuando-Cubango",
                        country_alpha2: Alpha2::AO,
                        code: "CCU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.4180824), longitude: Some(18.8076195), max_latitude: Some(-13.60931), min_latitude: Some(-18.042076), max_longitude: Some(23.4281539), min_longitude: Some(16.463013)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cuando Cubango-provinsie"), ("am", "ኩዋንዶ ኩባንጎ"), ("ar", "مقاطعة كواندو كوبانغو"), ("be", "Правінцыя Кванда-Кубанга"), ("bg", "Куандо Кубанго"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ন\u{9cd}ড\u{9c1} ক\u{9c1}ব\u{9be}ং প\u{9cd}রদেশ"), ("ca", "Cuando Cubango"), ("ccp", "𑄇\u{1112a}𑄠𑄚\u{11134}𑄘\u{1112e} 𑄇\u{1112a}𑄝𑄋\u{11134}𑄉\u{1112e}"), ("ceb", "Kuando Kubango"), ("cs", "Cuando Cubango"), ("da", "Cuando Cubango"), ("de", "Provinz Cuando Cubango"), ("el", "Κουάντο"), ("en", "Cuando Cubango"), ("es", "Cuando Cubango"), ("fa", "استان کواندو کوبانگو"), ("fi", "Cuando Cubango"), ("fr", "Kwando-Kubango"), ("gu", "ક\u{ac1}આન\u{acd}ડો ક\u{acd}ય\u{ac1}બા\u{a82}ગો પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{942}य\u{94d}न\u{94d}डो क\u{941}ब\u{902}गो प\u{94d}रा\u{902}त"), ("id", "Cuando Cubango"), ("it", "provincia di Cuando Cubango"), ("ja", "クアンド・クバンゴ州"), ("ka", "კუანდო-კუბანგოს პროვინცია"), ("kn", "ಕ\u{ccd}ವಾಂಡೋ ಕ\u{ccd}ಯುಬಾಂಗೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿠안두쿠방구 주"), ("lt", "Kvandas Kubangas"), ("lv", "Kvando-Kubango province"), ("mr", "क\u{941}आ\u{902}डो क\u{94d}य\u{941}ब\u{947}\u{902}जो प\u{94d}रा\u{902}त"), ("ms", "Cuando Cubango Province"), ("nb", "Cuando Cubango"), ("nl", "Cuando Cubango"), ("no", "Cuando Cubango"), ("pl", "Prowincja Cuando-Cubango"), ("pt", "Cuando-Cubango"), ("ro", "Provincia Cuando Cubango"), ("ru", "Квандо-Кубанго"), ("si", "ක\u{dd4}අන\u{dca}ඩෝ ක\u{dd4}බන\u{dca}ගෝ පළ\u{dcf}ත"), ("so", "Cuando Cubango Province"), ("sr", "Квандо Кубанго"), ("sr_Latn", "Kvando Kubango"), ("sv", "Cuando Cubango"), ("sw", "Cuando Cubango"), ("ta", "சுஅண\u{bcd}டோ சுப\u{bbe}ங\u{bcd}கோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కువ\u{c3e}ండ\u{c4b} క\u{c4d}యుబ\u{c3e}ంగ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดควนโด ค\u{e39}บ\u{e31}นโก"), ("tr", "Cuando Cubango Bölgesi"), ("uk", "Квандо-Кубанго"), ("ur", "کواندو کوبانگو صوبہ"), ("vi", "Cuando Cubango"), ("yue", "庫安多古班哥省"), ("yue_Hans", "库安多古班哥省"), ("zh", "庫安多古班哥省")]),
                        unofficial_name_list: ["Cuando-Cubango"].to_vec(),
                    }
                ),
                (
                    "CNN",
                    Subdivision{
                        name: "Cunene",
                        country_alpha2: Alpha2::AO,
                        code: "CNN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.2802221), longitude: Some(16.1580937), max_latitude: Some(-15.158318), min_latitude: Some(-17.442467), max_longitude: Some(17.402746), min_longitude: Some(13.1554)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cunene-provinsie"), ("ar", "مقاطعة كونيني"), ("be", "Правінцыя Куненэ"), ("bg", "Кунене"), ("bn", "ক\u{9c1}নেনে প\u{9cd}রদেশ"), ("ca", "Cunene"), ("ccp", "𑄇\u{1112a}𑄚𑄬𑄚\u{11134}"), ("ceb", "Cunene Province"), ("da", "Cunene"), ("de", "Provinz Cunene"), ("el", "Κουκένε"), ("en", "Cunene"), ("es", "Cunene"), ("fa", "استان کونه\u{200c}نه"), ("fi", "Cunene"), ("fr", "Kunene"), ("gu", "ક\u{ac1}ન\u{ac7}ન પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{941}न\u{947}न\u{947} प\u{94d}रा\u{902}त"), ("id", "Cunene"), ("it", "provincia del Cunene"), ("ja", "クネネ州"), ("ka", "კუნენეს პროვინცია"), ("kn", "ಕುನ\u{cc6}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿠네느 주"), ("lt", "Kunenės provincija"), ("lv", "Kunenes province"), ("mr", "क\u{941}न\u{947}न प\u{94d}रा\u{902}त"), ("ms", "Cunene Province"), ("nb", "Cunene"), ("nl", "Cunene"), ("no", "Cunene"), ("pl", "Prowincja Cunene"), ("pt", "Cunene"), ("ro", "Provincia Cunene"), ("ru", "Кунене"), ("si", "ස\u{dd2}ය\u{dd4}න\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("so", "Cunene Province"), ("sr", "Кунене"), ("sr_Latn", "Kunene"), ("sv", "Cunene"), ("sw", "Cunene"), ("ta", "குனேன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}యూన\u{c46}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e39}เนเน\u{e48}"), ("tr", "Cunene Bölgesi"), ("uk", "Кунене"), ("ur", "کونینے صوبہ"), ("vi", "Cunene"), ("yue", "庫內納省"), ("yue_Hans", "库内纳省"), ("zh", "庫內納省")]),
                        unofficial_name_list: ["Cunene"].to_vec(),
                    }
                ),
                (
                    "CNO",
                    Subdivision{
                        name: "Cuanza Norte",
                        country_alpha2: Alpha2::AO,
                        code: "CNO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.2398513), longitude: Some(14.6587821), max_latitude: Some(-7.937077099999999), min_latitude: Some(-9.787161), max_longitude: Some(15.830849), min_longitude: Some(13.9980831)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cuanza Norte-provinsie"), ("ar", "مقاطعة كوانزا نورت"), ("be", "Правінцыя Паўночная Кванза"), ("bg", "Куанза Норте"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}ঞ\u{9cd}জ\u{9be} নর\u{9cd}টে প\u{9cd}রদেশ"), ("ca", "Kwanza-Nord"), ("ccp", "𑄇\u{1112a}𑄠\u{1112e}𑄚\u{11134}𑄎 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Cuanza Norte Province"), ("da", "Cuanza Norte"), ("de", "Provinz Cuanza Norte"), ("el", "Κουάνζα Νόρτε"), ("en", "Cuanza Norte"), ("es", "Cuanza Norte"), ("fa", "استان کوانزای شمالی"), ("fi", "Cuanza Norte"), ("fr", "Kwanza-Nord"), ("gu", "ક\u{ac1}આન\u{acd}ઝા નોર\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{941}आ\u{902}जा नोर\u{94d}ट\u{947} प\u{94d}रा\u{902}त"), ("id", "Cuanza Norte"), ("it", "provincia di Cuanza Nord"), ("ja", "クアンザ・ノルテ州"), ("ka", "ჩრდილოეთი კვანზა"), ("kn", "ಕ\u{ccd}ವಾನ\u{ccd}ಜಾ ನಾರ\u{ccd}ಟ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿠안자노르트 주"), ("lt", "Šiaurės Kvanza"), ("lv", "Ziemeļkvanzas province"), ("mr", "च\u{941}एन\u{94d}झा नॉरट\u{947} प\u{94d}रा\u{902}त"), ("ms", "Cuanza Norte Province"), ("nb", "Cuanza Norte"), ("nl", "Cuanza Norte"), ("no", "Cuanza Norte"), ("pl", "Prowincja Kwanza Północna"), ("pt", "Kwanza-Norte"), ("ro", "Provincia Cuanza Norte"), ("ru", "Северная Кванза"), ("si", "ක\u{dd4}අන\u{dca}ස\u{dcf} නොර\u{dca}ටේ පළ\u{dcf}ත"), ("so", "Cuanza Norte Province"), ("sr", "Северна Кванза"), ("sr_Latn", "Severna Kvanza"), ("sv", "Cuanza Norte"), ("sw", "Cuanza Kaskazini"), ("ta", "ஸுஆன\u{bcd}ச நோர\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కువ\u{c3e}ంజ\u{c3e} న\u{c4b}ర\u{c4d}ట\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ควนซาเหน\u{e37}อ"), ("tr", "Cuanza Norte Bölgesi"), ("uk", "Північна Кванза"), ("ur", "کوانزا شمالی صوبہ"), ("vi", "Cuanza Norte"), ("yue", "北廣薩省"), ("yue_Hans", "北广萨省"), ("zh", "北廣薩省")]),
                        unofficial_name_list: ["Cuanza-Norte"].to_vec(),
                    }
                ),
                (
                    "CUS",
                    Subdivision{
                        name: "Cuanza Sul",
                        country_alpha2: Alpha2::AO,
                        code: "CUS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.595191), longitude: Some(15.4068079), max_latitude: Some(-9.6833171), min_latitude: Some(-12.22624), max_longitude: Some(16.6080669), min_longitude: Some(13.4936401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Cuanza Sul-provinsie"), ("ar", "مقاطعة كوانزا سول"), ("be", "Правінцыя Паўднёвая Кванза"), ("bg", "Куанза Сул"), ("bn", "ক\u{9c1}য\u{9bc}\u{9be}নজ\u{9be} স\u{9c1}ল"), ("ca", "Kwanza-Sud"), ("ccp", "𑄇\u{1112a}𑄠\u{1112e}𑄚\u{11134}𑄎 𑄥\u{1112a}𑄣\u{11134}"), ("ceb", "Kwanza Sul"), ("da", "Cuanza Sul"), ("de", "Provinz Cuanza Sul"), ("el", "Κουάνζα Σούλ"), ("en", "Cuanza Sul"), ("es", "Cuanza Sur"), ("fa", "استان کوانزای جنوبی"), ("fi", "Cuanza Sul"), ("fr", "Kwanza-Sud"), ("gu", "ક\u{ac1}આન\u{acd}ઝા સ\u{ac1}લ"), ("hi", "क\u{941}आ\u{902}जा स\u{941}ल प\u{94d}रा\u{902}त"), ("id", "Cuanza Sul"), ("it", "provincia di Cuanza Sud"), ("ja", "クアンザ・スル州"), ("ka", "სამხრეთი კვანზა"), ("kn", "ಕ\u{ccd}ವಾನ\u{ccd}ಜಾ ಸುಲ\u{ccd}"), ("ko", "쿠안자술 주"), ("lt", "Pietų Kvanza"), ("lv", "Dienvidkvanza"), ("mr", "क\u{941}आन\u{94d}जा स\u{941}ल\u{94d}ल"), ("ms", "Cuanza Sul"), ("nb", "Cuanza Sul"), ("nl", "Cuanza Sul"), ("no", "Cuanza Sul"), ("pl", "Prowincja Kwanza Południowa"), ("pt", "Kwanza-Sul"), ("ro", "Provincia Cuanza Sul"), ("ru", "Южная Кванза"), ("si", "ක\u{dd4}වන\u{dca}ස\u{dcf} සල\u{dca}"), ("so", "Cuanza Sul Province"), ("sr", "Јужна Кванза"), ("sr_Latn", "Južna Kvanza"), ("sv", "Cuanza Sul"), ("sw", "Cuanza Kusini"), ("ta", "சுயநஜ\u{bbe} சூல\u{bcd}"), ("te", "కువ\u{c3e}ంజ\u{c3e} సుల\u{c4d}"), ("th", "แอนเซบ\u{e49}า"), ("tr", "Cuanza Sul Bölgesi"), ("uk", "Південна Кванза"), ("ur", "کوانزا جنوبی صوبہ"), ("vi", "Cuanza Sul"), ("yue", "南廣薩省"), ("yue_Hans", "南广萨省"), ("zh", "南廣薩省")]),
                        unofficial_name_list: ["Cuanza-Sul"].to_vec(),
                    }
                ),
                (
                    "HUA",
                    Subdivision{
                        name: "Huambo",
                        country_alpha2: Alpha2::AO,
                        code: "HUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.766667), longitude: Some(15.733333), max_latitude: Some(-12.7248395), min_latitude: Some(-12.8151062), max_longitude: Some(15.7939885), min_longitude: Some(15.68882)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Huambo-provinsie"), ("ar", "مقاطعة هوامبو"), ("be", "Правінцыя Уамба"), ("bg", "Уамбо"), ("bn", "হ\u{9c1}য\u{9bc}\u{9be}ম\u{9cd}বো প\u{9cd}রদেশ"), ("ca", "Huambo"), ("ccp", "𑄦𑄅\u{1112a}𑄟\u{11134}𑄝\u{1112e}"), ("ceb", "Huambo (lalawigan)"), ("cs", "Huambo"), ("da", "Huambo"), ("de", "Provinz Huambo"), ("el", "Χουάμπο"), ("en", "Huambo"), ("es", "Provincia de Huambo"), ("et", "Huambo provints"), ("fa", "استان هوامبو"), ("fi", "Huambon maakunta"), ("fr", "Huambo"), ("gu", "હ\u{ac1}આમ\u{acd}બો પ\u{acd}રા\u{a82}ત"), ("hi", "ह\u{941}आ\u{902}बो प\u{94d}रा\u{902}त"), ("id", "Provinsi Huambo"), ("it", "provincia di Huambo"), ("ja", "ウアンボ州"), ("ka", "უამბოს პროვინცია"), ("kn", "ಹುಮಾಂಬೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우암부 주"), ("lt", "Huambo provincija"), ("lv", "Hvambo province"), ("mr", "ह\u{941}अम\u{94d}बो प\u{94d}रा\u{902}त"), ("ms", "Huambo Province"), ("nb", "Huambo"), ("nl", "Huambo"), ("no", "Huambo"), ("pl", "Prowincja Huambo"), ("pt", "Huambo"), ("ro", "Provincia Huambo"), ("ru", "Уамбо"), ("si", "හ\u{dd4}අම\u{dca}බෝ පළ\u{dcf}ත"), ("so", "Huambo Province"), ("sr", "Хуамбо"), ("sr_Latn", "Huambo"), ("sv", "Huambo"), ("sw", "Huambo"), ("ta", "ஹுவ\u{bbe}ம\u{bcd}போ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హువ\u{c3e}ంబ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ฮ\u{e39}แอมโบ"), ("tr", "Huambo Bölgesi"), ("uk", "Уамбо"), ("ur", "ہوامبو صوبہ"), ("vi", "Huambo"), ("yue", "萬博省"), ("yue_Hans", "万博省"), ("zh", "万博省")]),
                        unofficial_name_list: ["Huambo"].to_vec(),
                    }
                ),
                (
                    "HUI",
                    Subdivision{
                        name: "Huíla",
                        country_alpha2: Alpha2::AO,
                        code: "HUI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.9280553), longitude: Some(14.6587821), max_latitude: Some(-13.364136), min_latitude: Some(-16.3515149), max_longitude: Some(16.753408), min_longitude: Some(13.2254381)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Huíla-provinsie"), ("ar", "محافظة هويلا"), ("be", "Правінцыя Уіла"), ("bg", "Уила"), ("bn", "ইল\u{9be} প\u{9cd}রদেশ"), ("ca", "Huíla"), ("ccp", "𑄦\u{1112a}𑄃\u{11128}𑄣"), ("ceb", "Huila Province"), ("da", "Huíla"), ("de", "Provinz Huíla"), ("el", "Χουίλα"), ("en", "Huíla"), ("es", "Huila"), ("fa", "استان هوییلا"), ("fi", "Huila"), ("fr", "Huila"), ("gu", "હ\u{ac1}ઈલા પ\u{acd}રા\u{a82}ત"), ("hi", "ह\u{941}इला प\u{94d}रा\u{902}त"), ("id", "Provinsi Huíla"), ("it", "provincia di Huíla"), ("ja", "ウイラ州"), ("ka", "უილის პროვინცია"), ("kn", "ಹುಯ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우일라 주"), ("lt", "Huilos provincija"), ("lv", "Uilas province"), ("mr", "ह\u{941}ईला प\u{94d}रा\u{902}त"), ("ms", "Huila"), ("nb", "Huíla"), ("nl", "Huíla"), ("no", "Huíla"), ("pl", "Prowincja Huíla"), ("pt", "Huíla"), ("ro", "Provincia Huíla"), ("ru", "Уила"), ("si", "හ\u{dd4}ය\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("so", "Huíla Province"), ("sr", "Хуила"), ("sr_Latn", "Huila"), ("sv", "Huíla"), ("sw", "Huíla"), ("ta", "ஹுஇல ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హుయ\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e38}ยลา"), ("tr", "Huíla Bölgesi"), ("uk", "Уїла"), ("ur", "ہوئلا صوبہ"), ("vi", "Huíla"), ("yue", "威拉省"), ("yue_Hans", "威拉省"), ("zh", "威拉省")]),
                        unofficial_name_list: ["Huíla"].to_vec(),
                    }
                ),
                (
                    "LNO",
                    Subdivision{
                        name: "Lunda Norte",
                        country_alpha2: Alpha2::AO,
                        code: "LNO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.352502200000002), longitude: Some(19.1880047), max_latitude: Some(-6.911378), min_latitude: Some(-10.451853), max_longitude: Some(21.942862), min_longitude: Some(17.286396)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lunda Norte-provinsie"), ("ar", "مقاطعة لوندا نورتي"), ("be", "Паўночная Лунда"), ("bg", "Лунда Норте"), ("bn", "ল\u{9c1}ন\u{9cd}ড\u{9be} নর\u{9cd}টে প\u{9cd}রদেশ"), ("ca", "Lunda Norte"), ("ccp", "𑄣\u{1112a}𑄚\u{11134}𑄘 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Lunda Norte Province"), ("da", "Lunda Norte"), ("de", "Provinz Lunda Norte"), ("el", "Λούντα Νόρτε"), ("en", "Lunda Norte"), ("es", "Lunda Norte"), ("fa", "استان لوندای شمالی"), ("fi", "Lunda Norte"), ("fr", "Lunda-Nord"), ("gu", "લ\u{ac1}ન\u{acd}ડા નોર\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{941}\u{902}डा नोर\u{94d}ट\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Lunda Norte"), ("it", "provincia di Lunda Nord"), ("ja", "ルンダ・ノルテ州"), ("ka", "ჩრდილოეთი ლუნდა"), ("kn", "ಲುಂಡಾ ನಾರ\u{ccd}ಟ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "룬다노르트 주"), ("lt", "Šiaurės Lunda"), ("lv", "Ziemeļlundas province"), ("mr", "ल\u{941}\u{902}डा नॉर\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Daerah Lunda Norte"), ("nb", "Lunda Norte"), ("nl", "Lunda Norte"), ("no", "Lunda Norte"), ("pl", "Prowincja Lunda Północna"), ("pt", "Lunda-Norte"), ("ro", "Provincia Lunda Norte"), ("ru", "Северная Лунда"), ("si", "ල\u{dd4}න\u{dca}ඩ\u{dcf} නොර\u{dca}ටේ පළ\u{dcf}ත"), ("so", "Lunda Norte Province"), ("sr", "Северна Лунда"), ("sr_Latn", "Severna Lunda"), ("sv", "Lunda Norte"), ("sw", "Lunda Kaskazini"), ("ta", "லூண\u{bcd}ட\u{bbe} நோர\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లుండ\u{c3e} న\u{c3e}ర\u{c4d}ట\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}นดานอร\u{e4c}เต"), ("tr", "Lunda Norte Bölgesi"), ("uk", "Північна Лунда"), ("ur", "لوندا شمالی صوبہ"), ("vi", "Lunda Norte"), ("yue", "北倫達省"), ("yue_Hans", "北伦达省"), ("zh", "北倫達省")]),
                        unofficial_name_list: ["Lunda Norte"].to_vec(),
                    }
                ),
                (
                    "LSU",
                    Subdivision{
                        name: "Lunda Sul",
                        country_alpha2: Alpha2::AO,
                        code: "LSU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.2866578), longitude: Some(20.7122465), max_latitude: Some(-8.4244639), min_latitude: Some(-11.5039), max_longitude: Some(22.3292389), min_longitude: Some(18.481274)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Lunda Sul-provinsie"), ("ar", "مقاطعة لوندا سول"), ("be", "Правінцыя Паўднёвая Лунда"), ("bg", "Лунда Сул"), ("bn", "ল\u{9c1}ন\u{9cd}ড\u{9be} স\u{9c1}ল প\u{9cd}রদেশ"), ("ca", "Lunda Sul"), ("ccp", "𑄣\u{1112a}𑄚\u{11134}𑄘 𑄥\u{1112a}𑄣\u{11134}"), ("ceb", "Lunda Sul"), ("da", "Lunda Sul"), ("de", "Provinz Lunda Sul"), ("el", "Λούντα Σουλ"), ("en", "Lunda Sul"), ("es", "Lunda Sul"), ("fa", "استان لوندای جنوبی"), ("fi", "Lunda Sul"), ("fr", "Lunda-Sud"), ("gu", "લ\u{ac1}ન\u{acd}ડા સ\u{ac1}લ પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{941}\u{902}डा स\u{941}ल प\u{94d}रा\u{902}त"), ("id", "Provinsi Lunda Sul"), ("it", "provincia di Lunda Sud"), ("ja", "ルンダ・スル州"), ("ka", "სამხრეთი ლუნდა"), ("kn", "ಲುಂಡಾ ಸುಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "룬다술 주"), ("lt", "Pietų Lunda"), ("lv", "Dienvidlundas province"), ("mr", "ल\u{941}\u{902}डा सौ प\u{94d}रा\u{902}त"), ("ms", "Daerah Lunda Sul"), ("nb", "Lunda Sul"), ("nl", "Lunda Sul"), ("no", "Lunda Sul"), ("pl", "Prowincja Lunda Południowa"), ("pt", "Lunda-Sul"), ("ro", "Provincia Lunda Sul"), ("ru", "Южная Лунда"), ("si", "ල\u{dd4}න\u{dca}ඩ\u{dcf} ස\u{dd4}ල\u{dca} පළ\u{dcf}ත"), ("so", "Lunda Sul Province"), ("sr", "Јужна Лунда"), ("sr_Latn", "Južna Lunda"), ("sv", "Lunda Sul"), ("sw", "Lunda Kusini"), ("ta", "லுண\u{bcd}ட சூல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లుండ\u{c3e} సుల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e31}นดาซ\u{e39}ล"), ("tr", "Lunda Sul Bölgesi"), ("uk", "Південна Лунда"), ("ur", "لوندا جنوبی صوبہ"), ("vi", "Lunda Sul"), ("yue", "南倫達省"), ("yue_Hans", "南伦达省"), ("zh", "南倫達省")]),
                        unofficial_name_list: ["Lunda Sul"].to_vec(),
                    }
                ),
                (
                    "LUA",
                    Subdivision{
                        name: "Luanda",
                        country_alpha2: Alpha2::AO,
                        code: "LUA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.838333), longitude: Some(13.234444), max_latitude: Some(-8.756154), min_latitude: Some(-8.9509107), max_longitude: Some(13.4101221), min_longitude: Some(13.1580869)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Luanda-provinsie"), ("ar", "مقاطعة لواندا"), ("be", "правінцыя Луанда"), ("bg", "Луанда"), ("bn", "ল\u{9c1}য\u{9bc}\u{9be}ন\u{9cd}ড\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Luanda"), ("ccp", "𑄣\u{1112a}𑄚\u{11134}𑄘"), ("ceb", "Luanda Province"), ("cs", "Luanda"), ("da", "Luanda"), ("de", "Provinz Luanda"), ("el", "Λουάντα"), ("en", "Luanda"), ("es", "Provincia de Luanda"), ("fa", "استان لوآندا"), ("fi", "Luandan maakunta"), ("fr", "Luanda"), ("gl", "Provincia de Luanda"), ("gu", "લ\u{ac1}આ\u{a82}ડા પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{941}आण\u{94d}डा प\u{94d}रा\u{902}त"), ("id", "Provinsi Luanda"), ("it", "provincia di Luanda"), ("ja", "ルアンダ州"), ("ka", "ლუანდის პროვინცია"), ("kn", "ಲುವಾಂಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루안다 주"), ("lt", "Luandos provincija"), ("lv", "Luandas province"), ("mr", "ल\u{941}आ\u{902}डा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Luanda"), ("nb", "Luanda"), ("nl", "Luanda"), ("no", "Luanda"), ("pl", "Prowincja Luanda"), ("pt", "Luanda"), ("ro", "Provincia Luanda"), ("ru", "Луанда"), ("si", "ල\u{dd4}අන\u{dca}ඩ\u{dcf} පළ\u{dcf}ත"), ("so", "Luanda Province"), ("sr", "Луанда"), ("sr_Latn", "Luanda"), ("sv", "Luanda"), ("sw", "Luanda"), ("ta", "லுஅன\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లువ\u{c3e}ండ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}อ\u{e31}นดา"), ("tr", "Luanda Bölgesi"), ("uk", "Луанда"), ("ur", "لواندا صوبہ"), ("vi", "Luanda"), ("yue", "羅安達省"), ("yue_Hans", "罗安达省"), ("zh", "羅安達省")]),
                        unofficial_name_list: ["Luanda"].to_vec(),
                    }
                ),
                (
                    "MAL",
                    Subdivision{
                        name: "Malange",
                        country_alpha2: Alpha2::AO,
                        code: "MAL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.546899999999999), longitude: Some(16.3387), max_latitude: Some(-9.5124286), min_latitude: Some(-9.572059699999999), max_longitude: Some(16.3876533), min_longitude: Some(16.2992908)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Malanje-provinsie"), ("ar", "مقاطعة مالانجي"), ("be", "Правінцыя Маланжэ"), ("bg", "Маланже"), ("bn", "ম\u{9be}ল\u{9be}ঞ\u{9cd}জি প\u{9cd}রদেশ"), ("ca", "Malanje"), ("ccp", "𑄟𑄣𑄚\u{11134}𑄎𑄬"), ("ceb", "Malanje Province"), ("da", "Malanje"), ("de", "Provinz Malanje"), ("el", "Μαλαντζέ"), ("en", "Malanje"), ("es", "Malanje"), ("fa", "استان مالانژه"), ("fi", "Malanjen maakunta"), ("fr", "Malanje"), ("gu", "મલા\u{a82}જ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "मल\u{902}ज\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Malanje"), ("it", "provincia di Malanje"), ("ja", "マランジェ州"), ("ka", "მალანჟეს პროვინცია"), ("kn", "ಮಲಾನ\u{ccd}ಜ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "말란즈 주"), ("lt", "Malanžės provincija"), ("lv", "Malanžes province"), ("mr", "माला\u{902}ज\u{947} प\u{94d}रा\u{902}त"), ("ms", "Malanje Province"), ("nb", "Malanje"), ("nl", "Malanje"), ("no", "Malanje"), ("pl", "Prowincja Malanje"), ("pt", "Malanje"), ("ro", "Provincia Malanje"), ("ru", "Маланже"), ("si", "මලන\u{dca}ජේ පළ\u{dcf}ත"), ("so", "Malanje Province"), ("sr", "Маланже"), ("sr_Latn", "Malanže"), ("sv", "Malanje"), ("sw", "Malanje"), ("ta", "மலஞ\u{bcd}சே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మల\u{c3e}ంజ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาล\u{e31}นเจ"), ("tr", "Malanje Bölgesi"), ("uk", "Маланже"), ("ur", "مالانجے صوبہ"), ("vi", "Malanje"), ("yue", "馬蘭哲省"), ("yue_Hans", "马兰哲省"), ("zh", "馬蘭哲省")]),
                        unofficial_name_list: ["Malange"].to_vec(),
                    }
                ),
                (
                    "MOX",
                    Subdivision{
                        name: "Moxico",
                        country_alpha2: Alpha2::AO,
                        code: "MOX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.4293579), longitude: Some(20.3308814), max_latitude: Some(-10.58553), min_latitude: Some(-16.207985), max_longitude: Some(24.0821189), min_longitude: Some(17.922985)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Moxico-provinsie"), ("ar", "مقاطعة موكسيكو"), ("be", "Правінцыя Мошыка"), ("bg", "Мошико"), ("bn", "মক\u{9cd}সিকো প\u{9cd}রদেশ"), ("ca", "Moxico"), ("ccp", "𑄟\u{11133}𑄠𑄇\u{11134}𑄥\u{11128}𑄇\u{1112e}"), ("ceb", "Moxico"), ("cs", "Moxico"), ("da", "Moxico"), ("de", "Provinz Moxico"), ("el", "Μοξίκο"), ("en", "Moxico"), ("es", "Moxico"), ("et", "Moxico provints"), ("fa", "استان موشیکو"), ("fi", "Moxico"), ("fr", "Moxico"), ("gl", "Provincia de Moxico"), ("gu", "મોક\u{acd}સીકો પ\u{acd}રા\u{a82}ત"), ("hi", "मोक\u{94d}सिको प\u{94d}रा\u{902}त"), ("id", "Provinsi Moxico"), ("it", "provincia di Moxico"), ("ja", "モシコ州"), ("ka", "მოშიკოს პროვინცია"), ("kn", "ಮೊಕ\u{ccd}ಸ\u{cbf}ಕೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "모시쿠 주"), ("lt", "Mošiko provincija"), ("lv", "Mošiko province"), ("mr", "मोक\u{94d}सिको प\u{94d}रा\u{902}त"), ("ms", "Moxico Province"), ("nb", "Moxico"), ("nl", "Moxico"), ("no", "Moxico"), ("pl", "Prowincja Moxico"), ("pt", "Moxico"), ("ro", "Provincia Moxico"), ("ru", "Мошико"), ("si", "මොක\u{dca}ස\u{dd2}කෝ පළ\u{dcf}ත"), ("so", "Moxico Province"), ("sr", "Мошико"), ("sr_Latn", "Mošiko"), ("sv", "Moxico"), ("sw", "Moxico"), ("ta", "மெக\u{bcd}ஸிகோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c4b}క\u{c4d}స\u{c3f}క\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมอกซ\u{e34}โก"), ("tr", "Moxico Bölgesi"), ("uk", "Мошико"), ("ur", "موشیکو صوبہ"), ("vi", "Moxico"), ("yue", "莫西古省"), ("yue_Hans", "莫西古省"), ("zh", "莫希科省")]),
                        unofficial_name_list: ["Moxico"].to_vec(),
                    }
                ),
                (
                    "NAM",
                    Subdivision{
                        name: "Namibe",
                        country_alpha2: Alpha2::AO,
                        code: "NAM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.195278), longitude: Some(12.150833), max_latitude: Some(-15.1747363), min_latitude: Some(-15.218767), max_longitude: Some(12.1851254), min_longitude: Some(12.120602)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Namibe-provinsie"), ("ar", "مقاطعة ناميبي"), ("be", "Правінцыя Намібэ"), ("bg", "Намибе"), ("bn", "ন\u{9be}মিবে প\u{9cd}রদেশ"), ("ca", "Namibe"), ("ccp", "𑄚𑄟\u{11128}𑄝𑄬"), ("ceb", "Namibe Province"), ("da", "Namibe"), ("de", "Provinz Namibe"), ("el", "Ναμίμπε"), ("en", "Namibe"), ("es", "Namibe"), ("eu", "Namibe"), ("fa", "استان نامیبه"), ("fi", "Namiben maakunta"), ("fr", "Namibe"), ("gu", "નામીબ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "न\u{947}मिब\u{947} प\u{94d}रा\u{902}त"), ("hu", "Namibe Provincia"), ("id", "Provinsi Namibe"), ("it", "provincia di Namibe"), ("ja", "ナミベ州"), ("ka", "ნამიბეს პროვინცია"), ("kn", "ನಾಮ\u{cbf}ಬ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "나미브 주"), ("lt", "Namibo provincija"), ("lv", "Namibes province"), ("mr", "नमीब प\u{94d}रा\u{902}त"), ("ms", "Namibe Province"), ("nb", "Namibe"), ("nl", "Namibe"), ("no", "Namibe"), ("pl", "Prowincja Namibe"), ("pt", "Namibe"), ("ro", "Provincia Namibe"), ("ru", "Намибе"), ("si", "නම\u{dd2}බේ පළ\u{dcf}ත"), ("so", "Namibe Province"), ("sr", "Намибе"), ("sr_Latn", "Namibe"), ("sv", "Namibe"), ("sw", "Namibe"), ("ta", "நமிபே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "నమ\u{c40}బ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนาม\u{e34}บ\u{e35}"), ("tr", "Namibe Bölgesi"), ("uk", "Намібе"), ("ur", "نامیبے صوبہ"), ("vi", "Namibe"), ("yue", "納米貝省"), ("yue_Hans", "纳米贝省"), ("zh", "納米貝省")]),
                        unofficial_name_list: ["Namibe"].to_vec(),
                    }
                ),
                (
                    "UIG",
                    Subdivision{
                        name: "Uíge",
                        country_alpha2: Alpha2::AO,
                        code: "UIG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.616667), longitude: Some(15.05), max_latitude: Some(-7.5901119), min_latitude: Some(-7.6323089), max_longitude: Some(15.0770187), min_longitude: Some(15.0356484)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Uíge-provinsie"), ("ar", "مقاطعة أوجي"), ("be", "Правінцыя Уіжэ"), ("bg", "Уиже"), ("bn", "উগে প\u{9cd}রদেশ"), ("ca", "Uige"), ("ccp", "𑄅\u{1112a}𑄃\u{11128}𑄉𑄬"), ("ceb", "Província do Uíge"), ("da", "Uíge"), ("de", "Provinz Uíge"), ("el", "Ουίγκε"), ("en", "Uíge"), ("es", "Uige"), ("fa", "استان اوییگه"), ("fi", "Uigen maakunta"), ("fr", "Uíge"), ("gu", "ઉએગ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "उइग\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Uíge"), ("it", "provincia di Uíge"), ("ja", "ウイジェ州"), ("ka", "უიჟეს პროვინცია"), ("kn", "ಯುಯ\u{cbf}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우이즈 주"), ("lt", "Uižės provincija"), ("lv", "Uižes province"), ("mr", "उइग\u{947} प\u{94d}रा\u{902}त"), ("ms", "Uíge Province"), ("nb", "Uíge"), ("nl", "Uíge"), ("no", "Uíge"), ("pl", "Prowincja Uíge"), ("pt", "Uíge"), ("ro", "Provincia Uíge"), ("ru", "Уиже"), ("si", "උය\u{dd2}ගේ පළ\u{dcf}ත"), ("so", "Uíge Province"), ("sr", "Ујиже"), ("sr_Latn", "Ujiže"), ("sv", "Uíge"), ("sw", "Uíge"), ("ta", "உய\u{bcd}ஜ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉయ\u{c3f}గ\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาแวรง"), ("tr", "Uíge Bölgesi"), ("uk", "Уїже"), ("ur", "اوئگے صوبہ"), ("vi", "Uíge"), ("yue", "威熱省"), ("yue_Hans", "威热省"), ("zh", "威熱省")]),
                        unofficial_name_list: ["Uíge"].to_vec(),
                    }
                ),
                (
                    "ZAI",
                    Subdivision{
                        name: "Zaire",
                        country_alpha2: Alpha2::AO,
                        code: "ZAI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.573345799999999), longitude: Some(13.1740348), max_latitude: Some(-5.841628099999999), min_latitude: Some(-7.8169699), max_longitude: Some(15.0035509), min_longitude: Some(12.2700424)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zaire-provinsie"), ("ar", "مقاطعة زائير"), ("be", "Правінцыя Заірэ"), ("bg", "Зайре"), ("bn", "জেইরে প\u{9cd}রদেশ"), ("ca", "Zaire"), ("ccp", "𑄎𑄠𑄢\u{11134}"), ("ceb", "Zaire (lalawigan sa Angola)"), ("da", "Zaire"), ("de", "Provinz Zaire"), ("el", "Ζαΐρε"), ("en", "Zaire"), ("es", "Provincia de Zaire"), ("fa", "استان زئیر"), ("fi", "Zairen maakunta"), ("fr", "Zaïre"), ("gu", "ઝ\u{ac8}ર પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{93c}\u{948}र\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Zaire"), ("it", "provincia dello Zaire"), ("ja", "ザイーレ州"), ("ka", "ზაირეს პროვინცია"), ("kn", "ಝೈರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "자이르 주"), ("lt", "Zairo provincija"), ("lv", "Zaires province"), ("mr", "झ\u{948}र प\u{94d}रा\u{902}त"), ("ms", "Zaire Province"), ("nb", "Zaire"), ("nl", "Zaire"), ("no", "Zaire"), ("pl", "Prowincja Zaire"), ("pt", "Zaire"), ("ro", "Provincia Zaire"), ("ru", "Заире"), ("si", "සය\u{dd2}රේ පළ\u{dcf}ත"), ("sr", "Заире"), ("sr_Latn", "Zaire"), ("sv", "Zaire"), ("sw", "Zaire"), ("ta", "சய\u{bcd}யர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c48}ర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแซร\u{e4c}"), ("tr", "Zaire Bölgesi"), ("uk", "Заїре"), ("ur", "زائیر صوبہ"), ("vi", "Zaire"), ("yue", "薩伊省"), ("yue_Hans", "萨伊省"), ("zh", "薩伊省")]),
                        unofficial_name_list: ["Zaire"].to_vec(),
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
#[cfg(feature = "ao")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AO,
        alpha3: Alpha3::AGO,
        address_format: None,
        continent: Continent::Africa,
        country_code: 244,
        currency_code: CurrencyCode::AOA,
        gec: Some(GEC::AO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ANG),
        iso_long_name: "The Republic of Angola",
        iso_short_name: "Angola",
        official_language_list: ["pt"].to_vec(),
        spoken_language_list: ["pt"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "0",
        nationality: Some("Angolan"),
        number: "024",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::MiddleAfrica),
        un_locode: "AO",
        unofficial_name_list: ["Angola", "アンゴラ"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Angola"),
            ("af", "Angola"),
            ("ak", "Angola"),
            ("am", "አንጎላ"),
            ("an", "Angola"),
            ("ar", "أنغولا"),
            ("as", "এংগোল\u{9be}"),
            ("ay", "Angola"),
            ("az", "Anqola"),
            ("ba", "Angola"),
            ("be", "Ангола"),
            ("bg", "Ангола"),
            ("bi", "Angola"),
            ("bn", "অ\u{9cd}য\u{9be}ঙ\u{9cd}গোল\u{9be}"),
            ("bn_IN", "অ\u{9cd}য\u{9be}ঙ\u{9cd}গোল\u{9be}"),
            ("br", "Angola"),
            ("bs", "Angola"),
            ("ca", "Angola"),
            ("ce", "Ангола"),
            ("ch", "Angola"),
            ("cs", "Angola"),
            ("cv", "Ангола"),
            ("cy", "Angola"),
            ("da", "Angola"),
            ("de", "Angola"),
            ("dv", "އ\u{7ac}ނ\u{7b0}ގ\u{7af}ލ\u{7a7}"),
            ("dz", "ཨ\u{f7a}ང་ག\u{f7c}་ལ།"),
            ("ee", "Angola"),
            ("el", "Ανγκόλα"),
            ("en", "Angola"),
            ("eo", "Angolo"),
            ("es", "Angola"),
            ("et", "Angola"),
            ("eu", "Angola"),
            ("fa", "آنگولا"),
            ("ff", "Anngolaa"),
            ("fi", "Angola"),
            ("fo", "Angola"),
            ("fr", "Angola"),
            ("fy", "Angoala"),
            ("ga", "Angóla"),
            ("gl", "Angola"),
            ("gn", "Angola"),
            ("gu", "અ\u{a82}ગોલા"),
            ("gv", "Angoley"),
            ("ha", "Angola"),
            ("he", "אנגולה"),
            ("hi", "अ\u{902}गोला"),
            ("hr", "Angola"),
            ("ht", "Angola"),
            ("hu", "Angola"),
            ("hy", "Անգոլա"),
            ("ia", "Angola"),
            ("id", "Angola"),
            ("io", "Angola"),
            ("is", "Angóla"),
            ("it", "Angola"),
            ("iu", "Angola"),
            ("ja", "アンゴラ"),
            ("ka", "ანგოლა"),
            ("ki", "Angola"),
            ("kk", "Ангола"),
            ("kl", "Angola"),
            ("km", "អង\u{17cb}ហ\u{17d2}គោឡា"),
            ("kn", "ಅಂಗೋಲಾ"),
            ("ko", "앙골라"),
            ("ku", "Angola"),
            ("kv", "Angola"),
            ("kw", "Angola"),
            ("ky", "Ангола"),
            ("lo", "ປະເທດອ\u{eb1}ງໂກລາ"),
            ("lt", "Angola"),
            ("lv", "Angola"),
            ("mi", "Anakora"),
            ("mk", "Ангола"),
            ("ml", "അംഗോള"),
            ("mn", "Ангол"),
            ("mr", "अ\u{902}गोला"),
            ("ms", "Angola"),
            ("mt", "Angola"),
            (
                "my",
                "အင\u{103a}ဂ\u{102d}\u{102f}လာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Angora"),
            ("nb", "Angola"),
            ("ne", "एनगोला"),
            ("nl", "Angola"),
            ("nn", "Angola"),
            ("nv", "Angola"),
            ("oc", "Angòla"),
            ("or", "ଆଙ\u{b4d}ଗୋଲ\u{b3e}"),
            ("pa", "ਅ\u{a70}ਗ\u{a4b}ਲਾ"),
            ("pi", "अ\u{902}गोला"),
            ("pl", "Angola"),
            ("ps", "انګولا"),
            ("pt", "Angola"),
            ("pt_BR", "Angola"),
            ("ro", "Angola"),
            ("ru", "Ангола"),
            ("rw", "Angola"),
            ("sc", "Angola"),
            ("sd", "Angola"),
            ("si", "ඇන\u{dca}ගෝල\u{dcf}ව"),
            ("sk", "Angola"),
            ("sl", "Angola"),
            ("so", "Angoola"),
            ("sq", "Angolë"),
            ("sr", "Ангола"),
            ("sv", "Angola"),
            ("sw", "Angola"),
            ("ta", "அங\u{bcd}கோல\u{bbe}"),
            ("te", "అంగ\u{c4b}ల\u{c3e}"),
            ("tg", "Ангола"),
            ("th", "แองโกลา"),
            ("ti", "ኣንጎላ"),
            ("tk", "Angola"),
            ("tl", "Anggola"),
            ("tr", "Angola"),
            ("tt", "Анgола"),
            ("ug", "ئانگولا"),
            ("uk", "Ангола"),
            ("ur", "انگولا"),
            ("uz", "Angola"),
            ("ve", "Angola"),
            ("vi", "Ăng-gô-la"),
            ("wa", "Angola"),
            ("wo", "Angoolaa"),
            ("xh", "Angola"),
            ("yo", "Àngólà"),
            ("zh_CN", "安哥拉"),
            ("zh_HK", "安哥拉"),
            ("zh_TW", "安哥拉"),
            ("zu", "I-Angola"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
