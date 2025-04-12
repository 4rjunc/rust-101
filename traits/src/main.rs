fn main() {
    let dog = Dog;
    let cat = Cat;

    // Trait objects allow calling methods on different types through the same interface
    animal_sound(&dog);
    animal_sound(&cat);
}
