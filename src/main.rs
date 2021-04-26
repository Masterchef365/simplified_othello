use othello::*;
fn main() {
    recursive(State::new(), 0);
}

fn recursive(state: State, depth: usize) {
    println!("{}\n", state);
    if depth > 30000 {
        return;
    }
    for (_mov, state) in legal_moves(state).into_iter().take(1) {
        recursive(state, depth + 1);
    }
}
