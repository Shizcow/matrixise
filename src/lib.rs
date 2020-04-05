mod message;
use crate::message::MessageQueue;
pub use crate::message::Message;
mod streak;
use crate::streak::Streak;

use ncurses::*;

extern crate rand;
use rand::Rng;


use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

pub fn init(){
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);
    start_color();
}

struct ThreadMsg {
    title:       String,
    content:     Option<Message>,
}

impl ThreadMsg {
    pub fn new(title: &str, content: Option<Message>) -> Self {
	Self{title: title.to_string(), content}
    }
}

struct ForkedScene { // the version of Scene that lives in another thread
    columns:     Vec<Column>,  // holds all streaks in the scene
    height:      i32,          // height of the scene
    queue:       MessageQueue, // Messages yet to be printed
    background:  attr_t,       // used for derendering
    max_padding: i32,
    rx:          Option<std::sync::mpsc::Receiver<ThreadMsg>>
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
	Self{columns, height, queue: MessageQueue::new(width as usize, is_closed), background: bkg_attr, max_padding, rx: Some(rx)}
    }
    fn init(background: i16) -> attr_t { // ncurses initialize
	let background_attr = 99; // NOTE: find a portabale value that users won't touch
	init_pair(background_attr, background, COLOR_BLACK);
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
	refresh();
        thread::sleep(Duration::from_millis(100));
        match rx.try_recv() {
	    Err(TryRecvError::Disconnected) => {
                self.kill();
                return false;
	    }
	    Ok(thread_msg) => {
		match thread_msg.title.as_str() {
		    "push" => self.push(thread_msg.content.unwrap()),
		    "push_update" => self.push_update(thread_msg.content.unwrap()),
		    "kill" => self.kill(),
		    &_ => panic!("Invalid command {:}"),
		}
		
	    }
	    Err(TryRecvError::Empty) => {}
        }
	self.advance();
	true
    }
    pub fn push(&mut self, message: Message){
	self.queue.push(message);
    }
    pub fn push_update(&mut self, message: Message){
	self.queue.push_update(message);
    }
    pub fn advance(&mut self){ // move all streaks, clean up dead ones, try to spawn new ones
	let mut rng = rand::thread_rng();
	for (i, column) in self.columns.iter_mut().enumerate() {
	    for streak in &mut column.streaks { // advance all
		streak.derender(self.background);
		streak.advance();
	    }
	    let height = self.height; // always fighting with the borrow checker
	    column.streaks.retain(|streak| !streak.finished(height)); // clean up dead streaks

	    // now, try to spawn new streaks
	    if column.streaks.len()==0 || column.streaks.iter().all(|streak| streak.top_space() > 5) { // there's need to
		column.streaks.push(Streak::new_with_queue(&mut self.queue, i as i32, rng.gen_range(5, self.height*2), self.height, self.max_padding)); // add new streak, consuming from queue
		// TODO: better length requirements
	    }

	    
	    for streak in &mut column.streaks { // advance all
		streak.render(self.height);
	    }
	}
    }
}

// Scene struct
// Holds all data for the scene, including:
//   Streaks (light things up and hold messages)
//   Dimensions
//   MessageQueue (messages yet to be printed)
// Handles updating, can render

pub struct Scene {
    tx:          Option<std::sync::mpsc::Sender<ThreadMsg>>, // used to communicate with other threads
}

impl Scene {
    pub fn push(&mut self, message: Message){
	if self.tx.is_none() {
	    return;
	}
	if let Ok(_) = self.tx.as_ref().unwrap().send(ThreadMsg::new("push", Some(message))) {}
    }
    pub fn push_update(&mut self, message: Message){
	if self.tx.is_none() {
	    return;
	}
	if let Ok(_) = self.tx.as_ref().unwrap().send(ThreadMsg::new("push_update", Some(message))) {}
    }
    pub fn append(&mut self, messages: Vec<Message>){
	if self.tx.is_none() {
	    return;
	}
	for message in messages {
	    self.push(message);
	}
    }
    pub fn append_update(&mut self, messages: Vec<Message>){
	if self.tx.is_none() {
	    return;
	}
	for message in messages {
	    self.push_update(message);
	}
    }
    pub fn new() -> Self {
	Self{tx: None}
    }
    pub fn start(&mut self, max_padding: i32, background: i16, is_closed: bool){ // start and fork to background
	if let Some(_) = self.tx {
	    return; // already started
	}
	let (tx, rx) = mpsc::channel();
	self.tx = Some(tx); // set up communication channel
	let mut background = ForkedScene::new(max_padding, background, is_closed, rx);

	thread::spawn(move || while background.update() {});
    }
    pub fn kill(&self){
	if let Some(tx) = &self.tx {
	    if let Ok(_) = tx.send(ThreadMsg::new("kill", None)){} // send kill signal
	}
    }
}

// Column struct
// Just makes reading things easier
struct Column {
    streaks: Vec<Streak>
}

impl Column {
    fn new() -> Self {
	Self{streaks: Vec::new()}
    }
}

#[cfg(test)]
mod tests;
