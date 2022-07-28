mod draw;
mod engine;
mod map;
mod object;
mod room;

//use std::*;
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
    let mut game = Game {
        map: vec![vec![tile; MAP_WIDTH]; MAP_HEIGHT],
        objects: Vec::new(),
        window: pancurses::initscr(),
        log_win: pancurses::newwin(MAP_HEIGHT as i32, 45, 0, MAP_WIDTH as i32 + 1),
    };


    //let mut objects: Vec<Object> = Vec::new();
    game.objects.push(Object::new(
        "player".to_string(),
        Position { x: 0, y: 0 },
        Draw {
            ch: '@',
            color: pancurses::COLOR_PAIR(PLAYER_COLOR),
        },
    ));
    game.objects.push(Object::new(
        "npc".to_string(),
        Position { x: 0, y: 0 },
        Draw {
            ch: '@',
            color: pancurses::COLOR_PAIR(NPC_COLOR),
        },
    ));

    //let mut objects = vec![player, npc];
    let player = &mut game.objects[0];
    //let npc = &mut objects[1];

    
    
    //let mut map = vec![vec![tile; MAP_WIDTH]; MAP_HEIGHT];

    let mut window = pancurses::newwin(MAP_HEIGHT as i32, MAP_WIDTH as i32, 0, 0);
    window.keypad(true);
    player.move_to(&setup_map(&mut game.map), &game.map);
    game_loop(&mut window, &mut game.objects, &mut game.map);
    close_game();
}
