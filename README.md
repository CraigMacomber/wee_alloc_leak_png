This test code, in a loop, generates a random PNG, compresses it, decompresses it, and validates it. Using the default Rust allocator the memory is stable, using wee_alloc, it leaks.
