use crate::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Opening {
	None = 0,
	Pro = 1,
	LongPro = 2,
}

impl Opening {
	#[inline]
	#[must_use]
	pub fn is_valid_opening_move(&self, location: &Location, turn: u32) -> bool {
		match self {
			Self::None => true,
			Self::Pro     => Self::is_valid_pro_move(location, turn, 3),
			Self::LongPro => Self::is_valid_pro_move(location, turn, 4),
		}
	}

	fn is_valid_pro_move(location: &Location, turn: u32, distance: isize) -> bool {
		match turn {
			0 => {
				// First player must place in the center
				location == Location::CENTER
			}
			2 => {
				// First player's second move must be at least `distance` intersections away
				distance <= Location::CENTER.manhattan_distance(location)
			}
			_ => true,
		}
	}
}
