# Pente

[![CI](https://github.com/walidmoovin/Gomoku/actions/workflows/ci.yml/badge.svg)](https://github.com/walidmoovin/Gomoku/actions/workflows/ci.yml)

A variant of the game Pente. [Demo](https://gomoku.mathias.ninja/)

## Tech Stack

- Rust -> Wasm
- Svelte
- Icons: <https://icon-sets.iconify.design/majesticons/>

## Install

- rust: `curl https://sh.rustup.rs -sSf | sh`
- cargo tools: `cargo install cargo-watch cargo-pgo wasm-pack cargo-asm`

## Bonus

- Web + Pretty UI
- Debug panel
- Sound
- Unit test
- CI/CD
- Opening handicaps:
    -Pro: 1st player has to place his second stone at least two rows and columns away from the center of the board
    -LongPro: Pro but at least four rows and columns
    -Swap: 1st player puts 3 stones on the board (2 black + 1 white), 2nd player choose color to play
    -Swap2: 2nd player also can choose if he wants to put two more stones (1 black + 1 white) and let first player decide if swap colors or not
<!-- - Multiple AI -->

Won  43.4% (981) Lost  56.6% (1277) Draw   0.0% (0)     Forfeit  23.6% (533)
Won  49.7% (3352) Lost  50.3% (3390) Draw   0.0% (0)     Forfeit  21.5% (1450)


## Wons on latest

`[180,160,143,183,164,144,145,126,162,198,162,181,182,179,141,217,236,199,163,219,239,218,220,123,201,163,258]`
`[180,160,140,122,141,142,124,162,178,142,102,139,140,182,202,121,120,84,123,100,121,160,122,120,125,82,122]`

## TODO

- leaderboard
- save matches
