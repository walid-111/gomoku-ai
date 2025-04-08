use heuristic::WIN_SCORE;
use lookup::{use_lookup_table, LOOKUP_TABLE_USE_SCORING, LOOKUP_TABLE_USE_ORDERING};
use threading::{thread_send, TaskRequest, TaskResult};

use futures::{executor::block_on, future::join_all};

use std::marker::ConstParamTy;

use crate::*;

mod heuristic;

// TODO fix count on sides ?
// TODO ui slight arrow between sidetab & board to close / open on phone
// TODO ui leaderboard + store all matches
// TODO fix compute turn win if equal
// TODO bounding box opti for lookup table

// blue win (cant defent bcz double three) [180,160,198,162,216,252,217,159,161,158,236,255,215,218,214,213,196,156,157,256,199,197,235,195,235,254,253,216]
// [180,160,198,162,216,252,235,197,254,273,215,163,161,164,199,218,142,123,214,200,213,212,217]
// [180,160,198,162,216,252,217,159,161,158,199,218,142,123,236,215,217,157,156,255,200,220,200,197,201,202,163,181,143,181,144,235,145,141,146,180,147,183]
// [180,179,160,200,140,120,162,144,198,159,216,234,139,141,138,137,199,197,198,217,178,177,157,237,257,119,158,176,157,121,122,175,157,118,117,156,194,99]
// [343,307,324,288,305,267,287,269,250,323,304,326,345,270,268,271,286,232,251,252,212,234,272,232,252,253,215,253,249,248,231,289,235,325,309,289,269,271,327]
// [180,18,140,56,160,200,100,120,102,37,101,75,103,94]
// [180,161,160,200,140,120,122,158,104,139,101,103,196,85,121,123,121,102,159,178,67,198,138,201,84,199,202,197,216,197]
// [67,85,68,66,104,47,86,50,122,140,28,69,31,88,107,105,65,125,145,87,89,71,88,68,69,50,107,126,127,124,123,142,104,143,141,144,164,144,183,161,179,160,202,221,159,162,163,181,124,126,32]
// [180,199,198,216,162,144,160,140,200,220,179,141,178,142,143,138,139,181,124,163,196,217,235,145,146,145]
// [180,199,198,162,216,181,234,252,143,200,201,200,238,219,178,218,158,236,138,118,182,254,272,253,255,251,250,220,163,221,222,217,240,220]
// [180,160,199,161,162,159,218,144,198,158,157,197,216,234,217,200,219,215,220]
// [180,162,142,161,160,200,178,140,160,196,124,106,163,197,161,162,123,180,198,218,144,125,182,177,158,157,122,121,236,217,237,137,195,197]
// [180,160,140,122,141,142,124,162,178,142,102,139,140,182,202,121,120,84,123,100,121,160,122,120,125,82,122]
// [161,141,121,103,122,123,105,143,159,123,163,63,83,120,121,102,84,138,156,82,101,141,104,103,86,81,80,102,122,101,100,106,82,85,81,60,80]
// [181,163,200,162,161,164,201,221,141,121,199,219,200,181,202,203,198]
// [180,160,139,162,179,178,181,196,183,182,142,199,140,202,141,143,138]
// [180,160,234,198,237,179,141,199,159,219,177,123,195,213,236,235,238,239,259,217,253,163,181,143,196,215,255,235,215,175,215]

// [180,160,234,198,237,179,141,199,159,219,177,123,195,213,236,235,238,239,259,217,253,163,181,143]
// [180,160,196,198,217,158,157,178,142,178,218,138,118,219,216,176,199,161,256,237,235,236,216,238,181,163,253]

// [180,160,179,181,178,177,197,233,161,199,143,125,215,142,161,180,179]

// TODO fix compute time for threat / in advance cant win
//      + use max instead or in + of add ?


// TODO
// - improve ordering and value heuristic
// - optimize
//   - align checks and stuff using threat ?
//   - avoid cloning of array and stuff in PackedGameState
//   - vec with no alloc (constant size)
//   - multi threat base of tree and share result
//   - avoid dup processing in threat analyser
//   - simd for threat etc ?
//   - play all opp move at same time in first pass ?
//   - ordering heuristic without make_move ?
//   - avoid collect to pass to threat
// - only keep moves that force play as threat ??
// - full board threat space?? else could forget
// - use threat space for ordering
// - fix win ours forfeit against pente ai
// do not limit ai / prevent forget

// don't ignore opp threat

// ---

