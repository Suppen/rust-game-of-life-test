use std::fmt;

/// Possible states for a cell
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellState {
    /// A cell which is alive
    Alive,
    /// A cell which is dead
    Dead,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            CellState::Alive => "Alive",
            CellState::Dead => "Dead",
        };

        write!(f, "{}", res)
    }
}

impl CellState {
    /// Revives the cell, making sure it is alive
    ///
    /// # Examples
    /// ```
    /// use game_of_life::CellState;
    ///
    /// let mut cell = CellState::Dead;
    /// cell.revive();
    ///
    /// assert_eq!(cell, CellState::Alive);
    /// ```
    pub fn revive(&mut self) {
        *self = CellState::Alive;
    }

    /// Kills the cell, making sure it is dead
    ///
    /// # Examples
    /// ```
    /// use game_of_life::CellState;
    ///
    /// let mut cell = CellState::Alive;
    /// cell.kill();
    ///
    /// assert_eq!(cell, CellState::Dead);
    /// ```
    pub fn kill(&mut self) {
        *self = CellState::Dead;
    }

    /// Toggles the state of the cell, making a dead one come alive, and a live one dead
    ///
    /// # Examples
    /// ```
    /// use game_of_life::CellState;
    ///
    /// let mut cell = CellState::Dead;
    ///
    /// cell.toggle();
    ///
    /// assert_eq!(cell, CellState::Alive);
    ///
    /// cell.toggle();
    ///
    /// assert_eq!(cell, CellState::Dead);
    /// ```
    pub fn toggle(&mut self) {
        match self {
            CellState::Alive => self.kill(),
            CellState::Dead => self.revive(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn alive_displays_correctly() {
        let res = format!("{}", CellState::Alive);

        assert_eq!(res, "Alive");
    }

    #[test]
    fn dead_displays_correctly() {
        let res = format!("{}", CellState::Dead);

        assert_eq!(res, "Dead");
    }

    #[test]
    fn revive_makes_dead_cell_alive() {
        let mut cell = CellState::Dead;
        cell.revive();

        assert_eq!(cell, CellState::Alive);
    }

    #[test]
    fn revive_doesnt_change_live_cell() {
        let mut cell = CellState::Alive;
        cell.revive();

        assert_eq!(cell, CellState::Alive);
    }

    #[test]
    fn kill_makes_live_cell_dead() {
        let mut cell = CellState::Alive;
        cell.kill();

        assert_eq!(cell, CellState::Dead);
    }

    #[test]
    fn kill_doesnt_change_dead_cell() {
        let mut cell = CellState::Dead;
        cell.kill();

        assert_eq!(cell, CellState::Dead);
    }

    #[test]
    fn toggle_makes_dead_cell_alive() {
        let mut cell = CellState::Dead;
        cell.toggle();

        assert_eq!(cell, CellState::Alive);
    }

    #[test]
    fn toggle_makes_live_cell_dead() {
        let mut cell = CellState::Alive;
        cell.toggle();

        assert_eq!(cell, CellState::Dead);
    }
}
