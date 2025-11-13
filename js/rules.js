// the game itself, no html in here

export const SIZE = 19;
export const EMPTY = 0;
export const BLACK = 1;
export const WHITE = 2;

export function newGame() {
  return {
    board: new Array(SIZE * SIZE).fill(EMPTY),
    player: BLACK, // whose turn it is
  };
}

export function opponent(player) {
  return player === BLACK ? WHITE : BLACK;
}

export function play(game, index) {
  if (game.board[index] !== EMPTY) return false;

  game.board[index] = game.player;
  game.player = opponent(game.player);
  return true;
}
