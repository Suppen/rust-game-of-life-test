/// Coordinates on a rectangular Game of Life board
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Creates a new coordinate
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_a_coord_with_expected_components() {
        let x = 10;
        let y = 5;
        let coord = Coord::new(x, y);

        assert_eq!(coord.x, x);
        assert_eq!(coord.y, y);
    }
}
