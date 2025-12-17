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
    moves: [], // everything played, so we can undo
    forced: null, // when there's a five that can be broken, the only moves allowed
    winner: EMPTY,
    winReason: '',
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

// X O O X -> a stone of player at index would take the 2 stones in between
// returns the cells that would be captured (doesn't touch the board)
export function findCaptures(board, index, player) {
  const other = opponent(player);
  const x = index % SIZE;
  const y = Math.floor(index / SIZE);
  const captured = [];

  for (const [dx, dy] of ALL_DIRECTIONS) {
    if (stoneAt(board, x + dx, y + dy) === other
      && stoneAt(board, x + 2 * dx, y + 2 * dy) === other
      && stoneAt(board, x + 3 * dx, y + 3 * dy) === player) {
      captured.push((y + dy) * SIZE + x + dx, (y + 2 * dy) * SIZE + x + 2 * dx);
    }
  }
  return captured;
}

// free three = 3 stones that become an open four if nobody blocks them.
// we look at 6 cells in a row: both ends empty, and in the 4 cells in the middle
// 3 of our stones + 1 empty (_XXX__, __XXX_, _XX_X_, _X_XX_)
function isFreeThree(board, index, dx, dy) {
  const me = board[index];
  const x = index % SIZE;
  const y = Math.floor(index / SIZE);

  // our stone has to be in the middle part, so the 6 cells start 1 to 4 cells before it
  for (let start = -4; start <= -1; start++) {
    const cells = [];
    for (let k = start; k < start + 6; k++) cells.push(stoneAt(board, x + k * dx, y + k * dy));

    const middle = cells.slice(1, 5);
    if (cells[0] === EMPTY && cells[5] === EMPTY
      && middle.filter((c) => c === me).length === 3
      && middle.filter((c) => c === EMPTY).length === 1) {
      return true;
    }
  }
  return false;
}

// you can't make 2 free threes with one move, except if the move captures something
function isDoubleThree(board, index, player) {
  if (findCaptures(board, index, player).length > 0) return false;

  board[index] = player; // try the move for a sec
  let count = 0;
  for (const [dx, dy] of DIRECTIONS) {
    if (isFreeThree(board, index, dx, dy)) count++;
  }
  board[index] = EMPTY;
  return count >= 2;
}

export function isLegal(game, index) {
  if (game.winner || game.board[index] !== EMPTY) return false;
  if (game.forced) return game.forced.includes(index);
  return !isDoubleThree(game.board, index, game.player);
}

// moves that let player capture a pair out of the five, so it's not 5 in a row anymore
function movesThatBreak(board, line, player) {
  const moves = [];
  for (let i = 0; i < board.length; i++) {
    if (board[i] !== EMPTY) continue;
    const captured = findCaptures(board, i, player);
    if (captured.length === 0) continue;

    // try it on a copy and check there's no five left
    const copy = board.slice();
    for (const c of captured) copy[c] = EMPTY;
    copy[i] = player;
    if (!line.some((c) => copy[c] !== EMPTY && findFive(copy, c))) moves.push(i);
  }
  return moves;
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
  if (!isLegal(game, index)) return false;

  const captured = findCaptures(game.board, index, game.player);
  game.moves.push({ index, player: game.player, captured, forced: game.forced, winLine: game.winLine });
  for (const i of captured) game.board[i] = EMPTY;
  game.board[index] = game.player;
  game.captures[game.player] += captured.length / 2;
  game.forced = null;
  game.winLine = [];

  const five = findFive(game.board, index);
  if (game.captures[game.player] >= 5) {
    game.winner = game.player;
    game.winReason = '5 captures';
  } else if (five) {
    game.winLine = five;
    // the other player can still save himself by capturing in the five
    const breaks = movesThatBreak(game.board, five, opponent(game.player));
    if (breaks.length > 0) {
      game.forced = breaks;
    } else {
      game.winner = game.player;
      game.winReason = '5 in a row';
    }
  }

  game.player = opponent(game.player);
  return true;
}

export function undo(game) {
  const last = game.moves.pop();
  if (!last) return;

  game.board[last.index] = EMPTY;
  for (const i of last.captured) game.board[i] = opponent(last.player);
  game.captures[last.player] -= last.captured.length / 2;
  game.player = last.player;
  game.forced = last.forced;
  game.winLine = last.winLine;
  game.winner = EMPTY;
  game.winReason = '';
}
