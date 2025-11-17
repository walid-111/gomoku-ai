// everything about drawing the board on the canvas

import { SIZE, BLACK, EMPTY } from './rules.js';

const canvas = document.getElementById('board');
const ctx = canvas.getContext('2d');

let cell = 0; // space between two lines, in pixels

export function resizeBoard() {
  const size = canvas.clientWidth;
  // sharper on retina screens
  canvas.width = size * devicePixelRatio;
  canvas.height = size * devicePixelRatio;
  ctx.setTransform(devicePixelRatio, 0, 0, devicePixelRatio, 0, 0);
  cell = size / (SIZE + 1);
}

// pixel position of a line (0 to 18)
function pos(i) {
  return cell * (i + 1);
}

// hover = cell under the mouse (or -1), drawn as a see-through stone
export function drawBoard(game, hover = -1) {
  ctx.fillStyle = '#e3b465';
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  ctx.strokeStyle = '#5c4320';
  ctx.lineWidth = 1;
  for (let i = 0; i < SIZE; i++) {
    ctx.beginPath();
    ctx.moveTo(pos(0), pos(i));
    ctx.lineTo(pos(SIZE - 1), pos(i));
    ctx.moveTo(pos(i), pos(0));
    ctx.lineTo(pos(i), pos(SIZE - 1));
    ctx.stroke();
  }

  // the 9 little dots
  ctx.fillStyle = '#5c4320';
  for (const x of [3, 9, 15]) {
    for (const y of [3, 9, 15]) {
      ctx.beginPath();
      ctx.arc(pos(x), pos(y), cell * 0.12, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  for (let i = 0; i < game.board.length; i++) {
    if (game.board[i] !== EMPTY) drawStone(i % SIZE, Math.floor(i / SIZE), game.board[i]);
  }

  // red dots on the winning stones
  for (const i of game.winLine) {
    ctx.beginPath();
    ctx.arc(pos(i % SIZE), pos(Math.floor(i / SIZE)), cell * 0.12, 0, Math.PI * 2);
    ctx.fillStyle = '#e53935';
    ctx.fill();
  }

  if (hover !== -1 && !game.winner && game.board[hover] === EMPTY) {
    drawStone(hover % SIZE, Math.floor(hover / SIZE), game.player, 0.5);
  }
}

function drawStone(x, y, player, alpha = 1) {
  const r = cell * 0.45;
  // little gradient so they look a bit round
  const light = ctx.createRadialGradient(pos(x) - r / 3, pos(y) - r / 3, r / 10, pos(x), pos(y), r);
  if (player === BLACK) {
    light.addColorStop(0, '#555');
    light.addColorStop(1, '#111');
  } else {
    light.addColorStop(0, '#fff');
    light.addColorStop(1, '#ccc');
  }
  ctx.globalAlpha = alpha;
  ctx.beginPath();
  ctx.arc(pos(x), pos(y), r, 0, Math.PI * 2);
  ctx.fillStyle = light;
  ctx.fill();
  ctx.globalAlpha = 1;
}

// which cell was clicked, or -1 if outside the board
export function cellAt(event) {
  const rect = canvas.getBoundingClientRect();
  const x = Math.round((event.clientX - rect.left) / cell - 1);
  const y = Math.round((event.clientY - rect.top) / cell - 1);
  if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) return -1;
  return y * SIZE + x;
}
