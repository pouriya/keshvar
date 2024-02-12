// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/search.rs`)

#[cfg(feature = "search-iso-short-name")]
use crate::Alpha2;
#[cfg(feature = "search-iso-short-name")]
use hashbrown::HashMap;
#[cfg(feature = "search-iso-short-name")]
use lazy_static::lazy_static;
#[cfg(feature = "search-iso-short-name")]
lazy_static! { pub static ref SUPPORTED_ISO_SHORT_NAMES: HashMap<&'static str, Alpha2> = HashMap::from([
    #[cfg(feature = "ad")] // The Principality of Andorra (Europe)
    ("andorra", Alpha2::AD),
    #[cfg(feature = "ae")] // The United Arab Emirates (Asia)
    ("united arab emirates", Alpha2::AE),
    #[cfg(feature = "af")] // The Islamic Republic of Afghanistan (Asia)
    ("afghanistan", Alpha2::AF),
    #[cfg(feature = "ag")] // Antigua and Barbuda (Americas)
    ("antigua and barbuda", Alpha2::AG),
    #[cfg(feature = "ai")] // Anguilla (Americas)
    ("anguilla", Alpha2::AI),
    #[cfg(feature = "al")] // The Republic of Albania (Europe)
    ("albania", Alpha2::AL),
    #[cfg(feature = "am")] // The Republic of Armenia (Asia)
    ("armenia", Alpha2::AM),
    #[cfg(feature = "ao")] // The Republic of Angola (Africa)
    ("angola", Alpha2::AO),
    #[cfg(feature = "aq")] // Antarctica
    ("antarctica", Alpha2::AQ),
    #[cfg(feature = "ar")] // The Argentine Republic (Americas)
    ("argentina", Alpha2::AR),
    #[cfg(feature = "as")] // The Territory of American Samoa (Oceania)
    ("american samoa", Alpha2::AS),
    #[cfg(feature = "at")] // The Republic of Austria (Europe)
    ("austria", Alpha2::AT),
    #[cfg(feature = "au")] // The Commonwealth of Australia (Oceania)
    ("australia", Alpha2::AU),
    #[cfg(feature = "aw")] // Aruba (Americas)
    ("aruba", Alpha2::AW),
    #[cfg(feature = "ax")] // Åland (Europe)
    ("åland islands", Alpha2::AX),
    #[cfg(feature = "az")] // The Republic of Azerbaijan (Asia)
    ("azerbaijan", Alpha2::AZ),
    #[cfg(feature = "ba")] // Bosnia and Herzegovina (Europe)
    ("bosnia and herzegovina", Alpha2::BA),
    #[cfg(feature = "bb")] // Barbados (Americas)
    ("barbados", Alpha2::BB),
    #[cfg(feature = "bd")] // The People's Republic of Bangladesh (Asia)
    ("bangladesh", Alpha2::BD),
    #[cfg(feature = "be")] // The Kingdom of Belgium (Europe)
    ("belgium", Alpha2::BE),
    #[cfg(feature = "bf")] // Burkina Faso (Africa)
    ("burkina faso", Alpha2::BF),
    #[cfg(feature = "bg")] // The Republic of Bulgaria (Europe)
    ("bulgaria", Alpha2::BG),
    #[cfg(feature = "bh")] // The Kingdom of Bahrain (Asia)
    ("bahrain", Alpha2::BH),
    #[cfg(feature = "bi")] // The Republic of Burundi (Africa)
    ("burundi", Alpha2::BI),
    #[cfg(feature = "bj")] // The Republic of Benin (Africa)
    ("benin", Alpha2::BJ),
    #[cfg(feature = "bl")] // The Collectivity of Saint-Barthélemy (Americas)
    ("saint barthélemy", Alpha2::BL),
    #[cfg(feature = "bm")] // Bermuda (Americas)
    ("bermuda", Alpha2::BM),
    #[cfg(feature = "bn")] // The Nation of Brunei, the Abode of Peace (Asia)
    ("brunei darussalam", Alpha2::BN),
    #[cfg(feature = "bo")] // The Plurinational State of Bolivia (Americas)
    ("bolivia (plurinational state of)", Alpha2::BO),
    #[cfg(feature = "bq")] // Bonaire, Sint Eustatius and Saba (Americas)
    ("bonaire, sint eustatius and saba", Alpha2::BQ),
    #[cfg(feature = "br")] // The Federative Republic of Brazil (Americas)
    ("brazil", Alpha2::BR),
    #[cfg(feature = "bs")] // The Commonwealth of The Bahamas (Americas)
    ("bahamas", Alpha2::BS),
    #[cfg(feature = "bt")] // The Kingdom of Bhutan (Asia)
    ("bhutan", Alpha2::BT),
    #[cfg(feature = "bv")] // Bouvet Island
    ("bouvet island", Alpha2::BV),
    #[cfg(feature = "bw")] // The Republic of Botswana (Africa)
    ("botswana", Alpha2::BW),
    #[cfg(feature = "by")] // The Republic of Belarus (Europe)
    ("belarus", Alpha2::BY),
    #[cfg(feature = "bz")] // Belize (Americas)
    ("belize", Alpha2::BZ),
    #[cfg(feature = "ca")] // Canada (Americas)
    ("canada", Alpha2::CA),
    #[cfg(feature = "cc")] // The Territory of Cocos (Keeling) Islands (Oceania)
    ("cocos (keeling) islands", Alpha2::CC),
    #[cfg(feature = "cd")] // The Democratic Republic of the Congo (Africa)
    ("congo (democratic republic of the)", Alpha2::CD),
    #[cfg(feature = "cf")] // The Central African Republic (Africa)
    ("central african republic", Alpha2::CF),
    #[cfg(feature = "cg")] // The Republic of the Congo (Africa)
    ("congo", Alpha2::CG),
    #[cfg(feature = "ch")] // The Swiss Confederation (Europe)
    ("switzerland", Alpha2::CH),
    #[cfg(feature = "ci")] // The Republic of Côte d'Ivoire (Africa)
    ("côte d'ivoire", Alpha2::CI),
    #[cfg(feature = "ck")] // The Cook Islands (Oceania)
    ("cook islands", Alpha2::CK),
    #[cfg(feature = "cl")] // The Republic of Chile (Americas)
    ("chile", Alpha2::CL),
    #[cfg(feature = "cm")] // The Republic of Cameroon (Africa)
    ("cameroon", Alpha2::CM),
    #[cfg(feature = "cn")] // The People's Republic of China (Asia)
    ("china", Alpha2::CN),
    #[cfg(feature = "co")] // The Republic of Colombia (Americas)
    ("colombia", Alpha2::CO),
    #[cfg(feature = "cr")] // The Republic of Costa Rica (Americas)
    ("costa rica", Alpha2::CR),
    #[cfg(feature = "cu")] // The Republic of Cuba (Americas)
    ("cuba", Alpha2::CU),
    #[cfg(feature = "cv")] // The Republic of Cabo Verde (Africa)
    ("cabo verde", Alpha2::CV),
    #[cfg(feature = "cw")] // The Country of Curaçao (Americas)
    ("curaçao", Alpha2::CW),
    #[cfg(feature = "cx")] // The Territory of Christmas Island (Oceania)
    ("christmas island", Alpha2::CX),
    #[cfg(feature = "cy")] // The Republic of Cyprus (Asia)
    ("cyprus", Alpha2::CY),
    #[cfg(feature = "cz")] // The Czech Republic (Europe)
    ("czechia", Alpha2::CZ),
    #[cfg(feature = "de")] // The Federal Republic of Germany (Europe)
    ("germany", Alpha2::DE),
    #[cfg(feature = "dj")] // The Republic of Djibouti (Africa)
    ("djibouti", Alpha2::DJ),
    #[cfg(feature = "dk")] // The Kingdom of Denmark (Europe)
    ("denmark", Alpha2::DK),
    #[cfg(feature = "dm")] // The Commonwealth of Dominica (Americas)
    ("dominica", Alpha2::DM),
    #[cfg(feature = "do")] // The Dominican Republic (Americas)
    ("dominican republic", Alpha2::DO),
    #[cfg(feature = "dz")] // The People's Democratic Republic of Algeria (Africa)
    ("algeria", Alpha2::DZ),
    #[cfg(feature = "ec")] // The Republic of Ecuador (Americas)
    ("ecuador", Alpha2::EC),
    #[cfg(feature = "ee")] // The Republic of Estonia (Europe)
    ("estonia", Alpha2::EE),
    #[cfg(feature = "eg")] // The Arab Republic of Egypt (Africa)
    ("egypt", Alpha2::EG),
    #[cfg(feature = "eh")] // The Sahrawi Arab Democratic Republic (Africa)
    ("western sahara", Alpha2::EH),
    #[cfg(feature = "er")] // The State of Eritrea (Africa)
    ("eritrea", Alpha2::ER),
    #[cfg(feature = "es")] // The Kingdom of Spain (Europe)
    ("spain", Alpha2::ES),
    #[cfg(feature = "et")] // The Federal Democratic Republic of Ethiopia (Africa)
    ("ethiopia", Alpha2::ET),
    #[cfg(feature = "fi")] // The Republic of Finland (Europe)
    ("finland", Alpha2::FI),
    #[cfg(feature = "fj")] // The Republic of Fiji (Oceania)
    ("fiji", Alpha2::FJ),
    #[cfg(feature = "fk")] // The Falkland Islands (Americas)
    ("falkland islands (malvinas)", Alpha2::FK),
    #[cfg(feature = "fm")] // The Federated States of Micronesia (Oceania)
    ("micronesia (federated states of)", Alpha2::FM),
    #[cfg(feature = "fo")] // The Faroe Islands (Europe)
    ("faroe islands", Alpha2::FO),
    #[cfg(feature = "fr")] // The French Republic (Europe)
    ("france", Alpha2::FR),
    #[cfg(feature = "ga")] // The Gabonese Republic (Africa)
    ("gabon", Alpha2::GA),
    #[cfg(feature = "gb")] // The United Kingdom of Great Britain and Northern Ireland (Europe)
    ("united kingdom of great britain and northern ireland", Alpha2::GB),
    #[cfg(feature = "gd")] // Grenada (Americas)
    ("grenada", Alpha2::GD),
    #[cfg(feature = "ge")] // Georgia (Asia)
    ("georgia", Alpha2::GE),
    #[cfg(feature = "gf")] // Guyane (Americas)
    ("french guiana", Alpha2::GF),
    #[cfg(feature = "gg")] // The Bailiwick of Guernsey (Europe)
    ("guernsey", Alpha2::GG),
    #[cfg(feature = "gh")] // The Republic of Ghana (Africa)
    ("ghana", Alpha2::GH),
    #[cfg(feature = "gi")] // Gibraltar (Europe)
    ("gibraltar", Alpha2::GI),
    #[cfg(feature = "gl")] // Kalaallit Nunaat (Americas)
    ("greenland", Alpha2::GL),
    #[cfg(feature = "gm")] // The Republic of The Gambia (Africa)
    ("gambia", Alpha2::GM),
    #[cfg(feature = "gn")] // The Republic of Guinea (Africa)
    ("guinea", Alpha2::GN),
    #[cfg(feature = "gp")] // Guadeloupe (Americas)
    ("guadeloupe", Alpha2::GP),
    #[cfg(feature = "gq")] // The Republic of Equatorial Guinea (Africa)
    ("equatorial guinea", Alpha2::GQ),
    #[cfg(feature = "gr")] // The Hellenic Republic (Europe)
    ("greece", Alpha2::GR),
    #[cfg(feature = "gs")] // South Georgia and the South Sandwich Islands (Americas)
    ("south georgia and the south sandwich islands", Alpha2::GS),
    #[cfg(feature = "gt")] // The Republic of Guatemala (Americas)
    ("guatemala", Alpha2::GT),
    #[cfg(feature = "gu")] // The Territory of Guam (Oceania)
    ("guam", Alpha2::GU),
    #[cfg(feature = "gw")] // The Republic of Guinea-Bissau (Africa)
    ("guinea-bissau", Alpha2::GW),
    #[cfg(feature = "gy")] // The Co-operative Republic of Guyana (Americas)
    ("guyana", Alpha2::GY),
    #[cfg(feature = "hk")] // The Hong Kong Special Administrative Region of China (Asia)
    ("hong kong", Alpha2::HK),
    #[cfg(feature = "hm")] // The Territory of Heard Island and McDonald Islands
    ("heard island and mcdonald islands", Alpha2::HM),
    #[cfg(feature = "hn")] // The Republic of Honduras (Americas)
    ("honduras", Alpha2::HN),
    #[cfg(feature = "hr")] // The Republic of Croatia (Europe)
    ("croatia", Alpha2::HR),
    #[cfg(feature = "ht")] // The Republic of Haiti (Americas)
    ("haiti", Alpha2::HT),
    #[cfg(feature = "hu")] // Hungary (Europe)
    ("hungary", Alpha2::HU),
    #[cfg(feature = "id")] // The Republic of Indonesia (Asia)
    ("indonesia", Alpha2::ID),
    #[cfg(feature = "ie")] // Ireland (Europe)
    ("ireland", Alpha2::IE),
    #[cfg(feature = "il")] // The State of Israel (Asia)
    ("israel", Alpha2::IL),
    #[cfg(feature = "im")] // The Isle of Man (Europe)
    ("isle of man", Alpha2::IM),
    #[cfg(feature = "in")] // The Republic of India (Asia)
    ("india", Alpha2::IN),
    #[cfg(feature = "io")] // The British Indian Ocean Territory (Africa)
    ("british indian ocean territory", Alpha2::IO),
    #[cfg(feature = "iq")] // The Republic of Iraq (Asia)
    ("iraq", Alpha2::IQ),
    #[cfg(feature = "ir")] // The Islamic Republic of Iran (Asia)
    ("iran (islamic republic of)", Alpha2::IR),
    #[cfg(feature = "is")] // Iceland (Europe)
    ("iceland", Alpha2::IS),
    #[cfg(feature = "it")] // The Italian Republic (Europe)
    ("italy", Alpha2::IT),
    #[cfg(feature = "je")] // The Bailiwick of Jersey (Europe)
    ("jersey", Alpha2::JE),
    #[cfg(feature = "jm")] // Jamaica (Americas)
    ("jamaica", Alpha2::JM),
    #[cfg(feature = "jo")] // The Hashemite Kingdom of Jordan (Asia)
    ("jordan", Alpha2::JO),
    #[cfg(feature = "jp")] // Japan (Asia)
    ("japan", Alpha2::JP),
    #[cfg(feature = "ke")] // The Republic of Kenya (Africa)
    ("kenya", Alpha2::KE),
    #[cfg(feature = "kg")] // The Kyrgyz Republic (Asia)
    ("kyrgyzstan", Alpha2::KG),
    #[cfg(feature = "kh")] // The Kingdom of Cambodia (Asia)
    ("cambodia", Alpha2::KH),
    #[cfg(feature = "ki")] // The Republic of Kiribati (Oceania)
    ("kiribati", Alpha2::KI),
    #[cfg(feature = "km")] // The Union of the Comoros (Africa)
    ("comoros", Alpha2::KM),
    #[cfg(feature = "kn")] // Saint Kitts and Nevis (Americas)
    ("saint kitts and nevis", Alpha2::KN),
    #[cfg(feature = "kp")] // The Democratic People's Republic of Korea (Asia)
    ("korea (democratic people's republic of)", Alpha2::KP),
    #[cfg(feature = "kr")] // The Republic of Korea (Asia)
    ("korea (republic of)", Alpha2::KR),
    #[cfg(feature = "kw")] // The State of Kuwait (Asia)
    ("kuwait", Alpha2::KW),
    #[cfg(feature = "ky")] // The Cayman Islands (Americas)
    ("cayman islands", Alpha2::KY),
    #[cfg(feature = "kz")] // The Republic of Kazakhstan (Asia)
    ("kazakhstan", Alpha2::KZ),
    #[cfg(feature = "la")] // The Lao People's Democratic Republic (Asia)
    ("lao people's democratic republic", Alpha2::LA),
    #[cfg(feature = "lb")] // The Lebanese Republic (Asia)
    ("lebanon", Alpha2::LB),
    #[cfg(feature = "lc")] // Saint Lucia (Americas)
    ("saint lucia", Alpha2::LC),
    #[cfg(feature = "li")] // The Principality of Liechtenstein (Europe)
    ("liechtenstein", Alpha2::LI),
    #[cfg(feature = "lk")] // The Democratic Socialist Republic of Sri Lanka (Asia)
    ("sri lanka", Alpha2::LK),
    #[cfg(feature = "lr")] // The Republic of Liberia (Africa)
    ("liberia", Alpha2::LR),
    #[cfg(feature = "ls")] // The Kingdom of Lesotho (Africa)
    ("lesotho", Alpha2::LS),
    #[cfg(feature = "lt")] // The Republic of Lithuania (Europe)
    ("lithuania", Alpha2::LT),
    #[cfg(feature = "lu")] // The Grand Duchy of Luxembourg (Europe)
    ("luxembourg", Alpha2::LU),
    #[cfg(feature = "lv")] // The Republic of Latvia (Europe)
    ("latvia", Alpha2::LV),
    #[cfg(feature = "ly")] // The State of Libya (Africa)
    ("libya", Alpha2::LY),
    #[cfg(feature = "ma")] // The Kingdom of Morocco (Africa)
    ("morocco", Alpha2::MA),
    #[cfg(feature = "mc")] // The Principality of Monaco (Europe)
    ("monaco", Alpha2::MC),
    #[cfg(feature = "md")] // The Republic of Moldova (Europe)
    ("moldova (republic of)", Alpha2::MD),
    #[cfg(feature = "me")] // Montenegro (Europe)
    ("montenegro", Alpha2::ME),
    #[cfg(feature = "mf")] // The Collectivity of Saint-Martin (Americas)
    ("saint martin (french part)", Alpha2::MF),
    #[cfg(feature = "mg")] // The Republic of Madagascar (Africa)
    ("madagascar", Alpha2::MG),
    #[cfg(feature = "mh")] // The Republic of the Marshall Islands (Oceania)
    ("marshall islands", Alpha2::MH),
    #[cfg(feature = "mk")] // The Republic of North Macedonia (Europe)
    ("north macedonia", Alpha2::MK),
    #[cfg(feature = "ml")] // The Republic of Mali (Africa)
    ("mali", Alpha2::ML),
    #[cfg(feature = "mm")] // The Republic of the Union of Myanmar (Asia)
    ("myanmar", Alpha2::MM),
    #[cfg(feature = "mn")] // Mongolia (Asia)
    ("mongolia", Alpha2::MN),
    #[cfg(feature = "mo")] // The Macao Special Administrative Region of China (Asia)
    ("macao", Alpha2::MO),
    #[cfg(feature = "mp")] // The Commonwealth of the Northern Mariana Islands (Oceania)
    ("northern mariana islands", Alpha2::MP),
    #[cfg(feature = "mq")] // Martinique (Americas)
    ("martinique", Alpha2::MQ),
    #[cfg(feature = "mr")] // The Islamic Republic of Mauritania (Africa)
    ("mauritania", Alpha2::MR),
    #[cfg(feature = "ms")] // Montserrat (Americas)
    ("montserrat", Alpha2::MS),
    #[cfg(feature = "mt")] // The Republic of Malta (Europe)
    ("malta", Alpha2::MT),
    #[cfg(feature = "mu")] // The Republic of Mauritius (Africa)
    ("mauritius", Alpha2::MU),
    #[cfg(feature = "mv")] // The Republic of Maldives (Asia)
    ("maldives", Alpha2::MV),
    #[cfg(feature = "mw")] // The Republic of Malawi (Africa)
    ("malawi", Alpha2::MW),
    #[cfg(feature = "mx")] // The United Mexican States (Americas)
    ("mexico", Alpha2::MX),
    #[cfg(feature = "my")] // Malaysia (Asia)
    ("malaysia", Alpha2::MY),
    #[cfg(feature = "mz")] // The Republic of Mozambique (Africa)
    ("mozambique", Alpha2::MZ),
    #[cfg(feature = "na")] // The Republic of Namibia (Africa)
    ("namibia", Alpha2::NA),
    #[cfg(feature = "nc")] // New Caledonia (Oceania)
    ("new caledonia", Alpha2::NC),
    #[cfg(feature = "ne")] // The Republic of the Niger (Africa)
    ("niger", Alpha2::NE),
    #[cfg(feature = "nf")] // The Territory of Norfolk Island (Oceania)
    ("norfolk island", Alpha2::NF),
    #[cfg(feature = "ng")] // The Federal Republic of Nigeria (Africa)
    ("nigeria", Alpha2::NG),
    #[cfg(feature = "ni")] // The Republic of Nicaragua (Americas)
    ("nicaragua", Alpha2::NI),
    #[cfg(feature = "nl")] // The Kingdom of the Netherlands (Europe)
    ("netherlands", Alpha2::NL),
    #[cfg(feature = "no")] // The Kingdom of Norway (Europe)
    ("norway", Alpha2::NO),
    #[cfg(feature = "np")] // The Federal Democratic Republic of Nepal (Asia)
    ("nepal", Alpha2::NP),
    #[cfg(feature = "nr")] // The Republic of Nauru (Oceania)
    ("nauru", Alpha2::NR),
    #[cfg(feature = "nu")] // Niue (Oceania)
    ("niue", Alpha2::NU),
    #[cfg(feature = "nz")] // New Zealand (Oceania)
    ("new zealand", Alpha2::NZ),
    #[cfg(feature = "om")] // The Sultanate of Oman (Asia)
    ("oman", Alpha2::OM),
    #[cfg(feature = "pa")] // The Republic of Panamá (Americas)
    ("panama", Alpha2::PA),
    #[cfg(feature = "pe")] // The Republic of Perú (Americas)
    ("peru", Alpha2::PE),
    #[cfg(feature = "pf")] // French Polynesia (Oceania)
    ("french polynesia", Alpha2::PF),
    #[cfg(feature = "pg")] // The Independent State of Papua New Guinea (Oceania)
    ("papua new guinea", Alpha2::PG),
    #[cfg(feature = "ph")] // The Republic of the Philippines (Asia)
    ("philippines", Alpha2::PH),
    #[cfg(feature = "pk")] // The Islamic Republic of Pakistan (Asia)
    ("pakistan", Alpha2::PK),
    #[cfg(feature = "pl")] // The Republic of Poland (Europe)
    ("poland", Alpha2::PL),
    #[cfg(feature = "pm")] // The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    ("saint pierre and miquelon", Alpha2::PM),
    #[cfg(feature = "pn")] // The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    ("pitcairn", Alpha2::PN),
    #[cfg(feature = "pr")] // The Commonwealth of Puerto Rico (Americas)
    ("puerto rico", Alpha2::PR),
    #[cfg(feature = "ps")] // The State of Palestine (Asia)
    ("palestine, state of", Alpha2::PS),
    #[cfg(feature = "pt")] // The Portuguese Republic (Europe)
    ("portugal", Alpha2::PT),
    #[cfg(feature = "pw")] // The Republic of Palau (Oceania)
    ("palau", Alpha2::PW),
    #[cfg(feature = "py")] // The Republic of Paraguay (Americas)
    ("paraguay", Alpha2::PY),
    #[cfg(feature = "qa")] // The State of Qatar (Asia)
    ("qatar", Alpha2::QA),
    #[cfg(feature = "re")] // Réunion (Africa)
    ("réunion", Alpha2::RE),
    #[cfg(feature = "ro")] // Romania (Europe)
    ("romania", Alpha2::RO),
    #[cfg(feature = "rs")] // The Republic of Serbia (Europe)
    ("serbia", Alpha2::RS),
    #[cfg(feature = "ru")] // The Russian Federation (Europe)
    ("russian federation", Alpha2::RU),
    #[cfg(feature = "rw")] // The Republic of Rwanda (Africa)
    ("rwanda", Alpha2::RW),
    #[cfg(feature = "sa")] // The Kingdom of Saudi Arabia (Asia)
    ("saudi arabia", Alpha2::SA),
    #[cfg(feature = "sb")] // The Solomon Islands (Oceania)
    ("solomon islands", Alpha2::SB),
    #[cfg(feature = "sc")] // The Republic of Seychelles (Africa)
    ("seychelles", Alpha2::SC),
    #[cfg(feature = "sd")] // The Republic of the Sudan (Africa)
    ("sudan", Alpha2::SD),
    #[cfg(feature = "se")] // The Kingdom of Sweden (Europe)
    ("sweden", Alpha2::SE),
    #[cfg(feature = "sg")] // The Republic of Singapore (Asia)
    ("singapore", Alpha2::SG),
    #[cfg(feature = "sh")] // Saint Helena, Ascension and Tristan da Cunha (Africa)
    ("saint helena, ascension and tristan da cunha", Alpha2::SH),
    #[cfg(feature = "si")] // The Republic of Slovenia (Europe)
    ("slovenia", Alpha2::SI),
    #[cfg(feature = "sj")] // Svalbard and Jan Mayen (Europe)
    ("svalbard and jan mayen", Alpha2::SJ),
    #[cfg(feature = "sk")] // The Slovak Republic (Europe)
    ("slovakia", Alpha2::SK),
    #[cfg(feature = "sl")] // The Republic of Sierra Leone (Africa)
    ("sierra leone", Alpha2::SL),
    #[cfg(feature = "sm")] // The Republic of San Marino (Europe)
    ("san marino", Alpha2::SM),
    #[cfg(feature = "sn")] // The Republic of Senegal (Africa)
    ("senegal", Alpha2::SN),
    #[cfg(feature = "so")] // The Federal Republic of Somalia (Africa)
    ("somalia", Alpha2::SO),
    #[cfg(feature = "sr")] // The Republic of Suriname (Americas)
    ("suriname", Alpha2::SR),
    #[cfg(feature = "ss")] // The Republic of South Sudan (Africa)
    ("south sudan", Alpha2::SS),
    #[cfg(feature = "st")] // The Democratic Republic of São Tomé and Príncipe (Africa)
    ("sao tome and principe", Alpha2::ST),
    #[cfg(feature = "sv")] // The Republic of El Salvador (Americas)
    ("el salvador", Alpha2::SV),
    #[cfg(feature = "sx")] // Sint Maarten (Americas)
    ("sint maarten (dutch part)", Alpha2::SX),
    #[cfg(feature = "sy")] // The Syrian Arab Republic (Asia)
    ("syrian arab republic", Alpha2::SY),
    #[cfg(feature = "sz")] // The Kingdom of Eswatini (Africa)
    ("eswatini", Alpha2::SZ),
    #[cfg(feature = "tc")] // The Turks and Caicos Islands (Americas)
    ("turks and caicos islands", Alpha2::TC),
    #[cfg(feature = "td")] // The Republic of Chad (Africa)
    ("chad", Alpha2::TD),
    #[cfg(feature = "tf")] // The French Southern and Antarctic Lands (Africa)
    ("french southern territories", Alpha2::TF),
    #[cfg(feature = "tg")] // The Togolese Republic (Africa)
    ("togo", Alpha2::TG),
    #[cfg(feature = "th")] // The Kingdom of Thailand (Asia)
    ("thailand", Alpha2::TH),
    #[cfg(feature = "tj")] // The Republic of Tajikistan (Asia)
    ("tajikistan", Alpha2::TJ),
    #[cfg(feature = "tk")] // Tokelau (Oceania)
    ("tokelau", Alpha2::TK),
    #[cfg(feature = "tl")] // The Democratic Republic of Timor-Leste (Asia)
    ("timor-leste", Alpha2::TL),
    #[cfg(feature = "tm")] // Turkmenistan (Asia)
    ("turkmenistan", Alpha2::TM),
    #[cfg(feature = "tn")] // The Republic of Tunisia (Africa)
    ("tunisia", Alpha2::TN),
    #[cfg(feature = "to")] // The Kingdom of Tonga (Oceania)
    ("tonga", Alpha2::TO),
    #[cfg(feature = "tr")] // The Republic of Türkiye (Asia)
    ("türkiye", Alpha2::TR),
    #[cfg(feature = "tt")] // The Republic of Trinidad and Tobago (Americas)
    ("trinidad and tobago", Alpha2::TT),
    #[cfg(feature = "tv")] // Tuvalu (Oceania)
    ("tuvalu", Alpha2::TV),
    #[cfg(feature = "tw")] // Taiwan, Province of China (Asia)
    ("taiwan, province of china", Alpha2::TW),
    #[cfg(feature = "tz")] // The United Republic of Tanzania (Africa)
    ("tanzania, united republic of", Alpha2::TZ),
    #[cfg(feature = "ua")] // Ukraine (Europe)
    ("ukraine", Alpha2::UA),
    #[cfg(feature = "ug")] // The Republic of Uganda (Africa)
    ("uganda", Alpha2::UG),
    #[cfg(feature = "um")] // United States Minor Outlying Islands (Americas)
    ("united states minor outlying islands", Alpha2::UM),
    #[cfg(feature = "us")] // The United States of America (Americas)
    ("united states of america", Alpha2::US),
    #[cfg(feature = "uy")] // The Oriental Republic of Uruguay (Americas)
    ("uruguay", Alpha2::UY),
    #[cfg(feature = "uz")] // The Republic of Uzbekistan (Asia)
    ("uzbekistan", Alpha2::UZ),
    #[cfg(feature = "va")] // The Holy See (Europe)
    ("holy see", Alpha2::VA),
    #[cfg(feature = "vc")] // Saint Vincent and the Grenadines (Americas)
    ("saint vincent and the grenadines", Alpha2::VC),
    #[cfg(feature = "ve")] // The Bolivarian Republic of Venezuela (Americas)
    ("venezuela (bolivarian republic of)", Alpha2::VE),
    #[cfg(feature = "vg")] // The Virgin Islands (Americas)
    ("virgin islands (british)", Alpha2::VG),
    #[cfg(feature = "vi")] // The Virgin Islands of the United States (Americas)
    ("virgin islands (u.s.)", Alpha2::VI),
    #[cfg(feature = "vn")] // The Socialist Republic of Viet Nam (Asia)
    ("viet nam", Alpha2::VN),
    #[cfg(feature = "vu")] // The Republic of Vanuatu (Oceania)
    ("vanuatu", Alpha2::VU),
    #[cfg(feature = "wf")] // The Territory of the Wallis and Futuna Islands (Oceania)
    ("wallis and futuna", Alpha2::WF),
    #[cfg(feature = "ws")] // The Independent State of Samoa (Oceania)
    ("samoa", Alpha2::WS),
    #[cfg(feature = "ye")] // The Republic of Yemen (Asia)
    ("yemen", Alpha2::YE),
    #[cfg(feature = "yt")] // The Department of Mayotte (Africa)
    ("mayotte", Alpha2::YT),
    #[cfg(feature = "za")] // The Republic of South Africa (Africa)
    ("south africa", Alpha2::ZA),
    #[cfg(feature = "zm")] // The Republic of Zambia (Africa)
    ("zambia", Alpha2::ZM),
    #[cfg(feature = "zw")] // The Republic of Zimbabwe (Africa)
    ("zimbabwe", Alpha2::ZW),
]);}
