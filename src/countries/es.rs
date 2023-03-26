// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Spain

#[cfg(all(feature = "es", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::ES;
    pub const ALPHA3: Alpha3 = Alpha3::ESP;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 34;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::SP);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::ESP);
    pub const ISO_SHORT_NAME: &str = "Spain";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Spain";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Spanish");
    pub const NUMBER: &str = "724";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "ES";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Spain",
        "Spanien",
        "Espagne",
        "España",
        "スペイン",
        "Spanje",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Spain"),
        ("af", "Spanje"),
        ("ak", "Spain"),
        ("am", "ስፔን"),
        ("an", "Spain"),
        ("ar", "إسبانيا"),
        ("as", "স\u{9cd}পেইন"),
        ("ay", "Spain"),
        ("az", "İspaniya"),
        ("ba", "Spain"),
        ("be", "Іспанія"),
        ("bg", "Испания"),
        ("bi", "Spain"),
        ("bn", "স\u{9cd}পেন"),
        ("bn_IN", "স\u{9cd}পেন"),
        ("br", "Spagn"),
        ("bs", "Španija"),
        ("ca", "Espanya"),
        ("ce", "Испани"),
        ("ch", "España"),
        ("cs", "Španělsko"),
        ("cv", "Испани"),
        ("cy", "Sbaen"),
        ("da", "Spanien"),
        ("de", "Spanien"),
        ("dv", "އ\u{7a8}ސ\u{7b0}ޕ\u{7ac}އ\u{7a8}ނ\u{7b0}"),
        ("dz", "ས\u{f72}པ\u{f7a}ན།"),
        ("ee", "Spain"),
        ("el", "Ισπανία"),
        ("en", "Spain"),
        ("eo", "Hispanio"),
        ("es", "España"),
        ("et", "Hispaania"),
        ("eu", "Espainia"),
        ("fa", "اسپانیا"),
        ("ff", "Hispaanya"),
        ("fi", "Espanja"),
        ("fo", "Spania"),
        ("fr", "Espagne"),
        ("fy", "Spanje"),
        ("ga", "An Spáinn"),
        ("gl", "España"),
        ("gn", "Spain"),
        ("gu", "સ\u{acd}પ\u{ac7}ન"),
        ("gv", "Yn Spaainey"),
        ("ha", "An Spàinn"),
        ("he", "ספרד"),
        ("hi", "स\u{94d}प\u{947}न"),
        ("hr", "Španjolska"),
        ("ht", "Espay"),
        ("hu", "Spanyolország"),
        ("hy", "Իսպանիա"),
        ("ia", "Espania"),
        ("id", "Spanyol"),
        ("io", "Hispania"),
        ("is", "Spánn"),
        ("it", "Spagna"),
        ("iu", "ᓯᐸᐃᓐ"),
        ("ja", "スペイン"),
        ("ka", "ესპანეთი"),
        ("ki", "Spain"),
        ("kk", "Испания"),
        ("kl", "Spain"),
        ("km", "អេស\u{17d2}ប\u{17c9}ាញ"),
        ("kn", "ಸ\u{ccd}ಪೇನ\u{ccd}"),
        ("ko", "스페인"),
        ("ku", "Spanya"),
        ("kv", "Испания"),
        ("kw", "Spayn"),
        ("ky", "Испания"),
        ("lo", "ປະເທດແອສະປາຍ"),
        ("lt", "Ispanija"),
        ("lv", "Spānija"),
        ("mi", "Pāniora"),
        ("mk", "Шпанија"),
        ("ml", "സ\u{d4d}പെയിന\u{d4d}\u{200d}"),
        ("mn", "Испани"),
        ("mr", "स\u{94d}प\u{947}न"),
        ("ms", "Sepanyol"),
        ("mt", "Spanja"),
        (
            "my",
            "စပ\u{102d}န\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Pain"),
        ("nb", "Spania"),
        ("ne", "स\u{94d}प\u{947}न"),
        ("nl", "Spanje"),
        ("nn", "Spania"),
        ("nv", "Dibé Diníí Bikéyah"),
        ("oc", "Espanha"),
        ("or", "ସ\u{b4d}ପେନ"),
        ("pa", "ਸਪ\u{a47}ਨ"),
        ("pi", "स\u{94d}प\u{947}न"),
        ("pl", "Hiszpania"),
        ("ps", "هسپانیه"),
        ("pt", "Espanha"),
        ("pt_BR", "Espanha"),
        ("ro", "Spania"),
        ("ru", "Испания"),
        ("rw", "Esipanye"),
        ("sc", "Ispagna"),
        ("sd", "اسپين"),
        ("si", "ස\u{dca}ප\u{dcf}ඤ\u{dca}ඤය"),
        ("sk", "Španielsko"),
        ("sl", "Španija"),
        ("so", "Isbeyn"),
        ("sq", "Spanjë"),
        ("sr", "Шпанија"),
        ("sv", "Spanien"),
        ("sw", "Spain"),
        ("ta", "ஸ\u{bcd}பெயின\u{bcd}"),
        ("te", "స\u{c4d}ప\u{c46}య\u{c3f}న\u{c4d}"),
        ("tg", "Испания"),
        ("th", "สเปน"),
        ("ti", "ስጳኛ"),
        ("tk", "Ispaniýa"),
        ("tl", "Espanya"),
        ("tr", "İspanya"),
        ("tt", "İспаниа"),
        ("ug", "ئىسپانىيە"),
        ("uk", "Іспанія"),
        ("ur", "ہسپانیہ"),
        ("uz", "Ispaniya"),
        ("ve", "Spain"),
        ("vi", "Tây Ban Nha"),
        ("wa", "Espagne"),
        ("wo", "Ispaañ"),
        ("xh", "Spain"),
        ("yo", "Spéìn"),
        ("zh_CN", "西班牙"),
        ("zh_HK", "西班牙"),
        ("zh_TW", "西班牙"),
        ("zu", "ISpeyini"),
    ];
    #[cfg(all(feature = "es", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 40.46366700000001;
        pub const LONGITUDE: f64 = -3.74922;
        pub const MAX_LATITUDE: f64 = 43.8504;
        pub const MAX_LONGITUDE: f64 = 4.6362;
        pub const MIN_LATITUDE: f64 = 27.4985;
        pub const MIN_LONGITUDE: f64 = -18.2648001;
        pub const NORTHEAST_LATITUDE: f64 = 43.8504;
        pub const NORTHEAST_LONGITUDE: f64 = 4.6362;
        pub const SOUTHWEST_LATITUDE: f64 = 27.4985;
        pub const SOUTHWEST_LONGITUDE: f64 = -18.2648001;
    }
}
#[cfg(all(feature = "es", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 40.46366700000001,
            longitude: -3.74922,
            max_latitude: 43.8504,
            max_longitude: 4.6362,
            min_latitude: 27.4985,
            min_longitude: -18.2648001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 43.8504,
                    longitude: 4.6362,
                },
                southwest: CountryGeoBound {
                    latitude: 27.4985,
                    longitude: -18.2648001,
                },
            },
        }
    }
}

