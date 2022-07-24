use crate::*;
fn draw_map(map: &mut Vec<Vec<TileType>>, window: &mut pancurses::Window) {
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            window.mvaddch(y as i32, x as i32, map[y][x].ch);
        }
    }
}

fn draw_entity(entity: &mut Entity, window: &mut pancurses::Window) {
    //window.attron(pancurses::COLOR_PAIR(1));
    //window.mvaddch(entity.pos.y as i32, entity.pos.x as i32, entity.ch);
    //window.attroff(pancurses::COLOR_PAIR(1));
    entity.draw(window);
}

pub fn draw_everything(
    map: &mut Vec<Vec<TileType>>,
    player: &mut Entity,
    window: &mut pancurses::Window,
) {
    window.clear();
    draw_map(map, window);
    draw_entity(player, window);
}
