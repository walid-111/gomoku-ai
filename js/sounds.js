const sounds = {
  place: new Audio('sounds/placing.mp3'),
  win: new Audio('sounds/win.mp3'),
  loss: new Audio('sounds/loss.mp3'),
};

let muted = localStorage.getItem('muted') === 'true';

export function playSound(name) {
  if (muted) return;
  const sound = sounds[name];
  sound.currentTime = 0;
  sound.play().catch(() => {}); // browsers block sound before the first click, not a big deal
}

export function isMuted() {
  return muted;
}

export function toggleMute() {
  muted = !muted;
  localStorage.setItem('muted', muted);
}
