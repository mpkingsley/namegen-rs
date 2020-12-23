use std::io;
use std::fs;
extern crate markovr;
extern crate roxmltree;

struct Cli {

    file: std::path::PathBuf,
}

struct Chain {
    m: markovr::MarkovChain<char>,
    id: String, //id from xml file, used to select name generation 
    order: u8
}

fn main() {  
    let path = std::env::args().nth(1).expect("no file given");
    let args = Cli {
        file: std::path::PathBuf::from(path),
    };  
    let mut menu = String::new();
    let mut c: char = ' ';
    while c != 'q'{
        let mut choice  = String::new();
        println!("The File contains the following types:");
        //TODO for loop iterator to create a menu
        print!("Please select your option: or q to quit");
        io::stdin()
          .read_line(&mut choice)
          .expect("Invalid Input; try again"); 
        let b: Vec<char> = choice.chars().collect();         
        c = b[0];
    }


}

