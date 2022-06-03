use gdextras::*;
use gdnative::{
    prelude::*,
    api::{
        KinematicBody,
        Path as Path3D,
        PathFollow
    }
};
use rand::prelude::*;
use crate::entity::player;
use crate::scene::persistent::Persistent;

pub type Base = KinematicBody;

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct Mob {

    min_speed: f32,

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

        builder.signal("squashed").done();

    }

    pub fn initialize(&mut self, owner: TRef<Base, Unique>, start_pos: Vector3, player_pos: Vector3, rng: &mut ThreadRng) {

        use core::f32::consts::FRAC_PI_4;

        let rotation = start_pos.direction_to(player_pos);
        let mut rotation = Vector2::new(rotation.x, rotation.z);
        let mut angle = rotation.angle_to(Vector2::UP);

        angle += rng.gen_range(-FRAC_PI_4..FRAC_PI_4);
        rotation = Vector2::UP.rotated(angle);

        owner.set_translation(start_pos);
        owner.set_rotation(Vector3::new(0.,angle, 0.));

        let speed = rng.gen_range(self.min_speed..self.max_speed);

        self.velocity = Vector3::new(rotation.x, -0.1, rotation.y);

        self.velocity *= speed;
    }

    pub fn squash(&self, owner: TRef<Base>) {
        let spawn_timer: TRef<Timer> = get_node(owner.clone(), "../SpawnInterval").unwrap();
        let persistant = get_instance::<Base, Node, Persistent>(owner.clone(), "/root/Persistent").unwrap();

        let time = match persistant.map(|s, _|s.score).unwrap() {
            0 => 0.5,
            a => a as f64
        };

        spawn_timer.set_wait_time(1. / time);

        owner.emit_signal("squashed", &[]);
        owner.queue_free();
    }
}

#[methods]
impl Mob {

    #[export]
    fn _physics_process(&self, owner: TRef<Base>, _delta: f32) {

        owner.move_and_slide(self.velocity, Vector3::FORWARD, false, 4, 0.7, true);

    }

    #[export]
    fn _mob_screen_exited(&self, owner: TRef<Base>) {
        owner.queue_free()
    }

}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct MobSpawn {
    scene: Option<Ref<PackedScene>>,
    target: Option<String>,
    s_spawn_interval: f64,
    rng: ThreadRng
}

impl MobSpawn {
    pub fn new(_owner: TRef<Node>) -> Self {
        Self {
            scene: None,
            target: None,
            s_spawn_interval: 1.,
            rng: thread_rng()
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.property("PackedScene")
            .with_setter(|mut s, _, v| s.scene = Some(v))
            .with_getter(
                |s,_| match &s.scene {
                    None => PackedScene::new().into_shared(),
                    Some(scene) => scene.clone()
                }
            )
            .done();

        builder.property("Spawn Interval Seconds")
            .with_setter(|mut s, _, v|s.s_spawn_interval = v)
            .with_getter(|s,_| s.s_spawn_interval)
            .with_default(1.)
            .done();

        builder.property("Look at target")
            .with_setter(|mut s,_,v: String|s.target = Some(v))
            .with_getter(|s,_| {
                match &s.target {
                    None => String::new(),
                    Some(t) => t.clone()
                }
            })
            .done();
    }
}

#[methods]
impl MobSpawn {

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        match get_node::<Node, Path3D>(owner.clone(), "SpawnPath") {
            Err(err) => gd_print!(owner, e, "Missing requirement -> {:?}", err),
            Ok(spawn_path) => {

                let spawn_path_follower = PathFollow::new();
                spawn_path.add_child(spawn_path_follower, true);

                let spawn_timer = Timer::new();
                spawn_timer.set_name("SpawnInterval");
                spawn_timer.set_wait_time(self.s_spawn_interval);
                spawn_timer.set_one_shot(false);
                spawn_timer.set_autostart(true);

                spawn_timer.connect("timeout", owner.clone(), "_spawn_mob", VariantArray::new().into_shared(), 0).unwrap();

                owner.add_child(spawn_timer, true);

            }
        }
    }

    #[export]
    fn _spawn_mob(&mut self, owner: TRef<Node>) {
        match &mut self.scene {
            None => gd_print!(owner, e, "No mob entity assigned"),
            Some(mob_res) => {

                let mob_res = unsafe {
                    mob_res.assume_safe()
                };

                let instance = mob_res.instance(0).unwrap();
                let mob = unsafe {
                    instance.assume_unique()
                        .cast::<Base>().unwrap()
                        .cast_instance::<Mob>().unwrap()
                };

                let persistent = get_instance::<Node, Node, Persistent>(owner.clone(), "/root/Persistent").unwrap();
                let location_gen: TRef<PathFollow> = get_node(owner.clone(), "SpawnPath/PathFollow").unwrap();

                location_gen.set_unit_offset(self.rng.gen_range(0_f64..1.));

                let start_pos = location_gen.translation();

                let target_pos = match &self.target {
                    Some(target) => get_node::<Node, player::Base>(owner, target.as_str()).unwrap().translation(),
                    None => {
                        location_gen.set_unit_offset(self.rng.gen_range(0_f64..1.));
                        location_gen.translation()
                    }
                };

                mob.map_mut(|s, o| {
                    s.initialize(o, start_pos, target_pos, &mut self.rng);
                }).unwrap();

                //  Connect the signal emitted when mob dies to score tracker
                mob.base().connect("squashed", persistent.base(), "_on_mob_squashed", VariantArray::new().into_shared(), 0).unwrap();

                owner.add_child(mob, false)
            }
        }
    }

    #[export]
    fn _on_player_hit(&self, owner: TRef<Node>) {
        get_node::<Node, Timer>(owner, "SpawnInterval").unwrap().set_paused(true);
    }
}
