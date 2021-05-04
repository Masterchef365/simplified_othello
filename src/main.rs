use othello::*;
use std::cmp::Ordering;
use std::io::Write;
use std::str::FromStr;

fn main() {
    // Arg parsing
    let mut args = std::env::args();
    let program_name = args.next().unwrap();
    let usage = format!("Usage: {} <Dark player> <Light player>", program_name);
    let player_err = "Player type must be `minimax` or `human`";
    let dark_player: PlayerType = args.next().expect(&usage).parse().expect(player_err);
    let light_player: PlayerType = args.next().expect(&usage).parse().expect(player_err);

    // Play the game
    let mut state = State::new();
    loop {
        println!("{}", state);
        let legal = legal_moves(state);

        // Skip a ply if there are no legal moves for the player. End the game if there are none
        // for either.
        if legal.is_empty() {
            println!("No valid moves");
            state.next_player = state.next_player.opposite();
            if state.last_skipped {
                println!("No valid moves for either player");
                break;
            }
            state.last_skipped = true;
            continue;
        } else {
            state.last_skipped = false;
        }

        // Ply
        let minimax_fail = "Failed to find successor state, this should be unreachable...";
        state = match state.next_player {
            Player::Dark => match dark_player {
                PlayerType::Human => human_player(&legal),
                PlayerType::Minimax => minimax(state).expect(minimax_fail),
            },
            Player::Light => match light_player {
                PlayerType::Human => human_player(&legal),
                PlayerType::Minimax => minimax(state).expect(minimax_fail),
            },
        }
    }

    let (dark, light) = state.board.scores();
    println!("Game ended, scores:");
    println!("\tDark: {}", dark);
    println!("\tLight: {}", light);
    match dark.cmp(&light) {
        Ordering::Less => println!("Light wins!"),
        Ordering::Greater => println!("Dark wins!"),
        Ordering::Equal => println!("Tie!"),
    }
}

/*
fn dumb_player(legal_moves: &[Successor]) -> State {
    println!("Machine played.");
    legal_moves.get(0).map(|(_, s)| *s).unwrap()
}
*/

fn human_player(legal_moves: &[Successor]) -> State {
    loop {
        let col = prompt_int("Enter col: ");
        let row = prompt_int("Enter row: ");
        match legal_moves.iter().find(|(pos, _)| *pos == (col, row)) {
            Some((_, s)) => break *s,
            None => println!("Invalid move! Please select another."),
        };
    }
}

fn prompt_int(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        std::io::stdout().lock().flush().unwrap();
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line");
        match s.trim_end().parse() {
            Ok(i) => break i,
            Err(_) => eprintln!("Please input an integer."),
        }
    }
}

enum PlayerType {
    Human,
    Minimax,
}

impl FromStr for PlayerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "human" => Ok(Self::Human),
            "minimax" => Ok(Self::Minimax),
            _ => Err(()),
        }
    }
}
