// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Belgium

#[cfg(all(feature = "be", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::BE;
    pub const ALPHA3: Alpha3 = Alpha3::BEL;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 32;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::BE);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::BEL);
    pub const ISO_SHORT_NAME: &str = "Belgium";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Belgium";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["de", "fr", "nl"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["de", "fr", "nl"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Belgian");
    pub const NUMBER: &str = "056";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "BE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Belgium",
        "Belgien",
        "Belgique",
        "Bélgica",
        "ベルギー",
        "België",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Belgium"),
        ("af", "België"),
        ("ak", "Belgium"),
        ("am", "ቤሔጄሤ"),
        ("an", "Belchica"),
        ("ar", "بلجيكا"),
        ("as", "বেলজিয়\u{9be}ম"),
        ("ay", "Belgium"),
        ("az", "Belçika"),
        ("ba", "Belgium"),
        ("be", "Бельгія"),
        ("bg", "Белгия"),
        ("bi", "Belgium"),
        ("bn", "বেলজিয়\u{9be}ম"),
        ("bn_IN", "বেলজিয়\u{9be}ম"),
        ("br", "Belgia"),
        ("bs", "Belgija"),
        ("ca", "Bèlgica"),
        ("ce", "Бельги"),
        ("ch", "Belgium"),
        ("cs", "Belgie"),
        ("cv", "Бельги"),
        ("cy", "Gwlad Belg"),
        ("da", "Belgien"),
        ("de", "Belgien"),
        ("dv", "ބ\u{7ac}ލ\u{7b0}ޖ\u{7a8}އ\u{7a6}މ\u{7b0}"),
        ("dz", "བ\u{f7a}ལ་ཇ\u{f72}་ཡམ།"),
        ("ee", "Belgium"),
        ("el", "Βέλγιο"),
        ("en", "Belgium"),
        ("eo", "Belgio"),
        ("es", "Bélgica"),
        ("et", "Belgia"),
        ("eu", "Belgika"),
        ("fa", "بلژیک"),
        ("ff", "Beljik"),
        ("fi", "Belgia"),
        ("fo", "Belgia"),
        ("fr", "Belgique"),
        ("fy", "Belgje"),
        ("ga", "An Bheilg"),
        ("gl", "Bélxica"),
        ("gn", "Belgium"),
        ("gu", "બ\u{ac7}લ\u{acd}જીયમ"),
        ("gv", "Yn Velg"),
        ("ha", "Beljik"),
        ("he", "בלגיה"),
        ("hi", "ब\u{947}ल\u{94d}जियम"),
        ("hr", "Belgija"),
        ("ht", "Bèljik"),
        ("hu", "Belgium"),
        ("hy", "Բելգիա"),
        ("ia", "Belgica"),
        ("id", "Belgia"),
        ("io", "Belgia"),
        ("is", "Belgía"),
        ("it", "Belgio"),
        ("iu", "Belgium"),
        ("ja", "ベルギー"),
        ("ka", "ბელგია"),
        ("ki", "Belgium"),
        ("kk", "Бельгия"),
        ("kl", "Belgium"),
        ("km", "បែល\u{200b}ហ\u{17d2}ស\u{17ca}\u{17b7}ក"),
        ("kn", "ಬ\u{cc6}ಲ\u{ccd}ಜ\u{cbf}ಯಂ"),
        ("ko", "벨기에"),
        ("ku", "Belçîka"),
        ("kv", "Бельгия"),
        ("kw", "Pow Belg"),
        ("ky", "Бельгия"),
        ("lo", "ປະເທດແບນຊ\u{eb4}ກ"),
        ("lt", "Belgija"),
        ("lv", "Beļģija"),
        ("mi", "Pehiamu"),
        ("mk", "Белгија"),
        ("ml", "ബെല\u{d4d}\u{200d}ജിയം"),
        ("mn", "Белги"),
        ("mr", "ब\u{947}ल\u{94d}जियम"),
        ("ms", "Belgium"),
        ("mt", "Belġju"),
        (
            "my",
            "ဘယ\u{103a}လ\u{103a}ဂျ\u{102e}ယမ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Berdjiyum"),
        ("nb", "Belgia"),
        ("ne", "ब\u{947}ल\u{94d}जियम"),
        ("nl", "België"),
        ("nn", "Belgia"),
        ("nv", "Bélgii Bikéyah"),
        ("oc", "Belgica"),
        ("or", "ବେଲଜ\u{b3f}ୟମ"),
        ("pa", "ਬ\u{a48}ਲਜੀਅਮ"),
        ("pi", "ब\u{947}ल\u{94d}जियम"),
        ("pl", "Belgia"),
        ("ps", "بلجیم"),
        ("pt", "Bélgica"),
        ("pt_BR", "Bélgica"),
        ("ro", "Belgia"),
        ("ru", "Бельгия"),
        ("rw", "Ububiligi"),
        ("sc", "Bèlgiu"),
        ("sd", "بيلجيم"),
        ("si", "බෙල\u{dca}ජ\u{dd2}යම"),
        ("sk", "Belgicko"),
        ("sl", "Belgija"),
        ("so", "Beljiyam"),
        ("sq", "Belgjikë"),
        ("sr", "Белгија"),
        ("sv", "Belgien"),
        ("sw", "Ubelgiji"),
        ("ta", "பெல\u{bcd}ஜியம\u{bcd}"),
        ("te", "బ\u{c46}ల\u{c4d}జ\u{c3f}యమ\u{c4d}"),
        ("tg", "Белгия"),
        ("th", "เบลเย\u{e35}ยม"),
        ("ti", "ቤልጄም"),
        ("tk", "Belgiýa"),
        ("tl", "Belhika"),
        ("tr", "Belçika"),
        ("tt", "Белgиа"),
        ("ug", "بېلگىيە"),
        ("uk", "Бельгія"),
        ("ur", "بلجئیم"),
        ("uz", "Belgiya"),
        ("ve", "Belgium"),
        ("vi", "Bỉ"),
        ("wa", "Beldjike"),
        ("wo", "Beljik"),
        ("xh", "Belgium"),
        ("yo", "Bẹ\u{301}ljíọ\u{300}m"),
        ("zh_CN", "比利时"),
        ("zh_HK", "比利時"),
        ("zh_TW", "比利時"),
        ("zu", "Isi-Bhelijiyamu"),
    ];
    #[cfg(all(feature = "be", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 50.503887;
        pub const LONGITUDE: f64 = 4.469936;
        pub const MAX_LATITUDE: f64 = 51.5051449;
        pub const MAX_LONGITUDE: f64 = 6.408124099999999;
        pub const MIN_LATITUDE: f64 = 49.497013;
        pub const MIN_LONGITUDE: f64 = 2.5240999;
        pub const NORTHEAST_LATITUDE: f64 = 51.5051449;
        pub const NORTHEAST_LONGITUDE: f64 = 6.408124099999999;
        pub const SOUTHWEST_LATITUDE: f64 = 49.497013;
        pub const SOUTHWEST_LONGITUDE: f64 = 2.5240999;
    }
}
#[cfg(all(feature = "be", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 50.503887,
            longitude: 4.469936,
            max_latitude: 51.5051449,
            max_longitude: 6.408124099999999,
            min_latitude: 49.497013,
            min_longitude: 2.5240999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 51.5051449,
                    longitude: 6.408124099999999,
                },
                southwest: CountryGeoBound {
                    latitude: 49.497013,
                    longitude: 2.5240999,
                },
            },
        }
    }
}

