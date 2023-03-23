// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/alpha2.rs`)

use crate::{Alpha3, Country};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

/// An enum containing Alpha2 codes for all countries.
///
/// All countries features are enabled by default. You can disable default features and enabled features for all countries that you need.
///
/// # Examples
/// ```
/// use keshvar::{Alpha2, Alpha3, Country, SearchError, SearchedItems};
///
/// assert_eq!(Ok(Alpha2::US), Alpha2::try_from("us")); // not case-sensitive
/// assert_eq!("US", Alpha2::US.to_string().as_str());
///
/// // If enabled all countries features:
/// assert_eq!(
///     Err(SearchError::NotFound {
///         searched_items: SearchedItems::AllCountries
///     }),
///     Alpha2::try_from("xx")
/// );
/// assert_eq!(
///     Err("Could not be found in all countries".to_string()),
///     Alpha2::try_from("xx").map_err(|error| error.to_string())
/// );
///
/// // If enabled some countries features:
/// // For example we enabled supporting just 10 countries and the US
/// // is not one of them:
/// // assert_eq!(
/// //     Err(SearchError::NotFound {
/// //         searched_items: SearchedItems::SupportedCountries(10)
/// //     }),
/// //     Alpha2::try_from("us")
/// // );
/// // assert_eq!(
/// //     Err("Could not be found in 10 supported countries".to_string()),
/// //     Alpha2::try_from("us").map_err(|error| error.to_string())
/// // );
///
/// // Convert to `Alpha3` code:
/// assert_eq!(Alpha3::USA, Alpha2::US.to_alpha3());
/// // Convert to `Country`:
/// let country: Country = Alpha2::US.to_country();
/// // Get subdivisions of country:
/// let subdivisions = Alpha2::US.to_subdivisions();
/// ```
/// We usually need to convert it to [`Country`](crate::Country) and use that object instead.
///
/// # Panics
/// Most methods will panic if you do not enable any country feature!
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum Alpha2 {
    #[cfg(feature = "ad")]
    /// The Principality of Andorra (Europe)
    AD,
    #[cfg(feature = "ae")]
    /// The United Arab Emirates (Asia)
    AE,
    #[cfg(feature = "af")]
    /// The Islamic Republic of Afghanistan (Asia)
    AF,
    #[cfg(feature = "ag")]
    /// Antigua and Barbuda (Americas)
    AG,
    #[cfg(feature = "ai")]
    /// Anguilla (Americas)
    AI,
    #[cfg(feature = "al")]
    /// The Republic of Albania (Europe)
    AL,
    #[cfg(feature = "am")]
    /// The Republic of Armenia (Asia)
    AM,
    #[cfg(feature = "ao")]
    /// The Republic of Angola (Africa)
    AO,
    #[cfg(feature = "aq")]
    /// Antarctica
    AQ,
    #[cfg(feature = "ar")]
    /// The Argentine Republic (Americas)
    AR,
    #[cfg(feature = "as")]
    /// The Territory of American Samoa (Oceania)
    AS,
    #[cfg(feature = "at")]
    /// The Republic of Austria (Europe)
    AT,
    #[cfg(feature = "au")]
    /// The Commonwealth of Australia (Oceania)
    AU,
    #[cfg(feature = "aw")]
    /// Aruba (Americas)
    AW,
    #[cfg(feature = "ax")]
    /// Åland (Europe)
    AX,
    #[cfg(feature = "az")]
    /// The Republic of Azerbaijan (Asia)
    AZ,
    #[cfg(feature = "ba")]
    /// Bosnia and Herzegovina (Europe)
    BA,
    #[cfg(feature = "bb")]
    /// Barbados (Americas)
    BB,
    #[cfg(feature = "bd")]
    /// The People's Republic of Bangladesh (Asia)
    BD,
    #[cfg(feature = "be")]
    /// The Kingdom of Belgium (Europe)
    BE,
    #[cfg(feature = "bf")]
    /// Burkina Faso (Africa)
    BF,
    #[cfg(feature = "bg")]
    /// The Republic of Bulgaria (Europe)
    BG,
    #[cfg(feature = "bh")]
    /// The Kingdom of Bahrain (Asia)
    BH,
    #[cfg(feature = "bi")]
    /// The Republic of Burundi (Africa)
    BI,
    #[cfg(feature = "bj")]
    /// The Republic of Benin (Africa)
    BJ,
    #[cfg(feature = "bl")]
    /// The Collectivity of Saint-Barthélemy (Americas)
    BL,
    #[cfg(feature = "bm")]
    /// Bermuda (Americas)
    BM,
    #[cfg(feature = "bn")]
    /// The Nation of Brunei, the Abode of Peace (Asia)
    BN,
    #[cfg(feature = "bo")]
    /// The Plurinational State of Bolivia (Americas)
    BO,
    #[cfg(feature = "bq")]
    /// Bonaire, Sint Eustatius and Saba (Americas)
    BQ,
    #[cfg(feature = "br")]
    /// The Federative Republic of Brazil (Americas)
    BR,
    #[cfg(feature = "bs")]
    /// The Commonwealth of The Bahamas (Americas)
    BS,
    #[cfg(feature = "bt")]
    /// The Kingdom of Bhutan (Asia)
    BT,
    #[cfg(feature = "bv")]
    /// Bouvet Island
    BV,
    #[cfg(feature = "bw")]
    /// The Republic of Botswana (Africa)
    BW,
    #[cfg(feature = "by")]
    /// The Republic of Belarus (Europe)
    BY,
    #[cfg(feature = "bz")]
    /// Belize (Americas)
    BZ,
    #[cfg(feature = "ca")]
    /// Canada (Americas)
    CA,
    #[cfg(feature = "cc")]
    /// The Territory of Cocos (Keeling) Islands (Oceania)
    CC,
    #[cfg(feature = "cd")]
    /// The Democratic Republic of the Congo (Africa)
    CD,
    #[cfg(feature = "cf")]
    /// The Central African Republic (Africa)
    CF,
    #[cfg(feature = "cg")]
    /// The Republic of the Congo (Africa)
    CG,
    #[cfg(feature = "ch")]
    /// The Swiss Confederation (Europe)
    CH,
    #[cfg(feature = "ci")]
    /// The Republic of Côte d'Ivoire (Africa)
    CI,
    #[cfg(feature = "ck")]
    /// The Cook Islands (Oceania)
    CK,
    #[cfg(feature = "cl")]
    /// The Republic of Chile (Americas)
    CL,
    #[cfg(feature = "cm")]
    /// The Republic of Cameroon (Africa)
    CM,
    #[cfg(feature = "cn")]
    /// The People's Republic of China (Asia)
    CN,
    #[cfg(feature = "co")]
    /// The Republic of Colombia (Americas)
    CO,
    #[cfg(feature = "cr")]
    /// The Republic of Costa Rica (Americas)
    CR,
    #[cfg(feature = "cu")]
    /// The Republic of Cuba (Americas)
    CU,
    #[cfg(feature = "cv")]
    /// The Republic of Cabo Verde (Africa)
    CV,
    #[cfg(feature = "cw")]
    /// The Country of Curaçao (Americas)
    CW,
    #[cfg(feature = "cx")]
    /// The Territory of Christmas Island (Oceania)
    CX,
    #[cfg(feature = "cy")]
    /// The Republic of Cyprus (Asia)
    CY,
    #[cfg(feature = "cz")]
    /// The Czech Republic (Europe)
    CZ,
    #[cfg(feature = "de")]
    /// The Federal Republic of Germany (Europe)
    DE,
    #[cfg(feature = "dj")]
    /// The Republic of Djibouti (Africa)
    DJ,
    #[cfg(feature = "dk")]
    /// The Kingdom of Denmark (Europe)
    DK,
    #[cfg(feature = "dm")]
    /// The Commonwealth of Dominica (Americas)
    DM,
    #[cfg(feature = "do")]
    /// The Dominican Republic (Americas)
    DO,
    #[cfg(feature = "dz")]
    /// The People's Democratic Republic of Algeria (Africa)
    DZ,
    #[cfg(feature = "ec")]
    /// The Republic of Ecuador (Americas)
    EC,
    #[cfg(feature = "ee")]
    /// The Republic of Estonia (Europe)
    EE,
    #[cfg(feature = "eg")]
    /// The Arab Republic of Egypt (Africa)
    EG,
    #[cfg(feature = "eh")]
    /// The Sahrawi Arab Democratic Republic (Africa)
    EH,
    #[cfg(feature = "er")]
    /// The State of Eritrea (Africa)
    ER,
    #[cfg(feature = "es")]
    /// The Kingdom of Spain (Europe)
    ES,
    #[cfg(feature = "et")]
    /// The Federal Democratic Republic of Ethiopia (Africa)
    ET,
    #[cfg(feature = "fi")]
    /// The Republic of Finland (Europe)
    FI,
    #[cfg(feature = "fj")]
    /// The Republic of Fiji (Oceania)
    FJ,
    #[cfg(feature = "fk")]
    /// The Falkland Islands (Americas)
    FK,
    #[cfg(feature = "fm")]
    /// The Federated States of Micronesia (Oceania)
    FM,
    #[cfg(feature = "fo")]
    /// The Faroe Islands (Europe)
    FO,
    #[cfg(feature = "fr")]
    /// The French Republic (Europe)
    FR,
    #[cfg(feature = "ga")]
    /// The Gabonese Republic (Africa)
    GA,
    #[cfg(feature = "gb")]
    /// The United Kingdom of Great Britain and Northern Ireland (Europe)
    GB,
    #[cfg(feature = "gd")]
    /// Grenada (Americas)
    GD,
    #[cfg(feature = "ge")]
    /// Georgia (Asia)
    GE,
    #[cfg(feature = "gf")]
    /// Guyane (Americas)
    GF,
    #[cfg(feature = "gg")]
    /// The Bailiwick of Guernsey (Europe)
    GG,
    #[cfg(feature = "gh")]
    /// The Republic of Ghana (Africa)
    GH,
    #[cfg(feature = "gi")]
    /// Gibraltar (Europe)
    GI,
    #[cfg(feature = "gl")]
    /// Kalaallit Nunaat (Americas)
    GL,
    #[cfg(feature = "gm")]
    /// The Republic of The Gambia (Africa)
    GM,
    #[cfg(feature = "gn")]
    /// The Republic of Guinea (Africa)
    GN,
    #[cfg(feature = "gp")]
    /// Guadeloupe (Americas)
    GP,
    #[cfg(feature = "gq")]
    /// The Republic of Equatorial Guinea (Africa)
    GQ,
    #[cfg(feature = "gr")]
    /// The Hellenic Republic (Europe)
    GR,
    #[cfg(feature = "gs")]
    /// South Georgia and the South Sandwich Islands (Americas)
    GS,
    #[cfg(feature = "gt")]
    /// The Republic of Guatemala (Americas)
    GT,
    #[cfg(feature = "gu")]
    /// The Territory of Guam (Oceania)
    GU,
    #[cfg(feature = "gw")]
    /// The Republic of Guinea-Bissau (Africa)
    GW,
    #[cfg(feature = "gy")]
    /// The Co-operative Republic of Guyana (Americas)
    GY,
    #[cfg(feature = "hk")]
    /// The Hong Kong Special Administrative Region of China (Asia)
    HK,
    #[cfg(feature = "hm")]
    /// The Territory of Heard Island and McDonald Islands
    HM,
    #[cfg(feature = "hn")]
    /// The Republic of Honduras (Americas)
    HN,
    #[cfg(feature = "hr")]
    /// The Republic of Croatia (Europe)
    HR,
    #[cfg(feature = "ht")]
    /// The Republic of Haiti (Americas)
    HT,
    #[cfg(feature = "hu")]
    /// Hungary (Europe)
    HU,
    #[cfg(feature = "id")]
    /// The Republic of Indonesia (Asia)
    ID,
    #[cfg(feature = "ie")]
    /// Ireland (Europe)
    IE,
    #[cfg(feature = "il")]
    /// The State of Israel (Asia)
    IL,
    #[cfg(feature = "im")]
    /// The Isle of Man (Europe)
    IM,
    #[cfg(feature = "in")]
    /// The Republic of India (Asia)
    IN,
    #[cfg(feature = "io")]
    /// The British Indian Ocean Territory (Africa)
    IO,
    #[cfg(feature = "iq")]
    /// The Republic of Iraq (Asia)
    IQ,
    #[cfg(feature = "ir")]
    /// The Islamic Republic of Iran (Asia)
    IR,
    #[cfg(feature = "is")]
    /// Iceland (Europe)
    IS,
    #[cfg(feature = "it")]
    /// The Italian Republic (Europe)
    IT,
    #[cfg(feature = "je")]
    /// The Bailiwick of Jersey (Europe)
    JE,
    #[cfg(feature = "jm")]
    /// Jamaica (Americas)
    JM,
    #[cfg(feature = "jo")]
    /// The Hashemite Kingdom of Jordan (Asia)
    JO,
    #[cfg(feature = "jp")]
    /// Japan (Asia)
    JP,
    #[cfg(feature = "ke")]
    /// The Republic of Kenya (Africa)
    KE,
    #[cfg(feature = "kg")]
    /// The Kyrgyz Republic (Asia)
    KG,
    #[cfg(feature = "kh")]
    /// The Kingdom of Cambodia (Asia)
    KH,
    #[cfg(feature = "ki")]
    /// The Republic of Kiribati (Oceania)
    KI,
    #[cfg(feature = "km")]
    /// The Union of the Comoros (Africa)
    KM,
    #[cfg(feature = "kn")]
    /// Saint Kitts and Nevis (Americas)
    KN,
    #[cfg(feature = "kp")]
    /// The Democratic People's Republic of Korea (Asia)
    KP,
    #[cfg(feature = "kr")]
    /// The Republic of Korea (Asia)
    KR,
    #[cfg(feature = "kw")]
    /// The State of Kuwait (Asia)
    KW,
    #[cfg(feature = "ky")]
    /// The Cayman Islands (Americas)
    KY,
    #[cfg(feature = "kz")]
    /// The Republic of Kazakhstan (Asia)
    KZ,
    #[cfg(feature = "la")]
    /// The Lao People's Democratic Republic (Asia)
    LA,
    #[cfg(feature = "lb")]
    /// The Lebanese Republic (Asia)
    LB,
    #[cfg(feature = "lc")]
    /// Saint Lucia (Americas)
    LC,
    #[cfg(feature = "li")]
    /// The Principality of Liechtenstein (Europe)
    LI,
    #[cfg(feature = "lk")]
    /// The Democratic Socialist Republic of Sri Lanka (Asia)
    LK,
    #[cfg(feature = "lr")]
    /// The Republic of Liberia (Africa)
    LR,
    #[cfg(feature = "ls")]
    /// The Kingdom of Lesotho (Africa)
    LS,
    #[cfg(feature = "lt")]
    /// The Republic of Lithuania (Europe)
    LT,
    #[cfg(feature = "lu")]
    /// The Grand Duchy of Luxembourg (Europe)
    LU,
    #[cfg(feature = "lv")]
    /// The Republic of Latvia (Europe)
    LV,
    #[cfg(feature = "ly")]
    /// The State of Libya (Africa)
    LY,
    #[cfg(feature = "ma")]
    /// The Kingdom of Morocco (Africa)
    MA,
    #[cfg(feature = "mc")]
    /// The Principality of Monaco (Europe)
    MC,
    #[cfg(feature = "md")]
    /// The Republic of Moldova (Europe)
    MD,
    #[cfg(feature = "me")]
    /// Montenegro (Europe)
    ME,
    #[cfg(feature = "mf")]
    /// The Collectivity of Saint-Martin (Americas)
    MF,
    #[cfg(feature = "mg")]
    /// The Republic of Madagascar (Africa)
    MG,
    #[cfg(feature = "mh")]
    /// The Republic of the Marshall Islands (Oceania)
    MH,
    #[cfg(feature = "mk")]
    /// The Republic of North Macedonia (Europe)
    MK,
    #[cfg(feature = "ml")]
    /// The Republic of Mali (Africa)
    ML,
    #[cfg(feature = "mm")]
    /// The Republic of the Union of Myanmar (Asia)
    MM,
    #[cfg(feature = "mn")]
    /// Mongolia (Asia)
    MN,
    #[cfg(feature = "mo")]
    /// The Macao Special Administrative Region of China (Asia)
    MO,
    #[cfg(feature = "mp")]
    /// The Commonwealth of the Northern Mariana Islands (Oceania)
    MP,
    #[cfg(feature = "mq")]
    /// Martinique (Americas)
    MQ,
    #[cfg(feature = "mr")]
    /// The Islamic Republic of Mauritania (Africa)
    MR,
    #[cfg(feature = "ms")]
    /// Montserrat (Americas)
    MS,
    #[cfg(feature = "mt")]
    /// The Republic of Malta (Europe)
    MT,
    #[cfg(feature = "mu")]
    /// The Republic of Mauritius (Africa)
    MU,
    #[cfg(feature = "mv")]
    /// The Republic of Maldives (Asia)
    MV,
    #[cfg(feature = "mw")]
    /// The Republic of Malawi (Africa)
    MW,
    #[cfg(feature = "mx")]
    /// The United Mexican States (Americas)
    MX,
    #[cfg(feature = "my")]
    /// Malaysia (Asia)
    MY,
    #[cfg(feature = "mz")]
    /// The Republic of Mozambique (Africa)
    MZ,
    #[cfg(feature = "na")]
    /// The Republic of Namibia (Africa)
    NA,
    #[cfg(feature = "nc")]
    /// New Caledonia (Oceania)
    NC,
    #[cfg(feature = "ne")]
    /// The Republic of the Niger (Africa)
    NE,
    #[cfg(feature = "nf")]
    /// The Territory of Norfolk Island (Oceania)
    NF,
    #[cfg(feature = "ng")]
    /// The Federal Republic of Nigeria (Africa)
    NG,
    #[cfg(feature = "ni")]
    /// The Republic of Nicaragua (Americas)
    NI,
    #[cfg(feature = "nl")]
    /// The Kingdom of the Netherlands (Europe)
    NL,
    #[cfg(feature = "no")]
    /// The Kingdom of Norway (Europe)
    NO,
    #[cfg(feature = "np")]
    /// The Federal Democratic Republic of Nepal (Asia)
    NP,
    #[cfg(feature = "nr")]
    /// The Republic of Nauru (Oceania)
    NR,
    #[cfg(feature = "nu")]
    /// Niue (Oceania)
    NU,
    #[cfg(feature = "nz")]
    /// New Zealand (Oceania)
    NZ,
    #[cfg(feature = "om")]
    /// The Sultanate of Oman (Asia)
    OM,
    #[cfg(feature = "pa")]
    /// The Republic of Panamá (Americas)
    PA,
    #[cfg(feature = "pe")]
    /// The Republic of Perú (Americas)
    PE,
    #[cfg(feature = "pf")]
    /// French Polynesia (Oceania)
    PF,
    #[cfg(feature = "pg")]
    /// The Independent State of Papua New Guinea (Oceania)
    PG,
    #[cfg(feature = "ph")]
    /// The Republic of the Philippines (Asia)
    PH,
    #[cfg(feature = "pk")]
    /// The Islamic Republic of Pakistan (Asia)
    PK,
    #[cfg(feature = "pl")]
    /// The Republic of Poland (Europe)
    PL,
    #[cfg(feature = "pm")]
    /// The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    PM,
    #[cfg(feature = "pn")]
    /// The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    PN,
    #[cfg(feature = "pr")]
    /// The Commonwealth of Puerto Rico (Americas)
    PR,
    #[cfg(feature = "ps")]
    /// The State of Palestine (Asia)
    PS,
    #[cfg(feature = "pt")]
    /// The Portuguese Republic (Europe)
    PT,
    #[cfg(feature = "pw")]
    /// The Republic of Palau (Oceania)
    PW,
    #[cfg(feature = "py")]
    /// The Republic of Paraguay (Americas)
    PY,
    #[cfg(feature = "qa")]
    /// The State of Qatar (Asia)
    QA,
    #[cfg(feature = "re")]
    /// Réunion (Africa)
    RE,
    #[cfg(feature = "ro")]
    /// Romania (Europe)
    RO,
    #[cfg(feature = "rs")]
    /// The Republic of Serbia (Europe)
    RS,
    #[cfg(feature = "ru")]
    /// The Russian Federation (Europe)
    RU,
    #[cfg(feature = "rw")]
    /// The Republic of Rwanda (Africa)
    RW,
    #[cfg(feature = "sa")]
    /// The Kingdom of Saudi Arabia (Asia)
    SA,
    #[cfg(feature = "sb")]
    /// The Solomon Islands (Oceania)
    SB,
    #[cfg(feature = "sc")]
    /// The Republic of Seychelles (Africa)
    SC,
    #[cfg(feature = "sd")]
    /// The Republic of the Sudan (Africa)
    SD,
    #[cfg(feature = "se")]
    /// The Kingdom of Sweden (Europe)
    SE,
    #[cfg(feature = "sg")]
    /// The Republic of Singapore (Asia)
    SG,
    #[cfg(feature = "sh")]
    /// Saint Helena, Ascension and Tristan da Cunha (Africa)
    SH,
    #[cfg(feature = "si")]
    /// The Republic of Slovenia (Europe)
    SI,
    #[cfg(feature = "sj")]
    /// Svalbard and Jan Mayen (Europe)
    SJ,
    #[cfg(feature = "sk")]
    /// The Slovak Republic (Europe)
    SK,
    #[cfg(feature = "sl")]
    /// The Republic of Sierra Leone (Africa)
    SL,
    #[cfg(feature = "sm")]
    /// The Republic of San Marino (Europe)
    SM,
    #[cfg(feature = "sn")]
    /// The Republic of Senegal (Africa)
    SN,
    #[cfg(feature = "so")]
    /// The Federal Republic of Somalia (Africa)
    SO,
    #[cfg(feature = "sr")]
    /// The Republic of Suriname (Americas)
    SR,
    #[cfg(feature = "ss")]
    /// The Republic of South Sudan (Africa)
    SS,
    #[cfg(feature = "st")]
    /// The Democratic Republic of São Tomé and Príncipe (Africa)
    ST,
    #[cfg(feature = "sv")]
    /// The Republic of El Salvador (Americas)
    SV,
    #[cfg(feature = "sx")]
    /// Sint Maarten (Americas)
    SX,
    #[cfg(feature = "sy")]
    /// The Syrian Arab Republic (Asia)
    SY,
    #[cfg(feature = "sz")]
    /// The Kingdom of Eswatini (Africa)
    SZ,
    #[cfg(feature = "tc")]
    /// The Turks and Caicos Islands (Americas)
    TC,
    #[cfg(feature = "td")]
    /// The Republic of Chad (Africa)
    TD,
    #[cfg(feature = "tf")]
    /// The French Southern and Antarctic Lands (Africa)
    TF,
    #[cfg(feature = "tg")]
    /// The Togolese Republic (Africa)
    TG,
    #[cfg(feature = "th")]
    /// The Kingdom of Thailand (Asia)
    TH,
    #[cfg(feature = "tj")]
    /// The Republic of Tajikistan (Asia)
    TJ,
    #[cfg(feature = "tk")]
    /// Tokelau (Oceania)
    TK,
    #[cfg(feature = "tl")]
    /// The Democratic Republic of Timor-Leste (Asia)
    TL,
    #[cfg(feature = "tm")]
    /// Turkmenistan (Asia)
    TM,
    #[cfg(feature = "tn")]
    /// The Republic of Tunisia (Africa)
    TN,
    #[cfg(feature = "to")]
    /// The Kingdom of Tonga (Oceania)
    TO,
    #[cfg(feature = "tr")]
    /// The Republic of Turkey (Asia)
    TR,
    #[cfg(feature = "tt")]
    /// The Republic of Trinidad and Tobago (Americas)
    TT,
    #[cfg(feature = "tv")]
    /// Tuvalu (Oceania)
    TV,
    #[cfg(feature = "tw")]
    /// The Republic of China (Asia)
    TW,
    #[cfg(feature = "tz")]
    /// The United Republic of Tanzania (Africa)
    TZ,
    #[cfg(feature = "ua")]
    /// Ukraine (Europe)
    UA,
    #[cfg(feature = "ug")]
    /// The Republic of Uganda (Africa)
    UG,
    #[cfg(feature = "um")]
    /// United States Minor Outlying Islands (Americas)
    UM,
    #[cfg(feature = "us")]
    /// The United States of America (Americas)
    US,
    #[cfg(feature = "uy")]
    /// The Oriental Republic of Uruguay (Americas)
    UY,
    #[cfg(feature = "uz")]
    /// The Republic of Uzbekistan (Asia)
    UZ,
    #[cfg(feature = "va")]
    /// The Holy See (Europe)
    VA,
    #[cfg(feature = "vc")]
    /// Saint Vincent and the Grenadines (Americas)
    VC,
    #[cfg(feature = "ve")]
    /// The Bolivarian Republic of Venezuela (Americas)
    VE,
    #[cfg(feature = "vg")]
    /// The Virgin Islands (Americas)
    VG,
    #[cfg(feature = "vi")]
    /// The Virgin Islands of the United States (Americas)
    VI,
    #[cfg(feature = "vn")]
    /// The Socialist Republic of Viet Nam (Asia)
    VN,
    #[cfg(feature = "vu")]
    /// The Republic of Vanuatu (Oceania)
    VU,
    #[cfg(feature = "wf")]
    /// The Territory of the Wallis and Futuna Islands (Oceania)
    WF,
    #[cfg(feature = "ws")]
    /// The Independent State of Samoa (Oceania)
    WS,
    #[cfg(feature = "ye")]
    /// The Republic of Yemen (Asia)
    YE,
    #[cfg(feature = "yt")]
    /// The Department of Mayotte (Africa)
    YT,
    #[cfg(feature = "za")]
    /// The Republic of South Africa (Africa)
    ZA,
    #[cfg(feature = "zm")]
    /// The Republic of Zambia (Africa)
    ZM,
    #[cfg(feature = "zw")]
    /// The Republic of Zimbabwe (Africa)
    ZW,
}

