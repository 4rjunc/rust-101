enum Color{
    Red,
    Green,
    Yellow,
}

fn main() {
    let mut signal_color = Color::Green;
    println!("signal_color: Green");
    signal_means(signal_color);

    signal_color = Color::Red;
    println!("signal_color: Red");
    signal_means(signal_color);

    signal_color = Color::Yellow;
    println!("signal_color: Yellow");
    signal_means(signal_color);

}


fn signal_means(color: Color){
    match color {
        Color::Red => println!("Stop!"),
        Color::Green => println!("Go!"),
        Color::Yellow => println!("Wait!")
    }
}
