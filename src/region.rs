// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/region.rs`)

use crate::{SearchError, SearchedItems};
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all seven continents.
///
/// # Example
/// ```
/// use keshvar::{Continent, Alpha2};
///
/// let continent = Continent::try_from("australia").unwrap();
/// let country_list: Vec<_> = continent
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::NZ)); // New Zealand
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum Continent {
    /// * The Republic of Angola (Africa)
    /// * Burkina Faso (Africa)
    /// * The Republic of Burundi (Africa)
    /// * The Republic of Benin (Africa)
    /// * The Republic of Botswana (Africa)
    /// * The Democratic Republic of the Congo (Africa)
    /// * The Central African Republic (Africa)
    /// * The Republic of the Congo (Africa)
    /// * The Republic of Côte d'Ivoire (Africa)
    /// * The Republic of Cameroon (Africa)
    /// * The Republic of Cabo Verde (Africa)
    /// * The Republic of Djibouti (Africa)
    /// * The People's Democratic Republic of Algeria (Africa)
    /// * The Arab Republic of Egypt (Africa)
    /// * The Sahrawi Arab Democratic Republic (Africa)
    /// * The State of Eritrea (Africa)
    /// * The Federal Democratic Republic of Ethiopia (Africa)
    /// * The Gabonese Republic (Africa)
    /// * The Republic of Ghana (Africa)
    /// * The Republic of The Gambia (Africa)
    /// * The Republic of Guinea (Africa)
    /// * The Republic of Equatorial Guinea (Africa)
    /// * The Republic of Guinea-Bissau (Africa)
    /// * The Republic of Kenya (Africa)
    /// * The Union of the Comoros (Africa)
    /// * The Republic of Liberia (Africa)
    /// * The Kingdom of Lesotho (Africa)
    /// * The State of Libya (Africa)
    /// * The Kingdom of Morocco (Africa)
    /// * The Republic of Madagascar (Africa)
    /// * The Republic of Mali (Africa)
    /// * The Islamic Republic of Mauritania (Africa)
    /// * The Republic of Mauritius (Africa)
    /// * The Republic of Malawi (Africa)
    /// * The Republic of Mozambique (Africa)
    /// * The Republic of Namibia (Africa)
    /// * The Republic of the Niger (Africa)
    /// * The Federal Republic of Nigeria (Africa)
    /// * Réunion (Africa)
    /// * The Republic of Rwanda (Africa)
    /// * The Republic of Seychelles (Africa)
    /// * The Republic of the Sudan (Africa)
    /// * Saint Helena, Ascension and Tristan da Cunha (Africa)
    /// * The Republic of Sierra Leone (Africa)
    /// * The Republic of Senegal (Africa)
    /// * The Federal Republic of Somalia (Africa)
    /// * The Republic of South Sudan (Africa)
    /// * The Democratic Republic of São Tomé and Príncipe (Africa)
    /// * The Kingdom of Eswatini (Africa)
    /// * The Republic of Chad (Africa)
    /// * The French Southern and Antarctic Lands (Africa)
    /// * The Togolese Republic (Africa)
    /// * The Republic of Tunisia (Africa)
    /// * The United Republic of Tanzania (Africa)
    /// * The Republic of Uganda (Africa)
    /// * The Department of Mayotte (Africa)
    /// * The Republic of South Africa (Africa)
    /// * The Republic of Zambia (Africa)
    /// * The Republic of Zimbabwe (Africa)
    Africa,
    /// * Antarctica
    /// * Bouvet Island
    /// * South Georgia and the South Sandwich Islands (Americas)
    /// * The Territory of Heard Island and McDonald Islands
    Antarctica,
    /// * The United Arab Emirates (Asia)
    /// * The Islamic Republic of Afghanistan (Asia)
    /// * The Republic of Armenia (Asia)
    /// * The Republic of Azerbaijan (Asia)
    /// * The People's Republic of Bangladesh (Asia)
    /// * The Kingdom of Bahrain (Asia)
    /// * The Nation of Brunei, the Abode of Peace (Asia)
    /// * The Kingdom of Bhutan (Asia)
    /// * The Territory of Cocos (Keeling) Islands (Oceania)
    /// * The People's Republic of China (Asia)
    /// * The Territory of Christmas Island (Oceania)
    /// * The Republic of Cyprus (Asia)
    /// * Georgia (Asia)
    /// * The Hong Kong Special Administrative Region of China (Asia)
    /// * The Republic of Indonesia (Asia)
    /// * The State of Israel (Asia)
    /// * The Republic of India (Asia)
    /// * The British Indian Ocean Territory (Africa)
    /// * The Republic of Iraq (Asia)
    /// * The Islamic Republic of Iran (Asia)
    /// * The Hashemite Kingdom of Jordan (Asia)
    /// * Japan (Asia)
    /// * The Kyrgyz Republic (Asia)
    /// * The Kingdom of Cambodia (Asia)
    /// * The Democratic People's Republic of Korea (Asia)
    /// * The Republic of Korea (Asia)
    /// * The State of Kuwait (Asia)
    /// * The Republic of Kazakhstan (Asia)
    /// * The Lao People's Democratic Republic (Asia)
    /// * The Lebanese Republic (Asia)
    /// * The Democratic Socialist Republic of Sri Lanka (Asia)
    /// * The Republic of the Union of Myanmar (Asia)
    /// * Mongolia (Asia)
    /// * The Macao Special Administrative Region of China (Asia)
    /// * The Republic of Maldives (Asia)
    /// * Malaysia (Asia)
    /// * The Federal Democratic Republic of Nepal (Asia)
    /// * The Sultanate of Oman (Asia)
    /// * The Republic of the Philippines (Asia)
    /// * The Islamic Republic of Pakistan (Asia)
    /// * The State of Palestine (Asia)
    /// * The State of Qatar (Asia)
    /// * The Kingdom of Saudi Arabia (Asia)
    /// * The Republic of Singapore (Asia)
    /// * The Syrian Arab Republic (Asia)
    /// * The Kingdom of Thailand (Asia)
    /// * The Republic of Tajikistan (Asia)
    /// * The Democratic Republic of Timor-Leste (Asia)
    /// * Turkmenistan (Asia)
    /// * The Republic of China (Asia)
    /// * The Republic of Uzbekistan (Asia)
    /// * The Socialist Republic of Viet Nam (Asia)
    /// * The Republic of Yemen (Asia)
    Asia,
    /// * The Territory of American Samoa (Oceania)
    /// * The Commonwealth of Australia (Oceania)
    /// * The Cook Islands (Oceania)
    /// * The Republic of Fiji (Oceania)
    /// * The Federated States of Micronesia (Oceania)
    /// * The Territory of Guam (Oceania)
    /// * The Republic of Kiribati (Oceania)
    /// * The Republic of the Marshall Islands (Oceania)
    /// * The Commonwealth of the Northern Mariana Islands (Oceania)
    /// * New Caledonia (Oceania)
    /// * The Territory of Norfolk Island (Oceania)
    /// * The Republic of Nauru (Oceania)
    /// * Niue (Oceania)
    /// * New Zealand (Oceania)
    /// * French Polynesia (Oceania)
    /// * The Independent State of Papua New Guinea (Oceania)
    /// * The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    /// * The Republic of Palau (Oceania)
    /// * The Solomon Islands (Oceania)
    /// * Tokelau (Oceania)
    /// * The Kingdom of Tonga (Oceania)
    /// * Tuvalu (Oceania)
    /// * United States Minor Outlying Islands (Americas)
    /// * The Republic of Vanuatu (Oceania)
    /// * The Territory of the Wallis and Futuna Islands (Oceania)
    /// * The Independent State of Samoa (Oceania)
    Australia,
    /// * The Principality of Andorra (Europe)
    /// * The Republic of Albania (Europe)
    /// * The Republic of Austria (Europe)
    /// * Åland (Europe)
    /// * Bosnia and Herzegovina (Europe)
    /// * The Kingdom of Belgium (Europe)
    /// * The Republic of Bulgaria (Europe)
    /// * The Republic of Belarus (Europe)
    /// * The Swiss Confederation (Europe)
    /// * The Czech Republic (Europe)
    /// * The Federal Republic of Germany (Europe)
    /// * The Kingdom of Denmark (Europe)
    /// * The Republic of Estonia (Europe)
    /// * The Kingdom of Spain (Europe)
    /// * The Republic of Finland (Europe)
    /// * The Faroe Islands (Europe)
    /// * The French Republic (Europe)
    /// * The United Kingdom of Great Britain and Northern Ireland (Europe)
    /// * The Bailiwick of Guernsey (Europe)
    /// * Gibraltar (Europe)
    /// * The Hellenic Republic (Europe)
    /// * The Republic of Croatia (Europe)
    /// * Hungary (Europe)
    /// * Ireland (Europe)
    /// * The Isle of Man (Europe)
    /// * Iceland (Europe)
    /// * The Italian Republic (Europe)
    /// * The Bailiwick of Jersey (Europe)
    /// * The Principality of Liechtenstein (Europe)
    /// * The Republic of Lithuania (Europe)
    /// * The Grand Duchy of Luxembourg (Europe)
    /// * The Republic of Latvia (Europe)
    /// * The Principality of Monaco (Europe)
    /// * The Republic of Moldova (Europe)
    /// * Montenegro (Europe)
    /// * The Republic of North Macedonia (Europe)
    /// * The Republic of Malta (Europe)
    /// * The Kingdom of the Netherlands (Europe)
    /// * The Kingdom of Norway (Europe)
    /// * The Republic of Poland (Europe)
    /// * The Portuguese Republic (Europe)
    /// * Romania (Europe)
    /// * The Republic of Serbia (Europe)
    /// * The Russian Federation (Europe)
    /// * The Kingdom of Sweden (Europe)
    /// * The Republic of Slovenia (Europe)
    /// * Svalbard and Jan Mayen (Europe)
    /// * The Slovak Republic (Europe)
    /// * The Republic of San Marino (Europe)
    /// * The Republic of Turkey (Asia)
    /// * Ukraine (Europe)
    /// * The Holy See (Europe)
    Europe,
    /// * Antigua and Barbuda (Americas)
    /// * Anguilla (Americas)
    /// * Aruba (Americas)
    /// * Barbados (Americas)
    /// * The Collectivity of Saint-Barthélemy (Americas)
    /// * Bermuda (Americas)
    /// * Bonaire, Sint Eustatius and Saba (Americas)
    /// * The Commonwealth of The Bahamas (Americas)
    /// * Belize (Americas)
    /// * Canada (Americas)
    /// * The Republic of Costa Rica (Americas)
    /// * The Republic of Cuba (Americas)
    /// * The Country of Curaçao (Americas)
    /// * The Commonwealth of Dominica (Americas)
    /// * The Dominican Republic (Americas)
    /// * Grenada (Americas)
    /// * Kalaallit Nunaat (Americas)
    /// * Guadeloupe (Americas)
    /// * The Republic of Guatemala (Americas)
    /// * The Republic of Honduras (Americas)
    /// * The Republic of Haiti (Americas)
    /// * Jamaica (Americas)
    /// * Saint Kitts and Nevis (Americas)
    /// * The Cayman Islands (Americas)
    /// * Saint Lucia (Americas)
    /// * The Collectivity of Saint-Martin (Americas)
    /// * Martinique (Americas)
    /// * Montserrat (Americas)
    /// * The United Mexican States (Americas)
    /// * The Republic of Nicaragua (Americas)
    /// * The Republic of Panamá (Americas)
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    /// * The Commonwealth of Puerto Rico (Americas)
    /// * The Republic of El Salvador (Americas)
    /// * Sint Maarten (Americas)
    /// * The Turks and Caicos Islands (Americas)
    /// * The Republic of Trinidad and Tobago (Americas)
    /// * The United States of America (Americas)
    /// * Saint Vincent and the Grenadines (Americas)
    /// * The Virgin Islands (Americas)
    /// * The Virgin Islands of the United States (Americas)
    NorthAmerica,
    /// * The Argentine Republic (Americas)
    /// * The Plurinational State of Bolivia (Americas)
    /// * The Federative Republic of Brazil (Americas)
    /// * The Republic of Chile (Americas)
    /// * The Republic of Colombia (Americas)
    /// * The Republic of Ecuador (Americas)
    /// * The Falkland Islands (Americas)
    /// * Guyane (Americas)
    /// * The Co-operative Republic of Guyana (Americas)
    /// * The Republic of Perú (Americas)
    /// * The Republic of Paraguay (Americas)
    /// * The Republic of Suriname (Americas)
    /// * The Oriental Republic of Uruguay (Americas)
    /// * The Bolivarian Republic of Venezuela (Americas)
    SouthAmerica,
}

