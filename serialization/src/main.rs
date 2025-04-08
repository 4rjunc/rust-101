use borsh::{BorshDeserialize, BorshSerialize};

// Define a simple struct
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}

// Define an enum
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum Status {
    Online,
    Offline(u64), // Variant with associated data
    Idle,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Serialization example for a struct
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
    };

    let serialized_person = person.try_to_vec()?;
    println!("Serialized Person: {:?}", serialized_person);

    // Deserialization example for the struct
    let deserialized_person = Person::try_from_slice(&serialized_person)?;
    println!("Deserialized Person: {:?}", deserialized_person);
    assert_eq!(person, deserialized_person);

    // Serialization example for an enum
    let online_status = Status::Online;
    let serialized_online = online_status.try_to_vec()?;
    println!("Serialized Online Status: {:?}", serialized_online);

    let offline_status = Status::Offline(1648732800);
    let serialized_offline = offline_status.try_to_vec()?;
    println!("Serialized Offline Status: {:?}", serialized_offline);

    // Deserialization example for the enum
    let deserialized_online = Status::try_from_slice(&serialized_online)?;
    println!("Deserialized Online Status: {:?}", deserialized_online);
    assert_eq!(online_status, deserialized_online);

    let deserialized_offline = Status::try_from_slice(&serialized_offline)?;
    println!("Deserialized Offline Status: {:?}", deserialized_offline);
    assert_eq!(offline_status, deserialized_offline);

    Ok(())
}
