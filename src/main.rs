mod draw;
mod engine;
mod map;
mod player;
mod room;

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

//use std::*;
use engine::*;
use map::*;
use player::*;


fn main() {
    curses_setup();
    let tile = TileType { ch: '#', walkable: false};
    let mut map = vec![vec![tile;MAP_WIDTH]; MAP_HEIGHT];
    let mut player = Entity::create_player(setup_map(&mut map));
    let mut window = pancurses::newwin(MAP_HEIGHT as i32, MAP_WIDTH as i32, 0, 0);
    window.keypad(true);

    //let tiles = vec![vec![Tile{ch:'#', walkable: false}; MAP_WIDTH]; MAP_HEIGHT];

    game_loop(&mut window, &mut player, &mut map);

    close_game();
}