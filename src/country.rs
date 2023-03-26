use crate::{
    Alpha2, Alpha3, Continent, CurrencyCode, Region, SearchError, SubRegion, WorldRegion, GEC, IOC,
};

#[cfg(feature = "subdivisions")]
use crate::{SearchedItems, SubdivisionType};

#[cfg(feature = "subdivisions")]
use std::collections::HashMap;

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Country {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub country_code: usize,
    pub continent: Continent,
    pub currency_code: CurrencyCode,
    pub address_format: Option<&'static str>,
    pub gec: Option<GEC>,
    #[cfg(feature = "geo")]
    pub geo: CountryGeo,
    pub international_prefix: &'static str,
    pub ioc: Option<IOC>,
    pub iso_long_name: &'static str,
    pub iso_short_name: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub official_language_list: Vec<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub spoken_language_list: Vec<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub national_destination_code_length_list: Vec<usize>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub national_number_length_list: Vec<u8>,
    pub national_prefix: &'static str,
    pub nationality: Option<&'static str>,
    pub number: &'static str,
    pub postal_code: bool,
    pub postal_code_format: Option<&'static str>,
    pub region: Option<Region>,
    pub start_of_week: WeekDay,
    pub subregion: Option<SubRegion>,
    pub un_locode: &'static str,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub unofficial_name_list: Vec<&'static str>,
    pub world_region: WorldRegion,
    #[cfg(feature = "translations")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub translations: HashMap<&'static str, &'static str>,
    #[cfg(feature = "subdivisions")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub subdivisions: HashMap<&'static str, Subdivision>,
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.alpha2 == other.alpha2
    }
}

impl TryFrom<&str> for Country {
    type Error = SearchError;

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
            Alpha3::try_from(value).map(|alpha3| alpha3.to_country())
        } else {
            Err(SearchError::BadInput {
                expected: "a string with three alpha3 characters or two alpha2 or GEC characters",
            })
        }
    }
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CountryGeo {
    pub latitude: f64,
    pub longitude: f64,
    pub max_latitude: f64,
    pub max_longitude: f64,
    pub min_latitude: f64,
    pub min_longitude: f64,
    pub bounds: CountryGeoBounds,
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CountryGeoBounds {
    pub northeast: CountryGeoBound,
    pub southwest: CountryGeoBound,
}

#[cfg(feature = "geo")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CountryGeoBound {
    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(feature = "subdivisions")]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Subdivision {
    pub code: &'static str,
    pub country_alpha2: Alpha2,
    pub name: &'static str,
    #[cfg(feature = "geo")]
    pub geo: Option<SubdivisionGeo>,
    pub comments: Option<&'static str>,
    #[cfg_attr(feature = "serde-derive", serde(alias = "type"))]
    pub subdivision_type: SubdivisionType,
    #[cfg(feature = "translations")]
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub translations: HashMap<&'static str, &'static str>,
    #[cfg_attr(feature = "serde-derive", serde(default))]
    pub unofficial_name_list: Vec<&'static str>,
}

#[cfg(all(feature = "geo", feature = "subdivisions"))]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SubdivisionGeo {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub max_latitude: Option<f64>,
    pub max_longitude: Option<f64>,
    pub min_latitude: Option<f64>,
    pub min_longitude: Option<f64>,
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
