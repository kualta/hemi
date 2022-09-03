#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::vec::Vec;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

pub(crate) struct WordBuffer {
    pub(crate) keys: Vec<char>,
    pub(crate) buffer: Vec<String>,
}

impl WordBuffer {
    fn from_dictionary(&mut self, amount: usize, dictionary: &Vec<String>) {
        let mut rng = rand::thread_rng();

        self.buffer = dictionary
            .choose_multiple(&mut rng, amount)
            .cloned()
            .collect::<Vec<String>>();
    }

    fn from_keys(&mut self, amount: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..amount {
            let mut new_word: String = "".to_owned();
            let max_length: u32 = rng.gen_range(3..=7);

            for _ in 1..max_length {
                let index = rng.gen_range(0..self.keys.len()) as usize;
                new_word.push(self.keys[index]);
            }
            self.buffer.push(new_word.to_string());
        }
    }
}
