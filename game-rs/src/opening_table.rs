use crate::{location::PackedLocation, settings::CELL_COUNT, PackedGameState, Player};

#[cfg(feature = "opening_table")]
include!("./opening_table_generated.rs");

#[must_use]
pub fn find_stone(game_state: &PackedGameState, player: Player) -> PackedLocation {
	let mut i = 0;
	while i < CELL_COUNT {
		if game_state.get_stone_at(i) == player {
			return i as PackedLocation;
		}
		i += 1;
	}
	unreachable!();
}

#[cfg(feature = "opening_table")]
#[must_use]
pub fn get_opening_move(game_state: &PackedGameState) -> Option<PackedLocation> {
	match game_state.turn {
		0 => Some(OPENING_MOVE),
		1 => {
			let opp_stone = find_stone(game_state, Player::Player1);
			Some(OPENING_TABLE_2[opp_stone as usize])
		},
		2 => {
			if game_state.get_stone_at(OPENING_MOVE as usize) != Player::Player1 {
				return None;
			}
			let opp_stone = find_stone(game_state, Player::Player2);
			Some(OPENING_TABLE_3[opp_stone as usize])
		},
		_ => None,
	}
}

#[cfg(not(feature = "opening_table"))]
#[must_use]
pub fn get_opening_move(game_state: &PackedGameState) -> Option<PackedLocation> {
	None
}
