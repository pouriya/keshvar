// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Democratic People's Republic of Korea

#[cfg(all(feature = "kp", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::KP;
    pub const ALPHA3: Alpha3 = Alpha3::PRK;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 850;
    pub const CURRENCY_CODE: &str = "KPW";
    pub const GEC: Option<GEC> = Some(GEC::KN);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("PRK");
    pub const ISO_SHORT_NAME: &str = "Korea (Democratic People's Republic of)";
    pub const ISO_LONG_NAME: &str = "The Democratic People's Republic of Korea";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ko"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ko"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("North Korean");
    pub const NUMBER: &str = "408";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAsia);
    pub const UN_LOCODE: &str = "KP";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Korea (North)",
        "North Korea",
        "Nordkorea",
        "Corée du Nord",
        "Corea del Norte",
        "朝鮮民主主義人民共和国",
        "Noord-Korea",
        "Korea Democratic People's Republic",
        "Korea (Democratic People s Republic of)",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "North Korea"),
        ("af", "Noord-Korea"),
        ("ak", "North Korea"),
        ("am", "ሰሜናዊ ኮሪያ"),
        ("an", "North Korea"),
        ("ar", "كوريا، جمهورية كوريا الش\u{651}عبي\u{651}ة الد\u{651}يموقراطي\u{651}ة"),
        ("as", "কোৰিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}ৰিক গণপ\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰী"),
        ("ay", "North Korea"),
        ("az", "Koreya, Demokratık Xalq Respublikası"),
        ("ba", "North Korea"),
        ("be", "Карэя, Народна-Дэмакратычная Рэспубліка"),
        ("bg", "Корея, Демократична народна република"),
        ("bi", "North Korea"),
        ("bn", "কোরিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}রিক গণপ\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}রী"),
        ("bn_IN", "কোরিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}রিক গণপ\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}রী"),
        ("br", "North Korea"),
        ("bs", "Koreja, Demokratska Narodna Republika"),
        ("ca", "Corea del Nord"),
        ("ce", "North Korea"),
        ("ch", "North Korea"),
        ("cs", "Korea, lidově demokratická republika"),
        ("cv", "North Korea"),
        ("cy", "Korea (Gweriniaeth Democrataidd Pobl)"),
        ("da", "Korea, Den Demokratiske Folkerepublik"),
        ("de", "Nordkorea"),
        ("dv", "North Korea"),
        ("dz", "ཀ\u{f7c}་ར\u{f72}་ཡ། ཌ\u{f7a}་མ\u{f7c}་ཀ\u{f7a}ར\u{f7a}་ཊ\u{f72}ཀ་ པ\u{f72}་པ\u{f71}ལས\u{f72}་ ར\u{f72}་པཔ་ལ\u{f72}ཀ་ ཨ\u{f7c}ཕ་"),
        ("ee", "North Korea"),
        ("el", "Κορέα, Λαοκρατική Δημοκρατία της"),
        ("en", "North Korea"),
        ("eo", "Nord-Koreio"),
        ("es", "Corea, República Democrática Popular de"),
        ("et", "Põhja-Korea"),
        ("eu", "Korea, Herri Errepublika Demokratikoa"),
        ("fa", "کره، جمهوری دموکراتیک خلق"),
        ("ff", "North Korea"),
        ("fi", "Korean demokraattinen kansantasavalta"),
        ("fo", "North Korea"),
        ("fr", "Corée du Nord"),
        ("fy", "North Korea"),
        ("ga", "Daonphoblacht Dhaonlathach na Cóiré"),
        ("gl", "Corea, República Democrática Popular de"),
        ("gn", "North Korea"),
        ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"),
        ("gv", "North Korea"),
        ("ha", "North Korea"),
        ("he", "קוריאה הצפונית"),
        ("hi", "उत\u{94d}तर कोरिया"),
        ("hr", "Sjeverna Koreja"),
        ("ht", "North Korea"),
        ("hu", "Koreai Népi Demokratikus Köztársaság"),
        ("hy", "Կորեա, Ժողովրդադեմոկրատական Հանրապետություն"),
        ("ia", "Corea, Republica Popular Democratic de"),
        ("id", "Korea Utara"),
        ("io", "North Korea"),
        ("is", "Norður-Kórea"),
        ("it", "Corea del Nord"),
        ("iu", "North Korea"),
        ("ja", "朝鮮民主主義人民共和国"),
        ("ka", "კორეის სახალხო-დემოკრატიული რესპუბლიკა"),
        ("ki", "North Korea"),
        ("kk", "Корея (Солтүстік)"),
        ("kl", "North Korea"),
        ("km", "ក\u{17bc}រ\u{17c9}េ សាធារណ\u{200b}រដ\u{17d2}ឋ\u{200b}ប\u{17d2}រជាមាន\u{17b7}ត\u{200b}ប\u{17d2}រជាធ\u{17b7}បតេយ\u{17d2}យ\u{200b}\u{200b}នៃ"),
        ("kn", "ಕೋರ\u{cbf}ಯಾ, ಡ\u{cc6}ಮೊಕ\u{ccd}ರಟ\u{cbf}ಕ\u{ccd} ಪೀಪಲ\u{ccd}ಸ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"),
        ("ko", "조선민주주의인민공화국"),
        ("ku", "Kore, Komara Demokratîk a Gelê"),
        ("kv", "North Korea"),
        ("kw", "North Korea"),
        ("ky", "Корея Элдик Демократиялык Республикасы"),
        ("lo", "North Korea"),
        ("lt", "Korėjos Liaudies Demokratinė Respublika"),
        ("lv", "Ziemeļkoreja"),
        ("mi", "North Korea"),
        ("mk", "Кореја, Демократска народна република"),
        ("ml", "കൊറിയ, ഡെമോക\u{d4d}ര\u{d3e}റ\u{d4d}റിക\u{d4d} പീപ\u{d4d}പിള\u{d4d}\u{200d}സ\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"),
        ("mn", "Ардчилсан солонгос улс"),
        ("mr", "कोरिया, ड\u{947}मोक\u{94d}र\u{945}टिक पिपल\u{94d}स रिपब\u{94d}लिक ऑफ"),
        ("ms", "North Korea"),
        ("mt", "North Korea"),
        ("my", "North Korea"),
        ("na", "North Korea"),
        ("nb", "Nord-Korea"),
        ("ne", "कोरियाको प\u{94d}रजातन\u{94d}त\u{94d}रिक जनगणराज\u{94d}य"),
        ("nl", "Noord-Korea"),
        ("nn", "Nord-Korea"),
        ("nv", "North Korea"),
        ("oc", "Corèa del Nòrd"),
        ("or", "କୋର\u{b3f}ଆ, ସର\u{b4d}ବସ\u{b3e}ଧ\u{b3e}ରଣ ଜନସମ\u{b3e}ଜ ଗଣତନ\u{b4d}ତ\u{b4d}ର"),
        ("pa", "ਕ\u{a4b}ਰੀਆ, ਲ\u{a4b}ਕਤ\u{a70}ਤਰੀ ਗਣਰਾਜ"),
        ("pi", "North Korea"),
        ("pl", "Korea Północna"),
        ("ps", "North Korea"),
        ("pt", "Coreia do Norte"),
        ("pt_BR", "Coreia do Norte"),
        ("ro", "Republica democrată populară Coreea"),
        ("ru", "Северная Корея"),
        ("rw", "Koreya, Repubulika Iharanira Demokarasi ya Rubanda ya"),
        ("sc", "Corea de su Nord"),
        ("sd", "North Korea"),
        ("si", "සම\u{dcf}ජව\u{dcf}ද\u{dd3} මහජන කොර\u{dd2}ය\u{dcf} ජනරජය"),
        ("sk", "Kórejská ľudovodemokratická republika"),
        ("sl", "Severna Koreja"),
        ("so", "North Korea"),
        ("sq", "Korea, Republika Popullore Demokratike e"),
        ("sr", "Кореја, Демократска Народна Република"),
        ("sv", "Nordkorea"),
        ("sw", "North Korea"),
        ("ta", "கொரிய ஜனந\u{bbe}யக மக\u{bcd}கள\u{bcd} குடியரசு"),
        ("te", "క\u{c4b}ర\u{c3f}య\u{c3e}, డ\u{c46}మ\u{c4b}క\u{c4d}రట\u{c3f}క\u{c4d} ప\u{c40}పుల\u{c4d}స\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}"),
        ("tg", "Ҷумҳурии Халқӣ-Демократии Корея"),
        ("th", "เกาหล\u{e35}เหน\u{e37}อ"),
        ("ti", "North Korea"),
        ("tk", "Koreýa, Halk Demokratik Respublikasy"),
        ("tl", "Korea, Taumbayang Demokratikong Republika of"),
        ("tr", "Kuzey Kore"),
        ("tt", "North Korea"),
        ("ug", "چاۋشيەن"),
        ("uk", "Північна Корея"),
        ("ur", "North Korea"),
        ("uz", "North Korea"),
        ("ve", "North Korea"),
        ("vi", "Bắc Hàn, Cộng hoà Nhân dân Dân chủ"),
        ("wa", "Corêye (bijhrece)"),
        ("wo", "Koore, Republik Popileer Demokaraatik bu"),
        ("xh", "North Korea"),
        ("yo", "North Korea"),
        ("zh_CN", "朝鲜"),
        ("zh_HK", "北韓"),
        ("zh_TW", "朝鮮民主主義人民共和國"),
        ("zu", "North Korea"),
];
    #[cfg(all(feature = "kp", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 40.339852;
        pub const LONGITUDE: f64 = 127.510093;
        pub const MAX_LATITUDE: f64 = 43.01159;
        pub const MAX_LONGITUDE: f64 = 130.6990167;
        pub const MIN_LATITUDE: f64 = 37.5892001;
        pub const MIN_LONGITUDE: f64 = 124.1491605;
        pub const NORTHEAST_LATITUDE: f64 = 43.01159;
        pub const NORTHEAST_LONGITUDE: f64 = 130.6990167;
        pub const SOUTHWEST_LATITUDE: f64 = 37.5892001;
        pub const SOUTHWEST_LONGITUDE: f64 = 124.1491605;
    }
}
#[cfg(all(feature = "kp", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 40.339852,
            longitude: 127.510093,
            max_latitude: 43.01159,
            max_longitude: 130.6990167,
            min_latitude: 37.5892001,
            min_longitude: 124.1491605,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 43.01159,
                    longitude: 130.6990167,
                },
                southwest: CountryGeoBound {
                    latitude: 37.5892001,
                    longitude: 124.1491605,
                },
            },
        }
    }
}

