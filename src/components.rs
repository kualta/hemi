use crate::keyboard::KeyboardState;
use eframe::egui::{Context, InputState, Vec2};
use eframe::epaint::Color32;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Not;
use std::path::Path;
use std::rc::Rc;
use std::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum PanelState {
    Enabled,
    Disabled,
}

pub(crate) enum AppPanels {
    LeftTypingPanel,
    RightTypingPanel,
    AboutPanel,
}

impl Into<bool> for PanelState {
    fn into(self) -> bool {
        match self {
            PanelState::Enabled => true,
            PanelState::Disabled => false,
        }
    }
}

impl Not for PanelState {
    type Output = PanelState;

    fn not(self) -> Self::Output {
        match self {
            PanelState::Enabled => PanelState::Disabled,
            PanelState::Disabled => PanelState::Enabled,
        }
    }
}

impl PanelState {
    pub(crate) fn reverse(&mut self) {
        *self = Not::not(*self);
    }
}

pub struct Panel {
    pub(crate) title: String,
}

impl Panel {
    pub(crate) fn new(title: String) -> Self {
        Panel { title }
    }
}

pub struct TextContainer {
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

    fn update_input_buffer(&mut self, keyboard: &KeyboardState) {
        //              spacebar
        if keyboard.rows[3][0].pressed {
            self.input_buffer.clear();
            self.try_increment();
        }
    }
}

pub struct AboutPanel {}

pub(crate) struct TypingPanel {
    pub(crate) info: Panel,
    pub(crate) text: TextContainer,
    pub(crate) keyboard_panel: KeyboardPanel,
    pub(crate) keyboard: Rc<RefCell<KeyboardState>>,
}

impl TypingPanel {
    pub fn new(keys: &str) -> Self {
        let keyboard = Rc::new(RefCell::new(KeyboardState::new(keys)));

        TypingPanel {
            info: Panel::new(keys.to_owned() + " panel"),
            text: TextContainer::new(keys, 10),
            keyboard_panel: KeyboardPanel::new(
                keyboard.clone(),
                75.,
                Vec2::new(10., 10.),
                35.,
                Color32::WHITE,
            ),
            keyboard,
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.update_keyboard_state(&ctx.input());
        self.text.update_input_buffer(&self.keyboard.borrow());
    }

    pub fn update_keyboard_state(&mut self, input: &InputState) {
        for row in &mut self.keyboard.borrow_mut().rows {
            for key in row {
                key.down = input.key_down(key.key);
                key.pressed = input.key_pressed(key.key);
            }
        }
    }
}

pub(crate) struct TopBar {
    pub(crate) left_panel: Rc<RefCell<TypingPanel>>,
    pub(crate) right_panel: Rc<RefCell<TypingPanel>>,
    pub(crate) active_panel: Rc<RefCell<AppPanels>>,
}

pub(crate) struct KeyboardPanel {
    pub(crate) keyboard: Rc<RefCell<KeyboardState>>,
    pub(crate) state: PanelState,
    pub(crate) button_size: f32,
    pub(crate) button_spacing: Vec2,
    pub(crate) row_indent: f32,
    pub(crate) stroke_color: Color32,
}

impl KeyboardPanel {
    pub(crate) fn new(
        keyboard: Rc<RefCell<KeyboardState>>,
        button_size: f32,
        button_spacing: Vec2,
        row_indent: f32,
        stroke_color: Color32,
    ) -> Self {
        KeyboardPanel {
            keyboard,
            button_size,
            button_spacing,
            row_indent,
            stroke_color,
            state: PanelState::Enabled,
        }
    }
}
