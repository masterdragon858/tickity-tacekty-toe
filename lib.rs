pub mod displaying {
    pub fn print_welcome() {
        println!(
            "
Welcome to my tic-tac-toe game!\n
Players will alternate between picking locations on the board to place their X or O.\n
The player that starts first will be randomly chosen.\n
The player that goes first will be the O's\n
When it is a player's turn they will type in where they wish to put their marrk, Ex: Top Middle\n
Use command 'lc' to see all placement commads and locations you'll need to type in
"
        );
    }

    pub fn print_game_board(board: &[String; 3]) {
        for text in board {
            print!("{}", text);
        }
    }

    pub fn print_location_commands() {
        println!("The location commands are: TL (Top Left), TM (Top Middle), TR (Top Right), CL (Center left), CM (Center Middle), CR (Center Right), BL (Bottom Left), BM (Bottom Middle), and finally BR (Bottom Right)");
    }
}

pub mod game_processes {
    pub use std::{collections::HashMap, io};

    pub fn handling_player_choice(
        player: &str,
        game_board: &mut [String; 3],
        map: &HashMap<String, [usize; 2]>,
        is_player1: bool,
        cmd_valid: &mut bool,
    ) -> bool {
        super::displaying::print_game_board(game_board);
        let mut player_decision: String = String::new();
        println!("{}'s turn, enter your choice", player);

        io::stdin()
            .read_line(&mut player_decision)
            .expect("Problem reading the player decision");

        if player_decision.trim().to_lowercase() == "lc" {
            super::displaying::print_location_commands();
            *cmd_valid = false;
        } else if map.contains_key(player_decision.trim()) {
            let positions_to_change = map.get(player_decision.trim()).unwrap();
            if game_board[positions_to_change[0]]
                .chars()
                .nth(positions_to_change[1])
                .unwrap()
                != '-'
            {
                println!("That spot has already been chosen");
                *cmd_valid = false;
            } else {
                if is_player1 {
                    game_board[positions_to_change[0]].remove(positions_to_change[1]);
                    game_board[positions_to_change[0]].insert(positions_to_change[1], 'o');
                } else {
                    game_board[positions_to_change[0]].remove(positions_to_change[1]);
                    game_board[positions_to_change[0]].insert(positions_to_change[1], 'x');
                }
            }
        } else if player_decision.trim().to_lowercase() == "quit" {
            return false;
        } else {
            println!("Plese enter a valid command.");
            *cmd_valid = false;
        }

        return true;
    }

    pub fn check_winner(game_board: &[String; 3], mark: char) -> bool {
        // Checking if any rows have won yet
        for i in 0..3 {
            if game_board[i].chars().nth(0).unwrap() == mark
                && game_board[i].chars().nth(2).unwrap() == mark
                && game_board[i].chars().nth(4).unwrap() == mark
            {
                return true;
            }
        }

        // Checking if any diagnol wins ahve been made yet
        if game_board[0].chars().nth(4).unwrap() == mark
            && game_board[1].chars().nth(2).unwrap() == mark
            && game_board[2].chars().nth(0).unwrap() == mark
        {
            return true;
        } else if game_board[0].chars().nth(0).unwrap() == mark
            && game_board[1].chars().nth(2).unwrap() == mark
            && game_board[2].chars().nth(4).unwrap() == mark
        {
            return true;
        }

        // Checking if any columns have oen yet
        if game_board[0].chars().nth(0).unwrap() == mark
            && game_board[1].chars().nth(0).unwrap() == mark
            && game_board[2].chars().nth(0).unwrap() == mark
        {
            return true;
        } else if game_board[0].chars().nth(2).unwrap() == mark
            && game_board[1].chars().nth(2).unwrap() == mark
            && game_board[2].chars().nth(2).unwrap() == mark
        {
            return true;
        } else if game_board[0].chars().nth(4).unwrap() == mark
            && game_board[1].chars().nth(4).unwrap() == mark
            && game_board[2].chars().nth(4).unwrap() == mark
        {
            return true;
        }

        false
    }

    pub fn play_again() -> bool {
        loop {
            let mut decision: String = String::new();
            println!("Would you like to play again?");

            io::stdin()
                .read_line(&mut decision)
                .expect("Somehting went wrong finding out of the user wanted to paly again");
            decision = decision.trim().to_lowercase();
            if decision == "no".to_string() {
                return false;
            } else if decision == "yes".to_string() {
                return true;
            } else {
                println!("Enter either 'yes' or 'no'");
                continue;
            }
        }
    }
}

