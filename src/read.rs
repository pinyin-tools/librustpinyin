use std::io;

use pinyin::db::create_db;
use pinyin::parser::string2tokens;

mod pinyin;


fn main() {

    let db = create_db("../data/filtered_db.json");

    for line in io::stdin().lines() {
        let string = line.unwrap();
        println!("{} ", string);
        let mut complete_pinyin = String::new();
        for token in string2tokens(string.as_slice()).iter() {
            println!(
                "initial {}, final {}, tone: {}",
                token.initial,
                token.final,
                token.tone
            );
            let full_pinyin = vec![
                token.initial.clone(),
                token.final.clone(),
                token.tone.to_string()
            ].concat();

            complete_pinyin.push_str(full_pinyin.as_slice());

            if db.contains_key(&complete_pinyin) {

                for sinogram in db[complete_pinyin].iter() {
                    println!("{}", sinogram);
                }
            }
        }
    }
}

