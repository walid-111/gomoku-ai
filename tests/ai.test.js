import test from 'node:test';
import assert from 'node:assert';
import { newGame, play, SIZE, BLACK } from '../js/rules.js';
import { bestMove } from '../js/ai.js';

const at = (x, y) => y * SIZE + x;

function playAll(game, moves) {
  for (const [x, y] of moves) assert.ok(play(game, at(x, y)), `move ${x},${y} refused`);
}

test('first move goes in the middle', () => {
  assert.strictEqual(bestMove(newGame()).move, at(9, 9));
});

test('takes the win when it has 4 in a row', () => {
  const game = newGame();
  playAll(game, [[5, 9], [5, 12], [6, 9], [7, 13], [7, 9], [15, 15], [8, 9], [15, 3]]);
  const { move } = bestMove(game);
  play(game, move);
  assert.strictEqual(game.winner, BLACK);
});

test('blocks a four', () => {
  const game = newGame();
  // white has 4 on row 9 blocked on the left, black has to block 9,9
  playAll(game, [[4, 9], [5, 9], [3, 3], [6, 9], [15, 15], [7, 9], [3, 15], [8, 9]]);
  assert.strictEqual(bestMove(game).move, at(9, 9));
});

test('answers in about half a second', () => {
  const game = newGame();
  playAll(game, [[9, 9], [10, 10], [10, 9], [8, 9], [9, 10], [9, 8]]);
  const start = Date.now();
  bestMove(game, 500);
  assert.ok(Date.now() - start < 700);
});
