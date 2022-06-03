use gdextras::*;
use gdnative::prelude::*;
use gdnative::api::{
    ResourcePreloader,
    Theme
};
use crate::scene::persistent::{Persistent, ScoreStore};

type Base = Control;

#[derive(NativeClass)]
#[inherit(Base)]
pub struct UI;

impl UI {

    fn new(_owner: TRef<Base>) -> Self {
        Self
    }

    fn build_scoreboard(&self, owner: TRef<Base>) {

        let mut scores = get_instance::<Base, Node, ScoreStore>(owner.clone(), "/root/Persistent/ScoreStorage").unwrap()
            .map(|s,_|s.get_scores()).unwrap();
        scores.sort_by(|a,b|
            b.score.cmp(&a.score)
        );

        while scores.len() > 5 {
            scores.pop();
        }


        let theme = get_node::<Base, ResourcePreloader>(owner.clone(), "ResourcePreloader").unwrap()
            .get_resource("theme").unwrap()
            .cast::<Theme>().unwrap();

        let scoreboard: TRef<Control> = get_node(owner.clone(), "Retry/Dialog/List").unwrap();

        for score in scores {

            let label = Label::new();
            label.set_theme(theme.clone());

            label.set_text(format!("{:}: {:}", score.name, score.score));
            label.set_align(1);

            scoreboard.add_child(label, true)
        }
    }
}

#[methods]
impl UI {

    #[export]
    fn _ready(&self, owner: TRef<Base>) {

        let persistent = get_instance::<Base, Node, Persistent>(owner.clone(), "/root/Persistent").unwrap();
        persistent.base().connect("score_update", owner.clone(), "_on_score_update", VariantArray::new().into_shared(), 0).unwrap();

        let theme = ResourceLoader::godot_singleton().load("default theme.theme", "", false).unwrap();
        get_node::<Base, ResourcePreloader>(owner.clone(), "ResourcePreloader").unwrap().add_resource("theme", theme);

        get_node::<Base, ColorRect>(owner, "Retry").unwrap().hide()
    }

    #[export]
    fn _input(&self, owner: TRef<Base>, event: Ref<InputEvent>) {
        let event = unsafe {event.assume_safe()};
        if event.is_action_pressed("ui_accept", false, false)
            && get_node::<Base, ColorRect>(owner.clone(), "Retry").unwrap().is_visible() {
            let scene = owner.get_tree().unwrap();
            unsafe {
                scene.assume_safe().change_scene("scenes/StartMenu.tscn").unwrap()
            }
        }
    }

    #[export]
    fn _on_score_update(&mut self, owner: TRef<Base>, score: i32) {
        get_node::<Base, Label>(owner, "ScoreLabel").unwrap().set_text(format!("Score: {:}", score));
    }

    #[export]
    fn _on_player_hit(&self, owner: TRef<Base>) {
        self.build_scoreboard(owner.clone());
        get_node::<Base, ColorRect>(owner, "Retry").unwrap().show();
    }

}
