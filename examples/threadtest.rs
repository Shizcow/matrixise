// Measuring speeds of various parts of the program

extern crate matrixise;
use matrixise::*;

use std::time::{Instant, Duration};

use ncurses::{COLOR_PAIR, init_pair};
use ncurses::constants::*;
static COLOR_PAIR_WHITE: i16 = 1;


fn testfull_push(spawn: i32) -> Duration {
    let start_time = Instant::now();
    matrixise::init();
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    let mut screen = Scene::new(20, COLOR_BLACK, true);

    for _ in 0..spawn {
	screen.push(Message::new_simple("Message", COLOR_PAIR(COLOR_PAIR_WHITE), ""));
    }
    
    screen.kill();
    start_time.elapsed()
}

fn testfull_append(spawn: i32) -> Duration {
    let start_time = Instant::now();
    matrixise::init();
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    let mut screen = Scene::new(20, COLOR_BLACK, true);

    screen.append((0..spawn).into_iter().map(|_| Message::new_simple("Message", COLOR_PAIR(COLOR_PAIR_WHITE), "")).collect());
    
    screen.kill();
    start_time.elapsed()
}

fn main() {

    let mut times = Vec::new();

    for e in 0..5 {
	for i in (1..5).into_iter().map(|x| 10_i32.pow(e)*x*2) {
	    times.push(format!("Time to process {} requests using push: {} ms", i, testfull_push(i).as_millis()));
	    times.push(format!("Time to process {} requests using append: {} ms", i, testfull_append(i).as_millis()));
	}
    }

    for time in times {
	println!("{}", &time);
    }
}
