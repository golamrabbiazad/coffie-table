use rusqlite::{Connection, Error, Result};

#[derive(Debug, Clone)]
struct Coffie {
    name: String,
    description: String,
    count: i32,
}

trait CoffieProps {
    fn print_coffie(self) -> ();
}

impl Coffie {
    fn new(name: String, description: String) -> Coffie {
        Coffie {
            name,
            description,
            count: 1,
        }
    }
}

impl CoffieProps for Coffie {
    fn print_coffie(self) -> () {
        println!(
            "Coffie Name: {}, Coffie Description: {}, Coffie Count: {}!",
            self.name, self.description, self.count
        );
    }
}

fn main() -> Result<(), Error> {
    let first_coffie = Coffie::new("Black Coffie".to_string(), "Awesome".to_string());

    first_coffie.clone().print_coffie();

    let conn = Connection::open("coffie.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS coffie (
        id integer primary key,
        name text not null unique,
        description text,
        count integer
    )",
        [],
    )?;

    conn.execute(
        "INSERT INTO coffie (name, description, count) VALUES (?1, ?2, ?3)",
        [
            first_coffie.name,
            first_coffie.description,
            first_coffie.count.to_string(),
        ],
    )?;

    Ok(())
}
