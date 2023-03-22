use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_translations(filename: &PathBuf) -> Result<HashMap<String, String>> {
    let file = fs::File::open(filename.clone())
        .context(format!("Could not open translation file {:?}", filename))?;
    let file_reader = BufReader::new(file);
    let file_translations: HashMap<String, String> = serde_yaml::from_reader(file_reader)
        .context(format!("Could not deserialize {:?} data", filename))?;
    let mut translations = HashMap::new();
    for (alpha2, translation) in file_translations {
        translations.insert(alpha2.to_uppercase(), translation);
    }
    Ok(translations)
}
