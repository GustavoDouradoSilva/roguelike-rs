use crate::*;
use pancurses::*;

pub const PLAYER_COLOR: u32 = 1;
pub const WALL_COLOR: u32 = 2;
pub const FLOOR_COLOR: u32 = 3;
pub const NPC_COLOR: u32 = 4;

pub fn curses_setup() {
    initscr();
    noecho();
    curs_set(0);
    start_color();
    init_pair(PLAYER_COLOR as i16, COLOR_GREEN, COLOR_BLACK);
    init_pair(WALL_COLOR as i16, COLOR_WHITE, COLOR_WHITE);
    init_pair(FLOOR_COLOR as i16, COLOR_BLACK, COLOR_BLACK);
    init_pair(NPC_COLOR as i16, COLOR_YELLOW, COLOR_BLACK);
}
//writes to the log file
pub fn write_log(log: String) {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open("target/log.txt")
        .unwrap();

    write!(file, "{}\n", log).expect("Unable to write file");
}

//writes to the log_win
pub fn write_log_win(log_win: &mut pancurses::Window) {
    let tmp_log = std::fs::read_to_string("target/log.txt").expect("Error in reading the file");

    let mut counter = 0;
    let mut log = String::new();
    for line in tmp_log.lines().rev() {
        if counter > 5 {
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

pub fn game_loop(window: &mut Window, player: &mut Creature, map: &mut Vec<Vec<TileType>>) {
    write_log("Game loop started".to_string());
    draw::draw_everything(map, player, window);
    let mut log_win = pancurses::newwin(10 as i32, MAP_WIDTH as i32, MAP_HEIGHT as i32 + 1, 0);
    write_log_win(&mut log_win);

    loop {
        match window.getch() {
            //'\u{1b}' = ESC
            Some(Input::Character('\u{1b}')) => {
                break;
            }
            Some(Input::Character(input)) => {
                player.handle_input(input, map);
                draw::draw_everything(map, player, window);
                write_log_win(&mut log_win);
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
