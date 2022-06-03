use gdextras::{get_instance, get_node};
use gdnative::prelude::*;
use gdnative::api::LineEdit;
use crate::scene::persistent::Persistent;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct StartMenu;

#[methods]
impl StartMenu {
    fn new(_:TRef<Node>) -> Self {Self}

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        get_node::<Node, LineEdit>(owner, "UI/NameInput").unwrap().grab_focus();
    }

    #[export]
    fn _input(&self, owner: TRef<Node>, event: Ref<InputEvent>) {
        let event = unsafe {event.assume_safe()};


        if event.is_action_pressed("ui_accept", false, false)
        {

            let name_input: TRef<LineEdit> = get_node(owner.clone(), "UI/NameInput").unwrap();

            if name_input.text() == GodotString::new() {
                get_node::<Node, Label>(owner.clone(), "UI/Message").unwrap()
                    .set_text("You must have a name!")
            }
            else {

                let persistent = get_instance::<Node, Node, Persistent>(owner.clone(), "/root/Persistent").unwrap();
                persistent.map_mut(|s, _|
                    s.name = Some(name_input.text().to_string())
                ).unwrap();

                unsafe {
                    owner.get_tree().unwrap().assume_safe()
                        .change_scene("scenes/Main.tscn").unwrap()
                }
            }

        }
    }
}
