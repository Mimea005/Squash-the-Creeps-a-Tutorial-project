use gdextras::*;
use gdnative::{
    prelude::*,
    api::KinematicBody
};
use rand::{Rng, thread_rng};

pub type Base = KinematicBody;

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct Mob {

    #[export]
    min_speed: f32,

    #[export]
    max_speed: f32,

    velocity: Vector3

}

impl Mob {

    pub fn new(_owner: TRef<Base>) -> Self {
        Self {
            min_speed: 10.,
            max_speed:18.,
            velocity: Vector3::ZERO
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.property("Speed/Min")
            .with_setter(|mut s, _, v: f32| s.min_speed = v)
            .with_getter(|s,_|s.min_speed)
            .with_default(10.)
            .done();

        builder.property("Speed/Max")
            .with_setter(|mut s, _, v: f32| s.max_speed = v)
            .with_getter(|s,_|s.max_speed)
            .with_default(18.)
            .done();
    }

    pub fn initialize(&mut self, owner:TRef<Base>, start_pos: Vector3, player_pos: Vector3) {

        use std::f64::consts::FRAC_PI_4;
        let mut rng = thread_rng();

        owner.look_at_from_position(start_pos, player_pos, Vector3::UP);
        owner.rotate_y(rng.gen_range(-FRAC_PI_4..FRAC_PI_4));

        let speed = rng.gen_range(self.min_speed..self.max_speed);
        self.velocity = Vector3::new(0.,0.,speed);

        self.velocity.rotated(Vector3::UP, owner.rotation().y);
    }
}

#[methods]
impl Mob {

    #[export]
    fn _physics_process(&self, owner: TRef<Base>, _delta: f32) {

        owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.7, true);

    }

    #[export]
    fn _mob_screen_exited(&self, owner: TRef<Base>) {
        gd_print!(owner, p, "I'm free!");
        owner.queue_free()
    }

}
