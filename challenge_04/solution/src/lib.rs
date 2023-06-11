use std::io::BufRead;

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

/// A room in the dungeons. It is defined by name only, although in a more interesting implementation it might
/// holds items, enemies...
///
pub struct Room {
    pub name: String,
    pub neighbors: Vec<(Direction, String)>,
}

/// Container for the rooms and more. We will mostly work with this structure.
///
pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    /// Constructs an empty Dungeon that does not have any rooms.
    ///
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }

    /// Adding room to Dungeon named `name`. Returns `Ok(())` on success. If there is already a room with
    /// such a name, we expect you to return `Errors::DuplicateRoom` with the name.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        // Check if a room with the same name already exists
        if self.rooms.iter().any(|room| room.name == name) {
            return Err(Errors::DuplicateRoom(name.to_owned()));
        }

        // Create a new room and add it to the dungeon
        let new_room = Room { name: name.to_owned(), neighbors: vec![] };
        self.rooms.push(new_room);

        Ok(())
    }

    /// Reading a given room -- when we call `get_room`, we expect a reference to `Room`
    /// the structure with this name.
    ///
    /// If there is no such room, we expect `Errors::UnknownRoom` with the supplied name.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        self.rooms.iter()
            .find(|room| room.name == room_name)
            .ok_or(Errors::UnknownRoom(room_name.to_owned()))
    }

    /// Add a room neighbor. After calling the function, we expect the room with a name
    /// `room_name` has a connection in the `direction` direction with the room named `other_room_name`.
    ///
    /// We also expect `other_room_name` to relate to `room_name` in the *reverse* direction.
    ///
    /// A successful result is `Ok(())`. In the event that one of the two rooms does not exist, we expect an error
    /// `Errors::UnknownRoom` with the corresponding missing room name. If both are missing, then
    /// return the one you check first.
    ///
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        // Check if both rooms exist
        let (room_index, other_room_index) = match (
            self.rooms.iter().position(|room| room.name == room_name),
            self.rooms.iter().position(|room| room.name == other_room_name),
        ) {
            (Some(room_index), Some(other_room_index)) => (room_index, other_room_index),
            (None, Some(_)) => return Err(Errors::UnknownRoom(room_name.to_owned())),
            (Some(_), None) => return Err(Errors::UnknownRoom(other_room_name.to_owned())),
            (None, None) => return Err(Errors::UnknownRoom(room_name.to_owned())),
        };

        // Add the neighbor connections in both directions
        self.rooms[room_index]
            .neighbors
            .push((direction, other_room_name.to_owned()));
        self.rooms[other_room_index]
            .neighbors
            .push((reverse_direction(direction), room_name.to_owned()));

        Ok(())
    }

    /// Reading neighbor of room named `room_name` in direction `direction`. There are several here
    /// the output variant:
    ///
    /// - If the room passed does not exist, we expect an `Errors::UnknownRoom` error
    /// - If the given room has no neighbor in that direction, Ok(None) is the meaningful result
    /// - Otherwise, we wait for a reference to the `Room` structure of the neighbor in question, wrapped in `Ok(Some(`.
    ///
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        self.rooms.iter()
            .find(|room| room.name == room_name)
            .ok_or(Errors::UnknownRoom(room_name.to_owned()))
            .and_then(|room| {
                match room.neighbors.iter().rev().find(|(dir, _)| *dir == direction) {
                    Some((_, name)) => self.get_room(name).map(Some),
                    None => Ok(None),
                }
            })
    }

    /// We read the dungeon structure from something that implements `BufRead`. This could be it
    /// file, or, if we're testing, it could just be a collection of bytes.
    ///
    /// A successful result returns the newly created dungeon packaged in `Ok`.
    ///
    /// See below for an explanation of the errors we expect.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        todo!()
    }

    /// Searches for a path from `start_room_name` to `end_room_name` and returns it in a vector packed in
    /// `Ok(Some(` if found.
    ///
    /// If there is no path between these two rooms, returns `Ok(None)`.
    ///
    /// If reading rooms at some point returns an error, we expect you to raise the error.
    ///
    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {
        todo!()
    }
}

fn reverse_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

/// match_prefix("- ", "- Foo") //=> Some("Foo")
/// match_prefix("- ", "Bar")   //=> None
///
fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    todo!()
}

#[test]
fn test_basic() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.add_room("Magic Lab").unwrap();

    dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();
    dungeon.set_link("Hallway", Direction::West, "Magic Lab").unwrap();

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
    assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Magic Lab");
}