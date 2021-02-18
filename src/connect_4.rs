/*
    Illustration of Structs and Enums:
    Implementing the game of Connect 4
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

const BOARD_LEN: usize = 10;
const BOARD_HGT: usize = 5;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    board: [Vec<Player>; BOARD_LEN],
    // Alternative: Vec<Vec<Player>>, but
    // the above is technically a bit more accurate as we don't need dynamic
    // modification.
    to_play: Player,
}

// Instead of using the #[derive(...)] we can also implement traits manually
impl Default for Game {
    fn default() -> Self {
        Self { board: Default::default(), to_play: Player::X }
    }
}

impl Game {
    // pub fn: public API
    // fn: internal API

    pub fn new() -> Self {
        Default::default()
    }

    // Note: we shouldn't require ourselves to take &self
    // as a parameter if it isn't needed! E.g.:
    fn in_range(col: usize, row: usize) -> bool {
        col <= BOARD_LEN && row <= BOARD_HGT
    }

    pub fn get(&self, col: usize, row: usize) -> Option<Player> {
        debug_assert!(Self::in_range(col, row));
        self.board[col].get(row).cloned()
    }
    pub fn playable(&self, col: usize) -> bool {
        debug_assert!(col <= BOARD_LEN);
        self.board[col].len() < BOARD_HGT
    }
    pub fn play(&mut self, col: usize, player: Player) {
        debug_assert!(col <= BOARD_LEN);
        debug_assert!(self.playable(col));
        self.board[col].push(player);
    }

    /*
        Useful to know: implementing lightweight iterators over your
        data structures
    */

    pub fn valid_plays(&self) -> impl Iterator<Item = usize> + '_ {
        (0..BOARD_LEN).filter(move |&i| self.playable(i))
    }

    fn cells() -> impl Iterator<Item = (usize, usize)> {
        // To Discuss: The 'move' keyword
        (0..BOARD_LEN).flat_map(|i| (0..BOARD_HGT).map(move |j| (i, j)))
    }

    fn blocks_of_four() -> impl Iterator<Item = [(usize, usize); 4]> {
        let horiz = Self::cells()
            .map(|(i, j)| [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)]);
        let vert = Self::cells()
            .map(|(i, j)| [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)]);
        let diag1 = Self::cells().map(|(i, j)| {
            [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)]
        });
        let diag2 = Self::cells().map(|(i, j)| {
            [(i, j + 3), (i + 1, j + 2), (i + 2, j + 1), (i + 3, j)]
        });
        horiz
            .chain(vert)
            .chain(diag1)
            .chain(diag2)
            .filter(|&blck| Self::in_range(blck[0].0, blck[0].1))
            .filter(|&blck| Self::in_range(blck[3].0, blck[3].1))
    }

    pub fn winner(&self) -> Option<Player> {
        for blck in Self::blocks_of_four() {
            for &player in &[Player::X, Player::O] {
                if blck.iter().all(|&(i, j)| self.get(i, j) == Some(player)) {
                    return Some(player);
                }
            }
        }
        None
    }
}
