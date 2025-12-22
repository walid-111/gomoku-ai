// runs the computer in the background, so the page doesn't freeze while it thinks

import { bestMove } from './ai.js';

onmessage = (event) => {
  const { game, request } = event.data;
  const start = performance.now();
  const result = bestMove(game);
  postMessage({ ...result, request, time: performance.now() - start });
};
