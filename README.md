# nand2tetris

## Setup

```sh
git clone git@github.com:cola119/nand2tetris-rs.git
cd nand2tetris-rs
# prepare the display
cd display
nvm use
npm i
cd ..
```

## How to boot this computer

```sh
# open display
cd display
open index.html
cd ..
# boot the computer
cargo run -p integrate
# And then need to reload index.html to make WebSocket connection between the display and the computer
```

## Testing

```sh
nand2tetris-rs: RUST_MIN_STACK=8388608 cargo test -- --nocapture
```
