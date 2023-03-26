use crate::structs::CountryInfo;
use crate::utils;
use crate::utils::country_name;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &Vec<(String, CountryInfo)>,
) -> Result<()> {
    let mut currency_codes = HashMap::new();
    let mut sorted_currency_code_list = Vec::new();
    for (_, info) in countries_info_list {
        if !currency_codes.contains_key(&info.currency_code) {
            currency_codes.insert(&info.currency_code, Vec::new());
        }
        currency_codes
            .get_mut(&info.currency_code)
            .unwrap()
            .push(&info.alpha2_upper);
        if !sorted_currency_code_list.contains(&info.currency_code) {
            sorted_currency_code_list.push(info.currency_code.clone())
        }
    }
    sorted_currency_code_list.sort();
    for (_, alpha2_list) in currency_codes.iter_mut() {
        alpha2_list.sort()
    }

    let mut currency_code_rs_file = utils::create_new_file(destination_file, "Currency Code")?;
    utils::write_first_comments(&mut currency_code_rs_file, file!())?;
    currency_code_rs_file.write_all(b"use crate::SearchError;\n\n")?;
    currency_code_rs_file.write_all(
        r#"
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing currency codes.
///
/// # Example
/// ```
/// use keshvar::{CurrencyCode, Alpha2};
///
/// assert_eq!(Ok(CurrencyCode::QAR), CurrencyCode::try_from("qAr")); // not case-sensitive
/// let eur_alpha2_country_list: Vec<Alpha2> = CurrencyCode::EUR
///     .alpha2_list()
///     .iter()
///     .filter_map(|alpha2_str| Alpha2::try_from(*alpha2_str).ok())
///     .collect();
/// assert!(eur_alpha2_country_list.contains(&Alpha2::ES)); // Spain
/// assert!(eur_alpha2_country_list.contains(&Alpha2::NL)); // Netherlands
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum CurrencyCode {
"#
            .as_bytes(),
    )?;
    for currency_code in &sorted_currency_code_list {
        for alpha2 in currency_codes.get(currency_code).unwrap() {
            let info = countries_info_list
                .iter()
                .filter(|(_, info)| info.alpha2_upper == **alpha2)
                .collect::<Vec<_>>()[0]
                .clone()
                .1;
            currency_code_rs_file
                .write_all(format!("    /// * {}\n", country_name(&info)).as_bytes())?;
        }
        currency_code_rs_file.write_all(
            format!("    {},\n", utils::capitalize(currency_code).to_uppercase()).as_bytes(),
        )?;
    }
    currency_code_rs_file.write_all(b"}\n")?;

    currency_code_rs_file.write_all(
        r#"
impl CurrencyCode {
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
"#
        .as_bytes(),
    )?;
    for currency_code in &sorted_currency_code_list {
        currency_code_rs_file.write_all(
            format!(
                "           Self::{} => {{\n               &{:?}\n           }}\n",
                utils::capitalize(currency_code).to_uppercase(),
                currency_codes.get(currency_code).unwrap()
            )
            .as_bytes(),
        )?;
    }
    currency_code_rs_file.write_all(
        r#"        }
    }
}
"#
        .as_bytes(),
    )?;

    currency_code_rs_file.write_all(
        r#"
impl ToString for CurrencyCode {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for currency_code in &sorted_currency_code_list {
        currency_code_rs_file.write_all(
            format!(
                "           Self::{} => {:?},\n",
                utils::capitalize(currency_code).to_uppercase(),
                utils::capitalize(currency_code).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    currency_code_rs_file.write_all(
        r#"        }.to_string()
    }
}
"#
        .as_bytes(),
    )?;

    currency_code_rs_file.write_all(
        r#"
impl TryFrom<&str> for CurrencyCode {
    type Error = SearchError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(SearchError::BadInput{expected: "a string with three characters"})
        }
        match value.to_uppercase().as_str() {
"#
        .as_bytes(),
    )?;
    for currency_code in &sorted_currency_code_list {
        currency_code_rs_file.write_all(
            format!(
                "           {:?} => Ok(Self::{}),\n",
                utils::capitalize(currency_code).to_uppercase(),
                utils::capitalize(currency_code).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    currency_code_rs_file.write_all(
        r#"            _ => Err(SearchError::BadInput {
                expected: "valid currency code",
            }),
"#
        .as_bytes(),
    )?;
    currency_code_rs_file.write_all(
        r#"        }
    }
}
"#
        .as_bytes(),
    )?;

    currency_code_rs_file.write_all(
        r#"
#[cfg(feature = "iso-currency-integration")]
impl CurrencyCode {
    /// If `iso-currency-integration` feature is enabled, you can convert it to [`iso_currency::Currency`](iso_currency::Currency) enum.
    /// Note that [`CurrencyCode::STD`](crate::CurrencyCode::STD) is not supported by [iso_currency](iso_currency) library.
    pub fn to_iso_currency(&self) -> iso_currency::Currency {
        match self {
"#
        .as_bytes(),
    )?;
    for currency_code in &sorted_currency_code_list {
        if currency_code == "STD" {
            continue;
        }
        currency_code_rs_file.write_all(
            format!(
                "           Self::{} => iso_currency::Currency::{},\n",
                utils::capitalize(currency_code).to_uppercase(),
                utils::capitalize(currency_code).to_uppercase(),
            )
            .as_bytes(),
        )?;
    }
    currency_code_rs_file.write_all(
        r#"           _ => unimplemented!("Not implement by iso_currency library"),
            }
    }
}
"#
        .as_bytes(),
    )?;
    Ok(())
}
