use crate::{
    Alpha2, Alpha3, Continent, CurrencyCode, Region, SearchError, SubRegion, WorldRegion, GEC, IOC,
};

#[cfg(feature = "subdivisions")]
use crate::{SearchedItems, SubdivisionType};

#[cfg(any(feature = "subdivisions", feature = "translations"))]
use std::collections::HashMap;

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// A struct containing useful information for a country.
pub struct Country {
    pub(crate) alpha2: Alpha2,
    pub(crate) alpha3: Alpha3,
    pub(crate) country_code: usize,
    pub(crate) continent: Continent,
    pub(crate) currency_code: CurrencyCode,
    pub(crate) address_format: Option<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(rename = "gec"))]
    pub(crate) maybe_gec: Option<GEC>,
    #[cfg(feature = "geo")]
    pub(crate) geo: CountryGeo,
    pub(crate) international_prefix: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(rename = "ioc"))]
    pub(crate) maybe_ioc: Option<IOC>,
    pub(crate) iso_long_name: &'static str,
    pub(crate) iso_short_name: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) official_language_list: Vec<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) spoken_language_list: Vec<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) national_destination_code_length_list: Vec<usize>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) national_number_length_list: Vec<u8>,
    pub(crate) national_prefix: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(rename = "nationality"))]
    pub(crate) maybe_nationality: Option<&'static str>,
    pub(crate) number: &'static str,
    pub(crate) postal_code: bool,
    pub(crate) postal_code_format: Option<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(rename = "region"))]
    pub(crate) maybe_region: Option<Region>,
    pub(crate) start_of_week: WeekDay,
    #[cfg_attr(feature = "serde-derive", serde(rename = "subregion"))]
    pub(crate) maybe_subregion: Option<SubRegion>,
    pub(crate) un_locode: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) unofficial_name_list: Vec<&'static str>,
    pub(crate) world_region: WorldRegion,
    #[cfg(feature = "emojis")]
    pub(crate) emoji: &'static str,
    #[cfg(feature = "translations")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) translations: HashMap<&'static str, &'static str>,
    #[cfg(feature = "subdivisions")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) subdivisions: HashMap<&'static str, Subdivision>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) g7_member: bool,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) g20_member: bool,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) eu_member: bool,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) eea_member: bool,
    #[cfg_attr(feature = "serde-derive", serde(rename = "vat_rates"))]
    pub(crate) maybe_vat_rates: Option<VatRates>,
    pub(crate) distance_unit: DistanceUnit,
    #[cfg_attr(feature = "serde-derive", serde(rename = "population"))]
    pub(crate) maybe_population: Option<u64>,
}

impl Country {
    pub fn alpha2(&self) -> Alpha2 {
        self.alpha2
    }

    pub fn alpha3(&self) -> Alpha3 {
        self.alpha3
    }

    pub fn country_code(&self) -> usize {
        self.country_code
    }

    pub fn continent(&self) -> Continent {
        self.continent
    }

