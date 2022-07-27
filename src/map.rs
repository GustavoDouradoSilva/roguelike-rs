use crate::{entity::*, room::*};
//use crate::{Position, Tile, Room};
use rand::Rng;

pub const MAP_HEIGHT: usize = 25;
pub const MAP_WIDTH: usize = 100;

#[derive(Clone)]
pub struct TileType {
    pub ch: char,
    pub walkable: bool,
    pub color: u32,
    //pub visible: bool,
    //pub revealed: bool
}

pub fn setup_map(map: &mut Vec<Vec<TileType>>) -> Position {
    let (mut x, mut y, mut height, mut width): (usize, usize, i32, i32);

    let n_rooms: i32 = rand::thread_rng().gen_range(5..14);

    let rooms = &mut vec![
        Room {
            height: 1,
            width: 1,
            pos: Position { x: 1, y: 1 },
            center: Position { x: 1, y: 1 }
        };
        n_rooms as usize
    ];

    for i in 0..(n_rooms) {
        x = rand::thread_rng().gen_range(1..(MAP_WIDTH - 20));
        y = rand::thread_rng().gen_range(1..(MAP_HEIGHT - 10));
        height = rand::thread_rng().gen_range(3..9);
        width = rand::thread_rng().gen_range(5..19);
        rooms[i as usize] = create_room(x as i32, y as i32, width, height);
        add_room_to_map(&mut rooms[i as usize], map);

        if i > 0 {
            connect_room_centers(
                &rooms[i as usize - 1].center,
                &rooms[i as usize].center,
                map,
            )
        }
    }

    let start_pos = Position {
        x: rooms[0].center.x,
        y: rooms[0].center.y,
    };
    start_pos
}
