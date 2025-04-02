## Why unsafe rust?
- static analysis is concervative my nature: rust will follows its own rules to validate a program is memory safe or not. even if a developer know a code is memory safe but rust rejects it because of its rules
- if underlying computer hardware is inherently unsafe: so if you want to code in that device you should break the rules of rust

## use `unsafe` keyword
```rust
unsafe{
    //code
}
```

- to dereference raw pointer
- call an unsafe function or method
- access or modify a mutable static variable
- to implement a unsafe trait
- access fields of unions

* `unsafe` does't turn off borrow checker or rust saftey check 

