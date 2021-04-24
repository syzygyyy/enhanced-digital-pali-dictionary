use crate::inflection_generator::InflectionGenerator;
use crate::stardict::input_parsers::PaliWord;
use crate::{create_base_path, write_dictionary, DictionaryInfo, EdpdLogger};
use std::path::Path;

mod glib;
pub(crate) mod input_parsers;
mod output_generators;

pub struct StarDictFile {
    pub extension: String,
    pub data: Vec<u8>,
}

pub fn run_for_ods_type<'a, T: 'a + serde::de::DeserializeOwned + PaliWord>(
    dict_info: &DictionaryInfo,
    csv_path: &Path,
    igen: &dyn InflectionGenerator,
    logger: &dyn EdpdLogger,
) -> Result<(), String> {
    let words = input_parsers::load_words::<T>(csv_path, logger)?;
    let sd_files = output_generators::create_dictionary(&dict_info, words, igen, logger)?;

    let base_path = create_base_path(csv_path, dict_info.ods_type)?;
    write_dictionary(&base_path, sd_files, logger)
}