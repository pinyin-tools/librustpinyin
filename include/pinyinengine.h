#ifndef RUST_PINYIN_ENGINE
#define RUST_PINYIN_ENGINE

#include <stdint.h>

// create pinyin->sinograms database
void *db_new (const char* fname);
void db_free(void* db);

// dump the given database into a csv format (same accepted by db_new)
void db_dump(void* db, const char* fname);

void db_update_with_word(
    void* db,
    const char* pinyin,
    const char* sinograms
);

// create a vector of suggestions based on a search string
void* pinyin2suggestions_c(
    void* db,
    const char* pinyinRawString
);

// split a search string into an array of pinyin tokens
void* string_to_tokens_as_strings_c(
    const char* pinyinRawString
);

void vec_string_free(const char* pinyinRawString);

// get the number of elements in vector
uint32_t vec_string_size(void* suggestions);
// get the value at number #index
const char* vec_string_value_get(void *suggestions, uint32_t index);
void vec_string_value_free(const char* value);

#endif
