use super::state::GameStatePair;
use crate::auto_splitter_settings::Settings;
use asr::{print_message, time::Duration,
    timer::{self, TimerState},
};
use core::ops::Add;

pub fn auto_splitter_init(_settings: &Settings) {
    print_message(
        "KotOR 2 Auto Splitter - Attached to process, beginning main auto splitter loop",
    )
}
pub fn auto_splitter_is_loading(
    _vars: &GameStatePair,
    _settings: &Settings,
) -> bool {
    true
}


pub fn auto_splitter_loop(vars: &GameStatePair, settings: &Settings) {


    // If the timer is currently either running or paused,
    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        // then the isLoading action will be run
        if auto_splitter_is_loading(&vars, &settings) {
            timer::pause_game_time()
        } else {
            timer::resume_game_time();
        }
    }
}
