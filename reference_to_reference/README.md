This code demonstrates several key concepts about Rust's references and pointers:

- Basic References: When you create r1 = &num, you're creating a reference to num. This reference stores the memory address of num.
- References to References: With r2 = &r1, you're creating a reference to a reference. r2 holds the memory address of r1.
- Reference Values vs. Reference Addresses:

    - When you print r1 and r2 directly, you see the addresses they point to
    - When you print &r1 and &r2, you see the addresses where these references themselves are stored


- Memory Layout: This code helps visualize how references are stored in memory. Each reference is itself stored at some location and points to another location.
- No Raw Pointers: Note that Rust uses references (safe pointers) by default rather than raw pointers, which helps with memory safety.
- Format Specifier: The {:p} format specifier is specifically for printing pointer addresses in a platform-specific format.

