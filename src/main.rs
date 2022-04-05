use std::alloc::GlobalAlloc;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    loop {
        // This leaks when using wee_alloc

        // let a = Box::new([0; 85196]);
        // let b = Box::new([0; 80000]);
        // drop(a);
        // drop(b);

        // Produce leak using allocation API directly:
        let l1 = core::alloc::Layout::from_size_align(85196, 1).unwrap();
        let l2 = core::alloc::Layout::from_size_align(80000, 1).unwrap();
        unsafe {
            // Allocate first block: this one gets reused from the free list,
            // but also causes the head of the free list to be set to null when it should still have space in it.
            let x1 = ALLOC.alloc(l1);
            // Allocate second block: this one does not reuse space from the free list (which seems like a bug).
            // At this point wee_alloc's free list's head (as used in alloc_first_fit) is null.
            // This results in a new allocation.
            let x2 = ALLOC.alloc(l2);
            println!("{x1:p}, {x2:p}");
            ALLOC.dealloc(x1, l1);
            ALLOC.dealloc(x2, l2);
        }
    }
}
