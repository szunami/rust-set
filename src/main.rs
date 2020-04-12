use crate::Color::{GREEN, RED, PURPLE};
use crate::Quantity::{ONE, TWO, THREE};
use crate::Shading::{EMPTY, PARTIAL, FULL};
use core::fmt;
use rand::Rng;

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
    let mut pile = initial_deck();
    println!("Initial pile:\n\n{:?}", pile);
    println!("Shuffled pile:\n\n{:?}", shuffle(&pile))
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
