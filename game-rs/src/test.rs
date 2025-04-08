use std::collections::BTreeMap;

use super::*;

const PLAYERS: [(Player, Player); 2] = [(Player::Player1, Player::Player2),
                                        (Player::Player2, Player::Player1)];

fn non_parallel_directions_pairs<'a>() -> Vec<(&'a Location, &'a Location)> {
	directions::ALL.iter()
	               .flat_map(|&a| directions::ALL.iter().map(move |&b| (a, b)))
	               .filter(|&(a, b)| a != b && a != &b.neg())
	               .collect::<Vec<_>>()
}

impl std::fmt::Display for Player {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::Player1 => "\x1b[1m1",
			Self::Player2 => "\x1b[1m2",
			Self::None => ".",
			Self::OutOfBounds => "\x1b[1mE",
		})
	}
}

impl From<char> for Player {
	fn from(c: char) -> Self {
		match c {
			'1' => Self::Player1,
			'2' => Self::Player2,
			'.' => Self::None,
			_ => Self::OutOfBounds,
		}
	}
}

struct TestGameState {
	state:       PackedGameState,
	trash_index: usize,
}

impl TestGameState {
	pub fn new() -> Self {
		Self {
			state: PackedGameState::new(),
			trash_index: 0,
		}
	}

	fn trash_move(&mut self) {
		let location = Location::from_index(self.trash_index);
		self.make_move(&location);
		self.trash_index += 2;
	}

	fn _check_move(&mut self, expected: bool, location: &Location) {
		let poi: BTreeMap<_, _> = [(location.index, "\x1b[1;91m".to_owned())].into();

		let before = self.debug(&poi);
		let got = self.state.make_move(location);
		if (got != MoveResult::Invalid) != expected {
			eprintln!("before:");
			eprintln!("{before}");
			eprintln!("after:");
			eprintln!("{}", self.debug(&poi));
			println!("player: {}", self.state.current_player);
			panic!("expected {:?} move at {:?} got {:?}",
				if expected { "valid" } else { "invalid" },
				location,
				got,
			);
		}
	}

	fn make_move<T: std::borrow::Borrow<Location>>(&mut self, location: T) {
		self._check_move(true, location.borrow());
	}

	fn failing_move<T: std::borrow::Borrow<Location>>(&mut self, location: T) {
		self._check_move(false, location.borrow());
	}

	#[must_use]
	pub fn debug(&self, poi: &BTreeMap<usize, String>) -> String {
		let mut board = String::new();
		for y in 0..BOARD_SIZE_I {
			for x in 0..BOARD_SIZE_I {
				let location = Location::from(x, y);

				if let Some(poi) = poi.get(&location.index) {
					board.push_str(poi);
				}
				board.push_str(&self.state.get_stone_at(location.index).to_string());
				board.push_str("\x1b[0m");
			}
			board.push('\n');
		}

		board
	}

	fn assert_eq(&self, expected_board_str: &str) {
		let expected_board = expected_board_str.chars()
		                                       .filter(|c| !c.is_whitespace())
		                                       .collect::<Vec<_>>();

		if expected_board.len() != CELL_COUNT {
			eprintln!("got:");
			eprintln!("{}", self.debug(&BTreeMap::new()));
			panic!("expected_board has a wrong size");
		}

		expected_board.iter()
            .enumerate()
            .for_each(|(i, expected_char)| {
                let location = Location::from_index(i);
                let expected_player = Player::from(*expected_char);

                if self.state.get_stone_at(location.index) != expected_player {
                    eprintln!("\x1b[1;92mexpected\x1b[0m \x1b[1;91mgot\x1b[0m");
                    eprintln!("{}", self.debug(
                        &expected_board.iter()
                            .enumerate()
                            .map(|(i, expected_char)| {
                                let location = Location::from_index(i);

                                (
                                    location.index,
                                    if self.state.get_stone_at(location.index) == Player::from(*expected_char) {
                                        "  ".to_owned()
                                    } else {
                                        format!(" \x1b[1;92m{expected_char}\x1b[1;91m")
                                    }
                                )
                            })
                            .collect::<BTreeMap<_, _>>()
                    ));
                    panic!("expected {} at {:?}", expected_player, location);
                }
            });
	}
}

impl std::fmt::Display for TestGameState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.debug(&BTreeMap::new()))
	}
}

#[test]
fn test_non_parallel_directions_pairs() {
	assert_eq!(non_parallel_directions_pairs().len(), 48);
}

#[test]
fn already_occupied() {
	let mut game = TestGameState::new();

	game.make_move(Location::CENTER);
	game.failing_move(Location::CENTER);

	game.assert_eq("
		...................
		...................
		...................
		...................
		...................
		...................
		...................
		...................
		...................
		.........1.........
		...................
		...................
		...................
		...................
		...................
		...................
		...................
		...................
		...................
	");
}

#[test]
fn win_align_5() {
	for (winner, _) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			let mut game = TestGameState::new();

			if winner == Player::Player2 {
				game.trash_move();
			}

			for i in 1..5 {
				game.make_move(Location::CENTER.add(&a_dir.mul(i)));
				game.make_move(Location::CENTER.add(&a_dir.mul(i)).add(b_dir));
			}

			game.make_move(Location::CENTER);

			assert_eq!(game.state.winner, winner);
			game.failing_move(Location::MAX);
		}
	}
}

