use std::sync::atomic::{AtomicUsize, Ordering};

use ra_salsa::InternValueTrivial;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HotPotato(u32);

static N_POTATOES: AtomicUsize = AtomicUsize::new(0);

impl HotPotato {
    fn new(id: u32) -> HotPotato {
        N_POTATOES.fetch_add(1, Ordering::SeqCst);
        HotPotato(id)
    }
}

impl Drop for HotPotato {
    fn drop(&mut self) {
        N_POTATOES.fetch_sub(1, Ordering::SeqCst);
    }
}

impl InternValueTrivial for HotPotato {}

#[ra_salsa::query_group(InternQueryGroupStorage)]
trait InternQueryGroup: ra_salsa::Database {
    #[ra_salsa::interned]
    fn intern_potato(&self, tater: HotPotato) -> ra_salsa::InternId;

    #[ra_salsa::lru]
    fn get_interned(&self, id: ra_salsa::InternId) -> HotPotato;

    fn get_interned_no_lru(&self, id: ra_salsa::InternId) -> HotPotato;
}

fn get_interned(db: &dyn InternQueryGroup, id: ra_salsa::InternId) -> HotPotato {
    db.lookup_intern_potato(id)
}

fn get_interned_no_lru(db: &dyn InternQueryGroup, id: ra_salsa::InternId) -> HotPotato {
    db.lookup_intern_potato(id)
}

#[ra_salsa::query_group(InputQueryGroupStorage)]
trait InputQueryGroup: ra_salsa::Database {
    #[ra_salsa::input]
    fn potato(&self, id: u32) -> HotPotato;

    #[ra_salsa::lru]
    fn get(&self, id: u32) -> HotPotato;

    fn get_no_lru(&self, id: u32) -> HotPotato;
}

fn get(db: &dyn InputQueryGroup, id: u32) -> HotPotato {
    db.get(id)
}

fn get_no_lru(db: &dyn InputQueryGroup, id: u32) -> HotPotato {
    db.get(id)
}

#[ra_salsa::database(InternQueryGroupStorage, InputQueryGroupStorage)]
#[derive(Default)]
struct Database {
    storage: ra_salsa::Storage<Self>,
}

impl ra_salsa::Database for Database {}

#[test]
fn intern_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut db = Database::default();
    GetInternedQuery.in_db_mut(&mut db).set_lru_capacity(32);
    assert_eq!(N_POTATOES.load(Ordering::SeqCst), 0);

    for i in 0..32 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned(id);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before LRU: {}", &stats.curr_bytes);

    for i in 32..256 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned(id);
        assert_eq!(p.0, i);
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after LRU: {}", &stats.curr_bytes);
}

#[test]
fn intern_no_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let db = Database::default();
    assert_eq!(N_POTATOES.load(Ordering::SeqCst), 0);

    for i in 0..32 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned_no_lru(id);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 32 values: {}", &stats.curr_bytes);

    for i in 32..256 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned_no_lru(id);
        assert_eq!(p.0, i);
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 256 values: {}", &stats.curr_bytes);
}

#[test]
fn input_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let mut db = Database::default();
    GetQuery.in_db_mut(&mut db).set_lru_capacity(32);
    assert_eq!(N_POTATOES.load(Ordering::SeqCst), 0);

    for i in 0..32 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned(id);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before LRU: {}", &stats.curr_bytes);

    for i in 32..256 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned(id);
        assert_eq!(p.0, i);
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after LRU: {}", &stats.curr_bytes);
}

#[test]
fn input_no_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let db = Database::default();
    assert_eq!(N_POTATOES.load(Ordering::SeqCst), 0);

    for i in 0..32 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned_no_lru(id);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 32 values: {}", &stats.curr_bytes);

    for i in 32..256 {
        let id = db.intern_potato(HotPotato::new(i));
        let p = db.get_interned_no_lru(id);
        assert_eq!(p.0, i);
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 256 values: {}", &stats.curr_bytes);
}
