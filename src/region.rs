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
    /// * Antarctica
    /// * Bouvet Island
    /// * South Georgia and the South Sandwich Islands (Americas)
    /// * The Territory of Heard Island and McDonald Islands
    Antarctica,
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
}

impl Continent {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::NorthAmerica => &[
                "AG", "AI", "AW", "BB", "BL", "BM", "BQ", "BS", "BZ", "CA", "CR", "CU", "CW", "DM",
                "DO", "GD", "GL", "GP", "GT", "HN", "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MS",
                "MX", "NI", "PA", "PM", "PR", "SV", "SX", "TC", "TT", "US", "VC", "VG", "VI",
            ],
            Self::SouthAmerica => &[
                "AR", "BO", "BR", "CL", "CO", "EC", "FK", "GF", "GY", "PE", "PY", "SR", "UY", "VE",
            ],
            Self::Australia => &[
                "AS", "AU", "CK", "FJ", "FM", "GU", "KI", "MH", "MP", "NC", "NF", "NR", "NU", "NZ",
                "PF", "PG", "PN", "PW", "SB", "TK", "TO", "TV", "UM", "VU", "WF", "WS",
            ],
            Self::Asia => &[
                "AE", "AF", "AM", "AZ", "BD", "BH", "BN", "BT", "CC", "CN", "CX", "CY", "GE", "HK",
                "ID", "IL", "IN", "IO", "IQ", "IR", "JO", "JP", "KG", "KH", "KP", "KR", "KW", "KZ",
                "LA", "LB", "LK", "MM", "MN", "MO", "MV", "MY", "NP", "OM", "PH", "PK", "PS", "QA",
                "SA", "SG", "SY", "TH", "TJ", "TL", "TM", "TW", "UZ", "VN", "YE",
            ],
            Self::Antarctica => &["AQ", "BV", "GS", "HM"],
            Self::Africa => &[
                "AO", "BF", "BI", "BJ", "BW", "CD", "CF", "CG", "CI", "CM", "CV", "DJ", "DZ", "EG",
                "EH", "ER", "ET", "GA", "GH", "GM", "GN", "GQ", "GW", "KE", "KM", "LR", "LS", "LY",
                "MA", "MG", "ML", "MR", "MU", "MW", "MZ", "NA", "NE", "NG", "RE", "RW", "SC", "SD",
                "SH", "SL", "SN", "SO", "SS", "ST", "SZ", "TD", "TF", "TG", "TN", "TZ", "UG", "YT",
                "ZA", "ZM", "ZW",
            ],
            Self::Europe => &[
                "AD", "AL", "AT", "AX", "BA", "BE", "BG", "BY", "CH", "CZ", "DE", "DK", "EE", "ES",
                "FI", "FO", "FR", "GB", "GG", "GI", "GR", "HR", "HU", "IE", "IM", "IS", "IT", "JE",
                "LI", "LT", "LU", "LV", "MC", "MD", "ME", "MK", "MT", "NL", "NO", "PL", "PT", "RO",
                "RS", "RU", "SE", "SI", "SJ", "SK", "SM", "TR", "UA", "VA",
            ],
        }
    }
}
impl TryFrom<&str> for Continent {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "north-america" => Ok(Self::NorthAmerica),
            "south-america" => Ok(Self::SouthAmerica),
            "australia" => Ok(Self::Australia),
            "asia" => Ok(Self::Asia),
            "antarctica" => Ok(Self::Antarctica),
            "africa" => Ok(Self::Africa),
            "europe" => Ok(Self::Europe),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllContinents,
            }),
        }
    }
}
impl ToString for Continent {
    fn to_string(&self) -> String {
        match self {
            Self::NorthAmerica => "north-america",
            Self::SouthAmerica => "south-america",
            Self::Australia => "australia",
            Self::Asia => "asia",
            Self::Antarctica => "antarctica",
            Self::Africa => "africa",
            Self::Europe => "europe",
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
            Self::Antarctica => &[],
            Self::Asia => &[
                "AE", "AF", "AM", "AZ", "BD", "BH", "BN", "BT", "CN", "CY", "GE", "HK", "ID", "IL",
                "IN", "IQ", "IR", "JO", "JP", "KG", "KH", "KP", "KR", "KW", "KZ", "LA", "LB", "LK",
                "MM", "MN", "MO", "MV", "MY", "NP", "OM", "PH", "PK", "PS", "QA", "SA", "SG", "SY",
                "TH", "TJ", "TL", "TM", "TR", "TW", "UZ", "VN", "YE",
            ],
            Self::Americas => &[
                "AG", "AI", "AR", "AW", "BB", "BL", "BM", "BO", "BQ", "BR", "BS", "BZ", "CA", "CL",
                "CO", "CR", "CU", "CW", "DM", "DO", "EC", "FK", "GD", "GF", "GL", "GP", "GS", "GT",
                "GY", "HN", "HT", "JM", "KN", "KY", "LC", "MF", "MQ", "MS", "MX", "NI", "PA", "PE",
                "PM", "PR", "PY", "SR", "SV", "SX", "TC", "TT", "UM", "US", "UY", "VC", "VE", "VG",
                "VI",
            ],
            Self::Africa => &[
                "AO", "BF", "BI", "BJ", "BW", "CD", "CF", "CG", "CI", "CM", "CV", "DJ", "DZ", "EG",
                "EH", "ER", "ET", "GA", "GH", "GM", "GN", "GQ", "GW", "IO", "KE", "KM", "LR", "LS",
                "LY", "MA", "MG", "ML", "MR", "MU", "MW", "MZ", "NA", "NE", "NG", "RE", "RW", "SC",
                "SD", "SH", "SL", "SN", "SO", "SS", "ST", "SZ", "TD", "TF", "TG", "TN", "TZ", "UG",
                "YT", "ZA", "ZM", "ZW",
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
            "antarctica" => Ok(Self::Antarctica),
            "asia" => Ok(Self::Asia),
            "americas" => Ok(Self::Americas),
            "africa" => Ok(Self::Africa),
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
            Self::Antarctica => "Antarctica",
            Self::Asia => "asia",
            Self::Americas => "americas",
            Self::Africa => "africa",
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
    /// * The Republic of Botswana
    /// * The Kingdom of Lesotho
    /// * The Republic of Namibia
    /// * The Kingdom of Eswatini
    /// * The Republic of South Africa
    SouthernAfrica,
    /// * The Federated States of Micronesia
    /// * The Territory of Guam
    /// * The Republic of Kiribati
    /// * The Republic of the Marshall Islands
    /// * The Commonwealth of the Northern Mariana Islands
    /// * The Republic of Nauru
    /// * The Republic of Palau
    Micronesia,
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
    /// * The People's Democratic Republic of Algeria
    /// * The Arab Republic of Egypt
    /// * The Sahrawi Arab Democratic Republic
    /// * The State of Libya
    /// * The Kingdom of Morocco
    /// * The Republic of the Sudan
    /// * The Republic of South Sudan
    /// * The Republic of Tunisia
    NorthernAfrica,
    /// * The Commonwealth of Australia
    /// * The Territory of Cocos (Keeling) Islands
    /// * The Territory of Christmas Island
    /// * The Territory of Norfolk Island
    /// * New Zealand
    AustraliaAndNewZealand,
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
    /// * Bermuda
    /// * Canada
    /// * Kalaallit Nunaat
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon
    /// * United States Minor Outlying Islands
    /// * The United States of America
    NorthernAmerica,
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
    /// * The Republic of Fiji
    /// * New Caledonia
    /// * The Independent State of Papua New Guinea
    /// * The Solomon Islands
    /// * The Republic of Vanuatu
    Melanesia,
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
    /// * The People's Republic of China
    /// * The Hong Kong Special Administrative Region of China
    /// * Japan
    /// * The Democratic People's Republic of Korea
    /// * The Republic of Korea
    /// * Mongolia
    /// * The Macao Special Administrative Region of China
    /// * The Republic of China
    EasternAsia,
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
}

impl SubRegion {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::SouthernAfrica => &["BW", "LS", "NA", "SZ", "ZA"],
            Self::Micronesia => &["FM", "GU", "KI", "MH", "MP", "NR", "PW"],
            Self::SouthernEurope => &[
                "AD", "AL", "BA", "ES", "GI", "GR", "HR", "IT", "ME", "MK", "MT", "PT", "RS", "SI",
                "SM", "VA",
            ],
            Self::MiddleAfrica => &["AO", "CD", "CF", "CG", "CM", "GA", "GQ", "ST", "TD"],
            Self::WesternAfrica => &[
                "BF", "BJ", "CI", "CV", "GH", "GM", "GN", "GW", "LR", "ML", "MR", "NE", "NG", "SH",
                "SL", "SN", "TG",
            ],
            Self::EasternEurope => &["BG", "BY", "CZ", "HU", "MD", "PL", "RO", "RU", "SK", "UA"],
            Self::CentralAmerica => &["BZ", "CR", "GT", "HN", "MX", "NI", "PA", "SV"],
            Self::CentralAsia => &["KG", "KZ", "TJ", "TM", "UZ"],
            Self::SouthernAsia => &["AF", "BD", "BT", "IN", "IR", "LK", "MV", "NP", "PK"],
            Self::WesternEurope => &["AT", "BE", "CH", "DE", "FR", "LI", "LU", "MC", "NL"],
            Self::NorthernAfrica => &["DZ", "EG", "EH", "LY", "MA", "SD", "SS", "TN"],
            Self::AustraliaAndNewZealand => &["AU", "CC", "CX", "NF", "NZ"],
            Self::Polynesia => &["AS", "CK", "NU", "PF", "PN", "TK", "TO", "TV", "WF", "WS"],
            Self::EasternAfrica => &[
                "BI", "DJ", "ER", "ET", "IO", "KE", "KM", "MG", "MU", "MW", "MZ", "RE", "RW", "SC",
                "SO", "TF", "TZ", "UG", "YT", "ZM", "ZW",
            ],
            Self::SouthEasternAsia => &[
                "BN", "ID", "KH", "LA", "MM", "MY", "PH", "SG", "TH", "TL", "VN",
            ],
            Self::NorthernAmerica => &["BM", "CA", "GL", "PM", "UM", "US"],
            Self::SouthAmerica => &[
                "AR", "BO", "BR", "CL", "CO", "EC", "FK", "GF", "GS", "GY", "PE", "PY", "SR", "UY",
                "VE",
            ],
            Self::Melanesia => &["FJ", "NC", "PG", "SB", "VU"],
            Self::WesternAsia => &[
                "AE", "AM", "AZ", "BH", "CY", "GE", "IL", "IQ", "JO", "KW", "LB", "OM", "PS", "QA",
                "SA", "SY", "TR", "YE",
            ],
            Self::EasternAsia => &["CN", "HK", "JP", "KP", "KR", "MN", "MO", "TW"],
            Self::Caribbean => &[
                "AG", "AI", "AW", "BB", "BL", "BQ", "BS", "CU", "CW", "DM", "DO", "GD", "GP", "HT",
                "JM", "KN", "KY", "LC", "MF", "MQ", "MS", "PR", "SX", "TC", "TT", "VC", "VG", "VI",
            ],
            Self::NorthernEurope => &[
                "AX", "DK", "EE", "FI", "FO", "GB", "GG", "IE", "IM", "IS", "JE", "LT", "LV", "NO",
                "SE", "SJ",
            ],
        }
    }
}
impl TryFrom<&str> for SubRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "southern-africa" => Ok(Self::SouthernAfrica),
            "micronesia" => Ok(Self::Micronesia),
            "southern-europe" => Ok(Self::SouthernEurope),
            "middle-africa" => Ok(Self::MiddleAfrica),
            "western-africa" => Ok(Self::WesternAfrica),
            "eastern-europe" => Ok(Self::EasternEurope),
            "central-america" => Ok(Self::CentralAmerica),
            "central-asia" => Ok(Self::CentralAsia),
            "southern-asia" => Ok(Self::SouthernAsia),
            "western-europe" => Ok(Self::WesternEurope),
            "northern-africa" => Ok(Self::NorthernAfrica),
            "australia-and-new-zealand" => Ok(Self::AustraliaAndNewZealand),
            "polynesia" => Ok(Self::Polynesia),
            "eastern-africa" => Ok(Self::EasternAfrica),
            "south-eastern-asia" => Ok(Self::SouthEasternAsia),
            "northern-america" => Ok(Self::NorthernAmerica),
            "south-america" => Ok(Self::SouthAmerica),
            "melanesia" => Ok(Self::Melanesia),
            "western-asia" => Ok(Self::WesternAsia),
            "eastern-asia" => Ok(Self::EasternAsia),
            "caribbean" => Ok(Self::Caribbean),
            "northern-europe" => Ok(Self::NorthernEurope),
            _ => Err(SearchError::NotFound {
                searched_items: SearchedItems::AllSubRegions,
            }),
        }
    }
}
impl ToString for SubRegion {
    fn to_string(&self) -> String {
        match self {
            Self::SouthernAfrica => "southern-africa",
            Self::Micronesia => "micronesia",
            Self::SouthernEurope => "southern-europe",
            Self::MiddleAfrica => "middle-africa",
            Self::WesternAfrica => "western-africa",
            Self::EasternEurope => "eastern-europe",
            Self::CentralAmerica => "central-america",
            Self::CentralAsia => "central-asia",
            Self::SouthernAsia => "southern-asia",
            Self::WesternEurope => "western-europe",
            Self::NorthernAfrica => "northern-africa",
            Self::AustraliaAndNewZealand => "australia-and-new-zealand",
            Self::Polynesia => "polynesia",
            Self::EasternAfrica => "eastern-africa",
            Self::SouthEasternAsia => "south-eastern-asia",
            Self::NorthernAmerica => "northern-america",
            Self::SouthAmerica => "south-america",
            Self::Melanesia => "melanesia",
            Self::WesternAsia => "western-asia",
            Self::EasternAsia => "eastern-asia",
            Self::Caribbean => "caribbean",
            Self::NorthernEurope => "northern-europe",
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
            Self::APAC => &[
                "AF", "AS", "AU", "BD", "BL", "BN", "BQ", "BT", "BV", "CC", "CK", "CN", "CX", "FJ",
                "FM", "GU", "HK", "HM", "ID", "IN", "IO", "JP", "KH", "KI", "KP", "KR", "LA", "LK",
                "MH", "MM", "MN", "MO", "MP", "MV", "MY", "NC", "NF", "NP", "NR", "NU", "NZ", "PF",
                "PG", "PH", "PK", "PN", "PW", "SB", "SG", "SH", "TC", "TH", "TK", "TL", "TO", "TV",
                "TW", "VN", "VU", "WF", "WS",
            ],
        }
    }
}
impl TryFrom<&str> for WorldRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "amer" => Ok(Self::AMER),
            "emea" => Ok(Self::EMEA),
            "apac" => Ok(Self::APAC),
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
            Self::EMEA => "EMEA",
            Self::APAC => "APAC",
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
    /// * `Canillo` in `The Principality of Andorra (Europe)`
    /// * `Encamp` in `The Principality of Andorra (Europe)`
    /// * `La Massana` in `The Principality of Andorra (Europe)`
    Parish,
    /// Examples:
    /// * `'Ajmān` in `The United Arab Emirates (Asia)`
    /// * `Abū Z̧aby [Abu Dhabi]` in `The United Arab Emirates (Asia)`
    /// * `Dubayy` in `The United Arab Emirates (Asia)`
    Emirate,
    /// Examples:
    /// * `Balkh` in `The Islamic Republic of Afghanistan (Asia)`
    /// * `Bamian` in `The Islamic Republic of Afghanistan (Asia)`
    /// * `Badghis` in `The Islamic Republic of Afghanistan (Asia)`
    Province,
    /// Examples:
    /// * `Barbuda` in `Antigua and Barbuda (Americas)`
    /// * `Redonda` in `Antigua and Barbuda (Americas)`
    /// * `Rotuma` in `The Republic of Fiji (Oceania)`
    Dependency,
    /// Examples:
    /// * `Berat` in `The Republic of Albania (Europe)`
    /// * `Durrës` in `The Republic of Albania (Europe)`
    /// * `Elbasan` in `The Republic of Albania (Europe)`
    County,
    /// Examples:
    /// * `Aragacotn` in `The Republic of Armenia (Asia)`
    /// * `Ararat` in `The Republic of Armenia (Asia)`
    /// * `Armavir` in `The Republic of Armenia (Asia)`
    Region,
    /// Examples:
    /// * `Erevan` in `The Republic of Armenia (Asia)`
    /// * `Capital federal` in `The Argentine Republic (Americas)`
    /// * `Francistown` in `The Republic of Botswana (Africa)`
    City,
    /// Examples:
    /// * `Burgenland` in `The Republic of Austria (Europe)`
    /// * `Kärnten` in `The Republic of Austria (Europe)`
    /// * `Niederösterreich` in `The Republic of Austria (Europe)`
    State,
    /// Examples:
    /// * `Australian Capital Territory` in `The Commonwealth of Australia (Oceania)`
    /// * `Northern Territory` in `The Commonwealth of Australia (Oceania)`
    /// * `Northwest Territories` in `Canada (Americas)`
    Territory,
    /// Examples:
    /// * `Abseron` in `The Republic of Azerbaijan (Asia)`
    /// * `Agstafa` in `The Republic of Azerbaijan (Asia)`
    /// * `Agcabädi` in `The Republic of Azerbaijan (Asia)`
    Rayon,
    /// Examples:
    /// * `Baki` in `The Republic of Azerbaijan (Asia)`
    /// * `Gäncä` in `The Republic of Azerbaijan (Asia)`
    /// * `Länkäran City` in `The Republic of Azerbaijan (Asia)`
    Municipality,
    /// Examples:
    /// * `Naxçivan` in `The Republic of Azerbaijan (Asia)`
    /// * `Abkhazia` in `Georgia (Asia)`
    /// * `Ajaria` in `Georgia (Asia)`
    AutonomousRepublic,
    /// Examples:
    /// * `Federacija Bosna i Hercegovina` in `Bosnia and Herzegovina (Europe)`
    /// * `Republika Srpska` in `Bosnia and Herzegovina (Europe)`
    Entity,
    /// Examples:
    /// * `Brčko Distrikt` in `Bosnia and Herzegovina (Europe)`
    DistrictWithSpecialStatus,
    /// Examples:
    /// * `Bandarban zila` in `The People's Republic of Bangladesh (Asia)`
    /// * `Barguna zila` in `The People's Republic of Bangladesh (Asia)`
    /// * `Bogra zila` in `The People's Republic of Bangladesh (Asia)`
    District,
    /// Examples:
    /// * `বরিশাল বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    /// * `চট্টগ্রাম বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    /// * `ঢাকা বিভাগ` in `The People's Republic of Bangladesh (Asia)`
    Division,
    /// Examples:
    /// * `Al Manamah (Al ‘Asimah)` in `The Kingdom of Bahrain (Asia)`
    /// * `Al Janubiyah` in `The Kingdom of Bahrain (Asia)`
    /// * `Al Muharraq` in `The Kingdom of Bahrain (Asia)`
    Governorate,
    /// Examples:
    /// * `Atakora` in `The Republic of Benin (Africa)`
    /// * `Alibori` in `The Republic of Benin (Africa)`
    /// * `Atlantique` in `The Republic of Benin (Africa)`
    Departments,
    /// Examples:
    /// * `El Beni` in `The Plurinational State of Bolivia (Americas)`
    /// * `Cochabamba` in `The Plurinational State of Bolivia (Americas)`
    /// * `Chuquisaca` in `The Plurinational State of Bolivia (Americas)`
    Department,
    /// Examples:
    /// * `Bonaire` in `Bonaire, Sint Eustatius and Saba (Americas)`
    /// * `Saba` in `Bonaire, Sint Eustatius and Saba (Americas)`
    /// * `Sint Eustatius` in `Bonaire, Sint Eustatius and Saba (Americas)`
    SpecialMunicipality,
    /// Examples:
    /// * `Distrito Federal` in `The Federative Republic of Brazil (Americas)`
    FederalDistrict,
    /// Examples:
    /// * `New Providence` in `The Commonwealth of The Bahamas (Americas)`
    /// * `Anjouan` in `The Union of the Comoros (Africa)`
    /// * `Grande Comore` in `The Union of the Comoros (Africa)`
    Island,
    /// Examples:
    /// * `Jwaneng` in `The Republic of Botswana (Africa)`
    /// * `Lobatse` in `The Republic of Botswana (Africa)`
    /// * `Selibe Phikwe` in `The Republic of Botswana (Africa)`
    Town,
    /// Examples:
    /// * `Brestskaya voblasts' (be) Brestskaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    /// * `Homyel'skaya voblasts' (be) Gomel'skaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    /// * `Hrodzenskaya voblasts' (be) Grodnenskaya oblast' (ru)` in `The Republic of Belarus (Europe)`
    Oblast,
    /// Examples:
    /// * `Ouham` in `The Central African Republic (Africa)`
    /// * `Bamingui-Bangoran` in `The Central African Republic (Africa)`
    /// * `Basse-Kotto` in `The Central African Republic (Africa)`
    Prefecture,
    /// Examples:
    /// * `Bangui` in `The Central African Republic (Africa)`
    /// * `Balzers` in `The Principality of Liechtenstein (Europe)`
    /// * `Eschen` in `The Principality of Liechtenstein (Europe)`
    Commune,
    /// Examples:
    /// * `Nana-Grébizi` in `The Central African Republic (Africa)`
    /// * `Sangha-Mbaéré` in `The Central African Republic (Africa)`
    EconomicPrefecture,
    /// Examples:
    /// * `Aargau (de)` in `The Swiss Confederation (Europe)`
    /// * `Appenzell Innerrhoden (de)` in `The Swiss Confederation (Europe)`
    /// * `Appenzell Ausserrhoden (de)` in `The Swiss Confederation (Europe)`
    Canton,
    /// Examples:
    /// * `Abidjan` in `The Republic of Côte d'Ivoire (Africa)`
    /// * `Yamoussoukro` in `The Republic of Côte d'Ivoire (Africa)`
    /// * `Chukotskiy avtonomnyy okrug` in `The Russian Federation (Europe)`
    AutonomousDistrict,
    /// Examples:
    /// * `Guangxi` in `The People's Republic of China (Asia)`
    /// * `Nei Mongol (mn)` in `The People's Republic of China (Asia)`
    /// * `Ningxia` in `The People's Republic of China (Asia)`
    AutonomousRegion,
    /// Examples:
    /// * `Xianggang (zh) **` in `The People's Republic of China (Asia)`
    /// * `Aomen (zh) ***` in `The People's Republic of China (Asia)`
    /// * `Oecussi` in `The Democratic Republic of Timor-Leste (Asia)`
    SpecialAdministrativeRegion,
    /// Examples:
    /// * `Distrito Capital de Bogotá` in `The Republic of Colombia (Americas)`
    /// * `Distrito Federal` in `The Bolivarian Republic of Venezuela (Americas)`
    CapitalDistrict,
    /// Examples:
    /// * `Ilhas de Barlavento` in `The Republic of Cabo Verde (Africa)`
    /// * `Ilhas de Sotavento` in `The Republic of Cabo Verde (Africa)`
    /// * `Central` in `The Republic of Uganda (Africa)`
    GeographicalRegion,
    /// Examples:
    /// * `Praha, Hlavní město` in `The Czech Republic (Europe)`
    /// * `Budapest` in `Hungary (Europe)`
    /// * `평양직할시` in `The Democratic People's Republic of Korea (Asia)`
    CapitalCity,
    /// Examples:
    /// * `Alutaguse` in `The Republic of Estonia (Europe)`
    /// * `Anija` in `The Republic of Estonia (Europe)`
    /// * `Antsla` in `The Republic of Estonia (Europe)`
    RuralMunicipality,
    /// Examples:
    /// * `Haapsalu` in `The Republic of Estonia (Europe)`
    /// * `Keila` in `The Republic of Estonia (Europe)`
    /// * `Kohtla-Järve` in `The Republic of Estonia (Europe)`
    UrbanMunicipality,
    /// Examples:
    /// * `Andalucía` in `The Kingdom of Spain (Europe)`
    /// * `Aragón` in `The Kingdom of Spain (Europe)`
    /// * `Principado de Asturias` in `The Kingdom of Spain (Europe)`
    AutonomousCommunity,
    /// Examples:
    /// * `Ceuta` in `The Kingdom of Spain (Europe)`
    /// * `Melilla` in `The Kingdom of Spain (Europe)`
    AutonomousCityInNorthAfrica,
    /// Examples:
    /// * `Adis Abeba` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Dire Dawa` in `The Federal Democratic Republic of Ethiopia (Africa)`
    Administration,
    /// Examples:
    /// * `Afar` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Amara` in `The Federal Democratic Republic of Ethiopia (Africa)`
    /// * `Binshangul Gumuz` in `The Federal Democratic Republic of Ethiopia (Africa)`
    RegionalState,
    /// Examples:
    /// * `Ain` in `The French Republic (Europe)`
    /// * `Aisne` in `The French Republic (Europe)`
    /// * `Allier` in `The French Republic (Europe)`
    MetropolitanDepartment,
    /// Examples:
    /// * `Corse` in `The French Republic (Europe)`
    /// * `Métropole de Lyon` in `The French Republic (Europe)`
    /// * `Paris` in `The French Republic (Europe)`
    MetropolitanCollectivityWithSpecialStatus,
    /// Examples:
    /// * `Alsace` in `The French Republic (Europe)`
    EuropeanCollectivity,
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
    /// * `Auvergne-Rhône-Alpes` in `The French Republic (Europe)`
    /// * `Bourgogne-Franche-Comté` in `The French Republic (Europe)`
    /// * `Saint-Barthélemy` in `The French Republic (Europe)`
    MetropolitanRegion,
    /// Examples:
    /// * `Polynésie française` in `The French Republic (Europe)`
    /// * `Saint-Pierre-et-Miquelon` in `The French Republic (Europe)`
    /// * `Terres Australes Françaises` in `The French Republic (Europe)`
    OverseasCollectivity,
    /// Examples:
    /// * `Aberdeenshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Aberdeen City` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Argyll and Bute` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    CouncilArea,
    /// Examples:
    /// * `Isle of Anglesey [Sir Ynys Môn GB-YNM]` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bath and North East Somerset` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Blackburn with Darwen` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    UnitaryAuthority,
    /// Examples:
    /// * `Barking and Dagenham` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Brent` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bexley` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    LondonBorough,
    /// Examples:
    /// * `Birmingham` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Barnsley` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Bolton` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    MetropolitanDistrict,
    /// Examples:
    /// * `Buckinghamshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Cambridgeshire` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Cumbria` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    TwoTierCounty,
    /// Examples:
    /// * `England` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Scotland` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    /// * `Wales` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    Country,
    /// Examples:
    /// * `London, City of` in `The United Kingdom of Great Britain and Northern Ireland (Europe)`
    CityCorporation,
    /// Examples:
    /// * `Région de Boké` in `The Republic of Guinea (Africa)`
    /// * `Région de Kindia` in `The Republic of Guinea (Africa)`
    /// * `Région de Faranah` in `The Republic of Guinea (Africa)`
    AdministrativeRegion,
    /// Examples:
    /// * `Ágion Óros` in `The Hellenic Republic (Europe)`
    SelfGovernedPart,
    /// Examples:
    /// * `Bissau` in `The Republic of Guinea-Bissau (Africa)`
    AutonomousSector,
    /// Examples:
    /// * `Békéscsaba` in `Hungary (Europe)`
    /// * `Debrecen` in `Hungary (Europe)`
    /// * `Dunaújváros` in `Hungary (Europe)`
    CityWithCountyRights,
    /// Examples:
    /// * `Daerah Khusus Ibukota Jakarta` in `The Republic of Indonesia (Asia)`
    CapitalRegion,
    /// Examples:
    /// * `Jawa` in `The Republic of Indonesia (Asia)`
    /// * `Kalimantan` in `The Republic of Indonesia (Asia)`
    /// * `Kepulauan Maluku` in `The Republic of Indonesia (Asia)`
    GeographicalUnit,
    /// Examples:
    /// * `Yogyakarta` in `The Republic of Indonesia (Asia)`
    SpecialRegion,
    /// Examples:
    /// * `Chandigarh` in `The Republic of India (Asia)`
    /// * `Dādra and Nagar Haveli and Damān and Diu` in `The Republic of India (Asia)`
    /// * `Delhi` in `The Republic of India (Asia)`
    UnionTerritory,
    /// Examples:
    /// * `Agrigento` in `The Italian Republic (Europe)`
    /// * `Caltanissetta` in `The Italian Republic (Europe)`
    /// * `Enna` in `The Italian Republic (Europe)`
    FreeMunicipalConsortium,
    /// Examples:
    /// * `Bari` in `The Italian Republic (Europe)`
    /// * `Bologna` in `The Italian Republic (Europe)`
    /// * `Cagliari` in `The Italian Republic (Europe)`
    MetropolitanCity,
    /// Examples:
    /// * `Bolzano` in `The Italian Republic (Europe)`
    /// * `Trento` in `The Italian Republic (Europe)`
    /// * `Косово и Метохија` in `The Republic of Serbia (Europe)`
    AutonomousProvince,
    /// Examples:
    /// * `Gorizia` in `The Italian Republic (Europe)`
    /// * `Pordenone` in `The Italian Republic (Europe)`
    /// * `Trieste` in `The Italian Republic (Europe)`
    DecentralizedRegionalEntity,
    /// Examples:
    /// * `Phnom Penh [Phnum Pénh]` in `The Kingdom of Cambodia (Asia)`
    AutonomousMunicipality,
    /// Examples:
    /// * `Gilbert Islands` in `The Republic of Kiribati (Oceania)`
    /// * `Line Islands` in `The Republic of Kiribati (Oceania)`
    /// * `Phoenix Islands` in `The Republic of Kiribati (Oceania)`
    GroupOfIslands,
    /// Examples:
    /// * `라선특별시` in `The Democratic People's Republic of Korea (Asia)`
    /// * `Seoul Teugbyeolsi [Seoul-T'ukpyolshi]` in `The Republic of Korea (Asia)`
    SpecialCity,
    /// Examples:
    /// * `Jejudo [Cheju-do]` in `The Republic of Korea (Asia)`
    SpecialSelfGoverningProvince,
    /// Examples:
    /// * `세종특별자치시` in `The Republic of Korea (Asia)`
    SpecialSelfGoverningCity,
    /// Examples:
    /// * `Akmenės rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Alytaus rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Anykščių rajono savivaldybė` in `The Republic of Lithuania (Europe)`
    DistrictMunicipality,
    /// Examples:
    /// * `Alytaus miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Kauno miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    /// * `Klaipėdos miesto savivaldybė` in `The Republic of Lithuania (Europe)`
    CityMunicipality,
    /// Examples:
    /// * `Daugavpils` in `The Republic of Latvia (Europe)`
    /// * `Jelgava` in `The Republic of Latvia (Europe)`
    /// * `Jurmala` in `The Republic of Latvia (Europe)`
    StateCity,
    /// Examples:
    /// * `Banghazi` in `The State of Libya (Africa)`
    /// * `Al Butnan` in `The State of Libya (Africa)`
    /// * `Darnah` in `The State of Libya (Africa)`
    Popularate,
    /// Examples:
    /// * `La Colle` in `The Principality of Monaco (Europe)`
    /// * `La Condamine` in `The Principality of Monaco (Europe)`
    /// * `Fontvieille` in `The Principality of Monaco (Europe)`
    Quarter,
    /// Examples:
    /// * `Gagauzia, Unitate Teritoriala Autonoma (UTAG)` in `The Republic of Moldova (Europe)`
    AutonomousTerritorialUnit,
    /// Examples:
    /// * `Stînga Nistrului, unitatea teritoriala din` in `The Republic of Moldova (Europe)`
    TerritorialUnit,
    /// Examples:
    /// * `Ralik Chain` in `The Republic of the Marshall Islands (Oceania)`
    /// * `Ratak Chain` in `The Republic of the Marshall Islands (Oceania)`
    ChainOfIslands,
    /// Examples:
    /// * `Attard` in `The Republic of Malta (Europe)`
    /// * `Balzan` in `The Republic of Malta (Europe)`
    /// * `Birgu` in `The Republic of Malta (Europe)`
    LocalCouncil,
    /// Examples:
    /// * `Ariatholhu Dhekunuburi` in `The Republic of Maldives (Asia)`
    /// * `Ariatholhu Uthuruburi` in `The Republic of Maldives (Asia)`
    /// * `Faadhippolhu` in `The Republic of Maldives (Asia)`
    AdministrativeAtoll,
    /// Examples:
    /// * `Ciudad de México` in `The United Mexican States (Americas)`
    /// * `Asunción` in `The Republic of Paraguay (Americas)`
    Capital,
    /// Examples:
    /// * `Wilayah Persekutuan Kuala Lumpur` in `Malaysia (Asia)`
    /// * `Wilayah Persekutuan Labuan` in `Malaysia (Asia)`
    /// * `Wilayah Persekutuan Putrajaya` in `Malaysia (Asia)`
    FederalTerritory,
    /// Examples:
    /// * `Niamey` in `The Republic of the Niger (Africa)`
    UrbanCommunity,
    /// Examples:
    /// * `Abuja Capital Territory` in `The Federal Republic of Nigeria (Africa)`
    /// * `Capital Territory (Honiara)` in `The Solomon Islands (Oceania)`
    /// * `Dushanbe` in `The Republic of Tajikistan (Asia)`
    CapitalTerritory,
    /// Examples:
    /// * `Svalbard (Arctic Region)` in `The Kingdom of Norway (Europe)`
    /// * `Jan Mayen (Arctic Region)` in `The Kingdom of Norway (Europe)`
    ArcticRegion,
    /// Examples:
    /// * `मध्यमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `मध्य-पश्चिमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `पश्चिमाञ्चल विकास क्षेत्र` in `The Federal Democratic Republic of Nepal (Asia)`
    DevelopmentRegion,
    /// Examples:
    /// * `Bagmati` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `Bheri` in `The Federal Democratic Republic of Nepal (Asia)`
    /// * `Dhawalagiri` in `The Federal Democratic Republic of Nepal (Asia)`
    Zone,
    /// Examples:
    /// * `Chatham Islands Territory` in `New Zealand (Oceania)`
    SpecialIslandAuthority,
    /// Examples:
    /// * `Emberá-Wounaan` in `The Republic of Panamá (Americas)`
    /// * `Guna Yala` in `The Republic of Panamá (Americas)`
    /// * `Ngäbe-Buglé` in `The Republic of Panamá (Americas)`
    IndigenousRegion,
    /// Examples:
    /// * `Gilgit-Baltistan` in `The Islamic Republic of Pakistan (Asia)`
    /// * `Azad Kashmir` in `The Islamic Republic of Pakistan (Asia)`
    PakistanAdministeredArea,
    /// Examples:
    /// * `Islamabad` in `The Islamic Republic of Pakistan (Asia)`
    FederalCapitalTerritory,
    /// Examples:
    /// * `Dolnośląskie` in `The Republic of Poland (Europe)`
    /// * `Kujawsko-pomorskie` in `The Republic of Poland (Europe)`
    /// * `Lubelskie` in `The Republic of Poland (Europe)`
    Voivodeship,
    /// Examples:
    /// * `Adygeya, Respublika` in `The Russian Federation (Europe)`
    /// * `Altay, Respublika` in `The Russian Federation (Europe)`
    /// * `Bashkortostan, Respublika` in `The Russian Federation (Europe)`
    Republic,
    /// Examples:
    /// * `Altayskiy kray` in `The Russian Federation (Europe)`
    /// * `Kamchatskaya oblast'` in `The Russian Federation (Europe)`
    /// * `Krasnodarskiy kray` in `The Russian Federation (Europe)`
    AdministrativeTerritory,
    /// Examples:
    /// * `Moskva` in `The Russian Federation (Europe)`
    /// * `Sankt-Peterburg` in `The Russian Federation (Europe)`
    AutonomousCity,
    /// Examples:
    /// * `Ascension` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    /// * `Saint Helena` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    /// * `Tristan da Cunha` in `Saint Helena, Ascension and Tristan da Cunha (Africa)`
    GeographicalEntity,
    /// Examples:
    /// * `Western Area (Freetown)` in `The Republic of Sierra Leone (Africa)`
    Area,
    /// Examples:
    /// * `Krung Thep Maha Nakhon [Bangkok]` in `The Kingdom of Thailand (Asia)`
    MetropolitanAdministration,
    /// Examples:
    /// * `Phatthaya` in `The Kingdom of Thailand (Asia)`
    SpecialAdministrativeCity,
    /// Examples:
    /// * `Nohiyahoi Tobei Jumhurí` in `The Republic of Tajikistan (Asia)`
    DistrictsUnderRepublicAdministration,
    /// Examples:
    /// * `Arima` in `The Republic of Trinidad and Tobago (Americas)`
    /// * `Chaguanas` in `The Republic of Trinidad and Tobago (Americas)`
    /// * `Point Fortin` in `The Republic of Trinidad and Tobago (Americas)`
    Borough,
    /// Examples:
    /// * `Tobago` in `The Republic of Trinidad and Tobago (Americas)`
    Ward,
    /// Examples:
    /// * `Funafuti` in `Tuvalu (Oceania)`
    TownCouncil,
    /// Examples:
    /// * `Niutao` in `Tuvalu (Oceania)`
    /// * `Nukufetau` in `Tuvalu (Oceania)`
    /// * `Nukulaelae` in `Tuvalu (Oceania)`
    IslandCouncil,
    /// Examples:
    /// * `American Samoa` in `The United States of America (Americas)`
    /// * `Guam` in `The United States of America (Americas)`
    /// * `Northern Mariana Islands` in `The United States of America (Americas)`
    OutlyingArea,
    /// Examples:
    /// * `Dependencias Federales` in `The Bolivarian Republic of Venezuela (Americas)`
    FederalDependency,
    /// Examples:
    /// * `Alo` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    /// * `Sigave` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    /// * `Uvea` in `The Territory of the Wallis and Futuna Islands (Oceania)`
    AdministrativePrecinct,
}

