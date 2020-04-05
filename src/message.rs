/*
 * message.rs
 *
 * Holds the Message struct and other important structs and aliases:
 *   ColorStringQueue
 *   ColorString
 *   ColorChar
 */

use std::fmt;
use std::collections::VecDeque;
use ncurses::{attr_t, A_BOLD};

// MessageQueue struct
// Holds many messages waiting to be used
pub struct MessageQueue {
    data: VecDeque<Message>,
    closed: bool,            // Do we recycle data?
}

impl MessageQueue {
    pub fn new(capacity: usize, closed: bool) -> Self {
	Self{data: VecDeque::with_capacity(capacity), closed}
    }
    pub fn pop(&mut self) -> Option<Message> { // pop from queue, check if we need to recycle
	let message_check = self.data.front(); // front() is used so data doesn't leave
	if message_check.is_none() {           // until it's safe (for updating)
	    return None;
	}
	
	if self.closed {
	    self.push(self.data.front().unwrap().clone());
	}
	self.data.pop_front()
    }
    pub fn push(&mut self, message: Message) {
	self.data.push_back(message);
    }
    pub fn push_update(&mut self, message: Message) {
	// Check through current messages
	// If there's one with the same title, update the body
	// else, push normally
	if let Some(compare_msg) = self.data.iter_mut().rev().find(|cmp| cmp.title == cmp.body) { // starting from the back is recycle-pop safe
	    compare_msg.body = message.body;
	} else {
	    self.push(message);
	}
    }
}

pub type ColorString = Vec<ColorChar>;

#[derive(Copy, Clone)]
pub struct ColorChar {
    pub data: u32,
    pub attr: attr_t
}

impl fmt::Debug for ColorChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&(self.data as u8 as char))
         .field(&self.attr)
         .finish()
    }
}

// Message struct
// Holds basic info about a message
pub struct Message {
    pub title: String, // the bolded part: prints first
    pub body: String,  // non-bolded part: prints second
    pub color: attr_t, // color of the message
}

impl Message {
    pub fn new(title: String, body: String, color: attr_t) -> Self {
	Self{title, body, color}
    }
    pub fn len(&self) -> usize {
	self.title.len()+self.body.len()
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
	Message::new(self.title.clone(), self.body.clone(), self.color)
    }
}

impl From<&Message> for ColorString {
    fn from(message: &Message) -> ColorString {
	let mut ret_str = ColorString::with_capacity(message.len());
	for i in 0..message.title.len() {
	    ret_str.push(ColorChar{data: message.title.as_bytes()[i] as u32, attr: message.color | A_BOLD()});
	}
	for i in 0..message.body.len() {
	    ret_str.push(ColorChar{data: message.body.as_bytes()[i] as u32, attr: message.color});
	}
	ret_str
    }
}
