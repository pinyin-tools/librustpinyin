///
///
///
pub fn is_one_letter_initial (letter: &char) -> bool {
    [
        'b',
        'p',
        'm',
        'f',
        'd',
        't',
        'n',
        'l',
        'g',
        'k',
        'h',
        'r',
        'j',
        'q',
        'x',
        'w',
        'y'
    ].contains(letter)
}

///
///
///
pub fn is_possible_two_letter_initial(letter: &char) -> bool {
    [
        'z',
        'c',
        's'
    ].contains(letter)
}

///
///
///
pub fn is_second_initial_letter(letter: char) -> bool {
    letter == 'h'
}


///
///
///
pub fn is_tone(letter: &char) -> bool {
    [
        '1',
        '2',
        '3',
        '4',
        '5',
    ].contains(letter)
}

///
///
///
pub fn is_possible_no_initial(letter: &char) -> bool {
    ['a','o','e'].contains(letter)
}

///
///
///
pub fn is_valid_final (final: &str) -> bool {
    [
        "",
        "a",
        "o",
        "e",
        "ai",
        "ei",
        "ao",
        "ou",
        "an",
        "en",
        "ang",
        "eng",
        "ong",
        "u",
        "ua",
        "uo",
        "uai",
        "ui",
        "uan",
        "uang",
        "un",
        "ueng",
        "i",
        "ia",
        "ie",
        "iao",
        "iu",
        "ian",
        "iang",
        "in",
        "ing",
        "iong",
        "v",
        "ve",
    ].contains(&final)

}

