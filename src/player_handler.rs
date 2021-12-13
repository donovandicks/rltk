use crate::components::{Player, Position};
use rltk::{Rltk, VirtualKeyCode};
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

fn try_move_player(d_x: i32, d_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_p, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + d_x));
        pos.y = min(49, max(0, pos.y + d_y));
    }
}

pub fn get_player_input(ecs: &mut World, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::A | VirtualKeyCode::Left | VirtualKeyCode::H => {
                try_move_player(-1, 0, ecs)
            }
            VirtualKeyCode::D | VirtualKeyCode::Right | VirtualKeyCode::L => {
                try_move_player(1, 0, ecs)
            }
            VirtualKeyCode::W | VirtualKeyCode::Up | VirtualKeyCode::K => {
                try_move_player(0, -1, ecs)
            }
            VirtualKeyCode::S | VirtualKeyCode::Down | VirtualKeyCode::J => {
                try_move_player(0, 1, ecs)
            }
            _ => {}
        },
    }
}
