// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Portuguese Republic

#[cfg(all(feature = "pt", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::PT;
    pub const ALPHA3: Alpha3 = Alpha3::PRT;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 351;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = Some(GEC::PO);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::POR);
    pub const ISO_SHORT_NAME: &str = "Portugal";
    pub const ISO_LONG_NAME: &str = "The Portuguese Republic";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["pt"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[9];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Portuguese");
    pub const NUMBER: &str = "620";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}-\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "PT";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Portugal", "ポルトガル"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Portugal"),
        ("af", "Portugal"),
        ("ak", "Portugal"),
        ("am", "ፖርቱጋል"),
        ("an", "Portugal"),
        ("ar", "البرتغال"),
        ("as", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
        ("ay", "Portugal"),
        ("az", "Portuqaliya"),
        ("ba", "Portugal"),
        ("be", "Партугалія"),
        ("bg", "Португалия"),
        ("bi", "Portugal"),
        ("bn", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
        ("bn_IN", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
        ("br", "Portugal"),
        ("bs", "Portugal"),
        ("ca", "Portugal"),
        ("ce", "Португали"),
        ("ch", "Portugal"),
        ("cs", "Portugalsko"),
        ("cv", "Португали"),
        ("cy", "Portiwgal"),
        ("da", "Portugal"),
        ("de", "Portugal"),
        ("dv", "ޕ\u{7af}ޗ\u{7aa}ގ\u{7a6}ލ\u{7b0}"),
        ("dz", "པ\u{f7c}ར་ཊ\u{f74}་ག\u{f71}ལ།"),
        ("ee", "Portugal"),
        ("el", "Πορτογαλία"),
        ("en", "Portugal"),
        ("eo", "Portugalio"),
        ("es", "Portugal"),
        ("et", "Portugal"),
        ("eu", "Portugal"),
        ("fa", "پرتغال"),
        ("ff", "Portokeesi"),
        ("fi", "Portugali"),
        ("fo", "Portugal"),
        ("fr", "Portugal"),
        ("fy", "Portegal"),
        ("ga", "An Phortaingéil"),
        ("gl", "Portugal"),
        ("gn", "Portugal"),
        ("gu", "પોર\u{acd}ટ\u{ac1}ગલ"),
        ("gv", "Yn Phortiugal"),
        ("ha", "Portugal"),
        ("he", "פורטוגל"),
        ("hi", "प\u{941}र\u{94d}तगाल"),
        ("hr", "Portugal"),
        ("ht", "Pòtigal"),
        ("hu", "Portugália"),
        ("hy", "Պորտուգալիա"),
        ("ia", "Portugal"),
        ("id", "Portugal"),
        ("io", "Portugal"),
        ("is", "Portúgal"),
        ("it", "Portogallo"),
        ("iu", "Portugal"),
        ("ja", "ポルトガル"),
        ("ka", "პორტუგალია"),
        ("ki", "Portugal"),
        ("kk", "Португалия"),
        ("kl", "Portugal"),
        ("km", "ព\u{17d0}រទ\u{17bb}យហ\u{17d2}គាល\u{17cb}"),
        ("kn", "ಪೋರ\u{ccd}ತುಗಾಲ\u{ccd}"),
        ("ko", "포르투갈"),
        ("ku", "Portekîz"),
        ("kv", "Португалия"),
        ("kw", "Portyngal"),
        ("ky", "Португалия"),
        ("lo", "ປະເທດປອກຕ\u{eb8}ຍການ"),
        ("lt", "Portugalija"),
        ("lv", "Portugāle"),
        ("mi", "Potukara"),
        ("mk", "Португалија"),
        ("ml", "പോര\u{d4d}\u{200d}ച\u{d4d}ച\u{d41}ഗല\u{d4d}\u{200d}"),
        ("mn", "Португаль"),
        ("mr", "पोर\u{94d}त\u{941}गाल"),
        ("ms", "Feringgi"),
        ("mt", "Portugall"),
        (
            "my",
            "ပေါ\u{103a}တ\u{1030}ဂ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Portsiugar"),
        ("nb", "Portugal"),
        ("ne", "पोर\u{94d}च\u{941}गल"),
        ("nl", "Portugal"),
        ("nn", "Portugal"),
        ("nv", "Portugal"),
        ("oc", "Portugal"),
        ("or", "ପର\u{b4d}ତ\u{b41}ଗ\u{b3e}ଲ"),
        ("pa", "ਪ\u{a41}ਰਤਗਾਲ"),
        ("pi", "प\u{941}र\u{94d}तगाल"),
        ("pl", "Portugalia"),
        ("ps", "پورتګال"),
        ("pt", "Portugal"),
        ("pt_BR", "Portugal"),
        ("ro", "Portugalia"),
        ("ru", "Португалия"),
        ("rw", "Porutigali"),
        ("sc", "Portugallu"),
        ("sd", "پرتگال"),
        ("si", "පෘත\u{dd4}ග\u{dcf}ලය"),
        ("sk", "Portugalsko"),
        ("sl", "Portugalska"),
        ("so", "Bortuqaal"),
        ("sq", "Portugali"),
        ("sr", "Португал"),
        ("sv", "Portugal"),
        ("sw", "Portugal"),
        ("ta", "போர\u{bcd}ச\u{bcd}சுகல\u{bcd}"),
        ("te", "ప\u{c4b}ర\u{c4d}చుగల\u{c4d}"),
        ("tg", "Португалия"),
        ("th", "โปรต\u{e38}เกส"),
        ("ti", "ፖርቱጋል"),
        ("tk", "Portugaliýa"),
        ("tl", "Portugal"),
        ("tr", "Portekiz"),
        ("tt", "Портуgалиа"),
        ("ug", "پورتۇگالىيە"),
        ("uk", "Португалія"),
        ("ur", "پرتگال"),
        ("uz", "Portugaliya"),
        ("ve", "Portugal"),
        ("vi", "Bồ Đào Nha"),
        ("wa", "Portugal"),
        ("wo", "Portugaal"),
        ("xh", "Portugal"),
        ("yo", "Pọ\u{301}rtúgàl"),
        ("zh_CN", "葡萄牙"),
        ("zh_HK", "葡萄牙"),
        ("zh_TW", "葡萄牙"),
        ("zu", "IPhothugali"),
    ];
    #[cfg(all(feature = "pt", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 39.39987199999999;
        pub const LONGITUDE: f64 = -8.224454;
        pub const MAX_LATITUDE: f64 = 42.1543111;
        pub const MAX_LONGITUDE: f64 = -6.189159200000001;
        pub const MIN_LATITUDE: f64 = 32.2895;
        pub const MIN_LONGITUDE: f64 = -31.4647999;
        pub const NORTHEAST_LATITUDE: f64 = 42.1543111;
        pub const NORTHEAST_LONGITUDE: f64 = -6.189159200000001;
        pub const SOUTHWEST_LATITUDE: f64 = 32.2895;
        pub const SOUTHWEST_LONGITUDE: f64 = -31.4647999;
    }
}
#[cfg(all(feature = "pt", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 39.39987199999999,
            longitude: -8.224454,
            max_latitude: 42.1543111,
            max_longitude: -6.189159200000001,
            min_latitude: 32.2895,
            min_longitude: -31.4647999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 42.1543111,
                    longitude: -6.189159200000001,
                },
                southwest: CountryGeoBound {
                    latitude: 32.2895,
                    longitude: -31.4647999,
                },
            },
        }
    }
}