    pub fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }

    /// A regex that helps you validate address format of this country.
    pub fn address_format(&self) -> Option<&'static str> {
        self.address_format
    }

    /// GEC (Geopolitical Entities and Codes) for this country.
    pub fn maybe_gec(&self) -> Option<GEC> {
        self.maybe_gec
    }

    #[cfg(feature = "geo")]
    /// Enabled if `geo` feature is enabled.
    pub fn geo(&self) -> CountryGeo {
        self.geo
    }

    pub fn international_prefix(&self) -> &'static str {
        self.international_prefix
    }

    /// IOC (International Olympic Committee) for this country.
    pub fn maybe_ioc(&self) -> Option<IOC> {
        self.maybe_ioc
    }

    pub fn iso_long_name(&self) -> &'static str {
        self.iso_long_name
    }

    pub fn iso_short_name(&self) -> &'static str {
        self.iso_short_name
    }

    pub fn official_language_list(&self) -> &[&'static str] {
        &self.official_language_list
    }

    pub fn spoken_language_list(&self) -> &[&'static str] {
        &self.spoken_language_list
    }

    pub fn national_destination_code_length_list(&self) -> &[usize] {
        &self.national_destination_code_length_list
    }

    pub fn national_number_length_list(&self) -> &[u8] {
        &self.national_number_length_list
    }

    pub fn national_prefix(&self) -> &'static str {
        self.national_prefix
    }

    pub fn maybe_nationality(&self) -> Option<&'static str> {
        self.maybe_nationality
    }

    pub fn number(&self) -> &'static str {
        self.number
    }

    pub fn postal_code(&self) -> bool {
        self.postal_code
    }

    pub fn postal_code_format(&self) -> Option<&'static str> {
        self.postal_code_format
    }

    pub fn maybe_region(&self) -> Option<Region> {
        self.maybe_region
    }

    pub fn start_of_week(&self) -> WeekDay {
        self.start_of_week
    }

    pub fn maybe_subregion(&self) -> Option<SubRegion> {
        self.maybe_subregion
    }

    pub fn un_locode(&self) -> &'static str {
        self.un_locode
    }

    pub fn unofficial_name_list(&self) -> &[&'static str] {
        &self.unofficial_name_list
    }

    pub fn world_region(&self) -> WorldRegion {
        self.world_region
    }

    #[cfg(feature = "emojis")]
    pub fn emoji(&self) -> &'static str {
        self.emoji
    }

    #[cfg(feature = "translations")]
    /// A hashmap containing languages as keys and translations as values.
    /// Enabled if `translations` feature is enabled.
    pub fn translations(&self) -> &HashMap<&'static str, &'static str> {
        &self.translations
    }

    #[cfg(feature = "subdivisions")]
    /// Enabled if `subdivisions` feature is enabled.
    pub fn subdivisions(&self) -> &HashMap<&'static str, Subdivision> {
        &self.subdivisions
    }

    /// Is this country a member of [G7](https://en.wikipedia.org/wiki/G7)?.
    pub fn g7_member(&self) -> bool {
        self.g7_member
    }

    /// Is this country a member of [G20](https://en.wikipedia.org/wiki/G7)?
    pub fn g20_member(&self) -> bool {
        self.g20_member
    }

    /// Is this country a member of [EU](https://en.wikipedia.org/wiki/European_Union)?
    pub fn eu_member(&self) -> bool {
        self.eu_member
    }

    /// Is this country a member of [EEA](https://en.wikipedia.org/wiki/European_Economic_Area)?
    pub fn eea_member(&self) -> bool {
        self.eea_member
    }

    /// Is this country [GDPR](https://en.wikipedia.org/wiki/General_Data_Protection_Regulation) compliant?
    pub fn gdpr_compliant(&self) -> bool {
        self.eea_member || self.alpha2.to_string() == "GB"
    }

    /// [Value-added Tax](https://en.wikipedia.org/wiki/Value-added_tax) for this country.
    pub fn maybe_vat_rates(&self) -> Option<&VatRates> {
        self.maybe_vat_rates.as_ref()
    }

    /// `Km` or `Mi`.
    pub fn distance_unit(&self) -> DistanceUnit {
        self.distance_unit
    }

    /// Total population.
    ///
    /// From [data.worldbank.org](https://data.worldbank.org/indicator/SP.POP.TOTL).
    pub fn maybe_population(&self) -> Option<u64> {
        self.maybe_population
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.alpha2 == other.alpha2
    }
}

impl TryFrom<&str> for Country {
    type Error = SearchError;

    /// The `value` should be [`Alpha2`](crate::Alpha2), [`Alpha3`](crate::Alpha3), [`GEC`](crate::GEC), [`IOC`](crate::IOC) of a country in string format.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let length = value.len();
        if length == 2 {
            Alpha2::try_from(value)
                .or_else(|_| GEC::try_from(value).map(|gec| gec.to_alpha2()))
                .map(|alpha2| alpha2.to_country())
                .map_err(|_| SearchError::BadInput {
                    expected: "a string with two alpha2 or GEC characters",
                })
        } else if length == 3 {
            Alpha3::try_from(value)
                .or_else(|_| IOC::try_from(value).map(|ioc| ioc.to_alpha2().to_alpha3()))
                .map(|alpha3| alpha3.to_country())
                .map_err(|_| SearchError::BadInput {
                    expected: "a string with three alpha3 or IOC characters",
                })
        } else {
            Err(SearchError::BadInput {
                expected:
                    "a string with three alpha3 or IOC characters or two alpha2 or GEC characters",
            })
        }
    }
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// GEO information for a [`Country`](crate::Country).
pub struct CountryGeo {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) max_latitude: f64,
    pub(crate) max_longitude: f64,
    pub(crate) min_latitude: f64,
    pub(crate) min_longitude: f64,
    pub(crate) bounds: CountryGeoBounds,
}

