// OFFICIAL REVONS OS REPO / REVONS OS
// revons company: contact.revons.co@support.co,
// fayssal chokri : contact.fayssal.revons@revons.co or
                //  contact.fayssal.chokri@developer.me
//! A simple application to print the fault log

#![no_std]

extern crate alloc;
extern crate fault_log;

use alloc::vec::Vec;
use alloc::string::String;
use fault_log::print_fault_log;

pub fn main(_args: Vec<String>) -> isize {
    print_fault_log();
    0
}
