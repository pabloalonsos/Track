extern crate term;

use std::io;
use std::io::prelude::*;

fn capture_input() -> io::Result<String> {
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input));
    Ok(input)
}

fn print_welcome() {
    println!("Welcome to Track");
}

fn print_menu() {
    println!("Please use one of the following commands: ", );
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

fn main() {

    print_welcome();

    let mut input = String::new();
    while input != "exit\n" {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::GREEN).unwrap();
        print!("Command: ");
        input = capture_input().unwrap();
        t.fg(term::color::RED).unwrap();
        println!("{:?}", input);

        match input.trim() {
            "m" | "menu" => print_menu(),
            "h" | "help" => print_help(),
            "e" | "exit" => print_exit(),
            _ => print_wrong_command(),
        };


        t.reset().unwrap();

    }

}
