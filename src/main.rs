mod creature;
mod draw;
mod engine;
mod entity;
mod map;
mod room;

//use std::*;
use creature::*;
use engine::*;
use entity::*;
use map::*;

fn main() {
    std::fs::File::create("log.txt").expect("Unable to create file");

    let mut player = Creature {
        name: "player".to_string(),
        pos: Position { x: 0, y: 0 },
        draw: Draw {
            ch: '@',
            color: PLAYER_COLOR,
        },
    };

    /*
    let npc = Creature {
        name: "npc".to_string(),
        pos: Position { x: 0, y: 0 },
        draw: Draw {
            ch: '@',
            color: PLAYER_COLOR,
        },
    };
    */

    let tile = TileType {
        ch: '#',
        walkable: false,
        color: pancurses::COLOR_PAIR(crate::engine::PLAYER_COLOR),
    };

    /*
    let wall = Tile {
        name: "wall".to_string(),
        draw: Draw {
            ch: '#',
            color: WALL_COLOR,
        },
        walkable: false,
    };

    let floor = Tile {
        name: "floor".to_string(),
        draw: Draw {
            ch: '.',
            color: FLOOR_COLOR,
        },
        walkable: true,
    };
    */

    curses_setup();
    let mut map = vec![vec![tile; MAP_WIDTH]; MAP_HEIGHT];

    let mut window = pancurses::newwin(MAP_HEIGHT as i32, MAP_WIDTH as i32, 0, 0);
    window.keypad(true);
    player.pos = setup_map(&mut map);
    game_loop(&mut window, &mut player, &mut map);
    close_game();
}
