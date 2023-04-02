use crate::consts::{SUPPORTED_COUNTRIES_COUNT, SUPPORT_ALL_COUNTRIES};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
/// Error enum for different search errors.
pub enum SearchError {
    /// When no entry is found.
    #[error("Could not be found in {searched_items}")]
    NotFound { searched_items: SearchedItems },
    /// When there were nothing to search (e.g. no subdivision is loaded but you are trying to search in subdivisions!)
    #[error("There is no {0} to search")]
    NoItemToSearch(&'static str),
    /// When the input size, etc. is wrong.
    #[error("Expected {expected}")]
    BadInput { expected: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Error)]
/// An enum showing what items we did search before yielding error.
pub enum SearchedItems {
    /// We searched all continents.
    #[error("all continents")]
    AllContinents,
    /// We searched all regions.
    #[error("all regions")]
    AllRegions,
    /// We searched all sub-regions.
    #[error("all sub-regions")]
    AllSubRegions,
    #[error("all world regions")]
    /// We searched all world regions.
    AllWorldRegions,
    #[error("all countries")]
    /// We searched all countries.
    AllCountries,
    /// We searched supported countries (when you did not include all countries).
    ///
    /// It also contains number of countries that you support.
    #[error("{0} supported countries")]
    SupportedCountries(usize),
    #[cfg(feature = "subdivisions")]
    /// We searched all subdivision types.
    #[cfg_attr(feature = "subdivisions", error("all subdivision types"))]
    AllSubDivisionTypes,
    #[cfg(feature = "subdivisions")]
    /// We searched all subdivisions.
    ///
    /// It also contains the number of subdivisions.
    #[cfg_attr(feature = "subdivisions", error("all {0} subdivisions"))]
    AllSubdivisions(usize),
}

#[allow(dead_code)]
pub(crate) fn make_search_error() -> SearchError {
    if SUPPORT_ALL_COUNTRIES {
        SearchError::NotFound {
            searched_items: SearchedItems::AllCountries,
        }
    } else {
        SearchError::NotFound {
            searched_items: SearchedItems::SupportedCountries(*SUPPORTED_COUNTRIES_COUNT),
        }
    }
}
