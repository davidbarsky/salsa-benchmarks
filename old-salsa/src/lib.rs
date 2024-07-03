#[salsa::query_group(HelloWorldStorage)]
trait HelloWorld {
    #[salsa::input]
    fn input_string(&self, key: ()) -> String;

    fn length(&self, key: ()) -> usize;
}

fn length(db: &dyn HelloWorld, (): ()) -> usize {
    let input_string = db.input_string(());

    input_string.len()
}

#[salsa::database(HelloWorldStorage)]
#[derive(Default)]
pub struct TextDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for TextDatabase {}

pub fn run(db: &mut TextDatabase) {
    db.set_input_string(
        (),
        "/Users/dbarsky/Developer/salsa-benchmarks/src/lib.rs".to_string(),
    );
    db.length(());
}
