use pinyin::token::PinyinToken;
use pinyin::check::is_second_initial_letter;
use pinyin::check::is_valid_final;
use pinyin::check::is_possible_no_initial;
use pinyin::check::is_possible_two_letter_initial;
use pinyin::check::is_one_letter_initial;
use pinyin::check::is_tone;

///
///
///
pub fn string2tokens (string: &str) -> Vec<PinyinToken> {

    let mut tokens : Vec<PinyinToken> = vec![];
    let mut iterator = string.chars();
    loop {

        let mut initial = String::new();
        let mut accumulator = String::new();

        match iterator.next() {
            Some(character)
            if is_possible_no_initial(&character) => {
                accumulator.push(character);
            }

            Some(character)
            if is_one_letter_initial(&character) => {
                initial.push(character);
            }

            Some(character)
            if is_possible_two_letter_initial(&character) => {

                initial.push(character);
                match iterator.next() {

                    Some(character)
                    if is_second_initial_letter(character) => {
                        initial.push(character);

                    }

                    Some(character)
                    if is_tone(&character) => {
                        tokens.push(PinyinToken {
                            initial: initial,
                            final: String::new(),
                            tone: character
                        });
                        continue;
                    }
                    Some(character) => accumulator.push(character),
                    _ => return tokens
                }
            }


            _ => return tokens
        }

        static MAX_FINAL_SIZE: uint = 4;
        
        for character in iterator {
            if is_tone(&character) && is_valid_final(accumulator.as_slice()) {
                tokens.push(PinyinToken {
                    initial: initial,
                    final: accumulator,
                    tone: character
                });
                break;
            } 
            if accumulator.len() >= MAX_FINAL_SIZE {
                return tokens;
            }
            accumulator.push(character);
        }

    }
}

/// Take a string parse it as much as it can into pinyin
/// and return a vector of the different consecutive pinyin token string
/// we've been able to parse
///
/// for example "n3hao3blablanzhong1bla" will return ["n3", "hao3"]
///
pub fn string_to_tokens_as_strings(string: &str) -> Vec<String> {
    let mut string_tokens : Vec<String> = vec![];

    for token in string2tokens(string.as_slice()).iter() {

        let pinyin = vec![
            token.initial.clone(),
            token.final.clone(),
            token.tone.to_string()
        ].concat();


        string_tokens.push(pinyin);
    }

    string_tokens
}
