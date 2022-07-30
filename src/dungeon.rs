use text_colorizer::*;
use std::vec;
use rand::Rng;

pub struct Dungeon {
    board: Vec<Vec<i8>>
}

pub enum Tiles {
    EMPTY = 0,
    WALL = 1,
    PLAYER = 2,
    ENEMY = 3,
    EXIT = 4,
    ENTRANCE = 5,
}

impl Dungeon {
    pub fn new(row : usize, col : usize) -> Dungeon {
        let mut vec = vec![vec![Tiles::EMPTY as i8; row]; col];
        Dungeon {
            board: vec,
        }
    }
    pub fn get_raw_board_string(&self) -> String {
        let mut output:String = "".to_string();
        for row in self.board.iter() {
            for col in row.iter() {
                output += &col.to_string();
            }
            output += "\n";
        }
        return output;
    }
    pub fn print_board(&self) -> String {
        let mut output:String = "".to_string();
        for row in self.board.iter() {
            for col in row.iter() {
                let mut rng = rand::thread_rng();
                match &col {
                    0 => print!("{}", if rng.gen_range(0..2) == 0 {".".green()} else {",".green()}),
                    1 => print!("{}", "#".red().bold()),
                    2 => print!("{}", "P"),
                    3 => print!("{}", "E"),
                    4 => print!("{}", ">"),
                    5 => print!("{}", "<"),
                    _ => print!("{}", "u"),
                }
            }
            println!();
        }
        return output;
    }
}
