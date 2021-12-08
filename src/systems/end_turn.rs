use crate::prelude::*;

// setup system to transition to next turn
#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn_state = new_state;
}