impl Continent {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::Africa => &[
                "AO", "BF", "BI", "BJ", "BW", "CD", "CF", "CG", "CI", "CM", "CV", "DJ", "DZ", "EG",
                "EH", "ER", "ET", "GA", "GH", "GM", "GN", "GQ", "GW", "KE", "KM", "LR", "LS", "LY",
                "MA", "MG", "ML", "MR", "MU", "MW", "MZ", "NA", "NE", "NG", "RE", "RW", "SC", "SD",
                "SH", "SL", "SN", "SO", "SS", "ST", "SZ", "TD", "TF", "TG", "TN", "TZ", "UG", "YT",
                "ZA", "ZM", "ZW",
            ],
            Self::Antarctica => &["AQ", "BV", "GS", "HM"],
            Self::Asia => &[
                "AE", "AF", "AM", "AZ", "BD", "BH", "BN", "BT", "CC", "CN", "CX", "CY", "GE", "HK",
                "ID", "IL", "IN", "IO", "IQ", "IR", "JO", "JP", "KG", "KH", "KP", "KR", "KW", "KZ",
                "LA", "LB", "LK", "MM", "MN", "MO", "MV", "MY", "NP", "OM", "PH", "PK", "PS", "QA",
                "SA", "SG", "SY", "TH", "TJ", "TL", "TM", "TW", "UZ", "VN", "YE",
            ],
            Self::Australia => &[
                "AS", "AU", "CK", "FJ", "FM", "GU", "KI", "MH", "MP", "NC", "NF", "NR", "NU", "NZ",
                "PF", "PG", "PN", "PW", "SB", "TK", "TO", "TV", "UM", "VU", "WF", "WS",
            ],
            Self::Europe => &[
                "AD", "AL", "AT", "AX", "BA", "BE", "BG", "BY", "CH", "CZ", "DE", "DK", "EE", "ES",
                "FI", "FO", "FR", "GB", "GG", "GI", "GR", "HR", "HU", "IE", "IM", "IS", "IT", "JE",
                "LI", "LT", "LU", "LV", "MC", "MD", "ME", "MK", "MT", "NL", "NO", "PL", "PT", "RO",
                "RS", "RU", "SE", "SI", "SJ", "SK", "SM", "TR", "UA", "VA",
            ],
            Self::NorthAmerica => &[
                "AG", "AI", "AW", "BB", "BL", "BM", "BQ", "BS", "BZ", "CA", "CR", "CU", "CW", "DM",
                "DO", "GD", "GL", "GP", "GT", "HN", "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MS",
                "MX", "NI", "PA", "PM", "PR", "SV", "SX", "TC", "TT", "US", "VC", "VG", "VI",
            ],
            Self::SouthAmerica => &[
                "AR", "BO", "BR", "CL", "CO", "EC", "FK", "GF", "GY", "PE", "PY", "SR", "UY", "VE",
            ],
        }
    }
}
impl TryFrom<&str> for Continent {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "africa" => Ok(Self::Africa),
            "antarctica" => Ok(Self::Antarctica),
            "asia" => Ok(Self::Asia),
            "australia" => Ok(Self::Australia),
            "europe" => Ok(Self::Europe),
            "north-america" => Ok(Self::NorthAmerica),
            "south-america" => Ok(Self::SouthAmerica),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllContinents,
            }),
        }
    }
}
impl ToString for Continent {
    fn to_string(&self) -> String {
        match self {
            Self::Africa => "africa",
            Self::Antarctica => "antarctica",
            Self::Asia => "asia",
            Self::Australia => "australia",
            Self::Europe => "europe",
            Self::NorthAmerica => "north-america",
            Self::SouthAmerica => "south-america",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all six regions.
///
/// # Example
/// ```
/// use keshvar::{Region, Alpha2};
///
/// let region = Region::try_from("asia").unwrap();
/// let country_list: Vec<_> = region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::TR)); // Turkey
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum Region {
    /// * The Republic of Angola
    /// * Burkina Faso
    /// * The Republic of Burundi
    /// * The Republic of Benin
    /// * The Republic of Botswana
    /// * The Democratic Republic of the Congo
    /// * The Central African Republic
    /// * The Republic of the Congo
    /// * The Republic of Côte d'Ivoire
    /// * The Republic of Cameroon
    /// * The Republic of Cabo Verde
    /// * The Republic of Djibouti
    /// * The People's Democratic Republic of Algeria
    /// * The Arab Republic of Egypt
    /// * The Sahrawi Arab Democratic Republic
    /// * The State of Eritrea
    /// * The Federal Democratic Republic of Ethiopia
    /// * The Gabonese Republic
    /// * The Republic of Ghana
    /// * The Republic of The Gambia
    /// * The Republic of Guinea
    /// * The Republic of Equatorial Guinea
    /// * The Republic of Guinea-Bissau
    /// * The British Indian Ocean Territory
    /// * The Republic of Kenya
    /// * The Union of the Comoros
    /// * The Republic of Liberia
    /// * The Kingdom of Lesotho
    /// * The State of Libya
    /// * The Kingdom of Morocco
    /// * The Republic of Madagascar
    /// * The Republic of Mali
    /// * The Islamic Republic of Mauritania
    /// * The Republic of Mauritius
    /// * The Republic of Malawi
    /// * The Republic of Mozambique
    /// * The Republic of Namibia
    /// * The Republic of the Niger
    /// * The Federal Republic of Nigeria
    /// * Réunion
    /// * The Republic of Rwanda
    /// * The Republic of Seychelles
    /// * The Republic of the Sudan
    /// * Saint Helena, Ascension and Tristan da Cunha
    /// * The Republic of Sierra Leone
    /// * The Republic of Senegal
    /// * The Federal Republic of Somalia
    /// * The Republic of South Sudan
    /// * The Democratic Republic of São Tomé and Príncipe
    /// * The Kingdom of Eswatini
    /// * The Republic of Chad
    /// * The French Southern and Antarctic Lands
    /// * The Togolese Republic
    /// * The Republic of Tunisia
    /// * The United Republic of Tanzania
    /// * The Republic of Uganda
    /// * The Department of Mayotte
    /// * The Republic of South Africa
    /// * The Republic of Zambia
    /// * The Republic of Zimbabwe
    Africa,
    /// * Antigua and Barbuda
    /// * Anguilla
    /// * The Argentine Republic
    /// * Aruba
    /// * Barbados
    /// * The Collectivity of Saint-Barthélemy
    /// * Bermuda
    /// * The Plurinational State of Bolivia
    /// * Bonaire, Sint Eustatius and Saba
    /// * The Federative Republic of Brazil
    /// * The Commonwealth of The Bahamas
    /// * Belize
    /// * Canada
    /// * The Republic of Chile
    /// * The Republic of Colombia
    /// * The Republic of Costa Rica
    /// * The Republic of Cuba
    /// * The Country of Curaçao
    /// * The Commonwealth of Dominica
    /// * The Dominican Republic
    /// * The Republic of Ecuador
    /// * The Falkland Islands
    /// * Grenada
    /// * Guyane
    /// * Kalaallit Nunaat
    /// * Guadeloupe
    /// * South Georgia and the South Sandwich Islands
    /// * The Republic of Guatemala
    /// * The Co-operative Republic of Guyana
    /// * The Republic of Honduras
    /// * The Republic of Haiti
    /// * Jamaica
    /// * Saint Kitts and Nevis
    /// * The Cayman Islands
    /// * Saint Lucia
    /// * The Collectivity of Saint-Martin
    /// * Martinique
    /// * Montserrat
    /// * The United Mexican States
    /// * The Republic of Nicaragua
    /// * The Republic of Panamá
    /// * The Republic of Perú
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon
    /// * The Commonwealth of Puerto Rico
    /// * The Republic of Paraguay
    /// * The Republic of Suriname
    /// * The Republic of El Salvador
    /// * Sint Maarten
    /// * The Turks and Caicos Islands
    /// * The Republic of Trinidad and Tobago
    /// * United States Minor Outlying Islands
    /// * The United States of America
    /// * The Oriental Republic of Uruguay
    /// * Saint Vincent and the Grenadines
    /// * The Bolivarian Republic of Venezuela
    /// * The Virgin Islands
    /// * The Virgin Islands of the United States
    Americas,
    Antarctica,
    /// * The United Arab Emirates
    /// * The Islamic Republic of Afghanistan
    /// * The Republic of Armenia
    /// * The Republic of Azerbaijan
    /// * The People's Republic of Bangladesh
    /// * The Kingdom of Bahrain
    /// * The Nation of Brunei, the Abode of Peace
    /// * The Kingdom of Bhutan
    /// * The People's Republic of China
    /// * The Republic of Cyprus
    /// * Georgia
    /// * The Hong Kong Special Administrative Region of China
    /// * The Republic of Indonesia
    /// * The State of Israel
    /// * The Republic of India
    /// * The Republic of Iraq
    /// * The Islamic Republic of Iran
    /// * The Hashemite Kingdom of Jordan
    /// * Japan
    /// * The Kyrgyz Republic
    /// * The Kingdom of Cambodia
    /// * The Democratic People's Republic of Korea
    /// * The Republic of Korea
    /// * The State of Kuwait
    /// * The Republic of Kazakhstan
    /// * The Lao People's Democratic Republic
    /// * The Lebanese Republic
    /// * The Democratic Socialist Republic of Sri Lanka
    /// * The Republic of the Union of Myanmar
    /// * Mongolia
    /// * The Macao Special Administrative Region of China
    /// * The Republic of Maldives
    /// * Malaysia
    /// * The Federal Democratic Republic of Nepal
    /// * The Sultanate of Oman
    /// * The Republic of the Philippines
    /// * The Islamic Republic of Pakistan
    /// * The State of Palestine
    /// * The State of Qatar
    /// * The Kingdom of Saudi Arabia
    /// * The Republic of Singapore
    /// * The Syrian Arab Republic
    /// * The Kingdom of Thailand
    /// * The Republic of Tajikistan
    /// * The Democratic Republic of Timor-Leste
    /// * Turkmenistan
    /// * The Republic of Turkey
    /// * The Republic of China
    /// * The Republic of Uzbekistan
    /// * The Socialist Republic of Viet Nam
    /// * The Republic of Yemen
    Asia,
    /// * The Principality of Andorra
    /// * The Republic of Albania
    /// * The Republic of Austria
    /// * Åland
    /// * Bosnia and Herzegovina
    /// * The Kingdom of Belgium
    /// * The Republic of Bulgaria
    /// * The Republic of Belarus
    /// * The Swiss Confederation
    /// * The Czech Republic
    /// * The Federal Republic of Germany
    /// * The Kingdom of Denmark
    /// * The Republic of Estonia
    /// * The Kingdom of Spain
    /// * The Republic of Finland
    /// * The Faroe Islands
    /// * The French Republic
    /// * The United Kingdom of Great Britain and Northern Ireland
    /// * The Bailiwick of Guernsey
    /// * Gibraltar
    /// * The Hellenic Republic
    /// * The Republic of Croatia
    /// * Hungary
    /// * Ireland
    /// * The Isle of Man
    /// * Iceland
    /// * The Italian Republic
    /// * The Bailiwick of Jersey
    /// * The Principality of Liechtenstein
    /// * The Republic of Lithuania
    /// * The Grand Duchy of Luxembourg
    /// * The Republic of Latvia
    /// * The Principality of Monaco
    /// * The Republic of Moldova
    /// * Montenegro
    /// * The Republic of North Macedonia
    /// * The Republic of Malta
    /// * The Kingdom of the Netherlands
    /// * The Kingdom of Norway
    /// * The Republic of Poland
    /// * The Portuguese Republic
    /// * Romania
    /// * The Republic of Serbia
    /// * The Russian Federation
    /// * The Kingdom of Sweden
    /// * The Republic of Slovenia
    /// * Svalbard and Jan Mayen
    /// * The Slovak Republic
    /// * The Republic of San Marino
    /// * Ukraine
    /// * The Holy See
    Europe,
    /// * The Territory of American Samoa
    /// * The Commonwealth of Australia
    /// * The Territory of Cocos (Keeling) Islands
    /// * The Cook Islands
    /// * The Territory of Christmas Island
    /// * The Republic of Fiji
    /// * The Federated States of Micronesia
    /// * The Territory of Guam
    /// * The Republic of Kiribati
    /// * The Republic of the Marshall Islands
    /// * The Commonwealth of the Northern Mariana Islands
    /// * New Caledonia
    /// * The Territory of Norfolk Island
    /// * The Republic of Nauru
    /// * Niue
    /// * New Zealand
    /// * French Polynesia
    /// * The Independent State of Papua New Guinea
    /// * The Pitcairn, Henderson, Ducie and Oeno Islands
    /// * The Republic of Palau
    /// * The Solomon Islands
    /// * Tokelau
    /// * The Kingdom of Tonga
    /// * Tuvalu
    /// * The Republic of Vanuatu
    /// * The Territory of the Wallis and Futuna Islands
    /// * The Independent State of Samoa
    Oceania,
}

impl Region {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::Africa => &[
                "AO", "BF", "BI", "BJ", "BW", "CD", "CF", "CG", "CI", "CM", "CV", "DJ", "DZ", "EG",
                "EH", "ER", "ET", "GA", "GH", "GM", "GN", "GQ", "GW", "IO", "KE", "KM", "LR", "LS",
                "LY", "MA", "MG", "ML", "MR", "MU", "MW", "MZ", "NA", "NE", "NG", "RE", "RW", "SC",
                "SD", "SH", "SL", "SN", "SO", "SS", "ST", "SZ", "TD", "TF", "TG", "TN", "TZ", "UG",
                "YT", "ZA", "ZM", "ZW",
            ],
            Self::Americas => &[
                "AG", "AI", "AR", "AW", "BB", "BL", "BM", "BO", "BQ", "BR", "BS", "BZ", "CA", "CL",
                "CO", "CR", "CU", "CW", "DM", "DO", "EC", "FK", "GD", "GF", "GL", "GP", "GS", "GT",
                "GY", "HN", "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MS", "MX", "NI", "PA", "PE",
                "PM", "PR", "PY", "SR", "SV", "SX", "TC", "TT", "UM", "US", "UY", "VC", "VE", "VG",
                "VI",
            ],
            Self::Antarctica => &[],
            Self::Asia => &[
                "AE", "AF", "AM", "AZ", "BD", "BH", "BN", "BT", "CN", "CY", "GE", "HK", "ID", "IL",
                "IN", "IQ", "IR", "JO", "JP", "KG", "KH", "KP", "KR", "KW", "KZ", "LA", "LB", "LK",
                "MM", "MN", "MO", "MV", "MY", "NP", "OM", "PH", "PK", "PS", "QA", "SA", "SG", "SY",
                "TH", "TJ", "TL", "TM", "TR", "TW", "UZ", "VN", "YE",
            ],
            Self::Europe => &[
                "AD", "AL", "AT", "AX", "BA", "BE", "BG", "BY", "CH", "CZ", "DE", "DK", "EE", "ES",
                "FI", "FO", "FR", "GB", "GG", "GI", "GR", "HR", "HU", "IE", "IM", "IS", "IT", "JE",
                "LI", "LT", "LU", "LV", "MC", "MD", "ME", "MK", "MT", "NL", "NO", "PL", "PT", "RO",
                "RS", "RU", "SE", "SI", "SJ", "SK", "SM", "UA", "VA",
            ],
            Self::Oceania => &[
                "AS", "AU", "CC", "CK", "CX", "FJ", "FM", "GU", "KI", "MH", "MP", "NC", "NF", "NR",
                "NU", "NZ", "PF", "PG", "PN", "PW", "SB", "TK", "TO", "TV", "VU", "WF", "WS",
            ],
        }
    }
}
impl TryFrom<&str> for Region {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "africa" => Ok(Self::Africa),
            "americas" => Ok(Self::Americas),
            "antarctica" => Ok(Self::Antarctica),
            "asia" => Ok(Self::Asia),
            "europe" => Ok(Self::Europe),
            "oceania" => Ok(Self::Oceania),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllRegions,
            }),
        }
    }
}
impl ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Self::Africa => "africa",
            Self::Americas => "americas",
            Self::Antarctica => "antarctica",
            Self::Asia => "asia",
            Self::Europe => "europe",
            Self::Oceania => "oceania",
        }
        .to_string()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing sub-regions.
