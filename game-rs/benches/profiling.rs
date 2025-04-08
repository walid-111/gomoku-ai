use game_rs::{ai_player::default_ai_pick_move, location::{directions, Location}, MoveResult, PackedGameState, Player};
use iai_callgrind::{library_benchmark, library_benchmark_group, main, FlamegraphConfig, LibraryBenchmarkConfig};
use std::hint::black_box;

struct BenchData {
	moves: Vec<usize>,
	/// expected winner after ai play
	expected_winner: Player
	// best_move: Option<usize>
}

macro_rules! bench_all {
	($fn:item) => {
		#[library_benchmark]
		#[bench::empty(&BenchData {
			moves: vec![],
			expected_winner: Player::None
		})]
		#[bench::center_stone(&BenchData {
			moves: vec![Location::CENTER.index],
			expected_winner: Player::None
		})]
		#[bench::simple_winning_alignment(&BenchData {
			moves: vec![
				Location::CENTER.index,
				Location::CENTER.add(directions::RIGHT).index,
				Location::CENTER.add(directions::TOP).index,
				Location::CENTER.add(directions::TOP).add(directions::RIGHT).index,
				Location::CENTER.add(&directions::TOP.mul(2)).index,
				Location::CENTER.add(&directions::TOP.mul(2)).add(directions::RIGHT).index,
				Location::CENTER.add(&directions::TOP.mul(3)).index,
				Location::CENTER.add(&directions::TOP.mul(3)).add(directions::RIGHT).index,
			],
			expected_winner: Player::Player1
		})]
		#[bench::slow_case(&BenchData {
			moves: vec![181,101,163,145,179,159],
			expected_winner: Player::None
		})]
		#[bench::pentevpente1(&BenchData {
			moves: vec![180,199,236,239,179,181,217,178,163,198,255,293,218,158,219,118,138,216,221,220,201,237,258,183,219,220,254,200,257,256,275,256,239,259,257,199,239],
			expected_winner: Player::Player2
		})]
		#[bench::pentevpente2(&BenchData {
			moves: vec![180,199,177,235,217,197,179,178,216,198,159,139,179,178,218,215,220,219,239,219,158,196,177,178,234,181,215,214,196,176,158,236],
			expected_winner: Player::Player1
		})]
		#[bench::pentevpente3(&BenchData {
			moves: vec![180,142,176,182,140,218,158,122,162,200,212,194,236,146,164,144,102,198,104,122,196,216,180,156,234,176,158,145,143,148,147,163,201,106,125,161,107,127,89,71,109,181,183,182,165,166,165,164],
			expected_winner: Player::Player1
		})]
		#[bench::pentevpente4(&BenchData {
			moves: vec![180,181,239,143,220,200,162,219,238,198,144,201,202,258,126,108,199,161,220,256,166,184,182,222,201,163,200,218,203,164,204,184],
			expected_winner: Player::Player1
		})]
		#[bench::jbettini_slow(&BenchData {
			moves: vec![180,160,142,161,162,144,159,182,160],
			expected_winner: Player::None
		})]
		// [180,160,234,198,237,179,141,199,159,219,177,123,195,213,236,235,238,239,259,217,253,163,181] not 143 -> 196
		// #[bench::pentevpente5(&BenchData {
		// 	moves: vec![180,199,260,201,200,220,182,160,239,220,180,179,198,141,162,122,103,144,216,234,222,202,184,142,203,241,143,182,162,124,102,181,146,165,104,105,101,100,164,144,159,219,221,223,222,204],
		// 	expected_winner: Player::Player2
		// })]
		// #[bench::pentevpente2(&BenchData {
		// 	moves: vec!,
		// 	expected_winner: Player::Player1
		// })]
		$fn
	};
}

bench_all!(fn bench_eval_function(data: &BenchData) {
	_ = black_box({
		let mut game_state = PackedGameState::new();
		for &index in data.moves.iter() {
			assert_eq!(game_state.make_move(&Location::from_index(index)), MoveResult::Ok);
		}
		game_state.evaluate(0)
	})
});

bench_all!(fn bench_make_move(data: &BenchData) {
	_ = black_box({
		let mut game_state = PackedGameState::new();
		for &index in data.moves.iter() {
			assert_eq!(game_state.make_move(&Location::from_index(index)), MoveResult::Ok);
		}
		game_state
	})
});

bench_all!(fn bench_ai_player(data: &BenchData) {
	_ = black_box({
		let mut game_state = PackedGameState::new();
		for &index in data.moves.iter() {
			assert_eq!(game_state.make_move(&Location::from_index(index)), MoveResult::Ok);
		}
		assert_ne!(game_state.make_move(&Location::from_index(default_ai_pick_move(&game_state).picked_move)), MoveResult::Invalid);
		assert_eq!(game_state.winner, data.expected_winner);
		game_state
	});
});

library_benchmark_group!(
	name = bench_ai_group;
	benchmarks = bench_eval_function, bench_make_move, bench_ai_player
);

main!(
	config = LibraryBenchmarkConfig::default()
		.flamegraph(FlamegraphConfig::default())
		.truncate_description(Some(0));
	library_benchmark_groups = bench_ai_group
);
