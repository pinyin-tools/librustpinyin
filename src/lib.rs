#![crate_name = "pinyinengine"]
#![crate_type = "dylib"]

pub use pinyin::db::create_db;
use pinyin::parser::string2tokens;

use std::collections::HashMap;

mod pinyin;



///
///
///
//#[no_mangle]
//pub extern fn pinyin2suggestion_c (
//    pinyin_raw_string : &str
//) -> Vec<&[u8]> {
//
//}

pub extern fn pinyin2suggestion(
    db: &HashMap<String, Vec<String>>,
    pinyin_raw_string : &str
) -> Vec<String> {
    let mut suggestions : Vec<String> = vec![];

    let mut complete_pinyin = String::new();
    for token in string2tokens(pinyin_raw_string.as_slice()).iter() {

        let full_pinyin = vec![
            token.initial.clone(),
            token.final.clone(),
            token.tone.to_string()
        ].concat();

        complete_pinyin.push_str(full_pinyin.as_slice());

        if db.contains_key(&complete_pinyin) {
            suggestions = vec![];
            for sinogram in db[complete_pinyin].iter() {
                suggestions.push(sinogram.clone());
            }
        } else {
            break;
        }
    }
    return suggestions;

}
