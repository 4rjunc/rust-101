Rust program demonstrating the use of Rc<T> — which stands for Reference Counted pointer. Rc<T> is used for shared ownership in single-threaded scenarios.

✅ Concept:
You use Rc<T> when multiple parts of your program need read-only access to the same data, and that data lives as long as there are references to it.

It keeps a reference count: when the last reference goes out of scope, the data is dropped.
