use std::convert::TryFrom;

use crate::{Normalisation, ProcessingOptions, TokenisationOptions};
use api_rs::preprocessor::{
    ProcessingOptions as ApiProcessingOptions, TokenisationOptions as ApiTokenisationOptions,
};

impl From<ApiTokenisationOptions> for TokenisationOptions {
    fn from(api_tokenisation_options: ApiTokenisationOptions) -> Self {
        let ApiTokenisationOptions {
            remove_numbers,
            remove_coordinates,
            remove_dates,
            remove_special_characters,
            remove_punctuation,
        } = api_tokenisation_options;

        TokenisationOptions {
            remove_numbers,
            remove_coordinates,
            remove_dates,
            remove_special_characters,
            remove_punctuation,
        }
    }
}

impl TryFrom<ApiProcessingOptions> for ProcessingOptions {
    type Error = String;
    fn try_from(api_processing_option: ApiProcessingOptions) -> Result<Self, Self::Error> {
        let ApiProcessingOptions {
            tokenisation_options,
            fold_case,
            remove_stop_words,
            normalisation,
        } = api_processing_option;

        let tokenisation_options: TokenisationOptions =
            tokenisation_options.map(|t| t.into()).ok_or(
                "The tokenisation options did not contain any tokenisation options: None"
                    .to_string(),
            )?;

        let normalisation = match normalisation {
            1 => Normalisation::Stemming,
            _ => Normalisation::None,
        };

        Ok(ProcessingOptions {
            tokenisation_options,
            fold_case,
            remove_stop_words,
            normalisation,
        })
    }
}
