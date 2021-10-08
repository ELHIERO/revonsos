// OFFICIAL REVONS OS REPO / REVONS OS
// revons company: contact.revons.co@support.co,
// fayssal chokri : contact.fayssal.revons@revons.co or
                //  contact.fayssal.chokri@developer.me

#![cfg_attr(feature = "unstable", feature(const_fn_trait_bound))]
#![feature(const_mut_refs)]
#![no_std]

// #[macro_use] extern crate log;

mod pages;
mod sc;
mod zone;

pub use pages::*;
pub use zone::*;

use core::alloc::Layout;
use core::fmt;
use core::mem;
use core::ptr::{self, NonNull};


#[cfg(target_arch = "x86_64")]
const CACHE_LINE_SIZE: usize = 64;

#[cfg(target_arch = "x86_64")]
type VAddr = usize;
