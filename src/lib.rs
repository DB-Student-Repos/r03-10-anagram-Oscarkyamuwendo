use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Helper function to sort the characters of a word
    fn sorted_chars(word: &str) -> Vec<char> {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        chars
    }

    // Get the sorted characters of the input word
    let sorted_word = sorted_chars(&word.to_lowercase());
    let word_lower = word.to_lowercase();

    // Iterate over possible anagrams and collect the ones that match
    possible_anagrams.iter().filter_map(|&candidate| {
        if candidate.to_lowercase() != word_lower && sorted_chars(&candidate.to_lowercase()) == sorted_word {
            Some(candidate)
        } else {
            None
        }
    }).collect()
}
