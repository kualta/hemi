#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::vec::Vec;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

pub(crate) struct WordDictionary<'a> {
    buffer: Vec<&'a str>,
}

/// WASM doesn't support std::fs yet ¯\_(ツ)_/¯
/// RFC issue: https://github.com/rust-lang/rust/issues/41619
// impl WordDictionary {
//     pub(crate) fn new(path: &str) -> Self {
//         let mut file_string = String::new();
//         File::open(path)
//             .unwrap_or_else(|_| panic!("Couldn't open path {}", path))
//             .read_to_string(&mut file_string)
//             .expect("Couldn't read file contents");
//         let buffer = serde_json::from_str(&file_string).expect("Couldn't parse the dictionary");
//         WordDictionary { buffer }
//     }
// }

/// Initialize dictionary with words
///
/// It's a temporary workaround until [RFC](https://github.com/rust-lang/rfcs/pull/1868) is implemented
pub(crate) fn init_left_dictionary() -> WordDictionary<'static> {
    WordDictionary {
        buffer: vec!["QWERT", "ASDFG", "ZXCVB", "FWWET"],
    }
}

/// Initialize dictionary with words
///
/// It's a temporary workaround until [RFC](https://github.com/rust-lang/rfcs/pull/1868) is implemented
pub(crate) fn init_right_dictionary() -> WordDictionary<'static> {
    WordDictionary {
        buffer: vec!["YUIOP", "HJKL:", "BNM<>", ",./'"],
    }
}

pub(crate) struct WordBuffer {
    pub(crate) buffer: Vec<String>,
}

impl WordBuffer {
    /// Copies `amount` of elements from provided `dictionary` and constructs [WordBuffer] from them
    pub(crate) fn from_dictionary(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut rng = rand::thread_rng();

        let buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        WordBuffer { buffer }
    }

    // pub(crate) fn from_keys(&mut self, amount: usize, keys: &Vec<char>) {
    //     let mut rng = rand::thread_rng();

    //     for _ in 0..amount {
    //         let mut new_word = "";
    //         let max_length: u32 = rng.gen_range(3..=7);

    //         for _ in 1..max_length {
    //             let index = rng.gen_range(0..keys.len()) as usize;
    //             new_word.push(keys[index]);
    //         }
    //         self.buffer.push(new_word);
    //     }
    // }
}
