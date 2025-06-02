# Concept: Arc<T> — Atomically Reference Counted Smart Pointer
Arc<T> stands for Atomic Reference Counted. It's used when you want to share ownership of data across multiple threads safely. It's like Rc<T>, but safe to use in concurrent environments.

Rc<T> is for single-threaded scenarios.

Arc<T> is for multi-threaded scenarios.

When the last Arc pointing to the data goes out of scope, the data is dropped.
