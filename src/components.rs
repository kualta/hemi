use crate::keyboard::KeyboardState;
use eframe::egui::{Context, InputState, Vec2};
use eframe::epaint::Color32;
use rand::Rng;
use std::cell::RefCell;
use std::default::Default;
use std::ops::Not;
use std::rc::Rc;
use std::vec::Vec;

#[derive(PartialEq, Clone, Copy)]
pub enum PanelState {
    Enabled,
    Disabled,
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
    pub(crate) state: PanelState,
}

impl Panel {
    pub fn new(title: String, state: PanelState) -> Self {
        Panel { title, state }
    }
}

pub struct TextContainer {
    pub(crate) keys: String,
    pub(crate) input_buffer: String,
    pub(crate) words_buffer: Vec<String>,
    pub(crate) current_index: usize,
}

impl TextContainer {
    pub(crate) fn generate_words(&mut self, amount: u32) {
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

    pub(crate) fn new(keys: &str, words_amount: u32) -> TextContainer {
        let mut container = TextContainer {
            input_buffer: "".to_owned(),
            words_buffer: Default::default(),
            current_index: 0,
            keys: keys.to_owned(),
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
            self.generate_words(32);
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

pub struct AboutPanel {
    pub info: Panel,
}

impl Default for AboutPanel {
    fn default() -> Self {
        Self {
            info: Panel::new("About panel".to_owned(), PanelState::Disabled),
        }
    }
}

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
            info: Panel::new(keys.to_owned() + " panel", PanelState::Enabled),
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
    pub(crate) about_panel: Rc<RefCell<AboutPanel>>,
    pub(crate) left_panel: Rc<RefCell<TypingPanel>>,
    pub(crate) right_panel: Rc<RefCell<TypingPanel>>,
}

pub(crate) struct KeyboardPanel {
    pub(crate) keyboard: Rc<RefCell<KeyboardState>>,
    pub(crate) info: Panel,
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
            info: Panel {
                title: "Keyboard Panel".to_owned(),
                state: PanelState::Enabled,
            },
        }
    }
}