// Algos:
// - https://en.wikipedia.org/wiki/MTD(f) | negascout | https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
// X - https://datascience.stackexchange.com/questions/10932/difference-between-alphagos-policy-network-and-value-network
// Policy-value network https://freedium.cfd/https://medium.com/@mrconnor/mastering-gomoku-with-alphazero-a-step-by-step-guide-e6d75cfb74be

// Strategy:
// - https://en.wikipedia.org/wiki/Pente#Strategy_and_tactics
// - Identify dangerous moves

// Game database:
// - https://pente.org/gameServer/controller/search?quick_start=1

// Ai:
// - https://www.myabandonware.com/game/pente-1qk/play-1qk
// - https://stanford-cs221.github.io/autumn2019-extra/posters/14.pdf
// - https://kevin-qualls.github.io/AlphaGomoku/

// Online game:
// - https://pente.org/gameServer/mmai/
// - https://www.myabandonware.com/game/pente-1qk/play-1qk

// Minmax opti: https://arxiv.org/pdf/2006.08141
// - Reduce search space: Threat-space search
// 		- Killer move / defence https://www.youtube.com/watch?v=i35ErvsbdgQ&t=14s / History Heuristic
// 		- Prio/lines and not working toghether https://www.youtube.com/watch?v=1boqoa2rQfU
// 		- Fast preorder before gen ??

// Polynomial fn for length score ??
// - If win for opp/or use don't check other moves ?
// - Improved heuristic & presort
// - Skip opp moves ?? make all defensive moves at once
// - Monte carlo
// - Parrallelize (rayon lib) ??
// - "Forced moves" (need block else win) ??
// - Efficient storage (bitboard etc)
// - Only play defensive as a last resort
// - Reject if opp can win
// - Use alpha go technique?
// - Do not do threat that lead to nothing? / unnecessary threats
// - Eval all counter moves at the same time ?
// TODO How to handle if opp can win with no mistake


// https://github.com/zuoyigehaobing/Gomoku-GameTreeSearch/blob/master/csc384-project.pdf
// https://www.cs.cornell.edu/courses/cs2110/2022fa/hw/a5/a5.pdf
// https://web.mit.edu/dimitrib/www/LessonsfromAlphazero.pdf
// https://suragnair.github.io/posts/alphazero.html
// https://stackoverflow.com/questions/47838665/alphago-zero-board-evaluation-function-uses-multiple-time-steps-as-an-input-w
// https://medium.com/@LukeASalamone/creating-an-ai-for-gomoku-28a4c84c7a52
// https://kevin-qualls.github.io/AlphaGomoku/
// https://github.com/2ToTheNthPower/Pente-AI/blob/main/pente_zero.ipynb
// https://en.wikipedia.org/wiki/Temporal_difference_learning
// https://en.wikipedia.org/wiki/Q-learning
// https://peizhiyan.github.io/docs/paper_informal.pdf
// https://blog.theofekfoundation.org/artificial-intelligence/2015/12/11/minimax-for-gomoku-connect-five/

// [180,160,240,161,159,162,164,124,202,143,183,221,181,179,145,126,182,197]
// [217,235,218,216,219,254,273,221,197,177,237,255,201,256,257,277,253,276,296,196,236,256,183,165,158,215,176,216,214,216,275,194,215,255,274,233,272,271,276,293,273,239,275,316,276]

// minmax (3 depth):
// 1621275 nodes
// 1070.2ms

// alphabeta (3 depth):
// 441217 nodes
// 398.9ms

// alphabeta root node (3 depth):
// 189604 nodes
// 149.8ms

// negascout (3 depth):
// 22822 nodes
// 65.0ms

// prev: 435 633 n/s
// new : 126 885 n/s
// new2: 568 173 n/s

pub const MINMAX_DEPTH: usize = 10;

pub const MOVES_LIMIT_LARGE: usize = 10;
pub const MOVES_LIMIT_SMALL: usize = 4;

#[wasm_bindgen]
pub struct AIResult {
	#[wasm_bindgen(readonly, js_name = pickedMove)]
	pub picked_move: usize,
	#[wasm_bindgen(readonly, js_name = pickedMoveScore)]
	pub picked_move_score: f32,

	debug_data: Option<AIResultDebugData>,
}

struct AIResultDebugData {
	game_state: PackedGameState,
	moves_list: Vec<Location>,

	root_move_score: [Option<f32>; CELL_COUNT],
}

pub struct AIRuntimeData {
	debug_evaluated_moves: [usize; MINMAX_DEPTH],
	debug_pruned_branch: [usize; MINMAX_DEPTH],
	debug_sorting: [usize; MINMAX_DEPTH],

