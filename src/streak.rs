/*
 * streak.rs
 *
 * Holds the Streak struct and all it's methods
 */

use rand::Rng;
use ncurses::{attron, attroff, mvaddch, attr_t};
use crate::message::{MessageQueue, ColorString, ColorChar};

// Streak struct
// Holds a streak's location&length
// Handles streak movement
// Can render all characters in a streak
pub struct Streak {
    head_x: i32, // horizontal coord
    head_y: i32, // Bottom of the streak
    length: i32, // length of streak
    inner_text: ColorString,    
}

impl Streak {
    // Takes a queue of messages, consuming when needed
    pub fn new_with_queue(queue: &mut MessageQueue, head_x: i32, length: i32, screen_height: i32, max_padding: i32) -> Self {
	let mut rng = rand::thread_rng();
	let mut inner_text = ColorString::with_capacity(screen_height as usize); // prealloc
	let first_msg_check = queue.pop();
	if let None = first_msg_check {
	    for _ in 0..screen_height {
		inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
	    }
	    return Streak{head_x, head_y: 0, length, inner_text}; // nothing to do!
	}
	let first_msg = first_msg_check.unwrap();
	{
	    let first_string: ColorString = first_msg.contents;
	    let mut start: i32 = rng.gen_range(0, first_string.len()+max_padding as usize) as i32 - first_string.len() as i32 + 1; // make sure there's at least one char printed, space up to max_padding is allowed at top
	    if start > screen_height {
		start = screen_height; // don't overflow
	    }
	    if start > 0 {
		for _ in 0..start {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
		}
	    }
	    for i in (
		if start < 0 {
		    -start // cut off relevant portion of message if required
		} else {
		    0
		}
	    )..(screen_height.min(first_string.len() as i32)) {
		inner_text.push(first_string[i as usize]);
		if inner_text.len() as i32 >= screen_height {
		    return Streak{head_x, head_y: 0, length, inner_text}; // if first message is too long
		}
	    }
	}
	
	loop {
	    let r: i32 = if max_padding > 0 {
		rng.gen_range(1,max_padding)
	    } else {
		0 // if padding is forced to 0, never pad ever
	    };
	    if inner_text.len() as i32+r >= screen_height { // terminate early
		for _ in 0..(screen_height as usize-inner_text.len()) {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // fill remaining
		}
		break; // streak is full
	    } else { // still need more content to fill
		for _ in 0..r {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0});
		}
	    }

	    
	    let next_msg_check = queue.pop();
	    if let None = next_msg_check {
		for _ in inner_text.len() as i32..screen_height {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
		}
		return Streak{head_x, head_y: 0, length, inner_text}; // nothing to do!
	    }
	    let next_msg = next_msg_check.unwrap();
	    {
		let next_string: ColorString = next_msg.contents;
		
		if inner_text.len()+next_string.len() >= screen_height as usize { // terminate early
		    for i in 0..(screen_height as usize-inner_text.len()) {
			inner_text.push(next_string[i as usize]); // fill remaining
		    }
		    break; // streak is full
		} else {
		    for i in 0..next_string.len() {
			inner_text.push(next_string[i as usize]); // print full string, move on
		    }
		}
	    }
	}
	Streak{head_x, head_y: 0, length, inner_text}
    }
    pub fn render(&self, screen_height: i32) { // print contents to screen
	for i in (self.head_y-self.length-1)..self.head_y {
	    if i >= 0 && i < screen_height {
		attron(self.inner_text[i as usize].attr);
		mvaddch(i,self.head_x,self.inner_text[i as usize].data);
		attroff(self.inner_text[i as usize].attr);
	    }
	}
    }
    pub fn derender(&self, attr: attr_t) { // removes first char, makes streak look like it's moving down
	attron(attr);
	mvaddch(self.head_y-self.length-1, self.head_x, ' ' as u32);
	attron(attr);
    }
    pub fn advance(&mut self) {
	self.head_y+=1;
    }
    pub fn finished(&self, screen_height: i32) -> bool { // can this streak be safely deleted?
	self.head_y-self.length >= screen_height
    }
    pub fn top_space(&self) -> i32 { // how much unallocated space at the top of the screen?
	self.head_y-self.length+1
    }
}
