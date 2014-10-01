extern crate pinyinengine;

use std::io;

use pinyinengine::pinyin2suggestion;
use pinyinengine::create_db_from_csv;
fn main() {

    let db = create_db_from_csv("../data/filtered_db.csv");

    for line in io::stdin().lines() {
        let string = line.unwrap();
        for suggestion in pinyin2suggestion(&db, string.as_slice()).iter() {
            println!("{} ", suggestion);
        }
    }
}

