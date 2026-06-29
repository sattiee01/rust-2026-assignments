pub fn longest_word(sentence: &str) -> Option<&str> {
    let mut longest: Option<&str> = None;

    for word in sentence.split_whitespace() {
        match longest {
            None => longest = Some(word),
            Some(current) => {
                if word.len() > current.len() {
                    longest = Some(word);
                }
            }
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest_of_four() {
        assert_eq!(longest_word("the quick brown fox"), Some("quick"));
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(longest_word("   "), None);
    }

    #[test]
    fn empty_input() {
        assert_eq!(longest_word(""), None);
    }

    #[test]
    fn ascending_lengths() {
        assert_eq!(longest_word("a bb ccc dd"), Some("ccc"));
    }

    #[test]
    fn single_word() {
        assert_eq!(longest_word("hello"), Some("hello"));
    }

    #[test]
    fn single_letter() {
        assert_eq!(longest_word("a"), Some("a"));
    }

    #[test]
    fn first_on_tie() {
        assert_eq!(longest_word("abc xyz def"), Some("abc"));
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        assert_eq!(longest_word("  rust ferris  "), Some("ferris"));
    }
}