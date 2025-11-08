// everything about drawing the board on the canvas

export const SIZE = 19;

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

export function drawBoard() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  ctx.strokeStyle = '#000';
  ctx.lineWidth = 1;
  for (let i = 0; i < SIZE; i++) {
    ctx.beginPath();
    ctx.moveTo(pos(0), pos(i));
    ctx.lineTo(pos(SIZE - 1), pos(i));
    ctx.moveTo(pos(i), pos(0));
    ctx.lineTo(pos(i), pos(SIZE - 1));
    ctx.stroke();
  }
}
