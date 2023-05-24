pub mod person;

fn main() {
    bincode_example();
}


fn bincode_example() {
    let person = person::Person {
        name: "Alice".to_string(),
        age: 20,
    };

    let encoded: Vec<u8> = bincode::serialize(&person).unwrap();
    println!("encoded: {:?}", encoded);

    let decoded: person::Person = bincode::deserialize(&encoded[..]).unwrap();
    println!("decoded: {:?}", decoded);
}