impl Alpha2 {
    pub fn to_country(&self) -> Country {
        Country::from(*self)
    }
}

impl From<Alpha3> for Alpha2 {
    fn from(alpha3: Alpha3) -> Self {
        alpha3.to_alpha2()
    }
}
#[cfg(any(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "ai",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "aq",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
    feature = "ax",
    feature = "az",
    feature = "ba",
    feature = "bb",
    feature = "bd",
    feature = "be",
    feature = "bf",
    feature = "bg",
    feature = "bh",
    feature = "bi",
    feature = "bj",
    feature = "bl",
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "bq",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bv",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
    feature = "cc",
    feature = "cd",
    feature = "cf",
    feature = "cg",
    feature = "ch",
    feature = "ci",
    feature = "ck",
    feature = "cl",
    feature = "cm",
    feature = "cn",
    feature = "co",
    feature = "cr",
    feature = "cu",
    feature = "cv",
    feature = "cw",
    feature = "cx",
    feature = "cy",
    feature = "cz",
    feature = "de",
    feature = "dj",
    feature = "dk",
    feature = "dm",
    feature = "do",
    feature = "dz",
    feature = "ec",
    feature = "ee",
    feature = "eg",
    feature = "eh",
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fk",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gf",
    feature = "gg",
    feature = "gh",
    feature = "gi",
    feature = "gl",
    feature = "gm",
    feature = "gn",
    feature = "gp",
    feature = "gq",
    feature = "gr",
    feature = "gs",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hm",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "im",
    feature = "in",
    feature = "io",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
    feature = "je",
    feature = "jm",
    feature = "jo",
    feature = "jp",
    feature = "ke",
    feature = "kg",
    feature = "kh",
    feature = "ki",
    feature = "km",
    feature = "kn",
    feature = "kp",
    feature = "kr",
    feature = "kw",
    feature = "ky",
    feature = "kz",
    feature = "la",
    feature = "lb",
    feature = "lc",
    feature = "li",
    feature = "lk",
    feature = "lr",
    feature = "ls",
    feature = "lt",
    feature = "lu",
    feature = "lv",
    feature = "ly",
    feature = "ma",
    feature = "mc",
    feature = "md",
    feature = "me",
    feature = "mf",
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mo",
    feature = "mp",
    feature = "mq",
    feature = "mr",
    feature = "ms",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "nc",
    feature = "ne",
    feature = "nf",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nu",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pf",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pm",
    feature = "pn",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
    feature = "re",
    feature = "ro",
    feature = "rs",
    feature = "ru",
    feature = "rw",
    feature = "sa",
    feature = "sb",
    feature = "sc",
    feature = "sd",
    feature = "se",
    feature = "sg",
    feature = "sh",
    feature = "si",
    feature = "sj",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "ss",
    feature = "st",
    feature = "sv",
    feature = "sx",
    feature = "sy",
    feature = "sz",
    feature = "tc",
    feature = "td",
    feature = "tf",
    feature = "tg",
    feature = "th",
    feature = "tj",
    feature = "tk",
    feature = "tl",
    feature = "tm",
    feature = "tn",
    feature = "to",
    feature = "tr",
    feature = "tt",
    feature = "tv",
    feature = "tw",
    feature = "tz",
    feature = "ua",
    feature = "ug",
    feature = "um",
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "va",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "wf",
    feature = "ws",
    feature = "ye",
    feature = "yt",
    feature = "za",
    feature = "zm",
    feature = "zw",
))]
mod impls {
    use super::{Alpha2, Alpha3, Country};
    use crate::{SearchError, SearchedItems};

