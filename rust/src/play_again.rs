use gdnative::api::{AnimationPlayer, OS};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Button)]
pub struct PlayAgain {}

#[gdnative::methods]
impl PlayAgain {
    fn new(_owner: &Button) -> Self {
        PlayAgain {}
    }

    #[export]
    fn _ready(&self, _owner: &Button) {
        godot_print!("{:?}", "button");
    }

    #[export]
    fn on_play_again_button_down(&self, owner: &Button) {
        godot_print!("{:?}", "click");

        let tree = owner.get_tree().expect("could not retrieve SceneTree");
        let tree = unsafe { tree.assume_safe() };

        tree.change_scene("res://Game.tscn");

        // let button = unsafe { owner.get_node_as::<Button>("start_button").unwrap() };
        // button.hide();
        // owner.emit_signal("start_game", &[]);
    }
}
