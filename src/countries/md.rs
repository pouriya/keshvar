// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Moldova

#[cfg(all(feature = "md", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MD;
    pub const ALPHA3: Alpha3 = Alpha3::MDA;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 373;
    pub const CURRENCY_CODE: &str = "MDL";
    pub const GEC: Option<GEC> = Some(GEC::MD);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::MDA);
    pub const ISO_SHORT_NAME: &str = "Moldova (Republic of)";
    pub const ISO_LONG_NAME: &str = "The Republic of Moldova";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ro"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ro"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Moldovan");
    pub const NUMBER: &str = "498";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "MD";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Moldova",
        "Moldawien",
        "Moldavie",
        "Moldavia",
        "the Republic of Moldova",
        "モルドバ共和国",
        "Moldavië",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Moldova"),
        ("af", "Moldawië"),
        ("ak", "Moldova"),
        ("am", "ሞልዶቫ"),
        ("an", "Moldova"),
        ("ar", "المالديف"),
        ("as", "মোলডোভ\u{9be}"),
        ("ay", "Moldova"),
        ("az", "Moldova"),
        ("ba", "Moldova"),
        ("be", "Малдова"),
        ("bg", "Молдова"),
        ("bi", "Moldova"),
        ("bn", "মোলডোভ\u{9be}"),
        ("bn_IN", "মোলডোভ\u{9be}"),
        ("br", "Moldova"),
        ("bs", "Moldavija"),
        ("ca", "Moldàvia"),
        ("ce", "Молдави"),
        ("ch", "Moldova"),
        ("cs", "Moldavsko"),
        ("cv", "Молдави"),
        ("cy", "Moldofa"),
        ("da", "Moldova"),
        ("de", "Moldau"),
        ("dv", "މ\u{7ae}ލ\u{7b0}ޑ\u{7af}ވ\u{7a7}"),
        ("dz", "མ\u{f7c}ལ་ཌ\u{f7c}་ཝ།"),
        ("ee", "Moldova"),
        ("el", "Μολδαβία"),
        ("en", "Moldova"),
        ("eo", "Moldavio"),
        ("es", "Moldavia"),
        ("et", "Moldova"),
        ("eu", "Moldavia"),
        ("fa", "مولداوی"),
        ("ff", "Moldowa"),
        ("fi", "Moldova"),
        ("fo", "Moldova"),
        ("fr", "Moldavie"),
        ("fy", "Moldaavje"),
        ("ga", "An Mholdóiv"),
        ("gl", "Moldavia"),
        ("gn", "Moldova"),
        ("gu", "મોલ\u{acd}ડોવા"),
        ("gv", "Moldova"),
        ("ha", "MOldufiniya"),
        ("he", "מולדובה"),
        ("hi", "मॉल\u{94d}डोवा"),
        ("hr", "Moldavija"),
        ("ht", "Moldavi"),
        ("hu", "Moldova"),
        ("hy", "Մոլդովա"),
        ("ia", "Moldavia"),
        ("id", "Moldova"),
        ("io", "Moldova"),
        ("is", "Moldavía"),
        ("it", "Moldavia"),
        ("iu", "Moldova"),
        ("ja", "モルドバ"),
        ("ka", "მოლდოვა"),
        ("ki", "Moldova"),
        ("kk", "Молдова"),
        ("kl", "Moldova"),
        ("km", "ម\u{17bb}លឌ\u{17bc}វ\u{17c9}ា"),
        ("kn", "Moldova"),
        ("ko", "몰도바"),
        ("ku", "Moldova"),
        ("kv", "Молдова"),
        ("kw", "Moldova"),
        ("ky", "Молдова"),
        ("lo", "Moldova"),
        ("lt", "Moldova"),
        ("lv", "Moldova"),
        ("mi", "Morotawa"),
        ("mk", "Молдавија"),
        ("ml", "മോള\u{d4d}\u{200d}ഡോവ"),
        ("mn", "Молдав"),
        ("mr", "मोल\u{94d}डोवा"),
        ("ms", "Moldova"),
        ("mt", "Moldova"),
        (
            "my",
            "မော\u{103a}လ\u{103a}ဒ\u{102d}\u{102f}ဗာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ripubrikin Mordowa"),
        ("nb", "Moldova"),
        ("ne", "मोल\u{94d}डोभा"),
        ("nl", "Moldavië"),
        ("nn", "Moldova"),
        ("nv", "Moldova"),
        ("oc", "Moldàvia"),
        ("or", "ମ\u{b3e}ଲଡୋଭ\u{b3e}"),
        ("pa", "ਮ\u{a4b}ਲਡੀਵਾ"),
        ("pi", "मोल\u{94d}दोवा"),
        ("pl", "Mołdawia"),
        ("ps", "مولدوا"),
        ("pt", "Moldávia"),
        ("pt_BR", "Moldávia"),
        ("ro", "Moldova"),
        ("ru", "Молдавия"),
        ("rw", "Molidova"),
        ("sc", "Moldàvia"),
        ("sd", "Moldova"),
        ("si", "මොල\u{dca}ඩෝව\u{dcf}"),
        ("sk", "Moldavsko"),
        ("sl", "Moldavija"),
        ("so", "Moldofa"),
        ("sq", "Moldavi"),
        ("sr", "Молдавија"),
        ("sv", "Moldavien"),
        ("sw", "Moldova"),
        ("ta", "மோல\u{bcd}டோவ\u{bbe}"),
        ("te", "మ\u{c3e}ల\u{c4d}డ\u{c4b}వ\u{c3e}"),
        ("tg", "Молдова"),
        ("th", "มอลโดวา"),
        ("ti", "Moldova"),
        ("tk", "Moldawiýa"),
        ("tl", "Moldova"),
        ("tr", "Moldova"),
        ("tt", "Moldova"),
        ("ug", "مولدوۋا"),
        ("uk", "Молдова"),
        ("ur", "مالدووا"),
        ("uz", "Moldova"),
        ("ve", "Moldova"),
        ("vi", "Mon-đô-va"),
        ("wa", "Moldova"),
        ("wo", "Moldaawi"),
        ("xh", "Moldova"),
        ("yo", "Móldófà"),
        ("zh_CN", "摩尔多瓦"),
        ("zh_HK", "摩爾多瓦"),
        ("zh_TW", "摩爾多瓦"),
        ("zu", "Moldova"),
    ];
    #[cfg(all(feature = "md", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 47.411631;
        pub const LONGITUDE: f64 = 28.369885;
        pub const MAX_LATITUDE: f64 = 48.492029;
        pub const MAX_LONGITUDE: f64 = 30.1635898;
        pub const MIN_LATITUDE: f64 = 45.4674379;
        pub const MIN_LONGITUDE: f64 = 26.6164248;
        pub const NORTHEAST_LATITUDE: f64 = 48.492029;
        pub const NORTHEAST_LONGITUDE: f64 = 30.1635898;
        pub const SOUTHWEST_LATITUDE: f64 = 45.4674379;
        pub const SOUTHWEST_LONGITUDE: f64 = 26.6164248;
    }
}
#[cfg(all(feature = "md", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 47.411631,
            longitude: 28.369885,
            max_latitude: 48.492029,
            max_longitude: 30.1635898,
            min_latitude: 45.4674379,
            min_longitude: 26.6164248,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 48.492029,
                    longitude: 30.1635898,
                },
                southwest: CountryGeoBound {
                    latitude: 45.4674379,
                    longitude: 26.6164248,
                },
            },
        }
    }
}

