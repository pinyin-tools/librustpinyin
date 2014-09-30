extern crate serialize;

use std::io::BufferedReader;
use std::io::File;
use self::serialize::json;
use std::collections::HashMap;

use std::hash::sip::SipHasher;

pub type PinyinDB = HashMap<String, Vec<String>, SipHasher>;

pub fn create_db(fname: &str) -> PinyinDB {

    let hasher = SipHasher::new();
    let mut db = HashMap::with_hasher(hasher);

    let path = Path::new(fname);
    let mut file = BufferedReader::new(File::open(&path));

    for line_iter in file.lines() {
        let word : HashMap<String, Vec<Vec<String>>> = match line_iter {
            Ok(x) => match json::decode(x.as_slice()) {
                Ok(x) => x,
                Err(e) => fail!(e)

            },
            Err(e) => fail!(e)
        };

        for (sinogram, pinyins) in word.iter() {
            let mut full_pinyin = String::new();
            let mut min_pinyin = String::new();
            for pinyin in pinyins.iter() {
                full_pinyin.push_str(pinyin.concat().as_slice());

                min_pinyin.push_str(pinyin[0].as_slice());
                min_pinyin.push_str(pinyin[2].as_slice());
            }
            db.insert_or_update_with(
                full_pinyin,
                vec![sinogram.clone()],
                |_key, value| value.push(sinogram.clone())
            );

            db.insert_or_update_with(
                min_pinyin,
                vec![sinogram.clone()],
                |_key, value| value.push(sinogram.clone())
            );

        }
    }
    return db;


}
