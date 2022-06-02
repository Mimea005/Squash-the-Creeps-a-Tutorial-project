use gdnative::prelude::*;
use gdextras::*;

mod player;

fn lib_init(register: InitHandle) {

    register.add_class::<player::Player>();

    gd_print!(p, "Registered lib")
}

godot_init!(lib_init);



#[cfg(test)]
mod tests {

}
