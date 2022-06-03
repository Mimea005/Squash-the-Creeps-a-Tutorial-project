use gdnative::prelude::*;

mod scene;
mod entity;
mod ui;

fn lib_init(register: InitHandle) {

    register.add_class::<entity::player::Player>();
    register.add_class::<scene::main_scene::Main>();
    register.add_class::<entity::mob::Mob>();
    register.add_class::<entity::mob::MobSpawn>();
    register.add_class::<ui::gameplay::UI>();
    register.add_class::<scene::persistent::Persistent>();
    register.add_class::<scene::startmenu::StartMenu>();
    register.add_class::<scene::persistent::ScoreStore>();
}

godot_init!(lib_init);



#[cfg(test)]
mod tests {

}
