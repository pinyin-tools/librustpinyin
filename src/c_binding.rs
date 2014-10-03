extern crate native;
extern crate libc;

use pinyin::db::create_db_from_csv;
use pinyin::db::PinyinDB;

use std::c_str::CString;
use std::string::raw::from_buf;
use pinyin2suggestion;
use pinyin::parser::string_to_tokens_as_strings;

#[no_mangle]
pub extern fn db_new(fname: *const libc::c_char) -> Box<PinyinDB> {
    let string = unsafe { from_buf(fname as *const u8) };

    box create_db_from_csv(string.as_slice())
}

#[no_mangle]
pub extern fn db_free(db: Box<PinyinDB>) {
    let _ = db;
}

///
///
///
#[no_mangle]
pub extern fn pinyin2suggestions_c (
    db: &mut PinyinDB,
    pinyin_raw_string: *const libc::c_char
) -> Box<Vec<String>> {

    let pinyin = unsafe {
        from_buf(pinyin_raw_string as *const u8)
    };

    box pinyin2suggestion(
        db,
        pinyin.as_slice()
    )
}

/// C compatible version of string_to_tokens_as_strings
///
///
#[no_mangle]
pub extern fn string_to_tokens_as_strings_c(
    pinyin_raw_string: *const libc::c_char
) -> Box<Vec<String>> {

    let pinyin = unsafe {
        from_buf(pinyin_raw_string as *const u8)
    };

    box string_to_tokens_as_strings(pinyin.as_slice())
}

///
///
///
#[no_mangle]
pub extern fn vec_string_free (
    suggestions: Box<Vec<String>>
) {
    let _ = suggestions;
}

///
///
///
#[no_mangle]
pub extern fn vec_string_size (suggestions: &mut Vec<String>) -> u32  {
    return suggestions.len() as u32;
}

///
///
///
#[no_mangle]
pub extern fn vec_string_value_get(
    suggestions: &mut Vec<String>,
    index: u32
) -> *const libc::c_char {
  
    // (*suggestions) should not be necessary, there's a bug in current
    // rust (as of end of september 2014) which force us to do that
    unsafe { (*suggestions)[index as uint].to_c_str().unwrap() }
}

///
///
///
#[no_mangle]
pub extern fn vec_string_value_free(buffer: *const i8) {
    unsafe { CString::new(buffer, true); }
}


