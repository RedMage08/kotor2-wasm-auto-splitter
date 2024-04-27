use asr::{
    watcher::{Pair, Watcher},
    Address,
};

macro_rules! define_address {
    ($name:ident, $addr:expr) => {
        pub const $name: Address = Address::new($addr);
    };
}

define_address!(isNotLoadingAddress, 0x02C1D4);
//define_address!(isActiveWindowAddress, 0x61B4E0);
//define_address!(isMoviePlayingAddress, 0x07A00C);
//define_address!(modifierKeysAddress, 0x0C1FD8);

#[derive(Default)]
pub struct GameState {
    pub isNotLoading: Watcher<bool>,
    //pub isActiveWindow: Watcher<bool>,
    //pub isMoviePlaying: Watcher<bool>,
    //pub modifierKeys: Watcher<bool>,

}

#[derive()]
pub struct GameStatePair {
    pub isNotLoading: Pair<bool>,
    //pub isActiveWindow: Pair<bool>,
    //pub isMoviePlaying: Pair<bool>,
    //pub modifierKeys: Pair<bool>,
}
