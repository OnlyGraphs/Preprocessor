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
            remove_url,
        } = processing_options.into();

        let tokens = Self::tokenise(tokenisation_options, raw_text);
        let tokens = Self::fold(*fold_case, tokens);
        let tokens = Self::remove_urls(*remove_url, tokens);
        let tokens = Self::stopping(*remove_stop_words, tokens);
        let tokens = Self::normalise(normalisation, tokens);
        tokens
    }

    pub fn remove_urls(remove_url: bool, tokens: Vec<String>) -> Vec<String> {
        if remove_url {
            tokens
                .into_iter()
                .filter(|t| !(t.starts_with("http") && t.len() > 5))
                .collect()
        } else {
            tokens
        }
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
    use crate::{Normalisation, Preprocessor, ProcessingOptions};

    #[test]
    fn stem() {
        let normalisation = Normalisation::Stemming;
        let result = Preprocessor::normalise(
            &normalisation,
            vec!["Throwing".to_string(), "a".to_string(), "ball.".to_string()],
        );
        assert_eq!(
            result,
            vec!["Throw".to_string(), "a".to_string(), "ball.".to_string()]
        );
    }

    #[test]
    fn remove_urls1() {
        let text = vec![
            "httpwwwhellocom".to_owned(),
            "https".to_owned(),
            "http".to_owned(),
            "httpskylecottonnet".to_owned(),
        ];
        let result = Preprocessor::remove_urls(true, text);
        assert_eq!(result, vec!["https".to_owned(), "http".to_owned()]);
    }

    #[test]
    fn full_text_urls() {
        let text = "This is a very long message with url https://blog.kylecotton.net with words in the middle https, http; http://www.somesite.com".to_owned();
        let result = Preprocessor::process(&ProcessingOptions::default(), text);
        assert_eq!(
            result,
            vec![
                "messag".to_owned(),
                "url".to_owned(),
                "middl".to_owned(),
                "https".to_owned()
            ]
        );
    }

    #[test]
    fn http_text() {
        let text = "http".to_owned();
        let result = Preprocessor::process(
            &ProcessingOptions {
                remove_stop_words: false,
                ..Default::default()
            },
            text,
        );

        assert_eq!(result, vec!["http".to_owned()]);
    }
}
