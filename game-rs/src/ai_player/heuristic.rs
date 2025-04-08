use crate::*;

/*
https://link.springer.com/chapter/10.1007/978-0-387-35706-5_16
https://peizhiyan.github.io/docs/paper_informal.pdf
https://gbersac.github.io/gomoku_42/src/gomoku/ia/heuristic.rs.html#1-157
https://arxiv.org/pdf/2111.01016

Heuristic:
- Neighborhood ?
- Alignments ? (ally + enemy)
- Captures ? (ally + enemy)
- Patterns ?
*/

pub const WIN_SCORE: f32 = 10_000_000_000_000.0;

impl PackedGameState {
	/// Evaluate the current game state.
	/// + Player 1 wins
	/// - Player 2 wins
	#[must_use]
	pub fn evaluate(&self, depth: usize) -> f32 {
		// Higher depth means faster win.
		// So, we multiply the winning score by the depth.
		// We add 1000.0 for float error margin
		match self.winner {
			#[allow(clippy::cast_precision_loss)]
			Player::Player1 => return (WIN_SCORE * (depth+1) as f32 + 1000.0),
			#[allow(clippy::cast_precision_loss)]
			Player::Player2 => return -(WIN_SCORE * (depth+1) as f32 + 1000.0),
			_ => (),
		};

		let captures_score = f32::from(self.get_player_captures_count(Player::Player1))
		                        - f32::from(self.get_player_captures_count(Player::Player2));

		(captures_score * 750_000.0)
			// TODO limit ?
			+ self.alignments_score as f32

			+ ((self.alignments_score_minmax.min as f32)
				+ (self.alignments_score_minmax.max as f32))
	}
}
