use gdnative::prelude::*;

use gdnative::api::OS;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player {
    velocity: Vector2,
    #[property(path = "base/gravity")]
    gravity: f32,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::new(0.0, 0.0),
            gravity: 1000.0,
        }
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        let os = OS::godot_singleton();
        os.center_window();
        godot_print!("hello, Dane.");
    }

    #[export]
    pub fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        godot_print!("{:?}", self.velocity);
        let input = Input::godot_singleton();
        self.velocity.y += self.gravity * delta as f32;

        let mut direction = Vector2::new(0.0, 0.0);
        direction.x = Input::get_action_strength(&input, "move_right") as f32
            - Input::get_action_strength(&input, "move_left") as f32;

        if owner.is_on_floor() && Input::is_action_pressed(&input, "jump") {
            direction.y = -15.0;
        }

        self.velocity += direction * 50.0;

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector2::new(0.0, 1.0),
            false,
            100,
            45.0,
            true,
        );
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