	debug_make_moves_count: usize,
	debug_regen_make_moves_count: usize,

	debug_root_move_score: [Option<f32>; CELL_COUNT],
}

#[repr(u8)]
#[derive(PartialEq, Eq, ConstParamTy)]
pub enum OptionDebug {
	Enabled,
	Disabled
}

impl OptionDebug {
	fn enabled(self) -> bool {
		self == Self::Enabled
	}
}

#[repr(u8)]
#[derive(PartialEq, Eq, ConstParamTy)]
pub enum OptionMultiThreading {
	Enabled,
	Disabled
}

impl OptionMultiThreading {
	fn enabled(self) -> bool {
		self == Self::Enabled
	}
}


// NegaScout
// https://www.chessprogramming.org/NegaScout
fn negascout<const DEBUG: OptionDebug>(
	runtime_data: &mut AIRuntimeData,
	game_state: &mut PackedGameState,
	depth: usize,
	mut alpha: f32,
	mut beta: f32,
) -> f32 {
	if DEBUG.enabled() {
		runtime_data.debug_evaluated_moves[depth] += 1;
	}

	if depth == 0 || game_state.winner != Player::None {
		use_lookup_table::<LOOKUP_TABLE_USE_SCORING>(game_state);
		let side = if game_state.turn & 1 == 1 { -1.0 } else { 1.0 };
		return game_state.evaluate(depth) * side;
	}

	let next_depth = depth - 1;
	let next_game_state = unsafe { std::ptr::from_mut(game_state).add(1).as_mut().unwrap() };

	let instant_win = WIN_SCORE*depth as f32;
	beta = beta.min(instant_win);

	let moves_list = game_state.sorted_moves_list(MOVES_LIMIT_SMALL);

	if DEBUG.enabled() {
		runtime_data.debug_sorting[depth] += 1;
		runtime_data.debug_make_moves_count += moves_list.len();
		runtime_data.debug_pruned_branch[depth] += moves_list.len();
	}

	for move_location in moves_list {
		if DEBUG.enabled() {
			runtime_data.debug_make_moves_count += 1;

			if 1 < depth {
				runtime_data.debug_regen_make_moves_count += 1;
				runtime_data.debug_pruned_branch[depth] -= 1;
			}
		}

		next_game_state.clone_from(game_state);
		next_game_state.make_move_unchecked(&move_location);

		let t = -negascout::<DEBUG>(runtime_data, next_game_state, next_depth, -beta, -alpha);

		alpha = alpha.max(t);

		if alpha >= beta {
			break;
		}
	}

	alpha
}

/// # Panics
/// Panics if there are no moves
#[must_use]
pub async fn ai_pick_move<
	const DEBUG: OptionDebug,
	const MULTI_THREADING: OptionMultiThreading
