extern crate pinyinengine;

use std::io;

use pinyinengine::pinyin2suggestion;
use pinyinengine::create_db;
fn main() {

    let db = create_db("../data/filtered_db.json");

    for line in io::stdin().lines() {
        let string = line.unwrap();
        for suggestion in pinyin2suggestion(&db, string.as_slice()).iter() {
            println!("{} ", suggestion);
        }
    }
}

