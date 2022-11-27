use std::collections::HashMap;

fn main() {
    // ----- Obtain key/value arguments -----
    let mut arguments = std::env::args().skip(1); // Skip first element of iterator type

    // "unwrap" means to guarantee the return, else panic
    let key = arguments.next().unwrap();

    // "expect" is another way to do this and pass in a message
    let value = arguments.next().expect("The value was not there");

    println!("The key is: '{}', the value is: '{}'", key, value);

    // NOTE: More information about `unwrap`
    // In the case of `arguments.next()`, the return type is `Option<String>`.
    // Rust has no concept of a NULL String - A String always has valid context.
    // `Option<String>` means you may, or may not get a `String` back.
    // A quick way to deal with this, is to use `unwrap()`, which guarantees to
    // return a `String`, else there is "None", so it panics and terminates the program.
    // It effectively "unwrapped" the `Option<String>` into just a `String`.

    // ----- Create a file to store our key/value -----
    let contents = format!("{}\t{}\n", key, value);
//    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database { // "impl" is a way to add functionality to a type
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
//        let contents = match std::fs::read_to_string("kv.db") {
//            Ok(c) => c,
//            Err(error) => {
//                return Err(error);
//            }
//        };
        let contents = std::fs::read_to_string("kv.db")?; // <- Equivalent to commented out block

        // parse the string
        // populate the map
        Ok(Database {
            map: HashMap::new()
        })
    }
}

