use crate::game::Game;
use crate::game::MoveType::{Rock, Paper, Scissors};

pub trait Command {
    fn run(&self, game: &Game) -> String;

    fn is_command(&self, input: &str) -> bool;
}
 
pub struct RockMove;
impl Command for RockMove {
    fn run(&self, game: &Game) -> String {
        game.do_move(Rock)
    }

    fn is_command(&self, input: &str) -> bool {
        input == "rock"
    }
}

pub struct PaperMove;
impl Command for PaperMove {
    fn run(&self, game: &Game) -> String {
        game.do_move(Paper)
    }

    fn is_command(&self, input: &str) -> bool {
        input == "paper"
    }
}

pub struct ScissorsMove;
impl Command for ScissorsMove {
    fn run(&self, game: &Game) -> String {
        game.do_move(Scissors)
    }

    fn is_command(&self, input: &str) -> bool {
        input == "scissors"
    }
}
