#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for Database {}

#[salsa::db]
impl Db for Database {}

#[salsa::input]
pub struct Input {
    pub text: String,
}

#[salsa::tracked]
pub fn length(db: &dyn Db, input: Input) -> usize {
    input.text(db).len()
}

#[salsa::interned]
pub struct InternedInput<'db> {
    pub text: String,
}

#[salsa::tracked]
pub fn interned_length<'db>(db: &'db dyn Db, input: InternedInput<'db>) -> usize {
    input.text(db).len()
}
