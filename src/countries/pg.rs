// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Independent State of Papua New Guinea

#[cfg(all(feature = "pg", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::PG;
    pub const ALPHA3: Alpha3 = Alpha3::PNG;
    pub const CONTINENT: Continent = Continent::Australia;
    pub const COUNTRY_CODE: usize = 675;
    pub const CURRENCY_CODE: &str = "PGK";
    pub const GEC: Option<GEC> = Some(GEC::PP);
    pub const INTERNATIONAL_PREFIX: &str = "05";
    pub const IOC: Option<IOC> = Some(IOC::PNG);
    pub const ISO_SHORT_NAME: &str = "Papua New Guinea";
    pub const ISO_LONG_NAME: &str = "The Independent State of Papua New Guinea";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Papua New Guinean");
    pub const NUMBER: &str = "598";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Oceania);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::Melanesia);
    pub const UN_LOCODE: &str = "PG";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Papua New Guinea",
        "Papua-Neuguinea",
        "Papouasie Nouvelle-Guinée",
        "Papúa Nueva Guinea",
        "パプアニューギニア",
        "Papoea-Nieuw-Guinea",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Papua New Guinea"),
        ("af", "Papoea-Nieu-Guinee"),
        ("ak", "Papua New Guinea"),
        ("am", "ፓፑፂ ኒፄ \u{1311}ኒ"),
        ("an", "Papua New Guinea"),
        ("ar", "بابوا غينيا الجديدة"),
        ("as", "প\u{9be}প\u{9c1}ৱ\u{9be} নিউ গিনি"),
        ("ay", "Papua New Guinea"),
        ("az", "Papua Yeni Qvineya"),
        ("ba", "Papua New Guinea"),
        ("be", "Папуа — Новая Гвінея"),
        ("bg", "Папуа Нова Гвинея"),
        ("bi", "Papua New Guinea"),
        ("bn", "প\u{9be}প\u{9c1}য়\u{9be} নিউ গিনি"),
        ("bn_IN", "প\u{9be}প\u{9c1}য়\u{9be} নিউ গিনি"),
        ("br", "Papoua Ginea-Nevez"),
        ("bs", "Papua Nova Gvineja"),
        ("ca", "Papua Nova Guinea"),
        ("ce", "Папуа — Керла Гвине"),
        ("ch", "Papua New Guinea"),
        ("cs", "Papua Nová Guinea"),
        ("cv", "Папуа — Керла Гвине"),
        ("cy", "Papua Guinea Newydd"),
        ("da", "Papua Ny Guinea"),
        ("de", "Papua-Neuguinea"),
        (
            "dv",
            "ޕ\u{7a6}ޕ\u{7aa}އ\u{7a7} ނ\u{7a8}އ\u{7aa} ގ\u{7a8}ނ\u{7a9}",
        ),
        ("dz", "པ་པ\u{f74}་འ་ ན\u{f72}འ\u{f74}་ག\u{f72}་ན\u{f72}།"),
        ("ee", "Papua New Guinea"),
        ("el", "Παπούα Νέα Γουινέα"),
        ("en", "Papua New Guinea"),
        ("eo", "Papuo-Nov-Gvineo"),
        ("es", "Papúa Nueva Guinea"),
        ("et", "Paapua Uus-Guinea"),
        ("eu", "Papua Ginea Berria"),
        ("fa", "پاپوا گینه\u{654} نو"),
        ("ff", "Papua New Guinea"),
        ("fi", "Papua-Uusi-Guinea"),
        ("fo", "Papua Nýguinea"),
        ("fr", "Papouasie-Nouvelle-Guinée"),
        ("fy", "Papoea Nij-Guinea"),
        ("ga", "Papua Nua-Ghuine"),
        ("gl", "Papúa Nova Guinea"),
        ("gn", "Papua New Guinea"),
        ("gu", "પાપ\u{ac1}આ ન\u{acd}ય\u{ac1} ગીની"),
        ("gv", "Papooey Guinea Noa"),
        ("ha", "Papua New Guinea"),
        ("he", "פפואה גינאה החדשה"),
        ("hi", "पाप\u{941}आ न\u{94d}य\u{942} गिनी"),
        ("hr", "Papua Nova Gvineja"),
        ("ht", "Papwazi-Nouvèl-Gine"),
        ("hu", "Pápua Új-Guinea"),
        ("hy", "Պապուա Նոր Գվինեա"),
        ("ia", "Papua Nove Guinea"),
        ("id", "Papua Nugini"),
        ("io", "Papua-Nova-Guinea"),
        ("is", "Papúa Nýja-Gínea"),
        ("it", "Papua Nuova Guinea"),
        ("iu", "Papua New Guinea"),
        ("ja", "パプアニューギニア"),
        ("ka", "პაპუა-ახალი გვინეა"),
        ("ki", "Papua New Guinea"),
        ("kk", "Папуа - Жаңа Гвинея"),
        ("kl", "Papua New Guinea"),
        ("km", "ប\u{17c9}ាព\u{17bc}ញ\u{17bc}វហ\u{17d2}គ\u{17b8}ណេ"),
        ("kn", "ಪಪುವಾ ನ\u{ccd}ಯ\u{cc2} ಗ\u{cbf}ನ\u{cbf}"),
        ("ko", "파푸아뉴기니"),
        ("ku", "Papûa Gîneya Nû"),
        ("kv", "Papua New Guinea"),
        ("kw", "Papua New Guinea"),
        ("ky", "Папуа-Жаӊы Гвинея"),
        ("lo", "ປະເທດປາປ\u{ebb}ວຊ\u{eb5}ນ\u{eb9}ແວນກ\u{eb5}ເນ"),
        ("lt", "Papua Naujoji Gvinėja"),
        ("lv", "Papua-Jaungvineja"),
        ("mi", "Papua Nūkini"),
        ("mk", "Папуа Нова Гвинеја"),
        ("ml", "പ\u{d3e}പ\u{d41}വ ന\u{d4d}യ\u{d42} ഗിനിയ"),
        ("mn", "Папуа Шинэ Гвиней"),
        ("mr", "पाप\u{942} न\u{94d}य\u{942} गिनी"),
        ("ms", "Papua New Guinea"),
        ("mt", "Papwa-Ginea Ġdida"),
        (
            "my",
            "ပါပ\u{1030}အာနယ\u{1030}းဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Papua New Guinea"),
        ("nb", "Papua Ny-Guinea"),
        ("ne", "पप\u{941}वा न\u{94d}य\u{942} जिनिया"),
        ("nl", "Papoea-Nieuw-Guinea"),
        ("nn", "Papua Ny-Guinea"),
        ("nv", "Chooyééł Dineʼé Bikéyah"),
        ("oc", "Papoa Nòva Guinèa"),
        ("or", "ପ\u{b3e}ପ\u{b41}ଆ ନ\u{b4d}ଯ\u{b41} ଗ\u{b3f}ନୀ"),
        ("pa", "ਪਾਪ\u{a42}ਆ ਨਵਾ\u{a02} ਗ\u{a42}ਈਨਿਆ"),
        ("pi", "पप\u{941}वा न\u{94d}य\u{942} गिनी"),
        ("pl", "Papua-Nowa Gwinea"),
        ("ps", "Papua New Guinea"),
        ("pt", "Papua Nova Guiné"),
        ("pt_BR", "Papua-Nova Guiné"),
        ("ro", "Papua Noua Guinee"),
        ("ru", "Папуа — Новая Гвинея"),
        ("rw", "Papuwa Nuveli Gineya"),
        ("sc", "Pàpua Guinea Noa"),
        ("sd", "Papua New Guinea"),
        (
            "si",
            "පැප\u{dd4}ව\u{dcf} න\u{dd2}ව\u{dca}ග\u{dd2}න\u{dd2}ය\u{dcf}ව",
        ),
        ("sk", "Papua - Nová Guinea"),
        ("sl", "Papua Nova Gvineja"),
        ("so", "Papua New Guinea"),
        ("sq", "Guinea e Re Papua"),
        ("sr", "Папуа Нова Гвинеја"),
        ("sv", "Papua Nya Guinea"),
        ("sw", "Papua New Guinea"),
        ("ta", "ப\u{bbe}ப\u{bcd}புவ\u{bbe}-நியூகினி"),
        ("te", "ప\u{c3e}పూ న\u{c4d}యూ గ\u{c3f}న\u{c40}"),
        ("tg", "Папуа Гвинеяи Нав"),
        ("th", "ปาป\u{e31}วน\u{e34}วก\u{e34}น\u{e35}"),
        ("ti", "ፓፑዋ ኒው ጊኒ"),
        ("tk", "Papua-Täze Gwineýa"),
        ("tl", "Papua New Guinea"),
        ("tr", "Papua Yeni Gine"),
        ("tt", "Папуа Яңа Gуинеа"),
        ("ug", "پاپۇئا يېڭى گىۋىنېيەسى"),
        ("uk", "Папуа Нова Гвінея"),
        ("ur", "پاپوا نیو گنی"),
        ("uz", "Papua Yangi Gvineya"),
        ("ve", "Papua New Guinea"),
        ("vi", "Pa-pu-a Niu Ghi-nê"),
        ("wa", "Papouwazeye Nouve Guinêye"),
        ("wo", "Papwa Nuweel Ginne"),
        ("xh", "Papua New Guinea"),
        ("yo", "Papua Guinea Titun"),
        ("zh_CN", "巴布亚新几内亚"),
        ("zh_HK", "巴布亞新畿內亞"),
        ("zh_TW", "巴布亞紐幾內亞"),
        ("zu", "Papua New Guinea"),
    ];
    #[cfg(all(feature = "pg", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -6.314992999999999;
        pub const LONGITUDE: f64 = 143.95555;
        pub const MAX_LATITUDE: f64 = -0.6702;
        pub const MAX_LONGITUDE: f64 = 159.9609001;
        pub const MIN_LATITUDE: f64 = -12.0823;
        pub const MIN_LONGITUDE: f64 = 140.8419695;
        pub const NORTHEAST_LATITUDE: f64 = -0.6702;
        pub const NORTHEAST_LONGITUDE: f64 = 159.9609001;
        pub const SOUTHWEST_LATITUDE: f64 = -12.0823;
        pub const SOUTHWEST_LONGITUDE: f64 = 140.8419695;
    }
}
#[cfg(all(feature = "pg", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -6.314992999999999,
            longitude: 143.95555,
            max_latitude: -0.6702,
            max_longitude: 159.9609001,
            min_latitude: -12.0823,
            min_longitude: 140.8419695,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -0.6702,
                    longitude: 159.9609001,
                },
                southwest: CountryGeoBound {
                    latitude: -12.0823,
                    longitude: 140.8419695,
                },
            },
        }
    }
}

