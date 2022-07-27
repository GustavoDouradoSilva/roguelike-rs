use crate::*;

pub struct Creature {
    pub name: String,
    pub pos: Position,
    pub draw: Draw,
}

impl Creature {
    /*
    pub fn new(&mut self) -> Creature {
        Creature {
            name: "*".to_string(),
            pos: Position { x: 0, y: 0 },
            draw: Draw { ch: '*', color: 0 },
        }
    }
    */
    /*
    // move incrementally self.pos += new_pos.x
    fn move_by(&mut self, new_pos: &Position, map: &Vec<Vec<TileType>>) {
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
        self.pos.x += new_pos.x;
        self.pos.y += new_pos.y;
    }
    */
    // teleports self.pos = new_pos
    pub fn move_to(&mut self, new_pos: &Position, map: &Vec<Vec<TileType>>) {
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
        self.pos = new_pos.clone();
    }

    pub fn draw(&mut self, window: &mut pancurses::Window) {
        window.attron(pancurses::COLOR_PAIR(self.draw.color));
        window.mvaddch(self.pos.y, self.pos.x, self.draw.ch);
        window.attroff(pancurses::COLOR_PAIR(self.draw.color));
    }

    pub fn handle_input(&mut self, input: char, map: &mut Vec<Vec<TileType>>) {
        let mut new_pos = self.pos.clone();

        match input {
            'w' => new_pos.y -= 1,
            'a' => new_pos.x -= 1,
            's' => new_pos.y += 1,
            'd' => new_pos.x += 1,
            _ => return,
        }

        self.move_to(&new_pos, &map);
    }
}
