//built on the foundation of code from Jujstme(https://github.com/SonicSpeedrunning/LiveSplit.SonicColorsUltimate/) and Refragg(https://github.com/refragg/sadx-wasm-auto-splitter)

#![no_std]

mod auto_splitter;
mod auto_splitter_settings;
pub mod state;

use crate::auto_splitter_settings::Settings;
use crate::auto_splitter::*;
use crate::state::*;
use asr::future::retry;
use asr::{future::next_tick, print_limited, Process};
use asr::settings::Gui;

//use crate::debug::*;

asr::async_main!(stable);
asr::panic_handler!();
const PROCESS_NAMES: &[&str] = &["swkotor2.exe", "KOTOR2.exe"];


async fn main() {

    let mut settings = Settings::register();



    loop {
      // Hook to the target process
        let process = retry(|| PROCESS_NAMES.iter().find_map(|&name| Process::attach(name))).await;

        process
            .until_closes(async {

            let mut game_state = GameState::default();

                loop {
                   settings.update();

                    macro_rules! unwrap_or_next_tick_opt {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Some(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! unwrap_or_next_tick_res {
                        ( $e:expr, $s:expr ) => {
                            match $e {
                                Ok(x) => x,
                                _ => {
                                    print_limited::<128>($s);
                                    next_tick().await;
                                    continue;
                                }
                            }
                        };
                    }

                    macro_rules! read_mem {
                        ( $name:ident, $addr:expr, $t:ty) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state.$name.update(Some(unwrap_or_next_tick_res!(
                                    process.read::<$t>($addr),
                                    &format_args!("Failed reading {}", stringify!($name))
                                ))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }

                   macro_rules! read_mem_and_map {
                        ( $name:ident, $addr:expr, $t:ty, $mapper:expr) => {
                            let $name = *unwrap_or_next_tick_opt!(
                                game_state
                                    .$name
                                    .update(Some($mapper(unwrap_or_next_tick_res!(
                                        process.read::<$t>($addr),
                                        &format_args!("Failed reading {}", stringify!($name))
                                    )))),
                                &format_args!("Failed updating {}", stringify!($name))
                            );
                        };
                    }

                    read_mem_and_map!(isNotLoading, isNotLoadingAddress, i32, |value| {
                        value != 0
                    });
                   // read_mem_and_map!(isActiveWindow, isActiveWindowAddress, i32, |value| {
                   //     value != 0
                   // });
                   // read_mem_and_map!(isMoviePlaying, isMoviePlayingAddress, i32, |value| {
                   //     value != 0
                   // });
                   // read_mem_and_map!(modifierKeys, modifierKeysAddress, i32, |value| {
                   //     value != 0
                   // });

                    let vars = GameStatePair {
                        isNotLoading,
                    };


                    auto_splitter_loop(&vars, &settings);
                    //print_game_state(&vars);

                    next_tick().await;
                }
            })
            .await;
    }
}



