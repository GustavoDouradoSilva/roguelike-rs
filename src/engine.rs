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

pub fn game_loop(window: &mut Window, player: &mut Creature, map: &mut Vec<Vec<TileType>>) {
    draw::draw_everything(map, player, window);
    loop {
        match window.getch() {
            //'\u{1b}' = ESC
            Some(Input::Character('\u{1b}')) => {
                break;
            }
            Some(Input::Character(input)) => {
                player.handle_input(input, map);
                draw::draw_everything(map, player, window);
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
