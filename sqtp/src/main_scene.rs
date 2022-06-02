use gdextras::get_node;
use gdnative::{
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main;

impl Main {

    pub fn new(_owner: TRef<Node>) -> Self {
        Main
    }
}

#[methods]
impl Main {

    #[export]
    fn _unhandled_input(&self, owner: TRef<Node>, event: Ref<InputEvent>) {
        let event = unsafe {event.assume_safe()};
        if event.is_action_pressed("ui_accept", false, false)
            && get_node::<Node, ColorRect>(owner.clone(), "UI/Retry").unwrap().is_visible() {
                let scene = owner.get_tree().unwrap();
                unsafe {
                    scene.assume_safe().reload_current_scene().unwrap()
                }
        }

        else if event.is_action_pressed("program_exit", false, false) {
            unsafe {
                owner.get_tree().unwrap().assume_safe()
                    .quit(0);
            }
        }
    }
}
