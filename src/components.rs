use rand::seq::SliceRandom;
use rand::Rng;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::vec::Vec;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";
pub(crate) struct TextContainer {
    pub(crate) keys: String,
    pub(crate) input_buffer: String,
    pub(crate) words_buffer: Vec<String>,
    pub(crate) current_index: usize,
    pub(crate) dictionary: Option<Vec<String>>,
}

impl TextContainer {
    pub(crate) fn generate_words(&mut self, amount: usize) {
        match self.dictionary {
            Some(_) => self.generate_words_from_dictionary(amount),
            None => self.generate_words_from_keys(amount),
        }
    }

    fn generate_words_from_dictionary(&mut self, amount: usize) {
        let mut rng = rand::thread_rng();

        // FIXME: Might avoid cloning and store words in buffer as & to dictionary
        self.words_buffer = self
            .dictionary
            .as_ref()
            .expect("Dictionary is not loaded")
            .choose_multiple(&mut rng, amount)
            .cloned()
            .collect();
    }

    fn generate_words_from_keys(&mut self, amount: usize) {
        let mut rng = rand::thread_rng();
        let clean_char_set: Vec<char> = self.keys.clone().replace(" ", "").chars().collect();

        for _ in 0..amount {
            let mut new_word: String = "".to_owned();
            let max_length: u32 = rng.gen_range(3..=7);

            for _ in 1..max_length {
                let index = rng.gen_range(0..clean_char_set.len()) as usize;
                new_word.push(clean_char_set[index]);
            }
            self.words_buffer.push(new_word.to_string());
        }
    }

    pub(crate) fn last_word(&self) -> Option<&String> {
        if self.current_index == 0 {
            return None;
        }
        return Some(&self.words_buffer[self.current_index]);
    }

    pub(crate) fn load_dictionary(&mut self, path: &Path) -> Result<(), io::Error> {
        let mut buf = String::new();

        File::open(path)?.read_to_string(&mut buf)?;
        self.dictionary = serde_json::from_str(&buf)?;

        Ok(())
    }

    pub(crate) fn new(keys: &str, words_amount: usize) -> TextContainer {
        let mut container = TextContainer {
            input_buffer: "".to_owned(),
            words_buffer: Default::default(),
            current_index: 0,
            keys: keys.to_owned(),
            dictionary: None,
        };
        container.generate_words(words_amount);

        container
    }

    pub(crate) fn next_word(&self) -> Option<&String> {
        if self.current_index + 1 >= self.words_buffer.len() {
            return None;
        }
        return Some(&self.words_buffer[self.current_index + 1]);
    }

    pub(crate) fn try_increment(&mut self) {
        if self.current_index + 1 >= self.words_buffer.len() {
            self.generate_words_from_keys(32);
        } else {
            self.current_index += 1;
        }
    }
}
