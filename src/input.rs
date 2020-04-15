pub enum Move {
    RequestDeal,
    RequestHelp,
    RequestExit,
    IdentifySet(usize, usize, usize),
}

pub struct DummyInputProvider {}

impl DummyInputProvider {
    pub(crate) fn get_move() -> Move {
        return Move::IdentifySet(0, 1, 2);
    }

    pub(crate) fn deny_move() {}

    pub fn give_move(given_move: &(usize, usize, usize)) {}
}