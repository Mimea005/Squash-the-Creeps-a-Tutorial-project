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

}
