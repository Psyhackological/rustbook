pub fn convert_to_pig_latin(text: &str) -> String {
    // Split the input text by whitespace to obtain individual words
    text.split_whitespace()
        .map(|word| {
            // Extract the first character of each word
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                // Check if the first character is a vowel
                if is_vowel(first_char) {
                    // If the word starts with a vowel, add "-hay" to the end of the word
                    let mut word_with_hay = word.to_string();
                    word_with_hay.push_str("-hay");
                    word_with_hay
                } else {
                    // If the word starts with a consonant, move the first character to the end and add "ay"
                    let mut rest_with_ay = chars.as_str().to_string();
                    rest_with_ay.push_str(&format!("-{}ay", first_char));
                    rest_with_ay
                }
            } else {
                // If the word is empty, return an empty string
                String::new()
            }
        })
        // Collect the transformed words into a vector of strings
        .collect::<Vec<String>>()
        // Join the words in the vector with spaces to create the final Pig Latin string
        .join(" ")
}

// Only checks for the basic English vowels: 'a', 'e', 'i', 'o', and 'u'
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowel_apple() {
        assert_eq!(convert_to_pig_latin("apple"), "apple-hay");
    }

    #[test]
    fn consonant_first() {
        assert_eq!(convert_to_pig_latin("first"), "irst-fay");
    }
}
