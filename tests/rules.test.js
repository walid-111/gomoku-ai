import test from 'node:test';
import assert from 'node:assert';
import { newGame, play, undo, SIZE, BLACK, WHITE, EMPTY } from '../js/rules.js';

const at = (x, y) => y * SIZE + x;

// plays a list of [x, y], black starts
function playAll(game, moves) {
  for (const [x, y] of moves) assert.ok(play(game, at(x, y)), `move ${x},${y} refused`);
}

test('5 in a row wins', () => {
  const game = newGame();
  playAll(game, [[0, 0], [0, 5], [1, 0], [1, 5], [2, 0], [2, 5], [3, 0], [3, 5], [4, 0]]);
  assert.strictEqual(game.winner, BLACK);
  assert.strictEqual(game.winLine.length, 5);
});

test('4 in a row is not a win', () => {
  const game = newGame();
  playAll(game, [[0, 0], [0, 5], [1, 0], [1, 5], [2, 0], [2, 5], [3, 0]]);
  assert.strictEqual(game.winner, EMPTY);
});

test('capture removes the pair', () => {
  const game = newGame();
  // black 9,9 / white 10,9 / black 5,5 / white 11,9 / black 12,9 takes them
  playAll(game, [[9, 9], [10, 9], [5, 5], [11, 9], [12, 9]]);
  assert.strictEqual(game.board[at(10, 9)], EMPTY);
  assert.strictEqual(game.board[at(11, 9)], EMPTY);
  assert.strictEqual(game.captures[BLACK], 1);
});

test('playing in between two stones is not a capture', () => {
  const game = newGame();
  // black 9,9 and 12,9, white plays 10,9 then 11,9 itself
  playAll(game, [[9, 9], [10, 9], [12, 9], [11, 9]]);
  assert.strictEqual(game.board[at(10, 9)], WHITE);
  assert.strictEqual(game.board[at(11, 9)], WHITE);
  assert.strictEqual(game.captures[BLACK], 0);
});

test('double free three is not allowed', () => {
  const game = newGame();
  // black: 9,9 10,9 (horizontal) and 11,10 11,11 (vertical), white plays far away
  playAll(game, [[9, 9], [0, 0], [10, 9], [0, 2], [11, 10], [0, 4], [11, 11], [0, 6]]);
  // 11,9 would make _XXX_ horizontally and vertically
  assert.strictEqual(play(game, at(11, 9)), false);
  assert.strictEqual(game.board[at(11, 9)], EMPTY);
});

test('a single free three is fine', () => {
  const game = newGame();
  playAll(game, [[9, 9], [0, 0], [10, 9], [0, 2]]);
  assert.strictEqual(play(game, at(11, 9)), true);
});

test('double free three is allowed when it captures', () => {
  const game = newGame();
  // same shape as before, but white has a pair on the diagonal that 11,9 takes
  playAll(game, [[9, 9], [12, 8], [10, 9], [13, 7], [11, 10], [0, 0], [11, 11], [0, 2], [14, 6], [0, 4]]);
  assert.strictEqual(play(game, at(11, 9)), true);
  assert.strictEqual(game.captures[BLACK], 1);
});

test('a five that can be broken is not a win yet, and the other player has to break it', () => {
  const game = newGame();
  // black makes 5 on row 9 (5,9 to 9,9). black also has 7,10 under 7,9 and white has 7,11,
  // so white can capture 7,9 + 7,10 by playing 7,8
  playAll(game, [[5, 9], [7, 11], [6, 9], [0, 0], [7, 10], [0, 2], [8, 9], [0, 4], [7, 9], [0, 6], [9, 9]]);
  assert.strictEqual(game.winner, EMPTY);
  assert.deepStrictEqual(game.forced, [at(7, 8)]);

  // white can only play the breaking move
  assert.strictEqual(play(game, at(15, 15)), false);
  assert.strictEqual(play(game, at(7, 8)), true);
  assert.strictEqual(game.board[at(7, 9)], EMPTY);
  assert.strictEqual(game.forced, null);
});

test('a five that cannot be broken wins', () => {
  const game = newGame();
  playAll(game, [[5, 9], [0, 0], [6, 9], [0, 2], [7, 9], [0, 4], [8, 9], [0, 6], [9, 9]]);
  assert.strictEqual(game.winner, BLACK);
});

test('undo puts everything back, captured stones too', () => {
  const game = newGame();
  playAll(game, [[9, 9], [10, 9], [5, 5], [11, 9]]);
  const before = JSON.stringify(game);
  play(game, at(12, 9)); // captures
  undo(game);
  assert.strictEqual(JSON.stringify(game), before);
});

test('pro opening: first stone in the middle, black second stone 3+ cells away', () => {
  const game = newGame('pro');
  assert.strictEqual(play(game, at(3, 3)), false);
  assert.strictEqual(play(game, at(9, 9)), true);
  assert.strictEqual(play(game, at(10, 10)), true); // white can play anywhere
  assert.strictEqual(play(game, at(10, 9)), false); // too close
  assert.strictEqual(play(game, at(11, 10)), true); // 2 + 1 = 3 away
});

test('long pro opening needs 4 cells', () => {
  const game = newGame('long-pro');
  playAll(game, [[9, 9], [10, 10]]);
  assert.strictEqual(play(game, at(11, 10)), false);
  assert.strictEqual(play(game, at(13, 9)), true);
});

test('full board is a draw', () => {
  const game = newGame();
  // fill everything except the last cell: runs of 3 on each row, so no five and no capture
  for (let i = 0; i < SIZE * SIZE - 1; i++) {
    const x = i % SIZE;
    const y = Math.floor(i / SIZE);
    game.board[i] = (x + 3 * y) % 6 < 3 ? BLACK : WHITE;
  }
  game.player = BLACK;
  assert.strictEqual(play(game, SIZE * SIZE - 1), true);
  assert.strictEqual(game.winner, EMPTY);
  assert.strictEqual(game.winReason, 'draw');
  assert.strictEqual(play(game, 0), false);
});

test('capturing the end of a six is not a break, there is still a five', () => {
  const game = newGame();
  // black makes 4,9 to 9,9 (six) with the last move 8,9.
  // white could take 4,9 + 4,10 by playing 4,8, but 5,9 to 9,9 would still be five
  playAll(game, [[4, 10], [4, 11], [4, 9], [0, 0], [5, 9], [0, 2], [6, 9], [0, 4], [7, 9], [0, 6], [9, 9], [0, 8], [8, 9]]);
  assert.strictEqual(game.winner, BLACK);
});
