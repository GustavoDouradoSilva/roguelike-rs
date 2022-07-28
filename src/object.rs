use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Draw {
    pub ch: char,
    pub color: u32,
}

#[derive(Debug)]
pub struct Object {
    draw: Draw,
    name: String,
    pos: Position,
}

impl Object {
    pub fn new(name: String, pos: Position, draw: Draw) -> Object {
        Object {
            name,
            pos,
            draw,
        }
    }

    pub fn move_to(&mut self, new_pos: Position, game: &mut Game) {
        if new_pos.x < 0
            || new_pos.x >= map::MAP_WIDTH as i32
            || new_pos.y < 0
            || new_pos.y >= map::MAP_HEIGHT as i32
        {
            return;
        }
        if !game.map[new_pos.y as usize][new_pos.x as usize].walkable {
            return;
        }
        self.pos = new_pos.clone();

        write_log(format!("{} moved to {:?}", self.name, self.pos), game);
    }
    pub fn draw(&self, window: &mut pancurses::Window) {
        window.attron(self.draw.color);
        window.mvaddch(self.pos.y, self.pos.x, self.draw.ch);
        window.attroff(self.draw.color);
    }

    pub fn handle_input(&mut self, input: char, game: &mut Game) {
        let mut new_pos = self.pos.clone();

        match input {
            'w' => new_pos.y -= 1,
            'a' => new_pos.x -= 1,
            's' => new_pos.y += 1,
            'd' => new_pos.x += 1,
            _ => return,
        }

        self.move_to(new_pos, game);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool
}
/* 
impl  Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true
        }
    }
}
*/

