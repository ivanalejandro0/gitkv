use std::env;
use std::process;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Simple key:value store")
        .version("0.1")
        .author("Ivan <ivanalejandro0@gmail.com>")
        .about("Simple store for data in key:value shape")
        .subcommand(
            SubCommand::with_name("get")
                .about("gets a value by its key")
                .arg(
                    Arg::with_name("key") // And their own arguments
                        .help("the key to look for")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("sets a key/value pair")
                .arg(
                    Arg::with_name("key") // And their own arguments
                        .help("the key to save the value")
                        .required(true),
                )
                .arg(
                    Arg::with_name("value") // And their own arguments
                        .help("the value to save")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("list all the existing keys")
        )
        .subcommand(
            SubCommand::with_name("git")
                .about("work in progress: do stuff with git")
        )
        .get_matches();


    let store_path = match env::var("STORE") {
        Ok(value) => value,
        Err(e) => {
            eprintln!("You must define a STORE variable with the path to a repo. Error: {}", e);
            process::exit(1);
        }
    };

    let store = gitkv::Store::new(&store_path).unwrap();

    // Calling .unwrap() is safe for getting the values of key/value arguments since they are
    // required.
    // If they weren't required we could have used an 'if let' to conditionally get the values.
    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.value_of("key").unwrap();
        let value = matches.value_of("value").unwrap();
        match store.set(key, value) {
            Ok(()) => println!("[set ok] {}: {}", key, value),
            Err(e) => {
                eprintln!("There was a problem while setting key/value. Error: {}", e);
                process::exit(1);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.value_of("key").unwrap();
        match store.get(key) {
            Ok(value) => println!("[get ok] {}: {}", key, value),
            Err(e) => {
                eprintln!("There was a problem while getting key/value. Error: {}", e);
                process::exit(1);
            }
        }
    }

    if matches.is_present("list") {
        let entries = store.list();
        println!("Entries: {:?}", entries);
    }

    if matches.is_present("git") {
        store.git();
    }

}
