// How to use various color options

extern crate matrixise;
use matrixise::*;

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_BLACK: i16 = 1;
static COLOR_PAIR_GREEN: i16 = 2;
static COLOR_PAIR_RED: i16 = 3;

fn main() {
    matrixise::init();
    init_pair(COLOR_PAIR_BLACK, COLOR_BLACK, COLOR_WHITE);
    init_pair(COLOR_PAIR_GREEN, COLOR_BLACK, COLOR_GREEN);
    init_pair(COLOR_PAIR_RED,   COLOR_BLUE,  COLOR_RED  );

    // Here, we set up a white background
    let mut screen = Scene::new(20, COLOR_WHITE, true);
    screen.start(); // forks into a new thread

    // Black text on white background
    screen.push(Message::new_simple("Msg", "1", COLOR_PAIR(COLOR_PAIR_BLACK), 0));
    
    // Black text on green background
    screen.push(Message::new_simple("Msg", "2", COLOR_PAIR(COLOR_PAIR_GREEN), 1));
    
    // Black text on green background
    screen.push(Message::new_simple("Msg", "3", COLOR_PAIR(COLOR_PAIR_RED), 2));

    // 7-color rainbow using the most basic ncurses colors
    let mut string = ColorString::new();
    for i in 0..7 {
	init_pair(i+4, COLOR_BLACK+i, COLOR_BLACK+i); // create a color pair
	string.push(ColorChar::new(' ' as u32, COLOR_PAIR(i+4))); // and use it
    }
    screen.push(Message::new(string, 3)); // and turn it into a message

    // Create with inline color codes
    //screen.push(Message::new_from_ANSI("", 4)); NYI
    
    screen.join(); // wait for screen to die (user presses q)
}
