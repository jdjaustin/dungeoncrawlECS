mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod fov;
use crate::prelude::*;

// old scheduler function
/*
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system()).flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move())
        .build()
}
 */

// scheduler function for awaiting input turn
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system()).flush()
        .add_system(fov::fov_system()).flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

// scheduler function for player turn
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system()).flush()
        .add_system(movement::movement_system()).flush()
        .add_system(fov::fov_system()).flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

// scheduler function for monster turn
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system()).flush()
        .add_system(chasing::chasing_system()).flush()
        .add_system(combat::combat_system()).flush()
        .add_system(movement::movement_system()).flush()
        .add_system(fov::fov_system()).flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}