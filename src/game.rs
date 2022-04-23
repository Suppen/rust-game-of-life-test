use crate::board::Board;
use crate::cell_state::CellState;
use crate::coord::Coord;
use crate::rules::Rules;

/// A life-like game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    rules: Rules,
}

impl Game {
    /// Creates a new Game
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, rules};
    ///
    /// let game = Game::new(
    ///     Board::from_str(&vec![
    ///         "#_",
    ///         "_#",
    ///     ].join("\n"), '#'),
    ///     rules::conways()
    /// );
    /// ```
    pub fn new(board: Board, rules: Rules) -> Game {
        Game { board, rules }
    }

    /// Gets a reference to the game's board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, rules};
    ///
    /// let game = Game::new(
    ///     Board::from_str(&vec![
    ///         "#_",
    ///         "_#",
    ///     ].join("\n"), '#'),
    ///     rules::conways()
    /// );
    ///
    /// let board = game.board();
    /// ```
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Gets a reference to the game's rules
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, rules};
    ///
    /// let game = Game::new(
    ///     Board::from_str(&vec![
    ///         "#_",
    ///         "_#",
    ///     ].join("\n"), '#'),
    ///     rules::conways()
    /// );
    ///
    /// let rules = game.rules();
    /// ```
    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    /// Gets a mutable reference to the game's board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, Coord, rules};
    ///
    /// let mut game = Game::new(
    ///     Board::from_str(&vec![
    ///         "#_",
    ///         "_#",
    ///     ].join("\n"), '#'),
    ///     rules::conways()
    /// );
    ///
    /// let mut board = game.mut_board();
    ///
    /// board.revive_cell(&Coord::new(0, 0));
    /// ```
    pub fn mut_board(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Makes a map of all coordinates which will be toggled in the next iteration of this game
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, Coord, CellState, rules};
    ///
    /// let board = Board::from_str(&vec![
    ///     "_____",
    ///     "__#__",
    ///     "__#__",
    ///     "__#__",
    ///     "_____",
    ///     ].join("\n"), '#');
    ///
    /// let game = Game::new(board, rules::conways());
    ///
    /// let next_gen_toggle_coords = game.next_gen_toggles();
    ///
    /// // The top and bottom of the line will be toggled, together with the left and right side of
    /// // the center
    /// assert_eq!(next_gen_toggle_coords.len(), 4);
    /// assert!(next_gen_toggle_coords.contains(&&Coord::new(2, 1)));
    /// assert!(next_gen_toggle_coords.contains(&&Coord::new(2, 3)));
    /// assert!(next_gen_toggle_coords.contains(&&Coord::new(1, 2)));
    /// assert!(next_gen_toggle_coords.contains(&&Coord::new(3, 2)));
    /// ```
    pub fn next_gen_toggles(&self) -> Vec<&Coord> {
        let board = self.board();

        board
            .cell_coords()
            .iter()
            .map(|c| (c, board.get_cell_state(c), board.get_live_neighbours_of(c)))
            .filter(|(_, cell_state, live_neighbours)| match cell_state {
                CellState::Alive => !self.rules().survives(*live_neighbours),
                CellState::Dead => self.rules().birthed(*live_neighbours),
            })
            .map(|(c, _, _)| c)
            .collect()
    }

    /// Applies a list of coordinates to toggle on the game's board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, Coord, CellState, rules};
    ///
    /// let board = Board::from_str(&vec![
    ///     "_____",
    ///     "_____",
    ///     "__#__",
    ///     "_____",
    ///     "_____",
    /// ].join("\n"), '#');
    ///
    /// let mut game = Game::new(board, rules::conways());
    ///
    /// let coord1 = Coord::new(1, 1);
    /// let coord2 = Coord::new(2, 2);
    ///
    /// let toggle_coords = vec![
    ///     &coord1,
    ///     &coord2,
    /// ];
    ///
    /// game.apply_toggles(&toggle_coords);
    ///
    /// assert_eq!(game.board().get_cell_state(&coord1), &CellState::Alive);
    /// assert_eq!(game.board().get_cell_state(&coord2), &CellState::Dead);
    /// ```
    pub fn apply_toggles(&mut self, toggles: &Vec<&Coord>) {
        for coord in toggles {
            self.board.toggle_cell(coord);
        }
    }

    /// Advances the game to the next generation
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Game, Board, rules};
    ///
    /// let mut game = Game::new(
    ///     Board::from_str(&vec![
    ///         "_____",
    ///         "__#__",
    ///         "__#__",
    ///         "__#__",
    ///         "_____",
    ///     ].join("\n"), '#'),
    ///     rules::conways()
    /// );
    ///
    /// game.advance_to_next_gen();
    ///
    /// let expected_board_str = vec![
    ///     "_____",
    ///     "_____",
    ///     "_###_",
    ///     "_____",
    ///     "_____",
    /// ].join("\n");
    ///
    /// assert_eq!(game.board().to_str('#', '_'), expected_board_str);
    /// ```
    pub fn advance_to_next_gen(&mut self) {
        let toggles = self.next_gen_toggles();

        // XXX `toggles` contains references to `self`, so `self.apply_toggles` complains about
        // using mutable references while immutable references exists. The references are to
        // `self.board.cell_coords`, which will not be read or changed in this method.
        // To get around this, we clone the coordinates and make a new toggle vector. Is there
        // a better way?
        let mut coords = Vec::with_capacity(toggles.len());
        for coord in toggles {
            let c = coord.clone();
            coords.push(c);
        }
        let toggles: Vec<&Coord> = coords.iter().map(|c| c).collect();

        self.apply_toggles(&toggles);
    }
}
