use borsh::{BorshDeserialize, BorshSerialize};

fn main() {
    let pda_seeds: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![7, 8], vec![6]];

    println!("Original pda_seeds: {:?}", pda_seeds);

    //Serialize pda_seeds to byte array
    let serialized: Vec<Vec<u8>> = pda_seeds.to_vec();
    println!("Serialized: {:?}", serialized);

    //deserialize
    let deserialized: Vec<Vec<u8>> = Vec::<Vec<u8>>::try_from_slice(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    assert_eq!(pda_seeds, deserialized);
    println!("Serialization and deserialization successful!");
}
