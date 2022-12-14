//!  # Poggers
//!  A Work in Progress Memory (game cheating) Library for Rust
//!  # Safety
//!  We do not wish to strive away from the safe nature of rust, so all the code in this library **should** be safe, and shouldn't violate any of the rules of rust.
//!
//!  # Introduction for windows
//!  ## External
//!  Poggers allows you to effortlessly make external cheats for games. This is made possible with currently two modules which we feel like are the most important.
//!  Those being the following:
//!  * [`ExProcess`] - A wrapper around a process, allowing you to do basic things like read, write and change protections on memory..
//!  * [`ExModule`] - A wrapper around a module.
//! 
//!  With these two constructs it should make it pretty easy to safe and efficient external cheats.
//!  ## Internal
//!  Not complete.
//!
//!  # Introduction for linux
//!  Not complete.
//!
//!  # Example
//!  ```rust
//!  use poggers::external::windows::process::ExProcess;
//!  use poggers::external::windows::module::ExModule;
//!  let process = Process::new("notepad.exe").unwrap();
//!  let module = Module::new("user32.dll", &process).unwrap();
//!  let what = module.scan_virtual("48 8B 05 ? ? ? ? 48 8B 88 ? ? ? ? 48 85 C9 74 0A").unwrap();
//!  process.read::<u32>(what).unwrap();
//!  ```
//!
//!  ## License
//!  This project is licensed under the GPL-2.0 license.



#![warn(missing_docs)]
#![feature(generic_const_exprs)]
/// The main module for the memory library.
pub mod mem;
/// exports primarily for [`poggers-derive`]
pub mod exports;


// re exports
pub use mem::*;