#[cfg(all(feature = "md", feature = "subdivisions"))]
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
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::MD,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أنيني نوي"), ("bg", "Новоаненски район"), ("bn", "আনেনি নোই জেল\u{9be}"), ("ca", "Districte d’Anenii Noi"), ("ccp", "𑄃𑄚𑄬𑄚\u{11128} 𑄚\u{11130}"), ("ceb", "Anenii Noi"), ("cs", "Okres Anenii Noi"), ("da", "Anenii Noi District"), ("de", "Rajon Anenii Noi"), ("el", "Ανένιι Νόι"), ("en", "Anenii Noi"), ("es", "Distrito de Anenii Noi"), ("fi", "Anenii Noin piiri"), ("fr", "Raion d’Anenii Noi"), ("gu", "અન\u{ac7}ની નોઇ જિલ\u{acd}લો"), ("hi", "अननीई नोई जिला"), ("hu", "Anenii Noi járás"), ("id", "Raionul Anenii Noi"), ("it", "distretto di Anenii Noi"), ("ja", "アネニイ・ノイ県"), ("ka", "ანენი-ნოის რაიონი"), ("kn", "ಅನ\u{cc6}ನ\u{cbf} ನೋಯ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "아네니노이 구"), ("lt", "Naujųjų Anėnų rajonas"), ("lv", "Aneni Nojas rajons"), ("mr", "अननी नोई जिल\u{94d}हा"), ("ms", "Anenii Noi District"), ("nb", "Anenii Noi"), ("nl", "Anenii Noi"), ("no", "Anenii Noi"), ("pl", "Rejon Anenii Noi"), ("pt", "Anenii Noi (condado)"), ("ro", "raionul Anenii Noi"), ("ru", "Новоаненский район"), ("si", "අනෙන\u{dca}ල\u{dd2} නොය\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Anenii Noi (okres)"), ("sq", "Raionul Anenii Noi"), ("sv", "Anenii Noi (distrikt)"), ("ta", "அனேந\u{bc0} ந\u{bbe}ய\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "అ\u{c46}న\u{c40} న\u{c4b}య\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อาเนอ\u{e34}ลนอย"), ("tr", "Anenii Noi District"), ("uk", "Аненій-Нойський район"), ("ur", "انینئی نوی ضلع"), ("vi", "Quận Anenii Noi"), ("zh", "新阿內尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::MD,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.7539947), longitude: Some(27.9184148), max_latitude: Some(47.81119469999999), min_latitude: Some(47.7226969), max_longitude: Some(27.9682947), min_longitude: Some(27.8344632)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bălţi"), ("am", "በልጺ"), ("ar", "بالتسي"), ("az", "Belsı"), ("be", "Горад Бельцы"), ("bg", "Белци"), ("bn", "ব\u{9be}লতি"), ("bs", "Bălți"), ("ca", "Bălți"), ("ccp", "𑄝𑄣\u{11134}𑄑\u{11128}"), ("ceb", "Municipiul Bălți"), ("cs", "Bălți"), ("cy", "Bălţi"), ("da", "Bălți"), ("de", "Bălți"), ("el", "Μπέλτσι"), ("en", "Bălţi"), ("es", "Bălți"), ("et", "Bălți"), ("eu", "Bălți"), ("fa", "بالتی"), ("fi", "Bălți"), ("fr", "Municipalité de Bălți"), ("ga", "Bălți"), ("gu", "બાલ\u{acd}ટી"), ("he", "בלץ"), ("hi", "बाल\u{94d}टी"), ("hr", "Bălți"), ("hu", "Bălți"), ("hy", "Բելցի"), ("id", "Bălţi"), ("is", "Bălţi"), ("it", "Bălți"), ("ja", "バルツィ"), ("ka", "ბელცი"), ("kn", "ಬಾಲ\u{ccd}ಟ\u{cbf}"), ("ko", "벌치"), ("lt", "Belcis"), ("lv", "Belci"), ("mk", "Белци"), ("mn", "Балць"), ("mr", "बाल\u{94d}टी"), ("ms", "Bălţi"), ("nb", "Bălți"), ("nl", "Bălți"), ("no", "Bălți"), ("pl", "Bielce"), ("pt", "Bălţi"), ("ro", "Municipiul Bălți"), ("ru", "Бельцы"), ("si", "බ\u{dcf}ල\u{dca}ට\u{dd2}"), ("sk", "Bălţi"), ("sl", "Bălţi"), ("sr", "Балци"), ("sr_Latn", "Balci"), ("sv", "Bălţi"), ("sw", "Bălţi"), ("ta", "ப\u{bbe}லட\u{bc0}"), ("te", "బ\u{c3e}ల\u{c4d}ట\u{c3f}"), ("th", "บ\u{e31}ลตส\u{e4c}"), ("tr", "Bălţi"), ("uk", "Бєльці"), ("ur", "بالتسی"), ("uz", "Bălţi"), ("vi", "Bălţi"), ("yue", "伯爾茲"), ("yue_Hans", "伯尔兹"), ("zh", "伯爾茲")]),
                        unofficial_name_list: ["Balti"].to_vec(),
                    }
                ),
                (
                    "BD",
                    Subdivision{
                        name: "BD",
                        country_alpha2: Alpha2::MD,
                        code: "BD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Bender"), ("ar", "بيندر، مولدوفيا"), ("az", "Bender"), ("be", "Горад Бендэр"), ("bg", "Бендери"), ("bn", "বেন\u{9cd}ড\u{9be}র"), ("bs", "Bender"), ("ca", "Bender"), ("ccp", "𑄝𑄬𑄚\u{11134}𑄓𑄢\u{11134}"), ("ceb", "Bender"), ("cs", "Bender"), ("cy", "Bender"), ("da", "Bender"), ("de", "Bender"), ("el", "Μπέντερ"), ("en", "Bender"), ("es", "Bender"), ("et", "Bender"), ("eu", "Bender"), ("fa", "بندر، مولداوی"), ("fi", "Bender"), ("fr", "Municipalité de Bender"), ("ga", "Bender"), ("gl", "Bender"), ("gu", "બ\u{ac7}ન\u{acd}ડર"), ("ha", "Bender"), ("ha_NE", "Bender"), ("he", "טיגינה"), ("hi", "ब\u{947}\u{902}डर"), ("hr", "Bender"), ("hu", "Bender"), ("hy", "Բենդերի"), ("id", "Bender"), ("ig", "Bender"), ("is", "Bender"), ("it", "Bender"), ("ja", "ベンデル"), ("jv", "Bender"), ("ka", "ბენდერი"), ("kn", "ಬ\u{cc6}ಂಡರ\u{ccd}"), ("ko", "벤데르"), ("lt", "Benderai"), ("lv", "Bendera"), ("mk", "Бендер"), ("mr", "ब\u{947}न\u{94d}डर"), ("ms", "Bender"), ("nb", "Bender"), ("nl", "Bender"), ("no", "Bender"), ("pl", "Bender"), ("pt", "Bender"), ("ro", "Municipiul Bender"), ("ru", "Бендеры"), ("si", "බෙන\u{dca}ඩෙර\u{dca}"), ("sk", "Bender"), ("sl", "Bender"), ("so", "Bender"), ("sr", "Бендер"), ("sr_Latn", "Bender"), ("sv", "Bender"), ("sw", "Bender"), ("ta", "பெண\u{bcd}டெர\u{bcd}"), ("te", "బ\u{c46}ండ\u{c46}ర\u{c4d}"), ("th", "เบนเดอร\u{e4c}"), ("tr", "Bender"), ("uk", "Бендери"), ("ur", "بیندر، مالدووا"), ("uz", "Bender"), ("vi", "Bender"), ("yo", "Bender"), ("yo_BJ", "Bender"), ("zh", "賓傑里"), ("zu", "Bender")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BR",
                    Subdivision{
                        name: "BR",
                        country_alpha2: Alpha2::MD,
                        code: "BR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بريسيني"), ("be", "Брычанскі раён"), ("bg", "Бриченски район"), ("bn", "ব\u{9cd}রিকেনি জেল\u{9be}"), ("ca", "Districte de Briceni"), ("ccp", "𑄝\u{11133}𑄢\u{11128}𑄥𑄬𑄚\u{11128}"), ("ceb", "Briceni"), ("cs", "Okres Briceni"), ("da", "Briceni District"), ("de", "Rajon Briceni"), ("el", "Μπριτσένι"), ("en", "Briceni"), ("es", "Distrito de Briceni"), ("fi", "Bricenin piiri"), ("fr", "Raion de Briceni"), ("gu", "બ\u{acd}રિસ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "ब\u{94d}रिस\u{947}नी जिला"), ("hu", "Briceni járás"), ("id", "Raionul Briceni"), ("it", "distretto di Briceni"), ("ja", "ブリチェニー県"), ("ka", "ბრიჩენის რაიონი"), ("kn", "ಬ\u{ccd}ರ\u{cbf}ಕ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "브리체니 구"), ("lt", "Bričenio rajonas"), ("lv", "Bričenu rajons"), ("mr", "ब\u{94d}रिसनी जिल\u{94d}हा"), ("ms", "Briceni District"), ("nb", "Briceni"), ("nl", "Briceni"), ("no", "Briceni"), ("pl", "Rejon Briceni"), ("pt", "Briceni"), ("ro", "raionul Briceni"), ("ru", "Бричанский район"), ("si", "බ\u{dca}\u{200d}ර\u{dd2}සෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Briceni"), ("sq", "Raionul Briceni"), ("sv", "Briceni (distrikt)"), ("ta", "பிரிசினி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3f}స\u{c47}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องเอสเตล\u{e35}"), ("tr", "Briceni District"), ("uk", "Бричанський район"), ("ur", "بریچینی ضلع"), ("vi", "Quận Briceni"), ("zh", "布里切尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "BS",
                    Subdivision{
                        name: "BS",
                        country_alpha2: Alpha2::MD,
                        code: "BS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باسارابيسكا"), ("bg", "Бесарабски район"), ("bn", "ব\u{9be}শ\u{9be}র\u{9be}বিয\u{9bc}\u{9be}স\u{9cd}ক\u{9be} জেল\u{9be}"), ("ca", "Districte de Basarabeasca"), ("ccp", "𑄝𑄥𑄢\u{11134}𑄝\u{11128}𑄠𑄌\u{11134}𑄇"), ("ceb", "Basarabeasca"), ("cs", "Okres Basarabeasca"), ("da", "Basarabeasca District"), ("de", "Rajon Basarabeasca"), ("el", "Μπασαραμπεάσκα"), ("en", "Basarabeasca"), ("es", "Distrito de Basarabeasca"), ("et", "Basarabeasca rajoon"), ("fi", "Basarabeascan piiri"), ("fr", "Raion de Basarabeasca"), ("gu", "બાસારબ\u{ac7}શા જિલ\u{acd}લો"), ("hi", "बसाराबीसाका ज\u{93c}िला"), ("hu", "Basarabeasca járás"), ("id", "Raionul Basarabeasca"), ("it", "distretto di Basarabeasca"), ("ja", "バサラベアスカ県"), ("ka", "ბასარაბეასკის რაიონი"), ("kn", "ಬಸಾರಬೀಸ\u{ccd}ಕಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "바사라베아스카 구"), ("lt", "Basarabiaskos rajonas"), ("lv", "Besarabjaskas rajons"), ("mr", "बसराब\u{947}सा जिल\u{94d}हा"), ("ms", "Basarabeasca District"), ("nb", "Basarabeasca"), ("nl", "Basarabeasca"), ("no", "Basarabeasca"), ("pl", "Rejon Basarabeasca"), ("pt", "Basarabeasca"), ("ro", "raionul Basarabeasca"), ("ru", "Бессарабский район"), ("si", "බසරබ\u{dd2}ස\u{dca}ක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Basarabeasca"), ("sq", "Raionul Basarabeasca"), ("sv", "Basarabeasca (distrikt)"), ("ta", "பசரபேக\u{bcd}க\u{bbe}ஸ\u{bcd}கே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బస\u{c3e}ర\u{c3e}బ\u{c40}స\u{c4d}క\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e39}อ\u{e31}นดา"), ("tr", "Basarabeasca District"), ("uk", "Бессарабський район"), ("ur", "باسارابیاسکا ضلع"), ("vi", "Quận Basarabeasca"), ("zh", "巴薩拉貝亞斯卡區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CA",
                    Subdivision{
                        name: "CA",
                        country_alpha2: Alpha2::MD,
                        code: "CA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8939404), longitude: Some(28.1890275), max_latitude: Some(45.9423021), min_latitude: Some(45.8736666), max_longitude: Some(28.231709), min_longitude: Some(28.1741595)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاهول"), ("bg", "Кахулски район"), ("bn", "ক\u{9be}হোল জেল\u{9be}"), ("ca", "Districte de Cahul"), ("ccp", "𑄇𑄦\u{1112a}𑄣\u{11134}"), ("ceb", "Raionul Cahul"), ("cs", "Okres Cahul"), ("da", "Cahul District"), ("de", "Rajon Cahul"), ("el", "Καχούλ"), ("en", "Cahul"), ("es", "Distrito de Cahul"), ("fi", "Cahulin piiri"), ("fr", "Raion de Cahul"), ("gu", "કાહ\u{ac1}લ જિલ\u{acd}લો"), ("hi", "काह\u{941}ल जिला"), ("hu", "Cahul járás"), ("id", "Raionul Cahul"), ("it", "distretto di Cahul"), ("ja", "カフル県"), ("ka", "კაგულის რაიონი"), ("kn", "ಕಾಹುಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "카훌 구"), ("lt", "Kahulio rajonas"), ("lv", "Kahulas rajons"), ("mr", "काह\u{941}ल जिल\u{94d}हा"), ("ms", "Cahul District"), ("nb", "Cahul"), ("nl", "Cahul"), ("no", "Cahul"), ("pl", "Rejon Kaguł"), ("pt", "Cahul (distrito)"), ("ro", "raionul Cahul"), ("ru", "Кагульский район"), ("si", "ක\u{dcf}හ\u{dd4}ල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Cahul (okres)"), ("sq", "Raionul Cahul"), ("sv", "Cahul rajon"), ("ta", "ச\u{bbe}ஹுல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}హుల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตคาฮ\u{e39}ล"), ("tr", "Cahul District"), ("uk", "Кагульський район"), ("ur", "کاہول ضلع"), ("vi", "Quận Cahul"), ("zh", "卡胡爾區")]),
                        unofficial_name_list: ["Cahul"].to_vec(),
                    }
                ),
                (
                    "CL",
                    Subdivision{
                        name: "CL",
                        country_alpha2: Alpha2::MD,
                        code: "CL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كالاراسي"), ("be", "Каларашскі раён"), ("bg", "Кълърашки район"), ("bn", "ক\u{9be}ল\u{9be}র\u{9be}সি জেল\u{9be}"), ("ca", "Districte de Călărași"), ("ccp", "𑄇𑄣𑄢𑄥\u{11128}"), ("ceb", "Raionul Călărași"), ("da", "Calarasi District"), ("de", "Rajon Călărași"), ("el", "Καλαράσι"), ("en", "Călărași"), ("es", "Distrito de Călărași"), ("fi", "Călărașin piiri"), ("fr", "Raion de Călărași"), ("gl", "Distrito de Călărași"), ("gu", "ક\u{ac7}લારાસી જિલ\u{acd}લો"), ("hi", "कालारासी जिला"), ("hu", "Călăraşi járás"), ("id", "Raionul Călăraşi"), ("it", "distretto di Călărași"), ("ja", "カララシ県 (モルドヴァ)"), ("ka", "კელერაშის რაიონი"), ("kn", "ಕ\u{ccd}ಯಾಲಾರಾಶ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "컬러라시 구"), ("lt", "Kelerašio rajonas"), ("lv", "Kelerasu rajons"), ("mr", "क\u{945}ल\u{945}झी जिल\u{94d}हा"), ("ms", "Calarasi District"), ("nb", "Călărași"), ("nl", "Călărași"), ("no", "Călărași"), ("pl", "Rejon Călăraşi"), ("pt", "Călărași"), ("ro", "raionul Călărași"), ("ru", "Каларашский район"), ("si", "කල\u{dcf}රස\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Călăraşi (okres)"), ("sq", "Raionul Călărași"), ("sv", "Călărași"), ("ta", "கல\u{bbe}ரசி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}ల\u{c3e}ర\u{c3e}స\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "คาลาราช\u{e35}"), ("tr", "Quận Calarasi"), ("uk", "Калараський район"), ("ur", "کالاراشی ضلع"), ("vi", "Quận Calarasi"), ("zh", "卡拉拉什區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CM",
                    Subdivision{
                        name: "CM",
                        country_alpha2: Alpha2::MD,
                        code: "CM",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيميشليا"), ("bg", "Чимишлийски район"), ("bn", "কিমিসিল\u{9be} জেল\u{9be}"), ("ca", "Districte de Cimişlia"), ("ccp", "𑄥\u{11128}𑄟\u{11128}𑄌\u{11134}𑄣\u{11128}𑄠"), ("ceb", "Cimișlia (distrito)"), ("da", "Cimișlia District"), ("de", "Rajon Cimișlia"), ("el", "Κιμισλία"), ("en", "Cimișlia"), ("es", "Distrito de Cimişlia"), ("fi", "Cimișlian kaupunginosa"), ("fr", "Raion de Cimișlia"), ("gu", "સિમિસ\u{acd}લિયા જિલ\u{acd}લો"), ("hi", "किमिस\u{94d}लिया जिला"), ("hu", "Cimişlia járás"), ("id", "Raionul Cimişlia"), ("it", "distretto di Cimişlia"), ("ja", "チミシリア県"), ("ka", "ჩიმიშლიის რაიონი"), ("kn", "ಸ\u{cbf}ಮ\u{cbf}ಸ\u{ccd}ಲ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "치미슐리아 구"), ("lt", "Čimišlijos rajonas"), ("lv", "Čimišlijas rajons"), ("mr", "सिमिस\u{94d}टलिया जिल\u{94d}हा"), ("ms", "Mukim Cimislia"), ("nb", "Cimișlia"), ("nl", "Cimișlia"), ("no", "Cimișlia"), ("pl", "Rejon Cimişlia"), ("pt", "Cimişlia"), ("ro", "raionul Cimișlia"), ("ru", "Чимишлийский район"), ("si", "ස\u{dd2}ම\u{dd2}ස\u{dd2}ල\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Cimişlia"), ("sq", "Raionul Cimișlia"), ("sv", "Cimişlia (distrikt)"), ("ta", "சிமிஸ\u{bcd}ல\u{bc0}ல\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3f}మ\u{c3f}స\u{c4d}ల\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "\u{e34}ซ\u{e34}ม\u{e34}สเล\u{e35}ย แบงค\u{e4c}"), ("tr", "Cimislia District"), ("uk", "Чимішлійський район"), ("ur", "چیمیشلیا ضلع"), ("vi", "Quận Cimislia"), ("zh", "奇米什利亞區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::MD,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كريوليني"), ("bg", "Криуленски район"), ("bn", "ক\u{9cd}রিলেনি জেল\u{9be}"), ("ca", "Districte de Criuleni"), ("ccp", "𑄇\u{11133}𑄢\u{1112d}𑄅\u{1112a}𑄣𑄬𑄚\u{11128}"), ("ceb", "Criuleni"), ("da", "Criuleni District"), ("de", "Rajon Criuleni"), ("el", "Κριουλένι"), ("en", "Criuleni"), ("es", "Distrito de Criuleni"), ("fi", "Criulenin kaupunginosa"), ("fr", "Raion de Criuleni"), ("gu", "ક\u{acd}રિઉલ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "क\u{94d}रिऊल\u{947}नी जिला"), ("id", "Raionul Criuleni"), ("it", "distretto di Criuleni"), ("ja", "クリウレニ県"), ("ka", "კრიულენის რაიონი"), ("kn", "ಕ\u{ccd}ರ\u{cc2}ಲ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "크리울레니 구"), ("lt", "Kriulenio rajonas"), ("lv", "Krjulenu rajons"), ("mr", "शिउल\u{947}न\u{947} जिल\u{94d}हा"), ("ms", "Criuleni District"), ("nb", "Criuleni"), ("nl", "District Criuleni"), ("no", "Criuleni"), ("pl", "Rejon Criuleni"), ("pt", "Criuleni (condado)"), ("ro", "raionul Criuleni"), ("ru", "Криулянский район"), ("si", "ක\u{dca}ර\u{dd2}ය\u{dd4}ලෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Criuleni (okres)"), ("sq", "Raionul Criuleni"), ("sv", "Criuleni (distrikt)"), ("ta", "சிரியலெனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c4d}ర\u{c3f}యుల\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตคร\u{e39}เลน\u{e34}"), ("tr", "Criuelni District"), ("uk", "Кріуленський район"), ("ur", "کریولینی ضلع"), ("vi", "Quận Criuleni"), ("zh", "克留萊尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "CS",
                        country_alpha2: Alpha2::MD,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوسيني"), ("bg", "Каушенски район"), ("bn", "ক\u{9be}সেরি জেল\u{9be}"), ("ca", "Districte de Căuşeni"), ("ccp", "𑄇\u{1112f}𑄥𑄬𑄚\u{11128}"), ("ceb", "Raionul Căușeni"), ("da", "Căușeni District"), ("de", "Rajon Căușeni"), ("el", "Κοσένι"), ("en", "Căușeni"), ("es", "Distrito de Căuşeni"), ("fi", "Căușenin kaupunginosa"), ("fr", "Raion de Căușeni"), ("gu", "કાઉસ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "कौस\u{947}नी जिला"), ("hu", "Căuşeni járás"), ("id", "Raionul Căuşeni"), ("it", "distretto di Căuşeni"), ("ja", "カウシェニ県"), ("ka", "კეუშენის რაიონი"), ("kn", "ಕಯ\u{cc2}ಸ\u{ccd}ಸ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "커우셰니 구"), ("lt", "Keušenio rajonas"), ("lv", "Keušenu rajons"), ("mr", "क\u{941}स\u{947}नी जिल\u{94d}हा"), ("ms", "Causeni District"), ("nb", "Căușeni"), ("nl", "Căușeni"), ("no", "Căușeni"), ("pl", "Rejon Căuşeni"), ("pt", "Căuşeni (condado)"), ("ro", "raionul Căușeni"), ("ru", "Каушанский район"), ("si", "ක\u{dcf}උසේන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Căuşeni (okres)"), ("sq", "Raionul Căușeni"), ("sv", "Raionul Căuşeni"), ("ta", "க\u{bbe}செனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}స\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตค\u{e31}วซ\u{e34}น\u{e35}"), ("tr", "Causeni Distict"), ("uk", "Каушенський район"), ("ur", "کاؤشینی ضلع"), ("vi", "Quận Causeni"), ("zh", "克烏謝尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "CT",
                        country_alpha2: Alpha2::MD,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كانتيمير"), ("bg", "Кантемирски район"), ("bn", "ক\u{9be}ন\u{9cd}তেমির জেল\u{9be}"), ("ca", "Districte de Cantemir"), ("ccp", "𑄇𑄚\u{11134}𑄑𑄬𑄟\u{11128}𑄢\u{11134}"), ("ceb", "Cantemir"), ("cs", "Okres Cantemir"), ("da", "Cantemir District"), ("de", "Rajon Cantemir"), ("el", "Καντεμίρ"), ("en", "Cantemir"), ("es", "Distrito de Cantemir"), ("fi", "Cantemirin kaupunginosa"), ("fr", "raion de Cantemir"), ("gu", "ક\u{ac7}ન\u{acd}ટ\u{ac7}મિર જિલ\u{acd}લો"), ("hi", "क\u{948}न\u{94d}त\u{947}मिर जिला"), ("hu", "Cantemir járás"), ("id", "Raionul Cantemir"), ("it", "distretto di Cantemir"), ("ja", "カンテミール県"), ("ka", "კანტემირის რაიონი"), ("kn", "ಕ\u{ccd}ಯಾಂಟ\u{cc6}ಮ\u{cbf}ರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "칸테미르 구"), ("lt", "Kantemyro rajonas"), ("lv", "Kantemiras rajons"), ("mr", "का\u{902}टिमिर जिल\u{94d}हा"), ("ms", "Cantemir District"), ("nb", "Cantemir"), ("nl", "Cantemir"), ("no", "Cantemir"), ("pl", "Rejon Cantemir"), ("pt", "Cantemir"), ("ro", "raionul Cantemir"), ("ru", "Кантемирский район"), ("si", "කන\u{dca}ටෙම\u{dd3}ර\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Cantemir"), ("sq", "Raionul Cantemir"), ("sv", "Cantemir (distrikt)"), ("ta", "க\u{bbe}ண\u{bcd}டெமிர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}ంట\u{c46}మ\u{c3f}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องเซนท\u{e34}เมอ"), ("tr", "Cantemir District"), ("uk", "Кантемірський район"), ("ur", "کانتیمیر ضلع"), ("vi", "Quận Cantemir"), ("zh", "岡代米爾區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CU",
                    Subdivision{
                        name: "CU",
                        country_alpha2: Alpha2::MD,
                        code: "CU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.0240102), longitude: Some(28.8313156), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Chişinău"), ("am", "ኪሺንው"), ("ar", "كيشيناو"), ("az", "Kişinyov"), ("be", "Кішынёў"), ("bg", "Кишинев"), ("bn", "চিসিন\u{9be}উ"), ("bs", "Kišinjev"), ("ca", "Chișinău"), ("ccp", "𑄇\u{11128}𑄥\u{11128}𑄚\u{1112f}"), ("ceb", "Municipiul Chișinău"), ("cs", "Kišiněv"), ("cy", "Chişinău"), ("da", "Chișinău"), ("de", "Chișinău"), ("el", "Κισινάου"), ("en", "Chișinău"), ("es", "Chisináu"), ("et", "Chișinău"), ("eu", "Chisinau"), ("fa", "کیشیناو"), ("fi", "Chișinău"), ("fr", "Municipalité de Chișinău"), ("ga", "Císineá"), ("gl", "Chișinău"), ("gu", "કીશિન\u{ac7}વ"), ("he", "קישינב"), ("hi", "चिशिनाउ"), ("hr", "Kišinjev"), ("hu", "Chișinău"), ("hy", "Քիշնև"), ("id", "Kishinev"), ("is", "Kisínev"), ("it", "Chișinău"), ("ja", "キシナウ"), ("jv", "Chişinău"), ("ka", "კიშინიოვი"), ("kk", "Кишинев"), ("kn", "ಚ\u{cbf}ಸ\u{cbf}ನಾವ\u{ccd}"), ("ko", "키시너우"), ("ky", "Кишинёв"), ("lt", "Kišiniovas"), ("lv", "Kišiņeva"), ("mk", "Кишињев"), ("mn", "Кишинёв"), ("mr", "चिशिनाउ"), ("ms", "Chişinău"), ("nb", "Chișinău"), ("nl", "Chisinau"), ("no", "Chișinău"), ("pa", "ਕਿਸ\u{a3c}ਨਾਓ"), ("pl", "Kiszyniów"), ("pt", "Chișinău"), ("ro", "Municipiul Chișinău"), ("ru", "Кишинёв"), ("si", "ච\u{dd2}ස\u{dd2}න\u{dcf}ව\u{dd4}"), ("sk", "Kišiňov"), ("sl", "Kišinjev"), ("sq", "Municipiul Chișinău"), ("sr", "Кишињев"), ("sr_Latn", "Kišinjev"), ("sv", "Chișinău"), ("sw", "Kishineu"), ("ta", "சிஷினோ"), ("te", "చ\u{c3f}ష\u{c3f}న\u{c4b}"), ("th", "ค\u{e35}ช\u{e35}เนา"), ("tk", "Kişinew"), ("tr", "Kişinev"), ("uk", "Кишинів"), ("ur", "کیشیناو"), ("uz", "Kishinyov"), ("vi", "Chişinău"), ("yo", "Chişinău"), ("yo_BJ", "Chişinău"), ("yue", "基希涅夫"), ("yue_Hans", "基希涅夫"), ("zh", "基希讷乌")]),
                        unofficial_name_list: ["Chisinau"].to_vec(),
                    }
                ),
                (
                    "DO",
                    Subdivision{
                        name: "DO",
                        country_alpha2: Alpha2::MD,
                        code: "DO",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوندوسيني"), ("bg", "Дондушенски район"), ("bn", "ডনডোসেনি জেল\u{9be}"), ("ca", "Districte de Dondușeni"), ("ccp", "𑄓\u{11127}𑄚\u{11134}𑄓\u{1112a}𑄥𑄬𑄚\u{11128}"), ("ceb", "Dondușeni"), ("da", "Dondușeni District"), ("de", "Rajon Dondușeni"), ("el", "Ντοντουσένι"), ("en", "Dondușeni"), ("es", "Distrito de Dondușeni"), ("fi", "Dondușenin kaupunginosa"), ("fr", "Raion de Dondușeni"), ("gu", "ડોન\u{acd}ડ\u{acd}ય\u{ac1}સ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "दो\u{902}द\u{941}स\u{947}नी जिला"), ("id", "Raionul Donduşeni"), ("it", "distretto di Donduşeni"), ("ja", "ドンドゥセニ県"), ("ka", "დონდუშენის რაიონი"), ("kn", "ಡೋಂಡುಸ\u{ccd}ಸ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "돈두셰니 구"), ("lt", "Dondušenio rajonas"), ("lv", "Dondušenu rajons"), ("mr", "दो\u{902}डिस\u{947}नी जिल\u{94d}हा"), ("ms", "Donduseni District"), ("nb", "Dondușeni"), ("nl", "Dondușeni"), ("no", "Dondușeni"), ("pl", "Rejon Donduşeni"), ("pt", "Donduşeni"), ("ro", "raionul Dondușeni"), ("ru", "Дондюшанский район"), ("si", "ඩොන\u{dca}ඩ\u{dd4}සෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Donduşeni"), ("sq", "Raionul Dondușeni"), ("sv", "Donduşeni (distrikt)"), ("ta", "டோண\u{bcd}டுஸ\u{bcd}இனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "డ\u{c4b}ండూస\u{c47}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตดอนด\u{e39}เซน\u{e34}"), ("tr", "Donduseni District"), ("uk", "Дондушенський район"), ("ur", "دوندوشینی ضلع"), ("vi", "Quận Donduseni"), ("zh", "棟杜謝尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DR",
                    Subdivision{
                        name: "DR",
                        country_alpha2: Alpha2::MD,
                        code: "DR",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دروتشيا"), ("be", "Дрокіеўскі раён"), ("bg", "Дрокиевски район"), ("bn", "ড\u{9cd}রোক\u{9be}ইয\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Districte de Drochia"), ("ccp", "𑄓\u{11133}𑄢\u{1112e}𑄌\u{11128}𑄠"), ("ceb", "Raionul Drochia"), ("da", "Drochia District"), ("de", "Rajon Drochia"), ("el", "Ντρότσια"), ("en", "Drochia"), ("es", "Distrito de Drochia"), ("fi", "Drochian piiri"), ("fr", "Raion de Drochia"), ("gu", "ડ\u{acd}રોચિયા જિલ\u{acd}લો"), ("hi", "द\u{94d}रोचिया जिला"), ("id", "Raionul Drochia"), ("it", "distretto di Drochia"), ("ja", "ドロキア県"), ("ka", "დროკიის რაიონი"), ("kn", "ಡ\u{ccd}ರೋಶ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "드로키아 구"), ("lt", "Drokijos rajonas"), ("lv", "Drokijas rajons"), ("mr", "ड\u{94d}रॉचीया जिल\u{94d}हा"), ("ms", "Drochia District"), ("nb", "Drochia"), ("nl", "Drochia"), ("no", "Drochia"), ("pl", "Rejon Drochia"), ("pt", "Drochia"), ("ro", "raionul Drochia"), ("ru", "Дрокиевский район"), ("si", "ඩ\u{dca}රෝච\u{dd2}ය\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Drochia"), ("sq", "Raionul Drochia"), ("sv", "Drochia rajon"), ("ta", "ட\u{bcd}ர\u{bbe}சிய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "డ\u{c4d}ర\u{c4b}చ\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโดรเช\u{e35}ย"), ("tr", "Drochia District"), ("uk", "Дрокійський район"), ("ur", "دروکیا ضلع"), ("vi", "Quận Drochia"), ("zh", "德羅基亞區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "DU",
                    Subdivision{
                        name: "DU",
                        country_alpha2: Alpha2::MD,
                        code: "DU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوباساري"), ("bg", "Дубосарски район"), ("bn", "ডোব\u{9be}স\u{9be}রি জেল\u{9be}"), ("ca", "Districte de Dubăsari"), ("ccp", "𑄓\u{1112a}𑄝𑄥𑄢\u{11128}"), ("ceb", "Raionul Dubăsari"), ("cs", "Okres Dubăsari"), ("da", "Dubăsari District"), ("de", "Rajon Dubăsari"), ("el", "Ντουμπασάρι"), ("en", "Dubăsari"), ("es", "Distrito de Dubăsari"), ("fi", "Dubăsarin kaupunginosa"), ("fr", "Raion de Dubăsari"), ("gu", "દ\u{ac1}બાસારી જિલ\u{acd}લો"), ("hi", "द\u{941}ब\u{947}सरी जिला"), ("id", "Raionul Dubăsari"), ("it", "distretto di Dubăsari"), ("ja", "ドゥベサリ県"), ("ka", "დუბესარის რაიონი"), ("kn", "ಡುಬಸರ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "두버사리 구"), ("lt", "Dubosarų rajonas"), ("lv", "Dubesaru rajons"), ("mr", "द\u{941}ब\u{947}सीरी जिल\u{94d}हा"), ("ms", "Dubasari District"), ("nb", "Dubășari"), ("nl", "Dubăsari"), ("no", "Dubășari"), ("pl", "Rejon Dubosary"), ("pt", "Dubăsari"), ("ro", "raionul Dubăsari"), ("ru", "Дубоссарский район"), ("si", "ඩ\u{dd4}බ\u{dcf}සර\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Dubăsari"), ("sq", "Raionul Dubăsari"), ("sv", "Raionul Dubăsari"), ("ta", "டூபசரி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "డుబ\u{c3e}స\u{c3e}ర\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ด\u{e39}บาซาร\u{e34}"), ("tr", "Dubasari District"), ("uk", "Дубесарський район"), ("ur", "دوباساری ضلع"), ("vi", "Quận Dubasari"), ("zh", "杜伯薩里區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "ED",
                    Subdivision{
                        name: "ED",
                        country_alpha2: Alpha2::MD,
                        code: "ED",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.1678991), longitude: Some(27.2936143), max_latitude: Some(48.1892079), min_latitude: Some(48.1522871), max_longitude: Some(27.3280621), min_longitude: Some(27.2787093)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إيدينت"), ("bg", "Единецки район"), ("bn", "এডিনেত জেল\u{9be}"), ("ca", "Districte d’Edineț"), ("ccp", "𑄃𑄬𑄓\u{11128}𑄚𑄬𑄖\u{11134}"), ("ceb", "Raionul Edineț"), ("da", "Edineț District"), ("de", "Rajon Edineț"), ("el", "Εντινέτ"), ("en", "Edineț"), ("es", "Distrito de Edineț"), ("fi", "Edinețin kaupunginosa"), ("fr", "Raion d’Edineț"), ("gu", "એડિન\u{ac7}ટ જિલ\u{acd}લો"), ("hi", "एडिन\u{947}ट जिला"), ("id", "Raionul Edineţ"), ("it", "distretto di Edineț"), ("ja", "エディネツ県"), ("ka", "ედინეცის რაიონი"), ("kn", "ಎಡ\u{cbf}ನ\u{cc6}ಟ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "에디네츠 구"), ("lt", "Jedineco rajonas"), ("lv", "Jedinecas rajons"), ("mr", "एडिनट जिल\u{94d}हा"), ("ms", "Edinet District"), ("nb", "Edineț"), ("nl", "Edineț"), ("no", "Edineț"), ("pl", "Rejon Jedyńce"), ("pt", "Edineţ"), ("ro", "raionul Edineț"), ("ru", "Единецкий район"), ("si", "එද\u{dd2}නෙට\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Edineţ"), ("sq", "Raionul Edineț"), ("sv", "Raionul Edineţ"), ("ta", "எடின\u{bcd}ட\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఎడ\u{c3f}న\u{c46}ట\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e35}ด\u{e34}เนท"), ("tr", "Edinet District"), ("uk", "Єдинецький район"), ("ur", "ایدینیتس ضلع"), ("vi", "Quận Edinet"), ("zh", "埃迪內茨區")]),
                        unofficial_name_list: ["Edinet"].to_vec(),
                    }
                ),
                (
                    "FA",
                    Subdivision{
                        name: "FA",
                        country_alpha2: Alpha2::MD,
                        code: "FA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فاليستي"), ("bg", "Фалещки район"), ("bn", "ফ\u{9be}লেস\u{9cd}তি জেল\u{9be}"), ("ca", "Districte de Faleşti"), ("ccp", "𑄜𑄣𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Fălești"), ("cs", "Okres Fălești"), ("da", "Fălești District"), ("de", "Rajon Fălești"), ("el", "Φαλέστι"), ("en", "Fălești"), ("es", "Distrito de Fălești"), ("fi", "Făleștin kaupunginosa"), ("fr", "Raion de Falești"), ("gu", "ફલ\u{ac7}સ\u{acd}ટી જિલ\u{acd}લો"), ("hi", "फ\u{948}ल\u{947}स\u{94d}टी जिला"), ("id", "Raionul Făleşti"), ("it", "distretto di Făleşti"), ("ja", "ファレシュティ県"), ("ka", "ფელეშტის რაიონი"), ("kn", "ಫಾಲ\u{cc6}ಸ\u{ccd}ಟ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "펄레슈티 구"), ("lt", "Feleščio rajonas"), ("lv", "Feleštu rajons"), ("mr", "फल\u{947}स\u{94d}टी जिल\u{94d}हा"), ("ms", "Falesti District"), ("nb", "Fălești"), ("nl", "Fălești"), ("no", "Fălești"), ("pl", "Rejon Făleşti"), ("pt", "Făleşti"), ("ro", "raionul Fălești"), ("ru", "Фалештский район"), ("si", "ෆ\u{dcf}ලෙස\u{dca}ට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Făleşti"), ("sq", "Raionul Falești"), ("sv", "Făleşti (distrikt)"), ("ta", "ப\u{bbe}ல\u{bcd}ஸ\u{bcd}த\u{bc0} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c3e}ల\u{c46}స\u{c4d}ట\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องฟาเรสต\u{e35}"), ("tr", "Falesti District"), ("uk", "Фалештський район"), ("ur", "فالیشتی ضلع"), ("vi", "Quận Falesti"), ("zh", "弗萊什蒂區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "FL",
                    Subdivision{
                        name: "FL",
                        country_alpha2: Alpha2::MD,
                        code: "FL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فلوريستي"), ("be", "Фларэшцкі раён"), ("bg", "Флорещки район"), ("bn", "ফ\u{9cd}লরেস\u{9cd}তি জেল\u{9be}"), ("ca", "Districte de Floreşti"), ("ccp", "𑄜\u{11133}𑄣\u{1112e}𑄢𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Florești (distrito)"), ("cs", "Okres Florești"), ("da", "Florești"), ("de", "Rajon Florești"), ("el", "Φλορέστι"), ("en", "Florești"), ("es", "Distrito de Florești"), ("fi", "Floreștin kaupunginosa"), ("fr", "Raion de Florești"), ("gu", "ફ\u{acd}લોર\u{ac7}સ\u{acd}ટી જિલ\u{acd}લો"), ("hi", "फ\u{94d}लोर\u{947}स\u{94d}टी जिला"), ("id", "Raionul Floreşti"), ("it", "distretto di Florești"), ("ja", "フロレシュティ県"), ("ka", "ფლორეშტის რაიონი"), ("kn", "ಫ\u{ccd}ಲೋರ\u{cc6}ಸ\u{ccd}ಟ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "플로레슈티 구"), ("lt", "Floreščio rajonas"), ("lv", "Floreštu rajons"), ("mr", "फ\u{94d}लॉर\u{947}स\u{94d}टी जिल\u{94d}हा"), ("ms", "Floresti District"), ("nb", "Florești"), ("nl", "Florești"), ("no", "Florești"), ("pl", "Rejon Floreşti"), ("pt", "Floreşti"), ("ro", "raionul Florești"), ("ru", "Флорештский район"), ("si", "ෆ\u{dca}ලොරෙස\u{dca}ට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Floreşti"), ("sq", "Raionul Florești"), ("sv", "Floreşti (distrikt)"), ("ta", "பில\u{bbe}ஸ\u{bcd}டி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c4d}ల\u{c4b}ర\u{c46}స\u{c4d}ట\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตฟลอเรสต\u{e34}"), ("tr", "Floresti District"), ("uk", "Флорештський район"), ("ur", "فلوریشتی ضلع"), ("vi", "Quận Floresti"), ("zh", "弗洛雷什蒂區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "GA",
                    Subdivision{
                        name: "GA",
                        country_alpha2: Alpha2::MD,
                        code: "GA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousTerritorialUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Gagausië"), ("ar", "غاغاوزيا"), ("az", "Qaqauziya"), ("be", "Гагаузія"), ("bg", "Гагаузия"), ("bn", "গ\u{9be}গ\u{9c1}জিয\u{9bc}\u{9be}"), ("ca", "Gagaúsia"), ("ccp", "𑄉𑄉𑄅\u{1112a}𑄎\u{11128}𑄠"), ("ceb", "Găgăuzia"), ("cs", "Gagauzsko"), ("cy", "Gagauzia"), ("da", "Gagauzien"), ("de", "Gagausien"), ("el", "Γκαγκαουζία"), ("en", "Gagauzia"), ("es", "Gagauzia"), ("et", "Gagauusia"), ("eu", "Gagauzia"), ("fa", "گاگائوزیا"), ("fi", "Gagauzia"), ("fr", "Gagaouzie"), ("gl", "Gagauzia"), ("gu", "ગાગાઉઝિયા"), ("he", "גגאוזיה"), ("hi", "गगाउज\u{93c}िया"), ("hr", "Gagauzija"), ("hu", "Gagauzia"), ("id", "Gagauzia"), ("is", "Gagásía"), ("it", "Gagauzia"), ("ja", "ガガウズ自治区"), ("ka", "გაგაუზია"), ("kk", "Гәгәузия"), ("kn", "ಗಾಗುಜ\u{cbf}ಯ"), ("ko", "가가우지아"), ("ky", "Гагаузия"), ("lt", "Gagaūzija"), ("lv", "Gagauzija"), ("mk", "Гагаузија"), ("mn", "Гагауз нутаг"), ("mr", "ग\u{945}गाउझिया"), ("ms", "Gagauzia"), ("nb", "Gagaus"), ("nl", "Gagaoezië"), ("no", "Gagaus"), ("pl", "Gagauzja"), ("pt", "Gagaúzia"), ("ro", "Găgăuzia"), ("ru", "Гагаузия"), ("si", "ග\u{dcf}ගෝස\u{dd2}ය\u{dcf}"), ("sk", "Gagauzsko"), ("sq", "Gagauzia"), ("sr", "Гагаузија"), ("sr_Latn", "Gagauzija"), ("sv", "Gagauzien"), ("ta", "க\u{bbe}கவுசிங\u{bcd}"), ("te", "గగ\u{c3e}వూజ\u{c3f}య\u{c3e}"), ("th", "กาเกาเซ\u{e35}ย"), ("tk", "Gagauzystan"), ("tr", "Gagavuzya"), ("uk", "Гагаузія"), ("ur", "گاگاؤزیا"), ("vi", "Gagauzia"), ("zh", "加告茲自治區")]),
                        unofficial_name_list: ["Gagauzia, Unitate Teritoriala Autonoma (UTAG)"].to_vec(),
                    }
                ),
                (
                    "GL",
                    Subdivision{
                        name: "GL",
                        country_alpha2: Alpha2::MD,
                        code: "GL",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة غلوديني"), ("bg", "Голденски район"), ("bn", "গ\u{9cd}ল\u{9c1}ডেনি জেল\u{9be}"), ("ca", "Districte de Glodeni"), ("ccp", "𑄉\u{11133}𑄣\u{1112e}𑄓𑄬𑄚\u{11128}"), ("ceb", "Glodeni (distrito)"), ("cs", "Okres Glodeni"), ("da", "Glodeni District"), ("de", "Rajon Glodeni"), ("el", "Γκλοντένι"), ("en", "Glodeni"), ("es", "Distrito de Glodeni"), ("fi", "Glodenin piiri"), ("fr", "Raion de Glodeni"), ("gu", "ગ\u{acd}લોડ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "ग\u{94d}लोड\u{947}नी जिला"), ("id", "Raionul Glodeni"), ("it", "distretto di Glodeni"), ("ja", "グロデニ県"), ("ka", "გლოდენის რაიონი"), ("kn", "ಗ\u{ccd}ಲೋಡ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "글로데니 구"), ("lt", "Glodenio rajonas"), ("lv", "Glodenu rajons"), ("mr", "ग\u{94d}लोड\u{947}नी जिल\u{94d}हा"), ("ms", "Glodeni District"), ("nb", "Glodeni"), ("nl", "Glodeni"), ("no", "Glodeni"), ("pl", "Rejon Glodeni"), ("pt", "Glodeni"), ("ro", "raionul Glodeni"), ("ru", "Глодянский район"), ("si", "ග\u{dca}ලොඩෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Glodeni"), ("sq", "Raionul Glodeni"), ("sv", "Glodeni (distrikt)"), ("ta", "கிலோடெனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "గ\u{c4d}ల\u{c4b}డ\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโกลเดน\u{e35}"), ("tr", "Glodeni District"), ("uk", "Глоденський район"), ("ur", "گلودینی ضلع"), ("vi", "Quận Glodeni"), ("zh", "格洛代尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "HI",
                    Subdivision{
                        name: "HI",
                        country_alpha2: Alpha2::MD,
                        code: "HI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هينسيستي"), ("bg", "Хънчещки район"), ("bn", "হিনকেস\u{9cd}তি জেল\u{9be}"), ("ca", "Districte de Hînceşti"), ("ccp", "𑄦\u{11128}𑄚\u{11134}𑄥𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Hîncești (distrito)"), ("da", "Hîncești District"), ("de", "Rajon Hîncești"), ("el", "Χινκέστι"), ("en", "Hîncești"), ("es", "Distrito de Hînceşti"), ("fi", "Hînceștin kaupunginosa"), ("fr", "Raion de Hîncești"), ("gu", "હિન\u{acd}સ\u{ac7}સ\u{acd}ટી જિલ\u{acd}લો"), ("hi", "हि\u{902}स\u{947}स\u{94d}टी जिला"), ("hy", "Հնչեշտի շրջան"), ("id", "Raionul Hînceşti"), ("it", "distretto di Hînceşti"), ("ja", "ヒンチェシュティ県"), ("ka", "ჰინჩეშტის რაიონი"), ("kn", "ಹ\u{cbf}ನ\u{ccd}ಸ\u{ccd}ಟ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "흔체슈티 구"), ("lt", "Hinčeščio rajonas"), ("lv", "Hinčeštu rajons"), ("mr", "ह\u{947}\u{902}स\u{94d}ट\u{947}टी जिल\u{94d}हा"), ("ms", "Hincesti District"), ("nb", "Hîncești"), ("nl", "Hîncești"), ("no", "Hîncești"), ("pl", "Rejon Hînceşti"), ("pt", "Hînceşti"), ("ro", "raionul Hîncești"), ("ru", "Хынчештский район"), ("si", "හ\u{dd2}න\u{dca}සේස\u{dca}ට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Hînceşti"), ("sq", "Raionul Hîncești"), ("sv", "Hînceşti (distrikt)"), ("ta", "இன\u{bcd}செஸ\u{bcd}ட\u{bcd}டி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "హ\u{c3f}న\u{c4d}స\u{c46}స\u{c4d}ట\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตฮ\u{e34}นเชสต\u{e34}"), ("tr", "Hinceşti District"), ("uk", "Гинчештський район"), ("ur", "ہینچیشتی ضلع"), ("vi", "Quận Hincesti"), ("zh", "亨切什蒂區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "IA",
                    Subdivision{
                        name: "IA",
                        country_alpha2: Alpha2::MD,
                        code: "IA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إيلوفيني"), ("bg", "Яловенски район"), ("bn", "ল\u{9be}লোভেনি জেল\u{9be}"), ("ca", "Districte de Ialoveni"), ("ccp", "𑄃\u{1112d}𑄃𑄣\u{1112e}𑄞𑄬𑄚\u{11128}"), ("ceb", "Ialoveni"), ("cs", "Okres Ialoveni"), ("da", "Ialoveni District"), ("de", "Rajon Ialoveni"), ("el", "Ιαλοβένι"), ("en", "Ialoveni"), ("es", "Distrito de Ialoveni"), ("fi", "Ialovenin hallintopiiri"), ("fr", "Raion de Ialoveni"), ("gu", "ઈઅલોવ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "लालोव\u{947}नी जिला"), ("id", "Raionul Ialoveni"), ("it", "distretto di Ialoveni"), ("ja", "ヤロベニ県"), ("ka", "იალოვენის რაიონი"), ("kn", "ಐಲೋವ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "이알로베니 구"), ("lt", "Jalovenio rajonas"), ("lv", "Jalovenu rajons"), ("mr", "इलाव\u{947}नी जिल\u{94d}हा"), ("ms", "Ialoveni District"), ("nb", "Ialoveni"), ("nl", "Ialoveni"), ("no", "Ialoveni"), ("pl", "Rejon Ialoveni"), ("pt", "Ialoveni (distrito)"), ("ro", "raionul Ialoveni"), ("ru", "Яловенский район"), ("si", "ලලොවෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Ialoveni (okres)"), ("sq", "Raionul Ialoveni"), ("sv", "Ialoveni"), ("ta", "இஅலோவேனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3e}ల\u{c4b}వ\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตลาเล\u{e34}ฟน\u{e34}"), ("tr", "ialoveni District"), ("uk", "Яловенський район"), ("ur", "یالووہنی ضلع"), ("vi", "Quận Ialoveni"), ("zh", "亞洛韋尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "LE",
                    Subdivision{
                        name: "LE",
                        country_alpha2: Alpha2::MD,
                        code: "LE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليوفا"), ("bg", "Леовски район"), ("bn", "লিওভ\u{9be} জেল\u{9be}"), ("ca", "Districte de Leova"), ("ccp", "𑄣\u{11128}𑄃\u{1112e}𑄞"), ("ceb", "Leova"), ("cs", "Okres Leova"), ("da", "Leova District"), ("de", "Rajon Leova"), ("el", "Λεόβα"), ("en", "Leova"), ("es", "Distrito de Leova"), ("fi", "Leovan kaupunginosa"), ("fr", "Raion de Leova"), ("gu", "લિઓવા જિલ\u{acd}લો"), ("hi", "लिओवा जिला"), ("id", "Raionul Leova"), ("it", "distretto di Leova"), ("ja", "レオバ県"), ("ka", "ლეოვის რაიონი"), ("kn", "ಲ\u{cc6}ವಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "레오바 구"), ("lt", "Leovos rajonas"), ("lv", "Leovas rajons"), ("mr", "लिओवा जिल\u{94d}हा"), ("ms", "Leova District"), ("nb", "Leova"), ("nl", "Leova"), ("no", "Leova"), ("pl", "Rejon Leova"), ("pt", "Leova"), ("ro", "raionul Leova"), ("ru", "Леовский район"), ("si", "ල\u{dd2}යෝව\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Leova"), ("sq", "Raionul Leova"), ("sv", "Leova (distrikt)"), ("ta", "லேச\u{bbe}வ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3f}య\u{c4b}వ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตล\u{e35}โอวา"), ("tr", "Leova District"), ("uk", "Леовський район"), ("ur", "لیووا ضلع"), ("vi", "Quận Leova"), ("zh", "萊奧瓦區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::MD,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نيسبوريني"), ("bg", "Ниспоренски район"), ("bn", "নিস\u{9cd}পরেনি জেল\u{9be}"), ("ca", "Districte de Nisporeni"), ("ccp", "𑄚\u{11128}𑄌\u{11134}𑄛\u{11127}𑄢𑄬𑄚\u{11128}"), ("ceb", "Nisporeni"), ("da", "Nisporeni District"), ("de", "Rajon Nisporeni"), ("el", "Νισπορένι"), ("en", "Nisporeni"), ("es", "Distrito de Nisporeni"), ("fi", "Nisporenin kaupunginosa"), ("fr", "Raion de Nisporeni"), ("gu", "નિસ\u{acd}પોર\u{ac7}ની જિલ\u{acd}લો"), ("hi", "निस\u{94d}पोरनी जिला"), ("id", "Raionul Nisporeni"), ("it", "distretto di Nisporeni"), ("ja", "ニスポレニ県"), ("ka", "ნისპორენის რაიონი"), ("kn", "ನ\u{cbf}ಸ\u{ccd}ಪೊರ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "니스포레니 구"), ("lt", "Nisporenio rajonas"), ("lv", "Nisperonu rajons"), ("mr", "निस\u{94d}पोरनी जिल\u{94d}हा"), ("ms", "Nisporeni District"), ("nb", "Nisporeni"), ("nl", "Nisporeni"), ("no", "Nisporeni"), ("pl", "Rejon Nisporeni"), ("pt", "Nisporeni"), ("ro", "raionul Nisporeni"), ("ru", "Ниспоренский район"), ("si", "න\u{dd2}ස\u{dca}පොරෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Nisporeni"), ("sq", "Raionul Nisporeni"), ("sv", "Nisporeni (distrikt)"), ("ta", "நிஸ\u{bcd}ப\u{bcd}பூனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "న\u{c3f}స\u{c4d}ప\u{c4b}ర\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตน\u{e34}สโปเรน\u{e35}"), ("tr", "Nisporeni District"), ("uk", "Ніспоренський район"), ("ur", "نیسپورینی ضلع"), ("vi", "Quận Nisporeni"), ("zh", "尼斯波雷尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "OC",
                    Subdivision{
                        name: "OC",
                        country_alpha2: Alpha2::MD,
                        code: "OC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أونيتا"), ("bg", "Окницки район"), ("bn", "ওকনিত\u{9be} জেল\u{9be}"), ("ca", "Districte d’Ocniţa"), ("ccp", "𑄃\u{11127}𑄇\u{11134}𑄚\u{11128}𑄑"), ("ceb", "Raionul Ocnița"), ("cs", "Okres Ocniţa"), ("da", "Ocnița District"), ("de", "Rajon Ocnița"), ("el", "Οκνίτα"), ("en", "Ocniţa"), ("es", "Distrito de Ocniţa"), ("fi", "Ocnițan piiri"), ("fr", "Raion d’Ocnița"), ("gu", "ઓસ\u{acd}નીટા જિલ\u{acd}લો"), ("hi", "ऑकनिटा जिला"), ("id", "Raionul Ocniţa"), ("it", "distretto di Ocniţa"), ("ja", "オクニタ県"), ("ka", "ოკნიცის რაიონი"), ("kn", "ಓಕ\u{ccd}ನ\u{cbf}ಟಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "오크니차 구"), ("lt", "Oknicos rajonas"), ("lv", "Oknicas rajons"), ("mr", "ओकिन\u{94d}टा जिल\u{94d}हा"), ("ms", "Ocnita District"), ("nb", "Ocnița"), ("nl", "Ocnița"), ("no", "Ocnița"), ("pl", "Rejon Ocniţa"), ("pt", "Ocniţa"), ("ro", "raionul Ocnița"), ("ru", "Окницкий район"), ("si", "ඔක\u{dca}න\u{dd2}ට\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Ocniţa"), ("sq", "Raionul Ocnița"), ("sv", "Raionul Ocniţa"), ("ta", "ஓக\u{bcd}ந\u{bc0}த\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఒస\u{c3f}న\u{c3f}త జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e47}อกน\u{e35}ตซา"), ("tr", "Ornita District"), ("uk", "Окницький район"), ("ur", "اوکنیتسا ضلع"), ("vi", "Quận Ocnita"), ("zh", "奧克尼察區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "OR",
                    Subdivision{
                        name: "OR",
                        country_alpha2: Alpha2::MD,
                        code: "OR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.383333), longitude: Some(28.816667), max_latitude: Some(47.40391169999999), min_latitude: Some(47.333195), max_longitude: Some(28.8649678), min_longitude: Some(28.7943291)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أورهي"), ("bg", "Орхеевски район"), ("bn", "ওরহেয\u{9bc} জেল\u{9be}"), ("ca", "Districte d’Orhei"), ("ccp", "𑄃\u{11127}𑄢\u{11134}𑄦𑄬\u{1112d}"), ("ceb", "Orhei"), ("cs", "Okres Orhei"), ("da", "Orhei District"), ("de", "Rajon Orhei"), ("el", "Ορχέι"), ("en", "Orhei"), ("es", "Distrito de Orhei"), ("fi", "Orhein kaupunginosa"), ("fr", "Raion d’Orhei"), ("gu", "ઓરહ\u{ac7}ઇ જિલ\u{acd}લો"), ("hi", "औरह\u{947}ई ज\u{93c}िला"), ("id", "Raionul Orhei"), ("it", "distretto di Orhei"), ("ja", "オルゲイ県"), ("ka", "ორჰეის რაიონი"), ("kn", "ಒರೈ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "오르헤이 구"), ("lt", "Orhėjaus rajonas"), ("lv", "Orhejas rajons"), ("mr", "ओरिई जिल\u{94d}हा"), ("ms", "Orhei District"), ("nb", "Orhei"), ("nl", "Orhei"), ("no", "Orhei"), ("pl", "Rejon Orgiejów"), ("pt", "Orhei"), ("ro", "raionul Orhei"), ("ru", "Оргеевский район"), ("si", "ඔර\u{dca}හෙය\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Orhei"), ("sq", "Raionul Orhei"), ("sv", "Orhei rajon"), ("ta", "போர\u{bcd}ஹெய\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఓర\u{c4d}హ\u{c46} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ร\u{e31}ฐบาร\u{e35}น\u{e31}ส"), ("tr", "Orhei District"), ("uk", "Оргіївський район"), ("ur", "اورہئی ضلع"), ("vi", "Quận Orhei"), ("zh", "奧爾海伊區")]),
                        unofficial_name_list: ["Orhei"].to_vec(),
                    }
                ),
                (
                    "RE",
                    Subdivision{
                        name: "RE",
                        country_alpha2: Alpha2::MD,
                        code: "RE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ريزاينا"), ("bg", "Резински район"), ("bn", "রেজিন\u{9be} জেল\u{9be}"), ("ca", "Districte de Rezina"), ("ccp", "𑄢𑄬𑄎𑄚"), ("ceb", "Rezina"), ("da", "Rezina District"), ("de", "Rajon Rezina"), ("el", "Ρεζίνα"), ("en", "Rezina"), ("es", "Distrito de Rezina"), ("fi", "Rezinan kaupunginosa"), ("fr", "Raion de Rezina"), ("gu", "ર\u{ac7}ઝીના જિલ\u{acd}લો"), ("hi", "र\u{947}जिना जिला"), ("id", "Raionul Rezina"), ("it", "distretto di Rezina"), ("ja", "レジナ県"), ("ka", "რეზინის რაიონი"), ("kn", "ರ\u{cc6}ಝ\u{cbf}ನಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "레지나 구"), ("lt", "Rezinos rajonas"), ("lv", "Rezinas rajons"), ("mr", "र\u{947}जिना जिल\u{94d}हा"), ("ms", "Rezina District"), ("nb", "Rezina"), ("nl", "Rezina"), ("no", "Rezina"), ("pl", "Rejon Rezina"), ("pt", "Rezina"), ("ro", "raionul Rezina"), ("ru", "Резинский район"), ("si", "රෙස\u{dd2}න\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Rezina"), ("sq", "Raionul Rezina"), ("sv", "Rezina (distrikt)"), ("ta", "ரெஜின\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ర\u{c46}జ\u{c40}న\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเรซ\u{e35}นา"), ("tr", "Rezina District"), ("uk", "Резинський район"), ("ur", "ریزینا ضلع"), ("vi", "Quận Rezina"), ("zh", "雷濟納區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "RI",
                    Subdivision{
                        name: "RI",
                        country_alpha2: Alpha2::MD,
                        code: "RI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة روشكاني"), ("bg", "Ръшкански район"), ("bn", "রিসক\u{9be}নি জেল\u{9be}"), ("ca", "Districte de Rîşcani"), ("ccp", "𑄢\u{11128}𑄌\u{11134}𑄇𑄚\u{11128}"), ("ceb", "Rîșcani"), ("da", "Rîșcani District"), ("de", "Rajon Rîșcani"), ("el", "Ρισκάνι"), ("en", "Rîșcani"), ("es", "Distrito de Rîșcani"), ("fi", "Rîșcanin kaupunginosa"), ("fr", "Raion de Rîșcani"), ("gl", "Distrito de Rîșcani"), ("gu", "રિસની જિલ\u{acd}લો"), ("hi", "रिस\u{94d}कानी जिला"), ("id", "Raionul Rîşcani"), ("it", "distretto di Rîşcani"), ("ja", "リスカニ県"), ("ka", "რიშკანის რაიონი"), ("kn", "ರ\u{cbf}ಸ\u{ccd}ಕನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "르슈카니 구"), ("lt", "Raškanio rajonas"), ("lv", "Riškanu rajons"), ("mr", "रिसकानी जिल\u{94d}हा"), ("ms", "Riscani District"), ("nb", "Rîșcani"), ("nl", "Rîșcani"), ("no", "Rîșcani"), ("pl", "Rejon Rîşcani"), ("pt", "Rîşcani"), ("ro", "raionul Rîșcani"), ("ru", "Рышканский район"), ("si", "ර\u{dd2}ස\u{dca}ක\u{dcf}න\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Rîşcani"), ("sq", "Raionul Rîșcani"), ("sv", "Rîşcani (distrikt)"), ("ta", "ரிஸ\u{bcd}கேணி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ర\u{c3f}స\u{c4d}క\u{c3e}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตร\u{e34}ชคาน\u{e35}"), ("tr", "Riscani District"), ("uk", "Ришканський район"), ("ur", "ریشکانی ضلع"), ("vi", "Quận Riscani"), ("zh", "雷什卡內區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::MD,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شولدانيشتي"), ("bg", "Шолданещки район"), ("bn", "সোল\u{9cd}ড\u{9be}নেস\u{9cd}তি জেল\u{9be}"), ("ca", "Districte de Şoldăneşti"), ("ccp", "𑄥\u{1112e}𑄣\u{11134}𑄓𑄚𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Șoldănești"), ("cs", "Okres Şoldăneşti"), ("da", "Soldanesti District"), ("de", "Rajon Șoldănești"), ("el", "Επαρχία Σολντανέστι"), ("en", "Șoldănești"), ("es", "Distrito de Șoldănești"), ("fi", "Șoldăneștin kaupunginosa"), ("fr", "Raion de Șoldănești"), ("gl", "Distrito de Șoldănești"), ("gu", "સોલ\u{acd}ડ\u{ac7}ન\u{ac7}સ\u{acd}ટી જિલ\u{acd}લો"), ("hi", "सोल\u{94d}डान\u{947}स\u{94d}ती जिला"), ("id", "Raionul Şoldăneşti"), ("it", "distretto di Şoldăneşti"), ("ja", "ソルダネシュティ県"), ("ka", "შოლდენეშტის რაიონი"), ("kn", "ಸಲ\u{ccd}ಡಾನ\u{cc6}ಸ\u{ccd}ಟ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "숄더네슈티 구"), ("lt", "Šoldeneščio rajonas"), ("lv", "Šoldeneštu rajons"), ("mr", "सोल\u{94d}डन\u{947}स\u{94d}टी जिल\u{94d}हा"), ("ms", "Daerah Soldanesti"), ("nb", "Șoldănești"), ("nl", "Șoldănești"), ("no", "Șoldănești"), ("pl", "Rejon Şoldăneşti"), ("pt", "Şoldăneşti"), ("ro", "raionul Șoldănești"), ("ru", "Шолданештский район"), ("si", "සෝල\u{dca}ද\u{dcf}නෙස\u{dca}ට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Şoldăneşti"), ("sq", "Raionul Șoldănești"), ("sv", "Şoldăneşti (distrikt)"), ("ta", "ஸ\u{bcd}ஓல\u{bcd}டன\u{bcd}ஸ\u{bcd}தி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c4b}ల\u{c4d}డ\u{c3e}న\u{c46}స\u{c4d}ట\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโซแดเนสต\u{e34}"), ("tr", "Soldanesti Dstric"), ("uk", "Шолданештський район"), ("ur", "شولدانیشتی ضلع"), ("vi", "Quận Soldanesti"), ("zh", "紹爾德內什蒂區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SI",
                    Subdivision{
                        name: "SI",
                        country_alpha2: Alpha2::MD,
                        code: "SI",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سونجري"), ("bg", "Сънджерейски район"), ("bn", "সিঙ\u{9cd}গেরি জেল\u{9be}"), ("ca", "Districte de Sîngerei"), ("ccp", "𑄥\u{11128}𑄚\u{11134}𑄉𑄬𑄢\u{1112d}"), ("ceb", "Sîngerei"), ("da", "Sîngerei District"), ("de", "Rajon Sîngerei"), ("el", "Σίνγκερεϊ"), ("en", "Sîngerei"), ("es", "Distrito de Sîngerei"), ("fi", "Sîngerein piiri"), ("fr", "Raion de Sîngerei"), ("gu", "સિન\u{acd}ગ\u{ac7}રી જિલ\u{acd}લો"), ("hi", "सि\u{902}ग\u{947}री जिला"), ("id", "Raionul Sîngerei"), ("it", "distretto di Sîngerei"), ("ja", "シンジェレイ県"), ("ka", "სინჯერეის რაიონი"), ("kn", "ಸ\u{cbf}ಂಗೇರ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "슨제레이 구"), ("lt", "Sindžerėjaus rajonas"), ("lv", "Sindžerejas rajons"), ("mr", "सिन\u{947}ग\u{947}री जिल\u{94d}हा"), ("ms", "Singerei District"), ("nb", "Sîngerei"), ("nl", "Sîngerei"), ("no", "Sîngerei"), ("pl", "Rejon Sîngerei"), ("pt", "Sîngerei"), ("ro", "raionul Sîngerei"), ("ru", "Сынжерейский район"), ("si", "ස\u{dd2}න\u{dca}ගේරෙය\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Sîngerei"), ("sq", "Raionul Sîngerei"), ("sv", "Sîngerei (distrikt)"), ("ta", "சிகெரெய\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3f}ంగర\u{c40} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซ\u{e34}นก\u{e35}ไร"), ("tr", "Singerei District"), ("uk", "Синжерейський район"), ("ur", "سینگیرئی ضلع"), ("vi", "Quận Singerei"), ("zh", "森傑雷區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::MD,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::TerritorialUnit,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("be", "Аўтаномнае тэрытарыяльнае ўтварэнне з асаблівым прававым статусам Прыднястроўе"), ("ccp", "𑄑\u{11133}𑄢𑄚\u{11134}𑄚\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Unitatea Teritorială din Stînga Nistrului"), ("cs", "Autonomní územně správní jednotka se zvláštním statusem Podněstří"), ("en", "Transnistria"), ("es", "Unidad territorial autónoma con un estatus jurídico especial Transnistria"), ("fr", "Transnistrie"), ("ja", "沿ドニエストル地域"), ("ko", "트란스니스트리아 자치 영토 단위"), ("ro", "Unitățile Administrativ-Teritoriale din Stînga Nistrului"), ("ru", "Административно-территориальные единицы левобережья Днестра"), ("uk", "Придністровʼя"), ("ur", "ٹرینسنیسٹریا خود مختار علاقائی اکائی مع خصوصی قانونی حیثیت"), ("zh", "德涅斯特河沿岸自治领土单位")]),
                        unofficial_name_list: ["Stînga Nistrului, unitatea teritoriala din"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::MD,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.166667), longitude: Some(28.3), max_latitude: Some(48.1874054), min_latitude: Some(48.1209551), max_longitude: Some(28.3286763), min_longitude: Some(28.2504846)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سوروكا"), ("bg", "Сорокски район"), ("bn", "সরোক\u{9be} জেল\u{9be}"), ("ca", "Districte de Soroca"), ("ccp", "𑄥\u{11127}𑄢\u{1112e}𑄇"), ("ceb", "Raionul Soroca"), ("cs", "Okres Soroca"), ("da", "Soroca District"), ("de", "Rajon Soroca"), ("el", "Σορόκα"), ("en", "Soroca"), ("es", "Distrito de Soroca"), ("fi", "Sorocan kaupunginosa"), ("fr", "Raion de Soroca"), ("gu", "સોરોકા જિલ\u{acd}લો"), ("hi", "सोरोका जिला"), ("hy", "Սորոկի շրջան"), ("id", "Raionul Soroca"), ("it", "distretto di Soroca"), ("ja", "ソロカ県"), ("ka", "სოროკის რაიონი"), ("kn", "ಸೊರೊಕಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "소로카 구"), ("lt", "Sorokos rajonas"), ("lv", "Sorokas rajons"), ("mr", "सोरकोआ जिल\u{94d}हा"), ("ms", "Soroca District"), ("nb", "Soroca"), ("nl", "Soroca"), ("no", "Soroca"), ("pl", "Rejon Soroca"), ("pt", "Distrito de Soroca"), ("ro", "raionul Soroca"), ("ru", "Сорокский район"), ("si", "සොරෝක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Soroca (okres)"), ("sq", "Raionul Soroca"), ("sv", "Soroca rajon"), ("ta", "ஸோரோச\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c4a}ర\u{c4b}క\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "โซโรคา"), ("tr", "Soroco District"), ("uk", "Сороцький район"), ("ur", "سوروکا ضلع"), ("vi", "Quận Soroca"), ("zh", "索羅卡區")]),
                        unofficial_name_list: ["Soroca"].to_vec(),
                    }
                ),
                (
                    "ST",
                    Subdivision{
                        name: "ST",
                        country_alpha2: Alpha2::MD,
                        code: "ST",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ستراسيني"), ("be", "Страшэнскі раён"), ("bg", "Страшенски район"), ("bn", "স\u{9cd}ট\u{9cd}র\u{9be}সেনি জেল\u{9be}"), ("ca", "Districte de Străşeni"), ("ccp", "𑄑\u{11133}𑄢𑄥𑄬𑄚\u{11128}"), ("ceb", "Raionul Strășeni"), ("da", "Strășeni District"), ("de", "Rajon Strășeni"), ("el", "Στρασένι"), ("en", "Strășeni"), ("es", "Distrito de Străşeni"), ("fi", "Strășenin kaupunginosa"), ("fr", "Raion de Strășeni"), ("gu", "સ\u{acd}ટ\u{acd}રાસ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "स\u{94d}त\u{94d}रस\u{947}नी जिला"), ("id", "Raionul Străşeni"), ("it", "distretto di Strășeni"), ("ja", "ストラセニ県"), ("ka", "სტრეშენის რაიონი"), ("kn", "ಸ\u{ccd}ಟ\u{ccd}ರಾಷ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "스트러셰니 구"), ("lt", "Strešenio rajonas"), ("lv", "Strešenu rajons"), ("mr", "स\u{94d}ट\u{94d}रस\u{947}नी जिल\u{94d}हा"), ("ms", "Straseni District"), ("nb", "Strășeni"), ("nl", "Strășeni"), ("no", "Strășeni"), ("pl", "Rejon Străşeni"), ("pt", "Străşeni"), ("ro", "raionul Strășeni"), ("ru", "Страшенский район"), ("si", "ස\u{dca}ට\u{dca}ර\u{dcf}සෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Străşeni"), ("sq", "Raionul Strășeni"), ("sv", "Raionul Străşeni"), ("ta", "ஸ\u{bcd}டரசேனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c4d}ట\u{c4d}ర\u{c3e}స\u{c46}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตสตราเซน\u{e34}"), ("tr", "Straseni District"), ("uk", "Страшенський район"), ("ur", "ستراشینی ضلع"), ("vi", "Quận Straseni"), ("zh", "斯特勒謝尼區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SV",
                    Subdivision{
                        name: "SV",
                        country_alpha2: Alpha2::MD,
                        code: "SV",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ستيفان فودا"), ("bg", "Щефанводски район"), ("bn", "স\u{9cd}টেফ\u{9be}ন ভোদ\u{9be} জেল\u{9be}"), ("ca", "Districte de Ştefan Vodă"), ("ccp", "𑄌\u{11133}𑄑𑄬𑄜𑄚\u{11134} 𑄞\u{1112e}𑄓"), ("ceb", "Raionul Ștefan Vodă"), ("cs", "Okres Ștefan Vodă"), ("da", "Stefan Voda"), ("de", "Rajon Ștefan Vodă"), ("el", "Στεφάν Βόντα"), ("en", "Ştefan Vodă"), ("es", "Distrito de Ştefan Vodă"), ("fi", "Ștefan Vodăn piiri"), ("fr", "raion de Ștefan Vodă"), ("gu", "સ\u{acd}ટ\u{ac7}ફન વોડા જિલ\u{acd}લો"), ("hi", "स\u{94d}ट\u{947}फन वोडा जिला"), ("id", "Raionul Ştefan Vodă"), ("it", "distretto di Ştefan Vodă"), ("ja", "シュテファン・ボーダ県"), ("ka", "შტეფან-ვოდეს რაიონი"), ("kn", "ಸ\u{ccd}ಟ\u{cc6}ಫಾನ\u{ccd} ವೋಡಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "슈테판보더 구"), ("lt", "Štefan Vodės rajonas"), ("lv", "Štefanvodes rajons"), ("mr", "स\u{947}ट\u{94d}फ\u{93c}न वोडा जिल\u{94d}हा"), ("ms", "Stefan Voda District"), ("nb", "Ștefan Vodă"), ("nl", "Ștefan Vodă"), ("no", "Ștefan Vodă"), ("pl", "Rejon Ştefan Vodă"), ("pt", "Ştefan Vodă (condado)"), ("ro", "raionul Ștefan Vodă"), ("ru", "Штефан-Водский район"), ("si", "ස\u{dca}ටෙෆ\u{dcf}න\u{dca} වොඩ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Ştefan Vodă (okres)"), ("sq", "Raionul Ștefan Vodă"), ("sv", "Raionul Ştefan Vodă"), ("ta", "ஸ\u{bcd}ட\u{bc0}பன\u{bcd} வோட\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c4d}ట\u{c46}ఫ\u{c3e}న\u{c4d} వ\u{c4b}డ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตสเตฟาน แบงค\u{e4c}"), ("tr", "Stefan Voda"), ("uk", "Штефан-Водський район"), ("ur", "شتیفان وودا ضلع"), ("vi", "Quận Stefan Voda"), ("zh", "斯特凡大公區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::MD,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5703), longitude: Some(29.11559999999999), max_latitude: Some(46.5827462), min_latitude: Some(46.547203), max_longitude: Some(29.1362143), min_longitude: Some(29.0931702)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تاراكليا"), ("be", "Тараклійскі раён"), ("bg", "Тараклийски район"), ("bn", "ত\u{9be}র\u{9be}কলিয\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Districte de Taraclia"), ("ccp", "𑄑𑄢𑄇\u{11134}𑄣\u{11128}𑄠"), ("ceb", "Taraclia"), ("da", "Taraclia District"), ("de", "Rajon Taraclia"), ("el", "Ταράκλια"), ("en", "Taraclia"), ("es", "Distrito de Taraclia"), ("fi", "Taraclian kaupunginosa"), ("fr", "Raion de Taraclia"), ("gu", "તરાક\u{acd}લિયા જિલ\u{acd}લો"), ("hi", "ट\u{947}राक\u{94d}लाया जिला"), ("id", "Raionul Taraclia"), ("it", "distretto di Taraclia"), ("ja", "タラクリア県"), ("ka", "ტარაკლიის რაიონი"), ("kn", "ಟರಾಕ\u{ccd}ಲ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "타라클리아 구"), ("lt", "Taraklijos rajonas"), ("lv", "Taraklijas rajons"), ("mr", "तारकिया जिल\u{94d}हा"), ("ms", "Taraclia District"), ("nb", "Taraclia"), ("nl", "Taraclia"), ("no", "Taraclia"), ("pl", "Rejon Taraclia"), ("pt", "Taraclia"), ("ro", "raionul Taraclia"), ("ru", "Тараклийский район"), ("si", "ටරස\u{dd2}ල\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Taraclia"), ("sq", "Raionul Taraclia"), ("sv", "Taraclia rajon"), ("ta", "ட\u{bcd}ரைகிளிய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c3e}ర\u{c3e}స\u{c4d}ల\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตตาราเคล\u{e35}ย"), ("tr", "Taraclia District"), ("uk", "Тараклійський район"), ("ur", "تاراکلیا ضلع"), ("vi", "Quận Taraclia"), ("zh", "塔拉克利亞區")]),
                        unofficial_name_list: ["Taraclia"].to_vec(),
                    }
                ),
                (
                    "TE",
                    Subdivision{
                        name: "TE",
                        country_alpha2: Alpha2::MD,
                        code: "TE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تيلينيشتي"), ("be", "Целянешцкі раён"), ("bg", "Телнещки район"), ("bn", "তেলেনেস\u{9cd}তি জেল\u{9be}"), ("ca", "Districte de Teleneşti"), ("ccp", "𑄑𑄬𑄣𑄬𑄚𑄬𑄌\u{11134}𑄑\u{11128}"), ("ceb", "Telenești (distrito)"), ("da", "Telenești District"), ("de", "Rajon Telenești"), ("el", "Τελενέστι"), ("en", "Telenești"), ("es", "Distrito de Teleneşti"), ("fi", "Teleneștin kaupunginosa"), ("fr", "Raion de Telenești"), ("gu", "ટ\u{ac7}લ\u{ac7}ન\u{ac7}સ\u{acd}ટી જિલ\u{acd}લો"), ("hi", "ट\u{947}ल\u{947}न\u{947}स\u{94d}टी जिला"), ("id", "Raionul Teleneşti"), ("it", "distretto di Telenești"), ("ja", "テレネシュティ県"), ("ka", "ტელენეშტის რაიონი"), ("kn", "ಟ\u{cc6}ಲ\u{cc6}ನ\u{cc6}ಸ\u{ccd}ತ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "텔레네슈티 구"), ("lt", "Teleneščio rajonas"), ("lv", "Teleneštu rajons"), ("mr", "त\u{947}लन\u{947}स\u{94d}टी जिल\u{94d}हा"), ("ms", "Telenesti District"), ("nb", "Telenești"), ("nl", "Telenești"), ("no", "Telenești"), ("pl", "Rejon Teleneşti"), ("pt", "Teleneşti (condado)"), ("ro", "raionul Telenești"), ("ru", "Теленештский район"), ("si", "ටෙලෙන\u{dca}ස\u{dca}ට\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Teleneşti (okres)"), ("sq", "Raionul Telenești"), ("sv", "Teleneşti (distrikt)"), ("ta", "டெலென\u{bcd}ஸ\u{bcd}தி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ట\u{c46}ల\u{c3f}న\u{c46}స\u{c4d}ట\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเทเลเนสต\u{e34}"), ("tr", "Teleneşti District"), ("uk", "Теленештський район"), ("ur", "تیلینیشتی ضلع"), ("vi", "Quận Telenesti"), ("zh", "泰萊內什蒂區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "UN",
                    Subdivision{
                        name: "UN",
                        country_alpha2: Alpha2::MD,
                        code: "UN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.216667), longitude: Some(27.816667), max_latitude: Some(47.2493487), min_latitude: Some(47.1690154), max_longitude: Some(27.8279203), min_longitude: Some(27.7592753)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة اونغيني"), ("bg", "Унгенски район"), ("bn", "আনগেনি জেল\u{9be}"), ("ca", "Districte d’Ungheni"), ("ccp", "𑄃𑄚\u{11134}𑄊𑄬𑄚\u{11128}"), ("ceb", "Raionul Ungheni"), ("cs", "Okres Ungheni"), ("da", "Ungheni District"), ("de", "Rajon Ungheni"), ("el", "Ουνγκένι"), ("en", "Ungheni"), ("es", "Distrito de Ungheni"), ("et", "Ungheni maakond"), ("fi", "Unghenin kaupunginosa"), ("fr", "Raion d’Ungheni"), ("gu", "ઉ\u{a82}ઘ\u{ac7}ની જિલ\u{acd}લો"), ("hi", "उ\u{902}ग\u{947}नी जिला"), ("id", "Raionul Ungheni"), ("it", "distretto di Ungheni"), ("ja", "ウンゲニ県"), ("ka", "უნგენის რაიონი"), ("kn", "ಉಂಗ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "운게니 구"), ("lt", "Ungenio rajonas"), ("lv", "Ungenu rajons"), ("mr", "ऊनघ\u{947}णी जिल\u{94d}हा"), ("ms", "Ungheni District"), ("nb", "Ungheni"), ("nl", "Ungheni"), ("no", "Ungheni"), ("pl", "Rejon Ungheni"), ("pt", "Ungheni"), ("ro", "raionul Ungheni"), ("ru", "Унгенский район"), ("si", "උන\u{dca}ගෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Ungheni"), ("sq", "Raionul Ungheni"), ("sv", "Ungheni rajon"), ("ta", "உங\u{bcd}க\u{bcd}கேணி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఉంగ\u{c47}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อ\u{e39}นเกน\u{e35}"), ("tr", "Ungheni District"), ("uk", "Унгенський район"), ("ur", "اونگینی ضلع"), ("vi", "Quận Ungheni"), ("zh", "溫格內區")]),
                        unofficial_name_list: ["Ungheni"].to_vec(),
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
#[cfg(feature = "md")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MD,
        alpha3: Alpha3::MDA,
        address_format: None,
        continent: Continent::Europe,
        country_code: 373,
        currency_code: "MDL",
        gec: Some(GEC::MD),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::MDA),
        iso_long_name: "The Republic of Moldova",
        iso_short_name: "Moldova (Republic of)",
        official_language_list: ["ro"].to_vec(),
        spoken_language_list: ["ro"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Moldovan"),
        number: "498",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "MD",
        unofficial_name_list: [
            "Moldova",
            "Moldawien",
            "Moldavie",
            "Moldavia",
            "the Republic of Moldova",
            "モルドバ共和国",
            "Moldavië",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Moldova"),
            ("af", "Moldawië"),
            ("ak", "Moldova"),
            ("am", "ሞልዶቫ"),
            ("an", "Moldova"),
            ("ar", "المالديف"),
            ("as", "মোলডোভ\u{9be}"),
            ("ay", "Moldova"),
            ("az", "Moldova"),
            ("ba", "Moldova"),
            ("be", "Малдова"),
            ("bg", "Молдова"),
            ("bi", "Moldova"),
            ("bn", "মোলডোভ\u{9be}"),
            ("bn_IN", "মোলডোভ\u{9be}"),
            ("br", "Moldova"),
            ("bs", "Moldavija"),
            ("ca", "Moldàvia"),
            ("ce", "Молдави"),
            ("ch", "Moldova"),
            ("cs", "Moldavsko"),
            ("cv", "Молдави"),
            ("cy", "Moldofa"),
            ("da", "Moldova"),
            ("de", "Moldau"),
            ("dv", "މ\u{7ae}ލ\u{7b0}ޑ\u{7af}ވ\u{7a7}"),
            ("dz", "མ\u{f7c}ལ་ཌ\u{f7c}་ཝ།"),
            ("ee", "Moldova"),
            ("el", "Μολδαβία"),
            ("en", "Moldova"),
            ("eo", "Moldavio"),
            ("es", "Moldavia"),
            ("et", "Moldova"),
            ("eu", "Moldavia"),
            ("fa", "مولداوی"),
            ("ff", "Moldowa"),
            ("fi", "Moldova"),
            ("fo", "Moldova"),
            ("fr", "Moldavie"),
            ("fy", "Moldaavje"),
            ("ga", "An Mholdóiv"),
            ("gl", "Moldavia"),
            ("gn", "Moldova"),
            ("gu", "મોલ\u{acd}ડોવા"),
            ("gv", "Moldova"),
            ("ha", "MOldufiniya"),
            ("he", "מולדובה"),
            ("hi", "मॉल\u{94d}डोवा"),
            ("hr", "Moldavija"),
            ("ht", "Moldavi"),
            ("hu", "Moldova"),
            ("hy", "Մոլդովա"),
            ("ia", "Moldavia"),
            ("id", "Moldova"),
            ("io", "Moldova"),
            ("is", "Moldavía"),
            ("it", "Moldavia"),
            ("iu", "Moldova"),
            ("ja", "モルドバ"),
            ("ka", "მოლდოვა"),
            ("ki", "Moldova"),
            ("kk", "Молдова"),
            ("kl", "Moldova"),
            ("km", "ម\u{17bb}លឌ\u{17bc}វ\u{17c9}ា"),
            ("kn", "Moldova"),
            ("ko", "몰도바"),
            ("ku", "Moldova"),
            ("kv", "Молдова"),
            ("kw", "Moldova"),
            ("ky", "Молдова"),
            ("lo", "Moldova"),
            ("lt", "Moldova"),
            ("lv", "Moldova"),
            ("mi", "Morotawa"),
            ("mk", "Молдавија"),
            ("ml", "മോള\u{d4d}\u{200d}ഡോവ"),
            ("mn", "Молдав"),
            ("mr", "मोल\u{94d}डोवा"),
            ("ms", "Moldova"),
            ("mt", "Moldova"),
            (
                "my",
                "မော\u{103a}လ\u{103a}ဒ\u{102d}\u{102f}ဗာန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ripubrikin Mordowa"),
            ("nb", "Moldova"),
            ("ne", "मोल\u{94d}डोभा"),
            ("nl", "Moldavië"),
            ("nn", "Moldova"),
            ("nv", "Moldova"),
            ("oc", "Moldàvia"),
            ("or", "ମ\u{b3e}ଲଡୋଭ\u{b3e}"),
            ("pa", "ਮ\u{a4b}ਲਡੀਵਾ"),
            ("pi", "मोल\u{94d}दोवा"),
            ("pl", "Mołdawia"),
            ("ps", "مولدوا"),
            ("pt", "Moldávia"),
            ("pt_BR", "Moldávia"),
            ("ro", "Moldova"),
            ("ru", "Молдавия"),
            ("rw", "Molidova"),
            ("sc", "Moldàvia"),
            ("sd", "Moldova"),
            ("si", "මොල\u{dca}ඩෝව\u{dcf}"),
            ("sk", "Moldavsko"),
            ("sl", "Moldavija"),
            ("so", "Moldofa"),
            ("sq", "Moldavi"),
            ("sr", "Молдавија"),
            ("sv", "Moldavien"),
            ("sw", "Moldova"),
            ("ta", "மோல\u{bcd}டோவ\u{bbe}"),
            ("te", "మ\u{c3e}ల\u{c4d}డ\u{c4b}వ\u{c3e}"),
            ("tg", "Молдова"),
            ("th", "มอลโดวา"),
            ("ti", "Moldova"),
            ("tk", "Moldawiýa"),
            ("tl", "Moldova"),
            ("tr", "Moldova"),
            ("tt", "Moldova"),
            ("ug", "مولدوۋا"),
            ("uk", "Молдова"),
            ("ur", "مالدووا"),
            ("uz", "Moldova"),
            ("ve", "Moldova"),
            ("vi", "Mon-đô-va"),
            ("wa", "Moldova"),
            ("wo", "Moldaawi"),
            ("xh", "Moldova"),
            ("yo", "Móldófà"),
            ("zh_CN", "摩尔多瓦"),
            ("zh_HK", "摩爾多瓦"),
            ("zh_TW", "摩爾多瓦"),
            ("zu", "Moldova"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
