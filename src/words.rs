use dioxus::html::input_data::keyboard_types::{Code, Key};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, vec::Vec};
use web_sys::HtmlAudioElement;

/// Stores pressed state of keys
#[derive(PartialEq, Clone)]
pub(crate) struct KeyState {
    key: Key,
    enabled: bool,
}

impl KeyState {
    pub(crate) fn new(key: &Key, enabled: bool) -> Self {
        KeyState {
            key: key.clone(),
            enabled,
        }
    }

    pub(crate) fn key(&self) -> &Key {
        &self.key
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Stores rows of [`KeyState`]s for the keyboard
#[derive(Clone)]
pub(crate) struct KeyboardState {
    keys: Vec<Vec<KeyState>>,
}

impl KeyboardState {
    pub(crate) fn new(dictionary: &WordDictionary) -> Self {
        let keys = dictionary
            .keys()
            .split_whitespace()
            .map(|row| {
                row.chars()
                    .map(|key| KeyState {
                        key: Key::from_str(&key.to_string()).expect("Non-existent key supplied"),
                        enabled: false,
                    })
                    .collect()
            })
            .collect();

        KeyboardState { keys }
    }

    pub(crate) fn update_for(&mut self, key: &KeyState) {
        self.keys.iter_mut().for_each(|row| {
            if let Some(key_state) = row.iter_mut().find(|key_state| {
                // FIXME: slow, ugly, stupid
                key_state.key.to_string().to_uppercase() == key.key.to_string().to_uppercase()
            }) {
                key_state.enabled = key.enabled
            }
        });
    }

    pub(crate) fn keys(&self) -> &Vec<Vec<KeyState>> {
        self.keys.as_ref()
    }
}

/// Stores dictionaries of words and keys they consist of.
///
/// # Note
/// `keys` is expected to be a whitespace-separated uppercase sequence of key rows
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct WordDictionary {
    buffer: Vec<String>,
    keys: String,
}

impl WordDictionary {
    pub(crate) fn keys(&self) -> &str {
        self.keys.as_ref()
    }
}

/// Maps Key [`Code`] to audio file path
#[derive(Clone)]
pub(crate) struct AudioLibrary {
    sounds: HashMap<Code, String>,
}

impl Default for AudioLibrary {
    fn default() -> Self {
        let path = "assets/tealios/";
        let extra: Vec<String> = vec![
            "Space".to_owned(),
            "Enter".to_owned(),
            "Backspace".to_owned(),
        ];
        let keys: Vec<String> = ('A'..='Z').map(|c| c.to_string()).chain(extra).collect();
        let files = keys.iter().map(|key| path.to_owned() + key + ".mp3");
        let codes = keys.iter().map(|key| match key.as_str() {
            "Space" => Code::Space,
            "Enter" => Code::Enter,
            "Backspace" => Code::Backspace,
            other => Code::from_str(&("Key".to_owned() + other))
                .unwrap_or_else(|_| panic!("key {} not found!", other)),
        });
        let sounds = codes.zip(files).collect();

        Self { sounds }
    }
}

impl AudioLibrary {
    pub(crate) fn play(&self, key: Code) {
        if self.sounds.contains_key(&key) {
            let _ = HtmlAudioElement::new_with_src(self.sounds.get(&key).unwrap())
                .expect("Audio file not found!")
                .play();
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct LayoutDictionary {
    pub(crate) left: WordDictionary,
    pub(crate) right: WordDictionary,
}

impl Default for LayoutDictionary {
    fn default() -> Self {
        LayoutDictionary {
            left: WordDictionary {
                buffer: vec![],
                keys: "".to_owned(),
            },
            right: WordDictionary {
                buffer: vec![],
                keys: "".to_owned(),
            },
        }
    }
}

impl LayoutDictionary {
    pub async fn pull() -> Self {
        let url = "https://raw.githubusercontent.com/kualta/Hemi/master/assets/qwerty.json";
        let data = reqwest::get(url)
            .await
            .unwrap()
            .json::<LayoutDictionary>()
            .await
            .unwrap();

        data
    }
}

/// Stores data for typing panel
#[derive(Default, Clone)]
pub(crate) struct TypingData {
    input: String,
    streak: i32,
    last_word: String,
    words_buffer: Vec<String>,
}

impl TypingData {
    /// Copies `amount` of elements from provided `dictionary` and constructs [WordBuffer] from them
    pub(crate) fn new(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut data = TypingData::default();
        data.generate_words(amount, dictionary);
        data
    }

    pub(crate) fn submit(&mut self) {
        self.last_word = self.input.clone();
        if !self.words_buffer.is_empty() {
            if self.input().trim() == self.words_buffer.get(0).unwrap() {
                self.streak += 1;
            } else {
                self.streak = 0;
            }

            self.words_buffer.remove(0);
        }

        self.input.clear();
    }

    pub(crate) fn last_word(&self) -> &str {
        self.last_word.as_ref()
    }

    pub(crate) fn next_word(&self) -> Option<&str> {
        match self.words_buffer.get(0) {
            Some(word) => Some(word.as_str()),
            None => None,
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.input.push_str(string)
    }

    pub(crate) fn input(&self) -> &str {
        self.input.as_ref()
    }

    pub(crate) fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub(crate) fn buffer(&self) -> &Vec<String> {
        self.words_buffer.as_ref()
    }

    pub(crate) fn drain(&mut self) {
        self.words_buffer.drain(..);
    }

    pub(crate) fn streak(&self) -> i32 {
        self.streak
    }

    pub(crate) fn generate_words(&mut self, amount: usize, dictionary: &WordDictionary) {
        let mut rng = rand::thread_rng();

        self.words_buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .map(|str| str.to_string())
            .collect::<Vec<String>>();
    }
}
