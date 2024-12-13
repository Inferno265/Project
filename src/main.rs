use std::fs::File;
use crate::game::fgame;

mod game;

macro_rules! scanf {
    ($var: expr) => {
        std::io::stdin()
            .read_line(&mut $var)
            .expect("Error whilst reading line.");
    };
}

macro_rules! savExists {
    () => {
        match File::open("save.sav") {
            Ok(_) => {fgame("NaN");},
            Err(_) => {
                println!("Please enter your name.");
                let mut input = String::new();
                scanf!(&mut input);
                input = input.replace("\n", "");
                fgame(&input);
            },
        }
    };
}

fn main() {
    savExists!();
}