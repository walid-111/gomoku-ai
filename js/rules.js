// the game itself, no html in here

export const SIZE = 19;
export const EMPTY = 0;
export const BLACK = 1;
export const WHITE = 2;

// horizontal, vertical and the 2 diagonals
const DIRECTIONS = [[1, 0], [0, 1], [1, 1], [1, -1]];

export function newGame() {
  return {
    board: new Array(SIZE * SIZE).fill(EMPTY),
    player: BLACK, // whose turn it is
    winner: EMPTY,
    winLine: [],
  };
}

export function opponent(player) {
  return player === BLACK ? WHITE : BLACK;
}

function inside(x, y) {
  return x >= 0 && y >= 0 && x < SIZE && y < SIZE;
}

// -1 when outside the board
function stoneAt(board, x, y) {
  return inside(x, y) ? board[y * SIZE + x] : -1;
}

// all the stones of the same color touching index in one direction (index included)
function lineThrough(board, index, dx, dy) {
  const player = board[index];
  const x = index % SIZE;
  const y = Math.floor(index / SIZE);
  const line = [index];

  for (let k = 1; stoneAt(board, x + dx * k, y + dy * k) === player; k++) {
    line.push((y + dy * k) * SIZE + x + dx * k);
  }
  for (let k = 1; stoneAt(board, x - dx * k, y - dy * k) === player; k++) {
    line.unshift((y - dy * k) * SIZE + x - dx * k);
  }
  return line;
}

// returns the line if the stone at index makes 5 or more in a row
export function findFive(board, index) {
  for (const [dx, dy] of DIRECTIONS) {
    const line = lineThrough(board, index, dx, dy);
    if (line.length >= 5) return line;
  }
  return null;
}

export function play(game, index) {
  if (game.winner || game.board[index] !== EMPTY) return false;

  game.board[index] = game.player;

  const five = findFive(game.board, index);
  if (five) {
    game.winner = game.player;
    game.winLine = five;
  }

  game.player = opponent(game.player);
  return true;
}
