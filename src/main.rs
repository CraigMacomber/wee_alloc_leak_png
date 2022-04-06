use std::alloc::GlobalAlloc;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    static WEE: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    loop {
        // This leaks when using wee_alloc

        // let a = Box::new([0; 85196]);
        // let b = Box::new([0; 80000]);
        // drop(a);
        // drop(b);

        // Produce leak using allocation API directly:
        // These are magic sizes for when size catagories are enabled. They leak without size catagories as well, but differently.
        // If either of these sizes is decreased, it no longer behaves the same (no longer gets a free list hit for the first, and a miss for the second).
        // let l1 = core::alloc::Layout::from_size_align(65536 + 4081, 1).unwrap();
        // let l2 = core::alloc::Layout::from_size_align(65536 - 31, 1).unwrap();
        // unsafe {
        //     // Allocate first block: this one gets reused from the free list,
        //     // but also causes the head of the free list to be set to null when it should still have space in it.
        //     let x1 = WEE.alloc(l1);
        //     // Allocate second block: this one does not reuse space from the free list (which seems like a bug).
        //     // At this point wee_alloc's free list's head (as used in alloc_first_fit) is null.
        //     // This results in a new allocation.
        //     let x2 = WEE.alloc(l2);
        //     println!("{x1:p}, {x2:p}");
        //     WEE.dealloc(x1, l1);
        //     WEE.dealloc(x2, l2);
        // }

        // These are magic sizes for when size catagories are disabled.
        // If either of these sizes is decreased, it no longer behaves the same (no longer gets new allocations for both every time).
        let l1 = core::alloc::Layout::from_size_align(2033, 1).unwrap(); // Rounds up to 255 words.
        let l2 = core::alloc::Layout::from_size_align(2025, 1).unwrap(); // Rounds up to 254 words.
        unsafe {
            // Allocate first block: this one gets space that was use by the second allocation on the last iteration, but starting at an address 8 bytes lower.
            let x1 = WEE.alloc(l1);
            // Allocate second block: this one does not reuse space from the free list (which seems like a bug).
            // At this point wee_alloc's free list seems to only contain one item which does not fit.
            let x2 = WEE.alloc(l2);
            println!("{x1:p}, {x2:p}");
            WEE.dealloc(x1, l1);
            WEE.dealloc(x2, l2);
        }
    }
}
