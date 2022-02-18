use crate::{Normalisation, Preprocessor};
use rust_stemmers::{Algorithm, Stemmer};

impl Preprocessor {
    pub fn normalise(normalisation: Normalisation, tokens: Vec<String>) -> Vec<String> {
        match normalisation {
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
    use crate::{Normalisation, Preprocessor};

    #[test]
    fn stem() {
        let normalisation = Normalisation::Stemming;
        let result = Preprocessor::normalise(
            normalisation,
            vec!["Throwing".to_string(), "a".to_string(), "ball.".to_string()],
        );
        assert_eq!(
            result,
            vec!["Throw".to_string(), "a".to_string(), "ball.".to_string()]
        );
    }
}
