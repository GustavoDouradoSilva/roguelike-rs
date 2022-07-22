use pancurses::*;

use crate::*;


pub fn curses_setup() {
    initscr();
    noecho();
    curs_set(0);
    /* 
    if has_colors() {
        start_color();
        init_pair(VISIBLE_COLOR, COLOR_WHITE, COLOR_BLACK);
        init_pair(SEEN_COLOR, COLOR_BLUE, COLOR_BLACK);
    }
    */
}

pub fn game_loop(window: &mut Window, player: &mut Entity, map: &mut Vec<Vec<TileType>>) {
    draw::draw_everything(map, player, window);
    loop {
        match window.getch() {
            Some(Input::Character('\u{1b}')) => {
                println!("quitting");
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
