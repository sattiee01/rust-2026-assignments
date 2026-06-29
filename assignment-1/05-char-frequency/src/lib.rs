use std::collections::HashMap;

pub fn char_frequency(input: &str) -> Vec<(char, u32)> {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for ch in input.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    let mut result: Vec<(char, u32)> = freq.into_iter().collect();

    result.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mississippi() {
        assert_eq!(
            char_frequency("mississippi"),
            vec![('i', 4), ('s', 4), ('p', 2), ('m', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(char_frequency(""), vec![]);
    }

    #[test]
    fn all_tied() {
        assert_eq!(
            char_frequency("abcabc"),
            vec![('a', 2), ('b', 2), ('c', 2)]
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(char_frequency("z"), vec![('z', 1)]);
    }

    #[test]
    fn unique_chars_sorted_alphabetically() {
        assert_eq!(
            char_frequency("cba"),
            vec![('a', 1), ('b', 1), ('c', 1)]
        );
    }

    #[test]
    fn one_dominant_char() {
        assert_eq!(
            char_frequency("aaaaab"),
            vec![('a', 5), ('b', 1)]
        );
    }

    #[test]
    fn spaces_count_too() {
        assert_eq!(
            char_frequency("a a"),
            vec![('a', 2), (' ', 1)]
        );
    }

    #[test]
    fn six_unique_letters_sorted() {
        assert_eq!(
            char_frequency("fedcba"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1)]
        );
    }
}