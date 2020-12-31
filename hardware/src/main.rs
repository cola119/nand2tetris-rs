mod base;
mod ws_server;

use base::logic::bit::{I, O};
use base::screen::Screen;
use base::{dff::Clock, logic::Word};

fn main() {
    let mut clock = Clock::new();
    let mut screen = Screen::new();
    let word1 = Word::new([I; 16]);

    screen.start_ws();
    screen.input(&clock, word1, I, [O, O, O, O, O, O, O, O, O, O, O, O, O]);
    clock.next();
    clock.next();
}
