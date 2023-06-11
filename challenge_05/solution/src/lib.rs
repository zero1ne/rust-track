// WIP

use std::fmt::{Display, Formatter};

/// The various errors we'll expect to return as a result of some invalid operation.
/// More details below:
///
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

/// The four directions in which a room can have neighbors. You can add more traits
/// implementations to make your life easier.
///
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Container for the rooms and more. We will mostly work with this structure.
///
pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    // TODO: need to refer challenge_04
    todo!();
}

#[test]
fn test_short() {
    let room = Room {
        name: String::from("X"),
        north: String::from("N"),
        south: String::from("S"),
        east: String::from("E"),
        west: String::from("W"),
    };

    let expected = "
    [ N ]
      |
    +-N-+
W - | X | - E
    +-S-+
      |
    [ S ]";

    assert_eq!(format!("{}", room), expected);
}