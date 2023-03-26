// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Belarus

#[cfg(all(feature = "by", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BY;
    pub const ALPHA3: Alpha3 = Alpha3::BLR;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 375;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::BYN;
    pub const GEC: Option<GEC> = Some(GEC::BO);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::BLR);
    pub const ISO_SHORT_NAME: &str = "Belarus";
    pub const ISO_LONG_NAME: &str = "The Republic of Belarus";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["be", "ru"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["be", "ru"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Belarusian");
    pub const NUMBER: &str = "112";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "BY";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Belarus",
        "Weißrussland",
        "Biélorussie",
        "Bielorrusia",
        "ベラルーシ",
        "Wit-Rusland",
        "Беларусь",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Belarus"),
        ("af", "Wit-Rusland"),
        ("ak", "Belarus"),
        ("am", "ቤሒሩስ"),
        ("an", "Belarrusia"),
        ("ar", "روسيا البيضاء"),
        ("as", "বেল\u{9be}ৰছ"),
        ("ay", "Belarus"),
        ("az", "Belarusiya"),
        ("ba", "Belarus"),
        ("be", "Беларусь"),
        ("bg", "Беларус"),
        ("bi", "Belarus"),
        ("bn", "বেল\u{9be}র\u{9c1}স"),
        ("bn_IN", "বেল\u{9be}র\u{9c1}স"),
        ("br", "Belarus"),
        ("bs", "Bjelorusija"),
        ("ca", "Bielorússia"),
        ("ce", "Белорусси"),
        ("ch", "Belarus"),
        ("cs", "Bělorusko"),
        ("cv", "Белорусси"),
        ("cy", "Belarws"),
        ("da", "Hviderusland"),
        ("de", "Weißrussland"),
        ("dv", "ބ\u{7ac}ލ\u{7a6}ރ\u{7ab}ސ\u{7b0}"),
        ("dz", "བ\u{f7a}་ལ་ར\u{f71}ས\u{f72}།"),
        ("ee", "Belarus"),
        ("el", "Λευκορωσία"),
        ("en", "Belarus"),
        ("eo", "Belorusio"),
        ("es", "Bielorrusia"),
        ("et", "Valgevene"),
        ("eu", "Bielorrusia"),
        ("fa", "بلاروس"),
        ("ff", "Belaruusiya"),
        ("fi", "Valko-Venäjä"),
        ("fo", "Hvítarussland"),
        ("fr", "Bélarus"),
        ("fy", "Wyt-Ruslân"),
        ("ga", "An Bhílearúis"),
        ("gl", "Bielorrusia"),
        ("gn", "Belarus"),
        ("gu", "બ\u{ac7}લાર\u{ac1}સ"),
        ("gv", "Yn Velaroosh"),
        ("ha", "A' Bhealaruis"),
        ("he", "בלארוס"),
        ("hi", "ब\u{947}लार\u{942}स"),
        ("hr", "Bjelorusija"),
        ("ht", "Byelorisi"),
        ("hu", "Fehéroroszország"),
        ("hy", "Բելոռուս"),
        ("ia", "Bielorussia"),
        ("id", "Belarus"),
        ("io", "Bielorusia"),
        ("is", "Hvítarússland"),
        ("it", "Bielorussia"),
        ("iu", "Belarus"),
        ("ja", "ベラルーシ"),
        ("ka", "ბელორუსია"),
        ("ki", "Belarus"),
        ("kk", "Беларусь"),
        ("kl", "Belarus"),
        ("km", "បេឡារ\u{17bb}ស\u{17d2}ស"),
        ("kn", "ಬೇಲಾರುಸ\u{ccd}"),
        ("ko", "벨라루스"),
        ("ku", "Belarus"),
        ("kv", "Беларусь"),
        ("kw", "Belarussi"),
        ("ky", "Беларусия"),
        ("lo", "ປະເທດບ\u{eb5}ເອໂລລ\u{eb8}ດ"),
        ("lt", "Baltarusija"),
        ("lv", "Baltkrievija"),
        ("mi", "Pērara"),
        ("mk", "Белорусија"),
        ("ml", "ബെല\u{d3e}റസ\u{d4d}"),
        ("mn", "Белорусс"),
        ("mr", "ब\u{947}लार\u{941}स"),
        ("ms", "Belarus"),
        ("mt", "Belarus"),
        (
            "my",
            "ဘ\u{102e}လာရ\u{102f}ဇ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Berarut"),
        ("nb", "Hviterussland"),
        ("ne", "ब\u{947}लार\u{942}स"),
        ("nl", "Wit-Rusland"),
        ("nn", "Kviterussland"),
        ("nv", "Belarus"),
        ("oc", "Bielorussia"),
        ("or", "ବେଲ\u{b3e}ର\u{b41}ଷ"),
        ("pa", "ਬ\u{a47}ਲਾਰ\u{a42}ਸ"),
        ("pi", "ब\u{947}लार\u{942}स"),
        ("pl", "Białoruś"),
        ("ps", "بېلاروس"),
        ("pt", "Bielorússia"),
        ("pt_BR", "Bielo-Rússia"),
        ("ro", "Bielorusia"),
        ("ru", "Беларусь"),
        ("rw", "Belarusi"),
        ("sc", "Bielorùssia"),
        ("sd", "Belarus"),
        ("si", "බෙල\u{dcf}ර\u{dd4}ස\u{dca}"),
        ("sk", "Bielorusko"),
        ("sl", "Belorusija"),
        ("so", "Belarus"),
        ("sq", "Bjellorusi"),
        ("sr", "Белорусија"),
        ("sv", "Vitryssland"),
        ("sw", "Belarus"),
        ("ta", "பெல\u{bbe}ருஸ\u{bcd}"),
        ("te", "బ\u{c47}ల\u{c3e}రుస\u{c4d}"),
        ("tg", "Беларус"),
        ("th", "เบลาร\u{e38}ส"),
        ("ti", "ቤላሩስ"),
        ("tk", "Belarus"),
        ("tl", "Belarus"),
        ("tr", "Belarus"),
        ("tt", "Беларус"),
        ("ug", "بېلارۇسىيە"),
        ("uk", "Білорусь"),
        ("ur", "بیلاروس"),
        ("uz", "Belarus"),
        ("ve", "Belarus"),
        ("vi", "Be-la-ruxợ"),
        ("wa", "Belaruss"),
        ("wo", "Belaarus"),
        ("xh", "Belarus"),
        ("yo", "Bẹ\u{300}lárùs"),
        ("zh_CN", "白俄罗斯"),
        ("zh_HK", "白俄羅斯"),
        ("zh_TW", "白俄羅斯"),
        ("zu", "IBelarusi"),
    ];
    #[cfg(all(feature = "by", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 53.709807;
        pub const LONGITUDE: f64 = 27.953389;
        pub const MAX_LATITUDE: f64 = 56.1718719;
        pub const MAX_LONGITUDE: f64 = 32.7768202;
        pub const MIN_LATITUDE: f64 = 51.26201100000001;
        pub const MIN_LONGITUDE: f64 = 23.1783377;
        pub const NORTHEAST_LATITUDE: f64 = 56.1718719;
        pub const NORTHEAST_LONGITUDE: f64 = 32.7768202;
        pub const SOUTHWEST_LATITUDE: f64 = 51.26201100000001;
        pub const SOUTHWEST_LONGITUDE: f64 = 23.1783377;
    }
}
#[cfg(all(feature = "by", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 53.709807,
            longitude: 27.953389,
            max_latitude: 56.1718719,
            max_longitude: 32.7768202,
            min_latitude: 51.26201100000001,
            min_longitude: 23.1783377,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 56.1718719,
                    longitude: 32.7768202,
                },
                southwest: CountryGeoBound {
                    latitude: 51.26201100000001,
                    longitude: 23.1783377,
                },
            },
        }
    }
}

