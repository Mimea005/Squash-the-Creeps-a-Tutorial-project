use gdnative::prelude::*;
use gdextras::*;

fn lib_init(register: InitHandle) {

    gd_print!(p, "Registered lib")
}

godot_init!(lib_init);



#[cfg(test)]
mod tests {

}
