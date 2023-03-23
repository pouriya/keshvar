use crate::structs::CountryInfo;
use crate::{log, utils};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &[(String, CountryInfo)],
    continent_features: &HashMap<String, Vec<String>>,
    region_features: &HashMap<String, Vec<String>>,
    subregion_features: &HashMap<String, Vec<String>>,
    world_region_features: &HashMap<String, Vec<String>>,
    subdivision_type_name_list: &[String],
) -> Result<()> {
    let mut region_rs_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!("Could not open {:?}", destination_file))?;
    utils::write_first_comments(&mut region_rs_file, file!())?;
    region_rs_file.write_all(
        r#"
use crate::{SearchError, SearchedItems};
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all seven continents.
///
/// # Example
/// ```
/// use keshvar::{Continent, Alpha2};
///
/// let continent = Continent::try_from("australia").unwrap();
/// let country_list: Vec<_> = continent
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::NZ)); // New Zealand
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum Continent {
"#
        .as_bytes(),
    )?;
    let mut sorted_continent_features = continent_features.keys().collect::<Vec<_>>();
    sorted_continent_features.sort();
    for continent in &sorted_continent_features {
        for (_, info) in countries_info_list {
            if **continent == utils::to_cargo_feature_name(&info.continent) {
                region_rs_file
                    .write_all(format!("    /// * {}\n", utils::country_name(info)).as_bytes())?;
            }
        }
        region_rs_file.write_all(format!("    {},\n", utils::capitalize(continent)).as_bytes())?
    }
    region_rs_file.write_all(b"}\n")?;
    region_rs_file.write_all(
        r#"
impl Continent {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
"#
        .as_bytes(),
    )?;
    for continent in &sorted_continent_features {
        let countries = continent_features.get(*continent).unwrap();
        region_rs_file.write_all(
            format!("            Self::{} => {{\n", utils::capitalize(continent)).as_bytes(),
        )?;
        region_rs_file.write_all(
            format!(
                "                &{:?}\n",
                countries
                    .iter()
                    .map(|x| x.to_uppercase())
                    .collect::<Vec<_>>()
            )
            .as_bytes(),
        )?;
        region_rs_file.write_all(b"                }\n")?;
    }
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl TryFrom<&str> for Continent {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
"#
        .as_bytes(),
    )?;
    for continent in &sorted_continent_features {
        region_rs_file.write_all(
            format!(
                "            {:?} => Ok(Self::{}),\n",
                continent.to_lowercase(),
                utils::capitalize(continent)
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(b"            _ => Err(SearchError::NotFound { searched_items: SearchedItems::AllContinents }),\n")?;
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl ToString for Continent {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for continent in &sorted_continent_features {
        region_rs_file.write_all(
            format!(
                "            Self::{} => {:?},\n",
                utils::capitalize(continent),
                continent,
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(
        r#"        }.to_string()
    }
}"#
        .as_bytes(),
    )?;

    region_rs_file.write_all(
        r#"

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all six regions.
///
/// # Example
/// ```
/// use keshvar::{Region, Alpha2};
///
/// let region = Region::try_from("asia").unwrap();
/// let country_list: Vec<_> = region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::TR)); // Turkey
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum Region {
"#
        .as_bytes(),
    )?;
    for region in region_features.keys() {
        for (_, info) in countries_info_list {
            if let Some(ref country_region) = info.region {
                if region == &utils::to_cargo_feature_name(country_region) {
                    region_rs_file
                        .write_all(format!("    /// * {}\n", info.iso_long_name).as_bytes())?;
                }
            }
        }
        region_rs_file.write_all(format!("    {},\n", utils::capitalize(region)).as_bytes())?
    }
    region_rs_file.write_all(b"}\n")?;
    region_rs_file.write_all(
        r#"
impl Region {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
"#
        .as_bytes(),
    )?;
    let mut sorted_region_features = region_features.keys().collect::<Vec<_>>();
    sorted_region_features.sort();
    for region in &sorted_region_features {
        let countries = region_features.get(*region).unwrap();
        region_rs_file.write_all(
            format!("            Self::{} => {{\n", utils::capitalize(region)).as_bytes(),
        )?;
        region_rs_file.write_all(
            format!(
                "                &{:?}\n",
                countries
                    .iter()
                    .map(|x| x.to_uppercase())
                    .collect::<Vec<_>>()
            )
            .as_bytes(),
        )?;
        region_rs_file.write_all(b"                }\n")?;
    }
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl TryFrom<&str> for Region {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
"#
        .as_bytes(),
    )?;
    for region in &sorted_region_features {
        region_rs_file.write_all(
            format!(
                "            {:?} => Ok(Self::{}),\n",
                region.to_lowercase(),
                utils::capitalize(region)
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(b"            _ => Err(SearchError::NotFound { searched_items: SearchedItems::AllRegions }),\n")?;
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl ToString for Region {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for region in &sorted_region_features {
        region_rs_file.write_all(
            format!(
                "            Self::{} => {:?},\n",
                utils::capitalize(region),
                region,
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(
        r#"        }.to_string()
    }
}"#
        .as_bytes(),
    )?;

    region_rs_file.write_all(
        r#"
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing sub-regions.
///
/// # Example
/// ```
/// use keshvar::{SubRegion, Alpha2};
///
/// let sub_region = SubRegion::try_from("northern-america").unwrap();
/// let country_list: Vec<_> = sub_region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::CA)); // Canada
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum SubRegion {
"#
        .as_bytes(),
    )?;
    let mut sorted_subregion_features = subregion_features.keys().collect::<Vec<_>>();
    sorted_subregion_features.sort();
    for subregion in &sorted_subregion_features {
        for (_, info) in countries_info_list {
            if let Some(ref country_subregion) = info.subregion {
                if **subregion == utils::to_cargo_feature_name(country_subregion) {
                    region_rs_file
                        .write_all(format!("    /// * {}\n", info.iso_long_name).as_bytes())?;
                }
            }
        }
        region_rs_file.write_all(format!("    {},\n", utils::capitalize(subregion)).as_bytes())?
    }
    region_rs_file.write_all(b"}\n")?;
    region_rs_file.write_all(
        r#"
impl SubRegion {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
"#
        .as_bytes(),
    )?;
    for subregion in &sorted_subregion_features {
        let countries = subregion_features.get(*subregion).unwrap();
        region_rs_file.write_all(
            format!("            Self::{} => {{\n", utils::capitalize(subregion)).as_bytes(),
        )?;
        region_rs_file.write_all(
            format!(
                "                &{:?}\n",
                countries
                    .iter()
                    .map(|x| x.to_uppercase())
                    .collect::<Vec<_>>()
            )
            .as_bytes(),
        )?;
        region_rs_file.write_all(b"                }\n")?;
    }
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl TryFrom<&str> for SubRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
"#
        .as_bytes(),
    )?;
    for subregion in &sorted_subregion_features {
        region_rs_file.write_all(
            format!(
                "            {:?} => Ok(Self::{}),\n",
                subregion.to_lowercase(),
                utils::capitalize(subregion)
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(b"            _ => Err(SearchError::NotFound { searched_items: SearchedItems::AllSubRegions }),\n")?;
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl ToString for SubRegion {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for subregion in &sorted_subregion_features {
        region_rs_file.write_all(
            format!(
                "            Self::{} => {:?},\n",
                utils::capitalize(subregion),
                subregion,
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(
        r#"        }.to_string()
    }
}"#
        .as_bytes(),
    )?;

    region_rs_file.write_all(
        r#"
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing all four world regions.
///
/// # Example
/// ```
/// use keshvar::{WorldRegion, Alpha2};
///
/// let world_region = WorldRegion::try_from("AMER").unwrap();
/// let country_list: Vec<_> = world_region
///     .alpha2_list()
///     .iter()
///     .map(|alpha2_str| Alpha2::try_from(*alpha2_str).unwrap())
///     .collect();
/// assert!(country_list.contains(&Alpha2::JM)); // Jamaica
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum WorldRegion {
"#
        .as_bytes(),
    )?;
    let mut sorted_world_region_features = world_region_features.keys().collect::<Vec<_>>();
    sorted_world_region_features.sort();
    for world_region in &sorted_world_region_features {
        for (_, info) in countries_info_list {
            if **world_region == utils::to_cargo_feature_name(&info.world_region) {
                region_rs_file
                    .write_all(format!("    /// * {}\n", utils::country_name(info)).as_bytes())?;
            }
        }
        region_rs_file.write_all(
            format!("    {},\n", utils::capitalize(world_region).to_uppercase()).as_bytes(),
        )?
    }
    region_rs_file.write_all(b"}\n")?;
    region_rs_file.write_all(
        r#"
impl WorldRegion {
    /// You can try converting each item to [`Alpha2`](crate::Alpha2) and if you did not include some
    /// country features the conversion will fail.
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
"#
        .as_bytes(),
    )?;
    for world_region in &sorted_world_region_features {
        let countries = world_region_features.get(*world_region).unwrap();
        region_rs_file.write_all(
            format!(
                "            Self::{} => {{\n",
                utils::capitalize(world_region).to_uppercase()
            )
            .as_bytes(),
        )?;
        region_rs_file.write_all(
            format!(
                "                &{:?}\n",
                countries
                    .iter()
                    .map(|x| x.to_uppercase())
                    .collect::<Vec<_>>()
            )
            .as_bytes(),
        )?;
        region_rs_file.write_all(b"                }\n")?;
    }
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl TryFrom<&str> for WorldRegion {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
"#
        .as_bytes(),
    )?;
    for world_region in &sorted_world_region_features {
        region_rs_file.write_all(
            format!(
                "            {:?} => Ok(Self::{}),\n",
                world_region.to_lowercase(),
                utils::capitalize(world_region).to_uppercase()
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(b"            _ => Err(SearchError::NotFound { searched_items: SearchedItems::AllWorldRegions }),\n")?;
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
impl ToString for WorldRegion {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for world_region in &sorted_world_region_features {
        region_rs_file.write_all(
            format!(
                "            Self::{} => {:?},\n",
                utils::capitalize(world_region).to_uppercase(),
                world_region.to_uppercase(),
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(
        r#"        }.to_string()
    }
}"#
        .as_bytes(),
    )?;

    region_rs_file.write_all(
        r#"
#[cfg(feature = "subdivisions")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing countries subdivision types.
pub enum SubdivisionType {
"#
        .as_bytes(),
    )?;
    let max_example_count = 3;
    let mut sorted_subdivision_type_name_list = subdivision_type_name_list.to_owned();
    sorted_subdivision_type_name_list.sort();
    for subdivision_type_name in &sorted_subdivision_type_name_list {
        let mut example_count = 0;
        region_rs_file.write_all(b"    /// Examples:\n")?;
        'outer: for (_, info) in countries_info_list {
            for (_, subdivision) in &info.subdivisions {
                if subdivision_type_name == &subdivision._type {
                    region_rs_file.write_all(
                        format!(
                            "    /// * `{}` in `{}`\n",
                            subdivision.name,
                            utils::country_name(info)
                        )
                        .as_bytes(),
                    )?;
                    example_count += 1;
                    if example_count == max_example_count {
                        break 'outer;
                    }
                }
            }
        }
        region_rs_file
            .write_all(format!("    {},\n", utils::capitalize(subdivision_type_name)).as_bytes())?;
    }
    region_rs_file.write_all(b"}\n")?;
    region_rs_file.write_all(
        r#"
#[cfg(feature = "subdivisions")]
impl TryFrom<&str> for SubdivisionType {
    type Error = SearchError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
"#
        .as_bytes(),
    )?;
    for subdivision_type_name in &sorted_subdivision_type_name_list {
        region_rs_file.write_all(
            format!(
                "            {:?} => Ok(Self::{}),\n",
                subdivision_type_name.to_lowercase().replace('_', " "),
                utils::capitalize(subdivision_type_name)
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(b"            _ => Err(SearchError::NotFound { searched_items: SearchedItems::AllSubDivisionTypes }),\n")?;
    region_rs_file.write_all(
        r#"        }
    }
}"#
        .as_bytes(),
    )?;
    region_rs_file.write_all(
        r#"
#[cfg(feature = "subdivisions")]
impl ToString for SubdivisionType {
    fn to_string(&self) -> String {
        match self {
"#
        .as_bytes(),
    )?;
    for subdivision_type_name in &sorted_subdivision_type_name_list {
        region_rs_file.write_all(
            format!(
                "            Self::{} => {:?},\n",
                utils::capitalize(subdivision_type_name),
                subdivision_type_name,
            )
            .as_bytes(),
        )?
    }
    region_rs_file.write_all(
        r#"        }.to_string()
    }
}"#
        .as_bytes(),
    )?;

    log!("Generated {:?}", destination_file);

    Ok(())
}
