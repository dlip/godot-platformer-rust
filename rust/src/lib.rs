use gdnative::prelude::*;

use gdnative::api::{AnimationPlayer, OS};

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
        // godot_print!("{:?}", self.velocity);
        let input = Input::godot_singleton();
        self.velocity.y += self.gravity * delta as f32;

        let player = unsafe {
            owner
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .unwrap()
        };

        let dragon = unsafe { owner.get_node_as::<Sprite>("dragon").unwrap() };

        let direction = Input::get_action_strength(&input, "move_right") as f32
            - Input::get_action_strength(&input, "move_left") as f32;

        if owner.is_on_floor() && Input::is_action_pressed(&input, "jump") {
            self.velocity.y = -20000.0 * delta as f32;
        }

        self.velocity.x += direction * 2000.0 * delta as f32;
        self.velocity.x = self
            .velocity
            .x
            .min(7000.0 * delta as f32)
            .max(-7000.0 * delta as f32);

        if self.velocity.x > 0.0 {
            self.velocity.x -= 400.0 * delta as f32;
            self.velocity.x = self.velocity.x.max(0.0);
        } else if self.velocity.x < 0.0 {
            self.velocity.x += 500.0 * delta as f32;
            self.velocity.x = self.velocity.x.min(0.0);
        }

        self.velocity = owner.move_and_slide(
            self.velocity,
            Vector2::new(0.0, 1.0),
            false,
            100,
            45.0,
            true,
        );

        if self.velocity.length() > 0.0 {
            player.play("walk", 0.0, 1.0, false);
        } else {
            player.play("idle", 0.0, 1.0, false);
        }

        if self.velocity.x != 0.0 {
            dragon.set_flip_h(self.velocity.x > 0.0);
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
