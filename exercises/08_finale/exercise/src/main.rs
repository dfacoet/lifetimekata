use std::cmp::{max, min};

use require_lifetimes::require_lifetimes;

#[derive(Debug, PartialEq, Eq)]
enum MatcherToken<'a> {
    /// This is just text without anything special.
    RawText(&'a str),
    /// This is when text could be any one of multiple
    /// strings. It looks like `(one|two|three)`, where
    /// `one`, `two` or `three` are the allowed strings.
    OneOfText(Vec<&'a str>),
    /// This is when you're happy to accept any single character.
    /// It looks like `.`
    WildCard,
}

#[derive(Debug, PartialEq, Eq)]
struct Matcher<'a> {
    /// This is the actual text of the matcher
    text: &'a str,
    /// This is a vector of the tokens inside the expression.
    tokens: Vec<MatcherToken<'a>>,
    /// This keeps track of the most tokens that this matcher has matched.
    most_tokens_matched: usize,
}

impl<'a> Matcher<'a> {
    /// This should take a string reference, and return
    /// an `Matcher` which has parsed that reference.
    #[require_lifetimes]
    fn new(text: &'a str) -> Option<Matcher<'a>> {
        let mut tokens = vec![];
        // TODO: I think this means that () cannot be nested - so it's simpler
        // let mut to_parse = vec![text];
        // while let Some(next) = to_parse.pop() {
        //     match next.chars().next() {
        //         Some('.') => {
        //             tokens.push(MatcherToken::WildCard);
        //             to_parse.push(&next[1..])
        //         }
        //         Some('(') => {
        //             let last_closed = next.rfind(')')?;

        //         }
        //         _ => (),
        //     }
        // }

        let mut text_left = text;
        while let Some(i) = text_left.chars().next() {
            let next_idx = match i {
                '.' => {
                    tokens.push(MatcherToken::WildCard);
                    1
                }
                '(' => {
                    let right = text_left.find(')')?;
                    tokens.push(MatcherToken::OneOfText(
                        text_left[1..right].split('|').collect(),
                    ));
                    right + 1
                }
                _ => {
                    let first_dot = text_left.find('.').unwrap_or(text_left.len());
                    let first_open = text_left.find('(').unwrap_or(text_left.len());
                    let right = min(first_dot, first_open);
                    tokens.push(MatcherToken::RawText(&text_left[0..right]));
                    right
                }
            };
            text_left = &text_left[next_idx..];
        }

        Some(Matcher {
            text,
            tokens,
            most_tokens_matched: 0,
        })
    }

    /// This should take a string, and return a vector of tokens, and the corresponding part
    /// of the given string. For examples, see the test cases below.
    #[require_lifetimes]
    fn match_string<'b>(&'b mut self, string: &'b str) -> Vec<(&'b MatcherToken<'a>, &'b str)> {
        let mut string_left = string;
        let matches: Vec<_> = self
            .tokens
            .iter()
            .map_while(|t| {
                if string_left.is_empty() {
                    None
                } else {
                    match t {
                        MatcherToken::RawText(raw_text) => {
                            if string_left.starts_with(raw_text) {
                                let s = &string_left[..raw_text.len()];
                                string_left = &string_left[raw_text.len()..];
                                Some((t, s))
                            } else {
                                None
                            }
                        }
                        MatcherToken::OneOfText(options) => {
                            let matching = options
                                .iter()
                                .find(|option| string_left.starts_with(*option))?;
                            let s = &string_left[..matching.len()];
                            string_left = &string_left[matching.len()..];
                            Some((t, s))
                        }
                        MatcherToken::WildCard => {
                            let c = &string_left[..1];
                            string_left = &string_left[1..];
                            Some((t, c))
                        }
                    }
                }
            })
            .collect();

        self.most_tokens_matched = max(self.most_tokens_matched, matches.len());
        matches
    }
}

fn main() {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::{Matcher, MatcherToken};
    #[test]
    fn simple_test() {
        let match_string = "abc(d|e|f).".to_string();
        let mut matcher = Matcher::new(&match_string).unwrap();

        assert_eq!(matcher.most_tokens_matched, 0);

        {
            let candidate1 = "abcge".to_string();
            let result = matcher.match_string(&candidate1);
            assert_eq!(result, vec![(&MatcherToken::RawText("abc"), "abc"),]);
            assert_eq!(matcher.most_tokens_matched, 1);
        }

        {
            // Change 'e' to '💪' if you want to test unicode.
            let candidate1 = "abcde".to_string();
            let result = matcher.match_string(&candidate1);
            assert_eq!(
                result,
                vec![
                    (&MatcherToken::RawText("abc"), "abc"),
                    (&MatcherToken::OneOfText(vec!["d", "e", "f"]), "d"),
                    (&MatcherToken::WildCard, "e") // or '💪'
                ]
            );
            assert_eq!(matcher.most_tokens_matched, 3);
        }
    }

    #[test]
    fn broken_matcher() {
        let match_string = "abc(d|e|f.".to_string();
        let matcher = Matcher::new(&match_string);
        assert_eq!(matcher, None);
    }
}
