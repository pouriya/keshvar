// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Bosnia and Herzegovina

#[cfg(all(feature = "ba", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::BA;
    pub const ALPHA3: Alpha3 = Alpha3::BIH;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 387;
    pub const CURRENCY_CODE: &str = "BAM";
    pub const GEC: Option<GEC> = Some(GEC::BK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("BIH");
    pub const ISO_SHORT_NAME: &str = "Bosnia and Herzegovina";
    pub const ISO_LONG_NAME: &str = "Bosnia and Herzegovina";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["bs", "hr", "sr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["bs", "hr", "sr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Bosnian, Herzegovinian");
    pub const NUMBER: &str = "070";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernEurope);
    pub const UN_LOCODE: &str = "BA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Bosnia and Herzegovina",
        "Bosnien und Herzegowina",
        "Bosnie et Herzégovine",
        "Bosnia y Herzegovina",
        "ボスニア・ヘルツェゴビナ",
        "Bosnië en Herzegovina",
        "Bosnia Herzegovina",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Bosnia and Herzegovina"),
        ("af", "Bosnië en Herzegowina"),
        ("ak", "Bosnia and Herzegovina"),
        ("am", "ቦስኒ። ጕና ሄሴፕጕቱኒ።"),
        ("an", "Bosnia y Herzegovina"),
        ("ar", "البوسنة و الهرسك"),
        ("as", "বোছনীয\u{9be} আৰ\u{9c1} হ\u{9be}ৰজেগোইন\u{9be}"),
        ("ay", "Bosnia and Herzegovina"),
        ("az", "Bosniya və Herzoqovina"),
        ("ba", "Bosnia and Herzegovina"),
        ("be", "Боснія і Герцагавіна"),
        ("bg", "Босна и Херцеговина"),
        ("bi", "Bosnia and Herzegovina"),
        ("bn", "বসনিয়\u{9be} ও হ\u{9be}র\u{9cd}জগোভিন\u{9be}"),
        ("bn_IN", "বসনিয়\u{9be} ও হ\u{9be}র\u{9cd}জগোভিন\u{9be}"),
        ("br", "Bosnia-ha-Herzegovina"),
        ("bs", "Bosna i Hercegovina"),
        ("ca", "Bòsnia i Hercegovina"),
        ("ce", "Босни а Герцеговина"),
        ("ch", "Bosnia and Herzegovina"),
        ("cs", "Bosna a Hercegovina"),
        ("cv", "Боснипа Герцеговина"),
        ("cy", "Bosnia a Herzegovina"),
        ("da", "Bosnien-Hercegovina"),
        ("de", "Bosnien und Herzegowina"),
        ("dv", "ބ\u{7ae}ސ\u{7b0}ނ\u{7a8}ޔ\u{7a7} އ\u{7ac}ނ\u{7b0}ޑ\u{7b0} ހ\u{7ac}ރ\u{7b0}ޒ\u{7a8}ގ\u{7ae}ވ\u{7a9}ނ\u{7a7}"),
        ("dz", "བ\u{f71}\u{f7c}ས་ན\u{f72}་ཡ་དང་ཧར་ཟ\u{f72}་ག\u{f71}\u{f7c}་བ\u{f71}\u{f72}་ན\u{f71}།"),
        ("ee", "Bosnia and Herzegovina"),
        ("el", "Βοσνία και Ερζεγοβίνη"),
        ("en", "Bosnia and Herzegovina"),
        ("eo", "Bosnio kaj Hercegovino"),
        ("es", "Bosnia y Herzegovina"),
        ("et", "Bosnia ja Hertsegoviina"),
        ("eu", "Bosnia eta Herzegovina"),
        ("fa", "بوسنی و هرزگوین"),
        ("ff", "Bosniya"),
        ("fi", "Bosnia-Hertsegovina"),
        ("fo", "Bosnia-Hersegovina"),
        ("fr", "Bosnie-Herzégovine"),
        ("fy", "Bosnje"),
        ("ga", "An Bhoisnia-Heirseagaivéin"),
        ("gl", "Bosnia e Hercegovina"),
        ("gn", "Bosnia and Herzegovina"),
        ("gu", "બોસ\u{acd}નિયા અન\u{ac7} હર\u{acd}ઝ\u{ac7}ગોવીનીઆ"),
        ("gv", "Bosnia as Herzegovina"),
        ("ha", "Herzegovina"),
        ("he", "בוסניה והרצגובינה"),
        ("hi", "बॉस\u{94d}निया और हर\u{94d}ज\u{93c}\u{947}गोविना"),
        ("hr", "Bosna i Hercegovina"),
        ("ht", "Bosnia and Herzegovina"),
        ("hu", "Bosznia-Hercegovina"),
        ("hy", "Բոսնիա-Հերցեգովինա"),
        ("ia", "Bosnia e Herzegovina"),
        ("id", "Bosnia dan Herzegovina"),
        ("io", "Bosnia e Herzegovina"),
        ("is", "Bosnía og Hersegóvína"),
        ("it", "Bosnia-Erzegovina"),
        ("iu", "Bosnia and Herzegovina"),
        ("ja", "ボスニア・ヘルツェゴビナ"),
        ("ka", "ბოსნია და ჰერცეგოვინა"),
        ("ki", "Mbocinia na Hecengobina"),
        ("kk", "Босния және Герцеговина"),
        ("kl", "Bosnia and Herzegovina"),
        ("km", "ប\u{17bc}ស\u{17d2}ន\u{17ca}\u{17b8} ន\u{17b7}ង \u{200b}ហ\u{17ba}ហ\u{17d2}ស\u{17ca}េហ\u{17d2}គោវ\u{17b8}ណា"),
        ("kn", "ಭೊಸ\u{ccd}ನ\u{cbf}ಯಾ ಮತ\u{ccd}ತು ಹ\u{cc6}ರ\u{ccd}ಜ\u{cc6}ಗೋವ\u{cbf}ನಾ"),
        ("ko", "보스니아 헤르체고비나"),
        ("ku", "Bosna Hersek"),
        ("kv", "Босния да Герцеговина"),
        ("kw", "Bosni–Hercegovina"),
        ("ky", "Босния жана Герцеговина"),
        ("lo", "ປະເທດບ\u{ebb}ດສະນ\u{eb5}ແຮກເຊໂກວ\u{eb5}ນ"),
        ("lt", "Bosnija ir Hercegovina"),
        ("lv", "Bosnija un Hercegovina"),
        ("mi", "Pōngia-Herekōmina"),
        ("mk", "Босна и Херцеговина"),
        ("ml", "ബോസ\u{d4d}നിയയ\u{d41}ം ഹെര\u{d4d}\u{200d}സഗോവിനയ\u{d41}ം"),
        ("mn", "Босний ба Нерцеговин"),
        ("mr", "बोस\u{94d}निया आणी हर\u{94d}झ\u{947}गोव\u{94d}हिना"),
        ("ms", "Bosnia dan Herzegovina"),
        ("mt", "Bożnija Ħerżegovina"),
        ("my", "ဘော\u{1037}စန\u{102e}းယားန\u{103e}င\u{1037}\u{103a} ဟာဇ\u{102e}ဂ\u{102d}\u{102f}ဗ\u{102e}းနားန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Boteniya me Erdegobina"),
        ("nb", "Bosnia-Hercegovina"),
        ("ne", "बोस\u{94d}नीया र हर\u{94d}जगोभिना"),
        ("nl", "Bosnië en Herzegovina"),
        ("nn", "Bosnia-Hercegovina"),
        ("nv", "Bosna dóó Hetsog Bikéyah"),
        ("oc", "Bòsnia e Ercegovina"),
        ("or", "ବୋସନ\u{b3f}ଆ ଏବଂ ହର\u{b4d}ଜେଗୋଭ\u{b3f}ନ\u{b3e}"),
        ("pa", "ਬ\u{a4b}ਸਨੀਆ ਤ\u{a47} ਹਰਜ਼ੀਗ\u{a4b}ਨੀਆ"),
        ("pi", "बास\u{94d}निया"),
        ("pl", "Bośnia i Hercegowina"),
        ("ps", "Bosnia and Herzegovina"),
        ("pt", "Bósnia e Herzegovina"),
        ("pt_BR", "Bósnia-Herzegóvina"),
        ("ro", "Bosnia și Herțegovina"),
        ("ru", "Босния и Герцеговина"),
        ("rw", "Bosiniya na Herizegovina"),
        ("sc", "Bòsnia e Erzegòvina"),
        ("sd", "Bosnia and Herzegovina"),
        ("si", "බොස\u{dca}න\u{dd2}ය\u{dcf}ව හ\u{dcf} හර\u{dca}සගෝව\u{dd2}න\u{dd2}ය\u{dcf}ව"),
        ("sk", "Bosna a Hercegovina"),
        ("sl", "Bosna in Hercegovina"),
        ("so", "Boosniya Heersigoviina"),
        ("sq", "Bosnjë dhe Hercegovinë"),
        ("sr", "Босна и Херцеговина"),
        ("sv", "Bosnien-Hercegovina"),
        ("sw", "Bosnia na Herzegovina"),
        ("ta", "போஸ\u{bcd}னிய\u{bbe} ஹெர\u{bcd}ஸிகோவின\u{bbe}"),
        ("te", "బ\u{c4b}స\u{c4d}న\u{c3f}య\u{c3e} మర\u{c3f}యు హర\u{c4d}ఝ\u{c47}గ\u{c4b}వ\u{c4d}హ\u{c3f}న\u{c3e}"),
        ("tg", "Босния ва Ҳерсеговина"),
        ("th", "บอสเน\u{e35}ยและเฮอร\u{e4c}เซโกว\u{e35}นา"),
        ("ti", "ቦስኒያ እና ሄርዞጎቪኒያ"),
        ("tk", "Bosniýa we Gersogowina"),
        ("tl", "Bosnia at Herzegovina"),
        ("tr", "Bosna-Hersek"),
        ("tt", "Босниа белән Һерзеgовина"),
        ("ug", "بوسنىيە گېرتسېگوۋىنا"),
        ("uk", "Боснія і Герцеговина"),
        ("ur", "بوسنیا و ہرزیگووینا"),
        ("uz", "Bosniya va Gersegovina"),
        ("ve", "Bosnia na Herzegovina"),
        ("vi", "Bô-xni-a và Hẻ-xê-gô-vi-na"),
        ("wa", "Bosneye"),
        ("wo", "Bosni Hersegowin"),
        ("xh", "Bosnia and Herzegovina"),
        ("yo", "Bósníà àti Hẹrjẹgòfínà"),
        ("zh_CN", "波斯尼亚和黑塞哥维那"),
        ("zh_HK", "波斯尼亞和黑塞哥維那"),
        ("zh_TW", "波士尼亞及赫塞哥維納"),
        ("zu", "IBhosniya neHerzegovina"),
];
    #[cfg(all(feature = "ba", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 43.915886;
        pub const LONGITUDE: f64 = 17.679076;
        pub const MAX_LATITUDE: f64 = 45.2766262;
        pub const MAX_LONGITUDE: f64 = 19.6237016;
        pub const MIN_LATITUDE: f64 = 42.5564808;
        pub const MIN_LONGITUDE: f64 = 15.7223665;
        pub const NORTHEAST_LATITUDE: f64 = 45.2766262;
        pub const NORTHEAST_LONGITUDE: f64 = 19.6237016;
        pub const SOUTHWEST_LATITUDE: f64 = 42.5564808;
        pub const SOUTHWEST_LONGITUDE: f64 = 15.7223665;
    }
}
#[cfg(all(feature = "ba", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 43.915886,
            longitude: 17.679076,
            max_latitude: 45.2766262,
            max_longitude: 19.6237016,
            min_latitude: 42.5564808,
            min_longitude: 15.7223665,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 45.2766262,
                    longitude: 19.6237016,
                },
                southwest: CountryGeoBound {
                    latitude: 42.5564808,
                    longitude: 15.7223665,
                },
            },
        }
    }
}

