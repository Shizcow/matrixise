// Measuring speeds of various parts of the program

extern crate matrixise;
use matrixise::*;

use std::time::{Instant, Duration};

static COLOR_PAIR_WHITE: i16 = 1;

fn testfull_push(spawn: i32) -> Duration {
    let start_time = Instant::now();
    let mut screen = Scene::new(20, COLOR_BLACK, true, Duration::from_millis(0));

    for _ in 0..spawn {
	screen.push(Message::new_simple("Message", COLOR_PAIR_WHITE, ""));
    }
    
    screen.kill();
    start_time.elapsed()
}

fn testfull_append(spawn: i32) -> Duration {
    let start_time = Instant::now();
    let mut screen = Scene::new(20, COLOR_BLACK, true, Duration::from_millis(0));

    screen.append((0..spawn).into_iter().map(|_| Message::new_simple("Message", COLOR_PAIR_WHITE, "")).collect());
    
    screen.kill();
    start_time.elapsed()
}

fn main() {

    testfull_push(0); // normalize ncurses overhead

    let mut times = Vec::new();

    // first, really small tests
    times.push(format!("Time to process {} requests using push: {} ms", 1, testfull_push(1).as_millis()));
    times.push(format!("Time to process {} requests using append: {} ms", 1, testfull_append(1).as_millis()));
    
    for e in 0..5 { // then, larger ones
	for i in (1..3).into_iter().map(|x| 10_i32.pow(e)*x*5) {
	    times.push(format!("Time to process {} requests using push: {} ms", i, testfull_push(i).as_millis()));
	    times.push(format!("Time to process {} requests using append: {} ms", i, testfull_append(i).as_millis()));
	}
    }

    for time in times {
	println!("{}", &time);
    }
}
