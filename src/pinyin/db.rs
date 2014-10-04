extern crate serialize;

use std::io::BufferedReader;
use self::serialize::json;
use std::collections::{HashMap, hashmap};
use std::collections::hashmap::{Occupied, Vacant};
use std::collections::TreeMap;

use std::hash::sip::SipHasher;
use pinyin::myfile::open_read_only;

use pinyin::dbentry::DBEntry;

pub type PinyinDB = HashMap<String, Vec<DBEntry>, SipHasher>;

/// create the database from a csv file, use this one if you dont
/// want to depend on the runtime being present
///
pub fn create_db_from_csv(fname: &str) -> PinyinDB {
    let hasher = SipHasher::new();
    let mut db: PinyinDB = HashMap::with_hasher(hasher);

    let path = Path::new(fname);
    let mut file = BufferedReader::new(open_read_only(&path));

    for line_iter in file.lines() {
        let line = match line_iter {
            Ok(line) => line,
            Err(e) => fail!(e)
        };

        let mut iter = line.as_slice().split(',');
        let sinogram =  iter.next().unwrap();
        let pinyin = iter.next().unwrap();
        let frequency = from_str(iter.next().unwrap()).unwrap_or(0u);

        let entry = DBEntry::new(
            sinogram.to_string(),
            frequency
        );

        match db.entry(pinyin.to_string()) {
            hashmap::Occupied(o) => o.into_mut().push(entry),
            hashmap::Vacant(v) => { v.set(vec![entry]); }
        }
    }

    return db;
}

pub fn create_db(fname: &str) -> PinyinDB {

    let hasher = SipHasher::new();
    let mut db: PinyinDB = HashMap::with_hasher(hasher);

    let path = Path::new(fname);
    let mut file = BufferedReader::new(open_read_only(&path));

    for line_iter in file.lines() {
        let word : TreeMap<String, Vec<Vec<String>>> = match line_iter {
            Ok(x) => {
                match json::decode(x.clone().as_slice()) {
                    Ok(x) => x ,
                    Err(e) => fail!(e)
                }
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

            let full_entry = DBEntry::new(
                sinogram.to_string(),
                0
            );

            let min_entry = DBEntry::new(
                sinogram.to_string(),
                0
            );

            match db.entry(full_pinyin.to_string()) {
                hashmap::Occupied(o) => o.into_mut().push(full_entry),
                hashmap::Vacant(v) => { v.set(vec![full_entry]); }
            }

            match db.entry(min_pinyin.to_string()) {
                hashmap::Occupied(o) => o.into_mut().push(min_entry),
                hashmap::Vacant(v) => { v.set(vec![min_entry]); }
            }
        }
    }
    return db;
}
