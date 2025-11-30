import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play, undo, BLACK } from './rules.js';

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
  else if (game.forced) status.textContent = `${colorName(game.player)} has to break the five!`;
  else status.textContent = `${colorName(game.player)} to play`;

  for (const player of [1, 2]) {
    const card = document.getElementById(`player-${player}`);
    card.classList.toggle('active', !game.winner && game.player === player);
    card.querySelector('.captures').textContent = `${game.captures[player]}/5 captures`;
  }
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

document.getElementById('undo').addEventListener('click', () => {
  undo(game);
  draw();
});

document.getElementById('new-game').addEventListener('click', () => {
  game = newGame();
  draw();
});

window.addEventListener('resize', () => {
  resizeBoard();
  draw();
});

resizeBoard();
draw();