#[cfg(feature = "geo")]
impl CountryGeo {
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn max_latitude(&self) -> f64 {
        self.max_latitude
    }

    pub fn max_longitude(&self) -> f64 {
        self.max_longitude
    }

    pub fn min_latitude(&self) -> f64 {
        self.min_latitude
    }

    pub fn min_longitude(&self) -> f64 {
        self.min_longitude
    }

    pub fn bounds(&self) -> CountryGeoBounds {
        self.bounds
    }
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Northeast and southwest GEO bounds for a [`Country`](crate::Country).
pub struct CountryGeoBounds {
    pub(crate) northeast: CountryGeoBound,
    pub(crate) southwest: CountryGeoBound,
}

#[cfg(feature = "geo")]
impl CountryGeoBounds {
    pub fn northeast(&self) -> CountryGeoBound {
        self.northeast
    }

    pub fn southwest(&self) -> CountryGeoBound {
        self.southwest
    }
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// GEO information for a northeast/southwest bound for a [`Country`](crate::Country)..
pub struct CountryGeoBound {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

#[cfg(feature = "geo")]
impl CountryGeoBound {
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}

#[cfg(feature = "subdivisions")]
/// A struct containing useful information about a subdivision.
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Subdivision {
    pub(crate) code: &'static str,
    pub(crate) country_alpha2: Alpha2,
    pub(crate) name: &'static str,
    #[cfg(feature = "geo")]
    pub(crate) geo: Option<SubdivisionGeo>,
    pub(crate) comments: Option<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(alias = "type"))]
    pub(crate) subdivision_type: SubdivisionType,
    #[cfg(feature = "translations")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) translations: HashMap<&'static str, &'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) unofficial_name_list: Vec<&'static str>,
}

#[cfg(feature = "subdivisions")]
impl Subdivision {
    /// A unique code for this subdivision in its country.
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn country_alpha2(&self) -> Alpha2 {
        self.country_alpha2
    }

    /// Name of this subdivision.
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[cfg(feature = "geo")]
    /// Enabled if `geo` feature is enabled.
    pub fn geo(&self) -> Option<SubdivisionGeo> {
        self.geo
    }

    pub fn comments(&self) -> Option<&'static str> {
        self.comments
    }

    pub fn subdivision_type(&self) -> SubdivisionType {
        self.subdivision_type
    }

    #[cfg(feature = "translations")]
    /// Enabled if `translations` feature is enabled.
    pub fn translations(&self) -> &HashMap<&'static str, &'static str> {
        &self.translations
    }

    pub fn unofficial_name_list(&self) -> &[&'static str] {
        &self.unofficial_name_list
    }
}

#[cfg(all(feature = "geo", feature = "subdivisions"))]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// GEO information for a [`Subdivision`](crate::Subdivision).
pub struct SubdivisionGeo {
    pub(crate) latitude: Option<f64>,
    pub(crate) longitude: Option<f64>,
    pub(crate) max_latitude: Option<f64>,
    pub(crate) max_longitude: Option<f64>,
    pub(crate) min_latitude: Option<f64>,
    pub(crate) min_longitude: Option<f64>,
}

#[cfg(all(feature = "geo", feature = "subdivisions"))]
impl SubdivisionGeo {
    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    pub fn longitude(&self) -> Option<f64> {
        self.longitude
    }

    pub fn max_latitude(&self) -> Option<f64> {
        self.max_latitude
    }

    pub fn max_longitude(&self) -> Option<f64> {
        self.max_longitude
    }

    pub fn min_latitude(&self) -> Option<f64> {
        self.min_latitude
    }

    pub fn min_longitude(&self) -> Option<f64> {
        self.min_longitude
    }
}

