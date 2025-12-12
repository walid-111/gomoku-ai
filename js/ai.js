// the computer player. for now it just plays somewhere next to a stone

import { SIZE, EMPTY, isLegal } from './rules.js';

function cell(board, x, y) {
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return -1;
  return board[y * SIZE + x];
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

export function bestMove(game) {
  const moves = candidates(game);
  if (moves.length === 0) return { move: Math.floor(game.board.length / 2) };
  return { move: moves[Math.floor(Math.random() * moves.length)] };
}