#[cfg(all(feature = "pg", feature = "subdivisions"))]
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
                    "CPK",
                    Subdivision{
                        name: "CPK",
                        country_alpha2: Alpha2::PG,
                        code: "CPK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.308768199999999), longitude: Some(144.8731219), max_latitude: Some(-5.775725), min_latitude: Some(-6.8954711), max_longitude: Some(145.33975), min_longitude: Some(144.4285419)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تشيمبو"), ("bg", "Симбу"), ("bn", "চিম\u{9cd}ব\u{9c1} প\u{9cd}রদেশ"), ("ccp", "𑄌\u{11128}𑄟\u{11134}𑄝\u{1112a}"), ("ceb", "Chimbu Province"), ("cs", "Chimbu"), ("da", "Chimbu Province"), ("de", "Chimbu Province"), ("el", "Σίμπου"), ("en", "Chimbu"), ("es", "Simbu"), ("et", "Chimbu"), ("eu", "Simbu"), ("fa", "استان چیمبو"), ("fi", "Chimbun lääni"), ("fr", "Simbu"), ("gl", "Simbu"), ("gu", "ચિમ\u{acd}બ\u{ac1} પ\u{acd}રા\u{a82}ત"), ("hi", "किम\u{94d}ब\u{942} प\u{94d}रा\u{902}त"), ("id", "Provinsi Simbu"), ("it", "provincia di Chimbu"), ("ja", "シンブ州"), ("ka", "სიმბუ"), ("kn", "ಚ\u{cbf}ಂಬು ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "심부 주"), ("lt", "Čimbu provincija"), ("lv", "Simbu (Čimbu) province"), ("mr", "चि\u{902}ब प\u{94d}रा\u{902}त"), ("ms", "Simbu"), ("nb", "Simbu provins"), ("nl", "Chimbu"), ("no", "Simbu provins"), ("pl", "Simbu"), ("pt", "Simbu"), ("ro", "Provincia Simbu"), ("ru", "Симбу"), ("si", "ච\u{dd2}ම\u{dca}බ\u{dd4} පළ\u{dcf}ත"), ("sv", "Simbu"), ("ta", "சிம\u{bcd}பு ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చ\u{c3f}ంబూ ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}มบ\u{e39}"), ("tr", "Chimbu"), ("uk", "Сімбу"), ("ur", "چیمبو صوبہ"), ("vi", "Simbu"), ("zh", "欽布省")]),
                        unofficial_name_list: ["Chimbu", "Simbúa"].to_vec(),
                    }
                ),
                (
                    "CPM",
                    Subdivision{
                        name: "CPM",
                        country_alpha2: Alpha2::PG,
                        code: "CPM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.1360187), longitude: Some(147.4627259), max_latitude: Some(-7.773521), min_latitude: Some(-10.3948627), max_longitude: Some(149.6684618), min_longitude: Some(146.381344)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المحافظة الوسطى"), ("bg", "Централна провинция"), ("bn", "সেন\u{9cd}ট\u{9cd}র\u{9be}ল প\u{9cd}রদেশ"), ("ca", "Província Central"), ("ceb", "Central Province (lalawigan sa Papua New Guinea)"), ("da", "Central Province"), ("de", "Central Province"), ("el", "Κεντρική Επαρχία, Παπούα Νέα Γουινέα"), ("en", "Central"), ("es", "Provincia Central"), ("et", "Keskprovints"), ("eu", "Erdialdeko probintzia"), ("fa", "استان مرکزی"), ("fi", "Central Province"), ("fr", "Province centrale"), ("gl", "Central"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રલ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{947}\u{902}द\u{94d}रीय प\u{94d}रा\u{902}त"), ("id", "Provinsi Tengah"), ("it", "provincia Centrale"), ("ja", "中央州"), ("ka", "ცენტრალური პროვინცია"), ("kn", "ಮಧ\u{ccd}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "중앙 주"), ("lt", "Centrinė provincija"), ("lv", "Centrālā province"), ("mr", "मध\u{94d}यवर\u{94d}ती प\u{94d}रा\u{902}त"), ("ms", "Central Province"), ("nb", "Central"), ("nl", "Central"), ("no", "Central"), ("pl", "Prowincja Centralna"), ("pt", "Central"), ("ro", "Provincia Centrală"), ("ru", "Центральная провинция"), ("si", "මද\u{dca}\u{200d}යම පළ\u{dcf}ත"), ("sv", "Central, Papua Nya Guinea"), ("ta", "சென\u{bcd}ட\u{bcd}ரல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}రల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซ\u{e47}นทร\u{e31}ล โพว\u{e34}\u{e49}น"), ("tr", "Central Province"), ("uk", "Центральна провінція"), ("ur", "مرکزی صوبہ"), ("vi", "Tỉnh Trung Ương"), ("zh", "中央省")]),
                        unofficial_name_list: ["Papua Central"].to_vec(),
                    }
                ),
                (
                    "EBR",
                    Subdivision{
                        name: "EBR",
                        country_alpha2: Alpha2::PG,
                        code: "EBR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.612894300000001), longitude: Some(151.8877321), max_latitude: Some(-4.0885039), min_latitude: Some(-6.068317899999999), max_longitude: Some(152.4966978), min_longitude: Some(150.5976151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شرق بريطانيا الجديدة"), ("bg", "Източна Нова Британия"), ("bn", "ইস\u{9cd}ট নিউ ব\u{9cd}রিটেন"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄚\u{11131} 𑄝\u{11133}𑄢\u{11128}𑄑𑄬𑄚\u{11134}"), ("ceb", "East New Britain Province"), ("da", "East New Britain"), ("de", "East New Britain Province"), ("el", "Ανατολική Νέα Βρετανία"), ("en", "East New Britain"), ("es", "Nueva Bretaña del Este"), ("et", "Ida-Uus-Britannia"), ("eu", "Ekialdeko Britainia Berria"), ("fa", "استان بریتانیای نو شرقی"), ("fi", "East New Britain"), ("fr", "Nouvelle-Bretagne orientale"), ("gl", "Nova Bretaña Oriental"), ("gu", "ઇસ\u{acd}ટ ન\u{acd}ય\u{ac2} બ\u{acd}રિટન"), ("hi", "प\u{942}र\u{94d}वी न\u{94d}य\u{942} ब\u{94d}रिट\u{947}न"), ("id", "Britania Baru Timur"), ("it", "provincia della Nuova Britannia Est"), ("ja", "東ニューブリテン州"), ("ka", "აღმოსავლეთი ახალი ბრიტანეთი"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ನ\u{ccd}ಯ\u{cc2} ಬ\u{ccd}ರ\u{cbf}ಟನ\u{ccd}"), ("ko", "동뉴브리튼 주"), ("lt", "Rytinė Naujosios Britanijos provincija"), ("lv", "Austrumjaunbritānijas province"), ("mk", "Источна Нова Британија"), ("mr", "प\u{942}र\u{94d}व न\u{94d}य\u{942} ब\u{94d}रिटन"), ("ms", "East New Britain"), ("nb", "Øst Ny Britland"), ("nl", "East New Britain"), ("no", "Øst Ny Britland"), ("pl", "Nowa Brytania Wschodnia"), ("pt", "Nova Bretanha Oriental"), ("ru", "Восточная Новая Британия"), ("si", "නැගෙනහ\u{dd2}ර නව බ\u{dca}\u{200d}ර\u{dd2}ත\u{dcf}න\u{dca}\u{200d}ය"), ("sv", "East New Britain"), ("ta", "கிழக\u{bcd}கு நியூ பிரிட\u{bcd}டன\u{bcd}"), ("te", "తూర\u{c4d}పు న\u{c4d}యూ బ\u{c4d}ర\u{c3f}టన\u{c4d}"), ("th", "น\u{e34}วบร\u{e34}เตนตะว\u{e31}นออก"), ("tr", "Doğu Yeni Britanya"), ("uk", "Східна Нова Британія"), ("ur", "مشرقی نیا برطانیہ صوبہ"), ("vi", "Đông New Britain"), ("zh", "東新不列顛省")]),
                        unofficial_name_list: ["New Britain East"].to_vec(),
                    }
                ),
                (
                    "EHG",
                    Subdivision{
                        name: "EHG",
                        country_alpha2: Alpha2::PG,
                        code: "EHG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.5861674), longitude: Some(145.6689636), max_latitude: Some(-5.853335), min_latitude: Some(-7.1572681), max_longitude: Some(146.149775), min_longitude: Some(144.985481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هايلاند الشرقية"), ("bg", "Източни височини"), ("bn", "প\u{9c2}র\u{9cd}ব হ\u{9be}ইল\u{9cd}য\u{9be}ন\u{9cd}ডস প\u{9cd}রদেশ"), ("ccp", "𑄛\u{1112a}𑄇\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄦\u{1112d}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Eastern Highlands Province"), ("da", "Eastern Highlands Province"), ("de", "Eastern Highlands Province"), ("el", "Ανατολικά Υψίπεδα"), ("en", "Eastern Highlands"), ("es", "Tierras Altas Orientales"), ("et", "Eastern Highlands"), ("eu", "Ekialdeko Lur Garaiak"), ("fa", "استان ارتفاعات شرقی"), ("fi", "Eastern Highlands lääni"), ("fr", "Hautes-Terres orientales"), ("gl", "Eastern Highlands"), ("gu", "પ\u{ac2}ર\u{acd}વીય હાઈલ\u{ac7}ન\u{acd}ડ\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{942}र\u{94d}वी हाइल\u{948}\u{902}ड\u{94d}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Dataran Tinggi Timur"), ("it", "provincia degli Altopiani Orientali"), ("ja", "東部山岳州"), ("ka", "ისტერნ ჰაილენდსი"), ("kn", "ಪ\u{cc2}ರ\u{ccd}ವ ಹೈಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "이스턴하일랜즈 주"), ("lt", "Rytų Kanų provincija"), ("lv", "Austrumu kalnienes province"), ("mr", "प\u{942}र\u{94d}व हाईल\u{901}ड\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Eastern Highlands Province"), ("nb", "Øst høylands provins"), ("nl", "Eastern Highlands"), ("no", "Øst høylands provins"), ("pl", "Eastern Highlands"), ("pt", "Eastern Highlands"), ("ru", "Истерн-Хайлендс"), ("si", "නැගෙනහ\u{dd2}ර හය\u{dd2}ලන\u{dca}ඩ\u{dca}ස\u{dca}"), ("sv", "Eastern Highlands"), ("ta", "கிழக\u{bcd}கு ஹயில\u{bbe}ன\u{bcd}ட\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు ద\u{c40}వుల ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "อ\u{e35}สเท\u{e34}ร\u{e4c}นไฮแลนดส\u{e4c}"), ("tr", "Doğu Highlands"), ("uk", "Східний Гайлендс"), ("ur", "مشرقی سطح مرتفع صوبہ"), ("vi", "Tỉnh Eastern Highlands"), ("zh", "東高地省")]),
                        unofficial_name_list: ["Highlands East"].to_vec(),
                    }
                ),
                (
                    "EPW",
                    Subdivision{
                        name: "EPW",
                        country_alpha2: Alpha2::PG,
                        code: "EPW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.3005849), longitude: Some(143.5635637), max_latitude: Some(-5.0077089), min_latitude: Some(-5.9661789), max_longitude: Some(144.2541971), min_longitude: Some(142.7393411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إنغا"), ("bg", "Енга"), ("bn", "এঙ\u{9cd}গ\u{9be} প\u{9cd}রদেশ"), ("ccp", "𑄃𑄬𑄚\u{11134}𑄉"), ("ceb", "Enga Province"), ("da", "Enga Province"), ("de", "Enga Province"), ("el", "Ένγκα"), ("en", "Enga"), ("es", "Enga"), ("et", "Enga"), ("eu", "Enga"), ("fa", "استان انگا"), ("fi", "Engan provinssi"), ("fr", "Enga"), ("gl", "Enga"), ("gu", "ઈ\u{a82}ગા પ\u{acd}રા\u{a82}ત"), ("hi", "ए\u{902}गा प\u{94d}रा\u{902}त"), ("id", "Provinsi Enga"), ("it", "provincia di Enga"), ("ja", "エンガ州"), ("ka", "ენგა"), ("kn", "ಎಂಗಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "엥가 주"), ("lt", "Engos provincija"), ("lv", "Engas province"), ("mr", "ए\u{902}गा प\u{94d}रा\u{902}त"), ("ms", "Enga Province"), ("nb", "Enga provins"), ("nl", "Enga"), ("no", "Enga provins"), ("pl", "Enga"), ("pt", "Enga"), ("ru", "Энга"), ("si", "එන\u{dca}ග\u{dcf} පළ\u{dcf}ත"), ("sv", "Enga"), ("ta", "எங\u{bcd}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఎంగ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเอนก\u{e49}า"), ("tr", "Enga Province"), ("uk", "Енга"), ("ur", "انگا صوبہ"), ("vi", "Tỉnh Enga"), ("zh", "恩加省")]),
                        unofficial_name_list: ["Enga"].to_vec(),
                    }
                ),
                (
                    "ESW",
                    Subdivision{
                        name: "ESW",
                        country_alpha2: Alpha2::PG,
                        code: "ESW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.3150058), longitude: Some(143.045893), max_latitude: Some(-3.1911646), min_latitude: Some(-5.151646), max_longitude: Some(144.8318195), min_longitude: Some(141.314843)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيبيك الشرقية"), ("bg", "Източен Сепик"), ("bn", "প\u{9c2}র\u{9cd}ব সেপিক প\u{9cd}রদেশ"), ("ccp", "𑄛\u{1112a}𑄇\u{11134} 𑄥𑄬𑄛\u{1112d}𑄇\u{11134}"), ("ceb", "East Sepik Province"), ("da", "East Sepik Province"), ("de", "East Sepik Province"), ("el", "Ανατολική Σέπικ"), ("en", "East Sepik"), ("es", "Sepik Oriental"), ("et", "Ida-Sepik"), ("eu", "Ekialdeko Sepik"), ("fa", "استان سپیک شرقی"), ("fi", "East Sepikin lääni"), ("fr", "Sepik oriental"), ("gl", "Sepik do Leste"), ("gu", "પ\u{ac2}ર\u{acd}વ સ\u{ac7}પિક પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{942}र\u{94d}वी स\u{947}पिक प\u{94d}रा\u{902}त"), ("id", "Provinsi Sepik Timur"), ("it", "provincia di Sepik Est"), ("ja", "東セピック州"), ("ka", "აღმოსავლეთი სეპიკი"), ("kn", "ಈಸ\u{ccd}ಟ\u{ccd} ಸ\u{cc6}ಪ\u{cbf}ಕ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "동세픽 주"), ("lt", "Rytų Sepiko provincija"), ("lv", "Austrumu Sepikas province"), ("mr", "प\u{942}र\u{94d}व स\u{947}पिक प\u{94d}रा\u{902}त"), ("ms", "East Sepik Province"), ("nb", "East Sepik"), ("nl", "East Sepik"), ("no", "East Sepik"), ("pl", "Sepik Wschodni"), ("pt", "East Sepik"), ("ru", "Восточный Сепик"), ("si", "නැගෙනහ\u{dd2}ර සේප\u{dd2}ක\u{dca} පළ\u{dcf}ත"), ("sv", "East Sepik"), ("ta", "கிழக\u{bcd}கு செபிக\u{bcd}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "తూర\u{c4d}పు స\u{c46}ప\u{c3f}క\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซป\u{e34}กตะว\u{e31}นออก"), ("tr", "Doğu Sepik Province"), ("uk", "Східний Сепік"), ("ur", "مشرقی سپیک صوبہ"), ("vi", "Tỉnh Đông Sepik"), ("zh", "东塞皮克省")]),
                        unofficial_name_list: ["Sepik East"].to_vec(),
                    }
                ),
                (
                    "GPK",
                    Subdivision{
                        name: "GPK",
                        country_alpha2: Alpha2::PG,
                        code: "GPK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.269182100000001), longitude: Some(145.1375834), max_latitude: Some(-6.700165999999999), min_latitude: Some(-8.5877783), max_longitude: Some(146.6471049), min_longitude: Some(143.00871)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الخليج"), ("bg", "Гулф"), ("bn", "ডোলেঞ\u{9cd}জস\u{9cd}ক\u{9be} টপ\u{9cd}লিস পৌরসভ\u{9be}"), ("ccp", "𑄉\u{11127}𑄣\u{11134}𑄛\u{11134}"), ("ceb", "Gulf Province"), ("da", "Gulf"), ("de", "Gulf Province"), ("el", "Κόλπος, Παπούα Νέα Γουινέα"), ("en", "Gulf"), ("es", "Provincia del Golfo"), ("et", "Gulfi provints"), ("eu", "Golkoa (Papua Ginea Berria)"), ("fa", "استان خلیج"), ("fi", "Gulf"), ("fr", "Golfe"), ("gl", "Gulf"), ("gu", "ગલ\u{acd}ફ"), ("hi", "खाड\u{93c}ी"), ("id", "Provinsi Teluk"), ("it", "provincia del Golfo"), ("ja", "湾岸州"), ("ka", "გალფის პროვინცია"), ("kn", "ಗಲ\u{ccd}ಫ\u{ccd}"), ("ko", "걸프 주"), ("lt", "Galfas"), ("lv", "Galfa"), ("mr", "गल\u{94d}फ"), ("ms", "Gulf"), ("nb", "Gulf"), ("nl", "Gulf"), ("no", "Gulf"), ("pl", "Gulf"), ("pt", "Gulf (província)"), ("ru", "Галф"), ("si", "ගල\u{dca}ෆ\u{dca}"), ("sv", "Gulf"), ("ta", "கோல\u{bcd}ப\u{bcd}"), ("te", "గల\u{c4d}ఫ\u{c4d}"), ("th", "ก\u{e31}ฟ"), ("tr", "gulf"), ("uk", "Галф"), ("ur", "گلف صوبہ"), ("vi", "Gulf"), ("zh", "海灣省")]),
                        unofficial_name_list: ["Papua Gulf"].to_vec(),
                    }
                ),
                (
                    "HLA",
                    Subdivision{
                        name: "HLA",
                        country_alpha2: Alpha2::PG,
                        code: "HLA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄦𑄬𑄣"), ("ceb", "Hela (lalawigan)"), ("de", "Hela Province"), ("en", "Hela"), ("es", "Hela"), ("et", "Hela provints"), ("eu", "Hela"), ("fa", "استان هلا"), ("fi", "Hela (maakunta)"), ("fr", "Hela"), ("it", "Provincia di Hela"), ("ja", "ヘラ州"), ("ka", "ჰელა"), ("ko", "헬라 주"), ("nl", "Hela Province"), ("pl", "Hela"), ("pt", "Hela"), ("ru", "Хела"), ("uk", "Гела"), ("ur", "ہیلا صوبہ"), ("zh", "赫拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "JWK",
                    Subdivision{
                        name: "JWK",
                        country_alpha2: Alpha2::PG,
                        code: "JWK",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄎\u{11128}𑄤𑄇"), ("ceb", "Jiwaka"), ("de", "Jiwaka Province"), ("en", "Jiwaka"), ("es", "Jiwaka"), ("et", "Jiwaka provints"), ("eu", "Jiwaka (probintzia)"), ("fa", "استان جیواکا"), ("fi", "Jiwaka"), ("fr", "Jiwaka Province"), ("it", "Provincia di Jiwaka"), ("ja", "ジワカ州"), ("ka", "ჯივაკა"), ("ko", "지와카 주"), ("nl", "Jiwaka Province"), ("pl", "Jiwaka"), ("pt", "Jiwaka"), ("ru", "Дживака"), ("uk", "Дживака"), ("ur", "جیواکا صوبہ"), ("zh", "吉瓦卡省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MBA",
                    Subdivision{
                        name: "MBA",
                        country_alpha2: Alpha2::PG,
                        code: "MBA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-10.4460842), longitude: Some(150.7214362), max_latitude: Some(-10.2622222), min_latitude: Some(-10.5289949), max_longitude: Some(150.8965302), min_longitude: Some(150.346527)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة خليج ميلن"), ("bg", "Милн Бей"), ("bn", "মিলনে বে প\u{9cd}রদেশ"), ("ccp", "𑄟\u{11128}𑄣\u{11134}𑄚𑄬 𑄝𑄬"), ("ceb", "Milne Bay Province"), ("da", "Milne Bay Province"), ("de", "Milne Bay Province"), ("el", "Κόλπος Μίλνε"), ("en", "Milne Bay"), ("es", "Provincia de Milne Bay"), ("eu", "Milne badia (probintzia)"), ("fa", "استان خلیج میلنه"), ("fi", "Milne Bayn provinssi"), ("fr", "Baie de Milne"), ("gl", "Milne Bay"), ("gu", "મિલન\u{ac7} બ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "मिल\u{94d}न\u{947} खाड\u{93c}ी प\u{94d}रा\u{902}त"), ("id", "Provinsi Teluk Milne"), ("it", "provincia di Baia di Milne"), ("ja", "ミルン湾州"), ("ka", "მილნ-ბეი"), ("kn", "ಮ\u{cbf}ಲ\u{ccd}ನ\u{cc6} ಬೇ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "밀른베이 주"), ("lt", "Milne Bėjaus provincija"), ("lv", "Milnbejas province"), ("mr", "मिलब\u{947} ब\u{947} प\u{94d}रा\u{902}त"), ("ms", "Milne Bay Province"), ("nb", "Milne Bay"), ("nl", "Milne Bay"), ("no", "Milne Bay"), ("pl", "Milne Bay"), ("pt", "Milne Bay"), ("ru", "Милн-Бей"), ("si", "ම\u{dd2}ල\u{dca}නේ බේ පළ\u{dcf}ත"), ("sv", "Milne Bay"), ("ta", "மில\u{bcd}னே பே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3f}ల\u{c4d}న\u{c3f} బ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดม\u{e34}ลเน\u{e48} เบย\u{e4c}"), ("tr", "Milne Bay Province"), ("uk", "Мілн-Бей"), ("ur", "خلیج میلنے صوبہ"), ("vi", "Tỉnh Milne Bay"), ("zh", "米爾恩灣省")]),
                        unofficial_name_list: ["Milne Bay"].to_vec(),
                    }
                ),
                (
                    "MPL",
                    Subdivision{
                        name: "MPL",
                        country_alpha2: Alpha2::PG,
                        code: "MPL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.801373700000001), longitude: Some(146.561647), max_latitude: Some(-5.281522300000001), min_latitude: Some(-8.031376999999999), max_longitude: Some(148.1150151), min_longitude: Some(145.742827)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة موروب"), ("bg", "Моробе"), ("bn", "মোরবে প\u{9cd}রদেশের"), ("ccp", "𑄟\u{11127}𑄢\u{1112e}𑄝𑄬"), ("ceb", "Morobe Province"), ("da", "Morobe Province"), ("de", "Morobe Province"), ("el", "Μορόμπε"), ("en", "Morobe"), ("es", "Morobe"), ("et", "Morobe provints"), ("eu", "Morobe"), ("fa", "استان موروبه"), ("fi", "Moroben lääni"), ("fr", "Morobe"), ("gl", "Morobe"), ("gu", "મોરોબ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "मोरोब\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Morobe"), ("it", "provincia di Morobe"), ("ja", "モロベ州"), ("ka", "მორობე"), ("kn", "ಮೊರೊಬ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "모로베 주"), ("lt", "Morobės provincija"), ("lv", "Morobes province"), ("mr", "मोरोब\u{947} प\u{94d}रा\u{902}त"), ("ms", "Morobe Province"), ("nb", "Morobe provins"), ("nl", "Morobe"), ("no", "Morobe provins"), ("pl", "Morobe"), ("pt", "Morobe"), ("ru", "Моробе"), ("si", "මොරෝබේ පළ\u{dcf}ත"), ("sv", "Morobe"), ("ta", "மோர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c4b}ర\u{c4b}బ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ด โมโรเบ"), ("tr", "Morobe Province"), ("uk", "Моробе"), ("ur", "موروبے صوبہ"), ("vi", "Tỉnh Morobe"), ("zh", "莫雷贝省")]),
                        unofficial_name_list: ["Morobe"].to_vec(),
                    }
                ),
                (
                    "MPM",
                    Subdivision{
                        name: "MPM",
                        country_alpha2: Alpha2::PG,
                        code: "MPM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.216667), longitude: Some(145.8), max_latitude: Some(-5.203685699999999), min_latitude: Some(-5.2554161), max_longitude: Some(145.8166081), min_longitude: Some(145.7496071)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مادانغ"), ("bg", "Маданг"), ("bn", "ম\u{9be}দ\u{9be}ং প\u{9cd}রদেশ"), ("ccp", "𑄟𑄬𑄓𑄋\u{11134}"), ("ceb", "Madang Province"), ("da", "Madang Province"), ("de", "Madang Province"), ("el", "Μάντανγκ"), ("en", "Madang"), ("es", "Provincia de Madang"), ("et", "Madangi provints"), ("eu", "Madang"), ("fa", "استان مادانگ"), ("fi", "Madang"), ("fr", "Madang"), ("gl", "Provincia de Madang"), ("gu", "મદા\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "मदा\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Madang tartomány"), ("id", "Provinsi Madang"), ("it", "provincia di Madang"), ("ja", "マダン州"), ("ka", "მადანგი"), ("kn", "ಮಡಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마당 주"), ("lt", "Madango provincija"), ("lv", "Madangas province"), ("mr", "माद\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Madang Province"), ("nb", "Madang provins"), ("nl", "Madang"), ("no", "Madang provins"), ("pl", "Madang"), ("pt", "Madang"), ("ru", "Маданг"), ("si", "මඩ\u{dcf}ංග\u{dca} පළ\u{dcf}ත"), ("sv", "Madang"), ("ta", "மடங\u{bcd}கி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మడ\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมาด\u{e31}ง"), ("tr", "Madang ili"), ("uk", "Маданг"), ("ur", "مادنگ صوبہ"), ("vi", "Tỉnh Madang"), ("zh", "馬當省")]),
                        unofficial_name_list: ["Madang"].to_vec(),
                    }
                ),
                (
                    "MRL",
                    Subdivision{
                        name: "MRL",
                        country_alpha2: Alpha2::PG,
                        code: "MRL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-2.0941169), longitude: Some(146.8760951), max_latitude: Some(-0.8726387000000001), min_latitude: Some(-2.5770971), max_longitude: Some(148.2065104), min_longitude: Some(142.8257212)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مانوس"), ("bg", "Манус"), ("bn", "ম\u{9be}ন\u{9c1}স প\u{9cd}রদেশ"), ("ccp", "𑄟𑄚\u{1112a}𑄌\u{11134}"), ("ceb", "Manus Province"), ("da", "Manus Province"), ("de", "Manus Province"), ("el", "Μάνους"), ("en", "Manus"), ("es", "Provincia de Manus"), ("eu", "Manus"), ("fa", "استان مانوس"), ("fi", "Manus"), ("fr", "Manus"), ("gl", "Provincia de Manus"), ("gu", "મ\u{ac7}નસ પ\u{acd}રા\u{a82}ત"), ("hi", "मन\u{941}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Manus"), ("it", "provincia di Manus"), ("ja", "マヌス州"), ("jv", "Provinsi Manus"), ("ka", "მანუსი"), ("kn", "ಮನುಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "마누스 주"), ("lt", "Manuso provincija"), ("lv", "Manusas province"), ("mr", "मन\u{941}स प\u{94d}रा\u{902}त"), ("ms", "Manus Province"), ("nb", "Manus provins"), ("nl", "Manus"), ("no", "Manus provins"), ("pl", "Manus"), ("pt", "Manus (província)"), ("ru", "Манус"), ("si", "මන\u{dd4}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Manus"), ("ta", "மனுஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మనుస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกรานมา"), ("tr", "Manus Province"), ("uk", "Манус (провінція)"), ("ur", "مانوس صوبہ"), ("vi", "Tỉnh Manus"), ("zh", "馬努斯省")]),
                        unofficial_name_list: ["Great Admiralty Island", "Mwanus"].to_vec(),
                    }
                ),
                (
                    "NCD",
                    Subdivision{
                        name: "NCD",
                        country_alpha2: Alpha2::PG,
                        code: "NCD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.443800399999999), longitude: Some(147.1802671), max_latitude: Some(-9.3703217), min_latitude: Some(-9.5045786), max_longitude: Some(147.2438668), min_longitude: Some(147.136652)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Port Moresby"), ("am", "ፖርት ሞርስቢ"), ("ar", "بورت مورسبي"), ("az", "Port Morsbi"), ("be", "Горад Порт-Морсбі"), ("bg", "Порт Морсби"), ("bn", "পোর\u{9cd}টমোরেস\u{9cd}বি"), ("bs", "Port Moresby"), ("ca", "Port Moresby"), ("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{11134} 𑄟\u{1112e}𑄢\u{11134}𑄝\u{1112d}"), ("ceb", "Port Moresby"), ("cs", "Port Moresby"), ("cy", "Port Moresby"), ("da", "Port Moresby"), ("de", "Port Moresby"), ("el", "Πορτ Μόρεσμπι"), ("en", "Port Moresby"), ("es", "Puerto Moresby"), ("et", "Port Moresby"), ("eu", "Port Moresby"), ("fa", "پورت مورسبی"), ("fi", "Port Moresby"), ("fr", "Port Moresby"), ("ga", "Port Moresby"), ("gl", "Port Moresby"), ("gu", "પોર\u{acd}ટ મોર\u{acd}સબી"), ("he", "פורט מורסבי"), ("hi", "पोर\u{94d}ट मोर\u{947}स\u{94d}बी"), ("hr", "Port Moresby"), ("hu", "Port Moresby"), ("hy", "Պորտ Մորսբի"), ("id", "Port Moresby"), ("is", "Port Moresby"), ("it", "Port Moresby"), ("ja", "ポートモレスビー"), ("jv", "Port Moresby"), ("ka", "პორტ-მორზბი"), ("kk", "Порт-Морсби"), ("kn", "ಪೋರ\u{ccd}ಟ\u{ccd} ಮೊರ\u{cc6}ಸ\u{ccd}ಬ\u{cbf}"), ("ko", "포트모르즈비"), ("ky", "Порт-Морсби"), ("lt", "Port Morsbis"), ("lv", "Portmorsbi"), ("mk", "Порт Морсби"), ("ml", "പോർട\u{d4d}ട\u{d4d} മോറെസ\u{d4d}ബി"), ("mn", "Порт-Морсби"), ("mr", "पोर\u{94d}ट मॉर\u{947}स\u{94d}बी"), ("ms", "Port Moresby"), ("nb", "Port Moresby"), ("ne", "पोर\u{94d}ट मोर\u{947}स\u{94d}बी"), ("nl", "Port Moresby"), ("no", "Port Moresby"), ("pa", "ਪ\u{a4b}ਰਟ ਮ\u{a4b}ਰ\u{a48}ਸਬੀ"), ("pl", "Port Moresby"), ("pt", "Port Moresby"), ("ro", "Port Moresby"), ("ru", "Порт-Морсби"), ("si", "පොර\u{dca}ට\u{dca} මොරෙස\u{dca}බ\u{dd2}"), ("sk", "Port Moresby"), ("sl", "Port Moresby"), ("sq", "Port Moresbi"), ("sr", "Порт Морсби"), ("sr_Latn", "Port Morsbi"), ("sv", "Port Moresby"), ("sw", "Port Moresby"), ("ta", "ம\u{bbe}ர\u{bcd}சுபி துறைமுகம\u{bcd}"), ("te", "ప\u{c4b}ర\u{c4d}ట\u{c4d} మ\u{c4b}ర\u{c46}స\u{c4d}బ\u{c40}"), ("th", "พอร\u{e4c}ตมอร\u{e4c}สบ\u{e35}"), ("tk", "Port-Morsbi"), ("tr", "Port Moresby"), ("uk", "Порт-Морсбі"), ("ur", "پورٹ مورسبی"), ("uz", "Port Morsbi"), ("vi", "Port Moresby"), ("yo", "Port Moresby"), ("yo_BJ", "Port Moresby"), ("yue", "莫士比港"), ("yue_Hans", "莫士比港"), ("zh", "莫尔兹比港")]),
                        unofficial_name_list: ["National Capital District (Port Moresby)"].to_vec(),
                    }
                ),
                (
                    "NIK",
                    Subdivision{
                        name: "NIK",
                        country_alpha2: Alpha2::PG,
                        code: "NIK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-4.2853256), longitude: Some(152.9205918), max_latitude: Some(-1.315154), min_latitude: Some(-4.8519637), max_longitude: Some(153.7421607), min_longitude: Some(149.5108796)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة نيو إريلاند"), ("bg", "Нова Ирландия"), ("bn", "নিউ আয\u{9bc}\u{9be}রল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}রদেশ"), ("ccp", "𑄚\u{11128}𑄅\u{1112a} 𑄃𑄠𑄢\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}"), ("ceb", "New Ireland Province"), ("da", "New Ireland Province"), ("de", "New Ireland Province"), ("el", "Νέα Ιρλανδία"), ("en", "New Ireland"), ("es", "Provincia de Nueva Irlanda"), ("et", "Uus-Iirimaa provints"), ("eu", "Irlanda Berria (probintzia)"), ("fa", "استان ایرلند نو"), ("fi", "New Ireland"), ("fr", "Nouvelle-Irlande"), ("gl", "Provincia de Nova Irlanda"), ("gu", "ન\u{acd}ય\u{ac2} આયર\u{acd}લ\u{ac7}ન\u{acd}ડ પ\u{acd}રા\u{a82}ત"), ("hi", "न\u{94d}य\u{942} आयरल\u{948}\u{902}ड प\u{94d}रा\u{902}त"), ("id", "Provinsi Irlandia Baru"), ("it", "provincia della Nuova Irlanda"), ("ja", "ニューアイルランド州"), ("ka", "ახალი ირლანდია"), ("kn", "ನ\u{ccd}ಯ\u{cc2} ಐರ\u{ccd}ಲ\u{cc6}ಂಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "뉴아일랜드 주"), ("lt", "Naujosios Airijos provincija"), ("lv", "Jaunīrijas province"), ("mr", "न\u{94d}य\u{942} आयर\u{94d}ल\u{902}ड प\u{94d}रा\u{902}त"), ("ms", "New Ireland Province"), ("nb", "Ny Irland provins"), ("nl", "New Ireland Province"), ("no", "Ny Irland provins"), ("pl", "Nowa Irlandia"), ("pt", "Nova Irlanda"), ("ru", "Новая Ирландия"), ("si", "නව අයර\u{dca}ලන\u{dca}ත පළ\u{dcf}ත"), ("sv", "New Ireland"), ("ta", "நியூ இரேலண\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "న\u{c4d}యూ ఐర\u{c4d}లండ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดน\u{e34}วไอร\u{e4c}แลนด\u{e4c}"), ("tr", "New Ireland Province"), ("uk", "Нова Ірландія"), ("ur", "نیو آئر لینڈ صوبہ"), ("vi", "Tỉnh New Ireland"), ("zh", "新愛爾蘭省")]),
                        unofficial_name_list: ["Niu Ailan"].to_vec(),
                    }
                ),
                (
                    "NPP",
                    Subdivision{
                        name: "NPP",
                        country_alpha2: Alpha2::PG,
                        code: "NPP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-8.8988063), longitude: Some(148.1892921), max_latitude: Some(-8.002543), min_latitude: Some(-9.977337), max_longitude: Some(149.439636), min_longitude: Some(147.0030289)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أورو"), ("bg", "Оро"), ("bn", "ওরো প\u{9cd}রদেশ"), ("ccp", "𑄃\u{11127}𑄢\u{1112e}"), ("ceb", "Northern Province (lalawigan sa Papua New Guinea)"), ("da", "Oro Province"), ("de", "Oro Province"), ("el", "Όρο (Βόρεια Επαρχία)"), ("en", "Oro"), ("es", "Provincia de Oro"), ("et", "Põhjaprovints"), ("eu", "Iparraldeko probintzia"), ("fa", "استان شمالی"), ("fi", "Oro"), ("fr", "Province nord"), ("gl", "Oro"), ("gu", "ઓરો પ\u{acd}રા\u{a82}ત"), ("hi", "ओरो प\u{94d}रा\u{902}त"), ("id", "Provinsi Oro"), ("it", "provincia di Oro"), ("ja", "オロ州"), ("ka", "ორო"), ("kn", "ಓರೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오로 주"), ("lt", "Oro provincija"), ("lv", "Oro province"), ("mr", "ओरो प\u{94d}रा\u{902}त"), ("ms", "Oro Province"), ("nb", "Oro"), ("nl", "Northern"), ("no", "Oro"), ("pl", "Oro"), ("pt", "Oro"), ("ro", "Provincia Oro"), ("ru", "Оро"), ("si", "ඔරෝ පළ\u{dcf}ත"), ("sv", "Oro"), ("ta", "ஒரோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఓర\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดนอร\u{e4c}ทเท\u{e34}ร\u{e4c}น"), ("tr", "Oro Province"), ("uk", "Оро"), ("ur", "اورو صوبہ"), ("vi", "Tỉnh Oro"), ("zh", "北部省")]),
                        unofficial_name_list: ["Northern"].to_vec(),
                    }
                ),
                (
                    "NSB",
                    Subdivision{
                        name: "NSB",
                        country_alpha2: Alpha2::PG,
                        code: "NSB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوغانفيل ذات الحكم الذاتي"), ("bg", "Автономен регион Бугенвил"), ("bn", "ব\u{9c1}গ\u{9be}ইনভিলি"), ("ca", "Bougainville"), ("ccp", "𑄝\u{1112f}𑄉𑄬\u{1112d}𑄚\u{11134}𑄞\u{11128}𑄣𑄬"), ("ceb", "Bougainville (lalawigan)"), ("cs", "Autonomní území Bougainville"), ("da", "Bougainville"), ("de", "Bougainville"), ("el", "Μπουγκαϊνβίλε"), ("en", "Bougainville"), ("es", "Bougainville"), ("eu", "Bougainvilleko Eskualde Autonomoa"), ("fa", "استان خودمختار بوگاینویل"), ("fi", "Bougainville"), ("fr", "Bougainville"), ("gl", "Rexión Autónoma de Bougainville"), ("gu", "બૌગ\u{ac8}નવિલ"), ("hi", "बोग\u{947}नविल"), ("id", "Daerah Otonom Bougainville"), ("it", "regione autonoma di Bougainville"), ("ja", "ブーゲンビル州"), ("ka", "ბუგენვილის ავტონომიური რეგიონი"), ("kn", "ಬ\u{ccc}ಗ\u{cc6}ನ\u{ccd}ವ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "부건빌 주"), ("lt", "Bugenvilis"), ("lv", "Bugenvile"), ("mr", "बोगनविल\u{947}"), ("ms", "Bougainville"), ("nb", "Bougainville"), ("nl", "Bougainville"), ("no", "Bougainville"), ("pl", "Bougainville"), ("pt", "Bougainville"), ("ru", "Автономный регион Бугенвиль"), ("si", "බෝගෙන\u{dca}ව\u{dd2}ල\u{dd2}"), ("sv", "Bougainville"), ("ta", "போகஇன\u{bcd}வில\u{bcd}லே"), ("te", "బ\u{c4b}గన\u{c4d}\u{200c}వ\u{c3f}ల\u{c4d}ల\u{c47}"), ("th", "บ\u{e39}แก\u{e47}งว\u{e35}ล"), ("tr", "Bougainville"), ("uk", "Автономний регіон Бугенвіль"), ("ur", "بووجاینویلی"), ("vi", "Khu tự trị Bougainville"), ("zh", "布干维尔省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SAN",
                    Subdivision{
                        name: "SAN",
                        country_alpha2: Alpha2::PG,
                        code: "SAN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-3.7126179), longitude: Some(141.6834275), max_latitude: Some(-2.6086511), min_latitude: Some(-5.370609), max_longitude: Some(143.1048729), min_longitude: Some(140.998795)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سانداون"), ("bg", "Западен Сепик"), ("bn", "স\u{9cd}য\u{9be}নড\u{9c1}য\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ccp", "𑄥𑄚\u{11134}𑄓𑄅\u{1112a}𑄚\u{11134}"), ("ceb", "West Sepik Province"), ("da", "Sandaun Province"), ("de", "Sandaun Province"), ("el", "Σάντον"), ("en", "Sandaun"), ("es", "Sandaun"), ("et", "Sandaun"), ("eu", "Sandaun"), ("fa", "استان سپیک غربی"), ("fi", "Sandaunin lääni"), ("fr", "Sandaun"), ("gl", "Sandaun"), ("gu", "સાન\u{acd}ડોન પ\u{acd}રા\u{a82}ત"), ("hi", "सा\u{902}ड\u{941}न प\u{94d}रा\u{902}त"), ("hr", "Sandaun"), ("id", "Provinsi Sandaun"), ("it", "provincia di Sandaun"), ("ja", "サンダウン州"), ("ka", "სანდაუნი"), ("kn", "ಸಾಂಡನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산다운 주"), ("lt", "Sandauno provincija"), ("lv", "Sandaunas province"), ("mr", "सा\u{902}डान प\u{94d}रा\u{902}त"), ("ms", "Sandaun Province"), ("nb", "Sandaun provins"), ("nl", "Sandaun"), ("no", "Sandaun provins"), ("pl", "Sandaun"), ("pt", "Sandaun"), ("ru", "Сандаун"), ("si", "සැන\u{dca}ඩ\u{dd4}ආන\u{dca} පළ\u{dcf}ත"), ("sv", "Sandaun"), ("ta", "சண\u{bcd}டையும\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ండ\u{c4c}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซานดาอ\u{e38}น"), ("tr", "Sandaun Province"), ("uk", "Сандаун"), ("ur", "سانداون صوبہ"), ("vi", "Tỉnh Sandaun"), ("zh", "桑道恩省")]),
                        unofficial_name_list: ["West Sepik"].to_vec(),
                    }
                ),
                (
                    "SHM",
                    Subdivision{
                        name: "SHM",
                        country_alpha2: Alpha2::PG,
                        code: "SHM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-6.5156911), longitude: Some(143.045893), max_latitude: Some(-4.971731999999999), min_latitude: Some(-6.863055), max_longitude: Some(144.683789), min_longitude: Some(142.0664641)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هايلاند الجنوبية"), ("bg", "Южни височини"), ("bn", "স\u{9be}উদ\u{9be}র\u{9cd}ন হ\u{9be}ইল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}রদেশ"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄦\u{1112d}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Southern Highlands Province"), ("da", "Southern Highlands Province"), ("de", "Southern Highlands Province"), ("el", "Νότια Υψίπεδα"), ("en", "Southern Highlands"), ("es", "Tierras Altas del Sur"), ("et", "Southern Highlands"), ("fa", "استان ارتفاعات جنوبی"), ("fi", "Southern Highlandsin lääni"), ("fr", "Hautes-Terres méridionales"), ("gl", "Southern Highlands"), ("gu", "દક\u{acd}ષિણી હાઈલ\u{ac7}ન\u{acd}ડ\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("hi", "दक\u{94d}षिणी हाइल\u{948}\u{902}ड\u{94d}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Pegunungan Selatan"), ("it", "provincia degli Altopiani del Sud"), ("ja", "南部山岳州"), ("ka", "საუტერნ ჰაილენდსი"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಹೈಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "서던하일랜즈 주"), ("lt", "Pietinis Kalnų regionas"), ("lv", "Dienvidu kalnienes province"), ("mr", "साऊथर\u{94d}न हाईल\u{901}ड\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Southern Highlands Province"), ("nb", "Sør høylands provins"), ("nl", "Southern Highlands"), ("no", "Sør høylands provins"), ("pl", "Southern Highlands"), ("pt", "Southern Highlands"), ("ru", "Саутерн-Хайлендс"), ("si", "දක\u{dd4}ණ\u{dd4} හය\u{dd2}ලන\u{dca}ඩ\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Southern Highlands"), ("ta", "தெற\u{bcd}கு ஹிக\u{bcd}ஹ\u{bcd}ல\u{bbe}ன\u{bcd}ட\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ హ\u{c48}ల\u{c3e}ండ\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซาเท\u{e34}ร\u{e4c}นไฮแลนดส\u{e4c}"), ("tr", "Southern Highlands"), ("uk", "Південний Гайлендс"), ("ur", "جنوبی سطح مرتفع صوبہ"), ("vi", "Tỉnh Phía Nam Highlands"), ("zh", "南高地省")]),
                        unofficial_name_list: ["Highlands South"].to_vec(),
                    }
                ),
                (
                    "WBK",
                    Subdivision{
                        name: "WBK",
                        country_alpha2: Alpha2::PG,
                        code: "WBK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.704743199999999), longitude: Some(150.0259466), max_latitude: Some(-4.542134), min_latitude: Some(-6.310895599999999), max_longitude: Some(151.6839161), min_longitude: Some(148.3099281)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غرب بريطانيا الجديدة"), ("bg", "Западна Нова Британия"), ("bn", "পশ\u{9cd}চিম নিউ ব\u{9cd}রিটেন প\u{9cd}রদেশ"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134} 𑄚\u{11131} 𑄝\u{11133}𑄢\u{11128}𑄑𑄬𑄚\u{11134}"), ("ceb", "West New Britain Province"), ("da", "West New Britain Province"), ("de", "West New Britain Province"), ("el", "Δυτική Νέα Βρετανία"), ("en", "West New Britain"), ("es", "Nueva Bretaña del Oeste"), ("et", "Lääne-Uus-Britannia"), ("eu", "Mendebaldeko Britainia Berria"), ("fa", "استان بریتانیای نو غربی"), ("fi", "Westnew Britainin provinssi"), ("fr", "Nouvelle-Bretagne occidentale"), ("gl", "Nova Bretaña Occidental"), ("gu", "પશ\u{acd}ચિમ ન\u{acd}ય\u{ac2} બ\u{acd}રિટન પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{947}स\u{94d}ट न\u{94d}य\u{942} ब\u{94d}रिट\u{947}न प\u{94d}रोवि\u{902}स"), ("id", "Provinsi Britania Baru Barat"), ("it", "provincia della Nuova Britannia Ovest"), ("ja", "西ニューブリテン州"), ("ka", "დასავლეთი ახალი ბრიტანეთი"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಟ\u{ccd} ನ\u{ccd}ಯ\u{cc2} ಬ\u{ccd}ರ\u{cbf}ಟನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "서뉴브리튼 주"), ("lt", "Rytų Naujosios Britanijos provincija"), ("lv", "Rietumu Jaunbritānijas province"), ("mk", "Западна Нова Британија"), ("mr", "पश\u{94d}चिम न\u{94d}य\u{942} ब\u{94d}रिटन प\u{94d}रा\u{902}त"), ("ms", "West New Britain Province"), ("nb", "Vest Nye Britiske provins"), ("nl", "West New Britain"), ("no", "Vest Nye Britiske provins"), ("pl", "Nowa Brytania Zachodnia"), ("pt", "Nova Bretanha Ocidental"), ("ru", "Западная Новая Британия"), ("si", "බටහ\u{dd2}ර නව බ\u{dca}\u{200d}ර\u{dd2}ත\u{dcf}න\u{dca}\u{200d}ය පළ\u{dcf}ත"), ("sv", "West New Britain"), ("ta", "மேற\u{bcd}கு நியூ பிரிட\u{bcd}டன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}స\u{c4d}ట\u{c4d} న\u{c4d}యూ బ\u{c4d}ర\u{c3f}టన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบร\u{e34}เท\u{e35}ยนตะว\u{e31}นตกใหม\u{e48}"), ("tr", "West New Britaion"), ("uk", "Західна Нова Британія"), ("ur", "مغربی نیا برطانیہ صوبہ"), ("vi", "Tỉnh Tây New Britain"), ("zh", "西新不列顛省")]),
                        unofficial_name_list: ["New Britain West"].to_vec(),
                    }
                ),
                (
                    "WHM",
                    Subdivision{
                        name: "WHM",
                        country_alpha2: Alpha2::PG,
                        code: "WHM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-5.6268128), longitude: Some(144.2593118), max_latitude: Some(-5.2128599), min_latitude: Some(-6.3646218), max_longitude: Some(144.5993042), min_longitude: Some(143.771268)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة الهضاب الغربية"), ("bg", "Западни височини"), ("bn", "ওয\u{9bc}েস\u{9cd}ট\u{9be}র\u{9cd}ন হ\u{9be}ইল\u{9cd}য\u{9be}ন\u{9cd}ড প\u{9cd}রদেশ"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬 𑄦\u{1112d}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Western Highlands Province"), ("da", "Western Highlands Province"), ("de", "Western Highlands Province"), ("el", "Δυτικά Υψίπεδα"), ("en", "Western Highlands"), ("es", "Tierras Altas Occidentales"), ("et", "Western Highlands"), ("eu", "Mendebaldeko Lur Garaiak"), ("fa", "استان ارتفاعات غربی"), ("fi", "Western Highlands - provinssi"), ("fr", "Hautes-Terres occidentales"), ("gl", "Western Highlands"), ("gu", "પશ\u{acd}ચિમી હાઈલ\u{ac7}ન\u{acd}ડ\u{acd}સ પ\u{acd}રા\u{a82}ત"), ("hi", "पश\u{94d}चिमी हाइल\u{948}\u{902}ड\u{94d}स प\u{94d}रा\u{902}त"), ("id", "Provinsi Dataran Tinggi Barat"), ("it", "provincia degli Altopiani Occidentali"), ("ja", "西部山岳州"), ("ka", "უესტერნ ჰაილენდსი"), ("kn", "ಪಶ\u{ccd}ಚ\u{cbf}ಮ ಹೈಲ\u{ccd}ಯಾಂಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "웨스턴하일랜즈 주"), ("lt", "Rytų Kalniečių provincija"), ("lv", "Rietumu kalnienes province"), ("mr", "पश\u{94d}चिम हाईल\u{901}ड\u{94d}स प\u{94d}रा\u{902}त"), ("ms", "Western Highlands Province"), ("nb", "Vest Høyland provins"), ("nl", "Western Highlands"), ("no", "Vest Høyland provins"), ("pl", "Western Highlands"), ("pt", "Western Highlands"), ("ru", "Уэстерн-Хайлендс"), ("si", "බටහ\u{dd2}ර හය\u{dd2}ලෑන\u{dca}ඩ\u{dca}ස\u{dca}"), ("sv", "Western Highlands"), ("ta", "மேற\u{bcd}கு ஹயிலன\u{bcd}ட\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పశ\u{c4d}చ\u{c3f}మ హ\u{c48}ల\u{c3e}ండ\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวสเท\u{e34}ร\u{e4c}น ไฮแลนด\u{e4c}"), ("tr", "Western Highlands Province"), ("uk", "Західний Гайлендс"), ("ur", "مغربی سطح مرتفع صوبہ"), ("vi", "Tỉnh Western Highlands"), ("zh", "西高地省")]),
                        unofficial_name_list: ["Highlands West"].to_vec(),
                    }
                ),
                (
                    "WPD",
                    Subdivision{
                        name: "WPD",
                        country_alpha2: Alpha2::PG,
                        code: "WPD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-7.584644), longitude: Some(142.3613378), max_latitude: Some(-5.000000099999999), min_latitude: Some(-9.3337498), max_longitude: Some(143.9240419), min_longitude: Some(140.842865)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("bg", "Западна провинция"), ("ccp", "𑄛\u{11127}𑄏\u{11128}𑄟\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Western Province (lalawigan sa Papua New Guinea)"), ("de", "Western Province"), ("en", "Western"), ("es", "Provincia Occidental"), ("et", "Lääneprovints"), ("eu", "Mendebaldeko probintzia"), ("fa", "استان غربی"), ("fi", "Western Province"), ("fr", "Province ouest"), ("gl", "Occidental, Papúa Nova Guinea"), ("id", "Provinsi Barat, Papua Nugini"), ("it", "provincia Occidentale"), ("ja", "西部州"), ("ka", "დასავლეთი პროვინცია"), ("ko", "서부 주"), ("nl", "Western"), ("pl", "Prowincja Zachodnia"), ("pt", "Província Ocidental"), ("ru", "Западная провинция"), ("sr", "Западна покрајина"), ("sr_Latn", "Zapadna pokrajina"), ("sv", "Western Province"), ("uk", "Західна провінція"), ("ur", "مغربی صوبہ"), ("zh", "西部省")]),
                        unofficial_name_list: ["Papua West", "Western"].to_vec(),
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
#[cfg(feature = "pg")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PG,
        alpha3: Alpha3::PNG,
        address_format: None,
        continent: Continent::Australia,
        country_code: 675,
        currency_code: "PGK",
        gec: Some(GEC::PP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "05",
        ioc: Some(IOC::PNG),
        iso_long_name: "The Independent State of Papua New Guinea",
        iso_short_name: "Papua New Guinea",
        official_language_list: ["en"].to_vec(),
        spoken_language_list: ["en"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        nationality: Some("Papua New Guinean"),
        number: "598",
        postal_code: true,
        postal_code_format: Some("\\d{3}"),
        region: Some(Region::Oceania),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::Melanesia),
        un_locode: "PG",
        unofficial_name_list: [
            "Papua New Guinea",
            "Papua-Neuguinea",
            "Papouasie Nouvelle-Guinée",
            "Papúa Nueva Guinea",
            "パプアニューギニア",
            "Papoea-Nieuw-Guinea",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Papua New Guinea"),
            ("af", "Papoea-Nieu-Guinee"),
            ("ak", "Papua New Guinea"),
            ("am", "ፓፑፂ ኒፄ \u{1311}ኒ"),
            ("an", "Papua New Guinea"),
            ("ar", "بابوا غينيا الجديدة"),
            ("as", "প\u{9be}প\u{9c1}ৱ\u{9be} নিউ গিনি"),
            ("ay", "Papua New Guinea"),
            ("az", "Papua Yeni Qvineya"),
            ("ba", "Papua New Guinea"),
            ("be", "Папуа — Новая Гвінея"),
            ("bg", "Папуа Нова Гвинея"),
            ("bi", "Papua New Guinea"),
            ("bn", "প\u{9be}প\u{9c1}য়\u{9be} নিউ গিনি"),
            ("bn_IN", "প\u{9be}প\u{9c1}য়\u{9be} নিউ গিনি"),
            ("br", "Papoua Ginea-Nevez"),
            ("bs", "Papua Nova Gvineja"),
            ("ca", "Papua Nova Guinea"),
            ("ce", "Папуа — Керла Гвине"),
            ("ch", "Papua New Guinea"),
            ("cs", "Papua Nová Guinea"),
            ("cv", "Папуа — Керла Гвине"),
            ("cy", "Papua Guinea Newydd"),
            ("da", "Papua Ny Guinea"),
            ("de", "Papua-Neuguinea"),
            (
                "dv",
                "ޕ\u{7a6}ޕ\u{7aa}އ\u{7a7} ނ\u{7a8}އ\u{7aa} ގ\u{7a8}ނ\u{7a9}",
            ),
            ("dz", "པ་པ\u{f74}་འ་ ན\u{f72}འ\u{f74}་ག\u{f72}་ན\u{f72}།"),
            ("ee", "Papua New Guinea"),
            ("el", "Παπούα Νέα Γουινέα"),
            ("en", "Papua New Guinea"),
            ("eo", "Papuo-Nov-Gvineo"),
            ("es", "Papúa Nueva Guinea"),
            ("et", "Paapua Uus-Guinea"),
            ("eu", "Papua Ginea Berria"),
            ("fa", "پاپوا گینه\u{654} نو"),
            ("ff", "Papua New Guinea"),
            ("fi", "Papua-Uusi-Guinea"),
            ("fo", "Papua Nýguinea"),
            ("fr", "Papouasie-Nouvelle-Guinée"),
            ("fy", "Papoea Nij-Guinea"),
            ("ga", "Papua Nua-Ghuine"),
            ("gl", "Papúa Nova Guinea"),
            ("gn", "Papua New Guinea"),
            ("gu", "પાપ\u{ac1}આ ન\u{acd}ય\u{ac1} ગીની"),
            ("gv", "Papooey Guinea Noa"),
            ("ha", "Papua New Guinea"),
            ("he", "פפואה גינאה החדשה"),
            ("hi", "पाप\u{941}आ न\u{94d}य\u{942} गिनी"),
            ("hr", "Papua Nova Gvineja"),
            ("ht", "Papwazi-Nouvèl-Gine"),
            ("hu", "Pápua Új-Guinea"),
            ("hy", "Պապուա Նոր Գվինեա"),
            ("ia", "Papua Nove Guinea"),
            ("id", "Papua Nugini"),
            ("io", "Papua-Nova-Guinea"),
            ("is", "Papúa Nýja-Gínea"),
            ("it", "Papua Nuova Guinea"),
            ("iu", "Papua New Guinea"),
            ("ja", "パプアニューギニア"),
            ("ka", "პაპუა-ახალი გვინეა"),
            ("ki", "Papua New Guinea"),
            ("kk", "Папуа - Жаңа Гвинея"),
            ("kl", "Papua New Guinea"),
            ("km", "ប\u{17c9}ាព\u{17bc}ញ\u{17bc}វហ\u{17d2}គ\u{17b8}ណេ"),
            ("kn", "ಪಪುವಾ ನ\u{ccd}ಯ\u{cc2} ಗ\u{cbf}ನ\u{cbf}"),
            ("ko", "파푸아뉴기니"),
            ("ku", "Papûa Gîneya Nû"),
            ("kv", "Papua New Guinea"),
            ("kw", "Papua New Guinea"),
            ("ky", "Папуа-Жаӊы Гвинея"),
            ("lo", "ປະເທດປາປ\u{ebb}ວຊ\u{eb5}ນ\u{eb9}ແວນກ\u{eb5}ເນ"),
            ("lt", "Papua Naujoji Gvinėja"),
            ("lv", "Papua-Jaungvineja"),
            ("mi", "Papua Nūkini"),
            ("mk", "Папуа Нова Гвинеја"),
            ("ml", "പ\u{d3e}പ\u{d41}വ ന\u{d4d}യ\u{d42} ഗിനിയ"),
            ("mn", "Папуа Шинэ Гвиней"),
            ("mr", "पाप\u{942} न\u{94d}य\u{942} गिनी"),
            ("ms", "Papua New Guinea"),
            ("mt", "Papwa-Ginea Ġdida"),
            (
                "my",
                "ပါပ\u{1030}အာနယ\u{1030}းဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Papua New Guinea"),
            ("nb", "Papua Ny-Guinea"),
            ("ne", "पप\u{941}वा न\u{94d}य\u{942} जिनिया"),
            ("nl", "Papoea-Nieuw-Guinea"),
            ("nn", "Papua Ny-Guinea"),
            ("nv", "Chooyééł Dineʼé Bikéyah"),
            ("oc", "Papoa Nòva Guinèa"),
            ("or", "ପ\u{b3e}ପ\u{b41}ଆ ନ\u{b4d}ଯ\u{b41} ଗ\u{b3f}ନୀ"),
            ("pa", "ਪਾਪ\u{a42}ਆ ਨਵਾ\u{a02} ਗ\u{a42}ਈਨਿਆ"),
            ("pi", "पप\u{941}वा न\u{94d}य\u{942} गिनी"),
            ("pl", "Papua-Nowa Gwinea"),
            ("ps", "Papua New Guinea"),
            ("pt", "Papua Nova Guiné"),
            ("pt_BR", "Papua-Nova Guiné"),
            ("ro", "Papua Noua Guinee"),
            ("ru", "Папуа — Новая Гвинея"),
            ("rw", "Papuwa Nuveli Gineya"),
            ("sc", "Pàpua Guinea Noa"),
            ("sd", "Papua New Guinea"),
            (
                "si",
                "පැප\u{dd4}ව\u{dcf} න\u{dd2}ව\u{dca}ග\u{dd2}න\u{dd2}ය\u{dcf}ව",
            ),
            ("sk", "Papua - Nová Guinea"),
            ("sl", "Papua Nova Gvineja"),
            ("so", "Papua New Guinea"),
            ("sq", "Guinea e Re Papua"),
            ("sr", "Папуа Нова Гвинеја"),
            ("sv", "Papua Nya Guinea"),
            ("sw", "Papua New Guinea"),
            ("ta", "ப\u{bbe}ப\u{bcd}புவ\u{bbe}-நியூகினி"),
            ("te", "ప\u{c3e}పూ న\u{c4d}యూ గ\u{c3f}న\u{c40}"),
            ("tg", "Папуа Гвинеяи Нав"),
            ("th", "ปาป\u{e31}วน\u{e34}วก\u{e34}น\u{e35}"),
            ("ti", "ፓፑዋ ኒው ጊኒ"),
            ("tk", "Papua-Täze Gwineýa"),
            ("tl", "Papua New Guinea"),
            ("tr", "Papua Yeni Gine"),
            ("tt", "Папуа Яңа Gуинеа"),
            ("ug", "پاپۇئا يېڭى گىۋىنېيەسى"),
            ("uk", "Папуа Нова Гвінея"),
            ("ur", "پاپوا نیو گنی"),
            ("uz", "Papua Yangi Gvineya"),
            ("ve", "Papua New Guinea"),
            ("vi", "Pa-pu-a Niu Ghi-nê"),
            ("wa", "Papouwazeye Nouve Guinêye"),
            ("wo", "Papwa Nuweel Ginne"),
            ("xh", "Papua New Guinea"),
            ("yo", "Papua Guinea Titun"),
            ("zh_CN", "巴布亚新几内亚"),
            ("zh_HK", "巴布亞新畿內亞"),
            ("zh_TW", "巴布亞紐幾內亞"),
            ("zu", "Papua New Guinea"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
