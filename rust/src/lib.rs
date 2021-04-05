use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player;

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        godot_print!("hello, Dane.")
    }

    #[export]
    pub fn _physics_process(&self, owner: &KinematicBody2D, _delta: f64) {
        let velocity = Vector2::new(1.0, 0.0);

        owner.move_and_collide(velocity, false, false, false);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
