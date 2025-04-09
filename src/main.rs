mod p1;
mod book;
mod dumb_search;

use std::io::stdin;

fn main() {
    loop {
        let mut opt = String::new();
        println!("Choose 1 to run the lessons from some video I've found.");
        println!("Choose 2 to run the lessons from the official Rust book.");
        println!("Choose 3 to run my dumb search algo.");
        println!("Choose 4 to convert some temps.");
        println!("Choose 0 if you a quitter.");
        let _size = stdin().read_line(&mut opt).unwrap();
        // shadow opt 
        let opt: i8 = opt.trim().parse().expect("Please type only integers, or crabs may panic!!!");
        if opt == 0 {
            break;
        }
        match opt {
            1 => p1::p1_lessons::run_p1(),
            2 => book::book_lessons::run_book_lessons(),
            3 => dumb_search::dumb_search::setup(),
            4 => book::temperature_converter::main(), 
            _ => {println!("Ok bye");break;},
        }
    }
}
