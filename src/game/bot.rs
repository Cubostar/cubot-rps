use crate::game::MoveType;

pub struct Cubot {
    rr: u32,
    rp: u32,
    rs: u32,
    pr: u32,
    pp: u32,
    ps: u32,
    sr: u32,
    sp: u32,
    ss: u32,
    rounds: u32,
    wins: u32,
    losses: u32,
    prev: Option<MoveType>,
}

impl Cubot {
    pub fn new() -> Self {
        Cubot {
            rr: 0,
            rp: 0,
            rs: 0,
            pr: 0,
            pp: 0,
            ps: 0,
            sr: 0,
            sp: 0,
            ss: 0,
            rounds: 0,
            wins: 0,
            losses: 0,
            prev: None,
        }
    }

    pub fn get_move(&self) -> MoveType {
        if self.prev.is_none() {
            
        }
    }
}
