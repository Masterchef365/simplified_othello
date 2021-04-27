use crate::{legal_moves, Board, Player, State};

pub fn minimax(state: State) -> Option<State> {
    let player = state.next_player;
    let mut best = None;
    let mut best_score = std::isize::MIN;
    for (_, state) in legal_moves(state) {
        let score = min_value(state, player);
        if score > best_score {
            best = Some(state);
            best_score = score;
        }
    }
    best
}

fn min_value(state: State, player: Player) -> isize {
    let successors = legal_moves(state);
    let min = successors
        .into_iter()
        .map(|(_, state)| max_value(state, player))
        .min();
    if let Some(min) = min {
        return min;
    } else {
        let mut state = state;
        state.next_player = state.next_player.opposite();
        if state.last_skipped {
            return utility(state.board, player);
        }
        state.last_skipped = true;
        return max_value(state, player);
    }
}

fn max_value(state: State, player: Player) -> isize {
    let successors = legal_moves(state);
    let min = successors
        .into_iter()
        .map(|(_, state)| min_value(state, player))
        .min();
    if let Some(min) = min {
        return min;
    } else {
        let mut state = state;
        state.next_player = state.next_player.opposite();
        if state.last_skipped {
            return utility(state.board, player);
        }
        state.last_skipped = true;
        return min_value(state, player);
    }
}

/// Get the utility value of the board for this player
fn utility(board: Board, player: Player) -> isize {
    let (dark, light) = board.scores();
    let (us, them) = match player {
        Player::Dark => (dark, light),
        Player::Light => (light, dark),
    };
    return us as isize - them as isize;
}
