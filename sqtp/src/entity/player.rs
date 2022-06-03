use gdextras::*;
use gdnative::{
    prelude::*,
    api::{KinematicBody}
};
use crate::entity::mob::Mob;

pub type Base = KinematicBody;

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct Player {

    speed: f32,

    fall_acceleration: f32,

    jump_force: f32,

    bounce_force: f32,

    velocity: Vector3
}

impl Player {

    fn register(builder: &ClassBuilder<Self>) {

        builder.property("Speed")
            .with_setter(|mut s, _, v: f32| s.speed = v)
            .with_getter(|s, _| s.speed)
            .with_default(14.)
            .done();

        builder.property("Fall Acceleration")
            .with_setter(|mut s, _, v:f32| s.fall_acceleration = v)
            .with_getter(|s, _| s.fall_acceleration)
            .with_default(75.)
            .done();

        builder.property("Jump force")
            .with_getter(|s,_|s.jump_force)
            .with_setter(|mut s, _, v: f32| s.jump_force = v)
            .with_default(20.)
            .done();

        builder.property("Bounce force")
            .with_setter(|mut s, _, v| s.bounce_force = v)
            .with_getter(|s,_|s.bounce_force)
            .with_default(16.)
            .done();

        builder.signal("hit").done();
    }

    pub fn new(_owner: TRef<Base>) -> Self {
        Player {
            speed: 14.,
            fall_acceleration: 75.,
            jump_force: 20.,
            bounce_force: 16.,
            velocity: Vector3::ZERO
        }
    }

}

#[methods]
impl Player {

    #[export]
    fn _ready(&self, owner: TRef<Base>) {
        let persistent = get_node::<Base, Node>(owner.clone(), "/root/Persistent").unwrap();
        owner.connect("hit", persistent, "_on_player_hit", VariantArray::new().into_shared(), 0).unwrap();
    }

    #[export]
    fn _physics_process(&mut self, owner: TRef<Base>, delta: f32) {

        let input_events = Input::godot_singleton();

        let mut direction = Vector3::ZERO;

        if input_events.is_action_pressed("move_forward", false) {direction.z -= 1.}
        if input_events.is_action_pressed("move_right", false) {direction.x += 1.}
        if input_events.is_action_pressed("move_backward", false) {direction.z += 1.}
        if input_events.is_action_pressed("move_left", false) {direction.x -= 1.}

        //Jump
        if owner.is_on_floor() && input_events.is_action_pressed("action_jump", false) {
            self.velocity.y += self.jump_force;
        }

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

        for i in 0..owner.get_slide_count() {

            let collision = owner.get_slide_collision(i).unwrap();

            let collision = unsafe {
                collision.assume_safe()
            };

            let collider = unsafe {
                collision.collider().unwrap().assume_safe()
                    .cast::<Node>().unwrap()
            };

            if collider.is_in_group("mob") {

                let mob = collider.cast::<KinematicBody>().unwrap()
                            .cast_instance::<Mob>().unwrap();

                if Vector3::UP.dot(collision.normal()) > 0.1 {
                    mob.map(|s,o|s.squash(o)).unwrap();
                    self.velocity.y = self.bounce_force;
                }
            }

        }

        //  Update player position
        self.velocity = owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.7, true);

    }

    #[export]
    fn _die(&self, owner: TRef<Base>, _body: Ref<Node>) {
        owner.emit_signal("hit", &[]);
        owner.queue_free();
    }

}
