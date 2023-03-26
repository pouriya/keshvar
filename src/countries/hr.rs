// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Croatia

#[cfg(all(feature = "hr", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::HR;
    pub const ALPHA3: Alpha3 = Alpha3::HRV;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 385;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::HRK;
    pub const GEC: Option<GEC> = Some(GEC::HR);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::CRO);
    pub const ISO_SHORT_NAME: &str = "Croatia";
    pub const ISO_LONG_NAME: &str = "The Republic of Croatia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["hr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["hr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Croatian");
    pub const NUMBER: &str = "191";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "HR";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Croatia",
        "Kroatien",
        "Croatie",
        "Croacia",
        "クロアチア",
        "Kroatië",
        "Croatia (Hrvatska)",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Croatia"),
        ("af", "Kroasië"),
        ("ak", "Croatia"),
        ("am", "ጤስጔቄ።"),
        ("an", "Croatia"),
        ("ar", "كرواتيا"),
        ("as", "ক\u{9cd}ৰোয়েছিয়\u{9be}"),
        ("ay", "Croatia"),
        ("az", "Xırvatıstan"),
        ("ba", "Croatia"),
        ("be", "Харватыя"),
        ("bg", "Хърватия"),
        ("bi", "Croatia"),
        ("bn", "ক\u{9cd}রোয়েশিয়\u{9be}"),
        ("bn_IN", "ক\u{9cd}রোয়েশিয়\u{9be}"),
        ("br", "Kroatia"),
        ("bs", "Hrvatska"),
        ("ca", "Croàcia"),
        ("ce", "Хорвати"),
        ("ch", "Croatia"),
        ("cs", "Chorvatsko"),
        ("cv", "Хорвати"),
        ("cy", "Croatia"),
        ("da", "Kroatien"),
        ("de", "Kroatien"),
        ("dv", "ކ\u{7aa}ރ\u{7ae}އ\u{7ad}ޝ\u{7a8}އ\u{7a7}"),
        ("dz", "ཀ\u{f7c}ར\u{f7c}་ཤ\u{f72}་ཡ།"),
        ("ee", "Croatia"),
        ("el", "Κροατία"),
        ("en", "Croatia"),
        ("eo", "Kroatio"),
        ("es", "Croacia"),
        ("et", "Horvaatia"),
        ("eu", "Kroazia"),
        ("fa", "کرواسی"),
        ("ff", "Korowaasiya"),
        ("fi", "Kroatia"),
        ("fo", "Kroatia"),
        ("fr", "Croatie"),
        ("fy", "Kroaasje"),
        ("ga", "An Chróit"),
        ("gl", "Croacia"),
        ("gn", "Croatia"),
        ("gu", "ક\u{acd}રોએશિયા"),
        ("gv", "Yn Chroit"),
        ("ha", "Kroatiya"),
        ("he", "קרואטיה"),
        ("hi", "क\u{94d}रोएशिया"),
        ("hr", "Hrvatska"),
        ("ht", "Kroasi"),
        ("hu", "Horvátország"),
        ("hy", "Խորվաթիա"),
        ("ia", "Croatia"),
        ("id", "Kroasia"),
        ("io", "Kroatia"),
        ("is", "Króatía"),
        ("it", "Croazia"),
        ("iu", "Croatia"),
        ("ja", "クロアチア"),
        ("ka", "ჰორვატია"),
        ("ki", "Croatia"),
        ("kk", "Хорватия"),
        ("kl", "Croatia"),
        ("km", "ក\u{17d2}រ\u{17bc}អាត"),
        ("kn", "ಕ\u{ccd}ರೊಏಶ\u{cbf}ಯಾ"),
        ("ko", "크로아티아"),
        ("ku", "Hirvatistan"),
        ("kv", "Хорватия"),
        ("kw", "Kroati"),
        ("ky", "Хорватия"),
        ("lo", "ປະເທດກະໂລອາຊ\u{eb5}"),
        ("lt", "Kroatija"),
        ("lv", "Horvātija"),
        ("mi", "Koroātia"),
        ("mk", "Хрватска"),
        ("ml", "ക\u{d4d}രോയേഷ\u{d4d}യ"),
        ("mn", "Хорват"),
        ("mr", "क\u{94d}रोशिया"),
        ("ms", "Kroatia"),
        ("mt", "Kroazja"),
        (
            "my",
            "ခရ\u{102d}\u{102f}အေးရ\u{103e}ားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Kroaitsiya"),
        ("nb", "Kroatia"),
        ("ne", "क\u{94d}रोएशिया"),
        ("nl", "Kroatië"),
        ("nn", "Kroatia"),
        ("nv", "Kwóóʼad Bikéyah"),
        ("oc", "Croàcia"),
        ("or", "କ\u{b4d}ରୋଏଶ\u{b3f}ଆ"),
        ("pa", "ਕਰ\u{a4b}ਟੀਆ"),
        ("pi", "क\u{94d}रोएशिया"),
        ("pl", "Chorwacja"),
        ("ps", "کروواسيا"),
        ("pt", "Croácia"),
        ("pt_BR", "Croácia"),
        ("ro", "Croația"),
        ("ru", "Хорватия"),
        ("rw", "Korowatiya"),
        ("sc", "Croàtzia"),
        ("sd", "Croatia"),
        ("si", "ක\u{dca}\u{200d}රෝශ\u{dd2}ය\u{dcf}"),
        ("sk", "Chorvátsko"),
        ("sl", "Hrvaška"),
        ("so", "Korweeshiya"),
        ("sq", "Kroaci"),
        ("sr", "Хрватска"),
        ("sv", "Kroatien"),
        ("sw", "Croatia"),
        ("ta", "குரோவேசிய\u{bbe}"),
        ("te", "క\u{c4d}ర\u{c4b}ట\u{c3f}య\u{c3e}"),
        ("tg", "Хорватия"),
        ("th", "โครเอเช\u{e35}ย"),
        ("ti", "ክሮኤሽያ"),
        ("tk", "Horwatiýa"),
        ("tl", "Croatia"),
        ("tr", "Hırvatistan"),
        ("tt", "Кроатиа"),
        ("ug", "كىرودىيە"),
        ("uk", "Хорватія"),
        ("ur", "کروشیا"),
        ("uz", "Xorvatiya"),
        ("ve", "Croatia"),
        ("vi", "Cợ-rô-a-ti-a"),
        ("wa", "Crowåceye"),
        ("wo", "Korwaasi"),
        ("xh", "Croatia"),
        ("yo", "Kroatíà"),
        ("zh_CN", "克罗地亚"),
        ("zh_HK", "克羅地亞"),
        ("zh_TW", "克羅埃西亞"),
        ("zu", "IKrowati"),
    ];
    #[cfg(all(feature = "hr", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 45.1;
        pub const LONGITUDE: f64 = 15.2000001;
        pub const MAX_LATITUDE: f64 = 46.5549857;
        pub const MAX_LONGITUDE: f64 = 19.4480523;
        pub const MIN_LATITUDE: f64 = 42.3385087;
        pub const MIN_LONGITUDE: f64 = 13.3649;
        pub const NORTHEAST_LATITUDE: f64 = 46.5549857;
        pub const NORTHEAST_LONGITUDE: f64 = 19.4480523;
        pub const SOUTHWEST_LATITUDE: f64 = 42.3385087;
        pub const SOUTHWEST_LONGITUDE: f64 = 13.3649;
    }
}
#[cfg(all(feature = "hr", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 45.1,
            longitude: 15.2000001,
            max_latitude: 46.5549857,
            max_longitude: 19.4480523,
            min_latitude: 42.3385087,
            min_longitude: 13.3649,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 46.5549857,
                    longitude: 19.4480523,
                },
                southwest: CountryGeoBound {
                    latitude: 42.3385087,
                    longitude: 13.3649,
                },
            },
        }
    }
}

