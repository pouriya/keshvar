// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Malawi

#[cfg(all(feature = "mw", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::MW;
    pub const ALPHA3: Alpha3 = Alpha3::MWI;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 265;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::MWK;
    pub const GEC: Option<GEC> = Some(GEC::MI);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::MAW);
    pub const ISO_SHORT_NAME: &str = "Malawi";
    pub const ISO_LONG_NAME: &str = "The Republic of Malawi";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "ny"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "ny"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Malawian");
    pub const NUMBER: &str = "454";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternAfrica);
    pub const UN_LOCODE: &str = "MW";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Malawi", "マラウイ"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Malawi"),
        ("af", "Malawi"),
        ("ak", "Malawi"),
        ("am", "ማላዊ"),
        ("an", "Malawi"),
        ("ar", "ملاوي"),
        ("as", "ম\u{9be}ল\u{9be}ৱী"),
        ("ay", "Malawi"),
        ("az", "Malavi"),
        ("ba", "Malawi"),
        ("be", "Малаві"),
        ("bg", "Малави"),
        ("bi", "Malawi"),
        ("bn", "ম\u{9be}ল\u{9be}উই"),
        ("bn_IN", "ম\u{9be}ল\u{9be}উই"),
        ("br", "Malawi"),
        ("bs", "Malavi"),
        ("ca", "Malawi"),
        ("ce", "Малави"),
        ("ch", "Malawi"),
        ("cs", "Malawi"),
        ("cv", "Малави"),
        ("cy", "Malawi"),
        ("da", "Malawi"),
        ("de", "Malawi"),
        ("dv", "މ\u{7a6}ލ\u{7a7}ވ\u{7a9}"),
        ("dz", "མ་ལ་ཝ\u{f72}།"),
        ("ee", "Malawi"),
        ("el", "Μαλάουι"),
        ("en", "Malawi"),
        ("eo", "Malavio"),
        ("es", "Malaui"),
        ("et", "Malawi"),
        ("eu", "Malawi"),
        ("fa", "مالاوی"),
        ("ff", "Malawi"),
        ("fi", "Malawi"),
        ("fo", "Malavi"),
        ("fr", "Malawi"),
        ("fy", "Malawy"),
        ("ga", "An Mhaláiv"),
        ("gl", "Malaui"),
        ("gn", "Malawi"),
        ("gu", "માલાવી"),
        ("gv", "Malawi"),
        ("ha", "Malawi"),
        ("he", "מלאווי"),
        ("hi", "मलावी"),
        ("hr", "Malavi"),
        ("ht", "Malawi"),
        ("hu", "Malawi"),
        ("hy", "Մալավի"),
        ("ia", "Malawi"),
        ("id", "Malawi"),
        ("io", "Malawi"),
        ("is", "Malaví"),
        ("it", "Malawi"),
        ("iu", "Malawi"),
        ("ja", "マラウイ"),
        ("ka", "მალავი"),
        ("ki", "Malawi"),
        ("kk", "Малави"),
        ("kl", "Malawi"),
        ("km", "ម\u{17c9}ាឡាវ\u{17b8}"),
        ("kn", "ಮಲಾವ\u{cbf}"),
        ("ko", "말라위"),
        ("ku", "Malawî"),
        ("kv", "Malawi"),
        ("kw", "Malawi"),
        ("ky", "Малави"),
        ("lo", "Malawi"),
        ("lt", "Malavis"),
        ("lv", "Malāvija"),
        ("mi", "Marāwi"),
        ("mk", "Малави"),
        ("ml", "മല\u{d3e}വി"),
        ("mn", "Малави"),
        ("mr", "मलवी"),
        ("ms", "Malawi"),
        ("mt", "Malawi"),
        ("my", "မာလဝ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Malawi"),
        ("nb", "Malawi"),
        ("ne", "मालावी"),
        ("nl", "Malawi"),
        ("nn", "Malawi"),
        ("nv", "Malawi"),
        ("oc", "Malawi"),
        ("or", "ମ\u{b3e}ଲ\u{b3e}ଓ\u{b4d}ବ\u{b3f}"),
        ("pa", "ਮਾਲਾਵੀਆ"),
        ("pi", "मलावी"),
        ("pl", "Malawi"),
        ("ps", "مالاوي"),
        ("pt", "Malawi"),
        ("pt_BR", "Malaui"),
        ("ro", "Malawi"),
        ("ru", "Малави"),
        ("rw", "Malawi"),
        ("sc", "Malawi"),
        ("sd", "Malawi"),
        ("si", "මල\u{dcf}ව\u{dd2}"),
        ("sk", "Malawi"),
        ("sl", "Malavi"),
        ("so", "Malaawi"),
        ("sq", "Malaui"),
        ("sr", "Малави"),
        ("sv", "Malawi"),
        ("sw", "Malawi"),
        ("ta", "மல\u{bbe}வி"),
        ("te", "మలవ\u{c40}"),
        ("tg", "Малави"),
        ("th", "มาลาว\u{e35}"),
        ("ti", "ማላዊ"),
        ("tk", "Malawi"),
        ("tl", "Malawi"),
        ("tr", "Malavi"),
        ("tt", "Малави"),
        ("ug", "مالاۋى"),
        ("uk", "Малаві"),
        ("ur", "ملاوی"),
        ("uz", "Malavi"),
        ("ve", "Malawi"),
        ("vi", "Ma-la-uy"),
        ("wa", "Malawi"),
        ("wo", "Malawi"),
        ("xh", "Malawi"),
        ("yo", "Màláwì"),
        ("zh_CN", "马拉维"),
        ("zh_HK", "馬拉維"),
        ("zh_TW", "馬拉威"),
        ("zu", "IMalawi"),
    ];
    #[cfg(all(feature = "mw", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = -13.254308;
        pub const LONGITUDE: f64 = 34.301525;
        pub const MAX_LATITUDE: f64 = -9.3672272;
        pub const MAX_LONGITUDE: f64 = 35.91857299999999;
        pub const MIN_LATITUDE: f64 = -17.1295216;
        pub const MIN_LONGITUDE: f64 = 32.6725205;
        pub const NORTHEAST_LATITUDE: f64 = -9.3672272;
        pub const NORTHEAST_LONGITUDE: f64 = 35.91857299999999;
        pub const SOUTHWEST_LATITUDE: f64 = -17.1295216;
        pub const SOUTHWEST_LONGITUDE: f64 = 32.6725205;
    }
}
#[cfg(all(feature = "mw", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -13.254308,
            longitude: 34.301525,
            max_latitude: -9.3672272,
            max_longitude: 35.91857299999999,
            min_latitude: -17.1295216,
            min_longitude: 32.6725205,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -9.3672272,
                    longitude: 35.91857299999999,
                },
                southwest: CountryGeoBound {
                    latitude: -17.1295216,
                    longitude: 32.6725205,
                },
            },
        }
    }
}

