use gdextras::*;
use gdnative::{
    prelude::*,
    api::{KinematicBody}
};

pub type Base = KinematicBody;

#[derive(NativeClass)]
#[inherit(Base)]
pub struct Player {

    #[property]
    speed: f32,

    #[property]
    fall_acceleration: f32,

    velocity: Vector3
}

impl Player {

    pub fn new(_owner: TRef<Base>) -> Self {
        Player {
            speed: 14.,
            fall_acceleration: 75.,
            velocity: Vector3::ZERO
        }
    }

}

#[methods]
impl Player {

    #[export]
    fn _physics_process(&mut self, owner: TRef<Base>, delta: f32) {

        let input_events = Input::godot_singleton();

        let mut direction = Vector3::ZERO;

        if input_events.is_action_pressed("move_forward", false) {direction.z -= 1.}
        if input_events.is_action_pressed("move_right", false) {direction.x += 1.}
        if input_events.is_action_pressed("move_backward", false) {direction.z += 1.}
        if input_events.is_action_pressed("move_left", false) {direction.x -= 1.}

        if direction != Vector3::ZERO {
            direction = direction.normalized();

            match get_node::<Base, Spatial>(owner.clone(), "Pivot") {
                Err(e) => gd_print!(owner, e, "{:?}", e),
                Ok(pivot) => {
                    pivot.look_at(direction + owner.translation(), Vector3::UP)
                }
            }
        }

        //  Ground velocity
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;

        //  Fall velocity
        self.velocity.y -= self.fall_acceleration * delta;

        //  Update player position
        self.velocity = owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.7, true)
    }

}
