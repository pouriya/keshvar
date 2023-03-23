use crate::structs::CountryInfo;
use crate::utils;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &Vec<(String, CountryInfo)>,
    region_features: &HashMap<String, Vec<String>>,
    subregion_features: &HashMap<String, Vec<String>>,
    world_region_features: &HashMap<String, Vec<String>>,
) -> Result<()> {
    let mut consts_rs_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!("Could not open {:?}", destination_file))?;
    utils::write_first_comments(&mut consts_rs_file, file!())?;
    consts_rs_file.write_all(b"#[allow(unused_imports)]\n")?;
    consts_rs_file.write_all(b"use crate::{Alpha2, Region, SubRegion, WorldRegion};\n")?;
    consts_rs_file.write_all(b"use lazy_static::lazy_static;\n")?;
    consts_rs_file.write_all(b"use hashbrown::HashMap;\n")?;
    consts_rs_file.write_all(
        format!(
            "pub const ALL_COUNTRIES_COUNT: usize = {};",
            countries_info_list.len()
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(
        r#"
lazy_static! { pub static ref SUPPORTED_COUNTRIES_COUNT: usize = SUPPORTED_ALPHA2_LIST.len(); }
lazy_static! { pub static ref UNSUPPORTED_COUNTRIES_COUNT: usize = ALL_COUNTRIES_COUNT - *SUPPORTED_COUNTRIES_COUNT; }
"#
        .as_bytes(),
    )?;

    consts_rs_file.write_all(
        utils::country_cfg_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "all",
            0,
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORT_ALL_COUNTRIES: bool = true;\n")?;
    consts_rs_file.write_all(
        utils::country_cfg_not_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "all",
            0,
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORT_ALL_COUNTRIES: bool = false;\n")?;

    // consts_rs_file.write_all(
    //     b"lazy_static! { pub static ref ALL_ALPHA2_LIST: &'static [&'static str] = &[\n",
    // )?;
    // for (_, info) in countries_info_list.iter() {
    //     consts_rs_file.write_all(
    //         format!(
    //             "    {:?}, {}",
    //             info.alpha2_upper,
    //             utils::commented_country_name(info)
    //         )
    //         .as_bytes(),
    //     )?;
    // }
    // consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file.write_all(b"pub const SUPPORTED_ALPHA2_LIST: &[Alpha2] = &[\n")?;
    for (_, info) in countries_info_list.iter() {
        consts_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
        consts_rs_file.write_all(format!("    Alpha2::{},\n", info.alpha2_upper).as_bytes())?;
    }
    consts_rs_file.write_all(b"];\n")?;

    // consts_rs_file.write_all(
    //     b"lazy_static! { pub static ref UNSUPPORTED_ALPHA2_LIST: &'static [&'static str] = &[\n",
    // )?;
    // for (_, info) in countries_info_list.iter() {
    //     consts_rs_file.write_all(
    //         format!(
    //             "    #[cfg(not(feature = {:?}))] {}",
    //             info.feature_name,
    //             utils::commented_country_name(info)
    //         )
    //         .as_bytes(),
    //     )?;
    //     consts_rs_file.write_all(format!("    {:?},\n", info.alpha2_upper).as_bytes())?;
    // }
    // consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file.write_all(
        b"lazy_static! { pub static ref SUPPORTED_REGION_LIST: &'static [Region] = &[\n",
    )?;
    let mut sorted_region_features = region_features.keys().collect::<Vec<_>>();
    sorted_region_features.sort();
    for region in sorted_region_features {
        let feature_list = region_features.get(region).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file
            .write_all(format!("    Region::{},\n", utils::capitalize(region)).as_bytes())?;
    }
    consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file.write_all(
        b"lazy_static! { pub static ref SUPPORTED_SUBREGION_LIST: &'static [SubRegion] = &[\n",
    )?;
    let mut sorted_subregion_features = subregion_features.keys().collect::<Vec<_>>();
    sorted_subregion_features.sort();
    for subregion in sorted_subregion_features {
        let feature_list = subregion_features.get(subregion).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file
            .write_all(format!("    SubRegion::{},\n", utils::capitalize(subregion)).as_bytes())?;
    }
    consts_rs_file.write_all(b"];}\n")?;
    consts_rs_file.write_all(
        b"lazy_static! { pub static ref SUPPORTED_WORLD_REGION_LIST: &'static [WorldRegion] = &[\n",
    )?;
    let mut sorted_world_region_features = world_region_features.keys().collect::<Vec<_>>();
    sorted_world_region_features.sort();
    for world_region in sorted_world_region_features {
        let feature_list = world_region_features.get(world_region).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file.write_all(
            format!(
                "    WorldRegion::{},\n",
                utils::capitalize(world_region).to_uppercase()
            )
            .as_bytes(),
        )?;
    }
    consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file
        .write_all(b"lazy_static! { pub static ref SUPPORTED_ISO_SHORT_NAMES: HashMap<&'static str, Alpha2> = HashMap::from([\n")?;
    for (_, info) in countries_info_list.iter() {
        consts_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
        consts_rs_file.write_all(
            format!(
                "    ({:?}, Alpha2::{}),\n",
                info.iso_short_name.to_lowercase(),
                info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    consts_rs_file.write_all(b"]);}\n")?;

    consts_rs_file
        .write_all(b"lazy_static! { pub static ref SUPPORTED_ISO_LONG_NAMES: HashMap<&'static str, Alpha2> = HashMap::from([\n")?;
    for (_, info) in countries_info_list.iter() {
        consts_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
        consts_rs_file.write_all(
            format!(
                "    ({:?}, Alpha2::{}),\n",
                info.iso_long_name.to_lowercase(),
                info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    consts_rs_file.write_all(b"]);}\n")?;

    consts_rs_file
        .write_all(b"lazy_static! { pub static ref SUPPORTED_COUNTRY_CODE: HashMap<usize, Alpha2> = HashMap::from([\n")?;
    for (_, info) in countries_info_list.iter() {
        consts_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
        consts_rs_file.write_all(
            format!(
                "    ({}, Alpha2::{}),\n",
                info.country_code, info.alpha2_upper
            )
            .as_bytes(),
        )?;
    }
    consts_rs_file.write_all(b"]);}\n")?;
    Ok(())
}
