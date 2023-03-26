// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Jamaica

#[cfg(all(feature = "jm", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::JM;
    pub const ALPHA3: Alpha3 = Alpha3::JAM;
    pub const CONTINENT: Continent = Continent::NorthAmerica;
    pub const COUNTRY_CODE: usize = 1;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::JMD;
    pub const GEC: Option<GEC> = Some(GEC::JM);
    pub const INTERNATIONAL_PREFIX: &str = "011";
    pub const IOC: Option<IOC> = Some(IOC::JAM);
    pub const ISO_SHORT_NAME: &str = "Jamaica";
    pub const ISO_LONG_NAME: &str = "Jamaica";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[10];
    pub const NATIONAL_PREFIX: &str = "1";
    pub const NATIONALITY: Option<&str> = Some("Jamaican");
    pub const NUMBER: &str = "388";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Americas);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Caribbean);
    pub const UN_LOCODE: &str = "JM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Jamaica", "Jamaika", "Jamaïque", "ジャマイカ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::AMER;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Jamaica"),
        ("af", "Jamaika"),
        ("ak", "Jamaica"),
        ("am", "ጃሢ፤ጢ"),
        ("an", "Jamaica"),
        ("ar", "جامايكا"),
        ("as", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
        ("ay", "Jamaica"),
        ("az", "Yamayka"),
        ("ba", "Jamaica"),
        ("be", "Ямайка"),
        ("bg", "Ямайка"),
        ("bi", "Jamaica"),
        ("bn", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
        ("bn_IN", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
        ("br", "Jamaika"),
        ("bs", "Jamajka"),
        ("ca", "Jamaica"),
        ("ce", "Ямайка"),
        ("ch", "Jamaica"),
        ("cs", "Jamajka"),
        ("cv", "Ямайка"),
        ("cy", "Jamaica"),
        ("da", "Jamaica"),
        ("de", "Jamaika"),
        ("dv", "ޖ\u{7ac}މ\u{7ac}އ\u{7a8}ކ\u{7a7}"),
        ("dz", "ཇ་མའ\u{f72}་ཀ"),
        ("ee", "Jamaica"),
        ("el", "Τζαμάικα"),
        ("en", "Jamaica"),
        ("eo", "Jamajko"),
        ("es", "Jamaica"),
        ("et", "Jamaica"),
        ("eu", "Jamaika"),
        ("fa", "جاماییکا"),
        ("ff", "Jamaica"),
        ("fi", "Jamaika"),
        ("fo", "Jameika"),
        ("fr", "Jamaïque"),
        ("fy", "Jamaika"),
        ("ga", "Iamáice"),
        ("gl", "Xamaica"),
        ("gn", "Jamaica"),
        ("gu", "જમ\u{ac8}કા"),
        ("gv", "Yn Yamaicey"),
        ("ha", "Jamaica"),
        ("he", "ג'מייקה"),
        ("hi", "जम\u{948}का"),
        ("hr", "Jamajka"),
        ("ht", "Jamayik"),
        ("hu", "Jamaica"),
        ("hy", "Ջամայկա"),
        ("ia", "Jamaica"),
        ("id", "Jamaika"),
        ("io", "Jamaika"),
        ("is", "Jamaíka"),
        ("it", "Giamaica"),
        ("iu", "Jamaica"),
        ("ja", "ジャマイカ"),
        ("ka", "იამაიკა"),
        ("ki", "Jamaica"),
        ("kk", "Ямайка"),
        ("kl", "Jamaica"),
        ("km", "ចាម\u{17c9}ៃកា"),
        ("kn", "ಜಮೈಕ"),
        ("ko", "자메이카"),
        ("ku", "Jamayîka"),
        ("kv", "Ямайка"),
        ("kw", "Jamayka"),
        ("ky", "Ямайка"),
        ("lo", "Jamaica"),
        ("lt", "Jamaika"),
        ("lv", "Jamaika"),
        ("mi", "Jamaica"),
        ("mk", "Јамајка"),
        ("ml", "ജമൈക\u{d4d}ക"),
        ("mn", "Ямайк"),
        ("mr", "जम\u{948}का"),
        ("ms", "Jamaika"),
        ("mt", "Jamaica"),
        ("my", "ဂျမေကာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Djamaika"),
        ("nb", "Jamaica"),
        ("ne", "जम\u{948}का"),
        ("nl", "Jamaica"),
        ("nn", "Jamaica"),
        ("nv", "Jamaica"),
        ("oc", "Jamaica"),
        ("or", "ଜ\u{b3e}ମୈକ\u{b3e}"),
        ("pa", "ਜ\u{a48}ਮਾਈਕਾ"),
        ("pi", "जम\u{948}का"),
        ("pl", "Jamajka"),
        ("ps", "جمیکا"),
        ("pt", "Jamaica"),
        ("pt_BR", "Jamaica"),
        ("ro", "Jamaica"),
        ("ru", "Ямайка"),
        ("rw", "Jamayika"),
        ("sc", "Giamàica"),
        ("sd", "جميڪا"),
        ("si", "ජැමෙය\u{dd2}ක\u{dcf}ව"),
        ("sk", "Jamajka"),
        ("sl", "Jamajka"),
        ("so", "Jameyka"),
        ("sq", "Xhamajkë"),
        ("sr", "Јамајка"),
        ("sv", "Jamaica"),
        ("sw", "Jamaica"),
        ("ta", "ஜமைக\u{bcd}க\u{bbe}"),
        ("te", "జమ\u{c48}క\u{c3e}"),
        ("tg", "Ҷомайка"),
        ("th", "จาเมกา"),
        ("ti", "ጃማይካ"),
        ("tk", "Ýamaýka"),
        ("tl", "Jamaica"),
        ("tr", "Jamaika"),
        ("tt", "Жамайка"),
        ("ug", "يامايكا"),
        ("uk", "Ямайка"),
        ("ur", "جمیکا"),
        ("uz", "Yamayka"),
        ("ve", "Jamaica"),
        ("vi", "Gia-mê-ca"),
        ("wa", "Djamayike"),
        ("wo", "Jamayka"),
        ("xh", "Jamaica"),
        ("yo", "Jamáíkà"),
        ("zh_CN", "牙买加"),
        ("zh_HK", "牙買加"),
        ("zh_TW", "牙買加"),
        ("zu", "Jamaica"),
    ];
    #[cfg(all(feature = "jm", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 18.109581;
        pub const LONGITUDE: f64 = -77.297508;
        pub const MAX_LATITUDE: f64 = 18.5697821;
        pub const MAX_LONGITUDE: f64 = -76.1448669;
        pub const MIN_LATITUDE: f64 = 17.6688854;
        pub const MIN_LONGITUDE: f64 = -78.4073639;
        pub const NORTHEAST_LATITUDE: f64 = 18.5697821;
        pub const NORTHEAST_LONGITUDE: f64 = -76.1448669;
        pub const SOUTHWEST_LATITUDE: f64 = 17.6688854;
        pub const SOUTHWEST_LONGITUDE: f64 = -78.4073639;
    }
}
#[cfg(all(feature = "jm", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 18.109581,
            longitude: -77.297508,
            max_latitude: 18.5697821,
            max_longitude: -76.1448669,
            min_latitude: 17.6688854,
            min_longitude: -78.4073639,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 18.5697821,
                    longitude: -76.1448669,
                },
                southwest: CountryGeoBound {
                    latitude: 17.6688854,
                    longitude: -78.4073639,
                },
            },
        }
    }
}

