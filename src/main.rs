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

    let mut database = Database::new().expect("Failed to create db");
    database.insert(&key, &value);
    database.insert(&key.to_uppercase(), &value);
    // Database::insert(database, key value); // Equivalent to previous line see notes about methods below.
}

struct Database {
    map: HashMap<String, String>,
}

impl Database { // "impl" is a way to add functionality to a type
    // All functions defined in an `impl` of a type are called `associated functions`.
    // `new()` is an associated function.
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
//        let contents: String = match std::fs::read_to_string("kv.db") {
//            Ok(c) => c,
//            Err(error) => {
//                return Err(error);
//            }
//        };
        let contents = std::fs::read_to_string("kv.db")?; // <- Equivalent to commented out block

        // parse the string
        let mut map = HashMap::new();

        for line in contents.lines() { // `lines` returns an iterator
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned()); // Since key/value are pointers to String owned by `contents`, we used `to_owned()` method to make a copy of the memory and bind it to this map insert.
        }

        // populate the map
        Ok(Database { map: map })
    }

    // `insert()` is an associated function, but a special kind called a `method`.
    // `Methods` have the `self` argument and let you specify the behavior that instances of your structs have. They can be called with the `dot` notation:`instance.your_method();`, instead of `turbofish` notation:`Struct::your_method(instance);`
    // Associated functions without `self` are often used as `constructors`, where the function will return a new instance of the struct. We called it `new()` only by Rust convention. It could have been any function name. `new()` does not have special meaning in Rust.
    fn insert(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn flush(self) -> std::io::Result<()> { // Returns `Unit` (== `()`) if success (like void in other languages)
        let contents = String::new();
        for pairs in self.map {
            let kvpair = format!("{}\t{}\n", pairs.0, pairs.1);
            contents.push_str(kvpair); <-- Pick up here
        }
    }
}