#[cfg(feature = "subdivisions")]
impl TryFrom<&str> for SubdivisionType {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "parish" => Ok(Self::Parish),
            "emirate" => Ok(Self::Emirate),
            "province" => Ok(Self::Province),
            "dependency" => Ok(Self::Dependency),
            "county" => Ok(Self::County),
            "region" => Ok(Self::Region),
            "city" => Ok(Self::City),
            "state" => Ok(Self::State),
            "territory" => Ok(Self::Territory),
            "rayon" => Ok(Self::Rayon),
            "municipality" => Ok(Self::Municipality),
            "autonomous republic" => Ok(Self::AutonomousRepublic),
            "entity" => Ok(Self::Entity),
            "district with special status" => Ok(Self::DistrictWithSpecialStatus),
            "district" => Ok(Self::District),
            "division" => Ok(Self::Division),
            "governorate" => Ok(Self::Governorate),
            "departments" => Ok(Self::Departments),
            "department" => Ok(Self::Department),
            "special municipality" => Ok(Self::SpecialMunicipality),
            "federal district" => Ok(Self::FederalDistrict),
            "island" => Ok(Self::Island),
            "town" => Ok(Self::Town),
            "oblast" => Ok(Self::Oblast),
            "prefecture" => Ok(Self::Prefecture),
            "commune" => Ok(Self::Commune),
            "economic prefecture" => Ok(Self::EconomicPrefecture),
            "canton" => Ok(Self::Canton),
            "autonomous district" => Ok(Self::AutonomousDistrict),
            "autonomous region" => Ok(Self::AutonomousRegion),
            "special administrative region" => Ok(Self::SpecialAdministrativeRegion),
            "capital district" => Ok(Self::CapitalDistrict),
            "geographical region" => Ok(Self::GeographicalRegion),
            "capital city" => Ok(Self::CapitalCity),
            "rural municipality" => Ok(Self::RuralMunicipality),
            "urban municipality" => Ok(Self::UrbanMunicipality),
            "autonomous community" => Ok(Self::AutonomousCommunity),
            "autonomous city in north africa" => Ok(Self::AutonomousCityInNorthAfrica),
            "administration" => Ok(Self::Administration),
            "regional state" => Ok(Self::RegionalState),
            "metropolitan department" => Ok(Self::MetropolitanDepartment),
            "metropolitan collectivity with special status" => {
                Ok(Self::MetropolitanCollectivityWithSpecialStatus)
            }
            "european collectivity" => Ok(Self::EuropeanCollectivity),
            "overseas departmental collectivity" => Ok(Self::OverseasDepartmentalCollectivity),
            "overseas unique territorial collectivity" => {
                Ok(Self::OverseasUniqueTerritorialCollectivity)
            }
            "metropolitan region" => Ok(Self::MetropolitanRegion),
            "overseas collectivity" => Ok(Self::OverseasCollectivity),
            "council area" => Ok(Self::CouncilArea),
            "unitary authority" => Ok(Self::UnitaryAuthority),
            "london borough" => Ok(Self::LondonBorough),
            "metropolitan district" => Ok(Self::MetropolitanDistrict),
            "two-tier county" => Ok(Self::TwoTierCounty),
            "country" => Ok(Self::Country),
            "city corporation" => Ok(Self::CityCorporation),
            "administrative region" => Ok(Self::AdministrativeRegion),
            "self-governed part" => Ok(Self::SelfGovernedPart),
            "autonomous sector" => Ok(Self::AutonomousSector),
            "city with county rights" => Ok(Self::CityWithCountyRights),
            "capital region" => Ok(Self::CapitalRegion),
            "geographical unit" => Ok(Self::GeographicalUnit),
            "special region" => Ok(Self::SpecialRegion),
            "union territory" => Ok(Self::UnionTerritory),
            "free municipal consortium" => Ok(Self::FreeMunicipalConsortium),
            "metropolitan city" => Ok(Self::MetropolitanCity),
            "autonomous province" => Ok(Self::AutonomousProvince),
            "decentralized regional entity" => Ok(Self::DecentralizedRegionalEntity),
            "autonomous municipality" => Ok(Self::AutonomousMunicipality),
            "group of islands" => Ok(Self::GroupOfIslands),
            "special city" => Ok(Self::SpecialCity),
            "special self-governing province" => Ok(Self::SpecialSelfGoverningProvince),
            "special self-governing city" => Ok(Self::SpecialSelfGoverningCity),
            "district municipality" => Ok(Self::DistrictMunicipality),
            "city municipality" => Ok(Self::CityMunicipality),
            "state city" => Ok(Self::StateCity),
            "popularate" => Ok(Self::Popularate),
            "quarter" => Ok(Self::Quarter),
            "autonomous territorial unit" => Ok(Self::AutonomousTerritorialUnit),
            "territorial unit" => Ok(Self::TerritorialUnit),
            "chain of islands" => Ok(Self::ChainOfIslands),
            "local council" => Ok(Self::LocalCouncil),
            "administrative atoll" => Ok(Self::AdministrativeAtoll),
            "capital" => Ok(Self::Capital),
            "federal territory" => Ok(Self::FederalTerritory),
            "urban community" => Ok(Self::UrbanCommunity),
            "capital territory" => Ok(Self::CapitalTerritory),
            "arctic region" => Ok(Self::ArcticRegion),
            "development region" => Ok(Self::DevelopmentRegion),
            "zone" => Ok(Self::Zone),
            "special island authority" => Ok(Self::SpecialIslandAuthority),
            "indigenous region" => Ok(Self::IndigenousRegion),
            "pakistan administered area" => Ok(Self::PakistanAdministeredArea),
            "federal capital territory" => Ok(Self::FederalCapitalTerritory),
            "voivodeship" => Ok(Self::Voivodeship),
            "republic" => Ok(Self::Republic),
            "administrative territory" => Ok(Self::AdministrativeTerritory),
            "autonomous city" => Ok(Self::AutonomousCity),
            "geographical entity" => Ok(Self::GeographicalEntity),
            "area" => Ok(Self::Area),
            "metropolitan administration" => Ok(Self::MetropolitanAdministration),
            "special administrative city" => Ok(Self::SpecialAdministrativeCity),
            "districts under republic administration" => {
                Ok(Self::DistrictsUnderRepublicAdministration)
            }
            "borough" => Ok(Self::Borough),
            "ward" => Ok(Self::Ward),
            "town council" => Ok(Self::TownCouncil),
            "island council" => Ok(Self::IslandCouncil),
            "outlying area" => Ok(Self::OutlyingArea),
            "federal dependency" => Ok(Self::FederalDependency),
            "administrative precinct" => Ok(Self::AdministrativePrecinct),
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
            Self::Parish => "parish",
            Self::Emirate => "emirate",
            Self::Province => "province",
            Self::Dependency => "dependency",
            Self::County => "county",
            Self::Region => "region",
            Self::City => "city",
            Self::State => "state",
            Self::Territory => "territory",
            Self::Rayon => "rayon",
            Self::Municipality => "municipality",
            Self::AutonomousRepublic => "autonomous_republic",
            Self::Entity => "entity",
            Self::DistrictWithSpecialStatus => "district_with_special_status",
            Self::District => "district",
            Self::Division => "division",
            Self::Governorate => "governorate",
            Self::Departments => "departments",
            Self::Department => "department",
            Self::SpecialMunicipality => "special_municipality",
            Self::FederalDistrict => "federal_district",
            Self::Island => "island",
            Self::Town => "town",
            Self::Oblast => "oblast",
            Self::Prefecture => "prefecture",
            Self::Commune => "commune",
            Self::EconomicPrefecture => "economic_prefecture",
            Self::Canton => "canton",
            Self::AutonomousDistrict => "autonomous_district",
            Self::AutonomousRegion => "autonomous_region",
            Self::SpecialAdministrativeRegion => "special_administrative_region",
            Self::CapitalDistrict => "capital_district",
            Self::GeographicalRegion => "geographical_region",
            Self::CapitalCity => "capital_city",
            Self::RuralMunicipality => "rural_municipality",
            Self::UrbanMunicipality => "urban_municipality",
            Self::AutonomousCommunity => "autonomous_community",
            Self::AutonomousCityInNorthAfrica => "autonomous_city_in_north_africa",
            Self::Administration => "administration",
            Self::RegionalState => "regional_state",
            Self::MetropolitanDepartment => "metropolitan_department",
            Self::MetropolitanCollectivityWithSpecialStatus => {
                "metropolitan_collectivity_with_special_status"
            }
            Self::EuropeanCollectivity => "european_collectivity",
            Self::OverseasDepartmentalCollectivity => "overseas_departmental_collectivity",
            Self::OverseasUniqueTerritorialCollectivity => {
                "overseas_unique_territorial_collectivity"
            }
            Self::MetropolitanRegion => "metropolitan_region",
            Self::OverseasCollectivity => "overseas_collectivity",
            Self::CouncilArea => "council_area",
            Self::UnitaryAuthority => "unitary_authority",
            Self::LondonBorough => "london_borough",
            Self::MetropolitanDistrict => "metropolitan_district",
            Self::TwoTierCounty => "two-tier_county",
            Self::Country => "country",
            Self::CityCorporation => "city_corporation",
            Self::AdministrativeRegion => "administrative_region",
            Self::SelfGovernedPart => "self-governed_part",
            Self::AutonomousSector => "autonomous_sector",
            Self::CityWithCountyRights => "city_with_county_rights",
            Self::CapitalRegion => "capital_region",
            Self::GeographicalUnit => "geographical_unit",
            Self::SpecialRegion => "special_region",
            Self::UnionTerritory => "union_territory",
            Self::FreeMunicipalConsortium => "free_municipal_consortium",
            Self::MetropolitanCity => "metropolitan_city",
            Self::AutonomousProvince => "autonomous_province",
            Self::DecentralizedRegionalEntity => "decentralized_regional_entity",
            Self::AutonomousMunicipality => "autonomous_municipality",
            Self::GroupOfIslands => "group_of_islands",
            Self::SpecialCity => "special_city",
            Self::SpecialSelfGoverningProvince => "special_self-governing_province",
            Self::SpecialSelfGoverningCity => "special_self-governing_city",
            Self::DistrictMunicipality => "district_municipality",
            Self::CityMunicipality => "city_municipality",
            Self::StateCity => "state_city",
            Self::Popularate => "popularate",
            Self::Quarter => "quarter",
            Self::AutonomousTerritorialUnit => "autonomous_territorial_unit",
            Self::TerritorialUnit => "territorial_unit",
            Self::ChainOfIslands => "chain_of_islands",
            Self::LocalCouncil => "local_council",
            Self::AdministrativeAtoll => "administrative_atoll",
            Self::Capital => "capital",
            Self::FederalTerritory => "federal_territory",
            Self::UrbanCommunity => "urban_community",
            Self::CapitalTerritory => "capital_territory",
            Self::ArcticRegion => "arctic_region",
            Self::DevelopmentRegion => "development_region",
            Self::Zone => "zone",
            Self::SpecialIslandAuthority => "special_island_authority",
            Self::IndigenousRegion => "indigenous_region",
            Self::PakistanAdministeredArea => "pakistan_administered_area",
            Self::FederalCapitalTerritory => "federal_capital_territory",
            Self::Voivodeship => "voivodeship",
            Self::Republic => "republic",
            Self::AdministrativeTerritory => "administrative_territory",
            Self::AutonomousCity => "autonomous_city",
            Self::GeographicalEntity => "geographical_entity",
            Self::Area => "area",
            Self::MetropolitanAdministration => "metropolitan_administration",
            Self::SpecialAdministrativeCity => "special_administrative_city",
            Self::DistrictsUnderRepublicAdministration => "districts_under_republic_administration",
            Self::Borough => "borough",
            Self::Ward => "ward",
            Self::TownCouncil => "town_council",
            Self::IslandCouncil => "island_council",
            Self::OutlyingArea => "outlying_area",
            Self::FederalDependency => "federal_dependency",
            Self::AdministrativePrecinct => "administrative_precinct",
        }
        .to_string()
    }
}
