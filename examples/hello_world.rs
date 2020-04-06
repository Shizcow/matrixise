extern crate matrixise;
use matrixise::*;

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_WHITE     : i16 = 1;

use std::io::{self, BufRead};

fn main() {
    matrixise::init(); // init must be called prior to defininf colors
    init_pair(COLOR_PAIR_WHITE,     COLOR_WHITE, COLOR_BLACK);
    
    let mut screen = Scene::new();
    screen.start(20, COLOR_BLACK, true);
    screen.push(Message::new_simple("Hello ", "world", COLOR_PAIR(COLOR_PAIR_WHITE), 0));
    
    if let Err(_) = io::stdin().lock().read_line(&mut String::new()) { // wait for a keypress
	panic!("Could not take control of stdin");
    }
	
    screen.kill();
}
