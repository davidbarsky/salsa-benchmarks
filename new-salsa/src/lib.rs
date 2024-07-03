#[salsa::jar(db = Db)]
pub struct Jar(Input, length);

#[salsa::input]
pub struct Input {
    pub text: String,
}

pub trait Db: salsa::DbWithJar<Jar> {
    fn set_text(&self, text: String) -> Input;
}

#[derive(Default)]
#[salsa::db(Jar)]
pub struct TextDatabase {
    storage: salsa::Storage<Self>,
}

#[salsa::tracked]
fn length(db: &dyn Db, input: Input) -> usize {
    input.text(db).len()
}

impl Db for TextDatabase {
    fn set_text(&self, text: String) -> Input {
        let input = Input::new(self, text);
        input
    }
}

impl salsa::Database for TextDatabase {
    // fn salsa_event(&self, event: salsa::Event) {}
}

pub fn run(db: &mut TextDatabase) {
    let input = db.set_text("/Users/dbarsky/Developer/salsa-benchmarks/src/lib.rs".to_string());
    length(db, input);
}
