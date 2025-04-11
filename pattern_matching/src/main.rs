// Enum is a way to define a type which can be one of several variants
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Implementing behavior (methods) for the enum
impl TrafficLight {
    fn action(&self) -> &str {
        // Pattern matching on the enum variant
        match self {
            TrafficLight::Red => "STOP 🛑",
            TrafficLight::Yellow => "SLOW DOWN ⚠️",
            TrafficLight::Green => "GO 🟢",
        }
    }
}

fn main() {
    // Creating instances of enum variants
    let light1 = TrafficLight::Red;
    let light2 = TrafficLight::Green;
    let light3 = TrafficLight::Yellow;

    // Calling the method based on the variant
    println!("Red light: {}", light1.action());
    println!("Green light: {}", light2.action());
    println!("Yellow light: {}", light3.action());
}
