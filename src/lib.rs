#![feature(default_type_params)]
#![crate_name = "pinyinengine"]
#![crate_type = "dylib"]


pub use pinyin::db::create_db;
pub use pinyin::db::create_db_from_csv;
pub use pinyin::db::PinyinDB;
use pinyin::parser::string2tokens;

pub mod c_binding;
mod pinyin;

pub extern fn pinyin2suggestion(
    db: &PinyinDB,
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
            for entity in db[complete_pinyin].iter() {
                suggestions.push(entity.sinogram.clone());
            }
        }
    }

    // for the moment we reverse the vector in order to have
    // the longest suggestions first, we could have used "unshift"
    // to prepend, but unshift would have required to recopy the vector
    // everytime, which would have been highly unefficient (see rust doc)
    suggestions.reverse();
    return suggestions;

}
