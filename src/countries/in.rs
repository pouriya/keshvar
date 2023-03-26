// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of India

#[cfg(all(feature = "in", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::IN;
    pub const ALPHA3: Alpha3 = Alpha3::IND;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 91;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::INR;
    pub const GEC: Option<GEC> = Some(GEC::IN);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::IND);
    pub const ISO_SHORT_NAME: &str = "India";
    pub const ISO_LONG_NAME: &str = "The Republic of India";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "hi"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "hi"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Indian");
    pub const NUMBER: &str = "356";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAsia);
    pub const UN_LOCODE: &str = "IN";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["India", "Indien", "Inde", "インド"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "India"),
        ("af", "Indië"),
        ("ak", "India"),
        ("am", "ህን፥"),
        ("an", "India"),
        ("ar", "الهند"),
        ("as", "ভ\u{9be}ৰত"),
        ("ay", "India"),
        ("az", "Hindistan"),
        ("ba", "India"),
        ("be", "Індыя"),
        ("bg", "Индия"),
        ("bi", "India"),
        ("bn", "ভ\u{9be}রত"),
        ("bn_IN", "ভ\u{9be}রত"),
        ("br", "India"),
        ("bs", "Indija"),
        ("ca", "Índia"),
        ("ce", "Инди"),
        ("ch", "India"),
        ("cs", "Indie"),
        ("cv", "Инди"),
        ("cy", "India"),
        ("da", "Indien"),
        ("de", "Indien"),
        ("dv", "އ\u{7a8}ނ\u{7b0}ޑ\u{7a8}ޔ\u{7a7}"),
        ("dz", "ར\u{f92}\u{fb1}་གར།"),
        ("ee", "India"),
        ("el", "Ινδία"),
        ("en", "India"),
        ("eo", "Barato"),
        ("es", "India"),
        ("et", "India"),
        ("eu", "India"),
        ("fa", "هندوستان"),
        ("ff", "India"),
        ("fi", "Intia"),
        ("fo", "India"),
        ("fr", "Inde"),
        ("fy", "Yndia"),
        ("ga", "An India"),
        ("gl", "A India"),
        ("gn", "India"),
        ("gu", "ભારત"),
        ("gv", "Yn Injey"),
        ("ha", "Indiya"),
        ("he", "הודו"),
        ("hi", "भारत"),
        ("hr", "Indija"),
        ("ht", "End"),
        ("hu", "India"),
        ("hy", "Հնդկաստան"),
        ("ia", "India"),
        ("id", "India"),
        ("io", "India"),
        ("is", "Indland"),
        ("it", "India"),
        ("iu", "ᐃᓐᑎᐊ"),
        ("ja", "インド"),
        ("ka", "ინდოეთი"),
        ("ki", "India"),
        ("kk", "Үндістан"),
        ("kl", "India"),
        ("km", "ឥណ\u{17d2}ឌា"),
        ("kn", "ಭಾರತ"),
        ("ko", "인도"),
        ("ku", "Hindistan"),
        ("kv", "Индия"),
        ("kw", "Eynda"),
        ("ky", "Индия"),
        ("lo", "India"),
        ("lt", "Indija"),
        ("lv", "Indija"),
        ("mi", "Īnia"),
        ("mk", "Индија"),
        ("ml", "ഇന\u{d4d}ത\u{d4d}യ"),
        ("mn", "Энэтхэг"),
        ("mr", "भारत"),
        ("ms", "Hindia"),
        ("mt", "Indja"),
        (
            "my",
            "အ\u{102d}န\u{1039}ဒ\u{102d}ယန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Indjiya"),
        ("nb", "India"),
        ("ne", "भारत"),
        ("nl", "India"),
        ("nn", "India"),
        (
            "nv",
            "Tó Wónaanídę\u{301}ę\u{301}ʼ Bitsįʼ Yishtłizhii Bikéyah",
        ),
        ("oc", "Índia"),
        ("or", "ଭ\u{b3e}ରତ"),
        ("pa", "ਭਾਰਤ"),
        ("pi", "भारत"),
        ("pl", "Indie"),
        ("ps", "هند"),
        ("pt", "Índia"),
        ("pt_BR", "Índia"),
        ("ro", "India"),
        ("ru", "Индия"),
        ("rw", "Ubuhinde"),
        ("sc", "Ìndia"),
        ("sd", "ڀارت"),
        ("si", "ඉන\u{dca}ද\u{dd2}ය\u{dcf}ව"),
        ("sk", "India"),
        ("sl", "Indija"),
        ("so", "Hindiya"),
        ("sq", "Indi"),
        ("sr", "Индија"),
        ("sv", "Indien"),
        ("sw", "India"),
        ("ta", "இந\u{bcd}திய\u{bbe}"),
        ("te", "భ\u{c3e}రతద\u{c47}శము"),
        ("tg", "Ҳиндустон"),
        ("th", "อ\u{e34}นเด\u{e35}ย"),
        ("ti", "ህንዲ"),
        ("tk", "Hindistan"),
        ("tl", "Indiya"),
        ("tr", "Hindistan"),
        ("tt", "Һиндстан"),
        ("ug", "ھىندىستان"),
        ("uk", "Індія"),
        ("ur", "بھارت"),
        ("uz", "Hindiston"),
        ("ve", "India"),
        ("vi", "Ấn-độ"),
        ("wa", "Inde"),
        ("wo", "Eend"),
        ("xh", "India"),
        ("yo", "Índíà"),
        ("zh_CN", "印度"),
        ("zh_HK", "印度"),
        ("zh_TW", "印度"),
        ("zu", "India"),
    ];
    #[cfg(all(feature = "in", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 20.593684;
        pub const LONGITUDE: f64 = 78.96288;
        pub const MAX_LATITUDE: f64 = 35.513327;
        pub const MAX_LONGITUDE: f64 = 97.39535869999999;
        pub const MIN_LATITUDE: f64 = 6.4626999;
        pub const MIN_LONGITUDE: f64 = 68.1097;
        pub const NORTHEAST_LATITUDE: f64 = 35.513327;
        pub const NORTHEAST_LONGITUDE: f64 = 97.39535869999999;
        pub const SOUTHWEST_LATITUDE: f64 = 6.4626999;
        pub const SOUTHWEST_LONGITUDE: f64 = 68.1097;
    }
}
#[cfg(all(feature = "in", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 20.593684,
            longitude: 78.96288,
            max_latitude: 35.513327,
            max_longitude: 97.39535869999999,
            min_latitude: 6.4626999,
            min_longitude: 68.1097,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 35.513327,
                    longitude: 97.39535869999999,
                },
                southwest: CountryGeoBound {
                    latitude: 6.4626999,
                    longitude: 68.1097,
                },
            },
        }
    }
}

