import test from 'node:test';
import assert from 'node:assert';
import { newGame, play, SIZE, BLACK, WHITE, EMPTY } from '../js/rules.js';

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
