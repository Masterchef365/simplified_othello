use othello::*;

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
            Player::Dark => dumb_player(&legal),
            Player::Light => dumb_player(&legal),
        }
    }

    let (dark, light) = state.board.scores();
    println!("Game ended, scores:");
    println!("\tDark: {}", dark);
    println!("\tLight: {}", light);
}

fn dumb_player(legal_moves: &[Successor]) -> State {
    legal_moves.get(0).map(|(_, s)| *s).unwrap()
}
