use std::convert::TryInto;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Player {
    Dark,
    Light,
}

impl Player {
    /// Yin and yang...
    pub fn opposite(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
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
pub type Successor = (Move, State);

/// Returns all legal moves from this state
pub fn legal_moves(state: State) -> Vec<Successor> {
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

/// Legal moves starting from `state`, position (x, y), and moving along the (dx, dy) diagonal
fn legal_move_dir(
    mut state: State,
    mut x: usize,
    mut y: usize,
    (dx, dy): (isize, isize),
) -> Option<State> {
    let mut saw_opposite = false;

    // Optimistically set the current square to our color, and game state to the opposite
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
            (s @ Square::Dark, Player::Light) => {
                saw_opposite = true;
                *s = Square::Light;
            }
            (s @ Square::Light, Player::Dark) => {
                saw_opposite = true;
                *s = Square::Dark;
            }
            // We've met our anchor
            (Square::Light, Player::Light) => {
                if saw_opposite {
                    state.next_player = Player::Dark;
                    return Some(state);
                } else {
                    return None;
                }
            }
            (Square::Dark, Player::Dark) => {
                if saw_opposite {
                    state.next_player = Player::Light;
                    return Some(state);
                } else {
                    return None;
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

    /// Return scores of (Dark, Light)
    pub fn scores(&self) -> (usize, usize) {
        let mut dark = 0;
        let mut light = 0;
        for sq in &self.0 {
            match sq {
                Square::Dark => dark += 1,
                Square::Light => light += 1,
                Square::Empty => ()
            }
        }
        (dark, light)
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "{} to play.", self.next_player)
    }
}


