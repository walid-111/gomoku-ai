pub type BitMask = u8;
pub type BitMaskLarge = u32;

pub type StonesMask = usize;

#[derive(Debug, Clone)]
pub struct ThreatMinMaxScore {
	pub min: i32,
	pub max: i32,
}

pub const STONES_COUNT: usize = 6;
pub const STONES_PERMU_COUNT: usize = 1 << (2 * STONES_COUNT);
pub const STONES_MASK: StonesMask = STONES_PERMU_COUNT - 1;

fn invert_stones_mask(stones_mask: StonesMask, size: usize) -> StonesMask {
	let mut new_mask = 0;
	let mut i = 0;
	while i < (size * 2) {
		new_mask |= match (stones_mask >> i) & 0b11 {
			0b00 => 0b00,
			0b01 => 0b10,
			0b10 => 0b01,
			0b11 => 0b11,
			_ => unreachable!(),
		} << i;
		i += 2;
	}
	new_mask
}

const fn reverse_stones_mask(stones_mask: StonesMask, size: usize) -> StonesMask {
	let mut new_mask = 0;
	let mut i = 0;
	while i < size {
		new_mask |= ((stones_mask >> (i * 2)) & 0b11) << ((size - i - 1) * 2);
		i += 1;
	}
	new_mask
}

const fn reverse_threat_mask(threat_mask: BitMaskLarge, size: usize) -> BitMaskLarge {
	let mut new_mask = 0;
	let mut i = 0;
	while i < size {
		new_mask |= ((threat_mask >> i) & 0b1) << (size - i - 1);
		i += 1;
	}
	new_mask
}

#[derive(Debug)]
pub struct LookupTableValue {
	pub free_three_mask: BitMask,
	pub threat_score: [ThreatMinMaxScore; STONES_COUNT],
}

