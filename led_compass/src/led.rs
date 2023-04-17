#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

type ARRAY = [[u8; 5]; 5];

const NORTH : ARRAY = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 1, 0, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const NORTH_EAST: ARRAY = [
    [1, 1, 1, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 1],
];

const EAST: ARRAY = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
];

const SOUTH_EAST: ARRAY = [
    [0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [1, 0, 1, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 1, 1, 0, 0],
];

const SOUTH: ARRAY = [
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
];

const SOUTH_WEST: ARRAY = [
    [1, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 1, 0, 1],
    [0, 0, 0, 1, 1],
    [0, 0, 1, 1, 1]
];

const WEST: ARRAY = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

const NORTH_WEST: ARRAY = [
    [0, 0, 1, 1, 1],
    [0, 0, 0, 1, 1],
    [0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0],
];

pub (crate) fn direction_to_led(direction: Direction) -> ARRAY {
    match direction {
        Direction::North => NORTH,
        Direction::NorthWest => NORTH_WEST,
        Direction::South => SOUTH,
        Direction::SouthEast => SOUTH_EAST,
        Direction::SouthWest => SOUTH_WEST,
        Direction::NorthEast => NORTH_EAST,
        Direction::East => EAST,
        Direction::West => WEST,
    }
}
