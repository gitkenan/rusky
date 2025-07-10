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
 * Import serde for serialization - converting Rust data structures to/from JSON
 * This allows us to save commands to a file and read them back
 */
use serde::{Deserialize, Serialize};

/*
 * Import file system operations and error handling
 * std::fs for file operations, std::io for input/output operations
 */
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

/*
 * Define a Command enum that represents all possible operations
 * This will be serialized to JSON and stored in our log file
 * 
 * ENUM: A type that can be one of several variants. Like a union in C
 * but much safer - Rust ensures you handle all possible cases.
 * 
 * DERIVE: Automatically implement traits (like interfaces) for our type
 * - Serialize/Deserialize: Convert to/from JSON
 * - Debug: Allow printing with {:?}
 * - Clone: Allow making copies
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Command {
    Set { key: String, value: String },
    Delete { key: String },
}

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
    
    /*
     * Path to our log file where we'll store all commands
     * This enables persistence across program restarts
     */
    log_path: String,
}

/*
 * Implementation block - this is where we define methods for our struct
 * Think of this like adding methods to a class
 */
impl KeyValueStore {
    /*
     * Constructor function - creates a new KeyValueStore and loads from log file
     * 'Self' is shorthand for KeyValueStore
     * The -> Self means this function returns an instance of KeyValueStore
     * 
     * RESULT: Rust's error handling type. Either Ok(value) or Err(error)
     * This forces us to handle potential errors explicitly.
     */
    fn new(log_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut store = KeyValueStore {
            data: HashMap::new(),
            log_path: log_path.clone(),
        };
        
        /*
         * Try to load existing commands from the log file
         * If the file doesn't exist, that's okay - we'll create it later
         */
        store.load_from_log()?;
        
        Ok(store)
    }
    
    /*
     * Load and replay all commands from the log file
     * This rebuilds our in-memory state from the persistent log
     */
    fn load_from_log(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        /*
         * Try to open the log file for reading
         * If it doesn't exist, just return Ok - we'll create it when needed
         */
        let file = match std::fs::File::open(&self.log_path) {
            Ok(file) => file,
            Err(_) => return Ok(()), /* File doesn't exist yet, that's fine */
        };
        
        /*
         * BufReader reads the file line by line efficiently
         * Much better than reading the entire file into memory at once
         */
        let reader = BufReader::new(file);
        
        /*
         * Read each line and deserialize it back into a Command
         * Then apply that command to rebuild our state
         */
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue; /* Skip empty lines */
            }
            
            /*
             * Parse the JSON line back into a Command
             * serde_json::from_str converts JSON string to our Command enum
             */
            let command: Command = serde_json::from_str(&line)?;
            
            /* Apply the command to our in-memory data */
            self.apply_command(&command);
        }
        
        Ok(())
    }
    
    /*
     * Apply a command to our in-memory data structure
     * This is used both for new commands and when replaying the log
     */
    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Set { key, value } => {
                self.data.insert(key.clone(), value.clone());
            }
            Command::Delete { key } => {
                self.data.remove(key);
            }
        }
    }
    
    /*
     * Write a command to the log file
     * This ensures every operation is persisted
     */
    fn write_to_log(&self, command: &Command) -> Result<(), Box<dyn std::error::Error>> {
        /*
         * Open the log file in append mode
         * create(true) means create the file if it doesn't exist
         * append(true) means write to the end of the file
         */
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        
        /*
         * Convert the command to JSON and write it to the file
         * Each command gets its own line in the file
         */
        let json = serde_json::to_string(command)?;
        writeln!(file, "{}", json)?;
        
        /*
         * Flush ensures the data is actually written to disk immediately
         * This is important for durability - we don't want to lose data
         */
        file.flush()?;
        
        Ok(())
    }

    /*
     * Method to set a key-value pair
     * Now this method also persists the command to the log file
     * 
     * RESULT: We return Result to handle potential I/O errors
     */
    fn set(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        /*
         * Create a command representing this operation
         */
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        
        /*
         * Write the command to the log file first (Write-Ahead Log pattern)
         * This ensures we don't lose the operation even if the program crashes
         */
        self.write_to_log(&command)?;
        
        /*
         * Apply the command to our in-memory data
         */
        self.apply_command(&command);
        
        /* Print confirmation to the user */
        println!("Set: {} = {}", key, value);
        
        Ok(())
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
     * Now this method also persists the command to the log file
     */
    fn delete(&mut self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        /*
         * Check if the key exists before trying to delete it
         * We only want to log successful deletions
         */
        if self.data.contains_key(key) {
            /*
             * Create a command representing this operation
             */
            let command = Command::Delete {
                key: key.to_string(),
            };
            
            /*
             * Write the command to the log file first
             */
            self.write_to_log(&command)?;
            
            /*
             * Apply the command to our in-memory data
             */
            self.apply_command(&command);
            
            println!("Deleted: {}", key);
            Ok(true)
        } else {
            println!("Key '{}' not found", key);
            Ok(false)
        }
    }
}

/*
 * Main function - entry point of our program
 */
fn main() {
    /*
     * Create a new mutable instance of our key-value store
     * Now we pass the log file path and handle potential errors
     * 
     * ERROR HANDLING: The ? operator is Rust's way of early return on error
     * If KeyValueStore::new() returns an Err, the ? will return that error
     * from our main function immediately
     */
    let mut store = match KeyValueStore::new("rusky.log".to_string()) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Error initializing store: {}", e);
            return;
        }
    };

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
             * Now we also need to handle potential I/O errors
             */
            if let Err(e) = store.set(key.clone(), value.clone()) {
                eprintln!("Error setting value: {}", e);
            }
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
            if let Err(e) = store.delete(key) {
                eprintln!("Error deleting key: {}", e);
            }
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