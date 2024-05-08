#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

extern crate alloc;

mod data;
mod library;
mod lua;
#[allow(improper_ctypes, dead_code)]
mod lua_core;
mod mtype;
mod stack;

use alloc::string::String;
pub use data::*;
pub use library::*;
pub use lua::*;
pub use lua_core::{Int, State};
pub use mtype::*;
pub use stack::*;

/// Various errors that may be returned.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Runtime(String),
    Data(DataErr),
    Library(LibraryErr),
}
impl From<LibraryErr> for Error {
    fn from(e: LibraryErr) -> Self {
        Error::Library(e)
    }
}
impl From<DataErr> for Error {
    fn from(e: DataErr) -> Self {
        Error::Data(e)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Runtime(s) => write!(f, "Runtime error: {}", s),
            Error::Data(e) => write!(f, "Data error: {}", e),
            Error::Library(e) => write!(f, "Library error: {}", e),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}