#[cfg(all(feature = "mw", feature = "subdivisions"))]
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
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::MW,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.9876054), longitude: Some(34.9561748), max_latitude: Some(-14.9766781), min_latitude: Some(-15.0083795), max_longitude: Some(34.9850202), min_longitude: Some(34.9307894)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بالاكا"), ("az", "Balaka"), ("bg", "Балака"), ("bn", "ব\u{9be}ল\u{9be}ক\u{9be} জেল\u{9be}"), ("ccp", "𑄝𑄣𑄇"), ("ceb", "Balaka"), ("da", "Balaka District"), ("de", "Balaka"), ("el", "Μπαλάκα"), ("en", "Balaka"), ("es", "Distrito de Balaka"), ("fa", "بخش بالاکا"), ("fi", "Balakan piirikunta"), ("fr", "District de Balaka"), ("gl", "Distrito de Balaka"), ("gu", "બાલાકા જિલ\u{acd}લો"), ("hi", "बलाका जिला"), ("id", "Distrik Balaka"), ("it", "Distretto di Balaka"), ("ja", "バラカ"), ("ka", "ბალაკის რაიონი"), ("kn", "ಬಾಲಕ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "발라카 현"), ("lt", "Balakos apskritis"), ("lv", "Balakas distrikts"), ("mr", "बालाक जिल\u{94d}हा"), ("ms", "Daerah Balaka"), ("nb", "Balaka"), ("nl", "Balaka"), ("no", "Balaka"), ("pl", "Dystrykt Balaka"), ("pt", "Balaka"), ("ro", "Districtul Balaka"), ("ru", "Балака"), ("si", "බලක\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Балака"), ("sr_Latn", "Balaka"), ("sv", "Balaka"), ("ta", "ப\u{bbe}ல\u{bbe}க ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బల\u{c3e}క\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "บาลากา"), ("tr", "Balaka"), ("uk", "Район Балака"), ("ur", "بالاکا ضلع"), ("vi", "Quận Balaka"), ("zh", "巴拉卡縣")]),
                        unofficial_name_list: ["Balaka"].to_vec(),
                    }
                ),
                (
                    "BL",
                    Subdivision{
                        name: "BL",
                        country_alpha2: Alpha2::MW,
                        code: "BL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.786111), longitude: Some(35.005833), max_latitude: Some(-15.6822126), min_latitude: Some(-15.8702044), max_longitude: Some(35.1090431), min_longitude: Some(34.952488)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بلانتير"), ("az", "Blantyre"), ("bg", "Блантайр"), ("bn", "ব\u{9cd}ল\u{9be}ন\u{9cd}তি জেল\u{9be}"), ("ccp", "𑄝\u{11133}𑄣𑄚\u{11134}𑄑𑄠𑄢\u{11134}"), ("ceb", "Blantyre District"), ("da", "Blantyre District"), ("de", "Blantyre"), ("el", "Μπλαντάιρ"), ("en", "Blantyre"), ("es", "Distrito de Blantyre"), ("fa", "بخش بلانتایر"), ("fi", "Blantyren piirikunta"), ("fr", "District de Blantyre"), ("gl", "Distrito de Blantyre"), ("gu", "બ\u{acd}લાન\u{acd}ટીર જિલ\u{acd}લો"), ("hi", "ब\u{94d}ला\u{902}तिर\u{947} जिला"), ("id", "Distrik Blantyre"), ("it", "Distretto di Blantyre"), ("ja", "ブランタイヤ"), ("ka", "ბლანტირეს რაიონი"), ("kn", "ಬ\u{ccd}ಲಾಂಟೈರ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "블랜타이어 현"), ("lt", "Blantiro apskritis"), ("lv", "Blantairas distrikts"), ("mr", "ब\u{94d}ल\u{901}टायर\u{947} जिल\u{94d}हा"), ("ms", "Blantyre District"), ("nb", "Blantyre"), ("nl", "Blantyre"), ("no", "Blantyre"), ("pl", "Dystrykt Blantyre"), ("pt", "Blantyre"), ("ro", "Districtul Blantyre"), ("ru", "Блантайр"), ("si", "බ\u{dca}ලන\u{dca}ටය\u{dca}රේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Блантајер"), ("sr_Latn", "Blantajer"), ("sv", "Blantyre"), ("ta", "ப\u{bcd}ள\u{bbe}ண\u{bcd}ட\u{bcd}யர\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c4d}ల\u{c3e}ంట\u{c48}ర\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "บลานไทร\u{e4c}"), ("tr", "Blantyre"), ("uk", "Блантайр"), ("ur", "بلانتیری ضلع"), ("vi", "Quận Blantyre"), ("zh", "布蘭太爾縣")]),
                        unofficial_name_list: ["Blantyre"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::MW,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Mərkəz bölgəsi"), ("be", "Цэнтральны рэгіён"), ("bg", "Централен регион"), ("ca", "Regió Central"), ("ccp", "𑄥𑄬𑄚𑄑\u{11133}𑄢𑄣\u{11134}"), ("ceb", "Central Region"), ("de", "Central Region"), ("el", "Κεντρική Περιφέρεια"), ("en", "Central"), ("es", "Región Central"), ("fa", "منطقه مرکزی مالاوی"), ("fi", "Keskinen alue"), ("fr", "Malawi central"), ("gl", "Rexión Central"), ("id", "Region Tengah"), ("it", "regione Centrale"), ("ja", "中部州"), ("ka", "მალავის ცენტრალური რეგიონი"), ("ko", "중부 주"), ("lt", "Centrinis regionas"), ("nb", "Central"), ("nl", "Central"), ("no", "Central"), ("pt", "Região Central"), ("ro", "Regiunea Central"), ("ru", "Центральный регион"), ("sr", "Централни регион"), ("sr_Latn", "Centralni region"), ("sv", "Central Region"), ("tr", "Merkez Bölgesi"), ("uk", "Центральний регіон"), ("ur", "وسطی علاقہ، ملاوی"), ("zh", "中央区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "CK",
                    Subdivision{
                        name: "CK",
                        country_alpha2: Alpha2::MW,
                        code: "CK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.035), longitude: Some(34.801), max_latitude: Some(-16.0070408), min_latitude: Some(-16.0552991), max_longitude: Some(34.8167897), min_longitude: Some(34.7784233)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيكواوا"), ("az", "Chikwawa"), ("bg", "Чикуауа"), ("bn", "চিক\u{9be}ওয\u{9bc}\u{9be}ল\u{9be} জেল\u{9be}"), ("ccp", "𑄌𑄇\u{11134}𑄤𑄤"), ("ceb", "Chikwawa District"), ("da", "Chikwawa District"), ("de", "Chikwawa-Distrikt"), ("el", "Τσικβάβα"), ("en", "Chikwawa"), ("es", "Distrito de Chikwawa"), ("fa", "بخش چیکواوا"), ("fi", "Chikwawan piirikunta"), ("fr", "District de Chikwawa"), ("gl", "Distrito de Chikwawa"), ("gu", "ચિકવાવા જિલ\u{acd}લો"), ("hi", "चिकवावा जिला"), ("id", "Distrik Chikwawa"), ("it", "Distretto di Chikwawa"), ("ja", "チクワワ"), ("ka", "ჩიკუაუის რაიონი"), ("kn", "ಚ\u{cbf}ಕ\u{ccd}ವಾವಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "치콰와 현"), ("lt", "Čikvavos apskritis"), ("lv", "Čikvavas distrikts"), ("mr", "चिखवा जिल\u{94d}हा"), ("ms", "Chikwawa District"), ("nb", "Chikwawa"), ("nl", "Chikwawa"), ("no", "Chikwawa"), ("pl", "Dystrykt Chikwawa"), ("pt", "Chikwawa"), ("ro", "Districtul Chikwawa"), ("ru", "Чиквава"), ("si", "ච\u{dca}කව\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Chikwawa"), ("ta", "சிகிவ\u{bbe}வ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "చ\u{c3f}క\u{c4d}వ\u{c3e}వ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ช\u{e34}กวาวา"), ("tr", "Chikwawa"), ("uk", "Район Чіквава"), ("ur", "چیکواوا ضلع"), ("vi", "Quận Chikwawa"), ("zh", "奇克瓦瓦縣")]),
                        unofficial_name_list: ["Chikwawa"].to_vec(),
                    }
                ),
                (
                    "CR",
                    Subdivision{
                        name: "CR",
                        country_alpha2: Alpha2::MW,
                        code: "CR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.6759485), longitude: Some(35.1406293), max_latitude: Some(-15.6594037), min_latitude: Some(-15.6875012), max_longitude: Some(35.1468944), min_longitude: Some(35.1309299)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم تشيرازولو"), ("az", "Chiradzulu"), ("bg", "Чирадзулу"), ("bn", "কির\u{9be}দজ\u{9c1}ল\u{9c1} জেল\u{9be}"), ("ccp", "𑄌\u{11128}𑄢𑄖\u{11134}𑄎\u{1112a}𑄣\u{1112a}"), ("ceb", "Chiradzulu District"), ("da", "Chiradzulu District"), ("de", "Chiradzulu-Distrikt"), ("el", "Τσιραντζούλου"), ("en", "Chiradzulu"), ("es", "Distrito de Chiradzulu"), ("fa", "بخش چیرادزولو"), ("fi", "Chiradzulun piirikunta"), ("fr", "District de Chiradzulu"), ("gl", "Distrito de Chiradzulu"), ("gu", "ચિરાડઝ\u{ac1}લ\u{ac1} જિલ\u{acd}લો"), ("hi", "चिरादज\u{93c}\u{941}ल\u{941} जिला"), ("id", "Distrik Chiradzulu"), ("it", "Distretto di Chiradzulu"), ("ja", "チラズル"), ("ka", "ჩირაძულუს რაიონი"), ("kn", "ಚ\u{cbf}ರಾದ\u{ccd}ಜುಲು ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "치라줄루 현"), ("lt", "Čiradzulo apskritis"), ("lv", "Čiradzulu distrikts"), ("mr", "चिराडझल\u{942} जिल\u{94d}हा"), ("ms", "Chiradzulu District"), ("nb", "Chiradzulu"), ("nl", "Chiradzulu"), ("no", "Chiradzulu"), ("pl", "Dystrykt Chiradzulu"), ("pt", "Chiradzulu"), ("ro", "Districtul Chiradzulu"), ("ru", "Чирадзулу"), ("si", "ච\u{dd2}රඩ\u{dca}ස\u{dd4}ල\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Chiradzulu"), ("ta", "சிரதஸுலு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "చ\u{c3f}ర\u{c3e}డ\u{c4d}\u{200c}జులు జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ช\u{e34}ราดซ\u{e39}ล\u{e39}"), ("tr", "Chiradzulu"), ("uk", "Район Чірадзулу"), ("ur", "چیرادزولو ضلع"), ("vi", "Quận Chiradzulu"), ("zh", "奇拉朱盧縣")]),
                        unofficial_name_list: ["Chiradzulu"].to_vec(),
                    }
                ),
                (
                    "CT",
                    Subdivision{
                        name: "CT",
                        country_alpha2: Alpha2::MW,
                        code: "CT",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.7037655), longitude: Some(33.2700253), max_latitude: Some(-9.6740303), min_latitude: Some(-9.7162481), max_longitude: Some(33.2946682), min_longitude: Some(33.2463455)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشيتيبا"), ("az", "Chitipa"), ("bg", "Читипа"), ("bn", "কিটিপ\u{9be} জেল\u{9be}"), ("ccp", "𑄌\u{11128}𑄑\u{11128}𑄛"), ("ceb", "Chitipa District"), ("da", "Chitipa District"), ("de", "Chitipa-Distrikt"), ("el", "Τσιτίπα"), ("en", "Chitipa"), ("es", "Distrito de Chitipa"), ("fa", "بخش چیتیپا"), ("fi", "Chitipan piirikunta"), ("fr", "District de Chitipa"), ("gl", "Distrito de Chitipa"), ("gu", "ચિતીપા જિલ\u{acd}લો"), ("hi", "चितिपा जिला"), ("id", "Distrik Chitipa"), ("it", "Distretto di Chitipa"), ("ja", "チティパ"), ("ka", "ჩიტიპის რაიონი"), ("kn", "ಚ\u{cbf}ತ\u{cbf}ಪಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "치티파 현"), ("lt", "Čitipos apskritis"), ("lv", "Čitipas distrikts"), ("mr", "चिट\u{94d}टीपा जिल\u{94d}हा"), ("ms", "Chitipa District"), ("nb", "Chitipa"), ("nl", "Chitipa"), ("no", "Chitipa"), ("pl", "Dystrykt Chitipa"), ("pt", "Chitipa"), ("ro", "Districtul Chitipa"), ("ru", "Читипа"), ("si", "ච\u{dd2}ට\u{dd2}ප\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Читипа"), ("sr_Latn", "Čitipa"), ("sv", "Chitipa"), ("ta", "நகரம\u{bcd}ப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "చ\u{c3f}ట\u{c3f}ప\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตช\u{e34}ท\u{e34}พา"), ("tr", "Chitipa"), ("uk", "Район Чітіпа"), ("ur", "چیتیپا ضلع"), ("vi", "Quận Chitipa"), ("zh", "奇蒂帕縣")]),
                        unofficial_name_list: ["Chitipa"].to_vec(),
                    }
                ),
                (
                    "DE",
                    Subdivision{
                        name: "DE",
                        country_alpha2: Alpha2::MW,
                        code: "DE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.3816618), longitude: Some(34.3254842), max_latitude: Some(-14.3712497), min_latitude: Some(-14.398852), max_longitude: Some(34.34515), min_longitude: Some(34.3036938)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ديدزا"), ("az", "Dedza"), ("bg", "Дедза"), ("bn", "ডেডজ\u{9be} জেল\u{9be}"), ("ccp", "𑄓𑄬𑄖\u{11134}𑄎"), ("ceb", "Dedza District"), ("da", "Dedza District"), ("de", "Distrikt Dedza"), ("el", "Ντέντζα"), ("en", "Dedza"), ("es", "Distrito de Dedza"), ("fa", "بخش ددزا"), ("fi", "Dedzan piirikunta"), ("fr", "District de Dedza"), ("gl", "Distrito de Dedza"), ("gu", "ડ\u{ac7}ઝા જિલ\u{acd}લો"), ("hi", "द\u{947}ड\u{94d}ज\u{93c}ा जिला"), ("id", "Distrik Dedza"), ("is", "Dedza"), ("it", "Distretto di Dedza"), ("ja", "デッザ"), ("ka", "დედზის რაიონი"), ("kn", "ಡ\u{cbf}ದ\u{ccd}ಝಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "데자 현"), ("lt", "Dedzo apskritis"), ("lv", "Dedzas distrikts"), ("mr", "ड\u{947}झा जिल\u{94d}हा"), ("ms", "Dedza District"), ("nb", "Dedza"), ("nl", "Dedza"), ("no", "Dedza"), ("pl", "Dystrykt Dedza"), ("pt", "Dedza"), ("ro", "Districtul Dedza"), ("ru", "Дедза"), ("si", "ඩෙඩ\u{dca}ස\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Dedza"), ("ta", "டெட\u{bcd}ஜ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "డ\u{c46}డ\u{c4d}జ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อำเภอเดดซา"), ("tr", "Dedza"), ("uk", "Район Дедза"), ("ur", "دیدزا ضلع"), ("vi", "Quận Dedza"), ("zh", "代扎縣")]),
                        unofficial_name_list: ["Dedza"].to_vec(),
                    }
                ),
                (
                    "DO",
                    Subdivision{
                        name: "DO",
                        country_alpha2: Alpha2::MW,
                        code: "DO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.6514687), longitude: Some(33.9359756), max_latitude: Some(-13.6396478), min_latitude: Some(-13.6647528), max_longitude: Some(33.9471531), min_longitude: Some(33.9223479)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة دوا"), ("az", "Dowa"), ("bg", "Дова"), ("bn", "ড\u{9c1}ওয\u{9bc}\u{9be} জেল\u{9be}"), ("ccp", "𑄓\u{1112e}𑄤"), ("ceb", "Dowa District"), ("da", "Dowa District"), ("de", "Distrikt Dowa"), ("el", "Ντόουα"), ("en", "Dowa"), ("es", "Distrito de Dowa"), ("fa", "بخش دووا"), ("fi", "Dowan piirikunta"), ("fr", "District de Dowa"), ("gl", "Distrito de Dowa"), ("gu", "ડોવા જિલ\u{acd}લો"), ("hi", "डोवा जिला"), ("id", "Distrik Dowa"), ("it", "Distretto di Dowa"), ("ja", "ドーワ"), ("ka", "დოვის რაიონი"), ("kn", "ಡೋವಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "도와 현"), ("lt", "Dovos apskritis"), ("lv", "Dovas reģions"), ("mr", "डोवा जिल\u{94d}हा"), ("ms", "Daerah Dowa"), ("nb", "Dowa"), ("nl", "Dowa"), ("no", "Dowa"), ("pl", "Dystrykt Dowa"), ("pt", "Dowa"), ("ro", "Districtul Dowa"), ("ru", "Дова"), ("si", "දොව\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Dowa"), ("ta", "டோவ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "డ\u{c4b}వ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโดวา"), ("tr", "Dowa"), ("uk", "Район Дова"), ("ur", "دوا ضلع"), ("vi", "Quận Dowa"), ("zh", "多瓦縣")]),
                        unofficial_name_list: ["Dowa"].to_vec(),
                    }
                ),
                (
                    "KR",
                    Subdivision{
                        name: "KR",
                        country_alpha2: Alpha2::MW,
                        code: "KR",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-9.933333), longitude: Some(33.933333), max_latitude: Some(-9.9243401), min_latitude: Some(-9.967624899999999), max_longitude: Some(33.94612310000001), min_longitude: Some(33.8861703)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Karonga"), ("bg", "Каронга"), ("ccp", "𑄇𑄢\u{11127}\u{11101}𑄉"), ("ceb", "Karonga District"), ("en", "Karonga"), ("es", "Distrito de Karonga"), ("fa", "بخش کارونگا"), ("fi", "Karongan piirikunta"), ("fr", "District de Karonga"), ("gl", "Distrito de Karonga"), ("id", "Distrik Karonga"), ("it", "Distretto di Karonga"), ("ja", "カロンガ"), ("ka", "კარონგის რაიონი"), ("ko", "카롱가 현"), ("nb", "Karonga"), ("nl", "Karonga"), ("no", "Karonga"), ("pl", "Dystrykt Karonga"), ("pt", "Karonga"), ("ro", "Districtul Karonga"), ("ru", "Каронга"), ("sr", "Каронга"), ("sr_Latn", "Karonga"), ("sv", "Karonga"), ("tr", "Karonga"), ("zh", "卡龍加縣")]),
                        unofficial_name_list: ["Karonga"].to_vec(),
                    }
                ),
                (
                    "KS",
                    Subdivision{
                        name: "KS",
                        country_alpha2: Alpha2::MW,
                        code: "KS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.033333), longitude: Some(33.483333), max_latitude: Some(-13.007652), min_latitude: Some(-13.0698636), max_longitude: Some(33.5083008), min_longitude: Some(33.4539701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كاسونغو"), ("az", "Kasungu"), ("be", "раён Касунгу"), ("bg", "Касунгу"), ("bn", "ক\u{9be}সোগ\u{9c1} জেল\u{9be}"), ("ccp", "𑄇𑄥\u{1112a}𑄚\u{11134}𑄉\u{1112a}"), ("ceb", "Kasungu District"), ("da", "Kasungu District"), ("de", "Kasungu-Distrikt"), ("el", "Κασουνγκού"), ("en", "Kasungu"), ("es", "Distrito de Kasungu"), ("fa", "بخش کاسونگو"), ("fi", "Kasungun piirikunta"), ("fr", "District de Kasungu"), ("gl", "Distrito de Kasungu"), ("gu", "કાસ\u{ac1}ન\u{acd}ગ\u{ac1} જિલ\u{acd}લો"), ("hi", "कस\u{941}\u{902}ग\u{941} जिला"), ("id", "Distrik Kasungu"), ("it", "Distretto di Kasungu"), ("ja", "カスング"), ("ka", "კასუნგუს რაიონი"), ("kn", "ಕಸ\u{ccd}ಸಂಗ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "카숭구 현"), ("lt", "Kasungo apskritis"), ("lv", "Kasungu distrikts"), ("mr", "कास\u{902}ग\u{941} जिल\u{94d}हा"), ("ms", "Daerah Kasungu"), ("nb", "Kasungu"), ("nl", "Kasungu"), ("no", "Kasungu"), ("pl", "Dystrykt Kasungu"), ("pt", "Kasungu"), ("ro", "Districtul Kasungu"), ("ru", "Касунгу"), ("si", "කස\u{dd4}න\u{dca}ග\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Kasungu"), ("ta", "க\u{bbe}சுக\u{bcd}கு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "కసుంగు జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตคาซ\u{e38}นก\u{e39}"), ("tr", "Kasungu"), ("uk", "Район Касунгу"), ("ur", "کاسونجو ضلع"), ("vi", "Quận Kasungu"), ("zh", "卡松古縣")]),
                        unofficial_name_list: ["Kasungu"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::MW,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.983333), longitude: Some(33.783333), max_latitude: Some(-13.8624138), min_latitude: Some(-14.0644008), max_longitude: Some(33.8478469), min_longitude: Some(33.7162716)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليلونغوي"), ("az", "Lilongwe"), ("be", "раён Лілонгвэ"), ("bg", "Лилонгве"), ("bn", "লিলংওয\u{9bc}ে জেল\u{9be}"), ("ccp", "𑄣\u{11128}𑄣\u{11127}\u{11101}𑄅\u{1112a}𑄃\u{11128}"), ("ceb", "Lilongwe District"), ("da", "Lilongwe District"), ("de", "Distrikt Lilongwe"), ("el", "Λιλόνγκουε"), ("en", "Lilongwe"), ("es", "Distrito de Lilongwe"), ("fa", "بخش لیلونگوه"), ("fi", "Lilongwen piirikunta"), ("fr", "District de Lilongwe"), ("gl", "Distrito de Lilongwe"), ("gu", "લીલો\u{a82}ગવ\u{ac7} જિલ\u{acd}લો"), ("hi", "लिलो\u{902}ग\u{94d}व\u{947} जिला"), ("id", "Distrik Lilongwe"), ("it", "Distretto di Lilongwe"), ("ja", "リロングウェ県"), ("ka", "ლილონგვეს რაიონი"), ("kn", "ಲ\u{cbf}ಲೊಂಗ\u{ccd}ವ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "릴롱궤 현"), ("lt", "Lilongvės apskritis"), ("lv", "Lilongves distrikts"), ("mr", "लिला\u{902}गॉ जिल\u{94d}हा"), ("ms", "Daerah Lilongwe"), ("nb", "Lilongwe"), ("nl", "Lilongwe"), ("no", "Lilongwe"), ("pl", "Dystrykt Lilongwe"), ("pt", "Lilongwe"), ("ro", "Districtul Lilongwe"), ("ru", "Лилонегве"), ("si", "ල\u{dd2}ලොන\u{dca}ග\u{dca}වේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Lilongwe"), ("ta", "லிலோங\u{bcd}வெ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3f}ల\u{c4b}ంగ\u{c4d}వ\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตล\u{e34}ลองเว"), ("tr", "Lilongwe"), ("uk", "Район Лилонгве"), ("ur", "لیلونجوی ضلع"), ("vi", "Quận Lilongwe"), ("zh", "利隆圭縣")]),
                        unofficial_name_list: ["Lilongwe"].to_vec(),
                    }
                ),
                (
                    "LK",
                    Subdivision{
                        name: "LK",
                        country_alpha2: Alpha2::MW,
                        code: "LK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.0584005), longitude: Some(34.7354031), max_latitude: Some(-12.0311794), min_latitude: Some(-12.1000348), max_longitude: Some(34.7604815), min_longitude: Some(34.7087596)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليكوما"), ("az", "Likoma"), ("bg", "Ликома"), ("bn", "লিক\u{9c1}ম\u{9be} জেল\u{9be}"), ("ccp", "𑄣\u{11128}𑄇\u{1112e}𑄟"), ("ceb", "Likoma District"), ("da", "Likoma"), ("de", "Likoma"), ("el", "Λικόμα"), ("en", "Likoma"), ("es", "Distrito de Likoma"), ("fa", "بخش لیکوما"), ("fi", "Likoman piirikunta"), ("fr", "District de Likoma"), ("gl", "Distrito de Likoma"), ("gu", "લિકોમા જિલ\u{acd}લો"), ("hi", "लिकोमा जिला"), ("id", "Distrik Likoma"), ("it", "Distretto di Likoma"), ("ja", "リコマ"), ("ka", "ლიკომის რაიონი"), ("kn", "ಲ\u{cbf}ಕೋಮಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "리코마 현"), ("lt", "Likomos apskritis"), ("lv", "Likomas distrikts"), ("mr", "लिकामा जिल\u{94d}हा"), ("ms", "Likoma District"), ("nb", "Likoma"), ("nl", "Likoma"), ("no", "Likoma"), ("pl", "Dystrykt Likoma"), ("pt", "Likoma"), ("ro", "Districtul Likoma"), ("ru", "Ликома"), ("si", "ල\u{dd2}කොම\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Ликома"), ("sr_Latn", "Likoma"), ("sv", "Likoma"), ("ta", "லிகோம\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c3f}క\u{c4b}మ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตล\u{e34}โกมา"), ("tr", "Likoma"), ("uk", "Район Лікома"), ("ur", "لیکوما ڈسٹرک"), ("vi", "Quận Likoma"), ("zh", "利科馬縣")]),
                        unofficial_name_list: ["Likoma Island"].to_vec(),
                    }
                ),
                (
                    "MC",
                    Subdivision{
                        name: "MC",
                        country_alpha2: Alpha2::MW,
                        code: "MC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.8000058), longitude: Some(32.8930441), max_latitude: Some(-13.7765674), min_latitude: Some(-13.8171608), max_longitude: Some(32.90946), min_longitude: Some(32.8671455)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مشينجي"), ("az", "Mchinji"), ("bg", "Мчинджи"), ("bn", "এম সিঞ\u{9cd}জি জেল\u{9be}"), ("ccp", "𑄌\u{11128}𑄚\u{11134}𑄎\u{11128}"), ("ceb", "Mchinji District"), ("da", "Mchinji District"), ("de", "Mchinji-Distrikt"), ("el", "Μτσίνζι"), ("en", "Mchinji"), ("es", "Distrito de Mchinji"), ("fa", "بخش مچینجی"), ("fi", "Mchinjin piirikunta"), ("fr", "District de Mchinji"), ("gl", "Distrito de Mchinji"), ("gu", "મચિ\u{a82}જી જિલ\u{acd}લો"), ("hi", "मची\u{902}जी जिला"), ("id", "Distrik Mchinji"), ("it", "Distretto di Mchinji"), ("ja", "ムチンジ"), ("ka", "მჩინჯის რაიონი"), ("kn", "ಮಚ\u{ccd}ಚ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "음친지 현"), ("lt", "Mčinio apskritis"), ("lv", "Mčindži distrikts"), ("mr", "म\u{94d}च\u{94d}चिजी जिल\u{94d}हा"), ("ms", "Daerah Mchinji"), ("nb", "Mchinji"), ("nl", "Mchinji"), ("no", "Mchinji"), ("pl", "Dystrykt Mchinji"), ("pt", "Mchinji"), ("ro", "Districtul Mchinji"), ("ru", "Мчиньи"), ("si", "ම\u{dd2}ච\u{dd2}න\u{dca}ජ\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Mchinji"), ("ta", "மசிஞ\u{bcd}சி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3e}చ\u{c3f}ంజ\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มช\u{e34}นจ\u{e34}"), ("tr", "Mchinji"), ("uk", "Район Мчіньї"), ("ur", "مچینجی ضلع"), ("vi", "Quận Mchinji"), ("zh", "姆欽吉縣")]),
                        unofficial_name_list: ["Mchinji"].to_vec(),
                    }
                ),
                (
                    "MG",
                    Subdivision{
                        name: "MG",
                        country_alpha2: Alpha2::MW,
                        code: "MG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.4861733), longitude: Some(35.253304), max_latitude: Some(-14.4638539), min_latitude: Some(-14.5005024), max_longitude: Some(35.2740097), min_longitude: Some(35.2314378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مانغوتشي"), ("az", "Mangochi"), ("bg", "Мангочи"), ("bn", "ম\u{9be}ঙ\u{9cd}গোচি জেল\u{9be}"), ("ccp", "𑄟\u{11101}𑄉\u{1112e}𑄌\u{11128}"), ("ceb", "Mangochi District"), ("da", "Mangochi District"), ("de", "Distrikt Mangochi"), ("el", "Μανγκότσι"), ("en", "Mangochi"), ("es", "Distrito de Mangochi"), ("fa", "بخش مانگوچی"), ("fi", "Mangochin piirikunta"), ("fr", "District de Mangochi"), ("gl", "Distrito de Mangochi"), ("gu", "મ\u{a82}ગોચી જિલ\u{acd}લો"), ("hi", "मन\u{94d}गोची जिला"), ("id", "Distrik Mangochi"), ("it", "Distretto di Mangochi"), ("ja", "マンゴチ"), ("ka", "მანგოჩის რაიონი"), ("kn", "ಮಂಗೊಚ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "망고치 현"), ("lt", "Magončio apskritis"), ("lv", "Mangonči distrikts"), ("mr", "म\u{902}गोची जिल\u{94d}हा"), ("ms", "Daerah Mangochi"), ("nb", "Mangochi"), ("nl", "Mangochi"), ("no", "Mangochi"), ("pl", "Dystrykt Mangochi"), ("pt", "Mangochi"), ("ro", "Districtul Mangochi"), ("ru", "Мангочи"), ("si", "මන\u{dca}ගොච\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Mangochi"), ("ta", "மண\u{bcd}கொச\u{bcd}சி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3e}ంగ\u{c4b}చ\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตแมนโกช\u{e34}"), ("tr", "Mangochi"), ("uk", "Район Мангочі"), ("ur", "مانجوچی ضلع"), ("vi", "Quận Mangochi"), ("zh", "曼戈切縣")]),
                        unofficial_name_list: ["Mangochi"].to_vec(),
                    }
                ),
                (
                    "MH",
                    Subdivision{
                        name: "MH",
                        country_alpha2: Alpha2::MW,
                        code: "MH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.1699934), longitude: Some(35.2999779), max_latitude: Some(-15.1613515), min_latitude: Some(-15.1822628), max_longitude: Some(35.31739719999999), min_longitude: Some(35.2879491)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ماشينكا"), ("az", "Machinga"), ("be", "раён Мачынга"), ("bg", "Мачинга"), ("bn", "মেশিগ\u{9be}ন জেল\u{9be}"), ("ccp", "𑄟𑄌\u{11128}\u{11101}𑄉"), ("ceb", "Machinga District"), ("da", "Machinga"), ("de", "Machinga"), ("el", "Ματσίνγκα"), ("en", "Machinga"), ("es", "Distrito de Machinga"), ("fa", "بخش ماچینگا"), ("fi", "Machingan piirikunta"), ("fr", "District de Machinga"), ("gl", "Distrito de Machinga"), ("gu", "મચીન\u{acd}ગા જિલ\u{acd}લો"), ("hi", "माचि\u{902}गा जिला"), ("id", "Distrik Machinga"), ("it", "Distretto di Machinga"), ("ja", "マチンガ"), ("ka", "მაჩინგის რაიონი"), ("kn", "ಮಚ\u{cbf}ಂಗ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "마칭가 현"), ("lt", "Mačingos apskritis"), ("lv", "Mačingas distrikts"), ("mr", "मचा\u{902}गा जिल\u{94d}हा"), ("ms", "Daerah Machinga"), ("nb", "Machinga"), ("nl", "Machinga"), ("no", "Machinga"), ("pl", "Dystrykt Machinga"), ("pt", "Machinga"), ("ro", "Districtul Machinga"), ("ru", "Мачинга"), ("si", "මච\u{dd2}න\u{dca}ග\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Machinga"), ("ta", "மசிங\u{bcd}க\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మచ\u{c3f}ంగ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มาช\u{e34}งกา"), ("tr", "Machinga"), ("uk", "Район Мачінга"), ("ur", "ماچینجا ضلع"), ("vi", "Quận Machinga"), ("zh", "馬欽加縣")]),
                        unofficial_name_list: ["Machinga"].to_vec(),
                    }
                ),
                (
                    "MU",
                    Subdivision{
                        name: "MU",
                        country_alpha2: Alpha2::MW,
                        code: "MU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.033333), longitude: Some(35.5), max_latitude: Some(-16.0180114), min_latitude: Some(-16.0381583), max_longitude: Some(35.5212391), min_longitude: Some(35.4918706)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مولاني"), ("az", "Mulanje"), ("bg", "Мулание"), ("bn", "মোল\u{9be}নজে জেল\u{9be}"), ("ccp", "𑄟\u{1112a}𑄣𑄚\u{11134}𑄎𑄬"), ("ceb", "Mulanje District"), ("da", "Mulanje District"), ("de", "Mulanje-Distrikt"), ("el", "Μουλάντζε"), ("en", "Mulanje"), ("es", "Distrito de Mulanje"), ("fa", "بخش مولانجه"), ("fi", "Mulanjen piirikunta"), ("fr", "District de Mulanje"), ("gl", "Distrito de Mulanje"), ("gu", "મ\u{ac1}લા\u{a82}જ\u{ac7} જિલ\u{acd}લો"), ("hi", "म\u{941}लान\u{94d}य\u{947} जिला"), ("id", "Distrik Mulanje"), ("it", "Distretto di Mulanje"), ("ja", "ムランジェ"), ("ka", "მულანჯეს რაიონი"), ("kn", "ಮುಲಾನ\u{ccd}ಜ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "물란제 현"), ("lt", "Mulandžės apskritis"), ("lv", "Mulandžes distrikts"), ("mr", "म\u{941}लान\u{947}\u{902} जिल\u{94d}हा"), ("ms", "Mulanje District"), ("nb", "Mulanje"), ("nl", "Mulanje"), ("no", "Mulanje"), ("pl", "Dystrykt Mulanje"), ("pt", "Mulanje"), ("ro", "Districtul Mulanje"), ("ru", "Муланье"), ("si", "ම\u{dd4}ලන\u{dca}ජේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Mulanje"), ("ta", "மூலஞ\u{bcd}சே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ముల\u{c3e}ంజ\u{c46} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตม\u{e39}ลานเจ"), ("tr", "Mulanje"), ("uk", "Район Муланье"), ("ur", "ملانجی ضلع"), ("vi", "Quận Mulanje"), ("zh", "姆蘭傑縣")]),
                        unofficial_name_list: ["Mulanje"].to_vec(),
                    }
                ),
                (
                    "MW",
                    Subdivision{
                        name: "MW",
                        country_alpha2: Alpha2::MW,
                        code: "MW",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.609722), longitude: Some(34.5225), max_latitude: Some(-15.5777469), min_latitude: Some(-15.6189038), max_longitude: Some(34.5392461), min_longitude: Some(34.4934655)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة موانزا"), ("az", "Mwanza"), ("bg", "Мванза"), ("bn", "এমওয\u{9bc}\u{9be}ঞ\u{9cd}জ\u{9be} জেল\u{9be}"), ("ccp", "𑄤𑄚\u{11134}𑄎"), ("ceb", "Mwanza District (distrito sa Malawi)"), ("da", "Mwanza District"), ("de", "Mwanza-Distrikt"), ("el", "Μουάνζα"), ("en", "Mwanza"), ("es", "Distrito de Mwanza"), ("fa", "بخش موانزا"), ("fi", "Mwanzan piirikunta"), ("fr", "District de Mwanza"), ("gl", "Distrito de Mwanza"), ("gu", "મ\u{acd}વા\u{a82}ઝા જિલ\u{acd}લો"), ("hi", "मवान\u{94d}ज\u{93c}ा जिला"), ("id", "Distrik Mwanza"), ("it", "Distretto di Mwanza"), ("ja", "ムワンザ"), ("ka", "მვანზის რაიონი"), ("kn", "ಮ\u{ccd}ವಾಂಝಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "음완자 현"), ("lt", "Manzos apskritis"), ("lv", "Mvanzas distrikts"), ("mr", "म\u{94d}व\u{94d}हाझा जिल\u{94d}हा"), ("ms", "Mwanza District"), ("nb", "Mwanza"), ("nl", "Mwanza"), ("no", "Mwanza"), ("pl", "Dystrykt Mwanza"), ("pt", "Mwanza"), ("ro", "Districtul Mwanza"), ("ru", "Мванза"), ("si", "ම\u{dca}වන\u{dca}ස\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Mwanza"), ("ta", "மவன\u{bcd}சங\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మవ\u{c3e}ంజ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตมว\u{e31}นซา"), ("tr", "Mwanza"), ("uk", "Район Мванза"), ("ur", "موانزا ضلع"), ("vi", "Quận Mwanza"), ("zh", "姆萬紮縣")]),
                        unofficial_name_list: ["Mwanza"].to_vec(),
                    }
                ),
                (
                    "MZ",
                    Subdivision{
                        name: "MZ",
                        country_alpha2: Alpha2::MW,
                        code: "MZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.9), longitude: Some(33.6), max_latitude: Some(-11.8862493), min_latitude: Some(-11.913209), max_longitude: Some(33.60408779999999), min_longitude: Some(33.5802269)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة مزيمبا"), ("az", "Mzimba"), ("be", "раён Мзімба"), ("bg", "Мзимба"), ("bn", "এমজিম\u{9cd}ব\u{9be} জেল\u{9be}"), ("ccp", "𑄎\u{11128}𑄟\u{11134}𑄝"), ("ceb", "Mzimba District"), ("da", "Mzimba District"), ("de", "Mzimba-Distrikt"), ("el", "Μζίμπα"), ("en", "Mzimba"), ("es", "Distrito de Mzimba"), ("fa", "بخش مزیمبا"), ("fi", "Mzimban piirikunta"), ("fr", "District de Mzimba"), ("gl", "Distrito de Mzimba"), ("gu", "મઝીબા જિલ\u{acd}લો"), ("hi", "ज\u{93c}िम\u{94d}बा जिला"), ("id", "Distrik Mzimba"), ("it", "Distretto di Mzimba"), ("ja", "ムジンバ"), ("ka", "მზიმბის რაიონი"), ("kn", "ಮ\u{cbf}ಜ\u{cbf}ಂಬ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "음짐바 현"), ("lt", "Mzimbos apskritis"), ("lv", "Mzimbas distrikts"), ("mr", "मझीबा जिल\u{94d}हा"), ("ms", "Mzimba District"), ("nb", "Mzimba"), ("nl", "Mzimba"), ("no", "Mzimba"), ("pl", "Dystrykt Mzimba"), ("pt", "Mzimba"), ("ro", "Districtul Mzimba"), ("ru", "Мзимба"), ("si", "මස\u{dd2}ම\u{dca}බ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Мзимба"), ("sr_Latn", "Mzimba"), ("sv", "Mzimba"), ("ta", "மஜிம\u{bcd}ப\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఎంజ\u{c3f}ంబ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเอ\u{e47}มซ\u{e34}มบ\u{e49}า"), ("tr", "Mzimba"), ("uk", "Район Мзімба"), ("ur", "مزیمبا ضلع"), ("vi", "Quận Mzimba"), ("zh", "姆津巴縣")]),
                        unofficial_name_list: ["Mzimba"].to_vec(),
                    }
                ),
                (
                    "N",
                    Subdivision{
                        name: "N",
                        country_alpha2: Alpha2::MW,
                        code: "N",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Şimal bölgəsi"), ("be", "Паўночны рэгіён"), ("bg", "Северен регион"), ("ca", "Regió del Nort"), ("ccp", "𑄅\u{1112a}𑄖\u{11133}𑄦\u{11127}𑄢\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Northern Region"), ("de", "Northern Region"), ("el", "Βόρεια Περιφέρεια"), ("en", "Northern"), ("es", "Región del Norte"), ("fa", "منطقه شمالی مالاوی"), ("fi", "Pohjoinen alue"), ("fr", "Région Nord"), ("gl", "Rexión Norte"), ("it", "regione Settentrionale"), ("ja", "北部州"), ("ka", "მალავის ჩრდილოეთი რეგიონი"), ("ko", "북부 주"), ("lt", "Šiaurės regionas"), ("nb", "Northern"), ("nl", "Northern"), ("no", "Northern"), ("pt", "Região Norte"), ("ro", "Regiunea Northern"), ("ru", "Северный регион"), ("sr", "Северни регион"), ("sr_Latn", "Severni region"), ("sv", "Northern Region"), ("tr", "Kuzey Bölgesi"), ("uk", "Північний регіон"), ("ur", "شمالی علاقہ، ملاوی"), ("zh", "北部区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NB",
                    Subdivision{
                        name: "NB",
                        country_alpha2: Alpha2::MW,
                        code: "NB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.6085556), longitude: Some(34.2949409), max_latitude: Some(-11.6009539), min_latitude: Some(-11.6133972), max_longitude: Some(34.3032989), min_longitude: Some(34.2894459)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة خليج نخاتا"), ("az", "Nkhata Bay"), ("bg", "Нхата Бей"), ("bn", "এন কে হ\u{9be}ত\u{9be} বে জেল\u{9be}"), ("ccp", "𑄈𑄑 𑄝𑄬"), ("ceb", "Nkhata Bay District"), ("da", "Nkhata Bay District"), ("de", "Nkhata Bay Distrikt"), ("el", "Κόλπος Νχάτα"), ("en", "Nkhata Bay"), ("es", "Distrito de Nkhata Bay"), ("fa", "بخش نکهاتا بای"), ("fi", "Nkhata Bayn piirikunta"), ("fr", "District de Nkhata Bay"), ("gl", "Distrito de Nkhata Bay"), ("gu", "નખાતા બ\u{ac7} જિલ\u{acd}લો"), ("hi", "खाता ब\u{947} जिला"), ("id", "Distrik Nkhata Bay"), ("it", "Distretto di Nkhata Bay"), ("ja", "カタベイ"), ("ka", "ნხატა ბაის რაიონი"), ("kn", "ನ\u{cc6}ಖತಾ ಬೇ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "은카타베이 현"), ("lt", "Nkata Bėjaus apskritis"), ("lv", "Nkhatabejas distrikts"), ("mr", "नखाता ब\u{947} जिल\u{94d}हा"), ("ms", "Nkhata Bay District"), ("nb", "Nkhata Bay"), ("nl", "Nkhata Bay"), ("no", "Nkhata Bay"), ("pl", "Dystrykt Nkhata Bay"), ("pt", "Nkhata Bay"), ("ro", "Districtul Nkhata Bay"), ("ru", "Нхата Бэй"), ("si", "එන\u{dca}ඛට\u{dcf} කලප\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Нхата Беј"), ("sr_Latn", "Nhata Bej"), ("sv", "Nkhata Bay"), ("ta", "நக\u{bbe}ட\u{bbe} பே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఎన\u{c4d}\u{200c}\u{200c}ఖ\u{c3e}ట\u{c3e} బ\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตคฮาทา เบย\u{e4c}"), ("tr", "Nkhata Bay"), ("uk", "Район Нхата-Бей"), ("ur", "نخاتا بے ضلع"), ("vi", "Quận Nkhata Bay"), ("zh", "恩卡塔灣縣")]),
                        unofficial_name_list: ["Nkhata Bay"].to_vec(),
                    }
                ),
                (
                    "NE",
                    Subdivision{
                        name: "NE",
                        country_alpha2: Alpha2::MW,
                        code: "NE",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نينو"), ("az", "Neno"), ("bg", "Нено"), ("bn", "নেনো জেল\u{9be}"), ("ccp", "𑄚𑄬𑄚\u{1112e}"), ("ceb", "Neno District"), ("da", "Neno District"), ("de", "Neno-Distrikt"), ("el", "Νένο"), ("en", "Neno"), ("es", "Distrito de Neno"), ("fa", "بخش ننو"), ("fi", "Nenon piirikunta"), ("fr", "District de Neno"), ("gl", "Distrito de Neno"), ("gu", "ન\u{ac7}નો જિલ\u{acd}લો"), ("hi", "न\u{947}नो जिला"), ("id", "Distrik Neno"), ("it", "Distretto di Neno"), ("ja", "ネノ"), ("ka", "ნენოს რაიონი"), ("kn", "ನ\u{cc6}ನೊ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "네노 현"), ("lt", "Neno apskritis"), ("lv", "Neno distrikts"), ("mr", "न\u{947}नो जिल\u{94d}हा"), ("ms", "Neno District"), ("nb", "Neno"), ("nl", "Neno District"), ("no", "Neno"), ("pl", "Dystrykt Neno"), ("pt", "Neno"), ("ro", "Districtul Neno"), ("ru", "Нено"), ("si", "නෙනෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Neno"), ("ta", "நேனோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "న\u{c46}న\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตน\u{e35}โน"), ("tr", "Neno"), ("uk", "Район Нено"), ("ur", "نینو ضلع"), ("vi", "Neno Quận")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "NI",
                    Subdivision{
                        name: "NI",
                        country_alpha2: Alpha2::MW,
                        code: "NI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.2841992), longitude: Some(33.88577470000001), max_latitude: Some(-13.0142031), min_latitude: Some(-13.534227), max_longitude: Some(34.131686), min_longitude: Some(33.703213)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نتشيسى"), ("az", "Ntchisi"), ("bg", "Нтчиси"), ("bn", "ছিসি জেল\u{9be}"), ("ccp", "𑄌\u{11128}𑄥\u{11128}"), ("ceb", "Ntchisi District"), ("da", "Ntchisi District"), ("de", "Ntchisi-Distrikt"), ("el", "Ντσίσι"), ("en", "Ntchisi"), ("es", "Distrito de Ntchisi"), ("fa", "بخش نتچیسی"), ("fi", "Ntchisin piirikunta"), ("fr", "District de Ntchisi"), ("gl", "Distrito de Ntchisi"), ("gu", "ન\u{acd}ટચીસી જિલ\u{acd}લો"), ("hi", "नटशिसी जिला"), ("id", "Distrik Ntchisi"), ("it", "Distretto di Ntchisi"), ("ja", "ンチシ"), ("ka", "ნტჩისის რაიონი"), ("kn", "ಎನ\u{ccd}ಟ\u{cbf}ಚ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "은치시 현"), ("lt", "Nčisio apskritis"), ("lv", "Ntsiči rajons"), ("mr", "न\u{94d}टचीसी जिल\u{94d}हा"), ("ms", "Daerah Ntchisi"), ("nb", "Ntchisi"), ("nl", "Ntchisi"), ("no", "Ntchisi"), ("pl", "Dystrykt Ntchisi"), ("pt", "Ntchisi"), ("ro", "Districtul Ntchisi"), ("ru", "Нтчиси"), ("si", "න\u{dca}ට\u{dca}ච\u{dd2}ස\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Ntchisi"), ("ta", "நட\u{bcd}சிசி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఎన\u{c4d}చ\u{c3f}జ\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตนช\u{e34}ส\u{e34}"), ("tr", "Ntchisi"), ("uk", "Район Нтчісі"), ("ur", "نتچیسی ضلع"), ("vi", "Quận Ntchisi"), ("zh", "恩奇斯縣")]),
                        unofficial_name_list: ["Ntchisi"].to_vec(),
                    }
                ),
                (
                    "NK",
                    Subdivision{
                        name: "NK",
                        country_alpha2: Alpha2::MW,
                        code: "NK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-12.9316863), longitude: Some(34.2810541), max_latitude: Some(-12.9116282), min_latitude: Some(-12.9423296), max_longitude: Some(34.2997456), min_longitude: Some(34.2674732)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نكوتاكوتا"), ("az", "Nkhotakota"), ("be", "раён Нхатакота"), ("bg", "Нхотакота"), ("bn", "কোট\u{9be}কোট\u{9be} জেল\u{9be}"), ("ccp", "𑄈\u{1112e}𑄑𑄇\u{1112e}𑄑"), ("ceb", "Nkhotakota District"), ("da", "Nkhotakota District"), ("de", "Nkhotakota"), ("el", "Νχοτακότα"), ("en", "Nkhotakota"), ("es", "Distrito de Nkhotakota"), ("fa", "بخش نخوتاکوتا"), ("fi", "Nkhotakotan piirikunta"), ("fr", "District de Nkhotakota"), ("gl", "Distrito de Nkhotakota"), ("gu", "નાખોટાકોટા જિલ\u{acd}લો"), ("hi", "खोताकोता जिला"), ("id", "Distrik Nkhotakota"), ("is", "Nkhotakota"), ("it", "Distretto di Nkhotakota"), ("ja", "コタコタ"), ("ka", "ნხოტაკოტის რაიონი"), ("kn", "ನ\u{cc6}ಕೊಟಕೋಟಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "은코타코타 현"), ("lt", "Nkotakotos apskritis"), ("lv", "Nkhotakotas distrikts"), ("mr", "नखोटाकोटा जिल\u{94d}हा"), ("ms", "Nkhotakota District"), ("nb", "Nkhotakota"), ("nl", "Nkhotakota"), ("no", "Nkhotakota"), ("pl", "Dystrykt Nkhotakota"), ("pt", "Nkhotakota"), ("ro", "Districtul Nkhotakota"), ("ru", "Нхотакота"), ("si", "න\u{dca}ඛෝටකොට\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Nkhotakota"), ("ta", "ந\u{bcd}கஹோடகோட\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "న\u{c3f}క\u{c4b}ట\u{c3e}క\u{c4b}ట\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตอ\u{e36}งโคทาโกตา"), ("tr", "Nkhotakota"), ("uk", "Район Нхотакота"), ("ur", "نخوتاکوتا ضلع"), ("vi", "Quận Nkhotakota"), ("zh", "恩科塔科塔縣")]),
                        unofficial_name_list: ["Nkhotakota"].to_vec(),
                    }
                ),
                (
                    "NS",
                    Subdivision{
                        name: "NS",
                        country_alpha2: Alpha2::MW,
                        code: "NS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.916667), longitude: Some(35.266667), max_latitude: Some(-16.8957132), min_latitude: Some(-16.9436398), max_longitude: Some(35.27349), min_longitude: Some(35.2404084)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نسانج"), ("az", "Nsanje"), ("bg", "Нсане"), ("bn", "স\u{9be}ঞ\u{9cd}জে ডিস\u{9cd}টিক\u{9cd}ট"), ("ccp", "𑄥𑄚\u{11134}𑄎𑄬"), ("ceb", "Nsanje District"), ("da", "Nsanje District"), ("de", "Nsanje-Distrikt"), ("el", "Νσάντζε"), ("en", "Nsanje"), ("es", "Distrito de Nsanje"), ("fa", "بخش نسانجه"), ("fi", "Nsanjen piirikunta"), ("fr", "District de Nsanje"), ("gl", "Distrito de Nsanje"), ("gu", "સા\u{a82}જ\u{ac7} જિલ\u{acd}લો"), ("hi", "नस\u{902}ज\u{947} जिला"), ("id", "Distrik Nsanje"), ("it", "Distretto di Nsanje"), ("ja", "ンサンジェ"), ("ka", "ნსანეს რაიონი"), ("kn", "ಎನ\u{ccd}ಸಾಂಜ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "은산제 현"), ("lt", "Nsanjės apskritis"), ("lv", "Nsandžes distrikts"), ("mr", "ना\u{902}झान जिल\u{94d}हा"), ("ms", "Daerah Nsanje"), ("nb", "Nsanje"), ("nl", "Nsanje"), ("no", "Nsanje"), ("pl", "Dystrykt Nsanje"), ("pt", "Nsanje"), ("ro", "Districtul Nsanje"), ("ru", "Нсанье"), ("si", "එන\u{dca}සන\u{dca}ජේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Nsanje"), ("ta", "ன\u{bcd}சஞ\u{bcd}சே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}న\u{c4d}య\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเอนแซนเจ"), ("tr", "Nsanje"), ("uk", "Район Нсанье"), ("ur", "نسانجی ضلع"), ("vi", "Quận Nsanje"), ("zh", "恩桑傑縣")]),
                        unofficial_name_list: ["Nsanje"].to_vec(),
                    }
                ),
                (
                    "NU",
                    Subdivision{
                        name: "NU",
                        country_alpha2: Alpha2::MW,
                        code: "NU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-14.8219765), longitude: Some(34.6358702), max_latitude: Some(-14.7955467), min_latitude: Some(-14.8403539), max_longitude: Some(34.6578313), min_longitude: Some(34.6122551)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نتشيو"), ("az", "Ntcheu"), ("bg", "Нтчеу"), ("bn", "এনটি চেঊ ডিস\u{9cd}ট\u{9cd}রি"), ("ccp", "𑄌𑄬𑄅\u{1112a}"), ("ceb", "Ntcheu District"), ("da", "Ntcheu District"), ("de", "Ntcheu-Distrikt"), ("el", "Ντσέου"), ("en", "Ntcheu"), ("es", "Distrito de Ntcheu"), ("fa", "بخش نتچئو"), ("fi", "Ntcheun piirikunta"), ("fr", "District de Ntcheu"), ("gl", "Distrito de Ntcheu"), ("gu", "નટ\u{acd}ચ\u{ac7}ઉ જિલ\u{acd}લો"), ("hi", "तिच\u{947}ऊ जिला"), ("id", "Distrik Ntcheu"), ("it", "Distretto di Ntcheu"), ("ja", "ンチェウ"), ("ka", "ნტჩეუს რაიონი"), ("kn", "ನ\u{ccd}ಟ\u{cc6}ಚ\u{ccd}ಯ\u{cc2} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "은체우 현"), ("lt", "Nčou apskritis"), ("lv", "Nčheu distrikts"), ("mr", "नटाच\u{947}उ जिल\u{94d}हा"), ("ms", "Ntcheu District"), ("nb", "Ntcheu"), ("nl", "Ntcheu"), ("no", "Ntcheu"), ("pl", "Dystrykt Ntcheu"), ("pt", "Ntcheu"), ("ro", "Districtul Ntcheu"), ("ru", "Нтчеу"), ("si", "න\u{dca}ට\u{dca}චේය\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Ntcheu"), ("ta", "நிட\u{bcd}சேக\u{bcd}கு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఎన\u{c4d}చ\u{c3f}యూ జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเน\u{e47}ทช\u{e34}ว"), ("tr", "Ntcheu"), ("uk", "Нтчеу"), ("ur", "نتچیو ضلع"), ("vi", "Quận Ntcheu"), ("zh", "恩徹烏縣")]),
                        unofficial_name_list: ["Ntcheu"].to_vec(),
                    }
                ),
                (
                    "PH",
                    Subdivision{
                        name: "PH",
                        country_alpha2: Alpha2::MW,
                        code: "PH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.7633303), longitude: Some(35.6587503), max_latitude: Some(-15.748579), min_latitude: Some(-15.7868579), max_longitude: Some(35.6769688), min_longitude: Some(35.638361)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فالومبي"), ("az", "Phalombe"), ("bg", "Паломбе"), ("bn", "প\u{9be}লোম\u{9cd}বে জেল\u{9be}"), ("ca", "districte de Phalombe"), ("ccp", "𑄜𑄣\u{1112e}𑄟\u{11134}𑄝𑄬"), ("ceb", "Phalombe District"), ("da", "Phalombe District"), ("de", "Distrikt Phalombe"), ("el", "Φαλόμπε"), ("en", "Phalombe"), ("es", "Distrito de Phalombe"), ("fa", "بخش پهالومبه"), ("fi", "Phalomben piirikunta"), ("fr", "District de Phalombe"), ("gl", "Distrito de Phalombe"), ("gu", "ફાલોમ\u{acd}બ\u{ac7} જિલ\u{acd}લો"), ("hi", "फलोम\u{94d}बी जिला"), ("id", "Distrik Phalombe"), ("it", "Distretto di Phalombe"), ("ja", "パロンベ"), ("ka", "ფალომბეს რაიონი"), ("kn", "ಫಲೋಂಬ\u{cc6} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "팔롬베 현"), ("lt", "Falombės apskritis"), ("lv", "Falombes distrikts"), ("mr", "फलोमब\u{947} जिल\u{94d}हा"), ("ms", "Phalombe District"), ("nb", "Phalombe"), ("nl", "Phalombe"), ("no", "Phalombe"), ("pl", "Dystrykt Phalombe"), ("pt", "Phalombe"), ("ro", "Districtul Phalombe"), ("ru", "Фаломбе"), ("si", "පලොඹේ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Phalombe"), ("ta", "ப\u{bbe}லம\u{bcd}பே ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c3e}ల\u{c4b}ంబ\u{c47} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตปาล\u{e31}มเบ"), ("tr", "Phalombe"), ("uk", "Район Фаломбе"), ("ur", "پھالومبی ضلع"), ("vi", "Quận Phalombe"), ("zh", "法隆貝縣")]),
                        unofficial_name_list: ["Phalombe"].to_vec(),
                    }
                ),
                (
                    "RU",
                    Subdivision{
                        name: "RU",
                        country_alpha2: Alpha2::MW,
                        code: "RU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-11.016667), longitude: Some(33.866667), max_latitude: Some(-11.0083478), min_latitude: Some(-11.0211537), max_longitude: Some(33.8720083), min_longitude: Some(33.8431264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إقليم رومفي"), ("az", "Rumphi"), ("bg", "Румфи"), ("bn", "র\u{9c1}ম\u{9cd}পি জেল\u{9be}"), ("ccp", "𑄢\u{1112a}𑄟\u{11134}𑄜\u{11128}"), ("ceb", "Rumphi District"), ("da", "Rumphi"), ("de", "Rumphi-Distrikt"), ("el", "Ρούμφι"), ("en", "Rumphi"), ("es", "Distrito de Rumphi"), ("fa", "بخش رامفی"), ("fi", "Rumphin piirikunta"), ("fr", "District de Rumphi"), ("gl", "Distrito de Rumphi"), ("gu", "ર\u{ac1}મ\u{acd}ફી જિલ\u{acd}લો"), ("hi", "र\u{941}म\u{94d}फी जिला"), ("id", "Distrik Rumphi"), ("it", "Distretto di Rumphi"), ("ja", "ルンピ"), ("ka", "რუმფის რაიონი"), ("kn", "ರಂಫ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "룸피 현"), ("lt", "Rumfio apskritis"), ("lv", "Rumfi distrikts"), ("mr", "र\u{902}फि जिल\u{94d}हा"), ("ms", "Rumphi District"), ("nb", "Rumphi"), ("nl", "Rumphi"), ("no", "Rumphi"), ("pl", "Dystrykt Rumphi"), ("pt", "Rumphi"), ("ro", "Districtul Rumphi"), ("ru", "Румфи"), ("si", "රම\u{dca}ප\u{dca}හ\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Румфи"), ("sr_Latn", "Rumfi"), ("sv", "Rumphi"), ("ta", "ர\u{bbe}மபி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "రుంఫ\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "อำเภอร\u{e31}มฟ\u{e35}"), ("tr", "Rumphi"), ("uk", "Район Румфі"), ("ur", "رومپحی ضلع"), ("vi", "Quận Rumphi"), ("zh", "倫比縣")]),
                        unofficial_name_list: ["Rumphi"].to_vec(),
                    }
                ),
                (
                    "S",
                    Subdivision{
                        name: "S",
                        country_alpha2: Alpha2::MW,
                        code: "S",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Cənub bölgəsi"), ("be", "Паўднёвы рэгіён"), ("bg", "Южен регион"), ("ca", "Regió del Sud"), ("ccp", "𑄘\u{11127}𑄊\u{11128}𑄚\u{11134}𑄟𑄬𑄇\u{11134}𑄈𑄬"), ("ceb", "Southern Region"), ("de", "Southern Region"), ("el", "Νότια Περιφέρεια"), ("en", "Southern"), ("es", "Región del Sur"), ("fa", "منطقه جنوبی مالاوی"), ("fi", "Eteläinen alue"), ("fr", "Région Sud"), ("gl", "Rexión Sur"), ("it", "regione Meridionale"), ("ja", "南部州"), ("ka", "მალავის სამხრეთი რეგიონი"), ("ko", "남부 주"), ("lt", "Pietų regionas"), ("nb", "Southern"), ("nl", "Southern"), ("no", "Southern"), ("pt", "Região Sul"), ("ro", "Regiunea Southern"), ("ru", "Южный регион"), ("sr", "Јужни регион"), ("sr_Latn", "Južni region"), ("sv", "Southern Region"), ("tr", "Güney Bölgesi"), ("uk", "Південний регіон"), ("ur", "جنوبی علاقہ، ملاوی"), ("zh", "南部区")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SA",
                    Subdivision{
                        name: "SA",
                        country_alpha2: Alpha2::MW,
                        code: "SA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-13.7795533), longitude: Some(34.458641), max_latitude: Some(-13.7622286), min_latitude: Some(-13.7959897), max_longitude: Some(34.4751836), min_longitude: Some(34.4338989)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ساليما"), ("az", "Salima"), ("bg", "Салима"), ("bn", "স\u{9be}লিম\u{9be} জেল\u{9be}"), ("ccp", "𑄥𑄣\u{11128}𑄟"), ("ceb", "Salima District"), ("da", "Salima District"), ("de", "Distrikt Salima"), ("el", "Σαλίμα"), ("en", "Salima"), ("es", "Distrito de Salima"), ("fa", "بخش سالیما"), ("fi", "Saliman piirikunta"), ("fr", "District de Salima"), ("gl", "Distrito de Salima"), ("gu", "સલિમા જિલ\u{acd}લો"), ("hi", "सलीमा जिला"), ("id", "Distrik Salima"), ("it", "Distretto di Salima"), ("ja", "サリマ"), ("ka", "სალიმის რაიონი"), ("kn", "ಸಲ\u{cbf}ಮಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "살리마 현"), ("lt", "Salimos apskritis"), ("lv", "Salimas distrikts"), ("mr", "सलीमा जिल\u{94d}हा"), ("ms", "Daerah Salima"), ("nb", "Salima"), ("nl", "Salima"), ("no", "Salima"), ("pl", "Dystrykt Salima"), ("pt", "Salima"), ("ro", "Districtul Salima"), ("ru", "Салима"), ("si", "සල\u{dd2}ම\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Salima"), ("ta", "சல\u{bc0}ம\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}ల\u{c3f}మ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซาล\u{e34}มะ"), ("tr", "Salima"), ("uk", "Район Саліма"), ("ur", "سلیمہ ضلع"), ("vi", "Quận Salima"), ("zh", "薩利馬縣")]),
                        unofficial_name_list: ["Salima"].to_vec(),
                    }
                ),
                (
                    "TH",
                    Subdivision{
                        name: "TH",
                        country_alpha2: Alpha2::MW,
                        code: "TH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-16.066667), longitude: Some(35.133333), max_latitude: Some(-16.0617329), min_latitude: Some(-16.0789703), max_longitude: Some(35.1527309), min_longitude: Some(35.1302433)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Thyolo"), ("bg", "Тхиоло"), ("ccp", "𑄒\u{1112d}𑄠\u{1112e}𑄣\u{1112e}"), ("ceb", "Thyolo District"), ("en", "Thyolo"), ("es", "Distrito de Thyolo"), ("fa", "بخش تیولو"), ("fi", "Thyolon piirikunta"), ("fr", "District de Thyolo"), ("gl", "Distrito de Thyolo"), ("id", "Distrik Thyolo"), ("it", "Distretto di Thyolo"), ("ja", "チョロ"), ("ka", "ტიოლოს რაიონი"), ("ko", "티올로 현"), ("nb", "Thyolo"), ("nl", "Thyolo"), ("no", "Thyolo"), ("pl", "Dystrykt Thyolo"), ("pt", "Thyolo"), ("ro", "Districtul Thyolo"), ("ru", "Тайоло"), ("sv", "Thyolo"), ("tr", "Thyolo"), ("zh", "蒂約羅縣")]),
                        unofficial_name_list: ["Thyolo"].to_vec(),
                    }
                ),
                (
                    "ZO",
                    Subdivision{
                        name: "ZO",
                        country_alpha2: Alpha2::MW,
                        code: "ZO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-15.383333), longitude: Some(35.333333), max_latitude: Some(-15.3602598), min_latitude: Some(-15.4255095), max_longitude: Some(35.3924561), min_longitude: Some(35.2947378)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Zomba"), ("be", "раён Зомба"), ("bg", "Зомба"), ("ca", "districte de Zomba"), ("ccp", "𑄎\u{1112e}𑄟\u{11134}𑄝"), ("ceb", "Zomba District"), ("en", "Zomba"), ("es", "Distrito de Zomba"), ("fa", "بخش زومبا"), ("fi", "Zomban piirikunta"), ("fr", "District de Zomba"), ("gl", "Distrito de Zomba"), ("id", "Distrik Zomba"), ("it", "Distretto di Zomba"), ("ja", "ゾンバ県"), ("ka", "ზომბის რაიონი"), ("ko", "좀바 현"), ("nb", "Zomba"), ("nl", "Zomba"), ("no", "Zomba"), ("pt", "Zomba"), ("ro", "Districtul Zomba"), ("sr", "Зомба"), ("sr_Latn", "Zomba"), ("sv", "Zomba"), ("tr", "Zomba"), ("zh", "松巴縣")]),
                        unofficial_name_list: ["Zomba"].to_vec(),
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
#[cfg(feature = "mw")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::MW,
        alpha3: Alpha3::MWI,
        address_format: None,
        continent: Continent::Africa,
        country_code: 265,
        currency_code: CurrencyCode::MWK,
        gec: Some(GEC::MI),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::MAW),
        iso_long_name: "The Republic of Malawi",
        iso_short_name: "Malawi",
        official_language_list: ["en", "ny"].to_vec(),
        spoken_language_list: ["en", "ny"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Malawian"),
        number: "454",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternAfrica),
        un_locode: "MW",
        unofficial_name_list: ["Malawi", "マラウイ"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Malawi"),
            ("af", "Malawi"),
            ("ak", "Malawi"),
            ("am", "ማላዊ"),
            ("an", "Malawi"),
            ("ar", "ملاوي"),
            ("as", "ম\u{9be}ল\u{9be}ৱী"),
            ("ay", "Malawi"),
            ("az", "Malavi"),
            ("ba", "Malawi"),
            ("be", "Малаві"),
            ("bg", "Малави"),
            ("bi", "Malawi"),
            ("bn", "ম\u{9be}ল\u{9be}উই"),
            ("bn_IN", "ম\u{9be}ল\u{9be}উই"),
            ("br", "Malawi"),
            ("bs", "Malavi"),
            ("ca", "Malawi"),
            ("ce", "Малави"),
            ("ch", "Malawi"),
            ("cs", "Malawi"),
            ("cv", "Малави"),
            ("cy", "Malawi"),
            ("da", "Malawi"),
            ("de", "Malawi"),
            ("dv", "މ\u{7a6}ލ\u{7a7}ވ\u{7a9}"),
            ("dz", "མ་ལ་ཝ\u{f72}།"),
            ("ee", "Malawi"),
            ("el", "Μαλάουι"),
            ("en", "Malawi"),
            ("eo", "Malavio"),
            ("es", "Malaui"),
            ("et", "Malawi"),
            ("eu", "Malawi"),
            ("fa", "مالاوی"),
            ("ff", "Malawi"),
            ("fi", "Malawi"),
            ("fo", "Malavi"),
            ("fr", "Malawi"),
            ("fy", "Malawy"),
            ("ga", "An Mhaláiv"),
            ("gl", "Malaui"),
            ("gn", "Malawi"),
            ("gu", "માલાવી"),
            ("gv", "Malawi"),
            ("ha", "Malawi"),
            ("he", "מלאווי"),
            ("hi", "मलावी"),
            ("hr", "Malavi"),
            ("ht", "Malawi"),
            ("hu", "Malawi"),
            ("hy", "Մալավի"),
            ("ia", "Malawi"),
            ("id", "Malawi"),
            ("io", "Malawi"),
            ("is", "Malaví"),
            ("it", "Malawi"),
            ("iu", "Malawi"),
            ("ja", "マラウイ"),
            ("ka", "მალავი"),
            ("ki", "Malawi"),
            ("kk", "Малави"),
            ("kl", "Malawi"),
            ("km", "ម\u{17c9}ាឡាវ\u{17b8}"),
            ("kn", "ಮಲಾವ\u{cbf}"),
            ("ko", "말라위"),
            ("ku", "Malawî"),
            ("kv", "Malawi"),
            ("kw", "Malawi"),
            ("ky", "Малави"),
            ("lo", "Malawi"),
            ("lt", "Malavis"),
            ("lv", "Malāvija"),
            ("mi", "Marāwi"),
            ("mk", "Малави"),
            ("ml", "മല\u{d3e}വി"),
            ("mn", "Малави"),
            ("mr", "मलवी"),
            ("ms", "Malawi"),
            ("mt", "Malawi"),
            ("my", "မာလဝ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Malawi"),
            ("nb", "Malawi"),
            ("ne", "मालावी"),
            ("nl", "Malawi"),
            ("nn", "Malawi"),
            ("nv", "Malawi"),
            ("oc", "Malawi"),
            ("or", "ମ\u{b3e}ଲ\u{b3e}ଓ\u{b4d}ବ\u{b3f}"),
            ("pa", "ਮਾਲਾਵੀਆ"),
            ("pi", "मलावी"),
            ("pl", "Malawi"),
            ("ps", "مالاوي"),
            ("pt", "Malawi"),
            ("pt_BR", "Malaui"),
            ("ro", "Malawi"),
            ("ru", "Малави"),
            ("rw", "Malawi"),
            ("sc", "Malawi"),
            ("sd", "Malawi"),
            ("si", "මල\u{dcf}ව\u{dd2}"),
            ("sk", "Malawi"),
            ("sl", "Malavi"),
            ("so", "Malaawi"),
            ("sq", "Malaui"),
            ("sr", "Малави"),
            ("sv", "Malawi"),
            ("sw", "Malawi"),
            ("ta", "மல\u{bbe}வி"),
            ("te", "మలవ\u{c40}"),
            ("tg", "Малави"),
            ("th", "มาลาว\u{e35}"),
            ("ti", "ማላዊ"),
            ("tk", "Malawi"),
            ("tl", "Malawi"),
            ("tr", "Malavi"),
            ("tt", "Малави"),
            ("ug", "مالاۋى"),
            ("uk", "Малаві"),
            ("ur", "ملاوی"),
            ("uz", "Malavi"),
            ("ve", "Malawi"),
            ("vi", "Ma-la-uy"),
            ("wa", "Malawi"),
            ("wo", "Malawi"),
            ("xh", "Malawi"),
            ("yo", "Màláwì"),
            ("zh_CN", "马拉维"),
            ("zh_HK", "馬拉維"),
            ("zh_TW", "馬拉威"),
            ("zu", "IMalawi"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
