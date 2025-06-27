use borsh::{to_vec, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Create(String),
    Delete,
}

fn main() {
    let original = Instruction::Create("Hello Rust!".to_string());
    let data = to_vec(&original);
    let decoded = Instruction::try_from_slice(&data).unwrap();

    println!("Decoded instruction: {:?}", decoded);
}