#[cfg(all(feature = "es", feature = "subdivisions"))]
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
                    "A",
                    Subdivision{
                        name: "A",
                        country_alpha2: Alpha2::ES,
                        code: "A",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.3459963), longitude: Some(-0.4906855), max_latitude: Some(38.3909328), min_latitude: Some(38.32474879999999), max_longitude: Some(-0.4034191), min_longitude: Some(-0.5416291999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أليكانتي"), ("az", "Alikante vilayəti"), ("be", "правінцыя Алікантэ"), ("bg", "Аликанте"), ("bn", "আলিক\u{9be}ন\u{9cd}তে প\u{9cd}রদেশ"), ("ca", "província d’Alacant"), ("ccp", "𑄃𑄣\u{11128}𑄇\u{11133}𑄠𑄚\u{11134}𑄑𑄬"), ("ceb", "Provincia de Alicante"), ("cs", "Provincie Alicante"), ("da", "Alicante"), ("de", "Provinz Alicante"), ("el", "Επαρχία του Αλικάντε"), ("en", "Alicante"), ("es", "Provincia de Alicante"), ("et", "Alicante provints"), ("eu", "Alacanteko probintzia"), ("fa", "استان آلیکانته"), ("fi", "Alicante"), ("fr", "province d’Alicante"), ("gl", "Provincia de Alacant"), ("gu", "એલિક\u{ac7}ન\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "एलिसाण\u{94d}ट प\u{94d}रान\u{94d}त"), ("hr", "Alicante"), ("hu", "Alicante tartomány"), ("hy", "Ալիկանտե"), ("id", "Provinsi Alicante"), ("it", "provincia di Alicante"), ("ja", "アリカンテ県"), ("ka", "ალიკანტეს პროვინცია"), ("kk", "Аликанте"), ("kn", "ಅಲ\u{cbf}ಸ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알리칸테 주"), ("lt", "Alikantės provincija"), ("lv", "Alakantas province"), ("mk", "Аликанте"), ("mr", "अ\u{945}लिक\u{947}\u{902}ट\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Alicante"), ("nb", "Alicante"), ("nl", "Alicante"), ("no", "Alicante"), ("pl", "Alicante"), ("pt", "Alicante"), ("ro", "Provincia Alicante"), ("ru", "Аликанте"), ("si", "අල\u{dd2}කන\u{dca}ටේ පළ\u{dcf}ත"), ("sl", "Alicante"), ("sq", "Provinca Alicante"), ("sr", "Покрајина Аликанте"), ("sr_Latn", "Pokrajina Alikante"), ("sv", "Alicante"), ("sw", "Mkoa wa Alicante"), ("ta", "அலிக\u{bbe}ண\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అల\u{c3f}క\u{c3e}ంట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เขตอโมลาทา"), ("tr", "Alicante ili"), ("uk", "Аліканте"), ("ur", "صوبہ الیکانتے"), ("uz", "Alicante"), ("vi", "Alicante"), ("yue", "阿利坎特省"), ("yue_Hans", "阿利坎特省"), ("zh", "阿利坎特省")]),
                        unofficial_name_list: ["Alacant", "Alacant/Alicante", "Alicante", "Alicante/Alacant"].to_vec(),
                    }
                ),
                (
                    "AB",
                    Subdivision{
                        name: "AB",
                        country_alpha2: Alpha2::ES,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.994349), longitude: Some(-1.8585424), max_latitude: Some(39.0129156), min_latitude: Some(38.9714828), max_longitude: Some(-1.834899), min_longitude: Some(-1.8862798)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "البسيط"), ("az", "Albasete vilayəti"), ("be", "Правінцыя Альбасетэ"), ("bg", "Албасете"), ("bn", "আল\u{9cd}ব\u{9be}সেটে প\u{9cd}রদেশ"), ("ca", "Província d’Albacete"), ("ccp", "𑄃𑄣\u{11134}𑄝𑄥𑄬𑄖\u{11134}"), ("ceb", "Provincia de Albacete"), ("cs", "Provincie Albacete"), ("da", "Albacete"), ("de", "Provinz Albacete"), ("el", "Αλμπαθέτε"), ("en", "Albacete"), ("es", "Provincia de Albacete"), ("et", "Albacete provints"), ("eu", "Albaceteko probintzia"), ("fa", "استان آلباسته"), ("fi", "Albacete"), ("fr", "province d’Albacete"), ("gl", "Provincia de Albacete"), ("gu", "અલ\u{acd}બાસ\u{ac7}ટ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("he", "מחוז אלבסטה"), ("hi", "आल\u{94d}बास\u{947}त\u{947} प\u{94d}रान\u{94d}त"), ("hr", "Albacete"), ("hu", "Albacete"), ("hy", "Ալբասետե"), ("id", "Provinsi Albacete"), ("it", "Provincia di Albacete"), ("ja", "アルバセテ県"), ("ka", "ალბასეტეს პროვინცია"), ("kk", "Альбасете"), ("kn", "ಅಲ\u{ccd}ಬ\u{cc6}ಸ\u{cc6}ಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알바세테 주"), ("lt", "Albasetės provincija"), ("lv", "Albasetes province"), ("mr", "अल\u{94d}बासिट\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Albacete"), ("nb", "Albacete"), ("nl", "Albacete"), ("no", "Albacete"), ("pl", "Albacete"), ("pt", "Albacete"), ("ro", "Provincia Albacete"), ("ru", "Альбасете"), ("si", "අල\u{dca}බසේටේ පළ\u{dcf}ත"), ("sq", "Provinca Albacete"), ("sr", "Покрајина Албасете"), ("sr_Latn", "Pokrajina Albasete"), ("sv", "Albacete"), ("ta", "அல\u{bcd}ப\u{bbe}ஸ\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆల\u{c4d}బస\u{c40}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ลเบอเซต\u{e35}"), ("tr", "Albacete ili"), ("uk", "Альбасете"), ("ur", "صوبہ الباسیتے"), ("uz", "Albacete"), ("vi", "Albacete"), ("zh", "阿爾瓦塞特省")]),
                        unofficial_name_list: ["Albacete"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "AL",
                        country_alpha2: Alpha2::ES,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.834047), longitude: Some(-2.4637136), max_latitude: Some(36.8658532), min_latitude: Some(36.817203), max_longitude: Some(-2.4286886), min_longitude: Some(-2.4855446)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "المرية"), ("az", "Almeriya vilayəti"), ("be", "Правінцыя Альмерыя"), ("bg", "Алмерия"), ("bn", "আল\u{9cd}মেরিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província d’Almeria"), ("ccp", "𑄃𑄣\u{11134}𑄟𑄬𑄢\u{11128}𑄠"), ("ceb", "Provincia de Almería"), ("cs", "Provincie Almería"), ("da", "Almería"), ("de", "Provinz Almería"), ("el", "Αλμερία"), ("en", "Almería"), ("es", "Provincia de Almería"), ("et", "Almería provints"), ("eu", "Almeríako probintzia"), ("fa", "استان آلمریا"), ("fi", "Almería"), ("fr", "province d’Almería"), ("gl", "Provincia de Almería"), ("gu", "અલમ\u{ac7}રીયા પ\u{acd}રા\u{a82}ત"), ("hi", "आल\u{94d}म\u{947}रिया प\u{94d}रान\u{94d}त"), ("hr", "Almería"), ("hu", "Almería"), ("hy", "Ալմերիա"), ("id", "Provinsi Almería"), ("it", "Provincia di Almería"), ("ja", "アルメリア県"), ("ka", "ალმერიის პროვინცია"), ("kk", "Альмерия"), ("kn", "ಅಲ\u{ccd}ಮ\u{cc6}ರ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "알메리아 주"), ("lt", "Almerijos provincija"), ("lv", "Almerijas province"), ("mk", "Алмерија"), ("mr", "अल\u{94d}म\u{947}रिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Almería"), ("nb", "Almería"), ("nl", "Almería"), ("no", "Almería"), ("pl", "Almería"), ("pt", "Almeria"), ("ro", "Provincia Almería"), ("ru", "Альмерия"), ("si", "අල\u{dca}මෙර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sl", "Almeria (pokrajina)"), ("sq", "Provinca Almería"), ("sr", "Провинција Алмерија"), ("sr_Latn", "Provincija Almerija"), ("sv", "Almería"), ("sw", "Mkoa wa Almería"), ("ta", "அல\u{bcd}மேரிஆ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఆల\u{c4d}మ\u{c47}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e31}ลม\u{e35}เร\u{e35}ย"), ("tr", "Almería ili"), ("uk", "Альмерія"), ("ur", "صوبہ المریہ"), ("uz", "Almería"), ("vi", "Almería"), ("yue", "阿爾梅里亞省"), ("yue_Hans", "阿尔梅里亚省"), ("zh", "阿爾梅里亞省")]),
                        unofficial_name_list: ["Almería"].to_vec(),
                    }
                ),
                (
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::ES,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Andalusië"), ("am", "አንዳሉሲያ"), ("ar", "منطقة أندلوسيا"), ("az", "Andalusiya"), ("be", "Андалусія"), ("bg", "Андалусия"), ("bn", "আন\u{9cd}দ\u{9be}ল\u{9c1}সিয\u{9bc}\u{9be}"), ("bs", "Andaluzija"), ("ca", "Andalusia"), ("ccp", "𑄃𑄚\u{11134}𑄓𑄣\u{11134}𑄣\u{1112d}\u{1112a}𑄥\u{11128}𑄠"), ("ceb", "Andalucía"), ("cs", "Andalusie"), ("cy", "Andalucía"), ("da", "Andalusien"), ("de", "Autonome Gemeinschaft Andalusien"), ("el", "Ανδαλουσία"), ("en", "Andalusia"), ("es", "Andalucía"), ("et", "Andaluusia"), ("eu", "Andaluzia"), ("fa", "اندلس"), ("fi", "Andalusia"), ("fr", "Andalousie"), ("ga", "An Andalúis"), ("gl", "Andalucía"), ("he", "אנדלוסיה"), ("hi", "आ\u{902}दाल\u{941}सिया"), ("hr", "Andaluzija"), ("hu", "Andalúzia"), ("hy", "Անդալուզիա"), ("id", "Andalusia"), ("is", "Andalúsía"), ("it", "Andalusia"), ("ja", "アンダルシア州"), ("jv", "Andalusia"), ("ka", "ანდალუსია"), ("kk", "Андалусия"), ("ko", "안달루시아 지방"), ("ky", "Андалусия"), ("lt", "Andalūzija"), ("lv", "Andalūzija"), ("mk", "Андалузија"), ("ml", "ആൻഡല\u{d42}ഷ\u{d4d}യ"), ("mn", "Андалус орон"), ("mr", "आ\u{902}दाल\u{941}सिया"), ("ms", "Andalusia"), ("nb", "Andalucía"), ("nl", "Andalusië"), ("no", "Andalucía"), ("pl", "Andaluzja"), ("pt", "Andaluzia"), ("ro", "Andaluzia"), ("ru", "Андалусия"), ("sk", "Andalúzia"), ("sl", "Andaluzija"), ("sq", "Andalusia"), ("sr", "Андалузија"), ("sr_Latn", "Andaluzija"), ("sv", "Andalusien"), ("sw", "Andalusia"), ("ta", "அந\u{bcd}த\u{bbe}லூசிய\u{bbe}"), ("th", "แคว\u{e49}นอ\u{e31}นดาล\u{e39}ซ\u{e35}อา"), ("tr", "Endülüs"), ("uk", "Андалусія"), ("ur", "اندلوسیا"), ("uz", "Andalusiya"), ("vi", "Andalusia"), ("yue", "安達盧西亞"), ("yue_Hans", "安达卢西亚"), ("zh", "安達魯西亞")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "AR",
                    Subdivision{
                        name: "AR",
                        country_alpha2: Alpha2::ES,
                        code: "AR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "አራጎን"), ("ar", "منطقة أراغـون"), ("az", "Araqon"), ("be", "Арагон"), ("bg", "Арагон"), ("bs", "Aragon"), ("ca", "Aragó"), ("ccp", "𑄃𑄢𑄉\u{11127}𑄚\u{11134}"), ("ceb", "Aragon"), ("cs", "Aragonie"), ("cy", "Aragón"), ("da", "Aragonien"), ("de", "Aragonien"), ("el", "Αραγονία"), ("en", "Aragon"), ("es", "Aragón"), ("et", "Aragón"), ("eu", "Aragoi"), ("fa", "آراگون"), ("fi", "Aragonia"), ("fr", "Aragon"), ("ga", "An Aragóin"), ("gl", "Aragón"), ("he", "אראגון"), ("hi", "आरागोन"), ("hr", "Aragonija"), ("hu", "Aragónia"), ("hy", "Արագոն"), ("id", "Aragon"), ("is", "Aragon"), ("it", "Aragona"), ("ja", "アラゴン州"), ("jv", "Aragon"), ("ka", "არაგონი"), ("kk", "Арагон"), ("ko", "아라곤 지방"), ("lt", "Aragonas"), ("lv", "Aragona"), ("mk", "Арагон"), ("mn", "Арагон орон"), ("mr", "आरागोन"), ("ms", "Aragon"), ("nb", "Aragon"), ("nl", "Aragón"), ("no", "Aragon"), ("pl", "Aragonia"), ("pt", "Aragão"), ("ro", "Aragon"), ("ru", "Арагон"), ("sk", "Aragónsko"), ("sl", "Aragonija"), ("sq", "Aragon"), ("sr", "Арагон"), ("sr_Latn", "Aragon"), ("sv", "Aragonien"), ("ta", "அரகொன\u{bcd}"), ("th", "แคว\u{e49}นอารากอง"), ("tr", "Aragon"), ("uk", "Арагон"), ("ur", "اراغون"), ("uz", "Aragon"), ("vi", "Aragon"), ("yue", "亞拉岡"), ("yue_Hans", "亚拉冈"), ("zh", "阿拉贡自治区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "AS",
                    Subdivision{
                        name: "AS",
                        country_alpha2: Alpha2::ES,
                        code: "AS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Asturië"), ("am", "አስቱርያስ"), ("ar", "منطقة أستورياس"), ("az", "Asturiya"), ("be", "Астурыя"), ("bg", "Астурия"), ("bn", "আস\u{9cd}ত\u{9c1}রিয\u{9bc}\u{9be}স"), ("bs", "Asturija"), ("ca", "Astúries"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑𑄢\u{11128}𑄠𑄌\u{11134}"), ("ceb", "Principality of Asturias"), ("cs", "Asturie"), ("cy", "Asturias"), ("da", "Asturien"), ("de", "Asturien"), ("el", "Αστούριες"), ("en", "Asturias"), ("es", "Principado de Asturias"), ("et", "Astuuria"), ("eu", "Asturiesko Printzerria"), ("fa", "آستوریاس"), ("fi", "Asturia"), ("fr", "Asturies"), ("ga", "Asturies"), ("gl", "Principado de Asturias"), ("gu", "અસ\u{acd}ટ\u{ac1}રિયાસ"), ("he", "אסטוריאס"), ("hi", "आस\u{94d}त\u{941}रियास"), ("hr", "Asturija"), ("hu", "Asztúria"), ("hy", "Աստուրիա"), ("id", "Asturias"), ("is", "Astúría"), ("it", "Asturie"), ("ja", "アストゥリアス州"), ("jv", "Asturias"), ("ka", "ასტურია"), ("kk", "Астурия"), ("kn", "ಆಸ\u{ccd}ಟ\u{cc2}ರ\u{cbf}ಯಸ\u{ccd}"), ("ko", "아스투리아스 지방"), ("lt", "Astūrija"), ("lv", "Astūrija"), ("mk", "Астурија"), ("mn", "Астур муж"), ("mr", "आस\u{94d}त\u{941}रियास"), ("ms", "Asturias"), ("nb", "Asturias"), ("nl", "Asturië"), ("no", "Asturias"), ("pl", "Asturia"), ("pt", "Astúrias"), ("ro", "Asturia"), ("ru", "Астурия"), ("si", "අස\u{dca}ට\u{dd4}ර\u{dd2}යස\u{dca}"), ("sk", "Astúria"), ("sq", "Asturias"), ("sr", "Кнежевина Астурија"), ("sr_Latn", "Kneževina Asturija"), ("sv", "Asturien"), ("ta", "ஆதூரிய\u{bbe}"), ("te", "అస\u{c4d}టూర\u{c3f}య\u{c3e}స\u{c4d}"), ("th", "แคว\u{e49}นอ\u{e31}สต\u{e39}เร\u{e35}ยส"), ("tr", "Asturies"), ("uk", "Астурія"), ("ur", "استوریاس"), ("uz", "Asturiya"), ("vi", "Asturies"), ("yue", "阿斯圖里亞斯"), ("yue_Hans", "阿斯图里亚斯"), ("zh", "阿斯图里亚斯")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "AV",
                    Subdivision{
                        name: "AV",
                        country_alpha2: Alpha2::ES,
                        code: "AV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.656685), longitude: Some(-4.6812086), max_latitude: Some(40.6759493), min_latitude: Some(40.6378229), max_longitude: Some(-4.6566674), min_longitude: Some(-4.707720000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "آبلة"), ("az", "Avila vilayəti"), ("be", "Правінцыя Авіла"), ("bg", "Авила"), ("bn", "এভিল\u{9be} প\u{9cd}রদেশ"), ("ca", "província d’Àvila"), ("ccp", "𑄃𑄞\u{11128}𑄣"), ("ceb", "Provincia de Ávila"), ("cs", "Provincie Ávila"), ("da", "Ávila"), ("de", "Provinz Ávila"), ("el", "Αβίλα"), ("en", "Ávila"), ("es", "Provincia de Ávila"), ("et", "Ávila provints"), ("eu", "Ávilako probintzia"), ("fa", "استان آبیلا"), ("fi", "Ávila"), ("fr", "province d’Ávila"), ("gl", "Provincia de Ávila"), ("gu", "એવીલા પ\u{acd}રા\u{a82}ત"), ("hi", "एविला प\u{94d}रा\u{902}त"), ("hr", "Ávila"), ("hu", "Ávila"), ("id", "Provinsi Ávila"), ("it", "provincia di Ávila"), ("ja", "アビラ県"), ("ka", "ავილის პროვინცია"), ("kk", "Авила"), ("kn", "ಆವ\u{cbf}ಲಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "아빌라 주"), ("lt", "Avilos provincija"), ("lv", "Avilas province"), ("mk", "Авила"), ("mr", "एविला प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ávila"), ("nb", "Ávila"), ("nl", "Ávila"), ("no", "Ávila"), ("pl", "Ávila"), ("pt", "Ávila"), ("ro", "Provincia Ávila"), ("ru", "Авила"), ("si", "ආව\u{dd2}ල\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Ávila"), ("sr", "Покрајина Авила"), ("sr_Latn", "Pokrajina Avila"), ("sv", "Ávila"), ("sw", "Mkoa wa Ávila"), ("ta", "ஆவில\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అవ\u{c3f}ల\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาบ\u{e35}ลา"), ("tr", "Ávila ili"), ("uk", "Авіла"), ("ur", "صوبہ آبیلا"), ("uz", "Ávila"), ("vi", "Ávila"), ("zh", "阿维拉省")]),
                        unofficial_name_list: ["Ávila"].to_vec(),
                    }
                ),
                (
                    "B",
                    Subdivision{
                        name: "B",
                        country_alpha2: Alpha2::ES,
                        code: "B",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.3850639), longitude: Some(2.1734035), max_latitude: Some(41.4695761), min_latitude: Some(41.320004), max_longitude: Some(2.2280099), min_longitude: Some(2.0695258)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "برشلونة"), ("az", "Barselona vilayəti"), ("be", "правінцыя Барселона"), ("bn", "ব\u{9be}র\u{9cd}সেলোন\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Barcelona"), ("ccp", "𑄝𑄢\u{11134}𑄥𑄬𑄣\u{1112e}𑄚"), ("ceb", "Província de Barcelona"), ("cs", "Provincie Barcelona"), ("cy", "Talaith Barcelona"), ("da", "Barcelona"), ("de", "Provinz Barcelona"), ("el", "Επαρχία της Βαρκελώνης"), ("en", "Barcelona"), ("es", "Provincia de Barcelona"), ("et", "Barcelona provints"), ("eu", "Bartzelonako probintzia"), ("fa", "استان بارسلون"), ("fi", "Barcelona"), ("fr", "province de Barcelone"), ("gl", "Provincia de Barcelona"), ("gu", "બાર\u{acd}સિલોના પ\u{acd}રા\u{a82}ત"), ("he", "מחוז ברצלונה"), ("hi", "बार\u{94d}सीलोना प\u{94d}रा\u{902}त"), ("hr", "Barcelona"), ("hu", "Barcelona tartomány"), ("id", "Provinsi Barcelona"), ("it", "Provincia di Barcellona"), ("ja", "バルセロナ県"), ("ka", "ბარსელონის პროვინცია"), ("kk", "Барселона"), ("kn", "ಬಾರ\u{ccd}ಸ\u{cbf}ಲೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바르셀로나 주"), ("lt", "Barselonos provincija"), ("lv", "Barselonas province"), ("mr", "बार\u{94d}सिलोना प\u{94d}रा\u{902}त"), ("ms", "Barcelona Province"), ("nb", "Barcelona"), ("nl", "Barcelona"), ("no", "Barcelona"), ("pl", "Barcelona"), ("pt", "Barcelona"), ("ro", "Provincia Barcelona"), ("ru", "Барселона"), ("si", "බ\u{dcf}ස\u{dd2}ලෝන\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Barcelona"), ("sr", "Провинција Барселона"), ("sr_Latn", "Provincija Barselona"), ("sv", "Barcelona"), ("sw", "Mkoa wa Barcelona"), ("ta", "ப\u{bbe}ர\u{bcd}ஸலோன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ర\u{c4d}స\u{c3f}ల\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาร\u{e4c}เซโลนา"), ("tr", "Barselona ili"), ("uk", "провінція Барселона"), ("ur", "صوبہ برشلونہ"), ("uz", "Barselona"), ("vi", "Barcelona"), ("yue", "巴薩隆拿省"), ("yue_Hans", "巴萨隆拿省"), ("zh", "巴塞罗那省")]),
                        unofficial_name_list: ["Barcelona", "Barna"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::ES,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.8794495), longitude: Some(-6.9706535), max_latitude: Some(38.9069928), min_latitude: Some(38.8517673), max_longitude: Some(-6.9406957), min_longitude: Some(-7.0346625)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بطليوس"), ("az", "Badaxos vilayəti"), ("be", "Бадахас"), ("bn", "ব\u{9be}দ\u{9be}জোজ প\u{9cd}রদেশ"), ("ca", "Província de Badajoz"), ("ccp", "𑄝𑄖\u{11134}𑄎\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Badajoz"), ("cs", "Provincie Badajoz"), ("da", "Badajoz"), ("de", "Provinz Badajoz"), ("el", "Μπαδαχόθ"), ("en", "Badajoz"), ("es", "Provincia de Badajoz"), ("et", "Badajozi provints"), ("eu", "Badajozko probintzia"), ("fa", "استان باداخس"), ("fi", "Badajoz"), ("fr", "province de Badajoz"), ("ga", "Badajoz"), ("gl", "Provincia de Badaxoz"), ("gu", "બદાજોઝ પ\u{acd}રા\u{a82}ત"), ("hi", "बदाओज\u{93c} प\u{94d}रा\u{902}त"), ("hr", "Badajoz"), ("hu", "Badajoz tartomány"), ("hy", "Բադախոս"), ("id", "Provinsi Badajoz"), ("it", "Provincia di Badajoz"), ("ja", "バダホス県"), ("ka", "ბადახოსის პროვინცია"), ("kk", "Бадахос"), ("kn", "ಬಾಡೋಜೋಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바다호스 주"), ("lt", "Badachoso provincija"), ("lv", "Badahosas province"), ("mk", "Бадахоз"), ("mr", "बदाजोज प\u{94d}रा\u{902}त"), ("ms", "Wilayah Badajoz"), ("nb", "Badajoz"), ("nl", "Badajoz"), ("no", "Badajoz"), ("pl", "Badajoz"), ("pt", "Badajoz (província)"), ("ro", "Provincia Badajoz"), ("ru", "Бадахос"), ("si", "බඩජෝස\u{dca} පළ\u{dcf}ත"), ("sl", "Badajoz"), ("sq", "Provinca Badajoz"), ("sr", "Покрајина Бадахоз"), ("sr_Latn", "Pokrajina Badahoz"), ("sv", "Badajoz"), ("sw", "Mkoa wa Badajoz"), ("ta", "படஜோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బడ\u{c3e}జ\u{c4b}జ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบาดาจอซ"), ("tr", "Badajoz ili"), ("uk", "Бадахос"), ("ur", "صوبہ بطليوس"), ("uz", "Badajoz"), ("vi", "Badajoz"), ("yue", "巴達何斯省"), ("yue_Hans", "巴达何斯省"), ("zh", "巴达霍斯省")]),
                        unofficial_name_list: ["Badajoz"].to_vec(),
                    }
                ),
                (
                    "BI",
                    Subdivision{
                        name: "BI",
                        country_alpha2: Alpha2::ES,
                        code: "BI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.2204286), longitude: Some(-2.6983868), max_latitude: Some(43.4572451), min_latitude: Some(42.9818773), max_longitude: Some(-2.4127155), min_longitude: Some(-3.4492758)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Biskaje"), ("ar", "بيسكاي"), ("az", "Biskaya vilayəti"), ("be", "правінцыя Біская"), ("bg", "Бизкаиа"), ("bn", "বিসকে"), ("ca", "Biscaia"), ("ccp", "𑄝\u{11128}𑄌\u{11134}𑄇𑄬"), ("ceb", "Bizkaia"), ("cs", "Bizkaia"), ("cy", "Biskaia"), ("da", "Vizcaya"), ("de", "Bizkaia"), ("el", "Βισκαϊκή"), ("en", "Biscay"), ("es", "Vizcaya"), ("et", "Biskaia"), ("eu", "Bizkaia"), ("fa", "استان بیسکای"), ("fi", "Vizcaya"), ("fr", "Biscaye"), ("gl", "Biscaia - Bizkaia"), ("gu", "બિસ\u{acd}ક\u{ac7}"), ("he", "ביסקאיה"), ("hi", "बिस\u{94d}क\u{947}"), ("hr", "Biskaja"), ("hu", "Bizkaia"), ("id", "Vizcaya"), ("it", "Biscaglia"), ("ja", "ビスカヤ県"), ("ka", "ბისკაია"), ("kk", "Бискайя"), ("kn", "ಬ\u{cbf}ಸ\u{ccd}ಕ\u{cc6}"), ("ko", "비스카야 주"), ("ky", "Бискай"), ("lt", "Biskajos provincija"), ("lv", "Biskaja"), ("mr", "बिस\u{94d}क\u{947}"), ("ms", "Wilayah Biscay"), ("nb", "Bizkaia"), ("nl", "Biskaje"), ("no", "Bizkaia"), ("pl", "Vizcaya"), ("pt", "Biscaia"), ("ro", "Provincia Vizcaya"), ("ru", "Бискайя"), ("si", "බ\u{dd2}ස\u{dca}කේ"), ("sq", "Provinca Biscay"), ("sr", "Покрајина Бискаја"), ("sr_Latn", "Pokrajina Biskaja"), ("sv", "Biscaya"), ("sw", "Mkoa wa Vizcaya"), ("ta", "பிஸ\u{bcd}கேய\u{bcd}"), ("te", "బ\u{c3f}స\u{c4d}క\u{c47}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e34}ซกายา"), ("tr", "Biskay"), ("uk", "Біскайя"), ("ur", "بیسکای"), ("uz", "Vizcaya"), ("vi", "Biscay"), ("zh", "比斯開省")]),
                        unofficial_name_list: ["Bizkaia", "Vizcaya"].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::ES,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.3439925), longitude: Some(-3.696906), max_latitude: Some(42.3721341), min_latitude: Some(42.31611789999999), max_longitude: Some(-3.6367504), min_longitude: Some(-3.7526357)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "برغش"), ("az", "Burqos vilayəti"), ("be", "Правінцыя Бургас"), ("bg", "Бургос"), ("bn", "ব\u{9c1}র\u{9cd}গোস প\u{9cd}রদেশ"), ("ca", "província de Burgos"), ("ccp", "𑄝𑄢\u{11134}𑄉\u{1112e}𑄌\u{11134}"), ("ceb", "Provincia de Burgos"), ("cs", "Provincie Burgos"), ("da", "Burgos"), ("de", "Provinz Burgos"), ("el", "Επαρχία του Μπούργκος"), ("en", "Burgos"), ("es", "Provincia de Burgos"), ("et", "Burgose provints"), ("eu", "Burgosko probintzia"), ("fa", "استان بورگس"), ("fi", "Burgos"), ("fr", "province de Burgos"), ("gl", "Provincia de Burgos"), ("gu", "બર\u{acd}ગોસ પ\u{acd}રા\u{a82}ત"), ("hi", "बर\u{94d}गोस प\u{94d}रा\u{902}त"), ("hr", "Burgos"), ("hu", "Burgos"), ("id", "Provinsi Burgos"), ("it", "provincia di Burgos"), ("ja", "ブルゴス県"), ("ka", "ბურგოსის პროვინცია"), ("kk", "Бургос"), ("kn", "ಬರ\u{ccd}ಗೋಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "부르고스 주"), ("lt", "Burgoso provincija"), ("lv", "Burgosas province"), ("mr", "बर\u{94d}गोस प\u{94d}रा\u{902}त"), ("ms", "Burgos"), ("nb", "Burgos"), ("nl", "Burgos"), ("no", "Burgos"), ("pl", "Burgos"), ("pt", "Burgos"), ("ro", "Provincia Burgos"), ("ru", "Бургос"), ("si", "බර\u{dca}ගොස\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Burgos"), ("sr", "Покрајина Бургос"), ("sr_Latn", "Pokrajina Burgos"), ("sv", "Burgos"), ("sw", "Mkoa wa Burgos"), ("ta", "பர\u{bcd}கோஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బుర\u{c4d}గ\u{c4b}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "บ\u{e39}ร\u{e4c}โกส"), ("tr", "Burgos ili"), ("uk", "Бургос"), ("ur", "صوبہ بورگوس"), ("uz", "Burgos"), ("vi", "Burgos"), ("zh", "布尔戈斯省")]),
                        unofficial_name_list: ["Burgos"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::ES,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.3623436), longitude: Some(-8.4115401), max_latitude: Some(43.38640729999999), min_latitude: Some(43.337341), max_longitude: Some(-8.3871082), min_longitude: Some(-8.4382592)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لا كورونيا"), ("az", "La-Korunya vilayəti"), ("be", "Правінцыя Ла-Карунья"), ("bg", "Ла Коруня"), ("bn", "এ কর\u{9c1}ন\u{9be} প\u{9cd}রদেশ"), ("ca", "província de la Corunya"), ("ccp", "𑄃𑄬 𑄇\u{11127}𑄢\u{1112a}𑄚"), ("ceb", "Provincia da Coruña"), ("cs", "Provincie A Coruña"), ("da", "A Coruña"), ("de", "Provinz A Coruña"), ("el", "Α Κορούνια"), ("en", "A Coruña"), ("es", "Provincia de La Coruña"), ("et", "A Coruña provints"), ("eu", "Coruñako probintzia"), ("fa", "استان لاکرونیا"), ("fi", "A Coruña"), ("fr", "province de La Corogne"), ("ga", "A Coruña"), ("gl", "Provincia da Coruña"), ("gu", "આ કોર\u{ac1}ના પ\u{acd}રા\u{a82}ત"), ("hi", "ला कोर\u{942}निया प\u{94d}रान\u{94d}त"), ("hr", "A Coruña"), ("hu", "A Coruña"), ("hy", "Լա Կորունյա նահանգ"), ("id", "Provinsi A Coruña"), ("it", "Provincia della Coruña"), ("ja", "ア・コルーニャ県"), ("ka", "ლა-კორუნიის პროვინცია"), ("kk", "Ла-Корунья"), ("kn", "ಎ ಕೊರುನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라코루냐 주"), ("lt", "La Korunjos provincija"), ("lv", "Akoruņas province"), ("mr", "ला कोर\u{941}न\u{94d}या"), ("ms", "Wilayah A Coruña"), ("nb", "A Coruña"), ("nl", "A Coruña"), ("no", "A Coruña"), ("pl", "A Coruña"), ("pt", "Corunha"), ("ro", "Provincia La Coruña"), ("ru", "Ла-Корунья"), ("si", "ඒ කොර\u{dd4}න\u{dcf} පළ\u{dcf}ත"), ("sk", "Provincia La Coruña"), ("sq", "Provinca A Coruña"), ("sr", "Покрајина Коруња"), ("sr_Latn", "Pokrajina Korunja"), ("sv", "La Coruña"), ("sw", "Mkoa wa La Coruña"), ("ta", "கோரூஞ\u{bbe}"), ("te", "ఓ క\u{c4a}రూన\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอาโกร\u{e38}ญญา"), ("tr", "A Coruña ili"), ("uk", "А-Корунья"), ("ur", "صوبہ لا کورونا"), ("uz", "A Coruña"), ("vi", "A Coruña"), ("zh", "拉科鲁尼亚省")]),
                        unofficial_name_list: ["A Coruña", "Corunna", "Coruña", "Coruña, A", "La Coruña"].to_vec(),
                    }
                ),
                (
                    "CA",
                    Subdivision{
                        name: "CA",
                        country_alpha2: Alpha2::ES,
                        code: "CA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.5270612), longitude: Some(-6.2885962), max_latitude: Some(36.5431863), min_latitude: Some(36.4895662), max_longitude: Some(-6.255334599999999), min_longitude: Some(-6.309558)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قادس"), ("az", "Kadis vilayəti"), ("be", "правінцыя Кадыс"), ("bn", "ক\u{9be}দিস প\u{9cd}রদেশ"), ("ca", "província de Cadis"), ("ccp", "𑄇𑄓\u{11128}𑄌\u{11134}"), ("ceb", "Provincia de Cádiz"), ("cs", "Provincie Cádiz"), ("da", "Cádiz"), ("de", "Provinz Cádiz"), ("el", "Επαρχία Κάδιθ"), ("en", "Cádiz"), ("es", "Provincia de Cádiz"), ("et", "Cádizi provints"), ("eu", "Cádizko probintzia"), ("fa", "استان کادیس"), ("fi", "Cádiz"), ("fr", "province de Cadix"), ("gl", "Provincia de Cádiz"), ("gu", "કાડીઝ પ\u{acd}રા\u{a82}ત"), ("he", "מחוז קדיס"), ("hi", "क\u{948}डीज\u{93c} प\u{94d}रा\u{902}त"), ("hr", "Cádiz"), ("hu", "Cádiz"), ("hy", "Կադիս"), ("id", "Provinsi Cádiz"), ("it", "Provincia di Cadice"), ("ja", "カディス県"), ("ka", "კადისის პროვინცია"), ("kk", "Кадис"), ("kn", "ಕಾಡ\u{cbf}ಜ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카디스 주"), ("lt", "Kadiso provincija"), ("lv", "Kadisas province"), ("mk", "Кадиз"), ("mr", "काडिझ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cádiz"), ("nb", "Cádiz"), ("nl", "Cádiz"), ("no", "Cádiz"), ("pl", "Kadyks"), ("pt", "Cádis"), ("ro", "Provincia Cádiz"), ("ru", "Кадис"), ("si", "ක\u{dcf}ද\u{dd2}ස\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Cadiz"), ("sr", "Провинција Кадиз"), ("sr_Latn", "Provincija Kadiz"), ("sv", "Cádiz"), ("sw", "Mkoa wa Cádiz"), ("ta", "க\u{bbe}த\u{bc0}சு"), ("te", "క\u{c3e}డ\u{c3f}జ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกาด\u{e34}ซ"), ("tr", "Cádiz ili"), ("uk", "Кадіс"), ("ur", "صوبہ کادیز"), ("uz", "Cádiz"), ("vi", "Cádiz"), ("yue", "加的斯省"), ("yue_Hans", "加的斯省"), ("zh", "加的斯省")]),
                        unofficial_name_list: ["Cádiz"].to_vec(),
                    }
                ),
                (
                    "CB",
                    Subdivision{
                        name: "CB",
                        country_alpha2: Alpha2::ES,
                        code: "CB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ካንታብርያ"), ("ar", "منطقة كانتابريا"), ("az", "Kantabriya"), ("be", "Кантабрыя"), ("bg", "Кантабрия"), ("bn", "ক\u{9be}ন\u{9cd}ত\u{9be}ব\u{9cd}রিয\u{9bc}\u{9be}"), ("bs", "Kantabrija"), ("ca", "Cantàbria"), ("ccp", "𑄇\u{11133}𑄠𑄚\u{11134}𑄑𑄝\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Cantabria"), ("cs", "Kantábrie"), ("cy", "Cantabria"), ("da", "Kantabrien"), ("de", "Autonome Gemeinschaft Kantabrien"), ("el", "Κανταβρία"), ("en", "Cantabria"), ("es", "Cantabria"), ("et", "Kantaabria"), ("eu", "Kantabria"), ("fa", "کانتابریا"), ("fi", "Kantabria"), ("fr", "Cantabrie"), ("ga", "Cantabria"), ("gl", "Cantabria"), ("gu", "કાન\u{acd}તાબ\u{acd}રિયા"), ("he", "קנטבריה"), ("hi", "क\u{902}ट\u{948}ब\u{94d}रिया"), ("hr", "Kantabrija"), ("hu", "Kantábria"), ("hy", "Կանտաբրիա"), ("id", "Cantabria"), ("is", "Kantabría"), ("it", "Cantabria"), ("ja", "カンタブリア州"), ("ka", "კანტაბრია"), ("kk", "Кантабрия"), ("kn", "ಕ\u{ccd}ಯಾಂಥಬ\u{ccd}ರ\u{cbf}ಯಾ"), ("ko", "칸타브리아 지방"), ("lt", "Kantabrija"), ("lv", "Kantabrija"), ("mk", "Кантабрија"), ("mr", "का\u{902}ताब\u{94d}रिया"), ("ms", "Cantabria"), ("nb", "Cantabria"), ("nl", "Cantabrië"), ("no", "Cantabria"), ("pl", "Kantabria"), ("pt", "Cantábria"), ("ro", "Cantabria"), ("ru", "Кантабрия"), ("si", "කැන\u{dca}ටබ\u{dca}ර\u{dd2}ය\u{dcf}"), ("sk", "Kantábria"), ("sq", "Cantabria"), ("sr", "Кантабрија"), ("sr_Latn", "Kantabrija"), ("sv", "Kantabrien"), ("ta", "க\u{bbe}ந\u{bcd}த\u{bbe}பிரிய\u{bbe}"), ("te", "క\u{c3e}ంట\u{c3e}బ\u{c4d}ర\u{c3f}య\u{c3e}"), ("th", "แคว\u{e49}นก\u{e31}นตาเบร\u{e35}ย"), ("tr", "Kantabria"), ("uk", "Кантабрія"), ("ur", "کانتابریا"), ("uz", "Cantabria"), ("vi", "Cantabria"), ("yue", "坎塔布里亞"), ("yue_Hans", "坎塔布里亚"), ("zh", "坎塔布里亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CC",
                    Subdivision{
                        name: "CC",
                        country_alpha2: Alpha2::ES,
                        code: "CC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.4752765), longitude: Some(-6.3724247), max_latitude: Some(39.500104), min_latitude: Some(39.4431692), max_longitude: Some(-6.3543895), min_longitude: Some(-6.4270489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قصرش"), ("az", "Kaseres vilayəti"), ("be", "Касерэс"), ("bn", "সিরেরেস প\u{9cd}রদেশ"), ("ca", "Província de Càceres"), ("ccp", "𑄇𑄬𑄥𑄬𑄢𑄬𑄌\u{11134}"), ("ceb", "Provincia de Cáceres"), ("cs", "Provincie Cáceres"), ("da", "Cáceres"), ("de", "Provinz Cáceres"), ("el", "Επαρχία του Κάθερες"), ("en", "Cáceres"), ("es", "Provincia de Cáceres"), ("et", "Cácerese provints"), ("eu", "Cáceresko probintzia"), ("fa", "استان کاسرس"), ("fi", "Cáceres"), ("fr", "province de Cáceres"), ("ga", "Cáceres"), ("gl", "Provincia de Cáceres"), ("gu", "ક\u{ac7}સ\u{ac7}ર\u{ac7}સ પ\u{acd}રા\u{a82}ત"), ("hi", "कास\u{947}र\u{947}स प\u{94d}रा\u{902}त"), ("hr", "Cáceres"), ("hu", "Cáceres"), ("hy", "Կասարես"), ("id", "Provinsi Cáceres"), ("it", "Provincia di Cáceres"), ("ja", "カセレス県"), ("ka", "კასერესის პროვინცია"), ("kk", "Касерес"), ("kn", "ಕ\u{ccd}ಯಾಸ\u{cc6}ರ\u{cc6}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카세레스 주"), ("lt", "Kasereso provincija"), ("lv", "Kaseresas province"), ("mr", "क\u{945}सर\u{947}स प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cáceres"), ("nb", "Cáceres"), ("nl", "Cáceres"), ("no", "Cáceres"), ("pl", "Cáceres"), ("pt", "Cáceres (província)"), ("ro", "Provincia Cáceres"), ("ru", "Касерес"), ("si", "ක\u{dcf}සේරෙස\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Cáceres"), ("sr", "Покрајина Касерес"), ("sr_Latn", "Pokrajina Kaseres"), ("sv", "Cáceres"), ("ta", "கஸ\u{bcd}ர\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}కరస\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาเซเรส"), ("tr", "Cáceres ili"), ("uk", "Касерес"), ("ur", "صوبہ کاسیریس"), ("uz", "Cáceres"), ("vi", "Cáceres"), ("yue", "卡舍利斯省"), ("yue_Hans", "卡舍利斯省"), ("zh", "卡塞雷斯省")]),
                        unofficial_name_list: ["Cáceres"].to_vec(),
                    }
                ),
                (
                    "CE",
                    Subdivision{
                        name: "CE",
                        country_alpha2: Alpha2::ES,
                        code: "CE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.8893874), longitude: Some(-5.3213455), max_latitude: Some(35.9068648), min_latitude: Some(35.8710601), max_longitude: Some(-5.27836), min_longitude: Some(-5.3587124)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCityInNorthAfrica,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Ceuta"), ("am", "ሴውታ"), ("ar", "سبتة"), ("az", "Seuta"), ("be", "Сеўта"), ("bg", "Сеута"), ("bn", "সিউচ\u{9be}"), ("bs", "Ceuta"), ("ca", "Ceuta"), ("ccp", "𑄥𑄬𑄅\u{1112a}𑄑"), ("ceb", "Ceuta"), ("cs", "Ceuta"), ("cy", "Ceuta"), ("da", "Ceuta"), ("de", "Ceuta"), ("el", "Θέουτα"), ("en", "Ceuta"), ("es", "Ceuta"), ("et", "Ceuta"), ("eu", "Ceuta"), ("fa", "سبته"), ("fi", "Ceuta"), ("fr", "Ceuta"), ("ga", "Ceuta"), ("gl", "Ceuta"), ("gu", "સ\u{acd}ય\u{ac1}ટા"), ("he", "סאוטה"), ("hi", "स\u{947}उटा"), ("hr", "Ceuta"), ("hu", "Ceuta"), ("hy", "Սեուտա"), ("id", "Ceuta"), ("is", "Ceuta"), ("it", "Ceuta"), ("ja", "セウタ"), ("jv", "Ceuta"), ("ka", "სეუტა"), ("kk", "Сеута"), ("kn", "ಸ\u{cbf}ಯುಟಾ"), ("ko", "세우타"), ("lt", "Seuta"), ("lv", "Seuta"), ("mk", "Сеута"), ("mr", "स\u{947}उता"), ("ms", "Ceuta"), ("nb", "Ceuta"), ("ne", "स\u{947}उटा"), ("nl", "Ceuta"), ("no", "Ceuta"), ("pl", "Ceuta"), ("pt", "Ceuta"), ("ro", "Ceuta"), ("ru", "Сеута"), ("si", "ස\u{dd2}ය\u{dd4}ට\u{dcf}"), ("sk", "Ceuta"), ("sl", "Ceuta"), ("sq", "Ceuta"), ("sr", "Сеута"), ("sr_Latn", "Seuta"), ("sv", "Ceuta"), ("sw", "Ceuta"), ("ta", "சியூட\u{bbe}"), ("te", "స\u{c3f}యూట\u{c3e}"), ("th", "เซวตา"), ("tr", "Ceuta"), ("uk", "Сеута"), ("ur", "سبتہ"), ("vi", "Ceuta"), ("yo", "Ceuta"), ("yo_BJ", "Ceuta"), ("yue", "休達"), ("yue_Hans", "休达"), ("zh", "休达"), ("zu", "Ceuta")]),
                        unofficial_name_list: ["Ceuta", "Sebta"].to_vec(),
                    }
                ),
                (
                    "CL",
                    Subdivision{
                        name: "CL",
                        country_alpha2: Alpha2::ES,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Kastilië en León"), ("am", "ካስቲያና ሌዮን"), ("ar", "منطقة قـشتالة وليون"), ("az", "Kastiliya və Leon"), ("be", "Кастылія і Леон"), ("bg", "Кастилия и Леон"), ("bs", "Kastilja i Leon"), ("ca", "Castella i Lleó"), ("ccp", "𑄇\u{11133}𑄠𑄌\u{11134}𑄑\u{1112d}𑄣\u{11134} 𑄃\u{11133}𑄃 𑄣\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Castilla y León"), ("cs", "Kastilie a León"), ("cy", "Castilla y León"), ("da", "Castilla y León"), ("de", "Autonome Gemeinschaft Kastilien und León"), ("el", "Καστίλλη και Λεόν"), ("en", "Castile and León"), ("es", "Castilla y León"), ("et", "Castilla-León"), ("eu", "Gaztela eta Leon"), ("fa", "کاستیا و لئون"), ("fi", "Kastilia ja León"), ("fr", "Castille-et-León"), ("ga", "Castilla y León"), ("gl", "Castela e León"), ("he", "קסטיליה ולאון"), ("hi", "क\u{948}स\u{94d}टिल\u{947} और ल\u{947}ओन"), ("hr", "Kastilja i León"), ("hu", "Kasztília és León"), ("hy", "Կաստիլիա և Լեոն"), ("id", "Castilla y León"), ("is", "Kastilía-León"), ("it", "Castiglia e León"), ("ja", "カスティーリャ・レオン州"), ("jv", "Castile-Leon"), ("ka", "კასტილია და ლეონი"), ("kk", "Кастилия және Леон"), ("ko", "카스티야이레온 지방"), ("lt", "Kastilija ir Leonas"), ("lv", "Kastīlija un Leona"), ("mk", "Кастиља и Леон"), ("mr", "कास\u{94d}तिया इ ल\u{947}ओन"), ("ms", "Castile-León"), ("nb", "Castilla y León"), ("nl", "Castilië en León"), ("no", "Castilla y León"), ("pl", "Kastylia i León"), ("pt", "Castela e Leão"), ("ro", "Castilia și León"), ("ru", "Кастилия и Леон"), ("sq", "Castile dhe León"), ("sr", "Кастиља и Леон"), ("sr_Latn", "Kastilja i Leon"), ("sv", "Kastilien och León"), ("th", "แคว\u{e49}นคาสต\u{e35}ลและเลออน"), ("tr", "Kastilya ve Leon"), ("uk", "Кастилія-і-Леон"), ("ur", "قشتالہ اور لیون"), ("uz", "Castilla y León"), ("vi", "Castilla và León"), ("yue", "卡斯提亞與利昂"), ("yue_Hans", "卡斯提亚与利昂"), ("zh", "卡斯蒂利亚-莱昂")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CM",
                    Subdivision{
                        name: "CM",
                        country_alpha2: Alpha2::ES,
                        code: "CM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة كاستيا لا مانتشا"), ("az", "Kastiliya – La-Mança"), ("be", "Кастылія — Ла-Манча"), ("bg", "Кастилия - Ла Манча"), ("bs", "Castilla-La Mancha"), ("ca", "Castella - la Manxa"), ("ccp", "𑄇\u{11133}𑄠𑄌\u{11134}𑄑\u{1112d}𑄣\u{11134}-𑄣 𑄟𑄚\u{11134}𑄌"), ("ceb", "Castilla-La Mancha"), ("cs", "Kastilie – La Mancha"), ("cy", "Castilla-La Mancha"), ("da", "Castilla-La Mancha"), ("de", "Autonome Gemeinschaft Kastilien-La Mancha"), ("el", "Καστίλλη-Λα Μάντσα"), ("en", "Castile-La Mancha"), ("es", "Castilla-La Mancha"), ("eu", "Gaztela-Mantxa"), ("fa", "کاستیا-لامانچا"), ("fi", "Kastilia- La Mancha"), ("fr", "Castille-La Manche"), ("ga", "Castilla-La Mancha"), ("gl", "Castela-A Mancha"), ("he", "קסטיליה-לה מנצ׳ה"), ("hi", "कास\u{94d}तिया-ला मा\u{902}चा"), ("hr", "Kastilja-La Mancha"), ("hu", "Kasztília-La Mancha"), ("hy", "Կաստիլիա-Լա Մանչա"), ("id", "Castilla-La Mancha"), ("is", "Kastilía-La Mancha"), ("it", "Castiglia-La Mancia"), ("ja", "カスティーリャ・ラ・マンチャ州"), ("jv", "Castile-La Mancha"), ("ka", "კასტილია-ლა მანჩა"), ("kk", "Кастилия — Ла-Манча"), ("ko", "카스티야라만차 지방"), ("lt", "Kastilija ir La Manča"), ("lv", "Kastīlija-Lamanča"), ("mk", "Кастиља-Ла Манча"), ("mr", "कास\u{94d}तिया-ला मा\u{902}चा"), ("ms", "Castile-La Mancha"), ("nb", "Castilla-La Mancha"), ("nl", "Castilië-La Mancha"), ("no", "Castilla-La Mancha"), ("pl", "Kastylia-La Mancha"), ("pt", "Castela-Mancha"), ("ro", "Castilia-La Mancha"), ("ru", "Кастилия — Ла-Манча"), ("sl", "Kategorija:Kastilja-Manča"), ("sq", "Castilla-La Mancha"), ("sr", "Кастиља-Ла Манча"), ("sr_Latn", "Kastilja-La Manča"), ("sv", "Kastilien-La Mancha"), ("th", "แคว\u{e49}นคาสต\u{e35}ล-ลาม\u{e31}นชา"), ("tr", "Kastilya-La Mancha"), ("uk", "Кастилія — Ла-Манча"), ("ur", "کاستیا-لا مانچا"), ("uz", "Castilla-La Mancha"), ("vi", "Castile-La Mancha"), ("yue", "卡斯提亞-拉曼查"), ("yue_Hans", "卡斯提亚-拉曼查"), ("zh", "卡斯蒂利亚-拉曼恰")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CN",
                    Subdivision{
                        name: "CN",
                        country_alpha2: Alpha2::ES,
                        code: "CN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄇\u{11133}𑄠𑄚𑄢\u{11128} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("en", "Canary Islands")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::ES,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.8881751), longitude: Some(-4.7793835), max_latitude: Some(37.9272788), min_latitude: Some(37.8558932), max_longitude: Some(-4.7461769), min_longitude: Some(-4.8227937)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة قرطبة"), ("be", "Правінцыя Кордава"), ("bg", "Кордоба"), ("bn", "কর\u{9cd}ডোব\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Còrdova"), ("ccp", "𑄇\u{1112e}𑄢\u{11134}𑄓\u{1112e}𑄝"), ("ceb", "Province of Córdoba"), ("cs", "Provincie Córdoba"), ("da", "Córdoba"), ("de", "Córdoba"), ("el", "Κόρδοβα"), ("en", "Córdoba"), ("es", "Provincia de Córdoba"), ("et", "Córdoba provints"), ("eu", "Kordobako probintzia"), ("fa", "استان کوردوبا"), ("fi", "Córdoba"), ("fr", "province de Cordoue"), ("ga", "Córdoba"), ("gl", "Provincia de Córdoba"), ("gu", "કોર\u{acd}ડોબા પ\u{acd}રા\u{a82}ત"), ("hi", "कोर\u{94d}डोबा प\u{94d}रा\u{902}त"), ("hr", "Córdoba"), ("hu", "Córdoba"), ("id", "Provinsi Córdoba"), ("it", "Provincia di Cordova"), ("ja", "コルドバ県"), ("ka", "კორდობის პროვინცია"), ("kk", "Кордова"), ("kn", "ಕೊರ\u{ccd}ಡೊಬಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코르도바 주"), ("lt", "Kordobos provincija"), ("lv", "Kordovas province"), ("mk", "Кордоба"), ("mr", "कॉर\u{94d}डोबा प\u{94d}रा\u{902}त"), ("ms", "Provinsi Córdoba"), ("nb", "Córdoba"), ("nl", "Córdoba"), ("no", "Córdoba"), ("pl", "Kordoba"), ("pt", "Córdova (província da Espanha)"), ("ro", "Provincia Córdoba"), ("ru", "Кордова"), ("si", "කොර\u{dca}ඩොබ\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Cordoba"), ("sr", "Покрајина Кордоба"), ("sr_Latn", "Pokrajina Kordoba"), ("sv", "Córdoba"), ("sw", "Mkoa wa Córdoba"), ("ta", "கோர\u{bcd}டோப\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4a}ర\u{c4b}డ\u{c4b}బ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ร\u{e31}ฐกอร\u{e4c}โดบา"), ("tr", "Córdoba ili"), ("uk", "Кордова"), ("ur", "صوبہ قرطبہ"), ("uz", "Córdoba"), ("vi", "Córdoba"), ("zh", "科爾多瓦省")]),
                        unofficial_name_list: ["Córdoba"].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::ES,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.9848295), longitude: Some(-3.927377799999999), max_latitude: Some(39.0062168), min_latitude: Some(38.9658707), max_longitude: Some(-3.9005951), min_longitude: Some(-3.9533808)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ثيوداد ريال"), ("be", "Сьюдад-Рэаль"), ("bn", "কিউদ\u{9be}দ"), ("ca", "Província de Ciudad Real"), ("ccp", "𑄥\u{1112d}𑄅\u{1112a}𑄘𑄖\u{11134} 𑄢\u{11128}𑄠𑄬𑄣\u{11134}"), ("ceb", "Provincia de Ciudad Real"), ("cs", "Provincie Ciudad Real"), ("da", "Ciudad Real"), ("de", "Provinz Ciudad Real"), ("el", "Θιουδάδ Ρεάλ"), ("en", "Ciudad Real"), ("es", "Provincia de Ciudad Real"), ("et", "Ciudad Reali provints"), ("eu", "Ciudad Realgo probintzia"), ("fa", "استان سیوداد رئال"), ("fi", "Ciudad Real"), ("fr", "province de Ciudad Real"), ("gl", "Provincia de Cidade Real - Ciudad Real"), ("gu", "સિય\u{ac1}દાદ રિયલ પ\u{acd}રા\u{a82}ત"), ("hi", "सिदाद रियल प\u{94d}रा\u{902}त"), ("hr", "Ciudad Real"), ("hu", "Ciudad Real"), ("id", "Provinsi Ciudad Real"), ("it", "provincia di Ciudad Real"), ("ja", "シウダ・レアル県"), ("ka", "სიუდად-რეალის პროვინცია"), ("kk", "Сьюдад-Реаль"), ("kn", "ಸ\u{cbf}ಯುಡಾಡ\u{ccd} ರ\u{cbf}ಯಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시우다드레알 주"), ("lt", "Siudad Realio provincija"), ("lv", "Sjudadrealas province"), ("mr", "सिउदाद रिअल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ciudad Real"), ("nb", "Ciudad Real"), ("nl", "Ciudad Real"), ("no", "Ciudad Real"), ("pl", "Ciudad Real"), ("pt", "Cidade Real"), ("ro", "Provincia Ciudad Real"), ("ru", "Сьюдад-Реаль"), ("si", "ක\u{dd2}ය\u{dd4}ඩැඩ\u{dca}"), ("sq", "Provinca Ciudad Real"), ("sr", "Покрајина Сијудад Реал"), ("sr_Latn", "Pokrajina Sijudad Real"), ("sv", "Ciudad Real"), ("sw", "Mkoa wa Ciudad Real"), ("ta", "சியூடட\u{bcd} ரியல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}యుడ\u{c3e}డ\u{c4d} ర\u{c3f}యల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซ\u{e34}วด\u{e31}ดเรอ\u{e31}ล"), ("tr", "Ciudad Real ili"), ("uk", "Сьюдад-Реаль"), ("ur", "صوبہ سیوداد ریال"), ("uz", "Ciudad Real"), ("vi", "Ciudad Real"), ("zh", "雷阿爾城省")]),
                        unofficial_name_list: ["Ciudad Real"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "CS",
                        country_alpha2: Alpha2::ES,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.9863563), longitude: Some(-0.0513246), max_latitude: Some(40.0041729), min_latitude: Some(39.970746), max_longitude: Some(-0.0163697), min_longitude: Some(-0.0797563)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كاستيون"), ("be", "правінцыя Кастэльён"), ("bn", "ক\u{9cd}য\u{9be}স\u{9cd}টেল\u{9cd}লন প\u{9cd}রদেশ"), ("ca", "província de Castelló"), ("ccp", "𑄇𑄌\u{11134}𑄑𑄬𑄣\u{11127}𑄚\u{11134}"), ("ceb", "Província de Castelló"), ("cs", "Provincie Castellón"), ("da", "Castellón"), ("de", "Provinz Castellón"), ("el", "Επαρχία της Καστελιό"), ("en", "Castellón"), ("es", "Provincia de Castellón"), ("et", "Castellóni provints"), ("eu", "Castellóko probintzia"), ("fa", "استان کاستلیون"), ("fi", "Castellón"), ("fr", "province de Castellón"), ("gl", "Provincia de Castelló"), ("gu", "ક\u{ac7}સ\u{acd}ટ\u{ac7}લોન પ\u{acd}રા\u{a82}ત"), ("he", "מחוז קאסטיון"), ("hi", "कास\u{94d}ट\u{947}लोन प\u{94d}रा\u{902}त"), ("hr", "Castellón"), ("hu", "Castellón"), ("id", "Provinsi Castellón"), ("it", "provincia di Castellón"), ("ja", "カステリョン県"), ("ka", "კასტელონის პროვინცია"), ("kk", "Кастельон"), ("kn", "ಕ\u{ccd}ಯಾಸ\u{ccd}ಟ\u{cc6}ಲ\u{ccd}ಲನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "카스테욘 주"), ("lt", "Kasteljono provincija"), ("lv", "Kastelo province"), ("mk", "Кастелјон"), ("mr", "कास\u{94d}ट\u{947}लोन प\u{94d}रा\u{902}त"), ("ms", "Wilayah Castellón"), ("nb", "Castellón"), ("nl", "Castellón"), ("no", "Castellón"), ("pl", "Castellón"), ("pt", "Castellón"), ("ro", "Provincia Castellón"), ("ru", "Кастельон"), ("si", "කැස\u{dca}ටෙලෝන\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Castellón"), ("sr", "Покрајина Кастељон"), ("sr_Latn", "Pokrajina Kasteljon"), ("sv", "Castellón"), ("sw", "Mkoa wa Castellón"), ("ta", "க\u{bbe}ஸ\u{bcd}டெல\u{bcd}லோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}స\u{c4d}ట\u{c46}ల\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดคาสเตลลอน"), ("tr", "Castellón"), ("uk", "Кастельйон"), ("ur", "صوبہ کاستیلون"), ("uz", "Castellón"), ("vi", "Castellón"), ("zh", "卡斯特利翁省")]),
                        unofficial_name_list: ["Castelló", "Castelló/Castellón", "Castellón", "Castellón/Castelló"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "CT",
                        country_alpha2: Alpha2::ES,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Katalonië"), ("am", "ካታሎኒያ"), ("ar", "كتالونيا"), ("az", "Kataloniya"), ("be", "Каталонія"), ("bg", "Каталония"), ("bn", "ক\u{9be}ত\u{9be}লোনিয\u{9bc}\u{9be}"), ("bs", "Katalonija"), ("ca", "Catalunya"), ("ccp", "𑄇\u{11133}𑄠𑄑𑄣\u{11127}𑄚\u{11128}𑄠"), ("ceb", "Catalunya"), ("cs", "Katalánsko"), ("cy", "Catalwnia"), ("da", "Catalonien"), ("de", "Autonome Gemeinschaft Katalonien"), ("el", "Καταλονία"), ("en", "Catalonia"), ("es", "Cataluña"), ("et", "Kataloonia"), ("eu", "Katalunia"), ("fa", "کاتالونیا"), ("fi", "Katalonia"), ("fr", "Catalogne"), ("ga", "An Chatalóin"), ("gl", "Cataluña"), ("ha", "Katalunya"), ("ha_NE", "Katalunya"), ("he", "קטלוניה"), ("hi", "कातालोन\u{94d}या"), ("hr", "Katalonija"), ("hu", "Katalónia"), ("hy", "Կատալոնիա"), ("id", "Catalunya"), ("is", "Katalónía"), ("it", "Catalogna"), ("ja", "カタルーニャ州"), ("jv", "Catalunya"), ("ka", "კატალონია"), ("kk", "Каталония"), ("ko", "카탈루냐 지방"), ("ky", "Каталония"), ("lo", "ແຄວ\u{ec9}ນກາຕາໂລຍ"), ("lt", "Katalonija"), ("lv", "Katalonija"), ("mk", "Каталонија"), ("ml", "ക\u{d3e}റ\u{d4d}റലോണിയ"), ("mn", "Каталон орон"), ("mr", "कातालोनिया"), ("ms", "Catalonia"), ("my", "ကက\u{103a}တလ\u{102d}\u{102f}န\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("nb", "Catalonia"), ("nl", "Catalonië"), ("no", "Catalonia"), ("or", "କେଟଲୋନ\u{b3f}ଆ"), ("pa", "ਕਾਤਾਲ\u{a4b}ਨੀਆ"), ("pl", "Katalonia"), ("pt", "Catalunha"), ("ro", "Catalonia"), ("ru", "Каталония"), ("si", "කැටලෝන\u{dd2}ය\u{dcf}"), ("sk", "Katalánsko"), ("sl", "Katalonija"), ("so", "Katalooniya"), ("sq", "Katalonia"), ("sr", "Каталонија"), ("sr_Latn", "Katalonija"), ("sv", "Katalonien"), ("sw", "Catalonia"), ("ta", "க\u{bbe}த\u{bcd}தலோனிய\u{bbe}"), ("th", "แคว\u{e49}นกาตาล\u{e38}ญญา"), ("tk", "Kataloniýa"), ("tr", "Katalonya"), ("uk", "Каталонія"), ("ur", "کاتالونیا"), ("uz", "Kataloniya"), ("vi", "Catalunya"), ("yue", "加泰隆尼亞"), ("yue_Hans", "加泰隆尼亚"), ("zh", "加泰罗尼亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CU",
                    Subdivision{
                        name: "CU",
                        country_alpha2: Alpha2::ES,
                        code: "CU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.0703925), longitude: Some(-2.1374162), max_latitude: Some(40.083813), min_latitude: Some(40.0401312), max_longitude: Some(-2.1158454), min_longitude: Some(-2.1802044)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "قونكة"), ("az", "Kuenqa vilayəti"), ("be", "Куэнка"), ("bg", "Куенка"), ("bn", "ক\u{9c1}য\u{9bc}েঙ\u{9cd}ক\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Conca"), ("ccp", "𑄇\u{1112a}𑄠𑄬𑄚\u{11134}𑄇"), ("ceb", "Provincia de Cuenca"), ("cs", "Provincie Cuenca"), ("da", "Cuenca"), ("de", "Provinz Cuenca"), ("el", "Κουένκα"), ("en", "Cuenca"), ("es", "Provincia de Cuenca"), ("et", "Cuenca provints"), ("eu", "Cuencako probintzia"), ("fa", "استان کوئنکا"), ("fi", "Cuenca"), ("fr", "province de Cuenca"), ("ga", "Cuenca"), ("gl", "Provincia de Cuenca"), ("gu", "ક\u{ac1}એન\u{acd}કા પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{941}ए\u{901}का प\u{94d}रा\u{902}त"), ("hr", "Cuenca"), ("hu", "Cuenca"), ("id", "Provinsi Cuenca"), ("it", "Provincia di Cuenca"), ("ja", "クエンカ県"), ("ka", "კუენკის პროვინცია"), ("kk", "Куэнка"), ("kn", "ಕ\u{ccd}ಯುನ\u{cc6}ಕಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "쿠엥카 주"), ("lt", "Kuenkos provincija"), ("lv", "Kvenkas province"), ("mr", "क\u{941}एनका प\u{94d}रा\u{902}त"), ("ms", "Wilayah Cuenca"), ("nb", "Cuenca"), ("nl", "Cuenca"), ("no", "Cuenca"), ("pl", "Cuenca"), ("pt", "Cuenca"), ("ro", "Provincia Cuenca"), ("ru", "Куэнка"), ("si", "ක\u{dd4}එන\u{dca}ක\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Cuenca"), ("sr", "Покрајина Куенка"), ("sr_Latn", "Pokrajina Kuenka"), ("sv", "Cuenca"), ("sw", "Mkoa wa Cuenca"), ("ta", "குயென\u{bcd}க\u{bcd}க\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}యుయ\u{c46}ంక\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e39}เอนกา"), ("tr", "Cuenca ili"), ("uk", "Куенка"), ("ur", "صوبہ کوینکا"), ("uz", "Cuenca"), ("vi", "Cuenca"), ("zh", "昆卡省")]),
                        unofficial_name_list: ["Cuenca"].to_vec(),
                    }
                ),
                (
                    "EX",
                    Subdivision{
                        name: "EX",
                        country_alpha2: Alpha2::ES,
                        code: "EX",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Extremadura"), ("ar", "منطقة إكستـريمادورا"), ("az", "Estremadura"), ("be", "Эстрэмадура"), ("bg", "Естремадура"), ("bs", "Extremadura"), ("ca", "Extremadura"), ("ccp", "𑄃𑄬𑄇\u{11134}𑄑\u{11133}𑄢𑄬𑄟𑄓\u{1112a}𑄠𑄢"), ("ceb", "Extremadura"), ("cs", "Extremadura"), ("cy", "Extremadura"), ("da", "Extremadura"), ("de", "Autonome Gemeinschaft Extremadura"), ("el", "Εξτρεμαδούρα"), ("en", "Extremadura"), ("es", "Extremadura"), ("et", "Extremadura"), ("eu", "Extremadura"), ("fa", "اکسترمادورا"), ("fi", "Extremadura"), ("fr", "Estrémadure"), ("ga", "Extremadura"), ("gl", "Estremadura - Extremadura"), ("he", "אקסטרמדורה"), ("hi", "एक\u{94d}सत\u{94d}र\u{947}म\u{947}द\u{94d}य\u{941}रा"), ("hr", "Ekstremadura"), ("hu", "Extremadura"), ("hy", "Էստրեմադուրա"), ("id", "Extremadura"), ("is", "Extremadúra"), ("it", "Estremadura"), ("ja", "エストレマドゥーラ州"), ("ka", "ესტრემადურა"), ("kk", "Эстремадура"), ("ko", "에스트레마두라 지방"), ("lo", "ແຄວ\u{ec9}ນແອສະຕະເຣມາດ\u{eb9}"), ("lt", "Estremadūra"), ("lv", "Estremadura"), ("mk", "Екстремадура"), ("mr", "एस\u{94d}त\u{94d}र\u{947}माद\u{941}रा"), ("ms", "Extremadura"), ("nb", "Extremadura"), ("nl", "Extremadura"), ("no", "Extremadura"), ("pl", "Estremadura"), ("pt", "Estremadura (Espanha)"), ("ro", "Extremadura"), ("ru", "Эстремадура"), ("sk", "Extremadura"), ("sl", "Extremadura"), ("sq", "Extremadura"), ("sr", "Екстремадура"), ("sr_Latn", "Ekstremadura"), ("sv", "Extremadura"), ("th", "แคว\u{e49}นเอกซ\u{e4c}เตรมาด\u{e39}รา"), ("tr", "Ekstremadura"), ("uk", "Естремадура"), ("ur", "اکستریمادورا"), ("uz", "Extremadura"), ("vi", "Extremadura"), ("yue", "易斯特里馬杜拉"), ("yue_Hans", "易斯特里马杜拉"), ("zh", "埃斯特雷马杜拉")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::ES,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Galicië"), ("am", "ጋሊስያ"), ("ar", "منطقة غاليسيا"), ("az", "Qalisiya"), ("be", "Галісія"), ("bg", "Галисия"), ("bn", "গ\u{9be}লিথিয\u{9bc}\u{9be}"), ("bs", "Galicija"), ("ca", "Galícia"), ("ccp", "𑄉𑄣\u{11128}𑄥\u{11128}𑄠"), ("ceb", "Galicia"), ("cs", "Galicie"), ("cy", "Galisia"), ("da", "Galicien"), ("de", "Galicien"), ("el", "Γαλικία"), ("en", "Galicia"), ("es", "Galicia"), ("et", "Galicia"), ("eu", "Galizia"), ("fa", "گالیسیا"), ("fi", "Galicia"), ("fr", "Galice"), ("ga", "An Ghailís"), ("gl", "Galicia"), ("he", "גליסיה"), ("hi", "गलिशिया"), ("hr", "Galicija"), ("hu", "Galicia"), ("hy", "Գալիսիա"), ("id", "Galisia"), ("ig", "Galisa"), ("is", "Galisía"), ("it", "Galizia"), ("ja", "ガリシア州"), ("jv", "Galicia"), ("ka", "გალისია"), ("kk", "Галисия"), ("ko", "갈리시아 지방"), ("ky", "Галисия"), ("lt", "Galisija"), ("lv", "Galisija"), ("mk", "Галиција"), ("mr", "गालिसिया"), ("ms", "Galicia"), ("nb", "Galicia"), ("nl", "Galicië"), ("no", "Galicia"), ("pl", "Galicja"), ("pt", "Galiza"), ("ro", "Galicia"), ("ru", "Галисия"), ("sk", "Galícia"), ("sl", "Galicija"), ("sq", "Galicia"), ("sr", "Галиција"), ("sr_Latn", "Galicija"), ("sv", "Galicien"), ("ta", "கல\u{bc0}சிய\u{bbe}"), ("te", "గల\u{c3f}స\u{c3f}య\u{c3e}"), ("th", "แคว\u{e49}นกาล\u{e34}เซ\u{e35}ย"), ("tr", "Galiçya"), ("uk", "Галісія"), ("ur", "گالیسیا"), ("uz", "Galisiya"), ("vi", "Galicia"), ("yue", "加利西亞"), ("yue_Hans", "加利西亚"), ("zh", "加利西亚")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GC",
                    Subdivision{
                        name: "GC",
                        country_alpha2: Alpha2::ES,
                        code: "GC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.1235459), longitude: Some(-15.4362574), max_latitude: Some(28.1563403), min_latitude: Some(28.0783812), max_longitude: Some(-15.4117481), min_longitude: Some(-15.467486)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاس بالماس"), ("be", "Правінцыя Лас-Пальмас"), ("bn", "ল\u{9be}স প\u{9be}লম\u{9be}স প\u{9cd}রদেশ"), ("ca", "província de Las Palmas"), ("ccp", "𑄣𑄌\u{11134} 𑄛\u{11127}𑄣\u{11134}𑄟𑄌\u{11134}"), ("ceb", "Provincia de Las Palmas"), ("cs", "Provincie Las Palmas"), ("da", "Las Palmas"), ("de", "Provinz Las Palmas"), ("el", "Λας Πάλμας"), ("en", "Las Palmas"), ("es", "Provincia de Las Palmas"), ("et", "Las Palmase provints"), ("eu", "Las Palmasko probintzia"), ("fa", "استان لاسپالماس"), ("fi", "Las Palmas"), ("fr", "province de Las Palmas"), ("gl", "Provincia das Palmas"), ("gu", "લાસ પામસ પ\u{acd}રા\u{a82}ત"), ("hi", "लास पाल\u{94d}मास प\u{94d}रा\u{902}त"), ("hr", "Las Palmas"), ("hu", "Las Palmas"), ("hy", "Լաս Պալմաս"), ("id", "Provinsi Las Palmas"), ("it", "Provincia di Las Palmas"), ("ja", "ラス・パルマス県"), ("ka", "ლას პალმასის პროვინცია"), ("kk", "Лас-Пальмас"), ("kn", "ಲಾಸ\u{ccd} ಪಾಲ\u{ccd}ಮಾಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라스팔마스 주"), ("lt", "Las Palmo provincija"), ("lv", "Laspalmasas province"), ("mr", "लास पालमास प\u{94d}रा\u{902}त"), ("ms", "Wilayah Las Palmas"), ("nb", "Las Palmas"), ("nl", "Las Palmas"), ("no", "Las Palmas"), ("pl", "Las Palmas"), ("pt", "Las Palmas"), ("ro", "Provincia Las Palmas"), ("ru", "Лас-Пальмас"), ("si", "ල\u{dcf} පල\u{dca}ම\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Las Palmas"), ("sr", "Покрајина Лас Палмас"), ("sr_Latn", "Pokrajina Las Palmas"), ("sv", "Las Palmas"), ("sw", "Mkoa wa Las Palmas"), ("ta", "ல\u{bbe}ஸ\u{bcd} ப\u{bbe}ல\u{bcd}ம\u{bbe}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3e}స\u{c4d}\u{c4d} ప\u{c3e}ల\u{c4d}మ\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดลาส ปาลมาส"), ("tr", "Las Palmas ili"), ("uk", "Лас-Пальмас"), ("ur", "صوبہ لاس پاماس"), ("uz", "Las Palmas"), ("vi", "Las Palmas"), ("zh", "拉斯帕爾馬斯省")]),
                        unofficial_name_list: ["Las Palmas", "Las Palmas de Gran Canaria", "Palmas, Las"].to_vec(),
                    }
                ),
                (
                    "GI",
                    Subdivision{
                        name: "GI",
                        country_alpha2: Alpha2::ES,
                        code: "GI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.9794005), longitude: Some(2.8214264), max_latitude: Some(42.0142089), min_latitude: Some(41.9462966), max_longitude: Some(2.8389562), min_longitude: Some(2.7983782)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جرندة"), ("az", "Jirona vilayəti"), ("be", "правінцыя Херона"), ("bg", "Херона"), ("bn", "গিরোন\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Girona"), ("ccp", "𑄉\u{11128}𑄢\u{1112e}𑄚"), ("ceb", "Província de Girona"), ("cs", "Provincie Girona"), ("cy", "Talaith Girona"), ("da", "Girona (provins)"), ("de", "Provinz Girona"), ("el", "Ζιρόνα"), ("en", "Girona"), ("es", "Provincia de Gerona"), ("et", "Girona provints"), ("eu", "Gironako probintzia"), ("fa", "استان خرنا"), ("fi", "Girona"), ("fr", "province de Gérone"), ("gl", "Provincia de Xirona - Girona"), ("gu", "ગિરૉના પ\u{acd}રા\u{a82}ત"), ("he", "מחוז ז׳ירונה"), ("hi", "गिरोना प\u{94d}रा\u{902}त"), ("hr", "Girona"), ("hu", "Girona"), ("id", "Provinsi Girona"), ("it", "Provincia di Girona"), ("ja", "ジローナ県"), ("ka", "ხერონის პროვინცია"), ("kk", "Херона"), ("kn", "ಗ\u{cbf}ರೊನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "지로나 주"), ("lt", "Žironos provincija"), ("lv", "Žironas province"), ("mk", "Жирона"), ("mr", "गिरोना प\u{94d}रा\u{902}त"), ("ms", "Daearah Girona"), ("nb", "Girona"), ("nl", "Gerona"), ("no", "Girona"), ("pl", "Girona"), ("pt", "Girona"), ("ro", "Provincia Girona"), ("ru", "Херона"), ("si", "ගරෝන\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Girona"), ("sr", "Провинција Ђирона"), ("sr_Latn", "Provincija Đirona"), ("sv", "Girona"), ("sw", "Mkoa wa Gerona"), ("ta", "கிரோன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c3f}ర\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e34}โนนา"), ("tr", "Gerona ili"), ("uk", "Жирона"), ("ur", "صوبہ جیرونا"), ("uz", "Girona"), ("vi", "Girona"), ("yue", "赫羅納省"), ("yue_Hans", "赫罗纳省"), ("zh", "赫羅納省")]),
                        unofficial_name_list: ["Gerona", "Girona"].to_vec(),
                    }
                ),
                (
                    "GR",
                    Subdivision{
                        name: "GR",
                        country_alpha2: Alpha2::ES,
                        code: "GR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.1773363), longitude: Some(-3.5985571), max_latitude: Some(37.2124648), min_latitude: Some(37.1494277), max_longitude: Some(-3.5505714), min_longitude: Some(-3.6338351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غرناطة"), ("be", "Правінцыя Гранада"), ("bg", "Гранада"), ("bn", "গ\u{9cd}র\u{9be}ন\u{9be}ড\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Granada"), ("ccp", "𑄉\u{11133}𑄢𑄚𑄓"), ("ceb", "Provincia de Granada"), ("cs", "Provincie Granada"), ("da", "Granada"), ("de", "Provinz Granada"), ("el", "Γρανάδα"), ("en", "Granada"), ("es", "Provincia de Granada"), ("et", "Granada provints"), ("eu", "Granadako probintzia"), ("fa", "استان گرانادا"), ("fi", "Granada"), ("fr", "province de Grenade"), ("ga", "Granada"), ("gl", "Provincia de Granada"), ("gu", "ગ\u{acd}ર\u{ac7}નાડા પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{94d}रानादा प\u{94d}रान\u{94d}त"), ("hr", "Razgovor:Granada"), ("hu", "Granada"), ("id", "Provinsi Granada"), ("it", "Provincia di Granada"), ("ja", "グラナダ県"), ("ka", "გრანადის პროვინცია"), ("kk", "Гранада"), ("kn", "ಗ\u{ccd}ರಾನಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "그라나다 주"), ("lt", "Granados provincija"), ("lv", "Granadas province"), ("mr", "ग\u{94d}र\u{945}नडा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Granada"), ("nb", "Granada"), ("nl", "Granada"), ("no", "Granada"), ("pl", "Grenada"), ("pt", "Granada"), ("ro", "Provincia Granada"), ("ru", "Гранада"), ("si", "ග\u{dca}\u{200d}රන\u{dcf}ඩ\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Granada"), ("sr", "Провинција Гранада"), ("sr_Latn", "Provincija Granada"), ("sv", "Granada"), ("sw", "Mkoa wa Granada"), ("ta", "கிரனட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}రన\u{c3e}\u{c3e}డ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกรานาดา"), ("tr", "Granada ili"), ("uk", "Гранада"), ("ur", "صوبہ غرناطہ"), ("uz", "Granada"), ("vi", "Granada"), ("yue", "格蘭納達省"), ("yue_Hans", "格兰纳达省"), ("zh", "格拉納達省")]),
                        unofficial_name_list: ["Granada"].to_vec(),
                    }
                ),
                (
                    "GU",
                    Subdivision{
                        name: "GU",
                        country_alpha2: Alpha2::ES,
                        code: "GU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.632489), longitude: Some(-3.16017), max_latitude: Some(40.6458997), min_latitude: Some(40.6184806), max_longitude: Some(-3.1429369), min_longitude: Some(-3.2034081)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "غوادالاخارا"), ("be", "правінцыя Гвадалахара"), ("bg", "Гуадалахара"), ("bn", "গ\u{9c1}য\u{9bc}\u{9be}ড\u{9be}ল\u{9be}জ\u{9be}র\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de Guadalajara"), ("ccp", "𑄉\u{1112a}𑄠𑄓𑄣\u{11134}𑄎𑄢"), ("ceb", "Provincia de Guadalajara"), ("cs", "Provincie Guadalajara"), ("da", "Guadalajara"), ("de", "Provinz Guadalajara"), ("el", "Επαρχία της Γουαδαλαχάρα"), ("en", "Guadalajara"), ("es", "Provincia de Guadalajara"), ("et", "Guadalajara provints"), ("eu", "Guadalajarako probintzia"), ("fa", "استان گوادالاخارا"), ("fi", "Guadalajara"), ("fr", "province de Guadalajara"), ("gl", "Provincia de Guadalaxara - Guadalajara"), ("gu", "ગોડાલજારા પ\u{acd}રા\u{a82}ત"), ("hi", "ग\u{94d}वाडलाजारा प\u{94d}रा\u{902}त"), ("hr", "Guadalajara"), ("hu", "Guadalajara"), ("id", "Provinsi Guadalajara"), ("it", "Provincia di Guadalajara"), ("ja", "グアダラハラ県"), ("ka", "გვადალახარის პროვინცია"), ("kk", "Гвадалахара"), ("kn", "ಗ\u{ccd}ವಾಡಲಜರ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "과달라하라 주"), ("lt", "Gvadalacharos provincija"), ("lv", "Gvadalaharas province"), ("mk", "Гвадалахара"), ("mr", "गडालजारा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Guadalajara"), ("nb", "Guadalajara"), ("nl", "Guadalajara"), ("no", "Guadalajara"), ("pl", "Guadalajara"), ("pt", "Guadalajara"), ("ro", "Provincia Guadalajara"), ("ru", "Гвадалахара"), ("si", "ග\u{dd4}දලජ\u{dcf}ර\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Guadalajara"), ("sr", "Покрајина Гвадалахара"), ("sr_Latn", "Pokrajina Gvadalahara"), ("sv", "Guadalajara"), ("sw", "Mkoa wa Guadalajara"), ("ta", "குண\u{bcd}டல\u{bbe}ஜ\u{bbe}ர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "గ\u{c4d}వ\u{c3e}డ\u{c3e}ల\u{c3e}జ\u{c3e}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e31}วดาลาจารา"), ("tr", "Guadalajara ili"), ("uk", "Гвадалахара"), ("ur", "صوبہ گوادالاخارا"), ("uz", "Guadalajara"), ("vi", "Guadalajara"), ("zh", "瓜達拉哈拉省")]),
                        unofficial_name_list: ["Guadalajara"].to_vec(),
                    }
                ),
                (
                    "H",
                    Subdivision{
                        name: "H",
                        country_alpha2: Alpha2::ES,
                        code: "H",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.261421), longitude: Some(-6.9447224), max_latitude: Some(37.2913207), min_latitude: Some(37.2504151), max_longitude: Some(-6.916788899999999), min_longitude: Some(-6.962591799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ولبة"), ("be", "Правінцыя Уэльва"), ("bn", "হ\u{9c1}ল\u{9cd}ভ\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Huelva"), ("ccp", "𑄦\u{1112a}𑄠𑄬𑄣\u{11134}𑄞"), ("ceb", "Provincia de Huelva"), ("cs", "Provincie Huelva"), ("da", "Huelva"), ("de", "Provinz Huelva"), ("el", "Επαρχία της Ουέλβα"), ("en", "Huelva"), ("es", "Provincia de Huelva"), ("et", "Huelva provints"), ("eu", "Huelvako probintzia"), ("fa", "استان اوئلوا"), ("fi", "Huelva"), ("fr", "province de Huelva"), ("gl", "Provincia de Huelva"), ("gu", "હ\u{ac1}એલ\u{acd}વા પ\u{acd}રા\u{a82}ત"), ("hi", "ह\u{941}एल\u{94d}वा प\u{94d}रा\u{902}त"), ("hr", "Huelva"), ("hu", "Huelva tartomány"), ("hy", "Ուելվա"), ("id", "Provinsi Huelva"), ("it", "Provincia di Huelva"), ("ja", "ウエルバ県"), ("ka", "უელვის პროვინცია"), ("kk", "Уэльва"), ("kn", "ಹುಲ\u{ccd}ವಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우엘바 주"), ("lt", "Huelvos provincija"), ("lv", "Velvas province"), ("mr", "ह\u{941}एलवा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Huelva"), ("nb", "Huelva"), ("nl", "Huelva"), ("no", "Huelva"), ("pl", "Huelva"), ("pt", "Província de Huelva"), ("ro", "Provincia Huelva"), ("ru", "Уэльва"), ("si", "හ\u{dd4}එල\u{dca}ව\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Huelva"), ("sr", "Покрајина Уелва"), ("sr_Latn", "Pokrajina Uelva"), ("sv", "Huelva"), ("sw", "Mkoa wa Huelva"), ("ta", "ஹூயில\u{bcd}வ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హుయ\u{c46}ల\u{c4d}వ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e34}ววา"), ("tr", "Huelva ili"), ("uk", "Уельва"), ("ur", "صوبہ ویلبا"), ("uz", "Huelva"), ("vi", "Huelva"), ("zh", "韋爾瓦省")]),
                        unofficial_name_list: ["Huelva"].to_vec(),
                    }
                ),
                (
                    "HU",
                    Subdivision{
                        name: "HU",
                        country_alpha2: Alpha2::ES,
                        code: "HU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.131845), longitude: Some(-0.4078058), max_latitude: Some(42.1468304), min_latitude: Some(42.1249823), max_longitude: Some(-0.3885935), min_longitude: Some(-0.4231033)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "وشقة"), ("be", "Правінцыя Уэска"), ("bg", "Уеска"), ("bn", "হ\u{9c1}য\u{9bc}েস\u{9cd}ক\u{9be} প\u{9cd}রদেশ"), ("bs", "Huesca"), ("ca", "província d’Osca"), ("ccp", "𑄦\u{1112a}𑄠𑄬𑄌\u{11134}𑄇"), ("ceb", "Provincia de Huesca"), ("cs", "Provincie Huesca"), ("da", "Huesca"), ("de", "Provinz Huesca"), ("el", "Ουέσκα"), ("en", "Huesca"), ("es", "Provincia de Huesca"), ("et", "Huesca provints"), ("eu", "Huescako probintzia"), ("fa", "استان هوئسکا"), ("fi", "Huesca"), ("fr", "province de Huesca"), ("gl", "Provincia de Huesca - Uesca"), ("gu", "હ\u{ac1}એસ\u{acd}કા પ\u{acd}રા\u{a82}ત"), ("hi", "ह\u{94d}य\u{942}सका प\u{94d}रा\u{902}त"), ("hr", "Huesca"), ("hu", "Huesca"), ("hy", "Ուեսկա"), ("id", "Provinsi Huesca"), ("it", "Provincia di Huesca"), ("ja", "ウエスカ県"), ("ka", "უესკის პროვინცია"), ("kk", "Уэска"), ("kn", "ಹ\u{cc2}ಸ\u{ccd}ಕಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "우에스카 주"), ("lt", "Hueskos provincija"), ("lv", "Veskas province"), ("mk", "Уеска"), ("mr", "ह\u{941}सका प\u{94d}रा\u{902}त"), ("ms", "Wilayah Huesca"), ("nb", "Huesca"), ("nl", "Huesca"), ("no", "Huesca"), ("pl", "Huesca"), ("pt", "Huesca"), ("ro", "Provincia Huesca"), ("ru", "Уэска"), ("si", "හ\u{dd2}ය\u{dd4}ස\u{dca}ක\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Huesca"), ("sr", "Провинција Уеска"), ("sr_Latn", "Provincija Ueska"), ("sv", "Huesca"), ("sw", "Mkoa wa Huesca"), ("ta", "ஹஎஸ\u{bcd}கே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "హుయ\u{c46}స\u{c4d}క\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e39}สกา"), ("tr", "Huesca ili"), ("uk", "Уеска"), ("ur", "صوبہ ویسکا"), ("uz", "Huesca"), ("vi", "Huesca"), ("yue", "威斯卡省"), ("yue_Hans", "威斯卡省"), ("zh", "韋斯卡省")]),
                        unofficial_name_list: ["Huesca"].to_vec(),
                    }
                ),
                (
                    "IB",
                    Subdivision{
                        name: "IB",
                        country_alpha2: Alpha2::ES,
                        code: "IB",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Baleariese Eilande"), ("am", "ባሌያሪክ ደሴቶች"), ("ar", "منطقة جزر البليار"), ("az", "Balear adaları"), ("be", "Балеарскія астравы"), ("bg", "Балеарски острови"), ("bn", "বেল\u{9be}রিক আইল\u{9cd}য\u{9be}ন\u{9cd}ডস"), ("bs", "Balearska ostrva"), ("ca", "Illes Balears"), ("ccp", "𑄝𑄣𑄬𑄠𑄢\u{11128}𑄇\u{11134} 𑄃\u{11128}𑄌\u{11134}𑄣\u{11133}𑄠𑄚\u{11133}𑄓\u{11134}𑄥\u{11134}"), ("ceb", "Islas Baleares"), ("cs", "Baleáry"), ("cy", "Ynysoedd Balearig"), ("da", "Baleariske Øer"), ("de", "Balearische Inseln"), ("el", "Βαλεαρίδες Νήσοι"), ("en", "Balearic Islands"), ("es", "Islas Baleares"), ("et", "Baleaarid"), ("eu", "Balear Uharteak"), ("fa", "جزایر بالئاری"), ("fi", "Baleaarit"), ("fr", "Îles Baléares"), ("ga", "Na hOileáin Bhailéaracha"), ("gl", "Illas Baleares - Illes Balears"), ("gu", "બ\u{ac7}લ\u{ac7}રીક આઇલ\u{ac7}ન\u{acd}ડ\u{acd}સ"), ("he", "האיים הבלאריים"), ("hi", "ब\u{948}ल\u{947}रिक द\u{94d}वीपसम\u{942}ह"), ("hr", "Baleari"), ("hu", "Baleár-szigetek"), ("hy", "Բալեարյան կղզիներ"), ("id", "Kepulauan Balears"), ("is", "Baleareyjar"), ("it", "isole Baleari"), ("ja", "バレアレス諸島"), ("jv", "Kapuloan Baleares"), ("ka", "ბალეარის კუნძულები"), ("kk", "Балеар аралдары"), ("kn", "ಬಾಲೀರ\u{cbf}ಕ\u{ccd} ದ\u{ccd}ವೀಪಗಳು"), ("ko", "발레아레스 제도"), ("ky", "Балдар аралдары"), ("lt", "Balearų salos"), ("lv", "Baleāru Salas"), ("mk", "Балеарски Острови"), ("mn", "Балеарын арлууд"), ("mr", "बाल\u{947}आरिक द\u{94d}वीपसम\u{942}ह"), ("ms", "Kepulauan Balearic"), ("nb", "Balearene"), ("nl", "Balearen"), ("no", "Balearene"), ("pl", "Baleary"), ("pt", "Baleares"), ("ro", "Insulele Baleare"), ("ru", "Балеарские острова"), ("si", "බලේයර\u{dd2}ක\u{dca} ද\u{dd6}පත\u{dca}"), ("sk", "Baleáry"), ("sl", "Balearski otoki"), ("sq", "Ishujt Balearik"), ("sr", "Балеарска острва"), ("sr_Latn", "Balearska ostrva"), ("sv", "Balearerna"), ("sw", "Visiwa vya Baleari"), ("ta", "பலேரிக\u{bcd} த\u{bc0}வுகள\u{bcd}"), ("te", "బ\u{c3e}ల\u{c3f}య\u{c3e}ర\u{c3f}క\u{c4d} ద\u{c40}వులు"), ("th", "หม\u{e39}\u{e48}เกาะแบล\u{e35}แอร\u{e34}ก"), ("tr", "Balear Adaları"), ("uk", "Балеарські острови"), ("ur", "جزائر بلیبار"), ("uz", "Balear orollari"), ("vi", "Quần đảo Baleares"), ("yue", "巴利亞利群島"), ("yue_Hans", "巴利亚利群岛"), ("zh", "巴利阿里群島")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "J",
                    Subdivision{
                        name: "J",
                        country_alpha2: Alpha2::ES,
                        code: "J",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.7795941), longitude: Some(-3.784905699999999), max_latitude: Some(37.8040884), min_latitude: Some(37.75610030000001), max_longitude: Some(-3.7736049), min_longitude: Some(-3.8201251)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "خاين"), ("be", "Правінцыя Хаэн"), ("bg", "Хаен"), ("bn", "জিয\u{9bc}\u{9be}ন প\u{9cd}রদেশ"), ("ca", "província de Jaén"), ("ccp", "𑄎\u{11128}𑄚\u{11134}"), ("ceb", "Provincia de Jaén"), ("cs", "Provincie Jaén"), ("da", "Jaén"), ("de", "Jaén"), ("el", "Χαέν"), ("en", "Jaén"), ("es", "Provincia de Jaén"), ("et", "Jaéni provints"), ("eu", "Jaéngo probintzia"), ("fa", "استان خائن"), ("fi", "Jaén"), ("fr", "province de Jaén"), ("gl", "Provincia de Xaén"), ("gu", "જ\u{ac5}ન પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{948}एन प\u{94d}रा\u{902}त"), ("hr", "Jaén"), ("hu", "Jaén"), ("hy", "Խաեն"), ("id", "Provinsi Jaén"), ("is", "Jaén"), ("it", "Provincia di Jaén"), ("ja", "ハエン県"), ("ka", "ხაენის პროვინცია"), ("kk", "Хаэн"), ("kn", "ಜೇನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "하엔 주"), ("lt", "Chaeno provincija"), ("lv", "Haenas province"), ("mr", "ज\u{945}न प\u{94d}रा\u{902}त"), ("ms", "Wilayah Jaén"), ("nb", "Jaén"), ("nl", "Jaén"), ("no", "Jaén"), ("pl", "Jaén"), ("pt", "Jaén"), ("ro", "Provincia Jaén"), ("ru", "Хаэн"), ("si", "ජ\u{dcf}එන\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Jaén"), ("sr", "Покрајина Хаен"), ("sr_Latn", "Pokrajina Haen"), ("sv", "Jaén"), ("sw", "Mkoa wa Jaén"), ("ta", "ஜென\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జ\u{c47}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เจย\u{e31}น"), ("tr", "Jaén ili"), ("uk", "Хаен"), ("ur", "صوبہ خائن"), ("uz", "Jaén"), ("vi", "Jaén"), ("zh", "哈恩省")]),
                        unofficial_name_list: ["Jaén"].to_vec(),
                    }
                ),
                (
                    "L",
                    Subdivision{
                        name: "L",
                        country_alpha2: Alpha2::ES,
                        code: "L",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6175899), longitude: Some(0.6200146), max_latitude: Some(41.6396481), min_latitude: Some(41.5970365), max_longitude: Some(0.6497571999999999), min_longitude: Some(0.5901124999999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لاردة"), ("be", "правінцыя Льейда"), ("bn", "লিইড\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Lleida"), ("ccp", "𑄣𑄬\u{1112d}𑄓"), ("ceb", "Província de Lleida"), ("cs", "Provincie Lleida"), ("cy", "Talaith Lleida"), ("da", "Lleida"), ("de", "Provinz Lleida"), ("el", "Επαρχία της Λιέιδα"), ("en", "Lleida"), ("es", "Provincia de Lérida"), ("et", "Lleida provints"), ("eu", "Lleidako probintzia"), ("fa", "استان لریدا"), ("fi", "Lleida"), ("fr", "province de Lérida"), ("gl", "Provincia de Lleida"), ("gu", "લ\u{ac7}ઈડા પ\u{acd}રા\u{a82}ત"), ("hi", "लीडा प\u{94d}रा\u{902}त"), ("hr", "Lleida"), ("hu", "Lleida"), ("id", "Provinsi Lleida"), ("it", "Provincia di Lleida"), ("ja", "リェイダ県"), ("ka", "ლერიდის პროვინცია"), ("kk", "Лерида"), ("kn", "ಲ\u{cc6}ಲ\u{cc6}ಡಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "예이다 주"), ("lt", "Leridos provincija"), ("lv", "Ļeidas province"), ("mk", "Љејда"), ("mr", "ल\u{947}इडा प\u{94d}रा\u{902}त"), ("ms", "Lleida Province"), ("nb", "Lleida"), ("nl", "Lerida"), ("no", "Lleida"), ("pl", "Lleida"), ("pt", "Província de Lérida"), ("ro", "Provincia Lleida"), ("ru", "Льейда"), ("si", "ලෙය\u{dd2}ඩ\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Lleida"), ("sr", "Провинција Љеида"), ("sr_Latn", "Provincija Ljeida"), ("sv", "Lleida"), ("sw", "Mkoa wa Lérida"), ("ta", "ல\u{bcd}லேய\u{bcd}ட\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c47}య\u{c3f}డ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเยย\u{e4c}ดา"), ("tr", "Lérida ili"), ("uk", "Льєйда"), ("ur", "صوبہ لاریدا"), ("uz", "Lleida"), ("vi", "Lérida"), ("yue", "萊里達省"), ("yue_Hans", "莱里达省"), ("zh", "莱里达省")]),
                        unofficial_name_list: ["Lleida", "Lérida"].to_vec(),
                    }
                ),
                (
                    "LE",
                    Subdivision{
                        name: "LE",
                        country_alpha2: Alpha2::ES,
                        code: "LE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.5987263), longitude: Some(-5.5670959), max_latitude: Some(42.623248), min_latitude: Some(42.5839633), max_longitude: Some(-5.554772499999999), min_longitude: Some(-5.6000101)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ليون"), ("be", "Леон"), ("bg", "Леон"), ("ca", "província de Lleó"), ("ccp", "𑄣\u{11128}𑄠\u{11127}𑄚\u{11134}"), ("ceb", "Provincia de León"), ("cs", "Provincie León"), ("da", "León"), ("de", "Provinz León"), ("el", "Επαρχία Λεόν"), ("en", "León"), ("es", "Provincia de León"), ("et", "Leóni provints"), ("eu", "Leóngo probintzia"), ("fa", "استان لئن"), ("fi", "León"), ("fr", "province de León"), ("gl", "Provincia de León"), ("he", "תבנית:הקהילות האוטונומיות של ספרד"), ("hi", "सा\u{901}चा:स\u{94d}प\u{947}न क\u{947} प\u{94d}रान\u{94d}त"), ("hr", "León"), ("hu", "León"), ("hy", "Լեոնի շրջան"), ("id", "Provinsi León"), ("it", "provincia di León"), ("ja", "レオン県"), ("ka", "ლეონის პროვინცია"), ("kk", "Леон"), ("ko", "레온 주"), ("lt", "Leono provincija"), ("lv", "Leonas province"), ("ms", "Wilayah León"), ("nb", "León"), ("nl", "León"), ("no", "León"), ("pl", "León"), ("pt", "Leão"), ("ro", "Provincia León"), ("ru", "Леон"), ("sk", "León"), ("sl", "León"), ("sq", "Provinca León"), ("sr", "Покрајина Леон"), ("sr_Latn", "Pokrajina Leon"), ("sv", "León"), ("sw", "Mkoa wa León"), ("th", "แม\u{e48}แบบ:เขตการปกครองสเปน"), ("tr", "León ili"), ("uk", "Леон"), ("ur", "صوبہ لیون"), ("uz", "Leon"), ("vi", "León"), ("yue", "利昂省"), ("yue_Hans", "利昂省"), ("zh", "莱昂省")]),
                        unofficial_name_list: ["León"].to_vec(),
                    }
                ),
                (
                    "LO",
                    Subdivision{
                        name: "LO",
                        country_alpha2: Alpha2::ES,
                        code: "LO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.2870733), longitude: Some(-2.539603), max_latitude: Some(42.6442647), min_latitude: Some(41.9190339), max_longitude: Some(-1.6787014), min_longitude: Some(-3.1342713)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة لا ريوخا"), ("az", "Rioxa"), ("be", "Рыёха"), ("bg", "Ла Риоха"), ("bn", "ল\u{9be} রিওজ\u{9be}"), ("bs", "La Rioja"), ("ca", "La Rioja"), ("ccp", "𑄣 𑄢\u{11128}𑄃\u{1112e}𑄎 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("cs", "La Rioja"), ("cy", "La Rioja"), ("da", "La Rioja"), ("de", "La Rioja"), ("el", "Λα Ριόχα"), ("en", "La Rioja Province"), ("es", "La Rioja"), ("et", "La Rioja"), ("eu", "Errioxako Autonomia Erkidegoa"), ("fa", "لاریخا"), ("fi", "La Rioja"), ("fr", "La Rioja"), ("ga", "La Rioja"), ("gl", "A Rioxa"), ("gu", "લા રિયોજા"), ("he", "לה ריוחה"), ("hi", "ला रिओजा"), ("hr", "La Rioja"), ("hu", "La Rioja"), ("hy", "Ռիոխա"), ("id", "La Rioja"), ("is", "La Rioja"), ("it", "La Rioja"), ("ja", "ラ・リオハ州"), ("jv", "La Rioja"), ("ka", "ლა-რიოხა"), ("kk", "Риоха"), ("kn", "ಲಾ ರೈಜಾ"), ("ko", "라리오하 지방"), ("lt", "La Riocha"), ("lv", "Larjoha"), ("mk", "Риоха"), ("mr", "ला रियोहा"), ("ms", "La Rioja"), ("nb", "La Rioja"), ("nl", "La Rioja"), ("no", "La Rioja"), ("pl", "La Rioja"), ("pt", "La Rioja"), ("ro", "La Rioja"), ("ru", "Риоха"), ("si", "ල\u{dcf} රයෝජ\u{dcf}"), ("sl", "La Rioja"), ("sq", "La Rioja"), ("sr", "Риоха"), ("sr_Latn", "Rioha"), ("sv", "La Rioja"), ("sw", "La Rioja"), ("ta", "ல\u{bbe} ரியோஜ"), ("te", "ల\u{c3e} ర\u{c3f}య\u{c4b}జ\u{c3e}"), ("th", "แคว\u{e49}นลาร\u{e35}โอคา"), ("tr", "La Rioja"), ("uk", "Ла-Ріоха"), ("ur", "لا ریوخا"), ("uz", "La Rioja"), ("vi", "La Rioja"), ("yue", "拉里奧哈"), ("yue_Hans", "拉里奥哈"), ("zh", "拉里奥哈")]),
                        unofficial_name_list: ["La Rioja", "Logroño", "Rioja", "Rioja, La"].to_vec(),
                    }
                ),
                (
                    "LU",
                    Subdivision{
                        name: "LU",
                        country_alpha2: Alpha2::ES,
                        code: "LU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0097384), longitude: Some(-7.5567582), max_latitude: Some(43.03479859999999), min_latitude: Some(42.9897687), max_longitude: Some(-7.536812100000001), min_longitude: Some(-7.5761881)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "لوغو"), ("be", "Правінцыя Луга"), ("bn", "ল\u{9c1}গো প\u{9cd}রদেশ"), ("ca", "província de Lugo"), ("ccp", "𑄣𑄉\u{1112e}"), ("ceb", "Provincia de Lugo"), ("cs", "Provincie Lugo"), ("cy", "Talaith Lugo"), ("da", "Lugo"), ("de", "Provinz Lugo"), ("el", "Λούγκο"), ("en", "Lugo"), ("es", "Provincia de Lugo"), ("et", "Lugo provints"), ("eu", "Lugoko probintzia"), ("fa", "استان لوگو"), ("fi", "Lugo"), ("fr", "province de Lugo"), ("gl", "Provincia de Lugo"), ("gu", "લ\u{ac1}ગો પ\u{acd}રા\u{a82}ત"), ("hi", "ल\u{942}गो प\u{94d}रा\u{902}त"), ("hr", "Lugo"), ("hu", "Lugo"), ("hy", "Լուգո"), ("id", "Provinsi Lugo"), ("it", "Provincia di Lugo"), ("ja", "ルーゴ県"), ("ka", "ლუგოს პროვინცია"), ("kk", "Луго"), ("kn", "ಲುಗೊ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "루고 주"), ("lt", "Lugo provincija"), ("lv", "Lugo province"), ("mr", "ल\u{941}गो"), ("ms", "Wilayah Lugo"), ("nb", "Lugo"), ("nl", "Lugo"), ("no", "Lugo"), ("pl", "Lugo"), ("pt", "Lugo"), ("ro", "Provincia Lugo"), ("ru", "Луго"), ("si", "ල\u{dd4}ගෝ පළ\u{dcf}ත"), ("sq", "Provinca Lugo"), ("sr", "Покрајина Луго"), ("sr_Latn", "Pokrajina Lugo"), ("sv", "Lugo"), ("sw", "Mkoa wa Lugo"), ("ta", "லுகோ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "లూగ\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}โก"), ("tr", "Lugo ili"), ("uk", "Луго"), ("ur", "صوبہ لوگو"), ("uz", "Lugo"), ("vi", "Lugo"), ("zh", "卢戈省")]),
                        unofficial_name_list: ["Lugo"].to_vec(),
                    }
                ),
                (
                    "M",
                    Subdivision{
                        name: "M",
                        country_alpha2: Alpha2::ES,
                        code: "M",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.4167754), longitude: Some(-3.7037902), max_latitude: Some(40.5635903), min_latitude: Some(40.3120639), max_longitude: Some(-3.5249115), min_longitude: Some(-3.8341618)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Правінцыя Мадрыд"), ("ca", "província de Madrid"), ("ccp", "𑄟𑄓\u{11133}𑄢\u{11128}𑄖\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Provincia de Madrid"), ("en", "Madrid Province"), ("es", "provincia de Madrid"), ("fr", "province de Madrid"), ("ja", "マドリード県"), ("ru", "Мадрид"), ("sv", "Provincia de Madrid"), ("uk", "Провінція Мадрид")]),
                        unofficial_name_list: ["Madrid"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::ES,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(36.721261), longitude: Some(-4.4212655), max_latitude: Some(36.7575526), min_latitude: Some(36.6788914), max_longitude: Some(-4.3394965), min_longitude: Some(-4.5590373)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مالقة"), ("be", "Правінцыя Малага"), ("bg", "Малага"), ("ca", "província de Màlaga"), ("ccp", "𑄟𑄣𑄉"), ("ceb", "Provincia de Málaga"), ("cs", "Provincie Málaga"), ("da", "Málaga"), ("de", "Málaga"), ("el", "Μάλαγα"), ("en", "Málaga"), ("es", "provincia de Málaga"), ("et", "Málaga provints"), ("eu", "Málagako probintzia"), ("fa", "مالاگا"), ("fi", "Málaga"), ("fr", "province de Málaga"), ("ga", "Málaga"), ("gl", "Provincia de Málaga"), ("hr", "Razgovor:Málaga"), ("hu", "Málaga"), ("id", "Provinsi Málaga"), ("it", "Provincia di Malaga"), ("ja", "マラガ県"), ("ka", "მალაგის პროვინცია"), ("kk", "Малага"), ("ko", "말라가 주"), ("lt", "Malagos provincija"), ("lv", "Malagas province"), ("ms", "Wilayah Málaga"), ("nb", "Málaga"), ("nl", "Málaga"), ("no", "Málaga"), ("pl", "Malaga"), ("pt", "Málaga"), ("ro", "Provincia Málaga"), ("ru", "Малага"), ("sq", "Provinca Málaga"), ("sr", "Провинција Малага"), ("sr_Latn", "Provincija Malaga"), ("sv", "Málaga"), ("sw", "Mkoa wa Málaga"), ("tr", "Málaga ili"), ("uk", "Малага"), ("ur", "صوبہ مالقہ"), ("uz", "Málaga"), ("vi", "Málaga"), ("zh", "馬拉加省")]),
                        unofficial_name_list: ["Málaga"].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::ES,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة مرسية"), ("az", "Mursiya"), ("be", "Мурсія"), ("bg", "Мурсия"), ("bn", "ম\u{9c1}র\u{9cd}সিয\u{9bc}\u{9be} অঞ\u{9cd}চল"), ("bs", "Regija Murcia"), ("ca", "Regió de Múrcia"), ("ccp", "𑄟𑄢\u{11134}𑄥\u{11128}𑄠 𑄢𑄬𑄎\u{11133}𑄠\u{11127}"), ("ceb", "Región de Murcia"), ("cs", "Murcie"), ("cy", "Murcia (cymuned ymreolaethol)"), ("da", "Murcia"), ("de", "Autonome Gemeinschaft Region Murcia"), ("el", "Περιοχή της Μούρθια"), ("en", "Murcia Region"), ("es", "Región de Murcia"), ("et", "Murcia autonoomne piirkond"), ("eu", "Murtziako Eskualdea"), ("fa", "مورسیا"), ("fi", "Murcia"), ("fr", "région de Murcie"), ("ga", "Murcia"), ("gl", "Rexión de Murcia"), ("gu", "મ\u{ac1}ર\u{acd}સિયા પ\u{acd}રદ\u{ac7}શ"), ("he", "מורסיה"), ("hi", "मर\u{94d}सिया"), ("hr", "Regija Murcia"), ("hu", "Murcia tartomány"), ("hy", "Մուրսիա"), ("id", "Murcia"), ("is", "Múrsía"), ("it", "regione di Murcia"), ("ja", "ムルシア州"), ("jv", "Murcia"), ("ka", "მურსიის ავტონომიური გაერთიანება"), ("kk", "Мурсия"), ("kn", "ಮರ\u{ccd}ಸ\u{cbf}ಯ ಪ\u{ccd}ರದೇಶ"), ("ko", "무르시아 지방"), ("lt", "Mursija"), ("lv", "Mursijas reģions"), ("mk", "Мурсија"), ("mr", "म\u{941}र\u{94d}सिया"), ("ms", "Wilayah Murcia"), ("nb", "Murcia"), ("nl", "Murcia"), ("no", "Murcia"), ("pl", "Murcja"), ("pt", "Região de Múrcia"), ("ro", "Regiunea Murcia"), ("ru", "Мурсия"), ("si", "ම\u{dd4}ර\u{dd2}ක\u{dcf} කල\u{dcf}පය"), ("sq", "Rajoni Murcia"), ("sr", "Регион Мурсија"), ("sr_Latn", "Region Mursija"), ("sv", "Murcia"), ("ta", "முற\u{bcd}சிய\u{bbe} பகுதி"), ("te", "ముర\u{c4d}స\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}ంతం"), ("th", "แคว\u{e49}นม\u{e39}ร\u{e4c}เซ\u{e35}ย"), ("tr", "Murcia"), ("uk", "Мурсія"), ("ur", "ریجن مورکیا"), ("uz", "Murcia"), ("vi", "Vùng Murcia"), ("yue", "穆爾西亞自治區"), ("yue_Hans", "穆尔西亚自治区"), ("zh", "穆尔西亚自治区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "MD",
                    Subdivision{
                        name: "MD",
                        country_alpha2: Alpha2::ES,
                        code: "MD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة مدريد"), ("az", "Madrid cəmiyyəti"), ("be", "аўтаномная супольнасць Мадрыд"), ("bg", "Мадрид"), ("bs", "Zajednica Madrida"), ("ca", "Comunitat de Madrid"), ("ccp", "𑄟𑄓\u{11133}𑄢\u{11128}𑄖\u{11134} 𑄃\u{11127}𑄑\u{1112e}𑄚\u{1112e}𑄟𑄌\u{11134} 𑄇\u{11127}𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄑\u{11128}"), ("ceb", "Comunidad de Madrid"), ("cs", "Madridské autonomní společenství"), ("cy", "Madrid"), ("da", "Madrid"), ("de", "Autonome Gemeinschaft Madrid"), ("el", "Κοινότητα της Μαδρίτης"), ("en", "Madrid Autonomous Community"), ("es", "Comunidad de Madrid"), ("et", "Madridi autonoomne piirkond"), ("eu", "Madrilgo Erkidegoa"), ("fa", "مادرید"), ("fi", "Madrid"), ("fr", "communauté de Madrid"), ("ga", "Comhphobal Mhaidrid"), ("gl", "Comunidade de Madrid"), ("he", "מדריד"), ("hr", "Zajednica Madrida"), ("hu", "Madrid tartomány"), ("hy", "Մադրիդ"), ("id", "Madrid"), ("is", "Madríd (hérað)"), ("it", "comunità di Madrid"), ("ja", "マドリード州"), ("ka", "მადრიდის ავტონომიური გაერთიანება"), ("kk", "Мадрид"), ("ko", "마드리드 지방"), ("lt", "Madridas"), ("lv", "Madride"), ("mk", "Мадрид"), ("mr", "माद\u{94d}रिद"), ("ms", "Madrid"), ("nb", "Madrid"), ("nl", "Madrid"), ("no", "Madrid"), ("pl", "Madryt"), ("pt", "Comunidade de Madrid"), ("ro", "Madrid"), ("ru", "Мадрид²"), ("sk", "Madridské spoločenstvo"), ("sl", "avtonomna skupnost Madrid"), ("sq", "Komuniteti i Madridit"), ("sr", "Покрајина Мадрид"), ("sr_Latn", "Pokrajina Madrid"), ("sv", "Madrid"), ("th", "แคว\u{e49}นมาดร\u{e34}ด"), ("tr", "Madrid"), ("uk", "Мадрид"), ("uz", "Madrid"), ("vi", "Cộng đồng Madrid"), ("yue", "馬德里自治區"), ("yue_Hans", "马德里自治区"), ("zh", "马德里自治区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ML",
                    Subdivision{
                        name: "ML",
                        country_alpha2: Alpha2::ES,
                        code: "ML",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(35.2922775), longitude: Some(-2.9380973), max_latitude: Some(35.3030924), min_latitude: Some(35.2694508), max_longitude: Some(-2.9232595), min_longitude: Some(-2.9657935)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCityInNorthAfrica,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Melilla"), ("am", "ሜሊያ"), ("ar", "مليلية"), ("az", "Melilya"), ("be", "Мелілья"), ("bg", "Мелиля"), ("bn", "মেলিল\u{9be}"), ("bs", "Melilla"), ("ca", "Melilla"), ("ccp", "𑄟𑄬𑄣\u{11128}𑄣\u{11133}𑄦"), ("ceb", "Melilla"), ("cs", "Melilla"), ("cy", "Melilla"), ("da", "Melilla"), ("de", "Melilla"), ("el", "Μελίγια"), ("en", "Melilla"), ("es", "Melilla"), ("et", "Melilla"), ("eu", "Melilla"), ("fa", "ملیلیه"), ("fi", "Melilla"), ("fr", "Melilla"), ("ga", "Melilla"), ("gl", "Melilla"), ("gu", "મ\u{ac7}લિલા"), ("he", "מלייה"), ("hi", "म\u{947}लिला"), ("hr", "Melilla"), ("hu", "Melilla"), ("hy", "Մելիլյա"), ("id", "Melilla"), ("is", "Melilla"), ("it", "Melilla"), ("ja", "メリリャ"), ("jv", "Melilla"), ("ka", "მელილია"), ("kk", "Мелилья"), ("kn", "ಮ\u{cc6}ಲ\u{cbf}ಲ\u{ccd}ಲಾ"), ("ko", "멜리야"), ("lt", "Melilja"), ("lv", "Melilja"), ("mk", "Мелиља"), ("mr", "म\u{947}लिया"), ("ms", "Melilla"), ("nb", "Melilla"), ("nl", "Melilla"), ("no", "Melilla"), ("pl", "Melilla"), ("pt", "Melilla"), ("ro", "Melilla"), ("ru", "Мелилья"), ("si", "මෙල\u{dd2}ල\u{dca}ල\u{dcf}"), ("sk", "Melilla"), ("sl", "Melilla"), ("sq", "Melilla"), ("sr", "Мелиља"), ("sr_Latn", "Melilja"), ("sv", "Melilla"), ("sw", "Melilla"), ("ta", "மெலில\u{bcd}ல\u{bbe}"), ("te", "మ\u{c46}ల\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เมล\u{e35}ยา"), ("tr", "Melilla"), ("uk", "Мелілья"), ("ur", "ملیلہ"), ("vi", "Melilla"), ("yo", "Melilla"), ("yo_BJ", "Melilla"), ("yue", "梅利利亞"), ("yue_Hans", "梅利利亚"), ("zh", "梅利利亚"), ("zu", "IMelilla")]),
                        unofficial_name_list: ["Melilla"].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::ES,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.9922399), longitude: Some(-1.1306544), max_latitude: Some(38.0122586), min_latitude: Some(37.9649768), max_longitude: Some(-1.1066994), min_longitude: Some(-1.1533806)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "província de Múrcia"), ("ccp", "𑄟\u{1112a}𑄢\u{11134}𑄥\u{11128}𑄠"), ("en", "Murcia"), ("es", "provincia de Murcia"), ("fr", "province de Murcie"), ("ja", "ムルシア県")]),
                        unofficial_name_list: ["Murcia"].to_vec(),
                    }
                ),
                (
                    "NA",
                    Subdivision{
                        name: "NA",
                        country_alpha2: Alpha2::ES,
                        code: "NA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.6953909), longitude: Some(-1.6760691), max_latitude: Some(43.314792), min_latitude: Some(41.9098937), max_longitude: Some(-0.7239504), min_longitude: Some(-2.5000827)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ናቫራ"), ("ar", "منطقة نافارا²"), ("az", "Navarra²"), ("be", "аўтаномная супольнасць Навара"), ("bg", "Навара²"), ("bn", "ন\u{9be}ব\u{9be}র\u{9be}²"), ("bs", "Navara"), ("ca", "Navarra²"), ("ccp", "𑄚𑄞𑄢\u{11133}𑄦"), ("ceb", "Navarra"), ("cs", "Navarra²"), ("cy", "Nafarroa"), ("da", "Navarra²"), ("de", "Autonome Gemeinschaft Navarra²"), ("el", "Ναβάρρα²"), ("en", "Navarra"), ("es", "Navarra²"), ("et", "Navarra²"), ("eu", "Nafarroako Foru Erkidegoa²"), ("fa", "نابارا²"), ("fi", "Navarra²"), ("fr", "communauté forale de Navarre²"), ("ga", "Navarra"), ("gl", "Navarra²"), ("gu", "નાવાર\u{ac7}²"), ("he", "נווארה²"), ("hi", "नावारा²"), ("hr", "Navara²"), ("hu", "Navarra²"), ("hy", "Նավառա"), ("id", "Navarra²"), ("is", "Navarra²"), ("it", "Navarra²"), ("ja", "ナバラ州²"), ("ka", "ნავარა²"), ("kk", "Наварра"), ("kn", "ನವಾರ\u{ccd}ರ\u{cc6}²"), ("ko", "나바라 지방²"), ("lt", "Navara²"), ("lv", "Navarra²"), ("mk", "Навара"), ("mr", "नाबारा²"), ("ms", "Navarre²"), ("nb", "Navarra²"), ("nl", "Navarra²"), ("no", "Navarra²"), ("pl", "Nawarra²"), ("pt", "Navarra²"), ("ro", "Navarra²"), ("ru", "Наварра²"), ("si", "නවරේ²"), ("sk", "Navarra²"), ("sl", "Navarre²"), ("sq", "Navarre"), ("sr", "Навара²"), ("sr_Latn", "Navara²"), ("sv", "Navarra²"), ("ta", "நவரரே²"), ("te", "నవ\u{c3e}ర\u{c4d}²"), ("th", "แคว\u{e49}นนาวาร\u{e4c}²"), ("tr", "Navarra²"), ("uk", "Наварра²"), ("ur", "ناوار²"), ("uz", "Navarra"), ("vi", "Navarra²"), ("yue", "納華拉"), ("yue_Hans", "纳华拉"), ("zh", "納瓦拉²")]),
                        unofficial_name_list: ["Nafarroa", "Navarra", "Navarre"].to_vec(),
                    }
                ),
                (
                    "NC",
                    Subdivision{
                        name: "NC",
                        country_alpha2: Alpha2::ES,
                        code: "NC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ናቫራ²"), ("ar", "منطقة نافارا"), ("az", "Navarra"), ("be", "Навара, аўтаномная супольнасць"), ("bg", "Навара"), ("bn", "ন\u{9be}ব\u{9be}র\u{9be}"), ("bs", "Navara²"), ("ca", "Navarra"), ("ccp", "𑄚𑄞\u{11127}𑄢\u{11133}𑄦 𑄌𑄢\u{11134}𑄑𑄢\u{11134} 𑄇\u{11127}𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄑\u{11128}"), ("ceb", "Navarra²"), ("cs", "Navarra"), ("cy", "Nafarroa²"), ("da", "Navarra"), ("de", "Autonome Gemeinschaft Navarra"), ("el", "Ναβάρρα"), ("en", "Navarra Chartered Community"), ("es", "Navarra"), ("et", "Navarra"), ("eu", "Nafarroako Foru Erkidegoa"), ("fa", "نابارا"), ("fi", "Navarra"), ("fr", "communauté forale de Navarre"), ("ga", "Navarra²"), ("gl", "Navarra"), ("gu", "નાવાર\u{ac7}"), ("he", "נווארה"), ("hi", "नावारा"), ("hr", "Navara"), ("hu", "Navarra"), ("hy", "Նավարրա"), ("id", "Navarra"), ("is", "Navarra"), ("it", "Navarra"), ("ja", "ナバラ州"), ("ka", "ნავარა"), ("kk", "Наварра²"), ("kn", "ನವಾರ\u{ccd}ರ\u{cc6}"), ("ko", "나바라 지방"), ("lt", "Navara"), ("lv", "Navarra"), ("mk", "Навара²"), ("mr", "नाबारा"), ("ms", "Navarre"), ("nb", "Navarra"), ("nl", "Navarra"), ("no", "Navarra"), ("pl", "Nawarra"), ("pt", "Navarra"), ("ro", "Navarra"), ("ru", "Наварра"), ("si", "නවරේ"), ("sk", "Navarra"), ("sl", "Navarre"), ("sq", "Navarre²"), ("sr", "Навара"), ("sr_Latn", "Navara"), ("sv", "Navarra"), ("ta", "நவரரே"), ("te", "నవ\u{c3e}ర\u{c4d}"), ("th", "แคว\u{e49}นนาวาร\u{e4c}"), ("tr", "Navarra"), ("uk", "Наварра"), ("ur", "ناوار"), ("uz", "Navarra²"), ("vi", "Navarra"), ("yue", "納華拉²"), ("yue_Hans", "纳华拉²"), ("zh", "納瓦拉")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "O",
                    Subdivision{
                        name: "O",
                        country_alpha2: Alpha2::ES,
                        code: "O",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.3613953), longitude: Some(-5.8593267), max_latitude: Some(43.6665323), min_latitude: Some(42.8825428), max_longitude: Some(-4.5105944), min_longitude: Some(-7.1824889)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Астурыя²"), ("ca", "província d’Astúries"), ("ccp", "𑄃\u{11127}𑄌\u{11134}𑄑\u{11128}𑄅\u{1112a}𑄢\u{11128}𑄠𑄌\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("en", "Asturias Province"), ("es", "provincia de Asturias"), ("gl", "provincia de Asturias"), ("hy", "Աստուրիա²"), ("ru", "Астурия²"), ("uk", "Астурія²")]),
                        unofficial_name_list: ["Asturias", "Oviedo"].to_vec(),
                    }
                ),
                (
                    "OR",
                    Subdivision{
                        name: "OR",
                        country_alpha2: Alpha2::ES,
                        code: "OR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.33578929999999), longitude: Some(-7.863880999999998), max_latitude: Some(42.35829409999999), min_latitude: Some(42.3184311), max_longitude: Some(-7.8470722), min_longitude: Some(-7.880299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "أورينسي"), ("be", "Правінцыя Арэнсэ"), ("bg", "Оренсе"), ("bn", "ওরেন\u{9cd}স প\u{9cd}রদেশ"), ("ca", "província d’Ourense"), ("ccp", "𑄃\u{1112f}𑄢𑄬𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Provincia de Ourense"), ("cs", "Provincie Ourense"), ("da", "Ourense"), ("de", "Provinz Ourense"), ("el", "Ουρένσε"), ("en", "Ourense"), ("es", "Provincia de Orense"), ("et", "Ourense provints"), ("eu", "Ourenseko probintzia"), ("fa", "استان اورنسه"), ("fi", "Ourense"), ("fr", "province d’Ourense"), ("ga", "Ourense"), ("gl", "Provincia de Ourense"), ("gu", "ઓર\u{ac7}ન\u{acd}સ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "ओउर\u{947}\u{902}स प\u{94d}रा\u{902}त"), ("hr", "Ourense"), ("hu", "Ourense"), ("id", "Provinsi Ourense"), ("it", "Provincia di Ourense"), ("ja", "オウレンセ県"), ("ka", "ორენსეს პროვინცია"), ("kk", "Оренсе"), ("kn", "ಓರ\u{cc6}ನ\u{ccd}ಸ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오렌세 주"), ("lt", "Orensės provincija"), ("lv", "Ourenses province"), ("mr", "ओरन\u{947}स\u{947} प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ourense"), ("nb", "Ourense"), ("nl", "Ourense"), ("no", "Ourense"), ("pl", "Ourense"), ("pt", "Ourense"), ("ro", "Provincia Ourense"), ("ru", "Оренсе"), ("si", "අවරෙන\u{dca}ස\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Ourense"), ("sr", "Покрајина Оренсе"), ("sr_Latn", "Pokrajina Orense"), ("sv", "Orense"), ("sw", "Mkoa wa Orense"), ("ta", "ஓரன\u{bcd}ஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఔర\u{c46}న\u{c4d}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e39}เรนเซ"), ("tr", "Ourense ili"), ("uk", "Оренсе"), ("ur", "صوبہ اورینسے"), ("uz", "Ourense"), ("vi", "Ourense"), ("zh", "奥伦塞省")]),
                        unofficial_name_list: ["Orense", "Ourense"].to_vec(),
                    }
                ),
                (
                    "P",
                    Subdivision{
                        name: "P",
                        country_alpha2: Alpha2::ES,
                        code: "P",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.0096857), longitude: Some(-4.5288016), max_latitude: Some(42.0289194), min_latitude: Some(41.98871279999999), max_longitude: Some(-4.505785599999999), min_longitude: Some(-4.5481221)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بالنثيا"), ("be", "правінцыя Паленсія"), ("bg", "Паленсия"), ("bn", "প\u{9cd}য\u{9be}লেসিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Palència"), ("ccp", "𑄛\u{11127}𑄣𑄬𑄚\u{11134}𑄥\u{11128}𑄠"), ("ceb", "Provincia de Palencia"), ("cs", "Provincie Palencia"), ("da", "Palencia"), ("de", "Provinz Palencia"), ("el", "Επαρχία της Παλένθια"), ("en", "Palencia"), ("es", "Provincia de Palencia"), ("et", "Palencia provints"), ("eu", "Palentziako probintzia"), ("fa", "استان پالنسیا"), ("fi", "Palencia"), ("fr", "province de Palencia"), ("gl", "Provincia de Palencia"), ("gu", "પ\u{ac7}લ\u{ac7}ન\u{acd}સિયા પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{948}ल\u{947}न\u{94d}सिया प\u{94d}रा\u{902}त"), ("hr", "Palencia"), ("hu", "Palencia"), ("id", "Provinsi Palencia"), ("it", "provincia di Palencia"), ("ja", "パレンシア県"), ("ka", "პალენსიის პროვინცია"), ("kk", "Паленсия"), ("kn", "ಪಾಲ\u{cc6}ನ\u{ccd}ಸ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "팔렌시아 주"), ("lt", "Palensijos provincija"), ("lv", "Palensijas province"), ("mr", "प\u{945}ल\u{947}न\u{94d}सीया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Palencia"), ("nb", "Palencia"), ("nl", "Palencia"), ("no", "Palencia"), ("pl", "Palencia"), ("pt", "Palência"), ("ro", "Provincia Palencia"), ("ru", "Паленсия"), ("si", "පලේන\u{dca}ස\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Palencia"), ("sr", "Покрајина Паленсија"), ("sr_Latn", "Pokrajina Palensija"), ("sv", "Palencia"), ("sw", "Mkoa wa Palencia"), ("ta", "பலென\u{bcd}சிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}ల\u{c3f}న\u{c47}స\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปาเลนเซ\u{e35}ย"), ("tr", "Palencia ili"), ("uk", "Паленсія"), ("ur", "صوبہ پالینسیا"), ("uz", "Palencia"), ("vi", "Palencia"), ("zh", "帕伦西亚省")]),
                        unofficial_name_list: ["Palencia"].to_vec(),
                    }
                ),
                (
                    "PM",
                    Subdivision{
                        name: "PM",
                        country_alpha2: Alpha2::ES,
                        code: "PM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.5341789), longitude: Some(2.8577105), max_latitude: Some(40.0945744), min_latitude: Some(38.6403875), max_longitude: Some(4.3277839), min_longitude: Some(1.1572495)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Baleariese Eilande²"), ("am", "ባሌያሪክ ደሴቶች²"), ("ar", "منطقة جزر البليار²"), ("az", "Balear adaları²"), ("be", "Балеарскія астравы²"), ("bg", "Балеарски острови²"), ("bn", "বেল\u{9be}রিক আইল\u{9cd}য\u{9be}ন\u{9cd}ডস²"), ("bs", "Balearska ostrva²"), ("ca", "Illes Balears²"), ("ccp", "𑄝𑄣\u{11128}𑄠𑄢\u{11134}𑄥\u{11134} 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Islas Baleares²"), ("cs", "Baleáry²"), ("cy", "Ynysoedd Balearig²"), ("da", "Baleariske Øer²"), ("de", "Balearische Inseln²"), ("el", "Βαλεαρίδες Νήσοι²"), ("en", "Balears Province"), ("es", "Islas Baleares²"), ("et", "Baleaarid²"), ("eu", "Balear Uharteak²"), ("fa", "جزایر بالئاری²"), ("fi", "Baleaarit²"), ("fr", "Îles Baléares²"), ("ga", "Na hOileáin Bhailéaracha²"), ("gl", "Illas Baleares"), ("gu", "બ\u{ac7}લ\u{ac7}રીક આઇલ\u{ac7}ન\u{acd}ડ\u{acd}સ²"), ("he", "האיים הבלאריים²"), ("hi", "ब\u{948}ल\u{947}रिक द\u{94d}वीपसम\u{942}ह²"), ("hr", "Baleari²"), ("hu", "Baleár-szigetek²"), ("hy", "Բալեարյան կղզիներ²"), ("id", "Kepulauan Balears²"), ("is", "Baleareyjar²"), ("it", "isole Baleari²"), ("ja", "バレアレス諸島²"), ("jv", "Kapuloan Baleares²"), ("ka", "ბალეარის კუნძულები²"), ("kk", "Балеар аралдары²"), ("kn", "ಬಾಲೀರ\u{cbf}ಕ\u{ccd} ದ\u{ccd}ವೀಪಗಳು²"), ("ko", "발레아레스 제도²"), ("ky", "Балдар аралдары²"), ("lt", "Balearų salos²"), ("lv", "Baleāru Salas²"), ("mk", "Балеарски Острови²"), ("mn", "Балеарын арлууд²"), ("mr", "बाल\u{947}आरिक द\u{94d}वीपसम\u{942}ह²"), ("ms", "Kepulauan Balearic²"), ("nb", "Balearene²"), ("nl", "Balearen²"), ("no", "Balearene²"), ("pl", "Baleary²"), ("pt", "Baleares²"), ("ro", "Insulele Baleare²"), ("ru", "Балеарские острова²"), ("si", "බලේයර\u{dd2}ක\u{dca} ද\u{dd6}පත\u{dca}²"), ("sk", "Baleáry²"), ("sl", "Balearski otoki²"), ("sq", "Ishujt Balearik²"), ("sr", "Балеарска острва²"), ("sr_Latn", "Balearska ostrva²"), ("sv", "Balearerna²"), ("sw", "Visiwa vya Baleari²"), ("ta", "பலேரிக\u{bcd} த\u{bc0}வுகள\u{bcd}²"), ("te", "బ\u{c3e}ల\u{c3f}య\u{c3e}ర\u{c3f}క\u{c4d} ద\u{c40}వులు²"), ("th", "หม\u{e39}\u{e48}เกาะแบล\u{e35}แอร\u{e34}ก²"), ("tr", "Balear Adaları²"), ("uk", "Балеарські острови²"), ("ur", "جزائر بلیبار²"), ("uz", "Balear orollari²"), ("vi", "Quần đảo Baleares²"), ("yue", "巴利亞利群島²"), ("yue_Hans", "巴利亚利群岛²"), ("zh", "巴利阿里群島²")]),
                        unofficial_name_list: ["Baleares", "Balearic Islands", "Balears, Illes", "Illes Balears", "Islas Baleares"].to_vec(),
                    }
                ),
                (
                    "PO",
                    Subdivision{
                        name: "PO",
                        country_alpha2: Alpha2::ES,
                        code: "PO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.4298846), longitude: Some(-8.6446202), max_latitude: Some(42.4517667), min_latitude: Some(42.4159549), max_longitude: Some(-8.613833699999999), min_longitude: Some(-8.6648576)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بونتيفيدرا"), ("be", "правінцыя Пантэведра"), ("bn", "পন\u{9cd}টেভেদ\u{9cd}র\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Pontevedra"), ("ccp", "𑄛\u{1112e}𑄚\u{11134}𑄑𑄬𑄞𑄬𑄓\u{11133}𑄢"), ("ceb", "Provincia de Pontevedra"), ("cs", "Provincie Pontevedra"), ("da", "Pontevedra"), ("de", "Provinz Pontevedra"), ("el", "Επαρχία της Ποντεβέδρα"), ("en", "Pontevedra"), ("es", "Provincia de Pontevedra"), ("et", "Pontevedra provints"), ("eu", "Pontevedrako probintzia"), ("fa", "استان پنتبدرا"), ("fi", "Pontevedra"), ("fr", "province de Pontevedra"), ("gl", "Provincia de Pontevedra"), ("gu", "પો\u{a82}ટ\u{ac7}વ\u{ac7}ડ\u{acd}રા પ\u{acd}રા\u{a82}ત"), ("hi", "पो\u{902}ट\u{947}व\u{947}ड\u{94d}रा प\u{94d}रा\u{902}त"), ("hr", "Pontevedra"), ("hu", "Pontevedra"), ("id", "Provinsi Pontevedra"), ("it", "Provincia di Pontevedra"), ("ja", "ポンテベドラ県"), ("ka", "პონტევედრის პროვინცია"), ("kk", "Понтеведра"), ("kn", "ಪೊಂಟ\u{cc6}ವೇದ\u{ccd}ರ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "폰테베드라 주"), ("lt", "Pontevedros provincija"), ("lv", "Pontebedras province"), ("mr", "पो\u{902}ट\u{947}व\u{947}ड\u{94d}रा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Pontevedra"), ("nb", "Pontevedra"), ("nl", "Pontevedra"), ("no", "Pontevedra"), ("pl", "Pontevedra"), ("pt", "Pontevedra"), ("ro", "Provincia Pontevedra"), ("ru", "Понтеведра"), ("si", "පොන\u{dca}ටෙවෙද\u{dca}\u{200d}ර\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Pontevedra"), ("sr", "Покрајина Понтеведра"), ("sr_Latn", "Pokrajina Pontevedra"), ("sv", "Pontevedra"), ("sw", "Mkoa wa Pontevedra"), ("ta", "பொன\u{bcd}டேவெட\u{bcd}ற\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c3e}ంట\u{c3f}వ\u{c46}ద\u{c4d}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดปอนเตเบดรา"), ("tr", "Pontevedra ili"), ("uk", "провінція Понтеведра"), ("ur", "صوبہ پونتیبیدرا"), ("uz", "Pontevedra"), ("vi", "Pontevedra"), ("zh", "蓬特韋德拉省")]),
                        unofficial_name_list: ["Pontevedra"].to_vec(),
                    }
                ),
                (
                    "PV",
                    Subdivision{
                        name: "PV",
                        country_alpha2: Alpha2::ES,
                        code: "PV",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Outonome Gemeenskap van die Baskeland"), ("am", "ባስክ ሀገር"), ("ar", "منطقة إقليم الباسك"), ("az", "Basklar ölkəsi"), ("be", "Краіна Баскаў"), ("bg", "Баска автономна област"), ("bs", "Baskija (autonomna zajednica)"), ("ca", "Comunitat autònoma del País Basc"), ("ccp", "𑄥𑄌\u{11134}𑄇\u{1112a} 𑄇𑄚\u{11134}𑄑\u{11133}𑄢\u{11128}"), ("cs", "Autonomní společenství Baskicko"), ("cy", "Cymuned Ymreolaethol Gwlad y Basg"), ("da", "Baskerlandet"), ("de", "Autonome Gemeinschaft Baskenland"), ("el", "Χώρα των Βάσκων"), ("en", "Basque Country"), ("es", "País Vasco"), ("et", "Baski autonoomne piirkond"), ("eu", "Euskal Autonomia Erkidegoa"), ("fa", "سرزمین باسک"), ("fi", "Baskimaa"), ("fr", "Pays basque"), ("ga", "Tír na mBascach Theas"), ("gl", "País Vasco"), ("he", "חבל הבסקים"), ("hi", "बास\u{94d}क प\u{94d}रद\u{947}श"), ("hr", "Baskija"), ("hu", "Baszkföld"), ("hy", "Բասկերի երկիր"), ("id", "País Vasco"), ("is", "Baskaland"), ("it", "Paesi Baschi"), ("ja", "バスク州"), ("jv", "País Vasco"), ("ka", "ბასკეთი"), ("kk", "Басктар елі"), ("ko", "바스크 지방"), ("ky", "Басктар Өлкөсү"), ("lt", "Baskija"), ("lv", "Basku Zeme"), ("mk", "Баскија"), ("mn", "Баск орон"), ("mr", "पाईज बास\u{94d}को"), ("ms", "Negara Basque"), ("nb", "Baskerland"), ("nl", "Baskenland"), ("no", "Baskerland"), ("pl", "Kraj Basków"), ("pt", "Comunidade autónoma do País Basco"), ("ro", "Țara Bascilor"), ("ru", "Страна Басков"), ("sk", "Baskicko"), ("sl", "Baskija"), ("sq", "Baskia"), ("sr", "Баскија"), ("sr_Latn", "Baskija"), ("sv", "Regionen Baskien"), ("ta", "ப\u{bbe}சுக\u{bcd}கு ந\u{bbe}டு"), ("th", "แคว\u{e49}นบาสก\u{e4c}"), ("tr", "Bask Bölgesi"), ("uk", "Країна Басків"), ("ur", "باسک ملک"), ("uz", "Basklar mamlakati"), ("vi", "Xứ Basque"), ("yue", "巴斯克自治區"), ("yue_Hans", "巴斯克自治区"), ("zh", "巴斯克自治區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::ES,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة لا ريوخا²"), ("az", "Rioxa²"), ("be", "Рыёха²"), ("bg", "Ла Риоха²"), ("bn", "ল\u{9be} রিওজ\u{9be}²"), ("bs", "La Rioja²"), ("ca", "La Rioja²"), ("ccp", "𑄣 𑄢\u{11128}𑄃\u{1112e}𑄎"), ("cs", "La Rioja²"), ("cy", "La Rioja²"), ("da", "La Rioja²"), ("de", "La Rioja²"), ("el", "Λα Ριόχα²"), ("en", "La Rioja"), ("es", "La Rioja²"), ("et", "La Rioja²"), ("eu", "Errioxako Autonomia Erkidegoa²"), ("fa", "لاریخا²"), ("fi", "La Rioja²"), ("fr", "La Rioja²"), ("ga", "La Rioja²"), ("gl", "A Rioxa²"), ("gu", "લા રિયોજા²"), ("he", "לה ריוחה²"), ("hi", "ला रिओजा²"), ("hr", "La Rioja²"), ("hu", "La Rioja²"), ("hy", "Ռիոխա²"), ("id", "La Rioja²"), ("is", "La Rioja²"), ("it", "La Rioja²"), ("ja", "ラ・リオハ州²"), ("jv", "La Rioja²"), ("ka", "ლა-რიოხა²"), ("kk", "Риоха²"), ("kn", "ಲಾ ರೈಜಾ²"), ("ko", "라리오하 지방²"), ("lt", "La Riocha²"), ("lv", "Larjoha²"), ("mk", "Риоха²"), ("mr", "ला रियोहा²"), ("ms", "La Rioja²"), ("nb", "La Rioja²"), ("nl", "La Rioja²"), ("no", "La Rioja²"), ("pl", "La Rioja²"), ("pt", "La Rioja²"), ("ro", "La Rioja²"), ("ru", "Риоха²"), ("si", "ල\u{dcf} රයෝජ\u{dcf}²"), ("sl", "La Rioja²"), ("sq", "La Rioja²"), ("sr", "Риоха²"), ("sr_Latn", "Rioha²"), ("sv", "La Rioja²"), ("sw", "La Rioja²"), ("ta", "ல\u{bbe} ரியோஜ²"), ("te", "ల\u{c3e} ర\u{c3f}య\u{c4b}జ\u{c3e}²"), ("th", "แคว\u{e49}นลาร\u{e35}โอคา²"), ("tr", "La Rioja²"), ("uk", "Ла-Ріоха²"), ("ur", "لا ریوخا²"), ("uz", "La Rioja²"), ("vi", "La Rioja²"), ("yue", "拉里奧哈²"), ("yue_Hans", "拉里奥哈²"), ("zh", "拉里奥哈²")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "S",
                        country_alpha2: Alpha2::ES,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.1828396), longitude: Some(-3.9878427), max_latitude: Some(43.5136929), min_latitude: Some(42.75804979999999), max_longitude: Some(-3.149652), min_longitude: Some(-4.8517782)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "ካንታብርያ²"), ("ar", "منطقة كانتابريا²"), ("az", "Kantabriya²"), ("be", "Кантабрыя²"), ("bg", "Кантабрия²"), ("bn", "ক\u{9be}ন\u{9cd}ত\u{9be}ব\u{9cd}রিয\u{9bc}\u{9be}²"), ("bs", "Kantabrija²"), ("ca", "Cantàbria²"), ("ccp", "𑄇\u{11133}𑄠𑄚\u{11134}𑄑𑄝\u{11133}𑄢\u{11128}𑄠 𑄛\u{11133}𑄢\u{11127}𑄞\u{11128}𑄚\u{11134}𑄥\u{11134}"), ("ceb", "Cantabria²"), ("cs", "Kantábrie²"), ("cy", "Cantabria²"), ("da", "Kantabrien²"), ("de", "Autonome Gemeinschaft Kantabrien²"), ("el", "Κανταβρία²"), ("en", "Cantabria Province"), ("es", "Cantabria²"), ("et", "Kantaabria²"), ("eu", "Kantabria²"), ("fa", "کانتابریا²"), ("fi", "Kantabria²"), ("fr", "Cantabrie²"), ("ga", "Cantabria²"), ("gl", "Cantabria²"), ("gu", "કાન\u{acd}તાબ\u{acd}રિયા²"), ("he", "קנטבריה²"), ("hi", "क\u{902}ट\u{948}ब\u{94d}रिया²"), ("hr", "Kantabrija²"), ("hu", "Kantábria²"), ("hy", "Կանտաբրիա²"), ("id", "Cantabria²"), ("is", "Kantabría²"), ("it", "Cantabria²"), ("ja", "カンタブリア州²"), ("ka", "კანტაბრია²"), ("kk", "Кантабрия²"), ("kn", "ಕ\u{ccd}ಯಾಂಥಬ\u{ccd}ರ\u{cbf}ಯಾ²"), ("ko", "칸타브리아 지방²"), ("lt", "Kantabrija²"), ("lv", "Kantabrija²"), ("mk", "Кантабрија²"), ("mr", "का\u{902}ताब\u{94d}रिया²"), ("ms", "Cantabria²"), ("nb", "Cantabria²"), ("nl", "Cantabrië²"), ("no", "Cantabria²"), ("pl", "Kantabria²"), ("pt", "Cantábria²"), ("ro", "Cantabria²"), ("ru", "Кантабрия²"), ("si", "කැන\u{dca}ටබ\u{dca}ර\u{dd2}ය\u{dcf}²"), ("sk", "Kantábria²"), ("sq", "Cantabria²"), ("sr", "Кантабрија²"), ("sr_Latn", "Kantabrija²"), ("sv", "Kantabrien²"), ("ta", "க\u{bbe}ந\u{bcd}த\u{bbe}பிரிய\u{bbe}²"), ("te", "క\u{c3e}ంట\u{c3e}బ\u{c4d}ర\u{c3f}య\u{c3e}²"), ("th", "แคว\u{e49}นก\u{e31}นตาเบร\u{e35}ย²"), ("tr", "Kantabria²"), ("uk", "Кантабрія²"), ("ur", "کانتابریا²"), ("uz", "Cantabria²"), ("vi", "Cantabria²"), ("yue", "坎塔布里亞²"), ("yue_Hans", "坎塔布里亚²"), ("zh", "坎塔布里亚²")]),
                        unofficial_name_list: ["Cantabria", "Santander"].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::ES,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9701039), longitude: Some(-5.663539699999999), max_latitude: Some(40.9850971), min_latitude: Some(40.9417799), max_longitude: Some(-5.6312039), min_longitude: Some(-5.707220299999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شلمنقة"), ("be", "правінцыя Саламанка"), ("bg", "Саламанка"), ("bn", "স\u{9be}ল\u{9be}ম\u{9be}নক\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Salamanca"), ("ccp", "𑄥𑄣𑄟𑄚\u{11134}𑄇"), ("ceb", "Provincia de Salamanca"), ("cs", "Provincie Salamanca"), ("da", "Salamanca"), ("de", "Provinz Salamanca"), ("el", "Σαλαμάνκα"), ("en", "Salamanca"), ("es", "Provincia de Salamanca"), ("et", "Salamanca provints"), ("eu", "Salamancako probintzia"), ("fa", "سالامانکا"), ("fi", "Salamanca"), ("fr", "province de Salamanque"), ("gl", "Provincia de Salamanca"), ("gu", "સાલામા\u{a82}કા પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}लाम\u{948}\u{902}का प\u{94d}रा\u{902}त"), ("hu", "Salamanca"), ("id", "Provinsi Salamanca"), ("it", "Provincia di Salamanca"), ("ja", "サラマンカ県"), ("ka", "სალამანკის პროვინცია"), ("kk", "Саламанка"), ("kn", "ಸಲಾಮಾಂಕಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "살라망카 주"), ("lt", "Salamankos provincija"), ("lv", "Salamankas province"), ("mr", "स\u{945}लम\u{947}\u{902}का प\u{94d}रा\u{902}त"), ("ms", "Wilayah Salamanca"), ("nb", "Salamanca"), ("nl", "Salamanca"), ("no", "Salamanca"), ("pl", "Salamanka"), ("pt", "Salamanca"), ("ro", "Provincia Salamanca"), ("ru", "Саламанка"), ("si", "සලමන\u{dca}ස\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Salamanca"), ("sr", "Провинција Саламанка"), ("sr_Latn", "Provincija Salamanka"), ("sv", "Salamanca"), ("sw", "Mkoa wa Salamanca"), ("ta", "ச\u{bbe}லம\u{bbe}க ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "సలమంక\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาลาม\u{e31}สกา"), ("tr", "Salamanca ili"), ("uk", "Саламанка"), ("ur", "صوبہ سالامانکا"), ("uz", "Salamanka"), ("vi", "Salamanca"), ("yue", "薩拉曼卡省"), ("yue_Hans", "萨拉曼卡省"), ("zh", "萨拉曼卡省")]),
                        unofficial_name_list: ["Salamanca"].to_vec(),
                    }
                ),
                (
                    "SE",
                    Subdivision{
                        name: "SE",
                        country_alpha2: Alpha2::ES,
                        code: "SE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.3890924), longitude: Some(-5.9844589), max_latitude: Some(37.4355212), min_latitude: Some(37.3152203), max_longitude: Some(-5.8884587), min_longitude: Some(-6.0216578)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إشبيلية"), ("be", "правінцыя Севілья"), ("bg", "Севиля"), ("bn", "সেভিলি প\u{9cd}রদেশ"), ("bs", "Sevilla"), ("ca", "província de Sevilla"), ("ccp", "𑄥𑄬𑄞\u{11128}𑄣𑄬"), ("ceb", "Provincia de Sevilla"), ("cs", "Provincie Sevilla"), ("da", "Sevilla"), ("de", "Sevilla"), ("el", "Σεβίλλη"), ("en", "Seville"), ("es", "Provincia de Sevilla"), ("et", "Sevilla provints"), ("eu", "Sevillako probintzia"), ("fa", "سبیا"), ("fi", "Sevilla"), ("fr", "province de Séville"), ("gl", "Provincia de Sevilla"), ("gu", "સ\u{ac7}વિલ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{947}विल प\u{94d}रा\u{902}त"), ("hr", "Sevilla"), ("hu", "Sevilla"), ("hy", "Սևիլյա"), ("id", "Provinsi Sevilla"), ("it", "Provincia di Siviglia"), ("ja", "セビリア県"), ("ka", "სევილიის პროვინცია"), ("kk", "Севилья"), ("kn", "ಸ\u{cc6}ವ\u{cbf}ಲ\u{ccd}ಲ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "세비야 주"), ("lt", "Sevilijos provincija"), ("lv", "Seviljas province"), ("mr", "सिविल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Seville"), ("nb", "Sevilla"), ("nl", "Sevilla"), ("no", "Sevilla"), ("pl", "Sewilla"), ("pt", "Província de Sevilha"), ("ro", "Provincia Sevilla"), ("ru", "Севилья"), ("si", "සෙව\u{dd2}ල\u{dd2} පළ\u{dcf}ත"), ("sq", "Provinca Sevilla"), ("sr", "Покрајина Севиља"), ("sr_Latn", "Pokrajina Sevilja"), ("sv", "Sevilla"), ("sw", "Mkoa wa Sevilla"), ("ta", "செவில\u{bcd}லே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}వ\u{c3f}ల\u{c4d}ల\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเซบ\u{e35}ยา"), ("tr", "Sevilla ili"), ("uk", "Севілья"), ("ur", "صوبہ اشبیلیہ"), ("uz", "Sevilla"), ("vi", "Sevilla"), ("yue", "西維爾省"), ("yue_Hans", "西维尔省"), ("zh", "塞維利亞省")]),
                        unofficial_name_list: ["Sevilla"].to_vec(),
                    }
                ),
                (
                    "SG",
                    Subdivision{
                        name: "SG",
                        country_alpha2: Alpha2::ES,
                        code: "SG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.9429032), longitude: Some(-4.108806899999999), max_latitude: Some(40.9625017), min_latitude: Some(40.9199986), max_longitude: Some(-4.090816999999999), min_longitude: Some(-4.1335868)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "شقوبية"), ("be", "Сеговія"), ("bg", "Сеговия"), ("ca", "província de Segòvia"), ("ccp", "𑄥𑄬𑄉\u{1112e}𑄞\u{11128}𑄠"), ("ceb", "Provincia de Segovia"), ("cs", "Provincie Segovia"), ("da", "Segovia"), ("de", "Provinz Segovia"), ("el", "Επαρχία της Σεγόβια"), ("en", "Segovia"), ("es", "Provincia de Segovia"), ("et", "Segovia provints"), ("eu", "Segoviako probintzia"), ("fa", "سگبیا (استان)"), ("fi", "Segovia"), ("fr", "province de Ségovie"), ("gl", "Provincia de Segovia"), ("hr", "Segovia"), ("hu", "Segovia"), ("id", "Provinsi Segovia"), ("it", "provincia di Segovia"), ("ja", "セゴビア県"), ("ka", "სეგოვიის პროვინცია"), ("kk", "Сеговия"), ("ko", "세고비아 주"), ("lt", "Segovijos provincija"), ("lv", "Segovijas province"), ("ms", "Wilayah Segovia"), ("nb", "Segovia"), ("nl", "Segovia"), ("no", "Segovia"), ("pl", "Segowia"), ("pt", "Segóvia"), ("ro", "Provincia Segovia"), ("ru", "Сеговия"), ("sq", "Provinca Segovia"), ("sr", "Покрајина Сеговија"), ("sr_Latn", "Pokrajina Segovija"), ("sv", "Segovia"), ("sw", "Mkoa wa Segovia"), ("tr", "Segovia ili"), ("uk", "Сеговія"), ("ur", "صوبہ سیگوبیا"), ("uz", "Segovia"), ("vi", "Segovia"), ("zh", "塞哥維亞省")]),
                        unofficial_name_list: ["Segovia"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::ES,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.7665972), longitude: Some(-2.4790306), max_latitude: Some(41.7822829), min_latitude: Some(41.7519601), max_longitude: Some(-2.455352), min_longitude: Some(-2.4938061)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Provinsie Soria"), ("ar", "سوريا"), ("be", "Правінцыя Сорыя"), ("bn", "সোরিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Sòria"), ("ccp", "𑄥\u{1112e}𑄢\u{11128}𑄠"), ("ceb", "Provincia de Soria"), ("cs", "Provincie Soria"), ("da", "Soria"), ("de", "Provinz Soria"), ("el", "Επαρχία της Σόρια"), ("en", "Soria"), ("es", "Provincia de Soria"), ("et", "Soria provints"), ("eu", "Soriako probintzia"), ("fa", "سریا (استان)"), ("fi", "Soria"), ("fr", "province de Soria"), ("gl", "Provincia de Soria"), ("gu", "સોરિયા પ\u{acd}રા\u{a82}ત"), ("hi", "सोरिया प\u{94d}रा\u{902}त"), ("hr", "Soria"), ("hu", "Soria"), ("id", "Provinsi Soria"), ("it", "provincia di Soria"), ("ja", "ソリア県"), ("ka", "სორიის პროვინცია"), ("kk", "Сория"), ("kn", "ಸೊರ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "소리아 주"), ("lt", "Sorijos provincija"), ("lv", "Sorijas province"), ("mk", "Сорија"), ("mr", "सोरिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Soria"), ("nb", "Soria"), ("nl", "Soria"), ("no", "Soria"), ("pl", "Soria"), ("pt", "Sória"), ("ro", "Provincia Soria"), ("ru", "Сория"), ("si", "සොර\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Soria"), ("sr", "Покрајина Сорија"), ("sr_Latn", "Pokrajina Sorija"), ("sv", "Soria"), ("sw", "Mkoa wa Soria"), ("ta", "சொறிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4b}ర\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโซเร\u{e35}ย"), ("tr", "Soria ili"), ("uk", "Сорія"), ("ur", "سوریا صوبہ"), ("uz", "Soria"), ("vi", "Soria"), ("yue", "索里亞省"), ("yue_Hans", "索里亚省"), ("zh", "索里亚省")]),
                        unofficial_name_list: ["Soria"].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "SS",
                        country_alpha2: Alpha2::ES,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0756299), longitude: Some(-2.2236667), max_latitude: Some(43.3956796), min_latitude: Some(42.8952452), max_longitude: Some(-1.7293434), min_longitude: Some(-2.6026837)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gipuzkoa"), ("ar", "غيبوثكوا"), ("be", "Гіпускоа"), ("bg", "Гипузкоа"), ("bn", "গিপ\u{9c1}জকোয\u{9bc}\u{9be}"), ("ca", "Guipúscoa"), ("ccp", "𑄉\u{11128}𑄛\u{1112a}𑄌\u{11134}𑄇\u{1112e}𑄠"), ("ceb", "Gipuzkoa"), ("cs", "Gipuzkoa"), ("cy", "Gipuzkoa"), ("da", "Guipúzcoa"), ("de", "Gipuzkoa"), ("el", "Γκιπούθκοα"), ("en", "Gipuzkoa"), ("es", "Guipúzcoa"), ("et", "Gipuzkoa"), ("eu", "Gipuzkoa"), ("fa", "استان گیپوسکوا"), ("fi", "Guipúzcoa"), ("fr", "Guipuscoa"), ("gl", "Provincia de Guipúscoa - Gipuzkoa"), ("gu", "ગીપ\u{ac1}ઝકોઆ"), ("hi", "जिप\u{942}ज\u{93c}को"), ("hr", "Gipuskoa"), ("hu", "Gipuzkoa"), ("hy", "Գիպուսկոա"), ("id", "Gipuzkoa"), ("it", "Guipúzcoa"), ("ja", "ギプスコア県"), ("ka", "გიპუსკოა"), ("kk", "Гипускоа"), ("kn", "ಗ\u{cbf}ಪುಝೋಕಾ"), ("ko", "기푸스코아 주"), ("lt", "Gipuskoa provincija"), ("lv", "Gipuskoja"), ("mr", "गिप\u{942}झकोआ"), ("ms", "Gipuzkoa"), ("nb", "Gipuzkoa"), ("nl", "Gipuzkoa"), ("no", "Gipuzkoa"), ("pl", "Prowincja Guipúzcoa"), ("pt", "Guipúscoa"), ("ro", "Provincia Guipúzcoa"), ("ru", "Гипускоа"), ("si", "ජ\u{dd2}ප\u{dd4}ස\u{dca}කොආ"), ("sq", "Provinca Gipuzkoa"), ("sr", "Покрајина Гипускоа"), ("sr_Latn", "Pokrajina Gipuskoa"), ("sv", "Guipúzcoa"), ("ta", "குபூஸ\u{bcd}கோய\u{bbe}"), ("te", "జ\u{c3f}పుజ\u{c4d}క\u{c4b}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e35}ป\u{e38}ซโกอา"), ("tr", "Guipúzcoa ili"), ("uk", "Гіпускоа"), ("ur", "گیپوسکوا"), ("uz", "Guipúzcoa"), ("vi", "Guipuscoa"), ("zh", "吉普斯夸省")]),
                        unofficial_name_list: ["Gipuzkoa", "Guipúzcoa"].to_vec(),
                    }
                ),
                (
                    "T",
                    Subdivision{
                        name: "T",
                        country_alpha2: Alpha2::ES,
                        code: "T",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1188827), longitude: Some(1.2444909), max_latitude: Some(41.1419484), min_latitude: Some(41.1096612), max_longitude: Some(1.2898503), min_longitude: Some(1.1873165)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "طراغونة"), ("be", "правінцыя Тарагона"), ("bn", "ত\u{9be}র\u{9be}গোন\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Tarragona"), ("ccp", "𑄑𑄢\u{11133}𑄦𑄉\u{11127}𑄚"), ("ceb", "Província de Tarragona"), ("cs", "Provincie Tarragona"), ("cy", "Talaith Tarragona"), ("da", "Tarragona"), ("de", "Tarragona"), ("el", "Επαρχία της Ταρραγκόνα"), ("en", "Tarragona"), ("es", "provincia de Tarragona"), ("et", "Tarragona provints"), ("eu", "Tarragonako probintzia"), ("fa", "استان تاراگونا"), ("fi", "Tarragona"), ("fr", "province de Tarragone"), ("gl", "Provincia de Tarragona"), ("gu", "ટ\u{ac7}રગોનો પ\u{acd}રા\u{a82}ત"), ("he", "מחוז טרגונה"), ("hi", "ट\u{947}रागोना"), ("hr", "Tarragona"), ("hu", "Tarragona"), ("id", "Provinsi Tarragona"), ("it", "Provincia di Tarragona"), ("ja", "タラゴナ県"), ("ka", "ტარაგონის პროვინცია"), ("kk", "Таррагона"), ("kn", "ತರ\u{ccd}ರಾಗೋನಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "타라고나 주"), ("lt", "Taragonos provincija"), ("lv", "Taragonas province"), ("mr", "तारागॉनना प\u{94d}रा\u{902}त"), ("ms", "Tarragona Province"), ("nb", "Tarragona"), ("nl", "Tarragona"), ("no", "Tarragona"), ("pl", "Tarragona"), ("pt", "Tarragona"), ("ro", "Provincia Tarragona"), ("ru", "Таррагона"), ("si", "ටැරගොන\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Tarragona"), ("sr", "Провинција Тарагона"), ("sr_Latn", "Provincija Taragona"), ("sv", "Tarragona"), ("sw", "Mkoa wa Tarragona"), ("ta", "டர\u{bcd}ரகோன\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "టర\u{c4d}ర\u{c3e}గ\u{c4b}న\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตาราโกนา"), ("tr", "Tarragona İli"), ("uk", "Таррагона"), ("ur", "صوبہ تاریگؤنا"), ("uz", "Tarragona"), ("vi", "Tarragona"), ("yue", "塔拉戈納省"), ("yue_Hans", "塔拉戈纳省"), ("zh", "塔拉戈纳省")]),
                        unofficial_name_list: ["Tarragona"].to_vec(),
                    }
                ),
                (
                    "TE",
                    Subdivision{
                        name: "TE",
                        country_alpha2: Alpha2::ES,
                        code: "TE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.3456879), longitude: Some(-1.1064345), max_latitude: Some(40.3642951), min_latitude: Some(40.3257241), max_longitude: Some(-1.0809371), min_longitude: Some(-1.1209926)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تيروال"), ("be", "Тэруэль"), ("bg", "Теруел"), ("bs", "Teruel"), ("ca", "província de Terol"), ("ccp", "𑄑𑄬𑄢\u{1112a}𑄠𑄬𑄣\u{11134}"), ("ceb", "Provincia de Teruel"), ("cs", "Provincie Teruel"), ("da", "Teruel"), ("de", "Provinz Teruel"), ("el", "Επαρχία του Τερουέλ"), ("en", "Teruel"), ("es", "Provincia de Teruel"), ("et", "Terueli provints"), ("eu", "Teruelgo probintzia"), ("fa", "استان تروئل"), ("fi", "Teruel"), ("fr", "province de Teruel"), ("gl", "Provincia de Teruel"), ("hr", "Teruel"), ("hu", "Teruel"), ("id", "Provinsi Teruel"), ("it", "provincia di Teruel"), ("ja", "テルエル県"), ("ka", "ტერუელის პროვინცია"), ("kk", "Теруэль"), ("ko", "테루엘 주"), ("lt", "Teruelio provincija"), ("lv", "Tervelas province"), ("ms", "Wilayah Teruel"), ("nb", "Teruel"), ("nl", "Teruel"), ("no", "Teruel"), ("pl", "Teruel"), ("pt", "Teruel (província)"), ("ro", "Provincia Teruel"), ("ru", "Теруэль"), ("sq", "Provinca Teruel"), ("sr", "Покрајина Теруел"), ("sr_Latn", "Pokrajina Teruel"), ("sv", "Teruel"), ("sw", "Mkoa wa Teruel"), ("tr", "Teruel İli"), ("uk", "Теруель"), ("ur", "صوبہ تیرویل"), ("uz", "Teruel"), ("vi", "Teruel"), ("yue", "特魯埃爾省"), ("yue_Hans", "特鲁埃尔省"), ("zh", "特魯埃爾省")]),
                        unofficial_name_list: ["Teruel"].to_vec(),
                    }
                ),
                (
                    "TF",
                    Subdivision{
                        name: "TF",
                        country_alpha2: Alpha2::ES,
                        code: "TF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(28.4636296), longitude: Some(-16.2518467), max_latitude: Some(28.487616), min_latitude: Some(28.4280248), max_longitude: Some(-16.2356646), min_longitude: Some(-16.3370045)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سانتا كروث دي تينيريفه"), ("be", "правінцыя Санта-Крус-дэ-Тэнэрыфэ"), ("bg", "Санта Крус де Тенерифе"), ("bn", "স\u{9cd}য\u{9be}ন\u{9cd}ট\u{9be} ক\u{9cd}র\u{9c1}জ দে টেনেলিফে প\u{9cd}রদেশ"), ("ca", "província de Santa Cruz de Tenerife"), ("ccp", "𑄥𑄚\u{11134}𑄑 𑄇\u{11133}𑄢\u{1112a}𑄌\u{11134} 𑄓𑄬 𑄑𑄬𑄚𑄬𑄢\u{1112d}𑄛\u{11134}"), ("ceb", "Provincia de Santa Cruz de Tenerife"), ("cs", "Provincie Santa Cruz de Tenerife"), ("da", "Santa Cruz de Tenerife"), ("de", "Provinz Santa Cruz de Tenerife"), ("el", "Σάντα Κρούθ Ντε Τενερίφε"), ("en", "Santa Cruz de Tenerife"), ("es", "Provincia de Santa Cruz de Tenerife"), ("et", "Santa Cruz de Tenerife provints"), ("eu", "Santa Cruz Tenerifekoko probintzia"), ("fa", "سانتا کروس"), ("fi", "Santa Cruz de Tenerife"), ("fr", "province de Santa Cruz de Ténérife"), ("gl", "Provincia de Santa Cruz de Tenerife"), ("gu", "સા\u{a82}તા ક\u{acd}ર\u{ac1}ઝ ડ\u{ac7} ટ\u{ac7}ન\u{ac7}રિફ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{948}\u{902}टा क\u{94d}र\u{942}ज\u{93c} डी ट\u{948}न\u{947}रीफ प\u{94d}रा\u{902}त"), ("hr", "Santa Cruz de Tenerife"), ("hu", "Santa Cruz de Tenerife"), ("hy", "Սանտա Կրուս դե Տեներիֆե"), ("id", "Provinsi Santa Cruz de Tenerife"), ("it", "Provincia di Santa Cruz de Tenerife"), ("ja", "サンタ・クルス・デ・テネリフェ県"), ("ka", "სანტა-კრუს-დე-ტენერიფეს პროვინცია"), ("kk", "Санта-Крус-де-Тенерифе"), ("kn", "ಸಾಂಟಾ ಕ\u{ccd}ರ\u{cc2}ಜ\u{ccd} ಡ\u{cbf} ಟ\u{cc6}ನ\u{cc6}ರೈಫ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "산타크루스데테네리페 주"), ("lt", "Tenerifės Santa Kruso provincija"), ("lv", "Santakrusas de Tenerifes province"), ("mr", "सा\u{902}ता क\u{94d}र\u{942}ज\u{93c} डी ट\u{947}न\u{947}रिफ प\u{94d}रा\u{902}त"), ("ms", "Wilayah Santa Cruz de Tenerife"), ("nb", "Santa Cruz de Tenerife"), ("nl", "Santa Cruz de Tenerife"), ("no", "Santa Cruz de Tenerife"), ("pl", "Santa Cruz de Tenerife"), ("pt", "Santa Cruz de Tenerife"), ("ro", "Provincia Santa Cruz de Tenerife"), ("ru", "Санта-Крус-де-Тенерифе"), ("si", "සැන\u{dca}ට\u{dcf} කෘස\u{dca} ඩෙ ටෙනේර\u{dd2}ෆේ පළ\u{dcf}ත"), ("sq", "Provinca Santa Cruz de Tenerife"), ("sr", "Покрајина Санта Круз де Тенерифе"), ("sr_Latn", "Pokrajina Santa Kruz de Tenerife"), ("sv", "Santa Cruz de Tenerife"), ("sw", "Mkoa wa Santa Cruz de Tenerife"), ("ta", "ச\u{bbe}ண\u{bcd}ட\u{bbe} கிருஸ\u{bcd} டி தேநெறிப ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3e} క\u{c4d}రజ\u{c4d} ద ట\u{c46}న\u{c46}ర\u{c48}ఫ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ซานตาคร\u{e39}ซ เด เตเรน\u{e34}เฟ"), ("tr", "Santa Cruz de Tenerife İli"), ("uk", "Санта-Крус-де-Тенерифе"), ("ur", "صوبہ سانتا کروز دے فینیریفے"), ("uz", "Santa Cruz de Tenerife"), ("vi", "Santa Cruz de Tenerife"), ("zh", "聖克魯斯-德特內里費省")]),
                        unofficial_name_list: ["Santa Cruz de Tenerife", "Tenerife"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::ES,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.8628316), longitude: Some(-4.027323099999999), max_latitude: Some(39.88605099999999), min_latitude: Some(39.8383676), max_longitude: Some(-3.9192423), min_longitude: Some(-4.0629256)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "طليطلة"), ("be", "Правінцыя Таледа"), ("bs", "Toledo"), ("ca", "Província de Toledo"), ("ccp", "𑄑\u{11127}𑄣𑄬𑄓\u{1112e}"), ("ceb", "Province of Toledo"), ("cs", "Provincie Toledo"), ("da", "Toledo"), ("de", "Provinz Toledo"), ("el", "Τολέδο"), ("en", "Toledo"), ("es", "Provincia de Toledo"), ("et", "Toledo provints"), ("eu", "Toledoko probintzia"), ("fa", "استان تولدو"), ("fi", "Toledo"), ("fr", "province de Tolède"), ("gl", "Provincia de Toledo"), ("he", "פרובינציית טולדו"), ("hi", "टोल\u{947}डो प\u{94d}रान\u{94d}त"), ("hr", "Toledo"), ("hu", "Toledo"), ("id", "Provinsi Toledo"), ("it", "provincia di Toledo"), ("ja", "トレド県"), ("ka", "ტოლედოს პროვინცია"), ("kk", "Толедо"), ("ko", "톨레도 주"), ("lt", "Toledo provincija"), ("lv", "Toledo province"), ("mk", "Толедо"), ("ms", "Toledo"), ("nb", "Toledo"), ("nl", "Toledo"), ("no", "Toledo"), ("pl", "Toledo"), ("pt", "Toledo"), ("ro", "Provincia Toledo"), ("ru", "Толедо"), ("sq", "Provinca Toledo"), ("sr", "Покрајина Толедо"), ("sr_Latn", "Pokrajina Toledo"), ("sv", "Toledo"), ("sw", "Mkoa wa Toledo"), ("tr", "Toledo ili"), ("uk", "Толедо"), ("ur", "صوبہ طلیطلہ"), ("uz", "Toledo"), ("vi", "Toledo"), ("zh", "托萊多省")]),
                        unofficial_name_list: ["Toledo"].to_vec(),
                    }
                ),
                (
                    "V",
                    Subdivision{
                        name: "V",
                        country_alpha2: Alpha2::ES,
                        code: "V",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.4699075), longitude: Some(-0.3762881), max_latitude: Some(39.5073225), min_latitude: Some(39.308248), max_longitude: Some(-0.2914778), min_longitude: Some(-0.4315448)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلنسية"), ("be", "Правінцыя Валенсія"), ("bn", "ভ\u{9cd}য\u{9be}লেন\u{9cd}সিয\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "Província de València"), ("ccp", "𑄞𑄣𑄬𑄚\u{11134}𑄥\u{11128}𑄠"), ("ceb", "Província de València"), ("cs", "Provincie Valencia"), ("da", "Valencia"), ("de", "Provinz Valencia"), ("el", "Επαρχία της Βαλένθια"), ("en", "Valencia"), ("es", "Provincia de Valencia"), ("et", "Valencia provints"), ("eu", "Valentziako probintzia"), ("fa", "استان والنسیا"), ("fi", "Valencia"), ("fr", "province de Valence"), ("gl", "Provincia de Valencia"), ("gu", "વ\u{ac7}લ\u{ac7}ન\u{acd}સિયા પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{948}ल\u{947}\u{902}सिया प\u{94d}रा\u{902}त"), ("hr", "Valencia"), ("hu", "Valencia tartomány"), ("hy", "Վալենսիա"), ("id", "Provinsi Valencia"), ("it", "Provincia di Valencia"), ("ja", "バレンシア県"), ("ka", "ვალენსიის პროვინცია"), ("kn", "ವ\u{ccd}ಯಾಲ\u{cc6}ನ\u{ccd}ಸ\u{cbf}ಯಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "발렌시아 주"), ("lt", "Valensijos provincija"), ("lv", "Valensijas province"), ("mk", "Валенсија"), ("mr", "व\u{94d}ह\u{945}ल\u{947}न\u{94d}सिया प\u{94d}रा\u{902}त"), ("ms", "Wilayah Valencia"), ("nb", "Valencia"), ("nl", "Valencia"), ("no", "Valencia"), ("pl", "Walencja"), ("pt", "Valência (província)"), ("ro", "Provincia Valencia"), ("ru", "Валенсия"), ("si", "වැලෙන\u{dca}ස\u{dd2}ය\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Valencia"), ("sr", "Покрајина Валенсија"), ("sr_Latn", "Pokrajina Valensija"), ("sv", "Valencia"), ("sw", "Mkoa wa Valencia"), ("ta", "வ\u{bbe}லென\u{bcd}சிய\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3e}ల\u{c46}న\u{c4d}స\u{c3f}య\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดวาเลนเซ\u{e35}ย"), ("tr", "Valencia Province"), ("uk", "Валенсія"), ("ur", "صوبہ ویلنسیا"), ("uz", "Valencia"), ("vi", "Valencia"), ("yue", "華倫西亞省"), ("yue_Hans", "华伦西亚省"), ("zh", "巴倫西亞省")]),
                        unofficial_name_list: ["Valencia", "Valencia/València", "València", "València/Valencia"].to_vec(),
                    }
                ),
                (
                    "VA",
                    Subdivision{
                        name: "VA",
                        country_alpha2: Alpha2::ES,
                        code: "VA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.652251), longitude: Some(-4.724532099999999), max_latitude: Some(41.8155086), min_latitude: Some(41.6042843), max_longitude: Some(-4.6894439), min_longitude: Some(-4.9281803)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلد الوليد"), ("az", "Valyadolid vilayəti"), ("be", "Вальядалід"), ("bn", "ভ\u{9cd}য\u{9be}ল\u{9cd}ল\u{9be}দোলিড প\u{9cd}রদেশ"), ("ca", "província de Valladolid"), ("ccp", "𑄞𑄣\u{11133}𑄦𑄓\u{1112e}𑄣\u{11128}𑄖\u{11134}"), ("ceb", "Provincia de Valladolid"), ("cs", "Provincie Valladolid"), ("da", "Valladolid"), ("de", "Provinz Valladolid"), ("el", "Επαρχία του Βαγιαδολίδ"), ("en", "Valladolid"), ("es", "Provincia de Valladolid"), ("et", "Valladolidi provints"), ("eu", "Valladolideko probintzia"), ("fa", "استان بالادولید"), ("fi", "Valladolid"), ("fr", "province de Valladolid"), ("ga", "Valladolid"), ("gl", "Provincia de Valladolid"), ("gu", "વ\u{ac7}લ\u{ac7}ડોલીડ પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{947}लाडोलिड प\u{94d}रा\u{902}त"), ("hr", "Valladolid"), ("hu", "Valladolid"), ("hy", "Վալյադոլիդ"), ("id", "Provinsi Valladolid"), ("it", "Provincia di Valladolid"), ("ja", "バリャドリッド県"), ("ka", "ვალიადოლიდის პროვინცია"), ("kk", "Вальядолид"), ("kn", "ವಲ\u{ccd}ಲಾಡೋಲ\u{cbf}ಡ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바야돌리드 주"), ("lt", "Valjadolido provincija"), ("lv", "Valjadolidas province"), ("mk", "Ваљадолид"), ("mr", "व\u{945}ल\u{945}डॉलिड प\u{94d}रा\u{902}त"), ("ms", "Wilayah Valladolid"), ("nb", "Valladolid"), ("nl", "Valladolid"), ("no", "Valladolid"), ("pl", "Valladolid"), ("pt", "Valhadolide"), ("ro", "Provincia Valladolid"), ("ru", "Вальядолид"), ("si", "වල\u{dca}ලඩොල\u{dd2}ඩ\u{dca} පළ\u{dcf}ත"), ("sq", "Provinca Valladolid"), ("sr", "Покрајина Ваљадолид"), ("sr_Latn", "Pokrajina Valjadolid"), ("sv", "Valladolid"), ("sw", "Mkoa wa Valladolid"), ("ta", "வெள\u{bcd}ள\u{bbe}ட\u{bbe}ய\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c3e}ల\u{c3e}డ\u{c4b}ల\u{c3f}డ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบายาโดล\u{e34}ด"), ("tr", "Valladolid ili"), ("uk", "Вальядолід"), ("ur", "صوبہ بلدولید"), ("uz", "Valladolid"), ("vi", "Valladolid"), ("zh", "巴利亞多利德省")]),
                        unofficial_name_list: ["Valladolid"].to_vec(),
                    }
                ),
                (
                    "VC",
                    Subdivision{
                        name: "VC",
                        country_alpha2: Alpha2::ES,
                        code: "VC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousCommunity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("am", "የቫለንሲያ ኅብረተሠብ"), ("ar", "منطقة بلنسية"), ("be", "аўтаномная супольнасць Валенсія"), ("bg", "Валенсия"), ("bs", "Valencia (pokrajina)"), ("ca", "País Valencià"), ("ccp", "𑄞\u{11127}𑄣𑄬𑄚\u{11134}𑄥\u{11128}𑄠𑄚\u{11134} 𑄇\u{11127}𑄟\u{11128}𑄅\u{1112a}𑄚\u{11128}𑄑\u{11128}"), ("ceb", "Comunitat Valenciana"), ("cs", "Valencijské společenství"), ("cy", "Valencia"), ("da", "Valencia²"), ("de", "Valencianische Gemeinschaft"), ("el", "Βαλένθια"), ("en", "Valencian Community"), ("es", "Comunidad Valenciana"), ("et", "Valencia autonoomne piirkond"), ("eu", "Valentziako Erkidegoa"), ("fa", "بخش خودمختار والنسیا"), ("fi", "Valencia²"), ("fr", "communauté valencienne"), ("ga", "Comhphobal Valencia"), ("gl", "Comunidade Valenciana"), ("gu", "વ\u{ac5}લ\u{ac7}ન\u{acd}શિયા નો પ\u{acd}રદ\u{ac7}શ"), ("he", "ולנסיה"), ("hi", "व\u{948}ल\u{947}\u{902}सियाई सम\u{941}दाय"), ("hr", "Valencijska Zajednica"), ("hu", "Valencia"), ("hy", "Վալենսիա²"), ("id", "Negeri Valencia"), ("is", "Valensía (hérað)"), ("it", "comunità Valenzana"), ("ja", "バレンシア州"), ("ka", "ვალენსიის ავტონომიური გაერთიანება"), ("kk", "Валенсия"), ("ko", "발렌시아 지방"), ("lo", "ແຄວ\u{ec9}ນວາລ\u{eb1}ງ"), ("lt", "Valensija"), ("lv", "Valensija"), ("mk", "Валенсија²"), ("mr", "वाल\u{947}न\u{94d}सिया"), ("ms", "Komuniti Valencia"), ("nb", "Valencia²"), ("nl", "Valencia²"), ("no", "Valencia²"), ("pl", "Walencja²"), ("pt", "Comunidade Valenciana"), ("ro", "Comunitatea Valenciană"), ("ru", "Валенсия²"), ("sk", "Valencijské spoločenstvo"), ("sl", "Valencijska dežela"), ("sq", "Komuniteti i Valencias"), ("sr", "Валенсијанска Покрајина"), ("sr_Latn", "Valensijanska Pokrajina"), ("sv", "Valencia²"), ("ta", "வளன\u{bcd}சிய\u{bbe}ன\u{bcd} ம\u{bbe}நிலம\u{bcd}"), ("th", "แคว\u{e49}นบาเลนเซ\u{e35}ย"), ("tr", "Valensiya"), ("uk", "Валенсія²"), ("ur", "ویلنسیائی کمیونٹی"), ("uz", "Valencialar mamlakati"), ("vi", "Cộng đồng Valencia"), ("yue", "華倫西亞自治區"), ("yue_Hans", "华伦西亚自治区"), ("zh", "巴倫西亞自治區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "VI",
                    Subdivision{
                        name: "VI",
                        country_alpha2: Alpha2::ES,
                        code: "VI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(42.9099989), longitude: Some(-2.6983868), max_latitude: Some(43.216969), min_latitude: Some(42.4722552), max_longitude: Some(-2.2326871), min_longitude: Some(-3.2867669)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Álava"), ("ar", "ألافا"), ("az", "Alava vilayəti"), ("be", "правінцыя Алава"), ("bg", "Алава"), ("bn", "এল\u{9be}ভ\u{9be}"), ("ca", "Àlaba"), ("ccp", "𑄃𑄣𑄞"), ("ceb", "Araba / Álava"), ("cs", "Álava"), ("cy", "Araba"), ("da", "Álava"), ("de", "Álava"), ("el", "Αλάβα"), ("en", "Álava"), ("es", "Álava"), ("et", "Araba"), ("eu", "Araba"), ("fa", "استان آلابا"), ("fi", "Álava"), ("fr", "province d’Alava"), ("gl", "Provincia de Álava - Araba"), ("gu", "અલાવા"), ("hi", "अलवा प\u{94d}रान\u{94d}त"), ("hr", "Alava"), ("hu", "Araba"), ("hy", "Ալավա"), ("id", "Álava"), ("it", "Álava"), ("ja", "アラバ県"), ("ka", "ალავა"), ("kk", "Алава"), ("kn", "ಅಲಾವಾ"), ("ko", "알라바 주"), ("lt", "Alavos provincija"), ("lv", "Alavas province"), ("mk", "Алава"), ("mr", "अलावा"), ("ms", "Álava"), ("nb", "Álava"), ("nl", "Álava"), ("no", "Álava"), ("pl", "Araba"), ("pt", "Álava"), ("ro", "Provincia Álava"), ("ru", "Алава"), ("si", "අලව\u{dcf}"), ("sq", "Provinca Álava"), ("sr", "Провинција Алава"), ("sr_Latn", "Provincija Alava"), ("sv", "Álava"), ("sw", "Álava"), ("ta", "அல\u{bcd}லவ\u{bbe}"), ("te", "అల\u{c3e}వ\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดอาลาบา"), ("tr", "Álava ili"), ("uk", "Алава"), ("ur", "آلابا"), ("uz", "Álava"), ("vi", "Álava"), ("zh", "阿拉瓦省")]),
                        unofficial_name_list: ["Araba", "Araba/Álava", "Álava"].to_vec(),
                    }
                ),
                (
                    "Z",
                    Subdivision{
                        name: "Z",
                        country_alpha2: Alpha2::ES,
                        code: "Z",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6488226), longitude: Some(-0.8890853), max_latitude: Some(41.6894079), min_latitude: Some(41.6139746), max_longitude: Some(-0.8427317), min_longitude: Some(-0.9472301000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سرقسطة"), ("az", "Zaraqoza"), ("be", "правінцыя Сарагоса"), ("bn", "জ\u{9be}\u{9be}র\u{9be}গোজ\u{9be} প\u{9cd}রদেশ"), ("bs", "Zaragoza"), ("ca", "província de Saragossa"), ("ccp", "𑄎𑄢\u{11134}𑄉\u{1112e}𑄎"), ("ceb", "Provincia de Zaragoza"), ("cs", "Provincie Zaragoza"), ("da", "Zaragoza"), ("de", "Provinz Saragossa"), ("el", "Θαραγόθα"), ("en", "Zaragoza"), ("es", "Provincia de Zaragoza"), ("et", "Zaragoza provints"), ("eu", "Zaragozako probintzia"), ("fa", "استان ساراگوسا"), ("fi", "Zaragoza"), ("fr", "province de Saragosse"), ("gl", "Provincia de Zaragoza"), ("gu", "ઝારાગોઝા પ\u{acd}રા\u{a82}ત"), ("he", "מחוז סרגוסה"), ("hi", "ज\u{93c}ारागोज\u{93c}ा प\u{94d}रा\u{902}त"), ("hr", "Zaragoza"), ("hu", "Zaragoza"), ("hy", "Սարագոսա"), ("id", "Provinsi Zaragoza"), ("it", "provincia di Saragozza"), ("ja", "サラゴサ県"), ("ka", "სარაგოსის პროვინცია"), ("kk", "Сарагоса"), ("kn", "ಜರಾಗೊಝಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사라고사 주"), ("lt", "Saragosos provincija"), ("lv", "Saragosas province"), ("mk", "Сагароса"), ("mr", "ज\u{93c}ारागोजा प\u{94d}रा\u{902}त"), ("ms", "Wilayah Zaragoza"), ("nb", "Zaragoza"), ("nl", "Zaragoza"), ("no", "Zaragoza"), ("pl", "Saragossa"), ("pt", "Saragoça"), ("ro", "Provincia Zaragoza"), ("ru", "Сарагоса"), ("si", "සරගොස\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Zaragoza"), ("sr", "Покрајина Сарагоса"), ("sr_Latn", "Pokrajina Saragosa"), ("sv", "Zaragoza"), ("sw", "Mkoa wa Zaragoza"), ("ta", "சரகோஜ\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జర\u{c3e}గ\u{c4b}జ\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาราโกซา"), ("tr", "Zaragoza ili"), ("uk", "Сарагоса"), ("ur", "صوبہ سرقسطہ"), ("uz", "Zaragoza"), ("vi", "Zaragoza"), ("yue", "薩拉戈薩省"), ("yue_Hans", "萨拉戈萨省"), ("zh", "薩拉戈薩省")]),
                        unofficial_name_list: ["Zaragoza"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::ES,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5034712), longitude: Some(-5.7467879), max_latitude: Some(41.5226013), min_latitude: Some(41.4850183), max_longitude: Some(-5.720656399999999), min_longitude: Some(-5.768415699999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سمورة"), ("be", "Правінцыя Самора"), ("bg", "Самора"), ("bn", "জ\u{9be}মোড\u{9bc}\u{9be} প\u{9cd}রদেশ"), ("ca", "província de Zamora"), ("ccp", "𑄎𑄟\u{1112e}𑄢"), ("ceb", "Provincia de Zamora"), ("cs", "Provincie Zamora"), ("da", "Zamora"), ("de", "Provinz Zamora"), ("el", "Επαρχία της Θαμόρα"), ("en", "Zamora"), ("es", "Provincia de Zamora"), ("et", "Zamora provints"), ("eu", "Zamorako probintzia"), ("fa", "استان سامرا"), ("fi", "Zamora"), ("fr", "province de Zamora"), ("gl", "Provincia de Zamora"), ("gu", "ઝામોરા પ\u{acd}રા\u{a82}ત"), ("hi", "ज\u{93c}मोरा प\u{94d}रा\u{902}त"), ("hr", "Zamora"), ("hu", "Zamora"), ("id", "Provinsi Zamora"), ("it", "provincia di Zamora"), ("ja", "サモラ県"), ("ka", "სამორის პროვინცია"), ("kk", "Самора"), ("kn", "ಝಮೊರಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "사모라 주"), ("lt", "Samoros provincija"), ("lv", "Samoras province"), ("mr", "ज\u{93c}मोमो प\u{94d}रा\u{902}त"), ("ms", "Wilayah Zamora"), ("nb", "Zamora"), ("nl", "Zamora"), ("no", "Zamora"), ("pl", "Zamora"), ("pt", "Zamora"), ("ro", "Provincia Zamora"), ("ru", "Самора"), ("si", "සමෝර\u{dcf} පළ\u{dcf}ත"), ("sq", "Provinca Zamora"), ("sr", "Покрајина Замора"), ("sr_Latn", "Pokrajina Zamora"), ("sv", "Zamora"), ("sw", "Mkoa wa Zamora"), ("ta", "சமோர\u{bbe} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "జమ\u{c4b}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดซาโมรา"), ("tr", "Zamora ili"), ("uk", "Самора"), ("ur", "صوبہ سامورا"), ("uz", "Zamora"), ("vi", "Zamora"), ("zh", "萨莫拉省")]),
                        unofficial_name_list: ["Zamora"].to_vec(),
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
#[cfg(feature = "es")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::ES,
        alpha3: Alpha3::ESP,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{region}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 34,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::SP),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::ESP),
        iso_long_name: "The Kingdom of Spain",
        iso_short_name: "Spain",
        official_language_list: ["es"].to_vec(),
        spoken_language_list: ["es"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "None",
        nationality: Some("Spanish"),
        number: "724",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "ES",
        unofficial_name_list: [
            "Spain",
            "Spanien",
            "Espagne",
            "España",
            "スペイン",
            "Spanje",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Spain"),
            ("af", "Spanje"),
            ("ak", "Spain"),
            ("am", "ስፔን"),
            ("an", "Spain"),
            ("ar", "إسبانيا"),
            ("as", "স\u{9cd}পেইন"),
            ("ay", "Spain"),
            ("az", "İspaniya"),
            ("ba", "Spain"),
            ("be", "Іспанія"),
            ("bg", "Испания"),
            ("bi", "Spain"),
            ("bn", "স\u{9cd}পেন"),
            ("bn_IN", "স\u{9cd}পেন"),
            ("br", "Spagn"),
            ("bs", "Španija"),
            ("ca", "Espanya"),
            ("ce", "Испани"),
            ("ch", "España"),
            ("cs", "Španělsko"),
            ("cv", "Испани"),
            ("cy", "Sbaen"),
            ("da", "Spanien"),
            ("de", "Spanien"),
            ("dv", "އ\u{7a8}ސ\u{7b0}ޕ\u{7ac}އ\u{7a8}ނ\u{7b0}"),
            ("dz", "ས\u{f72}པ\u{f7a}ན།"),
            ("ee", "Spain"),
            ("el", "Ισπανία"),
            ("en", "Spain"),
            ("eo", "Hispanio"),
            ("es", "España"),
            ("et", "Hispaania"),
            ("eu", "Espainia"),
            ("fa", "اسپانیا"),
            ("ff", "Hispaanya"),
            ("fi", "Espanja"),
            ("fo", "Spania"),
            ("fr", "Espagne"),
            ("fy", "Spanje"),
            ("ga", "An Spáinn"),
            ("gl", "España"),
            ("gn", "Spain"),
            ("gu", "સ\u{acd}પ\u{ac7}ન"),
            ("gv", "Yn Spaainey"),
            ("ha", "An Spàinn"),
            ("he", "ספרד"),
            ("hi", "स\u{94d}प\u{947}न"),
            ("hr", "Španjolska"),
            ("ht", "Espay"),
            ("hu", "Spanyolország"),
            ("hy", "Իսպանիա"),
            ("ia", "Espania"),
            ("id", "Spanyol"),
            ("io", "Hispania"),
            ("is", "Spánn"),
            ("it", "Spagna"),
            ("iu", "ᓯᐸᐃᓐ"),
            ("ja", "スペイン"),
            ("ka", "ესპანეთი"),
            ("ki", "Spain"),
            ("kk", "Испания"),
            ("kl", "Spain"),
            ("km", "អេស\u{17d2}ប\u{17c9}ាញ"),
            ("kn", "ಸ\u{ccd}ಪೇನ\u{ccd}"),
            ("ko", "스페인"),
            ("ku", "Spanya"),
            ("kv", "Испания"),
            ("kw", "Spayn"),
            ("ky", "Испания"),
            ("lo", "ປະເທດແອສະປາຍ"),
            ("lt", "Ispanija"),
            ("lv", "Spānija"),
            ("mi", "Pāniora"),
            ("mk", "Шпанија"),
            ("ml", "സ\u{d4d}പെയിന\u{d4d}\u{200d}"),
            ("mn", "Испани"),
            ("mr", "स\u{94d}प\u{947}न"),
            ("ms", "Sepanyol"),
            ("mt", "Spanja"),
            (
                "my",
                "စပ\u{102d}န\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Pain"),
            ("nb", "Spania"),
            ("ne", "स\u{94d}प\u{947}न"),
            ("nl", "Spanje"),
            ("nn", "Spania"),
            ("nv", "Dibé Diníí Bikéyah"),
            ("oc", "Espanha"),
            ("or", "ସ\u{b4d}ପେନ"),
            ("pa", "ਸਪ\u{a47}ਨ"),
            ("pi", "स\u{94d}प\u{947}न"),
            ("pl", "Hiszpania"),
            ("ps", "هسپانیه"),
            ("pt", "Espanha"),
            ("pt_BR", "Espanha"),
            ("ro", "Spania"),
            ("ru", "Испания"),
            ("rw", "Esipanye"),
            ("sc", "Ispagna"),
            ("sd", "اسپين"),
            ("si", "ස\u{dca}ප\u{dcf}ඤ\u{dca}ඤය"),
            ("sk", "Španielsko"),
            ("sl", "Španija"),
            ("so", "Isbeyn"),
            ("sq", "Spanjë"),
            ("sr", "Шпанија"),
            ("sv", "Spanien"),
            ("sw", "Spain"),
            ("ta", "ஸ\u{bcd}பெயின\u{bcd}"),
            ("te", "స\u{c4d}ప\u{c46}య\u{c3f}న\u{c4d}"),
            ("tg", "Испания"),
            ("th", "สเปน"),
            ("ti", "ስጳኛ"),
            ("tk", "Ispaniýa"),
            ("tl", "Espanya"),
            ("tr", "İspanya"),
            ("tt", "İспаниа"),
            ("ug", "ئىسپانىيە"),
            ("uk", "Іспанія"),
            ("ur", "ہسپانیہ"),
            ("uz", "Ispaniya"),
            ("ve", "Spain"),
            ("vi", "Tây Ban Nha"),
            ("wa", "Espagne"),
            ("wo", "Ispaañ"),
            ("xh", "Spain"),
            ("yo", "Spéìn"),
            ("zh_CN", "西班牙"),
            ("zh_HK", "西班牙"),
            ("zh_TW", "西班牙"),
            ("zu", "ISpeyini"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
