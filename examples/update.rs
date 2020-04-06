// Put in a few messages, then update them over time

extern crate matrixise;
use matrixise::*;

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_WHITE: i16 = 1;

use std::time::Duration;
use std::thread;

fn main() {
    matrixise::init(); // init must be called prior to defining colors
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    
    let mut screen = Scene::new(20, COLOR_BLACK, true);

    // We'll add 10 messages with different IDs:
    for i in 0..10 {
	screen.push(Message::new_with_title(&("M".to_string()+&i.to_string()), "+0", COLOR_PAIR(COLOR_PAIR_WHITE), &i.to_string()));
    }
    
    // we wait to start until we have a good chunk of messages stored
    screen.start();
    
    // These are now being printed out to terminal
    // Let's now update each message to have different contents

    let mut j = 1;
    while screen.alive() { // wait for screen to die (user presses q)
	thread::sleep(Duration::from_millis(1000)); // update every second
	screen.append_update((0..10).into_iter().map(|i| Message::new_with_title(&("M".to_string()+&i.to_string()), &("+".to_string()+&j.to_string()), COLOR_PAIR(COLOR_PAIR_WHITE), &i.to_string())).collect()); // update pre-existing messages, this time with a one-liner
	j += 1;
    }

    // because we wait for the screen to die, we don't need to kill it here
    // However, doing so won't cause an error:
    screen.kill();
}
