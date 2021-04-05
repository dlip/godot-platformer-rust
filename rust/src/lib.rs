use gdnative::prelude::*;

use gdnative::api::OS;

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
        let os = OS::godot_singleton();
        os.center_window();
        godot_print!("hello, Dane.");
    }

    #[export]
    pub fn _physics_process(&self, owner: &KinematicBody2D, _delta: f64) {
        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(&input, "ui_right") {
            velocity.x = 1.0
        }
        if Input::is_action_pressed(&input, "ui_left") {
            velocity.x = -1.0
        }
        if Input::is_action_pressed(&input, "ui_down") {
            velocity.y = 1.0
        }
        if Input::is_action_pressed(&input, "ui_up") {
            velocity.y = -1.0
        }

        owner.move_and_collide(velocity, false, false, false);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
