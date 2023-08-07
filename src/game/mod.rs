mod bot;
mod commands;

use std::io;
use commands::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum MoveType {
    Rock,
    Paper,
    Scissors,
}

pub struct Game {
    pwins: u32,
    bwins: u32,
    bot: bot::Cubot,
    quit: bool,
    cmdlist: Vec<Box<dyn Command>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pwins: 0,
            bwins: 0,
            bot: bot::Cubot::new(),
            quit: false,
            cmdlist: vec![],
        }
    }

    pub fn start(&self) {
        println!{"Welcome to Cubot-RPS v{}", VERSION}
        println!{"Type 'help' to see a list of commands and what they do"}

        let mut input = String::new();
        let mut found_cmd = false;

        while !self.quit {
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.to_lowercase();
            
            for cmd in &self.cmdlist {
                if cmd.is_command(&input) {
                    cmd.run(self);
                    found_cmd = true;
                    break;
                }
            }

            if !found_cmd {
                println!{"Invalid command!"}
            }

            found_cmd = false;
        }
    }

    pub fn get_pwins(&self) -> u32 {
        self.pwins
    }

    pub fn get_bwins(&self) -> u32 {
        self.bwins
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn Command>) {
        self.cmdlist.push(cmd);
    }

    pub fn do_move(&mut self, move_type: MoveType) -> String {
        let bot_move = self.bot.get_move();
    }
}
