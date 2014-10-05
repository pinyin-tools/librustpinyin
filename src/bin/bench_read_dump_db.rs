extern crate pinyinengine;
extern crate time;

use pinyinengine::create_db_from_csv;
use pinyinengine::dump_db_to_file;
fn main() {

    let before_create = time::precise_time_ns();
    let db = create_db_from_csv("../data/filtered_db.csv");
    let after_create = time::precise_time_ns();
    dump_db_to_file(&db, "/tmp/dump.csv");
    let after_dump = time::precise_time_ns();

    println!("creation: {} us", (after_create - before_create)/1000);
    println!("dump: {} us", (after_dump - after_create)/1000);

}


