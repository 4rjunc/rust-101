use std::io::{self, Write};

enum Commands {
    Say(String),
    Add(i32, i32),
    Multiply(i32, i32),
    Subtract(i32, i32),
    Quit
}

fn parse_command(input: &str) -> Result<Commands, String>{
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty(){
        return Err("No Commands Entered".to_string());
    }

    match parts[0] {
        "say" => {
            if parts.len() < 2 {
                return  Err("The 'say' command requires a message".to_string());
            }
            Ok(Commands::Say(parts[1..].join(" "))) },
        "add" => {
            if parts.len() != 3 {
                return  Err("The 'add' command requires exactly 2 numbers".to_string());
            } else { 
            let x = parts[1].parse::<i32>().unwrap();
            let y = parts[2].parse::<i32>().unwrap();
            Ok(Commands::Add(x, y))
            }
        },
        "multiply" => {
            if parts.len() != 3 {
                return  Err("The 'multiply' command requires exactly 2 numbers".to_string());
            } else {
            let x = parts[1].parse::<i32>().unwrap();
            let y = parts[2].parse::<i32>().unwrap();
            Ok(Commands::Multiply(x, y))
            }
        },
        "subtract" => {
            if parts.len() != 3 {
                return  Err("The 'subtract' command requires exactly 2 numbers".to_string());
            } else {
            let x = parts[1].parse::<i32>().unwrap();
            let y = parts[2].parse::<i32>().unwrap();
            Ok(Commands::Subtract(x, y))
            }
        }
        "quit" => Ok(Commands::Quit),
        _ => Err(format!("Unknown command: '{}'", parts[0])),
    }
}

fn execute_command(command: Commands){
    match command {
        Commands::Say(message) => println!("Say: {}", message),
        Commands::Add(num1, num2 ) => println!("Add: {}", num1 + num2),
        Commands::Multiply(num1, num2) => println!("Multiply: {}", num1 * num2),
        Commands::Subtract(num1, num2) => println!("Subtract: {}", num1 - num2),
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

        // Try parsing the input and handle errors
        match parse_command(&input) {
           Ok(command) => execute_command(command),
            Err(err) => println!("Error: {}", err),
       }
    }
}

