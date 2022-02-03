use crate::Preprocessor;

impl Preprocessor {
    pub fn process(&self, raw_text: String) -> Vec<String> {
        let tokens = self.tokenise(raw_text);
        let tokens = self.fold(tokens);
        let tokens = self.stopping(tokens);
        let tokens = self.normalise(tokens);
        tokens
    }

    pub fn fold(&self, tokens: Vec<String>) -> Vec<String> {
        if self.processing_options.fold_case {
            tokens.iter().map(|t| t.to_lowercase()).collect()
        } else {
            tokens
        }
    }

    pub fn stopping(&self, tokens: Vec<String>) -> Vec<String> {
        let stop_words = stop_words::get(stop_words::LANGUAGE::English);
        if self.processing_options.remove_stop_words {
            tokens
                .into_iter()
                .filter(|t| !stop_words.contains(t))
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
        let preprocessor = Preprocessor {
            processing_options: ProcessingOptions {
                normalisation: Normalisation::Stemming,
                ..Default::default()
            },
        };
        let result = preprocessor.normalise(vec![
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
