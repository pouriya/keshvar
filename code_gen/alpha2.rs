use crate::structs::CountryInfo;
use crate::utils;
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &[(String, CountryInfo)],
) -> Result<()> {
    let mut alpha2_rs_file = utils::create_new_file(destination_file, "Alpha2")?;
    utils::write_first_comments(&mut alpha2_rs_file, file!())?;
    alpha2_rs_file.write_all(
        r#"
use crate::{Country, Alpha3};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

"#
        .as_bytes(),
    )?;
    alpha2_rs_file.write_all(
        r#"
/// An enum containing Alpha2 codes for all countries.
///
/// All countries features are enabled by default. You can disable default features and enabled features for all countries that you need.
///
/// # Examples
/// ```
/// use keshvar::{Alpha2, Alpha3, Country, SearchError, SearchedItems};
///
/// assert_eq!(Ok(Alpha2::US), Alpha2::try_from("us")); // not case-sensitive
/// assert_eq!("US", Alpha2::US.to_string().as_str());
///
/// // If enabled all countries features:
/// assert_eq!(
///     Err(SearchError::NotFound {
///         searched_items: SearchedItems::AllCountries
///     }),
///     Alpha2::try_from("xx")
/// );
/// assert_eq!(
///     Err("Could not be found in all countries".to_string()),
///     Alpha2::try_from("xx").map_err(|error| error.to_string())
/// );
///
/// // If enabled some countries features:
/// // For example we enabled supporting just 10 countries and the US
/// // is not one of them:
/// // assert_eq!(
/// //     Err(SearchError::NotFound {
/// //         searched_items: SearchedItems::SupportedCountries(10)
/// //     }),
/// //     Alpha2::try_from("us")
/// // );
/// // assert_eq!(
/// //     Err("Could not be found in 10 supported countries".to_string()),
/// //     Alpha2::try_from("us").map_err(|error| error.to_string())
/// // );
///
/// // Convert to `Alpha3` code:
/// assert_eq!(Alpha3::USA, Alpha2::US.to_alpha3());
/// // Convert to `Country`:
/// let country: Country = Alpha2::US.to_country();
/// // Get subdivisions of country:
/// let subdivisions = Alpha2::US.to_subdivisions();
/// ```
/// We usually need to convert it to [`Country`](crate::Country) and use that object instead.
///
/// # Panics
/// Most methods will panic if you do not enable any country feature!
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum Alpha2 {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file
            .write_all(utils::country_cfg_feature_and_doc_commented_name(info, 1).as_bytes())?;
        alpha2_rs_file.write_all(format!("    {},\n", info.alpha2_upper).as_bytes())?;
    }
    alpha2_rs_file.write_all(b"}\n\n")?;

    alpha2_rs_file.write_all(
        r#"
impl Alpha2 {
    pub fn to_country(&self) -> Country {
        Country::from(*self)
    }
}

impl From<Alpha3> for Alpha2 {
    fn from(alpha3: Alpha3) -> Self {
        alpha3.to_alpha2()
    }
}
"#
        .as_bytes(),
    )?;

    alpha2_rs_file.write_all(
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
    alpha2_rs_file.write_all(
        r#"mod impls {
    use super::{Country, Alpha3, Alpha2};
    use crate::{SearchError, SearchedItems};

    impl TryFrom<&str> for Alpha2 {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 2 {
                return Err(SearchError::BadInput{expected: "a string with two characters"})
            }
            match value.to_uppercase().as_str() {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha2_rs_file.write_all(
            format!(
                "                {:?} => Ok(Self::{}),\n",
                info.alpha2_upper, info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    alpha2_rs_file.write_all(
        utils::country_cfg_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "all",
            4,
        )
        .as_bytes(),
    )?;
    alpha2_rs_file.write_all(
        r#"                _ => Err(
                    SearchError::NotFound{
                        searched_items: SearchedItems::AllCountries
                    }
                ),
"#
        .as_bytes(),
    )?;
    alpha2_rs_file.write_all(
        r#"                #[allow(unreachable_patterns)]
                _ => Err(
                    SearchError::NotFound{
                        searched_items: SearchedItems::SupportedCountries(*crate::consts::SUPPORTED_COUNTRIES_COUNT)
                    }
                ),
"#.as_bytes()
    )?;
    alpha2_rs_file.write_all("            }\n        }\n    }\n".as_bytes())?;

    alpha2_rs_file.write_all(
        r#"
    impl ToString for Alpha2 {
        fn to_string(&self) -> String {
            match self {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file.write_all(
            format!("                {}", utils::country_cfg_feature(info)).as_bytes(),
        )?;
        alpha2_rs_file.write_all(
            format!(
                "                Alpha2::{} => {:?},\n",
                info.alpha2_upper, info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    alpha2_rs_file.write_all("            }.to_string()\n        }\n    }\n".as_bytes())?;

    alpha2_rs_file.write_all(
        r#"
    #[cfg(feature = "subdivisions")]
    use crate::Subdivision;
    #[cfg(feature = "subdivisions")]
    use std::collections::HashMap;
    #[cfg(feature = "subdivisions")]
    impl Alpha2 {
        pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
            match self {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file.write_all(
            format!("                {}", utils::country_cfg_feature(info)).as_bytes(),
        )?;
        alpha2_rs_file.write_all(
            format!(
                "                Alpha2::{} => crate::countries::{}::subdivisions::new(),\n",
                info.alpha2_upper, info.module_name
            )
            .as_bytes(),
        )?;
    }
    alpha2_rs_file.write_all("            }\n        }\n    }\n".as_bytes())?;

    alpha2_rs_file.write_all(
        r#"
    impl From<Alpha2> for Country {
        fn from(alpha2: Alpha2) -> Self {
            match alpha2 {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha2_rs_file.write_all(
            format!(
                "                Alpha2::{} => crate::countries::{}::new(),\n",
                info.alpha2_upper, info.module_name,
            )
            .as_bytes(),
        )?;
    }
    alpha2_rs_file.write_all("           }\n        }\n    }\n".as_bytes())?;

    alpha2_rs_file.write_all(
        r#"
    impl Alpha2 {
        pub fn to_alpha3(&self) -> Alpha3 {
            match self {
"#
        .as_bytes(),
    )?;
    for (_, info) in countries_info_list.iter() {
        alpha2_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        alpha2_rs_file.write_all(
            format!(
                "                Alpha2::{} => Alpha3::{},\n",
                info.alpha2_upper, info.alpha3_upper,
            )
            .as_bytes(),
        )?;
    }
    alpha2_rs_file.write_all("           }\n        }\n    }\n".as_bytes())?;

    alpha2_rs_file.write_all("}\n".as_bytes())?;

    alpha2_rs_file.write_all(
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
    alpha2_rs_file.write_all(
        r#"mod impls {
    use super::{Country, Alpha3, Alpha2};
    use crate::SearchError;

    impl TryFrom<&str> for Alpha2 {
        type Error = SearchError;

        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature is used");
        }
    }

    impl ToString for Alpha2 {
        fn to_string(&self) -> String {
            unimplemented!("No country feature is used");
        }
    }

    #[cfg(feature = "subdivisions")]
    use crate::Subdivision;
    #[cfg(feature = "subdivisions")]
    use std::collections::HashMap;
    #[cfg(feature = "subdivisions")]
    impl Alpha2 {
        pub fn to_subdivisions(&self) -> HashMap<&'static str, Subdivision> {
            unimplemented!("No country feature is used");
        }
    }

    impl Alpha2 {
        pub fn to_alpha3(&self) -> Alpha3 {
            unimplemented!("No country feature is used");
        }
    }

    impl From<Alpha2> for Country {
        fn from(_alpha2: Alpha2) -> Self {
            unimplemented!("No country feature is used");
        }
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}
