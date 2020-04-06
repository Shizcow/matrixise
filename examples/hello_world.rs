// A simple Hello World program

extern crate matrixise;
use matrixise::*;

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_WHITE: i16 = 1;

fn main() {
    matrixise::init(); // init must be called prior to defining ncurses colors
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);

    // creating a screne with [max_padding = 20, black background, reuse messages]:
    let mut screen = Scene::new(20, COLOR_BLACK, true);
    screen.start(); // forks into a new thread
    
    // Here's one way to add messages:
    screen.push(Message::new_simple("Hello world", COLOR_PAIR(COLOR_PAIR_WHITE), 0));
    
    screen.join(); // wait for screen to die (user presses q)
}