///
/// # Example
/// ```
/// use keshvar::{SubRegion, Alpha2};
///
/// let sub_region = SubRegion::try_from("northern-america").unwrap();
/// let country_list: Vec<_> = sub_region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::CA)); // Canada
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum SubRegion {
    /// * The Commonwealth of Australia
    /// * The Territory of Cocos (Keeling) Islands
    /// * The Territory of Christmas Island
    /// * The Territory of Norfolk Island
    /// * New Zealand
    AustraliaAndNewZealand,
    /// * Antigua and Barbuda
    /// * Anguilla
    /// * Aruba
    /// * Barbados
    /// * The Collectivity of Saint-Barthélemy
    /// * Bonaire, Sint Eustatius and Saba
    /// * The Commonwealth of The Bahamas
    /// * The Republic of Cuba
    /// * The Country of Curaçao
    /// * The Commonwealth of Dominica
    /// * The Dominican Republic
    /// * Grenada
    /// * Guadeloupe
    /// * The Republic of Haiti
    /// * Jamaica
    /// * Saint Kitts and Nevis
    /// * The Cayman Islands
    /// * Saint Lucia
    /// * The Collectivity of Saint-Martin
    /// * Martinique
    /// * Montserrat
    /// * The Commonwealth of Puerto Rico
    /// * Sint Maarten
    /// * The Turks and Caicos Islands
    /// * The Republic of Trinidad and Tobago
    /// * Saint Vincent and the Grenadines
    /// * The Virgin Islands
    /// * The Virgin Islands of the United States
    Caribbean,
    /// * Belize
    /// * The Republic of Costa Rica
    /// * The Republic of Guatemala
    /// * The Republic of Honduras
    /// * The United Mexican States
    /// * The Republic of Nicaragua
    /// * The Republic of Panamá
    /// * The Republic of El Salvador
    CentralAmerica,
    /// * The Kyrgyz Republic
    /// * The Republic of Kazakhstan
    /// * The Republic of Tajikistan
    /// * Turkmenistan
    /// * The Republic of Uzbekistan
    CentralAsia,
    /// * The Republic of Burundi
    /// * The Republic of Djibouti
    /// * The State of Eritrea
    /// * The Federal Democratic Republic of Ethiopia
    /// * The British Indian Ocean Territory
    /// * The Republic of Kenya
    /// * The Union of the Comoros
    /// * The Republic of Madagascar
    /// * The Republic of Mauritius
    /// * The Republic of Malawi
    /// * The Republic of Mozambique
    /// * Réunion
    /// * The Republic of Rwanda
    /// * The Republic of Seychelles
    /// * The Federal Republic of Somalia
    /// * The French Southern and Antarctic Lands
    /// * The United Republic of Tanzania
    /// * The Republic of Uganda
    /// * The Department of Mayotte
    /// * The Republic of Zambia
    /// * The Republic of Zimbabwe
    EasternAfrica,
    /// * The People's Republic of China
    /// * The Hong Kong Special Administrative Region of China
    /// * Japan
    /// * The Democratic People's Republic of Korea
    /// * The Republic of Korea
    /// * Mongolia
    /// * The Macao Special Administrative Region of China
    /// * The Republic of China
    EasternAsia,
    /// * The Republic of Bulgaria
    /// * The Republic of Belarus
    /// * The Czech Republic
    /// * Hungary
    /// * The Republic of Moldova
    /// * The Republic of Poland
    /// * Romania
    /// * The Russian Federation
    /// * The Slovak Republic
    /// * Ukraine
    EasternEurope,
    /// * The Republic of Fiji
    /// * New Caledonia
    /// * The Independent State of Papua New Guinea
    /// * The Solomon Islands
    /// * The Republic of Vanuatu
    Melanesia,
    /// * The Federated States of Micronesia
    /// * The Territory of Guam
    /// * The Republic of Kiribati
    /// * The Republic of the Marshall Islands
    /// * The Commonwealth of the Northern Mariana Islands
    /// * The Republic of Nauru
    /// * The Republic of Palau
    Micronesia,
    /// * The Republic of Angola
    /// * The Democratic Republic of the Congo
    /// * The Central African Republic
    /// * The Republic of the Congo
    /// * The Republic of Cameroon
    /// * The Gabonese Republic
    /// * The Republic of Equatorial Guinea
    /// * The Democratic Republic of São Tomé and Príncipe
    /// * The Republic of Chad
    MiddleAfrica,
    /// * The People's Democratic Republic of Algeria
    /// * The Arab Republic of Egypt
    /// * The Sahrawi Arab Democratic Republic
    /// * The State of Libya
    /// * The Kingdom of Morocco
    /// * The Republic of the Sudan
    /// * The Republic of South Sudan
    /// * The Republic of Tunisia
    NorthernAfrica,
    /// * Bermuda
    /// * Canada
    /// * Kalaallit Nunaat
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon
    /// * United States Minor Outlying Islands
    /// * The United States of America
    NorthernAmerica,
    /// * Åland
    /// * The Kingdom of Denmark
    /// * The Republic of Estonia
    /// * The Republic of Finland
    /// * The Faroe Islands
    /// * The United Kingdom of Great Britain and Northern Ireland
    /// * The Bailiwick of Guernsey
    /// * Ireland
    /// * The Isle of Man
    /// * Iceland
    /// * The Bailiwick of Jersey
    /// * The Republic of Lithuania
    /// * The Republic of Latvia
    /// * The Kingdom of Norway
    /// * The Kingdom of Sweden
    /// * Svalbard and Jan Mayen
    NorthernEurope,
    /// * The Territory of American Samoa
    /// * The Cook Islands
    /// * Niue
    /// * French Polynesia
    /// * The Pitcairn, Henderson, Ducie and Oeno Islands
    /// * Tokelau
    /// * The Kingdom of Tonga
    /// * Tuvalu
    /// * The Territory of the Wallis and Futuna Islands
    /// * The Independent State of Samoa
    Polynesia,
    /// * The Argentine Republic
    /// * The Plurinational State of Bolivia
    /// * The Federative Republic of Brazil
    /// * The Republic of Chile
    /// * The Republic of Colombia
    /// * The Republic of Ecuador
    /// * The Falkland Islands
    /// * Guyane
    /// * South Georgia and the South Sandwich Islands
    /// * The Co-operative Republic of Guyana
    /// * The Republic of Perú
    /// * The Republic of Paraguay
    /// * The Republic of Suriname
    /// * The Oriental Republic of Uruguay
    /// * The Bolivarian Republic of Venezuela
    SouthAmerica,
    /// * The Nation of Brunei, the Abode of Peace
    /// * The Republic of Indonesia
    /// * The Kingdom of Cambodia
    /// * The Lao People's Democratic Republic
    /// * The Republic of the Union of Myanmar
    /// * Malaysia
    /// * The Republic of the Philippines
    /// * The Republic of Singapore
    /// * The Kingdom of Thailand
    /// * The Democratic Republic of Timor-Leste
    /// * The Socialist Republic of Viet Nam
    SouthEasternAsia,
    /// * The Republic of Botswana
    /// * The Kingdom of Lesotho
    /// * The Republic of Namibia
    /// * The Kingdom of Eswatini
    /// * The Republic of South Africa
    SouthernAfrica,
    /// * The Islamic Republic of Afghanistan
    /// * The People's Republic of Bangladesh
    /// * The Kingdom of Bhutan
    /// * The Republic of India
    /// * The Islamic Republic of Iran
    /// * The Democratic Socialist Republic of Sri Lanka
    /// * The Republic of Maldives
    /// * The Federal Democratic Republic of Nepal
    /// * The Islamic Republic of Pakistan
    SouthernAsia,
    /// * The Principality of Andorra
    /// * The Republic of Albania
    /// * Bosnia and Herzegovina
    /// * The Kingdom of Spain
    /// * Gibraltar
    /// * The Hellenic Republic
    /// * The Republic of Croatia
    /// * The Italian Republic
    /// * Montenegro
    /// * The Republic of North Macedonia
    /// * The Republic of Malta
    /// * The Portuguese Republic
    /// * The Republic of Serbia
    /// * The Republic of Slovenia
    /// * The Republic of San Marino
    /// * The Holy See
    SouthernEurope,
    /// * Burkina Faso
    /// * The Republic of Benin
    /// * The Republic of Côte d'Ivoire
    /// * The Republic of Cabo Verde
    /// * The Republic of Ghana
    /// * The Republic of The Gambia
    /// * The Republic of Guinea
    /// * The Republic of Guinea-Bissau
    /// * The Republic of Liberia
    /// * The Republic of Mali
    /// * The Islamic Republic of Mauritania
    /// * The Republic of the Niger
    /// * The Federal Republic of Nigeria
    /// * Saint Helena, Ascension and Tristan da Cunha
    /// * The Republic of Sierra Leone
    /// * The Republic of Senegal
    /// * The Togolese Republic
    WesternAfrica,
    /// * The United Arab Emirates
    /// * The Republic of Armenia
    /// * The Republic of Azerbaijan
    /// * The Kingdom of Bahrain
    /// * The Republic of Cyprus
    /// * Georgia
    /// * The State of Israel
    /// * The Republic of Iraq
    /// * The Hashemite Kingdom of Jordan
    /// * The State of Kuwait
    /// * The Lebanese Republic
    /// * The Sultanate of Oman
    /// * The State of Palestine
    /// * The State of Qatar
    /// * The Kingdom of Saudi Arabia
    /// * The Syrian Arab Republic
    /// * The Republic of Turkey
    /// * The Republic of Yemen
    WesternAsia,
    /// * The Republic of Austria
    /// * The Kingdom of Belgium
    /// * The Swiss Confederation
    /// * The Federal Republic of Germany
    /// * The French Republic
    /// * The Principality of Liechtenstein
    /// * The Grand Duchy of Luxembourg
    /// * The Principality of Monaco
    /// * The Kingdom of the Netherlands
    WesternEurope,
}

