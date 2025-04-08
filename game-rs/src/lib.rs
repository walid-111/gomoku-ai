#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::use_self,
)]

#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_sign_loss,
	clippy::wildcard_imports,
	clippy::cargo,
	clippy::module_name_repetitions,
	clippy::inline_always,
	clippy::new_without_default,
	internal_features,
)]

#![feature(core_intrinsics)]
#![feature(adt_const_params)]

pub mod settings;
use settings::{BOARD_SIZE, BOARD_SIZE_I, CELL_COUNT};

pub mod location;
use location::{Location, PackedLocation, directions};

pub mod opening_table;

pub mod opening;
use opening::Opening;

pub mod ai_player;

pub mod debug;
pub use crate::debug::*;

pub mod lookup_build;
pub mod lookup;
use lookup::{use_lookup_table, ThreatMinMaxScore, LOOKUP_TABLE_USE_ORDERING};

#[cfg(test)]
mod test;

mod threading;


use std::collections::BTreeSet;

use std::intrinsics::{likely, unlikely};
use std::vec;

use itertools::Itertools;


#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Player {
	None    = 0,
	Player1 = 1,
	Player2 = 2,
	OutOfBounds = 3,
}

impl Player {
	#[inline]
	#[must_use]
	const fn from_u8_unsafe(value: u8) -> Self {
		unsafe { std::mem::transmute(value) }
	}
}

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MoveResult {
	Ok = 0,
	Invalid = 1,
	WonByAlignment = 3,
	WonByCapture = 4,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct PackedGameState {
	// u64   3 = lines
	// u128  6 = lines

	// 2 bit per cell
	board: [u8; usize::div_ceil(CELL_COUNT, 4)],
	/// Stones in the 19x19 grid played stones
	play_mask: [u32; BOARD_SIZE],
	free_three_mask: [u32; BOARD_SIZE],
	moves_scores: [i32; CELL_COUNT],

	#[wasm_bindgen(readonly, js_name = currentPlayer)]
	pub current_player: Player,
	pub opposite_player: Player,
	#[wasm_bindgen]
	pub winner: Player,

	// 4 bits for each player
	players_captures_count: u8,
	#[wasm_bindgen(readonly, js_name = alignmentsScore)]
	pub alignments_score: i32,
	alignments_score_minmax: ThreatMinMaxScore,

	forced_moves: BTreeSet<PackedLocation>,

	// TODO Dont copy / partial undo system?? :
	//      Vec of play_mask copy&modify forwards when makemove
	turn: u32,
	pub opening: Opening,
}

macro_rules! sliding_windows {
	(
		length=$length:literal,
		$game_state:ident,
		$location:ident,
		@init $init:expr,
		@init_add $init_add:expr,
		@slide $slide:expr
	) => {
		for &dir in directions::ALL_AXIS {
			// (ilocation + idir * window_start_i) && (ilocation + idir * window_end_i)
			// need to be in the board
			let mut window_start_i = -$length;
			let mut window_end_i = $length;

			if likely(dir.x != 0) {
				if unlikely(0 < dir.x) {
					window_start_i = window_start_i.max(-$location.x);
					window_end_i = window_end_i.min((BOARD_SIZE_I - 1) -$location.x);
				} else {
					window_start_i = window_start_i.max(-((BOARD_SIZE_I - 1) -$location.x));
					window_end_i = window_end_i.min($location.x);
				}
			}
			if likely(dir.y != 0) {
				window_start_i = window_start_i.max(-$location.y);
				window_end_i = window_end_i.min((BOARD_SIZE_I - 1) -$location.y);
			}

			let window_end_init_i = window_start_i + $length;

			// If location on border = not possible
			if window_end_i <= window_end_init_i {
				continue ;
			}

			let idir = dir.index as isize;

			let mut value = $init;

			let mut location_index = $location.index as isize + idir * window_start_i;
			let location_diff = idir * -($length + 1);

			// Initialize the sliding window with one extra cell
			for _ in window_start_i..=window_end_init_i {
				$init_add(&mut value, $game_state.get_stone_at(location_index as usize));
				location_index += idir;
			}

			// Move the sliding window
			for _ in window_end_init_i..window_end_i {
				let old_stone = $game_state.get_stone_at((location_index + location_diff) as usize);
				let new_stone = $game_state.get_stone_at(location_index as usize);
				location_index += idir;

				if !$slide(&mut value, old_stone, new_stone) {
					break ;
				}
			}

		}
	};
}

#[wasm_bindgen]
#[allow(clippy::missing_const_for_fn)]
impl PackedGameState {
	#[must_use]
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			board: [0; usize::div_ceil(CELL_COUNT, 4)],
			play_mask: std::array::from_fn(|y| {
				if y as isize == BOARD_SIZE_I / 2 {
					Self::BOARD_NEIGHBOR_MASK_1 << (BOARD_SIZE_I / 2)
				} else {
					0
				}
			}),
			free_three_mask: [0; BOARD_SIZE],
			moves_scores: [0; CELL_COUNT],

			current_player: Player::Player1,
			opposite_player: Player::Player2,
			winner: Player::None,

			players_captures_count: 0,
			alignments_score: 0,
			alignments_score_minmax: ThreatMinMaxScore { min: 0, max: 0 },

			forced_moves: BTreeSet::new(),

			opening: Opening::None,
			turn: 0,
		}
	}

	#[must_use]
	#[wasm_bindgen(getter)]
	pub fn board(&self) -> Vec<Player> {
		(0..CELL_COUNT).map(|i| self.get_stone_at(i)).collect()
	}

	#[inline]
	#[must_use]
	#[wasm_bindgen(js_name = getPlayerCapturesCount)]
	pub fn get_player_captures_count(&self, player: Player) -> u8 {
		let shift = (player as usize - 1) * 4;

		(self.players_captures_count >> shift) & 0b1111
	}

	#[must_use]
	#[wasm_bindgen(js_name = makeMoves)]
	pub fn _make_moves(&mut self, moves: &[usize]) -> MoveResult {
		for &index in moves {
			match self.make_move(&Location::from_index(index)) {
				MoveResult::Ok => (),
				result => return result,
			}
		}
		MoveResult::Ok
	}

	#[must_use]
	#[wasm_bindgen(js_name = allowedMovesBitMask)]
	pub fn allowed_moves_bit_mask(&self) -> Vec<u32> {
		(0..BOARD_SIZE_I)
			.map(|y| {
				(0..BOARD_SIZE_I)
					.map(|x| Location::from(x, y))
					.filter(|location| self.is_valid_move(location))
					.fold(0, |acc, location| acc | (1 << location.x))
			})
			.collect()
	}

	#[must_use]
	#[wasm_bindgen(js_name = winningAlignmentBitMask)]
	pub fn winning_alignment_bit_mask(&self, last_move: usize) -> Vec<u32> {
		let alignments = self.find_alignments::<5>(&Location::from_index(last_move))
			.flatten()
			.map(|location| location.index as PackedLocation)
			.collect::<BTreeSet<_>>();

		(0..BOARD_SIZE_I)
			.map(|y| {
				let mut res = 0;
				for x in 0..BOARD_SIZE_I {
					let i = Location::from(x, y).index;
					if alignments.contains(&(i as PackedLocation)) {
						res |= 1 << x;
					}
				}
				res
			})
			.collect()
	}
}

