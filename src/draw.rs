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

fn draw_entities(window: &mut pancurses::Window,objects: &Vec<Object>) {
    for object in objects {
        object.draw(window);
    }
}

pub fn draw_everything(
    objects: &Vec<Object>,
    game: &mut Game,
) {
    game.window.clear();
    draw_map(&game.map, &mut game.window);
    draw_entities(&mut game.window, objects);
}
