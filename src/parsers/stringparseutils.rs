use parseutils::*;
use std::iter;


pub fn starts_with_token<'a,'b>(
    input: &'a str,
    tokens: &'b [&str]) -> bool {
        match consume_token_in_list(input, tokens){
            Ok(_)=> return true,
            Err(_) => return false
        }
    }


// contrary to functions in parseutils, this function 'eats' the start/stop token characters.
// To let the caller keep track of the current position in the input slice, 
// this function also returns an offset value: the exact number of characters that were consumed
// string content is returned as a slice of input, no modification at all is made to it
// if start or stop token is not found in input slice, err is returned
pub fn consume_between_tokens<'a,'b>(
    input: &'a str,
    tokens: &'b [&str])
    -> Result<(&'a str, &'a str, usize), &'a str>{
    let mut offset:usize = 0;

    // Check which token in the provided list is the one that we actually find
    let (remaining, start_token) = consume_token_in_list(input, tokens)?;
    offset += start_token.len();
    let offset_start=offset;
    let mut cursor_slice = remaining;

    {
        let (rem, consumed) = consume_until_token_escape(remaining, &[start_token]).unwrap();
        cursor_slice=rem;
        offset += consumed.len();
    }

    if let Ok((rem, consumed)) = consume_token_in_list(cursor_slice, &[start_token]){
        if (consumed==start_token){
            cursor_slice=rem;
            // don't include the final token in the returned slice, but include it in the consumed offset
            return Ok((cursor_slice, &input[offset_start..offset], offset+consumed.len()));
        }
    }

    return Err(input);
}




// Consumes the input slice until on of the tokens in tokens array is found
// if a token exists in the input in the escaped form (\token), 
// it is ignored and slice consumption continues 
// until a non-escaped token is found
pub fn consume_until_token_escape<'a, 'b>(
    input: &'a str,
    tokens: &'b [&str],
) -> Result<(&'a str, &'a str), &'a str> {
    // build list of all tokens to consider : the provided ones and the excaped ones
    let escaped_tokens:Vec<String> = tokens.iter().map(|t| format!("\\{}", t)).collect();
    let mut all_tokens: Vec<&str>= escaped_tokens.iter()
        .map(|s| s.as_str()).collect();
    all_tokens.extend(tokens);

    let mut cursor_slice = input;
    let mut offset:usize = 0;
    let mut slice_to_return=input;

    while cursor_slice.len() >0 {
        match consume_until_token_in_list(cursor_slice, &all_tokens){
            Ok((rem, consumed)) =>{
                cursor_slice=rem;
                offset += consumed.len();
            },
            Err(_)=> break, // not possible in fact
        }
        match consume_token_in_list(cursor_slice, &all_tokens){
            Ok((rem, consumed)) =>{

                // check if token is an escaped one
                for token in tokens {
                    if consumed == *token {
                        // match one of the provided tokens
                        return Ok((cursor_slice, &input[0..offset]));
                    }
                }

                // that was just an escaped token or the end of slice... continue normally
                cursor_slice=rem;
                offset += consumed.len();                
            }
            Err(_) => break
        }
    }
    return Ok((cursor_slice, &input[0..offset]));
}

