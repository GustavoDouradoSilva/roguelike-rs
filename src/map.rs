use crate::{engine::write_log, engine::*, object::*, room::*};
//use crate::{Position, Tile, Room};
use rand::Rng;

pub const MAP_HEIGHT: usize = 25;
pub const MAP_WIDTH: usize = 100;

#[derive(Clone)]
pub struct TileType {
    pub ch: char,
    pub walkable: bool,
    pub color: u32,
}

pub fn setup_map(game: &mut Game) -> Position {
    let n_rooms: i32 = rand::thread_rng().gen_range(5..14);

    let mut rooms:Vec<Room> = Vec::new();

    for i in 0..(n_rooms) {
        rooms.push(Room::new_random());
        //Room::verify_room(rooms[i as usize].clone(), &mut rooms, game);
        add_room_to_map(&rooms[i as usize], &mut game.map);

        
        if i > 0 {
            connect_room_centers(
                &rooms[i as usize - 1].center(),
                &rooms[i as usize].center(),
                &mut game.map,
            )
        }
    }

    let start_pos = Position {
        x: rooms[0].center().x,
        y: rooms[0].center().y,
    };

    write_log("Map generated".to_string(), &mut game.log_file);
    start_pos
}
