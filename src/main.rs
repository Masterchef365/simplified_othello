use othello::*;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    let mut last_had_no_move = false;
    let mut state = State::new();
    loop {
        println!("{}", state);
        let legal = legal_moves(state);

        // Skip a ply if there are no legal moves for the player. End the game if there are none
        // for either.
        if legal.is_empty() {
            println!("No valid moves");
            state.next_player = state.next_player.opposite();
            if last_had_no_move {
                break;
            }
            last_had_no_move = true;
            continue;
        } else {
            last_had_no_move = false;
        }

        state = match state.next_player {
            Player::Dark => human_player(&legal),
            Player::Light => dumb_player(&legal),
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

fn dumb_player(legal_moves: &[Successor]) -> State {
    println!("Machine played.");
    legal_moves.get(0).map(|(_, s)| *s).unwrap()
}

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
