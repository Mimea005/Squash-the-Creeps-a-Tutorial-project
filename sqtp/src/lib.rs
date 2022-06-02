use gdnative::prelude::*;

mod player;
mod mob;
mod main_scene;
mod ui;

fn lib_init(register: InitHandle) {

    register.add_class::<player::Player>();
    register.add_class::<mob::Mob>();
    register.add_class::<mob::MobSpawn>();
    register.add_class::<main_scene::Main>();
    register.add_class::<ui::UI>();
}

godot_init!(lib_init);



#[cfg(test)]
mod tests {

}
