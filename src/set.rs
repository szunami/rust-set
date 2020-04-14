use colored::{ColoredString, Colorize};
use crate::fmt;
use crate::set::Color::{RED, GREEN, PURPLE};
use crate::set::Quantity::{ONE, TWO, THREE};
use crate::set::Shading::{PARTIAL, EMPTY, FULL};
use crate::set::Shape::{SQUIGGLE, CIRCLE, DIAMOND};
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color { RED, GREEN, PURPLE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Quantity { ONE, TWO, THREE }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shading { EMPTY, PARTIAL, FULL }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape { SQUIGGLE, CIRCLE, DIAMOND }

#[derive(Clone, PartialEq)]
pub(crate) struct Card {
    color: Color,
    quantity: Quantity,
    shading: Shading,
    shape: Shape,
}

pub(crate) fn find_set(board: &Vec<Card>) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    for index_0 in 0..board.len() {
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

impl Card {
    pub(crate) fn print(&self) -> ColoredString {
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

pub(crate) fn is_set(card_0: &Card, card_1: &Card, card_2: &Card) -> bool {
    debug!("Checking if {:?}, {:?}, {:?} is a set...", card_0, card_1, card_2);

    if !((card_0.color == card_1.color && card_0.color == card_2.color) ||
        (card_0.color != card_1.color && card_0.color != card_2.color && card_1.color != card_2.color)) {
        debug!("Color check failed");
        return false;
    }

    if !((card_0.quantity == card_1.quantity && card_0.quantity == card_2.quantity) ||
        (card_0.quantity != card_1.quantity && card_0.quantity != card_2.quantity && card_1.quantity != card_2.quantity)) {
        debug!("Quantity check failed");
        return false;
    }

    if !((card_0.shading == card_1.shading && card_0.shading == card_2.shading) ||
        (card_0.shading != card_1.shading && card_0.shading != card_2.shading && card_1.shading != card_2.shading)) {
        debug!("Shading check failed");
        return false;
    }

    if !((card_0.shape == card_1.shape && card_0.shape == card_2.shape) ||
        (card_0.shape != card_1.shape && card_0.shape != card_2.shape && card_1.shape != card_2.shape)) {
        debug!("Shape check failed");
        return false;
    }

    return true;
}


impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{{{:?}, {:?}, {:?}}}", &self.color, &self.quantity, &self.shading).as_str())
    }
}

pub(crate) fn initial_deck() -> Vec<Card> {
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

pub(crate) fn shuffle(pile: &Vec<Card>) -> Vec<Card> {
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