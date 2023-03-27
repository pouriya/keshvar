use crate::structs::CountryInfo;
use crate::{log, utils};
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &Vec<(String, CountryInfo)>,
) -> Result<()> {
    let mut new_countries_info_list = Vec::new();
    for (_, info) in countries_info_list {
        if info.ioc.is_some() {
            new_countries_info_list.push(info.clone());
        } else {
            log!("Skip {:?} - {:?}", info.iso_long_name, info.alpha2_upper);
        }
    }

    let mut ioc_rs_file = utils::create_new_file(destination_file, "IOC")?;
    utils::write_first_comments(&mut ioc_rs_file, file!())?;
    ioc_rs_file.write_all(
        r#"
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing `International Olympic Committee` (IOC) codes.
///
/// # Example
/// ```
/// use keshvar::{IOC, Alpha2};
///
/// assert_eq!(Ok(IOC::BER), IOC::try_from("ber"));
/// assert_eq!(IOC::BER.to_alpha2(), Alpha2::BM); // Bermuda (Americas)
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum IOC {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        ioc_rs_file
            .write_all(utils::country_cfg_feature_and_doc_commented_name(info, 1).as_bytes())?;
        ioc_rs_file.write_all(
            format!(
                "    {},\n",
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase()
            )
            .as_bytes(),
        )?;
    }
    ioc_rs_file.write_all(b"}\n")?;

    ioc_rs_file.write_all(
        utils::country_cfg_features(
            new_countries_info_list
                .iter()
                .map(|info| info.feature_name.clone())
                .collect(),
            "any",
            0,
        )
        .as_bytes(),
    )?;

    ioc_rs_file.write_all(
        r#"mod impls {
    use super::IOC;
    use crate::{Alpha2, Country, SearchError, SearchedItems};
"#
        .as_bytes(),
    )?;

    ioc_rs_file.write_all(
        r#"
    impl IOC {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        ioc_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        ioc_rs_file.write_all(
            format!(
                "               IOC::{} => Alpha2::{},\n",
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase(),
                info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    ioc_rs_file.write_all(
        r#"            }
        }

        pub fn to_country(&self) -> Country {
            self.to_alpha2().to_country()
        }
    }
"#
        .as_bytes(),
    )?;

    ioc_rs_file.write_all(
        r#"
    impl ToString for IOC {
        fn to_string(&self) -> String {
            match self {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        ioc_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        ioc_rs_file.write_all(
            format!(
                "               IOC::{} => {:?},\n",
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase(),
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    ioc_rs_file.write_all(
        r#"            }.to_string()
        }
    }
"#
        .as_bytes(),
    )?;

    ioc_rs_file.write_all(
        r#"
    impl TryFrom<&str> for IOC {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 3 {
                return Err(SearchError::BadInput{expected: "a string with three characters"})
            }
            match value.to_uppercase().as_str() {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        ioc_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        ioc_rs_file.write_all(
            format!(
                "               {:?} => Ok(IOC::{}),\n",
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase(),
                utils::capitalize(&info.ioc.clone().unwrap()).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    ioc_rs_file.write_all(
        utils::country_cfg_features(
            new_countries_info_list
                .iter()
                .map(|info| info.feature_name.clone())
                .collect(),
            "all",
            4,
        )
        .as_bytes(),
    )?;
    ioc_rs_file.write_all(
        r#"                _ => Err(
                    SearchError::NotFound{
                        searched_items: SearchedItems::AllCountries
                    }
                ),
"#
        .as_bytes(),
    )?;
    ioc_rs_file.write_all(
        r#"                #[allow(unreachable_patterns)]
                _ => Err(
                    SearchError::NotFound{
                        searched_items: SearchedItems::SupportedCountries(*crate::consts::SUPPORTED_COUNTRIES_COUNT)
                    }
                ),
"#.as_bytes()
    )?;
    ioc_rs_file.write_all(
        r#"            }
        }
    }
"#
        .as_bytes(),
    )?;
    ioc_rs_file.write_all(b"}\n")?;

    ioc_rs_file.write_all(
        utils::country_cfg_not_features(
            new_countries_info_list
                .iter()
                .map(|info| info.feature_name.clone())
                .collect(),
            "any",
            0,
        )
        .as_bytes(),
    )?;
    ioc_rs_file.write_all(
        r#"mod impls {
    use super::IOC;
    use crate::{Alpha2, Country, SearchError};

    impl IOC {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature with IOC code is used");
        }

        pub fn to_country(&self) -> Country {
            unimplemented!("No country feature with IOC code is used");
        }
    }

    impl ToString for IOC {
        fn to_string(&self) -> String {
            unimplemented!("No country feature with IOC code is used");
        }
    }

    impl TryFrom<&str> for IOC {
        type Error = SearchError;
        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature with IOC code is used");
        }
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}
