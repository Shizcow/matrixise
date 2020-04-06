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
	// If there's one with the same ID, update the contents
	// else, push normally
	if let Some(compare_msg) = self.data.iter_mut().rev().find(|cmp| cmp.id == message.id) { // starting from the back is recycle-pop safe
	    compare_msg.contents = message.contents;
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
    pub contents: ColorString,
    pub id:       i32,
    //pub title: String, // the bolded part: prints first
    //pub body: String,  // non-bolded part: prints second
    //pub color: attr_t, // color of the message
}

impl Message {
    pub fn new(contents: ColorString, id: i32) -> Self {
	Self{contents, id}
    }
    pub fn new_simple(title: &str, body: &str, color: attr_t, id: i32) -> Self { // creates new from body, title, and id
	let mut contents = ColorString::with_capacity(title.len()+body.len());
	for i in 0..title.len() {
	    contents.push(ColorChar{data: title.as_bytes()[i] as u32, attr: color | A_BOLD()});
	}
	for i in 0..body.len() {
	    contents.push(ColorChar{data: body.as_bytes()[i] as u32, attr: color});
	}
	Self::new(contents, id)
    }
    pub fn len(&self) -> usize {
	self.contents.len()
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
	Message::new(self.contents.clone(), self.id)
    }
}
