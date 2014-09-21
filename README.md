# librustpinyin

Implementation in rust of library meant to be used in IME (input method engine)
and related software

the library will be a standard .so/dll that can be embedded in other language
as you would do for any normal C library

the main goal compared to other IME library/software is to provide a way to 
type pinyin with the tone explicitly stated, in order to force learner to
know the tones in addition to the pinyin itself

Once the library will working and tested, the next goal will be to create:

  * IBUS engine for Linux
  * IME engine for android 

of course iphone/Mac OS/Windows implementation would be welcome if somebody
feels like doing it.


Contribution are more than welcome, dont hesitate to reach me by email
or to add new issues in the tracker

## building it

To build it, you simply need to have a recent rust compiler and cargo

   cargo build 

and you're done


## testing it

For the moment the project compile into a test binary that read from
stdin a pinyin string, will parse it and lookup possible match into its
internal databases 

the "final" part of the pinyin can be ommitted i.e you can type 法国人 either with

   * fa3guo2ren2
   * f3g2r2

(for the moment we dont implement mix of partial and full pinyin)

for example: `echo -n  "f3g2r2" | ./read` will print 

    f3g2r2 
    initial f, final , tone: 3
    法
    䂲
    乶
    仿
    // more character with pinyin f.. something and third tone
    佱
    俯
    俯
    仿
    郙
    釜
    釡
    鍅
    俯
    髣
    鬴
    魬
    鯆
    黼
    initial g, final , tone: 2
    法国
    返国
    initial r, final , tone: 2
    法国人


## License 

The code is licensed under MIT license 


## data

The data currently come from a manual modification of the CC-CEDICT dump
(basically converting it into a pseudo-json file, removing unecessary data
and pre-parsing the pinyin), hence the data file is under the same license
as CC-CEDICT



##Interfaces (random notes)

just a bunch of notes for me latter

### pinyin2suggestion

    pinyin2suggestion(
        search : String,
        page: Int  default 0,
        pageSize: Int default 10
    ) -> List<Words>

Return a list of chinese words matching the search string,
the function support pagination

### word\_chosen

    word_chosen(word : Word) -> List<Word>

update the frequency of the given word


### add\_word

### pinyin2token

    pinyin2token(
        pinyin: String
    ) -> List<Token>

Segment a pinyin string into a list of tokens


### token2suggestion

    pinyin2suggestion(
        search : String,
        page: Int  default 0,
        pageSize: Int default 10
    ) -> List<Words>

### one\_token2suggestion

### two\_token2suggestion

### three\_token2suggestion

### four\_token2suggestion

### five\_more\_token2suggestion




