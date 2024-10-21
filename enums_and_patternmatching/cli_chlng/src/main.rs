use std::io::{self, Write};

enum Commands {
    Say(String),
    Add(i32, i32),
    Multiply(i32, i32),
    Quit
}

fn parse_command(input: &str) -> Commands{
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts[0] {
        "say" => Commands::Say(parts[1..].join(" ")),
        "add" => {
            let x = parts[1].parse::<i32>().unwrap();
            let y = parts[2].parse::<i32>().unwrap();
            Commands::Add(x, y)
        }
        "multiply" => {
            let x = parts[1].parse::<i32>().unwrap();
            let y = parts[2].parse::<i32>().unwrap();
            Commands::Multiply(x, y)
        }
        "quit" => Commands::Quit,
        _=> panic!("Unknown Command"),
    }
}

fn execute_command(command: Commands){
    match command {
        Commands::Say(message) => println!("Say: {}", message),
        Commands::Add(num1, num2 ) => println!("Add: {}", num1 + num2),
        Commands::Multiply(num1, num2) => println!("Multiply: {}", num1 * num2),
        Commands::Quit =>  {
            println!("QUITTING");
            std::process::exit(0)
        }
    }
}

fn main() {
    println!("Hello, world!");
    loop{
        println!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(&input);
        execute_command(command);
    }

}

