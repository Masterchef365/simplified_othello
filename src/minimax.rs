use crate::{State, Player, Board, legal_moves};

pub fn minimax(state: State) -> Option<State> {
    minimax_recursive(state, state.next_player)
}

fn minimax_descision(state: State, player: Player) -> Option<State> {

}

fn min_value(state: State, player: Player) -> isize {
    //let successors = legal_moves(state);
    //if successors.is_empty()
}

fn utility(board: Board, player: Player) -> isize {
    let (dark, light) = board.scores();
    let (us, them) = match player {
        Player::Dark => (dark, light),
        Player::Light => (light, dark),
    };
    return us as isize - them as isize;
}
