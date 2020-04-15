use crate::game::Game;
use std::io;
use crate::set;
use std::num::ParseIntError;

pub enum Move {
    RequestDeal,
    RequestHelp,
    RequestExit,
    IdentifySet(usize, usize, usize),
}

pub struct DummyInputProvider {}

impl DummyInputProvider {
    pub(crate) fn get_move(game: &Game) -> Move {
        debug!("Getting move from user...");
        println!("Current board state");
        print_board(&game.board);
        println!("Enter help, deal, input a set, or exit.");

        match read_input() {
            Err(e) => {
                println!("{}", e.details);
                return DummyInputProvider::get_move(game);
            }
            Ok(input) => input
        }
    }

    pub
    (crate) fn
    deny_move() {
        println!("There is a valid set. Keep looking!");
    }

    pub fn
    give_move(given_move: &(usize, usize, usize)) {
        let (index_0, index_1, index_2) = given_move;
        println!("Did you think about {}, {}, {}?", index_0, index_1, index_2);
    }
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

fn read_input() -> Result<Move, MyError> {
    let mut raw_user_input = String::new();

    io::stdin()
        .read_line(&mut raw_user_input)
        .expect("Failed to read line");

    if raw_user_input.trim() == "deal" {
        return Ok(Move::RequestDeal);
    }

    if raw_user_input.trim() == "help" {
        return Ok(Move::RequestHelp);
    }

    if raw_user_input.trim() == "exit" {
        return Ok(Move::RequestExit);
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

    return Ok(Move::IdentifySet(card_index_0, card_index_1, card_index_2));
}


fn parse_card_index(potential_set_indices: &Vec<&str>, input_index: usize) -> Result<i32, ParseIntError> {
    let card_index: &str = potential_set_indices.get(input_index).unwrap();
    return card_index.trim().parse::<i32>();
}