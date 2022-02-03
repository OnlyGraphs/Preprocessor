// use api_rs::preprocessor::{Normalisation, ProcessingOptions, TokenisationOptions};
use ::std::default::Default;

mod api;
mod normalisation;
mod preprocess;
mod tokenise;

#[derive(Default)]
pub struct Preprocessor {
    pub(crate) processing_options: ProcessingOptions,
}

impl Preprocessor {
    pub fn new<T: Into<ProcessingOptions>>(options: T) -> Self {
        let processing_options = options.into();
        Self { processing_options }
    }
}

pub struct ProcessingOptions {
    pub tokenisation_options: TokenisationOptions,
    pub fold_case: bool,
    pub remove_stop_words: bool,
    pub normalisation: Normalisation,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        ProcessingOptions {
            tokenisation_options: Default::default(),
            fold_case: false,
            remove_stop_words: false,
            normalisation: Default::default(),
        }
    }
}

pub struct TokenisationOptions {
    pub remove_numbers: bool,
    pub remove_coordinates: bool,
    pub remove_dates: bool,
    pub remove_special_characters: bool,
    pub remove_punctuation: bool,
}

impl Default for TokenisationOptions {
    fn default() -> Self {
        TokenisationOptions {
            remove_numbers: false,
            remove_coordinates: false,
            remove_dates: false,
            remove_special_characters: false,
            remove_punctuation: false,
        }
    }
}

pub enum Normalisation {
    None,
    Stemming,
    Lemmatisation,
}

impl Default for Normalisation {
    fn default() -> Self {
        Normalisation::Stemming
    }
}