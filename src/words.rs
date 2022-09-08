#![allow(dead_code)]

use dioxus::prelude::dioxus_elements::input;
use rand::seq::SliceRandom;
use rand::{distributions, Rng};
use std::vec::Vec;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

pub(crate) struct WordDictionary<'a> {
    buffer: Vec<&'a str>,
    keys: String,
}
impl<'a> WordDictionary<'a> {
    pub(crate) fn keys(&self) -> &str {
        self.keys.as_ref()
    }
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
        keys: LEFT_QWERTY_KEYS.to_owned(),
    }
}

/// Initialize dictionary with words
///
/// It's a temporary workaround until [RFC](https://github.com/rust-lang/rfcs/pull/1868) is implemented
pub(crate) fn init_right_dictionary() -> WordDictionary<'static> {
    WordDictionary {
        buffer: vec!["YUIOP", "HJKL:", "BNM<>", ",./'"],
        keys: RIGHT_QWERTY_KEYS.to_owned(),
    }
}

#[derive(Default)]
pub(crate) struct WordBuffer {
    input: String,
    last_word: String,
    buffer: Vec<String>,
    keys: Vec<Vec<String>>,
}

impl WordBuffer {
    /// Copies `amount` of elements from provided `dictionary` and constructs [WordBuffer] from them,
    pub(crate) fn new(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut rng = rand::thread_rng();

        let buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        let mut keys = vec![];
        for row in dictionary.keys().split_whitespace() {
            let mut new_row = vec![];
            for char in row.chars() {
                if char == ' ' { break; }
                new_row.push(char.to_string());
            }
            keys.push(new_row);
        };

        WordBuffer {
            buffer,
            keys,
            ..Default::default()
        }
    }

    pub(crate) fn submit(&mut self) {
        self.last_word = self.input.clone();
        self.input.clear();
    }

    pub(crate) fn last_word(&self) -> &str {
        self.last_word.as_ref()
    }

    pub fn push_str(&mut self, string: &str) {
        self.input.push_str(string)
    }

    pub fn push(&mut self, ch: char) {
        self.input.push(ch)
    }

    pub(crate) fn input(&self) -> &str {
        self.input.as_ref()
    }

    pub fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub(crate) fn buffer(&self) -> &[String] {
        self.buffer.as_ref()
    }

    pub(crate) fn keys(&self) -> &Vec<Vec<String>> {
        self.keys.as_ref()
    }
}
