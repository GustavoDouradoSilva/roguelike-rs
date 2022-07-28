use crate::*;
fn draw_map(map: &Vec<Vec<TileType>>, window: &mut pancurses::Window) {
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            window.attron(map[y][x].color);
            window.mvaddch(y as i32, x as i32, map[y][x].ch);
            window.attroff(map[y][x].color);
        }
    }
}

fn draw_entities(objects: &Vec<Object>, window: &mut pancurses::Window) {
    for object in objects {
        object.draw(window);
    }
}

pub fn draw_everything(
    map: &Vec<Vec<TileType>>,
    objects: &Vec<Object>,
    window: &mut pancurses::Window,
) {
    window.clear();
    draw_map(map, window);
    draw_entities(objects, window);
}
