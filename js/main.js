import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play } from './rules.js';

const canvas = document.getElementById('board');
let game = newGame();
let hover = -1;

function draw() {
  drawBoard(game, hover);
}

canvas.addEventListener('click', (event) => {
  const index = cellAt(event);
  if (index === -1) return;
  if (play(game, index)) draw();
});

canvas.addEventListener('mousemove', (event) => {
  const index = cellAt(event);
  if (index !== hover) {
    hover = index;
    draw();
  }
});

canvas.addEventListener('mouseleave', () => {
  hover = -1;
  draw();
});

window.addEventListener('resize', () => {
  resizeBoard();
  draw();
});

resizeBoard();
draw();
