import { resizeBoard, drawBoard, cellAt } from './board.js';
import { newGame, play, undo, opponent, BLACK, WHITE, EMPTY, SIZE } from './rules.js';
import { playSound, isMuted, toggleMute } from './sounds.js';

const canvas = document.getElementById('board');
const status = document.getElementById('status');
const movesList = document.getElementById('moves');
let game = newGame();
let hover = -1;
let computer = WHITE; // EMPTY when 2 people play
const menu = document.getElementById('menu');
let thinking = false;
let request = 0; // to know which answer from the computer is the latest one

const worker = new Worker('js/worker.js', { type: 'module' });

function colorName(player) {
  return player === BLACK ? 'Black' : 'White';
}

// "You" and "Computer" when playing against it, colors when 2 people play
function playerName(player) {
  if (computer === EMPTY) return colorName(player);
  return player === computer ? 'Computer' : 'You';
}

// like on a real board: letter for the column, number for the row (1 at the bottom)
function moveName(index) {
  return 'ABCDEFGHIJKLMNOPQRS'[index % SIZE] + (SIZE - Math.floor(index / SIZE));
}

function draw() {
  drawBoard(game, hover);
  const name = playerName(game.player);
  if (game.winner) status.textContent = `${playerName(game.winner)} won with ${game.winReason}!`;
  else if (thinking) status.textContent = 'Computer is thinking...';
  else if (game.forced) status.textContent = `${name} has to break the five!`;
  else status.textContent = name === 'You' ? 'Your turn' : `${name} to play`;

  // remind the opening rule when it matters
  if (game.opening !== 'standard' && game.moves.length === 0) status.textContent += ' (in the middle)';
  if (game.opening !== 'standard' && game.moves.length === 2) {
    status.textContent += ` (at least ${game.opening === 'pro' ? 3 : 4} cells from the middle)`;
  }

  for (const player of [1, 2]) {
    const card = document.getElementById(`player-${player}`);
    card.classList.toggle('active', !game.winner && game.player === player);
    card.querySelector('.captures').textContent = `${game.captures[player]}/5 captures`;
    card.querySelector('.name').textContent = playerName(player);
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
  request++;
  draw();
  worker.postMessage({ game, request });
}

worker.onmessage = (event) => {
  // undo or new game while it was thinking, the answer is for an old board
  if (!thinking || event.data.request !== request) return;
  const { move, time, depth } = event.data;
  thinking = false;
  document.getElementById('ai-time').textContent = `Computer took ${(time / 1000).toFixed(2)}s (${depth} moves ahead)`;
  makeMove(move);
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
  thinking = false;
  undo(game);
  // take back the computer's move too, so it's your turn again
  if (game.player === computer && game.moves.length > 0) undo(game);
  draw();
  computerTurn(); // if the computer started, it has to play again
});

document.getElementById('new-game').addEventListener('click', () => menu.showModal());

menu.addEventListener('close', () => {
  if (menu.returnValue !== 'start') return;
  const form = new FormData(menu.querySelector('form'));
  const you = Number(form.get('color'));
  computer = form.get('mode') === 'computer' ? opponent(you) : EMPTY;

  thinking = false;
  game = newGame(form.get('opening'));
  document.getElementById('ai-time').textContent = '';
  draw();
  computerTurn(); // the computer starts if you took white
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
menu.showModal();
