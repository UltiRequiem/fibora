mod lib;
use clap::{clap_app, ArgMatches};

fn get_arguments() -> ArgMatches {
    clap_app!(fibora =>
        (version: "1.0.0")
        (author: "UltiRequiem <eliaz.bobadilladev@gmail.com>")
        (about: "Fibonnaci Utilities for CLI.")
        (license: "MIT OR Apache-2.0")
        (@arg NUMBER: -n --number [NUMBER] "Print the Nth Fibonacci Number")
        (@arg SEQUENCE: -s --sequence [SEQUENCE] "Print an array with the first N numbers in the Fibonacci sequence")
    )
    .get_matches()
}

fn main() {
    let matches = get_arguments();

    if matches.is_present("NUMBER") && matches.is_present("SEQUENCE") {
        println!("You can't use number and sequence flag at the same time!")
    }

    if !matches.is_present("NUMBER") && !matches.is_present("SEQUENCE") {
        println!("Try fibora --help for more information!");
    }

    if matches.is_present("NUMBER") {
        let number = matches
            .value_of("NUMBER")
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        println!("{}", lib::fibonacci(number));
    }

    if matches.is_present("SEQUENCE") {
        let sequence = matches
            .value_of("SEQUENCE")
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        println!("{:?}", lib::fibonacci_sequence(sequence));
    }
}