impl Default for PackedGameState {
	fn default() -> Self { Self::new() }
}

impl PackedGameState {
	#[allow(clippy::unusual_byte_groupings)]
	// const BOARD_NEIGHBOR_MASK_1: u32 = 0b0000_1_0000;
	const BOARD_NEIGHBOR_MASK_1: u32 = 0b00_1_00;
	#[allow(clippy::unusual_byte_groupings)]
	// const BOARD_NEIGHBOR_MASK_5: u32 = 0b1111_1_1111;
	const BOARD_NEIGHBOR_MASK_5: u32 = 0b11_1_11;

	#[inline]
	#[must_use]
	fn get_stone_at(&self, index: usize) -> Player {
		let byte = unsafe { *self.board.get_unchecked(index >> 2) };
		let shift = (index & 0b11) * 2;
		Player::from_u8_unsafe((byte >> shift) & 0b11)
	}

	fn clear_stone_at(&mut self, index: usize) {
		let byte = unsafe { self.board.get_unchecked_mut(index >> 2) };
		let shift = (index & 0b11) * 2;

		*byte &= !((0b11) << shift);
	}

	#[inline]
	fn set_stone_at(&mut self, index: usize, new_player: Player) {
		let byte = unsafe { self.board.get_unchecked_mut(index >> 2) };
		let shift = (index & 0b11) * 2;

		*byte ^= (new_player as u8) << shift;
	}

	#[inline]
	#[must_use]
	fn get_stone_at_safe(&self, location: &Location) -> Player {
		if unlikely(!location.is_in_bound()) {
			return Player::OutOfBounds;
		}

		self.get_stone_at(location.index)
	}

