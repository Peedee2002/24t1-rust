#[warn(missing_docs)]
mod coordinate;
mod direction;

#[cfg(test)]
mod tests {
    use crate::{coordinate::Coordinate, direction::{CardinalDirection, Direction}};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn add_direction_with_coord() {
        let direction = Direction { x: 1, y: 1};
        let coord = Coordinate { x: 1, y: 1};
        assert_eq!(coord + direction, Coordinate {x: 2, y: 2});
    }

    #[test]
    fn create_directions_from_cardinals() {
        assert_eq!(Into::<Direction>::into(CardinalDirection::East), Direction { x: 1, y: 1})
    }
}
