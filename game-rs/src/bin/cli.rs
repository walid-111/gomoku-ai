// cargo run --bin cli --release

// pgo + bolt + lto

use std::time::Instant;

use game_rs::{ai_player::default_ai_pick_move, location::Location, MoveResult, PackedGameState, Player};

fn main() {
	let start_time = Instant::now();

	let mut game_state = PackedGameState::new();
	let mut move_count = 0;

	while game_state.winner == Player::None {
		let move_start_time = Instant::now();
		let ai_res = default_ai_pick_move(&game_state);
		assert!(game_state.make_move(&Location::from_index(ai_res.picked_move)) != MoveResult::Invalid);
		println!("ai move: {} ({}ms)", ai_res.picked_move, move_start_time.elapsed().as_millis());
		move_count += 1;
	}

	println!("played {} moves", move_count);
	let run_time = start_time.elapsed();
	println!("run time: {:?}", run_time);
	println!("time / m: {:?}s", run_time.as_secs_f64() / move_count as f64);
}
