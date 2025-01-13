use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug, PartialEq, Eq)]
struct HotPotato(u32);

thread_local! {
    static N_POTATOES: AtomicUsize = const { AtomicUsize::new(0) }
}

impl HotPotato {
    fn new(id: u32) -> HotPotato {
        N_POTATOES.with(|n| n.fetch_add(1, Ordering::SeqCst));
        HotPotato(id)
    }
}

impl Drop for HotPotato {
    fn drop(&mut self) {
        N_POTATOES.with(|n| n.fetch_sub(1, Ordering::SeqCst));
    }
}

#[salsa::input]
struct MyInput {
    field: u32,
}

#[salsa::tracked]
fn get_hot_potato_no_lru(db: &dyn salsa::Database, input: MyInput) -> Arc<HotPotato> {
    Arc::new(HotPotato::new(input.field(db)))
}

#[salsa::tracked(lru = 32)]
fn get_hot_potato(db: &dyn salsa::Database, input: MyInput) -> Arc<HotPotato> {
    Arc::new(HotPotato::new(input.field(db)))
}

fn load_n_potatoes() -> usize {
    N_POTATOES.with(|n| n.load(Ordering::SeqCst))
}

#[salsa::interned]
struct Interned<'db> {
    field: u32,
}

#[salsa::tracked]
fn get_hot_potato_interned_no_lru<'db>(
    db: &'db dyn salsa::Database,
    input: Interned<'db>,
) -> Arc<HotPotato> {
    Arc::new(HotPotato::new(input.field(db)))
}

#[salsa::tracked(lru = 32)]
fn get_hot_potato_interned<'db>(
    db: &'db dyn salsa::Database,
    input: Interned<'db>,
) -> Arc<HotPotato> {
    Arc::new(HotPotato::new(input.field(db)))
}

#[test]
fn input_no_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let db = salsa::DatabaseImpl::new();
    assert_eq!(load_n_potatoes(), 0);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before before starting: {}", &stats.curr_bytes);

    for i in 0..32 {
        let input = MyInput::new(&db, i);
        let p = get_hot_potato_no_lru(&db, input);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 32 inputs: {}", &stats.curr_bytes);

    // allocate some more
    for i in 32..256 {
        let input = MyInput::new(&db, i);
        let p = get_hot_potato_no_lru(&db, input);
        assert_eq!(p.0, i)
    }

    assert_eq!(load_n_potatoes(), 256);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 256 inputs: {}", &stats.curr_bytes);
}

#[test]
fn input_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let db = salsa::DatabaseImpl::new();
    assert_eq!(load_n_potatoes(), 0);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before before starting: {}", &stats.curr_bytes);

    for i in 0..32 {
        let input = MyInput::new(&db, i);
        let p = get_hot_potato(&db, input);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 32 inputs; before LRU: {}", &stats.curr_bytes);

    // make sure these are LRU'd.
    for i in 32..256 {
        let input = MyInput::new(&db, i);
        let p = get_hot_potato(&db, input);
        assert_eq!(p.0, i)
    }
    assert_eq!(load_n_potatoes(), 32);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 256 inputs; after LRU: {}", &stats.curr_bytes);
}

#[test]
fn intern_no_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let db = salsa::DatabaseImpl::new();
    assert_eq!(load_n_potatoes(), 0);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before before starting: {}", &stats.curr_bytes);

    for i in 0..32 {
        let input = Interned::new(&db, i);
        let p = get_hot_potato_interned_no_lru(&db, input);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 32 inputs: {}", &stats.curr_bytes);

    // allocate some more
    for i in 32..256 {
        let input = Interned::new(&db, i);
        let p = get_hot_potato_interned_no_lru(&db, input);
        assert_eq!(p.0, i)
    }

    assert_eq!(load_n_potatoes(), 256);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated after 256 inputs: {}", &stats.curr_bytes);
}

#[test]
fn intern_lru() {
    let _profiler = dhat::Profiler::builder().testing().build();
    let db = salsa::DatabaseImpl::new();
    assert_eq!(load_n_potatoes(), 0);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated before before starting: {}", &stats.curr_bytes);

    for i in 0..32 {
        let input = Interned::new(&db, i);
        let p = get_hot_potato_interned(&db, input);
        assert_eq!(p.0, i)
    }

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 32 inputs; before LRU: {}", &stats.curr_bytes);

    // make sure these are LRU'd.
    for i in 32..256 {
        let input = Interned::new(&db, i);
        let p = get_hot_potato_interned(&db, input);
        assert_eq!(p.0, i)
    }
    assert_eq!(load_n_potatoes(), 32);

    let stats = dhat::HeapStats::get();
    eprintln!("Allocated at 256 inputs; after LRU: {}", &stats.curr_bytes);
}
