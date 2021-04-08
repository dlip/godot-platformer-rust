use gdnative::prelude::*;

use gdnative::api::{AnimationPlayer, OS};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
    #[property(default = 1000.0)]
    gravity: f32,
    #[property(default = 10000.0)]
    max_speed: f32,
    #[property(default = 4000.0)]
    acceleration: f32,
    #[property(default = 1000.0)]
    friction: f32,
    #[property(default = 25000.0)]
    jump_power: f32,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::new(0.0, 0.0),
            gravity: 1000.0,
            max_speed: 10000.0,
            friction: 1000.0,
            jump_power: 25000.0,
            acceleration: 4000.0,
        }
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        let os = OS::godot_singleton();
        os.center_window();
    }

    #[export]
    pub fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        // godot_print!("{:?}", owner.position().y);

        if owner.position().y > 600.0 {
            let tree = owner.get_tree().expect("could not retrieve SceneTree");
            let tree = unsafe { tree.assume_safe() };
            let res: std::result::Result<(), GodotError> = tree.change_scene("res://Dead.tscn");
        }
        let input = Input::godot_singleton();
        self.velocity.y += self.gravity * delta as f32;

        let direction = Input::get_action_strength(&input, "move_right") as f32
            - Input::get_action_strength(&input, "move_left") as f32;

        if owner.is_on_floor() && Input::is_action_pressed(&input, "jump") {
            self.velocity.y = -self.jump_power * delta as f32;
        }

        self.velocity.x += direction * self.acceleration * delta as f32;
        self.velocity.x = self
            .velocity
            .x
            .min(self.max_speed * delta as f32)
            .max(-self.max_speed* delta as f32);

        if self.velocity.x > 0.0 {
            self.velocity.x -= self.friction * delta as f32;
            self.velocity.x = self.velocity.x.max(0.0);
        } else if self.velocity.x < 0.0 {
            self.velocity.x += self.friction * delta as f32;
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

        unsafe {
            owner
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .map(|player| {
                    if self.velocity.length() > 0.0 {
                        player.play("walk", 0.0, 1.0, false);
                    } else {
                        player.play("idle", 0.0, 1.0, false);
                    }
                });

            if self.velocity.x != 0.0 {
                owner
                    .get_node_as::<Sprite>("dragon")
                    .map(|d| d.set_flip_h(self.velocity.x > 0.0));
            }
        }
    }
}
