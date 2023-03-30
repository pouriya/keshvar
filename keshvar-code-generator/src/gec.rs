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
        if info.gec.is_some() {
            new_countries_info_list.push(info.clone());
        } else {
            log!("Skip {:?} - {:?}", info.iso_long_name, info.alpha2_upper);
        }
    }

    let mut gec_rs_file = utils::create_new_file(destination_file, "GEC")?;
    utils::write_first_comments(&mut gec_rs_file, file!())?;
    gec_rs_file.write_all(
        r#"
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing `Geopolitical Entities and Codes` (GEC).
///
/// # Example
/// ```
/// use keshvar::{GEC, Alpha2};
///
/// assert_eq!(Ok(GEC::FP), GEC::try_from("fp"));
/// assert_eq!(GEC::FP.to_alpha2(), Alpha2::PF); // French Polynesia (Oceania)
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum GEC {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        gec_rs_file
            .write_all(utils::country_cfg_feature_and_doc_commented_name(info, 1).as_bytes())?;
        gec_rs_file.write_all(
            format!(
                "    {},\n",
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase()
            )
            .as_bytes(),
        )?;
    }
    gec_rs_file.write_all(b"}\n")?;

    gec_rs_file.write_all(
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

    gec_rs_file.write_all(
        r#"mod impls {
    use super::GEC;
    use crate::{Alpha2, Country, SearchError, make_search_error};
"#
        .as_bytes(),
    )?;

    gec_rs_file.write_all(
        r#"
    impl GEC {
        pub fn to_alpha2(&self) -> Alpha2 {
            match self {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        gec_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        gec_rs_file.write_all(
            format!(
                "               GEC::{} => Alpha2::{},\n",
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase(),
                info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    gec_rs_file.write_all(
        r#"            }
        }

        pub fn to_country(&self) -> Country {
            self.to_alpha2().to_country()
        }
    }
"#
        .as_bytes(),
    )?;

    gec_rs_file.write_all(
        r#"
    impl ToString for GEC {
        fn to_string(&self) -> String {
            match self {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        gec_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        gec_rs_file.write_all(
            format!(
                "               GEC::{} => {:?},\n",
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase(),
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    gec_rs_file.write_all(
        r#"            }.to_string()
        }
    }
"#
        .as_bytes(),
    )?;

    gec_rs_file.write_all(
        r#"
    impl TryFrom<&str> for GEC {
        type Error = SearchError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.len() != 2 {
                return Err(SearchError::BadInput{expected: "a string with two characters"})
            }
            match value.to_uppercase().as_str() {
"#
        .as_bytes(),
    )?;
    for info in new_countries_info_list.iter() {
        gec_rs_file.write_all(utils::country_cfg_feature_and_commented_name(info, 4).as_bytes())?;
        gec_rs_file.write_all(
            format!(
                "               {:?} => Ok(GEC::{}),\n",
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase(),
                utils::capitalize(&info.gec.clone().unwrap()).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    gec_rs_file.write_all(b"                _ => Err(make_search_error()),")?;
    gec_rs_file.write_all(
        r#"            }
        }
    }
"#
        .as_bytes(),
    )?;
    gec_rs_file.write_all(b"}\n")?;

    gec_rs_file.write_all(
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
    gec_rs_file.write_all(
        r#"mod impls {
    use super::GEC;
    use crate::{Alpha2, Country, SearchError};

    impl GEC {
        pub fn to_alpha2(&self) -> Alpha2 {
            unimplemented!("No country feature with GEC code is used");
        }

        pub fn to_country(&self) -> Country {
            unimplemented!("No country feature with GEC code is used");
        }
    }

    impl ToString for GEC {
        fn to_string(&self) -> String {
            unimplemented!("No country feature with GEC code is used");
        }
    }

    impl TryFrom<&str> for GEC {
        type Error = SearchError;
        fn try_from(_value: &str) -> Result<Self, Self::Error> {
            unimplemented!("No country feature with GEC code is used");
        }
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}