#[cfg(all(feature = "by", feature = "subdivisions"))]
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
                    "BR",
                    Subdivision{
                        name: "BR",
                        country_alpha2: Alpha2::BY,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.5296641), longitude: Some(25.460648), max_latitude: Some(53.4118989), min_latitude: Some(51.4980539), max_longitude: Some(27.5825019), min_longitude: Some(23.1783377)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بريست أوبلاست"), ("az", "Brest vilayəti"), ("be", "Брэсцкая вобласць"), ("bg", "Брестка област"), ("bn", "ব\u{9cd}রেস\u{9cd}ট অঞ\u{9cd}চল"), ("bs", "Brestska oblast"), ("ca", "Província de Brest"), ("ccp", "𑄝\u{11133}𑄢𑄬𑄥\u{11133}𑄑\u{11134}"), ("ceb", "Brest Oblast"), ("cs", "Brestská oblast"), ("da", "Brest oblast"), ("de", "Breszkaja Woblasz"), ("el", "Μπρεστ"), ("en", "Brest"), ("es", "Provincia de Brest"), ("et", "Bresti oblast"), ("eu", "Bresteko oblasta"), ("fa", "منطقه بریست"), ("fi", "Brestin alue"), ("fr", "Voblast de Brest"), ("gl", "Rexión de Bierascie"), ("gu", "બ\u{acd}ર\u{ac7}સ\u{acd}ટ પ\u{acd}રદ\u{ac7}શ"), ("he", "ברסט"), ("hi", "ब\u{94d}र\u{947}स\u{94d}ट प\u{94d}रद\u{947}श"), ("hr", "Brestska oblast"), ("hu", "Breszti terület"), ("hy", "Բրեստի մարզ"), ("id", "Provinsi Brest"), ("it", "Regione di Brėst"), ("ja", "ブレスト州"), ("ka", "ბრესტის ოლქი"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "브레스트 주"), ("lt", "Bresto sritis"), ("lv", "Brestas apgabals"), ("mr", "ब\u{94d}र\u{947}स\u{94d}ट प\u{94d}रद\u{947}श"), ("ms", "Brest Region"), ("nb", "Brest"), ("nl", "Oblast Brest"), ("no", "Brest"), ("pl", "Obwód brzeski"), ("pt", "Brest"), ("ro", "Regiunea Brest"), ("ru", "Брестская область"), ("si", "බ\u{dca}\u{200d}රෙස\u{dca}ට\u{dca} කල\u{dcf}පය"), ("sk", "Brestská oblasť"), ("sr", "Брестска област"), ("sr_Latn", "Brestska oblast"), ("sv", "Brests voblast"), ("ta", "பர\u{bcd}ஸ\u{bcd}ட\u{bcd} பகுதி"), ("te", "బ\u{c4d}ర\u{c46}స\u{c4d}ట\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขคเบรสต\u{e4c}"), ("tr", "Brest Voblastı"), ("uk", "Берестейська область"), ("ur", "بریسٹ علاقہ"), ("uz", "Brest viloyati"), ("vi", "Khu vực Brest"), ("zh", "布列斯特州")]),
                        unofficial_name_list: ["Bierascie", "Brest-Litovsk", "Brestskaja Oblastʿ", "Brestskaja Voblastsʿ", "Brestskaya Oblastʿ", "Brestskaya Voblastsʿ", "Brisk", "Brześć nad Bugiem", "Brześć-Litewski"].to_vec(),
                    }
                ),
                (
                    "HM",
                    Subdivision{
                        name: "HM",
                        country_alpha2: Alpha2::BY,
                        code: "HM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Minsk"), ("am", "ሚንስክ"), ("ar", "مينسك"), ("az", "Minsk"), ("be", "Мінск"), ("bg", "Минск"), ("bn", "মিন\u{9cd}\u{200c}স\u{9cd}ক"), ("bs", "Minsk"), ("ca", "Minsk"), ("ccp", "𑄟𑄚\u{11134}𑄥\u{11133}𑄇\u{11134}"), ("ceb", "Minsk"), ("cs", "Minsk"), ("cy", "Minsk"), ("da", "Minsk"), ("de", "Minsk"), ("el", "Μινσκ"), ("en", "Minsk"), ("es", "Minsk"), ("et", "Minsk"), ("eu", "Minsk"), ("fa", "مینسک"), ("fi", "Minsk"), ("fr", "Minsk"), ("ga", "Minsc"), ("gl", "Minsk"), ("gu", "મિન\u{acd}સ\u{acd}ક"), ("he", "מינסק"), ("hi", "मिन\u{94d}\u{200d}स\u{94d}\u{200d}क"), ("hr", "Minsk"), ("hu", "Minszk"), ("hy", "Մինսկ"), ("id", "Minsk"), ("is", "Minsk"), ("it", "Minsk"), ("ja", "ミンスク"), ("jv", "Minsk"), ("ka", "მინსკი"), ("kk", "Минск"), ("kn", "ಮ\u{cbf}ನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd}"), ("ko", "민스크"), ("ky", "Минск"), ("lt", "Minskas"), ("lv", "Minska"), ("mk", "Минск"), ("ml", "മിൻസ\u{d4d}ക\u{d4d}"), ("mn", "Минск"), ("mr", "मिन\u{94d}\u{200d}स\u{94d}\u{200d}क"), ("ms", "Minsk"), ("nb", "Minsk"), ("ne", "मिन\u{94d}स\u{94d}क"), ("nl", "Minsk"), ("no", "Minsk"), ("pa", "ਮਿ\u{a70}ਸਕ"), ("pl", "Mińsk"), ("pt", "Minsk"), ("ro", "Minsk"), ("ru", "Минск"), ("si", "ම\u{dd2}න\u{dca}ස\u{dca}ක\u{dca}"), ("sk", "Minsk"), ("sl", "Minsk"), ("sq", "Minsk"), ("sr", "Минск"), ("sr_Latn", "Minsk"), ("sv", "Minsk"), ("sw", "Minsk"), ("ta", "மின\u{bcd}ஸ\u{bcd}க\u{bcd}"), ("te", "మ\u{c3f}న\u{c4d}స\u{c4d}క\u{c4d}"), ("th", "ม\u{e34}นสค\u{e4c}"), ("tk", "Minsk"), ("tr", "Minsk"), ("uk", "Мінськ"), ("ur", "منسک"), ("uz", "Minsk"), ("vi", "Minsk"), ("yo", "Minsk"), ("yo_BJ", "Minsk"), ("yue", "明斯克"), ("yue_Hans", "明斯克"), ("zh", "明斯克"), ("zu", "Minsk")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "HO",
                    Subdivision{
                        name: "HO",
                        country_alpha2: Alpha2::BY,
                        code: "HO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(52.1648754), longitude: Some(29.1333251), max_latitude: Some(53.3679551), min_latitude: Some(51.2620722), max_longitude: Some(31.7992701), min_longitude: Some(27.2441409)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غومل أوبلاست"), ("az", "Qomel vilayəti"), ("be", "Гомельская вобласць"), ("bg", "Гомелска област"), ("bn", "গোমেল অঞ\u{9cd}চল"), ("bs", "Gomelska oblast"), ("ca", "Província de Hòmiel"), ("ccp", "𑄦\u{1112e}𑄟𑄬𑄣\u{11134}"), ("ceb", "Gomel Oblast"), ("cs", "Homelská oblast"), ("da", "Homel voblast"), ("de", "Woblast Homel"), ("el", "Γκόμελ"), ("en", "Homel"), ("es", "Provincia de Gómel"), ("et", "Homieli oblast"), ("eu", "Homelgo oblasta"), ("fa", "منطقه گومل"), ("fi", "Homelin alue"), ("fr", "voblast de Homiel"), ("gl", "Rexión de Homieĺ"), ("gu", "ગોમ\u{ac7}લ પ\u{acd}રદ\u{ac7}શ"), ("he", "הומל"), ("hi", "गोम\u{947}ल क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Gomelska oblast"), ("hu", "Homeli terület"), ("hy", "Գոմելի մարզ"), ("id", "Provinsi Homiel"), ("it", "Regione di Homel’"), ("ja", "ホメリ州"), ("ka", "გომელის ოლქი"), ("kn", "ಗೊಮ\u{cc6}ಲ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "호멜 주"), ("lt", "Gomelio sritis"), ("lv", "Gomeļas apgabals"), ("mk", "Гомељска област"), ("mr", "गोम\u{947}ल प\u{94d}रद\u{947}श"), ("ms", "Gomel Region"), ("nb", "Homjel"), ("nl", "Oblast Homel"), ("no", "Homjel"), ("pl", "obwód homelski"), ("pt", "Voblast de Homiel"), ("ro", "Regiunea Gomel"), ("ru", "Гомельская область"), ("si", "ගොමෙල\u{dca} කල\u{dcf}පය"), ("sk", "Homeľská oblasť"), ("sr", "Гомељска област"), ("sr_Latn", "Gomeljska oblast"), ("sv", "Homels voblast"), ("ta", "கோமேல\u{bcd} பகுதி"), ("te", "గ\u{c4b}మ\u{c46}ల\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "กอร\u{e4c}เมล"), ("tr", "Homyel Voblastı"), ("uk", "Гомельська область"), ("ur", "گومل علاقہ"), ("uz", "Gomel viloyati"), ("vi", "Khu vực Gomel"), ("zh", "戈梅利州")]),
                        unofficial_name_list: ["Gomel", "Gomelskaja Oblastʿ", "Gomelskaya Oblastʿ", "Gomelʿ", "Homelskaja Voblastsʿ", "Homelskaya Voblastsʿ", "Homiel", "Homyel"].to_vec(),
                    }
                ),
                (
                    "HR",
                    Subdivision{
                        name: "HR",
                        country_alpha2: Alpha2::BY,
                        code: "HR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أوبلاست جردونه"), ("az", "Qrodno vilayəti"), ("be", "Гродзенская вобласць"), ("bg", "Гродненска област"), ("bn", "গ\u{9cd}রোদনো অঞ\u{9cd}চল"), ("bs", "Grodnenska oblast"), ("ca", "Província de Hrodna"), ("ccp", "𑄦\u{11127}𑄢\u{11127}𑄚\u{11134}𑄓"), ("ceb", "Grodno Oblast"), ("cs", "Hrodenská oblast"), ("da", "Hrodna oblast"), ("de", "Woblast Hrodna"), ("el", "Περιφέρεια Γκροντνό"), ("en", "Hrodna"), ("es", "Provincia de Grodno"), ("et", "Hrodna oblast"), ("eu", "Hrodnako oblasta"), ("fa", "منطقه گرودنو"), ("fi", "Hrodnan alue"), ("fr", "Voblast de Hrodna"), ("gu", "ગ\u{acd}રોડનો પ\u{acd}રદ\u{ac7}શ"), ("he", "הורדנה"), ("hi", "ग\u{94d}रोद\u{94d}नो क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Grodnenska oblast"), ("hu", "Hrodnai terület"), ("hy", "Գրոդնոյի մարզ"), ("id", "Provinsi Hrodna"), ("it", "Regione di Hrodna"), ("ja", "フロドナ州"), ("ka", "გროდნოს ოლქი"), ("kn", "ಗ\u{ccd}ರೋಡ\u{ccd}ನೊ ಪ\u{ccd}ರದೇಶ"), ("ko", "흐로드나 주"), ("lt", "Gardino sritis"), ("lv", "Grodņas apgabals"), ("mk", "Гродненска област"), ("mr", "ग\u{94d}रोडनो प\u{94d}रद\u{947}श"), ("ms", "Grodno Region"), ("nb", "Hrodna"), ("nl", "Oblast Hrodna"), ("no", "Hrodna"), ("pl", "Obwód grodzieński"), ("pt", "Hrodna"), ("ro", "Regiunea Hrodna"), ("ru", "Гродненская область"), ("si", "ග\u{dca}රෝඩ\u{dca}නෝ කල\u{dcf}පය"), ("sk", "Hrodnianska oblasť"), ("sr", "Гродњенска област"), ("sr_Latn", "Grodnjenska oblast"), ("sv", "Hrodnas voblast"), ("ta", "கரோடனோ பகுதி"), ("te", "గ\u{c4d}ర\u{c3e}డ\u{c4d}న\u{c4b} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "กลอดนอร\u{e4c}"), ("tr", "Hrodna Voblastı"), ("uk", "Гродненська область"), ("ur", "گرودنو علاقہ"), ("uz", "Grodno viloyati"), ("vi", "Khu vực Grodno"), ("zh", "格羅德諾州")]),
                        unofficial_name_list: ["Gardinas", "Grodnenskaja Oblastʿ", "Grodnenskaya Oblastʿ", "Grodno", "Horadnia", "Hrodno", "Hrodzenskaja Voblastsʿ", "Hrodzenskaya Voblastsʿ"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::BY,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "موغيليوف أوبلاست"), ("az", "Mogilyov vilayəti"), ("be", "Магілёўская вобласць"), ("bg", "Могильовска област"), ("bn", "ম\u{9c1}গিলেভ অঞ\u{9cd}চল"), ("bs", "Mogiljovska oblast"), ("ca", "Província de Mahiliou"), ("ccp", "𑄟\u{11133}𑄠𑄉\u{11128}𑄣\u{11128}𑄅\u{1112a}"), ("ceb", "Mogilyov Oblast"), ("cs", "Mohylevská oblast"), ("da", "Mahiljow voblast"), ("de", "Woblast Mahiljou"), ("el", "Μογκίλεφ"), ("en", "Magileu"), ("es", "Provincia de Maguilov"), ("et", "Mahiloŭ oblast"), ("eu", "Mahiliouko oblasta"), ("fa", "منطقه موگیلف"), ("fi", "Mahiljoun alue"), ("fr", "voblast de Moguilev"), ("gl", "Rexión de Mahilioŭ"), ("gu", "મોગીલ\u{ac7}વ પ\u{acd}રદ\u{ac7}શ"), ("he", "מחוז מוהילב"), ("hi", "मोगिल\u{947}व क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Mogilevska oblast"), ("hu", "Mahiljovi terület"), ("hy", "Մոգիլյովի մարզ"), ("id", "Provinsi Mahilyow"), ("it", "Regione di Mahilëŭ"), ("ja", "マヒリョウ州"), ("ka", "მოგილევის ოლქი"), ("kk", "Могилев облысы"), ("kn", "ಮೋಗ\u{cbf}ಲ\u{cc6}ವ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "마힐료우 주"), ("ky", "Могилёв областы"), ("lt", "Mogiliavo sritis"), ("lv", "Mogiļevas apgabals"), ("mr", "मोगिल\u{947}व\u{94d}ह प\u{94d}रद\u{947}श"), ("ms", "Mogilev Region"), ("nb", "Mahiljow"), ("ne", "मोगिल\u{947}भ क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Oblast Mahiljow"), ("no", "Mahiljow"), ("pl", "Obwód mohylewski"), ("pt", "Voblast de Mahilou"), ("ro", "Regiunea Moghilău"), ("ru", "Могилёвская область"), ("si", "මොග\u{dd2}ලෙව\u{dca} කල\u{dcf}පය"), ("sk", "Mahiľovská oblasť"), ("sr", "Могиљовска област"), ("sr_Latn", "Mogiljovska oblast"), ("sv", "Mahiljoŭs voblast"), ("ta", "மோகிளேவ\u{bcd} பகுதி"), ("te", "మ\u{c4b}గ\u{c3f}ల\u{c47}వ\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "จ\u{e31}งหว\u{e31}ดโบล\u{e34}วาร\u{e4c}"), ("tr", "Mahilyov Voblastı"), ("uk", "Могильовська область"), ("ur", "موگیلیف علاقہ"), ("uz", "Mogilyov viloyati"), ("vi", "Khu vực Mogilev"), ("zh", "莫吉廖夫州")]),
                        unofficial_name_list: ["Mahiljov", "Mahiljowskaja Voblastsʿ", "Mahilyov", "Mahilyowskaya Voblastsʿ", "Mahilëv", "Mahilëŭ", "Mogilev", "Mogiliov", "Mogiljovskaja Oblastʿ", "Mogilov", "Mogilyovskaya Oblast", "Mogilëv", "Mogilʿov"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "MI",
                        country_alpha2: Alpha2::BY,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(53.4718561), longitude: Some(27.6969909), max_latitude: Some(55.017477), min_latitude: Some(52.37261179999999), max_longitude: Some(29.487841), min_longitude: Some(26.06101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم مينسك"), ("az", "Minsk vilayəti"), ("be", "Мінская вобласць"), ("bg", "Минска област"), ("bn", "মিন\u{9cd}সক অঞ\u{9cd}চল"), ("bs", "Minska oblast"), ("ca", "Província de Minsk"), ("ccp", "𑄟\u{11128}𑄚\u{11134}𑄥\u{11133}𑄇\u{11134} 𑄢𑄬𑄎\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Minsk Oblast"), ("cs", "Minská oblast"), ("cy", "Minsk Region"), ("da", "Minsk voblast"), ("de", "Woblast Minsk"), ("el", "Περιφέρεια Μινσκ"), ("en", "Minsk Region"), ("es", "Provincia de Minsk"), ("et", "Minski oblast"), ("eu", "Minskeko oblasta"), ("fa", "منطقه مینسک"), ("fi", "Minskin alue"), ("fr", "Voblast de Minsk"), ("gl", "Rexión de Minsk"), ("gu", "મિન\u{acd}સ\u{acd}ક પ\u{acd}રદ\u{ac7}શ"), ("he", "מינסק²"), ("hi", "मिन\u{94d}स\u{94d}क क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Minska oblast"), ("hu", "Minszki terület"), ("hy", "Մինսկի մարզ"), ("id", "Provinsi Minsk"), ("it", "Regione di Minsk"), ("ja", "ミンスク州"), ("ka", "მინსკის ოლქი"), ("kn", "ಮ\u{cbf}ನ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "민스크 주"), ("lt", "Minsko sritis"), ("lv", "Minskas apgabals"), ("mk", "Минска област"), ("mr", "मिन\u{94d}स\u{94d}क प\u{94d}रद\u{947}श"), ("ms", "Minsk Region"), ("nb", "Minsk²"), ("nl", "Oblast Minsk"), ("no", "Minsk²"), ("pl", "Obwód miński"), ("pt", "Voblast de Minsk"), ("ro", "Regiunea Minsk"), ("ru", "Минская область"), ("si", "ම\u{dd2}න\u{dca}ස\u{dca}ක\u{dca} කල\u{dcf}පය"), ("sk", "Minská oblasť"), ("sl", "regija Minsk"), ("sr", "Минска област"), ("sr_Latn", "Minska oblast"), ("sv", "Minsks voblast"), ("ta", "மின\u{bcd}ஸ\u{bcd}க\u{bcd} பகுதி"), ("te", "మ\u{c3f}న\u{c4d}స\u{c46}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "ม\u{e34}นสก\u{e4c}"), ("tr", "Minsk Voblastı"), ("uk", "Мінська область"), ("ur", "منسک علاقہ"), ("uz", "Minsk viloyati"), ("vi", "Khu vực Minsk"), ("zh", "明斯克州")]),
                        unofficial_name_list: ["Minskaja Oblastʿ", "Minskaya Oblastʿ", "Minskaya Voblastsʿ"].to_vec(),
                    }
                ),
                (
                    "VI",
                    Subdivision{
                        name: "VI",
                        country_alpha2: Alpha2::BY,
                        code: "VI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Oblast,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فيتسبك أوبلاست"), ("az", "Vitebsk vilayəti"), ("be", "Віцебская вобласць"), ("bg", "Витебска област"), ("bn", "ভিটেবস\u{9cd}ক অঞ\u{9cd}চল"), ("bs", "Vitebska oblast"), ("ca", "Província de Vítsiebsk"), ("ccp", "𑄞\u{1112d}𑄑𑄬𑄛\u{11134}𑄥\u{11133}𑄇\u{11134}"), ("ceb", "Vitebsk Oblast"), ("cs", "Vitebská oblast"), ("da", "Vitebsk voblast"), ("de", "Woblast Wizebsk"), ("el", "Περιφέρεια Βιτσέμπσκ"), ("en", "Vitebsk"), ("es", "Provincia de Vítebsk"), ("et", "Viciebski oblast"), ("eu", "Vitsebskeko oblasta"), ("fa", "منطقه ویتبسک"), ("fi", "Vitsebskin alue"), ("fr", "Voblast de Vitebsk"), ("gu", "વિટ\u{ac7}બ\u{acd}સ\u{acd}ક પ\u{acd}રદ\u{ac7}શ"), ("he", "ויטבסק"), ("hi", "वाईट\u{947}ब\u{94d}स\u{94d}क प\u{94d}रा\u{902}त"), ("hr", "Vitebska oblast"), ("hu", "Vicebszki terület"), ("hy", "Վիտեբսկի մարզ"), ("id", "Provinsi Vitsebsk"), ("it", "Regione di Vicebsk"), ("ja", "ヴィーツェプスク州"), ("ka", "ვიტებსკის ოლქი"), ("kn", "ವೀಟ\u{cc6}ಬ\u{ccd}ಸ\u{ccd}ಕ\u{ccd} ಪ\u{ccd}ರದೇಶ"), ("ko", "비쳅스크 주"), ("ky", "Витебск облусу"), ("lt", "Vitebsko sritis"), ("lv", "Vitebskas apgabals"), ("mk", "Витепска област"), ("mr", "विट\u{947}ब\u{94d}स\u{94d}क प\u{94d}रद\u{947}श"), ("ms", "Daerah Vitebsk"), ("nb", "Vitebsk"), ("nl", "Oblast Vitebsk"), ("no", "Vitebsk"), ("pl", "obwód witebski"), ("pt", "Voblast de Viciebsk"), ("ro", "Regiunea Vițebsk"), ("ru", "Витебская область"), ("si", "ව\u{dd2}ටෙබ\u{dca}ස\u{dca}ක\u{dca} කල\u{dcf}පය"), ("sk", "Vicebská oblasť"), ("sr", "Витепска област"), ("sr_Latn", "Vitepska oblast"), ("sv", "Vitsebsks voblast"), ("ta", "விடேபஸ\u{bcd}க\u{bcd} பகுதி"), ("te", "వ\u{c3f}ట\u{c46}స\u{c4d}క\u{c4d} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "เขตไวเทบสก\u{e4c}"), ("tr", "Vitsebsk Voblastı"), ("uk", "Вітебська область"), ("ur", "ویٹبسک علاقہ"), ("uz", "Vitebsk viloyati"), ("vi", "Khu vực Vitebsk"), ("zh", "维捷布斯克州")]),
                        unofficial_name_list: ["Vicebskaja Voblastsʿ", "Vicebskaya Voblastsʿ", "Viciebsk", "Vicjebsk", "Vitebsk", "Vitebskaja Oblastʿ", "Vitebskaya Oblastʿ", "Vitsyebsk"].to_vec(),
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
#[cfg(feature = "by")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BY,
        alpha3: Alpha3::BLR,
        address_format: None,
        continent: Continent::Europe,
        country_code: 375,
        currency_code: CurrencyCode::BYN,
        gec: Some(GEC::BO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::BLR),
        iso_long_name: "The Republic of Belarus",
        iso_short_name: "Belarus",
        official_language_list: ["be", "ru"].to_vec(),
        spoken_language_list: ["be", "ru"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "8",
        nationality: Some("Belarusian"),
        number: "112",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "BY",
        unofficial_name_list: [
            "Belarus",
            "Weißrussland",
            "Biélorussie",
            "Bielorrusia",
            "ベラルーシ",
            "Wit-Rusland",
            "Беларусь",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Belarus"),
            ("af", "Wit-Rusland"),
            ("ak", "Belarus"),
            ("am", "ቤሒሩስ"),
            ("an", "Belarrusia"),
            ("ar", "روسيا البيضاء"),
            ("as", "বেল\u{9be}ৰছ"),
            ("ay", "Belarus"),
            ("az", "Belarusiya"),
            ("ba", "Belarus"),
            ("be", "Беларусь"),
            ("bg", "Беларус"),
            ("bi", "Belarus"),
            ("bn", "বেল\u{9be}র\u{9c1}স"),
            ("bn_IN", "বেল\u{9be}র\u{9c1}স"),
            ("br", "Belarus"),
            ("bs", "Bjelorusija"),
            ("ca", "Bielorússia"),
            ("ce", "Белорусси"),
            ("ch", "Belarus"),
            ("cs", "Bělorusko"),
            ("cv", "Белорусси"),
            ("cy", "Belarws"),
            ("da", "Hviderusland"),
            ("de", "Weißrussland"),
            ("dv", "ބ\u{7ac}ލ\u{7a6}ރ\u{7ab}ސ\u{7b0}"),
            ("dz", "བ\u{f7a}་ལ་ར\u{f71}ས\u{f72}།"),
            ("ee", "Belarus"),
            ("el", "Λευκορωσία"),
            ("en", "Belarus"),
            ("eo", "Belorusio"),
            ("es", "Bielorrusia"),
            ("et", "Valgevene"),
            ("eu", "Bielorrusia"),
            ("fa", "بلاروس"),
            ("ff", "Belaruusiya"),
            ("fi", "Valko-Venäjä"),
            ("fo", "Hvítarussland"),
            ("fr", "Bélarus"),
            ("fy", "Wyt-Ruslân"),
            ("ga", "An Bhílearúis"),
            ("gl", "Bielorrusia"),
            ("gn", "Belarus"),
            ("gu", "બ\u{ac7}લાર\u{ac1}સ"),
            ("gv", "Yn Velaroosh"),
            ("ha", "A' Bhealaruis"),
            ("he", "בלארוס"),
            ("hi", "ब\u{947}लार\u{942}स"),
            ("hr", "Bjelorusija"),
            ("ht", "Byelorisi"),
            ("hu", "Fehéroroszország"),
            ("hy", "Բելոռուս"),
            ("ia", "Bielorussia"),
            ("id", "Belarus"),
            ("io", "Bielorusia"),
            ("is", "Hvítarússland"),
            ("it", "Bielorussia"),
            ("iu", "Belarus"),
            ("ja", "ベラルーシ"),
            ("ka", "ბელორუსია"),
            ("ki", "Belarus"),
            ("kk", "Беларусь"),
            ("kl", "Belarus"),
            ("km", "បេឡារ\u{17bb}ស\u{17d2}ស"),
            ("kn", "ಬೇಲಾರುಸ\u{ccd}"),
            ("ko", "벨라루스"),
            ("ku", "Belarus"),
            ("kv", "Беларусь"),
            ("kw", "Belarussi"),
            ("ky", "Беларусия"),
            ("lo", "ປະເທດບ\u{eb5}ເອໂລລ\u{eb8}ດ"),
            ("lt", "Baltarusija"),
            ("lv", "Baltkrievija"),
            ("mi", "Pērara"),
            ("mk", "Белорусија"),
            ("ml", "ബെല\u{d3e}റസ\u{d4d}"),
            ("mn", "Белорусс"),
            ("mr", "ब\u{947}लार\u{941}स"),
            ("ms", "Belarus"),
            ("mt", "Belarus"),
            (
                "my",
                "ဘ\u{102e}လာရ\u{102f}ဇ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Berarut"),
            ("nb", "Hviterussland"),
            ("ne", "ब\u{947}लार\u{942}स"),
            ("nl", "Wit-Rusland"),
            ("nn", "Kviterussland"),
            ("nv", "Belarus"),
            ("oc", "Bielorussia"),
            ("or", "ବେଲ\u{b3e}ର\u{b41}ଷ"),
            ("pa", "ਬ\u{a47}ਲਾਰ\u{a42}ਸ"),
            ("pi", "ब\u{947}लार\u{942}स"),
            ("pl", "Białoruś"),
            ("ps", "بېلاروس"),
            ("pt", "Bielorússia"),
            ("pt_BR", "Bielo-Rússia"),
            ("ro", "Bielorusia"),
            ("ru", "Беларусь"),
            ("rw", "Belarusi"),
            ("sc", "Bielorùssia"),
            ("sd", "Belarus"),
            ("si", "බෙල\u{dcf}ර\u{dd4}ස\u{dca}"),
            ("sk", "Bielorusko"),
            ("sl", "Belorusija"),
            ("so", "Belarus"),
            ("sq", "Bjellorusi"),
            ("sr", "Белорусија"),
            ("sv", "Vitryssland"),
            ("sw", "Belarus"),
            ("ta", "பெல\u{bbe}ருஸ\u{bcd}"),
            ("te", "బ\u{c47}ల\u{c3e}రుస\u{c4d}"),
            ("tg", "Беларус"),
            ("th", "เบลาร\u{e38}ส"),
            ("ti", "ቤላሩስ"),
            ("tk", "Belarus"),
            ("tl", "Belarus"),
            ("tr", "Belarus"),
            ("tt", "Беларус"),
            ("ug", "بېلارۇسىيە"),
            ("uk", "Білорусь"),
            ("ur", "بیلاروس"),
            ("uz", "Belarus"),
            ("ve", "Belarus"),
            ("vi", "Be-la-ruxợ"),
            ("wa", "Belaruss"),
            ("wo", "Belaarus"),
            ("xh", "Belarus"),
            ("yo", "Bẹ\u{300}lárùs"),
            ("zh_CN", "白俄罗斯"),
            ("zh_HK", "白俄羅斯"),
            ("zh_TW", "白俄羅斯"),
            ("zu", "IBelarusi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