#[cfg(all(feature = "in", feature = "subdivisions"))]
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
                    "AN",
                    Subdivision{
                        name: "Andaman and Nicobar Islands",
                        country_alpha2: Alpha2::IN,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.7400867), longitude: Some(92.6586401), max_latitude: Some(13.6746932), min_latitude: Some(6.7535159), max_longitude: Some(94.3014984), min_longitude: Some(92.2080496)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Andaman- en Nicobar-eilande"), ("ar", "جزر أندمان ونيكوبار"), ("as", "আন\u{9cd}দ\u{9be}ম\u{9be}ন আৰ\u{9c1} নিকোবৰ দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"), ("be", "Андаманскія і Нікабарскія астравы"), ("bg", "Андамански и Никобарски острови"), ("bn", "আন\u{9cd}দ\u{9be}ম\u{9be}ন ও নিকোবর দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"), ("bs", "Andamani i Nikobari"), ("ca", "Illes Andaman i Nicobar"), ("ccp", "𑄃𑄚\u{11134}𑄓𑄟𑄚\u{11134} 𑄃\u{11133}𑄃 𑄚\u{11128}𑄇\u{1112e}𑄝\u{11127}𑄢\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Union Territory of Andaman and Nicobar Islands"), ("cs", "Andamany a Nikobary"), ("da", "Andamanerne og Nicobarerne"), ("de", "Andamanen und Nikobaren"), ("el", "Νησιά Άνταμαν και Νίκομπαρ"), ("en", "Andaman and Nicobar Islands"), ("es", "Islas Andamán y Nicobar"), ("et", "Andamanid ja Nicobarid"), ("eu", "Andaman eta Nicobar uharteak"), ("fa", "جزایر آندامان و نیکوبار"), ("fi", "Andamaanit ja Nikobaarit"), ("fr", "Îles Andaman-et-Nicobar"), ("gl", "Andaman e Nicobar"), ("gu", "અ\u{a82}દામાન અન\u{ac7} નિકોબાર દ\u{acd}વીપસમ\u{ac2}હ"), ("he", "איי אנדמן וניקובר"), ("hi", "अण\u{94d}डमान और निकोबार द\u{94d}वीपसम\u{942}ह"), ("hr", "Andamanski i Nikobarski otoci"), ("hu", "Andamán- és Nikobár-szigetek"), ("id", "Kepulauan Andaman dan Nikobar"), ("is", "Andaman- og Níkóbareyjar"), ("it", "Andamane e Nicobare"), ("ja", "アンダマン・ニコバル諸島"), ("jv", "Kapuloan Andaman lan Nikobar"), ("ka", "ანდამანის და ნიკობარის კუნძულები"), ("kn", "ಅಂಡಮಾನ\u{ccd} ಮತ\u{ccd}ತು ನ\u{cbf}ಕೊಬಾರ\u{ccd} ದ\u{ccd}ವೀಪಗಳು"), ("ko", "안다만 니코바르 제도"), ("ky", "Андаман жана Никобар аралдары"), ("lt", "Andamanų ir Nikobarų salos"), ("lv", "Andamanu un Nikobaru Salas"), ("mk", "Андамани и Никобари"), ("ml", "ആന\u{d4d}തമ\u{d3e}ൻ നിക\u{d4d}കോബ\u{d3e}ർ ദ\u{d4d}വീപ\u{d41}കൾ"), ("mr", "अ\u{902}दमान आणि निकोबार"), ("ms", "Kepulauan Andaman dan Nicobar"), ("my", "ကပ\u{1039}ပလ\u{102e}ကျ\u{103d}န\u{103a}း"), ("nb", "Andamanene og Nikobarene"), ("ne", "अण\u{94d}डमान र निकोबार द\u{94d}वीप सम\u{942}ह"), ("nl", "Andamanerna och Nikobarerna"), ("no", "Andamanene og Nikobarene"), ("or", "ଆଣ\u{b4d}ଡମ\u{b3e}ନ ଓ ନ\u{b3f}କୋବର ଦ\u{b4d}ଵୀପପ\u{b41}ଞ\u{b4d}ଜ"), ("pa", "ਅ\u{a70}ਡ\u{a47}ਮਾਨ ਅਤ\u{a47} ਨਿਕ\u{a4b}ਬਾਰ ਦੀਪ ਸਮ\u{a42}ਹ"), ("pl", "Andamany i Nikobary"), ("pt", "Andamão e Nicobar"), ("ro", "Insulele Andaman și Nicobar"), ("ru", "Андаманские и Никобарские острова"), ("si", "අන\u{dca}දමන\u{dca} සහ න\u{dd2}කොබ\u{dcf}ර\u{dca} ද\u{dd6}පත\u{dca}"), ("sk", "Andamany a Nikobary"), ("sl", "Andamanski in Nikobarski otoki"), ("sr", "Андамани и Никобари"), ("sr_Latn", "Andamani i Nikobari"), ("sv", "Andamanerna och Nikobarerna"), ("sw", "Visiwa vya Andaman na Nicobar"), ("ta", "அந\u{bcd}தம\u{bbe}ன\u{bcd} நிக\u{bcd}கோப\u{bbe}ர\u{bcd} த\u{bc0}வுகள\u{bcd}"), ("te", "అండమ\u{c3e}న\u{c4d} న\u{c3f}క\u{c4b}బ\u{c3e}ర\u{c4d} ద\u{c40}వులు"), ("th", "หม\u{e39}\u{e48}เกาะอ\u{e31}นดาม\u{e31}นและน\u{e34}โคบาร\u{e4c}"), ("tr", "Andaman ve Nikobar adaları"), ("uk", "Андаманські і Нікобарські острови"), ("ur", "جزائر انڈمان و نکوبار"), ("uz", "Andaman va Nikobar orollari"), ("vi", "Quần đảo Andaman và Nicobar"), ("yo", "Àwọn Erékùṣù Andaman àti Nicobar"), ("yo_BJ", "Àwɔn Erékùshù Andaman àti Nicobar"), ("yue", "安達曼-尼科巴群島"), ("yue_Hans", "安达曼-尼科巴群岛"), ("zh", "安达曼-尼科巴群岛")]),
                        unofficial_name_list: ["Andaman and Nicobar Islands"].to_vec(),
                    }
                ),
                (
                    "AP",
                    Subdivision{
                        name: "Andhra Pradesh",
                        country_alpha2: Alpha2::IN,
                        code: "AP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.9128998), longitude: Some(79.7399875), max_latitude: Some(19.1565479), min_latitude: Some(12.596836), max_longitude: Some(84.7919452), min_longitude: Some(76.749786)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Andhra Pradesh"), ("am", "አንድረ ፕረደሽ"), ("ar", "أندرا برديش"), ("as", "অন\u{9cd}ধ\u{9cd}ৰপ\u{9cd}ৰদেশ"), ("az", "Andhra Pradeş"), ("be", "Андхра-Прадэш"), ("bg", "Андхра Прадеш"), ("bn", "অন\u{9cd}ধ\u{9cd}রপ\u{9cd}রদেশ"), ("bs", "Andhra Pradesh"), ("ca", "Andhra Pradesh"), ("ccp", "𑄃𑄚\u{11134}𑄙\u{11133}𑄢\u{11127} 𑄛\u{11133}𑄢\u{11127}𑄘𑄬𑄌\u{11134}"), ("ceb", "State of Andhra Pradesh"), ("cs", "Ándhrapradéš"), ("cy", "Andhra Pradesh"), ("da", "Andhra Pradesh"), ("de", "Andhra Pradesh"), ("el", "Άντρα Πραντές"), ("en", "Andhra Pradesh"), ("es", "Andhra Pradesh"), ("et", "Andhra Pradesh"), ("eu", "Andhra Pradesh"), ("fa", "آندرا پرادش"), ("fi", "Andhra Pradesh"), ("fr", "Andhra Pradesh"), ("ga", "Andhra Pradesh"), ("gl", "Andra Pradex"), ("gu", "આ\u{a82}ધ\u{acd}ર પ\u{acd}રદ\u{ac7}શ"), ("he", "אנדרה פרדש"), ("hi", "आन\u{94d}ध\u{94d}र प\u{94d}रद\u{947}श"), ("hr", "Andhra Pradesh"), ("hu", "Ándhra Prades"), ("hy", "Անդհրա-Պրադեշ"), ("id", "Andhra Pradesh"), ("is", "Andhra Pradesh"), ("it", "Andhra Pradesh"), ("ja", "アーンドラ・プラデーシュ州"), ("ka", "ანდჰრა-პრადეში"), ("kk", "Андхра-Прадеш"), ("kn", "ಆಂಧ\u{ccd}ರ ಪ\u{ccd}ರದೇಶ"), ("ko", "안드라프라데시 주"), ("lt", "Andhra Pradešas"), ("lv", "Āndhra Pradēša"), ("mk", "Андра Прадеш"), ("ml", "ആന\u{d4d}ധ\u{d4d}ര\u{d3e}പ\u{d4d}രദേശ\u{d4d}\u{200c}"), ("mn", "Андра Прадеш"), ("mr", "आ\u{902}ध\u{94d}र प\u{94d}रद\u{947}श"), ("ms", "Andhra Pradesh"), ("my", "အန\u{103a}ဒရာပရာဒစ\u{103a} ပြည\u{103a}နယ\u{103a}"), ("nb", "Andhra Pradesh"), ("ne", "आन\u{94d}ध\u{94d}र प\u{94d}रद\u{947}श"), ("nl", "Andhra Pradesh"), ("no", "Andhra Pradesh"), ("or", "ଆନ\u{b4d}ଧ\u{b4d}ର ପ\u{b4d}ରଦେଶ"), ("pa", "ਆ\u{a02}ਧਰਾ ਪ\u{a4d}ਰਦ\u{a47}ਸ\u{a3c}"), ("pl", "Andhra Pradesh"), ("ps", "آندرا پرديش"), ("pt", "Andhra Pradesh"), ("ro", "Andhra Pradesh"), ("ru", "Андхра-Прадеш"), ("si", "ආන\u{dca}ද\u{dca}\u{200d}ර ප\u{dca}\u{200d}රදේශ\u{dca}"), ("sk", "Ándhrapradéš"), ("sq", "Andra Pradesh"), ("sr", "Андра Прадеш"), ("sr_Latn", "Andra Pradeš"), ("sv", "Andhra Pradesh"), ("sw", "Andhra Pradesh"), ("ta", "ஆந\u{bcd}திரப\u{bcd} பிரதேசம\u{bcd}"), ("te", "ఆంధ\u{c4d}ర ప\u{c4d}రద\u{c47}శ\u{c4d}"), ("th", "ร\u{e31}ฐอานธรประเทศ"), ("tr", "Andhra Pradeş"), ("uk", "Андхра-Прадеш"), ("ur", "آندھرا پردیش"), ("uz", "Andhra Pradesh"), ("vi", "Andhra Pradesh"), ("yo", "Andhra Pradesh"), ("yo_BJ", "Andhra Pradesh"), ("yue", "安德拉邦"), ("yue_Hans", "安德拉邦"), ("zh", "安得拉邦")]),
                        unofficial_name_list: ["Andhra Pradesh"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "Arunachal Pradesh",
                        country_alpha2: Alpha2::IN,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.2179994), longitude: Some(94.7277528), max_latitude: Some(29.453453), min_latitude: Some(26.64258), max_longitude: Some(97.403297), min_longitude: Some(91.558064)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "አረናቸል ፕረዴሽ"), ("ar", "أروناجل برديش"), ("as", "অৰ\u{9c1}ণ\u{9be}চল প\u{9cd}ৰদেশ"), ("be", "Аруначал-Прадэш"), ("bg", "Аруначал Прадеш"), ("bn", "অর\u{9c1}ণ\u{9be}চল প\u{9cd}রদেশ"), ("ca", "Arunachal Pradesh"), ("ccp", "𑄃𑄢\u{1112a}𑄚𑄚\u{11134}𑄌\u{11127}𑄣\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄘𑄬𑄌\u{11134}"), ("ceb", "State of Arunāchal Pradesh"), ("cs", "Arunáčalpradéš"), ("cy", "Arunachal Pradesh"), ("da", "Arunachal Pradesh"), ("de", "Arunachal Pradesh"), ("el", "Αρουνάτσαλ Πραντές"), ("en", "Arunachal Pradesh"), ("es", "Arunachal Pradesh"), ("et", "Arunāchal Pradesh"), ("eu", "Arunachal Pradesh"), ("fa", "آروناچال پرادش"), ("fi", "Arunachal Pradesh"), ("fr", "Arunachal Pradesh"), ("ga", "Arunachal Pradesh"), ("gl", "Arunachal Pradesh"), ("gu", "અર\u{ac1}ણાચલ પ\u{acd}રદ\u{ac7}શ"), ("he", "ארונאצ׳ל פרדש"), ("hi", "अर\u{941}णाचल प\u{94d}रद\u{947}श"), ("hr", "Arunachal Pradesh"), ("hu", "Arunácsal Prades"), ("hy", "Արունաչալ Պրադեշ"), ("id", "Arunachal Pradesh"), ("is", "Arunachal Pradesh"), ("it", "Arunachal Pradesh"), ("ja", "アルナーチャル・プラデーシュ州"), ("ka", "არუნაჩალ-პრადეში"), ("kn", "ಅರುಣಾಚಲ ಪ\u{ccd}ರದೇಶ"), ("ko", "아루나찰프라데시 주"), ("lt", "Arunačal Pradešas"), ("lv", "Arunāčala Pradēša"), ("mk", "Аруначал Прадеш"), ("ml", "അര\u{d41}ണ\u{d3e}ചൽ പ\u{d4d}രദേശ\u{d4d}"), ("mn", "Аруначалпрадеш"), ("mr", "अर\u{941}णाचल प\u{94d}रद\u{947}श"), ("ms", "Arunachal Pradesh"), ("nb", "Arunachal Pradesh"), ("ne", "अर\u{941}णाचल प\u{94d}रद\u{947}श"), ("nl", "Arunachal Pradesh"), ("no", "Arunachal Pradesh"), ("or", "ଅର\u{b41}ଣ\u{b3e}ଚଳ ପ\u{b4d}ରଦେଶ"), ("pa", "ਅਰ\u{a41}ਨਾਚਲ ਪ\u{a4d}ਰਦ\u{a47}ਸ\u{a3c}"), ("pl", "Arunachal Pradesh"), ("ps", "اروناچل پرديش"), ("pt", "Arunachal Pradesh"), ("ro", "Arunachal Pradesh"), ("ru", "Аруначал-Прадеш"), ("si", "අර\u{dd4}න\u{dcf}චල\u{dca} ප\u{dca}\u{200d}රදේශ\u{dca}"), ("sk", "Arunáčalpradéš"), ("sq", "Arunaçal Pradesh"), ("sr", "Аруначал Прадеш"), ("sr_Latn", "Arunačal Pradeš"), ("sv", "Arunachal Pradesh"), ("sw", "Arunachal Pradesh"), ("ta", "அருண\u{bbe}சலப\u{bcd} பிரதேசம\u{bcd}"), ("te", "అరుణ\u{c3e}చల\u{c4d} ప\u{c4d}రద\u{c47}శ\u{c4d}"), ("th", "ร\u{e31}ฐอร\u{e38}ณาจ\u{e31}ลประเทศ"), ("tk", "Arunachal Pradesh"), ("tr", "Arunaçhal Pradesh"), ("uk", "Аруначал-Прадеш"), ("ur", "اروناچل پردیش"), ("uz", "Arunachal-Pradesh"), ("vi", "Arunachal Pradesh"), ("yo", "Arunachal Pradesh"), ("yo_BJ", "Arunachal Pradesh"), ("yue", "阿魯納恰爾"), ("yue_Hans", "阿鲁纳恰尔"), ("zh", "阿鲁纳恰尔邦")]),
                        unofficial_name_list: ["Arunachal Pradesh"].to_vec(),
                    }
                ),
                (
                    "AS",
                    Subdivision{
                        name: "Assam",
                        country_alpha2: Alpha2::IN,
                        code: "AS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.2006043), longitude: Some(92.9375739), max_latitude: Some(27.968216), min_latitude: Some(24.1384989), max_longitude: Some(96.0131609), min_longitude: Some(89.68563789999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Assam"), ("am", "አሳም"), ("ar", "أسام"), ("as", "অসম"), ("az", "Assam"), ("be", "Асам"), ("bg", "Асам"), ("bn", "আস\u{9be}ম"), ("ca", "Assam"), ("ccp", "𑄃𑄥𑄟\u{11134}"), ("ceb", "Assam"), ("cs", "Ásám"), ("cy", "Assam"), ("da", "Assam"), ("de", "Assam"), ("el", "Ασσάμ"), ("en", "Assam"), ("es", "Assam"), ("et", "Assam"), ("eu", "Assam"), ("fa", "آسام"), ("fi", "Assam"), ("fr", "Assam"), ("ga", "Assam"), ("gl", "Assam"), ("gu", "આસામ"), ("he", "אסאם"), ("hi", "असम"), ("hr", "Assam"), ("hu", "Asszám"), ("hy", "Ասսամ"), ("id", "Assam"), ("is", "Assam"), ("it", "Assam"), ("ja", "アッサム州"), ("jv", "Assam"), ("ka", "ასამი"), ("kk", "Ассам"), ("kn", "ಅಸ\u{ccd}ಸಾಂ"), ("ko", "아삼 주"), ("lt", "Asamas"), ("lv", "Asama"), ("mk", "Асам"), ("ml", "ആസ\u{d3e}ം"), ("mn", "Ассам"), ("mr", "आसाम"), ("ms", "Assam"), ("my", "အာသ\u{1036}ပြည\u{103a}နယ\u{103a}"), ("nb", "Assam"), ("ne", "आसाम"), ("nl", "Assam"), ("no", "Assam"), ("or", "ଆସ\u{b3e}ମ"), ("pa", "ਅਸਾਮ"), ("pl", "Asam"), ("ps", "آسام"), ("pt", "Assam"), ("ro", "Assam"), ("ru", "Ассам"), ("sd", "آسام"), ("si", "ඇසෑම\u{dca}"), ("sk", "Ásam"), ("sq", "Assam"), ("sr", "Асам"), ("sr_Latn", "Asam"), ("sv", "Assam"), ("sw", "Assam"), ("ta", "அச\u{bbe}ம\u{bcd}"), ("te", "అస\u{c4b}ం"), ("th", "ร\u{e31}ฐอ\u{e31}สส\u{e31}ม"), ("tr", "Assam"), ("uk", "Ассам"), ("ur", "آسام"), ("uz", "Accom"), ("vi", "Assam"), ("yo", "Assam"), ("yo_BJ", "Assam"), ("yue", "阿薩姆邦"), ("yue_Hans", "阿萨姆邦"), ("zh", "阿萨姆邦")]),
                        unofficial_name_list: ["Assam"].to_vec(),
                    }
                ),
                (
                    "BR",
                    Subdivision{
                        name: "Bihar",
                        country_alpha2: Alpha2::IN,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.0960742), longitude: Some(85.31311939999999), max_latitude: Some(27.520895), min_latitude: Some(24.286278), max_longitude: Some(88.289752), min_longitude: Some(83.31777)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bihar"), ("am", "ብሓር"), ("ar", "بيهار"), ("as", "বিহ\u{9be}ৰ"), ("az", "Bihar"), ("be", "Біхар"), ("bg", "Бихар"), ("bn", "বিহ\u{9be}র"), ("ca", "Bihar"), ("ccp", "𑄝𑄦𑄢\u{11134}"), ("ceb", "State of Bihār"), ("cs", "Bihár"), ("cy", "Bihar"), ("da", "Bihar"), ("de", "Bihar"), ("el", "Μπιχάρ"), ("en", "Bihar"), ("es", "Bihar"), ("et", "Bihār"), ("eu", "Bihar"), ("fa", "بیهار"), ("fi", "Bihar"), ("fr", "Bihar"), ("ga", "Bihar"), ("gl", "Bihar"), ("gu", "બિહાર"), ("he", "ביהר"), ("hi", "बिहार"), ("hr", "Bihar"), ("hu", "Bihár"), ("hy", "Բիհար"), ("id", "Bihar"), ("is", "Bíhar"), ("it", "Bihar"), ("ja", "ビハール州"), ("ka", "ბიჰარი"), ("kn", "ಬ\u{cbf}ಹಾರ"), ("ko", "비하르 주"), ("lt", "Biharas"), ("lv", "Bihāra"), ("mk", "Бихар"), ("ml", "ബിഹ\u{d3e}ർ"), ("mn", "Бихар"), ("mr", "बिहार"), ("ms", "Bihar"), ("my", "ဘ\u{102e}ဟာပြည\u{103a}နယ\u{103a}"), ("nb", "Bihar"), ("ne", "बिहार"), ("nl", "Bihar"), ("no", "Bihar"), ("or", "ବ\u{b3f}ହ\u{b3e}ର"), ("pa", "ਬਿਹਾਰ"), ("pl", "Bihar"), ("ps", "بهار"), ("pt", "Bihar"), ("ro", "Bihar"), ("ru", "Бихар"), ("si", "බ\u{dd2}හ\u{dcf}රය"), ("sk", "Bihár"), ("sr", "Бихар"), ("sr_Latn", "Bihar"), ("sv", "Bihar"), ("sw", "Bihar"), ("ta", "ப\u{bc0}க\u{bbe}ர\u{bcd}"), ("te", "బ\u{c40}హ\u{c3e}ర\u{c4d}"), ("th", "ร\u{e31}ฐพ\u{e34}หาร"), ("tr", "Bihar"), ("uk", "Біхар"), ("ur", "بہار"), ("uz", "Bihar"), ("vi", "Bihar"), ("yo", "Bihar"), ("yo_BJ", "Bihar"), ("yue", "比哈爾邦"), ("yue_Hans", "比哈尔邦"), ("zh", "比哈尔邦")]),
                        unofficial_name_list: ["Bihar"].to_vec(),
                    }
                ),
                (
                    "CH",
                    Subdivision{
                        name: "Chandigarh",
                        country_alpha2: Alpha2::IN,
                        code: "CH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.7333148), longitude: Some(76.7794179), max_latitude: Some(30.7958938), min_latitude: Some(30.5944928), max_longitude: Some(76.8529701), min_longitude: Some(76.6798587)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chandigarh"), ("ar", "شانديغار"), ("be", "Чандыгарх"), ("bg", "Чандигарх"), ("bn", "চণ\u{9cd}ডীগড\u{9bc}"), ("ca", "Chandigar"), ("ccp", "𑄌𑄚\u{11134}𑄘\u{11128}𑄊\u{11127}𑄢\u{11134}𑄦\u{11134}"), ("ceb", "Union Territory of Chandīgarh"), ("cs", "Čandígarh"), ("da", "Chandigarh"), ("de", "Chandigarh"), ("el", "Τσαντιγκάρ"), ("en", "Chandigarh"), ("es", "Chandigarh"), ("et", "Chandīgarh"), ("eu", "Chandigarh"), ("fa", "چندی\u{200c}گر"), ("fi", "Chandigarh"), ("fr", "Chandigarh"), ("ga", "Chandigarh"), ("gu", "ચ\u{a82}ડીગઢ"), ("he", "צ׳אנדיגאר"), ("hi", "चण\u{94d}डीगढ\u{93c}"), ("hr", "Chandigarh"), ("hu", "Csandígarh"), ("id", "Chandigarh"), ("is", "Chandigarh"), ("it", "Chandigarh"), ("ja", "チャンディーガル"), ("ka", "ჩანდიგარჰი"), ("kn", "ಚಂಡೀಗಡ"), ("ko", "찬디가르"), ("lt", "Čandigarchas"), ("lv", "Čandīgarha"), ("mk", "Чандигар"), ("ml", "ചണ\u{d4d}ഡീഗഢ\u{d4d}"), ("mr", "च\u{902}दिगढ"), ("ms", "Chandigarh"), ("nb", "Chandigarh"), ("ne", "चण\u{94d}डिगढ"), ("nl", "Chandigarh"), ("no", "Chandigarh"), ("or", "ଚଣ\u{b4d}ଡୀଗଡ\u{b3c}"), ("pa", "ਚ\u{a70}ਡੀਗੜ\u{a4d}ਹ"), ("pl", "Czandigarh"), ("ps", "چندي گړ"), ("pt", "Chandigarh"), ("ro", "Chandigarh"), ("ru", "Чандигарх"), ("sd", "چندي ڳڙھ"), ("si", "චන\u{dca}ද\u{dd2}ග\u{dcf}ර\u{dca}"), ("sk", "Čandígarh"), ("sr", "Чандигар"), ("sr_Latn", "Čandigar"), ("sv", "Chandigarh"), ("sw", "Chandigarh"), ("ta", "சண\u{bcd}டிகர\u{bcd}"), ("te", "చండ\u{c40}గఢ\u{c4d}"), ("th", "จ\u{e31}ณฑ\u{e35}ครห\u{e4c}"), ("tr", "Çhandigarh"), ("uk", "Чандігарх"), ("ur", "چندی گڑھ"), ("uz", "Chandigarh"), ("vi", "Chandigarh"), ("yo", "Chandigarh"), ("yo_BJ", "Chandigarh"), ("yue", "昌迪加爾"), ("yue_Hans", "昌迪加尔"), ("zh", "昌迪加尔")]),
                        unofficial_name_list: ["Chandigarh"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "Chhattisgarh",
                        country_alpha2: Alpha2::IN,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.2786567), longitude: Some(81.8661442), max_latitude: Some(24.1187091), min_latitude: Some(17.782531), max_longitude: Some(84.39599799999999), min_longitude: Some(80.2439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ጨቲስገርህ"), ("ar", "تشاتيسغار"), ("as", "ছত\u{9cd}তীছগঢ\u{9bc}"), ("be", "Чхатысгарх"), ("bg", "Чхатисгарх"), ("bn", "ছত\u{9cd}তীসগঢ\u{9bc}"), ("ca", "Chhattisgarh"), ("ccp", "𑄍𑄑\u{11133}𑄦\u{11128}𑄌\u{11134}𑄊\u{11127}𑄢\u{11134}𑄦\u{11134}"), ("ceb", "State of Chhattīsgarh"), ("cs", "Čhattísgarh"), ("cy", "Chhattisgarh"), ("da", "Chattisgarh"), ("de", "Chhattisgarh"), ("el", "Τσατίσγκαρ"), ("en", "Chhattisgarh"), ("es", "Chhattisgarh"), ("et", "Chhattīsgarh"), ("eu", "Chhattisgarh"), ("fa", "چتیسگر"), ("fi", "Chhattisgarh"), ("fr", "Chhattisgarh"), ("ga", "Chhattisgarh"), ("gu", "છત\u{acd}તીસગઢ"), ("he", "צ׳האטיסגאר"), ("hi", "छत\u{94d}तीसगढ\u{93c}"), ("hr", "Chhattisgarh"), ("hu", "Cshattíszgarh"), ("id", "Chhattisgarh"), ("is", "Chhattisgarh"), ("it", "Chhattisgarh"), ("ja", "チャッティースガル州"), ("ka", "ჩატისგარჰი"), ("kn", "ಛತ\u{ccd}ತೀಸ\u{ccd}\u{200c}ಘಡ\u{ccd}"), ("ko", "차티스가르 주"), ("lt", "Čatisgarchas"), ("lv", "Čhatīsgarha"), ("mk", "Чатисгар"), ("ml", "ഛത\u{d4d}തീസ\u{d4d}\u{200c}ഗഢ\u{d4d}"), ("mn", "Чаттисгар"), ("mr", "छत\u{94d}तीसगढ"), ("ms", "Chhattisgarh"), ("nb", "Chhattisgarh"), ("ne", "छत\u{94d}तीसगढ"), ("nl", "Chhattisgarh"), ("no", "Chhattisgarh"), ("or", "ଛତ\u{b3f}ଶଗଡ\u{b3c}"), ("pa", "ਛ\u{a71}ਤੀਸਗੜ\u{a4d}ਹ"), ("pl", "Chhattisgarh"), ("ps", "چتيس گړ"), ("pt", "Chhattisgarh"), ("ro", "Chhattisgarh"), ("ru", "Чхаттисгарх"), ("si", "චට\u{dca}ට\u{dd2}ස\u{dca}ග\u{dcf}ර\u{dca}"), ("sk", "Čhattísgarh"), ("sq", "Çatisgar"), ("sr", "Чатисгар"), ("sr_Latn", "Čatisgar"), ("sv", "Chhattisgarh"), ("sw", "Chhattisgarh"), ("ta", "சத\u{bcd}த\u{bc0}சுகர\u{bcd}"), ("te", "ఛత\u{c4d}త\u{c40}స\u{c4d}\u{200c}గఢ\u{c4d}"), ("th", "ร\u{e31}ฐฉ\u{e31}ตต\u{e35}สครห\u{e4c}"), ("tr", "Chhattisgarh"), ("uk", "Чхаттісґарх"), ("ur", "چھتیس گڑھ"), ("uz", "Chhattisgarh"), ("vi", "Chhattisgarh"), ("yo", "Chhattisgarh"), ("yo_BJ", "Chhattisgarh"), ("yue", "恰蒂斯加爾邦"), ("yue_Hans", "恰蒂斯加尔邦"), ("zh", "恰蒂斯加尔邦")]),
                        unofficial_name_list: ["Chhattisgarh"].to_vec(),
                    }
                ),
                (
                    "DH",
                    Subdivision{
                        name: "Dādra and Nagar Haveli and Damān and Diu",
                        country_alpha2: Alpha2::IN,
                        code: "DH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Dādra and Nagar Haveli and Damān and Diu")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DL",
                    Subdivision{
                        name: "Delhi",
                        country_alpha2: Alpha2::IN,
                        code: "DL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.6139391), longitude: Some(77.2090212), max_latitude: Some(28.889816), min_latitude: Some(28.4010669), max_longitude: Some(77.3418147), min_longitude: Some(76.8396999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Delhi"), ("am", "ዴሊ"), ("ar", "دلهي"), ("as", "দিল\u{9cd}লী"), ("az", "Dehli"), ("be", "Горад Дэлі"), ("bg", "Делхи"), ("bn", "দিল\u{9cd}লি"), ("bs", "Delhi"), ("ca", "Delhi"), ("ccp", "𑄘\u{11128}𑄣\u{11133}𑄣\u{11128}"), ("cs", "Dillí"), ("cy", "Delhi"), ("da", "Delhi"), ("de", "Delhi"), ("el", "Δελχί"), ("en", "Delhi"), ("es", "Delhi"), ("et", "Delhi"), ("eu", "Delhi"), ("fa", "دهلی"), ("fi", "Delhi"), ("fr", "Delhi"), ("ga", "Delhi"), ("gl", "Delhi"), ("gu", "દિલ\u{acd}હી"), ("he", "דלהי"), ("hi", "दिल\u{94d}ली"), ("hr", "Delhi"), ("hu", "Delhi"), ("hy", "Դելի"), ("id", "Delhi"), ("is", "Delí"), ("it", "Delhi"), ("ja", "デリー"), ("ka", "დელი"), ("kk", "Дели"), ("kn", "ದ\u{cc6}ಹಲ\u{cbf}"), ("ko", "델리"), ("ky", "Дели шаары"), ("lt", "Delis"), ("lv", "Deli"), ("mk", "Делхи"), ("ml", "ഡെൽഹി"), ("mn", "Дели"), ("mr", "दिल\u{94d}ली"), ("ms", "Delhi"), ("my", "ဒေလ\u{102e}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Delhi"), ("ne", "दिल\u{94d}ली"), ("nl", "Delhi"), ("no", "Delhi"), ("or", "ଦ\u{b3f}ଲ\u{b4d}ଲୀ"), ("pa", "ਦਿ\u{a71}ਲੀ"), ("pl", "Delhi"), ("ps", "ډېلي"), ("pt", "Deli"), ("ro", "Delhi"), ("ru", "Дели"), ("sd", "دهلي"), ("si", "ද\u{dd2}ල\u{dca}ල\u{dd2}ය"), ("sk", "Dillí"), ("sl", "Delhi"), ("sr", "Делхи"), ("sr_Latn", "Delhi"), ("sv", "Delhi"), ("sw", "Delhi"), ("ta", "தில\u{bcd}லி"), ("te", "ఢ\u{c3f}ల\u{c4d}ల\u{c40}"), ("th", "เดล\u{e35}"), ("tk", "Deli"), ("tr", "Delhi"), ("uk", "Делі"), ("ur", "دلی"), ("uz", "Dehli"), ("vi", "Delhi"), ("yo", "Delhi"), ("yo_BJ", "Delhi"), ("yue", "德里"), ("yue_Hans", "德里"), ("zh", "德里")]),
                        unofficial_name_list: ["Delhi"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "Goa",
                        country_alpha2: Alpha2::IN,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.2993265), longitude: Some(74.12399599999999), max_latitude: Some(15.799917), min_latitude: Some(14.8971223), max_longitude: Some(74.3405329), min_longitude: Some(73.6894238)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Goa"), ("am", "ጎዋ"), ("ar", "غوا"), ("as", "গোৱ\u{9be}"), ("az", "Qoa"), ("be", "Гоа"), ("bg", "Гоа"), ("bn", "গোয\u{9bc}\u{9be}"), ("ca", "Goa"), ("ccp", "𑄉\u{1112e}𑄠"), ("ceb", "Goa"), ("cs", "Goa"), ("cy", "Goa"), ("da", "Goa"), ("de", "Goa"), ("el", "Γκόα"), ("en", "Goa"), ("es", "Goa"), ("et", "Goa osariik"), ("eu", "Goa"), ("fa", "گوا"), ("fi", "Goa"), ("fr", "Goa"), ("ga", "Goa"), ("gl", "Goa"), ("gu", "ગોઆ"), ("he", "גואה"), ("hi", "गोआ"), ("hr", "Goa"), ("hu", "Goa"), ("hy", "Գոա"), ("id", "Goa, India"), ("is", "Góa"), ("it", "Goa"), ("ja", "ゴア州"), ("ka", "გოა"), ("kk", "Гоа"), ("kn", "ಗೋವಾ"), ("ko", "고아 주"), ("lt", "Goa"), ("lv", "Goa"), ("mk", "Гоа"), ("ml", "ഗോവ"), ("mn", "Гоа"), ("mr", "गोवा"), ("ms", "Goa"), ("my", "ဂ\u{102d}\u{102f}အာ"), ("nb", "Goa"), ("ne", "गोआ"), ("nl", "Goa"), ("no", "Goa"), ("or", "ଗୋଆ"), ("pa", "ਗ\u{a4b}ਆ"), ("pl", "Goa"), ("ps", "گوا"), ("pt", "Goa"), ("ro", "Goa"), ("ru", "Гоа"), ("si", "ගෝව\u{dcf}"), ("sk", "Goa"), ("sl", "Goa"), ("sq", "Goa"), ("sr", "Гоа"), ("sr_Latn", "Goa"), ("sv", "Goa"), ("sw", "Goa"), ("ta", "கோவ\u{bbe}"), ("te", "గ\u{c4b}వ\u{c3e}"), ("th", "ร\u{e31}ฐก\u{e31}ว"), ("tr", "Goa"), ("uk", "Гоа"), ("ur", "گوا"), ("uz", "Goa"), ("vi", "Goa"), ("yo", "Goa"), ("yo_BJ", "Goa"), ("yue", "果阿"), ("yue_Hans", "果阿"), ("zh", "果阿邦")]),
                        unofficial_name_list: ["Goa"].to_vec(),
                    }
                ),
                (
                    "GJ",
                    Subdivision{
                        name: "Gujarat",
                        country_alpha2: Alpha2::IN,
                        code: "GJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.258652), longitude: Some(71.1923805), max_latitude: Some(24.705709), min_latitude: Some(20.127954), max_longitude: Some(74.4764881), min_longitude: Some(68.1915379)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gujarat"), ("am", "ጉጀራት"), ("ar", "غوجارات"), ("as", "গ\u{9c1}জৰ\u{9be}ট"), ("az", "Qucarat"), ("be", "Гуджарат"), ("bg", "Гуджарат"), ("bn", "গ\u{9c1}জর\u{9be}ত"), ("ca", "Gujarat"), ("ccp", "𑄉\u{1112a}𑄌\u{11134}𑄢𑄖\u{11134}"), ("ceb", "State of Gujarāt"), ("cs", "Gudžarát"), ("cy", "Gujarat"), ("da", "Gujarat"), ("de", "Gujarat"), ("el", "Γκουτζαράτ"), ("en", "Gujarat"), ("es", "Guyarat"), ("et", "Gujarāt"), ("eu", "Gujarat"), ("fa", "گجرات"), ("fi", "Gujarat"), ("fr", "Gujarat"), ("ga", "An Ghúisearáit"), ("gu", "ગ\u{ac1}જરાત"), ("he", "גוג׳ראט"), ("hi", "ग\u{941}जरात"), ("hr", "Gudžerat"), ("hu", "Gudzsarát"), ("hy", "Գուջարաթ"), ("id", "Gujarat"), ("is", "Gujarat"), ("it", "Gujarat"), ("ja", "グジャラート州"), ("ka", "გუჯარათი"), ("kn", "ಗುಜರಾತ\u{ccd}"), ("ko", "구자라트 주"), ("lt", "Gudžaratas"), ("lv", "Gudžarāta"), ("mk", "Гуџарат"), ("ml", "ഗ\u{d41}ജറ\u{d3e}ത\u{d4d}ത\u{d4d}\u{200c}"), ("mn", "Гужарат"), ("mr", "ग\u{941}जरात"), ("ms", "Gujarat"), ("nb", "Gujarat"), ("ne", "ग\u{941}जरात"), ("nl", "Gujarat"), ("no", "Gujarat"), ("or", "ଗ\u{b41}ଜର\u{b3e}ଟ"), ("pa", "ਗ\u{a41}ਜਰਾਤ"), ("pl", "Gudźarat"), ("ps", "گوجرات"), ("pt", "Gujarate"), ("ro", "Gujarat"), ("ru", "Гуджарат"), ("sd", "گوجارات"), ("si", "ග\u{dd4}ජර\u{dcf}ටය"), ("sk", "Gudžarát"), ("sl", "Gudžarat"), ("sq", "Guxharat"), ("sr", "Гуџарат"), ("sr_Latn", "Gudžarat"), ("sv", "Gujarat"), ("sw", "Gujarat"), ("ta", "குசர\u{bbe}த\u{bcd}"), ("te", "గుజర\u{c3e}త\u{c4d}"), ("th", "ร\u{e31}ฐค\u{e38}ชราต"), ("tk", "Güjerat"), ("tr", "Gucerat"), ("uk", "Гуджарат"), ("ur", "گجرات"), ("uz", "Gujarot"), ("vi", "Gujarat"), ("yo", "Gujarat"), ("yo_BJ", "Gujarat"), ("yue", "古吉拉特邦"), ("yue_Hans", "古吉拉特邦"), ("zh", "古吉拉特邦")]),
                        unofficial_name_list: ["Gujarat"].to_vec(),
                    }
                ),
                (
                    "HP",
                    Subdivision{
                        name: "Himachal Pradesh",
                        country_alpha2: Alpha2::IN,
                        code: "HP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.1048294), longitude: Some(77.17339009999999), max_latitude: Some(33.257958), min_latitude: Some(30.382469), max_longitude: Some(79.003309), min_longitude: Some(75.5874709)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ሂማቸል ፕረዴሽ"), ("ar", "هيماجل برديش"), ("as", "হিম\u{9be}চল প\u{9cd}ৰদেশ"), ("az", "Himaçal Pradeş"), ("be", "Хімачал-Прадэш"), ("bg", "Химачал Прадеш"), ("bn", "হিম\u{9be}চল প\u{9cd}রদেশ"), ("ca", "Himachal Pradesh"), ("ccp", "𑄦\u{11128}𑄟𑄌\u{11127}𑄣\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄘𑄬𑄌\u{11134}"), ("ceb", "State of Himāchal Pradesh"), ("cs", "Himáčalpradéš"), ("cy", "Himachal Pradesh"), ("da", "Himachal Pradesh"), ("de", "Himachal Pradesh"), ("el", "Χιμάτσαλ Πραντές"), ("en", "Himachal Pradesh"), ("es", "Himachal Pradesh"), ("et", "Himachal Pradesh"), ("eu", "Himachal Pradesh"), ("fa", "هیماچال پرادش"), ("fi", "Himachal Pradesh"), ("fr", "Himachal Pradesh"), ("ga", "Himachal Pradesh"), ("gu", "હિમાચલ પ\u{acd}રદ\u{ac7}શ"), ("he", "הימאצ׳ל פרדש"), ("hi", "हिमाचल प\u{94d}रद\u{947}श"), ("hr", "Himachal Pradesh"), ("hu", "Himácsal Prades"), ("hy", "Հիմաչալ-Պրադեշ"), ("id", "Himachal Pradesh"), ("is", "Himachal Pradesh"), ("it", "Himachal Pradesh"), ("ja", "ヒマーチャル・プラデーシュ州"), ("ka", "ჰიმაჩალ-პრადეში"), ("kn", "ಹ\u{cbf}ಮಾಚಲ ಪ\u{ccd}ರದೇಶ"), ("ko", "히마찰프라데시 주"), ("lt", "Himačal Pradešas"), ("lv", "Himāčala Pradēša"), ("mk", "Химачал Прадеш"), ("ml", "ഹിമ\u{d3e}ചൽ പ\u{d4d}രദേശ\u{d4d}\u{200c}"), ("mn", "Химачалпрадеш"), ("mr", "हिमाचल प\u{94d}रद\u{947}श"), ("ms", "Himachal Pradesh"), ("nb", "Himachal Pradesh"), ("ne", "हिमाचल प\u{94d}रद\u{947}श"), ("nl", "Himachal Pradesh"), ("no", "Himachal Pradesh"), ("or", "ହ\u{b3f}ମ\u{b3e}ଚଳ ପ\u{b4d}ରଦେଶ"), ("pa", "ਹਿਮਾਚਲ ਪ\u{a4d}ਰਦ\u{a47}ਸ\u{a3c}"), ("pl", "Himachal Pradesh"), ("ps", "هماچل پرديش"), ("pt", "Himachal Pradesh"), ("ro", "Himachal Pradesh"), ("ru", "Химачал-Прадеш"), ("si", "හ\u{dd2}ම\u{dcf}චල\u{dca} ප\u{dca}\u{200d}රදේශ\u{dca}"), ("sk", "Himáčalpradéš"), ("sl", "Himačal Pradeš"), ("sr", "Химачал Прадеш"), ("sr_Latn", "Himačal Pradeš"), ("sv", "Himachal Pradesh"), ("sw", "Himachal Pradesh"), ("ta", "இம\u{bbe}சலப\u{bcd} பிரதேசம\u{bcd}"), ("te", "హ\u{c3f}మ\u{c3e}చల\u{c4d} ప\u{c4d}రద\u{c47}శ\u{c4d}"), ("th", "ร\u{e31}ฐห\u{e34}มาจ\u{e31}ลประเทศ"), ("tk", "Himachal Pradesh"), ("tr", "Himaçhal Pradeş"), ("uk", "Гімачал-Прадеш"), ("ur", "ہماچل پردیش"), ("uz", "Ximachalpradesh"), ("vi", "Himachal Pradesh"), ("yo", "Himachal Pradesh"), ("yo_BJ", "Himachal Pradesh"), ("yue", "喜馬偕爾邦"), ("yue_Hans", "喜马偕尔邦"), ("zh", "喜马偕尔邦")]),
                        unofficial_name_list: ["Himachal Pradesh"].to_vec(),
                    }
                ),
                (
                    "HR",
                    Subdivision{
                        name: "Haryana",
                        country_alpha2: Alpha2::IN,
                        code: "HR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(29.0587757), longitude: Some(76.085601), max_latitude: Some(30.9128649), min_latitude: Some(27.6529931), max_longitude: Some(77.59544799999999), min_longitude: Some(74.457616)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Harjana"), ("am", "ሀርያና"), ("ar", "هاريانا"), ("as", "হ\u{9be}ৰিয\u{9bc}\u{9be}ন\u{9be}"), ("bg", "Харяна"), ("bn", "হরিয\u{9bc}\u{9be}ন\u{9be}"), ("ca", "Haryana"), ("ccp", "𑄦\u{11127}𑄢\u{11128}𑄠𑄚"), ("ceb", "State of Haryāna"), ("cs", "Harijána"), ("cy", "Haryana"), ("da", "Haryana"), ("de", "Haryana"), ("el", "Χαρυάνα"), ("en", "Haryana"), ("es", "Haryana"), ("et", "Haryana"), ("eu", "Haryana"), ("fa", "هاریانا"), ("fi", "Haryana"), ("fr", "Haryana"), ("ga", "Haryana"), ("gl", "Haryana"), ("gu", "હરિયાણા"), ("he", "הריאנה"), ("hi", "हरियाणा"), ("hr", "Haryana"), ("hu", "Harijána"), ("hy", "Հարյանա"), ("id", "Haryana"), ("is", "Haryana"), ("it", "Haryana"), ("ja", "ハリヤーナー州"), ("ka", "ჰარიანა"), ("kn", "ಹರ\u{cbf}ಯಾಣ"), ("ko", "하리아나 주"), ("lt", "Harjana"), ("lv", "Harjāna"), ("mk", "Харајана"), ("ml", "ഹരിയ\u{d3e}ണ"), ("mn", "Харяна"), ("mr", "हरियाणा"), ("ms", "Haryana"), ("nb", "Haryana"), ("ne", "हरियाणा"), ("nl", "Haryana"), ("no", "Haryana"), ("or", "ହର\u{b3f}ୟ\u{b3e}ଣ\u{b3e}"), ("pa", "ਹਰਿਆਣਾ"), ("pl", "Hariana"), ("ps", "هريانه"), ("pt", "Haryana"), ("ro", "Haryana"), ("ru", "Харьяна"), ("si", "හර\u{dca}ය\u{dcf}න"), ("sk", "Harijána"), ("sr", "Харајана"), ("sr_Latn", "Harajana"), ("sv", "Haryana"), ("sw", "Haryana"), ("ta", "அரிய\u{bbe}ன\u{bbe}"), ("te", "హర\u{c4d}య\u{c3e}న\u{c3e}"), ("th", "ร\u{e31}ฐหรยาณา"), ("tr", "Haryana"), ("uk", "Харʼяна"), ("ur", "ہریانہ"), ("uz", "Xaryana"), ("vi", "Haryana"), ("yo", "Haryana"), ("yo_BJ", "Haryana"), ("yue", "哈里亞納邦"), ("yue_Hans", "哈里亚纳邦"), ("zh", "哈里亚纳邦")]),
                        unofficial_name_list: ["Haryana"].to_vec(),
                    }
                ),
                (
                    "JH",
                    Subdivision{
                        name: "Jharkhand",
                        country_alpha2: Alpha2::IN,
                        code: "JH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.6101808), longitude: Some(85.2799354), max_latitude: Some(25.328823), min_latitude: Some(21.9729309), max_longitude: Some(87.947529), min_longitude: Some(83.32382799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ጃርኸንድ"), ("ar", "جهارخاند"), ("as", "ঝ\u{9be}ড\u{9bc}খণ\u{9cd}ড"), ("be", "Джхаркханд"), ("bg", "Джаркханд"), ("bn", "ঝ\u{9be}ড\u{9bc}খণ\u{9cd}ড"), ("ca", "Jharkhand"), ("ccp", "𑄏𑄢\u{11134}𑄈\u{11127}𑄚\u{11134}𑄓\u{11127}"), ("ceb", "State of Jharkhand"), ("cs", "Džhárkhand"), ("cy", "Jharkhand"), ("da", "Jharkhand"), ("de", "Jharkhand"), ("el", "Τζαρκάντ"), ("en", "Jharkhand"), ("es", "Jharkhand"), ("et", "Jharkhand"), ("eu", "Jharkhand"), ("fa", "جارکند"), ("fi", "Jharkhand"), ("fr", "Jharkhand"), ("ga", "Jharkhand"), ("gu", "ઝારખ\u{a82}ડ"), ("he", "ג׳הרקאנד"), ("hi", "झारखण\u{94d}ड"), ("hr", "Jharkhand"), ("hu", "Dzshárkhand"), ("id", "Jharkhand"), ("is", "Jharkhand"), ("it", "Jharkhand"), ("ja", "ジャールカンド州"), ("ka", "ჯარხანდი"), ("kn", "ಝಾರ\u{ccd}ಖಂಡ\u{ccd}"), ("ko", "자르칸드 주"), ("lt", "Džarkandas"), ("lv", "Džhārkhanda"), ("mk", "Џарканд"), ("ml", "ഝ\u{d3e}ർഖണ\u{d4d}ഡ\u{d4d}\u{200c}"), ("mn", "Жарканд"), ("mr", "झारख\u{902}ड"), ("ms", "Jharkhand"), ("nb", "Jharkhand"), ("ne", "झारखण\u{94d}ड"), ("nl", "Jharkhand"), ("no", "Jharkhand"), ("or", "ଝ\u{b3e}ଡ\u{b3c}ଖଣ\u{b4d}ଡ"), ("pa", "ਝਾਰਖ\u{a70}ਡ"), ("pl", "Jharkhand"), ("ps", "جارکنډ"), ("pt", "Jharkhand"), ("ro", "Jharkhand"), ("ru", "Джаркханд"), ("si", "ජ\u{dcf}ර\u{dca}ක\u{dca}හ\u{dcf}න\u{dca}ඩ\u{dca}"), ("sk", "Džhárkhand"), ("sr", "Џарканд"), ("sr_Latn", "Džarkand"), ("sv", "Jharkhand"), ("sw", "Jharkhand"), ("ta", "ச\u{bbe}ர\u{bcd}க\u{bcd}கண\u{bcd}ட\u{bcd}"), ("te", "జ\u{c3e}ర\u{c4d}ఖండ\u{c4d}"), ("th", "ร\u{e31}ฐฌารข\u{e31}ณฑ\u{e4c}"), ("tr", "Jharkhand"), ("uk", "Джхаркханд"), ("ur", "جھاڑکھنڈ"), ("vi", "Jharkhand"), ("yo", "Jharkhand"), ("yo_BJ", "Jharkhand"), ("yue", "乍拉肯德邦"), ("yue_Hans", "乍拉肯德邦"), ("zh", "贾坎德邦")]),
                        unofficial_name_list: ["Vananchal"].to_vec(),
                    }
                ),
                (
                    "JK",
                    Subdivision{
                        name: "Jammu and Kashmir",
                        country_alpha2: Alpha2::IN,
                        code: "JK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.1490875), longitude: Some(76.8259652), max_latitude: Some(35.5042109), min_latitude: Some(32.252741), max_longitude: Some(79.56429290000001), min_longitude: Some(73.7632221)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ጃሙና ካሽሚር"), ("ar", "جامو وكشمير"), ("as", "জম\u{9cd}ম\u{9c1} আৰ\u{9c1} ক\u{9be}শ\u{9cd}মীৰ"), ("az", "Cammu və Kəşmir"), ("be", "Джаму і Кашмір"), ("bg", "Джаму и Кашмир"), ("bn", "জম\u{9cd}ম\u{9c1} ও ক\u{9be}শ\u{9cd}মীর"), ("bs", "Jammu i Kashmir"), ("ca", "Jammu i Caixmir"), ("ccp", "𑄎𑄟\u{11134}𑄟\u{1112a} 𑄃\u{11133}𑄃 𑄇𑄌\u{11134}𑄟\u{11128}𑄢\u{11134}"), ("ceb", "State of Jammu and Kashmīr"), ("cs", "Džammú a Kašmír"), ("cy", "Jammu a Kashmir"), ("da", "Jammu and Kashmir"), ("de", "Jammu und Kashmir"), ("el", "Γιαμού και Κασμίρ"), ("en", "Jammu and Kashmir"), ("es", "Jammu y Cachemira"), ("et", "Jammu ja Kashmir"), ("eu", "Jammu eta Kaxmir"), ("fa", "جامو و کشمیر"), ("fi", "Jammu ja Kashmir"), ("fr", "Jammu-et-Cachemire"), ("ga", "Jammu agus Kashmir"), ("gu", "જમ\u{acd}મ\u{ac1} અન\u{ac7} કાશ\u{acd}મીર"), ("he", "ג׳אמו וקשמיר"), ("hi", "जम\u{94d}म\u{942} और कश\u{94d}मीर"), ("hr", "Jammu i Kashmir"), ("hu", "Dzsammu és Kasmír"), ("hy", "Ջամմու և Քաշմիր"), ("id", "Jammu dan Kashmir"), ("is", "Jammú og Kasmír"), ("it", "Jammu e Kashmir"), ("ja", "ジャンムー・カシミール州"), ("ka", "ჯამუ და ქაშმირი"), ("kn", "ಜಮ\u{ccd}ಮು ಮತ\u{ccd}ತು ಕಾಶ\u{ccd}ಮೀರ"), ("ko", "잠무 카슈미르 주"), ("lt", "Džamu ir Kašmyras"), ("lv", "Džammu un Kašmīra"), ("mk", "Џаму-Кашмир"), ("ml", "ജമ\u{d4d}മ\u{d41}-കശ\u{d4d}മീർ"), ("mn", "Жамму-Кашмир"), ("mr", "जम\u{94d}म\u{942} आणि काश\u{94d}मीर"), ("ms", "Jammu dan Kashmir"), ("my", "ဂျမ\u{103a}မ\u{1030}း န\u{103e}င\u{1037}\u{103a} ကက\u{103a}ရ\u{103e}မ\u{102e}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "Jammu og Kashmir"), ("ne", "जम\u{94d}म\u{941} कश\u{94d}मीर"), ("nl", "Jammu en Kasjmir"), ("no", "Jammu og Kashmir"), ("or", "ଜମ\u{b4d}ମ\u{b41} ଓ କଶ\u{b4d}ମୀର"), ("pa", "ਜ\u{a70}ਮ\u{a42} ਅਤ\u{a47} ਕਸ\u{a3c}ਮੀਰ"), ("pl", "Dżammu i Kaszmir"), ("ps", "جمو او کشمير"), ("pt", "Jammu e Caxemira"), ("ro", "Jammu și Cașmir"), ("ru", "Джамму и Кашмир"), ("si", "ජම\u{dca}ම\u{dd4} සහ ක\u{dcf}ශ\u{dca}ම\u{dd3}රය"), ("sk", "Džammú a Kašmír"), ("sl", "Džamu in Kašmir"), ("sr", "Џаму и Кашмир"), ("sr_Latn", "Džamu i Kašmir"), ("sv", "Jammu och Kashmir"), ("ta", "சம\u{bcd}மு க\u{bbe}சும\u{bc0}ர\u{bcd}"), ("te", "జమ\u{c4d}మూ క\u{c3e}శ\u{c4d}మ\u{c40}రు"), ("th", "ร\u{e31}ฐช\u{e31}มม\u{e39}และก\u{e31}ศม\u{e35}ร\u{e4c}"), ("tr", "Cemmu ve Keşmir"), ("uk", "Джамму й Кашмір"), ("ur", "جموں و کشمیر"), ("uz", "Jammu va kashmir"), ("vi", "Jammu và Kashmir"), ("yo", "Jammu àti Kashmir"), ("yo_BJ", "Jammu àti Kashmir"), ("yue", "查謨-克什米爾邦"), ("yue_Hans", "查谟-克什米尔邦"), ("zh", "查谟－克什米尔邦")]),
                        unofficial_name_list: ["Jammu and Kashmir"].to_vec(),
                    }
                ),
                (
                    "KA",
                    Subdivision{
                        name: "Karnataka",
                        country_alpha2: Alpha2::IN,
                        code: "KA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.3172775), longitude: Some(75.7138884), max_latitude: Some(18.4411689), min_latitude: Some(11.593352), max_longitude: Some(78.5860101), min_longitude: Some(74.0928802)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Karnataka"), ("am", "ካርናተከ"), ("ar", "كارناتاكا"), ("as", "কৰ\u{9cd}ণ\u{9be}টক"), ("be", "Карнатака"), ("bg", "Карнатака"), ("bn", "কর\u{9cd}ণ\u{9be}টক"), ("ca", "Estat de Karnataka"), ("ccp", "𑄇\u{11127}𑄢\u{11134}𑄚𑄖\u{11134}𑄇"), ("ceb", "State of Karnātaka"), ("cs", "Karnátaka"), ("cy", "Karnataka"), ("da", "Karnataka"), ("de", "Karnataka"), ("el", "Καρνάτακα"), ("en", "Karnataka"), ("es", "Karnataka"), ("et", "Karnataka"), ("eu", "Karnataka"), ("fa", "کارناتاکا"), ("fi", "Karnataka"), ("fr", "Karnataka"), ("ga", "Karnataka"), ("gu", "કર\u{acd}ણાટક"), ("he", "קרנאטקה"), ("hi", "कर\u{94d}नाटक"), ("hr", "Karnataka"), ("hu", "Karnátaka"), ("hy", "Կառնատակա"), ("id", "Karnataka"), ("is", "Karnataka"), ("it", "Karnataka"), ("ja", "カルナータカ州"), ("ka", "კარნატაკა"), ("kn", "ಕರ\u{ccd}ನಾಟಕ"), ("ko", "카르나타카 주"), ("lt", "Karnataka"), ("lv", "Karnātaka"), ("mk", "Карнатака"), ("ml", "കർണ\u{d3e}ടക"), ("mn", "Карнатака"), ("mr", "कर\u{94d}नाटक"), ("ms", "Karnataka"), ("nb", "Karnataka"), ("ne", "कर\u{94d}नाटक"), ("nl", "Karnataka"), ("no", "Karnataka"), ("or", "କର\u{b4d}ଣ\u{b4d}ଣ\u{b3e}ଟକ"), ("pa", "ਕਰਨਾਟਕ"), ("pl", "Karnataka"), ("ps", "کرناټک"), ("pt", "Karnataka"), ("ro", "Karnataka"), ("ru", "Карнатака"), ("si", "කමටක\u{dcf}"), ("sk", "Karnátaka"), ("sr", "Карнатака"), ("sr_Latn", "Karnataka"), ("sv", "Karnataka"), ("sw", "Karnataka"), ("ta", "கருந\u{bbe}டகம\u{bcd}"), ("te", "కర\u{c4d}ణ\u{c3e}టక"), ("th", "ร\u{e31}ฐกรณาฏกะ"), ("tr", "Karnataka"), ("uk", "Карнатака"), ("ur", "کرناٹک"), ("uz", "Karnataka"), ("vi", "Karnataka"), ("yo", "Karnataka"), ("yo_BJ", "Karnataka"), ("yue", "卡納塔克邦"), ("yue_Hans", "卡纳塔克邦"), ("zh", "卡纳塔克邦")]),
                        unofficial_name_list: ["Kanara"].to_vec(),
                    }
                ),
                (
                    "KL",
                    Subdivision{
                        name: "Kerala",
                        country_alpha2: Alpha2::IN,
                        code: "KL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.8505159), longitude: Some(76.2710833), max_latitude: Some(12.7883001), min_latitude: Some(8.294896999999999), max_longitude: Some(77.3956369), min_longitude: Some(74.8649065)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kerala"), ("am", "ኬረለ"), ("ar", "كيرلا"), ("as", "কেৰেল\u{9be}"), ("be", "Керала"), ("bg", "Керала"), ("bn", "কেরল"), ("bs", "Kerala"), ("ca", "Kerala"), ("ccp", "𑄇𑄬𑄢𑄬𑄣"), ("ceb", "State of Kerala"), ("cs", "Kérala"), ("cy", "Kerala"), ("da", "Kerala"), ("de", "Kerala"), ("el", "Κεράλα"), ("en", "Kerala"), ("es", "Kerala"), ("et", "Kerala"), ("eu", "Kerala"), ("fa", "کرالا"), ("fi", "Kerala"), ("fr", "Kerala"), ("ga", "Kerala"), ("gu", "ક\u{ac7}રળ"), ("he", "קרלה"), ("hi", "क\u{947}रल"), ("hr", "Kerala"), ("hu", "Kerala"), ("hy", "Կերալա"), ("id", "Kerala"), ("is", "Kerala"), ("it", "Kerala"), ("ja", "ケーララ州"), ("ka", "კერალა"), ("kn", "ಕೇರಳ"), ("ko", "케랄라 주"), ("lt", "Kerala"), ("lv", "Kerala"), ("mk", "Керала"), ("ml", "കേരളം"), ("mn", "Керала"), ("mr", "क\u{947}रळ"), ("ms", "Kerala"), ("nb", "Kerala"), ("ne", "क\u{947}रल"), ("nl", "Kerala"), ("no", "Kerala"), ("or", "କେରଳ"), ("pa", "ਕ\u{a47}ਰਲਾ"), ("pl", "Kerala"), ("ps", "کيرالا"), ("pt", "Kerala"), ("ro", "Kerala"), ("ru", "Керала"), ("si", "කේරල"), ("sk", "Kérala"), ("sl", "Kerala"), ("sq", "Kerala"), ("sr", "Керала"), ("sr_Latn", "Kerala"), ("sv", "Kerala"), ("sw", "Kerala"), ("ta", "கேரளம\u{bcd}"), ("te", "క\u{c47}రళ"), ("th", "ร\u{e31}ฐเกรละ"), ("tr", "Kerala"), ("uk", "Керала"), ("ur", "کیرلا"), ("uz", "Kerala"), ("vi", "Kerala"), ("yo", "Kerala"), ("yo_BJ", "Kerala"), ("yue", "基拉拉邦"), ("yue_Hans", "基拉拉邦"), ("zh", "喀拉拉邦")]),
                        unofficial_name_list: ["Kerala"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "Ladakh",
                        country_alpha2: Alpha2::IN,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0522342), longitude: Some(-118.2436849), max_latitude: Some(34.3373061), min_latitude: Some(33.7036519), max_longitude: Some(-118.1552891), min_longitude: Some(-118.6681759)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Ladakh")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LD",
                    Subdivision{
                        name: "Lakshadweep",
                        country_alpha2: Alpha2::IN,
                        code: "LD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.0760115), longitude: Some(73.6303446), max_latitude: Some(12.3934258), min_latitude: Some(8.2658179), max_longitude: Some(73.6848297), min_longitude: Some(71.8844221)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لكشديب"), ("be", "Лакшадвіп"), ("bg", "Лакшадвип"), ("bn", "লক\u{9cd}ষদ\u{9cd}বীপ"), ("ca", "Lakshadweep"), ("ccp", "𑄣𑄇\u{11134}𑄥\u{11128}𑄓\u{11129}𑄛\u{11134}"), ("ceb", "Union Territory of Lakshadweep"), ("cs", "Lakadivy"), ("da", "Lakshadweep"), ("de", "Lakshadweep"), ("el", "Λακσαντγουίπ"), ("en", "Lakshadweep"), ("es", "Laquedivas"), ("et", "Lakshadweep"), ("eu", "Lakshadweep"), ("fa", "لاکشادویپ"), ("fi", "Lakkadiivit"), ("fr", "Lakshadweep"), ("gu", "લક\u{acd}ષદ\u{acd}વીપ"), ("he", "לקשאדוויפ"), ("hi", "लक\u{94d}षद\u{94d}वीप"), ("hr", "Lakadivi"), ("hu", "Laksadíva"), ("id", "Lakshadweep"), ("is", "Lakshadweep"), ("it", "Laccadive"), ("ja", "ラクシャディープ諸島"), ("ka", "ლაქშადვიპი"), ("kn", "ಲಕ\u{ccd}ಷದ\u{ccd}ವೀಪ"), ("ko", "락샤드위프 제도"), ("lt", "Lakšadvipas"), ("lv", "Lakšadvīpa"), ("mk", "Лакадиви"), ("ml", "ലക\u{d4d}ഷദ\u{d4d}വീപ\u{d4d}"), ("mr", "लक\u{94d}षद\u{94d}वीप"), ("ms", "Lakshadweep"), ("nb", "Lakkadivene"), ("ne", "लक\u{94d}षद\u{94d}वीप"), ("nl", "Laccadiven"), ("no", "Lakkadivene"), ("or", "ଲ\u{b3e}କ\u{b4d}ଷ\u{b3e}ଦ\u{b4d}ଵୀପ"), ("pa", "ਲਕਸ\u{a3c}ਦੀਪ"), ("pl", "Lakszadiwy"), ("pt", "Laquedivas"), ("ro", "Lakshadweep"), ("ru", "Лакшадвип"), ("si", "ලක\u{dca}ෂද\u{dca}ව\u{dd3}ප\u{dca}"), ("sk", "Lakadivy"), ("sr", "Лакадиви"), ("sr_Latn", "Lakadivi"), ("sv", "Lakshadweep"), ("sw", "Lakshadweep"), ("ta", "இலட\u{bcd}சத\u{bcd}த\u{bc0}வுகள\u{bcd}"), ("te", "లక\u{c4d}షద\u{c4d}వ\u{c40}పములు"), ("th", "ล\u{e31}กษทว\u{e35}ป"), ("tr", "Lakşadvip Adaları"), ("uk", "Лакшадвіп"), ("ur", "لکشادیپ"), ("uz", "Lakshadvip"), ("vi", "Lakshadweep"), ("yo", "Lakshadweep"), ("yo_BJ", "Lakshadweep"), ("yue", "拉克沙群島"), ("yue_Hans", "拉克沙群岛"), ("zh", "拉克沙群島")]),
                        unofficial_name_list: ["Laccadive", "Lakkadiven"].to_vec(),
                    }
                ),
                (
                    "MH",
                    Subdivision{
                        name: "Maharashtra",
                        country_alpha2: Alpha2::IN,
                        code: "MH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.7514798), longitude: Some(75.7138884), max_latitude: Some(22.028441), min_latitude: Some(15.6024121), max_longitude: Some(80.890924), min_longitude: Some(72.659363)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Maharashtra"), ("am", "ማሃራሽትራ"), ("ar", "ماهاراشترا"), ("as", "মহ\u{9be}ৰ\u{9be}ষ\u{9cd}ট\u{9cd}ৰ"), ("az", "Maxaraştra"), ("be", "Махараштра"), ("bg", "Махаращра"), ("bn", "মহ\u{9be}র\u{9be}ষ\u{9cd}ট\u{9cd}র"), ("ca", "Maharashtra"), ("ccp", "𑄟\u{11127}𑄦𑄢𑄌\u{11134}𑄖\u{11133}𑄢\u{11127}"), ("ceb", "State of Mahārāshtra"), ("cs", "Maháráštra"), ("cy", "Maharashtra"), ("da", "Maharashtra"), ("de", "Maharashtra"), ("el", "Μαχαράστρα"), ("en", "Maharashtra"), ("es", "Maharastra"), ("et", "Mahārāshtra"), ("eu", "Maharashtra"), ("fa", "مهاراشترا"), ("fi", "Maharashtra"), ("fr", "Maharashtra"), ("ga", "Maharashtra"), ("gu", "મહારાષ\u{acd}ટ\u{acd}ર"), ("he", "מהאראשטרה"), ("hi", "महाराष\u{94d}ट\u{94d}र"), ("hr", "Maharashtra"), ("hu", "Mahárástra"), ("hy", "Մահարաշտրա"), ("id", "Maharashtra"), ("is", "Maharashtra"), ("it", "Maharashtra"), ("ja", "マハーラーシュトラ州"), ("ka", "მაჰარაშტრა"), ("kk", "Махараштра"), ("kn", "ಮಹಾರಾಷ\u{ccd}ಟ\u{ccd}ರ"), ("ko", "마하라슈트라 주"), ("ky", "Махараштра"), ("lt", "Maharaštra"), ("lv", "Mahārāštra"), ("mk", "Махараштра"), ("ml", "മഹ\u{d3e}ര\u{d3e}ഷ\u{d4d}ട\u{d4d}ര"), ("mn", "Махараштра"), ("mr", "महाराष\u{94d}ट\u{94d}र"), ("ms", "Maharashtra"), ("nb", "Maharashtra"), ("ne", "महाराष\u{94d}ट\u{94d}र"), ("nl", "Maharashtra"), ("no", "Maharashtra"), ("or", "ମହ\u{b3e}ର\u{b3e}ଷ\u{b4d}ଟ\u{b4d}ର"), ("pa", "ਮਹਾਰਾਸ\u{a3c}ਟਰ"), ("pl", "Maharasztra"), ("ps", "مهاراشترا"), ("pt", "Maharashtra"), ("ro", "Maharashtra"), ("ru", "Махараштра"), ("si", "මහ\u{dcf}ර\u{dcf}ෂ\u{dca}ට\u{dca}\u{200d}ර\u{dcf}"), ("sk", "Maháraštra"), ("sl", "Maharaštra"), ("sr", "Махараштра"), ("sr_Latn", "Maharaštra"), ("sv", "Maharashtra"), ("sw", "Maharashtra"), ("ta", "மக\u{bbe}ர\u{bbe}ட\u{bcd}டிரம\u{bcd}"), ("te", "మహ\u{c3e}ర\u{c3e}ష\u{c4d}ట\u{c4d}ర"), ("th", "ร\u{e31}ฐมหาราษฏระ"), ("tr", "Maharaştra"), ("uk", "Махараштра"), ("ur", "مہاراشٹر"), ("uz", "Maharashtra"), ("vi", "Maharashtra"), ("yo", "Maharashtra"), ("yo_BJ", "Maharashtra"), ("yue", "馬哈拉施特拉邦"), ("yue_Hans", "马哈拉施特拉邦"), ("zh", "马哈拉施特拉邦")]),
                        unofficial_name_list: ["Maharashtra"].to_vec(),
                    }
                ),
                (
                    "ML",
                    Subdivision{
                        name: "Meghalaya",
                        country_alpha2: Alpha2::IN,
                        code: "ML",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.4670308), longitude: Some(91.366216), max_latitude: Some(26.1204059), min_latitude: Some(25.0333579), max_longitude: Some(92.80226599999999), min_longitude: Some(89.815674)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميغالايا"), ("as", "মেঘ\u{9be}লয\u{9bc}"), ("be", "Мегхалая"), ("bg", "Мегхалая"), ("bn", "মেঘ\u{9be}লয\u{9bc}"), ("ca", "Meghalaya"), ("ccp", "𑄟𑄬𑄊𑄣\u{11127}𑄠\u{11134}"), ("ceb", "Meghālaya"), ("cs", "Méghálaja"), ("cy", "Meghalaya"), ("da", "Meghalaya"), ("de", "Meghalaya"), ("el", "Μεγκαλάγια"), ("en", "Meghalaya"), ("es", "Megalaya"), ("et", "Meghālaya"), ("eu", "Meghalaya"), ("fa", "مگالایا"), ("fi", "Meghalaya"), ("fr", "Meghalaya"), ("ga", "Meghalaya"), ("gu", "મ\u{ac7}ઘાલય"), ("he", "מגהלאיה"), ("hi", "म\u{947}घालय"), ("hr", "Meghalaya"), ("hu", "Meghálaja"), ("hy", "Մեգհալայա"), ("id", "Meghalaya"), ("is", "Meghalaya"), ("it", "Meghalaya"), ("ja", "メーガーラヤ州"), ("ka", "მეგჰალაია"), ("kn", "ಮ\u{cc6}ಘಾಲಯ"), ("ko", "메갈라야 주"), ("lt", "Meghalaja"), ("lv", "Meghālaja"), ("mk", "Мегхалаја"), ("ml", "മേഘ\u{d3e}ലയ"), ("mn", "Мегалая"), ("mr", "म\u{947}घालय"), ("ms", "Meghalaya"), ("nb", "Meghalaya"), ("ne", "म\u{947}घालय"), ("nl", "Meghalaya"), ("no", "Meghalaya"), ("or", "ମେଘ\u{b3e}ଳୟ"), ("pa", "ਮ\u{a47}ਘਾਲਿਆ"), ("pl", "Meghalaya"), ("ps", "ميگالايا"), ("pt", "Meghalaya"), ("ro", "Meghalaya"), ("ru", "Мегалая"), ("si", "මෙඝ\u{dcf}ලය\u{dcf}"), ("sk", "Meghálaj"), ("sr", "Мегхалаја"), ("sr_Latn", "Meghalaja"), ("sv", "Meghalaya"), ("sw", "Meghalaya"), ("ta", "மேக\u{bbe}லய\u{bbe}"), ("te", "మ\u{c47}ఘ\u{c3e}లయ"), ("th", "ร\u{e31}ฐเมฆาล\u{e31}ย"), ("tr", "Meghalaya"), ("uk", "Меґхалая"), ("ur", "میگھالیہ"), ("uz", "Meghalaya"), ("vi", "Meghalaya"), ("yo", "Meghalaya"), ("yo_BJ", "Meghalaya"), ("yue", "梅加拉亞邦"), ("yue_Hans", "梅加拉亚邦"), ("zh", "梅加拉亚邦")]),
                        unofficial_name_list: ["Meghalaya"].to_vec(),
                    }
                ),
                (
                    "MN",
                    Subdivision{
                        name: "Manipur",
                        country_alpha2: Alpha2::IN,
                        code: "MN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(24.6637173), longitude: Some(93.90626879999999), max_latitude: Some(25.691874), min_latitude: Some(23.8360479), max_longitude: Some(94.74324), min_longitude: Some(92.97107799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Manipoer"), ("am", "መኒፑር"), ("ar", "مانيبور"), ("as", "মণিপ\u{9c1}ৰ"), ("az", "Manipur"), ("be", "Маніпур"), ("bg", "Манипур"), ("bn", "মণিপ\u{9c1}র"), ("ca", "Manipur"), ("ccp", "𑄟\u{11127}𑄚\u{11128}𑄛\u{1112a}𑄢\u{11134}"), ("ceb", "Manipur"), ("cs", "Manípur"), ("cy", "Manipur"), ("da", "Manipur"), ("de", "Manipur"), ("el", "Μανιπούρ"), ("en", "Manipur"), ("es", "Manipur"), ("et", "Manipur"), ("eu", "Manipur"), ("fa", "مانیپور"), ("fi", "Manipur"), ("fr", "Manipur"), ("ga", "Manipur"), ("gu", "મણિપ\u{ac1}ર"), ("he", "מניפור"), ("hi", "मणिप\u{941}र"), ("hr", "Manipur"), ("hu", "Manipur"), ("hy", "Մանիպուր"), ("id", "Manipur"), ("is", "Manipur"), ("it", "Manipur"), ("ja", "マニプル州"), ("ka", "მანიპური"), ("kn", "ಮಣ\u{cbf}ಪುರ"), ("ko", "마니푸르 주"), ("lt", "Manipuras"), ("lv", "Manipura"), ("mk", "Манипур"), ("ml", "മണിപ\u{d4d}പ\u{d42}ർ"), ("mn", "Манипур"), ("mr", "मणिप\u{942}र"), ("ms", "Manipur"), ("my", "မဏ\u{102d}ပ\u{1030}ရပြည\u{103a}နယ\u{103a}"), ("nb", "Manipur"), ("ne", "मणिप\u{941}र"), ("nl", "Manipur"), ("no", "Manipur"), ("or", "ମଣ\u{b3f}ପ\u{b41}ର"), ("pa", "ਮਨੀਪ\u{a41}ਰ"), ("pl", "Manipur"), ("ps", "منيپور"), ("pt", "Manipur"), ("ro", "Manipur"), ("ru", "Манипур"), ("si", "මන\u{dd2}ප\u{dd6}ර\u{dca}"), ("sk", "Manípur"), ("sl", "Manipur"), ("sr", "Манипур"), ("sr_Latn", "Manipur"), ("sv", "Manipur"), ("sw", "Manipur"), ("ta", "மணிப\u{bcd}பூர\u{bcd}"), ("te", "మణ\u{c3f}పూర\u{c4d}"), ("th", "ร\u{e31}ฐมณ\u{e35}ป\u{e38}ระ"), ("tr", "Manipur"), ("uk", "Маніпур"), ("ur", "منی پور"), ("uz", "Manipur"), ("vi", "Manipur"), ("yo", "Manipur"), ("yo_BJ", "Manipur"), ("yue", "曼尼普爾邦"), ("yue_Hans", "曼尼普尔邦"), ("zh", "曼尼普尔邦")]),
                        unofficial_name_list: ["Manipur"].to_vec(),
                    }
                ),
                (
                    "MP",
                    Subdivision{
                        name: "Madhya Pradesh",
                        country_alpha2: Alpha2::IN,
                        code: "MP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.9734229), longitude: Some(78.6568942), max_latitude: Some(26.8681089), min_latitude: Some(21.0799139), max_longitude: Some(82.809674), min_longitude: Some(74.0362481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ማድየ ፕረዴሽ"), ("ar", "ماديا براديش"), ("as", "মধ\u{9cd}য প\u{9cd}ৰদেশ"), ("bg", "Мадхя Прадеш"), ("bn", "মধ\u{9cd}যপ\u{9cd}রদেশ"), ("ca", "Madhya Pradesh"), ("ccp", "𑄟\u{11127}𑄖\u{11134}𑄙\u{11133}𑄠\u{11127} 𑄛\u{11133}𑄢\u{11127}𑄘𑄬𑄌\u{11134}"), ("ceb", "Madhya Pradesh"), ("cs", "Madhjapradéš"), ("cy", "Madhya Pradesh"), ("da", "Madhya Pradesh"), ("de", "Madhya Pradesh"), ("el", "Μάντια Πραντές"), ("en", "Madhya Pradesh"), ("es", "Madhya Pradesh"), ("et", "Madhya Pradesh"), ("eu", "Madhya Pradesh"), ("fa", "مادایا پرادش"), ("fi", "Madhya Pradesh"), ("fr", "Madhya Pradesh"), ("ga", "Madhya Pradesh"), ("gu", "મધ\u{acd}ય પ\u{acd}રદ\u{ac7}શ"), ("he", "מאדהיה פרדש"), ("hi", "मध\u{94d}य प\u{94d}रद\u{947}श"), ("hr", "Madhya Pradesh"), ("hu", "Madhja Prades"), ("hy", "Մադհյա Պրադեշ"), ("id", "Madhya Pradesh"), ("is", "Madhya Pradesh"), ("it", "Madhya Pradesh"), ("ja", "マディヤ・プラデーシュ州"), ("ka", "მადჰია-პრადეში"), ("kk", "Мадхья-Прадеш"), ("kn", "ಮಧ\u{ccd}ಯ ಪ\u{ccd}ರದೇಶ"), ("ko", "마디아프라데시 주"), ("ky", "Мадхья-Прадеш"), ("lt", "Madhja Pradešas"), ("lv", "Madhja Pradēša"), ("mk", "Мадја Прадеш"), ("ml", "മധ\u{d4d}യപ\u{d4d}രദേശ\u{d4d}\u{200c}"), ("mn", "Мадхья-Прадеш"), ("mr", "मध\u{94d}य प\u{94d}रद\u{947}श"), ("ms", "Madhya Pradesh"), ("my", "မဇ\u{1039}ဈဒေသ ပြည\u{103a}နယ\u{103a}"), ("nb", "Madhya Pradesh"), ("ne", "मध\u{94d}य प\u{94d}रद\u{947}श"), ("nl", "Madhya Pradesh"), ("no", "Madhya Pradesh"), ("or", "ମଧ\u{b4d}ୟ ପ\u{b4d}ରଦେଶ"), ("pa", "ਮ\u{a71}ਧ ਪ\u{a4d}ਰਦ\u{a47}ਸ\u{a3c}"), ("pl", "Madhya Pradesh"), ("ps", "مدهيه پرديش"), ("pt", "Madhya Pradesh"), ("ro", "Madhya Pradesh"), ("ru", "Мадхья-Прадеш"), ("sd", "مڌيا پرديش"), ("si", "මද\u{dca}\u{200d}ය ප\u{dca}\u{200d}රදේශ\u{dca}"), ("sk", "Madhjapradéš"), ("sl", "Umarkot"), ("sr", "Мадја Прадеш"), ("sr_Latn", "Madja Pradeš"), ("sv", "Madhya Pradesh"), ("sw", "Madhya Pradesh"), ("ta", "மத\u{bcd}தியப\u{bcd} பிரதேசம\u{bcd}"), ("te", "మధ\u{c4d}య ప\u{c4d}రద\u{c47}శ\u{c4d}"), ("th", "ร\u{e31}ฐม\u{e31}ธยประเทศ"), ("tr", "Madhya Pradesh"), ("uk", "Мадхʼя-Прадеш"), ("ur", "مدھیہ پردیش"), ("uz", "Madhya-pradesh"), ("vi", "Madhya Pradesh"), ("yo", "Madhya Pradesh"), ("yo_BJ", "Madhya Pradesh"), ("yue", "中央邦"), ("yue_Hans", "中央邦"), ("zh", "中央邦")]),
                        unofficial_name_list: ["Madhya Pradesh"].to_vec(),
                    }
                ),
                (
                    "MZ",
                    Subdivision{
                        name: "Mizoram",
                        country_alpha2: Alpha2::IN,
                        code: "MZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.164543), longitude: Some(92.9375739), max_latitude: Some(24.5174359), min_latitude: Some(21.946661), max_longitude: Some(93.4375611), min_longitude: Some(92.258479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ميزورام"), ("as", "মিজোৰ\u{9be}ম"), ("az", "Mizoram"), ("be", "Мізарам"), ("bg", "Мизорам"), ("bn", "মিজোর\u{9be}ম"), ("ca", "Mizoram"), ("ccp", "𑄟\u{11128}𑄎\u{1112e}𑄢𑄟\u{11134}"), ("ceb", "Mizoram"), ("cs", "Mizóram"), ("cy", "Mizoram"), ("da", "Mizoram"), ("de", "Mizoram"), ("el", "Μιζόραμ"), ("en", "Mizoram"), ("es", "Mizorán"), ("et", "Mizoram"), ("eu", "Mizoram"), ("fa", "میزورام"), ("fi", "Mizoram"), ("fr", "Mizoram"), ("ga", "Mizoram"), ("gu", "મિઝોરમ"), ("he", "מיזוראם"), ("hi", "मिज\u{93c}ोरम"), ("hr", "Mizoram"), ("hu", "Mizoram"), ("id", "Mizoram"), ("is", "Mizoram"), ("it", "Mizoram"), ("ja", "ミゾラム州"), ("ka", "მიზორამი"), ("kn", "ಮ\u{cbf}ಝೋರಂ"), ("ko", "미조람 주"), ("lt", "Mizoramas"), ("lv", "Mizorāma"), ("mk", "Мизорам"), ("ml", "മിസോറം"), ("mn", "Мизорам"), ("mr", "मिझोरम"), ("ms", "Mizoram"), ("nb", "Mizoram"), ("ne", "मिजोरम"), ("nl", "Mizoram"), ("no", "Mizoram"), ("or", "ମ\u{b3f}ଜୋର\u{b3e}ମ"), ("pa", "ਮਿਜ\u{a3c}\u{a4b}ਰਮ"), ("pl", "Mizoram"), ("ps", "ميزورام"), ("pt", "Mizoram"), ("ro", "Mizoram"), ("ru", "Мизорам"), ("si", "ම\u{dd2}සොරම\u{dca}"), ("sk", "Mizorám"), ("sq", "Mizoram"), ("sr", "Мизорам"), ("sr_Latn", "Mizoram"), ("sv", "Mizoram"), ("sw", "Mizoram"), ("ta", "மிசோரம\u{bcd}"), ("te", "మ\u{c3f}జ\u{c4b}ర\u{c3e}ం"), ("th", "ร\u{e31}ฐม\u{e34}โซร\u{e31}ม"), ("tr", "Mizoram"), ("uk", "Мізорам"), ("ur", "میزورم"), ("vi", "Mizoram"), ("yo", "Mizoram"), ("yo_BJ", "Mizoram"), ("yue", "米佐拉姆邦"), ("yue_Hans", "米佐拉姆邦"), ("zh", "米佐拉姆邦")]),
                        unofficial_name_list: ["Mizoram"].to_vec(),
                    }
                ),
                (
                    "NL",
                    Subdivision{
                        name: "Nagaland",
                        country_alpha2: Alpha2::IN,
                        code: "NL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.1584354), longitude: Some(94.5624426), max_latitude: Some(27.036123), min_latitude: Some(25.198068), max_longitude: Some(95.244715), min_longitude: Some(93.327578)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nagaland"), ("am", "ናገላንድ"), ("ar", "ناجالاند"), ("as", "ন\u{9be}গ\u{9be}লেণ\u{9cd}ড"), ("az", "Naqalend"), ("be", "Нагаленд"), ("bg", "Нагаланд"), ("bn", "ন\u{9be}গ\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("ca", "Nagaland"), ("ccp", "𑄚𑄉𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "State of Nāgāland"), ("cs", "Nágáland"), ("cy", "Nagaland"), ("da", "Nagaland"), ("de", "Nagaland"), ("el", "Ναγκαλάντ"), ("en", "Nagaland"), ("es", "Nagaland"), ("et", "Nagaland"), ("eu", "Nagaland"), ("fa", "ناگالند"), ("fi", "Nagaland"), ("fr", "Nagaland"), ("ga", "Nagaland"), ("gu", "નાગાલ\u{ac7}\u{a82}ડ"), ("he", "נאגאלנד"), ("hi", "नागाल\u{948}ण\u{94d}ड"), ("hr", "Nagaland"), ("hu", "Nágaföld"), ("id", "Nagaland"), ("is", "Nagaland"), ("it", "Nagaland"), ("ja", "ナガランド州"), ("ka", "ნაგალენდი"), ("kn", "ನಾಗಲ\u{ccd}ಯಂಡ\u{ccd}"), ("ko", "나갈랜드 주"), ("lt", "Nagalandas"), ("lv", "Nāgālenda"), ("mk", "Нагаланд"), ("ml", "ന\u{d3e}ഗ\u{d3e}ല\u{d3e}\u{200c}ൻഡ\u{d4d}"), ("mn", "Нагаланд"), ("mr", "नागाल\u{901}ड"), ("ms", "Nagaland"), ("nb", "Nagaland"), ("ne", "नागाल\u{94d}याण\u{94d}ड"), ("nl", "Nagaland"), ("no", "Nagaland"), ("or", "ନ\u{b3e}ଗ\u{b3e}ଲ\u{b3e}ଣ\u{b4d}ଡ"), ("pa", "ਨਾਗਾਲ\u{a48}\u{a02}ਡ"), ("pl", "Nagaland"), ("ps", "ناگالېډ"), ("pt", "Nagaland"), ("ro", "Nagaland"), ("ru", "Нагаленд"), ("si", "න\u{dcf}ගලන\u{dca}තය"), ("sk", "Nágsko"), ("sq", "Nagaland"), ("sr", "Нагаланд"), ("sr_Latn", "Nagaland"), ("sv", "Nagaland"), ("sw", "Nagaland"), ("ta", "ந\u{bbe}க\u{bbe}ல\u{bbe}ந\u{bcd}து"), ("te", "న\u{c3e}గ\u{c3e}ల\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐนาคาแลนด\u{e4c}"), ("tr", "Nagaland"), ("uk", "Нагаленд"), ("ur", "ناگالینڈ"), ("uz", "Nagalend"), ("vi", "Nagaland"), ("yo", "Nagaland"), ("yo_BJ", "Nagaland"), ("yue", "那加蘭邦"), ("yue_Hans", "那加兰邦"), ("zh", "那加兰邦")]),
                        unofficial_name_list: ["Nagaland"].to_vec(),
                    }
                ),
                (
                    "OR",
                    Subdivision{
                        name: "Orissa",
                        country_alpha2: Alpha2::IN,
                        code: "OR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(20.9516658), longitude: Some(85.0985236), max_latitude: Some(22.5700271), min_latitude: Some(17.8120961), max_longitude: Some(87.483385), min_longitude: Some(81.388607)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Odisha"), ("am", "ኦዲሣ"), ("ar", "أوريسا"), ("as", "উৰিষ\u{9cd}য\u{9be}"), ("az", "Odisha"), ("be", "Арыса"), ("bg", "Ориса"), ("bn", "ওড\u{9bc}িশ\u{9be}"), ("ca", "Odisha"), ("ccp", "𑄃\u{1112e}𑄢\u{11128}𑄥"), ("ceb", "State of Odisha"), ("cs", "Urísa"), ("cy", "Odisha"), ("da", "Odisha"), ("de", "Odisha"), ("el", "Ορίσα"), ("en", "Odisha"), ("es", "Odisha"), ("et", "Odisha"), ("eu", "Odisha"), ("fa", "اوریسا"), ("fi", "Odisha"), ("fr", "Odisha"), ("ga", "Odisha"), ("gl", "Odisha"), ("gu", "ઑડિશા"), ("he", "אודישה"), ("hi", "ओडिशा"), ("hr", "Odisha"), ("hu", "Orisza"), ("id", "Odisha"), ("is", "Odisha"), ("it", "Odisha"), ("ja", "オリッサ州"), ("jv", "Odisha"), ("ka", "ორისა"), ("kn", "ಒರ\u{cbf}ಸ\u{ccd}ಸಾ"), ("ko", "오디샤 주"), ("ky", "Орисса"), ("lt", "Orisa"), ("lv", "Orisa"), ("mk", "Одиша"), ("ml", "ഒഡീഷ"), ("mn", "Орисса"), ("mr", "ओडिशा"), ("ms", "Odisha"), ("my", "သြရ\u{102d}ဿပြည\u{103a}နယ\u{103a}"), ("nb", "Odisha"), ("ne", "उड\u{93c}िसा"), ("nl", "Odisha"), ("no", "Odisha"), ("or", "ଓଡ\u{b3c}\u{b3f}ଶ\u{b3e}"), ("pa", "ਓਡੀਸ\u{a3c}ਾ"), ("pl", "Orisa"), ("ps", "اوړېشه"), ("pt", "Odisha"), ("ro", "Odisha"), ("ru", "Орисса"), ("si", "ඔර\u{dd2}ස\u{dca}ස\u{dcf}"), ("sk", "Urísa"), ("sq", "Odisha"), ("sr", "Ориса"), ("sr_Latn", "Orisa"), ("sv", "Odisha"), ("sw", "Odisha"), ("ta", "ஒடிச\u{bbe}"), ("te", "ఒర\u{c3f}స\u{c4d}స\u{c3e}"), ("th", "ร\u{e31}ฐโอร\u{e34}ศา"), ("tr", "Odisha"), ("uk", "Орісса"), ("ur", "اڑیسہ"), ("uz", "Odisha"), ("vi", "Odisha"), ("yo", "Odisha"), ("yo_BJ", "Odisha"), ("yue", "奧里薩邦"), ("yue_Hans", "奥里萨邦"), ("zh", "奥里萨邦")]),
                        unofficial_name_list: ["Orissa"].to_vec(),
                    }
                ),
                (
                    "PB",
                    Subdivision{
                        name: "Punjab",
                        country_alpha2: Alpha2::IN,
                        code: "PB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.1471305), longitude: Some(75.34121789999999), max_latitude: Some(32.4981352), min_latitude: Some(29.537147), max_longitude: Some(76.92175809999999), min_longitude: Some(73.8708879)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ፐንጃብ፣ ሕንድ"), ("ar", "بنجاب"), ("as", "পঞ\u{9cd}জ\u{9be}ব, ভ\u{9be}ৰত"), ("az", "Pəncab ştatı"), ("be", "Пенджаб, Індыя"), ("bg", "Пенджаб"), ("bn", "প\u{9be}ঞ\u{9cd}জ\u{9be}ব, ভ\u{9be}রত"), ("ca", "Panjab"), ("ccp", "𑄛𑄚\u{11134}𑄎𑄛\u{11134}"), ("ceb", "State of Punjab"), ("cs", "Paňdžáb"), ("cy", "Punjab"), ("da", "Punjab"), ("de", "Punjab"), ("el", "Παντζάμπ"), ("en", "Punjab"), ("es", "Punyab"), ("et", "Pandžab"), ("eu", "Punjab"), ("fa", "پنجاب (هند)"), ("fi", "Punjab"), ("fr", "Pendjab"), ("gu", "પ\u{a82}જાબ, ભારત"), ("he", "פנג׳אב"), ("hi", "प\u{902}जाब"), ("hr", "Punjab"), ("hu", "Pandzsáb"), ("hy", "Փենջաբ"), ("id", "Punjab"), ("is", "Púnjab"), ("it", "Punjab"), ("ja", "パンジャーブ州"), ("ka", "პენჯაბი"), ("kn", "ಪಂಜಾಬ\u{ccd}"), ("ko", "펀자브 주"), ("lt", "Pandžabas"), ("lv", "Pendžāba"), ("mk", "Пенџаб"), ("ml", "പഞ\u{d4d}ച\u{d3e}ബ\u{d4d}"), ("mn", "Панжаб"), ("mr", "प\u{902}जाब"), ("ms", "Punjab"), ("nb", "Punjab"), ("ne", "पञ\u{94d}जाब"), ("nl", "Punjab"), ("no", "Punjab"), ("or", "ପଞ\u{b4d}ଜ\u{b3e}ବ"), ("pa", "ਪ\u{a70}ਜਾਬ"), ("pl", "Pendżab"), ("ps", "پنجاب"), ("pt", "Punjab"), ("ro", "Punjab"), ("ru", "Пенджаб"), ("sk", "Pandžáb"), ("sl", "Punjab"), ("sr", "Панџаб"), ("sr_Latn", "Pandžab"), ("sv", "Punjab"), ("sw", "Punjab"), ("ta", "பஞ\u{bcd}ச\u{bbe}ப\u{bcd}"), ("te", "పంజ\u{c3e}బ\u{c4d}"), ("th", "ร\u{e31}ฐป\u{e31}ญจาบ"), ("tr", "Pencap"), ("uk", "Пенджаб"), ("ur", "پنجاب"), ("uz", "Panjob"), ("vi", "Punjab"), ("yo", "Punjab"), ("yo_BJ", "Punjab"), ("yue", "旁遮普邦"), ("yue_Hans", "旁遮普邦"), ("zh", "旁遮普邦")]),
                        unofficial_name_list: ["Punjab"].to_vec(),
                    }
                ),
                (
                    "PY",
                    Subdivision{
                        name: "Pondicherry",
                        country_alpha2: Alpha2::IN,
                        code: "PY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.9138598), longitude: Some(79.8144722), max_latitude: Some(11.973176), min_latitude: Some(11.8994375), max_longitude: Some(79.84057229999999), min_longitude: Some(79.7857511)}),
                        comments: None,
                        subdivision_type: SubdivisionType::UnionTerritory,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بودوتشيري"), ("be", "Пандышэры"), ("bg", "Пондичери"), ("bn", "প\u{9c1}দ\u{9c1}চেরি"), ("ca", "Pondicherry"), ("ccp", "𑄛\u{1112a}𑄘\u{1112a}𑄌𑄬𑄢\u{11128}"), ("ceb", "Puducherry"), ("cs", "Puduččéri"), ("cy", "Puducherry"), ("da", "Pondicherry"), ("de", "Puducherry"), ("el", "Ποντιτσερί"), ("en", "Puducherry"), ("es", "Puducherry"), ("et", "Puducherry liiduterritoorium"), ("eu", "Puducherry"), ("fa", "پودوچری"), ("fi", "Puducherry"), ("fr", "Territoire de Pondichéry"), ("gu", "પૉ\u{a82}ડિચ\u{ac7}રી"), ("he", "פודוצ׳רי"), ("hi", "प\u{941}द\u{941}च\u{94d}च\u{947}री"), ("hr", "Pondicherry"), ("hu", "Puduccseri"), ("id", "Puducherry"), ("is", "Puducherry"), ("it", "Pondicherry"), ("ja", "ポンディシェリ連邦直轄領"), ("ka", "პონდიშერი"), ("kn", "ಪುದುಚೇರ\u{cbf}"), ("ko", "푸두체리"), ("lt", "Pudučeris"), ("lv", "Pudučerri"), ("mk", "Пондишери"), ("ml", "പ\u{d41}ത\u{d41}ച\u{d4d}ചേരി"), ("mr", "प\u{941}ड\u{941}च\u{947}री"), ("ms", "Puducherry"), ("nb", "Puducherry"), ("ne", "प\u{941}द\u{941}च\u{947}री"), ("nl", "Puducherry"), ("no", "Puducherry"), ("or", "ପ\u{b41}ଡ\u{b41}ଚେରୀ"), ("pa", "ਪਾ\u{a02}ਡੀਚਰੀ"), ("pl", "Puducherry"), ("pt", "Puducherry"), ("ro", "Puducherry"), ("ru", "Пондичерри"), ("si", "ප\u{dd4}ද\u{dd4}චේර\u{dd2}"), ("sk", "Puttučéri"), ("so", "Puducherry"), ("sr", "Територија Пондишери"), ("sr_Latn", "Teritorija Pondišeri"), ("sv", "Pondicherry"), ("sw", "Puducherry"), ("ta", "புதுச\u{bcd}சேரி"), ("te", "పుదుచ\u{c4d}చ\u{c47}ర\u{c3f}"), ("th", "ป\u{e38}ท\u{e38}จเจร\u{e35}"), ("tk", "Puducherry"), ("tr", "Puduçeri"), ("uk", "Пудучеррі"), ("ur", "پونڈیچری"), ("vi", "Puducherry"), ("yo", "Puducherry"), ("yo_BJ", "Puducherry"), ("yue", "本地治里"), ("yue_Hans", "本地治里"), ("zh", "本地治里")]),
                        unofficial_name_list: ["Pondicherry"].to_vec(),
                    }
                ),
                (
                    "RJ",
                    Subdivision{
                        name: "Rajasthan",
                        country_alpha2: Alpha2::IN,
                        code: "RJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.0238036), longitude: Some(74.21793260000001), max_latitude: Some(30.195124), min_latitude: Some(23.0632669), max_longitude: Some(78.26338109999999), min_longitude: Some(69.483734)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ራጀስጣን"), ("ar", "راجستان"), ("as", "ৰ\u{9be}জস\u{9cd}থ\u{9be}ন"), ("az", "Racastan"), ("be", "Раджастхан"), ("bg", "Раджастан"), ("bn", "র\u{9be}জস\u{9cd}থ\u{9be}ন"), ("ca", "Rajasthan"), ("ccp", "𑄢𑄌\u{11134}𑄌\u{11133}𑄒𑄚\u{11134}"), ("ceb", "State of Rājasthān"), ("cs", "Rádžasthán"), ("cy", "Rajasthan"), ("da", "Rajasthan"), ("de", "Rajasthan"), ("el", "Ράτζασταν"), ("en", "Rajasthan"), ("es", "Rajastán"), ("et", "Rājasthān"), ("eu", "Rajasthan"), ("fa", "راجستان"), ("fi", "Rajasthan"), ("fr", "Rajasthan"), ("ga", "Rajasthan"), ("gu", "રાજસ\u{acd}થાન"), ("he", "ראג׳סטאן"), ("hi", "राजस\u{94d}थान"), ("hr", "Radžastan"), ("hu", "Rádzsasztán"), ("hy", "Ռաջասթան"), ("id", "Rajasthan"), ("is", "Rajasthan"), ("it", "Rajasthan"), ("ja", "ラージャスターン州"), ("ka", "რაჯასტანი"), ("kk", "Раджастхан"), ("kn", "ರಾಜಸ\u{ccd}ಥಾನ"), ("ko", "라자스탄 주"), ("lt", "Radžastanas"), ("lv", "Rādžastāna"), ("mk", "Раџастан"), ("ml", "ര\u{d3e}ജസ\u{d4d}ഥ\u{d3e}ൻ"), ("mn", "Ражастан"), ("mr", "राजस\u{94d}थान"), ("ms", "Rajasthan"), ("my", "ရာဇဌာန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Rajasthan"), ("ne", "राजस\u{94d}थान"), ("nl", "Rajasthan"), ("no", "Rajasthan"), ("or", "ର\u{b3e}ଜସ\u{b4d}ଥ\u{b3e}ନ"), ("pa", "ਰਾਜਸਥਾਨ"), ("pl", "Radżastan"), ("ps", "راجستهان"), ("pt", "Rajastão"), ("ro", "Rajasthan"), ("ru", "Раджастхан"), ("sd", "راجستان"), ("si", "ර\u{dcf}ජස\u{dca}ත\u{dcf}න\u{dca}"), ("sk", "Radžastan"), ("sl", "Radžastan"), ("sr", "Раџастан"), ("sr_Latn", "Radžastan"), ("sv", "Rajasthan"), ("sw", "Rajasthan"), ("ta", "இர\u{bbe}ச\u{bcd}சசுத\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "ర\u{c3e}జస\u{c4d}థ\u{c3e}న\u{c4d}"), ("th", "ร\u{e31}ฐราชสถาน"), ("tr", "Racasthan"), ("uk", "Раджастхан"), ("ur", "راجستھان"), ("uz", "Rojasthon"), ("vi", "Rajasthan"), ("yo", "Rajasthan"), ("yo_BJ", "Rajasthan"), ("yue", "拉賈斯坦邦"), ("yue_Hans", "拉贾斯坦邦"), ("zh", "拉贾斯坦邦")]),
                        unofficial_name_list: ["Rajasthan"].to_vec(),
                    }
                ),
                (
                    "SK",
                    Subdivision{
                        name: "Sikkim",
                        country_alpha2: Alpha2::IN,
                        code: "SK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.5329718), longitude: Some(88.5122178), max_latitude: Some(28.128759), min_latitude: Some(27.079261), max_longitude: Some(88.9108059), min_longitude: Some(88.0063541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sikkim"), ("am", "ሲኪም"), ("ar", "سيكيم"), ("as", "ছিক\u{9cd}কিম"), ("az", "Sikkim"), ("be", "Сікім"), ("bg", "Сиким"), ("bn", "সিক\u{9cd}কিম"), ("bs", "Sikkim"), ("ca", "Sikkim"), ("ccp", "𑄥\u{11128}𑄇\u{11134}𑄇\u{11128}𑄟\u{11134}"), ("ceb", "Sikkim"), ("cs", "Sikkim"), ("cy", "Sikkim"), ("da", "Sikkim"), ("de", "Sikkim"), ("el", "Σικκίμ"), ("en", "Sikkim"), ("es", "Sikkim"), ("et", "Sikkim"), ("eu", "Sikkim"), ("fa", "سیکیم"), ("fi", "Sikkim"), ("fr", "Sikkim"), ("ga", "Sikkim"), ("gl", "Sikkim"), ("gu", "સિક\u{acd}કિમ"), ("he", "סיקים"), ("hi", "सिक\u{94d}किम"), ("hr", "Sikkim"), ("hu", "Szikkim"), ("id", "Sikkim"), ("is", "Sikkim"), ("it", "Sikkim"), ("ja", "シッキム州"), ("ka", "სიკიმი"), ("kn", "ಸ\u{cbf}ಕ\u{ccd}ಕ\u{cbf}ಂ"), ("ko", "시킴 주"), ("lt", "Sikimas"), ("lv", "Sikima"), ("mk", "Сиким"), ("ml", "സിക\u{d4d}കിം"), ("mn", "Сикким"), ("mr", "सिक\u{94d}किम"), ("ms", "Sikkim"), ("my", "ဆစ\u{103a}ကင\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Sikkim"), ("ne", "सिक\u{94d}किम"), ("nl", "Sikkim"), ("no", "Sikkim"), ("or", "ସ\u{b3f}କ\u{b3f}ମ"), ("pa", "ਸਿ\u{a71}ਕਮ"), ("pl", "Sikkim"), ("ps", "سيکيم"), ("pt", "Siquim"), ("ro", "Sikkim"), ("ru", "Сикким"), ("si", "ස\u{dd2}ක\u{dd2}ම\u{dca}"), ("sk", "Sikkim"), ("sq", "Sikkim"), ("sr", "Сиким"), ("sr_Latn", "Sikim"), ("sv", "Sikkim"), ("sw", "Sikkim"), ("ta", "சிக\u{bcd}கிம\u{bcd}"), ("te", "స\u{c3f}క\u{c4d}క\u{c3f}ం"), ("th", "ร\u{e31}ฐส\u{e34}กข\u{e34}ม"), ("tr", "Sikkim"), ("uk", "Сіккім"), ("ur", "سکم"), ("vi", "Sikkim"), ("yo", "Sikkim"), ("yo_BJ", "Sikkim"), ("yue", "錫金"), ("yue_Hans", "锡金"), ("zh", "锡金邦")]),
                        unofficial_name_list: ["Denjong"].to_vec(),
                    }
                ),
                (
                    "TG",
                    Subdivision{
                        name: "त\u{947}ल\u{902}गाना",
                        country_alpha2: Alpha2::IN,
                        code: "TG",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تيلانغانا"), ("as", "তেলেংগ\u{9be}ন\u{9be}"), ("az", "Telanqana"), ("bg", "Телангана"), ("bn", "তেলঙ\u{9cd}গ\u{9be}ন\u{9be}"), ("ca", "Telangana"), ("ccp", "𑄑𑄬𑄣𑄚\u{11134}𑄉𑄚"), ("ceb", "Telangana"), ("cs", "Telangána"), ("cy", "Telangana"), ("da", "Telangana"), ("de", "Telangana"), ("el", "Τελανγκάνα"), ("en", "Telangana"), ("es", "Telingana"), ("et", "Telangana"), ("fa", "تلنگانا"), ("fi", "Telangana"), ("fr", "Telangana"), ("ga", "Telangana"), ("gl", "Telangana"), ("gu", "ત\u{ac7}લ\u{a82}ગાણા"), ("he", "טלנגאנה"), ("hi", "त\u{947}ल\u{902}गाना"), ("hu", "Telangána"), ("id", "Telangana"), ("it", "Telangana"), ("ja", "テランガナ"), ("ka", "ტელანგანა"), ("kn", "ತ\u{cc6}ಲಂಗಾಣ"), ("ko", "텔랑가나 주"), ("lt", "Telangana"), ("lv", "Telangāna"), ("mk", "Телангана"), ("ml", "തെലങ\u{d4d}ക\u{d3e}ന"), ("mn", "Телангана"), ("mr", "त\u{947}ल\u{902}गणा"), ("ms", "Telangana"), ("nb", "Telangana"), ("ne", "त\u{947}ल\u{902}गाना"), ("nl", "Telangana"), ("no", "Telangana"), ("or", "ତେଲେଙ\u{b4d}ଗ\u{b3e}ନ\u{b3e}"), ("pa", "ਤ\u{a47}ਲ\u{a70}ਗਾਨਾ"), ("pl", "Telangana"), ("pt", "Telangana"), ("ro", "Telangana"), ("ru", "Телангана"), ("sr", "Телангана"), ("sr_Latn", "Telangana"), ("sv", "Telangana"), ("sw", "Telangana"), ("ta", "தெலுங\u{bcd}க\u{bbe}ன\u{bbe}"), ("te", "త\u{c46}లంగ\u{c3e}ణ"), ("th", "ร\u{e31}ฐเตล\u{e31}นกานา"), ("tr", "Telangana"), ("uk", "Телангана"), ("ur", "تیلنگانا"), ("vi", "Telangana"), ("yue", "泰倫加納邦"), ("yue_Hans", "泰伦加纳邦"), ("zh", "特伦甘纳邦")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TN",
                    Subdivision{
                        name: "Tamil Nadu",
                        country_alpha2: Alpha2::IN,
                        code: "TN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.1271225), longitude: Some(78.6568942), max_latitude: Some(13.496666), min_latitude: Some(8.0774077), max_longitude: Some(80.3464511), min_longitude: Some(76.23055409999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tamil Nadu"), ("am", "ታሚል ናዱ"), ("ar", "تامل نادو"), ("as", "ত\u{9be}মিলন\u{9be}ড\u{9c1}"), ("az", "Tamilnad"), ("be", "Тамілнад"), ("bg", "Тамил Наду"), ("bn", "ত\u{9be}মিলন\u{9be}ড\u{9bc}\u{9c1}"), ("ca", "Tamil Nadu"), ("ccp", "𑄑𑄟\u{11128}𑄣\u{11134} 𑄚𑄓\u{1112a}"), ("ceb", "State of Tamil Nādu"), ("cs", "Tamilnádu"), ("cy", "Tamil Nadu"), ("da", "Tamil Nadu"), ("de", "Tamil Nadu"), ("el", "Ταμίλ Ναντού"), ("en", "Tamil Nadu"), ("es", "Tamil Nadu"), ("et", "Tamil Nadu"), ("eu", "Tamil Nadu"), ("fa", "تامیل نادو"), ("fi", "Tamil Nadu"), ("fr", "Tamil Nadu"), ("ga", "Tamil Nadu"), ("gl", "Tamil Nadu"), ("gu", "તમિલનાડ\u{ac1}"), ("he", "טאמיל נאדו"), ("hi", "तमिल नाड\u{941}"), ("hr", "Tamil Nadu"), ("hu", "Tamilnádu"), ("hy", "Տամիլանդ"), ("id", "Tamil Nadu"), ("is", "Tamil Nadu"), ("it", "Tamil Nadu"), ("ja", "タミル・ナードゥ州"), ("ka", "ტამილნადუ"), ("kk", "Тамилнад"), ("kn", "ತಮ\u{cbf}ಳುನಾಡು"), ("ko", "타밀나두 주"), ("lt", "Tamilnadas"), ("lv", "Tamilnāda"), ("mk", "Тамил Наду"), ("ml", "തമിഴ\u{d4d}\u{200c}ന\u{d3e}ട\u{d4d}"), ("mn", "Тамилнаду"), ("mr", "तमिळनाड\u{942}"), ("ms", "Tamil Nadu"), ("my", "မဒရပ\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Tamil Nadu"), ("ne", "तमिलनाड\u{941}"), ("nl", "Tamil Nadu"), ("no", "Tamil Nadu"), ("or", "ତ\u{b3e}ମ\u{b3f}ଲନ\u{b3e}ଡ\u{b41}"), ("pa", "ਤਾਮਿਲ ਨਾਡ\u{a42}"), ("pl", "Tamilnadu"), ("ps", "تامل ناډو"), ("pt", "Tamil Nadu"), ("ro", "Tamil Nadu"), ("ru", "Тамилнад"), ("si", "තම\u{dd2}ල\u{dca}න\u{dcf}ඩ\u{dd4}ව"), ("sk", "Tamilnádu"), ("sq", "Tamil Nadu"), ("sr", "Тамил Наду"), ("sr_Latn", "Tamil Nadu"), ("sv", "Tamil Nadu"), ("sw", "Tamil Nadu"), ("ta", "தமிழ\u{bcd}ந\u{bbe}டு"), ("te", "తమ\u{c3f}ళన\u{c3e}డు"), ("th", "ร\u{e31}ฐทม\u{e34}ฬนาฑ\u{e39}"), ("tr", "Tamil Nadu"), ("uk", "Тамілнаду"), ("ur", "تامل ناڈو"), ("uz", "Tamilnad"), ("vi", "Tamil Nadu"), ("yo", "Tamil Nadu"), ("yo_BJ", "Tamil Nadu"), ("yue", "淡米爾納德邦"), ("yue_Hans", "淡米尔纳德邦"), ("zh", "泰米尔纳德邦")]),
                        unofficial_name_list: ["Tamil Nadu"].to_vec(),
                    }
                ),
                (
                    "TR",
                    Subdivision{
                        name: "Tripura",
                        country_alpha2: Alpha2::IN,
                        code: "TR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(23.9408482), longitude: Some(91.9881527), max_latitude: Some(24.533733), min_latitude: Some(22.929054), max_longitude: Some(92.336001), min_longitude: Some(91.15093189999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tripoera"), ("am", "ትሪፑረ"), ("ar", "ترايبورا"), ("as", "ত\u{9cd}ৰিপ\u{9c1}ৰ\u{9be}"), ("az", "Tripura"), ("be", "Трыпура"), ("bg", "Трипура"), ("bn", "ত\u{9cd}রিপ\u{9c1}র\u{9be}"), ("ca", "Tripura"), ("ccp", "𑄑\u{11133}𑄢\u{11128}𑄛\u{1112a}𑄢"), ("ceb", "Tripura"), ("cs", "Tripura"), ("cy", "Tripura"), ("da", "Tripura"), ("de", "Tripura"), ("el", "Τρίπουρα"), ("en", "Tripura"), ("es", "Tripura"), ("et", "Tripura"), ("eu", "Tripura"), ("fa", "تریپورا"), ("fi", "Tripura"), ("fr", "Tripura"), ("ga", "Tripura"), ("gu", "ત\u{acd}રિપ\u{ac1}રા"), ("he", "טריפורה"), ("hi", "त\u{94d}रिप\u{941}रा"), ("hr", "Tripura"), ("hu", "Tripura"), ("hy", "Տրիպուրա"), ("id", "Tripura"), ("is", "Tripura"), ("it", "Tripura"), ("ja", "トリプラ州"), ("ka", "ტრიპურა"), ("kk", "Трипура"), ("kn", "ತ\u{ccd}ರ\u{cbf}ಪುರ"), ("ko", "트리푸라 주"), ("lt", "Tripura"), ("lv", "Tripura"), ("mk", "Трипура"), ("ml", "ത\u{d4d}രിപ\u{d41}ര"), ("mn", "Трипура"), ("mr", "त\u{94d}रिप\u{941}रा"), ("ms", "Tripura"), ("nb", "Tripura"), ("ne", "त\u{94d}रिप\u{941}रा"), ("nl", "Tripura"), ("no", "Tripura"), ("or", "ତ\u{b4d}ର\u{b3f}ପ\u{b41}ର\u{b3e}"), ("pa", "ਤ\u{a4d}ਰਿਪ\u{a41}ਰਾ"), ("pl", "Tripura"), ("ps", "تريپوره"), ("pt", "Tripura"), ("ro", "Tripura"), ("ru", "Трипура"), ("si", "ට\u{dca}\u{200d}ර\u{dd2}ප\u{dd4}ර"), ("sk", "Tripura"), ("sq", "Tripura"), ("sr", "Трипура"), ("sr_Latn", "Tripura"), ("sv", "Tripura"), ("sw", "Tripura"), ("ta", "திரிபுர\u{bbe}"), ("te", "త\u{c4d}ర\u{c3f}పుర"), ("th", "ร\u{e31}ฐตร\u{e34}ป\u{e38}ระ"), ("tr", "Tripura"), ("uk", "Тріпура"), ("ur", "تری پورہ"), ("uz", "Tripura"), ("vi", "Tripura"), ("yo", "Tripura"), ("yo_BJ", "Tripura"), ("yue", "特里普拉邦"), ("yue_Hans", "特里普拉邦"), ("zh", "特里普拉邦")]),
                        unofficial_name_list: ["Tripura"].to_vec(),
                    }
                ),
                (
                    "UP",
                    Subdivision{
                        name: "Uttar Pradesh",
                        country_alpha2: Alpha2::IN,
                        code: "UP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.5705886), longitude: Some(80.0981869), max_latitude: Some(30.411635), min_latitude: Some(23.870839), max_longitude: Some(84.6743269), min_longitude: Some(77.0924369)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oetarpradesj"), ("am", "ኡተር ፕረዴሽ"), ("ar", "أتر برديش"), ("as", "উত\u{9cd}তৰ প\u{9cd}ৰদেশ"), ("az", "Uttar Pradeş"), ("be", "Утар-Прадэш"), ("bg", "Утар Прадеш"), ("bn", "উত\u{9cd}তরপ\u{9cd}রদেশ"), ("ca", "Uttar Pradesh"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄘𑄬𑄌\u{11134}"), ("ceb", "Uttar Pradesh"), ("cs", "Uttarpradéš"), ("cy", "Uttar Pradesh"), ("da", "Uttar Pradesh"), ("de", "Uttar Pradesh"), ("el", "Ούταρ Πραντές"), ("en", "Uttar Pradesh"), ("es", "Uttar Pradesh"), ("et", "Uttar Pradesh"), ("eu", "Uttar Pradesh"), ("fa", "اوتار پرادش"), ("fi", "Uttar Pradesh"), ("fr", "Uttar Pradesh"), ("ga", "Uttar Pradesh"), ("gl", "Uttar Pradesh"), ("gu", "ઉત\u{acd}તર પ\u{acd}રદ\u{ac7}શ"), ("he", "אוטר פרדש"), ("hi", "उत\u{94d}तर प\u{94d}रद\u{947}श"), ("hr", "Uttar Pradesh"), ("hu", "Uttar Prades"), ("hy", "Ուտան Պրադեչ"), ("id", "Uttar Pradesh"), ("is", "Uttar Pradesh"), ("it", "Uttar Pradesh"), ("ja", "ウッタル・プラデーシュ州"), ("ka", "უტარ-პრადეში"), ("kn", "ಉತ\u{ccd}ತರ ಪ\u{ccd}ರದೇಶ"), ("ko", "우타르프라데시 주"), ("lt", "Utar Pradešas"), ("lv", "Utarpradēša"), ("mk", "Утар Прадеш"), ("ml", "ഉത\u{d4d}തർ\u{200c}പ\u{d4d}രദേശ\u{d4d}"), ("mn", "Уттар Прадеш"), ("mr", "उत\u{94d}तर प\u{94d}रद\u{947}श"), ("ms", "Uttar Pradesh"), ("my", "ဥတ\u{103a}တရဒသေ"), ("nb", "Uttar Pradesh"), ("ne", "उत\u{94d}तर प\u{94d}रद\u{947}श"), ("nl", "Uttar Pradesh"), ("no", "Uttar Pradesh"), ("or", "ଉତ\u{b4d}ତର ପ\u{b4d}ରଦେଶ"), ("pa", "ਉ\u{a71}ਤਰ ਪ\u{a4d}ਰਦ\u{a47}ਸ\u{a3c}"), ("pl", "Uttar Pradesh"), ("ps", "اتر پرديش"), ("pt", "Uttar Pradesh"), ("ro", "Uttar Pradesh"), ("ru", "Уттар-Прадеш"), ("si", "උත\u{dca}තර\u{dca} ප\u{dca}\u{200d}රදේශ\u{dca}"), ("sk", "Uttarpradéš"), ("sl", "Uttar Pradesh"), ("sq", "Uttar Pradesh"), ("sr", "Утар Прадеш"), ("sr_Latn", "Utar Pradeš"), ("sv", "Uttar Pradesh"), ("sw", "Uttar Pradesh"), ("ta", "உத\u{bcd}தரப\u{bcd} பிரதேசம\u{bcd}"), ("te", "ఉత\u{c4d}తర ప\u{c4d}రద\u{c47}శ\u{c4d}"), ("th", "ร\u{e31}ฐอ\u{e38}ตตรประเทศ"), ("tr", "Uttar Pradeş"), ("uk", "Уттар-Прадеш"), ("ur", "اتر پردیش"), ("uz", "Uttarpradesh"), ("vi", "Uttar Pradesh"), ("yo", "Uttar Pradesh"), ("yo_BJ", "Uttar Pradesh"), ("yue", "北方邦"), ("yue_Hans", "北方邦"), ("zh", "北方邦")]),
                        unofficial_name_list: ["Uttar Pradesh"].to_vec(),
                    }
                ),
                (
                    "UT",
                    Subdivision{
                        name: "उत\u{94d}तराखण\u{94d}ड",
                        country_alpha2: Alpha2::IN,
                        code: "UT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ኡተራኸንድ"), ("ar", "أوتاراخند"), ("as", "উত\u{9cd}তৰ\u{9be}খণ\u{9cd}ড"), ("be", "Утаракханд"), ("bg", "Утаракханд"), ("bn", "উত\u{9cd}তর\u{9be}খণ\u{9cd}ড"), ("ca", "Uttarakhand"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄃𑄈\u{11127}𑄚\u{11134}𑄓\u{11127}"), ("ceb", "State of Uttarakhand"), ("cs", "Uttarákhand"), ("cy", "Uttarakhand"), ("da", "Uttarakhand"), ("de", "Uttarakhand"), ("el", "Ουταράχαντ"), ("en", "Uttarakhand"), ("es", "Uttarakhand"), ("et", "Uttarakhand"), ("eu", "Uttarakhand"), ("fa", "اوتاراکند"), ("fi", "Uttarakhand"), ("fr", "Uttarakhand"), ("ga", "Uttarakhand"), ("gu", "ઉત\u{acd}તરાખ\u{a82}ડ"), ("he", "אוטראקהאנד"), ("hi", "उत\u{94d}तराखण\u{94d}ड"), ("hr", "Uttarakhand"), ("hu", "Uttarakhand"), ("id", "Uttarakhand"), ("is", "Uttarakhand"), ("it", "Uttarakhand"), ("ja", "ウッタラーカンド州"), ("ka", "უტარაკჰანდი"), ("kn", "ಉತ\u{ccd}ತರಾಖಂಡ"), ("ko", "우타라칸드 주"), ("lt", "Utarakhandas"), ("lv", "Utarakhanda"), ("mk", "Утараканд"), ("ml", "ഉത\u{d4d}തര\u{d3e}ഖണ\u{d4d}ഡ\u{d4d}"), ("mn", "Уттараканд"), ("mr", "उत\u{94d}तराख\u{902}ड"), ("ms", "Uttarakhand"), ("nb", "Uttarakhand"), ("ne", "उत\u{94d}तराखण\u{94d}ड"), ("nl", "Uttarakhand"), ("no", "Uttarakhand"), ("or", "ଉତ\u{b4d}ତର\u{b3e}ଖଣ\u{b4d}ଡ"), ("pa", "ਉ\u{a71}ਤਰਖ\u{a70}ਡ"), ("pl", "Uttarakhand"), ("ps", "اتر کنډ"), ("pt", "Uttarakhand"), ("ro", "Uttarakhand"), ("ru", "Уттаракханд"), ("si", "උට\u{dca}ටරඛ\u{dcf}න\u{dca}ඩ\u{dca}"), ("sk", "Uttarákhand"), ("sq", "Uttarakand"), ("sr", "Утараханд"), ("sr_Latn", "Utarahand"), ("sv", "Uttarakhand"), ("sw", "Uttarakhand"), ("ta", "உத\u{bcd}தர\u{bbe}கண\u{bcd}டம\u{bcd}"), ("te", "ఉత\u{c4d}తర\u{c3e}ఖండ\u{c4d}"), ("th", "ร\u{e31}ฐอ\u{e38}ตตราข\u{e31}ณฑ\u{e4c}"), ("tr", "Uttarakhand"), ("uk", "Уттаракханд"), ("ur", "اتراکھنڈ"), ("vi", "Uttarakhand"), ("yo", "Uttarakhand"), ("yo_BJ", "Uttarakhand"), ("yue", "北阿坎德邦"), ("yue_Hans", "北阿坎德邦"), ("zh", "北阿坎德邦")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "WB",
                    Subdivision{
                        name: "West Bengal",
                        country_alpha2: Alpha2::IN,
                        code: "WB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(22.9867569), longitude: Some(87.8549755), max_latitude: Some(27.220707), min_latitude: Some(21.528355), max_longitude: Some(89.880039), min_longitude: Some(85.82095799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Bengale"), ("am", "ምዕራብ ቤንጋል"), ("ar", "بنغال الغربية"), ("as", "পশ\u{9cd}চিমবঙ\u{9cd}গ"), ("be", "Заходняя Бенгалія"), ("bg", "Западна Бенгалия"), ("bn", "পশ\u{9cd}চিমবঙ\u{9cd}গ"), ("ca", "Bengala Occidental"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄌\u{11134} 𑄝𑄬\u{11101}𑄉\u{11127}𑄣\u{11134}"), ("ceb", "West Bengal"), ("cs", "Západní Bengálsko"), ("cy", "Gorllewin Bengal"), ("da", "Vestbengalen"), ("de", "Westbengalen"), ("el", "Δυτική Βενγκάλη"), ("en", "West Bengal"), ("es", "Bengala Occidental"), ("et", "Lääne-Bengali osariik"), ("eu", "Mendebaldeko Bengala"), ("fa", "بنگال غربی"), ("fi", "Länsi-Bengali"), ("fr", "Bengale-Occidental"), ("ga", "Beangál Thiar"), ("gl", "Bengala Occidental"), ("gu", "પશ\u{acd}ચિમ બ\u{a82}ગાળ"), ("he", "מערב בנגל"), ("hi", "पश\u{94d}चिम ब\u{902}गाल"), ("hr", "Zapadni Bengal"), ("hu", "Nyugat-Bengál"), ("hy", "Արևմտյան Բենգալիա"), ("id", "Benggala Barat"), ("is", "Vestur-Bengal"), ("it", "Bengala Occidentale"), ("ja", "西ベンガル州"), ("ka", "დასავლეთი ბენგალი"), ("kk", "Батыс Бенгал"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಬಂಗಾಳ"), ("ko", "서벵골 주"), ("lt", "Vakarų Bengalija"), ("lv", "Rietumbengāle"), ("mk", "Западен Бенгал"), ("ml", "പശ\u{d4d}ചിമ ബംഗ\u{d3e}ൾ"), ("mn", "Өрнө Бенгал"), ("mr", "पश\u{94d}चिम ब\u{902}गाल"), ("ms", "Bengal Barat"), ("nb", "Vest-Bengal"), ("ne", "पश\u{94d}चिम बङ\u{94d}गाल"), ("nl", "West-Bengalen"), ("no", "Vest-Bengal"), ("or", "ପଶ\u{b4d}ଚ\u{b3f}ମବଙ\u{b4d}ଗ"), ("pa", "ਪ\u{a71}ਛਮੀ ਬ\u{a70}ਗਾਲ"), ("pl", "Bengal Zachodni"), ("ps", "مغربي بنگال"), ("pt", "Bengala Ocidental"), ("ro", "Bengalul de Vest"), ("ru", "Западная Бенгалия"), ("sd", "اولھ بنگال"), ("si", "බටහ\u{dd2}ර බෙංග\u{dcf}ලය"), ("sk", "Západné Bengálsko"), ("sl", "Zahodna Bengalija"), ("sq", "Bengali Perëndimor"), ("sr", "Западни Бенгал"), ("sr_Latn", "Zapadni Bengal"), ("sv", "Västbengalen"), ("sw", "West Bengal"), ("ta", "மேற\u{bcd}கு வங\u{bcd}க\u{bbe}ளம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ బ\u{c46}ంగ\u{c3e}ల\u{c4d}"), ("th", "ร\u{e31}ฐเบงกอลตะว\u{e31}นตก"), ("tk", "West Bengal"), ("tr", "Batı Bengal"), ("uk", "Західний Бенгал"), ("ur", "مغربی بنگال"), ("uz", "Gʻarbiy bengaliya"), ("vi", "Tây Bengal"), ("yo", "Ìwọòrùn Bẹ\u{300}ngál"), ("yo_BJ", "Ìwɔòrùn Bɛ\u{300}ngál"), ("yue", "西孟加拉邦"), ("yue_Hans", "西孟加拉邦"), ("zh", "西孟加拉邦")]),
                        unofficial_name_list: ["West Bengal"].to_vec(),
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
#[cfg(feature = "in")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::IN,
        alpha3: Alpha3::IND,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{region}}\n{{city}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::Asia,
        country_code: 91,
        currency_code: CurrencyCode::INR,
        gec: Some(GEC::IN),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::IND),
        iso_long_name: "The Republic of India",
        iso_short_name: "India",
        official_language_list: ["en", "hi"].to_vec(),
        spoken_language_list: ["en", "hi"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "0",
        nationality: Some("Indian"),
        number: "356",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernAsia),
        un_locode: "IN",
        unofficial_name_list: ["India", "Indien", "Inde", "インド"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "India"),
            ("af", "Indië"),
            ("ak", "India"),
            ("am", "ህን፥"),
            ("an", "India"),
            ("ar", "الهند"),
            ("as", "ভ\u{9be}ৰত"),
            ("ay", "India"),
            ("az", "Hindistan"),
            ("ba", "India"),
            ("be", "Індыя"),
            ("bg", "Индия"),
            ("bi", "India"),
            ("bn", "ভ\u{9be}রত"),
            ("bn_IN", "ভ\u{9be}রত"),
            ("br", "India"),
            ("bs", "Indija"),
            ("ca", "Índia"),
            ("ce", "Инди"),
            ("ch", "India"),
            ("cs", "Indie"),
            ("cv", "Инди"),
            ("cy", "India"),
            ("da", "Indien"),
            ("de", "Indien"),
            ("dv", "އ\u{7a8}ނ\u{7b0}ޑ\u{7a8}ޔ\u{7a7}"),
            ("dz", "ར\u{f92}\u{fb1}་གར།"),
            ("ee", "India"),
            ("el", "Ινδία"),
            ("en", "India"),
            ("eo", "Barato"),
            ("es", "India"),
            ("et", "India"),
            ("eu", "India"),
            ("fa", "هندوستان"),
            ("ff", "India"),
            ("fi", "Intia"),
            ("fo", "India"),
            ("fr", "Inde"),
            ("fy", "Yndia"),
            ("ga", "An India"),
            ("gl", "A India"),
            ("gn", "India"),
            ("gu", "ભારત"),
            ("gv", "Yn Injey"),
            ("ha", "Indiya"),
            ("he", "הודו"),
            ("hi", "भारत"),
            ("hr", "Indija"),
            ("ht", "End"),
            ("hu", "India"),
            ("hy", "Հնդկաստան"),
            ("ia", "India"),
            ("id", "India"),
            ("io", "India"),
            ("is", "Indland"),
            ("it", "India"),
            ("iu", "ᐃᓐᑎᐊ"),
            ("ja", "インド"),
            ("ka", "ინდოეთი"),
            ("ki", "India"),
            ("kk", "Үндістан"),
            ("kl", "India"),
            ("km", "ឥណ\u{17d2}ឌា"),
            ("kn", "ಭಾರತ"),
            ("ko", "인도"),
            ("ku", "Hindistan"),
            ("kv", "Индия"),
            ("kw", "Eynda"),
            ("ky", "Индия"),
            ("lo", "India"),
            ("lt", "Indija"),
            ("lv", "Indija"),
            ("mi", "Īnia"),
            ("mk", "Индија"),
            ("ml", "ഇന\u{d4d}ത\u{d4d}യ"),
            ("mn", "Энэтхэг"),
            ("mr", "भारत"),
            ("ms", "Hindia"),
            ("mt", "Indja"),
            (
                "my",
                "အ\u{102d}န\u{1039}ဒ\u{102d}ယန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Indjiya"),
            ("nb", "India"),
            ("ne", "भारत"),
            ("nl", "India"),
            ("nn", "India"),
            (
                "nv",
                "Tó Wónaanídę\u{301}ę\u{301}ʼ Bitsįʼ Yishtłizhii Bikéyah",
            ),
            ("oc", "Índia"),
            ("or", "ଭ\u{b3e}ରତ"),
            ("pa", "ਭਾਰਤ"),
            ("pi", "भारत"),
            ("pl", "Indie"),
            ("ps", "هند"),
            ("pt", "Índia"),
            ("pt_BR", "Índia"),
            ("ro", "India"),
            ("ru", "Индия"),
            ("rw", "Ubuhinde"),
            ("sc", "Ìndia"),
            ("sd", "ڀارت"),
            ("si", "ඉන\u{dca}ද\u{dd2}ය\u{dcf}ව"),
            ("sk", "India"),
            ("sl", "Indija"),
            ("so", "Hindiya"),
            ("sq", "Indi"),
            ("sr", "Индија"),
            ("sv", "Indien"),
            ("sw", "India"),
            ("ta", "இந\u{bcd}திய\u{bbe}"),
            ("te", "భ\u{c3e}రతద\u{c47}శము"),
            ("tg", "Ҳиндустон"),
            ("th", "อ\u{e34}นเด\u{e35}ย"),
            ("ti", "ህንዲ"),
            ("tk", "Hindistan"),
            ("tl", "Indiya"),
            ("tr", "Hindistan"),
            ("tt", "Һиндстан"),
            ("ug", "ھىندىستان"),
            ("uk", "Індія"),
            ("ur", "بھارت"),
            ("uz", "Hindiston"),
            ("ve", "India"),
            ("vi", "Ấn-độ"),
            ("wa", "Inde"),
            ("wo", "Eend"),
            ("xh", "India"),
            ("yo", "Índíà"),
            ("zh_CN", "印度"),
            ("zh_HK", "印度"),
            ("zh_TW", "印度"),
            ("zu", "India"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
