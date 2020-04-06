// read from stdin
// Good commands to pipe in:
//   ps -co cmd | tail -n +2
//   find . -type d \( -path ./target -o -path ./.git \) -prune -o -printf "%f\n" | tail -n +2

extern crate matrixise;
use matrixise::*;

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_WHITE: i16 = 1;

use std::io::{self, BufRead};

fn main() {
    matrixise::init(); // init must be called prior to defining ncurses colors
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);

    // creating a screne with [max_padding = 20, black background, reuse messages]:
    let mut screen = Scene::new(20, COLOR_BLACK, true);
    screen.start(); // forks into a new thread
    
    let stdin = io::stdin();
    while screen.alive() {
	for line in stdin.lock().lines() { // lock blocks q, so this it Control-C to exit
            let line = line.expect("Could not read line from standard in");
            screen.push(Message::new_simple(&line, COLOR_PAIR(COLOR_PAIR_WHITE), 0));
	}
    }

    screen.kill();
}
