extern crate serialize;

use std::io::BufferedReader;
use std::io::BufferedWriter;
use self::serialize::json;
use std::collections::{HashMap, hashmap};
use std::collections::TreeMap;

use std::hash::sip::SipHasher;
use pinyin::myfile::open_read_only;
use pinyin::myfile::open_write_only;

use pinyin::dbentry::DbEntry;

pub type PinyinDB = HashMap<String, Vec<DbEntry>, SipHasher>;

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
        let sinogram =  iter.next().unwrap().to_string();
        let pinyin = iter.next().unwrap().to_string();
        let frequency = from_str(iter.next().unwrap()).unwrap_or(0u);

        let entry = DbEntry::new(
            sinogram,
            frequency
        );

        match db.entry(pinyin) {
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

            let full_entry = DbEntry::new(
                sinogram.to_string(),
                0
            );

            let min_entry = DbEntry::new(
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

/// Update database for the word link with a pinyin string
/// either add it, or update the frequency
///
pub fn update_db_with_word(
    db: &mut PinyinDB,
    pinyin: &str,
    word: &DbEntry
) {
    // we check if db has already this pinyin
    match db.entry(pinyin.to_string()) {
        // if so...
        hashmap::Occupied(o) => {
            let mut new_word = true;
            // we check if main db has already this Chinese word
            // in which case we update the frequency by the user
            // one
            let mut_o = o.into_mut();
            for main_db_entry in mut_o.iter_mut() {
                if main_db_entry.sinogram == word.sinogram {
                    main_db_entry.frequency += word.frequency;
                    new_word = false;
                    break;
                }
            }
            // if we dont have this Chinese word, we simply add it
            if new_word {
                mut_o.push(word.clone())
            }
        }
        // else, if no pinyin, we add that pinyin with our user
        // word inside it
        hashmap::Vacant(v) => { v.set(vec![word.clone()]); }
    }
}


/// Dump the given in the file at the given address
/// dumped using the same CSV format as create_db_from_csv
///
pub fn dump_db_to_file(db: &PinyinDB, fname: &str) {

    let path = Path::new(fname);
    let mut file = BufferedWriter::new(open_write_only(&path));

    for (pinyin, entries) in db.iter() {
        for entry in entries.iter() {
            file.write_str(pinyin.as_slice());
            file.write_char(',');
            file.write_str(entry.sinogram.as_slice());
            file.write_char(',');
            file.write_uint(entry.frequency);
            file.write_char(',');
            file.write_char('\n');
        }
    }
}