#[cfg(feature = "subdivisions")]
impl PartialEq for Subdivision {
    fn eq(&self, other: &Self) -> bool {
        self.country_alpha2 == other.country_alpha2 && self.code == other.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// Name of the weekday. Used to show start day of the week for each country.
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl TryFrom<&str> for WeekDay {
    type Error = SearchError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "monday" | "mon" => Ok(Self::Monday),
            "tuesday" | "tue" => Ok(Self::Tuesday),
            "wednesday" | "wed" => Ok(Self::Wednesday),
            "thursday" | "thu" => Ok(Self::Thursday),
            "friday" | "fri" => Ok(Self::Friday),
            "saturday" | "sat" => Ok(Self::Saturday),
            "sunday" | "sun" => Ok(Self::Sunday),
            _ => Err(SearchError::BadInput {
                expected: "one of weekday names",
            }),
        }
    }
}

impl ToString for WeekDay {
    fn to_string(&self) -> String {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
        .to_string()
    }
}

#[cfg(feature = "chrono-integration")]
impl WeekDay {
    /// If the feaure `chrono-integration` is specified, You can convert it to [`chrono::Weekday`](chrono::Weekday).
    pub fn to_chrono_weekday(&self) -> chrono::Weekday {
        match self {
            Self::Monday => chrono::Weekday::Mon,
            Self::Tuesday => chrono::Weekday::Tue,
            Self::Wednesday => chrono::Weekday::Wed,
            Self::Thursday => chrono::Weekday::Thu,
            Self::Friday => chrono::Weekday::Fri,
            Self::Saturday => chrono::Weekday::Sat,
            Self::Sunday => chrono::Weekday::Sun,
        }
    }
}

#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// [Value-added Tax](https://en.wikipedia.org/wiki/Value-added_tax)
pub struct VatRates {
    pub(crate) standard: f64,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub(crate) reduced: Vec<f64>,
    pub(crate) super_reduced: Option<f64>,
    pub(crate) parking: Option<f64>,
}

impl VatRates {
    pub fn standard(&self) -> f64 {
        self.standard
    }

    pub fn reduced(&self) -> Vec<f64> {
        self.reduced.clone()
    }

    pub fn super_reduced(&self) -> Option<f64> {
        self.super_reduced
    }

    pub fn parking(&self) -> Option<f64> {
        self.parking
    }
}

#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-derive", serde(rename_all = "UPPERCASE"))]
#[derive(Clone, Debug, Copy)]
pub enum DistanceUnit {
    Km,
    Mi,
}

impl DistanceUnit {
    pub fn is_km(&self) -> bool {
        matches!(self, Self::Km)
    }

    pub fn is_mi(&self) -> bool {
        matches!(self, Self::Mi)
    }
}

#[cfg(feature = "subdivisions")]
#[inline]
pub fn find_in_subdivisions<F>(
    subdivisions: &HashMap<&str, Subdivision>,
    search: F,
) -> Result<Subdivision, SearchError>
where
    F: Fn((&str, &Subdivision)) -> bool,
{
    if subdivisions.is_empty() {
        return Err(SearchError::NoItemToSearch("subdivision"));
    }
    if let Some((_, subdivision)) = subdivisions
        .iter()
        .find(|(code, subdivision)| search((code, subdivision)))
    {
        return Ok(subdivision.clone());
    }
    Err(SearchError::NotFound {
        searched_items: SearchedItems::AllSubdivisions(subdivisions.len()),
    })
}

#[cfg(feature = "subdivisions")]
impl Country {
    #[inline]
    pub fn find_in_subdivisions<F>(&self, search: F) -> Result<Subdivision, SearchError>
    where
        F: Fn((&str, &Subdivision)) -> bool,
    {
        find_in_subdivisions(&self.subdivisions, search)
    }

    pub fn find_in_subdivisions_by_code(&self, code: &str) -> Result<Subdivision, SearchError> {
        self.find_in_subdivisions(|(subdivision_code, _)| subdivision_code == code)
    }

    pub fn find_in_subdivisions_by_name(&self, name: &str) -> Result<Subdivision, SearchError> {
        self.find_in_subdivisions(|(_, subdivision)| subdivision.name == name)
    }

    pub fn find_in_subdivisions_by_unofficial_name(
        &self,
        unofficial_name: &str,
    ) -> Result<Subdivision, SearchError> {
        self.find_in_subdivisions(|(_, subdivision)| {
            subdivision.unofficial_name_list.contains(&unofficial_name)
        })
    }
}
