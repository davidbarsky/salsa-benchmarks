use std::usize;

#[salsa::query_group(HelloWorldStorage)]
trait HelloWorld {
    #[salsa::input]
    fn input_string(&self, key: ()) -> String;

    fn length(&self, key: ()) -> usize;

    fn constant(&self, key: ()) -> usize;
}

fn length(db: &dyn HelloWorld, (): ()) -> usize {
    let input_string = db.input_string(());

    input_string.len()
}

fn constant(_db: &dyn HelloWorld, (): ()) -> usize {
    44
}

#[salsa::database(HelloWorldStorage)]
#[derive(Default)]
pub struct TestDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for TestDatabase {}

pub fn run_string_length(db: &mut TestDatabase, text: String) {
    db.set_input_string((), text);
    db.length(());
}

pub fn run_constant(db: &mut TestDatabase) {
    db.constant(());
}
