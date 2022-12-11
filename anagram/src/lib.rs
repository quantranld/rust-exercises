use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sort_unstable(&word_lower);

    possible_anagrams
        .iter()
        .filter(|w| {
            let w_lower = w.to_lowercase();

            w_lower.len() == word_lower.len()
                && w_lower != word_lower
                && sort_unstable(&w_lower) == word_sorted
        })
        .copied()
        .collect()
}

fn sort_unstable(word: &str) -> Vec<char> {
    let mut word_array: Vec<char> = word.chars().collect();

    word_array.sort_unstable();

    word_array
}
