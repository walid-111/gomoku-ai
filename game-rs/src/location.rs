use crate::*;

#[derive(Clone)]
pub struct Location {
	pub x: isize,
	pub y: isize,
	pub index: usize,
}

impl std::fmt::Debug for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "loc({},{})", self.x, self.y)
	}
}

impl Location {
	pub const ZERO: &Self = &Self::from(0, 0);
	pub const CENTER: &Self = &Self::from(BOARD_SIZE_I / 2, BOARD_SIZE_I / 2);
	pub const MAX: &Self = &Self::from(BOARD_SIZE_I - 1, BOARD_SIZE_I - 1);

	pub const A: &Self = &Self::from(BOARD_SIZE_I / 3    , BOARD_SIZE_I / 2);
	pub const B: &Self = &Self::from(BOARD_SIZE_I / 3 * 2, BOARD_SIZE_I / 2);

	#[inline]
	#[must_use]
	pub const fn manhattan_distance(&self, other: &Self) -> isize {
		(self.x - other.x).abs() + (self.y - other.y).abs()
	}

	#[inline]
	#[must_use]
	pub const fn is_in_bound(&self) -> bool {
		self.x >= 0 && self.x < BOARD_SIZE_I && self.y >= 0 && self.y < BOARD_SIZE_I
	}

	#[inline]
	#[must_use]
	pub const fn from(x: isize, y: isize) -> Self {
		Self {
			x,
			y,
			index: (BOARD_SIZE_I * y + x) as usize
		}
	}

	#[inline]
	#[must_use]
	pub const fn from_index(index: usize) -> Self {
		Self {
			x: (index % BOARD_SIZE) as isize,
			y: (index / BOARD_SIZE) as isize,
			index
		}
	}

	#[inline]
	#[must_use]
	pub const fn add(&self, other: &Self) -> Self {
		Self {
			x:     self.x.wrapping_add(other.x),
			y:     self.y.wrapping_add(other.y),
			index: self.index.wrapping_add(other.index),
		}
	}

	#[inline]
	#[must_use]
	pub const fn sub(&self, other: &Self) -> Self {
		Self {
			x:     self.x.wrapping_sub(other.x),
			y:     self.y.wrapping_sub(other.y),
			index: self.index.wrapping_sub(other.index),
		}
	}

	#[inline]
	#[must_use]
	pub const fn neg(&self) -> Self {
		Self {
			x:    -self.x,
			y:    -self.y,
			index: self.index.wrapping_neg(),
		}
	}

	#[inline]
	#[must_use]
	pub const fn mul(&self, other: isize) -> Self {
		Self {
			x:     self.x.wrapping_mul(other),
			y:     self.y.wrapping_mul(other),
			index: self.index.wrapping_mul(other as usize),
		}
	}
}

impl std::cmp::PartialEq for Location {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}
impl std::cmp::Eq for Location {}

impl std::cmp::PartialOrd for Location {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other))
	}
}
impl std::cmp::Ord for Location {
	#[inline]
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.index.cmp(&other.index)
	}
}


pub mod directions {
	use super::Location;

	pub const LEFT:    &Location = &Location::from(-1,  0);
	pub const RIGHT:   &Location = &Location::from( 1,  0);
	pub const BOTTOM:  &Location = &Location::from( 0,  1);
	pub const TOP:     &Location = &Location::from( 0, -1);

	pub const BOTTOM_LEFT:  &Location = &Location::from(-1,  1);
	pub const BOTTOM_RIGHT: &Location = &Location::from( 1,  1);
	pub const TOP_LEFT:     &Location = &Location::from(-1, -1);
	pub const TOP_RIGHT:    &Location = &Location::from( 1, -1);

	pub const ALL: &[&Location; 8] = &[
		LEFT, RIGHT, BOTTOM, TOP, BOTTOM_LEFT, BOTTOM_RIGHT, TOP_LEFT, TOP_RIGHT,
	];

	pub const DIAGONAL: &[&Location; 4] = &[
		BOTTOM_LEFT, BOTTOM_RIGHT, TOP_LEFT, TOP_RIGHT,
	];
	pub const ORTHOGONAL: &[&Location; 4] = &[
		LEFT, RIGHT, BOTTOM, TOP,
	];

	pub const ALL_AXIS: &[&Location; 4] = &[
		RIGHT, BOTTOM,
		BOTTOM_RIGHT, BOTTOM_LEFT
	];
}


pub type PackedLocation = u16;

pub const NO_MOVE: PackedLocation = 0xFFFF;