#[cfg(all(feature = "ba", feature = "subdivisions"))]
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
                    "BIH",
                    Subdivision{
                        name: "BIH",
                        country_alpha2: Alpha2::BA,
                        code: "BIH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(43.8874897), longitude: Some(17.842793), max_latitude: Some(45.2271323), min_latitude: Some(42.6075035), max_longitude: Some(19.0392512), min_longitude: Some(15.7237473)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Entity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "اتحاد البوسنة والهرسك"), ("az", "Bosniya və Herseqovina Federasiyası"), ("be", "Федэрацыя Босніі і Герцагавіны"), ("bg", "Федерация Босна и Херцеговина"), ("bs", "Federacija Bosne i Hercegovine"), ("ca", "Federació de Bòsnia i Hercegovina"), ("ccp", "𑄜𑄬𑄓𑄢𑄬𑄥\u{11127}𑄚\u{11134} 𑄃\u{11127}𑄛\u{11134} 𑄝\u{11127}𑄌\u{11134}𑄚\u{11128}𑄠 𑄃𑄬𑄚\u{11133}𑄓\u{11134} 𑄦𑄢\u{11134}𑄎\u{1112e}𑄉\u{11127}𑄞\u{11128}𑄚"), ("ceb", "Federation of Bosnia and Herzegovina"), ("cs", "Federace Bosny a Hercegoviny"), ("de", "Föderation Bosnien und Herzegowina"), ("el", "Ομοσπονδία Βοσνίας-Ερζεγοβίνης"), ("en", "Federation of Bosnia and Herzegovina"), ("es", "Federación de Bosnia y Herzegovina"), ("et", "Bosnia ja Hertsegoviina Föderatsioon"), ("eu", "Bosnia-Herzegovinako Federazioa"), ("fa", "فدراسیون بوسنی و هرزگوین"), ("fi", "Bosnia ja Hertsegovinan federaatio"), ("fr", "Fédération de Bosnie-et-Herzégovine"), ("gl", "Federación de Bosnia e Hercegovina"), ("he", "הפדרציה של בוסניה והרצגובינה"), ("hr", "Federacija Bosne i Hercegovine"), ("hu", "Bosznia-hercegovinai Föderáció"), ("hy", "Բոսնիա և Հերցեգովինա ֆեդերացիա"), ("id", "Federasi Bosnia dan Herzegovina"), ("it", "Federazione di Bosnia ed Erzegovina"), ("ja", "ボスニア・ヘルツェゴビナ連邦"), ("jv", "Federasi Bosnia Herzegovina"), ("ka", "ბოსნია და ჰერცეგოვინის ფედერაცია"), ("kk", "Босния және Герцеговина федерациясы"), ("ko", "보스니아 헤르체고비나 연방"), ("lt", "Bosnijos ir Hercegovinos Federacija"), ("mk", "Федерација Босна и Херцеговина"), ("ms", "Persekutuan Bosnia dan Herzegovina"), ("nb", "Føderasjonen av Bosnia og Hercegovina"), ("nl", "Federatie van Bosnië en Herzegovina"), ("no", "Føderasjonen av Bosnia og Hercegovina"), ("pl", "Federacja Bośni i Hercegowiny"), ("pt", "Federação da Bósnia e Herzegovina"), ("ro", "Federația Bosniei și Herțegovinei"), ("ru", "Федерация Боснии и Герцеговины"), ("sl", "Federacija Bosne in Hercegovine"), ("sq", "Federata e Bosnjës dhe Hercegovinës"), ("sr", "Федерација Босне и Херцеговине"), ("sr_Latn", "Federacija Bosne i Hercegovine"), ("sv", "Federationen Bosnien och Hercegovina"), ("th", "สหพ\u{e31}นธร\u{e31}ฐบอสเน\u{e35}ยและเฮอร\u{e4c}เซโกว\u{e35}นา"), ("tr", "Bosna-Hersek Federasyonu"), ("uk", "Федерація Боснія і Герцеговина"), ("ur", "وفاق بوسنیا و ہرزیگووینا"), ("vi", "Liên bang Bosna và Hercegovina"), ("zh", "波士尼亞與赫塞哥維納聯邦")]),
                        unofficial_name_list: ["Federacija Bosna i Hercegovina"].to_vec(),
                    }
                ),
                (
                    "BRC",
                    Subdivision{
                        name: "BRC",
                        country_alpha2: Alpha2::BA,
                        code: "BRC",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::DistrictWithSpecialStatus,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة برتشو"), ("be", "Акруга Брчко"), ("bg", "Бръчко"), ("bn", "ব\u{9be}ক\u{9cd}ক\u{9c1} জেল\u{9be}"), ("bs", "Brčko Distrikt"), ("ca", "Districte de Brčko"), ("ccp", "𑄝\u{11133}𑄢\u{11127}𑄇\u{11134}𑄇\u{1112e} 𑄎𑄬𑄣"), ("ceb", "Brčko (entidad)"), ("cs", "Brčko"), ("da", "Brčko"), ("de", "Distrikt Brčko"), ("el", "Μπρσκο"), ("en", "Brčko District"), ("es", "Distrito de Brčko"), ("et", "Brčko ringkond"), ("eu", "Brčkoko Barrutia"), ("fa", "بخش برچکو"), ("fi", "Brčkon kaupunginosa"), ("fr", "District de Brčko"), ("gl", "Distrito de Brčko"), ("gu", "બ\u{acd}રકો જિલ\u{acd}લો"), ("he", "מחוז ברצ׳קו"), ("hi", "ब\u{94d}राको जिला"), ("hr", "Brčko Distrikt Bosne i Hercegovine"), ("hu", "Brčkói Körzet"), ("id", "Distrik Brčko"), ("it", "Distretto di Brčko"), ("ja", "ブルチコ行政区"), ("jv", "Distrik Brčko"), ("ka", "ბრჩკოს ოლქი"), ("kn", "ಬ\u{ccd}ರ\u{cc6}ಸ\u{ccd}ಕೊ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "브르치코 행정구"), ("lt", "Brčko apygarda"), ("lv", "Brčko apgabals"), ("mk", "Брчко"), ("mr", "ब\u{94d}र\u{901}चको जिल\u{94d}हा"), ("ms", "Brcko District"), ("nb", "Brčko-distriktet"), ("nl", "Brčko"), ("no", "Brčko-distriktet"), ("pl", "Dystrykt Brczko"), ("pt", "Distrito de Brčko"), ("ro", "Districtul Brčko"), ("ru", "Округ Брчко"), ("si", "බ\u{dca}ර\u{dca}කෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sk", "Dištrikt Brčko"), ("sl", "Distrikt Brčko"), ("sq", "Distrikti i Brçkos"), ("sr", "Брчко Дистрикт"), ("sr_Latn", "Brčko Distrikt"), ("sv", "Brčko"), ("ta", "ப\u{bcd}ரக\u{bcd}க\u{bcd}கோ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "బ\u{c4d}రక\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "เขตบร\u{e36}ชโก"), ("tr", "Brçko Bölgesi"), ("uk", "Округ Брчко"), ("ur", "ضلع برچکو"), ("vi", "Quận Brčko"), ("zh", "布爾奇科特區")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "SRP",
                    Subdivision{
                        name: "SRP",
                        country_alpha2: Alpha2::BA,
                        code: "SRP",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(44.7280186), longitude: Some(17.3148136), max_latitude: Some(45.2737124), min_latitude: Some(42.5561996), max_longitude: Some(19.6256174), min_longitude: Some(16.1924385)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Entity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "جمهورية صرب البوسنة"), ("az", "Bosniya Serb Respublikası"), ("be", "Рэспубліка Сербская"), ("bg", "Република Сръбска"), ("bn", "রেপ\u{9c1}বলিক\u{9be} সর\u{9cd}পস\u{9cd}ক\u{9be}"), ("bs", "Republika Srpska"), ("ca", "República Sèrbia"), ("ccp", "𑄢𑄬𑄛𑄛\u{11134}𑄣\u{11128}𑄇 𑄃𑄬𑄌\u{11134}𑄃𑄢\u{11134}𑄛\u{11128}𑄃\u{11128}𑄌\u{11134}𑄇\u{11133}𑄃"), ("ceb", "Republika Srpska"), ("cs", "Republika srbská"), ("da", "Republika Srpska"), ("de", "Republika Srpska"), ("el", "Σερβική Δημοκρατία"), ("en", "Republika Srpska"), ("es", "República Srpska"), ("et", "Serblaste Vabariik"), ("eu", "Bosniako Serbiar Errepublika"), ("fa", "جمهوری صرب بوسنی"), ("fi", "Serbitasavalta"), ("fr", "République serbe de Bosnie"), ("gl", "República Serbia - Republika Srpska"), ("he", "רפובליקה סרפסקה"), ("hr", "Republika Srpska"), ("hu", "Boszniai Szerb Köztársaság"), ("hy", "Սերբական Հանրապետություն"), ("id", "Republik Srpska"), ("it", "Repubblica Serba di Bosnia ed Erzegovina"), ("ja", "スルプスカ共和国"), ("jv", "Republika Srpska"), ("ka", "ბოსნიელთა რესპუბლიკა"), ("ko", "스릅스카 공화국"), ("ky", "Серб Республикасы"), ("lt", "Serbų respublika"), ("lv", "Serbu Republika"), ("mk", "Република Српска"), ("mr", "स\u{94d}राप\u{94d}स\u{94d}काच\u{947} प\u{94d}रजासत\u{94d}ताक"), ("ms", "Republika Srpska"), ("nb", "Republika Srpska"), ("nl", "Servische Republiek"), ("no", "Republika Srpska"), ("pl", "Republika Serbska"), ("pt", "República Sérvia"), ("ro", "Republika Srpska"), ("ru", "Республика Сербская"), ("sk", "Republika srbská"), ("sl", "Republika srbska"), ("sq", "Republika Serbe"), ("sr", "Република Српска"), ("sr_Latn", "Republika Srpska"), ("sv", "Republika Srpska"), ("ta", "சிறுப\u{bcd}ஸ\u{bcd}க\u{bbe} குடியரசு"), ("th", "สาธารณร\u{e31}ฐเซ\u{e34}ร\u{e4c}ปสกา"), ("tr", "Sırp Cumhuriyeti"), ("uk", "Республіка Сербська"), ("ur", "سرپسکا"), ("vi", "Cộng hòa Srpska"), ("yo", "Sérbíà Orílẹ\u{300}-èdè Olómìnira"), ("yo_BJ", "Sérbíà Orílɛ\u{300}-èdè Olómìnira"), ("zh", "塞族共和國")]),
                        unofficial_name_list: ["Republika Srpska"].to_vec(),
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
#[cfg(feature = "ba")]
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::BA,
        alpha3: Alpha3::BIH,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 387,
        currency_code: "BAM",
        gec: Some(GEC::BK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("BIH"),
        iso_long_name: "Bosnia and Herzegovina",
        iso_short_name: "Bosnia and Herzegovina",
        official_language_list: ["bs", "hr", "sr"].to_vec(),
        spoken_language_list: ["bs", "hr", "sr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Bosnian, Herzegovinian"),
        number: "070",
        postal_code: true,
        postal_code_format: Some("\\d{5}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthernEurope),
        un_locode: "BA",
        unofficial_name_list: ["Bosnia and Herzegovina", "Bosnien und Herzegowina", "Bosnie et Herzégovine", "Bosnia y Herzegovina", "ボスニア・ヘルツェゴビナ", "Bosnië en Herzegovina", "Bosnia Herzegovina"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Bosnia and Herzegovina"), ("af", "Bosnië en Herzegowina"), ("ak", "Bosnia and Herzegovina"), ("am", "ቦስኒ። ጕና ሄሴፕጕቱኒ።"), ("an", "Bosnia y Herzegovina"), ("ar", "البوسنة و الهرسك"), ("as", "বোছনীয\u{9be} আৰ\u{9c1} হ\u{9be}ৰজেগোইন\u{9be}"), ("ay", "Bosnia and Herzegovina"), ("az", "Bosniya və Herzoqovina"), ("ba", "Bosnia and Herzegovina"), ("be", "Боснія і Герцагавіна"), ("bg", "Босна и Херцеговина"), ("bi", "Bosnia and Herzegovina"), ("bn", "বসনিয়\u{9be} ও হ\u{9be}র\u{9cd}জগোভিন\u{9be}"), ("bn_IN", "বসনিয়\u{9be} ও হ\u{9be}র\u{9cd}জগোভিন\u{9be}"), ("br", "Bosnia-ha-Herzegovina"), ("bs", "Bosna i Hercegovina"), ("ca", "Bòsnia i Hercegovina"), ("ce", "Босни а Герцеговина"), ("ch", "Bosnia and Herzegovina"), ("cs", "Bosna a Hercegovina"), ("cv", "Боснипа Герцеговина"), ("cy", "Bosnia a Herzegovina"), ("da", "Bosnien-Hercegovina"), ("de", "Bosnien und Herzegowina"), ("dv", "ބ\u{7ae}ސ\u{7b0}ނ\u{7a8}ޔ\u{7a7} އ\u{7ac}ނ\u{7b0}ޑ\u{7b0} ހ\u{7ac}ރ\u{7b0}ޒ\u{7a8}ގ\u{7ae}ވ\u{7a9}ނ\u{7a7}"), ("dz", "བ\u{f71}\u{f7c}ས་ན\u{f72}་ཡ་དང་ཧར་ཟ\u{f72}་ག\u{f71}\u{f7c}་བ\u{f71}\u{f72}་ན\u{f71}།"), ("ee", "Bosnia and Herzegovina"), ("el", "Βοσνία και Ερζεγοβίνη"), ("en", "Bosnia and Herzegovina"), ("eo", "Bosnio kaj Hercegovino"), ("es", "Bosnia y Herzegovina"), ("et", "Bosnia ja Hertsegoviina"), ("eu", "Bosnia eta Herzegovina"), ("fa", "بوسنی و هرزگوین"), ("ff", "Bosniya"), ("fi", "Bosnia-Hertsegovina"), ("fo", "Bosnia-Hersegovina"), ("fr", "Bosnie-Herzégovine"), ("fy", "Bosnje"), ("ga", "An Bhoisnia-Heirseagaivéin"), ("gl", "Bosnia e Hercegovina"), ("gn", "Bosnia and Herzegovina"), ("gu", "બોસ\u{acd}નિયા અન\u{ac7} હર\u{acd}ઝ\u{ac7}ગોવીનીઆ"), ("gv", "Bosnia as Herzegovina"), ("ha", "Herzegovina"), ("he", "בוסניה והרצגובינה"), ("hi", "बॉस\u{94d}निया और हर\u{94d}ज\u{93c}\u{947}गोविना"), ("hr", "Bosna i Hercegovina"), ("ht", "Bosnia and Herzegovina"), ("hu", "Bosznia-Hercegovina"), ("hy", "Բոսնիա-Հերցեգովինա"), ("ia", "Bosnia e Herzegovina"), ("id", "Bosnia dan Herzegovina"), ("io", "Bosnia e Herzegovina"), ("is", "Bosnía og Hersegóvína"), ("it", "Bosnia-Erzegovina"), ("iu", "Bosnia and Herzegovina"), ("ja", "ボスニア・ヘルツェゴビナ"), ("ka", "ბოსნია და ჰერცეგოვინა"), ("ki", "Mbocinia na Hecengobina"), ("kk", "Босния және Герцеговина"), ("kl", "Bosnia and Herzegovina"), ("km", "ប\u{17bc}ស\u{17d2}ន\u{17ca}\u{17b8} ន\u{17b7}ង \u{200b}ហ\u{17ba}ហ\u{17d2}ស\u{17ca}េហ\u{17d2}គោវ\u{17b8}ណា"), ("kn", "ಭೊಸ\u{ccd}ನ\u{cbf}ಯಾ ಮತ\u{ccd}ತು ಹ\u{cc6}ರ\u{ccd}ಜ\u{cc6}ಗೋವ\u{cbf}ನಾ"), ("ko", "보스니아 헤르체고비나"), ("ku", "Bosna Hersek"), ("kv", "Босния да Герцеговина"), ("kw", "Bosni–Hercegovina"), ("ky", "Босния жана Герцеговина"), ("lo", "ປະເທດບ\u{ebb}ດສະນ\u{eb5}ແຮກເຊໂກວ\u{eb5}ນ"), ("lt", "Bosnija ir Hercegovina"), ("lv", "Bosnija un Hercegovina"), ("mi", "Pōngia-Herekōmina"), ("mk", "Босна и Херцеговина"), ("ml", "ബോസ\u{d4d}നിയയ\u{d41}ം ഹെര\u{d4d}\u{200d}സഗോവിനയ\u{d41}ം"), ("mn", "Босний ба Нерцеговин"), ("mr", "बोस\u{94d}निया आणी हर\u{94d}झ\u{947}गोव\u{94d}हिना"), ("ms", "Bosnia dan Herzegovina"), ("mt", "Bożnija Ħerżegovina"), ("my", "ဘော\u{1037}စန\u{102e}းယားန\u{103e}င\u{1037}\u{103a} ဟာဇ\u{102e}ဂ\u{102d}\u{102f}ဗ\u{102e}းနားန\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Boteniya me Erdegobina"), ("nb", "Bosnia-Hercegovina"), ("ne", "बोस\u{94d}नीया र हर\u{94d}जगोभिना"), ("nl", "Bosnië en Herzegovina"), ("nn", "Bosnia-Hercegovina"), ("nv", "Bosna dóó Hetsog Bikéyah"), ("oc", "Bòsnia e Ercegovina"), ("or", "ବୋସନ\u{b3f}ଆ ଏବଂ ହର\u{b4d}ଜେଗୋଭ\u{b3f}ନ\u{b3e}"), ("pa", "ਬ\u{a4b}ਸਨੀਆ ਤ\u{a47} ਹਰਜ਼ੀਗ\u{a4b}ਨੀਆ"), ("pi", "बास\u{94d}निया"), ("pl", "Bośnia i Hercegowina"), ("ps", "Bosnia and Herzegovina"), ("pt", "Bósnia e Herzegovina"), ("pt_BR", "Bósnia-Herzegóvina"), ("ro", "Bosnia și Herțegovina"), ("ru", "Босния и Герцеговина"), ("rw", "Bosiniya na Herizegovina"), ("sc", "Bòsnia e Erzegòvina"), ("sd", "Bosnia and Herzegovina"), ("si", "බොස\u{dca}න\u{dd2}ය\u{dcf}ව හ\u{dcf} හර\u{dca}සගෝව\u{dd2}න\u{dd2}ය\u{dcf}ව"), ("sk", "Bosna a Hercegovina"), ("sl", "Bosna in Hercegovina"), ("so", "Boosniya Heersigoviina"), ("sq", "Bosnjë dhe Hercegovinë"), ("sr", "Босна и Херцеговина"), ("sv", "Bosnien-Hercegovina"), ("sw", "Bosnia na Herzegovina"), ("ta", "போஸ\u{bcd}னிய\u{bbe} ஹெர\u{bcd}ஸிகோவின\u{bbe}"), ("te", "బ\u{c4b}స\u{c4d}న\u{c3f}య\u{c3e} మర\u{c3f}యు హర\u{c4d}ఝ\u{c47}గ\u{c4b}వ\u{c4d}హ\u{c3f}న\u{c3e}"), ("tg", "Босния ва Ҳерсеговина"), ("th", "บอสเน\u{e35}ยและเฮอร\u{e4c}เซโกว\u{e35}นา"), ("ti", "ቦስኒያ እና ሄርዞጎቪኒያ"), ("tk", "Bosniýa we Gersogowina"), ("tl", "Bosnia at Herzegovina"), ("tr", "Bosna-Hersek"), ("tt", "Босниа белән Һерзеgовина"), ("ug", "بوسنىيە گېرتسېگوۋىنا"), ("uk", "Боснія і Герцеговина"), ("ur", "بوسنیا و ہرزیگووینا"), ("uz", "Bosniya va Gersegovina"), ("ve", "Bosnia na Herzegovina"), ("vi", "Bô-xni-a và Hẻ-xê-gô-vi-na"), ("wa", "Bosneye"), ("wo", "Bosni Hersegowin"), ("xh", "Bosnia and Herzegovina"), ("yo", "Bósníà àti Hẹrjẹgòfínà"), ("zh_CN", "波斯尼亚和黑塞哥维那"), ("zh_HK", "波斯尼亞和黑塞哥維那"), ("zh_TW", "波士尼亞及赫塞哥維納"), ("zu", "IBhosniya neHerzegovina")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
