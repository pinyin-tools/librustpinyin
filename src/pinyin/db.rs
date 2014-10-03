extern crate serialize;

use std::io::BufferedReader;
use self::serialize::json;
use std::collections::HashMap;
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
    let mut db = HashMap::with_hasher(hasher);

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
        let frequency = iter.next().unwrap();

        db.insert_or_update_with(
            pinyin.to_string(),
            vec![DBEntry{
                    sinogram: sinogram.to_string(),
                    frequency: from_str(frequency).unwrap_or(0u)
                }
            ],
            |_key, value| value.push(DBEntry{
                    sinogram: sinogram.to_string(),
                    frequency: from_str(frequency).unwrap_or(0u)
                }
            )
        );
    }

    return db;
}

pub fn create_db(fname: &str) -> PinyinDB {

    let hasher = SipHasher::new();
    let mut db = HashMap::with_hasher(hasher);

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
            db.insert_or_update_with(
                full_pinyin,
                vec![DBEntry{
                        sinogram: sinogram.clone(),
                        frequency: 0
                    }
                ],
                |_key, value| value.push(DBEntry{
                        sinogram: sinogram.clone(),
                        frequency: 0
                    }
                )
            );

            db.insert_or_update_with(
                min_pinyin,
                vec![DBEntry{
                        sinogram: sinogram.clone(),
                        frequency: 0
                    }
                ],
                |_key, value| value.push(DBEntry{
                        sinogram: sinogram.clone(),
                        frequency: 0
                    }
                )
            );
        }
    }
    return db;
}