>(moves: &[PackedLocation], root_game_state: &mut PackedGameState, depth: usize) -> AIResult {
	if let Some(opening_move) = opening_table::get_opening_move(root_game_state) {
		if root_game_state.is_valid_move(&Location::from_index(opening_move as usize)) {
			return AIResult {
				picked_move: opening_move as usize,
				picked_move_score: 0.0,
				debug_data: None,
			};
		}
	}

	let next_minmax_depth: usize = depth-1;

	let mut game_state_stack: [PackedGameState; MINMAX_DEPTH * 3] = core::array::from_fn(|_| PackedGameState::new());
	let next_game_state: &mut PackedGameState = unsafe { game_state_stack.as_mut_ptr().add(MINMAX_DEPTH * 3 / 2).as_mut().unwrap() };

	let mut runtime_data = AIRuntimeData {
		debug_evaluated_moves: [0; MINMAX_DEPTH],
		debug_pruned_branch: [0; MINMAX_DEPTH],
		debug_sorting: [0; MINMAX_DEPTH],

		debug_make_moves_count: 0,
		debug_regen_make_moves_count: 0,

		debug_root_move_score: [None; CELL_COUNT],
	};

	let instant_win = WIN_SCORE*MINMAX_DEPTH as f32;

	let beta = instant_win;
	let mut alpha = -instant_win;

	let moves_list = root_game_state.sorted_moves_list(MOVES_LIMIT_LARGE);

	let mut picked_move: usize = moves_list.first().map_or(0, |m| m.index);

	if 1 < moves_list.len() && 1 < MINMAX_DEPTH {
		if DEBUG.enabled() {
			runtime_data.debug_make_moves_count += moves_list.len();
		}

		debug_time!("negascout", {
			if MULTI_THREADING.enabled() {
				let tasks: Vec<_> = debug_time!("create_tasks", { moves_list.iter().map(|m| {
					let mut task_moves = moves.iter().copied().collect::<Vec<_>>();
					task_moves.push(m.index as PackedLocation);

					thread_send(TaskRequest {
						moves: task_moves,
						depth: next_minmax_depth,
						beta: -alpha,
						alpha: -beta,
					})
				}).collect()});

				let results = debug_time!("join_tasks", {
					join_all(tasks).await
				});

				for (i, &t) in results.iter().enumerate() {
					if alpha < t {
						alpha = t;
						picked_move = moves_list[i].index;
					}
				}
			} else {
				for move_location in &moves_list {
					if DEBUG.enabled() {
						runtime_data.debug_make_moves_count += 1;
						runtime_data.debug_regen_make_moves_count += 1;
					}

					next_game_state.clone_from(&root_game_state);
					next_game_state.make_move_unchecked(move_location);

					let t = -negascout::<DEBUG>(&mut runtime_data, next_game_state, next_minmax_depth, -beta, -alpha);

					if alpha < t {
						alpha = t;
						picked_move = move_location.index;
					}

					if DEBUG.enabled() {
						runtime_data.debug_root_move_score[move_location.index] = Some(t);
					}

					if alpha >= beta {
						break ;
					}
				}
			}
		});

		if DEBUG.enabled() {
			debug_log!("-----------------");
			debug_log!("{:?} depth", depth);
			debug_log!("{:?} evals", runtime_data.debug_evaluated_moves);
			debug_log!("{:?} pruned", runtime_data.debug_pruned_branch);
			debug_log!("{:?} sorting", runtime_data.debug_sorting);
			debug_log!("{:.2?} avg childs",
				runtime_data.debug_evaluated_moves.iter()
					.zip(runtime_data.debug_evaluated_moves.iter().skip(1))
					.map(#[allow(clippy::cast_precision_loss)] |(a, b)|
						if *b == 0 { 0.0 } else { *a as f32 / *b as f32 }
					).collect::<Vec<_>>()
			);
			debug_log!("total evals: {}", runtime_data.debug_evaluated_moves.iter().sum::<usize>());
			debug_log!("make_moves: {:?}", runtime_data.debug_make_moves_count);
			debug_log!("regen make_moves: {:?}", runtime_data.debug_regen_make_moves_count);
			debug_log!("alignments_score: {:?}", root_game_state.alignments_score);
			debug_log!("picked_move: {}", picked_move);
			debug_log!("picked_move_score: {:?}", alpha);

			if alpha.abs() > WIN_SCORE {
				debug_log!("win in {}", (alpha.signum() * (MINMAX_DEPTH as f32 - alpha.abs() / WIN_SCORE).floor()) as usize);
			}
		}
	}

	AIResult {
		picked_move,
		picked_move_score: alpha,
		debug_data: if DEBUG.enabled() {
			Some(AIResultDebugData {
				game_state: root_game_state.clone(),
				moves_list,
				root_move_score: runtime_data.debug_root_move_score,
			})
		} else {
			None
		},
	}
}

#[must_use]
#[wasm_bindgen(js_name = pickMoveDefault)]
pub fn default_ai_pick_move(root_game_state: &PackedGameState) -> AIResult {
	let mut root_game_state = root_game_state.clone();
	block_on(ai_pick_move::<{OptionDebug::Disabled}, {OptionMultiThreading::Disabled}>
		(&[], &mut root_game_state, MINMAX_DEPTH))
}

pub fn thread_handle(task: TaskRequest) -> TaskResult {
	let mut game_state = PackedGameState::new();
	for m in task.moves {
		assert_ne!(game_state.make_move(&Location::from_index(m as usize)), MoveResult::Invalid);
	}

	let mut game_state_stack: [PackedGameState; MINMAX_DEPTH * 3] = core::array::from_fn(|_| PackedGameState::new());
	let next_game_state: &mut PackedGameState = unsafe { game_state_stack.as_mut_ptr().add(MINMAX_DEPTH * 3 / 2).as_mut().unwrap() };
	next_game_state.clone_from(&game_state);

	let mut runtime_data = AIRuntimeData {
		debug_evaluated_moves: [0; MINMAX_DEPTH],
		debug_pruned_branch: [0; MINMAX_DEPTH],
		debug_sorting: [0; MINMAX_DEPTH],

		debug_make_moves_count: 0,
		debug_regen_make_moves_count: 0,

		debug_root_move_score: [None; CELL_COUNT],
	};

	-negascout::<{OptionDebug::Disabled}>(&mut runtime_data, next_game_state, task.depth, task.alpha, task.beta)
}