#[cfg(all(feature = "jm", feature = "subdivisions"))]
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
                        name: "Kingston",
                        country_alpha2: Alpha2::JM,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.983333), longitude: Some(-76.8), max_latitude: Some(18.0724068), min_latitude: Some(17.9634658), max_longitude: Some(-76.7293739), min_longitude: Some(-76.8719387)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية كينغستون"), ("bn", "কিংস\u{9cd}টোন প\u{9be}রিশ"), ("ccp", "𑄇\u{11128}\u{11101}𑄌\u{11133}𑄑\u{11127}𑄚\u{11134}"), ("ceb", "Parish of Kingston"), ("da", "Kingston Parish"), ("de", "Kingston Parish"), ("el", "Κίνγκστον"), ("en", "Kingston"), ("es", "Parroquia de Kingston"), ("eu", "Kingston Parish"), ("fi", "Kingstonin kunta"), ("fr", "Paroisse de Kingston"), ("gu", "કિ\u{a82}ગ\u{acd}સ\u{acd}ટન પ\u{ac5}રિશ"), ("hi", "कि\u{902}ग\u{94d}स\u{94d}टन प\u{948}रिश"), ("id", "Paroki Kingston"), ("it", "Parrocchia di Kingston"), ("ja", "キングストン教区"), ("kn", "ಕ\u{cbf}ಂಗ\u{ccd}ಸ\u{ccd}ಟನ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "킹스턴 교구"), ("lt", "Kingstono parapija"), ("lv", "Kingstonas pagasts"), ("mr", "कि\u{902}ग\u{94d}सटोन प\u{945}रीश"), ("ms", "Kingston Parish"), ("nb", "Kingston prestegjeld"), ("nl", "Kingston"), ("no", "Kingston prestegjeld"), ("pl", "Kingston"), ("pt", "Kingston"), ("ru", "Приход Кингстон"), ("si", "ක\u{dd2}ංග\u{dca}ස\u{dca}ටන\u{dca} වසම"), ("sv", "Kingston Parish"), ("ta", "கிங\u{bcd}ஸ\u{bcd}டன\u{bcd} பரிஷ\u{bcd}"), ("te", "క\u{c3f}ంగ\u{c4d}\u{200c}స\u{c4d}టన\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ค\u{e34}งสต\u{e31}น เพร\u{e34}ช"), ("tr", "Kingston Parish"), ("uk", "Парафія Кінгстон"), ("ur", "کنگسٹن پیرش"), ("vi", "Giáo xứ Kingston"), ("zh", "金斯敦區")]),
                        unofficial_name_list: ["Kingston"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "Saint Andrew",
                        country_alpha2: Alpha2::JM,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0028906), longitude: Some(-76.8418246), max_latitude: Some(18.011791), min_latitude: Some(17.9982883), max_longitude: Some(-76.8372715), min_longitude: Some(-76.8447946)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبريشة سانت أندرو"), ("bn", "সেন\u{9cd}ট অ\u{9cd}য\u{9be}ন\u{9cd}ড\u{9cd}র\u{9c1}"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄃𑄬𑄚\u{11134}𑄓\u{11133}𑄢\u{1112a}"), ("da", "Saint Andrew"), ("de", "Saint Andrew"), ("el", "Άγιος Ανδρέας, Τζαμάικα"), ("en", "Saint Andrew"), ("es", "Parroquia de Saint Andrew (Jamaica)"), ("eu", "Saint Andrew (Jamaika)"), ("fi", "Saint Andrew"), ("fr", "Saint Andrew"), ("gu", "સ\u{ac7}ન\u{acd}ટ એન\u{acd}ડ\u{acd}ર\u{ac1}"), ("he", "סנט אנדרו"), ("hi", "स\u{947}\u{902}ट ए\u{902}ड\u{94d}र\u{942} प\u{948}रिश, जमाइका"), ("id", "Saint Andrew"), ("it", "Parrocchia di Saint Andrew"), ("ja", "セント・アンドリュー教区"), ("kn", "ಸೇಂಟ\u{ccd} ಆಂಡ\u{ccd}ರ\u{ccd}ಯ\u{cc2}"), ("ko", "세인트앤드루 교구"), ("lt", "Saint Endrius"), ("lv", "Senendrjū pagasts"), ("mr", "स\u{947}\u{902}ट ऍन\u{94d}ड\u{94d}र\u{94d}य\u{942}"), ("ms", "Saint Andrew"), ("nb", "Saint Andrew prestegjeld (Jamaica)"), ("nl", "Saint Andrew"), ("no", "Saint Andrew prestegjeld (Jamaica)"), ("pl", "Saint Andrew (Jamajka)"), ("pt", "Saint Andrew (Jamaica)"), ("ru", "Приход Сент-Андру"), ("si", "ශ\u{dcf}න\u{dca}ත ඇන\u{dca}ඩෘ"), ("sl", "Saint Andrew"), ("sv", "Saint Andrew Parish"), ("ta", "செயின\u{bcd}ட\u{bcd} ஆண\u{bcd}ட\u{bcd}ரு"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఆండ\u{c4d}ర\u{c4d}యూ ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซน แอนดร\u{e34}ว"), ("tr", "Saint Andrew"), ("uk", "Сент-Ендрю (Ямайка)"), ("ur", "سینٹ اینڈریو"), ("vi", "Saint Andrew"), ("zh", "聖安德魯區")]),
                        unofficial_name_list: ["Saint Andrew"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "Saint Thomas",
                        country_alpha2: Alpha2::JM,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.9700261), longitude: Some(-76.4331698), max_latitude: Some(18.087991), min_latitude: Some(17.8502263), max_longitude: Some(-76.183159), min_longitude: Some(-76.6640716)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت توماس"), ("bg", "Сейнт Томас"), ("bn", "সেন\u{9cd}ট থম\u{9be}স প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄗\u{1112e}𑄟𑄌\u{11134}"), ("ceb", "Parish of Saint Thomas"), ("da", "Saint Thomas Parish"), ("de", "Saint Thomas Parish"), ("el", "Άγιος Θωμάς"), ("en", "Saint Thomas"), ("es", "Parroquia de Saint Thomas"), ("eu", "Saint Thomas"), ("fi", "Saint Thomasin kunta"), ("fr", "Paroisse de Saint-Thomas"), ("gu", "સ\u{ac7}\u{a82}ટ થોમસ પ\u{ac5}રિશ"), ("he", "סנט תומאס"), ("hi", "स\u{947}\u{902}ट थॉमस प\u{948}रिश"), ("id", "Paroki Saint Thomas"), ("it", "Parrocchia di Saint Thomas"), ("ja", "セント・トーマス教区"), ("kn", "ಸೇಂಟ\u{ccd} ಥಾಮಸ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트토머스 교구"), ("lt", "Švento Tomaso parapija, Jamaika"), ("lv", "Sentomasa pagasts"), ("mr", "स\u{947}\u{902}ट थॉमस परश"), ("ms", "Saint Thomas Parish"), ("nb", "Saint Thomas prestegjeld"), ("nl", "Saint Thomas"), ("no", "Saint Thomas prestegjeld"), ("pl", "Saint Thomas"), ("pt", "Saint Thomas"), ("ru", "Сейнт Томас"), ("si", "ශ\u{dcf}න\u{dca}ත තෝමස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Parish of Saint Thomas"), ("ta", "செயின\u{bcd}ட\u{bcd} த\u{bbe}மஸ\u{bcd} ப\u{bbe}ரிஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} థ\u{c3e}మస\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c}โทม\u{e31}ส"), ("tr", "Saint Thomas Parish"), ("uk", "Парафія Сент-Томас"), ("ur", "سینٹ تھامس پیرش، جمیکا"), ("vi", "Giáo xứ Saint Thomas"), ("zh", "聖托馬斯區")]),
                        unofficial_name_list: ["Saint Thomas"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "Portland",
                        country_alpha2: Alpha2::JM,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0844274), longitude: Some(-76.4100267), max_latitude: Some(18.267005), min_latitude: Some(17.9826419), max_longitude: Some(-76.25551589999999), min_longitude: Some(-76.7609059)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية بورتلاند"), ("bn", "পোর\u{9cd}টল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Parish of Portland"), ("da", "Portland Parish"), ("de", "Portland Parish"), ("el", "Πόρτλαντ, Τζαμάικα"), ("en", "Portland"), ("es", "Parroquia de Portland"), ("eu", "Portland"), ("fi", "Portlandin kunta"), ("fr", "Paroisse de Portland"), ("gu", "પોર\u{acd}ટલ\u{ac7}ન\u{acd}ડ પ\u{ac5}રિશ"), ("hi", "पोर\u{94d}टल\u{948}\u{902}ड प\u{948}रिश"), ("hy", "Պորտլենդի համայնք"), ("id", "Paroki Portland"), ("it", "Parrocchia di Portland"), ("ja", "ポートランド教区"), ("kn", "ಪೋರ\u{ccd}ಟ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "포틀랜드 교구"), ("lt", "Portlando parapija"), ("lv", "Portlendas pagasts"), ("mr", "पोर\u{94d}टल\u{901}ड प\u{945}रीश"), ("ms", "Portland Parish"), ("nb", "Portland prestegjeld"), ("nl", "Portland"), ("no", "Portland prestegjeld"), ("pl", "Portland"), ("pt", "Portland"), ("ru", "Приход Портленд"), ("si", "පොර\u{dca}ට\u{dca}ලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Parish of Portland"), ("ta", "போர\u{bcd}ட\u{bcd}லேண\u{bcd}ட\u{bcd} பரிஷ\u{bcd}"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "พอร\u{e4c}ตแลนด\u{e4c}"), ("tr", "Portland Parish"), ("uk", "Парафія Портленд"), ("ur", "پورٹلینڈ پیرش"), ("vi", "Giáo xứ Portland"), ("zh", "波特蘭區")]),
                        unofficial_name_list: ["Portland"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "Saint Mary",
                        country_alpha2: Alpha2::JM,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.3092711), longitude: Some(-76.964306), max_latitude: Some(18.4221861), min_latitude: Some(18.1388119), max_longitude: Some(-76.701178), min_longitude: Some(-77.071198)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت ماري"), ("bn", "সেন\u{9cd}ট মেরি প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄟𑄬𑄢\u{11128}"), ("ceb", "Parish of Saint Mary (parokya sa Jamaica)"), ("da", "Saint Mary Parish"), ("de", "Saint Mary Parish"), ("el", "Αγία Μαρία, Τζαμάικα"), ("en", "Saint Mary"), ("es", "Parroquia de Saint Mary"), ("eu", "Saint Mary"), ("fi", "Saint Maryin pitäjä"), ("fr", "paroisse de Saint Mary"), ("gu", "સ\u{ac7}ન\u{acd}ટ મ\u{ac7}રી પ\u{ac5}રિશ"), ("hi", "स\u{947}\u{902}ट म\u{948}री प\u{948}रिश"), ("id", "Paroki Saint Mary"), ("it", "Parrocchia di Saint Mary"), ("ja", "セント・メアリー教区"), ("kn", "ಸೇಂಟ\u{ccd} ಮೇರ\u{cbf} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트메리 교구"), ("lt", "Šventosios Marijos Parapija"), ("lv", "Sentmerī pagasts"), ("mr", "स\u{947}\u{902}ट म\u{947}री प\u{945}रीश"), ("ms", "Saint Mary Parish"), ("nb", "Saint Mary prestegjeld"), ("nl", "Saint Mary"), ("no", "Saint Mary prestegjeld"), ("pl", "Saint Mary"), ("pt", "Saint Mary"), ("ru", "Сент-Мэри"), ("si", "ශ\u{dcf}න\u{dca}ත මේර\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "okrožje Saint Mary"), ("sv", "Parish of Saint Mary (parish i Jamaica)"), ("ta", "செயின\u{bcd}ட\u{bcd} மேரி ப\u{bbe}ரிஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} మ\u{c3e}\u{c47}ర\u{c40} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c}แมร\u{e35}\u{e48}"), ("tr", "Saint Mary Parish"), ("uk", "Парафія Сент-Мері"), ("ur", "سینٹ میری پیرش، جمیکا"), ("vi", "Giáo xứ Saint Mary"), ("zh", "聖瑪麗區")]),
                        unofficial_name_list: ["Saint Mary"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "Saint Ann",
                        country_alpha2: Alpha2::JM,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.3281428), longitude: Some(-77.2405153), max_latitude: Some(18.4788759), min_latitude: Some(18.179688), max_longitude: Some(-77.00158379999999), min_longitude: Some(-77.4926529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية القديس آن"), ("be", "Прыход Сент-Эн"), ("bn", "সেন\u{9cd}ট অ\u{9cd}য\u{9be}ন প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄃𑄬𑄚\u{11134}"), ("ceb", "Parish of Saint Ann"), ("da", "Saint Ann Parish"), ("de", "Saint Ann Parish"), ("el", "Αγία Άννα, Τζαμάικα"), ("en", "Saint Ann"), ("es", "Parroquia de Saint Ann"), ("et", "Saint Anni vald"), ("eu", "Saint Ann"), ("fi", "Saint Ann"), ("fr", "Paroisse de Saint Ann"), ("gu", "સ\u{ac7}ન\u{acd}ટ એન પ\u{ac5}રિશ"), ("he", "סנט אן"), ("hi", "स\u{947}\u{902}ट ऐन प\u{948}रिश"), ("id", "Paroki Saint Ann"), ("it", "Parrocchia di Saint Ann"), ("ja", "セント・アン教区"), ("kn", "ಸೇಂಟ\u{ccd} ಆನ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트앤 교구"), ("lt", "Šventosios Anos parapija"), ("lv", "Sentenas pagasts"), ("mr", "स\u{947}\u{902}ट एन प\u{945}रिश"), ("ms", "Saint Ann Parish"), ("nb", "Saint Ann prestegjeld"), ("nl", "Saint Ann"), ("no", "Saint Ann prestegjeld"), ("pl", "Saint Ann"), ("pt", "Saint Ann"), ("ru", "приход Сент-Энн"), ("si", "ශ\u{dcf}න\u{dca}ත ආන\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Parish of Saint Ann"), ("ta", "செயின\u{bcd}ட\u{bcd} ஆன\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఆన\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเซนต\u{e4c}แอน"), ("tr", "Saint Ann Parish"), ("uk", "Парафія Сент-Ан"), ("ur", "سینٹ این پیرش"), ("vi", "Giáo xứ Saint Ann"), ("zh", "聖安娜區")]),
                        unofficial_name_list: ["Saint Ann"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "Trelawny",
                        country_alpha2: Alpha2::JM,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.3526143), longitude: Some(-77.6077865), max_latitude: Some(18.511424), min_latitude: Some(18.2023229), max_longitude: Some(-77.44148299999999), min_longitude: Some(-77.777457)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية تريلاوني"), ("bn", "ত\u{9cd}রেল\u{9be}ওনি প\u{9be}রিশ"), ("ca", "parròquia de Trelawny"), ("ccp", "𑄑\u{11133}𑄢𑄬𑄣\u{11127}𑄚\u{11128}"), ("ceb", "Parish of Trelawny"), ("cs", "Trelawny Parish"), ("cy", "Trelawny"), ("da", "Trelawny Parish"), ("de", "Trelawny Parish"), ("el", "Τρελόνι"), ("en", "Trelawny"), ("es", "Parroquia de Trelawny"), ("et", "Trelawny"), ("eu", "Trelawny"), ("fi", "Trelawnyin kunta"), ("fr", "Paroisse de Trelawny"), ("gu", "ટ\u{acd}ર\u{ac7}લ\u{ac7}વ\u{acd}ની પ\u{ac5}રિશ"), ("he", "מחוז טרלוני"), ("hi", "ट\u{94d}रिलॉनी प\u{948}रिश"), ("hy", "Թրելոնի եկեղեցական ծուխ"), ("id", "Paroki Trelawny"), ("it", "Trelawny"), ("ja", "トレローニー教区"), ("kn", "ಟ\u{ccd}ರ\u{cc6}ಲೋನ\u{cbf} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "트렐로니 교구"), ("lt", "Treloni parapija"), ("lv", "Triloni pagasts"), ("mr", "ट\u{94d}रिलॉनी"), ("ms", "Trelawny Parish"), ("nb", "Trelawny prestegjeld"), ("nl", "Trelawny"), ("no", "Trelawny prestegjeld"), ("pl", "Trelawny"), ("pt", "Trelawny"), ("ru", "Трелони"), ("si", "ට\u{dca}\u{200d}රේලව\u{dca}න\u{dca}ය\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Trelawny"), ("sl", "okrožje Trelawny"), ("sv", "Trelawny"), ("ta", "ட\u{bcd}ரெல\u{bbe}வ\u{bcd}னி பரிஷ\u{bcd}"), ("te", "ట\u{c4d}ర\u{c46}ల\u{c3e}న\u{c40} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเทรลอว\u{e4c}น\u{e35}"), ("tr", "Trelawny Parish"), ("uk", "Парафія Трелоні"), ("ur", "ٹریلاونی پیرش"), ("vi", "Giáo xứ Trelawny"), ("zh", "特里洛尼區")]),
                        unofficial_name_list: ["Trelawny"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "Saint James",
                        country_alpha2: Alpha2::JM,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.3923184), longitude: Some(-77.85959629999999), max_latitude: Some(18.5253104), min_latitude: Some(18.2089849), max_longitude: Some(-77.737876), min_longitude: Some(-78.00069189999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت جيمس، جامايكا"), ("bn", "সেইন\u{9cd}ট জেমস প\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄇𑄬𑄟\u{11134}𑄌\u{11134}"), ("ceb", "Parish of Saint James"), ("cs", "Saint James Parish"), ("da", "Saint James Parish"), ("de", "Saint James Parish"), ("el", "Άγιος Ιάκωβος"), ("en", "Saint James"), ("es", "Parroquia de Saint James"), ("eu", "Saint James"), ("fi", "Saint Jamesin pitäjä"), ("fr", "Paroisse de Saint James"), ("gu", "સ\u{ac7}\u{a82}ટ જ\u{ac7}મ\u{acd}સ પ\u{ac5}રિશ"), ("he", "מחוז סיינט ג׳יימס"), ("hi", "स\u{947}\u{902}ट ज\u{947}म\u{94d}स प\u{947}रिश, जमाइका"), ("id", "Paroki Saint James"), ("it", "Parrocchia di Saint James"), ("ja", "セント・ジェームズ教区"), ("kn", "ಸೇಂಟ\u{ccd} ಜೇಮ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트제임스 교구"), ("lt", "Saint Džeimso parapija"), ("lv", "Sendžeimsa pagasts"), ("mr", "स\u{947}\u{902}ट ज\u{947}म\u{94d}स प\u{945}रीश"), ("ms", "Saint James Parish"), ("nb", "Saint James prestegjeld"), ("nl", "Saint James"), ("no", "Saint James prestegjeld"), ("pl", "Saint James"), ("pt", "Saint James"), ("ru", "Сент-Джеймс"), ("si", "ශ\u{dcf}න\u{dca}ත ජේම\u{dca}ස\u{dca} කෝරලය"), ("sv", "Parish of Saint James"), ("ta", "செயின\u{bcd}ட\u{bcd} ஜேம\u{bcd}ஸ\u{bcd} ப\u{bbe}ரிஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} జ\u{c47}మ\u{c4d}స\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเซนต\u{e4c}เจมส\u{e4c}"), ("tr", "Saint James Parish"), ("uk", "Парафія Сент-Джеймс"), ("ur", "سینٹ جیمز پیرش، جمیکا"), ("vi", "Giáo xứ Saint James"), ("zh", "聖詹姆斯區")]),
                        unofficial_name_list: ["Saint James"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "Hanover",
                        country_alpha2: Alpha2::JM,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.4097707), longitude: Some(-78.13363799999999), max_latitude: Some(18.4628808), min_latitude: Some(18.3051631), max_longitude: Some(-77.913105), min_longitude: Some(-78.34310889999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية هانوفر"), ("bg", "Хановър"), ("bn", "হ\u{9cd}য\u{9be}নওভ\u{9be}র প\u{9cd}য\u{9be}রিশ"), ("ca", "Parròquia de Hanover"), ("ccp", "𑄦𑄚\u{1112e}𑄞𑄢\u{11134}"), ("ceb", "Parish of Hanover"), ("da", "Hanover Parish"), ("de", "Hanover Parish"), ("el", "Αννόβερο"), ("en", "Hanover"), ("es", "Parroquia de Hanover"), ("et", "Hanoveri vald"), ("eu", "Hanover"), ("fi", "Hanoverin kunta"), ("fr", "Paroisse de Hanover"), ("gu", "હ\u{ac7}નોવર પ\u{ac5}રિશ"), ("he", "הנובר"), ("hi", "हनोवर प\u{948}रिश"), ("id", "Paroki Hanover"), ("it", "Parrocchia di Hanover"), ("ja", "ハノーバー教区"), ("kn", "ಹ\u{ccd}ಯಾನೋವರ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "해노버 교구"), ("lt", "Hanoverio parapija"), ("lv", "Henoveras pagasts"), ("mr", "ह\u{945}नोव\u{94d}हर प\u{945}रीश"), ("ms", "Hanover Parish"), ("nb", "Hanover prestegjeld"), ("nl", "Hanover"), ("no", "Hanover prestegjeld"), ("pl", "Hanover"), ("pt", "Hanover"), ("ru", "Приход Ганновер"), ("si", "හැනෝවර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "okrožje Hanover"), ("sv", "Hanover"), ("ta", "ஹனோவர\u{bcd} ப\u{bbe}ரிஸ\u{bcd}"), ("te", "హన\u{c4b}వర\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ฮ\u{e31}นโนเฟอร\u{e4c}"), ("tr", "Hanover Parish"), ("uk", "Гановер (Ямайка)"), ("ur", "ہینور پیرش"), ("vi", "Giáo xứ Hanover"), ("zh", "汉诺威区")]),
                        unofficial_name_list: ["Hanover"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "Westmoreland",
                        country_alpha2: Alpha2::JM,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.2944378), longitude: Some(-78.1564432), max_latitude: Some(18.360146), min_latitude: Some(18.0689012), max_longitude: Some(-77.87850809999999), min_longitude: Some(-78.366638)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية ويستمورلاند"), ("bn", "ওয\u{9bc}েস\u{9cd}ট\u{9be}রমল\u{9cd}য\u{9be}ন\u{9cd}ডল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄌\u{11134}𑄟\u{1112e}𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "Parish of Westmoreland"), ("da", "Westmoreland Parish"), ("de", "Westmoreland Parish"), ("el", "Γουεστμόρλαντ"), ("en", "Westmoreland"), ("es", "Parroquia de Westmoreland"), ("eu", "Westmoreland"), ("fi", "Westmorelandin pitäjä"), ("fr", "Paroisse de Westmoreland"), ("gu", "વ\u{ac7}સ\u{acd}ટમોરલ\u{ac7}ન\u{acd}ડ પ\u{ac5}રિશ"), ("hi", "व\u{947}स\u{94d}टमोरल\u{948}\u{902}ड प\u{948}रिश"), ("id", "Paroki Westmoreland"), ("it", "Parrocchia di Westmoreland"), ("ja", "ウェストモアランド教区"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}ಮೋರ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "웨스트모얼랜드 교구"), ("lt", "Vestmorelando parapija"), ("lv", "Vestmorlendas pagasts"), ("mr", "व\u{947}स\u{94d}टमोरल\u{901}ड प\u{945}रीश"), ("ms", "Westmoreland Parish"), ("nb", "Westmoreland prestegjeld"), ("nl", "Westmoreland"), ("no", "Westmoreland prestegjeld"), ("pl", "Westmoreland"), ("pt", "Westmoreland"), ("ru", "Приход Вестморленд"), ("si", "වෙස\u{dca}ටෝම\u{dca}රෙලන\u{dca}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Parish of Westmoreland"), ("ta", "வெஸ\u{bcd}ட\u{bcd}மோர\u{bcd}ல\u{bbe}ண\u{bcd}ட\u{bcd} பரிஷ\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d}\u{200c}మ\u{c4b}ర\u{c4d}\u{200c}ల\u{c3e}ండ\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เวสต\u{e4c}มอร\u{e4c}แลนด\u{e4c}"), ("tr", "Westmoreland Parish"), ("uk", "Парафія Вестморленд"), ("ur", "ویسٹمورلینڈ پیرش"), ("vi", "Giáo xứ Westmoreland"), ("zh", "西摩蘭區")]),
                        unofficial_name_list: ["Westmoreland"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "Saint Elizabeth",
                        country_alpha2: Alpha2::JM,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0788461), longitude: Some(-77.69941969999999), max_latitude: Some(18.2497811), min_latitude: Some(17.8543891), max_longitude: Some(-77.56239699999999), min_longitude: Some(-77.956349)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت إليزابيث"), ("bn", "সেন\u{9cd}ট এলিজ\u{9be}বেথ প\u{9cd}রদেশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄃𑄬𑄣\u{11128}𑄎𑄝𑄬𑄖\u{11134}"), ("ceb", "Parish of Saint Elizabeth"), ("da", "Saint Elizabeth Parish"), ("de", "Saint Elizabeth Parish"), ("el", "Αγία Ελίζαμπεθ"), ("en", "Saint Elizabeth"), ("es", "Parroquia de Saint Elizabeth"), ("eu", "Saint Elizabeth"), ("fa", "سنت الیزابت، جامائیکا"), ("fi", "Saint Elizabethin kunta"), ("fr", "Paroisse de Saint Elizabeth"), ("gu", "સ\u{ac7}ન\u{acd}ટ એલિઝાબ\u{ac7}થ પ\u{ac5}રિશ"), ("he", "מחוז סנט אליזבת"), ("hi", "स\u{947}\u{902}ट एलिज\u{93c}ाब\u{947}थ प\u{948}रिश"), ("id", "Paroki Saint Elizabeth"), ("it", "Parrocchia di Saint Elizabeth"), ("ja", "セント・エリザベス教区"), ("kn", "ಸೇಂಟ\u{ccd} ಎಲ\u{cbf}ಜಬ\u{cc6}ತ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트엘리자베스 교구"), ("lt", "Šventosios Elžbietos parapija"), ("lv", "Senelizabetes pagasts"), ("mr", "स\u{947}\u{902}ट एलिझाब\u{947}थ प\u{945}रीश"), ("ms", "Saint Elizabeth Parish"), ("nb", "Saint Elizabeth prestegjeld"), ("nl", "Saint Elizabeth"), ("no", "Saint Elizabeth prestegjeld"), ("pl", "Saint Elizabeth"), ("pt", "Saint Elizabeth"), ("ru", "приход Сент-Элизабет"), ("si", "ශ\u{dcf}න\u{dca}ත එල\u{dd2}සබත\u{dca} කෝරලය"), ("sv", "Parish of Saint Elizabeth"), ("ta", "செயின\u{bcd}ட\u{bcd} எலிசபெத\u{bcd} ப\u{bbe}ரிஸ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} ఎల\u{c3f}జబ\u{c46}త\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลเซนต\u{e4c}เอล\u{e34}ซเบธ"), ("tr", "Saint Elizabeth Parish"), ("uk", "Парафія Сент-Елізабет"), ("ur", "سینٹ الزبتھ پیرش"), ("vi", "Giáo xứ Saint Elizabeth"), ("zh", "聖伊麗莎白區")]),
                        unofficial_name_list: ["Saint Elizabeth"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "Manchester",
                        country_alpha2: Alpha2::JM,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0669654), longitude: Some(-77.5160788), max_latitude: Some(18.2492469), min_latitude: Some(17.8387709), max_longitude: Some(-77.34482899999999), min_longitude: Some(-77.637006)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية مانشستر"), ("bn", "ম\u{9cd}য\u{9be}নচেস\u{9cd}ট\u{9be}র প\u{9be}রিস"), ("ccp", "𑄟\u{11133}𑄠𑄚\u{11134}𑄌𑄬𑄌\u{11134}𑄑𑄢\u{11134}"), ("ceb", "Parish of Manchester"), ("da", "Manchester Parish"), ("de", "Manchester Parish"), ("el", "Μάντσεστερ, Τζαμάικα"), ("en", "Manchester"), ("es", "Parroquia de Manchester"), ("eu", "Manchester"), ("fi", "Manchesterin kunta"), ("fr", "Paroisse de Manchester"), ("gu", "માન\u{acd}ચ\u{ac7}સ\u{acd}ટર પ\u{ac5}રિશ"), ("he", "מחוז מנצ׳סטר"), ("hi", "म\u{947}नच\u{947}स\u{94d}टर प\u{948}रिश"), ("id", "Paroki Manchester"), ("it", "Parrocchia di Manchester"), ("ja", "マンチェスター教区"), ("kn", "ಮ\u{ccd}ಯಾಂಚ\u{cc6}ಸ\u{ccd}ಟರ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "맨체스터 교구"), ("lt", "Mančesterio parapija"), ("lv", "Mančestras pagasts"), ("mr", "म\u{901}च\u{947}स\u{94d}टर परग"), ("ms", "Manchester Parish"), ("nb", "Manchester prestegjeld"), ("nl", "Manchester"), ("no", "Manchester prestegjeld"), ("pl", "Manchester"), ("pt", "Manchester (Middlesex)"), ("ru", "Приход Манчестер"), ("si", "මැන\u{dca}චෙස\u{dca}ටර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "Manchester, Jamajka"), ("sv", "Parish of Manchester"), ("ta", "ம\u{bbe}ன\u{bcd}செஸ\u{bcd}டர\u{bcd} ப\u{bbe}ரிஸ\u{bcd}"), ("te", "మ\u{c3e}ంచ\u{c46}స\u{c4d}టర\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "ตำบลแมนเชสเตอร\u{e4c}"), ("tr", "Manchester Parish"), ("uk", "Манчестер (Ямайка)"), ("ur", "مانچسٹر پیرش"), ("vi", "Giáo xứ Manchester"), ("zh", "曼徹斯特區")]),
                        unofficial_name_list: ["Manchester"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "Clarendon",
                        country_alpha2: Alpha2::JM,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(17.9557183), longitude: Some(-77.2405153), max_latitude: Some(18.2174819), min_latitude: Some(17.7057243), max_longitude: Some(-77.1253108), min_longitude: Some(-77.4926529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية كلاريندون، جمايكا"), ("bn", "ক\u{9cd}ল\u{9be}রেন\u{9cd}ডন প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄇\u{11133}𑄣𑄢𑄬𑄚\u{11134}𑄓\u{11127}𑄚\u{11134}"), ("ceb", "Parish of Clarendon"), ("da", "Clarendon Parish"), ("de", "Clarendon Parish"), ("el", "Κλάρεντον"), ("en", "Clarendon"), ("es", "Parroquia de Clarendon"), ("eu", "Clarendon"), ("fa", "کلارندون پریش، جامائیکا"), ("fi", "Clarendon kunta"), ("fr", "Paroisse de Clarendon"), ("gu", "ક\u{acd}લ\u{ac7}ર\u{ac7}ન\u{acd}ડન પ\u{ac5}રિશ"), ("hi", "क\u{94d}ल\u{948}र\u{947}\u{902}डोन प\u{948}रिश, जमाइका"), ("id", "Paroki Clarendon"), ("it", "Parrocchia di Clarendon"), ("ja", "クラレンドン教区"), ("kn", "ಕ\u{ccd}ಲಾರ\u{cc6}ಂಡನ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "클래런던 교구"), ("lt", "Klarendono parapija"), ("lv", "Klarendonas pagasts"), ("mr", "क\u{94d}ल\u{947}र\u{947}डन परिश"), ("ms", "Clarendon Parish"), ("nb", "Clarendon prestegjeld"), ("nl", "Clarendon"), ("no", "Clarendon prestegjeld"), ("pl", "Clarendon (region Jamajki)"), ("pt", "Clarendon"), ("ru", "Кларендон (приход)"), ("si", "ක\u{dca}ලරෙන\u{dca}ඩන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sl", "Clarendon"), ("sv", "Clarendon, Jamaica"), ("ta", "கிளர\u{bcd}ந\u{bcd}தோன\u{bcd} பரிஷ\u{bcd}"), ("te", "క\u{c4d}ల\u{c3e}ర\u{c46}ండన\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "คลาเรนดอน"), ("tr", "Clarendon Parish"), ("uk", "Парафія Кларендон"), ("ur", "کلاریندون پارش"), ("vi", "Giáo xứ Clarendon"), ("zh", "克拉倫登區")]),
                        unofficial_name_list: ["Clarendon"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "Saint Catherine",
                        country_alpha2: Alpha2::JM,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(18.0364134), longitude: Some(-77.0564464), max_latitude: Some(18.258774), min_latitude: Some(17.8379222), max_longitude: Some(-76.848185), min_longitude: Some(-77.21647999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Parish,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أبرشية سانت كاثرين"), ("bn", "সেন\u{9cd}ট ক\u{9cd}য\u{9be}থেরিন প\u{9cd}য\u{9be}রিশ"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11134} 𑄇\u{11133}𑄠𑄗\u{11127}𑄢\u{1112d}𑄚\u{11134}"), ("ceb", "Parish of Saint Catherine"), ("da", "Saint Catherine Parish"), ("de", "Saint Catherine Parish"), ("el", "Αγία Αικατερίνη"), ("en", "Saint Catherine"), ("es", "Parroquia de Saint Catherine"), ("eu", "Saint Catherine"), ("fi", "Saint Catherine Parish"), ("fr", "Paroisse de Sainte-Catherine"), ("gu", "સ\u{ac7}ન\u{acd}ટ ક\u{ac7}થરિન પ\u{ac5}રિશ"), ("he", "סנט קתרין"), ("hi", "स\u{947}\u{902}ट क\u{948}थरीन प\u{948}रिश"), ("id", "Paroki Saint Catherine"), ("it", "Parrocchia di Saint Catherine"), ("ja", "セント・キャサリン教区"), ("kn", "ಸೇಂಟ\u{ccd} ಕ\u{ccd}ಯಾಥರೀನ\u{ccd} ಪ\u{ccd}ಯಾರ\u{cbf}ಷ\u{ccd}"), ("ko", "세인트캐서린 교구"), ("lt", "Šventosios Katerinos parapija"), ("lv", "Sentketerinas pagasts"), ("mr", "स\u{947}\u{902}ट क\u{945}थरीन प\u{945}रीश"), ("ms", "Saint Catherine Parish"), ("nb", "Saint Catherine prestegjeld"), ("nl", "Saint Catherine"), ("no", "Saint Catherine prestegjeld"), ("pl", "Saint Catherine"), ("pt", "Saint Catherine"), ("ru", "Сэнт-Кэтрин"), ("si", "ශ\u{dcf}න\u{dca}ත කැතර\u{dd3}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sv", "Parish of Saint Catherine"), ("ta", "செயின\u{bcd}ட\u{bcd} கேத\u{bcd}தரின\u{bcd} பரிஷ\u{bcd}"), ("te", "స\u{c46}య\u{c3f}ంట\u{c4d} క\u{c47}థర\u{c40}న\u{c4d} ప\u{c3e}ర\u{c3f}ష\u{c4d}"), ("th", "เซนต\u{e4c} แคเธอร\u{e35}น"), ("tr", "Saint Catherine District"), ("uk", "Сент-Кетрін"), ("ur", "سینٹ کیتھرین پیرش"), ("vi", "Giáo xứ Saint Catherine"), ("zh", "聖凱瑟琳區")]),
                        unofficial_name_list: ["Saint Catherine"].to_vec(),
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
#[cfg(feature = "jm")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::JM,
        alpha3: Alpha3::JAM,
        address_format: None,
        continent: Continent::NorthAmerica,
        country_code: 1,
        currency_code: CurrencyCode::JMD,
        gec: Some(GEC::JM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "011",
        ioc: Some(IOC::JAM),
        iso_long_name: "Jamaica",
        iso_short_name: "Jamaica",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [3].to_vec(),
        national_number_length_list: [10].to_vec(),
        national_prefix: "1",
        nationality: Some("Jamaican"),
        number: "388",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Americas),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Caribbean),
        un_locode: "JM",
        unofficial_name_list: ["Jamaica", "Jamaika", "Jamaïque", "ジャマイカ"].to_vec(),
        world_region: WorldRegion::AMER,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Jamaica"),
            ("af", "Jamaika"),
            ("ak", "Jamaica"),
            ("am", "ጃሢ፤ጢ"),
            ("an", "Jamaica"),
            ("ar", "جامايكا"),
            ("as", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
            ("ay", "Jamaica"),
            ("az", "Yamayka"),
            ("ba", "Jamaica"),
            ("be", "Ямайка"),
            ("bg", "Ямайка"),
            ("bi", "Jamaica"),
            ("bn", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
            ("bn_IN", "জ\u{9be}ম\u{9be}ইক\u{9be}"),
            ("br", "Jamaika"),
            ("bs", "Jamajka"),
            ("ca", "Jamaica"),
            ("ce", "Ямайка"),
            ("ch", "Jamaica"),
            ("cs", "Jamajka"),
            ("cv", "Ямайка"),
            ("cy", "Jamaica"),
            ("da", "Jamaica"),
            ("de", "Jamaika"),
            ("dv", "ޖ\u{7ac}މ\u{7ac}އ\u{7a8}ކ\u{7a7}"),
            ("dz", "ཇ་མའ\u{f72}་ཀ"),
            ("ee", "Jamaica"),
            ("el", "Τζαμάικα"),
            ("en", "Jamaica"),
            ("eo", "Jamajko"),
            ("es", "Jamaica"),
            ("et", "Jamaica"),
            ("eu", "Jamaika"),
            ("fa", "جاماییکا"),
            ("ff", "Jamaica"),
            ("fi", "Jamaika"),
            ("fo", "Jameika"),
            ("fr", "Jamaïque"),
            ("fy", "Jamaika"),
            ("ga", "Iamáice"),
            ("gl", "Xamaica"),
            ("gn", "Jamaica"),
            ("gu", "જમ\u{ac8}કા"),
            ("gv", "Yn Yamaicey"),
            ("ha", "Jamaica"),
            ("he", "ג'מייקה"),
            ("hi", "जम\u{948}का"),
            ("hr", "Jamajka"),
            ("ht", "Jamayik"),
            ("hu", "Jamaica"),
            ("hy", "Ջամայկա"),
            ("ia", "Jamaica"),
            ("id", "Jamaika"),
            ("io", "Jamaika"),
            ("is", "Jamaíka"),
            ("it", "Giamaica"),
            ("iu", "Jamaica"),
            ("ja", "ジャマイカ"),
            ("ka", "იამაიკა"),
            ("ki", "Jamaica"),
            ("kk", "Ямайка"),
            ("kl", "Jamaica"),
            ("km", "ចាម\u{17c9}ៃកា"),
            ("kn", "ಜಮೈಕ"),
            ("ko", "자메이카"),
            ("ku", "Jamayîka"),
            ("kv", "Ямайка"),
            ("kw", "Jamayka"),
            ("ky", "Ямайка"),
            ("lo", "Jamaica"),
            ("lt", "Jamaika"),
            ("lv", "Jamaika"),
            ("mi", "Jamaica"),
            ("mk", "Јамајка"),
            ("ml", "ജമൈക\u{d4d}ക"),
            ("mn", "Ямайк"),
            ("mr", "जम\u{948}का"),
            ("ms", "Jamaika"),
            ("mt", "Jamaica"),
            ("my", "ဂျမေကာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Djamaika"),
            ("nb", "Jamaica"),
            ("ne", "जम\u{948}का"),
            ("nl", "Jamaica"),
            ("nn", "Jamaica"),
            ("nv", "Jamaica"),
            ("oc", "Jamaica"),
            ("or", "ଜ\u{b3e}ମୈକ\u{b3e}"),
            ("pa", "ਜ\u{a48}ਮਾਈਕਾ"),
            ("pi", "जम\u{948}का"),
            ("pl", "Jamajka"),
            ("ps", "جمیکا"),
            ("pt", "Jamaica"),
            ("pt_BR", "Jamaica"),
            ("ro", "Jamaica"),
            ("ru", "Ямайка"),
            ("rw", "Jamayika"),
            ("sc", "Giamàica"),
            ("sd", "جميڪا"),
            ("si", "ජැමෙය\u{dd2}ක\u{dcf}ව"),
            ("sk", "Jamajka"),
            ("sl", "Jamajka"),
            ("so", "Jameyka"),
            ("sq", "Xhamajkë"),
            ("sr", "Јамајка"),
            ("sv", "Jamaica"),
            ("sw", "Jamaica"),
            ("ta", "ஜமைக\u{bcd}க\u{bbe}"),
            ("te", "జమ\u{c48}క\u{c3e}"),
            ("tg", "Ҷомайка"),
            ("th", "จาเมกา"),
            ("ti", "ጃማይካ"),
            ("tk", "Ýamaýka"),
            ("tl", "Jamaica"),
            ("tr", "Jamaika"),
            ("tt", "Жамайка"),
            ("ug", "يامايكا"),
            ("uk", "Ямайка"),
            ("ur", "جمیکا"),
            ("uz", "Yamayka"),
            ("ve", "Jamaica"),
            ("vi", "Gia-mê-ca"),
            ("wa", "Djamayike"),
            ("wo", "Jamayka"),
            ("xh", "Jamaica"),
            ("yo", "Jamáíkà"),
            ("zh_CN", "牙买加"),
            ("zh_HK", "牙買加"),
            ("zh_TW", "牙買加"),
            ("zu", "Jamaica"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
