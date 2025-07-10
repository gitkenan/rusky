/*
 * Import the clap crate for command-line argument parsing
 * clap makes it easy to build CLI applications with subcommands and arguments
 */
use clap::{Arg, Command};

/*
 * Import HashMap from the standard library
 * HashMap is Rust's built-in key-value data structure (like a dictionary in Python)
 */
use std::collections::HashMap;

/*
 * Define our main data structure - a struct that holds our key-value store
 * In Rust, structs are like classes in other languages but without methods by default
 */
struct KeyValueStore {
    /*
     * The 'data' field holds our actual key-value pairs
     * HashMap<String, String> means keys are Strings and values are Strings
     * String is Rust's owned string type (as opposed to &str which is a string slice)
     */
    data: HashMap<String, String>,
}

/*
 * Implementation block - this is where we define methods for our struct
 * Think of this like adding methods to a class
 */
impl KeyValueStore {
    /*
     * Constructor function - creates a new empty KeyValueStore
     * 'Self' is shorthand for KeyValueStore
     * The -> Self means this function returns an instance of KeyValueStore
     */
    fn new() -> Self {
        KeyValueStore {
            /* HashMap::new() creates an empty HashMap */
            data: HashMap::new(),
        }
    }

    /*
     * Method to set a key-value pair
     * &mut self means this method can modify the struct (mutable reference)
     * 
     * MUTABLE: In Rust, variables are immutable by default. To change them,
     * you need to explicitly mark them as 'mut' (mutable). This prevents
     * accidental modifications and makes code safer.
     * 
     * String parameters mean we take ownership of the strings passed in
     */
    fn set(&mut self, key: String, value: String) {
        /*
         * .clone() creates a copy of the String - needed because we use key/value multiple times
         * In Rust, values can only have one owner, so we clone to avoid ownership issues
         * OWNERSHIP: Rust's way of managing memory without garbage collection.
         * Each value has exactly one owner, and when the owner goes out of scope,
         * the value is automatically cleaned up.
         */
        self.data.insert(key.clone(), value.clone());
        
        /* Print confirmation to the user */
        println!("Set: {} = {}", key, value);
    }

    /*
     * Method to get a value by key
     * &self means this method only reads the struct (immutable reference)
     * 
     * &str parameter means we borrow a string slice (don't take ownership)
     * STRING SLICE (&str): A view into a string. It's like a window that shows
     * part (or all) of a string without owning it. More efficient than String
     * when you just need to read the data.
     * 
     * Option<&String> return type means we might return Some(value) or None
     * OPTION: Rust's way of handling null values safely. Instead of null,
     * we use Option<T> which is either Some(value) or None.
     */
    fn get(&self, key: &str) -> Option<&String> {
        /*
         * .get() returns an Option - either Some(value) if key exists, or None if it doesn't
         * PATTERN MATCHING: Rust's 'match' is like switch/case but much more powerful.
         * It ensures you handle all possible cases.
         */
        match self.data.get(key) {
            /* If we found the key, print it and return the value */
            Some(value) => {
                println!("Get: {} = {}", key, value);
                Some(value)
            }
            /* If key doesn't exist, print error message and return None */
            None => {
                println!("Key '{}' not found", key);
                None
            }
        }
    }

    /*
     * Method to delete a key-value pair
     * Returns bool to indicate success/failure
     */
    fn delete(&mut self, key: &str) -> bool {
        /*
         * .remove() removes the key and returns the old value (if it existed)
         */
        match self.data.remove(key) {
            /* If something was removed, we got Some(old_value) */
            Some(_) => {
                /*
                 * The underscore _ means we don't care about the actual value that was removed
                 * This is Rust's way of saying "I know there's a value here but I don't need it"
                 */
                println!("Deleted: {}", key);
                true
            }
            /* If nothing was removed, the key didn't exist */
            None => {
                println!("Key '{}' not found", key);
                false
            }
        }
    }
}

/*
 * Main function - entry point of our program
 */
fn main() {
    /*
     * Create a new mutable instance of our key-value store
     * 'mut' keyword makes it mutable so we can call methods that modify it
     * MUTABLE: Remember, Rust variables are immutable by default for safety.
     * We need 'mut' to allow modifications.
     */
    let mut store = KeyValueStore::new();

    /*
     * Build the CLI using clap
     * Command::new() creates a new CLI application
     */
    let matches = Command::new("Rusky")
        .version("1.0")
        .about("A Rust Key-Value Store")
        
        /* Define the "set" subcommand */
        .subcommand(
            Command::new("set")
                .about("Set a key-value pair")
                /* Add required arguments - key and value */
                .arg(Arg::new("key").required(true).help("The key"))
                .arg(Arg::new("value").required(true).help("The value")),
        )
        
        /* Define the "get" subcommand */
        .subcommand(
            Command::new("get")
                .about("Get a value by key")
                .arg(Arg::new("key").required(true).help("The key")),
        )
        
        /* Define the "delete" subcommand */
        .subcommand(
            Command::new("delete")
                .about("Delete a key-value pair")
                .arg(Arg::new("key").required(true).help("The key")),
        )
        
        /* Parse the command line arguments */
        .get_matches();

    /*
     * Match on which subcommand was used
     * This is Rust's pattern matching - very powerful!
     * It's like switch/case but guarantees you handle all possibilities
     */
    match matches.subcommand() {
        /* If "set" subcommand was used */
        Some(("set", sub_matches)) => {
            /*
             * Extract the key and value arguments
             * .get_one::<String>() gets the argument as a String
             * .unwrap() says "I know this will succeed" (because we marked them as required)
             * 
             * UNWRAP: This extracts the value from an Option or Result.
             * It panics (crashes) if there's no value, so only use when you're certain!
             */
            let key = sub_matches.get_one::<String>("key").unwrap();
            let value = sub_matches.get_one::<String>("value").unwrap();
            
            /*
             * Call our set method (need to clone because set takes ownership)
             * CLONE: Creates a deep copy of the data. Necessary here because
             * our set method takes ownership of the strings.
             */
            store.set(key.clone(), value.clone());
        }
        
        /* If "get" subcommand was used */
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap();
            /*
             * Call our get method (no cloning needed since get borrows)
             * BORROWING: Using a reference (&) to let another function use
             * our data without taking ownership of it.
             */
            store.get(key);
        }
        
        /* If "delete" subcommand was used */
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap();
            store.delete(key);
        }
        
        /*
         * If no subcommand or unknown subcommand was used
         * The _ is a catch-all pattern that matches anything not already matched
         */
        _ => {
            /* Print usage instructions */
            println!("Usage: rusky <set|get|delete> [args]");
            println!("  set <key> <value>    Set a key-value pair");
            println!("  get <key>            Get a value by key");
            println!("  delete <key>         Delete a key-value pair");
        }
    }
}