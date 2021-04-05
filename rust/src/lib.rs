use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct Player;

#[gdnative::methods]
impl Player {
    fn new(_owner: &Node) -> Self {
        Player
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.")
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