#[cfg(all(feature = "pt", feature = "subdivisions"))]
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
                        country_alpha2: Alpha2::PT,
                        code: "01",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6405055), longitude: Some(-8.6537539), max_latitude: Some(40.6545896), min_latitude: Some(40.6241712), max_longitude: Some(-8.6238121), min_longitude: Some(-8.665443800000002)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أفييرو"), ("be", "акруга Авейру"), ("bg", "Авейру"), ("ca", "Districte d’Aveiro"), ("ccp", "𑄃𑄞𑄬\u{1112d}𑄢\u{1112e}"), ("ceb", "Distrito de Aveiro"), ("de", "Distrikt Aveiro"), ("en", "Aveiro"), ("es", "Distrito de Aveiro"), ("eu", "Aveiro"), ("fi", "Aveiron piiri"), ("fr", "District d’Aveiro"), ("gl", "Distrito de Aveiro"), ("hu", "Aveiro"), ("hy", "Ավեյրու"), ("id", "Distrik Aveiro"), ("it", "distretto di Aveiro"), ("ja", "アヴェイロ県"), ("ko", "아베이루 현"), ("ms", "Daerah Aveiro"), ("nb", "Aveiro"), ("nl", "Aveiro"), ("no", "Aveiro"), ("pl", "Dystrykt Aveiro"), ("pt", "Aveiro"), ("ro", "Districtul Aveiro"), ("ru", "Авейру"), ("sr", "Авеиро"), ("sr_Latn", "Aveiro"), ("sv", "Aveiro"), ("uk", "Авейру"), ("ur", "آواریو ضلع"), ("vi", "Aveiro"), ("zh", "阿威羅區")]),
                        unofficial_name_list: ["Aveiro"].to_vec(),
                    }
                ),
                (
                    "02",
                    Subdivision{
                        name: "02",
                        country_alpha2: Alpha2::PT,
                        code: "02",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.0153039), longitude: Some(-7.8627308), max_latitude: Some(38.0746209), min_latitude: Some(37.97009), max_longitude: Some(-7.8177491), min_longitude: Some(-7.959504799999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بيجا"), ("be", "Бежа"), ("bg", "Бежа"), ("bn", "বেজ\u{9be} জেল\u{9be}"), ("ca", "Districte de Beja"), ("ccp", "𑄝𑄬𑄎"), ("ceb", "Distrito de Beja"), ("da", "Beja District"), ("de", "Distrikt Beja"), ("el", "Μπέτζα"), ("en", "Beja"), ("es", "Distrito de Beja"), ("eu", "Beja"), ("fi", "Bejan piiri"), ("fr", "District de Beja"), ("gl", "Distrito de Beja"), ("gu", "બ\u{ac7}જા જિલ\u{acd}લો"), ("he", "בז׳ה"), ("hi", "ब\u{947}जा जिला"), ("hu", "Beja"), ("id", "Distrik Beja"), ("it", "distretto di Beja"), ("ja", "ベージャ県"), ("kn", "ಬೇಜಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "베자 현"), ("lt", "Bežos apskritis"), ("lv", "Bedžas distrikts"), ("mr", "ब\u{947}जा जिल\u{94d}हा"), ("ms", "Daerah Beja"), ("nb", "Beja"), ("nl", "Beja"), ("no", "Beja"), ("pl", "Dystrykt Beja"), ("pt", "Beja"), ("ro", "Districtul Beja"), ("ru", "Бежа"), ("si", "බෙජ\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Бежа"), ("sr_Latn", "Beža"), ("sv", "Beja"), ("ta", "பெஜ\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c47}జ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เบจา"), ("tr", "Beja District"), ("uk", "Бежа"), ("ur", "بیجا ضلع"), ("vi", "Beja"), ("zh", "貝雅區")]),
                        unofficial_name_list: ["Beja"].to_vec(),
                    }
                ),
                (
                    "03",
                    Subdivision{
                        name: "03",
                        country_alpha2: Alpha2::PT,
                        code: "03",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.5454486), longitude: Some(-8.426506999999999), max_latitude: Some(41.5841477), min_latitude: Some(41.510402), max_longitude: Some(-8.3421791), min_longitude: Some(-8.4806633)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة براغا"), ("be", "акруга Брага"), ("bg", "Брага"), ("bn", "ব\u{9cd}র\u{9be}গ\u{9be} জেল\u{9be}"), ("ca", "Districte de Braga"), ("ccp", "𑄝\u{11133}𑄢𑄉"), ("ceb", "Distrito de Braga"), ("da", "Braga District"), ("de", "Distrikt Braga"), ("el", "Μπράγκα"), ("en", "Braga"), ("es", "Distrito de Braga"), ("eu", "Braga"), ("fi", "Bragan piiri"), ("fr", "District de Braga"), ("gl", "Distrito de Braga"), ("gu", "બ\u{acd}ર\u{ac7}ગા જિલ\u{acd}લો"), ("hi", "ब\u{94d}रागा जिला"), ("hu", "Braga"), ("id", "Distrik Braga"), ("it", "distretto di Braga"), ("ja", "ブラガ県"), ("ka", "ბრაგა"), ("kn", "ಬ\u{ccd}ರಾಗಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "브라가 현"), ("lt", "Bragos apskritis"), ("lv", "Bragas distrikts"), ("mr", "ब\u{94d}रागा जिल\u{94d}हा"), ("ms", "Daerah Braga"), ("nb", "Braga"), ("nl", "Braga"), ("no", "Braga"), ("pl", "Dystrykt Braga"), ("pt", "Braga"), ("ro", "Districtul Braga"), ("ru", "Брага"), ("si", "බ\u{dca}\u{200d}රග\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Брага"), ("sr_Latn", "Braga"), ("sv", "Braga"), ("ta", "ப\u{bcd}ர\u{bbe}க ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c4d}ర\u{c3e}గ\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "บรากา"), ("tr", "Braga District"), ("uk", "Браґа"), ("ur", "براگا ضلع"), ("vi", "Braga"), ("zh", "布拉加區")]),
                        unofficial_name_list: ["Braga"].to_vec(),
                    }
                ),
                (
                    "04",
                    Subdivision{
                        name: "04",
                        country_alpha2: Alpha2::PT,
                        code: "04",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.8061131), longitude: Some(-6.756737999999999), max_latitude: Some(41.8369891), min_latitude: Some(41.7861053), max_longitude: Some(-6.7063504), min_longitude: Some(-6.7907131)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة براغانزا"), ("be", "акруга Браганса"), ("bg", "Браганса"), ("ca", "Districte de Bragança"), ("ccp", "𑄝\u{11133}𑄢𑄉𑄚\u{11134}𑄇"), ("ceb", "Distrito de Bragança"), ("de", "Distrikt Bragança"), ("en", "Bragança"), ("es", "Braganza"), ("eu", "Bragantza"), ("fi", "Bragançan piiri"), ("fr", "District de Bragance"), ("gl", "Distrito de Braganza - Bragança"), ("hu", "Bragança"), ("id", "Distrik Bragança"), ("it", "distretto di Braganza"), ("ja", "ブラガンサ県"), ("ko", "브라간사 현"), ("ms", "Daerah Bragança"), ("nb", "Bragança"), ("nl", "Bragança"), ("no", "Bragança"), ("pl", "Dystrykt Bragança"), ("pt", "Bragança"), ("ro", "Districtul Bragança"), ("ru", "Браганса"), ("sr", "Браганса"), ("sr_Latn", "Bragansa"), ("sv", "Bragança"), ("uk", "Браґанса"), ("ur", "براگانسا ضلع"), ("vi", "Bragança"), ("zh", "布拉干薩區")]),
                        unofficial_name_list: ["Bragança"].to_vec(),
                    }
                ),
                (
                    "05",
                    Subdivision{
                        name: "05",
                        country_alpha2: Alpha2::PT,
                        code: "05",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.8197117), longitude: Some(-7.4964662), max_latitude: Some(39.9093461), min_latitude: Some(39.7136873), max_longitude: Some(-7.367089999999999), min_longitude: Some(-7.5591241)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كاستيلو برانكو"), ("be", "акруга Каштэлу-Бранку"), ("bg", "Кащелу Бранку"), ("bn", "ক\u{9be}স\u{9cd}টেলো ব\u{9cd}র\u{9be}ঙ\u{9cd}কো জেল\u{9be}"), ("ca", "Districte de Castelo Branco"), ("ccp", "𑄇𑄌\u{11134}𑄑𑄬𑄣\u{1112e} 𑄝\u{11133}𑄢𑄚\u{11134}𑄇\u{1112e}"), ("ceb", "Distrito de Castelo Branco"), ("da", "Castelo Branco District"), ("de", "Distrikt Castelo Branco"), ("el", "Καστέλο Μπράνκο"), ("en", "Castelo Branco"), ("es", "Distrito de Castelo Branco"), ("eu", "Castelo Branco"), ("fi", "Castelo Brancon piiri"), ("fr", "District de Castelo Branco"), ("gl", "Distrito de Castelo Branco"), ("gu", "કાસ\u{acd}ટ\u{ac7}લો બ\u{acd}રાન\u{acd}કો જિલ\u{acd}લો"), ("hi", "कास\u{94d}ट\u{947}लो ब\u{94d}र\u{948}\u{902}को जिला"), ("hu", "Castelo Branco"), ("id", "Distrik Castelo Branco"), ("it", "distretto di Castelo Branco"), ("ja", "カステロ・ブランコ県"), ("kn", "ಕ\u{ccd}ಯಾಸ\u{ccd}ಟ\u{cc6}ಲೊ ಬ\u{ccd}ರಾಂಕೊ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "카스텔루브랑쿠 현"), ("lt", "Kastelo Branko apskritis"), ("lv", "Kaštelu Branku distrikts"), ("mr", "क\u{945}स\u{94d}ट\u{947}लो ब\u{94d}र\u{945}\u{902}को जिल\u{94d}हा"), ("ms", "Daerah Castelo Branco"), ("nb", "Castelo Branco"), ("nl", "Castelo Branco"), ("no", "Castelo Branco"), ("pl", "Dystrykt Castelo Branco"), ("pt", "Castelo Branco"), ("ro", "Districtul Castelo Branco"), ("ru", "Каштелу-Бранку"), ("si", "කැස\u{dca}ටෙලෝ බ\u{dca}\u{200d}රන\u{dca}කෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Кастело Бранко"), ("sr_Latn", "Kastelo Branko"), ("sv", "Castelo Branco"), ("ta", "க\u{bbe}ஸ\u{bcd}டெல\u{bcd}லோ ப\u{bcd}ர\u{bbe}ன\u{bcd}க\u{bcd}கோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c3e}స\u{c4d}ట\u{c46}ల\u{c4b} బ\u{c4d}ర\u{c3e}ంక\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตเคสเตโล บรานโค"), ("tr", "Castello Branco District"), ("uk", "Каштелу-Бранку"), ("ur", "کاشتیلو برانکو ضلع"), ("vi", "Castelo Branco"), ("zh", "布朗庫堡區")]),
                        unofficial_name_list: ["Castelo Branco"].to_vec(),
                    }
                ),
                (
                    "06",
                    Subdivision{
                        name: "06",
                        country_alpha2: Alpha2::PT,
                        code: "06",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.2033145), longitude: Some(-8.4102573), max_latitude: Some(40.2861477), min_latitude: Some(40.1713722), max_longitude: Some(-8.322907599999999), min_longitude: Some(-8.5096411)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كويمبرا"), ("be", "Каімбра"), ("bg", "Куимбра"), ("bn", "কমব\u{9cd}র\u{9be} জেল\u{9be}"), ("ca", "Districte de Coïmbra"), ("ccp", "𑄇\u{11130}𑄟\u{11134}𑄝\u{11133}𑄢"), ("ceb", "Distrito de Coimbra"), ("da", "Coimbra District"), ("de", "Distrikt Coimbra"), ("el", "Κόιμπρα"), ("en", "Coimbra"), ("es", "Distrito de Coímbra"), ("eu", "Coimbra"), ("fi", "Coimbran piiri"), ("fr", "District de Coimbra"), ("gl", "Distrito de Coimbra"), ("gu", "કોઈમ\u{acd}બ\u{acd}રા જિલ\u{acd}લો"), ("hi", "कोइम\u{94d}ब\u{94d}रा जिला"), ("hu", "Coimbra"), ("id", "Distrik Coimbra"), ("it", "distretto di Coimbra"), ("ja", "コインブラ県"), ("kn", "ಕೊಯ\u{cbf}ಂಬ\u{ccd}ರಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "코임브라 현"), ("lt", "Koimbros apskritis"), ("lv", "Kombras distrikts"), ("mr", "कोइम\u{94d}बा जिल\u{94d}हा"), ("ms", "Daerah Coimbra"), ("nb", "Coimbra"), ("nl", "Coimbra"), ("no", "Coimbra"), ("pl", "Dystrykt Coimbra"), ("pt", "Coimbra"), ("ro", "Districtul Coimbra"), ("ru", "Коимбра"), ("si", "කොය\u{dd2}ම\u{dca}බ\u{dca}\u{200d}ර\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Коимбра"), ("sr_Latn", "Koimbra"), ("sv", "Coimbra"), ("ta", "கொய\u{bcd}ம\u{bcd}ப\u{bcd}ர\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "క\u{c4b}య\u{c3f}ంబ\u{c4d}ర\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตโคอ\u{e34}มบร\u{e34}"), ("tr", "Coimbra District"), ("uk", "Коїмбра"), ("ur", "کویمبرا ضلع"), ("vi", "Coimbra"), ("zh", "科英布拉區")]),
                        unofficial_name_list: ["Coimbra"].to_vec(),
                    }
                ),
                (
                    "07",
                    Subdivision{
                        name: "07",
                        country_alpha2: Alpha2::PT,
                        code: "07",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.571431), longitude: Some(-7.913501999999999), max_latitude: Some(38.6169193), min_latitude: Some(38.5227211), max_longitude: Some(-7.843514600000001), min_longitude: Some(-7.9785036)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة إيفورا"), ("be", "акруга Эвара"), ("bg", "Евура"), ("bn", "এভোর\u{9be} জেল\u{9be}"), ("ca", "Districte d’Évora"), ("ccp", "𑄃\u{11128}𑄞\u{1112e}𑄢"), ("ceb", "Distrito de Évora"), ("da", "Évora District"), ("de", "Distrikt Évora"), ("el", "Έβορα"), ("en", "Évora"), ("es", "Distrito de Évora"), ("eu", "Evora"), ("fi", "Évoran piiri"), ("fr", "District d’Évora"), ("gl", "Distrito de Évora"), ("gu", "ઇવોરા જિલ\u{acd}લો"), ("hi", "एवोरा जिला"), ("hu", "Évora"), ("id", "Distrik Évora"), ("it", "distretto di Évora"), ("ja", "エヴォラ県"), ("kn", "ಎವೊರಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "에보라 현"), ("lt", "Evoros apskritis"), ("lv", "Evoras distrikts"), ("mr", "एव\u{94d}होरा जिल\u{94d}हा"), ("ms", "Daerah Évora"), ("nb", "Évora"), ("nl", "Évora"), ("no", "Évora"), ("pl", "Dystrykt Évora"), ("pt", "Évora"), ("ro", "Districtul Évora"), ("ru", "Эвора"), ("si", "එවොර\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Евора"), ("sr_Latn", "Evora"), ("sv", "Évora"), ("ta", "ஏவோர\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఇవ\u{c4b}ర\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "แอว\u{e39}รา"), ("tr", "évora District"), ("uk", "Евора"), ("ur", "ایورا ضلع"), ("vi", "Évora"), ("zh", "埃武拉區")]),
                        unofficial_name_list: ["Évora"].to_vec(),
                    }
                ),
                (
                    "08",
                    Subdivision{
                        name: "08",
                        country_alpha2: Alpha2::PT,
                        code: "08",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.0193548), longitude: Some(-7.9304397), max_latitude: Some(37.0738998), min_latitude: Some(36.9617104), max_longitude: Some(-7.8093544), min_longitude: Some(-8.000400599999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فارو"), ("be", "акруга Фару"), ("bg", "Фару"), ("bn", "ফ\u{9be}রো জেল\u{9be}"), ("ca", "Districte de Faro"), ("ccp", "𑄜𑄢\u{1112e}"), ("ceb", "Distrito de Faro"), ("da", "Faro"), ("de", "Distrikt Faro"), ("el", "Φάρο"), ("en", "Faro"), ("es", "Distrito de Faro"), ("eu", "Faro"), ("fa", "ناحیه فارو"), ("fi", "Faron piiri"), ("fr", "District de Faro"), ("gl", "Distrito de Faro"), ("gu", "ફ\u{ac7}રો જિલ\u{acd}લો"), ("hi", "फ\u{93c}ारो जिला"), ("hu", "Faro"), ("id", "Distrik Faro"), ("it", "distretto di Faro"), ("ja", "ファーロ県"), ("kn", "ಫರೋ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "파루 현"), ("lt", "Faro apskritis"), ("lv", "Faru apgabals"), ("mr", "फ\u{947}रो जिल\u{94d}हा"), ("ms", "Daerah Faro"), ("nb", "Faro"), ("nl", "Faro"), ("no", "Faro"), ("pl", "Dystrykt Faro"), ("pt", "Faro"), ("ro", "Districtul Faro"), ("ru", "Фару"), ("si", "ෆ\u{dcf}රො ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Фаро"), ("sr_Latn", "Faro"), ("sv", "Faro"), ("ta", "ப\u{bbe}ரோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ఫ\u{c3e}ర\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ฟาร\u{e39}"), ("tr", "Faro District"), ("uk", "Фару"), ("ur", "فارو ضلع"), ("vi", "Faro"), ("zh", "法魯區")]),
                        unofficial_name_list: ["Faro"].to_vec(),
                    }
                ),
                (
                    "09",
                    Subdivision{
                        name: "09",
                        country_alpha2: Alpha2::PT,
                        code: "09",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.5383482), longitude: Some(-7.266131499999998), max_latitude: Some(40.5738913), min_latitude: Some(40.50446), max_longitude: Some(-7.1980682), min_longitude: Some(-7.2926517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة غواردا"), ("be", "акруга Гуарда"), ("bg", "Гуарда"), ("ca", "Districte de Guarda"), ("ccp", "𑄉\u{1112a}𑄠𑄢\u{11134}𑄓"), ("ceb", "Distrito da Guarda"), ("de", "Distrikt Guarda"), ("en", "Guarda"), ("es", "Distrito de Guarda"), ("eu", "Guarda"), ("fi", "Guardan piiri"), ("fr", "District de Guarda"), ("gl", "Distrito da Guarda"), ("hu", "Guarda"), ("id", "Distrik Guarda"), ("it", "distretto di Guarda"), ("ja", "グアルダ県"), ("ko", "구아르다 현"), ("ms", "Daerah Guarda"), ("nb", "Guarda"), ("nl", "Guarda"), ("no", "Guarda"), ("pl", "Dystrykt Guarda"), ("pt", "Guarda"), ("ro", "Districtul Guarda"), ("ru", "Гуарда"), ("sr", "Гварда"), ("sr_Latn", "Gvarda"), ("sv", "Guarda"), ("uk", "Ґуарда"), ("ur", "گواردا ضلع"), ("vi", "Guarda"), ("zh", "瓜達區")]),
                        unofficial_name_list: ["Guarda"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::PT,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.74953310000001), longitude: Some(-8.807682999999999), max_latitude: Some(39.798205), min_latitude: Some(39.70991799999999), max_longitude: Some(-8.7350657), min_longitude: Some(-8.8768517)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ليريا"), ("be", "акруга Лейрыя"), ("bg", "Лейрия"), ("bn", "লেইর\u{9be}ইয\u{9bc}\u{9be} জেল\u{9be}"), ("ca", "Districte de Leiria"), ("ccp", "𑄣𑄬𑄃\u{11128}𑄠𑄢\u{11128}𑄠"), ("ceb", "Distrito de Leiria"), ("da", "Leiria District"), ("de", "Distrikt Leiria"), ("el", "Λειρία"), ("en", "Leiria"), ("es", "Distrito de Leiria"), ("eu", "Leiria"), ("fa", "استان لیریا"), ("fi", "Leirian piiri"), ("fr", "District de Leiria"), ("gl", "Distrito de Leiria"), ("gu", "લ\u{ac7}રીયા જિલ\u{acd}લો"), ("hi", "लीरिया जिला"), ("hu", "Leiria"), ("id", "Distrik Leiria"), ("it", "distretto di Leiria"), ("ja", "レイリア県"), ("kn", "ಲೀರ\u{cbf}ಯಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "레이리아 현"), ("lt", "Leirijos apskritis"), ("lv", "Leirijas distrikts"), ("mr", "लीरीया जिल\u{94d}हा"), ("ms", "Daerah Leiria"), ("nb", "Leiria"), ("nl", "Leiria"), ("no", "Leiria"), ("pl", "Dystrykt Leiria"), ("pt", "Leiria"), ("ro", "Districtul Leiria"), ("ru", "Лейрия"), ("si", "ලේය\u{dd2}ර\u{dd2}ය\u{dcf} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Леирија"), ("sr_Latn", "Leirija"), ("sv", "Leiria"), ("ta", "லேரிய\u{bbe} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ల\u{c40}ర\u{c3f}య\u{c3e} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "มณฑลบ\u{e38}มท\u{e31}ง"), ("tr", "Leiria District"), ("uk", "Лейрія"), ("ur", "لائریا ضلع"), ("vi", "Leiria"), ("zh", "萊里亞區")]),
                        unofficial_name_list: ["Leiria"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::PT,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.7222524), longitude: Some(-9.1393366), max_latitude: Some(38.7958538), min_latitude: Some(38.6913994), max_longitude: Some(-9.0905718), min_longitude: Some(-9.2298356)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة لشبونة"), ("be", "акруга Лісабон"), ("bg", "Лисабон"), ("ca", "Districte de Lisboa"), ("ccp", "𑄣\u{11128}𑄌\u{11134}𑄝\u{11127}𑄚\u{11134}"), ("ceb", "Distrito de Lisboa"), ("cs", "Distrikt Lisabon"), ("da", "Lissabon"), ("de", "Distrikt Lissabon"), ("en", "Lisbon"), ("es", "Distrito de Lisboa"), ("eu", "Lisboa"), ("fa", "ناحیه لیسبون"), ("fi", "Lissabonin piiri"), ("fr", "District de Lisbonne"), ("gl", "Distrito de Lisboa"), ("hu", "Lisszabon"), ("id", "Distrik Lisboa"), ("it", "distretto di Lisbona"), ("ja", "リスボン県"), ("ko", "리스보아 현"), ("ms", "Daerah Lisboa"), ("nb", "Lisboa"), ("nl", "Lissabon"), ("no", "Lisboa"), ("pl", "Dystrykt Lizbona"), ("pt", "Lisboa"), ("ro", "Districtul Lisabona"), ("ru", "Лиссабон"), ("sr", "Лисабон"), ("sr_Latn", "Lisabon"), ("sv", "Lissabon"), ("uk", "Лісабон"), ("ur", "لزبن ضلع"), ("vi", "Lisbon"), ("zh", "里斯本區")]),
                        unofficial_name_list: ["Lisboa"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::PT,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2967086), longitude: Some(-7.4284755), max_latitude: Some(39.3153866), min_latitude: Some(39.262495), max_longitude: Some(-7.3995175), min_longitude: Some(-7.4664739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بورتاليجري"), ("be", "акруга Порталегры"), ("bg", "Порталегри"), ("ca", "Districte de Portalegre"), ("ccp", "𑄛\u{11127}𑄢\u{11134}𑄑𑄣𑄬𑄇\u{11134}𑄢𑄬"), ("ceb", "Distrito de Portalegre"), ("de", "Distrikt Portalegre"), ("en", "Portalegre"), ("es", "Distrito de Portalegre"), ("eu", "Portalegre"), ("fi", "Portalegren piiri"), ("fr", "District de Portalegre"), ("gl", "Distrito de Portalegre"), ("hu", "Portalegre"), ("id", "Distrik Portalegre"), ("it", "distretto di Portalegre"), ("ja", "ポルタレグレ県"), ("ko", "포르탈레그르 현"), ("ms", "Daerah Portalegre"), ("nb", "Portalegre"), ("nl", "Portalegre"), ("no", "Portalegre"), ("pl", "Dystrykt Portalegre"), ("pt", "Portalegre"), ("ro", "Districtul Portalegre"), ("ru", "Порталегри"), ("sr", "Порталегре"), ("sr_Latn", "Portalegre"), ("sv", "Portalegre"), ("uk", "Порталеґрі"), ("ur", "پورتالیگرے ضلع"), ("vi", "Portalegre"), ("zh", "波塔萊格雷區")]),
                        unofficial_name_list: ["Portalegre"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::PT,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.1579438), longitude: Some(-8.629105299999999), max_latitude: Some(41.1859353), min_latitude: Some(41.1383506), max_longitude: Some(-8.5526134), min_longitude: Some(-8.6910927)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بورتو"), ("be", "акруга Порту"), ("bg", "Порту"), ("ca", "Districte de Porto"), ("ccp", "𑄛\u{1112e}𑄢\u{11134}𑄑\u{1112e}"), ("ceb", "Distrito do Porto"), ("de", "Distrikt Porto"), ("en", "Porto"), ("es", "Distrito de Oporto"), ("eu", "Porto"), ("fi", "Porton piiri"), ("fr", "District de Porto"), ("gl", "Distrito do Porto"), ("id", "Distrik Porto"), ("it", "distretto di Porto"), ("ja", "ポルト県"), ("ko", "포르투 현"), ("mk", "Порто"), ("ms", "Daerah Porto"), ("nb", "Porto"), ("nl", "Porto"), ("no", "Porto"), ("pt", "Porto"), ("ro", "Districtul Porto"), ("ru", "Порту"), ("sr", "Порто"), ("sr_Latn", "Porto"), ("sv", "Porto"), ("uk", "Порту"), ("ur", "پورتو ضلع"), ("vi", "Porto"), ("zh", "波爾圖區")]),
                        unofficial_name_list: ["Porto"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::PT,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(39.2366618), longitude: Some(-8.686011900000002), max_latitude: Some(39.2869109), min_latitude: Some(39.1687465), max_longitude: Some(-8.626665299999999), min_longitude: Some(-8.7494918)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سانتاريم"), ("be", "акруга Сантарэн"), ("bg", "Сантарем"), ("bn", "স\u{9be}ন\u{9cd}ত\u{9be}রেম জেল\u{9be}"), ("ca", "Districte de Santarém"), ("ccp", "𑄥𑄚\u{11134}𑄑𑄢\u{11134}𑄢𑄬𑄟\u{11134}"), ("ceb", "Distrito de Santarém"), ("da", "Santarém District"), ("de", "Distrikt Santarém"), ("el", "Σανταρέμ"), ("en", "Santarém"), ("es", "Distrito de Santarém"), ("eu", "Santarem"), ("fa", "ناحیه سانتاری"), ("fi", "Santarémin piiri"), ("fr", "District de Santarém"), ("gl", "Distrito de Santarém"), ("gu", "સ\u{ac7}ન\u{acd}ટર\u{ac7}મ જિલ\u{acd}લો"), ("hi", "स\u{948}न\u{94d}तार\u{947}म जिला"), ("hu", "Santarém"), ("id", "Distrik Santarém"), ("it", "distretto di Santarém"), ("ja", "サンタレン県"), ("kn", "ಸ\u{ccd}ಯಾಂಟರ\u{cc6}ಮ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "산타렝 현"), ("lt", "Santaremo apskritis"), ("lv", "Santarēmas distrikts"), ("mr", "सा\u{902}तार\u{947}म जिल\u{94d}हा"), ("ms", "Daerah Santarém"), ("nb", "Santarém"), ("nl", "Santarém"), ("no", "Santarém"), ("pl", "Dystrykt Santarém"), ("pt", "Santarém"), ("ro", "Districtul Santarém"), ("ru", "Сантарен"), ("si", "සැන\u{dca}ටරේම\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Сантарем"), ("sr_Latn", "Santarem"), ("sv", "Santarém"), ("ta", "சண\u{bcd}டரெம\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c3e}ంట\u{c3e}ర\u{c46}మ\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตซ\u{e31}งตาไร"), ("tr", "Santarém District"), ("uk", "Сантарен"), ("ur", "سانتارامی ضلع"), ("vi", "Santarém"), ("zh", "聖塔倫區")]),
                        unofficial_name_list: ["Santarém"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::PT,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(38.5260437), longitude: Some(-8.8909328), max_latitude: Some(38.5641093), min_latitude: Some(38.4840016), max_longitude: Some(-8.7556919), min_longitude: Some(-8.9725114)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيتوبال"), ("be", "акруга Сетубал"), ("bg", "Сетубал"), ("bn", "সেতোব\u{9be}ল জেল\u{9be}"), ("ca", "Setúbal"), ("ccp", "𑄥𑄬𑄑\u{1112a}𑄝\u{11127}𑄣\u{11134}"), ("ceb", "Distrito de Setúbal"), ("da", "Setúbal District"), ("de", "Distrikt Setúbal"), ("el", "Σετούμπαλ"), ("en", "Setúbal"), ("es", "Setúbal"), ("eu", "Setubal"), ("fi", "Setúbalin piiri"), ("fr", "District de Setúbal"), ("gl", "Setúbal"), ("gu", "સ\u{ac7}ત\u{ac1}બલ જિલ\u{acd}લો"), ("hi", "स\u{947}त\u{941}बल जिला"), ("hu", "Setúbal"), ("id", "Distrik Setúbal"), ("it", "distretto di Setúbal"), ("ja", "セトゥーバル県"), ("kn", "ಸ\u{cc6}ಟುಬಾಲ\u{ccd} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "세투발 현"), ("lt", "Setubalo apskritis"), ("lv", "Setubalas distrikts"), ("mr", "स\u{947}त\u{941}बल जिल\u{94d}हा"), ("ms", "Daerah Setúbal"), ("nb", "Setúbal"), ("nl", "Setúbal"), ("no", "Setúbal"), ("pl", "Dystrykt Setúbal"), ("pt", "Setúbal"), ("ro", "Districtul Setúbal"), ("ru", "Сетубал"), ("si", "සෙට\u{dd4}බල\u{dca} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Сетубал"), ("sr_Latn", "Setubal"), ("sv", "Setúbal"), ("ta", "சேட\u{bcd}டுப\u{bbe}ல\u{bcd} ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "స\u{c46}టుబల\u{c4d} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เม\u{e37}องเซต\u{e39}เบล"), ("tr", "Setubal District"), ("uk", "Сетубал"), ("ur", "سیتوبال ضلع"), ("vi", "Setúbal"), ("zh", "塞圖巴爾區")]),
                        unofficial_name_list: ["Setúbal"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::PT,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.6918275), longitude: Some(-8.8344101), max_latitude: Some(41.7605686), min_latitude: Some(41.6714037), max_longitude: Some(-8.7483575), min_longitude: Some(-8.8693403)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فيانادو كاستيلو"), ("bg", "Виана ду Кащелу"), ("bn", "ভিয\u{9bc}\u{9be}ন\u{9be} ড\u{9c1} ক\u{9be}স\u{9cd}টেলো জেল\u{9be}"), ("ca", "Districte de Viana do Castelo"), ("ccp", "𑄞\u{11128}𑄠𑄚 𑄓\u{1112e} 𑄇𑄌\u{11134}𑄑𑄬𑄣\u{1112e}"), ("ceb", "Distrito de Viana do Castelo"), ("da", "Viana do Castelo District"), ("de", "Distrikt Viana do Castelo"), ("el", "Βιάνα ντο Καστέλο"), ("en", "Viana do Castelo"), ("es", "Distrito de Viana do Castelo"), ("eu", "Viana do Castelo"), ("fi", "Viana do Castelon piiri"), ("fr", "District de Viana do Castelo"), ("gl", "Distrito de Viana do Castelo"), ("gu", "વિયાના દો કાસ\u{acd}ટ\u{ac7}લો જિલ\u{acd}લો"), ("hi", "वियाना डो क\u{948}स\u{94d}ट\u{947}लो जिला"), ("hu", "Viana do Castelo"), ("id", "Distrik Viana do Castelo"), ("it", "distretto di Viana do Castelo"), ("ja", "ヴィアナ・ド・カステロ県"), ("ka", "ვიანა-დუ-კაშტელუ"), ("kn", "ವ\u{cbf}ಯಾನಾ ಕ\u{ccd}ಯಾಸ\u{ccd}ಟ\u{cc6}ಲೊ ಡ\u{cbf}ಸ\u{ccd}ಟ\u{ccd}ರ\u{cbf}ಕ\u{ccd}ಟ\u{ccd}"), ("ko", "비아나두카스텔루 현"), ("lt", "Vijanos de Kastelo apskritis"), ("lv", "Viana du Kaštelu distrikts"), ("mr", "वियाना ना कास\u{94d}ट\u{947}लो जिल\u{94d}हा"), ("ms", "Daerah Viana do Castelo"), ("nb", "Viana do Castelo"), ("nl", "Viana do Castelo"), ("no", "Viana do Castelo"), ("pl", "Dystrykt Viana do Castelo"), ("pt", "Viana do Castelo"), ("ro", "Districtul Viana do Castelo"), ("ru", "Виана-ду-Каштелу"), ("si", "ව\u{dd2}ය\u{dcf}න\u{dcf} ඩො කැස\u{dca}ටෙලෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Вијана до Кастело"), ("sr_Latn", "Vijana do Kastelo"), ("sv", "Viana do Castelo"), ("ta", "விய\u{bbe}ன டூ க\u{bbe}ஸ\u{bcd}டெல\u{bcd}லோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "వ\u{c3f}య\u{c3e}న\u{c3e} డూ క\u{c3e}స\u{c4d}ట\u{c46}ల\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เว\u{e35}ยนาด\u{e39}ก\u{e31}ชเตล\u{e39}"), ("tr", "Viano do Castelo District"), ("uk", "Віана-ду-Каштелу"), ("ur", "ویانا دو کاشتیلو ضلع"), ("vi", "Viana do Castelo"), ("zh", "維亞納堡區")]),
                        unofficial_name_list: ["Viana do Castelo"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::PT,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(41.3010351), longitude: Some(-7.7422354), max_latitude: Some(41.3635472), min_latitude: Some(41.2401615), max_longitude: Some(-7.6639146), min_longitude: Some(-7.790294800000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فيلا ريال"), ("be", "акруга Віла-Рэал"), ("bg", "Виля Реал"), ("ca", "Districte de Vila Real"), ("ccp", "𑄞\u{11128}𑄣 𑄢\u{11128}𑄠𑄬𑄣\u{11134}"), ("ceb", "Distrito de Vila Real"), ("de", "Distrikt Vila Real"), ("en", "Vila Real"), ("es", "Distrito de Vila Real"), ("eu", "Vila Real"), ("fi", "Vila Realin piiri"), ("fr", "District de Vila Real"), ("gl", "Distrito de Vila Real"), ("id", "Distrik Vila Real"), ("it", "distretto di Vila Real"), ("ja", "ヴィラ・レアル県"), ("ko", "빌라헤알 현"), ("ms", "Daerah Vila Real"), ("nb", "Vila Real"), ("nl", "Vila Real"), ("no", "Vila Real"), ("pl", "Dystrykt Vila Real"), ("pt", "Vila Real"), ("ro", "Districtul Vila Real"), ("ru", "Вила-Реал"), ("sr", "Вила Реал"), ("sr_Latn", "Vila Real"), ("sv", "Vila Real"), ("uk", "Віла-Реал"), ("ur", "ویلا ریال ضلع"), ("vi", "Vila Real"), ("zh", "雷亞爾城區")]),
                        unofficial_name_list: ["Vila Real"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::PT,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(40.6565861), longitude: Some(-7.9124712), max_latitude: Some(40.7325793), min_latitude: Some(40.6072458), max_longitude: Some(-7.8250273), min_longitude: Some(-7.9660481)}),
                        comments: None,
                        subdivision_type: SubdivisionType::District,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة فيسيو"), ("be", "акруга Візеу"), ("bg", "Визеу"), ("bn", "ভিসেউ জেল\u{9be}"), ("ca", "Districte de Viseu"), ("ccp", "𑄞\u{11128}𑄥𑄬𑄅\u{1112a}"), ("ceb", "Distrito de Viseu"), ("da", "Viseu District"), ("de", "Distrikt Viseu"), ("el", "Βισέου"), ("en", "Viseu"), ("es", "Distrito de Viseu"), ("eu", "Viseu"), ("fi", "Viseun piiri"), ("fr", "District de Viseu"), ("gl", "Distrito de Viseu"), ("gu", "વિઝ\u{ac1} જિલ\u{acd}લો"), ("hi", "विस\u{942} जिला"), ("hu", "Viseu"), ("id", "Distrik Viseu"), ("it", "distretto di Viseu"), ("ja", "ヴィゼウ県"), ("kn", "ವೈಸು ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "비제우 현"), ("lt", "Vizou apskritis"), ("lv", "Viseu distrikts"), ("mr", "विझ\u{942} जिल\u{94d}हा"), ("ms", "Daerah Viseu"), ("nb", "Viseu"), ("nl", "Viseu"), ("no", "Viseu"), ("pl", "Dystrykt Viseu"), ("pt", "Viseu"), ("ro", "Districtul Viseu"), ("ru", "Визеу"), ("si", "ව\u{dd2}සෙය\u{dd4} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Визеу"), ("sr_Latn", "Vizeu"), ("sv", "Viseu"), ("ta", "விஸு ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "వ\u{c3f}స\u{c3f}యూ జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ว\u{e35}เซว"), ("tr", "Viseu District"), ("uk", "Візеу"), ("ur", "ویزیو ضلع"), ("vi", "Viseu"), ("zh", "維塞烏區")]),
                        unofficial_name_list: ["Viseu"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::PT,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(37.7412488), longitude: Some(-25.6755944), max_latitude: Some(39.7261497), min_latitude: Some(36.9278178), max_longitude: Some(-25.0131855), min_longitude: Some(-31.2687948)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Asore"), ("am", "አዞሬስ"), ("ar", "الأزور"), ("az", "Azor adaları"), ("be", "Азорскія астравы"), ("bg", "Азорски острови"), ("bn", "অ\u{9cd}য\u{9be}জোরেস"), ("bs", "Azores"), ("ca", "Açores"), ("ccp", "𑄃𑄎\u{1112e}𑄢\u{11128}𑄌\u{11134}"), ("ceb", "Azores"), ("cs", "Azory"), ("cy", "Azores"), ("da", "Azorerne"), ("de", "Azoren"), ("el", "Αζόρες"), ("en", "Azores"), ("es", "Azores"), ("et", "Assoorid"), ("eu", "Azoreak"), ("fa", "آزور"), ("fi", "Azorit"), ("fr", "Açores"), ("ga", "Na hAsóir"), ("gl", "Azores"), ("gu", "એઝોર\u{acd}સ"), ("he", "האיים האזוריים"), ("hi", "एज\u{93c}ोर\u{947}स"), ("hr", "Azori"), ("hu", "Azori-szigetek"), ("hy", "Ազորյան կղզիներ"), ("id", "Azores"), ("is", "Asóreyjar"), ("it", "Azzorre"), ("ja", "アゾレス諸島"), ("jv", "Azores"), ("ka", "აზორის კუნძულები"), ("kk", "Азор аралдары"), ("kn", "ಅಝೊರ\u{ccd}ಸ\u{ccd}"), ("ko", "아소르스 제도"), ("lt", "Azorai"), ("lv", "Azoru salas"), ("mk", "Азорски Острови"), ("mn", "Азорын арлууд"), ("mr", "असोर\u{947}स"), ("ms", "Azores"), ("nb", "Asorene"), ("nl", "Azoren"), ("no", "Asorene"), ("pl", "Azory"), ("pt", "Açores"), ("ro", "Azore"), ("ru", "Азорские острова"), ("si", "ඇසෝර\u{dca}ස\u{dca}"), ("sk", "Azory"), ("sl", "Azori"), ("sq", "Ishujt Azore"), ("sr", "Азорска острва"), ("sr_Latn", "Azorska ostrva"), ("sv", "Azorerna"), ("sw", "Azori"), ("ta", "அஸோர\u{bcd}ஸ\u{bcd}"), ("te", "అజ\u{c4b}ర\u{c46}స\u{c4d}"), ("th", "อะโซร\u{e4c}ส"), ("tk", "Azor adalary"), ("tr", "Azorlar"), ("uk", "Азорські острови"), ("ur", "آزورس"), ("uz", "Azor orollari"), ("vi", "Açores"), ("yo", "Àwọn Azore"), ("yo_BJ", "Àwɔn Azore"), ("yue", "亞速爾"), ("yue_Hans", "亚速尔"), ("zh", "亚速尔群岛")]),
                        unofficial_name_list: ["Açores"].to_vec(),
                    }
                ),
                (
                    "30",
                    Subdivision{
                        name: "30",
                        country_alpha2: Alpha2::PT,
                        code: "30",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(32.76070740000001), longitude: Some(-16.9594723), max_latitude: Some(33.1281375), min_latitude: Some(30.0303451), max_longitude: Some(-15.8566922), min_longitude: Some(-17.2659373)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousRegion,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Madeiraeilande"), ("am", "ማደይራ"), ("ar", "جزر ماديرا"), ("az", "Madeyra"), ("be", "Мадэйра"), ("bg", "Мадейра"), ("bs", "Madeira"), ("ca", "Madeira"), ("ccp", "𑄟𑄓𑄬\u{1112d}𑄢"), ("ceb", "Madeira"), ("cs", "Madeira"), ("cy", "Madeira"), ("da", "Madeira"), ("de", "Autonome Region Madeira"), ("el", "Μαδέρα"), ("en", "Madeira"), ("es", "Madeira"), ("et", "Madeira"), ("eu", "Madeira"), ("fa", "مادیرا"), ("fi", "Madeira"), ("fr", "Madère"), ("ga", "Maidéara"), ("gl", "Rexión Autónoma da Madeira"), ("he", "מדיירה"), ("hi", "मद\u{947}रा"), ("hr", "Madeira"), ("hu", "Madeira-szigetek"), ("hy", "Մադեյրա"), ("id", "Madeira"), ("is", "Madeiraeyjar"), ("it", "Madera"), ("ja", "マデイラ諸島"), ("jv", "Madeira"), ("ka", "მადეირა"), ("ko", "마데이라 제도"), ("ky", "Мадейра аралдар тобу"), ("lt", "Madeira"), ("lv", "Madeira"), ("mk", "Мадеира"), ("mr", "माद\u{947}ईरा"), ("ms", "Madeira"), ("my", "မဒ\u{102e}းရကျ\u{103d}န\u{103a}းများ"), ("nb", "Madeira"), ("nl", "Madeira"), ("no", "Madeira"), ("pl", "Madera"), ("pt", "Região Autónoma da Madeira"), ("ro", "Madeira"), ("ru", "Мадейра"), ("sk", "Madeira"), ("sl", "Madeira"), ("sq", "Ishujt Medeira"), ("sr", "Мадеира"), ("sr_Latn", "Madeira"), ("sv", "Madeira"), ("sw", "Visiwa vya Madeira"), ("ta", "மத\u{bc0}ர\u{bbe}"), ("th", "มาเดรา"), ("tr", "Madeira Adaları"), ("uk", "Мадейра"), ("ur", "مادیعیرا"), ("vi", "Madeira"), ("yo", "Madeira"), ("yo_BJ", "Madeira"), ("yue", "馬德拉"), ("yue_Hans", "马德拉"), ("zh", "馬德拉"), ("zu", "IMadira")]),
                        unofficial_name_list: ["Madeira"].to_vec(),
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
#[cfg(feature = "pt")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::PT,
        alpha3: Alpha3::PRT,
        address_format: Some(
            "{{recipient}}\n{{street}}\n{{postalcode}} {{city}} {{region_short}}\n{{country}}",
        ),
        continent: Continent::Europe,
        country_code: 351,
        currency_code: CurrencyCode::EUR,
        gec: Some(GEC::PO),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some(IOC::POR),
        iso_long_name: "The Portuguese Republic",
        iso_short_name: "Portugal",
        official_language_list: ["pt"].to_vec(),
        spoken_language_list: ["pt"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [9].to_vec(),
        national_prefix: "None",
        nationality: Some("Portuguese"),
        number: "620",
        postal_code: true,
        postal_code_format: Some("\\d{4}-\\d{3}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "PT",
        unofficial_name_list: ["Portugal", "ポルトガル"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Portugal"),
            ("af", "Portugal"),
            ("ak", "Portugal"),
            ("am", "ፖርቱጋል"),
            ("an", "Portugal"),
            ("ar", "البرتغال"),
            ("as", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
            ("ay", "Portugal"),
            ("az", "Portuqaliya"),
            ("ba", "Portugal"),
            ("be", "Партугалія"),
            ("bg", "Португалия"),
            ("bi", "Portugal"),
            ("bn", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
            ("bn_IN", "পোর\u{9cd}ট\u{9c1}গ\u{9be}ল"),
            ("br", "Portugal"),
            ("bs", "Portugal"),
            ("ca", "Portugal"),
            ("ce", "Португали"),
            ("ch", "Portugal"),
            ("cs", "Portugalsko"),
            ("cv", "Португали"),
            ("cy", "Portiwgal"),
            ("da", "Portugal"),
            ("de", "Portugal"),
            ("dv", "ޕ\u{7af}ޗ\u{7aa}ގ\u{7a6}ލ\u{7b0}"),
            ("dz", "པ\u{f7c}ར་ཊ\u{f74}་ག\u{f71}ལ།"),
            ("ee", "Portugal"),
            ("el", "Πορτογαλία"),
            ("en", "Portugal"),
            ("eo", "Portugalio"),
            ("es", "Portugal"),
            ("et", "Portugal"),
            ("eu", "Portugal"),
            ("fa", "پرتغال"),
            ("ff", "Portokeesi"),
            ("fi", "Portugali"),
            ("fo", "Portugal"),
            ("fr", "Portugal"),
            ("fy", "Portegal"),
            ("ga", "An Phortaingéil"),
            ("gl", "Portugal"),
            ("gn", "Portugal"),
            ("gu", "પોર\u{acd}ટ\u{ac1}ગલ"),
            ("gv", "Yn Phortiugal"),
            ("ha", "Portugal"),
            ("he", "פורטוגל"),
            ("hi", "प\u{941}र\u{94d}तगाल"),
            ("hr", "Portugal"),
            ("ht", "Pòtigal"),
            ("hu", "Portugália"),
            ("hy", "Պորտուգալիա"),
            ("ia", "Portugal"),
            ("id", "Portugal"),
            ("io", "Portugal"),
            ("is", "Portúgal"),
            ("it", "Portogallo"),
            ("iu", "Portugal"),
            ("ja", "ポルトガル"),
            ("ka", "პორტუგალია"),
            ("ki", "Portugal"),
            ("kk", "Португалия"),
            ("kl", "Portugal"),
            ("km", "ព\u{17d0}រទ\u{17bb}យហ\u{17d2}គាល\u{17cb}"),
            ("kn", "ಪೋರ\u{ccd}ತುಗಾಲ\u{ccd}"),
            ("ko", "포르투갈"),
            ("ku", "Portekîz"),
            ("kv", "Португалия"),
            ("kw", "Portyngal"),
            ("ky", "Португалия"),
            ("lo", "ປະເທດປອກຕ\u{eb8}ຍການ"),
            ("lt", "Portugalija"),
            ("lv", "Portugāle"),
            ("mi", "Potukara"),
            ("mk", "Португалија"),
            ("ml", "പോര\u{d4d}\u{200d}ച\u{d4d}ച\u{d41}ഗല\u{d4d}\u{200d}"),
            ("mn", "Португаль"),
            ("mr", "पोर\u{94d}त\u{941}गाल"),
            ("ms", "Feringgi"),
            ("mt", "Portugall"),
            (
                "my",
                "ပေါ\u{103a}တ\u{1030}ဂ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Portsiugar"),
            ("nb", "Portugal"),
            ("ne", "पोर\u{94d}च\u{941}गल"),
            ("nl", "Portugal"),
            ("nn", "Portugal"),
            ("nv", "Portugal"),
            ("oc", "Portugal"),
            ("or", "ପର\u{b4d}ତ\u{b41}ଗ\u{b3e}ଲ"),
            ("pa", "ਪ\u{a41}ਰਤਗਾਲ"),
            ("pi", "प\u{941}र\u{94d}तगाल"),
            ("pl", "Portugalia"),
            ("ps", "پورتګال"),
            ("pt", "Portugal"),
            ("pt_BR", "Portugal"),
            ("ro", "Portugalia"),
            ("ru", "Португалия"),
            ("rw", "Porutigali"),
            ("sc", "Portugallu"),
            ("sd", "پرتگال"),
            ("si", "පෘත\u{dd4}ග\u{dcf}ලය"),
            ("sk", "Portugalsko"),
            ("sl", "Portugalska"),
            ("so", "Bortuqaal"),
            ("sq", "Portugali"),
            ("sr", "Португал"),
            ("sv", "Portugal"),
            ("sw", "Portugal"),
            ("ta", "போர\u{bcd}ச\u{bcd}சுகல\u{bcd}"),
            ("te", "ప\u{c4b}ర\u{c4d}చుగల\u{c4d}"),
            ("tg", "Португалия"),
            ("th", "โปรต\u{e38}เกส"),
            ("ti", "ፖርቱጋል"),
            ("tk", "Portugaliýa"),
            ("tl", "Portugal"),
            ("tr", "Portekiz"),
            ("tt", "Портуgалиа"),
            ("ug", "پورتۇگالىيە"),
            ("uk", "Португалія"),
            ("ur", "پرتگال"),
            ("uz", "Portugaliya"),
            ("ve", "Portugal"),
            ("vi", "Bồ Đào Nha"),
            ("wa", "Portugal"),
            ("wo", "Portugaal"),
            ("xh", "Portugal"),
            ("yo", "Pọ\u{301}rtúgàl"),
            ("zh_CN", "葡萄牙"),
            ("zh_HK", "葡萄牙"),
            ("zh_TW", "葡萄牙"),
            ("zu", "IPhothugali"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