// un-escape characters from a string slice
// this function returns a new string and the number of consumed characters from the input slice
// in case the last characters can't be unescaped, they will be returned as the remaining slice
pub fn unescape_to_string<'a>(input: & 'a str) -> (& 'a str, String) {
    let mut slice=input;
    let mut collector = String::new();

    while slice.len() > 0 {
        let stop_tokens = ["\\"];

        {
            let (new_slice, consumed) =
                consume_until_token_in_list(slice, &stop_tokens).unwrap();
            collector.push_str(consumed);
            slice = new_slice;
        }

        // if we actually stopped because of '\' char and not end of slice'
        if slice.len() > 0 {
            //remaining characters in current line
            let (new_slice, consumed) = consume_token_in_list(slice, &stop_tokens).unwrap();
            if consumed == "\\" {
                // next char has top be escaped
                let mut iter = new_slice.chars();

                if let Some(c) = iter.next() {
                    match c {
                        'r' => collector.push('\r'),
                        'n' => collector.push('\n'),
                        't' => collector.push('\t'),
                        _ => collector.push(c),
                        // todo : handle \u, \U
                    }
                    slice = &new_slice[c.len_utf8()..];
                } else {
                    // string is just '... \', maybe it's an attempt to define a multiline string ...
                    // just return what we actually processed successfully, the the rest as a slice
                    return (slice, collector);
                }
            }
        }
    }
    //normally slice is empty at this point
    return (slice, collector);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_consume_with_escape() {
        assert_eq!(consume_until_token_escape("toto\\\"\"", &["\""]), Ok(("\"", "toto\\\"")));
        assert_eq!(consume_until_token_escape("toto\\*tata*titi", &["\"", "*"]), Ok(("*titi", "toto\\*tata")));
        assert_eq!(consume_until_token_escape("", &["\"", "*"]), Ok(("", "")));
        assert_eq!(consume_until_token_escape("", &[]), Ok(("", "")));
        assert_eq!(consume_until_token_escape("toto\\*tata*titi", &["", ""]), Ok(("", "toto\\*tata*titi")));
        assert_eq!(consume_until_token_escape("toto\\*tata*titi", &[]), Ok(("", "toto\\*tata*titi")));
        assert_eq!(consume_until_token_escape("toto\\*", &["\"", "*"]), Ok(("", "toto\\*")));
        assert_eq!(consume_until_token_escape("toto*tata", &["\"", "*"]), Ok(("*tata", "toto")));
        // test utf-8
        assert_eq!(consume_until_token_escape("きゃりー\\*ぱみゅ*ぱみゅ", &["\"", "*"]), Ok(("*ぱみゅ", "きゃりー\\*ぱみゅ")));
    }



    #[test]
    fn test_consume_nominal_between_tokens() {
        assert_eq!(consume_between_tokens("\"toto\"", &["\""]), Ok(("", "toto", 6)));
        assert_eq!(consume_between_tokens("'toto'", &["'"]), Ok(("", "toto", 6)));
        assert_eq!(consume_between_tokens("'toto'", &["\"", "'"]), Ok(("", "toto", 6)));
        assert_eq!(consume_between_tokens("**toto**", &["**", "*"]), Ok(("", "toto", 8)));
        assert_eq!(consume_between_tokens("*to\\*to*", &["*"]), Ok(("", "to\\*to", 8)));
        assert_eq!(consume_between_tokens("**toto**", &["*"]), Ok(("toto**", "", 2)));
        //utf-8
        assert_eq!(consume_between_tokens("**きゃりー**ぱみゅぱみゅ", &["**"]), Ok(("ぱみゅぱみゅ", "きゃりー", "**きゃりー**".len())));
    }



    #[test]
    fn test_consume_pathologic_between_tokens() {
        assert_eq!(consume_between_tokens("\"toto\\\"", &["\""]), Err("\"toto\\\""));
        assert_eq!(consume_between_tokens("\"toto\"", &["'"]), Err("\"toto\""));
        assert_eq!(consume_between_tokens("\"toto", &["\""]), Err("\"toto"));
    }


    #[test]
    fn test_unescape_to_string() {
        assert_eq!(unescape_to_string("\"toto\""), ("", String::from("\"toto\"")));
        assert_eq!(unescape_to_string("\\\"toto\\\""), ("", String::from("\"toto\"")));
        assert_eq!(unescape_to_string("\\ttoto"), ("", String::from("\ttoto")));
        assert_eq!(unescape_to_string("tot\\o"), ("", String::from("toto")));
        assert_eq!(unescape_to_string("tot\\\\o"), ("", String::from("tot\\o")));        
    }


}
