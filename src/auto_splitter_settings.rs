use asr::settings::Gui;
use asr::settings::gui::Title;

#[derive(Gui)]
pub struct Settings {
    /// DUMMY SETTING
    #[default = false]
    pub reset: bool,
    }
