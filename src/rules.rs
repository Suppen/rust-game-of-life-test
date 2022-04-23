/// Rules for a Game of Life
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rules {
    /// Number of neighbour cells which must be alive for a cell to be born
    pub b: Vec<u8>,
    /// Number of neighbour cells which must be alive for a cell to survive
    pub s: Vec<u8>,
}

impl Rules {
    /// Checks whether the rules says a cell with the given number of live neighbours survives to
    /// next generation
    ///
    /// # Examples
    /// ```
    /// use game_of_life::rules;
    ///
    /// let rules = rules::conways();
    ///
    /// assert!(!rules.survives(0));
    /// assert!(rules.survives(2));
    /// assert!(rules.survives(3));
    /// assert!(!rules.survives(4));
    /// ```
    pub fn survives(&self, live_neighbours: u8) -> bool {
        self.s.contains(&live_neighbours)
    }

    /// Checks whether the rules says a cell with the given number of live neighbours survives to
    /// next generation
    ///
    /// # Examples
    /// ```
    /// use game_of_life::rules;
    ///
    /// let rules = rules::conways();
    ///
    /// assert!(!rules.birthed(0));
    /// assert!(!rules.birthed(2));
    /// assert!(rules.birthed(3));
    /// assert!(!rules.birthed(4));
    /// ```
    pub fn birthed(&self, live_neighbours: u8) -> bool {
        self.b.contains(&live_neighbours)
    }
}

/// Rules for the original Conway's Game of Life
pub fn conways() -> Rules {
    Rules {
        b: vec![3],
        s: vec![2, 3],
    }
}
