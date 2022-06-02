use gdnative::prelude::*;
use gdextras::*;

mod player;
mod mob;

fn lib_init(register: InitHandle) {

    register.add_class::<player::Player>();
    register.add_class::<mob::Mob>();

    gd_print!(p, "Registered lib")
}

godot_init!(lib_init);



#[cfg(test)]
mod tests {

}
