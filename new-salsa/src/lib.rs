#[salsa::jar(db = Db)]
pub struct Jar(Input, length, tracked_constant_fn);

#[salsa::input]
pub struct Input {
    #[return_ref]
    pub text: String,
}

pub trait Db: salsa::DbWithJar<Jar> {
    fn set_text(&self, text: String) -> Input;
}

#[derive(Default)]
#[salsa::db(Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

#[salsa::tracked]
fn length(db: &dyn Db, input: Input) -> usize {
    input.text(db).len()
}

impl Db for Database {
    fn set_text(&self, text: String) -> Input {
        let input = Input::new(self, text);
        input
    }
}

impl salsa::Database for Database {}

pub fn run_length(db: &mut Database, text: String) {
    let input = db.set_text(text);
    length(db, input);
}

/// benchmark that that a constant `tracked` fn (has no inputs)
/// compiles and executes successfully.
#[salsa::tracked]
fn tracked_constant_fn(_db: &dyn Db) -> u32 {
    44
}

pub fn run_constant(db: &Database) {
    tracked_constant_fn(db);
}
