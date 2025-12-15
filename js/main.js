import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play, undo, BLACK, WHITE, SIZE } from './rules.js';
import { playSound, isMuted, toggleMute } from './sounds.js';

const canvas = document.getElementById('board');
const status = document.getElementById('status');
const movesList = document.getElementById('moves');
let game = newGame();
let hover = -1;
const computer = WHITE; // the computer plays white for now
let thinking = false;

const worker = new Worker('js/worker.js', { type: 'module' });

function colorName(player) {
  return player === BLACK ? 'Black' : 'White';
}

// like on a real board: letter for the column, number for the row (1 at the bottom)
function moveName(index) {
  return 'ABCDEFGHIJKLMNOPQRS'[index % SIZE] + (SIZE - Math.floor(index / SIZE));
}

function draw() {
  drawBoard(game, hover);
  if (game.winner) status.textContent = `${colorName(game.winner)} wins with ${game.winReason}!`;
  else if (game.forced) status.textContent = `${colorName(game.player)} has to break the five!`;
  else if (thinking) status.textContent = 'Computer is thinking...';
  else status.textContent = `${colorName(game.player)} to play`;

  for (const player of [1, 2]) {
    const card = document.getElementById(`player-${player}`);
    card.classList.toggle('active', !game.winner && game.player === player);
    card.querySelector('.captures').textContent = `${game.captures[player]}/5 captures`;
  }

  movesList.innerHTML = '';
  for (const move of game.moves) {
    const li = document.createElement('li');
    li.textContent = moveName(move.index) + (move.captured.length ? ' x' + move.captured.length / 2 : '');
    movesList.appendChild(li);
  }
  movesList.scrollTop = movesList.scrollHeight;

  document.getElementById('sound').textContent = isMuted() ? 'Sound off' : 'Sound on';
}

function makeMove(index) {
  if (!play(game, index)) return false;
  playSound(game.winner ? 'win' : 'place');
  draw();
  return true;
}

function computerTurn() {
  if (game.winner || game.player !== computer) return;
  thinking = true;
  draw();
  worker.postMessage(game);
}

worker.onmessage = (event) => {
  thinking = false;
  makeMove(event.data.move);
};

canvas.addEventListener('click', (event) => {
  const index = cellAt(event);
  if (index === -1 || game.player === computer) return;
  if (makeMove(index)) computerTurn();
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

document.getElementById('sound').addEventListener('click', () => {
  toggleMute();
  draw();
});

window.addEventListener('resize', () => {
  resizeBoard();
  draw();
});

resizeBoard();
draw();
