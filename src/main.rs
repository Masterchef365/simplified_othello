use othello::*;
fn main() {
    let mut last_had_no_move = false;
    let mut state = State::new();
    loop {
        println!("{}", state);
        let legal_moves = legal_moves(state);
        match dumb_player(&legal_moves) {
            Some(s) => {
                last_had_no_move = false;
                state = s;
            }
            None => {
                if last_had_no_move {
                    println!("Draw");
                    break;
                }
                last_had_no_move = true;
                state.next_player = state.next_player.opposite();
            }
        };
    }
}

fn dumb_player(legal_moves: &[Successor]) -> Option<State> {
    legal_moves.get(0).map(|(_, s)| *s)
}