    impl TryFrom<&str> for Alpha2 {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 2 {
                return Err(SearchError::BadInput {
                    expected: "a string with two characters",
                });
            }
            match value.to_uppercase().as_str() {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                "AD" => Ok(Self::AD),
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                "AE" => Ok(Self::AE),
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                "AF" => Ok(Self::AF),
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                "AG" => Ok(Self::AG),
                #[cfg(feature = "ai")] // Anguilla (Americas)
                "AI" => Ok(Self::AI),
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                "AL" => Ok(Self::AL),
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                "AM" => Ok(Self::AM),
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                "AO" => Ok(Self::AO),
                #[cfg(feature = "aq")] // Antarctica
                "AQ" => Ok(Self::AQ),
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                "AR" => Ok(Self::AR),
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                "AS" => Ok(Self::AS),
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                "AT" => Ok(Self::AT),
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                "AU" => Ok(Self::AU),
                #[cfg(feature = "aw")] // Aruba (Americas)
                "AW" => Ok(Self::AW),
                #[cfg(feature = "ax")] // Åland (Europe)
                "AX" => Ok(Self::AX),
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                "AZ" => Ok(Self::AZ),
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                "BA" => Ok(Self::BA),
                #[cfg(feature = "bb")] // Barbados (Americas)
                "BB" => Ok(Self::BB),
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                "BD" => Ok(Self::BD),
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                "BE" => Ok(Self::BE),
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                "BF" => Ok(Self::BF),
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                "BG" => Ok(Self::BG),
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                "BH" => Ok(Self::BH),
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                "BI" => Ok(Self::BI),
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                "BJ" => Ok(Self::BJ),
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                "BL" => Ok(Self::BL),
                #[cfg(feature = "bm")] // Bermuda (Americas)
                "BM" => Ok(Self::BM),
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                "BN" => Ok(Self::BN),
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                "BO" => Ok(Self::BO),
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                "BQ" => Ok(Self::BQ),
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                "BR" => Ok(Self::BR),
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                "BS" => Ok(Self::BS),
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                "BT" => Ok(Self::BT),
                #[cfg(feature = "bv")] // Bouvet Island
                "BV" => Ok(Self::BV),
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                "BW" => Ok(Self::BW),
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                "BY" => Ok(Self::BY),
                #[cfg(feature = "bz")] // Belize (Americas)
                "BZ" => Ok(Self::BZ),
                #[cfg(feature = "ca")] // Canada (Americas)
                "CA" => Ok(Self::CA),
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                "CC" => Ok(Self::CC),
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                "CD" => Ok(Self::CD),
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                "CF" => Ok(Self::CF),
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                "CG" => Ok(Self::CG),
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                "CH" => Ok(Self::CH),
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                "CI" => Ok(Self::CI),
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                "CK" => Ok(Self::CK),
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                "CL" => Ok(Self::CL),
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                "CM" => Ok(Self::CM),
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                "CN" => Ok(Self::CN),
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                "CO" => Ok(Self::CO),
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                "CR" => Ok(Self::CR),
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                "CU" => Ok(Self::CU),
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                "CV" => Ok(Self::CV),
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                "CW" => Ok(Self::CW),
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                "CX" => Ok(Self::CX),
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                "CY" => Ok(Self::CY),
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                "CZ" => Ok(Self::CZ),
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                "DE" => Ok(Self::DE),
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                "DJ" => Ok(Self::DJ),
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                "DK" => Ok(Self::DK),
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                "DM" => Ok(Self::DM),
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                "DO" => Ok(Self::DO),
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                "DZ" => Ok(Self::DZ),
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                "EC" => Ok(Self::EC),
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                "EE" => Ok(Self::EE),
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                "EG" => Ok(Self::EG),
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                "EH" => Ok(Self::EH),
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                "ER" => Ok(Self::ER),
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                "ES" => Ok(Self::ES),
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                "ET" => Ok(Self::ET),
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                "FI" => Ok(Self::FI),
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                "FJ" => Ok(Self::FJ),
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                "FK" => Ok(Self::FK),
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                "FM" => Ok(Self::FM),
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                "FO" => Ok(Self::FO),
                #[cfg(feature = "fr")] // The French Republic (Europe)
                "FR" => Ok(Self::FR),
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                "GA" => Ok(Self::GA),
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                "GB" => Ok(Self::GB),
                #[cfg(feature = "gd")] // Grenada (Americas)
                "GD" => Ok(Self::GD),
                #[cfg(feature = "ge")] // Georgia (Asia)
                "GE" => Ok(Self::GE),
                #[cfg(feature = "gf")] // Guyane (Americas)
                "GF" => Ok(Self::GF),
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                "GG" => Ok(Self::GG),
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                "GH" => Ok(Self::GH),
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                "GI" => Ok(Self::GI),
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                "GL" => Ok(Self::GL),
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                "GM" => Ok(Self::GM),
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                "GN" => Ok(Self::GN),
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                "GP" => Ok(Self::GP),
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                "GQ" => Ok(Self::GQ),
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                "GR" => Ok(Self::GR),
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                "GS" => Ok(Self::GS),
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                "GT" => Ok(Self::GT),
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                "GU" => Ok(Self::GU),
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                "GW" => Ok(Self::GW),
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                "GY" => Ok(Self::GY),
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                "HK" => Ok(Self::HK),
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                "HM" => Ok(Self::HM),
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                "HN" => Ok(Self::HN),
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                "HR" => Ok(Self::HR),
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                "HT" => Ok(Self::HT),
                #[cfg(feature = "hu")] // Hungary (Europe)
                "HU" => Ok(Self::HU),
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                "ID" => Ok(Self::ID),
                #[cfg(feature = "ie")] // Ireland (Europe)
                "IE" => Ok(Self::IE),
                #[cfg(feature = "il")] // The State of Israel (Asia)
                "IL" => Ok(Self::IL),
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                "IM" => Ok(Self::IM),
                #[cfg(feature = "in")] // The Republic of India (Asia)
                "IN" => Ok(Self::IN),
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                "IO" => Ok(Self::IO),
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                "IQ" => Ok(Self::IQ),
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                "IR" => Ok(Self::IR),
                #[cfg(feature = "is")] // Iceland (Europe)
                "IS" => Ok(Self::IS),
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                "IT" => Ok(Self::IT),
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                "JE" => Ok(Self::JE),
                #[cfg(feature = "jm")] // Jamaica (Americas)
                "JM" => Ok(Self::JM),
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                "JO" => Ok(Self::JO),
                #[cfg(feature = "jp")] // Japan (Asia)
                "JP" => Ok(Self::JP),
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                "KE" => Ok(Self::KE),
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                "KG" => Ok(Self::KG),
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                "KH" => Ok(Self::KH),
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                "KI" => Ok(Self::KI),
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                "KM" => Ok(Self::KM),
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                "KN" => Ok(Self::KN),
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                "KP" => Ok(Self::KP),
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                "KR" => Ok(Self::KR),
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                "KW" => Ok(Self::KW),
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                "KY" => Ok(Self::KY),
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                "KZ" => Ok(Self::KZ),
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                "LA" => Ok(Self::LA),
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                "LB" => Ok(Self::LB),
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                "LC" => Ok(Self::LC),
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                "LI" => Ok(Self::LI),
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                "LK" => Ok(Self::LK),
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                "LR" => Ok(Self::LR),
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                "LS" => Ok(Self::LS),
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                "LT" => Ok(Self::LT),
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                "LU" => Ok(Self::LU),
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                "LV" => Ok(Self::LV),
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                "LY" => Ok(Self::LY),
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                "MA" => Ok(Self::MA),
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                "MC" => Ok(Self::MC),
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                "MD" => Ok(Self::MD),
                #[cfg(feature = "me")] // Montenegro (Europe)
                "ME" => Ok(Self::ME),
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                "MF" => Ok(Self::MF),
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                "MG" => Ok(Self::MG),
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                "MH" => Ok(Self::MH),
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                "MK" => Ok(Self::MK),
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                "ML" => Ok(Self::ML),
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                "MM" => Ok(Self::MM),
                #[cfg(feature = "mn")] // Mongolia (Asia)
                "MN" => Ok(Self::MN),
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                "MO" => Ok(Self::MO),
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                "MP" => Ok(Self::MP),
                #[cfg(feature = "mq")] // Martinique (Americas)
                "MQ" => Ok(Self::MQ),
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                "MR" => Ok(Self::MR),
                #[cfg(feature = "ms")] // Montserrat (Americas)
                "MS" => Ok(Self::MS),
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                "MT" => Ok(Self::MT),
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                "MU" => Ok(Self::MU),
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                "MV" => Ok(Self::MV),
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                "MW" => Ok(Self::MW),
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                "MX" => Ok(Self::MX),
                #[cfg(feature = "my")] // Malaysia (Asia)
                "MY" => Ok(Self::MY),
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                "MZ" => Ok(Self::MZ),
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                "NA" => Ok(Self::NA),
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                "NC" => Ok(Self::NC),
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                "NE" => Ok(Self::NE),
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                "NF" => Ok(Self::NF),
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                "NG" => Ok(Self::NG),
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                "NI" => Ok(Self::NI),
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                "NL" => Ok(Self::NL),
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                "NO" => Ok(Self::NO),
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                "NP" => Ok(Self::NP),
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                "NR" => Ok(Self::NR),
                #[cfg(feature = "nu")] // Niue (Oceania)
                "NU" => Ok(Self::NU),
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                "NZ" => Ok(Self::NZ),
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                "OM" => Ok(Self::OM),
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                "PA" => Ok(Self::PA),
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                "PE" => Ok(Self::PE),
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                "PF" => Ok(Self::PF),
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                "PG" => Ok(Self::PG),
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                "PH" => Ok(Self::PH),
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                "PK" => Ok(Self::PK),
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                "PL" => Ok(Self::PL),
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                "PM" => Ok(Self::PM),
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                "PN" => Ok(Self::PN),
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                "PR" => Ok(Self::PR),
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                "PS" => Ok(Self::PS),
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                "PT" => Ok(Self::PT),
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                "PW" => Ok(Self::PW),
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                "PY" => Ok(Self::PY),
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                "QA" => Ok(Self::QA),
                #[cfg(feature = "re")] // Réunion (Africa)
                "RE" => Ok(Self::RE),
                #[cfg(feature = "ro")] // Romania (Europe)
                "RO" => Ok(Self::RO),
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                "RS" => Ok(Self::RS),
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                "RU" => Ok(Self::RU),
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                "RW" => Ok(Self::RW),
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                "SA" => Ok(Self::SA),
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                "SB" => Ok(Self::SB),
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                "SC" => Ok(Self::SC),
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                "SD" => Ok(Self::SD),
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                "SE" => Ok(Self::SE),
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                "SG" => Ok(Self::SG),
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                "SH" => Ok(Self::SH),
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                "SI" => Ok(Self::SI),
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                "SJ" => Ok(Self::SJ),
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                "SK" => Ok(Self::SK),
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                "SL" => Ok(Self::SL),
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                "SM" => Ok(Self::SM),
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                "SN" => Ok(Self::SN),
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                "SO" => Ok(Self::SO),
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                "SR" => Ok(Self::SR),
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                "SS" => Ok(Self::SS),
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                "ST" => Ok(Self::ST),
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                "SV" => Ok(Self::SV),
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                "SX" => Ok(Self::SX),
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                "SY" => Ok(Self::SY),
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                "SZ" => Ok(Self::SZ),
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                "TC" => Ok(Self::TC),
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                "TD" => Ok(Self::TD),
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                "TF" => Ok(Self::TF),
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                "TG" => Ok(Self::TG),
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                "TH" => Ok(Self::TH),
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                "TJ" => Ok(Self::TJ),
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                "TK" => Ok(Self::TK),
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                "TL" => Ok(Self::TL),
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                "TM" => Ok(Self::TM),
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                "TN" => Ok(Self::TN),
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                "TO" => Ok(Self::TO),
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                "TR" => Ok(Self::TR),
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                "TT" => Ok(Self::TT),
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                "TV" => Ok(Self::TV),
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                "TW" => Ok(Self::TW),
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                "TZ" => Ok(Self::TZ),
                #[cfg(feature = "ua")] // Ukraine (Europe)
                "UA" => Ok(Self::UA),
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                "UG" => Ok(Self::UG),
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                "UM" => Ok(Self::UM),
                #[cfg(feature = "us")] // The United States of America (Americas)
                "US" => Ok(Self::US),
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                "UY" => Ok(Self::UY),
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                "UZ" => Ok(Self::UZ),
                #[cfg(feature = "va")] // The Holy See (Europe)
                "VA" => Ok(Self::VA),
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                "VC" => Ok(Self::VC),
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                "VE" => Ok(Self::VE),
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                "VG" => Ok(Self::VG),
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                "VI" => Ok(Self::VI),
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                "VN" => Ok(Self::VN),
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                "VU" => Ok(Self::VU),
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                "WF" => Ok(Self::WF),
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                "WS" => Ok(Self::WS),
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                "YE" => Ok(Self::YE),
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                "YT" => Ok(Self::YT),
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                "ZA" => Ok(Self::ZA),
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                "ZM" => Ok(Self::ZM),
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                "ZW" => Ok(Self::ZW),
                #[cfg(all(
                    feature = "ad",
                    feature = "ae",
                    feature = "af",
                    feature = "ag",
                    feature = "ai",
                    feature = "al",
                    feature = "am",
                    feature = "ao",
                    feature = "aq",
                    feature = "ar",
                    feature = "as",
                    feature = "at",
                    feature = "au",
                    feature = "aw",
                    feature = "ax",
                    feature = "az",
                    feature = "ba",
                    feature = "bb",
                    feature = "bd",
                    feature = "be",
                    feature = "bf",
                    feature = "bg",
                    feature = "bh",
                    feature = "bi",
                    feature = "bj",
                    feature = "bl",
                    feature = "bm",
                    feature = "bn",
                    feature = "bo",
                    feature = "bq",
                    feature = "br",
                    feature = "bs",
                    feature = "bt",
                    feature = "bv",
                    feature = "bw",
                    feature = "by",
                    feature = "bz",
                    feature = "ca",
                    feature = "cc",
                    feature = "cd",
                    feature = "cf",
                    feature = "cg",
                    feature = "ch",
                    feature = "ci",
                    feature = "ck",
                    feature = "cl",
                    feature = "cm",
                    feature = "cn",
                    feature = "co",
                    feature = "cr",
                    feature = "cu",
                    feature = "cv",
                    feature = "cw",
                    feature = "cx",
                    feature = "cy",
                    feature = "cz",
                    feature = "de",
                    feature = "dj",
                    feature = "dk",
                    feature = "dm",
                    feature = "do",
                    feature = "dz",
                    feature = "ec",
                    feature = "ee",
                    feature = "eg",
                    feature = "eh",
                    feature = "er",
                    feature = "es",
                    feature = "et",
                    feature = "fi",
                    feature = "fj",
                    feature = "fk",
                    feature = "fm",
                    feature = "fo",
                    feature = "fr",
                    feature = "ga",
                    feature = "gb",
                    feature = "gd",
                    feature = "ge",
                    feature = "gf",
                    feature = "gg",
                    feature = "gh",
                    feature = "gi",
                    feature = "gl",
                    feature = "gm",
                    feature = "gn",
                    feature = "gp",
                    feature = "gq",
                    feature = "gr",
                    feature = "gs",
                    feature = "gt",
                    feature = "gu",
                    feature = "gw",
                    feature = "gy",
                    feature = "hk",
                    feature = "hm",
                    feature = "hn",
                    feature = "hr",
                    feature = "ht",
                    feature = "hu",
                    feature = "id",
                    feature = "ie",
                    feature = "il",
                    feature = "im",
                    feature = "in",
                    feature = "io",
                    feature = "iq",
                    feature = "ir",
                    feature = "is",
                    feature = "it",
                    feature = "je",
                    feature = "jm",
                    feature = "jo",
                    feature = "jp",
                    feature = "ke",
                    feature = "kg",
                    feature = "kh",
                    feature = "ki",
                    feature = "km",
                    feature = "kn",
                    feature = "kp",
                    feature = "kr",
                    feature = "kw",
                    feature = "ky",
                    feature = "kz",
                    feature = "la",
                    feature = "lb",
                    feature = "lc",
                    feature = "li",
                    feature = "lk",
                    feature = "lr",
                    feature = "ls",
                    feature = "lt",
                    feature = "lu",
                    feature = "lv",
                    feature = "ly",
                    feature = "ma",
                    feature = "mc",
                    feature = "md",
                    feature = "me",
                    feature = "mf",
                    feature = "mg",
                    feature = "mh",
                    feature = "mk",
                    feature = "ml",
                    feature = "mm",
                    feature = "mn",
                    feature = "mo",
                    feature = "mp",
                    feature = "mq",
                    feature = "mr",
                    feature = "ms",
                    feature = "mt",
                    feature = "mu",
                    feature = "mv",
                    feature = "mw",
                    feature = "mx",
                    feature = "my",
                    feature = "mz",
                    feature = "na",
                    feature = "nc",
                    feature = "ne",
                    feature = "nf",
                    feature = "ng",
                    feature = "ni",
                    feature = "nl",
                    feature = "no",
                    feature = "np",
                    feature = "nr",
                    feature = "nu",
                    feature = "nz",
                    feature = "om",
                    feature = "pa",
                    feature = "pe",
                    feature = "pf",
                    feature = "pg",
                    feature = "ph",
                    feature = "pk",
                    feature = "pl",
                    feature = "pm",
                    feature = "pn",
                    feature = "pr",
                    feature = "ps",
                    feature = "pt",
                    feature = "pw",
                    feature = "py",
                    feature = "qa",
                    feature = "re",
                    feature = "ro",
                    feature = "rs",
                    feature = "ru",
                    feature = "rw",
                    feature = "sa",
                    feature = "sb",
                    feature = "sc",
                    feature = "sd",
                    feature = "se",
                    feature = "sg",
                    feature = "sh",
                    feature = "si",
                    feature = "sj",
                    feature = "sk",
                    feature = "sl",
                    feature = "sm",
                    feature = "sn",
                    feature = "so",
                    feature = "sr",
                    feature = "ss",
                    feature = "st",
                    feature = "sv",
                    feature = "sx",
                    feature = "sy",
                    feature = "sz",
                    feature = "tc",
                    feature = "td",
                    feature = "tf",
                    feature = "tg",
                    feature = "th",
                    feature = "tj",
                    feature = "tk",
                    feature = "tl",
                    feature = "tm",
                    feature = "tn",
                    feature = "to",
                    feature = "tr",
                    feature = "tt",
                    feature = "tv",
                    feature = "tw",
                    feature = "tz",
                    feature = "ua",
                    feature = "ug",
                    feature = "um",
                    feature = "us",
                    feature = "uy",
                    feature = "uz",
                    feature = "va",
                    feature = "vc",
                    feature = "ve",
                    feature = "vg",
                    feature = "vi",
                    feature = "vn",
                    feature = "vu",
                    feature = "wf",
                    feature = "ws",
                    feature = "ye",
                    feature = "yt",
                    feature = "za",
                    feature = "zm",
                    feature = "zw",
                ))]
                _ => Err(SearchError::NotFound {
                    searched_items: SearchedItems::AllCountries,
                }),
                #[allow(unreachable_patterns)]
                _ => Err(SearchError::NotFound {
                    searched_items: SearchedItems::SupportedCountries(
                        *crate::consts::SUPPORTED_COUNTRIES_COUNT,
                    ),
                }),
            }
        }
    }

    impl ToString for Alpha2 {
        fn to_string(&self) -> String {
            match self {
                #[cfg(feature = "ad")]
                Alpha2::AD => "AD",
                #[cfg(feature = "ae")]
                Alpha2::AE => "AE",
                #[cfg(feature = "af")]
                Alpha2::AF => "AF",
                #[cfg(feature = "ag")]
                Alpha2::AG => "AG",
                #[cfg(feature = "ai")]
                Alpha2::AI => "AI",
                #[cfg(feature = "al")]
                Alpha2::AL => "AL",
                #[cfg(feature = "am")]
                Alpha2::AM => "AM",
                #[cfg(feature = "ao")]
                Alpha2::AO => "AO",
                #[cfg(feature = "aq")]
                Alpha2::AQ => "AQ",
                #[cfg(feature = "ar")]
                Alpha2::AR => "AR",
                #[cfg(feature = "as")]
                Alpha2::AS => "AS",
                #[cfg(feature = "at")]
                Alpha2::AT => "AT",
                #[cfg(feature = "au")]
                Alpha2::AU => "AU",
                #[cfg(feature = "aw")]
                Alpha2::AW => "AW",
                #[cfg(feature = "ax")]
                Alpha2::AX => "AX",
                #[cfg(feature = "az")]
                Alpha2::AZ => "AZ",
                #[cfg(feature = "ba")]
                Alpha2::BA => "BA",
                #[cfg(feature = "bb")]
                Alpha2::BB => "BB",
                #[cfg(feature = "bd")]
                Alpha2::BD => "BD",
                #[cfg(feature = "be")]
                Alpha2::BE => "BE",
                #[cfg(feature = "bf")]
                Alpha2::BF => "BF",
                #[cfg(feature = "bg")]
                Alpha2::BG => "BG",
                #[cfg(feature = "bh")]
                Alpha2::BH => "BH",
                #[cfg(feature = "bi")]
                Alpha2::BI => "BI",
                #[cfg(feature = "bj")]
                Alpha2::BJ => "BJ",
                #[cfg(feature = "bl")]
                Alpha2::BL => "BL",
                #[cfg(feature = "bm")]
                Alpha2::BM => "BM",
                #[cfg(feature = "bn")]
                Alpha2::BN => "BN",
                #[cfg(feature = "bo")]
                Alpha2::BO => "BO",
                #[cfg(feature = "bq")]
                Alpha2::BQ => "BQ",
                #[cfg(feature = "br")]
                Alpha2::BR => "BR",
                #[cfg(feature = "bs")]
                Alpha2::BS => "BS",
                #[cfg(feature = "bt")]
                Alpha2::BT => "BT",
                #[cfg(feature = "bv")]
                Alpha2::BV => "BV",
                #[cfg(feature = "bw")]
                Alpha2::BW => "BW",
                #[cfg(feature = "by")]
                Alpha2::BY => "BY",
                #[cfg(feature = "bz")]
                Alpha2::BZ => "BZ",
                #[cfg(feature = "ca")]
                Alpha2::CA => "CA",
                #[cfg(feature = "cc")]
                Alpha2::CC => "CC",
                #[cfg(feature = "cd")]
                Alpha2::CD => "CD",
                #[cfg(feature = "cf")]
                Alpha2::CF => "CF",
                #[cfg(feature = "cg")]
                Alpha2::CG => "CG",
                #[cfg(feature = "ch")]
                Alpha2::CH => "CH",
                #[cfg(feature = "ci")]
                Alpha2::CI => "CI",
                #[cfg(feature = "ck")]
                Alpha2::CK => "CK",
                #[cfg(feature = "cl")]
                Alpha2::CL => "CL",
                #[cfg(feature = "cm")]
                Alpha2::CM => "CM",
                #[cfg(feature = "cn")]
                Alpha2::CN => "CN",
                #[cfg(feature = "co")]
                Alpha2::CO => "CO",
                #[cfg(feature = "cr")]
                Alpha2::CR => "CR",
                #[cfg(feature = "cu")]
                Alpha2::CU => "CU",
                #[cfg(feature = "cv")]
                Alpha2::CV => "CV",
                #[cfg(feature = "cw")]
                Alpha2::CW => "CW",
                #[cfg(feature = "cx")]
                Alpha2::CX => "CX",
                #[cfg(feature = "cy")]
                Alpha2::CY => "CY",
                #[cfg(feature = "cz")]
                Alpha2::CZ => "CZ",
                #[cfg(feature = "de")]
                Alpha2::DE => "DE",
                #[cfg(feature = "dj")]
                Alpha2::DJ => "DJ",
                #[cfg(feature = "dk")]
                Alpha2::DK => "DK",
                #[cfg(feature = "dm")]
                Alpha2::DM => "DM",
                #[cfg(feature = "do")]
                Alpha2::DO => "DO",
                #[cfg(feature = "dz")]
                Alpha2::DZ => "DZ",
                #[cfg(feature = "ec")]
                Alpha2::EC => "EC",
                #[cfg(feature = "ee")]
                Alpha2::EE => "EE",
                #[cfg(feature = "eg")]
                Alpha2::EG => "EG",
                #[cfg(feature = "eh")]
                Alpha2::EH => "EH",
                #[cfg(feature = "er")]
                Alpha2::ER => "ER",
                #[cfg(feature = "es")]
                Alpha2::ES => "ES",
                #[cfg(feature = "et")]
                Alpha2::ET => "ET",
                #[cfg(feature = "fi")]
                Alpha2::FI => "FI",
                #[cfg(feature = "fj")]
                Alpha2::FJ => "FJ",
                #[cfg(feature = "fk")]
                Alpha2::FK => "FK",
                #[cfg(feature = "fm")]
                Alpha2::FM => "FM",
                #[cfg(feature = "fo")]
                Alpha2::FO => "FO",
                #[cfg(feature = "fr")]
                Alpha2::FR => "FR",
                #[cfg(feature = "ga")]
                Alpha2::GA => "GA",
                #[cfg(feature = "gb")]
                Alpha2::GB => "GB",
                #[cfg(feature = "gd")]
                Alpha2::GD => "GD",
                #[cfg(feature = "ge")]
                Alpha2::GE => "GE",
                #[cfg(feature = "gf")]
                Alpha2::GF => "GF",
                #[cfg(feature = "gg")]
                Alpha2::GG => "GG",
                #[cfg(feature = "gh")]
                Alpha2::GH => "GH",
                #[cfg(feature = "gi")]
                Alpha2::GI => "GI",
                #[cfg(feature = "gl")]
                Alpha2::GL => "GL",
                #[cfg(feature = "gm")]
                Alpha2::GM => "GM",
                #[cfg(feature = "gn")]
                Alpha2::GN => "GN",
                #[cfg(feature = "gp")]
                Alpha2::GP => "GP",
                #[cfg(feature = "gq")]
                Alpha2::GQ => "GQ",
                #[cfg(feature = "gr")]
                Alpha2::GR => "GR",
                #[cfg(feature = "gs")]
                Alpha2::GS => "GS",
                #[cfg(feature = "gt")]
                Alpha2::GT => "GT",
                #[cfg(feature = "gu")]
                Alpha2::GU => "GU",
                #[cfg(feature = "gw")]
                Alpha2::GW => "GW",
                #[cfg(feature = "gy")]
                Alpha2::GY => "GY",
                #[cfg(feature = "hk")]
                Alpha2::HK => "HK",
                #[cfg(feature = "hm")]
                Alpha2::HM => "HM",
                #[cfg(feature = "hn")]
                Alpha2::HN => "HN",
                #[cfg(feature = "hr")]
                Alpha2::HR => "HR",
                #[cfg(feature = "ht")]
                Alpha2::HT => "HT",
                #[cfg(feature = "hu")]
                Alpha2::HU => "HU",
                #[cfg(feature = "id")]
                Alpha2::ID => "ID",
                #[cfg(feature = "ie")]
                Alpha2::IE => "IE",
                #[cfg(feature = "il")]
                Alpha2::IL => "IL",
                #[cfg(feature = "im")]
                Alpha2::IM => "IM",
                #[cfg(feature = "in")]
                Alpha2::IN => "IN",
                #[cfg(feature = "io")]
                Alpha2::IO => "IO",
                #[cfg(feature = "iq")]
                Alpha2::IQ => "IQ",
                #[cfg(feature = "ir")]
                Alpha2::IR => "IR",
                #[cfg(feature = "is")]
                Alpha2::IS => "IS",
                #[cfg(feature = "it")]
                Alpha2::IT => "IT",
                #[cfg(feature = "je")]
                Alpha2::JE => "JE",
                #[cfg(feature = "jm")]
                Alpha2::JM => "JM",
                #[cfg(feature = "jo")]
                Alpha2::JO => "JO",
                #[cfg(feature = "jp")]
                Alpha2::JP => "JP",
                #[cfg(feature = "ke")]
                Alpha2::KE => "KE",
                #[cfg(feature = "kg")]
                Alpha2::KG => "KG",
                #[cfg(feature = "kh")]
                Alpha2::KH => "KH",
                #[cfg(feature = "ki")]
                Alpha2::KI => "KI",
                #[cfg(feature = "km")]
                Alpha2::KM => "KM",
                #[cfg(feature = "kn")]
                Alpha2::KN => "KN",
                #[cfg(feature = "kp")]
                Alpha2::KP => "KP",
                #[cfg(feature = "kr")]
                Alpha2::KR => "KR",
                #[cfg(feature = "kw")]
                Alpha2::KW => "KW",
                #[cfg(feature = "ky")]
                Alpha2::KY => "KY",
                #[cfg(feature = "kz")]
                Alpha2::KZ => "KZ",
                #[cfg(feature = "la")]
                Alpha2::LA => "LA",
                #[cfg(feature = "lb")]
                Alpha2::LB => "LB",
                #[cfg(feature = "lc")]
                Alpha2::LC => "LC",
                #[cfg(feature = "li")]
                Alpha2::LI => "LI",
                #[cfg(feature = "lk")]
                Alpha2::LK => "LK",
                #[cfg(feature = "lr")]
                Alpha2::LR => "LR",
                #[cfg(feature = "ls")]
                Alpha2::LS => "LS",
                #[cfg(feature = "lt")]
                Alpha2::LT => "LT",
                #[cfg(feature = "lu")]
                Alpha2::LU => "LU",
                #[cfg(feature = "lv")]
                Alpha2::LV => "LV",
                #[cfg(feature = "ly")]
                Alpha2::LY => "LY",
                #[cfg(feature = "ma")]
                Alpha2::MA => "MA",
                #[cfg(feature = "mc")]
                Alpha2::MC => "MC",
                #[cfg(feature = "md")]
                Alpha2::MD => "MD",
                #[cfg(feature = "me")]
                Alpha2::ME => "ME",
                #[cfg(feature = "mf")]
                Alpha2::MF => "MF",
                #[cfg(feature = "mg")]
                Alpha2::MG => "MG",
                #[cfg(feature = "mh")]
                Alpha2::MH => "MH",
                #[cfg(feature = "mk")]
                Alpha2::MK => "MK",
                #[cfg(feature = "ml")]
                Alpha2::ML => "ML",
                #[cfg(feature = "mm")]
                Alpha2::MM => "MM",
                #[cfg(feature = "mn")]
                Alpha2::MN => "MN",
                #[cfg(feature = "mo")]
                Alpha2::MO => "MO",
                #[cfg(feature = "mp")]
                Alpha2::MP => "MP",
                #[cfg(feature = "mq")]
                Alpha2::MQ => "MQ",
                #[cfg(feature = "mr")]
                Alpha2::MR => "MR",
                #[cfg(feature = "ms")]
                Alpha2::MS => "MS",
                #[cfg(feature = "mt")]
                Alpha2::MT => "MT",
                #[cfg(feature = "mu")]
                Alpha2::MU => "MU",
                #[cfg(feature = "mv")]
                Alpha2::MV => "MV",
                #[cfg(feature = "mw")]
                Alpha2::MW => "MW",
                #[cfg(feature = "mx")]
                Alpha2::MX => "MX",
                #[cfg(feature = "my")]
                Alpha2::MY => "MY",
                #[cfg(feature = "mz")]
                Alpha2::MZ => "MZ",
                #[cfg(feature = "na")]
                Alpha2::NA => "NA",
                #[cfg(feature = "nc")]
                Alpha2::NC => "NC",
                #[cfg(feature = "ne")]
                Alpha2::NE => "NE",
                #[cfg(feature = "nf")]
                Alpha2::NF => "NF",
                #[cfg(feature = "ng")]
                Alpha2::NG => "NG",
                #[cfg(feature = "ni")]
                Alpha2::NI => "NI",
                #[cfg(feature = "nl")]
                Alpha2::NL => "NL",
                #[cfg(feature = "no")]
                Alpha2::NO => "NO",
                #[cfg(feature = "np")]
                Alpha2::NP => "NP",
                #[cfg(feature = "nr")]
                Alpha2::NR => "NR",
                #[cfg(feature = "nu")]
                Alpha2::NU => "NU",
                #[cfg(feature = "nz")]
                Alpha2::NZ => "NZ",
                #[cfg(feature = "om")]
                Alpha2::OM => "OM",
                #[cfg(feature = "pa")]
                Alpha2::PA => "PA",
                #[cfg(feature = "pe")]
                Alpha2::PE => "PE",
                #[cfg(feature = "pf")]
                Alpha2::PF => "PF",
                #[cfg(feature = "pg")]
                Alpha2::PG => "PG",
                #[cfg(feature = "ph")]
                Alpha2::PH => "PH",
                #[cfg(feature = "pk")]
                Alpha2::PK => "PK",
                #[cfg(feature = "pl")]
                Alpha2::PL => "PL",
                #[cfg(feature = "pm")]
                Alpha2::PM => "PM",
                #[cfg(feature = "pn")]
                Alpha2::PN => "PN",
                #[cfg(feature = "pr")]
                Alpha2::PR => "PR",
                #[cfg(feature = "ps")]
                Alpha2::PS => "PS",
                #[cfg(feature = "pt")]
                Alpha2::PT => "PT",
                #[cfg(feature = "pw")]
                Alpha2::PW => "PW",
                #[cfg(feature = "py")]
                Alpha2::PY => "PY",
                #[cfg(feature = "qa")]
                Alpha2::QA => "QA",
                #[cfg(feature = "re")]
                Alpha2::RE => "RE",
                #[cfg(feature = "ro")]
                Alpha2::RO => "RO",
                #[cfg(feature = "rs")]
                Alpha2::RS => "RS",
                #[cfg(feature = "ru")]
                Alpha2::RU => "RU",
                #[cfg(feature = "rw")]
                Alpha2::RW => "RW",
                #[cfg(feature = "sa")]
                Alpha2::SA => "SA",
                #[cfg(feature = "sb")]
                Alpha2::SB => "SB",
                #[cfg(feature = "sc")]
                Alpha2::SC => "SC",
                #[cfg(feature = "sd")]
                Alpha2::SD => "SD",
                #[cfg(feature = "se")]
                Alpha2::SE => "SE",
                #[cfg(feature = "sg")]
                Alpha2::SG => "SG",
                #[cfg(feature = "sh")]
                Alpha2::SH => "SH",
                #[cfg(feature = "si")]
                Alpha2::SI => "SI",
                #[cfg(feature = "sj")]
                Alpha2::SJ => "SJ",
                #[cfg(feature = "sk")]
                Alpha2::SK => "SK",
                #[cfg(feature = "sl")]
                Alpha2::SL => "SL",
                #[cfg(feature = "sm")]
                Alpha2::SM => "SM",
                #[cfg(feature = "sn")]
                Alpha2::SN => "SN",
                #[cfg(feature = "so")]
                Alpha2::SO => "SO",
                #[cfg(feature = "sr")]
                Alpha2::SR => "SR",
                #[cfg(feature = "ss")]
                Alpha2::SS => "SS",
                #[cfg(feature = "st")]
                Alpha2::ST => "ST",
                #[cfg(feature = "sv")]
                Alpha2::SV => "SV",
                #[cfg(feature = "sx")]
                Alpha2::SX => "SX",
                #[cfg(feature = "sy")]
                Alpha2::SY => "SY",
                #[cfg(feature = "sz")]
                Alpha2::SZ => "SZ",
                #[cfg(feature = "tc")]
                Alpha2::TC => "TC",
                #[cfg(feature = "td")]
                Alpha2::TD => "TD",
                #[cfg(feature = "tf")]
                Alpha2::TF => "TF",
                #[cfg(feature = "tg")]
                Alpha2::TG => "TG",
                #[cfg(feature = "th")]
                Alpha2::TH => "TH",
                #[cfg(feature = "tj")]
                Alpha2::TJ => "TJ",
                #[cfg(feature = "tk")]
                Alpha2::TK => "TK",
                #[cfg(feature = "tl")]
                Alpha2::TL => "TL",
                #[cfg(feature = "tm")]
                Alpha2::TM => "TM",
                #[cfg(feature = "tn")]
                Alpha2::TN => "TN",
                #[cfg(feature = "to")]
                Alpha2::TO => "TO",
                #[cfg(feature = "tr")]
                Alpha2::TR => "TR",
                #[cfg(feature = "tt")]
                Alpha2::TT => "TT",
                #[cfg(feature = "tv")]
                Alpha2::TV => "TV",
                #[cfg(feature = "tw")]
                Alpha2::TW => "TW",
                #[cfg(feature = "tz")]
                Alpha2::TZ => "TZ",
                #[cfg(feature = "ua")]
                Alpha2::UA => "UA",
                #[cfg(feature = "ug")]
                Alpha2::UG => "UG",
                #[cfg(feature = "um")]
                Alpha2::UM => "UM",
                #[cfg(feature = "us")]
                Alpha2::US => "US",
                #[cfg(feature = "uy")]
                Alpha2::UY => "UY",
                #[cfg(feature = "uz")]
                Alpha2::UZ => "UZ",
                #[cfg(feature = "va")]
                Alpha2::VA => "VA",
                #[cfg(feature = "vc")]
                Alpha2::VC => "VC",
                #[cfg(feature = "ve")]
                Alpha2::VE => "VE",
                #[cfg(feature = "vg")]
                Alpha2::VG => "VG",
                #[cfg(feature = "vi")]
                Alpha2::VI => "VI",
                #[cfg(feature = "vn")]
                Alpha2::VN => "VN",
                #[cfg(feature = "vu")]
                Alpha2::VU => "VU",
                #[cfg(feature = "wf")]
                Alpha2::WF => "WF",
                #[cfg(feature = "ws")]
                Alpha2::WS => "WS",
                #[cfg(feature = "ye")]
                Alpha2::YE => "YE",
                #[cfg(feature = "yt")]
                Alpha2::YT => "YT",
                #[cfg(feature = "za")]
                Alpha2::ZA => "ZA",
                #[cfg(feature = "zm")]
                Alpha2::ZM => "ZM",
                #[cfg(feature = "zw")]
                Alpha2::ZW => "ZW",
            }
            .to_string()
        }
    }

    #[cfg(feature = "subdivisions")]
    use crate::Subdivision;
    #[cfg(feature = "subdivisions")]
    use std::collections::HashMap;
    #[cfg(feature = "subdivisions")]
    impl Alpha2 {
        pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
            match self {
                #[cfg(feature = "ad")]
                Alpha2::AD => crate::countries::ad::subdivisions::new(),
                #[cfg(feature = "ae")]
                Alpha2::AE => crate::countries::ae::subdivisions::new(),
                #[cfg(feature = "af")]
                Alpha2::AF => crate::countries::af::subdivisions::new(),
                #[cfg(feature = "ag")]
                Alpha2::AG => crate::countries::ag::subdivisions::new(),
                #[cfg(feature = "ai")]
                Alpha2::AI => crate::countries::ai::subdivisions::new(),
                #[cfg(feature = "al")]
                Alpha2::AL => crate::countries::al::subdivisions::new(),
                #[cfg(feature = "am")]
                Alpha2::AM => crate::countries::am::subdivisions::new(),
                #[cfg(feature = "ao")]
                Alpha2::AO => crate::countries::ao::subdivisions::new(),
                #[cfg(feature = "aq")]
                Alpha2::AQ => crate::countries::aq::subdivisions::new(),
                #[cfg(feature = "ar")]
                Alpha2::AR => crate::countries::ar::subdivisions::new(),
                #[cfg(feature = "as")]
                Alpha2::AS => crate::countries::r#as::subdivisions::new(),
                #[cfg(feature = "at")]
                Alpha2::AT => crate::countries::at::subdivisions::new(),
                #[cfg(feature = "au")]
                Alpha2::AU => crate::countries::au::subdivisions::new(),
                #[cfg(feature = "aw")]
                Alpha2::AW => crate::countries::aw::subdivisions::new(),
                #[cfg(feature = "ax")]
                Alpha2::AX => crate::countries::ax::subdivisions::new(),
                #[cfg(feature = "az")]
                Alpha2::AZ => crate::countries::az::subdivisions::new(),
                #[cfg(feature = "ba")]
                Alpha2::BA => crate::countries::ba::subdivisions::new(),
                #[cfg(feature = "bb")]
                Alpha2::BB => crate::countries::bb::subdivisions::new(),
                #[cfg(feature = "bd")]
                Alpha2::BD => crate::countries::bd::subdivisions::new(),
                #[cfg(feature = "be")]
                Alpha2::BE => crate::countries::be::subdivisions::new(),
                #[cfg(feature = "bf")]
                Alpha2::BF => crate::countries::bf::subdivisions::new(),
                #[cfg(feature = "bg")]
                Alpha2::BG => crate::countries::bg::subdivisions::new(),
                #[cfg(feature = "bh")]
                Alpha2::BH => crate::countries::bh::subdivisions::new(),
                #[cfg(feature = "bi")]
                Alpha2::BI => crate::countries::bi::subdivisions::new(),
                #[cfg(feature = "bj")]
                Alpha2::BJ => crate::countries::bj::subdivisions::new(),
                #[cfg(feature = "bl")]
                Alpha2::BL => crate::countries::bl::subdivisions::new(),
                #[cfg(feature = "bm")]
                Alpha2::BM => crate::countries::bm::subdivisions::new(),
                #[cfg(feature = "bn")]
                Alpha2::BN => crate::countries::bn::subdivisions::new(),
                #[cfg(feature = "bo")]
                Alpha2::BO => crate::countries::bo::subdivisions::new(),
                #[cfg(feature = "bq")]
                Alpha2::BQ => crate::countries::bq::subdivisions::new(),
                #[cfg(feature = "br")]
                Alpha2::BR => crate::countries::br::subdivisions::new(),
                #[cfg(feature = "bs")]
                Alpha2::BS => crate::countries::bs::subdivisions::new(),
                #[cfg(feature = "bt")]
                Alpha2::BT => crate::countries::bt::subdivisions::new(),
                #[cfg(feature = "bv")]
                Alpha2::BV => crate::countries::bv::subdivisions::new(),
                #[cfg(feature = "bw")]
                Alpha2::BW => crate::countries::bw::subdivisions::new(),
                #[cfg(feature = "by")]
                Alpha2::BY => crate::countries::by::subdivisions::new(),
                #[cfg(feature = "bz")]
                Alpha2::BZ => crate::countries::bz::subdivisions::new(),
                #[cfg(feature = "ca")]
                Alpha2::CA => crate::countries::ca::subdivisions::new(),
                #[cfg(feature = "cc")]
                Alpha2::CC => crate::countries::cc::subdivisions::new(),
                #[cfg(feature = "cd")]
                Alpha2::CD => crate::countries::cd::subdivisions::new(),
                #[cfg(feature = "cf")]
                Alpha2::CF => crate::countries::cf::subdivisions::new(),
                #[cfg(feature = "cg")]
                Alpha2::CG => crate::countries::cg::subdivisions::new(),
                #[cfg(feature = "ch")]
                Alpha2::CH => crate::countries::ch::subdivisions::new(),
                #[cfg(feature = "ci")]
                Alpha2::CI => crate::countries::ci::subdivisions::new(),
                #[cfg(feature = "ck")]
                Alpha2::CK => crate::countries::ck::subdivisions::new(),
                #[cfg(feature = "cl")]
                Alpha2::CL => crate::countries::cl::subdivisions::new(),
                #[cfg(feature = "cm")]
                Alpha2::CM => crate::countries::cm::subdivisions::new(),
                #[cfg(feature = "cn")]
                Alpha2::CN => crate::countries::cn::subdivisions::new(),
                #[cfg(feature = "co")]
                Alpha2::CO => crate::countries::co::subdivisions::new(),
                #[cfg(feature = "cr")]
                Alpha2::CR => crate::countries::cr::subdivisions::new(),
                #[cfg(feature = "cu")]
                Alpha2::CU => crate::countries::cu::subdivisions::new(),
                #[cfg(feature = "cv")]
                Alpha2::CV => crate::countries::cv::subdivisions::new(),
                #[cfg(feature = "cw")]
                Alpha2::CW => crate::countries::cw::subdivisions::new(),
                #[cfg(feature = "cx")]
                Alpha2::CX => crate::countries::cx::subdivisions::new(),
                #[cfg(feature = "cy")]
                Alpha2::CY => crate::countries::cy::subdivisions::new(),
                #[cfg(feature = "cz")]
                Alpha2::CZ => crate::countries::cz::subdivisions::new(),
                #[cfg(feature = "de")]
                Alpha2::DE => crate::countries::de::subdivisions::new(),
                #[cfg(feature = "dj")]
                Alpha2::DJ => crate::countries::dj::subdivisions::new(),
                #[cfg(feature = "dk")]
                Alpha2::DK => crate::countries::dk::subdivisions::new(),
                #[cfg(feature = "dm")]
                Alpha2::DM => crate::countries::dm::subdivisions::new(),
                #[cfg(feature = "do")]
                Alpha2::DO => crate::countries::r#do::subdivisions::new(),
                #[cfg(feature = "dz")]
                Alpha2::DZ => crate::countries::dz::subdivisions::new(),
                #[cfg(feature = "ec")]
                Alpha2::EC => crate::countries::ec::subdivisions::new(),
                #[cfg(feature = "ee")]
                Alpha2::EE => crate::countries::ee::subdivisions::new(),
                #[cfg(feature = "eg")]
                Alpha2::EG => crate::countries::eg::subdivisions::new(),
                #[cfg(feature = "eh")]
                Alpha2::EH => crate::countries::eh::subdivisions::new(),
                #[cfg(feature = "er")]
                Alpha2::ER => crate::countries::er::subdivisions::new(),
                #[cfg(feature = "es")]
                Alpha2::ES => crate::countries::es::subdivisions::new(),
                #[cfg(feature = "et")]
                Alpha2::ET => crate::countries::et::subdivisions::new(),
                #[cfg(feature = "fi")]
                Alpha2::FI => crate::countries::fi::subdivisions::new(),
                #[cfg(feature = "fj")]
                Alpha2::FJ => crate::countries::fj::subdivisions::new(),
                #[cfg(feature = "fk")]
                Alpha2::FK => crate::countries::fk::subdivisions::new(),
                #[cfg(feature = "fm")]
                Alpha2::FM => crate::countries::fm::subdivisions::new(),
                #[cfg(feature = "fo")]
                Alpha2::FO => crate::countries::fo::subdivisions::new(),
                #[cfg(feature = "fr")]
                Alpha2::FR => crate::countries::fr::subdivisions::new(),
                #[cfg(feature = "ga")]
                Alpha2::GA => crate::countries::ga::subdivisions::new(),
                #[cfg(feature = "gb")]
                Alpha2::GB => crate::countries::gb::subdivisions::new(),
                #[cfg(feature = "gd")]
                Alpha2::GD => crate::countries::gd::subdivisions::new(),
                #[cfg(feature = "ge")]
                Alpha2::GE => crate::countries::ge::subdivisions::new(),
                #[cfg(feature = "gf")]
                Alpha2::GF => crate::countries::gf::subdivisions::new(),
                #[cfg(feature = "gg")]
                Alpha2::GG => crate::countries::gg::subdivisions::new(),
                #[cfg(feature = "gh")]
                Alpha2::GH => crate::countries::gh::subdivisions::new(),
                #[cfg(feature = "gi")]
                Alpha2::GI => crate::countries::gi::subdivisions::new(),
                #[cfg(feature = "gl")]
                Alpha2::GL => crate::countries::gl::subdivisions::new(),
                #[cfg(feature = "gm")]
                Alpha2::GM => crate::countries::gm::subdivisions::new(),
                #[cfg(feature = "gn")]
                Alpha2::GN => crate::countries::gn::subdivisions::new(),
                #[cfg(feature = "gp")]
                Alpha2::GP => crate::countries::gp::subdivisions::new(),
                #[cfg(feature = "gq")]
                Alpha2::GQ => crate::countries::gq::subdivisions::new(),
                #[cfg(feature = "gr")]
                Alpha2::GR => crate::countries::gr::subdivisions::new(),
                #[cfg(feature = "gs")]
                Alpha2::GS => crate::countries::gs::subdivisions::new(),
                #[cfg(feature = "gt")]
                Alpha2::GT => crate::countries::gt::subdivisions::new(),
                #[cfg(feature = "gu")]
                Alpha2::GU => crate::countries::gu::subdivisions::new(),
                #[cfg(feature = "gw")]
                Alpha2::GW => crate::countries::gw::subdivisions::new(),
                #[cfg(feature = "gy")]
                Alpha2::GY => crate::countries::gy::subdivisions::new(),
                #[cfg(feature = "hk")]
                Alpha2::HK => crate::countries::hk::subdivisions::new(),
                #[cfg(feature = "hm")]
                Alpha2::HM => crate::countries::hm::subdivisions::new(),
                #[cfg(feature = "hn")]
                Alpha2::HN => crate::countries::hn::subdivisions::new(),
                #[cfg(feature = "hr")]
                Alpha2::HR => crate::countries::hr::subdivisions::new(),
                #[cfg(feature = "ht")]
                Alpha2::HT => crate::countries::ht::subdivisions::new(),
                #[cfg(feature = "hu")]
                Alpha2::HU => crate::countries::hu::subdivisions::new(),
                #[cfg(feature = "id")]
                Alpha2::ID => crate::countries::id::subdivisions::new(),
                #[cfg(feature = "ie")]
                Alpha2::IE => crate::countries::ie::subdivisions::new(),
                #[cfg(feature = "il")]
                Alpha2::IL => crate::countries::il::subdivisions::new(),
                #[cfg(feature = "im")]
                Alpha2::IM => crate::countries::im::subdivisions::new(),
                #[cfg(feature = "in")]
                Alpha2::IN => crate::countries::r#in::subdivisions::new(),
                #[cfg(feature = "io")]
                Alpha2::IO => crate::countries::io::subdivisions::new(),
                #[cfg(feature = "iq")]
                Alpha2::IQ => crate::countries::iq::subdivisions::new(),
                #[cfg(feature = "ir")]
                Alpha2::IR => crate::countries::ir::subdivisions::new(),
                #[cfg(feature = "is")]
                Alpha2::IS => crate::countries::is::subdivisions::new(),
                #[cfg(feature = "it")]
                Alpha2::IT => crate::countries::it::subdivisions::new(),
                #[cfg(feature = "je")]
                Alpha2::JE => crate::countries::je::subdivisions::new(),
                #[cfg(feature = "jm")]
                Alpha2::JM => crate::countries::jm::subdivisions::new(),
                #[cfg(feature = "jo")]
                Alpha2::JO => crate::countries::jo::subdivisions::new(),
                #[cfg(feature = "jp")]
                Alpha2::JP => crate::countries::jp::subdivisions::new(),
                #[cfg(feature = "ke")]
                Alpha2::KE => crate::countries::ke::subdivisions::new(),
                #[cfg(feature = "kg")]
                Alpha2::KG => crate::countries::kg::subdivisions::new(),
                #[cfg(feature = "kh")]
                Alpha2::KH => crate::countries::kh::subdivisions::new(),
                #[cfg(feature = "ki")]
                Alpha2::KI => crate::countries::ki::subdivisions::new(),
                #[cfg(feature = "km")]
                Alpha2::KM => crate::countries::km::subdivisions::new(),
                #[cfg(feature = "kn")]
                Alpha2::KN => crate::countries::kn::subdivisions::new(),
                #[cfg(feature = "kp")]
                Alpha2::KP => crate::countries::kp::subdivisions::new(),
                #[cfg(feature = "kr")]
                Alpha2::KR => crate::countries::kr::subdivisions::new(),
                #[cfg(feature = "kw")]
                Alpha2::KW => crate::countries::kw::subdivisions::new(),
                #[cfg(feature = "ky")]
                Alpha2::KY => crate::countries::ky::subdivisions::new(),
                #[cfg(feature = "kz")]
                Alpha2::KZ => crate::countries::kz::subdivisions::new(),
                #[cfg(feature = "la")]
                Alpha2::LA => crate::countries::la::subdivisions::new(),
                #[cfg(feature = "lb")]
                Alpha2::LB => crate::countries::lb::subdivisions::new(),
                #[cfg(feature = "lc")]
                Alpha2::LC => crate::countries::lc::subdivisions::new(),
                #[cfg(feature = "li")]
                Alpha2::LI => crate::countries::li::subdivisions::new(),
                #[cfg(feature = "lk")]
                Alpha2::LK => crate::countries::lk::subdivisions::new(),
                #[cfg(feature = "lr")]
                Alpha2::LR => crate::countries::lr::subdivisions::new(),
                #[cfg(feature = "ls")]
                Alpha2::LS => crate::countries::ls::subdivisions::new(),
                #[cfg(feature = "lt")]
                Alpha2::LT => crate::countries::lt::subdivisions::new(),
                #[cfg(feature = "lu")]
                Alpha2::LU => crate::countries::lu::subdivisions::new(),
                #[cfg(feature = "lv")]
                Alpha2::LV => crate::countries::lv::subdivisions::new(),
                #[cfg(feature = "ly")]
                Alpha2::LY => crate::countries::ly::subdivisions::new(),
                #[cfg(feature = "ma")]
                Alpha2::MA => crate::countries::ma::subdivisions::new(),
                #[cfg(feature = "mc")]
                Alpha2::MC => crate::countries::mc::subdivisions::new(),
                #[cfg(feature = "md")]
                Alpha2::MD => crate::countries::md::subdivisions::new(),
                #[cfg(feature = "me")]
                Alpha2::ME => crate::countries::me::subdivisions::new(),
                #[cfg(feature = "mf")]
                Alpha2::MF => crate::countries::mf::subdivisions::new(),
                #[cfg(feature = "mg")]
                Alpha2::MG => crate::countries::mg::subdivisions::new(),
                #[cfg(feature = "mh")]
                Alpha2::MH => crate::countries::mh::subdivisions::new(),
                #[cfg(feature = "mk")]
                Alpha2::MK => crate::countries::mk::subdivisions::new(),
                #[cfg(feature = "ml")]
                Alpha2::ML => crate::countries::ml::subdivisions::new(),
                #[cfg(feature = "mm")]
                Alpha2::MM => crate::countries::mm::subdivisions::new(),
                #[cfg(feature = "mn")]
                Alpha2::MN => crate::countries::mn::subdivisions::new(),
                #[cfg(feature = "mo")]
                Alpha2::MO => crate::countries::mo::subdivisions::new(),
                #[cfg(feature = "mp")]
                Alpha2::MP => crate::countries::mp::subdivisions::new(),
                #[cfg(feature = "mq")]
                Alpha2::MQ => crate::countries::mq::subdivisions::new(),
                #[cfg(feature = "mr")]
                Alpha2::MR => crate::countries::mr::subdivisions::new(),
                #[cfg(feature = "ms")]
                Alpha2::MS => crate::countries::ms::subdivisions::new(),
                #[cfg(feature = "mt")]
                Alpha2::MT => crate::countries::mt::subdivisions::new(),
                #[cfg(feature = "mu")]
                Alpha2::MU => crate::countries::mu::subdivisions::new(),
                #[cfg(feature = "mv")]
                Alpha2::MV => crate::countries::mv::subdivisions::new(),
                #[cfg(feature = "mw")]
                Alpha2::MW => crate::countries::mw::subdivisions::new(),
                #[cfg(feature = "mx")]
                Alpha2::MX => crate::countries::mx::subdivisions::new(),
                #[cfg(feature = "my")]
                Alpha2::MY => crate::countries::my::subdivisions::new(),
                #[cfg(feature = "mz")]
                Alpha2::MZ => crate::countries::mz::subdivisions::new(),
                #[cfg(feature = "na")]
                Alpha2::NA => crate::countries::na::subdivisions::new(),
                #[cfg(feature = "nc")]
                Alpha2::NC => crate::countries::nc::subdivisions::new(),
                #[cfg(feature = "ne")]
                Alpha2::NE => crate::countries::ne::subdivisions::new(),
                #[cfg(feature = "nf")]
                Alpha2::NF => crate::countries::nf::subdivisions::new(),
                #[cfg(feature = "ng")]
                Alpha2::NG => crate::countries::ng::subdivisions::new(),
                #[cfg(feature = "ni")]
                Alpha2::NI => crate::countries::ni::subdivisions::new(),
                #[cfg(feature = "nl")]
                Alpha2::NL => crate::countries::nl::subdivisions::new(),
                #[cfg(feature = "no")]
                Alpha2::NO => crate::countries::no::subdivisions::new(),
                #[cfg(feature = "np")]
                Alpha2::NP => crate::countries::np::subdivisions::new(),
                #[cfg(feature = "nr")]
                Alpha2::NR => crate::countries::nr::subdivisions::new(),
                #[cfg(feature = "nu")]
                Alpha2::NU => crate::countries::nu::subdivisions::new(),
                #[cfg(feature = "nz")]
                Alpha2::NZ => crate::countries::nz::subdivisions::new(),
                #[cfg(feature = "om")]
                Alpha2::OM => crate::countries::om::subdivisions::new(),
                #[cfg(feature = "pa")]
                Alpha2::PA => crate::countries::pa::subdivisions::new(),
                #[cfg(feature = "pe")]
                Alpha2::PE => crate::countries::pe::subdivisions::new(),
                #[cfg(feature = "pf")]
                Alpha2::PF => crate::countries::pf::subdivisions::new(),
                #[cfg(feature = "pg")]
                Alpha2::PG => crate::countries::pg::subdivisions::new(),
                #[cfg(feature = "ph")]
                Alpha2::PH => crate::countries::ph::subdivisions::new(),
                #[cfg(feature = "pk")]
                Alpha2::PK => crate::countries::pk::subdivisions::new(),
                #[cfg(feature = "pl")]
                Alpha2::PL => crate::countries::pl::subdivisions::new(),
                #[cfg(feature = "pm")]
                Alpha2::PM => crate::countries::pm::subdivisions::new(),
                #[cfg(feature = "pn")]
                Alpha2::PN => crate::countries::pn::subdivisions::new(),
                #[cfg(feature = "pr")]
                Alpha2::PR => crate::countries::pr::subdivisions::new(),
                #[cfg(feature = "ps")]
                Alpha2::PS => crate::countries::ps::subdivisions::new(),
                #[cfg(feature = "pt")]
                Alpha2::PT => crate::countries::pt::subdivisions::new(),
                #[cfg(feature = "pw")]
                Alpha2::PW => crate::countries::pw::subdivisions::new(),
                #[cfg(feature = "py")]
                Alpha2::PY => crate::countries::py::subdivisions::new(),
                #[cfg(feature = "qa")]
                Alpha2::QA => crate::countries::qa::subdivisions::new(),
                #[cfg(feature = "re")]
                Alpha2::RE => crate::countries::re::subdivisions::new(),
                #[cfg(feature = "ro")]
                Alpha2::RO => crate::countries::ro::subdivisions::new(),
                #[cfg(feature = "rs")]
                Alpha2::RS => crate::countries::rs::subdivisions::new(),
                #[cfg(feature = "ru")]
                Alpha2::RU => crate::countries::ru::subdivisions::new(),
                #[cfg(feature = "rw")]
                Alpha2::RW => crate::countries::rw::subdivisions::new(),
                #[cfg(feature = "sa")]
                Alpha2::SA => crate::countries::sa::subdivisions::new(),
                #[cfg(feature = "sb")]
                Alpha2::SB => crate::countries::sb::subdivisions::new(),
                #[cfg(feature = "sc")]
                Alpha2::SC => crate::countries::sc::subdivisions::new(),
                #[cfg(feature = "sd")]
                Alpha2::SD => crate::countries::sd::subdivisions::new(),
                #[cfg(feature = "se")]
                Alpha2::SE => crate::countries::se::subdivisions::new(),
                #[cfg(feature = "sg")]
                Alpha2::SG => crate::countries::sg::subdivisions::new(),
                #[cfg(feature = "sh")]
                Alpha2::SH => crate::countries::sh::subdivisions::new(),
                #[cfg(feature = "si")]
                Alpha2::SI => crate::countries::si::subdivisions::new(),
                #[cfg(feature = "sj")]
                Alpha2::SJ => crate::countries::sj::subdivisions::new(),
                #[cfg(feature = "sk")]
                Alpha2::SK => crate::countries::sk::subdivisions::new(),
                #[cfg(feature = "sl")]
                Alpha2::SL => crate::countries::sl::subdivisions::new(),
                #[cfg(feature = "sm")]
                Alpha2::SM => crate::countries::sm::subdivisions::new(),
                #[cfg(feature = "sn")]
                Alpha2::SN => crate::countries::sn::subdivisions::new(),
                #[cfg(feature = "so")]
                Alpha2::SO => crate::countries::so::subdivisions::new(),
                #[cfg(feature = "sr")]
                Alpha2::SR => crate::countries::sr::subdivisions::new(),
                #[cfg(feature = "ss")]
                Alpha2::SS => crate::countries::ss::subdivisions::new(),
                #[cfg(feature = "st")]
                Alpha2::ST => crate::countries::st::subdivisions::new(),
                #[cfg(feature = "sv")]
                Alpha2::SV => crate::countries::sv::subdivisions::new(),
                #[cfg(feature = "sx")]
                Alpha2::SX => crate::countries::sx::subdivisions::new(),
                #[cfg(feature = "sy")]
                Alpha2::SY => crate::countries::sy::subdivisions::new(),
                #[cfg(feature = "sz")]
                Alpha2::SZ => crate::countries::sz::subdivisions::new(),
                #[cfg(feature = "tc")]
                Alpha2::TC => crate::countries::tc::subdivisions::new(),
                #[cfg(feature = "td")]
                Alpha2::TD => crate::countries::td::subdivisions::new(),
                #[cfg(feature = "tf")]
                Alpha2::TF => crate::countries::tf::subdivisions::new(),
                #[cfg(feature = "tg")]
                Alpha2::TG => crate::countries::tg::subdivisions::new(),
                #[cfg(feature = "th")]
                Alpha2::TH => crate::countries::th::subdivisions::new(),
                #[cfg(feature = "tj")]
                Alpha2::TJ => crate::countries::tj::subdivisions::new(),
                #[cfg(feature = "tk")]
                Alpha2::TK => crate::countries::tk::subdivisions::new(),
                #[cfg(feature = "tl")]
                Alpha2::TL => crate::countries::tl::subdivisions::new(),
                #[cfg(feature = "tm")]
                Alpha2::TM => crate::countries::tm::subdivisions::new(),
                #[cfg(feature = "tn")]
                Alpha2::TN => crate::countries::tn::subdivisions::new(),
                #[cfg(feature = "to")]
                Alpha2::TO => crate::countries::to::subdivisions::new(),
                #[cfg(feature = "tr")]
                Alpha2::TR => crate::countries::tr::subdivisions::new(),
                #[cfg(feature = "tt")]
                Alpha2::TT => crate::countries::tt::subdivisions::new(),
                #[cfg(feature = "tv")]
                Alpha2::TV => crate::countries::tv::subdivisions::new(),
                #[cfg(feature = "tw")]
                Alpha2::TW => crate::countries::tw::subdivisions::new(),
                #[cfg(feature = "tz")]
                Alpha2::TZ => crate::countries::tz::subdivisions::new(),
                #[cfg(feature = "ua")]
                Alpha2::UA => crate::countries::ua::subdivisions::new(),
                #[cfg(feature = "ug")]
                Alpha2::UG => crate::countries::ug::subdivisions::new(),
                #[cfg(feature = "um")]
                Alpha2::UM => crate::countries::um::subdivisions::new(),
                #[cfg(feature = "us")]
                Alpha2::US => crate::countries::us::subdivisions::new(),
                #[cfg(feature = "uy")]
                Alpha2::UY => crate::countries::uy::subdivisions::new(),
                #[cfg(feature = "uz")]
                Alpha2::UZ => crate::countries::uz::subdivisions::new(),
                #[cfg(feature = "va")]
                Alpha2::VA => crate::countries::va::subdivisions::new(),
                #[cfg(feature = "vc")]
                Alpha2::VC => crate::countries::vc::subdivisions::new(),
                #[cfg(feature = "ve")]
                Alpha2::VE => crate::countries::ve::subdivisions::new(),
                #[cfg(feature = "vg")]
                Alpha2::VG => crate::countries::vg::subdivisions::new(),
                #[cfg(feature = "vi")]
                Alpha2::VI => crate::countries::vi::subdivisions::new(),
                #[cfg(feature = "vn")]
                Alpha2::VN => crate::countries::vn::subdivisions::new(),
                #[cfg(feature = "vu")]
                Alpha2::VU => crate::countries::vu::subdivisions::new(),
                #[cfg(feature = "wf")]
                Alpha2::WF => crate::countries::wf::subdivisions::new(),
                #[cfg(feature = "ws")]
                Alpha2::WS => crate::countries::ws::subdivisions::new(),
                #[cfg(feature = "ye")]
                Alpha2::YE => crate::countries::ye::subdivisions::new(),
                #[cfg(feature = "yt")]
                Alpha2::YT => crate::countries::yt::subdivisions::new(),
                #[cfg(feature = "za")]
                Alpha2::ZA => crate::countries::za::subdivisions::new(),
                #[cfg(feature = "zm")]
                Alpha2::ZM => crate::countries::zm::subdivisions::new(),
                #[cfg(feature = "zw")]
                Alpha2::ZW => crate::countries::zw::subdivisions::new(),
            }
        }
    }

    impl From<Alpha2> for Country {
        fn from(alpha2: Alpha2) -> Self {
            match alpha2 {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                Alpha2::AD => crate::countries::ad::new(),
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                Alpha2::AE => crate::countries::ae::new(),
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                Alpha2::AF => crate::countries::af::new(),
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                Alpha2::AG => crate::countries::ag::new(),
                #[cfg(feature = "ai")] // Anguilla (Americas)
                Alpha2::AI => crate::countries::ai::new(),
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                Alpha2::AL => crate::countries::al::new(),
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                Alpha2::AM => crate::countries::am::new(),
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                Alpha2::AO => crate::countries::ao::new(),
                #[cfg(feature = "aq")] // Antarctica
                Alpha2::AQ => crate::countries::aq::new(),
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                Alpha2::AR => crate::countries::ar::new(),
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                Alpha2::AS => crate::countries::r#as::new(),
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                Alpha2::AT => crate::countries::at::new(),
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                Alpha2::AU => crate::countries::au::new(),
                #[cfg(feature = "aw")] // Aruba (Americas)
                Alpha2::AW => crate::countries::aw::new(),
                #[cfg(feature = "ax")] // Åland (Europe)
                Alpha2::AX => crate::countries::ax::new(),
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                Alpha2::AZ => crate::countries::az::new(),
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                Alpha2::BA => crate::countries::ba::new(),
                #[cfg(feature = "bb")] // Barbados (Americas)
                Alpha2::BB => crate::countries::bb::new(),
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                Alpha2::BD => crate::countries::bd::new(),
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                Alpha2::BE => crate::countries::be::new(),
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                Alpha2::BF => crate::countries::bf::new(),
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                Alpha2::BG => crate::countries::bg::new(),
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                Alpha2::BH => crate::countries::bh::new(),
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                Alpha2::BI => crate::countries::bi::new(),
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                Alpha2::BJ => crate::countries::bj::new(),
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                Alpha2::BL => crate::countries::bl::new(),
                #[cfg(feature = "bm")] // Bermuda (Americas)
                Alpha2::BM => crate::countries::bm::new(),
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                Alpha2::BN => crate::countries::bn::new(),
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                Alpha2::BO => crate::countries::bo::new(),
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                Alpha2::BQ => crate::countries::bq::new(),
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                Alpha2::BR => crate::countries::br::new(),
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                Alpha2::BS => crate::countries::bs::new(),
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                Alpha2::BT => crate::countries::bt::new(),
                #[cfg(feature = "bv")] // Bouvet Island
                Alpha2::BV => crate::countries::bv::new(),
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                Alpha2::BW => crate::countries::bw::new(),
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                Alpha2::BY => crate::countries::by::new(),
                #[cfg(feature = "bz")] // Belize (Americas)
                Alpha2::BZ => crate::countries::bz::new(),
                #[cfg(feature = "ca")] // Canada (Americas)
                Alpha2::CA => crate::countries::ca::new(),
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                Alpha2::CC => crate::countries::cc::new(),
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                Alpha2::CD => crate::countries::cd::new(),
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                Alpha2::CF => crate::countries::cf::new(),
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                Alpha2::CG => crate::countries::cg::new(),
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                Alpha2::CH => crate::countries::ch::new(),
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                Alpha2::CI => crate::countries::ci::new(),
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                Alpha2::CK => crate::countries::ck::new(),
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                Alpha2::CL => crate::countries::cl::new(),
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                Alpha2::CM => crate::countries::cm::new(),
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                Alpha2::CN => crate::countries::cn::new(),
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                Alpha2::CO => crate::countries::co::new(),
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                Alpha2::CR => crate::countries::cr::new(),
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                Alpha2::CU => crate::countries::cu::new(),
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                Alpha2::CV => crate::countries::cv::new(),
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                Alpha2::CW => crate::countries::cw::new(),
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                Alpha2::CX => crate::countries::cx::new(),
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                Alpha2::CY => crate::countries::cy::new(),
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                Alpha2::CZ => crate::countries::cz::new(),
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                Alpha2::DE => crate::countries::de::new(),
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                Alpha2::DJ => crate::countries::dj::new(),
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                Alpha2::DK => crate::countries::dk::new(),
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                Alpha2::DM => crate::countries::dm::new(),
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                Alpha2::DO => crate::countries::r#do::new(),
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                Alpha2::DZ => crate::countries::dz::new(),
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                Alpha2::EC => crate::countries::ec::new(),
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                Alpha2::EE => crate::countries::ee::new(),
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                Alpha2::EG => crate::countries::eg::new(),
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                Alpha2::EH => crate::countries::eh::new(),
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                Alpha2::ER => crate::countries::er::new(),
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                Alpha2::ES => crate::countries::es::new(),
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                Alpha2::ET => crate::countries::et::new(),
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                Alpha2::FI => crate::countries::fi::new(),
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                Alpha2::FJ => crate::countries::fj::new(),
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                Alpha2::FK => crate::countries::fk::new(),
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                Alpha2::FM => crate::countries::fm::new(),
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                Alpha2::FO => crate::countries::fo::new(),
                #[cfg(feature = "fr")] // The French Republic (Europe)
                Alpha2::FR => crate::countries::fr::new(),
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                Alpha2::GA => crate::countries::ga::new(),
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                Alpha2::GB => crate::countries::gb::new(),
                #[cfg(feature = "gd")] // Grenada (Americas)
                Alpha2::GD => crate::countries::gd::new(),
                #[cfg(feature = "ge")] // Georgia (Asia)
                Alpha2::GE => crate::countries::ge::new(),
                #[cfg(feature = "gf")] // Guyane (Americas)
                Alpha2::GF => crate::countries::gf::new(),
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                Alpha2::GG => crate::countries::gg::new(),
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                Alpha2::GH => crate::countries::gh::new(),
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                Alpha2::GI => crate::countries::gi::new(),
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                Alpha2::GL => crate::countries::gl::new(),
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                Alpha2::GM => crate::countries::gm::new(),
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                Alpha2::GN => crate::countries::gn::new(),
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                Alpha2::GP => crate::countries::gp::new(),
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                Alpha2::GQ => crate::countries::gq::new(),
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                Alpha2::GR => crate::countries::gr::new(),
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                Alpha2::GS => crate::countries::gs::new(),
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                Alpha2::GT => crate::countries::gt::new(),
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                Alpha2::GU => crate::countries::gu::new(),
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                Alpha2::GW => crate::countries::gw::new(),
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                Alpha2::GY => crate::countries::gy::new(),
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                Alpha2::HK => crate::countries::hk::new(),
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                Alpha2::HM => crate::countries::hm::new(),
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                Alpha2::HN => crate::countries::hn::new(),
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                Alpha2::HR => crate::countries::hr::new(),
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                Alpha2::HT => crate::countries::ht::new(),
                #[cfg(feature = "hu")] // Hungary (Europe)
                Alpha2::HU => crate::countries::hu::new(),
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                Alpha2::ID => crate::countries::id::new(),
                #[cfg(feature = "ie")] // Ireland (Europe)
                Alpha2::IE => crate::countries::ie::new(),
                #[cfg(feature = "il")] // The State of Israel (Asia)
                Alpha2::IL => crate::countries::il::new(),
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                Alpha2::IM => crate::countries::im::new(),
                #[cfg(feature = "in")] // The Republic of India (Asia)
                Alpha2::IN => crate::countries::r#in::new(),
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                Alpha2::IO => crate::countries::io::new(),
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                Alpha2::IQ => crate::countries::iq::new(),
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                Alpha2::IR => crate::countries::ir::new(),
                #[cfg(feature = "is")] // Iceland (Europe)
                Alpha2::IS => crate::countries::is::new(),
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                Alpha2::IT => crate::countries::it::new(),
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                Alpha2::JE => crate::countries::je::new(),
                #[cfg(feature = "jm")] // Jamaica (Americas)
                Alpha2::JM => crate::countries::jm::new(),
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                Alpha2::JO => crate::countries::jo::new(),
                #[cfg(feature = "jp")] // Japan (Asia)
                Alpha2::JP => crate::countries::jp::new(),
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                Alpha2::KE => crate::countries::ke::new(),
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                Alpha2::KG => crate::countries::kg::new(),
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                Alpha2::KH => crate::countries::kh::new(),
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                Alpha2::KI => crate::countries::ki::new(),
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                Alpha2::KM => crate::countries::km::new(),
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                Alpha2::KN => crate::countries::kn::new(),
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                Alpha2::KP => crate::countries::kp::new(),
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                Alpha2::KR => crate::countries::kr::new(),
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                Alpha2::KW => crate::countries::kw::new(),
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                Alpha2::KY => crate::countries::ky::new(),
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                Alpha2::KZ => crate::countries::kz::new(),
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                Alpha2::LA => crate::countries::la::new(),
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                Alpha2::LB => crate::countries::lb::new(),
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                Alpha2::LC => crate::countries::lc::new(),
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                Alpha2::LI => crate::countries::li::new(),
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                Alpha2::LK => crate::countries::lk::new(),
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                Alpha2::LR => crate::countries::lr::new(),
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                Alpha2::LS => crate::countries::ls::new(),
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                Alpha2::LT => crate::countries::lt::new(),
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                Alpha2::LU => crate::countries::lu::new(),
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                Alpha2::LV => crate::countries::lv::new(),
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                Alpha2::LY => crate::countries::ly::new(),
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                Alpha2::MA => crate::countries::ma::new(),
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                Alpha2::MC => crate::countries::mc::new(),
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                Alpha2::MD => crate::countries::md::new(),
                #[cfg(feature = "me")] // Montenegro (Europe)
                Alpha2::ME => crate::countries::me::new(),
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                Alpha2::MF => crate::countries::mf::new(),
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                Alpha2::MG => crate::countries::mg::new(),
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                Alpha2::MH => crate::countries::mh::new(),
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                Alpha2::MK => crate::countries::mk::new(),
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                Alpha2::ML => crate::countries::ml::new(),
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                Alpha2::MM => crate::countries::mm::new(),
                #[cfg(feature = "mn")] // Mongolia (Asia)
                Alpha2::MN => crate::countries::mn::new(),
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                Alpha2::MO => crate::countries::mo::new(),
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                Alpha2::MP => crate::countries::mp::new(),
                #[cfg(feature = "mq")] // Martinique (Americas)
                Alpha2::MQ => crate::countries::mq::new(),
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                Alpha2::MR => crate::countries::mr::new(),
                #[cfg(feature = "ms")] // Montserrat (Americas)
                Alpha2::MS => crate::countries::ms::new(),
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                Alpha2::MT => crate::countries::mt::new(),
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                Alpha2::MU => crate::countries::mu::new(),
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                Alpha2::MV => crate::countries::mv::new(),
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                Alpha2::MW => crate::countries::mw::new(),
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                Alpha2::MX => crate::countries::mx::new(),
                #[cfg(feature = "my")] // Malaysia (Asia)
                Alpha2::MY => crate::countries::my::new(),
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                Alpha2::MZ => crate::countries::mz::new(),
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                Alpha2::NA => crate::countries::na::new(),
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                Alpha2::NC => crate::countries::nc::new(),
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                Alpha2::NE => crate::countries::ne::new(),
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                Alpha2::NF => crate::countries::nf::new(),
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                Alpha2::NG => crate::countries::ng::new(),
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                Alpha2::NI => crate::countries::ni::new(),
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                Alpha2::NL => crate::countries::nl::new(),
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                Alpha2::NO => crate::countries::no::new(),
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                Alpha2::NP => crate::countries::np::new(),
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                Alpha2::NR => crate::countries::nr::new(),
                #[cfg(feature = "nu")] // Niue (Oceania)
                Alpha2::NU => crate::countries::nu::new(),
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                Alpha2::NZ => crate::countries::nz::new(),
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                Alpha2::OM => crate::countries::om::new(),
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                Alpha2::PA => crate::countries::pa::new(),
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                Alpha2::PE => crate::countries::pe::new(),
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                Alpha2::PF => crate::countries::pf::new(),
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                Alpha2::PG => crate::countries::pg::new(),
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                Alpha2::PH => crate::countries::ph::new(),
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                Alpha2::PK => crate::countries::pk::new(),
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                Alpha2::PL => crate::countries::pl::new(),
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                Alpha2::PM => crate::countries::pm::new(),
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                Alpha2::PN => crate::countries::pn::new(),
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                Alpha2::PR => crate::countries::pr::new(),
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                Alpha2::PS => crate::countries::ps::new(),
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                Alpha2::PT => crate::countries::pt::new(),
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                Alpha2::PW => crate::countries::pw::new(),
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                Alpha2::PY => crate::countries::py::new(),
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                Alpha2::QA => crate::countries::qa::new(),
                #[cfg(feature = "re")] // Réunion (Africa)
                Alpha2::RE => crate::countries::re::new(),
                #[cfg(feature = "ro")] // Romania (Europe)
                Alpha2::RO => crate::countries::ro::new(),
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                Alpha2::RS => crate::countries::rs::new(),
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                Alpha2::RU => crate::countries::ru::new(),
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                Alpha2::RW => crate::countries::rw::new(),
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                Alpha2::SA => crate::countries::sa::new(),
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                Alpha2::SB => crate::countries::sb::new(),
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                Alpha2::SC => crate::countries::sc::new(),
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                Alpha2::SD => crate::countries::sd::new(),
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                Alpha2::SE => crate::countries::se::new(),
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                Alpha2::SG => crate::countries::sg::new(),
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                Alpha2::SH => crate::countries::sh::new(),
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                Alpha2::SI => crate::countries::si::new(),
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                Alpha2::SJ => crate::countries::sj::new(),
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                Alpha2::SK => crate::countries::sk::new(),
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                Alpha2::SL => crate::countries::sl::new(),
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                Alpha2::SM => crate::countries::sm::new(),
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                Alpha2::SN => crate::countries::sn::new(),
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                Alpha2::SO => crate::countries::so::new(),
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                Alpha2::SR => crate::countries::sr::new(),
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                Alpha2::SS => crate::countries::ss::new(),
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                Alpha2::ST => crate::countries::st::new(),
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                Alpha2::SV => crate::countries::sv::new(),
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                Alpha2::SX => crate::countries::sx::new(),
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                Alpha2::SY => crate::countries::sy::new(),
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                Alpha2::SZ => crate::countries::sz::new(),
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                Alpha2::TC => crate::countries::tc::new(),
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                Alpha2::TD => crate::countries::td::new(),
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                Alpha2::TF => crate::countries::tf::new(),
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                Alpha2::TG => crate::countries::tg::new(),
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                Alpha2::TH => crate::countries::th::new(),
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                Alpha2::TJ => crate::countries::tj::new(),
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                Alpha2::TK => crate::countries::tk::new(),
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                Alpha2::TL => crate::countries::tl::new(),
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                Alpha2::TM => crate::countries::tm::new(),
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                Alpha2::TN => crate::countries::tn::new(),
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                Alpha2::TO => crate::countries::to::new(),
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                Alpha2::TR => crate::countries::tr::new(),
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                Alpha2::TT => crate::countries::tt::new(),
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                Alpha2::TV => crate::countries::tv::new(),
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                Alpha2::TW => crate::countries::tw::new(),
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                Alpha2::TZ => crate::countries::tz::new(),
                #[cfg(feature = "ua")] // Ukraine (Europe)
                Alpha2::UA => crate::countries::ua::new(),
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                Alpha2::UG => crate::countries::ug::new(),
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                Alpha2::UM => crate::countries::um::new(),
                #[cfg(feature = "us")] // The United States of America (Americas)
                Alpha2::US => crate::countries::us::new(),
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                Alpha2::UY => crate::countries::uy::new(),
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                Alpha2::UZ => crate::countries::uz::new(),
                #[cfg(feature = "va")] // The Holy See (Europe)
                Alpha2::VA => crate::countries::va::new(),
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                Alpha2::VC => crate::countries::vc::new(),
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                Alpha2::VE => crate::countries::ve::new(),
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                Alpha2::VG => crate::countries::vg::new(),
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                Alpha2::VI => crate::countries::vi::new(),
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                Alpha2::VN => crate::countries::vn::new(),
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                Alpha2::VU => crate::countries::vu::new(),
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                Alpha2::WF => crate::countries::wf::new(),
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                Alpha2::WS => crate::countries::ws::new(),
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                Alpha2::YE => crate::countries::ye::new(),
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                Alpha2::YT => crate::countries::yt::new(),
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                Alpha2::ZA => crate::countries::za::new(),
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                Alpha2::ZM => crate::countries::zm::new(),
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                Alpha2::ZW => crate::countries::zw::new(),
            }
        }
    }

    impl Alpha2 {
        pub fn to_alpha3(&self) -> Alpha3 {
            match self {
                #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
                Alpha2::AD => Alpha3::AND,
                #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
                Alpha2::AE => Alpha3::ARE,
                #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
                Alpha2::AF => Alpha3::AFG,
                #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
                Alpha2::AG => Alpha3::ATG,
                #[cfg(feature = "ai")] // Anguilla (Americas)
                Alpha2::AI => Alpha3::AIA,
                #[cfg(feature = "al")] // The Republic of Albania (Europe)
                Alpha2::AL => Alpha3::ALB,
                #[cfg(feature = "am")] // The Republic of Armenia (Asia)
                Alpha2::AM => Alpha3::ARM,
                #[cfg(feature = "ao")] // The Republic of Angola (Africa)
                Alpha2::AO => Alpha3::AGO,
                #[cfg(feature = "aq")] // Antarctica
                Alpha2::AQ => Alpha3::ATA,
                #[cfg(feature = "ar")] // The Argentine Republic (Americas)
                Alpha2::AR => Alpha3::ARG,
                #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
                Alpha2::AS => Alpha3::ASM,
                #[cfg(feature = "at")] // The Republic of Austria (Europe)
                Alpha2::AT => Alpha3::AUT,
                #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
                Alpha2::AU => Alpha3::AUS,
                #[cfg(feature = "aw")] // Aruba (Americas)
                Alpha2::AW => Alpha3::ABW,
                #[cfg(feature = "ax")] // Åland (Europe)
                Alpha2::AX => Alpha3::ALA,
                #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
                Alpha2::AZ => Alpha3::AZE,
                #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
                Alpha2::BA => Alpha3::BIH,
                #[cfg(feature = "bb")] // Barbados (Americas)
                Alpha2::BB => Alpha3::BRB,
                #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
                Alpha2::BD => Alpha3::BGD,
                #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
                Alpha2::BE => Alpha3::BEL,
                #[cfg(feature = "bf")] // Burkina Faso (Africa)
                Alpha2::BF => Alpha3::BFA,
                #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
                Alpha2::BG => Alpha3::BGR,
                #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
                Alpha2::BH => Alpha3::BHR,
                #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
                Alpha2::BI => Alpha3::BDI,
                #[cfg(feature = "bj")] // The Republic of Benin (Africa)
                Alpha2::BJ => Alpha3::BEN,
                #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
                Alpha2::BL => Alpha3::BLM,
                #[cfg(feature = "bm")] // Bermuda (Americas)
                Alpha2::BM => Alpha3::BMU,
                #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
                Alpha2::BN => Alpha3::BRN,
                #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
                Alpha2::BO => Alpha3::BOL,
                #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
                Alpha2::BQ => Alpha3::BES,
                #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
                Alpha2::BR => Alpha3::BRA,
                #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
                Alpha2::BS => Alpha3::BHS,
                #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
                Alpha2::BT => Alpha3::BTN,
                #[cfg(feature = "bv")] // Bouvet Island
                Alpha2::BV => Alpha3::BVT,
                #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
                Alpha2::BW => Alpha3::BWA,
                #[cfg(feature = "by")] // The Republic of Belarus (Europe)
                Alpha2::BY => Alpha3::BLR,
                #[cfg(feature = "bz")] // Belize (Americas)
                Alpha2::BZ => Alpha3::BLZ,
                #[cfg(feature = "ca")] // Canada (Americas)
                Alpha2::CA => Alpha3::CAN,
                #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
                Alpha2::CC => Alpha3::CCK,
                #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
                Alpha2::CD => Alpha3::COD,
                #[cfg(feature = "cf")] // The Central African Republic (Africa)
                Alpha2::CF => Alpha3::CAF,
                #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
                Alpha2::CG => Alpha3::COG,
                #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
                Alpha2::CH => Alpha3::CHE,
                #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
                Alpha2::CI => Alpha3::CIV,
                #[cfg(feature = "ck")] // The Cook Islands (Oceania)
                Alpha2::CK => Alpha3::COK,
                #[cfg(feature = "cl")] // The Republic of Chile (Americas)
                Alpha2::CL => Alpha3::CHL,
                #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
                Alpha2::CM => Alpha3::CMR,
                #[cfg(feature = "cn")] // The People's Republic of China (Asia)
                Alpha2::CN => Alpha3::CHN,
                #[cfg(feature = "co")] // The Republic of Colombia (Americas)
                Alpha2::CO => Alpha3::COL,
                #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
                Alpha2::CR => Alpha3::CRI,
                #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
                Alpha2::CU => Alpha3::CUB,
                #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
                Alpha2::CV => Alpha3::CPV,
                #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
                Alpha2::CW => Alpha3::CUW,
                #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
                Alpha2::CX => Alpha3::CXR,
                #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
                Alpha2::CY => Alpha3::CYP,
                #[cfg(feature = "cz")] // The Czech Republic (Europe)
                Alpha2::CZ => Alpha3::CZE,
                #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
                Alpha2::DE => Alpha3::DEU,
                #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
                Alpha2::DJ => Alpha3::DJI,
                #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
                Alpha2::DK => Alpha3::DNK,
                #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
                Alpha2::DM => Alpha3::DMA,
                #[cfg(feature = "do")] // The Dominican Republic (Americas)
                Alpha2::DO => Alpha3::DOM,
                #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
                Alpha2::DZ => Alpha3::DZA,
                #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
                Alpha2::EC => Alpha3::ECU,
                #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
                Alpha2::EE => Alpha3::EST,
                #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
                Alpha2::EG => Alpha3::EGY,
                #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
                Alpha2::EH => Alpha3::ESH,
                #[cfg(feature = "er")] // The State of Eritrea (Africa)
                Alpha2::ER => Alpha3::ERI,
                #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
                Alpha2::ES => Alpha3::ESP,
                #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
                Alpha2::ET => Alpha3::ETH,
                #[cfg(feature = "fi")] // The Republic of Finland (Europe)
                Alpha2::FI => Alpha3::FIN,
                #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
                Alpha2::FJ => Alpha3::FJI,
                #[cfg(feature = "fk")] // The Falkland Islands (Americas)
                Alpha2::FK => Alpha3::FLK,
                #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
                Alpha2::FM => Alpha3::FSM,
                #[cfg(feature = "fo")] // The Faroe Islands (Europe)
                Alpha2::FO => Alpha3::FRO,
                #[cfg(feature = "fr")] // The French Republic (Europe)
                Alpha2::FR => Alpha3::FRA,
                #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
                Alpha2::GA => Alpha3::GAB,
                #[cfg(feature = "gb")]
                // The United Kingdom of Great Britain and Northern Ireland (Europe)
                Alpha2::GB => Alpha3::GBR,
                #[cfg(feature = "gd")] // Grenada (Americas)
                Alpha2::GD => Alpha3::GRD,
                #[cfg(feature = "ge")] // Georgia (Asia)
                Alpha2::GE => Alpha3::GEO,
                #[cfg(feature = "gf")] // Guyane (Americas)
                Alpha2::GF => Alpha3::GUF,
                #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
                Alpha2::GG => Alpha3::GGY,
                #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
                Alpha2::GH => Alpha3::GHA,
                #[cfg(feature = "gi")] // Gibraltar (Europe)
                Alpha2::GI => Alpha3::GIB,
                #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
                Alpha2::GL => Alpha3::GRL,
                #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
                Alpha2::GM => Alpha3::GMB,
                #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
                Alpha2::GN => Alpha3::GIN,
                #[cfg(feature = "gp")] // Guadeloupe (Americas)
                Alpha2::GP => Alpha3::GLP,
                #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
                Alpha2::GQ => Alpha3::GNQ,
                #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
                Alpha2::GR => Alpha3::GRC,
                #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
                Alpha2::GS => Alpha3::SGS,
                #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
                Alpha2::GT => Alpha3::GTM,
                #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
                Alpha2::GU => Alpha3::GUM,
                #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
                Alpha2::GW => Alpha3::GNB,
                #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
                Alpha2::GY => Alpha3::GUY,
                #[cfg(feature = "hk")]
                // The Hong Kong Special Administrative Region of China (Asia)
                Alpha2::HK => Alpha3::HKG,
                #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
                Alpha2::HM => Alpha3::HMD,
                #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
                Alpha2::HN => Alpha3::HND,
                #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
                Alpha2::HR => Alpha3::HRV,
                #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
                Alpha2::HT => Alpha3::HTI,
                #[cfg(feature = "hu")] // Hungary (Europe)
                Alpha2::HU => Alpha3::HUN,
                #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
                Alpha2::ID => Alpha3::IDN,
                #[cfg(feature = "ie")] // Ireland (Europe)
                Alpha2::IE => Alpha3::IRL,
                #[cfg(feature = "il")] // The State of Israel (Asia)
                Alpha2::IL => Alpha3::ISR,
                #[cfg(feature = "im")] // The Isle of Man (Europe)
                Alpha2::IM => Alpha3::IMN,
                #[cfg(feature = "in")] // The Republic of India (Asia)
                Alpha2::IN => Alpha3::IND,
                #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
                Alpha2::IO => Alpha3::IOT,
                #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
                Alpha2::IQ => Alpha3::IRQ,
                #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
                Alpha2::IR => Alpha3::IRN,
                #[cfg(feature = "is")] // Iceland (Europe)
                Alpha2::IS => Alpha3::ISL,
                #[cfg(feature = "it")] // The Italian Republic (Europe)
                Alpha2::IT => Alpha3::ITA,
                #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
                Alpha2::JE => Alpha3::JEY,
                #[cfg(feature = "jm")] // Jamaica (Americas)
                Alpha2::JM => Alpha3::JAM,
                #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
                Alpha2::JO => Alpha3::JOR,
                #[cfg(feature = "jp")] // Japan (Asia)
                Alpha2::JP => Alpha3::JPN,
                #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
                Alpha2::KE => Alpha3::KEN,
                #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
                Alpha2::KG => Alpha3::KGZ,
                #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
                Alpha2::KH => Alpha3::KHM,
                #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
                Alpha2::KI => Alpha3::KIR,
                #[cfg(feature = "km")] // The Union of the Comoros (Africa)
                Alpha2::KM => Alpha3::COM,
                #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
                Alpha2::KN => Alpha3::KNA,
                #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
                Alpha2::KP => Alpha3::PRK,
                #[cfg(feature = "kr")] // The Republic of Korea (Asia)
                Alpha2::KR => Alpha3::KOR,
                #[cfg(feature = "kw")] // The State of Kuwait (Asia)
                Alpha2::KW => Alpha3::KWT,
                #[cfg(feature = "ky")] // The Cayman Islands (Americas)
                Alpha2::KY => Alpha3::CYM,
                #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
                Alpha2::KZ => Alpha3::KAZ,
                #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
                Alpha2::LA => Alpha3::LAO,
                #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
                Alpha2::LB => Alpha3::LBN,
                #[cfg(feature = "lc")] // Saint Lucia (Americas)
                Alpha2::LC => Alpha3::LCA,
                #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
                Alpha2::LI => Alpha3::LIE,
                #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
                Alpha2::LK => Alpha3::LKA,
                #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
                Alpha2::LR => Alpha3::LBR,
                #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
                Alpha2::LS => Alpha3::LSO,
                #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
                Alpha2::LT => Alpha3::LTU,
                #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
                Alpha2::LU => Alpha3::LUX,
                #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
                Alpha2::LV => Alpha3::LVA,
                #[cfg(feature = "ly")] // The State of Libya (Africa)
                Alpha2::LY => Alpha3::LBY,
                #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
                Alpha2::MA => Alpha3::MAR,
                #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
                Alpha2::MC => Alpha3::MCO,
                #[cfg(feature = "md")] // The Republic of Moldova (Europe)
                Alpha2::MD => Alpha3::MDA,
                #[cfg(feature = "me")] // Montenegro (Europe)
                Alpha2::ME => Alpha3::MNE,
                #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
                Alpha2::MF => Alpha3::MAF,
                #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
                Alpha2::MG => Alpha3::MDG,
                #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
                Alpha2::MH => Alpha3::MHL,
                #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
                Alpha2::MK => Alpha3::MKD,
                #[cfg(feature = "ml")] // The Republic of Mali (Africa)
                Alpha2::ML => Alpha3::MLI,
                #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
                Alpha2::MM => Alpha3::MMR,
                #[cfg(feature = "mn")] // Mongolia (Asia)
                Alpha2::MN => Alpha3::MNG,
                #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
                Alpha2::MO => Alpha3::MAC,
                #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
                Alpha2::MP => Alpha3::MNP,
                #[cfg(feature = "mq")] // Martinique (Americas)
                Alpha2::MQ => Alpha3::MTQ,
                #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
                Alpha2::MR => Alpha3::MRT,
                #[cfg(feature = "ms")] // Montserrat (Americas)
                Alpha2::MS => Alpha3::MSR,
                #[cfg(feature = "mt")] // The Republic of Malta (Europe)
                Alpha2::MT => Alpha3::MLT,
                #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
                Alpha2::MU => Alpha3::MUS,
                #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
                Alpha2::MV => Alpha3::MDV,
                #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
                Alpha2::MW => Alpha3::MWI,
                #[cfg(feature = "mx")] // The United Mexican States (Americas)
                Alpha2::MX => Alpha3::MEX,
                #[cfg(feature = "my")] // Malaysia (Asia)
                Alpha2::MY => Alpha3::MYS,
                #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
                Alpha2::MZ => Alpha3::MOZ,
                #[cfg(feature = "na")] // The Republic of Namibia (Africa)
                Alpha2::NA => Alpha3::NAM,
                #[cfg(feature = "nc")] // New Caledonia (Oceania)
                Alpha2::NC => Alpha3::NCL,
                #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
                Alpha2::NE => Alpha3::NER,
                #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
                Alpha2::NF => Alpha3::NFK,
                #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
                Alpha2::NG => Alpha3::NGA,
                #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
                Alpha2::NI => Alpha3::NIC,
                #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
                Alpha2::NL => Alpha3::NLD,
                #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
                Alpha2::NO => Alpha3::NOR,
                #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
                Alpha2::NP => Alpha3::NPL,
                #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
                Alpha2::NR => Alpha3::NRU,
                #[cfg(feature = "nu")] // Niue (Oceania)
                Alpha2::NU => Alpha3::NIU,
                #[cfg(feature = "nz")] // New Zealand (Oceania)
                Alpha2::NZ => Alpha3::NZL,
                #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
                Alpha2::OM => Alpha3::OMN,
                #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
                Alpha2::PA => Alpha3::PAN,
                #[cfg(feature = "pe")] // The Republic of Perú (Americas)
                Alpha2::PE => Alpha3::PER,
                #[cfg(feature = "pf")] // French Polynesia (Oceania)
                Alpha2::PF => Alpha3::PYF,
                #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
                Alpha2::PG => Alpha3::PNG,
                #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
                Alpha2::PH => Alpha3::PHL,
                #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
                Alpha2::PK => Alpha3::PAK,
                #[cfg(feature = "pl")] // The Republic of Poland (Europe)
                Alpha2::PL => Alpha3::POL,
                #[cfg(feature = "pm")]
                // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
                Alpha2::PM => Alpha3::SPM,
                #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
                Alpha2::PN => Alpha3::PCN,
                #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
                Alpha2::PR => Alpha3::PRI,
                #[cfg(feature = "ps")] // The State of Palestine (Asia)
                Alpha2::PS => Alpha3::PSE,
                #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
                Alpha2::PT => Alpha3::PRT,
                #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
                Alpha2::PW => Alpha3::PLW,
                #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
                Alpha2::PY => Alpha3::PRY,
                #[cfg(feature = "qa")] // The State of Qatar (Asia)
                Alpha2::QA => Alpha3::QAT,
                #[cfg(feature = "re")] // Réunion (Africa)
                Alpha2::RE => Alpha3::REU,
                #[cfg(feature = "ro")] // Romania (Europe)
                Alpha2::RO => Alpha3::ROU,
                #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
                Alpha2::RS => Alpha3::SRB,
                #[cfg(feature = "ru")] // The Russian Federation (Europe)
                Alpha2::RU => Alpha3::RUS,
                #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
                Alpha2::RW => Alpha3::RWA,
                #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
                Alpha2::SA => Alpha3::SAU,
                #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
                Alpha2::SB => Alpha3::SLB,
                #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
                Alpha2::SC => Alpha3::SYC,
                #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
                Alpha2::SD => Alpha3::SDN,
                #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
                Alpha2::SE => Alpha3::SWE,
                #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
                Alpha2::SG => Alpha3::SGP,
                #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
                Alpha2::SH => Alpha3::SHN,
                #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
                Alpha2::SI => Alpha3::SVN,
                #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
                Alpha2::SJ => Alpha3::SJM,
                #[cfg(feature = "sk")] // The Slovak Republic (Europe)
                Alpha2::SK => Alpha3::SVK,
                #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
                Alpha2::SL => Alpha3::SLE,
                #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
                Alpha2::SM => Alpha3::SMR,
                #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
                Alpha2::SN => Alpha3::SEN,
                #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
                Alpha2::SO => Alpha3::SOM,
                #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
                Alpha2::SR => Alpha3::SUR,
                #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
                Alpha2::SS => Alpha3::SSD,
                #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
                Alpha2::ST => Alpha3::STP,
                #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
                Alpha2::SV => Alpha3::SLV,
                #[cfg(feature = "sx")] // Sint Maarten (Americas)
                Alpha2::SX => Alpha3::SXM,
                #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
                Alpha2::SY => Alpha3::SYR,
                #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
                Alpha2::SZ => Alpha3::SWZ,
                #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
                Alpha2::TC => Alpha3::TCA,
                #[cfg(feature = "td")] // The Republic of Chad (Africa)
                Alpha2::TD => Alpha3::TCD,
                #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
                Alpha2::TF => Alpha3::ATF,
                #[cfg(feature = "tg")] // The Togolese Republic (Africa)
                Alpha2::TG => Alpha3::TGO,
                #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
                Alpha2::TH => Alpha3::THA,
                #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
                Alpha2::TJ => Alpha3::TJK,
                #[cfg(feature = "tk")] // Tokelau (Oceania)
                Alpha2::TK => Alpha3::TKL,
                #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
                Alpha2::TL => Alpha3::TLS,
                #[cfg(feature = "tm")] // Turkmenistan (Asia)
                Alpha2::TM => Alpha3::TKM,
                #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
                Alpha2::TN => Alpha3::TUN,
                #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
                Alpha2::TO => Alpha3::TON,
                #[cfg(feature = "tr")] // The Republic of Turkey (Asia)
                Alpha2::TR => Alpha3::TUR,
                #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
                Alpha2::TT => Alpha3::TTO,
                #[cfg(feature = "tv")] // Tuvalu (Oceania)
                Alpha2::TV => Alpha3::TUV,
                #[cfg(feature = "tw")] // The Republic of China (Asia)
                Alpha2::TW => Alpha3::TWN,
                #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
                Alpha2::TZ => Alpha3::TZA,
                #[cfg(feature = "ua")] // Ukraine (Europe)
                Alpha2::UA => Alpha3::UKR,
                #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
                Alpha2::UG => Alpha3::UGA,
                #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
                Alpha2::UM => Alpha3::UMI,
                #[cfg(feature = "us")] // The United States of America (Americas)
                Alpha2::US => Alpha3::USA,
                #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
                Alpha2::UY => Alpha3::URY,
                #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
                Alpha2::UZ => Alpha3::UZB,
                #[cfg(feature = "va")] // The Holy See (Europe)
                Alpha2::VA => Alpha3::VAT,
                #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
                Alpha2::VC => Alpha3::VCT,
                #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
                Alpha2::VE => Alpha3::VEN,
                #[cfg(feature = "vg")] // The Virgin Islands (Americas)
                Alpha2::VG => Alpha3::VGB,
                #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
                Alpha2::VI => Alpha3::VIR,
                #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
                Alpha2::VN => Alpha3::VNM,
                #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
                Alpha2::VU => Alpha3::VUT,
                #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
                Alpha2::WF => Alpha3::WLF,
                #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
                Alpha2::WS => Alpha3::WSM,
                #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
                Alpha2::YE => Alpha3::YEM,
                #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
                Alpha2::YT => Alpha3::MYT,
                #[cfg(feature = "za")] // The Republic of South Africa (Africa)
                Alpha2::ZA => Alpha3::ZAF,
                #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
                Alpha2::ZM => Alpha3::ZMB,
                #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
                Alpha2::ZW => Alpha3::ZWE,
            }
        }
    }
}
#[cfg(not(any(
    feature = "ad",
    feature = "ae",
    feature = "af",
    feature = "ag",
    feature = "ai",
    feature = "al",
    feature = "am",
    feature = "ao",
    feature = "aq",
    feature = "ar",
    feature = "as",
    feature = "at",
    feature = "au",
    feature = "aw",
    feature = "ax",
    feature = "az",
    feature = "ba",
    feature = "bb",
    feature = "bd",
    feature = "be",
    feature = "bf",
    feature = "bg",
    feature = "bh",
    feature = "bi",
    feature = "bj",
    feature = "bl",
    feature = "bm",
    feature = "bn",
    feature = "bo",
    feature = "bq",
    feature = "br",
    feature = "bs",
    feature = "bt",
    feature = "bv",
    feature = "bw",
    feature = "by",
    feature = "bz",
    feature = "ca",
    feature = "cc",
    feature = "cd",
    feature = "cf",
    feature = "cg",
    feature = "ch",
    feature = "ci",
    feature = "ck",
    feature = "cl",
    feature = "cm",
    feature = "cn",
    feature = "co",
    feature = "cr",
    feature = "cu",
    feature = "cv",
    feature = "cw",
    feature = "cx",
    feature = "cy",
    feature = "cz",
    feature = "de",
    feature = "dj",
    feature = "dk",
    feature = "dm",
    feature = "do",
    feature = "dz",
    feature = "ec",
    feature = "ee",
    feature = "eg",
    feature = "eh",
    feature = "er",
    feature = "es",
    feature = "et",
    feature = "fi",
    feature = "fj",
    feature = "fk",
    feature = "fm",
    feature = "fo",
    feature = "fr",
    feature = "ga",
    feature = "gb",
    feature = "gd",
    feature = "ge",
    feature = "gf",
    feature = "gg",
    feature = "gh",
    feature = "gi",
    feature = "gl",
    feature = "gm",
    feature = "gn",
    feature = "gp",
    feature = "gq",
    feature = "gr",
    feature = "gs",
    feature = "gt",
    feature = "gu",
    feature = "gw",
    feature = "gy",
    feature = "hk",
    feature = "hm",
    feature = "hn",
    feature = "hr",
    feature = "ht",
    feature = "hu",
    feature = "id",
    feature = "ie",
    feature = "il",
    feature = "im",
    feature = "in",
    feature = "io",
    feature = "iq",
    feature = "ir",
    feature = "is",
    feature = "it",
    feature = "je",
    feature = "jm",
    feature = "jo",
    feature = "jp",
    feature = "ke",
    feature = "kg",
    feature = "kh",
    feature = "ki",
    feature = "km",
    feature = "kn",
    feature = "kp",
    feature = "kr",
    feature = "kw",
    feature = "ky",
    feature = "kz",
    feature = "la",
    feature = "lb",
    feature = "lc",
    feature = "li",
    feature = "lk",
    feature = "lr",
    feature = "ls",
    feature = "lt",
    feature = "lu",
    feature = "lv",
    feature = "ly",
    feature = "ma",
    feature = "mc",
    feature = "md",
    feature = "me",
    feature = "mf",
    feature = "mg",
    feature = "mh",
    feature = "mk",
    feature = "ml",
    feature = "mm",
    feature = "mn",
    feature = "mo",
    feature = "mp",
    feature = "mq",
    feature = "mr",
    feature = "ms",
    feature = "mt",
    feature = "mu",
    feature = "mv",
    feature = "mw",
    feature = "mx",
    feature = "my",
    feature = "mz",
    feature = "na",
    feature = "nc",
    feature = "ne",
    feature = "nf",
    feature = "ng",
    feature = "ni",
    feature = "nl",
    feature = "no",
    feature = "np",
    feature = "nr",
    feature = "nu",
    feature = "nz",
    feature = "om",
    feature = "pa",
    feature = "pe",
    feature = "pf",
    feature = "pg",
    feature = "ph",
    feature = "pk",
    feature = "pl",
    feature = "pm",
    feature = "pn",
    feature = "pr",
    feature = "ps",
    feature = "pt",
    feature = "pw",
    feature = "py",
    feature = "qa",
    feature = "re",
    feature = "ro",
    feature = "rs",
    feature = "ru",
    feature = "rw",
    feature = "sa",
    feature = "sb",
    feature = "sc",
    feature = "sd",
    feature = "se",
    feature = "sg",
    feature = "sh",
    feature = "si",
    feature = "sj",
    feature = "sk",
    feature = "sl",
    feature = "sm",
    feature = "sn",
    feature = "so",
    feature = "sr",
    feature = "ss",
    feature = "st",
    feature = "sv",
    feature = "sx",
    feature = "sy",
    feature = "sz",
    feature = "tc",
    feature = "td",
    feature = "tf",
    feature = "tg",
    feature = "th",
    feature = "tj",
    feature = "tk",
    feature = "tl",
    feature = "tm",
    feature = "tn",
    feature = "to",
    feature = "tr",
    feature = "tt",
    feature = "tv",
    feature = "tw",
    feature = "tz",
    feature = "ua",
    feature = "ug",
    feature = "um",
    feature = "us",
    feature = "uy",
    feature = "uz",
    feature = "va",
    feature = "vc",
    feature = "ve",
    feature = "vg",
    feature = "vi",
    feature = "vn",
    feature = "vu",
    feature = "wf",
    feature = "ws",
    feature = "ye",
    feature = "yt",
    feature = "za",
    feature = "zm",
    feature = "zw",
)))]
mod impls {
    use super::{Alpha2, Alpha3, Country};
    use crate::SearchError;

    impl TryFrom<&str> for Alpha2 {
        type Error = SearchError;

        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature is used");
        }
    }

    impl ToString for Alpha2 {
        fn to_string(&self) -> String {
            unimplemented!("No country feature is used");
        }
    }

    #[cfg(feature = "subdivisions")]
    use crate::Subdivision;
    #[cfg(feature = "subdivisions")]
    use std::collections::HashMap;
    #[cfg(feature = "subdivisions")]
    impl Alpha2 {
        pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
            unimplemented!("No country feature is used");
        }
    }

    impl Alpha2 {
        pub fn to_alpha3(&self) -> Alpha3 {
            unimplemented!("No country feature is used");
        }
    }

    impl From<Alpha2> for Country {
        fn from(_alpha2: Alpha2) -> Self {
            unimplemented!("No country feature is used");
        }
    }
}