	#[inline]
	#[must_use]
	const fn is_in_play_mask(&self, x: isize, y: isize) -> bool {
		(self.play_mask[y as usize] & (Self::BOARD_NEIGHBOR_MASK_1 << x)) != 0
	}

	#[inline]
	fn set_player_captures_count(&mut self, player: Player, captures: u8) {
		let shift = (player as usize - 1) * 4;

		self.players_captures_count = (
			self.players_captures_count
			& !(0b1111 << shift)
		) | (captures << shift);
	}

	#[inline]
	#[must_use]
	fn introduced_free_three(&self, location: &Location) -> usize {
		// Check if the move would introduce a double-three
		// (before computing any captures)
		let mut free_three: usize = 0;

		sliding_windows!(length=4,
			self,
			location,
			@init [0usize; 3],
			@init_add |window_counters: &mut [usize; 3], stone| {
				window_counters[stone as usize] += 1;
			},
			@slide |window_counters: &mut [usize; 3], old_stone, new_stone| {
				window_counters[old_stone as usize] -= 1;

				// Check that it can become a 4 stones alignment
				// knowing that one of the cell
				// will be filled by one of our stone this turn
				if window_counters[self.current_player as usize] == 2
				&& window_counters[Player::None as usize] == 2
				// check that the border are clear
				&& old_stone == Player::None
				&& new_stone == Player::None {
					free_three += 1;
					// Two free-three on the same axis is not possible
					return false;
				}

				window_counters[new_stone as usize] += 1;

				true
			}
		);

		free_three
	}

	#[inline]
	#[must_use]
	fn is_valid_non_forced_move(&self, location: &Location, hint: bool) -> bool {
		self.get_stone_at(location.index) == Player::None
			&& self.opening.is_valid_opening_move(location, self.turn)
			&& ((hint && (self.free_three_mask[location.y as usize] >> location.x) & 1 == 0)
				|| self.introduced_free_three(location) < 2)
	}

	#[must_use]
	fn is_valid_move(&self, location: &Location) -> bool {
		if unlikely(!self.forced_moves.is_empty() && !self.forced_moves.contains(&(location.index as PackedLocation))) {
			return false;
		}

		self.is_valid_non_forced_move(location, false)
	}

	#[inline]
	#[must_use]
	fn capture(&mut self, location: &Location)-> u8 {
		let mut captures = self.get_player_captures_count(self.current_player);

		for &dir in directions::ALL {
			let close_stone = location.add(&dir.mul(3));

			if !close_stone.is_in_bound() {
				continue;
			}

			let stone1 = location.index.wrapping_add(dir.index);
			let stone2 = stone1.wrapping_add(dir.index);

			if unlikely(self.get_stone_at(stone1) == self.opposite_player
			&& self.get_stone_at(stone2) == self.opposite_player
			&& self.get_stone_at(close_stone.index) == self.current_player) {
				// Capture
				self.clear_stone_at(stone1);
				self.clear_stone_at(stone2);

				captures += 1;
			}
		}

		self.set_player_captures_count(self.current_player, captures);

		captures
	}

	#[inline]
	fn find_alignments<'a, const LENGTH: isize>(&'a self, location: &'a Location) -> impl Iterator<Item = Vec<Location>> + 'a {
		directions::ALL_AXIS.iter()
			.filter_map(move |&dir| {
				let mut aligned_stones = vec![];

				for i in 1..LENGTH {
					let new_location = location.add(&dir.mul(-i));
					if unlikely(self.get_stone_at_safe(&new_location) != self.current_player) {
						break;
					}
					aligned_stones.push(new_location);
				}

				aligned_stones = aligned_stones.into_iter().rev().collect::<Vec<_>>();

				aligned_stones.push(location.clone());

				for i in 1..LENGTH {
					let new_location = location.add(&dir.mul(i));
					if unlikely(self.get_stone_at_safe(&new_location) != self.current_player) {
						break;
					}
					aligned_stones.push(new_location);
				}

				if aligned_stones.len() >= LENGTH as usize {
					Some(aligned_stones)
				} else {
					None
				}
			})
	}