pub fn generate_lookup_table() -> [LookupTableValue; STONES_PERMU_COUNT] {
	struct ThreatLookupTableGenMask {
		stones_expected: StonesMask,
		free_three_mask: BitMaskLarge,
		mask_size: usize,
		compare_size: usize,
		threat_score: Vec<i32>,
	}

	#[allow(clippy::unusual_byte_groupings)]
	let masks = [
		// 4 align
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_01_01_01_00,
			free_three_mask:      0,
			mask_size: 6,
			compare_size: 5,
			threat_score: vec![1_000_000, 0, 0, 0, 0, 1_000_000],
		},

		// 4 align with hole
		ThreatLookupTableGenMask {
			stones_expected: 0b_01_01_01_00_01,
			free_three_mask:      0,
			mask_size: 5,
			compare_size: 5,
			threat_score: vec![0, 0, 0, 1_000_000, 0],
		},
		ThreatLookupTableGenMask {
			stones_expected: 0b_01_01_00_01_01,
			free_three_mask:      0,
			mask_size: 5,
			compare_size: 5,
			threat_score: vec![0, 0, 1_000_000, 0, 0],
		},

		// 3 align (dangerous)
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_01_01_00,
			free_three_mask:      0,
			mask_size: 5,
			compare_size: 5,
			threat_score: vec![100_000, 0, 0, 0, 100_000],
		},

		// 3 align
		ThreatLookupTableGenMask {
			stones_expected: 0b_01_01_01_00_00,
			free_three_mask: 0,
			mask_size: 5,
			compare_size: 5,
			threat_score: vec![0, 0, 0, 2500, 2500],
		},

		// 3 align with hole (dangerous)
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_01_00_01_00,
			free_three_mask:      0,
			mask_size: 6,
			compare_size: 6,
			threat_score: vec![50_000, 0, 0, 100_000, 0, 50_000],
		},

		// 3 align with hole
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_01_00_01_00,
			free_three_mask:      0,
			mask_size: 6,
			compare_size: 5,
			threat_score: vec![2500, 0, 0, 2500, 0, 2500],
		},

		// 3 align with double hole
		ThreatLookupTableGenMask {
			stones_expected: 0b_01_01_00_00_01,
			free_three_mask:      0,
			mask_size: 5,
			compare_size: 5,
			threat_score: vec![0, 0, 0, 2500, 2500, 0],
		},

		// capture
		ThreatLookupTableGenMask {
			stones_expected: 0b_01_10_10_00,
			free_three_mask:      0,
			mask_size: 4,
			compare_size: 4,
			threat_score: vec![0, 0, 0, 100_000],
		},

		// free three
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_01_00_00_00,
			free_three_mask: 0b__0__0__0__1__1__0,
			mask_size: 6,
			compare_size: 6,
			threat_score: vec![0, 0, 0, 2500, 2500, 0],
		},

		ThreatLookupTableGenMask {
			stones_expected: 0b_00_00_01_01_00_00,
			free_three_mask: 0b__0__1__0__0__1__0,
			mask_size: 6,
			compare_size: 6,
			threat_score: vec![0, 2500, 0, 0, 2500, 0],
		},

		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_00_01_00_00,
			free_three_mask: 0b__0__0__1__0__1__0,
			mask_size: 6,
			compare_size: 6,
			threat_score: vec![0, 0, 2500, 0, 2500, 0],
		},

		ThreatLookupTableGenMask {
			stones_expected: 0b_00_01_00_00_01_00,
			free_three_mask: 0b__0__0__1__1__0__0,
			mask_size: 6,
			compare_size: 6,
			threat_score: vec![0, 0, 2500, 2500, 0, 0],
		},

		// 1 align
		ThreatLookupTableGenMask {
			stones_expected: 0b_00_00_00_00_01_00_00_00_00,
			free_three_mask: 0,
			mask_size: 9,
			compare_size: 5,
			threat_score: vec![2, 2, 2, 1, 0, 1, 2, 2, 2],
		},

		// TODO ?? Capturable cell debuf
		// ThreatLookupTableGenMask {
		// 	stones_expected: 0b_00_01_00_10,
		// 	free_three_mask: 0,
		// 	mask_size: 4,
		// 	compare_size: 4,
		// 	score: vec![0, 0, -50_000, 0],
		// },
		// ThreatLookupTableGenMask {
		// 	stones_expected: 0b_00_00_01_10,
		// 	free_three_mask: 0,
		// 	mask_size: 4,
		// 	compare_size: 4,
		// 	score: vec![0, -50_000, 0, 0],
		// },
	];

	let rev_inv_variants = [(true, true), (false, false), (true, false), (false, true)];

	let mut table = core::array::from_fn(|_| {
		LookupTableValue {
			free_three_mask: 0,
			threat_score: core::array::from_fn(|_| {
				ThreatMinMaxScore { min: 0, max: 0 }
			}),
		}
	});
	let mut i = 0;
	while i < table.len() {
		let res = &mut table[i];

		// all masks
		let mut mask_i = 0;
		while mask_i < masks.len() {
			let mask = &masks[mask_i];

			// rev + inv
			let mut rev_inv_i = 0;
			while rev_inv_i < rev_inv_variants.len() {
				let (rev, inv) = rev_inv_variants[rev_inv_i];

				let mut stones_expected = mask.stones_expected;
				if rev { stones_expected = reverse_stones_mask(stones_expected, mask.mask_size); }
				if inv { stones_expected = invert_stones_mask(stones_expected, mask.mask_size); }

				let mut free_three_mask = mask.free_three_mask;
				if rev { free_three_mask = reverse_threat_mask(free_three_mask, mask.mask_size); }

				// take the submask
				let mut mask_suboffset = 0;
				while mask_suboffset + mask.compare_size <= mask.mask_size {

					let sub_stones_mask = (1 << (mask.compare_size * 2)) - 1;
					let sub_stones_expected = (stones_expected >> (mask_suboffset * 2))
							& sub_stones_mask;

					let sub_free_three_mask = (free_three_mask >> mask_suboffset)
							& ((1 << mask.compare_size) - 1);

					// move the mask
					let mut mask_offset = 0;
					while mask_offset + mask.compare_size <= STONES_COUNT {
						if (i >> (mask_offset * 2)) & sub_stones_mask == sub_stones_expected {
							res.free_three_mask |= (sub_free_three_mask << mask_offset) as BitMask;

							let mut j = 0;
							while j < mask.compare_size {
								let current_score = &mut res.threat_score[(STONES_COUNT - 1) - mask_offset - j];
								let iter_score = mask.threat_score[
									if rev { mask_suboffset + j }
									else { (mask.mask_size - 1) - mask_suboffset - j }
								];
								if inv {
									current_score.min = current_score.min.min(-iter_score);
								} else {
									current_score.max = current_score.max.max(iter_score);
								}
								j += 1;
							}
						}

						mask_offset += 1;
					}

					mask_suboffset += 1;
				}

				rev_inv_i += 1;
			}

			mask_i += 1;
		}

		i += 1;
	}
	table
}
