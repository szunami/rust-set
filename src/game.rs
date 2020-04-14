use crate::{set, input};
use crate::input::{Move, DummyInputProvider};


pub(crate) struct Game {
    pile: Vec<set::Card>,
    board: Vec<set::Card>,
    move_provider: input::DummyInputProvider,
}


impl Game {
    pub fn initialize(move_provider: input::DummyInputProvider) -> Game {
        let mut pile = set::shuffle(&set::initial_deck());
        let mut board = Vec::new();

        return Game {
            pile,
            board,
            move_provider,
        };
    }

    pub(crate) fn begin_playing(&mut self) {
        while !self.pile.is_empty() {
            while self.board.len() < 12 {
                self.deal_three_cards();
            }
        }

        match input::DummyInputProvider::get_move() {
            Move::IdentifySet(index_0, index_1, index_2) => {
                let card_0: &set::Card = self.board.get(index_0).unwrap();
                let card_1: &set::Card = self.board.get(index_1).unwrap();
                let card_2: &set::Card = self.board.get(index_2).unwrap();
                if set::is_set(card_0, card_1, card_2) {
                    self.remove_cards(index_0, index_1, index_2);
                }
            }
            Move::RequestExit => {
                return;
            }
            Move::RequestDeal => {
                if !set::find_set(&self.board).is_empty() {
                    input::DummyInputProvider::deny_move();
                }
            }
            Move::RequestHelp => {
                let sets = set::find_set(&self.board);
                if !sets.is_empty() {
                    DummyInputProvider::give_move(sets.get(0).unwrap());
                }
            }
        }
    }

    fn deal_three_cards(&mut self) {
        self.board.push(self.pile.pop().unwrap());
        self.board.push(self.pile.pop().unwrap());
        self.board.push(self.pile.pop().unwrap());
    }

    fn remove_cards(&mut self, index_0: usize, index_1: usize, index_2: usize) {
        let mut indices = vec![index_0, index_1, index_2];
        indices.sort();
        self.board.remove(indices[2]);
        self.board.remove(indices[1]);
        self.board.remove(indices[0]);
    }
}

