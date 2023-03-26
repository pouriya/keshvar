// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Yemen

#[cfg(all(feature = "ye", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::YE;
    pub const ALPHA3: Alpha3 = Alpha3::YEM;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 967;
    pub const CURRENCY_CODE: &str = "YER";
    pub const GEC: Option<GEC> = Some(GEC::YM);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::YEM);
    pub const ISO_SHORT_NAME: &str = "Yemen";
    pub const ISO_LONG_NAME: &str = "The Republic of Yemen";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Yemeni");
    pub const NUMBER: &str = "887";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "YE";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Yemen", "اليمن", "Jemen", "Yémen", "イエメン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Yemen"),
        ("af", "Jemen"),
        ("ak", "Yemen"),
        ("am", "ፘመን"),
        ("an", "Yemen"),
        ("ar", "اليمن"),
        ("as", "য়েমেন"),
        ("ay", "Yemen"),
        ("az", "Yəmən"),
        ("ba", "Yemen"),
        ("be", "Емен"),
        ("bg", "Йемен"),
        ("bi", "Yemen"),
        ("bn", "ইয়েমেন"),
        ("bn_IN", "ইয়েমেন"),
        ("br", "Yemen"),
        ("bs", "Jemen"),
        ("ca", "Iemen"),
        ("ce", "Йемен"),
        ("ch", "Yemen"),
        ("cs", "Jemen"),
        ("cv", "Йемен"),
        ("cy", "Yemen"),
        ("da", "Yemen"),
        ("de", "Jemen"),
        ("dv", "ޔ\u{7a6}މ\u{7a6}ނ\u{7b0}"),
        ("dz", "ཡ\u{f7a}་མ\u{f7a}ན།"),
        ("ee", "Yemen"),
        ("el", "Υεμένη"),
        ("en", "Yemen"),
        ("eo", "Jemeno"),
        ("es", "Yemen"),
        ("et", "Jeemen"),
        ("eu", "Yemen"),
        ("fa", "یمن"),
        ("ff", "Yemen"),
        ("fi", "Jemen"),
        ("fo", "Jemen"),
        ("fr", "Yémen"),
        ("fy", "Jemen"),
        ("ga", "Éimin"),
        ("gl", "Iemen"),
        ("gn", "Yemen"),
        ("gu", "યમન"),
        ("gv", "Yn Yeaman"),
        ("ha", "Yemen"),
        ("he", "תימן"),
        ("hi", "यमन"),
        ("hr", "Jemen"),
        ("ht", "Yemèn"),
        ("hu", "Jemen"),
        ("hy", "Եմեն"),
        ("ia", "Yemen"),
        ("id", "Yaman"),
        ("io", "Yemen"),
        ("is", "Jemen"),
        ("it", "Yemen"),
        ("iu", "Yemen"),
        ("ja", "イエメン"),
        ("ka", "იემენი"),
        ("ki", "Yemen"),
        ("kk", "Йемен"),
        ("kl", "Yemen"),
        ("km", "យេមែន"),
        ("kn", "ಯ\u{cc6}ಮ\u{cc6}ನ\u{ccd}"),
        ("ko", "예멘"),
        ("ku", "Yemen"),
        ("kv", "Йемен"),
        ("kw", "Yemen"),
        ("ky", "Йемен"),
        ("lo", "Yemen"),
        ("lt", "Jemenas"),
        ("lv", "Jemena"),
        ("mi", "Yemen"),
        ("mk", "Јемен"),
        ("ml", "യെമന\u{d4d}\u{200d}"),
        ("mn", "Иемэн"),
        ("mr", "य\u{947}म\u{947}न"),
        ("ms", "Yaman"),
        ("mt", "Jemen"),
        (
            "my",
            "ယ\u{102e}မင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Yemen"),
        ("nb", "Jemen"),
        ("ne", "य\u{947}म\u{947}न"),
        ("nl", "Jemen"),
        ("nn", "Jemen"),
        ("nv", "Shádiʼááhjí Ásáí Dineʼé Bikéyah"),
        ("oc", "Iemèn"),
        ("or", "ୟମନ"),
        ("pa", "ਯਮਨ"),
        ("pi", "यमन"),
        ("pl", "Jemen"),
        ("ps", "یمن"),
        ("pt", "Iémen"),
        ("pt_BR", "Iêmen"),
        ("ro", "Yemen"),
        ("ru", "Йемен"),
        ("rw", "Yemeni"),
        ("sc", "Yemen"),
        ("sd", "يمن"),
        ("si", "යේමනය"),
        ("sk", "Jemen"),
        ("sl", "Jemen"),
        ("so", "Yaman"),
        ("sq", "Jemen"),
        ("sr", "Јемен"),
        ("sv", "Yemen"),
        ("sw", "Yemen"),
        ("ta", "யேமன\u{bcd}"),
        ("te", "య\u{c46}మ\u{c46}న\u{c4d}"),
        ("tg", "Яман"),
        ("th", "เยเมน"),
        ("ti", "የመን"),
        ("tk", "Ýemen"),
        ("tl", "Yemen"),
        ("tr", "Yemen"),
        ("tt", "Йемен"),
        ("ug", "يەمەن"),
        ("uk", "Ємен"),
        ("ur", "یمن"),
        ("uz", "Yaman"),
        ("ve", "Yemen"),
        ("vi", "Y-ê-men"),
        ("wa", "Yemen"),
        ("wo", "Yaman"),
        ("xh", "Yemen"),
        ("yo", "Yemen"),
        ("zh_CN", "也门"),
        ("zh_HK", "也門"),
        ("zh_TW", "葉門"),
        ("zu", "IYemen"),
    ];
    #[cfg(all(feature = "ye", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 15.552727;
        pub const LONGITUDE: f64 = 48.516388;
        pub const MAX_LATITUDE: f64 = 18.9996331;
        pub const MAX_LONGITUDE: f64 = 54.67899999999999;
        pub const MIN_LATITUDE: f64 = 11.7975;
        pub const MIN_LONGITUDE: f64 = 41.70959999999999;
        pub const NORTHEAST_LATITUDE: f64 = 18.9996331;
        pub const NORTHEAST_LONGITUDE: f64 = 54.67899999999999;
        pub const SOUTHWEST_LATITUDE: f64 = 11.7975;
        pub const SOUTHWEST_LONGITUDE: f64 = 41.70959999999999;
    }
}
#[cfg(all(feature = "ye", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 15.552727,
            longitude: 48.516388,
            max_latitude: 18.9996331,
            max_longitude: 54.67899999999999,
            min_latitude: 11.7975,
            min_longitude: 41.70959999999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 18.9996331,
                    longitude: 54.67899999999999,
                },
                southwest: CountryGeoBound {
                    latitude: 11.7975,
                    longitude: 41.70959999999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "ye", feature = "subdivisions"))]
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
                    "AB",
                    Subdivision{
                        name: "AB",
                        country_alpha2: Alpha2::YE,
                        code: "AB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.6343413), longitude: Some(46.0563212), max_latitude: Some(14.303409), min_latitude: Some(12.9191754), max_longitude: Some(47.166517), min_longitude: Some(45.046048)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أبين"), ("az", "Abya mühafazası"), ("bg", "Абян"), ("bn", "আবিয\u{9bc}\u{9be}ন গভর\u{9cd}নোরেট"), ("ca", "Governació d’Abyan"), ("ccp", "𑄃𑄝\u{11128}𑄠𑄚\u{11134}"), ("cs", "Abján"), ("da", "Abyan Governorate"), ("de", "Gouvernement Abyan"), ("el", "Αμπγιάν"), ("en", "Abyan"), ("es", "Gobernación de Abyan"), ("eu", "Abyan gobernantzia"), ("fa", "استان ابین"), ("fi", "Abyan"), ("fr", "Gouvernorat d’Abyan"), ("gu", "અબ\u{acd}યાન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אבין"), ("hi", "अबयन प\u{94d}रान\u{94d}त"), ("hr", "Abyan"), ("hu", "Abjan kormányzóság"), ("id", "Kegubernuran Abyan"), ("it", "governatorato di Abyan"), ("ja", "アビヤン県"), ("ka", "აბიანის მუჰაფაზა"), ("kn", "ಅಬ\u{ccd}ಯಾನ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "아브얀 주"), ("lt", "Abjano gubernija"), ("lv", "Abjānas muhāfaza"), ("mr", "अब\u{94d}यान\u{902} गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Abyan Governorate"), ("nb", "Abyan"), ("nl", "Abyan"), ("no", "Abyan"), ("pl", "Abjan"), ("pt", "Abyan"), ("ro", "Guvernoratul Abyan"), ("ru", "Абьян"), ("si", "අබ\u{dca}යන\u{dca} පළ\u{dcf}ත"), ("sv", "Abyan"), ("sw", "Wilaya ya Abyan"), ("ta", "அபயன\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "అబ\u{c4d}య\u{c3e}న\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}บย\u{e31}น"), ("tr", "Abyan ili"), ("uk", "Абʼян"), ("ur", "محافظہ ابین"), ("vi", "Tỉnh Abyan"), ("zh", "阿比扬省")]),
                        unofficial_name_list: ["Abyan"].to_vec(),
                    }
                ),
                (
                    "AD",
                    Subdivision{
                        name: "AD",
                        country_alpha2: Alpha2::YE,
                        code: "AD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.8257481), longitude: Some(44.7943804), max_latitude: Some(12.92427), min_latitude: Some(12.6697487), max_longitude: Some(45.0821905), min_longitude: Some(44.4078031)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة عدن"), ("az", "Aden mühafazası"), ("bg", "Аден"), ("ca", "Governació d’Adan"), ("ccp", "𑄃𑄓𑄚\u{11134}"), ("de", "Gouvernement Adan"), ("en", "’Adan"), ("es", "Gobernación de ‘Adan"), ("eu", "‘Adan gobernantzia"), ("fa", "استان عدن"), ("fi", "Aden"), ("fr", "Gouvernorat d’Aden"), ("he", "עדן"), ("hi", "अदन प\u{94d}रान\u{94d}त"), ("hr", "Adan"), ("hu", "Áden kormányzóság"), ("id", "Kegubernuran ‘Adan"), ("ja", "アデン県"), ("ka", "ადენის მუჰაფაზა"), ("ko", "아덴 주"), ("nl", "Aden"), ("pl", "Aden"), ("pt", "Áden"), ("ro", "Guvernoratul ‘Adan"), ("ru", "Аден"), ("sv", "Aden"), ("sw", "Wilaya ya Adan"), ("tr", "Aden ili"), ("uk", "Аден"), ("ur", "محافظہ عدن"), ("zh", "亞丁省")]),
                        unofficial_name_list: ["Adan", "Aden", "Aden", "Adén", "ʿAdan"].to_vec(),
                    }
                ),
                (
                    "AM",
                    Subdivision{
                        name: "AM",
                        country_alpha2: Alpha2::YE,
                        code: "AM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.2569214), longitude: Some(43.9436788), max_latitude: Some(16.641641), min_latitude: Some(15.488677), max_longitude: Some(44.3594831), min_longitude: Some(43.524034)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة عمران"), ("az", "Amran mühafazası"), ("bg", "Амран"), ("bn", "আমর\u{9be}ন গভর\u{9cd}নোরেট"), ("ca", "Governació d’Amran"), ("ccp", "𑄃𑄟𑄢𑄚\u{11134}"), ("da", "Amran"), ("el", "Αμράν Γκοβερνοράτε"), ("en", "Amran"), ("es", "Gobernación de ‘Amran"), ("eu", "‘Amran gobernantzia"), ("fa", "استان عمران"), ("fi", "Amran"), ("fr", "Gouvernorat d’Amran"), ("gu", "અમ\u{acd}રાન ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "עמראן"), ("hi", "अमरान प\u{94d}रान\u{94d}त"), ("hr", "Amran"), ("hu", "Amrán kormányzóság"), ("id", "Kegubernuran ‘Amran"), ("ja", "アムラーン県"), ("ka", "ამრანის მუჰაფაზა"), ("kn", "ಅಮ\u{ccd}ರಾನ\u{ccd} ಗವರ\u{ccd}ನೇಟೇಟ\u{ccd}"), ("ko", "암란 주"), ("lt", "Amrano gubernija"), ("lv", "Amrānas muhāfaza"), ("mr", "अम\u{94d}रान गव\u{94d}हनर\u{94d}ट\u{947}ट"), ("ms", "Pentadbiran Amran"), ("nl", "‘Amran"), ("pl", "Amran"), ("pt", "‘Amran"), ("ro", "Guvernoratul ‘Amran"), ("ru", "Амран"), ("si", "අම\u{dca}රන\u{dca} පළ\u{dcf}ත"), ("sv", "Amran"), ("sw", "Wilaya ya Amran"), ("ta", "அம\u{bcd}ர\u{bbe}ன\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "అమ\u{c4d}ర\u{c3e}న\u{c4d} గవర\u{c4d}న\u{c4b}ర\u{c47}ట\u{c4d}"), ("th", "เขตการปกครองแอมร\u{e31}น"), ("tr", "Amran ili"), ("uk", "Амран"), ("ur", "محافظہ عمران"), ("vi", "‘Amrān"), ("zh", "阿姆蘭省")]),
                        unofficial_name_list: ["'Amran"].to_vec(),
                    }
                ),
                (
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::YE,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.3588662), longitude: Some(45.4498065), max_latitude: Some(14.7964711), min_latitude: Some(13.806784), max_longitude: Some(46.0480639), min_longitude: Some(44.58074209999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة البيضاء"), ("bg", "Ал Бейда"), ("bn", "আল-ব\u{9be}য\u{9bc}দ\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació d’Al Bayda"), ("ccp", "𑄃𑄣\u{11134} 𑄝𑄬𑄓"), ("da", "Al Bayda’ Governorate"), ("de", "Gouvernement al-Baida’"), ("el", "Αλ Μπάιντα"), ("en", "Al Bayda"), ("es", "Gobernación de Al Bayda’"), ("eu", "Al Bayda’ gobernantzia"), ("fa", "شهر البیضاء"), ("fi", "Al Bayda"), ("fr", "Gouvernorat d’Al Bayda’"), ("gu", "અલ બાયડા ‘ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-ביידא"), ("hi", "अल-ब\u{948}दा प\u{94d}रान\u{94d}त"), ("hr", "Al Baida’"), ("hu", "Bajdá kormányzóság"), ("id", "Kegubernuran Al-Bayda’"), ("it", "governatorato di al-Bayda’"), ("ja", "バイダー県"), ("ka", "ელ-ბეიდის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಬೇಡಾ ‘ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "바이다 주"), ("lt", "Al Beidos gubernija"), ("lv", "Baidas muhāfaza"), ("mr", "अल ब\u{947}डा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Bayda’ Governorate"), ("nb", "Al Bayda’"), ("nl", "Al Bayda’"), ("no", "Al Bayda’"), ("pl", "Al-Bajda"), ("pt", "Al Bayda’"), ("ro", "Guvernoratul Al Bayda’"), ("ru", "Эль-Бейда"), ("si", "අල\u{dca} බඩ\u{dca}ය\u{dcf} පළ\u{dcf}ත"), ("sv", "Al-Bayda"), ("sw", "Wilaya ya Al Bayda"), ("ta", "அல\u{bcd} பயட\u{bbe}‘ கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} బ\u{c47}డ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลไบย\u{e4c}ดา"), ("tr", "El Beyda ili"), ("uk", "Ель-Бейда"), ("ur", "محافظہ البیضاء"), ("vi", "Tỉnh Al Bayda’"), ("zh", "贝达省")]),
                        unofficial_name_list: ["Al Baida"].to_vec(),
                    }
                ),
                (
                    "DA",
                    Subdivision{
                        name: "DA",
                        country_alpha2: Alpha2::YE,
                        code: "DA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.696667), longitude: Some(44.730833), max_latitude: Some(13.7195688), min_latitude: Some(13.6921764), max_longitude: Some(44.7446966), min_longitude: Some(44.7181321)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الضالع"), ("bg", "Ал Дали"), ("bn", "আদ দ\u{9be}লি গভর\u{9cd}নোরেট"), ("ca", "Governació d’Ad Dali"), ("ccp", "𑄙𑄣\u{11128}"), ("da", "Ad Dali’ Governorate"), ("de", "Gouvernement ad-Dali’"), ("el", "Αντ Νταλί"), ("en", "Dhale"), ("es", "Gobernación de Ad Dali’"), ("eu", "Ad Dali’ gobernantzia"), ("fa", "استان ضالع"), ("fi", "Al Dali"), ("fr", "Gouvernorat d’Ad Dali’"), ("gu", "એડ દાલી ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "א-דאלע"), ("hi", "अद-दाली प\u{94d}रान\u{94d}त"), ("hr", "Ad Dali’"), ("hu", "Dáli kormányzóság"), ("id", "Kegubernuran Ad-Dali’"), ("it", "governatorato di al-Dali’"), ("ja", "ダーリウ県"), ("ka", "ელ-დალის მუჰაფაზა"), ("kn", "ಆಡ\u{ccd} ಡಾಲ\u{cbf} ‘ಗವರ\u{ccd}ನರ\u{ccd}"), ("ko", "달리 주"), ("lt", "Ad Dalio gubernija"), ("lv", "Dālī muhāfaza"), ("mr", "ऍड दाली गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ad Dali’ Governorate"), ("nb", "Ad Dali’"), ("nl", "Ad Dali’"), ("no", "Ad Dali’"), ("pl", "Ad-Dali"), ("pt", "Ad Dali’"), ("ro", "Guvernoratul Ad Dali’"), ("ru", "Ад-Дали"), ("si", "අඩ\u{dca} ඩල\u{dd2} පළ\u{dcf}ත"), ("sv", "Ad-Dali"), ("sw", "Wilaya ya Ad Dali"), ("ta", "அட ட\u{bbe}லி ‘ கோவெர\u{bcd}னோரே"), ("te", "అడ\u{c4d}ర\u{c3e}\u{c4d} డ\u{c3e}ల\u{c3f} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ด ดาล\u{e35} โกเวอโนเรท"), ("tr", "Ed Dali ili"), ("uk", "Ед-Даля"), ("ur", "محافظہ الضالع"), ("vi", "Tỉnh Ad Dali’"), ("zh", "达利省")]),
                        unofficial_name_list: ["Ad¸ D¸ali'"].to_vec(),
                    }
                ),
                (
                    "DH",
                    Subdivision{
                        name: "DH",
                        country_alpha2: Alpha2::YE,
                        code: "DH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.7195344), longitude: Some(44.2479015), max_latitude: Some(14.9834611), min_latitude: Some(14.031811), max_longitude: Some(44.8365489), min_longitude: Some(43.475584)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ذمار"), ("az", "Damar mühafazası"), ("bg", "Дамар"), ("bn", "দ\u{9be}ম\u{9be}র গভর\u{9cd}নোরেট"), ("ca", "Governació de Dhamar"), ("ccp", "𑄙𑄟𑄢\u{11134}"), ("da", "Dhamar Governorate"), ("de", "Gouvernement Dhamar"), ("el", "Νταμάρ"), ("en", "Dhamar"), ("es", "Gobernación de Dhamar"), ("eu", "Dhamar gobernantzia"), ("fa", "استان ذمار"), ("fi", "Dhamar"), ("fr", "Gouvernorat de Dhamar"), ("gu", "ધમર ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "ד׳מאר"), ("hi", "दमार प\u{94d}रान\u{94d}त"), ("hr", "Dhamar"), ("hu", "Dzamár kormányzóság"), ("id", "Kegubernuran Dhamar"), ("it", "governatorato di Dhamar"), ("ja", "ザマール県"), ("ka", "დამარის მუჰაფაზა"), ("kn", "ಧಮರ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "다마르 주"), ("lt", "Damaro gubernija"), ("lv", "Demāras muhāfaza"), ("mr", "धरम गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Dhamar Governorate"), ("nb", "Dhamar"), ("nl", "Dhamar"), ("no", "Dhamar"), ("pl", "Zamar"), ("pt", "Dhamar"), ("ro", "Guvernoratul Dhamar"), ("ru", "Дамар"), ("si", "ධම\u{dcf}ර\u{dca} පළ\u{dcf}ත"), ("sv", "Dhamar"), ("sw", "Wilaya ya Dhamar"), ("ta", "த\u{bbe}மர\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "ఢ\u{c3e}మర\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ดะมาร\u{e4c}"), ("tr", "Zamar ili"), ("uk", "Дамар"), ("ur", "محافظہ ذمار"), ("vi", "Tỉnh Dhamar"), ("zh", "扎玛尔省")]),
                        unofficial_name_list: ["Dhamar", "Dhomar", "Đomar"].to_vec(),
                    }
                ),
                (
                    "HD",
                    Subdivision{
                        name: "HD",
                        country_alpha2: Alpha2::YE,
                        code: "HD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.9304135), longitude: Some(49.36531489999999), max_latitude: Some(19.002331), min_latitude: Some(12.1115582), max_longitude: Some(54.53053990000001), min_longitude: Some(46.2974891)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حضرموت"), ("bg", "Хадрамаут"), ("bn", "হ\u{9be}ধর\u{9be}ম\u{9be}উত গভর\u{9cd}নোরেট"), ("ca", "Governació d’Hadramaut"), ("ccp", "𑄦𑄓\u{11133}𑄢\u{1112d}𑄟\u{11127}𑄅\u{1112a}𑄖\u{11134}"), ("da", "Hadhramaut Governorate"), ("de", "Gouvernement Hadramaut"), ("el", "Χαντχραμάουτ"), ("en", "Hadramaut"), ("es", "Gobernación de Hadramaut"), ("eu", "Hadramaut gobernantzia"), ("fa", "استان حضرموت"), ("fi", "Hadramaut"), ("fr", "Gouvernorat de l’Hadramaout"), ("gu", "હાધ\u{acd}રમૌટ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "חצרמוות"), ("hi", "हदरामौत प\u{94d}रान\u{94d}त"), ("hr", "Hadramaut"), ("hu", "Hadramaut kormányzóság"), ("id", "Kegubernuran Hadhramaut"), ("it", "governatorato di Hadramawt"), ("ja", "ハドラマウト県"), ("ka", "ჰადრამავთის მუჰაფაზა"), ("kn", "ಹಧ\u{ccd}ರಮ\u{ccc}ತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "하드라마우트 주"), ("lt", "Hadramuto gubernija"), ("lv", "Hadramautas muhāfaza"), ("mr", "हधर\u{94d}मउट गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Hadhramaut Governorate"), ("nb", "Guvernementet Hadhramaut"), ("nl", "Hadramaut"), ("no", "Guvernementet Hadhramaut"), ("pl", "Hadramaut"), ("pt", "Hadramaute"), ("ro", "Guvernoratul Hadhramaut"), ("ru", "Хадрамаут"), ("si", "හද\u{dca}රමව\u{dd4}ට\u{dca} පළ\u{dcf}ත"), ("sv", "Guvernementet Hadhramaut"), ("sw", "Wilaya ya Hadhramaut"), ("ta", "அத\u{bcd}ரமௌட\u{bcd} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "హధ\u{c4d}ర\u{c3e}మట\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตกร\u{e35}ทเตอร\u{e4c} อ\u{e31}กกรา"), ("tr", "Hadramut ili"), ("uk", "Хадрамаут"), ("ur", "محافظہ حضرموت"), ("vi", "Tỉnh Hadhramaut"), ("zh", "哈德拉毛省")]),
                        unofficial_name_list: ["Hadramout"].to_vec(),
                    }
                ),
                (
                    "HJ",
                    Subdivision{
                        name: "HJ",
                        country_alpha2: Alpha2::YE,
                        code: "HJ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.1180631), longitude: Some(43.329466), max_latitude: Some(16.6797261), min_latitude: Some(15.463984), max_longitude: Some(43.766581), min_longitude: Some(42.778236)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة حجة"), ("bg", "Хаджа"), ("bn", "হ\u{9be}জ\u{9cd}জ\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació d’Hajjah"), ("ccp", "𑄦𑄎\u{11133}𑄦𑄦\u{11134}"), ("da", "Hajjah Governorate"), ("de", "Gouvernement Haddscha"), ("el", "Χάτζα"), ("en", "Hajjah"), ("es", "Gobernación de Hajjah"), ("eu", "Hajjah gobernantzia"), ("fa", "استان حجه"), ("fi", "Hajjah"), ("fr", "Gouvernorat de Hajjah"), ("gu", "હઝહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "חג׳ה"), ("hi", "हज\u{94d}जाह प\u{94d}रान\u{94d}त"), ("hr", "Hajjah"), ("hu", "Haddzsa kormányzóság"), ("id", "Kegubernuran Hajjah"), ("it", "governatorato di Hajja"), ("ja", "ハッジャ県"), ("ka", "ჰაჯის მუჰაფაზა"), ("kn", "ಹಜ\u{ccd}ಜಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "하자 주"), ("lt", "Hadžos gubernija"), ("lv", "Hadžas muhāfaza"), ("mr", "हजराज गव\u{94d}हनर\u{94d}ट\u{947}ट"), ("ms", "Hajjah Governorate"), ("nb", "Hajjah"), ("nl", "Hajjah"), ("no", "Hajjah"), ("pl", "Hadżdża"), ("pt", "Hajjah"), ("ro", "Guvernoratul Hajjah"), ("ru", "Хадджа"), ("si", "හජ\u{dca}ජ\u{dcf} පළ\u{dcf}ත"), ("sr", "Хаџах"), ("sr_Latn", "Hadžah"), ("sv", "Hajjah"), ("sw", "Wilaya ya Hajjah"), ("ta", "ஹஜ\u{bcd}ஜஹ\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "హజ\u{c4d}జ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เขตฮ\u{e31}จจะห\u{e4c}"), ("tr", "Hacca ili"), ("uk", "Хадджа"), ("ur", "محافظہ حجہ"), ("vi", "Tỉnh Hajjah"), ("zh", "哈杰省")]),
                        unofficial_name_list: ["Hajjah"].to_vec(),
                    }
                ),
                (
                    "HU",
                    Subdivision{
                        name: "HU",
                        country_alpha2: Alpha2::YE,
                        code: "HU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.3053072), longitude: Some(43.0194897), max_latitude: Some(15.9224578), min_latitude: Some(13.6042295), max_longitude: Some(43.777108), min_longitude: Some(41.8160553)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الحديدة"), ("bg", "Ходейда"), ("bn", "আল হ\u{9c1}দ\u{9be}য\u{9bc}দ\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació d’al-Hudaydah"), ("ccp", "𑄃𑄣\u{11134} 𑄦\u{1112a}𑄓𑄬𑄘𑄦\u{11134}"), ("da", "Al Hudaydah Governorate"), ("de", "Gouvernement al-Hudaida"), ("el", "Αλ Χουντάγντια"), ("en", "Al Hudaydah"), ("es", "Gobernación de Al Hudayda"), ("eu", "Al-Hudaida gobernantzia"), ("fa", "استان حدیده"), ("fi", "Hodeida"), ("fr", "Gouvernorat d’Al Hudaydah"), ("gu", "અલ હ\u{ac1}દાયદાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-חודיידה"), ("hi", "अल-ह\u{941}द\u{948}दाह प\u{94d}रान\u{94d}त"), ("hr", "Al Hudaida"), ("hu", "Hudajda kormányzóság"), ("id", "Kegubernuran Al-Hudaydah"), ("it", "governatorato di al-Hudayda"), ("ja", "フダイダ県"), ("ka", "ჰოდეიდის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಹದಾದಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "호데이다 주"), ("lt", "Al Hudaidos gubernija"), ("lv", "Hudeidas muhāfaza"), ("mr", "अल ह\u{941}डायदाह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Hudaydah Governorate"), ("nb", "Al Huaydah"), ("nl", "Al Hudaydah"), ("no", "Al Huaydah"), ("pl", "Al-Hudajda"), ("pt", "Al Hudaydah"), ("ro", "Guvernoratul Al Hudaydah"), ("ru", "Ходейда"), ("si", "අල\u{dca} හ\u{dd4}ඩේඩ\u{dcf} පළ\u{dcf}ත"), ("sr", "Ел Худаида"), ("sr_Latn", "El Hudaida"), ("sv", "Al-Hudaydah"), ("sw", "Wilaya ya Al Hudaydah"), ("ta", "அல\u{bcd} ஹுடையத கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} హుద\u{c3e}య\u{c4d}\u{200c}ద\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ล ฮ\u{e39}ไมย\u{e4c}ดะฮ\u{e4c}"), ("tr", "El Hudeyde ili"), ("uk", "Ходейда"), ("ur", "محافظہ الحدیدہ"), ("vi", "Tỉnh Al Hudaydah"), ("zh", "荷台达省")]),
                        unofficial_name_list: ["Hodeida", "Hodeidah", "Hodeïah", "H\u{328}udaydah", "al-Hudaydah"].to_vec(),
                    }
                ),
                (
                    "IB",
                    Subdivision{
                        name: "IB",
                        country_alpha2: Alpha2::YE,
                        code: "IB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.1415717), longitude: Some(44.2479015), max_latitude: Some(14.433573), min_latitude: Some(13.674915), max_longitude: Some(44.6729879), min_longitude: Some(43.647663)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة إب"), ("az", "İbb mühafazası"), ("bg", "Иб"), ("bn", "ইব\u{9cd}ব গভর\u{9cd}নোরেট"), ("ca", "Governació d’Ibb"), ("ccp", "𑄃\u{11128}𑄛\u{11133}𑄦\u{11134}"), ("ceb", "Ibb (lalawigan)"), ("da", "Ibb"), ("de", "Gouvernement Ibb"), ("el", "Ιμπ"), ("en", "Ibb"), ("es", "Gobernación de Ibb"), ("eu", "Ibb gobernantzia"), ("fa", "استان اب"), ("fi", "Ibb"), ("fr", "Gouvernorat d’Ibb"), ("gu", "ઈબ\u{acd}બ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "איב"), ("hi", "इब\u{94d}ब प\u{94d}रान\u{94d}त"), ("hr", "Ibb"), ("hu", "Ibb kormányzóság"), ("id", "Kegubernuran Ibb"), ("it", "governatorato di Ibb"), ("ja", "イッブ県"), ("ka", "იბის მუჰაფაზა"), ("kn", "ಇಬ\u{ccd}ಬ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "이브 주"), ("lt", "Ibo gubernija"), ("lv", "Ibas muhāfaza"), ("mr", "इब\u{94d}ब गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ibb Governorate"), ("nb", "Ibb"), ("nl", "Ibb"), ("no", "Ibb"), ("pl", "Ibb"), ("pt", "Ibb"), ("ro", "Guvernoratul Ibb"), ("ru", "Ибб"), ("si", "ඉබ\u{dca} පළ\u{dcf}ත"), ("sv", "Ibb"), ("sw", "Wilaya ya Ibb"), ("ta", "இபப கோவெர\u{bcd}னோரே"), ("te", "ఇబ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e34}บบ\u{e4c}"), ("tr", "İb ili"), ("uk", "Ібб"), ("ur", "محافظہ اب"), ("vi", "Tỉnh Ibb"), ("zh", "伊卜省")]),
                        unofficial_name_list: ["Ibb"].to_vec(),
                    }
                ),
                (
                    "JA",
                    Subdivision{
                        name: "JA",
                        country_alpha2: Alpha2::YE,
                        code: "JA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.7901819), longitude: Some(45.29938620000001), max_latitude: Some(17.4041709), min_latitude: Some(15.722451), max_longitude: Some(47.0087311), min_longitude: Some(44.067691)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة الجوف"), ("be", "Эль-Джауф"), ("bg", "Джауф"), ("bn", "আল জ\u{9be}ওয\u{9bc}\u{9be}ফ গভর\u{9cd}নোরেট"), ("ca", "Governació d’Al Jawf"), ("ccp", "𑄃𑄣\u{11134} 𑄎\u{1112e}𑄛\u{11134}"), ("da", "Al Jawf Governorate"), ("de", "Gouvernement al-Dschauf"), ("el", "Αλ Τζόφ"), ("en", "Al Jawf"), ("es", "Gobernación de Yauf"), ("eu", "Al Jawf gobernantzia"), ("fa", "استان جوف"), ("fi", "Al Jawf"), ("fr", "Gouvernorat d’Al Jawf"), ("gu", "અલ જૌફ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-ג׳וף"), ("hi", "अल-जौफ\u{93c} प\u{94d}रान\u{94d}त"), ("hr", "Al Jawf"), ("hu", "Dzsauf kormányzóság"), ("id", "Kegubernuran Al-Jawf"), ("it", "governatorato di al-Jawf"), ("ja", "ジャウフ県"), ("ka", "ელ-ჯაუფის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಜಾವ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "자우프 주"), ("lt", "Al Džafo gubernija"), ("lv", "Džaufas muhāfaza"), ("mr", "अल जौफ गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Jawf Governorate"), ("nb", "Al Jawf"), ("nl", "Al Jawf"), ("no", "Al Jawf"), ("pl", "Al-Dżauf"), ("pt", "Al Jawf"), ("ro", "Guvernoratul Al Jawf"), ("ru", "Эль-Джауф"), ("si", "අල\u{dca} ජෝෆ\u{dca} පළ\u{dcf}ත"), ("sr", "Ел Џауф"), ("sr_Latn", "El Džauf"), ("sv", "Al-Jawf"), ("sw", "Wilaya ya Al Jawf"), ("ta", "அல\u{bcd} ஜ\u{bbe}வ\u{bcd}ப\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "అల\u{c4d} జ\u{c3e}\u{c3e}ఫ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ลเจาฟ\u{e4c}"), ("tr", "El Cavf ili"), ("uk", "Ель-Джауф"), ("ur", "محافظہ الجوف"), ("vi", "Tỉnh Al Jawf"), ("zh", "焦夫省")]),
                        unofficial_name_list: ["Al Jawf"].to_vec(),
                    }
                ),
                (
                    "LA",
                    Subdivision{
                        name: "LA",
                        country_alpha2: Alpha2::YE,
                        code: "LA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.4022222), longitude: Some(44.3238889), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لحج"), ("bg", "Лахидж"), ("ca", "Governació de Lahij"), ("ccp", "𑄣𑄝𑄦\u{11128}𑄌\u{11134}"), ("de", "Gouvernement Lahidsch"), ("en", "Lahij"), ("es", "Gobernación de Lahij"), ("eu", "Lahij gobernantzia"), ("fa", "استان لحج"), ("fi", "Lahij"), ("fr", "Gouvernorat de Lahij"), ("he", "לחג׳"), ("hi", "लहिज प\u{94d}रान\u{94d}त"), ("hr", "Lahij"), ("hu", "Lahidzs kormányzóság"), ("id", "Kegubernuran Lahij"), ("it", "governatorato di Lahij"), ("ja", "ラヒジュ県"), ("ka", "ლაჰჯის მუჰაფაზა"), ("ko", "라히즈 주"), ("nb", "Lahij"), ("nl", "Lahij"), ("no", "Lahij"), ("pl", "Lahidż"), ("pt", "Lahij"), ("ro", "Guvernoratul Lahij"), ("ru", "Лахдж"), ("sv", "Lahij"), ("sw", "Wilaya ya Lahij"), ("tr", "Lahic ili"), ("uk", "Лахдж"), ("ur", "محافظہ لحج"), ("zh", "拉赫季省")]),
                        unofficial_name_list: ["Lahej", "Lahj"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "MA",
                        country_alpha2: Alpha2::YE,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.4700312), longitude: Some(45.3228575), max_latitude: Some(15.4981702), min_latitude: Some(15.4511678), max_longitude: Some(45.3470603), min_longitude: Some(45.2984496)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة مأرب"), ("bg", "Мариб"), ("bn", "ম\u{9be}‘রিব গভর\u{9cd}নোরেট"), ("ca", "Governació de Marib"), ("ccp", "𑄟‘𑄢\u{11128}𑄛\u{11134}"), ("da", "Ma’rib Governorate"), ("de", "Gouvernement Ma’rib"), ("el", "Μαρίμπ"), ("en", "Ma’rib"), ("es", "Gobernación de Mareb"), ("eu", "Ma’rib gobernantzia"), ("fa", "استان مأرب"), ("fi", "Marib"), ("fr", "Gouvernorat de Ma’rib"), ("gu", "મરીબ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "מארב"), ("hi", "मारिब प\u{94d}रान\u{94d}त"), ("hr", "Ma’rib"), ("hu", "Marib kormányzóság"), ("id", "Kegubernuran Ma’rib"), ("it", "governatorato di Ma’rib"), ("ja", "マアリブ県"), ("ka", "მარიბის მუჰაფაზა"), ("kn", "ಮಾರ\u{cbf}ಬ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "마리브 주"), ("lt", "Maribo gubernija"), ("lv", "Maaribas muhāfaza"), ("mr", "मारीब गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Ma’rib Governorate"), ("nb", "Ma’rib"), ("nl", "Ma’rib"), ("no", "Ma’rib"), ("pl", "Marib"), ("pt", "Ma’rib"), ("ro", "Guvernoratul Ma’rib"), ("ru", "Мариб"), ("si", "ම\u{dcf}ර\u{dd2}බ\u{dca} පළ\u{dcf}ත"), ("sv", "Marib"), ("sw", "Wilaya ya Marib"), ("ta", "ம ‘ரிப\u{bcd} கோவெர\u{bcd}னோரே"), ("te", "మ\u{c3e}ర\u{c3f}బ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "มะร\u{e34}บ"), ("tr", "Ma’rib ili"), ("uk", "Маріб"), ("ur", "محافظہ مآرب"), ("vi", "Tỉnh Ma’rib"), ("zh", "马里卜省")]),
                        unofficial_name_list: ["Marab", "Mareb"].to_vec(),
                    }
                ),
                (
                    "MR",
                    Subdivision{
                        name: "MR",
                        country_alpha2: Alpha2::YE,
                        code: "MR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.5238423), longitude: Some(51.6834275), max_latitude: Some(19.002331), min_latitude: Some(15.0801567), max_longitude: Some(53.0783), min_longitude: Some(50.1137559)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة المهرة"), ("bg", "Махра"), ("bn", "আল ম\u{9be}হর\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació d’al-Mahra"), ("ccp", "𑄃𑄣\u{11134} 𑄟𑄦\u{11134}𑄢𑄦\u{11134}"), ("ceb", "Al Mahrah"), ("da", "Al Mahrah Governorate"), ("de", "Gouvernement al-Mahra"), ("el", "Αλ Μαχράχ"), ("en", "Al Mahrah"), ("es", "Gobernación de Al Mahrah"), ("eu", "Al Mahrah gobernantzia"), ("fa", "استان مهره"), ("fi", "Mahra"), ("fr", "Mahra"), ("gu", "અલ મહ\u{acd}રાહ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-מהרה"), ("hi", "अल-महराह प\u{94d}रान\u{94d}त"), ("hr", "Al Mahra"), ("hu", "Mahra kormányzóság"), ("id", "Kegubernuran Al-Mahrah"), ("it", "governatorato di al-Mahra"), ("ja", "マフラ県"), ("ka", "ელ-მაჰრის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಮಹ\u{ccd}ರಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "마라 주"), ("lt", "Al Machros gubernija"), ("lv", "Mahras muhāfaza"), ("mr", "अल माहह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Mahrah Governorate"), ("nb", "Al-Mahrah"), ("nl", "Al Mahrah"), ("no", "Al-Mahrah"), ("pl", "Al-Mahra"), ("pt", "Al Mahra"), ("ro", "Guvernoratul Al Mahrah"), ("ru", "Эль-Махра"), ("si", "අල\u{dca} මර\u{dcf} පළ\u{dcf}ත"), ("sv", "Al-Mahrah"), ("sw", "Wilaya ya Al Mahrah"), ("ta", "அல\u{bcd} மஹரஹ கோவெர\u{bcd}னோரே"), ("te", "అంల\u{c4d} మహ\u{c3e}ర\u{c3e}హ\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ล มาฮ\u{e37}ราฮ\u{e4c} โกเวอโนเรท"), ("tr", "El Mahra ili"), ("uk", "Ель-Махра"), ("ur", "محافظہ المہرہ"), ("vi", "Tỉnh Al Mahrah"), ("zh", "马哈拉省")]),
                        unofficial_name_list: ["Al Mahrah"].to_vec(),
                    }
                ),
                (
                    "MW",
                    Subdivision{
                        name: "MW",
                        country_alpha2: Alpha2::YE,
                        code: "MW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.3963229), longitude: Some(43.5606946), max_latitude: Some(15.5304159), min_latitude: Some(15.006784), max_longitude: Some(43.924874), min_longitude: Some(43.25229710000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة المحويت"), ("bg", "Махуит"), ("bn", "আল ম\u{9be}হিত গভর\u{9cd}নোরেট"), ("ca", "Governació d’Al Mahwit"), ("ccp", "𑄃𑄣\u{11134} 𑄟𑄦\u{11134}𑄃\u{1112a}𑄃\u{11128}𑄖\u{11134}"), ("ceb", "Al Maḩwīt"), ("da", "Al Mahwit Governorate"), ("de", "Gouvernement al-Mahwit"), ("el", "Αλ Μαχβίτ Γκοβερνοράτε"), ("en", "Al Mahwit"), ("es", "Gobernación de Al Mahwit"), ("eu", "Al Mahwit gobernantzia"), ("fa", "استان محویت"), ("fi", "Al Mahwit"), ("fr", "Gouvernorat d’Al Mahwit"), ("gu", "અલ માહવિટ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "אל-מחוית"), ("hi", "अल-महवीत प\u{94d}रान\u{94d}त"), ("hr", "Al Mahwit"), ("hu", "Mahvít kormányzóság"), ("id", "Kegubernuran Al-Mahwit"), ("it", "governatorato di al-Mahwit"), ("ja", "マフウィート県"), ("ka", "ელ-მაჰვიტის მუჰაფაზა"), ("kn", "ಅಲ\u{ccd} ಮಹ\u{ccd}ವ\u{cbf}ತ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "마위트 주"), ("lt", "Al Machvito gubernija"), ("lv", "Mehvītas muhāfaza"), ("mr", "अल म\u{947}हविट गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Al Mahwit Governorate"), ("nb", "Al Mahwit"), ("nl", "Al Mahwit"), ("no", "Al Mahwit"), ("pl", "Al-Mahwit"), ("pt", "Al Mahwit"), ("ro", "Guvernoratul Al Mahwit"), ("ru", "Махвит"), ("si", "අල\u{dca} මහ\u{dca}ව\u{dd2}ට\u{dca} පළ\u{dcf}ත"), ("sv", "Al-Mahwit"), ("sw", "Wilaya ya Al Mahwit"), ("ta", "அல\u{bcd} மஹவிட\u{bcd} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "అల\u{c4d} మహ\u{c4d}వ\u{c3f}త\u{c4d} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "อ\u{e31}ล มะห\u{e4c}ว\u{e34}ท กอฟเวอโนเลท"), ("tr", "El Mahvit ili"), ("uk", "Махвіт"), ("ur", "محافظہ المحویت"), ("vi", "Tỉnh Al Mahwit"), ("zh", "迈赫维特省")]),
                        unofficial_name_list: ["Al Mahwit"].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "RA",
                        country_alpha2: Alpha2::YE,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ريمة"), ("bg", "Райма"), ("bn", "র\u{9be}য\u{9bc}ম\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació de Raimah"), ("ccp", "𑄢𑄬𑄟𑄦\u{11134}"), ("da", "Raymah Governorate"), ("de", "Gouvernement Raima"), ("el", "Ρέιμαχ"), ("en", "Raymah"), ("es", "Gobernación de Raymah"), ("eu", "Raymah gobernantzia"), ("fa", "استان ریمه"), ("fi", "Raima"), ("fr", "Gouvernorat de Raima"), ("gu", "ર\u{ac7}મ\u{ac7}હ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "רימה"), ("hi", "र\u{947}माह प\u{94d}रान\u{94d}त"), ("hr", "Raima"), ("hu", "Rajma kormányzóság"), ("id", "Kegubernuran Raymah"), ("it", "governatorato di Rayma"), ("ja", "ライマ県"), ("ka", "რაიმის მუჰაფაზა"), ("kn", "ರೇಮಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "라이마 주"), ("lt", "Raimos gubernija"), ("lv", "Raimas muhāfaza"), ("mr", "र\u{947}मह गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Raymah Governorate"), ("nb", "Raymah"), ("nl", "Raymah"), ("no", "Raymah"), ("pl", "Rajma"), ("pt", "Raymah"), ("ro", "Guvernoratul Raymah"), ("ru", "Райма"), ("si", "රේම\u{dcf}හ\u{dca} පළ\u{dcf}ත"), ("sv", "Raymah"), ("sw", "Wilaya ya Raymah"), ("ta", "ரெய\u{bcd}ம\u{bbe}ஹ\u{bcd} கோவெர\u{bcd}னோகைட\u{bcd}"), ("te", "ర\u{c47}మ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "เรมาฮ\u{e4c} โกเวอโนเรท"), ("tr", "Rayima ili"), ("uk", "Рейма"), ("ur", "محافظہ ریمہ"), ("vi", "Tỉnh Raymah"), ("zh", "赖马省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::YE,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sanaa"), ("am", "ሳና"), ("ar", "صنعاء"), ("az", "Sana"), ("be", "Горад Сана"), ("bg", "Сана"), ("bn", "স\u{9be}ন\u{9be}"), ("bs", "Sana’a"), ("ca", "Sanà"), ("ccp", "𑄃𑄟𑄚\u{11127}𑄖\u{11134} 𑄃𑄣\u{11134} 𑄃\u{1112a}𑄥\u{11128}𑄟𑄦\u{11134}"), ("ceb", "Sanaa"), ("cs", "San’á"), ("cy", "Sana’a"), ("da", "Sanaá"), ("de", "Sanaa"), ("el", "Σαναά"), ("en", "Amanat Al Asimah"), ("es", "Saná"), ("eu", "Sana"), ("fa", "صنعا"), ("fi", "Sanaa"), ("fr", "Sanaa"), ("ga", "San’a"), ("gl", "Saná"), ("gu", "સાના"), ("he", "צנעא"), ("hi", "साना"), ("hr", "Sana"), ("hu", "Szanaa"), ("hy", "Սանաա"), ("id", "Sana’a"), ("is", "Sana"), ("it", "Sana’a"), ("ja", "サヌア"), ("ka", "სანა"), ("kk", "Сана"), ("kn", "ಸನಾ"), ("ko", "사나"), ("ky", "Сана"), ("lt", "Sana"), ("lv", "Sana"), ("mk", "Сана"), ("ml", "സന"), ("mn", "Санаа хот"), ("mr", "साना"), ("ms", "Sana’a"), ("nb", "Sanaá"), ("ne", "साना"), ("nl", "Sanaa"), ("no", "Sanaá"), ("pa", "ਸਨਾ"), ("pl", "Sana"), ("ps", "صنعا"), ("pt", "Sana"), ("ro", "Sana’a"), ("ru", "Сана"), ("si", "සැන\u{dcf}"), ("sk", "Saná"), ("sl", "Sana"), ("so", "Sana’a"), ("sq", "Sana"), ("sr", "Сана"), ("sr_Latn", "Sana"), ("sv", "Sanaa"), ("sw", "Sana’a"), ("ta", "சன\u{bcd}ஆ"), ("te", "సన\u{c3e}"), ("th", "ซานา"), ("tk", "Sana"), ("tr", "San’a"), ("uk", "Сана"), ("ur", "صنعاء"), ("uz", "Sano"), ("vi", "Sana’a"), ("yo", "Sana’a"), ("yo_BJ", "Sana’a"), ("yue", "薩那"), ("yue_Hans", "萨那"), ("zh", "萨那")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::YE,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.9509413), longitude: Some(43.7477743), max_latitude: Some(16.9842336), min_latitude: Some(16.9213471), max_longitude: Some(43.7776101), min_longitude: Some(43.7216377)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة صعدة"), ("bg", "Саада"), ("bn", "স\u{9be}\u{981}দ\u{9be} গভর\u{9cd}নোরেট"), ("ca", "Governació de Sa’dah"), ("ccp", "𑄥‘𑄓𑄦\u{11134}"), ("cs", "Provincie Sa’ada"), ("da", "Saada Governorate"), ("de", "Gouvernement Sa’da"), ("el", "Σαάντα Γκοβερνοράτε"), ("en", "Sa’dah"), ("es", "Gobernación de Sa’dah"), ("eu", "Sa’dah gobernantzia"), ("fa", "استان صعده"), ("fi", "Saada"), ("fr", "Gouvernorat de Sa’dah"), ("gu", "સાદા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "צעדה"), ("hi", "सआदाह प\u{94d}रान\u{94d}त"), ("hr", "Sa’da"), ("hu", "Szaada kormányzóság"), ("id", "Kegubernuran Saada"), ("it", "governatorato di Sa’da"), ("ja", "サアダ県"), ("ka", "საადის მუჰაფაზა"), ("kn", "ಸಾಡ ಗವರ\u{ccd}ನರ\u{ccd}"), ("ko", "사다 주"), ("lt", "Saados gubernija"), ("lv", "Saadas muhāfaza"), ("mr", "सादा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Saada Governorate"), ("nb", "Sa’dah"), ("nl", "Sa’dah"), ("no", "Sa’dah"), ("pl", "Sada"), ("pt", "Sa’dah"), ("ro", "Guvernoratul Sa’dah"), ("ru", "Саада"), ("si", "ස\u{dcf}ද\u{dcf} පළ\u{dcf}ත"), ("sv", "Sadah"), ("sw", "Wilaya ya Sadah"), ("ta", "ச\u{bbe}த\u{bbe} கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "స\u{c3e}ద\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ซาดา กอฟเวอโนเลต"), ("tr", "Saada ili"), ("uk", "Саада"), ("ur", "سادہ گوورنوراتے"), ("vi", "Tỉnh Saada"), ("zh", "萨达省")]),
                        unofficial_name_list: ["Saʿadah"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::YE,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(15.368889), longitude: Some(47.023611), max_latitude: Some(15.3717846), min_latitude: Some(15.3665084), max_longitude: Some(47.0317305), min_longitude: Some(47.0220423)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة شبوة"), ("bg", "Шабуа"), ("bn", "স\u{9be}ব\u{9be}হ গভর\u{9cd}নোরেট"), ("ca", "Governació de Xabwa"), ("ccp", "𑄥𑄛\u{11134}𑄤𑄦\u{11134}"), ("ceb", "Shabwah"), ("da", "Shabwah Governorate"), ("de", "Gouvernement Schabwa"), ("el", "Σαμπγουά Γκοβερνοράτε"), ("en", "Shabwah"), ("es", "Gobernación de Shabwah"), ("fa", "استان شبوه"), ("fi", "Šabwa"), ("fr", "Shabwah"), ("gu", "શ\u{ac7}બવા ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "שבוה"), ("hi", "शबवाह प\u{94d}रान\u{94d}त"), ("hr", "Shabwa"), ("hu", "Sabva kormányzóság"), ("id", "Kegubernuran Shabwah"), ("it", "governatorato di Shabwa"), ("ja", "シャブワ県"), ("ka", "შაბვის მუჰაფაზა"), ("kn", "ಷಬ\u{ccd}ವಾ ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "샤브와 주"), ("lt", "Šabvos gubernija"), ("lv", "Šebvas muhāfaza"), ("mr", "शबवा गव\u{94d}हर\u{94d}नोर\u{947}ट"), ("ms", "Shabwah Governorate"), ("nb", "Shabwah"), ("nl", "Shabwah"), ("no", "Shabwah"), ("pl", "Szabwa"), ("pt", "Shabwa"), ("ro", "Guvernoratul Shabwah"), ("ru", "Шабва"), ("si", "ශබ\u{dca}ව\u{dcf} පළ\u{dcf}ත"), ("sl", "Šabva"), ("sr", "Шабва"), ("sr_Latn", "Šabva"), ("sv", "Shabwah"), ("sw", "Wilaya ya Shabwah"), ("ta", "ஷபவ\u{bbe}ஹ கோவெர\u{bcd}னோரேட\u{bcd}"), ("te", "షబ\u{c4d}వ\u{c3e} గవర\u{c4d}నర\u{c47}ట\u{c4d}"), ("th", "ช\u{e31}ปว\u{e4a}ะห\u{e4c} เกอเวอโนเลต"), ("tr", "Şabva ili"), ("uk", "Шабва"), ("ur", "محافظہ شبوہ"), ("vi", "Tỉnh Shabwah"), ("zh", "舍卜沃省")]),
                        unofficial_name_list: ["Shabwah"].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::YE,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(16.0905301), longitude: Some(49.6554124), max_latitude: Some(16.0981446), min_latitude: Some(16.08029), max_longitude: Some(49.6662712), min_longitude: Some(49.64571480000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة صنعاء"), ("be", "Сана"), ("bg", "Сана²"), ("ca", "Governació de Sanà"), ("ccp", "𑄥𑄚‘𑄃\u{11127}"), ("ceb", "Sanaa²"), ("de", "Gouvernement Sanaa"), ("en", "Sana’a"), ("es", "Gobernación de Saná"), ("fa", "استان صنعا"), ("fi", "Sanaa²"), ("fr", "Gouvernorat de Sanaa"), ("he", "צנעא²"), ("hi", "सनआ प\u{94d}रान\u{94d}त"), ("hr", "Sana²"), ("hu", "Szanaa kormányzóság"), ("id", "Kegubernuran Sana’a"), ("it", "governatorato di San’a’"), ("ja", "サナア県"), ("ka", "სანის მუჰაფაზა"), ("ko", "사나 주"), ("nb", "Sanaá²"), ("nl", "Sanaa²"), ("no", "Sanaá²"), ("pl", "Sana²"), ("pt", "Sana²"), ("ro", "Guvernoratul Sana’a"), ("ru", "Сана²"), ("sv", "Guvernementet Sana’a"), ("sw", "Wilaya ya Sana’a"), ("tr", "San’a ili"), ("uk", "Сана²"), ("ur", "محافظہ صنعاء"), ("zh", "薩那省")]),
                        unofficial_name_list: ["Sana", "Sanaa", "Sanaʿa", "Sanaʿa City", "Sanʿaʿ"].to_vec(),
                    }
                ),
                (
                    "SU",
                    Subdivision{
                        name: "SU",
                        country_alpha2: Alpha2::YE,
                        code: "SU",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أرخبيل سقطرى"), ("ca", "Governació de Socotra"), ("ccp", "𑄃𑄢\u{11134}𑄈𑄝\u{11128}𑄣\u{11134} 𑄥\u{1112a}𑄇𑄑\u{11133}𑄢"), ("ceb", "Socotra"), ("de", "Gouvernement Sokotra"), ("en", "Arkhabil Suqutra"), ("es", "Gobernación de Socotra"), ("fa", "استان ارخبیل سقطری"), ("fr", "Gouvernorat de Socotra"), ("it", "governatorato di Socotra"), ("ja", "ソコトラ県"), ("ko", "소코트라 주"), ("ur", "محافظہ سقطری"), ("zh", "索科特拉省")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "TA",
                    Subdivision{
                        name: "TA",
                        country_alpha2: Alpha2::YE,
                        code: "TA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.4131166), longitude: Some(43.6375314), max_latitude: Some(13.887883), min_latitude: Some(12.6340961), max_longitude: Some(44.510999), min_longitude: Some(43.2397839)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Governorate,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تعز"), ("bg", "Таиз"), ("bn", "ত\u{9be}‘ইজ গভর\u{9cd}নোরেট"), ("ca", "Governació de Taizz"), ("ccp", "𑄑\u{1112d}𑄌\u{11134}"), ("da", "Ta’izz Governorate"), ("de", "Gouvernement Ta’izz"), ("el", "Ταΐζ Γκοβερνοράτε"), ("en", "Taiz"), ("es", "Gobernación de Ta’izz"), ("fa", "استان تعز"), ("fi", "Taizz"), ("fr", "Gouvernorat de Ta’izz"), ("gu", "તા‘ઝ ગવર\u{acd}નોર\u{ac7}ટ"), ("he", "תעז"), ("hi", "ताइज\u{93c} प\u{94d}रान\u{94d}त"), ("hr", "Taizz"), ("hu", "Taizz kormányzóság"), ("id", "Kegubernuran Ta’izz"), ("it", "governatorato di Ta’izz"), ("ja", "タイズ県"), ("ka", "ტაიზის მუჰაფაზა"), ("kn", "ತಾಯ\u{cbf}ಜ\u{ccd} ಗವರ\u{ccd}ನೇಟ\u{ccd}"), ("ko", "타이즈 주"), ("lt", "Taiz gubernija"), ("lv", "Taizas muhāfaza"), ("mr", "ता‘ज गव\u{94d}हर\u{947}नर\u{947}ट"), ("ms", "Ta’izz Governorate"), ("nb", "Ta’izz"), ("nl", "Ta’izz"), ("no", "Ta’izz"), ("pl", "Taizz"), ("pt", "Ta’izz"), ("ro", "Guvernoratul Ta’izz"), ("ru", "Таиз"), ("si", "ට\u{dcf}ඉස\u{dca} පළ\u{dcf}ත"), ("sr", "Таиз"), ("sr_Latn", "Taiz"), ("sv", "Taizz"), ("sw", "Wilaya ya Taizz"), ("ta", "ட\u{bbe} ‘இஸ\u{bcd}ஸ\u{bcd} கோவெர\u{bcd}னோர\u{bbe}ட\u{bcd}"), ("te", "ట\u{c3e}య\u{c3f}జ\u{c4d}"), ("th", "ทา อ\u{e34}ซ กอฟเวอโนเลท"), ("tr", "Taiz ili"), ("uk", "Таїз"), ("ur", "محافظہ تعز"), ("vi", "Tỉnh Ta’izz"), ("zh", "塔伊茲省")]),
                        unofficial_name_list: ["Taiz"].to_vec(),
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
#[cfg(feature = "ye")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::YE,
        alpha3: Alpha3::YEM,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 967,
        currency_code: "YER",
        gec: Some(GEC::YM),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::YEM),
        iso_long_name: "The Republic of Yemen",
        iso_short_name: "Yemen",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7, 8, 9].to_vec(),
        national_prefix: "0",
        nationality: Some("Yemeni"),
        number: "887",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        subregion: Some(SubRegion::WesternAsia),
        un_locode: "YE",
        unofficial_name_list: ["Yemen", "اليمن", "Jemen", "Yémen", "イエメン"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Yemen"),
            ("af", "Jemen"),
            ("ak", "Yemen"),
            ("am", "ፘመን"),
            ("an", "Yemen"),
            ("ar", "اليمن"),
            ("as", "য়েমেন"),
            ("ay", "Yemen"),
            ("az", "Yəmən"),
            ("ba", "Yemen"),
            ("be", "Емен"),
            ("bg", "Йемен"),
            ("bi", "Yemen"),
            ("bn", "ইয়েমেন"),
            ("bn_IN", "ইয়েমেন"),
            ("br", "Yemen"),
            ("bs", "Jemen"),
            ("ca", "Iemen"),
            ("ce", "Йемен"),
            ("ch", "Yemen"),
            ("cs", "Jemen"),
            ("cv", "Йемен"),
            ("cy", "Yemen"),
            ("da", "Yemen"),
            ("de", "Jemen"),
            ("dv", "ޔ\u{7a6}މ\u{7a6}ނ\u{7b0}"),
            ("dz", "ཡ\u{f7a}་མ\u{f7a}ན།"),
            ("ee", "Yemen"),
            ("el", "Υεμένη"),
            ("en", "Yemen"),
            ("eo", "Jemeno"),
            ("es", "Yemen"),
            ("et", "Jeemen"),
            ("eu", "Yemen"),
            ("fa", "یمن"),
            ("ff", "Yemen"),
            ("fi", "Jemen"),
            ("fo", "Jemen"),
            ("fr", "Yémen"),
            ("fy", "Jemen"),
            ("ga", "Éimin"),
            ("gl", "Iemen"),
            ("gn", "Yemen"),
            ("gu", "યમન"),
            ("gv", "Yn Yeaman"),
            ("ha", "Yemen"),
            ("he", "תימן"),
            ("hi", "यमन"),
            ("hr", "Jemen"),
            ("ht", "Yemèn"),
            ("hu", "Jemen"),
            ("hy", "Եմեն"),
            ("ia", "Yemen"),
            ("id", "Yaman"),
            ("io", "Yemen"),
            ("is", "Jemen"),
            ("it", "Yemen"),
            ("iu", "Yemen"),
            ("ja", "イエメン"),
            ("ka", "იემენი"),
            ("ki", "Yemen"),
            ("kk", "Йемен"),
            ("kl", "Yemen"),
            ("km", "យេមែន"),
            ("kn", "ಯ\u{cc6}ಮ\u{cc6}ನ\u{ccd}"),
            ("ko", "예멘"),
            ("ku", "Yemen"),
            ("kv", "Йемен"),
            ("kw", "Yemen"),
            ("ky", "Йемен"),
            ("lo", "Yemen"),
            ("lt", "Jemenas"),
            ("lv", "Jemena"),
            ("mi", "Yemen"),
            ("mk", "Јемен"),
            ("ml", "യെമന\u{d4d}\u{200d}"),
            ("mn", "Иемэн"),
            ("mr", "य\u{947}म\u{947}न"),
            ("ms", "Yaman"),
            ("mt", "Jemen"),
            (
                "my",
                "ယ\u{102e}မင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Yemen"),
            ("nb", "Jemen"),
            ("ne", "य\u{947}म\u{947}न"),
            ("nl", "Jemen"),
            ("nn", "Jemen"),
            ("nv", "Shádiʼááhjí Ásáí Dineʼé Bikéyah"),
            ("oc", "Iemèn"),
            ("or", "ୟମନ"),
            ("pa", "ਯਮਨ"),
            ("pi", "यमन"),
            ("pl", "Jemen"),
            ("ps", "یمن"),
            ("pt", "Iémen"),
            ("pt_BR", "Iêmen"),
            ("ro", "Yemen"),
            ("ru", "Йемен"),
            ("rw", "Yemeni"),
            ("sc", "Yemen"),
            ("sd", "يمن"),
            ("si", "යේමනය"),
            ("sk", "Jemen"),
            ("sl", "Jemen"),
            ("so", "Yaman"),
            ("sq", "Jemen"),
            ("sr", "Јемен"),
            ("sv", "Yemen"),
            ("sw", "Yemen"),
            ("ta", "யேமன\u{bcd}"),
            ("te", "య\u{c46}మ\u{c46}న\u{c4d}"),
            ("tg", "Яман"),
            ("th", "เยเมน"),
            ("ti", "የመን"),
            ("tk", "Ýemen"),
            ("tl", "Yemen"),
            ("tr", "Yemen"),
            ("tt", "Йемен"),
            ("ug", "يەمەن"),
            ("uk", "Ємен"),
            ("ur", "یمن"),
            ("uz", "Yaman"),
            ("ve", "Yemen"),
            ("vi", "Y-ê-men"),
            ("wa", "Yemen"),
            ("wo", "Yaman"),
            ("xh", "Yemen"),
            ("yo", "Yemen"),
            ("zh_CN", "也门"),
            ("zh_HK", "也門"),
            ("zh_TW", "葉門"),
            ("zu", "IYemen"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
