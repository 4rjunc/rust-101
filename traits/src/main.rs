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
