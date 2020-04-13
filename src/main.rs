use crate::Color::{GREEN, RED, PURPLE};
use crate::Quantity::{ONE, TWO, THREE};
use crate::Shading::{EMPTY, PARTIAL, FULL};
use core::fmt;
use rand::Rng;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color { RED, GREEN, PURPLE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Quantity { ONE, TWO, THREE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shading { EMPTY, PARTIAL, FULL }

#[derive(Clone, PartialEq)]
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
        println!("Current board state");
        print_board(&board);
        println!("Enter a potential set, or type deal.");

        let (card_index_0, card_index_1, card_index_2) = read_input();

        let card_0: &Card = board.get(card_index_0).unwrap();
        let card_1: &Card = board.get(card_index_1).unwrap();
        let card_2: &Card = board.get(card_index_2).unwrap();

        println!("You chose {:?}, {:?}, {:?}", card_0, card_1, card_2);

        if is_set(card_0, card_1, card_2) {
            let mut indices = vec![card_index_0, card_index_1, card_index_2];
            indices.sort();
            println!("Nicely done!");
            board.remove(indices[2]);
            board.remove(indices[1]);
            board.remove(indices[0]);
        } else {
            println!("Keep looking!")
        }
    }
}

fn print_board(board: &Vec<Card>) {
    for (index, card) in board.iter().enumerate() {
        print!("{}:{:?}\t\t", index, card);
        if (index + 1) % 3 == 0 {
            print!("\n");
        }
    }
}

fn read_input() -> (usize, usize, usize) {
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

    return (card_index_0, card_index_1, card_index_2);
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

    if !((card_0.color == card_1.color && card_0.color == card_2.color) ||
        (card_0.color != card_1.color && card_0.color != card_2.color && card_1.color != card_2.color)) {
        println!("Color check failed");
        return false;
    }

    if !((card_0.quantity == card_1.quantity && card_0.quantity == card_2.quantity) ||
        (card_0.quantity != card_1.quantity && card_0.quantity != card_2.quantity && card_1.quantity != card_2.quantity)) {
        println!("Quantity check failed");
        return false;
    }

    if !((card_0.shading == card_1.shading && card_0.shading == card_2.shading) ||
        (card_0.shading != card_1.shading && card_0.shading != card_2.shading && card_1.shading != card_2.shading)) {
        println!("Shading check failed");
        return false;
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_set_works() {
        let card_0 = Card {
            color: RED,
            quantity: ONE,
            shading: EMPTY,
        };
        let card_1 = Card {
            color: RED,
            quantity: ONE,
            shading: PARTIAL,
        };
        let card_2 = Card {
            color: RED,
            quantity: ONE,
            shading: FULL,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_2() {
        let card_0 = Card {
            color: RED,
            quantity: ONE,
            shading: EMPTY,
        };
        let card_1 = Card {
            color: RED,
            quantity: TWO,
            shading: PARTIAL,
        };
        let card_2 = Card {
            color: RED,
            quantity: THREE,
            shading: FULL,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_3() {
        let card_0 = Card {
            color: RED,
            quantity: ONE,
            shading: EMPTY,
        };
        let card_1 = Card {
            color: GREEN,
            quantity: TWO,
            shading: PARTIAL,
        };
        let card_2 = Card {
            color: PURPLE,
            quantity: THREE,
            shading: FULL,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_4() {
        let card_0 = Card {
            color: PURPLE,
            quantity: TWO,
            shading: FULL,
        };
        let card_1 = Card {
            color: RED,
            quantity: THREE,
            shading: PARTIAL,
        };
        let card_2 = Card {
            color: RED,
            quantity: THREE,
            shading: EMPTY,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), false)
    }
}