use std::io::{self, Read, Write};  // Include Write here
use serde::{Deserialize, Serialize};
use std::fs::File;
use base64::{encode, decode};

#[derive(Deserialize, Serialize, Debug)]
struct PlayerBoard {
    name: String,
    money: f32,
    work1: i32,
    work2: i32,
    bills: f32,
    day: i32,
    hunger: f32,
    thirst: f32,
    hours: i32
}

macro_rules! exit {
    () => {
        std::process::exit(0);
    };
}

macro_rules! board {
    // Accept a reference to the PlayerBoard struct and use its fields
    ($pb:expr) => {
        println!("You currently have ${}", $pb.money);
        println!("1. Work - Will earn you ${}/hr", $pb.work1);
        println!("2. Work - Will earn you ${}/hr", $pb.work2);
        println!("Your bills are ${}", $pb.bills);
        println!("Day: {}", $pb.day);
        println!("Hunger: {:.2}%", $pb.hunger);  // Print hunger level
        println!("Thirst: {:.2}%", $pb.thirst);  // Print thirst level
    };
}

macro_rules! scanf {
    // Accept a mutable reference to a string for input
    ($edit:expr) => {
        io::stdin()
            .read_line(&mut $edit)
            .expect("Unable to read line.");
    };
}

fn create_save(cbrd: &PlayerBoard) -> io::Result<()> {
    // Create or overwrite the file "save.sav"
    let mut f = File::create("save.sav")?;

    // Serialize the PlayerBoard to JSON
    let serialized = serde_json::to_string(cbrd)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Base64 encode the serialized JSON and write it to the file
    f.write_all(encode(serialized).as_bytes())?;

    Ok(())
}

fn load_board(f: &str) -> io::Result<PlayerBoard> {
    let mut _file = File::open(f)?;
    let mut contents = String::new();
    _file.read_to_string(&mut contents)?;

    // Decode the Base64-encoded contents into bytes
    let cont_bytes = base64::decode(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Base64 decode error: {}", e)))?;

    // Convert bytes into a UTF-8 string
    let decoded_str = String::from_utf8(cont_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid UTF-8: {}", e)))?;

    // Deserialize the decoded JSON string into PlayerBoard
    let board: PlayerBoard = serde_json::from_str(&decoded_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("JSON deserialization error: {}", e)))?;

    Ok(board)
}


fn main() {
    // Initialize the PlayerBoard struct
    let mut cbrd: PlayerBoard;

    // Try loading the board from a file, or set a default if it fails
    match load_board("save.sav") {
        Ok(board) => cbrd = board,
        Err(_) => {
            cbrd = PlayerBoard {
                name: "Player".to_string(),
                money: 0.0,
                work1: 8,
                work2: 12,
                bills: 4.0,
                day: 1,
                hunger: 100.0,
                thirst: 100.0,
                hours: 0
            };
            match create_save(&cbrd) {
                Ok(_) => {},
                Err(e) => {eprintln!("{}", e)},
            }
        }
    }

    println!("Hello {}! Are you ready for some gameplay?", cbrd.name);

    loop {
        // Call the board! macro and pass the PlayerBoard reference
        board!(&cbrd);

        // Create a new String for user input
        let mut input = String::new();

        // Read user input using the scanf! macro
        scanf!(&mut input);

        // Print the user's input
        if cbrd.hours >= 24 {
            input = "next".to_string();
            cbrd.hours = 0;
        }

        // Optionally, process the input (e.g., choose work, pay bills, etc.)

        // If you want to save the board at any point, call create_save:
        // For example, if the user typed "save":
        if input.trim() == "save" {
            match create_save(&cbrd) {
                Ok(_) => println!("Game saved successfully!"),
                Err(e) => eprintln!("Error saving the game: {}", e),
            }
        } else if input.trim() == "exit" {
            exit!();
        } else if input.trim() == "1" {
            let mut input = String::new();
            let mut hours: i32 = 0;
            println!("Please enter how many hours you want to work.");
            scanf!(&mut input);
            match input.trim().parse::<i32>() {
                Ok(num) => hours = num,
                Err(_) => println!("Please enter a number e.g. 8"),
            }
            
            // Update money based on the work
            cbrd.money += (cbrd.work1 * hours) as f32;
            
            // Update hunger and thirst based on work
            cbrd.hunger -= (cbrd.work1 as f32) / 4.0 * (hours as f32) / 3.0;
            cbrd.thirst -= (cbrd.work1 as f32) / 5.25 * (hours as f32);

            // Check if hunger or thirst is too low and game over
            if cbrd.hunger <= 0.0 || cbrd.thirst <= 0.0 {
                println!("You passed out! Game Over.");
                exit!();
            }
        } else if input.trim() == "next" {
            cbrd.money -= cbrd.bills;
            cbrd.bills += 0.5;
            cbrd.day += 1;
        } else if input.trim() == "2" {
            let mut input = String::new();
            let mut hours: i32 = 0;
            println!("Please enter how many hours you want to work.");
            scanf!(&mut input);
            match input.trim().parse::<i32>() {
                Ok(num) => hours = num,
                Err(_) => println!("Please enter a number e.g. 8"),
            }
            
            // Update money based on the work
            cbrd.money += (cbrd.work2 * hours) as f32;
            cbrd.hours += hours;
            
            // Update hunger and thirst based on work
            cbrd.hunger -= (cbrd.work2 as f32)  / 2.0 * (hours as f32) / 3.0 * 2.0;
            cbrd.thirst -= (cbrd.work2 as f32) / 4.0 * (hours as f32) * 2.0;

            // Check if hunger or thirst is too low and game over
            if cbrd.hunger <= 0.0 || cbrd.thirst <= 0.0 {
                println!("You passed out! Game Over.");
                exit!();
            }
        } else if input.trim() == "eat" {
            cbrd.money -= 24.0;
            cbrd.hunger = 100.0;
            cbrd.thirst = 100.0;
        }
    }
}
