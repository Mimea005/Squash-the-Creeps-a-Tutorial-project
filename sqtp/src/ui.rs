use gdextras::{gd_print, get_node};
use gdnative::prelude::*;

type Base = Control;

#[derive(NativeClass)]
#[inherit(Base)]
pub struct UI {
    pub score: i32
}

impl UI {

    fn new(_owner: TRef<Base>) -> Self {
        Self {
            score: 0
        }
    }
}

#[methods]
impl UI {

    #[export]
    fn _ready(&self, owner: TRef<Base>) {
        get_node::<Base, ColorRect>(owner, "Retry").unwrap().hide()
    }

    #[export]
    fn _on_mob_squashed(&mut self, owner: TRef<Base>) {
        self.score +=1;
        get_node::<Base, Label>(owner, "ScoreLabel").unwrap().set_text(format!("Score: {:}", self.score));
    }

    #[export]
    fn _on_player_hit(&self, owner: TRef<Base>) {
        get_node::<Base, ColorRect>(owner, "Retry").unwrap().show();
        gd_print!(p, "Score: {:}", self.score)
    }
}