impl SubRegion {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::AustraliaAndNewZealand => &["AU", "CC", "CX", "NF", "NZ"],
            Self::Caribbean => &[
                "AG", "AI", "AW", "BB", "BL", "BQ", "BS", "CU", "CW", "DM", "DO", "GD", "GP", "HT",
                "JM", "KN", "KY", "LC", "MF", "MQ", "MS", "PR", "SX", "TC", "TT", "VC", "VG", "VI",
            ],
            Self::CentralAmerica => &["BZ", "CR", "GT", "HN", "MX", "NI", "PA", "SV"],
            Self::CentralAsia => &["KG", "KZ", "TJ", "TM", "UZ"],
            Self::EasternAfrica => &[
                "BI", "DJ", "ER", "ET", "IO", "KE", "KM", "MG", "MU", "MW", "MZ", "RE", "RW", "SC",
                "SO", "TF", "TZ", "UG", "YT", "ZM", "ZW",
            ],
            Self::EasternAsia => &["CN", "HK", "JP", "KP", "KR", "MN", "MO", "TW"],
            Self::EasternEurope => &["BG", "BY", "CZ", "HU", "MD", "PL", "RO", "RU", "SK", "UA"],
            Self::Melanesia => &["FJ", "NC", "PG", "SB", "VU"],
            Self::Micronesia => &["FM", "GU", "KI", "MH", "MP", "NR", "PW"],
            Self::MiddleAfrica => &["AO", "CD", "CF", "CG", "CM", "GA", "GQ", "ST", "TD"],
            Self::NorthernAfrica => &["DZ", "EG", "EH", "LY", "MA", "SD", "SS", "TN"],
            Self::NorthernAmerica => &["BM", "CA", "GL", "PM", "UM", "US"],
            Self::NorthernEurope => &[
                "AX", "DK", "EE", "FI", "FO", "GB", "GG", "IE", "IM", "IS", "JE", "LT", "LV", "NO",
                "SE", "SJ",
            ],
            Self::Polynesia => &["AS", "CK", "NU", "PF", "PN", "TK", "TO", "TV", "WF", "WS"],
            Self::SouthAmerica => &[
                "AR", "BO", "BR", "CL", "CO", "EC", "FK", "GF", "GS", "GY", "PE", "PY", "SR", "UY",
                "VE",
            ],
            Self::SouthEasternAsia => &[
                "BN", "ID", "KH", "LA", "MM", "MY", "PH", "SG", "TH", "TL", "VN",
            ],
            Self::SouthernAfrica => &["BW", "LS", "NA", "SZ", "ZA"],
            Self::SouthernAsia => &["AF", "BD", "BT", "IN", "IR", "LK", "MV", "NP", "PK"],
            Self::SouthernEurope => &[
                "AD", "AL", "BA", "ES", "GI", "GR", "HR", "IT", "ME", "MK", "MT", "PT", "RS", "SI",
                "SM", "VA",
            ],
            Self::WesternAfrica => &[
                "BF", "BJ", "CI", "CV", "GH", "GM", "GN", "GW", "LR", "ML", "MR", "NE", "NG", "SH",
                "SL", "SN", "TG",
            ],
            Self::WesternAsia => &[
                "AE", "AM", "AZ", "BH", "CY", "GE", "IL", "IQ", "JO", "KW", "LB", "OM", "PS", "QA",
                "SA", "SY", "TR", "YE",
            ],
            Self::WesternEurope => &["AT", "BE", "CH", "DE", "FR", "LI", "LU", "MC", "NL"],
        }
    }
}
impl TryFrom<&str> for SubRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "australia-and-new-zealand" => Ok(Self::AustraliaAndNewZealand),
            "caribbean" => Ok(Self::Caribbean),
            "central-america" => Ok(Self::CentralAmerica),
            "central-asia" => Ok(Self::CentralAsia),
            "eastern-africa" => Ok(Self::EasternAfrica),
            "eastern-asia" => Ok(Self::EasternAsia),
            "eastern-europe" => Ok(Self::EasternEurope),
            "melanesia" => Ok(Self::Melanesia),
            "micronesia" => Ok(Self::Micronesia),
            "middle-africa" => Ok(Self::MiddleAfrica),
            "northern-africa" => Ok(Self::NorthernAfrica),
            "northern-america" => Ok(Self::NorthernAmerica),
            "northern-europe" => Ok(Self::NorthernEurope),
            "polynesia" => Ok(Self::Polynesia),
            "south-america" => Ok(Self::SouthAmerica),
            "south-eastern-asia" => Ok(Self::SouthEasternAsia),
            "southern-africa" => Ok(Self::SouthernAfrica),
            "southern-asia" => Ok(Self::SouthernAsia),
            "southern-europe" => Ok(Self::SouthernEurope),
            "western-africa" => Ok(Self::WesternAfrica),
            "western-asia" => Ok(Self::WesternAsia),
            "western-europe" => Ok(Self::WesternEurope),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllSubRegions,
            }),
        }
    }
}
impl ToString for SubRegion {
    fn to_string(&self) -> String {
        match self {
            Self::AustraliaAndNewZealand => "australia-and-new-zealand",
            Self::Caribbean => "caribbean",
            Self::CentralAmerica => "central-america",
            Self::CentralAsia => "central-asia",
            Self::EasternAfrica => "eastern-africa",
            Self::EasternAsia => "eastern-asia",
            Self::EasternEurope => "eastern-europe",
            Self::Melanesia => "melanesia",
            Self::Micronesia => "micronesia",
            Self::MiddleAfrica => "middle-africa",
            Self::NorthernAfrica => "northern-africa",
            Self::NorthernAmerica => "northern-america",
            Self::NorthernEurope => "northern-europe",
            Self::Polynesia => "polynesia",
            Self::SouthAmerica => "south-america",
            Self::SouthEasternAsia => "south-eastern-asia",
            Self::SouthernAfrica => "southern-africa",
            Self::SouthernAsia => "southern-asia",
            Self::SouthernEurope => "southern-europe",
            Self::WesternAfrica => "western-africa",
            Self::WesternAsia => "western-asia",
            Self::WesternEurope => "western-europe",
        }
        .to_string()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all four world regions.
///
/// # Example
/// ```
/// use keshvar::{WorldRegion, Alpha2};
///
/// let world_region = WorldRegion::try_from("AMER").unwrap();
/// let country_list: Vec<_> = world_region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::JM)); // Jamaica
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum WorldRegion {
    /// * Antigua and Barbuda (Americas)
    /// * Anguilla (Americas)
    /// * Antarctica
    /// * The Argentine Republic (Americas)
    /// * Aruba (Americas)
    /// * Barbados (Americas)
    /// * Bermuda (Americas)
    /// * The Plurinational State of Bolivia (Americas)
    /// * The Federative Republic of Brazil (Americas)
    /// * The Commonwealth of The Bahamas (Americas)
    /// * Belize (Americas)
    /// * Canada (Americas)
    /// * The Republic of Chile (Americas)
    /// * The Republic of Colombia (Americas)
    /// * The Republic of Costa Rica (Americas)
    /// * The Republic of Cuba (Americas)
    /// * The Country of Curaçao (Americas)
    /// * The Commonwealth of Dominica (Americas)
    /// * The Dominican Republic (Americas)
    /// * The Republic of Ecuador (Americas)
    /// * The Falkland Islands (Americas)
    /// * Grenada (Americas)
    /// * Guyane (Americas)
    /// * Guadeloupe (Americas)
    /// * South Georgia and the South Sandwich Islands (Americas)
    /// * The Republic of Guatemala (Americas)
    /// * The Co-operative Republic of Guyana (Americas)
    /// * The Republic of Honduras (Americas)
    /// * The Republic of Haiti (Americas)
    /// * Jamaica (Americas)
    /// * Saint Kitts and Nevis (Americas)
    /// * The Cayman Islands (Americas)
    /// * Saint Lucia (Americas)
    /// * The Collectivity of Saint-Martin (Americas)
    /// * Martinique (Americas)
    /// * The United Mexican States (Americas)
    /// * The Republic of Nicaragua (Americas)
    /// * The Republic of Panamá (Americas)
    /// * The Republic of Perú (Americas)
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    /// * The Commonwealth of Puerto Rico (Americas)
    /// * The Republic of Paraguay (Americas)
    /// * The Republic of Suriname (Americas)
    /// * The Republic of El Salvador (Americas)
    /// * Sint Maarten (Americas)
    /// * The Republic of Trinidad and Tobago (Americas)
    /// * United States Minor Outlying Islands (Americas)
    /// * The United States of America (Americas)
    /// * The Oriental Republic of Uruguay (Americas)
    /// * Saint Vincent and the Grenadines (Americas)
    /// * The Bolivarian Republic of Venezuela (Americas)
    /// * The Virgin Islands (Americas)
    /// * The Virgin Islands of the United States (Americas)
    AMER,
    /// * The Islamic Republic of Afghanistan (Asia)
    /// * The Territory of American Samoa (Oceania)
    /// * The Commonwealth of Australia (Oceania)
    /// * The People's Republic of Bangladesh (Asia)
    /// * The Collectivity of Saint-Barthélemy (Americas)
    /// * The Nation of Brunei, the Abode of Peace (Asia)
    /// * Bonaire, Sint Eustatius and Saba (Americas)
    /// * The Kingdom of Bhutan (Asia)
    /// * Bouvet Island
    /// * The Territory of Cocos (Keeling) Islands (Oceania)
    /// * The Cook Islands (Oceania)
    /// * The People's Republic of China (Asia)
    /// * The Territory of Christmas Island (Oceania)
    /// * The Republic of Fiji (Oceania)
    /// * The Federated States of Micronesia (Oceania)
    /// * The Territory of Guam (Oceania)
    /// * The Hong Kong Special Administrative Region of China (Asia)
    /// * The Territory of Heard Island and McDonald Islands
    /// * The Republic of Indonesia (Asia)
    /// * The Republic of India (Asia)
    /// * The British Indian Ocean Territory (Africa)
    /// * Japan (Asia)
    /// * The Kingdom of Cambodia (Asia)
    /// * The Republic of Kiribati (Oceania)
    /// * The Democratic People's Republic of Korea (Asia)
    /// * The Republic of Korea (Asia)
    /// * The Lao People's Democratic Republic (Asia)
    /// * The Democratic Socialist Republic of Sri Lanka (Asia)
    /// * The Republic of the Marshall Islands (Oceania)
    /// * The Republic of the Union of Myanmar (Asia)
    /// * Mongolia (Asia)
    /// * The Macao Special Administrative Region of China (Asia)
    /// * The Commonwealth of the Northern Mariana Islands (Oceania)
    /// * The Republic of Maldives (Asia)
    /// * Malaysia (Asia)
    /// * New Caledonia (Oceania)
    /// * The Territory of Norfolk Island (Oceania)
    /// * The Federal Democratic Republic of Nepal (Asia)
    /// * The Republic of Nauru (Oceania)
    /// * Niue (Oceania)
    /// * New Zealand (Oceania)
    /// * French Polynesia (Oceania)
    /// * The Independent State of Papua New Guinea (Oceania)
    /// * The Republic of the Philippines (Asia)
    /// * The Islamic Republic of Pakistan (Asia)
    /// * The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    /// * The Republic of Palau (Oceania)
    /// * The Solomon Islands (Oceania)
    /// * The Republic of Singapore (Asia)
    /// * Saint Helena, Ascension and Tristan da Cunha (Africa)
    /// * The Turks and Caicos Islands (Americas)
    /// * The Kingdom of Thailand (Asia)
    /// * Tokelau (Oceania)
    /// * The Democratic Republic of Timor-Leste (Asia)
    /// * The Kingdom of Tonga (Oceania)
    /// * Tuvalu (Oceania)
    /// * The Republic of China (Asia)
    /// * The Socialist Republic of Viet Nam (Asia)
    /// * The Republic of Vanuatu (Oceania)
    /// * The Territory of the Wallis and Futuna Islands (Oceania)
    /// * The Independent State of Samoa (Oceania)
    APAC,
    /// * The Principality of Andorra (Europe)
    /// * The United Arab Emirates (Asia)
    /// * The Republic of Albania (Europe)
    /// * The Republic of Armenia (Asia)
    /// * The Republic of Angola (Africa)
    /// * The Republic of Austria (Europe)
    /// * Åland (Europe)
    /// * The Republic of Azerbaijan (Asia)
    /// * Bosnia and Herzegovina (Europe)
    /// * The Kingdom of Belgium (Europe)
    /// * Burkina Faso (Africa)
    /// * The Republic of Bulgaria (Europe)
    /// * The Kingdom of Bahrain (Asia)
    /// * The Republic of Burundi (Africa)
    /// * The Republic of Benin (Africa)
    /// * The Republic of Botswana (Africa)
    /// * The Republic of Belarus (Europe)
    /// * The Democratic Republic of the Congo (Africa)
    /// * The Central African Republic (Africa)
    /// * The Republic of the Congo (Africa)
    /// * The Swiss Confederation (Europe)
    /// * The Republic of Côte d'Ivoire (Africa)
    /// * The Republic of Cameroon (Africa)
    /// * The Republic of Cabo Verde (Africa)
    /// * The Republic of Cyprus (Asia)
    /// * The Czech Republic (Europe)
    /// * The Federal Republic of Germany (Europe)
    /// * The Republic of Djibouti (Africa)
    /// * The Kingdom of Denmark (Europe)
    /// * The People's Democratic Republic of Algeria (Africa)
    /// * The Republic of Estonia (Europe)
    /// * The Arab Republic of Egypt (Africa)
    /// * The Sahrawi Arab Democratic Republic (Africa)
    /// * The State of Eritrea (Africa)
    /// * The Kingdom of Spain (Europe)
    /// * The Federal Democratic Republic of Ethiopia (Africa)
    /// * The Republic of Finland (Europe)
    /// * The Faroe Islands (Europe)
    /// * The French Republic (Europe)
    /// * The Gabonese Republic (Africa)
    /// * The United Kingdom of Great Britain and Northern Ireland (Europe)
    /// * Georgia (Asia)
    /// * The Bailiwick of Guernsey (Europe)
    /// * The Republic of Ghana (Africa)
    /// * Gibraltar (Europe)
    /// * Kalaallit Nunaat (Americas)
    /// * The Republic of The Gambia (Africa)
    /// * The Republic of Guinea (Africa)
    /// * The Republic of Equatorial Guinea (Africa)
    /// * The Hellenic Republic (Europe)
    /// * The Republic of Guinea-Bissau (Africa)
    /// * The Republic of Croatia (Europe)
    /// * Hungary (Europe)
    /// * Ireland (Europe)
    /// * The State of Israel (Asia)
    /// * The Isle of Man (Europe)
    /// * The Republic of Iraq (Asia)
    /// * The Islamic Republic of Iran (Asia)
    /// * Iceland (Europe)
    /// * The Italian Republic (Europe)
    /// * The Bailiwick of Jersey (Europe)
    /// * The Hashemite Kingdom of Jordan (Asia)
    /// * The Republic of Kenya (Africa)
    /// * The Kyrgyz Republic (Asia)
    /// * The Union of the Comoros (Africa)
    /// * The State of Kuwait (Asia)
    /// * The Republic of Kazakhstan (Asia)
    /// * The Lebanese Republic (Asia)
    /// * The Principality of Liechtenstein (Europe)
    /// * The Republic of Liberia (Africa)
    /// * The Kingdom of Lesotho (Africa)
    /// * The Republic of Lithuania (Europe)
    /// * The Grand Duchy of Luxembourg (Europe)
    /// * The Republic of Latvia (Europe)
    /// * The State of Libya (Africa)
    /// * The Kingdom of Morocco (Africa)
    /// * The Principality of Monaco (Europe)
    /// * The Republic of Moldova (Europe)
    /// * Montenegro (Europe)
    /// * The Republic of Madagascar (Africa)
    /// * The Republic of North Macedonia (Europe)
    /// * The Republic of Mali (Africa)
    /// * The Islamic Republic of Mauritania (Africa)
    /// * Montserrat (Americas)
    /// * The Republic of Malta (Europe)
    /// * The Republic of Mauritius (Africa)
    /// * The Republic of Malawi (Africa)
    /// * The Republic of Mozambique (Africa)
    /// * The Republic of Namibia (Africa)
    /// * The Republic of the Niger (Africa)
    /// * The Federal Republic of Nigeria (Africa)
    /// * The Kingdom of the Netherlands (Europe)
    /// * The Kingdom of Norway (Europe)
    /// * The Sultanate of Oman (Asia)
    /// * The Republic of Poland (Europe)
    /// * The State of Palestine (Asia)
    /// * The Portuguese Republic (Europe)
    /// * The State of Qatar (Asia)
    /// * Réunion (Africa)
    /// * Romania (Europe)
    /// * The Republic of Serbia (Europe)
    /// * The Russian Federation (Europe)
    /// * The Republic of Rwanda (Africa)
    /// * The Kingdom of Saudi Arabia (Asia)
    /// * The Republic of Seychelles (Africa)
    /// * The Republic of the Sudan (Africa)
    /// * The Kingdom of Sweden (Europe)
    /// * The Republic of Slovenia (Europe)
    /// * Svalbard and Jan Mayen (Europe)
    /// * The Slovak Republic (Europe)
    /// * The Republic of Sierra Leone (Africa)
    /// * The Republic of San Marino (Europe)
    /// * The Republic of Senegal (Africa)
    /// * The Federal Republic of Somalia (Africa)
    /// * The Republic of South Sudan (Africa)
    /// * The Democratic Republic of São Tomé and Príncipe (Africa)
    /// * The Syrian Arab Republic (Asia)
    /// * The Kingdom of Eswatini (Africa)
    /// * The Republic of Chad (Africa)
    /// * The French Southern and Antarctic Lands (Africa)
    /// * The Togolese Republic (Africa)
    /// * The Republic of Tajikistan (Asia)
    /// * Turkmenistan (Asia)
    /// * The Republic of Tunisia (Africa)
    /// * The Republic of Turkey (Asia)
    /// * The United Republic of Tanzania (Africa)
    /// * Ukraine (Europe)
    /// * The Republic of Uganda (Africa)
    /// * The Republic of Uzbekistan (Asia)
    /// * The Holy See (Europe)
    /// * The Republic of Yemen (Asia)
    /// * The Department of Mayotte (Africa)
    /// * The Republic of South Africa (Africa)
    /// * The Republic of Zambia (Africa)
    /// * The Republic of Zimbabwe (Africa)
    EMEA,
}

impl WorldRegion {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::AMER => &[
                "AG", "AI", "AQ", "AR", "AW", "BB", "BM", "BO", "BR", "BS", "BZ", "CA", "CL", "CO",
                "CR", "CU", "CW", "DM", "DO", "EC", "FK", "GD", "GF", "GP", "GS", "GT", "GY", "HN",
                "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MX", "NI", "PA", "PE", "PM", "PR", "PY",
                "SR", "SV", "SX", "TT", "UM", "US", "UY", "VC", "VE", "VG", "VI",
            ],
            Self::APAC => &[
                "AF", "AS", "AU", "BD", "BL", "BN", "BQ", "BT", "BV", "CC", "CK", "CN", "CX", "FJ",
                "FM", "GU", "HK", "HM", "ID", "IN", "IO", "JP", "KH", "KI", "KP", "KR", "LA", "LK",
                "MH", "MM", "MN", "MO", "MP", "MV", "MY", "NC", "NF", "NP", "NR", "NU", "NZ", "PF",
                "PG", "PH", "PK", "PN", "PW", "SB", "SG", "SH", "TC", "TH", "TK", "TL", "TO", "TV",
                "TW", "VN", "VU", "WF", "WS",
            ],
            Self::EMEA => &[
                "AD", "AE", "AL", "AM", "AO", "AT", "AX", "AZ", "BA", "BE", "BF", "BG", "BH", "BI",
                "BJ", "BW", "BY", "CD", "CF", "CG", "CH", "CI", "CM", "CV", "CY", "CZ", "DE", "DJ",
                "DK", "DZ", "EE", "EG", "EH", "ER", "ES", "ET", "FI", "FO", "FR", "GA", "GB", "GE",
                "GG", "GH", "GI", "GL", "GM", "GN", "GQ", "GR", "GW", "HR", "HU", "IE", "IL", "IM",
                "IQ", "IR", "IS", "IT", "JE", "JO", "KE", "KG", "KM", "KW", "KZ", "LB", "LI", "LR",
                "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MG", "MK", "ML", "MR", "MS",
                "MT", "MU", "MW", "MZ", "NA", "NE", "NG", "NL", "NO", "OM", "PL", "PS", "PT", "QA",
                "RE", "RO", "RS", "RU", "RW", "SA", "SC", "SD", "SE", "SI", "SJ", "SK", "SL", "SM",
                "SN", "SO", "SS", "ST", "SY", "SZ", "TD", "TF", "TG", "TJ", "TM", "TN", "TR", "TZ",
                "UA", "UG", "UZ", "VA", "YE", "YT", "ZA", "ZM", "ZW",
            ],
        }
    }
}
impl TryFrom<&str> for WorldRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "amer" => Ok(Self::AMER),
            "apac" => Ok(Self::APAC),
            "emea" => Ok(Self::EMEA),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllWorldRegions,
            }),
        }
    }
}
impl ToString for WorldRegion {
    fn to_string(&self) -> String {
        match self {
            Self::AMER => "AMER",
            Self::APAC => "APAC",
            Self::EMEA => "EMEA",
        }
        .to_string()
    }
}
#[cfg(feature = "subdivisions")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing countries subdivision types.
pub enum SubdivisionType {
    /// Examples:
    /// * `Adis Abeba` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Dire Dawa` in `The Federal Democratic Republic of Ethiopia (Africa)`
    Administration,
    /// Examples:
    /// * `Ariatholhu Dhekunuburi` in `The Republic of Maldives (Asia)`
    /// * `Ariatholhu Uthuruburi` in `The Republic of Maldives (Asia)`
    /// * `Faadhippolhu` in `The Republic of Maldives (Asia)`
    AdministrativeAtoll,
    /// Examples:
    /// * `Alo` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    /// * `Sigave` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    /// * `Uvea` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    AdministrativePrecinct,
    /// Examples:
    /// * `Région de Boké` in `The Republic of Guinea (Africa)`
    /// * `Région de Kindia` in `The Republic of Guinea (Africa)`
    /// * `Région de Faranah` in `The Republic of Guinea (Africa)`
    AdministrativeRegion,
    /// Examples:
    /// * `Altayskiy kray` in `The Russian Federation (Europe)`
    /// * `Kamchatskaya oblast'` in `The Russian Federation (Europe)`
    /// * `Krasnodarskiy kray` in `The Russian Federation (Europe)`
    AdministrativeTerritory,
    /// Examples:
    /// * `Svalbard (Arctic Region)` in `The Kingdom of Norway (Europe)`
    /// * `Jan Mayen (Arctic Region)` in `The Kingdom of Norway (Europe)`
    ArcticRegion,
    /// Examples:
    /// * `Western Area (Freetown)` in `The Republic of Sierra Leone (Africa)`
    Area,
    /// Examples:
    /// * `Moskva` in `The Russian Federation (Europe)`
    /// * `Sankt-Peterburg` in `The Russian Federation (Europe)`
    AutonomousCity,
    /// Examples:
    /// * `Ceuta` in `The Kingdom of Spain (Europe)`
    /// * `Melilla` in `The Kingdom of Spain (Europe)`
    AutonomousCityInNorthAfrica,
    /// Examples:
    /// * `Andalucía` in `The Kingdom of Spain (Europe)`
    /// * `Aragón` in `The Kingdom of Spain (Europe)`
    /// * `Principado de Asturias` in `The Kingdom of Spain (Europe)`
    AutonomousCommunity,
    /// Examples:
    /// * `Abidjan` in `The Republic of Côte d'Ivoire (Africa)`
    /// * `Yamoussoukro` in `The Republic of Côte d'Ivoire (Africa)`
    /// * `Chukotskiy avtonomnyy okrug` in `The Russian Federation (Europe)`
    AutonomousDistrict,
    /// Examples:
    /// * `Phnom Penh [Phnum Pénh]` in `The Kingdom of Cambodia (Asia)`
    AutonomousMunicipality,
    /// Examples:
    /// * `Bolzano` in `The Italian Republic (Europe)`
    /// * `Trento` in `The Italian Republic (Europe)`
    /// * `Косово и Метохија` in `The Republic of Serbia (Europe)`
    AutonomousProvince,
    /// Examples:
    /// * `Guangxi` in `The People's Republic of China (Asia)`
    /// * `Nei Mongol (mn)` in `The People's Republic of China (Asia)`
    /// * `Ningxia` in `The People's Republic of China (Asia)`
    AutonomousRegion,
    /// Examples:
    /// * `Naxçivan` in `The Republic of Azerbaijan (Asia)`
    /// * `Abkhazia` in `Georgia (Asia)`
    /// * `Ajaria` in `Georgia (Asia)`
    AutonomousRepublic,
    /// Examples:
    /// * `Bissau` in `The Republic of Guinea-Bissau (Africa)`
    AutonomousSector,
    /// Examples:
    /// * `Gagauzia, Unitate Teritoriala Autonoma (UTAG)` in `The Republic of Moldova (Europe)`
    AutonomousTerritorialUnit,
    /// Examples:
    /// * `Arima` in `The Republic of Trinidad and Tobago (Americas)`
    /// * `Chaguanas` in `The Republic of Trinidad and Tobago (Americas)`
    /// * `Point Fortin` in `The Republic of Trinidad and Tobago (Americas)`
    Borough,
    /// Examples:
    /// * `Aargau (de)` in `The Swiss Confederation (Europe)`
    /// * `Appenzell Innerrhoden (de)` in `The Swiss Confederation (Europe)`
    /// * `Appenzell Ausserrhoden (de)` in `The Swiss Confederation (Europe)`
    Canton,
    /// Examples:
    /// * `Ciudad de México` in `The United Mexican States (Americas)`
    /// * `Asunción` in `The Republic of Paraguay (Americas)`
    Capital,
    /// Examples:
    /// * `Praha, Hlavní město` in `The Czech Republic (Europe)`
    /// * `Budapest` in `Hungary (Europe)`
    /// * `평양직할시` in `The Democratic People's Republic of Korea (Asia)`
    CapitalCity,
    /// Examples:
    /// * `Distrito Capital de Bogotá` in `The Republic of Colombia (Americas)`
    /// * `Distrito Federal` in `The Bolivarian Republic of Venezuela (Americas)`
    CapitalDistrict,
    /// Examples:
    /// * `Daerah Khusus Ibukota Jakarta` in `The Republic of Indonesia (Asia)`
    CapitalRegion,
    /// Examples:
    /// * `Abuja Capital Territory` in `The Federal Republic of Nigeria (Africa)`
    /// * `Capital Territory (Honiara)` in `The Solomon Islands (Oceania)`
    /// * `Dushanbe` in `The Republic of Tajikistan (Asia)`
    CapitalTerritory,
    /// Examples:
    /// * `Ralik Chain` in `The Republic of the Marshall Islands (Oceania)`
    /// * `Ratak Chain` in `The Republic of the Marshall Islands (Oceania)`
    ChainOfIslands,
    /// Examples:
    /// * `Erevan` in `The Republic of Armenia (Asia)`
    /// * `Capital federal` in `The Argentine Republic (Americas)`
    /// * `Francistown` in `The Republic of Botswana (Africa)`
    City,
    /// Examples:
    /// * `London, City of` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    CityCorporation,
    /// Examples:
    /// * `Alytaus miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Kauno miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Klaipėdos miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    CityMunicipality,
    /// Examples:
    /// * `Békéscsaba` in `Hungary (Europe)`
    /// * `Debrecen` in `Hungary (Europe)`
    /// * `Dunaújváros` in `Hungary (Europe)`
    CityWithCountyRights,
    /// Examples:
    /// * `Bangui` in `The Central African Republic (Africa)`
    /// * `Balzers` in `The Principality of Liechtenstein (Europe)`
    /// * `Eschen` in `The Principality of Liechtenstein (Europe)`
    Commune,
    /// Examples:
    /// * `Aberdeenshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Aberdeen City` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Argyll and Bute` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    CouncilArea,
    /// Examples:
    /// * `England` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Scotland` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Wales` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    Country,
    /// Examples:
    /// * `Berat` in `The Republic of Albania (Europe)`
    /// * `Durrës` in `The Republic of Albania (Europe)`
    /// * `Elbasan` in `The Republic of Albania (Europe)`
    County,
    /// Examples:
    /// * `Gorizia` in `The Italian Republic (Europe)`
    /// * `Pordenone` in `The Italian Republic (Europe)`
    /// * `Trieste` in `The Italian Republic (Europe)`
    DecentralizedRegionalEntity,
    /// Examples:
    /// * `El Beni` in `The Plurinational State of Bolivia (Americas)`
    /// * `Cochabamba` in `The Plurinational State of Bolivia (Americas)`
    /// * `Chuquisaca` in `The Plurinational State of Bolivia (Americas)`
    Department,
    /// Examples:
    /// * `Atakora` in `The Republic of Benin (Africa)`
    /// * `Alibori` in `The Republic of Benin (Africa)`
    /// * `Atlantique` in `The Republic of Benin (Africa)`
    Departments,
    /// Examples:
    /// * `Barbuda` in `Antigua and Barbuda (Americas)`
    /// * `Redonda` in `Antigua and Barbuda (Americas)`
    /// * `Rotuma` in `The Republic of Fiji (Oceania)`
    Dependency,
    /// Examples:
    /// * `मध्यमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `मध्य-पश्चिमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `पश्चिमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    DevelopmentRegion,
    /// Examples:
    /// * `Bandarban zila` in `The People's Republic of Bangladesh (Asia)`
    /// * `Barguna zila` in `The People's Republic of Bangladesh (Asia)`
    /// * `Bogra zila` in `The People's Republic of Bangladesh (Asia)`
    District,
    /// Examples:
    /// * `Akmenės rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Alytaus rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Anykščių rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    DistrictMunicipality,
    /// Examples:
    /// * `Brčko Distrikt` in `Bosnia and Herzegovina (Europe)`
    DistrictWithSpecialStatus,
    /// Examples:
    /// * `Nohiyahoi Tobei Jumhurí` in `The Republic of Tajikistan (Asia)`
    DistrictsUnderRepublicAdministration,
    /// Examples:
    /// * `বরিশাল বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    /// * `চট্টগ্রাম বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    /// * `ঢাকা বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    Division,
    /// Examples:
    /// * `Nana-Grébizi` in `The Central African Republic (Africa)`
    /// * `Sangha-Mbaéré` in `The Central African Republic (Africa)`
    EconomicPrefecture,
    /// Examples:
    /// * `'Ajmān` in `The United Arab Emirates (Asia)`
    /// * `Abū Z̧aby [Abu Dhabi]` in `The United Arab Emirates (Asia)`
    /// * `Dubayy` in `The United Arab Emirates (Asia)`
    Emirate,
    /// Examples:
    /// * `Federacija Bosna i Hercegovina` in `Bosnia and Herzegovina (Europe)`
    /// * `Republika Srpska` in `Bosnia and Herzegovina (Europe)`
    Entity,
    /// Examples:
    /// * `Alsace` in `The French Republic (Europe)`
    EuropeanCollectivity,
    /// Examples:
    /// * `Islamabad` in `The Islamic Republic of Pakistan (Asia)`
    FederalCapitalTerritory,
    /// Examples:
    /// * `Dependencias Federales` in `The Bolivarian Republic of Venezuela (Americas)`
    FederalDependency,
    /// Examples:
    /// * `Distrito Federal` in `The Federative Republic of Brazil (Americas)`
    FederalDistrict,
    /// Examples:
    /// * `Wilayah Persekutuan Kuala Lumpur` in `Malaysia (Asia)`
    /// * `Wilayah Persekutuan Labuan` in `Malaysia (Asia)`
    /// * `Wilayah Persekutuan Putrajaya` in `Malaysia (Asia)`
    FederalTerritory,
    /// Examples:
    /// * `Agrigento` in `The Italian Republic (Europe)`
    /// * `Caltanissetta` in `The Italian Republic (Europe)`
    /// * `Enna` in `The Italian Republic (Europe)`
    FreeMunicipalConsortium,
    /// Examples:
    /// * `Ascension` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    /// * `Saint Helena` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    /// * `Tristan da Cunha` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    GeographicalEntity,
    /// Examples:
    /// * `Ilhas de Barlavento` in `The Republic of Cabo Verde (Africa)`
    /// * `Ilhas de Sotavento` in `The Republic of Cabo Verde (Africa)`
    /// * `Central` in `The Republic of Uganda (Africa)`
    GeographicalRegion,
    /// Examples:
    /// * `Jawa` in `The Republic of Indonesia (Asia)`
    /// * `Kalimantan` in `The Republic of Indonesia (Asia)`
    /// * `Kepulauan Maluku` in `The Republic of Indonesia (Asia)`
    GeographicalUnit,
    /// Examples:
    /// * `Al Manamah (Al ‘Asimah)` in `The Kingdom of Bahrain (Asia)`
    /// * `Al Janubiyah` in `The Kingdom of Bahrain (Asia)`
    /// * `Al Muharraq` in `The Kingdom of Bahrain (Asia)`
    Governorate,
    /// Examples:
    /// * `Gilbert Islands` in `The Republic of Kiribati (Oceania)`
    /// * `Line Islands` in `The Republic of Kiribati (Oceania)`
    /// * `Phoenix Islands` in `The Republic of Kiribati (Oceania)`
    GroupOfIslands,
    /// Examples:
    /// * `Emberá-Wounaan` in `The Republic of Panamá (Americas)`
    /// * `Guna Yala` in `The Republic of Panamá (Americas)`
    /// * `Ngäbe-Buglé` in `The Republic of Panamá (Americas)`
    IndigenousRegion,
    /// Examples:
    /// * `New Providence` in `The Commonwealth of The Bahamas (Americas)`
    /// * `Anjouan` in `The Union of the Comoros (Africa)`
    /// * `Grande Comore` in `The Union of the Comoros (Africa)`
    Island,
    /// Examples:
    /// * `Niutao` in `Tuvalu (Oceania)`
    /// * `Nukufetau` in `Tuvalu (Oceania)`
    /// * `Nukulaelae` in `Tuvalu (Oceania)`
    IslandCouncil,
    /// Examples:
    /// * `Attard` in `The Republic of Malta (Europe)`
    /// * `Balzan` in `The Republic of Malta (Europe)`
    /// * `Birgu` in `The Republic of Malta (Europe)`
    LocalCouncil,
    /// Examples:
    /// * `Barking and Dagenham` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Brent` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bexley` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    LondonBorough,
    /// Examples:
    /// * `Krung Thep Maha Nakhon [Bangkok]` in `The Kingdom of Thailand (Asia)`
    MetropolitanAdministration,
    /// Examples:
    /// * `Bari` in `The Italian Republic (Europe)`
    /// * `Bologna` in `The Italian Republic (Europe)`
    /// * `Cagliari` in `The Italian Republic (Europe)`
    MetropolitanCity,
    /// Examples:
    /// * `Corse` in `The French Republic (Europe)`
    /// * `Métropole de Lyon` in `The French Republic (Europe)`
    /// * `Paris` in `The French Republic (Europe)`
    MetropolitanCollectivityWithSpecialStatus,
    /// Examples:
    /// * `Ain` in `The French Republic (Europe)`
    /// * `Aisne` in `The French Republic (Europe)`
    /// * `Allier` in `The French Republic (Europe)`
    MetropolitanDepartment,
    /// Examples:
    /// * `Birmingham` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Barnsley` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bolton` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    MetropolitanDistrict,
    /// Examples:
    /// * `Auvergne-Rhône-Alpes` in `The French Republic (Europe)`
    /// * `Bourgogne-Franche-Comté` in `The French Republic (Europe)`
    /// * `Saint-Barthélemy` in `The French Republic (Europe)`
    MetropolitanRegion,
    /// Examples:
    /// * `Baki` in `The Republic of Azerbaijan (Asia)`
    /// * `Gäncä` in `The Republic of Azerbaijan (Asia)`
    /// * `Länkäran City` in `The Republic of Azerbaijan (Asia)`
    Municipality,
    /// Examples:
    /// * `Brestskaya voblasts' (be) Brestskaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    /// * `Homyel'skaya voblasts' (be) Gomel'skaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    /// * `Hrodzenskaya voblasts' (be) Grodnenskaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    Oblast,
    /// Examples:
    /// * `American Samoa` in `The United States of America (Americas)`
    /// * `Guam` in `The United States of America (Americas)`
    /// * `Northern Mariana Islands` in `The United States of America (Americas)`
    OutlyingArea,
    /// Examples:
    /// * `Polynésie française` in `The French Republic (Europe)`
    /// * `Saint-Pierre-et-Miquelon` in `The French Republic (Europe)`
    /// * `Terres Australes Françaises` in `The French Republic (Europe)`
    OverseasCollectivity,
    /// Examples:
    /// * `Guadeloupe` in `The French Republic (Europe)`
    /// * `La Réunion` in `The French Republic (Europe)`
    /// * `Mayotte` in `The French Republic (Europe)`
    OverseasDepartmentalCollectivity,
    /// Examples:
    /// * `Martinique` in `The French Republic (Europe)`
    /// * `Guyane (française)` in `The French Republic (Europe)`
    OverseasUniqueTerritorialCollectivity,
    /// Examples:
    /// * `Gilgit-Baltistan` in `The Islamic Republic of Pakistan (Asia)`
    /// * `Azad Kashmir` in `The Islamic Republic of Pakistan (Asia)`
    PakistanAdministeredArea,
    /// Examples:
    /// * `Canillo` in `The Principality of Andorra (Europe)`
    /// * `Encamp` in `The Principality of Andorra (Europe)`
    /// * `La Massana` in `The Principality of Andorra (Europe)`
    Parish,
    /// Examples:
    /// * `Banghazi` in `The State of Libya (Africa)`
    /// * `Al Butnan` in `The State of Libya (Africa)`
    /// * `Darnah` in `The State of Libya (Africa)`
    Popularate,
    /// Examples:
    /// * `Ouham` in `The Central African Republic (Africa)`
    /// * `Bamingui-Bangoran` in `The Central African Republic (Africa)`
    /// * `Basse-Kotto` in `The Central African Republic (Africa)`
    Prefecture,
    /// Examples:
    /// * `Balkh` in `The Islamic Republic of Afghanistan (Asia)`
    /// * `Bamian` in `The Islamic Republic of Afghanistan (Asia)`
    /// * `Badghis` in `The Islamic Republic of Afghanistan (Asia)`
    Province,
    /// Examples:
    /// * `La Colle` in `The Principality of Monaco (Europe)`
    /// * `La Condamine` in `The Principality of Monaco (Europe)`
    /// * `Fontvieille` in `The Principality of Monaco (Europe)`
    Quarter,
    /// Examples:
    /// * `Abseron` in `The Republic of Azerbaijan (Asia)`
    /// * `Agstafa` in `The Republic of Azerbaijan (Asia)`
    /// * `Agcabädi` in `The Republic of Azerbaijan (Asia)`
    Rayon,
    /// Examples:
    /// * `Aragacotn` in `The Republic of Armenia (Asia)`
    /// * `Ararat` in `The Republic of Armenia (Asia)`
    /// * `Armavir` in `The Republic of Armenia (Asia)`
    Region,
    /// Examples:
    /// * `Afar` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Amara` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Binshangul Gumuz` in `The Federal Democratic Republic of Ethiopia (Africa)`
    RegionalState,
    /// Examples:
    /// * `Adygeya, Respublika` in `The Russian Federation (Europe)`
    /// * `Altay, Respublika` in `The Russian Federation (Europe)`
    /// * `Bashkortostan, Respublika` in `The Russian Federation (Europe)`
    Republic,
    /// Examples:
    /// * `Alutaguse` in `The Republic of Estonia (Europe)`
    /// * `Anija` in `The Republic of Estonia (Europe)`
    /// * `Antsla` in `The Republic of Estonia (Europe)`
    RuralMunicipality,
    /// Examples:
    /// * `Ágion Óros` in `The Hellenic Republic (Europe)`
    SelfGovernedPart,
    /// Examples:
    /// * `Phatthaya` in `The Kingdom of Thailand (Asia)`
    SpecialAdministrativeCity,
    /// Examples:
    /// * `Xianggang (zh) **` in `The People's Republic of China (Asia)`
    /// * `Aomen (zh) ***` in `The People's Republic of China (Asia)`
    /// * `Oecussi` in `The Democratic Republic of Timor-Leste (Asia)`
    SpecialAdministrativeRegion,
    /// Examples:
    /// * `라선특별시` in `The Democratic People's Republic of Korea (Asia)`
    /// * `Seoul Teugbyeolsi [Seoul-T'ukpyolshi]` in `The Republic of Korea (Asia)`
    SpecialCity,
    /// Examples:
    /// * `Chatham Islands Territory` in `New Zealand (Oceania)`
    SpecialIslandAuthority,
    /// Examples:
    /// * `Bonaire` in `Bonaire, Sint Eustatius and Saba (Americas)`
    /// * `Saba` in `Bonaire, Sint Eustatius and Saba (Americas)`
    /// * `Sint Eustatius` in `Bonaire, Sint Eustatius and Saba (Americas)`
    SpecialMunicipality,
    /// Examples:
    /// * `Yogyakarta` in `The Republic of Indonesia (Asia)`
    SpecialRegion,
    /// Examples:
    /// * `세종특별자치시` in `The Republic of Korea (Asia)`
    SpecialSelfGoverningCity,
    /// Examples:
    /// * `Jejudo [Cheju-do]` in `The Republic of Korea (Asia)`
    SpecialSelfGoverningProvince,
    /// Examples:
    /// * `Burgenland` in `The Republic of Austria (Europe)`
    /// * `Kärnten` in `The Republic of Austria (Europe)`
    /// * `Niederösterreich` in `The Republic of Austria (Europe)`
    State,
    /// Examples:
    /// * `Daugavpils` in `The Republic of Latvia (Europe)`
    /// * `Jelgava` in `The Republic of Latvia (Europe)`
    /// * `Jurmala` in `The Republic of Latvia (Europe)`
    StateCity,
    /// Examples:
    /// * `Stînga Nistrului, unitatea teritoriala din` in `The Republic of Moldova (Europe)`
    TerritorialUnit,
    /// Examples:
    /// * `Australian Capital Territory` in `The Commonwealth of Australia (Oceania)`
    /// * `Northern Territory` in `The Commonwealth of Australia (Oceania)`
    /// * `Northwest Territories` in `Canada (Americas)`
    Territory,
    /// Examples:
    /// * `Jwaneng` in `The Republic of Botswana (Africa)`
    /// * `Lobatse` in `The Republic of Botswana (Africa)`
    /// * `Selibe Phikwe` in `The Republic of Botswana (Africa)`
    Town,
    /// Examples:
    /// * `Funafuti` in `Tuvalu (Oceania)`
    TownCouncil,
    /// Examples:
    /// * `Buckinghamshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Cambridgeshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Cumbria` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    TwoTierCounty,
    /// Examples:
    /// * `Chandigarh` in `The Republic of India (Asia)`
    /// * `Dādra and Nagar Haveli and Damān and Diu` in `The Republic of India (Asia)`
    /// * `Delhi` in `The Republic of India (Asia)`
    UnionTerritory,
    /// Examples:
    /// * `Isle of Anglesey [Sir Ynys Môn GB-YNM]` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bath and North East Somerset` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Blackburn with Darwen` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    UnitaryAuthority,
    /// Examples:
    /// * `Niamey` in `The Republic of the Niger (Africa)`
    UrbanCommunity,
    /// Examples:
    /// * `Haapsalu` in `The Republic of Estonia (Europe)`
    /// * `Keila` in `The Republic of Estonia (Europe)`
    /// * `Kohtla-Järve` in `The Republic of Estonia (Europe)`
    UrbanMunicipality,
    /// Examples:
    /// * `Dolnośląskie` in `The Republic of Poland (Europe)`
    /// * `Kujawsko-pomorskie` in `The Republic of Poland (Europe)`
    /// * `Lubelskie` in `The Republic of Poland (Europe)`
    Voivodeship,
    /// Examples:
    /// * `Tobago` in `The Republic of Trinidad and Tobago (Americas)`
    Ward,
    /// Examples:
    /// * `Bagmati` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `Bheri` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `Dhawalagiri` in `The Federal Democratic Republic of Nepal (Asia)`
    Zone,
}

