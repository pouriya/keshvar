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
pub mod countries;
mod iterator;
pub use iterator::*;
mod consts;
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
