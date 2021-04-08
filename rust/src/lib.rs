use gdnative::prelude::*;

mod play_again;
mod player;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<play_again::PlayAgain>();
}

godot_init!(init);