#[cfg(all(feature = "kp", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::KP,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pjongjang"), ("am", "ፕዮንግያንግ"), ("ar", "بيونغيانغ"), ("as", "পিয\u{9bc}ং ইয\u{9bc}\u{9be}ং"), ("az", "Pxenyan"), ("be", "Горад Пхеньян"), ("bg", "Пхенян"), ("bn", "পিয\u{9bc}ং ইয\u{9bc}\u{9be}ং"), ("bs", "Pjongjang"), ("ca", "Pyongyang"), ("ccp", "𑄛\u{11128}𑄠\u{11127}\u{11101}𑄃\u{11128}𑄠\u{11127}\u{11101}"), ("ceb", "Pyongyang (lalawigan)"), ("cs", "Pchjongjang"), ("cy", "P’yŏngyang"), ("da", "Pyongyang"), ("de", "Pjöngjang"), ("el", "Πιονγιάνγκ"), ("en", "Pyongyang"), ("es", "Pionyang"), ("et", "P’yŏngyang"), ("eu", "Piongiang"), ("fa", "پیونگ\u{200c}یانگ"), ("fi", "Pjongjang"), ("fr", "Pyongyang"), ("ga", "Pyongyang"), ("gl", "Pyongyang"), ("gu", "ફિયો\u{a82}ગયા\u{a82}ગ"), ("he", "פיונגיאנג"), ("hi", "प\u{94d}यो\u{902}गया\u{902}ग"), ("hr", "Pjongjang"), ("hu", "Phenjan"), ("hy", "Փհենյան"), ("id", "Pyongyang"), ("is", "Pjongjang"), ("it", "Pyongyang"), ("ja", "平壌"), ("jv", "Pyongyang"), ("ka", "ფხენიანი"), ("kk", "Пхеньян"), ("km", "ព\u{17d2}យ\u{17bb}ងយ\u{17c9}ាង"), ("kn", "ಪ\u{ccd}ಯೊನ\u{ccd}ಗ\u{ccd}ಯಾಂಗ\u{ccd}"), ("ko", "평양직할시"), ("ky", "Пхеньян"), ("lt", "Pchenjanas"), ("lv", "Phenjana"), ("mk", "Пјонгјанг"), ("ml", "പ\u{d4d}യോംങ\u{d4d}യ\u{d3e}ംഗ\u{d4d}"), ("mn", "Пхеньян"), ("mr", "प\u{94d}या\u{901}गया\u{902}ग"), ("ms", "Pyongyang"), ("my", "ပြ\u{102f}\u{1036}ယမ\u{103a}းမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Pyongyang"), ("ne", "प\u{94d}योङ\u{94d}गयाङ\u{94d}ग"), ("nl", "Pyongyang"), ("no", "Pyongyang"), ("pa", "ਪਿਓ\u{a02}ਗਯਾ\u{a02}ਗ"), ("pl", "Pjongjang"), ("ps", "پيونګيانګ"), ("pt", "Pyongyang"), ("ro", "Phenian"), ("ru", "Пхеньян"), ("si", "ප\u{dd2}යොන\u{dca}යෑං"), ("sk", "Pchjongjang"), ("sl", "Pjongjang"), ("sq", "Pjongian"), ("sr", "Пјонгјанг"), ("sr_Latn", "Pjongjang"), ("sv", "Pyongyang"), ("sw", "Pyongyang"), ("ta", "பியொங\u{bcd}ய\u{bbe}ங\u{bcd}"), ("te", "ప\u{c4d}య\u{c4b}ంగ\u{c4d}య\u{c3e}ంగ\u{c4d}"), ("th", "เป\u{e35}ยงยาง"), ("tk", "Phenýan"), ("tr", "Pyongyang"), ("uk", "Пхеньян"), ("ur", "پیانگ یانگ"), ("uz", "Pxenyan"), ("vi", "Bình Nhưỡng"), ("yo", "Pyongyang"), ("yo_BJ", "Pyongyang"), ("yue", "平壤"), ("yue_Hans", "平壤"), ("zh", "平壤")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::KP,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيونغان الجنوبية"), ("bg", "Южен Пхьонан"), ("bn", "দক\u{9cd}ষিণ পঙ\u{9cd}গ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "P’yongan del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄛\u{11128}𑄠\u{11127}\u{11101}𑄉𑄚\u{11134}"), ("ceb", "P’yŏngan-namdo"), ("cs", "Jižní Pchjŏngan"), ("da", "Sydpyongan"), ("de", "P’yŏngan-namdo"), ("el", "Νότιο Πγιονγκάν"), ("en", "South Pyongan"), ("es", "P’yŏngan del Sur"), ("eu", "Hego Pyongan"), ("fa", "استان پیونگان جنوبی"), ("fi", "Etelä-P’yŏngan"), ("fr", "Pyongan du Sud"), ("gu", "દક\u{acd}ષિણ પ\u{acd}યો\u{a82}ગન પ\u{acd}રા\u{a82}ત"), ("hi", "दक\u{94d}षिण प\u{94d}यो\u{902}गान प\u{94d}रा\u{902}त"), ("hu", "Dél-Phjongan"), ("id", "P’yŏngan Selatan"), ("it", "Sud Pyongan"), ("ja", "平安南道"), ("kn", "ದಕ\u{ccd}ಷ\u{cbf}ಣ ಪಯೋಂಗ\u{ccd}ಗನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "평안남도"), ("lt", "Pietų Pchenjano provincija"), ("lv", "Phjonannamdo province"), ("mn", "Өмнө Пён-Ань"), ("mr", "दक\u{94d}षिण प\u{94d}यो\u{902}गान प\u{94d}रा\u{902}त"), ("ms", "South Pyongan Province"), ("nb", "Sør-Pyongan"), ("nl", "P’yŏngan-namdo"), ("no", "Sør-Pyongan"), ("pl", "P’yŏngan Południowy"), ("pt", "Pyongan Sul"), ("ro", "Provincia Pyongan de Sud"), ("ru", "Пхёнан-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} ප\u{dca}යෝන\u{dca}ගන\u{dca} පළ\u{dcf}ත"), ("sv", "Södra P’yŏngan"), ("ta", "தெற\u{bcd}கு பியோன\u{bcd}கண\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ ప\u{c4d}య\u{c4b}ంగ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพย\u{e47}องอ\u{e31}นใต\u{e49}"), ("tr", "Güney Pyongan"), ("uk", "Південна провінція Пхьонан"), ("ur", "جنوبی پیونگان صوبہ"), ("vi", "Pyongan Nam"), ("yue", "平安南道"), ("yue_Hans", "平安南道"), ("zh", "平安南道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::KP,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيونغان الشمالية"), ("bg", "Северен Пхьонан"), ("bn", "নর\u{9cd}থ পিয\u{9bc}ংগ\u{9be}ন প\u{9cd}রদেশ"), ("ca", "P’yŏngan del Nord"), ("ccp", "𑄅\u{11127}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄛\u{11128}𑄠\u{11127}\u{11101}𑄉𑄚\u{11134}"), ("ceb", "P’yŏngan-bukto"), ("cs", "Severní Pchjŏngan"), ("da", "Nordpyongan"), ("de", "P’yŏngan-pukto"), ("el", "Βόρειο Πιονγκάν"), ("en", "North Pyongan"), ("es", "P’yŏngan del Norte"), ("eu", "Ipar Pyongan"), ("fa", "استان پیونگان شمالی"), ("fi", "Pohjois-P’yŏngan"), ("fr", "Pyongan du Nord"), ("gu", "ઉત\u{acd}તર પ\u{acd}યો\u{a82}ગન પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तरी प\u{94d}यो\u{902}गन प\u{94d}रा\u{902}त"), ("hu", "Észak-Phjongan"), ("id", "P’yŏngan Utara"), ("it", "Nord Pyongan"), ("ja", "平安北道"), ("kn", "ಉತ\u{ccd}ತರ ಪಯೋನ\u{ccd}ಗಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "평안북도"), ("lt", "Šiaurės Pchenjano provincija"), ("lv", "Phjonanpukto province"), ("mn", "Умар Пён-Ань"), ("mr", "उत\u{94d}तर प\u{94d}यो\u{902}गान प\u{94d}रा\u{902}त"), ("ms", "Daerah Pyongan Utara"), ("nb", "Nord-Pyongan"), ("nl", "P’yŏngan-pukto"), ("no", "Nord-Pyongan"), ("pl", "P’yŏngan Północny"), ("pt", "Pyongan Norte"), ("ro", "Provincia Pyongan de Nord"), ("ru", "Пхёнан-Пукто"), ("si", "උත\u{dd4}ර\u{dd4} ප\u{dca}යෝන\u{dca}ගන\u{dca} පළ\u{dcf}ත"), ("sv", "Norra P’yŏngan"), ("ta", "வடக\u{bcd}கு பியொண\u{bcd}கண\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర పయ\u{c4b}న\u{c4d}యన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพย\u{e47}องอ\u{e31}นเหน\u{e37}อ"), ("tr", "Kuzey Pyongan"), ("uk", "Північна провінція Пхьонан"), ("ur", "شمالی پیونگان صوبہ"), ("vi", "Pyongan Bắc"), ("yue", "平安北道"), ("yue_Hans", "平安北道"), ("zh", "平安北道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::KP,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تشاغانغ"), ("bg", "Чаган"), ("bn", "চ\u{9cd}য\u{9be}গ\u{9be}ং প\u{9cd}রদেশ"), ("ca", "Chagang"), ("ccp", "𑄇𑄉\u{11133}𑄠𑄋\u{11134}"), ("ceb", "Chagang-do"), ("cs", "Čagang"), ("cy", "Talaith Chagang"), ("da", "Chagang"), ("de", "Chagang-do"), ("el", "Τσαγκάνγκ"), ("en", "Chagang"), ("es", "Chagang"), ("eu", "Chagang"), ("fa", "چاگانگ"), ("fi", "Chagang"), ("fr", "Jagang"), ("gu", "છગા\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "चगा\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Csagang tartomány"), ("id", "Chagang"), ("it", "Chagang"), ("ja", "慈江道"), ("kn", "ಚಾಗಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "자강도"), ("lt", "Čagango provincija"), ("lv", "Čagando province"), ("mn", "Чаган-ду"), ("mr", "चाग\u{945}\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Daerah Chagang"), ("nb", "Chagang"), ("nl", "Chagang-do"), ("no", "Chagang"), ("pl", "Chagang"), ("pt", "Chagang"), ("ro", "Provincia Chagang"), ("ru", "Чагандо"), ("si", "චග\u{dcf}න\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Chagang"), ("ta", "சகங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "చగ\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดชาก\u{e31}ง"), ("tr", "Chagang"), ("uk", "Провінція Чаган"), ("ur", "چانگانگ صوبہ"), ("vi", "Chagang"), ("yue", "慈江"), ("yue_Hans", "慈江"), ("zh", "慈江道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::KP,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جنوب مقاطعة هوانغاي"), ("bg", "Южен Хванхе"), ("bn", "দক\u{9cd}ষিন হয\u{9bc}\u{9be}ংহে প\u{9cd}রদেশ"), ("ca", "Hwanghae del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄦\u{11127}𑄅\u{1112a}𑄚\u{11134}𑄊"), ("ceb", "Hwanghae-namdo"), ("cs", "Jižní Hwanghä"), ("da", "Sydhwanghae"), ("de", "Hwanghae-namdo"), ("el", "Νότιο Χουάνγκχε (Σάουθ Χουάνγκχε)"), ("en", "South Hwanghae"), ("es", "Hwanghae del Sur"), ("eu", "Hego Hwanghae"), ("fa", "استان هوانگهائه جنوبی"), ("fi", "Etelä-Hwanghae"), ("fr", "Hwanghae du Sud"), ("gu", "દક\u{acd}ષિણ હવા\u{a82}ઘાઈ પ\u{acd}રા\u{a82}ત"), ("hi", "दक\u{94d}षिण ह\u{94d}वा\u{902}ग\u{947} प\u{94d}रा\u{902}त"), ("hu", "Dél-Hvanghe"), ("id", "Hwanghae Selatan"), ("it", "Sud Hwanghae"), ("ja", "黄海南道"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಹಂಗ\u{ccd}ಹೇ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "황해남도"), ("lt", "Pietų Hvanghės provincija"), ("lv", "Hvanhenamdo province"), ("mn", "Өмнө Хуанхэ"), ("mr", "दक\u{94d}षिण व\u{94d}हानघाई प\u{94d}रा\u{902}त"), ("ms", "Daerah Selatan Hwanghae"), ("nb", "Sør-Hwanghae"), ("nl", "Hwanghae-namdo"), ("no", "Sør-Hwanghae"), ("pl", "Hwanghae Południowe"), ("pt", "Hwanghae Sul"), ("ro", "Provincia Hwanghae de Sud"), ("ru", "Хванхэ-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} හ\u{dca}වන\u{dca}ග\u{dca}හය\u{dd2} පළ\u{dcf}ත"), ("sv", "Södra Hwanghae"), ("ta", "தெற\u{bcd}கு வ\u{bbe}ஙகவே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ హవ\u{c3e}ంగ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮว\u{e31}งแฮใต\u{e49} เกาหล\u{e35}เหน\u{e37}อ"), ("tr", "Güney Hwanghae"), ("uk", "Південна провінція Хванхе"), ("ur", "جنوبی ہوانگہائے صوبہ"), ("vi", "Hwanghae Nam"), ("yue", "黃海南道"), ("yue_Hans", "黄海南道"), ("zh", "黄海南道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::KP,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "هوانغهاي الشمالية"), ("bg", "Северен Хванхе"), ("bn", "নর\u{9cd}থ হোয\u{9bc}\u{9be}ংঘে প\u{9cd}রদেশ"), ("ca", "Hwanghae del Nord"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄦\u{11127}𑄅\u{1112a}𑄚\u{11134}𑄊"), ("ceb", "Hwanghae-bukto"), ("cs", "Severní Hwanghä"), ("da", "Nordhwanghae"), ("de", "Hwanghae-pukto"), ("el", "Βόρειο Χουανγκέ"), ("en", "North Hwanghae"), ("es", "Hwanghae del Norte"), ("eu", "Ipar Hwanghae"), ("fa", "استان هوانگهائه شمالی"), ("fi", "Pohjois-Hwanghae"), ("fr", "Hwanghae du Nord"), ("gu", "ઉત\u{acd}તર હ\u{acd}વા\u{a82}ગહ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "उत\u{94d}तरी ह\u{94d}वा\u{902}गह\u{947} प\u{94d}रा\u{902}त"), ("hu", "Észak-Hvanghe"), ("id", "Hwanghae Utara"), ("it", "Nord Hwanghae"), ("ja", "黄海北道"), ("kn", "ಉತ\u{ccd}ತರ ಹ\u{ccd}ವಾಂಗ\u{ccd}ಹ\u{cc6}ಯ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "황해북도"), ("lt", "Šiaurės Hvanghės provincija"), ("lv", "Hvanhepukto province"), ("mn", "Умар Хуанхэ"), ("mr", "उत\u{94d}तर ह\u{94d}व\u{902}ग\u{94d}ह\u{947} प\u{94d}रा\u{902}त"), ("ms", "North Hwanghae Province"), ("nb", "Nord-Hwanghae"), ("nl", "Hwanghae-pukto"), ("no", "Nord-Hwanghae"), ("pl", "Hwanghae Północne"), ("pt", "Hwanghae Norte"), ("ro", "Provincia Hwanghae de Nord"), ("ru", "Хванхэ-Пукто"), ("si", "උත\u{dd4}ර\u{dd4} හ\u{dca}වන\u{dca}හේ පළ\u{dcf}ත"), ("sv", "Norra Hwanghae"), ("ta", "வடக\u{bcd}கு வ\u{bbe}ஙஹயே ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఉత\u{c4d}తర హ\u{c4d}వ\u{c3e}ంగ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "ฮ\u{e31}มกย\u{e47}องใต\u{e49}"), ("tr", "Kuzey Hwanghae"), ("uk", "Північна провінція Хванхе"), ("ur", "شمالی ہوانگہائے صوبہ"), ("vi", "Hwanghae Bắc"), ("yue", "黃海北道"), ("yue_Hans", "黄海北道"), ("zh", "黄海北道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::KP,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كانغوون"), ("bg", "Кануън"), ("bn", "ক\u{9be}ংওন প\u{9cd}রদেশ"), ("ca", "Kangwon"), ("ccp", "𑄇\u{11127}\u{11101}𑄤𑄚\u{11134}"), ("ceb", "Kangwŏn-do"), ("cs", "Kangwŏn"), ("da", "Kangwon"), ("de", "Kangwŏn-do"), ("el", "Κάνγκγουον"), ("en", "Kangwon"), ("es", "Kangwon-do"), ("eu", "Kangwon"), ("fa", "استان کانگوون"), ("fi", "Kangwŏn"), ("fr", "Kangwon"), ("gu", "કા\u{a82}ગવૉન પ\u{acd}રા\u{a82}ત"), ("hi", "का\u{902}गवॉन प\u{94d}रा\u{902}त"), ("hu", "Kangvon"), ("hy", "Կանգվանդո"), ("id", "Kangwon-do"), ("it", "Kangwon"), ("ja", "江原道 (北)"), ("kn", "ಕಾಂಗ\u{ccd}ವಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "강원도"), ("lt", "Kangvono provincija"), ("lv", "Kanvondo province"), ("mn", "Умард Солонгосын Канвон"), ("mr", "का\u{902}गवा\u{901} प\u{94d}रा\u{902}त"), ("ms", "Kangwŏn"), ("nb", "Kangwon"), ("nl", "Kangwŏn-do"), ("no", "Kangwon"), ("pl", "Kangwŏn"), ("pt", "Kangwon"), ("ro", "Provincia Kangwon"), ("ru", "Канвондо"), ("si", "කන\u{dca}ග\u{dca}වොන\u{dca} පළ\u{dcf}ත"), ("sv", "Kangwon"), ("ta", "க\u{bbe}ங\u{bcd}வ\u{bcd}ன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "కంగ\u{c4d}వ\u{c3e}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e31}งว\u{e47}อน"), ("tr", "Kangvon"), ("uk", "Провінція Канвон"), ("ur", "کانگوون صوبہ"), ("vi", "Kangwon"), ("yue", "江原道"), ("yue_Hans", "江原道"), ("zh", "江原道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::KP,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة هامغيونغ الجنوبية"), ("bg", "Южен Хамгьон"), ("bn", "দক\u{9cd}ষিন হ\u{9cd}য\u{9be}মইয\u{9bc}ং প\u{9cd}রদেশ"), ("ca", "Hamgyong del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134} 𑄦𑄟\u{11134}𑄎\u{11128}𑄠\u{11127}\u{11101}"), ("ceb", "Hamnam"), ("cs", "Jižní Hamgjŏng"), ("da", "Syd-Hamgyong"), ("de", "Hamgyŏng-namdo"), ("el", "Σάουθ Χάμγκιονγκ"), ("en", "South Hamgyong"), ("es", "Hamgyŏng del Sur"), ("eu", "Hego Hamgyong"), ("fa", "استان هامگیونگ جنوبی"), ("fi", "Etelä-Hamgyŏng"), ("fr", "Hamgyong du Sud"), ("gu", "દક\u{acd}ષિણ હ\u{ac7}મગ\u{acd}યો\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "दक\u{94d}षिण हमग\u{94d}यो\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Dél-Hamgjong"), ("id", "Hamgyong Selatan"), ("it", "Sud Hamgyong"), ("ja", "咸鏡南道"), ("kn", "ಸ\u{ccc}ತ\u{ccd} ಹ\u{ccd}ಯಾಮ\u{ccd}ಯಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "함경남도"), ("lt", "Pietų Hamgiongo provincija"), ("lv", "Hamgjonnamdo province"), ("mn", "Өмнө Хамгён"), ("mr", "दक\u{94d}षिण ह\u{945}मगो\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "South Hamgyong Province"), ("nb", "Sør-Hamgyong"), ("ne", "दक\u{94d}षिण हाम\u{94d}गयोग क\u{94d}ष\u{947}त\u{94d}र"), ("nl", "Hamgyŏng-namdo"), ("no", "Sør-Hamgyong"), ("pl", "Hamgyŏng Południowy"), ("pt", "Hamgyong Sul"), ("ro", "Provincia Hamgyong de Sud"), ("ru", "Хамгён-Намдо"), ("si", "දක\u{dd4}ණ\u{dd4} හැම\u{dca}යොන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Södra Hamgyong"), ("ta", "தெற\u{bcd}கு ஹம\u{bcd}ஜியோங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "దక\u{c4d}ష\u{c3f}ణ హ\u{c3e}మ\u{c4d}\u{200c}గ\u{c4d}య\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดฮ\u{e31}มพย\u{e47}องใต\u{e49}"), ("tr", "Güney Hamgyong"), ("uk", "Південна провінція Хамгьон"), ("ur", "جنوبی ہامگیونگ صوبہ"), ("vi", "Hamgyong Nam"), ("yue", "咸鏡南道"), ("yue_Hans", "咸镜南道"), ("zh", "咸镜南道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::KP,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "منطقة شمال هامغيونغ"), ("az", "Hamqyon-Pukto"), ("be", "Хамгён-Пукта"), ("bg", "Северен Хамгьон"), ("ca", "Hamgyong del Nord"), ("ccp", "𑄅\u{11127}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134} 𑄦𑄟\u{11134}𑄎\u{11128}𑄠\u{11127}\u{11101}"), ("ceb", "Hambuk"), ("cs", "Severní Hamgjŏng"), ("cy", "Talaith Gogledd Hamgyong"), ("da", "Nord-Hamgyong"), ("de", "Hamgyŏng-pukto"), ("en", "North Hamgyong"), ("es", "Hamgyŏng del Norte"), ("eu", "Ipar Hamgyong"), ("fa", "استان هامگیونگ شمالی"), ("fi", "Pohjois-Hamgyŏng"), ("fr", "Hamgyong du Nord"), ("he", "צפון האמגיונג"), ("hu", "Észak-Hamgjong"), ("id", "Hamgyong Utara"), ("it", "Nord Hamgyong"), ("ja", "咸鏡北道"), ("ko", "함경북도"), ("lt", "Šiaurės Hamgiongo provincija"), ("mn", "Умар Хамгён"), ("nb", "Nord-Hamgyong"), ("nl", "Hamgyŏng-pukto"), ("no", "Nord-Hamgyong"), ("pl", "Hamgyŏng Północny"), ("pt", "Hamgyong Norte"), ("ro", "Provincia Hamgyong de Nord"), ("ru", "Хамгён-Пукто"), ("sl", "Provinca Južni Hamgyong"), ("sv", "Norra Hamgyong"), ("tr", "Kuzey Hamgyong"), ("uk", "Північна провінція Хамгьон"), ("ur", "شمالی ہامگیونگ صوبہ"), ("vi", "Hamgyong Bắc"), ("yue", "咸鏡北道"), ("yue_Hans", "咸镜北道"), ("zh", "咸镜北道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::KP,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريانغانغ"), ("bg", "Рянган"), ("bn", "রিয\u{9bc}\u{9be}ংগ\u{9be}ং"), ("ca", "Ryanggang"), ("ccp", "𑄢\u{11128}𑄠\u{11101}𑄉\u{11127}\u{11101}"), ("ceb", "Yanggang-do"), ("cs", "Rjanggang"), ("cy", "Talaith Ryanggang"), ("da", "Ryanggang"), ("de", "Ryanggang-do"), ("el", "Ριανγκάνγκ"), ("en", "Ryanggang"), ("es", "Ryanggang"), ("eu", "Ryanggang"), ("fa", "استان ریانگ\u{200c}گانگ"), ("fi", "Ryanggang"), ("fr", "Ryanggang"), ("gu", "રાય\u{a82}ગગ\u{ac7}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "र\u{947}या\u{902}गग\u{948}\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Rjanggang"), ("id", "Ryanggang"), ("it", "Ryanggang"), ("ja", "両江道"), ("kn", "ರಯಾಂಗ\u{ccd}ಗಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "량강도"), ("lt", "Jangango provincija"), ("lv", "Rjangando province"), ("mn", "Янган"), ("mr", "रिया\u{902}गग\u{945}\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Wilayah Ryanggang"), ("nb", "Ryanggang"), ("nl", "Ryanggang-do"), ("no", "Ryanggang"), ("pl", "Ryanggang"), ("pt", "Ryanggang"), ("ro", "Provincia Ryanggang"), ("ru", "Янгандо"), ("si", "රයන\u{dca}ග\u{dcf}න\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Ryanggang"), ("ta", "ரியங\u{bcd}கங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ర\u{c3f}య\u{c3e}ంగ\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "รย\u{e31}งก\u{e31}ง"), ("tr", "Ryanggang"), ("uk", "Провінція Янган"), ("ur", "ریانگگانگ صوبہ"), ("vi", "Ryanggang"), ("yue", "兩江道"), ("yue_Hans", "两江道"), ("zh", "两江道")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::KP,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::SpecialCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "راسون"), ("az", "Rason"), ("bg", "Расън"), ("bn", "র\u{9be}সোন"), ("ca", "Rasŏn"), ("ccp", "𑄢𑄥\u{11127}𑄚\u{11134}"), ("ceb", "Rason"), ("cs", "Rasŏn"), ("da", "Rason"), ("de", "Rasŏn"), ("el", "Ράσον"), ("en", "Rason"), ("es", "Rasŏn"), ("eu", "Rason"), ("fa", "راسون"), ("fi", "Rasŏn"), ("fr", "Rasŏn"), ("ga", "Rason"), ("gu", "રાસન"), ("hi", "रासो\u{902}"), ("hu", "Raszon"), ("id", "Rason"), ("it", "Rasŏn"), ("ja", "羅先特別市"), ("kn", "ರಾಸನ\u{ccd}"), ("ko", "라선특별시"), ("lt", "Rasonas"), ("lv", "Rasona"), ("mr", "रासन"), ("ms", "Rason"), ("nb", "Rason"), ("nl", "Rasŏn"), ("no", "Rason"), ("pl", "Rasŏn"), ("pt", "Rason"), ("ro", "Rasŏn"), ("ru", "Насон"), ("si", "රසෝන\u{dca}"), ("sv", "Rason"), ("ta", "ர\u{bbe}சன\u{bcd}"), ("te", "ర\u{c3e}సన\u{c4d}"), ("th", "ราซ\u{e47}อน"), ("tr", "Rason"), ("uk", "Насон"), ("ur", "راسون"), ("vi", "Rason"), ("yue", "羅先"), ("yue_Hans", "罗先"), ("zh", "羅先特別市")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::KP,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::MetropolitanCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "نامبو"), ("az", "Nampxo"), ("bg", "Нампхо"), ("ca", "Namp’o"), ("ceb", "Namp’o"), ("cs", "Nampcho"), ("da", "Nampho"), ("de", "Namp’o"), ("en", "Nampho"), ("es", "Namp’o"), ("et", "Namp’o"), ("eu", "Nampo"), ("fa", "نامپو"), ("fi", "Namp’o"), ("fr", "Nampo"), ("he", "נאמפו"), ("hu", "Nampho"), ("it", "Namp’o"), ("ja", "南浦特別市"), ("ko", "남포특별시"), ("lt", "Nampo"), ("mn", "Нампу"), ("ms", "Nampo"), ("nb", "Nampo"), ("nl", "Namp’o"), ("no", "Nampo"), ("pl", "Namp’o"), ("pt", "Nampo"), ("ro", "Namp’o"), ("ru", "Нампхо"), ("sv", "Nampo"), ("sw", "Nampho"), ("th", "น\u{e31}มโพ"), ("tr", "Nampo"), ("uk", "Нампхо"), ("ur", "نامپو"), ("vi", "Nampho"), ("yue", "南浦特別市"), ("yue_Hans", "南浦特别市"), ("zh", "南浦特別市")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "kp")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::KP,
        alpha3: Alpha3::PRK,
        address_format: None,
        continent: Continent::Asia,
        country_code: 850,
        currency_code: "KPW",
        gec: Some(GEC::KN),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("PRK"),
        iso_long_name: "The Democratic People's Republic of Korea",
        iso_short_name: "Korea (Democratic People's Republic of)",
        official_language_list: ["ko"].to_vec(),
        spoken_language_list: ["ko"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("North Korean"),
        number: "408",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAsia),
        un_locode: "KP",
        unofficial_name_list: ["Korea (North)", "North Korea", "Nordkorea", "Corée du Nord", "Corea del Norte", "朝鮮民主主義人民共和国", "Noord-Korea", "Korea Democratic People's Republic", "Korea (Democratic People s Republic of)"].to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "North Korea"), ("af", "Noord-Korea"), ("ak", "North Korea"), ("am", "ሰሜናዊ ኮሪያ"), ("an", "North Korea"), ("ar", "كوريا، جمهورية كوريا الش\u{651}عبي\u{651}ة الد\u{651}يموقراطي\u{651}ة"), ("as", "কোৰিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}ৰিক গণপ\u{9cd}ৰজ\u{9be}তন\u{9cd}ত\u{9cd}ৰী"), ("ay", "North Korea"), ("az", "Koreya, Demokratık Xalq Respublikası"), ("ba", "North Korea"), ("be", "Карэя, Народна-Дэмакратычная Рэспубліка"), ("bg", "Корея, Демократична народна република"), ("bi", "North Korea"), ("bn", "কোরিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}রিক গণপ\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}রী"), ("bn_IN", "কোরিয়\u{9be}, গণত\u{9be}ন\u{9cd}ত\u{9cd}রিক গণপ\u{9cd}রজ\u{9be}তন\u{9cd}ত\u{9cd}রী"), ("br", "North Korea"), ("bs", "Koreja, Demokratska Narodna Republika"), ("ca", "Corea del Nord"), ("ce", "North Korea"), ("ch", "North Korea"), ("cs", "Korea, lidově demokratická republika"), ("cv", "North Korea"), ("cy", "Korea (Gweriniaeth Democrataidd Pobl)"), ("da", "Korea, Den Demokratiske Folkerepublik"), ("de", "Nordkorea"), ("dv", "North Korea"), ("dz", "ཀ\u{f7c}་ར\u{f72}་ཡ། ཌ\u{f7a}་མ\u{f7c}་ཀ\u{f7a}ར\u{f7a}་ཊ\u{f72}ཀ་ པ\u{f72}་པ\u{f71}ལས\u{f72}་ ར\u{f72}་པཔ་ལ\u{f72}ཀ་ ཨ\u{f7c}ཕ་"), ("ee", "North Korea"), ("el", "Κορέα, Λαοκρατική Δημοκρατία της"), ("en", "North Korea"), ("eo", "Nord-Koreio"), ("es", "Corea, República Democrática Popular de"), ("et", "Põhja-Korea"), ("eu", "Korea, Herri Errepublika Demokratikoa"), ("fa", "کره، جمهوری دموکراتیک خلق"), ("ff", "North Korea"), ("fi", "Korean demokraattinen kansantasavalta"), ("fo", "North Korea"), ("fr", "Corée du Nord"), ("fy", "North Korea"), ("ga", "Daonphoblacht Dhaonlathach na Cóiré"), ("gl", "Corea, República Democrática Popular de"), ("gn", "North Korea"), ("gu", "ન\u{acd}ય\u{ac1} ક\u{ac7}લ\u{ac7}ડોનિયા"), ("gv", "North Korea"), ("ha", "North Korea"), ("he", "קוריאה הצפונית"), ("hi", "उत\u{94d}तर कोरिया"), ("hr", "Sjeverna Koreja"), ("ht", "North Korea"), ("hu", "Koreai Népi Demokratikus Köztársaság"), ("hy", "Կորեա, Ժողովրդադեմոկրատական Հանրապետություն"), ("ia", "Corea, Republica Popular Democratic de"), ("id", "Korea Utara"), ("io", "North Korea"), ("is", "Norður-Kórea"), ("it", "Corea del Nord"), ("iu", "North Korea"), ("ja", "朝鮮民主主義人民共和国"), ("ka", "კორეის სახალხო-დემოკრატიული რესპუბლიკა"), ("ki", "North Korea"), ("kk", "Корея (Солтүстік)"), ("kl", "North Korea"), ("km", "ក\u{17bc}រ\u{17c9}េ សាធារណ\u{200b}រដ\u{17d2}ឋ\u{200b}ប\u{17d2}រជាមាន\u{17b7}ត\u{200b}ប\u{17d2}រជាធ\u{17b7}បតេយ\u{17d2}យ\u{200b}\u{200b}នៃ"), ("kn", "ಕೋರ\u{cbf}ಯಾ, ಡ\u{cc6}ಮೊಕ\u{ccd}ರಟ\u{cbf}ಕ\u{ccd} ಪೀಪಲ\u{ccd}ಸ\u{ccd} ರ\u{cbf}ಪಬ\u{ccd}ಲ\u{cbf}ಕ\u{ccd}"), ("ko", "조선민주주의인민공화국"), ("ku", "Kore, Komara Demokratîk a Gelê"), ("kv", "North Korea"), ("kw", "North Korea"), ("ky", "Корея Элдик Демократиялык Республикасы"), ("lo", "North Korea"), ("lt", "Korėjos Liaudies Demokratinė Respublika"), ("lv", "Ziemeļkoreja"), ("mi", "North Korea"), ("mk", "Кореја, Демократска народна република"), ("ml", "കൊറിയ, ഡെമോക\u{d4d}ര\u{d3e}റ\u{d4d}റിക\u{d4d} പീപ\u{d4d}പിള\u{d4d}\u{200d}സ\u{d4d} റിപ\u{d4d}പബ\u{d4d}ലിക\u{d4d} ഓഫ\u{d4d}"), ("mn", "Ардчилсан солонгос улс"), ("mr", "कोरिया, ड\u{947}मोक\u{94d}र\u{945}टिक पिपल\u{94d}स रिपब\u{94d}लिक ऑफ"), ("ms", "North Korea"), ("mt", "North Korea"), ("my", "North Korea"), ("na", "North Korea"), ("nb", "Nord-Korea"), ("ne", "कोरियाको प\u{94d}रजातन\u{94d}त\u{94d}रिक जनगणराज\u{94d}य"), ("nl", "Noord-Korea"), ("nn", "Nord-Korea"), ("nv", "North Korea"), ("oc", "Corèa del Nòrd"), ("or", "କୋର\u{b3f}ଆ, ସର\u{b4d}ବସ\u{b3e}ଧ\u{b3e}ରଣ ଜନସମ\u{b3e}ଜ ଗଣତନ\u{b4d}ତ\u{b4d}ର"), ("pa", "ਕ\u{a4b}ਰੀਆ, ਲ\u{a4b}ਕਤ\u{a70}ਤਰੀ ਗਣਰਾਜ"), ("pi", "North Korea"), ("pl", "Korea Północna"), ("ps", "North Korea"), ("pt", "Coreia do Norte"), ("pt_BR", "Coreia do Norte"), ("ro", "Republica democrată populară Coreea"), ("ru", "Северная Корея"), ("rw", "Koreya, Repubulika Iharanira Demokarasi ya Rubanda ya"), ("sc", "Corea de su Nord"), ("sd", "North Korea"), ("si", "සම\u{dcf}ජව\u{dcf}ද\u{dd3} මහජන කොර\u{dd2}ය\u{dcf} ජනරජය"), ("sk", "Kórejská ľudovodemokratická republika"), ("sl", "Severna Koreja"), ("so", "North Korea"), ("sq", "Korea, Republika Popullore Demokratike e"), ("sr", "Кореја, Демократска Народна Република"), ("sv", "Nordkorea"), ("sw", "North Korea"), ("ta", "கொரிய ஜனந\u{bbe}யக மக\u{bcd}கள\u{bcd} குடியரசு"), ("te", "క\u{c4b}ర\u{c3f}య\u{c3e}, డ\u{c46}మ\u{c4b}క\u{c4d}రట\u{c3f}క\u{c4d} ప\u{c40}పుల\u{c4d}స\u{c4d} ర\u{c3f}పబ\u{c4d}ల\u{c3f}క\u{c4d} ఆఫ\u{c4d}"), ("tg", "Ҷумҳурии Халқӣ-Демократии Корея"), ("th", "เกาหล\u{e35}เหน\u{e37}อ"), ("ti", "North Korea"), ("tk", "Koreýa, Halk Demokratik Respublikasy"), ("tl", "Korea, Taumbayang Demokratikong Republika of"), ("tr", "Kuzey Kore"), ("tt", "North Korea"), ("ug", "چاۋشيەن"), ("uk", "Північна Корея"), ("ur", "North Korea"), ("uz", "North Korea"), ("ve", "North Korea"), ("vi", "Bắc Hàn, Cộng hoà Nhân dân Dân chủ"), ("wa", "Corêye (bijhrece)"), ("wo", "Koore, Republik Popileer Demokaraatik bu"), ("xh", "North Korea"), ("yo", "North Korea"), ("zh_CN", "朝鲜"), ("zh_HK", "北韓"), ("zh_TW", "朝鮮民主主義人民共和國"), ("zu", "North Korea")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}