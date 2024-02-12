use crate::structs::CountryInfo;
use anyhow::{Context, Result};
use std::fs;
use std::io::BufReader;
use std::path::Path;

pub fn read_from_file<F: AsRef<Path>>(
    filename: F,
    countries_info_list: &mut Vec<(String, CountryInfo)>,
) -> Result<()> {
    let file = fs::File::open(&filename).context(format!(
        "Could not open population file {:?}",
        filename.as_ref()
    ))?;
    let file_reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(file_reader);
    for record_result in csv_reader.records() {
        let record = record_result?;
        if record.len() < 3 {
            continue;
        }
        let alpha3 = &record[1];
        if let Some((_, info)) = countries_info_list
            .iter_mut()
            .find(|(_, info)| info.alpha3 == alpha3)
        {
            info.population = if record.iter().last().unwrap().trim().is_empty() {
                &record[record.len() - 2]
            } else {
                record.iter().last().unwrap()
            }
            .parse()
            .unwrap()
        }
    }
    Ok(())
}
