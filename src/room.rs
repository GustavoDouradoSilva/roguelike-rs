use crate::{object::*, *};

#[derive(Clone)]
pub struct Room {
    pub height: i32,
    pub width: i32,
    pub pos: Position,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            height: height,
            width: width,
            pos: Position { x: x, y: y },
        }
    }

    pub fn center(&self) -> Position {
        Position {
            x: self.pos.x + self.width / 2,
            y: self.pos.y + self.height / 2,
        }
    }
    /*
    pub fn intersecst_with(&self, other: &Room) -> bool {
        let center_a = self.center();
        let center_b = other.center();

        let x_overlap = (center_a.x - center_b.x).abs() < (self.width / 2 + other.width / 2);
        let y_overlap = (center_a.y - center_b.y).abs() < (self.height / 2 + other.height / 2);

        x_overlap && y_overlap
    }
    */
    fn intersects_with(&self, other: &Room) -> bool {
        (self.width <= other.pos.x)
            && (self.pos.x >= other.width)
            && (self.height <= other.pos.y)
            && (self.pos.y >= other.height)
    }
    pub fn verify_room(room: Room, rooms: &mut Vec<Room>, game: &mut Game) {
        if rooms.len() < 2 {
            return;
        }
        for i in 0..(rooms.len() - 1) {
            if room.intersects_with(&rooms[i]) {
                rooms.remove(rooms.len() - 1);
                write_log(format!("room:{} removed", rooms.len()), &mut game.log_file);
            }
        }
    }

    pub fn new_random() -> Self {
        use rand::Rng;
        Room {
            pos: Position {
                x: rand::thread_rng().gen_range(1..(MAP_WIDTH - 20) as i32),
                y: rand::thread_rng().gen_range(1..(MAP_HEIGHT - 10)) as i32,
            },
            height: rand::thread_rng().gen_range(3..9),
            width: rand::thread_rng().gen_range(5..19),
        }
    }
}

pub fn add_room_to_map(room: &Room, map: &mut Vec<Vec<TileType>>) {
    for y in room.pos.y..room.pos.y + room.height {
        for x in room.pos.x..room.pos.x + room.width {
            map[y as usize][x as usize] = TileType {
                ch: ' ',
                walkable: true,
                color: pancurses::COLOR_PAIR(crate::engine::FLOOR_COLOR),
            };
        }
    }
}

/*
pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Vec<Vec<TileType>>) {
    for x in std::cmp::min(x1, x2)..std::cmp::max(x1, x2) {
        map[y as usize][x as usize] = TileType {
            ch: ' ',
            walkable: true,
            color: pancurses::COLOR_PAIR(crate::engine::FLOOR_COLOR),
        };
    }
}
*/

pub fn connect_room_centers(center1: &Position, center2: &Position, map: &mut Vec<Vec<TileType>>) {
    let mut temp = Position {
        x: center1.x,
        y: center1.y,
    };
    loop {
        if ((temp.x - 1) - center2.x).abs() < (temp.x - center2.x).abs() {
            temp.x -= 1
        } else if ((temp.x + 1) - center2.x).abs() < (temp.x - center2.x).abs() {
            temp.x += 1
        } else if ((temp.y + 1) - center2.y).abs() < (temp.y - center2.y).abs() {
            temp.y += 1
        } else if ((temp.y - 1) - center2.y).abs() < (temp.y - center2.y).abs() {
            temp.y -= 1
        } else {
            break;
        }
        map[temp.y as usize][temp.x as usize].ch = ' ';
        map[temp.y as usize][temp.x as usize].walkable = true;
    }
}
