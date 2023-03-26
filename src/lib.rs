#![doc = include_str!("../README.md")]

mod country;
pub use country::*;
mod alpha2;
pub use alpha2::*;
mod alpha3;
pub use alpha3::*;
mod region;
pub use region::*;
mod gec;
pub use gec::*;
mod ioc;
pub use ioc::*;
mod currency_code;
pub use currency_code::*;
mod error;
pub use error::*;
mod consts;
pub mod countries;
pub use consts::{
    SUPPORTED_ALPHA2_LIST, SUPPORTED_CONTINENT_LIST, SUPPORTED_REGION_LIST,
    SUPPORTED_SUBREGION_LIST, SUPPORTED_WORLD_REGION_LIST,
};

// re-export integrations:
#[cfg(feature = "chrono-integration")]
#[doc(hidden)]
pub use chrono;
#[doc(hidden)]
pub use hashbrown;
#[cfg(feature = "iso-currency-integration")]
#[doc(hidden)]
pub use iso_currency;

/// Find first match in all included country iso short names.
///
/// Note that provided argument SHOULD be lowercase.
///
/// # Example
/// ```
/// use keshvar::{Alpha2, Country, find_by_iso_short_name};
///
/// let short_name = "united arab emirates"; // I'm sure it's lowercase
/// assert_eq!(Ok(Country::from(Alpha2::AE)), find_by_iso_short_name(short_name));
///
/// let other_short_name = "iTaLy"; // I'm NOT sure if it's lowercase
/// assert_eq!(
///     Ok(Country::from(Alpha2::IT)),
///     find_by_iso_short_name(&other_short_name.to_lowercase())
/// );
/// ```
pub fn find_by_iso_short_name(iso_short_name: &str) -> Result<Country, SearchError> {
    if let Some(alpha2) =
        consts::SUPPORTED_ISO_SHORT_NAMES.get(iso_short_name.to_lowercase().as_str())
    {
        Ok((*alpha2).to_country())
    } else {
        Err(make_search_error())
    }
}

/// Search for all matches in all included country iso short names.
///
/// Note that provided argument SHOULD be lowercase.
///
/// # Example
/// ```
/// use keshvar::search_in_iso_short_names;
///
/// let search_text = "republic";
/// let republic_country_list: Vec<&str> = search_in_iso_short_names(search_text)
///     .into_iter()
///     .map(|country| country.iso_short_name) // For example we just need their iso short names
///     .collect();
/// // Found 11 countries that have `republic` in their iso short names
/// assert_eq!(11, republic_country_list.len());
/// assert!(republic_country_list.contains(&"Iran (Islamic Republic of)"));
/// ```
pub fn search_in_iso_short_names(name: &str) -> Vec<Country> {
    let name = name.to_lowercase();
    consts::SUPPORTED_ISO_SHORT_NAMES
        .iter()
        .filter(|(iso_short_name, _)| iso_short_name.contains(&name))
        .map(|(_, alpha2)| alpha2.to_country())
        .collect()
}

/// Find first match in all included country iso long names.
///
/// Note that provided argument SHOULD be lowercase.
///
/// # Example
/// ```
/// use keshvar::{Alpha2, Country, find_by_iso_long_name};
///
/// let long_name = "the united mexican states"; // I'm sure it's lowercase
/// assert_eq!(Ok(Country::from(Alpha2::MX)), find_by_iso_long_name(long_name));
///
/// let other_long_name = "tHe repUblic Of UgAndA"; // I'm NOT sure if it's lowercase
/// assert_eq!(
///     Ok(Country::from(Alpha2::UG)),
///     find_by_iso_long_name(&other_long_name.to_lowercase())
/// );
/// ```
pub fn find_by_iso_long_name(iso_long_name: &str) -> Result<Country, SearchError> {
    if let Some(alpha2) =
        consts::SUPPORTED_ISO_LONG_NAMES.get(iso_long_name.to_lowercase().as_str())
    {
        Ok((*alpha2).to_country())
    } else {
        Err(make_search_error())
    }
}

/// Search for all matches in all included country iso long names.
///
/// Note that provided argument SHOULD be lowercase.
///
/// # Example
/// ```
/// use keshvar::search_in_iso_long_names;
///
/// let search_text = "nia";
/// let republic_country_list: Vec<&str> = search_in_iso_long_names(search_text)
///     .into_iter()
///     .map(|country| country.iso_long_name) // For example we just need their iso long names
///     .collect();
/// // Found 11 countries that have `nia` in their iso long names
/// assert_eq!(11, republic_country_list.len());
/// assert!(republic_country_list.contains(&"The Republic of Estonia"));
/// assert!(republic_country_list.contains(&"Bosnia and Herzegovina"));
/// ```
pub fn search_in_iso_long_names(iso_long_name: &str) -> Vec<Country> {
    let iso_long_name = iso_long_name.to_lowercase();
    consts::SUPPORTED_ISO_LONG_NAMES
        .iter()
        .filter(|(name, _)| name.contains(&iso_long_name))
        .map(|(_, alpha2)| alpha2.to_country())
        .collect()
}

/// Find by country code.
///
/// # Example
/// ```
/// use keshvar::{Alpha2, Country, find_by_code};
///
/// let country_code = 880; // The People's Republic of Bangladesh (Asia)
/// assert_eq!(Ok(Country::from(Alpha2::BD)), find_by_code(country_code));
/// ```
pub fn find_by_code(code: usize) -> Result<Country, SearchError> {
    if let Some(alpha2) = consts::SUPPORTED_COUNTRY_CODE.get(&code) {
        Ok((*alpha2).to_country())
    } else {
        Err(make_search_error())
    }
}

