#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub ch: char,
    pub walkable: bool,
}

#[derive(Clone)]
pub struct Room {
    pub height: i32,
    pub width: i32,
    pub pos: Position,
    pub center: Position,
}

pub struct Entity {
    pub pos: Position,
    pub ch: char,
}
impl Entity {
    pub fn create_player(start_pos: Position) -> Entity {
        let new_player = Entity {
            pos: Position {
                x: start_pos.x,
                y: start_pos.y,
            },
            ch: '@',
        };
        return new_player;
    }
}