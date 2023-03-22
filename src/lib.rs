#[doc = include_str!("../README.md")]
mod consts;
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
pub mod countries;
mod error;

pub use error::{SearchError, SearchedItems};

pub fn find_by_iso_short_name(iso_short_name: &str) -> Result<Country, SearchError> {
    for (country_iso_short_name, alpha2) in consts::SUPPORTED_ISO_SHORT_NAMES.iter() {
        if country_iso_short_name == &iso_short_name {
            return Ok((*alpha2).to_country());
        }
    }
    Err(make_search_error())
}

pub fn find_by_iso_long_name(iso_long_name: &str) -> Result<Country, SearchError> {
    for (country_iso_long_name, alpha2) in consts::SUPPORTED_ISO_LONG_NAMES.iter() {
        if country_iso_long_name == &iso_long_name {
            return Ok((*alpha2).to_country());
        }
    }
    Err(make_search_error())
}

pub fn find_by_code(code: usize) -> Result<Country, SearchError> {
    for (country_code, alpha2) in consts::SUPPORTED_COUNTRY_CODE.iter() {
        if country_code == &code {
            return Ok((*alpha2).to_country());
        }
    }
    Err(make_search_error())
}

pub fn find<F>(filter: F) -> Result<Country, SearchError>
where
    F: Fn(Country) -> bool,
{
    if let Some(country) = consts::SUPPORTED_ALPHA2_LIST
        .iter()
        .find(|alpha2| filter(alpha2.to_country()))
        .map(|alpha2| alpha2.to_country())
    {
        Ok(country)
    } else {
        Err(make_search_error())
    }
}

// #[inline]
// pub fn supported_countries_iterator() -> Iter<'static, Country> {
//     consts::SUPPORTED_ALPHA2_LIST.iter().map(|alpha2| alpha2.to_country()).collect::<Vec<_>>().iter().collect()
// }

#[inline]
fn make_search_error() -> SearchError {
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

#[cfg(test)]
mod unit_tests {
    use super::{Alpha2, Alpha3, Country, SearchError, SearchedItems, GEC};

    #[test]
    fn alpha2() {
        assert_eq!(Alpha2::UA, Alpha2::from(Alpha3::UKR));
        // assert_eq!("IS", Alpha2::IS.to_string().as_str());
        assert_eq!("IRN", Alpha2::IR.to_alpha3().to_string().as_str());
        assert_eq!(Ok(Alpha2::US), Alpha2::try_from("us"));
        assert_eq!(Ok(Alpha2::IQ), Alpha2::try_from(&*"iQ".to_string()));
        assert_eq!(
            Err("Could not be found in all countries".to_string()),
            Alpha2::try_from("xx").map_err(|error| error.to_string())
        );
        assert_eq!(
            Err("Expected a string with two characters".to_string()),
            Alpha2::try_from("123").map_err(|error| error.to_string())
        );
    }

    #[test]
    fn alpha3() {
        assert_eq!(Alpha3::UKR, Alpha3::from(Alpha2::UA));
        // assert_eq!("ISR", Alpha3::ISR.to_string().as_str());
        assert_eq!("IR", Alpha3::IRN.to_alpha2().to_string().as_str());
        assert_eq!(Ok(Alpha3::USA), Alpha3::try_from("uSa"));
        assert_eq!(Ok(Alpha3::IRQ), Alpha3::try_from(&*"iRq".to_string()));
        assert_eq!(
            Err(SearchError::NotFound {
                searched_items: SearchedItems::AllCountries
            }),
            Alpha3::try_from("xxx")
        );
        assert_eq!(
            Err(SearchError::BadInput {
                expected: "a string with three characters"
            }),
            Alpha3::try_from("1234")
        );
    }

    #[test]
    fn convert() {
        assert_eq!(Country::from(Alpha2::AF), Country::from(Alpha3::AFG));
    }

    #[test]
    fn gec() {
        assert_eq!(None, Country::from(Alpha3::ALA).gec); // Åland (Europe)
        assert_eq!(Some(GEC::FP), Country::from(Alpha2::PF).gec); // French Polynesia (Oceania)
        assert_eq!(GEC::PM.to_alpha2(), Alpha2::PA); // The Republic of Panamá (Americas)
        assert_eq!(Country::try_from("ai"), Ok(GEC::AV.to_country())); // Anguilla (Americas)
        assert_eq!(Country::try_from("av"), Ok(GEC::AV.to_country())); // Anguilla (Americas)
    }

    #[test]
    fn search() {
        assert_eq!(
            Ok(Alpha2::KE.to_country()),
            super::find_by_iso_short_name("kenya")
        );
        assert_eq!(
            Ok(Alpha2::KE.to_country()),
            super::find_by_iso_long_name("the republic of kenya")
        );
        assert_eq!(
            Ok(Alpha2::YE.to_country()),
            super::find_by_code(967) // The Republic of Yemen (Asia)
        );
    }
}
