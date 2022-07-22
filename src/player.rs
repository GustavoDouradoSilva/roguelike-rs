use crate::*;

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

    pub fn handle_input(&mut self, input: char, map: &mut Vec<Vec<TileType>>) {
        let mut new_pos = Position {
            x: self.pos.x,
            y: self.pos.y,
        };
        match input {
            'w' => new_pos.y -= 1,
            'a' => new_pos.x -= 1,
            's' => new_pos.y += 1,
            'd' => new_pos.x += 1,
            _ => return,
        }
        self.move_player(&mut new_pos, map)
    }

    pub fn move_player(&mut self, new_pos: &mut Position, map: &mut Vec<Vec<TileType>>) {
        if new_pos.x < 0
            || new_pos.x >= map::MAP_WIDTH as i32
            || new_pos.y < 0
            || new_pos.y >= map::MAP_HEIGHT as i32
        {
            return;
        }
        if !map[new_pos.y as usize][new_pos.x as usize].walkable {
            return;
        }
        self.pos.x = new_pos.x;
        self.pos.y = new_pos.y;
    }
}
