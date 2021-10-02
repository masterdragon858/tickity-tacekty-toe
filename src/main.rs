use rand::Rng;
use std::{collections::HashMap, io};

mod lib;

fn main() {
    let mut game_board: [String; 3] = [
        "- - -\n".to_string(),
        "- - -\n".to_string(),
        "- - -\n".to_string(),
    ];
    let mut players: String = String::new();
    let mut location_commands_to_indexes: HashMap<String, [usize; 2]> = HashMap::new();
    let mut playing: bool = true;
    location_commands_to_indexes.insert("tl".to_string(), [0, 0]);
    location_commands_to_indexes.insert("tm".to_string(), [0, 2]);
    location_commands_to_indexes.insert("tr".to_string(), [0, 4]);
    location_commands_to_indexes.insert("cl".to_string(), [1, 0]);
    location_commands_to_indexes.insert("cm".to_string(), [1, 2]);
    location_commands_to_indexes.insert("cr".to_string(), [1, 4]);
    location_commands_to_indexes.insert("bl".to_string(), [2, 0]);
    location_commands_to_indexes.insert("bm".to_string(), [2, 2]);
    location_commands_to_indexes.insert("br".to_string(), [2, 4]);

    while playing {
        game_board = [
            "- - -\n".to_string(),
            "- - -\n".to_string(),
            "- - -\n".to_string(),
        ];
        lib::displaying::print_welcome();
        println!("Enter the names of all players with a space in betweem. Ex: 'John Jessica'");
        io::stdin()
            .read_line(&mut players)
            .expect("Trouble getting palyer names");

        let player_list: Vec<&str> = players.split_whitespace().collect();

        let first_player: &str = player_list[rand::thread_rng().gen_range(0..2)];
        let second_player: &str = if first_player == player_list[0] {
            player_list[1]
        } else {
            player_list[0]
        };

        println!("{} goes first", first_player);
        let mut player1_command_valid: bool;
        let mut player2_command_valid: bool = true;

        loop {
            player1_command_valid = true;

            if player2_command_valid {
                if !lib::game_processes::handling_player_choice(
                    first_player,
                    &mut game_board,
                    &location_commands_to_indexes,
                    true,
                    &mut player1_command_valid,
                ) {
                    println!("Thanks for playing! Do you wanna try again?")
                }

                if !player1_command_valid {
                    continue;
                }
            }

            if lib::game_processes::check_winner(&game_board, 'o') {
                println!("{} has won!", first_player);
                if !lib::game_processes::play_again() {
                    playing = false;
                    break;
                } else {
                    break;
                }
            }

            player2_command_valid = true;
            if !lib::game_processes::handling_player_choice(
                second_player,
                &mut game_board,
                &location_commands_to_indexes,
                false,
                &mut player2_command_valid,
            ) {}

            if !player2_command_valid {
                continue;
            }

            if lib::game_processes::check_winner(&game_board, 'x') {
                println!("{} has won!", first_player);
                println!("{} has won!", first_player);
                if !lib::game_processes::play_again() {
                    playing = false;
                    break;
                } else {
                    break;
                }
            }
        }
    }
}
