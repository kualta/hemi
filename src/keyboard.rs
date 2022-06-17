use eframe::egui;

pub(crate) struct KeyboardState {
    pub(crate) rows: Vec<Vec<InputKey>>,
}

impl KeyboardState {
    pub(crate) fn new(keys: &str) -> Self {
        let mut keyboard = KeyboardState { rows: Vec::new() };

        for row in keys.split_whitespace() {
            let mut input_row = Vec::new();
            for character in row.chars() {
                input_row.push(InputKey::from(character));
            }
            keyboard.rows.push(input_row);
        }

        // Add space bar as last input row
        let mut space_bar: Vec<InputKey> = Vec::new();
        space_bar.push(InputKey::new(' ', egui::Key::Space, false));
        keyboard.rows.push(space_bar);

        return keyboard;
    }
}

pub(crate) struct InputKey {
    pub(crate) character: char,
    pub(crate) key: egui::Key,
    pub(crate) pressed: bool,
}

impl InputKey {
    fn new(character: char, key: egui::Key, pressed: bool) -> Self {
        InputKey {
            character,
            key,
            pressed,
        }
    }
}

impl From<char> for InputKey {
    fn from(character: char) -> Self {
        let key = match character {
            'A' => egui::Key::A,
            'B' => egui::Key::B,
            'C' => egui::Key::C,
            'D' => egui::Key::D,
            'E' => egui::Key::E,
            'F' => egui::Key::F,
            'G' => egui::Key::G,
            'H' => egui::Key::H,
            'I' => egui::Key::I,
            'J' => egui::Key::J,
            'K' => egui::Key::K,
            'L' => egui::Key::L,
            'M' => egui::Key::M,
            'N' => egui::Key::N,
            'O' => egui::Key::O,
            'P' => egui::Key::P,
            'Q' => egui::Key::Q,
            'R' => egui::Key::R,
            'S' => egui::Key::S,
            'T' => egui::Key::T,
            'U' => egui::Key::U,
            'V' => egui::Key::V,
            'W' => egui::Key::W,
            'X' => egui::Key::X,
            'Y' => egui::Key::Y,
            'Z' => egui::Key::Z,
            ' ' => egui::Key::Space,
            // ';' => egui::Key::Semicolon,
            // TODO: Add special characters handling when egui adds support for them ¯\_(ツ)_/¯
            _ => egui::Key::Escape,
        };

        InputKey {
            character,
            key,
            pressed: false,
        }
    }
}
