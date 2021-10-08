// OFFICIAL REVONS OS REPO / REVONS OS
// revons company: contact.revons.co@support.co,
// fayssal chokri : contact.fayssal.revons@revons.co or
                //  contact.fayssal.chokri@developer.me

#![no_std]
// #![feature(plugin)]
// #![plugin(application_main_fn)]


extern crate alloc;
#[macro_use] extern crate terminal_print;
extern crate rtc;

use alloc::vec::Vec;
use alloc::string::String;


pub fn main(_args: Vec<String>) -> isize {
    let now = rtc::read_rtc();
    println!("{}", now);

    0
}
