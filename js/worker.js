// runs the computer in the background, so the page doesn't freeze while it thinks

import { bestMove } from './ai.js';

onmessage = (event) => {
  const start = performance.now();
  const result = bestMove(event.data);
  postMessage({ ...result, time: performance.now() - start });
};
