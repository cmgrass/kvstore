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
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database { // "impl" is a way to add functionality to a type
    // All functions defined in an `impl` of a type are called `associated functions`.
    // `new()` is an associated function.
    fn new() -> Result<Database, std::io::Error> {
        // read the kv.db file
        let contents: String = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                println!("kv.db error: {}", error);
                return Err(error);
            }
        };
//        let contents = std::fs::read_to_string("kv.db")?; // <- Equivalent to commented out block above

        // parse the string
        let mut map = HashMap::new();

        for line in contents.lines() { // `lines` returns an iterator
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned()); // Since key/value are pointers to String owned by `contents`, we used `to_owned()` method to make a copy of the memory and bind it to this map insert.
        }

        // populate the map
        //Ok(Database { map: map, flush: false})
        Ok(Database { map, flush: false}) // Because field is same name as variable, we only have to write it once. It's a cool shorthand equivalent to above.
    }

    // `insert()` is an associated function, but a special kind called a `method`.
    // `Methods` have the `self` argument and let you specify the behavior that instances of your structs have. They can be called with the `dot` notation:`instance.your_method();`, instead of `turbofish` notation:`Struct::your_method(instance);`
    // Associated functions without `self` are often used as `constructors`, where the function will return a new instance of the struct. We called it `new()` only by Rust convention. It could have been any function name. `new()` does not have special meaning in Rust.
    fn insert(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn flush(mut self) -> std::io::Result<()> { // Returns `Unit` (== `()`) if success (like void in other languages)
        self.flush = true;
        do_flush(&self)
    }
}

// Drop trait: By implementing this trait, you can adding extra functionality when an owned binding "drops" out of scope. For example - in this case - wouldn't it be cool to not need to call our `flush()` method? It just drops and flushes automatically!?
// (A "trait" is a way to specify functionality for multiple types, like an "interface" in other languages)
impl Drop for Database {
    fn drop(&mut self) { // Mandatory to define the `drop()` method when implementing the `Drop` trait.
        if !self.flush {
            let _ = do_flush(self); // The `_` says to compiler: put the Result in a variable, I don't care about it.
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {

    println!("do_flush called");

    let mut contents = String::new();
    for (key, value) in &database.map {
            //let kvpair = format!("{}\t{}\n", key, value); // More optimal to do below, no allocating String at each iteration.
            contents.push_str(&key);
            contents.push('\t');
            contents.push_str(&value);
            contents.push('\n');

    }
    std::fs::write("kv.db", contents)
}

