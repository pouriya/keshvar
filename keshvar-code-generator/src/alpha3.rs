use crate::structs::CountryInfo;
use crate::utils;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &[(String, CountryInfo)],
) -> Result<()> {
    let mut alpha3_rs_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!("Could not open {:?}", destination_file))?;
    utils::write_first_comments(&mut alpha3_rs_file, file!())?;
    alpha3_rs_file.write_all(
        r#"
use crate::{Alpha2, Country};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

"#
        .as_bytes(),
    )?;
    alpha3_rs_file.write_all(
        r#"
/// An enum containing Alpha3 codes for all countries.
///
/// All countries features are enabled by default. You can disable default features and enabled features for all countries that you need.
///
/// # Examples
/// ```
/// use keshvar::{Alpha2, Alpha3, Country, SearchError, SearchedItems};
///
/// assert_eq!(Ok(Alpha3::USA), Alpha3::try_from("usa")); // not case-sensitive
/// assert_eq!("USA", Alpha3::USA.to_string().as_str());
///
/// // If enabled all countries features:
/// assert_eq!(
///     Err(SearchError::NotFound {
///         searched_items: SearchedItems::AllCountries
///     }),
///     Alpha3::try_from("xxx")
/// );
/// assert_eq!(
///     Err("Could not be found in all countries".to_string()),
///     Alpha3::try_from("xxx").map_err(|error| error.to_string())
/// );
///
/// // If enabled some countries features:
/// // For example we enabled supporting just 10 countries and the US
/// // is not one of them:
/// // assert_eq!(
/// //     Err(SearchError::NotFound {
/// //         searched_items: SearchedItems::SupportedCountries(10)
/// //     }),
/// //     Alpha3::try_from("usa")
/// // );
/// // assert_eq!(
/// //     Err("Could not be found in 10 supported countries".to_string()),
/// //     Alpha3::try_from("usa").map_err(|error| error.to_string())
/// // );
///
/// // Convert to `Alpha2` code:
/// assert_eq!(Alpha2::US, Alpha3::USA.to_alpha2());
/// // Convert to `Country`:
/// let country: Country = Alpha3::USA.to_country();
/// // Get subdivisions of country:
/// let subdivisions = Alpha3::USA.to_subdivisions();
/// ```
/// We usually need to convert it to [`Country`](crate::Country) and use that object instead.
///
/// # Panics
/// Most methods will panic if you do not enable any country feature!
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum Alpha3 {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha3_rs_file
            .write_all(utils::country_cfg_feature_and_doc_commented_name(info, 1).as_bytes())?;
        alpha3_rs_file.write_all(format!("    {},\n", info.alpha3_upper).as_bytes())?;
    }
    alpha3_rs_file.write_all(b"}\n")?;
    alpha3_rs_file.write_all(
        r#"
impl From<Alpha3> for Country {
    fn from(alpha3: Alpha3) -> Self {
        Self::from(alpha3.to_alpha2())
    }
}

impl Alpha3 {
    pub fn to_country(&self) -> Country {
        Country::from(*self)
    }
}

impl From<Alpha2> for Alpha3 {
    fn from(alpha2: Alpha2) -> Self {
        alpha2.to_alpha3()
    }
}

#[cfg(feature = "subdivisions")]
use crate::Subdivision;
#[cfg(feature = "subdivisions")]
use std::collections::HashMap;
#[cfg(feature = "subdivisions")]
impl Alpha3 {
    pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
        self.to_alpha2().to_subdivisions()
    }
}
"#
        .as_bytes(),
    )?;

    alpha3_rs_file.write_all(
        utils::country_cfg_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "any",
            0,
        )
        .as_bytes(),
    )?;
    alpha3_rs_file.write_all(
        r#"mod impls {
    use super::{Alpha3, Alpha2};
    use crate::{SearchError, make_search_error};

    impl TryFrom<&str> for Alpha3 {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 3 {
                return Err(SearchError::BadInput{expected: "a string with three characters"})
            }
            match value.to_uppercase().as_str() {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha3_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha3_rs_file.write_all(
            format!(
                "                {:?} => Ok(Self::{}),\n",
                info.alpha3_upper, info.alpha3_upper
            )
            .as_bytes(),
        )?;
    }
    alpha3_rs_file.write_all(b"                _ => Err(make_search_error()),\n")?;
    alpha3_rs_file.write_all("            }\n        }\n    }\n".as_bytes())?;

    alpha3_rs_file.write_all(
        r#"
    impl ToString for Alpha3 {
        fn to_string(&self) -> String {
            match self {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha3_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha3_rs_file.write_all(
            format!(
                "                Alpha3::{} => {:?},\n",
                info.alpha3.to_uppercase(),
                info.alpha3.to_uppercase()
            )
            .as_bytes(),
        )?;
    }
    alpha3_rs_file.write_all("            }.to_string()\n        }\n    }\n".as_bytes())?;

    alpha3_rs_file.write_all(
        r#"
    impl Alpha3 {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha3_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha3_rs_file.write_all(
            format!(
                "                Alpha3::{} => Alpha2::{},\n",
                info.alpha3_upper, info.alpha2_upper,
            )
            .as_bytes(),
        )?;
    }
    alpha3_rs_file.write_all("           }\n        }\n    }\n".as_bytes())?;

    alpha3_rs_file.write_all("}\n".as_bytes())?;

    alpha3_rs_file.write_all(
        utils::country_cfg_not_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "any",
            0,
        )
        .as_bytes(),
    )?;
    alpha3_rs_file.write_all(
        r#"mod impls {
    use super::{Alpha2, Alpha3};
    use crate::SearchError;

    impl TryFrom<&str> for Alpha3 {
        type Error = SearchError;

        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature is used");
        }
    }

    impl ToString for Alpha3 {
        fn to_string(&self) -> String {
            unimplemented!("No country feature is used");
        }
    }

    impl Alpha3 {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature is used");
        }
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}
