// How to use various color options

extern crate matrixise;
use matrixise::*;

static COLOR_PAIR_BLACK: i16 = 1;
static COLOR_PAIR_GREEN: i16 = 2;
static COLOR_PAIR_RED: i16 = 3;

use std::time::Duration;

fn main() {
    // Here, we set up a white background
    let mut screen = Scene::new(20, COLOR_WHITE, true, Duration::from_millis(20));
    screen.init_pair(COLOR_PAIR_BLACK, COLOR_BLACK, COLOR_WHITE);
    screen.init_pair(COLOR_PAIR_GREEN, COLOR_BLACK, COLOR_GREEN);
    screen.init_pair(COLOR_PAIR_RED,   COLOR_BLUE,  COLOR_RED  );

    screen.start(); // forks into a new thread

    // Black text on white background
    screen.push(Message::new_with_title("Msg", "1", COLOR_PAIR_BLACK, "0"));
    
    // Black text on green background
    screen.push(Message::new_with_title("Msg", "2", COLOR_PAIR_GREEN, "1"));
    
    // Black text on green background
    screen.push(Message::new_with_title("Msg", "3", COLOR_PAIR_RED, "2"));

    // 7-color rainbow using the most basic curses colors
    let mut string = ColorString::new();
    for i in 0..7 {
	screen.init_pair(i+4, COLOR_BLACK+i, COLOR_BLACK+i); // create a color pair
	string.push(ColorChar::new(' ' as u32, COLOR_PAIR(i as u32+4))); // and use it
	// Note: COLOR_PAIR() is used here because ColorChar can hold more than just
	//       colors. It can handle bold, italic, and other pancurses attributes too
    }
    screen.push(Message::new(string, "3")); // and turn it into a message
    
    screen.join(); // wait for screen to die (user presses q)
}
