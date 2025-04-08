use crate::{settings::{BOARD_SIZE, BOARD_SIZE_I}, PackedGameState, Player};

pub use crate::lookup_build::*;

include!(concat!(env!("OUT_DIR"), "/lookup_table.rs"));

pub type LookupTableUse = u8;
pub trait LookupTableUseTrait {
	fn is_ordering(&self) -> bool;
	fn is_scoring(&self) -> bool;
}
impl LookupTableUseTrait for LookupTableUse {
	fn is_ordering(&self) -> bool {
		(self & LOOKUP_TABLE_USE_ORDERING) != 0
	}
	fn is_scoring(&self) -> bool {
		(self & LOOKUP_TABLE_USE_SCORING) != 0
	}
}
pub const LOOKUP_TABLE_USE_ORDERING: LookupTableUse = 0b01;
pub const LOOKUP_TABLE_USE_SCORING: LookupTableUse = 0b10;

struct LookupTableIterData {
	pub stones: usize,
	pub full_free_three_mask: BitMaskLarge,
	pub full_score: [ThreatMinMaxScore; BOARD_SIZE + STONES_COUNT],
	pub index: usize,
}

impl LookupTableIterData {
	fn new() -> Self {
		Self {
			stones: STONES_MASK,
			full_free_three_mask: 0,
			full_score: core::array::from_fn(|_| {
				ThreatMinMaxScore { min: 0, max: 0 }
			}),
			index: BOARD_SIZE,
		}
	}

	fn add_stone<const USE: LookupTableUse>(&mut self, stone: Player) {
		self.stones <<= 2;
		self.stones |= stone as usize;
		let lookup_value = &THREAT_LOOKUP_TABLE[self.stones & STONES_MASK];
		if USE.is_ordering() {
			self.full_free_three_mask <<= 1;
			self.full_free_three_mask |= BitMaskLarge::from(lookup_value.free_three_mask);
		}
		self.index -= 1;
		for i in 0..STONES_COUNT {
			// get max score for all directions
			let score = &mut self.full_score[self.index + (STONES_COUNT - 1 - i)];
			let lookup_score = &lookup_value.threat_score[i];
			// score.min = score.min.min(lookup_score.min);
			// score.max = score.max.max(lookup_score.max);
			score.min += lookup_score.min;
			score.max += lookup_score.max;
		}
	}
}

macro_rules! update_score {
	($USE:expr, $game_state:expr, $out_move_score:expr, $full_score:expr) => {
		let full_score = &$full_score;
		if $USE.is_scoring() {
			$game_state.alignments_score += full_score.min + full_score.max;
			$game_state.alignments_score_minmax.min = $game_state.alignments_score_minmax.min.min(full_score.min);
			$game_state.alignments_score_minmax.max = $game_state.alignments_score_minmax.max.max(full_score.max);
		}
		if $USE.is_ordering() {
			$out_move_score += (-full_score.min).max(full_score.max);
		}
	};
}

const STARTING_POINTS_UP_RIGHT: [(isize, isize); 31] = [
	/*(0, 0), (0, 1), (0, 2),*/ (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
	(0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (0, 16), (0, 17), (0, 18),
	(1, 18), (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18), (8, 18), (9, 18),
	(10, 18), (11, 18), (12, 18), (13, 18), (14, 18), (15, 18), /*(16, 18), (17, 18), (18, 18)*/
];

const STARTING_POINTS_DOWN_RIGHT: [(isize, isize); 31] = [
	/*(0, 18), (0, 17), (0, 16),*/ (0, 15), (0, 14), (0, 13), (0, 12), (0, 11), (0, 10), (0, 9),
	(0, 8), (0, 7), (0, 6), (0, 5), (0, 4), (0, 3), (0, 2), (0, 1), (0, 0),
	(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0),
	(10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), /*(16, 0), (17, 0), (18, 0)*/
];


#[allow(clippy::items_after_statements)]
pub fn use_lookup_table<const USE: LookupTableUse>(game_state: &mut PackedGameState) {
	if USE.is_ordering() {
		game_state.moves_scores.fill(0);
		game_state.free_three_mask.fill(0);
	}
	if USE.is_scoring() {
		game_state.alignments_score = 0;
		game_state.alignments_score_minmax = ThreatMinMaxScore { min: 0, max: 0 };
	}

	// horizontal
	for y in 0..BOARD_SIZE {
		let mut iter_data = LookupTableIterData::new();
		for x in (0..BOARD_SIZE).rev() {
			iter_data.add_stone::<USE>(game_state.get_stone_at(y * BOARD_SIZE + x));
		}
		if USE.is_ordering() {
			game_state.free_three_mask[y] |= iter_data.full_free_three_mask;
		}
		for x in 0..BOARD_SIZE {
			update_score!(USE, game_state,
				game_state.moves_scores[y * BOARD_SIZE + x],
				iter_data.full_score[x]
			);
		}
	}

	// vertical
	for x in 0..BOARD_SIZE {
		let mut iter_data = LookupTableIterData::new();
		for y in (0..BOARD_SIZE).rev() {
			iter_data.add_stone::<USE>(game_state.get_stone_at(y * BOARD_SIZE + x));
		}
		for y in 0..BOARD_SIZE {
			if USE.is_ordering() {
				game_state.free_three_mask[y] |= ((iter_data.full_free_three_mask >> y) & 1) << x;
			}
			update_score!(USE, game_state,
				game_state.moves_scores[y * BOARD_SIZE + x],
				iter_data.full_score[y]
			);
		}
	}

	// diagonal
	// the first three diags are useless bcz can't align/capture

	for s in STARTING_POINTS_UP_RIGHT {
		let mut iter_data = LookupTableIterData::new();
		let mut x = s.0;
		let mut y = s.1;
		while (x < BOARD_SIZE_I) && (0 <= y) {
			iter_data.add_stone::<USE>(game_state.get_stone_at(y as usize * BOARD_SIZE + x as usize));
			x += 1;
			y -= 1;
		}
		while x != s.0 {
			x -= 1;
			y += 1;
			if USE.is_ordering() {
				game_state.free_three_mask[y as usize] |= (iter_data.full_free_three_mask & 1) << (x as usize);
				iter_data.full_free_three_mask >>= 1;
			}
			update_score!(USE, game_state,
				game_state.moves_scores[y as usize * BOARD_SIZE + x as usize],
				iter_data.full_score[iter_data.index]
			);
			iter_data.index += 1;
		}
	}

	for s in STARTING_POINTS_DOWN_RIGHT {
		let mut iter_data = LookupTableIterData::new();
		let mut x = s.0;
		let mut y = s.1;
		while (x < BOARD_SIZE_I) && (y < BOARD_SIZE_I) {
			iter_data.add_stone::<USE>(game_state.get_stone_at(y as usize * BOARD_SIZE + x as usize));
			x += 1;
			y += 1;
		}
		while x != s.0 {
			x -= 1;
			y -= 1;
			if USE.is_ordering() {
				game_state.free_three_mask[y as usize] |= (iter_data.full_free_three_mask & 1) << (x as usize);
				iter_data.full_free_three_mask >>= 1;
			}
			update_score!(USE, game_state,
				game_state.moves_scores[y as usize * BOARD_SIZE + x as usize],
				iter_data.full_score[iter_data.index]
			);
			iter_data.index += 1;
		}
	}
}