#[must_use]
#[wasm_bindgen(js_name = pickMove)]
pub async fn _ai_pick_move(moves: Vec<PackedLocation>, root_game_state: &PackedGameState, debug: bool) -> AIResult {
	let mut root_game_state = root_game_state.clone();

	if debug {
		ai_pick_move::<{OptionDebug::Enabled}, {OptionMultiThreading::Disabled}>
			(&moves, &mut root_game_state, MINMAX_DEPTH).await
	} else {
		ai_pick_move::<{OptionDebug::Disabled}, {OptionMultiThreading::Enabled}>
			(&moves, &mut root_game_state, MINMAX_DEPTH).await
	}
}



#[wasm_bindgen(js_name = updateOverlays)]
pub fn update_overlays(game_state: &PackedGameState) {
	let mut game_state = game_state.clone();

	let new_game_states = (0..CELL_COUNT).map(|i| {
		let mut new_game_state = game_state.clone();
		if new_game_state.make_move(&Location::from_index(i)) == MoveResult::Invalid {
			return None;
		}
		Some(new_game_state)
	}).collect::<Vec<_>>();

	debug_overlay("eval", DebugOverlaySettings {
		max:  1000.0,
		precision: 0,
	}, new_game_states.iter().map(|new_game_states| {
		new_game_states.as_ref().map(|new_game_state| new_game_state.evaluate(10))
	}).collect());

	debug_overlay("alignments", DebugOverlaySettings {
		max:  1000.0,
		precision: 0,
	}, new_game_states.iter().map(|new_game_states| {
		new_game_states.as_ref().map(|new_game_state| {
			(new_game_state.alignments_score - game_state.alignments_score) as f32
		})
	}).collect());

	debug_overlay("captures", DebugOverlaySettings {
		max:  5.0,
		precision: 0,
	}, new_game_states.iter().map(|new_game_states| {
		new_game_states.as_ref().map(|new_game_state| {
			(f32::from(new_game_state.get_player_captures_count(Player::Player1))
				- f32::from(new_game_state.get_player_captures_count(Player::Player2)))
			- (f32::from(game_state.get_player_captures_count(Player::Player1))
				- f32::from(game_state.get_player_captures_count(Player::Player2)))
		})
	}).collect());

	debug_overlay("play mask", DebugOverlaySettings {
		max:  1.0,
		precision: 0,
	}, (0..CELL_COUNT).map(|i| {
		let location = Location::from_index(i);
		if game_state.is_in_play_mask(location.x, location.y) { Some(1.0) } else { None }
	}).collect());

	debug_overlay("free three when play overlay", DebugOverlaySettings {
		max:  1.0,
		precision: 0,
	}, (0..CELL_COUNT).map(|i| {
		let location = Location::from_index(i);
		let free_three: u32 = game_state.free_three_mask[location.y as usize] >> location.x & 1;
		if 0 < free_three { Some(free_three as f32) } else { None }
	}).collect());

	use_lookup_table::<LOOKUP_TABLE_USE_ORDERING>(&mut game_state);
	debug_overlay("ordering v2", DebugOverlaySettings {
		max: 10_000.0,
		precision: 0
	}, game_state.moves_scores.iter().map(|score| Some(*score as f32)).collect());
}



#[wasm_bindgen]
impl AIResult {
	#[must_use]
	#[wasm_bindgen(js_name = updateOverlays)]
	pub fn update_overlays(&self, overlay_moves: &[PackedLocation]) -> bool {
		let Some(debug_data) = &self.debug_data else  {
			return false;
		};

		let mut game_state = debug_data.game_state.clone();

		for location in overlay_moves {
			if game_state.make_move(&Location::from_index(*location as usize)) == MoveResult::Invalid {
				debug_log!("invalid move {:?}", location);
				return false;
			}
		}

		debug_overlay("root moves ordering", DebugOverlaySettings {
			max: (BOARD_SIZE*BOARD_SIZE) as f32,
			precision: 0,
		}, (0..CELL_COUNT).map(|i| {
			let location = Location::from_index(i);
			debug_data.moves_list.iter().position(|l| *l == location)
				.map(|pos| pos as f32)
		}).collect());

		debug_overlay("root moves score", DebugOverlaySettings {
			max:  1000.0,
			precision: 0,
		}, debug_data.root_move_score.to_vec());

		true
	}
}