#[cfg(all(feature = "hr", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::HR,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8706612), longitude: Some(16.395491), max_latitude: Some(46.0695047), min_latitude: Some(45.4676982), max_longitude: Some(16.7137865), min_longitude: Some(15.3262593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زغرب"), ("be", "Заграбская жупанія"), ("bg", "Загребска жупания"), ("bn", "জ\u{9be}গরেব ক\u{9be}উন\u{9cd}টি"), ("bs", "Zagrebačka županija"), ("ca", "Comtat de Zagreb"), ("ccp", "𑄎𑄉\u{11133}𑄢𑄬𑄛\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Zagreb County"), ("cs", "Záhřebská župa"), ("da", "Zagreb County"), ("de", "Gespanschaft Zagreb"), ("el", "Ζάγκρεμπ"), ("en", "Zagreb County"), ("es", "Zagreb"), ("et", "Zagrebi maakond"), ("eu", "Zagreb eskualdea"), ("fa", "ایالت زاگرب"), ("fi", "Zagreb"), ("fr", "Comitat de Zagreb"), ("ga", "Contae Ságrab"), ("gu", "ઝાગ\u{acd}ર\u{ac7}બ કાઉન\u{acd}ટી"), ("hi", "ज\u{93c}ाग\u{94d}र\u{947}ब काउ\u{902}टी"), ("hr", "Zagrebačka županija"), ("hu", "Zágráb megye"), ("id", "Kabupaten Zagreb"), ("it", "regione di Zagabria"), ("ja", "ザグレブ郡"), ("ka", "ზაგრების ოლქი"), ("kn", "ಝಾಗ\u{ccd}ರ\u{cc6}ಬ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "자그레브 주"), ("lt", "Zagrebo apskritis"), ("lv", "Zagrebas župānija"), ("mk", "Загрепска жупанија"), ("mr", "झगाब\u{947}ब काउ\u{902}टी"), ("ms", "Zagreb County"), ("nb", "Zagreb"), ("nl", "Zagreb"), ("no", "Zagreb"), ("pl", "Zagrebačka županija"), ("pt", "Condado de Zagreb"), ("ro", "Cantonul Zagreb"), ("ru", "Загребская жупания"), ("si", "සග\u{dca}\u{200d}රෙබ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Záhrebská župa"), ("sl", "Zagrebška županija"), ("sq", "Zhupania e Zagrebit"), ("sr", "Загребачка жупанија"), ("sr_Latn", "Zagrebačka županija"), ("sv", "Zagrebs län"), ("ta", "சக\u{bcd}ரேப\u{bcd} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}గ\u{c4d}ర\u{c46}బ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ซาเกร\u{e47}บ"), ("tr", "Zagreb Bölgesi"), ("uk", "Загребська жупанія"), ("ur", "زگریب کاؤنٹی"), ("vi", "Zagreb"), ("zh", "薩格勒布縣")]),
                        unofficial_name_list: ["Zagreb County"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::HR,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.10133930000001), longitude: Some(15.8809693), max_latitude: Some(46.2800141), min_latitude: Some(45.9023158), max_longitude: Some(16.2571245), min_longitude: Some(15.5954704)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كرابينا زاغوريه"), ("bg", "Крапинско-загорска жупания"), ("bn", "ক\u{9cd}র\u{9be}পিন\u{9be}-জ\u{9be}গোর\u{9cd}জে ক\u{9be}উন\u{9cd}টি"), ("bs", "Krapinsko-zagorska županija"), ("ca", "Comtat de Krapina-Zagorje"), ("ccp", "𑄇\u{11133}𑄢𑄛\u{11128}𑄚-𑄎𑄉\u{1112e}𑄢\u{11134}𑄎𑄬"), ("ceb", "Krapinsko-Zagorska Županija"), ("cs", "Krapinsko-zagorská župa"), ("da", "Krapina-Zagorje County"), ("de", "Gespanschaft Krapina-Zagorje"), ("el", "Κραπίνα-Ζαγκόρτζε"), ("en", "Krapina-Zagorje"), ("es", "Krapina-Zagorje"), ("et", "Krapina-Zagorje maakond"), ("eu", "Krapina-Zagorje eskualdea"), ("fa", "کراپینا-زاگریه"), ("fi", "Krapina-Zagorje"), ("fr", "Comitat de Krapina-Zagorje"), ("gu", "ક\u{acd}ર\u{ac7}પીના-ઝાગોર\u{acd}જ\u{ac7} કાઉન\u{acd}ટી"), ("hi", "क\u{94d}रापिना-ज\u{93c}गोरिए काउ\u{902}टी"), ("hr", "Krapinsko-zagorska županija"), ("hu", "Krapina-Zagorje megye"), ("id", "Kabupaten Krapina-Zagorje"), ("it", "regione di Krapina e dello Zagorje"), ("kn", "ಕ\u{ccd}ರಾಪ\u{cbf}ನಾ-ಝಗೋರ\u{ccd}ಜ\u{cc6} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "크라피나자고레 주"), ("lt", "Krapinos–Zagorjės apskritis"), ("lv", "Krapinas-Zagorjes župānija"), ("mk", "Крапинско-загорска жупанија"), ("mr", "क\u{94d}रिपाना-ज\u{93c}गोर\u{94d}ज\u{947} काउ\u{902}टी"), ("ms", "Daerah Krapina-Zagorje"), ("nb", "Krapina-Zagorje"), ("nl", "Krapina-Zagorje"), ("no", "Krapina-Zagorje"), ("pl", "Krapinsko-zagorska županija"), ("pt", "Condado de Krapina-Zagorje"), ("ro", "Cantonul Krapina-Zagorje"), ("ru", "Крапинско-Загорская жупания"), ("si", "ක\u{dca}රප\u{dd2}න\u{dcf}-සගොර\u{dca}ජේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Krapinsko-zagorská župa"), ("sl", "Krapinsko-zagorska županija"), ("sq", "Zhupania e Zagorës dhe Krapinës"), ("sr", "Крапинско-загорска жупанија"), ("sr_Latn", "Krapinsko-zagorska županija"), ("sv", "Krapina-Zagorjes län"), ("ta", "க\u{bcd}ர\u{bbe}பின\u{bbe} - சகோரஜே கவுண\u{bcd}டி"), ("te", "క\u{c4d}ర\u{c3e}ప\u{c3f}న\u{c3e}-జ\u{c3e}గ\u{c4b}ర\u{c4d}య\u{c47} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลคราป\u{e34}นาซาโกรเจ"), ("tr", "Krapina-Zagorje"), ("uk", "Крапинсько-Загорська жупанія"), ("ur", "کراپینا-زاکوریے کاؤنٹی"), ("vi", "Hạt Krapina-Zagorje"), ("zh", "克拉皮納-扎戈列縣")]),
                        unofficial_name_list: ["Krapina-Zagorje"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::HR,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.3837926), longitude: Some(16.5380995), max_latitude: Some(45.6699168), min_latitude: Some(44.9959092), max_longitude: Some(17.1949134), min_longitude: Some(15.7676525)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سيساك-موسلافينا"), ("bg", "Сисашко-мославска жупания"), ("bn", "সিস\u{9be}ক-মস\u{9cd}ল\u{9be}ভিন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Sisačko-moslavačka županija"), ("ca", "Comtat de Sisak-Moslavina"), ("ccp", "𑄥\u{11128}𑄥𑄇\u{11134}-𑄟\u{1112e}𑄌\u{11134}𑄣𑄞\u{11128}𑄚"), ("ceb", "Sisačko-Moslavačka Županija"), ("cs", "Sisacko-moslavinská župa"), ("da", "Sisak-Moslavina County"), ("de", "Gespanschaft Sisak-Moslavina"), ("el", "Σισάκ-Μοσλαβίνα"), ("en", "Sisak-Moslavina"), ("es", "Sisak-Moslavina"), ("et", "Sisaki-Moslavina maakond"), ("eu", "Sisak-Moslavina eskualdea"), ("fa", "شهرستان سیساک-موسلاوینا"), ("fi", "Sisak-Moslavina"), ("fr", "Comitat de Sisak-Moslavina"), ("gu", "સિસ\u{ac7}ક-મોસ\u{acd}લાવિના કાઉન\u{acd}ટી"), ("he", "מחוז סיסאק-מוסלאבינה"), ("hi", "सिसाक-मोस\u{94d}लाविना काउ\u{902}टी"), ("hr", "Sisačko-moslavačka županija"), ("hu", "Sziszek-Moslavina megye"), ("id", "Kabupaten Sisak-Moslavina"), ("it", "regione di Sisak e della Moslavina"), ("kn", "ಸ\u{cbf}ಸಾಕ\u{ccd}-ಮೊಸ\u{ccd}ಲಾವ\u{cbf}ನ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "시사크모슬라비나 주"), ("lt", "Sisako–Moslavinos apskritis"), ("lv", "Sisakas-Moslavinas župānija"), ("mk", "Сисачко-мославачка жупанија"), ("mr", "सिस\u{945}क-मोसलविणा काउ\u{902}टी"), ("ms", "Sisak-Moslavina County"), ("nb", "Sisak-Moslavina"), ("nl", "Sisak-Moslavina"), ("no", "Sisak-Moslavina"), ("pl", "Sisačko-moslavačka županija"), ("pt", "Condado de Sisak-Moslavina"), ("ro", "Cantonul Sisak-Moslavina"), ("ru", "Сисацко-Мославинская жупания"), ("si", "ස\u{dd2}සක\u{dca}-මොස\u{dca}ලව\u{dd2}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sisacko-moslavinská župa"), ("sl", "Sisaško-moslavaška županija"), ("sq", "Zhupania e Sisakut dhe Mosllovaçës"), ("sr", "Сисачко-мославачка жупанија"), ("sr_Latn", "Sisačko-moslavačka županija"), ("sv", "Sisak-Moslavinas län"), ("ta", "சிசக\u{bcd}-மொஸில\u{bcd}ல\u{bbe}வின\u{bbe} கவுண\u{bcd}டி"), ("te", "స\u{c3f}స\u{c3e}క\u{c4d}-మ\u{c4b}స\u{c4d}ల\u{c3e}వ\u{c3f}న\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "\u{e34}เม\u{e37}องไซซ\u{e31}ก-มอสลาว\u{e34}นา"), ("tr", "Sisak-Moslavina"), ("uk", "Сісацько-Мославінська жупанія"), ("ur", "سیساک-موسلاوینا کاؤنٹی"), ("vi", "Hạt Sisak-Moslavina"), ("zh", "錫薩克-莫斯拉維納縣")]),
                        unofficial_name_list: ["Sisak-Moslavina"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::HR,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.2389445), longitude: Some(15.3980067), max_latitude: Some(45.7628058), min_latitude: Some(44.88255729999999), max_longitude: Some(15.9247762), min_longitude: Some(14.961508)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كارلوفاتش"), ("be", "Карлавачка"), ("bg", "Карловацка жупания"), ("bn", "ক\u{9be}র\u{9cd}লোভ\u{9be}ক ক\u{9be}উন\u{9cd}টি"), ("bs", "Karlovačka županija"), ("ca", "Comtat de Karlovac"), ("ccp", "𑄇𑄢\u{11134}𑄣\u{1112e}𑄞𑄇\u{11134}"), ("ceb", "Karlovačka Županija"), ("cs", "Karlovacká župa"), ("da", "Karlovac County"), ("de", "Gespanschaft Karlovac"), ("el", "Κάρλοβακ"), ("en", "Karlovac"), ("es", "Karlovac"), ("et", "Karlovaci maakond"), ("eu", "Karlovac eskualdea"), ("fa", "ایالت کارلوواک"), ("fi", "Karlovac"), ("fr", "Comitat de Karlovac"), ("gu", "કાર\u{acd}લોવાક કાઉન\u{acd}ટી"), ("hi", "कार\u{94d}लोव\u{948}क काउ\u{902}टी"), ("hr", "Karlovačka županija"), ("hu", "Károlyváros megye"), ("id", "Kabupaten Karlovac"), ("it", "regione di Karlovac"), ("ja", "カルロヴァツ郡"), ("ka", "კარლოვაცი"), ("kn", "ಕಾರ\u{ccd}ಲೋವಾಕ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "카를로바츠 주"), ("lt", "Karlovaco apskritis"), ("lv", "Karlovacas župānija"), ("mk", "Карловачка жупанија"), ("mr", "कार\u{94d}लोव\u{94d}ह\u{945}क काउ\u{902}टी"), ("ms", "Karlovac County"), ("nb", "Karlovac"), ("nl", "Karlovac"), ("no", "Karlovac"), ("pl", "Karlovačka županija"), ("pt", "Condado de Karlovac"), ("ro", "Cantonul Karlovac"), ("ru", "Карловачка"), ("si", "කර\u{dca}ලොවැක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Karlovecká župa"), ("sl", "Karlovška županija"), ("sq", "Zhupania e Karllovcit"), ("sr", "Карловачка жупанија"), ("sr_Latn", "Karlovačka županija"), ("sv", "Karlovacs län"), ("ta", "க\u{bbe}ர\u{bcd}லோவக\u{bcd} கவுண\u{bcd}டி"), ("te", "క\u{c3e}ర\u{c4d}ల\u{c4b}వ\u{c3e}క\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลคาร\u{e4c}โลว\u{e31}ตส\u{e4c}"), ("tr", "Karlovac Bölgesi"), ("uk", "Карловацька жупанія"), ("ur", "کارلوواتس کاؤنٹی"), ("vi", "Hạt Karlovac"), ("zh", "卡爾洛瓦茨縣")]),
                        unofficial_name_list: ["Karlovac"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::HR,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.23174729999999), longitude: Some(16.3360558), max_latitude: Some(46.4048343), min_latitude: Some(46.0103401), max_longitude: Some(16.7739716), min_longitude: Some(15.8765333)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فرازدين"), ("be", "Вараждзінская жупанія"), ("bg", "Вараждинска жупания"), ("bn", "ভ\u{9be}র\u{9be}জদিন ক\u{9be}উন\u{9cd}টি"), ("bs", "Varaždinska županija"), ("ca", "Comtat de Varaždin"), ("ccp", "𑄞𑄢𑄌\u{11134}𑄓\u{11128}𑄚\u{11134}"), ("ceb", "Varaždinska Županija"), ("cs", "Varaždinská župa"), ("da", "Varaždin County"), ("de", "Gespanschaft Varaždin"), ("el", "Βαραζντίν"), ("en", "Varaždin"), ("es", "Varaždin"), ("et", "Varaždini maakond"), ("eu", "Varaždin eskualdea"), ("fa", "ایالت واراژدین"), ("fi", "Varaždinin piirikunta"), ("fr", "Comitat de Varaždin"), ("ga", "Contae Varaždin"), ("gu", "વરાઝડિન કાઉન\u{acd}ટી"), ("hi", "वराज\u{93c}दिन काउ\u{902}टी"), ("hr", "Varaždinska županija"), ("hu", "Varasd megye"), ("id", "Kabupaten Varaždin"), ("it", "regione di Varaždin"), ("ja", "ヴァラジュディン郡"), ("kn", "ವಾರಾಜ\u{ccd}ಡ\u{cbf}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "바라주딘 주"), ("lt", "Varaždino apskritis"), ("lv", "Varaždinas župānija"), ("mk", "Вараждинска жупанија"), ("mr", "वाराझ\u{947}डीन काउ\u{902}टी"), ("ms", "Varazdin County"), ("nb", "Varaždin"), ("nl", "Varaždin"), ("no", "Varaždin"), ("pl", "Varaždinska županija"), ("pt", "Condado de Varaždin"), ("ro", "Cantonul Varaždin"), ("ru", "Вараждинская жупания"), ("si", "වරස\u{dca}ඩ\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Varaždínska župa"), ("sl", "Varaždinska županija"), ("sq", "Zhupania e Varazhdinit"), ("sr", "Вараждинска жупанија"), ("sr_Latn", "Varaždinska županija"), ("sv", "Varaždins län"), ("ta", "வரஸ\u{bcd}ட\u{bcd}டின\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}ర\u{c3e}జ\u{c4d}డ\u{c3f}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "แอททาร\u{e4c}ด"), ("tr", "Varaždin Bölgesi"), ("uk", "Вараждинська жупанія"), ("ur", "واراژدن کاؤنٹی"), ("vi", "Hạt Varazdin"), ("zh", "瓦拉日丁縣")]),
                        unofficial_name_list: ["Varaždin"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::HR,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.1568919), longitude: Some(16.8390826), max_latitude: Some(46.3551961), min_latitude: Some(45.8802736), max_longitude: Some(17.3057121), min_longitude: Some(16.3417055)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوبريفنيكا-كريزفتسي"), ("bg", "Копривнишко-крижевска жупания"), ("bn", "কপ\u{9cd}রিভনিক\u{9be}-ক\u{9cd}রিজেভচি ক\u{9be}উন\u{9cd}টি"), ("bs", "Koprivničko-križevačka županija"), ("ca", "Comtat de Koprivnica-Križevci"), ("ccp", "𑄇\u{1112e}𑄛\u{11133}𑄢\u{11128}𑄛\u{11134}𑄚\u{11128}𑄇- 𑄇\u{11133}𑄢\u{11128}𑄎𑄬𑄛\u{11134}𑄥\u{1112d}"), ("ceb", "Koprivničko-Križevačka Županija"), ("cs", "Koprivnicko-križevecká župa"), ("cy", "Koprivnica-Križevci County"), ("da", "Koprivnica-Križevci County"), ("de", "Gespanschaft Koprivnica-Križevci"), ("el", "Κοπριβνίτσα-Κριζέβσκι"), ("en", "Koprivnica-Križevci"), ("es", "Koprivnica-Križevci"), ("et", "Koprivnica-Križevci maakond"), ("eu", "Koprivnica-Križevci eskualdea"), ("fa", "کپریونیتسا-کریژوتسی"), ("fr", "Comitat de Koprivnica-Križevci"), ("gu", "કોપ\u{acd}રીવ\u{acd}નિકા-ક\u{acd}રિઝ\u{ac7}વ\u{acd}કી કાઉન\u{acd}ટી"), ("hi", "कोप\u{94d}रिव\u{94d}निका-क\u{94d}रिझ\u{947}व\u{94d}की काउ\u{902}टी"), ("hr", "Koprivničko-križevačka županija"), ("hu", "Kapronca-Körös megye"), ("id", "Kabupaten Koprivnica-Križevci"), ("it", "regione di Koprivnica e Križevci"), ("kn", "ಕೊಪ\u{ccd}ರ\u{cbf}ನ\u{cbf}ಕಾ-ಕ\u{ccd}ರ\u{cbf}ಜ\u{ccd}ಜ\u{cc6}ವ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "코프리브니차크리제브치 주"), ("lt", "Koprivnicų–Križevcų apskritis"), ("lv", "Koprivnica Križevci župānija"), ("mk", "Копривничко-крижевачка жупанија"), ("mr", "कोप\u{94d}रिविन\u{94d}का-क\u{94d}रीझ\u{947}व\u{94d}हि काउ\u{902}टी"), ("ms", "Koprivnica-Krizevci County"), ("nb", "Koprivnica-Križevci"), ("nl", "Koprivnica-Križevci"), ("no", "Koprivnica-Križevci"), ("pl", "Koprivničko-križevačka županija"), ("pt", "Condado de Koprivnica-Križevci"), ("ro", "Cantonul Koprivnica-Križevci"), ("ru", "Копривницко-Крижевацкая жупания"), ("si", "කොප\u{dca}ර\u{dd2}ව\u{dd2}න\u{dca}ක\u{dcf}-ක\u{dca}\u{200d}ර\u{dd2}සේව\u{dca}ස\u{dd2} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Koprivnicko-križevatská župa"), ("sl", "Koprivničko-križevaška županija"), ("sq", "Zhupania e Koprivnicës dhe Krizhevcit"), ("sr", "Копривничко-крижевачка жупанија"), ("sr_Latn", "Koprivničko-križevačka županija"), ("sv", "Koprivnica-Križevcis län"), ("ta", "கோப\u{bcd}ரிவனிக\u{bcd}க\u{bbe} -க\u{bcd}ரிஸிவசி கவுண\u{bcd}டி"), ("te", "క\u{c4b}ప\u{c4d}ర\u{c3f}వ\u{c3f}న\u{c3f}క\u{c3e}-క\u{c4d}ర\u{c3f}జ\u{c47}వ\u{c4d}చ\u{c40} క\u{c4c}ంట\u{c40}"), ("th", "คอพร\u{e35}ฟน\u{e35}ตซา-คร\u{e35}เชฟต\u{e4c}ซ\u{e35}"), ("tr", "Koprivnica-Križevci"), ("uk", "Копривницько-Крижевецька жупанія"), ("ur", "کوپریونیتسا-کریژیوتسی کاؤنٹی"), ("vi", "Hạt Koprivnica-Krizevci"), ("zh", "科普里夫尼察-克里熱夫齊縣")]),
                        unofficial_name_list: ["Koprivnica-Križevci"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::HR,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.7809992), longitude: Some(16.9936575), max_latitude: Some(46.088761), min_latitude: Some(45.4661945), max_longitude: Some(17.4986747), min_longitude: Some(16.4888635)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيلوفار-بيلوغورا"), ("be", "Белаварска-Білагорская жупанія"), ("bg", "Беловарско-билогорска жупания"), ("bn", "বিলোভ\u{9be}র-বিলোগোর\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Bjelovarsko-bilogorska županija"), ("ca", "Comtat de Bjelovar-Bilogora"), ("ccp", "𑄎𑄬𑄣\u{1112e}𑄞𑄢\u{11134}-𑄝\u{1112d}𑄣\u{1112e}𑄉\u{1112e}𑄢"), ("ceb", "Bjelovarsko-Bilogorska Županija"), ("cs", "Bjelovarsko-bilogorská župa"), ("da", "Bjelovar-Bilogora County"), ("de", "Gespanschaft Bjelovar-Bilogora"), ("el", "κομητεία Μπιέλοβαρ-Μπιλόγκονα"), ("en", "Bjelovar-Bilogora"), ("es", "Bjelovar-Bilogora"), ("et", "Bjelovari-Bilo gora maakond"), ("eu", "Bjelovar-Bilogora eskualdea"), ("fa", "بیلوار-بیلگرا"), ("fi", "Bjelovar-Bilogora"), ("fr", "Comitat de Bjelovar-Bilogora"), ("gu", "બ\u{acd}જ\u{ac7}લોવર-બિલોગોરા કાઉન\u{acd}ટી"), ("hi", "बिएलोवर-बिलोगोरा काउ\u{902}टी"), ("hr", "Bjelovarsko-bilogorska županija"), ("hu", "Belovár-Bilogora megye"), ("id", "Kabupaten Bjelovar-Bilogora"), ("it", "regione di Bjelovar e della Bilogora"), ("ka", "ბელოვარ-ბილოგორა"), ("kn", "ಬ\u{cc6}ಲೋವೊವರ\u{ccd}-ಬ\u{cbf}ಲೋಗೊರಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "벨로바르빌로고라 주"), ("lt", "Bjelovaro–Bilogoros apskritis"), ("lv", "Bjelovaras-Bilogoras župānija"), ("mk", "Бјеловарско-билогорска жупанија"), ("mr", "ब\u{947}\u{902}जोवर- बिलोगोरा काउ\u{902}टी"), ("ms", "Bjelovar-Bilogora County"), ("nb", "Bjelovar-Bilogora"), ("nl", "Bjelovar-Bilogora"), ("no", "Bjelovar-Bilogora"), ("pl", "Bjelovarsko-bilogorska županija"), ("pt", "Condado de Bjelovar-Bilogora"), ("ro", "Cantonul Bjelovar-Bilogora"), ("ru", "Беловарско-Билогорская жупания"), ("si", "බ\u{dca}ජෙලොව\u{dcf}ර\u{dca}-බ\u{dd2}ලොගොර\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Bjelovarsko-bilogorská župa"), ("sl", "Bjelovarsko-bilogorska županija"), ("sq", "Zhupania e Bjellovarit dhe Billogorit"), ("sr", "Бјеловарско-билогорска жупанија"), ("sr_Latn", "Bjelovarsko-bilogorska županija"), ("sv", "Bjelovar-Bilogoras län"), ("ta", "பிஜெலோவர\u{bcd} -பிலோகோர\u{bbe} கவுண\u{bcd}டி"), ("te", "బ\u{c46}జ\u{c46}ల\u{c4b}వర\u{c4d}-బ\u{c3f}ల\u{c4b}గ\u{c4d}ర\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "บ\u{e35}เยลอวาร\u{e4c}-บ\u{e35}ลอกอรา"), ("tr", "Bjelovar-Bilogora"), ("uk", "Беловарсько-Білогорська жупанія"), ("ur", "بیئلووار-بیلوگورا کاؤنٹی"), ("vi", "Bjelovar-Bilogora Hạt"), ("zh", "別洛瓦爾-比洛戈拉縣")]),
                        unofficial_name_list: ["Bjelovar-Bilogora"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::HR,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.31739959999999), longitude: Some(14.8167466), max_latitude: Some(45.67344840000001), min_latitude: Some(44.4406968), max_longitude: Some(15.2352703), min_longitude: Some(14.1078905)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بريموريه-غورسكي كوتار"), ("be", "Прыморска-Горанская жупанія"), ("bg", "Приморско-горанска жупания"), ("bn", "প\u{9cd}রিমোজে-গোরস\u{9cd}কি কোট\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("bs", "Primorsko-goranska županija"), ("ca", "Comtat de Primorje – Gorski Kotar"), ("ccp", "𑄛\u{11133}𑄢\u{1112d}𑄟\u{1112e}𑄢\u{11134}𑄎𑄬-𑄉\u{1112e}𑄢\u{11134}𑄇\u{11128} 𑄇\u{1112e}𑄑𑄢\u{11134}"), ("ceb", "Primorsko-Goranska Županija"), ("cs", "Přímořsko-gorskokotarská župa"), ("da", "Primorje-Gorski Kotar County"), ("de", "Gespanschaft Primorje-Gorski kotar"), ("el", "Πριμόρτζε-Γκόρσκι Κοτάρ"), ("en", "Primorje-Gorski Kotar"), ("es", "Primorje-Gorski Kotar"), ("et", "Primorje-Gora maakond"), ("eu", "Primorje-Gorski Kotar eskualdea"), ("fa", "استان پریموری-گورسکی"), ("fi", "Primorje-Gorski Kotar"), ("fr", "Comitat de Primorje-Gorski Kotar"), ("gu", "પ\u{acd}રિમોર\u{acd}જ\u{ac7}-ગોર\u{acd}સ\u{acd}કી કોટાર કાઉન\u{acd}ટી"), ("he", "פרימוריה-גורה"), ("hi", "प\u{94d}रिमोर\u{94d}य\u{947}-गर\u{94d}स\u{94d}की कोटार काउ\u{902}टी"), ("hr", "Primorsko-goranska županija"), ("hu", "Tengermellék-Hegyvidék megye"), ("hy", "Պրիմորսկո Գորանսկա"), ("id", "Kabupaten Primorje-Gorski Kotar"), ("it", "regione litoraneo-montana"), ("kn", "ಪ\u{ccd}ರ\u{cbf}ಮೊರ\u{ccd}ಜ\u{cc6}-ಗೋರ\u{ccd}ಸ\u{ccd}ಕ\u{cbf} ಕೊಟಾರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "프리모레고르스키코타르 주"), ("lt", "Primorjės–Kalnų Kotaro apskritis"), ("lv", "Primorskas-Goranskas župānija"), ("mk", "Приморско-горска жупанија"), ("mr", "प\u{94d}रमोर\u{947}स-गर\u{94d}स\u{94d}की कोटार काउ\u{902}टी"), ("ms", "Primorje-Gorski Kotar County"), ("nb", "Primorje-Gorski Kotar"), ("nl", "Primorje-Gorski Kotar"), ("no", "Primorje-Gorski Kotar"), ("pl", "Primorsko-goranska županija"), ("pt", "Condado Litoral-Serrano"), ("ro", "Cantonul Primorje-Gorski Kotar"), ("ru", "Приморско-Горанская жупания"), ("si", "ප\u{dca}\u{200d}ර\u{dd2}මොර\u{dca}ජේ-ගොර\u{dca}ස\u{dca}ක\u{dd2} කොට\u{dcf}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Prímorsko-gorskokotarská župa"), ("sl", "Primorsko-goranska županija"), ("sq", "Zhupania bregdetaro-malore"), ("sr", "Приморско-горанска жупанија"), ("sr_Latn", "Primorsko-goranska županija"), ("sv", "Primorje-Gorski kotars län"), ("ta", "பிரிமொர\u{bcd}ஜே -கோர\u{bcd}ஸ\u{bcd}கி கோட\u{bcd}டர\u{bcd} கவுண\u{bcd}டி"), ("te", "ప\u{c4d}ర\u{c48}మ\u{c4b}ర\u{c4d}జ\u{c46}-గ\u{c4b}ర\u{c4d}స\u{c4d}క\u{c40} క\u{c4b}ట\u{c3e}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "พร\u{e34}มอเจ กอร\u{e4c}สก\u{e34} คอตาร\u{e4c}"), ("tr", "Primorje-Gorski Kotar"), ("uk", "Приморсько-Ґоранська жупанія"), ("ur", "پریموریے-گورسکی کوتار کاؤنٹی"), ("vi", "Hạt Primorje-Gorski Kotar"), ("zh", "濱海和山區縣")]),
                        unofficial_name_list: ["Primorje-Gorski Kotar"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::HR,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.6192218), longitude: Some(15.4701608), max_latitude: Some(45.1228954), min_latitude: Some(44.2712303), max_longitude: Some(16.1408169), min_longitude: Some(14.6828545)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليكا سني"), ("bg", "Лишко-сенска жупания"), ("bn", "লিক\u{9be}-সেঞ\u{9cd}জ ক\u{9be}উন\u{9cd}টি"), ("bs", "Ličko-senjska županija"), ("ca", "Comtat de Lika-Senj"), ("ccp", "𑄣\u{11128}𑄇-𑄥𑄚\u{11134}𑄎\u{11128}"), ("ceb", "Ličko-Senjska Županija"), ("cs", "Licko-senjská župa"), ("da", "Lika-Senj County"), ("de", "Gespanschaft Lika-Senj"), ("el", "Λίκα-Σεντζ"), ("en", "Lika-Senj"), ("es", "Lika-Senj"), ("et", "Lika-Senji maakond"), ("eu", "Lika-Senj eskualdea"), ("fa", "شهرستان لیکا-سنی"), ("fi", "Lika-Senj"), ("fr", "Comitat de Lika-Senj"), ("gu", "લિકા-સ\u{ac7}\u{a82}જ કાઉન\u{acd}ટી"), ("he", "מחוז ליקה-סני"), ("hi", "लिका-स\u{947}\u{902}ज काउ\u{902}टी"), ("hr", "Ličko-senjska županija"), ("hu", "Lika-Zengg megye"), ("hy", "Լիչկո Սենսկա"), ("id", "Kabupaten Lika-Senj"), ("it", "regione della Licca e di Segna"), ("kn", "ಲ\u{cbf}ಕಾ-ಸ\u{cc6}ನ\u{ccd}ಜ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "리카센 주"), ("lt", "Likos–Senio apskritis"), ("lv", "Likas-Seņas župānija"), ("mk", "Личко-сењска жупанија"), ("mr", "लिका-स\u{947}नज काउ\u{902}टी"), ("ms", "Lika-Senj County"), ("nb", "Lika-Senj"), ("nl", "Lika-Senj"), ("no", "Lika-Senj"), ("pl", "Ličko-senjska županija"), ("pt", "Condado de Lika-Senj"), ("ro", "Lika-Senj"), ("ru", "Лицко-Сеньская жупания"), ("si", "ල\u{dd2}ක\u{dcf}-සේන\u{dca}ජ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Licko-senjská župa"), ("sl", "Ličko-senjska županija"), ("sq", "Zhupania e Likës dhe Senjit"), ("sr", "Личко-сењска жупанија"), ("sr_Latn", "Ličko-senjska županija"), ("sv", "Lika-Senjs län"), ("ta", "லிக\u{bbe} -செஞ\u{bcd} கவுண\u{bcd}டி"), ("te", "ల\u{c3f}క\u{c3e}-స\u{c46}ంజ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลลา-เซนจ\u{e4c}"), ("tr", "Lika-Senj"), ("uk", "Ліцько-Сенська жупанія"), ("ur", "لیکا-سینی کاؤنٹی"), ("vi", "Hạt Lika-Senj"), ("zh", "利卡-塞尼縣")]),
                        unofficial_name_list: ["Lika-Senj"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::HR,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.6557985), longitude: Some(17.7932472), max_latitude: Some(45.99621200000001), min_latitude: Some(45.4725689), max_longitude: Some(18.0678268), min_longitude: Some(17.1197372)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيروفيتيكا-بودرافينا"), ("be", "Віравіціцка-Падраўская жупанія"), ("bg", "Вировитишко-подравска жупания"), ("bn", "ভিরোভিটিক\u{9be}-পোদ\u{9cd}র\u{9be}ভিন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Virovitičko-podravska županija"), ("ca", "Comtat de Virovitica-Podravina"), ("ccp", "𑄞\u{1112d}𑄢\u{1112e}𑄞\u{11128}𑄑\u{11128}𑄇-𑄛\u{11127}𑄓\u{11133}𑄢𑄞\u{11128}𑄚"), ("ceb", "Virovitičko-Podravska Županija"), ("cs", "Viroviticko-podrávská župa"), ("da", "Virovitica-Podravina County"), ("de", "Gespanschaft Virovitica-Podravina"), ("el", "Βιροβιτίκα-Ποντραβίνα"), ("en", "Virovitica-Podravina"), ("es", "Virovitica-Podravina"), ("et", "Virovitica-Podravina maakond"), ("eu", "Virovitica-Podravina eskualdea"), ("fa", "شهرستان ویروویتیتسا-پودراوینا"), ("fi", "Virovitica-Podravina"), ("fr", "Comitat de Virovitica-Podravina"), ("gu", "વિરોવિટિકા-પોડ\u{acd}રાવીના કાઉન\u{acd}ટી"), ("hi", "विरोविटिका-पॉडराविना काउ\u{902}टी"), ("hr", "Virovitičko-Podravska županija"), ("hu", "Verőce-Drávamente megye"), ("id", "Kabupaten Virovitica-Podravina"), ("it", "regione di Virovitica e della Podravina"), ("kn", "ವ\u{cbf}ವೊರೊಟ\u{cbf}ಕಾ-ಪೊಡ\u{ccd}ರವ\u{cbf}ನ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "비로비티차포드라비나 주"), ("lt", "Viroviticos–Podravinos apskritis"), ("lv", "Viroviticas–Podravinas župānija"), ("mk", "Вировитичко-подравска жупанија"), ("mr", "विरोविटिका-पोड\u{94d}राविना काउ\u{902}टी"), ("ms", "Virovitica-Podravina County"), ("nb", "Virovitica-Podravina"), ("nl", "Virovitica-Podravina"), ("no", "Virovitica-Podravina"), ("pl", "Virovitičko-podravska županija"), ("pt", "Condado de Virovitica-Podravina"), ("ro", "Cantonul Virovitica-Podravina"), ("ru", "Вировитицко-Подравская жупания"), ("si", "ව\u{dd2}රෝව\u{dd2}ට\u{dd2}ක\u{dcf} -පොද\u{dca}\u{200d}රව\u{dd2}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Viroviticko-podrávska župa"), ("sl", "Virovitiško-podravska županija"), ("sq", "Zhupania e Viroviticës dhe Podravkës"), ("sr", "Вировитичко-подравска жупанија"), ("sr_Latn", "Virovitičko-podravska županija"), ("sv", "Virovitica-Podravinas län"), ("ta", "விப\u{bcd}ரோவிடிக\u{bbe} -போட\u{bcd}ற\u{bbe}வின\u{bbe} கவுண\u{bcd}டி"), ("te", "వ\u{c3f}ర\u{c4b}వ\u{c3f}ట\u{c3f}క\u{c3e}-ప\u{c4b}డ\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลว\u{e34}โรว\u{e34}ต\u{e34}ก\u{e49}า-โพดราว\u{e34}นา"), ("tr", "Virovitica-Podravina"), ("uk", "Вировитицько-Подравська жупанія"), ("ur", "ویروسیتیتسا-پودراوینا کاؤنٹی"), ("vi", "Hạt Virovitica-Podravina"), ("zh", "維羅維蒂察-波德拉維納縣")]),
                        unofficial_name_list: ["Virovitica-Podravina"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::HR,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.3417868), longitude: Some(17.8114359), max_latitude: Some(45.5801629), min_latitude: Some(45.1823306), max_longitude: Some(18.1141807), min_longitude: Some(16.9284931)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بوزيغا-سلافونيا"), ("be", "Пожажска-Славонская жупанія"), ("bg", "Пожежко-славонска жупания"), ("bn", "পোজেগ\u{9be}-স\u{9cd}ল\u{9be}ভনিয\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Požeško-slavonska županija"), ("ca", "Comtat de Požega-Eslavònia"), ("ccp", "𑄛\u{11127}𑄎𑄬𑄉-𑄥\u{11133}𑄣𑄞\u{1112e}𑄚\u{11128}𑄠"), ("ceb", "Požeško-Slavonska Županija"), ("cs", "Požežsko-slavonská župa"), ("da", "Požega-Slavonia County"), ("de", "Gespanschaft Požega-Slawonien"), ("el", "Ποζέγκα-Σλαβονία"), ("en", "Požega-Slavonia"), ("es", "Požega-Eslavonia"), ("et", "Požega-Slavoonia maakond"), ("eu", "Požega-Eslavonia eskualdea"), ("fa", "شهرستان پوژگا-اسلاونیا"), ("fi", "Požega-Slavonia"), ("fr", "Comitat de Požega-Slavonie"), ("gu", "પોઝ\u{ac7}ગા-સ\u{acd}લ\u{ac7}વોનિયા કાઉન\u{acd}ટી"), ("hi", "पोज\u{947}गा-स\u{94d}लावोनिया काउ\u{902}टी"), ("hr", "Požeško-slavonska županija"), ("hu", "Pozsega-Szlavónia megye"), ("id", "Kabupaten Požega-Slavonia"), ("it", "regione di Požega e della Slavonia"), ("kn", "ಪೋಜ\u{cc6}ಗಾ-ಸ\u{ccd}ಲಾವೊನ\u{cbf}ಯಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "포제가슬라보니아 주"), ("lt", "Požegos–Slavonijos apskritis"), ("lv", "Požegas-Slavonskas župānija"), ("mk", "Пожешко-славонска жупанија"), ("mr", "पोझ\u{947}गा-स\u{94d}लाव\u{94d}होनिया काउ\u{902}टी"), ("ms", "Pozega-Slavonia County"), ("nb", "Požega-Slavonia"), ("nl", "Požega-Slavonië"), ("no", "Požega-Slavonia"), ("pl", "Požeško-slavonska županija"), ("pt", "Condado de Požega-Eslavônia"), ("ro", "Cantonul Požega-Slavonia"), ("ru", "Пожежско-Славонская жупания"), ("si", "පොසේග\u{dcf} ස\u{dca}ලැවෝන\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Požecko-slavónska župa"), ("sl", "Požeško-slavonska županija"), ("sr", "Пожешко-славонска жупанија"), ("sr_Latn", "Požeško-slavonska županija"), ("sv", "Požega-Slavoniens län"), ("ta", "போஸிக\u{bbe} -ஸ\u{bcd}ல\u{bbe}வோனிய\u{bbe} கவுண\u{bcd}டி"), ("te", "ప\u{c4b}జ\u{c46}గ\u{c3e}-స\u{c4d}ల\u{c3e}వ\u{c4b}న\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "โปเซก\u{e49}า สโลวาเน\u{e35}ย"), ("tr", "Požega-Slavonija"), ("uk", "Пожезько-Славонська жупанія"), ("ur", "پوژیگا-سلاونیا کاؤنٹی"), ("vi", "Hạt Pozega-Slavonia"), ("zh", "波熱加-斯拉沃尼亞縣")]),
                        unofficial_name_list: ["Požega-Slavonia"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::HR,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.2637951), longitude: Some(17.3264562), max_latitude: Some(45.39235980000001), min_latitude: Some(45.0434571), max_longitude: Some(18.5746296), min_longitude: Some(17.0736625)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برود-بوسافينا"), ("be", "Бродска-Посаўская жупанія"), ("bg", "Бродско-посавска жупания"), ("bn", "ব\u{9cd}রড-পোস\u{9be}ভিন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Brodsko-posavska županija"), ("ca", "Comtat de Brod-Posavina"), ("ccp", "𑄝\u{11133}𑄢𑄖\u{11134}-𑄛\u{1112e}𑄥𑄞\u{11128}𑄚"), ("ceb", "Brodsko-Posavska Županija"), ("cs", "Brodsko-posávská župa"), ("da", "Brod-Posavina"), ("de", "Gespanschaft Brod-Posavina"), ("el", "Μπρόντ-Ποσαβίνα"), ("en", "Brod-Posavina"), ("es", "Brod-Posavina"), ("et", "Brodi-Posavina maakond"), ("eu", "Brod-Posavina eskualdea"), ("fa", "برد پساوینا"), ("fi", "Brod-Posavina"), ("fr", "Comitat de Brod-Posavina"), ("gu", "બ\u{acd}રોડ-પોસવિના કાઉન\u{acd}ટી"), ("hi", "ब\u{94d}रोड-पोसाविना काउ\u{902}टी"), ("hr", "Brodsko-posavska županija"), ("hu", "Bród-Szávamente megye"), ("id", "Kabupaten Brod-Posavina"), ("it", "regione di Brod e della Posavina"), ("ka", "ბროდ-პოსავინა"), ("kn", "ಬ\u{ccd}ರಾಡ\u{ccd}-ಪೊಸವ\u{cbf}ನಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "브로드포사비나 주"), ("lt", "Brodo–Posavinos apskritis"), ("lv", "Brodas-Posavinas župānija"), ("mk", "Бродско-посавска жупанија"), ("mr", "ब\u{94d}रॉड-पोसाव\u{94d}हिना काउ\u{902}टी"), ("ms", "Daerah Brod-Posavina"), ("nb", "Brod-Posavina"), ("nl", "Brod-Posavina"), ("no", "Brod-Posavina"), ("pl", "Brodsko-posavska županija"), ("pt", "Condado de Brod-Posavina"), ("ro", "Cantonul Brod-Posavina"), ("ru", "Бродско-Посавская жупания"), ("si", ", බ\u{dca}\u{200d}රොඩ\u{dca} - පොසව\u{dd2}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Brodsko-posávska župa"), ("sl", "Brodsko-posavska županija"), ("sq", "Zhupania e Brodit dhe Posavinës"), ("sr", "Бродско-посавска жупанија"), ("sr_Latn", "Brodsko-posavska županija"), ("sv", "Brod-Posavinas län"), ("ta", "ப\u{bcd}ரோட\u{bcd} -போச\u{bbe}வின\u{bbe} கவுண\u{bcd}டி"), ("te", "బ\u{c4d}ర\u{c3e}డ\u{c4d}-ప\u{c4b}స\u{c3e}వ\u{c3f}న\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลบรอดโพแซว\u{e34}นา"), ("tr", "Brod-Posavina"), ("uk", "Бродсько-Посавська жупанія"), ("ur", "برود-پوساوینا کاؤنٹی"), ("vi", "Hạt Brod-Posavina"), ("zh", "布羅德-波薩維納縣")]),
                        unofficial_name_list: ["Brod-Posavina"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::HR,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.146939), longitude: Some(15.6164943), max_latitude: Some(44.5384474), min_latitude: Some(43.8228067), max_longitude: Some(16.2202414), min_longitude: Some(14.5670193)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زادار"), ("be", "Задарская жупанія"), ("bg", "Задарска жупания"), ("bn", "জ\u{9be}রদ\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("bs", "Zadarska županija"), ("ca", "Comtat de Zadar"), ("ccp", "𑄎𑄓𑄢\u{11134}"), ("ceb", "Zadarska Županija"), ("cs", "Zadarská župa"), ("da", "Zadar County"), ("de", "Gespanschaft Zadar"), ("el", "Ζαντάρ"), ("en", "Zadar"), ("es", "Zadar"), ("et", "Zadari maakond"), ("eu", "Zadar eskualdea"), ("fa", "ایالت زادار"), ("fi", "Zadarin piirikunta"), ("fr", "Comitat de Zadar"), ("gu", "ઝ\u{ac7}ડર કાઉન\u{acd}ટી"), ("he", "מחוז זאדר"), ("hi", "ज\u{93c}दार काउ\u{902}टी"), ("hr", "Zadarska županija"), ("hu", "Zára megye"), ("hy", "Զադար"), ("id", "Kabupaten Zadar"), ("it", "regione zaratina"), ("ja", "ザダル郡"), ("ka", "ზადარის ოლქი"), ("kn", "ಝದರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "자다르 주"), ("lt", "Zadaro apskritis"), ("lv", "Zadaras župānija"), ("mk", "Задарска жупанија"), ("mr", "झदर काउ\u{902}टी"), ("ms", "Zadar County"), ("nb", "Zadar"), ("nl", "Zadar"), ("no", "Zadar"), ("pl", "Zadarska županija"), ("pt", "Condado de Zadar"), ("ro", "Cantonul Zadar"), ("ru", "Задарская жупания"), ("si", "සඩ\u{dcf}ර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Zadarská župa"), ("sl", "Zadarska županija"), ("sq", "Zhupania e Zarës"), ("sr", "Задарска жупанија"), ("sr_Latn", "Zadarska županija"), ("sv", "Zadars län"), ("ta", "சட\u{bbe}ர\u{bcd} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}దర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ซาดาร\u{e4c}"), ("tr", "Zadar Bölgesi"), ("uk", "Задарська жупанія"), ("ur", "زدار کاؤنٹی"), ("vi", "Hạt Zadar"), ("yue", "扎達爾縣"), ("yue_Hans", "扎达尔县"), ("zh", "扎達爾縣")]),
                        unofficial_name_list: ["Zadar"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::HR,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.5576428), longitude: Some(18.3942141), max_latitude: Some(45.9217976), min_latitude: Some(45.2042832), max_longitude: Some(19.1025689), min_longitude: Some(17.8840624)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أوسييك-بارانيا"), ("be", "Осіецка-Бараньская жупанія"), ("bg", "Осиешко-баранска жупания"), ("bn", "ওসিজেক-ব\u{9be}র\u{9be}ঞ\u{9cd}জ\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Osječko-baranjska županija"), ("ca", "Comtat d’Osijek-Baranja"), ("ccp", "𑄃\u{11127}𑄥\u{11128}𑄎𑄬𑄇\u{11134}-𑄝𑄢𑄚\u{11134}𑄎"), ("ceb", "Osječko-Baranjska Županija"), ("cs", "Osijecko-baranjská župa"), ("da", "Osijek-Baranja County"), ("de", "Gespanschaft Osijek-Baranja"), ("el", "Οσιτζέκ-Μπαράντζα"), ("en", "Osijek-Baranja"), ("es", "Osijek-Baranja"), ("et", "Osijeki-Baranja maakond"), ("eu", "Osijek-Baranja eskualdea"), ("fa", "ایالت اوسییک-بارانجا"), ("fi", "Osijek-Baranja"), ("fr", "Comitat d’Osijek-Baranja"), ("gu", "ઓસ\u{acd}જ\u{ac7}ક-બાર\u{a82}જા કાઉન\u{acd}ટી"), ("hi", "ओसिएक-बरान\u{94d}खा काउ\u{902}टी"), ("hr", "Osječko-baranjska županija"), ("hu", "Eszék-Baranya megye"), ("id", "Kabupaten Osijek-Baranja"), ("it", "regione di Osijek e della Baranja"), ("kn", "ಒಸ\u{cbf}ಜ\u{cc6}ಕ\u{ccd}-ಬರಾನ\u{ccd}ಜಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "오시예크바라냐 주"), ("lt", "Osijeko–Baranijos apskritis"), ("lv", "Osijekas-Baranjas župānija"), ("mk", "Осиечко-барањска жупанија"), ("mr", "ओस\u{94d}ज\u{947}क-बारानजा काउ\u{902}टी"), ("ms", "Osijek-Baranja County"), ("nb", "Osijek-Baranja"), ("nl", "Osijek-Baranja"), ("no", "Osijek-Baranja"), ("pl", "Osječko-baranjska županija"), ("pt", "Condado de Osijek-Barânia"), ("ro", "Cantonul Osijek-Baranja"), ("ru", "Осиецко-Бараньская жупания"), ("si", "ඔස\u{dd2}ජෙක\u{dca}-බරන\u{dca}ජ\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Osijecko-baranjská župa"), ("sl", "Osiješko-baranjska županija"), ("sq", "Zhupania e Osijekut dhe Baranjës"), ("sr", "Осјечко-барањска жупанија"), ("sr_Latn", "Osječko-baranjska županija"), ("sv", "Osijek-Baranjas län"), ("ta", "ஓசிஜேக\u{bcd} -பரஞ\u{bcd}ச\u{bbe} கவுண\u{bcd}டி"), ("te", "ఓస\u{c3f}జ\u{c46}క\u{c4d}-బర\u{c3e}ంజ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องโอซ\u{e34}เจค-บารานจา"), ("tr", "Osijek-Baranya"), ("uk", "Осієцько-Баранська жупанія"), ("ur", "اوسیئک-بارانیا کاؤنٹی"), ("vi", "Hạt Osijek-Baranja"), ("zh", "奧西耶克-巴拉尼亞縣")]),
                        unofficial_name_list: ["Osijek-Baranja"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::HR,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.9281485), longitude: Some(16.1037694), max_latitude: Some(44.2164979), min_latitude: Some(43.4868297), max_longitude: Some(16.5426211), min_longitude: Some(15.2068282)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شيبينيك-كنين"), ("be", "Шыбенска-Кнінская жупанія"), ("bg", "Шибенишко-книнска жупания"), ("bn", "সিবেনিক-নিন ক\u{9be}উন\u{9cd}টি"), ("bs", "Šibensko-kninska županija"), ("ca", "Comtat de Šibenik-Knin"), ("ccp", "𑄥\u{11128}𑄝𑄬𑄚\u{11128}𑄇\u{11134}-𑄚\u{11128}𑄚\u{11134}"), ("ceb", "Šibensko-Kninska Županija"), ("cs", "Šibenicko-kninská župa"), ("da", "Šibenik-Knin County"), ("de", "Gespanschaft Šibenik-Knin"), ("el", "Σιμπένικ-Κνιν"), ("en", "Šibenik-Knin"), ("es", "Šibenik-Knin"), ("et", "Šibeniki-Knini maakond"), ("eu", "Šibenik-Knin eskualdea"), ("fa", "شهرستان شیبنیک-کنین"), ("fi", "Šibenik-Knin"), ("fr", "Comitat de Šibenik-Knin"), ("gu", "સિબ\u{ac7}નીક-નિન કાઉન\u{acd}ટી"), ("hi", "सिबिनिक-निन काउ\u{902}टी"), ("hr", "Šibensko-kninska županija"), ("hu", "Šibenik-Knin megye"), ("hy", "Շիբենիկ Կնին"), ("id", "Kabupaten Šibenik-Knin"), ("it", "regione di Sebenico e Tenin"), ("kn", "ಸ\u{cbf}ಬ\u{cc6}ನ\u{cbf}ಕ\u{ccd}-ನ\u{cbf}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "시베니크크닌 주"), ("lt", "Šibeniko–Knino apskritis"), ("lv", "Šibenikas-Kninas županija"), ("mk", "Шибенско-книнска жупанија"), ("mr", "सिबिनिक-निन काउ\u{902}टी"), ("ms", "Sibenik-Knin County"), ("nb", "Šibenik-Knin"), ("nl", "Šibenik-Knin"), ("no", "Šibenik-Knin"), ("pl", "Šibensko-kninska županija"), ("pt", "Condado de Šibenik-Knin"), ("ro", "Cantonul Šibenik-Knin"), ("ru", "Шибенско-Книнская жупания"), ("si", "ස\u{dd2}බෙන\u{dd2}ක\u{dca}-ක\u{dca}න\u{dd2}න\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Šibenicko-kninská župa"), ("sl", "Šibensko-kninska županija"), ("sq", "Zhupania e Kninit dhe Shibenikut"), ("sr", "Шибенско-книнска жупанија"), ("sr_Latn", "Šibensko-kninska županija"), ("sv", "Šibenik-Knins län"), ("ta", "சிபேனிக\u{bcd} -க\u{bbe}னின\u{bcd} கவுண\u{bcd}டி"), ("te", "స\u{c3f}బ\u{c46}న\u{c3f}క\u{c4d}-క\u{c4d}న\u{c3f}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมลฑลช\u{e35}เบน\u{e34}ค-คน\u{e35}น"), ("tr", "Šibenik-Knin"), ("uk", "Шибеницько-Кнінська жупанія"), ("ur", "شیبینک-کنین کاؤنٹی"), ("vi", "Hạt Sibenik-Knin"), ("zh", "希貝尼克-克寧縣")]),
                        unofficial_name_list: ["Šibenik-Knin"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::HR,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1773552), longitude: Some(18.8053527), max_latitude: Some(45.4856329), min_latitude: Some(44.8497502), max_longitude: Some(19.4480523), min_longitude: Some(18.4948404)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فوكوفار-سيرميا"), ("be", "Вукаварска-Срэмская жупанія"), ("bg", "Вуковарско-сремска жупания"), ("bn", "ভ\u{9c1}কোভ\u{9be}র-সিরমিয\u{9bc}\u{9be}"), ("bs", "Vukovarsko-srijemska županija"), ("ca", "Comtat de Vukovar-Srijem"), ("ccp", "𑄞\u{1112a}𑄇\u{1112e}𑄞𑄢\u{11134}-𑄥\u{11128}𑄢\u{11134}𑄟\u{11128}𑄠"), ("ceb", "Vukovarsko-Srijemska Županija"), ("cs", "Vukovarsko-sremská župa"), ("da", "Vukovar-Syrmia County"), ("de", "Gespanschaft Vukovar-Srijem"), ("el", "Βούκοβαρ-Σρίτζεμ"), ("en", "Vukovar-Syrmia"), ("es", "Vukovar-Srijem"), ("et", "Vukovari-Srijemi maakond"), ("eu", "Vukovar-Syrmia eskualdea"), ("fa", "شهرستان ووکوار اسریجم"), ("fi", "Vukovar-Syrmia"), ("fr", "Comitat de Vukovar-Syrmie"), ("gu", "વકોવર-સિરમિઆ કાઉન\u{acd}ટી"), ("hi", "वोकोवर-सीरमिया काउ\u{902}टी"), ("hr", "Vukovarsko-srijemska županija"), ("hu", "Vukovár-Szerém megye"), ("id", "Kabupaten Vukovar-Syrmia"), ("it", "regione di Vukovar e della Sirmia"), ("kn", "ವುಕೋವರ\u{ccd} -ಸ\u{cbf}ರ\u{ccd}ಮ\u{cbf}ಯಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "부코바르스리옘 주"), ("lt", "Vukovaro–Srijemo apskritis"), ("lv", "Vukovaras-Srijemas župānija"), ("mk", "Вуковарско-сремска жупанија"), ("mr", "वक\u{94d}व\u{947}र-सिरिमीया काउ\u{902}टी"), ("ms", "Vukovar-Syrmia County"), ("nb", "Vukovar-Syrmia"), ("nl", "Vukovar-Srijem"), ("no", "Vukovar-Syrmia"), ("pl", "Vukovarsko-srijemska županija"), ("pt", "Condado de Vukovar-Sírmia"), ("ro", "Cantonul Vukovar-Srijem"), ("ru", "Вуковарско-Сремская жупания"), ("si", "ව\u{dd4}කොවර\u{dca}-ස\u{dd2}ර\u{dca}ම\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vukovarsko-sriemska župa"), ("sl", "Vukovarsko-srijemska županija"), ("sq", "Zhupania e Vukovarit dhe Sremit"), ("sr", "Вуковарско-сријемска жупанија"), ("sr_Latn", "Vukovarsko-srijemska županija"), ("sv", "Vukovar-Srijems län"), ("ta", "வுக\u{bcd}கோவைர\u{bcd} -சிறுமிய\u{bbe} கவுண\u{bcd}டி"), ("te", "వుక\u{c4b}వర\u{c4d}-స\u{c3f}మ\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "ว\u{e39}โควาร\u{e4c}"), ("tr", "Vukovar-Syrmia"), ("uk", "Вуковарсько-Сремська жупанія"), ("ur", "ووکووار-سریئیم کاؤنٹی"), ("vi", "Vukovar-Syrmia Hạt"), ("zh", "武科瓦爾-斯里耶姆縣")]),
                        unofficial_name_list: ["Vukovar-Sirmium"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::HR,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.5240328), longitude: Some(16.8178377), max_latitude: Some(43.9736159), min_latitude: Some(42.38713629999999), max_longitude: Some(17.4510685), min_longitude: Some(15.709013)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سبليت-دالماسيا"), ("be", "Спліцка-Далмацінская жупанія"), ("bg", "Сплитско-далматинска жупания"), ("bn", "স\u{9cd}প\u{9cd}লিত-দ\u{9be}ল\u{9cd}মেশিয\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Splitsko-dalmatinska županija"), ("ca", "Comtat de Split-Dalmàcia"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄛\u{11133}𑄣\u{11128}𑄖\u{11134}-𑄓𑄣\u{11134}𑄟𑄑\u{11128}𑄠"), ("ceb", "Splitsko-Dalmatinska Županija"), ("cs", "Splitsko-dalmatská župa"), ("da", "Split-Dalmatia County"), ("de", "Gespanschaft Split-Dalmatien"), ("el", "Σπλιτ-Νταλματία"), ("en", "Split-Dalmatia"), ("es", "Split-Dalmacia"), ("et", "Spliti-Dalmaatsia maakond"), ("eu", "Split-Dalmazia eskualdea"), ("fa", "شهرستان اسپلیت-دالماسی"), ("fi", "Split-Dalmatia"), ("fr", "Comitat de Split-Dalmatie"), ("gu", "સ\u{acd}પ\u{acd}લિટ-ડલ\u{acd}માટિયા કાઉન\u{acd}ટી"), ("hi", "स\u{94d}प\u{94d}लिट-डालम\u{947}शिया काउ\u{902}टी"), ("hr", "Splitsko-dalmatinska županija"), ("hu", "Split-Dalmácia megye"), ("hy", "Սպլիտսկո Դալմատինսկա"), ("id", "Kabupaten Split-Dalmatia"), ("it", "regione spalatino-dalmata"), ("kn", "ಸ\u{ccd}ಪ\u{ccd}ಲ\u{cbf}ಟ\u{ccd}-ಡಾಲ\u{ccd}ಮಾಟ\u{cbf}ಯಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "스플리트달마티아 주"), ("lt", "Splito–Dalmatijos apskritis"), ("lv", "Splitas-Dalmācijas župānija"), ("mk", "Сплитско-далматинска жупанија"), ("mr", "स\u{94d}प\u{94d}लिट-डल\u{94d}म\u{945}टिया काउ\u{902}टी"), ("ms", "Split-Dalmatia County"), ("nb", "Split-Dalmatia"), ("nl", "Split-Dalmatië"), ("no", "Split-Dalmatia"), ("pl", "Splitsko-dalmatinska županija"), ("pt", "Condado de Split-Dalmácia"), ("ro", "Cantonul Split-Dalmația"), ("ru", "Сплитско-Далматинская жупания"), ("si", "ස\u{dca}ප\u{dca}ල\u{dd2}ට\u{dca} ඩල\u{dca}මට\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Splitsko-dalmatínska župa"), ("sl", "Splitsko-dalmatinska županija"), ("sq", "Zhupania e Splitit dhe e Dalmacisë"), ("sr", "Сплитско-далматинска жупанија"), ("sr_Latn", "Splitsko-dalmatinska županija"), ("sv", "Split-Dalmatiens län"), ("ta", "ஸ\u{bcd}ப\u{bcd}ளிட\u{bcd} -ட\u{bbe}ல\u{bcd}மதிஆ கவுண\u{bcd}டி"), ("te", "స\u{c4d}ప\u{c4d}ల\u{c3f}ట\u{c4d}-డ\u{c3e}ల\u{c4d}మ\u{c3e}ట\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "นอร\u{e4c}ทอ\u{e35}ส ด\u{e34}สทร\u{e34}ค"), ("tr", "Split-Dalmaçya"), ("uk", "Сплітсько-Далматинська жупанія"), ("ur", "سپلیت-دالماتیا کاؤنٹی"), ("vi", "Hạt Split-Dalmatia"), ("zh", "斯普利特-達爾馬提亞縣")]),
                        unofficial_name_list: ["Split-Dalmatia"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::HR,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.1286455), longitude: Some(13.901542), max_latitude: Some(45.5196036), min_latitude: Some(44.75755170000001), max_longitude: Some(14.2285896), min_longitude: Some(13.4896865)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة إستريا"), ("bg", "Истрийска жупания"), ("bn", "ইস\u{9cd}ত\u{9cd}রিয\u{9bc}\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Istarska županija"), ("ca", "Comtat d’Ístria"), ("ccp", "𑄃\u{11128}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Istarska Županija"), ("cs", "Istrijská župa"), ("da", "Istria County"), ("de", "Gespanschaft Istrien"), ("el", "Ίστρια"), ("en", "Istria"), ("es", "Istria"), ("et", "Istra maakond"), ("eu", "Istria eskualdea"), ("fa", "ایستریا"), ("fi", "Istrian piirikunta"), ("fr", "Comitat d’Istrie"), ("gu", "આઇસ\u{acd}ટ\u{acd}રિયા કાઉન\u{acd}ટી"), ("hi", "इस\u{94d}ट\u{94d}रिया काउ\u{902}टी"), ("hr", "Istarska županija"), ("hu", "Isztria megye"), ("hy", "Իստրիա"), ("id", "Kabupaten Istria"), ("it", "regione istriana"), ("ja", "イストラ郡"), ("ka", "ისტრია"), ("kn", "ಇಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಯಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "이스트라 주"), ("lt", "Istrijos apskritis"), ("lv", "Istrijas župānija"), ("mk", "Истарска жупанија"), ("mr", "इस\u{94d}रीया काउ\u{902}टी"), ("ms", "Istria County"), ("nb", "Istria"), ("nl", "Istrië"), ("no", "Istria"), ("pl", "Istarska županija"), ("pt", "Condado de Ístria"), ("ro", "Cantonul Istria"), ("ru", "Истрийская жупания"), ("si", "ඉස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ය\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Istrijská župa"), ("sl", "Istrska županija"), ("sq", "Zhupania e Istrës"), ("sr", "Истарска жупанија"), ("sr_Latn", "Istarska županija"), ("sv", "Istriens län"), ("ta", "இஸ\u{bcd}திரிய\u{bbe} கவுண\u{bcd}டி"), ("te", "ఇస\u{c4d}ట\u{c4d}ర\u{c3f}య\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลอ\u{e34}สเตร\u{e35}ย"), ("tr", "İstriya"), ("uk", "Істрійська жупанія"), ("ur", "استریا کاؤنٹی"), ("vi", "Hạt Istria"), ("zh", "伊斯特拉縣")]),
                        unofficial_name_list: ["Istria"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::HR,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.0766588), longitude: Some(17.5268471), max_latitude: Some(43.17750849999999), min_latitude: Some(42.3922402), max_longitude: Some(18.5337474), min_longitude: Some(16.4876151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوبروفنيك-نيريتفا"), ("be", "Дуброўніцка-Нерэтванская жупанія"), ("bg", "Дубровнишко-неретванска жупания"), ("bn", "দ\u{9c1}ব\u{9cd}রভনিক-নেরেতভ\u{9be} ক\u{9be}উন\u{9cd}টি"), ("bs", "Dubrovačko-neretvanska županija"), ("ca", "Comtat de Dubrovnik-Neretva"), ("ccp", "𑄓\u{1112a}𑄝\u{11133}𑄢\u{1112e}𑄛\u{11134}𑄚\u{11128}𑄇\u{11134}-𑄚𑄬𑄢𑄬𑄖\u{11134}𑄞"), ("ceb", "Dubrovačko-Neretvanska Županija"), ("cs", "Dubrovnicko-neretvanská župa"), ("da", "Dubrovnik-Neretva County"), ("de", "Gespanschaft Dubrovnik-Neretva"), ("el", "Ντουμπρόβνικ-Νερέτβα"), ("en", "Dubrovnik-Neretva"), ("es", "Dubrovnik-Neretva"), ("et", "Dubrovniki-Neretva maakond"), ("eu", "Dubrovnik-Neretva eskualdea"), ("fa", "دوبروونیک نرتوا"), ("fi", "Dubrovnik-Neretva"), ("fr", "Comitat de Dubrovnik-Neretva"), ("gu", "ડ\u{ac1}બ\u{acd}રૉવૉનિક-ન\u{ac7}ર\u{ac7}ટ\u{acd}વા કાઉન\u{acd}ટી"), ("he", "מחוז דוברובניק-נרטבה"), ("hi", "ड\u{942}ब\u{94d}रोव\u{94d}निक-न\u{947}र\u{947}ट\u{94d}वा काउ\u{902}टी"), ("hr", "Dubrovačko-neretvanska županija"), ("hu", "Dubrovnik-Neretva megye"), ("hy", "Դուբրովնիկ Ներետվա"), ("id", "Kabupaten Dubrovnik-Neretva"), ("it", "regione raguseo-narentana"), ("ka", "დუბროვნიკ-ნერეტვა"), ("kn", "ಡುಬ\u{ccd}ರೊವ\u{ccd}ನ\u{cbf}ಕ\u{ccd}-ನ\u{cc6}ರ\u{cc6}ಟ\u{ccd}ವಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "두브로브니크네레트바 주"), ("lt", "Dubrovnikų–Neretvos apskritis"), ("lv", "Dubrovnikas-Neretvas župānija"), ("mk", "Дубровничко-неретванска жупанија"), ("mr", "डबरोवनिक-न\u{947}र\u{947}वा काउ\u{902}टी"), ("ms", "Dubrovnik-Neretva County"), ("nb", "Dubrovnik-Neretva"), ("nl", "Dubrovnik-Neretva"), ("no", "Dubrovnik-Neretva"), ("pl", "Dubrovačko-neretvanska županija"), ("pt", "Dubrovnik-Neretva"), ("ro", "Cantonul Dubrovnik-Neretva"), ("ru", "Дубровницко-Неретванская жупания"), ("si", "ඩබ\u{dca}රෝව\u{dca}න\u{dd2}ක\u{dca} නෙරෙට\u{dca}ව\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Dubrovnícko-neretvianska župa"), ("sl", "Dubrovniško-neretvanska županija"), ("sq", "Zhupania e Dubrovnikut dhe Neretvës"), ("sr", "Дубровачко-неретванска жупанија"), ("sr_Latn", "Dubrovačko-neretvanska županija"), ("sv", "Dubrovnik-Neretvas län"), ("ta", "துப\u{bcd}ரோவ\u{bcd}னிக\u{bcd} -நேரிட\u{bcd}டவ\u{bbe} கவுண\u{bcd}டி"), ("te", "డుబ\u{c4d}ర\u{c3e}వ\u{c4b}న\u{c3f}క\u{c4d}-న\u{c46}ర\u{c46}ట\u{c4d}వ\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "จ\u{e31}งหว\u{e31}ดช\u{e35}ร\u{e31}ค"), ("tr", "Dubrovnik-Neretva"), ("uk", "Дубровницько-Неретванська жупанія"), ("ur", "دوبروونیک-نیریتوا کاؤنٹی"), ("vi", "Hạt Dubrovnik-Neretva"), ("zh", "杜布羅夫斯克-內雷特瓦縣")]),
                        unofficial_name_list: ["Dubrovnik-Neretva"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::HR,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3766644), longitude: Some(16.4213298), max_latitude: Some(46.5552234), min_latitude: Some(46.2877571), max_longitude: Some(16.8580089), min_longitude: Some(16.2384532)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماجيموريه"), ("bg", "Меджимурска жупания"), ("bn", "মেডিম\u{9c1}রজে ক\u{9be}উন\u{9cd}টি"), ("bs", "Međimurska županija"), ("ca", "Comtat de Međimurje"), ("ccp", "𑄟𑄬𑄓\u{11128}𑄟\u{1112a}𑄢\u{11134}𑄎𑄬"), ("ceb", "Međimurska Županija"), ("cs", "Mezimuřská župa"), ("da", "Međimurje County"), ("de", "Gespanschaft Međimurje"), ("el", "Μεντιμούρτζε"), ("en", "Međimurje"), ("es", "Međimurje"), ("et", "Međimurje maakond"), ("eu", "Međimurje eskualdea"), ("fa", "شهرستان مجیموریه"), ("fi", "Međimurje"), ("fr", "Comitat de Međimurje"), ("gu", "મ\u{ac7}ડીમ\u{ac1}રજ\u{ac7} કાઉન\u{acd}ટી"), ("hi", "म\u{947}डीम\u{941}र\u{94d}ज\u{947} काउ\u{902}टी"), ("hr", "Međimurska županija"), ("hu", "Muraköz megye"), ("id", "Kabupaten Međimurje"), ("it", "regione del Međimurje"), ("ja", "メジムリェ郡"), ("kn", "ಮ\u{cc6}ಡ\u{cbf}ಮ\u{cc2}ರ\u{ccd}ಜ\u{cc6} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "메지무레 주"), ("lt", "Medžimurjės apskritis"), ("lv", "Medžimurjes župānija"), ("mk", "Меѓимурска жупанија"), ("mr", "म\u{947}ग\u{947}म\u{942}र\u{94d}ज\u{947} काउ\u{902}टी"), ("ms", "Medimurje County"), ("nb", "Međimurje"), ("nl", "Međimurje"), ("no", "Međimurje"), ("pl", "Međimurska županija"), ("pt", "Condado de Međimurje"), ("ro", "Cantonul Međimurje"), ("ru", "Меджумурская жупания"), ("si", "මෙඩ\u{dd2}ම\u{dd4}ර\u{dca}ජේ ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Medzimurská župa"), ("sl", "Medžimurska županija"), ("sq", "Zhupania e Megjimurës"), ("sr", "Међимурска жупанија"), ("sr_Latn", "Međimurska županija"), ("sv", "Međimurjes län"), ("ta", "மெடிமிர\u{bcd}ஜே கவுண\u{bcd}டி"), ("te", "మ\u{c46}డ\u{c3f}ముర\u{c4d}జ\u{c46} క\u{c4c}ంట\u{c40}"), ("th", "เมด\u{e34}เมอเจ"), ("tr", "Međimurje"), ("uk", "Меджимурська жупанія"), ("ur", "میجیموریے کاؤنٹی"), ("vi", "Hạt Medimurje"), ("zh", "梅吉穆列縣")]),
                        unofficial_name_list: ["Međimurje"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::HR,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(45.8150108), longitude: Some(15.981919), max_latitude: Some(45.9395392), min_latitude: Some(45.7408527), max_longitude: Some(16.1069727), min_longitude: Some(15.8216904)}),
                        comments: None,
                        subdivision_type: SubdivisionType::City,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Zagreb"), ("am", "ዛግሬብ"), ("ar", "زغرب"), ("az", "Zaqreb"), ("be", "Заграб"), ("bg", "Загреб"), ("bn", "জ\u{9be}গরেব"), ("bs", "Zagreb"), ("ca", "Zagreb"), ("ccp", "𑄎𑄉\u{11133}𑄢𑄬𑄛\u{11134}"), ("ceb", "Zagreb"), ("cs", "Záhřeb"), ("cy", "Zagreb"), ("da", "Zagreb"), ("de", "Zagreb"), ("el", "Ζάγκρεμπ²"), ("en", "Zagreb"), ("es", "Zagreb²"), ("et", "Zagreb"), ("eu", "Zagreb"), ("fa", "زاگرب"), ("fi", "Zagreb²"), ("fr", "Zagreb"), ("ga", "Ságrab"), ("gl", "Zagreb"), ("gu", "જ\u{abc}ાગ\u{acd}ર\u{ac7}બ"), ("he", "זאגרב"), ("hi", "ज\u{93c}ग\u{94d}र\u{947}ब"), ("hr", "Zagreb"), ("hu", "Zágráb"), ("hy", "Զագրեբ"), ("id", "Zagreb"), ("is", "Zagreb"), ("it", "Zagabria"), ("ja", "ザグレブ"), ("jv", "Zagreb"), ("ka", "ზაგრები"), ("kk", "Загреб"), ("kn", "ಝಗ\u{ccd}ರೇಬ\u{ccd}"), ("ko", "자그레브"), ("ky", "Загреб"), ("lt", "Zagrebas"), ("lv", "Zagreba"), ("mk", "Загреб"), ("ml", "സ\u{d3e}ഗ\u{d4d}രെബ\u{d4d}"), ("mn", "Загреб"), ("mr", "झाग\u{94d}र\u{947}ब"), ("ms", "Zagreb"), ("nb", "Zagreb²"), ("ne", "जग\u{94d}रिब"), ("nl", "Zagreb²"), ("no", "Zagreb²"), ("pa", "ਜ\u{a3c}ਾਗਰਬ"), ("pl", "Zagrzeb"), ("ps", "زګريب"), ("pt", "Zagreb"), ("ro", "Zagreb"), ("ru", "Загреб"), ("si", "ස\u{dcf}ග\u{dca}\u{200d}රබ\u{dca}"), ("sk", "Záhreb"), ("sl", "Zagreb"), ("sq", "Zagrebi"), ("sr", "Загреб"), ("sr_Latn", "Zagreb"), ("sv", "Zagreb"), ("sw", "Zagreb"), ("ta", "ச\u{bbe}கிரேப\u{bcd}"), ("te", "జగ\u{c4d}ర\u{c47}బ\u{c4d}"), ("th", "ซาเกร\u{e47}บ²"), ("tk", "Zagreb"), ("tr", "Zagreb"), ("uk", "Загреб"), ("ur", "زگریب"), ("uz", "Zagreb"), ("vi", "Zagreb²"), ("yo", "Zagreb"), ("yo_BJ", "Zagreb"), ("yue", "薩格勒布"), ("yue_Hans", "萨格勒布"), ("zh", "萨格勒布")]),
                        unofficial_name_list: ["City of Zagreb"].to_vec(),
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
#[cfg(feature = "hr")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::HR,
        alpha3: Alpha3::HRV,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 385,
        currency_code: CurrencyCode::HRK,
        gec: Some(GEC::HR),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::CRO),
        iso_long_name: "The Republic of Croatia",
        iso_short_name: "Croatia",
        official_language_list: ["hr"].to_vec(),
        spoken_language_list: ["hr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Croatian"),
        number: "191",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "HR",
        unofficial_name_list: [
            "Croatia",
            "Kroatien",
            "Croatie",
            "Croacia",
            "クロアチア",
            "Kroatië",
            "Croatia (Hrvatska)",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Croatia"),
            ("af", "Kroasië"),
            ("ak", "Croatia"),
            ("am", "ጤስጔቄ።"),
            ("an", "Croatia"),
            ("ar", "كرواتيا"),
            ("as", "ক\u{9cd}ৰোয়েছিয়\u{9be}"),
            ("ay", "Croatia"),
            ("az", "Xırvatıstan"),
            ("ba", "Croatia"),
            ("be", "Харватыя"),
            ("bg", "Хърватия"),
            ("bi", "Croatia"),
            ("bn", "ক\u{9cd}রোয়েশিয়\u{9be}"),
            ("bn_IN", "ক\u{9cd}রোয়েশিয়\u{9be}"),
            ("br", "Kroatia"),
            ("bs", "Hrvatska"),
            ("ca", "Croàcia"),
            ("ce", "Хорвати"),
            ("ch", "Croatia"),
            ("cs", "Chorvatsko"),
            ("cv", "Хорвати"),
            ("cy", "Croatia"),
            ("da", "Kroatien"),
            ("de", "Kroatien"),
            ("dv", "ކ\u{7aa}ރ\u{7ae}އ\u{7ad}ޝ\u{7a8}އ\u{7a7}"),
            ("dz", "ཀ\u{f7c}ར\u{f7c}་ཤ\u{f72}་ཡ།"),
            ("ee", "Croatia"),
            ("el", "Κροατία"),
            ("en", "Croatia"),
            ("eo", "Kroatio"),
            ("es", "Croacia"),
            ("et", "Horvaatia"),
            ("eu", "Kroazia"),
            ("fa", "کرواسی"),
            ("ff", "Korowaasiya"),
            ("fi", "Kroatia"),
            ("fo", "Kroatia"),
            ("fr", "Croatie"),
            ("fy", "Kroaasje"),
            ("ga", "An Chróit"),
            ("gl", "Croacia"),
            ("gn", "Croatia"),
            ("gu", "ક\u{acd}રોએશિયા"),
            ("gv", "Yn Chroit"),
            ("ha", "Kroatiya"),
            ("he", "קרואטיה"),
            ("hi", "क\u{94d}रोएशिया"),
            ("hr", "Hrvatska"),
            ("ht", "Kroasi"),
            ("hu", "Horvátország"),
            ("hy", "Խորվաթիա"),
            ("ia", "Croatia"),
            ("id", "Kroasia"),
            ("io", "Kroatia"),
            ("is", "Króatía"),
            ("it", "Croazia"),
            ("iu", "Croatia"),
            ("ja", "クロアチア"),
            ("ka", "ჰორვატია"),
            ("ki", "Croatia"),
            ("kk", "Хорватия"),
            ("kl", "Croatia"),
            ("km", "ក\u{17d2}រ\u{17bc}អាត"),
            ("kn", "ಕ\u{ccd}ರೊಏಶ\u{cbf}ಯಾ"),
            ("ko", "크로아티아"),
            ("ku", "Hirvatistan"),
            ("kv", "Хорватия"),
            ("kw", "Kroati"),
            ("ky", "Хорватия"),
            ("lo", "ປະເທດກະໂລອາຊ\u{eb5}"),
            ("lt", "Kroatija"),
            ("lv", "Horvātija"),
            ("mi", "Koroātia"),
            ("mk", "Хрватска"),
            ("ml", "ക\u{d4d}രോയേഷ\u{d4d}യ"),
            ("mn", "Хорват"),
            ("mr", "क\u{94d}रोशिया"),
            ("ms", "Kroatia"),
            ("mt", "Kroazja"),
            (
                "my",
                "ခရ\u{102d}\u{102f}အေးရ\u{103e}ားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Kroaitsiya"),
            ("nb", "Kroatia"),
            ("ne", "क\u{94d}रोएशिया"),
            ("nl", "Kroatië"),
            ("nn", "Kroatia"),
            ("nv", "Kwóóʼad Bikéyah"),
            ("oc", "Croàcia"),
            ("or", "କ\u{b4d}ରୋଏଶ\u{b3f}ଆ"),
            ("pa", "ਕਰ\u{a4b}ਟੀਆ"),
            ("pi", "क\u{94d}रोएशिया"),
            ("pl", "Chorwacja"),
            ("ps", "کروواسيا"),
            ("pt", "Croácia"),
            ("pt_BR", "Croácia"),
            ("ro", "Croația"),
            ("ru", "Хорватия"),
            ("rw", "Korowatiya"),
            ("sc", "Croàtzia"),
            ("sd", "Croatia"),
            ("si", "ක\u{dca}\u{200d}රෝශ\u{dd2}ය\u{dcf}"),
            ("sk", "Chorvátsko"),
            ("sl", "Hrvaška"),
            ("so", "Korweeshiya"),
            ("sq", "Kroaci"),
            ("sr", "Хрватска"),
            ("sv", "Kroatien"),
            ("sw", "Croatia"),
            ("ta", "குரோவேசிய\u{bbe}"),
            ("te", "క\u{c4d}ర\u{c4b}ట\u{c3f}య\u{c3e}"),
            ("tg", "Хорватия"),
            ("th", "โครเอเช\u{e35}ย"),
            ("ti", "ክሮኤሽያ"),
            ("tk", "Horwatiýa"),
            ("tl", "Croatia"),
            ("tr", "Hırvatistan"),
            ("tt", "Кроатиа"),
            ("ug", "كىرودىيە"),
            ("uk", "Хорватія"),
            ("ur", "کروشیا"),
            ("uz", "Xorvatiya"),
            ("ve", "Croatia"),
            ("vi", "Cợ-rô-a-ti-a"),
            ("wa", "Crowåceye"),
            ("wo", "Korwaasi"),
            ("xh", "Croatia"),
            ("yo", "Kroatíà"),
            ("zh_CN", "克罗地亚"),
            ("zh_HK", "克羅地亞"),
            ("zh_TW", "克羅埃西亞"),
            ("zu", "IKrowati"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
