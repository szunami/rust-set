use crate::Color::{GREEN, RED, PURPLE};
use crate::Quantity::{ONE, TWO, THREE};
use crate::Shading::{EMPTY, PARTIAL, FULL};
use core::fmt;
use rand::Rng;
use std::io;

#[derive(Debug, Clone)]
enum Color { RED, GREEN, PURPLE }

#[derive(Debug, Clone)]
enum Quantity { ONE, TWO, THREE }

#[derive(Debug, Clone)]
enum Shading { EMPTY, PARTIAL, FULL }

#[derive(Clone)]
struct Card {
    color: Color,
    quantity: Quantity,
    shading: Shading,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{{{:?}, {:?}, {:?}}}", &self.color, &self.quantity, &self.shading).as_str())
    }
}

fn main() {
    let mut pile = shuffle(&initial_deck());
    let mut board = Vec::new();

    while !pile.is_empty() {
        while board.len() < 12 {
            println!("Dishing out 3 more cards");
            board.push(pile.pop().unwrap());
            board.push(pile.pop().unwrap());
            board.push(pile.pop().unwrap());
        }
        println!("Current board state: {:?}", board);
        println!("Enter a potential set, or type deal.");

        let mut raw_user_input = String::new();

        io::stdin()
            .read_line(&mut raw_user_input)
            .expect("Failed to read line");

        let potential_set_indices: Vec<&str> = raw_user_input.split(" ").collect::<Vec<&str>>();
        if potential_set_indices.len() != 3 {
            println!("Failed to read line");
        }

        let card_index_0 = parse_card_index(&potential_set_indices, 0) as usize;
        let card_index_1 = parse_card_index(&potential_set_indices, 1) as usize;
        let card_index_2 = parse_card_index(&potential_set_indices, 2) as usize;

        let card_0: &Card = board.get(card_index_0).unwrap();
        let card_1: &Card = board.get(card_index_1).unwrap();
        let card_2: &Card = board.get(card_index_2).unwrap();

        if (is_set(card_0, card_1, card_2)) {
            println!("well alright!");
        }
    }
}

fn parse_card_index(potential_set_indices: &Vec<&str>, input_index: usize) -> i32 {
    let card_index: &str = match potential_set_indices.get(input_index) {
        Some(num) => *num,
        _ => panic!("asdf")
    };
    return card_index.trim().parse::<i32>().unwrap();
}

fn is_set(card_0: &Card, card_1: &Card, card_2: &Card) -> bool {
    println!("Checking if {:?}, {:?}, {:?} is a set...", card_0, card_1, card_2);

    return true;
}

fn initial_deck() -> Vec<Card> {
    let mut pile = Vec::new();
    for color in [RED, GREEN, PURPLE].iter() {
        for quantity in [ONE, TWO, THREE].iter() {
            for shading in [EMPTY, PARTIAL, FULL].iter() {
                let card = Card {
                    color: color.clone(),
                    quantity: quantity.clone(),
                    shading: shading.clone(),
                };
                pile.push(card);
            }
        }
    }
    return pile;
}

fn shuffle(pile: &Vec<Card>) -> Vec<Card> {
    let mut result = pile.clone();
    for (index, _card) in pile.iter().enumerate() {
        let low = index;
        let high = pile.len() - 1;
        if low == high {
            break;
        }
        let swap_index = rand::thread_rng().gen_range(index, pile.len() - 1);
        result.swap(index, swap_index);
    }
    return result;
}