#[cfg(all(feature = "be", feature = "subdivisions"))]
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
                    "BRU",
                    Subdivision{
                        name: "BRU",
                        country_alpha2: Alpha2::BE,
                        code: "BRU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.8503396), longitude: Some(4.3517103), max_latitude: Some(50.91370999999999), min_latitude: Some(50.7962401), max_longitude: Some(4.4369799), min_longitude: Some(4.3138)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Brusselse Hoofstedelike Gewes"), ("am", "ብሩክሴል"), ("ar", "إقليم بروكسل العاصمة"), ("be", "Брусельскі сталічны рэгіён"), ("bg", "Столичен регион Брюксел"), ("bn", "ব\u{9cd}র\u{9be}সেল\u{9cd}\u{200c}স"), ("bs", "Regija glavnog grada Bruxellesa"), ("ca", "Regió de Brussel·les-Capital"), ("ccp", "𑄝\u{11133}𑄢𑄥𑄬𑄣\u{11134}𑄥\u{11134}"), ("ceb", "Bruxelles-Capitale"), ("cs", "Bruselský region"), ("cy", "Rhanbarth Brwsel-Prifddinas"), ("da", "Region Bruxelles"), ("de", "Brüssel"), ("el", "Περιοχή των Βρυξελλών"), ("en", "Brussels"), ("es", "Región de Bruselas-Capital"), ("et", "Pealinna Brüsseli piirkond"), ("eu", "Brusela-Hiriburua eskualdea"), ("fa", "بروکسل"), ("fi", "Brysselin pääkaupunkialue"), ("fr", "Région de Bruxelles-Capitale"), ("ga", "An Bhruiséil"), ("gl", "Bruxelas-Capital"), ("gu", "બ\u{acd}રસ\u{ac7}લ\u{acd}સ-ક\u{ac7}પિટલ પ\u{acd}રદ\u{ac7}શ"), ("he", "בריסל"), ("hi", "ब\u{94d}र\u{941}स\u{947}ल\u{94d}स"), ("hr", "Regija glavnoga grada Bruxellesa"), ("hu", "Brüsszel fővárosi régió"), ("hy", "Բրյուսել"), ("id", "Daerah Ibu Kota Brussel"), ("is", "Brussel"), ("it", "Regione di Bruxelles-Capitale"), ("ja", "ブリュッセル首都圏地域"), ("ka", "ბრიუსელის რეგიონი"), ("kk", "Брюссель"), ("kn", "ಬ\u{ccd}ರಸ\u{cc6}ಲ\u{ccd}ಸ\u{ccd}"), ("ko", "브뤼셀"), ("lt", "Briuselio-sostinės regionas"), ("lv", "Briseles galvaspilsētas reģions"), ("mk", "Брисел"), ("ml", "ബ\u{d4d}രസൽസ\u{d4d}"), ("mn", "Брюссель"), ("mr", "ब\u{94d}रस\u{947}ल\u{94d}स"), ("ms", "Brussels"), ("my", "ဘရပ\u{103a}ဆ\u{1032}လ\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Brussel"), ("ne", "ब\u{94d}रस\u{947}ल\u{94d}स राजधानी क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Brussels Hoofdstedelijk Gewest"), ("no", "Brussel"), ("or", "ବ\u{b43}ସେଲ"), ("pa", "ਬਰ\u{a42}ਸਲ"), ("pl", "Region Stołeczny Brukseli"), ("ps", "بروکسل"), ("pt", "Região de Bruxelas-Capital"), ("ro", "Regiunea Capitalei Bruxelles"), ("ru", "Брюссельский столичный регион"), ("si", "බ\u{dca}\u{200d}රසල\u{dca}ස\u{dca}-කැප\u{dd2}ටල\u{dca} කල\u{dcf}පය"), ("sk", "Región Brusel-hlavné mesto"), ("sl", "Regija Bruselj-glavno mesto"), ("sq", "Brukseli"), ("sr", "Регион главног града Брисела"), ("sr_Latn", "Region glavnog grada Brisela"), ("sv", "Bryssel"), ("sw", "Brussels"), ("ta", "பிரஸ\u{bcd}செல\u{bcd}ஸ\u{bcd}"), ("te", "బ\u{c4d}రస\u{c46}ల\u{c4d}స\u{c4d}-ర\u{c3e}జధ\u{c3e}న\u{c3f} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "บร\u{e31}สเซลส\u{e4c}"), ("tr", "Brüksel Başkent Bölgesi"), ("uk", "Брюссельський столичний регіон"), ("ur", "برسلز"), ("uz", "Brussel Poytaxt Regioni"), ("vi", "Bruxelles"), ("yo", "Brussels"), ("yo_BJ", "Brussels"), ("yue", "布魯塞爾首都區"), ("yue_Hans", "布鲁塞尔首都区"), ("zh", "布魯塞爾首都大區市鎮")]),
                        unofficial_name_list: ["Brussel", "Brussels Hoofdstedelijk Gewest", "Bruxelles", "Brüssel", "Région de Bruxelles-Capitale"].to_vec(),
                    }
                ),
                (
                    "VAN",
                    Subdivision{
                        name: "VAN",
                        country_alpha2: Alpha2::BE,
                        code: "VAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.2194475), longitude: Some(4.4024643), max_latitude: Some(51.3774301), min_latitude: Some(51.14333999999999), max_longitude: Some(4.49784), min_longitude: Some(4.217600099999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Antwerpen"), ("ar", "أنتفيرب"), ("be", "правінцыя Антверпен"), ("bg", "Антверпен"), ("bn", "এন\u{9cd}টওয\u{9bc}\u{9be}র\u{9cd}প"), ("bs", "Antwerpen"), ("ca", "Província d’Anvers"), ("ccp", "𑄃𑄚\u{11133}𑄑\u{11134}𑄃\u{1112e}𑄠𑄬𑄢\u{11134}𑄛\u{11134}"), ("ceb", "Provincie Antwerpen"), ("cs", "Antverpy"), ("cy", "Antwerp"), ("da", "Antwerpen"), ("de", "Antwerpen"), ("el", "Αμβέρσα"), ("en", "Antwerp"), ("es", "Provincia de Amberes"), ("et", "Antwerpeni provints"), ("eu", "Anberesko probintzia"), ("fa", "استان آنتورپ"), ("fi", "Antwerpenin lääni"), ("fr", "Anvers"), ("gl", "Provincia de Antwerp"), ("gu", "એન\u{acd}ટવર\u{acd}પ"), ("hi", "ए\u{902}टवर\u{94d}प (प\u{94d}रा\u{902}त )"), ("hr", "Antwerpen"), ("hu", "Antwerpen"), ("hy", "Անտվերպեն"), ("id", "Antwerpen"), ("is", "Antwerpen-hérað"), ("it", "provincia d’Anversa"), ("ja", "アントウェルペン州"), ("ka", "ანტვერპენის პროვინცია"), ("kn", "ಆಂಟ\u{ccd}ವರ\u{ccd}ಪ\u{ccd}"), ("ko", "안트베르펜 주"), ("lt", "Antverpeno provincija"), ("lv", "Antverpene"), ("mk", "Антверпен"), ("mr", "अ\u{901}टवर\u{94d}प"), ("ms", "Wilayah Antwerpen"), ("nb", "Antwerpen"), ("nl", "Provincie Antwerpen"), ("no", "Antwerpen"), ("pl", "Prowincja Antwerpia"), ("pt", "Antuérpia"), ("ro", "Anvers"), ("ru", "Антверпен"), ("si", "අන\u{dca}ට\u{dca}වර\u{dca}ප\u{dca}"), ("sk", "Antverpy"), ("sr", "Провинција Антверпен"), ("sr_Latn", "Provincija Antverpen"), ("sv", "Antwerpen"), ("ta", "அன\u{bcd}ட\u{bcd}வெர\u{bcd}ப\u{bcd}"), ("te", "ఆంట\u{c4d}వర\u{c4d}ప\u{c4d}"), ("th", "มณฑลแอนต\u{e4c}เว\u{e34}ร\u{e4c}ป"), ("tr", "Anvers"), ("uk", "Антверпен"), ("ur", "اینٹورپ"), ("vi", "Antwerp"), ("zh", "安特衛普省")]),
                        unofficial_name_list: ["Antwerpen", "Anvers"].to_vec(),
                    }
                ),
                (
                    "VBR",
                    Subdivision{
                        name: "VBR",
                        country_alpha2: Alpha2::BE,
                        code: "VBR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.3663862), longitude: Some(5.615708800000001), max_latitude: Some(51.378742), min_latitude: Some(51.3563098), max_longitude: Some(5.625843), min_longitude: Some(5.6001346)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vlaams-Brabant"), ("ar", "برابانت فلاماند"), ("be", "Фламандскі Брабант"), ("bg", "Фламандски Брабант"), ("bn", "ফ\u{9cd}লেমিশ ব\u{9cd}র\u{9cd}য\u{9be}বেন\u{9cd}ট"), ("bs", "Flamanski Brabant"), ("ca", "Brabant Flamenc"), ("ccp", "𑄜\u{11133}𑄣𑄬𑄟\u{11128}𑄌\u{11134} 𑄝\u{11133}𑄢𑄝𑄚\u{11133}𑄑\u{11134}"), ("ceb", "Provincie Vlaams-Brabant"), ("cs", "Vlámský Brabant"), ("cy", "Brabant Fflandrysaidd"), ("da", "Vlaams-Brabant"), ("de", "Flämisch-Brabant"), ("el", "Φλαμανδική Μπραμπάντ"), ("en", "Flemish Brabant"), ("es", "Brabante Flamenco"), ("et", "Flaami Brabant"), ("eu", "Flandriako Brabante"), ("fa", "برابانت فلاندر"), ("fi", "Flanderin Brabant"), ("fr", "Brabant flamand"), ("gl", "Provincia do Brabante flamengo"), ("gu", "ફ\u{acd}લ\u{ac7}મિશ બ\u{acd}રાબ\u{ac7}\u{a82}ટ"), ("he", "ברבנט הפלמית"), ("hi", "फ\u{94d}ल\u{947}मिश ब\u{94d}रब\u{947}\u{902}ट"), ("hr", "Flamanski Brabant"), ("hu", "Flamand-Brabant"), ("hy", "Ֆլամանդական Բրաբանտ"), ("id", "Flemish Brabant"), ("is", "Flæmska Brabant"), ("it", "provincia del Brabante Fiammingo"), ("ka", "ფლამანდიის ბრაბანტი"), ("kn", "ಫ\u{ccd}ಲ\u{cc6}ಮ\u{cbf}ಶ\u{ccd} ಬ\u{ccd}ರಬಂಟ\u{ccd}"), ("ko", "플람스브라반트 주"), ("lt", "Flamandų Brabanto provincija"), ("lv", "Flāmu Brabante"), ("mk", "Фламански Брабант"), ("mr", "फ\u{94d}लाम\u{94d}स ब\u{94d}राबा\u{902}त"), ("ms", "Brabant Flanders"), ("nb", "Flamsk Brabant"), ("nl", "Vlaams-Brabant"), ("no", "Flamsk Brabant"), ("pl", "Prowincja Brabancja Flamandzka"), ("pt", "Brabante Flamengo"), ("ro", "Brabantul Flamand"), ("ru", "Фламандский Брабант"), ("si", "ෆ\u{dca}ලෙම\u{dd2}ෂ\u{dca} බ\u{dca}\u{200d}රබන\u{dca}ට\u{dca}"), ("sk", "Flámsky Brabant"), ("sr", "Фламански Брабант"), ("sr_Latn", "Flamanski Brabant"), ("sv", "Flamländska Brabant"), ("ta", "பிலெமிஷ\u{bcd} பிரப\u{bbe}ன\u{bcd}ட\u{bcd}"), ("te", "ఫ\u{c4d}ల\u{c46}మ\u{c3f}ష\u{c4d} బ\u{c4d}ర\u{c3e}బంట\u{c4d}"), ("th", "มณฑลเฟลม\u{e34}ชบราบ\u{e31}นต\u{e4c}"), ("tr", "Flaman Brabant"), ("uk", "Фламандський Брабант"), ("ur", "فلیمش برابنٹ"), ("vi", "Vlaams-Brabant"), ("zh", "弗拉芒-布拉班特省")]),
                        unofficial_name_list: ["Brabant-Flamand", "Brabant-Vlanderen", "Flämisch Brabant", "Vlaams-Brabant"].to_vec(),
                    }
                ),
                (
                    "VLG",
                    Subdivision{
                        name: "VLG",
                        country_alpha2: Alpha2::BE,
                        code: "VLG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vlaandere"), ("ar", "الإقليم الفلامندي"), ("az", "Flandriya"), ("be", "Фламандскі рэгіён"), ("bg", "Фламандски регион"), ("bs", "Flandrija"), ("ca", "regió Flamenca"), ("ccp", "𑄜\u{11133}𑄣𑄚\u{11134}𑄓𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Flanders"), ("cs", "Vlámský region"), ("cy", "Fflandrys"), ("da", "Flandern"), ("de", "Flandern"), ("el", "Φλαμανδική Περιοχή"), ("en", "Flanders"), ("es", "Región Flamenca"), ("et", "Flandria"), ("eu", "Flandria"), ("fa", "منطقه فلمیش"), ("fi", "Flanderi"), ("fr", "Région flamande"), ("ga", "Flóndras"), ("gl", "Flandres - Vlaanderen"), ("he", "פלנדריה"), ("hi", "फ\u{94d}ल\u{947}मिश क\u{94d}ष\u{947}त\u{94d}र"), ("hr", "Flandrija"), ("hu", "Flandria"), ("hy", "Ֆլանդրիա"), ("id", "Flandria"), ("is", "Flæmingjaland"), ("it", "Fiandre"), ("ja", "フランデレン地域"), ("ka", "ფლანდრია"), ("kk", "Фландрия"), ("ko", "플라망 지역"), ("lt", "Flandrija"), ("lv", "Flandrija"), ("mk", "Фламански регион"), ("mr", "फ\u{94d}ला\u{902}डर\u{94d}स"), ("ms", "Flanders"), ("nb", "Flandern"), ("ne", "फ\u{94d}ल\u{947}मिस क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Vlaams Gewest"), ("no", "Flandern"), ("pl", "Region Flamandzki"), ("pt", "Flandres"), ("ro", "Flandra"), ("ru", "Фламандский регион"), ("sk", "Flámsky región"), ("sl", "Flanders"), ("sr", "Фландрија"), ("sr_Latn", "Flandrija"), ("sv", "Flandern"), ("sw", "Flandria"), ("ta", "பில\u{bbe}ன\u{bcd}டர\u{bcd}ஸ\u{bcd}"), ("th", "เขตฟลามส\u{e4c}"), ("tr", "Flaman Bölgesi"), ("uk", "Фламандській регіон"), ("ur", "فلیمش علاقہ"), ("vi", "Vùng Flemish"), ("yue", "法蘭德斯"), ("yue_Hans", "法兰德斯"), ("zh", "弗拉芒大區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "VLI",
                    Subdivision{
                        name: "VLI",
                        country_alpha2: Alpha2::BE,
                        code: "VLI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.4427238), longitude: Some(6.0608726), max_latitude: Some(51.778577), min_latitude: Some(50.75038379999999), max_longitude: Some(6.226801399999999), min_longitude: Some(5.5660666)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Limburg, België"), ("ar", "ليمبورغ"), ("be", "Правінцыя Лімбург"), ("bg", "Лимбург"), ("bn", "লিমব\u{9be}র\u{9cd}গ"), ("ca", "Limburg"), ("ccp", "𑄣\u{11128}𑄟\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Provincie Limburg (lalawigan)"), ("cs", "Limburk"), ("cy", "Limburg"), ("da", "Limburg"), ("de", "Limburg"), ("el", "Λιμβουργία"), ("en", "Limburg"), ("es", "Limburgo"), ("et", "Limburgi provints"), ("eu", "Linburgo"), ("fa", "استان لمبورگ"), ("fi", "Limburgin lääni"), ("fr", "Limbourg"), ("gl", "Provincia de Limburg"), ("gu", "લિમ\u{acd}બર\u{acd}ગ"), ("he", "לימבורג (בלגיה)"), ("hi", "लिम\u{94d}बर\u{94d}ग (नीदरल\u{948}\u{902}ड\u{94d}स)"), ("hr", "Limburg"), ("hu", "Limburg"), ("hy", "Լիմբուրգ"), ("id", "Limburg"), ("is", "Limburg"), ("it", "Limburgo"), ("ja", "リンブルフ州"), ("jv", "Limburg"), ("ka", "ლიმბურგის პროვინცია"), ("kn", "ಲ\u{cbf}ಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "림뷔르흐 주"), ("lt", "Limburgo provincija"), ("lv", "Limburga"), ("mk", "Лимбург"), ("mr", "लिमबर\u{94d}ग"), ("ms", "Limburg"), ("nb", "Limburg"), ("nl", "Limburg"), ("no", "Limburg"), ("pl", "Prowincja Limburgia"), ("pt", "Limburgo"), ("ro", "Limburg"), ("ru", "Лимбург"), ("si", "ල\u{dd2}ම\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Limbursko"), ("sr", "Провинција Лимбург"), ("sr_Latn", "Provincija Limburg"), ("sv", "Limburg"), ("ta", "லிம\u{bcd}பெர\u{bcd}க\u{bcd}"), ("te", "ల\u{c3f}ంబర\u{c4d}గ\u{c4d}"), ("th", "มณฑลล\u{e34}มเบ\u{e34}ร\u{e4c}ก"), ("tr", "Limburg"), ("uk", "Лімбург"), ("ur", "لمبرگ"), ("vi", "Limburg"), ("zh", "林堡省")]),
                        unofficial_name_list: ["Limbourg"].to_vec(),
                    }
                ),
                (
                    "VOV",
                    Subdivision{
                        name: "VOV",
                        country_alpha2: Alpha2::BE,
                        code: "VOV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.0362101), longitude: Some(3.7373124), max_latitude: Some(51.35284), min_latitude: Some(50.72094999999999), max_longitude: Some(4.3301), min_longitude: Some(3.3312501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oos-Vlaandere"), ("ar", "فلاندر الشرقية"), ("az", "Şərqi Flandiriya"), ("be", "Усходняя Фландрыя"), ("bg", "Източна Фландрия"), ("bn", "ইস\u{9cd}ট ফ\u{9cd}ল\u{9be}ন\u{9cd}ড\u{9be}র\u{9cd}স"), ("ca", "Flandes Oriental"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄜\u{11133}𑄣𑄚\u{11134}𑄓𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Provincie Oost-Vlaanderen"), ("cs", "Východní Flandry"), ("cy", "Dwyrain Fflandrys"), ("da", "Østflandern"), ("de", "Ostflandern"), ("el", "Ανατολική Φλάνδρα"), ("en", "East Flanders"), ("es", "Provincia de Flandes Oriental"), ("et", "Ida-Flandria"), ("eu", "Ekialdeko Flandria"), ("fa", "استان فلاندری شرقی"), ("fi", "Itä-Flanderi"), ("fr", "Flandre-Orientale"), ("gl", "Provincia de Flandres oriental"), ("gu", "ઇસ\u{acd}ટ ફ\u{acd}લ\u{ac7}ન\u{acd}ડર\u{acd}સ"), ("he", "פלנדריה המזרחית"), ("hi", "प\u{942}र\u{94d}वी फ\u{94d}ल\u{948}\u{902}डर\u{94d}स"), ("hr", "Istočna Flandrija"), ("hu", "Kelet-Flandria"), ("hy", "Արևելյան Ֆլանդրիա"), ("id", "Flandria Timur"), ("is", "Austur-Flæmingjaland"), ("it", "provincia delle Fiandre Orientali"), ("ka", "აღმოსავლეთი ფლანდრია"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಫ\u{ccd}ಲಾಂಡರ\u{ccd}ಸ\u{ccd}"), ("ko", "오스트플란데런 주"), ("lt", "Rytų Flandrijos provincija"), ("lv", "Austrumflandrija"), ("mk", "Источна Фландрија"), ("mr", "प\u{942}र\u{94d}व फ\u{94d}ला\u{902}डर\u{94d}स"), ("ms", "Flanders Timur"), ("nb", "Øst-Flandern"), ("nl", "Oost-Vlaanderen"), ("no", "Øst-Flandern"), ("pl", "Prowincja Flandria Wschodnia"), ("pt", "Flandres Oriental"), ("ro", "Flandra de Est"), ("ru", "Восточная Фландрия"), ("si", "නැගෙනහ\u{dd2}ර ෆ\u{dca}ලන\u{dca}ඩර\u{dca}ස\u{dca}"), ("sk", "Východné Flámsko"), ("sr", "Источна Фландрија"), ("sr_Latn", "Istočna Flandrija"), ("sv", "Östflandern"), ("ta", "கிழக\u{bcd}கு பில\u{bbe}ன\u{bcd}டெர\u{bcd}ஸ\u{bcd}"), ("te", "ఈస\u{c4d}ట\u{c4d} ఫ\u{c4d}ల\u{c3e}ండర\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฟลานเดอร\u{e4c}ตะว\u{e31}นออก"), ("tr", "Doğu Flandre"), ("uk", "Східна Фландрія"), ("ur", "مشرقی فلانڈرز"), ("vi", "Oost-Vlaanderen"), ("zh", "东弗兰德省")]),
                        unofficial_name_list: ["Flandre-Orientale", "Oos-Vlanderen", "Oost-Vlaanderen", "Ost-Flandern"].to_vec(),
                    }
                ),
                (
                    "VWV",
                    Subdivision{
                        name: "VWV",
                        country_alpha2: Alpha2::BE,
                        code: "VWV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.0536024), longitude: Some(3.1457942), max_latitude: Some(51.3685479), min_latitude: Some(50.7081601), max_longitude: Some(3.5232999), min_longitude: Some(2.5449401)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Vlaandere"), ("ar", "فلاندرز الغربية"), ("az", "Qərbi Flandiriya"), ("be", "Заходняя Фландрыя"), ("bg", "Западна Фландрия"), ("bn", "ওয\u{9bc}েস\u{9cd}ট ফ\u{9cd}ল\u{9be}ন\u{9cd}ড\u{9be}রস"), ("ca", "Flandes Occidental"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄜\u{11133}𑄣𑄚\u{11134}𑄓𑄢\u{11134}𑄥\u{11134}"), ("ceb", "Provincie West-Vlaanderen"), ("cs", "Západní Flandry"), ("cy", "Gorllewin Fflandrys"), ("da", "Vestflandern"), ("de", "Westflandern"), ("el", "Δυτική Φλάνδρα"), ("en", "West Flanders"), ("es", "Provincia de Flandes Occidental"), ("et", "Lääne-Flandria provints"), ("eu", "Mendebaldeko Flandria"), ("fa", "استان فلاندری غربی"), ("fi", "Länsi-Flanderi"), ("fr", "Flandre-Occidentale"), ("gl", "Flandres Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ ફ\u{acd}લ\u{ac7}ન\u{acd}ડર\u{acd}સ"), ("he", "מערב פלנדריה"), ("hi", "व\u{947}स\u{94d}ट फ\u{94d}ल\u{948}\u{902}डर\u{94d}स"), ("hr", "Zapadna Flandrija"), ("hu", "Nyugat-Flandria"), ("hy", "Արևմտյան Ֆլանդրիա"), ("id", "Flandria Barat"), ("is", "Vestur-Flæmingjaland"), ("it", "provincia delle Fiandre Occidentali"), ("ka", "დასავლეთი ფლანდრია"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ಫ\u{ccd}ಲಾಂಡರ\u{ccd}ಸ\u{ccd}"), ("ko", "베스트플란데런 주"), ("lt", "Vakarų Flandrijos provincija"), ("lv", "Rietumflandrija"), ("mk", "Западна Фландрија"), ("mr", "पश\u{94d}चिम फ\u{94d}ला\u{902}डर\u{94d}स"), ("ms", "Flanders Barat"), ("nb", "Vest-Flandern"), ("nl", "West-Vlaanderen"), ("no", "Vest-Flandern"), ("pl", "Prowincja Flandria Zachodnia"), ("pt", "Flandres Ocidental"), ("ro", "Flandra de Vest"), ("ru", "Западная Фландрия"), ("si", "බටහ\u{dd2}ර ෆ\u{dca}ලන\u{dca}ඩර\u{dca}ස\u{dca}"), ("sk", "Západné Flámsko"), ("sl", "Zahodna Flamska"), ("sr", "Западна Фландрија"), ("sr_Latn", "Zapadna Flandrija"), ("sv", "Västflandern"), ("ta", "மேற\u{bcd}கு பில\u{bbe}ன\u{bcd}டெர\u{bcd}ஸ\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d} ఫ\u{c4d}ల\u{c3e}ండర\u{c4d}స\u{c4d}"), ("th", "มณฑลฟลานเดอร\u{e4c}ตะว\u{e31}นตก"), ("tr", "Batı Flandre"), ("uk", "Західна Фландрія"), ("ur", "مغربی فلانڈرز"), ("vi", "West-Vlaanderen"), ("zh", "西弗兰德省")]),
                        unofficial_name_list: ["Flandre-Occidentale", "Wes-Vlanderen", "West-Flandern", "West-Vlaanderen"].to_vec(),
                    }
                ),
                (
                    "WAL",
                    Subdivision{
                        name: "WAL",
                        country_alpha2: Alpha2::BE,
                        code: "WAL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wallonië"), ("ar", "والونيا"), ("be", "Валонія"), ("bg", "Валония"), ("bs", "Valonija"), ("ca", "Valònia"), ("ccp", "𑄃\u{1112e}𑄠𑄣\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Wallonia"), ("cs", "Valonsko"), ("cy", "Walonia"), ("da", "Vallonien"), ("de", "Wallonien"), ("el", "Βαλλωνία"), ("en", "Wallonia"), ("es", "Valonia"), ("et", "Valloonia"), ("eu", "Valonia"), ("fa", "والونیا"), ("fi", "Vallonia"), ("fr", "Wallonie"), ("ga", "An Vallóin"), ("gl", "Valonia"), ("he", "ולוניה"), ("hi", "वालोनिया"), ("hr", "Valonija"), ("hu", "Vallónia"), ("hy", "Վալոնիա"), ("id", "Walonia"), ("is", "Vallónía"), ("it", "Vallonia"), ("ja", "ワロン地域"), ("ka", "ვალონია"), ("ko", "왈롱"), ("lt", "Valonija"), ("lv", "Valonija"), ("mk", "Валонија"), ("mr", "वालोनी"), ("ms", "Walonia"), ("nb", "Vallonia"), ("ne", "वाल\u{941}न क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Wallonië"), ("no", "Vallonia"), ("pl", "Walonia"), ("pt", "Valônia"), ("ro", "Valonia"), ("ru", "Валлония"), ("sk", "Valónsko"), ("sl", "Valonija"), ("sr", "Валонија"), ("sr_Latn", "Valonija"), ("sv", "Vallonien"), ("sw", "Wallonia"), ("ta", "வ\u{bbe}ல\u{bcd}லொனிய\u{bbe}"), ("th", "เขตว\u{e31}ลล\u{e39}น"), ("tr", "Valonya"), ("uk", "Валлонія"), ("ur", "والونیا"), ("vi", "Wallonie"), ("yue", "華隆"), ("yue_Hans", "华隆"), ("zh", "瓦隆")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WBR",
                    Subdivision{
                        name: "WBR",
                        country_alpha2: Alpha2::BE,
                        code: "WBR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.633241), longitude: Some(4.524315), max_latitude: Some(50.80735), min_latitude: Some(50.52542), max_longitude: Some(5.02037), min_longitude: Some(4.0911501)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Waals-Brabant"), ("ar", "برابانت والون"), ("az", "Vallonskiy Brabant"), ("be", "Валонскі Брабант"), ("bg", "Валонски Брабант"), ("bn", "ওয\u{9bc}\u{9be}ল\u{9cd}ল\u{9c1}ন ব\u{9cd}র\u{9be}ব\u{9be}ন\u{9cd}ট"), ("ca", "Brabant Való"), ("ccp", "𑄃\u{1112e}𑄠𑄣\u{1112a}𑄚\u{11134} 𑄝\u{11133}𑄢𑄝𑄚\u{11133}𑄑\u{11134}"), ("ceb", "Province du Brabant Wallon"), ("cs", "Valonský Brabant"), ("cy", "Brabant Walonaidd"), ("da", "Brabant Wallon"), ("de", "Wallonisch-Brabant"), ("el", "Βαλλωνική Βραβάντη"), ("en", "Walloon Brabant"), ("es", "Provincia del Brabante Valón"), ("et", "Vallooni Brabant"), ("eu", "Valoniako Brabante"), ("fa", "استان اوئلون بربان"), ("fi", "Vallonian Brabant"), ("fr", "Brabant wallon"), ("gl", "Brabante Valón"), ("gu", "વાલ\u{ac2}ન બ\u{acd}રાબ\u{ac7}\u{a82}ત"), ("he", "ברבנט הוולונית"), ("hi", "वाल\u{942}न ब\u{94d}रब\u{948}\u{902}ट"), ("hr", "Valonski Brabant"), ("hu", "Vallon-Brabant"), ("hy", "Վալոնյան Բրաբանտ"), ("id", "Brabant Walonia"), ("is", "Vallónska Brabant"), ("it", "provincia del Brabante Vallone"), ("ja", "ブラバン・ワロン州"), ("ka", "ვალონიის ბრაბანტი"), ("kn", "ವಾಲ\u{cc2}ನ\u{ccd} ಬ\u{ccd}ರಬಂಟ\u{ccd}"), ("ko", "브라방왈롱 주"), ("lt", "Valonų Brabanto provincija"), ("lv", "Valoņu Brabante"), ("mr", "ब\u{94d}राबा\u{902}त वालो\u{902}"), ("ms", "Brabant Walonia"), ("nb", "Vallonsk Brabant"), ("nl", "Waals-Brabant"), ("no", "Vallonsk Brabant"), ("pl", "Prowincja Brabancja Walońska"), ("pt", "Brabante Valão"), ("ro", "Brabantul Valon"), ("ru", "Валлонский Брабант"), ("si", "වල\u{dd4}න\u{dca} බ\u{dca}\u{200d}රබන\u{dca}ට\u{dca}"), ("sk", "Valónsky Brabant"), ("sl", "Provinca Walloon Brabant"), ("sr", "Валонски Брабант"), ("sr_Latn", "Valonski Brabant"), ("sv", "Vallonska Brabant"), ("ta", "வலூன\u{bcd} பிரப\u{bbe}ன\u{bcd}ட\u{bcd}"), ("te", "వ\u{c3e}లూన\u{c4d} బ\u{c4d}ర\u{c3e}బంట\u{c4d}"), ("th", "มณฑลว\u{e31}ลล\u{e39}นบราบ\u{e31}นต\u{e4c}"), ("tr", "Valon Brabant"), ("uk", "Валлонський Брабант"), ("ur", "والون برابنٹ"), ("vi", "Walloon Brabant"), ("zh", "瓦隆-布拉班特省")]),
                        unofficial_name_list: ["Waals-Brabant", "Wallonisch Brabant", "Walloon Brabant"].to_vec(),
                    }
                ),
                (
                    "WHT",
                    Subdivision{
                        name: "WHT",
                        country_alpha2: Alpha2::BE,
                        code: "WHT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.5257076), longitude: Some(4.062101699999999), max_latitude: Some(50.81077), min_latitude: Some(49.94183), max_longitude: Some(4.6171299), min_longitude: Some(2.8421299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hainaut"), ("ar", "هينو"), ("az", "Eno"), ("be", "Правінцыя Эно"), ("bg", "Ено"), ("bn", "হ\u{9be}ইনট"), ("ca", "Hainaut"), ("ccp", "𑄦\u{1112d}𑄚𑄅\u{1112a}𑄖\u{11134}"), ("ceb", "Province du Hainaut"), ("cs", "Henegavsko"), ("cy", "Hainaut"), ("da", "Hainaut"), ("de", "Hennegau"), ("el", "Αινώ"), ("en", "Hainaut"), ("es", "Provincia de Henao"), ("et", "Hainaut"), ("eu", "Hainauteko probintzia"), ("fa", "استان انو"), ("fi", "Hainaut’n lääni"), ("fr", "Hainaut"), ("gl", "Provincia de Hainaut"), ("gu", "હ\u{ac8}નૌટ"), ("he", "אנו"), ("hi", "ह\u{948}नौत (प\u{94d}रा\u{902}त)"), ("hr", "Hainaut"), ("hu", "Hainaut"), ("hy", "Էնո"), ("id", "Hainaut"), ("is", "Hainaut"), ("it", "provincia dell’Hainaut"), ("ja", "エノー州"), ("ka", "ენოს პროვინცია"), ("kn", "ಹೈನಾಟ\u{ccd}"), ("ko", "에노 주"), ("lt", "Heno provincija"), ("lv", "Eno"), ("mk", "Ено"), ("mr", "एनो"), ("ms", "Hainaut"), ("nb", "Hainaut"), ("nl", "Henegouwen"), ("no", "Hainaut"), ("pl", "Prowincja Hainaut"), ("pt", "Hainaut"), ("ro", "Hainaut"), ("ru", "Эно"), ("si", "හය\u{dd2}නෞට\u{dca}"), ("sk", "Hennegavsko"), ("sr", "Провинција Ено"), ("sr_Latn", "Provincija Eno"), ("sv", "Hainaut"), ("ta", "ஹெயின\u{bbe}ட\u{bcd}"), ("te", "హ\u{c3e}య\u{c3f}న\u{c3e}ట\u{c4d}"), ("th", "มณฑลแอโน"), ("tr", "Hainaut"), ("uk", "Ено"), ("ur", "ہائنو"), ("vi", "Hainaut"), ("zh", "埃諾省")]),
                        unofficial_name_list: ["Henegouwen", "Hennegau"].to_vec(),
                    }
                ),
                (
                    "WLG",
                    Subdivision{
                        name: "WLG",
                        country_alpha2: Alpha2::BE,
                        code: "WLG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.6325574), longitude: Some(5.5796662), max_latitude: Some(50.68819), min_latitude: Some(50.56109010000001), max_longitude: Some(5.675110099999999), min_longitude: Some(5.5230701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Luik"), ("ar", "لياج"), ("az", "Lej"), ("be", "правінцыя Льеж"), ("bg", "Лиеж"), ("bn", "লিয\u{9bc}েজ"), ("ca", "Província de Lieja"), ("ccp", "𑄣\u{1112d}𑄎𑄬"), ("ceb", "Province de Liège"), ("cs", "Lutych"), ("cy", "Liège"), ("da", "Liège"), ("de", "Lüttich"), ("el", "Λιέγη"), ("en", "Liège"), ("es", "Provincia de Lieja"), ("et", "Liège’i provints"), ("eu", "Liejako probintzia"), ("fa", "استان لیژ"), ("fi", "Liègen pronssi"), ("fr", "Liège"), ("gl", "Provincia de Liexa"), ("gu", "લીજ"), ("hi", "लीज"), ("hr", "Liège"), ("hu", "Liège"), ("hy", "Լիեժ"), ("id", "Liège"), ("is", "Liege"), ("it", "provincia di Liegi"), ("ja", "リエージュ州"), ("ka", "ლიეჟის პროვინცია"), ("kn", "ಲೀಜ\u{ccd}"), ("ko", "리에주 주"), ("lt", "Lježo provincija"), ("lv", "Ljēža"), ("mk", "Лиеж"), ("mr", "लीज"), ("ms", "Liege"), ("nb", "Liège"), ("nl", "provincie Luik"), ("no", "Liège"), ("pl", "Prowincja Liège"), ("pt", "Liège"), ("ro", "Liège"), ("ru", "Льеж"), ("si", "ලය\u{dd2}ගේ"), ("sk", "Lutyšsko"), ("sl", "provinca Liège"), ("sr", "Провинција Лијеж"), ("sr_Latn", "Provincija Lijež"), ("sv", "Liège"), ("ta", "ல\u{bc0}ஜ\u{bcd}"), ("te", "ల\u{c3f}య\u{c47}జ\u{c4d}"), ("th", "มณฑลล\u{e35}แยฌ"), ("tr", "Liège"), ("uk", "Льєж"), ("ur", "لییج"), ("vi", "Liège"), ("zh", "列日省")]),
                        unofficial_name_list: ["Luik", "Lüttich"].to_vec(),
                    }
                ),
                (
                    "WLX",
                    Subdivision{
                        name: "WLX",
                        country_alpha2: Alpha2::BE,
                        code: "WLX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.0546886), longitude: Some(5.467698299999999), max_latitude: Some(50.4306101), min_latitude: Some(49.49701), max_longitude: Some(6.034400000000001), min_longitude: Some(4.9683901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Luxemburg"), ("ar", "لوكسمبورغ"), ("be", "правінцыя Люксембург"), ("bg", "Люксембург"), ("bn", "ল\u{9be}ক\u{9cd}সেমব\u{9be}র\u{9cd}গ"), ("ca", "Província de Luxemburg"), ("ccp", "𑄣\u{1112a}𑄇\u{11134}𑄥𑄬𑄟\u{11134}𑄝𑄢\u{11134}𑄉\u{11134}"), ("ceb", "Province du Luxembourg"), ("cs", "Lucemburk"), ("cy", "Luxembourg"), ("da", "Luxembourg"), ("de", "Luxemburg"), ("el", "Λουξεμβούργο"), ("en", "Luxembourg"), ("es", "Provincia de Luxemburgo"), ("et", "Luxembourg’i provints"), ("eu", "Luxenburgo"), ("fa", "کانتون لوکزامبورگ"), ("fi", "Luxembourgin lääni"), ("fr", "Luxembourg"), ("gl", "Provincia de Luxemburgo"), ("gu", "લક\u{acd}ઝમબર\u{acd}ગ"), ("he", "לוקסמבורג"), ("hi", "लक\u{94d}ज\u{93c}मबर\u{94d}ग"), ("hr", "Luxembourg"), ("hu", "Luxembourg"), ("hy", "Լյուքսեմբուրգ"), ("id", "Luksemburg"), ("is", "Lúxemborg"), ("it", "provincia del Lussemburgo"), ("ja", "リュクサンブール州"), ("ka", "ლუქსემბურგის პროვინცია (ბელგია)"), ("kn", "ಲಕ\u{ccd}ಸ\u{cc6}ಂಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "뤽상부르 주"), ("lt", "Liuksemburgo provincija"), ("lv", "Luksemburga"), ("mk", "Луксембург"), ("mr", "लक\u{94d}झ\u{947}\u{902}बर\u{94d}ग"), ("ms", "Luxembourg"), ("nb", "Luxembourg"), ("nl", "Provincie Luxemburg"), ("no", "Luxembourg"), ("pl", "Prowincja Luksemburg"), ("pt", "Luxemburgo"), ("ro", "Luxemburg"), ("ru", "Люксембург"), ("si", "ලක\u{dca}සම\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Luxembourg (provincia)"), ("sr", "Провинција Луксембург"), ("sr_Latn", "Provincija Luksemburg"), ("sv", "Luxemburg"), ("ta", "லக\u{bcd}செம\u{bcd}பெர\u{bcd}க\u{bcd}"), ("te", "లక\u{c4d}స\u{c46}మ\u{c4d}\u{200c}బర\u{c4d}గ\u{c4d}"), ("th", "มณฑลล\u{e31}กเซมเบ\u{e34}ร\u{e4c}ก"), ("tr", "Lüksemburg, Belçika"), ("uk", "Люксембург"), ("ur", "لکسمبرگ"), ("vi", "Luxembourg"), ("zh", "盧森堡省")]),
                        unofficial_name_list: ["Luxembourg", "Luxemburg"].to_vec(),
                    }
                ),
                (
                    "WNA",
                    Subdivision{
                        name: "WNA",
                        country_alpha2: Alpha2::BE,
                        code: "WNA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(50.4673883), longitude: Some(4.8719854), max_latitude: Some(50.5312201), min_latitude: Some(50.38738), max_longitude: Some(4.98398), min_longitude: Some(4.7229)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Namur"), ("ar", "نامور"), ("az", "Namyur"), ("be", "Правінцыя Намюр"), ("bg", "Намюр"), ("bn", "ন\u{9be}ম\u{9c1}র"), ("ca", "Província de Namur"), ("ccp", "𑄚𑄟\u{1112a}𑄢\u{11134}"), ("ceb", "Province de Namur"), ("cs", "Namur"), ("cy", "Namur"), ("da", "Namur"), ("de", "Namur"), ("el", "Ναμύρ"), ("en", "Namur"), ("es", "Provincia de Namur"), ("et", "Namuri provints"), ("eu", "Namurreko probintzia"), ("fa", "استان نمور"), ("fi", "Namurin lääni"), ("fr", "Namur"), ("gl", "Provincia de Namur"), ("gu", "નામ\u{ac1}ર"), ("hi", "नाम\u{941}र"), ("hr", "Namur"), ("hu", "Namur"), ("hy", "Նամյուր"), ("id", "Namur"), ("it", "provincia di Namur"), ("ja", "ナミュール州"), ("ka", "ნამიურის პროვინცია"), ("kn", "ನಾಮುರ\u{ccd}"), ("ko", "나무르 주"), ("lt", "Namiūro provincija"), ("lv", "Namīra"), ("mr", "नाम\u{941}र"), ("ms", "Namur"), ("nb", "Namur"), ("nl", "Provincie Namen"), ("no", "Namur"), ("pl", "Prowincja Namur"), ("pt", "Namur"), ("ro", "Namur"), ("ru", "Намюр"), ("si", "නම\u{dd4}ර\u{dca}"), ("sk", "Namursko"), ("sr", "Провинција Намир"), ("sr_Latn", "Provincija Namir"), ("sv", "Namur"), ("ta", "நமுர\u{bcd}"), ("te", "న\u{c3e}ముర\u{c4d}"), ("th", "มณฑลนาม\u{e39}ร\u{e4c}"), ("tr", "Namur"), ("uk", "Намюр"), ("ur", "نامور"), ("vi", "Namur"), ("zh", "那慕爾省")]),
                        unofficial_name_list: ["Namen"].to_vec(),
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
#[cfg(feature = "be")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BE,
        alpha3: Alpha3::BEL,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 32,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::BE),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::BEL),
        iso_long_name: "The Kingdom of Belgium",
        iso_short_name: "Belgium",
        official_language_list: ["de", "fr", "nl"].to_vec(),
        spoken_language_list: ["de", "fr", "nl"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Belgian"),
        number: "056",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "BE",
        unofficial_name_list: [
            "Belgium",
            "Belgien",
            "Belgique",
            "Bélgica",
            "ベルギー",
            "België",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Belgium"),
            ("af", "België"),
            ("ak", "Belgium"),
            ("am", "ቤሔጄሤ"),
            ("an", "Belchica"),
            ("ar", "بلجيكا"),
            ("as", "বেলজিয়\u{9be}ম"),
            ("ay", "Belgium"),
            ("az", "Belçika"),
            ("ba", "Belgium"),
            ("be", "Бельгія"),
            ("bg", "Белгия"),
            ("bi", "Belgium"),
            ("bn", "বেলজিয়\u{9be}ম"),
            ("bn_IN", "বেলজিয়\u{9be}ম"),
            ("br", "Belgia"),
            ("bs", "Belgija"),
            ("ca", "Bèlgica"),
            ("ce", "Бельги"),
            ("ch", "Belgium"),
            ("cs", "Belgie"),
            ("cv", "Бельги"),
            ("cy", "Gwlad Belg"),
            ("da", "Belgien"),
            ("de", "Belgien"),
            ("dv", "ބ\u{7ac}ލ\u{7b0}ޖ\u{7a8}އ\u{7a6}މ\u{7b0}"),
            ("dz", "བ\u{f7a}ལ་ཇ\u{f72}་ཡམ།"),
            ("ee", "Belgium"),
            ("el", "Βέλγιο"),
            ("en", "Belgium"),
            ("eo", "Belgio"),
            ("es", "Bélgica"),
            ("et", "Belgia"),
            ("eu", "Belgika"),
            ("fa", "بلژیک"),
            ("ff", "Beljik"),
            ("fi", "Belgia"),
            ("fo", "Belgia"),
            ("fr", "Belgique"),
            ("fy", "Belgje"),
            ("ga", "An Bheilg"),
            ("gl", "Bélxica"),
            ("gn", "Belgium"),
            ("gu", "બ\u{ac7}લ\u{acd}જીયમ"),
            ("gv", "Yn Velg"),
            ("ha", "Beljik"),
            ("he", "בלגיה"),
            ("hi", "ब\u{947}ल\u{94d}जियम"),
            ("hr", "Belgija"),
            ("ht", "Bèljik"),
            ("hu", "Belgium"),
            ("hy", "Բելգիա"),
            ("ia", "Belgica"),
            ("id", "Belgia"),
            ("io", "Belgia"),
            ("is", "Belgía"),
            ("it", "Belgio"),
            ("iu", "Belgium"),
            ("ja", "ベルギー"),
            ("ka", "ბელგია"),
            ("ki", "Belgium"),
            ("kk", "Бельгия"),
            ("kl", "Belgium"),
            ("km", "បែល\u{200b}ហ\u{17d2}ស\u{17ca}\u{17b7}ក"),
            ("kn", "ಬ\u{cc6}ಲ\u{ccd}ಜ\u{cbf}ಯಂ"),
            ("ko", "벨기에"),
            ("ku", "Belçîka"),
            ("kv", "Бельгия"),
            ("kw", "Pow Belg"),
            ("ky", "Бельгия"),
            ("lo", "ປະເທດແບນຊ\u{eb4}ກ"),
            ("lt", "Belgija"),
            ("lv", "Beļģija"),
            ("mi", "Pehiamu"),
            ("mk", "Белгија"),
            ("ml", "ബെല\u{d4d}\u{200d}ജിയം"),
            ("mn", "Белги"),
            ("mr", "ब\u{947}ल\u{94d}जियम"),
            ("ms", "Belgium"),
            ("mt", "Belġju"),
            (
                "my",
                "ဘယ\u{103a}လ\u{103a}ဂျ\u{102e}ယမ\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Berdjiyum"),
            ("nb", "Belgia"),
            ("ne", "ब\u{947}ल\u{94d}जियम"),
            ("nl", "België"),
            ("nn", "Belgia"),
            ("nv", "Bélgii Bikéyah"),
            ("oc", "Belgica"),
            ("or", "ବେଲଜ\u{b3f}ୟମ"),
            ("pa", "ਬ\u{a48}ਲਜੀਅਮ"),
            ("pi", "ब\u{947}ल\u{94d}जियम"),
            ("pl", "Belgia"),
            ("ps", "بلجیم"),
            ("pt", "Bélgica"),
            ("pt_BR", "Bélgica"),
            ("ro", "Belgia"),
            ("ru", "Бельгия"),
            ("rw", "Ububiligi"),
            ("sc", "Bèlgiu"),
            ("sd", "بيلجيم"),
            ("si", "බෙල\u{dca}ජ\u{dd2}යම"),
            ("sk", "Belgicko"),
            ("sl", "Belgija"),
            ("so", "Beljiyam"),
            ("sq", "Belgjikë"),
            ("sr", "Белгија"),
            ("sv", "Belgien"),
            ("sw", "Ubelgiji"),
            ("ta", "பெல\u{bcd}ஜியம\u{bcd}"),
            ("te", "బ\u{c46}ల\u{c4d}జ\u{c3f}యమ\u{c4d}"),
            ("tg", "Белгия"),
            ("th", "เบลเย\u{e35}ยม"),
            ("ti", "ቤልጄም"),
            ("tk", "Belgiýa"),
            ("tl", "Belhika"),
            ("tr", "Belçika"),
            ("tt", "Белgиа"),
            ("ug", "بېلگىيە"),
            ("uk", "Бельгія"),
            ("ur", "بلجئیم"),
            ("uz", "Belgiya"),
            ("ve", "Belgium"),
            ("vi", "Bỉ"),
            ("wa", "Beldjike"),
            ("wo", "Beljik"),
            ("xh", "Belgium"),
            ("yo", "Bẹ\u{301}ljíọ\u{300}m"),
            ("zh_CN", "比利时"),
            ("zh_HK", "比利時"),
            ("zh_TW", "比利時"),
            ("zu", "Isi-Bhelijiyamu"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
