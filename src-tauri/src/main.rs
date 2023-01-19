#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

const NUM_OF_LETTERS: u8 = 8;
const BOARD_SIZE: u8 = NUM_OF_LETTERS * 2;
const NUM_OF_COLUMNS: u8 = (BOARD_SIZE / 2) / 2; // To create a 4 x 4

#[derive(Debug, Serialize, Deserialize)]
struct MemoryGame {
    letters: Mutex<Vec<char>>,
    board: Mutex<Vec<Vec<BoardItem>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BoardItem {
    value: char,
    marked: bool,
}

#[tauri::command]
fn init_game(state: tauri::State<'_, MemoryGame>) -> Vec<char> {
    let mut letters: Vec<char> = std::vec![];
    let mut board: Vec<Vec<BoardItem>> = std::vec![];
    let mut temp_board: Vec<BoardItem> = std::vec![];

    // Create letters to be on game
    for _ in 0..NUM_OF_LETTERS {
        let mut letter: Option<char> = None;

        while letter == None {
            let numeral_char: u8 = rand::thread_rng().gen_range(65..90);
            let new_value = numeral_char as char;

            if !letters.contains(&new_value) {
                letter = Some(new_value);
            }
        }

        letters.push(letter.unwrap());
    }

    // Build a board
    for index in 0..NUM_OF_LETTERS {
        let letter = letters[index as usize];

        temp_board.push(BoardItem {
            value: letter,
            marked: false,
        });
        temp_board.push(BoardItem {
            value: letter,
            marked: false,
        });
    }
    temp_board.shuffle(&mut rand::thread_rng());
    let chunked: Vec<Vec<BoardItem>> = temp_board
        .chunks(NUM_OF_COLUMNS as usize)
        .map(|x| x.to_vec())
        .collect();

    *state.letters.lock().unwrap() = letters;
    *state.board.lock().unwrap() = chunked;

    return state.letters.lock().unwrap().clone();
}

#[tauri::command]
fn get_letters(state: tauri::State<MemoryGame>) -> Vec<char> {
    state.letters.lock().unwrap().clone()
}

#[tauri::command]
fn get_board(state: tauri::State<MemoryGame>) -> Vec<Vec<BoardItem>> {
    let mut board = vec![];

    for item in state.board.lock().unwrap().clone().iter() {
        board.push(item.to_vec())
    }

    println!("{:?}", board);

    return board;
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(MemoryGame {
            letters: Mutex::new(std::vec![]),
            board: Mutex::new(std::vec![]),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            init_game,
            get_letters,
            get_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
