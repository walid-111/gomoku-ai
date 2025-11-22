// the game itself, no html in here

export const SIZE = 19;
export const EMPTY = 0;
export const BLACK = 1;
export const WHITE = 2;

// horizontal, vertical and the 2 diagonals
const DIRECTIONS = [[1, 0], [0, 1], [1, 1], [1, -1]];
// same but both ways, for captures
const ALL_DIRECTIONS = [...DIRECTIONS, ...DIRECTIONS.map(([dx, dy]) => [-dx, -dy])];

export function newGame() {
  return {
    board: new Array(SIZE * SIZE).fill(EMPTY),
    player: BLACK, // whose turn it is
    captures: [0, 0, 0], // pairs captured, by player (index 0 not used)
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

// X O O X -> the stone at index takes the 2 stones in between
// returns the captured cells
function capture(board, index) {
  const me = board[index];
  const other = opponent(me);
  const x = index % SIZE;
  const y = Math.floor(index / SIZE);
  const captured = [];

  for (const [dx, dy] of ALL_DIRECTIONS) {
    if (stoneAt(board, x + dx, y + dy) === other
      && stoneAt(board, x + 2 * dx, y + 2 * dy) === other
      && stoneAt(board, x + 3 * dx, y + 3 * dy) === me) {
      const a = (y + dy) * SIZE + x + dx;
      const b = (y + 2 * dy) * SIZE + x + 2 * dx;
      board[a] = EMPTY;
      board[b] = EMPTY;
      captured.push(a, b);
    }
  }
  return captured;
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
  game.captures[game.player] += capture(game.board, index).length / 2;

  const five = findFive(game.board, index);
  if (five) {
    game.winner = game.player;
    game.winLine = five;
  }

  game.player = opponent(game.player);
  return true;
}
