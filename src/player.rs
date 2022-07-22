use crate::*;

pub fn handle_input(input: char, player: &mut Entity, map: &mut Vec<Vec<Tile>>) {
    let mut new_pos = Position {
        x: player.pos.x,
        y: player.pos.y,
    };
    match input {
        'w' => new_pos.y -= 1,
        'a' => new_pos.x -= 1,
        's' => new_pos.y += 1,
        'd' => new_pos.x += 1,
        _ => return,
    }
    move_player(player, &mut new_pos, map)
}

pub fn move_player(player: &mut Entity, new_pos: &mut Position, map: &mut Vec<Vec<Tile>>) {
    if new_pos.x < 0
        || new_pos.x >= map::MAP_WIDTH as i32
        || new_pos.y < 0
        || new_pos.y >= map::MAP_HEIGHT as i32
    {
        return;
    }
    if !map[new_pos.y as usize][new_pos.x as usize].walkable {
        return;
    }
    player.pos.x = new_pos.x;
    player.pos.y = new_pos.y;
}