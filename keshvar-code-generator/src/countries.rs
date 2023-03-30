use crate::structs::CountryInfo;
use crate::{log, utils};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_mod(
    destination_file: &PathBuf,
    countries_info_list: &Vec<(String, CountryInfo)>,
) -> Result<()> {
    let mut mod_rs_file = fs::File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!("Could not open {:?}", destination_file))?;
    utils::write_first_comments(&mut mod_rs_file, file!())?;
    // `filename_list` is sorted so we iterate over it and lookup info from `countries_info`:
    for (_, info) in countries_info_list {
        mod_rs_file
            .write_all(utils::country_cfg_feature_and_doc_commented_name(info, 0).as_bytes())?;
        mod_rs_file.write_all(format!("pub mod {};\n", info.module_name).as_bytes())?;
    }
    log!("Generated {:?}", destination_file);
    Ok(())
}

pub fn generate_country(destination_file: &PathBuf, info: &CountryInfo) -> Result<()> {
    let mut file = fs::File::options()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!(
            "Could not open file {:?} to write",
            destination_file
        ))?;
    utils::write_first_comments(&mut file, file!())?;
    file.write_all(format!("// {}\n\n", info.iso_long_name.clone()).as_bytes())?;
    file.write_all(
        format!(
            "#[cfg(all(feature = {:?}, feature = \"constants\"))]\n",
            info.feature_name
        )
        .as_bytes(),
    )?;
    file.write_all(
        format!(
            r#"/// A module to access all constant country data for `{}`.
///
/// Note that to use this module, `constant` feature should be enabled.
"#,
            info.iso_long_name.clone()
        )
        .as_bytes(),
    )?;
    file.write_all(b"pub mod consts {\n")?;
    file.write_all(b"    #[allow(unused_imports)]\n")?;
    file.write_all(
        b"    use crate::{Alpha2, Alpha3, GEC, IOC, CurrencyCode, Continent, Region, SubRegion, WorldRegion, WeekDay};\n\n",
    )?;
    for (name, _type, value) in [
        (
            "const ADDRESS_FORMAT",
            "Option<&str>",
            utils::option_string_to_string(&info.address_format),
        ),
        (
            "const ALPHA2",
            "Alpha2",
            format!("Alpha2::{}", info.alpha2_upper),
        ),
        (
            "const ALPHA3",
            "Alpha3",
            format!("Alpha3::{}", info.alpha3_upper),
        ),
        (
            "const CONTINENT",
            "Continent",
            format!("Continent::{}", utils::capitalize(&info.continent)),
        ),
        ("const COUNTRY_CODE", "usize", info.country_code.to_string()),
        (
            "const CURRENCY_CODE",
            "CurrencyCode",
            format!("CurrencyCode::{}", info.currency_code),
        ),
        (
            "const GEC",
            "Option<GEC>",
            if let Some(ref gec) = info.gec {
                format!("Some(GEC::{})", utils::capitalize(gec).to_uppercase())
            } else {
                "None".to_string()
            },
        ),
        (
            "const INTERNATIONAL_PREFIX",
            "&str",
            format!("{:?}", info.international_prefix),
        ),
        (
            "const IOC",
            "Option<IOC>",
            if let Some(ref ioc) = info.ioc {
                format!("Some(IOC::{})", utils::capitalize(ioc).to_uppercase())
            } else {
                "None".to_string()
            },
        ),
        (
            "const ISO_SHORT_NAME",
            "&str",
            format!("{:?}", info.iso_short_name),
        ),
        (
            "const ISO_LONG_NAME",
            "&str",
            format!("{:?}", info.iso_long_name),
        ),
        (
            "const OFFICIAL_LANGUAGE_LIST",
            "&[&str]",
            format!("&{:?}", info.languages_official),
        ),
        (
            "const SPOKEN_LANGUAGE_LIST",
            "&[&str]",
            format!("&{:?}", info.languages_spoken),
        ),
        (
            "const NATIONAL_DESTINATION_CODE_LENGTH_LIST",
            "&[usize]",
            format!("&{:?}", info.national_destination_code_lengths),
        ),
        (
            "const NATIONAL_NUMBER_LENGTH_LIST",
            "&[usize]",
            format!("&{:?}", info.national_number_lengths),
        ),
        (
            "const NATIONAL_PREFIX",
            "&str",
            format!("{:?}", info.national_prefix),
        ),
        (
            "const NATIONALITY",
            "Option<&str>",
            utils::option_string_to_string(&info.nationality),
        ),
        ("const NUMBER", "&str", format!("{:?}", info.number)),
        (
            "const POSTAL_CODE",
            "bool",
            format!("{:?}", info.postal_code),
        ),
        (
            "const POSTAL_CODE_FORMAT",
            "Option<&str>",
            utils::option_string_to_string(&info.postal_code_format),
        ),
        (
            "const REGION",
            "Option<Region>",
            if let Some(ref region) = info.region {
                format!("Some(Region::{})", utils::capitalize(region))
            } else {
                "None".to_string()
            },
        ),
        (
            "const START_DAY_OF_WEEK",
            "WeekDay",
            format!("WeekDay::{}", utils::capitalize(&info.start_of_week)),
        ),
        (
            "const SUBREGION",
            "Option<SubRegion>",
            if let Some(ref subregion) = info.subregion {
                format!("Some(SubRegion::{})", utils::capitalize(subregion))
            } else {
                "None".to_string()
            },
        ),
        ("const UN_LOCODE", "&str", format!("{:?}", info.un_locode)),
        (
            "const UNOFFICIAL_NAME_LIST",
            "&[&str]",
            format!("&{:?}", info.unofficial_names),
        ),
        (
            "const WORLD_REGION",
            "WorldRegion",
            format!(
                "WorldRegion::{}",
                utils::capitalize(&info.world_region).to_uppercase()
            ),
        ),
    ] {
        file.write_all(format!("    pub {}: {} = {};\n", name, _type, value).as_bytes())?;
    }
    file.write_all(b"    #[cfg(feature = \"translations\")]\n")?;
    file.write_all(b"    pub const TRANSLATIONS: &[(&str, &str)] = &[\n")?;
    for (language, translation) in info.translation_list.iter() {
        file.write_all(format!("        ({:?}, {:?}),\n", language, translation).as_bytes())?;
    }
    file.write_all(b"];\n")?;

    file.write_all(
        format!(
            "    #[cfg(all(feature = {:?}, feature = \"geo\", feature = \"constants\"))]\n",
            info.feature_name
        )
        .as_bytes(),
    )?;
    file.write_all(b"    /// GEO data as constants\n")?;
    file.write_all(b"    pub mod geo {\n")?;
    for (name, _type, value) in [
        ("const LATITUDE", "f64", info.geo.latitude),
        ("const LONGITUDE", "f64", info.geo.longitude),
        ("const MAX_LATITUDE", "f64", info.geo.max_latitude),
        ("const MAX_LONGITUDE", "f64", info.geo.max_longitude),
        ("const MIN_LATITUDE", "f64", info.geo.min_latitude),
        ("const MIN_LONGITUDE", "f64", info.geo.min_longitude),
        (
            "const NORTHEAST_LATITUDE",
            "f64",
            info.geo.bounds.northeast.lat,
        ),
        (
            "const NORTHEAST_LONGITUDE",
            "f64",
            info.geo.bounds.northeast.lng,
        ),
        (
            "const SOUTHWEST_LATITUDE",
            "f64",
            info.geo.bounds.southwest.lat,
        ),
        (
            "const SOUTHWEST_LONGITUDE",
            "f64",
            info.geo.bounds.southwest.lng,
        ),
    ] {
        file.write_all(
            format!(
                "        pub {}: {} = {};\n",
                name,
                _type,
                utils::to_float_string(value)
            )
            .as_bytes(),
        )?;
    }
    file.write_all(b"    }\n}\n")?;
    file.write_all(
        format!(
            "#[cfg(all(feature = {:?}, feature = \"geo\"))]\n",
            info.feature_name
        )
        .as_bytes(),
    )?;
    file.write_all(b"/// GEO module for this country.\n")?;
    file.write_all(b"pub mod geo {\n")?;
    file.write_all(b"    use crate::{CountryGeo, CountryGeoBounds, CountryGeoBound};\n")?;
    file.write_all(
        format!(
            r#"
    /// GEO information for this country.
    pub fn new() -> CountryGeo {{
        CountryGeo {{
            latitude: {},
            longitude: {},
            max_latitude: {},
            max_longitude: {},
            min_latitude: {},
            min_longitude: {},
            bounds: CountryGeoBounds {{
                northeast: CountryGeoBound {{
                    latitude: {},
                    longitude: {},
                }},
                southwest: CountryGeoBound {{
                    latitude: {},
                    longitude: {},
                }},
            }},
        }}
    }}
}}
"#,
            utils::to_float_string(info.geo.latitude),
            utils::to_float_string(info.geo.longitude),
            utils::to_float_string(info.geo.max_latitude),
            utils::to_float_string(info.geo.max_longitude),
            utils::to_float_string(info.geo.min_latitude),
            utils::to_float_string(info.geo.min_longitude),
            utils::to_float_string(info.geo.bounds.northeast.lat),
            utils::to_float_string(info.geo.bounds.northeast.lng),
            utils::to_float_string(info.geo.bounds.southwest.lat),
            utils::to_float_string(info.geo.bounds.southwest.lng),
        )
        .as_bytes(),
    )?;
    file.write_all(
        format!(
            "\n#[cfg(all(feature = {:?}, feature = \"subdivisions\"))]\n",
            info.feature_name
        )
        .as_bytes(),
    )?;
    file.write_all(b"/// Subdivision module for this country.\n")?;
    file.write_all(b"pub mod subdivisions {\n")?;
    file.write_all(b"    use std::collections::HashMap;\n")?;
    file.write_all(b"    #[allow(unused_imports)]\n")?;
    file.write_all(b"    use crate::{Subdivision, Alpha2, SubdivisionType};\n")?;
    file.write_all(b"    // In this state, We do not know if subdivisions have geo or not!\n")?;
    file.write_all(b"    #[cfg(feature = \"geo\")]\n")?;
    file.write_all(b"    #[allow(unused_imports)]\n")?;
    file.write_all(b"    use crate::SubdivisionGeo;\n\n")?;
    file.write_all(b"    /// Subdivisions for this country.\n")?;
    file.write_all(b"    pub fn new() -> HashMap<&'static str, Subdivision> {\n")?;
    file.write_all(b"        HashMap::from(\n")?;
    file.write_all(b"            [\n")?;
    if !info.subdivisions.is_empty() {
        for (_, value) in info.subdivisions.iter() {
            file.write_all(
                format!(r#"
                (
                    {:?},
                    Subdivision{{
                        name: {:?},
                        country_alpha2: Alpha2::{},
                        code: {:?},
                        #[cfg(feature = "geo")]
                        geo: {},
                        comments: None,
                        subdivision_type: SubdivisionType::{},
                        #[cfg(feature = "translations")]
                        translations: HashMap::from({:?}),
                        unofficial_name_list: {:?}.to_vec(),
                    }}
                ),"#,
                        value.code,
                        value.name,
                        info.alpha2_upper,
                        value.code,
                        if let Some(ref geo) = value.geo {
                            format!("Some(SubdivisionGeo{{latitude: {}, longitude: {}, max_latitude: {}, min_latitude: {}, max_longitude: {}, min_longitude: {}}})",
                                    utils::option_f64_to_string(&geo.latitude),
                                    utils::option_f64_to_string(&geo.longitude),
                                    utils::option_f64_to_string(&geo.max_latitude),
                                    utils::option_f64_to_string(&geo.min_latitude),
                                    utils::option_f64_to_string(&geo.max_longitude),
                                    utils::option_f64_to_string(&geo.min_longitude),
                            )
                        } else {
                            "None".to_string()
                        },
                        utils::capitalize(&value._type),
                        value.translations_sorted,
                        value.unofficial_names_list,
                ).as_bytes())?;
        }
    }
    file.write_all(b"\n            ]\n")?;
    file.write_all(b"\n        )\n")?;
    file.write_all(b"    }\n")?;
    file.write_all(b"}\n")?;
    file.write_all(b"#[allow(unused_imports)]\n")?;
    file.write_all(
        b"use crate::{Alpha2, Alpha3, GEC, IOC, Country, CurrencyCode, Continent, Region, SubRegion, WorldRegion, WeekDay};\n",
    )?;
    file.write_all(b"#[allow(unused_imports)]\n")?;
    file.write_all(b"use std::collections::HashMap;\n")?;
    file.write_all(format!("#[cfg(feature = {:?})]", info.feature_name).as_bytes())?;
    file.write_all(
        format!(
            r#"
/// [`Country`](crate::Country) struct for this country.
pub fn new() -> Country {{
    Country{{
        alpha2: Alpha2::{},
        alpha3: Alpha3::{},
        address_format: {},
        continent: Continent::{},
        country_code: {},
        currency_code: CurrencyCode::{},
        gec: {},
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: {:?},
        ioc: {},
        iso_long_name: {:?},
        iso_short_name: {:?},
        official_language_list: {:?}.to_vec(),
        spoken_language_list: {:?}.to_vec(),
        national_destination_code_length_list: {:?}.to_vec(),
        national_number_length_list: {:?}.to_vec(),
        national_prefix: {:?},
        nationality: {},
        number: {:?},
        postal_code: {:?},
        postal_code_format: {},
        region: {},
        start_of_week: WeekDay::{},
        subregion: {},
        un_locode: {:?},
        unofficial_name_list: {:?}.to_vec(),
        world_region: WorldRegion::{},
        #[cfg(feature = "translations")]
        translations: HashMap::from({:?}),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }}
}}
            "#,
            info.alpha2_upper,
            info.alpha3_upper,
            utils::option_string_to_string(&info.address_format),
            utils::capitalize(&info.continent),
            info.country_code,
            info.currency_code,
            if let Some(ref gec) = info.gec {
                format!("Some(GEC::{})", utils::capitalize(gec).to_uppercase())
            } else {
                "None".to_string()
            },
            info.international_prefix,
            if let Some(ref ioc) = info.ioc {
                format!("Some(IOC::{})", utils::capitalize(ioc).to_uppercase())
            } else {
                "None".to_string()
            },
            info.iso_long_name,
            info.iso_short_name,
            info.languages_official,
            info.languages_spoken,
            info.national_destination_code_lengths,
            info.national_number_lengths,
            info.national_prefix,
            utils::option_string_to_string(&info.nationality),
            info.number,
            info.postal_code,
            utils::option_string_to_string(&info.postal_code_format),
            if let Some(ref region) = info.region {
                format!("Some(Region::{})", utils::capitalize(region))
            } else {
                "None".to_string()
            },
            utils::capitalize(&info.start_of_week),
            if let Some(ref subregion) = info.subregion {
                format!("Some(SubRegion::{})", utils::capitalize(subregion))
            } else {
                "None".to_string()
            },
            info.un_locode,
            info.unofficial_names,
            utils::capitalize(&info.world_region).to_uppercase(),
            info.translation_list
        )
        .as_bytes(),
    )?;
    log!(
        "Generated country module {:?} for {:?}",
        destination_file,
        info.iso_long_name
    );
    Ok(())
}
