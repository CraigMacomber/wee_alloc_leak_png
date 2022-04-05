This test code, in a loop, allocates then frees some memory.
Using the default Rust allocator the memory is stable, using wee_alloc, it leaks.
Note that it leaks memory very fast (several GB per second), so you might not want to run it very long.