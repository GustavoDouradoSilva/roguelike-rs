use crate::*;

pub fn create_room(x: i32, y: i32, width: i32, height: i32) -> Room {
    let new_room = Room {
        height: height,
        width: width,
        pos: Position { x: x, y: y },
        center: Position {
            x: x + width / 2,
            y: y + height / 2,
        },
    };
    new_room
}

pub fn add_room_to_map(room: &mut Room, map: &mut Vec<Vec<Tile>>) {
    //let mut new_map = map;
    for y in room.pos.y..room.pos.y + room.height {
        for x in room.pos.x..room.pos.x + room.width {
            map[y as usize][x as usize] = Tile {
                ch: '.',
                walkable: true,
                
            };
        }
    }
}

pub fn connect_room_centers(center1: &Position, center2: &Position, map: &mut Vec<Vec<Tile>>) {
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
        map[temp.y as usize][temp.x as usize].ch = '.';
        map[temp.y as usize][temp.x as usize].walkable = true;
    }
}