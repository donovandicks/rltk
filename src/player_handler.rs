use crate::{
    components::{Player, Position},
    map::{xy_idx, TileType},
};
use rltk::{Rltk, VirtualKeyCode};
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

fn try_move_player(d_x: i32, d_y: i32, ecs: &mut World, width: i32, height: i32) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_p, pos) in (&mut players, &mut positions).join() {
        let dest_idx = xy_idx(pos.x + d_x, pos.y + d_y);
        if map[dest_idx] != TileType::Wall {
            pos.x = min(width - 1, max(0, pos.x + d_x));
            pos.y = min(height - 1, max(0, pos.y + d_y));
        }
    }
}

pub fn get_player_input(ecs: &mut World, ctx: &mut Rltk, width: i32, height: i32) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::A | VirtualKeyCode::Left | VirtualKeyCode::H => {
                try_move_player(-1, 0, ecs, width, height)
            }
            VirtualKeyCode::D | VirtualKeyCode::Right | VirtualKeyCode::L => {
                try_move_player(1, 0, ecs, width, height)
            }
            VirtualKeyCode::W | VirtualKeyCode::Up | VirtualKeyCode::K => {
                try_move_player(0, -1, ecs, width, height)
            }
            VirtualKeyCode::S | VirtualKeyCode::Down | VirtualKeyCode::J => {
                try_move_player(0, 1, ecs, width, height)
            }
            _ => {}
        },
    }
}
