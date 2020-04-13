use crate::Color::{GREEN, RED, PURPLE};
use crate::Quantity::{ONE, TWO, THREE};
use crate::Shading::{EMPTY, PARTIAL, FULL};
use core::fmt;
use rand::Rng;
use std::io;
use colored::*;
use crate::Shape::{DIAMOND, SQUIGGLE, CIRCLE};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color { RED, GREEN, PURPLE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Quantity { ONE, TWO, THREE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shading { EMPTY, PARTIAL, FULL }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape { SQUIGGLE, CIRCLE, DIAMOND }

#[derive(Clone, PartialEq)]
struct Card {
    color: Color,
    quantity: Quantity,
    shading: Shading,
    shape: Shape,
}

enum InputType {
    Deal,
    Help,
    PotentialSet(usize, usize, usize),
}

impl Card {
    fn print(&self) -> ColoredString {
        let shape = match self.shape {
            SQUIGGLE => "S",
            CIRCLE => "O",
            DIAMOND => "<>",
        };

        let repeated_shape = match self.quantity {
            ONE => String::from(shape),
            TWO => shape.repeat(2),
            THREE => shape.repeat(3)
        };

        let colored_repeated_shape: ColoredString = match self.color {
            RED => repeated_shape.red(),
            GREEN => repeated_shape.green(),
            PURPLE => repeated_shape.purple(),
        };

        let shaded_colored_repeated_shape = match self.shading {
            EMPTY => colored_repeated_shape.dimmed(),
            PARTIAL => colored_repeated_shape,
            FULL => colored_repeated_shape.bold(),
        };

        return shaded_colored_repeated_shape;
    }
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
        println!("Enter help, deal, or input a set.");

        match read_input() {
            InputType::Deal => {
                if !find_set(&board).is_empty() {
                    println!("There is a set here. Type help if you need help.");
                } else {
                    println!("There are no sets. Dealing 3 more cards.");
                    println!("Dishing out 3 more cards");
                    board.push(pile.pop().unwrap());
                    board.push(pile.pop().unwrap());
                    board.push(pile.pop().unwrap());
                }
            }
            InputType::Help => {
                let valid_sets = find_set(&board);
                if valid_sets.is_empty() {
                    println!("There are no sets here. Type deal.");
                    return;
                }
                println!("{:?}", valid_sets.get(0).unwrap());
            }
            InputType::PotentialSet(card_index_0, card_index_1, card_index_2) => {
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
        };
    }
}

fn find_set(board: &Vec<Card>) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    for index_0  in 0..board.len() {
        for index_1 in (index_0 + 1)..board.len() {
            for index_2 in (index_1 + 1)..board.len() {
                let card_0: &Card = board.get(index_0).unwrap();
                let card_1: &Card = board.get(index_1).unwrap();
                let card_2: &Card = board.get(index_2).unwrap();

                if is_set(card_0, card_1, card_2) {
                    result.push((index_0, index_1, index_2));
                }
            }
        }
    }
    return result;
}

fn print_board(board: &Vec<Card>) {
    for (index, card) in board.iter().enumerate() {
        print!("{:3}: [{:10}]", index, card.print());
        print!("\t\t");
        if (index + 1) % 3 == 0 {
            print!("\n");
        }
    }
}

fn read_input() -> InputType {
    let mut raw_user_input = String::new();

    io::stdin()
        .read_line(&mut raw_user_input)
        .expect("Failed to read line");

    if raw_user_input.trim() == "deal" {
        return InputType::Deal;
    }

    if raw_user_input.trim() == "help" {
        return InputType::Help;
    }

    let potential_set_indices: Vec<&str> = raw_user_input.split(" ").collect::<Vec<&str>>();
    if potential_set_indices.len() != 3 {
        println!("Failed to read line");
    }

    let card_index_0 = parse_card_index(&potential_set_indices, 0) as usize;
    let card_index_1 = parse_card_index(&potential_set_indices, 1) as usize;
    let card_index_2 = parse_card_index(&potential_set_indices, 2) as usize;

    return InputType::PotentialSet(card_index_0, card_index_1, card_index_2);
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

    if !((card_0.shape == card_1.shape && card_0.shape == card_2.shape) ||
        (card_0.shape != card_1.shape && card_0.shape != card_2.shape && card_1.shape != card_2.shape)) {
        println!("Shape check failed");
        return false;
    }

    return true;
}

fn initial_deck() -> Vec<Card> {
    let mut pile = Vec::new();
    for color in [RED, GREEN, PURPLE].iter() {
        for quantity in [ONE, TWO, THREE].iter() {
            for shading in [EMPTY, PARTIAL, FULL].iter() {
                for shape in [SQUIGGLE, CIRCLE, DIAMOND].iter() {
                    let card = Card {
                        color: color.clone(),
                        quantity: quantity.clone(),
                        shading: shading.clone(),
                        shape: shape.clone(),
                    };
                    pile.push(card);
                }
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
            shape: SQUIGGLE,
        };
        let card_1 = Card {
            color: RED,
            quantity: ONE,
            shading: PARTIAL,
            shape: SQUIGGLE,
        };
        let card_2 = Card {
            color: RED,
            quantity: ONE,
            shading: FULL,
            shape: SQUIGGLE,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_2() {
        let card_0 = Card {
            color: RED,
            quantity: ONE,
            shading: EMPTY,
            shape: SQUIGGLE,
        };
        let card_1 = Card {
            color: RED,
            quantity: TWO,
            shading: PARTIAL,
            shape: SQUIGGLE,
        };
        let card_2 = Card {
            color: RED,
            quantity: THREE,
            shading: FULL,
            shape: SQUIGGLE,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_3() {
        let card_0 = Card {
            color: RED,
            quantity: ONE,
            shading: EMPTY,
            shape: SQUIGGLE,
        };
        let card_1 = Card {
            color: GREEN,
            quantity: TWO,
            shading: PARTIAL,
            shape: SQUIGGLE,
        };
        let card_2 = Card {
            color: PURPLE,
            quantity: THREE,
            shading: FULL,
            shape: SQUIGGLE,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), true)
    }

    #[test]
    fn is_set_works_4() {
        let card_0 = Card {
            color: PURPLE,
            quantity: TWO,
            shading: FULL,
            shape: SQUIGGLE,
        };
        let card_1 = Card {
            color: RED,
            quantity: THREE,
            shading: PARTIAL,
            shape: SQUIGGLE,
        };
        let card_2 = Card {
            color: RED,
            quantity: THREE,
            shading: EMPTY,
            shape: SQUIGGLE,
        };

        assert_eq!(is_set(&card_0, &card_1, &card_2), false)
    }
}