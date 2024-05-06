use crate::{CombatStats, WantsToMelee};

use super::{Map, Player, Position, State, Viewshed, RunState};
use rltk::{console, Point, Rltk};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        if pos.x + dx < 1 || pos.x + dx > map.width - 1 || pos.y + dy < 1 || pos.y + dy > map.height - 1 { return; }
        let destination_idx = map.xy_idx(pos.x + dx, pos.y + dy);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                return;
            }
        }
        
        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
            
            viewshed.dirty = true;

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput }
        Some(key) => match key {
            rltk::VirtualKeyCode::Left | rltk::VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Right | rltk::VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Up | rltk::VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::Down | rltk::VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),

            rltk::VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),
            rltk::VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),
            _ => { return RunState::AwaitingInput }
        },
    }
    RunState::PlayerTurn
}
