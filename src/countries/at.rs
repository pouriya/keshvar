// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Austria

#[cfg(all(feature = "at", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::AT;
    pub const ALPHA3: Alpha3 = Alpha3::AUT;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 43;
    pub const CURRENCY_CODE: &str = "EUR";
    pub const GEC: Option<GEC> = Some(GEC::AU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::AUT);
    pub const ISO_SHORT_NAME: &str = "Austria";
    pub const ISO_LONG_NAME: &str = "The Republic of Austria";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["de"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["de"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[1, 2, 3];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7, 8, 9, 10, 11, 12, 13];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Austrian");
    pub const NUMBER: &str = "040";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternEurope);
    pub const UN_LOCODE: &str = "AT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Austria",
        "Österreich",
        "Autriche",
        "オーストリア",
        "Oostenrijk",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Austria"),
        ("af", "Oostenryk"),
        ("ak", "Austria"),
        ("am", "\u{1316}ስትሱ።"),
        ("an", "Austria"),
        ("ar", "الن\u{651}مسا"),
        ("as", "অস\u{9cd}ট\u{9cd}ৰিয়\u{9be}"),
        ("ay", "Austria"),
        ("az", "Avstriya"),
        ("ba", "Austria"),
        ("be", "Аўстрыя"),
        ("bg", "Австрия"),
        ("bi", "Austria"),
        ("bn", "অস\u{9cd}ট\u{9cd}রিয়\u{9be}"),
        ("bn_IN", "অস\u{9cd}ট\u{9cd}রিয়\u{9be}"),
        ("br", "Aostria"),
        ("bs", "Austrija"),
        ("ca", "Àustria"),
        ("ce", "Австри"),
        ("ch", "Austria"),
        ("cs", "Rakousko"),
        ("cv", "Австри"),
        ("cy", "Awstria"),
        ("da", "Østrig"),
        ("de", "Österreich"),
        ("dv", "އ\u{7ae}ސ\u{7b0}ޓ\u{7b0}ރ\u{7a8}އ\u{7a7}"),
        ("dz", "ཨ\u{f71}ས\u{f72}་ཊ\u{f72}་ཡ།"),
        ("ee", "Austria"),
        ("el", "Αυστρία"),
        ("en", "Austria"),
        ("eo", "Aŭstrio"),
        ("es", "Austria"),
        ("et", "Austria"),
        ("eu", "Austria"),
        ("fa", "اتریش"),
        ("ff", "Otiris"),
        ("fi", "Itävalta"),
        ("fo", "Eysturríki"),
        ("fr", "Autriche"),
        ("fy", "Eastenryk"),
        ("ga", "An Ostair"),
        ("gl", "Austria"),
        ("gn", "Austria"),
        ("gu", "ઓસ\u{acd}ટ\u{acd}રિયા"),
        ("gv", "Yn Austeyr"),
        ("ha", "Austriya"),
        ("he", "אוסטריה"),
        ("hi", "ऑस\u{94d}ट\u{94d}रिया"),
        ("hr", "Austrija"),
        ("ht", "Otrich"),
        ("hu", "Ausztria"),
        ("hy", "Ավստրիա"),
        ("ia", "Austria"),
        ("id", "Austria"),
        ("io", "Austria"),
        ("is", "Austurríki"),
        ("it", "Austria"),
        ("iu", "Austria"),
        ("ja", "オーストリア"),
        ("ka", "ავსტრია"),
        ("ki", "Austria"),
        ("kk", "Австрия"),
        ("kl", "Austria"),
        ("km", "អ\u{17bc}ទ\u{17d2}រ\u{17b8}ស"),
        ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಯ"),
        ("ko", "오스트리아"),
        ("ku", "Awusturya"),
        ("kv", "Австрия"),
        ("kw", "Estrych"),
        ("ky", "Австрия"),
        ("lo", "ປະເທດໂອຕະລ\u{eb4}ດ"),
        ("lt", "Austrija"),
        ("lv", "Austrija"),
        ("mi", "Ateria"),
        ("mk", "Австрија"),
        ("ml", "ഓസ\u{d4d}ട\u{d4d}രിയ"),
        ("mn", "Австри"),
        ("mr", "ऑस\u{94d}ट\u{94d}रिया"),
        ("ms", "Austria"),
        ("mt", "Awtrija"),
        ("my", "သြစတြ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Oteriya"),
        ("nb", "Østerrike"),
        ("ne", "अष\u{94d}ट\u{94d}रिया"),
        ("nl", "Oostenrijk"),
        ("nn", "Austerrike"),
        ("nv", "Óóswiya"),
        ("oc", "Àustria"),
        ("or", "ଅଷ\u{b4d}ଟ\u{b4d}ର\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਆਸਟਰੀਆ"),
        ("pi", "आस\u{94d}ट\u{94d}रिया"),
        ("pl", "Austria"),
        ("ps", "اتریش"),
        ("pt", "Áustria"),
        ("pt_BR", "Áustria"),
        ("ro", "Austria"),
        ("ru", "Австрия"),
        ("rw", "Ositiriya"),
        ("sc", "Àustria"),
        ("sd", "آسٽريا"),
        ("si", "ඔස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ය\u{dcf}ව"),
        ("sk", "Rakúsko"),
        ("sl", "Avstrija"),
        ("so", "Osteeriya"),
        ("sq", "Austri"),
        ("sr", "Аустрија"),
        ("sv", "Österrike"),
        ("sw", "Austria"),
        ("ta", "ஆஸ\u{bcd}திரிய\u{bbe}"),
        ("te", "ఓస\u{c4d}ట\u{c4d}ర\u{c3f}య\u{c3e}"),
        ("tg", "Австрия"),
        ("th", "ออสเตร\u{e35}ย"),
        ("ti", "ኦስትሪያ"),
        ("tk", "Awstriýa"),
        ("tl", "Austriya"),
        ("tr", "Avusturya"),
        ("tt", "Аустриа"),
        ("ug", "ئاۋسترىيە"),
        ("uk", "Австрія"),
        ("ur", "آسٹریا"),
        ("uz", "Avstriya"),
        ("ve", "Austria"),
        ("vi", "Ao"),
        ("wa", "Otriche"),
        ("wo", "Ótriis"),
        ("xh", "Austria"),
        ("yo", "Austríà"),
        ("zh_CN", "奥地利"),
        ("zh_HK", "奧地利"),
        ("zh_TW", "奧地利"),
        ("zu", "I-Ostriya"),
    ];
    #[cfg(all(feature = "at", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 47.516231;
        pub const LONGITUDE: f64 = 14.550072;
        pub const MAX_LATITUDE: f64 = 49.0206081;
        pub const MAX_LONGITUDE: f64 = 17.1607329;
        pub const MIN_LATITUDE: f64 = 46.37233579999999;
        pub const MIN_LONGITUDE: f64 = 9.530783399999999;
        pub const NORTHEAST_LATITUDE: f64 = 49.0206081;
        pub const NORTHEAST_LONGITUDE: f64 = 17.1607329;
        pub const SOUTHWEST_LATITUDE: f64 = 46.37233579999999;
        pub const SOUTHWEST_LONGITUDE: f64 = 9.530783399999999;
    }
}
#[cfg(all(feature = "at", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 47.516231,
            longitude: 14.550072,
            max_latitude: 49.0206081,
            max_longitude: 17.1607329,
            min_latitude: 46.37233579999999,
            min_longitude: 9.530783399999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 49.0206081,
                    longitude: 17.1607329,
                },
                southwest: CountryGeoBound {
                    latitude: 46.37233579999999,
                    longitude: 9.530783399999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "at", feature = "subdivisions"))]
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
                    "1",
                    Subdivision{
                        name: "1",
                        country_alpha2: Alpha2::AT,
                        code: "1",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.15371649999999), longitude: Some(16.2688797), max_latitude: Some(48.11879), min_latitude: Some(46.83047), max_longitude: Some(17.1603999), min_longitude: Some(15.99632)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Burgenland"), ("ar", "بورغنلاند"), ("be", "Бургенланд"), ("bg", "Бургенланд"), ("bn", "ব\u{9c1}র\u{9cd}গেনল\u{9cd}য\u{9be}ন\u{9cd}ড"), ("bs", "Gradišće"), ("ca", "Burgenland"), ("ccp", "𑄝\u{1112a}𑄢\u{11134}𑄉\u{11133}𑄠𑄚\u{11134}𑄣\u{11133}𑄠𑄚\u{11134}𑄓\u{11134}"), ("ceb", "Burgenland"), ("cs", "Burgenland"), ("cy", "Burgenland"), ("da", "Burgenland"), ("de", "Burgenland"), ("el", "Μπούργκενλαντ"), ("en", "Burgenland"), ("es", "Burgenland"), ("et", "Burgenland"), ("eu", "Burgenland"), ("fa", "بورگن\u{200c}لاند"), ("fi", "Burgenland"), ("fr", "Burgenland"), ("ga", "Burgenland"), ("gl", "Burgenland"), ("gu", "બર\u{acd}ગ\u{ac7}નલ\u{ac7}ન\u{acd}ડ"), ("he", "בורגנלנד"), ("hi", "ब\u{941}र\u{94d}ग\u{947}नल\u{948}\u{902}ड"), ("hr", "Gradišće"), ("hu", "Burgenland"), ("hy", "Բուրգենլանդ"), ("id", "Burgenland"), ("is", "Burgenland"), ("it", "Burgenland"), ("ja", "ブルゲンラント州"), ("ka", "ბურგენლანდი"), ("kn", "ಬರ\u{ccd}ಗ\u{cc6}ನ\u{ccd}ಲ\u{ccd}ಯಾಂಡ\u{ccd}"), ("ko", "부르겐란트 주"), ("lt", "Burgenlandas"), ("lv", "Burgenlande"), ("mk", "Градиште"), ("mn", "Бургенланд"), ("mr", "ब\u{941}र\u{94d}गनला\u{902}ड"), ("ms", "Burgenland"), ("nb", "Burgenland"), ("nl", "Burgenland"), ("no", "Burgenland"), ("pl", "Burgenland"), ("pt", "Burgenland"), ("ro", "Burgenland"), ("ru", "Бургенланд"), ("si", "බර\u{dca}ගන\u{dca}ලෑන\u{dca}ඩ\u{dca}"), ("sk", "Burgenland"), ("sl", "Gradiščanska"), ("sr", "Бургенланд"), ("sr_Latn", "Burgenland"), ("sv", "Burgenland"), ("ta", "பெர\u{bcd}ஜென\u{bcd}ல\u{bbe}ண\u{bcd}ட\u{bcd}"), ("te", "బర\u{c4d}జ\u{c46}న\u{c4d}ల\u{c3e}ండ\u{c4d}"), ("th", "ร\u{e31}ฐบ\u{e39}ร\u{e4c}เกนล\u{e31}นด\u{e4c}"), ("tr", "Burgenland"), ("uk", "Бургенланд"), ("ur", "بورگنلینڈ"), ("vi", "Burgenland"), ("yue", "布爾根蘭"), ("yue_Hans", "布尔根兰"), ("zh", "布尔根兰州")]),
                        unofficial_name_list: ["Burgenland"].to_vec(),
                    }
                ),
                (
                    "2",
                    Subdivision{
                        name: "2",
                        country_alpha2: Alpha2::AT,
                        code: "2",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.722203), longitude: Some(14.1805881), max_latitude: Some(47.13131), min_latitude: Some(46.3723), max_longitude: Some(15.0651401), min_longitude: Some(12.6563901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Karinthië"), ("ar", "كيرنتن"), ("be", "Карынтыя"), ("bg", "Каринтия"), ("bn", "ক\u{9cd}য\u{9be}রিন\u{9cd}থিয\u{9bc}\u{9be}"), ("bs", "Koruška"), ("ca", "Caríntia"), ("ccp", "𑄇\u{11133}𑄠𑄢\u{11128}𑄚\u{11134}𑄑\u{11128}𑄠"), ("ceb", "Kärnten"), ("cs", "Korutany"), ("cy", "Carinthia"), ("da", "Kärnten"), ("de", "Kärnten"), ("el", "Καρινθία"), ("en", "Carinthia"), ("es", "Carintia"), ("et", "Kärnteni liidumaa"), ("eu", "Karintia"), ("fa", "کرنتن"), ("fi", "Kärnten"), ("fr", "Carinthie"), ("ga", "Carinthia"), ("gl", "Carintia - Kärnten"), ("gu", "કાર\u{acd}ટ\u{ac7}ન"), ("he", "קרינתיה"), ("hi", "क\u{947}रिन\u{94d}थिया"), ("hr", "Koruška"), ("hu", "Karintia"), ("hy", "Կարինտիա"), ("id", "Kärnten"), ("is", "Kärnten"), ("it", "Carinzia"), ("ja", "ケルンテン州"), ("ka", "კარინტია"), ("kn", "ಕಾರ\u{ccd}ನ\u{ccd}ಟ\u{cc6}ನ\u{ccd}"), ("ko", "케른텐 주"), ("lt", "Karintija"), ("lv", "Karintija"), ("mk", "Корушка"), ("mn", "Каринт"), ("mr", "क\u{94d}यार\u{94d}न\u{94d}टन"), ("ms", "Kärnten"), ("nb", "Kärnten"), ("nl", "Karinthië"), ("no", "Kärnten"), ("pl", "Karyntia"), ("pt", "Caríntia"), ("ro", "Carintia"), ("ru", "Каринтия"), ("si", "කර\u{dd2}න\u{dca}ත\u{dd2}ය\u{dcf}"), ("sk", "Korutánsko"), ("sl", "Koroška"), ("sr", "Корушка"), ("sr_Latn", "Koruška"), ("sv", "Kärnten"), ("sw", "Karinthia"), ("ta", "க\u{bbe}ரின\u{bcd}திஆ"), ("te", "క\u{c46}ంర\u{c4d}ట\u{c46}న\u{c4d}"), ("th", "ร\u{e31}ฐคาร\u{e34}นเท\u{e35}ย"), ("tr", "Karintiya"), ("uk", "Каринтія"), ("ur", "کارنتھیا (ریاست)"), ("vi", "Kärnten"), ("yue", "克恩滕"), ("yue_Hans", "克恩滕"), ("zh", "克恩顿州")]),
                        unofficial_name_list: ["Carinthia", "Koroška"].to_vec(),
                    }
                ),
                (
                    "3",
                    Subdivision{
                        name: "3",
                        country_alpha2: Alpha2::AT,
                        code: "3",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.10807699999999), longitude: Some(15.8049558), max_latitude: Some(49.02062), min_latitude: Some(47.42198), max_longitude: Some(17.06847), min_longitude: Some(14.4521301)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Neder-Oostenryk"), ("ar", "النمسا السفلى"), ("be", "Ніжняя Аўстрыя"), ("bg", "Долна Австрия"), ("bn", "লোয\u{9bc}\u{9be}র অস\u{9cd}ট\u{9cd}রিয\u{9bc}\u{9be}"), ("bs", "Donja Austrija"), ("ca", "Baixa Àustria"), ("ccp", "𑄑\u{11127}𑄣\u{11134}𑄟\u{1112a}𑄇\u{11134}𑄈𑄬 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Niederösterreich"), ("cs", "Dolní Rakousy"), ("cy", "Awstria Isaf"), ("da", "Niederösterreich"), ("de", "Niederösterreich"), ("el", "Κάτω Αυστρία"), ("en", "Lower Austria"), ("es", "Baja Austria"), ("et", "Alam-Austria"), ("eu", "Austria Beherea"), ("fa", "نیدراسترایش"), ("fi", "Ala-Itävalta"), ("fr", "Basse-Autriche"), ("gl", "Baixa Austria"), ("gu", "લોઅર ઑસ\u{acd}ટ\u{acd}રિયા"), ("he", "אוסטריה תחתית"), ("hi", "निचला ऑस\u{94d}ट\u{94d}रिया"), ("hr", "Donja Austrija"), ("hu", "Alsó-Ausztria"), ("hy", "Ստորին Ավստրիա"), ("id", "Austria Hilir"), ("is", "Neðra Austurríki"), ("it", "Bassa Austria"), ("ja", "ニーダーエスターライヒ州"), ("ka", "ქვემო ავსტრია"), ("kk", "Төмеңгі Австрия"), ("kn", "ಕ\u{cc6}ಳಗ\u{cbf}ನ ಆಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಯಾ"), ("ko", "니더외스터라이히 주"), ("lt", "Žemutinė Austrija"), ("lv", "Lejasaustrija"), ("mk", "Долна Австрија"), ("mn", "Доор Австри"), ("mr", "नीडरओस\u{94d}टराईश"), ("ms", "Niederösterreich"), ("nb", "Niederösterreich"), ("nl", "Neder-Oostenrijk"), ("no", "Niederösterreich"), ("pl", "Dolna Austria"), ("pt", "Baixa Áustria"), ("ro", "Austria Inferioară"), ("ru", "Нижняя Австрия"), ("si", "පහල ඔස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ය\u{dcf}ව"), ("sk", "Dolné Rakúsko"), ("sl", "Spodnja Avstrija"), ("sr", "Доња Аустрија"), ("sr_Latn", "Donja Austrija"), ("sv", "Niederösterreich"), ("sw", "Austria Chini"), ("ta", "லோயர\u{bcd} ஆஸ\u{bcd}திரிய\u{bbe}"), ("te", "ల\u{c4b}యర\u{c4d} ఆస\u{c4d}ట\u{c4d}ర\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐโลเวอร\u{e4c}ออสเตร\u{e35}ย"), ("tk", "Aşaky Awstriýa"), ("tr", "Aşağı Avusturya"), ("uk", "Нижня Австрія"), ("ur", "زیریں آسٹریا"), ("vi", "Niederösterreich"), ("yue", "下奧地利"), ("yue_Hans", "下奥地利"), ("zh", "下奧地利州")]),
                        unofficial_name_list: ["Lower Austria"].to_vec(),
                    }
                ),
                (
                    "4",
                    Subdivision{
                        name: "4",
                        country_alpha2: Alpha2::AT,
                        code: "4",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.025854), longitude: Some(13.9723665), max_latitude: Some(48.7726901), min_latitude: Some(47.46098), max_longitude: Some(14.99129), min_longitude: Some(12.74895)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Opper-Oostenryk"), ("ar", "النمسا العليا"), ("be", "Верхняя Аўстрыя"), ("bg", "Горна Австрия"), ("bn", "আপ\u{9be}র অস\u{9cd}ট\u{9cd}রিয\u{9bc}\u{9be}"), ("bs", "Gornja Austrija"), ("ca", "Alta Àustria"), ("ccp", "𑄅\u{1112a}𑄉\u{1112a}𑄢𑄬 𑄟\u{1112a}𑄇\u{11134}𑄈𑄬 𑄃\u{11127}𑄌\u{11134}𑄑\u{11133}𑄢\u{11128}𑄠"), ("ceb", "Oberösterreich"), ("cs", "Horní Rakousy"), ("cy", "Awstria Uchaf"), ("da", "Oberösterreich"), ("de", "Oberösterreich"), ("el", "Άνω Αυστρία"), ("en", "Upper Austria"), ("es", "Alta Austria"), ("et", "Ülem-Austria"), ("eu", "Austria Garaia"), ("fa", "اوبراسترایش"), ("fi", "Ylä-Itävalta"), ("fr", "Haute-Autriche"), ("gl", "Alta Austria - Oberösterreich"), ("gu", "અપર ઑસ\u{acd}ટ\u{acd}રિયા"), ("he", "אוסטריה עילית"), ("hi", "ऊपरी ऑस\u{94d}ट\u{94d}रिया"), ("hr", "Gornja Austrija"), ("hu", "Felső-Ausztria"), ("hy", "Վերին Ավստրիա"), ("id", "Austria Hulu"), ("is", "Efra Austurríki"), ("it", "Alta Austria"), ("ja", "オーバーエスターライヒ州"), ("ka", "ზემო ავსტრია"), ("kn", "ಮೇಲ\u{ccd} ಆಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಯಾ"), ("ko", "오버외스터라이히 주"), ("lt", "Aukštutinė Austrija"), ("lv", "Augšaustrija"), ("mk", "Горна Австрија"), ("mn", "Дээр Австри"), ("mr", "ओबरओस\u{94d}टराईश"), ("ms", "Oberösterreich"), ("nb", "Oberösterreich"), ("nl", "Opper-Oostenrijk"), ("no", "Oberösterreich"), ("pl", "Górna Austria"), ("pt", "Alta Áustria"), ("ro", "Austria Superioară"), ("ru", "Верхняя Австрия"), ("si", "ඉහල ඔස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ය\u{dcf}ව"), ("sk", "Horné Rakúsko"), ("sl", "Gornja Avstrija"), ("sr", "Горња Аустрија"), ("sr_Latn", "Gornja Austrija"), ("sv", "Oberösterreich"), ("sw", "Austria Juu"), ("ta", "அப\u{bcd}பர\u{bcd} ஆஸ\u{bcd}திரிய\u{bbe}"), ("te", "అప\u{c4d}పర\u{c4d} ఆస\u{c4d}ట\u{c4d}ర\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐอ\u{e31}ปเปอร\u{e4c}ออสเตร\u{e35}ย"), ("tr", "Yukarı Avusturya"), ("uk", "Верхня Австрія"), ("ur", "بالائی آسٹریا"), ("vi", "Oberösterreich"), ("yue", "上奧地利"), ("yue_Hans", "上奥地利"), ("zh", "上奧地利")]),
                        unofficial_name_list: ["Upper Austria"].to_vec(),
                    }
                ),
                (
                    "5",
                    Subdivision{
                        name: "5",
                        country_alpha2: Alpha2::AT,
                        code: "5",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.80949), longitude: Some(13.05501), max_latitude: Some(47.85431), min_latitude: Some(47.75131), max_longitude: Some(13.1268799), min_longitude: Some(12.9859801)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Salzburg"), ("ar", "سالزبورغ"), ("be", "Зальцбург, федэральная зямля"), ("bg", "Залцбург"), ("bn", "সলজব\u{9c1}র\u{9cd}গ"), ("bs", "Salzburg"), ("ca", "Estat de Salzburg"), ("ccp", "𑄥𑄣\u{11134}𑄝𑄢\u{11133}𑄉\u{11134}"), ("ceb", "Salzburg (estado pederal)"), ("cs", "Salcbursko"), ("da", "Salzburg"), ("de", "Salzburg"), ("el", "Σάλτσμπουργκ"), ("en", "Salzburg"), ("es", "Salzburgo"), ("et", "Salzburgi liidumaa"), ("eu", "Salzburg"), ("fa", "زالتسبورگ"), ("fi", "Salzburg"), ("fr", "Salzbourg"), ("ga", "Salzburg"), ("gl", "Estado de Salzburgo"), ("gu", "સાલ\u{acd}ઝબર\u{acd}ગ"), ("he", "זלצבורג"), ("hi", "साल\u{94d}ज\u{93c}बर\u{94d}ग"), ("hr", "Salzburg"), ("hu", "Salzburg"), ("hy", "Զալցբուրգ"), ("id", "Salzburg"), ("is", "Salzburg"), ("it", "Salisburghese"), ("ja", "ザルツブルク州"), ("ka", "ზალცბურგი"), ("kn", "ಸಾಲ\u{ccd}ಜ\u{ccd}\u{200c}ಬರ\u{ccd}ಗ\u{ccd}\u{200c}"), ("ko", "잘츠부르크 주"), ("lt", "Zalcburgas"), ("lv", "Zalcburga"), ("mk", "Салцбург"), ("mn", "Зальцбург муж"), ("mr", "जाल\u{94d}त\u{94d}सब\u{941}र\u{94d}ग"), ("ms", "Salzburg"), ("nb", "Salzburg"), ("nl", "Salzburg"), ("no", "Salzburg"), ("pl", "Salzburg"), ("pt", "Salzburgo"), ("ro", "Salzburg"), ("ru", "Зальцбург"), ("si", "සැල\u{dca}ස\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Salzbursko"), ("sl", "Salzburg"), ("sr", "Салцбург"), ("sr_Latn", "Salcburg"), ("sv", "Salzburg"), ("sw", "Salzburg"), ("ta", "சலஸ\u{bcd}புரஃ"), ("te", "స\u{c3e}ల\u{c4d}జ\u{c4d}\u{200c}బర\u{c4d}గ\u{c4d}"), ("th", "ร\u{e31}ฐซาลซ\u{e4c}บ\u{e39}ร\u{e4c}ก"), ("tr", "Salzburg"), ("uk", "Зальцбург"), ("ur", "سالزبرگ (ریاست)"), ("vi", "Salzburg"), ("yue", "薩爾斯堡邦"), ("yue_Hans", "萨尔斯堡邦"), ("zh", "萨尔茨堡州")]),
                        unofficial_name_list: ["Salzbourg"].to_vec(),
                    }
                ),
                (
                    "6",
                    Subdivision{
                        name: "6",
                        country_alpha2: Alpha2::AT,
                        code: "6",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3593442), longitude: Some(14.4699827), max_latitude: Some(47.82789), min_latitude: Some(46.61163), max_longitude: Some(16.17014), min_longitude: Some(13.56417)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Stiermarke"), ("ar", "شتايرمارك"), ("az", "Ştiriya"), ("be", "Штырыя"), ("bg", "Щирия"), ("bn", "স\u{9cd}ট\u{9be}য\u{9bc}রিয\u{9bc}\u{9be}"), ("bs", "Štajerska"), ("ca", "Estíria"), ("ccp", "𑄃𑄬𑄌\u{11134}𑄑\u{1112d}𑄢\u{11128}𑄠"), ("ceb", "Steiermark"), ("cs", "Štýrsko"), ("cy", "Styria"), ("da", "Steiermark"), ("de", "Steiermark"), ("el", "Στυρία"), ("en", "Styria"), ("es", "Estiria"), ("et", "Steiermargi liidumaa"), ("eu", "Estiria"), ("fa", "اشتایرمارک"), ("fi", "Steiermark"), ("fr", "Styrie"), ("ga", "Styria"), ("gl", "Estiria"), ("gu", "સ\u{acd}ટાયરિયા"), ("he", "שטיריה"), ("hi", "स\u{94d}टायरिया"), ("hr", "Štajerska"), ("hu", "Stájerország"), ("hy", "Շտիրիա"), ("id", "Stiria"), ("is", "Steiermark"), ("it", "Stiria"), ("ja", "シュタイアーマルク州"), ("ka", "შტირია"), ("kn", "ಸ\u{ccd}ಟ\u{cbf}ರ\u{cbf}ಯಾ"), ("ko", "슈타이어마르크 주"), ("lt", "Štirija"), ("lv", "Štīrija"), ("mk", "Штаерска"), ("mn", "Штир"), ("mr", "श\u{94d}टायरमार\u{94d}क"), ("ms", "Stiria"), ("nb", "Steiermark"), ("nl", "Stiermarken"), ("no", "Steiermark"), ("pl", "Styria"), ("pt", "Estíria"), ("ro", "Stiria"), ("ru", "Штирия"), ("si", "ස\u{dca}ටය\u{dca}ර\u{dd2}ය\u{dcf}"), ("sk", "Štajersko"), ("sl", "Štajerska"), ("sr", "Штајерска"), ("sr_Latn", "Štajerska"), ("sv", "Steiermark"), ("sw", "Steiermark"), ("ta", "ஸ\u{bcd}டைரிய\u{bbe}"), ("te", "స\u{c4d}ట\u{c48}ర\u{c3f}య\u{c3e}"), ("th", "ร\u{e31}ฐสต\u{e35}เร\u{e35}ย"), ("tr", "Steiermark"), ("uk", "Штирія"), ("ur", "سٹیریا"), ("vi", "Steiermark"), ("yue", "施泰爾馬克"), ("yue_Hans", "施泰尔马克"), ("zh", "施蒂利亞州")]),
                        unofficial_name_list: ["Styria"].to_vec(),
                    }
                ),
                (
                    "7",
                    Subdivision{
                        name: "7",
                        country_alpha2: Alpha2::AT,
                        code: "7",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2537414), longitude: Some(11.601487), max_latitude: Some(47.74310999999999), min_latitude: Some(46.6515599), max_longitude: Some(12.9662801), min_longitude: Some(10.0980701)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Tirool"), ("ar", "تيرول"), ("be", "Ціроль"), ("bg", "Тирол"), ("bn", "টিরোল"), ("bs", "Tirol"), ("ca", "Estat del Tirol"), ("ccp", "𑄑\u{1112d}𑄢\u{11127}𑄣\u{11134}"), ("ceb", "Tirol (estado pederal)"), ("cs", "Tyrolsko"), ("cy", "Tirol"), ("da", "Tyrol"), ("de", "Tirol"), ("el", "Τιρόλο"), ("en", "Tyrol"), ("es", "Tirol"), ("et", "Tirool"), ("eu", "Tirol"), ("fa", "تیرول"), ("fi", "Tiroli"), ("fr", "Tyrol"), ("ga", "An Tioróil"), ("gl", "Estado de Tirol"), ("gu", "ટાયરોલ"), ("he", "טירול"), ("hi", "टिरोल"), ("hr", "Tirol"), ("hu", "Tirol"), ("hy", "Տիրոլ"), ("id", "Tirol"), ("is", "Tirol"), ("it", "Tirolo"), ("ja", "チロル州"), ("ka", "ტიროლი"), ("kn", "ಟೈರೋಲ\u{ccd}"), ("ko", "티롤 주"), ("lt", "Tirolis"), ("lv", "Tirole"), ("mk", "Тирол"), ("mn", "Тироль"), ("mr", "तिरोल"), ("ms", "Tirol"), ("nb", "Tirol"), ("nl", "Tirol"), ("no", "Tirol"), ("pl", "Tyrol"), ("pt", "Tirol"), ("ro", "Tirol"), ("ru", "Тироль"), ("si", "ටය\u{dd2}රෝල\u{dca}"), ("sk", "Tirolsko"), ("sl", "Tirolska"), ("sr", "Тирол"), ("sr_Latn", "Tirol"), ("sv", "Tyrolen"), ("ta", "டிரோள\u{bcd}"), ("te", "ట\u{c48}ర\u{c4b}ల\u{c4d}"), ("th", "ร\u{e31}ฐท\u{e35}โรล"), ("tr", "Tirol"), ("uk", "Тіроль"), ("ur", "ٹیرول (ریاست)"), ("vi", "Tirol"), ("yue", "蒂羅爾"), ("yue_Hans", "蒂罗尔"), ("zh", "蒂罗尔州")]),
                        unofficial_name_list: ["Tyrol"].to_vec(),
                    }
                ),
                (
                    "8",
                    Subdivision{
                        name: "8",
                        country_alpha2: Alpha2::AT,
                        code: "8",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2497427), longitude: Some(9.9797373), max_latitude: Some(47.59621), min_latitude: Some(46.84081), max_longitude: Some(10.23689), min_longitude: Some(9.5309099)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Vorarlberg"), ("ar", "فورآرلبرغ"), ("be", "Форарльберг"), ("bg", "Форарлберг"), ("bn", "ভোর\u{9be}লব\u{9be}র\u{9cd}গ"), ("bs", "Vorarlberg"), ("ca", "Vorarlberg"), ("ccp", "𑄞\u{11127}𑄢𑄢\u{11134}𑄣\u{11134}𑄝𑄢\u{11133}𑄉\u{11134}"), ("ceb", "Vorarlberg"), ("cs", "Vorarlbersko"), ("cy", "Vorarlberg"), ("da", "Vorarlberg"), ("de", "Vorarlberg"), ("el", "Φόραρλμπεργκ"), ("en", "Vorarlberg"), ("es", "Vorarlberg"), ("et", "Vorarlberg"), ("eu", "Vorarlberg"), ("fa", "فورآرلبرگ"), ("fi", "Vorarlberg"), ("fr", "Vorarlberg"), ("ga", "Vorarlberg"), ("gl", "Vorarlberg"), ("gu", "વોરાર\u{acd}લબર\u{acd}ગ"), ("he", "פורארלברג"), ("hi", "वोरार\u{94d}लबर\u{94d}ग"), ("hr", "Vorarlberg"), ("hu", "Vorarlberg"), ("hy", "Ֆորալբերգ"), ("id", "Vorarlberg"), ("is", "Vorarlberg"), ("it", "Vorarlberg"), ("ja", "フォアアールベルク州"), ("ka", "ფორარლბერგი"), ("kn", "ವೋರಾರ\u{ccd}ಲ\u{ccd} ಬರ\u{ccd}ಗ\u{ccd}"), ("ko", "포어아를베르크 주"), ("lt", "Forarlbergas"), ("lv", "Forarlberga"), ("mk", "Предарлска"), ("mr", "फोरार\u{94d}लबर\u{94d}ग"), ("ms", "Vorarlberg"), ("nb", "Vorarlberg"), ("ne", "भोरालबर\u{94d}ग"), ("nl", "Vorarlberg"), ("no", "Vorarlberg"), ("pl", "Vorarlberg"), ("pt", "Vorarlberg"), ("ro", "Vorarlberg"), ("ru", "Форарльберг"), ("si", "වොරර\u{dca}ල\u{dca}බර\u{dca}ග\u{dca}"), ("sk", "Vorarlbersko"), ("sl", "Predarlska"), ("sq", "Vorarlberg"), ("sr", "Форарлберг"), ("sr_Latn", "Forarlberg"), ("sv", "Vorarlberg"), ("sw", "Vorarlberg"), ("ta", "வோர\u{bbe}ல\u{bcd}பேர\u{bcd}க\u{bcd}"), ("te", "వ\u{c4b}ర\u{c3e}ర\u{c4d}ల\u{c4d} బ\u{c46}ర\u{c4d}గ\u{c4d}"), ("th", "ร\u{e31}ฐโฟราร\u{e4c}ลแบร\u{e4c}ก"), ("tr", "Vorarlberg"), ("uk", "Форарльберг"), ("ur", "وورارلبرگ"), ("vi", "Vorarlberg"), ("yue", "福拉爾貝格"), ("yue_Hans", "福拉尔贝格"), ("zh", "福拉尔贝格州")]),
                        unofficial_name_list: ["Vorarlberg"].to_vec(),
                    }
                ),
                (
                    "9",
                    Subdivision{
                        name: "9",
                        country_alpha2: Alpha2::AT,
                        code: "9",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.2081743), longitude: Some(16.3738189), max_latitude: Some(48.3230999), min_latitude: Some(48.1182699), max_longitude: Some(16.5774999), min_longitude: Some(16.1826199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::State,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Wene"), ("am", "ቪየና"), ("ar", "فيينا"), ("as", "ভিয\u{9bc}েন\u{9be}"), ("az", "Vyana"), ("be", "Вена"), ("bg", "Виена"), ("bn", "ভিয\u{9bc}েন\u{9be}"), ("bs", "Beč"), ("ca", "Viena"), ("ccp", "𑄞\u{11128}𑄠𑄬𑄚"), ("ceb", "Vienna"), ("cs", "Vídeň"), ("cy", "Fienna"), ("da", "Wien"), ("de", "Wien"), ("el", "Βιέννη"), ("en", "Vienna"), ("es", "Viena"), ("et", "Viin"), ("eu", "Viena"), ("fa", "وین"), ("fi", "Wien"), ("fr", "Vienne"), ("ga", "Vín"), ("gl", "Viena"), ("gu", "વિય\u{ac7}ના"), ("ha", "Vienna"), ("ha_NE", "Vienna"), ("he", "וינה"), ("hi", "वियना"), ("hr", "Beč"), ("hu", "Bécs"), ("hy", "Վիեննա"), ("id", "Wina"), ("is", "Vín"), ("it", "Vienna"), ("ja", "ウィーン"), ("jv", "Wina"), ("ka", "ვენა"), ("kk", "Вена"), ("kn", "ವ\u{cbf}ಯ\u{cc6}ನ\u{ccd}ನ"), ("ko", "빈"), ("ky", "Вена"), ("lt", "Viena"), ("lv", "Vīne"), ("mk", "Виена"), ("ml", "വിയന\u{d4d}ന"), ("mn", "Вена"), ("mr", "व\u{94d}हिय\u{947}ना"), ("ms", "Vienna"), ("my", "ဗ\u{102e}ယင\u{103a}နာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Wien"), ("ne", "भियना"), ("nl", "Wenen"), ("no", "Wien"), ("or", "ଭ\u{b3f}ଏନ\u{b3e}"), ("pa", "ਵਿਆਨਾ"), ("pl", "Wiedeń"), ("ps", "وين"), ("pt", "Viena"), ("ro", "Viena"), ("ru", "Вена"), ("si", "ව\u{dd2}ය\u{dcf}න\u{dcf}"), ("sk", "Viedeň"), ("sl", "Dunaj"), ("so", "Fiyena"), ("sq", "Vjena"), ("sr", "Беч"), ("sr_Latn", "Beč"), ("sv", "Wien"), ("sw", "Vienna"), ("ta", "வியன\u{bcd}ன\u{bbe}"), ("te", "వ\u{c3f}య\u{c46}న\u{c4d}న\u{c3e}"), ("th", "เว\u{e35}ยนนา"), ("tk", "Wena"), ("tr", "Viyana"), ("uk", "Відень"), ("ur", "ویانا"), ("uz", "Vena"), ("vi", "Viên"), ("yo", "Fienna"), ("yo_BJ", "Fienna"), ("yue", "維也納"), ("yue_Hans", "维也纳"), ("zh", "維也納")]),
                        unofficial_name_list: ["Vienna"].to_vec(),
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
#[cfg(feature = "at")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AT,
        alpha3: Alpha3::AUT,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 43,
        currency_code: "EUR",
        gec: Some(GEC::AU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::AUT),
        iso_long_name: "The Republic of Austria",
        iso_short_name: "Austria",
        official_language_list: ["de"].to_vec(),
        spoken_language_list: ["de"].to_vec(),
        national_destination_code_length_list: [1, 2, 3].to_vec(),
        national_number_length_list: [7, 8, 9, 10, 11, 12, 13].to_vec(),
        national_prefix: "0",
        nationality: Some("Austrian"),
        number: "040",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternEurope),
        un_locode: "AT",
        unofficial_name_list: [
            "Austria",
            "Österreich",
            "Autriche",
            "オーストリア",
            "Oostenrijk",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Austria"),
            ("af", "Oostenryk"),
            ("ak", "Austria"),
            ("am", "\u{1316}ስትሱ።"),
            ("an", "Austria"),
            ("ar", "الن\u{651}مسا"),
            ("as", "অস\u{9cd}ট\u{9cd}ৰিয়\u{9be}"),
            ("ay", "Austria"),
            ("az", "Avstriya"),
            ("ba", "Austria"),
            ("be", "Аўстрыя"),
            ("bg", "Австрия"),
            ("bi", "Austria"),
            ("bn", "অস\u{9cd}ট\u{9cd}রিয়\u{9be}"),
            ("bn_IN", "অস\u{9cd}ট\u{9cd}রিয়\u{9be}"),
            ("br", "Aostria"),
            ("bs", "Austrija"),
            ("ca", "Àustria"),
            ("ce", "Австри"),
            ("ch", "Austria"),
            ("cs", "Rakousko"),
            ("cv", "Австри"),
            ("cy", "Awstria"),
            ("da", "Østrig"),
            ("de", "Österreich"),
            ("dv", "އ\u{7ae}ސ\u{7b0}ޓ\u{7b0}ރ\u{7a8}އ\u{7a7}"),
            ("dz", "ཨ\u{f71}ས\u{f72}་ཊ\u{f72}་ཡ།"),
            ("ee", "Austria"),
            ("el", "Αυστρία"),
            ("en", "Austria"),
            ("eo", "Aŭstrio"),
            ("es", "Austria"),
            ("et", "Austria"),
            ("eu", "Austria"),
            ("fa", "اتریش"),
            ("ff", "Otiris"),
            ("fi", "Itävalta"),
            ("fo", "Eysturríki"),
            ("fr", "Autriche"),
            ("fy", "Eastenryk"),
            ("ga", "An Ostair"),
            ("gl", "Austria"),
            ("gn", "Austria"),
            ("gu", "ઓસ\u{acd}ટ\u{acd}રિયા"),
            ("gv", "Yn Austeyr"),
            ("ha", "Austriya"),
            ("he", "אוסטריה"),
            ("hi", "ऑस\u{94d}ट\u{94d}रिया"),
            ("hr", "Austrija"),
            ("ht", "Otrich"),
            ("hu", "Ausztria"),
            ("hy", "Ավստրիա"),
            ("ia", "Austria"),
            ("id", "Austria"),
            ("io", "Austria"),
            ("is", "Austurríki"),
            ("it", "Austria"),
            ("iu", "Austria"),
            ("ja", "オーストリア"),
            ("ka", "ავსტრია"),
            ("ki", "Austria"),
            ("kk", "Австрия"),
            ("kl", "Austria"),
            ("km", "អ\u{17bc}ទ\u{17d2}រ\u{17b8}ស"),
            ("kn", "ಆಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಯ"),
            ("ko", "오스트리아"),
            ("ku", "Awusturya"),
            ("kv", "Австрия"),
            ("kw", "Estrych"),
            ("ky", "Австрия"),
            ("lo", "ປະເທດໂອຕະລ\u{eb4}ດ"),
            ("lt", "Austrija"),
            ("lv", "Austrija"),
            ("mi", "Ateria"),
            ("mk", "Австрија"),
            ("ml", "ഓസ\u{d4d}ട\u{d4d}രിയ"),
            ("mn", "Австри"),
            ("mr", "ऑस\u{94d}ट\u{94d}रिया"),
            ("ms", "Austria"),
            ("mt", "Awtrija"),
            ("my", "သြစတြ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Oteriya"),
            ("nb", "Østerrike"),
            ("ne", "अष\u{94d}ट\u{94d}रिया"),
            ("nl", "Oostenrijk"),
            ("nn", "Austerrike"),
            ("nv", "Óóswiya"),
            ("oc", "Àustria"),
            ("or", "ଅଷ\u{b4d}ଟ\u{b4d}ର\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਆਸਟਰੀਆ"),
            ("pi", "आस\u{94d}ट\u{94d}रिया"),
            ("pl", "Austria"),
            ("ps", "اتریش"),
            ("pt", "Áustria"),
            ("pt_BR", "Áustria"),
            ("ro", "Austria"),
            ("ru", "Австрия"),
            ("rw", "Ositiriya"),
            ("sc", "Àustria"),
            ("sd", "آسٽريا"),
            ("si", "ඔස\u{dca}ට\u{dca}\u{200d}ර\u{dd2}ය\u{dcf}ව"),
            ("sk", "Rakúsko"),
            ("sl", "Avstrija"),
            ("so", "Osteeriya"),
            ("sq", "Austri"),
            ("sr", "Аустрија"),
            ("sv", "Österrike"),
            ("sw", "Austria"),
            ("ta", "ஆஸ\u{bcd}திரிய\u{bbe}"),
            ("te", "ఓస\u{c4d}ట\u{c4d}ర\u{c3f}య\u{c3e}"),
            ("tg", "Австрия"),
            ("th", "ออสเตร\u{e35}ย"),
            ("ti", "ኦስትሪያ"),
            ("tk", "Awstriýa"),
            ("tl", "Austriya"),
            ("tr", "Avusturya"),
            ("tt", "Аустриа"),
            ("ug", "ئاۋسترىيە"),
            ("uk", "Австрія"),
            ("ur", "آسٹریا"),
            ("uz", "Avstriya"),
            ("ve", "Austria"),
            ("vi", "Ao"),
            ("wa", "Otriche"),
            ("wo", "Ótriis"),
            ("xh", "Austria"),
            ("yo", "Austríà"),
            ("zh_CN", "奥地利"),
            ("zh_HK", "奧地利"),
            ("zh_TW", "奧地利"),
            ("zu", "I-Ostriya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
