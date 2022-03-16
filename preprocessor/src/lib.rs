use ::std::default::Default;

mod api;
mod normalisation;
mod preprocess;
mod tokenise;

#[derive(Default, Debug)]
pub struct Preprocessor;

#[derive(Debug)]
pub struct ProcessingOptions {
    pub tokenisation_options: TokenisationOptions,
    pub fold_case: bool,
    pub remove_stop_words: bool,
    pub normalisation: Normalisation,
    pub remove_url: bool,
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        ProcessingOptions {
            tokenisation_options: Default::default(),
            fold_case: true,
            remove_stop_words: true,
            normalisation: Default::default(),
            remove_url: true,
        }
    }
}

#[derive(Debug)]
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
            remove_numbers: true,
            remove_coordinates: true,
            remove_dates: true,
            remove_special_characters: true,
            remove_punctuation: true,
        }
    }
}

#[derive(Debug)]
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
