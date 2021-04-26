use othello::*;
fn main() {
    dbg!(std::mem::size_of::<State>());
    dbg!(std::mem::size_of::<Player>());
    dbg!(std::mem::size_of::<Square>());
}
