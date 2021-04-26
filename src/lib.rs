use std::fmt;

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

pub fn legal_moves(state: &State) -> Vec<(Move, State)> {
    let mut moves = Vec::new();
    for row in 0..WIDTH {
        for col in 0..HEIGHT {
            for &(dx, dy) in &DIRECTIONS {

            }
        }
    }
    moves
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
        let (tlx, tly) = (WIDTH / 2, HEIGHT / 2);
        *board.get_mut(tlx, tly).unwrap() = Square::Light;
        *board.get_mut(tlx + 1, tly).unwrap() = Square::Dark;
        *board.get_mut(tlx + 1, tly + 1).unwrap() = Square::Light;
        *board.get_mut(tlx, tly + 1).unwrap() = Square::Dark;

        Self {
            board,
            next_player: Player::Dark,
        }
    }
}

/*
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
*/

/*
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn vector(&self) -> (isize, isize) {
        match self {
            Self::N => (0, 1),
            Self::NE => (1, 1),
            Self::E => (1, 0),
            Self::SE => (1, -1),
            Self::S => (0, -1),
            Self::SW => (-1, -1),
            Self::W => (-1, 0),
            Self::NW => (-1, 1),
        }
    }
}
*/
