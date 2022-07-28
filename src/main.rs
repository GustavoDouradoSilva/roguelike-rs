mod draw;
mod engine;
mod map;
mod object;
mod room;

use engine::*;
use map::*;
use object::*;

fn main() {
    std::fs::File::create("log.txt").expect("Unable to create file");
    
    let tile = TileType {
        ch: '#',
        walkable: false,
        color: pancurses::COLOR_PAIR(WALL_COLOR),
    };

    curses_setup();
    let mut objects:Vec<Object> = Vec::new();
    let mut game = Game {
        map: vec![vec![tile; MAP_WIDTH]; MAP_HEIGHT],
        //objects: Vec::new(),
        window: pancurses::newwin(MAP_HEIGHT as i32, MAP_WIDTH as i32, 0, 0),
        log_win: pancurses::newwin(MAP_HEIGHT as i32, 45, 0, MAP_WIDTH as i32 + 1),
        log_file: std::fs::OpenOptions::new()
            .write(true)
            .append(true) // This is needed to append to file
            .open("log.txt")
            .unwrap(),
    };

    objects.push(Object::new(
        "player".to_string(),
        Position { x: 0, y: 0 },
        Draw {
            ch: '@',
            color: pancurses::COLOR_PAIR(PLAYER_COLOR),
        },
    ));
    objects.push(Object::new(
        "npc".to_string(),
        Position { x: 0, y: 0 },
        Draw {
            ch: '@',
            color: pancurses::COLOR_PAIR(NPC_COLOR),
        },
    ));
    let new_pos = setup_map(&mut game);
    game.window.keypad(true);
    let player = &mut objects[0];
    player.move_to(new_pos, &mut game);
    game_loop(&mut game, &mut objects);
    close_game();
}