#[test]
fn win_align_5_middle() {
	for (winner, _) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			let mut game = TestGameState::new();

			if winner == Player::Player2 {
				game.trash_move();
			}

			for i in 0..=1 {
				game.make_move(Location::CENTER.add(&a_dir.mul(i)));
				game.make_move(Location::CENTER.add(&a_dir.mul(i)).add(b_dir));
			}

			for i in 3..=4 {
				game.make_move(Location::CENTER.add(&a_dir.mul(i)));
				game.make_move(Location::CENTER.add(&a_dir.mul(i)).add(b_dir));
			}

			game.make_move(Location::CENTER.add(&a_dir.mul(2)));

			assert_eq!(game.state.winner, winner);
			game.failing_move(Location::MAX);
		}
	}
}

#[test]
fn win_by_capture() {
	for (winner, loser) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			let mut game = TestGameState::new();

			if winner == Player::Player1 {
				game.trash_move();
			}

			for i in 0..5 {
				let base_location = Location::CENTER.add(&a_dir.mul((i - 2) * 2));

				let inner_stone1 = &base_location.add(&b_dir.mul(2));
				let inner_stone2 = &base_location.add(&b_dir.mul(3));

				let outer_stone1 = &base_location.add(&b_dir.mul(1));
				let outer_stone2 = &base_location.add(&b_dir.mul(4));

				eprintln!("{outer_stone1:?} {inner_stone1:?} {inner_stone2:?} {outer_stone2:?}");

				game.make_move(inner_stone1);
				game.make_move(outer_stone1);
				game.make_move(inner_stone2);
				game.make_move(outer_stone2);

				assert_eq!(game.state.get_player_captures_count(winner), i as u8 + 1);
				assert_eq!(game.state.get_player_captures_count(loser), 0);

				assert_eq!(game.state.get_stone_at(inner_stone1.index), Player::None);
				assert_eq!(game.state.get_stone_at(inner_stone2.index), Player::None);
				assert_eq!(game.state.get_stone_at(outer_stone1.index), winner);
				assert_eq!(game.state.get_stone_at(outer_stone2.index), winner);
			}

			assert_eq!(game.state.winner, winner);
			game.failing_move(Location::ZERO);
		}
	}
}

#[test]
fn forbidden_double_three() {
	for (a_dir, b_dir) in non_parallel_directions_pairs() {
		let mut game = TestGameState::new();

		game.make_move(Location::A.add(a_dir));
		game.make_move(Location::B.add(a_dir));

		game.make_move(Location::A.add(&a_dir.mul(2)));
		game.make_move(Location::B.add(&a_dir.mul(2)));

		game.make_move(Location::A.add(b_dir));
		game.make_move(Location::B.add(b_dir));

		game.make_move(Location::A.add(&b_dir.mul(2)));
		game.make_move(Location::B.add(&b_dir.mul(2)));

		game.failing_move(Location::A);
		game.trash_move();
		game.failing_move(Location::B);
	}
}

#[test]
fn break_win_align_5() {
	for (winner, _) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			for break_i in 0..5 {
				let mut game = TestGameState::new();

				if winner == Player::Player2 {
					game.trash_move();
				}

				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).add(b_dir));
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).sub(b_dir));

				// Create the winning align
				for i in 0..5 {
					game.trash_move();
					game.make_move(Location::CENTER.add(&a_dir.mul(i)));
				}

				// Can only break the alignment
				for i in 0..5 {
					if i != break_i {
						game.failing_move(Location::CENTER.add(&a_dir.mul(i)).sub(&b_dir.mul(2)));
					}
				}
				game.failing_move(Location::MAX);

				// Capture winning align
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).sub(&b_dir.mul(2)));

				game.trash_move();
				game.trash_move();

				assert_eq!(game.state.winner, Player::None);
			}
		}
	}
}

#[test]
fn break_win_align_6() {
	for (winner, _) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			for break_i in 1..5 {
				let mut game = TestGameState::new();

				if winner == Player::Player2 {
					game.trash_move();
				}

				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).add(b_dir));
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).sub(b_dir));

				// Create the winning align
				for i in 0..6 {
					if i != break_i {
						game.trash_move();
						game.make_move(Location::CENTER.add(&a_dir.mul(i)));
					}
				}

				game.trash_move();
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)));

				// Can only break the alignment
				for i in 0..6 {
					if i != break_i {
						game.failing_move(Location::CENTER.add(&a_dir.mul(i)).sub(&b_dir.mul(2)));
					}
				}
				game.failing_move(Location::MAX);

				// Capture winning align
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).sub(&b_dir.mul(2)));

				game.trash_move();
				game.trash_move();

				assert_eq!(game.state.winner, Player::None);
			}
		}
	}
}

#[test]
fn cannot_break_win_align_6() {
	for (winner, _) in PLAYERS {
		for (a_dir, b_dir) in non_parallel_directions_pairs() {
			for break_i in [0, 5] {
				let mut game = TestGameState::new();

				if winner == Player::Player1 {
					game.trash_move();
				}

				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).add(b_dir));
				game.make_move(Location::CENTER.add(&a_dir.mul(break_i)).sub(b_dir));

				let final_align_i = 3;

				// Create the winning align
				for i in 0..6 {
					if i != final_align_i {
						game.trash_move();
						game.make_move(Location::CENTER.add(&a_dir.mul(i)));
					}
				}

				game.trash_move();
				game.make_move(Location::CENTER.add(&a_dir.mul(final_align_i)));

				assert_eq!(game.state.winner, winner);
			}
		}
	}
}
