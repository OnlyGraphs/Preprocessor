use crate::{Preprocessor, TokenisationOptions};

impl Preprocessor {
    pub fn tokenise(&self, raw_text: String) -> Vec<String> {
        let TokenisationOptions {
            remove_numbers,
            // remove_coordinates,
            // remove_dates,
            remove_special_characters,
            remove_punctuation,
            ..
        } = self.processing_options.tokenisation_options;

        // let re = regex::Regex::new("[^a-zA-Z]+").expect("Invalid Regex");
        let re = regex::Regex::new("\\s+").expect("Invalid Regex");
        let tokens = re
            .split(raw_text.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // Remove all of the numbers from each of the strings
        let tokens = remove_numbers
            .then(|| {
                tokens
                    .iter()
                    .map(|t| t.chars().filter(|c| !c.is_digit(10)).collect())
                    .collect()
            })
            .unwrap_or(tokens);

        // Remove all of the coordinates from each of the strings
        // let tokens = remove_coordinates
        //     .then(|| {
        //         tokens
        //             .iter()
        //             .map(|t| t.chars().filter(|c| !c.is_digit(10)).collect())
        //             .collect()
        //     })
        //     .unwrap_or(tokens);

        // Remove all of the dates from each of the strings
        // let tokens = remove_dates
        //     .then(|| {
        //         tokens
        //             .iter()
        //             .map(|t| t.chars().filter(|c| !c.is_digit(10)).collect())
        //             .collect()
        //     })
        //     .unwrap_or(tokens);

        // Remove all of the special characters from each of the strings
        let tokens = remove_special_characters
            .then(|| {
                tokens
                    .iter()
                    .map(|t| t.chars().filter(|c| c.is_ascii()).collect())
                    .collect()
            })
            .unwrap_or(tokens);

        // Remove all of the special characters from each of the strings
        let tokens = remove_punctuation
            .then(|| {
                tokens
                    .iter()
                    .map(|t| t.chars().filter(|c| !c.is_ascii_punctuation()).collect())
                    .collect()
            })
            .unwrap_or(tokens);

        // Remove all of the zero length tokens
        let tokens = tokens
            .into_iter()
            .filter(|s| s.len() > 0)
            .collect::<Vec<String>>();

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::Preprocessor;

    #[test]
    fn tokenise_split_white_space() {
        let preprocessor: Preprocessor = Default::default();
        let result = preprocessor.tokenise("Hello there".to_string());
        assert_eq!(result, vec!["Hello".to_string(), "there".to_string()]);
    }
    #[test]
    fn tokenise_keep_punctuation() {
        let preprocessor: Preprocessor = Default::default();
        let result = preprocessor.tokenise("An example, sentence.".to_string());
        assert_eq!(
            result,
            vec![
                "An".to_string(),
                "example,".to_string(),
                "sentence.".to_string()
            ]
        );
    }
    //     #[test]
    //     fn tokenise_split_white_space() {
    //         let result = Preprocessor::tokenise("Hello there".to_string());
    //         assert_eq!(result, vec!["Hello".to_string(), "there".to_string()]);
    //     }
    //     #[test]
    //     fn tokenise_split_white_space() {
    //         let result = Preprocessor::tokenise("Hello there".to_string());
    //         assert_eq!(result, vec!["Hello".to_string(), "there".to_string()]);
    //     }
    //     #[test]
    //     fn tokenise_split_white_space() {
    //         let result = Preprocessor::tokenise("Hello there".to_string());
    //         assert_eq!(result, vec!["Hello".to_string(), "there".to_string()]);
    //     }
}
