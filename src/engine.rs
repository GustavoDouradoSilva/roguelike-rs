use crate::*;
use pancurses::*;

pub const PLAYER_COLOR: u32 = 1;
pub const WALL_COLOR: u32 = 2;
pub const FLOOR_COLOR: u32 = 3;
pub const NPC_COLOR: u32 = 4;

pub struct Game {
    pub map: Vec<Vec<TileType>>,
    //pub objects: Vec<Object>,
    pub window: pancurses::Window,
    pub log_win: pancurses::Window,
    pub log_file: std::fs::File,
}

pub fn curses_setup() {
    initscr();
    noecho();
    curs_set(0);
    start_color();
    init_pair(PLAYER_COLOR as i16, COLOR_GREEN, COLOR_BLACK);
    init_pair(WALL_COLOR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(FLOOR_COLOR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(NPC_COLOR as i16, COLOR_YELLOW, COLOR_BLACK);
}
//writes to the log file

enum LogColors 
{
    Green, Red, White, Yellow,
}

pub fn write_log(log: String, log_file: &mut std::fs::File) {
    use std::io::Write;

    write!(log_file, "{}\n", log).expect("Unable to write file");
}


pub fn write_log_win(log_win: &mut pancurses::Window) {
    let tmp_log = std::fs::read_to_string("log.txt").expect("Error in reading the file");

    let mut counter = 0;
    let mut log = String::new();
    for line in tmp_log.lines().rev() {
        if counter >= MAP_HEIGHT {
            break;
        }
        log = log + line + "\n";
        counter += 1;
    }

    log = log.lines().rev().collect::<Vec<&str>>().join("\n");

    log_win.clear();
    log_win.mvaddstr(0, 0, log);
    log_win.refresh();
}
pub fn game_loop(game: &mut Game, objects: &mut Vec<Object>) {
    write_log("Game loop started".to_string(), &mut game.log_file);
    draw::draw_everything(objects, game);

    write_log_win(&mut game.log_win);
    loop {
        match game.window.getch() {
            //'\u{1b}' = ESC
            Some(Input::Character('\u{1b}')) => {
                break;
            }
            Some(Input::Character(input)) => {
                objects[0].handle_input(input, game);
                draw::draw_everything(objects, game);
                write_log_win(&mut game.log_win);
            }
            Some(Input::KeyDC) => break,
            //Some(pancurses::Input::KeyUp) => { ch = 'w'; },
            Some(_input) => {
                //window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }
}

pub fn close_game() {
    pancurses::endwin();
}
