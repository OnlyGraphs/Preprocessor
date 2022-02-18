use crate::Preprocessor;
use crate::ProcessingOptions;
use std::collections::HashSet;

lazy_static::lazy_static! {
    // Define the stop words as constant
    static ref STOP_WORDS: HashSet<String> = HashSet::from_iter(
        stop_words::get(stop_words::LANGUAGE::English)
    );
}

impl Preprocessor {
    pub fn process<'a, T: Into<&'a ProcessingOptions>>(
        processing_options: T,
        raw_text: String,
    ) -> Vec<String> {
        let ProcessingOptions {
            tokenisation_options,
            fold_case,
            remove_stop_words,
            normalisation,
        } = processing_options.into();

        let tokens = Self::tokenise(tokenisation_options, raw_text);
        let tokens = Self::fold(*fold_case, tokens);
        let tokens = Self::stopping(*remove_stop_words, tokens);
        let tokens = Self::normalise(normalisation, tokens);
        tokens
    }

    pub fn fold(fold_case: bool, tokens: Vec<String>) -> Vec<String> {
        if fold_case {
            tokens.iter().map(|t| t.to_lowercase()).collect()
        } else {
            tokens
        }
    }

    pub fn stopping(remove_stop_words: bool, tokens: Vec<String>) -> Vec<String> {
        if remove_stop_words {
            tokens
                .into_iter()
                .filter(|t| !STOP_WORDS.contains(t))
                .collect()
        } else {
            tokens
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Normalisation, Preprocessor};

    #[test]
    fn stem() {
        let normalisation = Normalisation::Stemming;
        let result = Preprocessor::normalise(&normalisation, vec![
            "Throwing".to_string(),
            "a".to_string(),
            "ball.".to_string(),
        ]);
        assert_eq!(
            result,
            vec!["Throw".to_string(), "a".to_string(), "ball.".to_string()]
        );
    }
}
