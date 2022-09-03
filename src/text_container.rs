#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::vec::Vec;

use crate::WordDictionary;

pub(crate) struct WordBuffer {
    pub(crate) buffer: Vec<String>,
}

impl WordBuffer {
    pub(crate) fn from_dictionary(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut rng = rand::thread_rng();

        let buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .cloned()
            .collect::<Vec<String>>();

        WordBuffer { buffer }
    }

    pub(crate) fn from_keys(&mut self, amount: usize, keys: &Vec<char>) {
        let mut rng = rand::thread_rng();

        for _ in 0..amount {
            let mut new_word: String = "".to_owned();
            let max_length: u32 = rng.gen_range(3..=7);

            for _ in 1..max_length {
                let index = rng.gen_range(0..keys.len()) as usize;
                new_word.push(keys[index]);
            }
            self.buffer.push(new_word.to_string());
        }
    }
}
