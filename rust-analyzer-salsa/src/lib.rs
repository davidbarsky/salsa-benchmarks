use std::usize;

#[ra_salsa::query_group(HelloWorldStorage)]
trait HelloWorld {
    #[ra_salsa::input]
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

#[ra_salsa::database(HelloWorldStorage)]
#[derive(Default)]
pub struct TestDatabase {
    storage: ra_salsa::Storage<Self>,
}

impl ra_salsa::Database for TestDatabase {}

pub fn run_string_length(db: &mut TestDatabase, text: String) {
    db.set_input_string((), text);
    db.length(());
}

pub fn run_constant(db: &mut TestDatabase) {
    db.constant(());
}
