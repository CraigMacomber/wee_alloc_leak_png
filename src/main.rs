extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn leak_test() {
    // This leaks when using wee_alloc
    let a = Box::new([0; 85196]);
    let b = Box::new([0; 80000]);
    drop(a);
    drop(b);
}

fn main() {
    loop {
        leak_test();
    }
}
