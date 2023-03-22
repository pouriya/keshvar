use crate::{log, utils};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Country {
    Country(HashMap<serde_yaml::Value, CountryInfo>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryInfo {
    #[serde(default, skip)]
    pub module_name: String,
    #[serde(default, skip)]
    pub feature_name: String,
    #[serde(default, skip)]
    pub alpha2_upper: String,
    #[serde(default, skip)]
    pub alpha2_lower: String,
    #[serde(default, skip)]
    pub alpha3_upper: String,
    #[serde(default, skip)]
    pub alpha3_lower: String,
    #[serde(default, skip)]
    pub subdivisions: Vec<(String, Subdivision)>,
    #[serde(default, skip)]
    pub translation_list: Vec<(String, String)>,

    pub address_format: Option<String>,
    pub alpha2: String,
    pub alpha3: String,
    pub continent: String,
    pub country_code: String,
    pub currency_code: String,
    pub gec: Option<String>,
    pub geo: CountryGeo,
    pub international_prefix: String,
    pub ioc: Option<String>,
    pub iso_long_name: String,
    pub iso_short_name: String,
    pub languages_official: Vec<String>,
    pub languages_spoken: Vec<String>,
    pub national_destination_code_lengths: Vec<u8>,
    pub national_number_lengths: Vec<u8>,
    pub national_prefix: String,
    pub nationality: Option<String>,
    pub number: String,
    pub postal_code: bool,
    pub postal_code_format: Option<String>,
    pub region: Option<String>,
    pub start_of_week: String,
    pub subregion: Option<String>,
    pub un_locode: String,
    pub unofficial_names: Vec<String>,
    pub world_region: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryGeo {
    pub latitude: f64,
    pub longitude: f64,
    pub max_latitude: f64,
    pub max_longitude: f64,
    pub min_latitude: f64,
    pub min_longitude: f64,
    pub bounds: CountryGeoBounds,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryGeoBounds {
    pub northeast: CountryGeoBound,
    pub southwest: CountryGeoBound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryGeoBound {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subdivision {
    #[serde(default, skip)]
    pub translations_sorted: Vec<(String, String)>,
    #[serde(default, skip)]
    pub unofficial_names_list: Vec<String>,

    pub name: String,
    pub unofficial_names: Option<SubdivisionUnofficialName>,
    pub code: String,
    pub geo: Option<SubdivisionGeo>,
    pub translations: HashMap<String, String>,
    pub comments: Option<String>,
    #[serde(alias = "type")]
    pub _type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubdivisionGeo {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub max_latitude: Option<f64>,
    pub max_longitude: Option<f64>,
    pub min_latitude: Option<f64>,
    pub min_longitude: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubdivisionUnofficialName {
    ListOfNames(Vec<String>),
    Name(String),
}

pub fn load_country_info(filename: &PathBuf) -> Result<(String, CountryInfo)> {
    let file = fs::File::open(filename.clone())
        .context(format!("Could not open country info file {:?}", filename))?;
    let file_reader = BufReader::new(file);
    let Country::Country(data) = serde_yaml::from_reader(file_reader)
        .context(format!("Could not deserialize {:?} data", filename))?;
    let object_count = data.len();
    if object_count != 1 {
        bail!(
            "{} object inside {:?}",
            if object_count > 1 {
                "There is more than 1"
            } else {
                "There is no"
            },
            filename
        )
    }
    let (name, mut info) = data.into_iter().next().unwrap();
    let name = name.as_str().unwrap().to_string();
    // Test all `String` values and if one is empty we should panic and make all of them `Option<String>`
    for (value, name) in [
        (&info.alpha2, "alpha2"),
        (&info.alpha3, "alpha3"),
        (&info.continent, "continent"),
        (&info.country_code, "country_code"),
        (&info.currency_code, "currency_code"),
        (&info.iso_short_name, "iso_short_name"),
        (&info.iso_long_name, "iso_long_name"),
        (&info.number, "number"),
        (&info.start_of_week, "start_of_week"),
        (&info.un_locode, "un_locode"),
        (&info.world_region, "world_region"),
    ] {
        if value.trim().is_empty() {
            bail!("`{}` inside filename {:?} is empty!", name, filename)
        }
    }
    info.alpha2_upper = info.alpha2.to_uppercase();
    info.alpha2_lower = info.alpha2.to_lowercase();
    info.alpha3_upper = info.alpha3.to_uppercase();
    info.alpha3_lower = info.alpha3.to_lowercase();
    info.feature_name = info.alpha2_lower.clone();
    info.module_name = utils::to_module_name(&info.alpha2);
    info.languages_spoken.sort();
    info.languages_official.sort();

    // Replace `Some("\"\"")` with `None`:
    utils::set_none_if_empty_string(&mut info.address_format);
    utils::set_none_if_empty_string(&mut info.gec);
    utils::set_none_if_empty_string(&mut info.ioc);
    utils::set_none_if_empty_string(&mut info.nationality);
    utils::set_none_if_empty_string(&mut info.postal_code_format);
    utils::set_none_if_empty_string(&mut info.region);
    utils::set_none_if_empty_string(&mut info.subregion);
    log!(
        "Loaded and decoded country information for `{}` from {:?}",
        name,
        filename
    );
    Ok((name, info))
}

pub fn load_country_subdivisions(filename: &PathBuf) -> Result<Vec<(String, Subdivision)>> {
    let mut subdivision_list = Vec::new();
    if !filename.exists() {
        log!("Subdivision file {:?} does not exist", filename);
        return Ok(subdivision_list);
    }
    let file = fs::File::open(filename)
        .context(format!("Could not open subdivision file {:?}", filename))?;
    let file_reader = BufReader::new(file);
    let mut subdivisions: HashMap<String, Subdivision> = serde_yaml::from_reader(file_reader)
        .context(format!(
            "Could not deserialize subdivisions data from {:?}",
            filename
        ))?;
    for (name, subdivision) in subdivisions.iter_mut() {
        for (value, field_name) in [
            (&subdivision._type, "type"),
            (&subdivision.code, "code"),
            (&subdivision.name, "name"),
        ] {
            if value.trim().is_empty() {
                bail!(
                    "`{}` inside subdivisions filename {:?} for subdivision {} is empty!",
                    field_name,
                    filename,
                    name
                )
            }
        }
        for (language, translated) in subdivision.translations.iter() {
            if language.trim().is_empty() {
                bail!(
                    "Empty language key inside subdivision file {:?} for subdivision {} key",
                    filename,
                    name,
                )
            }
            if translated.trim().is_empty() {
                panic!(
                    "Empty translation value inside subdivision file {:?} for subdivision {} and language {}",
                    filename,
                    name,
                    language
                )
            }
            subdivision
                .translations_sorted
                .push((language.clone(), translated.clone()))
        }
        subdivision.translations_sorted.sort_by_key(|x| x.0.clone());
        subdivision.translations = HashMap::new();
        subdivision.unofficial_names_list = match subdivision.unofficial_names.clone() {
            Some(SubdivisionUnofficialName::ListOfNames(mut list)) => {
                list.sort();
                list
            }
            Some(SubdivisionUnofficialName::Name(name)) => Vec::from([name]),
            None => Vec::new(),
        };
        subdivision.unofficial_names = None;
        utils::set_none_if_empty_string(&mut subdivision.comments);
        if let Some(ref geo) = subdivision.geo {
            if let (None, None, None, None, None, None) = (
                geo.latitude,
                geo.longitude,
                geo.max_latitude,
                geo.min_latitude,
                geo.max_longitude,
                geo.min_longitude,
            ) {
                subdivision.geo = None
            }
        };
        subdivision_list.push((name.clone(), subdivision.clone()))
    }
    log!(
        "Loaded and decoded subdivision information from {:?}",
        filename
    );
    subdivision_list.sort_by_key(|x| x.0.clone());
    Ok(subdivision_list)
}