	#[inline]
	#[must_use]
	fn ways_of_capturing(&self, location: &Location) -> BTreeSet<PackedLocation> {
		let mut capture_moves = BTreeSet::new();

		for &dir in directions::ALL {
			let ally_location = location.add(dir);
			let threat_1_location = location.sub(dir);
			let threat_2_location = &ally_location.add(dir);

			if !threat_1_location.is_in_bound() || !threat_2_location.is_in_bound() {
				continue;
			}

			let threat_1 = self.get_stone_at(threat_1_location.index);
			let threat_2 = self.get_stone_at(threat_2_location.index);

			if self.get_stone_at(ally_location.index) != self.current_player {
				continue ;
			}

			if threat_1 == self.opposite_player {
				if threat_2 == Player::None {
					capture_moves.insert(threat_2_location.index as PackedLocation);
				}
			} else if threat_1 == Player::None
					&& threat_2 == self.opposite_player {
				capture_moves.insert(threat_1_location.index as PackedLocation);
			}
		}

		capture_moves
	}

	#[inline]
	#[must_use]
	fn ways_of_breaking_alignment(&self, aligned_stones: &[Location]) -> BTreeSet<PackedLocation> {
		let alignment_len = aligned_stones.len();

		// Ex: if alignment of 6 stone we can capture only from index 1 to last-1
		// to split in piece less than 5
		let capturable_start = alignment_len - 5;
		let capturable_end = alignment_len - capturable_start;

		let mut capture_moves = BTreeSet::new();

		for location in &aligned_stones[capturable_start..capturable_end] {
			capture_moves.append(&mut self.ways_of_capturing(location));
		}

		capture_moves
	}

	#[inline]
	#[must_use]
	fn check_alignments(&self, location: &Location) -> Option<BTreeSet<PackedLocation>> {
		self.find_alignments::<5>(location)
			.map(|alignment|
				self.ways_of_breaking_alignment(&alignment)
			)
			.reduce(|moves_intersection, moves|
				moves_intersection
					.intersection(&moves)
					.copied()
					.collect()
			)
	}

	pub fn make_move_unchecked(&mut self, location: &Location) -> MoveResult {
		self.turn += 1;

		// Update play mask
		for y in (location.y - 2).max(0)..=(location.y + 2).min(BOARD_SIZE_I - 1) {
			self.play_mask[y as usize] |= Self::BOARD_NEIGHBOR_MASK_5 << location.x;
		}

		self.set_stone_at(location.index, self.current_player);

		// Check captures
		if unlikely(self.capture(location) >= 5) {
			self.winner = self.current_player;
			return MoveResult::WonByCapture;
		}

		// Check alignments
		if let Some(forced_moves) = self.check_alignments(location) {
			if unlikely(forced_moves.is_empty()) {
				// No way to break the alignment
				self.winner = self.current_player;
				return MoveResult::WonByAlignment;
			}
			self.forced_moves = forced_moves;
		} else {
			// No alignment
			self.forced_moves.clear();
		}

		// Pass to next player
		std::mem::swap(&mut self.current_player, &mut self.opposite_player);

		MoveResult::Ok
	}

	#[must_use]
	pub fn make_move(&mut self, location: &Location) -> MoveResult {
		if self.winner != Player::None {
			return MoveResult::Invalid;
		}

		if !self.is_valid_move(location) {
			return MoveResult::Invalid;
		}

		self.make_move_unchecked(location)
	}

	#[must_use]
	pub fn sorted_moves_list(&mut self, limit: usize) -> Vec<Location> {
		const MIN_RATIO: i32 = 101;

		use_lookup_table::<LOOKUP_TABLE_USE_ORDERING>(self);

		if !self.forced_moves.is_empty() {
			let mut moves = self.forced_moves.iter()
				.map(|&index| Location::from_index(index as usize))
				.collect::<Vec<_>>();
			moves.sort_by_key(|move_location| -self.moves_scores[move_location.index]);
			return moves;
		}

		let mut moves = Vec::with_capacity(CELL_COUNT / 3);
		let mut best_score = i32::MIN;

		for y in 0..BOARD_SIZE_I {
			if self.play_mask[y as usize] == 0 {
				continue ;
			}

			for x in 0..BOARD_SIZE_I {
				if !self.is_in_play_mask(x, y) {
					continue ;
				}

				let location = Location::from(x, y);
				let score = self.moves_scores[location.index];
				best_score = best_score.max(score);

				// early skip if bad score
				if !((best_score / MIN_RATIO) < self.moves_scores[location.index]) {
					continue ;
				}

				if self.is_valid_non_forced_move(&location, true) {
					moves.push(location);
				}
			}
		}

		moves.into_iter()
			.filter(|m| (best_score / MIN_RATIO) < self.moves_scores[m.index])
			.sorted_unstable_by_key(|m| -self.moves_scores[m.index])
			.take(limit)
			.collect()
	}
}

#[wasm_bindgen(start)]
pub fn start() {
	set_panic_hook();
}
