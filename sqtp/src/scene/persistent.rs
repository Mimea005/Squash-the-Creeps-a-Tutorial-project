use gdextras::*;
use gdnative::prelude::*;
use gdnative::api::OS;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Persistent {
    pub score: i32,
    pub name: Option<String>
}

impl Persistent {
    fn new(_owner: TRef<Node>) -> Self {
        Self {
            score: 0,
            name: None
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.signal("score_update").done();
    }
}

#[methods]
impl Persistent {

    #[export]
    fn _on_mob_squashed(&mut self, owner: TRef<Node>) {
        self.score += 1;

        owner.emit_signal("score_update", &[Variant::new(self.score)]);
    }

    #[export]
    fn _on_player_hit(&mut self, owner: TRef<Node>) {

        let score_storage = get_instance::<Node, Node, ScoreStore>(owner.clone(), "ScoreStorage").unwrap();
        score_storage.map_mut(|s,_| {
            let name = match &self.name {
                None => "Unknown".to_string(),
                Some(name) => name.clone()
            };

            s.add_score(
                Score {
                    name,
                    score: self.score
                }
            )
        }).unwrap();

        self.score = 0;
    }

    #[export]
    fn _unhandled_input(&self, owner: TRef<Node>, event: Ref<InputEvent>) {
        let event = unsafe {event.assume_safe()};

        if event.is_action_pressed("program_exit", false, false) {
            unsafe {
                owner.get_tree().unwrap().assume_safe().quit(0);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Score {
    pub name: String,
    pub score: i32
}

use std::fs::File;
use std::io::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ScoreStore {
    scores: Vec<Score>,
}

impl ScoreStore {
    fn new(_owner: TRef<Node>) -> Self {
        Self { scores: Vec::new()}
    }

    fn load_scores() -> Vec<Score> {

        let score_storage_path = OS::godot_singleton().get_user_data_dir().to_string() + "/scores.csv";

        let mut file = match File::open(score_storage_path.clone()) {
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        //  Score file does not exist, create a new one
                        File::create(score_storage_path).unwrap()
                    },
                    err => panic!("{:?}", err)
                }
            },

            Ok(file) => {
                file
            }
        };

        let mut data = vec!();

        match file.read_to_end(&mut data) {
            Err(e) => gd_print!(w, "Created new score.csv -> {:?}", e),
            Ok(_) => {}
        }

        let scores = String::from_utf8(data).unwrap();
        let scores_array = scores.split("\n");

        let mut scores = Vec::new();


        for line in scores_array {
            let line = line.split(",").collect::<Vec<&str>>();
            if line.len() == 2 {
                scores.push(Score {
                    name: line.get(0).unwrap().to_string(),
                    score: line.get(1).unwrap().parse().unwrap()
                })
            }
        }

        scores
    }

    fn save_scores(scores: &Vec<Score>) -> Result<()> {

        let mut data = vec![];

        for score in scores {
            data.append(&mut score.name.as_bytes().to_vec());
            data.append(&mut ",".as_bytes().to_vec());
            data.append(&mut score.score.to_string().as_bytes().to_vec());

            data.append( &mut "\n".as_bytes().to_vec());
        }

        let scores_path = OS::godot_singleton().get_user_data_dir().to_string() + "/scores.csv";

        File::create(scores_path)?.write_all(data.as_slice())?;

        Ok(())
    }

    pub fn add_score(&mut self, score: Score) {
        self.scores.push(score);
    }

    pub fn get_scores<'a>(&self) -> Vec<Score> {
        self.scores.clone()
    }

}

#[methods]
impl ScoreStore {

    #[export]
    fn _ready(&mut self, _owner: TRef<Node>) {
        self.scores = Self::load_scores()
    }

    #[export]
    fn _exit_tree(&self, _owner: TRef<Node>) {
        Self::save_scores(&self.scores).unwrap();
    }
}
