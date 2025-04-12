// Define a trait: a collection of method signatures
trait Animal {
    fn speak(&self) -> &str;
}

// Struct that implements the Animal trait
struct Dog;
struct Cat;

// Implement the trait for Dog
impl Animal for Dog {
    fn speak(&self) -> &str {
        "Woof 🐶"
    }
}

// Implement the trait for Cat
impl Animal for Cat {
    fn speak(&self) -> &str {
        "Meow 🐱"
    }
}

// A function that accepts a trait object
fn animal_sound(animal: &dyn Animal) {
    println!("The animal says: {}", animal.speak());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // Trait objects allow calling methods on different types through the same interface
    animal_sound(&dog);
    animal_sound(&cat);
}
