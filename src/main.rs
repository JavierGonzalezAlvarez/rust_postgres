use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
use std::process;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn insert(name: &String) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://test:2525_ap@localhost:5432/apirust", NoTls)?;
    println!("name: {}", name);
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Ana"), "India");
    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }
    Ok(())
}

fn select_name(name: &String) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://test:2525_ap@localhost:5432/apirust", NoTls)?;
    //println!("name: {}", name);
    //we set $1 for first value, and so on
    for row in client.query(
        "SELECT id, name, country FROM author WHERE name = $1",
        &[&name],
    )? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!(
            "Select_Name => Author {} is from {}",
            author.name, author.country
        );
    }
    Ok(())
}

fn create() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://test:2525_ap@localhost:5432/apirust", NoTls)?;
    //create table if not exists
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    )?;
    Ok(())
}

fn select_all() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://test:2525_ap@localhost:5432/apirust", NoTls)?;

    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }
    Ok(())
    // function returns "()"" and "Error"
}

struct DataStruct {
    author: String,
    count: i64,
}

fn select_count_fields() -> Result<(), Error> {
    let mut database_url =
        Client::connect("postgresql://test:2525_ap@localhost:5432/apirust", NoTls)?;

    for row in database_url.query(
        "SELECT name, COUNT(author) AS count FROM author GROUP BY name ORDER BY count DESC",
        &[],
    )? {
        //same result: another way
        //let author: Option<String> = row.get(0);
        //let count: i64 = row.get(1);
        //println!("found person: {:?} {}", author, count);
        let (author, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));
        if author.is_some() && count.is_some() {
            let data = DataStruct {
                author: author.unwrap(),
                count: count.unwrap(),
            };
            println!("Author => {} has counted => {}", data.author, data.count);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = create() {
        println!("Error: {:?}", err);
        process::exit(1);
    }
    let name = String::from("Ana");

    if let Err(err) = insert(&name) {
        println!("Error: {:?}", err);
        process::exit(1);
    }

    if let Err(err) = select_name(&name) {
        println!("Error: {:?}", err);
        process::exit(1);
    }

    if let Err(err) = select_all() {
        println!("Error: {:?}", err);
        process::exit(1);
    }

    if let Err(err) = select_count_fields() {
        println!("Error: {:?}", err);
        process::exit(1);
    }
}
