// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The United States of America

#[cfg(all(feature = "us", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::US;
    pub const ALPHA3: Alpha3 = Alpha3::USA;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::USD;
    pub const GEC: Option<GEC> = Some(GEC::US);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::USA);
    pub const ISO_SHORT_NAME: &str = "United States of America";
    pub const ISO_LONG_NAME: &str = "The United States of America";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("American");
    pub const NUMBER: &str = "840";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("(\\d{5})(?:[ \\-](\\d{4}))?");
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernAmerica);
    pub const UN_LOCODE: &str = "US";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "United States",
        "USA",
        "Vereinigte Staaten von Amerika",
        "États-Unis",
        "Estados Unidos",
        "アメリカ合衆国",
        "Verenigde Staten",
        "Соединенные Штаты Америки",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "United States"),
        ("af", "Verenigde State"),
        ("ak", "United States"),
        ("am", "ጐሣሱጢ"),
        ("an", "United States"),
        ("ar", "الولايات المت\u{651}حدة"),
        (
            "as",
            "ম\u{9be}ৰ\u{9cd}কিন য\u{9c1}ক\u{9cd}তৰ\u{9be}ষ\u{9cd}ট\u{9cd}ৰ",
        ),
        ("ay", "United States"),
        ("az", "Birləşmiş Ştatlar"),
        ("ba", "United States"),
        ("be", "Злучаныя Штаты"),
        ("bg", "Съединени щати"),
        ("bi", "United States"),
        (
            "bn",
            "ম\u{9be}র\u{9cd}কিন য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
        ),
        (
            "bn_IN",
            "ম\u{9be}র\u{9cd}কিন য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
        ),
        ("br", "Stadoù Unanet"),
        ("bs", "SAD"),
        ("ca", "Estats Units"),
        ("ce", "United States"),
        ("ch", "United States"),
        ("cs", "Spojené státy"),
        ("cv", "United States"),
        ("cy", "Yr Unol Daleithiau"),
        ("da", "USA"),
        ("de", "Vereinigte Staaten"),
        ("dv", "United States"),
        (
            "dz",
            "ཡ\u{f74}་ན་ཡ\u{f7a}་ཊ\u{f7a}ཊ\u{f72}་ ས\u{f72}ཊ\u{f7a}ཊ\u{f72}ས\u{f72}།",
        ),
        ("ee", "United States"),
        ("el", "Ηνωμένες Πολιτείες"),
        ("en", "United States"),
        ("eo", "Usono"),
        ("es", "Estados Unidos"),
        ("et", "Ameerika Ühendriigid"),
        ("eu", "Estatu Batuak"),
        ("fa", "ایالات متحده\u{654} آمریکا"),
        ("ff", "United States"),
        ("fi", "Yhdysvallat"),
        ("fo", "Sambandsríki Amerika"),
        ("fr", "États-Unis"),
        ("fy", "United States"),
        ("ga", "Na Stáit Aontaithe"),
        ("gl", "Estados Unidos de América"),
        ("gn", "United States"),
        ("gu", "ય\u{ac1}નાઇટ\u{ac7}ડ સ\u{acd}ટ\u{ac7}ટ\u{acd}સ"),
        ("gv", "United States"),
        ("ha", "United States"),
        ("he", "ארצות הברית"),
        ("hi", "स\u{902}य\u{941}क\u{94d}त राज\u{94d}य"),
        ("hr", "Sjedinjene Države"),
        ("ht", "Etazini"),
        ("hu", "Egyesült Államok"),
        ("hy", "Ամէրիկայի Միացյալ Նահանգնէր"),
        ("ia", "Statos Unite"),
        ("id", "Amerika Serikat"),
        ("io", "United States"),
        ("is", "Bandaríkin"),
        ("it", "Stati Uniti"),
        ("iu", "United States"),
        ("ja", "米国"),
        ("ka", "ამერიკის შეერთებული შტატები"),
        ("ki", "United States"),
        ("kk", "АҚШ"),
        ("kl", "United States"),
        ("km", "សហរដ\u{17d2}ឋ\u{200b}អាមេរ\u{17b7}ក"),
        ("kn", "ಸಂಯುಕ\u{ccd}ತ ಸಂಸ\u{ccd}ಥಾನಗಳು"),
        ("ko", "미국"),
        ("ku", "Dewletên Yekbûyî"),
        ("kv", "United States"),
        ("kw", "United States"),
        ("ky", "АКШ"),
        ("lo", "United States"),
        ("lt", "Jungtinės Amerikos Valstijos"),
        ("lv", "Amerikas Savienotās Valstis"),
        ("mi", "Amerika"),
        ("mk", "Соединети држави"),
        ("ml", "ഐക\u{d4d}യന\u{d3e}ട\u{d41}കള\u{d4d}\u{200d}"),
        ("mn", "Америкын нэгдсэн улс"),
        ("mr", "य\u{941}नायट\u{947}ड स\u{94d}ट\u{947}टस\u{94d}"),
        ("ms", "Amerika Syarikat"),
        ("mt", "United States"),
        ("my", "United States"),
        ("na", "United States"),
        ("nb", "USA"),
        ("ne", "स\u{902}य\u{941}क\u{94d}त राज\u{94d}य"),
        ("nl", "Verenigde Staten"),
        ("nn", "USA"),
        ("nv", "United States"),
        ("oc", "Estats Units"),
        ("or", "ଯ\u{b41}କ\u{b4d}ତର\u{b3e}ଷ\u{b4d}ଟ\u{b4d}ର"),
        ("pa", "ਅਮਰੀਕਾ"),
        ("pi", "United States"),
        ("pl", "Stany Zjednoczone"),
        ("ps", "United States"),
        ("pt", "Estados Unidos"),
        ("pt_BR", "Estados Unidos"),
        ("ro", "Statele Unite"),
        ("ru", "Соединённые штаты"),
        ("rw", "Leta Zunze Ubumwe"),
        ("sc", "Istados Unidos"),
        ("sd", "United States"),
        ("si", "එක\u{dca}සත\u{dca} ජනපද"),
        ("sk", "Spojené štáty"),
        ("sl", "Združene države"),
        ("so", "Qaramada Midoobey ee Maraykanka"),
        ("sq", "Shtetet e Bashkuara"),
        ("sr", "Сједињене Државе"),
        ("sv", "USA"),
        ("sw", "United States"),
        ("ta", "ஐக\u{bcd}கிய அமெரிக\u{bcd}க\u{bbe}"),
        (
            "te",
            "యున\u{c48}ట\u{c46}డ\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}స\u{c4d}",
        ),
        ("tg", "Иёлоти Муттаҳида"),
        ("th", "สหร\u{e31}ฐ"),
        ("ti", "ኣመሪካ"),
        ("tk", "Birleşen Ştatlar"),
        ("tl", "Estados Unidos"),
        ("tr", "Amerika Birleşik Devletleri"),
        ("tt", "Берләшкән Штатлар"),
        ("ug", "ئامېرىكا"),
        ("uk", "США"),
        ("ur", "United States"),
        ("uz", "United States"),
        ("ve", "United States"),
        ("vi", "Mỹ"),
        ("wa", "Estats Unis"),
        ("wo", "Aamerik"),
        ("xh", "United States"),
        ("yo", "United States"),
        ("zh_CN", "美国"),
        ("zh_HK", "美國"),
        ("zh_TW", "美國"),
        ("zu", "United States"),
    ];
    #[cfg(all(feature = "us", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 37.09024;
        pub const LONGITUDE: f64 = -95.712891;
        pub const MAX_LATITUDE: f64 = 71.3577635769;
        pub const MAX_LONGITUDE: f64 = -66.96466;
        pub const MIN_LATITUDE: f64 = 18.91619;
        pub const MIN_LONGITUDE: f64 = -171.791110603;
        pub const NORTHEAST_LATITUDE: f64 = 71.3577635769;
        pub const NORTHEAST_LONGITUDE: f64 = -66.96466;
        pub const SOUTHWEST_LATITUDE: f64 = 18.91619;
        pub const SOUTHWEST_LONGITUDE: f64 = -171.791110603;
    }
}
#[cfg(all(feature = "us", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 37.09024,
            longitude: -95.712891,
            max_latitude: 71.3577635769,
            max_longitude: -66.96466,
            min_latitude: 18.91619,
            min_longitude: -171.791110603,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 71.3577635769,
                    longitude: -66.96466,
                },
                southwest: CountryGeoBound {
                    latitude: 18.91619,
                    longitude: -171.791110603,
                },
            },
        }
    }
}

