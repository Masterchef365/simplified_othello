use std::fmt;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub enum Player {
    Dark,
    Light,
}

#[derive(Copy, Clone, Debug)]
pub enum Square {
    Dark,
    Light,
    Empty,
}

#[derive(Copy, Clone, Debug)]
pub struct State {
    pub board: Board,
    pub next_player: Player,
}

pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 4;

pub type Move = (usize, usize);

/// Returns all legal moves from this state
pub fn legal_moves(state: State) -> Vec<(Move, State)> {
    let mut moves = Vec::new();
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            if let Some(&Square::Empty) = state.board.get(x, y) {
                moves.extend(legal_moves_pos(state, x, y).map(|state| ((x, y), state)));
            }
        }
    }
    moves
}

fn legal_moves_pos(state: State, x: usize, y: usize) -> impl Iterator<Item = State> {
    DIRECTIONS
        .iter()
        .filter_map(move |&dir| legal_move_dir(state, x, y, dir))
}

fn legal_move_dir(mut state: State, mut x: usize, mut y: usize, (dx, dy): (isize, isize)) -> Option<State> {
    let mut saw_opposite = false;

    // Optimistically set the square
    *state.board.get_mut(x, y).unwrap() = match state.next_player {
        Player::Dark => Square::Dark,
        Player::Light => Square::Light,
    };

    // Step along the direction vector
    loop {
        // Go to the next square, or return None if out of bounds
        x = (x as isize + dx).try_into().ok()?;
        y = (y as isize + dy).try_into().ok()?;
        let square = state.board.get_mut(x, y)?;
        match (square, state.next_player) {
            // No anchor
            (Square::Empty, _) => return None,
            // We've met another square opposite our color, set it opposite
            (s@Square::Dark, Player::Light) => {
                saw_opposite = true;
                *s = Square::Light;
            }
            (s@Square::Light, Player::Dark) => {
                saw_opposite = true;
                *s = Square::Dark;
            }
            // We've met our anchor
            (Square::Light, Player::Light) => {
                if saw_opposite {
                    return Some(state);
                }
            }
            (Square::Dark, Player::Dark) => {
                if saw_opposite {
                    return Some(state);
                }
            }
        }
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

/// Game board
#[derive(Copy, Clone, Debug)]
pub struct Board([Square; WIDTH * HEIGHT]);

impl Board {
    /// Create an empty board
    pub fn empty() -> Self {
        Self([Square::Empty; WIDTH * HEIGHT])
    }

    /// Check if this is a valid position on the board, and if so return it's index
    fn coord(x: usize, y: usize) -> Option<usize> {
        (x < WIDTH && y < HEIGHT).then(|| x + WIDTH * y)
    }

    /// Get the peice at this coordinate, returns None if it is out of bounds
    pub fn get(&self, x: usize, y: usize) -> Option<&Square> {
        self.0.get(Self::coord(x, y)?)
    }

    /// Get the peice at this coordinate, returns None if it is out of bounds
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Square> {
        self.0.get_mut(Self::coord(x, y)?)
    }
}

impl State {
    /// Create a new game state
    pub fn new() -> Self {
        let mut board = Board::empty();

        // Center pattern
        let (brx, bry) = (WIDTH / 2, HEIGHT / 2);
        *board.get_mut(brx, bry - 1).unwrap() = Square::Light;
        *board.get_mut(brx - 1, bry - 1).unwrap() = Square::Dark;
        *board.get_mut(brx - 1, bry).unwrap() = Square::Light;
        *board.get_mut(brx, bry).unwrap() = Square::Dark;

        Self {
            board,
            next_player: Player::Dark,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Square::Dark => "X",
            Square::Light => "O",
            Square::Empty => ".",
        })
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Player::Dark => "Dark (X)",
            Player::Light => "Light (O)",
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..HEIGHT).rev() {
            write!(f, "{}:|", row)?;
            for col in 0..WIDTH {
                write!(f, " {}", self.get(col, row).unwrap())?;
            }
            writeln!(f)?;
        }

        write!(f, "   ")?;
        for _ in 0..WIDTH {
            write!(f, "--")?;
        }
        writeln!(f)?;

        write!(f, "   ")?;
        for col in 0..WIDTH {
            write!(f, " {}", col)?;
        }
        writeln!(f)
    }
}
