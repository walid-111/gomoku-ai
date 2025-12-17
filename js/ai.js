// the computer player: minimax with alpha-beta, it looks a few moves ahead
// and keeps going deeper until the time is up

import { SIZE, EMPTY, play, undo, isLegal, opponent, findCaptures } from './rules.js';

const WIN = 1000000;
const DIRECTIONS = [[1, 0], [0, 1], [1, 1], [1, -1]];

// score of a group of 5 cells, by how many of our stones are in it (if no enemy stone)
const WINDOW_SCORES = [0, 1, 10, 100, 1000, 100000];
// captures get scary fast, 5 is a win
const CAPTURE_SCORES = [0, 300, 1000, 3000, 10000, WIN];

let deadline = 0;
const TIMEOUT = 'timeout';

function cell(board, x, y) {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return -1;
  return board[y * SIZE + x];
}

// how good the board is for player (negative = good for the other one).
// we look at every group of 5 cells in a row: if only one player has stones in it,
// it's a possible five for him, and more stones = better
export function evaluate(game, player) {
  const board = game.board;
  let score = CAPTURE_SCORES[game.captures[player]] - CAPTURE_SCORES[game.captures[opponent(player)]];

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

// quick guess of how interesting a move is, to try the good ones first
function moveScore(game, index) {
  const board = game.board;
  const x = index % SIZE;
  const y = Math.floor(index / SIZE);
  let score = findCaptures(board, index, game.player).length * 50;

  for (const [dx, dy] of DIRECTIONS) {
    // stones touching this cell on both sides, for both players
    for (const player of [1, 2]) {
      let count = 0;
      for (let k = 1; cell(board, x + k * dx, y + k * dy) === player; k++) count++;
      for (let k = 1; cell(board, x - k * dx, y - k * dy) === player; k++) count++;
      score += count * count * 10;
    }
  }
  return score;
}

// empty cells next to a stone (2 cells max), best looking ones first
function candidates(game, limit) {
  if (game.forced) return game.forced.slice();

  const board = game.board;
  const moves = [];
  for (let i = 0; i < board.length; i++) {
    if (board[i] !== EMPTY) continue;
    const x = i % SIZE;
    const y = Math.floor(i / SIZE);

    let near = false;
    for (let dy = -2; dy <= 2 && !near; dy++) {
      for (let dx = -2; dx <= 2 && !near; dx++) {
        const c = cell(board, x + dx, y + dy);
        if (c !== EMPTY && c !== -1) near = true;
      }
    }
    if (near && isLegal(game, i)) moves.push(i);
  }

  // empty board: middle
  if (moves.length === 0 && board[Math.floor(board.length / 2)] === EMPTY) {
    return [Math.floor(board.length / 2)];
  }

  return moves
    .map((move) => ({ move, score: moveScore(game, move) }))
    .sort((a, b) => b.score - a.score)
    .slice(0, limit)
    .map((m) => m.move);
}

// negamax: the score is always for the player who has to play
function search(game, depth, alpha, beta) {
  // the player before just won
  if (game.winner) return -WIN - depth;
  if (depth === 0) return evaluate(game, game.player);
  if (Date.now() > deadline) throw TIMEOUT;

  const moves = candidates(game, 10);
  if (moves.length === 0) return 0;

  let best = -Infinity;
  for (const move of moves) {
    play(game, move);
    const score = -search(game, depth - 1, -beta, -alpha);
    undo(game);

    if (score > best) best = score;
    if (best > alpha) alpha = best;
    if (alpha >= beta) break; // the other player won't let this happen anyway
  }
  return best;
}

// returns { move, depth } for the player whose turn it is
export function bestMove(game, timeLimit = 500) {
  deadline = Date.now() + timeLimit;
  let moves = candidates(game, 15);
  if (moves.length <= 1) return { move: moves[0], depth: 0 };

  let best = moves[0];
  let bestDepth = 0;

  for (let depth = 1; depth <= 10; depth++) {
    // work on a copy, if the time runs out in the middle the board is a mess
    const copy = structuredClone(game);
    let alpha = -Infinity;
    let bestHere = moves[0];

    try {
      for (const move of moves) {
        play(copy, move);
        const score = -search(copy, depth - 1, -Infinity, -alpha);
        undo(copy);
        if (score > alpha) {
          alpha = score;
          bestHere = move;
        }
      }
    } catch (error) {
      if (error !== TIMEOUT) throw error;
      break; // keep the result of the last depth we finished
    }

    best = bestHere;
    bestDepth = depth;
    // best move first next time, alpha-beta cuts a lot more like that
    moves = [best, ...moves.filter((m) => m !== best)];
    if (alpha >= WIN) break; // found a win, no need to look further
  }

  return { move: best, depth: bestDepth };
}
