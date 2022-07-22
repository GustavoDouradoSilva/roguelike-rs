use crate::*;
fn draw_map(map: &mut Vec<Vec<Tile>>, window: &mut pancurses::Window) {
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            window.mvaddch(y as i32, x as i32, map[y][x].ch);
        }
    }
}

fn draw_entity(entity: &mut Entity, window: &mut pancurses::Window) {
    window.mvaddch(entity.pos.y as i32, entity.pos.x as i32, entity.ch);
}

pub fn draw_everything(
    map: &mut Vec<Vec<Tile>>,
    player: &mut Entity,
    window: &mut pancurses::Window,
) {
    window.clear();
    draw_map(map, window);
    draw_entity(player, window);
}