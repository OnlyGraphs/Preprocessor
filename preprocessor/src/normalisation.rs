use crate::{Normalisation, Preprocessor};
use rust_stemmers::{Algorithm, Stemmer};

impl Preprocessor {
    pub fn normalise(&self, tokens: Vec<String>) -> Vec<String> {
        match self.processing_options.normalisation {
            Normalisation::None => tokens,
            // TODO: Implement a specific normalisation for Lemmatisation
            Normalisation::Stemming | Normalisation::Lemmatisation => {
                let stemmer = Stemmer::create(Algorithm::English);
                tokens
                    .iter()
                    .map(|t| stemmer.stem(t).into_owned())
                    .collect()
            }
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
