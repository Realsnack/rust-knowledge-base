use person::Person;
use postgres::{Client, NoTls};

pub mod person;

fn main() {
    let person = person::Person {
        name: "Alice".to_string(),
        age: 20,
    };

    bincode_example(&person);

    postgres_example(&person);
}

fn bincode_example(person: &Person) {
    let encoded: Vec<u8> = bincode::serialize(&person).unwrap();
    println!("encoded: {:?}", encoded);

    let decoded: person::Person = bincode::deserialize(&encoded[..]).unwrap();
    println!("decoded: {:?}", decoded);
}

fn postgres_example(person: &Person) {
    let mut client = Client::connect(
        "postgresql://rust_app:karel123@centos01.vs.msvacina.cz/rust",
        NoTls,
    ).unwrap();

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            age     INT NOT NULL
        )",
    ).unwrap();

    client.execute(
        "INSERT INTO person (name, age) VALUES ($1, $2)",
        &[&person.name, &person.age],
    ).unwrap();

    let person_from_db = client.query_one(
        "SELECT name, age FROM person WHERE id = 1",
        &[],
    ).unwrap();

    let person_from_db = Person {
        name: person_from_db.get(0),
        age: person_from_db.get(1),
    };

    println!("person from db: {:?}", person_from_db);
}
