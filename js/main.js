import { resizeBoard, drawBoard } from './board.js';

function draw() {
  resizeBoard();
  drawBoard();
}

window.addEventListener('resize', draw);
draw();
