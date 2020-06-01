// Collection of functions to 'consume' a part of string slice
// based on some search/match criterion
//
// All those functions process an input str with specific objective and return either:
// - if we found what we expected in the str : OK with a tuple of :
//     .the remaining (not consumed) string slice
//     .the consumed string slice
// - if we didn't find what we expected : Err with the input str forwarded
//
// All these functions retuirn the same kind of tuples even in usecases where it is not
// completely relevant so that they can be used generically and combined

// Returns Ok((remaining str slice, consumed str slice))
// with the FIRST token that matches
// Err if no token match
pub fn consume_token_in_list<'a, 'b>(
    input: &'a str,
    tokens: &'b [&str],
) -> Result<(&'a str, &'a str), &'a str> {
    for token in tokens {
        if token.len() > 0 && input.starts_with(token) {
            return Ok((&input[token.len()..], &input[..token.len()]));
        }
    }
    // No token matched
    return Err(input);
}

pub fn consume_until_token_in_list<'a, 'b>(
    input: &'a str,
    tokens: &'b [&str],
) -> Result<(&'a str, &'a str), &'a str> {
    let mut iter = input.char_indices();

    while let Some((i, c)) = iter.next() {
        for token in tokens {
            if token.len() > 0 && input[i..].starts_with(token) {
                return Ok((&input[i..], &input[..i]));
            }
        }
    }
    // No token was encountered, consume full input
    return Ok((&input[input.len()..], &input));
}

// Returns Ok((remaining str slice, consumed str slice))
// No case where Err is returned
// Whitespace is included in remaining slice
pub fn consume_until_whitespace(input: &str) -> Result<(&str, &str), &str> {
    let mut iter = input.char_indices();

    while let Some((i, c)) = iter.next() {
        if c.is_whitespace() {
            return Ok((&input[i..], &input[..i]));
        }
    }
    // No whitespace was encountered, consume full input
    return Ok((&input[input.len()..], &input));
}

// Returns Ok((remaining str slice, consumed str slice))
// No case where Err is returned
pub fn consume_whitespaces(input: &str) -> (&str, &str) {
    let mut iter = input.char_indices();

    while let Some((i, c)) = iter.next() {
        if !c.is_whitespace() {
            return (&input[i..], &input[..i]);
        }
    }
    // No whitespace was encountered, consume full input
    return (&input[input.len()..], &input);
}

// Returns Ok((remaining str slice, consumed str slice))
// No case where Err is returned
pub fn consume_while_char(input: &str, cond: char) -> (&str, &str) {
    let mut iter = input.char_indices();

    while let Some((i, c)) = iter.next() {
        if c != cond {
            return (&input[i..], &input[..i]);
        }
    }
    // No whitespace was encountered, consume full input
    return (&input[input.len()..], &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_token_in_list() {
        assert_eq!(consume_token_in_list("toto", &["to"]), Ok(("to", "to")));
        assert_eq!(
            consume_token_in_list("toto", &["ta", "to"]),
            Ok(("to", "to"))
        );
        assert_eq!(consume_token_in_list("tata", &["to"]), Err("tata"));
        assert_eq!(
            consume_token_in_list("きゃりーぱみゅぱみゅ", &["ぱみゅ", "きゃりー"]),
            Ok(("ぱみゅぱみゅ", "きゃりー"))
        );
        // test empty input
        assert_eq!(consume_token_in_list("", &["token", "cartman"]), Err(""));
        // test empty token
        assert_eq!(
            consume_token_in_list("cartman", &["token", ""]),
            Err("cartman")
        );
        assert_eq!(consume_token_in_list("cartman", &[]), Err("cartman"));

        assert_eq!(
            consume_token_in_list("\" \"", &["\"", "\'", "\"\"\""]),
            Ok((" \"", "\""))
        );

        // This test is pretty important because we use this assumption to differentiate between """ and ", ** and *
        assert_eq!(
            consume_token_in_list("\"\"\"\"\"\"", &["\"\"\"", "\"", "\'"]),
            Ok(("\"\"\"", "\"\"\""))
        );
    }

    #[test]
    fn test_consume_until_token_in_list() {
        assert_eq!(
            consume_until_token_in_list("tototiti", &["ta", "ti"]),
            Ok(("titi", "toto"))
        );
    }

    #[test]
    fn test_consume_until_whitespace() {
        assert_eq!(
            consume_until_whitespace("nowhitespace"),
            Ok(("", "nowhitespace"))
        );
        assert_eq!(consume_until_whitespace("    "), Ok(("    ", "")));
        assert_eq!(
            consume_until_whitespace("Arthur Dent"),
            Ok((" Dent", "Arthur"))
        );
        assert_eq!(
            consume_until_whitespace("guitar\ttab"),
            Ok(("\ttab", "guitar"))
        );
        assert_eq!(
            consume_until_whitespace("XÆA-12 Musk"),
            Ok((" Musk", "XÆA-12"))
        );
        assert_eq!(
            consume_until_whitespace("きゃりー ぱみゅぱみゅ"),
            Ok((" ぱみゅぱみゅ", "きゃりー"))
        );
        assert_eq!(consume_until_whitespace(""), Ok(("", "")));
    }

    #[test]
    fn test_consume_whitespaces() {
        assert_eq!(consume_whitespaces(" toto"), ("toto", " "));
        assert_eq!(consume_whitespaces("\t\n toto"), ("toto", "\t\n "));
        assert_eq!(consume_whitespaces(""), ("", ""));
    }

    #[test]
    fn test_consume_while_char() {
        assert_eq!(
            consume_while_char("cccccccombo breaker", 'c'),
            ("ombo breaker", "ccccccc")
        );
        assert_eq!(
            consume_while_char("blah blah blah", 'c'),
            ("blah blah blah", "")
        );
    }
}
