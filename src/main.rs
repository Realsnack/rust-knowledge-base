use diesel::{Connection, PgConnection, RunQueryDsl, QueryDsl};
use person::Person;
use postgres::{Client, NoTls};
use redis::Commands;

pub mod person;
pub mod person_diesel;
pub mod schema;

fn main() {
    let person = person::Person {
        name: "Alice".to_string(),
        age: 20,
    };

    bincode_example(&person);

    postgres_example(&person);

    redis_example(&person);

    diesel_example(&person);
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
    )
    .unwrap();

    client.batch_execute("SET search_path TO example").unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            age     INT NOT NULL
        )",
        )
        .unwrap();

    client
        .execute(
            "INSERT INTO person (name, age) VALUES ($1, $2)",
            &[&person.name, &person.age],
        )
        .unwrap();

    let person_from_db = client
        .query_one("SELECT name, age FROM person WHERE id = 1", &[])
        .unwrap();

    let person_from_db = Person {
        name: person_from_db.get(0),
        age: person_from_db.get(1),
    };

    println!("person from db: {:?}", person_from_db);
}

fn redis_example(person: &Person) {
    let client = redis::Client::open("redis://default:redispw@localhost:32768").unwrap();
    let mut con = client.get_connection().unwrap();

    let _: () = con
        .set("person", bincode::serialize(person).unwrap())
        .unwrap();

    let person_from_redis: Person =
        bincode::deserialize(&con.get::<_, Vec<u8>>("person").unwrap()).unwrap();

    println!("person from redis: {:?}", person_from_redis)
}

fn diesel_example(person: &Person) {
    let database_url = "postgresql://rust_app:karel123@centos01.vs.msvacina.cz/rust";

    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let person_diesel = person_diesel::PersonDiesel {
        name: person.name.clone(),
        age: person.age,
    };

    diesel::insert_into(schema::person_diesel::table)
        .values(&person_diesel)
        .execute(&mut connection)
        .unwrap();

    let person_diesel_from_db = schema::person_diesel::table
        .select((schema::person_diesel::name, schema::person_diesel::age))
        .first::<person_diesel::PersonDiesel>(&mut connection)
        .unwrap();

    println!("person from db: {:?}", person_diesel_from_db);
}
