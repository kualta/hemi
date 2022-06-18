use eframe::egui::{self, CentralPanel};
use eframe::egui::{ Align, Align2, Context, InputState, RichText, Vec2, };
use rand::Rng;
use std::cell::RefCell;
use std::default::Default;
use std::ops::Not;
use std::vec::Vec;
use std::rc::Rc;

use crate::app::App;
use crate::keyboard::{KeyboardState};
use crate::{StyleConfig};

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
    fn reverse(&mut self) {
        *self = Not::not(*self);
    }
}

pub trait Drawable {
    fn draw(&mut self, ctx: &Context);
}

pub struct Panel {
    pub style: StyleConfig,
    pub title: String,
    pub state: PanelState,
}

impl Panel {
    pub fn new(title: String, state: PanelState) -> Self {
        Panel {
            style: Default::default(),
            title, 
            state,
        }
    }
}

pub struct TextContainer {
    keys: String,
    input_buffer: String,
    words_buffer: Vec<String>,
    current_index: usize,
    max_buffered_chars: u32,
}

impl TextContainer {
    fn new(keys: &str, words_amount: u32) -> TextContainer {
        let mut container = TextContainer {
            input_buffer: "".to_owned(),
            words_buffer: Default::default(),
            current_index: 0,
            max_buffered_chars: 10,
            keys: keys.to_owned(),
        };
        container.generate_words(words_amount);

        container
    }

    fn get_last_word(&self) -> Option<&String> {
        if self.current_index == 0 {
            return None;
        }
        return Some(&self.words_buffer[self.current_index]);
    }

    fn get_next_word(&self) -> Option<&String> {
        if self.current_index + 1 >= self.words_buffer.len() {
            return None;
        }
        return Some(&self.words_buffer[self.current_index + 1]);
    }

    fn generate_words(&mut self, amount: u32) {
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

    fn update_input_buffer(&mut self, keyboard: &KeyboardState) {
        //              spacebar
        if keyboard.rows[3][0].pressed {
            self.input_buffer.clear();
            self.try_increment();
        }
    }

    fn try_increment(&mut self) {
        if self.current_index + 1 >= self.words_buffer.len() {
            self.generate_words(32);
        } else {
            self.current_index += 1;
        }
    }
}

pub struct AboutPanel {
    pub info: Panel
}

impl Default for AboutPanel {
    fn default() -> Self {
        Self { info: Panel::new("About panel".to_owned(), PanelState::Disabled) }
    }
}

impl Drawable for AboutPanel {
    fn draw(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::Area::new("about_area")
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.horizontal(|ui| {
                        egui::Layout::top_down_justified(Align::Center);
                        ui.label("made by ");
                        ui.hyperlink_to("lectro.moe", "https://lectro.moe/");
                    });
                    ui.horizontal(|ui| {
                        ui.label("powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to( "eframe", "https://github.com/emilk/egui/tree/master/eframe",);
                    });
                    ui.add_space(4.);
                    egui::warn_if_debug_build(ui);
                });
        });
    }
}

pub struct TypingPanel {
    pub info: Panel,
    pub text: TextContainer,
    pub keyboard: KeyboardState,
}

impl TypingPanel {
    pub fn new(keys: &str) -> Self {
        TypingPanel {
            info: Panel::new(keys.to_owned() + " panel", PanelState::Enabled),
            text: TextContainer::new(keys, 10),
            keyboard: KeyboardState::new(keys)
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.update_keyboard_state(&ctx.input());
        self.text.update_input_buffer(&self.keyboard);
    }

    pub fn update_keyboard_state(&mut self, input: &InputState) {
        for row in &mut self.keyboard.rows {
            for key in row {
                key.down = input.key_down(key.key);
                key.pressed = input.key_pressed(key.key);
            }
        }
    }
}

impl Drawable for TypingPanel {
    fn draw(&mut self, ctx: &Context) {
        egui::Window::new(&self.info.title)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., 0.))
            .min_height(800.)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top(&self.info.title)
                    .resizable(false)
                    .height_range(250. ..=250.)
                    .show_inside(ui, |ui| {
                        ui.add_space(125.);
                        ui.horizontal(|ui| {
                            ui.add_space(75.);
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text.get_last_word().unwrap_or(&"".to_owned()),
                                )),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::widgets::TextEdit::singleline(&mut self.text.input_buffer)
                                    .cursor_at_end(true),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text.get_next_word().unwrap_or(&"".to_owned()),
                                )),
                            );
                        });
                    });
                ui.add_space(120.);
                App::draw_keys(&self.info.style, ui, &mut self.keyboard);
                ui.add_space(50.);
            });
    }
}

pub(crate) struct TopBar {
    pub(crate) info: Panel,
    pub(crate) about_panel: Rc<RefCell<AboutPanel>>,
    pub(crate) left_panel: Rc<RefCell<TypingPanel>>,
    pub(crate) right_panel: Rc<RefCell<TypingPanel>>,
}

impl Drawable for TopBar {
    fn draw(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("Switch side")
                    .clicked()
                {
                    self.about_panel.borrow_mut().info.state = PanelState::Disabled;
                    self.left_panel.borrow_mut().info.state.reverse();
                    self.right_panel.borrow_mut().info.state.reverse();
                }
                if ui.button("About").clicked() {
                    self.about_panel.borrow_mut().info.state.reverse();
                }
            });
        });
    }
}

