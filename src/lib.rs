//! keepass: KeePass .kdbx database file parser for Rust
//!
//!
//! ```
//! extern crate keepass;
//!
//! use keepass::{Database, Node};
//! use keepass::result::{Result, ResultExt, Error};
//! use std::fs::File;
//!
//! fn main() -> Result<()> {
//!     // Open KeePass database
//!     let path = std::path::Path::new("tests/resources/test_db_with_password.kdbx");
//!     let db = Database::open(&mut File::open(path)?, Some("demopass"), None)?;
//!
//!     // Iterate over all Groups and Nodes
//!     for node in &db.root {
//!         match node {
//!             Node::GroupNode(g) => {
//!                 println!("Saw group '{0}'", g.name);
//!             },
//!             Node::EntryNode(e) => {
//!                 let title = e.get_title().unwrap();
//!                 let user = e.get_username().unwrap();
//!                 let pass = e.get_password().unwrap();
//!                 println!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

#![recursion_limit = "1024"]

extern crate base64;
extern crate byteorder;
extern crate crypto;
extern crate flate2;
extern crate secstr;
extern crate xml;

#[macro_use]
extern crate error_chain;

mod crypt;
mod db;
mod db_parse;
mod decompress;
pub mod result;
mod keyfile;
mod xml_parse;

pub use self::db::*;
// see https://gist.github.com/msmuenchen/9318327 for file format details
