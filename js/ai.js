// the computer player: minimax, it looks a few moves ahead

import { SIZE, EMPTY, play, undo, isLegal, opponent } from './rules.js';

const WIN = 1000000;
const DIRECTIONS = [[1, 0], [0, 1], [1, 1], [1, -1]];

// score of a group of 5 cells, by how many of our stones are in it (if no enemy stone)
const WINDOW_SCORES = [0, 1, 10, 100, 1000, 100000];

function cell(board, x, y) {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return -1;
  return board[y * SIZE + x];
}

// how good the board is for player (negative = good for the other one).
// we look at every group of 5 cells in a row: if only one player has stones in it,
// it's a possible five for him, and more stones = better
export function evaluate(game, player) {
  const board = game.board;
  let score = 0;

  for (let y = 0; y < SIZE; y++) {
    for (let x = 0; x < SIZE; x++) {
      for (const [dx, dy] of DIRECTIONS) {
        // the 5 cells have to fit on the board
        if (cell(board, x + 4 * dx, y + 4 * dy) === -1) continue;

        let mine = 0;
        let theirs = 0;
        for (let k = 0; k < 5; k++) {
          const c = board[(y + k * dy) * SIZE + x + k * dx];
          if (c === player) mine++;
          else if (c !== EMPTY) theirs++;
        }
        if (theirs === 0) score += WINDOW_SCORES[mine];
        else if (mine === 0) score -= WINDOW_SCORES[theirs];
      }
    }
  }
  return score;
}

// empty cells right next to a stone
function candidates(game) {
  const board = game.board;
  const moves = [];
  for (let i = 0; i < board.length; i++) {
    if (board[i] !== EMPTY) continue;
    const x = i % SIZE;
    const y = Math.floor(i / SIZE);

    let near = false;
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        const c = cell(board, x + dx, y + dy);
        if (c !== EMPTY && c !== -1) near = true;
      }
    }
    if (near && isLegal(game, i)) moves.push(i);
  }
  return moves;
}

// negamax: the score is always for the player who has to play
function search(game, depth) {
  // the player before just won
  if (game.winner) return -WIN - depth;
  if (depth === 0) return evaluate(game, game.player);

  const moves = candidates(game);
  if (moves.length === 0) return 0;

  let best = -Infinity;
  for (const move of moves) {
    play(game, move);
    const score = -search(game, depth - 1);
    undo(game);
    if (score > best) best = score;
  }
  return best;
}

const DEPTH = 2;

export function bestMove(game) {
  const moves = candidates(game);
  if (moves.length === 0) return { move: Math.floor(game.board.length / 2) };

  let best = moves[0];
  let bestScore = -Infinity;
  for (const move of moves) {
    play(game, move);
    const score = -search(game, DEPTH - 1);
    undo(game);
    if (score > bestScore) {
      bestScore = score;
      best = move;
    }
  }
  return { move: best };
}