#[cfg(feature = "subdivisions")]
impl TryFrom<&str> for SubdivisionType {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "administration" => Ok(Self::Administration),
            "administrative atoll" => Ok(Self::AdministrativeAtoll),
            "administrative precinct" => Ok(Self::AdministrativePrecinct),
            "administrative region" => Ok(Self::AdministrativeRegion),
            "administrative territory" => Ok(Self::AdministrativeTerritory),
            "arctic region" => Ok(Self::ArcticRegion),
            "area" => Ok(Self::Area),
            "autonomous city" => Ok(Self::AutonomousCity),
            "autonomous city in north africa" => Ok(Self::AutonomousCityInNorthAfrica),
            "autonomous community" => Ok(Self::AutonomousCommunity),
            "autonomous district" => Ok(Self::AutonomousDistrict),
            "autonomous municipality" => Ok(Self::AutonomousMunicipality),
            "autonomous province" => Ok(Self::AutonomousProvince),
            "autonomous region" => Ok(Self::AutonomousRegion),
            "autonomous republic" => Ok(Self::AutonomousRepublic),
            "autonomous sector" => Ok(Self::AutonomousSector),
            "autonomous territorial unit" => Ok(Self::AutonomousTerritorialUnit),
            "borough" => Ok(Self::Borough),
            "canton" => Ok(Self::Canton),
            "capital" => Ok(Self::Capital),
            "capital city" => Ok(Self::CapitalCity),
            "capital district" => Ok(Self::CapitalDistrict),
            "capital region" => Ok(Self::CapitalRegion),
            "capital territory" => Ok(Self::CapitalTerritory),
            "chain of islands" => Ok(Self::ChainOfIslands),
            "city" => Ok(Self::City),
            "city corporation" => Ok(Self::CityCorporation),
            "city municipality" => Ok(Self::CityMunicipality),
            "city with county rights" => Ok(Self::CityWithCountyRights),
            "commune" => Ok(Self::Commune),
            "council area" => Ok(Self::CouncilArea),
            "country" => Ok(Self::Country),
            "county" => Ok(Self::County),
            "decentralized regional entity" => Ok(Self::DecentralizedRegionalEntity),
            "department" => Ok(Self::Department),
            "departments" => Ok(Self::Departments),
            "dependency" => Ok(Self::Dependency),
            "development region" => Ok(Self::DevelopmentRegion),
            "district" => Ok(Self::District),
            "district municipality" => Ok(Self::DistrictMunicipality),
            "district with special status" => Ok(Self::DistrictWithSpecialStatus),
            "districts under republic administration" => {
                Ok(Self::DistrictsUnderRepublicAdministration)
            }
            "division" => Ok(Self::Division),
            "economic prefecture" => Ok(Self::EconomicPrefecture),
            "emirate" => Ok(Self::Emirate),
            "entity" => Ok(Self::Entity),
            "european collectivity" => Ok(Self::EuropeanCollectivity),
            "federal capital territory" => Ok(Self::FederalCapitalTerritory),
            "federal dependency" => Ok(Self::FederalDependency),
            "federal district" => Ok(Self::FederalDistrict),
            "federal territory" => Ok(Self::FederalTerritory),
            "free municipal consortium" => Ok(Self::FreeMunicipalConsortium),
            "geographical entity" => Ok(Self::GeographicalEntity),
            "geographical region" => Ok(Self::GeographicalRegion),
            "geographical unit" => Ok(Self::GeographicalUnit),
            "governorate" => Ok(Self::Governorate),
            "group of islands" => Ok(Self::GroupOfIslands),
            "indigenous region" => Ok(Self::IndigenousRegion),
            "island" => Ok(Self::Island),
            "island council" => Ok(Self::IslandCouncil),
            "local council" => Ok(Self::LocalCouncil),
            "london borough" => Ok(Self::LondonBorough),
            "metropolitan administration" => Ok(Self::MetropolitanAdministration),
            "metropolitan city" => Ok(Self::MetropolitanCity),
            "metropolitan collectivity with special status" => {
                Ok(Self::MetropolitanCollectivityWithSpecialStatus)
            }
            "metropolitan department" => Ok(Self::MetropolitanDepartment),
            "metropolitan district" => Ok(Self::MetropolitanDistrict),
            "metropolitan region" => Ok(Self::MetropolitanRegion),
            "municipality" => Ok(Self::Municipality),
            "oblast" => Ok(Self::Oblast),
            "outlying area" => Ok(Self::OutlyingArea),
            "overseas collectivity" => Ok(Self::OverseasCollectivity),
            "overseas departmental collectivity" => Ok(Self::OverseasDepartmentalCollectivity),
            "overseas unique territorial collectivity" => {
                Ok(Self::OverseasUniqueTerritorialCollectivity)
            }
            "pakistan administered area" => Ok(Self::PakistanAdministeredArea),
            "parish" => Ok(Self::Parish),
            "popularate" => Ok(Self::Popularate),
            "prefecture" => Ok(Self::Prefecture),
            "province" => Ok(Self::Province),
            "quarter" => Ok(Self::Quarter),
            "rayon" => Ok(Self::Rayon),
            "region" => Ok(Self::Region),
            "regional state" => Ok(Self::RegionalState),
            "republic" => Ok(Self::Republic),
            "rural municipality" => Ok(Self::RuralMunicipality),
            "self-governed part" => Ok(Self::SelfGovernedPart),
            "special administrative city" => Ok(Self::SpecialAdministrativeCity),
            "special administrative region" => Ok(Self::SpecialAdministrativeRegion),
            "special city" => Ok(Self::SpecialCity),
            "special island authority" => Ok(Self::SpecialIslandAuthority),
            "special municipality" => Ok(Self::SpecialMunicipality),
            "special region" => Ok(Self::SpecialRegion),
            "special self-governing city" => Ok(Self::SpecialSelfGoverningCity),
            "special self-governing province" => Ok(Self::SpecialSelfGoverningProvince),
            "state" => Ok(Self::State),
            "state city" => Ok(Self::StateCity),
            "territorial unit" => Ok(Self::TerritorialUnit),
            "territory" => Ok(Self::Territory),
            "town" => Ok(Self::Town),
            "town council" => Ok(Self::TownCouncil),
            "two-tier county" => Ok(Self::TwoTierCounty),
            "union territory" => Ok(Self::UnionTerritory),
            "unitary authority" => Ok(Self::UnitaryAuthority),
            "urban community" => Ok(Self::UrbanCommunity),
            "urban municipality" => Ok(Self::UrbanMunicipality),
            "voivodeship" => Ok(Self::Voivodeship),
            "ward" => Ok(Self::Ward),
            "zone" => Ok(Self::Zone),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllSubDivisionTypes,
            }),
        }
    }
}
#[cfg(feature = "subdivisions")]
impl ToString for SubdivisionType {
    fn to_string(&self) -> String {
        match self {
            Self::Administration => "administration",
            Self::AdministrativeAtoll => "administrative_atoll",
            Self::AdministrativePrecinct => "administrative_precinct",
            Self::AdministrativeRegion => "administrative_region",
            Self::AdministrativeTerritory => "administrative_territory",
            Self::ArcticRegion => "arctic_region",
            Self::Area => "area",
            Self::AutonomousCity => "autonomous_city",
            Self::AutonomousCityInNorthAfrica => "autonomous_city_in_north_africa",
            Self::AutonomousCommunity => "autonomous_community",
            Self::AutonomousDistrict => "autonomous_district",
            Self::AutonomousMunicipality => "autonomous_municipality",
            Self::AutonomousProvince => "autonomous_province",
            Self::AutonomousRegion => "autonomous_region",
            Self::AutonomousRepublic => "autonomous_republic",
            Self::AutonomousSector => "autonomous_sector",
            Self::AutonomousTerritorialUnit => "autonomous_territorial_unit",
            Self::Borough => "borough",
            Self::Canton => "canton",
            Self::Capital => "capital",
            Self::CapitalCity => "capital_city",
            Self::CapitalDistrict => "capital_district",
            Self::CapitalRegion => "capital_region",
            Self::CapitalTerritory => "capital_territory",
            Self::ChainOfIslands => "chain_of_islands",
            Self::City => "city",
            Self::CityCorporation => "city_corporation",
            Self::CityMunicipality => "city_municipality",
            Self::CityWithCountyRights => "city_with_county_rights",
            Self::Commune => "commune",
            Self::CouncilArea => "council_area",
            Self::Country => "country",
            Self::County => "county",
            Self::DecentralizedRegionalEntity => "decentralized_regional_entity",
            Self::Department => "department",
            Self::Departments => "departments",
            Self::Dependency => "dependency",
            Self::DevelopmentRegion => "development_region",
            Self::District => "district",
            Self::DistrictMunicipality => "district_municipality",
            Self::DistrictWithSpecialStatus => "district_with_special_status",
            Self::DistrictsUnderRepublicAdministration => "districts_under_republic_administration",
            Self::Division => "division",
            Self::EconomicPrefecture => "economic_prefecture",
            Self::Emirate => "emirate",
            Self::Entity => "entity",
            Self::EuropeanCollectivity => "european_collectivity",
            Self::FederalCapitalTerritory => "federal_capital_territory",
            Self::FederalDependency => "federal_dependency",
            Self::FederalDistrict => "federal_district",
            Self::FederalTerritory => "federal_territory",
            Self::FreeMunicipalConsortium => "free_municipal_consortium",
            Self::GeographicalEntity => "geographical_entity",
            Self::GeographicalRegion => "geographical_region",
            Self::GeographicalUnit => "geographical_unit",
            Self::Governorate => "governorate",
            Self::GroupOfIslands => "group_of_islands",
            Self::IndigenousRegion => "indigenous_region",
            Self::Island => "island",
            Self::IslandCouncil => "island_council",
            Self::LocalCouncil => "local_council",
            Self::LondonBorough => "london_borough",
            Self::MetropolitanAdministration => "metropolitan_administration",
            Self::MetropolitanCity => "metropolitan_city",
            Self::MetropolitanCollectivityWithSpecialStatus => {
                "metropolitan_collectivity_with_special_status"
            }
            Self::MetropolitanDepartment => "metropolitan_department",
            Self::MetropolitanDistrict => "metropolitan_district",
            Self::MetropolitanRegion => "metropolitan_region",
            Self::Municipality => "municipality",
            Self::Oblast => "oblast",
            Self::OutlyingArea => "outlying_area",
            Self::OverseasCollectivity => "overseas_collectivity",
            Self::OverseasDepartmentalCollectivity => "overseas_departmental_collectivity",
            Self::OverseasUniqueTerritorialCollectivity => {
                "overseas_unique_territorial_collectivity"
            }
            Self::PakistanAdministeredArea => "pakistan_administered_area",
            Self::Parish => "parish",
            Self::Popularate => "popularate",
            Self::Prefecture => "prefecture",
            Self::Province => "province",
            Self::Quarter => "quarter",
            Self::Rayon => "rayon",
            Self::Region => "region",
            Self::RegionalState => "regional_state",
            Self::Republic => "republic",
            Self::RuralMunicipality => "rural_municipality",
            Self::SelfGovernedPart => "self-governed_part",
            Self::SpecialAdministrativeCity => "special_administrative_city",
            Self::SpecialAdministrativeRegion => "special_administrative_region",
            Self::SpecialCity => "special_city",
            Self::SpecialIslandAuthority => "special_island_authority",
            Self::SpecialMunicipality => "special_municipality",
            Self::SpecialRegion => "special_region",
            Self::SpecialSelfGoverningCity => "special_self-governing_city",
            Self::SpecialSelfGoverningProvince => "special_self-governing_province",
            Self::State => "state",
            Self::StateCity => "state_city",
            Self::TerritorialUnit => "territorial_unit",
            Self::Territory => "territory",
            Self::Town => "town",
            Self::TownCouncil => "town_council",
            Self::TwoTierCounty => "two-tier_county",
            Self::UnionTerritory => "union_territory",
            Self::UnitaryAuthority => "unitary_authority",
            Self::UrbanCommunity => "urban_community",
            Self::UrbanMunicipality => "urban_municipality",
            Self::Voivodeship => "voivodeship",
            Self::Ward => "ward",
            Self::Zone => "zone",
        }
        .to_string()
    }
}
