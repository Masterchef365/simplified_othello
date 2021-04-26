use othello::*;
fn main() {
    let state = State::new();
    for (_mov, state) in legal_moves(state) {
        println!("{}", state.board);
    }
}
