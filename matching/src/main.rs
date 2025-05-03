enum Direction {
    North,
    South,
    East,
    West,
}

fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("You are heading up!"),
        Direction::South => println!("Going down south."),
        Direction::East => println!("To the right!"),
        Direction::West => println!("To the left!"),
    }
}

fn main() {
    let my_direction = Direction::East;
    describe_direction(my_direction);
}
