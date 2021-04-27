use crate::{legal_moves, Board, Player, State};

pub fn minimax(state: State) -> State {
    let player = state.next_player;
    let mut best = None;
    let mut best_score = isize::MIN;

    let (mut alpha, mut beta) = (isize::MIN, isize::MAX);
    for (_, state) in legal_moves(state) {
        let score = min_value(state, player, &mut alpha, &mut beta);
        if score > best_score {
            best = Some(state);
            best_score = score;
        }
    }
    best.expect("Failed to find successor state...")
}

fn min_value(state: State, player: Player, alpha: &mut isize, beta: &mut isize) -> isize {
    let successors = legal_moves(state);

    let mut min = isize::MAX;
    for (_, succ) in successors {
        min = min.min(max_value(succ, player, alpha, beta));
        if min <= *alpha {
            return min;
        } else {
            *alpha = (*alpha).min(min);
        }
    }

    if min != isize::MAX {
        return min;
    } else {
        let mut state = state;
        state.next_player = state.next_player.opposite();
        if state.last_skipped {
            return utility(state.board, player);
        }
        state.last_skipped = true;
        return max_value(state, player, alpha, beta);
    }
}

fn max_value(state: State, player: Player, alpha: &mut isize, beta: &mut isize) -> isize {
    let successors = legal_moves(state);

    let mut max = isize::MIN;
    for (_, succ) in successors {
        max = max.max(min_value(succ, player, alpha, beta));
        if max >= *beta {
            return max;
        } else {
            *beta = (*beta).max(max);
        }
    }

    if max != isize::MIN {
        return max;
    } else {
        let mut state = state;
        state.next_player = state.next_player.opposite();
        if state.last_skipped {
            return utility(state.board, player);
        }
        state.last_skipped = true;
        return min_value(state, player, alpha, beta);
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