/// Find first match in all included countries by your own provided function.
///
/// # Example
/// ```
/// use keshvar::{Country, Continent, Alpha3, CurrencyCode, find};
///
/// // Find first country in Africa with currency code `USD`:
/// let result = find(
///     |country: &Country| {
///         country.continent == Continent::Africa && country.currency_code == CurrencyCode::USD
///     }
/// );
/// assert_eq!(Ok(Country::from(Alpha3::ZWE)), result); // Zimbabwe
/// ```
pub fn find<F>(mut filter: F) -> Result<Country, SearchError>
where
    F: FnMut(&Country) -> bool,
{
    if let Some(country) = SUPPORTED_ALPHA2_LIST
        .iter()
        .map(|alpha2| alpha2.to_country())
        .find(|country| filter(country))
    {
        Ok(country)
    } else {
        Err(make_search_error())
    }
}

/// Search for all matches in all included countries by your own provided function.
///
/// # Example
/// ```
/// use keshvar::{Country, WeekDay, Alpha3, SubRegion, search};
///
/// // Search for all countries in Southern-Asia that their start of the week day is sunday:
/// let result = search(
///     |country: &Country| {
///         if let Some(SubRegion::SouthernAsia) = country.subregion {
///             country.start_of_week == WeekDay::Sunday
///         } else {
///                 false
///         }
///     }
/// );
/// // Found 3 countries:
/// assert_eq!(3, result.len());
/// assert!(result.contains(&Country::from(Alpha3::NPL))); // Nepal
/// ```
pub fn search<F>(mut search: F) -> Vec<Country>
where
    F: FnMut(&Country) -> bool,
{
    SUPPORTED_ALPHA2_LIST
        .iter()
        .map(|alpha2| alpha2.to_country())
        .filter(|country| search(country))
        .collect()
}

/// Run a task function for all included countries.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use keshvar::{Country, Continent, for_each};
///
/// // Create a map for spoken languages of all included countries in South-America:
/// let mut spoken_languages = HashMap::new();
/// let result = for_each(
///     |country: &Country| {
///         if country.continent != Continent::SouthAmerica {
///             return
///         }
///         for lang in &country.spoken_language_list {
///             // Check if we already added this lang or not:
///             if !spoken_languages.contains_key(lang) {
///                 spoken_languages.insert(*lang, Vec::new());
///             }
///             // Add its iso short name to our map:
///             spoken_languages.get_mut(lang).unwrap().push(country.iso_short_name)
///         };
///     }
/// );
/// // println!("{spoken_languages:#?}");
/// // {
/// //     "nl": [
/// //         "Suriname",
/// //     ],
/// //     "gn": [
/// //         "Argentina",
/// //         "Paraguay",
/// //     ],
/// //     "es": [
/// //         "Argentina",
/// //         "Bolivia (Plurinational State of)",
/// //         "Chile",
/// //         "Colombia",
/// //         "Ecuador",
/// //         "Peru",
/// //         "Paraguay",
/// //         "Uruguay",
/// //         "Venezuela (Bolivarian Republic of)",
/// //     ],
/// //     "pt": [
/// //         "Brazil",
/// //     ],
/// //     "qu": [
/// //         "Bolivia (Plurinational State of)",
/// //     ],
/// //     "en": [
/// //         "Falkland Islands (Malvinas)",
/// //         "Guyana",
/// //     ],
/// //     "fr": [
/// //         "French Guiana",
/// //     ],
/// //     "ay": [
/// //         "Bolivia (Plurinational State of)",
/// //     ],
/// // }
/// assert!(spoken_languages.get("es").unwrap().contains(&"Colombia"));
/// assert!(spoken_languages.get("gn").unwrap().contains(&"Argentina"));
/// assert!(spoken_languages.get("en").unwrap().contains(&"Guyana"));
/// ```
pub fn for_each<F>(mut task: F)
where
    F: FnMut(&Country),
{
    SUPPORTED_ALPHA2_LIST
        .iter()
        .map(|alpha2| alpha2.to_country())
        .for_each(|country| task(&country));
}

/// Run a task function for all included countries.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use keshvar::{Country, WeekDay, CurrencyCode, filter_map};
///
/// let country_list = filter_map(
///     |country: &Country| {
///         if country.start_of_week == WeekDay::Monday && country.currency_code == CurrencyCode::USD {
///             Some(country.iso_short_name)
///         } else {
///             None
///         }
///     }
/// );
/// // Found 17 countries:
/// assert_eq!(17, country_list.len());
/// assert!(country_list.contains(&"Ecuador"));
/// ```
pub fn filter_map<F, B>(mut filter_map: F) -> Vec<B>
where
    F: FnMut(&Country) -> Option<B>,
{
    SUPPORTED_ALPHA2_LIST
        .iter()
        .map(|alpha2| alpha2.to_country())
        .filter_map(|country| filter_map(&country))
        .collect()
}

// TODO: use this function in other modules:
#[inline]
pub(crate) fn make_search_error() -> SearchError {
    #[allow(dead_code)]
    if consts::SUPPORT_ALL_COUNTRIES {
        SearchError::NotFound {
            searched_items: SearchedItems::AllCountries,
        }
    } else {
        SearchError::NotFound {
            searched_items: SearchedItems::SupportedCountries(*consts::SUPPORTED_COUNTRIES_COUNT),
        }
    }
}
