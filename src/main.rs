#[macro_use]
extern crate log;
extern crate env_logger;

mod set;
mod game;
mod input;

use core::fmt;
use std::io;
use std::error::Error;
use std::num::ParseIntError;
use crate::set::find_set;

enum InputType {
    Deal,
    Help,
    PotentialSet(usize, usize, usize),
    Exit,
}


fn main() {
    env_logger::init();

    let move_provider = input::DummyInputProvider{};
    let mut game = game::Game::initialize(move_provider);
    game.begin_playing();

//    let mut pile = set::shuffle(&set::initial_deck());
//    let mut board = Vec::new();
//
//    while !pile.is_empty() {
//        while board.len() < 12 {
//            println!("Dishing out 3 more cards");
//            board.push(pile.pop().unwrap());
//            board.push(pile.pop().unwrap());
//            board.push(pile.pop().unwrap());
//        }
//        println!("Current board state");
//        print_board(&board);
//        println!("Enter help, deal, input a set, or exit.");
//
//        match read_input() {
//            Err(e) => println!("{}", e.details),
//            Ok(input) => {
//                match input {
//                    InputType::Exit => {
//                        println!("Goodbye for now.");
//                        return;
//                    }
//                    InputType::Deal => {
//                        if !set::find_set(&board).is_empty() {
//                            println!("There is a set here. Type help if you need help.");
//                        } else {
//                            println!("There are no sets. Dealing 3 more cards.");
//                            board.push(pile.pop().unwrap());
//                            board.push(pile.pop().unwrap());
//                            board.push(pile.pop().unwrap());
//                        }
//                    }
//                    InputType::Help => {
//                        let valid_sets = find_set(&board);
//                        if valid_sets.is_empty() {
//                            println!("There are no sets here. Type deal.");
//                            return;
//                        }
//                        println!("{:?}", valid_sets.get(0).unwrap());
//                    }
//                    InputType::PotentialSet(card_index_0, card_index_1, card_index_2) => {
//                        let card_0: &set::Card = board.get(card_index_0).unwrap();
//                        let card_1: &set::Card = board.get(card_index_1).unwrap();
//                        let card_2: &set::Card = board.get(card_index_2).unwrap();
//
//                        println!("You chose {:?}, {:?}, {:?}", card_0, card_1, card_2);
//
//                        if set::is_set(card_0, card_1, card_2) {
//                            let mut indices = vec![card_index_0, card_index_1, card_index_2];
//                            indices.sort();
//                            println!("Nicely done!");
//                            board.remove(indices[2]);
//                            board.remove(indices[1]);
//                            board.remove(indices[0]);
//                        } else {
//                            println!("Keep looking!")
//                        }
//                    }
//                }
//            }
//        };
//    }
}



fn print_board(board: &Vec<set::Card>) {
    for (index, card) in board.iter().enumerate() {
        print!("{:3}: [{:10}]", index, card.print());
        print!("\t\t");
        if (index + 1) % 3 == 0 {
            print!("\n");
        }
    }
}

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError { details: msg.to_string() }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn read_input() -> Result<InputType, MyError> {
    let mut raw_user_input = String::new();

    io::stdin()
        .read_line(&mut raw_user_input)
        .expect("Failed to read line");

    if raw_user_input.trim() == "deal" {
        return Ok(InputType::Deal);
    }

    if raw_user_input.trim() == "help" {
        return Ok(InputType::Help);
    }

    if raw_user_input.trim() == "exit" {
        return Ok(InputType::Exit);
    }

    let potential_set_indices: Vec<&str> = raw_user_input.split(" ").collect::<Vec<&str>>();
    if potential_set_indices.len() != 3 {
        return Err(MyError::new(format!("Expected 3 arguments, found {}.", potential_set_indices.len()).as_str()));
    }

    let card_index_0 = match parse_card_index(&potential_set_indices, 0) {
        Ok(num) => num,
        Err(_str) => return Err(MyError::new("Argument 0 must be an integer."))
    } as usize;
    let card_index_1 = match parse_card_index(&potential_set_indices, 1) {
        Ok(num) => num,
        Err(_str) => return Err(MyError::new("Argument 1 must be an integer."))
    } as usize;
    let card_index_2 = match parse_card_index(&potential_set_indices, 2) {
        Ok(num) => num,
        Err(_str) => return Err(MyError::new("Argument 2 must be an integer."))
    } as usize;

    return Ok(InputType::PotentialSet(card_index_0, card_index_1, card_index_2));
}

fn parse_card_index(potential_set_indices: &Vec<&str>, input_index: usize) -> Result<i32, ParseIntError> {
    let card_index: &str = potential_set_indices.get(input_index).unwrap();
    return card_index.trim().parse::<i32>();
}
