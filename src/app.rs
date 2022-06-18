use eframe::egui::{self, CentralPanel, Style, Visuals};
use eframe::egui::{
    Align, Align2, Button, Color32, Context, InputState, RichText, Stroke, Ui, Vec2,
};
use eframe::Frame;
use rand::Rng;
use std::default::Default;
use std::ops::Not;
use std::vec::Vec;

use crate::keyboard::{InputKey, KeyboardState};
use crate::{ApplicationConfig, StyleConfig};

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

#[derive(PartialEq, Clone, Copy)]
enum PanelState {
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

trait Drawable {
    fn draw(&mut self, ctx: &Context);
}

pub(crate) struct Panel {
    style: StyleConfig,
    title: String,
    state: PanelState,
}

impl Panel {
    fn new(title: String, state: PanelState) -> Self {
        Panel {
            style: Default::default(),
            title, 
            state,
        }
    }
}

pub(crate) struct AboutPanel {
    info: Panel
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
    info: Panel,
    text: TextContainer,
    keyboard: KeyboardState,
}

impl TypingPanel {
    fn new(keys: &str) -> Self {
        TypingPanel {
            info: Panel::new(keys.to_owned() + " panel", PanelState::Enabled),
            text: TextContainer::new(keys, 10),
            keyboard: KeyboardState::new(keys)
        }
    }

    fn update(&mut self, ctx: &Context) {
        self.update_keyboard_state(&ctx.input());
        self.text.update_input_buffer(&self.keyboard);
    }

    fn update_keyboard_state(&mut self, input: &InputState) {
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

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub config: ApplicationConfig,
    left_panel: TypingPanel,
    right_panel: TypingPanel,
    about_panel: AboutPanel
}

impl Default for App {
    fn default() -> Self {
        const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
        const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

        let mut app = App {
            config: ApplicationConfig::new(),
            left_panel: TypingPanel::new(LEFT_QWERTY_KEYS),
            right_panel: TypingPanel::new(RIGHT_QWERTY_KEYS),
            about_panel: AboutPanel::default()
        };
        app.right_panel.info.state = PanelState::Disabled;

        app
    }
}

impl App {
    fn draw_keys(style_config: &StyleConfig, ui: &mut Ui, keyboard: &KeyboardState) {
        let button_size = style_config.button_size;
        ui.spacing_mut().item_spacing = style_config.button_spacing;

        let mut current_row_indent = 0.;
        for row in &keyboard.rows {
            ui.horizontal(|ui| {
                ui.add_space(current_row_indent);
                for key in row {
                    let width_mul = if key.key == egui::Key::Space { 4.6 } else { 1. };
                    ui.add_sized(
                        Vec2::new(button_size * width_mul, button_size),
                        Button::new(key.character.to_string())
                            //                   converting bool to either 0. or 1.
                            .stroke(Stroke::new(key.down as i32 as f32, Color32::WHITE)),
                    );
                }
            });
            current_row_indent += style_config.button_indent;
        }
    }

    fn draw_top_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("Switch side")
                    .clicked()
                {
                    self.about_panel.info.state = PanelState::Disabled;
                    self.left_panel.info.state = !self.left_panel.info.state;
                    self.right_panel.info.state = !self.right_panel.info.state;
                }
                if ui.button("About").clicked() {
                    self.about_panel.info.state = !self.about_panel.info.state;
                }
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.draw_top_bar(ctx, frame);

        CentralPanel::default().show(ctx, |_ui| {
            if self.about_panel.info.state == PanelState::Enabled {
                self.about_panel.draw(ctx);
                return;
            }
            if self.left_panel.info.state == PanelState::Enabled {
                self.left_panel.update(ctx);
                self.left_panel.draw(ctx);
            } else if self.right_panel.info.state == PanelState::Enabled {
                self.right_panel.update(ctx);
                self.right_panel.draw(ctx);
            }         
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }
}
