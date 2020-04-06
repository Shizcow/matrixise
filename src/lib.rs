mod message;
use crate::message::MessageQueue;
pub use crate::message::{Message, ColorString, ColorChar};
mod streak;
use crate::streak::Streak;

use ncurses::*;

extern crate rand;
use rand::Rng;


use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn init(){
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);
    start_color();
}

enum ThreadMsg {
    Kill,
    Start,
    Push(Message),
    PushUpdate(Message),
    Append(Vec<Message>), 
    AppendUpdate(Vec<Message>),
}

struct ForkedScene { // the version of Scene that lives in another thread
    columns:     Vec<Column>,  // holds all streaks in the scene
    height:      i32,          // height of the scene
    queue:       MessageQueue, // Messages yet to be printed
    background:  attr_t,       // used for derendering
    max_padding: i32,
    rx:          Option<std::sync::mpsc::Receiver<ThreadMsg>>,
    started:     bool,
}
impl ForkedScene {
    pub fn new(max_padding: i32, background: i16, is_closed: bool, rx: std::sync::mpsc::Receiver<ThreadMsg>) -> Self {
	// infer screen size
	let bkg_attr = Self::init(background);
	let mut height : i32 = 0;
	let mut width  : i32 = 0;
	getmaxyx(stdscr(), &mut height, &mut width);
	if width == -1 {
	    panic!("Could not get screen size!");
	}
	let mut columns = Vec::new();
	for _ in 0..width {
	    columns.push(Column::new());
	}
	Self{columns, height, queue: MessageQueue::new(width as usize, is_closed), background: bkg_attr, max_padding, rx: Some(rx), started: false}
    }
    fn init(background: i16) -> attr_t { // ncurses initialize
	let background_attr = 99; // NOTE: find a portabale value that users won't touch
	init_pair(background_attr, COLOR_BLACK, background);
	bkgd(' ' as chtype | COLOR_PAIR(background_attr) as chtype);
	COLOR_PAIR(background_attr)
    }
    pub fn kill(&self){
	endwin();
    }
    pub fn update(&mut self) -> bool {
	if self.rx.is_none() {
	    return false;
	}
	let rx = self.rx.as_ref().unwrap();
        match rx.try_recv() {
	    Err(TryRecvError::Disconnected) => {
                self.kill();
                return false;
	    }
	    Ok(thread_msg) => {
		match thread_msg {
		    ThreadMsg::Start => {
			if self.started == true {
			    panic!("Tried to start screen twice!");
			} else {
			    self.started = true;
			}
			return true;
		    },
		    ThreadMsg::Push(message) => {
			self.queue.push(message);
			return true;
		    },
		    ThreadMsg::PushUpdate(message) => {
			self.queue.push_update(message);
			return true;
		    },
		    ThreadMsg::Append(messages) => {
			self.queue.append(messages);
			return true;
		    }
		    ThreadMsg::AppendUpdate(messages) => {
			self.queue.append_update(messages);
			return true;
		    }
		    ThreadMsg::Kill => {
			self.kill();
			return false; // make sure main exits
		    },
		}
	    }
	    Err(TryRecvError::Empty) => {
		if self.started {
		    thread::sleep(Duration::from_millis(25));
		}
	    }
        }
	if self.started {
	    self.advance();
	}
	true
    }
    pub fn advance(&mut self){ // move all streaks, clean up dead ones, try to spawn new ones
	let mut rng = rand::thread_rng();
	let untouched = self.columns.iter().fold(0, |sum, column| sum + if column.touched  {0} else {1}); // counting untouched to make it progressively more likely to spawn a streak
	for (i, column) in self.columns.iter_mut().enumerate() {
	    for streak in &mut column.streaks { // advance all
		streak.derender(self.background);
		streak.advance();
	    }
	    let height = self.height; // always fighting with the borrow checker
	    column.streaks.retain(|streak| !streak.finished(height)); // clean up dead streaks

	    // now, try to spawn new streaks
	    if column.streaks.len()==0 || column.streaks.iter().all(|streak| streak.top_space() > 5) { // check if there's need to
		if (!column.touched && rng.gen_range(0, untouched) == 0) || column.touched { // if we started recently, thin things out to look better
		// add new streak, consuming from queue
		column.add_streak(Streak::new_with_queue(&mut self.queue, i as i32, rng.gen_range(5, self.height*2), self.height, self.max_padding));
		    
		}
	    }

	    
	    for streak in &mut column.streaks { // advance all
		streak.render(self.height);
	    }
	}
	refresh();
    }
}

// Scene struct
// Holds all data for the scene, including:
//   Streaks (light things up and hold messages)
//   Dimensions
//   MessageQueue (messages yet to be printed)
// Handles updating, can render

pub struct Scene {
    tx:              Option<std::sync::mpsc::Sender<ThreadMsg>>, // used to communicate with other threads
    join_handle:     Option<JoinHandle<()>>, // needed for rejoining
    thread_control:  Option<Weak<AtomicBool>>, // needed for seeing if the thread is still alive
}

impl Scene {
    pub fn new(max_padding: i32, background: i16, is_closed: bool) -> Self {
	let (tx, rx) = mpsc::channel();
	let mut background = ForkedScene::new(max_padding, background, is_closed, rx);

	let working = Arc::new(AtomicBool::new(true));
	let control = Arc::downgrade(&working);

	let join_handle = thread::spawn(move ||
					while (*working).load(Ordering::Relaxed) {
					    if !background.update() {
						break;
					    }
					    if getch() == 113 { // if q is pressed, exit
						background.kill();
						break;
					    }
					});
	
	Self{tx: Some(tx), join_handle: Some(join_handle), thread_control: Some(control)}
    }
    pub fn push(&mut self, message: Message){
	if !self.alive() {
	    return;
	}
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::Push(message));
    }
    pub fn push_update(&mut self, message: Message){
	if !self.alive() {
	    return;
	}
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::PushUpdate(message));
    }
    pub fn append(&mut self, messages: Vec<Message>){
	if !self.alive() {
	    return;
	}
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::Append(messages));
    }
    pub fn append_update(&mut self, messages: Vec<Message>){
	if !self.alive() {
	    return;
	}
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::AppendUpdate(messages));
    }
    pub fn start(&mut self){ // start and fork to background
	refresh();
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::Start);
    }
    pub fn alive(&self) -> bool { // ping the background thread to see if it's alive
	self.thread_control.is_some() && self.thread_control.as_ref().unwrap().upgrade().is_some()
    }
    pub fn kill(&mut self){
	if !self.alive() {
	    return;
	}
	let _ = self.tx.as_ref().unwrap().send(ThreadMsg::Kill);
	let _ = self.join_handle.take().unwrap().join(); // give ncurses time to clean up
	self.join_handle = None;
	self.tx = None;
    }
    pub fn join(&self) {
	while self.alive() {} // wait till screen is dead
    }
}

// Column struct
struct Column {
    streaks: Vec<Streak>,
    touched: bool,          // have we ever put a streak into this column?
}

impl Column {
    fn new() -> Self {
	Self{streaks: Vec::new(), touched: false}
    }
    fn add_streak(&mut self, streak: Streak){
	self.streaks.push(streak);
	self.touched = true;
    }
}

#[cfg(test)]
mod tests;