#[cfg(all(feature = "us", feature = "subdivisions"))]
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
                    "AK",
                    Subdivision{
                        name: "Alaska",
                        country_alpha2: Alpha2::US,
                        code: "AK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(64.2008413), longitude: Some(-149.4936733), max_latitude: Some(71.3868712), min_latitude: Some(51.214766), max_longitude: Some(-129.9945562), min_longitude: Some(172.4445167)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alaska"), ("am", "አላስካ"), ("ar", "ألاسكا"), ("az", "Alyaska"), ("be", "Штат Аляска"), ("bg", "Аляска"), ("bn", "আল\u{9be}স\u{9cd}ক\u{9be}"), ("bs", "Alaska"), ("ca", "Alaska"), ("ccp", "𑄃𑄣𑄌\u{11134}𑄇"), ("ceb", "Alaska"), ("cs", "Aljaška"), ("cy", "Alaska"), ("da", "Alaska"), ("de", "Alaska"), ("el", "Αλάσκα"), ("en", "Alaska"), ("es", "Alaska"), ("et", "Alaska"), ("eu", "Alaska"), ("fa", "آلاسکا"), ("fi", "Alaska"), ("fr", "Alaska"), ("ga", "Alasca"), ("gl", "Alasca"), ("gu", "અલાસ\u{acd}કા"), ("ha", "Alaska"), ("ha_NE", "Alaska"), ("he", "אלסקה"), ("hi", "अलास\u{94d}का"), ("hr", "Aljaska"), ("hu", "Alaszka"), ("hy", "Ալյասկա"), ("id", "Alaska"), ("ig", "Àláskà"), ("is", "Alaska"), ("it", "Alaska"), ("ja", "アラスカ州"), ("jv", "Alaska"), ("ka", "ალასკა"), ("kk", "Аляска штаты"), ("kn", "ಅಲಾಸ\u{ccd}ಕ"), ("ko", "알래스카 주"), ("ky", "Аляска"), ("lt", "Aliaska"), ("lv", "Aļaska"), ("mk", "Алјаска"), ("ml", "അല\u{d3e}സ\u{d4d}ക"), ("mn", "Аляска"), ("mr", "अलास\u{94d}का"), ("ms", "Alaska"), ("my", "အလက\u{103a}စကာပြည\u{103a}နယ\u{103a}"), ("nb", "Alaska"), ("ne", "अलास\u{94d}का"), ("nl", "Alaska"), ("no", "Alaska"), ("or", "ଆଲ\u{b3e}ସ\u{b4d}କ\u{b3e}"), ("pa", "ਅਲਾਸਕਾ"), ("pl", "Alaska"), ("pt", "Alasca"), ("ro", "Alaska"), ("ru", "Аляска"), ("sd", "الاسڪا"), ("si", "ඇලස\u{dca}ක\u{dcf}ව"), ("sk", "Aljaška"), ("sl", "Aljaska"), ("so", "Alaska"), ("sq", "Alaska"), ("sr", "Аљаска"), ("sr_Latn", "Aljaska"), ("sv", "Alaska"), ("sw", "Alaska"), ("ta", "அல\u{bbe}ஸ\u{bcd}க\u{bbe}"), ("te", "అల\u{c3e}స\u{c4d}క\u{c3e}"), ("th", "ร\u{e31}ฐอะแลสกา"), ("tk", "Alýaska"), ("tr", "Alaska"), ("uk", "Аляска"), ("ur", "الاسکا"), ("uz", "Alyaska"), ("vi", "Alaska"), ("yo", "Alaska"), ("yo_BJ", "Alaska"), ("yue", "阿拉斯加州"), ("yue_Hans", "阿拉斯加州"), ("zh", "阿拉斯加州")]),
                        unofficial_name_list: ["Alaska"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "Alabama",
                        country_alpha2: Alpha2::US,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.3182314), longitude: Some(-86.902298), max_latitude: Some(35.007889), min_latitude: Some(30.1941334), max_longitude: Some(-84.888246), min_longitude: Some(-88.4732269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Alabama"), ("am", "አላባማ"), ("ar", "ألاباما"), ("az", "Alabama"), ("be", "Штат Алабама"), ("bg", "Алабама"), ("bn", "অ\u{9cd}য\u{9be}ল\u{9be}ব\u{9be}ম\u{9be}"), ("bs", "Alabama"), ("ca", "Alabama"), ("ccp", "𑄃𑄣𑄝𑄟"), ("ceb", "Alabama"), ("cs", "Alabama"), ("cy", "Alabama"), ("da", "Alabama"), ("de", "Alabama"), ("el", "Αλαμπάμα"), ("en", "Alabama"), ("es", "Alabama"), ("et", "Alabama"), ("eu", "Alabama"), ("fa", "آلاباما"), ("fi", "Alabama"), ("fr", "Alabama"), ("ga", "Alabama"), ("gl", "Alabama"), ("gu", "અલાબામા"), ("ha", "Alabama"), ("ha_NE", "Alabama"), ("he", "אלבמה"), ("hi", "अलाबामा"), ("hr", "Alabama"), ("hu", "Alabama"), ("hy", "Ալաբամա"), ("id", "Alabama"), ("ig", "Alabama"), ("is", "Alabama"), ("it", "Alabama"), ("ja", "アラバマ州"), ("jv", "Alabama"), ("ka", "ალაბამა"), ("kk", "Алабама"), ("kn", "ಅಲಬಾಮ"), ("ko", "앨라배마 주"), ("lt", "Alabama"), ("lv", "Alabama"), ("mk", "Алабама"), ("ml", "അലബ\u{d3e}മ"), ("mn", "Алабама"), ("mr", "अलाबामा"), ("ms", "Alabama"), ("my", "အလာဘားမားပြည\u{103a}နယ\u{103a}"), ("nb", "Alabama"), ("ne", "अलाबामा"), ("nl", "Alabama"), ("no", "Alabama"), ("pa", "ਅਲਾਬਾਮਾ"), ("pl", "Alabama"), ("pt", "Alabama"), ("ro", "Alabama"), ("ru", "Алабама"), ("sd", "الاباما"), ("si", "ඇලබ\u{dcf}ම\u{dcf}"), ("sk", "Alabama"), ("sl", "Alabama"), ("so", "Alabama"), ("sq", "Alabama"), ("sr", "Алабама"), ("sr_Latn", "Alabama"), ("sv", "Alabama"), ("sw", "Alabama"), ("ta", "அலப\u{bbe}ம\u{bbe}"), ("te", "అలబ\u{c3e}మ\u{c3e}"), ("th", "ร\u{e31}ฐแอละแบมา"), ("tk", "Alabama"), ("tr", "Alabama"), ("uk", "Алабама"), ("ur", "الاباما"), ("uz", "Alabama"), ("vi", "Alabama"), ("yo", "Ìpínlẹ\u{300} Alabama"), ("yo_BJ", "Ìpínlɛ\u{300} Alabama"), ("yue", "阿拉巴馬州"), ("yue_Hans", "阿拉巴马州"), ("zh", "亚拉巴马州")]),
                        unofficial_name_list: ["Alabama"].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "Arkansas",
                        country_alpha2: Alpha2::US,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.20105), longitude: Some(-91.8318334), max_latitude: Some(36.4997491), min_latitude: Some(33.0041059), max_longitude: Some(-89.64483790000001), min_longitude: Some(-94.61791900000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Arkansas"), ("am", "አርካንሳው"), ("ar", "أركنساس"), ("az", "Arkanzas"), ("be", "Штат Арканзас"), ("bg", "Арканзас"), ("bn", "আর\u{9cd}ক\u{9be}নস\u{9be}স"), ("bs", "Arkansas"), ("ca", "Arkansas"), ("ccp", "𑄃𑄢\u{11134}𑄇𑄚\u{11134}𑄥𑄌\u{11134}"), ("ceb", "Arkansas"), ("cs", "Arkansas"), ("cy", "Arkansas"), ("da", "Arkansas"), ("de", "Arkansas"), ("el", "Αρκάνσας"), ("en", "Arkansas"), ("es", "Arkansas"), ("et", "Arkansas"), ("eu", "Arkansas"), ("fa", "آرکانزاس"), ("fi", "Arkansas"), ("fr", "Arkansas"), ("ga", "Arkansas"), ("gl", "Arcansas - Arkansas"), ("gu", "અરકાનસાસ"), ("ha", "Arkansas"), ("ha_NE", "Arkansas"), ("he", "ארקנסו"), ("hi", "अरका\u{902}सास"), ("hr", "Arkansas"), ("hu", "Arkansas"), ("hy", "Արկանզաս"), ("id", "Arkansas"), ("ig", "Arkinsọ"), ("is", "Arkansas"), ("it", "Arkansas"), ("ja", "アーカンソー州"), ("jv", "Arkansas"), ("ka", "არკანზასი"), ("kk", "Арканзас"), ("kn", "ಅರ\u{ccd}ಕಾನ\u{ccd}ಸಾಸ\u{ccd}"), ("ko", "아칸소 주"), ("lt", "Arkanzasas"), ("lv", "Ārkanzasa"), ("mk", "Арканзас"), ("ml", "അർക\u{d4d}കൻസ\u{d3e}സ\u{d4d}"), ("mn", "Арканзас"), ("mr", "आर\u{94d}कान\u{94d}सा"), ("ms", "Arkansas"), ("my", "အာကင\u{103a}ဆောပြည\u{103a}နယ\u{103a}"), ("nb", "Arkansas"), ("ne", "आर\u{94d}कन\u{94d}सा"), ("nl", "Arkansas"), ("no", "Arkansas"), ("pa", "ਆਰਕ\u{a70}ਸਾ"), ("pl", "Arkansas"), ("pt", "Arkansas"), ("ro", "Arkansas"), ("ru", "Арканзас"), ("sd", "ارڪنساس"), ("si", "අර\u{dca}කන\u{dca}සස\u{dca}"), ("sk", "Arkansas"), ("sl", "Arkansas"), ("so", "Arkansas"), ("sq", "Arkansas"), ("sr", "Арканзас"), ("sr_Latn", "Arkanzas"), ("sv", "Arkansas"), ("sw", "Arkansas"), ("ta", "ஆர\u{bcd}கன\u{bcd}ச\u{bbe}"), ("te", "ఆర\u{c4d}క\u{c3e}న\u{c4d}స\u{c3e}"), ("th", "ร\u{e31}ฐอาร\u{e4c}ค\u{e31}นซอ"), ("tr", "Arkansas"), ("uk", "Арканзас"), ("ur", "آرکنساس"), ("uz", "Arkanzas"), ("vi", "Arkansas"), ("yo", "Arkansas"), ("yo_BJ", "Arkansas"), ("yue", "阿肯色州"), ("yue_Hans", "阿肯色州"), ("zh", "阿肯色州")]),
                        unofficial_name_list: ["Arkansas"].to_vec(),
                    }
                ),
                (
                    "AS",
                    Subdivision{
                        name: "American Samoa",
                        country_alpha2: Alpha2::US,
                        code: "AS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(21.3239718), longitude: Some(-157.876498), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃𑄟𑄬𑄢\u{11128}𑄇𑄚\u{11134} 𑄥\u{1112a}𑄟\u{1112e}𑄠"), ("en", "American Samoa")]),
                        unofficial_name_list: ["American Samoa"].to_vec(),
                    }
                ),
                (
                    "AZ",
                    Subdivision{
                        name: "Arizona",
                        country_alpha2: Alpha2::US,
                        code: "AZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.0489281), longitude: Some(-111.0937311), max_latitude: Some(37.0042599), min_latitude: Some(31.3321771), max_longitude: Some(-109.0452231), min_longitude: Some(-114.8165909)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Arizona"), ("am", "አሪዞና"), ("ar", "أريزونا"), ("az", "Arizona"), ("be", "Штат Арызона"), ("bg", "Аризона"), ("bn", "অ\u{9cd}য\u{9be}রিজোন\u{9be}"), ("bs", "Arizona"), ("ca", "Arizona"), ("ccp", "𑄃𑄬𑄢\u{11128}𑄎\u{1112e}𑄚"), ("ceb", "Arizona"), ("cs", "Arizona"), ("cy", "Arizona"), ("da", "Arizona"), ("de", "Arizona"), ("el", "Αριζόνα"), ("en", "Arizona"), ("es", "Arizona"), ("et", "Arizona"), ("eu", "Arizona"), ("fa", "آریزونا"), ("fi", "Arizona"), ("fr", "Arizona"), ("ga", "Arizona"), ("gl", "Arizona"), ("gu", "એરિઝોના"), ("ha", "Arizona"), ("ha_NE", "Arizona"), ("he", "אריזונה"), ("hi", "एरीजोना"), ("hr", "Arizona"), ("hu", "Arizona"), ("hy", "Արիզոնա"), ("id", "Arizona"), ("ig", "Arizona"), ("is", "Arizona"), ("it", "Arizona"), ("ja", "アリゾナ州"), ("jv", "Arizona"), ("ka", "არიზონა"), ("kk", "Аризона"), ("kn", "ಆರ\u{cbf}ಜೋನ"), ("ko", "애리조나 주"), ("lt", "Arizona"), ("lv", "Arizona"), ("mk", "Аризона"), ("ml", "അരിസോണ"), ("mn", "Аризона"), ("mr", "ॲरिझोना"), ("ms", "Arizona"), ("my", "အရ\u{102e}ဇ\u{102d}\u{102f}းနားပြည\u{103a}နယ\u{103a}"), ("nb", "Arizona"), ("ne", "एरिजोना"), ("nl", "Arizona"), ("no", "Arizona"), ("pa", "ਐਰੀਜ\u{a3c}\u{a4b}ਨਾ"), ("pl", "Arizona"), ("pt", "Arizona"), ("ro", "Arizona"), ("ru", "Аризона"), ("sd", "ايرزونا"), ("si", "ඇර\u{dd2}සෝන\u{dcf}"), ("sk", "Arizona"), ("sl", "Arizona"), ("so", "Arizona"), ("sq", "Arizona"), ("sr", "Аризона"), ("sr_Latn", "Arizona"), ("sv", "Arizona"), ("sw", "Arizona"), ("ta", "அரிசோன\u{bbe}"), ("te", "ఆర\u{c3f}జ\u{c4b}న\u{c3e}"), ("th", "ร\u{e31}ฐแอร\u{e34}โซนา"), ("tr", "Arizona"), ("uk", "Аризона"), ("ur", "ایریزونا"), ("uz", "Arizona"), ("vi", "Arizona"), ("yo", "Arizona"), ("yo_BJ", "Arizona"), ("yue", "亞利桑拿州"), ("yue_Hans", "亚利桑拿州"), ("zh", "亞利桑那州"), ("zu", "Arizona")]),
                        unofficial_name_list: ["Arizona"].to_vec(),
                    }
                ),
                (
                    "CA",
                    Subdivision{
                        name: "California",
                        country_alpha2: Alpha2::US,
                        code: "CA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.778261), longitude: Some(-119.4179324), max_latitude: Some(42.0095169), min_latitude: Some(32.5342321), max_longitude: Some(-114.131211), min_longitude: Some(-124.4096196)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kalifornië"), ("am", "ካሊፎርኒያ"), ("ar", "كاليفورنيا"), ("az", "Kaliforniya"), ("be", "Штат Каліфорнія"), ("bg", "Калифорния"), ("bn", "ক\u{9cd}য\u{9be}লিফোর\u{9cd}নিয\u{9bc}\u{9be}"), ("bs", "Kalifornija"), ("ca", "Califòrnia"), ("ccp", "𑄇\u{11133}𑄠𑄣\u{11128}𑄜\u{1112e}𑄢\u{11134}𑄚\u{11128}𑄠"), ("ceb", "California"), ("cs", "Kalifornie"), ("cy", "Califfornia"), ("da", "Californien"), ("de", "Kalifornien"), ("el", "Καλιφόρνια"), ("en", "California"), ("es", "California"), ("et", "California"), ("eu", "Kalifornia"), ("fa", "کالیفرنیا"), ("fi", "Kalifornia"), ("fr", "Californie"), ("ga", "California"), ("gl", "California"), ("gu", "ક\u{ac7}લિફોર\u{acd}નિયા"), ("ha", "California"), ("ha_NE", "California"), ("he", "קליפורניה"), ("hi", "क\u{948}लिफ\u{93c}ोर\u{94d}निया"), ("hr", "Kalifornija"), ("hu", "Kalifornia"), ("hy", "Կալիֆոռնիա"), ("id", "California"), ("ig", "California"), ("is", "Kalifornía"), ("it", "California"), ("ja", "カリフォルニア州"), ("jv", "California"), ("ka", "კალიფორნია"), ("kk", "Калифорния"), ("km", "កាល\u{17b8}ហ\u{17d2}វញ\u{17c9}ា"), ("kn", "ಕ\u{ccd}ಯಾಲ\u{cbf}ಫೊರ\u{ccd}ನ\u{cbf}ಯ"), ("ko", "캘리포니아 주"), ("ky", "Калифорния"), ("lt", "Kalifornija"), ("lv", "Kalifornija"), ("mk", "Калифорнија"), ("ml", "ക\u{d3e}ലിഫോർണിയ"), ("mn", "Калифорни"), ("mr", "क\u{945}लिफोर\u{94d}निया"), ("ms", "California"), ("my", "ကယ\u{103a}လ\u{102e}ဖ\u{102d}\u{102f}းန\u{102e}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "California"), ("ne", "क\u{94d}यालिफोर\u{94d}निया"), ("nl", "Californië"), ("no", "California"), ("or", "କ\u{b3e}ଲ\u{b3f}ଫର\u{b4d}ଣ\u{b4d}ଣ\u{b3f}ଆ"), ("pa", "ਕ\u{a48}ਲੀਫ\u{a3c}\u{a4b}ਰਨੀਆ"), ("pl", "Kalifornia"), ("ps", "کالېفورنیا"), ("pt", "Califórnia"), ("ro", "California"), ("ru", "Калифорния"), ("sd", "ڪيليفورنيا"), ("si", "කැල\u{dd2}ෆෝන\u{dd2}ය\u{dcf}"), ("sk", "Kalifornia"), ("sl", "Kalifornija"), ("so", "Kalifornia"), ("sq", "Kaliforni"), ("sr", "Калифорнија"), ("sr_Latn", "Kalifornija"), ("sv", "Kalifornien"), ("sw", "California"), ("ta", "கலிபோர\u{bcd}னிய\u{bbe}"), ("te", "క\u{c3e}ల\u{c3f}ఫ\u{c4b}ర\u{c4d}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐแคล\u{e34}ฟอร\u{e4c}เน\u{e35}ย"), ("tr", "Kaliforniya"), ("uk", "Каліфорнія"), ("ur", "کیلی فورنیا"), ("uz", "Kaliforniya"), ("vi", "California"), ("yo", "Kalifọ\u{301}rníà"), ("yo_BJ", "Kalifɔ\u{301}rníà"), ("yue", "加利福尼亞州"), ("yue_Hans", "加利福尼亚州"), ("zh", "加利福尼亚州"), ("zu", "California")]),
                        unofficial_name_list: ["California"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "Colorado",
                        country_alpha2: Alpha2::US,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5500507), longitude: Some(-105.7820674), max_latitude: Some(41.0034439), min_latitude: Some(36.992424), max_longitude: Some(-102.040878), min_longitude: Some(-109.060256)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Colorado"), ("am", "ኮሎራዶ"), ("ar", "كولورادو"), ("az", "Kolorado"), ("be", "Штат Каларада"), ("bg", "Колорадо"), ("bn", "কলোর\u{9be}ডো"), ("bs", "Colorado"), ("ca", "Colorado"), ("ccp", "𑄇\u{11127}𑄣\u{11127}𑄢𑄓\u{1112e}"), ("ceb", "Colorado"), ("cs", "Colorado"), ("cy", "Colorado"), ("da", "Colorado"), ("de", "Colorado"), ("el", "Κολοράντο"), ("en", "Colorado"), ("es", "Colorado"), ("et", "Colorado"), ("eu", "Colorado"), ("fa", "کلرادو"), ("fi", "Colorado"), ("fr", "Colorado"), ("ga", "Colorado"), ("gl", "Colorado, Estados Unidos de América"), ("gu", "કોલોરાડો"), ("ha", "Colorado"), ("ha_NE", "Colorado"), ("he", "קולורדו"), ("hi", "कॉलराडो"), ("hr", "Colorado"), ("hu", "Colorado"), ("hy", "Կոլորադո"), ("id", "Colorado"), ("ig", "Colorado"), ("is", "Colorado"), ("it", "Colorado"), ("ja", "コロラド州"), ("jv", "Colorado"), ("ka", "კოლორადოს შტატი"), ("kk", "Колорадо"), ("kn", "ಕೊಲೊರಾಡೋ"), ("ko", "콜로라도 주"), ("lt", "Koloradas"), ("lv", "Kolorādo"), ("mk", "Колорадо"), ("ml", "കൊളറ\u{d3e}ഡോ"), ("mn", "Колорадо"), ("mr", "कॉलोराडो"), ("ms", "Colorado"), ("my", "ကော\u{103a}လ\u{102d}\u{102f}ရာဒ\u{102d}\u{102f}ပြည\u{103a}နယ\u{103a}"), ("nb", "Colorado"), ("ne", "कोलोराडो"), ("nl", "Colorado"), ("no", "Colorado"), ("pa", "ਕ\u{a4b}ਲ\u{a4b}ਰਾਡ\u{a4b}"), ("pl", "Kolorado"), ("ps", "کلراډو"), ("pt", "Colorado"), ("ro", "Colorado"), ("ru", "Колорадо"), ("sd", "ڪولراڊو"), ("si", "කොලර\u{dcf}ඩෝ"), ("sk", "Colorado"), ("sl", "Kolorado"), ("sq", "Colorado"), ("sr", "Колорадо"), ("sr_Latn", "Kolorado"), ("sv", "Colorado"), ("sw", "Colorado"), ("ta", "கொலர\u{bbe}டோ"), ("te", "క\u{c4a}లర\u{c3e}డ\u{c4b}"), ("th", "ร\u{e31}ฐโคโลราโด"), ("tr", "Colorado"), ("uk", "Колорадо"), ("ur", "کولوراڈو"), ("uz", "Kolorado"), ("vi", "Colorado"), ("yo", "Colorado"), ("yo_BJ", "Colorado"), ("yue", "科羅拉多州"), ("yue_Hans", "科罗拉多州"), ("zh", "科羅拉多州"), ("zu", "Colorado")]),
                        unofficial_name_list: ["Colorado"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "Connecticut",
                        country_alpha2: Alpha2::US,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6032207), longitude: Some(-73.087749), max_latitude: Some(42.050587), min_latitude: Some(40.9869801), max_longitude: Some(-71.787239), min_longitude: Some(-73.727775)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Connecticut"), ("am", "ኮነቲከት"), ("ar", "كونيتيكت"), ("az", "Konnektikut"), ("be", "Канектыкут"), ("bg", "Кънектикът"), ("bn", "ক\u{9be}নেটিক\u{9be}ট"), ("bs", "Connecticut"), ("ca", "Connecticut"), ("ccp", "𑄇𑄚𑄬𑄇\u{11134}𑄑\u{11128}𑄇\u{1112a}𑄖\u{11134}"), ("ceb", "Connecticut"), ("cs", "Connecticut"), ("cy", "Connecticut"), ("da", "Connecticut"), ("de", "Connecticut"), ("el", "Κονέκτικατ"), ("en", "Connecticut"), ("es", "Connecticut"), ("et", "Connecticut"), ("eu", "Connecticut"), ("fa", "کانتیکت"), ("fi", "Connecticut"), ("fr", "Connecticut"), ("ga", "Connecticut"), ("gl", "Connecticut"), ("gu", "કન\u{ac7}ક\u{acd}ટિકટ"), ("ha", "Connecticut"), ("ha_NE", "Connecticut"), ("he", "קונטיקט"), ("hi", "कन\u{947}क\u{94d}टिकट"), ("hr", "Connecticut"), ("hu", "Connecticut"), ("hy", "Կոնեկտիկուտ"), ("id", "Connecticut"), ("ig", "Kónétíkùt"), ("is", "Connecticut"), ("it", "Connecticut"), ("ja", "コネチカット州"), ("jv", "Connecticut"), ("ka", "კონექტიკუტი"), ("kk", "Коннектикут"), ("kn", "ಕನ\u{cc6}ಕ\u{ccd}ಟ\u{cbf}ಕಟ\u{ccd}"), ("ko", "코네티컷 주"), ("lt", "Konektikutas"), ("lv", "Konektikuta"), ("mk", "Конектикат"), ("ml", "കണെക\u{d4d}റ\u{d4d}റിക\u{d4d}കട\u{d4d}ട\u{d4d}"), ("mn", "Коннектикут"), ("mr", "कन\u{947}टिकट"), ("ms", "Connecticut"), ("nb", "Connecticut"), ("ne", "कन\u{947}क\u{94d}टिकट"), ("nl", "Connecticut"), ("no", "Connecticut"), ("pa", "ਕਨ\u{a48}ਟੀਕਟ"), ("pl", "Connecticut"), ("pt", "Connecticut"), ("ro", "Connecticut"), ("ru", "Коннектикут"), ("si", "කොන\u{dca}නේස\u{dca}ට\u{dd2}කට\u{dca}"), ("sk", "Connecticut"), ("sl", "Connecticut"), ("sq", "Connecticut"), ("sr", "Конектикат"), ("sr_Latn", "Konektikat"), ("sv", "Connecticut"), ("sw", "Connecticut"), ("ta", "கனெடிகட\u{bcd}"), ("te", "కన\u{c46}క\u{c4d}ట\u{c3f}కట\u{c4d}"), ("th", "ร\u{e31}ฐคอนเนตท\u{e34}ค\u{e31}ต"), ("tr", "Connecticut"), ("uk", "Коннектикут"), ("ur", "کنیکٹیکٹ"), ("uz", "Konnektikut"), ("vi", "Connecticut"), ("yo", "Connecticut"), ("yo_BJ", "Connecticut"), ("yue", "康湼狄格州"), ("yue_Hans", "康湼狄格州"), ("zh", "康乃狄克州"), ("zu", "Connecticut")]),
                        unofficial_name_list: ["Connecticut"].to_vec(),
                    }
                ),
                (
                    "DC",
                    Subdivision{
                        name: "District of Columbia",
                        country_alpha2: Alpha2::US,
                        code: "DC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.9071923), longitude: Some(-77.0368707), max_latitude: Some(38.995548), min_latitude: Some(38.8031495), max_longitude: Some(-76.909393), min_longitude: Some(-77.11974)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Washington"), ("am", "ዋሺንግተን ዲሲ"), ("ar", "واشنطن العاصمة"), ("as", "ৱ\u{9be}শ\u{9cd}বিংটন ডি.চি."), ("az", "Vaşinqton"), ("be", "Вашынгтон"), ("bg", "Вашингтон"), ("bn", "ওয\u{9bc}\u{9be}শিংটন, ডি.সি."), ("ca", "Washington DC"), ("ccp", "𑄤𑄥\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134} 𑄓\u{11128}𑄥\u{11128}"), ("ceb", "District of Columbia"), ("cs", "Washington, D.C."), ("da", "Washington D.C."), ("de", "Washington, D.C."), ("el", "Ουάσινγκτον"), ("en", "Washington DC"), ("es", "Washington D. C."), ("et", "Washington"), ("eu", "Washington"), ("fa", "واشینگتن، دی. سی."), ("fi", "Washington"), ("fr", "Washington"), ("gl", "Washington, D.C."), ("gu", "વોશિ\u{a82}ગ\u{acd}ટન,ડી.સી."), ("he", "וושינגטון די. סי."), ("hi", "वॉशि\u{902}गटन डी॰ सी॰"), ("hr", "Washington"), ("hu", "Washington"), ("hy", "Վաշինգտոն, Կոլումբիայի շրջան"), ("id", "Washington"), ("is", "Washington"), ("it", "Washington"), ("ka", "ვაშინგტონი"), ("kn", "ವಾಷ\u{cbf}ಂಗ\u{ccd}ಟನ\u{ccd}"), ("lt", "Vašingtonas"), ("lv", "Vašingtona"), ("ml", "വ\u{d3e}ഷിങ\u{d4d}ടൺ"), ("mn", "Вашингтон хот"), ("mr", "वॉशि\u{902}ग\u{94d}टन"), ("ms", "Washington"), ("nb", "Washington D.C."), ("ne", "वासिङटन, डिसि"), ("nl", "Washington D.C."), ("no", "Washington D.C."), ("or", "ୱ\u{b3e}ଶ\u{b3f}ଙ\u{b4d}ଗଟନ, ଡ\u{b3f}. ସ\u{b3f}."), ("pl", "Waszyngton"), ("ps", "واشنګټن ډي سي"), ("pt", "Washington"), ("ro", "Washington"), ("ru", "Вашингтон"), ("si", "වොෂ\u{dd2}න\u{dca}ටන\u{dca}"), ("sk", "Washington D.C."), ("sl", "Washington"), ("sr", "Вашингтон"), ("sr_Latn", "Vašington"), ("sv", "Washington, D.C."), ("sw", "Washington"), ("ta", "வ\u{bbe}சிங\u{bcd}டன\u{bcd}"), ("te", "వ\u{c3e}ష\u{c3f}ంగ\u{c4d}టన\u{c4d}"), ("th", "วอช\u{e34}งต\u{e31}น ด\u{e35}.ซ\u{e35}."), ("tk", "Waşington"), ("tr", "Washington"), ("uk", "Вашингтон"), ("ur", "واشنگٹن ڈی سی"), ("uz", "Kolumbiya okrugi"), ("vi", "Washington"), ("zh", "華盛頓哥倫比亞特區")]),
                        unofficial_name_list: ["District of Columbia"].to_vec(),
                    }
                ),
                (
                    "DE",
                    Subdivision{
                        name: "Delaware",
                        country_alpha2: Alpha2::US,
                        code: "DE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.9108325), longitude: Some(-75.52766989999999), max_latitude: Some(39.8394839), min_latitude: Some(38.451018), max_longitude: Some(-75.04869339999999), min_longitude: Some(-75.78914790000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Delaware"), ("am", "ዴላዌር"), ("ar", "ديلاوير"), ("az", "Delaver"), ("be", "Штат Дэлавэр"), ("bg", "Делауеър"), ("bn", "ডেল\u{9be}ওয\u{9bc}\u{9cd}য\u{9be}র"), ("bs", "Delaware"), ("ca", "Delaware"), ("ccp", "𑄓𑄬𑄣\u{11134}𑄤𑄢\u{11134}"), ("ceb", "Delaware (estado)"), ("cs", "Delaware"), ("cy", "Delaware"), ("da", "Delaware"), ("de", "Delaware"), ("el", "Ντέλαγουερ"), ("en", "Delaware"), ("es", "Delaware"), ("et", "Delaware"), ("eu", "Delaware"), ("fa", "دلاویر"), ("fi", "Delaware"), ("fr", "Delaware"), ("ga", "Delaware"), ("gl", "Delaware"), ("gu", "ડ\u{ac7}લાવ\u{ac7}ર"), ("ha", "Delaware"), ("ha_NE", "Delaware"), ("he", "דלאוור"), ("hi", "ड\u{947}लाव\u{947}यर"), ("hr", "Delaware"), ("hu", "Delaware"), ("hy", "Դելավեր"), ("id", "Delaware"), ("ig", "Déláwè"), ("is", "Delaware"), ("it", "Delaware"), ("ja", "デラウェア州"), ("jv", "Delaware"), ("ka", "დელავერი"), ("kk", "Делавэр"), ("kn", "ಡ\u{cc6}ಲಾವೇರ\u{ccd}"), ("ko", "델라웨어 주"), ("lt", "Delaveras"), ("lv", "Delavēra"), ("mk", "Делавер"), ("ml", "ഡെലവെയർ"), ("mn", "Делавар"), ("mr", "ड\u{947}लाव\u{947}र"), ("ms", "Delaware"), ("my", "ဒယ\u{103a}လာဝ\u{1032}ပြည\u{103a}နယ\u{103a}"), ("nb", "Delaware"), ("ne", "ड\u{947}लाव\u{947}यर"), ("nl", "Delaware"), ("no", "Delaware"), ("pa", "ਡ\u{a47}ਲਾਵ\u{a47}ਅਰ"), ("pl", "Delaware"), ("pt", "Delaware"), ("ro", "Delaware"), ("ru", "Делавэр"), ("si", "දෙල\u{dcf}වරේ"), ("sk", "Delaware"), ("sl", "Delaware"), ("sq", "Delaware"), ("sr", "Делавер"), ("sr_Latn", "Delaver"), ("sv", "Delaware"), ("sw", "Delaware"), ("ta", "டெலவெயர\u{bcd}"), ("te", "డ\u{c46}ల\u{c3e}వ\u{c47}ర\u{c4d}"), ("th", "ร\u{e31}ฐเดลาแวร\u{e4c}"), ("tr", "Delaware"), ("uk", "Делавер"), ("ur", "ڈیلاویئر"), ("uz", "Delaver"), ("vi", "Delaware"), ("yo", "Delaware"), ("yo_BJ", "Delaware"), ("yue", "特拉華州"), ("yue_Hans", "特拉华州"), ("zh", "特拉华州")]),
                        unofficial_name_list: ["Delaware"].to_vec(),
                    }
                ),
                (
                    "FL",
                    Subdivision{
                        name: "Florida",
                        country_alpha2: Alpha2::US,
                        code: "FL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(27.6648274), longitude: Some(-81.5157535), max_latitude: Some(31.000968), min_latitude: Some(24.5210795), max_longitude: Some(-80.0311371), min_longitude: Some(-87.634896)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Florida"), ("am", "ፍሎሪዳ"), ("ar", "فلوريدا"), ("az", "Florida"), ("be", "Штат Фларыда"), ("bg", "Флорида"), ("bn", "ফ\u{9cd}লোরিড\u{9be}"), ("bs", "Florida"), ("ca", "Florida"), ("ccp", "𑄜\u{11133}𑄣\u{1112e}𑄢\u{11128}𑄓"), ("ceb", "Florida"), ("cs", "Florida"), ("cy", "Florida"), ("da", "Florida"), ("de", "Florida"), ("el", "Φλόριντα"), ("en", "Florida"), ("es", "Florida"), ("et", "Florida"), ("eu", "Florida"), ("fa", "فلوریدا"), ("fi", "Florida"), ("fr", "Floride"), ("ga", "Florida"), ("gl", "Florida"), ("gu", "ફ\u{acd}લોરિડા"), ("ha", "Florida"), ("ha_NE", "Florida"), ("he", "פלורידה"), ("hi", "फ\u{93c}\u{94d}लोरिडा"), ("hr", "Florida"), ("hu", "Florida"), ("hy", "Ֆլորիդա"), ("id", "Florida"), ("ig", "Flórídạ"), ("is", "Flórída"), ("it", "Florida"), ("ja", "フロリダ州"), ("jv", "Florida"), ("ka", "ფლორიდა"), ("kk", "Флорида"), ("kn", "ಫ\u{ccd}ಲಾರ\u{cbf}ಡ"), ("ko", "플로리다 주"), ("lt", "Florida"), ("lv", "Florida"), ("mk", "Флорида"), ("ml", "ഫ\u{d4d}ലോറിഡ"), ("mn", "Флорида"), ("mr", "फ\u{94d}लोरिडा"), ("ms", "Florida"), ("my", "ဖလော\u{103a}ရ\u{102e}ဒါပြည\u{103a}နယ\u{103a}"), ("nb", "Florida"), ("ne", "फ\u{94d}लोरिडा"), ("nl", "Florida"), ("no", "Florida"), ("or", "ଫ\u{b4d}ଲୋର\u{b3f}ଡ\u{b3c}\u{b3e}"), ("pa", "ਫ\u{a3c}ਲ\u{a4c}ਰਿਡਾ"), ("pl", "Floryda"), ("pt", "Flórida"), ("ro", "Florida"), ("ru", "Флорида"), ("si", "ෆ\u{dca}ලොර\u{dd2}ඩ\u{dcf}"), ("sk", "Florida"), ("sl", "Florida"), ("so", "Florida"), ("sq", "Florida"), ("sr", "Флорида"), ("sr_Latn", "Florida"), ("sv", "Florida"), ("sw", "Florida"), ("ta", "புளோரிட\u{bbe}"), ("te", "ఫ\u{c4d}ల\u{c4b}ర\u{c3f}డ\u{c3e}"), ("th", "ร\u{e31}ฐฟลอร\u{e34}ดา"), ("tr", "Florida"), ("uk", "Флорида"), ("ur", "فلوریڈا"), ("uz", "Florida"), ("vi", "Florida"), ("yo", "Florida"), ("yo_BJ", "Florida"), ("yue", "科利打省"), ("yue_Hans", "科利打省"), ("zh", "佛罗里达州"), ("zu", "Florida")]),
                        unofficial_name_list: ["Florida"].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "Georgia",
                        country_alpha2: Alpha2::US,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.1656221), longitude: Some(-82.9000751), max_latitude: Some(35.0006589), min_latitude: Some(30.3555908), max_longitude: Some(-80.8407866), min_longitude: Some(-85.6051649)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Georgia"), ("am", "ጆርጂያ"), ("ar", "جورجيا"), ("az", "Corciya"), ("be", "Штат Джорджыя"), ("bg", "Джорджия"), ("bn", "জর\u{9cd}জিয\u{9bc}\u{9be}"), ("bs", "Georgia"), ("ca", "Geòrgia"), ("ccp", "𑄎\u{11127}𑄢\u{11134}𑄎\u{11128}𑄠"), ("ceb", "Georgia"), ("cs", "Georgie"), ("cy", "Georgia"), ("da", "Georgia"), ("de", "Georgia"), ("el", "Τζόρτζια"), ("en", "Georgia"), ("es", "Georgia"), ("et", "Georgia"), ("eu", "Georgia (AEB)"), ("fa", "جورجیا"), ("fi", "Georgia"), ("fr", "Géorgie"), ("ga", "Georgia"), ("gl", "Xeorxia - Georgia"), ("gu", "જ\u{acd}યોર\u{acd}જિયા"), ("he", "ג׳ורג׳יה"), ("hi", "जॉर\u{94d}जिया"), ("hr", "Georgia"), ("hu", "Georgia"), ("hy", "Ջորջիա"), ("id", "Georgia"), ("ig", "Jorjiạ"), ("is", "Georgía"), ("it", "Georgia"), ("ja", "ジョージア州"), ("jv", "Georgia"), ("ka", "ჯორჯია"), ("kk", "Джорджия"), ("kn", "ಜಾರ\u{ccd}ಜ\u{cbf}ಯ"), ("ko", "조지아 주"), ("ky", "Жоржия"), ("lt", "Džordžija"), ("lv", "Džordžija"), ("mk", "Џорџија"), ("ml", "ജോർജിയ"), ("mn", "Жоржиа"), ("mr", "जॉर\u{94d}जिया"), ("ms", "Georgia"), ("my", "ဂျော\u{103a}ဂျ\u{102e}ယာပြည\u{103a}နယ\u{103a}"), ("nb", "Georgia"), ("ne", "जर\u{94d}जिया"), ("nl", "Georgia"), ("no", "Georgia"), ("pa", "ਜਾਰਜੀਆ"), ("pl", "Georgia"), ("pt", "Geórgia"), ("ro", "Georgia"), ("ru", "Джорджия"), ("si", "ජෝර\u{dca}ජ\u{dd2}ය\u{dcf}ව"), ("sk", "Georgia"), ("sl", "Georgia"), ("sq", "Georgia"), ("sr", "Џорџија"), ("sr_Latn", "Džordžija"), ("sv", "Georgia"), ("sw", "Georgia"), ("ta", "ஜோர\u{bcd}ஜிய\u{bbe}"), ("te", "జ\u{c3e}ర\u{c4d}జ\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐจอร\u{e4c}เจ\u{e35}ย"), ("tk", "Jorjiýa"), ("tr", "Georgia"), ("uk", "Джорджія"), ("ur", "ریاست جارجیا"), ("uz", "Jorjiya"), ("vi", "Georgia"), ("yo", "Ìpínlẹ\u{300} Georgia"), ("yo_BJ", "Ìpínlɛ\u{300} Georgia"), ("yue", "喬治亞州"), ("yue_Hans", "乔治亚州"), ("zh", "喬治亞州")]),
                        unofficial_name_list: ["Georgia"].to_vec(),
                    }
                ),
                (
                    "GU",
                    Subdivision{
                        name: "Guam",
                        country_alpha2: Alpha2::US,
                        code: "GU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.90194), longitude: Some(-89.82361), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄉\u{1112a}𑄠𑄟\u{11134}"), ("en", "Guam")]),
                        unofficial_name_list: ["Guam"].to_vec(),
                    }
                ),
                (
                    "HI",
                    Subdivision{
                        name: "Hawaii",
                        country_alpha2: Alpha2::US,
                        code: "HI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(19.8967662), longitude: Some(-155.5827818), max_latitude: Some(28.4014611), min_latitude: Some(18.9108109), max_longitude: Some(-154.8067783), min_longitude: Some(-178.3059168)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Hawaii"), ("am", "ሃዋኢ"), ("ar", "هاواي"), ("az", "Havay"), ("be", "Штат Гаваі"), ("bg", "Хаваи"), ("bn", "হ\u{9be}ওয\u{9bc}\u{9be}ই"), ("bs", "Hawaii"), ("ca", "Hawaii"), ("ccp", "𑄦\u{11127}𑄤𑄃\u{11129}"), ("ceb", "Hawaii"), ("cs", "Havaj"), ("cy", "Hawaii"), ("da", "Hawaii"), ("de", "Hawaii"), ("el", "Χαβάη"), ("en", "Hawaii"), ("es", "Hawái"), ("et", "Hawaii osariik"), ("eu", "Hawaii"), ("fa", "هاوائی"), ("fi", "Havaiji"), ("fr", "Hawaï"), ("ga", "Haváí"), ("gl", "Hawai - Hawai’i"), ("gu", "હવાઈ"), ("ha", "Hawaii"), ("ha_NE", "Hawaii"), ("he", "הוואי"), ("hi", "हवाई"), ("hr", "Havaji"), ("hu", "Hawaii"), ("hy", "Հավայի"), ("id", "Hawaii"), ("ig", "Hawaii"), ("is", "Hawaii / Havaí / Hawaí"), ("it", "Hawaii"), ("ja", "ハワイ州"), ("jv", "Hawaii"), ("ka", "ჰავაის შტატი"), ("kk", "Гавайи"), ("kn", "ಹವಾಯ\u{cbf}"), ("ko", "하와이 주"), ("ky", "Гавайи"), ("lt", "Havajai"), ("lv", "Havajas"), ("mk", "Хаваи"), ("ml", "ഹവ\u{d3e}യി"), ("mn", "Хавай"), ("mr", "हवाई"), ("ms", "Hawaii"), ("my", "ဟာဝ\u{102d}\u{102f}င\u{103a}ယ\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "Hawaii"), ("ne", "हवाई"), ("nl", "Hawaï"), ("no", "Hawaii"), ("pa", "ਹਵਾਈ"), ("pl", "Hawaje"), ("pt", "Havaí"), ("ro", "Hawaii"), ("ru", "Гавайи"), ("si", "හව\u{dcf}ය\u{dd2}"), ("sk", "Havaj"), ("sl", "Havaji"), ("so", "Hawaay"), ("sq", "Hawaii"), ("sr", "Хаваји"), ("sr_Latn", "Havaji"), ("sv", "Hawaii"), ("sw", "Hawaii"), ("ta", "ஹவ\u{bbe}ய\u{bcd}"), ("te", "హవ\u{c3e}య\u{c3f}"), ("th", "ร\u{e31}ฐฮาวาย"), ("tk", "Gawaý adalary"), ("tr", "Hawaii"), ("uk", "Гаваї"), ("ur", "ہوائی"), ("uz", "Gavayi"), ("vi", "Hawaii"), ("yo", "Hawaii"), ("yo_BJ", "Hawaii"), ("yue", "夏威夷州"), ("yue_Hans", "夏威夷州"), ("zh", "夏威夷州"), ("zu", "Hawaii")]),
                        unofficial_name_list: ["Hawaii"].to_vec(),
                    }
                ),
                (
                    "IA",
                    Subdivision{
                        name: "Iowa",
                        country_alpha2: Alpha2::US,
                        code: "IA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8780025), longitude: Some(-93.097702), max_latitude: Some(43.5011961), min_latitude: Some(40.375437), max_longitude: Some(-90.1400609), min_longitude: Some(-96.639535)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Iowa"), ("am", "አዮዋ"), ("ar", "آيوا"), ("az", "Ayova"), ("be", "Штат Аява"), ("bg", "Айова"), ("bn", "আইওয\u{9bc}\u{9be}"), ("bs", "Iowa"), ("ca", "Iowa"), ("ccp", "𑄃\u{1112d}𑄤"), ("ceb", "Iowa"), ("cs", "Iowa"), ("cy", "Iowa"), ("da", "Iowa"), ("de", "Iowa"), ("el", "Αϊόβα"), ("en", "Iowa"), ("es", "Iowa"), ("et", "Iowa"), ("eu", "Iowa"), ("fa", "آیووا"), ("fi", "Iowa"), ("fr", "Iowa"), ("ga", "Iowa"), ("gl", "Iowa"), ("gu", "આયોવા"), ("ha", "Iowa"), ("ha_NE", "Iowa"), ("he", "איווה"), ("hi", "आयोवा"), ("hr", "Iowa"), ("hu", "Iowa"), ("hy", "Այովա"), ("id", "Iowa"), ("ig", "Áyowạ"), ("is", "Iowa"), ("it", "Iowa"), ("ja", "アイオワ州"), ("jv", "Iowa"), ("ka", "აიოვა"), ("kk", "Айова"), ("kn", "ಅಯೋವಾ"), ("ko", "아이오와 주"), ("lt", "Ajova"), ("lv", "Aiova"), ("mk", "Ајова"), ("ml", "ഐയവ"), ("mn", "Айова"), ("mr", "आयोवा"), ("ms", "Iowa"), ("my", "အ\u{102d}\u{102f}င\u{103a}အ\u{102d}\u{102f}ဝါပြည\u{103a}နယ\u{103a}"), ("nb", "Iowa"), ("ne", "आयोवा"), ("nl", "Iowa"), ("no", "Iowa"), ("pa", "ਆਇਓਵਾ"), ("pl", "Iowa"), ("pt", "Iowa"), ("ro", "Iowa"), ("ru", "Айова"), ("si", "අයෝව\u{dcf}"), ("sk", "Iowa"), ("sl", "Iowa"), ("so", "Iowa"), ("sq", "Iowa"), ("sr", "Ајова"), ("sr_Latn", "Ajova"), ("sv", "Iowa"), ("sw", "Iowa"), ("ta", "அயோவ\u{bbe}"), ("te", "అయ\u{c4b}వ\u{c3e}"), ("th", "ร\u{e31}ฐไอโอวา"), ("tr", "Iowa"), ("uk", "Айова"), ("ur", "آئیووا"), ("uz", "Ayova"), ("vi", "Iowa"), ("yo", "Iowa"), ("yo_BJ", "Iowa"), ("yue", "埃奧華省"), ("yue_Hans", "埃奥华省"), ("zh", "艾奥瓦州")]),
                        unofficial_name_list: ["Iowa"].to_vec(),
                    }
                ),
                (
                    "ID",
                    Subdivision{
                        name: "Idaho",
                        country_alpha2: Alpha2::US,
                        code: "ID",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.0682019), longitude: Some(-114.7420408), max_latitude: Some(49.0011461), min_latitude: Some(41.9880051), max_longitude: Some(-111.043495), min_longitude: Some(-117.2413657)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Idaho"), ("am", "አይዳሆ"), ("ar", "أيداهو"), ("az", "Aydaho"), ("be", "Штат Айдаха"), ("bg", "Айдахо"), ("bn", "আইড\u{9be}হো"), ("bs", "Idaho"), ("ca", "Idaho"), ("ccp", "𑄃\u{1112d}𑄓𑄦\u{1112e}"), ("ceb", "Idaho"), ("cs", "Idaho"), ("cy", "Idaho"), ("da", "Idaho"), ("de", "Idaho"), ("el", "Αϊντάχο"), ("en", "Idaho"), ("es", "Idaho"), ("et", "Idaho"), ("eu", "Idaho"), ("fa", "آیداهو"), ("fi", "Idaho"), ("fr", "Idaho"), ("ga", "Idaho"), ("gl", "Idaho"), ("gu", "ઇદાહો"), ("he", "איידהו"), ("hi", "आयडाहो"), ("hr", "Idaho"), ("hu", "Idaho"), ("hy", "Այդահո"), ("id", "Idaho"), ("ig", "Idaho"), ("is", "Idaho"), ("it", "Idaho"), ("ja", "アイダホ州"), ("jv", "Idaho"), ("ka", "აიდაჰო"), ("kk", "Айдахо"), ("kn", "ಐಡಹೋ"), ("ko", "아이다호 주"), ("lt", "Aidahas"), ("lv", "Aidaho"), ("mk", "Ајдахо"), ("ml", "ഐഡഹോ"), ("mn", "Айдахо"), ("mr", "आयडाहो"), ("ms", "Idaho"), ("my", "အ\u{102d}\u{102f}င\u{103a}ဒါဟ\u{102d}\u{102f}ပြည\u{103a}နယ\u{103a}"), ("nb", "Idaho"), ("ne", "आयडाहो"), ("nl", "Idaho"), ("no", "Idaho"), ("pa", "ਆਇਡਾਹ\u{a4b}"), ("pl", "Idaho"), ("pt", "Idaho"), ("ro", "Idaho"), ("ru", "Айдахо"), ("si", "ඉඩ\u{dcf}හෝ"), ("sk", "Idaho"), ("sl", "Idaho"), ("sq", "Idaho"), ("sr", "Ајдахо"), ("sr_Latn", "Ajdaho"), ("sv", "Idaho"), ("sw", "Idaho"), ("ta", "ஐடஹோ"), ("te", "ఐడహ\u{c4a}"), ("th", "ร\u{e31}ฐไอดาโฮ"), ("tr", "Idaho"), ("uk", "Айдахо"), ("ur", "ایڈاہو"), ("uz", "Aydaho"), ("vi", "Idaho"), ("yo", "Idaho"), ("yo_BJ", "Idaho"), ("yue", "愛達荷州"), ("yue_Hans", "爱达荷州"), ("zh", "爱达荷州"), ("zu", "Idaho")]),
                        unofficial_name_list: ["Idaho"].to_vec(),
                    }
                ),
                (
                    "IL",
                    Subdivision{
                        name: "Illinois",
                        country_alpha2: Alpha2::US,
                        code: "IL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6331249), longitude: Some(-89.3985283), max_latitude: Some(42.5083379), min_latitude: Some(36.970298), max_longitude: Some(-87.4951991), min_longitude: Some(-91.5130789)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Illinois"), ("am", "ኢሊኖይ"), ("ar", "إلينوي"), ("az", "İllinoys"), ("be", "Штат Ілінойс"), ("bg", "Илинойс"), ("bn", "ইলিনয\u{9bc}"), ("bs", "Illinois"), ("ca", "Illinois"), ("ccp", "𑄣\u{11128}𑄣\u{11133}𑄦\u{11128}𑄚\u{1112e}𑄃\u{11128}𑄌\u{11134}"), ("ceb", "Illinois"), ("cs", "Illinois"), ("cy", "Illinois"), ("da", "Illinois"), ("de", "Illinois"), ("el", "Ιλινόι"), ("en", "Illinois"), ("es", "Illinois"), ("et", "Illinois"), ("eu", "Illinois"), ("fa", "ایلی\u{200c}نوی"), ("fi", "Illinois"), ("fr", "Illinois"), ("ga", "Illinois"), ("gl", "Illinois"), ("gu", "ઇલિનોઇસ"), ("he", "אילינוי"), ("hi", "इलिनॉय"), ("hr", "Illinois"), ("hu", "Illinois"), ("hy", "Իլինոյս"), ("id", "Illinois"), ("ig", "Ilinoi"), ("is", "Illinois"), ("it", "Illinois"), ("ja", "イリノイ州"), ("jv", "Illinois"), ("ka", "ილინოისი"), ("kk", "Иллинойс"), ("kn", "ಇಲ\u{cbf}ನಾಯ\u{ccd}ಸ\u{ccd}"), ("ko", "일리노이 주"), ("ky", "Иллинойс"), ("lt", "Ilinojus"), ("lv", "Ilinoisa"), ("mk", "Илиноис"), ("ml", "ഇല\u{d4d}ലിനോയി"), ("mn", "Иллиной"), ("mr", "इलिनॉय"), ("ms", "Illinois"), ("my", "အ\u{102e}လ\u{102e}န\u{103d}\u{102d}\u{102f}င\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Illinois"), ("ne", "इलिनाय"), ("nl", "Illinois"), ("no", "Illinois"), ("pa", "ਇਲੀਨਾਏ"), ("pl", "Illinois"), ("pt", "Illinois"), ("ro", "Illinois"), ("ru", "Иллинойс"), ("si", "ඉල\u{dd2}නොය\u{dd2}ස\u{dca}"), ("sk", "Illinois"), ("sl", "Illinois"), ("so", "Illinois"), ("sq", "Illinois"), ("sr", "Илиноис"), ("sr_Latn", "Ilinois"), ("sv", "Illinois"), ("sw", "Illinois"), ("ta", "இலினொய\u{bcd}"), ("te", "ఇల\u{c4d}ల\u{c3f}న\u{c3e}య\u{c3f}స\u{c4d}"), ("th", "ร\u{e31}ฐอ\u{e34}ลล\u{e34}นอยส\u{e4c}"), ("tr", "Illinois"), ("uk", "Іллінойс"), ("ur", "الینوائے"), ("uz", "Illinoys"), ("vi", "Illinois"), ("yo", "Illinois"), ("yo_BJ", "Illinois"), ("yue", "伊利奈省"), ("yue_Hans", "伊利奈省"), ("zh", "伊利诺伊州")]),
                        unofficial_name_list: ["Illinois"].to_vec(),
                    }
                ),
                (
                    "IN",
                    Subdivision{
                        name: "Indiana",
                        country_alpha2: Alpha2::US,
                        code: "IN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.2671941), longitude: Some(-86.1349019), max_latitude: Some(41.7606971), min_latitude: Some(37.7717419), max_longitude: Some(-84.784662), min_longitude: Some(-88.097892)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Indiana"), ("am", "ኢንዲያና"), ("ar", "إنديانا"), ("az", "İndiana"), ("be", "Штат Індыяна"), ("bg", "Индиана"), ("bn", "ইন\u{9cd}ডিয\u{9bc}\u{9be}ন\u{9be}"), ("bs", "Indiana"), ("ca", "Indiana"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄓\u{11128}𑄠𑄚"), ("ceb", "Indiana (estado)"), ("cs", "Indiana"), ("cy", "Indiana"), ("da", "Indiana"), ("de", "Indiana"), ("el", "Ιντιάνα"), ("en", "Indiana"), ("es", "Indiana"), ("et", "Indiana"), ("eu", "Indiana"), ("fa", "ایندیانا"), ("fi", "Indiana"), ("fr", "Indiana"), ("ga", "Indiana"), ("gl", "Indiana"), ("gu", "ઇન\u{acd}ડિયાના"), ("ha", "Indiana"), ("ha_NE", "Indiana"), ("he", "אינדיאנה"), ("hi", "इ\u{902}डियाना"), ("hr", "Indiana"), ("hu", "Indiana"), ("hy", "Ինդիանա"), ("id", "Indiana"), ("ig", "Ndiánà"), ("is", "Indiana"), ("it", "Indiana"), ("ja", "インディアナ州"), ("jv", "Indiana"), ("ka", "ინდიანა"), ("kk", "Индиана"), ("kn", "ಇಂಡ\u{cbf}ಯಾನಾ"), ("ko", "인디애나 주"), ("lt", "Indiana"), ("lv", "Indiāna"), ("mk", "Индијана"), ("ml", "ഇന\u{d4d}ത\u{d4d}യ\u{d3e}ന"), ("mn", "Индиана муж"), ("mr", "इ\u{902}डियाना"), ("ms", "Indiana"), ("my", "အင\u{103a}ဒ\u{102e}ယားနားပြည\u{103a}နယ\u{103a}"), ("nb", "Indiana"), ("ne", "इण\u{94d}डियाना"), ("nl", "Indiana"), ("no", "Indiana"), ("pa", "ਇ\u{a70}ਡਿਆਨਾ"), ("pl", "Indiana"), ("pt", "Indiana"), ("ro", "Indiana"), ("ru", "Индиана"), ("si", "ඉන\u{dca}ද\u{dd2}ය\u{dcf}න\u{dcf}"), ("sk", "Indiana"), ("sl", "Indiana"), ("sq", "Indiana"), ("sr", "Индијана"), ("sr_Latn", "Indijana"), ("sv", "Indiana"), ("sw", "Indiana"), ("ta", "இந\u{bcd}திய\u{bbe}ன\u{bbe}"), ("te", "ఇండ\u{c3f}య\u{c3e}న\u{c3e}"), ("th", "ร\u{e31}ฐอ\u{e34}นด\u{e35}แอนา"), ("tr", "Indiana"), ("uk", "Індіана"), ("ur", "انڈیانا"), ("uz", "Indiana"), ("vi", "Indiana"), ("yo", "Indiana"), ("yo_BJ", "Indiana"), ("yue", "印第安納州"), ("yue_Hans", "印第安纳州"), ("zh", "印第安纳州"), ("zu", "Indiana")]),
                        unofficial_name_list: ["Indiana"].to_vec(),
                    }
                ),
                (
                    "KS",
                    Subdivision{
                        name: "Kansas",
                        country_alpha2: Alpha2::US,
                        code: "KS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.011902), longitude: Some(-98.4842465), max_latitude: Some(40.0045419), min_latitude: Some(36.9930159), max_longitude: Some(-94.588387), min_longitude: Some(-102.0517688)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kansas"), ("am", "ካንሳስ"), ("ar", "كانساس"), ("az", "Kanzas"), ("be", "Штат Канзас"), ("bg", "Канзас"), ("bn", "ক\u{9cd}য\u{9be}ন\u{9cd}স\u{9be}স"), ("bs", "Kansas"), ("ca", "Kansas"), ("ccp", "𑄇𑄚\u{11134}𑄥𑄌\u{11134}"), ("ceb", "Kansas"), ("cs", "Kansas"), ("cy", "Kansas"), ("da", "Kansas"), ("de", "Kansas"), ("el", "Κάνσας"), ("en", "Kansas"), ("es", "Kansas"), ("et", "Kansas"), ("eu", "Kansas"), ("fa", "کانزاس"), ("fi", "Kansas"), ("fr", "Kansas"), ("ga", "Kansas"), ("gl", "Kansas"), ("gu", "ક\u{ac7}ન\u{acd}સાસ"), ("he", "קנזס"), ("hi", "क\u{947}न\u{94d}सास"), ("hr", "Kansas"), ("hu", "Kansas"), ("hy", "Կանզաս"), ("id", "Kansas"), ("ig", "Kánzạs"), ("is", "Kansas"), ("it", "Kansas"), ("ja", "カンザス州"), ("jv", "Kansas"), ("ka", "კანზასი"), ("kk", "Канзас"), ("kn", "ಕನ\u{ccd}ಸಾಸ\u{ccd}/ಕಾನ\u{ccd}ಸಾಸ\u{ccd}\u{200c}\u{200c}"), ("ko", "캔자스 주"), ("lt", "Kanzasas"), ("lv", "Kanzasa"), ("mk", "Канзас"), ("ml", "ക\u{d3e}ൻസസ\u{d4d}"), ("mn", "Канзас"), ("mr", "क\u{945}न\u{94d}सस"), ("ms", "Kansas"), ("my", "ကန\u{103a}းဆပ\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Kansas"), ("ne", "कन\u{94d}सास"), ("nl", "Kansas"), ("no", "Kansas"), ("pa", "ਕਾ\u{a02}ਸਸ"), ("pl", "Kansas"), ("pt", "Kansas"), ("ro", "Kansas"), ("ru", "Канзас"), ("si", "කැන\u{dca}ස\u{dcf}ස\u{dca}"), ("sk", "Kansas"), ("sl", "Kansas"), ("sq", "Kansas"), ("sr", "Канзас"), ("sr_Latn", "Kanzas"), ("sv", "Kansas"), ("sw", "Kansas"), ("ta", "கேன\u{bcd}சஸ\u{bcd}"), ("te", "క\u{c3e}న\u{c4d}స\u{c3e}స\u{c4d}"), ("th", "ร\u{e31}ฐแคนซ\u{e31}ส"), ("tr", "Kansas"), ("uk", "Канзас"), ("ur", "کنساس"), ("uz", "Kanzas"), ("vi", "Kansas"), ("yo", "Kansas"), ("yo_BJ", "Kansas"), ("yue", "干沙省"), ("yue_Hans", "干沙省"), ("zh", "堪薩斯州")]),
                        unofficial_name_list: ["Kansas"].to_vec(),
                    }
                ),
                (
                    "KY",
                    Subdivision{
                        name: "Kentucky",
                        country_alpha2: Alpha2::US,
                        code: "KY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.8393332), longitude: Some(-84.2700179), max_latitude: Some(39.147458), min_latitude: Some(36.4971289), max_longitude: Some(-81.9649708), min_longitude: Some(-89.5715089)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kentucky"), ("am", "ኬንታኪ"), ("ar", "كنتاكي"), ("az", "Kentukki"), ("be", "Штат Кентукі"), ("bg", "Кентъки"), ("bn", "কেন\u{9cd}ট\u{9be}কি"), ("bs", "Kentucky"), ("ca", "Kentucky"), ("ccp", "𑄇𑄬𑄚\u{11134}𑄑\u{1112a}𑄇\u{11128}"), ("ceb", "Kentucky"), ("cs", "Kentucky"), ("cy", "Kentucky"), ("da", "Kentucky"), ("de", "Kentucky"), ("el", "Κεντάκι"), ("en", "Kentucky"), ("es", "Kentucky"), ("et", "Kentucky"), ("eu", "Kentucky"), ("fa", "کنتاکی"), ("fi", "Kentucky"), ("fr", "Kentucky"), ("ga", "Kentucky"), ("gl", "Kentucky"), ("gu", "ક\u{ac7}ન\u{acd}ટકી"), ("he", "קנטקי"), ("hi", "क\u{947}न\u{94d}टकी"), ("hr", "Kentucky"), ("hu", "Kentucky"), ("hy", "Կենտուկի"), ("id", "Kentucky"), ("ig", "Kentórkị"), ("is", "Kentucky"), ("it", "Kentucky"), ("ja", "ケンタッキー州"), ("jv", "Kentucky"), ("ka", "კენტუკი"), ("kk", "Кентукки"), ("kn", "ಕ\u{cc6}ಂಟುಕ\u{cbf}"), ("ko", "켄터키 주"), ("lt", "Kentukis"), ("lv", "Kentuki"), ("mk", "Кентаки"), ("ml", "കെന\u{d4d}റക\u{d4d}കി"), ("mn", "Кентакки"), ("mr", "क\u{947}\u{902}टकी"), ("ms", "Kentucky"), ("my", "ကင\u{103a}တပ\u{103a}က\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "Kentucky"), ("ne", "क\u{947}न\u{94d}टकी"), ("nl", "Kentucky"), ("no", "Kentucky"), ("pa", "ਕਿ\u{a70}ਟਕੀ"), ("pl", "Kentucky"), ("pt", "Kentucky"), ("ro", "Kentucky"), ("ru", "Кентукки"), ("si", "කෙන\u{dca}ච\u{dd4}ක\u{dd2}"), ("sk", "Kentucky"), ("sl", "Kentucky"), ("sq", "Kentucky"), ("sr", "Кентаки"), ("sr_Latn", "Kentaki"), ("sv", "Kentucky"), ("sw", "Kentucky"), ("ta", "கென\u{bcd}டக\u{bcd}கி"), ("te", "క\u{c46}ంటక\u{c40}"), ("th", "ร\u{e31}ฐเคนท\u{e31}กก\u{e35}"), ("tr", "Kentucky"), ("uk", "Кентуккі"), ("ur", "کینٹکی"), ("uz", "Kentukki"), ("vi", "Kentucky"), ("yo", "Kentucky"), ("yo_BJ", "Kentucky"), ("yue", "肯塔基州"), ("yue_Hans", "肯塔基州"), ("zh", "肯塔基州")]),
                        unofficial_name_list: ["Kentucky"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "Louisiana",
                        country_alpha2: Alpha2::US,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(30.9842977), longitude: Some(-91.96233269999999), max_latitude: Some(33.019544), min_latitude: Some(28.9254296), max_longitude: Some(-88.8162401), min_longitude: Some(-94.043352)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Louisiana"), ("am", "ሉዊዚያና"), ("ar", "لويزيانا"), ("az", "Luiziana"), ("be", "Штат Луізіяна"), ("bg", "Луизиана"), ("bn", "ল\u{9c1}ইজিয\u{9bc}\u{9be}ন\u{9be}"), ("bs", "Louisiana"), ("ca", "Louisiana"), ("ccp", "𑄣\u{1112d}\u{1112a}𑄎\u{11128}𑄠𑄚"), ("ceb", "Louisiana (estado)"), ("cs", "Louisiana"), ("cy", "Louisiana"), ("da", "Louisiana"), ("de", "Louisiana"), ("el", "Λουιζιάνα"), ("en", "Louisiana"), ("es", "Luisiana"), ("et", "Louisiana osariik"), ("eu", "Louisiana"), ("fa", "لوئیزیانا"), ("fi", "Louisiana"), ("fr", "Louisiane"), ("ga", "Louisiana"), ("gl", "Luisiana - Louisiana"), ("gu", "લ\u{acd}ય\u{ac1}ઇસિયાના"), ("he", "לואיזיאנה"), ("hi", "ल\u{941}ईज\u{93c}ियाना"), ("hr", "Louisiana"), ("hu", "Louisiana"), ("hy", "Լուիզիանա"), ("id", "Louisiana"), ("ig", "Luwisiánà"), ("is", "Louisiana"), ("it", "Louisiana"), ("ja", "ルイジアナ州"), ("jv", "Louisiana"), ("ka", "ლუიზიანა"), ("kk", "Луизиана"), ("kn", "ಲ\u{cc2}ಯ\u{cbf}ಸ\u{cbf}ಯಾನ"), ("ko", "루이지애나 주"), ("lt", "Luiziana"), ("lv", "Luiziāna"), ("mk", "Луизијана"), ("ml", "ല\u{d41}യീസിയ\u{d3e}ന"), ("mn", "Луйзиана"), ("mr", "ल\u{941}ईझियाना"), ("ms", "Louisiana"), ("my", "လ\u{1030}ဝ\u{102e}စ\u{102e}ယားနားပြည\u{103a}နယ\u{103a}"), ("nb", "Louisiana"), ("ne", "ल\u{941}जियाना"), ("nl", "Louisiana"), ("no", "Louisiana"), ("pa", "ਲ\u{a42}ਈਜ\u{a3c}ੀਆਨਾ"), ("pl", "Luizjana"), ("pt", "Luisiana"), ("ro", "Louisiana"), ("ru", "Луизиана"), ("si", "ල\u{dd4}ස\u{dd2}ය\u{dcf}න\u{dcf}"), ("sk", "Louisiana"), ("sl", "Louisiana"), ("sq", "Louisiana"), ("sr", "Луизијана"), ("sr_Latn", "Luizijana"), ("sv", "Louisiana"), ("sw", "Louisiana"), ("ta", "லூசிய\u{bbe}ன\u{bbe}"), ("te", "లూస\u{c3f}య\u{c3e}న\u{c3e}"), ("th", "ร\u{e31}ฐล\u{e38}ยเซ\u{e35}ยนา"), ("tr", "Louisiana"), ("uk", "Луїзіана"), ("ur", "لوزیانا"), ("uz", "Luiziana"), ("vi", "Louisiana"), ("yo", "Louisiana"), ("yo_BJ", "Louisiana"), ("yue", "路易斯安那州"), ("yue_Hans", "路易斯安那州"), ("zh", "路易斯安那州")]),
                        unofficial_name_list: ["Louisiana"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "Massachusetts",
                        country_alpha2: Alpha2::US,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4072107), longitude: Some(-71.3824374), max_latitude: Some(42.88679), min_latitude: Some(41.2390448), max_longitude: Some(-69.92855109999999), min_longitude: Some(-73.5081419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Massachusetts"), ("am", "ማሣቹሰትስ"), ("ar", "ماساتشوستس"), ("az", "Massaçusets"), ("be", "Масачусетс"), ("bg", "Масачузетс"), ("bn", "ম\u{9cd}য\u{9be}স\u{9be}চ\u{9c1}সেট\u{9cd}\u{200c}স"), ("bs", "Massachusetts"), ("ca", "Massachusetts"), ("ccp", "𑄟𑄥𑄌\u{1112a}𑄥𑄬𑄖\u{11134}𑄌\u{11134}"), ("ceb", "Massachusetts"), ("cs", "Massachusetts"), ("cy", "Massachusetts"), ("da", "Massachusetts"), ("de", "Massachusetts"), ("el", "Μασαχουσέτη"), ("en", "Massachusetts"), ("es", "Massachusetts"), ("et", "Massachusetts"), ("eu", "Massachusetts"), ("fa", "ماساچوست"), ("fi", "Massachusetts"), ("fr", "Massachusetts"), ("ga", "Massachusetts"), ("gl", "Massachusetts"), ("gu", "મ\u{ac7}સ\u{ac7}ચ\u{acd}ય\u{ac1}સ\u{ac7}ટ\u{acd}સ"), ("he", "מסצ׳וסטס"), ("hi", "म\u{948}साच\u{942}सिट\u{94d}स"), ("hr", "Massachusetts"), ("hu", "Massachusetts"), ("hy", "Մասաչուսեթս"), ("id", "Massachusetts"), ("ig", "Másáchusẹts"), ("is", "Massachusetts"), ("it", "Massachusetts"), ("ja", "マサチューセッツ州"), ("jv", "Massachusetts"), ("ka", "მასაჩუსეტსი"), ("kk", "Массачусетс"), ("kn", "ಮ\u{ccd}ಯಾಸಚ\u{cc2}ಸ\u{cc6}ಟ\u{ccd}ಸ\u{ccd}"), ("ko", "매사추세츠 주"), ("ky", "Массачусетс"), ("lt", "Masačusetsas"), ("lv", "Masačūsetsa"), ("mk", "Масачусетс"), ("ml", "മസ\u{d3e}ച\u{d4d}യ\u{d41}സെറ\u{d4d}റ\u{d4d}സ\u{d4d}"), ("mn", "Массачусеттс"), ("mr", "म\u{945}स\u{947}च\u{94d}य\u{941}स\u{947}ट\u{94d}स"), ("ms", "Massachusetts"), ("my", "မက\u{103a}ဆာချ\u{1030}းဆက\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Massachusetts"), ("ne", "म\u{94d}यास\u{947}च\u{941}स\u{947}ट\u{94d}स"), ("nl", "Massachusetts"), ("no", "Massachusetts"), ("pa", "ਮ\u{a48}ਸਾਚ\u{a42}ਸਟਸ"), ("pl", "Massachusetts"), ("pt", "Massachusetts"), ("ro", "Massachusetts"), ("ru", "Массачусетс"), ("sd", "مئسچوسٽس"), ("si", "මැසච\u{dd4}සෙට\u{dca}ස\u{dca}"), ("sk", "Massachusetts"), ("sl", "Massachusetts"), ("sq", "Massachusetts"), ("sr", "Масачусетс"), ("sr_Latn", "Masačusets"), ("sv", "Massachusetts"), ("sw", "Massachusetts"), ("ta", "ம\u{bbe}சச\u{bcd}சூசெட\u{bcd}ஸ\u{bcd}"), ("te", "మస\u{c3e}చూస\u{c46}ట\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐแมสซาช\u{e39}เซตส\u{e4c}"), ("tr", "Massachusetts"), ("uk", "Массачусетс"), ("ur", "میساچوسٹس"), ("uz", "Massachusetts"), ("vi", "Massachusetts"), ("yo", "Massachusetts"), ("yo_BJ", "Massachusetts"), ("yue", "麻省"), ("yue_Hans", "麻省"), ("zh", "麻薩諸塞州"), ("zu", "Massachusetts")]),
                        unofficial_name_list: ["Massachusetts"].to_vec(),
                    }
                ),
                (
                    "MD",
                    Subdivision{
                        name: "Maryland",
                        country_alpha2: Alpha2::US,
                        code: "MD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.0457549), longitude: Some(-76.64127119999999), max_latitude: Some(39.723037), min_latitude: Some(37.8895135), max_longitude: Some(-75.04918119999999), min_longitude: Some(-79.4876511)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Maryland"), ("am", "ሜሪላንድ"), ("ar", "ماريلند"), ("az", "Merilend"), ("be", "Штат Мэрыленд"), ("bg", "Мериленд"), ("bn", "মেরিল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Maryland"), ("ca", "Maryland"), ("ccp", "𑄟𑄬𑄢\u{11128}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Maryland (estado)"), ("cs", "Maryland"), ("cy", "Maryland"), ("da", "Maryland"), ("de", "Maryland"), ("el", "Μέριλαντ"), ("en", "Maryland"), ("es", "Maryland"), ("et", "Maryland"), ("eu", "Maryland"), ("fa", "مریلند"), ("fi", "Maryland"), ("fr", "Maryland"), ("ga", "Maryland"), ("gl", "Maryland"), ("gu", "મ\u{ac7}રીલ\u{ac7}ન\u{acd}ડ"), ("he", "מרילנד"), ("hi", "म\u{948}रील\u{948}\u{902}ड"), ("hr", "Maryland"), ("hu", "Maryland"), ("hy", "Մերիլենդ"), ("id", "Maryland"), ("ig", "Mérílạnd"), ("is", "Maryland"), ("it", "Maryland"), ("ja", "メリーランド州"), ("jv", "Maryland"), ("ka", "მერილენდი"), ("kk", "Мэриленд"), ("kn", "ಮೇರ\u{cbf}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "메릴랜드 주"), ("lt", "Merilandas"), ("lv", "Mērilenda"), ("mk", "Мериленд"), ("ml", "മെരില\u{d3e}ൻ\u{200c}ഡ\u{d4d}"), ("mn", "Мэрилэнд"), ("mr", "म\u{947}रील\u{901}ड"), ("ms", "Maryland"), ("my", "မေရ\u{102e}လန\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Maryland"), ("ne", "म\u{947}रिल\u{94d}याण\u{94d}ड"), ("nl", "Maryland"), ("no", "Maryland"), ("pa", "ਮ\u{a48}ਰੀਲ\u{a48}\u{a02}ਡ"), ("pl", "Maryland"), ("pt", "Maryland"), ("ro", "Maryland"), ("ru", "Мэриленд"), ("si", "මේර\u{dd2}ලන\u{dca}ඩ\u{dca}"), ("sk", "Maryland"), ("sl", "Maryland"), ("sq", "Maryland"), ("sr", "Мериленд"), ("sr_Latn", "Merilend"), ("sv", "Maryland"), ("sw", "Maryland"), ("ta", "மேரில\u{bbe}ந\u{bcd}து"), ("te", "మ\u{c47}ర\u{c40}ల\u{c4d}య\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐแมร\u{e34}แลนด\u{e4c}"), ("tr", "Maryland"), ("uk", "Меріленд"), ("ur", "میری لینڈ"), ("uz", "Maryland"), ("vi", "Maryland"), ("yo", "Maryland"), ("yo_BJ", "Maryland"), ("yue", "馬利蘭州"), ("yue_Hans", "马利兰州"), ("zh", "马里兰州"), ("zu", "Maryland")]),
                        unofficial_name_list: ["Maryland"].to_vec(),
                    }
                ),
                (
                    "ME",
                    Subdivision{
                        name: "Maine",
                        country_alpha2: Alpha2::US,
                        code: "ME",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.253783), longitude: Some(-69.4454689), max_latitude: Some(47.4596734), min_latitude: Some(42.9747059), max_longitude: Some(-66.94539720000002), min_longitude: Some(-71.08433409999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Maine"), ("am", "መይን"), ("ar", "مين"), ("az", "Men"), ("be", "Штат Мэн"), ("bg", "Мейн"), ("bn", "মেইন"), ("bs", "Maine"), ("ca", "Maine"), ("ccp", "𑄟\u{1112d}𑄚𑄬"), ("ceb", "Maine"), ("cs", "Maine"), ("cy", "Maine"), ("da", "Maine"), ("de", "Maine"), ("el", "Μέιν"), ("en", "Maine"), ("es", "Maine"), ("et", "Maine"), ("eu", "Maine"), ("fa", "مین"), ("fi", "Maine"), ("fr", "Maine"), ("ga", "Maine"), ("gl", "Maine"), ("gu", "મ\u{ac8}ન"), ("he", "מיין"), ("hi", "म\u{947}न"), ("hr", "Maine"), ("hu", "Maine"), ("hy", "Մեն"), ("id", "Maine"), ("ig", "Mën"), ("is", "Maine"), ("it", "Maine"), ("ja", "メイン州"), ("jv", "Maine"), ("ka", "მეინი"), ("kk", "Мэн"), ("kn", "ಮೈನ\u{cc6}"), ("ko", "메인 주"), ("lt", "Meinas"), ("lv", "Mena"), ("mk", "Мејн"), ("ml", "മെയ\u{d4d}ൻ"), ("mn", "Мэн"), ("mr", "म\u{947}न"), ("ms", "Maine"), ("my", "မ\u{102d}န\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Maine"), ("ne", "म\u{947}न"), ("nl", "Maine"), ("no", "Maine"), ("pa", "ਮ\u{a47}ਨ"), ("pl", "Maine"), ("pt", "Maine"), ("ro", "Maine"), ("ru", "Мэн"), ("si", "මය\u{dd2}නේ"), ("sk", "Maine"), ("sl", "Maine"), ("sq", "Maine"), ("sr", "Мејн"), ("sr_Latn", "Mejn"), ("sv", "Maine"), ("sw", "Maine"), ("ta", "மேய\u{bcd}ன\u{bcd}"), ("te", "మ\u{c46}య\u{c3f}న\u{c4d}"), ("th", "ร\u{e31}ฐเมน"), ("tr", "Maine"), ("uk", "Мен"), ("ur", "مینے"), ("uz", "Meyn"), ("vi", "Maine"), ("yo", "Maine"), ("yo_BJ", "Maine"), ("yue", "緬因州"), ("yue_Hans", "缅因州"), ("zh", "缅因州")]),
                        unofficial_name_list: ["Maine"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "Michigan",
                        country_alpha2: Alpha2::US,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.3148443), longitude: Some(-85.60236429999999), max_latitude: Some(48.1986461), min_latitude: Some(41.696118), max_longitude: Some(-82.3990376), min_longitude: Some(-90.4181358)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Michigan"), ("am", "ሚሺጋን"), ("ar", "ميشيغان"), ("az", "Miçiqan"), ("be", "Штат Мічыган"), ("bg", "Мичиган"), ("bn", "মিশিগ\u{9be}ন"), ("bs", "Michigan"), ("ca", "Michigan"), ("ccp", "𑄟\u{11128}𑄥\u{11128}𑄉𑄚\u{11134}"), ("ceb", "Michigan"), ("cs", "Michigan"), ("cy", "Michigan"), ("da", "Michigan"), ("de", "Michigan"), ("el", "Μίσιγκαν"), ("en", "Michigan"), ("es", "Míchigan"), ("et", "Michigan"), ("eu", "Michigan"), ("fa", "میشیگان"), ("fi", "Michigan"), ("fr", "Michigan"), ("ga", "Michigan"), ("gl", "Míchigan"), ("gu", "મિચિગન"), ("he", "מישיגן"), ("hi", "मिशिगन"), ("hr", "Michigan"), ("hu", "Michigan"), ("hy", "Միչիգան"), ("id", "Michigan"), ("ig", "Michigan"), ("is", "Michigan"), ("it", "Michigan"), ("ja", "ミシガン州"), ("jv", "Michigan"), ("ka", "მიჩიგანის შტატი"), ("kk", "Мичиган"), ("kn", "ಮ\u{cbf}ಚ\u{cbf}ಗನ\u{ccd}"), ("ko", "미시간 주"), ("lo", "ລ\u{eb1}ດມ\u{eb4}ຊ\u{eb4}ແກນ"), ("lt", "Mičiganas"), ("lv", "Mičigana"), ("mk", "Мичиген"), ("ml", "മിഷിഗൺ"), ("mn", "Мичиган"), ("mr", "मिशिगन"), ("ms", "Michigan"), ("my", "မ\u{102e}ချ\u{102e}ဂန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Michigan"), ("ne", "मिचिगन"), ("nl", "Michigan"), ("no", "Michigan"), ("pa", "ਮਿਸ\u{a3c}ੀਗਨ"), ("pl", "Michigan"), ("pt", "Michigan"), ("ro", "Michigan"), ("ru", "Мичиган"), ("si", "ම\u{dd2}ච\u{dd2}ගන\u{dca}"), ("sk", "Michigan"), ("sl", "Michigan"), ("sq", "Michigan"), ("sr", "Мичиген"), ("sr_Latn", "Mičigen"), ("sv", "Michigan"), ("sw", "Michigan"), ("ta", "மிச\u{bcd}சிகன\u{bcd}"), ("te", "మ\u{c3f}ష\u{c3f}గన\u{c4d}"), ("th", "ร\u{e31}ฐม\u{e34}ช\u{e34}แกน"), ("tr", "Michigan"), ("uk", "Мічиган"), ("ur", "مشی گن"), ("uz", "Michigan"), ("vi", "Michigan"), ("yo", "Michigan"), ("yo_BJ", "Michigan"), ("yue", "密芝根州"), ("yue_Hans", "密芝根州"), ("zh", "密歇根州")]),
                        unofficial_name_list: ["Michigan"].to_vec(),
                    }
                ),
                (
                    "MN",
                    Subdivision{
                        name: "Minnesota",
                        country_alpha2: Alpha2::US,
                        code: "MN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.729553), longitude: Some(-94.6858998), max_latitude: Some(49.3828204), min_latitude: Some(43.499361), max_longitude: Some(-89.49183339999999), min_longitude: Some(-97.23919579999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Minnesota"), ("am", "ሚንሶታ"), ("ar", "مينيسوتا"), ("az", "Minnesota"), ("be", "Штат Мінесота"), ("bg", "Минесота"), ("bn", "মিনেসোট\u{9be}"), ("bs", "Minnesota"), ("ca", "Minnesota"), ("ccp", "𑄟\u{11128}𑄚𑄬𑄥\u{1112e}𑄑"), ("ceb", "Minnesota"), ("cs", "Minnesota"), ("cy", "Minnesota"), ("da", "Minnesota"), ("de", "Minnesota"), ("el", "Μινεσότα"), ("en", "Minnesota"), ("es", "Minnesota"), ("et", "Minnesota"), ("eu", "Minnesota"), ("fa", "مینه\u{200c}سوتا"), ("fi", "Minnesota"), ("fr", "Minnesota"), ("ga", "Minnesota"), ("gl", "Minnesota"), ("gu", "મિન\u{ac7}સોટા"), ("he", "מינסוטה"), ("hi", "मिन\u{947}सोटा"), ("hr", "Minnesota"), ("hu", "Minnesota"), ("hy", "Մինեսոտա"), ("id", "Minnesota"), ("ig", "Minésótạ"), ("is", "Minnesota"), ("it", "Minnesota"), ("ja", "ミネソタ州"), ("jv", "Minnesota"), ("ka", "მინესოტა"), ("kk", "Миннесота"), ("kn", "ಮ\u{cbf}ನ\u{ccd}ನೇಸೋಟ"), ("ko", "미네소타 주"), ("lt", "Minesota"), ("lv", "Minesota"), ("mk", "Минесота"), ("ml", "മിനസോട\u{d4d}ട"), ("mn", "Миннесота"), ("mr", "मिन\u{947}सोटा"), ("ms", "Minnesota"), ("my", "မင\u{103a}န\u{102e}ဆ\u{102d}\u{102f}းတားပြည\u{103a}နယ\u{103a}"), ("nb", "Minnesota"), ("ne", "मिनिसोटा"), ("nl", "Minnesota"), ("no", "Minnesota"), ("pa", "ਮਿਨ\u{a47}ਸ\u{a4b}ਟਾ"), ("pl", "Minnesota"), ("pt", "Minnesota"), ("ro", "Minnesota"), ("ru", "Миннесота"), ("si", "ම\u{dd2}න\u{dca}නෙසොට\u{dcf}"), ("sk", "Minnesota"), ("sl", "Minnesota"), ("so", "Minnesota"), ("sq", "Minnesota"), ("sr", "Минесота"), ("sr_Latn", "Minesota"), ("sv", "Minnesota"), ("sw", "Minnesota"), ("ta", "மினசோட\u{bcd}ட\u{bbe}"), ("te", "మ\u{c3f}న\u{c4d}నస\u{c4b}ట\u{c3e}"), ("th", "ร\u{e31}ฐม\u{e34}นน\u{e34}โซตา"), ("tr", "Minnesota"), ("uk", "Міннесота"), ("ur", "مینیسوٹا"), ("uz", "Minnesota"), ("vi", "Minnesota"), ("yo", "Minnesota"), ("yo_BJ", "Minnesota"), ("yue", "明尼蘇達州"), ("yue_Hans", "明尼苏达州"), ("zh", "明尼蘇達州")]),
                        unofficial_name_list: ["Minnesota"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "Missouri",
                        country_alpha2: Alpha2::US,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9642529), longitude: Some(-91.8318334), max_latitude: Some(40.61364), min_latitude: Some(35.9956829), max_longitude: Some(-89.09949399999999), min_longitude: Some(-95.774704)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Missouri"), ("am", "ሚዙሪ"), ("ar", "ميزوري"), ("az", "Missuri"), ("be", "Штат Місуры"), ("bg", "Мисури"), ("bn", "মিসৌরি"), ("bs", "Missouri"), ("ca", "Missouri"), ("ccp", "𑄟\u{11128}𑄥\u{1112f}𑄢\u{11128}"), ("ceb", "Missouri"), ("cs", "Missouri"), ("cy", "Missouri"), ("da", "Missouri"), ("de", "Missouri"), ("el", "Μιζούρι"), ("en", "Missouri"), ("es", "Misuri"), ("et", "Missouri osariik"), ("eu", "Missouri"), ("fa", "میزوری"), ("fi", "Missouri"), ("fr", "Missouri"), ("ga", "Missouri"), ("gl", "Misuri"), ("gu", "મિઝોરી"), ("he", "מיזורי"), ("hi", "मिसौरी"), ("hr", "Missouri"), ("hu", "Missouri"), ("hy", "Միսսուրի"), ("id", "Missouri"), ("ig", "Mizurị"), ("is", "Missouri"), ("it", "Missouri"), ("ja", "ミズーリ州"), ("jv", "Missouri"), ("ka", "მისურის შტატი"), ("kk", "Миссури"), ("kn", "ಮ\u{cbf}ಸ\u{ccc}ರ\u{cbf}"), ("ko", "미주리 주"), ("lt", "Misūris"), ("lv", "Misūri"), ("mk", "Мисури"), ("ml", "മിസോറി"), ("mn", "Миссури"), ("mr", "मिस\u{942}री"), ("ms", "Missouri"), ("my", "မစ\u{103a}ဆ\u{102d}\u{102f}ရ\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "Missouri"), ("ne", "मिसौरी"), ("nl", "Missouri"), ("no", "Missouri"), ("pa", "ਮਿਜ\u{a3c}\u{a42}ਰੀ"), ("pl", "Missouri"), ("pt", "Missouri"), ("ro", "Missouri"), ("ru", "Миссури"), ("si", "ම\u{dd2}ස\u{dd4}ර\u{dd2}"), ("sk", "Missouri"), ("sl", "Misuri"), ("sq", "Missouri"), ("sr", "Мисури"), ("sr_Latn", "Misuri"), ("sv", "Missouri"), ("sw", "Missouri"), ("ta", "மிசூரி"), ("te", "మ\u{c3f}స\u{c4d}స\u{c4b}ర\u{c40}"), ("th", "ร\u{e31}ฐม\u{e34}สซ\u{e39}ร\u{e35}"), ("tr", "Missouri"), ("uk", "Міссурі"), ("ur", "مسوری"), ("uz", "Missuri"), ("vi", "Missouri"), ("yo", "Ìpínlẹ\u{300} Missouri"), ("yo_BJ", "Ìpínlɛ\u{300} Missouri"), ("yue", "美疏利省"), ("yue_Hans", "美疏利省"), ("zh", "密蘇里州")]),
                        unofficial_name_list: ["Missouri"].to_vec(),
                    }
                ),
                (
                    "MP",
                    Subdivision{
                        name: "Northern Mariana Islands",
                        country_alpha2: Alpha2::US,
                        code: "MP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.1466435), longitude: Some(145.7023038), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄟𑄬𑄢\u{11128}𑄠𑄚 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("en", "Northern Mariana Islands")]),
                        unofficial_name_list: ["Northern Mariana Islands"].to_vec(),
                    }
                ),
                (
                    "MS",
                    Subdivision{
                        name: "Mississippi",
                        country_alpha2: Alpha2::US,
                        code: "MS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.3546679), longitude: Some(-89.3985283), max_latitude: Some(34.9961091), min_latitude: Some(30.1741032), max_longitude: Some(-88.0994303), min_longitude: Some(-91.6550089)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Mississippi"), ("am", "ሚሲሲፒ"), ("ar", "مسيسيبي"), ("az", "Missisipi"), ("be", "Штат Місісіпі"), ("bg", "Мисисипи"), ("bn", "মিসিসিপি"), ("bs", "Mississippi"), ("ca", "Mississipí"), ("ccp", "𑄟\u{11128}𑄥\u{11128}𑄥\u{11128}𑄛\u{11128}"), ("ceb", "Mississippi"), ("cs", "Mississippi"), ("cy", "Mississippi"), ("da", "Mississippi"), ("de", "Mississippi"), ("el", "Μισισίπι"), ("en", "Mississippi"), ("es", "Misisipi"), ("et", "Mississippi osariik"), ("eu", "Mississippi"), ("fa", "میسیسیپی"), ("fi", "Mississippi"), ("fr", "Mississippi"), ("ga", "Mississippi"), ("gl", "Mississippi"), ("gu", "મિસિસિપી"), ("he", "מיסיסיפי"), ("hi", "मिसिसिप\u{94d}पी"), ("hr", "Mississippi"), ("hu", "Mississippi"), ("hy", "Միսսիսիպի"), ("id", "Mississippi"), ("ig", "Mississippi"), ("is", "Mississippi"), ("it", "Mississippi"), ("ja", "ミシシッピ州"), ("jv", "Mississippi"), ("ka", "მისისიპის შტატი"), ("kk", "Миссисипи"), ("kn", "ಮ\u{cbf}ಸ\u{ccd}ಸ\u{cbf}ಸ\u{ccd}ಸ\u{cbf}ಪ\u{ccd}ಪ\u{cbf}"), ("ko", "미시시피 주"), ("lt", "Misisipė"), ("lv", "Misisipi"), ("mk", "Мисисипи"), ("ml", "മിസിസിപ\u{d4d}പി"), ("mn", "Миссиссиппи"), ("mr", "मिसिसिपी"), ("ms", "Mississippi"), ("my", "မစ\u{1039}စစ\u{1039}စပ\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "Mississippi"), ("ne", "मिसिसिपी"), ("nl", "Mississippi"), ("no", "Mississippi"), ("pa", "ਮਿਸੀਸਿ\u{a71}ਪੀ"), ("pl", "Missisipi"), ("pt", "Mississippi"), ("ro", "Mississippi"), ("ru", "Миссисипи"), ("si", "ම\u{dd2}ස\u{dd2}ස\u{dd2}ප\u{dd2}"), ("sk", "Mississippi"), ("sl", "Misisipi"), ("sq", "Mississippi"), ("sr", "Мисисипи"), ("sr_Latn", "Misisipi"), ("sv", "Mississippi"), ("sw", "Mississippi"), ("ta", "மிசிசிப\u{bcd}பி"), ("te", "మ\u{c3f}స\u{c3f}స\u{c3f}ప\u{c40}"), ("th", "ร\u{e31}ฐม\u{e34}สซ\u{e34}สซ\u{e34}ปป\u{e35}"), ("tr", "Mississippi"), ("uk", "Міссісіпі"), ("ur", "مسیسپی"), ("uz", "Mississippi"), ("vi", "Mississippi"), ("yo", "Ìpínlẹ\u{300} Mississippi"), ("yo_BJ", "Ìpínlɛ\u{300} Mississippi"), ("yue", "密西西比州"), ("yue_Hans", "密西西比州"), ("zh", "密西西比州")]),
                        unofficial_name_list: ["Mississippi"].to_vec(),
                    }
                ),
                (
                    "MT",
                    Subdivision{
                        name: "Montana",
                        country_alpha2: Alpha2::US,
                        code: "MT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8796822), longitude: Some(-110.3625658), max_latitude: Some(49.00139), min_latitude: Some(44.3582089), max_longitude: Some(-104.039563), min_longitude: Some(-116.050003)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Montana"), ("am", "ሞንታና"), ("ar", "مونتانا"), ("az", "Montana"), ("be", "Штат Мантана"), ("bg", "Монтана"), ("bn", "মন\u{9cd}ট\u{9be}ন\u{9be}"), ("bs", "Montana"), ("ca", "Montana"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄑𑄚"), ("ceb", "Montana (estado)"), ("cs", "Montana"), ("cy", "Montana"), ("da", "Montana"), ("de", "Montana"), ("el", "Μοντάνα"), ("en", "Montana"), ("es", "Montana"), ("et", "Montana"), ("eu", "Montana"), ("fa", "ایالت مونتانا"), ("fi", "Montana"), ("fr", "Montana"), ("ga", "Montana"), ("gl", "Montana"), ("gu", "મોન\u{acd}ટાના"), ("ha", "Montana"), ("ha_NE", "Montana"), ("he", "מונטנה"), ("hi", "मोन\u{94d}टाना"), ("hr", "Montana"), ("hu", "Montana"), ("hy", "Մոնտանա"), ("id", "Montana"), ("ig", "Montana"), ("is", "Montana"), ("it", "Montana"), ("ja", "モンタナ州"), ("jv", "Montana"), ("ka", "მონტანა"), ("kk", "Монтана"), ("kn", "ಮೊಂಟಾನಾ"), ("ko", "몬태나 주"), ("ky", "Монтана"), ("lt", "Montana"), ("lv", "Montāna"), ("mk", "Монтана"), ("ml", "മൊണ\u{d4d}ട\u{d3e}ന"), ("mr", "मो\u{902}टाना"), ("ms", "Montana"), ("my", "မ\u{103d}န\u{103a}တားနားပြည\u{103a}နယ\u{103a}"), ("nb", "Montana"), ("ne", "मोन\u{94d}टाना"), ("nl", "Montana"), ("no", "Montana"), ("pa", "ਮ\u{a4b}\u{a02}ਟਾਨਾ"), ("pl", "Montana"), ("pt", "Montana"), ("ro", "Montana"), ("ru", "Монтана"), ("si", "මොන\u{dca}ට\u{dcf}න\u{dcf}"), ("sk", "Montana"), ("sl", "Montana"), ("sq", "Montana"), ("sr", "Монтана"), ("sr_Latn", "Montana"), ("sv", "Montana"), ("sw", "Montana"), ("ta", "மொன\u{bcd}ட\u{bbe}ன\u{bbe}"), ("te", "మ\u{c4a}ంట\u{c3e}న\u{c3e}"), ("th", "ร\u{e31}ฐมอนแทนา"), ("tr", "Montana"), ("uk", "Монтана"), ("ur", "مونٹانا"), ("uz", "Montana"), ("vi", "Montana"), ("yo", "Montana"), ("yo_BJ", "Montana"), ("yue", "望丹拿省"), ("yue_Hans", "望丹拿省"), ("zh", "蒙大拿州")]),
                        unofficial_name_list: ["Montana"].to_vec(),
                    }
                ),
                (
                    "NC",
                    Subdivision{
                        name: "North Carolina",
                        country_alpha2: Alpha2::US,
                        code: "NC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.7595731), longitude: Some(-79.01929969999999), max_latitude: Some(36.5881568), min_latitude: Some(33.8409689), max_longitude: Some(-75.4599515), min_longitude: Some(-84.32186899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Carolina"), ("am", "ኖርዝ ካሮላይና"), ("ar", "كارولاينا الشمالية"), ("az", "Şimali Karolina"), ("be", "Штат Паўночная Караліна"), ("bg", "Северна Каролина"), ("bn", "নর\u{9cd}থ ক\u{9cd}য\u{9be}রোল\u{9be}ইন\u{9be}"), ("bs", "Sjeverna Karolina"), ("ca", "Carolina del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄇\u{11133}𑄠𑄢\u{1112e}𑄣\u{11128}𑄚"), ("ceb", "North Carolina"), ("cs", "Severní Karolína"), ("cy", "Gogledd Carolina"), ("da", "North Carolina"), ("de", "North Carolina"), ("el", "Βόρεια Καρολίνα"), ("en", "North Carolina"), ("es", "Carolina del Norte"), ("et", "Põhja-Carolina"), ("eu", "Ipar Carolina"), ("fa", "کارولینای شمالی"), ("fi", "Pohjois-Carolina"), ("fr", "Caroline du Nord"), ("ga", "Carolina Thuaidh"), ("gl", "Carolina do Norte - North Carolina"), ("gu", "નોર\u{acd}થ ક\u{ac7}રોલીના"), ("he", "קרוליינה הצפונית"), ("hi", "उत\u{94d}तरी क\u{947}रोलिना"), ("hr", "Sjeverna Karolina"), ("hu", "Észak-Karolina"), ("hy", "Հյուսիսային Կարոլինա"), ("id", "Carolina Utara"), ("ig", "Nort Kárólínạ"), ("is", "Norður-Karólína"), ("it", "Carolina del Nord"), ("ja", "ノースカロライナ州"), ("jv", "North Carolina"), ("ka", "ჩრდილოეთი კაროლინა"), ("kk", "Солтүстік Каролина"), ("kn", "ಉತ\u{ccd}ತರ ಕ\u{cc6}ರೊಲೀನ"), ("ko", "노스캐롤라이나 주"), ("lt", "Šiaurės Karolina"), ("lv", "Ziemeļkarolīna"), ("mk", "Северна Каролина"), ("ml", "വടക\u{d4d}കൻ കരൊലൈന"), ("mr", "नॉर\u{94d}थ क\u{945}रोलिना"), ("ms", "Carolina Utara"), ("my", "မြောက\u{103a}ကယ\u{103a}ရ\u{102d}\u{102f}လ\u{102d}\u{102f}င\u{103a}းနားပြည\u{103a}နယ\u{103a}"), ("nb", "Nord-Carolina"), ("ne", "नर\u{94d}थ क\u{94d}यारोलाइना"), ("nl", "North Carolina"), ("no", "Nord-Carolina"), ("pa", "ਉ\u{a71}ਤਰੀ ਕ\u{a48}ਰ\u{a4b}ਲੀਨਾ"), ("pl", "Karolina Północna"), ("pt", "Carolina do Norte"), ("ro", "Carolina de Nord"), ("ru", "Северная Каролина"), ("si", "උත\u{dd4}ර\u{dd4} කැරොල\u{dd2}න\u{dcf}"), ("sk", "Severná Karolína"), ("sl", "Severna Karolina"), ("sq", "North Carolina"), ("sr", "Северна Каролина"), ("sr_Latn", "Severna Karolina"), ("sv", "North Carolina"), ("sw", "North Carolina"), ("ta", "வட கரொலைன\u{bbe}"), ("te", "ఉత\u{c4d}తర కర\u{c4a}ల\u{c3f}న\u{c3e}"), ("th", "ร\u{e31}ฐนอร\u{e4c}ทแคโรไลนา"), ("tr", "Kuzey Karolina"), ("uk", "Північна Кароліна"), ("ur", "شمالی کیرولینا"), ("uz", "Shimoliy Karolina"), ("vi", "Bắc Carolina"), ("yo", "Àríwá Carolina"), ("yo_BJ", "Àríwá Carolina"), ("yue", "北卡羅萊納州"), ("yue_Hans", "北卡罗莱纳州"), ("zh", "北卡罗来纳州")]),
                        unofficial_name_list: ["North Carolina"].to_vec(),
                    }
                ),
                (
                    "ND",
                    Subdivision{
                        name: "North Dakota",
                        country_alpha2: Alpha2::US,
                        code: "ND",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.5514926), longitude: Some(-101.0020119), max_latitude: Some(49.000692), min_latitude: Some(45.9350719), max_longitude: Some(-96.554491), min_longitude: Some(-104.05004)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Noord-Dakota"), ("am", "ኖርዝ ዳኮታ"), ("ar", "داكوتا الشمالية"), ("az", "Şimali Dakota"), ("be", "Штат Паўночная Дакота"), ("bg", "Северна Дакота"), ("bn", "নর\u{9cd}থ ড\u{9be}কোট\u{9be}"), ("bs", "Sjeverna Dakota"), ("ca", "Dakota del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄓𑄇\u{1112e}𑄑"), ("ceb", "North Dakota"), ("cs", "Severní Dakota"), ("cy", "Gogledd Dakota"), ("da", "North Dakota"), ("de", "North Dakota"), ("el", "Βόρεια Ντακότα"), ("en", "North Dakota"), ("es", "Dakota del Norte"), ("et", "Põhja-Dakota"), ("eu", "Ipar Dakota"), ("fa", "داکوتای شمالی"), ("fi", "Pohjois-Dakota"), ("fr", "Dakota du Nord"), ("ga", "Dakota Thuaidh"), ("gl", "Dacota do Norte"), ("gu", "નોર\u{acd}થ ડકોટા"), ("ha", "North Dakota"), ("ha_NE", "North Dakota"), ("he", "דקוטה הצפונית"), ("hi", "उत\u{94d}तर ड\u{947}कोटा"), ("hr", "Sjeverna Dakota"), ("hu", "Észak-Dakota"), ("hy", "Հյուսիսային Դակոտա"), ("id", "Dakota Utara"), ("ig", "North Dakota"), ("is", "Norður-Dakóta"), ("it", "Dakota del Nord"), ("ja", "ノースダコタ州"), ("jv", "North Dakota"), ("ka", "ჩრდილოეთი დაკოტა"), ("kk", "Солтүстік Дакота"), ("kn", "ಉತ\u{ccd}ತರ ಡಕೋಟ"), ("ko", "노스다코타 주"), ("lt", "Šiaurės Dakota"), ("lv", "Ziemeļdakota"), ("mk", "Северна Дакота"), ("ml", "വടക\u{d4d}കൻ ഡക\u{d4d}കോട\u{d4d}ട"), ("mn", "Хойд Дакота"), ("mr", "नॉर\u{94d}थ डकोटा"), ("ms", "Dakota Utara"), ("my", "မြောက\u{103a}ဒါက\u{102d}\u{102f}တာပြည\u{103a}နယ\u{103a}"), ("nb", "Nord-Dakota"), ("ne", "नर\u{94d}थ ड\u{947}कोटा"), ("nl", "North Dakota"), ("no", "Nord-Dakota"), ("pa", "ਉ\u{a71}ਤਰੀ ਡਕ\u{a4b}ਟਾ"), ("pl", "Dakota Północna"), ("pt", "Dakota do Norte"), ("ro", "Dakota de Nord"), ("ru", "Северная Дакота"), ("si", "උත\u{dd4}ර\u{dd4} ඩකොට\u{dcf}"), ("sk", "Severná Dakota"), ("sl", "Severna Dakota"), ("sq", "North Dakota"), ("sr", "Северна Дакота"), ("sr_Latn", "Severna Dakota"), ("sv", "North Dakota"), ("sw", "North Dakota"), ("ta", "வடக\u{bcd}கு டகோட\u{bcd}ட\u{bbe}"), ("te", "న\u{c3e}ర\u{c4d}త\u{c4d} డక\u{c4b}ట\u{c3e}"), ("th", "ร\u{e31}ฐนอร\u{e4c}ทดาโคตา"), ("tr", "Kuzey Dakota"), ("uk", "Північна Дакота"), ("ur", "شمالی ڈکوٹا"), ("uz", "Shimoliy Dakota"), ("vi", "Bắc Dakota"), ("yo", "Àríwá Dakota"), ("yo_BJ", "Àríwá Dakota"), ("yue", "北得高打省"), ("yue_Hans", "北得高打省"), ("zh", "北达科他州")]),
                        unofficial_name_list: ["North Dakota"].to_vec(),
                    }
                ),
                (
                    "NE",
                    Subdivision{
                        name: "Nebraska",
                        country_alpha2: Alpha2::US,
                        code: "NE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.4925374), longitude: Some(-99.9018131), max_latitude: Some(43.00170689999999), min_latitude: Some(39.999932), max_longitude: Some(-95.30829000000001), min_longitude: Some(-104.053514)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nebraska"), ("am", "ነብራስካ"), ("ar", "نبراسكا"), ("az", "Nebraska"), ("be", "Штат Небраска"), ("bg", "Небраска"), ("bn", "নেব\u{9cd}র\u{9be}স\u{9cd}ক\u{9be}"), ("bs", "Nebraska"), ("ca", "Nebraska"), ("ccp", "𑄚𑄬𑄝\u{11133}𑄢𑄌\u{11134}𑄇"), ("ceb", "Nebraska"), ("cs", "Nebraska"), ("cy", "Nebraska"), ("da", "Nebraska"), ("de", "Nebraska"), ("el", "Νεμπράσκα"), ("en", "Nebraska"), ("es", "Nebraska"), ("et", "Nebraska"), ("eu", "Nebraska"), ("fa", "نبراسکا"), ("fi", "Nebraska"), ("fr", "Nebraska"), ("ga", "Nebraska"), ("gl", "Nebrasca"), ("gu", "ન\u{ac7}બ\u{acd}રાસ\u{acd}કા"), ("ha", "Nebraska"), ("ha_NE", "Nebraska"), ("he", "נברסקה"), ("hi", "न\u{947}ब\u{94d}रास\u{94d}का"), ("hr", "Nebraska"), ("hu", "Nebraska"), ("hy", "Նեբրասկա"), ("id", "Nebraska"), ("ig", "Nebraska"), ("is", "Nebraska"), ("it", "Nebraska"), ("ja", "ネブラスカ州"), ("jv", "Nebraska"), ("ka", "ნებრასკა"), ("kk", "Небраска"), ("kn", "ನ\u{cc6}ಬ\u{ccd}ರಸ\u{ccd}ಕಾ"), ("ko", "네브래스카 주"), ("lt", "Nebraska"), ("lv", "Nebraska"), ("mk", "Небраска"), ("ml", "നെബ\u{d4d}ര\u{d3e}സ\u{d4d}ക"), ("mn", "Небраска"), ("mr", "न\u{947}ब\u{94d}रास\u{94d}का"), ("ms", "Nebraska"), ("my", "န\u{102e}ဘရားစကားပြည\u{103a}နယ\u{103a}"), ("nb", "Nebraska"), ("ne", "न\u{947}ब\u{94d}रास\u{94d}का"), ("nl", "Nebraska"), ("no", "Nebraska"), ("pa", "ਨਬਰਾਸਕਾ"), ("pl", "Nebraska"), ("pt", "Nebraska"), ("ro", "Nebraska"), ("ru", "Небраска"), ("si", "නෙබ\u{dca}\u{200d}රස\u{dca}ක\u{dcf}"), ("sk", "Nebraska"), ("sl", "Nebraska"), ("sq", "Nebraska"), ("sr", "Небраска"), ("sr_Latn", "Nebraska"), ("sv", "Nebraska"), ("sw", "Nebraska"), ("ta", "நெப\u{bcd}ர\u{bbe}ஸ\u{bcd}க\u{bbe}"), ("te", "న\u{c46}బ\u{c4d}ర\u{c3e}స\u{c4d}క\u{c3e}"), ("th", "ร\u{e31}ฐเนแบรสกา"), ("tr", "Nebraska"), ("uk", "Небраска"), ("ur", "نیبراسکا"), ("uz", "Nebraska"), ("vi", "Nebraska"), ("yo", "Nebraska"), ("yo_BJ", "Nebraska"), ("yue", "內布拉斯加州"), ("yue_Hans", "内布拉斯加州"), ("zh", "內布拉斯加州")]),
                        unofficial_name_list: ["Nebraska"].to_vec(),
                    }
                ),
                (
                    "NH",
                    Subdivision{
                        name: "New Hampshire",
                        country_alpha2: Alpha2::US,
                        code: "NH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1938516), longitude: Some(-71.5723953), max_latitude: Some(45.3054761), min_latitude: Some(42.696985), max_longitude: Some(-70.602656), min_longitude: Some(-72.55718499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "New Hampshire"), ("am", "ኒው ሃምፕሸር"), ("ar", "نيوهامشير"), ("az", "Nyu Hempşir"), ("be", "Штат Нью-Гэмпшыр"), ("bg", "Ню Хампшър"), ("bn", "নিউ হ\u{9cd}য\u{9be}ম\u{9cd}প\u{9cd}\u{200c}শ\u{9be}য\u{9bc}\u{9be}র"), ("bs", "New Hampshire"), ("ca", "Nou Hampshire"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄦𑄟\u{11134}𑄥𑄠𑄢\u{11134}"), ("ceb", "New Hampshire"), ("cs", "New Hampshire"), ("cy", "New Hampshire"), ("da", "New Hampshire"), ("de", "New Hampshire"), ("el", "Νιου Χάμσαϊρ"), ("en", "New Hampshire"), ("es", "Nuevo Hampshire"), ("et", "New Hampshire"), ("eu", "New Hampshire"), ("fa", "نیوهمپشایر"), ("fi", "New Hampshire"), ("fr", "New Hampshire"), ("ga", "New Hampshire"), ("gl", "Nova Hampshire - New Hampshire"), ("gu", "ન\u{acd}ય\u{ac2} હ\u{ac7}મ\u{acd}પશાયર"), ("he", "ניו המפשייר"), ("hi", "नया ह\u{947}म\u{94d}पशायर"), ("hr", "New Hampshire"), ("hu", "New Hampshire"), ("hy", "Նյու Հեմպշիր"), ("id", "New Hampshire"), ("ig", "Hampshire Ohúrú"), ("is", "New Hampshire"), ("it", "New Hampshire"), ("ja", "ニューハンプシャー州"), ("jv", "New Hampshire"), ("ka", "ნიუ-ჰემფშირი"), ("kk", "Нью-Гэмпшир"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಹ\u{ccd}ಯಾಂಪ\u{ccd}ಶೈರ\u{ccd}"), ("ko", "뉴햄프셔 주"), ("ky", "Нью-Хемпшир"), ("lt", "Naujasis Hampšyras"), ("lv", "Ņūhempšīra"), ("mk", "Њу Хемпшир"), ("ml", "ന\u{d4d}യ\u{d42} ഹ\u{d3e}ംഷെയർ"), ("mn", "Нью-Хэмпшир"), ("mr", "न\u{94d}य\u{942} ह\u{945}म\u{94d}पशायर"), ("ms", "New Hampshire"), ("my", "နယ\u{1030}းဟမ\u{103a}ရ\u{103e}\u{102d}\u{102f}င\u{103a}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "New Hampshire"), ("ne", "न\u{94d}य\u{941} ह\u{94d}याम\u{94d}पसायर"), ("nl", "New Hampshire"), ("no", "New Hampshire"), ("pa", "ਨਿਊ ਹ\u{a48}\u{a02}ਪਸ\u{a3c}ਰ"), ("pl", "New Hampshire"), ("pt", "Nova Hampshire"), ("ro", "New Hampshire"), ("ru", "Нью-Гэмпшир"), ("si", "න\u{dd2}ව\u{dca} හැම\u{dca}ප\u{dca}ෂයර\u{dca}"), ("sk", "New Hampshire"), ("sl", "New Hampshire"), ("sq", "New Hampshire"), ("sr", "Њу Хемпшир"), ("sr_Latn", "Nju Hempšir"), ("sv", "New Hampshire"), ("sw", "New Hampshire"), ("ta", "நியூ ஹ\u{bbe}ம\u{bcd}சயர\u{bcd}"), ("te", "న\u{c4d}యూహ\u{c3e}ంప\u{c4d}\u{200c}ష\u{c48}ర\u{c4d}"), ("th", "ร\u{e31}ฐน\u{e34}วแฮมป\u{e4c}เช\u{e35}ยร\u{e4c}"), ("tr", "New Hampshire"), ("uk", "Нью-Гемпшир"), ("ur", "نیو ہیمپشائر"), ("uz", "Nyu-Xempshir"), ("vi", "New Hampshire"), ("yo", "New Hampshire"), ("yo_BJ", "New Hampshire"), ("yue", "紐咸西州"), ("yue_Hans", "纽咸西州"), ("zh", "新罕布什尔州")]),
                        unofficial_name_list: ["New Hampshire"].to_vec(),
                    }
                ),
                (
                    "NJ",
                    Subdivision{
                        name: "New Jersey",
                        country_alpha2: Alpha2::US,
                        code: "NJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.0583238), longitude: Some(-74.4056612), max_latitude: Some(41.357423), min_latitude: Some(38.9285589), max_longitude: Some(-73.9024391), min_longitude: Some(-75.5597921)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "New Jersey"), ("am", "ኒው ጄርዚ"), ("ar", "نيوجيرسي"), ("az", "Nyu Cersi"), ("be", "Штат Нью-Джэрсі"), ("bg", "Ню Джърси"), ("bn", "নিউ জ\u{9be}র\u{9cd}সি"), ("bs", "New Jersey"), ("ca", "Nova Jersey"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄎𑄢\u{11134}𑄥\u{11128}"), ("ceb", "New Jersey"), ("cs", "New Jersey"), ("cy", "New Jersey"), ("da", "New Jersey"), ("de", "New Jersey"), ("el", "Νιου Τζέρσεϊ"), ("en", "New Jersey"), ("es", "Nueva Jersey"), ("et", "New Jersey osariik"), ("eu", "New Jersey"), ("fa", "نیوجرسی"), ("fi", "New Jersey"), ("fr", "New Jersey"), ("ga", "New Jersey"), ("gl", "Nova Jersey"), ("gu", "ન\u{acd}ય\u{ac2} જર\u{acd}સી"), ("he", "ניו ג׳רזי"), ("hi", "न\u{94d}य\u{942} जर\u{94d}सी"), ("hr", "New Jersey"), ("hu", "New Jersey"), ("hy", "Նյու Ջերսի"), ("id", "New Jersey"), ("ig", "Nú Jezị"), ("is", "New Jersey"), ("it", "New Jersey"), ("ja", "ニュージャージー州"), ("jv", "New Jersey"), ("ka", "ნიუ-ჯერსი"), ("kk", "Нью-Джерси"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಜ\u{cc6}ರ\u{ccd}ಸ\u{cbf}"), ("ko", "뉴저지 주"), ("ky", "Нью-Джерси"), ("lt", "Naujasis Džersis"), ("lv", "Ņūdžersija"), ("mk", "Њу Џерси"), ("ml", "ന\u{d4d}യ\u{d42} ജെഴ\u{d4d}സി"), ("mr", "न\u{94d}य\u{942} जर\u{94d}सी"), ("ms", "New Jersey"), ("my", "နယ\u{1030}းဂျာစ\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "New Jersey"), ("ne", "न\u{94d}य\u{942} जर\u{94d}सी"), ("nl", "New Jersey"), ("no", "New Jersey"), ("pa", "ਨਿਊ ਜਰਸੀ"), ("pl", "New Jersey"), ("pt", "Nova Jérsia"), ("ro", "New Jersey"), ("ru", "Нью-Джерси"), ("si", "න\u{dd2}ව\u{dca} ජර\u{dca}ස\u{dd2}"), ("sk", "New Jersey"), ("sl", "New Jersey"), ("sq", "New Jersey"), ("sr", "Њу Џерзи"), ("sr_Latn", "Nju Džerzi"), ("sv", "New Jersey"), ("sw", "New Jersey"), ("ta", "நியூ செர\u{bcd}சி"), ("te", "న\u{c4d}యూజ\u{c46}ర\u{c4d}స\u{c40}"), ("th", "ร\u{e31}ฐน\u{e34}วเจอร\u{e4c}ซ\u{e35}ย\u{e4c}"), ("tr", "New Jersey"), ("uk", "Нью-Джерсі"), ("ur", "نیو جرسی"), ("uz", "Nʼyu-Jersi"), ("vi", "New Jersey"), ("yo", "New Jersey"), ("yo_BJ", "New Jersey"), ("yue", "紐澤西州"), ("yue_Hans", "纽泽西州"), ("zh", "新泽西州"), ("zu", "New Jersey")]),
                        unofficial_name_list: ["New Jersey"].to_vec(),
                    }
                ),
                (
                    "NM",
                    Subdivision{
                        name: "New Mexico",
                        country_alpha2: Alpha2::US,
                        code: "NM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(34.5199402), longitude: Some(-105.8700901), max_latitude: Some(37.0002931), min_latitude: Some(31.332172), max_longitude: Some(-103.001964), min_longitude: Some(-109.050173)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nieu-Meksiko"), ("am", "ኒው ሜክሲኮ"), ("ar", "نيومكسيكو"), ("az", "Nyu Meksiko"), ("be", "Штат Нью-Мексіка"), ("bg", "Ню Мексико"), ("bn", "নিউ মেক\u{9cd}সিকো"), ("bs", "New Mexico"), ("ca", "Nou Mèxic"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄟𑄬𑄇\u{11134}𑄥\u{11128}𑄇\u{1112e}"), ("ceb", "New Mexico"), ("cs", "Nové Mexiko"), ("cy", "New Mexico"), ("da", "New Mexico"), ("de", "New Mexico"), ("el", "Νέο Μεξικό"), ("en", "New Mexico"), ("es", "Nuevo México"), ("et", "New Mexico"), ("eu", "Mexiko Berria"), ("fa", "نیومکزیکو"), ("fi", "New Mexico"), ("fr", "Nouveau-Mexique"), ("ga", "Nua-Mheicsiceo"), ("gl", "Novo México"), ("gu", "ન\u{acd}ય\u{ac2} મ\u{ac7}ક\u{acd}સિકો"), ("ha", "New Mexico"), ("ha_NE", "New Mexico"), ("he", "ניו מקסיקו"), ("hi", "नया म\u{947}क\u{94d}सिको"), ("hr", "Novi Meksiko"), ("hu", "Új-Mexikó"), ("hy", "Նյու Մեքսիկո"), ("id", "New Mexico"), ("ig", "Nú Mézíkọ"), ("is", "New Mexico"), ("it", "Nuovo Messico"), ("ja", "ニューメキシコ州"), ("jv", "New Mexico"), ("ka", "ნიუ-მექსიკო"), ("kk", "Нью-Мексико"), ("kn", "ಹೊಸ ಮ\u{cc6}ಕ\u{ccd}ಸ\u{cbf}ಕೋ"), ("ko", "뉴멕시코 주"), ("ky", "Нью-Мексико"), ("lt", "Naujoji Meksika"), ("lv", "Ņūmeksika"), ("mk", "Ново Мексико"), ("ml", "ന\u{d4d}യ\u{d42} മെക\u{d4d}സിക\u{d4d}കോ"), ("mn", "Нью-Мексико"), ("mr", "न\u{94d}य\u{942} म\u{947}क\u{94d}सिको"), ("ms", "New Mexico"), ("my", "နယ\u{1030}းမက\u{1039}ကဆ\u{102e}က\u{102d}\u{102f}ပြည\u{103a}နယ\u{103a}"), ("nb", "New Mexico"), ("ne", "न\u{94d}य\u{942} म\u{947}क\u{94d}सिको"), ("nl", "New Mexico"), ("no", "New Mexico"), ("pa", "ਨਿਊ ਮ\u{a48}ਕਸੀਕ\u{a4b}"), ("pl", "Nowy Meksyk"), ("pt", "Novo México"), ("ro", "New Mexico"), ("ru", "Нью-Мексико"), ("si", "න\u{dd2}ව\u{dca} මෙක\u{dca}ස\u{dd2}කෝ"), ("sk", "Nové Mexiko"), ("sl", "Nova Mehika"), ("sq", "New Mexico"), ("sr", "Нови Мексико"), ("sr_Latn", "Novi Meksiko"), ("sv", "New Mexico"), ("sw", "New Mexico"), ("ta", "நியூ மெக\u{bcd}சிகோ"), ("te", "న\u{c4d}యూ మ\u{c46}క\u{c4d}స\u{c3f}క\u{c4b}"), ("th", "ร\u{e31}ฐน\u{e34}วเม\u{e47}กซ\u{e34}โก"), ("tr", "New Mexico"), ("uk", "Нью-Мексико"), ("ur", "نیو میکسیکو"), ("uz", "Nʼyu-Meksiko"), ("vi", "New Mexico"), ("yo", "Ìpínlẹ\u{300} Titun Mẹ\u{301}ksíkò"), ("yo_BJ", "Ìpínlɛ\u{300} Titun Mɛ\u{301}ksíkò"), ("yue", "新墨西哥州"), ("yue_Hans", "新墨西哥州"), ("zh", "新墨西哥州")]),
                        unofficial_name_list: ["New Mexico"].to_vec(),
                    }
                ),
                (
                    "NV",
                    Subdivision{
                        name: "Nevada",
                        country_alpha2: Alpha2::US,
                        code: "NV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.8026097), longitude: Some(-116.419389), max_latitude: Some(42.002207), min_latitude: Some(35.001857), max_longitude: Some(-114.0396479), min_longitude: Some(-120.0064729)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nevada"), ("am", "ነቫዳ"), ("ar", "نيفادا"), ("az", "Nevada"), ("be", "Штат Невада"), ("bg", "Невада"), ("bn", "নেভ\u{9be}ড\u{9be}"), ("bs", "Nevada"), ("ca", "Nevada"), ("ccp", "𑄚𑄬𑄞𑄓"), ("ceb", "Nevada"), ("cs", "Nevada"), ("cy", "Nevada"), ("da", "Nevada"), ("de", "Nevada"), ("el", "Νεβάδα"), ("en", "Nevada"), ("es", "Nevada"), ("et", "Nevada"), ("eu", "Nevada"), ("fa", "نوادا"), ("fi", "Nevada"), ("fr", "Nevada"), ("ga", "Nevada"), ("gl", "Nevada"), ("gu", "ન\u{ac7}વાડા"), ("ha", "Nevada"), ("ha_NE", "Nevada"), ("he", "נבדה"), ("hi", "न\u{947}वाडा"), ("hr", "Nevada"), ("hu", "Nevada"), ("hy", "Նևադա"), ("id", "Nevada"), ("ig", "Náevadạ"), ("is", "Nevada"), ("it", "Nevada"), ("ja", "ネバダ州"), ("jv", "Nevada"), ("ka", "ნევადა"), ("kk", "Невада"), ("kn", "ನ\u{cc6}ವಾಡಾ"), ("ko", "네바다 주"), ("lt", "Nevada"), ("lv", "Nevada"), ("mk", "Невада"), ("ml", "നെവ\u{d3e}ഡ"), ("mn", "Невада"), ("mr", "न\u{947}व\u{94d}हाडा"), ("ms", "Nevada"), ("my", "န\u{102e}ဗားဒါးပြည\u{103a}နယ\u{103a}"), ("nb", "Nevada"), ("ne", "न\u{947}वाडा"), ("nl", "Nevada"), ("no", "Nevada"), ("pa", "ਨਵਾਡਾ"), ("pl", "Nevada"), ("pt", "Nevada"), ("ro", "Nevada"), ("ru", "Невада"), ("si", "නෙව\u{dcf}ඩ\u{dcf}"), ("sk", "Nevada"), ("sl", "Nevada"), ("sq", "Nevada"), ("sr", "Невада"), ("sr_Latn", "Nevada"), ("sv", "Nevada"), ("sw", "Nevada"), ("ta", "நெவ\u{bbe}ட\u{bbe}"), ("te", "న\u{c46}వ\u{c3e}డ\u{c3e}"), ("th", "ร\u{e31}ฐเนวาดา"), ("tr", "Nevada"), ("uk", "Невада"), ("ur", "نیواڈا"), ("uz", "Nevada"), ("vi", "Nevada"), ("yo", "Nevada"), ("yo_BJ", "Nevada"), ("yue", "內華達州"), ("yue_Hans", "内华达州"), ("zh", "内华达州"), ("zu", "Nevada")]),
                        unofficial_name_list: ["Nevada"].to_vec(),
                    }
                ),
                (
                    "NY",
                    Subdivision{
                        name: "New York",
                        country_alpha2: Alpha2::US,
                        code: "NY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2994285), longitude: Some(-74.21793260000001), max_latitude: Some(45.015865), min_latitude: Some(40.4913686), max_longitude: Some(-71.8562755), min_longitude: Some(-79.76214379999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "New York"), ("am", "ኒው ዮርክ"), ("ar", "نيويورك"), ("az", "Nyu York"), ("be", "Штат Нью-Ёрк"), ("bg", "Ню Йорк"), ("bn", "নিউ ইয\u{9bc}র\u{9cd}ক"), ("bs", "New York"), ("ca", "Nova York"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄃\u{11128}𑄠\u{11127}𑄢\u{11134}𑄇\u{11134}"), ("ceb", "New York"), ("cs", "New York"), ("cy", "Efrog Newydd"), ("da", "New York"), ("de", "New York"), ("el", "Νέα Υόρκη"), ("en", "New York"), ("es", "Nueva York"), ("et", "New Yorgi osariik"), ("eu", "New York"), ("fa", "نیویورک"), ("fi", "New York"), ("fr", "New York"), ("ga", "Nua-Eabhrac"), ("gl", "Estado de Nova York"), ("gu", "ન\u{acd}ય\u{ac1} યોર\u{acd}ક"), ("ha", "New York (jiha)"), ("ha_NE", "New York (jiha)"), ("he", "ניו יורק"), ("hi", "न\u{94d}य\u{942}यॉर\u{94d}क"), ("hr", "New York"), ("hu", "New York"), ("hy", "Նյու Յորք"), ("id", "New York"), ("ig", "Nú Yọk"), ("is", "New York-fylki"), ("it", "New York"), ("ja", "ニューヨーク州"), ("jv", "New York"), ("ka", "ნიუ-იორკი"), ("kk", "Нью-Йорк"), ("km", "ញ\u{17bc}វយ\u{17c9}ក"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಯಾರ\u{ccd}ಕ\u{ccd}"), ("ko", "뉴욕 주"), ("lo", "ລ\u{eb1}ດນ\u{eb5}ວຢອກ"), ("lt", "Niujorko valstija"), ("lv", "Ņujorka"), ("mk", "Њујорк"), ("ml", "ന\u{d4d}യ\u{d42}യോർക\u{d4d}ക\u{d4d}"), ("mn", "Нью-Йорк Муж Улс"), ("mr", "न\u{94d}य\u{942} यॉर\u{94d}क"), ("ms", "New York"), ("my", "နယ\u{1030}းယောက\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "New York"), ("ne", "न\u{94d}य\u{942} योर\u{94d}क"), ("nl", "New York"), ("no", "New York"), ("or", "ନ\u{b3f}ଉ ୟର\u{b4d}କ"), ("pa", "ਨਿਊਯਾਰਕ"), ("pl", "Nowy Jork"), ("ps", "نيويارک"), ("pt", "Nova Iorque"), ("ro", "New York"), ("ru", "Нью-Йорк"), ("sd", "نيو يارڪ"), ("si", "න\u{dd2}ව\u{dca}යෝර\u{dca}ක\u{dca}"), ("sk", "New York"), ("sl", "New York"), ("so", "New York"), ("sq", "New York"), ("sr", "Њујорк"), ("sr_Latn", "Njujork"), ("sv", "New York"), ("sw", "New York"), ("ta", "நியூ யோர\u{bcd}க\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("te", "న\u{c4d}యూయ\u{c3e}ర\u{c4d}క\u{c4d} ర\u{c3e}ష\u{c4d}ట\u{c4d}రం"), ("th", "ร\u{e31}ฐน\u{e34}วยอร\u{e4c}ก"), ("tk", "Nýu-Ýork ştaty"), ("tr", "New York"), ("uk", "Нью-Йорк"), ("ur", "نیویارک"), ("uz", "Nyu York"), ("vi", "Tiểu bang New York"), ("yo", "Ìpínlẹ\u{300} New York"), ("yo_BJ", "Ìpínlɛ\u{300} New York"), ("yue", "紐約州"), ("yue_Hans", "纽约州"), ("zh", "纽约州"), ("zu", "New York Isifunda")]),
                        unofficial_name_list: ["New York"].to_vec(),
                    }
                ),
                (
                    "OH",
                    Subdivision{
                        name: "Ohio",
                        country_alpha2: Alpha2::US,
                        code: "OH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.4172871), longitude: Some(-82.90712300000001), max_latitude: Some(41.9773019), min_latitude: Some(38.4034229), max_longitude: Some(-80.5182), min_longitude: Some(-84.8203049)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ohio"), ("am", "ኦሃዮ"), ("ar", "أوهايو"), ("az", "Ohayo"), ("be", "Штат Агая"), ("bg", "Охайо"), ("bn", "ওহ\u{9be}ইও"), ("bs", "Ohio"), ("ca", "Ohio"), ("ccp", "𑄃\u{1112e}𑄦\u{11128}𑄃\u{1112e}"), ("ceb", "Ohio"), ("cs", "Ohio"), ("cy", "Ohio"), ("da", "Ohio"), ("de", "Ohio"), ("el", "Οχάιο"), ("en", "Ohio"), ("es", "Ohio"), ("et", "Ohio"), ("eu", "Ohio"), ("fa", "اوهایو"), ("fi", "Ohio"), ("fr", "Ohio"), ("ga", "Ohio"), ("gl", "Ohio"), ("gu", "ઓહિયો"), ("ha", "Ohio"), ("ha_NE", "Ohio"), ("he", "אוהיו"), ("hi", "ओहायो"), ("hr", "Ohio"), ("hu", "Ohio"), ("hy", "Օհայո"), ("id", "Ohio"), ("ig", "Ohaïyo"), ("is", "Ohio"), ("it", "Ohio"), ("ja", "オハイオ州"), ("jv", "Ohio"), ("ka", "ოჰაიოს შტატი"), ("kk", "Огайо"), ("kn", "ಒಹಾಯೊ"), ("ko", "오하이오 주"), ("lt", "Ohajas"), ("lv", "Ohaio"), ("mk", "Охајо"), ("ml", "ഒഹ\u{d3e}യോ"), ("mn", "Охайо"), ("mr", "ओहायो"), ("ms", "Ohio"), ("my", "အ\u{102d}\u{102f}ဟ\u{102d}\u{102f}င\u{103a}းယ\u{102d}\u{102f}းပြည\u{103a}နယ\u{103a}"), ("nb", "Ohio"), ("ne", "ओहायो"), ("nl", "Ohio"), ("no", "Ohio"), ("pa", "ਓਹਾਇਓ"), ("pl", "Ohio"), ("pt", "Ohio"), ("ro", "Ohio"), ("ru", "Огайо"), ("si", "ඔහ\u{dcf}යෝ"), ("sk", "Ohio"), ("sl", "Ohio"), ("sq", "Ohio"), ("sr", "Охајо"), ("sr_Latn", "Ohajo"), ("sv", "Ohio"), ("sw", "Ohio"), ("ta", "ஒகையோ"), ("te", "ఒహ\u{c3e}య\u{c4b}"), ("th", "ร\u{e31}ฐโอไฮโอ"), ("tr", "Ohio"), ("uk", "Огайо"), ("ur", "اوہائیو"), ("uz", "Ogayo"), ("vi", "Ohio"), ("yo", "Ohio"), ("yo_BJ", "Ohio"), ("yue", "俄亥俄州"), ("yue_Hans", "俄亥俄州"), ("zh", "俄亥俄州")]),
                        unofficial_name_list: ["Ohio"].to_vec(),
                    }
                ),
                (
                    "OK",
                    Subdivision{
                        name: "Oklahoma",
                        country_alpha2: Alpha2::US,
                        code: "OK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.0077519), longitude: Some(-97.092877), max_latitude: Some(37.0023119), min_latitude: Some(33.615787), max_longitude: Some(-94.430662), min_longitude: Some(-103.002455)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oklahoma"), ("am", "ኦክላሆማ"), ("ar", "أوكلاهوما"), ("az", "Oklahoma"), ("be", "Штат Аклахома"), ("bg", "Оклахома"), ("bn", "ওক\u{200c}ল\u{9be}হোম\u{9be}"), ("bs", "Oklahoma"), ("ca", "Oklahoma"), ("ccp", "𑄃𑄇\u{11134}𑄣𑄦\u{1112e}𑄟"), ("ceb", "Oklahoma"), ("cs", "Oklahoma"), ("cy", "Oclahoma"), ("da", "Oklahoma"), ("de", "Oklahoma"), ("el", "Οκλαχόμα"), ("en", "Oklahoma"), ("es", "Oklahoma"), ("et", "Oklahoma"), ("eu", "Oklahoma"), ("fa", "اکلاهما"), ("fi", "Oklahoma"), ("fr", "Oklahoma"), ("ga", "Oklahoma"), ("gl", "Oklahoma"), ("gu", "ઓક\u{acd}લાહોમા"), ("ha", "Oklahoma"), ("ha_NE", "Oklahoma"), ("he", "אוקלהומה"), ("hi", "ओक\u{94d}लाहोमा"), ("hr", "Oklahoma"), ("hu", "Oklahoma"), ("hy", "Օկլահոմա"), ("id", "Oklahoma"), ("ig", "Oklahoma"), ("is", "Oklahoma"), ("it", "Oklahoma"), ("ja", "オクラホマ州"), ("jv", "Oklahoma"), ("ka", "ოკლაჰომა"), ("kk", "Оклахома"), ("kn", "ಒಕ\u{ccd}ಲಹೋಮ"), ("ko", "오클라호마 주"), ("ky", "Оклахома"), ("lt", "Oklahoma"), ("lv", "Oklahoma"), ("mk", "Оклахома"), ("ml", "ഒക\u{d4d}\u{200c}ലഹോമ"), ("mn", "Оклахома"), ("mr", "ओक\u{94d}लाहोमा"), ("ms", "Oklahoma"), ("my", "ဥက\u{1039}ကလာဟ\u{102d}\u{102f}းမားပြည\u{103a}နယ\u{103a}"), ("nb", "Oklahoma"), ("ne", "ओक\u{94d}लाहोमा"), ("nl", "Oklahoma"), ("no", "Oklahoma"), ("pa", "ਓਕਲਾਹ\u{a4b}ਮਾ"), ("pl", "Oklahoma"), ("pt", "Oklahoma"), ("ro", "Oklahoma"), ("ru", "Оклахома"), ("si", "ඔක\u{dca}ලොහොම\u{dcf}"), ("sk", "Oklahoma"), ("sl", "Oklahoma"), ("sq", "Oklahoma"), ("sr", "Оклахома"), ("sr_Latn", "Oklahoma"), ("sv", "Oklahoma"), ("sw", "Oklahoma"), ("ta", "ஓக\u{bcd}லகோம\u{bbe}"), ("te", "ఓక\u{c4d}లహ\u{c4b}మ\u{c3e}"), ("th", "ร\u{e31}ฐโอคลาโฮมา"), ("tr", "Oklahoma"), ("uk", "Оклахома"), ("ur", "اوکلاہوما"), ("uz", "Oklaxoma"), ("vi", "Oklahoma"), ("yo", "Oklahoma"), ("yo_BJ", "Oklahoma"), ("yue", "奧克拉何馬州"), ("yue_Hans", "奥克拉何马州"), ("zh", "奧克拉荷馬州")]),
                        unofficial_name_list: ["Oklahoma"].to_vec(),
                    }
                ),
                (
                    "OR",
                    Subdivision{
                        name: "Oregon",
                        country_alpha2: Alpha2::US,
                        code: "OR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8041334), longitude: Some(-120.5542012), max_latitude: Some(46.2920157), min_latitude: Some(41.9917941), max_longitude: Some(-116.463262), min_longitude: Some(-124.6129365)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Oregon"), ("am", "ኦረጎን"), ("ar", "أوريغون"), ("az", "Oreqon"), ("be", "Штат Арэгон"), ("bg", "Орегон"), ("bn", "অরেগন"), ("bs", "Oregon"), ("ca", "Oregon"), ("ccp", "𑄃𑄢\u{11128}𑄉\u{11127}𑄚\u{11134}"), ("ceb", "Oregon"), ("cs", "Oregon"), ("cy", "Oregon"), ("da", "Oregon"), ("de", "Oregon"), ("el", "Όρεγκον"), ("en", "Oregon"), ("es", "Oregón"), ("et", "Oregon"), ("eu", "Oregon"), ("fa", "اورگن"), ("fi", "Oregon"), ("fr", "Oregon"), ("ga", "Oregon"), ("gl", "Oregón - Oregon"), ("gu", "ઑર\u{ac7}ગોન"), ("ha", "Oregon"), ("ha_NE", "Oregon"), ("he", "אורגון"), ("hi", "औरिगन"), ("hr", "Oregon"), ("hu", "Oregon"), ("hy", "Օրեգոն"), ("id", "Oregon"), ("ig", "Oregon"), ("is", "Oregon"), ("it", "Oregon"), ("ja", "オレゴン州"), ("jv", "Oregon"), ("ka", "ორეგონი"), ("kk", "Орегон"), ("kn", "ಆರ\u{cc6}ಗನ\u{ccd}"), ("ko", "오리건 주"), ("lt", "Oregonas"), ("lv", "Oregona"), ("mk", "Орегон"), ("ml", "ഒറിഗൺ"), ("mn", "Орегон"), ("mr", "ओर\u{947}गन"), ("ms", "Oregon"), ("my", "အ\u{102d}\u{102f}ရ\u{102e}ဂ\u{103d}န\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Oregon"), ("ne", "ओर\u{947}गन"), ("nl", "Oregon"), ("no", "Oregon"), ("pa", "ਔਰ\u{a47}ਗਨ"), ("pl", "Oregon"), ("pt", "Oregon"), ("ro", "Oregon"), ("ru", "Орегон"), ("si", "ඔරෙගොන\u{dca}"), ("sk", "Oregon"), ("sl", "Oregon"), ("sq", "Oregon"), ("sr", "Орегон"), ("sr_Latn", "Oregon"), ("sv", "Oregon"), ("sw", "Oregon"), ("ta", "ஓரிகன\u{bcd}"), ("te", "ఓర\u{c46}గ\u{c3e}న\u{c4d}"), ("th", "ร\u{e31}ฐออร\u{e34}กอน"), ("tr", "Oregon"), ("uk", "Орегон"), ("ur", "اوریگون"), ("uz", "Oregon"), ("vi", "Oregon"), ("yo", "Oregon"), ("yo_BJ", "Oregon"), ("yue", "柯利近省"), ("yue_Hans", "柯利近省"), ("zh", "俄勒冈州")]),
                        unofficial_name_list: ["Oregon"].to_vec(),
                    }
                ),
                (
                    "PA",
                    Subdivision{
                        name: "Pennsylvania",
                        country_alpha2: Alpha2::US,
                        code: "PA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.2033216), longitude: Some(-77.1945247), max_latitude: Some(42.2693866), min_latitude: Some(39.7197989), max_longitude: Some(-74.6895018), min_longitude: Some(-80.51989499999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pennsilvanië"), ("am", "ፔንስልቬኒያ"), ("ar", "بنسيلفانيا"), ("az", "Pensilvaniya"), ("be", "Штат Пенсільванія"), ("bg", "Пенсилвания"), ("bn", "পেন\u{9cd}সিল\u{9cd}\u{200c}ভেনিয\u{9bc}\u{9be}"), ("bs", "Pennsylvania"), ("ca", "Pennsilvània"), ("ccp", "𑄛𑄬𑄚\u{11128}𑄚\u{11134}𑄥\u{11128}𑄣\u{11134}𑄞𑄚\u{11128}𑄠"), ("ceb", "Pennsylvania"), ("cs", "Pensylvánie"), ("cy", "Pennsylvania"), ("da", "Pennsylvania"), ("de", "Pennsylvania"), ("el", "Πενσυλβάνια"), ("en", "Pennsylvania"), ("es", "Pensilvania"), ("et", "Pennsylvania"), ("eu", "Pennsylvania"), ("fa", "پنسیلوانیا"), ("fi", "Pennsylvania"), ("fr", "Pennsylvanie"), ("ga", "Pennsylvania"), ("gl", "Pensilvania - Pennsylvania"), ("gu", "પ\u{ac7}ન\u{acd}સિલ\u{acd}વ\u{ac7}નિયા"), ("he", "פנסילבניה"), ("hi", "प\u{947}न\u{94d}सिलव\u{947}निया"), ("hr", "Pennsylvania"), ("hu", "Pennsylvania"), ("hy", "Փենսիլվանիա"), ("id", "Pennsylvania"), ("ig", "Pensílvéníyạ"), ("is", "Pennsylvanía"), ("it", "Pennsylvania"), ("ja", "ペンシルベニア州"), ("jv", "Pennsylvania"), ("ka", "პენსილვანია"), ("kk", "Пенсильвания"), ("kn", "ಪ\u{cc6}ನ\u{ccd}ಸ\u{cbf}ಲ\u{ccd}ವೇನ\u{cbf}ಯಾ"), ("ko", "펜실베이니아 주"), ("ky", "Пенсильвания"), ("lt", "Pensilvanija"), ("lv", "Pensilvānija"), ("mk", "Пенсилванија"), ("ml", "പെൻ\u{200c}സിൽ\u{200c}വ\u{d3e}നിയ"), ("mn", "Пеннсилвани"), ("mr", "प\u{947}नसिल\u{94d}व\u{94d}ह\u{947}निया"), ("ms", "Pennsylvania"), ("my", "ပင\u{103a}ဆယ\u{103a}ဗေးန\u{102e}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "Pennsylvania"), ("ne", "प\u{947}न\u{94d}सिलभ\u{947}निया"), ("nl", "Pennsylvania"), ("no", "Pennsylvania"), ("pa", "ਪ\u{a48}\u{a71}ਨਸਿਲਵ\u{a47}ਨੀਆ"), ("pl", "Pensylwania"), ("pt", "Pensilvânia"), ("ro", "Pennsylvania"), ("ru", "Пенсильвания"), ("si", "පෙන\u{dca}සේල\u{dca}වේන\u{dd2}ය\u{dcf}"), ("sk", "Pensylvánia"), ("sl", "Pensilvanija"), ("sq", "Pennsylvania"), ("sr", "Пенсилванија"), ("sr_Latn", "Pensilvanija"), ("sv", "Pennsylvania"), ("sw", "Pennsylvania"), ("ta", "பென\u{bcd}சில\u{bcd}வேனிய\u{bbe}"), ("te", "ప\u{c46}న\u{c4d}స\u{c3f}ల\u{c4d}వ\u{c47}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐเพนซ\u{e34}ลเวเน\u{e35}ย"), ("tr", "Pensilvanya"), ("uk", "Пенсильванія"), ("ur", "پنسلوانیا"), ("uz", "Pennsilvaniya"), ("vi", "Pennsylvania"), ("yo", "Pennsylvania"), ("yo_BJ", "Pennsylvania"), ("yue", "賓夕凡尼亞州"), ("yue_Hans", "宾夕凡尼亚州"), ("zh", "宾夕法尼亚州"), ("zu", "Pennsylvania")]),
                        unofficial_name_list: ["Pennsylvania"].to_vec(),
                    }
                ),
                (
                    "PR",
                    Subdivision{
                        name: "Puerto Rico",
                        country_alpha2: Alpha2::US,
                        code: "PR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.7290818), longitude: Some(-117.1517016), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{1112e} 𑄢\u{11128}𑄇\u{1112e}"), ("en", "Puerto Rico")]),
                        unofficial_name_list: ["Puerto Rico"].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "Rhode Island",
                        country_alpha2: Alpha2::US,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5800945), longitude: Some(-71.4774291), max_latitude: Some(42.0188), min_latitude: Some(41.1466563), max_longitude: Some(-71.1205471), min_longitude: Some(-71.8923425)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rhode Island"), ("am", "ሮድ አይላንድ"), ("ar", "رود آيلاند"), ("az", "Rod-Aylend"), ("be", "Штат Род-Айленд"), ("bg", "Род Айлънд"), ("bn", "রোড আইল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Rhode Island"), ("ca", "Rhode Island"), ("ccp", "𑄢\u{1112e}𑄖\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Rhode Island"), ("cs", "Rhode Island"), ("cy", "Rhode Island"), ("da", "Rhode Island"), ("de", "Rhode Island"), ("el", "Ρόουντ Άιλαντ"), ("en", "Rhode Island"), ("es", "Rhode Island"), ("et", "Rhode Island"), ("eu", "Rhode Island"), ("fa", "رود آیلند"), ("fi", "Rhode Island"), ("fr", "Rhode Island"), ("ga", "Oileán Rhode"), ("gl", "Rhode Island"), ("gu", "ર\u{acd}હોડ આયલ\u{ac7}ન\u{acd}ડ"), ("he", "רוד איילנד"), ("hi", "रोड आइल\u{948}\u{902}ड"), ("hr", "Rhode Island"), ("hu", "Rhode Island"), ("hy", "Ռոդ Այլենդ"), ("id", "Rhode Island"), ("ig", "Àlá mmírí Rhode"), ("is", "Rhode Island"), ("it", "Rhode Island"), ("ja", "ロードアイランド州"), ("jv", "Rhode Island"), ("ka", "როდ-აილენდი"), ("kk", "Род-Айленд"), ("kn", "ರೋಡ\u{ccd} ಐಲ\u{cc6}ಂಡ\u{ccd}"), ("ko", "로드아일랜드 주"), ("lt", "Rod Ailandas"), ("lv", "Rodailenda"), ("mk", "Род Ајленд"), ("ml", "റോഡ\u{d4d} ഐലൻഡ\u{d4d}"), ("mn", "Рөүд-Айленд"), ("mr", "र\u{94d}\u{200d}होड आयल\u{902}ड"), ("ms", "Rhode Island"), ("my", "ရ\u{102f}ဒ\u{103a}အ\u{102d}\u{102f}င\u{103a}းလန\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Rhode Island"), ("ne", "रोड आइल\u{94d}याण\u{94d}ड"), ("nl", "Rhode Island"), ("no", "Rhode Island"), ("pa", "ਰ\u{a4b}ਡ ਟਾਪ\u{a42}"), ("pl", "Rhode Island"), ("pt", "Rhode Island"), ("ro", "Rhode Island"), ("ru", "Род-Айленд"), ("si", "රෝධේ ද\u{dd6}පත"), ("sk", "Rhode Island"), ("sl", "Rhode Island"), ("sq", "Rhode Island"), ("sr", "Роуд Ајланд"), ("sr_Latn", "Roud Ajland"), ("sv", "Rhode Island"), ("sw", "Rhode Island"), ("ta", "றோட\u{bcd} த\u{bc0}வு"), ("te", "ర\u{c4b}డ\u{c4d} ఐలండ\u{c4d}"), ("th", "ร\u{e31}ฐโรดไอแลนด\u{e4c}"), ("tr", "Rhode Island"), ("uk", "Род-Айленд"), ("ur", "رہوڈ آئی لینڈ"), ("uz", "Rod-Aylend"), ("vi", "Rhode Island"), ("yo", "Erékùṣù Rhode"), ("yo_BJ", "Erékùshù Rhode"), ("yue", "羅德島州"), ("yue_Hans", "罗德岛州"), ("zh", "羅德島州")]),
                        unofficial_name_list: ["Rhode Island"].to_vec(),
                    }
                ),
                (
                    "SC",
                    Subdivision{
                        name: "South Carolina",
                        country_alpha2: Alpha2::US,
                        code: "SC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.836081), longitude: Some(-81.1637245), max_latitude: Some(35.2155401), min_latitude: Some(32.0345599), max_longitude: Some(-78.5408175), min_longitude: Some(-83.35325879999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Carolina"), ("am", "ሳውዝ ካሮላይና"), ("ar", "كارولاينا الجنوبية"), ("az", "Cənubi Karolina"), ("be", "Штат Паўднёвая Караліна"), ("bg", "Южна Каролина"), ("bn", "স\u{9be}উথ ক\u{9cd}য\u{9be}রোল\u{9be}ইন\u{9be}"), ("bs", "Južna Karolina"), ("ca", "Carolina del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄇\u{11133}𑄠𑄢\u{1112e}𑄣\u{11128}𑄚"), ("ceb", "South Carolina"), ("cs", "Jižní Karolína"), ("cy", "De Carolina"), ("da", "South Carolina"), ("de", "South Carolina"), ("el", "Νότια Καρολίνα"), ("en", "South Carolina"), ("es", "Carolina del Sur"), ("et", "Lõuna-Carolina"), ("eu", "Hego Carolina"), ("fa", "کارولینای جنوبی"), ("fi", "Etelä-Carolina"), ("fr", "Caroline du Sud"), ("ga", "Carolina Theas"), ("gl", "Carolina do Sur"), ("gu", "સાઉથ ક\u{ac7}રોલિના"), ("he", "קרוליינה הדרומית"), ("hi", "दक\u{94d}षिणी क\u{947}रोलाइना"), ("hr", "Južna Karolina"), ("hu", "Dél-Karolina"), ("hy", "Հարավային Կարոլինա"), ("id", "Carolina Selatan"), ("ig", "Nleda anyanwu Kàròlina"), ("is", "Suður-Karólína"), ("it", "Carolina del Sud"), ("ja", "サウスカロライナ州"), ("jv", "South Carolina"), ("ka", "სამხრეთი კაროლინა"), ("kk", "Оңтүстік Каролина"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಕ\u{cc6}ರೊಲ\u{cbf}ನಾ"), ("ko", "사우스캐롤라이나 주"), ("lt", "Pietų Karolina"), ("lv", "Dienvidkarolīna"), ("mk", "Јужна Каролина"), ("ml", "തെക\u{d4d}കൻ കരൊലൈന"), ("mr", "साउथ क\u{945}रोलिना"), ("ms", "Carolina Selatan"), ("my", "တောင\u{103a}ကယ\u{103a}ရ\u{102d}\u{102f}လ\u{102d}\u{102f}င\u{103a}းနားပြည\u{103a}နယ\u{103a}"), ("nb", "Sør-Carolina"), ("ne", "साउथ क\u{94d}यारोलाइना"), ("nl", "South Carolina"), ("no", "Sør-Carolina"), ("pa", "ਦ\u{a71}ਖਣੀ ਕ\u{a48}ਰ\u{a4b}ਲੀਨਾ"), ("pl", "Karolina Południowa"), ("pt", "Carolina do Sul"), ("ro", "Carolina de Sud"), ("ru", "Южная Каролина"), ("si", "දක\u{dd4}ණ\u{dd4} කැරොල\u{dd2}න\u{dcf}"), ("sk", "Južná Karolína"), ("sl", "Južna Karolina"), ("sq", "South Carolina"), ("sr", "Јужна Каролина"), ("sr_Latn", "Južna Karolina"), ("sv", "South Carolina"), ("sw", "South Carolina"), ("ta", "தென\u{bcd} கரொலைன\u{bbe}"), ("te", "దక\u{c4d}ష\u{c3f}ణ కర\u{c4a}ల\u{c3f}న\u{c3e}"), ("th", "ร\u{e31}ฐเซาท\u{e4c}แคโรไลนา"), ("tr", "Güney Karolina"), ("uk", "Південна Кароліна"), ("ur", "جنوبی کیرولینا"), ("uz", "Janubiy Karolina"), ("vi", "Nam Carolina"), ("yo", "Gúúsù Carolina"), ("yo_BJ", "Gúúsù Carolina"), ("yue", "南卡羅萊納州"), ("yue_Hans", "南卡罗莱纳州"), ("zh", "南卡罗来纳州")]),
                        unofficial_name_list: ["South Carolina"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "South Dakota",
                        country_alpha2: Alpha2::US,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9695148), longitude: Some(-99.9018131), max_latitude: Some(45.945713), min_latitude: Some(42.479686), max_longitude: Some(-96.436589), min_longitude: Some(-104.0577391)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Suid-Dakota"), ("am", "ደቡብ ዳኮታ"), ("ar", "داكوتا الجنوبية"), ("az", "Cənubi Dakota"), ("be", "Штат Паўднёвая Дакота"), ("bg", "Южна Дакота"), ("bn", "স\u{9be}উথ ড\u{9be}কোট\u{9be}"), ("bs", "Južna Dakota"), ("ca", "Dakota del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄓𑄇\u{1112e}𑄑"), ("ceb", "South Dakota"), ("cs", "Jižní Dakota"), ("cy", "De Dakota"), ("da", "South Dakota"), ("de", "South Dakota"), ("el", "Νότια Ντακότα"), ("en", "South Dakota"), ("es", "Dakota del Sur"), ("et", "Lõuna-Dakota"), ("eu", "Hego Dakota"), ("fa", "داکوتای جنوبی"), ("fi", "Etelä-Dakota"), ("fr", "Dakota du Sud"), ("ga", "Dakota Theas"), ("gl", "Dacota do Sur"), ("gu", "સાઉથ ડકોટા"), ("ha", "South Dakota"), ("ha_NE", "South Dakota"), ("he", "דקוטה הדרומית"), ("hi", "दक\u{94d}षिण डकोटा"), ("hr", "Južna Dakota"), ("hu", "Dél-Dakota"), ("hy", "Հարավային Դակոթա"), ("id", "Dakota Selatan"), ("ig", "Sawt Dakótạ"), ("is", "Suður-Dakóta"), ("it", "Dakota del Sud"), ("ja", "サウスダコタ州"), ("jv", "South Dakota"), ("ka", "სამხრეთი დაკოტა"), ("kk", "Оңтүстік Дакота"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಡಕೋಟಾ"), ("ko", "사우스다코타 주"), ("lt", "Pietų Dakota"), ("lv", "Dienviddakota"), ("mk", "Јужна Дакота"), ("ml", "തെക\u{d4d}കൻ ഡക\u{d4d}കോട\u{d4d}ട"), ("mn", "Өмнөд Дакота"), ("mr", "साउथ डकोटा"), ("ms", "Dakota Selatan"), ("my", "တောင\u{103a}ဒါက\u{102d}\u{102f}တာပြည\u{103a}နယ\u{103a}"), ("nb", "Sør-Dakota"), ("ne", "साउथ ड\u{947}कोटा"), ("nl", "South Dakota"), ("no", "Sør-Dakota"), ("pa", "ਦ\u{a71}ਖਣੀ ਡਕ\u{a4b}ਟਾ"), ("pl", "Dakota Południowa"), ("pt", "Dakota do Sul"), ("ro", "Dakota de Sud"), ("ru", "Южная Дакота"), ("si", "දක\u{dd4}ණ\u{dd4} ඩකොට\u{dcf}"), ("sk", "Južná Dakota"), ("sl", "Južna Dakota"), ("sq", "South Dakota"), ("sr", "Јужна Дакота"), ("sr_Latn", "Južna Dakota"), ("sv", "South Dakota"), ("sw", "South Dakota"), ("ta", "தெற\u{bcd}கு டகோட\u{bcd}ட\u{bbe}"), ("te", "స\u{c4c}త\u{c4d} డక\u{c4b}ట\u{c3e}"), ("th", "ร\u{e31}ฐเซาท\u{e4c}ดาโคตา"), ("tr", "Güney Dakota"), ("uk", "Південна Дакота"), ("ur", "جنوبی ڈکوٹا"), ("uz", "Janubiy Dakota"), ("vi", "Nam Dakota"), ("yo", "Gúúsù Dakota"), ("yo_BJ", "Gúúsù Dakota"), ("yue", "南達科他州"), ("yue_Hans", "南达科他州"), ("zh", "南达科他州")]),
                        unofficial_name_list: ["South Dakota"].to_vec(),
                    }
                ),
                (
                    "TN",
                    Subdivision{
                        name: "Tennessee",
                        country_alpha2: Alpha2::US,
                        code: "TN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.5174913), longitude: Some(-86.5804473), max_latitude: Some(36.678118), min_latitude: Some(34.9829239), max_longitude: Some(-81.6469), min_longitude: Some(-90.3102978)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tennessee"), ("am", "ቴነሲ"), ("ar", "تينيسي"), ("az", "Tennessi"), ("be", "Штат Тэнесі"), ("bg", "Тенеси"), ("bn", "টেনেসী"), ("bs", "Tennessee"), ("ca", "Tennessee"), ("ccp", "𑄑𑄬𑄚\u{11129}𑄥\u{11129}"), ("ceb", "Tennessee"), ("cs", "Tennessee"), ("cy", "Tennessee"), ("da", "Tennessee"), ("de", "Tennessee"), ("el", "Τενεσί"), ("en", "Tennessee"), ("es", "Tennessee"), ("et", "Tennessee"), ("eu", "Tennessee"), ("fa", "تنسی"), ("fi", "Tennessee"), ("fr", "Tennessee"), ("ga", "Tennessee"), ("gl", "Tennessee"), ("gu", "ટ\u{ac7}ન\u{ac7}સી"), ("he", "טנסי"), ("hi", "ट\u{947}न\u{947}सी"), ("hr", "Tennessee"), ("hu", "Tennessee"), ("hy", "Թենեսի"), ("id", "Tennessee"), ("ig", "Tennessee"), ("is", "Tennessee"), ("it", "Tennessee"), ("ja", "テネシー州"), ("jv", "Tennessee"), ("ka", "ტენესი"), ("kk", "Теннесси"), ("kn", "ಟ\u{cc6}ನ\u{ccd}ನ\u{cc6}ಸ\u{ccd}ಸೀ"), ("ko", "테네시 주"), ("lt", "Tenesis"), ("lv", "Tenesī"), ("mk", "Тенеси"), ("ml", "ടെന\u{d4d}നസി"), ("mn", "Теннесси"), ("mr", "ट\u{947}न\u{947}सी"), ("ms", "Tennessee"), ("my", "တင\u{103a}နက\u{103a}ဆ\u{102e}ပြည\u{103a}နယ\u{103a}"), ("nb", "Tennessee"), ("ne", "ट\u{947}न\u{947}सी"), ("nl", "Tennessee"), ("no", "Tennessee"), ("pa", "ਟ\u{a48}ਨ\u{a47}ਸੀ"), ("pl", "Tennessee"), ("pt", "Tennessee"), ("ro", "Tennessee"), ("ru", "Теннесси"), ("si", "ටෙන\u{dca}නෙස\u{dca}සේ"), ("sk", "Tennessee"), ("sl", "Tennessee"), ("sq", "Tennessee"), ("sr", "Тенеси"), ("sr_Latn", "Tenesi"), ("sv", "Tennessee"), ("sw", "Tennessee"), ("ta", "டென\u{bcd}னிசி"), ("te", "ట\u{c47}నస\u{c4d}స\u{c40}"), ("th", "ร\u{e31}ฐเทนเนสซ\u{e35}"), ("tr", "Tennessee"), ("uk", "Теннессі"), ("ur", "ٹینیسی"), ("uz", "Tennessi"), ("vi", "Tennessee"), ("yo", "Tennessee"), ("yo_BJ", "Tennessee"), ("yue", "田納西州"), ("yue_Hans", "田纳西州"), ("zh", "田纳西州")]),
                        unofficial_name_list: ["Tennessee"].to_vec(),
                    }
                ),
                (
                    "TX",
                    Subdivision{
                        name: "Texas",
                        country_alpha2: Alpha2::US,
                        code: "TX",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(31.9685988), longitude: Some(-99.9018131), max_latitude: Some(36.5007041), min_latitude: Some(25.8371638), max_longitude: Some(-93.5080389), min_longitude: Some(-106.6456461)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Texas"), ("am", "ቴክሳስ"), ("ar", "تكساس"), ("az", "Texas"), ("be", "Штат Тэхас"), ("bg", "Тексас"), ("bn", "টেক\u{9cd}স\u{9be}স"), ("bs", "Texas"), ("ca", "Texas"), ("ccp", "𑄑𑄬𑄇\u{11134}𑄥𑄌\u{11134}"), ("ceb", "Texas"), ("cs", "Texas"), ("cy", "Texas"), ("da", "Texas"), ("de", "Texas"), ("el", "Τέξας"), ("en", "Texas"), ("es", "Texas"), ("et", "Texas"), ("eu", "Texas"), ("fa", "تگزاس"), ("fi", "Texas"), ("fr", "Texas"), ("ga", "Texas"), ("gl", "Texas"), ("gu", "ટ\u{ac7}ક\u{acd}સાસ"), ("ha", "Texas"), ("ha_NE", "Texas"), ("he", "טקסס"), ("hi", "ट\u{945}क\u{94d}सस"), ("hr", "Teksas"), ("hu", "Texas"), ("hy", "Տեխաս"), ("id", "Texas"), ("ig", "Texas"), ("is", "Texas"), ("it", "Texas"), ("ja", "テキサス州"), ("jv", "Texas"), ("ka", "ტეხასი"), ("kk", "Техас"), ("kn", "ಟ\u{cc6}ಕ\u{ccd}ಸಸ\u{ccd}"), ("ko", "텍사스 주"), ("lt", "Teksasas"), ("lv", "Teksasa"), ("mk", "Тексас"), ("ml", "ടെക\u{d4d}സസ\u{d4d}"), ("mn", "Техас"), ("mr", "ट\u{947}क\u{94d}सास"), ("ms", "Texas"), ("my", "တက\u{1039}ကဆပ\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Texas"), ("ne", "ट\u{947}क\u{94d}सस"), ("nl", "Texas"), ("no", "Texas"), ("pa", "ਟ\u{a48}ਕਸਸ"), ("pl", "Teksas"), ("pt", "Texas"), ("ro", "Texas"), ("ru", "Техас"), ("si", "ටෙක\u{dca}ස\u{dcf}ස\u{dca}"), ("sk", "Texas"), ("sl", "Teksas"), ("so", "Texas"), ("sq", "Texas"), ("sr", "Тексас"), ("sr_Latn", "Teksas"), ("sv", "Texas"), ("sw", "Texas"), ("ta", "டெக\u{bcd}சஸ\u{bcd}"), ("te", "ట\u{c46}క\u{c4d}సస\u{c4d}"), ("th", "ร\u{e31}ฐเทกซ\u{e31}ส"), ("tk", "Tehas"), ("tr", "Teksas"), ("uk", "Техас"), ("ur", "ٹیکساس"), ("uz", "Texas"), ("vi", "Texas"), ("yo", "Texas"), ("yo_BJ", "Texas"), ("yue", "德州"), ("yue_Hans", "德州"), ("zh", "得克萨斯州"), ("zu", "Texas")]),
                        unofficial_name_list: ["Texas"].to_vec(),
                    }
                ),
                (
                    "UM",
                    Subdivision{
                        name: "United States Minor Outlying Islands",
                        country_alpha2: Alpha2::US,
                        code: "UM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.2021222), longitude: Some(-177.3804525), max_latitude: Some(28.2150965), min_latitude: Some(28.1963806), max_longitude: Some(-177.3695147), min_longitude: Some(-177.3946094)}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃\u{11128}𑄅\u{1112a}. 𑄃𑄬𑄌\u{11134}. 𑄃𑄅\u{1112a}𑄖\u{11134}𑄣\u{1112d}𑄃\u{11128}\u{11101} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("en", "U.S. Outlying Islands")]),
                        unofficial_name_list: ["United States Minor Outlying Islands"].to_vec(),
                    }
                ),
                (
                    "UT",
                    Subdivision{
                        name: "Utah",
                        country_alpha2: Alpha2::US,
                        code: "UT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.3209801), longitude: Some(-111.0937311), max_latitude: Some(42.001618), min_latitude: Some(36.99790309999999), max_longitude: Some(-109.0410581), min_longitude: Some(-114.0529979)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Utah"), ("am", "ዩታህ"), ("ar", "يوتا"), ("az", "Yuta"), ("be", "Штат Юта"), ("bg", "Юта"), ("bn", "ইউট\u{9be}"), ("bs", "Utah"), ("ca", "Utah"), ("ccp", "𑄅\u{1112a}𑄑𑄦\u{11134}"), ("ceb", "Utah"), ("cs", "Utah"), ("cy", "Utah"), ("da", "Utah"), ("de", "Utah"), ("el", "Γιούτα"), ("en", "Utah"), ("es", "Utah"), ("et", "Utah"), ("eu", "Utah"), ("fa", "یوتا"), ("fi", "Utah"), ("fr", "Utah"), ("ga", "Utah"), ("gl", "Utah"), ("gu", "ય\u{ac1}ટા"), ("ha", "Utah"), ("ha_NE", "Utah"), ("he", "יוטה"), ("hi", "य\u{942}टाह"), ("hr", "Utah"), ("hu", "Utah"), ("hy", "Յուտա"), ("id", "Utah"), ("ig", "Yútạh"), ("is", "Utah"), ("it", "Utah"), ("ja", "ユタ州"), ("jv", "Utah"), ("ka", "იუტა"), ("kk", "Юта"), ("kn", "ಯ\u{cc2}ಟ"), ("ko", "유타 주"), ("lt", "Juta"), ("lv", "Jūta"), ("mk", "Јута"), ("ml", "യ\u{d42}റ\u{d4d}റ\u{d3e}"), ("mn", "Юта"), ("mr", "य\u{941}टा"), ("ms", "Utah"), ("my", "ယ\u{1030}းတားပြည\u{103a}နယ\u{103a}"), ("nb", "Utah"), ("ne", "उताह"), ("nl", "Utah"), ("no", "Utah"), ("pa", "ਯ\u{a42}ਟਾ"), ("pl", "Utah"), ("pt", "Utah"), ("ro", "Utah"), ("ru", "Юта"), ("si", "ඌටහ\u{dca}"), ("sk", "Utah"), ("sl", "Utah"), ("sq", "Utah"), ("sr", "Јута"), ("sr_Latn", "Juta"), ("sv", "Utah"), ("sw", "Utah"), ("ta", "யூட\u{bcd}ட\u{bbe}"), ("te", "యూట\u{c3e}"), ("th", "ร\u{e31}ฐย\u{e39}ทาห\u{e4c}"), ("tr", "Utah"), ("uk", "Юта"), ("ur", "یوٹاہ"), ("uz", "Yuta"), ("vi", "Utah"), ("yo", "Utah"), ("yo_BJ", "Utah"), ("yue", "猶他州"), ("yue_Hans", "犹他州"), ("zh", "犹他州"), ("zu", "Utah")]),
                        unofficial_name_list: ["Utah"].to_vec(),
                    }
                ),
                (
                    "VA",
                    Subdivision{
                        name: "Virginia",
                        country_alpha2: Alpha2::US,
                        code: "VA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.4315734), longitude: Some(-78.6568942), max_latitude: Some(39.466012), min_latitude: Some(36.5407589), max_longitude: Some(-75.2421573), min_longitude: Some(-83.675415)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Virginië"), ("am", "ቨርጂኒያ"), ("ar", "فرجينيا"), ("az", "Virciniya"), ("be", "Штат Вірджынія"), ("bg", "Вирджиния"), ("bn", "ভ\u{9be}র\u{9cd}জিনিয\u{9bc}\u{9be}"), ("bs", "Virginia"), ("ca", "Virgínia"), ("ccp", "𑄞𑄢\u{11134}𑄎\u{11128}𑄚\u{11128}𑄠"), ("ceb", "Virginia"), ("cs", "Virginie"), ("cy", "Virginia"), ("da", "Virginia"), ("de", "Virginia"), ("el", "Βιρτζίνια"), ("en", "Virginia"), ("es", "Virginia"), ("et", "Virginia"), ("eu", "Virginia"), ("fa", "ویرجینیا"), ("fi", "Virginia"), ("fr", "Virginie"), ("ga", "Virginia"), ("gl", "Virxinia"), ("gu", "વર\u{acd}જિનિયા"), ("ha", "Virginia"), ("ha_NE", "Virginia"), ("he", "וירג׳יניה"), ("hi", "वर\u{94d}जीनिया"), ("hr", "Virginia"), ("hu", "Virginia"), ("hy", "Վիրջինիա"), ("id", "Virginia"), ("ig", "Végíníyà"), ("is", "Virginía"), ("it", "Virginia"), ("ja", "バージニア州"), ("jv", "Virginia"), ("ka", "ვირჯინია"), ("kk", "Виргиния"), ("kn", "ವರ\u{ccd}ಜೀನ\u{cbf}ಯ"), ("ko", "버지니아 주"), ("ky", "Виргиния штаты"), ("lt", "Virdžinija"), ("lv", "Virdžīnija"), ("mk", "Вирџинија"), ("ml", "വിർജീനിയ"), ("mn", "Виржини"), ("mr", "व\u{94d}हर\u{94d}जिनिया"), ("ms", "Virginia"), ("my", "ဗာဂျ\u{102e}းန\u{102e}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "Virginia"), ("ne", "भर\u{94d}जिनिया"), ("nl", "Virginia"), ("no", "Virginia"), ("pa", "ਵਰਜਿਨੀਆ"), ("pl", "Wirginia"), ("pt", "Virgínia"), ("ro", "Virginia"), ("ru", "Виргиния"), ("si", "වර\u{dca}ජ\u{dd2}න\u{dd2}ය\u{dcf}"), ("sk", "Virgínia"), ("sl", "Virginija"), ("sq", "Virginia"), ("sr", "Вирџинија"), ("sr_Latn", "Virdžinija"), ("sv", "Virginia"), ("sw", "Virginia"), ("ta", "வர\u{bcd}ஜ\u{bc0}னிய\u{bbe}"), ("te", "వర\u{c4d}జ\u{c40}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐเวอร\u{e4c}จ\u{e34}เน\u{e35}ย"), ("tr", "Virjinya"), ("uk", "Вірджинія"), ("ur", "ورجینیا"), ("uz", "Virjiniya"), ("vi", "Virginia"), ("yo", "Firginia"), ("yo_BJ", "Firginia"), ("yue", "維珍尼亞州"), ("yue_Hans", "维珍尼亚州"), ("zh", "弗吉尼亚州"), ("zu", "Virginia")]),
                        unofficial_name_list: ["Virginia"].to_vec(),
                    }
                ),
                (
                    "VI",
                    Subdivision{
                        name: "Virgin Islands, U.S.",
                        country_alpha2: Alpha2::US,
                        code: "VI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(33.8569878), longitude: Some(-84.5904089), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::OutlyingArea,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄃\u{11128}𑄅\u{1112a}. 𑄃𑄬𑄌\u{11134} 𑄞𑄢\u{11134}𑄎\u{11128}𑄚\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("en", "U.S. Virgin Islands")]),
                        unofficial_name_list: ["Virgin Islands, U.S."].to_vec(),
                    }
                ),
                (
                    "VT",
                    Subdivision{
                        name: "Vermont",
                        country_alpha2: Alpha2::US,
                        code: "VT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.5588028), longitude: Some(-72.57784149999999), max_latitude: Some(45.0166591), min_latitude: Some(42.7268499), max_longitude: Some(-71.46503899999999), min_longitude: Some(-73.43055980000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vermont"), ("am", "ቬርሞንት"), ("ar", "فيرمونت"), ("az", "Vermont"), ("be", "Штат Вермонт"), ("bg", "Върмонт"), ("bn", "ভ\u{9be}র\u{9cd}মন\u{9cd}ট"), ("bs", "Vermont"), ("ca", "Vermont"), ("ccp", "𑄞𑄢\u{11134}𑄟𑄚\u{11134}𑄑\u{11134}"), ("ceb", "Vermont"), ("cs", "Vermont"), ("cy", "Vermont"), ("da", "Vermont"), ("de", "Vermont"), ("el", "Βερμόντ"), ("en", "Vermont"), ("es", "Vermont"), ("et", "Vermont"), ("eu", "Vermont"), ("fa", "ورمونت"), ("fi", "Vermont"), ("fr", "Vermont"), ("ga", "Vermont"), ("gl", "Vermont"), ("gu", "વર\u{acd}મોન\u{acd}ટ"), ("ha", "Vermont"), ("ha_NE", "Vermont"), ("he", "ורמונט"), ("hi", "वर\u{94d}मा\u{902}ट"), ("hr", "Vermont"), ("hu", "Vermont"), ("hy", "Վերմոնտ"), ("id", "Vermont"), ("ig", "Vermont"), ("is", "Vermont"), ("it", "Vermont"), ("ja", "バーモント州"), ("jv", "Vermont"), ("ka", "ვერმონტი"), ("kk", "Вермонт"), ("kn", "ವರ\u{ccd}ಮೊಂಟ\u{ccd}"), ("ko", "버몬트 주"), ("lt", "Vermontas"), ("lv", "Vērmonta"), ("mk", "Вермонт"), ("ml", "വെർമോണ\u{d4d}ട\u{d4d}"), ("mn", "Вермонт"), ("mr", "व\u{94d}हरमा\u{901}ट"), ("ms", "Vermont"), ("my", "ဗားမောင\u{1037}\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Vermont"), ("ne", "भर\u{94d}मा\u{902}ट"), ("nl", "Vermont"), ("no", "Vermont"), ("pa", "ਵਰਮਾ\u{a02}ਟ"), ("pl", "Vermont"), ("pt", "Vermont"), ("ro", "Vermont"), ("ru", "Вермонт"), ("si", "වර\u{dca}මොන\u{dca}ට\u{dca}"), ("sk", "Vermont"), ("sl", "Vermont"), ("sq", "Vermont"), ("sr", "Вермонт"), ("sr_Latn", "Vermont"), ("sv", "Vermont"), ("sw", "Vermont"), ("ta", "வெர\u{bcd}ம\u{bbe}ன\u{bcd}ட\u{bcd}"), ("te", "వ\u{c46}ర\u{c4d}మ\u{c3e}ంట\u{c4d}"), ("th", "ร\u{e31}ฐเวอร\u{e4c}มอนต\u{e4c}"), ("tr", "Vermont"), ("uk", "Вермонт"), ("ur", "ورمونٹ"), ("uz", "Vermont"), ("vi", "Vermont"), ("yo", "Fermont"), ("yo_BJ", "Fermont"), ("yue", "佛蒙特州"), ("yue_Hans", "佛蒙特州"), ("zh", "佛蒙特州"), ("zu", "Vermont")]),
                        unofficial_name_list: ["Vermont"].to_vec(),
                    }
                ),
                (
                    "WA",
                    Subdivision{
                        name: "Washington",
                        country_alpha2: Alpha2::US,
                        code: "WA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.7510741), longitude: Some(-120.7401386), max_latitude: Some(49.0024305), min_latitude: Some(45.5485987), max_longitude: Some(-116.91558), min_longitude: Some(-124.7857167)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Washington²"), ("am", "ዋሽንግተን"), ("ar", "واشنطن"), ("az", "Vaşinqton²"), ("be", "Вашынгтон²"), ("bg", "Вашингтон²"), ("bn", "ওয\u{9bc}\u{9be}শিংটন"), ("bs", "Washington"), ("ca", "Washington"), ("ccp", "𑄤𑄥\u{11128}\u{11101}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Washington"), ("cs", "Washington"), ("cy", "Washington"), ("da", "Washington"), ("de", "Washington"), ("el", "Ουάσινγκτον²"), ("en", "Washington"), ("es", "Washington"), ("et", "Washingtoni osariik"), ("eu", "Washington²"), ("fa", "ایالت واشینگتن"), ("fi", "Washington²"), ("fr", "État de Washington"), ("ga", "Washington"), ("gl", "Estado de Washington"), ("gu", "વૉશિ\u{a82}ગ\u{acd}ટન"), ("he", "וושינגטון"), ("hi", "वाशि\u{902}गटन"), ("hr", "Washington²"), ("hu", "Washington²"), ("hy", "Վաշինգտոն"), ("id", "Washington²"), ("ig", "Washington"), ("is", "Washington²"), ("it", "Washington²"), ("ja", "ワシントン州"), ("jv", "Washington"), ("ka", "ვაშინგტონის შტატი"), ("kk", "Вашингтон"), ("kn", "ವಾಶ\u{cbf}ಂಗ\u{ccd}ಟನ\u{ccd} ರಾಜ\u{ccd}ಯ"), ("ko", "워싱턴 주"), ("lt", "Vašingtonas²"), ("lv", "Vašingtona²"), ("mk", "Вашингтон"), ("ml", "വ\u{d3e}ഷിങ\u{d4d}ടൺ²"), ("mn", "Вашингтон Муж Улс"), ("mr", "वॉशि\u{902}ग\u{94d}टन²"), ("ms", "Washington²"), ("my", "ဝါရ\u{103e}င\u{103a}တန\u{103a}ပြည\u{103a}နယ\u{103a}"), ("nb", "Washington"), ("ne", "वाशिङ\u{94d}गटन डि.सि."), ("nl", "Washington"), ("no", "Washington"), ("pa", "ਵਾਸ\u{a3c}ਿ\u{a70}ਗਟਨ"), ("pl", "Waszyngton²"), ("pt", "Washington²"), ("ro", "Washington²"), ("ru", "Вашингтон²"), ("sd", "واشنگٽن رياست"), ("si", "වොෂ\u{dd2}න\u{dca}ටන\u{dca}²"), ("sk", "Washington"), ("sl", "Washington²"), ("so", "Washington"), ("sq", "Washington"), ("sr", "Вашингтон²"), ("sr_Latn", "Vašington²"), ("sv", "Washington"), ("sw", "Washington²"), ("ta", "வ\u{bbe}ஷிங\u{bcd}டன\u{bcd}"), ("te", "వ\u{c3e}ష\u{c3f}ంగ\u{c4d}టన\u{c4d}²"), ("th", "ร\u{e31}ฐวอช\u{e34}งต\u{e31}น"), ("tr", "Vaşington"), ("uk", "Вашингтон²"), ("ur", "ریاست واشنگٹن"), ("uz", "Vashington shtati"), ("vi", "Washington²"), ("yo", "Ìpínlẹ\u{300} Washington"), ("yo_BJ", "Ìpínlɛ\u{300} Washington"), ("yue", "華盛頓州"), ("yue_Hans", "华盛顿州"), ("zh", "华盛顿州"), ("zu", "Washington")]),
                        unofficial_name_list: ["Washington"].to_vec(),
                    }
                ),
                (
                    "WI",
                    Subdivision{
                        name: "Wisconsin",
                        country_alpha2: Alpha2::US,
                        code: "WI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.7844397), longitude: Some(-88.7878678), max_latitude: Some(47.0807296), min_latitude: Some(42.491864), max_longitude: Some(-86.7639767), min_longitude: Some(-92.88943309999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wisconsin"), ("am", "ዊስኮንሲን"), ("ar", "ويسكونسن"), ("as", "উইচকনচিন"), ("az", "Viskonsin"), ("be", "Штат Вісконсін"), ("bg", "Уисконсин"), ("bn", "উইসকনসিন"), ("bs", "Wisconsin"), ("ca", "Wisconsin"), ("ccp", "𑄅\u{1112a}𑄃\u{11128}𑄌\u{11134}𑄇\u{11127}𑄚\u{11134}𑄥\u{11128}𑄚\u{11134}"), ("ceb", "Wisconsin (estado)"), ("cs", "Wisconsin"), ("cy", "Wisconsin"), ("da", "Wisconsin"), ("de", "Wisconsin"), ("el", "Ουισκόνσιν"), ("en", "Wisconsin"), ("es", "Wisconsin"), ("et", "Wisconsin"), ("eu", "Wisconsin"), ("fa", "ویسکانسین"), ("fi", "Wisconsin"), ("fr", "Wisconsin"), ("ga", "Wisconsin"), ("gl", "Wisconsin"), ("gu", "વિસ\u{acd}કોન\u{acd}સિન"), ("ha", "Wisconsin"), ("ha_NE", "Wisconsin"), ("he", "ויסקונסין"), ("hi", "विस\u{94d}कॉन\u{94d}सिन"), ("hr", "Wisconsin"), ("hu", "Wisconsin"), ("hy", "Վիսկոնսին"), ("id", "Wisconsin"), ("ig", "Wiskonsin"), ("is", "Wisconsin"), ("it", "Wisconsin"), ("ja", "ウィスコンシン州"), ("jv", "Wisconsin"), ("ka", "უისკონსინი"), ("kk", "Висконсин"), ("kn", "ವ\u{cbf}ಸ\u{ccd}ಕೊನ\u{ccd}\u{200c}ಸ\u{cbf}ನ\u{ccd}"), ("ko", "위스콘신 주"), ("ky", "Висконсин"), ("lt", "Viskonsinas"), ("lv", "Viskonsina"), ("mk", "Висконсин"), ("ml", "വിസ\u{d4d}കോൺസിൻ"), ("mn", "Висконсин"), ("mr", "विस\u{94d}कॉन\u{94d}सिन"), ("ms", "Wisconsin"), ("my", "ဝစ\u{1039}စက\u{103d}န\u{103a}းဆင\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Wisconsin"), ("ne", "विस\u{94d}कान\u{94d}सिन"), ("nl", "Wisconsin"), ("no", "Wisconsin"), ("pa", "ਵਿਸਕਾ\u{a02}ਸਨ"), ("pl", "Wisconsin"), ("pt", "Wisconsin"), ("ro", "Wisconsin"), ("ru", "Висконсин"), ("si", "ව\u{dd2}ස\u{dca}කොන\u{dca}ස\u{dd2}න\u{dca}"), ("sk", "Wisconsin"), ("sl", "Wisconsin"), ("sq", "Wisconsin"), ("sr", "Висконсин"), ("sr_Latn", "Viskonsin"), ("sv", "Wisconsin"), ("sw", "Wisconsin"), ("ta", "விஸ\u{bcd}கொன\u{bcd}சின\u{bcd}"), ("te", "వ\u{c3f}స\u{c4d}క\u{c3e}న\u{c4d}స\u{c3f}న\u{c4d}"), ("th", "ร\u{e31}ฐว\u{e34}สคอนซ\u{e34}น"), ("tr", "Wisconsin"), ("uk", "Вісконсин"), ("ur", "وسکونسن"), ("uz", "Viskonsin"), ("vi", "Wisconsin"), ("yo", "Wisconsin"), ("yo_BJ", "Wisconsin"), ("yue", "威斯康辛州"), ("yue_Hans", "威斯康辛州"), ("zh", "威斯康辛州")]),
                        unofficial_name_list: ["Wisconsin"].to_vec(),
                    }
                ),
                (
                    "WV",
                    Subdivision{
                        name: "West Virginia",
                        country_alpha2: Alpha2::US,
                        code: "WV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.5976262), longitude: Some(-80.4549026), max_latitude: Some(40.638801), min_latitude: Some(37.2015399), max_longitude: Some(-77.7189679), min_longitude: Some(-82.644413)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wes-Virginië"), ("am", "ዌስት ቨርጂኒያ"), ("ar", "فيرجينيا الغربية"), ("az", "Qərbi Virciniya"), ("be", "Штат Заходняя Вірджынія"), ("bg", "Западна Вирджиния"), ("bn", "পশ\u{9cd}চিম ভ\u{9be}র\u{9cd}জিনিয\u{9bc}\u{9be}"), ("bs", "Zapadna Virginia"), ("ca", "Virgínia de l’Oest"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄞𑄢\u{11134}𑄎\u{11128}𑄚\u{11128}𑄠"), ("ceb", "West Virginia"), ("cs", "Západní Virginie"), ("cy", "Gorllewin Virginia"), ("da", "West Virginia"), ("de", "West Virginia"), ("el", "Δυτική Βιρτζίνια"), ("en", "West Virginia"), ("es", "Virginia Occidental"), ("et", "Lääne-Virginia"), ("eu", "Mendebaldeko Virginia"), ("fa", "ویرجینیای غربی"), ("fi", "Länsi-Virginia"), ("fr", "Virginie-Occidentale"), ("ga", "Virginia Thiar"), ("gl", "Virxinia Occidental"), ("gu", "વ\u{ac7}સ\u{acd}ટ વર\u{acd}જિનિયા"), ("he", "וירג׳יניה המערבית"), ("hi", "पश\u{94d}चिमी वर\u{94d}जीनिया"), ("hr", "Zapadna Virginia"), ("hu", "Nyugat-Virginia"), ("hy", "Արևմտյան Վիրջինիա"), ("id", "Virginia Barat"), ("ig", "West Virginia"), ("is", "Vestur-Virginía"), ("it", "Virginia Occidentale"), ("ja", "ウェストバージニア州"), ("jv", "West Virginia"), ("ka", "დასავლეთი ვირჯინია"), ("kk", "Батыс Виргиния"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ವರ\u{ccd}ಜೀನ\u{cbf}ಯ"), ("ko", "웨스트버지니아 주"), ("lt", "Vakarų Virdžinija"), ("lv", "Rietumvirdžīnija"), ("mk", "Западна Вирџинија"), ("ml", "പടിഞ\u{d4d}ഞ\u{d3e}റൻ വിർജീന\u{d4d}യ"), ("mn", "Баруун Виржини"), ("mr", "व\u{947}स\u{94d}ट व\u{94d}हर\u{94d}जिनिया"), ("ms", "Virginia Barat"), ("my", "အနောက\u{103a} ဗာဂျ\u{102e}းန\u{102e}းယားပြည\u{103a}နယ\u{103a}"), ("nb", "Vest-Virginia"), ("ne", "व\u{947}स\u{94d}ट भर\u{94d}जिनिया"), ("nl", "West Virginia"), ("no", "Vest-Virginia"), ("pa", "ਪ\u{a71}ਛਮੀ ਵਰਜਿਨੀਆ"), ("pl", "Wirginia Zachodnia"), ("pt", "Virgínia Ocidental"), ("ro", "Virginia de Vest"), ("ru", "Западная Виргиния"), ("si", "බටහ\u{dd2}ර වර\u{dca}ජ\u{dd3}න\u{dd2}ය\u{dcf}"), ("sk", "Západná Virgínia"), ("sl", "Zahodna Virginija"), ("sq", "West Virginia"), ("sr", "Западна Вирџинија"), ("sr_Latn", "Zapadna Virdžinija"), ("sv", "West Virginia"), ("sw", "West Virginia"), ("ta", "மேற\u{bcd}கு வர\u{bcd}ஜ\u{bc0}னிய\u{bbe}"), ("te", "పశ\u{c4d}చ\u{c3f}మ వర\u{c4d}జ\u{c40}న\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐเวสต\u{e4c}เวอร\u{e4c}จ\u{e34}เน\u{e35}ย"), ("tr", "Batı Virginia"), ("uk", "Західна Вірджинія"), ("ur", "مغربی ورجینیا"), ("vi", "Tây Virginia"), ("yo", "Ìwọ\u{300}orùn Firginia"), ("yo_BJ", "Ìwɔ\u{300}orùn Firginia"), ("yue", "西維珍尼亞州"), ("yue_Hans", "西维珍尼亚州"), ("zh", "西維吉尼亞州")]),
                        unofficial_name_list: ["West Virginia"].to_vec(),
                    }
                ),
                (
                    "WY",
                    Subdivision{
                        name: "Wyoming",
                        country_alpha2: Alpha2::US,
                        code: "WY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0759678), longitude: Some(-107.2902839), max_latitude: Some(45.005904), min_latitude: Some(40.994746), max_longitude: Some(-104.0522449), min_longitude: Some(-111.056889)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wyoming"), ("am", "ዋዮሚንግ"), ("ar", "وايومنغ"), ("as", "ৱ\u{9be}য\u{9bc}মিং"), ("az", "Vayominq"), ("be", "Штат Ваёмінг"), ("bg", "Уайоминг"), ("bn", "ওয\u{9bc}\u{9be}ইয\u{9bc}োমিং"), ("bs", "Wyoming"), ("ca", "Wyoming"), ("ccp", "𑄃\u{11128}𑄠\u{1112e}𑄟\u{11128}\u{11101}"), ("ceb", "Wyoming"), ("cs", "Wyoming"), ("cy", "Wyoming"), ("da", "Wyoming"), ("de", "Wyoming"), ("el", "Ουαϊόμινγκ"), ("en", "Wyoming"), ("es", "Wyoming"), ("et", "Wyoming"), ("eu", "Wyoming"), ("fa", "وایومینگ"), ("fi", "Wyoming"), ("fr", "Wyoming"), ("ga", "Wyoming"), ("gl", "Wyoming"), ("gu", "વ\u{acd}યોમિ\u{a82}ગ"), ("ha", "Wyoming"), ("ha_NE", "Wyoming"), ("he", "ויומינג"), ("hi", "वायोमि\u{902}ग"), ("hr", "Wyoming"), ("hu", "Wyoming"), ("hy", "Վայոմինգ"), ("id", "Wyoming"), ("ig", "Waịómịn"), ("is", "Wyoming"), ("it", "Wyoming"), ("ja", "ワイオミング州"), ("jv", "Wyoming"), ("ka", "ვაიომინგი"), ("kk", "Вайоминг"), ("kn", "ವಯೋಮ\u{cbf}ಂಗ\u{ccd}"), ("ko", "와이오밍 주"), ("lt", "Vajomingas"), ("lv", "Vaiominga"), ("mk", "Вајоминг"), ("ml", "വയോമിങ\u{d4d}"), ("mn", "Вайоминг"), ("mr", "वायोमि\u{902}ग"), ("ms", "Wyoming"), ("my", "ဝ\u{102d}\u{102f}င\u{103a}အ\u{102d}\u{102f}းမင\u{103a}းပြည\u{103a}နယ\u{103a}"), ("nb", "Wyoming"), ("ne", "वायोमिङ"), ("nl", "Wyoming"), ("no", "Wyoming"), ("pa", "ਵਾਇਓਮਿ\u{a70}ਗ"), ("pl", "Wyoming"), ("pt", "Wyoming"), ("ro", "Wyoming"), ("ru", "Вайоминг"), ("si", "ව\u{dd2}යෝම\u{dd2}න\u{dca}ග\u{dca}"), ("sk", "Wyoming"), ("sl", "Wyoming"), ("sq", "Wyoming"), ("sr", "Вајоминг"), ("sr_Latn", "Vajoming"), ("sv", "Wyoming"), ("sw", "Wyoming"), ("ta", "வயோமிங\u{bcd}"), ("te", "వయ\u{c4b}మ\u{c3f}ంగ\u{c4d}"), ("th", "ร\u{e31}ฐไวโอม\u{e34}ง"), ("tr", "Wyoming"), ("uk", "Вайомінг"), ("ur", "وائیومنگ"), ("uz", "Vayoming"), ("vi", "Wyoming"), ("yo", "Wyoming"), ("yo_BJ", "Wyoming"), ("yue", "懷俄明州"), ("yue_Hans", "怀俄明州"), ("zh", "怀俄明州"), ("zu", "Wyoming")]),
                        unofficial_name_list: ["Wyoming"].to_vec(),
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
#[cfg(feature = "us")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::US,
        alpha3: Alpha3::USA,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{city}} {{region_short}} {{postalcode}}\n{{country}}",
        ),
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: CurrencyCode::USD,
        gec: Some(GEC::US),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::USA),
        iso_long_name: "The United States of America",
        iso_short_name: "United States of America",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("American"),
        number: "840",
        postal_code: true,
        postal_code_format: Some("(\\d{5})(?:[ \\-](\\d{4}))?"),
        region: Some(Region::Americas),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::NorthernAmerica),
        un_locode: "US",
        unofficial_name_list: [
            "United States",
            "USA",
            "Vereinigte Staaten von Amerika",
            "États-Unis",
            "Estados Unidos",
            "アメリカ合衆国",
            "Verenigde Staten",
            "Соединенные Штаты Америки",
        ]
        .to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "United States"),
            ("af", "Verenigde State"),
            ("ak", "United States"),
            ("am", "ጐሣሱጢ"),
            ("an", "United States"),
            ("ar", "الولايات المت\u{651}حدة"),
            (
                "as",
                "ম\u{9be}ৰ\u{9cd}কিন য\u{9c1}ক\u{9cd}তৰ\u{9be}ষ\u{9cd}ট\u{9cd}ৰ",
            ),
            ("ay", "United States"),
            ("az", "Birləşmiş Ştatlar"),
            ("ba", "United States"),
            ("be", "Злучаныя Штаты"),
            ("bg", "Съединени щати"),
            ("bi", "United States"),
            (
                "bn",
                "ম\u{9be}র\u{9cd}কিন য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
            ),
            (
                "bn_IN",
                "ম\u{9be}র\u{9cd}কিন য\u{9c1}ক\u{9cd}তর\u{9be}ষ\u{9cd}ট\u{9cd}র",
            ),
            ("br", "Stadoù Unanet"),
            ("bs", "SAD"),
            ("ca", "Estats Units"),
            ("ce", "United States"),
            ("ch", "United States"),
            ("cs", "Spojené státy"),
            ("cv", "United States"),
            ("cy", "Yr Unol Daleithiau"),
            ("da", "USA"),
            ("de", "Vereinigte Staaten"),
            ("dv", "United States"),
            (
                "dz",
                "ཡ\u{f74}་ན་ཡ\u{f7a}་ཊ\u{f7a}ཊ\u{f72}་ ས\u{f72}ཊ\u{f7a}ཊ\u{f72}ས\u{f72}།",
            ),
            ("ee", "United States"),
            ("el", "Ηνωμένες Πολιτείες"),
            ("en", "United States"),
            ("eo", "Usono"),
            ("es", "Estados Unidos"),
            ("et", "Ameerika Ühendriigid"),
            ("eu", "Estatu Batuak"),
            ("fa", "ایالات متحده\u{654} آمریکا"),
            ("ff", "United States"),
            ("fi", "Yhdysvallat"),
            ("fo", "Sambandsríki Amerika"),
            ("fr", "États-Unis"),
            ("fy", "United States"),
            ("ga", "Na Stáit Aontaithe"),
            ("gl", "Estados Unidos de América"),
            ("gn", "United States"),
            ("gu", "ય\u{ac1}નાઇટ\u{ac7}ડ સ\u{acd}ટ\u{ac7}ટ\u{acd}સ"),
            ("gv", "United States"),
            ("ha", "United States"),
            ("he", "ארצות הברית"),
            ("hi", "स\u{902}य\u{941}क\u{94d}त राज\u{94d}य"),
            ("hr", "Sjedinjene Države"),
            ("ht", "Etazini"),
            ("hu", "Egyesült Államok"),
            ("hy", "Ամէրիկայի Միացյալ Նահանգնէր"),
            ("ia", "Statos Unite"),
            ("id", "Amerika Serikat"),
            ("io", "United States"),
            ("is", "Bandaríkin"),
            ("it", "Stati Uniti"),
            ("iu", "United States"),
            ("ja", "米国"),
            ("ka", "ამერიკის შეერთებული შტატები"),
            ("ki", "United States"),
            ("kk", "АҚШ"),
            ("kl", "United States"),
            ("km", "សហរដ\u{17d2}ឋ\u{200b}អាមេរ\u{17b7}ក"),
            ("kn", "ಸಂಯುಕ\u{ccd}ತ ಸಂಸ\u{ccd}ಥಾನಗಳು"),
            ("ko", "미국"),
            ("ku", "Dewletên Yekbûyî"),
            ("kv", "United States"),
            ("kw", "United States"),
            ("ky", "АКШ"),
            ("lo", "United States"),
            ("lt", "Jungtinės Amerikos Valstijos"),
            ("lv", "Amerikas Savienotās Valstis"),
            ("mi", "Amerika"),
            ("mk", "Соединети држави"),
            ("ml", "ഐക\u{d4d}യന\u{d3e}ട\u{d41}കള\u{d4d}\u{200d}"),
            ("mn", "Америкын нэгдсэн улс"),
            ("mr", "य\u{941}नायट\u{947}ड स\u{94d}ट\u{947}टस\u{94d}"),
            ("ms", "Amerika Syarikat"),
            ("mt", "United States"),
            ("my", "United States"),
            ("na", "United States"),
            ("nb", "USA"),
            ("ne", "स\u{902}य\u{941}क\u{94d}त राज\u{94d}य"),
            ("nl", "Verenigde Staten"),
            ("nn", "USA"),
            ("nv", "United States"),
            ("oc", "Estats Units"),
            ("or", "ଯ\u{b41}କ\u{b4d}ତର\u{b3e}ଷ\u{b4d}ଟ\u{b4d}ର"),
            ("pa", "ਅਮਰੀਕਾ"),
            ("pi", "United States"),
            ("pl", "Stany Zjednoczone"),
            ("ps", "United States"),
            ("pt", "Estados Unidos"),
            ("pt_BR", "Estados Unidos"),
            ("ro", "Statele Unite"),
            ("ru", "Соединённые штаты"),
            ("rw", "Leta Zunze Ubumwe"),
            ("sc", "Istados Unidos"),
            ("sd", "United States"),
            ("si", "එක\u{dca}සත\u{dca} ජනපද"),
            ("sk", "Spojené štáty"),
            ("sl", "Združene države"),
            ("so", "Qaramada Midoobey ee Maraykanka"),
            ("sq", "Shtetet e Bashkuara"),
            ("sr", "Сједињене Државе"),
            ("sv", "USA"),
            ("sw", "United States"),
            ("ta", "ஐக\u{bcd}கிய அமெரிக\u{bcd}க\u{bbe}"),
            (
                "te",
                "యున\u{c48}ట\u{c46}డ\u{c4d} స\u{c4d}ట\u{c47}ట\u{c4d}స\u{c4d}",
            ),
            ("tg", "Иёлоти Муттаҳида"),
            ("th", "สหร\u{e31}ฐ"),
            ("ti", "ኣመሪካ"),
            ("tk", "Birleşen Ştatlar"),
            ("tl", "Estados Unidos"),
            ("tr", "Amerika Birleşik Devletleri"),
            ("tt", "Берләшкән Штатлар"),
            ("ug", "ئامېرىكا"),
            ("uk", "США"),
            ("ur", "United States"),
            ("uz", "United States"),
            ("ve", "United States"),
            ("vi", "Mỹ"),
            ("wa", "Estats Unis"),
            ("wo", "Aamerik"),
            ("xh", "United States"),
            ("yo", "United States"),
            ("zh_CN", "美国"),
            ("zh_HK", "美國"),
            ("zh_TW", "美國"),
            ("zu", "United States"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
