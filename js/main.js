import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play, BLACK } from './rules.js';

const canvas = document.getElementById('board');
const status = document.getElementById('status');
let game = newGame();
let hover = -1;

function colorName(player) {
  return player === BLACK ? 'Black' : 'White';
}

function draw() {
  drawBoard(game, hover);
  if (game.winner) status.textContent = `${colorName(game.winner)} wins with ${game.winReason}!`;
  else status.textContent = `${colorName(game.player)} to play`;
  status.textContent += ` (captures: black ${game.captures[1]}, white ${game.captures[2]})`;
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
