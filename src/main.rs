extern crate term;

use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;

fn capture_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => String::from(input.trim_right_matches("\n")),
        Err(why) => panic!("Error capturing input: {}", why.description())
    }
}

fn capture_command() -> String {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    print!("Command: ");
    io::stdout().flush().unwrap();
    t.reset().unwrap();
    capture_input()
}

fn print_welcome() {
    println!("Welcome to Track");
}

fn print_menu() {
    println!("Please use one of the following commands: ", );

    println!("\t'add-entry' (ae): Add a new Entry.");
    println!("\t'add-tag' (at): Add a new Tag.");

    println!("\t'list' (l): List all Entries.");
    println!("\t'tags' (t): List all Tags.");

    println!("\t'remove-entry #entry-number' (re #): Remove Entry number #.");
    println!("\t'remove-tag #tag-number' (rt #): Remove Tag number #.");
}

fn print_exit() {
    println!("thanks! bye!");
}

fn print_help() {
    println!("... - - - ...");
}

fn print_wrong_command() {
    println!("please use one of the available commands. for help enter 'help' (h)");
}

fn open_file() -> File{
    match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(".track") {
            Err(why) => panic!("couldn't create file: {}", why.description()),
            Ok(file) => file,
        }
}

fn parse_file(mut file: File) -> String {
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read file: {}", why.description()),
        Ok(bytes) => println!("Number of bytes read: {}", bytes),
    }
    buffer
}

fn add_entry() {
    println!("\n\n");
    print!("Enter title: ");
    io::stdout().flush().unwrap();
    let title = capture_input();
    println!("{:?}", title);
    print!("Enter tags: ");
    io::stdout().flush().unwrap();
    let tags = capture_input();
    print!("Enter cost: ");
    io::stdout().flush().unwrap();
    let cost = capture_input();
    print!("Enter priority: ");
    io::stdout().flush().unwrap();
    let priority = capture_input();
    println!("\n{} [{}]: ${}, {}\n", title, tags, cost, priority);
}

fn main() {

    print_welcome();
    let file_buffer: File = open_file();
    parse_file(file_buffer);

    let mut input = String::new();
    loop {
        input = capture_command();
        match input.trim() {
            "m" | "menu" => print_menu(),
            "h" | "help" => print_help(),
            "e" | "exit" => { print_exit(); break; },

            "ae" | "add-entry" => add_entry(),
            "at" | "add-tag" => print_exit(),
            "t" | "tags" => print_exit(),
            "l" | "list" => print_exit(),
            _ => print_wrong_command(),
        };
    }
}
