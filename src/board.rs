use crate::cell_state::CellState;
use crate::coord::Coord;

/// A rectangular board for a life-like game
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The cells on the board
    cells: Vec<Vec<CellState>>,
    /// Coordinates of all cells on the board
    cell_coords: Vec<Coord>,
}

impl Board {
    /// Creates a new board with the desired size, initializing all cells as dead
    ///
    /// # Examples
    /// ```
    /// use game_of_life::Board;
    ///
    /// let width = 15;
    /// let height = 10;
    /// let board = Board::new(width, height);
    ///
    /// assert_eq!(board.width(), width);
    /// assert_eq!(board.height(), height);
    /// ```
    ///
    /// # Panics
    /// If width or height is 0
    pub fn new(width: usize, height: usize) -> Board {
        if width == 0 || height == 0 {
            panic!("Width and height must be at least 1");
        }

        let mut rows = Vec::with_capacity(height);
        let mut cell_coords = Vec::with_capacity(width * height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);

            for x in 0..width {
                row.push(CellState::Dead);
                cell_coords.push(Coord::new(x, y));
            }

            rows.push(row);
        }

        Board {
            cell_coords,
            cells: rows,
        }
    }

    /// Creates a new board from a grid of cell states. Rows will be padded with dead cells to have
    /// the same length as the longest row
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, CellState, Coord};
    ///
    /// let grid = vec![
    ///     vec![CellState::Dead, CellState::Alive],
    ///     vec![CellState::Alive, CellState::Dead],
    ///     vec![CellState::Alive, CellState::Alive],
    /// ];
    ///
    /// let board = Board::from_grid(&grid);
    ///
    /// assert_eq!(board.width(), 2);
    /// assert_eq!(board.height(), 3);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 0)), &CellState::Dead);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 0)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 1)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 1)), &CellState::Dead);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 2)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 2)), &CellState::Alive);
    /// ```
    ///
    /// # Panics
    /// If there are no rows or no columns in the grid
    pub fn from_grid(grid: &Vec<Vec<CellState>>) -> Board {
        // Figure out the board's dimensions
        let height = grid.len();
        let width = grid.iter().map(Vec::len).max().unwrap_or(0);

        // Create the board itself
        let mut board = Board::new(width, height);

        // Make all live cells alive
        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell_state)| {
                if cell_state == &CellState::Alive {
                    board.revive_cell(&Coord::new(x, y));
                }
            })
        });

        board
    }

    /// Creates a new grid from a string. Newlines separate the rows, and all other characters
    /// except the alive character count as dead. Lines will be padded with dead cells to have
    /// the same length as the longest line, and a trailing newline counts as a dead row!
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let board_str = vec![
    ///    "#_",
    ///    "_#",
    ///    "#_",
    /// ].join("\n");
    ///
    /// let board = Board::from_str(&board_str, '#');
    ///
    /// assert_eq!(board.width(), 2);
    /// assert_eq!(board.height(), 3);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 0)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 0)), &CellState::Dead);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 1)), &CellState::Dead);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 1)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(0, 2)), &CellState::Alive);
    /// assert_eq!(board.get_cell_state(&Coord::new(1, 2)), &CellState::Dead);
    /// ```
    ///
    /// # Panics
    /// If the alive character is a newline character, or if there are no rows or no columns in the
    /// string
    pub fn from_str(string: &str, alive: char) -> Board {
        if alive == '\n' || alive == '\r' {
            panic!("The alive character cannot be a newline character");
        }

        // Convert the string to a grid
        let grid: Vec<Vec<CellState>> =
            string
                .replace("\r", "")
                .chars()
                .fold(Vec::new(), |mut rows, c| {
                    // If this is a newline, make a new row and move on to the next char
                    if c == '\n' {
                        rows.push(Vec::new());
                        return rows;
                    };

                    // Get the last avaliable row, or make a new one if there are no rows
                    let mut row = rows.pop().unwrap_or(Vec::new());

                    // Figure out the state of this cell
                    let cell_state = if c == alive {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    };

                    // Add it to the row, add the row (back) to the rows vec, and return it
                    row.push(cell_state);
                    rows.push(row);
                    rows
                });

        Board::from_grid(&grid)
    }

    /// Creates a string representation of the board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::Board;
    ///
    /// let board_str = vec![
    ///     "#___#",
    ///     "_#_#_",
    ///     "__#__",
    ///     "_#_#_",
    ///     "#___#",
    /// ].join("\n");
    ///
    /// let board = Board::from_str(&board_str, '#');
    ///
    /// assert_eq!(board.to_str('#', '_'), board_str);
    /// ```
    pub fn to_str(&self, alive: char, dead: char) -> String {
        if alive == '\n' || alive == '\r' {
            panic!("The alive character cannot be a newline character");
        }
        if dead == '\n' || dead == '\r' {
            panic!("The alive character cannot be a newline character");
        }

        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell_state| match cell_state {
                        CellState::Alive => alive,
                        CellState::Dead => dead,
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// The width of the board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::Board;
    ///
    /// let width = 15;
    /// let height = 10;
    /// let board = Board::new(width, height);
    ///
    /// assert_eq!(board.width(), width);
    /// ```
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    /// The height of the board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::Board;
    ///
    /// let width = 15;
    /// let height = 10;
    /// let board = Board::new(width, height);
    ///
    /// assert_eq!(board.height(), height);
    /// ```
    pub fn height(&self) -> usize {
        self.cells.len()
    }

    /// Set of all cell coordinates on the board
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord};
    ///
    /// let board = Board::from_str(&vec![
    ///     "#_",
    ///     "_#",
    /// ].join("\n"), '#');
    ///
    /// let coords = board.cell_coords();
    ///
    /// assert_eq!(coords.len(), 4);
    /// assert!(coords.contains(&Coord::new(0, 0)));
    /// assert!(coords.contains(&Coord::new(0, 1)));
    /// assert!(coords.contains(&Coord::new(1, 0)));
    /// assert!(coords.contains(&Coord::new(1, 1)));
    /// ```
    pub fn cell_coords(&self) -> &Vec<Coord> {
        &self.cell_coords
    }

    /// Gets a reference to the state of the cell at the given coordinate
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let board = Board::from_str(&vec![
    ///     "#_",
    ///     "_#",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(0, 0);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Alive);
    /// ```
    pub fn get_cell_state(&self, coord: &Coord) -> &CellState {
        &self.cells[coord.y][coord.x]
    }

    /// Gets a mutable reference to the state of the cell at the given coordinate
    fn get_cell_state_mut(&mut self, coord: &Coord) -> &mut CellState {
        &mut self.cells[coord.y][coord.x]
    }

    /// Kills the cell at the given coordinate, making sure it is dead
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let mut board = Board::from_str(&vec![
    ///     "##",
    ///     "##",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(0, 0);
    /// board.kill_cell(&coord);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Dead);
    /// ```
    pub fn kill_cell(&mut self, coord: &Coord) {
        self.get_cell_state_mut(coord).kill();
    }

    /// Revives the cell at the given coordinate, making sure it is alive
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let mut board = Board::from_str(&vec![
    ///     "__",
    ///     "__",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(0, 0);
    /// board.revive_cell(&coord);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Alive);
    /// ```
    pub fn revive_cell(&mut self, coord: &Coord) {
        self.get_cell_state_mut(coord).revive();
    }

    /// Toggles the state of the cell at the given coordinate
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let mut board = Board::from_str(&vec![
    ///     "__",
    ///     "__",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(0, 0);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Dead);
    ///
    /// board.toggle_cell(&coord);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Alive);
    ///
    /// board.toggle_cell(&coord);
    ///
    /// assert_eq!(board.get_cell_state(&coord), &CellState::Dead);
    /// ```
    pub fn toggle_cell(&mut self, coord: &Coord) {
        self.get_cell_state_mut(coord).toggle();
    }

    /// Gets the neighbours of a given coord, wrapping around if it is on an edge
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord};
    ///
    /// let board = Board::from_str(&vec![
    ///     "#__",
    ///     "_#_",
    ///     "__#",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(0, 0);
    /// let neighbour_coords = board.get_neighbour_coords(&coord);
    ///
    /// assert!(neighbour_coords.contains(&Coord::new(1, 1)));
    /// assert!(neighbour_coords.contains(&Coord::new(1, board.height() - 1)));
    /// assert!(neighbour_coords.contains(&Coord::new(board.width() - 1, 1)));
    /// // and 5 more
    /// ```
    pub fn get_neighbour_coords(&self, coord: &Coord) -> Vec<Coord> {
        let x = coord.x;
        let xa = if coord.x == self.width() - 1 {
            0
        } else {
            coord.x + 1
        };
        let xs = if coord.x == 0 {
            self.width() - 1
        } else {
            coord.x - 1
        };
        let y = coord.y;
        let ya = if coord.y == self.height() - 1 {
            0
        } else {
            coord.y + 1
        };
        let ys = if coord.y == 0 {
            self.height() - 1
        } else {
            coord.y - 1
        };

        vec![
            Coord::new(xs, y),  // West
            Coord::new(xa, y),  // East
            Coord::new(x, ys),  // North
            Coord::new(x, ya),  // South
            Coord::new(xs, ys), // North West
            Coord::new(xa, ys), // North East
            Coord::new(xs, ya), // South West
            Coord::new(xa, ya), // South East
        ]
    }

    /// Gets the count of live neighbours of a cell
    ///
    /// # Examples
    /// ```
    /// use game_of_life::{Board, Coord, CellState};
    ///
    /// let board = Board::from_str(&vec![
    ///     "_____",
    ///     "_##__",
    ///     "_____",
    ///     "__#__",
    ///     "_____",
    /// ].join("\n"), '#');
    ///
    /// let coord = Coord::new(2, 2);
    ///
    /// assert_eq!(board.get_live_neighbours_of(&coord), 3);
    /// ```
    pub fn get_live_neighbours_of(&self, coord: &Coord) -> u8 {
        self.get_neighbour_coords(coord)
            .iter()
            .filter(|c| self.get_cell_state(c) == &CellState::Alive)
            .count() as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn creating_board_with_0_width_panics() {
        Board::new(0, 10);
    }

    #[test]
    #[should_panic]
    fn creating_board_with_0_height_panics() {
        Board::new(10, 0);
    }

    #[test]
    #[should_panic]
    fn creating_board_with_0_cells_panics() {
        Board::new(0, 0);
    }

    #[test]
    fn all_cells_initially_dead() {
        let height = 10;
        let width = 5;
        let board = Board::new(width, height);

        println!("{:?}", board.cell_coords());

        let all_dead = board
            .cell_coords()
            .iter()
            .map(|c| board.get_cell_state(c))
            .all(|s| s == &CellState::Dead);

        assert!(all_dead);
    }

    #[test]
    #[should_panic]
    fn creating_board_from_grid_with_0_cells_panics() {
        Board::from_grid(&Vec::new());
    }

    #[test]
    fn creating_board_from_grid_gets_correct_dimensions() {
        let grid = vec![vec![CellState::Alive, CellState::Dead]];

        let board = Board::from_grid(&grid);

        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 1);
    }

    #[test]
    fn cearing_board_from_grid_with_uneven_rows_works() {
        let grid = vec![
            vec![],
            vec![CellState::Dead, CellState::Alive],
            vec![CellState::Dead],
        ];

        let board = Board::from_grid(&grid);

        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 3);
    }

    #[test]
    #[should_panic]
    fn creating_board_from_empty_string_panics() {
        Board::from_str("", '#');
    }

    #[test]
    #[should_panic]
    fn creating_board_from_string_with_only_newlines_panics() {
        Board::from_str("\n\n\n\n\n", '#');
    }

    #[test]
    #[should_panic]
    fn creating_board_from_string_with_only_carriage_returns_panics() {
        Board::from_str("\r\r\r\r", '#');
    }

    #[test]
    #[should_panic]
    fn creating_board_from_string_with_newline_as_alive_char_panics() {
        Board::from_str("#", '\n');
    }

    #[test]
    #[should_panic]
    fn creating_board_from_string_with_carriage_return_as_alive_char_panics() {
        Board::from_str("#", '\r');
    }

    #[test]
    fn creating_board_from_string_ignores_carriage_returns() {
        let board = Board::from_str(&vec!["##", "##"].join("\n"), '#');

        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 2);
    }

    #[test]
    fn creating_board_from_string_trailing_newline() {
        // Trailing newline causes a bottom row with dead cells
        let board = Board::from_str("##\n##\n", '#');

        assert_eq!(board.width(), 2);
        assert_eq!(board.height(), 3);
    }

    #[test]
    fn creating_board_from_string_counts_non_live_characters_as_dead() {
        let board = Board::from_str(&vec!["*T#5.", " #_#?"].join("\n"), '#');

        assert_eq!(board.get_cell_state(&Coord::new(0, 0)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(1, 0)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(2, 0)), &CellState::Alive);
        assert_eq!(board.get_cell_state(&Coord::new(3, 0)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(4, 0)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(0, 1)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(1, 1)), &CellState::Alive);
        assert_eq!(board.get_cell_state(&Coord::new(2, 1)), &CellState::Dead);
        assert_eq!(board.get_cell_state(&Coord::new(3, 1)), &CellState::Alive);
        assert_eq!(board.get_cell_state(&Coord::new(4, 1)), &CellState::Dead);
    }

    #[test]
    #[should_panic]
    fn to_str_panics_if_alive_char_is_newline() {
        let board = Board::new(1, 1);

        board.to_str('\n', ' ');
    }

    #[test]
    #[should_panic]
    fn to_str_panics_if_alive_char_is_carriage_return() {
        let board = Board::new(1, 1);

        board.to_str('\r', ' ');
    }

    #[test]
    #[should_panic]
    fn to_str_panics_if_dead_char_is_newline() {
        let board = Board::new(1, 1);

        board.to_str('#', '\n');
    }

    #[test]
    #[should_panic]
    fn to_str_panics_if_dead_char_is_carriage_return() {
        let board = Board::new(1, 1);

        board.to_str('#', '\r');
    }

    #[test]
    fn gets_and_sets_state_of_cell() {
        let height = 10;
        let width = 5;
        let mut board = Board::new(width, height);
        let coord = Coord::new(0, 0);

        assert_eq!(board.get_cell_state(&coord), &CellState::Dead);

        board.revive_cell(&coord);

        assert_eq!(board.get_cell_state(&coord), &CellState::Alive);

        board.kill_cell(&coord);

        assert_eq!(board.get_cell_state(&coord), &CellState::Dead);

        board.toggle_cell(&coord);

        assert_eq!(board.get_cell_state(&coord), &CellState::Alive);

        board.toggle_cell(&coord);

        assert_eq!(board.get_cell_state(&coord), &CellState::Dead);
    }

    #[test]
    fn neighbours_should_be_correct_in_middle_of_board() {
        let height = 10;
        let width = 5;
        let board = Board::new(width, height);
        let coord = Coord::new(1, 1);

        let expected_neighbours = vec![
            Coord::new(0, 1),
            Coord::new(2, 1),
            Coord::new(1, 0),
            Coord::new(1, 2),
            Coord::new(0, 0),
            Coord::new(0, 2),
            Coord::new(2, 0),
            Coord::new(2, 2),
        ];

        let neighbours = board.get_neighbour_coords(&coord);

        for expected in expected_neighbours {
            assert!(neighbours.contains(&expected));
        }
    }

    #[test]
    fn neighbours_should_be_correct_upper_left() {
        let height = 10;
        let width = 5;
        let board = Board::new(width, height);
        let coord = Coord::new(0, 0);

        let expected_neighbours = vec![
            Coord::new(width - 1, 0),
            Coord::new(1, 0),
            Coord::new(0, height - 1),
            Coord::new(0, 1),
            Coord::new(width - 1, height - 1),
            Coord::new(width - 1, 1),
            Coord::new(1, height - 1),
            Coord::new(1, 1),
        ];

        let neighbours = board.get_neighbour_coords(&coord);

        for expected in expected_neighbours {
            assert!(neighbours.contains(&expected));
        }
    }

    #[test]
    fn neighbours_should_be_correct_lower_right() {
        let height = 10;
        let width = 5;
        let board = Board::new(width, height);
        let coord = Coord::new(width - 1, height - 1);

        let expected_neighbours = vec![
            Coord::new(0, 0),
            Coord::new(0, height - 1),
            Coord::new(0, height - 2),
            Coord::new(width - 1, 0),
            Coord::new(width - 2, 0),
            Coord::new(width - 2, height - 2),
            Coord::new(width - 1, height - 2),
            Coord::new(width - 2, height - 1),
        ];

        let neighbours = board.get_neighbour_coords(&coord);

        println!("{:?}", neighbours);
        println!("{:?}", expected_neighbours);

        for expected in expected_neighbours {
            assert!(neighbours.contains(&expected));
        }
    }

    #[test]
    fn three_live_neighbours() {
        let board = Board::from_str(
            &vec!["_____", "_##__", "_____", "__#__", "_____"].join("\n"),
            '#',
        );

        let coord = Coord::new(2, 2);

        assert_eq!(board.get_live_neighbours_of(&coord), 3);
    }
}
