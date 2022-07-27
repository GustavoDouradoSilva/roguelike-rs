use crate::*;
fn draw_map(map: &mut Vec<Vec<TileType>>, window: &mut pancurses::Window) {
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            //window.attron(map[y][x].color);
            window.mvaddch(y as i32, x as i32, map[y][x].ch);
            //window.attron(map[y][x].color);
        }
    }
}

fn draw_entities(creature: &mut Creature, window: &mut pancurses::Window) {
    creature.draw(window);
}

pub fn draw_everything(
    map: &mut Vec<Vec<TileType>>,
    creature: &mut Creature,
    window: &mut pancurses::Window,
) {
    window.clear();
    draw_map(map, window);
    draw_entities(creature, window);
}
