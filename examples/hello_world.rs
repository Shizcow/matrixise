// A simple Hello World program

extern crate matrixise;
use matrixise::*;

use std::time::Duration;

const WHITE_PAIR : i16 = 1;

fn main() {
    // creating a screne with [max_padding = 20, black background, reuse messages, move down every 50ms]:
    let mut screen = Scene::new(20, COLOR_BLACK, true, Duration::from_millis(50));

    // initalize test color
    screen.init_pair(WHITE_PAIR, COLOR_WHITE, COLOR_BLACK);

    screen.start(); // forks into a new thread
    
    // Here's one way to add messages:
    screen.push(Message::new_simple("Hello world", WHITE_PAIR, "0"));
    
    screen.join(); // wait for screen to die (user presses q)
}
