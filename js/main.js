import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play } from './rules.js';

const canvas = document.getElementById('board');
let game = newGame();

function draw() {
  drawBoard(game);
}

canvas.addEventListener('click', (event) => {
  const index = cellAt(event);
  if (index === -1) return;
  if (play(game, index)) draw();
});

window.addEventListener('resize', () => {
  resizeBoard();
  draw();
});

resizeBoard();
draw();
